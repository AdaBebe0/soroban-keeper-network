---
title: "feat(indexer): optional JSON log output"
labels: [indexer, observability, good-first-issue]
epic: E18
wave: 5
depends_on: [0474]
---

## Summary

The indexer logs with `tracing_subscriber::fmt()` in human format only. Add `INDEXER_LOG_FORMAT=json` for log pipelines.

## Acceptance criteria

- [ ] JSON output includes the fields required by docs/OBSERVABILITY.md.
- [ ] The default stays human-readable; indexer/README.md documents the variable.

## Files

- indexer/src/main.rs
- indexer/src/config.rs
- indexer/README.md
