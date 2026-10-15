---
title: "feat(ops): testnet canary task that proves end-to-end liveness"
labels: [observability, testing, advanced]
epic: E18
wave: 5
depends_on: [0487]
---

## Summary

Metrics show components are up, not that a task actually gets executed. Register a small recurring canary task on testnet and alert when it is not executed within a target time.

## Acceptance criteria

- [ ] The canary registers on a schedule and records time-to-execution.
- [ ] An alert fires when execution exceeds the target.

## Files

- ops/canary/
- docs/OBSERVABILITY.md
