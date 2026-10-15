---
title: "feat(dashboard): shared formatting for amounts, times and addresses"
labels: [dashboard, enhancement, good-first-issue]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Consistent stroops-to-token formatting using `bigint`, relative and absolute times, and truncated addresses with copy buttons.

## Acceptance criteria

- [ ] Amount formatting never converts through `number`.
- [ ] Unit tests cover large values and zero.

## Files

- apps/dashboard/src/format.ts
