---
title: "feat(verifiers): reference implementation — oracle price attestation verifier"
labels: [contract, enhancement, advanced]
epic: E04
wave: 2
depends_on: [0071, 0074]
---

## Summary

The second reference verifier: proves an `OraclePricePush` task (one of the `TaskType` variants already defined in `lib.rs`) was executed against a genuine, sufficiently-recent oracle price, rather than a keeper-fabricated number. This is a natural pairing with epic E11 (Oracle Integration) in the full roadmap, and should be built with an eye toward reuse once that epic starts, but scoped narrowly here to just what E04 needs: a working verifier example.

## Expected behaviour

A verifier contract that, given a `proof` encoding a price and a timestamp, cross-checks it against a configured oracle contract's own on-chain state (read via a cross-contract call, not trusted from the `proof` bytes alone) and confirms:
- The price in the proof matches what the oracle currently reports (within a configurable tolerance, since prices can move between the keeper reading and the transaction landing).
- The oracle's own last-updated timestamp is recent enough to not be stale.

## Suggested approach

Use a minimal mock oracle contract for this issue's tests (a real Reflector/Band integration is out of scope until E11) — the point here is proving the verifier *pattern* of cross-checking a proof against independently-readable on-chain state, not integrating a specific production oracle.

## Acceptance criteria

- [ ] Verifies a proof against a mock oracle's current state.
- [ ] Rejects a proof whose claimed price doesn't match the oracle.
- [ ] Rejects a proof based on stale oracle data (past a configurable staleness threshold).
- [ ] Documents explicitly that this is a reference pattern, not a production-ready oracle integration — E11 will need to adapt it to a real oracle's actual interface.

## Files

- `contracts/verifiers/oracle-verifier/src/lib.rs`
- `contracts/verifiers/oracle-verifier/src/test.rs`
