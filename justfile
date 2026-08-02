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
# Install:  cargo install cargo-nextest cargo-public-api mdbook

default:
    @just --list --list-submodules

# verify the toolchain this justfile assumes
doctor:
    @cargo nextest --version    >/dev/null 2>&1 || echo "missing: cargo-nextest"
    @cargo public-api --version >/dev/null 2>&1 || echo "missing: cargo-public-api"
    @command -v rustup >/dev/null 2>&1 || echo "note: no rustup (Homebrew rust) - cargo-public-api cannot build rustdoc JSON; check-seam uses its grep fallback"
    @command -v mdbook >/dev/null 2>&1 || echo "missing: mdbook"
    @just --version

# cross-reference and count audit of the plans/docs corpus
audit:
    ./docs/scripts/check-xrefs.sh

# ---- workspace-wide ---------------------------------------------------------

# everything CI runs
ci: fmt-check lint test doc-check

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo nextest run --workspace --all-features

# property tests only — slower, run before merging engine changes
test-prop:
    cargo nextest run --workspace --features proptest -E 'test(prop_)'

bench:
    cargo bench --workspace

doc-check:
    cargo doc --workspace --no-deps --all-features

# ---- evidence ---------------------------------------------------------------

# reproduce RESEARCH-0002/0003/0004  (~8 min)
evidence:
    ./evidence/run_all.sh

# ---- datastore matrix -------------------------------------------------------

# run the full suite against every repository backend
matrix:
    cargo nextest run -p orrery --no-default-features --features repo-memory
    cargo nextest run -p orrery --no-default-features --features repo-sqlite
    @echo "add the graph backend here once ADR-0015 closes"

# differential test: all backends must agree (SPEC orrery/05)
differential:
    cargo nextest run -p orrery --features differential -E 'test(diff_)'

# ---- fallback if `mod` is unavailable ---------------------------------------

orrery-test:
    cd crates/orrery && just test
sdk-test:
    cd crates/muster-sdk && just test
app-test:
    cd crates/muster && just test
docs-build:
    cd docs && just build
