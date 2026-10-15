---
title: "docs(ops): runbook — indexer falling behind"
labels: [observability, docs, good-first-issue]
epic: E18
wave: 5
depends_on: [0480]
---

## Summary

What to check and do when the lag alert fires: RPC health, database load, backfill page size, restarting safely given idempotent ingestion.

## Acceptance criteria

- [ ] Steps are ordered from cheapest to most disruptive.
- [ ] Each step names the metric or log line that confirms or rules it out.

## Files

- docs/runbooks/indexer-lag.md
