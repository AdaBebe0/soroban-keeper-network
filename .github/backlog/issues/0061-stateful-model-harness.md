---
title: "test(property): build a shared stateful model-checking harness for multi-task, multi-keeper sequences"
labels: [testing, contract, advanced]
epic: E03
wave: 2
depends_on: [0054]
---

## Summary

Several of this wave's property tests (0054, 0055, 0058) need the same underlying capability: generate a randomized, valid sequence of contract calls across multiple tasks and keepers, and check an invariant after every step. Rather than duplicating that generator three times, this issue builds it once as a shared harness the others can depend on.

## Expected behaviour

A `contracts/keeper-registry/tests/model.rs` (or `src/test_model.rs` behind `#[cfg(test)]`) module providing:
- An `Action` enum covering every state-mutating entry point with its arguments.
- A `proptest` `Strategy` that generates a `Vec<Action>` biased toward actions that are valid given the model's current tracked state (so the generator doesn't spend most of its budget on trivially-rejected calls).
- A `Model` struct tracking, in plain Rust (no contract calls), what the *expected* state should be — open tasks, keeper balances, accrued fees — updated as each `Action` is applied.
- A runner that applies each `Action` to both the real contract and the `Model`, and after every step asserts the real contract's observable state (`get_task`, `keeper_balance`, `fees_accrued`, `task_count`) matches the `Model`.

## Why build this instead of letting each property test roll its own

A hand-rolled model that tracks expected state is the actual property-testing technique here — issues 0054/0055/0058 as originally scoped could be read as "write a proptest," but the valuable version of that is "write a model and diff it against reality," which is considerably more work to get right once (input generation that's actually likely to hit interesting states, not just uniformly random garbage that's rejected immediately) than three times.

## Suggested approach

Start narrow: model only `register_task`, `claim_task`, `execute_task`, `cancel_task`, `expire_task`, `withdraw_rewards` (skip `increase_reward`/`extend_deadline`/admin functions initially) and expand once the core loop is proven out. Ledger time/sequence advancement should itself be a modeled action, not a side effect, so the generator can interleave "wait N ledgers" with contract calls.

## Acceptance criteria

- [ ] `Model` tracks per-task status/reward/claimer and per-keeper balance independently of the contract.
- [ ] The generator produces sequences that are mostly-valid (rejected-call rate should be measured and kept low, not just accepted as noise).
- [ ] At least one property (reasonable to reuse 0054's solvency check as the first consumer) is ported onto this harness in the same PR, so the harness ships proven-useful rather than speculative.
- [ ] Documented well enough that 0055 and 0058 can build on it without re-reading this issue.

## Files

- `contracts/keeper-registry/tests/model.rs`
