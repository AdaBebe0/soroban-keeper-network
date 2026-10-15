---
title: "feat(cli): human-readable messages for every KeeperErrorCode"
labels: [cli, enhancement, good-first-issue]
epic: E16
wave: 5
depends_on: [0454]
---

## Summary

A raw `Error(Contract, #12)` tells a user nothing. Map every `KeeperErrorCode` to a one-line explanation and a suggested next step.

## Acceptance criteria

- [ ] Every code in `KeeperErrorCode` has a message; a test fails if a new code is added without one.
- [ ] Messages say what to do next (for example, wait for the lock window, or check the deadline).

## Files

- packages/cli/src/errors.ts
- packages/cli/test/errors.test.ts
