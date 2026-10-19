---
title: "docs(contributing): write the fuzzing and property-testing guide"
labels: [docs, testing, good-first-issue]
epic: E03
wave: 2
depends_on: [0051, 0061, 0069]
---

## Summary

By the end of this epic there will be a `fuzz/` crate, a shared model-checking harness, a corpus-seeding convention, and a crash-to-regression process — none of it documented in one place a new contributor can read start to finish. This issue closes out E03 with that guide.

## Expected behaviour

A "Fuzzing & Property Testing" section in `CONTRIBUTING.md` (or a linked `docs/FUZZING.md` if the section would otherwise make `CONTRIBUTING.md` unwieldy — use judgement based on final length) covering:
- How to run an existing fuzz target locally, and for how long is "enough" before trusting a change.
- How to add a new fuzz target, including the shared `fuzz/src/support.rs` helper from 0052.
- How to use the model-checking harness from 0061 to add a new property.
- The crash-to-regression convention from 0069.
- Which CI jobs run automatically (0066) versus what's expected to be run locally before opening a PR.

## Suggested approach

Write this last, after 0051–0069 land, so it documents what actually exists rather than what was planned — the wave 1 precedent (issue 0043, the CI guide) is a good model for tone and structure: explain what runs, why, and what a contributor is expected to do locally versus what CI catches.

## Acceptance criteria

- [ ] A new contributor can go from "I want to add a fuzz target for function X" to a working, seeded, CI-wired target using only this document.
- [ ] Cross-references `docs/ARCHITECTURE.md`'s invariants section (issue 0050) rather than restating the invariants.
- [ ] Cross-references `docs/CI.md` (issue 0043) for the CI-job side rather than duplicating it.

## Files

- `CONTRIBUTING.md` or `docs/FUZZING.md`
