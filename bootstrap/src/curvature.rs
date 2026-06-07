//! `@epistemologic/math/curvature` — boundary Rust for the discrete-
//! curvature numerical primitive (Balanced Forman, Topping et al. 2022).
//!
//! This module realizes substrate-declared types and actions from
//! [`shards/epistemologic/math/curvature.mirror`] as Rust bodies.
//! T9 in the implementation cascade after T8.5's
//! [`crate::tensor::tensor_of_with_restrictions`] / LAPACK bridge.
//!
//! ## What `balanced_forman` is
//!
//! The per-edge Balanced Forman curvature `Ric(i, j)` per
//! Topping et al. 2022 (arXiv:2111.14522) §4 Definition 1, equation (3).
//! When `min{d_i, d_j} > 1`:
//!
//! ```text
//! Ric(i,j) = 2/d_i + 2/d_j − 2
//!          + 2·|#_Δ(i,j)|/max{d_i, d_j}
//!          + 2·|#_Δ(i,j)|/min{d_i, d_j}
//!          + (γ_max)^{−1} · (|#_□^i| + |#_□^j|) / max{d_i, d_j}
//! ```
//!
//! When `min{d_i, d_j} = 1`: `Ric(i, j) := 0` (Topping et al. 2022 §4
//! Definition 1). The γ_max term is zero when `|#_□^i| = |#_□^j| = 0`.
//!
//! - `d_i, d_j` — degrees of `i` and `j` in the underlying graph.
//! - `|#_Δ(i, j)|` — number of triangles through edge `(i, j)`.
//! - `|#_□^i|, |#_□^j|` — 4-cycle-forming neighbours of `i` and `j`
//!   without diagonals (i.e., not completing a triangle).
//! - `γ_max(i, j)` — maximal number of 4-cycles based at `i ~ j`
//!   traversing a common node.
//!
//! Negative curvature → edge behaves as a bridge between
//! neighbourhoods (bottleneck). Positive curvature → neighbourhoods
//! stay connected after edge removal. Bound: `Ric(i, j) > −2`.
//!
//! ## What lives here
//!
//! - [`Curvature`] — the Rust mirror of the substrate's
//!   `curvature = { value: ref, edge: restriction }`.
//! - [`balanced_forman`] — the executable form of
//!   `balanced_forman(op: operator, edge: restriction) -> curvature`.
//!
//! ## Why this lives in its own module (not `sheaf_laplacian.rs`)
//!
//! The substrate-pull discipline: curvature is its own substrate-altitude
//! carrier (declared in `shards/epistemologic/math/curvature.mirror`)
//! with its own prior-art chain (Topping 2022, Forman 2003, Ollivier
//! 2009). The shard `sheaf_laplacian.mirror` declares the operator
//! family; `curvature.mirror` declares the per-edge read on top of it.
//! Same split as `bundle` vs `spectral-triple` at the boot-std altitude.
//!
//! ## Why boundary Rust (not in-substrate)
//!
//! The triangle / 4-cycle counts are combinatorial and currently best
//! expressed at the boundary. The substrate names the obligation
//! (`balanced_forman(op, edge) -> curvature`); this module discharges
//! it. When the in-substrate counting primitive lands the body lifts
//! to a `\` resolution per the substrate-pull staircase.
//!
//! [`shards/epistemologic/math/curvature.mirror`]: ../../../../shards/epistemologic/math/curvature.mirror
#![allow(dead_code)]

use crate::sheaf_laplacian::{Operator, Restriction};

// ---------------------------------------------------------------------------
// Curvature — the Rust mirror of the substrate's curvature carrier.
// ---------------------------------------------------------------------------

/// The Balanced Forman curvature read on one edge of the operator's
/// underlying graph.
///
/// Per [`shards/epistemologic/math/curvature.mirror`]:
///
/// ```mirror
/// type curvature = {
///   value: ref,
///   edge:  restriction,
/// }
/// ```
///
/// `value` is the scalar curvature (Topping 2022 §4 Definition 1's
/// `Ric(i, j)`); `edge` is the source [`Restriction`] the value applies
/// to. The carrier records both because `@fate.minimize` (per
/// `gap-tension-tensor-substrate.md` §6) ranks edges by curvature and
/// needs to recover edge identity from the ranked list.
///
/// [`shards/epistemologic/math/curvature.mirror`]: ../../../../shards/epistemologic/math/curvature.mirror
#[derive(Clone, Debug, PartialEq)]
pub struct Curvature {
    value: f64,
    edge: Restriction,
}

impl Curvature {
    /// Construct a curvature from its (value, edge) pair.
    pub fn new(value: f64, edge: Restriction) -> Self {
        Curvature { value, edge }
    }

    /// Read this curvature's scalar value (Topping 2022's `Ric(i, j)`).
    pub fn value(c: &Curvature) -> f64 {
        c.value
    }

    /// Borrow the [`Restriction`] this curvature reads.
    pub fn edge(c: &Curvature) -> &Restriction {
        &c.edge
    }
}

// ---------------------------------------------------------------------------
// balanced_forman — Topping 2022 §4 Definition 1 equation (3).
// ---------------------------------------------------------------------------

/// `balanced_forman(op, edge) -> curvature` — Balanced Forman curvature.
///
/// Per [`shards/epistemologic/math/curvature.mirror`]:
///
/// ```mirror
/// balanced_forman(op: operator, edge: restriction) -> curvature { \ }
/// ```
///
/// Implements Topping et al. 2022 (arXiv:2111.14522) §4 Definition 1
/// equation (3):
///
/// ```text
/// Ric(i,j) = 2/d_i + 2/d_j − 2
///          + 2·|#_Δ(i,j)|/max{d_i, d_j}
///          + 2·|#_Δ(i,j)|/min{d_i, d_j}
///          + (γ_max)^{−1} · (|#_□^i| + |#_□^j|) / max{d_i, d_j}
/// ```
///
/// Degenerate case `min{d_i, d_j} = 1`: returns `Curvature(0.0, edge)`
/// per Topping 2022 Definition 1.
///
/// The operator's underlying graph is reconstructed from its
/// [`Operator::entries`]: each [`Restriction(s, t, _)`] is treated as
/// the undirected edge `{s, t}` (matching
/// [`crate::sheaf_laplacian::dense_laplacian`]'s symmetrisation). Self-
/// loops (`s == t`) are skipped in the adjacency reconstruction
/// (they don't contribute to degree by the standard convention).
///
/// Pure; no I/O; allocates a small adjacency for the local
/// neighbourhood walk.
pub fn balanced_forman(op: &Operator, edge: &Restriction) -> Curvature {
    let n = Operator::dimension(op) as usize;
    let i = Restriction::source(edge) as usize;
    let j = Restriction::target(edge) as usize;

    // Build undirected adjacency as a vector of unique neighbour sets.
    // Skip self-loops and out-of-range references. n bounds the walk;
    // the property layer's corpora are small.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for r in Operator::entries(op) {
        let s = Restriction::source(r) as usize;
        let t = Restriction::target(r) as usize;
        if s >= n || t >= n || s == t {
            continue;
        }
        if !adj[s].contains(&t) {
            adj[s].push(t);
        }
        if !adj[t].contains(&s) {
            adj[t].push(s);
        }
    }

    if i >= n || j >= n || i == j {
        return Curvature::new(0.0, edge.clone());
    }

    let d_i = adj[i].len();
    let d_j = adj[j].len();

    // Topping 2022 Definition 1 degenerate case: min{d_i, d_j} = 1
    // (or zero, when the edge isn't represented in the operator).
    let min_d = d_i.min(d_j);
    if min_d <= 1 {
        return Curvature::new(0.0, edge.clone());
    }
    let max_d = d_i.max(d_j);

    // Triangle count: |#_Δ(i, j)| = |N(i) ∩ N(j)|.
    let triangles: usize = adj[i].iter().filter(|&&k| adj[j].contains(&k)).count();

    // 4-cycle-forming neighbours without diagonals, per Topping 2022
    // §4 Definition 1:
    //
    //   #_□^i counts k ∈ N(i), k ≠ j, with k ∉ N(j) (no triangle
    //   through i-j-k) AND ∃ l ∈ N(j), l ≠ i, l ∉ N(i) (no triangle
    //   through j-i-l), with (k, l) an edge — i.e., the 4-cycle
    //   i → k → l → j → i closes without any chord.
    //
    //   #_□^j counts symmetrically.
    //
    //   γ_max(i, j) is the maximal multiplicity of a single k (or l)
    //   participating in 4-cycles through (i, j) — we take the max
    //   over per-vertex counts on both sides.
    let mut sharp_i_count = 0usize;
    let mut sharp_j_count = 0usize;
    let mut gamma_max = 0usize;

    for &k in &adj[i] {
        if k == j {
            continue;
        }
        if adj[j].contains(&k) {
            // k forms a triangle through (i, j) — chord; skip.
            continue;
        }
        let mut count = 0usize;
        for &l in &adj[j] {
            if l == i || l == k {
                continue;
            }
            if adj[i].contains(&l) {
                // l would form a triangle with i; skip.
                continue;
            }
            if adj[k].contains(&l) {
                count += 1;
            }
        }
        if count > 0 {
            sharp_i_count += 1;
            if count > gamma_max {
                gamma_max = count;
            }
        }
    }
    for &l in &adj[j] {
        if l == i {
            continue;
        }
        if adj[i].contains(&l) {
            continue;
        }
        let mut count = 0usize;
        for &k in &adj[i] {
            if k == j || k == l {
                continue;
            }
            if adj[j].contains(&k) {
                continue;
            }
            if adj[l].contains(&k) {
                count += 1;
            }
        }
        if count > 0 {
            sharp_j_count += 1;
            if count > gamma_max {
                gamma_max = count;
            }
        }
    }

    let d_i_f = d_i as f64;
    let d_j_f = d_j as f64;
    let max_d_f = max_d as f64;
    let min_d_f = min_d as f64;
    let triangles_f = triangles as f64;

    let base = 2.0 / d_i_f + 2.0 / d_j_f - 2.0;
    let triangle_term = if triangles == 0 {
        0.0
    } else {
        2.0 * triangles_f / max_d_f + 2.0 * triangles_f / min_d_f
    };
    // γ_max term: zero if both 4-cycle counts are zero (Topping 2022
    // §4 Definition 1) or if γ_max collapsed to zero (no 4-cycles).
    let four_cycle_term = if sharp_i_count == 0 && sharp_j_count == 0 {
        0.0
    } else if gamma_max == 0 {
        0.0
    } else {
        let s = sharp_i_count as f64 + sharp_j_count as f64;
        (1.0 / gamma_max as f64) * s / max_d_f
    };

    Curvature::new(base + triangle_term + four_cycle_term, edge.clone())
}

#[cfg(test)]
mod tests {
    //! The executable spec for [`balanced_forman`] and [`Curvature`] —
    //! T9's Balanced Forman curvature body.
    //!
    //! Per `shards/epistemologic/math/curvature.mirror` and
    //! `docs/specs/gap-tension-tensor-substrate.md` §3.2 / §6:
    //! `balanced_forman` reads the per-edge Topping 2022 §4 Definition 1
    //! curvature on the operator's underlying graph. These tests RED
    //! first; the body lands GREEN in the next commit.
    //!
    //! Canonical graphs and their expected curvatures (Topping 2022):
    //!
    //! - K₂: min degree = 1 → Ric = 0 (degenerate per Definition 1).
    //! - K₃: edge (0,1) has d=2 each, one triangle through it →
    //!   Ric = 2/2 + 2/2 - 2 + 2·1/2 + 2·1/2 = 2.
    //! - P₃ (0-1-2): both edges have min degree 1 → Ric = 0.
    //! - P₄ (0-1-2-3): middle edge (1,2) has d=2 each, no triangles, no
    //!   4-cycles → Ric = 2/2 + 2/2 - 2 = 0; outer edges have min
    //!   degree 1 → Ric = 0.
    //! - C₄ (0-1-2-3-0): edge (0,1) has d=2 each, no triangles
    //!   (bipartite), one 4-cycle through (0,1) → #_□^0 = #_□^1 = 1,
    //!   γ_max = 1 → Ric = 0 + 0 + 1·(1+1)/2 = 1.
    //! - K₄: edge (0,1) has d=3 each, 2 triangles → Ric = 2/3 + 2/3
    //!   - 2 + 2·2/3 + 2·2/3 = 2.

    use super::*;

    fn k2() -> Operator {
        // Edge 0-1.
        Operator::new(2, vec![Restriction::new(0, 1, 1.0)])
    }

    fn k3() -> Operator {
        // Triangle 0-1-2.
        Operator::new(
            3,
            vec![
                Restriction::new(0, 1, 1.0),
                Restriction::new(1, 2, 1.0),
                Restriction::new(0, 2, 1.0),
            ],
        )
    }

    fn p3() -> Operator {
        // Path 0-1-2.
        Operator::new(
            3,
            vec![Restriction::new(0, 1, 1.0), Restriction::new(1, 2, 1.0)],
        )
    }

    fn p4() -> Operator {
        // Path 0-1-2-3.
        Operator::new(
            4,
            vec![
                Restriction::new(0, 1, 1.0),
                Restriction::new(1, 2, 1.0),
                Restriction::new(2, 3, 1.0),
            ],
        )
    }

    fn c4() -> Operator {
        // 4-cycle 0-1-2-3-0.
        Operator::new(
            4,
            vec![
                Restriction::new(0, 1, 1.0),
                Restriction::new(1, 2, 1.0),
                Restriction::new(2, 3, 1.0),
                Restriction::new(3, 0, 1.0),
            ],
        )
    }

    fn k4() -> Operator {
        // Complete graph on 4 vertices.
        Operator::new(
            4,
            vec![
                Restriction::new(0, 1, 1.0),
                Restriction::new(0, 2, 1.0),
                Restriction::new(0, 3, 1.0),
                Restriction::new(1, 2, 1.0),
                Restriction::new(1, 3, 1.0),
                Restriction::new(2, 3, 1.0),
            ],
        )
    }

    // -----------------------------------------------------------------------
    // Curvature carrier shape.
    // -----------------------------------------------------------------------

    /// Curvature carries (value, edge) — the substrate-floor reading.
    #[test]
    fn curvature_carries_value_and_edge() {
        let r = Restriction::new(0, 1, 1.0);
        let c = Curvature::new(-0.5, r.clone());
        assert!((Curvature::value(&c) - (-0.5)).abs() < 1e-12);
        assert_eq!(Curvature::edge(&c), &r);
    }

    // -----------------------------------------------------------------------
    // Canonical graphs — known curvature values per Topping 2022.
    // -----------------------------------------------------------------------

    /// K₂: min degree = 1 → Ric = 0 per Topping 2022 Definition 1.
    #[test]
    fn balanced_forman_on_k2_is_zero() {
        let op = k2();
        let edge = Restriction::new(0, 1, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            Curvature::value(&c).abs() < 1e-12,
            "K₂ edge has min degree 1 → Ric must be 0; got {}",
            Curvature::value(&c),
        );
    }

    /// K₃: every edge has d_i = d_j = 2, one triangle → Ric = 2.
    /// Formula: 2/2 + 2/2 − 2 + 2·1/2 + 2·1/2 = 1 + 1 − 2 + 1 + 1 = 2.
    #[test]
    fn balanced_forman_on_k3_is_two() {
        let op = k3();
        let edge = Restriction::new(0, 1, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            (Curvature::value(&c) - 2.0).abs() < 1e-9,
            "K₃ edge curvature must be 2.0; got {}",
            Curvature::value(&c),
        );
    }

    /// P₃: both edges have min degree 1 → Ric = 0.
    #[test]
    fn balanced_forman_on_p3_outer_edge_is_zero() {
        let op = p3();
        let edge = Restriction::new(0, 1, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            Curvature::value(&c).abs() < 1e-12,
            "P₃ outer edge (min degree 1) → Ric = 0; got {}",
            Curvature::value(&c),
        );
    }

    /// P₄ middle edge (1,2): d_1 = d_2 = 2, no triangles, no 4-cycles
    /// → Ric = 2/2 + 2/2 − 2 = 0.
    #[test]
    fn balanced_forman_on_p4_middle_edge_is_zero() {
        let op = p4();
        let edge = Restriction::new(1, 2, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            Curvature::value(&c).abs() < 1e-12,
            "P₄ middle edge (no triangles, no 4-cycles) → Ric = 0; got {}",
            Curvature::value(&c),
        );
    }

    /// C₄ edge (0,1): d_0 = d_1 = 2, no triangles (bipartite), one
    /// 4-cycle through (0,1) → #_□^0 = #_□^1 = 1, γ_max = 1 →
    /// Ric = 0 + 0 + 1·(1+1)/2 = 1.
    #[test]
    fn balanced_forman_on_c4_is_one() {
        let op = c4();
        let edge = Restriction::new(0, 1, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            (Curvature::value(&c) - 1.0).abs() < 1e-9,
            "C₄ edge curvature must be 1.0; got {}",
            Curvature::value(&c),
        );
    }

    /// K₄ edge (0,1): d_0 = d_1 = 3, 2 triangles through it, no
    /// 4-cycles without diagonals → Ric = 2/3 + 2/3 − 2 + 2·2/3 +
    /// 2·2/3 = 4/3 − 2 + 8/3 = 4 − 2 = 2.
    #[test]
    fn balanced_forman_on_k4_is_two() {
        let op = k4();
        let edge = Restriction::new(0, 1, 1.0);
        let c = balanced_forman(&op, &edge);
        assert!(
            (Curvature::value(&c) - 2.0).abs() < 1e-9,
            "K₄ edge curvature must be 2.0; got {}",
            Curvature::value(&c),
        );
    }

    // -----------------------------------------------------------------------
    // Sign checks: positive (well-connected), zero (neutral), negative
    // (bottleneck).
    // -----------------------------------------------------------------------

    /// A "barbell" graph (two K₃ joined by a single bridge edge) has
    /// the bridge edge with strictly negative curvature — the bridge is
    /// the bottleneck. Construction: K₃ on {0,1,2}; K₃ on {3,4,5};
    /// single bridge edge (2, 3).
    ///
    /// Bridge edge (2,3): d_2 = 3 (neighbours 0, 1, 3); d_3 = 3
    /// (neighbours 2, 4, 5). Triangles through (2,3): none (no shared
    /// neighbours). 4-cycles through (2,3) without diagonals: none
    /// (no k ∈ N(2)\{3} connects to any l ∈ N(3)\{2}).
    /// → Ric = 2/3 + 2/3 − 2 + 0 + 0 = 4/3 − 2 = −2/3 ≈ −0.667.
    #[test]
    fn balanced_forman_on_barbell_bridge_is_negative() {
        let op = Operator::new(
            6,
            vec![
                // K₃ on {0,1,2}
                Restriction::new(0, 1, 1.0),
                Restriction::new(0, 2, 1.0),
                Restriction::new(1, 2, 1.0),
                // K₃ on {3,4,5}
                Restriction::new(3, 4, 1.0),
                Restriction::new(3, 5, 1.0),
                Restriction::new(4, 5, 1.0),
                // Bridge edge
                Restriction::new(2, 3, 1.0),
            ],
        );
        let bridge = Restriction::new(2, 3, 1.0);
        let c = balanced_forman(&op, &bridge);
        assert!(
            Curvature::value(&c) < 0.0,
            "barbell bridge edge must have negative curvature; got {}",
            Curvature::value(&c),
        );
        // Quantitative: −2/3.
        assert!(
            (Curvature::value(&c) - (-2.0 / 3.0)).abs() < 1e-9,
            "barbell bridge edge Ric must be −2/3; got {}",
            Curvature::value(&c),
        );
    }

    /// Bound from Topping 2022 §4 Proposition 4: Ric(i, j) > −2 on
    /// every edge of every graph.
    #[test]
    fn balanced_forman_respects_topping_lower_bound() {
        for op in &[k2(), k3(), p3(), p4(), c4(), k4()] {
            for edge in Operator::entries(op) {
                let c = balanced_forman(op, edge);
                assert!(
                    Curvature::value(&c) > -2.0,
                    "Ric bound violated: {} on edge {:?}",
                    Curvature::value(&c),
                    edge,
                );
            }
        }
    }

    /// Edge that references vertices outside the operator's dimension
    /// returns Ric = 0 (defensive boundary read; substrate-altitude
    /// consumers should not produce such edges, but the body refuses to
    /// panic).
    #[test]
    fn balanced_forman_on_out_of_range_edge_is_zero() {
        let op = k3();
        let bad = Restriction::new(0, 99, 1.0);
        let c = balanced_forman(&op, &bad);
        assert!(
            Curvature::value(&c).abs() < 1e-12,
            "out-of-range edge → Ric = 0 (defensive); got {}",
            Curvature::value(&c),
        );
    }

    /// Self-loop edge (i == j) returns Ric = 0 (the formula is only
    /// defined for i ≠ j; self-loops have no Forman analog).
    #[test]
    fn balanced_forman_on_self_loop_is_zero() {
        let op = k3();
        let loop_edge = Restriction::new(1, 1, 1.0);
        let c = balanced_forman(&op, &loop_edge);
        assert!(
            Curvature::value(&c).abs() < 1e-12,
            "self-loop edge → Ric = 0; got {}",
            Curvature::value(&c),
        );
    }
}
