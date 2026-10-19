---
title: "chore(ci): track CPU/memory resource cost per entry point over time"
labels: [tooling, testing, intermediate]
epic: E05
wave: 2
depends_on: []
---

## Summary

Wave 1's wasm-size advisory CI job reports the compiled contract's binary size on every PR, so a size regression is visible before merge. This issue adds the analogous report for runtime resource cost -- CPU instructions and memory -- per entry point, since a change that keeps the binary small can still make a specific function meaningfully more expensive to call (exactly the concern epic E04's verifier work raises, per issue 0076).

## Expected behaviour

A new advisory CI job that runs each entry point once via the test harness's Env (which already exposes resource-usage introspection through soroban-sdk's budget tracking utilities) and publishes a per-function instruction-count and memory-usage table to the job summary, in the same style as the existing wasm-size job.

## Suggested approach

Look at the SDK's budget-tracking APIs exposed under testutils (confirm the exact API surface for the SDK version this repo pins) for a way to read cumulative CPU instructions consumed during a call without needing a live network. Run one representative call per entry point (reusing the existing test.rs setup fixtures) and tabulate the result.

## Acceptance criteria

- [ ] A table of entry point, CPU instructions, and memory bytes is published to the job summary on every PR.
- [ ] The job is advisory (continue-on-error: true), consistent with wasm-size and the other non-blocking checks documented in docs/CI.md.
- [ ] The report is diffable enough (or explicitly compared against a baseline) that a reviewer can spot a large regression at a glance, not just see an absolute number with no context.
- [ ] Documented in docs/CI.md alongside the other advisory jobs.

## Files

- .github/workflows/ci.yml
- docs/CI.md
