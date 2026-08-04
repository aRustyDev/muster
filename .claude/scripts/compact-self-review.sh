#!/usr/bin/env bash
# SessionStart(compact) hook — the W6 mechanical trigger (ADR-0027 §7).
# Injects the self-review reminder into the freshly-compacted context.
# PreCompact/PostCompact cannot inject context (verified 2026-08-03,
# code.claude.com/docs/en/hooks.md), so the review runs over what
# compaction retained; the slice close-out step is the deliberate,
# full-context harvest.
cat <<'EOF'
{"hookSpecificOutput":{"hookEventName":"SessionStart","additionalContext":"Compaction just occurred. Per ADR-0027 §7, run the indicator self-review now: scan the retained context for ledger classes (repeated procedure · context pollution · repeated delegation shape · correction pattern · convention stated but not recorded · whenever-X-by-memory · permission friction · re-composed pipeline · blocked-on-owner) and append dated rows to .claude/observations.md before resuming work. Also scan loaded instructions against repo state; fix or ledger stale instances (F-13)."}}
EOF
