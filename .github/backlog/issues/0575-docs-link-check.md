---
title: "chore(docs): check links in Markdown files"
labels: [docs, tooling, good-first-issue]
epic: E20
wave: 5
depends_on: []
---

## Summary

Docs reference files that move. Add a link checker for relative links and anchors in every Markdown file.

## Acceptance criteria

- [ ] Broken relative links and anchors fail the check.
- [ ] External links are checked on a schedule, not per PR.

## Files

- .github/workflows/pr-hygiene.yml
