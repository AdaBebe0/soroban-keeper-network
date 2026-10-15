---
title: "chore(docs): build and deploy the docs site from CI"
labels: [docs, tooling, intermediate]
epic: E20
wave: 5
depends_on: [0573]
---

## Summary

Build the site on PRs (fail on broken build) and deploy it on pushes to `main`.

## Acceptance criteria

- [ ] PRs touching docs/ build the site.
- [ ] `main` deploys to GitHub Pages.

## Files

- .github/workflows/docs.yml
