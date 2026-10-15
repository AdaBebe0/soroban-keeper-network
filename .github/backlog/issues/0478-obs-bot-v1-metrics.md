---
title: "feat(keeper-bot): optional Prometheus metrics for the v1 bot"
labels: [keeper-bot, observability, intermediate]
epic: E18
wave: 5
depends_on: [0474]
---

## Summary

examples/keeper-bot/index.js mentions a Prometheus endpoint in its header comment but has none. Add an opt-in endpoint with rounds, claims, executions, skips by reason and RPC errors, without adding dependencies that complicate the beginner example.

## Acceptance criteria

- [ ] Metrics are off by default and enabled by one environment variable.
- [ ] Names match keeper-bot-v2 where the meaning is the same.
- [ ] The header comment matches what the bot actually does.

## Files

- examples/keeper-bot/index.js
- examples/keeper-bot/README.md
