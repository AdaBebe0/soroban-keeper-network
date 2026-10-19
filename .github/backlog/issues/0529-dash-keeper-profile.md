---
title: "feat(dashboard): keeper profile page"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0528]
---

## Summary

Show a keeper's balance, stake, pending unbond, effective reputation, slash history and recent tasks.

## Acceptance criteria

- [ ] Contract-only values (stake, reputation) are read live; history comes from the indexer.
- [ ] Slash history shows appeals and their outcome.

## Files

- apps/dashboard/src/pages/Keeper.tsx
