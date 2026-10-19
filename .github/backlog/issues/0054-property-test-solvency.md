---
title: "test(property): assert the solvency invariant (I-1) across random call sequences"
labels: [testing, contract, advanced]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Issue 0050 states Invariant I-1 (Solvency): the registry's token balance always equals open task escrow plus credited keeper balances plus accrued fees. This issue encodes that as an executable property test using `proptest`, run against random sequences of contract calls, rather than the fixed scenarios in `test.rs`.

## Why a property test and not another unit test

Unit tests each pick one specific sequence of calls and assert one specific outcome. A solvency violation is more likely to show up from an *unexpected* sequence — e.g. `increase_reward` immediately followed by `cancel_task`, or `claim_task` racing `expire_task` at the exact deadline boundary — than from any sequence a human thought to write by hand. Property testing generates the sequence for you and shrinks a failure to the smallest reproducing case.

## Expected behaviour

A `proptest` (or hand-rolled sequential state machine, see 0061 for the shared harness) that:
1. Registers 1–5 tasks with randomized rewards.
2. Applies a randomized sequence of valid operations (claim, execute, cancel, expire, increase_reward, withdraw) drawn from what each task's current status allows.
3. After every single operation, asserts: `token.balance(&registry) == sum(open task escrow) + sum(keeper balances) + fees_accrued()`.

## Suggested approach

Build this on top of the shared model-state-machine harness from issue 0061 if that lands first; otherwise a standalone `proptest!` block is acceptable and can be refactored onto the shared harness later — don't block this issue on 0061 if the ordering doesn't work out.

## Acceptance criteria

- [ ] The property runs for at least 256 generated cases per `cargo test` invocation.
- [ ] A failure prints the exact operation sequence that broke solvency (proptest's shrinking should handle this for free — verify it actually does).
- [ ] The test currently passes against `main` (i.e., this issue is asserting a true property today, not documenting a known-broken one).
- [ ] Comments cite `I-1` from `docs/ARCHITECTURE.md` (issue 0050) so the two artifacts stay linked.

## Files

- `contracts/keeper-registry/src/test.rs` (or a new `contracts/keeper-registry/tests/proptest_solvency.rs`)
