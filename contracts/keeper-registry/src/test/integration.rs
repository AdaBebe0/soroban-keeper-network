//! End-to-end flows across several tasks and keepers.

use soroban_sdk::{testutils::Address as _, token, Address, Bytes};

use super::common::*;

// ─────────────────────────────────────────────────────────────────────────────
// initialize
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// End-to-end integration: multiple tasks, multiple keepers
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_multi_keeper_end_to_end_conserves_funds() {
    let s = setup();
    let token = token::Client::new(&s.env, &s.token_id);
    let k1 = Address::generate(&s.env);
    let k2 = Address::generate(&s.env);

    // Three tasks funded from admin, 1_000_000 each.
    let t_exec = register_default_task(&s); // will be executed by k1
    let t_expire = register_default_task(&s); // will be claimed by k2 then expire
    let t_cancel = register_default_task(&s); // will be cancelled by owner

    // The contract now escrows all three rewards.
    assert_eq!(token.balance(&s.registry.address), 3_000_000i128);

    // k1 executes the first task (3% fee → 970_000 to k1, 30_000 accrued).
    s.registry.claim_task(&k1, &t_exec);
    s.registry
        .execute_task(&k1, &t_exec, &Bytes::from_slice(&s.env, b"p1"));

    // k2 claims the second but never executes; owner cancels the third now.
    s.registry.claim_task(&k2, &t_expire);
    s.registry.cancel_task(&s.admin, &t_cancel); // refunds 1_000_000

    // Time passes; the abandoned task is expired permissionlessly.
    advance(&s.env, 200, 3_601);
    s.registry.expire_task(&t_expire); // refunds 1_000_000 to owner

    // k1 withdraws its earnings; admin sweeps the fee.
    assert_eq!(s.registry.withdraw_rewards(&k1), 970_000i128);
    let treasury = Address::generate(&s.env);
    s.registry.sweep_fees(&s.admin, &treasury, &30_000i128);

    // Conservation: the contract should hold nothing left over — every stroop
    // is now either with the keeper, the treasury, or refunded to the owner.
    assert_eq!(token.balance(&s.registry.address), 0i128);
    assert_eq!(token.balance(&k1), 970_000i128);
    assert_eq!(token.balance(&treasury), 30_000i128);
    assert_eq!(s.registry.fees_accrued(), 0i128);
}
