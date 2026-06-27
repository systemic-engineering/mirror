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

pub mod ast;
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
pub mod sheaf_laplacian;
pub mod spectral;
pub mod tensor;
pub mod tokenize;

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;

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
use crate::grammar::{grammar_for_file, load_grammar};
use crate::hash::canonical_hash;
use crate::pipeline::{
    apply_rewrites, execute_pipeline, is_mq_query, parse_rewrite, split_pipeline,
};
use crate::spectral::{compute_content_oid, render_ast};
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
    merr!("commands: compile [--strict] <file>, craft [--strict] [--target <crystal|binary>] <target>, kintsugi [--ci [--out @data/json|@data/mirror|@io/dir('path')]] [--shatter N] <file|dir>");
    merr!("examples:");
    merr!("  cat mirror.ll | mirror '@code/llvm/ir |> @mirror/kintsugi |> @mirror/butterfly'");
}

fn read_stdin_all() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

fn cmd_compile(file: &str, no_cache: bool, strict: bool) -> i32 {
    let source = match fs::read(file) {
        Ok(s) => s,
        Err(e) => {
            merr!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
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

fn cmd_craft_with(target: &str, no_cache: bool, kind: TargetKind, strict: bool) -> i32 {
    let mut files: Vec<String> = Vec::new();
    match target {
        "boot" | "std" => collect_files("boot", ".mirror", &mut files),
        "cargo" => collect_files("src", ".rs", &mut files),
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
        let grammar_path = grammar_for_file(file);
        let grammar = match load_grammar(grammar_path) {
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
        return build_self_binary();
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
fn build_self_binary() -> i32 {
    merr!("== craft --target binary ==");
    merr!("1/3 cargo rustc --emit=llvm-ir");

    let status = Command::new("cargo")
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
    let deps_dir = PathBuf::from(&target_dir).join("release").join("deps");
    let ll_path = match find_bootstrap_ll(&deps_dir) {
        Some(p) => p,
        None => {
            merr!("could not find mirror-*.ll under {}", deps_dir.display());
            return 1;
        }
    };
    merr!("    found: {}", ll_path.display());

    let dest = PathBuf::from("bootstrap/mirror.ll");
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
    let status = Command::new("clang")
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

#[allow(dead_code)]
fn dump_ast(node: &crate::ast::AstNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind_str = format!("{:?}", node.kind);
    let body_marker = node
        .body
        .as_deref()
        .map(|b| format!(" body={:?}", b))
        .unwrap_or_default();
    let kw_marker = if node.keyword.is_empty() {
        String::new()
    } else {
        format!(" kw={}", node.keyword)
    };
    let tag_marker = if node.grammar_tag.is_empty() {
        String::new()
    } else {
        format!(" tag={}", node.grammar_tag)
    };
    merr!(
        "{}{} name={:?}{}{}{} oid={}",
        indent,
        kind_str,
        node.name,
        kw_marker,
        tag_marker,
        body_marker,
        compute_content_oid(node)
    );
    for c in &node.children {
        dump_ast(c, depth + 1);
    }
}

#[allow(dead_code)]
fn cmd_dump(file: &str) -> i32 {
    let source = match fs::read(file) {
        Ok(s) => s,
        Err(e) => {
            merr!("cannot read {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(_) => return 1,
    };
    let ast = tokenize(&source, &grammar);
    dump_ast(&ast, 0);
    0
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
        return cmd_kintsugi_spec(file, format);
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
        if let Ok(md) = fs::metadata(file) {
            if md.is_dir() {
                return cmd_kintsugi_ci_corpus(file, shatter, transform, format);
            }
        }
        return cmd_kintsugi_ci_single(file, shatter, transform, format, out_dir);
    }
    // Migration mode: --out + a directory input — walk and migrate
    // every .mirror file under the directory.
    if let Some(out_root) = out_dir {
        let md = match fs::metadata(file) {
            Ok(m) => m,
            Err(e) => {
                merr!("cannot stat {}: {}", file, e);
                return 1;
            }
        };
        if md.is_dir() {
            return cmd_kintsugi_migrate(file, out_root, transform);
        }
    }
    cmd_kintsugi_single(file, shatter, transform, out_dir)
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
fn cmd_kintsugi_spec(spec_path: &str, format: CiFormat) -> i32 {
    let source = match fs::read(spec_path) {
        Ok(s) => s,
        Err(_) => {
            return emit_spec_verdict("failure", spec_path, 0.0, 1, 0, 0, &[], format);
        }
    };
    // Substrate-pull dispatch: the file extension picks the grammar,
    // the grammar loader merges in keyword companions, the tokenizer
    // produces an AST. `parse_spec_targets`'s text scanner retires.
    let grammar_path = grammar_for_file(spec_path);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(_) => {
            return emit_spec_verdict("failure", spec_path, 0.0, 1, 0, 0, &[], format);
        }
    };
    let ast = tokenize(&source, &grammar);
    let targets = spec_targets_from_ast(&ast);

    // Spec-relative root for the default `Cargo.toml` location. The
    // substrate-declared `manifest ~f'...'` override isn't captured
    // by the bootstrap tokenizer today (substrate gap; see
    // `spec_targets_from_ast` docstring); every target uses the
    // spec-dir default.
    let spec_dir: PathBuf = std::path::Path::new(spec_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

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
        let mut cmd = Command::new("cargo");
        cmd.args(cargo_args).arg("--manifest-path").arg(&manifest);
        let out = cmd.output();
        let cargo_pretty = cargo_args.join(" ");
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

fn parse_ci_format(s: &str) -> Option<CiFormat> {
    match s {
        "mirror" | "mirror-text" => Some(CiFormat::MirrorText),
        "json" => Some(CiFormat::Json),
        _ => None,
    }
}

/// Map a substrate ref (`@<namespace>`) to a CiFormat at the @io
/// boundary. Per Alex's 2026-06-16 substrate-pull insight: `--out` should
/// accept substrate refs (the `out` keyword's value space IS the
/// substrate namespace, not a closed enum of bare strings). The receiver
/// glasses live at `shards/mirror/data/<X>.mirror` (Mara's T16 cascade);
/// this hardcoded dispatch is the @io floor that a future tick lifts to
/// a substrate-driven registry via the cross-shard resolver.
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
) -> (&'static str, f64, u64, u64) {
    let source = match fs::read(file) {
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
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
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
) -> i32 {
    let (verdict, objective, iterations, dark_count) =
        kintsugi_ci_compute(file, shatter, transform);
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
fn collect_declared_namespaces(dir: &str, out: &mut std::collections::HashSet<String>) {
    let mut files: Vec<String> = Vec::new();
    collect_files(dir, ".mirror", &mut files);
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
            // Match `glass `, `prism `, `grammar ` prefixes.
            let rest = if let Some(r) = line.strip_prefix("glass ") {
                r
            } else if let Some(r) = line.strip_prefix("prism ") {
                r
            } else if let Some(r) = line.strip_prefix("grammar ") {
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
fn count_unresolved_imports(file: &str, declared: &std::collections::HashSet<String>) -> u64 {
    let source = match fs::read_to_string(file) {
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
) -> i32 {
    let mut files: Vec<String> = Vec::new();
    collect_files(dir, ".mirror", &mut files);
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
    collect_declared_namespaces(dir, &mut declared);
    if std::path::Path::new("shards").is_dir() {
        collect_declared_namespaces("shards", &mut declared);
    }
    if std::path::Path::new("boot/std").is_dir() {
        collect_declared_namespaces("boot/std", &mut declared);
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
            kintsugi_ci_compute(path, shatter, transform);
        // Cross-shard semantic resolution: count unresolved `in @<path>`
        // statements as additional dark regions and downgrade the
        // per-file verdict to `failure` when any unresolved import is
        // found. A dangling cross-shard import is a structural break,
        // not a soft `partial` — the dependency graph has no edge to
        // close.
        let unresolved = count_unresolved_imports(path, &declared);
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
fn cmd_kintsugi_migrate(src_root: &str, out_root: &str, transform: Option<&str>) -> i32 {
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
    collect_files(src_root, ".mirror", &mut files);
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
        let rel = path
            .strip_prefix(src_root)
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
        let dest = format!("{}/{}", out_root.trim_end_matches('/'), rel_out);
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
) -> i32 {
    let source = match fs::read(file) {
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
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
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
    render_ast(&ast, 0, &mut out);

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
pub fn dispatch(args: &[String]) -> i32 {
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
        let source = match read_stdin_all() {
            Ok(s) => s,
            Err(e) => {
                merr!("stdin read error: {}", e);
                return 1;
            }
        };
        return execute_pipeline(&segs, &source);
    }

    // Path B: file + mq query
    if args.len() == 3 && is_mq_query(&args[2]) {
        let segs = split_pipeline(&args[2]);
        if segs.is_empty() {
            merr!("empty query");
            return 1;
        }
        let source = match fs::read(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                merr!("cannot read {}: {}", args[1], e);
                return 1;
            }
        };
        return execute_pipeline(&segs, &source);
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
    let mut i = 2;
    while i < args.len() {
        let a = &args[i];
        if a == "--no-cache" {
            no_cache = true;
        } else if a == "--strict" {
            strict = true;
        } else if a == "--target" {
            if i + 1 >= args.len() {
                merr!("--target requires a value (crystal|binary|rust|gleam)");
                return 1;
            }
            match parse_target(&args[i + 1]) {
                Some(k) => target_kind = k,
                None => {
                    merr!("unknown --target value: {}", args[i + 1]);
                    return 1;
                }
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--target=") {
            match parse_target(rest) {
                Some(k) => target_kind = k,
                None => {
                    merr!("unknown --target value: {}", rest);
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
            if a == "--target" || a == "--shatter" || a == "--transform" || a == "--out" {
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

    let rc = match args[1].as_str() {
        "compile" => match positional {
            Some(p) => cmd_compile(p, no_cache, strict),
            None => {
                merr!("usage: mirror compile [--strict] <file>");
                1
            }
        },
        "craft" => match positional {
            Some(p) => cmd_craft_with(p, no_cache, target_kind, strict),
            None => {
                merr!("usage: mirror craft [--strict] [--target <crystal|binary>] <target>");
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
            ),
            None => {
                merr!("usage: mirror kintsugi [--ci [--out @data/json|@data/mirror|@io/dir('path')]] [--shatter N] [--transform <mq>] <file|dir>");
                1
            }
        },
        "recall" => match positional {
            Some(p) => cmd_recall(p),
            None => {
                merr!("usage: mirror recall <spec-dir>");
                1
            }
        },
        "spawn" => match positional {
            Some(p) => {
                let hello_world = args.iter().any(|a| a == "--hello-world");
                cmd_spawn(p, hello_world)
            }
            None => {
                merr!("usage: mirror spawn <peer-home> [--hello-world]");
                1
            }
        },
        other => {
            merr!("unknown: {}", other);
            1
        }
    };
    rc
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
fn cmd_recall(spec_dir: &str) -> i32 {
    // Edge case: missing/invalid directory returns non-zero.
    let dir_path = std::path::Path::new(spec_dir);
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

    // Compose the four payloads per Mara spec §3 (b034a60).
    let cascade = recall_cascade(spec_dir);
    let pack_trail = recall_pack_trail(spec_dir);
    let pull_frontier = recall_pull_frontier(spec_dir);
    let dogfood = recall_dogfood(spec_dir);

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

    let mut records: Vec<(i64, serde_json::Value)> = by_peer
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
            (acc.last_at, v)
        })
        .collect();

    records.sort_by(|a, b| b.0.cmp(&a.0));
    let values: Vec<serde_json::Value> = records.into_iter().map(|(_, v)| v).collect();
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
fn cmd_spawn(peer_home: &str, hello_world: bool) -> i32 {
    // Piece 1 (insight §2.1): cli surface. The peer-home argument is the
    // single positional. Context (frame, repository, pack) is resolved
    // FROM peer-home, not passed in.
    let spec_path = std::path::Path::new(peer_home).join("mirror.spec");

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
        let peer_recall = serde_json::json!({
            "spec_version":  "v0.1.0",
            "cascade":       recall_cascade(peer_home),
            "pack_trail":    recall_pack_trail(peer_home),
            "pull_frontier": recall_pull_frontier(peer_home),
            "dogfood":       recall_dogfood(peer_home),
        });

        let envelope = serde_json::json!({
            "spec_version": "v0.1.0",
            "spawn": "hello_world",
            "peer": peer_name,
            "home": peer_home,
            "lead": lead_str,
            "source": source_decl,
            "spec_oid": "uncommitted",
            "excitation": "λ₀→runtime",
            "composition_pieces": {
                "1_cli_surface":            "real",
                "2_peer_resolution":        "real",
                "3_contextual_pack":        "real",
                "4_lead_at_n_plus_1":       "stub@N+1",
                "5_supervisor_kick":        "stub@spectral/supervisor.start_child",
                "6_fate_inference":         "partial@recall (no @fate; structured observation only)",
                "7_lambda_zero_transition": "stub@λ₀→runtime",
            },
            "peer_recall": peer_recall,
        });
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

/// Process-wide serialization for `kintsugi_main_in`'s cwd swap. The cwd
/// is a process-wide resource (`std::env::set_current_dir`), so two
/// concurrent test threads calling `kintsugi_main_in` would race on it.
/// The fd 1 / 2 capture is parallel-safe (per-thread thread-local cells
/// in `CAPTURE_STDOUT` / `CAPTURE_STDERR`), so `kintsugi_main` itself
/// does NOT need to take this lock; only the cwd-setting variant does.
fn kintsugi_main_lock() -> &'static std::sync::Mutex<()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
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
/// For tests that need the binary to run with a specific working
/// directory (the mirror binary loads grammars via paths relative to
/// its cwd), use [`kintsugi_main_in`] which performs the cwd swap
/// under a package-internal mutex.
pub fn kintsugi_main(args: &[String]) -> ExitOutput {
    kintsugi_main_inner(args)
}

/// Like [`kintsugi_main`] but runs the dispatch with the process working
/// directory temporarily set to `cwd`. The cwd change is process-wide,
/// so this function holds a package-internal mutex around the
/// set_current_dir / dispatch / restore sequence; concurrent tests under
/// `cargo test` (default parallelism) take the mutex one at a time.
/// The original cwd is restored before this function returns.
///
/// Integration tests use this when they need the binary to resolve
/// grammar paths relative to the repo root rather than the test process
/// cwd. Otherwise prefer [`kintsugi_main`].
pub fn kintsugi_main_in(args: &[String], cwd: &std::path::Path) -> ExitOutput {
    let _guard = kintsugi_main_lock()
        .lock()
        .unwrap_or_else(|p| p.into_inner());

    let saved_cwd = std::env::current_dir().ok();
    let _ = std::env::set_current_dir(cwd);

    let out = kintsugi_main_inner(args);

    if let Some(prev) = saved_cwd {
        let _ = std::env::set_current_dir(prev);
    }
    out
}

/// Run `dispatch(args)` with the thread-local capture cells installed,
/// so `mout!` / `merr!` calls land in the per-call `Vec<u8>` buffers
/// instead of fd 1 / 2. Shared body of `kintsugi_main` and
/// `kintsugi_main_in`.
fn kintsugi_main_inner(args: &[String]) -> ExitOutput {
    // Install fresh capture buffers on this thread. Any prior install
    // (recursive call) is saved + restored so nested invocations don't
    // lose each other's output — dispatch is not currently re-entrant
    // but the swap-save-swap discipline is the right shape regardless.
    let prior_stdout = CAPTURE_STDOUT.with(|cell| cell.replace(Some(Vec::new())));
    let prior_stderr = CAPTURE_STDERR.with(|cell| cell.replace(Some(Vec::new())));

    // Catch panics so we always restore the prior cells before
    // returning to the caller.
    let dispatch_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dispatch(args)));

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
