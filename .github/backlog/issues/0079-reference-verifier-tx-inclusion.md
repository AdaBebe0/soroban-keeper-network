---
title: "feat(verifiers): reference implementation — target-contract event-inclusion verifier"
labels: [contract, enhancement, advanced]
epic: E04
wave: 2
depends_on: [0071, 0074]
---

## Summary

The third reference verifier: proves a keeper actually performed the off-chain-coordinated action (e.g. a `Liquidation` task's target call) by checking that the target contract emitted a specific event as a result. This is the pattern most directly applicable to the `Liquidation` and `FundingRateUpdate` task types, where "did the keeper really call the lending pool's `liquidate` function" is the thing that currently has to be trusted (per the README's Known Design Decisions).

## Expected behaviour

A verifier contract whose `verify(env, task, proof)` decodes `proof` as a reference to a specific event (contract address, topics, or a ledger/tx position depending on what Soroban actually exposes for cross-contract event introspection — investigate this first, it may constrain the design significantly) and confirms that event was actually emitted by the expected target contract as part of the same transaction or a recent one.

## Investigation required before implementation

Soroban contracts have limited ability to introspect events emitted by *other* contracts, especially from prior transactions — this may not be a solved problem the way it would be on an EVM chain with full log access from within a call. Spend real time here confirming what's actually possible on Soroban today before committing to an implementation approach; if full inclusion-proof verification isn't feasible on-chain, document that finding and scope this issue down to what is (e.g., verifying the target call happened *within the same transaction* as `execute_task`, via `require_auth` chaining or a callback pattern, which is a materially different and more limited guarantee than full retroactive proof).

## Acceptance criteria

- [ ] The investigation's finding is documented in `docs/VERIFIER_DESIGN.md`, whatever it concludes.
- [ ] The implementation matches what's actually achievable, not the issue's aspirational framing above — update the acceptance criteria based on the investigation before writing the PR.
- [ ] At minimum, a working example exists that demonstrates whatever inclusion-proof pattern Soroban does support.

## Files

- `docs/VERIFIER_DESIGN.md`
- `contracts/verifiers/inclusion-verifier/src/lib.rs`
- `contracts/verifiers/inclusion-verifier/src/test.rs`
