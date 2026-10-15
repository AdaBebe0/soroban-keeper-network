---
title: "feat(examples): auction settlement task"
labels: [enhancement, intermediate]
epic: E22
wave: 5
depends_on: [0554]
---

## Summary

An auction contract that a keeper settles after the end time, transferring the item and funds.

## Acceptance criteria

- [ ] Settlement before the end time fails.
- [ ] The task deadline is set relative to the auction end.

## Files

- examples/auction-settlement/
