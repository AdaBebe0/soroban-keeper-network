---
title: "chore(ci): test the TypeScript packages on every supported Node version"
labels: [tooling, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

packages/sdk-ts and packages/indexer-client declare `engines.node >=18` but CI only runs Node 20. Run their tests on 18, 20 and 22 so the declared range is actually true, or narrow it.

## Acceptance criteria

- [ ] The sdk-ts and indexer-client jobs run as a Node version matrix.
- [ ] `engines.node` matches the tested range in every package.json.

## Files

- .github/workflows/ci.yml
- packages/sdk-ts/package.json
- packages/indexer-client/package.json
