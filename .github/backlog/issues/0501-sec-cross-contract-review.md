---
title: "security(registry): cross-contract call review (verifier, token, condition)"
labels: [security, contract, advanced]
epic: E19
wave: 5
depends_on: []
---

## Summary

The registry calls verifiers, the reward token, and (with E10) condition contracts. Review each call for state written before the call, failure handling, and a malicious callee.

## Acceptance criteria

- [ ] Each external call site is listed with state changes before and after it.
- [ ] A malicious-callee test exists for each call type.

## Files

- contracts/keeper-registry/src/verifier.rs
- contracts/keeper-registry/src/task.rs
