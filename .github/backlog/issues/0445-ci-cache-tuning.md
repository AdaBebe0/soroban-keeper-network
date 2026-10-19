---
title: "perf(ci): share cargo caches between jobs and measure the saving"
labels: [tooling, performance, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

format, test, clippy, build-wasm and rust-sdk each restore their own cargo cache. Consolidate cache keys where jobs build the same targets and record wall-clock time before and after.

## Acceptance criteria

- [ ] Cache keys are shared where the build targets match and kept separate where they do not.
- [ ] The PR description records median CI duration before and after over at least five runs.

## Files

- .github/workflows/ci.yml
