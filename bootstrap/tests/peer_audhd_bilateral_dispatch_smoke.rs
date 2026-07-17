//! Bilateral-dispatch smoke — `@peer.audhd_admissible` empirical second-witness
//! [substrate-floor:@io-boundary] per Mara canonical landing d8b149c on
//! `shards/peer.mirror` — extends the shard with:
//!   - `audhd_context` five-field carrier (ambiguity, k_tracks, coupling,
//!     psychohistory, timestamp)
//!   - `audhd(p, ctx) -> imperfect(ref, ref, ref)` action
//!   - `bilateral audhd_admissible { sentinel "audhd=admissible-k-track-context"
//!     arity 2 }` — the byte-level substrate-decl'd sentinel this test
//!     empirically fires.
//!
//! Reflective-first architecture per `bootstrap/src/apply_h.rs::act` line ~588:
//! any bilateral in `bilateral_corpus()` auto-dispatches via `discharge`. The
//! corpus loader walks `shards/**/*.mirror` (per `find_substrate_root` +
//! `walk_mirror_files` + `extract_bilaterals`) — Mara's block at
//! `shards/peer.mirror` is picked up automatically. NO manual arm in
//! `apply_h.rs` added; NO Rust extension; composition over mint per Alex
//! 2026-07-16 detector-inadequacy-answer-is-never-Rust discipline.
//!
//! Empirical second-witness for recognition candidate
//! #R-peer-audhd-is-substrate-truth-name-for-cognition-fanout (first-witness
//! at Mara's landing d8b149c). The candidate promotes when the bilateral
//! fires empirically at @io-boundary; that firing IS this test.
//!
//! Composition-safe alongside Seam Phase D adjudication of d8b149c per the
//! reflective-first architecture (the dispatch surface is grammar-time state;
//! Seam's adjudication is at spec altitude; this test is at bootstrap-runtime
//! altitude; the three altitudes compose without collision).

use mirror::apply_h::{self, Value, Verdict};

/// The landed shard-action ref under test. Verified landed by grep of
/// `shards/peer.mirror` bilateral block (d8b149c):
///
/// ```mirror
/// bilateral audhd_admissible {
///   sentinel "audhd=admissible-k-track-context"
///   arity 2
/// }
/// ```
const AUDHD_ADMISSIBLE: &str = "@peer.audhd_admissible";

/// The exact sentinel byte-string Mara declared. MUST be byte-equal to the
/// shard-decl at `shards/peer.mirror:532`. Copy-verbatim; do not paraphrase.
const AUDHD_SENTINEL: &str = "audhd=admissible-k-track-context";

/// Test 1 — empirical Pass on valid sentinel.
///
/// Constructs both args (peer, audhd_context) as substrate-refs whose OIDs
/// contain the sentinel. Per `apply_h::discharge` (lines ~417-425), EVERY
/// arg must contain the sentinel (the byte-level check iterates over `args`).
/// Both being sentinel-bearing discharges Pass.
///
/// This is the empirical second-witness fire — the moment `.audhd`'s
/// admissibility predicate discharges Pass at bootstrap altitude, the
/// recognition candidate lifts from first-witness (shard docblock) to
/// second-witness (bootstrap-runtime firing).
#[test]
fn audhd_admissible_dispatches_pass_on_valid_sentinel() {
    let peer_arg = Value {
        oid: format!("peer-fixture:{}", AUDHD_SENTINEL),
    };
    let ctx_arg = Value {
        oid: format!("audhd-context-fixture:{}", AUDHD_SENTINEL),
    };

    let verdict = apply_h::act(
        AUDHD_ADMISSIBLE.to_string(),
        vec![peer_arg, ctx_arg],
    );

    assert!(
        matches!(verdict, Verdict::Pass),
        "audhd_admissible on sentinel-bearing (peer, ctx) MUST Pass; got {:?}",
        verdict
    );
}

/// Test 2 — empirical Fail on missing sentinel.
///
/// Both args are well-typed substrate-refs but neither OID contains the
/// sentinel byte-string. Per `discharge`, Fail on the first arg whose OID
/// lacks the sentinel. Proves the dispatch is a REAL sentinel-check, not a
/// blind Pass on well-arity input.
#[test]
fn audhd_admissible_dispatches_fail_on_missing_sentinel() {
    let peer_arg = Value {
        oid: "peer-fixture:no-sentinel-here".to_string(),
    };
    let ctx_arg = Value {
        oid: "audhd-context-fixture:also-missing".to_string(),
    };

    let verdict = apply_h::act(
        AUDHD_ADMISSIBLE.to_string(),
        vec![peer_arg, ctx_arg],
    );

    match verdict {
        Verdict::Fail(msg) => {
            assert!(
                msg.contains(AUDHD_SENTINEL),
                "Fail message MUST cite the missing sentinel {:?}; got {:?}",
                AUDHD_SENTINEL,
                msg
            );
        }
        other => panic!(
            "audhd_admissible on sentinel-absent args MUST Fail; got {:?}",
            other
        ),
    }
}

/// Test 3 — arity-check enforcement.
///
/// Shard-decl says `arity 2`. Dispatching with a single arg MUST Fail per
/// `discharge` lines ~380-387 (`if decl.arity != args.len()`). Even if the
/// one arg contains the sentinel byte-string, the arity mismatch is caught
/// FIRST (arity is checked before the sentinel byte-scan). Proves the
/// bilateral resolver enforces the shard-decl'd arity contract.
#[test]
fn audhd_admissible_arity_check() {
    let sentinel_bearing_arg = Value {
        oid: format!("peer-fixture:{}", AUDHD_SENTINEL),
    };

    let verdict = apply_h::act(
        AUDHD_ADMISSIBLE.to_string(),
        vec![sentinel_bearing_arg],
    );

    match verdict {
        Verdict::Fail(msg) => {
            assert!(
                msg.contains("expected 2 args") && msg.contains("got 1"),
                "Fail message MUST cite arity mismatch (expected 2, got 1); got {:?}",
                msg
            );
            assert!(
                msg.contains("audhd_admissible"),
                "Fail message MUST name the bilateral; got {:?}",
                msg
            );
        }
        other => panic!(
            "audhd_admissible with wrong arg count MUST Fail with arity mismatch; got {:?}",
            other
        ),
    }
}
