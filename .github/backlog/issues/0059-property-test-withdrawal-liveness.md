---
title: "test(property): assert withdrawal liveness (I-6) — a keeper's balance is always withdrawable, including while paused"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Invariant I-6: a keeper's credited balance is always withdrawable, including while the contract is paused. This is the promise that makes pausing acceptable to keepers — issue 0029 (wave 1) tests this for a handful of fixed scenarios; this issue generalizes it to a property covering arbitrary pause/unpause interleavings and arbitrary numbers of prior credits.

## Expected behaviour

For any sequence of `execute_task` calls that credits a keeper some balance, and any subsequent sequence of `pause`/`unpause` admin calls, `withdraw_rewards` for that keeper:
- Never fails with `ContractPaused` (it must not be gated by the pause switch at all, at any point in the sequence).
- Always transfers exactly the keeper's full accrued balance and zeroes it.

## Suggested approach

This is a smaller state space than 0054/0058 — the property only needs to range over "how many times was pause toggled, and in what order relative to the withdrawal" — so a targeted `proptest!` over a bounded sequence of booleans (pause/unpause/withdraw) is sufficient; the full state-machine harness from 0061 is not required here.

## Acceptance criteria

- [ ] Property covers: withdraw while never paused, withdraw while currently paused, withdraw after multiple pause/unpause cycles.
- [ ] Asserts the withdrawal amount and post-withdrawal zero balance in every case.
- [ ] References `I-6`.

## Files

- `contracts/keeper-registry/src/test.rs`
