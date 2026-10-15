---
title: "feat(dashboard): keeper actions — withdraw rewards and staking"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0529, 0524]
---

## Summary

On the connected keeper's own profile, offer withdraw, stake, unbond and withdraw-stake.

## Acceptance criteria

- [ ] Withdraw is disabled with a reason when the balance is zero.
- [ ] Unbond shows when funds become withdrawable.

## Files

- apps/dashboard/src/components/KeeperActions.tsx
