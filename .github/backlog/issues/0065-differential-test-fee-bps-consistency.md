---
title: "test(property): differential-test get_fee_bps against the fee actually applied, across arbitrary FeeBps history"
labels: [testing, contract, intermediate]
epic: E03
wave: 2
depends_on: [0059]
---

## Summary

Wave 1's issue 0001 fixed a specific instance of `get_fee_bps` disagreeing with the rate `execute_task` actually applies (the view defaulted to 300, the execution path defaulted to 0). The fix — routing both through a single `fee_bps(&e)` helper — should make this class of bug structurally impossible, but "should" is worth confirming with a property test rather than trusting the refactor.

## Expected behaviour

A property test that applies a randomized sequence of `set_fee_bps` calls (including none at all, to hit the never-configured default) interleaved with `execute_task` calls, and after every execution asserts: the fee actually deducted (computable from the keeper's balance delta and the task's reward) matches `get_fee_bps()` as read *immediately before* that execution.

## Suggested approach

This is a differential/consistency check rather than a property about a single invariant from issue 0050 — it's specifically testing that two independent read paths (a view function and an execution side-effect) never diverge, which is a different failure mode than the numbered invariants and is worth keeping distinct.

## Acceptance criteria

- [ ] Covers: never-configured `FeeBps`, single `set_fee_bps` call, multiple calls with different values interleaved with executions.
- [ ] Fails clearly (naming the divergent values) if `get_fee_bps()` and the applied fee ever disagree.

## Files

- `contracts/keeper-registry/src/test.rs`
