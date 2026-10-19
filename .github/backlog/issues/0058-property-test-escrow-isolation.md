---
title: "test(property): assert escrow isolation (I-5) — admin functions can never touch task escrow or keeper balances"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Invariant I-5: admin functions cannot touch task escrow or credited keeper balances. `sweep_fees` is bounded by the `FeesAccrued` accumulator specifically to enforce this. This property test asserts the isolation holds under randomized interleavings of admin actions and ordinary task/keeper activity, not just the fixed scenario already covered in wave 1 (`test_sweep_partial_sequence_conserves_remainder_and_leaves_other_balances_untouched`).

## Expected behaviour

Interleave randomized `sweep_fees` calls (including attempts to oversweep) with randomized task registration, claiming, execution, and keeper withdrawals, and after every step assert:
- No open task's `reward` field ever changes as a result of a `sweep_fees` call.
- No keeper's `keeper_balance` ever changes as a result of a `sweep_fees` call.
- `set_fee_bps`, `pause`/`unpause`, `transfer_admin`, and `upgrade` never move any token balance at all.

## Suggested approach

This pairs naturally with 0054's solvency property — both need the same randomized-sequence harness (see 0061) — but is worth its own issue because the assertion is different in kind: solvency is about the *sum*, isolation is about *which bucket* moved.

## Acceptance criteria

- [ ] Randomized interleaving covers at least 100 generated sequences per run.
- [ ] Asserts per-task and per-keeper balances are unaffected by every admin call, not just `sweep_fees`.
- [ ] References `I-5`.

## Files

- `contracts/keeper-registry/src/test.rs`
