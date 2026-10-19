//! Ingestion is idempotent under duplicate delivery (issue #358).
//!
//! RPC sources redeliver: a retried poll after a timeout whose response
//! actually succeeded, an at-least-once streaming guarantee, a restart
//! re-reading the page that was in flight. The uniqueness key that absorbs
//! all of it is **`(tx_hash, event_index)`** — enforced by the store's
//! `ON CONFLICT (tx_hash, event_index) DO NOTHING` insert — and it is the
//! same key on both ingestion paths, because backfill and steady-state
//! polling share the single [`Ingestor::ingest_batch`] path (there is no
//! second parser or writer to drift).
//!
//! These tests run against the real SQLite store, so they exercise the
//! actual conflict target, not a mock of it.

use keeper_indexer::events::{EventPayload, I128};
use keeper_indexer::rpc::{EventPage, EventSource, RawEvent, RawValue};
use keeper_indexer::{Backfiller, Ingestor, Store};

use anyhow::Result;

fn registered(ledger: u32, tx: &str, task_id: u64, owner: &str, reward: i128) -> RawEvent {
    RawEvent {
        ledger,
        ledger_close_time: i64::from(ledger) * 5,
        tx_hash: tx.to_string(),
        event_index: 0,
        topics: vec!["reg".into(), "task".into()],
        values: vec![
            RawValue::U64(task_id),
            RawValue::Address(owner.into()),
            RawValue::I128(reward),
            RawValue::U64(9_000),
        ],
    }
}

fn executed(ledger: u32, tx: &str, task_id: u64, keeper: &str, net_reward: i128) -> RawEvent {
    RawEvent {
        ledger,
        ledger_close_time: i64::from(ledger) * 5,
        tx_hash: tx.to_string(),
        event_index: 1,
        topics: vec!["exec".into(), "task".into()],
        values: vec![
            RawValue::U64(task_id),
            RawValue::Address(keeper.into()),
            RawValue::I128(net_reward),
            RawValue::Bytes("aa".into()),
        ],
    }
}

async fn ingestor() -> Ingestor {
    let store = Store::connect("sqlite::memory:").await.expect("store");
    Ingestor::new(store)
}

/// Every stored event, oldest first, as one comparable value.
async fn all_events(ingestor: &Ingestor) -> Vec<(String, EventPayload)> {
    let page = ingestor
        .store()
        .events_after(None, 500, None, None)
        .await
        .expect("event feed");
    page.events
        .into_iter()
        .map(|e| (e.tx_hash.clone(), e.payload))
        .collect()
}

#[tokio::test]
async fn delivering_the_same_batch_twice_equals_delivering_it_once() {
    let once = ingestor().await;
    let twice = ingestor().await;
    let batch = [
        registered(10, "tx1", 1, "GOWNER", 500),
        executed(11, "tx2", 1, "GKEEPER", 450),
    ];

    once.ingest_batch(&batch).await.expect("single delivery");

    twice.ingest_batch(&batch).await.expect("first delivery");
    let redelivery = twice.ingest_batch(&batch).await.expect("second delivery");

    // The redelivery is recognised, not re-applied.
    assert_eq!(redelivery.stored, 0);
    assert_eq!(redelivery.duplicates, batch.len());

    // And the stored state is byte-for-byte what a single delivery produces.
    assert_eq!(all_events(&once).await, all_events(&twice).await);
}

#[tokio::test]
async fn derived_views_are_unaffected_by_a_duplicate() {
    // Issue #358 acceptance: not just the raw event table — the folds a
    // consumer actually reads (task status, keeper totals) must not
    // double-apply a redelivered event either.
    let ingestor = ingestor().await;
    let batch = [
        registered(10, "tx1", 7, "GOWNER", 500),
        executed(11, "tx2", 7, "GKEEPER", 450),
    ];

    ingestor.ingest_batch(&batch).await.expect("first");
    let task_before = ingestor.store().task_state(7).await.expect("task").unwrap();
    let keeper_before = ingestor
        .store()
        .keeper_summary("GKEEPER")
        .await
        .expect("keeper");

    ingestor.ingest_batch(&batch).await.expect("redelivery");
    let task_after = ingestor.store().task_state(7).await.expect("task").unwrap();
    let keeper_after = ingestor
        .store()
        .keeper_summary("GKEEPER")
        .await
        .expect("keeper");

    assert_eq!(task_before, task_after);
    assert_eq!(
        keeper_before.executed_task_ids,
        keeper_after.executed_task_ids
    );
    // The financially dangerous one: a double-counted execution would
    // silently inflate the keeper's earnings.
    assert_eq!(keeper_before.total_earned, keeper_after.total_earned);
    assert_eq!(keeper_after.total_earned, I128(450));
}

#[tokio::test]
async fn the_same_transaction_can_still_carry_distinct_events() {
    // The uniqueness key is (tx_hash, event_index), not tx_hash alone: one
    // transaction legitimately emits several events, and deduplication must
    // not swallow the real second event while rejecting the redelivered
    // first one.
    let ingestor = ingestor().await;
    let first = registered(10, "tx1", 1, "GOWNER", 500);
    let mut second = registered(10, "tx1", 2, "GOWNER", 700);
    second.event_index = 1;

    let outcome = ingestor
        .ingest_batch(&[first.clone(), second, first])
        .await
        .expect("ingest");

    assert_eq!(outcome.stored, 2);
    assert_eq!(outcome.duplicates, 1);
}

/// An [`EventSource`] serving a fixed set of events, with a tip that can be
/// beyond them — enough to drive the real backfill walk in a test. (The
/// crate's own `rpc::fixture` is `#[cfg(test)]` and not visible to
/// integration tests.)
struct FixedSource {
    events: Vec<RawEvent>,
    tip: u32,
}

impl EventSource for FixedSource {
    async fn get_events(
        &self,
        _contract_id: &str,
        start_ledger: u32,
        limit: u32,
    ) -> Result<EventPage> {
        let end = start_ledger.saturating_add(limit.max(1) - 1);
        let events = self
            .events
            .iter()
            .filter(|e| e.ledger >= start_ledger && e.ledger <= end)
            .cloned()
            .collect();
        Ok(EventPage {
            events,
            latest_ledger_scanned: end.min(self.tip),
        })
    }

    async fn latest_ledger(&self) -> Result<u32> {
        Ok(self.tip)
    }
}

#[tokio::test]
async fn backfill_and_steady_state_share_the_key_a_restart_overlap_hits() {
    // Issue #358 acceptance: the uniqueness key is stable across the
    // backfill and steady-state paths. Both are the same walk re-entering
    // `run_to_tip`, so the overlap a crash-and-resume produces (the page in
    // flight is re-read) must land on the same (tx_hash, event_index) rows
    // and be skipped, not re-stored.
    let store = Store::connect("sqlite::memory:").await.expect("store");
    let ingestor = Ingestor::new(store);
    let source = FixedSource {
        events: vec![
            registered(10, "tx1", 1, "GOWNER", 500),
            executed(12, "tx2", 1, "GKEEPER", 450),
        ],
        tip: 20,
    };
    let backfiller = Backfiller::new(source, ingestor.clone(), "CCONTRACT", 5);

    // Initial backfill to the tip.
    let first = backfiller.run_to_tip(1).await.expect("backfill");
    assert_eq!(first.stored, 2);
    assert_eq!(first.duplicates, 0);

    // Simulate the restart overlap: forget the checkpoint's progress and
    // walk the same range again, exactly as a resume-from-older-checkpoint
    // (or an at-least-once source) would.
    let second = backfiller.run_to_tip(1).await.expect("steady-state pass");
    assert_eq!(second.stored, 0, "a re-walk must store nothing new");

    // The store agrees: two events, not four.
    assert_eq!(all_events(&ingestor).await.len(), 2);
}
