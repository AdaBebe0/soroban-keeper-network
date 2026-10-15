---
title: "design(dashboard): web dashboard scope, stack and data sources"
labels: [dashboard, advanced]
epic: E17
wave: 5
depends_on: []
---

## Summary

Epic E17 adds a web dashboard: task explorer, keeper leaderboard and protocol stats. Decide the stack (the React hooks in packages/sdk-ts/src/react are the natural base), which data comes from the indexer client versus direct contract reads, hosting, and wallet support.

## Acceptance criteria

- [ ] The document fixes the framework, routing and styling approach.
- [ ] It states, per page, whether data comes from the indexer or the contract, and why.
- [ ] It names the supported wallets and the network-switching model.

## Files

- docs/DASHBOARD_DESIGN.md
