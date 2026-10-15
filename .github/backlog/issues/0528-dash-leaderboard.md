---
title: "feat(dashboard): keeper leaderboard"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Rank keepers using the indexer's `/leaderboard` query, with executions, rewards earned and reputation.

## Acceptance criteria

- [ ] The table is sortable and paginated.
- [ ] Each row links to the keeper profile.

## Files

- apps/dashboard/src/pages/Leaderboard.tsx
