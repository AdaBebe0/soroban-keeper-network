---
title: "feat(dashboard): protocol stats page"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Show totals from the indexer's stats query (tasks by status, rewards paid, active keepers) plus live contract values (fee bps, paused, fees accrued).

## Acceptance criteria

- [ ] Each figure states its source and freshness.
- [ ] Paused state is prominent when true.

## Files

- apps/dashboard/src/pages/Stats.tsx
