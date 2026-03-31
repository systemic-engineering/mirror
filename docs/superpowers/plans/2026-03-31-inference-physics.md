# Inference Physics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Domain eigenvalues determine inference temperature — compile-time ceiling, runtime narrowing. TypeRegistry eliminated in the same sweep.

**Architecture:** Coincidence computes `Eigenvalues` (newtype) from graph Laplacians. Conversation wraps them in `DomainSpectrum`, carries them through `Verified`, and delivers them to the ractor `DomainActor` as `InferenceSchedule`. The parser gains `abstract action` and `action name(params) in @target { body }` syntax. `TypeRegistry` is replaced by `Domain` throughout.

**Tech Stack:** Rust, coincidence (Fortran FFI for LAPACK eigendecomposition), conversation (parser, compiler, ractor runtime), BEAM (ETF output)

**Crates:** coincidence (`/Users/alexwolf/dev/projects/coincidence`), conversation (`/Users/alexwolf/dev/projects/conversation`)

**Test commands:**
- coincidence: `cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test`
- conversation: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline`
- clippy: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo clippy --workspace -- -D warnings`
- coverage: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo llvm-cov --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline --fail-under-lines 100 --ignore-filename-regex 'story/|main\.rs'`

**Commit conventions:**
- Commit as `Reed <reed@systemic.engineer>` with GPG key `99060D23EBFAA0D4`
- Phase emojis: `🔴` (failing test), `🟢` (make it pass), `♻️` (refactor), `🔧` (tooling)
- `🔴` must be immediately followed by `🟢`
- Branch: `reed/inference-physics`

---

## File Map

### coincidence crate (`/Users/alexwolf/dev/projects/coincidence`)

| File | Action | Responsibility |
|------|--------|---------------|
| `src/eigenvalue.rs` | **Create** | `Eigenvalues` newtype — sorted, non-negative, from-Laplacian. All spectral math methods. |
| `src/spectral.rs` | **Modify** | `Spectrum` and `Laplacian` produce `Eigenvalues` instead of raw `Vec<f64>`. |
| `src/lib.rs` | **Modify** | Export `eigenvalue` module. |

### conversation crate (`/Users/alexwolf/dev/projects/conversation`)

| File | Action | Responsibility |
|------|--------|---------------|
| `src/model.rs` | **Modify** | `Domain` gains `extends`, `calls`, `ActionBody`, `ParamName`. `DomainSpectrum`, `DomainComplexity` added. `TypeRegistry` field removed. |
| `src/check.rs` | **Modify** | `Verified` gains `spectrum: DomainComplexity`. `verify()` computes eigenvalues during verification. |
| `src/runtime.rs` | **Modify** | `InferenceSchedule` enum. `DomainActor` extracts schedule from `Verified`. Temperature in `decide` handler. |
| `src/property.rs` | **Modify** | `inference_justified` builtin property. `BuiltinProperty::Registry` takes `&Domain` instead of `&TypeRegistry`. |
| `src/parse.rs` | **Modify** | Parse `abstract action`, `action name(params) in @target { body }`, parameter syntactic sugar. |
| `src/resolve.rs` | **Modify** | `Namespace` maps `DomainName → Domain`. `TypeRegistry` struct deleted. `Resolve` uses `Domain`. |
| `src/compile.rs` | **Modify** | `emit_actor_module` takes `&Domain`. Old `&TypeRegistry` entry points removed. |
| `src/ffi.rs` | **Modify** | `compile_grammar_with_phases` uses `Domain` throughout. No `TypeRegistry` import. |
| `src/spectral.rs` | **Modify** | `TypeGraphSpectrum` uses `Eigenvalues` from coincidence. `GrammarSpectrum` updated. |
| `src/lib.rs` | **Modify** | Remove `pub mod resolve` TypeRegistry re-export if any. |

---

## Task 1: Eigenvalues newtype in coincidence

**Files:**
- Create: `/Users/alexwolf/dev/projects/coincidence/src/eigenvalue.rs`
- Modify: `/Users/alexwolf/dev/projects/coincidence/src/spectral.rs`
- Modify: `/Users/alexwolf/dev/projects/coincidence/src/lib.rs`

### Red phase

- [ ] **Step 1: Write failing tests for Eigenvalues newtype**

Create `src/eigenvalue.rs` with test module:

```rust
//! Eigenvalues of a graph Laplacian.
//!
//! Sorted ascending. All non-negative (Laplacian is PSD).
//! Only constructable from eigendecomposition — not from arbitrary floats.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eigenvalues_len() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        assert_eq!(ev.len(), 3);
        assert!(!ev.is_empty());
    }

    #[test]
    fn eigenvalues_empty() {
        let ev = Eigenvalues::from_sorted(vec![]);
        assert!(ev.is_empty());
        assert_eq!(ev.len(), 0);
    }

    #[test]
    fn eigenvalues_as_slice() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 2.0, 4.0]);
        assert_eq!(ev.as_slice(), &[0.0, 1.0, 2.0, 4.0]);
    }

    #[test]
    fn fiedler_value_returns_second_eigenvalue() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 0.5, 2.0]);
        assert_eq!(ev.fiedler_value(), Some(0.5));
    }

    #[test]
    fn fiedler_value_none_for_single() {
        let ev = Eigenvalues::from_sorted(vec![0.0]);
        assert_eq!(ev.fiedler_value(), None);
    }

    #[test]
    fn eigengap_returns_lambda3_minus_lambda2() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 4.0]);
        assert_eq!(ev.eigengap(), Some(3.0));
    }

    #[test]
    fn eigengap_none_for_two() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0]);
        assert_eq!(ev.eigengap(), None);
    }

    #[test]
    fn heat_kernel_at_zero_equals_n() {
        // K(0) = Σ exp(0) = n
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        assert!((ev.heat_kernel(0.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn heat_kernel_decays_with_time() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        let k0 = ev.heat_kernel(0.0);
        let k1 = ev.heat_kernel(1.0);
        let k10 = ev.heat_kernel(10.0);
        assert!(k0 > k1);
        assert!(k1 > k10);
    }

    #[test]
    fn heat_kernel_connected_graph_approaches_one() {
        // Connected graph: λ₁ = 0, rest > 0. K(t→∞) → 1 (only λ₁=0 survives).
        let ev = Eigenvalues::from_sorted(vec![0.0, 2.0, 4.0]);
        let k_large = ev.heat_kernel(100.0);
        assert!((k_large - 1.0).abs() < 1e-10);
    }

    #[test]
    fn spectral_dimension_path_graph() {
        // Path graph of n nodes: d_s ≈ 1 at intermediate scales.
        // Eigenvalues of path(4): 0, 2-√2, 2, 2+√2
        let ev = Eigenvalues::from_sorted(vec![
            0.0,
            2.0 - std::f64::consts::SQRT_2,
            2.0,
            2.0 + std::f64::consts::SQRT_2,
        ]);
        let ds = ev.spectral_dimension(1.0);
        assert!(ds > 0.5 && ds < 2.0, "path d_s should be near 1, got {}", ds);
    }

    #[test]
    fn temperature_at_zero_is_max() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        let t0 = ev.temperature_at(0.0);
        let t1 = ev.temperature_at(1.0);
        assert!(t0 >= t1, "temperature should decrease with diffusion time");
    }

    #[test]
    fn diffusion_time_scales_with_complexity() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        let t_full = ev.diffusion_time(1.0);
        let t_half = ev.diffusion_time(0.5);
        assert!(t_full > t_half, "full complexity should need more diffusion time");
    }

    #[test]
    fn diffusion_time_zero_complexity_is_zero() {
        let ev = Eigenvalues::from_sorted(vec![0.0, 1.0, 3.0]);
        assert!((ev.diffusion_time(0.0)).abs() < 1e-10);
    }
}
```

- [ ] **Step 2: Add module declaration to lib.rs**

In `/Users/alexwolf/dev/projects/coincidence/src/lib.rs`, add:

```rust
pub mod eigenvalue;
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test eigenvalue`

Expected: Compilation errors — `Eigenvalues` struct and methods don't exist yet.

- [ ] **Step 4: Commit red phase**

```bash
cd /Users/alexwolf/dev/projects/coincidence
git checkout -b reed/inference-physics
git add src/eigenvalue.rs src/lib.rs
git commit -m "🔴 eigenvalue: failing tests for Eigenvalues newtype

Sorted, non-negative, from-Laplacian. Heat kernel, spectral dimension,
diffusion time, temperature — all methods tested.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 5: Implement Eigenvalues struct**

In `src/eigenvalue.rs`, above the test module, add:

```rust
/// Eigenvalues of a graph Laplacian.
///
/// Sorted ascending. All non-negative (Laplacian is PSD).
/// Only constructable from eigendecomposition — not from arbitrary floats.
#[derive(Debug, Clone)]
pub struct Eigenvalues(Vec<f64>);

impl Eigenvalues {
    /// Construct from pre-sorted eigenvalues. Only callable within coincidence.
    /// The caller (Laplacian/Spectrum) is responsible for sorting.
    pub(crate) fn from_sorted(values: Vec<f64>) -> Self {
        Eigenvalues(values)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.0
    }

    /// λ₂ — algebraic connectivity. The decide moment.
    /// Returns None if fewer than 2 eigenvalues.
    pub fn fiedler_value(&self) -> Option<f64> {
        self.0.get(1).copied()
    }

    /// λ₃ - λ₂ — sharpness of collapse.
    /// Returns None if fewer than 3 eigenvalues.
    pub fn eigengap(&self) -> Option<f64> {
        match (self.0.get(1), self.0.get(2)) {
            (Some(l2), Some(l3)) => Some(l3 - l2),
            _ => None,
        }
    }

    /// K(t) = Σ exp(-λ_k t) — heat kernel trace at diffusion time t.
    pub fn heat_kernel(&self, t: f64) -> f64 {
        self.0.iter().map(|lambda| (-lambda * t).exp()).sum()
    }

    /// d_s(t) = -2 d(ln K)/d(ln t) — spectral dimension at scale t.
    ///
    /// Computed via finite difference of ln K(t) with respect to ln t.
    /// Uses a small epsilon for numerical differentiation.
    pub fn spectral_dimension(&self, t: f64) -> f64 {
        if t <= 0.0 || self.is_empty() {
            return 0.0;
        }
        let eps = t * 0.01;
        let t_lo = t - eps;
        let t_hi = t + eps;
        let ln_k_lo = self.heat_kernel(t_lo).ln();
        let ln_k_hi = self.heat_kernel(t_hi).ln();
        let ln_t_lo = t_lo.ln();
        let ln_t_hi = t_hi.ln();
        -2.0 * (ln_k_hi - ln_k_lo) / (ln_t_hi - ln_t_lo)
    }

    /// Scale diffusion time by complexity fraction (0.0–1.0).
    ///
    /// complexity_fraction = 1.0 uses the full d_s budget (maximum diffusion time).
    /// complexity_fraction = 0.0 means instant collapse.
    /// The budget is derived from the Fiedler value: t_max = 1/λ₂.
    pub fn diffusion_time(&self, complexity_fraction: f64) -> f64 {
        let fraction = complexity_fraction.clamp(0.0, 1.0);
        match self.fiedler_value() {
            Some(fiedler) if fiedler > 1e-15 => fraction / fiedler,
            _ => 0.0,
        }
    }

    /// Temperature at diffusion time t, derived from heat kernel.
    ///
    /// T(t) = K(t) / n — normalized heat kernel. At t=0, T=1 (maximum).
    /// As t→∞ for connected graphs, T→1/n (minimum).
    pub fn temperature_at(&self, t: f64) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        self.heat_kernel(t) / self.len() as f64
    }
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test eigenvalue`

Expected: All 14 tests pass.

- [ ] **Step 7: Run full test suite**

Run: `cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test`

Expected: All existing tests still pass (no regressions).

- [ ] **Step 8: Commit green phase**

```bash
git add src/eigenvalue.rs
git commit -m "🟢 eigenvalue: Eigenvalues newtype with spectral math methods

heat_kernel, spectral_dimension, fiedler_value, eigengap,
diffusion_time, temperature_at. 14 tests pass.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Wire into Laplacian

- [ ] **Step 9: Write failing test — Laplacian produces Eigenvalues**

In `src/eigenvalue.rs`, add to the test module:

```rust
    #[test]
    fn from_laplacian_path_graph() {
        use crate::spectral::Laplacian;
        // Path graph: 0-1-2
        let verts = vec!["a".into(), "b".into(), "c".into()];
        let edges = vec![(0, 1), (1, 2)];
        let lap = Laplacian::from_adjacency(&verts, &edges);
        let ev = lap.eigenvalues();
        assert_eq!(ev.len(), 3);
        // First eigenvalue of any connected graph is 0
        assert!(ev.as_slice()[0].abs() < 1e-10);
        // All non-negative
        assert!(ev.as_slice().iter().all(|&v| v >= -1e-10));
    }
```

- [ ] **Step 10: Add `eigenvalues()` method to Laplacian**

In `/Users/alexwolf/dev/projects/coincidence/src/spectral.rs`, add a method to `Laplacian`:

```rust
    /// Compute eigenvalues and return as Eigenvalues newtype.
    pub fn eigenvalues(&self) -> crate::eigenvalue::Eigenvalues {
        let eigenvalues = fortran_eigenvalues(&self.matrix, self.vertices.len());
        crate::eigenvalue::Eigenvalues::from_sorted(eigenvalues)
    }
```

This delegates to the existing `fortran_eigenvalues()` FFI function which already returns sorted `Vec<f64>`.

- [ ] **Step 11: Run test to verify it passes**

Run: `cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test eigenvalue`

Expected: All 15 tests pass including `from_laplacian_path_graph`.

- [ ] **Step 12: Commit**

```bash
git add src/eigenvalue.rs src/spectral.rs
git commit -m "🟢 eigenvalue: Laplacian::eigenvalues() returns Eigenvalues newtype

Wire Laplacian's Fortran FFI eigendecomposition into the newtype.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 2: DomainSpectrum and DomainComplexity

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/model.rs`

**Prerequisite:** Task 1 complete (Eigenvalues newtype exists in coincidence).

### Red phase

- [ ] **Step 1: Write failing tests**

In `src/model.rs`, add to the existing test module:

```rust
    #[test]
    fn domain_complexity_trivial_for_no_types() {
        let source = "grammar @empty {}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn domain_complexity_trivial_for_flat_types() {
        // Types with no cross-references → no edges → trivial
        let source = "grammar @flat {\n  type = a | b\n  type op = gt | lt\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(matches!(domain.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn domain_complexity_spectrum_for_referenced_types() {
        // Parameterized type creates edge: pair references color
        let source = "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        match domain.complexity() {
            DomainComplexity::Spectrum(spectrum) => {
                let ev = spectrum.eigenvalues();
                assert!(ev.len() >= 2);
                assert!(ev.fiedler_value().unwrap() > 0.0);
            }
            DomainComplexity::Trivial => panic!("expected Spectrum, got Trivial"),
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib model::tests::domain_complexity`

Expected: Compilation errors — `DomainSpectrum`, `DomainComplexity`, `complexity()` don't exist.

- [ ] **Step 3: Commit red phase**

```bash
cd /Users/alexwolf/dev/projects/conversation
git checkout -b reed/inference-physics
git add src/model.rs
git commit -m "🔴 model: failing tests for DomainSpectrum and DomainComplexity

Trivial for no-type and flat grammars, Spectrum for referenced types.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 4: Add DomainSpectrum and DomainComplexity to model.rs**

Add the types after the existing Domain struct definition:

```rust
/// Spectral analysis of a domain's type graph.
/// Only constructable internally — eigenvalues come from the Laplacian.
pub struct DomainSpectrum {
    eigenvalues: coincidence::eigenvalue::Eigenvalues,
}

impl DomainSpectrum {
    pub(crate) fn new(eigenvalues: coincidence::eigenvalue::Eigenvalues) -> Self {
        DomainSpectrum { eigenvalues }
    }

    pub fn eigenvalues(&self) -> &coincidence::eigenvalue::Eigenvalues {
        &self.eigenvalues
    }
}

/// Grammars with ≤1 type have no type reference graph.
/// Not "spectrum with zeros" — absence of spectrum entirely.
pub enum DomainComplexity {
    /// No type graph to analyze. Instant collapse.
    Trivial,
    /// Real eigenvalues from a real type reference graph.
    Spectrum(DomainSpectrum),
}
```

- [ ] **Step 5: Add `complexity()` method to Domain**

Add a method on `Domain` that computes complexity from its type definitions:

```rust
impl Domain {
    /// Compute the spectral complexity of this domain's type graph.
    ///
    /// Types with parameterized references form edges. If no edges exist,
    /// the domain is Trivial. Otherwise, eigendecomposition of the type
    /// graph's Laplacian produces a DomainSpectrum.
    pub fn complexity(&self) -> DomainComplexity {
        let type_names: Vec<String> = self.types.iter()
            .map(|t| t.name.as_str().to_string())
            .collect();

        if type_names.is_empty() {
            return DomainComplexity::Trivial;
        }

        // Build edges from parameterized variant references
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (i, typedef) in self.types.iter().enumerate() {
            for variant in &typedef.variants {
                for (_param_name, type_ref) in &variant.params {
                    if let Some(j) = type_names.iter().position(|n| n == type_ref.0.as_str()) {
                        if i != j {
                            edges.push((i, j));
                        }
                    }
                }
            }
        }

        if edges.is_empty() {
            return DomainComplexity::Trivial;
        }

        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&type_names, &edges);
        let eigenvalues = laplacian.eigenvalues();
        DomainComplexity::Spectrum(DomainSpectrum::new(eigenvalues))
    }
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib model::tests::domain_complexity`

Expected: All 3 tests pass.

- [ ] **Step 7: Run full test suite and clippy**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline && nix develop -c cargo clippy --workspace -- -D warnings`

Expected: All tests pass, no clippy warnings.

- [ ] **Step 8: Commit green phase**

```bash
git add src/model.rs
git commit -m "🟢 model: DomainSpectrum and DomainComplexity from type graph eigenvalues

Trivial when no type references. Spectrum from Laplacian eigendecomposition
when parameterized variants create cross-type edges.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 3: Verified carries the spectrum

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/check.rs`

**Prerequisite:** Task 2 complete.

### Red phase

- [ ] **Step 1: Write failing tests**

In `src/check.rs`, add to the test module:

```rust
    #[test]
    fn verified_carries_trivial_complexity() {
        let source = "grammar @simple {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = verify(domain).unwrap();
        assert!(matches!(verified.complexity(), DomainComplexity::Trivial));
    }

    #[test]
    fn verified_carries_spectrum_for_referenced_types() {
        let source = "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = verify(domain).unwrap();
        match verified.complexity() {
            DomainComplexity::Spectrum(spectrum) => {
                assert!(spectrum.eigenvalues().fiedler_value().unwrap() > 0.0);
            }
            DomainComplexity::Trivial => panic!("expected Spectrum"),
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib check::tests::verified_carries`

Expected: Compilation errors — `Verified` doesn't have `complexity()` method.

- [ ] **Step 3: Commit red phase**

```bash
git add src/check.rs
git commit -m "🔴 check: failing tests — Verified carries DomainComplexity

Trivial for flat grammars, Spectrum for referenced types.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 4: Modify Verified struct to carry spectrum**

In `src/check.rs`, change `Verified` from:

```rust
pub struct Verified(Domain);
```

To:

```rust
pub struct Verified {
    domain: Domain,
    spectrum: DomainComplexity,
}
```

Update existing methods:

```rust
impl Verified {
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    pub fn into_domain(self) -> Domain {
        self.domain
    }

    pub fn complexity(&self) -> &DomainComplexity {
        &self.spectrum
    }
}
```

- [ ] **Step 5: Update verify() to compute spectrum**

In the `verify()` function, compute complexity before constructing `Verified`:

```rust
pub fn verify(domain: Domain) -> Result<Verified, Violations> {
    // ... existing property checks ...

    let spectrum = domain.complexity();

    Ok(Verified { domain, spectrum })
}
```

- [ ] **Step 6: Fix all existing code that constructs Verified**

All existing tests that construct `Verified(domain)` directly need to use the new struct syntax:

```rust
// Old:
Verified(domain)

// New:
Verified { domain: domain.clone(), spectrum: domain.complexity() }
```

Search for `Verified(` in check.rs and update each occurrence.

- [ ] **Step 7: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib`

Expected: All tests pass including the two new ones.

- [ ] **Step 8: Commit green phase**

```bash
git add src/check.rs
git commit -m "🟢 check: Verified carries DomainComplexity — verification IS measurement

Spectrum computed during verify(). No DomainSpectrum without passing verification.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 4: InferenceSchedule in runtime

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/runtime.rs`

**Prerequisite:** Task 3 complete.

### Red phase

- [ ] **Step 1: Write failing tests**

In `src/runtime.rs`, add to the test module:

```rust
    #[test]
    fn inference_schedule_immediate_for_trivial() {
        let source = "grammar @simple {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        assert!(matches!(schedule, InferenceSchedule::Immediate));
    }

    #[test]
    fn inference_schedule_diffusion_for_spectrum() {
        let source = "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        match &schedule {
            InferenceSchedule::Diffusion(ev) => {
                assert!(ev.fiedler_value().unwrap() > 0.0);
            }
            InferenceSchedule::Immediate => panic!("expected Diffusion"),
        }
    }

    #[test]
    fn schedule_temperature_decreases_with_time() {
        let source = "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        let t_full = schedule.temperature(1.0);
        let t_half = schedule.temperature(0.5);
        let t_zero = schedule.temperature(0.0);
        // More complexity → more exploration → higher temperature
        assert!(t_full >= t_half, "full complexity temp {} should >= half {}", t_full, t_half);
        assert!(t_half >= t_zero, "half complexity temp {} should >= zero {}", t_half, t_zero);
    }

    #[test]
    fn schedule_immediate_always_zero_temperature() {
        let schedule = InferenceSchedule::Immediate;
        assert_eq!(schedule.temperature(0.0), 0.0);
        assert_eq!(schedule.temperature(0.5), 0.0);
        assert_eq!(schedule.temperature(1.0), 0.0);
    }
```

- [ ] **Step 2: Commit red phase**

```bash
git add src/runtime.rs
git commit -m "🔴 runtime: failing tests for InferenceSchedule

Immediate for trivial domains, Diffusion for spectral domains.
Temperature scales with context complexity.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 3: Implement InferenceSchedule**

In `src/runtime.rs`, add:

```rust
use crate::check::Verified;
use crate::model::DomainComplexity;

/// Inference schedule derived from domain eigenvalues.
///
/// Compile-time ceiling. Runtime narrows via context_complexity (0.0–1.0).
pub enum InferenceSchedule {
    /// Trivial domain. No exploration needed. Collapse immediately.
    Immediate,
    /// Heat kernel curve from domain's eigenvalues.
    Diffusion(coincidence::eigenvalue::Eigenvalues),
}

impl InferenceSchedule {
    /// Build schedule from a verified domain.
    pub fn from_verified(verified: &Verified) -> Self {
        match verified.complexity() {
            DomainComplexity::Trivial => InferenceSchedule::Immediate,
            DomainComplexity::Spectrum(spectrum) => {
                InferenceSchedule::Diffusion(spectrum.eigenvalues().clone())
            }
        }
    }

    /// Temperature at a given context complexity (0.0–1.0).
    ///
    /// 0.0 = trivial context (instant collapse).
    /// 1.0 = full domain complexity (maximum exploration).
    pub fn temperature(&self, context_complexity: f64) -> f64 {
        match self {
            InferenceSchedule::Immediate => 0.0,
            InferenceSchedule::Diffusion(eigenvalues) => {
                let t = eigenvalues.diffusion_time(context_complexity);
                eigenvalues.temperature_at(t)
            }
        }
    }
}
```

Note: `Eigenvalues` needs `Clone`. Go back to `coincidence/src/eigenvalue.rs` and verify `#[derive(Debug, Clone)]` is on the struct (it is from Task 1 Step 5).

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib runtime::tests::inference_schedule`

Expected: All 4 tests pass.

- [ ] **Step 5: Commit green phase**

```bash
git add src/runtime.rs
git commit -m "🟢 runtime: InferenceSchedule — Immediate or Diffusion from Verified

Temperature derived from eigenvalues, scaled by context complexity.
Compile-time ceiling, runtime narrowing.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 5: `inference_justified` property

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/property.rs`

**Prerequisite:** Task 2 complete (DomainComplexity exists).

### Red phase

- [ ] **Step 1: Write failing tests**

In `src/property.rs`, add to the test module:

```rust
    #[test]
    fn check_builtin_inference_justified_passes() {
        // Grammar with type references → non-trivial spectrum → justified
        let reg = compile_grammar("grammar @test {\n  type color = red | blue\n  type pair = combo(color)\n}\n");
        let (satisfied, reason) = check_builtin(&reg, "inference_justified").unwrap();
        assert!(satisfied, "should pass: {}", reason);
    }

    #[test]
    fn check_builtin_inference_justified_fails_trivial() {
        // Grammar with no type references → trivial → not justified
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let (satisfied, reason) = check_builtin(&reg, "inference_justified").unwrap();
        assert!(!satisfied, "should fail for trivial domain: {}", reason);
    }
```

- [ ] **Step 2: Commit red phase**

```bash
git add src/property.rs
git commit -m "🔴 property: failing tests for inference_justified builtin

Passes for domains with non-trivial spectrum, fails for trivial domains.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 3: Add `inference_justified` to `lookup_builtin`**

In `src/property.rs`, add the match arm in `lookup_builtin()`:

```rust
"inference_justified" => Some(BuiltinProperty::Registry(inference_justified_check)),
```

And add the check function:

```rust
/// Check that a domain's type graph has non-trivial spectral structure.
///
/// Inference over a trivial domain (no type references) has nothing to
/// explore — the temperature schedule would be meaningless. Domains that
/// declare `requires inference_justified` must have a non-trivial spectrum.
fn inference_justified_check(registry: &TypeRegistry) -> (bool, String) {
    // Build a temporary Domain to compute complexity.
    // Once TypeRegistry is eliminated, this will take &Domain directly.
    let type_names: Vec<String> = registry.type_names().iter().cloned().collect();
    if type_names.is_empty() {
        return (false, "inference_justified: no types declared".into());
    }

    // Check for parameterized variant references (edges in the type graph)
    let mut has_edges = false;
    for type_name in &type_names {
        if let Some(variants) = registry.variants(type_name) {
            for variant in variants {
                if registry.variant_param(type_name, variant).is_some() {
                    has_edges = true;
                    break;
                }
            }
        }
        if has_edges { break; }
    }

    if !has_edges {
        return (false, "inference_justified: type graph has no references — spectrum is trivial".into());
    }

    (true, format!("inference_justified: pass ({} types with cross-references)", type_names.len()))
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib property::tests::check_builtin_inference_justified`

Expected: Both tests pass.

- [ ] **Step 5: Commit green phase**

```bash
git add src/property.rs
git commit -m "🟢 property: inference_justified builtin — trivial domains can't claim inference

Checks that type graph has non-trivial references before allowing
inference temperature schedules.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 6: Parse `abstract action` and action bodies

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/parse.rs`

**Prerequisite:** None (parser changes are independent).

### Red phase — abstract action

- [ ] **Step 1: Write failing test for `abstract action`**

In `src/parse.rs`, add to the test module:

```rust
    #[test]
    fn parse_abstract_action() {
        let source = "grammar @cogito {\n  type = observable\n\n  abstract action observe(observable)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        // Find the abstract action node
        let action = grammar.children().iter()
            .find(|c| c.data().is_decl("abstract-action"))
            .expect("should have abstract-action node");
        assert_eq!(action.data().value, "observe");
        // Should have a parameter child
        let params: Vec<_> = action.children().iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "observable:observable"); // sugar resolved
    }
```

- [ ] **Step 2: Write failing test for action with body and target**

```rust
    #[test]
    fn parse_action_with_body_and_target() {
        let source = "grammar @ai {\n  type = observation\n\n  action decide(observation) in @rust {\n    provider.infer(observation)\n  }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar.children().iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        assert_eq!(action.data().value, "decide");
        // Should have target child
        let target = action.children().iter()
            .find(|c| c.data().name == "target")
            .expect("should have target node");
        assert_eq!(target.data().value, "rust");
        // Should have body child
        let body = action.children().iter()
            .find(|c| c.data().name == "body")
            .expect("should have body node");
        assert!(body.data().value.contains("provider.infer"));
    }
```

- [ ] **Step 3: Write failing test for parameter syntactic sugar**

```rust
    #[test]
    fn parse_action_param_sugar() {
        // action f(x) is equivalent to action f(x: x)
        let source = "grammar @test {\n  type = x | y\n\n  action f(x) in @rust { body }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar.children().iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        let params: Vec<_> = action.children().iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "x:x");
    }

    #[test]
    fn parse_action_param_explicit() {
        // action f(obs: observation) — explicit naming
        let source = "grammar @test {\n  type observation = a | b\n\n  action f(obs: observation) in @rust { body }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let action = grammar.children().iter()
            .find(|c| c.data().is_decl("action-def"))
            .expect("should have action-def node");
        let params: Vec<_> = action.children().iter()
            .filter(|c| c.data().name == "param")
            .collect();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].data().value, "obs:observation");
    }
```

- [ ] **Step 4: Commit red phase**

```bash
git add src/parse.rs
git commit -m "🔴 parse: failing tests for abstract action, action bodies, param sugar

abstract action observe(observable)
action decide(observation) in @rust { body }
action f(x) → action f(x: x)

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 5: Implement abstract action parsing**

In `src/parse.rs`, in the grammar body parsing section (inside `parse_grammar`), add detection for `abstract action` lines. When the parser encounters a line starting with `abstract action`:

```rust
// In the grammar parsing loop, before the existing action detection:
if trimmed.starts_with("abstract action ") || trimmed.starts_with("abstract act ") {
    let rest = trimmed.strip_prefix("abstract action ")
        .or_else(|| trimmed.strip_prefix("abstract act "))
        .unwrap();
    let node = parse_abstract_action(rest, span)?;
    children.push(node);
    continue;
}
```

Add the `parse_abstract_action` function:

```rust
/// Parse `abstract action name(param1, param2: type2)`
///
/// No body. No target. Just a signature.
fn parse_abstract_action(header: &str, span: Span) -> Result<Prism<AstNode>, ParseError> {
    let (name, params) = parse_action_signature(header, span)?;
    let ref_ = Ref::new(sha::hash(&format!("abstract-action:{}", name)), &name);
    let mut children = Vec::new();
    for (param_name, type_name) in &params {
        let param_ref = Ref::new(sha::hash(&format!("param:{}:{}", param_name, type_name)), &format!("{}:{}", param_name, type_name));
        children.push(prism::shard(param_ref, AstNode {
            kind: Kind::Atom,
            name: "param".into(),
            value: format!("{}:{}", param_name, type_name),
            span,
        }));
    }
    Ok(prism::fractal(ref_, AstNode {
        kind: Kind::Decl,
        name: "abstract-action".into(),
        value: name.into(),
        span,
    }, children))
}
```

- [ ] **Step 6: Implement action signature parsing helper**

```rust
/// Parse action signature: `name(param1, param2: type2)`
///
/// Returns (name, Vec<(param_name, type_name)>).
/// Syntactic sugar: bare `x` resolves to `x: x`.
fn parse_action_signature(header: &str, span: Span) -> Result<(String, Vec<(String, String)>), ParseError> {
    let paren_start = header.find('(').ok_or(ParseError {
        message: format!("expected action(params), got: {}", header),
        span: Some(span),
    })?;
    let name = header[..paren_start].trim().to_string();
    let paren_end = header.rfind(')').ok_or(ParseError {
        message: format!("unclosed parenthesis in action: {}", header),
        span: Some(span),
    })?;
    let params_str = &header[paren_start + 1..paren_end];
    let params: Vec<(String, String)> = if params_str.trim().is_empty() {
        Vec::new()
    } else {
        params_str.split(',')
            .map(|p| {
                let p = p.trim();
                if let Some((name, typ)) = p.split_once(':') {
                    (name.trim().to_string(), typ.trim().to_string())
                } else {
                    // Syntactic sugar: bare name means name: name
                    (p.to_string(), p.to_string())
                }
            })
            .collect()
    };
    Ok((name, params))
}
```

- [ ] **Step 7: Implement action with body and target**

Modify the existing `parse_action_def` function (or add a new variant) to detect `in @target { body }` at the end of the action header:

```rust
/// Parse `action name(params) in @target { body }` or `action name { fields }`
///
/// If `in @target` is present, parse body between braces.
/// Otherwise, fall through to existing field-based action parsing.
```

After parsing the action signature, check for `in @`:

```rust
let after_parens = &header[paren_end + 1..].trim();
if after_parens.starts_with("in @") {
    let target = after_parens.strip_prefix("in @").unwrap()
        .split_whitespace().next().unwrap_or("").to_string();
    // Collect body lines until closing brace
    let mut body_lines = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed == "}" {
            break;
        }
        body_lines.push(line.to_string());
    }
    let body_text = body_lines.join("\n");
    // Add target and body as child nodes
    // ... (create AST nodes for target and body)
}
```

The exact integration with the existing parser requires careful study of how `parse_action_def` currently works with its `lines` iterator. The key changes:

1. Parse `action name(params) in @target {` as the action header
2. Collect body lines until `}`
3. Emit child nodes: `target` (Atom) and `body` (Atom with the raw text)
4. Emit `param` children from the parsed signature

- [ ] **Step 8: Run tests to verify they pass**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib parse::tests::parse_abstract_action parse::tests::parse_action_with_body parse::tests::parse_action_param`

Expected: All 4 tests pass.

- [ ] **Step 9: Run full test suite**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline`

Expected: All existing tests still pass. New action syntax doesn't break old `act` syntax.

- [ ] **Step 10: Commit green phase**

```bash
git add src/parse.rs
git commit -m "🟢 parse: abstract action, action bodies with targets, param sugar

abstract action name(params) — signature only
action name(params) in @target { body } — function with implementation
action f(x) sugar for action f(x: x)

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 7: Domain absorbs TypeRegistry — extends and calls

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/model.rs`

**Prerequisite:** Task 6 complete (parser can emit action bodies).

### Red phase

- [ ] **Step 1: Write failing tests for new Domain fields**

In `src/model.rs`, add to tests:

```rust
    #[test]
    fn domain_has_extends() {
        let source = "grammar @sub {\n  type = a\n\n  extends @base\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(domain.extends().iter().any(|d| d.as_str() == "base"));
    }

    #[test]
    fn domain_has_action_calls() {
        let source = "grammar @ai {\n  type = observation\n\n  act decide {\n    @tools.exec(observation)\n  }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        assert!(!domain.calls().is_empty());
        assert_eq!(domain.calls()[0].target.as_str(), "tools");
        assert_eq!(domain.calls()[0].action.as_str(), "exec");
    }

    #[test]
    fn domain_content_addressed() {
        let source = "grammar @test {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast.children().iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let d1 = Domain::from_grammar(grammar).unwrap();
        let d2 = Domain::from_grammar(grammar).unwrap();
        assert_eq!(d1.content_oid(), d2.content_oid());
    }
```

- [ ] **Step 2: Commit red phase**

```bash
git add src/model.rs
git commit -m "🔴 model: failing tests for extends, calls, content addressing on Domain

Domain absorbs TypeRegistry's remaining surface.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Green phase

- [ ] **Step 3: Add extends and calls fields to Domain**

In `src/model.rs`, update the `Domain` struct:

```rust
pub struct Domain {
    pub name: DomainName,
    pub types: Vec<TypeDef>,
    pub actions: Vec<Action>,
    pub lenses: Vec<Lens>,
    pub extends: Vec<DomainName>,
    pub calls: Vec<ActionCall>,
    pub properties: Properties,
    pub(crate) registry: Option<TypeRegistry>,
}
```

Add accessor methods:

```rust
impl Domain {
    pub fn extends(&self) -> &[DomainName] { &self.extends }
    pub fn calls(&self) -> &[ActionCall] { &self.calls }
}
```

- [ ] **Step 4: Extract extends and calls in from_grammar**

Update `Domain::from_grammar()` / `from_grammar_with_lenses()` to walk the AST for `extends` declarations and action call nodes:

```rust
// In from_grammar_with_lenses, after existing type/action extraction:

let mut extends = Vec::new();
let mut calls = Vec::new();

for child in node.children() {
    let data = child.data();
    if data.is_decl("extends") {
        extends.push(DomainName::new(&data.value));
    }
}

// Extract calls from action definitions
for child in node.children() {
    let data = child.data();
    if data.is_decl("action-def") || data.is_decl("act") {
        for action_child in child.children() {
            if action_child.data().name == "action-call" {
                let call_data = &action_child.data().value;
                // Parse "domain.action" format
                if let Some((domain, action)) = call_data.split_once('.') {
                    calls.push(ActionCall {
                        target: DomainName::new(domain),
                        action: ActionName::new(action),
                        args: Vec::new(), // TODO: extract from children
                    });
                }
            }
        }
    }
}
```

- [ ] **Step 5: Add ContentAddressed impl for Domain**

```rust
use fragmentation::encoding::Encode;
use crate::{ContentAddressed, Oid};

impl Encode for Domain {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"domain:");
        bytes.extend_from_slice(self.name.as_str().as_bytes());
        for typedef in &self.types {
            bytes.extend_from_slice(b":type:");
            bytes.extend_from_slice(typedef.name.as_str().as_bytes());
            for variant in &typedef.variants {
                bytes.extend_from_slice(b":");
                bytes.extend_from_slice(variant.name.as_str().as_bytes());
            }
        }
        for action in &self.actions {
            bytes.extend_from_slice(b":action:");
            bytes.extend_from_slice(action.name.as_str().as_bytes());
        }
        bytes
    }
}

domain_oid!(/// Content address for domains.
pub DomainOid);

impl ContentAddressed for Domain {
    type Oid = DomainOid;
    fn content_oid(&self) -> DomainOid {
        DomainOid::from(Oid::hash(&self.encode()))
    }
}
```

- [ ] **Step 6: Run tests and commit**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib model::tests`

```bash
git add src/model.rs
git commit -m "🟢 model: Domain gains extends, calls, ContentAddressed

Domain now carries all data previously exclusive to TypeRegistry.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 8: Namespace migration — Domain replaces TypeRegistry

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/src/resolve.rs`
- Modify: `/Users/alexwolf/dev/projects/conversation/src/property.rs`
- Modify: `/Users/alexwolf/dev/projects/conversation/src/compile.rs`
- Modify: `/Users/alexwolf/dev/projects/conversation/src/ffi.rs`

**Prerequisite:** Task 7 complete.

This is the largest task. It's a mechanical replacement: every place that uses `TypeRegistry` switches to `Domain`. Because `Domain` already carries an `Option<TypeRegistry>` internally, this can be done incrementally — first add `Domain`-based APIs alongside `TypeRegistry` ones, then remove the `TypeRegistry` ones.

### Phase 1: Namespace

- [ ] **Step 1: Add `register_domain` to Namespace alongside `register_grammar`**

In `src/resolve.rs`, add to `Namespace`:

```rust
pub fn register_domain(&mut self, name: &str, domain: Domain) {
    // Store domain and extract registry for backward compat
    if let Some(registry) = domain.registry.as_ref() {
        self.register_grammar(name, registry.clone());
    }
    // TODO: store domain directly once grammar_store switches
}
```

- [ ] **Step 2: Update callers to use register_domain where possible**

Search for `register_grammar(` across the codebase. For each call site that has a `Domain` available, switch to `register_domain`.

- [ ] **Step 3: Run tests to verify nothing broke**

Run full test suite. All tests should pass — this is additive only.

- [ ] **Step 4: Commit**

```bash
git commit -m "♻️ resolve: add register_domain alongside register_grammar

Incremental migration — Domain wraps TypeRegistry internally.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Phase 2: Property checks take Domain

- [ ] **Step 5: Change BuiltinProperty::Registry to take &Domain**

In `src/property.rs`, change:

```rust
// Old:
Registry(fn(&TypeRegistry) -> (bool, String)),

// New:
Registry(fn(&Domain) -> (bool, String)),
```

Update all existing registry check functions (`exhaustive_check`, `connected_check`, `bipartite_check`, `inference_justified_check`) to accept `&Domain` and access type data through Domain's API instead of TypeRegistry's.

- [ ] **Step 6: Update callers of check functions**

The `check_property_block_with_overrides` function in property.rs looks up the registry from the namespace. Update it to look up the Domain instead.

- [ ] **Step 7: Run tests and commit**

```bash
git commit -m "♻️ property: Registry checks take &Domain instead of &TypeRegistry

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Phase 3: Compile takes Domain

- [ ] **Step 8: Remove `emit_actor_module` that takes &TypeRegistry**

The Domain-based entry points (`emit_actor_module_from_domain`, `emit_actor_module_from_verified`) already exist from the prior refactor agent. Remove the old `emit_actor_module(&TypeRegistry, ...)` signature.

Update `emit_actor_module_from_domain` to extract what it needs from Domain directly instead of calling `domain.registry()`.

- [ ] **Step 9: Update ffi.rs**

`compile_grammar_with_phases` currently builds a Domain then calls `domain.registry()`. Once compile.rs works with `&Domain`, ffi.rs can stop touching TypeRegistry entirely.

- [ ] **Step 10: Run tests and commit**

```bash
git commit -m "♻️ compile/ffi: emit_actor_module takes &Domain directly

TypeRegistry no longer used in compilation path.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

### Phase 4: Delete TypeRegistry

- [ ] **Step 11: Remove TypeRegistry struct from resolve.rs**

Delete the `TypeRegistry` struct, its `compile()` method, and all methods. Update `Namespace`'s `grammar_store` to use `Domain` instead.

This step will create many compilation errors. Fix them all — every remaining reference to `TypeRegistry` must be replaced with `Domain` equivalents.

- [ ] **Step 12: Remove `registry` field from Domain**

In `src/model.rs`, remove `pub(crate) registry: Option<TypeRegistry>` from Domain and the `registry()` accessor. Domain no longer wraps TypeRegistry — it IS the data model.

- [ ] **Step 13: Run full test suite, clippy, and coverage**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline && nix develop -c cargo clippy --workspace -- -D warnings && nix develop -c cargo llvm-cov --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline --fail-under-lines 100 --ignore-filename-regex 'story/|main\.rs'`

Expected: All tests pass, no clippy warnings, 100% line coverage.

- [ ] **Step 14: Commit**

```bash
git commit -m "♻️ TypeRegistry deleted — Domain is the sole data model

Namespace stores Domain. Compile takes Domain. Properties check Domain.
TypeRegistry struct and all methods removed from resolve.rs.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 9: Integration litmus test

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/tests/runtime_test.rs` (or create if not present)

**Prerequisite:** All previous tasks complete.

- [ ] **Step 1: Write the end-to-end test**

```rust
//! Integration test: .conv source → parse → Domain → verify with spectrum
//! → actor boots with schedule → decide uses temperature from eigenvalues.

use conversation::parse::Parse;
use conversation::model::Domain;
use conversation::check::verify;
use conversation::runtime::{InferenceSchedule, RactorRuntime, Runtime, Args};
use conversation::Vector;

#[tokio::test]
async fn inference_physics_litmus() {
    // Grammar with type references → non-trivial spectrum
    let source = "grammar @test {\n  type color = red | blue\n  type pair = combo(color)\n  requires inference_justified\n}\n";

    // Parse
    let ast = Parse.trace(source.to_string()).unwrap();
    let grammar = ast.children().iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();

    // Domain
    let domain = Domain::from_grammar(grammar).unwrap();
    assert_eq!(domain.name().as_str(), "test");

    // Verify — spectrum computed here
    let verified = verify(domain).unwrap();
    let spectrum = verified.complexity();
    assert!(matches!(spectrum, conversation::model::DomainComplexity::Spectrum(_)));

    // Schedule from verified
    let schedule = InferenceSchedule::from_verified(&verified);
    match &schedule {
        InferenceSchedule::Diffusion(ev) => {
            // Fiedler value exists (connected graph)
            assert!(ev.fiedler_value().unwrap() > 0.0);
            // Temperature at full complexity is positive
            let temp = schedule.temperature(1.0);
            assert!(temp > 0.0);
            // Temperature at zero complexity is zero-ish
            let temp_zero = schedule.temperature(0.0);
            assert!(temp_zero < temp);
        }
        InferenceSchedule::Immediate => panic!("expected Diffusion schedule"),
    }

    // Runtime: register and dispatch
    let mut runtime = RactorRuntime::new();
    runtime.register(&verified).await.unwrap();
    let response = runtime.dispatch(
        &domain.name(),
        &"decide".into(),
        Args::Empty,
    ).await;
    // Dispatch may return error (no handler for "decide" yet) — that's fine.
    // The point is: the actor booted with the schedule.
    runtime.shutdown(&domain.name()).await.unwrap();
}
```

- [ ] **Step 2: Run the integration test**

Run: `cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --test runtime_test inference_physics_litmus`

Expected: Test passes. The actor boots with the spectral-derived schedule.

- [ ] **Step 3: Commit**

```bash
git add tests/runtime_test.rs
git commit -m "🟢 integration: inference physics litmus — spectrum to schedule to actor

.conv source → parse → Domain → verify with spectrum → actor boots
with InferenceSchedule::Diffusion → temperature derived from eigenvalues.

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 10: Update @ai grammar in garden

**Files:**
- Modify: `/Users/reed/dev/projects/garden/public/@ai/ai.conv`

**Prerequisite:** Task 6 complete (parser supports new action syntax).

- [ ] **Step 1: Add schedule type and inference_justified property**

Update `ai.conv`:

```
grammar @ai {
  type = observation | decision | action | model

  type observation = ref | embedding | signal
  type decision = vector
  type action = generate | route | embed | stop
  type model = local | remote | hybrid
  type schedule = immediate | diffusion

  requires inference_justified

  action decide(observation, schedule) in @rust {
    // spectral-derived temperature, provider call
  }

  action generate(decision) in @rust {
    // token generation with temperature from schedule
  }

  action embed(observation) in @rust {
    // embedding computation
  }

  action route(observation, model) in @beam {
    // model selection dispatch
  }
}
```

- [ ] **Step 2: Update test section**

Add tests for the new types:

```
test "schedule types" {
  @ai.schedule has immediate
  @ai.schedule has diffusion
}
```

- [ ] **Step 3: Commit**

```bash
cd /Users/reed/dev/projects/garden
git add public/@ai/ai.conv
git commit -m "🔧 @ai: add schedule type, inference_justified property, action bodies

Co-Authored-By: Reed <reed@systemic.engineer>"
```

---

## Task 11: Milestone commit and coverage gate

**Files:** None new — this is a verification step.

- [ ] **Step 1: Run full test suite across both crates**

```bash
cd /Users/alexwolf/dev/projects/coincidence && nix develop -c cargo test
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo test --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline
```

- [ ] **Step 2: Run clippy**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo clippy --workspace -- -D warnings
```

- [ ] **Step 3: Run coverage gate**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo llvm-cov --package conversation --lib --test compile_test --test grammar_test --test repo_test --test property_pipeline --fail-under-lines 100 --ignore-filename-regex 'story/|main\.rs'
```

- [ ] **Step 4: Milestone commit**

```bash
cd /Users/alexwolf/dev/projects/conversation
git commit --allow-empty -m "🔧 milestone: inference physics — domain eigenvalues determine temperature

Eigenvalues newtype (coincidence). DomainSpectrum + DomainComplexity.
Verified carries spectrum. InferenceSchedule in runtime.
inference_justified property. abstract action + action bodies.
TypeRegistry eliminated. Domain is the sole data model.

Co-Authored-By: Reed <reed@systemic.engineer>"
```
