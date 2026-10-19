---
title: "test(registry): execute_task with a verifier that always rejects"
labels: [testing, contract, good-first-issue]
epic: E04
wave: 2
depends_on: [0074, 0080]
---

## Summary

The rejection-path counterpart to 0083. A minimal test-only verifier that always returns `false`, exercising the `VerificationFailed` error and event added in 0080.

## Expected behaviour

A test registers a task with an always-reject verifier, claims it, and calls `execute_task`. Asserts:
- The call fails with `KeeperError::VerificationFailed`.
- The task's status is still `Claimed` (not `Executed`, not reverted to `Pending`) — it remains retryable.
- No token transfer occurred and no keeper balance was credited.
- The `TaskVerificationFailed` event fired with the correct task_id and keeper.
- A second `execute_task` call (simulating a retry with different proof bytes, still against the same always-reject verifier) also fails the same way — confirms the rejection is repeatable, not a one-shot state change.

## Acceptance criteria

- [ ] All assertions above covered.
- [ ] Confirms the task remains claimable-for-retry-by-the-same-keeper (not accidentally kicked back to `Pending` or opened to a different keeper).

## Files

- `contracts/keeper-registry/src/test.rs`
