---
title: "feat(examples): dollar-cost averaging bot task"
labels: [enhancement, intermediate]
epic: E22
wave: 5
depends_on: [0556]
---

## Summary

Periodic small swaps executed by keepers, built on the recurring-payment pattern.

## Acceptance criteria

- [ ] Each period swaps a fixed amount, enforced on-chain.
- [ ] The owner can stop the schedule by cancelling the pending task.

## Files

- examples/dca/
