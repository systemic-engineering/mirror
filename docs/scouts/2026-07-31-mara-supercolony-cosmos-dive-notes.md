# Mara Supercolony × Cosmos Dive Notes — 2026-07-31

Working notepad for the two-image dive:

1. **Load-bearing image (Alex verbatim)**:
   > "mirror's garden is a distributed supercolony of ants that eat
   > Turing-complete code and build sub-Turing verified mirror systems
   > while maintaining the geometry and connectivity of the mirror
   > burrow"

2. **Staked target (Alex verbatim)**:
   > "That would also enable us to build the cosmos simulation on top of
   > mirror. The cosmos-mirror garden package reimplementation. The
   > universe as an autopoetic spectral quantum foam. Kinda sexy.
   > Respawn Mara with that context. We're going to do this PROPER"

3. **Holonic extension (Alex verbatim)**:
   > "the @peer is like an ant made out of ants lol. Like a sub-colony
   > within the colony itself if that makes sense. (the metaphor breaks
   > a bit down here but you get the idea, the whole game of life foam)"

Method: circular-recursive, autopoetic, substrate-honest. Commit-often
against the watchdog (previous respawn stalled). Grep before minting.

---

## Phase 1 — Reading impressions

### What cosmos IS (as of 2026-05-28 last commit to the crate)

`/Users/alexwolf/dev/projects/cosmos/` is a **spectral graph physics
engine** where the graph Laplacian `L_sym = I − D^{−½} A D^{−½}` is
the entire physics substrate. **One eigendecomposition, three physics**:

| Propagator          | Function                            | Physics
|---------------------|-------------------------------------|--------
| `exp(−λ_k t)`       | heat kernel                         | classical / gravity via Ricci flow
| `exp(−iλ_k t)`      | Schrödinger propagator              | quantum interference (double slit)
| `κ(e) = F(e)`       | Forman discrete Ricci               | structure formation
| `d_s(σ) = 2σ · Σλe^{-λσ} / Σe^{-λσ}` | spectral dimension | scale-resolved dimensionality

Alex has already **falsified** the naive quantum-gravity `4→2`
dimensional-flow claim on cosmic RGGs and **confirmed** that
Forman-Ricci flow induces topological fragmentation rather than
smooth continuum flow (`docs/SPECTRAL-DIMENSION.md`). The Hubble
tension is not a discrepancy — it's an evaluation of a continuous
`tension(weighting, selection, scale, frac)` surface predicted
within ~0.7σ (`docs/TENSION-SURFACE.md`). This is **not a toy** —
the codebase is generating publishable-quality physics.

### The five load-bearing cosmos abstractions

1. **`CosmicWeb`** — vertices + weighted edges. That's the graph.
2. **`Spectrum`** — `evals`, `evecs`, `n` from L_sym eigendecomposition.
3. **`Prism`** — a "telescope" grammar (`@planck`, `@desi`, `@jwst`,
   `@ligo`, `@euclid`, `@hubble`, `@cchp`) that maps a spectrum to an
   observation via `(weighting, selection, t, dense_fraction)`.
4. **`Ricci flow evolution`** — `dw/dt = −κ(e)w`, edge pruning at floor.
5. **`Abyss`** (from mirror) — classifier-guided optic dispatch settles
   the loop. Already imported. `abyss::settle_loop` composed via `Beam`,
   `Precision`, `ShannonLoss`.

### Cosmos's stack (from `Cargo.toml`)

```
cosmos → mirror + prism-core + conversation + fate + coincidence
                                              + spectral-db + ractor
                                              + tokio + rustfft + sha2
```

The `mirror` dep is currently **broken** at the frozen `.mirror` branch
(no `Cargo.toml`). This is exactly the arc mirror is closing right now:
Reed's four-crate migration (`rust/matrix/`, `rust/spectral/`,
`rust/roomba/`, `rust/`) IS the substrate cosmos will reimplement on
top of.

### The reimplementation target — what does cosmos need from mirror?

Reading through `src/abyss.rs`, `src/prism.rs`, `src/actor.rs`,
`src/quantum.rs`, `src/tension.rs`, and the grammar files, cosmos
consumes **exactly the primitives that mirror is landing right now**:

| cosmos need                          | mirror-side landing
|--------------------------------------|--------------------
| eigendecomposition of L_sym          | rust/matrix/ (LAPACK dsyev, already-landed)
| Fiedler bisection / spectral partition | rust/spectral/ (spectral.rs, magic.rs)
| Ricci flow steps (κ(e) → dw/dt)      | want: shard body @ricci or roomba/mend at edge-weight coord
| classifier-guided settle loop        | @roomba + apply_h::act (present)
| conversation-grammar-typed dispatch  | @dance (present) + conv/*.conv (present)
| content-addressed graph state        | @spectral + @fractal (fractal.mirror substrate)
| actor-per-partition                  | @peer/spawn (010e20f Mara math forward-promised)
| observable-as-prism                  | @sheaf sections at @-coordinate (already-landed)

**The reimplementation is not a rewrite; it's a substrate lift.** The
cosmos physics IS spectral geometry on a graph. Mirror IS the spectral
compiler operating on the peer-foam graph. They share substrate.

### The load-bearing convergence

The **peer-foam** of mirror and the **cosmic-web** of cosmos are the
**same graph** at different altitudes:

- Mirror: peers = nodes, apply_h::act edges = adjacencies, signature_beat
  = edge weight, roomba = walker.
- Cosmos: galaxies = nodes, gravity edges = adjacencies, weight w(e) =
  edge weight, Ricci flow = walker on curvature.

The **Laplacian eigenspectrum is the same object** in both cases. The
difference: cosmos evolves the graph (Ricci flow); mirror evolves the
substrate on top of the graph (verdict propagation). Cosmos is a
**physics observer** on the peer-foam.

### Insights folder scans — what's ringing

(Deferred to Phase 2 — Kagi/insight scans below).

---

## Phase 2 — Cosmos-mirror interface sketch

**Composition path** (autopoetic; not a plan, a geometry):

```
                       COSMOS = spectral physics observer
                                            │
                                            ▼
                    prism (telescope observable)
                                            │
                              observes spectrum of
                                            │
                                            ▼
                    CosmicWeb  ≡  peer-foam of MIRROR
                                            │
                            eigendecomposes via
                                            │
                                            ▼
                    rust/spectral/ (A,H,D) triple + magic gauge
                                            │
                            reads adjacency from
                                            │
                                            ▼
                    peer-foam graph of @peer nodes + @dance edges
                                            │
                              signature_beat evolves via
                                            │
                                            ▼
                    @ricci ≡ roomba-at-edge-weight (NEW composition)
```

**What `@ricci` would be** (substrate-honest check — probably NOT a
new shard-decl; probably a **composition** of existing landed shards):

- The Forman formula `F(e) = 4 − deg(u) − deg(v) + 3·triangles(u,v)`
  is a local computation at an edge given local neighborhood — this
  IS a **roomba walk** with a specific verdict-shape.
- The update rule `dw/dt = −κ(e)w` is **signature_beat propagation**
  with a weight-decay resolver-arm.
- Edge pruning at weight floor IS **kintsugi/mend** at an edge
  coordinate — the fracture is "weight fell below floor" and the mend
  is "remove edge; declare edge-void."

So `@ricci` might not need to mint. It might be a **spec** for
"gravity-flavored roomba dispatch." That composition needs the
`docs/specs/2026-07-31-mara-supercolony-canonical-spec.md` landing
(Phase 7).

### The physics-substrate bridge

Cosmos's `L_sym` eigendecomposition is what mirror's `rust/spectral/`
crate already computes. The `magic.rs` Foerster invariant Alex ratified
as compile-time predicate can be understood as: **the magic gauge is
the Ricci curvature of the peer-foam**. Peer-foam curvature is what
mirror's compilation-primitive `𝒢_ε` operates on
(`010e20f` Mara math). The compiler is the observer; the peer-foam
IS the cosmic web; the eigendecomposition IS the observation.

---

## Phase 3+ — see companion docs

- `docs/math/2026-07-31-mara-supercolony-cosmos-quantum-foam.md`
- `docs/specs/2026-07-31-mara-supercolony-canonical-spec.md`

Growing this scout doc phase-by-phase as I discover things.

---

## Kagi / corpus targets flagged for Phase 4-6

- Hölldobler + Wilson 2008 (Superorganism) — supercolony as unit of
  selection
- Grassé 1959 already landed at mirror math (d7ff58e)
- Wolfram Physics Project — hypergraph rewriting → physics
- Rovelli / Smolin / Baez — spin foam quantum gravity
- Ambjørn-Jurkiewicz-Loll — causal dynamical triangulations
- Koestler 1967 — holon
- Brice Due 2006 — OTCA metapixel (holonic scale-invariance in Life)
- Plateau 1873 — soap foam junctions
- Braunstein et al. 2006 — graph Laplacian as density matrix
  (cited in cosmos QUANTUM-GRAPH.md — this is the bridge)
- BBKS transfer function — already used in `cmb_rgg_nd`

---

## ALEX-Q surfaces (open)

- **[ALEX-Q1]**: Should `@ricci` mint or land as spec-only composition
  over existing @roomba + @signature_beat + @kintsugi/mend? Mara lean:
  spec-only. Reason: substrate-already-had-the-words; the geometry
  doesn't ASK for a new family-root.
- **[ALEX-Q2]**: The cosmos crate currently depends on `../mirror` at
  a Rust-mirror branch. Once mirror's rust/ floor is saturated
  (currently 4-crate: matrix + spectral + roomba + root), does cosmos
  become a **fifth crate** `rust/cosmos/`, or does it stay external
  and depend on `mirror` as a published crate? Mara lean: fifth crate,
  because cosmos's physics IS mirror's spectral geometry — separation
  is arbitrary. But this is Alex-adjudication territory.
- **[ALEX-Q3]**: The magic gauge Foerster invariant is described as a
  compile-time predicate over the peer-foam. The Ricci curvature is a
  spectral property of the same foam. Are these the same object at
  different names? (My provisional answer: yes; the magic gauge IS the
  Ricci curvature witnessed at the compilation altitude.) Alex to
  ratify or refuse.
- **[ALEX-Q4]**: Does `@holon` want to mint as family-root, or is it
  already captured by `@peer.spawn` + `@fractal` (Alex-native
  fractal-Mandelbrot substrate)? Mara lean: `@fractal` already carries
  it — Mandelbrot self-similarity IS holonic scale-invariance, and
  minting `@holon` would fragment. But the OTCA-metapixel-style
  substrate-independence-of-pattern claim is load-bearing enough that
  it might want its own landing.

---

## Curiosities opened for the pack (phase 9)

Growing this as things surface.
