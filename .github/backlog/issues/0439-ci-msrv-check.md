---
title: "chore(ci): declare and test a minimum supported Rust version for the Rust SDK"
labels: [tooling, rust-sdk, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

The Rust SDK is meant for external integrators, who may not run the latest stable. Declare `rust-version` in rust-sdk/Cargo.toml and add a CI job that builds the crate on it.

## Acceptance criteria

- [ ] `rust-version` is set and justified in the PR description.
- [ ] A CI job builds and tests the crate on that toolchain.

## Files

- rust-sdk/Cargo.toml
- .github/workflows/ci.yml
