---
title: "feat(examples): recurring payment / subscription task"
labels: [enhancement, intermediate]
epic: E22
wave: 5
depends_on: [0554]
---

## Summary

A subscription contract that pays a merchant each period when a keeper calls it, with tasks registered per period.

## Acceptance criteria

- [ ] The contract refuses double payment within a period.
- [ ] A script registers the next period's task after each execution.

## Files

- examples/recurring-payment/
