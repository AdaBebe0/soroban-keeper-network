---
title: "design(cli): skn command-line tool scope and architecture"
labels: [cli, tooling, advanced]
epic: E16
wave: 5
depends_on: []
---

## Summary

Epic E16 adds `skn`, a CLI for deploying, registering and inspecting tasks. Decide the language and package location (the TypeScript SDK covers every entry point today; the Rust SDK does not yet), the command tree, configuration and key handling, and output formats.

## Acceptance criteria

- [ ] The document picks TypeScript-on-sdk-ts or Rust-on-rust-sdk with a reason tied to current SDK coverage.
- [ ] It fixes the command tree (task, keeper, admin, batch, deploy, events) so later issues implement against it.
- [ ] It specifies how keys are supplied and guarantees they are never printed.
- [ ] It defines human and `--json` output and the exit-code convention.

## Files

- docs/CLI_DESIGN.md
