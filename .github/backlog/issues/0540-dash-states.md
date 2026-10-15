---
title: "feat(dashboard): loading, empty and error states everywhere"
labels: [dashboard, enhancement, good-first-issue]
epic: E17
wave: 5
depends_on: [0525]
---

## Summary

Every data view needs a skeleton while loading, an empty state that explains why, and an error state with retry.

## Acceptance criteria

- [ ] Each page has all three states.
- [ ] Errors from the indexer and RPC are worded differently.

## Files

- apps/dashboard/src/components/
