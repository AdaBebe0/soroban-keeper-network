---
title: "docs(security): formal task state-machine specification"
labels: [security, contract, advanced]
epic: E19
wave: 5
depends_on: [0495]
---

## Summary

Write the task lifecycle as an explicit state machine (states, transitions, guards, effects on balances) that tests can be checked against.

## Acceptance criteria

- [ ] Every transition the contract allows is listed with its guard.
- [ ] A property test asserts no transition outside the table is reachable.

## Files

- docs/TASK_STATE_MACHINE.md
- contracts/keeper-registry/src/test/property.rs
