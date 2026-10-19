---
title: "feat(dashboard): Freighter wallet connection"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Connect a wallet using the wallet-signing pattern from packages/sdk-ts/examples/wallet-signing. Read-only pages must work without a wallet.

## Acceptance criteria

- [ ] Connect, disconnect and account change are handled.
- [ ] A wallet on the wrong network produces a clear message.

## Files

- apps/dashboard/src/wallet/
