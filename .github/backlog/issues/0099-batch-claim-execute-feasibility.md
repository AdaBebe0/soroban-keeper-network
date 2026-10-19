---
title: "design(registry): feasibility study -- batch claim and batch execute"
labels: [contract, docs, advanced]
epic: E05
wave: 2
depends_on: [0098]
---

## Summary

Batch registration (0098) is straightforward because all tasks in the batch share one owner and no cross-task interaction. Batch claiming or executing is a different problem: a keeper bot competing against other keepers for multiple tasks at once introduces race conditions a naive batch implementation could make worse, not better. This issue is a feasibility study, not a commitment to build it -- the output may reasonably be "don't."

## The core risk

claim_task's permissionless first-come-first-served design depends on each claim being an independent, atomic check against that one task's current status. A batch claim spanning task A and task B, submitted as one transaction, either succeeds for both or reverts for both under Soroban's atomicity -- which means if another keeper claims task B a moment before this transaction lands, the entire batch fails, including the claim on task A that would have succeeded standing alone. This could make batching strictly worse for a keeper competing in a busy market, not better.

## Questions to answer

- Does batch claiming actually help a keeper bot in practice, given the atomicity risk above, or does it only help in low-contention scenarios where the benefit is smallest anyway?
- Is there a version of batching that sidesteps this -- e.g. a claim_first_available(candidates) that claims whichever one of several candidate tasks is still available, rather than trying to claim all of them?
- For batch execute (a keeper submitting proofs for several tasks it already holds exclusive claims on): does the same atomicity risk apply? The keeper already holds the lock on each task, so a competing claim is not the failure mode -- but a verifier (epic E04) failing on one task in the batch could still cause an all-or-nothing revert that blocks crediting for the others. Same question: is there value here?

## Expected output

A written recommendation: build batch claim, build the claim_first_available alternative instead, build batch execute, or explicitly decide none of these are worth the complexity given the findings -- any of these is an acceptable outcome, including "don't build it."

## Acceptance criteria

- [ ] Both questions above are answered with reasoning, not asserted.
- [ ] A clear recommendation is made.
- [ ] If something is recommended for building, it is scoped as a new, separate issue rather than silently expanding this one from study to implementation.

## Files

- docs/BATCH_OPERATIONS.md
