---
title: "feat(ops): docker-compose observability stack"
labels: [observability, tooling, intermediate]
epic: E18
wave: 5
depends_on: [0480, 0481]
---

## Summary

One command brings up Prometheus and Grafana preloaded with the rules and dashboards, scraping a local indexer and bot.

## Acceptance criteria

- [ ] `docker compose up` in ops/ starts the stack with dashboards provisioned.
- [ ] docs/OBSERVABILITY.md explains how to point it at real targets.

## Files

- ops/docker-compose.yml
- docs/OBSERVABILITY.md
