//! `@coherence` — @io-boundary FLOOR (post-ouroboros-bite).
//!
//! # Arc-2 Tick 2.2 — SECOND OUROBOROS BITE (2026-07-15)
//!
//! This file was authored 2026-07-14 by Reed as a substrate-dishonest
//! Rust extension per the (now-renamed) `[substrate-pull:realize]`
//! marker; see `docs/audits/2026-07-15-reed-substrate-dishonest-rust-
//! extensions-during-gift-arc.md`. Arc-2 Tick 2.2 lifts the substrate-
//! decl bilaterals into `shards/epistemologic/cybernetic/coherence.mirror`
//! per Mara-B canonical spec `docs/specs/kintsugi-ouroboros-compiler-
//! self-collapse.md`. The four bilateral predicates (`coherence_increases`,
//! `is_narcissus_pole`, `is_splinter_pole`, `coherence_witnessing`) now
//! dispatch via `apply_h::act`; sbec lifts by four. Pattern proven at
//! Tick 2.1 (f211ee48 — @spectral/signature) applied again: SECOND BITE
//! proves the collapse is REPEATABLE.
//!
//! What remains in this file is the @io-boundary numerical primitive the
//! shard-decl's action bodies compose over — Fiedler value read on
//! `EigenvalueProfile` (LAPACK dsyev via `@mirror/index`). Called by
//! `bootstrap/src/roomba.rs` (walker's coherence-gradient ascent) as the
//! @io-boundary primitive.
//!
//! First substrate-decl citation of Foerster's ethical imperative
//! ("Act so as always to increase the number of choices." — von Foerster
//! 1979 Paris address, reprinted *Understanding Understanding* Springer
//! 2003 Ch. 11 p. 227) discharged as computable scalar over the
//! substrate's own DAG structure.
//!
//! ## Substrate authority
//!
//! - Canonical shard-decl: `shards/epistemologic/cybernetic/coherence.mirror`
//!   (Mara `e0a3e48`).
//! - Ouroboros spec: `docs/specs/kintsugi-ouroboros-compiler-self-
//!   collapse.md` Arc-2 Tick 2.2.
//! - Audit: `docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
//!   cascade-a2-a6.md`.
//!
//! ## Substrate authority
//!
//! - Mara `e0a3e48` — `shards/epistemologic/cybernetic/coherence.mirror`
//!   species-shard (3-day forward-promise discharge closed).
//! - Mara `9f3f4f1` — `shards/epistemologic/cybernetic/coherence-
//!   parametric.mirror` (2026-07-01 parametric carrier reserving this
//!   species path).
//! - Alex Wolf 2026-07-14 in-transcript: "@coherence score is Förster's
//!   ethical imperative operationalized. Always act to increase the
//!   available number of choices in the system. We have the geometric
//!   state space. We have the 5D spectral coordinate system. We have
//!   Narcissus and Splinter. We have everything we need."
//! - Alex Wolf 2026-07-14 manifesto `Weird - Violence.md`:
//!   Narcissus = star graph (few available choices);
//!   Splinter = complete graph (many available choices).
//! - Taut `c805e5d` D11 — substrate-already-had-the-word confirmation
//!   of coherence carrier ancestry + reserved-path forward-promise
//!   closure.
//! - Recognition #99 (`mirror-spec-is-lambda-zero`) — λ₀ = Fiedler value
//!   at substrate altitude; higher Fiedler = more Splinter-like.
//!
//! ## The operational form
//!
//! `coherence_score(profile)` maps a graph's `EigenvalueProfile` to a
//! scalar in `[0, 1]`. Higher = more Splinter-like (more available
//! choices; more graph-connectivity; substrate closer to the pole where
//! removing any node preserves connectivity). Lower = more Narcissus-
//! like (fewer available choices; substrate closer to the pole where
//! removing the hub collapses connectivity).
//!
//! Substrate-honest choice (Mara C1 Alex-relay #1 numerical form): use
//! the Fiedler value directly as the coherence scalar. Justified because:
//! - λ₀ = 0 on disconnected graphs (fully Narcissus-collapsed);
//! - λ₀ increases with algebraic connectivity;
//! - λ₀ approaches 1 as graph approaches complete (Splinter pole);
//! - `index.rs::EigenvalueProfile::fiedler_value()` already normalized
//!   to `[0, 1]` per Reed Landing 3 (`c53a97c`).
//!
//! Alternative numerical forms (`-‖sc‖₂` linear-inverse; `1/(1+‖sc‖₂)`
//! bounded-inverse) are per-runtime-freedom per Mara's spec
//! `[[feedback-craft-not-deliver]]` discipline. This runtime picks
//! Fiedler because it's the substrate-honest carrier already landed.
//!
//! ## Foerster admissibility
//!
//! `coherence_increases(before, after)` discharges Foerster's ethical
//! imperative as a bilateral predicate: an action is Foerster-admissible
//! iff it does not decrease coherence. Loop-composition with @kintsugi,
//! @roomba, @knife: every accepted transition either preserves or
//! increases available choices; retreats are rejected at substrate
//! altitude.

use crate::index::{ConceptGraph, EigenvalueProfile, GraphEdge};

/// `@coherence.coherence_score` — monotone scalar on graph profile.
///
/// Per Alex 2026-07-14 in-transcript naming: measures position along
/// Narcissus↔Splinter axis. Returns value in `[0, 1]`:
/// - `0.0`: graph is disconnected (Narcissus-collapsed pole)
/// - `1.0`: graph is fully connected (Splinter pole)
///
/// Substrate-honest choice: Fiedler value λ₀ as the scalar. Higher = more
/// available choices per Foerster's ethical imperative operationalized.
pub fn coherence_score(profile: &EigenvalueProfile) -> f64 {
    profile.fiedler_value()
}

/// `@coherence.is_narcissus_pole` — star graph detection bilateral.
///
/// Substrate-honest form of the Narcissus pole (Alex manifesto
/// `Weird - Violence.md`): a single hub node connected to all other
/// nodes; removing the hub collapses connectivity.
///
/// Structural predicate: exists a node with degree = |V| - 1 AND all
/// other nodes have degree 1.
pub fn is_narcissus_pole(graph: &ConceptGraph) -> bool {
    let n = graph.nodes.len();
    if n < 3 {
        return false;
    }
    let mut degrees = vec![0usize; n];
    for edge in &graph.edges {
        let (a, b) = edge_endpoints(edge);
        if a < n && b < n {
            degrees[a] += 1;
            degrees[b] += 1;
        }
    }
    let hub_degree = n - 1;
    let hub_count = degrees.iter().filter(|&&d| d == hub_degree).count();
    let leaf_count = degrees.iter().filter(|&&d| d == 1).count();
    hub_count == 1 && leaf_count == n - 1
}

/// `@coherence.is_splinter_pole` — complete graph detection bilateral.
///
/// Substrate-honest form of the Splinter pole (Alex manifesto
/// `Weird - Violence.md`): every node connected to every other node;
/// removing any node preserves connectivity.
///
/// Structural predicate: |E| = |V| * (|V| - 1) / 2 AND every node has
/// degree |V| - 1.
pub fn is_splinter_pole(graph: &ConceptGraph) -> bool {
    let n = graph.nodes.len();
    if n < 2 {
        return false;
    }
    let expected_edges = n * (n - 1) / 2;
    let unique_edges = count_unique_edges(graph);
    unique_edges == expected_edges
}

fn edge_endpoints(edge: &GraphEdge) -> (usize, usize) {
    match edge {
        GraphEdge::Contains { parent_idx, child_idx, .. } => (*parent_idx, *child_idx),
        GraphEdge::SimilarContent { a_idx, b_idx, .. } => (*a_idx, *b_idx),
        GraphEdge::CrossRef { source_idx, target_idx, .. } => (*source_idx, *target_idx),
    }
}

fn count_unique_edges(graph: &ConceptGraph) -> usize {
    let mut seen = std::collections::HashSet::new();
    for edge in &graph.edges {
        let (a, b) = edge_endpoints(edge);
        let key = if a < b { (a, b) } else { (b, a) };
        seen.insert(key);
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coherence_score_zero_on_dark_profile() {
        let profile = EigenvalueProfile::dark();
        assert_eq!(coherence_score(&profile), 0.0);
    }

    #[test]
    fn coherence_score_in_unit_interval() {
        let profile = EigenvalueProfile { values: [0.5; 16] };
        let score = coherence_score(&profile);
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn is_narcissus_rejects_small_graphs() {
        let graph = ConceptGraph { nodes: vec![], edges: vec![] };
        assert!(!is_narcissus_pole(&graph));
    }

    #[test]
    fn is_splinter_rejects_singleton() {
        let graph = ConceptGraph { nodes: vec![], edges: vec![] };
        assert!(!is_splinter_pole(&graph));
    }
}
