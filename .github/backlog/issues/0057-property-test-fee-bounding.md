---
title: "test(property): assert fee bounding (I-4) across arbitrary reward and fee_bps values"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Invariant I-4: the protocol never takes more than `fee_bps` of a reward, and the admin can never withdraw more than has accrued. The fee is floored by integer division, so the protocol may take marginally *less* than the nominal rate — never more. This issue is the property test that pins that exact rounding direction across the full input space, generalizing the fixed examples already in `test.rs` (e.g. `test_execute_task_credits_keeper_net_of_fee`).

## Expected behaviour

For any `reward` in the valid registration range and any `fee_bps` in `[0, 10_000]`:
- `keeper_net + fee == reward` exactly (see also 0053, which asserts this at the `execute_task` level — this issue tests `split_reward` directly and more exhaustively, since it's a pure function and cheap to fuzz at scale).
- `fee <= reward * fee_bps / 10_000` (the floor never rounds up).
- `fee >= 0` and `keeper_net >= 0` for all valid inputs (no negative shares).

Separately: `sweep_fees(admin, treasury, amount)` never succeeds for `amount > fees_accrued()`, for any accrual history the property generates.

## Suggested approach

`split_reward` is already a standalone function taking `(i128, u32)` — this is a natural target for a lightweight `proptest!` block with no `Env` involved at all for the arithmetic half, keeping this fast enough to run on every `cargo test`. The `sweep_fees` bound needs the full contract harness.

## Acceptance criteria

- [ ] The arithmetic property (`split_reward`) runs unconditionally on every `cargo test`, not gated behind a slow feature flag.
- [ ] The `sweep_fees` bound property covers at least: sweeping immediately after one execution, after several, and after a partial sweep followed by more accrual.
- [ ] Both properties reference `I-4`.

## Files

- `contracts/keeper-registry/src/test.rs`
