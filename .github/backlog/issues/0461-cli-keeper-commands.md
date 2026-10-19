---
title: "feat(cli): skn keeper balance and skn keeper withdraw"
labels: [cli, enhancement, good-first-issue]
epic: E16
wave: 5
depends_on: [0453]
---

## Summary

Show a keeper's accrued balance and reputation, and withdraw rewards.

## Acceptance criteria

- [ ] `balance` shows balance, stake and effective reputation.
- [ ] `withdraw` reports "nothing to withdraw" without sending a transaction when the balance is zero.

## Files

- packages/cli/src/commands/keeper/
