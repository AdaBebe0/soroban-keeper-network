---
title: "feat(indexer): Prometheus /metrics endpoint"
labels: [indexer, observability, intermediate]
epic: E18
wave: 5
depends_on: [0474]
---

## Summary

The indexer has no metrics endpoint. Expose ingestion lag, events ingested by type, API request counts and latencies, cache hit ratio, rate-limit rejections and WebSocket connections.

## Acceptance criteria

- [ ] `/metrics` serves Prometheus text format on the API listener, or a separate bind address if the design says so.
- [ ] Metric names follow docs/OBSERVABILITY.md.
- [ ] A test scrapes the endpoint and checks the core metrics exist.

## Files

- indexer/src/metrics.rs
- indexer/src/api/mod.rs
