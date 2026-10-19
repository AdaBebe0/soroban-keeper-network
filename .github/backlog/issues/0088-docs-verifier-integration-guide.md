---
title: "docs: verifier integration guide for dApp authors"
labels: [docs, intermediate]
epic: E04
wave: 2
depends_on: [0074, 0077, 0078, 0079]
---

## Summary

By the time 0071–0087 land, a dApp author registering a task will have a real decision to make: attach a verifier or not, and if so, which one, or how to write their own. Nothing currently explains that decision to them. This issue is the integration guide.

## Expected behaviour

A new `docs/VERIFIERS.md` (or a substantial new section in `docs/DEMO.md`/`ARCHITECTURE.md` if that fits better structurally) covering:
- When to attach a verifier versus relying on the base MVP trust model, with the tradeoffs stated plainly (a verifier costs extra resource budget per the findings from 0076, but closes the "keeper submits garbage proof" gap named in the README's Known Design Decisions).
- How to use each of the three reference verifiers (0077, 0078, 0079) as-is.
- The minimal interface a custom verifier must implement, with a worked example.
- The failure-handling and budget implications from 0075/0076, framed for someone integrating rather than someone building the registry itself.

## Acceptance criteria

- [ ] A dApp author can read this document and decide, correctly, whether they need a verifier for their use case.
- [ ] Each reference verifier is documented with its actual constructor/init parameters and a copy-pasteable registration example.
- [ ] Cross-references `docs/VERIFIER_DESIGN.md` (0071) for the underlying rationale rather than restating it.

## Files

- `docs/VERIFIERS.md`
