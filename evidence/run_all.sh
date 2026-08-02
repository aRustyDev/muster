#!/usr/bin/env bash
# Reproduces every figure in RESEARCH-0002/0003/0004. Order matters.
# Artefacts land in $ORRERY_WORK (default: evidence/_work).
set -euo pipefail
cd "$(dirname "$0")/.."
export ORRERY_WORK="${ORRERY_WORK:-$(pwd)/evidence/_work}"

echo "== install ==";            pip install ladybug --break-system-packages -q
echo "== probe 1: stop-gate ==   (~10s)";  python3 evidence/probe_01_recursive.py
echo "== spike S ==              (~5s)";   python3 evidence/orrery_spike.py S
echo "== spike M ==              (~30s)";  python3 evidence/orrery_spike.py M
echo "== probe 2: saturation ==  (~60s)";  python3 evidence/probe_02_cascade.py
echo "== spike L ==              (~4min)"; python3 evidence/orrery_spike.py L
echo "== sqlite head-to-head ==  (~2min)"; python3 evidence/sqlite_compare.py L
echo "== done. RESEARCH-0003 tables should reproduce within run-to-run noise. =="
