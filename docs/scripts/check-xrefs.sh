#!/usr/bin/env bash
# Cross-reference audit. Run from the repository root: just docs::check-links
#
# Catches the defect classes an adversarial review of the handoff package found:
#   - dangling ADR / QUESTION / RESEARCH references
#   - unqualified SPEC references across product boundaries
#   - stated counts drifting from actual
set -uo pipefail
cd "$(dirname "$0")/../.."
fail=0

# Phase-2 corrections (2026-08-01): exclude generated/archival paths
# (evidence/_work, __pycache__, target, the untracked orrery-handoff source
# package) so the audit doesn't cry wolf after a harness run; skip the
# MANIFEST count check when no MANIFEST.md exists (package-specific check).
EXCLUDES=( ! -path './.git/*' ! -path './orrery-handoff/*' ! -path '*/_work/*' \
           ! -path '*/__pycache__/*' ! -path './target/*' ! -name '.DS_Store' )

echo "-- dangling numeric references --"
adrs=$(ls docs/src/adrs/*.md 2>/dev/null | sed 's|.*/||;s/-.*//' | sort -u)
qs=$(find .claude/plans -path '*questions/[0-9]*.md' | sed 's|.*/||;s/-.*//' | sort -u)
rs=$(find .claude/plans -path '*research/[0-9]*.md' | sed 's|.*/||;s/-.*//' | sort -u)
for kind in ADR:"$adrs" QUESTION:"$qs" RESEARCH:"$rs"; do
  k=${kind%%:*}; have=${kind#*:}
  grep -rhoE "$k-[0-9]{4}" --include=*.md --exclude-dir=orrery-handoff . 2>/dev/null | sed "s/$k-//" | sort -u |
  while read -r n; do
    grep -qx "$n" <<< "$have" || { echo "  DANGLING $k-$n"; fail=1; }
  done
done

echo "-- unqualified SPEC references outside their own product directory --"
grep -rn "SPEC [0-9][0-9]" --include=*.md --exclude-dir=orrery-handoff . 2>/dev/null \
  | grep -v '/specs/' | grep -v '\.claude/plans/README\.md' \
  && { echo "  ^ qualify as orrery/SPEC-NN (see .claude/plans/README.md)"; fail=1; } \
  || echo "  none"

echo "-- file count vs MANIFEST --"
if [ -f MANIFEST.md ]; then
  actual=$(find . -type f "${EXCLUDES[@]}" | wc -l | tr -d ' ')
  claimed=$(grep -oE '\*\*Contents:\*\* [0-9]+' MANIFEST.md | grep -oE '[0-9]+' || echo "?")
  [ "$actual" = "$claimed" ] && echo "  ok ($actual)" \
    || { echo "  MISMATCH: actual $actual, MANIFEST claims $claimed"; fail=1; }
else
  echo "  skipped (no MANIFEST.md at root — package-specific check)"
fi

exit $fail
