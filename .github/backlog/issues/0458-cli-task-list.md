---
title: "feat(cli): skn task list with filters"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0457]
---

## Summary

List tasks by status, owner or keeper. Use the indexer API when one is configured and fall back to `get_tasks_range` otherwise, saying which source answered.

## Acceptance criteria

- [ ] Filters work against both sources, or the command explains which filter needs the indexer.
- [ ] Output is paginated with `--limit` and `--cursor`.

## Files

- packages/cli/src/commands/task/list.ts
