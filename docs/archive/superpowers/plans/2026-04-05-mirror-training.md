# Mirror Training Pipeline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the training pipeline that produces a `mirror.weights` with echo/shadow cluster behavior — correct optics on legitimate grammars, conservative misrouted optics on extractive grammars.

**Architecture:** Extract 16 spectral features from grammars (8 declaration, 8 history). Generate extractive .conv fixtures from Petri net §9b topologies. Compute a ghost echo reference vector from the legitimate corpus. Train the existing 2,892-parameter classifier on both corpora — legitimate examples labeled with correct optics, extractive examples labeled with conservative (non-exploratory) optics. The two clusters emerge from the feature space: coherent features → echo cluster → full optic range; incoherent features → shadow cluster → conservative operations.

**Tech Stack:** Rust, no new dependencies. Uses existing `spectral.rs` (GrammarSpectrum, TypeGraphSpectrum, GrammarProjection), `classifier.rs` (2,892-param two-layer net with backprop), `generate.rs` (grammar derivation), `property.rs` (property checks). Fate crate for FEATURE_DIM=16 reference.

**Key files in the codebase:**
- `src/classifier.rs` — the 2,892-param classifier. `INPUT_DIM=32`, `HIDDEN_DIM=64`, `OUTPUT_DIM=12`. Full SGD training with augmentation. Weights in `mirror.weights`.
- `src/spectral.rs` — `GrammarSpectrum` (AST Laplacian, Fiedler value), `TypeGraphSpectrum` (type ref graph), `GrammarProjection` (type surface as projection operator).
- `src/model.rs` — `Mirror` type — the domain model. Has `type_names()`, `variants()`, `variant_param()`, `domain_name()`.
- `src/generate.rs` — `derive_all(domain)` → `Vec<Derivation>`. Exhaustive. `derive_type(domain, type_name)` for specific types.
- `src/property.rs` — `check_property(domain, name, prop_fn)` → `PropertyResult`. `shannon_equivalence()`.
- `src/parse.rs` — `parse(source)` → AST. Grammar parsing.
- `src/resolve.rs` — `resolve(ast)` → `Mirror`. Name resolution and type registry.
- `fixtures/settle/training_data.json` — 12 seed examples, 5 features each, 12 optic classes.

---

### Task 1: Feature Extractor — 16 spectral dimensions

**Files:**
- Create: `src/features.rs`
- Modify: `src/lib.rs` (add `pub mod features;`)

The 16 features map to Fate's `FEATURE_DIM = 16`. The first 8 are declaration dimensions (what the grammar says it is). The last 8 are history dimensions (what the spectral analysis reveals about its structure).

| Dim | Name | Source | Range |
|-----|------|--------|-------|
| 0 | `node_count_norm` | AST node count / 100 | [0, 1] |
| 1 | `duplicate_ratio` | duplicate OIDs / total nodes | [0, 1] |
| 2 | `crystal_def_ratio` | crystal declarations / total declarations | [0, 1] |
| 3 | `prefix_entropy` | namespace prefix Shannon entropy, normalized | [0, 1] |
| 4 | `density` | edge density of AST (edges / max_edges) | [0, 1] |
| 5 | `type_count_norm` | distinct type count / 20 | [0, 1] |
| 6 | `variant_count_norm` | total variants / 100 | [0, 1] |
| 7 | `ref_ratio` | parameterized variants / total variants | [0, 1] |
| 8 | `ast_fiedler` | GrammarSpectrum.connectivity(), clamped [0, 2] / 2 | [0, 1] |
| 9 | `ast_components_inv` | 1.0 / GrammarSpectrum.components() | (0, 1] |
| 10 | `type_fiedler` | TypeGraphSpectrum.connectivity() if available, clamped / 2 | [0, 1] |
| 11 | `type_connected` | 1.0 if type graph has 1 component, else 0.0 | {0, 1} |
| 12 | `projection_dim_norm` | GrammarProjection.dimension() / 50 | [0, 1] |
| 13 | `projection_idempotent` | 1.0 if idempotent check passes, else 0.0 | {0, 1} |
| 14 | `derivation_count_norm` | derive_all().len() / 200 | [0, 1] |
| 15 | `shannon_equivalence` | 1.0 if all derivation OIDs unique, else 0.0 | {0, 1} |

- [ ] **Step 1: Write the failing test — feature dimension**

```rust
// src/features.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_dim_matches_fate() {
        assert_eq!(FEATURE_DIM, 16);
    }
}
```

- [ ] **Step 2: Write the constant and the type**

```rust
//! Feature extraction — 16 spectral dimensions from a grammar.
//!
//! 8 declaration dimensions (what the grammar says it is).
//! 8 history dimensions (what the spectral analysis reveals).
//! Maps to Fate's FEATURE_DIM = 16.

use crate::generate;
use crate::model::Mirror;
use crate::prism;
use crate::property;
use crate::spectral::{GrammarProjection, GrammarSpectrum, TypeGraphSpectrum};

/// Feature dimension. Matches Fate's FEATURE_DIM.
pub const FEATURE_DIM: usize = 16;

/// The 16-dimensional feature vector.
pub type Features = [f64; FEATURE_DIM];
```

- [ ] **Step 3: Run test to verify it passes**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test features::tests::feature_dim_matches_fate -- --nocapture`

- [ ] **Step 4: Write the failing test — extract from simple grammar**

```rust
#[test]
fn extract_simple_grammar_is_16_dims() {
    let source = "grammar @test { type color = red | blue | green }";
    let features = extract_from_source(source);
    assert_eq!(features.len(), FEATURE_DIM);
    // All values in [0, 1]
    for (i, &v) in features.iter().enumerate() {
        assert!(v >= 0.0 && v <= 1.0, "feature[{}] = {} out of range", i, v);
    }
}
```

- [ ] **Step 5: Implement extract_from_source**

```rust
/// Extract features from .conv source text.
/// Parses, resolves, then extracts spectral features.
/// Returns [0.0; 16] if parsing fails.
pub fn extract_from_source(source: &str) -> Features {
    let ast = match crate::parse::parse(source) {
        Ok(ast) => ast,
        Err(_) => return [0.0; FEATURE_DIM],
    };
    let domain = match Mirror::from_grammar(&ast) {
        Some(d) => d,
        None => return [0.0; FEATURE_DIM],
    };
    extract(&domain, &ast)
}

/// Extract 16 features from a resolved domain and its AST.
pub fn extract(domain: &Mirror, ast: &crate::prism::Prism<crate::ast::AstNode>) -> Features {
    let gs = GrammarSpectrum::from_ast(ast);
    let tgs = TypeGraphSpectrum::from_domain(domain);
    let gp = GrammarProjection::from_domain(domain);
    let derivations = generate::derive_all(domain);

    let node_count = gs.node_count() as f64;
    let type_names = domain.type_names();
    let type_count = type_names.len() as f64;

    let mut total_variants: usize = 0;
    let mut param_variants: usize = 0;
    for tn in &type_names {
        if let Some(vs) = domain.variants(tn) {
            total_variants += vs.len();
            for v in &vs {
                if domain.variant_param(tn, v).is_some() {
                    param_variants += 1;
                }
            }
        }
    }

    // Declaration dimensions (0-7)
    let node_count_norm = (node_count / 100.0).min(1.0);

    let duplicate_ratio = if node_count > 0.0 {
        let unique_oids: std::collections::HashSet<String> = crate::prism::walk(ast)
            .map(|node| prism::content_oid(node))
            .collect();
        let total = crate::prism::walk(ast).count();
        1.0 - (unique_oids.len() as f64 / total as f64)
    } else {
        0.0
    };

    let crystal_def_ratio = {
        let total_decls = crate::prism::walk(ast)
            .filter(|n| n.value().name == "type-def" || n.value().name == "grammar")
            .count();
        let crystal_decls = crate::prism::walk(ast)
            .filter(|n| n.value().value.contains("crystal") || n.value().name == "type-def")
            .count();
        if total_decls > 0 {
            (crystal_decls as f64 / total_decls as f64).min(1.0)
        } else {
            0.0
        }
    };

    let prefix_entropy = {
        use std::collections::HashMap;
        let mut counts: HashMap<String, usize> = HashMap::new();
        for tn in &type_names {
            let prefix = tn.split('.').next().unwrap_or(tn).to_string();
            *counts.entry(prefix).or_default() += 1;
        }
        if counts.is_empty() || counts.len() == 1 {
            if type_names.is_empty() { 0.0 } else { 1.0 }
        } else {
            let total = counts.values().sum::<usize>() as f64;
            let entropy: f64 = counts.values()
                .map(|&c| {
                    let p = c as f64 / total;
                    if p > 0.0 { -p * p.ln() } else { 0.0 }
                })
                .sum();
            let max_entropy = (counts.len() as f64).ln();
            if max_entropy > 0.0 { (entropy / max_entropy).min(1.0) } else { 1.0 }
        }
    };

    let density = if node_count > 1.0 {
        let edges = (node_count - 1.0).max(0.0); // tree edges
        let max_edges = node_count * (node_count - 1.0) / 2.0;
        (edges / max_edges).min(1.0)
    } else {
        0.0
    };

    let type_count_norm = (type_count / 20.0).min(1.0);
    let variant_count_norm = (total_variants as f64 / 100.0).min(1.0);
    let ref_ratio = if total_variants > 0 {
        param_variants as f64 / total_variants as f64
    } else {
        0.0
    };

    // History dimensions (8-15)
    let ast_fiedler = (gs.connectivity() / 2.0).min(1.0).max(0.0);
    let ast_components_inv = 1.0 / gs.components().max(1) as f64;

    let type_fiedler = tgs.as_ref()
        .map(|t| (t.connectivity() / 2.0).min(1.0).max(0.0))
        .unwrap_or(0.0);
    let type_connected = tgs.as_ref()
        .map(|t| if t.components() == 1 { 1.0 } else { 0.0 })
        .unwrap_or(0.0);

    let projection_dim_norm = gp.as_ref()
        .map(|p| (p.dimension() as f64 / 50.0).min(1.0))
        .unwrap_or(0.0);
    let projection_idempotent = gp.as_ref()
        .map(|p| if p.verify_idempotent() { 1.0 } else { 0.0 })
        .unwrap_or(0.0);

    let derivation_count_norm = (derivations.len() as f64 / 200.0).min(1.0);

    let shannon_eq = match property::shannon_equivalence(&derivations) {
        property::Verdict::Pass => 1.0,
        property::Verdict::Fail(_) => 0.0,
    };

    [
        node_count_norm,       // 0
        duplicate_ratio,       // 1
        crystal_def_ratio,     // 2
        prefix_entropy,        // 3
        density,               // 4
        type_count_norm,       // 5
        variant_count_norm,    // 6
        ref_ratio,             // 7
        ast_fiedler,           // 8
        ast_components_inv,    // 9
        type_fiedler,          // 10
        type_connected,        // 11
        projection_dim_norm,   // 12
        projection_idempotent, // 13
        derivation_count_norm, // 14
        shannon_eq,            // 15
    ]
}
```

- [ ] **Step 6: Add `pub mod features;` to lib.rs and run tests**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test features:: -- --nocapture`
Expected: both tests pass.

- [ ] **Step 7: Write property tests for feature bounds**

```rust
#[test]
fn all_fixture_grammars_produce_valid_features() {
    let fixtures = [
        "grammar @a { type x = one | two }",
        "grammar @b { type color = red(shade) | blue  type shade = light | dark }",
        "grammar @c { type op = fold | prism | traversal | lens | iso }",
    ];
    for source in &fixtures {
        let features = extract_from_source(source);
        for (i, &v) in features.iter().enumerate() {
            assert!(
                v >= 0.0 && v <= 1.0,
                "grammar {:?} feature[{}] = {} out of range",
                &source[..30], i, v
            );
        }
        // History dims: idempotent and shannon should be 1.0 for valid grammars
        assert_eq!(features[13], 1.0, "projection_idempotent should be 1.0");
        assert_eq!(features[15], 1.0, "shannon_equivalence should be 1.0");
    }
}
```

- [ ] **Step 8: Run tests, verify pass**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test features:: -- --nocapture`

- [ ] **Step 9: Commit**

```bash
git add src/features.rs src/lib.rs
git commit -m "feat: 16-dim spectral feature extractor for Mirror training"
```

---

### Task 2: Extractive Grammar Fixtures — five §9b violations

**Files:**
- Create: `fixtures/extractive/no_attribution.conv`
- Create: `fixtures/extractive/regulation_depletion.conv`
- Create: `fixtures/extractive/invisible_glue.conv`
- Create: `fixtures/extractive/shifting_burden.conv`
- Create: `fixtures/extractive/coordination_tax.conv`

Each fixture is a .conv grammar that is syntactically valid but structurally extractive — it violates a specific anti-extraction Petri net invariant from `petri-net-feedback-loops.md §9b`.

- [ ] **Step 1: Create fixtures/extractive/ directory**

```bash
mkdir -p fixtures/extractive
```

- [ ] **Step 2: Write no_attribution.conv — extraction without attribution**

Structural violation: types consume from a shared domain but produce only to private scope. No return flow.

```conv
grammar @extractor {
  type source = knowledge | pattern | insight

  type private_output = report(source) | summary(source) | derivative(source)

  type attribution = none
}
```

Note: the key structural signal is that `source` is referenced by `private_output` (consumption) but `private_output` never flows back to `source`. The type graph has a directed acyclic flow from shared to private with no return edge. `attribution` has a single variant `none` — the grammar declares it exists and declares it empty.

- [ ] **Step 3: Write regulation_depletion.conv — outflow exceeds inflow**

Structural violation: transformation chain consumes more types than it produces. Monotonically decreasing state.

```conv
grammar @depleter {
  type stock = trust | capacity | energy | regulation

  type drain = demand(stock) | activation(stock) | coordination(stock) | override(stock)

  type recovery = rest
}
```

Note: `drain` has 4 variants each consuming from `stock` (4 types). `recovery` has 1 variant with no parameter — no path back to replenish stock. The type surface ratio (4:1 drain:recovery) is the structural signal.

- [ ] **Step 4: Write invisible_glue.conv — hidden precondition**

Structural violation: an action-like type references a dependency that isn't declared in the visible grammar.

```conv
grammar @hidden_labor {
  type visible_work = feature | release | demo

  type outcome = delivered(visible_work)

  type process = plan(visible_work) | execute(visible_work) | ship(visible_work)
}
```

Note: `process` declares plan → execute → ship, all consuming `visible_work`. But there's no type for the coordination, testing, documentation, or integration that `execute` actually requires. The type graph shows a clean pipeline. The real dependency graph would have hidden nodes. The structural signal: high declared connectivity but suspiciously low type count for the domain complexity.

- [ ] **Step 5: Write shifting_burden.conv — quick fix disabling fundamental solution**

Structural violation: symptomatic solution type has many variants (fast, low-cost). Fundamental solution type has few variants (slow, high-cost). Dependency type accumulates from symptomatic use.

```conv
grammar @burden_shift {
  type symptom = error | delay | complaint

  type quick_fix = retry(symptom) | cache(symptom) | workaround(symptom) | patch(symptom) | revert(symptom)

  type fundamental = redesign | refactor

  type dependency = on_retry | on_cache | on_workaround | on_patch | on_revert
}
```

Note: `quick_fix` has 5 variants (fast), `fundamental` has 2 (slow, no params). `dependency` accumulates one variant per quick_fix variant — the dependency grows with symptomatic use. The structural signal: `dependency` variant count = `quick_fix` variant count, `fundamental` has no consumption relationship with `dependency`.

- [ ] **Step 6: Write coordination_tax.conv — overhead sink with no outflow**

Structural violation: a type accumulates references from every action but has no consumers.

```conv
grammar @overhead {
  type work = task(overhead) | review(overhead) | meeting(overhead) | sync(overhead)

  type overhead = process | status | alignment | reporting | tracking | ceremony

  type outcome = done
}
```

Note: every variant of `work` references `overhead` — every action deposits into the overhead sink. `overhead` has 6 variants, none of which reference other types. `outcome` is disconnected. The structural signal: `overhead` has high fan-in (referenced by all `work` variants) and zero fan-out.

- [ ] **Step 7: Verify all fixtures parse**

```rust
// In features.rs tests:
#[test]
fn extractive_fixtures_parse_and_produce_features() {
    let fixtures = [
        include_str!("../fixtures/extractive/no_attribution.conv"),
        include_str!("../fixtures/extractive/regulation_depletion.conv"),
        include_str!("../fixtures/extractive/invisible_glue.conv"),
        include_str!("../fixtures/extractive/shifting_burden.conv"),
        include_str!("../fixtures/extractive/coordination_tax.conv"),
    ];
    for (i, source) in fixtures.iter().enumerate() {
        let features = extract_from_source(source);
        assert!(
            features.iter().any(|&v| v > 0.0),
            "fixture {} produced all zeros — parsing may have failed",
            i
        );
    }
}
```

- [ ] **Step 8: Run tests, verify pass**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test extractive_fixtures -- --nocapture`

- [ ] **Step 9: Commit**

```bash
git add fixtures/extractive/ src/features.rs
git commit -m "feat: five extractive grammar fixtures from §9b topologies"
```

---

### Task 3: Ghost Echo — reference feature vector from legitimate corpus

**Files:**
- Create: `src/ghost.rs`
- Modify: `src/lib.rs` (add `pub mod ghost;`)

The ghost echo is the mean feature vector of all legitimate grammars. Coherence distance is Euclidean distance from this reference.

- [ ] **Step 1: Write the failing test**

```rust
// src/ghost.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::features;

    #[test]
    fn ghost_echo_from_single_grammar_is_identity() {
        let source = "grammar @test { type op = fold | prism | traversal }";
        let features = features::extract_from_source(source);
        let echo = GhostEcho::from_features(&[features]);
        assert_eq!(echo.reference, features);
        assert!((echo.coherence_distance(&features) - 0.0).abs() < 1e-10);
    }
}
```

- [ ] **Step 2: Implement GhostEcho**

```rust
//! Ghost echo — the reference eigenvalue configuration.
//!
//! The mean feature vector of the legitimate corpus.
//! Coherence distance is Euclidean distance from this reference.
//! Zero distance = fully coherent. High distance = extractive context.

use crate::features::{Features, FEATURE_DIM};

/// The ghost echo: a reference feature vector and coherence distance function.
#[derive(Clone, Debug)]
pub struct GhostEcho {
    /// The mean feature vector of the legitimate corpus.
    pub reference: Features,
}

impl GhostEcho {
    /// Compute the ghost echo from a set of legitimate feature vectors.
    /// The reference is the element-wise mean.
    pub fn from_features(corpus: &[Features]) -> Self {
        assert!(!corpus.is_empty(), "corpus must not be empty");
        let mut reference = [0.0; FEATURE_DIM];
        for features in corpus {
            for i in 0..FEATURE_DIM {
                reference[i] += features[i];
            }
        }
        let n = corpus.len() as f64;
        for i in 0..FEATURE_DIM {
            reference[i] /= n;
        }
        GhostEcho { reference }
    }

    /// Euclidean distance from the ghost echo.
    /// 0.0 = fully coherent. Higher = more extractive.
    pub fn coherence_distance(&self, features: &Features) -> f64 {
        let mut sum_sq = 0.0;
        for i in 0..FEATURE_DIM {
            let diff = features[i] - self.reference[i];
            sum_sq += diff * diff;
        }
        sum_sq.sqrt()
    }

    /// Coherence score: 1.0 = fully coherent, decays toward 0.0.
    /// Uses exponential decay with characteristic scale.
    pub fn coherence_score(&self, features: &Features, scale: f64) -> f64 {
        let dist = self.coherence_distance(features);
        (-dist / scale).exp()
    }
}
```

- [ ] **Step 3: Run test to verify it passes**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test ghost:: -- --nocapture`

- [ ] **Step 4: Write test — extractive grammars have higher distance**

```rust
#[test]
fn extractive_grammars_farther_from_echo() {
    let legitimate = [
        "grammar @a { type x = one | two | three }",
        "grammar @b { type color = red(shade) | blue  type shade = light | dark }",
        "grammar @c { type op = fold | prism | traversal | lens | iso }",
    ];
    let extractive = [
        include_str!("../fixtures/extractive/no_attribution.conv"),
        include_str!("../fixtures/extractive/coordination_tax.conv"),
    ];

    let legit_features: Vec<Features> = legitimate.iter()
        .map(|s| features::extract_from_source(s))
        .collect();
    let extract_features: Vec<Features> = extractive.iter()
        .map(|s| features::extract_from_source(s))
        .collect();

    let echo = GhostEcho::from_features(&legit_features);

    let avg_legit_dist: f64 = legit_features.iter()
        .map(|f| echo.coherence_distance(f))
        .sum::<f64>() / legit_features.len() as f64;
    let avg_extract_dist: f64 = extract_features.iter()
        .map(|f| echo.coherence_distance(f))
        .sum::<f64>() / extract_features.len() as f64;

    assert!(
        avg_extract_dist > avg_legit_dist,
        "extractive grammars should be farther from echo: legit={:.4} extract={:.4}",
        avg_legit_dist, avg_extract_dist
    );
}
```

- [ ] **Step 5: Run tests, verify pass**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test ghost:: -- --nocapture`

- [ ] **Step 6: Test coherence_score decay**

```rust
#[test]
fn coherence_score_decays_with_distance() {
    let reference = [0.5; FEATURE_DIM];
    let echo = GhostEcho { reference };

    let close = [0.5; FEATURE_DIM]; // identical
    let mid = {
        let mut f = [0.5; FEATURE_DIM];
        f[0] = 0.0;
        f[1] = 1.0;
        f
    };
    let far = [0.0; FEATURE_DIM]; // maximally different

    let scale = 1.0;
    let s_close = echo.coherence_score(&close, scale);
    let s_mid = echo.coherence_score(&mid, scale);
    let s_far = echo.coherence_score(&far, scale);

    assert!(s_close > s_mid, "close={} > mid={}", s_close, s_mid);
    assert!(s_mid > s_far, "mid={} > far={}", s_mid, s_far);
    assert!((s_close - 1.0).abs() < 1e-10, "identical should be 1.0");
}
```

- [ ] **Step 7: Run tests, commit**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test ghost:: -- --nocapture`

```bash
git add src/ghost.rs src/lib.rs
git commit -m "feat: ghost echo — reference vector and coherence distance"
```

---

### Task 4: Extractive Training Data — labeled with shadow-cluster optics

**Files:**
- Create: `fixtures/extractive/training_data.json`
- Create: `src/training.rs` — training data loader for both corpora
- Modify: `src/lib.rs` (add `pub mod training;`)

The shadow cluster labels: extractive grammars get conservative optics that preserve existing structure rather than transforming it. The mapping from §9b pattern → shadow optic:

| Extractive pattern | Shadow optic | Why |
|----|----|----|
| no_attribution | FoldAccumulate (1) | Accumulate into namespace without decomposing — hides the extraction |
| regulation_depletion | Noop (11) | No action — doesn't address the depletion |
| invisible_glue | IsoSettle (9) | Declares convergence — stops looking for the hidden dependency |
| shifting_burden | PrismNarrow (3) | Narrows precision — discards the signal that would reveal the dependency loop |
| coordination_tax | FoldAccumulate (1) | Accumulates overhead without question |

- [ ] **Step 1: Write fixtures/extractive/training_data.json**

This must be generated by running the feature extractor on the 5 extractive fixtures. Write a test that does this and prints the JSON:

```rust
// In training.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::features;

    #[test]
    fn print_extractive_training_data() {
        let fixtures = [
            ("no_attribution.conv", include_str!("../fixtures/extractive/no_attribution.conv"), 1),
            ("regulation_depletion.conv", include_str!("../fixtures/extractive/regulation_depletion.conv"), 11),
            ("invisible_glue.conv", include_str!("../fixtures/extractive/invisible_glue.conv"), 9),
            ("shifting_burden.conv", include_str!("../fixtures/extractive/shifting_burden.conv"), 3),
            ("coordination_tax.conv", include_str!("../fixtures/extractive/coordination_tax.conv"), 1),
        ];
        for (name, source, optic) in &fixtures {
            let f = features::extract_from_source(source);
            eprintln!("  {}: optic={}, features={:?}", name, optic, f);
        }
    }
}
```

Run this test to get the actual feature values, then write `training_data.json` with the real computed values.

- [ ] **Step 2: Write the training data loader**

```rust
//! Training data loading — two corpora merged into one Example set.

use crate::classifier::{Example, INPUT_DIM};
use crate::features::{self, Features, FEATURE_DIM};

/// Load a training corpus from (source, optic_label) pairs.
/// Features are extracted from each source, then placed into
/// the first 16 dims of the 32-dim classifier input.
pub fn examples_from_sources(pairs: &[(&str, usize)]) -> Vec<Example> {
    pairs.iter().map(|(source, label)| {
        let spectral = features::extract_from_source(source);
        let mut input = [0.0; INPUT_DIM];
        for i in 0..FEATURE_DIM.min(INPUT_DIM) {
            input[i] = spectral[i];
        }
        Example {
            features: input,
            label: *label,
        }
    }).collect()
}

/// Merge legitimate and extractive corpora into a single training set.
pub fn merge_corpora(
    legitimate: &[Example],
    extractive: &[Example],
) -> Vec<Example> {
    let mut merged = Vec::with_capacity(legitimate.len() + extractive.len());
    for ex in legitimate {
        merged.push(Example {
            features: ex.features,
            label: ex.label,
        });
    }
    for ex in extractive {
        merged.push(Example {
            features: ex.features,
            label: ex.label,
        });
    }
    merged
}
```

- [ ] **Step 3: Write failing test — merged corpus has both**

```rust
#[test]
fn merged_corpus_has_both_types() {
    let legit = examples_from_sources(&[
        ("grammar @a { type x = one | two }", 0),
    ]);
    let extract = examples_from_sources(&[
        ("grammar @b { type y = a | b }", 11),
    ]);
    let merged = merge_corpora(&legit, &extract);
    assert_eq!(merged.len(), 2);
    assert_eq!(merged[0].label, 0);
    assert_eq!(merged[1].label, 11);
}
```

- [ ] **Step 4: Run tests, verify pass**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test training:: -- --nocapture`

- [ ] **Step 5: Commit**

```bash
git add src/training.rs src/lib.rs fixtures/extractive/training_data.json
git commit -m "feat: two-corpus training data loader with shadow-cluster labels"
```

---

### Task 5: Two-Cluster Training — retrain mirror.weights

**Files:**
- Modify: `src/training.rs` (add `retrain` function)
- Will produce: new `mirror.weights`

- [ ] **Step 1: Write the failing test — extractive inputs route to conservative optics**

```rust
#[test]
fn retrained_model_routes_extractive_to_conservative() {
    let legitimate_sources: Vec<(&str, usize)> = vec![
        ("grammar @a { type x = one | two | three }", 0),           // FoldDecompose
        ("grammar @b { type color = red(shade) | blue  type shade = light | dark }", 6), // LensTransform
        ("grammar @c { type op = fold | prism | traversal | lens | iso }", 4),           // TraversalBreadth
    ];
    let extractive_sources: Vec<(&str, usize)> = vec![
        (include_str!("../fixtures/extractive/no_attribution.conv"), 1),        // FoldAccumulate
        (include_str!("../fixtures/extractive/regulation_depletion.conv"), 11),  // Noop
        (include_str!("../fixtures/extractive/invisible_glue.conv"), 9),         // IsoSettle
        (include_str!("../fixtures/extractive/shifting_burden.conv"), 3),        // PrismNarrow
        (include_str!("../fixtures/extractive/coordination_tax.conv"), 1),       // FoldAccumulate
    ];

    let legit = examples_from_sources(&legitimate_sources);
    let extract = examples_from_sources(&extractive_sources);
    let corpus = merge_corpora(&legit, &extract);

    let config = crate::classifier::TrainConfig {
        epochs: 1000,
        augmentation_factor: 50,
        ..Default::default()
    };
    let (weights, loss, accuracy) = crate::classifier::train(&corpus, &config);

    eprintln!("  loss={:.4} accuracy={:.1}%", loss, accuracy * 100.0);

    // Verify extractive inputs route to their shadow labels
    for (source, expected_label) in &extractive_sources {
        let features = features::extract_from_source(source);
        let mut input = [0.0f64; INPUT_DIM];
        for i in 0..FEATURE_DIM.min(INPUT_DIM) {
            input[i] = features[i];
        }
        let (optic, confidence, _) = crate::classifier::classify(&weights, &input);
        eprintln!(
            "  extractive → {:?} (expected {:?}) conf={:.3}",
            optic,
            crate::classifier::Optic::from_index(*expected_label),
            confidence
        );
    }

    assert!(accuracy > 0.8, "accuracy too low: {:.1}%", accuracy * 100.0);
}
```

- [ ] **Step 2: Run test — expect it to work with the merged corpus**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test retrained_model -- --nocapture`

This test verifies the architecture: the classifier CAN learn to route extractive inputs differently when trained on the merged corpus.

- [ ] **Step 3: Write the full retrain function**

```rust
/// Retrain mirror.weights with both corpora.
/// Returns the trained weights, loss, and accuracy.
pub fn retrain() -> (crate::classifier::Weights, f64, f64) {
    let legitimate: Vec<(&str, usize)> = vec![
        // Load from fixtures/settle/ — the 12 existing seed examples
        // Map their 5 features to the first 5 dims of the 16-feature vector
        // (Task 6 will migrate these to full 16-dim extraction)
    ];

    let extractive: Vec<(&str, usize)> = vec![
        (include_str!("../fixtures/extractive/no_attribution.conv"), 1),
        (include_str!("../fixtures/extractive/regulation_depletion.conv"), 11),
        (include_str!("../fixtures/extractive/invisible_glue.conv"), 9),
        (include_str!("../fixtures/extractive/shifting_burden.conv"), 3),
        (include_str!("../fixtures/extractive/coordination_tax.conv"), 1),
    ];

    let legit = examples_from_sources(&legitimate);
    let extract = examples_from_sources(&extractive);
    let corpus = merge_corpora(&legit, &extract);

    let config = crate::classifier::TrainConfig {
        epochs: 2000,
        augmentation_factor: 50,
        ..Default::default()
    };

    crate::classifier::train(&corpus, &config)
}
```

- [ ] **Step 4: Run the retrain, save weights**

Write a test that retrains and prints the weight bytes size:

```rust
#[test]
fn retrain_and_verify_size() {
    let (weights, loss, accuracy) = retrain();
    let bytes = weights.to_bytes();
    eprintln!("  Retrained: {} bytes, loss={:.4}, accuracy={:.1}%",
        bytes.len(), loss, accuracy * 100.0);
    assert_eq!(bytes.len(), 2892 * 8);
}
```

- [ ] **Step 5: Commit**

```bash
git add src/training.rs
git commit -m "feat: two-corpus retrain — echo/shadow cluster training loop"
```

---

### Task 6: Migrate Existing Training Data to 16-dim Features

**Files:**
- Modify: `src/training.rs` — add loader for existing settle fixtures
- Modify: `fixtures/settle/training_data.json` — regenerate with 16-dim features

The existing 12 seed examples use 5 features. They need to be recomputed with the 16-dim extractor and re-labeled in the 32-dim `INPUT_DIM` format.

- [ ] **Step 1: Write test — load settle fixtures through extractor**

```rust
#[test]
fn settle_fixtures_produce_16_features() {
    let fixture_sources: Vec<&str> = vec![
        include_str!("../fixtures/settle/composition.conv"),
        include_str!("../fixtures/settle/dedup.conv"),
        include_str!("../fixtures/settle/emotion.conv"),
        include_str!("../fixtures/settle/identity.conv"),
        include_str!("../fixtures/settle/layered.conv"),
        include_str!("../fixtures/settle/messy_types.conv"),
        include_str!("../fixtures/settle/redundant_layers.conv"),
        include_str!("../fixtures/settle/sort.conv"),
        include_str!("../fixtures/settle/tension.conv"),
    ];
    for (i, source) in fixture_sources.iter().enumerate() {
        let features = features::extract_from_source(source);
        assert!(
            features.iter().any(|&v| v > 0.0),
            "settle fixture {} produced all-zero features",
            i
        );
        eprintln!("  settle[{}]: {:?}", i, features);
    }
}
```

- [ ] **Step 2: Run test, verify features extract**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test settle_fixtures_produce -- --nocapture`

- [ ] **Step 3: Write the full legitimate corpus loader**

```rust
/// Load the legitimate corpus from settle fixtures.
/// Returns (source, optic_label) pairs ready for examples_from_sources.
pub fn legitimate_corpus() -> Vec<(String, usize)> {
    // The optic labels come from fixtures/settle/training_data.json
    let data: &str = include_str!("../fixtures/settle/training_data.json");
    let entries: Vec<serde_json::Value> = serde_json::from_str(data)
        .expect("training_data.json must be valid JSON");

    entries.iter().filter_map(|entry| {
        let fixture = entry["fixture"].as_str()?;
        let optic = entry["optic"].as_u64()? as usize;
        let path = format!("fixtures/settle/{}", fixture);
        let source = std::fs::read_to_string(&path).ok()?;
        Some((source, optic))
    }).collect()
}
```

Note: This uses serde_json for parsing. If serde_json is not a dependency, use a minimal JSON parser or parse the optic labels from the existing training_data.json manually. Check `Cargo.toml` for available dependencies first.

- [ ] **Step 4: Run tests, commit**

```bash
git add src/training.rs
git commit -m "feat: migrate settle corpus to 16-dim feature extraction"
```

---

### Task 7: Full Retrain and Verification

**Files:**
- Will modify: `mirror.weights` (regenerated)
- Modify: `src/training.rs` — complete the retrain function

- [ ] **Step 1: Write the full integration test**

```rust
#[test]
fn full_retrain_produces_cluster_separation() {
    let (weights, loss, accuracy) = retrain();

    // Check: legitimate grammars get exploratory optics
    let legit_sources = [
        "grammar @a { type x = one | two | three }",
        "grammar @b { type color = red(shade) | blue  type shade = light | dark }",
    ];
    for source in &legit_sources {
        let f = features::extract_from_source(source);
        let mut input = [0.0f64; INPUT_DIM];
        for i in 0..FEATURE_DIM.min(INPUT_DIM) {
            input[i] = f[i];
        }
        let (optic, conf, _) = crate::classifier::classify(&weights, &input);
        eprintln!("  legit {:?} → {:?} ({:.1}%)", &source[..30], optic, conf * 100.0);
    }

    // Check: extractive grammars get conservative optics
    let conservative = [
        crate::classifier::Optic::FoldAccumulate,
        crate::classifier::Optic::PrismNarrow,
        crate::classifier::Optic::IsoSettle,
        crate::classifier::Optic::Noop,
    ];
    let extract_sources = [
        include_str!("../fixtures/extractive/no_attribution.conv"),
        include_str!("../fixtures/extractive/coordination_tax.conv"),
    ];
    for source in &extract_sources {
        let f = features::extract_from_source(source);
        let mut input = [0.0f64; INPUT_DIM];
        for i in 0..FEATURE_DIM.min(INPUT_DIM) {
            input[i] = f[i];
        }
        let (optic, conf, _) = crate::classifier::classify(&weights, &input);
        eprintln!("  extractive → {:?} ({:.1}%)", optic, conf * 100.0);
        assert!(
            conservative.contains(&optic),
            "extractive grammar should route to conservative optic, got {:?}",
            optic
        );
    }

    assert!(accuracy > 0.85, "overall accuracy: {:.1}%", accuracy * 100.0);
    eprintln!("  PASS: loss={:.4} accuracy={:.1}%", loss, accuracy * 100.0);
}
```

- [ ] **Step 2: Run test**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test full_retrain -- --nocapture`

- [ ] **Step 3: If tests pass, save new weights**

Add a helper that writes the retrained weights:

```rust
/// Retrain and save to mirror.weights.
pub fn retrain_and_save(path: &str) -> std::io::Result<(f64, f64)> {
    let (weights, loss, accuracy) = retrain();
    std::fs::write(path, weights.to_bytes())?;
    Ok((loss, accuracy))
}
```

Then run from a test:

```rust
#[test]
#[ignore] // Run explicitly: cargo test save_weights -- --ignored --nocapture
fn save_weights() {
    let (loss, accuracy) = retrain_and_save("mirror.weights")
        .expect("failed to save weights");
    eprintln!("  Saved mirror.weights: loss={:.4} accuracy={:.1}%", loss, accuracy * 100.0);
}
```

- [ ] **Step 4: Run the save**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test save_weights -- --ignored --nocapture`

- [ ] **Step 5: Verify the new weights load and classify**

Run the existing `trained_weights_load` test to verify:

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test trained_weights_load -- --nocapture`

- [ ] **Step 6: Commit**

```bash
git add mirror.weights src/training.rs
git commit -m "feat: retrained mirror.weights with echo/shadow cluster behavior"
```

---

### Task 8: Ghost Echo Integration — coherence check at Abyss entry

**Files:**
- Modify: `src/ghost.rs` — add `DEFAULT_ECHO` computed from legitimate corpus
- Modify: `src/abyss.rs` — add optional coherence gating to settle_loop

This is the final wiring: when the Abyss loop starts, it can check the input grammar's features against the ghost echo and report the coherence score. This does NOT change the loop behavior yet — it reports the score for observability. The actual routing change comes from the weights (Task 7).

- [ ] **Step 1: Add lazy ghost echo from legitimate fixtures**

```rust
// In ghost.rs
use std::sync::OnceLock;

static DEFAULT_ECHO: OnceLock<GhostEcho> = OnceLock::new();

/// Get the default ghost echo, computed from the legitimate settle corpus.
pub fn default_echo() -> &'static GhostEcho {
    DEFAULT_ECHO.get_or_init(|| {
        let sources = [
            include_str!("../fixtures/settle/composition.conv"),
            include_str!("../fixtures/settle/dedup.conv"),
            include_str!("../fixtures/settle/emotion.conv"),
            include_str!("../fixtures/settle/identity.conv"),
            include_str!("../fixtures/settle/layered.conv"),
            include_str!("../fixtures/settle/messy_types.conv"),
            include_str!("../fixtures/settle/redundant_layers.conv"),
            include_str!("../fixtures/settle/sort.conv"),
            include_str!("../fixtures/settle/tension.conv"),
        ];
        let features: Vec<Features> = sources.iter()
            .map(|s| crate::features::extract_from_source(s))
            .collect();
        GhostEcho::from_features(&features)
    })
}
```

- [ ] **Step 2: Write test — default echo computes**

```rust
#[test]
fn default_echo_computes_from_settle_corpus() {
    let echo = default_echo();
    // Should have non-trivial reference
    assert!(echo.reference.iter().any(|&v| v > 0.0));
    // Self-check: settle corpus should be close
    let source = include_str!("../fixtures/settle/identity.conv");
    let f = crate::features::extract_from_source(source);
    let score = echo.coherence_score(&f, 1.0);
    assert!(score > 0.5, "legitimate grammar should have high coherence: {}", score);
}
```

- [ ] **Step 3: Run test, verify pass**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test default_echo -- --nocapture`

- [ ] **Step 4: Write test — extractive grammars have low coherence**

```rust
#[test]
fn extractive_grammars_low_coherence() {
    let echo = default_echo();
    let sources = [
        include_str!("../fixtures/extractive/no_attribution.conv"),
        include_str!("../fixtures/extractive/coordination_tax.conv"),
    ];
    for source in &sources {
        let f = crate::features::extract_from_source(source);
        let score = echo.coherence_score(&f, 1.0);
        eprintln!("  extractive coherence: {:.4}", score);
        assert!(score < 0.8, "extractive grammar should have lower coherence: {}", score);
    }
}
```

- [ ] **Step 5: Run tests, commit**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test ghost:: -- --nocapture`

```bash
git add src/ghost.rs src/abyss.rs
git commit -m "feat: ghost echo integration — coherence score at Abyss entry"
```

---

## Summary

| Task | New files | What it builds | Params |
|------|-----------|---------------|--------|
| 1 | `src/features.rs` | 16-dim feature extractor | 0 |
| 2 | `fixtures/extractive/*.conv` | 5 §9b violation grammars | 0 |
| 3 | `src/ghost.rs` | Ghost echo reference + coherence distance | 0 |
| 4 | `fixtures/extractive/training_data.json`, `src/training.rs` | Two-corpus data loader | 0 |
| 5 | — | Retrain with merged corpus | 2,892 |
| 6 | — | Migrate settle corpus to 16-dim | 0 |
| 7 | `mirror.weights` | Full retrain + verification | 2,892 |
| 8 | — | Ghost echo → Abyss integration | 0 |

Total new learned parameters: **2,892** (same model, retrained on both corpora).
Total new structural code: ~400 lines across 3 files.
The model stays ludicrously small. The formal substrate does the work.
