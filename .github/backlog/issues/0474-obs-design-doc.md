---
title: "design(ops): observability plan across contract, indexer and bots"
labels: [observability, docs, advanced]
epic: E18
wave: 5
depends_on: []
---

## Summary

keeper-bot-v2 exposes Prometheus metrics; the indexer only logs and reports lag; keeper-bot v1 has none. Decide one metrics naming scheme, the logging format, which signals matter per component, and what alerts exist.

## Acceptance criteria

- [ ] The document lists the metrics each component must expose, with names and labels.
- [ ] It fixes one structured-log format and the fields every log line carries.
- [ ] It names the alerts and links each to a runbook (written in later issues).

## Files

- docs/OBSERVABILITY.md
