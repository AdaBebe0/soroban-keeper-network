---
title: "docs(ops): service-level objectives for the indexer API and keeper network"
labels: [observability, docs, intermediate]
epic: E18
wave: 5
depends_on: [0475]
---

## Summary

Write SLOs (API availability and latency, ingestion freshness, canary time-to-execution) and how each is measured from existing metrics.

## Acceptance criteria

- [ ] Each SLO has a target, a window and a PromQL expression.
- [ ] The document says what happens when an error budget is exhausted.

## Files

- docs/SLO.md
