//! Phase A RED — thread-safety Option A source migration per /loop 2026-07-05
//! Arc 2 (Alex direction: full source migration; no half-measures per
//! [[feedback-no-shortcuts-in-compilation-work]]).
//!
//! **Substrate-pull rationale** (Taut `a6efbe5a2e0af97ab` + `ae063d68`):
//! - fd capture is ALREADY thread-safe (thread-local `CAPTURE_STDOUT`/
//!   `CAPTURE_STDERR` cells at `bootstrap/src/lib.rs:89-92`)
//! - Only `set_current_dir` (cwd) remains process-wide
//! - `kintsugi_main_lock` (`~line 3737`) serializes cwd swap
//! - Docstring at lines 1-25 is STALE (describes replaced dup2 approach)
//!
//! Option A per /loop: thread cwd through the dispatch chain via `Ctx`
//! struct; drop process-wide `set_current_dir`; drop `kintsugi_main_lock`;
//! every `Command::new` gets explicit `.current_dir(cwd)`; every file
//! read that resolves relative paths uses `cwd.join(path)`.
//!
//! **NO shortcuts per Alex 2026-07-05**: the full refactor threads cwd
//! through ~70+ call sites in `bootstrap/src/lib.rs`, ~10 in `grammar.rs`,
//! ~8 in `mcp.rs`. Every one lands.
//!
//! **RED phase**: current `dispatch` takes only `args`, `kintsugi_main_lock`
//! exists, `set_current_dir` is called process-wide. Text-check tests fail
//! on presence of the old shape.
//!
//! **GREEN phase** (Reed inline atomic commits):
//! 1. Introduce `Ctx { cwd: PathBuf }` type
//! 2. Thread `Ctx` through `dispatch(args, ctx)` and every command function
//! 3. Every `Command::new(_)` gets `.current_dir(ctx.cwd())`
//! 4. Every relative-path file read gets `ctx.cwd().join(path)`
//! 5. Grammar loader takes `&Ctx` parameter
//! 6. Remove `kintsugi_main_lock`; `kintsugi_main_in` no longer sets
//!    process cwd (constructs `Ctx` and threads through)
//! 7. Verify all 15+ integration tests pass WITHOUT `--test-threads=1`
//!
//! Downstream effect: `.cargo/config.toml`'s `RUST_TEST_THREADS = "1"`
//! becomes redundant; can be removed in a follow-up commit once the
//! source migration is fully in place and empirically verified.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn lib_declares_ctx_type() {
    let content = read_source("bootstrap/src/lib.rs");
    assert!(
        content.contains("pub struct Ctx") || content.contains("struct Ctx"),
        "bootstrap/src/lib.rs must declare a `Ctx` type carrying the cwd \
         (and any other thread-local dispatch state) per /loop 2026-07-05 \
         Arc 2 substrate-pull. Threads cwd through the dispatch chain \
         instead of relying on process-wide `set_current_dir`."
    );
}

#[test]
fn dispatch_takes_ctx() {
    let content = read_source("bootstrap/src/lib.rs");
    let has_ctx_dispatch = content.contains("pub fn dispatch(args: &[String], ctx: &Ctx)")
        || content.contains("pub fn dispatch(args: &[String], ctx:")
        || content.contains("pub fn dispatch(ctx: &Ctx, args:")
        || content.contains("pub fn dispatch(args: &[String], ctx: Ctx)");
    assert!(
        has_ctx_dispatch,
        "`pub fn dispatch` must take `&Ctx` (or `Ctx`) parameter to thread \
         cwd + dispatch state explicitly. Currently `pub fn dispatch(args: \
         &[String])` relies on process-wide `std::env::current_dir()`, \
         which forces the `kintsugi_main_lock` mutex serialization."
    );
}

#[test]
fn kintsugi_main_lock_removed() {
    let content = read_source("bootstrap/src/lib.rs");
    // The mutex serialized `set_current_dir`. Once cwd is threaded via Ctx,
    // the mutex is redundant and must be removed (leaving it as dead code
    // is substrate-dishonest per feedback-no-shortcuts-in-compilation-work).
    assert!(
        !content.contains("fn kintsugi_main_lock()") && !content.contains("static LOCK: OnceLock"),
        "`kintsugi_main_lock()` must be removed once cwd is threaded via \
         `Ctx` — no half-measures per [[feedback-no-shortcuts-in-\
         compilation-work]]. Dead-code retention would leave a stale \
         serialization primitive future ticks step on."
    );
}

#[test]
fn kintsugi_main_in_does_not_set_process_cwd() {
    let content = read_source("bootstrap/src/lib.rs");
    // The function `kintsugi_main_in(args, cwd)` should construct a `Ctx`
    // and thread it through, NOT call `set_current_dir`. Full-shape
    // refactor: process cwd is no longer modified.
    let has_set_current_dir_in_kintsugi_main_in = content
        .lines()
        .skip_while(|line| !line.contains("pub fn kintsugi_main_in"))
        .take_while(|line| !line.contains("kintsugi_main_inner") || !line.contains("fn "))
        .any(|line| line.contains("set_current_dir"));
    assert!(
        !has_set_current_dir_in_kintsugi_main_in,
        "`kintsugi_main_in` must NOT call `std::env::set_current_dir` — \
         thread cwd via `Ctx` instead. Process-wide cwd swap is the \
         thread-safety hazard Arc 2 eliminates."
    );
}

#[test]
fn grammar_loader_takes_ctx() {
    let content = read_source("bootstrap/src/grammar.rs");
    // Grammar loader reads files. Under Ctx threading, relative paths
    // resolve against `ctx.cwd()` explicitly — no process-wide dependence.
    // Function signature or docstring must reference Ctx.
    let load_grammar_takes_ctx = content.contains("pub fn load_grammar(ctx:")
        || content.contains("pub fn load_grammar(path: &str, ctx:")
        || content.contains("pub fn load_grammar_in(");
    assert!(
        load_grammar_takes_ctx,
        "`grammar.rs::load_grammar` (or a `load_grammar_in` variant) must \
         take `&Ctx` parameter to resolve relative paths against \
         `ctx.cwd()` — currently `fs::read_to_string(path)` resolves \
         against process cwd."
    );
}

#[test]
fn ctx_carries_cwd() {
    let content = read_source("bootstrap/src/lib.rs");
    // The Ctx struct must carry `cwd` as its load-bearing field.
    let ctx_has_cwd = content.contains("cwd: PathBuf") || content.contains("cwd: std::path::PathBuf");
    assert!(
        ctx_has_cwd,
        "`Ctx` type must carry `cwd: PathBuf` (or `cwd: std::path::PathBuf`) \
         as its load-bearing field. Other fields (env overrides, capture \
         state) may extend this; cwd is the first-class carrier per Arc 2."
    );
}
