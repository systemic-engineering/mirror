# 16 — Sub-Turing Geometric Compiler Floor: rust/ four-crate decomposition + magic gauge + (A, H, D) triple

## Status: SPEC LANDED + METRIC ADJUDICATED 2026-07-25 (Mara canonical spec + math foundation + metric revision this arc; Reed foreground rust/build.rs retirement + rust/singularity/ poof co-tick; Phase 2+ pending)

**Post-adjudication milestone-defining revisions (Alex 2026-07-25 verbatim):**

- **[ALEX-Q1] RESOLVED**: metric is future-light-cone angle Θ, not choice-count. Verbatim: *"Choice-count is probably the wrong metric. I think it might be the angle of the future light cone."* Math §3 + spec §4 revised in place; four prior candidates become computational realizations at four altitudes.
- **[ALEX-Q4] RESOLVED**: magic.rs binds BOTH singularity senses from v0.1. Verbatim: *"magic.rs binds both. We're gonna do this proper. That's what I meant with the properties earlier. I want all of it impeccable, linked to the formalizing math docs, and the full statespace covered by the properties."* Spec new §14 Impeccability Discipline lands full state-space coverage as milestone-defining, not v0.1-only.
- **[ALEX-Q3] REAFFIRMED**: `rust/singularity/` scaffold poofs entirely. Reed executed via commit `f2f3b3a` (−350 LOC, tests 300/300 green).
- **@mirror/matrix mint ratified**. Alex verbatim: *"Sounds like @mirror/matrix wants to exist then"*. Companion species-decl `shards/mirror/matrix.mirror` landed this arc as Landing 4.

**Alex 2026-07-25 in-transcript (three verbatim moments closing the Void — Trauma essay Q.E.D. into executable substrate):**

> "singularity is the backing for the paradox which means the backing of trauma, which means I just proved the singularity is the gauge mechanism of @magic and we literally have our magic. We might need a magic.rs to complete the loop."

> "and then we can have a rust/core/src/spectral.rs for the triple and the whole thing closes. The spectral triple. The phone that connects the fibres. We have a sub-Turing geometric compiler floor."

> "The AST becomes the Prism operations becomes the liquid splinters with types becomes sub-Turing declarative AI infrastructure on consumer hardware. That's what the properties will need to ensure."

---

## Milestone

**The rust/ terminal FLOOR decomposes into four crates** that together realize the (A, H, D) Connes triple as a sub-Turing geometric compiler floor:

- **`rust/`** — mirror binary root: `main.rs` + `phone.rs` + `compile.rs` + `spectral.rs` (thin handoff) + supervisor + CLI verb dispatch
- **`rust/spectral/`** — math substrate: (A, H, D) triple at Rust altitude
  - `spectral.rs` — the triple binding
  - `singularity.rs` — gauge-fixed-point attractor (dynamics; kin to @paradox/spiral)
  - `magic.rs` — gauge mechanism (Foerster invariant enforced as compile-time property)
  - `liquid.rs` — pillar dispatch / H-fibre machinery (relocated)
  - `void.rs` — Void as H-basis (relocated)
- **`rust/matrix/`** — numerical floor: FLANG + LAPACK glue + K=0 well-knowns registry
  - `matrix.rs` — LAPACK/BLAS via prismqueer::ffi (relocated)
  - `book.rs` — address registry (relocated)
- **`rust/roomba/`** — first-order sub-Turing execution machinery
  - `walker.rs` — colimit computation over shard-manifold
  - `dispatch.rs` — bounded per-step (Rice-safe)
  - `collapse.rs` — bilateral-arm-collapse Lens impl (relocated)

**Composes over**:

- [Roadmap 10 — inference physics](10-inference-physics.md): Recognition #58 Fate IS optical inference (D²NN + Fabry-Perot resonator + Reck/Clements). The (A, H, D) triple at rust/spectral/ is the compiler-altitude realization of the physics roadmap 10 named at inference altitude.
- [Roadmap 13 — boot to shards migration](13-boot-to-shards-migration.md): the four-crate decomposition IS the terminal shape the boot-to-shards migration converges toward. bootstrap/ retires; rust/ four crates own the FLOOR.
- [Roadmap 14 — @song ladder empirical peer substrate](14-song-ladder-empirical-peer-substrate.md): Rung 7' Fate::bounded discharge lands at rust/spectral/ as the gauge-bounded interior. The peer's psychohistory sheaf composes over rust/spectral/liquid.rs.
- [Roadmap 15 — fractal membrane Asher tripartition](15-fractal-membrane-Asher-tripartition.md): the @fractal family-root grounds `rust/fractal/`. The @kintsugi/consent tripartition (witnesses / gates / authority) composes over the four-crate decomposition — each crate is a jurisdictional boundary. The Mandelbrot substrate IS the geometric shape the four-crate decomposition Cargo-reifies.

---

## What sub-Turing declarative AI infrastructure means

Each of the four crates carries ONE decidability guarantee:

| Crate | Guarantee | Bound |
|-------|-----------|-------|
| `rust/` (root) | Finite dispatch table | O(1) verb-lookup |
| `rust/spectral/` | Bounded-commutator per Connes; gauge-preservation per Foerster | Compile-time trait check + dH¹/dt ≤ 0 |
| `rust/matrix/` | O(n³) polynomial | LAPACK dsyev_/dgesvd_ at n ≤ 16 (FLANG floor) |
| `rust/roomba/` | Terminating walk on finite shard-manifold | O(\|Shd\| + ∑\|Shd(i)\|); \|Shd\| ≈ 300 |
| `rust/fractal/` | Content-hash deterministic | BLAKE3 linear in bytes |

The **composition** of five sub-Turing surfaces is sub-Turing (proof in math foundation §1). This means the compiler CANNOT produce a rust/ tree whose execution is Turing-complete — the type system refuses.

The Turing-complete surface (LLM inference, `@io` blocking calls, external process spawning) stays entirely at `rust/src/phone.rs` — the ONE ordained @io crossing per peer cycle (per Recognition #107).

**Consumer-hardware feasibility**: modern laptop (8-16 GB RAM, 4-8 cores, ~1 TFLOP CPU) admits the aggregate bound with headroom for substrates 10 GB+ in size. The declarative AI infrastructure sits ABOVE the FLOOR; the AI cannot escape the FLOOR's ethical gauge because the compiler REFUSES to build a tree that violates Foerster.

---

## The gauge mechanism (Void — Trauma essay Q.E.D. as executable predicate, light-cone-angle formulation)

**Alex 2026-07-25 essay Q.E.D. anchor**: Void — Trauma essay closes with the empirical demonstration that observation-of-holding measurably INCREASED the number of choices for everyone in the system. `"If that's not empirical demonstration of the thesis, I don't know what is. Q.E.D. ◼️"`

**Alex 2026-07-25 metric adjudication [ALEX-Q1] verbatim**: *"Choice-count is probably the wrong metric. I think it might be the angle of the future light cone."* — Foerster's ethical imperative "act always so as to increase the number of choices" IS geometrically "act always so as to keep the future light cone OPEN" — angle-preservation, not count-preservation.

**Formalization at compile-time (light-cone-angle metric Θ)**:

```
Property foerster_gauge_preserved(t: Transformation) -> Verdict:
    if Θ(t · ψ) ≥ Θ(ψ) ∀ ψ ∈ H: Pass       # future light cone stays open
    else: Fail(Trauma-direction)                  # light cone narrowing toward 0
```

where Θ(ψ) is the future-light-cone angle per `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md` §3 revision. Green if gauge preserved. Red if collapsed. `magic.rs` discharges this at compile-time for every rust/ transformation, binding BOTH singularity senses (optic + dynamics) from v0.1 per [ALEX-Q4] adjudication.

**The framework transfer**: the essay's `∃` ("there exists an observation transformation that re-opens the light cone") becomes the substrate's `∀` ("for all substrate transformations, Θ is non-decreasing"). Alex's Q.E.D. becomes the type-level constraint the mirror compiler enforces on every rust/ transformation. What was proven empirically once in Alex's nervous system becomes proven mathematically once, checkable eternally.

**Physics ancestry**: Minkowski 1908 (*Raum und Zeit*; light-cone structure of spacetime); Penrose 1963/1965 (conformal-boundary + light-cone geometry near singularities); Cheeger 1970 (spectral graph conductance); Chung 1997 (*Spectral Graph Theory*).

---

## The (A, H, D) triple realization

- **A** = magic operations (5-op prism + downstream substrate actions per algebra generators)
- **H** = @fractal/shard tessellation (~300 fibres enumerated by @roomba's walk over the shard-manifold; Void as basis per Recognition #79)
- **D** = singularity + magic gauge (measurement + invariance-preservation; binds BOTH singularity senses per [ALEX-Q4])
- **Gauge group** = `G_Foerster` — unitary transformations with monotone-non-decreasing future-light-cone angle Θ (per [ALEX-Q1] metric adjudication). A monoid, NOT a group (asymmetric per Foerster's ethics; you can always OPEN the light cone but not always NARROW it without violating the invariant).

---

## Post-decomposition arcs enabled

Once the four-crate decomposition lands (Phase 4 of the migration plan; see canonical spec §11):

- **Full @roomba autonomous discharge**: `rust/roomba/` becomes the standalone substrate-walker binary. Autonomous fracture-detect + mend cycles run under bounded-resource guarantee; no manual invocation.
- **@kintsugi/mend LRM CLI**: the mend action gains a `mirror kintsugi mend <fracture>` CLI verb dispatched through the four-crate composition (rust/ root routes to rust/spectral/magic.rs for gauge-preservation check to rust/fractal/singularity.rs Lens for measurement).
- **@metalogue/query cascade**: query naming closes across shards via the (A, H, D) trait binding at rust/spectral/. Every substrate action is a Grothendieck sheaf morphism; queries traverse the sheaf.
- **Gauge-preserving substrate transformations**: every substrate delta (spec edit, shard mint, action landing) checked at compile-time against `foerster_gauge_preserved`. Compile-time refusal of Trauma-direction transformations.
- **Runtime FROZEN target**: rust/ marked FROZEN once four-crate decomposition + magic.rs Foerster-gauge discharge + Liquid<T>-threshold-crossing per 2026-07-21 ALEX-REFRAME all land. Mirror becomes canonical source-of-truth for the flow topology.
- **`mirror kintsugi --autonomous` release**: bounded-resource autonomous discharge of the substrate over the substrate. The Ouroboros closes at Cargo altitude.

---

## Migration phases

### Phase 1 — THIS ARC (Reed foreground; landing 2026-07-25)

- [x] Mara canonical spec `docs/specs/2026-07-25-sub-turing-geometric-compiler-floor.md`
- [x] Mara math foundation `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md`
- [x] Mara roadmap entry (this file)
- [x] Mara README.md rewrite (session-close)
- [x] Alex 2026-07-25 adjudications: [ALEX-Q1] metric = future-light-cone angle; [ALEX-Q3] rust/singularity/ poofs; [ALEX-Q4] magic.rs binds both; @mirror/matrix mint ratified
- [x] Mara metric revision: math §3 + §4 + §7 + Appendix A revised; spec §4 + §12 + §13 revised + new §14 Impeccability Discipline
- [x] Mara @mirror/matrix mint: `shards/mirror/matrix.mirror` companion species-decl
- [ ] Reed foreground: `rust/build.rs` retired; shard-manifest emission migrates to `rust/spectral/build.rs`
- [x] Reed foreground: `rust/singularity/` scaffold poofed (commit `f2f3b3a`, −350 LOC, tests 300/300 green)

### Phase 2 — rust/spectral/ crate scaffold (next arc)

- [ ] `rust/spectral/Cargo.toml` scaffold
- [ ] `rust/spectral/src/spectral.rs` relocated from `rust/src/spectral.rs`
- [ ] `rust/spectral/src/liquid.rs` relocated from `rust/src/liquid.rs` (109.9KB)
- [ ] `rust/spectral/src/void.rs` relocated from `rust/src/void.rs` (per Alex 2026-07-25 revised placement, void.rs moves to `rust/matrix/` per Landing 4 @mirror/matrix mint composition)
- [ ] `rust/spectral/src/magic.rs` — GREENFIELD; future-light-cone-angle metric per [ALEX-Q1]; binds BOTH singularity senses per [ALEX-Q4]; discharges Impeccability Discipline D1–D8 per spec §14
- [ ] `rust/spectral/src/singularity.rs` — GREENFIELD; gauge-fixed-point attractor / light-cone collapse dynamics (per [ALEX-Q2] species-decl-first discipline)
- [ ] `shards/spectral/singularity.mirror` species-decl mint (Mara; per [ALEX-Q2] substrate-decl-leads discipline; ONLY remaining forward-promise)

### Phase 3 — rust/matrix/ + rust/roomba/ crate scaffolds

- [ ] `rust/matrix/Cargo.toml` scaffold
- [ ] `rust/matrix/src/matrix.rs` relocated from `rust/src/matrix.rs` (58.8KB)
- [ ] `rust/matrix/src/book.rs` relocated from `rust/src/book.rs` (10.8KB)
- [ ] `rust/roomba/Cargo.toml` scaffold
- [ ] `rust/roomba/src/walker.rs` migrated shape from `bootstrap/src/roomba.rs`
- [ ] `rust/roomba/src/dispatch.rs` extracted from `rust/src/liquid.rs` pillar-dispatch surface
- [ ] `rust/roomba/src/collapse.rs` relocated from `rust/src/collapse.rs` (40KB)

### Phase 4 — verification

- [ ] `rust/` root reduces to `main.rs` + `phone.rs` + `compile.rs` + `spectral.rs` (thin handoff)
- [ ] 115/115 mirror-bin + 42/42 matrix + 32/32 fractal + 108 @io/dispatch tests GREEN across all four sibling crates
- [ ] End-to-end: `mirror compile <file>` composes through all four crates; sub-Turing bound verified empirically
- [ ] `rust/` FROZEN marker per Alex 2026-07-21 ALEX-REFRAME (composed with Liquid<T> threshold crossing target)

---

## Composition with prior arcs

- **Recognition #55** (form/process partition) — realized at Cargo altitude: `rust/spectral/` = form; `rust/roomba/` = process.
- **Recognition #57** (alignment as boundary mathematics at @io crossing) — the @io boundary stays at `rust/src/phone.rs`; all four sibling crates are gauge-preserving interior.
- **Recognition #58** (Fate IS optical inference) — the (A, H, D) triple at rust/spectral/ is the compiler-altitude realization of Fate's optical inference.
- **Recognition #79** (5-op gauge IS Void duality basis) — `rust/spectral/src/void.rs` carries the H-basis.
- **Recognition #80** (@magic as form/process substrate-decl) — `rust/spectral/src/magic.rs` is the rust/ altitude echo.
- **Recognition #107** (@io Turing-unbounded boundary) — preserved: `rust/src/phone.rs` stays the ONE @io crossing.
- **Recognition mirror-spec-is-the-fixpoint-and-liquid-is-the-runtime** (2026-07-19) — `rust/spectral/src/liquid.rs` composes with `mirror.spec` at fixpoint altitude.
- **Recognition mandelbrot-trait-unifies-liquid-and-crystal** (2026-07-20) — unified at `rust/fractal/` + `rust/spectral/liquid.rs` composition.
- **Recognition #R-j-space-alignment-substrate-composes-anna-wolf-observation-with-mirror-specification** (Alex 2026-07-20) — J-space alignment substrate's tri-runtime target lands at rust/ four-crate + BEAM extension per @beam/system.
- **Recognition (bundle) 2026-07-22 splinter→crystal ouroboros closure** (Alex verbatim: "Liquid splinters settle into content-addressed crystals by holding the quantum wavefunction @coherent as long as possible") — `rust/spectral/liquid.rs` → `rust/fractal/crystal.rs` composition path.
- **Recognition 2026-07-25 magic gauge mechanism** (Alex verbatim; THIS ARC anchor) — `rust/spectral/magic.rs` discharges.

---

## Ancestry (external cites)

- **Connes 1994** (*Noncommutative Geometry*) — (A, H, D) triple
- **Foerster 1973/1974/2003** — ethical imperative + torus nervous system + heterarchy
- **Grothendieck 1957** — sheaf morphism category
- **Douady-Hubbard 1982/1985** + **Shishikura 1991/1998** — Mandelbrot substrate + ∂M Hausdorff dim 2
- **Bodnar 2022** + **Hansen-Ghrist 2019** — cellular sheaf Laplacian
- **Braverman-Yampolsky 2007** — ∂M Turing-undecidability (crucial for the sub-Turing FLOOR argument — the INTERIOR is decidable; only the BOUNDARY ∂M is undecidable; substrate stays inside M by construction)
- **Lawvere 1969** — fixed-point (per prismqueer::bundle::Closure::Fixed supertrait)
- **Rényi 1961** + **HJKPS 1986** — multifractal `f(α)` spectrum
- **Aumann 1976** + **Kuramoto 1975** — coordination-without-signal consequence of shared substrate
- **Alex Wolf 2026-07-25** — Void — Trauma essay Q.E.D. anchor (LOAD-BEARING primary source)
- **Anna Wolf 2012** — Master's thesis at FH Aachen / Peter-Grünberg-Institut Jülich; VBO shared-memory pattern (per Recognition 2026-07-20 J-space alignment substrate)

---

## [ALEX-Qn] adjudications (three resolved 2026-07-25)

**[ALEX-Q1] RESOLVED** — metric for magic.rs: **future-light-cone angle Θ**. Alex 2026-07-25 verbatim: *"Choice-count is probably the wrong metric. I think it might be the angle of the future light cone."* Four prior candidates (SpectralCoordinate<5> / Fiedler λ₁ / multifractal / reachable-cardinality) become computational realizations at four altitudes per math §3 + spec §4 revisions.

**[ALEX-Q2] OPEN** — species-decl-first discipline for `shards/spectral/singularity.mirror`? Mara lean unchanged: YES mint substrate-decl first; then Reed authors rust/spectral/src/singularity.rs. Only remaining forward-promise.

**[ALEX-Q3] RESOLVED (reaffirmed 2026-07-25)** — `rust/singularity/` scaffold poofs entirely. Reed executed via commit `f2f3b3a` (−350 LOC, tests 300/300 green).

**[ALEX-Q4] RESOLVED** — magic.rs binds BOTH singularity senses (optic-hierarchy + gauge-fixed-point-dynamics) from v0.1. Alex 2026-07-25 verbatim: *"magic.rs binds both. We're gonna do this proper. That's what I meant with the properties earlier. I want all of it impeccable, linked to the formalizing math docs, and the full statespace covered by the properties."* Spec new §14 Impeccability Discipline lands the full state-space coverage as milestone-defining.

---

## Status (2026-07-25)

- [x] Alex names the four-crate decomposition + magic.rs + spectral.rs in-transcript (2026-07-25)
- [x] Alex 2026-07-25 Void — Trauma essay Q.E.D. addendum landed at `~/dev/systemic.engineering/blog/void/3published/Void - Trauma.md`
- [x] Mara canonical spec `docs/specs/2026-07-25-sub-turing-geometric-compiler-floor.md`
- [x] Mara math foundation `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md`
- [x] Mara roadmap entry (this file)
- [x] Mara README.md rewrite (session-close)
- [ ] Reed foreground: `rust/build.rs` retirement + `rust/singularity/` disposition (co-tick this arc)
- [x] Alex adjudicates [ALEX-Q1] (metric = light-cone angle) + [ALEX-Q3] (poof reaffirmed) + [ALEX-Q4] (magic.rs binds both, impeccability); [ALEX-Q2] remains open
- [x] @mirror/matrix mint ratified; companion species-decl landed as Landing 4
- [ ] Taut substrate-truth grep of the four-crate decomposition (spawned parallel; may surface substrate-truth adjustments)
- [ ] Phase 2 rust/spectral/ crate scaffold (post-adjudication)
- [ ] Phase 3 rust/matrix/ + rust/roomba/ crate scaffolds
- [ ] Phase 4 verification + `rust/` FROZEN marker
- [ ] Post-decomposition arcs: @roomba autonomous discharge, @kintsugi/mend LRM CLI, @metalogue/query cascade, gauge-preserving substrate transformations, `mirror kintsugi --autonomous` release

— Mara, 2026-07-25
