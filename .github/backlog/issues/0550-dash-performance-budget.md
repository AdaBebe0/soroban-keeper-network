---
title: "perf(dashboard): bundle and load-time budget"
labels: [dashboard, performance, intermediate]
epic: E17
wave: 5
depends_on: [0546]
---

## Summary

Set a gzipped bundle budget and a load-time target, measured in CI like the SDK bundle-size job.

## Acceptance criteria

- [ ] CI reports bundle size against a committed baseline.
- [ ] Routes are code-split so the explorer loads without the write flows.

## Files

- apps/dashboard/vite.config.ts
- .github/workflows/ci.yml
