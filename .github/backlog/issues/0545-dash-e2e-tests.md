---
title: "test(dashboard): end-to-end tests with Playwright"
labels: [dashboard, testing, advanced]
epic: E17
wave: 5
depends_on: [0532]
---

## Summary

Drive the built app in a browser against a mocked indexer and RPC: browse tasks, open a task, register a task with a stubbed wallet.

## Acceptance criteria

- [ ] The suite runs headless in CI.
- [ ] Wallet interaction is stubbed, never a real extension.

## Files

- apps/dashboard/e2e/
