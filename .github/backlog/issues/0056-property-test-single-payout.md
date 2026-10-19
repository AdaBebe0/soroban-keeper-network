---
title: "test(property): assert single payout (I-3) — no task's reward is ever paid out twice"
labels: [testing, contract, advanced]
epic: E03
wave: 2
depends_on: [0050]
---

## Summary

Invariant I-3: each task's reward is paid out exactly once — not zero times, not twice. Wave 1 fixed two concrete violations of the surrounding CEI ordering (issues 0002, 0003). This issue generalizes those regression tests into a property that would have caught both classes of bug, and stays as a permanent guard against a third one being introduced later.

## Expected behaviour

A property test that, for a randomly generated task, attempts every terminal transition twice in a row (`cancel_task` then `cancel_task` again, `expire_task` then `expire_task` again, `execute_task` then a second `execute_task` from the same or a different address) and asserts:
- The second call always fails with a typed error (never a panic, never a silent no-op that returns `Ok`).
- The token balance moved by the transfer changes exactly once across both calls.

## Suggested approach

This can reuse the reentrant-token pattern introduced in wave 1's CEI fix PRs (a mock token contract whose `transfer` calls back into the registry) as one *mode* of double-payout attempt, and a plain sequential double-call (no reentrancy involved) as the other mode — the two are different bug classes and both need coverage, per the reasoning already written into those PRs' regression tests.

## Acceptance criteria

- [ ] Covers all three payout paths: `cancel_task`, `expire_task`, `execute_task` → `withdraw_rewards`.
- [ ] Covers both reentrant and plain-sequential double-call attempts.
- [ ] Fails loudly (not just "no crash") if a second payout ever transfers a nonzero amount.
- [ ] References `I-3` from `docs/ARCHITECTURE.md`.

## Files

- `contracts/keeper-registry/src/test.rs`
