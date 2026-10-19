---
title: "feat(ops): a contract state monitor for admins"
labels: [observability, tooling, intermediate]
epic: E18
wave: 5
depends_on: [0474]
---

## Summary

Admins have no alert when fees accrue unswept, the contract is paused unexpectedly, or the instance TTL nears expiry. Add a small poller that exports these as metrics.

## Acceptance criteria

- [ ] It exports paused, fees accrued, task count, version and instance TTL remaining.
- [ ] It reads only views and needs no signing key.

## Files

- ops/contract-monitor/
- docs/OBSERVABILITY.md
