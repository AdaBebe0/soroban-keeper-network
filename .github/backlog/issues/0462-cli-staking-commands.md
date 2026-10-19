---
title: "feat(cli): staking commands for keepers"
labels: [cli, staking, intermediate]
epic: E16
wave: 5
depends_on: [0461]
---

## Summary

Wrap `stake_deposit`, `initiate_unbond`, `withdraw_stake` and the pending-unbond view.

## Acceptance criteria

- [ ] `unbond` prints when the stake becomes withdrawable.
- [ ] `withdraw-stake` before the unbonding period ends explains the remaining wait instead of submitting a doomed transaction.

## Files

- packages/cli/src/commands/keeper/stake.ts
