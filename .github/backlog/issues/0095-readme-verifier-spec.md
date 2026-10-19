---
title: "docs(readme): update the functional requirements spec with the optional verifier field"
labels: [docs, good-first-issue]
epic: E04
wave: 2
depends_on: [0073, 0074, 0081]
---

## Summary

The README's Functional Requirements section (FR-1 through FR-7, referenced throughout wave 1's issues) is the closest thing this repository has to a normative spec. It currently says nothing about verification, since the MVP had none. This issue brings it up to date once the epic's core mechanics (0073, 0074, 0081) have landed.

## Expected behaviour

A new or extended FR entry stating, precisely and testably (matching the style of the existing FR entries, e.g. FR-7's admin controls table):
- `register_task` MUST accept an optional verifier address.
- `execute_task` MUST call the attached verifier (if any) and MUST NOT credit the keeper if it returns false.
- `update_verifier` MUST be restricted to `Pending` tasks and MUST require owner auth.

## Acceptance criteria

- [ ] New FR entry added, numbered consistently with the existing FR-1..FR-7 scheme.
- [ ] Each statement is precise enough to map directly to a test (matching the spirit of issue 0050's invariant-writing guidance: precise enough to be testable, not just descriptive prose).
- [ ] Storage model table (already touched by wave 1's issue 0001 PR) reflects the new `Task.verifier` field.

## Files

- `README.md`
