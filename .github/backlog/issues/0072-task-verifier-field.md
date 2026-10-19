---
title: "feat(registry): add an optional verifier field to Task"
labels: [contract, enhancement, intermediate]
epic: E04
wave: 2
depends_on: [0071]
---

## Summary

The first concrete step of the design from 0071: add `verifier: Option<Address>` to the `Task` struct, unused by any logic yet. This issue is deliberately scoped to just the schema change plus migration considerations, kept separate from 0073/0074 (which make the field actually do something) so the storage-layout change gets reviewed on its own.

## Expected behaviour

- `Task` gains `pub verifier: Option<Address>`.
- Every existing constructor of `Task` (in `register_task`) sets it to `None` until 0073 adds a way to set it otherwise.
- `#[contracttype]` field addition is backward-compatible for *new* writes, but confirm (and document) what happens to *already-persisted* `Task` entries from before this change — Soroban's XDR encoding for struct fields added at the end is typically forward-compatible for reads of old data via `Option` defaulting, but this must be verified against the actual SDK version in use, not assumed.

## Suggested approach

Check `soroban-sdk`'s documented behavior for schema evolution of `#[contracttype]` structs — specifically whether an old, already-persisted `Task` (written before this field existed) can still be deserialized after this change deploys via `upgrade`. If it cannot, this needs a storage migration path, which is a materially bigger issue — surface that finding here rather than discovering it in production.

## Acceptance criteria

- [ ] `Task.verifier: Option<Address>` added.
- [ ] All existing tests pass unmodified (field defaults to `None`, no behavior change yet).
- [ ] A test specifically constructs a `Task` the old way (no verifier) and confirms `get_task` still round-trips it correctly.
- [ ] The schema-evolution question above is answered and documented, with a follow-up issue filed if a migration path turns out to be needed.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
