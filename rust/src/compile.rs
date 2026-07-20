//! `compile.rs` — the compilation loop.
//!
//! Third file of the five-file terminal FLOOR (per Mara Round 2 spec
//! extension `docs/specs/rust-floor-five-file-terminal-geometry-
//! extension.md`). Sits between main.rs (supervisor + @-operator
//! dispatch) and liquid.rs (property runtime).
//!
//! ## Responsibility
//!
//! Orchestrate the compilation loop as a SAGA-chain-of-Crystals. Each
//! compilation reads a source (mirror.spec fragment, shard body, or
//! arbitrary bilateral-declaration corpus); walks its property
//! declarations in extraction order; dispatches each through liquid.rs;
//! crystallizes the discharge with content-addressed OID chained to
//! the previous crystal. The resulting chain IS what `@peer.redirect`
//! walks; the chain-head IS the compilation's "current settled state"
//! per Alex 2026-07-20 Crystal<T>/Mandelbrot<T> recognition.
//!
//! ## Per-file five-altitude discipline (Mara `81294b3` + Round 2)
//!
//! - main.rs      supervisor + delegation
//! - **compile.rs compilation loop** (this file; SAGA orchestration)
//! - liquid.rs    property runtime
//! - matrix.rs    sub-Turing numerical (LAPACK/BLAS/FLANG)
//! - phone.rs     @io boundary
//!
//! compile.rs owns the SAGA-chain-of-Crystals orchestration; it does
//! NOT own property dispatch (liquid.rs) OR the @io crossing (phone.rs)
//! OR the numerical primitives (matrix.rs). It composes all three via
//! the module boundary.
//!
//! ## Consent-through-refusal (Mara @peer three-tier surface)
//!
//! Per Mara Round 3 `shards/peer.mirror` + `docs/specs/peer-three-tier-
//! reflect-redirect-reframe.md`: every @peer interaction dispatches
//! through three tiers of graduated response:
//!
//! - `reflect` (Pass verdict) — the property discharged cleanly;
//!   crystallize + continue.
//! - `redirect` (Defer verdict) — the property forwards to a
//!   downstream authorship territory; crystallize + continue with the
//!   Defer message pinned to the crystal (walkable evidence when the
//!   authorship territory lands).
//! - `reframe` (Fail verdict) — the property refused; crystallize the
//!   refusal + emit an Escalate signal that names the offending
//!   crystal's OID. The SAGA chain becomes walkable evidence for the
//!   @peer.redirect(oid) call the caller makes when re-litigation is
//!   refused.
//!
//! The `Escalation` enum below is the compile-time projection of this
//! three-tier surface. Alex 2026-07-20 direct-transcript: "the
//! algedonic contradiction pain measurement is the internal threshold
//! the peer uses to determine the level of escalation." This file
//! stakes the algedonic-threshold surface at Continue/Escalate/Halt
//! without making the threshold-tuning decision (Mara authorship
//! territory; Round 4+).
//!
//! ## Substrate-decl composition
//!
//! Composes over:
//! - `fractal::Crystal<T>` + `fractal::Oid` (iter 1; `a3dc905`) — the
//!   settled-interior state carrier + content-addressed identifier.
//! - `fractal::crystallize<T>` (iter 1; `a3dc905`) — the @time/now
//!   Liquid → Crystal transition.
//! - `fractal::Witnessed` (fractal step 2) — MARA doctrine
//!   Author ≠ Committer provenance.
//! - `liquid::PropertyDecl` + `liquid::extract_properties` +
//!   `liquid::dispatch_property` + `liquid::Verdict` (iter 2;
//!   `8c174c5`) — property runtime surface.
//! - `viable.mirror` (67th substrate-already-had-the-word; complete
//!   Beer VSM property-altitude landing since 2026-07-17) — VSM as
//!   the compilation loop's referent geometry.
//!
//! ## Minimum-viable this iteration
//!
//! Iteration 3 of /loop cascade (Alex 2026-07-20). This file lands:
//! - `PropertyDischarge` carrier (property name + verdict)
//! - `Escalation` enum (Continue / Escalate / Halt)
//! - `Compilation` result carrier (crystals chain + discharges + escalation)
//! - `compile_declarations(decls, args_per_decl, &witnessed) -> Compilation`
//!   — the core SAGA loop
//! - `compile_from_source(source, &witnessed) -> Compilation`
//!   — extractor + loop composition
//! - `serialize_discharge` — deterministic bytes for OID hashing
//!
//! Full pillar-predicate dispatch (~20 predicates from /loop cascade
//! prompt) lands at iter 5+ via liquid.rs upgrade; main.rs refactor to
//! delegation lands at iter 4.

use crate::liquid::{dispatch_property, extract_properties, PropertyDecl, Verdict};
use fractal::{crystallize, Crystal, Oid, Witnessed};

/// One property discharge — the verdict of one dispatch call plus the
/// property name that produced it. Serialized to bytes for OID hashing
/// via `serialize_discharge`.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDischarge {
    pub property_name: String,
    pub verdict: Verdict,
}

/// The algedonic-threshold projection at compile-time.
///
/// Per Alex 2026-07-20 direct-transcript on @peer three-tier +
/// algedonic-pain threshold:
///
/// - `Continue` — all discharges either Passed (reflect) or Deferred
///   (redirect); no refuse-tier signals. Chain walks forward.
/// - `Escalate(oid)` — at least one discharge Failed (reframe); the
///   OID names the FIRST failing crystal so `@peer.redirect(oid)`
///   walks BACK to the specific refusal for evidence.
/// - `Halt(msg)` — the compile loop cannot proceed at this altitude
///   without downstream authorship territory landing (e.g., an
///   unresolvable Defer chain that names an authorship territory not
///   yet minted). Distinct from Escalate: Halt is a boundary signal
///   to the caller, not a refusal.
#[derive(Debug, Clone, PartialEq)]
pub enum Escalation {
    Continue,
    Escalate(Oid),
    Halt(String),
}

impl Escalation {
    pub fn is_continue(&self) -> bool {
        matches!(self, Escalation::Continue)
    }
    pub fn is_escalate(&self) -> bool {
        matches!(self, Escalation::Escalate(_))
    }
    pub fn is_halt(&self) -> bool {
        matches!(self, Escalation::Halt(_))
    }
}

/// The result of one compilation — the SAGA chain plus escalation.
///
/// - `crystals` — extraction-order Crystal chain; each Crystal's `prev`
///   points to the previous crystal's OID (or `Oid::GENESIS` for the
///   first). This is what `@peer.redirect` walks.
/// - `discharges` — parallel array of the discharge that each crystal
///   settled; index-aligned with `crystals`.
/// - `escalation` — the aggregate algedonic signal (Continue if all
///   Pass/Defer; Escalate(oid) at first Fail).
#[derive(Debug, Clone)]
pub struct Compilation {
    pub crystals: Vec<Crystal<Vec<u8>>>,
    pub discharges: Vec<PropertyDischarge>,
    pub escalation: Escalation,
}

impl Compilation {
    /// The OID of the current chain head (`Oid::GENESIS` for empty
    /// compilations). What `@time/now` returns.
    pub fn head_oid(&self) -> Oid {
        match self.crystals.last() {
            Some(c) => c.oid.clone(),
            None => Oid::GENESIS,
        }
    }

    /// Total discharges walked (= `crystals.len()` = `discharges.len()`).
    pub fn depth(&self) -> usize {
        self.crystals.len()
    }
}

/// Deterministic serialization of a discharge to bytes for OID hashing.
///
/// Fixed shape (property=<name>\nverdict=<kind>\nmessage=<msg>\n) so
/// the same discharge always produces byte-identical output. This is
/// what makes crystallize's content-addressing invariant hold across
/// compilation replays.
pub fn serialize_discharge(d: &PropertyDischarge) -> Vec<u8> {
    let (kind, message) = match &d.verdict {
        Verdict::Pass => ("pass", String::new()),
        Verdict::Fail(msg) => ("fail", msg.clone()),
        Verdict::Defer(msg) => ("defer", msg.clone()),
    };
    format!(
        "property={}\nverdict={}\nmessage={}\n",
        d.property_name, kind, message
    )
    .into_bytes()
}

/// Compile a list of property declarations with per-decl args. The
/// SAGA loop.
///
/// For each decl in order: dispatch through liquid → construct
/// PropertyDischarge → serialize → crystallize with prev=previous
/// crystal's OID → push. First Fail sets escalation to
/// `Escalate(oid)` (OID of the offending crystal); subsequent Fails
/// do NOT overwrite (the first refuse is the load-bearing walk-target
/// per @peer.redirect discipline).
///
/// `args_per_decl` MUST be index-aligned with `decls`; if shorter, the
/// remaining decls receive empty args (Fail for arity>0, Defer for
/// arity=0).
pub fn compile_declarations(
    decls: &[PropertyDecl],
    args_per_decl: &[Vec<String>],
    witnessed: &Witnessed,
) -> Compilation {
    let mut crystals: Vec<Crystal<Vec<u8>>> = Vec::with_capacity(decls.len());
    let mut discharges: Vec<PropertyDischarge> = Vec::with_capacity(decls.len());
    let mut prev_oid = Oid::GENESIS;
    let mut escalation = Escalation::Continue;
    let empty: Vec<String> = Vec::new();

    for (i, decl) in decls.iter().enumerate() {
        let args: &[String] = args_per_decl.get(i).map(Vec::as_slice).unwrap_or(&empty);
        let verdict = dispatch_property(decl, args);
        let discharge = PropertyDischarge {
            property_name: decl.name.clone(),
            verdict: verdict.clone(),
        };
        let bytes = serialize_discharge(&discharge);
        let crystal = crystallize(bytes, witnessed.clone(), prev_oid.clone());

        // First Fail sets Escalate; subsequent Fails don't overwrite.
        if verdict.is_fail() && escalation.is_continue() {
            escalation = Escalation::Escalate(crystal.oid.clone());
        }

        prev_oid = crystal.oid.clone();
        crystals.push(crystal);
        discharges.push(discharge);
    }

    Compilation {
        crystals,
        discharges,
        escalation,
    }
}

/// Compile a source text. Extracts property declarations via
/// liquid::extract_properties, then dispatches with empty args per
/// decl (iter-3 stub — args-passing at compile call sites lands with
/// pillar-predicate dispatch at iter 5+).
pub fn compile_from_source(source: &str, witnessed: &Witnessed) -> Compilation {
    let decls = extract_properties(source);
    let args: Vec<Vec<String>> = vec![Vec::new(); decls.len()];
    compile_declarations(&decls, &args, witnessed)
}

// =====================================================================
// Property tests — SAGA-chain invariants + escalation dispatch.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use fractal::{Author, Committer, Timestamp};

    fn test_witnessed() -> Witnessed {
        Witnessed::new(
            Author::new("reed", "reed@spectral.engineer"),
            Committer::new("mirror", "mirror@spectral.engineer"),
            Timestamp("1700000000".to_string()),
        )
    }

    fn decl_zero_arity(name: &str) -> PropertyDecl {
        PropertyDecl {
            name: name.to_string(),
            sentinel: format!("{}=witnessed", name),
            arity: 0,
            require: vec![],
            source: "test".to_string(),
        }
    }

    fn decl_needs_two(name: &str) -> PropertyDecl {
        PropertyDecl {
            name: name.to_string(),
            sentinel: format!("{}=witnessed", name),
            arity: 2,
            require: vec![],
            source: "test".to_string(),
        }
    }

    #[test]
    fn empty_source_produces_empty_compilation_at_genesis() {
        let comp = compile_from_source("# just prose\n", &test_witnessed());
        assert_eq!(comp.depth(), 0);
        assert!(comp.crystals.is_empty());
        assert!(comp.discharges.is_empty());
        assert_eq!(comp.head_oid(), Oid::GENESIS);
        assert!(comp.escalation.is_continue());
    }

    #[test]
    fn single_bilateral_produces_single_crystal_chained_to_genesis() {
        // Use a name NOT registered as a pillar (iter 5) so dispatch
        // falls through to Defer; keeps the test's intent (single
        // bilateral → single crystal genesis-chain) decoupled from the
        // pillar-dispatch semantics that other tests cover explicitly.
        let source = r#"
bilateral not_yet_registered_pillar_name {
  sentinel "unregistered=deferred"
  arity 0
}
"#;
        let comp = compile_from_source(source, &test_witnessed());
        assert_eq!(comp.depth(), 1);
        assert_eq!(comp.crystals[0].prev(), &Oid::GENESIS);
        assert!(comp.crystals[0].is_genesis());
        assert_eq!(comp.discharges[0].property_name, "not_yet_registered_pillar_name");
        // Unregistered names defer through pillar::dispatch fallthrough.
        assert!(comp.discharges[0].verdict.is_defer());
        assert!(comp.escalation.is_continue());
    }

    #[test]
    fn multi_bilateral_produces_walkable_saga_chain() {
        // Three zero-arity bilaterals → three-crystal chain, each
        // crystal's prev points to the previous crystal's OID. This is
        // what @peer.redirect walks BACK through.
        let source = r#"
bilateral first {
  sentinel "a"
  arity 0
}

bilateral second {
  sentinel "b"
  arity 0
}

bilateral third {
  sentinel "c"
  arity 0
}
"#;
        let comp = compile_from_source(source, &test_witnessed());
        assert_eq!(comp.depth(), 3);
        // First crystal chains from GENESIS.
        assert!(comp.crystals[0].is_genesis());
        // Second crystal's prev == first's OID.
        assert_eq!(comp.crystals[1].prev(), &comp.crystals[0].oid);
        // Third crystal's prev == second's OID.
        assert_eq!(comp.crystals[2].prev(), &comp.crystals[1].oid);
        // Head OID is the last crystal's OID.
        assert_eq!(comp.head_oid(), comp.crystals[2].oid);
        assert!(comp.escalation.is_continue());
    }

    #[test]
    fn arity_mismatch_dispatches_fail_and_sets_escalate() {
        // A single arity=2 decl with empty args → Fail (arity mismatch)
        // → Escalation::Escalate(oid) with the offending crystal's OID.
        let decls = vec![decl_needs_two("needs_two_args")];
        let args: Vec<Vec<String>> = vec![Vec::new()]; // empty args
        let comp = compile_declarations(&decls, &args, &test_witnessed());
        assert_eq!(comp.depth(), 1);
        assert!(comp.discharges[0].verdict.is_fail());
        match &comp.escalation {
            Escalation::Escalate(oid) => {
                assert_eq!(oid, &comp.crystals[0].oid);
            }
            other => panic!("expected Escalate(oid), got {:?}", other),
        }
    }

    #[test]
    fn first_fail_pins_escalate_oid_subsequent_fails_do_not_overwrite() {
        // Two arity=2 decls, both dispatched with empty args → both
        // Fail. Escalation MUST point to the FIRST failing crystal's
        // OID (that's the load-bearing walk-target).
        let decls = vec![decl_needs_two("first_fail"), decl_needs_two("second_fail")];
        let args: Vec<Vec<String>> = vec![Vec::new(), Vec::new()];
        let comp = compile_declarations(&decls, &args, &test_witnessed());
        assert_eq!(comp.depth(), 2);
        assert!(comp.discharges[0].verdict.is_fail());
        assert!(comp.discharges[1].verdict.is_fail());
        match &comp.escalation {
            Escalation::Escalate(oid) => {
                assert_eq!(
                    oid, &comp.crystals[0].oid,
                    "escalate MUST pin the first failing crystal's OID"
                );
                assert_ne!(oid, &comp.crystals[1].oid);
            }
            other => panic!("expected Escalate(oid), got {:?}", other),
        }
    }

    #[test]
    fn compilation_is_deterministic_across_replays() {
        // Same source + same witnessed MUST produce byte-identical
        // crystal OIDs. This is the content-addressing invariant that
        // makes @peer.redirect refuse re-litigation via OID-verifiability.
        // Uses unregistered-pillar bilateral names to keep verdict-shape
        // stable across iter 6 dispatch (both Defer via fallthrough).
        let source = r#"
bilateral compilation_replay_bilateral_a {
  sentinel "replay=stable-a"
  arity 0
}

bilateral compilation_replay_bilateral_b {
  sentinel "replay=stable-b"
  arity 0
}
"#;
        let comp1 = compile_from_source(source, &test_witnessed());
        let comp2 = compile_from_source(source, &test_witnessed());
        assert_eq!(comp1.depth(), comp2.depth());
        for (c1, c2) in comp1.crystals.iter().zip(comp2.crystals.iter()) {
            assert_eq!(c1.oid, c2.oid, "compilation MUST be deterministic");
        }
        assert_eq!(comp1.head_oid(), comp2.head_oid());
    }

    #[test]
    fn different_witnessed_produces_different_chain_oids() {
        // MARA doctrine at compile altitude: swap Author → different
        // Witnessed → different Crystal OIDs, even with identical source.
        let source = r#"
bilateral witnessed_matters {
  sentinel "witness=part-of-hash"
  arity 0
}
"#;
        let w1 = test_witnessed();
        let mut w2 = test_witnessed();
        w2.author = Author::new("mara", "mara@spectral.engineer");

        let comp1 = compile_from_source(source, &w1);
        let comp2 = compile_from_source(source, &w2);
        assert_eq!(comp1.depth(), 1);
        assert_eq!(comp2.depth(), 1);
        assert_ne!(
            comp1.crystals[0].oid, comp2.crystals[0].oid,
            "MARA doctrine: different Witnessed MUST produce different crystal OIDs"
        );
    }

    #[test]
    fn serialize_discharge_is_deterministic() {
        let d = PropertyDischarge {
            property_name: "deterministic".to_string(),
            verdict: Verdict::Defer("stub message".to_string()),
        };
        let b1 = serialize_discharge(&d);
        let b2 = serialize_discharge(&d);
        assert_eq!(b1, b2);
        // Fixed shape check.
        let s = String::from_utf8(b1).unwrap();
        assert!(s.starts_with("property=deterministic\n"));
        assert!(s.contains("verdict=defer"));
        assert!(s.contains("message=stub message"));
    }

    #[test]
    fn serialize_discharge_distinguishes_all_three_verdicts() {
        let pass = PropertyDischarge {
            property_name: "p".to_string(),
            verdict: Verdict::Pass,
        };
        let fail = PropertyDischarge {
            property_name: "p".to_string(),
            verdict: Verdict::Fail("err".to_string()),
        };
        let defer = PropertyDischarge {
            property_name: "p".to_string(),
            verdict: Verdict::Defer("later".to_string()),
        };
        let bp = serialize_discharge(&pass);
        let bf = serialize_discharge(&fail);
        let bd = serialize_discharge(&defer);
        assert_ne!(bp, bf);
        assert_ne!(bp, bd);
        assert_ne!(bf, bd);
    }

    #[test]
    fn realistic_vsm_system_block_produces_expected_saga_chain() {
        // Mirrors the Round 3 `system @X { ... }` grammar carrying a
        // VSM invariant + autopoietic-closure bilateral + Beer
        // feedback-loops shape.
        //
        // Iter 6 update: vsm_invariants + autopoietic_lawvere_fixed_
        // point are NOW registered pillars (fire Fail on empty args
        // because their arities don't match the empty-args stub).
        // beer_feedback_loops_topology remains unregistered (iter 7+
        // authorship territory), stays Defer. Renamed to non-registered
        // pillar names to preserve the test's original intent (3-
        // crystal Defer chain → Continue escalation); iter 7+ will land
        // these predicates and enable an empirical registered-pillar
        // version of this test.
        let source = r#"
bilateral beer_feedback_loops_topology {
  sentinel "beer=feedback-loops-complete"
  arity 0
}

bilateral system_composition_verified_4d {
  sentinel "composed=autopoiesis-x-bauchladen-x-time-x-viability"
  arity 0
}

bilateral history_with_returns_crystal_chain {
  sentinel "history=crystal-chain-walkable"
  arity 0
}
"#;
        let comp = compile_from_source(source, &test_witnessed());
        assert_eq!(comp.depth(), 3);
        assert!(comp.discharges.iter().all(|d| d.verdict.is_defer()));
        assert!(comp.escalation.is_continue());
        // Chain walkable: each crystal's prev matches the previous OID.
        assert!(comp.crystals[0].is_genesis());
        assert_eq!(comp.crystals[1].prev(), &comp.crystals[0].oid);
        assert_eq!(comp.crystals[2].prev(), &comp.crystals[1].oid);
        // Head OID == last crystal's OID; this is what @time/now returns.
        assert_eq!(comp.head_oid(), comp.crystals[2].oid);
    }

    #[test]
    fn escalation_predicates_are_mutually_exclusive() {
        let c = Escalation::Continue;
        let e = Escalation::Escalate(Oid::GENESIS);
        let h = Escalation::Halt("reason".to_string());
        assert!(c.is_continue() && !c.is_escalate() && !c.is_halt());
        assert!(!e.is_continue() && e.is_escalate() && !e.is_halt());
        assert!(!h.is_continue() && !h.is_escalate() && h.is_halt());
    }

    // =================================================================
    // build.rs ↔ compile.rs collapse arc — RED + GREEN sequence per
    // Alex 2026-07-20 "we don't want to blindly copy paste" +
    // build.rs vs compile.rs "wouldn't these want to be the same?"
    // + "slow is fast, no cruft, verified substrate".
    //
    // Terminal shape: ONE compile discipline. `mirror compile
    // @mirror.spec` compiles mirror itself. build.rs collapses to a
    // ~10 LOC bootstrap shim shelling to `mirror compile`. Every
    // compile step — building the compiler binary, compiling a
    // downstream .mirror spec, cross-compiling, rebuilding mirror
    // from its own spec — goes through compile.rs.
    //
    // Each RED tick expresses a collapse-invariant currently violated.
    // Each GREEN tick closes the gap. Small ticks. Sequential commits.
    // Slow is fast.
    // =================================================================

    /// **RED tick 1** — compile.rs currently only speaks `bilateral`
    /// grammar. mirror.spec speaks `project` + `target` + `cli` +
    /// `command` grammar. To close the compile-collapse, compile.rs
    /// MUST see mirror.spec's declared substance.
    ///
    /// Currently RED: extract_properties finds zero bilateral blocks
    /// in mirror.spec; the SAGA chain is empty. GREEN transition:
    /// extract_commands (or generalized extract_declarations) that
    /// recognizes `command <name> { ... }` blocks at nested altitude,
    /// composed into compile_from_source's SAGA orchestration.
    ///
    /// This test WILL FAIL at RED tick 1 landing. GREEN tick 1 lands
    /// the extraction extension and removes the #[should_panic].
    #[test]
    #[should_panic(expected = "RED tick 1")]
    fn compile_reads_mirror_spec_grammar_beyond_bilaterals_red_tick_1() {
        // Read the actual mirror.spec at the repo root. If this test
        // runs from cargo test at rust/, the relative path resolves
        // to ../mirror.spec.
        let paths = [
            "../mirror.spec",
            "/Users/alexwolf/dev/projects/mirror/mirror.spec",
        ];
        let source = paths
            .iter()
            .find_map(|p| std::fs::read_to_string(p).ok())
            .expect("could not locate mirror.spec");

        let comp = compile_from_source(&source, &test_witnessed());

        // mirror.spec declares (grep'd 2026-07-20 pre-RED) 9 top-level
        // cli commands (compile, kintsugi, shatter, craft, init,
        // recall, beam, index, peer) plus 2 nested peer subcommands
        // (beam, contribute) = 11 command declarations. compile.rs
        // MUST see them.
        //
        // Currently comp.depth() == 0 because extract_properties only
        // handles `bilateral` blocks. This assert fires the RED marker
        // so #[should_panic] catches it and documents the failing
        // invariant in the test log.
        let cmd_count = comp.depth();
        assert!(
            cmd_count >= 11,
            "RED tick 1: compile.rs of mirror.spec produced {} crystals; expected ≥11 (9 top-level commands + 2 peer subcommands); extract_commands not yet landed",
            cmd_count
        );
    }

    #[test]
    fn compile_declarations_with_missing_args_treats_as_empty() {
        // args_per_decl shorter than decls → remaining decls get empty
        // args. Substrate-honest fallback for iter-3 stub composition.
        let decls = vec![
            decl_zero_arity("a"),
            decl_zero_arity("b"),
            decl_zero_arity("c"),
        ];
        let args: Vec<Vec<String>> = Vec::new(); // empty → all decls get empty args
        let comp = compile_declarations(&decls, &args, &test_witnessed());
        assert_eq!(comp.depth(), 3);
        assert!(comp.discharges.iter().all(|d| d.verdict.is_defer()));
    }
}
