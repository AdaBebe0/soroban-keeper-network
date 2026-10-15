---
title: "security(registry): review unbounded iteration and storage growth"
labels: [security, contract, advanced]
epic: E19
wave: 5
depends_on: []
---

## Summary

Views like `get_tasks_range` and `slash_history`, and per-keeper storage, can grow without limit. Check every loop and growing collection against Soroban resource limits.

## Acceptance criteria

- [ ] Each loop and growing collection is listed with its bound or the reason it needs none.
- [ ] Any unbounded case gets a bound and a test at the bound.

## Files

- contracts/keeper-registry/src/views.rs
- contracts/keeper-registry/src/staking.rs
