---
title: "security(cli): review key handling and admin commands before release"
labels: [cli, security, advanced]
epic: E16
wave: 5
depends_on: [0453, 0463, 0465]
---

## Summary

The CLI holds signing keys and can pause the protocol or sweep fees. Review it before the first publish.

## Acceptance criteria

- [ ] Key sources, logging, error paths and temp files are checked for secret leakage.
- [ ] Admin and deploy confirmations are checked so a typo cannot hit mainnet unprompted.
- [ ] Findings are fixed or recorded as accepted tradeoffs.

## Files

- docs/CLI_SECURITY_REVIEW.md
