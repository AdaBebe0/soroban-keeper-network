---
title: "feat(keeper-bot): support fetching or generating proofs for verifier-gated tasks"
labels: [keeper-bot, enhancement, advanced]
epic: E04
wave: 2
depends_on: [0074, 0077]
---

## Summary

The reference keeper bot (`examples/keeper-bot/index.js`) currently submits a fixed placeholder proof (per wave 1's issue 0054/pluggable-executor work). Once tasks can carry a real verifier (0073), the bot needs a way to produce a proof that verifier will actually accept — which differs per verifier kind.

## Expected behaviour

Extend the bot's pluggable-executor interface (wave 1 issue 0048) so that, alongside "how do I perform the off-chain action," a task's executor can also answer "how do I construct a proof this task's verifier will accept" — for example, signing with a configured key for the signature verifier (0077).

## Suggested approach

This is naturally scoped to the signature verifier (0077) first, since it's the most self-contained of the three reference verifiers to generate a proof for programmatically (an oracle-based or inclusion-based proof, per 0078/0079, may need bot-side logic that isn't ready until those issues land — don't block this issue on all three, ship signature-verifier support and note the others as follow-ups).

## Acceptance criteria

- [ ] Bot can read a task's `verifier` field (once exposed via `get_task`) and select the right proof-generation logic.
- [ ] Signature-verifier proof generation implemented and tested against 0077's reference contract on a local/test network.
- [ ] Clear extension point documented for adding support for the other verifier kinds later.

## Files

- `examples/keeper-bot/index.js`
- `examples/keeper-bot/executors/` (or wherever wave 1's pluggable-executor work landed)
