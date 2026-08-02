# docs/ — mdbook + ADRs

* Build: `just docs::build` (mdbook). `just docs::check-links` runs mdbook
  test plus `scripts/check-xrefs.sh` (dangling ADR/QUESTION/RESEARCH refs,
  unqualified SPEC refs, count drift).
* ADRs: `src/adrs/NNNN-kebab-title.md`, MADR format, sequential, never
  reused; `just docs::adr-next` prints the next free number. Never silently
  edit an accepted ADR — supersede or amend visibly (Rule 02).
* ADR-0015 is `proposed` (open). Current highest: 0023.
* SPEC references are product-qualified everywhere outside a product's own
  specs directory: `orrery/SPEC-03`, never a bare unqualified spec number.
* `src/SUMMARY.md` must list new chapters/ADRs or mdbook won't render them.
