---
title: "feat(ops): Prometheus alert rules for the indexer and keeper bots"
labels: [observability, tooling, intermediate]
epic: E18
wave: 5
depends_on: [0475]
---

## Summary

Ship alert rules for the signals in docs/OBSERVABILITY.md: indexer lag, ingest failures, API error rate, bot RPC errors, bot balance not growing, and bot stopped running rounds.

## Acceptance criteria

- [ ] Rules live in a file Prometheus can load and pass `promtool check rules` in CI.
- [ ] Each alert links to its runbook.

## Files

- ops/prometheus/alerts.yml
- .github/workflows/ci.yml
