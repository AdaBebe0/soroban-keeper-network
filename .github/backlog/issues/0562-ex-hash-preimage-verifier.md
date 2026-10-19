---
title: "feat(examples): hash-preimage verifier contract"
labels: [contract, enhancement, intermediate]
epic: E22
wave: 5
depends_on: []
---

## Summary

A minimal verifier for docs/VERIFIERS.md readers: execution is accepted only if the proof is the preimage of a hash set at registration.

## Acceptance criteria

- [ ] The verifier implements the interface from docs/VERIFIER_DESIGN.md.
- [ ] Tests cover a correct proof, a wrong proof and an empty proof.

## Files

- examples/verifiers/hash-preimage/
