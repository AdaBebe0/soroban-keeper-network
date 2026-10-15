---
title: "chore(ci): crates.io publish workflow for the Rust SDK"
labels: [rust-sdk, tooling, intermediate]
epic: E13
wave: 3
depends_on: []
---

## Summary

The TypeScript SDK has `.github/workflows/sdk-ts-publish.yml`; the Rust SDK (`rust-sdk/`, crate `soroban-keeper-sdk`) has no equivalent, so the only way to depend on it is a git or path dependency. Add a tag-triggered workflow that publishes the crate to crates.io, mirroring the TypeScript workflow's shape.

## Acceptance criteria

- [ ] A tag of the form `rust-sdk-vX.Y.Z` builds, tests and publishes the crate; any other tag does not trigger it.
- [ ] The workflow refuses to publish when the tag's version does not match `rust-sdk/Cargo.toml`.
- [ ] `cargo publish --dry-run` runs on every PR that touches `rust-sdk/`, so packaging errors surface before a release.
- [ ] The crates.io token is read from a repository secret and never echoed; the README documents the release steps.

## Files

- .github/workflows/rust-sdk-publish.yml
- rust-sdk/Cargo.toml
- rust-sdk/README.md
