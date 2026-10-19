---
title: "test(registry): confirm and pin that update_verifier is rejected once a task is claimed"
labels: [contract, security, good-first-issue]
epic: E04
wave: 2
depends_on: [0081]
---

## Summary

0081 restricts `update_verifier` to `Pending` tasks. This issue is the dedicated security-reasoning and test coverage for *why* that restriction matters, kept separate from 0081's general feature-completeness criteria because the security property deserves its own explicit regression test rather than being an incidental side effect of the status match arm.

## The concern

A keeper claims a task with no verifier attached (or an easy-to-satisfy one), begins preparing its execution (performing the off-chain action, building the `execute_task` transaction), and — before that transaction lands — the owner swaps in a verifier the keeper cannot satisfy. The keeper has now done real off-chain work for a task it can no longer collect on, with no compensation. This is a griefing vector against keepers, the mirror image of the keeper-squatting concern that motivated `lock_ledgers` (wave 1 issue 0016).

## Expected behaviour

`update_verifier` on a `Claimed` (or any non-`Pending`) task fails with `InvalidTaskStatus`, with a test that specifically constructs this scenario: claim, attempt update, assert rejection, confirm the original verifier (or lack thereof) is unchanged.

## Acceptance criteria

- [ ] Test claims a task, then attempts `update_verifier`, and asserts `InvalidTaskStatus`.
- [ ] Test confirms the task's verifier field is unchanged after the rejected attempt.
- [ ] The rationale above (griefing protection) is captured in a doc comment on `update_verifier`, not just in this issue — so a future contributor loosening the restriction understands what they'd be reintroducing.

## Files

- `contracts/keeper-registry/src/lib.rs` (doc comment)
- `contracts/keeper-registry/src/test.rs`
