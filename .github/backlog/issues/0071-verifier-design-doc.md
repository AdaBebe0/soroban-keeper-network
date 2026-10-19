---
title: "design(registry): write the IKeeperVerifier interface specification"
labels: [contract, docs, advanced]
epic: E04
wave: 2
depends_on: [0050]
---

## Summary

This MVP trusts the claiming keeper to submit an honest `proof` — `execute_task` records it (wave 1's issue 4/PR) but never checks it against anything. The README's Known Design Decisions section names this as a deliberate MVP tradeoff. E04 replaces "trust the keeper" with an optional, per-task, on-chain verification callback. This issue is the design document the other 25 issues in the epic implement against — no code changes here, get the interface right on paper first.

## Why a design doc before code

Every issue downstream of this one (0072–0096) depends on the exact shape of the verifier interface. Getting it wrong after several PRs have built on it means unwinding all of them. This is exactly the kind of "design-heavy, spans multiple components" work the `advanced` label and the CONTRIBUTING.md guidance ("discuss your approach on the issue before writing code") is for.

## Questions this doc must answer

- **Interface shape.** What does a verifier contract expose? A plausible minimal shape: `fn verify(env: Env, task: Task, proof: Bytes) -> bool`, but consider whether the verifier needs the *keeper's* address too (to verify a proof is bound to the specific keeper claiming credit, not replayable by anyone who observes it).
- **Failure semantics.** If the verifier call panics, does `execute_task` revert entirely (safe, but lets a broken verifier permanently brick a task) or catch the panic and reject with a typed error (requires cross-contract panic isolation — check what Soroban's host actually offers here before assuming it's possible)?
- **Resource budget.** A cross-contract call costs CPU/memory budget charged to the calling transaction. Who pays if the verifier is expensive — should there be a documented budget ceiling `execute_task` reserves for the verifier call?
- **Attachment timing.** Is a verifier chosen at `register_task` time (immutable once a keeper has claimed, per 0082) or can it be updated later by the owner?
- **Trust model.** Is any address allowed to be a verifier (permissionless, per the protocol's general design philosophy), or does the registry need an admin-curated allow-list (0092 explores this as a fork)?
- **Backward compatibility.** Existing tasks (and any dApp integration written against the current ABI) have no verifier concept. How does a task with no verifier attached behave — presumably identically to today (0087)?

## Expected output

A markdown document (`docs/VERIFIER_DESIGN.md`) answering each question above with a decision and its rationale, plus the exact Rust trait/interface signature the rest of the epic will implement.

## Acceptance criteria

- [ ] Every question above has an explicit decision, not left open.
- [ ] The exact interface signature is pinned in Rust syntax.
- [ ] Backward compatibility with existing (no-verifier) tasks is explicitly addressed.
- [ ] Reviewed and agreed before any of 0072–0096 begin implementation — this issue should be closed (or its decisions locked) before those are picked up.

## Files

- `docs/VERIFIER_DESIGN.md`
