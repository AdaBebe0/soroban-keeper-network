---
title: "test(rust-sdk): integration tests against a local Stellar quickstart network"
labels: [rust-sdk, testing, advanced]
epic: E13
wave: 3
depends_on: []
---

## Summary

`rust-sdk/tests/registry_client_tests.rs` exercises the client against fakes. Nothing proves the crate works against a real Soroban RPC. Add an opt-in integration suite that deploys the registry to a `stellar/quickstart` container and drives a full task lifecycle through `KeeperRegistryClient`.

## Acceptance criteria

- [ ] The suite is gated behind an environment variable (for example `SKN_INTEGRATION_RPC_URL`) and skips with a visible notice when it is unset, the same pattern the indexer uses for its database tests.
- [ ] One test registers, claims, executes and withdraws, asserting balances after each step.
- [ ] A CI job runs the suite against a quickstart service container on pushes to `main` (advisory, not required).
- [ ] docs/CI.md lists the new job and its gate.

## Files

- rust-sdk/tests/integration_quickstart.rs
- .github/workflows/ci.yml
- docs/CI.md
