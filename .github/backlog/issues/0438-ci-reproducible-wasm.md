---
title: "chore(ci): verify release WASM builds are reproducible"
labels: [tooling, security, advanced]
epic: E21
wave: 5
depends_on: []
---

## Summary

release.yml publishes the optimized WASM and its sha256 so anyone can match a deployment to a commit. That only holds if a second build produces the same bytes. Add a check that builds twice in clean environments and compares hashes.

## Acceptance criteria

- [ ] A workflow builds the release WASM twice in separate jobs and fails if the hashes differ.
- [ ] The toolchain and `stellar` CLI versions are pinned in one place.
- [ ] docs/DEPLOYING.md explains how to reproduce a release hash locally.

## Files

- .github/workflows/release.yml
- rust-toolchain.toml
- docs/DEPLOYING.md
