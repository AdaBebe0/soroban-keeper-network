---
title: "design(registry): decide and implement verifier call failure handling (panic vs typed rejection)"
labels: [contract, security, advanced]
epic: E04
wave: 2
depends_on: [0071]
---

## Summary

0071 flagged this as an open question: if a third-party verifier contract panics (rather than returning `false`), what should `execute_task` do? This issue resolves it with an actual investigation into Soroban's cross-contract call semantics, not a guess.

## The concern

A verifier is, by the permissionless design philosophy of this protocol, potentially any contract a task owner chooses. A buggy or malicious verifier that panics on every call would — if `execute_task` doesn't isolate that panic — cause every execution attempt against that task to revert, permanently bricking it (no reward, no refund path, since the task is stuck `Claimed` forever and `expire_task` only fires after the deadline). That's a denial-of-service vector against the escrowed funds, in tension with Invariant I-2 (escrow recoverability, issue 0050).

## Investigation required

Determine, concretely, using Soroban's actual host behavior (not assumption): does a panic in a called contract propagate and abort the entire calling transaction, or can it be caught? If Soroban does not support catching a callee panic (this is the case in many WASM-based cross-contract call models), the only real mitigation is architectural: `expire_task` already provides an eventual, deadline-gated recovery path, so a bricked-by-panic task still resolves — just not before its deadline. Confirm this is actually sufficient, or if not, what additional guard (e.g. a maximum number of failed verification attempts before the owner can force-cancel) is needed.

## Expected output

A short decision record (can be a section in `docs/VERIFIER_DESIGN.md` from 0071, or its own doc) stating: what actually happens on a panicking verifier, whether that's acceptable given `expire_task`'s existing recovery path, and if not, what new mechanism closes the gap.

## Acceptance criteria

- [ ] The actual (not assumed) Soroban cross-contract panic behavior is documented with a citation or a minimal reproducing test.
- [ ] A decision is recorded: acceptable as-is (relying on `expire_task`), or a new mitigation is specified.
- [ ] If a new mitigation is specified, it's scoped as its own follow-up issue rather than silently expanding this one.
- [ ] 0074 is unblocked with a clear answer to implement against.

## Files

- `docs/VERIFIER_DESIGN.md`
- `contracts/keeper-registry/src/test.rs` (proof-of-behavior test)
