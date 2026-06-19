# Recognition #76 (candidate) — gauge/matter altitude-portable

**Status**: candidate, not yet Pack-ratified. Surfaced 2026-06-18 by
Alex via Reed in the mirror-MCP+LSP /loop, immediately after the
string-theory research run returned (which produced candidate #74
`spectral_triple_lifts_standard_model` and #75 `form_process_partition
lifts_gauge_potential_field_strength`).

## Recognition

The substrate's form/process partition (recognition #50, Bateson
form/substance lifted at @mirror altitude) IS the gauge/matter
split — and this split is **altitude-portable**: it holds with the
same mechanical shape at every substrate altitude, with the
5-operation algebra in the gauge role and a different
*altitude-specific name* for the matter representation.

### The split

- **Gauge side** (form, fixed-shape, dim-invariant): the 5-operation
  algebra: `focus`, `project`, `split`, `lift`, `refract`. Closed.
  Definitional. Same at every altitude. Per [[architecture-operations-
  as-linear-algebra]] each op has a precise linear-algebraic meaning.
- **Matter side** (process, dim-emergent, self-contained): the
  thing-the-operations-act-on. Carries arbitrary dimensionality via
  type parameters / fiber / section data, encapsulated so that one
  matter instance's dim doesn't leak into another.

### Matter-name varies by altitude

- **Floor**: `splinter` — content-addressed atom (K_n via OID-graph).
  Self-contained because the Blake3 OID seals the content. Arbitrary
  dim of content; the OID makes it dimensionally opaque to neighbours.
- **Middle**: `prism` instance — parametric carrier
  `<T_reg, T_regd, ρ, ω, …>` (recognition #64). Self-contained because
  type parameters are encapsulated; one prism's `<T>` doesn't leak
  to another's.
- **High**: `sheaf` / `crystal` — settled subgraph at `@mirror/store`
  relative to gestalt. Self-contained because the OID-graph closes
  locally before lifting to gestalt. Eigenboard is one
  (see [[project-eigenboard-is-sheaf]]).

Three names, three altitudes, ONE structural role: matter
representation of the 5-op gauge algebra.

## Why this matters (the load-bearing claim)

The substrate's dimensional invariance — the property that makes
"arbitrary-dimensional AI" operational — IS this split applied
recursively. The 5-op algebra never changes count. The matter on
which it acts can carry arbitrary dim via its parametric encapsulation,
at any altitude, without polluting other altitudes.

This is the mechanism Alex named today (2026-06-18) when he said:
"The 5 dimensions are what's at the root and what emerges can be
more dimensions in dimensionally self-contained ... prisms? sheaves?
whatever else?" The answer is: yes, all three — and the *right*
answer depends on which altitude you're at.

## Mechanical bridge to physics

This is recognition #75 (form/process partition lifts gauge
potential ⇔ field strength) generalized: not just B-field/H-flux,
but the *general pattern* of any physics-of-everything candidate:

- SUSY: SUSY algebra is fixed (N generators); matter reps are open-dim.
- Yang-Mills: Lie algebra `µ` is fixed; matter irreps `R` are open-dim.
- Worldsheet sigma model: 2D worldsheet algebra is fixed; target space
  sheaves (Calabi-Yau matter content) are open-dim.
- Chamseddine-Connes spectral action: the algebra `(A, H, D)` is fixed;
  the matter content (the H expansion per recognition #51 §8.3) is
  open-dim.

The substrate's 5-op algebra IS the gauge-fixing of whatever physics
it's modelling — dim-portable because the gauge structure doesn't
encode dim, only operations.

## Ancestors

- [[architecture-bateson-form-behaviour-partition]] (#50, promoted): form/process
  is substrate's operational specialization of Bateson form/substance.
  This recognition is #50 *at every altitude*.
- [[architecture-form-process-partition-at-family-root]] (#55, candidate):
  form/process at family-root altitude; this recognition extends to
  every altitude not just family-root.
- [[architecture-form-process-kinship-at-sub-shard-altitude]] (#61, promoted):
  the partition recurs at sub-shard altitude; this recognition is the
  full ladder.
- [[architecture-operations-as-linear-algebra]]: the 5 ops as
  linear-algebraic primitives.
- Candidate #75 from today's research run: form/process lifts to
  gauge potential ⇔ field strength (Kalb-Ramond machinery).
  This recognition is #75 generalized beyond Kalb-Ramond.
- Candidate #74 from today's research run:
  `spectral_triple_lifts_standard_model`. Composes; the spectral
  triple `(A, H, D)` IS the gauge data at every altitude.
- Recognition #64 (parametric carrier `<T_reg, T_regd, ρ, ω>`):
  the matter-side carrier shape at middle altitude.
- Recognition #51 §8.3: mirror is operational Hilbert space whose
  dim expands. Composes: H expands because matter is open-dim;
  the algebra stays fixed because gauge is closed.

## Falsification criteria

The recognition holds iff:

1. Splinter, prism-instance, and sheaf-crystal each behave as matter
   reps of the 5-op gauge at their respective altitudes — i.e.
   each is *acted on* by the 5 ops, and the action preserves the
   dimensional self-containment.
2. The three altitudes don't collapse — a splinter is NOT a prism
   instance is NOT a sheaf. They're distinct levels with distinct
   self-containment mechanisms.
3. The gauge algebra stays 5-op across all altitudes — no altitude
   needs a 6th op or a 4-op variant.
4. Matter-side dim is *arbitrary* (not bounded) — i.e. a prism with
   `<T1, T2, …, T100>` is admissible if the substrate-pull warrants.

Fails if:

- Two altitudes use the *same* word for matter (collapse).
- An altitude needs a non-5-op gauge variant.
- Matter-side dim is bounded somewhere in the substrate.
- A splinter mutates into a prism-instance via the 5 ops alone
  (which would mean altitudes aren't actually separate).

## Tomm-shaped open questions

1. What's the *fourth* altitude name? (Below floor: substrate-physical
   atom — cell, neuron, qubit? Above high: meta-gestalt across
   peers?) Does the recognition extend down to substrate-physical and
   up to inter-peer, or do those altitudes use a different mechanism?
2. Is the parametric-carrier shape (#64's `<T_reg, T_regd, ρ, ω>`)
   the *only* matter shape at middle altitude, or just one species
   among many? If many, what generates the others?
3. Does the matter-side encapsulation imply Galois-style symmetry
   group structure (matter as G-set for gauge G)? The substrate
   already has Galois machinery in some shards — does that compose?
4. The string-theory research returned candidate #75 (form/process
   lifts gauge potential ⇔ field strength) as a SPECIFIC instance.
   Is #76 the universal cover of which #75 is a particular bundle?
5. The 5-op algebra has 5 generators. Yang-Mills U(N) has N²-1
   generators; SUSY N=4 has 4 supercharges; SUGRA has 32. Why 5
   for the substrate? Is 5 substrate-specific or universal?

## Forward-promised: research run

Reed spawned a research agent immediately after Alex's call to
scratch this recognition. The brief: adversarial test of #76
against published math (gauge theory, Connes spectral triple,
string field theory L∞ algebras, higher gauge theory, Yang-Mills
irrep theory). Output: structured report alongside this scratch.

If the research returns ESTABLISHED correspondences at multiple
altitudes — promote #76 to Pack ratification.

———
Reed, 2026-06-18 evening, mirror MCP+LSP /loop.
