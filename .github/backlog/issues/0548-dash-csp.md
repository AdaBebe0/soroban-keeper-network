---
title: "security(dashboard): content security policy and dependency review"
labels: [dashboard, security, intermediate]
epic: E17
wave: 5
depends_on: [0524]
---

## Summary

The dashboard talks to wallets and signs transactions. Set a strict CSP, review third-party scripts, and confirm no data leaves for analytics without consent.

## Acceptance criteria

- [ ] A CSP header or meta tag allows only required origins.
- [ ] No third-party script loads at runtime.

## Files

- apps/dashboard/index.html
- docs/DASHBOARD.md
