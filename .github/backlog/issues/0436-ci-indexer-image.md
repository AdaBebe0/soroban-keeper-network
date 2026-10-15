---
title: "chore(ci): publish a container image for the indexer"
labels: [tooling, indexer, intermediate]
epic: E21
wave: 5
depends_on: []
---

## Summary

docs/INDEXER_DEPLOYMENT.md assumes operators build the indexer themselves. Publish a versioned image to GHCR on release tags.

## Acceptance criteria

- [ ] A multi-stage Dockerfile builds a minimal runtime image that runs as a non-root user.
- [ ] A tag-triggered workflow pushes `ghcr.io/<org>/keeper-indexer:<version>`.
- [ ] docs/INDEXER_DEPLOYMENT.md documents the image and its environment variables.

## Files

- indexer/Dockerfile
- .github/workflows/indexer-image.yml
- docs/INDEXER_DEPLOYMENT.md
