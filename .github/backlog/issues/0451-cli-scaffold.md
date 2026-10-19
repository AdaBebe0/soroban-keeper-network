---
title: "feat(cli): scaffold the skn package"
labels: [cli, tooling, intermediate]
epic: E16
wave: 5
depends_on: [0450]
---

## Summary

Create the package chosen in issue 0450 with an argument parser, `--help` for every command, `skn --version`, and a test harness that runs commands against the SDK's fake RPC.

## Acceptance criteria

- [ ] `skn --help` lists the command groups from the design document.
- [ ] `skn --version` prints the CLI version and the contract versions it supports.
- [ ] A test runs a command end to end against the fake RPC.

## Files

- packages/cli/package.json
- packages/cli/src/index.ts
- packages/cli/test/
