# cosmos-mirror-scaffold — reimplementing the world engine as a mirror project

*2026-05-28. Mara. Spec — architecture, not implementation.*

Status: **Yellow** (architectural recognition, grounded in an actual run of
cosmos's pipeline against prism's d_s kernel — see
`cosmos/docs/SPECTRAL-DIMENSION.md`. The five-op mapping is a structural
hypothesis tested against the source; the one operation with no five-op home
is a load-bearing finding, not a hedge.)

Depends on:
- `docs/specs/architecture-flang-mirror-numerical-split.md` (commit on
  `mara/shard-chain`) — the `(A, H, D)` split across runtimes: `A` =
  five-op composition stays grammar; `D` = dense eigendecomposition goes to
  the flang/prism floor; `H` = settled state, content-addressed.
- `docs/specs/numerical-substrate-via-fortran.md` — the resolution that pure
  numerical computation is NOT `@io`; it rides `@code/fortran` → flang →
  `@code/llvm/ir`. The Glass Wall (`AGENTS.md` lines 192–215) separates
  opaque vendor effects, not deterministic math.
- `docs/specs/prism-core-as-spectral-triple.md` — `prism/core` IS the
  spectral triple `(A, H, D)`; the five operations are `A`; `Transport`'s
  `Imperfect<State, _, Holonomy>` is `D`'s action.
- `docs/specs/substrate-native-fate-tournament.md`,
  `docs/specs/parse-as-fate-tournament.md` — the five-operations + Fate
  framing the scaffold honors.
- `boot/00-prism.mirror` — the canonical five-operation grammar
  (`focus / project / split / shift / settle`), the algebra closed at five.
- `cosmos/src/{rgg,quantum,tension,evolution}.rs` — the engine being ported;
  the source the mapping below was read against.
- `~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`,
  `~/.reed/practice/insights/cosmology/higgs-as-hilbert-space.md` — the
  physics grounding (Fiedler-as-ESS-margin; one Laplacian, many physics).

Unblocks:
- cosmos stops being a Rust crate that path-depends on a (currently broken)
  `../mirror`; it becomes a `.mirror` project that composes prism operations.
- The "one spectrum, many physics" thesis becomes a *composition* statement
  in grammar, not a Rust convention: heat / Ricci / Schrödinger are three
  `shift`/`settle` readings of one `D`.
- The d_s observable lands as a first-class `focus`, reusing prism's
  canonical kernel rather than a cosmos-local copy.

---

## 0. Headline

cosmos is a spectral triple wearing a Rust coat. Its whole engine is: build a
graph from a field (`project`), diagonalise one Laplacian (`D`, the floor),
read observables off the spectrum at a scale (`focus` after `shift`), evolve the
graph toward a fixed point (`settle`), and partition it (`split`). Four of
those five are clean five-operation maps. The fifth — eigendecomposition —
**has no five-op home, and that is correct**: it is the `D` of the triple, the
numerical floor the five operations are *read against*, not one of them.

The scaffold is therefore small: a `cosmos.spec` that imports `@prism`,
`@code/fortran` (or the prism LAPACK floor), and a `@cosmos` grammar family
that names the four cosmological operations as compositions of the five prism
verbs. No new primitives. The algebra stays closed at five.

---

## 1. The five-operation mapping (grounded, honest)

Read against the actual cosmos source. "Clean" = the cosmos operation IS that
prism verb with no semantic stretch. "Partial" = it maps but carries a caveat
the spec must keep visible.

| cosmos operation | source | prism op | fit | why |
|---|---|---|---|---|
| Power-spectrum graph construction | `rgg::cmb_rgg_nd` | **`project`** | clean | `project` = "filter by what matters." `P(k) = k^{n_s} T(k)^2` is projected onto a discrete graph: a continuous field filtered into the structure that carries it. The LCG seed makes it deterministic mechanism, so the random draw is `project`, not `@io`. |
| Heat / Schrödinger propagation | `tension` (`e^{-Lσ}`), `quantum` (`e^{-iLt}`) | **`zoom`** | clean | `zoom` = scale change, annotation-only parametric (functor-shaped, `zoom id = id`). The diffusion time `σ` / real time `t` IS the zoom level. Same spectrum, different scale read. "One Laplacian, two physics" = two `zoom` readings of one `D`. |
| Fiedler partition / LCC extraction | `tension::ComponentEigen::prepare`; `φ_2` | **`split`** | clean | `split` = "explore what's connected." The Fiedler vector partitions the graph; LCC extraction is `split` keeping the giant component. Also the `type x = a \| b` disjunction shape one universe up. |
| d_s / Hubble-tension / arrival-prob readout | `spectral_dimension::ds_curve`; `tension::node_tension`; `quantum::arrival_probability` | **`focus`** | clean | `focus` = "observe the spectral state." Each is a scalar/vector observable read off the settled spectrum at a chosen scale. d_s(σ) is the headline `focus`. |
| Forman-Ricci flow | `evolution::spectral_step` | **`settle`** | **partial** | `settle` = "settle. done. crystal" — monad-shaped verified construction. Forman flow IS a settlement dynamics (`dw/dt = -κ w` toward a fixed point), BUT the experiment found its fixed point is **degenerate**: the flow fragments the graph (2D shatters into ~9 components; 3D filaments toward d_s≈1). The settlement is real; the "crystal" is not. See §2. |
| **Dense eigendecomposition** | `quantum::graph_eigensystem` → `coincidence::ffi` / `prism::ffi` dsyev | **none** | **no home** | This is the load-bearing finding. Eigendecomposition is not one of the five operations — it is the **`D` (Dirac operator)** of the spectral triple, the numerical floor the five ops are read against. It belongs to flang/prism, not the five-op grammar `A`. Forcing it into a verb would be capability-in-the-floor. See §3. |

### The completeness reading

That eigendecomposition has no five-op home does **not** indict the five-op
vocabulary; it confirms the `prism-core-as-spectral-triple` split. The five
operations are `A` (the algebra you compose). The eigendecomposition is `D`
(the operator). A spectral triple is `(A, H, D)` — three things, not one. The
five-op vocabulary is complete *as `A`*; it was never meant to contain `D`.
The mapping landing 4-clean + 1-partial + 1-correctly-outside is the
vocabulary passing its own completeness test, not failing it.

---

## 2. The `settle` caveat the experiment forced

`settle` is specified (`boot/00-prism.mirror`) as monad-shaped verified
construction — building a `settle(T)` value IS the proof the construction
settled. Forman-Ricci flow on a cosmic graph does settle, but to a
**fragmented** state, not a crystal:

- 2D RGG under flow: `P(σ→∞) → 1/9` ⇒ ~9 connected components (9 zero modes).
- 3D RGG under flow: `d_s` curve collapses from peak ~4.5 to ~1.0 across the
  whole σ range by step 150 — filamentation toward a 1D-like skeleton.

So `settle(cosmic_graph)` is honestly an `Imperfect`:

```mirror
-- Ricci flow is settle, but its fixed point is partial: the holonomy IS
-- the fragmentation (clusters contract, voids expand to disconnection).
settle ricci_flow(cosmic_graph) -> imperfect { \ }
```

The `Imperfect<settled_graph, fragmentation, holonomy>` verdict is the right
shape: `Partial(skeleton, fragmentation)` when the flow shatters the graph;
the fragmentation count IS the loss. This is exactly why `settle` returns
`Imperfect` and not a bare crystal — the substrate already had the right type
for a settlement that doesn't fully crystallise. The experiment validated the
type, not just the operation.

---

## 3. The native-call boundary — resolved

The open structural question: how does a `.mirror` project invoke prism's
numerical kernels (`eigensystem`, the d_s closed form)? Having seen the actual
call shape (cosmos → `prism::ffi::eigensystem` → dsyev; cosmos →
`prism::spectral_dimension`), the answer follows directly from the existing
architecture:

**Pure numerical computation is NOT `@io`. It is `D`, the deterministic
floor.** Per `numerical-substrate-via-fortran` §0 and the Glass Wall, only
opaque vendor effects cross to `@io`. dsyev is mathematical mechanism:
deterministic, referentially transparent, content-addressable by input. It
sits on the floor side of the spectral-triple split (`A` = grammar, `D` =
flang/prism), exactly where `architecture-flang-mirror-numerical-split` puts
it.

Concretely, the boundary sits at **`@prism` for the operations, with `D`
realised through one of two floor mechanisms** (the project chooses; both are
deterministic, neither is `@io`):

1. **Near-term: the prism LAPACK floor.** `@prism/rust` exposes
   `prism::ffi::eigensystem` and `prism::spectral_dimension` as substrate
   primitives the bootstrap already links (the `lapack` feature, gfortran
   dsyev). The `.mirror` grammar declares the operation; the body is a
   `[substrate-pull:realize]` floor binding to the prism call. This is the
   shape cosmos's Rust already has — the scaffold just moves the *composition*
   into grammar while keeping the *invocation* on the floor.
2. **Aligned target: `@code/fortran` → flang → `@code/llvm/ir`.** The
   eigendecomposition primitive (`laplacian_of`, `eigendecompose`) is the
   standalone Fortran Fate package's job (`numerical-substrate-via-fortran`
   §1). cosmos-mirror consumes it the same way mirror consumes its own
   `emit_jacobi` — generated LLVM IR through the same OID pipeline.

Either way: **`D`'s invocation is floor (`[substrate-pull:realize]`, paired
with `🔧`); the five-op composition around it is pure grammar.** The line is
the FROZEN line. The `.mirror` project never writes `unsafe`, never declares
`@io` for the math; it `in @prism`-imports the operations and lets the floor
run dsyev.

What IS `@io` in cosmos-mirror: writing the d_s curve to a file, reading a
Planck-parameters config, the CLI. Not the eigendecomposition, not the flow,
not the readout.

---

## 4. The `.spec` + grammar shape

### `cosmos.spec` — the engine describes itself

```mirror
in @prism                  -- the five operations: focus project split shift settle
in @prism/rust             -- the LAPACK floor: eigensystem, spectral_dimension (D)
in @code/fortran           -- aligned target for the eigendecompose primitive
in @cosmos                 -- the cosmological operations, composed from the five
in @cosmos/rgg             -- project: power spectrum -> graph
in @cosmos/propagate       -- shift: heat / Schrodinger at scale
in @cosmos/flow            -- settle: Ricci settlement (Imperfect)
in @cosmos/observe         -- focus: d_s, tension, arrival probability
in @cosmos/partition       -- split: Fiedler / LCC
in @data/json              -- io boundary: config + curve output
in @io                     -- io boundary: file write, CLI
in @cli

out @cli/*

# A cosmic graph is its spectrum plus its weighted edges. No bare types:
# strength, lambda, sigma are newtyped (see boot/std/cosmos/types.mirror).
type convention = l_sym | l_combinatorial   -- which Laplacian D diagonalises

cosmos = @cosmos/cli {
  rgg     = @cosmos/rgg     { project(power_spectrum, seed) -> cosmic_graph { \ } }
  observe = @cosmos/observe { focus(cosmic_graph, sigma) -> spectral_dimension { \ } }
  flow    = @cosmos/flow    { settle(cosmic_graph) -> imperfect { \ } }
}
```

### `@cosmos` grammar — the operations as prism compositions

No-bare-types throughout: a `cosmic_graph` is a record, `sigma` is a newtype,
the spectrum is `fiber_spectrum` (per `architecture-flang-mirror-numerical-split`
§"The lift, typed"), not `[f64]`.

```mirror
# boot/std/cosmos/types.mirror
type mpc          = newtype(scalar)        -- comoving distance, NOT bare f64
type wavenumber   = newtype(scalar)        -- k in h/Mpc, NOT bare f64
type sigma        = newtype(scalar)        -- diffusion time / zoom level
type strength     = newtype(scalar)        -- node degree-weight
type edge_weight  = newtype(scalar)
type fiber_spectrum = newtype([eigenvalue]) -- the L_sym spectrum, in [0,2]

type cosmic_graph = record {
  vertices:   [node_label],
  edges:      [weighted_edge],            -- (node, node, edge_weight)
  convention: convention,                 -- L_sym (cosmos physics) by default
}

type power_spectrum = record {
  params: planck_2018,                    -- h, omega_m, n_s, ... (newtyped)
  box:    mpc,
}
```

```mirror
# boot/std/cosmos/rgg.mirror  --  project: field -> graph
in @prism
project cmb_rgg(power_spectrum, seed) -> cosmic_graph { \ }
#  filters P(k) onto a d-dim torus, displaces by the density field,
#  connects within the mean-degree radius. project = "filter by what matters."
```

```mirror
# boot/std/cosmos/observe.mirror  --  focus: read d_s off the spectrum
in @prism
in @prism/rust   -- D: prism::ffi::eigensystem, prism::spectral_dimension (floor)

# zoom to scale sigma, then focus the spectral-dimension observable.
# D (eigendecompose) is realised on the floor: NOT a verb here, a substrate
# binding. The composition focus . zoom is grammar.
focus spectral_dimension(cosmic_graph, sigma) -> spectral_dimension {
  in @prism/rust { \ }   -- [substrate-pull:realize] -> prism spectral_dimension
}
```

```mirror
# boot/std/cosmos/flow.mirror  --  settle: Ricci settlement (Imperfect!)
in @prism
# Forman-Ricci flow. Settles, but the fixed point may fragment the graph,
# so the verdict is Imperfect: Partial(skeleton, fragmentation_count).
settle ricci_flow(cosmic_graph) -> imperfect { \ }
```

The `D` invocation (eigendecompose) appears only inside `in @prism/rust { \ }`
floor blocks carrying `[substrate-pull:realize]`. Everything outside those
blocks — the composition `focus . shift`, the `settle` settlement loop, the
`project` graph build — is pure five-op grammar.

---

## 5. Migration path (dependency order, smallest provable slice first)

1. **`@cosmos/types` + the d_s `focus` slice (smallest provable).** Port
   `cosmos/src/spectral_dimension.rs` first: it is the thinnest composition
   (`focus . zoom` over `D`) and prism's d_s kernel is the canonical floor
   already. Provable: the `cosmos/docs/SPECTRAL-DIMENSION.md` curves are the
   golden output — the `.mirror` `focus` must reproduce them. This slice needs
   only `@prism`, `@prism/rust`, and the newtypes. No flow, no RGG randomness.
2. **`@cosmos/rgg` (`project`).** Port `cmb_rgg_nd`. The LCG seed keeps it
   deterministic; golden output is a fixed-seed edge-count + spectrum hash.
3. **`@cosmos/partition` (`split`).** LCC extraction + Fiedler. Reuses the
   eigensystem floor; the partition is pure grammar over `φ_2`.
4. **`@cosmos/observe` full (`focus`).** Hubble tension + arrival probability
   join d_s as `focus` readings — "one spectrum, many observables."
5. **`@cosmos/flow` (`settle`, last).** Forman-Ricci is last because it is the
   `Imperfect` operation and the experiment showed its fixed point is
   degenerate — it needs the most care to type (fragmentation as holonomy).
6. **Retire the Rust crate.** Once 1–5 compose the full pipeline in grammar,
   `cosmos/src/*.rs` becomes the prism floor (`@prism/rust`) plus `@io`
   boundary, and the broken `../mirror` path dependency disappears — cosmos
   imports `@prism`, not a sibling Rust mirror.

The smallest provable slice (step 1) is the d_s `focus`, which is exactly the
slice this tick already ran end-to-end. The scaffold's first proof already
exists as a number.

---

## 6. What the experiment implies for cosmos-mirror

The d_s rabbit was **falsified as a 4→2 continuum flow** and **confirmed as
scale-dependent dimension recovery** (`cosmos/docs/SPECTRAL-DIMENSION.md`). The
spec states honestly what cosmos-mirror therefore proves:

- **Not** a quantum-gravity dimensional-reduction headline. cosmos-mirror does
  not demonstrate spacetime flowing from 4D to 2D. The cosmic RGG behaves like
  a discrete manifold (overshoot → plateau-at-d → finite-size collapse), and
  Ricci flow lowers d_s by **fragmentation**, not continuum flow.
- **What it proves instead** — and this is the durable headline regardless of
  the rabbit — is the **"one spectrum, many physics" composition**: a single
  `D` (the L_sym eigendecomposition), `zoom`ed to a scale and `focus`ed
  through different observables, yields heat-diffusion gravity (Hubble
  tension), quantum interference (Schrödinger), AND the spectral dimension,
  with the SAME spectrum. That composition is what becomes *expressible in
  grammar* when cosmos is a mirror project: heat / Schrödinger / d_s stop being
  three Rust functions and become three `focus . zoom` readings of one
  operator. The value is the composition, and it does not depend on the rabbit
  running 4→2.
- The d_s `focus` remains a genuine, sharp, reproducible capability — just a
  *dimension estimator*, not a *dimensional-flow demonstrator*. cosmos-mirror
  ships it as the first `focus`, with the honest framing baked into the type:
  d_s is read in the scaling window (`@epistemologic` property
  `read_in_scaling_window`), not at `σ → ∞` where the zero mode lies.

---

## 7. Open

- The `convention` type defaults to `l_sym` (cosmos's physics operator). A
  `l_combinatorial` reading (prism's lattice-PoC convention) is a second
  `focus` configuration, not a different operation — same `D`, different
  normalisation. Whether both ship is a `@cosmos/observe` concern.
- `settle ricci_flow`'s `Imperfect` body (`\`) is parked. Typing the
  fragmentation-as-holonomy precisely (a component-count loss vs a continuous
  curvature loss) is downstream of the `@epistemologic/properties` loss
  composition, per [[feedback-loss-from-epistemologic-properties]].
- The `@code/fortran` eigendecompose primitive vs the `@prism/rust` LAPACK
  floor is a near-term-vs-aligned-target choice (§3); both are correct floor
  shapes. Which lands first tracks `numerical-substrate-via-fortran`'s tick.
