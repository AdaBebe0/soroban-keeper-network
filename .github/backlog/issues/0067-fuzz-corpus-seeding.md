---
title: "chore(fuzz): seed each fuzz target's corpus from existing unit test inputs"
labels: [testing, good-first-issue]
epic: E03
wave: 2
depends_on: [0052, 0053, 0062, 0063, 0064]
---

## Summary

A fuzzer starting from an empty corpus spends its early runs rediscovering inputs the unit tests already know are interesting (a task at exactly the lock boundary, a proof at exactly `MAX_PROOF_LEN`, `fee_bps` at exactly `10_000`). Seeding the corpus with these values up front means fuzzing time goes toward genuinely novel inputs instead.

## Expected behaviour

For each fuzz target added in this wave, a `fuzz/corpus/<target>/` directory containing a handful of hand-picked seed inputs derived from the boundary values already asserted in `test.rs` (e.g. `MIN_LOCK_LEDGERS`, `MIN_LOCK_LEDGERS - 1`, `MAX_LOCK_LEDGERS`, `MAX_LOCK_LEDGERS + 1`, `MAX_PROOF_LEN`, `MAX_PROOF_LEN + 1`, `fee_bps` at `0` and `10_000`).

## Suggested approach

Write a small helper (a `#[test]` gated behind an `ignore` attribute, run manually) that serializes each boundary value in the exact byte format the corresponding `Arbitrary` impl expects, and writes it to the corpus directory — this keeps the seed values in sync with the *actual* test constants (`MIN_LOCK_LEDGERS` etc.) rather than being copy-pasted numbers that can drift if the constants change.

## Acceptance criteria

- [ ] Every fuzz target from this wave has at least 5 seed corpus entries derived from real boundary constants.
- [ ] A comment or small script documents how to regenerate the corpus if the underlying constants change.
- [ ] `cargo fuzz run <target> -- -runs=0` (corpus validation only, no new generation) passes for every seeded target.

## Files

- `fuzz/corpus/*/`
- `fuzz/src/seed.rs` (or equivalent generator)
