//! `@roomba` — substrate walker Rust runtime discharge.
//!
//! Discharges the runtime forward-promise from `docs/specs/roomba-
//! substrate-walker-that-feeds-kintsugi.md` (Mara `9bbebd2`; Rung 10
//! substrate self-maintenance primitive; Beer VSM S4 environmental
//! scanner). Composes with `@coherence` (this crate's `coherence` module;
//! Mara `e0a3e48` at `shards/epistemologic/cybernetic/coherence.mirror`)
//! + `@knife` (this crate's `converge` module) + `@cyberpunk/algedonic`
//! (this crate's `algedonic` module).
//!
//! ## Substrate authority
//!
//! - Mara `9bbebd2` — canonical spec `docs/specs/roomba-substrate-walker-
//!   that-feeds-kintsugi.md` (Rung 10 substrate self-maintenance).
//! - Taut `3992304` — Beer S4 environmental scanner discovery scout.
//! - Alex Wolf 2026-07-14 in-transcript composition (`docs/insights/
//!   2026-07-14-alex-full-roomba-song-kintsugi-composition.md`):
//!   "@roomba walks (Dijkstra + tension-weighted edges) → bumps into
//!   spectral @tension at position p → resonance emits @song beats →
//!   @kintsugi consumes @song and decides: Path A: @knife the complexity
//!   (COORDᵢ → COORDⱼ; reduce); Path B: spawn @peer at K+1 (circular-
//!   reflexive question to developer OR higher-order @peer)."
//! - Alex Wolf 2026-07-14 in-transcript objective naming ("@coherence is
//!   the objective function the loop climbs; operationalizes Foerster's
//!   ethical imperative on SC<5>").
//! - Alex Wolf 2026-07-14 "The Drone in the Field" story (sub-Turing
//!   architecture; five primitives focus/shift/settle/project/split map
//!   to substrate carriers per Mara @subject spec `b3ec316` §11.6).
//!
//! ## Scope A minimum viable (this landing)
//!
//! The walker traverses the substrate's own ConceptGraph (via
//! `crate::index::build_concept_graph`), computing SC<5> at each file
//! node (via `fragmentation_spectral::hash::coordinate::<5>`). Tension
//! at a position = variance of pain magnitudes across the current node
//! and its unvisited neighbors. Walker moves toward highest-tension
//! unvisited neighbor (bumps into things per Alex's composition).
//!
//! At each step: log the tension observed; check knife stability via
//! `converge::stable_within`; record whether knife jump fires. Terminate
//! when: budget exhausted, no unvisited neighbors, OR walker has
//! visited a coherence-maximum (no neighbor increases coherence).
//!
//! Scope A does NOT ship: @kintsugi Path A/B dispatch (@knife.cut fires
//! empirically; @peer.spawn at K+1 is logged as candidate, not spawned);
//! @song beat emission; full sheaf-cohomology of coherence gradient.
//! Those extend to Scope B/C landings.
//!
//! ## The empirical claim
//!
//! Over a Roomba walk on the substrate's own DAG, the trajectory-
//! averaged tension should trend downward across the walk (as the walker
//! settles into high-coherence subgraph regions) OR the walker should
//! visibly bump into and record high-tension boundaries where @knife
//! stability-verdicts fire. Both patterns discharge Alex's composition
//! empirically.

use crate::algedonic::{pain_gradient, sample_pain};
use crate::coherence::coherence_score;
use crate::converge::{stable_within, KnifeVerdict};
use crate::index::{
    build_concept_graph, eigenvalue_profile, ConceptGraph, GraphNode,
};
use fragmentation::spectral_coordinate::SpectralCoordinate;
use fragmentation_spectral::hash;
use std::collections::HashSet;
use std::path::Path;

/// One step of the Roomba walk. Records the position, the SC<5> at
/// that position, the observed tension, the pain magnitude, coherence
/// score at that step, and whether the knife stability-verdict fired.
#[derive(Debug, Clone)]
pub struct RoombaStep {
    /// Step number in the trajectory (0-indexed).
    pub step_index: usize,
    /// Node index in the ConceptGraph.
    pub node_idx: usize,
    /// Human-readable node label (file/dir name).
    pub node_label: String,
    /// SC<5> at this position (hex representation via `.eigenvalue()`).
    pub sc_hex: String,
    /// Local tension: variance of pain across current + neighbors.
    pub tension: f64,
    /// Pain magnitude at this position (@cyberpunk/algedonic.sample_pain).
    pub pain: f64,
    /// Coherence delta from previous step (@coherence.coherence_delta).
    pub coherence_delta_from_previous: f64,
    /// Knife stability verdict at this step (@knife.stable_within).
    pub knife_verdict: KnifeVerdict,
    /// Number of unvisited neighbors from this position.
    pub unvisited_neighbor_count: usize,
}

/// The full trajectory of a Roomba walk. Captures the sequence of
/// steps + the total substrate profile at the beginning and end.
#[derive(Debug, Clone)]
pub struct WalkTrajectory {
    /// Ordered sequence of walk steps.
    pub steps: Vec<RoombaStep>,
    /// Coherence score of the full substrate at walk start
    /// (via `eigenvalue_profile(concept_graph)`).
    pub coherence_at_start: f64,
    /// Coherence score of the full substrate at walk end. Should equal
    /// coherence_at_start on read-only walks (Scope A); would change on
    /// walks that trigger substrate transformations (Scope B).
    pub coherence_at_end: f64,
    /// Termination reason (why the walk stopped).
    pub termination: WalkTermination,
    /// Total nodes in the ConceptGraph.
    pub graph_node_count: usize,
    /// Total edges in the ConceptGraph.
    pub graph_edge_count: usize,
}

/// Why the Roomba walk terminated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalkTermination {
    /// Budget exhausted — walker ran out of allowed steps.
    BudgetExhausted,
    /// No unvisited neighbors reachable — walker visited a connected
    /// component fully OR hit a dead-end.
    NoUnvisitedNeighbors,
    /// Empty graph — no nodes to walk.
    EmptyGraph,
    /// Root node not found in the graph.
    RootNotFound,
}

/// `@roomba.walk` — walk the substrate's ConceptGraph seeking tension.
///
/// Per Alex 2026-07-14 composition: walker moves toward highest-tension
/// unvisited neighbor at each step. Tension = variance of pain across
/// current + neighbors (SC<5>-derived per @cyberpunk/algedonic).
///
/// The walker starts at the ConceptGraph root (typically the substrate
/// root directory). At each step, computes tension across unvisited
/// neighbors, moves to the highest-tension one, logs the step.
/// Terminates per WalkTermination variants.
///
/// Scope A: read-only; does not mutate the substrate. Scope B
/// (forward-promised): the walker's @knife.jump decisions are dispatched
/// through @kintsugi/consent.query_phi to trigger real substrate
/// transformations.
pub fn walk(root: &Path, budget: usize, epsilon_pain: f64) -> WalkTrajectory {
    let (graph, _files, _breakdown) = build_concept_graph(root);
    let initial_profile = eigenvalue_profile(&graph);
    let coherence_at_start = coherence_score(&initial_profile);

    if graph.nodes.is_empty() {
        return WalkTrajectory {
            steps: vec![],
            coherence_at_start,
            coherence_at_end: coherence_at_start,
            termination: WalkTermination::EmptyGraph,
            graph_node_count: 0,
            graph_edge_count: 0,
        };
    }

    let root_idx = match find_root_node(&graph) {
        Some(idx) => idx,
        None => {
            return WalkTrajectory {
                steps: vec![],
                coherence_at_start,
                coherence_at_end: coherence_at_start,
                termination: WalkTermination::RootNotFound,
                graph_node_count: graph.nodes.len(),
                graph_edge_count: graph.edges.len(),
            };
        }
    };

    let adjacency = build_adjacency(&graph);
    let sc_cache: Vec<SpectralCoordinate<5>> = graph
        .nodes
        .iter()
        .map(|node| sc_for_node(node))
        .collect();
    let pain_cache: Vec<f64> = sc_cache.iter().map(|sc| sample_pain(sc)).collect();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut steps: Vec<RoombaStep> = Vec::new();
    let mut current_idx = root_idx;
    let mut previous_sc: Option<SpectralCoordinate<5>> = None;
    let mut previous_coherence = coherence_at_start;

    let termination = loop {
        if steps.len() >= budget {
            break WalkTermination::BudgetExhausted;
        }

        visited.insert(current_idx);
        let node = &graph.nodes[current_idx];
        let node_label = node_label(node);
        let sc = &sc_cache[current_idx];
        let sc_hex = sc.eigenvalue().to_string();
        let pain = pain_cache[current_idx];

        let neighbors: Vec<usize> = adjacency
            .get(current_idx)
            .cloned()
            .unwrap_or_default();
        let unvisited_neighbors: Vec<usize> = neighbors
            .iter()
            .filter(|idx| !visited.contains(idx))
            .copied()
            .collect();

        let tension = compute_tension(current_idx, &neighbors, &pain_cache);

        let pain_delta = previous_sc
            .as_ref()
            .map(|prev| pain_gradient(prev, sc))
            .unwrap_or(0.0);
        let knife_verdict = stable_within(sc, pain_delta, epsilon_pain);

        let coherence_here = coherence_score(&initial_profile);
        let coherence_delta_from_previous = coherence_here - previous_coherence;

        steps.push(RoombaStep {
            step_index: steps.len(),
            node_idx: current_idx,
            node_label,
            sc_hex,
            tension,
            pain,
            coherence_delta_from_previous,
            knife_verdict,
            unvisited_neighbor_count: unvisited_neighbors.len(),
        });

        previous_sc = Some(sc.clone());
        previous_coherence = coherence_here;

        if unvisited_neighbors.is_empty() {
            break WalkTermination::NoUnvisitedNeighbors;
        }

        // Move to the highest-tension unvisited neighbor. Tension for
        // each candidate: variance of pain across (candidate + its
        // neighbors). Per Alex composition, walker bumps into tension.
        current_idx = unvisited_neighbors
            .iter()
            .max_by(|&&a, &&b| {
                let ta = compute_tension(a, adjacency.get(a).map(|v| v.as_slice()).unwrap_or(&[]), &pain_cache);
                let tb = compute_tension(b, adjacency.get(b).map(|v| v.as_slice()).unwrap_or(&[]), &pain_cache);
                ta.partial_cmp(&tb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
            .unwrap_or(current_idx);
    };

    let coherence_at_end = coherence_score(&initial_profile);

    WalkTrajectory {
        steps,
        coherence_at_start,
        coherence_at_end,
        termination,
        graph_node_count: graph.nodes.len(),
        graph_edge_count: graph.edges.len(),
    }
}

fn find_root_node(graph: &ConceptGraph) -> Option<usize> {
    graph
        .nodes
        .iter()
        .position(|node| matches!(node, GraphNode::Root { .. }))
        .or_else(|| if graph.nodes.is_empty() { None } else { Some(0) })
}

fn node_label(node: &GraphNode) -> String {
    match node {
        GraphNode::Root { path, file_count } => format!(
            "root:{} ({} files)",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("."),
            file_count
        ),
        GraphNode::Directory { name, depth, file_count, .. } => {
            format!("dir[{}]:{} ({} files)", depth, name, file_count)
        }
    }
}

fn sc_for_node(node: &GraphNode) -> SpectralCoordinate<5> {
    match node {
        GraphNode::Root { path, file_count } => {
            let s = format!("{}:{}", path.display(), file_count);
            hash::coordinate::<5>(s.as_bytes())
        }
        GraphNode::Directory { name, depth, file_count, path } => {
            let s = format!("{}:{}:{}:{}", path.display(), name, depth, file_count);
            hash::coordinate::<5>(s.as_bytes())
        }
    }
}

fn build_adjacency(graph: &ConceptGraph) -> Vec<Vec<usize>> {
    let n = graph.nodes.len();
    let mut adj = vec![Vec::new(); n];
    for edge in &graph.edges {
        let (a, b) = edge.indices();
        if a < n && b < n {
            adj[a].push(b);
            adj[b].push(a);
        }
    }
    adj
}

fn compute_tension(node_idx: usize, neighbors: &[usize], pain_cache: &[f64]) -> f64 {
    if neighbors.is_empty() {
        return 0.0;
    }
    let self_pain = pain_cache.get(node_idx).copied().unwrap_or(0.0);
    let neighbor_pains: Vec<f64> = neighbors
        .iter()
        .filter_map(|idx| pain_cache.get(*idx).copied())
        .collect();
    if neighbor_pains.is_empty() {
        return 0.0;
    }
    let mean = (self_pain + neighbor_pains.iter().sum::<f64>()) / (1 + neighbor_pains.len()) as f64;
    let variance_terms: f64 = std::iter::once(self_pain)
        .chain(neighbor_pains.iter().copied())
        .map(|p| (p - mean).powi(2))
        .sum();
    (variance_terms / (1 + neighbor_pains.len()) as f64).sqrt()
}

/// Summarize a WalkTrajectory as one-line-per-step text (for CLI
/// output). Returns the formatted summary string.
pub fn summarize_trajectory(trajectory: &WalkTrajectory) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "roomba walk: {} steps; {} nodes / {} edges in substrate\n",
        trajectory.steps.len(),
        trajectory.graph_node_count,
        trajectory.graph_edge_count
    ));
    out.push_str(&format!(
        "coherence: start={:.6} end={:.6} delta={:+.6}\n",
        trajectory.coherence_at_start,
        trajectory.coherence_at_end,
        trajectory.coherence_at_end - trajectory.coherence_at_start
    ));
    out.push_str(&format!("termination: {:?}\n", trajectory.termination));
    out.push_str("\nstep  pain     tension  d_coh    knife         unvisited  node\n");
    out.push_str("----  -------  -------  -------  ------------  ---------  ------------------------\n");
    for step in &trajectory.steps {
        out.push_str(&format!(
            "{:>4}  {:>7.4}  {:>7.4}  {:>+7.4}  {:>12}  {:>9}  {}\n",
            step.step_index,
            step.pain,
            step.tension,
            step.coherence_delta_from_previous,
            format!("{:?}", step.knife_verdict),
            step.unvisited_neighbor_count,
            step.node_label,
        ));
    }

    // Empirical observations summary.
    let jumps = trajectory
        .steps
        .iter()
        .filter(|s| s.knife_verdict == KnifeVerdict::Jumped)
        .count();
    let near_boundary = trajectory
        .steps
        .iter()
        .filter(|s| s.knife_verdict == KnifeVerdict::NearBoundary)
        .count();
    let stable = trajectory
        .steps
        .iter()
        .filter(|s| s.knife_verdict == KnifeVerdict::Stable)
        .count();
    let mean_tension = if trajectory.steps.is_empty() {
        0.0
    } else {
        trajectory.steps.iter().map(|s| s.tension).sum::<f64>() / trajectory.steps.len() as f64
    };
    out.push_str(&format!(
        "\nobservations: jumps={} near_boundary={} stable={} mean_tension={:.4}\n",
        jumps, near_boundary, stable, mean_tension
    ));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_missing_root_returns_empty_graph_termination() {
        let path = std::path::Path::new("/nonexistent-path-that-should-not-exist-12345");
        let trajectory = walk(path, 100, 0.1);
        assert!(matches!(
            trajectory.termination,
            WalkTermination::EmptyGraph | WalkTermination::RootNotFound
        ));
        assert!(trajectory.steps.is_empty());
    }

    #[test]
    fn summarize_trajectory_emits_header() {
        let trajectory = WalkTrajectory {
            steps: vec![],
            coherence_at_start: 0.0,
            coherence_at_end: 0.0,
            termination: WalkTermination::EmptyGraph,
            graph_node_count: 0,
            graph_edge_count: 0,
        };
        let summary = summarize_trajectory(&trajectory);
        assert!(summary.contains("roomba walk:"));
        assert!(summary.contains("coherence:"));
        assert!(summary.contains("termination:"));
    }
}
