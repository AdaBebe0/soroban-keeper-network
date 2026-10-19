---
title: "design(registry): batch_register_tasks API shape and cost model"
labels: [contract, docs, advanced]
epic: E05
wave: 2
depends_on: [0050]
---

## Summary

Opens epic E05 (Batch Operations & Gas). A dApp registering many similar tasks at once currently pays the fixed overhead of a full register_task transaction (auth, storage write, TTL extension, event) once per task. This issue designs a batch_register_tasks entry point that amortizes that overhead, before any implementation begins.

## Questions this doc must answer

- Auth model. All tasks in one batch call share the same owner, authorizing once for the whole batch. Confirm this is the desired UX and doesn't create a way to register more escrow than the owner expected, e.g. via a max_total_reward ceiling the caller commits to upfront.
- Partial failure semantics. If one task in a batch has an invalid reward, does the whole batch revert, or does it skip invalid entries and return per-entry results? Soroban transactions are atomic by default -- confirm whether partial success within one call is achievable at all.
- Resource ceiling. How many tasks can one batch call realistically hold before hitting Soroban per-transaction CPU/memory budget? This needs an empirical answer, not a guess.
- Return shape. register_task returns a single u64 task id. A batch call returning a vector needs to preserve ordering against the input so the caller can correlate results.

## Expected output

A design document (docs/BATCH_OPERATIONS.md) answering each question, with the exact function signature pinned, before issue 0098 begins implementation.

## Acceptance criteria

- [ ] Auth model decided and justified.
- [ ] Partial-failure semantics decided based on actual Soroban transaction atomicity, not assumption.
- [ ] An empirical resource-ceiling number is measured (even roughly) and recorded.
- [ ] Exact function signature pinned in Rust syntax.

## Files

- docs/BATCH_OPERATIONS.md
