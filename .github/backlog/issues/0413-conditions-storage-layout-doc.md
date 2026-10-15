---
title: "docs(registry): record the condition field in STORAGE_LAYOUT.md"
labels: [contract, docs, good-first-issue]
epic: E10
wave: 4
depends_on: []
---

## Summary

docs/STORAGE_LAYOUT.md describes every persisted `Task` field. Once issue 0391 adds `condition`, the document must describe it, including the schema-evolution finding that issue requires.

## Acceptance criteria

- [ ] The new field, its type and its default are listed.
- [ ] The schema-evolution conclusion from issue 0391 is restated with a link to the test that proves it.

## Files

- docs/STORAGE_LAYOUT.md
