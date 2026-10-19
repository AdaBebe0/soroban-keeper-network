---
title: "chore(cli): publish skn to npm with an executable bin"
labels: [cli, tooling, intermediate]
epic: E16
wave: 5
depends_on: [0469]
---

## Summary

Publish the CLI so `npx skn` works, following the SDK's publish workflow.

## Acceptance criteria

- [ ] The package declares a `bin` and installs globally.
- [ ] A tag-triggered workflow publishes it, refusing a version mismatch.

## Files

- .github/workflows/cli-publish.yml
- packages/cli/package.json
