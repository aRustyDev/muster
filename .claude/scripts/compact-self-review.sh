#!/usr/bin/env bash
# SessionStart(compact) hook — the W6 mechanical trigger (ADR-0027 §7).
# Injects the self-review reminder into the freshly-compacted context.
# PreCompact/PostCompact cannot inject context (verified 2026-08-03,
# code.claude.com/docs/en/hooks.md), so the review runs over what
# compaction retained plus the transcript on disk; the slice close-out
# step remains the deliberate, full-context harvest.
#
# 2026-08-03 (owner-directed): the harvest is delegated to the
# user-level retro agents (retro-debrief · retro-lessons ·
# retro-knowledge, in ~/.claude/agents/), which read the session
# transcript and write ledger rows through
# ~/.claude/scripts/append-observation.sh — the single write door.
# Hook stdin (JSON) supplies transcript_path and session_id; jq builds
# the payload, with a static heredoc fallback when jq is absent.
#
# Manual test:
#   echo '{"transcript_path":"/tmp/t.jsonl","session_id":"x"}' | ./compact-self-review.sh

input="$(cat)"

if command -v jq >/dev/null 2>&1; then
  transcript="$(printf '%s' "$input" | jq -r '.transcript_path // empty' 2>/dev/null)"
  session="$(printf '%s' "$input" | jq -r '.session_id // empty' 2>/dev/null)"

  ctx="Compaction just occurred. Per ADR-0027 §7, run the indicator self-review now, before resuming work.

Transcript: ${transcript:-unknown — take the newest .jsonl under ~/.claude/projects/ for this cwd}
Session id: ${session:-unknown}

Preferred path: launch the user-level agents retro-debrief, retro-lessons, and retro-knowledge IN PARALLEL, passing each the transcript path above, a short session label, and scope=project. They append rows to .claude/observations.md through ~/.claude/scripts/append-observation.sh (the single write door) and report back; review their reports for duplicates and mis-classed rows.

Fallback (agents unavailable): scan the retained context yourself for the ledger classes (repeated procedure · context pollution · repeated delegation shape · correction pattern · convention stated, not recorded · whenever-X-by-memory · permission friction · re-composed pipeline · blocked-on-owner) and append rows via the same script.

Either way: scan loaded instructions against repo state; fix or ledger stale instances (F-13)."

  jq -n --arg ctx "$ctx" \
    '{hookSpecificOutput:{hookEventName:"SessionStart",additionalContext:$ctx}}'
else
  cat <<'EOF'
{"hookSpecificOutput":{"hookEventName":"SessionStart","additionalContext":"Compaction just occurred. Per ADR-0027 §7, run the indicator self-review now: launch the user-level agents retro-debrief, retro-lessons, and retro-knowledge in parallel with the newest transcript .jsonl under ~/.claude/projects/ for this cwd (scope=project); they append rows via ~/.claude/scripts/append-observation.sh. Fallback: scan the retained context for the ledger classes and append rows via the same script. Also scan loaded instructions against repo state; fix or ledger stale instances (F-13)."}}
EOF
fi
