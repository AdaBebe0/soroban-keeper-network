---
title: "feat(registry): implement batch_register_tasks"
labels: [contract, enhancement, advanced]
epic: E05
wave: 2
depends_on: [0097]
---

## Summary

Implements the design from 0097: a single entry point that registers multiple tasks in one transaction, sharing one auth check, for a shared owner.

## Expected behaviour

batch_register_tasks(e, owner, tasks: Vec<TaskParams>) -> Result<Vec<u64>, KeeperError> where TaskParams bundles the per-task fields register_task currently takes individually (task_type, calldata, reward, deadline, ttl_ledgers, lock_ledgers, and the verifier field if epic E04 has landed by the time this is implemented -- coordinate ordering with that epic rather than assuming one finishes strictly before the other).

Per-task validation (reward positivity, minimum reward, deadline, lock/ttl bounds) runs identically to register_task's existing checks, applied to every entry before any escrow transfer happens, per 0097's all-or-nothing decision (or whatever 0097 actually concluded -- implement against the design doc, not this summary, if they have diverged).

## Suggested approach

Factor register_task's body into a shared internal helper that both the single and batch entry points call, so the two never drift in what they validate -- the same single-source-of-truth pattern wave 1's issue 0001 fix established for fee_bps.

## Acceptance criteria

- [ ] Shares validation logic with register_task via an extracted helper (no duplicated validation).
- [ ] Enforces the auth/ceiling model from 0097's design.
- [ ] Returns task ids in input order.
- [ ] A test registers a batch of N tasks and confirms all N are individually retrievable via get_task with correct fields.
- [ ] A test confirms an invalid entry anywhere in the batch causes the documented all-or-nothing (or partial, per 0097) behavior, with no escrow transferred for any entry if the batch as a whole fails.

## Files

- contracts/keeper-registry/src/lib.rs
- contracts/keeper-registry/src/test.rs
