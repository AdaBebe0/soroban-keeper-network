---
title: "feat(cli): skn inspect protocol summary"
labels: [cli, enhancement, good-first-issue]
epic: E16
wave: 5
depends_on: [0451]
---

## Summary

One command that prints protocol state: version, paused, fee bps, minimum reward and stake, fees accrued, task count, and compatibility with the CLI's SDK.

## Acceptance criteria

- [ ] Every value comes from a read-only view; no transaction is sent.
- [ ] A version outside `SUPPORTED_CONTRACT_VERSIONS` prints a warning.

## Files

- packages/cli/src/commands/inspect.ts
