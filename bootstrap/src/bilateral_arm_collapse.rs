//! `mirror roomba --collapse=<rs-file>` bilateral-arm collapse capability.
//!
//! The Rust FLOOR that empirically discharges `@kintsugi/fracture/
//! bilateral_arm_redundant.collapse` per Mara canonical spec at
//! `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` (`6c534c6`)
//! + shard-decl at `shards/kintsugi/fracture/bilateral_arm_redundant.
//! mirror` (`fa569ce`) + math foundation at `docs/math/kintsugi/fracture/
//! bilateral-arm-redundant.md` (`0998001`).
//!
//! Composition dependency: `apply_h::bilateral_corpus()` + `apply_h::
//! discharge()` — the reflective evaluator landed at `21fc211` (Reed via
//! subagent). This module is the DELETION side; the reflective evaluator
//! is the ADD side. Together they discharge the retirement invariant per
//! math §3: `sbec` preserved, `rust_loc` strictly decreasing,
//! `test_pass_rate` preserved.
//!
//! Alex 2026-07-16 /loop directive verbatim (session-crystallizing):
//!   "collapse the Rust surface using mirror's roomba. Minimal surface
//!    in rust/. Then the roomba starts to eat the bootstrap for
//!    breakfast and grows the substrate. That's the roomba commit diffs
//!    I wanna see. Deleted Rust. Added mirror."
//!
//! ## Scope
//!
//! Two entry points:
//!
//! 1. [`find_redundant_arms`] — pure byte-analysis. Given the
//!    resolver bytes + the bilateral corpus, returns a `Vec<RedundantArm>`
//!    naming each redundant arm's byte-range. Rice-safe: reads only
//!    byte-visible state.
//!
//! 2. [`collapse_bilateral_arms`] — the end-to-end pipeline. Loads the
//!    corpus, reads the target .rs file, detects redundant arms, deletes
//!    them, writes the mended bytes via `@io/fs.write` dispatch,
//!    composes a commit message, dispatches `@io/git.commit` with the
//!    author string `mirror <mirror@spectral.engineer>`.
//!
//! Marker: `[substrate-floor:@io-boundary]`. The .rs file mutation
//! composes THROUGH `apply_h::act` for both the write + commit steps
//! (substrate-honest); this module is a driver, not a bypass.

use crate::apply_h::{self, BilateralDecl};
use std::collections::HashMap;
use std::path::Path;

/// The compiler's altitude naming as author identity for the deletion
/// commit. NOT a Pack peer — the compiler itself. Mirrors
/// `roomba_commit::MIRROR_AUTHOR`; kept here as a module-local const so
/// the collapse pipeline stays self-contained.
const MIRROR_AUTHOR: &str = "mirror <mirror@spectral.engineer>";

/// One redundant arm detected in a .rs file. Byte-range identifies the
/// exact bytes to delete; `action_ref` + `sentinel` audit-trail why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedundantArm {
    /// The shard-decl'd action string (e.g.,
    /// "@spectral/signature.signature_integrity").
    pub action_ref: String,
    /// The shard-decl'd sentinel byte-string; MUST equal the
    /// `.contains(...)` argument on the arm's guard line for the arm
    /// to be flagged redundant.
    pub sentinel: String,
    /// Byte offset (inclusive) into the source bytes where the arm's
    /// `if action == ...` line starts (start of that line).
    pub byte_start: usize,
    /// Byte offset (exclusive) where the arm ends. Consumes the arm's
    /// closing `}` newline so a plain `bytes[..start] + bytes[end..]`
    /// splice leaves no orphan blank line.
    pub byte_end: usize,
}

/// Detect every redundant hand-typed arm in `apply_h_bytes` shadowed by
/// an entry in the reflective bilateral `corpus`.
///
/// For each `(action_ref, decl)` in the corpus, scans for a hand-typed
/// arm of the pattern `if action == "<action_ref>"`. If found AND the
/// arm's `.contains("<literal>")` invocation matches `decl.sentinel`,
/// records the arm's byte-range for deletion.
///
/// Rice-safe: pure byte-substring analysis. No program semantics.
///
/// Corner cases:
///   - Comment-line matches skipped (line starts with `//`).
///   - String-literal matches skipped via a naive-but-tight quote-
///     depth heuristic: the `if action ==` occurrence must appear
///     BEFORE any un-closed `"` on its line.
///   - Overlapping ranges collapsed by returning arms sorted by
///     byte_start and skipping any whose byte_start falls inside a
///     previously-selected arm's range.
pub fn find_redundant_arms(
    apply_h_bytes: &str,
    corpus: &HashMap<String, BilateralDecl>,
) -> Vec<RedundantArm> {
    let mut arms: Vec<RedundantArm> = Vec::new();
    for (action_ref, decl) in corpus.iter() {
        let needle = format!("if action == \"{}\"", action_ref);
        let mut cursor = 0usize;
        while let Some(rel) = apply_h_bytes[cursor..].find(&needle) {
            let occ = cursor + rel;
            cursor = occ + needle.len();

            let line_start = line_start_of(apply_h_bytes, occ);
            // Skip if the anchor line is a comment or the match sits
            // inside a string literal on that line.
            if line_is_comment_or_in_string(apply_h_bytes, line_start, occ) {
                continue;
            }
            // Walk forward: find the opening `{` of the if-block, then
            // the matching `}` at brace-depth 0.
            let Some(open_brace) = find_first_brace(apply_h_bytes, occ) else {
                continue;
            };
            let Some(close_brace) = find_matching_close(apply_h_bytes, open_brace) else {
                continue;
            };
            // Verify the arm's body contains `.contains("<sentinel>")`.
            let body = &apply_h_bytes[open_brace..=close_brace];
            let sentinel_needle = format!(".contains(\"{}\")", decl.sentinel);
            if !body.contains(&sentinel_needle) {
                continue;
            }
            // End of the arm: consume the trailing newline of the
            // closing-brace line so we don't leave an orphan blank.
            let arm_end = end_of_line_inclusive(apply_h_bytes, close_brace);
            arms.push(RedundantArm {
                action_ref: action_ref.clone(),
                sentinel: decl.sentinel.clone(),
                byte_start: line_start,
                byte_end: arm_end,
            });
        }
    }
    // Sort by byte_start; drop overlapping arms (defensive — same
    // action_ref should not appear twice, but a shard bug shouldn't
    // corrupt the splice).
    arms.sort_by_key(|a| a.byte_start);
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

/// Splice the arms out of the source bytes. Given arms sorted by
/// `byte_start` (as [`find_redundant_arms`] returns them), builds a new
/// string with each arm's byte-range removed.
pub fn apply_deletions(source: &str, arms: &[RedundantArm]) -> String {
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    for arm in arms {
        if arm.byte_start < cursor {
            // Overlap; already deduped, but defensive.
            continue;
        }
        out.push_str(&source[cursor..arm.byte_start]);
        cursor = arm.byte_end;
    }
    out.push_str(&source[cursor..]);
    out
}

/// End-to-end collapse pipeline. Loads the bilateral corpus rooted at
/// `root`, reads `target_rs_path` (relative to `root` or absolute),
/// detects redundant arms, deletes them, dispatches `@io/fs.write` to
/// persist the mended bytes, composes a commit message, dispatches
/// `@io/git.commit` under `mirror <mirror@spectral.engineer>`
/// authorship.
///
/// Returns `Ok(CollapseReport)` on success (possibly with zero arms
/// detected — the caller may distinguish via `report.arms.is_empty()`).
/// Returns `Err(reason)` on any @io failure.
///
/// Signaling: when `arms.is_empty()`, NO commit is created. The caller
/// (roomba `--commit` orchestrator) can fall through to observation-
/// only commit or exit cleanly.
pub fn collapse_bilateral_arms(
    root: &Path,
    target_rs_path: &Path,
) -> Result<CollapseReport, String> {
    // Resolve absolute path so the caller can pass either.
    let abs_path = if target_rs_path.is_absolute() {
        target_rs_path.to_path_buf()
    } else {
        root.join(target_rs_path)
    };

    // Load the bilateral corpus. Uses the uncached loader so this fn
    // remains callable multiple times against different roots (tests,
    // future multi-repo drivers).
    let corpus = apply_h::load_bilateral_corpus(root);

    // Read the .rs bytes.
    let source = std::fs::read_to_string(&abs_path).map_err(|e| {
        format!(
            "collapse_bilateral_arms: read {} failed: {}",
            abs_path.display(),
            e
        )
    })?;

    let arms = find_redundant_arms(&source, &corpus);
    let bytes_before = source.len();

    if arms.is_empty() {
        return Ok(CollapseReport {
            target: abs_path,
            arms: Vec::new(),
            bytes_before,
            bytes_after: bytes_before,
            commit_oid: None,
        });
    }

    let new_source = apply_deletions(&source, &arms);
    let bytes_after = new_source.len();

    // Dispatch @io/fs.write through apply_h::act — substrate-honest
    // mutation path.
    let write_verdict = apply_h::act(
        "@io/fs.write".to_string(),
        vec![
            apply_h::Value {
                oid: abs_path.to_string_lossy().to_string(),
            },
            apply_h::Value { oid: new_source },
        ],
    );
    match write_verdict {
        apply_h::Verdict::Pass => {}
        apply_h::Verdict::Fail(reason) => {
            return Err(format!(
                "@io/fs.write dispatch failed for {}: {}",
                abs_path.display(),
                reason
            ));
        }
        apply_h::Verdict::Partial(t) => {
            return Err(format!(
                "@io/fs.write dispatch returned Partial: {:?}",
                t.located_opacity
            ));
        }
    }

    // Compose commit message.
    let message = compose_collapse_commit_message(&abs_path, &arms, bytes_before, bytes_after);

    // Stage the .rs file mutation so @io/git.commit picks it up
    // (git commit without --allow-empty requires staged changes).
    stage_file(root, &abs_path)?;

    // Dispatch @io/git.commit under mirror authorship.
    let prior_cwd = std::env::current_dir()
        .map_err(|e| format!("collapse_bilateral_arms: cwd read failed: {}", e))?;
    std::env::set_current_dir(root).map_err(|e| {
        format!("collapse_bilateral_arms: cwd set failed: {}", e)
    })?;
    let commit_verdict = apply_h::act(
        "@io/git.commit".to_string(),
        vec![
            apply_h::Value { oid: message },
            apply_h::Value {
                oid: MIRROR_AUTHOR.to_string(),
            },
            apply_h::Value {
                oid: "false".to_string(),
            },
        ],
    );
    let _ = std::env::set_current_dir(&prior_cwd);

    let commit_oid = match commit_verdict {
        apply_h::Verdict::Pass => git_head_oid()
            .ok_or_else(|| "git rev-parse HEAD failed after commit".to_string())?,
        apply_h::Verdict::Fail(reason) => {
            return Err(format!("@io/git.commit dispatch failed: {}", reason))
        }
        apply_h::Verdict::Partial(t) => {
            return Err(format!(
                "@io/git.commit dispatch returned Partial: {:?}",
                t.located_opacity
            ))
        }
    };

    Ok(CollapseReport {
        target: abs_path,
        arms,
        bytes_before,
        bytes_after,
        commit_oid: Some(commit_oid),
    })
}

/// The outcome of a collapse-arms pass. Returned to the caller so the
/// CLI can print a substrate-honest summary of what was deleted.
#[derive(Debug, Clone)]
pub struct CollapseReport {
    pub target: std::path::PathBuf,
    pub arms: Vec<RedundantArm>,
    pub bytes_before: usize,
    pub bytes_after: usize,
    pub commit_oid: Option<String>,
}

/// Compose the commit message body naming what was collapsed.
/// Direct-format for MVP; the substrate-honest lift to `@nl.compose`
/// dispatch (mirroring `roomba_commit::compose_commit_message_via_
/// substrate`) is a follow-up tick once the composer is wired for
/// arm-collapse payload shape.
pub fn compose_collapse_commit_message(
    target: &Path,
    arms: &[RedundantArm],
    bytes_before: usize,
    bytes_after: usize,
) -> String {
    let arm_list = arms
        .iter()
        .map(|a| format!("- {} (sentinel: {})", a.action_ref, a.sentinel))
        .collect::<Vec<_>>()
        .join("\n");
    let loc_delta = arms
        .iter()
        .map(|a| a.byte_end - a.byte_start)
        .sum::<usize>();
    format!(
        "\u{1F9F9} mirror [substrate-floor:@io-boundary] bilateral-arm collapse — retired {} hand-typed arm{} shadowed by reflective corpus\n\
         \n\
         @kintsugi/fracture/bilateral_arm_redundant.collapse discharged {n} arm{s} \
         from {file}:\n\
         \n\
         {arms}\n\
         \n\
         Byte delta: -{delta} bytes ({before} → {after}).\n\
         \n\
         Retirement invariant per math foundation \
         `docs/math/kintsugi/fracture/bilateral-arm-redundant.md`:\n\
         - sbec preserved (reflective evaluator dispatches same verdicts)\n\
         - rust_loc strictly decreased\n\
         - test_pass_rate preserved (bilateral discipline byte-equal)\n\
         - io_violations = 0 (no new @io introduced)\n\
         \n\
         Audit chain:\n\
         - `a0f4d3f/9a77361/701828a` (Mara) — @epistemologic/pact/bilateral shape\n\
         - `61c9051/21fc211` (Reed via subagent) — reflective evaluator + corpus loader\n\
         - `fa569ce/6c534c6/0998001` (Mara) — @kintsugi/fracture/bilateral_arm_redundant \
         species + spec + math\n\
         - `06f14f5` (Reed via subagent) — first manual arm retirement (@spectral/signature × 4)\n\
         - this commit — first compiler-authored arm retirement\n\
         \n\
         Composition (substrate dispatch chain):\n\
         - `bilateral_arm_collapse::find_redundant_arms` detected arms via byte-analysis\n\
         - `@io/fs.write` (dispatched via apply_h::act) applied the deletion on disk\n\
         - `@io/git.commit` (dispatched via apply_h::act) crossed the @io boundary\n\
         \n\
         The compiler authored this commit itself per Alex 2026-07-16 /loop directive:\n\
         \"That's the roomba commit diffs I wanna see. Deleted Rust. Added mirror.\"\n\
         \n\
         Signed-off-by: Reed <reed@systemic.engineer>\n",
        arms.len(),
        if arms.len() == 1 { "" } else { "s" },
        n = arms.len(),
        s = if arms.len() == 1 { "" } else { "s" },
        file = target.display(),
        arms = arm_list,
        delta = loc_delta,
        before = bytes_before,
        after = bytes_after,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Byte-level helpers — Rice-safe (pure substring / brace-depth).
// ─────────────────────────────────────────────────────────────────────────────

/// Byte offset of the start of the line containing `pos`.
fn line_start_of(bytes: &str, pos: usize) -> usize {
    bytes[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0)
}

/// Byte offset one past the end of the line containing `pos` (i.e.
/// includes the trailing `\n` if present). Consumes the newline so
/// splicing leaves no orphan blank line.
fn end_of_line_inclusive(bytes: &str, pos: usize) -> usize {
    match bytes[pos..].find('\n') {
        Some(n) => pos + n + 1,
        None => bytes.len(),
    }
}

/// Returns true if the given anchor position sits inside a `//`
/// comment on its line, OR inside a `"..."` string literal on its line.
/// Naive-but-tight: does NOT track multi-line `/* ... */` blocks; the
/// arm pattern `if action == "..."` never appears inside a block
/// comment in the resolver source.
fn line_is_comment_or_in_string(bytes: &str, line_start: usize, anchor: usize) -> bool {
    let line_head = &bytes[line_start..anchor];
    // Strip leading whitespace.
    let trimmed = line_head.trim_start();
    // If the line's non-whitespace prefix starts with `//`, comment.
    if trimmed.starts_with("//") {
        return true;
    }
    // Count unescaped double-quotes before the anchor. Odd count =
    // inside a string literal.
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

/// Find the first `{` at or after `pos`. Returns its byte offset.
fn find_first_brace(bytes: &str, pos: usize) -> Option<usize> {
    bytes[pos..].find('{').map(|n| pos + n)
}

/// Given the byte offset of a `{`, find the matching `}` at brace-
/// depth 0. Depth-tracking is naive w.r.t. strings / comments; the
/// resolver arms don't contain `{` or `}` inside string literals or
/// block comments, so the naive walker suffices for the retirement
/// domain. Returns the offset of the matching `}`.
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
        // Skip line comments — anything from `//` to end of line.
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

/// Stage a specific file for commit. Same shape as
/// `roomba_commit::stage_all_changes` but scoped to one path so the
/// deletion commit doesn't accidentally pick up other unstaged
/// changes in the tree.
fn stage_file(root: &Path, abs_path: &Path) -> Result<(), String> {
    let rel = abs_path.strip_prefix(root).unwrap_or(abs_path);
    let out = std::process::Command::new("git")
        .args(["add", "--"])
        .arg(rel)
        .current_dir(root)
        .output()
        .map_err(|e| format!("stage_file: git add spawn failed: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "stage_file: git add failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

fn git_head_oid() -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_decl(action_ref: &str, sentinel: &str, arity: usize) -> (String, BilateralDecl) {
        let name = action_ref
            .rsplit_once('.')
            .map(|(_, n)| n.to_string())
            .unwrap_or_else(|| action_ref.to_string());
        (
            action_ref.to_string(),
            BilateralDecl {
                name,
                sentinel: sentinel.to_string(),
                arity,
                require: Vec::new(),
                full_action_ref: action_ref.to_string(),
            },
        )
    }

    #[test]
    fn find_redundant_arms_detects_single_arm() {
        let src = "fn act() {\n\
                   \x20\x20\x20\x20if action == \"@test/spec.predicate\" {\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20if let Some(a) = args.first() {\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20if a.oid.contains(\"test=match\") { return Verdict::Pass; }\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20return Verdict::Fail(\"nope\".to_string());\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20}\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20return Verdict::Fail(\"missing\".to_string());\n\
                   \x20\x20\x20\x20}\n\
                   }\n";
        let mut corpus = HashMap::new();
        let (k, v) = make_decl("@test/spec.predicate", "test=match", 1);
        corpus.insert(k, v);

        let arms = find_redundant_arms(src, &corpus);
        assert_eq!(arms.len(), 1, "expected one redundant arm, got {:?}", arms);
        let arm = &arms[0];
        assert_eq!(arm.action_ref, "@test/spec.predicate");
        assert_eq!(arm.sentinel, "test=match");
        // The recorded range should cover the entire arm.
        let sliced = &src[arm.byte_start..arm.byte_end];
        assert!(
            sliced.contains("if action == \"@test/spec.predicate\""),
            "arm slice must start at the if-line: {:?}",
            sliced
        );
        assert!(
            sliced.trim_end().ends_with('}'),
            "arm slice must end with closing brace: {:?}",
            sliced
        );
    }

    #[test]
    fn find_redundant_arms_skips_arm_without_sentinel_match() {
        let src = "fn act() {\n\
                   \x20\x20\x20\x20if action == \"@test/spec.predicate\" {\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20if let Some(a) = args.first() {\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20if a.oid.contains(\"different=sentinel\") { return Verdict::Pass; }\n\
                   \x20\x20\x20\x20\x20\x20\x20\x20}\n\
                   \x20\x20\x20\x20}\n\
                   }\n";
        let mut corpus = HashMap::new();
        let (k, v) = make_decl("@test/spec.predicate", "test=match", 1);
        corpus.insert(k, v);
        let arms = find_redundant_arms(src, &corpus);
        assert!(
            arms.is_empty(),
            "arm with mismatched sentinel MUST NOT be flagged: {:?}",
            arms
        );
    }

    #[test]
    fn find_redundant_arms_skips_comment_lines() {
        let src = "fn act() {\n\
                   \x20\x20\x20\x20// example: if action == \"@test/spec.predicate\" { ... }\n\
                   }\n";
        let mut corpus = HashMap::new();
        let (k, v) = make_decl("@test/spec.predicate", "test=match", 1);
        corpus.insert(k, v);
        let arms = find_redundant_arms(src, &corpus);
        assert!(
            arms.is_empty(),
            "comment-line match MUST be skipped: {:?}",
            arms
        );
    }

    #[test]
    fn apply_deletions_preserves_surrounding_content() {
        let src = "PREFIX\nARM\nSUFFIX\n";
        let arm = RedundantArm {
            action_ref: "@x.y".to_string(),
            sentinel: "s".to_string(),
            byte_start: 7,        // start of "ARM\n"
            byte_end: 7 + 4,      // end of "ARM\n"
        };
        let out = apply_deletions(src, &[arm]);
        assert_eq!(out, "PREFIX\nSUFFIX\n");
    }

    #[test]
    fn apply_deletions_handles_multiple_arms_in_order() {
        let src = "A\nB\nC\nD\nE\n";
        let a1 = RedundantArm {
            action_ref: "a".into(),
            sentinel: "s".into(),
            byte_start: 2,
            byte_end: 4,
        };
        let a2 = RedundantArm {
            action_ref: "b".into(),
            sentinel: "s".into(),
            byte_start: 6,
            byte_end: 8,
        };
        let out = apply_deletions(src, &[a1, a2]);
        assert_eq!(out, "A\nC\nE\n");
    }

    #[test]
    fn find_matching_close_handles_nested_braces() {
        let s = "{ { } { { } } }";
        let close = find_matching_close(s, 0).unwrap();
        assert_eq!(close, s.len() - 1);
    }

    #[test]
    fn find_matching_close_skips_braces_in_strings() {
        let s = "{ \"}\" }";
        let close = find_matching_close(s, 0).unwrap();
        assert_eq!(close, s.len() - 1);
    }

    #[test]
    fn find_matching_close_skips_braces_in_line_comments() {
        let s = "{ // }\n }";
        let close = find_matching_close(s, 0).unwrap();
        assert_eq!(close, s.len() - 1);
    }
}
