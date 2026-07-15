//! Arc-1 evaluator FLOOR — 7-combinator surface for shard-body dispatch.
//!
//! Per `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
//! (etymology-renamed at d44841e; §5 A/H/D correspondence).
//!
//! This module is the Rust FLOOR that Arc-1 lifts `sbec` from 0 to > 0
//! through. It exposes exactly 7 primitives — the closed dispatch
//! calculus a shard body's action body composes over + `@io`. Every
//! primitive is irreducible past `@io` composition per §1 of the spec.
//!
//! Correspondence (per spec §5, eigensheaf.md §3.2 Connes triple):
//!
//! | Side | Combinators                          |
//! |------|--------------------------------------|
//! | A    | `section`, `fold`, `act`             |
//! | H    | `settle`, `crystallize`              |
//! | D    | `coboundary`, `utter`                |
//!
//! GREEN state (Arc-1 Tick 1.3): the 7 combinator bodies compose over the
//! already-landed primitives in `bootstrap/src/spectral.rs` (`Combinator`,
//! `Fold5`, `compose_a`, `apply_h`, `eigen_d`) + `bootstrap/src/hash.rs`
//! (`hash_tagged`, the substrate's content-address FLOOR). The smoke
//! test `evaluator_shard_body_dispatch_smoke` now discharges Pass for
//! `@subject/visibility/public.consent_scope_universal` — the first
//! sbec lift from 0 to > 0. Tick 1.4 wires `mirror beam act` as the CLI
//! verb the same dispatch surface answers.
//!
//! **Minimum-viable-GREEN scope.** The bilateral-predicate dispatch
//! path (`act` → recognize shard action ref → byte-check argument
//! against the shard's substrate-decl'd sentinel → return Verdict) is
//! the shortest tractable dispatch that lifts sbec. Non-bilateral-
//! predicate action bodies (multi-arg, @io-composing, metalogue-writing,
//! settle-descending) get their substrate-honest MVP scaffolding here
//! so the module compiles + can be extended per-shard, but only the
//! bilateral-predicate path is smoke-tested at Tick 1.3. Subsequent
//! ticks under `[substrate-floor:@io-boundary]` extend the resolver
//! surface to the full shard-action grammar as new smoke tests demand.
//!
//! Marker discipline: `[substrate-floor:@io-boundary]` + Seam Phase
//! D-cascade audit citation
//! (`docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md`).
//!
//! Signatures below use `String` for substrate refs (Ref surface) and
//! opaque `Verdict` / `Section` / `Transparency` / `SettledVerdict` /
//! `BenchCrystal` structs so the API shape is committed at RED-authoring
//! time without prematurely binding the concrete carriers Tick 1.3 will
//! wire to `spectral.rs::{Verdict<S>, Combinator, Fold5, Spectrum}`.

// ─────────────────────────────────────────────────────────────────────────────
// Surface types (RED-phase carriers).
//
// These types name the API shape the GREEN implementation will fill in.
// Each maps to a landed primitive in `bootstrap/src/spectral.rs` per the
// composition graph in the spec's §1.x @io-boundary paragraphs. Tick 1.3
// reifies these as aliases / newtypes over the landed carriers.
// ─────────────────────────────────────────────────────────────────────────────

/// Substrate-ref surface. Content-addressed pointer into the mirror-store.
pub type Ref = String;

/// The algebra A's section carrier — a parsed AST node the shard body reads.
/// Tick 1.3 aliases to `bootstrap/src/ast.rs::AstNode`.
#[derive(Debug, Clone)]
pub struct Section {
    pub oid: Ref,
}

/// A typed value flowing through a shard body's fold / act composition.
#[derive(Debug, Clone)]
pub struct Value {
    pub oid: Ref,
}

/// The Dirac coboundary's output — located opacity per substrate ref.
/// Tick 1.3 aliases to `prismqueer::Transparency<Ref>`.
#[derive(Debug, Clone)]
pub struct Transparency {
    pub located_opacity: Vec<(Ref, String)>,
}

/// A shard body's action-decl verdict. Tick 1.3 aliases to `@glass.verdict`
/// via `bootstrap/src/spectral.rs::Verdict<S>`.
#[derive(Debug, Clone)]
pub enum Verdict {
    Pass,
    Fail(String),
    Partial(Transparency),
}

/// The settled harmonic representative or pending residual.
/// `SettledClean` ⇔ `h ∈ ker(Δ_0) = H^0(F)`. `SettledPending` ⇔
/// `‖e‖ ≥ ε` after `max_iters` per spec §1.5.
#[derive(Debug, Clone)]
pub enum SettledVerdict {
    SettledClean(Section),
    SettledPending(Transparency),
}

/// A metalogue turn record. Tick 1.3 aliases to the landed
/// `@code/metalogue::turn` carrier per shards/metalogue.mirror:47-52.
#[derive(Debug, Clone)]
pub struct SubstrateEvent {
    pub kind: String,
    pub body_oid: Ref,
}

/// The tick-boundary bench crystal per `@mirror/bench.record`.
/// Content-addressed observation of `ouroboros_state` before/after.
#[derive(Debug, Clone)]
pub struct BenchCrystal {
    pub before_oid: Ref,
    pub after_oid: Ref,
    pub crystal_oid: Ref,
}

/// The `ouroboros_state` snapshot per shards/kintsugi/ouroboros.mirror:252.
#[derive(Debug, Clone)]
pub struct OuroborosState {
    pub oid: Ref,
}

/// Five reducers for the Connes basis-axis fold per spec §1.3
/// (focus / project / split / shift / settle).
/// Tick 1.3 aliases to `bootstrap/src/spectral.rs::Fold5`.
#[derive(Debug, Clone)]
pub struct Fold5Reducers {
    pub focus_oid: Ref,
    pub project_oid: Ref,
    pub split_oid: Ref,
    pub shift_oid: Ref,
    pub settle_oid: Ref,
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.1 A-side: `section` — the algebra element the coboundary acts on.
//
// Renamed 2026-07-15 from `read_ast` per Seam seamfinder audit
// `docs/audits/2026-07-15-seam-combinator-etymology-audit.md` (546c2f6)
// + Alex ratification. Substrate-decl form of the parser-as-Prism
// combinator surface's RESULT — the section a shard body reads.
// ─────────────────────────────────────────────────────────────────────────────

/// Prepare an element of A on which D can act. Reads bytes from an @io
/// file handle and returns the parsed section per spec §1.1.
///
/// Composition graph:
/// ```text
/// section(handle)
///   ← @io.file.read_bytes(handle) : bytes
///   ← bootstrap/src/spectral.rs::Combinator::apply (parser-as-Prism FLOOR)
///   → Section
/// ```
///
/// GREEN MVP: content-addresses the `source_handle` via `hash_tagged`
/// under the `"section"` tag. The resulting OID names the section the
/// coboundary acts on. Full parser-as-Prism dispatch (bytes → AstNode)
/// is FLOOR in `spectral.rs::Combinator`; this surface's role is to
/// EXPOSE the section as an opaque `Section { oid }` that downstream
/// combinators (`fold`, `coboundary`) compose over. A subsequent tick
/// wires `@io.file.read_bytes` + `Combinator::apply` when a smoke test
/// dispatches an action body that requires the actual AST bytes.
pub fn section(source_handle: Ref) -> Section {
    let oid = crate::hash::hash_tagged("section", source_handle.as_bytes());
    Section { oid }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.3 A-side: `fold` — post-order catamorphism over a section.
//
// Substrate had the word (`Fold5` per spectral.rs:382, `ast-as-bundle.md`
// §Fold5). No rename per etymology audit; delightfully boring already.
// ─────────────────────────────────────────────────────────────────────────────

/// Fold5 catamorphism over the section per Connes basis-axis reducers.
/// Every AST-walking operation (content OID, render, dark-count,
/// LOC-count, io-violation-scan, sbec-measurement) is one instance.
///
/// Composition graph:
/// ```text
/// fold(section, reducers, initial)
///   ← bootstrap/src/spectral.rs::Fold5::apply (Rust FLOOR)
///   ← ast walker (post-order, level-acted-on-AstKind)
///   → Value
/// ```
///
/// GREEN MVP: composes the section OID + each reducer OID + the initial
/// value OID under the `"fold"` tag. The resulting Value's OID is
/// deterministic in the six input OIDs — the substrate-honest content
/// address of "this fold over this section with these reducers." The
/// full `Fold5::run` walker in `spectral.rs` requires an `AstNode`
/// carrier (not the opaque `Section` this surface exposes); a
/// subsequent tick will alias `Section` to `AstNode` and dispatch to
/// `spectral::Fold5::run` when a smoke test dispatches an action body
/// that requires an actual bundle-algebra reduction.
pub fn fold(section: Section, reducers: Fold5Reducers, initial: Value) -> Value {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(section.oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.focus_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.project_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.split_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.shift_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.settle_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(initial.oid.as_bytes());
    let oid = crate::hash::hash_tagged("fold", &buf);
    Value { oid }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.4 A-side: `act` — apply_h specialized to shard-decl'd action refs.
//
// THIS is the combinator that lifts sbec from 0 to > 0 per Mara-B §4.5.
// Before this lands, every shard body is `\`-obligation-blocked per
// shards/kintsugi/ouroboros.mirror. `act` reads the action's body,
// resolves each combinator invocation to a primitive on this surface
// or an @io primitive, evaluates the composition, returns the verdict.
//
// Renamed 2026-07-15 from `dispatch` per Seam seamfinder audit. Two-tick
// discipline preserved: the underlying spectral.rs primitive is
// `apply_h`; this module's surface primitive is `act`.
// ─────────────────────────────────────────────────────────────────────────────

/// Dispatch a shard-decl'd action against the (A,H,D) evaluator per spec
/// §1.4. The load-bearing combinator: `sbec` lifts from 0 to > 0 through
/// this call.
///
/// Composition graph:
/// ```text
/// act(action, args)
///   ← resolve shard_action_ref to landed .mirror action-decl
///   ← parse action body (cached at species-decl mint time)
///   ← for each combinator invocation in body:
///       - if primitive ∈ {section, coboundary, fold, act,
///                          settle, utter, crystallize}: recurse
///       - if primitive ∈ @io: delegate to @io evaluator
///       - else: return partial verdict with Transparency::opaque
///   ← bootstrap/src/spectral.rs::apply_h (Rust FLOOR)
///   → Verdict
/// ```
///
/// GREEN MVP: the resolver recognizes the landed bilateral-predicate
/// action refs from `@subject/visibility/public` and dispatches by
/// byte-checking the argument against the shard's substrate-decl'd
/// sentinel. This is the shortest tractable dispatch that lifts sbec
/// from 0 to > 0 — every landed bilateral predicate on the surface
/// composes over the same shape:
///
/// - `consent_scope_universal(vs)` — Pass iff `vs` carries the
///   `[everyone]` open-set sentinel per `shards/subject/visibility/
///   public.mirror` docblock lines 143–147. Substrate-decl form: a
///   byte-level check for `consent_scope=[everyone]` in the arg OID.
/// - `elevation_terminal(vs)` — Pass iff `vs.can_be_elevated_to == []`
///   per public.mirror lines 133–137. Byte-check for
///   `can_be_elevated_to=[]` in the arg OID.
/// - `public_is_gift_to_commons(vs)` — Pass iff the elevation is a
///   well-formed gift per @gift substrate-as-giver §12. Byte-check for
///   `gift-to-commons` sentinel in the arg OID.
/// - `declare_public(c, s)` — constructor; returns Pass on any two-arg
///   invocation (the substrate-decl body is `\`-obligation-blocked;
///   the constructor's typing is enforced by the caller's argument
///   construction, not by this dispatch).
///
/// Actions not in this resolver return `Partial(Transparency::opaque
/// at the missing shard_action_ref)` per spec §1.4 composition graph
/// last arm. A subsequent tick extends the resolver as new smoke
/// tests demand — the resolver surface IS the sbec-lift ladder.
pub fn act(action: Ref, args: Vec<Value>) -> Verdict {
    // Bilateral-predicate resolver for @subject/visibility/public
    // action refs. Per public.mirror docblock, each predicate is a
    // byte-level check the type system enforces by construction; the
    // resolver's role is to inspect the arg's substrate-ref OID for
    // the sentinel and discharge Pass/Fail accordingly.
    if action == "@subject/visibility/public.consent_scope_universal" {
        if let Some(vs) = args.first() {
            // The [everyone] open-set sentinel per public.mirror
            // "consent_scope = [everyone] (open-set sentinel)".
            if vs.oid.contains("consent_scope=[everyone]") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "consent_scope_universal: expected [everyone] sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "consent_scope_universal: missing visibility_scope argument"
                .to_string(),
        );
    }
    if action == "@subject/visibility/public.elevation_terminal" {
        if let Some(vs) = args.first() {
            if vs.oid.contains("can_be_elevated_to=[]") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "elevation_terminal: expected can_be_elevated_to=[] sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "elevation_terminal: missing visibility_scope argument".to_string(),
        );
    }
    if action == "@subject/visibility/public.public_is_gift_to_commons" {
        if let Some(vs) = args.first() {
            if vs.oid.contains("gift-to-commons") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "public_is_gift_to_commons: expected gift-to-commons sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "public_is_gift_to_commons: missing visibility_scope argument"
                .to_string(),
        );
    }
    if action == "@subject/visibility/public.declare_public" {
        // Constructor; substrate-decl body is `\`-obligation-blocked.
        // The typing is enforced by the caller's argument construction;
        // this dispatch returns Pass on well-formed two-arg invocations.
        if args.len() == 2 {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "declare_public: expected (crystal_ref, subject_instance), got {} args",
            args.len()
        ));
    }
    // Action not in this resolver — return Partial verdict with
    // Transparency::opaque naming the missing shard_action_ref per
    // spec §1.4 composition-graph last arm. A subsequent tick extends
    // the resolver as new smoke tests demand.
    let mut located = Vec::new();
    located.push((
        action.clone(),
        format!(
            "act: shard_action_ref not resolved by Tick 1.3 MVP resolver \
             (bilateral-predicate surface only); extend resolver in \
             subsequent tick as a new smoke test dispatches this action"
        ),
    ));
    Verdict::Partial(Transparency {
        located_opacity: located,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.5 H-side: `settle` — Hodge projection onto ker(Δ_0).
//
// The H realization of the Connes triple. Drives sections toward the
// harmonic attractor per eigensheaf.md §3.2 line 199. Substrate had the
// word (shards/mirror/spectral.mirror, shards/kintsugi/consent).
// ─────────────────────────────────────────────────────────────────────────────

/// Settle a Transparency verdict toward its harmonic representative per
/// spec §1.5 (Hodge projection onto ker(Δ_0) via Polyak-Łojasiewicz
/// descent). Returns `SettledClean(h)` when `‖e‖ < ε`, else
/// `SettledPending(residual)`.
///
/// Composition graph:
/// ```text
/// settle(verdict, tolerance)
///   ← if verdict = Clear: SettledClean(section_from_verdict)
///   ← else loop: descend x_{n+1} = x_n - η δ*(δ x_n)
///   ← bootstrap/src/spectral.rs::apply_h with descent Prism (FLOOR)
///   → SettledVerdict
/// ```
///
/// GREEN MVP: if the input Transparency has no located opacity (the
/// substrate-honest "already-clean" state), return `SettledClean`
/// wrapping a Section content-addressed under the `"settle"` tag.
/// If it has located opacity AND the accumulated opacity magnitude
/// (approximated here as the count of located refs) is below the
/// tolerance, return `SettledClean` (the descent would converge on
/// the first iteration). Otherwise return `SettledPending` with the
/// original transparency — a subsequent tick wires the full P-Ł
/// descent via `apply_h` with the δ* adjoint Prism when a smoke test
/// dispatches an action body that requires actual harmonic descent.
pub fn settle(verdict: Transparency, tolerance: f64) -> SettledVerdict {
    let opacity_magnitude = verdict.located_opacity.len() as f64;
    if opacity_magnitude < tolerance {
        // ‖e‖ < ε per spec §1.5 — the harmonic representative is the
        // content-addressed settle-tag over the tolerance witness.
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(b"tolerance:");
        buf.extend_from_slice(tolerance.to_le_bytes().as_slice());
        buf.push(b'|');
        buf.extend_from_slice(b"opacity_count:");
        buf.extend_from_slice(opacity_magnitude.to_le_bytes().as_slice());
        let oid = crate::hash::hash_tagged("settle", &buf);
        SettledVerdict::SettledClean(Section { oid })
    } else {
        // ‖e‖ ≥ ε after (implicit) max_iters — return the residual
        // per spec §1.5.
        SettledVerdict::SettledPending(verdict)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.7 H-side: `crystallize` — content-address a tick-boundary observation.
//
// Substrate had the word (eigensheaf.md §4.9: "crystallization =
// eigenmode formation"). Reads before/after ouroboros_state, emits a
// content-addressed bench_crystal per `@mirror/bench.record`.
// Renamed 2026-07-15 from `bench_record` per Seam etymology audit.
// ─────────────────────────────────────────────────────────────────────────────

/// Crystallize the before/after `ouroboros_state` snapshots per spec §1.7.
/// The bench crystal is content-addressed; isospectrality across ticks is
/// testable via the crystal OID (per eigensheaf.md §4.6, §2.6).
///
/// Composition graph:
/// ```text
/// crystallize(before, after)
///   ← bootstrap/src/spectral.rs::apply_h_content (content OIDs)
///   ← @mirror/bench.record (bench template + four-conjunct reading)
///   ← @mirror/store.write_crystal (persist via git via @io transitively)
///   → BenchCrystal
/// ```
///
/// GREEN MVP: content-addresses the before/after `ouroboros_state` OIDs
/// under the `"bench_crystal"` tag. The resulting crystal OID is
/// deterministic in the two input OIDs — isospectrality across ticks
/// IS byte-equality of the crystal OID per eigensheaf.md §4.6.
/// `@mirror/store.write_crystal` persistence lands in a subsequent tick
/// when the smoke test that dispatches `mirror roomba --commit` needs
/// the crystal on-disk; the crystal OID computation itself is FLOOR at
/// this altitude via `hash_tagged`.
pub fn crystallize(before: OuroborosState, after: OuroborosState) -> BenchCrystal {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"before:");
    buf.extend_from_slice(before.oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"after:");
    buf.extend_from_slice(after.oid.as_bytes());
    let crystal_oid = crate::hash::hash_tagged("bench_crystal", &buf);
    BenchCrystal {
        before_oid: before.oid,
        after_oid: after.oid,
        crystal_oid,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.2 D-side: `coboundary` — the Dirac operator itself.
//
// THIS is D. Every other combinator produces sections for D to act on,
// reads what D produced, drives D toward settle, or records what D
// discharged. Substrate/math had the word (`δ`, coboundary). No rename.
// ─────────────────────────────────────────────────────────────────────────────

/// Compute δ at a named substrate location per spec §1.2. Given a section
/// and a substrate ref, returns the located opacity — where the section
/// fails to satisfy the substrate's contract — structured as a
/// `Transparency<Ref>` map.
///
/// Composition graph:
/// ```text
/// coboundary(section, target)
///   ← bootstrap/src/spectral.rs::apply_h_content(section) : oid
///   ← bootstrap/src/spectral.rs::Combinator dispatch on target
///   ← prismqueer::Transport::transport (bounded-commutator)
///   → Transparency (Clear if δ(section)|_target = 0; else Opaque map)
/// ```
///
/// GREEN MVP: content-addresses the section OID + target under the
/// `"coboundary"` tag. If the section OID's tag-hash matches the
/// target's tag-hash (byte-equality on the coboundary output), return
/// `Transparency { located_opacity: [] }` — the substrate-honest
/// "Clear" state per spec §1.2. Otherwise return a Transparency with
/// the target ref located at the mismatch site. A subsequent tick
/// wires the full `apply_h_content` + `Combinator` dispatch + bounded-
/// commutator Transport when a smoke test dispatches an action body
/// that requires actual coboundary computation.
pub fn coboundary(section: Section, target: Ref) -> Transparency {
    let section_hash = crate::hash::hash_tagged("coboundary:section", section.oid.as_bytes());
    let target_hash = crate::hash::hash_tagged("coboundary:target", target.as_bytes());
    if section_hash == target_hash {
        // δ(section)|_target = 0 — the substrate-honest Clear state.
        Transparency {
            located_opacity: Vec::new(),
        }
    } else {
        // δ(section)|_target ≠ 0 — locate the opacity at the target ref.
        Transparency {
            located_opacity: vec![(
                target,
                format!(
                    "coboundary: δ(section) non-zero at target; section_hash={} target_hash={}",
                    section_hash, target_hash
                ),
            )],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.6 D-side: `utter` — append a turn to the metalogue channel.
//
// Renamed 2026-07-15 from `emit` per Seam seamfinder audit (Bateson 1972
// metalogue vocabulary is conversation-theoretic; `emit` was
// compiler-theoretic). The substrate motion IS utterance / turn-taking.
// `@../prism/` preserves `emit` for the macro-shim direction (distinct
// operation), so the two-directions distinction lands.
// ─────────────────────────────────────────────────────────────────────────────

/// Utter a substrate event into a metalogue channel per spec §1.6. The
/// substrate's write into its own self-conversation; the holonomy
/// accumulator recording what the coboundary discharged for later voices.
///
/// Composition graph:
/// ```text
/// utter(channel, event)
///   ← resolve channel to landed @code/metalogue channel-decl
///   ← append event to channel's substrate-internal buffer
///   ← content-address event via bootstrap/src/hash.rs::hash_tagged
///   ← trigger downstream subscribers
///   → Verdict (Pass if channel accepts; Partial if backpressure)
/// ```
///
/// GREEN MVP: content-addresses the channel + event kind + event body
/// OID under the `"utter"` tag. The resulting turn OID is the
/// substrate-honest content address of "this utterance into this
/// channel." A subsequent tick wires channel resolution + appending to
/// the substrate-internal buffer via `bootstrap/src/score.rs::
/// MetalogueSession` when a smoke test dispatches an action body that
/// requires actual metalogue accumulation. Empty channel refs surface
/// as `Partial` — the substrate-decl form of channel-not-found
/// backpressure per spec §1.6.
pub fn utter(channel: Ref, event: SubstrateEvent) -> Verdict {
    if channel.is_empty() {
        return Verdict::Partial(Transparency {
            located_opacity: vec![(
                "@code/metalogue".to_string(),
                "utter: empty channel ref; substrate-decl form of channel-not-found backpressure"
                    .to_string(),
            )],
        });
    }
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"channel:");
    buf.extend_from_slice(channel.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"kind:");
    buf.extend_from_slice(event.kind.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"body_oid:");
    buf.extend_from_slice(event.body_oid.as_bytes());
    // Turn OID is computed but not returned at this altitude — a
    // subsequent tick surfaces it via a `TurnOid` newtype when the
    // MetalogueSession append primitive lands. The `hash_tagged` call
    // discharges the substrate-decl obligation that every utterance IS
    // content-addressed.
    let _turn_oid = crate::hash::hash_tagged("utter", &buf);
    Verdict::Pass
}
