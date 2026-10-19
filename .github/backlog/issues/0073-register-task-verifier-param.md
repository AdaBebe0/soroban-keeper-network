---
title: "feat(registry): register_task accepts an optional verifier address"
labels: [contract, enhancement, intermediate]
epic: E04
wave: 2
depends_on: [0072]
---

## Summary

Extends `register_task` with an eighth parameter, `verifier: Option<Address>`, so a task owner can opt into on-chain proof verification at registration time. This is additive to the ABI in a breaking way (an existing 7-argument call site won't compile against the new signature) — the PR must update every call site in the same change, per 0071's backward-compatibility answer.

## Expected behaviour

- `register_task(e, owner, task_type, calldata, reward, deadline, ttl_ledgers, lock_ledgers, verifier: Option<Address>) -> Result<u64, KeeperError>`.
- `verifier: None` behaves exactly as `register_task` does today.
- `verifier: Some(addr)` stores `addr` on the `Task` (using the field from 0072) but does not yet change `execute_task`'s behavior — that's 0074. This issue is scoped to plumbing the parameter through, not consuming it.

## Suggested approach

Since this changes a public function's arity, update every existing call site: the README integration snippet, `test.rs`'s `setup()`/`register_default_task()` helpers, and the keeper-bot example if it constructs a `register_task` invocation directly. Search broadly rather than assuming `lib.rs` is the only place this signature is referenced.

## Acceptance criteria

- [ ] New parameter added; existing behavior unchanged when `None`.
- [ ] Every call site across the repo (contract tests, README examples, keeper-bot if applicable) updated to the new arity.
- [ ] `#[allow(clippy::too_many_arguments)]` comment (already present) updated if the justification needs adjusting for the new count.
- [ ] CHANGELOG entry noting the breaking ABI change.

## Files

- `contracts/keeper-registry/src/lib.rs`
- `contracts/keeper-registry/src/test.rs`
- `README.md`
- `CHANGELOG.md`
