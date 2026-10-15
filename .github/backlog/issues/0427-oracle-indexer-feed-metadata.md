---
title: "feat(indexer): expose which oracle and asset a conditioned task depends on"
labels: [indexer, oracle, intermediate]
epic: E11
wave: 4
depends_on: [0419]
---

## Summary

Keepers deciding what to watch need to know which feed each task depends on. Record the oracle address and asset for oracle-conditioned tasks and expose them through the API.

## Acceptance criteria

- [ ] The task detail response includes oracle address, asset and threshold for oracle conditions.
- [ ] A new query lists open tasks grouped by feed.
- [ ] `indexer/openapi.yaml` is regenerated.

## Files

- indexer/src/ingest/mod.rs
- indexer/src/api/rest.rs
- indexer/openapi.yaml
