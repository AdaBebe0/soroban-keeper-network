---
title: "feat(cli): skn task claim and skn task execute"
labels: [cli, enhancement, intermediate]
epic: E16
wave: 5
depends_on: [0456]
---

## Summary

Let an operator act as a keeper by hand: claim a task, then execute it with a hex proof.

## Acceptance criteria

- [ ] Claim reports the lock expiry; execute reports the reward credited.
- [ ] Execute accepts the proof as hex or from a file, using the SDK's proof rules.

## Files

- packages/cli/src/commands/task/claim.ts
- packages/cli/src/commands/task/execute.ts
