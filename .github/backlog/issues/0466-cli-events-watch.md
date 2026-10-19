---
title: "feat(cli): skn events watch"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

Stream contract events to the terminal, from the indexer WebSocket (`/v1/stream`) when configured or by polling `getEvents` otherwise.

## Acceptance criteria

- [ ] Events print as one line each, or as JSON lines with `--json`.
- [ ] `--type` filters by event name and `--task` by task ID.

## Files

- packages/cli/src/commands/events.ts
