//! mirror — the native binary, Rust port.
//!
//! Bit-exact CoincidenceHash<3> + content-OID compatibility with the C
//! original at native/mirror.c. The body-capture fix for LLVM IR keyword
//! forms (target datalayout = "...", source_filename = "...") is shared.
//! Content OIDs are computed by `spectral::compute_content_oid`, which
//! dispatches the recursive AST walk through
//! `prism_core::apply_h(&ContentOidPrism, ast)` per
//! `docs/specs/bootstrap-retirement-plan.md` Tick 1.

mod ast;
mod crystallize;
mod curvature;
mod exec;
mod gap;
mod git;
mod grammar;
mod hash;
mod kintsugi;
mod music;
mod oscillate;
mod pipeline;
mod property;
mod sheaf_laplacian;
mod spectral;
mod tensor;
mod tokenize;

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use prism_core::{Optic, Ref};
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

    eprintln!("  --> line {}, col {}", line, col);
    eprintln!("   {} |", pad);
    eprintln!("   {} | {}", line_str, src_line);
    // Caret line: spaces up to (col-1), then ^^^.
    let leading = (col as usize).saturating_sub(1);
    let mut caret_line = String::with_capacity(leading + caret_width);
    for _ in 0..leading {
        caret_line.push(' ');
    }
    for _ in 0..caret_width {
        caret_line.push('^');
    }
    eprintln!("   {} | {}", pad, caret_line);
    eprintln!("   {} |", pad);
    eprintln!(
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
    eprintln!(
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
    eprintln!("usage:");
    eprintln!("  mirror <command> [args...]            (legacy subcommand surface)");
    eprintln!("  mirror '<mq-query>' < input           (mq pipeline over stdin)");
    eprintln!("  mirror <input> '<mq-query>'           (mq pipeline over input file)");
    eprintln!("commands: compile [--strict] <file>, craft [--strict] [--target <crystal|binary>] <target>, kintsugi [--ci [--format mirror|json]] [--shatter N] <file|dir>");
    eprintln!("examples:");
    eprintln!("  cat mirror.ll | mirror '@code/llvm/ir |> @mirror/kintsugi |> @mirror/butterfly'");
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
            eprintln!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("cannot read grammar {}: {}", grammar_path, e);
            return 1;
        }
    };

    // --strict bypasses the OID cache: we need the AST to count Dark.
    if !no_cache && !strict {
        let source_oid = canonical_hash(&source);
        if let Some(cached) = git_crystal_exists(&source_oid) {
            eprintln!("(cached)");
            println!("{}", cached);
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
    println!("{}", oid);
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
            eprintln!("unknown target: {}", target);
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
                eprintln!("  skip {} (grammar error)", file);
                continue;
            }
        };
        let source = match fs::read(file) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("  skip {} (read error)", file);
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
            eprintln!("  {} -> {} (cached)", file, oid);
        } else {
            eprintln!("  {} -> {}", file, oid);
        }
        hasher_buf.extend_from_slice(oid.as_bytes());
    }

    let crystal = canonical_hash(&hasher_buf);
    if hits > 0 {
        eprintln!("cache: {}/{} hits", hits, total);
    }
    if strict && total_dark > 0 {
        eprintln!(
            "error[total_classification]: {} dark region(s) across {} file(s)",
            total_dark, files_with_dark
        );
        return 2;
    }
    println!("{}", crystal);

    if kind == TargetKind::Binary && (target == "boot" || target == "std") {
        return build_self_binary();
    }
    if kind == TargetKind::Rust || kind == TargetKind::Gleam {
        eprintln!("--target rust/gleam: not yet implemented (declared for surface stability)");
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
    eprintln!("== craft --target binary ==");
    eprintln!("1/3 cargo rustc --emit=llvm-ir");

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
            eprintln!("cargo rustc failed: exit {}", s);
            return 1;
        }
        Err(e) => {
            eprintln!("cargo rustc spawn error: {}", e);
            return 1;
        }
    }

    eprintln!("2/3 locate bootstrap/mirror.ll");
    let target_dir =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "bootstrap/target".to_string());
    let deps_dir = PathBuf::from(&target_dir).join("release").join("deps");
    let ll_path = match find_bootstrap_ll(&deps_dir) {
        Some(p) => p,
        None => {
            eprintln!("could not find mirror-*.ll under {}", deps_dir.display());
            return 1;
        }
    };
    eprintln!("    found: {}", ll_path.display());

    let dest = PathBuf::from("bootstrap/mirror.ll");
    if let Err(e) = fs::copy(&ll_path, &dest) {
        eprintln!(
            "copy {} -> {} failed: {}",
            ll_path.display(),
            dest.display(),
            e
        );
        return 1;
    }
    eprintln!("    copied to {}", dest.display());

    eprintln!("3/3 clang -O2 -o ./mirror-self -x ir bootstrap/mirror.ll -lm");
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
            eprintln!("clang failed: exit {}", s);
            return 1;
        }
        Err(e) => {
            eprintln!("clang spawn error: {}", e);
            return 1;
        }
    }

    eprintln!("== ./mirror-self ==");
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
    eprintln!(
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
            eprintln!("cannot read {}: {}", file, e);
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
            eprintln!("  dispatch {}: Success", tick_ref.as_str());
        }
        Imperfect::Partial(_, _) => {
            eprintln!(
                "  dispatch {}: Partial (with Transparency)",
                tick_ref.as_str()
            );
        }
        Imperfect::Failure(CrystallizeError::Uncrystallized(got), _) => {
            eprintln!(
                "  dispatch {}: Uncrystallized (floor has no body at {})",
                tick_ref.as_str(),
                got.as_str()
            );
        }
        Imperfect::Failure(err, _) => {
            eprintln!("  dispatch {}: Failure ({:?})", tick_ref.as_str(), err);
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
    eprintln!(
        "tick {}  dark_count: {}  loss: {:.1}  \u{0394}: {:.1}{}",
        tick, dark_count, loss, delta, suffix
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
        return cmd_kintsugi_ci_single(file, shatter, transform, format);
    }
    // Migration mode: --out + a directory input — walk and migrate
    // every .mirror file under the directory.
    if let Some(out_root) = out_dir {
        let md = match fs::metadata(file) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("cannot stat {}: {}", file, e);
                return 1;
            }
        };
        if md.is_dir() {
            return cmd_kintsugi_migrate(file, out_root, transform);
        }
    }
    cmd_kintsugi_single(file, shatter, transform, out_dir)
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
                    for field in &target.children {
                        if field.kind == AstKind::Project && field.keyword == "emit" {
                            emit = field.name.clone();
                        }
                    }
                    out.push(SpecTarget {
                        block_name: target.name.clone(),
                        emit,
                    });
                }
            }
        }
    }
    out
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
            // is not yet wired. Mark as failure so the spec walker is
            // honest about what it did NOT dispatch.
            let label = format!("target {} (emit {})", t.block_name, t.emit);
            per_target.push(PerFileVerdict {
                path: label,
                verdict: "failure",
                objective: 1.0,
                iterations: 1,
                dark_count: 1,
            });
            any_failure = true;
            total_objective += 1.0;
            total_dark += 1;
            continue;
        }
        let manifest = spec_dir.join("Cargo.toml");
        // Per the directive's "Minimum viable" §4: `cargo check` for
        // `binary.compiles`-style settle_on predicates. `cargo test` is
        // wired the same way but gated on the substrate's `tests_pass`
        // predicate, which the dispatcher can't read today (the
        // substrate's @epistemologic/property dispatch lands at a later
        // tick). Default to `cargo check`: it's the cheapest exit-code
        // signal that lifts to transparency under D3's contract, and it
        // dominates the runtime budget for `mirror kintsugi mirror.spec`
        // on mirror's own root.
        let out = Command::new("cargo")
            .arg("check")
            .arg("--manifest-path")
            .arg(&manifest)
            .output();
        let (verdict, objective, dark, label_path) = match out {
            Ok(o) if o.status.success() => (
                "success",
                0.0_f64,
                0_u64,
                manifest.to_string_lossy().into_owned(),
            ),
            Ok(o) => {
                // D3's exit-code lift is partial here: non-zero →
                // failure. Stderr is captured but not yet parsed into
                // opacity_map (deferred to @mirror/mosaic). The full
                // exit code is recorded in the label so the operator
                // can see WHICH non-zero arm fired.
                let code = o.status.code().unwrap_or(-1);
                eprintln!(
                    "[kintsugi spec] target `{}` cargo check exit {}; stderr (first 200 chars):",
                    t.block_name, code,
                );
                let stderr = String::from_utf8_lossy(&o.stderr);
                eprintln!("  {}", &stderr.chars().take(200).collect::<String>());
                let label = format!("{} (exit {})", manifest.to_string_lossy(), code);
                ("failure", 1.0_f64, 1_u64, label)
            }
            Err(e) => {
                eprintln!(
                    "[kintsugi spec] target `{}` cargo spawn error: {}",
                    t.block_name, e
                );
                (
                    "failure",
                    1.0_f64,
                    1_u64,
                    manifest.to_string_lossy().into_owned(),
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
                Ok(json) => {
                    let mut out = io::stdout();
                    if writeln!(out, "{}", json).is_err() {
                        return 1;
                    }
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
) -> i32 {
    let (verdict, objective, iterations, dark_count) =
        kintsugi_ci_compute(file, shatter, transform);
    match format {
        CiFormat::MirrorText => {
            emit_ci_verdict_mirror_text(verdict, file, objective, iterations, dark_count)
        }
        CiFormat::Json => emit_ci_verdict_json(verdict, file, objective, iterations, dark_count),
    }
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

    let mut per_file: Vec<PerFileVerdict> = Vec::with_capacity(files.len());
    let mut total_objective: f64 = 0.0;
    let mut total_dark: u64 = 0;
    let mut max_iterations: u64 = 0;
    let mut any_failure = false;
    let mut any_partial = false;
    let mut all_success = true;

    for path in &files {
        let (verdict, objective, iterations, dark_count) =
            kintsugi_ci_compute(path, shatter, transform);
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
                Ok(json) => {
                    let mut out = io::stdout();
                    if writeln!(out, "{}", json).is_err() {
                        return 1;
                    }
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
) -> i32 {
    let v = CiVerdict {
        verdict,
        target,
        objective,
        iterations,
        dark_count,
    };
    match serde_json::to_string(&v) {
        Ok(json) => {
            let mut out = io::stdout();
            if writeln!(out, "{}", json).is_err() {
                return 1;
            }
            0
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
    let mut out = io::stdout();
    if out.write_all(buf.as_bytes()).is_err() {
        return 1;
    }
    0
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
    let mut out = io::stdout();
    if out.write_all(buf.as_bytes()).is_err() {
        return 1;
    }
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
                    eprintln!("kintsugi --transform: not a rewrite query (expected `<sym> => <repl>`): {}", q);
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
                eprintln!("  skip {} (read error: {})", path, e);
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
                eprintln!("  cannot mkdir {}: {}", parent.display(), e);
                errs += 1;
                continue;
            }
        }
        if let Err(e) = fs::write(&dest, &rewritten) {
            eprintln!("  cannot write {}: {}", dest, e);
            errs += 1;
            continue;
        }
        eprintln!("  {} -> {}", path, dest);
    }
    eprintln!("migration: {} file(s), {} error(s)", files.len(), errs);
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
            eprintln!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    // Apply --transform rewrites before tokenize.
    let source = if let Some(q) = transform {
        match parse_rewrite(q) {
            Some(rules) => apply_rewrites(&rules, &source),
            None => {
                eprintln!("kintsugi --transform: not a rewrite query: {}", q);
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

    let mut out = Vec::new();
    render_ast(&ast, 0, &mut out);
    if let Some(dir) = out_dir {
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
            eprintln!("cannot write {}: {}", dest, e);
            return 1;
        }
        eprintln!("wrote {}", dest);
    } else {
        let _ = io::stdout().write_all(&out);
    }
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }

    // Path A: mq query as single argument (stdin input)
    if args.len() == 2 && is_mq_query(&args[1]) {
        let segs = split_pipeline(&args[1]);
        if segs.is_empty() {
            eprintln!("empty query");
            std::process::exit(1);
        }
        let source = match read_stdin_all() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("stdin read error: {}", e);
                std::process::exit(1);
            }
        };
        std::process::exit(execute_pipeline(&segs, &source));
    }

    // Path B: file + mq query
    if args.len() == 3 && is_mq_query(&args[2]) {
        let segs = split_pipeline(&args[2]);
        if segs.is_empty() {
            eprintln!("empty query");
            std::process::exit(1);
        }
        let source = match fs::read(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cannot read {}: {}", args[1], e);
                std::process::exit(1);
            }
        };
        std::process::exit(execute_pipeline(&segs, &source));
    }

    if args.len() < 3 {
        usage();
        std::process::exit(1);
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
                eprintln!("--target requires a value (crystal|binary|rust|gleam)");
                std::process::exit(1);
            }
            match parse_target(&args[i + 1]) {
                Some(k) => target_kind = k,
                None => {
                    eprintln!("unknown --target value: {}", args[i + 1]);
                    std::process::exit(1);
                }
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--target=") {
            match parse_target(rest) {
                Some(k) => target_kind = k,
                None => {
                    eprintln!("unknown --target value: {}", rest);
                    std::process::exit(1);
                }
            }
        } else if a == "--shatter" {
            if i + 1 >= args.len() {
                eprintln!("--shatter requires a non-negative integer");
                std::process::exit(1);
            }
            match args[i + 1].parse::<u64>() {
                Ok(n) => shatter = n,
                Err(_) => {
                    eprintln!(
                        "--shatter requires a non-negative integer, got: {}",
                        args[i + 1]
                    );
                    std::process::exit(1);
                }
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--shatter=") {
            match rest.parse::<u64>() {
                Ok(n) => shatter = n,
                Err(_) => {
                    eprintln!("--shatter requires a non-negative integer, got: {}", rest);
                    std::process::exit(1);
                }
            }
        } else if a == "--transform" {
            if i + 1 >= args.len() {
                eprintln!("--transform requires an mq-query value");
                std::process::exit(1);
            }
            transform = Some(args[i + 1].clone());
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--transform=") {
            transform = Some(rest.to_string());
        } else if a == "--out" {
            if i + 1 >= args.len() {
                eprintln!("--out requires a path value");
                std::process::exit(1);
            }
            out_dir = Some(args[i + 1].clone());
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--out=") {
            out_dir = Some(rest.to_string());
        } else if a == "--ci" {
            ci = true;
        } else if a == "--format" {
            if i + 1 >= args.len() {
                eprintln!("--format requires a value (mirror|json)");
                std::process::exit(1);
            }
            match parse_ci_format(&args[i + 1]) {
                Some(f) => ci_format = f,
                None => {
                    eprintln!(
                        "unknown --format value: {} (expected: mirror|json)",
                        args[i + 1]
                    );
                    std::process::exit(1);
                }
            }
            i += 1;
        } else if let Some(rest) = a.strip_prefix("--format=") {
            match parse_ci_format(rest) {
                Some(f) => ci_format = f,
                None => {
                    eprintln!("unknown --format value: {} (expected: mirror|json)", rest);
                    std::process::exit(1);
                }
            }
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
                || a == "--shatter"
                || a == "--transform"
                || a == "--out"
                || a == "--format"
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

    let rc = match args[1].as_str() {
        "compile" => match positional {
            Some(p) => cmd_compile(p, no_cache, strict),
            None => {
                eprintln!("usage: mirror compile [--strict] <file>");
                1
            }
        },
        "craft" => match positional {
            Some(p) => cmd_craft_with(p, no_cache, target_kind, strict),
            None => {
                eprintln!("usage: mirror craft [--strict] [--target <crystal|binary>] <target>");
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
                eprintln!("usage: mirror kintsugi [--ci [--format mirror|json]] [--shatter N] [--transform <mq>] [--out <path>] <file|dir>");
                1
            }
        },
        other => {
            eprintln!("unknown: {}", other);
            1
        }
    };
    std::process::exit(rc);
}
