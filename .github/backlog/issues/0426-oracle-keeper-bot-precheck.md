---
title: "feat(keeper-bot-v2): pre-check oracle conditions off-chain before claiming"
labels: [keeper-bot, oracle, intermediate]
epic: E11
wave: 4
depends_on: [0424]
---

## Summary

Issue 0402 makes the bot skip tasks whose condition is not met by simulating `is_claimable`. For oracle conditions the bot can read the feed directly and predict the outcome without a simulation per task, saving RPC calls per round.

## Acceptance criteria

- [ ] The bot reads each distinct feed once per round, not once per task.
- [ ] A predicted-unmet task is skipped with a dedicated skip reason in metrics.
- [ ] If the prediction and the on-chain result ever disagree, the bot logs it and falls back to simulation.

## Files

- examples/keeper-bot-v2/src/conditions.ts
- examples/keeper-bot-v2/src/loop.ts
