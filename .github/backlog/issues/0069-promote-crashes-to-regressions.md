---
title: "process: promote every fuzzer-found crash into a permanent regression test"
labels: [testing, docs, good-first-issue]
epic: E03
wave: 2
depends_on: [0051]
---

## Summary

A crash found by the fuzzer and merely "fixed" is a bug that can silently come back if a future refactor reintroduces the same shape of mistake — the fuzzer might not rediscover it for a long time, since it's searching randomly. This issue establishes the process (and the first piece of tooling) for converting every crash the fuzz harness finds into a checked-in, deterministic unit test.

## Expected behaviour

- A documented convention (in `CONTRIBUTING.md`'s fuzzing section, alongside 0070's broader guide): any PR that fixes a bug found by fuzzing must include the minimized crashing input, committed under `fuzz/corpus/<target>/regressions/`, *and* a corresponding `#[test]` in `test.rs` that reproduces the exact scenario in human-readable form (not just "replay these fuzzer bytes" — a fuzzer input replay is not reviewable by a human).
- A `PULL_REQUEST_TEMPLATE.md` checkbox item for "if this PR fixes a fuzzer-found bug, the regression is committed" (coordinate with issue 0042, wave 1, which already touches PR template drift).

## Suggested approach

This is process and documentation, not code — the goal is that this wave's fuzz targets (0051–0067) don't just find bugs once, they build a permanent, growing regression suite as a side effect of ever having been run.

## Acceptance criteria

- [ ] `CONTRIBUTING.md` states the convention plainly.
- [ ] `PULL_REQUEST_TEMPLATE.md` has the checkbox.
- [ ] If any crash was found while developing issues 0051–0067, it's used as the first real example of this process rather than the process shipping with zero examples.

## Files

- `CONTRIBUTING.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
