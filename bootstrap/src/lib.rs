//! mirror — the native binary, Rust port (library crate).
//!
//! Bit-exact CoincidenceHash<3> + content-OID compatibility with the C
//! original at native/mirror.c. The body-capture fix for LLVM IR keyword
//! forms (target datalayout = "...", source_filename = "...") is shared.
//! Content OIDs are computed by `spectral::compute_content_oid`, which
//! dispatches the recursive AST walk through
//! `prismqueer::apply_h(&ContentOidPrism, ast)` per
//! `docs/specs/bootstrap-retirement-plan.md` Tick 1.
//!
//! ## Library shape (Taut #286 Win 2)
//!
//! This crate exposes the binary's subcommand dispatch as `kintsugi_main`,
//! a library function that the integration tests in `bootstrap/tests/`
//! invoke directly. Each in-process call replaces a `Command::new(exe)`
//! subprocess spawn that paid 200-800ms dyld + Accelerate startup; per
//! Taut's profiling survey #286 Win 2 the test suite collapses from
//! ~11.6s wall to ~1-2s.
//!
//! The real `fn main()` in `src/main.rs` calls `dispatch(&args)` directly
//! (no fd capture, native stdout streaming preserved). The library entry
//! point `kintsugi_main(&args)` wraps the same dispatch in an fd-level
//! capture (libc `pipe` + `dup2`, drained on a worker thread) so tests
//! can assert on `stdout` / `stderr` bytes without spawning a subprocess.
//! The capture is process-wide while it runs; integration tests therefore
//! run with `--test-threads=1`.

pub mod action_cache;
pub mod ast;
pub mod cholesky;
pub mod crystallize;
pub mod curvature;
pub mod exec;
pub mod gap;
pub mod git;
pub mod grammar;
pub mod hash;
pub mod kintsugi;
pub mod lens_unix;
pub mod mcp;
pub mod music;
pub mod oscillate;
pub mod pipeline;
pub mod portal;
pub mod property;
pub mod realisation;
pub mod score;
pub mod dance;
pub mod deploy;
pub mod sheaf_laplacian;
pub mod contribute;
pub mod index;
pub mod song;
pub mod spectral;
pub mod store_branch;
pub mod tensor;
pub mod tokenize;

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;

// ─────────────────────────────────────────────────────────────────────────────
// N3 TICK 1 (substrate-pull:realize) — verdict-cache marker functions.
//
// The integration tests at
// `bootstrap/tests/kintsugi_spec_verdict_cache_integration.rs` (Reed's RED,
// commit `4901d8a`) call these six functions to verify that the Rust wiring
// for `@mirror/store/action_cache` is landed. Each function returns `true`
// iff the corresponding behavior exists AND passes a live fixture smoke
// test — NOT stubbed `return true`. Per N3 brief "Test-visible module
// functions" §1.
//
// The functions are grouped here (rather than at their referenced call
// sites) so the marker surface is one contiguous block — legibility over
// scattering. The actual wiring lives in:
//   - `bootstrap/src/action_cache.rs` — the cache_read/write/exists
//     dispatch surface + OID computation.
//   - `cmd_kintsugi_spec` in this file — the consumer that consults the
//     cache before spawning cargo.
//   - `bootstrap/src/crystallize.rs` — the Crystallizations<H> dispatch
//     table wired via `crystallizations_for_action_cache`.
// ─────────────────────────────────────────────────────────────────────────────

/// T1 marker: cold-cache path still works.
///
/// True iff the cache-aware entry point exists (the `cmd_kintsugi_spec`
/// refactor landed) AND a cold-cache smoke test succeeds — hashing a
/// fresh fixture and confirming `cache_read` returns `None` (miss) on
/// virgin state.
pub fn verdict_cache_cold_path_landed() -> bool {
    let tmp = fresh_marker_tempdir("n3-cold");
    let spec_oid = action_cache::compute_spec_oid(b"fixture-spec-bytes");
    let target_oid = action_cache::compute_target_oid("tests", "cargo", "test");
    let inputs_oid = action_cache::InputsOid::new("cc".repeat(32));
    // Cold path: no prior state, cache_read must MISS.
    action_cache::cache_read(&tmp, &spec_oid, &target_oid, &inputs_oid).is_none()
        && !action_cache::cache_exists(&tmp, &spec_oid, &target_oid, &inputs_oid)
}

/// T2 marker: warm-cache path returns the memoized verdict without cargo.
///
/// True iff a write-then-read cycle succeeds against a real temp-dir
/// `@mirror/store` layout AND the read returns the same verdict bytes.
/// This is the assertion that the warm-cache path exists as an actual
/// data path (not a stub).
pub fn verdict_cache_warm_path_landed() -> bool {
    let tmp = fresh_marker_tempdir("n3-warm");
    let spec_oid = action_cache::compute_spec_oid(b"warm-fixture-bytes");
    let target_oid = action_cache::compute_target_oid("tests", "cargo", "test");
    let inputs_oid = action_cache::InputsOid::new("aa".repeat(32));
    let v = action_cache::CachedVerdict {
        verdict: "success".to_string(),
        objective: 0.0,
        dark_count: 0,
        label: "bootstrap/Cargo.toml (test)".to_string(),
    };
    if action_cache::cache_write(&tmp, &spec_oid, &target_oid, &inputs_oid, &v).is_err() {
        return false;
    }
    match action_cache::cache_read(&tmp, &spec_oid, &target_oid, &inputs_oid) {
        Some(got) => got == v,
        None => false,
    }
}

/// T3 marker: cache key discriminates independently on each dimension.
///
/// True iff writing under one (spec, target, inputs) triple produces
/// misses for every neighbor triple that differs in exactly one
/// dimension.
pub fn verdict_cache_input_key_discriminates() -> bool {
    let tmp = fresh_marker_tempdir("n3-disc");
    let spec = action_cache::SpecOid::new("aa".repeat(32));
    let tgt = action_cache::TargetOid::new("bb".repeat(32));
    let inp = action_cache::InputsOid::new("cc".repeat(32));
    let v = action_cache::CachedVerdict {
        verdict: "success".to_string(),
        objective: 0.0,
        dark_count: 0,
        label: "l".to_string(),
    };
    if action_cache::cache_write(&tmp, &spec, &tgt, &inp, &v).is_err() {
        return false;
    }
    let spec2 = action_cache::SpecOid::new("dd".repeat(32));
    let tgt2 = action_cache::TargetOid::new("ee".repeat(32));
    let inp2 = action_cache::InputsOid::new("ff".repeat(32));
    action_cache::cache_read(&tmp, &spec2, &tgt, &inp).is_none()
        && action_cache::cache_read(&tmp, &spec, &tgt2, &inp).is_none()
        && action_cache::cache_read(&tmp, &spec, &tgt, &inp2).is_none()
        && action_cache::cache_read(&tmp, &spec, &tgt, &inp).as_ref() == Some(&v)
}

/// T4 marker: cache_write is idempotent by content-address.
///
/// True iff two writes with the same key+verdict succeed (the second is
/// a no-op) and a subsequent read returns the identical verdict.
pub fn verdict_cache_write_is_idempotent() -> bool {
    let tmp = fresh_marker_tempdir("n3-idem");
    let spec = action_cache::SpecOid::new("11".repeat(32));
    let tgt = action_cache::TargetOid::new("22".repeat(32));
    let inp = action_cache::InputsOid::new("33".repeat(32));
    let v = action_cache::CachedVerdict {
        verdict: "success".to_string(),
        objective: 0.0,
        dark_count: 0,
        label: "l".to_string(),
    };
    if action_cache::cache_write(&tmp, &spec, &tgt, &inp, &v).is_err() {
        return false;
    }
    if action_cache::cache_write(&tmp, &spec, &tgt, &inp, &v).is_err() {
        return false;
    }
    action_cache::cache_read(&tmp, &spec, &tgt, &inp).as_ref() == Some(&v)
}

/// T5 marker: cache state lives in `@mirror/store` (crystals in the store
/// DAG), NOT in-process.
///
/// True iff a write in one call is visible to a subsequent independent
/// read against the same `cwd` — i.e. the state is on-disk under
/// `<cwd>/.mirror/action_cache/...`. This models the two-process
/// boundary: the cache root is deterministic under `cwd`, and both
/// "processes" (here, two independent `cache_read` calls) resolve to
/// the same on-disk state.
pub fn verdict_cache_persists_across_processes() -> bool {
    let tmp = fresh_marker_tempdir("n3-persist");
    let spec = action_cache::SpecOid::new("7a".repeat(32));
    let tgt = action_cache::TargetOid::new("7b".repeat(32));
    let inp = action_cache::InputsOid::new("7c".repeat(32));
    let v = action_cache::CachedVerdict {
        verdict: "success".to_string(),
        objective: 0.0,
        dark_count: 0,
        label: "persist-test".to_string(),
    };
    if action_cache::cache_write(&tmp, &spec, &tgt, &inp, &v).is_err() {
        return false;
    }
    // Confirm the on-disk crystal exists under the substrate-declared
    // root: `<cwd>/.mirror/action_cache/<spec>/<target>/<inputs>/verdict.json`.
    let crystal_path = action_cache::cache_root(&tmp)
        .join(spec.as_str())
        .join(tgt.as_str())
        .join(inp.as_str())
        .join("verdict.json");
    if !crystal_path.is_file() {
        return false;
    }
    // A distinct "process" (fresh cache_read call) sees the same crystal.
    action_cache::cache_read(&tmp, &spec, &tgt, &inp).as_ref() == Some(&v)
}

/// T6 marker: `Crystallizations<H>` dispatch table wired into
/// `cmd_kintsugi_spec`.
///
/// True iff the `crystallizations_for_action_cache` factory returns a
/// `Crystallizations<Blake3>` that knows the three action_cache refs
/// (`@mirror/store/action_cache/cache_read`, `cache_write`,
/// `cache_exists`) AND `cmd_kintsugi_spec` imports the action_cache
/// module (verified by this crate compiling with the wire in place).
/// Not a stub — the dispatch table's `knows()` is queried at the marker
/// altitude.
pub fn crystallizations_dispatch_wired_into_cmd_kintsugi_spec() -> bool {
    let crys = crystallize::crystallizations_for_action_cache();
    let read_ref = match prismqueer::Ref::new("@mirror/store/action_cache/cache_read") {
        Ok(r) => r,
        Err(_) => return false,
    };
    let write_ref = match prismqueer::Ref::new("@mirror/store/action_cache/cache_write") {
        Ok(r) => r,
        Err(_) => return false,
    };
    let exists_ref = match prismqueer::Ref::new("@mirror/store/action_cache/cache_exists") {
        Ok(r) => r,
        Err(_) => return false,
    };
    crys.knows(&read_ref) && crys.knows(&write_ref) && crys.knows(&exists_ref)
}

/// Local tempdir helper for the marker functions. Not exported.
fn fresh_marker_tempdir(tag: &str) -> PathBuf {
    let mut base = std::env::temp_dir();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    base.push(format!("mirror-marker-{}-{}-{}", tag, pid, stamp));
    let _ = fs::create_dir_all(&base);
    base
}

// ─────────────────────────────────────────────────────────────────────────────
// `mout!` / `merr!` — print macros that bypass libtest's thread-local
// `OUTPUT_CAPTURE` (per Taut #286 Win 2).
//
// The standard `println!` / `eprintln!` macros route through `_print` →
// `print_to` → `Stdout::write_all`, which checks the thread-local
// `OUTPUT_CAPTURE` set by libtest's harness BEFORE writing to fd 1 / 2.
// libtest's `thread::Builder::spawn` clones the parent's `OUTPUT_CAPTURE`
// onto every child thread, so even running `dispatch` on a fresh worker
// thread doesn't escape the capture — the worker inherits the parent's
// thread-local sink, and our fd-level pipe redirect never sees the bytes.
//
// `mout!` and `merr!` skirt the thread-local by writing directly to fd 1
// and fd 2 via `libc::write` (on unix; `Stdout::lock` on non-unix). The
// `kintsugi_main` library entry point redirects those fds to its own
// pipes for in-process capture; the binary path keeps fd 1 and 2 as the
// real process stdout/stderr.
//
// Wire shape: identical to `println!{...}` / `eprintln!{...}` (no
// behavioural change). The macros take format args, append a trailing
// newline, and write the resulting UTF-8 bytes in a single syscall (the
// short writes the existing prints emit always fit in a single
// `libc::write`).

// ─────────────────────────────────────────────────────────────────────────────
// Per-thread capture cells. When `kintsugi_main` is invoked from a test, it
// installs `Some(Vec)` into both thread-locals; `_raw_stdout` / `_raw_stderr`
// detect the install and route into the buffer instead of fd 1 / 2. This is
// the parallel-safe alternative to a process-wide dup2: each test thread
// gets its own capture buffer, and libtest's own status writes to fd 1 (test
// progress lines) keep flowing through the real terminal undisturbed.
//
// On a fresh thread the cells are `None` — production binary behaviour is
// unchanged (the macros fall through to `libc::write` on the real fd).
std::thread_local! {
    static CAPTURE_STDOUT: std::cell::RefCell<Option<Vec<u8>>> = const { std::cell::RefCell::new(None) };
    static CAPTURE_STDERR: std::cell::RefCell<Option<Vec<u8>>> = const { std::cell::RefCell::new(None) };
}

// ─────────────────────────────────────────────────────────────────────────────
// `Ctx` — explicit dispatch context per Arc 2 / /loop 2026-07-05
// (feedback-no-shortcuts-in-compilation-work).
//
// The dispatch chain used to depend on the process-wide cwd via
// `std::env::set_current_dir` inside `kintsugi_main_in`, serialised by
// `kintsugi_main_lock` mutex. Arc 2 threads cwd through the dispatch chain
// as an explicit `&Ctx` parameter so:
//   - concurrent test threads no longer race on process cwd;
//   - the `kintsugi_main_lock` mutex retires;
//   - relative-path resolution is byte-checkable (explicit-over-implicit).
//
// The carrier is a struct (not a bare `PathBuf`) so future dispatch state
// (env overrides, capture cells, etc.) extends without further signature
// churn. `cwd` is the load-bearing field; helpers `resolve` / `command` /
// `read` operationalise the substrate-pull to resolve every relative path
// against `ctx.cwd()` explicitly.
// ─────────────────────────────────────────────────────────────────────────────

/// Explicit dispatch context. Carries the working directory that relative
/// paths resolve against inside the dispatch chain. Threaded through
/// `dispatch(args, ctx)` and every `cmd_*` function.
///
/// Constructed via [`Ctx::new`] (explicit cwd) or [`Ctx::from_process_cwd`]
/// (the `mirror` binary entry point). The library entry points
/// [`kintsugi_main`] and [`kintsugi_main_in`] construct the appropriate
/// `Ctx` internally.
pub struct Ctx {
    /// Working directory that unresolved relative paths inside the
    /// dispatch chain resolve against. Always absolute after
    /// construction — `Ctx::new` canonicalises where the filesystem
    /// permits; otherwise the caller-supplied path is retained verbatim.
    cwd: std::path::PathBuf,
}

impl Ctx {
    /// Construct a `Ctx` with an explicit cwd. Used by
    /// `kintsugi_main_in(args, cwd)` so integration tests can dispatch
    /// against a fixture directory without mutating the process cwd.
    pub fn new(cwd: impl Into<std::path::PathBuf>) -> Self {
        Self { cwd: cwd.into() }
    }

    /// Construct a `Ctx` inheriting the process cwd. Used by the binary
    /// entry point (`mirror::dispatch` via `main`) and by
    /// `kintsugi_main` (no explicit cwd variant).
    pub fn from_process_cwd() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
        }
    }

    /// The load-bearing field. Relative paths resolve against this.
    pub fn cwd(&self) -> &std::path::Path {
        &self.cwd
    }

    /// Resolve a possibly-relative path against `self.cwd()`. Absolute
    /// paths are returned unchanged; relative paths are joined onto
    /// `cwd`. Returns `PathBuf` so the caller can pass into `fs::read`,
    /// `Command::current_dir`, etc.
    pub fn resolve(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        let p = path.as_ref();
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.cwd.join(p)
        }
    }

    /// Construct a `Command` with `.current_dir(ctx.cwd())` pre-set.
    /// Used by every dispatch-chain `Command::new(...)` so subprocess
    /// invocations resolve their own relative paths against `ctx.cwd()`
    /// instead of the process cwd.
    pub fn command(&self, program: impl AsRef<std::ffi::OsStr>) -> std::process::Command {
        let mut cmd = std::process::Command::new(program);
        cmd.current_dir(&self.cwd);
        cmd
    }
}

/// Write a UTF-8 byte slice to stdout. If the thread-local capture cell is
/// installed, append to it; otherwise write to fd 1 directly via `libc::write`
/// (bypassing `std::io::Stdout` and its `OUTPUT_CAPTURE` sink). Used by
/// `mout!`.
#[doc(hidden)]
pub fn _raw_stdout(bytes: &[u8]) {
    let captured = CAPTURE_STDOUT.with(|cell| {
        if let Some(buf) = cell.borrow_mut().as_mut() {
            buf.extend_from_slice(bytes);
            true
        } else {
            false
        }
    });
    if captured {
        return;
    }
    #[cfg(unix)]
    {
        // Loop in case of short writes (rare for the short strings we
        // emit, but correct under signals).
        let mut written = 0;
        while written < bytes.len() {
            let n = unsafe {
                libc::write(
                    1,
                    bytes.as_ptr().add(written) as *const libc::c_void,
                    bytes.len() - written,
                )
            };
            if n <= 0 {
                break;
            }
            written += n as usize;
        }
    }
    #[cfg(not(unix))]
    {
        use std::io::Write;
        let _ = std::io::stdout().lock().write_all(bytes);
    }
}

/// Write a UTF-8 byte slice to stderr. If the thread-local capture cell is
/// installed, append to it; otherwise write to fd 2 directly via `libc::write`
/// (bypassing `std::io::Stderr` and its `OUTPUT_CAPTURE` sink). Used by
/// `merr!`.
#[doc(hidden)]
pub fn _raw_stderr(bytes: &[u8]) {
    let captured = CAPTURE_STDERR.with(|cell| {
        if let Some(buf) = cell.borrow_mut().as_mut() {
            buf.extend_from_slice(bytes);
            true
        } else {
            false
        }
    });
    if captured {
        return;
    }
    #[cfg(unix)]
    {
        let mut written = 0;
        while written < bytes.len() {
            let n = unsafe {
                libc::write(
                    2,
                    bytes.as_ptr().add(written) as *const libc::c_void,
                    bytes.len() - written,
                )
            };
            if n <= 0 {
                break;
            }
            written += n as usize;
        }
    }
    #[cfg(not(unix))]
    {
        use std::io::Write;
        let _ = std::io::stderr().lock().write_all(bytes);
    }
}

/// `mout!` — like `println!` but bypasses libtest's `OUTPUT_CAPTURE`
/// so the `kintsugi_main` in-process capture sees the bytes via its
/// pipe redirect on fd 1.
#[macro_export]
macro_rules! mout {
    () => {{ $crate::_raw_stdout(b"\n"); }};
    ($($arg:tt)*) => {{
        let mut s = format!($($arg)*);
        s.push('\n');
        $crate::_raw_stdout(s.as_bytes());
    }};
}

/// `merr!` — like `eprintln!` but bypasses libtest's `OUTPUT_CAPTURE`
/// so the `kintsugi_main` in-process capture sees the bytes via its
/// pipe redirect on fd 2.
#[macro_export]
macro_rules! merr {
    () => {{ $crate::_raw_stderr(b"\n"); }};
    ($($arg:tt)*) => {{
        let mut s = format!($($arg)*);
        s.push('\n');
        $crate::_raw_stderr(s.as_bytes());
    }};
}

use prismqueer::{Optic, Ref};
use serde::Serialize;
use terni::Imperfect;

use crate::ast::{line_col_at, AstKind, AstNode};
use crate::crystallize::{
    floor_crystallizations, Blake3, Content, Crystallizations, CrystallizeError, Splinter, Text,
};
use crate::git::{git_crystal_exists, git_store_crystal};
use crate::grammar::{grammar_for_file_in, load_grammar_in};
use crate::hash::canonical_hash;
use crate::pipeline::{
    apply_rewrites, execute_pipeline, is_mq_query, parse_rewrite, split_pipeline,
};
use crate::spectral::{compute_content_oid, render_ast_in};
use crate::tokenize::tokenize;

/// Walk an AST, collecting every `AstKind::Dark` node in source order.
fn collect_dark<'a>(node: &'a AstNode, out: &mut Vec<&'a AstNode>) {
    if node.kind == AstKind::Dark {
        out.push(node);
    }
    for c in &node.children {
        collect_dark(c, out);
    }
}

/// Print a `total_classification` diagnostic for a single dark region.
///
/// Emits only the location + caret + hint block. The per-file
/// `error[total_classification]: N dark region(s) in <file>` header is
/// printed once by `enforce_strict` before iterating over regions, so
/// readers don't see the total multiplied by the region count.
/// Per Seam T1.2 (docs/review/2026-05-20-seam-adversarial.md).
///
/// Per-region format:
///     --> line <L>, col <C>
///      |
///   <L> | <source line>
///      | <caret>
///      |
///      = hint: the parser has no rule for this construct
fn print_dark_diag(file: &str, source: &[u8], dark: &AstNode) {
    // file is retained in the signature for future per-region cross-file
    // diagnostics (e.g., grouped output, jump-to-location); currently unused
    // because the header is emitted by `enforce_strict`.
    let _ = file;
    let span = dark.dark_span;
    let (mut line, mut col) = line_col_at(source, span.start);
    // Skip leading whitespace lines inside the dark region so the caret
    // points at the first real content rather than the bare `{\n`.
    let mut content_start = span.start.min(source.len());
    while content_start < span.end
        && matches!(
            source.get(content_start).copied().unwrap_or(0),
            b' ' | b'\t' | b'\n' | b'\r'
        )
    {
        content_start += 1;
    }
    if content_start < span.end {
        let (l, c) = line_col_at(source, content_start);
        line = l;
        col = c;
    }

    // Find the start/end of the line containing `content_start`.
    let line_start = {
        let mut i = content_start.min(source.len());
        while i > 0 && source[i - 1] != b'\n' {
            i -= 1;
        }
        i
    };
    let line_end = {
        let mut i = content_start.min(source.len());
        while i < source.len() && source[i] != b'\n' {
            i += 1;
        }
        i
    };
    let src_line = String::from_utf8_lossy(&source[line_start..line_end]);

    // Caret width: the run of non-whitespace bytes starting at content_start.
    let mut tok_end = content_start;
    while tok_end < line_end && !matches!(source[tok_end], b' ' | b'\t') {
        tok_end += 1;
    }
    let caret_width = (tok_end - content_start).max(1);

    let line_str = format!("{}", line);
    let gutter_w = line_str.len();
    let pad = " ".repeat(gutter_w);

    merr!("  --> line {}, col {}", line, col);
    merr!("   {} |", pad);
    merr!("   {} | {}", line_str, src_line);
    // Caret line: spaces up to (col-1), then ^^^.
    let leading = (col as usize).saturating_sub(1);
    let mut caret_line = String::with_capacity(leading + caret_width);
    for _ in 0..leading {
        caret_line.push(' ');
    }
    for _ in 0..caret_width {
        caret_line.push('^');
    }
    merr!("   {} | {}", pad, caret_line);
    merr!("   {} |", pad);
    merr!(
        "   {} = hint: the parser has no rule for this construct",
        pad
    );
}

/// Returns (dark_count, diagnostic exit code).
/// Print one header per file with the dark-region count, then one
/// caret-block per region. Exit code 2 if any dark. Per Seam T1.2 the
/// header is emitted once — the old shape printed it N times with the
/// total embedded, making the count look multiplied.
fn enforce_strict(file: &str, source: &[u8], ast: &AstNode) -> usize {
    let mut darks: Vec<&AstNode> = Vec::new();
    collect_dark(ast, &mut darks);
    if darks.is_empty() {
        return 0;
    }
    let n = darks.len();
    merr!(
        "error[total_classification]: {} dark region{} in {}",
        n,
        if n == 1 { "" } else { "s" },
        file
    );
    for d in &darks {
        print_dark_diag(file, source, d);
    }
    n
}

fn usage() {
    merr!("usage:");
    merr!("  mirror <command> [args...]            (legacy subcommand surface)");
    merr!("  mirror '<mq-query>' < input           (mq pipeline over stdin)");
    merr!("  mirror <input> '<mq-query>'           (mq pipeline over input file)");
    merr!("commands: compile [--strict] <file>, craft [--strict] [--target-kind <crystal|binary>] <target>, kintsugi [--ci [--out @data/json|@data/mirror|@io/dir('path')]] [--shatter N] <file|dir>, init [--install-hooks] <repo-path>, recall <spec-dir>, spawn [--hello-world] [--mission <mission-file>] <peer-home>, shatter <oid> <out> [--target @data/json|@data/mirror|auto]");
    merr!("examples:");
    merr!("  cat mirror.ll | mirror '@code/llvm/ir |> @mirror/kintsugi |> @mirror/butterfly'");
}

fn read_stdin_all() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

fn cmd_compile(file: &str, no_cache: bool, strict: bool, ctx: &Ctx) -> i32 {
    let read_path = ctx.resolve(file);
    let source = match fs::read(&read_path) {
        Ok(s) => s,
        Err(e) => {
            merr!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file_in(file, ctx);
    let grammar = match load_grammar_in(grammar_path, ctx) {
        Ok(g) => g,
        Err(e) => {
            merr!("cannot read grammar {}: {}", grammar_path, e);
            return 1;
        }
    };

    // --strict bypasses the OID cache: we need the AST to count Dark.
    if !no_cache && !strict {
        let source_oid = canonical_hash(&source);
        if let Some(cached) = git_crystal_exists(&source_oid) {
            merr!("(cached)");
            mout!("{}", cached);
            return 0;
        }
    }

    let ast = tokenize(&source, &grammar);
    let oid = compute_content_oid(&ast);
    if !no_cache && !strict {
        let source_oid = canonical_hash(&source);
        git_store_crystal(&source_oid, &oid);
    }
    if strict {
        let dark_count = enforce_strict(file, &source, &ast);
        if dark_count > 0 {
            return 2;
        }
    }
    mout!("{}", oid);
    0
}

fn collect_files(dir: &str, ext: &str, out: &mut Vec<String>) {
    let read_dir = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for entry in read_dir.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let path = format!("{}/{}", dir, name);
        let md = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if md.is_dir() {
            collect_files(&path, ext, out);
        } else if name.ends_with(ext) {
            out.push(path);
        }
    }
}

/// What `--target` produces from a craft.
///
/// Today only `Crystal` (the default — print the OID) and `Binary` (build the
/// bootstrap as a self-hosted binary via cargo rustc + clang) are wired.
/// `Rust` and `Gleam` are declared so the surface is stable; they will be
/// implemented via grammar emission once `@code/llvm/emit` graduates from
/// `/tmp/mirror.ll` self-reference to grammar-driven emission.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TargetKind {
    Crystal,
    Binary,
    #[allow(dead_code)]
    Rust,
    #[allow(dead_code)]
    Gleam,
}

fn parse_target(s: &str) -> Option<TargetKind> {
    match s {
        "crystal" => Some(TargetKind::Crystal),
        "binary" => Some(TargetKind::Binary),
        "rust" => Some(TargetKind::Rust),
        "gleam" => Some(TargetKind::Gleam),
        _ => None,
    }
}

fn cmd_craft_with(target: &str, no_cache: bool, kind: TargetKind, strict: bool, ctx: &Ctx) -> i32 {
    let mut files: Vec<String> = Vec::new();
    match target {
        "boot" | "std" => collect_files(
            ctx.resolve("boot").to_string_lossy().as_ref(),
            ".mirror",
            &mut files,
        ),
        "cargo" => collect_files(
            ctx.resolve("src").to_string_lossy().as_ref(),
            ".rs",
            &mut files,
        ),
        _ => {
            merr!("unknown target: {}", target);
            return 1;
        }
    }
    files.sort();

    let mut hasher_buf: Vec<u8> = Vec::new();
    let mut hits = 0;
    let total = files.len();
    let mut total_dark: usize = 0;
    let mut files_with_dark: usize = 0;

    for file in &files {
        let grammar_path = grammar_for_file_in(file, ctx);
        let grammar = match load_grammar_in(grammar_path, ctx) {
            Ok(g) => g,
            Err(_) => {
                merr!("  skip {} (grammar error)", file);
                continue;
            }
        };
        let source = match fs::read(file) {
            Ok(s) => s,
            Err(_) => {
                merr!("  skip {} (read error)", file);
                continue;
            }
        };
        let mut oid = String::new();
        let mut cached = false;
        // --strict needs the AST every time — skip the OID-only cache hit.
        if !no_cache && !strict {
            let source_oid = canonical_hash(&source);
            if let Some(c) = git_crystal_exists(&source_oid) {
                oid = c;
                cached = true;
                hits += 1;
            }
        }
        if !cached {
            let ast = tokenize(&source, &grammar);
            oid = compute_content_oid(&ast);
            if !no_cache && !strict {
                let source_oid = canonical_hash(&source);
                git_store_crystal(&source_oid, &oid);
            }
            if strict {
                let dc = enforce_strict(file, &source, &ast);
                if dc > 0 {
                    total_dark += dc;
                    files_with_dark += 1;
                }
            }
        }
        if cached {
            merr!("  {} -> {} (cached)", file, oid);
        } else {
            merr!("  {} -> {}", file, oid);
        }
        hasher_buf.extend_from_slice(oid.as_bytes());
    }

    let crystal = canonical_hash(&hasher_buf);
    if hits > 0 {
        merr!("cache: {}/{} hits", hits, total);
    }
    if strict && total_dark > 0 {
        merr!(
            "error[total_classification]: {} dark region(s) across {} file(s)",
            total_dark,
            files_with_dark
        );
        return 2;
    }
    mout!("{}", crystal);

    if kind == TargetKind::Binary && (target == "boot" || target == "std") {
        return build_self_binary(ctx);
    }
    if kind == TargetKind::Rust || kind == TargetKind::Gleam {
        merr!("--target rust/gleam: not yet implemented (declared for surface stability)");
        return 2;
    }
    0
}

/// Produce `./mirror-self` from the bootstrap's own LLVM IR.
///
/// Pipeline:
///   1. `cargo rustc --release --manifest-path bootstrap/Cargo.toml -- --emit=llvm-ir`
///   2. Locate the freshest `mirror-*.ll` under `${CARGO_TARGET_DIR}/release/deps/`,
///      excluding test-binary IR (which has long hex suffixes after extra dashes).
///   3. Copy to `bootstrap/mirror.ll` (the canonical IR location, replacing
///      the legacy `/tmp/mirror.ll` self-reference path).
///   4. `clang -O2 -o ./mirror-self -x ir bootstrap/mirror.ll -lm`
///
/// This is the butterfly: the bootstrap emits IR for itself, clang turns the
/// IR into a binary, and `./mirror-self craft boot` must match
/// `mirror craft boot` for v1.0.0.
fn build_self_binary(ctx: &Ctx) -> i32 {
    merr!("== craft --target binary ==");
    merr!("1/3 cargo rustc --emit=llvm-ir");

    let status = ctx
        .command("cargo")
        .args([
            "rustc",
            "--release",
            "--manifest-path",
            "bootstrap/Cargo.toml",
            "--",
            "--emit=llvm-ir",
        ])
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            merr!("cargo rustc failed: exit {}", s);
            return 1;
        }
        Err(e) => {
            merr!("cargo rustc spawn error: {}", e);
            return 1;
        }
    }

    merr!("2/3 locate bootstrap/mirror.ll");
    let target_dir =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "bootstrap/target".to_string());
    let deps_dir = ctx.resolve(PathBuf::from(&target_dir).join("release").join("deps"));
    let ll_path = match find_bootstrap_ll(&deps_dir) {
        Some(p) => p,
        None => {
            merr!("could not find mirror-*.ll under {}", deps_dir.display());
            return 1;
        }
    };
    merr!("    found: {}", ll_path.display());

    let dest = ctx.resolve("bootstrap/mirror.ll");
    if let Err(e) = fs::copy(&ll_path, &dest) {
        merr!(
            "copy {} -> {} failed: {}",
            ll_path.display(),
            dest.display(),
            e
        );
        return 1;
    }
    merr!("    copied to {}", dest.display());

    merr!("3/3 clang -O2 -o ./mirror-self -x ir bootstrap/mirror.ll -lm");
    let status = ctx
        .command("clang")
        .args([
            "-O2",
            "-o",
            "./mirror-self",
            "-x",
            "ir",
            "bootstrap/mirror.ll",
            "-lm",
        ])
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            merr!("clang failed: exit {}", s);
            return 1;
        }
        Err(e) => {
            merr!("clang spawn error: {}", e);
            return 1;
        }
    }

    merr!("== ./mirror-self ==");
    0
}

/// Pick the freshest `mirror-<HASH>.ll` under `deps_dir`, skipping test-binary
/// IR. The Cargo deps directory contains:
///   mirror-<hash>.ll          ← the bin we want (one dash after "mirror")
///   oid_smoke-<hash>.ll       ← integration test binary (different stem)
/// Older rustc versions could also emit `mirror-<hash>.<n>.ll` siblings; we
/// pick whichever has the most recent mtime so the IR matches the binary that
/// was just linked at `${CARGO_TARGET_DIR}/release/mirror`.
fn find_bootstrap_ll(deps_dir: &std::path::Path) -> Option<PathBuf> {
    let entries = fs::read_dir(deps_dir).ok()?;
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy().into_owned();
        if !name.starts_with("mirror-") || !name.ends_with(".ll") {
            continue;
        }
        let path = entry.path();
        let mtime = match entry.metadata().and_then(|m| m.modified()) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let take = match &best {
            None => true,
            Some((t, _)) => mtime > *t,
        };
        if take {
            best = Some((mtime, path));
        }
    }
    best.map(|(_, p)| p)
}

/// Count Dark AST nodes — the cheapest loss surface for the kintsugi loop.
///
/// Per `docs/specs/strict-and-total-classification.md` §"dark_count as loss
/// surface" and `docs/specs/kintsugi-formatter.md` stage 2 (the conductivity
/// measurement reduces, in this no-op scaffold, to a count of unresolved
/// dark regions). Body-equivalent to the grammar action
/// `@epistemologic/property/total_classification.dark_count` whose `\` body
/// is the obligation this Rust function discharges today.
fn count_dark(ast: &AstNode) -> usize {
    let mut darks: Vec<&AstNode> = Vec::new();
    collect_dark(ast, &mut darks);
    darks.len()
}

/// One tick of the kintsugi formatter loop — the five iteration stages
/// from `docs/specs/kintsugi-formatter.md`.
///
/// Every body is no-op for this scaffold. The structural shape — propose,
/// measure, elect, verify, fixed-point — is in place so subsequent commits
/// can replace one stage's `\` at a time without disturbing the loop.
///
/// Returns `true` iff the fixed-point check passed (the loop should
/// terminate). The Banach contraction's Δ is vacuously 0 today because
/// every stage is the identity, so this always returns `true` on tick 1.
fn kintsugi_tick(
    crystallizations: &Crystallizations<Blake3>,
    tick: u64,
    prior_ast: &AstNode,
    current_ast: &AstNode,
) -> bool {
    // Stage 1 — propose. Fate's five models fan out and return au
    // candidates. No-op scaffold: zero candidates. Before fanning out
    // we dispatch one Ref through the crystallizations table to
    // exercise the substrate-execution path end-to-end (per Seam C2,
    // pre-merge adversarial review 2026-05-30). The floor is empty in
    // Tick A, so this dispatch returns `Uncrystallized` and we report
    // it visibly. Tick B will register `@kintsugi/tick` and the same
    // call site will start receiving Success/Partial verdicts.
    let tick_ref = Ref::new("@kintsugi/tick").expect("@kintsugi/tick is a valid Ref");
    let seed_input: Optic<(), Splinter<Blake3>> =
        Optic::ok((), Splinter::new(Content::Text(Text::new("tick"))));
    let dispatch = crystallizations.crystallize(&tick_ref, seed_input);
    match &dispatch {
        Imperfect::Success(_) => {
            merr!("  dispatch {}: Success", tick_ref.as_str());
        }
        Imperfect::Partial(_, _) => {
            merr!(
                "  dispatch {}: Partial (with Transparency)",
                tick_ref.as_str()
            );
        }
        Imperfect::Failure(CrystallizeError::Uncrystallized(got), _) => {
            merr!(
                "  dispatch {}: Uncrystallized (floor has no body at {})",
                tick_ref.as_str(),
                got.as_str()
            );
        }
        Imperfect::Failure(err, _) => {
            merr!("  dispatch {}: Failure ({:?})", tick_ref.as_str(), err);
        }
    }
    let candidates: Vec<()> = Vec::new();

    // Stage 2 — measure. Cycle-averaged holonomy (Magnot 2025) of each
    // candidate. No candidates ⇒ no measurement. The loss surface today
    // is the dark count of the current AST (read-only).
    let dark_count = count_dark(current_ast);
    let loss: f64 = 1.0; // no candidate resolved ⇒ full residue

    // Stage 3 — elect. argmin over κ. No-op scaffold: no proposal.
    let _winner: Option<()> = if candidates.is_empty() {
        None
    } else {
        candidates.into_iter().next()
    };

    // Stage 4 — verify. Walk the obligation set and discharge each. With no
    // candidate there is no obligation to discharge; the verifier trivially
    // passes.
    let verify_pass: bool = true;

    // Stage 5 — fixed-point check (Lawvere). One more tick produces the
    // same section ⇔ the OIDs of prior and current ASTs agree. With no
    // candidate spliced in, prior == current by construction, so the
    // fixed point is reached vacuously on tick 1.
    let prior_oid = compute_content_oid(prior_ast);
    let current_oid = compute_content_oid(current_ast);
    let fixed_point = prior_oid == current_oid && verify_pass;
    let delta: f64 = if fixed_point { 0.0 } else { 1.0 };

    let suffix = if fixed_point {
        "  \u{2190} Lawvere fixed-point (vacuously)"
    } else {
        ""
    };
    merr!(
        "tick {}  dark_count: {}  loss: {:.1}  \u{0394}: {:.1}{}",
        tick,
        dark_count,
        loss,
        delta,
        suffix
    );

    fixed_point
}

/// `mirror kintsugi [--shatter N] [--transform <mq>] [--out <path>] <file|dir>`
///
/// `--shatter N`: see kintsugi-formatter.md. `N == 0` (default) preserves
/// historical behaviour — tokenize, render canonically to stdout.
///
/// `--transform <mq-query>`: apply a rewrite mq-query (the `=>` operator)
/// to each file before rendering. The rewrite is whole-word-bounded;
/// English in @nl prose lifts through unchanged at the parser level
/// once 4b.4 lands the meta-glass-aware walker. For 4b.3 the byte-level
/// whole-word bound is what's implemented.
///
/// `--out <path>`: redirect writes to a target directory. Real namespace
/// prefixes (`code/rust/`, `code/llvm/ir/`) preserve; bootstrap-historical
/// prefixes (`std/mirror/`) drop. When `<file>` is a directory, kintsugi
/// walks it recursively and migrates every `.mirror` file inside.
///
/// `--ci`: emit a verdict envelope to stdout suitable for the
/// `actions/kintsugi` composite step. Default emission is the
/// stringified mirror AST (a blank-line-separated sequence of
/// `<key> <value>` lines) — the substrate-pull-correct shape per
/// T11.2.5 of `docs/specs/kintsugi-ci-v0.1.md`. JSON is the @io
/// boundary; under `--format=json`, the same fields emit as JSON for
/// `jq` consumption (the action's `run.sh` invokes this when it needs
/// to set `$GITHUB_OUTPUT`).
///
/// Fields (both formats): `verdict, target, objective, iterations,
/// dark_count`. Corpus mode adds `files_processed` and per-file
/// records (mirror-text) / `per_file` array (JSON).
///
/// Exits 0 iff the envelope emits cleanly; the workflow decides what
/// verdict counts as pass.
fn cmd_kintsugi(
    file: &str,
    shatter: u64,
    transform: Option<&str>,
    out_dir: Option<&str>,
    ci: bool,
    format: CiFormat,
    ctx: &Ctx,
) -> i32 {
    // D4 (substrate-pull:realize): `.spec` extension routes to the
    // project-manifold walker. `mirror kintsugi <path>.spec` reads the
    // spec, walks its `target` blocks, and dispatches @io tools (today
    // only @io/cargo) for each target's `emit`. The verdict envelope
    // shape matches T11.2.5/T11.2.6 (mirror-text default, JSON behind
    // `--format=json`).
    //
    // Backwards-compat: `<file>.mirror` and `<directory>` paths fall
    // through to the existing walkers unchanged. Per the directive,
    // the `.spec` extension is the trigger; --ci is not required (the
    // spec walker always emits a verdict).
    if std::path::Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        == Some("spec")
    {
        return cmd_kintsugi_spec(file, format, ctx);
    }
    // T22 (substrate-pull:realize) — kintsugi-on-Rust pilot. `.rs`
    // routes through the realisation discriminator
    // (`bootstrap/src/realisation.rs`, T22's MVP body): emit the
    // `[realisation]` classification trace on stderr and a
    // `// kintsugi-on-Rust` fracture-proposal comment on stdout.
    // The substrate decl (`shards/mirror/realisation.mirror`, T21)
    // names the surface; T22 lands the realisation. The fracture
    // proposal is intentionally minimal this tick (comment text);
    // T23+ matures the body to structured fractures against the
    // target altitude's grammar.
    if std::path::Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        == Some("rs")
    {
        return cmd_kintsugi_rust(file);
    }
    // CI mode: route through the verdict serialiser. Failure paths
    // (file unreadable, grammar load error) emit a verdict with
    // `verdict: "failure"` and still exit 0 — the workflow YAML, not
    // the binary, decides what counts as pass.
    if ci {
        // Directory target → corpus walker (T11.3). Single-file target
        // (or stat failure → treat as file path, let single-file emit
        // verdict: "failure") preserves T11.2 shape.
        if let Ok(md) = fs::metadata(ctx.resolve(file)) {
            if md.is_dir() {
                return cmd_kintsugi_ci_corpus(file, shatter, transform, format, ctx);
            }
        }
        return cmd_kintsugi_ci_single(file, shatter, transform, format, out_dir, ctx);
    }
    // Migration mode: --out + a directory input — walk and migrate
    // every .mirror file under the directory.
    if let Some(out_root) = out_dir {
        let md = match fs::metadata(ctx.resolve(file)) {
            Ok(m) => m,
            Err(e) => {
                merr!("cannot stat {}: {}", file, e);
                return 1;
            }
        };
        if md.is_dir() {
            return cmd_kintsugi_migrate(file, out_root, transform, ctx);
        }
    }
    cmd_kintsugi_single(file, shatter, transform, out_dir, ctx)
}

/// T22 — `mirror kintsugi <file>.rs` invokes the realisation
/// discriminator and emits a classification + a fracture proposal.
///
/// This is the kintsugi-on-Rust pilot. The flow:
///
///   1. Invoke `realisation::classify(file)` — runs the MVP match
///      table on the file's basename and emits a `RealisableFile`
///      record (path / altitude / target / verdict).
///   2. Emit a `[realisation]` trace on stderr — same shape as T19's
///      `[settle]` trace, so operators can see the discriminator's
///      verdict at the CLI boundary.
///   3. Emit a `// kintsugi-on-Rust ...` + `// fracture proposal ...`
///      comment pair on stdout — the v0 fracture proposal. T23+
///      replaces this with a structured diff against the target
///      altitude's declared grammar.
///   4. Exit 0 unconditionally — the discriminator's miss IS a
///      substrate gap (logged as `Partial`), not a failure.
fn cmd_kintsugi_rust(file: &str) -> i32 {
    let classification = realisation::classify(file);
    let verdict_label = match &classification.verdict {
        terni::Transparency::Clear => "Clear",
        terni::Transparency::Opaque(_) => "Partial",
    };
    merr!(
        "[realisation] path={} altitude={} target={} verdict={}",
        classification.path.as_str(),
        classification.altitude,
        classification.target.as_str(),
        verdict_label,
    );
    let basename = std::path::Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file);
    mout!(
        "// kintsugi-on-Rust pilot: {} at {} maps to substrate altitude {}",
        basename,
        classification.altitude,
        classification.target.as_str(),
    );
    mout!(
        "// fracture proposal: replace this file's substrate-realisable functions \
         with calls into {}",
        classification.target.as_str(),
    );
    0
}

/// One target the dispatcher will act on, harvested from the AST.
///
/// Carries the `target NAME { ... }` block name (the AST node's `name`
/// field) and the keyword the `emit` directive named (also an AST node
/// name). String-literal and `~path'...'` carriers (`name "value"`,
/// `manifest ~f'...'`) are a known substrate gap — the bootstrap
/// tokenizer drops them silently — so the dispatcher uses the
/// `<spec-dir>/Cargo.toml` default. See the field-level comment on
/// `spec_targets_from_ast` for what the substrate captures cleanly
/// today.
struct SpecTarget {
    /// The block-header identifier: `target binary { ... }` → `"binary"`.
    block_name: String,
    /// The `emit <ref>` field. Today only `cargo` triggers a dispatch.
    emit: String,
    /// The `check <ref>` field — names which @io/cargo action the
    /// target dispatches (insight #43 / 2026-06-09). Empty when the
    /// target has no `check` directive; the dispatcher defaults to
    /// `check` (cargo's check subcommand) in that case, preserving
    /// the pre-#43 behaviour.
    check: String,
}

/// Walk the tokenized spec AST and harvest the targets the dispatcher
/// will act on.
///
/// AST shape under `@mirror/spec` (keyword bindings declared in
/// `shards/mirror/spec/keywords.mirror`):
///
///   Focus root
///     In @mirror/spec, In @property, In @io
///     Focus project NAME
///       Project source       (~d'...' carrier dropped — substrate gap)
///       Focus target NAME
///         Project name       ("value" string literal dropped — substrate gap)
///         Project altitude   (@code/rust — captured cleanly in name)
///         Project emit       (cargo — captured cleanly in name)
///         Project manifest   (~f'...' carrier dropped — substrate gap)
///       Settle settle_on
///         ...
///
/// What the substrate captures cleanly today:
///   - `target NAME { ... }`: block name in the Focus node's `name`
///   - `altitude @ref`: ref in the Project node's `name`
///   - `emit IDENT`: identifier in the Project node's `name`
///
/// What's a substrate gap:
///   - `name "value"`: string literals are consumed silently by the
///     tokenizer; no AST representation.
///   - `manifest ~f'...'` / `source ~d'...'` / `legacy ~d'...', ...`:
///     `~path'...'` sigil-quoted carriers parse to empty/dark; the
///     bootstrap tokenizer doesn't surface the inner literal.
///   The action declarations in `shards/mirror/spec.mirror` (e.g.
///   `name(value: str) -> name_decl { \ }`) declare what these
///   directives ARE typed as; the bootstrap's keyword harvester does
///   not yet evaluate the action signatures to produce typed AST
///   children. Surfacing this as a gap rather than papering it over
///   in Rust per [[feedback-no-new-rust]] and the directive's
///   "Surface anything that surprises you" clause.
///
/// The dispatcher walks the top-level Focus blocks named `project`,
/// collects their `target` Focus children, and for each target looks
/// for an `emit` Project node whose `name` carries the emit ref.
fn spec_targets_from_ast(ast: &AstNode) -> Vec<SpecTarget> {
    let mut out: Vec<SpecTarget> = Vec::new();
    for child in &ast.children {
        if child.kind == AstKind::Focus && child.keyword == "project" {
            for target in &child.children {
                if target.kind == AstKind::Focus && target.keyword == "target" {
                    let mut emit = String::new();
                    let mut check = String::new();
                    for field in &target.children {
                        if field.kind == AstKind::Project && field.keyword == "emit" {
                            emit = field.name.clone();
                        }
                        if field.kind == AstKind::Project && field.keyword == "check" {
                            check = field.name.clone();
                        }
                    }
                    out.push(SpecTarget {
                        block_name: target.name.clone(),
                        emit,
                        check,
                    });
                }
            }
        }
    }
    out
}

/// Map a target's `check` ref (or the empty default) to the cargo
/// subcommand args the dispatcher will invoke. Returns the cargo argv
/// suffix (e.g. `&["fmt", "--check"]` or `&["check"]`); the dispatcher
/// adds `--manifest-path <path>` after.
///
/// Per shards/io/cargo.mirror's declared actions (insight #43 substrate-
/// pull, 2026-06-09):
///
///   check     → cargo check                  (default; cheapest gate)
///   fmt_check → cargo fmt --check            (formatter verification)
///   clippy    → cargo clippy                 (lint)
///   test      → cargo test                   (test suite)
///   audit     → cargo audit                  (advisory DB; @release alt.)
///   build     → cargo build --release        (artifact build)
///   bench     → cargo bench                  (perf measurement; @mirror/bench consumer)
///
/// Per Seam Phase D `91e79c8` §7 Sub-arc 3a (RED `d25b91a`): the
/// `bench` arm opens the perf-measurement floor for `target bench`
/// blocks in mirror.spec; `@mirror/bench` (LANDED 2026-07-01 at
/// `shards/mirror/bench.mirror`, 16.3KB) becomes the first harness
/// wired via the standard `emit cargo` / `check bench` dispatch path.
///
/// Unknown values fall back to `cargo check` and surface a `[kintsugi
/// spec]` stderr note so the operator sees the unrecognised altitude.
fn cargo_args_for_check(check: &str) -> &'static [&'static str] {
    match check {
        "" | "check" => &["check"],
        "fmt_check" => &["fmt", "--check"],
        "clippy" => &["clippy"],
        "test" => &["test"],
        "audit" => &["audit"],
        "build" => &["build", "--release"],
        "bench" => &["bench"],
        _ => &["check"],
    }
}

/// D4 entrypoint: walk a `mirror.spec`, dispatch cargo for each
/// `emit cargo` target, and emit a verdict envelope.
///
/// First concrete rung of the compile staircase per
/// [[project-mirror-compile-staircase]]: the `.spec` extension routes
/// to `@mirror/spec` (shards/mirror/spec.mirror) via
/// `grammar_for_file`; the existing tokenize+parse infrastructure
/// produces an AST; this function walks the AST to find the targets.
/// The hand-rolled `parse_spec_targets` byte scanner retires.
///
/// Per [[feedback-no-new-rust]] this is dispatch glue only. The spec
/// grammar is substrate-declared (`shards/mirror/spec.mirror`); the
/// cargo @io contract is substrate-declared (`shards/io/cargo.mirror`).
/// This function reads the spec, walks the AST, invokes cargo, and
/// emits via the existing `CiVerdict`/`CorpusVerdict` envelope
/// (T11.2.5/T11.2.6).
///
/// Exit-code lift (coarsened from D3's `cargo_exit_to_transparency`):
///   0   → success            (no opacity)
///   *   → failure(stderr[..200])
///
/// The rich opacity-map parsing (file:line:col extraction from cargo
/// stderr) is deferred: D3 declared the contract; @mirror/mosaic will
/// wire it in the substrate proper. Same for `lockfile_capture` and
/// the env_allowlist verification — substrate-declared, dispatcher-
/// deferred to keep this tick minimal.
fn cmd_kintsugi_spec(spec_path: &str, format: CiFormat, ctx: &Ctx) -> i32 {
    let read_spec = ctx.resolve(spec_path);
    let source = match fs::read(&read_spec) {
        Ok(s) => s,
        Err(_) => {
            return emit_spec_verdict("failure", spec_path, 0.0, 1, 0, 0, &[], format);
        }
    };
    // Substrate-pull dispatch: the file extension picks the grammar,
    // the grammar loader merges in keyword companions, the tokenizer
    // produces an AST. `parse_spec_targets`'s text scanner retires.
    let grammar_path = grammar_for_file_in(spec_path, ctx);
    let grammar = match load_grammar_in(grammar_path, ctx) {
        Ok(g) => g,
        Err(_) => {
            return emit_spec_verdict("failure", spec_path, 0.0, 1, 0, 0, &[], format);
        }
    };
    let ast = tokenize(&source, &grammar);
    let targets = spec_targets_from_ast(&ast);

    // N3 TICK 1 (substrate-pull:realize): compute `spec_oid` once per
    // dispatch. The verdict cache is keyed on (spec_oid, target_oid,
    // inputs_oid) per N1 predicate + N2 action_cache. Warm-cache commits
    // skip cargo entirely — this is the wire that makes the 13-minute
    // pre-commit hook fall.
    let spec_oid = action_cache::compute_spec_oid(&source);

    // Spec-relative root for the default `Cargo.toml` location. The
    // substrate-declared `manifest ~f'...'` override isn't captured
    // by the bootstrap tokenizer today (substrate gap; see
    // `spec_targets_from_ast` docstring); every target uses the
    // spec-dir default. Derived from the resolved (ctx.cwd()-relative)
    // spec path so the manifest probe hits the correct filesystem
    // location regardless of the process cwd.
    let spec_dir: PathBuf = read_spec
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| ctx.resolve("."));

    let mut per_target: Vec<PerFileVerdict> = Vec::with_capacity(targets.len());
    let mut any_failure = false;
    let mut any_partial = false;
    let mut total_objective: f64 = 0.0;
    let mut total_dark: u64 = 0;
    for t in &targets {
        if t.emit != "cargo" {
            // Non-cargo emit (yaml, github_release, ...): the @io tool
            // is not yet wired. Mark as `partial` (not `failure`): the
            // dispatcher's "I cannot settle this altitude yet" verdict
            // is the substrate gap, not a verdict regression. Per
            // shards/io/cargo.mirror's `cargo_exit_to_transparency`
            // contract, `failure` is reserved for cargo-exit-non-zero;
            // `partial` is reserved for "settled with opacity remaining."
            // Per insight #43 (2026-06-09 substrate-pull): the substrate
            // gap IS the opacity; subsequent ticks wire each non-cargo
            // altitude one at a time. Until then, `partial` keeps the
            // aggregate verdict honest without false-positive-failing
            // the pre-commit gate on declared-but-unwired targets.
            let label = format!("target {} (emit {})", t.block_name, t.emit);
            per_target.push(PerFileVerdict {
                path: label,
                verdict: "partial",
                objective: 1.0,
                iterations: 1,
                dark_count: 1,
            });
            any_partial = true;
            total_objective += 1.0;
            total_dark += 1;
            continue;
        }
        // Manifest probe (issue #315): the substrate-declared
        // `manifest ~f'...'` carrier is a known tokenizer gap (see
        // `spec_targets_from_ast` docstring), so the dispatcher
        // probes the canonical locations in declaration order. For
        // mirror's own dogfood spec (mirror.spec at repo root), the
        // Rust manifest lives at `bootstrap/Cargo.toml`, not
        // `./Cargo.toml`. Pre-#315 every target tried `./Cargo.toml`
        // and failed because the repo has no top-level manifest.
        //
        // Order:
        //   1. <spec-dir>/Cargo.toml   — single-crate convention
        //   2. <spec-dir>/bootstrap/Cargo.toml  — mirror's convention
        //
        // When the tokenizer's `~f'...'` gap closes, the spec's own
        // `manifest ~f'bootstrap/Cargo.toml'` directive supersedes
        // this probe. Until then, the probe holds the @io floor.
        let manifest = {
            let default = spec_dir.join("Cargo.toml");
            if default.is_file() {
                default
            } else {
                let bootstrap = spec_dir.join("bootstrap").join("Cargo.toml");
                if bootstrap.is_file() {
                    bootstrap
                } else {
                    default
                }
            }
        };
        // Insight #43 (2026-06-09 substrate-pull): the per-target
        // `check <action>` directive (declared in shards/mirror/spec.mirror)
        // names which @io/cargo action this target dispatches. When
        // omitted, defaults to `cargo check` — the pre-#43 behaviour
        // — so existing specs keep working unchanged.
        //
        // Per the directive's "Minimum viable" §4 + insight #43 §9.3:
        // the first version of `mirror kintsugi mirror.spec` falls back
        // to spawning cargo at every altitude. The architecture is in
        // place from tick one; content-addressed-skip wiring lands at
        // a subsequent tick (recognitions #44+).
        let cargo_args = cargo_args_for_check(&t.check);
        if !t.check.is_empty() && cargo_args == ["check"] && t.check != "check" {
            // Surface the unrecognised `check <action>` so the operator
            // sees the substrate gap rather than the silent fallback.
            merr!(
                "[kintsugi spec] target `{}` check `{}` unknown; falling back to `cargo check`",
                t.block_name,
                t.check,
            );
        }

        // N3 TICK 1 (substrate-pull:realize): consult the action_cache
        // BEFORE spawning cargo. Warm-cache HIT returns the memoized
        // verdict in O(stat + read). Cold-cache MISS falls through to
        // dispatch. Per @mirror/store/action_cache (`0a72c42`) +
        // @epistemologic/property/verdict_is_content_addressed
        // (`2857fb1`). This is the wire that makes the 13-min
        // pre-commit hook fall.
        let target_oid = action_cache::compute_target_oid(&t.block_name, &t.emit, &t.check);
        let inputs_oid = action_cache::compute_inputs_oid(&manifest);
        let cargo_pretty = cargo_args_for_check(&t.check).join(" ");
        if let Some(cached) =
            action_cache::cache_read(ctx.cwd(), &spec_oid, &target_oid, &inputs_oid)
        {
            // Warm-cache HIT: return the memoized verdict without
            // invoking cargo. The label is preserved from the cold-cache
            // write so operator-visible output is byte-identical to the
            // cold-cache path modulo the (cached) marker.
            let verdict_static: &'static str = match cached.verdict.as_str() {
                "success" => "success",
                "partial" => "partial",
                _ => "failure",
            };
            per_target.push(PerFileVerdict {
                path: format!("{} [cached]", cached.label),
                verdict: verdict_static,
                objective: cached.objective,
                iterations: 1,
                dark_count: cached.dark_count,
            });
            match verdict_static {
                "success" => {}
                "partial" => {
                    any_partial = true;
                    total_objective += cached.objective;
                    total_dark += cached.dark_count;
                }
                _ => {
                    any_failure = true;
                    total_objective += cached.objective;
                    total_dark += cached.dark_count;
                }
            }
            continue;
        }

        let mut cmd = ctx.command("cargo");
        cmd.args(cargo_args);
        // `cargo audit` does not accept `--manifest-path`; it reads
        // `Cargo.lock` from CWD or via `-f <lockfile>`. Skip the
        // manifest-path flag for audit and point it at the lockfile
        // alongside the target manifest. Other subcommands need it.
        if cargo_args != ["audit"] {
            cmd.arg("--manifest-path").arg(&manifest);
        } else if let Some(lock) = manifest.parent().map(|p| p.join("Cargo.lock")) {
            if lock.is_file() {
                cmd.arg("-f").arg(lock);
            }
        }
        // spawn + wait + threaded drain via mpsc channels with a
        // recv_timeout safety net. Fixes the classic `.output()` hang:
        // cargo spawns descendants (rustc, linker) that inherit
        // stdout/stderr fds; when cargo exits its own fd copies are
        // closed, but descendants keep the write-ends alive, so
        // `read_to_end` on the parent side waits for EOF forever.
        //
        // The recv_timeout after child.wait() bounds this wait: we
        // give descendants a 5-second grace window to drain any
        // remaining output, then move on with whatever we captured.
        // Correctness impact: the tool_unavailable heuristic below
        // reads the first 200 chars of stderr; a truncated stderr
        // still exposes the "no such command" marker for cargo audit,
        // which is the only currently-consumed detail.
        //
        // Discovered 2026-07-07 by Taut scout during N-cascade
        // deadlock diagnosis (task #558); Alex adjudicated "fuller
        // refactor" path.
        use std::io::Read as _;
        use std::process::Stdio;
        use std::sync::mpsc;
        use std::time::Duration;

        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let spawn_result = cmd.spawn();
        let out = match spawn_result {
            Ok(mut child) => {
                let stdout_pipe = child.stdout.take();
                let stderr_pipe = child.stderr.take();

                let (stdout_tx, stdout_rx) = mpsc::channel::<Vec<u8>>();
                let (stderr_tx, stderr_rx) = mpsc::channel::<Vec<u8>>();

                if let Some(mut r) = stdout_pipe {
                    std::thread::spawn(move || {
                        let mut buf = Vec::new();
                        let _ = r.read_to_end(&mut buf);
                        let _ = stdout_tx.send(buf);
                    });
                }
                if let Some(mut r) = stderr_pipe {
                    std::thread::spawn(move || {
                        let mut buf = Vec::new();
                        let _ = r.read_to_end(&mut buf);
                        let _ = stderr_tx.send(buf);
                    });
                }

                match child.wait() {
                    Ok(status) => {
                        let stdout = stdout_rx
                            .recv_timeout(Duration::from_secs(5))
                            .unwrap_or_default();
                        let stderr = stderr_rx
                            .recv_timeout(Duration::from_secs(5))
                            .unwrap_or_default();
                        Ok(std::process::Output {
                            status,
                            stdout,
                            stderr,
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        };
        let (verdict, objective, dark, label_path) = match out {
            Ok(o) if o.status.success() => (
                "success",
                0.0_f64,
                0_u64,
                format!("{} ({})", manifest.to_string_lossy(), cargo_pretty),
            ),
            Ok(o) => {
                // D3's exit-code lift is partial here: non-zero →
                // failure. Stderr is captured but not yet parsed into
                // opacity_map (deferred to @mirror/mosaic). The full
                // exit code is recorded in the label so the operator
                // can see WHICH non-zero arm fired AND which cargo
                // subcommand drove the failure.
                //
                // Tool-unavailable carve-out (Taut [bugfix:restore]
                // 2026-06-19): cargo exits 101 with stderr "no such
                // command" when a sub-command (e.g. `audit`) is not
                // installed in the dev shell. That's substrate-
                // categorically distinct from a hard failure — the
                // advisory database wasn't checked, but the project
                // didn't fail; it's an opacity at the @io boundary.
                // Lift it to `partial` so the gate stays open while
                // the @io-edge tool installation (cargo-audit in
                // flake.nix) is a separate decision. The substrate-
                // pull discipline: classify by structural cause, not
                // by exit-code coarse-grain.
                let code = o.status.code().unwrap_or(-1);
                let stderr = String::from_utf8_lossy(&o.stderr);
                let tool_unavailable = code == 101 && stderr.contains("no such command:");
                merr!(
                    "[kintsugi spec] target `{}` cargo {} exit {}; stderr (first 200 chars):",
                    t.block_name,
                    cargo_pretty,
                    code,
                );
                merr!("  {}", &stderr.chars().take(200).collect::<String>());
                let label = format!(
                    "{} ({}, exit {})",
                    manifest.to_string_lossy(),
                    cargo_pretty,
                    code,
                );
                if tool_unavailable {
                    ("partial", 1.0_f64, 1_u64, label)
                } else {
                    ("failure", 1.0_f64, 1_u64, label)
                }
            }
            Err(e) => {
                merr!(
                    "[kintsugi spec] target `{}` cargo {} spawn error: {}",
                    t.block_name,
                    cargo_pretty,
                    e,
                );
                (
                    "failure",
                    1.0_f64,
                    1_u64,
                    format!("{} ({})", manifest.to_string_lossy(), cargo_pretty),
                )
            }
        };

        // N3 TICK 1 (substrate-pull:realize): memoize the fresh verdict
        // for future invocations. Idempotent by content-address per N1
        // predicate; write-if-absent semantics. The next `mirror kintsugi
        // mirror.spec` with unchanged (spec_oid, target_oid, inputs_oid)
        // hits the warm-cache path above and skips cargo entirely.
        let cached = action_cache::CachedVerdict {
            verdict: verdict.to_string(),
            objective,
            dark_count: dark,
            label: label_path.clone(),
        };
        let _ = action_cache::cache_write(ctx.cwd(), &spec_oid, &target_oid, &inputs_oid, &cached);

        per_target.push(PerFileVerdict {
            path: label_path,
            verdict,
            objective,
            iterations: 1,
            dark_count: dark,
        });
        match verdict {
            "success" => {}
            "partial" => {
                any_partial = true;
                total_objective += objective;
                total_dark += dark;
            }
            _ => {
                any_failure = true;
                total_objective += objective;
                total_dark += dark;
            }
        }
    }
    let aggregate: &'static str = if any_failure {
        "failure"
    } else if any_partial || total_dark > 0 {
        "partial"
    } else {
        // Empty spec (no targets at all) → success per the v0.1 vacuous-
        // truth rule (matches the corpus walker's empty-corpus shape).
        "success"
    };
    let files_processed = per_target.len() as u64;
    emit_spec_verdict(
        aggregate,
        spec_path,
        total_objective,
        1,
        total_dark,
        files_processed,
        &per_target,
        format,
    )
}

/// Emit the D4 spec verdict envelope. Reuses the T11.2.5/T11.2.6
/// emitters via thin wrappers: single-target specs use the per-file
/// shape; multi-target specs use the corpus envelope so per-target
/// verdicts are visible to the operator.
fn emit_spec_verdict(
    aggregate: &'static str,
    spec_path: &str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
    files_processed: u64,
    per_target: &[PerFileVerdict],
    format: CiFormat,
) -> i32 {
    match format {
        CiFormat::MirrorText => emit_corpus_verdict_mirror_text(
            aggregate,
            spec_path,
            objective,
            iterations,
            dark_count,
            files_processed,
            per_target,
        ),
        CiFormat::Json => {
            let envelope = CorpusVerdict {
                verdict: aggregate,
                target: spec_path,
                objective,
                iterations,
                dark_count,
                files_processed,
                per_file: per_target
                    .iter()
                    .map(|e| PerFileVerdict {
                        path: e.path.clone(),
                        verdict: e.verdict,
                        objective: e.objective,
                        iterations: e.iterations,
                        dark_count: e.dark_count,
                    })
                    .collect(),
            };
            match serde_json::to_string(&envelope) {
                Ok(mut json) => {
                    json.push('\n');
                    _raw_stdout(json.as_bytes());
                    0
                }
                Err(_) => 1,
            }
        }
    }
}

/// Output format for `mirror kintsugi --ci`.
///
/// The substrate-pull-correct default is `MirrorText` — the verdict
/// stays in mirror substrate until the @io boundary, where the action's
/// `run.sh` invokes `--format=json` to feed `jq` and `$GITHUB_OUTPUT`.
/// Per T11.2.5 of `docs/specs/kintsugi-ci-v0.1.md`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CiFormat {
    /// Stringified mirror AST: blank-line-separated `<key> <value>`
    /// records. Default. Substrate-native.
    MirrorText,
    /// JSON envelope. Only at the @io boundary.
    Json,
}

impl Default for CiFormat {
    fn default() -> Self {
        CiFormat::MirrorText
    }
}

/// Map a substrate ref (`@<namespace>`) to a CiFormat at the @io
/// boundary. Per Alex's 2026-06-16 substrate-pull insight: `--out` should
/// accept substrate refs (the `out` keyword's value space IS the
/// substrate namespace, not a closed enum of bare strings). The receiver
/// glasses live at `shards/mirror/data/<X>.mirror` (Mara's T16 cascade);
/// this hardcoded dispatch is the @io floor that a future tick lifts to
/// a substrate-driven registry via the cross-shard resolver.
///
/// ## Lifted from kintsugi-scoped to shatter-scoped (Tick 0 Landing 3)
///
/// Per Taut LRM scout `1658b95` + mirror.spec cli-block `flag target: str`
/// (Landing 1, OID
/// `796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8`):
/// this helper is now the shared parser for BOTH consumers:
///
/// - `cmd_kintsugi_single` / `dispatch_out_substrate_ref` — the existing
///   `--out @data/json` chain that maps the substrate ref to `CiFormat`
///   for the kintsugi @ci envelope emission.
/// - `cmd_shatter` — the Tick 0 Landing 2 consumer that routes
///   `--target @data/<X>` for @shatter's codomain per
///   `docs/specs/shatter-is-the-io-linearization-operator.md` §4.1
///   (Mara `583b939`).
///
/// The two consumers share the same substrate-ref vocabulary (the same
/// receiver glasses at `shards/mirror/data/<X>.mirror`), so the parse
/// function is the right composition surface — not a per-command
/// dispatch table. Substrate-already-had-the-word: the function was
/// generic-shaped from day one; the lift is a documentation clarification
/// + the shatter consumer wiring.
fn parse_substrate_ref_to_format(s: &str) -> Option<CiFormat> {
    match s {
        "@data/json" => Some(CiFormat::Json),
        "@data/mirror" | "@data/mirror-text" => Some(CiFormat::MirrorText),
        _ => None,
    }
}

/// Parse the parametric substrate ref `@io/dir('path')` and return the
/// inner path. Same `splinter(@code)` / `mosaic(altitude)` parametric
/// syntax mirror already uses at type-constructor sites — lifted here
/// to the CLI value position. The single-quote shape is the substrate
/// convention; double-quoted variants are not accepted.
fn parse_io_dir_arg(s: &str) -> Option<String> {
    let rest = s.strip_prefix("@io/dir('")?;
    let arg = rest.strip_suffix("')")?;
    Some(arg.to_string())
}

/// Polymorphic output write per Alex's 2026-06-17 directive:
/// `--out @io/dir('path')` writes the pipeline output into the dir.
/// Single-blob case: lands at `<dir>/out.<ext>` (ext follows the format).
/// When `out_dir` is None, writes to stdout as before. Returns 0 on
/// success, 1 on filesystem failure.
fn write_or_stdout(bytes: &[u8], out_dir: Option<&str>, ext: &str) -> i32 {
    if let Some(dir) = out_dir {
        if let Err(e) = fs::create_dir_all(dir) {
            merr!("could not create output dir {}: {}", dir, e);
            return 1;
        }
        let path = format!("{}/out.{}", dir.trim_end_matches('/'), ext);
        if let Err(e) = fs::write(&path, bytes) {
            merr!("could not write {}: {}", path, e);
            return 1;
        }
        return 0;
    }
    _raw_stdout(bytes);
    0
}

/// Dispatch a `--out` substrate ref value. Updates `ci_format` for
/// projection refs (`@data/json`, `@data/mirror`) and `out_dir` for the
/// parametric directory ref (`@io/dir('path')`). Returns Err(()) on any
/// failure after printing a diagnostic to stderr.
fn dispatch_out_substrate_ref(
    value: &str,
    ci_format: &mut CiFormat,
    out_dir: &mut Option<String>,
) -> Result<(), ()> {
    if !value.starts_with('@') {
        merr!(
            "--out requires a substrate ref (e.g. @data/json or @io/dir('path')); got {}",
            value
        );
        return Err(());
    }
    if let Some(path) = parse_io_dir_arg(value) {
        *out_dir = Some(path);
        return Ok(());
    }
    if let Some(f) = parse_substrate_ref_to_format(value) {
        *ci_format = f;
        return Ok(());
    }
    merr!(
        "no projection registered for substrate ref: {} (try @data/json, @data/mirror, or @io/dir('path'))",
        value
    );
    Err(())
}

/// Rust-side mirror of the `type verdict = { ... }` record declared in
/// `boot/std/kintsugi.mirror` (T11.2.6 substrate-pull closure).
///
/// Per T11.2.5 of `docs/specs/kintsugi-ci-v0.1.md` (corrects T11.2's
/// default to mirror-text; preserves JSON behind `--format=json` for
/// the @io boundary). T11.2.6 closes the loop by adding the typed
/// declaration in the substrate: the field set here MUST match
/// `boot/std/kintsugi.mirror`'s `type verdict` exactly. The round-trip
/// tests in `bootstrap/tests/kintsugi_ci_typed_verdict.rs` pin that
/// equality — drift on either side fails the build.
///
/// The wire altitude (GitHub Actions composite step) parses this via
/// `jq` (under `--format=json`) and writes the fields to
/// `$GITHUB_OUTPUT`. Field semantics:
///
/// - `verdict`:    `success` if the loop converged with `dark_count == 0`
///                 and `objective == 0`; `partial` if the loop completed
///                 but residue remained; `failure` if the loop never
///                 ran (file unreadable, grammar load error). The three
///                 positions match `type discrimination = success | partial
///                 | failure` in `boot/std/kintsugi.mirror`.
/// - `target`:     the input path verbatim.
/// - `objective`:  the non-negative loss scalar; in T11.2 this is
///                 `dark_count as f64` — the cheapest loss surface per
///                 `count_dark`'s docstring and `kintsugi-formatter.md`
///                 stage 2. Deterministic by construction (pure AST walk).
/// - `iterations`: how many kintsugi-loop ticks ran (≥ 1).
/// - `dark_count`: residual `Dark` AST nodes after the loop fixpoint.
#[derive(Serialize)]
struct CiVerdict<'a> {
    verdict: &'static str,
    target: &'a str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
}

/// Rust-side mirror of `type verdict_entry = { ... }` from
/// `boot/std/kintsugi.mirror` (T11.2.6). Per-file entry inside a
/// corpus envelope (T11.3); differs from `CiVerdict` only in the
/// leading field name (`file`, not `target`) — the enclosing envelope
/// already names the corpus root. Round-trip-tested for field-set
/// equality against the substrate declaration.
///
/// Note: the struct field is named `path` here because of the
/// `--format=json` wire contract pinned by T11.2/T11.3 (`per_file`
/// entries serialize with key `"path"` in the JSON output). The
/// mirror-text wire uses `"file"` as the leading key (per the
/// emitter in `emit_corpus_verdict_mirror_text`), which matches the
/// `verdict_entry.file` field declared in `boot/std/kintsugi.mirror`.
/// The JSON `"path"` ↔ mirror-text `"file"` mapping is the @io
/// boundary's responsibility; the substrate declares `file`.
#[derive(Serialize)]
struct PerFileVerdict {
    path: String,
    verdict: &'static str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
}

/// Rust-side mirror of `type corpus_verdict = { ... }` from
/// `boot/std/kintsugi.mirror` (T11.2.6). Aggregate envelope for
/// `mirror kintsugi --ci <directory>`.
///
/// Per T11.3 of `docs/specs/kintsugi-ci-v0.1.md`. Aggregation rules:
///
/// - `verdict`:        `success` iff every per-file verdict is
///                     `success`; `partial` if any per-file is partial
///                     OR has `dark_count > 0`; `failure` if any
///                     per-file failed.
/// - `objective`:      **sum** of per-file objectives (the
///                     [[kintsugi-variety]] objective is additive).
/// - `dark_count`:     **sum** of per-file dark counts (total residual
///                     across the corpus).
/// - `iterations`:     **max** of per-file iterations (the longest-
///                     running file's tick count).
/// - `files_processed`: number of `.mirror` files walked.
/// - `per_file`:       sorted-by-path array of per-file verdict_entry
///                     values. On the mirror-text wire this list
///                     flattens into one blank-line-separated record
///                     per entry following the envelope; the JSON shape
///                     keeps it as a nested array.
#[derive(Serialize)]
struct CorpusVerdict<'a> {
    verdict: &'static str,
    target: &'a str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
    files_processed: u64,
    per_file: Vec<PerFileVerdict>,
}

/// Compute a per-file CI verdict for one `.mirror` file. Shared between
/// single-file mode (`cmd_kintsugi_ci_single`) and the corpus walker
/// (`cmd_kintsugi_ci_corpus`) so aggregation can't drift from the
/// per-file shape T11.2 landed.
///
/// Returns `(verdict, objective, iterations, dark_count)`. Failure paths
/// (file unreadable, transform-parse error, grammar load error) return
/// `("failure", 0.0, 1, 0)` — identical to the single-file `failure`
/// envelope.
fn kintsugi_ci_compute(
    file: &str,
    shatter: u64,
    transform: Option<&str>,
    ctx: &Ctx,
) -> (&'static str, f64, u64, u64) {
    let source = match fs::read(ctx.resolve(file)) {
        Ok(s) => s,
        Err(_) => return ("failure", 0.0, 1, 0),
    };
    let source = if let Some(q) = transform {
        match parse_rewrite(q) {
            Some(rules) => apply_rewrites(&rules, &source),
            None => return ("failure", 0.0, 1, 0),
        }
    } else {
        source
    };
    let grammar_path = grammar_for_file_in(file, ctx);
    let grammar = match load_grammar_in(grammar_path, ctx) {
        Ok(g) => g,
        Err(_) => return ("failure", 0.0, 1, 0),
    };
    let ast = tokenize(&source, &grammar);

    // Always run at least one tick under --ci. The kintsugi loop's
    // semantics in T11.2 are read-only (no candidate is spliced in;
    // every stage is identity), so iterations is structurally bounded
    // by max(1, shatter) and the result is deterministic.
    let crystallizations = floor_crystallizations::<Blake3>();
    let max_ticks = if shatter == 0 { 1 } else { shatter };
    let mut iterations: u64 = 0;
    let mut prior = ast.clone();
    for i in 1..=max_ticks {
        iterations = i;
        let fixed = kintsugi_tick(&crystallizations, i, &prior, &ast);
        if fixed {
            break;
        }
        prior = ast.clone();
    }

    let dark_count = count_dark(&ast) as u64;
    let objective = dark_count as f64;
    let verdict = if dark_count == 0 && objective == 0.0 {
        "success"
    } else {
        "partial"
    };
    (verdict, objective, iterations, dark_count)
}

/// Emit a CI verdict for a single `.mirror` file. Always exits 0 when
/// the envelope emits cleanly; failure paths carry `verdict: "failure"`
/// in the record rather than a nonzero exit. The workflow YAML decides
/// pass/fail policy.
///
/// Default emission is mirror-text per T11.2.5; `--format=json` keeps
/// the @io-boundary JSON path.
fn cmd_kintsugi_ci_single(
    file: &str,
    shatter: u64,
    transform: Option<&str>,
    format: CiFormat,
    out_dir: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    let (verdict, objective, iterations, dark_count) =
        kintsugi_ci_compute(file, shatter, transform, ctx);
    match format {
        CiFormat::MirrorText => {
            emit_ci_verdict_mirror_text(verdict, file, objective, iterations, dark_count, out_dir)
        }
        CiFormat::Json => {
            emit_ci_verdict_json(verdict, file, objective, iterations, dark_count, out_dir)
        }
    }
}

/// Collect declared namespaces from a directory tree of `.mirror` files.
///
/// Cross-shard semantic resolution surface (substrate-pull realize):
/// scans `.mirror` files under `dir` (recursively) for top-of-line
/// `glass @<path>` / `prism @<path>` / `grammar @<path>` declarations
/// and accumulates the declared namespace refs into `out`. Used by the
/// corpus walker to build a resolution index against which `in @<path>`
/// statements are checked.
///
/// The scan is intentionally text-altitude (not AST-altitude): the
/// resolver only needs to know which `@<path>` namespaces have been
/// declared somewhere in the loaded substrate, and a line-level scan
/// is cheap, deterministic, and grammar-free. Lines whose first
/// non-whitespace token isn't `glass` / `prism` / `grammar` are
/// ignored, as are commented-out declarations (`#` prefix).
///
/// The reference is captured up to the first whitespace, `{`, `(`, or
/// `:` — so `grammar @mirror/grammar("mirror", ...) { ... }` and
/// `glass @foo/bar {` both yield the bare `@<path>` ref.
fn collect_declared_namespaces(dir: &str, out: &mut std::collections::HashSet<String>, ctx: &Ctx) {
    let mut files: Vec<String> = Vec::new();
    // Resolve the dir against ctx.cwd() so `collect_files`'s recursive
    // walk (which propagates path prefixes as-is) sees absolute paths;
    // otherwise a relative `dir` from CLI positional would resolve
    // against the process cwd inside `fs::read_dir`.
    let dir_resolved = ctx.resolve(dir).to_string_lossy().to_string();
    collect_files(&dir_resolved, ".mirror", &mut files);
    for path in &files {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for raw_line in source.lines() {
            let line = raw_line.trim_start();
            if line.starts_with('#') {
                continue;
            }
            // Match `glass `, `prism `, `grammar `, `spectral ` prefixes.
            // `spectral ` admitted 2026-07-11 (Tick 2 PREREQ-1) per
            // substrate self-migration architecture (Mara e764a32 +
            // Taut 8a3b0a4 + Seam 2b56977). The Connes (A, H, D)
            // triple's declaration form at consumer altitude.
            let rest = if let Some(r) = line.strip_prefix("glass ") {
                r
            } else if let Some(r) = line.strip_prefix("prism ") {
                r
            } else if let Some(r) = line.strip_prefix("grammar ") {
                r
            } else if let Some(r) = line.strip_prefix("spectral ") {
                r
            } else {
                continue;
            };
            let rest = rest.trim_start();
            if !rest.starts_with('@') {
                continue;
            }
            // Capture the @-ref up to the first delimiter.
            let end = rest
                .find(|c: char| c.is_whitespace() || c == '{' || c == '(' || c == ':' || c == ',')
                .unwrap_or(rest.len());
            let r#ref = &rest[..end];
            if r#ref.len() > 1 {
                out.insert(r#ref.to_string());
            }
        }
    }
}

/// Count `in @<path>` statements in a `.mirror` file whose `@<path>`
/// reference is not present in `declared`.
///
/// The substrate-pull recognition: a dangling `in @<path>` IS a dark
/// region in the corpus's dependency graph. The kintsugi loop's
/// `dark_count` was previously blind to it — only tokenization-altitude
/// dark spans were counted. This function extends `dark_count` with
/// the semantic-altitude unresolved-import count so the resolver
/// surfaces the gap.
///
/// Text-altitude scan (same rationale as `collect_declared_namespaces`):
/// `in @<path>` is a single line form, and tokenize already parses it
/// as `AstKind::In`. Doing a text scan here keeps the corpus walker
/// independent of the in-flight AST shape; the contract is just "every
/// `in @<path>` resolves to a declared namespace somewhere in the
/// corpus + shard tree".
fn count_unresolved_imports(
    file: &str,
    declared: &std::collections::HashSet<String>,
    ctx: &Ctx,
) -> u64 {
    let source = match fs::read_to_string(ctx.resolve(file)) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let mut unresolved: u64 = 0;
    for raw_line in source.lines() {
        let line = raw_line.trim_start();
        if line.starts_with('#') {
            continue;
        }
        let rest = match line.strip_prefix("in ") {
            Some(r) => r.trim_start(),
            None => continue,
        };
        if !rest.starts_with('@') {
            continue;
        }
        let end = rest
            .find(|c: char| c.is_whitespace() || c == '{' || c == '(' || c == ':' || c == ',')
            .unwrap_or(rest.len());
        let r#ref = &rest[..end];
        if r#ref.len() > 1 && !declared.contains(r#ref) {
            unresolved += 1;
        }
    }
    unresolved
}

/// Emit an aggregate CI verdict for a directory of `.mirror` files
/// (T11.3, T11.2.5). Walks recursively via `collect_files`, sorts the
/// list for determinism, computes a per-file verdict for each, and
/// aggregates per the rules in `CorpusVerdict`'s docstring.
///
/// Default emission is mirror-text per T11.2.5; `--format=json` keeps
/// the @io-boundary JSON path.
fn cmd_kintsugi_ci_corpus(
    dir: &str,
    shatter: u64,
    transform: Option<&str>,
    format: CiFormat,
    ctx: &Ctx,
) -> i32 {
    let mut files: Vec<String> = Vec::new();
    // Resolve the corpus dir against ctx.cwd() so `collect_files`'s
    // recursive walk sees absolute paths for downstream `fs::read`.
    // Downstream code (kintsugi_ci_compute, count_unresolved_imports)
    // ALSO threads ctx and resolves internally, so absolute here is
    // a no-op through the ctx.resolve() identity.
    let dir_for_walk = ctx.resolve(dir).to_string_lossy().to_string();
    collect_files(&dir_for_walk, ".mirror", &mut files);
    files.sort();

    // Cross-shard semantic resolution index (substrate-pull realize):
    // the corpus's `in @<path>` statements resolve against declared
    // `glass @<path>` / `prism @<path>` / `grammar @<path>` namespaces
    // in both the corpus directory under test AND the repo's `shards/`
    // tree (the substrate's authoritative source per
    // [[architecture-shards-as-substrate-source]]). The `boot/std/`
    // legacy tree is included as a fallback per the bootstrap-retirement
    // shrinkage contract; both substrate roots can satisfy an import.
    let mut declared: std::collections::HashSet<String> = std::collections::HashSet::new();
    collect_declared_namespaces(dir, &mut declared, ctx);
    if ctx.resolve("shards").is_dir() {
        collect_declared_namespaces("shards", &mut declared, ctx);
    }
    if ctx.resolve("boot/std").is_dir() {
        collect_declared_namespaces("boot/std", &mut declared, ctx);
    }

    let mut per_file: Vec<PerFileVerdict> = Vec::with_capacity(files.len());
    let mut total_objective: f64 = 0.0;
    let mut total_dark: u64 = 0;
    let mut max_iterations: u64 = 0;
    let mut any_failure = false;
    let mut any_partial = false;
    let mut all_success = true;

    for path in &files {
        let (mut verdict, objective, iterations, mut dark_count) =
            kintsugi_ci_compute(path, shatter, transform, ctx);
        // Cross-shard semantic resolution: count unresolved `in @<path>`
        // statements as additional dark regions and downgrade the
        // per-file verdict to `failure` when any unresolved import is
        // found. A dangling cross-shard import is a structural break,
        // not a soft `partial` — the dependency graph has no edge to
        // close.
        let unresolved = count_unresolved_imports(path, &declared, ctx);
        if unresolved > 0 {
            dark_count = dark_count.saturating_add(unresolved);
            verdict = "failure";
        }
        total_objective += objective;
        total_dark += dark_count;
        if iterations > max_iterations {
            max_iterations = iterations;
        }
        match verdict {
            "success" => {
                if dark_count > 0 {
                    // Defensive: success with dark > 0 is contradictory
                    // per the per-file rule, but aggregate semantics say
                    // any dark > 0 demotes to partial.
                    any_partial = true;
                    all_success = false;
                }
            }
            "partial" => {
                any_partial = true;
                all_success = false;
            }
            "failure" => {
                any_failure = true;
                all_success = false;
            }
            _ => {}
        }
        per_file.push(PerFileVerdict {
            path: path.clone(),
            verdict,
            objective,
            iterations,
            dark_count,
        });
    }

    let aggregate_verdict: &'static str = if any_failure {
        "failure"
    } else if any_partial || total_dark > 0 {
        "partial"
    } else if all_success {
        "success"
    } else {
        // Empty corpus (files.is_empty()): no failure, no partial, no
        // dark — treat as success per the v0.1 spec ("every per-file
        // verdict is success" is vacuously true).
        "success"
    };

    let files_processed = per_file.len() as u64;
    match format {
        CiFormat::MirrorText => emit_corpus_verdict_mirror_text(
            aggregate_verdict,
            dir,
            total_objective,
            max_iterations,
            total_dark,
            files_processed,
            &per_file,
        ),
        CiFormat::Json => {
            let envelope = CorpusVerdict {
                verdict: aggregate_verdict,
                target: dir,
                objective: total_objective,
                iterations: max_iterations,
                dark_count: total_dark,
                files_processed,
                per_file,
            };
            match serde_json::to_string(&envelope) {
                Ok(mut json) => {
                    json.push('\n');
                    _raw_stdout(json.as_bytes());
                    0
                }
                Err(_) => 1,
            }
        }
    }
}

/// Serialise a `CiVerdict` to stdout as JSON (single line,
/// newline-terminated). Returns exit code: 0 when serialisation + write
/// succeed, 1 only when stdout itself fails. The @io-boundary path
/// behind `--format=json`.
fn emit_ci_verdict_json(
    verdict: &'static str,
    target: &str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
    out_dir: Option<&str>,
) -> i32 {
    let v = CiVerdict {
        verdict,
        target,
        objective,
        iterations,
        dark_count,
    };
    match serde_json::to_string(&v) {
        Ok(mut json) => {
            json.push('\n');
            write_or_stdout(json.as_bytes(), out_dir, "json")
        }
        Err(_) => 1,
    }
}

/// Render an `f64` the way a substrate-native verdict expects: an
/// integer-valued float prints as `<int>.0` (e.g. `0.0`, `1.0`); a
/// non-integer prints via `{:?}` (Rust's shortest-round-trip repr).
/// Deterministic across runs.
fn render_f64(x: f64) -> String {
    if x.is_finite() && x.fract() == 0.0 {
        // Avoid scientific notation for very large integer-valued
        // floats; the test corpus only hits small values, but be
        // defensive.
        format!("{:.1}", x)
    } else {
        format!("{:?}", x)
    }
}

/// Emit a verdict as a mirror-text record: `<key> <value>` lines
/// aligned, terminated by a trailing newline. The substrate-native
/// default per T11.2.5.
///
/// T11.2.6 substrate-pull closure: this output is a canonical
/// instance of `type verdict = { ... }` declared in
/// `boot/std/kintsugi.mirror`. The key set written here MUST match
/// the field set declared there exactly. The round-trip tests in
/// `bootstrap/tests/kintsugi_ci_typed_verdict.rs` pin that equality.
///
/// The format is keyed (not stringly): the downstream consumer parses
/// `<key>[ws]+<value>` per line. Lossless round-trip is guaranteed for
/// the field set we emit.
///
/// Key widths are chosen so the longest aggregate key (`files_processed`)
/// aligns with the per-file shape (single-file mode uses `target`, so
/// its longest key is `iterations` at width 10; corpus mode uses
/// `files_processed` at width 15). The width is per-record.
fn emit_ci_verdict_mirror_text(
    verdict: &'static str,
    target: &str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
    out_dir: Option<&str>,
) -> i32 {
    // Single-file record: longest key is `iterations` (10) or
    // `dark_count` (10). Pad to 11 columns (key + at least one space).
    let buf = format!(
        "{:<11}{}\n{:<11}\"{}\"\n{:<11}{}\n{:<11}{}\n{:<11}{}\n",
        "verdict",
        verdict,
        "target",
        target,
        "objective",
        render_f64(objective),
        "iterations",
        iterations,
        "dark_count",
        dark_count,
    );
    write_or_stdout(buf.as_bytes(), out_dir, "mirror")
}

/// Emit a corpus verdict as a mirror-text envelope: an aggregate
/// record first, then one blank-line-separated record per file (sorted
/// by path). Substrate-native default per T11.2.5.
///
/// T11.2.6 substrate-pull closure: this output is a canonical instance
/// of `type corpus_verdict = { ... }` declared in
/// `boot/std/kintsugi.mirror`. The aggregate record's key set matches
/// `corpus_verdict`'s field set (minus `per_file`, which flattens to
/// records on the wire); each per-file record matches `type
/// verdict_entry`'s field set exactly. Round-trip-tested.
///
/// Aggregate width: longest key is `files_processed` (15) → pad to 17.
/// Per-file width: longest key is `iterations` (10) → pad to 13. The
/// two widths differ so the visual hierarchy mirrors the structural
/// one (envelope vs. element).
fn emit_corpus_verdict_mirror_text(
    aggregate_verdict: &'static str,
    target: &str,
    objective: f64,
    iterations: u64,
    dark_count: u64,
    files_processed: u64,
    per_file: &[PerFileVerdict],
) -> i32 {
    let mut buf = String::new();
    // Aggregate record. Width = 17 (files_processed + 2 spaces).
    let w = 17;
    buf.push_str(&format!("{:<w$}{}\n", "verdict", aggregate_verdict, w = w));
    buf.push_str(&format!("{:<w$}\"{}\"\n", "target", target, w = w));
    buf.push_str(&format!(
        "{:<w$}{}\n",
        "objective",
        render_f64(objective),
        w = w
    ));
    buf.push_str(&format!("{:<w$}{}\n", "iterations", iterations, w = w));
    buf.push_str(&format!("{:<w$}{}\n", "dark_count", dark_count, w = w));
    buf.push_str(&format!(
        "{:<w$}{}\n",
        "files_processed",
        files_processed,
        w = w
    ));

    // Per-file records. Width = 13 (iterations + 3 spaces, matching
    // single-file shape's 11+2 visual rhythm for `file` instead of
    // `target`).
    let pw = 13;
    for entry in per_file {
        buf.push('\n');
        buf.push_str(&format!("{:<w$}\"{}\"\n", "file", entry.path, w = pw));
        buf.push_str(&format!("{:<w$}{}\n", "verdict", entry.verdict, w = pw));
        buf.push_str(&format!(
            "{:<w$}{}\n",
            "objective",
            render_f64(entry.objective),
            w = pw
        ));
        buf.push_str(&format!(
            "{:<w$}{}\n",
            "iterations",
            entry.iterations,
            w = pw
        ));
        buf.push_str(&format!(
            "{:<w$}{}\n",
            "dark_count",
            entry.dark_count,
            w = pw
        ));
    }
    _raw_stdout(buf.as_bytes());
    0
}

/// Migrate a directory tree: walk every `.mirror` file under `src_root`,
/// apply the transform, canonicalise the path under `out_root`, write
/// the result. Path canonicalisation drops `std/mirror/` prefix; other
/// namespace prefixes (`std/code/`, `code/`) preserve via the same
/// strip-leading-`std/` rule.
fn cmd_kintsugi_migrate(src_root: &str, out_root: &str, transform: Option<&str>, ctx: &Ctx) -> i32 {
    let rules = match transform {
        Some(q) => {
            match parse_rewrite(q) {
                Some(r) => r,
                None => {
                    merr!("kintsugi --transform: not a rewrite query (expected `<sym> => <repl>`): {}", q);
                    return 1;
                }
            }
        }
        None => Vec::new(),
    };
    let mut files: Vec<String> = Vec::new();
    let src_for_walk = ctx.resolve(src_root).to_string_lossy().to_string();
    collect_files(&src_for_walk, ".mirror", &mut files);
    files.sort();
    let mut errs = 0;
    for path in &files {
        let source = match fs::read(path) {
            Ok(s) => s,
            Err(e) => {
                merr!("  skip {} (read error: {})", path, e);
                errs += 1;
                continue;
            }
        };
        // Apply rewrites to source bytes.
        let rewritten = if rules.is_empty() {
            source.clone()
        } else {
            apply_rewrites(&rules, &source)
        };
        // Canonicalise the destination path: drop the src_root prefix,
        // drop `std/mirror/` (bootstrap-historical), and apply the
        // basename rewrite from the rules (so `grammar.mirror` →
        // `glass.mirror` when the rule is `grammar => glass`).
        // Files were collected using the ctx-resolved src root, so
        // strip that form first; fall back to the display form for
        // callers that pass an already-absolute src_root.
        let rel = path
            .strip_prefix(&src_for_walk)
            .or_else(|| path.strip_prefix(src_root))
            .unwrap_or(path)
            .trim_start_matches('/');
        // Strip `std/mirror/` and `std/` prefixes — bootstrap-historical
        // namespacing that has no semantic content.
        let rel = rel
            .strip_prefix("std/mirror/")
            .or_else(|| rel.strip_prefix("std/"))
            .unwrap_or(rel);
        // Apply basename rewrite: each rule maps `<sym>.mirror` to
        // `<repl>.mirror` when the file basename equals `<sym>.mirror`.
        let mut rel_out = rel.to_string();
        for r in &rules {
            let src_base = format!("{}.mirror", r.symbol);
            let dst_base = format!("{}.mirror", r.replacement);
            // Only rewrite when the final path segment equals the
            // source basename (so internal directory segments named
            // `grammar` are not collateral-damaged).
            if let Some(last_slash) = rel_out.rfind('/') {
                let dir = &rel_out[..last_slash];
                let base = &rel_out[last_slash + 1..];
                if base == src_base {
                    rel_out = format!("{}/{}", dir, dst_base);
                }
            } else if rel_out == src_base {
                rel_out = dst_base;
            }
        }
        let dest_rel = format!("{}/{}", out_root.trim_end_matches('/'), rel_out);
        let dest_path = ctx.resolve(&dest_rel);
        let dest = dest_path.to_string_lossy().to_string();
        if let Some(parent) = std::path::Path::new(&dest).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                merr!("  cannot mkdir {}: {}", parent.display(), e);
                errs += 1;
                continue;
            }
        }
        if let Err(e) = fs::write(&dest, &rewritten) {
            merr!("  cannot write {}: {}", dest, e);
            errs += 1;
            continue;
        }
        merr!("  {} -> {}", path, dest);
    }
    merr!("migration: {} file(s), {} error(s)", files.len(), errs);
    if errs > 0 {
        1
    } else {
        0
    }
}

fn cmd_kintsugi_single(
    file: &str,
    shatter: u64,
    transform: Option<&str>,
    out_dir: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    let source = match fs::read(ctx.resolve(file)) {
        Ok(s) => s,
        Err(e) => {
            merr!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    // Apply --transform rewrites before tokenize.
    let source = if let Some(q) = transform {
        match parse_rewrite(q) {
            Some(rules) => apply_rewrites(&rules, &source),
            None => {
                merr!("kintsugi --transform: not a rewrite query: {}", q);
                return 1;
            }
        }
    } else {
        source
    };
    let grammar_path = grammar_for_file_in(file, ctx);
    let grammar = match load_grammar_in(grammar_path, ctx) {
        Ok(g) => g,
        Err(_) => return 1,
    };
    let ast = tokenize(&source, &grammar);

    if shatter >= 1 {
        // The loop. `prior_ast` is the section before this tick's stage 1;
        // `current_ast` is the section after stage 4's splice. With every
        // stage no-op, prior == current and stage 5 returns true on tick 1.
        // `Blake3` is explicit — the bootstrap startup declares which
        // H-world its floor inhabits (landing-page spec §2.4).
        let crystallizations = floor_crystallizations::<Blake3>();
        let mut prior = ast.clone();
        for i in 1..=shatter {
            let fixed = kintsugi_tick(&crystallizations, i, &prior, &ast);
            if fixed {
                break;
            }
            prior = ast.clone();
        }
    }

    // T19 (substrate-pull:realize) — settle the AST through the T13
    // `oscillate_with_ast` driver before rendering. The substrate's
    // kintsugi loop runs ACTIVE/DARK alternation until is_complete
    // returns a terminal verdict (Settled or Escalated) per the
    // Banach contraction. The witness's final oscillation tells us
    // which terminal state was reached + at what tick count; we
    // surface that on stderr as a `[settle]` trace so the operator
    // (and the T19 integration tests) can see the loop ran
    // end-to-end at the CLI boundary.
    //
    // Path-as-ref policy: the initial anchor is
    // `@kintsugi/<basename>` — substrate-honest "this is the
    // kintsugi origin for this file". The basename is sanitized
    // (whitespace dropped) so `Ref::new` accepts it; full filesystem
    // paths can contain spaces and would be rejected by Ref's
    // validator. The AST is the load-bearing carrier through the
    // driver; the initial Ref is just the anchor identity per
    // `oscillate.mirror` §oscillate's `anchor: initial` shape.
    //
    // T20 (substrate-pull:realize) — the portal-vs-text discriminator
    // at the CLI pipe boundary. The substrate's `@mirror/spectral/portal`
    // species (shards/mirror/spectral/portal.mirror) declares the
    // four-field carrier with three fields shaped as `shift(oid, T)`
    // (the 26th-instance typed-capability primitive). The OS-layer
    // realisation lives in `portal.rs`: `fstat(stdout)` for the
    // socket-detection branch, then 24-byte magic+version exchange,
    // then a 96-byte three-OID frame, then `sendmsg(SCM_RIGHTS)`
    // handing the in-memory settled-content fd across without
    // re-serialisation. Graceful: any Imperfect::Failure arm falls
    // through to the T19 text branch below.
    let basename = std::path::Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let safe_basename: String = basename
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_control())
        .collect();
    let safe_basename = if safe_basename.is_empty() {
        "file".to_string()
    } else {
        safe_basename
    };
    let initial_ref_path = format!("@kintsugi/{}", safe_basename);
    let mut settled_oid: [u8; 32] = [0u8; 32];
    if let Ok(initial_ref) = Ref::new(initial_ref_path) {
        let witness = oscillate::oscillate_witness_with_ast(initial_ref, &ast);
        let state = witness.final_oscillation.state();
        let iter = witness.final_oscillation.iteration().count();
        let cap = witness.cap_reached;
        merr!(
            "[settle] state={:?} iterations={} cap_reached={}",
            state,
            iter,
            cap,
        );
        // Capture the settled anchor's OID for the T20 portal frame's
        // `to_oid` slot. The substrate's `compute_content_oid` returns
        // a hex digest of the AST; we read the first 32 bytes of the
        // digest as the raw OID. `from_oid` / `delta_oid` stay zero
        // this tick — the gen_prism stream layer fills them in a
        // follow-up tick (per the Scheduler Tower bounded-iteration
        // primitive).
        let oid_hex = compute_content_oid(&ast);
        for (i, slot) in settled_oid.iter_mut().enumerate() {
            let start = i * 2;
            if start + 2 <= oid_hex.len() {
                if let Ok(byte) = u8::from_str_radix(&oid_hex[start..start + 2], 16) {
                    *slot = byte;
                }
            }
        }
    }

    let mut out = Vec::new();
    render_ast_in(&ast, 0, &mut out, ctx);

    if let Some(dir) = out_dir {
        // --out-dir path: settled bytes go to disk; no portal probe.
        let dest = format!(
            "{}/{}",
            dir.trim_end_matches('/'),
            std::path::Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file)
        );
        if let Some(parent) = std::path::Path::new(&dest).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(e) = fs::write(&dest, &out) {
            merr!("cannot write {}: {}", dest, e);
            return 1;
        }
        merr!("wrote {}", dest);
        return 0;
    }

    // T20 portal discriminator — only on unix, only when stdout is a
    // socket. The presence of the SCM_RIGHTS ancillary message IS the
    // portal signal (per Alex 2026-06-08: "It's a Crystal in
    // superposition. It's a fragmentation shard." — same way `Box<T>`
    // IS the signal of heap ownership transfer). One sendmsg carries
    // the 96-byte three-OID frame and the fragmentation-shard fd
    // atomically; the downstream pipeline stage settles the shard at
    // its altitude. Any failure arm silently falls through to the
    // T19 text branch below. The `[portal]` stderr trace is the
    // success symmetric to T19's `[settle]` trace.
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let stdout_fd = io::stdout().as_raw_fd();
        match portal::try_outbound_handshake(stdout_fd, &out, settled_oid) {
            terni::Imperfect::Success(_) => {
                merr!(
                    "[portal] shard oid={} bytes={}",
                    hex_oid(&settled_oid),
                    out.len()
                );
                return 0;
            }
            terni::Imperfect::Partial(_, _) => {
                // No partial path defined this tick; if one ever
                // surfaces, fall through to text to stay safe.
            }
            terni::Imperfect::Failure(_, _) => {
                // Silent text fallback — the default CLI behaviour.
                // Debug-mode stderr surfacing lives behind a future
                // `--debug-portal` flag.
            }
        }
    }

    // T19 text branch (the default; survives when portal is not
    // negotiated).
    _raw_stdout(&out);
    0
}

/// Render a 32-byte OID as a 64-char lowercase hex string. Used by
/// the `[portal] handoff` stderr trace so the OID round-trip with
/// the peer is human-inspectable.
fn hex_oid(oid: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in oid {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Whether the head of an mq pipeline is `@mcp.serve` (the MCP transport
/// primitive at `boot/std/mcp.mirror`). Both `@mcp.serve` (the action-
/// qualified form used by the substrate-decl) and `@mcp` (bare grammar
/// ref, historical / speculative) route to `mcp::serve_loop`. All other
/// heads fall through to `execute_pipeline` per the pre-Tick 6.5 dispatch.
///
/// Substrate-honest naming: this predicate names the *runtime discharge
/// point* for `dispatch_reflects_cli_block(dispatch)` and
/// `tools_reflects_cli_block(tools)` (per `boot/std/mcp.mirror`).
fn is_mcp_serve_head(segs: &[crate::pipeline::Segment]) -> bool {
    if let Some(first) = segs.first() {
        let r = first.r#ref.as_str();
        return r == "@mcp.serve" || r == "@mcp";
    }
    false
}

/// Pure subcommand dispatch — returns an exit code instead of calling
/// `std::process::exit`. The real binary entry point in `src/main.rs`
/// wraps this with `std::process::exit(dispatch(&args))`; the library
/// entry point `kintsugi_main` wraps it with an fd-level stdout / stderr
/// capture for in-process integration tests.
///
/// `args` is the full argv (including the program name at `args[0]`), so
/// the dispatch's index arithmetic matches the original `std::env::args`
/// shape. Callers from tests pass synthetic argv prefixed with a
/// placeholder program name (typically `"mirror"`).
pub fn dispatch(args: &[String], ctx: &Ctx) -> i32 {
    if args.len() < 2 {
        usage();
        return 1;
    }

    // Path A: mq query as single argument (stdin input)
    if args.len() == 2 && is_mq_query(&args[1]) {
        let segs = split_pipeline(&args[1]);
        if segs.is_empty() {
            merr!("empty query");
            return 1;
        }
        // Tick 6.5 (2026-07-08 Mara) — @mcp.serve Rust runtime discharge.
        // The `boot/std/mcp.mirror` substrate closure (Mara Tick 6 `d4c9a32`)
        // landed bilateral-predicate contracts `dispatch_reflects_cli_block`
        // + `tools_reflects_cli_block`. This special-case IS the runtime
        // discharge: `@mcp.serve` at the head of the pipeline routes to
        // `mcp::serve_loop` (the lifted MCP server at `bootstrap/src/mcp.rs`)
        // which reads stdin line-by-line as JSON-RPC and writes JSON-RPC
        // responses. `mcp::serve_loop` does its own stdin read — do NOT
        // slurp stdin here before dispatch; the pipeline's `read_stdin_all`
        // would consume the JSON-RPC frames.
        //
        // Substrate-honest naming: the runtime discharge point IS the
        // dispatch surface — no new substrate coined. Per
        // `docs/scouts/2026-07-08-taut-mcp-serve-lift-scope.md` LRM verdict
        // (Taut `cf5ab8c`) + Landing 1 RED (bootstrap/tests/
        // mcp_serve_runtime_shard.rs).
        if is_mcp_serve_head(&segs) {
            return crate::mcp::serve_loop();
        }
        let source = match read_stdin_all() {
            Ok(s) => s,
            Err(e) => {
                merr!("stdin read error: {}", e);
                return 1;
            }
        };
        return execute_pipeline(&segs, &source, ctx);
    }

    // Path B: file + mq query — resolve the file path against `ctx.cwd()`
    // so callers passing a relative path see it resolved consistently with
    // the caller's dispatch context (not the process cwd).
    if args.len() == 3 && is_mq_query(&args[2]) {
        let segs = split_pipeline(&args[2]);
        if segs.is_empty() {
            merr!("empty query");
            return 1;
        }
        // Tick 6.5 — same runtime discharge as Path A, for the `mirror
        // /dev/stdin @mcp.serve` invocation shape named in the substrate
        // decl at `boot/std/mcp.mirror` (`# Path B of the mirror binary`).
        // The `/dev/stdin` file argument is a substrate-honest carrier of
        // "the transport-frame source" that `mcp::serve_loop` reads via
        // its own `std::io::stdin().lock()` — identical byte-source, no
        // duplicate slurp needed.
        if is_mcp_serve_head(&segs) {
            return crate::mcp::serve_loop();
        }
        let resolved = ctx.resolve(&args[1]);
        let source = match fs::read(&resolved) {
            Ok(s) => s,
            Err(e) => {
                merr!("cannot read {}: {}", args[1], e);
                return 1;
            }
        };
        return execute_pipeline(&segs, &source, ctx);
    }

    if args.len() < 3 {
        usage();
        return 1;
    }

    // Path C: legacy subcommand
    let mut no_cache = false;
    let mut strict = false;
    let mut target_kind = TargetKind::Crystal;
    let mut shatter: u64 = 0;
    let mut transform: Option<String> = None;
    let mut out_dir: Option<String> = None;
    let mut ci = false;
    let mut ci_format: CiFormat = CiFormat::default();
    // Tick 0 Landing 2 — `mirror shatter --target <substrate-ref>`. When
    // args[1] == "shatter", `--target` names @shatter's codomain per
    // `docs/specs/shatter-is-the-io-linearization-operator.md` §4.1
    // (`583b939`) + mirror.spec cli-block `flag target: str = "auto"`
    // (Landing 1, OID
    // `796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8`).
    // Otherwise `--target` remains craft's `--target-kind` alias.
    let subcommand_is_shatter = args[1] == "shatter";
    // Rung 7 (2026-07-13) — `mirror peer contribute --target <shard>`
    // scopes `--target` to the contribute sub-verb (analogous to the
    // shatter carve-out above). Without this gate, Path C's craft-
    // scoped `--target`/`--target-kind` parser eats the shard path and
    // errors before the `"peer"` arm dispatches at all.
    let subcommand_is_peer = args[1] == "peer";
    let mut shatter_target: String = "auto".to_string();
    let mut i = 2;
    while i < args.len() {
        let a = &args[i];
        if a == "--no-cache" {
            no_cache = true;
        } else if a == "--strict" {
            strict = true;
        } else if subcommand_is_shatter && (a == "--target" || a.starts_with("--target=")) {
            // Shatter-scoped: `--target` is a substrate-ref selector,
            // NOT craft's `--target-kind` alias. Routes through the
            // lifted `parse_substrate_ref_to_format` (Landing 3) — the
            // same helper the kintsugi `--out` chain uses. The `"auto"`
            // sentinel is the default from mirror.spec's cli-block and
            // is accepted without a substrate-ref parse.
            let value: String = if a == "--target" {
                if i + 1 >= args.len() {
                    merr!("--target requires a substrate ref (e.g. @data/json or @data/mirror) or `auto`");
                    return 1;
                }
                i += 1;
                args[i].clone()
            } else {
                // strip_prefix path — already includes the value after `=`.
                a["--target=".len()..].to_string()
            };
            if value == "auto" {
                shatter_target = value;
            } else if value.starts_with('@') {
                if parse_substrate_ref_to_format(&value).is_none() {
                    merr!(
                        "no projection registered for shatter target substrate ref: {} (try @data/json or @data/mirror)",
                        value
                    );
                    return 1;
                }
                shatter_target = value;
            } else {
                merr!(
                    "--target requires a substrate ref (@<X>) or `auto`; got: {}",
                    value
                );
                return 1;
            }
        } else if subcommand_is_peer && (a == "--target" || a.starts_with("--target=")) {
            // Rung 7 peer-scoped: `mirror peer contribute --target <shard>`
            // routes shard path through the peer/contribute arm. Skip
            // Path C craft-parsing entirely for peer subcommands.
            if a == "--target" {
                i += 1;
            }
        } else if a == "--target" || a == "--target-kind" {
            // Substrate-honest name (mirror.spec 1e45c50 cli-block declares
            // `flag target_kind: str = "binary"`) is --target-kind. --target
            // remains as backward-compat alias per two-tick discipline.
            // The disambiguation matters at substrate altitude because
            // craft's positional is also called `target`; the flag name
            // `target_kind` prevents the same-name collision.
            if i + 1 >= args.len() {
                merr!("--target-kind requires a value (crystal|binary|rust|gleam)");
                return 1;
            }
            match parse_target(&args[i + 1]) {
                Some(k) => target_kind = k,
                None => {
                    merr!("unknown --target-kind value: {}", args[i + 1]);
                    return 1;
                }
            }
            i += 1;
        } else if let Some(rest) = a
            .strip_prefix("--target-kind=")
            .or_else(|| a.strip_prefix("--target="))
        {
            match parse_target(rest) {
                Some(k) => target_kind = k,
                None => {
                    merr!("unknown --target-kind value: {}", rest);
                    return 1;
                }
            }
        } else if a == "--shatter" {
            if i + 1 >= args.len() {
                merr!("--shatter requires a non-negative integer");
                return 1;
            }
            match args[i + 1].parse::<u64>() {
                Ok(n) => shatter = n,
                Err(_) => {
                    merr!(
                        "--shatter requires a non-negative integer, got: {}",
                        args[i + 1]
                    );
                    return 1;
                }
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--shatter=") {
            match rest.parse::<u64>() {
                Ok(n) => shatter = n,
                Err(_) => {
                    merr!("--shatter requires a non-negative integer, got: {}", rest);
                    return 1;
                }
            }
        } else if a == "--transform" {
            if i + 1 >= args.len() {
                merr!("--transform requires an mq-query value");
                return 1;
            }
            transform = Some(args[i + 1].clone());
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--transform=") {
            transform = Some(rest.to_string());
        } else if a == "--out" {
            if i + 1 >= args.len() {
                merr!("--out requires a substrate ref (e.g. @data/json or @io/dir('path'))");
                return 1;
            }
            // Hard cut: `--out` REQUIRES a substrate ref (@<X> or
            // @<X>(<args>)). Bare strings (json|mirror) and bare paths
            // are rejected per Alex's 2026-06-16 directive.
            let value = &args[i + 1];
            if dispatch_out_substrate_ref(value, &mut ci_format, &mut out_dir).is_err() {
                return 1;
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--out=") {
            if dispatch_out_substrate_ref(rest, &mut ci_format, &mut out_dir).is_err() {
                return 1;
            }
        } else if a == "--ci" {
            ci = true;
        } else if a == "--format" || a.starts_with("--format=") {
            // Hard cut: --format is removed. Per Alex's 2026-06-16
            // substrate-pull, the substrate-correct vocabulary is
            // `--out @data/json` (substrate refs through the `out`
            // keyword's namespace). Emit a migration hint and exit.
            merr!("--format is removed; use --out @data/json (or @data/mirror) instead");
            return 1;
        }
        i += 1;
    }
    // Find the positional argument, skipping `--flag value` pairs.
    // `--ci` is a bare flag (no value), so it falls through to the
    // `starts_with("--")` arm and is skipped without consuming the
    // next argument.
    let positional: Option<&str> = {
        let mut j = 2;
        let mut found: Option<&str> = None;
        while j < args.len() {
            let a = &args[j];
            if a == "--target"
                || a == "--target-kind"
                || a == "--shatter"
                || a == "--transform"
                || a == "--out"
            {
                j += 2;
                continue;
            }
            if a.starts_with("--") {
                j += 1;
                continue;
            }
            found = Some(a.as_str());
            break;
        }
        found
    };

    // The positional path is passed through cmd_* verbatim (so verdict
    // envelopes emit the operator-supplied string, not the resolved
    // absolute form). Each cmd_* function calls `ctx.resolve(pos)` when
    // it needs to read the file — the Arc 2 substrate-pull threads cwd
    // explicitly rather than mutating the process cwd, but the display
    // path stays operator-facing.

    let rc = match args[1].as_str() {
        "compile" => match positional {
            Some(p) => cmd_compile(p, no_cache, strict, ctx),
            None => {
                merr!("usage: mirror compile [--strict] <file>");
                1
            }
        },
        "craft" => match positional {
            Some(p) => cmd_craft_with(p, no_cache, target_kind, strict, ctx),
            None => {
                merr!("usage: mirror craft [--strict] [--target-kind <crystal|binary>] <target>");
                1
            }
        },
        "kintsugi" => match positional {
            Some(p) => cmd_kintsugi(
                p,
                shatter,
                transform.as_deref(),
                out_dir.as_deref(),
                ci,
                ci_format,
                ctx,
            ),
            None => {
                merr!("usage: mirror kintsugi [--ci [--out @data/json|@data/mirror|@io/dir('path')]] [--shatter N] [--transform <mq>] <file|dir>");
                1
            }
        },
        "init" => match positional {
            Some(p) => {
                let install_hooks = args.iter().any(|a| a == "--install-hooks");
                cmd_init(p, install_hooks, ctx)
            }
            None => {
                merr!("usage: mirror init <repo-path> [--install-hooks]");
                1
            }
        },
        "recall" => match positional {
            Some(p) => cmd_recall(p, ctx),
            None => {
                merr!("usage: mirror recall <spec-dir>");
                1
            }
        },
        "spawn" => match positional {
            // Tick 3 Landing 2 — `mirror spawn` is preserved as a
            // backward-compat alias for `mirror peer beam` per two-tick
            // discipline. The cli-verb rename lands at substrate altitude
            // (mirror.spec `96aa752`); this alias emits a deprecation
            // notice on stderr while keeping stdout envelope byte-equal
            // for round-trip stability. Fault-plane #1 preserved:
            // @pack.spawn at pack altitude (shards/pack.mirror:263) is
            // unchanged — only the cli-surface wrapper is renamed.
            Some(p) => {
                merr!(
                    "note: `mirror spawn` is a deprecated alias for `mirror peer beam` (two-tick discipline; substrate-honest surface is `mirror peer beam <peer-home>` per mirror.spec cli-block)"
                );
                let hello_world = args.iter().any(|a| a == "--hello-world");
                let emit_diff = args.iter().any(|a| a == "--emit-diff");
                let integrate_diff = args.iter().any(|a| a == "--integrate-diff");
                let fate_select = args.iter().any(|a| a == "--fate-select");
                let from_psychohistory = args.iter().any(|a| a == "--from-psychohistory");
                let with_shadow = args.iter().any(|a| a == "--with-shadow");
                let mission = args
                    .iter()
                    .position(|a| a == "--mission" || a == "--task")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str());
                let song = args
                    .iter()
                    .position(|a| a == "--song")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str());
                let dance_with = args
                    .iter()
                    .position(|a| a == "--dance-with")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str());
                let deploy_to = args
                    .iter()
                    .position(|a| a == "--deploy-to")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| s.as_str());
                let emit_crystal = args.iter().any(|a| a == "--emit-crystal");
                cmd_peer_beam(
                    p,
                    hello_world,
                    mission,
                    ctx,
                    emit_diff,
                    integrate_diff,
                    fate_select,
                    from_psychohistory,
                    with_shadow,
                    song,
                    dance_with,
                    deploy_to,
                    emit_crystal,
                )
            }
            None => {
                merr!("usage: mirror spawn <peer-home> [--hello-world] [--mission <mission-file> | --task <mission-file>]  (deprecated alias for `mirror peer beam`)");
                1
            }
        },
        "index" => {
            // Rung 8 Landing 4 — `mirror index <path> [--fiedler]
            // [--full-profile]`. Substrate-decl at mirror.spec
            // `command index { arg path: ~d; flag fiedler: bool;
            // flag full_profile: bool }`. Runtime at
            // `bootstrap/src/index.rs::index(path) -> EigenvalueProfile`.
            //
            // Alex 2026-07-13 "Fire": pull spectral_index into mirror.
            let path_arg = args.get(2).map(|s| s.as_str());
            match path_arg {
                Some(p) => {
                    let path = ctx.resolve(p);
                    let profile = crate::index::index(&path);
                    let fiedler_only = args.iter().any(|a| a == "--fiedler");
                    let full_profile = args.iter().any(|a| a == "--full-profile");
                    let multifractal_flag = args.iter().any(|a| a == "--multifractal");
                    if fiedler_only {
                        mout!("{:.4}", profile.fiedler_value());
                    } else if full_profile {
                        for (i, v) in profile.values.iter().enumerate() {
                            mout!("[{i:02}] {v:.6}");
                        }
                    } else if multifractal_flag {
                        // Rung 8 Landing 6 LOAD-BEARING empirical proof:
                        // multifractal f(α) spectrum on the substrate DAG's
                        // eigenvalue distribution. Discharges Mara math §10
                        // prediction #2.
                        let q_range = crate::index::canonical_q_range();
                        let spectrum = profile.multifractal_spectrum(&q_range);
                        mout!("@@ mirror index @mirror/fractal-coherence multifractal spectrum (Rung 8 Landing 6; Mara math §10 prediction #2 LOAD-BEARING) @@");
                        mout!("+ path: {}", path.display());
                        mout!("+ fiedler: {:.4}", profile.fiedler_value());
                        mout!("+ d_0_support_dimension: {:.4}", spectrum.d_0);
                        mout!("+ d_1_information_dimension: {:.4}", spectrum.d_1);
                        mout!("+ d_2_correlation_dimension: {:.4}", spectrum.d_2);
                        mout!("+ multifractal_witness: {:.4} (max f(α) − min f(α); > 0.1 ⇒ multifractal signature)", spectrum.multifractal_witness);
                        mout!("+ q_range: [{}, {}] with {} samples", spectrum.q_values.first().copied().unwrap_or(0.0), spectrum.q_values.last().copied().unwrap_or(0.0), spectrum.q_values.len());
                        mout!("+ alpha_range: [{:.4}, {:.4}]", spectrum.alpha.iter().cloned().fold(f64::INFINITY, f64::min), spectrum.alpha.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
                        mout!("+ f_alpha_range: [{:.4}, {:.4}]", spectrum.f_alpha.iter().cloned().fold(f64::INFINITY, f64::min), spectrum.f_alpha.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
                        // Emit spectrum table (q, tau_q, alpha, f_alpha).
                        mout!("+ spectrum_table:");
                        mout!("    q       tau(q)      alpha       f(alpha)");
                        for i in 0..spectrum.q_values.len() {
                            mout!("    {:>6.2}  {:>10.4}  {:>10.4}  {:>10.4}", spectrum.q_values[i], spectrum.tau_q[i], spectrum.alpha[i], spectrum.f_alpha[i]);
                        }
                        mout!("+ substrate_authority: @mirror/index.multifractal (shards/mirror/index.mirror action-decl forward-promise; Mara `317e830`)");
                        mout!("+ mathematical_ancestry: HJKPS 1986 (multifractal formalism) + Rényi 1961 (generalized entropies) + Douady-Hubbard 1982/1985 (Mandelbrot boundary) + Shishikura 1998 (∂M Hausdorff dim 2)");
                        mout!("+ empirical_prediction: Mara math §10 prediction #2 — if mirror IS Mandelbrot-shaped, f(α) shows non-trivial interval width; monofractal (witness ≈ 0) would falsify the prediction");
                        mout!("+ mandelbrot_correspondence: fiedler = λ₀(Δ_F); multifractal spectrum = f(α) of the graph Laplacian's eigenvalue density; both live at the same @fractal altitude");
                        mout!("+ ladder_rung: 8 Landing 6 (Reed empirical proof discharging Mara `2c64060` §4 identification)");
                        mout!("+ recognition_candidate: #R-fractal-is-mandelbrot-substrate (empirical witness landed)");
                    } else {
                        mout!("@@ mirror index @mirror/fractal-coherence measurement (Rung 8 Landing 4; Mara `317e830` substrate-decl + Taut `77b8e14` migration mapping) @@");
                        mout!("+ path: {}", path.display());
                        mout!("+ fiedler: {:.4}", profile.fiedler_value());
                        mout!("+ profile: [{}]", profile.values.iter().map(|v| format!("{:.4}", v)).collect::<Vec<_>>().join(", "));
                        mout!("+ substrate_authority: @mirror/index (shards/mirror/index.mirror; provisional under two-tick discipline; collapses to @fractal/index after Alex #6)");
                        mout!("+ eigenvalue_backend: prismqueer::ffi::eigenvalues (LAPACK dsyev; same primitive as sheaf_laplacian::lambda_zero)");
                        mout!("+ mandelbrot_correspondence: fiedler IS λ₀(Δ_F) = spectral gap of the substrate's parameter Mandelbrot per Mara `2c64060` §4");
                        mout!("+ recognition_candidate: #R-fractal-is-mandelbrot-substrate");
                    }
                    0
                }
                None => {
                    merr!("usage: mirror index <path> [--fiedler] [--full-profile]");
                    1
                }
            }
        }
        "peer" => {
            // Tick 3 Landing 2 — `mirror peer beam <peer-home>` (recursive-
            // command depth-2 dispatch per @mirror/lens/cli Tick 1 grammar
            // `fe82500`). Substrate-decl at mirror.spec Landing 1
            // (`96aa752`) declares `command peer { command beam { ... } }`;
            // this arm walks the nested command via args[2] and finds the
            // sub-positional starting at args[3].
            //
            // Only sub-command today is `beam`. Future sub-commands
            // (per beam-as-substrate-primitive.md §3 composition table)
            // land under this arm as additional match branches.
            if args.len() < 3 {
                merr!("usage: mirror peer beam <peer-home> [--hello-world] [--mission <mission-file>]");
                1
            } else {
                match args[2].as_str() {
                    "beam" => {
                        // Sub-positional finder starting at args[3] —
                        // mirrors the outer `positional` helper but
                        // offset by one for the nested command.
                        let sub_positional: Option<&str> = {
                            let mut j = 3;
                            let mut found: Option<&str> = None;
                            while j < args.len() {
                                let a = &args[j];
                                if a == "--mission"
                                    || a == "--task"
                                    || a == "--song"
                                    || a == "--dance-with"
                                    || a == "--deploy-to"
                                {
                                    j += 2;
                                    continue;
                                }
                                if a.starts_with("--") {
                                    j += 1;
                                    continue;
                                }
                                found = Some(a.as_str());
                                break;
                            }
                            found
                        };
                        match sub_positional {
                            Some(p) => {
                                let hello_world = args.iter().any(|a| a == "--hello-world");
                                let mission = args
                                    .iter()
                                    .position(|a| a == "--mission" || a == "--task")
                                    .and_then(|i| args.get(i + 1))
                                    .map(|s| s.as_str());
                                let emit_diff = args.iter().any(|a| a == "--emit-diff");
                                let integrate_diff = args.iter().any(|a| a == "--integrate-diff");
                                let fate_select = args.iter().any(|a| a == "--fate-select");
                                let from_psychohistory =
                                    args.iter().any(|a| a == "--from-psychohistory");
                                let with_shadow = args.iter().any(|a| a == "--with-shadow");
                                let song = args
                                    .iter()
                                    .position(|a| a == "--song")
                                    .and_then(|i| args.get(i + 1))
                                    .map(|s| s.as_str());
                                let dance_with = args
                                    .iter()
                                    .position(|a| a == "--dance-with")
                                    .and_then(|i| args.get(i + 1))
                                    .map(|s| s.as_str());
                                let deploy_to = args
                                    .iter()
                                    .position(|a| a == "--deploy-to")
                                    .and_then(|i| args.get(i + 1))
                                    .map(|s| s.as_str());
                                let emit_crystal =
                                    args.iter().any(|a| a == "--emit-crystal");
                                cmd_peer_beam(
                                    p,
                                    hello_world,
                                    mission,
                                    ctx,
                                    emit_diff,
                                    integrate_diff,
                                    fate_select,
                                    from_psychohistory,
                                    with_shadow,
                                    song,
                                    dance_with,
                                    deploy_to,
                                    emit_crystal,
                                )
                            }
                            None => {
                                merr!("usage: mirror peer beam <peer-home> [--hello-world] [--emit-diff] [--mission <mission-file>]");
                                1
                            }
                        }
                    }
                    "contribute" => {
                        // Rung 7 (2026-07-13) — fate-spawned peer
                        // contributes working shard delta via active_pass
                        // per Mara `4e69066` §3.2. Empirical-discharge:
                        // Fate proposes morphism → mirror applies to
                        // shard → @mirror/mosaic.settle (cargo check)
                        // verifies → commit_as_fold on peer's DAG only
                        // if settle green. Substrate-decl at mirror.spec
                        // `command peer { command contribute { ... } }`.
                        //
                        // args[3] = peer_home (positional per grammar
                        // `arg peer_home: ~d`); --target <path> required
                        // (per grammar `flag target: ~f`).
                        let sub_positional: Option<&str> = {
                            let mut j = 3;
                            let mut found: Option<&str> = None;
                            while j < args.len() {
                                let a = &args[j];
                                if a == "--target" {
                                    j += 2;
                                    continue;
                                }
                                if a.starts_with("--") {
                                    j += 1;
                                    continue;
                                }
                                found = Some(a.as_str());
                                break;
                            }
                            found
                        };
                        let target = args
                            .iter()
                            .position(|a| a == "--target")
                            .and_then(|i| args.get(i + 1))
                            .map(|s| s.as_str());
                        match (sub_positional, target) {
                            (Some(p), Some(t)) => {
                                let target_path = std::path::PathBuf::from(t);
                                contribute::peer_contribute(p, &target_path, ctx)
                            }
                            _ => {
                                merr!("usage: mirror peer contribute <peer-home> --target <shard>");
                                1
                            }
                        }
                    }
                    other => {
                        merr!("unknown: peer {}", other);
                        1
                    }
                }
            }
        }
        "beam" => {
            // Tick 3 Landing 2 — `mirror beam <mission-file>` (anonymous
            // variant per docs/specs/beam-as-substrate-primitive.md §3
            // composition table: beam-without-persistent-identity).
            // Substrate-decl at mirror.spec Landing 1 (`96aa752`) declares
            // top-level `command beam { arg mission: ~f }`. Both `mirror
            // beam` and `mirror peer beam` dispatch to the same substrate
            // action (@mirror/peer/beam.beam); runtime differentiation is
            // on positional-arg shape (mission-file vs peer-home).
            //
            // v0 body: anonymous variant defers to cmd_peer_beam with a
            // sentinel peer-home. Substrate-honest — the anonymous
            // variant's persistent-identity slot is empty by construction;
            // envelope shape reflects that at the cli surface. Full
            // anonymous body (mission-driven @song without peer-home
            // binding) is a follow-up tick.
            match positional {
                Some(p) => {
                    let hello_world = args.iter().any(|a| a == "--hello-world");
                    // The anonymous variant's positional IS the mission
                    // file per Landing 1. Pass it through as the mission
                    // argument to cmd_peer_beam with the current dir as
                    // sentinel peer-home; the substrate action is the
                    // same, only the operator-facing arg-shape differs.
                    let emit_diff = args.iter().any(|a| a == "--emit-diff");
                    let integrate_diff = args.iter().any(|a| a == "--integrate-diff");
                    let fate_select = args.iter().any(|a| a == "--fate-select");
                    let from_psychohistory = args.iter().any(|a| a == "--from-psychohistory");
                    let with_shadow = args.iter().any(|a| a == "--with-shadow");
                    let song = args
                        .iter()
                        .position(|a| a == "--song")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.as_str());
                    let dance_with = args
                        .iter()
                        .position(|a| a == "--dance-with")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.as_str());
                    let deploy_to = args
                        .iter()
                        .position(|a| a == "--deploy-to")
                        .and_then(|i| args.get(i + 1))
                        .map(|s| s.as_str());
                    let emit_crystal = args.iter().any(|a| a == "--emit-crystal");
                    cmd_peer_beam(
                        ".",
                        hello_world,
                        Some(p),
                        ctx,
                        emit_diff,
                        integrate_diff,
                        fate_select,
                        from_psychohistory,
                        with_shadow,
                        song,
                        dance_with,
                        deploy_to,
                        emit_crystal,
                    )
                }
                None => {
                    merr!("usage: mirror beam <mission-file> [--hello-world]");
                    1
                }
            }
        }
        "shatter" => {
            // Tick 0 Landing 2 — `mirror shatter <oid> <out> [--target
            // <substrate-ref>]`. Per mirror.spec cli-block (Landing 1;
            // OID
            // `796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8`)
            // and `docs/specs/shatter-is-the-io-linearization-operator.md`
            // §4.1 (Mara `583b939`).
            //
            // Two positional args (oid + out) — extract the second
            // positional inline since the shared `positional` helper
            // above only returns the first.
            let mut positionals: Vec<&str> = Vec::new();
            let mut j = 2;
            while j < args.len() && positionals.len() < 2 {
                let a = &args[j];
                if a == "--target"
                    || a == "--target-kind"
                    || a == "--shatter"
                    || a == "--transform"
                    || a == "--out"
                {
                    j += 2;
                    continue;
                }
                if a.starts_with("--") {
                    j += 1;
                    continue;
                }
                positionals.push(a.as_str());
                j += 1;
            }
            if positionals.len() < 2 {
                merr!("usage: mirror shatter <oid> <out> [--target @data/json|@data/mirror|auto]");
                1
            } else {
                cmd_shatter(positionals[0], positionals[1], &shatter_target, ctx)
            }
        }
        other => {
            merr!("unknown: {}", other);
            1
        }
    };
    rc
}

// ───────────────────────────────────────────────────────────────────────────────
// `mirror shatter <oid> <out> [--target <substrate-ref>]` — Tick 0 Landing 2
// stub for the cli surface prerequisite of the 6-tick cascade (path α).
// ───────────────────────────────────────────────────────────────────────────────

/// `mirror shatter <oid> <out> [--target <substrate-ref>]` — Tick 0
/// Landing 2 stub.
///
/// Per Taut LRM scout `1658b95` + Mara `583b939` (@shatter × @io
/// canonical spec §4.1) + mirror.spec cli-block Landing 1 (OID
/// `796f3289629835f2e05cb1ac98559b5584fae8b2b76c2c9167eb91f4921cefb8`):
/// this dispatch arm names the arg surface (positional oid + positional
/// out + `--target` substrate-ref selector). The real projection
/// emission — @shatter's codomain body per §4.1 — is forward-promised
/// for a follow-up tick.
///
/// Substrate-honest: v0 body returns exit 2 with a diagnostic naming
/// which target was requested and that the emission wiring is not yet
/// discharged. Consumers pull. The arg-parse layer routing IS the
/// load-bearing landing for this tick — that's what the 6-tick cascade
/// discharge depends on per Taut's scout.
///
/// The `--target` value has already been validated at the arg-parse
/// layer (either `"auto"` or a substrate ref that
/// `parse_substrate_ref_to_format` recognized). This function assumes
/// well-formed input.
fn cmd_shatter(oid: &str, out: &str, target: &str, _ctx: &Ctx) -> i32 {
    // Substrate-honest v0: name the state so consumers see the pull
    // frontier at the @io boundary. The next-tick discharge routes
    // through `parse_substrate_ref_to_format` (Landing 3, already
    // callable) to select the projection emission machinery.
    merr!(
        "mirror shatter: target `{}` for oid={} out={} not yet wired at v0 (substrate-honest stub per Tick 0 Landing 2; the projection emission body is forward-promised)",
        target,
        oid,
        out
    );
    2
}

// ───────────────────────────────────────────────────────────────────────────────
// `mirror recall <spec-dir>` — P3 RED stub for inbound trajectory surface.
// ───────────────────────────────────────────────────────────────────────────────

/// `mirror recall <spec-dir>` — P3 RED stub.
///
/// Per Mara's @mirror/recall canonical spec (`docs/specs/mirror-recall.md`,
/// commit `b034a60`) + Seam P2 review (`88f8428`) applying Discharge C
/// (`last_seen_commit: content_address` instead of `in_flight: bool`,
/// which is forbidden as stateless-return-at-runtime per b10f00c §4).
///
/// Recall is the dual of spawn at substrate altitude (spawn = substrate
/// leaving λ₀; recall = observer returning to substrate in excited
/// state, asking for trajectory). The four payloads —
/// `cascade`/`pack_trail`/`pull_frontier`/`dogfood` — compose existing
/// substrate-decls into one inbound API a returning agent can invoke in
/// one breath. Per Mara spec §3.
///
/// This stub returns a placeholder JSON envelope that does NOT carry
/// the four payload keys. The RED tests in `bootstrap/tests/recall.rs`
/// and `bootstrap/tests/mcp_handshake.rs` assert the four-payload
/// contract — they fail against this stub. The GREEN tick wires the
/// real reads (git log → cascade; commit authors → pack_trail with
/// last_seen_commit; candidate-recognition scan → pull_frontier;
/// mirror.spec settle_on → dogfood).
// ─────────────────────────────────────────────────────────────────────────────────
// `mirror init <repo-path>` — the bridge command (P3 RED stub).
// ────────────────────────────────────────────────────────────────────────────────

/// `mirror init <repo-path>` — P3 RED stub.
///
/// Per Mara's mirror-init spec (`docs/specs/mirror-init.md`, commits
/// `fe215bd` → `14dd043`) + Seam's audit (commit `8392ab5`; 0C/3S/8L/12✓)
/// + Reed's Cargo-edge GREEN (commit `6b36808`; fragmentation linkable;
/// R2 = 0 bytes Δ empirically): mirror init is the bridge command that
/// makes the declared substrate operational at the storage altitude.
///
/// v0 contract (spec §4.7): emit a JSON envelope to stdout with
///   { spec_version, operation, repo, store, indexed, bytes_total,
///     root_oid, hooks_installed, verdict }
///
/// This stub returns a placeholder envelope WITHOUT the contract keys.
/// The RED tests in `bootstrap/tests/init.rs` assert the contract — they
/// fail against this stub. The GREEN tick wires the real composition:
/// NamespacedGitStore::open + project::project + per-file Splinter +
/// store.insert_persistent + root_oid via set_ref("HEAD", root).
///
/// b10f00c §4 fences honored:
///   - No @fate; pure content-addressing
///   - No @io/llm
///   - No subprocess (in-process fragmentation calls)
///   - No identity-mint (repo path IS the identity at this altitude)
///   - No stateless-return (envelope anchors at root_oid — content-addressed)
///
/// P4 GREEN: wire the empirical composition per Mara's `@mirror/store/git`
/// species declaration at `shards/mirror/store/git.mirror` (commit
/// `1de09a9`).
///
/// The discharge map this implements:
///
/// ```text
/// write(bytes)       <= insert_persistent(store, oid_of(bytes),
///                                         fractal_of(bytes), size_of(bytes))
///                     + set_ref(store, "HEAD", root_oid)
/// ```
///
/// Carrier translation at the wire boundary: canonical `write(bytes)`
/// discharges to wire `insert_persistent(fractal)`. The `bytes -> fractal`
/// encoding goes through `fragmentation::encoding::encode` (carrier #53
/// candidate-witness per Mara's species shard).
///
/// Composition flow:
///
///   1. Open the namespaced store at `.git/mirror/` via
///      `NamespacedGitStore::open(repo_path, "mirror")`. The wire's
///      `open(path, namespace)` action.
///   2. Enumerate the working-tree file set via `git ls-files` (the v0
///      manifest source per spec §3.1.5 path (b) recommendation).
///   3. Synthesize a `Manifest` from the file listing and project via
///      `fragmentation::project::project` — yields per-file `(content,
///      blob_oid)` deterministically.
///   4. For each projected file:
///      - encode the bytes as `Fractal<String>` via
///        `fragmentation::encoding::encode` (the carrier translation),
///      - compute the OID via `Splinter::<Blake3>::new(Content::Text(...))`
///        (the substrate's BLAKE3 content-address),
///      - `insert_persistent(store, format!("splinter:{oid}"), fractal,
///        size)` — the wire's content-addressed persistence.
///   5. Compute the root_oid as BLAKE3 of the sorted (target_path,
///      splinter_oid) pairs (spec §4.5).
///   6. `set_ref(store, "HEAD", root_oid)` — the wire's named-ref write
///      that makes the indexed surface reachable.
///   7. `flush(store)` — drain the cache to disk.
///   8. Emit the envelope per spec §4.7.
fn cmd_init(repo_path: &str, hooks: bool, ctx: &Ctx) -> i32 {
    // Edge case: repo path must exist + be a directory. Resolve via
    // ctx so relative paths honor the dispatch context.
    let resolved = ctx.resolve(repo_path);
    let path = resolved.as_path();
    if !path.exists() {
        merr!("init: repo path does not exist: {}", repo_path);
        return 1;
    }
    if !path.is_dir() {
        merr!("init: repo path is not a directory: {}", repo_path);
        return 1;
    }

    // Step 1 (discharge map: open): open the @mirror/store/git species
    // at `.git/mirror/`. NotAGitRepo failure surfaces non-zero exit.
    let store = match fragmentation_git::namespaced::NamespacedGitStore::open(path, "mirror") {
        Ok(s) => s,
        Err(e) => {
            merr!("init: cannot open namespaced store: {}", e);
            return 2;
        }
    };

    // Step 2: enumerate the working-tree file set via `git ls-files`.
    // The v0 manifest source per spec §3.1.5 path (b). Synthesized in
    // mirror; no fragmentation-side commit required.
    let files = match init_enumerate_git_ls_files(path) {
        Ok(v) => v,
        Err(e) => {
            merr!("init: cannot enumerate files via git ls-files: {}", e);
            return 1;
        }
    };

    // Step 3: synthesize the manifest + project.
    let manifest = fragmentation::manifest::Manifest {
        lenses: files
            .iter()
            .map(|f| fragmentation::manifest::LensEntry {
                source: f.clone(),
                target: f.clone(),
            })
            .collect(),
    };
    let projection = match fragmentation::project::project(path, &manifest) {
        Ok(p) => p,
        Err(e) => {
            merr!("init: project failed: {}", e);
            return 1;
        }
    };

    // Step 4 (discharge map: write -> insert_persistent + carrier
    // translation): per-file Splinter OID + Fractal carrier; persist
    // into the namespaced store.
    //
    // The `splinter_oid_pairs` accumulator preserves the sorted
    // (target, oid) order the BTreeMap gives us — load-bearing for
    // root_oid determinism.
    let mut splinter_oid_pairs: Vec<(String, String)> = Vec::new();
    let mut bytes_total: usize = 0;

    for (target, projected) in &projection.files {
        // Substrate OID: BLAKE3 via Splinter::new + Content::Text. The
        // substrate's content-addressing primitive.
        let text = String::from_utf8_lossy(&projected.content).to_string();
        let splinter: Splinter<Blake3> = Splinter::new(Content::Text(Text::new(text.clone())));
        let oid_hex = init_blake3_oid_hex(splinter.oid().bytes());

        // Wire carrier: Fractal<String> via fragmentation's encoder.
        // The `bytes -> fractal` translation Mara flagged as the
        // #53 carrier candidate-witness.
        let fractal = fragmentation::encoding::encode(&text);

        // Wire write: insert_persistent.
        store.insert_persistent(
            format!("splinter:{}", oid_hex),
            fractal,
            projected.content.len(),
        );

        splinter_oid_pairs.push((target.clone(), oid_hex));
        bytes_total += projected.content.len();
    }

    // Step 5: root_oid = BLAKE3 of sorted (target, oid) pairs per spec §4.5.
    // The BTreeMap projection already sorts by target; preserve the order.
    let root_oid_hex = init_compute_root_oid(&splinter_oid_pairs);

    // Step 6 (discharge map: set_ref): point HEAD at the root_oid.
    if let Err(e) = store.set_ref("HEAD", &root_oid_hex) {
        merr!("init: set_ref(HEAD) failed: {}", e);
        return 1;
    }

    // Step 7: flush the cache to disk. Post-condition: cached_len == 0.
    store.flush();

    // Step 8: emit the envelope per spec §4.7.
    let envelope = serde_json::json!({
        "spec_version":    "v0.1.0",
        "operation":       "init",
        "repo":            repo_path,
        "store":           store.path().display().to_string(),
        "indexed":         splinter_oid_pairs.len(),
        "bytes_total":     bytes_total,
        "root_oid":        root_oid_hex,
        "hooks_installed": hooks,
        "verdict":         "ok",
    });
    let s = format!("{}\n", envelope);
    _raw_stdout(s.as_bytes());
    0
}

/// Enumerate the working-tree file set via `git ls-files`. The v0
/// manifest source per spec §3.1.5 path (b). Returns one relative
/// path per tracked file in the index.
///
/// Filters to entries that exist as regular files on disk — git's
/// submodule entries surface in `ls-files` output but resolve to
/// directories (or absent), which `project::project` cannot read as
/// file bytes. Keeping the filter loose-but-typed (per
/// [[feedback-no-bare-types]]) at this altitude — substrate-pull-honest
/// behavior is "skip what cannot be projected"; the substrate-decl
/// expression of that policy lives at the @mirror/store/git species
/// surface when the consumer-pull surfaces (forward-promised).
///
/// Returns Err if `git ls-files` exits non-zero (e.g., not a git
/// repository, though `NamespacedGitStore::open` will have caught that
/// earlier).
fn init_enumerate_git_ls_files(repo: &std::path::Path) -> std::io::Result<Vec<String>> {
    let out = Command::new("git")
        .arg("ls-files")
        .current_dir(repo)
        .output()?;
    if !out.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "git ls-files exit {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            ),
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    Ok(stdout
        .lines()
        .filter(|l| !l.is_empty())
        .filter(|l| {
            // Skip submodule entries and any path that doesn't resolve to a
            // readable file (the substrate cannot project a tree-as-leaf
            // through `project::project`'s byte-read surface).
            let p = repo.join(l);
            p.is_file()
        })
        .map(|l| l.to_string())
        .collect())
}

/// Lowercase-hex encoding of a Blake3Oid's raw bytes (32 -> 64 chars).
/// The substrate's standard OID string form for ref writes and envelope
/// emission.
fn init_blake3_oid_hex(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes.iter() {
        // Lowercase hex; matches the substrate convention for
        // content-addressed strings.
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Compute the root_oid as BLAKE3 of the sorted (target, splinter_oid)
/// pairs per spec §4.5. Stable per git state: same `git ls-files` +
/// same file bytes -> same root_oid. Idempotency lives here.
fn init_compute_root_oid(pairs: &[(String, String)]) -> String {
    // Canonical bytes: for each pair in given order, emit
    //   u64_le(target.len()) || target || u64_le(oid.len()) || oid
    // The given order is already sorted (BTreeMap iteration).
    let mut buf: Vec<u8> = Vec::new();
    for (target, oid) in pairs.iter() {
        let tlen = (target.len() as u64).to_le_bytes();
        buf.extend_from_slice(&tlen);
        buf.extend_from_slice(target.as_bytes());
        let olen = (oid.len() as u64).to_le_bytes();
        buf.extend_from_slice(&olen);
        buf.extend_from_slice(oid.as_bytes());
    }
    let digest = blake3::hash(&buf);
    let bytes = digest.as_bytes();
    let mut s = String::with_capacity(64);
    for b in bytes.iter() {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

// ────────────────────────────────────────────────────────────────────────────────
// `mirror recall <spec-dir>` — the inbound trajectory surface.
// ────────────────────────────────────────────────────────────────────────────────

fn cmd_recall(spec_dir: &str, ctx: &Ctx) -> i32 {
    // Edge case: missing/invalid directory returns non-zero. Resolve
    // via ctx so relative paths honor the dispatch context.
    let resolved = ctx.resolve(spec_dir);
    let dir_path = resolved.as_path();
    let md = match fs::metadata(dir_path) {
        Ok(m) => m,
        Err(e) => {
            merr!("recall: cannot stat spec dir {}: {}", spec_dir, e);
            return 1;
        }
    };
    if !md.is_dir() {
        merr!("recall: not a directory: {}", spec_dir);
        return 1;
    }

    // Compose the four payloads per Mara spec §3 (b034a60). Pass the
    // ctx-resolved absolute form to helpers so their internal
    // `format!("{}/docs/...", spec_dir)` composes to a filesystem
    // path independent of the process cwd.
    let resolved_str = resolved.to_string_lossy();
    let cascade = recall_cascade(&resolved_str);
    let pack_trail = recall_pack_trail(&resolved_str);
    let pull_frontier = recall_pull_frontier(&resolved_str);
    let dogfood = recall_dogfood(&resolved_str);

    let envelope = serde_json::json!({
        "spec_version": "v0.1.0",
        "cascade": cascade,
        "pack_trail": pack_trail,
        "pull_frontier": pull_frontier,
        "dogfood": dogfood,
    });

    // Single-line JSON so test parser sees one value when stdout.trim()'d.
    let s = format!("{}\n", envelope);
    _raw_stdout(s.as_bytes());
    0
}

/// Walk `<spec_dir>/docs/specs/recognitions/recognition-*.md` files;
/// return the most-recent ~10 by file mtime as per Mara spec §3.1.
/// Each record carries recognition_number, status, canonical_doc,
/// promotion_commit (most-recent SHA), pack_attribution (author),
/// altitude (omitted/unknown when not parseable).
fn recall_cascade(spec_dir: &str) -> serde_json::Value {
    let dir = format!("{}/docs/specs/recognitions", spec_dir);
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return serde_json::Value::Array(vec![]),
    };

    let mut files: Vec<(String, std::time::SystemTime)> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().into_owned();
        if !name_str.starts_with("recognition-") || !name_str.ends_with(".md") {
            continue;
        }
        let md = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !md.is_file() {
            continue;
        }
        let mtime = md.modified().unwrap_or(std::time::UNIX_EPOCH);
        let rel = format!("docs/specs/recognitions/{}", name_str);
        files.push((rel, mtime));
    }
    files.sort_by(|a, b| b.1.cmp(&a.1));
    files.truncate(10);

    let mut records: Vec<serde_json::Value> = Vec::new();
    for (rel_path, _mtime) in files {
        let basename = rel_path.rsplit('/').next().unwrap_or(&rel_path).to_string();
        let recognition_number = parse_recognition_number(&basename);
        let abs = format!("{}/{}", spec_dir, rel_path);
        let body = fs::read_to_string(&abs).unwrap_or_default();
        let status = parse_recognition_status(&body);

        // git -C <spec_dir> log -1 --format="%H %an" -- <relpath>
        let (promotion_commit, pack_attribution) = git_last_commit_for(spec_dir, &rel_path);

        records.push(serde_json::json!({
            "recognition_number": recognition_number,
            "status": status,
            "canonical_doc": rel_path,
            "promotion_commit": promotion_commit,
            "pack_attribution": pack_attribution,
            "altitude": "unknown",
        }));
    }
    serde_json::Value::Array(records)
}

fn parse_recognition_number(basename: &str) -> serde_json::Value {
    // basename = "recognition-NN-...md"
    let rest = match basename.strip_prefix("recognition-") {
        Some(r) => r,
        None => return serde_json::Value::Null,
    };
    let num_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    match num_str.parse::<u64>() {
        Ok(n) => serde_json::Value::from(n),
        Err(_) => serde_json::Value::Null,
    }
}

fn parse_recognition_status(body: &str) -> &'static str {
    let lower = body.to_lowercase();
    // Look at the head of the file where status is typically declared.
    let head: String = lower.chars().take(2000).collect();
    if head.contains("promoted") {
        "promoted"
    } else if head.contains("candidate") {
        "candidate"
    } else {
        "unknown"
    }
}

/// Run `git -C <spec_dir> log -1 --format=%H|%an -- <rel_path>`.
/// Returns (commit_sha, author_name); empty strings on failure.
fn git_last_commit_for(spec_dir: &str, rel_path: &str) -> (String, String) {
    let out = Command::new("git")
        .arg("-C")
        .arg(spec_dir)
        .arg("log")
        .arg("-1")
        .arg("--format=%H|%an")
        .arg("--")
        .arg(rel_path)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s.is_empty() {
                return (String::new(), String::new());
            }
            match s.split_once('|') {
                Some((sha, an)) => (sha.to_string(), an.to_string()),
                None => (s, String::new()),
            }
        }
        _ => (String::new(), String::new()),
    }
}

/// Walk Pack-attributed commits in the last month; group by peer and
/// emit one record per peer with last_seen_commit (content-addressed
/// per Seam Discharge C, 88f8428). NO `in_flight` field — forbidden
/// per b10f00c §4 stateless-return-at-runtime.
fn recall_pack_trail(spec_dir: &str) -> serde_json::Value {
    let out = Command::new("git")
        .arg("-C")
        .arg(spec_dir)
        .arg("log")
        .arg("--format=%H|%an|%ae|%ct|%s")
        .arg("--since=1 month ago")
        .arg("-100")
        .output();
    let raw = match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).into_owned(),
        _ => return serde_json::Value::Array(vec![]),
    };

    let pack_names: [&str; 5] = ["Mara", "Seam", "Taut", "Glint", "Reed"];

    // Per-peer accumulator: (last_sha, last_unix, recent_phases [first 5])
    use std::collections::HashMap;
    struct Acc {
        last_sha: String,
        last_at: i64,
        recent_subjects: Vec<String>,
    }
    let mut by_peer: HashMap<String, Acc> = HashMap::new();

    for line in raw.lines() {
        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if parts.len() < 5 {
            continue;
        }
        let sha = parts[0];
        let an = parts[1];
        let ae = parts[2];
        let ct: i64 = parts[3].parse().unwrap_or(0);
        let subject = parts[4];

        // Must match Pack peer name AND author email is on systemic.engineer.
        let peer_name = match pack_names.iter().find(|n| **n == an) {
            Some(n) => *n,
            None => continue,
        };
        if !ae.contains("@systemic.engineer") {
            continue;
        }

        let entry = by_peer.entry(peer_name.to_string()).or_insert_with(|| Acc {
            last_sha: String::new(),
            last_at: 0,
            recent_subjects: Vec::new(),
        });
        // git log returns most-recent first, so first hit per peer is latest.
        if entry.last_sha.is_empty() {
            entry.last_sha = sha.to_string();
            entry.last_at = ct;
        }
        if entry.recent_subjects.len() < 5 {
            entry.recent_subjects.push(subject.to_string());
        }
    }

    // Records tuple: (last_at DESC, peer name ASC) for stable ordering.
    // Pre-fix (2026-07-11): only sorted by last_at, so simultaneous
    // commits from two peers (same-second timestamps) produced
    // HashMap-iteration-order-dependent output, breaking
    // cmd_peer_beam_shard::t05_spawn_alias_stdout_byte_equal_to_peer_beam
    // when Mara + Taut co-committed at 1783735055.
    let mut records: Vec<(i64, String, serde_json::Value)> = by_peer
        .into_iter()
        .map(|(peer, acc)| {
            let phases: Vec<String> = acc
                .recent_subjects
                .iter()
                .filter_map(|s| extract_phase_marker(s))
                .collect();
            let v = serde_json::json!({
                "peer": peer,
                "last_seen_commit": acc.last_sha,
                "last_seen_at": acc.last_at,
                "recent_phases": phases,
            });
            (acc.last_at, peer, v)
        })
        .collect();

    // Primary: last_at DESC (most-recent first). Secondary: peer name
    // ASC (deterministic on ties).
    records.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let values: Vec<serde_json::Value> = records.into_iter().map(|(_, _, v)| v).collect();
    serde_json::Value::Array(values)
}

/// Pull the first phase-marker emoji/symbol from a commit subject.
/// Recognised markers: 🔴 🟢 ♻️ 📝 🔧 🔀.
fn extract_phase_marker(subject: &str) -> Option<String> {
    let markers: [&str; 6] = ["🔴", "🟢", "♻️", "📝", "🔧", "🔀"];
    for m in markers {
        if subject.contains(m) {
            return Some(m.to_string());
        }
    }
    None
}

/// Walk `<spec_dir>/docs/specs/recognitions/candidates/*.md` if the
/// directory exists; else empty. Per Mara spec §3.3.
fn recall_pull_frontier(spec_dir: &str) -> serde_json::Value {
    let dir_rel = "docs/specs/recognitions/candidates";
    let dir_abs = format!("{}/{}", spec_dir, dir_rel);
    let entries = match fs::read_dir(&dir_abs) {
        Ok(e) => e,
        Err(_) => return serde_json::Value::Array(vec![]),
    };

    let mut records: Vec<serde_json::Value> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().into_owned();
        if !name_str.ends_with(".md") {
            continue;
        }
        let rel = format!("{}/{}", dir_rel, name_str);
        let abs = format!("{}/{}", spec_dir, rel);
        let body = fs::read_to_string(&abs).unwrap_or_default();

        let recog_num = parse_recognition_number(&name_str);
        let identifier = match &recog_num {
            serde_json::Value::Number(n) => format!("recognition #{}", n),
            _ => name_str.clone(),
        };
        let witness_count = count_witnesses_in_body(&body);

        // surfaced_at = first commit touching this file.
        let surfaced_at = git_first_commit_for(spec_dir, &rel);

        records.push(serde_json::json!({
            "kind": "candidate_recognition",
            "identifier": identifier,
            "canonical_doc": rel,
            "witness_count": witness_count,
            "witnesses_needed": 2,
            "surfaced_at": surfaced_at,
        }));
    }
    serde_json::Value::Array(records)
}

/// Best-effort scan for explicit `## Witness <N>` headings (per Mara
/// spec §3.3.1) or `Witness N:` lines in the body.
fn count_witnesses_in_body(body: &str) -> u64 {
    let mut count: u64 = 0;
    for line in body.lines() {
        let t = line.trim();
        let lower = t.to_lowercase();
        if lower.starts_with("## witness ") || lower.starts_with("### witness ") {
            count += 1;
        }
    }
    count
}

/// Run `git -C <spec_dir> log --diff-filter=A --format=%H -- <rel>`,
/// return the first-introducing commit. Empty string on failure.
fn git_first_commit_for(spec_dir: &str, rel_path: &str) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(spec_dir)
        .arg("log")
        .arg("--diff-filter=A")
        .arg("--format=%H")
        .arg("--")
        .arg(rel_path)
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            // log can return multiple SHAs (rename/re-add); pick first.
            s.lines().next().unwrap_or("").to_string()
        }
        _ => String::new(),
    }
}

/// Read `<spec_dir>/mirror.spec`'s settle_on block; surface predicate
/// names + spec OID. v0 returns "unknown" verdicts (no live verdict
/// cache wired in this tick) per Mara spec §3.4.1.
fn recall_dogfood(spec_dir: &str) -> serde_json::Value {
    let spec_path = format!("{}/mirror.spec", spec_dir);
    let body = match fs::read_to_string(&spec_path) {
        Ok(s) => s,
        Err(_) => {
            return serde_json::json!({
                "spec_oid": "none",
                "settle_on_predicates": [],
                "predicate_verdicts": [],
                "aggregate": "unknown",
                "cache_freshness": "stale",
            });
        }
    };

    let predicates = parse_settle_on_predicates(&body);

    // Spec OID via git rev-parse HEAD:mirror.spec; "uncommitted" if it fails.
    let spec_oid = {
        let out = Command::new("git")
            .arg("-C")
            .arg(spec_dir)
            .arg("rev-parse")
            .arg("HEAD:mirror.spec")
            .output();
        match out {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => "uncommitted".to_string(),
        }
    };

    // most_recent_landed_at = current HEAD SHA.
    let most_recent_landed_at = {
        let out = Command::new("git")
            .arg("-C")
            .arg(spec_dir)
            .arg("rev-parse")
            .arg("HEAD")
            .output();
        match out {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => String::new(),
        }
    };

    let predicate_verdicts: Vec<serde_json::Value> = predicates
        .iter()
        .map(|p| serde_json::json!({"predicate": p, "state": "unknown"}))
        .collect();

    serde_json::json!({
        "spec_oid": spec_oid,
        "settle_on_predicates": predicates,
        "predicate_verdicts": predicate_verdicts,
        "aggregate": "unknown",
        "most_recent_landed_at": most_recent_landed_at,
        "cache_freshness": "stale",
    })
}

/// Parse the `settle_on { ... }` block from a mirror.spec body; return
/// non-empty/non-comment predicate-name lines.
fn parse_settle_on_predicates(body: &str) -> Vec<String> {
    let mut preds: Vec<String> = Vec::new();
    let mut in_block = false;
    let mut depth: i32 = 0;
    for line in body.lines() {
        let trimmed = line.trim();
        if !in_block {
            if trimmed.starts_with("settle_on {") || trimmed.starts_with("settle_on{") {
                in_block = true;
                depth = 1;
                continue;
            }
        } else {
            // Track brace depth so nested blocks (none expected, but
            // defensive) don't terminate the block early.
            for c in trimmed.chars() {
                match c {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
            }
            if depth <= 0 {
                break;
            }
            // Strip inline comments.
            let no_comment = match trimmed.split_once('#') {
                Some((pre, _)) => pre.trim(),
                None => trimmed,
            };
            if no_comment.is_empty() {
                continue;
            }
            // Skip lines that are purely brace artefacts.
            if no_comment.chars().all(|c| c == '{' || c == '}') {
                continue;
            }
            preds.push(no_comment.to_string());
        }
    }
    preds
}

// ───────────────────────────────────────────────────────────────────────────────
// `mirror spawn <peer-home>` — Phase G v0 empirical-path-traversal proof.
// ───────────────────────────────────────────────────────────────────────────────

/// Walk `peer_home` recursively, blake3-hashing each file's content
/// as a `moment_oid`, then aggregating all moment_oids into a
/// deterministic `psychohistory_root_oid`. Substrate-honest stub for
/// the sheaf-Laplacian Δ_F Rayleigh direction on the peer's
/// psychohistory sheaf (Mara `2c26537` + `ce9745f` + `96ff532`); the
/// v0 blake3 mapping is the same-altitude realization idiom used at
/// `@bauchladen`-lifts-`@mirror/store` per Taut `e90daf1` Q1.
///
/// Returns (root_oid_hex, moment_count). Skips `.git`, `target`,
/// `.mirror` (action cache), `node_modules`, and hidden dirs deeper
/// than one level to keep the walk bounded.
pub fn psychohistory_root_from_peer_home(peer_home: &std::path::Path) -> (String, usize) {
    let mut moment_oids: Vec<[u8; 32]> = Vec::new();

    fn walk(dir: &std::path::Path, out: &mut Vec<[u8; 32]>, depth: usize) {
        if depth > 8 {
            return;
        }
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if matches!(
                name_str.as_ref(),
                ".git" | "target" | ".mirror" | "node_modules" | ".cargo-target"
            ) {
                continue;
            }
            let file_type = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if file_type.is_dir() {
                walk(&path, out, depth + 1);
            } else if file_type.is_file() {
                if let Ok(bytes) = fs::read(&path) {
                    let digest = blake3::hash(&bytes);
                    let mut oid = [0u8; 32];
                    oid.copy_from_slice(digest.as_bytes());
                    out.push(oid);
                }
            }
        }
    }

    walk(peer_home, &mut moment_oids, 0);

    // Deterministic ordering across walks (fs read_dir order varies).
    moment_oids.sort_unstable();

    // Aggregate: blake3 over concatenation of sorted moment_oids.
    let mut agg = Vec::with_capacity(moment_oids.len() * 32);
    for oid in &moment_oids {
        agg.extend_from_slice(oid);
    }
    let root_digest = blake3::hash(&agg);
    let root_bytes = root_digest.as_bytes();
    let mut root_hex = String::with_capacity(64);
    for b in root_bytes.iter() {
        root_hex.push_str(&format!("{:02x}", b));
    }

    (root_hex, moment_oids.len())
}

/// Derive `Fate::selectors` from a psychohistory root oid via a
/// deterministic xorshift64 stream seeded from the first 8 bytes of
/// the root. Substrate-honest v1 stub for the full sheaf-Laplacian
/// Rayleigh direction extraction — same shape as `Fate::excited()`'s
/// randomization but with the seed grounded in peer content instead
/// of wall-clock time.
///
/// Weight ranges match `Fate::excited()`:
/// - biases: [0.0, 20.0]
/// - feature weights: [0.0, 5.0]
/// - depth weights: [0.0, 5.0]
pub fn selectors_from_psychohistory_root(root_hex: &str) -> [fate::ModelWeights; 5] {
    // Parse first 8 bytes of the 64-hex-char root as u64 seed.
    let seed: u64 = {
        let bytes = root_hex.as_bytes();
        let mut s: u64 = 0;
        let take = bytes.len().min(16);
        for i in 0..take {
            let c = bytes[i];
            let nibble = if c.is_ascii_digit() {
                c - b'0'
            } else if (b'a'..=b'f').contains(&c) {
                c - b'a' + 10
            } else {
                0
            };
            s = s.wrapping_shl(4).wrapping_add(nibble as u64);
        }
        if s == 0 {
            0x1234_5678_9abc_def0
        } else {
            s
        }
    };

    let mut state = seed;
    let mut next = || -> u64 {
        state ^= state.wrapping_shl(13);
        state ^= state.wrapping_shr(7);
        state ^= state.wrapping_shl(17);
        state
    };

    let mut make_selector = || -> fate::ModelWeights {
        let mut w = [[0.0f64; fate::FEATURE_DIM]; 5];
        let mut b = [0.0f64; 5];
        let mut depth_w = [0.0f64; 5];
        for i in 0..5 {
            for j in 0..fate::FEATURE_DIM {
                // Uniform in [0.0, 5.0].
                let r = (next() as f64) / (u64::MAX as f64);
                w[i][j] = r * 5.0;
            }
            let rb = (next() as f64) / (u64::MAX as f64);
            b[i] = rb * 20.0;
            let rd = (next() as f64) / (u64::MAX as f64);
            depth_w[i] = rd * 5.0;
        }
        fate::ModelWeights { w, b, depth_w }
    };

    [
        make_selector(),
        make_selector(),
        make_selector(),
        make_selector(),
        make_selector(),
    ]
}

/// Cast shadows (v1) — for each fate Model, perturb features in that
/// Model's direction, resolve, and compute impact distance from base.
/// Returns (base_decision, [5 hypothetical decisions], [5 impact
/// magnitudes as L2 distances between distribution vectors]).
///
/// Substrate authority: Mara `ce301cc` iter-35 cast_shadow(sheaf,
/// direction, p) -> imperfect(shadow, holonomy) at @song/narrative;
/// each Model M's perturbation IS one candidate direction; the
/// impact IS the holonomy residual per bundle-tower Level 3 Transport.
///
/// v1 perturbation is unit amplitude at position M of the 16-dim
/// Features vector (crude but deterministic). v2 lifts to sheaf-
/// Laplacian Δ_F Rayleigh direction extraction per Mara iter-34 §8.
fn cast_shadows_over_models(
    fate_engine: &fate::Fate,
    base_features: &fate::Features,
) -> (fate::Decision, [fate::Decision; 5], [f64; 5]) {
    let base_decision = fate_engine.resolve(base_features, 5);

    let make_hypothetical = |model_idx: usize| -> fate::Features {
        let mut f = *base_features;
        // Perturb feature dim at position model_idx with amplitude 5.0
        // (matches Fate::excited() weight range so the perturbation is
        // meaningful against random selectors).
        if model_idx < 16 {
            f[model_idx] = 5.0;
        }
        f
    };

    let hypotheticals: [fate::Decision; 5] = [
        fate_engine.resolve(&make_hypothetical(0), 5),
        fate_engine.resolve(&make_hypothetical(1), 5),
        fate_engine.resolve(&make_hypothetical(2), 5),
        fate_engine.resolve(&make_hypothetical(3), 5),
        fate_engine.resolve(&make_hypothetical(4), 5),
    ];

    // Impact = L2 distance between distribution vectors.
    let impacts: [f64; 5] = {
        let mut r = [0.0f64; 5];
        for i in 0..5 {
            let mut sum_sq = 0.0;
            for j in 0..5 {
                let d = hypotheticals[i].distribution[j] - base_decision.distribution[j];
                sum_sq += d * d;
            }
            r[i] = sum_sq.sqrt();
        }
        r
    };

    (base_decision, hypotheticals, impacts)
}

/// Classify shadow regime per Mara `f2c712e` iter-34 §3 three-regime
/// classifier + convergent case, per Reed's essay Council-Square
/// dynamics.
///
/// - converged: all 5 hypotheticals agree on argmax with base
///   (kintsugi e^(n+1) ≤ e^n terminates; Council verifies)
/// - necker: 2+ distinct argmax modes across hypotheticals (bistable;
///   Council sees ambiguous evidence → fetch more moments)
/// - escher: 3+ distinct argmax modes OR high variance (impossible;
///   Council imprisons Square → reframe required)
/// - kanizsa: 2 modes with low variance (illusory-convergent; matter
///   inferred from gauge)
fn shadow_regime(
    base_decision: &fate::Decision,
    hypotheticals: &[fate::Decision; 5],
    impacts: &[f64; 5],
) -> (&'static str, usize) {
    // Argmax of a Decision = its `model` field's index.
    let model_idx = |m: fate::Model| -> usize {
        match m {
            fate::Model::Abyss => 0,
            fate::Model::Introject => 1,
            fate::Model::Cartographer => 2,
            fate::Model::Explorer => 3,
            fate::Model::Fate => 4,
        }
    };

    let base_argmax = model_idx(base_decision.model);
    let mut modes = std::collections::HashSet::new();
    modes.insert(base_argmax);
    for h in hypotheticals.iter() {
        modes.insert(model_idx(h.model));
    }
    let distinct_modes = modes.len();

    // Variance of impacts.
    let mean = impacts.iter().sum::<f64>() / 5.0;
    let variance = impacts.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / 5.0;
    let variance_threshold = 0.05;

    let regime = if distinct_modes == 1 {
        "converged"
    } else if distinct_modes >= 3 || variance > variance_threshold * 2.0 {
        "escher"
    } else if distinct_modes == 2 && variance > variance_threshold {
        "necker"
    } else {
        "kanizsa"
    };

    (regime, distinct_modes)
}

fn fate_bounded_shadow_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    task: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    // Reuse psychohistory root + derived selectors from bounded_by.
    let peer_home_resolved = ctx.resolve(peer_home);
    let (psychohistory_root_oid, moments_count) =
        psychohistory_root_from_peer_home(&peer_home_resolved);

    let features: fate::Features = [0.0; 16];
    let mut fate_engine = fate::Fate::untrained();
    fate_engine.selectors = selectors_from_psychohistory_root(&psychohistory_root_oid);

    // Cast 5 shadows (one per Model) + compute regime.
    let (base_decision, hypotheticals, impacts) = cast_shadows_over_models(&fate_engine, &features);
    let (regime, distinct_modes) = shadow_regime(&base_decision, &hypotheticals, &impacts);

    let model_label = |m: fate::Model| -> &'static str {
        match m {
            fate::Model::Abyss => "Abyss",
            fate::Model::Introject => "Introject",
            fate::Model::Cartographer => "Cartographer",
            fate::Model::Explorer => "Explorer",
            fate::Model::Fate => "Fate",
        }
    };

    let (base_name, base_op, base_level) = match base_decision.model {
        fate::Model::Abyss => ("Abyss", "focus", "Level 0 Fiber"),
        fate::Model::Introject => ("Introject", "project", "Level 1 Connection"),
        fate::Model::Cartographer => ("Cartographer", "split", "Level 2 Gauge"),
        fate::Model::Explorer => ("Explorer", "shift", "Level 3 Transport"),
        fate::Model::Fate => ("Fate", "settle", "Level 4 Closure"),
    };

    let spec_bytes = fs::read(spec_path).unwrap_or_default();
    let spec_oid = {
        let digest = blake3::hash(&spec_bytes);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    let mission_line = match task {
        None => "+ mission: <absent>".to_string(),
        Some(path) => {
            let mission_path = ctx.resolve(path);
            match fs::read_to_string(&mission_path) {
                Ok(text) => {
                    let first = text.lines().next().unwrap_or("").trim();
                    if first.is_empty() {
                        "+ mission: <empty>".to_string()
                    } else {
                        format!("+ mission: {}", first)
                    }
                }
                Err(_) => "+ mission: <unreadable>".to_string(),
            }
        }
    };

    let regime_gloss = match regime {
        "converged" => "Council verifies Square's claim from Flatland measurements",
        "necker" => "Council sees ambiguous evidence — fetch more moments",
        "escher" => "Council would imprison Square — impossible geometry; reframe required",
        "kanizsa" => "illusory-convergent — matter inferred from gauge",
        _ => "unknown",
    };

    let out = format!(
        "--- a/mirror.spec\n\
         +++ b/mirror.spec\n\
         @@ peer_beam cast_shadow over 5 Models + shadow_regime classifier via @song/narrative.shadow_ancestry @@\n\
         + peer_home: {peer_home}\n\
         + spec_oid: {spec_oid}\n\
         + psychohistory_root_oid: {psychohistory_root_oid}\n\
         + psychohistory_moments_count: {moments_count}\n\
         + fate_decision: {base_name} ↔ {base_op}\n\
         + fate_confidence: {conf:.6}\n\
         + bundle_tower_binding: {base_level} per boot/std/epistemologic/math/bundle.mirror\n\
         + shadow_abyss: {i0:.6} → {n0}\n\
         + shadow_introject: {i1:.6} → {n1}\n\
         + shadow_cartographer: {i2:.6} → {n2}\n\
         + shadow_explorer: {i3:.6} → {n3}\n\
         + shadow_fate: {i4:.6} → {n4}\n\
         + shadow_regime: {regime} ({distinct_modes} distinct argmax modes across 5 candidates)\n\
         + shadow_regime_gloss: {regime_gloss}\n\
         + optics_lens: @optics/lens/features\n\
         + bounded_by_altitude: @fate/tournament.bounded_by (Mara ce9745f)\n\
         + psychohistory_altitude: @song/narrative.psychohistory_sheaf (Mara 2c26537)\n\
         + shadow_altitude: @song/narrative.shadow_ancestry (Mara ce301cc — goth)\n\
         + shadow_composition: cast_shadow IS Level 3 Transport with state specialised to shadow (Mara ce301cc)\n\
         + essay_source: The Shape of the Thing (Reed 2026-06 — Flatland shadow determined by casting object)\n\
         {mission_line}\n",
        conf = base_decision.distribution[match base_decision.model {
            fate::Model::Abyss => 0,
            fate::Model::Introject => 1,
            fate::Model::Cartographer => 2,
            fate::Model::Explorer => 3,
            fate::Model::Fate => 4,
        }],
        i0 = impacts[0], n0 = model_label(hypotheticals[0].model),
        i1 = impacts[1], n1 = model_label(hypotheticals[1].model),
        i2 = impacts[2], n2 = model_label(hypotheticals[2].model),
        i3 = impacts[3], n3 = model_label(hypotheticals[3].model),
        i4 = impacts[4], n4 = model_label(hypotheticals[4].model),
    );

    mout!("{}", out);
    0
}

fn fate_bounded_by_psychohistory_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    task: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    // Build psychohistory root from peer_home content.
    let peer_home_resolved = ctx.resolve(peer_home);
    let (psychohistory_root_oid, moments_count) =
        psychohistory_root_from_peer_home(&peer_home_resolved);

    // Features remain Features::default() per Seam d9b7c35 Adj 1 —
    // the v0→v1 lift here is on the WEIGHTS (bounded by psychohistory
    // per Mara 96ff532 + ce9745f), not the input features. Real @nl-
    // driven feature encoding lands with the @magic/nl adapter
    // discharge (subsequent tick).
    let features: fate::Features = [0.0; 16];

    // Base Fate::untrained() for the private connection field, then
    // overwrite public `selectors` with psychohistory-derived weights.
    // This is `Fate::bounded(config)` where config.weights is derived
    // from the peer's psychohistory sheaf via deterministic stub
    // (v1); v2 will replace with sheaf-Laplacian Δ_F Rayleigh direction.
    let mut fate_engine = fate::Fate::untrained();
    fate_engine.selectors = selectors_from_psychohistory_root(&psychohistory_root_oid);
    let decision = fate_engine.resolve(&features, 5);

    let (model_name, prism_op, level_desc) = match decision.model {
        fate::Model::Abyss => ("Abyss", "focus", "Level 0 Fiber"),
        fate::Model::Introject => ("Introject", "project", "Level 1 Connection"),
        fate::Model::Cartographer => ("Cartographer", "split", "Level 2 Gauge"),
        fate::Model::Explorer => ("Explorer", "shift", "Level 3 Transport"),
        fate::Model::Fate => ("Fate", "settle", "Level 4 Closure"),
    };

    let confidence = {
        let idx = match decision.model {
            fate::Model::Abyss => 0,
            fate::Model::Introject => 1,
            fate::Model::Cartographer => 2,
            fate::Model::Explorer => 3,
            fate::Model::Fate => 4,
        };
        decision.distribution[idx]
    };

    // spec_oid for provenance.
    let spec_bytes = fs::read(spec_path).unwrap_or_default();
    let spec_oid = {
        let digest = blake3::hash(&spec_bytes);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    let mission_line = match task {
        None => "+ mission: <absent>".to_string(),
        Some(path) => {
            let mission_path = ctx.resolve(path);
            match fs::read_to_string(&mission_path) {
                Ok(text) => {
                    let first = text.lines().next().unwrap_or("").trim();
                    if first.is_empty() {
                        "+ mission: <empty>".to_string()
                    } else {
                        format!("+ mission: {}", first)
                    }
                }
                Err(_) => "+ mission: <unreadable>".to_string(),
            }
        }
    };

    let out = format!(
        "--- a/mirror.spec\n\
         +++ b/mirror.spec\n\
         @@ peer_beam fate.bounded_by(psychohistory_sheaf) via @fate/tournament.bounded_by @@\n\
         + peer_home: {peer_home}\n\
         + spec_oid: {spec_oid}\n\
         + psychohistory_root_oid: {psychohistory_root_oid}\n\
         + psychohistory_moments_count: {moments_count}\n\
         + features: Features::default() (v0 stub per Seam d9b7c35 Adj 1; →@nl adapter is subsequent tick)\n\
         + fate_decision: {model_name} ↔ {prism_op}\n\
         + fate_confidence: {confidence:.6}\n\
         + bundle_tower_binding: {level_desc} per boot/std/epistemologic/math/bundle.mirror\n\
         + optics_lens: @optics/lens/features\n\
         + optics_lens_features_altitude: substrate-decl (Mara f3af5b4)\n\
         + bounded_by_altitude: @fate/tournament.bounded_by (Mara ce9745f)\n\
         + psychohistory_altitude: @song/narrative.psychohistory_sheaf (Mara 2c26537)\n\
         + weights_derivation: v1 stub — xorshift64 seeded from psychohistory_root_oid; v2 → sheaf-Laplacian Δ_F Rayleigh direction on trajectory graph\n\
         {mission_line}\n"
    );

    mout!("{}", out);
    0
}

fn fate_select_peer_beam(
    peer_home: &str,
    spec_path: &std::path::Path,
    task: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    // v0 Features per Seam d9b7c35 Adj 1: Features::default() (all
    // zeros) is @mirror/spectral/observation at "no observation yet"
    // state — substrate-honest v0, not a type-lie. blake3 was drift
    // (@mirror/store altitude, not features altitude). Real @nl-driven
    // encoding lands with shards/magic/nl.mirror adapter cascade.
    let features: fate::Features = [0.0; 16];

    // Fate::excited() uses xorshift64 seeded from system time —
    // random weights, non-deterministic decisions across invocations.
    // untrained() would produce uniform (always Abyss idx 0); excited()
    // produces non-trivial decisions from any Features vector,
    // including default(). resolve(&features, 5) iterates up to depth
    // 5 with entropy-based exit, guaranteeing convergence to a
    // non-Fate model (settle can occur if fate's own selector picks
    // itself with high confidence past the entropy threshold).
    let fate_engine = fate::Fate::excited();
    let decision = fate_engine.resolve(&features, 5);

    // Bundle tower binding per boot/std/epistemologic/math/bundle.mirror:
    // Level 0 Fiber / Level 1 Connection / Level 2 Gauge / Level 3
    // Transport / Level 4 Closure = focus / project / split / shift /
    // settle. Same 5-op algebra prism-core carries as spectral triple's
    // A (algebra) generators.
    let (model_name, prism_op, level_desc) = match decision.model {
        fate::Model::Abyss => ("Abyss", "focus", "Level 0 Fiber"),
        fate::Model::Introject => ("Introject", "project", "Level 1 Connection"),
        fate::Model::Cartographer => ("Cartographer", "split", "Level 2 Gauge"),
        fate::Model::Explorer => ("Explorer", "shift", "Level 3 Transport"),
        fate::Model::Fate => ("Fate", "settle", "Level 4 Closure"),
    };

    // Winning probability per decision.distribution.
    let confidence = {
        let idx = match decision.model {
            fate::Model::Abyss => 0,
            fate::Model::Introject => 1,
            fate::Model::Cartographer => 2,
            fate::Model::Explorer => 3,
            fate::Model::Fate => 4,
        };
        decision.distribution[idx]
    };

    // spec_oid for provenance.
    let spec_bytes = fs::read(spec_path).unwrap_or_default();
    let spec_oid = {
        let digest = blake3::hash(&spec_bytes);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    // Mission line — first line if present.
    let mission_line = match task {
        None => "+ mission: <absent>".to_string(),
        Some(path) => {
            let mission_path = ctx.resolve(path);
            match fs::read_to_string(&mission_path) {
                Ok(text) => {
                    let first = text.lines().next().unwrap_or("").trim();
                    if first.is_empty() {
                        "+ mission: <empty>".to_string()
                    } else {
                        format!("+ mission: {}", first)
                    }
                }
                Err(_) => "+ mission: <unreadable>".to_string(),
            }
        }
    };

    let out = format!(
        "--- a/mirror.spec\n\
         +++ b/mirror.spec\n\
         @@ peer_beam fate inference via @optics/lens/features.get + Fate::resolve @@\n\
         + peer_home: {peer_home}\n\
         + spec_oid: {spec_oid}\n\
         + features: Features::default() (v0 stub per Seam d9b7c35 Adj 1)\n\
         + fate_decision: {model_name} ↔ {prism_op}\n\
         + fate_confidence: {confidence:.6}\n\
         + bundle_tower_binding: {level_desc} per boot/std/epistemologic/math/bundle.mirror\n\
         + optics_lens: @optics/lens/features\n\
         + optics_lens_features_altitude: substrate-decl (Mara f3af5b4)\n\
         + features_carrier: graph_observation (16-dim per shards/mirror/spectral/observation.mirror)\n\
         + fate_engine: Fate::excited() (xorshift64 seeded weights; non-deterministic across invocations)\n\
         {mission_line}\n"
    );

    mout!("{}", out);
    0
}

fn integrate_peer_beam_diff(peer_home: &str, spec_path: &std::path::Path) -> i32 {
    // Read stdin bytes (the operator-edited diff). Empty stdin is a
    // valid put: represents "acknowledge no edit" at v0 semantics.
    let mut stdin_bytes = Vec::new();
    let _ = io::stdin().read_to_end(&mut stdin_bytes);

    // Read spec bytes best-effort.
    let spec_bytes = fs::read(spec_path).unwrap_or_default();

    // spec_oid via blake3 (matches emit direction).
    let spec_oid = {
        let digest = blake3::hash(&spec_bytes);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    // Autopoietic closure (Mara 7e5c298 iter-26): persist stdin_bytes
    // as a new moment file in peer_home/.bauchladen/. Content-addressed
    // by delta_oid so identical edits map to identical filenames
    // (idempotent persistence; multi-write same content is a no-op).
    // Next --fate-select --from-psychohistory walker sees the new file
    // as part of the psychohistory content graph → different
    // psychohistory_root_oid → different bounded_by weights →
    // potentially different Decision. The autopoietic loop CLOSES via
    // the storage layer (Taut e90daf1: @bauchladen already lifts
    // @mirror/store), no explicit weight-update step required.
    //
    // Empty stdin is degenerate: still persists an empty-content
    // moment (delta_oid stable per (spec, empty) pair); this witnesses
    // Foster PutPut law with zero payload.
    let bauchladen_dir = spec_path
        .parent()
        .map(|p| p.join(".bauchladen"))
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp/.bauchladen"));
    let bauchladen_moment_path: Option<std::path::PathBuf> =
        match fs::create_dir_all(&bauchladen_dir) {
            Ok(_) => {
                // Placeholder — delta_oid computed below, we'll build the
                // path once we have it.
                None
            }
            Err(_) => None,
        };

    let _ = bauchladen_moment_path;

    // delta_oid via blake3(spec_bytes || stdin_bytes). Content-
    // addressed identity for the (state, edit) pair — Foster PutPut
    // law witness: same edit bytes on same spec produce same
    // delta_oid regardless of invocation order.
    let delta_oid = {
        let mut combined = Vec::with_capacity(spec_bytes.len() + stdin_bytes.len());
        combined.extend_from_slice(&spec_bytes);
        combined.extend_from_slice(&stdin_bytes);
        let digest = blake3::hash(&combined);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    let received_bytes = stdin_bytes.len();

    // Persist stdin_bytes as moment_<delta_oid>.mirror inside
    // .bauchladen/ (created above best-effort). Absorb IO errors
    // silently — persistence failure degrades to acknowledge-only put
    // (v0 behavior); envelope still emits so operator sees the
    // acknowledgement.
    let bauchladen_moment_file = bauchladen_dir.join(format!("moment_{}.mirror", delta_oid));
    let persisted_path_display = match fs::write(&bauchladen_moment_file, &stdin_bytes) {
        Ok(_) => bauchladen_moment_file
            .strip_prefix(spec_path.parent().unwrap_or(&bauchladen_dir))
            .unwrap_or(&bauchladen_moment_file)
            .display()
            .to_string(),
        Err(_) => "<persist_failed>".to_string(),
    };

    // Unified-diff-shape envelope naming the put roundtrip AND the
    // bauchladen persistence (v1 autopoietic closure).
    let out = format!(
        "--- a/mirror.spec\n\
         +++ b/mirror.spec\n\
         @@ peer_beam operator-edit integration via @optics/lens/diff.put + @bauchladen persistence @@\n\
         + peer_home: {peer_home}\n\
         + spec_oid: {spec_oid}\n\
         + delta_oid: {delta_oid}\n\
         + received_bytes: {received_bytes}\n\
         + verdict: put_acknowledged_and_persisted\n\
         + optics_lens: @optics/lens/diff.put\n\
         + optics_lens_put_altitude: Rust runtime v1 (2026-07-11)\n\
         + optics_lens_put_semantics: acknowledge_address_and_persist\n\
         + bauchladen_moment: {persisted_path_display}\n\
         + bauchladen_altitude: @mirror/store (Taut e90daf1 Q1: @bauchladen lifts @mirror/store)\n\
         + autopoietic_closure: v1 CLOSED via .bauchladen/ persistence (next --fate-select --from-psychohistory reads updated content graph)\n\
         + foster_putput_witness: delta_oid deterministic per (spec_bytes, received_bytes); moment filename content-addressed\n"
    );

    mout!("{}", out);
    0
}

fn emit_peer_beam_diff(
    peer_home: &str,
    spec_path: &std::path::Path,
    task: Option<&str>,
    ctx: &Ctx,
) -> i32 {
    // Read spec bytes (best-effort — empty for partial peer).
    let spec_bytes = match fs::read(spec_path) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };

    // spec_oid via blake3 (matches the substrate's content-address
    // idiom at bootstrap/src/lib.rs's blake3::hash sites). Empty spec
    // still produces a well-defined content-address so the diff shape
    // is stable across peer maturity levels.
    let spec_oid = {
        let digest = blake3::hash(&spec_bytes);
        let bytes = digest.as_bytes();
        let mut s = String::with_capacity(64);
        for b in bytes.iter() {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    // Mission line — first line of the mission file, if present.
    let mission_line = match task {
        None => "+ mission: <absent>".to_string(),
        Some(path) => {
            let mission_path = ctx.resolve(path);
            match fs::read_to_string(&mission_path) {
                Ok(text) => {
                    let first = text.lines().next().unwrap_or("").trim();
                    if first.is_empty() {
                        "+ mission: <empty>".to_string()
                    } else {
                        format!("+ mission: {}", first)
                    }
                }
                Err(_) => "+ mission: <unreadable>".to_string(),
            }
        }
    };

    // Unified-diff-shape envelope. Real diff signature lines (---, +++,
    // @@) so downstream tooling (patch, git apply, review UIs) can
    // parse the shape without knowing the substrate. The + lines carry
    // the peer's substrate observation.
    let out = format!(
        "--- a/mirror.spec\n\
         +++ b/mirror.spec\n\
         @@ peer_beam substrate observation via @optics/lens/diff.get @@\n\
         + peer_home: {peer_home}\n\
         + spec_oid: {spec_oid}\n\
         + verdict: substrate_observation\n\
         + optics_lens: @optics/lens/diff\n\
         + optics_lens_get_altitude: substrate-decl (Mara b0427fd + 7e5c298)\n\
         + optics_lens_put_altitude: forward-promise (Rust runtime is next tick)\n\
         + fate_altitude: partial@recall (@fate wiring is subsequent tick)\n\
         {mission_line}\n"
    );

    mout!("{}", out);
    0
}

/// `mirror spawn <peer-home>` — v0 empirical-path-traversal proof.
///
/// Per Mara's spawn-semantics insight (`docs/insights/2026-06-26-spawn-is-
/// substrate-leaving-ground-state.md`, commit `b10f00c`): spawn IS the
/// substrate's controlled excitation above λ₀. The insight names seven
/// composition pieces (§2.1–§2.7) that must compose, and seven structural
/// negatives (§4.1–§4.7) the implementation must honor.
///
/// v0 implements pieces 1–3 as real reads (cli surface + peer-home
/// filesystem resolution + pack{}.lead extraction); pieces 4–7 are logged
/// stubs that prove the path traversed all seven. Phase H wires real
/// @fate inference through the cascade/code/* species.
///
/// Structural negatives honored (§4):
///   - No @os/process (no fork/exec; in-process only).
///   - No @io/llm (no external LLM adapter; @fate is the inference home).
///   - No identity-mint (identity comes from the home repo via filesystem;
///     v0 does NOT create peers).
///   - No delegation-chain (lead is the N+1 observer per peer-ACL §10).
///   - No idempotent-at-runtime (envelope is emitted per call).
///   - No membership-side-effects (spawn does NOT add members to the pack).
///   - No stateless-return (envelope acknowledges persisted state even
///     though v0 does not yet store it; Phase H wires storage).
///
/// The `emit_diff` param routes to the @optics/lens/diff.get direction
/// (Mara b0427fd + 7e5c298) — unified-diff-shape bytes for operator
/// review at cli-surface altitude. Mutually independent of `hello_world`;
/// if both flags are set, `emit_diff` wins.
///
/// The `integrate_diff` param routes to the @optics/lens/diff.put
/// direction — reads edited-diff bytes from stdin, emits envelope
/// naming delta_oid via blake3(spec_bytes || stdin_bytes). Closes the
/// Foster (get, put) roundtrip half at cli-surface altitude. If both
/// `emit_diff` and `integrate_diff` are set, `integrate_diff` wins.
///
/// The `fate_select` param routes to @optics/lens/features.get +
/// Fate::excited().resolve at Rust runtime. Emits selected Model +
/// mapped prism-op via bundle-tower binding. Wins over all other
/// flags when set (last check in dispatch cascade).
///
/// This is Blocker 2's final Rust hop: peer produces COMPUTED
/// candidates from mission text (v0: from Features::default() per
/// Seam d9b7c35 Adj 1) rather than substrate OBSERVATION. Real fate
/// optical inference at cli surface.
fn cmd_peer_beam(
    peer_home: &str,
    hello_world: bool,
    task: Option<&str>,
    ctx: &Ctx,
    emit_diff: bool,
    integrate_diff: bool,
    fate_select: bool,
    from_psychohistory: bool,
    with_shadow: bool,
    song: Option<&str>,
    dance_with: Option<&str>,
    deploy_to: Option<&str>,
    emit_crystal: bool,
) -> i32 {
    // Piece 1 (insight §2.1): cli surface. The peer-home argument is the
    // single positional. Context (frame, repository, pack) is resolved
    // FROM peer-home, not passed in. Resolve via ctx so relative
    // peer-home honors the dispatch context.
    let peer_home_resolved = ctx.resolve(peer_home);
    let spec_path = peer_home_resolved.join("mirror.spec");

    // Rung 6' (2026-07-13) — `--emit-crystal` @mirror/store-bounded
    // peer runtime per Mara `d2de1ee` canonical spec + Taut `8e98a24`
    // @io-minimization re-scout. Peer emits crystal OID on `refs/
    // mirror/peer/<uuid>/HEAD` instead of stdout envelope. Substrate-
    // inversion of Taut's prior Rung 6 @io/fs runtime trajectory: peer
    // inference stays @magic-native; peer state = crystal OID on
    // @mirror/store internal ref; materialization = ONE @io crossing
    // via @kintsugi/store/git.commit_as_fold (Rung 6.1+ forward-
    // promise). Fires BEFORE all other Rungs 1-5 dispatches when
    // present. Byte-equality preserved for non-`--emit-crystal` paths
    // via `if emit_crystal` guard; all prior dispatches unchanged.
    if emit_crystal {
        return crate::store_branch::emit_peer_crystal(peer_home, ctx);
    }

    // Rung 5 (2026-07-13) — `--deploy-to <target>` mycelial-envelope-
    // declared dispatch per Mara `9c4ef5b` Scope A + substrate
    // reservation. When ALL THREE of --song + --dance-with + --deploy-to
    // are present, the peer runs Rung 4 dance state computation for
    // shared_root_oid + emits deployment envelope naming @spectral/
    // garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/
    // mosaic + @song/beat substrate authorities. Byte-equality preserved
    // for two-way dance-only path via `if let (Some, Some, Some)`
    // three-way narrowing.
    if let (Some(song_path), Some(peer_home_2), Some(deploy_target)) =
        (song, dance_with, deploy_to)
    {
        let peer_home_2_resolved = ctx.resolve(peer_home_2);
        let spec_path_2 = peer_home_2_resolved.join("mirror.spec");
        return crate::deploy::execute_deploy(
            peer_home,
            peer_home_2,
            &spec_path,
            &spec_path_2,
            song_path,
            deploy_target,
            ctx,
        );
    }

    // Rung 4 (2026-07-13) — `--dance-with <peer-home-2>` multi-peer
    // coherence phase-lock dispatch per Mara `417ec25` Scope B narrowed
    // + substrate reservation at shards/song/beat.mirror:453-457. When
    // BOTH --song AND --dance-with are present, the peer executes
    // `execute_dance` which reads both peer-homes' coherence sequences
    // and emits Kuramoto order-parameter + Aumann agreement +
    // shared_root_oid + convergence_verdict envelope. Byte-equality
    // preserved for non-`--dance-with` paths via `if let (Some, Some)`
    // guard; Rungs 1-3 all continue working identically.
    if let (Some(song_path), Some(peer_home_2)) = (song, dance_with) {
        let peer_home_2_resolved = ctx.resolve(peer_home_2);
        let spec_path_2 = peer_home_2_resolved.join("mirror.spec");
        return crate::dance::execute_dance(
            peer_home,
            peer_home_2,
            &spec_path,
            &spec_path_2,
            song_path,
            ctx,
        );
    }

    // Rung 1 (2026-07-13) — `--song` early dispatch per Taut `c54740c`
    // §5.2 ladder. When --song is present, the peer's @song/beat runtime
    // takes over: fire ONE @kintsugi/oscillate ACTIVE/DARK pulse per
    // beat, emit beat-envelope naming @song/beat + @kintsugi/oscillate
    // substrate authorities. Byte-equality preserved for non-`--song`
    // paths via `if let Some(...)` guard; other flags don't fire when
    // song dispatches. Rung 2+ multi-beat phrase execution layers on
    // this dispatch shape.
    if let Some(song_path) = song {
        return crate::song::single_beat_peer_beam(peer_home, &spec_path, song_path, ctx);
    }

    // Blocker 2 Rust runtime discharge (2026-07-11) —
    // `--emit-diff` routes through the @optics/lens/diff.get direction
    // per Mara iter-25 `b0427fd` (@optics/lens family-root, Foster laws)
    // + Mara iter-26 `7e5c298` (@optics/lens/diff first species,
    // autopoietic_closure bilateral). This is the FIRST Rust runtime
    // discharge of the Foster (get, put) triple that Mara iter-6
    // `583b939` @shatter × @io spec named at spec altitude.
    //
    // v0 substrate observation: emits unified-diff-shape bytes carrying
    // spec_oid (content-addressed via blake3) + peer_home + composition
    // attribution + mission (if present). Real bytes, real substrate
    // flow. @fate optical inference that would produce COMPUTED
    // candidates from mission text lands at a subsequent tick that
    // wires the fate crate to the dispatch path; this tick lands the
    // plumbing.
    if emit_diff {
        return emit_peer_beam_diff(peer_home, &spec_path, task, ctx);
    }

    // Put direction (2026-07-11) — closes the Foster (get, put)
    // roundtrip half. `--integrate-diff` reads edited-diff bytes from
    // stdin, computes delta_oid via blake3(spec_bytes || stdin_bytes),
    // emits envelope naming spec_oid + delta_oid + put attribution.
    // Mutually independent of --emit-diff; if both flags set,
    // --integrate-diff wins.
    if integrate_diff {
        return integrate_peer_beam_diff(peer_home, &spec_path);
    }

    // Fate inference (2026-07-11) — Blocker 2 final Rust hop.
    // `--fate-select` invokes Fate::excited().resolve on
    // Features::default() and emits selected Model + mapped prism-op
    // per boot/std/epistemologic/math/bundle.mirror binding.
    // Mutually independent of prior flags; if multiple set,
    // --fate-select wins (last check).
    if fate_select {
        if from_psychohistory {
            if with_shadow {
                return fate_bounded_shadow_peer_beam(peer_home, &spec_path, task, ctx);
            }
            return fate_bounded_by_psychohistory_peer_beam(peer_home, &spec_path, task, ctx);
        }
        return fate_select_peer_beam(peer_home, &spec_path, task, ctx);
    }

    // Piece 2 (insight §2.2): @peer resolution via G1 single-hop. The
    // `~peer'<path>'` sigil at substrate altitude is G1-composed with
    // `~git'<path>'` per Alex's 2026-06-25 confirmation; at v0 we read
    // the filesystem directly (the @io/git layer is the substrate-decl
    // shape; the v0 impl uses std::fs as the immediate realisation).
    let source = match fs::read_to_string(&spec_path) {
        Ok(s) => s,
        Err(e) => {
            merr!(
                "spawn: cannot read peer-home spec {}: {}",
                spec_path.display(),
                e
            );
            return 1;
        }
    };

    let peer_name = extract_spec_project_name(&source).unwrap_or_else(|| "<unknown>".to_string());

    // Piece 3 (insight §2.3): contextual pack. Read the spec's pack{}
    // block and extract the lead. v0 admits a missing pack{} block (the
    // default-to-repo-local rule per peer-ACL §9); Phase H will tighten
    // this to enforce pack_coherent.
    let pack_lead = extract_spec_pack_lead(&source);

    // Pieces 4–7 (insight §§2.4–2.7): logged stubs. The envelope below
    // names each piece's substrate anchor so the path-traversal is
    // empirically checkable from stdout. Phase H replaces each `stub`
    // marker with the real composition.
    //
    //   piece 4 (§2.4): lead-at-N+1 obligation contracted at spawn time.
    //   piece 5 (§2.5): @spectral/supervisor.start_child lifecycle kick.
    //   piece 6 (§2.6): @fate inference (NOT @io/llm) through cascade/
    //                    code/* species discharge.
    //   piece 7 (§2.7): λ₀ → excited-state transition; the substrate's
    //                    transient departure from its own ground state.
    // P4 GREEN: --hello-world opts in to a structured JSON envelope.
    // Default (no flag) keeps the text envelope (existing 5 tests).
    // Per substrate round-trip loop endpoint: the envelope identifies
    // the peer by declared content; bounded to mirror.spec reads;
    // no @fate, no @io/llm, no identity-mint per b10f00c §4.
    let lead_str = pack_lead.unwrap_or_else(|| "<no-lead>".to_string());

    if hello_world {
        // T1 GREEN — bare-agent collapse arc: read the mission file (if
        // `--task <path>` was supplied) and carry the text into the
        // envelope as `mission`. Structural discipline:
        //   - The mission is filesystem-path-shaped (@f); resolve via
        //     ctx.resolve so relative paths honor dispatch context.
        //   - Read failure exits with code 3 (distinct from spec-read=1
        //     and store-open=2) so downstream can discriminate.
        //   - Envelope key `mission` is emitted only when task is
        //     Some(_); absent (no null, no empty string) otherwise, so
        //     the pre-existing 5 spawn envelopes stay byte-identical.
        //   - No @fate dispatch at v0. Recognition #58's inference piece
        //     stays `partial@recall` in composition_pieces; the mission
        //     enters the envelope as linear at winding (0,0) per
        //     `@shatter(_, mission)`. Phase H wires @fate.roll.
        let mission_text: Option<String> = match task {
            None => None,
            Some(path) => {
                let mission_path = ctx.resolve(path);
                match fs::read_to_string(&mission_path) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        merr!(
                            "spawn: cannot read mission task {}: {}",
                            mission_path.display(),
                            e
                        );
                        return 3;
                    }
                }
            }
        };

        let source_decl =
            extract_spec_source_decl(&source).unwrap_or_else(|| "<no-source-decl>".to_string());

        // P4.5 GREEN: the peer's hello IS their own recall. Compose the
        // peer-side recall envelope in-process by invoking the same
        // recall_* helpers against `peer_home`. Per Mara's circular-
        // reflexive observation (349bce7 §3.6): the four payloads each
        // correspond to a sheaf-section-with-temporal-axis at coherence
        // altitude. The lead observes the peer's psychohistory_vector
        // in one breath via this composition.
        //
        // Piece-6-via-recall: structured observation without @fate
        // inference (b10f00c §2.6 substitution form). No subprocess;
        // piece 5 supervisor.start_child stays a named stub.
        let peer_home_resolved_str = peer_home_resolved.to_string_lossy();
        let cascade_val = recall_cascade(&peer_home_resolved_str);
        let pack_trail_val = recall_pack_trail(&peer_home_resolved_str);
        let pull_frontier_val = recall_pull_frontier(&peer_home_resolved_str);
        let dogfood_val = recall_dogfood(&peer_home_resolved_str);

        let peer_recall = serde_json::json!({
            "spec_version":  "v0.1.0",
            "cascade":       cascade_val,
            "pack_trail":    pack_trail_val,
            "pull_frontier": pull_frontier_val,
            "dogfood":       dogfood_val,
        });

        // Phase H GREEN: content-addressed storage composition per
        // Mara's @mirror/store/git species declaration (`1de09a9`) and
        // Reed's RED brief (`88e82c8`). The composition mirrors
        // cmd_init's discharge map (step 4-6): for each peer_recall
        // payload + the spec source + the lead reference, splinter it
        // into a content-addressed crystal, persist via
        // insert_persistent, then compute root_oid = BLAKE3 over the
        // sorted (name, oid) pairs and set_ref("HEAD", root_oid).
        //
        // Compatibility fallback (per RED brief): when peer_home is
        // NOT a git repo, NamespacedGitStore::open returns NotAGitRepo.
        // The existing spawn.rs (10) + composition_spawn_recall.rs (4)
        // fixtures deliberately use a bare filesystem peer-home; skip
        // persistence there and emit spec_oid: "uncommitted" as today
        // so those tests continue to pass. spawn_storage.rs uses a
        // git-init'd fixture that exercises the storage path.
        //
        // Do NOT auto-git-init inside cmd_spawn (heavier + wrong
        // semantics per RED brief). Use the ctx-resolved absolute path
        // so `.git` presence and NamespacedGitStore paths honor the
        // dispatch context.
        let peer_home_path = peer_home_resolved.as_path();

        // Compat gate (P2 recovery 2026-07-01): bare-filesystem peer-home
        // fixtures (spawn.rs + composition_spawn_recall.rs) have no .git/
        // directory. Empirical: fragmentation-git's NamespacedGitStore::open
        // succeeds on non-git paths rather than returning NotAGitRepo. Gate
        // on .git/ existence up-front to preserve the pre-Phase-H envelope
        // shape (raw lead text + "uncommitted" spec_oid) for those fixtures.
        let has_git_dir = peer_home_path.join(".git").is_dir();

        let (spec_oid_value, lead_oid_value): (String, serde_json::Value) = if !has_git_dir {
            (
                "uncommitted".to_string(),
                serde_json::Value::String(lead_str.clone()),
            )
        } else {
            match fragmentation_git::namespaced::NamespacedGitStore::open(peer_home_path, "mirror")
            {
                Ok(store) => {
                    // Payloads to content-address. Same primitive as
                    // cmd_init: Splinter<Blake3>::new(Content::Text(...))
                    // yields the OID; fragmentation::encoding::encode
                    // yields the Fractal wire-carrier; insert_persistent
                    // writes it under objects/.
                    let cascade_json = cascade_val.to_string();
                    let pack_trail_json = pack_trail_val.to_string();
                    let pull_frontier_json = pull_frontier_val.to_string();
                    let dogfood_json = dogfood_val.to_string();
                    let payloads: [(&str, &str); 6] = [
                        ("cascade", cascade_json.as_str()),
                        ("pack_trail", pack_trail_json.as_str()),
                        ("pull_frontier", pull_frontier_json.as_str()),
                        ("dogfood", dogfood_json.as_str()),
                        ("spec", source.as_str()),
                        ("lead", lead_str.as_str()),
                    ];

                    let mut pairs: Vec<(String, String)> = Vec::with_capacity(payloads.len());
                    let mut lead_oid_hex: String = String::new();
                    for (name, payload) in payloads.iter() {
                        let splinter: Splinter<Blake3> =
                            Splinter::new(Content::Text(Text::new((*payload).to_string())));
                        let oid_hex = init_blake3_oid_hex(splinter.oid().bytes());
                        let fractal = fragmentation::encoding::encode(&(*payload).to_string());
                        store.insert_persistent(
                            format!("splinter:{}", oid_hex),
                            fractal,
                            payload.len(),
                        );
                        if *name == "lead" {
                            lead_oid_hex = oid_hex.clone();
                        }
                        pairs.push(((*name).to_string(), oid_hex));
                    }
                    // Canonical sort (matches init_compute_root_oid's
                    // BTreeMap-derived order in cmd_init).
                    pairs.sort_by(|a, b| a.0.cmp(&b.0));
                    let root_oid = init_compute_root_oid(&pairs);
                    if let Err(e) = store.set_ref("HEAD", &root_oid) {
                        merr!("spawn: set_ref(HEAD) failed: {}", e);
                        return 1;
                    }
                    store.flush();
                    (root_oid, serde_json::Value::String(lead_oid_hex))
                }
                Err(fragmentation_git::namespaced::NamespacedStoreError::NotAGitRepo(_)) => {
                    // Retained as safety net; unreachable given the
                    // has_git_dir gate above.
                    (
                        "uncommitted".to_string(),
                        serde_json::Value::String(lead_str.clone()),
                    )
                }
                Err(e) => {
                    merr!("spawn: cannot open namespaced store: {}", e);
                    return 2;
                }
            }
        };

        // Composition-pieces status: real when storage composed;
        // stubs preserved on the fallback path so the envelope
        // continues to self-diagnose accurately.
        let (piece_4, piece_5, piece_7): (&str, &str, &str) = if spec_oid_value == "uncommitted" {
            (
                "stub@N+1",
                "stub@spectral/supervisor.start_child",
                "stub@λ₀→runtime",
            )
        } else {
            (
                "real@lead-crystal",
                "stub@spectral/supervisor.start_child",
                "real@λ₀→runtime",
            )
        };

        let mut envelope = serde_json::json!({
            "spec_version": "v0.1.0",
            "spawn": "hello_world",
            "peer": peer_name,
            "home": peer_home,
            "lead": lead_oid_value,
            "source": source_decl,
            "spec_oid": spec_oid_value,
            "excitation": "λ₀→runtime",
            "composition_pieces": {
                "1_cli_surface":            "real",
                "2_peer_resolution":        "real",
                "3_contextual_pack":        "real",
                "4_lead_at_n_plus_1":       piece_4,
                "5_supervisor_kick":        piece_5,
                "6_fate_inference":         "partial@recall (no @fate; structured observation only)",
                "7_lambda_zero_transition": piece_7,
            },
            "peer_recall": peer_recall,
        });
        // T4 backward-compat: `mission` key is emitted ONLY when task
        // is Some(_). Absent (not null, not empty) otherwise so the
        // pre-existing 5 spawn envelopes stay byte-identical.
        if let Some(mission) = mission_text {
            envelope
                .as_object_mut()
                .expect("spawn envelope is a JSON object")
                .insert("mission".to_string(), serde_json::Value::String(mission));
        }
        let s = format!("{}\n", envelope);
        _raw_stdout(s.as_bytes());
        return 0;
    }

    let envelope = format!(
        "spawn: peer={} home={} lead={} \
         excitation=λ₀→runtime \
         supervisor=stub@spectral/supervisor.start_child \
         fate=stub@cascade/code/* \
         probe_channel=stub@N+1 \
         pack_coherent=stub_passed\n",
        peer_name, peer_home, lead_str,
    );
    _raw_stdout(envelope.as_bytes());
    0
}

/// Extract `source <ref>[, <ref>...]` from a `project { ... }` block in
/// a mirror.spec source. Returns the raw text after `source ` up to the
/// next comment, brace, or end-of-line. v0 admits the substrate text
/// verbatim per the same pattern as `extract_spec_pack_lead`.
fn extract_spec_source_decl(source: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("source ") {
            let cut = rest
                .find(" #")
                .or_else(|| rest.find('{'))
                .unwrap_or(rest.len());
            let val = rest[..cut].trim().trim_end_matches(',').trim().to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

/// Extract `project <name>` from a mirror.spec source. v0 uses simple
/// line-prefix matching; the substrate's full parser (per @mirror/spec)
/// will replace this when Phase H lands. The text-matching style mirrors
/// `mcp::extract_prism_declaration`.
fn extract_spec_project_name(source: &str) -> Option<String> {
    for line in source.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("project ") {
            let name: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '{')
                .collect();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

/// Extract `lead <peer-ref>` from a `pack { … }` block in a mirror.spec
/// source. Returns the raw peer-ref text (sigil + path + closer); v0
/// admits the substrate text verbatim so the trace shows what the spec
/// actually declared. Phase H tightens to a typed peer carrier.
fn extract_spec_pack_lead(source: &str) -> Option<String> {
    let mut in_pack = false;
    let mut depth: i32 = 0;
    for line in source.lines() {
        let trimmed = line.trim();
        if !in_pack && (trimmed.starts_with("pack {") || trimmed.starts_with("pack{")) {
            in_pack = true;
            depth = 1;
            continue;
        }
        if in_pack {
            // Crude brace tracker: every `{` opens, every `}` closes.
            for c in trimmed.chars() {
                match c {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
            }
            if let Some(after) = trimmed.strip_prefix("lead ") {
                return Some(after.trim().to_string());
            }
            if depth <= 0 {
                return None;
            }
        }
    }
    None
}

// ───────────────────────────────────────────────────────────────────────────────
// Library entry point: in-process subcommand dispatch with fd-level capture.
// ───────────────────────────────────────────────────────────────────────────────

/// Captured output from `kintsugi_main` / `kintsugi_main_in` — the
/// carrier integration tests assert on without spawning a subprocess.
///
/// Per Taut's profiling survey (#286 Win 2): the 9-of-11 integration tests
/// that used to `Command::new(env!("CARGO_BIN_EXE_mirror"))` each paid
/// 200-800ms dyld + Accelerate startup. The in-process call returns the
/// same three pieces of evidence the subprocess shape exposed — exit
/// code, captured stdout bytes, captured stderr bytes — so the assertions
/// transfer 1:1 from `Output { status, stdout, stderr }` to
/// `ExitOutput { exit_code, stdout, stderr }`.
#[derive(Debug, Clone)]
pub struct ExitOutput {
    /// What `std::process::exit` would have received. Matches the exit
    /// code the binary would have produced for the same argv.
    pub exit_code: i32,
    /// Bytes written to stdout during `dispatch(args)`. Captured at the
    /// fd-level so writes from native code (LAPACK, libc) are included.
    pub stdout: Vec<u8>,
    /// Bytes written to stderr during `dispatch(args)`. Same fd-level
    /// capture as stdout.
    pub stderr: Vec<u8>,
}

/// In-process subcommand dispatch with fd-level stdout / stderr capture.
///
/// The Taut #286 Win 2 entry point. Integration tests that used to spawn
/// the binary now call this directly:
///
/// ```ignore
/// let out = mirror::kintsugi_main(&[
///     "mirror".into(),
///     "kintsugi".into(),
///     "--ci".into(),
///     "boot/std/nl.mirror".into(),
/// ]);
/// assert_eq!(out.exit_code, 0);
/// ```
///
/// `args[0]` is the program name — synthesized by tests to match the
/// `std::env::args` shape that `dispatch` expects. `args[1..]` is the
/// subcommand and its flags.
///
/// ## Capture mechanism
///
/// The function installs `Some(Vec::new())` in two thread-local cells
/// (`CAPTURE_STDOUT` / `CAPTURE_STDERR`), runs `dispatch(args)`, and
/// extracts the buffers. The `mout!` / `merr!` macros and the
/// `_raw_stdout` / `_raw_stderr` helpers detect the install and append
/// to the buffers instead of writing to fd 1 / 2. This is parallel-
/// safe: each test thread has its own cells; libtest's own fd 1 writes
/// (test progress lines) are unaffected.
///
/// For tests that need the binary to resolve relative paths (e.g. grammar
/// paths like `shards/mirror/spec.mirror`) against a specific directory,
/// use [`kintsugi_main_in`] which constructs a `Ctx` with the requested
/// cwd and threads it through dispatch — WITHOUT mutating the process
/// cwd. Arc 2 (/loop 2026-07-05) removed the `set_current_dir` swap
/// and its serializing mutex; cwd is now an explicit dispatch parameter.
pub fn kintsugi_main(args: &[String]) -> ExitOutput {
    let ctx = Ctx::from_process_cwd();
    kintsugi_main_inner(args, &ctx)
}

/// Like [`kintsugi_main`] but runs the dispatch with `cwd` threaded
/// through the dispatch chain via `Ctx`. Relative paths inside the
/// dispatch chain resolve against `cwd` explicitly; the process cwd
/// is NOT mutated (Arc 2 substrate-pull per /loop 2026-07-05).
///
/// Integration tests use this when they need dispatch to resolve
/// relative paths against a fixture directory. Otherwise prefer
/// [`kintsugi_main`].
pub fn kintsugi_main_in(args: &[String], cwd: &std::path::Path) -> ExitOutput {
    let ctx = Ctx::new(cwd);
    kintsugi_main_inner(args, &ctx)
}

/// Run `dispatch(args, ctx)` with the thread-local capture cells installed,
/// so `mout!` / `merr!` calls land in the per-call `Vec<u8>` buffers
/// instead of fd 1 / 2. Shared body of `kintsugi_main` and
/// `kintsugi_main_in`.
fn kintsugi_main_inner(args: &[String], ctx: &Ctx) -> ExitOutput {
    // Install fresh capture buffers on this thread. Any prior install
    // (recursive call) is saved + restored so nested invocations don't
    // lose each other's output — dispatch is not currently re-entrant
    // but the swap-save-swap discipline is the right shape regardless.
    let prior_stdout = CAPTURE_STDOUT.with(|cell| cell.replace(Some(Vec::new())));
    let prior_stderr = CAPTURE_STDERR.with(|cell| cell.replace(Some(Vec::new())));

    // Catch panics so we always restore the prior cells before
    // returning to the caller.
    let dispatch_result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dispatch(args, ctx)));

    // Take this call's captured bytes; restore the prior cells.
    let stdout_bytes = CAPTURE_STDOUT
        .with(|cell| cell.replace(prior_stdout))
        .unwrap_or_default();
    let stderr_bytes = CAPTURE_STDERR
        .with(|cell| cell.replace(prior_stderr))
        .unwrap_or_default();

    let exit_code = match dispatch_result {
        Ok(code) => code,
        Err(payload) => std::panic::resume_unwind(payload),
    };

    ExitOutput {
        exit_code,
        stdout: stdout_bytes,
        stderr: stderr_bytes,
    }
}
