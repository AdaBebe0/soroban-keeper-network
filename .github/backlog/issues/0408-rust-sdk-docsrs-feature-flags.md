---
title: "docs(rust-sdk): docs.rs metadata, feature flags and a crate-level example"
labels: [rust-sdk, docs, good-first-issue]
epic: E13
wave: 3
depends_on: [0405]
---

## Summary

Before the crate is published (issue 0405), its docs.rs page should be useful on day one. Add `[package.metadata.docs.rs]`, document each optional feature, and put a runnable example in the crate-level docs.

## Acceptance criteria

- [ ] `cargo doc --no-deps` builds without warnings.
- [ ] Every public item in `client.rs`, `retry.rs` and `signing.rs` has a doc comment.
- [ ] The crate-level docs contain an example that compiles as a doctest.
- [ ] Optional features, if any, are listed with what they enable.

## Files

- rust-sdk/Cargo.toml
- rust-sdk/src/lib.rs
