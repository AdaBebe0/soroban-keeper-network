//! Reputation bookkeeping integrated with task success and expired claim takeover.

// This module only compiles under cfg(test), where std is always linked.
extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events as _},
    Address, Bytes, IntoVal, Symbol, TryIntoVal, Val,
};

use super::common::*;
use crate::reputation::stored_record;
use crate::KeeperError;

fn record(s: &TestSetup, keeper: &Address) -> crate::reputation::ReputationRecord {
    s.env
        .as_contract(&s.registry.address, || stored_record(&s.env, keeper))
}

#[test]
fn successes_and_missed_lock_window_update_reputation() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let next_keeper = Address::generate(&s.env);

    for _ in 0..2 {
        let task_id = register_default_task(&s);
        s.registry.claim_task(&keeper, &task_id);
        s.registry
            .execute_task(&keeper, &task_id, &Bytes::from_slice(&s.env, b"proof"));
    }

    let missed_task = register_default_task(&s);
    s.registry.claim_task(&keeper, &missed_task);
    advance(&s.env, 120, 0);
    s.registry.claim_task(&next_keeper, &missed_task);

    let record = record(&s, &keeper);
    assert_eq!(record.successes, 2);
    assert_eq!(record.missed_claims, 1);
    assert_eq!(record.score_bps, 6_666);
    assert_eq!(record.last_updated_ledger, s.env.ledger().sequence());
}

#[test]
fn failed_or_rejected_actions_do_not_update_reputation() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let task_id = register_default_task(&s);

    s.registry.claim_task(&keeper, &task_id);
    assert_eq!(
        s.registry.try_execute_task(
            &keeper,
            &task_id,
            &Bytes::from_slice(&s.env, &[0; (crate::MAX_PROOF_LEN + 1) as usize]),
        ),
        Err(Ok(crate::KeeperError::ProofTooLarge))
    );

    let record = record(&s, &keeper);
    assert_eq!(record.successes, 0);
    assert_eq!(record.missed_claims, 0);
    assert_eq!(record.score_bps, 0);
}

#[test]
fn keeper_reputation_returns_stored_record_and_zero_for_new_keeper() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    assert_eq!(
        s.registry.keeper_reputation(&keeper),
        crate::ReputationRecord::zero()
    );

    let task_id = register_default_task(&s);
    s.registry.claim_task(&keeper, &task_id);
    s.registry
        .execute_task(&keeper, &task_id, &Bytes::from_slice(&s.env, b"proof"));

    let before = s.registry.keeper_reputation(&keeper);
    assert_eq!(before.successes, 1);
    assert_eq!(before.missed_claims, 0);
    assert_eq!(before.score_bps, 10_000);
    assert_eq!(s.registry.keeper_reputation(&keeper), before);
}

#[test]
fn effective_reputation_decays_at_exact_half_life_boundaries_without_writing() {
    let s = setup_long_lived();
    let keeper = Address::generate(&s.env);
    let task_id = register_default_task(&s);
    s.registry.claim_task(&keeper, &task_id);
    s.registry
        .execute_task(&keeper, &task_id, &Bytes::from_slice(&s.env, b"proof"));

    let stored = s.registry.keeper_reputation(&keeper);
    let half_life = crate::reputation::REPUTATION_DECAY_HALF_LIFE_LEDGERS;
    let origin = stored.last_updated_ledger;

    goto_ledger(&s.env, origin + half_life - 1);
    assert_eq!(s.registry.effective_reputation(&keeper).score_bps, 10_000);

    goto_ledger(&s.env, origin + half_life);
    let at_first_boundary = s.registry.effective_reputation(&keeper);
    assert_eq!(at_first_boundary.score_bps, 5_000);
    assert_eq!(s.registry.effective_reputation(&keeper), at_first_boundary);

    goto_ledger(&s.env, origin + 2 * half_life - 1);
    assert_eq!(s.registry.effective_reputation(&keeper).score_bps, 5_000);

    goto_ledger(&s.env, origin + 2 * half_life);
    assert_eq!(s.registry.effective_reputation(&keeper).score_bps, 2_500);

    // Read-time decay never changes the persisted history or base score.
    assert_eq!(s.registry.keeper_reputation(&keeper), stored);
}

#[test]
fn effective_reputation_for_untracked_keeper_is_zero_at_any_ledger() {
    let s = setup_long_lived();
    let keeper = Address::generate(&s.env);
    advance(
        &s.env,
        5 * crate::reputation::REPUTATION_DECAY_HALF_LIFE_LEDGERS,
        0,
    );
    assert_eq!(
        s.registry.effective_reputation(&keeper),
        crate::ReputationRecord::zero()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Reputation update events
// ─────────────────────────────────────────────────────────────────────────────

fn execute_as(s: &TestSetup, keeper: &Address) {
    let task_id = register_default_task(s);
    s.registry.claim_task(keeper, &task_id);
    s.registry
        .execute_task(keeper, &task_id, &Bytes::from_slice(&s.env, b"proof"));
}

/// Claims a task as `keeper` and lets another keeper take it over once the
/// lock lapses, recording one missed claim against `keeper`.
fn miss_as(s: &TestSetup, keeper: &Address) {
    let other = Address::generate(&s.env);
    let (task_id, unlock_at) = claim_with_lock(s, keeper, 120);
    goto_ledger(&s.env, unlock_at);
    s.registry.claim_task(&other, &task_id);
}

/// `(keeper, action, score_bps)` for every `("rep", "keeper")` event the last
/// invocation emitted, in order.
fn reputation_events(s: &TestSetup) -> std::vec::Vec<(Address, Symbol, u32)> {
    let expected_topics: soroban_sdk::Vec<Val> =
        (symbol_short!("rep"), symbol_short!("keeper")).into_val(&s.env);
    s.env
        .events()
        .all()
        .iter()
        .filter(|(contract, topics, _)| {
            *contract == s.registry.address && *topics == expected_topics
        })
        .map(|(_, _, data)| data.try_into_val(&s.env).unwrap())
        .collect()
}

#[test]
fn reputation_event_fires_on_success_with_resulting_score() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    execute_as(&s, &keeper);

    assert_eq!(
        reputation_events(&s),
        std::vec![(keeper, symbol_short!("success"), 10_000)]
    );
}

#[test]
fn reputation_event_fires_on_missed_claim_with_resulting_score() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    execute_as(&s, &keeper);
    miss_as(&s, &keeper);

    // Only the keeper that missed its window is updated; the keeper taking
    // the task over has done nothing reputation-relevant yet.
    assert_eq!(
        reputation_events(&s),
        std::vec![(keeper.clone(), symbol_short!("missed"), 5_000)]
    );
    assert_eq!(s.registry.keeper_reputation(&keeper).score_bps, 5_000);
}

#[test]
fn reputation_event_does_not_fire_without_a_record_change() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let (task_id, _) = claim_with_lock(&s, &keeper, 120);
    assert!(
        reputation_events(&s).is_empty(),
        "a first claim is not tracked"
    );

    let other = Address::generate(&s.env);
    assert_eq!(
        s.registry.try_claim_task(&other, &task_id),
        Err(Ok(KeeperError::LockPeriodActive))
    );
    assert!(
        reputation_events(&s).is_empty(),
        "a rejected takeover is not tracked"
    );
}
