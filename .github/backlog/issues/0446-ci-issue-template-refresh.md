---
title: "chore(github): refresh issue templates for the current component list"
labels: [tooling, good-first-issue]
epic: E21
wave: 5
depends_on: []
---

## Summary

The issue forms in .github/ISSUE_TEMPLATE predate the indexer, SDKs, treasury and bot v2. Their component dropdowns should list every component a reporter might mean.

## Acceptance criteria

- [ ] bug_report.yml and feature_request.yml list contracts, sdk-ts, rust-sdk, indexer, indexer-client, keeper-bot, keeper-bot-v2 and docs.
- [ ] config.yml links to SECURITY.md for vulnerability reports.

## Files

- .github/ISSUE_TEMPLATE/bug_report.yml
- .github/ISSUE_TEMPLATE/feature_request.yml
- .github/ISSUE_TEMPLATE/config.yml
