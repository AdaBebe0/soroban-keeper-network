---
title: "feat(registry): batch_cancel_tasks for an owner's own tasks"
labels: [contract, advanced]
epic: E05
wave: 2
depends_on: [0098, 0104]
---

## Summary

`docs/BATCH_OPERATIONS.md` §10 studied whether a `batch_cancel_tasks` is worth
building and recommended it. This issue is that implementation, filed so §10
stays a study.

Unlike issue 0099's batch claim/execute, cancellation carries no cross-keeper
race: the caller owns every task, holds sole authority over it, and no
competing party can invalidate an entry between simulation and submission.
All-or-nothing atomicity — fatal for batch claim — is here just the semantics
`cancel_task` already has.

Rank this below issue 0104 (the batch-size measurement) and issue 0202 (the
§9 escrow-transfer collapse). §10.1's demand analysis is honest that the need
is narrower than for registration: `expire_task` is permissionless and refunds
the owner for free, so an owner who can wait out the deadline already has a
zero-cost unwind path. The cases that justify this are owners with long-dated
tasks and meaningful capital escrowed, and owners correcting a mistake who need
tasks gone *before* a keeper executes against a bad payload.

## The constraint that makes this safe (§10.2)

**Each task must be loaded from storage inside the loop, immediately before it
is validated. Do not pre-load a snapshot of all N tasks.**

This is the opposite of what `batch_register_tasks` does, deliberately:
registration validates *inputs*, which cannot change under it; cancellation
validates *stored state*, which can. A "gather, validate, then refund"
structure copied from registration is a **double-spend**: a re-entrant
`cancel_task` during transfer `i` can cancel and refund task `j > i`, and the
outer loop then reaches its stale cached copy of `j`, still showing `Pending`,
and refunds it a second time. The status guard never fires because it was
evaluated against the snapshot.

A re-entrant call would also need its own owner auth, since Soroban scopes auth
per invocation. Treat that as a mitigating factor, not the guarantee — the
reward token is admin-configured, and a malicious one is exactly the threat
model CEI exists for.

## Proposed shape

Collapse the refunds into a single transfer after all effects (§10.3). This
reduces the re-entry windows from N to one, places that window after every
write in the call, and sidesteps the stale-snapshot hazard entirely by leaving
no task unwritten when the transfer happens.

```rust
pub fn batch_cancel_tasks(
    e: Env,
    owner: Address,
    task_ids: Vec<u64>,
) -> Result<i128, KeeperError> {   // returns the total refunded
    require_not_paused(&e)?;       // NOTE: cancel_task is NOT pause-gated —
                                   // see the open question below
    owner.require_auth();
    // reject empty; reject task_ids.len() > MAX_BATCH_SIZE

    let mut total_refund: i128 = 0;
    for task_id in task_ids.iter() {
        let mut task = load_task(&e, task_id)?;   // fresh load, inside the loop
        // ... same owner check, same status guard (Pending, or Claimed with
        //     an elapsed lock) as cancel_task
        total_refund = total_refund.checked_add(task.reward)?;
        task.status = TaskStatus::Cancelled;
        save_task(&e, task_id, &task);
        emit_task_cancelled(&e, task_id, &owner);
    }

    // Single interaction, after every effect.
    reward_token(&e)?.transfer(&e.current_contract_address(), &owner, &total_refund);
    Ok(total_refund)
}
```

## Open questions for the implementer

- **Pause gating.** `cancel_task` is deliberately *not* pause-gated: it is an
  owner reclaiming its own escrow, which the pause policy classifies as
  liveness rather than new exposure (see the policy matrix in `lib.rs`).
  `batch_cancel_tasks` should follow `cancel_task`, not `register_task` — the
  sketch above shows `require_not_paused` only to flag the decision, and it
  should almost certainly be removed. Whichever way it goes, add a row to the
  policy matrix comment and a case to
  `test_pause_policy_matrix_entry_point_by_entry_point`.
- **Duplicate ids in the input.** A `task_ids` vector containing the same id
  twice must not double-refund. With fresh per-iteration loads the second
  occurrence sees `Cancelled` and reverts the batch, which is the correct and
  safe outcome — but assert it explicitly rather than inheriting it.
- **Reusing `MAX_BATCH_SIZE`** versus a separate constant. Cancellation writes
  a `Task` per entry like registration does, so the same cap is a reasonable
  default; note that cancelling does not write `calldata`, so the per-entry
  write is smaller.

## Acceptance criteria

- [ ] `batch_cancel_tasks` implemented with fresh per-iteration loads and a
      single collapsed refund transfer after all effects.
- [ ] A test asserting the duplicate-id case rejects rather than double-refunds.
- [ ] An invariant test asserting I-1 (`invariants::assert_solvent`) after a
      batch cancel with **heterogeneous** rewards — equal rewards would not
      catch a totalling bug (§9.2's hazard applies here too: the summed refund
      is the money).
- [ ] Tests covering the mixed-status case (one Pending, one Claimed with an
      elapsed lock, one Claimed with an active lock) rejecting the whole batch,
      and the not-owned case.
- [ ] Pause-policy decision made, documented in the `lib.rs` matrix, and
      covered in `test_pause_policy_matrix_entry_point_by_entry_point`.
- [ ] `docs/BATCH_OPERATIONS.md` §10 updated from "recommended" to
      "implemented".
- [ ] CHANGELOG entry and a `VERSION` bump — a new public entry point, plus any
      new error variant, is an ABI change.

## Files

- contracts/keeper-registry/src/lib.rs
- contracts/keeper-registry/src/test.rs
- docs/BATCH_OPERATIONS.md
- CHANGELOG.md
