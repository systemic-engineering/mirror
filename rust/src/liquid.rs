//! `liquid.rs` — the property runtime.
//!
//! Fourth file of the five-file terminal FLOOR (per Mara Round 2 spec
//! extension `docs/specs/rust-floor-five-file-terminal-geometry-
//! extension.md`). Sits between compile.rs (compilation loop) and
//! matrix.rs (sub-Turing numerical primitives).
//!
//! ## Responsibility
//!
//! Read bilateral property declarations from `mirror.spec` + shard
//! bodies (via phone.rs `@io/fs.read`). Instantiate `LiquidVoid<T>`
//! witnesses at prismqueer altitude. Dispatch to pillar primitives.
//! Return `PropertyVerdict`. Consumed by compile.rs at each `@song/beat`
//! crystallization event.
//!
//! ## Per-file five-altitude discipline (Mara `81294b3` + Round 2)
//!
//! - main.rs      supervisor + delegation
//! - **liquid.rs  property runtime** (this file)
//! - compile.rs   compilation loop (SAGA-chain-of-Crystals)
//! - matrix.rs    sub-Turing numerical (LAPACK/BLAS/FLANG)
//! - phone.rs     @io boundary
//!
//! Each file has ONE responsibility. liquid.rs owns property-dispatch;
//! it does NOT own the compilation loop (compile.rs) OR the @io
//! crossing (phone.rs) OR the numerical primitives (matrix.rs). It
//! consumes all three via the module boundary.
//!
//! ## Substrate-decl composition
//!
//! Composes over Mara Round 3 landings (`c8215a3`+`7a334f7`+`9d443dd`+
//! `9cf07d2`+`eac2b30`):
//! - `@system` family-root marker (autopoietic bauchladen × time)
//! - `@mirror/spec/system` grammar species (VSM as .spec)
//! - `@aikido` + `@aikido/reflect` (runtime protocol; Tomm-shape metabolizer)
//! - `@peer/{reflect,redirect,reframe}` three-tier surface
//! - `@beam/system` (kills gen_prism as framing)
//!
//! And over viable.mirror (67th substrate-already-had-the-word;
//! complete Beer VSM property-altitude landing since 2026-07-17).
//!
//! ## Bilateral property declaration surface
//!
//! Every `bilateral <name> { sentinel "..." arity <n> require <ref>* }`
//! block across `shards/**/*.mirror` + `mirror.spec` is a
//! `PropertyDecl` at this altitude. Extraction is byte-scanning per
//! Mara §5.2 dispatch pseudocode (mirrors `bootstrap/src/apply_h.rs
//! ::extract_bilaterals` as FRESH REIMPLEMENTATION per
//! `collapse.rs` precedent — `rust/` doesn't depend on `bootstrap/`).
//!
//! ## Minimum-viable this iteration
//!
//! Iteration 2 of /loop cascade (Alex 2026-07-20). This file lands:
//! - `PropertyDecl` carrier (name + sentinel + arity + require list)
//! - `extract_properties(source: &str) -> Vec<PropertyDecl>` byte-scanner
//! - `dispatch_property(decl, args) -> PropertyVerdict` stub
//!   (composed dispatch through prismqueer::liquid::pillar arrives
//!   at iteration 3+ with the ~20 pillar predicates from the /loop
//!   prompt)
//!
//! Full spec-reading + shard-walking + real pillar dispatch lands at
//! iterations 3-5 of the cascade.

/// A bilateral property declaration extracted from mirror.spec or a
/// shard body. Mirrors `bootstrap/src/apply_h.rs::BilateralDecl`
/// shape (fresh reimplementation per collapse.rs precedent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDecl {
    /// Property name (e.g., "reflection_composes", "aikido_sequence_well_formed").
    pub name: String,
    /// Byte-substring sentinel the dispatch checks against argument OIDs.
    pub sentinel: String,
    /// Number of arguments this property takes.
    pub arity: usize,
    /// Composed-bilateral sub-predicates (each name is a `require`
    /// clause; the composed predicate is well-formed iff every
    /// required sub-predicate holds).
    pub require: Vec<String>,
    /// Source location (shard path + line number where the bilateral
    /// block starts). Empty when not tracked.
    pub source: String,
}

impl PropertyDecl {
    /// True iff this is a composed-bilateral (has require clauses).
    pub fn is_composed(&self) -> bool {
        !self.require.is_empty()
    }

    /// True iff this is a base-bilateral (sentinel-only; no require).
    pub fn is_base(&self) -> bool {
        self.require.is_empty() && !self.sentinel.is_empty()
    }
}

/// Extract bilateral property declarations from a source text.
///
/// Byte-scans line-by-line for `bilateral <name> { sentinel "..."
/// arity <n> require <ref>* }` blocks. Handles multi-line blocks;
/// preserves declaration order.
///
/// This is the minimum-viable extractor — handles the substrate-decl'd
/// bilateral form used across ~30+ landed shards. Grammar edge cases
/// (nested blocks, comments-inside-blocks, etc.) get progressively
/// tightened as the substrate uncovers them.
pub fn extract_properties(source: &str) -> Vec<PropertyDecl> {
    let mut out = Vec::new();
    let mut lines = source.lines().enumerate().peekable();
    while let Some((line_no, line)) = lines.next() {
        let trimmed = line.trim();
        // Look for "bilateral <name> {" opener
        if let Some(rest) = trimmed.strip_prefix("bilateral ") {
            if let Some(name) = rest.split_whitespace().next() {
                if rest.contains('{') {
                    let name = name.to_string();
                    let source_loc = format!("line {}", line_no + 1);
                    let mut sentinel = String::new();
                    let mut arity: usize = 0;
                    let mut require: Vec<String> = Vec::new();
                    // Consume until closing brace
                    while let Some((_, body_line)) = lines.next() {
                        let body_trimmed = body_line.trim();
                        if body_trimmed.starts_with('}') {
                            break;
                        }
                        if let Some(s) = body_trimmed.strip_prefix("sentinel ") {
                            // sentinel "..." — extract content between quotes
                            if let Some(q_start) = s.find('"') {
                                if let Some(q_end) = s[q_start + 1..].find('"') {
                                    sentinel = s[q_start + 1..q_start + 1 + q_end].to_string();
                                }
                            }
                        } else if let Some(s) = body_trimmed.strip_prefix("arity ") {
                            if let Ok(n) = s.trim().parse::<usize>() {
                                arity = n;
                            }
                        } else if let Some(s) = body_trimmed.strip_prefix("require ") {
                            let ref_name = s.split_whitespace().next().unwrap_or("").to_string();
                            if !ref_name.is_empty() {
                                require.push(ref_name);
                            }
                        }
                    }
                    out.push(PropertyDecl {
                        name,
                        sentinel,
                        arity,
                        require,
                        source: source_loc,
                    });
                }
            }
        }
    }
    out
}

/// The verdict returned by property dispatch.
///
/// Deliberately narrow at iteration 2 — lifts to full
/// `terni::PropertyVerdict` shape when iteration 3 wires actual pillar
/// dispatch. Keeps liquid.rs testable in isolation without pulling the
/// full prismqueer/terni surface into main-binary scope this early.
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    /// Property discharged: all invariants hold.
    Pass,
    /// Property refused: at least one invariant violated. Message names
    /// the specific violation (Rice-safe diagnostic).
    Fail(String),
    /// Property deferred: cannot be discharged at this altitude;
    /// forward-promised to a specific downstream landing (message
    /// names the authorship territory).
    Defer(String),
}

impl Verdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, Verdict::Pass)
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, Verdict::Fail(_))
    }
    pub fn is_defer(&self) -> bool {
        matches!(self, Verdict::Defer(_))
    }
}

/// Dispatch a property declaration against argument OIDs.
///
/// Iteration 2 stub: returns `Defer` for all inputs, naming the
/// authorship territory (pillar-predicate dispatch lands at iterations
/// 3+ with the ~20 predicates from the /loop cascade prompt).
///
/// The signature is stable at this iteration; only the body evolves as
/// iteration 3+ wires prismqueer::liquid::pillar dispatch.
pub fn dispatch_property(decl: &PropertyDecl, args: &[String]) -> Verdict {
    if args.len() != decl.arity {
        return Verdict::Fail(format!(
            "arity mismatch: property `{}` expects {} args, got {}",
            decl.name,
            decl.arity,
            args.len()
        ));
    }
    Verdict::Defer(format!(
        "iteration-2 stub: dispatch for `{}` composes prismqueer::liquid::pillar predicates at iterations 3+ per /loop cascade brief (Alex 2026-07-20)",
        decl.name
    ))
}

// =====================================================================
// Property tests — extractor + dispatcher + Verdict invariants.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_finds_single_base_bilateral() {
        let source = r#"
# some prose
bilateral foo_admissible {
  sentinel "foo=well-formed"
  arity 1
}
# more prose
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "foo_admissible");
        assert_eq!(props[0].sentinel, "foo=well-formed");
        assert_eq!(props[0].arity, 1);
        assert!(props[0].require.is_empty());
        assert!(props[0].is_base());
    }

    #[test]
    fn extract_finds_composed_bilateral() {
        let source = r#"
bilateral composed_of_admissibles {
  sentinel "composed=all-hold"
  arity 2
  require foo_admissible
  require bar_admissible
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].require, vec!["foo_admissible", "bar_admissible"]);
        assert!(props[0].is_composed());
        assert!(!props[0].is_base());
    }

    #[test]
    fn extract_finds_multiple_bilaterals_preserving_order() {
        let source = r#"
bilateral first {
  sentinel "a"
  arity 1
}

bilateral second {
  sentinel "b"
  arity 2
}

bilateral third {
  sentinel "c"
  arity 3
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].name, "first");
        assert_eq!(props[1].name, "second");
        assert_eq!(props[2].name, "third");
        assert_eq!(props[0].arity, 1);
        assert_eq!(props[1].arity, 2);
        assert_eq!(props[2].arity, 3);
    }

    #[test]
    fn extract_returns_empty_when_no_bilaterals() {
        let source = "# just prose\n# no bilateral blocks here\n";
        let props = extract_properties(source);
        assert!(props.is_empty());
    }

    #[test]
    fn extract_tracks_source_line_number() {
        let source = r#"# line 1
# line 2
bilateral on_line_three {
  sentinel "located"
  arity 1
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].source, "line 3");
    }

    #[test]
    fn dispatch_arity_mismatch_returns_fail() {
        let decl = PropertyDecl {
            name: "needs_two".to_string(),
            sentinel: "x".to_string(),
            arity: 2,
            require: vec![],
            source: "test".to_string(),
        };
        let v = dispatch_property(&decl, &["one".to_string()]);
        assert!(v.is_fail());
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("arity mismatch"));
            assert!(msg.contains("needs_two"));
        }
    }

    #[test]
    fn dispatch_matching_arity_returns_defer_at_iteration_2() {
        let decl = PropertyDecl {
            name: "stub".to_string(),
            sentinel: "x".to_string(),
            arity: 1,
            require: vec![],
            source: "test".to_string(),
        };
        let v = dispatch_property(&decl, &["arg".to_string()]);
        assert!(v.is_defer());
        if let Verdict::Defer(msg) = v {
            assert!(msg.contains("iteration-2 stub"));
            assert!(msg.contains("iterations 3+"));
        }
    }

    #[test]
    fn verdict_predicates_are_mutually_exclusive() {
        let p = Verdict::Pass;
        let f = Verdict::Fail("err".to_string());
        let d = Verdict::Defer("later".to_string());
        assert!(p.is_pass() && !p.is_fail() && !p.is_defer());
        assert!(!f.is_pass() && f.is_fail() && !f.is_defer());
        assert!(!d.is_pass() && !d.is_fail() && d.is_defer());
    }

    #[test]
    fn realistic_extraction_from_mirror_reflection_shape() {
        // Mirrors the actual shape at `shards/mirror/reflection.mirror`
        // (Mara `5e1f528`) — the 4-bilateral composed structure.
        let source = r#"
bilateral reflection_truthful {
  sentinel "mirror=traceable-non-labeling"
  arity 1
}

bilateral offer_ado_valid {
  sentinel "offer=ado-wrapped-gift-declinable"
  arity 1
}

bilateral wait_holds_without_pressure {
  sentinel "wait=held-without-pressure"
  arity 1
}

bilateral reflection_composes {
  sentinel "reflection=three-op-composed"
  arity 3
  require reflection_truthful
  require offer_ado_valid
  require wait_holds_without_pressure
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].name, "reflection_composes");
        assert_eq!(props[3].arity, 3);
        assert_eq!(props[3].require.len(), 3);
        assert!(props[3].is_composed());
        // The three sub-bilaterals are base (sentinel-only).
        assert!(props[0].is_base());
        assert!(props[1].is_base());
        assert!(props[2].is_base());
    }
}
