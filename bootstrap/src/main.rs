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
mod exec;
mod git;
mod grammar;
mod hash;
mod pipeline;
mod spectral;
mod tokenize;

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::Command;

use prism_core::{Optic, Ref};
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
    eprintln!("commands: compile [--strict] <file>, craft [--strict] [--target <crystal|binary>] <target>, kintsugi [--shatter N] <file>");
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
fn cmd_kintsugi(file: &str, shatter: u64, transform: Option<&str>, out_dir: Option<&str>) -> i32 {
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
        }
        i += 1;
    }
    // Find the positional argument, skipping `--flag value` pairs.
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
            Some(p) => cmd_kintsugi(p, shatter, transform.as_deref(), out_dir.as_deref()),
            None => {
                eprintln!("usage: mirror kintsugi [--shatter N] [--transform <mq>] [--out <path>] <file|dir>");
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
