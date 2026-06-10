# mirror Justfile
#
# Mirror-native: uses the `mirror` binary in recipes wherever it has a real
# CLI surface today (compile, craft, kintsugi). Falls back to cargo for the
# rest. As `mirror`'s CLI grows (mirror new, mirror refract, mirror serve),
# more recipes will migrate from cargo invocations to mirror invocations.
#
# Why a Justfile lives here: the global commit-msg + pre-push hooks at
# /nix/store/.../git-hooks/ probe for `just format`, `just pre-commit`, and
# `just pre-push` recipes. Without them, the hooks silently skip validation.
# These recipes close that gap.
#
# No `nix develop -c cargo` — the flake's direnv keeps the shell warm.
#
# direnv expectation: the repo's .envrc runs `use flake`, which populates
# CARGO_TARGET_DIR, BLAS_DIR, FLANG, FC_FOR_TARGET, and the rest of the
# Nix devShell env that the build needs (flang FFI linking in particular).
# Recipes assume that env is inherited from the shell. If you've just
# cloned the repo, run `just direnv-allow` first; otherwise `direnv allow`
# auto-fires when you `cd` in.

# Load .env files if present. .envrc → direnv → shell env is the primary
# path for this repo; .env is an additional knob for per-developer overrides
# (e.g. INSTALL_DIR=/usr/local/bin) without touching the Justfile.
set dotenv-load := true

# Cleaner positional-arg forwarding in *ARGS recipes.
set positional-arguments

# The workspace has no root Cargo.toml; bootstrap/ is the package.
# Pass --manifest-path so recipes work from the repo root without `cd`.
MANIFEST_PATH := "bootstrap/Cargo.toml"

# The cargo target dir. The flake's devShell sets CARGO_TARGET_DIR
# (typically /Users/reed/.cargo-target), so we honour that; fall back to
# the in-tree bootstrap/target/ when not in the flake shell.
CARGO_TARGET := env_var_or_default("CARGO_TARGET_DIR", justfile_directory() + "/bootstrap/target")

# The [[bin]] inherits the package name `mirror` (per bootstrap/Cargo.toml line 2).
MIRROR_BIN_DEBUG   := CARGO_TARGET + "/debug/mirror"
MIRROR_BIN_RELEASE := CARGO_TARGET + "/release/mirror"

# Install destination for `just install`. Override on the CLI:
#   just install INSTALL_DIR=/usr/local/bin
INSTALL_DIR := env_var_or_default("INSTALL_DIR", env_var("HOME") + "/.local/bin")

# Default: list available recipes.
default:
    @just --list

# ──────────────────────────────────────────────────────────────────────────
# Hook-required recipes — these MUST exist for the global git-hooks to work.
# ──────────────────────────────────────────────────────────────────────────

# Format Rust sources via rustfmt. Idempotent; safe to run on every commit.
# The commit-msg hook's auto_format calls this and re-stages the changes.
format:
    cargo fmt --manifest-path {{MANIFEST_PATH}} --all

# Pre-commit gate — substrate-native settlement of the pre-commit
# chain via `mirror kintsugi mirror.spec`.
#
# Per insight #43 (2026-06-09 substrate-pull, recognition that mirror IS
# already a content-addressed declarative build system; the 5-minute
# pre-commit chain was the substrate-not-consumed problem). The
# pre-commit altitudes (fmt / check / lint / test / audit) are
# declared as `target` blocks in mirror.spec; mirror.spec walks the
# manifold; per-target @io/cargo actions dispatch through
# shards/io/cargo.mirror's contract.
#
# First tick (this commit): every check altitude falls back to spawning
# cargo at the @io boundary. The substrate is consumed for the DISPATCH;
# subsequent ticks replace per-altitude execution with substrate-native
# settlement (content-addressed-skip; eigensheaf parallelism;
# transparency<p> aggregation) as recognitions #44+ land. The
# performance improvement is gradual and altitude-by-altitude; the
# architecture is in place from tick one.
#
# Verdict gating: the binary exits 0 when the envelope emits cleanly
# (per T11.2.5; the workflow decides what counts as pass). The
# substrate-pull-correct gate IS jq-at-the-@io-boundary, matching the
# kintsugi-ci-v0.1 §5.3 `kintsugi-ci-local` recipe shape. mirror-text
# is captured for the operator; JSON is the format jq reads to drive
# the exit.
#
# Old (pre-#43) chain — preserved as comments for the migration log:
#   cargo check --manifest-path {{MANIFEST_PATH}} --all-targets
#   cargo test  --manifest-path {{MANIFEST_PATH}}
#
# Diff-closure gate (recognition #53, second instance — promoted 2026-06-10):
# the `mirror kintsugi mirror.spec` chain dispatches cargo-altitude checks
# whose dependency closure is the Rust crate (bootstrap/). When the staged
# diff intersects ZERO of {*.rs, Cargo.toml, Cargo.lock}, the closure is
# empty and the chain over-runs (false-positive friction Reed surfaced
# across this session — `--no-verify` workaround on pure-.mirror /
# pure-markdown commits). This block IS the @io byte-level realisation of
# the typed proposal in:
#   shards/epistemologic/property/gate_matches_diff_closure.mirror
#   shards/kintsugi/fracture/gate.mirror
# The substrate names the rewrite; this recipe applies it.
pre-commit:
    #!/usr/bin/env bash
    set -euo pipefail
    # Diff-closure intersection: the @io closure of `mirror kintsugi mirror.spec`
    # at this tick is the Rust crate. Compute the staged-diff intersection;
    # short-circuit when empty.
    rust_closure=$(git diff --cached --name-only --diff-filter=AM \
        | grep -E '(\.rs$|^Cargo\.toml$|^Cargo\.lock$|/Cargo\.toml$|/Cargo\.lock$)' \
        || true)
    if [ -z "$rust_closure" ]; then
        echo "✓ pre-commit: diff-closure empty (no .rs, Cargo.toml, Cargo.lock staged)"
        echo "  skipping mirror kintsugi mirror.spec — gate matches diff closure"
        echo "  (recognition #53 second instance: @kintsugi/fracture/gate applied at @io)"
        exit 0
    fi
    # Rust closure non-empty: run the substrate-native settlement chain unchanged.
    just build
    {{MIRROR_BIN_RELEASE}} kintsugi mirror.spec | tee /tmp/mirror-spec-verdict.mirror
    {{MIRROR_BIN_RELEASE}} kintsugi --format=json mirror.spec > /tmp/mirror-spec-verdict.json
    jq -e '.verdict != "failure"' /tmp/mirror-spec-verdict.json > /dev/null || ( \
        echo "✖ mirror kintsugi mirror.spec: verdict failure — see /tmp/mirror-spec-verdict.mirror" >&2; \
        exit 1 )

# Pre-push gate. Re-runs the suite (coverage tooling not wired yet — when it
# is, this recipe migrates to cargo-llvm-cov + a threshold check).
pre-push:
    cargo test --manifest-path {{MANIFEST_PATH}}
    @echo "(coverage enforcement: TODO — cargo-llvm-cov + threshold)"

# ──────────────────────────────────────────────────────────────────────────
# Build the mirror binary
# ──────────────────────────────────────────────────────────────────────────

# Release build. Produces {{MIRROR_BIN_RELEASE}}.
build:
    cargo build --release --manifest-path {{MANIFEST_PATH}}

# Debug build — faster iteration, produces {{MIRROR_BIN_DEBUG}}.
build-debug:
    cargo build --manifest-path {{MANIFEST_PATH}}

# Install the release binary to {{INSTALL_DIR}}/mirror.
# Override with `just install INSTALL_DIR=/usr/local/bin`.
install: build
    @mkdir -p {{INSTALL_DIR}}
    install -m 0755 {{MIRROR_BIN_RELEASE}} {{INSTALL_DIR}}/mirror
    @echo "installed: {{INSTALL_DIR}}/mirror"
    @echo "ensure PATH contains {{INSTALL_DIR}}"

# Merge the current branch into main.
#
# - Refuses if on main, or if working tree is dirty.
# - Fast-forwards if possible; falls back to --no-ff merge commit.
# - Runs the test suite after the merge.
# - Rebuilds + installs the mirror binary.
# - Push stays explicit — run `git push origin main` when ready.
merge:
    #!/usr/bin/env bash
    set -euo pipefail
    branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$branch" = "main" ]; then
        echo "✖ error: already on main" >&2
        exit 1
    fi
    dirty=$(git status --porcelain --ignore-submodules=all | grep -v '^?? ' || true)
    if [ -n "$dirty" ]; then
        echo "✖ error: working tree dirty. Commit or stash first." >&2
        git status --short >&2
        exit 1
    fi
    echo "→ merging $branch into main"
    git checkout main
    git pull --ff-only origin main
    if ! git merge --ff-only "$branch" 2>/dev/null; then
        echo "→ ff-only failed; creating merge commit"
        git merge --no-ff --no-gpg-sign "$branch" -m "🔀 merge $branch into main"
    fi
    echo "→ running pre-commit gate"
    just pre-commit
    echo "→ rebuilding and installing mirror"
    just install
    echo "✔ merged $branch into main; mirror reinstalled at {{INSTALL_DIR}}/mirror"
    echo "  next: \`git push origin main\` when ready"

# ──────────────────────────────────────────────────────────────────────────
# Mirror-native recipes — build, then USE the binary
# ──────────────────────────────────────────────────────────────────────────

# Run the mirror binary with arbitrary args.
# Example:  just run kintsugi --shatter 1 boot/std/prism.mirror
run *ARGS: build
    {{MIRROR_BIN_RELEASE}} {{ARGS}}

# Shortcut for the kintsugi formatter loop (most-used subcommand).
# Example:  just kintsugi --shatter 1 boot/std/prism.mirror
kintsugi *ARGS: build
    {{MIRROR_BIN_RELEASE}} kintsugi {{ARGS}}

# Run the kintsugi-ci action's logic locally against a target.
# Same shell commands the action runs (actions/kintsugi/bin/run-kintsugi.sh);
# same verdict shape. Substrate-native artifact + JSON for the gate check.
#
# Per kintsugi-ci-v0.1 §5.3 — local/CI parity bar.
#
# Example:  just kintsugi-ci-local fixtures/kintsugi-pass
#           just kintsugi-ci-local ./src 4 0.8 partial
kintsugi-ci-local target shatter='4' threshold='0.8' fail_on='failure': build
    {{MIRROR_BIN_RELEASE}} kintsugi --ci \
        --shatter {{shatter}} \
        {{target}} \
        | tee /tmp/kintsugi-verdict.mirror
    {{MIRROR_BIN_RELEASE}} kintsugi --ci --format=json \
        --shatter {{shatter}} \
        {{target}} \
        > /tmp/kintsugi-verdict.json
    @jq -e --arg fail_on {{fail_on}} \
        'if .verdict == "failure" then false \
         elif .verdict == "partial" and $fail_on == "partial" then false \
         else true end' /tmp/kintsugi-verdict.json

# Dogfood mirror's own compiler against the boot/ grammar tree.
# Equivalent to `mirror craft boot` — emits the crystal OID for the floor.
compile: build
    {{MIRROR_BIN_RELEASE}} craft boot

# Strict compile — fail on any Dark (unclassified) AST nodes.
# This is the substrate's own type-check; useful as a CI gate.
compile-strict: build
    {{MIRROR_BIN_RELEASE}} craft --strict boot

# Build mirror-self: the self-hosted binary via `craft --target binary`.
# Produces ./mirror-self at the repo root.
craft-binary: build
    {{MIRROR_BIN_RELEASE}} craft --target binary boot

# ──────────────────────────────────────────────────────────────────────────
# Cargo conveniences
# ──────────────────────────────────────────────────────────────────────────

# Run the test suite.
test:
    cargo test --manifest-path {{MANIFEST_PATH}}

# Watch + retest on every change (requires cargo-watch).
# Install with: cargo install cargo-watch
test-watch:
    @if ! command -v cargo-watch >/dev/null 2>&1; then \
        echo "cargo-watch not installed — run: cargo install cargo-watch" >&2; \
        exit 1; \
    fi
    cargo watch --manifest-path {{MANIFEST_PATH}} -x test

# Fast type-check (no codegen).
check:
    cargo check --manifest-path {{MANIFEST_PATH}} --all-targets

# Clippy with warnings denied.
lint:
    cargo clippy --manifest-path {{MANIFEST_PATH}} --all-targets -- -D warnings

# Build and open rustdoc.
doc:
    cargo doc --manifest-path {{MANIFEST_PATH}} --open

# ──────────────────────────────────────────────────────────────────────────
# Maintenance
# ──────────────────────────────────────────────────────────────────────────

# Clean cargo build artifacts. Does NOT touch .spectral/ session state
# (that's user understanding state, not build output) or mirror-self.
clean:
    cargo clean --manifest-path {{MANIFEST_PATH}}
    rm -f bootstrap/mirror.ll mirror-self

# ──────────────────────────────────────────────────────────────────────────
# direnv conveniences
# ──────────────────────────────────────────────────────────────────────────

# Approve the repo's .envrc — one-shot for fresh clones.
# After this, direnv auto-loads the flake env on every `cd` in.
direnv-allow:
    @if ! command -v direnv >/dev/null 2>&1; then \
        echo "direnv not installed — see https://direnv.net/docs/installation.html" >&2; \
        exit 1; \
    fi
    direnv allow {{justfile_directory()}}

# Check the flake env is loaded (warn-only — does not fail).
# Useful as a pre-flight when a recipe is mysteriously failing on linker errors.
direnv-check:
    @if [ -z "${IN_NIX_SHELL:-}" ]; then \
        echo "⚠️  IN_NIX_SHELL unset — flake env not loaded." >&2; \
        echo "   Run 'just direnv-allow' or 'cd' out and back in." >&2; \
    else \
        echo "✓ flake env loaded (IN_NIX_SHELL=$IN_NIX_SHELL)"; \
    fi
