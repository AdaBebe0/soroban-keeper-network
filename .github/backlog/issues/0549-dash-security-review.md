---
title: "security(dashboard): review transaction-signing flows"
labels: [dashboard, security, advanced]
epic: E17
wave: 5
depends_on: [0532, 0533, 0534]
---

## Summary

Every write the dashboard offers must show the user exactly what they sign. Review each flow for misleading displays, wrong network, and replay.

## Acceptance criteria

- [ ] Each signing flow is listed with what the user sees before signing.
- [ ] Findings are fixed or recorded as accepted.

## Files

- docs/DASHBOARD_SECURITY_REVIEW.md
