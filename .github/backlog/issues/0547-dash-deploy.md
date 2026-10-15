---
title: "chore(dashboard): static hosting deployment with preview builds"
labels: [dashboard, tooling, intermediate]
epic: E17
wave: 5
depends_on: [0546]
---

## Summary

Deploy the static build on pushes to `main`, and publish preview builds for PRs.

## Acceptance criteria

- [ ] Production deploys only from `main`.
- [ ] Deployment docs explain how to self-host the static files.

## Files

- .github/workflows/dashboard-deploy.yml
- docs/DASHBOARD.md
