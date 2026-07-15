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
//! RED state (Arc-1 Tick 1.2): all 7 combinators are `todo!()`. Tick 1.3
//! GREEN fills the bodies by composing the already-landed primitives in
//! `bootstrap/src/spectral.rs` (`Combinator`, `Fold5`, `compose_a`,
//! `apply_h`, `eigen_d`) with substrate-ref resolution and shard-body
//! evaluation. Tick 1.4 wires `mirror beam act` as the CLI verb.
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
pub fn section(source_handle: Ref) -> Section {
    let _ = source_handle;
    todo!("Arc-1 Tick 1.3: compose @io.file.read_bytes + Combinator::apply")
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
pub fn fold(section: Section, reducers: Fold5Reducers, initial: Value) -> Value {
    let _ = (section, reducers, initial);
    todo!("Arc-1 Tick 1.3: dispatch to spectral::Fold5::apply")
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
/// this call. Panics with `not yet implemented` in RED phase; Tick 1.3
/// fills the resolver + recursive composition.
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
pub fn act(action: Ref, args: Vec<Value>) -> Verdict {
    let _ = (action, args);
    todo!("Arc-1 Tick 1.3: resolve shard_action_ref + recurse via apply_h")
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
pub fn settle(verdict: Transparency, tolerance: f64) -> SettledVerdict {
    let _ = (verdict, tolerance);
    todo!("Arc-1 Tick 1.3: run P-Ł descent via apply_h with δ* adjoint")
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
pub fn crystallize(before: OuroborosState, after: OuroborosState) -> BenchCrystal {
    let _ = (before, after);
    todo!("Arc-1 Tick 1.3: compute content OIDs + @mirror/store.write_crystal")
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
pub fn coboundary(section: Section, target: Ref) -> Transparency {
    let _ = (section, target);
    todo!("Arc-1 Tick 1.3: compose apply_h_content + Combinator dispatch + Transport")
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
pub fn utter(channel: Ref, event: SubstrateEvent) -> Verdict {
    let _ = (channel, event);
    todo!("Arc-1 Tick 1.3: resolve channel + append + hash_tagged + subscribers")
}
