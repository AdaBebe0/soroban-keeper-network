---
title: "feat(dashboard): task explorer list"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

A paginated table of tasks with status, reward, deadline, owner and claimer, filterable by status.

## Acceptance criteria

- [ ] Data comes from the indexer client with cursor pagination.
- [ ] Status filter and sort by reward or deadline work.

## Files

- apps/dashboard/src/pages/Tasks.tsx
