---
title: "chore(ci): publish the keeper-bot-v2 container image"
labels: [tooling, keeper-bot, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

examples/keeper-bot-v2 has a Dockerfile built from the repository root, but no workflow builds or publishes it, so nothing verifies the Dockerfile still works. Build it on PRs that touch the bot and publish it on release tags.

## Acceptance criteria

- [ ] PRs touching examples/keeper-bot-v2 or packages/sdk-ts build the image (no push).
- [ ] Release tags push it to GHCR.
- [ ] examples/keeper-bot-v2/README.md references the published image.

## Files

- .github/workflows/bot-v2-image.yml
- examples/keeper-bot-v2/Dockerfile
- examples/keeper-bot-v2/README.md
