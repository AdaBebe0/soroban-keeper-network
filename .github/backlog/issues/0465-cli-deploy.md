---
title: "feat(cli): skn deploy for the registry and treasury"
labels: [cli, tooling, advanced]
epic: E16
wave: 5
depends_on: [0452, 0453]
---

## Summary

scripts/deploy.sh deploys and initializes contracts with shell. A CLI command can validate parameters first and write the result to the config profile.

## Acceptance criteria

- [ ] The command uploads WASM, deploys, initializes and prints contract IDs and WASM hashes.
- [ ] It refuses mainnet without an explicit flag and shows every initialization parameter first.
- [ ] The deployed IDs are saved to the selected profile.

## Files

- packages/cli/src/commands/deploy.ts
- docs/DEPLOYING.md
