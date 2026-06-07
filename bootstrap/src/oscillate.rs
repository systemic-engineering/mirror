//! `@mirror/spectral/oscillate` — boundary Rust for the kintsugi-loop
//! termination check.
//!
//! This module realizes a substrate-declared action from
//! `shards/mirror/spectral/oscillate.mirror` as Rust body. The substrate
//! names the full kintsugi loop primitive (the five-state
//! `oscillation_state`, the `pulse` atomic step, the `active_pass` /
//! `dark_pass` two-pass alternation, the `oscillate` driver, the
//! `read_consent` four-to-five bridge, the `is_complete` termination
//! check); T4 lands only the **termination check** — `is_complete` —
//! and the carrier it reads ([`OscillationState`]). The rest of the
//! oscillate primitives are forward-promises to ticks that compose the
//! driver (consent.morphism + consent.query_phi + uuid_spectral.dark
//! must substrate-pull into Rust first; T4 sits at the discriminator
//! floor only).
//!
//! ## What lives here
//!
//! - [`OscillationState`] — the Rust mirror of
//!   `type oscillation_state = active | dark | settled | escalated | waiting`
//!   from `shards/mirror/spectral/oscillate.mirror`.
//! - [`is_complete`] — the executable form of
//!   `is_complete(s: oscillation_state) -> verdict`. Reads an
//!   `OscillationState` and emits the substrate's three-state verdict
//!   per `oscillate.mirror`'s declared mapping table (§is_complete):
//!
//!   | oscillation_state | verdict                                              | confidence | meaning                                       |
//!   |-------------------|------------------------------------------------------|------------|-----------------------------------------------|
//!   | settled           | `Success(())`                                        | 1.0        | autopoietic ground state; holonomy → 0        |
//!   | active            | `Partial((), Transparency::opaque(…, 0.5))`          | 0.5        | mid-cycle; ACTIVE pass live; proposing        |
//!   | dark              | `Partial((), Transparency::opaque(…, 0.5))`          | 0.5        | mid-cycle; DARK pass live; anchoring identity |
//!   | waiting           | `Partial((), Transparency::opaque(…, 0.25))`         | 0.25       | half-cadence; awaiting next consent tick      |
//!   | escalated         | `Failure(Gap{level:1, "escalation surfaced"}, …)`    | 0.0        | external resolution required; pause emitted   |
//!
//! ## The active/dark mapping call (T4 design call)
//!
//! Per `oscillate.mirror`'s `is_complete` declared mapping table
//! (lines 631–649): `active → partial(confidence)` and
//! `dark → partial(confidence)` — the substrate names BOTH as the same
//! `partial` shape and reads them as "driver continues with next
//! pulse". The substrate's own framing IS the answer to the brief's
//! design question: active and dark are not "stop here" (the substrate
//! says `partial`, not `failure`) and not "settled with caveat" (the
//! substrate says `partial`, not `pass`); they are the loop's two live
//! phases, and `partial(confidence)` IS the substrate's honest reading
//! of mid-cycle.
//!
//! The body chooses `confidence = 0.5` for both (the band midpoint).
//! Rationale: `Waiting` already sits at 0.25 (below 1/φ ≈ 0.618 — the
//! "closer to failure" Half-cadence band per `music/mod.rs`); active /
//! dark sit at 0.5 (still below 1/φ; mid-cycle is not yet consonant,
//! but it's structurally distinct from waiting — waiting is paused at
//! the dominant, active/dark is in-flight). Through
//! [`crate::gap::verdict_to_cadence_kind`] both 0.5 and 0.25 read as
//! `CadenceKind::Half` (paused-on-V shape); the substrate's
//! mid-progression IS a half-cadence read at the audible altitude,
//! consonant with `oscillate.mirror`'s own framing
//! (`read_consent.partial+half → waiting`).
//!
//! `escalated → failure(reason)` uses `Gap { level: 1, … }` per
//! [`docs/specs/gap-tension-tensor-substrate.md`] §3.2: an escalation
//! surfacing is a level-1 contradiction (simple unresolved tension; no
//! nested learning loop yet — the nested loop happens when Reflection
//! reads the escalation and responds). Higher levels lift when T5
//! (`gaps_of`) substrate-pulls richer `Gap` shapes.
//!
//! [`docs/specs/gap-tension-tensor-substrate.md`]: ../../../docs/specs/gap-tension-tensor-substrate.md
//! [`Gap`]: crate::gap::Gap
//!
//! ## The implementation cascade template
//!
//! This module is the third discriminator-floor tick in the
//! implementation cascade (after `is_settled` in T3 and `is_consonant`
//! / `is_pareto` in T4's sibling `music` body). It follows the same
//! template T3 established: substrate declares the action signature;
//! bootstrap realises the body in this module under
//! `[substrate-pull:realize]`. The rest of `oscillate.mirror`'s
//! primitives (`pulse`, `active_pass`, `dark_pass`, `oscillate`,
//! `read_consent`) land as sibling functions here when their upstream
//! consent / uuid_spectral dependencies substrate-pull.
//!
//! `#[allow(dead_code)]` at the module level: this module's surface is
//! the forward-promise to `@mirror/spectral/oscillate`'s consumer (the
//! future `mirror kintsugi` driver), which is not yet wired in
//! `main.rs`. The tests below DO use the surface; they prove the
//! contract regardless of the consumer's arrival.
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    //! The executable spec for [`is_complete`] — T4's `oscillate`
    //! body. RED first; the body lands GREEN in the next commit.
    //!
    //! Per `shards/mirror/spectral/oscillate.mirror`'s `is_complete`
    //! declared mapping (§is_complete, lines 627–669): the loop's
    //! position projects onto the substrate's three-state verdict
    //! surface. The five-state oscillation_state collapses onto the
    //! three-state verdict cleanly:
    //!
    //!   - `settled`             → `Success(())`              (pass)
    //!   - `active`              → `Partial((), opaque)`      (partial; mid-cycle ACTIVE)
    //!   - `dark`                → `Partial((), opaque)`      (partial; mid-cycle DARK)
    //!   - `waiting`             → `Partial((), opaque)`      (partial; half-cadence)
    //!   - `escalated`           → `Failure(Gap, opaque)`     (failure)
    //!
    //! Per AGENTS.md "TDD": these tests RED first; the body lands
    //! GREEN in the next commit. Drift on either side breaks them.

    use super::*;
    use crate::gap::{confidence_of, Gap};
    use prism_core::Transparency;
    use terni::Imperfect;

    /// `settled` is the autopoietic ground state — `Success(())`.
    /// Per `oscillate.mirror` §is_complete: "Loop completed cleanly;
    /// holonomy → 0; final ref emitted; driver returns."
    #[test]
    fn settled_is_success_unit() {
        let v = is_complete(OscillationState::Settled);
        assert!(matches!(v, Imperfect::Success(())));
        assert!(v.is_ok());
        assert!(!v.is_partial());
        assert!(!v.is_err());
    }

    /// `active` is mid-cycle, ACTIVE pass live — `Partial` at confidence 0.5.
    /// Per `oscillate.mirror` §is_complete: "Loop mid-cycle in ACTIVE phase;
    /// driver continues with next pulse."
    #[test]
    fn active_is_partial_with_mid_confidence() {
        let v = is_complete(OscillationState::Active);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "active transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.5).abs() < 1e-9,
                    "active confidence must be 0.5 (got {c})",
                );
            }
            other => panic!("active must yield Partial; got {other:?}"),
        }
        assert!(v.is_ok());
        assert!(v.is_partial());
        assert!(!v.is_err());
    }

    /// `dark` is mid-cycle, DARK pass live — `Partial` at confidence 0.5.
    /// Per `oscillate.mirror` §is_complete: "Loop mid-cycle in DARK phase;
    /// driver continues with next pulse." Same confidence as `active`:
    /// the substrate's distinction is which phase is live, not the
    /// loop's completeness.
    #[test]
    fn dark_is_partial_with_mid_confidence() {
        let v = is_complete(OscillationState::Dark);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "dark transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.5).abs() < 1e-9,
                    "dark confidence must be 0.5 (got {c})",
                );
            }
            other => panic!("dark must yield Partial; got {other:?}"),
        }
    }

    /// `waiting` is half-cadence; substrate paused at the dominant —
    /// `Partial` at confidence 0.25. Per `oscillate.mirror`
    /// §is_complete: "Loop paused mid-progression at half-cadence; will
    /// resume on next consent surface tick." The 0.25 mirrors
    /// `music/mod.rs`'s Half-cadence confidence exactly — the audible
    /// altitude and the oscillation altitude both read half-cadence as
    /// 0.25 (below 1/φ ≈ 0.618 — closer-to-failure-than-not).
    #[test]
    fn waiting_is_partial_with_half_cadence_confidence() {
        let v = is_complete(OscillationState::Waiting);
        match &v {
            Imperfect::Partial((), t) => {
                assert!(
                    !t.is_catastrophic(),
                    "waiting transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    (c - 0.25).abs() < 1e-9,
                    "waiting confidence must be 0.25 (got {c})",
                );
            }
            other => panic!("waiting must yield Partial; got {other:?}"),
        }
        assert!(v.is_partial());
    }

    /// `escalated` is failure — pause_event already emitted to metalogue.
    /// Per `oscillate.mirror` §is_complete: "Loop halted pending external
    /// resolution." Per `gap-tension-tensor-substrate.md` §3.2: escalation
    /// is a level-1 contradiction (simple unresolved tension; no nested
    /// learning loop).
    #[test]
    fn escalated_is_failure_with_level_one_gap() {
        let v = is_complete(OscillationState::Escalated);
        match &v {
            Imperfect::Failure(gap, t) => {
                assert_eq!(
                    Gap::level(gap),
                    1,
                    "escalation is a level-1 contradiction"
                );
                assert!(
                    !t.is_catastrophic(),
                    "escalated transparency must not be catastrophic"
                );
                let c = confidence_of(t);
                assert!(
                    c < 0.25,
                    "escalated confidence must be near zero (got {c})",
                );
            }
            other => panic!("escalated must yield Failure; got {other:?}"),
        }
        assert!(v.is_err());
        assert!(!v.is_ok());
    }

    /// Type-level: the substrate's three-state floor IS reachable as
    /// `crate::music::Verdict` (the same Hodge-framed triple T3 landed).
    /// Per `oscillate.mirror`'s §is_complete: "the verdict mapping IS the
    /// loop driver's termination discipline" — same verdict shape as
    /// `is_settled`, same altitude.
    #[test]
    fn is_complete_returns_music_verdict_shape() {
        let v: crate::music::Verdict = is_complete(OscillationState::Settled);
        // The type assertion is the test; the value just witnesses.
        assert!(matches!(v, Imperfect::Success(())));
    }
}
