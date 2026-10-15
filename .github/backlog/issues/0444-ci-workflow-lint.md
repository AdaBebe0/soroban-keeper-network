---
title: "chore(ci): lint GitHub Actions workflows with actionlint"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

ci.yml is large and edited by many contributors; expression typos only show up when a job runs. Add actionlint as a fast check on PRs that touch `.github/workflows/`.

## Acceptance criteria

- [ ] actionlint runs on workflow changes and passes on the current files.
- [ ] Any existing finding is fixed in the same PR.

## Files

- .github/workflows/pr-hygiene.yml
