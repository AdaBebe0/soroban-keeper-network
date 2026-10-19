#!/usr/bin/env bash
#
# Publish backlog issues from .github/backlog/issues/ to GitHub Issues.
#
# Each issue file carries YAML front matter (title, labels, epic, wave,
# depends_on) followed by the Markdown body. This script splits the two,
# creates the issue with `gh`, and skips any issue whose title is already
# open on the repository so it can be re-run safely. An issue whose
# depends_on lists a local issue number that hasn't been pushed yet, or is
# pushed but still open, is held back rather than created — re-run the
# script after the dependency closes to pick it up.
#
# --no-hold publishes dependents anyway, for releasing a whole wave at
# once. Each dependent's body then gets a "Depends on" section listing its
# dependencies by title, so the ordering is still visible on GitHub.
#
# --dry-run does not query GitHub, so it cannot tell what's actually
# closed — every depends_on will show as held back in dry-run output.
#
#   ./.github/backlog/push.sh --from 1 --to 50 --dry-run
#   ./.github/backlog/push.sh --from 1 --to 50
#   ./.github/backlog/push.sh --from 405 --to 604 --no-hold
#
# Requires: gh (authenticated), awk, sed.

set -euo pipefail

FROM=1
TO=600
DRY_RUN=0
NO_HOLD=0
REPO="${BACKLOG_REPO:-soroban-tooling/soroban-keeper-network}"

usage() {
  sed -n '3,22p' "$0" | sed 's/^# \{0,1\}//'
  exit "${1:-0}"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --from)    FROM="$2"; shift 2 ;;
    --to)      TO="$2";   shift 2 ;;
    --repo)    REPO="$2"; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    --no-hold) NO_HOLD=1; shift ;;
    -h|--help) usage 0 ;;
    *) echo "Unknown argument: $1" >&2; usage 1 ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ISSUE_DIR="$SCRIPT_DIR/issues"

if [ ! -d "$ISSUE_DIR" ]; then
  echo "No issues directory at $ISSUE_DIR" >&2
  exit 1
fi

# Pull existing titles once rather than querying per issue — 600 API round trips
# would rate-limit long before the run finished.
echo "Fetching existing issue titles from $REPO ..."
EXISTING="$(mktemp)"
# Titles of issues that are open (not yet closed) — used to hold back any
# issue whose depends_on isn't done yet. Title-keyed like $EXISTING, since
# gh has no cheap "closed" filter keyed by our own file numbering.
OPEN_TITLES="$(mktemp)"
trap 'rm -f "$EXISTING" "$OPEN_TITLES"' EXIT
if [ "$DRY_RUN" -eq 0 ]; then
  gh issue list --repo "$REPO" --state all --limit 1000 --json title \
    --jq '.[].title' > "$EXISTING"
  gh issue list --repo "$REPO" --state open --limit 1000 --json title \
    --jq '.[].title' > "$OPEN_TITLES"
else
  : > "$EXISTING"
  : > "$OPEN_TITLES"
fi

# Given a local issue number (e.g. "0050"), finds its front-matter title.
# Used to resolve depends_on: [0050] into the title push.sh already tracks.
title_for_issue_number() {
  local dep_file
  dep_file="$(ls "$ISSUE_DIR/$1"-*.md 2>/dev/null | head -1)"
  [ -n "$dep_file" ] && front_matter_value "$dep_file" title
}

# Reads the value of a single-line front-matter key.
front_matter_value() {
  awk -v key="$2" '
    NR == 1 && $0 == "---" { inside = 1; next }
    inside && $0 == "---"  { exit }
    inside && index($0, key ":") == 1 {
      v = substr($0, length(key) + 2)
      sub(/^[ \t]+/, "", v)
      # strip surrounding quotes and YAML flow-sequence brackets
      gsub(/^"|"$/, "", v)
      gsub(/^\[|\]$/, "", v)
      print v
      exit
    }
  ' "$1"
}

# Everything after the closing --- of the front matter is the issue body.
issue_body() {
  awk '
    NR == 1 && $0 == "---" { inside = 1; next }
    inside && $0 == "---"  { inside = 0; started = 1; next }
    started { print }
  ' "$1"
}

created=0
skipped=0
held=0
failed=0

for file in "$ISSUE_DIR"/*.md; do
  [ -e "$file" ] || continue
  base="$(basename "$file")"
  num="${base%%-*}"
  # Strip leading zeros so 0007 compares as 7, not as an invalid octal literal.
  num_int="$((10#$num))"

  if [ "$num_int" -lt "$FROM" ] || [ "$num_int" -gt "$TO" ]; then
    continue
  fi

  title="$(front_matter_value "$file" title)"
  labels="$(front_matter_value "$file" labels)"
  depends_on="$(front_matter_value "$file" depends_on)"

  if [ -z "$title" ]; then
    echo "  !  $base — no title in front matter, skipping" >&2
    failed=$((failed + 1))
    continue
  fi

  if grep -Fxq "$title" "$EXISTING" 2>/dev/null; then
    echo "  =  $base — already on GitHub, skipping"
    skipped=$((skipped + 1))
    continue
  fi

  # Hold back an issue whose depends_on lists an issue that isn't closed
  # yet — either never pushed at all, or pushed and still open. This is
  # what keeps two contributors from picking up sequenced work in
  # parallel: the dependent issue simply doesn't exist on GitHub yet for
  # anyone to claim.
  unmet=""
  dep_titles=()
  if [ -n "$depends_on" ]; then
    IFS=',' read -ra dep_nums <<< "$depends_on"
    for dep_num in "${dep_nums[@]}"; do
      dep_num="$(echo "$dep_num" | sed 's/^[ \t"]*//; s/[ \t"]*$//')"
      [ -z "$dep_num" ] && continue
      dep_title="$(title_for_issue_number "$dep_num")"
      [ -n "$dep_title" ] && dep_titles+=("$dep_title")
      if [ -z "$dep_title" ]; then
        unmet="$unmet $dep_num(no-file)"
      elif ! grep -Fxq "$dep_title" "$EXISTING" 2>/dev/null; then
        unmet="$unmet $dep_num(not-pushed)"
      elif grep -Fxq "$dep_title" "$OPEN_TITLES" 2>/dev/null; then
        unmet="$unmet $dep_num(open)"
      fi
    done
  fi
  if [ -n "$unmet" ] && [ "$NO_HOLD" -eq 0 ]; then
    echo "  ~  $base — held back, depends_on not closed:$unmet"
    held=$((held + 1))
    continue
  fi

  if [ "$DRY_RUN" -eq 1 ]; then
    echo "  +  [dry-run] $base"
    echo "       title : $title"
    echo "       labels: ${labels:-none}"
    created=$((created + 1))
    continue
  fi

  body_file="$(mktemp)"
  issue_body "$file" > "$body_file"
  if [ "$NO_HOLD" -eq 1 ] && [ "${#dep_titles[@]}" -gt 0 ]; then
    {
      printf '\n## Depends on\n\nStart after these are closed:\n\n'
      for t in "${dep_titles[@]}"; do printf -- '- %s\n' "$t"; done
    } >> "$body_file"
  fi

  # Build the --label arguments. Labels are comma-separated in front matter.
  label_args=()
  if [ -n "$labels" ]; then
    IFS=',' read -ra parts <<< "$labels"
    for l in "${parts[@]}"; do
      l="$(echo "$l" | sed 's/^[ \t"]*//; s/[ \t"]*$//')"
      [ -n "$l" ] && label_args+=(--label "$l")
    done
  fi

  if url="$(gh issue create --repo "$REPO" \
              --title "$title" \
              --body-file "$body_file" \
              "${label_args[@]}" 2>&1)"; then
    echo "  +  $base — $url"
    created=$((created + 1))
    echo "$title" >> "$EXISTING"
  else
    echo "  x  $base — failed: $url" >&2
    failed=$((failed + 1))
  fi
  rm -f "$body_file"

  # GitHub secondary rate limits trigger on rapid content creation. A short
  # pause keeps a 50-issue run comfortably under the threshold.
  sleep 2
done

echo
echo "created: $created   skipped: $skipped   held: $held   failed: $failed"
[ "$failed" -eq 0 ]
