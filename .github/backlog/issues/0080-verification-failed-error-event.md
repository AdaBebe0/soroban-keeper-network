---
title: "feat(registry): add VerificationFailed error and a TaskVerificationFailed event"
labels: [contract, enhancement, good-first-issue]
epic: E04
wave: 2
depends_on: [0074]
---

## Summary

0074 needs a typed error and, per the project's convention (every state transition emits an event for off-chain consumers), an event for the specific case of a verifier rejecting a proof — distinct from the existing generic `InvalidTaskStatus`/`NotTaskClaimer` errors, since a verification rejection is informative to a keeper deciding whether to retry (bad proof, try again) versus a status error (the task moved out from under it, don't retry the same way).

## Expected behaviour

- `KeeperError::VerificationFailed` — assign the next free discriminant per the numbering coordinated across wave 1's PRs (confirm the current highest-used discriminant in `main` before picking a number; do not guess without checking).
- `emit_verification_failed(e, task_id, keeper)` following the existing event-emission pattern (`emit_task_executed` etc.) in `lib.rs`.
- `execute_task` emits this event and returns this error when the attached verifier returns `false`, per 0074's contract.

## Acceptance criteria

- [ ] New error variant with a comment explaining when it fires and why it's distinct from existing errors.
- [ ] New event, following the existing `(topic1, topic2), (data...)` publish pattern.
- [ ] A test asserts the event is emitted with the correct task_id and keeper on a verifier rejection.
- [ ] README's event table (kept in sync per wave-1 issue 0017) is updated.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
- `README.md`
