---
title: "test(cli): command-level tests against the fake RPC"
labels: [cli, testing, intermediate]
epic: E16
wave: 5
depends_on: [0456, 0463]
---

## Summary

Every command needs a test that runs it end to end against the SDK's fake RPC, covering success, a contract error and `--json` output.

## Acceptance criteria

- [ ] Each command has success, contract-error and JSON-output tests.
- [ ] Tests never touch the network.

## Files

- packages/cli/test/
