---
title: "design(registry): feasibility study -- best-effort batch execute"
labels: [contract, docs, advanced]
epic: E05
wave: 2
depends_on: [0099, 0074, 0075]
---

## Summary

Issue 0099's batch claim/execute feasibility study concluded that a naive
all-or-nothing `batch_execute` should not be built, but that a **best-effort**
variant -- attempt every task in the batch, credit the ones whose verifier
passes, skip (not revert) the ones that fail -- is worth investigating once
its prerequisites exist. This issue is that follow-up, filed so 0099 stays a
study rather than silently expanding into implementation.

This issue should not be picked up until both of its dependencies have
actually landed on `main`, not just been scoped: epic E04's verifier
interface (0074) and its failure-handling policy (0075). Best-effort
semantics cannot be designed before a single `execute_task` call's verifier
failure behavior is pinned down.

## Questions to answer

- Per 0075's finding: can `execute_task` catch a panicking verifier without
  the panic unwinding the whole host invocation? If not, best-effort batching
  as described is not implementable, and this issue should conclude "don't
  build it" rather than force a design that doesn't fit Soroban's actual
  cross-contract call semantics.
- If catching is possible: does skipping a failed task (rather than
  reverting the whole batch) actually help a keeper in practice, given that,
  unlike claiming, execute is not a race against other keepers -- the keeper
  already holds every claim in the batch? Quantify against the alternative
  (independent `execute_task` calls, one per task).
- What batch size is safe given aggregate resource cost (per issue 0100's
  measurements)? A verifier's cost is not fixed or capped by the registry,
  so a batch could hit the transaction resource ceiling for reasons
  unrelated to any task's validity.

## Expected output

A written recommendation, following the same standard 0099 set: build
best-effort batch execute, or explicitly decide it is not worth the
complexity -- either is an acceptable outcome. If building it is
recommended, scope the implementation as its own separate issue rather than
expanding this one from study to implementation.

## Acceptance criteria

- [ ] Both questions above are answered with reasoning grounded in 0075's
      actual documented Soroban panic-handling finding, not assumed.
- [ ] A clear recommendation is made.
- [ ] If implementation is recommended, it is scoped as a new, separate
      issue.

## Files

- docs/BATCH_OPERATIONS.md
