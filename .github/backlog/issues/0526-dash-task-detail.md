---
title: "feat(dashboard): task detail page"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0525]
---

## Summary

Show one task's fields, its event history, and time remaining on deadline and lock.

## Acceptance criteria

- [ ] Event history comes from the indexer and is ordered.
- [ ] Unknown task IDs show a not-found state.

## Files

- apps/dashboard/src/pages/TaskDetail.tsx
