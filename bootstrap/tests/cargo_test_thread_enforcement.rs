//! Phase A RED — cargo test `--test-threads=1` enforcement per /loop 2026-07-04
//! Arc 0 Tick 2.
//!
//! **Heisenbug root cause** (Alex 2026-07-04 observed `cargo test` hang > 1hr):
//! `mirror::kintsugi_main_in` uses TWO process-wide mechanisms:
//! 1. `libc::pipe` + `dup2` fd capture per `bootstrap/src/lib.rs`
//! 2. `set_current_dir` cwd swap for CWD-relative grammar loading
//!    (`boot/std/mirror/grammar.mirror` and siblings)
//!
//! 15 integration tests use `kintsugi_main_in`. Running `cargo test` WITHOUT
//! `--test-threads=1` races these mechanisms → pipe-buffer-full deadlock
//! and/or cwd corruption.
//!
//! `bootstrap/src/lib.rs` documents "integration tests therefore run with
//! `--test-threads=1`" as convention, but `--test-threads=1` is enforced
//! NOWHERE per /loop 2026-07-04 Arc 0 Tick 1 audit:
//! - `.cargo/config.toml` does not exist (repo root or bootstrap/)
//! - `bootstrap/Cargo.toml` [profile.test] has no thread setting (only
//!   `codegen-units = 256` per Taut #286 Win 1)
//! - `Justfile` `test` recipe: `cargo test --manifest-path {{MANIFEST_PATH}}`
//!   (no flag)
//! - `Justfile` `pre-push` recipe: `cargo test --manifest-path {{MANIFEST_PATH}}`
//!   (no flag)
//!
//! Option D per /loop Arc 0: enforce via `.cargo/config.toml` `[env]`
//! section setting `RUST_TEST_THREADS = "1"`. Standard cargo env-var
//! propagation; no source migration required. Long-term thread-safety
//! (Options A/B/C) deferred to Arc 0 Tick 4+ per Alex Phase E.
//!
//! **RED phase**: `.cargo/config.toml` does not exist; RED tests fail on
//! absence. **GREEN phase** (Tick 3): create `.cargo/config.toml` with
//! `[env]` `RUST_TEST_THREADS = "1"`.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

#[test]
fn cargo_config_exists_at_repo_root() {
    let path = repo_root().join(".cargo/config.toml");
    assert!(
        path.exists(),
        ".cargo/config.toml must exist at repo root to enforce \
         RUST_TEST_THREADS=1. Currently absent per /loop 2026-07-04 Arc 0 \
         Tick 1 audit; see this file's docblock for heisenbug root cause."
    );
}

#[test]
fn cargo_config_enforces_single_test_thread() {
    let path = repo_root().join(".cargo/config.toml");
    let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read .cargo/config.toml at {:?}: {}. Enforcement is missing \
             — see documented root cause in this file's docblock.",
            path, e,
        )
    });
    let has_env_section = content.contains("[env]");
    let has_rust_test_threads = content.contains("RUST_TEST_THREADS");
    let has_value_one = content.contains("\"1\"") || content.contains("'1'");
    assert!(
        has_env_section && has_rust_test_threads && has_value_one,
        ".cargo/config.toml must set RUST_TEST_THREADS = \"1\" (via [env] \
         section) to enforce --test-threads=1 without requiring user to \
         remember the flag. Prevents heisenbug hang from process-wide \
         state races in kintsugi_main_in (libc::pipe + dup2 fd capture \
         + set_current_dir cwd swap)."
    );
}
