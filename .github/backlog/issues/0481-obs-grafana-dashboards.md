---
title: "feat(ops): Grafana dashboards for the indexer and keeper-bot-v2"
labels: [observability, tooling, intermediate]
epic: E18
wave: 5
depends_on: [0475]
---

## Summary

Provide importable dashboards so an operator sees health at a glance.

## Acceptance criteria

- [ ] One dashboard per component, exported as JSON with a datasource variable.
- [ ] Panels cover every metric listed in docs/OBSERVABILITY.md for that component.

## Files

- ops/grafana/indexer.json
- ops/grafana/keeper-bot-v2.json
