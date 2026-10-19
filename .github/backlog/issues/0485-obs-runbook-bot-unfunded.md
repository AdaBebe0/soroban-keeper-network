---
title: "docs(ops): runbook — keeper account running out of XLM"
labels: [observability, docs, good-first-issue]
epic: E18
wave: 5
depends_on: [0480]
---

## Summary

A keeper that cannot pay fees silently stops earning. Document detection, funding, and confirming recovery, including for multi-account pools.

## Acceptance criteria

- [ ] Detection points at a concrete metric or log line.
- [ ] Pool operators get per-account steps.

## Files

- docs/runbooks/keeper-unfunded.md
