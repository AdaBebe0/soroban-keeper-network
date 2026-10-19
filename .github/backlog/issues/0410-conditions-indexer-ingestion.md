---
title: "feat(indexer): ingest a task's condition and the condition-rejection event"
labels: [indexer, enhancement, intermediate]
epic: E10
wave: 4
depends_on: []
---

## Summary

Once tasks can carry a condition (issues 0391-0392) and a rejected claim emits an event (issue 0398), the indexer should store both, so the API can show which tasks are conditioned and how often a condition blocked a claim.

## Acceptance criteria

- [ ] A migration adds the condition address to the indexed task representation without editing a shipped migration.
- [ ] The rejection event is ingested with its exact payload fields and listed in indexer/README.md's event coverage.
- [ ] `GET /tasks/{task_id}` returns the condition (or null), and `indexer/openapi.yaml` is regenerated.

## Files

- indexer/migrations/
- indexer/src/ingest/mod.rs
- indexer/src/api/rest.rs
- indexer/openapi.yaml
