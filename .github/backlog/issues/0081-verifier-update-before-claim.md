---
title: "feat(registry): allow the owner to update a task's verifier before it's claimed"
labels: [contract, enhancement, intermediate]
epic: E04
wave: 2
depends_on: [0073]
---

## Summary

An owner who registered a task with the wrong verifier address (or wants to switch from no verification to a verifier after the fact, before anyone has claimed) currently has no path except cancelling and re-registering, which loses the task id and restarts the deadline clock. This issue adds a narrow `update_verifier` mutator, gated the same way `increase_reward`/`extend_deadline` are.

## Expected behaviour

`update_verifier(e, owner, task_id, new_verifier: Option<Address>) -> Result<(), KeeperError>`:
- `require_not_paused`, `owner.require_auth()`, ownership check — same guard pattern as `increase_reward`.
- Only valid while `task.status == Pending` — see 0082 for why `Claimed` must be excluded.
- Emits an event (extend the pattern from 0080, or add a dedicated one).

## Acceptance criteria

- [ ] Update succeeds only on `Pending` tasks, matching the restriction 0082 will assert the reason for.
- [ ] Non-owner calls rejected.
- [ ] Paused registry rejects the call.
- [ ] Event emitted on successful update.
- [ ] README FR spec updated to document this new entry point.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
- `README.md`
