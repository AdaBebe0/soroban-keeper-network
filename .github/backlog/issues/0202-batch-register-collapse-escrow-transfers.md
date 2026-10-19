---
title: "perf(registry): collapse batch_register_tasks' N escrow transfers into one"
labels: [contract, performance, advanced]
epic: E05
wave: 2
depends_on: [0098, 0104]
---

## Summary

`docs/BATCH_OPERATIONS.md` §9 studied whether `batch_register_tasks`' one
token transfer per entry can be collapsed into a single transfer of the
batch's total, and recommended doing so. This issue is that implementation,
filed so §9 stays a study rather than silently expanding into a code change.

The study's central finding is what makes this small: **per-task escrow is
already bookkeeping, not a separate token holding.** The registry holds one
pooled balance; `cancel_task` and `expire_task` refund by reading
`task.reward` and transferring that amount out of the pool. Collapsing the
inbound transfers changes how much moves per call, not what is recorded, and
leaves the per-task refund logic and the I-1 solvency invariant untouched
(both sides of I-1 are aggregates and move by the same amount either way).

Do not pick this up until 0104 has landed on `main`. Every number in §9.3 is
derived by differencing a baseline that predates `batch_register_tasks`; 0104
measures the real call at increasing N, which turns the estimated ~60% CPU
saving into an observed before/after.

## Proposed shape

`total_reward` is already computed for the `max_total_reward` ceiling check.
Hoist the transfer out of the registration loop:

```rust
// Before the loop, replacing the per-entry transfer inside it:
reward_token(&e)?.transfer(&owner, &e.current_contract_address(), &total_reward);

for params in tasks.iter() {
    // ... next_task_id, Task { .. }, save_task, emit_task_registered
}
```

## The hazard this introduces (§9.2)

Today the sum is used **only** for the ceiling check; the money that moves is
each entry's own `reward`. A wrong sum produces a wrong ceiling check and
nothing worse. After collapsing, **the sum is the money**: a total that
disagrees with the `Task.reward` values the loop goes on to write is a direct
I-1 violation with no error raised anywhere.

So the implementation must guarantee the totalling pass and the writing pass
cannot diverge — same immutable `Vec`, same field, no re-derivation.

## Acceptance criteria

- [ ] `batch_register_tasks` performs exactly one `token.transfer` regardless
      of batch size.
- [ ] An invariant test asserts I-1 (`invariants::assert_solvent`) immediately
      after a batch registration with **heterogeneous** rewards — equal
      rewards would not catch a totalling bug.
- [ ] A test asserts each entry's escrow is still independently refundable:
      cancel one task from a batch, expire another, and confirm each refund is
      that task's own `reward` and I-1 still holds.
- [ ] Resource baseline (`contracts/keeper-registry/resource-baseline.json`)
      regenerated, with the before/after CPU delta recorded in the PR.
- [ ] `docs/BATCH_OPERATIONS.md` §9 updated from "recommended" to "implemented",
      with the measured saving replacing the estimate in §9.3.
- [ ] CHANGELOG entry. Note this is **not** an ABI change — no signature, no
      new error variant, no new entry point — so `VERSION` does not bump. The
      observable difference is one lump-sum token `transfer` event instead of
      N (§9.2), which is worth calling out for indexers reconciling from token
      events rather than from `TaskRegistered`.

## Non-goals

Raising `MAX_BATCH_SIZE`. §9.4 is explicit that the binding constraint at 50
entries is ledger write bytes, not CPU, and collapsing the transfers removes
no writes. This is a fee reduction, not a capability increase.

## Files

- contracts/keeper-registry/src/lib.rs
- contracts/keeper-registry/src/test.rs
- contracts/keeper-registry/resource-baseline.json
- docs/BATCH_OPERATIONS.md
