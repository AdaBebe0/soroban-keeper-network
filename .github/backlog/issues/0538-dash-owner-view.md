---
title: "feat(dashboard): owner view of all tasks an address registered"
labels: [dashboard, enhancement, good-first-issue]
epic: E17
wave: 5
depends_on: [0525]
---

## Summary

Use the indexer's `/owners/{owner}/tasks` route to show an owner's tasks with totals locked in rewards.

## Acceptance criteria

- [ ] The view totals rewards by status.
- [ ] It links each task to its detail page.

## Files

- apps/dashboard/src/pages/Owner.tsx
