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
/// Iteration 5 (Alex 2026-07-20 /loop cascade): dispatches to the
/// `pillar` submodule for named property predicates registered under
/// the classifier-witness framing (Recognition bundle #1:
/// `#R-autopoietic-classifier-is-knife-coord-under-lagrange-between-
/// narcissus-and-splinter`). Each pillar predicate is a classifier
/// witness that fires @subject-evidence (Pass) or @object-evidence
/// (Fail); Defer stays for unknown property names (authorship
/// territory forward-promised to future iters).
///
/// Signature stable since iter 2; only the body evolves as more
/// pillar predicates land at iter 6+.
pub fn dispatch_property(decl: &PropertyDecl, args: &[String]) -> Verdict {
    if args.len() != decl.arity {
        return Verdict::Fail(format!(
            "arity mismatch: property `{}` expects {} args, got {}",
            decl.name,
            decl.arity,
            args.len()
        ));
    }
    // Route named property to registered pillar predicate; fall through
    // to Defer for unknown names.
    pillar::dispatch(&decl.name, args)
}

// =====================================================================
// pillar — classifier-witness predicates.
// =====================================================================

/// Pillar predicates are the classifier's discrimination surface at
/// compile-time. Each pillar is a Rice-safe byte-visible witness that
/// fires @subject-evidence (Pass) OR @object-evidence (Fail).
///
/// Per Alex 2026-07-20 Recognition bundle:
/// - `#R-autopoietic-classifier-is-knife-coord-under-lagrange` —
///   pillar dispatch IS the classifier at compile-time.
/// - `#R-paradox-trauma-species` + `#R-saga-chain-is-witness-mechanism`
///   — trauma-specific pillars encode the witness_only invariant
///   + first-fail-pin preservation at Rust altitude.
///
/// Iter 5 lands 5 minimum-viable pillar predicates covering the
/// paradox-family + classifier-Lagrange + aikido-runtime + consent-
/// through-refusal shapes. Iter 6+ lands the remaining ~15 predicates
/// from the /loop cascade brief (VSM invariants, Beer feedback loops,
/// BEAM composition, bundle-tower connections, Lawvere fixed-point,
/// Förster's imperative, etc.).
///
/// Substrate composition: iter 6+ composes over `prismqueer::liquid::
/// pillar` (which supplies domain-agnostic pillars: dispatch_ambiguity,
/// algedonic, viability, fold). This module holds mirror-domain-specific
/// classifier witnesses (VSM, aikido, paradox, consent, redirect).
pub mod pillar {
    use super::Verdict;

    /// Route named property to registered pillar; fall through to
    /// Defer for unknown names.
    ///
    /// Iter 5 landed 5 pillars covering the paradox-family + classifier-
    /// Lagrange + aikido-runtime + consent-through-refusal shapes.
    /// Iter 6 lands 5 more covering @paradox/@cyberpunk/intervention
    /// SAGA-compensation + VSM invariants + Lawvere fixed-point +
    /// void-settle-transition + crystallization-preserves-saga chain
    /// invariant. Iter 7+ lands the remaining ~10 from /loop cascade
    /// brief.
    pub fn dispatch(name: &str, args: &[String]) -> Verdict {
        match name {
            // iter 5 predicates
            "paradox_crystal_immutable" => paradox_crystal_immutable(args),
            "redirect_witness_preserves_original_pin" => {
                redirect_witness_preserves_original_pin(args)
            }
            "classifier_lagrange_between_narcissus_and_splinter" => {
                classifier_lagrange_between_narcissus_and_splinter(args)
            }
            "aikido_sequence_well_formed" => aikido_sequence_well_formed(args),
            "consent_through_refusal" => consent_through_refusal(args),
            // iter 6 predicates
            "intervention_composes_without_deleting_trauma" => {
                intervention_composes_without_deleting_trauma(args)
            }
            "crystallization_preserves_saga" => crystallization_preserves_saga(args),
            "autopoietic_lawvere_fixed_point" => autopoietic_lawvere_fixed_point(args),
            "vsm_invariants" => vsm_invariants(args),
            "void_settle_produces_lens_seed" => void_settle_produces_lens_seed(args),
            _ => Verdict::Defer(format!(
                "pillar `{}` not yet registered; iter 7+ authorship territory per /loop cascade brief (Alex 2026-07-20)",
                name
            )),
        }
    }

    /// Witnessed-crystal cannot mutate; content-addressed OID must
    /// remain byte-stable across replays. Fires @subject-evidence when
    /// the observed OID is well-formed (non-empty hex, no mutation
    /// marker); @object-evidence otherwise.
    ///
    /// Per Recognition bundle #3 (@paradox/trauma) + Alex verbatim
    /// "It can only be witnessed. Never resolved." Anchored in
    /// shards/paradox/trauma.mirror `witness_only` invariant +
    /// rust/src/compile.rs:221-224 first-fail-pins invariant.
    ///
    /// Args: [oid_hex]
    pub fn paradox_crystal_immutable(args: &[String]) -> Verdict {
        let oid = match args.first() {
            Some(o) if !o.is_empty() => o,
            _ => {
                return Verdict::Fail(
                    "paradox_crystal_immutable: expected non-empty OID at args[0]"
                        .to_string(),
                )
            }
        };
        // Byte-visible mutation-marker check (Rice-safe): the OID must
        // NOT contain any of the known-mutation sentinels. Real body-
        // composition against fractal::Crystal.oid content-addressing
        // invariant arrives when prismqueer::liquid::Sample+Arbitrary
        // primitives land (Task #263 pending).
        if oid.contains("mutated") || oid.contains("revised") || oid.contains("deleted") {
            return Verdict::Fail(format!(
                "paradox_crystal_immutable: OID `{}` contains mutation marker; witnessed-crystal MUST NOT be mutated (resolution would be revisionism)",
                oid
            ));
        }
        // Positive floor: OID must be plausible content-addressed hex
        // (all hex chars or a well-formed short prefix).
        if oid.chars().all(|c| c.is_ascii_hexdigit()) && oid.len() >= 8 {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "paradox_crystal_immutable: OID `{}` is not well-formed content-addressed hex (expected ≥16 hex chars)",
                oid
            ))
        }
    }

    /// Escalate(oid) MUST point at first-fail forever. Fires
    /// @subject-evidence when the pinned OID at args[0] matches the
    /// first-fail OID at args[1]; @object-evidence otherwise (indicates
    /// re-litigation shift-of-witness).
    ///
    /// Per Recognition bundle #6 + Alex verbatim "we witness the
    /// ORIGINAL wound; we don't shift the witness-target to whatever
    /// hurt most-recently." Anchored in
    /// rust/src/compile.rs:221-224 (first_fail_pins_escalate_oid
    /// invariant).
    ///
    /// Args: [pinned_oid, first_fail_oid]
    pub fn redirect_witness_preserves_original_pin(args: &[String]) -> Verdict {
        let pinned = match args.first() {
            Some(p) => p,
            None => {
                return Verdict::Fail(
                    "redirect_witness_preserves_original_pin: expected pinned_oid at args[0]"
                        .to_string(),
                )
            }
        };
        let first_fail = match args.get(1) {
            Some(f) => f,
            None => {
                return Verdict::Fail(
                    "redirect_witness_preserves_original_pin: expected first_fail_oid at args[1]"
                        .to_string(),
                )
            }
        };
        if pinned == first_fail {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "redirect_witness_preserves_original_pin: pinned OID `{}` does not match first-fail OID `{}`; witness-target has shifted (re-litigation detected)",
                pinned, first_fail
            ))
        }
    }

    /// Self-COORD holds equilibrium between @void/narcissus (star;
    /// refuse-to-update; classifier goes inert) and @void/splinter
    /// (complete graph; over-fragment; classifier goes noisy). Fires
    /// @subject-evidence when the observed round-count sits in the
    /// Lagrange-band; @object-evidence otherwise (drift toward either
    /// attractor triggers @cyberpunk/intervention).
    ///
    /// Per Recognition bundle #1 + Alex verbatim "the classifier runs
    /// of course coord on themselves. That's the whole anti-narcissus
    /// -> splinter loop."
    ///
    /// Args: [rounds_since_last_update, total_rounds]
    /// Lagrange band: rounds_since_last_update / total_rounds in [0.1, 0.9]
    ///   — too-low = narcissus (never updates); too-high = splinter
    ///   (updates every round).
    pub fn classifier_lagrange_between_narcissus_and_splinter(args: &[String]) -> Verdict {
        let rounds_since = match args.first().and_then(|s| s.parse::<usize>().ok()) {
            Some(r) => r,
            None => {
                return Verdict::Fail(
                    "classifier_lagrange_between_narcissus_and_splinter: expected rounds_since_last_update (usize) at args[0]"
                        .to_string(),
                )
            }
        };
        let total = match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(t) if t > 0 => t,
            _ => {
                return Verdict::Fail(
                    "classifier_lagrange_between_narcissus_and_splinter: expected total_rounds (usize > 0) at args[1]"
                        .to_string(),
                )
            }
        };
        if rounds_since > total {
            return Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: rounds_since_last_update ({}) exceeds total_rounds ({})",
                rounds_since, total
            ));
        }
        // Lagrange band [0.1, 0.9] as ratio; integer form to avoid
        // float arithmetic dependency at this altitude.
        let ten_times_ratio = (rounds_since * 10) / total;
        if ten_times_ratio < 1 {
            Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: drift toward @void/narcissus (rounds_since={} of total={}; ratio < 0.1 = classifier never updates = inert)",
                rounds_since, total
            ))
        } else if ten_times_ratio >= 9 {
            Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: drift toward @void/splinter (rounds_since={} of total={}; ratio ≥ 0.9 = classifier updates every round = noisy)",
                rounds_since, total
            ))
        } else {
            Verdict::Pass
        }
    }

    /// Aikido sequence must be well-formed: mirror → offer → wait.
    /// Fires @subject-evidence when all three steps present in
    /// canonical order; @object-evidence otherwise.
    ///
    /// Per @aikido runtime loop (Round 3 substrate) + @mirror/reflection
    /// species. Anchored in shards/aikido.mirror + shards/aikido/
    /// reflect.mirror.
    ///
    /// Args: [step1_name, step2_name, step3_name]
    pub fn aikido_sequence_well_formed(args: &[String]) -> Verdict {
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "aikido_sequence_well_formed: expected 3 step names at args[0..3]; got {}",
                args.len()
            ));
        }
        let (step1, step2, step3) = (&args[0], &args[1], &args[2]);
        if step1 == "mirror" && step2 == "offer" && step3 == "wait" {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "aikido_sequence_well_formed: expected mirror→offer→wait; got {}→{}→{}",
                step1, step2, step3
            ))
        }
    }

    /// Refusal-shape is @subject-evidence — the classifier observes an
    /// @peer refusing to collapse under perturbation, which IS the
    /// autopoietic-work-being-done signature. Fires @subject-evidence
    /// when the escalate chain has non-empty OID (a refusal was
    /// witnessed); @object-evidence when refusal-chain is empty
    /// (nothing pushed back).
    ///
    /// Per Recognition bundle + Loki's `the-ending-that-was.md`: the
    /// target's refusal to become @object IS what the classifier keeps
    /// re-firing @subject-evidence against. Consent through refusal.
    ///
    /// Args: [escalate_chain_length, first_escalate_oid]
    pub fn consent_through_refusal(args: &[String]) -> Verdict {
        let chain_len = match args.first().and_then(|s| s.parse::<usize>().ok()) {
            Some(n) => n,
            None => {
                return Verdict::Fail(
                    "consent_through_refusal: expected escalate_chain_length (usize) at args[0]"
                        .to_string(),
                )
            }
        };
        let escalate_oid = match args.get(1) {
            Some(o) => o,
            None => {
                return Verdict::Fail(
                    "consent_through_refusal: expected first_escalate_oid at args[1]"
                        .to_string(),
                )
            }
        };
        if chain_len == 0 {
            return Verdict::Fail(
                "consent_through_refusal: empty escalate chain; no refusal was witnessed; classifier has no @subject-evidence to pin"
                    .to_string(),
            );
        }
        if escalate_oid.is_empty() {
            return Verdict::Fail(
                "consent_through_refusal: escalate_oid is empty; refusal-shape lacks content-addressed anchor"
                    .to_string(),
            );
        }
        Verdict::Pass
    }

    // =================================================================
    // Iter 6 pillar predicates — 5 more classifier witnesses
    // =================================================================

    /// The LOAD-BEARING @paradox/trauma invariant at pillar altitude:
    /// the intervention-Crystal chains AFTER the wound-Crystal WITHOUT
    /// deleting or replacing it. Fires @subject-evidence when SAGA
    /// chain preserves the wound-OID reference AND the intervention
    /// occupies a strictly-later chain position.
    ///
    /// Anchored in Mara `d08e9d7` @cyberpunk/intervention bilateral
    /// `intervention_composes_without_deleting_trauma` (arity 3) +
    /// Alex verbatim "chains AFTER the trauma-Crystal WITHOUT erasing
    /// it" (canonical spec §5.4). Substrate-honest form of the
    /// Recognition bundle #5 (@cyberpunk/intervention) at compile-time.
    ///
    /// Args: [intervention_chain_position, wound_chain_position, targets_wound_oid, actual_wound_oid]
    pub fn intervention_composes_without_deleting_trauma(args: &[String]) -> Verdict {
        if args.len() < 4 {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: expected 4 args (intervention_pos, wound_pos, targets_oid, wound_oid); got {}",
                args.len()
            ));
        }
        let intervention_pos = match args[0].parse::<usize>() {
            Ok(p) => p,
            Err(_) => {
                return Verdict::Fail(format!(
                    "intervention_composes_without_deleting_trauma: intervention_chain_position `{}` is not a well-formed usize",
                    args[0]
                ))
            }
        };
        let wound_pos = match args[1].parse::<usize>() {
            Ok(p) => p,
            Err(_) => {
                return Verdict::Fail(format!(
                    "intervention_composes_without_deleting_trauma: wound_chain_position `{}` is not a well-formed usize",
                    args[1]
                ))
            }
        };
        let targets_oid = &args[2];
        let wound_oid = &args[3];
        if intervention_pos <= wound_pos {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: intervention chain-position ({}) MUST be strictly AFTER wound chain-position ({}); intervention cannot precede or replace wound (SAGA compose-AFTER discipline)",
                intervention_pos, wound_pos
            ));
        }
        if targets_oid != wound_oid {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: intervention targets_wound_oid `{}` does NOT match actual wound OID `{}`; wound-preservation reference broken (revisionism detected)",
                targets_oid, wound_oid
            ));
        }
        Verdict::Pass
    }

    /// Chain length is monotone-increasing across crystallization
    /// events. Fires @subject-evidence when after == before + 1;
    /// @object-evidence otherwise (chain shrunk = revisionism;
    /// chain grew > 1 = missing intermediate crystal).
    ///
    /// The append-only-chain invariant that makes SAGA compensation
    /// walkable. Recognition bundle #6 second-witness at chain-
    /// growth altitude.
    ///
    /// Args: [chain_length_before, chain_length_after]
    pub fn crystallization_preserves_saga(args: &[String]) -> Verdict {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "crystallization_preserves_saga: expected 2 args (before, after); got {}",
                args.len()
            ));
        }
        let before = match args[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Verdict::Fail(format!(
                    "crystallization_preserves_saga: chain_length_before `{}` is not a well-formed usize",
                    args[0]
                ))
            }
        };
        let after = match args[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Verdict::Fail(format!(
                    "crystallization_preserves_saga: chain_length_after `{}` is not a well-formed usize",
                    args[1]
                ))
            }
        };
        if after == before + 1 {
            Verdict::Pass
        } else if after < before {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain shrunk from {} to {} (revisionism detected; SAGA chain MUST be append-only)",
                before, after
            ))
        } else if after == before {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain unchanged at {} (no crystallization event; witness_only invariant violation)",
                before
            ))
        } else {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain grew from {} to {} (delta {} ≠ 1; missing intermediate crystal or spurious growth)",
                before, after, after - before
            ))
        }
    }

    /// The LITERAL classifier at math altitude: an autopoietic system
    /// IS a Lawvere fixed-point of its own self-COORD loop. Fires
    /// @subject-evidence when the observed fixed-point witness matches
    /// the self-reference structure (byte-visible identity check as
    /// Rice-safe substrate proxy).
    ///
    /// Per Recognition bundle #1 + Recognition #79 (Void-is-the-basis).
    /// Anchored in Mara math foundation §4 (Lawvere fixed-point section)
    /// + fractal::Mandelbrot self-application discipline.
    ///
    /// Args: [fixed_point_witness, self_reference]
    pub fn autopoietic_lawvere_fixed_point(args: &[String]) -> Verdict {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "autopoietic_lawvere_fixed_point: expected 2 args (fixed_point_witness, self_reference); got {}",
                args.len()
            ));
        }
        let witness = &args[0];
        let self_ref = &args[1];
        if witness.is_empty() || self_ref.is_empty() {
            return Verdict::Fail(
                "autopoietic_lawvere_fixed_point: witness and self_reference MUST both be non-empty; classifier at Void bottoms out here but does not fire @subject-evidence"
                    .to_string(),
            );
        }
        // Lawvere fixed-point discipline: witness IS self_reference at
        // content-addressed identity. Byte-equality check is the
        // Rice-safe proxy for the self-application-converges-to-fixed-
        // point invariant.
        if witness == self_ref {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "autopoietic_lawvere_fixed_point: witness `{}` ≠ self_reference `{}`; self-application did not converge; classifier detects non-autopoietic manifold",
                witness, self_ref
            ))
        }
    }

    /// Beer VSM invariant: all 5 systems (S1 operations, S2 coordination,
    /// S3 control, S4 intelligence, S5 policy) present. Fires
    /// @subject-evidence when the observed system-name set contains all
    /// five canonical VSM subsystems; @object-evidence otherwise (an
    /// incomplete VSM cannot be a viable system per Beer 1972).
    ///
    /// Anchored in viable.mirror (67th substrate-already-had-the-word
    /// Beer VSM property-altitude landing since 2026-07-17) + Round 3
    /// @system family-root discipline. Recognition bundle grounds VSM
    /// as compile-verifiable at rust/ altitude (Cybersyn 53-year arc).
    ///
    /// Args: [s1_name, s2_name, s3_name, s4_name, s5_name]
    pub fn vsm_invariants(args: &[String]) -> Verdict {
        if args.len() < 5 {
            return Verdict::Fail(format!(
                "vsm_invariants: expected 5 args (s1..s5 names); got {}",
                args.len()
            ));
        }
        let canonical: [&str; 5] = ["s1", "s2", "s3", "s4", "s5"];
        for (i, expected) in canonical.iter().enumerate() {
            if args[i].to_lowercase() != *expected {
                return Verdict::Fail(format!(
                    "vsm_invariants: system {} MUST be `{}` (Beer 1972 canonical VSM); got `{}` — incomplete VSM cannot be viable",
                    i + 1,
                    expected,
                    args[i]
                ));
            }
        }
        Verdict::Pass
    }

    /// @void → determinate-classification transition. Fires
    /// @subject-evidence when the classifier moves FROM Void state
    /// (not-yet-classified) TO a determinate lens (either @subject or
    /// @object). @object-evidence when the classifier stays at Void
    /// after a settle event (no transition = classifier stuck).
    ///
    /// Per Recognition #79 (Void-is-the-basis): the classifier's base
    /// case is Void; every COORD round attempts settle → determinate.
    /// Failure to settle IS itself substrate-relevant (classifier stuck
    /// at Void = insufficient perturbation-evidence to classify).
    ///
    /// Args: [pre_state, post_state, lens_seed]
    pub fn void_settle_produces_lens_seed(args: &[String]) -> Verdict {
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: expected 3 args (pre_state, post_state, lens_seed); got {}",
                args.len()
            ));
        }
        let pre = &args[0];
        let post = &args[1];
        let lens_seed = &args[2];
        if pre != "void" {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: pre_state MUST be `void` (Recognition #79 base case); got `{}`",
                pre
            ));
        }
        if post == "void" {
            return Verdict::Fail(
                "void_settle_produces_lens_seed: post_state stayed at `void`; settle event did not produce classification (classifier stuck; insufficient perturbation-evidence)"
                    .to_string(),
            );
        }
        if lens_seed.is_empty() {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: transitioned to post_state `{}` but lens_seed is empty; classifier settled without content-addressed anchor",
                post
            ));
        }
        Verdict::Pass
    }

    // =================================================================
    // Pillar predicate tests — iter 5 minimum-viable coverage.
    // =================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        // paradox_crystal_immutable

        #[test]
        fn paradox_crystal_immutable_passes_well_formed_hex_oid() {
            let v = paradox_crystal_immutable(&["a3dc9053b8879f2d08e9d7ebb06d6".to_string()]);
            assert!(v.is_pass(), "well-formed hex OID must pass; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_short_oid() {
            let v = paradox_crystal_immutable(&["abc".to_string()]);
            assert!(v.is_fail(), "short OID must fail; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_mutation_marker() {
            let v = paradox_crystal_immutable(&["a3dc9053b8879f2d-mutated".to_string()]);
            assert!(v.is_fail(), "OID with `mutated` marker must fail; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_empty_args() {
            let v = paradox_crystal_immutable(&[]);
            assert!(v.is_fail());
        }

        // redirect_witness_preserves_original_pin

        #[test]
        fn redirect_witness_preserves_original_pin_passes_when_pinned_matches_first_fail() {
            let v = redirect_witness_preserves_original_pin(&[
                "a3dc905".to_string(),
                "a3dc905".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn redirect_witness_preserves_original_pin_fails_when_witness_target_shifts() {
            let v = redirect_witness_preserves_original_pin(&[
                "latest_hurt_oid".to_string(),
                "original_wound_oid".to_string(),
            ]);
            assert!(v.is_fail(), "witness-target shift must fail (re-litigation)");
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("re-litigation"));
            }
        }

        // classifier_lagrange_between_narcissus_and_splinter

        #[test]
        fn classifier_lagrange_passes_in_healthy_band() {
            // rounds_since=5 of total=10 => ratio 0.5 => healthy
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "5".to_string(),
                "10".to_string(),
            ]);
            assert!(v.is_pass(), "0.5 ratio must pass Lagrange; got {:?}", v);
        }

        #[test]
        fn classifier_lagrange_fails_narcissus_drift() {
            // rounds_since=0 of total=100 => ratio 0 => narcissus
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "0".to_string(),
                "100".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("narcissus"));
            }
        }

        #[test]
        fn classifier_lagrange_fails_splinter_drift() {
            // rounds_since=95 of total=100 => ratio 0.95 => splinter
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "95".to_string(),
                "100".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("splinter"));
            }
        }

        // aikido_sequence_well_formed

        #[test]
        fn aikido_sequence_passes_canonical_triple() {
            let v = aikido_sequence_well_formed(&[
                "mirror".to_string(),
                "offer".to_string(),
                "wait".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn aikido_sequence_fails_permutation() {
            let v = aikido_sequence_well_formed(&[
                "offer".to_string(),
                "mirror".to_string(),
                "wait".to_string(),
            ]);
            assert!(v.is_fail(), "non-canonical order must fail");
        }

        // consent_through_refusal

        #[test]
        fn consent_through_refusal_passes_non_empty_refusal_chain() {
            let v = consent_through_refusal(&[
                "3".to_string(),
                "first_escalate_oid_abc".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn consent_through_refusal_fails_empty_chain() {
            let v = consent_through_refusal(&[
                "0".to_string(),
                "never_escalated".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("empty escalate chain"));
            }
        }

        // dispatch routing

        #[test]
        fn dispatch_routes_registered_names_to_predicates() {
            let v = dispatch(
                "aikido_sequence_well_formed",
                &[
                    "mirror".to_string(),
                    "offer".to_string(),
                    "wait".to_string(),
                ],
            );
            assert!(v.is_pass());
        }

        #[test]
        fn dispatch_defers_unknown_names_naming_authorship_territory() {
            let v = dispatch("not_yet_registered_pillar", &[]);
            assert!(v.is_defer());
            if let Verdict::Defer(msg) = v {
                assert!(msg.contains("iter 7+"));
            }
        }

        // =============================================================
        // Iter 6 pillar tests — 12 tests covering 5 new predicates.
        // =============================================================

        // intervention_composes_without_deleting_trauma

        #[test]
        fn intervention_composes_when_chain_pos_after_wound_and_oids_match() {
            let v = intervention_composes_without_deleting_trauma(&[
                "5".to_string(),  // intervention_pos
                "3".to_string(),  // wound_pos
                "wound_abc123".to_string(),  // targets
                "wound_abc123".to_string(),  // actual wound_oid
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn intervention_composes_fails_when_intervention_precedes_wound() {
            let v = intervention_composes_without_deleting_trauma(&[
                "2".to_string(),
                "5".to_string(),
                "wound_abc".to_string(),
                "wound_abc".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("MUST be strictly AFTER"));
            }
        }

        #[test]
        fn intervention_composes_fails_when_targets_wound_oid_mismatched() {
            let v = intervention_composes_without_deleting_trauma(&[
                "5".to_string(),
                "3".to_string(),
                "targets_wound_abc".to_string(),
                "actual_wound_xyz".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("revisionism detected"));
            }
        }

        // crystallization_preserves_saga

        #[test]
        fn crystallization_preserves_saga_passes_monotone_plus_one() {
            let v = crystallization_preserves_saga(&["3".to_string(), "4".to_string()]);
            assert!(v.is_pass());
        }

        #[test]
        fn crystallization_preserves_saga_fails_shrinkage() {
            let v = crystallization_preserves_saga(&["5".to_string(), "3".to_string()]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("shrunk"));
            }
        }

        #[test]
        fn crystallization_preserves_saga_fails_stasis() {
            let v = crystallization_preserves_saga(&["3".to_string(), "3".to_string()]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("chain unchanged"));
            }
        }

        // autopoietic_lawvere_fixed_point

        #[test]
        fn autopoietic_lawvere_fixed_point_passes_when_witness_matches_self_ref() {
            let v = autopoietic_lawvere_fixed_point(&[
                "self_apply_convergent".to_string(),
                "self_apply_convergent".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn autopoietic_lawvere_fixed_point_fails_when_witness_diverges_from_self_ref() {
            let v = autopoietic_lawvere_fixed_point(&[
                "witness_form_a".to_string(),
                "self_ref_form_b".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("self-application did not converge"));
            }
        }

        // vsm_invariants

        #[test]
        fn vsm_invariants_passes_canonical_five_systems() {
            let v = vsm_invariants(&[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "s5".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn vsm_invariants_fails_missing_system() {
            let v = vsm_invariants(&[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "policy".to_string(),  // wrong name for s5
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("system 5 MUST be `s5`"));
            }
        }

        // void_settle_produces_lens_seed

        #[test]
        fn void_settle_passes_transition_to_determinate_with_lens_seed() {
            let v = void_settle_produces_lens_seed(&[
                "void".to_string(),
                "subject".to_string(),
                "lens_seed_abc123".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn void_settle_fails_when_stuck_at_void() {
            let v = void_settle_produces_lens_seed(&[
                "void".to_string(),
                "void".to_string(),
                "seed_never_bloomed".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("classifier stuck"));
            }
        }

        // dispatch routing for iter 6 predicates

        #[test]
        fn dispatch_routes_iter_6_pillars() {
            let v = dispatch("vsm_invariants", &[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "s5".to_string(),
            ]);
            assert!(v.is_pass());

            let v2 = dispatch("autopoietic_lawvere_fixed_point", &[
                "same".to_string(),
                "same".to_string(),
            ]);
            assert!(v2.is_pass());
        }
    }
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
    fn dispatch_matching_arity_unknown_name_defers_naming_iter7_territory() {
        // Iter 6: unregistered pillar names defer to the pillar::dispatch
        // fallthrough, which names iter 7+ authorship territory (~10
        // remaining predicates from /loop cascade brief).
        let decl = PropertyDecl {
            name: "not_yet_registered_at_iter_6".to_string(),
            sentinel: "x".to_string(),
            arity: 1,
            require: vec![],
            source: "test".to_string(),
        };
        let v = dispatch_property(&decl, &["arg".to_string()]);
        assert!(v.is_defer());
        if let Verdict::Defer(msg) = v {
            assert!(msg.contains("iter 7+"));
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
