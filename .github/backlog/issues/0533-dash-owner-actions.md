---
title: "feat(dashboard): owner actions on the task page"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0526, 0524]
---

## Summary

Increase reward, extend deadline and cancel from the task page when the connected wallet owns the task.

## Acceptance criteria

- [ ] Actions only show for the owner and only in states where they are allowed.
- [ ] Contract errors are shown in plain language.

## Files

- apps/dashboard/src/components/OwnerActions.tsx
