---
title: "docs(examples): batch registration recipes"
labels: [docs, good-first-issue]
epic: E22
wave: 5
depends_on: []
---

## Summary

examples/batch-register supports JSON and CSV. Add recipes for common cases: many TTL extensions, staggered deadlines, and recovering from a partially failed batch.

## Acceptance criteria

- [ ] Each recipe has a sample input file that passes `--dry-run`.
- [ ] The partial-failure recipe explains how to resume safely.

## Files

- examples/batch-register/README.md
- examples/batch-register/recipes/
