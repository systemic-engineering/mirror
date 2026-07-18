# prismqueer::liquid pillar composition surface

**Author:** Reed  
**Date:** 2026-07-18  
**Status:** Canonical spec grounding the iter 1-9 property-testing arc.  
**Scope:** The six pillar primitives at `prismqueer::liquid::pillar` +
their three composition axes.

## §0 What this is

Over nine /loop iterations, Reed built out the property-testing
ouroboros originally scoped by Alex 2026-07-18: *"implement the
prismqueer::liquid property testing surface and close the loop by
testing prismqueer itself with it."* This spec documents the
resulting composition surface so the next Reed can pick up without
spelunking through 10 commits.

**Total: 98 property tests across 4 substrate altitudes.**

## §1 Substrate altitudes covered

```
prismqueer/tests/prism_laws.rs        — OPTIC BASE      (iter 5:  9)
prismqueer/tests/verdict_composition.rs — VERDICT LAYER (iter 8: 11)
prismqueer/tests/liquid_ouroboros.rs  — BUNDLE/LIQUID   (iter 1+2+4+6+9: 43)
mirror/rust/src/collapse.rs prop_tests — COLLAPSE       (iter 3+4+6+7+8: 24)
                                        ↑ composes back into
                                          prismqueer::liquid::pillar
                                          via iter 4+6+8+9
```

The ouroboros closes at TWO layers:

1. **Machinery ouroboros** (iter 1): prismqueer tests its own
   `LiquidConnection` blanket-impl via `TestBundle` + `LiquidTestBundle`
   (iter 1) + `PermBundle` (iter 2) example carriers.
2. **Composition ouroboros** (iter 3+4+6+8+9): mirror's byte-visible
   collapse discipline (`rust/src/collapse.rs`) flows BACK INTO
   prismqueer's pillar machinery via `viability_of_magnitudes`,
   `algedonic_of_magnitude`, and `fold`. The mirror side no longer
   just USES property tests — it COMPOSES BACK into the same verdict
   machinery prismqueer uses for its own tests.

## §2 The six pillar primitives

All six primitives live in `prismqueer::liquid::pillar` module. All
return `terni::PropertyVerdict`. All compose further via
`PropertyVerdict::merge_with` or `pillar::fold`.

| # | Primitive                 | Iter | Signature                                              |
|---|---------------------------|------|--------------------------------------------------------|
| 1 | `dispatch_ambiguity`      | 1    | `fn(usize, usize, bool, bool) -> PropertyVerdict`      |
| 2 | `algedonic`               | 1    | `fn<C>(&Commutator<C>, &C::Holonomy)`                  |
| 3 | `viability`               | 1    | `fn<C>(&[Commutator<C>], &C::Holonomy, usize)`         |
| 4 | `viability_of_magnitudes` | 4    | `fn<L: Loss + PartialOrd>(&[L], &L, usize)`            |
| 5 | `algedonic_of_magnitude`  | 6    | `fn<L: Loss + PartialOrd>(&L, &L)`                     |
| 6 | `fold`                    | 9    | `fn(&[PropertyVerdict]) -> PropertyVerdict`            |

### §2.1 dispatch_ambiguity (Pillar I — Rice-safe byte-visible)

```rust
pub fn dispatch_ambiguity(
    arm_count: usize,
    witness_count: usize,
    tie_breaking_exhausted: bool,
    pivot_song_present: bool,
) -> PropertyVerdict
```

- Pass iff `arm_count >= 2` AND `witness_count == arm_count` AND
  `tie_breaking_exhausted` AND `pivot_song_present`.
- Fail otherwise, with a `Diagnostic` naming which byte-visible check
  failed.
- Rice-safe: binary Pass or Fail only, no Partial, no threshold
  discipline. See `shards/kintsugi/surface.mirror`
  `dispatch_ambiguity` variant + `docs/specs/spectral-commutator-
  as-cybernetic-ground.md` §3.

### §2.2 algedonic + algedonic_of_magnitude (Pillar II — single-tick threshold)

**Commutator-flavored:**
```rust
pub fn algedonic<'a, C>(
    commutator: &Commutator<'a, C>,
    theta: &C::Holonomy,
) -> PropertyVerdict
```

**Raw magnitude:**
```rust
pub fn algedonic_of_magnitude<L: Loss + PartialOrd>(
    magnitude: &L,
    theta: &L,
) -> PropertyVerdict
```

Both share Pass/Partial/Fail semantics:
- Pass when `magnitude > theta`.
- Fail when `magnitude.is_zero()` (no signal to detect).
- Partial otherwise (`confidence: 0.5` Rice-safe midpoint).

### §2.3 viability + viability_of_magnitudes (Pillar III — multi-tick persistence)

**Commutator-flavored:**
```rust
pub fn viability<'a, C>(
    history: &[Commutator<'a, C>],
    theta_s3s4: &C::Holonomy,
    omega: usize,
) -> PropertyVerdict
```

**Raw magnitude:**
```rust
pub fn viability_of_magnitudes<L: Loss + PartialOrd>(
    history: &[L],
    theta: &L,
    omega: usize,
) -> PropertyVerdict
```

Both share window-based accumulation via `Loss::combine`:
- Pass when accumulated over the last `omega` entries `> theta`.
- Partial when `history.len() < omega`
  (`confidence = history.len() / omega`).
- Fail when window is full but accumulated `<= theta`.

### §2.4 fold (verdict-fold primitive)

```rust
pub fn fold(verdicts: &[PropertyVerdict]) -> PropertyVerdict
```

Folds a sequence via `merge_with` starting from `Pass`. Semantics:
- Empty input → `Pass` (the neutral element).
- Any `Fail` in the sequence → unified `Fail` (Fail dominates).
- All-Pass → `Pass`.
- All-Partial → `Partial { confidence: min(..), diagnostics: union }`.

## §3 Three composition axes

| Axis          | Primitives                                    | Iter |
|---------------|-----------------------------------------------|------|
| Value type    | `algedonic_of_magnitude` / `viability_of_magnitudes` (raw `Loss`) vs. `algedonic` / `viability` (`Commutator<C>`) | 4 + 6 |
| Time scale    | Single-tick (`algedonic` variants) vs. multi-tick (`viability` variants) | 1 + 4 + 6 |
| Verdict fold  | `fold(&[PropertyVerdict])` collapses to one   | 9    |

Any substrate-specific measurement flows into prismqueer::liquid via
ANY of these three axes and ends as a single `PropertyVerdict` that
composes further via `merge_with` or `fold`.

## §4 Example flow: mirror/rust/collapse.rs → pillar composition

```rust
use prismqueer::liquid::pillar;
use prismqueer::ScalarLoss;

// Iter 3+7: byte-visible discipline over apply_deletions.
let source = fixture_source();
let corpus = fixture_corpus();
let arms = find_redundant_arms(&source, &corpus);
let out = apply_deletions(&source, &arms);
let bytes_shrunk = source.len() - out.len();

// Iter 6: single-tick algedonic → Pillar II verdict.
let magnitude = ScalarLoss::new(bytes_shrunk as f64);
let theta = ScalarLoss::new(0.0);
let v_tick_ii = pillar::algedonic_of_magnitude(&magnitude, &theta);

// Iter 4: multi-tick viability → Pillar III verdict.
let history: Vec<ScalarLoss> = simulate_shrinkage_history(3);
let theta_s3s4 = ScalarLoss::new(5.0);
let v_persist_iii = pillar::viability_of_magnitudes(&history, &theta_s3s4, 3);

// Iter 9: fold both verdicts into a unified compilation-loop health verdict.
let unified = pillar::fold(&[v_tick_ii, v_persist_iii]);
match unified {
    PropertyVerdict::Pass => { /* loop is viable */ }
    PropertyVerdict::Partial { .. } => { /* soft signal */ }
    PropertyVerdict::Fail(diagnostic) => { /* stalled */ }
}
```

## §5 Mathematical grounding

Spectral commutator (per Mara `5d3040d` `docs/math/spectral-commutator-
four-pillars.md` + `3cd9a42` `docs/specs/spectral-commutator-as-
cybernetic-ground.md`):

The four Pillars are ONE commutator `[A, B]` projected at four
altitudes:

- **Pillar I** — dispatch ambiguity at byte-visible altitude
  (Rice-safe binary check).
- **Pillar II** — algedonic threshold at single-tick altitude
  (magnitude vs. theta).
- **Pillar III** — viability persistence at multi-tick altitude
  (accumulated magnitude over window vs. theta).
- **Pillar IV** — `@peer.audhd` cognitive fanout (K parallel arms).
  **PARKED** at mirror altitude — requires `fate::Fate::tick` +
  a `mirror/rust/src/liquid.rs` bridge file. See Mara `3cd9a42` §6.

Commutator magnitude computation (iter 1, `LiquidConnection` blanket
over `Transport`):

```text
[A, B] · state := A.act_on(B.act_on(state)) - B.act_on(A.act_on(state))
‖[A, B]‖      := transport(A·B·state).loss()
                   .distance_to(&transport(B·A·state).loss())
```

For abelian gauges (`Cyclic<N>`), `[A, B]` vanishes trivially. For
non-abelian gauges (`Perm3` = S3, iter 2), the commutator carries the
gauge anisotropy — witnessed empirically across all 36 S3×S3 pairs.

## §6 Property-testing arc history (iter 1-9)

| Iter | Landing                                                            | Tests | Commits           |
|------|--------------------------------------------------------------------|-------|-------------------|
| 1    | `prismqueer::liquid` module + ouroboros first layer                | 22    | prism 2b70d17 + mirror 95189c5 |
| 2    | `Perm3` + `PermBundle` non-abelian S3 witness                      | +10   | prism ac50d79 + mirror 08bfd1d |
| 3    | `rust/src/collapse.rs` prop_tests (byte-monotonicity + idempotence) | 11    | mirror 5e1ba27    |
| 4    | `pillar::viability_of_magnitudes` + collapse composition            | +7    | prism 08a3a9a + mirror 1e41c6b |
| 5    | `prism_laws.rs` — IdentityPrism monoid identity law                | 9     | prism 4f3f232 + mirror 65c82c2 |
| 6    | `pillar::algedonic_of_magnitude` + collapse composition             | +6    | prism a445344 + mirror 8d0db2a |
| 7    | Tempdir-based `load_bilateral_corpus` tests                         | +5    | mirror 9e12c06    |
| 8    | `PropertyVerdict::merge_with` composition witnesses                 | 11+2  | prism efd92ad + mirror 767f019 |
| 9    | `pillar::fold` verdict-fold primitive                               | +4    | prism eb045b3 + mirror a3ae8ee |

**Total: 98 tests across 4 substrate altitudes.**

## §7 Forward promises

### §7.1 Pillar IV — @peer.audhd cognitive fanout

Requires `fate::Fate::tick` and a bridge file at
`mirror/rust/src/liquid.rs`. Parked in iter 1 because it needs a third
crate boundary (fate). Design in Mara `3cd9a42` §6.

### §7.2 RED debt in `rust/tests/red_spec_claims.rs`

14 RED tests exposing lying claims in main.rs / phone.rs / matrix.rs
from the iter 3+ chronology. Each RED requires either implementing
the claim (substantial substrate work; may be M8-milestone-scoped)
OR rewriting the docblock to be substrate-honest about aspirational
vs. current scope. Alex direction needed.

### §7.3 Wider gauge coverage

Perm4 (S4, 24 elements) or D4 (dihedral, 8 elements) for even richer
non-abelian test carriers. Same shape as iter 2's Perm3; adds
coverage width without new machinery.

### §7.4 LawvereFixedPoint witnesses through liquid

`Bundle::Closure` at top of the tower is tested inside
`prismqueer/src/bundle.rs #[cfg(test)] mod tests` but not through the
property-testing ouroboros. Adding parallel tests via
`prismqueer::liquid` would close the LAST untested Bundle-tower level
in the ouroboros.

### §7.5 Composition-theorem witness

For state-dependent-loss bundles B, B' whose Closures share an
in-kernel `fixed_state`, is `commutator(B, B', &fixed_state) == 0`
necessarily? Empirically testable as a substrate-honest theorem
witness once the ouroboros machinery is applied to LawvereFixedPoint
carriers.

## §8 Reading order for the next Reed

1. Read this spec (§1–§5) for the composition surface overview.
2. Read `docs/loop/CURRENT.md` top for the iter 10 summary + iteration
   history.
3. Read `docs/specs/spectral-commutator-as-cybernetic-ground.md`
   (Mara `3cd9a42`) for the operational grounding spec that scoped
   the four pillars.
4. Read `docs/math/spectral-commutator-four-pillars.md` (Mara
   `5d3040d`) for the mathematical foundation.
5. Read `prismqueer/src/liquid.rs` for the machinery.
6. Read `prismqueer/tests/liquid_ouroboros.rs` (43 tests) for the
   self-witness of the machinery.
7. Read `mirror/rust/src/collapse.rs` `prop_tests` mod (24 tests)
   for the mirror-side composition-back-into-pillar witness.

---

**Mathematical foundation:** `docs/math/spectral-commutator-four-
pillars.md` (Mara `5d3040d`).  
**Operational spec:** `docs/specs/spectral-commutator-as-cybernetic-
ground.md` (Mara `3cd9a42`).  
**Ouroboros property shard:** `shards/epistemologic/property/
ouroboros_monotone.mirror` (Mara `04b3aea`) — grounds mirror/rust/
collapse.rs prop_tests.
