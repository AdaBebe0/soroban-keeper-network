---
title: "feat(dashboard): read-only admin status panel"
labels: [dashboard, enhancement, good-first-issue]
epic: E17
wave: 5
depends_on: [0530]
---

## Summary

Show admin-controlled parameters (admin address, fee bps, min reward, min stake, dispute window, reputation floor) in one place. Read-only by design; admin writes stay in the CLI.

## Acceptance criteria

- [ ] Every parameter has a short explanation.
- [ ] No write action is exposed.

## Files

- apps/dashboard/src/pages/Admin.tsx
