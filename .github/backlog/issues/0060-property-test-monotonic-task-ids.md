---
title: "test(property): assert monotonic task ids (I-7) — ids are unique and never reused"
labels: [testing, contract, good-first-issue]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Invariant I-7: task ids are unique and never reused, so an external reference to a task id (in an off-chain indexer, a keeper bot's local state, a dApp's UI) is stable forever. `next_task_id` increments a `u64` counter and never decrements it, so this should already hold — this issue is about proving it rather than trusting the implementation, and about pinning the `u64` overflow behavior at the boundary.

## Expected behaviour

A property test that registers a randomized number of tasks (including some that are subsequently cancelled or expired, to confirm ids aren't recycled from terminated tasks) and asserts:
- Every returned `task_id` is strictly greater than every previously returned one.
- `task_count()` after N registrations equals N, regardless of how many of those tasks have since reached a terminal state.

Separately, a focused (non-property) unit test should pin what happens at `u64::MAX`: `next_task_id` currently calls `.expect("task id overflow")`, which panics rather than returning a typed error. Decide whether that's acceptable (a `u64` counter reaching its max is astronomically unlikely in practice, unlike the `u32` cases fixed in wave 1) and document the decision — this issue is scoped to *testing* the current behavior, not necessarily changing it; if the panic is judged unacceptable, file a follow-up issue rather than scope-creeping this one.

## Acceptance criteria

- [ ] Property test asserts strict monotonicity across a randomized mix of registration and termination calls.
- [ ] A unit test exercises `next_task_id` at `u64::MAX - 1` and documents (in a comment, referencing this issue) whether the panic-at-overflow behavior is accepted as-is or needs a follow-up.
- [ ] References `I-7`.

## Files

- `contracts/keeper-registry/src/test.rs`
