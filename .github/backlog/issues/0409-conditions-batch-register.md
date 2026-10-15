---
title: "feat(registry): batch_register_tasks accepts a condition per task"
labels: [contract, enhancement, intermediate]
epic: E10
wave: 4
depends_on: []
---

## Summary

Issue 0392 adds the optional condition to `register_task`. `batch_register_tasks` (contracts/keeper-registry/src/batch.rs) has its own argument shape and would silently register every batched task without a condition. Bring the batch path to parity.

## Acceptance criteria

- [ ] Each batch entry can carry an optional condition address, and an omitted condition behaves exactly like today.
- [ ] Validation of a batch entry's condition matches `register_task` exactly, so the two paths cannot diverge.
- [ ] A test registers a mixed batch (some conditioned, some not) and checks each stored task's condition.

## Files

- contracts/keeper-registry/src/batch.rs
- contracts/keeper-registry/src/test/batch.rs
- docs/BATCH_OPERATIONS.md
