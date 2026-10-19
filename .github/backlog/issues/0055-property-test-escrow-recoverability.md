---
title: "test(property): assert escrow recoverability (I-2) — every task reaches a terminal state with funds resolved"
labels: [testing, contract, advanced]
epic: E03
wave: 2
depends_on: [0050, 0054]
---

## Summary

Invariant I-2 from issue 0050: every escrowed reward has at least one reachable path back out — to the owner via cancel or expire, or to a keeper via execute and withdraw. No state should strand funds permanently. This issue is a property test that drives a task through randomized valid transitions and asserts a terminal, fund-resolved state is always reachable.

## Expected behaviour

For any task in `Pending` or `Claimed` status, at least one of the following sequences of valid calls (given enough ledger/time advancement) succeeds and results in the escrow leaving the contract:
- `cancel_task` (Pending only)
- advance past deadline, then `expire_task`
- `claim_task` → `execute_task` → `withdraw_rewards`

The property test should attempt to construct an adversarial task (edge-case `ttl_ledgers`, `lock_ledgers`, `deadline`) and confirm none of them produce a task that is stuck — neither claimable, cancellable, nor expirable.

## Suggested approach

This is naturally a *reachability* property rather than a *conservation* one (0054): for each generated task, try each of the three exit paths under simulated ledger advancement and assert exactly one succeeds given the task's current status, and that after advancing sufficiently far, `expire_task` is always eventually available as a fallback for any non-terminal task.

Cross-reference issue 0005 (ttl shorter than deadline strands escrow) — if that bug is still open when this lands, this property test should fail in a way that clearly identifies it, and the issue should note that dependency rather than silently skipping the case.

## Acceptance criteria

- [ ] The property generates tasks across the full valid `ttl_ledgers`/`lock_ledgers`/`deadline` space (post-0006 validation).
- [ ] It proves reachability of a terminal, fund-resolved state for every generated task.
- [ ] If issue 0005 is unresolved, this test documents the known failing case with a `#[should_panic]` or explicit skip and a comment linking the issue — it must not be silently green over a real gap.
- [ ] Once 0005 is fixed, the exemption above is removed in the same PR or a fast follow-up is filed.

## Files

- `contracts/keeper-registry/src/test.rs` or `contracts/keeper-registry/tests/proptest_recoverability.rs`
