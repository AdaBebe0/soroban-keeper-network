---
title: "test(registry): execute_task against a verifier that panics must not permanently brick the task"
labels: [testing, contract, advanced]
epic: E04
wave: 2
depends_on: [0075]
---

## Summary

This is the concrete regression test for whatever policy 0075 establishes for a panicking verifier. Written as its own issue because it's the highest-stakes scenario in this epic — a task permanently stuck with escrow inside it would be a direct violation of Invariant I-2 (escrow recoverability, issue 0050) — and deserves a dedicated, carefully-reasoned test rather than being one assertion among several in a bigger PR.

## Expected behaviour

Depends entirely on 0075's conclusion. At minimum, this test must demonstrate whichever of the following actually holds:
- If Soroban isolates the panic: `execute_task` returns a typed error (not a panic propagating to the caller), the task remains `Claimed` and retryable (with a *different* verifier, if the owner can swap one in — but note 0082 blocks that once claimed, so realistically the recovery path here is `expire_task` after the deadline).
- If Soroban does not isolate the panic: the whole transaction reverts, and the test demonstrates that `expire_task` still successfully recovers the escrow once the deadline passes — proving the eventual-recovery fallback actually works even in the worst case.

## Acceptance criteria

- [ ] Test uses a genuinely panicking test-only verifier contract (not a verifier that returns `false` — that's 0084's scenario).
- [ ] Test demonstrates whichever recovery path 0075 concluded is the real one, end to end (including advancing the ledger to the deadline if the fallback is `expire_task`).
- [ ] If this test reveals the fallback does *not* actually work as assumed, that's a critical finding — stop and escalate rather than weakening the test to pass.

## Files

- `contracts/keeper-registry/src/test.rs`
