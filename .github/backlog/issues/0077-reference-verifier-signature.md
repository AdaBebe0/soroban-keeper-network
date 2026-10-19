---
title: "feat(verifiers): reference implementation — signature-based proof verifier"
labels: [contract, enhancement, intermediate]
epic: E04
wave: 2
depends_on: [0071, 0074]
---

## Summary

The first of three reference verifier contracts (this, 0078, 0079) that exercise the interface from 0071 end-to-end and give integrators a working example to copy. This one verifies that `proof` is a valid ed25519 signature over the task's calldata, produced by a key the task owner designates at registration.

## Expected behaviour

A separate contract crate (`contracts/verifiers/signature-verifier/`) implementing the `IKeeperVerifier` interface from 0071:
- Constructor/init takes the expected signer's public key.
- `verify(env, task, proof)` decodes `proof` as a signature, reconstructs the signed message from `task.calldata` (and `task.id` or similar, to prevent a valid signature over one task being replayed against another — get this binding right, it's the actual security property), and checks it against the configured public key using Soroban's crypto host functions.

## Suggested approach

This is the most self-contained of the three reference verifiers — no external oracle or event-log dependency — and is a reasonable one to build first to shake out the interface from 0071 against something concrete.

## Acceptance criteria

- [ ] A valid signature over the correct task verifies successfully.
- [ ] A signature valid for a *different* task_id is rejected (replay protection).
- [ ] A malformed or wrong-length `proof` is rejected without panicking.
- [ ] An end-to-end test registers a task with this verifier attached, executes with a real generated signature, and confirms the reward is credited.

## Files

- `contracts/verifiers/signature-verifier/src/lib.rs`
- `contracts/verifiers/signature-verifier/src/test.rs`
