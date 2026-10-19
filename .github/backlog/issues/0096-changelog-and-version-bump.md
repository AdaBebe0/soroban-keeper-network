---
title: "chore(release): CHANGELOG entry and VERSION bump for verifier support"
labels: [docs, good-first-issue]
epic: E04
wave: 2
depends_on: [0073, 0074, 0080, 0081]
---

## Summary

Closes out epic E04. The contract's `VERSION` constant exists specifically so "off-chain clients and indexers can detect which ABI they are talking to" (per its doc comment) — adding a verifier field to `Task` and a new argument to `register_task` is exactly the kind of ABI change that constant is for.

## Expected behaviour

- `pub const VERSION: u32 = 1;` bumped to `2`.
- A `CHANGELOG.md` entry under a new version heading, following the existing `[Unreleased]` → dated-version convention already in use (see wave 1's `fix(registry): unify FeeBps default` entry for the expected level of detail — what changed, why, and what a consumer needs to do differently).
- Explicitly calls out the breaking `register_task` arity change so any existing integration (the keeper-bot example, any external dApp) knows to update.

## Acceptance criteria

- [ ] `VERSION` bumped.
- [ ] `test_version_is_exposed` (or equivalent, if renamed) updated to assert the new value.
- [ ] CHANGELOG entry covers every user-visible change from this epic: new `verifier` field, new `register_task` argument, new `update_verifier` function, new `VerificationFailed` error/event.
- [ ] Cross-references the three reference verifier contracts (0077–0079) as available integrations.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
- `CHANGELOG.md`
