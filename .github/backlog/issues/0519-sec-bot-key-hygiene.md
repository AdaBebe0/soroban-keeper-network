---
title: "security(keeper-bot): key hygiene review for both bots"
labels: [security, keeper-bot, intermediate]
epic: E19
wave: 5
depends_on: []
---

## Summary

Check how both bots load, hold and log keys, including error paths and crash dumps, and that the examples never encourage committing a key.

## Acceptance criteria

- [ ] Every key-handling path is reviewed and listed.
- [ ] A test asserts errors never include the secret.

## Files

- examples/keeper-bot/index.js
- examples/keeper-bot-v2/src/config.ts
- docs/KEEPER_BOT_V2_SECURITY_REVIEW.md
