---
title: "feat(indexer): tracing spans for ingest passes and API requests"
labels: [indexer, observability, intermediate]
epic: E18
wave: 5
depends_on: [0476]
---

## Summary

Add a span per ingest pass (ledger range, events, duration) and per API request (route, status, duration) so a slow pass or request can be traced from logs.

## Acceptance criteria

- [ ] Each ingest pass and request logs within a span carrying the fields above.
- [ ] Request IDs are generated and returned in a response header.

## Files

- indexer/src/ingest/mod.rs
- indexer/src/api/mod.rs
