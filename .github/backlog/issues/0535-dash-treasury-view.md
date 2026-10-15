---
title: "feat(dashboard): treasury page"
labels: [dashboard, enhancement, intermediate]
epic: E17
wave: 5
depends_on: [0522]
---

## Summary

Show the treasury contract's recipients, their shares and pending distributions using the SDK's treasury client.

## Acceptance criteria

- [ ] Shares are shown as percentages summing to 100%.
- [ ] Recent distributions come from treasury events.

## Files

- apps/dashboard/src/pages/Treasury.tsx
