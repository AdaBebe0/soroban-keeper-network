---
title: "chore(ci): wire a time-boxed fuzz run into PR CI, with a longer nightly job"
labels: [tooling, testing, intermediate]
epic: E03
wave: 2
depends_on: [0051, 0052, 0053]
---

## Summary

Fuzz targets that only ever run on a contributor's laptop find bugs only as often as someone remembers to run them. This issue wires fuzzing into the pipeline #6 (wave 1) established: a short, advisory run on every PR, and a longer nightly run with a persistent corpus.

## Expected behaviour

Two new CI jobs, both advisory (`continue-on-error: true`, consistent with the existing `clippy`/`audit`/`wasm-size` jobs documented in `docs/CI.md`):
- `fuzz-pr`: runs every registered fuzz target for a short, fixed wall-clock budget (e.g. 60 seconds each) on every PR touching `contracts/keeper-registry/` or `fuzz/`.
- `fuzz-nightly`: a `schedule`-triggered workflow that runs each target for a much longer budget (e.g. 15 minutes each), restores the corpus from a cache, and saves it back — so coverage accumulates across nights instead of restarting from empty every run.

## Suggested approach

Look at how `wasm-size`'s advisory job posts to `$GITHUB_STEP_SUMMARY` in `ci.yml` (from wave 1's CI pipeline PR) and follow the same pattern for reporting fuzz results — total runs, any crash found, corpus size delta.

A crash found by either job should fail loudly enough to be noticed (a red advisory annotation, not just a buried log line) even though it doesn't block the merge — the whole point of "advisory" per `docs/CI.md` is that a false positive shouldn't block a contributor, not that a real crash should be easy to miss.

## Acceptance criteria

- [ ] `fuzz-pr` runs on every relevant PR and completes within a few minutes total.
- [ ] `fuzz-nightly` runs on a schedule, persists its corpus via `actions/cache`, and its budget is long enough to be meaningfully more thorough than the PR job.
- [ ] A crash is surfaced clearly in the job summary, including the minimized failing input.
- [ ] Both jobs are documented in `docs/CI.md` alongside the existing advisory jobs.

## Files

- `.github/workflows/ci.yml`
- `.github/workflows/fuzz-nightly.yml`
- `docs/CI.md`
