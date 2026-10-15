---
title: "feat(examples): token vesting release task"
labels: [enhancement, intermediate]
epic: E22
wave: 5
depends_on: [0554]
---

## Summary

A vesting contract whose cliff and tranche releases are triggered by keepers instead of beneficiaries remembering to claim.

## Acceptance criteria

- [ ] Releases before their date are refused by the contract, not just the keeper.
- [ ] Tests cover each tranche boundary.

## Files

- examples/vesting-release/
