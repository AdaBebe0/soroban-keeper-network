---
title: "feat(cli): admin command group"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0453]
---

## Summary

Wrap pause, unpause, set-fee-bps, set-min-reward, set-min-stake, set-dispute-window and sweep-fees. These are the operations most likely to be run under pressure, so they must be explicit.

## Acceptance criteria

- [ ] Every admin command shows the current and new value and requires confirmation.
- [ ] Commands refuse to run when the signer is not the contract admin, before submitting.

## Files

- packages/cli/src/commands/admin/
