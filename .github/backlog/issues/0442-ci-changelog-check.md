---
title: "chore(ci): require a CHANGELOG entry for user-facing contract changes"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

Release notes depend on contributors remembering CHANGELOG.md. Add a PR check that fails when files under contracts/*/src change without a CHANGELOG.md change, with a `no-changelog` label to opt out.

## Acceptance criteria

- [ ] The check runs on PRs and honours the opt-out label.
- [ ] Tests-only changes under `src/test/` do not trigger it.

## Files

- .github/workflows/pr-hygiene.yml
