---
title: "feat(registry): execute_task calls the attached verifier before crediting reward"
labels: [contract, enhancement, advanced]
epic: E04
wave: 2
depends_on: [0073, 0075]
---

## Summary

The core behavior change of this epic: when a task has a `verifier` attached, `execute_task` invokes it and only credits the keeper's reward if verification succeeds. This is the issue that makes the interface from 0071 load-bearing rather than decorative.

## Expected behaviour

Inside `execute_task`, after the existing status/claimer/deadline checks and before crediting the keeper:
- If `task.verifier` is `None`, behave exactly as today (no change to the current MVP path).
- If `task.verifier` is `Some(addr)`, construct a client for `addr` using the interface from 0071 and call it with the task and proof. If it returns `true`, proceed as today. If it returns `false`, reject the call with a new typed error (0080) — the task remains `Claimed`, nothing is credited, nothing is transferred, and the keeper may retry with a different proof or let the claim lapse for another keeper to attempt.

## Suggested approach

This depends on 0075 (failure-handling policy) being decided first, since "the verifier panics" needs an answer before this can be implemented safely — do not guess at panic-isolation behavior here; implement against whatever 0075 concludes.

## Acceptance criteria

- [ ] `None` verifier path is provably unchanged (existing wave-1 tests for `execute_task` all still pass without modification).
- [ ] `Some` verifier path: success credits exactly as before; failure rejects without crediting, transferring, or mutating task status.
- [ ] A test with a minimal always-approve verifier contract exercises the success path end-to-end.
- [ ] A test with a minimal always-reject verifier contract exercises the failure path and confirms the task is still `Claimed` afterward (retryable).

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
