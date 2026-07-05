//! Phase A RED — tokenizer change per `docs/math/kintsugi/doc-code-seam.md` §6.1.
//!
//! Load-bearing precondition per Seam audit `795f2b6`
//! (`docs/audits/2026-07-04-seam-doc-as-declaration.md`) §3 correction C3:
//! without the tokenizer change, the doc-as-declaration collapse is
//! DECLARATIVE-only. The eight-shard cascade in
//! `docs/specs/doc-code-seam-shards.md` cannot land as substrate-mechanism
//! until `#`-prefixed lines above `---` produce `Doc` AST nodes.
//!
//! Composes with:
//! - Reed + Alex `docs/specs/property-projection.md` (2026-05-19; the `---`
//!   seam ancestor: "above `---`: declaration; below `---`: observation").
//! - Mara `docs/math/kintsugi/doc-code-seam.md` §6.1 (analytical shape of
//!   the tokenizer change; `above_seam` state predicate + `Doc` node
//!   emission).
//! - Mara `docs/math/the-tower/projection-surface.md` (`63bdecc`; the
//!   recognition-candidate altitude sibling; same content-addressing
//!   discipline).
//!
//! **RED phase**: `bootstrap/src/ast.rs` does not declare a `Doc`
//! `AstKind` variant; `bootstrap/src/tokenize.rs` at lines 285-311 strips
//! `#`-prefixed lines silently to EOL. Text-check tests fail on absence.
//!
//! **GREEN phase** (Tick 2, agent): add `AstKind::Doc` variant;
//! add `above_seam` state tracking (true until first `---` at column 0;
//! false after); replace the `#` branch to emit `AstNode::doc_line`
//! above seam and keep the existing strip-to-EOL behavior below seam.
//!
//! **CASING COLLAPSE (Reed 2026-07-05 per Alex direction)**: renamed from
//! `Docblock`/`docblock` to `Doc`/`doc` — kills the `doc` + `block`
//! compound and the mixed casing seam. Substrate says `doc` lowercase
//! everywhere; Rust variant follows with `Doc` PascalCase per Rust
//! convention. Composes with @epistemologic/spec `doc<T>` = `@loop<@nl<T>>`
//! composite (Alex 2026-07-05 TICK 1 shape).
//!
//! Behavioral fixture verification is Tick 2 discipline; Tick 1 seeds
//! the fixture and asserts its shape.

use std::path::PathBuf;

use mirror::ast::{AstKind, AstNode};
use mirror::grammar::Grammar;
use mirror::tokenize::tokenize;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn ast_declares_doc_kind_variant() {
    let content = read_source("bootstrap/src/ast.rs");
    assert!(
        content.contains("Doc,"),
        "ast.rs must declare a `Doc` AstKind variant per \
         doc-code-seam.md §2 (the doc-as-declaration collapse requires \
         `#`-prefixed lines above `---` to be first-class AST nodes, not \
         silently stripped). Add `Doc` to `enum AstKind`."
    );
}

#[test]
fn tokenize_tracks_above_seam_state() {
    let content = read_source("bootstrap/src/tokenize.rs");
    assert!(
        content.contains("above_seam"),
        "tokenize.rs must track `above_seam` state (true until first `---` \
         at column 0; false after) per doc-code-seam.md §6.1. Load-bearing \
         precondition per Seam audit `795f2b6` §3 correction C3."
    );
}

#[test]
fn tokenize_emits_doc_above_seam() {
    let content = read_source("bootstrap/src/tokenize.rs");
    let has_emission = content.contains("AstKind::Doc") || content.contains("doc_line");
    assert!(
        has_emission,
        "tokenize.rs must emit `Doc` AST nodes for `#`-prefixed lines \
         above `---` seam (not strip silently as it does at lines 285-311). \
         Per Mara doc-code-seam.md §6.1: \
         `parent.add_child(AstNode::doc_line(&bytes[start..end], span))`."
    );
}

#[test]
fn fixture_has_doc_marker_seam_and_body() {
    let content = read_source("bootstrap/tests/fixtures/doc_above_seam/simple.mirror");
    assert!(
        content.contains("---"),
        "fixture must contain `---` seam per Reed + Alex 2026-05-19 \
         property-projection.md (above: declaration; below: observation)."
    );
    assert!(
        content.contains("# TOKENIZE_DOC_MARKER"),
        "fixture must contain marker `# TOKENIZE_DOC_MARKER` in \
         doc above `---` seam. Used by Tick 2 behavioral verification: \
         after GREEN, the marker string must appear in the AST content \
         (currently absent because `#`-lines are stripped by tokenize.rs)."
    );
}

/// Behavioral verification (Mara bonus per Reed brief 2026-07-04): invokes
/// `tokenize` on the fixture and walks the resulting AST asserting that the
/// marker string `TOKENIZE_DOC_MARKER` appears in some `Doc`
/// node's `body`. Complements the text-check tests above: text-check proves
/// the source pattern is present; this test proves the tokenizer's
/// behavior actually produces a `Doc` node whose contents survive
/// through to the AST.
///
/// Per `docs/math/kintsugi/doc-code-seam.md` §6.1: `#`-lines above `---`
/// must produce `AstKind::Doc` nodes carrying the verbatim line bytes.
#[test]
fn tokenize_produces_doc_carrying_marker() {
    let source = read_source("bootstrap/tests/fixtures/doc_above_seam/simple.mirror");
    let grammar = Grammar::new();
    let root = tokenize(source.as_bytes(), &grammar);

    fn find_doc_with(node: &AstNode, needle: &str) -> bool {
        if node.kind == AstKind::Doc {
            if let Some(body) = node.body.as_deref() {
                if body.contains(needle) {
                    return true;
                }
            }
        }
        node.children.iter().any(|c| find_doc_with(c, needle))
    }

    assert!(
        find_doc_with(&root, "TOKENIZE_DOC_MARKER"),
        "tokenize must produce an `AstKind::Doc` node whose body \
         contains `TOKENIZE_DOC_MARKER` when a `#`-prefixed line \
         above the `---` seam carries the marker. Per \
         `docs/math/kintsugi/doc-code-seam.md` §6.1: above-seam `#`-lines \
         must produce `AstKind::Doc` nodes carrying the verbatim line bytes."
    );
}
