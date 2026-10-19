#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Soroban Keeper Network — Resource cost regression report (CI)
#
# Compares contracts/keeper-registry/target/resource-report.json (written by
# the `resource_report` test in contracts/keeper-registry/src/test.rs — note
# cargo runs test binaries with the package directory as their working
# directory, not the workspace root, which is where that relative path
# resolves) against the checked-in baseline at
# contracts/keeper-registry/resource-baseline.json, and appends a diffable
# Markdown table to $GITHUB_STEP_SUMMARY — in the same spirit as the
# wasm-size job's report. See docs/CI.md.
#
# A per-entry-point delta of more than REGRESSION_THRESHOLD_PCT is flagged so
# a reviewer can spot a large regression at a glance instead of just reading
# an absolute number with no context.
#
# This script never fails the job itself (the resource-cost job is
# advisory); it only shapes the summary.
# ─────────────────────────────────────────────────────────────────────────────

set -euo pipefail

REGRESSION_THRESHOLD_PCT=10

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

CURRENT="contracts/keeper-registry/target/resource-report.json"
BASELINE="contracts/keeper-registry/resource-baseline.json"

{
  echo "### Resource cost per entry point (advisory)"
  echo
  if [ ! -f "$CURRENT" ]; then
    echo "No resource report was produced (\`$CURRENT\` missing) — see the previous step's log."
  elif [ ! -f "$BASELINE" ]; then
    echo "No baseline found at \`$BASELINE\` — showing absolute numbers only."
    echo
    echo "| Entry point | CPU instructions | Memory bytes |"
    echo "|---|---|---|"
    jq -r '.entry_points[] | "| `\(.name)` | \(.cpu_instructions) | \(.memory_bytes) |"' "$CURRENT"
  else
    echo "| Entry point | CPU instructions | Δ vs baseline | Memory bytes | Δ vs baseline |"
    echo "|---|---|---|---|---|"
    jq -n \
      --slurpfile current "$CURRENT" \
      --slurpfile baseline "$BASELINE" \
      --argjson threshold "$REGRESSION_THRESHOLD_PCT" -r '
      def by_name(arr): arr[0].entry_points | map({(.name): .}) | add // {};
      (by_name($current)) as $cur |
      (by_name($baseline)) as $base |
      $cur | keys[] as $name |
      ($cur[$name].cpu_instructions) as $cpu |
      ($cur[$name].memory_bytes) as $mem |
      ($base[$name].cpu_instructions // null) as $base_cpu |
      ($base[$name].memory_bytes // null) as $base_mem |
      (if $base_cpu == null then "n/a (new)"
       else
         (($cpu - $base_cpu) as $d |
          ($d * 100 / (if $base_cpu == 0 then 1 else $base_cpu end)) as $pct |
          "\(if $d >= 0 then "+" else "" end)\($d) (\($pct)%)" +
          (if ($pct >= $threshold or $pct <= -$threshold) then " " else "" end))
       end) as $cpu_delta |
      (if $base_mem == null then "n/a (new)"
       else
         (($mem - $base_mem) as $d |
          ($d * 100 / (if $base_mem == 0 then 1 else $base_mem end)) as $pct |
          "\(if $d >= 0 then "+" else "" end)\($d) (\($pct)%)" +
          (if ($pct >= $threshold or $pct <= -$threshold) then " " else "" end))
       end) as $mem_delta |
      "| `\($name)` | \($cpu) | \($cpu_delta) | \($mem) | \($mem_delta) |"
    '
    echo
    echo "marks a change of ${REGRESSION_THRESHOLD_PCT}% or more against \`$BASELINE\`."
    echo "Update the baseline in the same PR as an intentional cost change."
  fi
} >>"$GITHUB_STEP_SUMMARY"
