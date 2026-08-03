#!/usr/bin/env python3
"""Regenerate the generated section of docs/src/SUMMARY.md (ADR-0027).

Everything above and including the marker line is hand-authored and
preserved verbatim; everything below is generated deterministically:
Decision records (docs/src/adrs/ and docs/src/dev/adrs/, recursive,
sorted by number), then one section per dev/ class in a fixed order
(strategies, policies, patterns, roadmaps), then loose dev/*.md pages.
Chapter titles come from each file's first `# ` heading — a file
without one is an error, not a fallback.

Doors: `just docs::summary` (write) · `just docs::check-links` runs
`--check` (exit 1 if the committed file is stale).
"""

import re
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parent.parent / "src"
MARKER = "<!-- generated below by docs/scripts/gen-summary.py — hand-edit above this line only -->"
CLASSES = ["strategies", "policies", "patterns", "roadmaps"]


def title(path: Path) -> str:
    for line in path.read_text().splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    sys.exit(f"gen-summary: {path} has no `# ` heading")


def entry(path: Path) -> str:
    return f"- [{title(path)}]({path.relative_to(SRC).as_posix()})"


def adr_entries() -> list[str]:
    files: list[Path] = []
    for root in (SRC / "adrs", SRC / "dev" / "adrs"):
        if root.is_dir():
            files += [p for p in root.rglob("*.md") if re.match(r"\d{4}-", p.name)]
    dupes = [n for n in {p.name[:4] for p in files}
             if sum(1 for p in files if p.name.startswith(n)) > 1]
    if dupes:
        sys.exit(f"gen-summary: duplicate ADR numbers {sorted(dupes)}")
    return [entry(p) for p in sorted(files, key=lambda p: p.name)]


def class_entries(cls: str) -> list[str]:
    root = SRC / "dev" / cls
    if not root.is_dir():
        return []
    return [entry(p) for p in sorted(root.rglob("*.md")) if p.name != "README.md"]


def loose_dev_entries() -> list[str]:
    root = SRC / "dev"
    if not root.is_dir():
        return []
    return [entry(p) for p in sorted(root.glob("*.md")) if p.name != "README.md"]


def generate() -> str:
    lines = (SRC / "SUMMARY.md").read_text().splitlines()
    if MARKER not in lines:
        sys.exit("gen-summary: marker line not found in SUMMARY.md")
    out = lines[: lines.index(MARKER) + 1]
    out += ["", "# Decision records", ""] + adr_entries()
    for cls in CLASSES:
        entries = class_entries(cls)
        if entries:
            out += ["", f"# {cls.capitalize()}", ""] + entries
    loose = loose_dev_entries()
    if loose:
        out += ["", "# Reference", ""] + loose
    return "\n".join(out) + "\n"


def main() -> None:
    new = generate()
    path = SRC / "SUMMARY.md"
    if "--check" in sys.argv:
        if path.read_text() != new:
            sys.exit("gen-summary: SUMMARY.md is stale — run `just docs::summary`")
        print("SUMMARY.md current")
    else:
        path.write_text(new)
        print("SUMMARY.md regenerated")


if __name__ == "__main__":
    main()
