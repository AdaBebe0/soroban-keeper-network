---
title: "security(oracle): review oracle-conditioned tasks for manipulation risk"
labels: [oracle, security, advanced]
epic: E11
wave: 4
depends_on: [0419, 0420]
---

## Summary

A price condition is only as trustworthy as its feed. Review whether a feed can be moved cheaply enough to trigger tasks on demand, whether a stale feed can strand rewards, and whether a condition can be pointed at a contract that only imitates Reflector.

## Acceptance criteria

- [ ] Each risk above is answered with a finding or a confirmation it does not apply.
- [ ] The review states whether condition contracts must pin a known oracle address or may accept any.
- [ ] Concrete findings are fixed or recorded as accepted tradeoffs before the issue closes.

## Files

- docs/ORACLE_SECURITY_REVIEW.md
