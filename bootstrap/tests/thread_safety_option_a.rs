//! Thread-safety Option A source migration — LANDED (GREEN) per /loop
//! 2026-07-05 Arc 2 (`7d1ec39` GREEN + Seam audit `5e7fd6d` fold-forward
//! corrections). Regression pins for the Ctx-threading discipline.
//!
//! **Substrate-pull rationale** (Taut `a6efbe5a2e0af97ab` + `ae063d68`):
//! fd capture is thread-safe via thread-local `CAPTURE_STDOUT` /
//! `CAPTURE_STDERR` cells (`bootstrap/src/lib.rs:89-92`); the only
//! process-wide state that used to remain was cwd, serialized by the
//! (now-retired) `kintsugi_main_lock` mutex.
//!
//! **Landed shape** (GREEN — per Alex 2026-07-05 no-shortcuts direction):
//! 1. `Ctx { cwd: PathBuf }` declared in `bootstrap/src/lib.rs`
//! 2. `dispatch(args, ctx)` and every `cmd_*` function take `&Ctx`
//! 3. Every `Command::new` in the dispatch chain routes through
//!    `ctx.command(_)` so subprocesses inherit `ctx.cwd()`
//! 4. Every relative-path file read resolves via `ctx.resolve(path)`
//! 5. `grammar.rs` exposes `load_grammar_in` / `grammar_for_file_in` /
//!    `grammar_path_for_ref_in` variants taking `&Ctx`
//! 6. `kintsugi_main_lock` mutex removed; `kintsugi_main_in(args, cwd)`
//!    constructs a `Ctx` and threads it — never mutates process cwd
//! 7. Arc 2 fold-forward (`5e7fd6d`) closed the last three LIVE-dispatch
//!    process-cwd leaks: `pipeline::tokenize_with_ref`,
//!    `spectral::render_ast` fold-reducer, `mcp::serve_loop`
//!
//! Downstream effect: `.cargo/config.toml`'s `RUST_TEST_THREADS = "1"`
//! ceiling is no longer required and the fold-forward tick removes it.
//! These text-check tests pin the landed shape as regression coverage.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
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
    let ctx_has_cwd =
        content.contains("cwd: PathBuf") || content.contains("cwd: std::path::PathBuf");
    assert!(
        ctx_has_cwd,
        "`Ctx` type must carry `cwd: PathBuf` (or `cwd: std::path::PathBuf`) \
         as its load-bearing field. Other fields (env overrides, capture \
         state) may extend this; cwd is the first-class carrier per Arc 2."
    );
}
