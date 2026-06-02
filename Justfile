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

# Pre-commit gate — type-check + full test suite. Non-zero exit signals
# failure to the commit-msg hook so 🔴 vs 🟢 phase validation works.
#
# Deliberately omitted today (will tighten incrementally):
#   - `cargo fmt --check`: the codebase is not yet fully rustfmt'd. Use
#     `just format` to format; we'll gate-enforce once formatting is a
#     separate commit.
#   - `cargo clippy -- -D warnings`: the codebase carries lints today
#     (32+ as of 2026-06-01). Use `just lint` to see them; gate when zero.
#   - `mirror craft --strict boot`: dogfood gate. Currently the boot tree
#     has Dark regions (in-progress migration). Use `just compile-strict`
#     to surface them; gate when boot/ is total.
pre-commit:
    cargo check --manifest-path {{MANIFEST_PATH}} --all-targets
    cargo test --manifest-path {{MANIFEST_PATH}}

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
