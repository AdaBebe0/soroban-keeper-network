---
title: "security(ops): review metrics and logs for sensitive data"
labels: [observability, security, intermediate]
epic: E18
wave: 5
depends_on: [0475, 0478]
---

## Summary

Metrics labels and logs can leak keys, database URLs or internal addresses. Review every exported label and log field.

## Acceptance criteria

- [ ] No label or log field can contain a secret, a full database URL or a signing key.
- [ ] Metrics endpoints have a documented exposure recommendation (private network or auth).

## Files

- docs/OBSERVABILITY.md
