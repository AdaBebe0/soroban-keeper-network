//! Batch reads: `get_tasks` and `get_tasks_range`.

use soroban_sdk::{testutils::Address as _, Address, Env};

use super::common::*;
use crate::{KeeperError, KeeperRegistry, KeeperRegistryClient, TaskStatus, MAX_BATCH_READ};

// ─────────────────────────────────────────────────────────────────────────────
// Batch task reads: get_tasks / get_tasks_range
// ─────────────────────────────────────────────────────────────────────────────
//
// These views exist so an indexer or keeper bot can inspect a range of tasks
// without one RPC round trip per task (issue #25). Two properties are load
// bearing and are pinned here rather than left to the doc comment:
//
//   1. The result is POSITIONALLY ALIGNED with the request — `out.len() ==
//      ids.len()`, and `out[i]` corresponds to `ids[i]`. `Task` carries no
//      `task_id`, so a compacted result would make the mapping back to the
//      requested id unrecoverable.
//   2. Over-limit requests ERROR rather than truncate, so a clipped page can
//      never be mistaken for the end of a range.

/// Registers `n` tasks with a small reward each and returns their ids in
/// registration order. Rewards are kept small so a full `MAX_BATCH_READ` batch
/// fits inside the admin's minted balance.
fn register_n_tasks(s: &TestSetup, n: u32) -> soroban_sdk::Vec<u64> {
    let mut ids = soroban_sdk::Vec::new(&s.env);
    for _ in 0..n {
        ids.push_back(register_reward_task(s, 1_000i128));
    }
    ids
}

#[test]
fn test_get_tasks_full_batch_returns_every_task() {
    let s = setup();
    let ids = register_n_tasks(&s, MAX_BATCH_READ);

    // Exactly at the cap — the largest accepted batch.
    let tasks = s.registry.get_tasks(&ids);
    assert_eq!(tasks.len(), MAX_BATCH_READ);

    // Every entry is present and identical to what the single-key view returns
    // for the id at the same position: the batch view is a pure fan-out of
    // `get_task`, not a different read.
    for (i, id) in ids.iter().enumerate() {
        let batched = tasks.get(i as u32).unwrap();
        assert_eq!(batched, Some(s.registry.get_task(&id)));
    }
}

#[test]
fn test_get_tasks_returns_none_for_missing_ids_without_failing() {
    let s = setup();
    let live = register_n_tasks(&s, 3);
    let (a, b, c) = (
        live.get(0).unwrap(),
        live.get(1).unwrap(),
        live.get(2).unwrap(),
    );

    // Interleave live ids with ids that were never allocated. A single absent
    // id must not fail the call, and the holes must land at the positions that
    // were asked for — the whole point of the aligned result.
    let requested = soroban_sdk::vec![&s.env, a, 9_999u64, b, 0u64, c];
    let tasks = s.registry.get_tasks(&requested);

    assert_eq!(tasks.len(), 5);
    assert_eq!(tasks.get(0).unwrap(), Some(s.registry.get_task(&a)));
    assert_eq!(tasks.get(1).unwrap(), None);
    assert_eq!(tasks.get(2).unwrap(), Some(s.registry.get_task(&b)));
    // Id 0 is never handed out — the counter starts at 1.
    assert_eq!(tasks.get(3).unwrap(), None);
    assert_eq!(tasks.get(4).unwrap(), Some(s.registry.get_task(&c)));
}

#[test]
fn test_get_tasks_empty_input_returns_empty() {
    let s = setup();
    register_default_task(&s);

    // An empty request is legal and cheap, not an error: a bot whose candidate
    // filter matched nothing this round should not have to special-case the
    // call.
    let tasks = s.registry.get_tasks(&soroban_sdk::Vec::new(&s.env));
    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_get_tasks_over_limit_fails() {
    let s = setup();

    // One id over the cap — the smallest rejected batch. The ids need not
    // exist; the bound is checked before any storage is touched.
    let mut ids = soroban_sdk::Vec::new(&s.env);
    for id in 1..=(MAX_BATCH_READ as u64 + 1) {
        ids.push_back(id);
    }
    assert_eq!(ids.len(), MAX_BATCH_READ + 1);

    assert_eq!(
        s.registry.try_get_tasks(&ids),
        Err(Ok(KeeperError::BatchTooLarge))
    );
}

#[test]
fn test_get_tasks_duplicate_ids_resolved_independently() {
    let s = setup();
    let id = register_default_task(&s);

    // Duplicates are permitted rather than deduplicated: silently collapsing
    // them would break positional alignment, which is the stronger guarantee.
    let tasks = s.registry.get_tasks(&soroban_sdk::vec![&s.env, id, id, id]);
    assert_eq!(tasks.len(), 3);
    let expected = Some(s.registry.get_task(&id));
    for i in 0..3u32 {
        assert_eq!(tasks.get(i).unwrap(), expected);
    }
}

#[test]
fn test_get_tasks_on_uninitialized_registry_returns_all_none() {
    let env = Env::default();
    let registry_id = env.register(KeeperRegistry, ());
    let registry = KeeperRegistryClient::new(&env, &registry_id);

    // Per the read-only views policy, a view on an unconfigured registry has an
    // unambiguous answer (no tasks) and must not return NotInitialized, so a
    // keeper bot can probe a fresh deployment speculatively.
    let tasks = registry.get_tasks(&soroban_sdk::vec![&env, 1u64, 2u64]);
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks.get(0).unwrap(), None);
    assert_eq!(tasks.get(1).unwrap(), None);
}

#[test]
fn test_get_tasks_range_scans_contiguous_ids() {
    let s = setup();
    let ids = register_n_tasks(&s, 5);

    // Ids are handed out contiguously from 1, so the range form should agree
    // with the explicit-ids form over the same span.
    let ranged = s.registry.get_tasks_range(&ids.get(0).unwrap(), &5u32);
    let listed = s.registry.get_tasks(&ids);
    assert_eq!(ranged, listed);
    assert_eq!(ranged.len(), 5);
    for i in 0..5u32 {
        assert!(ranged.get(i).unwrap().is_some());
    }
}

#[test]
fn test_get_tasks_range_returns_none_past_the_last_allocated_id() {
    let s = setup();
    register_n_tasks(&s, 2);
    assert_eq!(s.registry.task_count(), 2u64);

    // Asking for a window that runs off the end of the allocated ids yields
    // trailing Nones rather than a short vector, so a bot walking a fixed
    // window size does not have to guess where the range stopped.
    let tasks = s.registry.get_tasks_range(&1u64, &4u32);
    assert_eq!(tasks.len(), 4);
    assert!(tasks.get(0).unwrap().is_some());
    assert!(tasks.get(1).unwrap().is_some());
    assert_eq!(tasks.get(2).unwrap(), None);
    assert_eq!(tasks.get(3).unwrap(), None);
}

#[test]
fn test_get_tasks_range_zero_count_returns_empty() {
    let s = setup();
    register_default_task(&s);

    let tasks = s.registry.get_tasks_range(&1u64, &0u32);
    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_get_tasks_range_at_limit_succeeds_and_over_limit_fails() {
    let s = setup();
    register_n_tasks(&s, 3);

    // At the cap: accepted, even though most of the window is unallocated.
    let at_limit = s.registry.get_tasks_range(&1u64, &MAX_BATCH_READ);
    assert_eq!(at_limit.len(), MAX_BATCH_READ);

    // One over: rejected with the typed error rather than silently clipped to
    // MAX_BATCH_READ, which a caller would read as "the range ended here".
    assert_eq!(
        s.registry.try_get_tasks_range(&1u64, &(MAX_BATCH_READ + 1)),
        Err(Ok(KeeperError::BatchTooLarge))
    );
}

#[test]
fn test_get_tasks_range_wrapping_past_u64_max_fails() {
    let s = setup();
    register_default_task(&s);

    // `from + count` would wrap and start returning unrelated low-numbered
    // tasks — including the live task 1. Reject instead.
    assert_eq!(
        s.registry.try_get_tasks_range(&u64::MAX, &2u32),
        Err(Ok(KeeperError::ArithmeticOverflow))
    );

    // The largest non-wrapping window ending exactly at u64::MAX is still
    // accepted; the guard rejects overflow, not the boundary itself.
    let edge = s.registry.get_tasks_range(&(u64::MAX - 1), &2u32);
    assert_eq!(edge.len(), 2);
    assert_eq!(edge.get(0).unwrap(), None);
    assert_eq!(edge.get(1).unwrap(), None);
}

#[test]
fn test_batch_reads_reflect_lifecycle_transitions() {
    let s = setup();
    let ids = register_n_tasks(&s, 2);
    let claimed_id = ids.get(0).unwrap();

    let keeper = Address::generate(&s.env);
    s.registry.claim_task(&keeper, &claimed_id);

    // The batch view reads live storage, so it must show the post-claim state
    // rather than a snapshot taken at registration.
    let tasks = s.registry.get_tasks(&ids);
    assert_eq!(tasks.get(0).unwrap().unwrap().status, TaskStatus::Claimed);
    assert_eq!(tasks.get(0).unwrap().unwrap().claimer, Some(keeper.clone()));
    assert_eq!(tasks.get(1).unwrap().unwrap().status, TaskStatus::Pending);
    assert_eq!(tasks.get(1).unwrap().unwrap().claimer, None);
}

#[test]
fn test_batch_reads_work_while_paused() {
    let s = setup();
    let ids = register_n_tasks(&s, 2);
    s.registry.pause(&s.admin);

    // Read-only views never gate on pause — an indexer must keep working
    // through an emergency stop.
    assert_eq!(s.registry.get_tasks(&ids).len(), 2);
    assert_eq!(s.registry.get_tasks_range(&1u64, &2u32).len(), 2);
}
