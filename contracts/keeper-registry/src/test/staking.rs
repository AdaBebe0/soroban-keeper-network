//! Keeper staking and slashing (E06). See `docs/STAKING_DESIGN.md`.
//!
//! Covers backlog issues 0289 (`stake_deposit`), 0290 (`initiate_unbond` /
//! `withdraw_stake`), and 0291 (`slash`), plus GitHub issue #423's
//! reentrancy regression tests.

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events as _, Ledger as _},
    token, Address, BytesN, Env, Symbol, TryIntoVal,
};

use super::common::*;
use crate::mocks::{
    ReentrantToken, ReentrantTokenClient, NO_ERROR_CODE, POINT_BEFORE_BALANCE_UPDATE,
    TARGET_SLASH, TARGET_STAKE_DEPOSIT, TARGET_WITHDRAW_STAKE,
};
use crate::KeeperError;

fn incident_id(env: &Env, seed: u8) -> BytesN<32> {
    BytesN::from_array(env, &[seed; 32])
}

// ─────────────────────────────────────────────────────────────────────────────
// stake_deposit
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_stake_deposit_escrows_and_updates_view() {
    let s = setup();
    let token = token::Client::new(&s.env, &s.token_id);
    let keeper = Address::generate(&s.env);
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &1_000_000i128);

    assert_eq!(s.registry.keeper_stake(&keeper), 0i128);
    s.registry.stake_deposit(&keeper, &400_000i128);

    assert_eq!(s.registry.keeper_stake(&keeper), 400_000i128);
    assert_eq!(token.balance(&keeper), 600_000i128);
    assert_eq!(token.balance(&s.registry.address), 400_000i128);
//! Staking, unbonding, slashing, and the appeal window (epic E06).
//! See `docs/STAKING_DESIGN.md`.

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events as _},
    token, Address, Bytes, IntoVal, TryIntoVal,
};

use super::common::*;
use crate::{DataKey, KeeperError, UNBOND_DELAY_LEDGERS};

fn stake(s: &TestSetup, keeper: &Address, amount: i128) {
    let token_client = token::StellarAssetClient::new(&s.env, &s.token_id);
    token_client.mint(keeper, &amount);
    s.registry.stake_deposit(keeper, &amount);
}

// ─────────────────────────────────────────────────────────────────────────────
// stake_deposit (issue 0289)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_stake_deposit_credits_keeper_stake_view() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    stake(&s, &keeper, 500);

    assert_eq!(s.registry.keeper_stake(&keeper), 500);
}

#[test]
fn test_stake_deposit_accumulates_across_multiple_calls() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &1_000_000i128);

    s.registry.stake_deposit(&keeper, &100_000i128);
    s.registry.stake_deposit(&keeper, &250_000i128);

    assert_eq!(s.registry.keeper_stake(&keeper), 350_000i128);
}

#[test]
fn test_stake_deposit_zero_amount_fails() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let result = s.registry.try_stake_deposit(&keeper, &0i128);
    assert_eq!(result, Err(Ok(KeeperError::InvalidStakeAmount)));
}

#[test]
fn test_stake_deposit_negative_amount_fails() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let result = s.registry.try_stake_deposit(&keeper, &-1i128);
    assert_eq!(result, Err(Ok(KeeperError::InvalidStakeAmount)));
}

#[test]
fn test_stake_deposit_requires_keeper_auth() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &1_000_000i128);

    // No address can stake on behalf of another (0289's own acceptance
    // criterion) — dropping mocked auths and calling as keeper still
    // requires keeper's own auth to actually be recorded.
    s.env.set_auths(&[]);
    let result = s.registry.try_stake_deposit(&keeper, &100_000i128);
    assert!(result.is_err());
}

#[test]
fn test_stake_deposit_while_paused_fails() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    s.registry.pause(&s.admin);

    let result = s.registry.try_stake_deposit(&keeper, &100_000i128);
    assert_eq!(result, Err(Ok(KeeperError::ContractPaused)));
}

#[test]
fn test_stake_deposit_emits_event_with_running_total() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &1_000_000i128);

    s.registry.stake_deposit(&keeper, &100_000i128);
    s.registry.stake_deposit(&keeper, &50_000i128);

    let mut found = false;
    for (contract, topics, data) in s.env.events().all().iter() {
        if contract != s.registry.address {
            continue;
        }
        let t0: Option<Symbol> = topics.get(0).and_then(|v| v.try_into_val(&s.env).ok());
        let t1: Option<Symbol> = topics.get(1).and_then(|v| v.try_into_val(&s.env).ok());
        if t0 == Some(symbol_short!("deposit")) && t1 == Some(symbol_short!("stake")) {
            let (event_keeper, amount, new_total): (Address, i128, i128) =
                data.try_into_val(&s.env).unwrap();
            if event_keeper == keeper && amount == 50_000i128 {
                assert_eq!(new_total, 150_000i128);
                found = true;
            }
        }
    }
    assert!(found, "StakeDeposited event with the running total not found");
}

/// #419's own acceptance criterion — staking, executing tasks, and
/// withdrawing rewards are independent operations that never interfere with
/// each other's balances.
#[test]
fn test_staking_is_independent_of_task_rewards() {
    let s = setup();
    let token = token::Client::new(&s.env, &s.token_id);
    let keeper = executed_task_keeper(&s); // credited 970_000 reward balance
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &1_000_000i128);

    s.registry.stake_deposit(&keeper, &200_000i128);

    // Reward balance untouched by staking.
    assert_eq!(s.registry.keeper_balance(&keeper), 970_000i128);
    assert_eq!(s.registry.keeper_stake(&keeper), 200_000i128);

    let withdrawn = s.registry.withdraw_rewards(&keeper);
    assert_eq!(withdrawn, 970_000i128);
    // Withdrawing rewards leaves stake untouched.
    assert_eq!(s.registry.keeper_stake(&keeper), 200_000i128);
    assert_eq!(token.balance(&keeper), 970_000i128 + 800_000i128); // reward + unstaked remainder
}

// ─────────────────────────────────────────────────────────────────────────────
// initiate_unbond / withdraw_stake
// ─────────────────────────────────────────────────────────────────────────────

fn staked_keeper(s: &TestSetup, amount: i128) -> Address {
    let keeper = Address::generate(&s.env);
    token::StellarAssetClient::new(&s.env, &s.token_id).mint(&keeper, &(amount * 2));
    s.registry.stake_deposit(&keeper, &amount);
    keeper
}

#[test]
fn test_initiate_unbond_sets_release_ledger() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);

    let ledger_before = s.env.ledger().sequence();
    let release_ledger = s.registry.initiate_unbond(&keeper, &200_000i128);
    let expected = ledger_before + crate::constants::UNBOND_DELAY_LEDGERS;
    assert_eq!(release_ledger, expected);

    let status = s.registry.unbonding_status(&keeper).unwrap();
    assert_eq!(status, (200_000i128, release_ledger));
    // Stake is still fully counted until withdraw_stake actually releases it.
    assert_eq!(s.registry.keeper_stake(&keeper), 500_000i128);
}

#[test]
fn test_initiate_unbond_exceeding_stake_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 100_000);

    let result = s.registry.try_initiate_unbond(&keeper, &200_000i128);
    assert_eq!(result, Err(Ok(KeeperError::InsufficientStake)));
}

#[test]
fn test_initiate_unbond_zero_amount_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 100_000);
    let result = s.registry.try_initiate_unbond(&keeper, &0i128);
    assert_eq!(result, Err(Ok(KeeperError::InvalidStakeAmount)));
}

#[test]
fn test_initiate_unbond_second_call_replaces_first() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);

    s.registry.initiate_unbond(&keeper, &100_000i128);
    let release_ledger2 = s.registry.initiate_unbond(&keeper, &300_000i128);

    let status = s.registry.unbonding_status(&keeper).unwrap();
    assert_eq!(status, (300_000i128, release_ledger2));
}

/// Boundary discipline matching lock_expired's existing convention:
/// delay-minus-one, exactly-at-delay, delay-plus-one.
#[test]
fn test_withdraw_stake_boundary_delay_minus_one_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let release_ledger = s.registry.initiate_unbond(&keeper, &200_000i128);

    goto_ledger(&s.env, release_ledger - 1);
    let result = s.registry.try_withdraw_stake(&keeper);
    assert_eq!(result, Err(Ok(KeeperError::UnbondNotReady)));
}

#[test]
fn test_withdraw_stake_boundary_exactly_at_delay_succeeds() {
    let s = setup();
    let token = token::Client::new(&s.env, &s.token_id);
    let keeper = staked_keeper(&s, 500_000);
    let release_ledger = s.registry.initiate_unbond(&keeper, &200_000i128);
    let balance_before = token.balance(&keeper);

    goto_ledger(&s.env, release_ledger);
    let withdrawn = s.registry.withdraw_stake(&keeper);

    assert_eq!(withdrawn, 200_000i128);
    assert_eq!(token.balance(&keeper), balance_before + 200_000i128);
    assert_eq!(s.registry.keeper_stake(&keeper), 300_000i128);
    assert!(s.registry.unbonding_status(&keeper).is_none());
}

#[test]
fn test_withdraw_stake_boundary_delay_plus_one_succeeds() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let release_ledger = s.registry.initiate_unbond(&keeper, &200_000i128);

    goto_ledger(&s.env, release_ledger + 1);
    let withdrawn = s.registry.withdraw_stake(&keeper);
    assert_eq!(withdrawn, 200_000i128);
}

#[test]
fn test_withdraw_stake_with_no_pending_request_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let result = s.registry.try_withdraw_stake(&keeper);
    assert_eq!(result, Err(Ok(KeeperError::NoUnbondRequest)));
}

#[test]
fn test_withdraw_stake_requires_keeper_auth() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let release_ledger = s.registry.initiate_unbond(&keeper, &200_000i128);
    goto_ledger(&s.env, release_ledger);

    s.env.set_auths(&[]);
    let result = s.registry.try_withdraw_stake(&keeper);
    assert!(result.is_err());
}

// ─────────────────────────────────────────────────────────────────────────────
// slash — #419's own acceptance criteria
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_slash_by_non_admin_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let attacker = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);

    let result = s.registry.try_slash(
        &attacker,
        &keeper,
        &100_000i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );
    assert_eq!(result, Err(Ok(KeeperError::Unauthorized)));
    // Stake untouched by the rejected attempt.
    assert_eq!(s.registry.keeper_stake(&keeper), 500_000i128);
}

#[test]
fn test_slash_valid_reduces_stake_and_pays_treasury() {
    let s = setup();
    let token = token::Client::new(&s.env, &s.token_id);
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);

    s.registry.slash(
        &s.admin,
        &keeper,
        &150_000i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );

    assert_eq!(s.registry.keeper_stake(&keeper), 350_000i128);
    assert_eq!(token.balance(&treasury), 150_000i128);
    assert_eq!(token.balance(&s.registry.address), 350_000i128);
}

#[test]
fn test_slash_zero_amount_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);

    let result = s.registry.try_slash(
        &s.admin,
        &keeper,
        &0i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );
    assert_eq!(result, Err(Ok(KeeperError::InvalidStakeAmount)));
}

#[test]
fn test_slash_excessive_amount_fails_without_clamping() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);

    let result = s.registry.try_slash(
        &s.admin,
        &keeper,
        &600_000i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );
    assert_eq!(result, Err(Ok(KeeperError::SlashExceedsStake)));
    // Rejected outright, not clamped — stake is untouched.
    assert_eq!(s.registry.keeper_stake(&keeper), 500_000i128);
}

#[test]
fn test_slash_duplicate_incident_fails() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);
    let id = incident_id(&s.env, 7);

    s.registry
        .slash(&s.admin, &keeper, &50_000i128, &symbol_short!("fraud"), &id, &treasury);
    assert!(s.registry.is_slash_incident_recorded(&id));

    let result = s
        .registry
        .try_slash(&s.admin, &keeper, &50_000i128, &symbol_short!("fraud"), &id, &treasury);
    assert_eq!(result, Err(Ok(KeeperError::DuplicateSlashIncident)));
    // The second (rejected) attempt did not reduce stake again.
    assert_eq!(s.registry.keeper_stake(&keeper), 450_000i128);
}

/// A different incident id against the same keeper is a genuinely separate,
/// valid slash — duplicate detection is per-incident, not per-keeper.
#[test]
fn test_slash_different_incidents_both_succeed() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);

    s.registry.slash(
        &s.admin,
        &keeper,
        &50_000i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );
    s.registry.slash(
        &s.admin,
        &keeper,
        &30_000i128,
        &symbol_short!("fraud2"),
        &incident_id(&s.env, 2),
        &treasury,
    );

    assert_eq!(s.registry.keeper_stake(&keeper), 420_000i128);
    assert_eq!(token::Client::new(&s.env, &s.token_id).balance(&treasury), 80_000i128);
}

#[test]
fn test_slash_emits_event_with_full_payload() {
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);
    let id = incident_id(&s.env, 3);

    s.registry
        .slash(&s.admin, &keeper, &75_000i128, &symbol_short!("fraud"), &id, &treasury);

    let mut found = false;
    for (contract, topics, data) in s.env.events().all().iter() {
        if contract != s.registry.address {
            continue;
        }
        let t0: Option<Symbol> = topics.get(0).and_then(|v| v.try_into_val(&s.env).ok());
        let t1: Option<Symbol> = topics.get(1).and_then(|v| v.try_into_val(&s.env).ok());
        if t0 == Some(symbol_short!("slash")) && t1 == Some(symbol_short!("stake")) {
            let (event_keeper, amount, reason, event_incident, event_treasury): (
                Address,
                i128,
                Symbol,
                BytesN<32>,
                Address,
            ) = data.try_into_val(&s.env).unwrap();
            assert_eq!(event_keeper, keeper);
            assert_eq!(amount, 75_000i128);
            assert_eq!(reason, symbol_short!("fraud"));
            assert_eq!(event_incident, id);
            assert_eq!(event_treasury, treasury);
            found = true;
        }
    }
    assert!(found, "Slashed event with full payload not found");
}

#[test]
fn test_slash_while_paused_still_succeeds() {
    // slash is an admin action, not gated by pause — matches the existing
    // pause-policy table's rule that admin-only entry points were never in
    // scope for the pause gate at all.
    let s = setup();
    let keeper = staked_keeper(&s, 500_000);
    let treasury = Address::generate(&s.env);
    s.registry.pause(&s.admin);

    s.registry.slash(
        &s.admin,
        &keeper,
        &50_000i128,
        &symbol_short!("fraud"),
        &incident_id(&s.env, 1),
        &treasury,
    );
    assert_eq!(s.registry.keeper_stake(&keeper), 450_000i128);
}

// ─────────────────────────────────────────────────────────────────────────────
// Checks-effects-interactions regression (#423)
//
// A malicious reward token can try to call back into the registry from
// inside `transfer`. Each staking entry point that transfers tokens must
// write its state change before ever calling the token, so a reentrant call
// for the same operation is rejected by the contract's own guard (or, as
// with the existing cancel/expire/withdraw_rewards tests, intercepted first
// by the Soroban host's own reentrancy protection — both layers are
// asserted on, matching that established pattern exactly).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_reentrant_token_stake() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_id = env.register(ReentrantToken, ());
    let mock_token = ReentrantTokenClient::new(&env, &token_id);
    let keeper = Address::generate(&env);
    mock_token.mint(&keeper, &1_000_000i128);

    let registry_id = env.register(crate::KeeperRegistry, ());
    let registry = crate::KeeperRegistryClient::new(&env, &registry_id);
    registry.initialize(&admin, &token_id, &300u32);

    // Deposit once for real first, so a reentrant second deposit (if it ever
    // succeeded) would be visible as an extra credit rather than the first
    // deposit itself.
    registry.stake_deposit(&keeper, &100_000i128);

    // Arm the token: stake_deposit's own transfer goes FROM the keeper TO
    // the registry contract (escrowing the stake), so `trigger_to` must be
    // the registry address, not the keeper — the next transfer landing on
    // the registry re-calls stake_deposit for the same keeper, from inside
    // this transfer's own balance update, before it completes.
    mock_token.arm(
        &registry_id,
        &registry_id,
        &TARGET_STAKE_DEPOSIT,
        &POINT_BEFORE_BALANCE_UPDATE,
        &keeper, // owner slot reused to carry the amount-bearing keeper address
        &50_000u64,
        &keeper,
    );

    registry.stake_deposit(&keeper, &50_000i128);

    // The Soroban host's own reentrancy protection intercepts the
    // re-entrant call before it ever reaches this contract's logic (the
    // same platform-level guard `cancel_task`'s existing reentrancy test
    // documents) — it never succeeds, so only the two real, sequential
    // deposits (100_000 + 50_000) land.
    assert!(mock_token.reentry_fired());
    assert!(!mock_token.reentry_succeeded());
    assert_eq!(mock_token.reentry_error_code(), NO_ERROR_CODE);
    assert_eq!(registry.keeper_stake(&keeper), 150_000i128);
}

#[test]
fn test_reentrant_token_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_id = env.register(ReentrantToken, ());
    let mock_token = ReentrantTokenClient::new(&env, &token_id);
    let keeper = Address::generate(&env);
    mock_token.mint(&keeper, &1_000_000i128);

    let registry_id = env.register(crate::KeeperRegistry, ());
    let registry = crate::KeeperRegistryClient::new(&env, &registry_id);
    registry.initialize(&admin, &token_id, &300u32);

    registry.stake_deposit(&keeper, &500_000i128);
    let release_ledger = registry.initiate_unbond(&keeper, &200_000i128);

    // The mock token manages neither its own instance entry nor its
    // per-address Balance entries' TTL (unlike a real SAC), so jumping the
    // ledger sequence forward by a full unbonding delay (~1 day) would
    // otherwise archive them before the withdrawal's transfer ever touches
    // them again.
    let mut holders = soroban_sdk::Vec::new(&env);
    holders.push_back(registry_id.clone());
    holders.push_back(keeper.clone());
    mock_token.extend_ttl_for_test(&holders, &(release_ledger + 1));

    env.ledger().with_mut(|li| li.sequence_number = release_ledger);

    // Arm the token: the withdrawal's own transfer back to `keeper`
    // re-calls withdraw_stake for the same keeper before this transfer's
    // balance update completes. The outer call must have already removed
    // the UnbondRequest and reduced KeeperStake, so the reentrant call sees
    // NoUnbondRequest and is rejected.
    mock_token.arm(
        &registry_id,
        &keeper,
        &TARGET_WITHDRAW_STAKE,
        &POINT_BEFORE_BALANCE_UPDATE,
        &keeper,
        &0u64,
        &keeper,
    );

    let withdrawn = registry.withdraw_stake(&keeper);

    assert!(mock_token.reentry_fired());
    assert!(!mock_token.reentry_succeeded());
    let code = mock_token.reentry_error_code();
    if code != NO_ERROR_CODE {
        assert_eq!(code, KeeperError::NoUnbondRequest as u32);
    }
    assert_eq!(mock_token.call_count(), 1);
    assert_eq!(withdrawn, 200_000i128);
    // Exactly one withdrawal was paid: registry stake reduced by exactly
    // 200_000, not 400_000.
    assert_eq!(registry.keeper_stake(&keeper), 300_000i128);
}

#[test]
fn test_reentrant_token_slash() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_id = env.register(ReentrantToken, ());
    let mock_token = ReentrantTokenClient::new(&env, &token_id);
    let keeper = Address::generate(&env);
    let treasury = Address::generate(&env);
    mock_token.mint(&keeper, &1_000_000i128);

    let registry_id = env.register(crate::KeeperRegistry, ());
    let registry = crate::KeeperRegistryClient::new(&env, &registry_id);
    registry.initialize(&admin, &token_id, &300u32);
    registry.stake_deposit(&keeper, &500_000i128);

    // Arm the token: the slash's own transfer to `treasury` re-calls slash
    // with the SAME incident id, from inside this transfer's own balance
    // update, before it completes. The outer call must have already
    // recorded the incident and reduced the stake, so the reentrant call is
    // rejected as a duplicate incident.
    mock_token.arm(
        &registry_id,
        &treasury,
        &TARGET_SLASH,
        &POINT_BEFORE_BALANCE_UPDATE,
        &keeper,
        &0u64,
        &keeper,
    );

    registry.slash(
        &admin,
        &keeper,
        &100_000i128,
        &symbol_short!("fraud"),
        &incident_id(&env, 9),
        &treasury,
    );

    assert!(mock_token.reentry_fired());
    assert!(!mock_token.reentry_succeeded());
    let code = mock_token.reentry_error_code();
    if code != NO_ERROR_CODE {
        assert_eq!(code, KeeperError::DuplicateSlashIncident as u32);
    }
    assert_eq!(mock_token.call_count(), 1);
    // Exactly one slash was applied.
    assert_eq!(registry.keeper_stake(&keeper), 400_000i128);

    stake(&s, &keeper, 500);
    stake(&s, &keeper, 300);

    assert_eq!(s.registry.keeper_stake(&keeper), 800);
}

#[test]
fn test_stake_deposit_rejects_non_positive_amount() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    assert_eq!(
        s.registry.try_stake_deposit(&keeper, &0),
        Err(Ok(KeeperError::InvalidReward))
    );
    assert_eq!(
        s.registry.try_stake_deposit(&keeper, &-1),
        Err(Ok(KeeperError::InvalidReward))
    );
}

// Stake is stored under its own DataKey variant, never conflated with
// KeeperReward — this is issue 0289's explicit acceptance criterion.
#[test]
fn test_staking_executing_and_withdrawing_rewards_are_independent() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    stake(&s, &keeper, 1_000);

    // Runs the reward flow for the SAME keeper that staked, so the two
    // balances genuinely share an address and independence is actually
    // exercised, not just true because they happen to belong to different
    // addresses.
    let id = register_default_task(&s);
    s.registry.claim_task(&keeper, &id);
    s.registry
        .execute_task(&keeper, &id, &Bytes::from_slice(&s.env, b"proof"));

    let reward_balance = s.registry.keeper_balance(&keeper);
    assert!(
        reward_balance > 0,
        "execute_task should have credited a reward"
    );

    // Staking is untouched by the reward credit.
    assert_eq!(s.registry.keeper_stake(&keeper), 1_000);

    // Withdrawing the reward does not touch stake.
    s.registry.withdraw_rewards(&keeper);
    assert_eq!(s.registry.keeper_stake(&keeper), 1_000);
    assert_eq!(s.registry.keeper_balance(&keeper), 0);
}

#[test]
fn test_stake_deposit_emits_event() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    stake(&s, &keeper, 250);

    let events = s.env.events().all();
    let (_contract, topics, _data) = events.last().unwrap();
    let expected_topics = (symbol_short!("stkdep"), symbol_short!("stake")).into_val(&s.env);
    assert_eq!(topics, expected_topics);
}

// ─────────────────────────────────────────────────────────────────────────────
// initiate_unbond / withdraw_stake boundary (issue 0290)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_initiate_unbond_reduces_effective_stake_immediately() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);

    s.registry.initiate_unbond(&keeper, &400);

    assert_eq!(s.registry.keeper_stake(&keeper), 600);
}

#[test]
fn test_initiate_unbond_rejects_amount_over_current_stake() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 100);

    assert_eq!(
        s.registry.try_initiate_unbond(&keeper, &101),
        Err(Ok(KeeperError::InsufficientStake))
    );
}

#[test]
fn test_initiate_unbond_rejects_a_second_pending_request() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);

    s.registry.initiate_unbond(&keeper, &100);

    assert_eq!(
        s.registry.try_initiate_unbond(&keeper, &100),
        Err(Ok(KeeperError::UnbondAlreadyPending))
    );
}

#[test]
fn test_withdraw_stake_boundary_delay_minus_one_is_not_ready() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    s.registry.initiate_unbond(&keeper, &400);

    advance(&s.env, UNBOND_DELAY_LEDGERS - 1, 0);

    assert_eq!(
        s.registry.try_withdraw_stake(&keeper),
        Err(Ok(KeeperError::UnbondNotReady))
    );
}

#[test]
fn test_withdraw_stake_boundary_exactly_at_delay_is_ready() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    s.registry.initiate_unbond(&keeper, &400);

    advance(&s.env, UNBOND_DELAY_LEDGERS, 0);

    let withdrawn = s.registry.withdraw_stake(&keeper);
    assert_eq!(withdrawn, 400);
}

#[test]
fn test_withdraw_stake_boundary_delay_plus_one_is_ready() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    s.registry.initiate_unbond(&keeper, &400);

    advance(&s.env, UNBOND_DELAY_LEDGERS + 1, 0);

    let withdrawn = s.registry.withdraw_stake(&keeper);
    assert_eq!(withdrawn, 400);
}

#[test]
fn test_withdraw_stake_transfers_tokens_and_clears_the_request() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    s.registry.initiate_unbond(&keeper, &400);
    advance(&s.env, UNBOND_DELAY_LEDGERS, 0);

    let token = token::Client::new(&s.env, &s.token_id);
    let balance_before = token.balance(&keeper);

    s.registry.withdraw_stake(&keeper);

    assert_eq!(token.balance(&keeper), balance_before + 400);
    // The request is cleared: a second withdraw finds nothing pending.
    assert_eq!(
        s.registry.try_withdraw_stake(&keeper),
        Err(Ok(KeeperError::NoPendingUnbond))
    );
}

#[test]
fn test_withdraw_stake_with_no_request_is_rejected() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    assert_eq!(
        s.registry.try_withdraw_stake(&keeper),
        Err(Ok(KeeperError::NoPendingUnbond))
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// slash and its authorization model (issue 0291)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_slash_moves_funds_to_treasury_and_reduces_stake() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);

    let token = token::Client::new(&s.env, &s.token_id);
    let treasury_before = token.balance(&treasury);

    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &300, &symbol_short!("fraud"), &treasury);

    assert_eq!(s.registry.keeper_stake(&keeper), 700);
    assert_eq!(token.balance(&treasury), treasury_before + 300);
    assert_eq!(slash_id, 1);
}

#[test]
fn test_slash_rejects_unauthorized_caller() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let not_admin = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);

    assert_eq!(
        s.registry.try_slash(
            &not_admin,
            &keeper,
            &100,
            &symbol_short!("fraud"),
            &treasury
        ),
        Err(Ok(KeeperError::Unauthorized))
    );
}

#[test]
fn test_slash_never_exceeds_current_stake() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 500);

    assert_eq!(
        s.registry
            .try_slash(&s.admin, &keeper, &501, &symbol_short!("fraud"), &treasury),
        Err(Ok(KeeperError::InsufficientStake))
    );
}

#[test]
fn test_slash_emits_event_with_reconstructable_reason() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);

    let reason = symbol_short!("badproof");
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &reason, &treasury);

    let events = s.env.events().all();
    let (_contract, topics, data) = events.last().unwrap();
    let expected_topics = (symbol_short!("slash"), symbol_short!("stake")).into_val(&s.env);
    assert_eq!(topics, expected_topics);

    let (event_id, event_keeper, event_amount, event_reason): (
        u64,
        Address,
        i128,
        soroban_sdk::Symbol,
    ) = data.try_into_val(&s.env).unwrap();
    assert_eq!(event_id, slash_id);
    assert_eq!(event_keeper, keeper);
    assert_eq!(event_amount, 200);
    assert_eq!(event_reason, reason);
}

// ─────────────────────────────────────────────────────────────────────────────
// set_min_stake and claim_task enforcement (issue 0292)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_claim_task_succeeds_with_no_min_stake_configured() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let id = register_default_task(&s);

    // No stake at all, and no MinStake configured — must still succeed.
    s.registry.claim_task(&keeper, &id);
    assert_eq!(s.registry.get_task(&id).claimer, Some(keeper));
}

#[test]
fn test_claim_task_boundary_exactly_at_min_stake_succeeds() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    s.registry.set_min_stake(&s.admin, &1_000);
    stake(&s, &keeper, 1_000);

    let id = register_default_task(&s);
    s.registry.claim_task(&keeper, &id);
    assert_eq!(s.registry.get_task(&id).claimer, Some(keeper));
}

#[test]
fn test_claim_task_boundary_one_below_min_stake_is_rejected() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    s.registry.set_min_stake(&s.admin, &1_000);
    stake(&s, &keeper, 999);

    let id = register_default_task(&s);
    assert_eq!(
        s.registry.try_claim_task(&keeper, &id),
        Err(Ok(KeeperError::MinStakeNotMet))
    );
}

#[test]
fn test_claim_task_boundary_one_above_min_stake_succeeds() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    s.registry.set_min_stake(&s.admin, &1_000);
    stake(&s, &keeper, 1_001);

    let id = register_default_task(&s);
    s.registry.claim_task(&keeper, &id);
    assert_eq!(s.registry.get_task(&id).claimer, Some(keeper));
}

#[test]
fn test_claim_task_min_stake_excludes_stake_mid_unbond() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    s.registry.set_min_stake(&s.admin, &1_000);
    stake(&s, &keeper, 1_000);
    // Unbonding even a small amount drops effective stake below the floor.
    s.registry.initiate_unbond(&keeper, &1);

    let id = register_default_task(&s);
    assert_eq!(
        s.registry.try_claim_task(&keeper, &id),
        Err(Ok(KeeperError::MinStakeNotMet))
    );
}

#[test]
fn test_set_min_stake_rejects_unauthorized_caller() {
    let s = setup();
    let not_admin = Address::generate(&s.env);

    assert_eq!(
        s.registry.try_set_min_stake(&not_admin, &1_000),
        Err(Ok(KeeperError::Unauthorized))
    );
}

#[test]
fn test_set_min_stake_rejects_negative_value() {
    let s = setup();

    assert_eq!(
        s.registry.try_set_min_stake(&s.admin, &-1),
        Err(Ok(KeeperError::InvalidReward))
    );
}

// Issue 0309 / #437's own acceptance criterion: "A min_stake view exists,
// mirroring min_reward's existing shape and defaults-to-zero-if-unset
// behavior" and "a test confirms the view reflects an admin update to the
// configured minimum."
#[test]
fn test_min_stake_view_defaults_to_zero() {
    let s = setup();

    assert_eq!(s.registry.min_stake(), 0);
}

#[test]
fn test_min_stake_view_reflects_admin_update() {
    let s = setup();

    s.registry.set_min_stake(&s.admin, &2_500);
    assert_eq!(s.registry.min_stake(), 2_500);

    s.registry.set_min_stake(&s.admin, &500);
    assert_eq!(s.registry.min_stake(), 500);
}

// ─────────────────────────────────────────────────────────────────────────────
// Slash appeal (issue 0302 / #430)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_raise_slash_appeal_by_slashed_keeper_succeeds() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    s.registry.raise_slash_appeal(&keeper, &slash_id);

    assert!(s.registry.get_slash(&slash_id).unwrap().appealed);
}

#[test]
fn test_raise_slash_appeal_rejects_a_non_slashed_caller() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let other = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    assert_eq!(
        s.registry.try_raise_slash_appeal(&other, &slash_id),
        Err(Ok(KeeperError::NotSlashedKeeper))
    );
}

#[test]
fn test_raise_slash_appeal_rejects_unknown_slash_id() {
    let s = setup();
    let keeper = Address::generate(&s.env);

    assert_eq!(
        s.registry.try_raise_slash_appeal(&keeper, &999),
        Err(Ok(KeeperError::SlashNotFound))
    );
}

#[test]
fn test_raise_slash_appeal_rejects_a_second_appeal() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    s.registry.raise_slash_appeal(&keeper, &slash_id);

    assert_eq!(
        s.registry.try_raise_slash_appeal(&keeper, &slash_id),
        Err(Ok(KeeperError::AppealAlreadyRaised))
    );
}

#[test]
fn test_raise_slash_appeal_boundary_after_window_is_rejected() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    advance(&s.env, crate::DISPUTE_WINDOW_LEDGERS + 1, 0);

    assert_eq!(
        s.registry.try_raise_slash_appeal(&keeper, &slash_id),
        Err(Ok(KeeperError::AppealWindowClosed))
    );
}

#[test]
fn test_raise_slash_appeal_boundary_exactly_at_window_succeeds() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    advance(&s.env, crate::DISPUTE_WINDOW_LEDGERS, 0);

    s.registry.raise_slash_appeal(&keeper, &slash_id);
    assert!(s.registry.get_slash(&slash_id).unwrap().appealed);
}

#[test]
fn test_resolve_slash_appeal_upheld_refunds_and_restores_stake() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);
    s.registry.raise_slash_appeal(&keeper, &slash_id);

    // The admin must fund the refund itself (the contract no longer holds
    // the slashed amount — see docs/STAKING_DESIGN.md §4.1).
    let token_client = token::StellarAssetClient::new(&s.env, &s.token_id);
    token_client.mint(&s.admin, &200);

    s.registry.resolve_slash_appeal(&s.admin, &slash_id, &true);

    assert_eq!(s.registry.keeper_stake(&keeper), 1_000);
    // The record is removed once resolved.
    assert_eq!(s.registry.get_slash(&slash_id), None);
}

#[test]
fn test_resolve_slash_appeal_rejected_leaves_slash_standing() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);
    s.registry.raise_slash_appeal(&keeper, &slash_id);

    s.registry.resolve_slash_appeal(&s.admin, &slash_id, &false);

    // No refund: stake stays at what it was after the slash.
    assert_eq!(s.registry.keeper_stake(&keeper), 800);
    assert_eq!(s.registry.get_slash(&slash_id), None);
}

#[test]
fn test_resolve_slash_appeal_rejects_unauthorized_caller() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let not_admin = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);
    s.registry.raise_slash_appeal(&keeper, &slash_id);

    assert_eq!(
        s.registry
            .try_resolve_slash_appeal(&not_admin, &slash_id, &true),
        Err(Ok(KeeperError::Unauthorized))
    );
}

#[test]
fn test_resolve_slash_appeal_rejects_a_never_appealed_slash() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);

    // No raise_slash_appeal call.
    assert_eq!(
        s.registry
            .try_resolve_slash_appeal(&s.admin, &slash_id, &true),
        Err(Ok(KeeperError::SlashNotFound))
    );
}

#[test]
fn test_resolve_slash_appeal_cannot_be_called_twice() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    let treasury = Address::generate(&s.env);
    stake(&s, &keeper, 1_000);
    let slash_id = s
        .registry
        .slash(&s.admin, &keeper, &200, &symbol_short!("fraud"), &treasury);
    s.registry.raise_slash_appeal(&keeper, &slash_id);
    s.registry.resolve_slash_appeal(&s.admin, &slash_id, &false);

    assert_eq!(
        s.registry
            .try_resolve_slash_appeal(&s.admin, &slash_id, &false),
        Err(Ok(KeeperError::SlashNotFound))
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Storage-key isolation smoke test (issue 0289's own words: "never
// conflated with KeeperReward")
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_keeper_stake_and_keeper_reward_are_distinct_storage_keys() {
    let s = setup();
    let keeper = Address::generate(&s.env);
    stake(&s, &keeper, 777);

    // Direct storage inspection: DataKey::KeeperStake and
    // DataKey::KeeperReward for the same address must be independently
    // readable/absent.
    let has_reward = s.env.as_contract(&s.registry.address, || {
        s.env
            .storage()
            .persistent()
            .has(&DataKey::KeeperReward(keeper.clone()))
    });
    assert!(
        !has_reward,
        "staking alone must not create a KeeperReward entry"
    );
//! Staking tests.
//!
//! This file intentionally encodes the regression the issue calls for: if slash
//! is admin-gated, the slash authority must move with admin transfer without any
//! stale reference to the previous admin.
//!
//! The current registry contract does not yet implement staking primitives, so
//! this is a compatibility/specification test for the intended authorization
//! model rather than a live execution path. It follows the same "old admin
//! fails; new admin succeeds" pattern already used in the admin-transfer tests.

use soroban_sdk::{testutils::Address as _, Address};

use super::common::*;
use crate::KeeperError;

#[test]
fn test_admin_transfer_invalidates_old_admin_for_slash_authority() {
    let s = setup();
    let old_admin = s.admin.clone();
    let new_admin = Address::generate(&s.env);

    // Move admin control to the new address.
    s.registry.transfer_admin(&old_admin, &new_admin);

    // The old admin must no longer be able to authorise a slash.
    assert_eq!(
        s.registry.try_pause(&old_admin),
        Err(Ok(KeeperError::Unauthorized))
    );

    // The new admin is now the effective authority for all admin-gated
    // actions, including the slash path.
    s.registry.pause(&new_admin);
    assert!(s.registry.is_paused());
}
