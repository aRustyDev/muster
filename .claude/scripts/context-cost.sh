#!/usr/bin/env bash
# Auto-loaded context cost in lines — the ADR-0027 §8.3 measurement.
# Unconditional files load at launch in every session; paths:-scoped
# rules load only when Claude reads a matching file. Run from repo root:
#   .claude/scripts/context-cost.sh
# Baseline (2026-08-03, pre-restructure): 404 unconditional lines.
set -euo pipefail
uncond=0; scoped=0
printf '%-48s %6s  %s\n' "file" "lines" "loading"
for f in CLAUDE.md .claude/CLAUDE.md .claude/rules/*.md; do
  [ -f "$f" ] || continue
  n=$(wc -l < "$f" | tr -d ' ')
  if head -1 "$f" | grep -q '^---$' && \
     awk '/^---$/{c++; next} c==1 && /^paths:/{found=1} END{exit !found}' "$f"; then
    scoped=$((scoped+n)); printf '%-48s %6d  scoped (paths:)\n' "$f" "$n"
  else
    uncond=$((uncond+n)); printf '%-48s %6d  unconditional\n' "$f" "$n"
  fi
done
echo
echo "unconditional at-launch total: $uncond lines (baseline 2026-08-03: 404)"
echo "paths:-scoped on-demand total: $scoped lines"
