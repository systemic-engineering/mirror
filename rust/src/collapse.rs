//! `collapse.rs` — bilateral-arm collapse capability at rust/ altitude.
//!
//! First substrate-delta surface birthed FROM the terminal floor.
//! Composed shard-body + @io: reads shards/**/*.mirror to extract
//! bilateral declarations; scans .rs corpus files for hand-typed arms
//! whose sentinel matches a landed shard-decl; splices those arms out;
//! writes mended bytes; commits under `mirror <mirror@spectral.engineer>`.
//!
//! Per Mara `81294b3` §7.4 dispatch matrix (walker's fracture table) +
//! Seam `c1775f1` 12/12 SHIP ratification (stigmergy witnessed
//! computation math root by Mara `d7ff58e` + canonical spec `95c0e4a`).
//!
//! This is a FRESH REIMPLEMENTATION at rust/ altitude — NOT a lift of
//! bootstrap/src/bilateral_arm_collapse.rs. The bootstrap module is
//! algorithm reference only per AGENTS.md "cite the existing reference"
//! discipline. Shape parallels; import/copy prohibited.
//!
//! ## Rice-safety
//!
//! Pure byte-substring analysis. Byte-visible state only. No program
//! semantics. Comment lines skipped; string-literal contexts skipped
//! via naive quote-depth on the anchor line.
//!
//! ## Composition anchors (LANDED)
//!
//! - `shards/epistemologic/pact/bilateral.mirror` — bilateral typed carrier.
//! - `shards/kintsugi/fracture/bilateral_arm_redundant.mirror` — species.
//! - `shards/spectral/signature.mirror` — 4 example landed bilaterals.
//! - `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` — Mara spec.
//! - `docs/math/kintsugi/fracture/bilateral-arm-redundant.md` — math root.
//! - `bootstrap/src/bilateral_arm_collapse.rs` — algorithm reference (READ ONLY).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// One bilateral declaration extracted from a `.mirror` shard file.
///
/// The corpus is keyed on `full_action_ref` (e.g.
/// `@spectral/signature.signature_integrity`); this record carries the
/// sentinel byte-string that the redundant-arm detector matches on.
#[derive(Debug, Clone)]
pub struct BilateralDecl {
    /// Predicate name (e.g. `signature_integrity`).
    pub name: String,
    /// Sentinel byte-string (e.g. `chain=merkle-linked`).
    pub sentinel: String,
    /// Arity — number of args expected. 1 = base; 2+ = composed.
    pub arity: usize,
    /// Full action ref (e.g. `@spectral/signature.signature_integrity`).
    pub full_action_ref: String,
}

/// One redundant hand-typed arm detected in a `.rs` file. Byte-range
/// identifies exact bytes to delete; `action_ref` + `sentinel` name
/// the shard-decl'd source of truth that shadows it.
#[derive(Debug, Clone)]
pub struct RedundantArm {
    pub action_ref: String,
    pub sentinel: String,
    /// Byte offset (inclusive) into source where the arm's
    /// `if action == "..."` line starts.
    pub byte_start: usize,
    /// Byte offset (exclusive) where the arm ends — consumes the
    /// closing `}` newline so splice leaves no orphan blank line.
    pub byte_end: usize,
}

/// The outcome of a collapse pass over one `.rs` file.
#[derive(Debug, Clone)]
pub struct CollapseReport {
    pub target: PathBuf,
    pub arms: Vec<RedundantArm>,
    pub bytes_before: usize,
    pub bytes_after: usize,
    /// Present iff a commit was authored (arms non-empty AND
    /// @io/git.commit succeeded).
    pub commit_oid: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────
// Bilateral corpus extraction (shard-body composition).
// ─────────────────────────────────────────────────────────────────────

/// Load bilateral corpus by walking `<root>/shards/**/*.mirror`. Errors
/// during file reads are silently skipped (per bootstrap precedent);
/// callers may report an empty corpus as substrate-honest "no shards".
pub fn load_bilateral_corpus(root: &Path) -> HashMap<String, BilateralDecl> {
    let mut corpus = HashMap::new();
    let shards_root = root.join("shards");
    if !shards_root.is_dir() {
        return corpus;
    }
    let mut files: Vec<PathBuf> = Vec::new();
    walk_mirror_files(&shards_root, &mut files);
    for path in files {
        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let shard_ref = shard_ref_from_source(&source, &path);
        for decl in extract_bilaterals(&source, &shard_ref) {
            corpus.insert(decl.full_action_ref.clone(), decl);
        }
    }
    corpus
}

/// Recursive .mirror file walker. Not phone.rs's WalkEntry-shaped API
/// because we want only `.mirror` files here; caller keeps this local.
fn walk_mirror_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for e in entries.flatten() {
        let path = e.path();
        let ft = match e.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            walk_mirror_files(&path, out);
        } else if path.extension().and_then(|x| x.to_str()) == Some("mirror") {
            out.push(path);
        }
    }
}

/// Derive the enclosing shard's `@`-ref from source. Prefers a
/// top-level `prism @X/Y {}`; falls back to path stripped of
/// `shards/` prefix + `.mirror` suffix.
fn shard_ref_from_source(source: &str, path: &Path) -> String {
    for line in source.lines() {
        let t = line.trim_start();
        if t.starts_with('#') {
            continue;
        }
        if let Some(rest) = t.strip_prefix("prism ") {
            let rest = rest.trim_start();
            if let Some(at) = rest.strip_prefix('@') {
                let end = at
                    .find(|c: char| c == ' ' || c == '\t' || c == '{' || c == '\n')
                    .unwrap_or(at.len());
                if end > 0 {
                    let mut s = String::with_capacity(end + 1);
                    s.push('@');
                    s.push_str(&at[..end]);
                    return s;
                }
            }
        }
    }
    let p_string = path.to_string_lossy().into_owned();
    let mut p: &str = p_string.as_ref();
    if let Some(idx) = p.find("shards/") {
        p = &p[idx + "shards/".len()..];
    }
    let p = p.strip_suffix(".mirror").unwrap_or(p);
    format!("@{}", p)
}

/// Parse `bilateral <name> { sentinel "..." arity <n> ... }` blocks
/// from `.mirror` source. Line-scan; brace-terminated body.
fn extract_bilaterals(source: &str, shard_ref: &str) -> Vec<BilateralDecl> {
    let mut out = Vec::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        let rest = match trimmed.strip_prefix("bilateral ") {
            Some(r) => r,
            None => continue,
        };
        let name_part = match rest.split_once('{') {
            Some((n, _)) => n.trim(),
            None => continue,
        };
        if name_part.is_empty() {
            continue;
        }
        let name = name_part.to_string();
        let mut sentinel = String::new();
        let mut arity: usize = 1;
        for body_line in lines.by_ref() {
            let bt = body_line.trim();
            if bt.starts_with('}') {
                break;
            }
            if let Some(v) = bt.strip_prefix("sentinel ") {
                let v = v.trim();
                sentinel = v
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or(v)
                    .to_string();
            } else if let Some(v) = bt.strip_prefix("arity ") {
                if let Ok(n) = v.trim().parse::<usize>() {
                    arity = n;
                }
            }
        }
        let full_action_ref = format!("{}.{}", shard_ref, name);
        out.push(BilateralDecl {
            name,
            sentinel,
            arity,
            full_action_ref,
        });
    }
    out
}

// ─────────────────────────────────────────────────────────────────────
// Arm detection (Rice-safe byte-substring analysis).
// ─────────────────────────────────────────────────────────────────────

/// Detect every redundant arm in `source` shadowed by the corpus.
///
/// For each `(action_ref, decl)`: scan `if action == "<action_ref>"`;
/// verify the arm's body contains `.contains("<sentinel>")`; record
/// the arm's byte-range.
pub fn find_redundant_arms(
    source: &str,
    corpus: &HashMap<String, BilateralDecl>,
) -> Vec<RedundantArm> {
    let mut arms: Vec<RedundantArm> = Vec::new();
    for (action_ref, decl) in corpus.iter() {
        let needle = format!("if action == \"{}\"", action_ref);
        let mut cursor = 0usize;
        while let Some(rel) = source[cursor..].find(&needle) {
            let occ = cursor + rel;
            cursor = occ + needle.len();

            let line_start = line_start_of(source, occ);
            if line_is_comment_or_in_string(source, line_start, occ) {
                continue;
            }
            let Some(open_brace) = find_first_brace(source, occ) else {
                continue;
            };
            let Some(close_brace) = find_matching_close(source, open_brace) else {
                continue;
            };
            let body = &source[open_brace..=close_brace];
            let sentinel_needle = format!(".contains(\"{}\")", decl.sentinel);
            if !body.contains(&sentinel_needle) {
                continue;
            }
            let arm_end = end_of_line_inclusive(source, close_brace);
            arms.push(RedundantArm {
                action_ref: action_ref.clone(),
                sentinel: decl.sentinel.clone(),
                byte_start: line_start,
                byte_end: arm_end,
            });
        }
    }
    arms.sort_by_key(|a| a.byte_start);
    // Dedupe overlapping ranges (defensive).
    let mut deduped: Vec<RedundantArm> = Vec::with_capacity(arms.len());
    for arm in arms {
        if let Some(prev) = deduped.last() {
            if arm.byte_start < prev.byte_end {
                continue;
            }
        }
        deduped.push(arm);
    }
    deduped
}

/// Splice arms out of the source. Arms must be sorted by `byte_start`.
pub fn apply_deletions(source: &str, arms: &[RedundantArm]) -> String {
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    for arm in arms {
        if arm.byte_start < cursor {
            continue;
        }
        out.push_str(&source[cursor..arm.byte_start]);
        cursor = arm.byte_end;
    }
    out.push_str(&source[cursor..]);
    out
}

// ─────────────────────────────────────────────────────────────────────
// Byte-level helpers (Rice-safe).
// ─────────────────────────────────────────────────────────────────────

fn line_start_of(bytes: &str, pos: usize) -> usize {
    bytes[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0)
}

fn end_of_line_inclusive(bytes: &str, pos: usize) -> usize {
    match bytes[pos..].find('\n') {
        Some(n) => pos + n + 1,
        None => bytes.len(),
    }
}

fn line_is_comment_or_in_string(bytes: &str, line_start: usize, anchor: usize) -> bool {
    let line_head = &bytes[line_start..anchor];
    let trimmed = line_head.trim_start();
    if trimmed.starts_with("//") {
        return true;
    }
    let mut in_string = false;
    let mut escape = false;
    for ch in line_head.chars() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            _ => {}
        }
    }
    in_string
}

fn find_first_brace(bytes: &str, pos: usize) -> Option<usize> {
    bytes[pos..].find('{').map(|n| pos + n)
}

fn find_matching_close(bytes: &str, open_brace_pos: usize) -> Option<usize> {
    let ch_bytes = bytes.as_bytes();
    if open_brace_pos >= ch_bytes.len() || ch_bytes[open_brace_pos] != b'{' {
        return None;
    }
    let mut depth: i32 = 0;
    let mut i = open_brace_pos;
    let mut in_string = false;
    let mut escape = false;
    while i < ch_bytes.len() {
        let ch = ch_bytes[i];
        if escape {
            escape = false;
            i += 1;
            continue;
        }
        if in_string {
            match ch {
                b'\\' => escape = true,
                b'"' => in_string = false,
                _ => {}
            }
            i += 1;
            continue;
        }
        if ch == b'/' && i + 1 < ch_bytes.len() && ch_bytes[i + 1] == b'/' {
            while i < ch_bytes.len() && ch_bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        match ch {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
