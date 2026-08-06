//! `apply_h::act` at rust/ altitude — the bilateral-dispatch primitive.
//!
//! Reed 2026-08-06 R-PRIM-3 per Taut scout `7af55ee` §7 smallest-
//! primitive-gap identification + Alex 2026-08-06 Q-1 adjudication
//! (expose as `apply_h::act` for naming honesty per bootstrap surface;
//! other 6 combinators land as extensions).
//!
//! ## Naming
//!
//! Mirrors bootstrap's `apply_h.rs` module structure. Bootstrap has the
//! full 7-combinator surface (~405 LOC per task #140); this rust/
//! sibling starts with the MINIMUM subset needed for MCP composition
//! at Phase 1 — the bilateral-sentinel-check `act` primitive. Additional
//! combinators (compose / fold / bind / etc.) land as extensions when
//! composition-shard bodies demand them, per Alex 2026-08-05 substrate-
//! honest reframe (rust/ delivers primitives; substrate delivers
//! composition).
//!
//! ## Composition
//!
//! `@mcp/serve.mirror` composition-shard body (Mara Fire B M-COMP-1)
//! composes `act` via pipeline:
//!
//! ```text
//! phone::read_stdin_frame  |> wire::parse
//!                          |> apply_h::act(root, action_ref, args)
//!                          |> wire::emit
//!                          |> phone::write_stdout_frame
//! ```
//!
//! Each pipe element is a landed rust/ primitive at terminal-geometry
//! altitude; the whole composition sits at substrate altitude in
//! `shards/mcp/serve.mirror`.
//!
//! ## Phase 1 vs Phase 2
//!
//! **Phase 1 (this landing)**: bilateral-sentinel-check act. Given an
//! action_ref like `@subject/visibility/public.consent_scope_universal`,
//! load the bilateral corpus from `<root>/shards`, look up the
//! substrate-decl'd bilateral by action_ref, verify sentinel byte-string
//! containment in the args. Returns [`Verdict::Pass`] / [`Verdict::Fail`].
//!
//! **Phase 2 (M5+ post-Mara-canonical-spec)**: grammar-driven dispatch
//! per `@mcp/tool` annotation walking via `@mirror/spectral.gestalt`.
//! Additional combinators (compose / fold / bind / focus / project /
//! split / shift / settle) land as demand surfaces.
//!
//! ## Composition anchors
//!
//! - `docs/scouts/2026-08-05-taut-primitives-vs-composition-scout.md`
//!   §7 smallest-primitive-gap (Fire A tick 3)
//! - `feedback-rust-delivers-primitives-substrate-delivers-composition`
//!   memory (Alex 2026-08-05 verbatim correction)
//! - `bootstrap/src/apply_h.rs` (source of the 7-combinator surface;
//!   task #140 GREEN ~405 LOC; RETIRING under Fire C when composition-
//!   path fires via rust/ altitude)
//! - Recognition `#R-reality-as-5d-spinning-foam` RATIFIED 2026-08-03
//!   (Layer 0 sub-Turing decidable floor = rust/ interpreter; apply_h.rs
//!   IS a Layer 0 primitive)
//!
//! ## Register
//!
//! Substrate-honest, decidable, sub-Turing. Given a bilateral corpus
//! (finite, byte-loaded from `<root>/shards/**/*.mirror`) + a bounded
//! args vector, this function terminates in O(corpus_size * args_len)
//! at worst — no unbounded recursion; no networking; no fs writes.
//! Pure predicate evaluation over the substrate-decl'd sentinel.

use std::path::Path;

use roomba::mend::{load_bilateral_corpus, BilateralDecl};

/// The verdict a bilateral-dispatch primitive returns. Mirrors
/// bootstrap's Verdict shape at bilateral-predicate altitude.
///
/// Phase 1 subset: [`Pass`](Verdict::Pass) + [`Fail`](Verdict::Fail).
/// Phase 2+ extension: [`Partial`](Verdict::Partial) with per-clause
/// transparency (opacity map) lands when substrate-decl'd composed
/// bilaterals (multi-clause) enter the composition surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Sentinel matched; substrate-decl'd predicate discharges.
    Pass,
    /// Sentinel not matched, or action_ref not in corpus.
    /// Contains substrate-honest reason string.
    Fail(String),
}

/// The bilateral-dispatch primitive. Load bilateral corpus rooted at
/// `root`, look up the substrate-decl'd bilateral by `action_ref`,
/// verify sentinel byte-string containment against `args`.
///
/// Reed 2026-08-06 R-PRIM-3 per Alex 2026-08-06 Q-1 adjudication
/// (expose as `apply_h::act` naming honesty).
///
/// # Arguments
///
/// * `root` — The substrate repo root (contains `shards/` subdirectory).
/// * `action_ref` — Full action reference like
///   `@subject/visibility/public.consent_scope_universal`.
///   Concatenation of shard-ref + `.` + bilateral-name per
///   `BilateralDecl::full_action_ref` semantics.
/// * `args` — Positional args. Sentinel-containment check runs against
///   the concatenation of `args` joined by ASCII space (matching
///   bootstrap's argv byte-substring semantics).
///
/// # Returns
///
/// [`Verdict::Pass`] if sentinel matches; [`Verdict::Fail`] with
/// substrate-honest reason otherwise.
///
/// # Substrate-honest semantics
///
/// This is the MINIMUM `act` primitive at rust/ altitude. It handles
/// the base-case bilateral-sentinel-check dispatch. Composition-shards
/// (Mara Fire B) compose over it for MCP tool dispatch that maps to
/// bilateral predicates. Non-bilateral dispatch (e.g. "execute a rust/
/// cmd_ verb") is composition-shard's responsibility, not this
/// primitive's.
pub fn act(root: &Path, action_ref: &str, args: &[String]) -> Verdict {
    let corpus = load_bilateral_corpus(root);
    let decl: &BilateralDecl = match corpus.get(action_ref) {
        Some(d) => d,
        None => {
            return Verdict::Fail(format!(
                "apply_h::act: action_ref `{}` not found in bilateral corpus at `{}`",
                action_ref,
                root.display()
            ))
        }
    };

    // Sentinel-containment check per bootstrap's byte-substring
    // semantics. Concat args by ASCII space and substring-match the
    // sentinel. Trivially decidable in O(args_concat_len * sentinel_len).
    let args_joined = args.join(" ");
    if args_joined.contains(&decl.sentinel) {
        Verdict::Pass
    } else {
        Verdict::Fail(format!(
            "apply_h::act: sentinel `{}` not found in args `{}` for `{}`",
            decl.sentinel, args_joined, action_ref
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Fixture: minimal substrate root with one bilateral-decl'd shard.
    /// Returns TempDir handle (dropped on test end — cleans up).
    fn fixture_root() -> TempDir {
        let dir = TempDir::new().expect("tempdir");
        let shards = dir.path().join("shards").join("test");
        fs::create_dir_all(&shards).expect("mkdir");
        let shard = shards.join("visibility.mirror");
        // Substrate-decl per shards/**/*.mirror grammar. `bilateral
        // <name> { sentinel "<bytes>" arity <n> }` shape extracted by
        // `roomba::mend::extract_bilaterals` line-scan (documented at
        // rust/roomba/src/mend.rs `fn extract_bilaterals`).
        let source = r#"# @test/visibility — fixture for apply_h::act tests.

bilateral consent_scope_universal {
  sentinel "scope=universal"
  arity 1
}
"#;
        fs::write(&shard, source).expect("write shard");
        dir
    }

    #[test]
    fn act_pass_on_sentinel_match() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &["scope=universal".to_string()],
        );
        assert_eq!(verdict, Verdict::Pass);
    }

    #[test]
    fn act_fail_on_sentinel_miss() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &["scope=private".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("sentinel"));
                assert!(reason.contains("scope=universal"));
            }
            _ => panic!("expected Fail on sentinel miss, got {:?}", verdict),
        }
    }

    #[test]
    fn act_fail_on_unknown_action_ref() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@nonexistent/shard.nonexistent_predicate",
            &["anything".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("not found in bilateral corpus"));
            }
            _ => panic!("expected Fail on unknown action_ref, got {:?}", verdict),
        }
    }

    #[test]
    fn act_multi_arg_sentinel_matches_concatenation() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &[
                "peer=alice".to_string(),
                "scope=universal".to_string(),
                "trust=1".to_string(),
            ],
        );
        assert_eq!(verdict, Verdict::Pass);
    }
}
