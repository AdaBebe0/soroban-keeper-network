---
title: "test(registry): execute_task against a verifier that consumes excessive resources"
labels: [testing, contract, advanced]
epic: E04
wave: 2
depends_on: [0076]
---

## Summary

Complements 0076's investigation with an actual test: what happens when the attached verifier does something resource-intensive — a large loop, big storage reads — that pushes the overall transaction close to or past its budget?

## Expected behaviour

A test-only verifier that deliberately does expensive work (e.g. writes a large number of storage entries, or loops a configurable number of times) proportional to a parameter the test controls, used to find the point at which `execute_task` starts failing due to resource exhaustion rather than any contract logic. Confirms the failure mode at that boundary is a clean, typed resource-limit error from the host (or whatever Soroban's actual behavior is — document it) rather than something that leaves storage in an inconsistent state.

## Acceptance criteria

- [ ] Test establishes the actual resource-exhaustion failure mode empirically.
- [ ] Confirms no partial state mutation occurs if the verifier call exhausts the budget mid-execution (i.e., the whole transaction aborts cleanly, consistent with 0075's investigation).
- [ ] Findings feed back into 0076's documentation if they reveal anything not already captured there.

## Files

- `contracts/keeper-registry/src/test.rs`
