---
title: "feat(keeper-bot-v2): liveness and readiness endpoints"
labels: [keeper-bot, observability, good-first-issue]
epic: E18
wave: 5
depends_on: []
---

## Summary

Orchestrators need `/healthz` (process alive) and `/readyz` (config valid, RPC reachable, database connected if configured). Serve them on the metrics port.

## Acceptance criteria

- [ ] `/readyz` fails while the RPC or database is unreachable and recovers when they return.
- [ ] examples/keeper-bot-v2/DEPLOYMENT.md shows Kubernetes probe configuration using them.

## Files

- examples/keeper-bot-v2/src/index.ts
- examples/keeper-bot-v2/DEPLOYMENT.md
