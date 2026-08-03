# Orrery / Muster — workspace entrypoint
#
# Modules require just >= 1.31 (may still need `set unstable := true` on some
# versions). VERIFY against your installed just before relying on this; if
# modules are unavailable, the `--fallback` recipes below do the same work.

set unstable := true

mod orrery        'crates/orrery/justfile'
mod muster_sdk    'crates/muster-sdk/justfile'
mod muster        'crates/muster/justfile'
mod muster_server 'crates/muster-server/justfile'
mod docs          'docs/justfile'

# Prerequisites: just>=1.31 · cargo-nextest · cargo-public-api · mdbook · python3
#                cargo-deny · cargo-hack (QF slice, 2026-08-03)
# Install:  cargo install cargo-nextest cargo-public-api mdbook cargo-deny cargo-hack

default:
    @just --list --list-submodules

# verify the toolchain this justfile assumes
doctor:
    @cargo nextest --version    >/dev/null 2>&1 || echo "missing: cargo-nextest"
    @cargo public-api --version >/dev/null 2>&1 || echo "missing: cargo-public-api"
    @cargo deny --version       >/dev/null 2>&1 || echo "missing: cargo-deny"
    @cargo hack --version       >/dev/null 2>&1 || echo "missing: cargo-hack"
    @command -v rustup >/dev/null 2>&1 || echo "note: no rustup (Homebrew rust) - cargo-public-api cannot build rustdoc JSON; check-seam uses its grep fallback"
    @command -v mdbook >/dev/null 2>&1 || echo "missing: mdbook"
    @just --version

# cross-reference and count audit of the plans/docs corpus
audit:
    ./docs/scripts/check-xrefs.sh

# ---- workspace-wide ---------------------------------------------------------

# everything CI runs
ci: fmt-check lint test test-doc doc-check deny

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo nextest run --workspace --all-features

# property tests only — slower, run before merging engine changes.
# (fixed 2026-08-03, QF slice / QR-2 W-1: `--features proptest` named a
# cargo feature that has never existed — proptest is a dependency; and the
# `prop_` filter missed every SDK property family. Naming policy going
# forward: NEW property tests take the `prop_` prefix workspace-wide.)
test-prop:
    cargo nextest run --workspace -E 'test(prop_) | test(optimality_) | test(monotone_)'

# no bench targets exist yet — this door fails loudly instead of silently
# succeeding over zero benchmarks (QR-2 W-1; the harness arrives with RR&P-2)
bench:
    @echo "ERROR: no bench targets exist yet — the micro-bench harness is RR&P-2" >&2
    @echo "       (plans/quality-review/02-additions-and-order.md)" >&2
    @exit 1

# rustdoc examples are tests; nextest does not run them (QR-2 W-3)
test-doc:
    cargo test --doc --workspace --all-features

doc-check:
    cargo doc --workspace --no-deps --all-features

# supply-chain + license gate (QR-2 W-5: cargo-deny subsumes cargo-audit's
# advisories check; allow-list in deny.toml is provisional pending owner)
deny:
    cargo deny check

# ---- evidence ---------------------------------------------------------------

# reproduce RESEARCH-0002/0003/0004  (~8 min)
evidence:
    ./evidence/run_all.sh

# ---- datastore matrix -------------------------------------------------------

# feature matrix (reshaped 2026-08-03, QF slice / QR-2 W-6, F-11): compile
# every feature in isolation — including the no-features leg, which is the
# documented-but-never-exercised bare muster-ui configuration — then run
# the suite on the one real backend. The repo-sqlite leg this recipe used
# to attempt has no feature to run: backend legs arrive with Phase 7.
matrix:
    cargo hack check --workspace --each-feature
    cargo nextest run -p orrery --no-default-features --features repo-memory
    @echo "repo-sqlite / graph backend legs arrive with Phase 7 (ADR-0021 down-select)"

# differential test: all backends must agree (orrery/SPEC-05). BLOCKED by
# design until Phase 7 delivers the second repository implementation
# (ADR-0021 hard gate for Orrery Beta) — this door fails loudly instead of
# erroring on a feature that never existed (QR-2 W-1/F-1).
differential:
    @echo "BLOCKED until Phase 7: differential testing needs the second repository" >&2
    @echo "implementation (ADR-0021; quality-review 02-additions-and-order.md O-7)" >&2
    @exit 1

# ---- fallback if `mod` is unavailable ---------------------------------------

orrery-test:
    cd crates/orrery && just test
sdk-test:
    cd crates/muster-sdk && just test
app-test:
    cd crates/muster && just test
docs-build:
    cd docs && just build
