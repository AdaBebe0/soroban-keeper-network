---
title: "feat(oracle): a typed client for the Reflector price-feed interface"
labels: [oracle, contract, intermediate]
epic: E11
wave: 4
depends_on: [0416]
---

## Summary

Condition contracts need to call Reflector's `lastprice`/`decimals` methods. Wrap that interface once in a small shared crate so each condition contract does not re-declare it.

## Acceptance criteria

- [ ] The crate declares the Reflector client with the exact method signatures of the deployed contract, with a link to the source of truth.
- [ ] Prices come back with their timestamp so callers can apply the staleness rule.
- [ ] It is `no_std` and usable from any condition contract in the workspace.

## Files

- contracts/oracle-client/src/lib.rs
- Cargo.toml
