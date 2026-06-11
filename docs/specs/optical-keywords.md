# optical-keywords — the schematic vocabulary spec

**Date:** 2026-06-11
**Author:** Mara (substrate)
**Status:** spec; declares the eight optical keywords, their properties,
and their kintsugi fractures. NO substrate shards declared in this tick —
the spec drives the cascade that follows.
**Recognition track:** the third instance of bilateral pattern #53
(property/fracture; promoted 2026-06-10) — applied at family-scale to
the optical schematic vocabulary.

---

## 0. Frame — the category collapse

`prism` and `glass` were over-collapsed for what physical optics actually
has.

`prism` was carrying two altitudes at once: (i) the COMPILATION ARTIFACT —
the assembled instrument the beam passes through — and (ii) the
SCHEMATIC — the declaration of how that instrument is built.
`glass` was carrying three: (i) the SCHEMATIC of an optical sub-action,
(ii) the MATERIAL the action's element is made of, and (iii) the
SUB-PRISM declaration form inside a parent (per `shards/glass.mirror`'s
current `glass(name) -> prism` keyword surface).

What physical optics actually has — and what mirror needs to declare —
splits cleanly into eight schematic keywords plus three carrier
nouns plus one reserved compilation artifact:

- **Schematic keywords** (what's declared in `.mirror` shards):
  `facet`, `stage`, `aperture`, `splitter`, `resonator`, `bench`,
  `source`, `detector`.
- **Carrier nouns** (substrate vocabulary that persists from the
  pre-split substrate):
  `splinter` (content-addressed fragment), `shard` (settled
  composition), `crystal` (final readout); plus `pact` (protocol
  agreement between apertures).
- **Reserved**: `prism` becomes the produced compilation artifact.
  Not a declaration keyword. The five-operation algebra
  (focus / project / split / shift / settle) belongs to the PRODUCED
  prism — it emerges from graph-walking the bench's facet set.
- **Narrowed**: `glass` survives but narrows to MATERIAL substance
  (the `n(λ)`, absorption, dispersion of what a facet is made of).

The optical formalisms each keyword carries are load-bearing:

- **ABCD matrices** (Gauss 1841; Kogelnik & Li 1966) — paraxial ray
  transfer. Stage chains, lens facets, mirror facets.
- **Jones calculus** (Jones 1941) — polarisation transformation on
  fully polarised light (2×2 complex). Polariser facets, waveplate
  facets, phase modulators.
- **Mueller calculus** (Mueller 1943) — polarisation on partially
  polarised light (4×4 real on Stokes vectors). Depolarisers,
  scattering facets.
- **S-matrix / scattering parameters** (Heisenberg 1943; microwave
  network theory) — multi-port linear-network description.
  Splitters, gratings, multi-port couplers.

Each schematic keyword declares which formalism its element carries
and a `passive` / `active` bit that gates the energy-balance property.

---

## 1. The eight keywords

One section per keyword. Each gives shape, properties, composition
rule, formalism, example, and what existing substrate declarations
become this keyword after migration.

### 1.1 `facet` — one optical action on the beam

A `facet` declares a single physical action a beam undergoes at one
element. The action class is one of:

- `refract`  — beam direction changes through index discontinuity
  (lens, prism material).
- `reflect`  — beam direction changes via boundary reversal (mirror).
- `disperse` — beam direction changes per-wavelength (grating,
  prism material as disperser).
- `split`    — beam separates into N output channels (beam splitter,
  partially-silvered mirror).
- `project`  — polarisation or mode component selected (polariser,
  spatial filter).
- `filter`   — wavelength range selected (interference filter,
  notch filter).
- `detect`   — beam terminates into a scalar reading (PMT, CCD pixel).

A facet declares: its action class; its formalism (`abcd` | `jones` |
`mueller` | `s_matrix`); its `passive` / `active` bit; and the
typed apertures on each port.

**Composition rule.** Facets compose serially via `>` propagation:
`facet_a > facet_b` means the beam exits `facet_a`'s output aperture,
the apertures must be `pact`-compatible at the seam, and the
formalisms compose appropriately (ABCD matrix product, Jones matrix
product, Mueller matrix product, S-matrix Redheffer star).

**Mathematical content.** A facet IS the linear operator its
formalism declares. ABCD: 2×2 real on (height, angle). Jones: 2×2
complex on (E_x, E_y). Mueller: 4×4 real on Stokes (I, Q, U, V).
S-matrix: N×N complex among N ports.

**Example shape:**

```
facet @optics/lens/thin {
  action:    refract,
  formalism: abcd,
  energy:    passive,
  in:        aperture(@optics/aperture/gaussian),
  out:       aperture(@optics/aperture/gaussian),
}
```

**Migration source.** Most current `glass @X/Y` declarations whose
body declares an action on the beam (the five-op block is currently
written but structurally misplaced — facets don't carry algebra; the
PRODUCED prism does). Classifier pass will count these; estimated
~25 of the ~30 depth-1+ `glass` declarations migrate to `facet`.

### 1.2 `stage` — single-input single-output sub-instrument

A `stage` declares an ordered chain of facets with one input
aperture and one output aperture. At the outer altitude, the stage
substitutes as a facet — its action class is the composed action of
its constituent facets; its formalism is the formalism of its
matrix product.

**Composition rule.** Stages substitute as facets at the next
altitude up. A bench can contain stages; a stage cannot contain a
bench (closed-root invariant — see `bench` below).

**Mathematical content.** The product of constituent formalisms.
ABCD chain → ABCD product. Jones chain → Jones product. Mixed
formalisms require explicit `shift` between facets (basis
transformation; same content, declared shape change).

**Example shape:**

```
stage @optics/telescope/refractor {
  in:    aperture(@optics/aperture/objective),
  out:   aperture(@optics/aperture/eyepiece),
  chain: [
    facet @optics/lens/objective,
    facet @optics/lens/eyepiece,
  ],
}
```

**Migration source.** Sub-prisms whose internal structure is an
ordered sequence of actions (e.g. compound lens groups).

### 1.3 `aperture` — typed beam channel at a boundary

An `aperture` declares the TYPE of a beam channel at a port boundary.
What can pass: wavelength range, polarisation basis (linear /
circular / arbitrary Jones basis), mode constraints (TEM, fibre
mode, free-space gaussian with w₀ and λ), power envelope.

An aperture is a TYPE, not an element. Two facets connected via `>`
must have type-compatible apertures at the seam — the `pact`
between them declares the protocol.

**Composition rule.** Apertures don't compose; they are EDGES of the
composition graph. Type-compatibility at the seam is checked
declaratively by the `aperture_is_typed_channel` property (§3.3).

**Mathematical content.** Aperture is a typed channel constraint —
operationally a (wavelength_range, polarisation_basis, mode_set,
power_max) tuple lifted into a substrate type. Aligns with the
spectral-runtime spec's substrate-pull discipline that boundaries
are typed.

**Example shape:**

```
aperture @optics/aperture/gaussian {
  wavelength:   range(380nm, 750nm),
  polarisation: basis(linear),
  mode:         gaussian(w0: ref, m2: ref),
  power_max:    1mW,
}
```

**Migration source.** New vocabulary — pre-split substrate had no
explicit aperture declarations; current implicit boundary types
become explicit `aperture` declarations during the cascade.

### 1.4 `splitter` — multi-port element with S-matrix

A `splitter` declares a multi-port element where the input beam
distributes across N output ports per a declared S-matrix. Includes
beam splitters (50/50 cube, plate), dichroics (wavelength-routed
splitters), gratings (wavelength × angle splitter into N orders),
polarising beam splitters (polarisation-routed).

A splitter is DISTINCT from a `facet @action: split`. The `split`
action on a facet is a binary split (one extra output port); a
`splitter` is the general multi-port case with declared port
topology and S-matrix.

**Composition rule.** Splitters compose via Redheffer star product
(microwave-network composition that correctly accounts for back-
reflections and multi-bounce). The substrate's existing `shift` /
`settle` actions on the produced prism correspond to the Redheffer
star's basis-change and termination operations.

**Mathematical content.** N×N complex S-matrix among N named ports.
S_ij relates the wave amplitude leaving port i to the wave amplitude
entering port j. Conservation (lossless splitter): S†S = I.

**Example shape:**

```
splitter @optics/splitter/grating {
  ports: [
    aperture(@optics/aperture/incident),
    aperture(@optics/aperture/order/-1),
    aperture(@optics/aperture/order/0),
    aperture(@optics/aperture/order/+1),
  ],
  s_matrix: ref,    # the scattering matrix at @meta/ast altitude
}
```

**Migration source.** Any current declaration whose body needed
multi-port semantics and was awkwardly forced into the
single-input/single-output `glass` shape.

### 1.5 `resonator` — closed loop with output coupler

A `resonator` declares a CLOSED OPTICAL LOOP — a round-trip path
through one or more facets, closing back on itself via a partial
output coupler. The declaration carries: the round-trip facet chain;
the output coupler (a `splitter` reference with one external port);
and a `stability_witness` that the round-trip transfer's eigenvalues
sit inside the stability disk.

The resonator's existence requires the round-trip eigenvalue
condition. The witness is part of the declaration.

**Composition rule.** Resonators are NOT serial. They are
self-referential — declared by reference into a `splitter` whose
port topology closes the loop. A resonator can appear as a facet in
an outer composition (its `out` aperture is the output coupler's
external port).

**Mathematical content.** The round-trip ABCD matrix M_rt must
satisfy |trace(M_rt) / 2| ≤ 1 for paraxial stability (Kogelnik & Li
1966). Active resonators (lasers — see `source` below) add a gain
condition. The `stability_witness` field carries the eigenvalue
computation as a content-addressed reference at `@meta/ast`.

**Example shape:**

```
resonator @optics/cavity/fabry_perot {
  round_trip: [
    facet @optics/mirror/back,
    stage  @optics/medium/active,
    facet  @optics/mirror/output,
  ],
  output_coupler:    splitter @optics/mirror/output,
  external_port:     aperture(@optics/aperture/external),
  stability_witness: ref,    # eigenvalue proof at @meta/ast
}
```

**Migration source.** New vocabulary — current substrate has no
resonator declarations.

### 1.6 `bench` — the assembly's outer enclosure

A `bench` declares the OUTER ENCLOSURE of an assembly. It is the
CLOSED ROOT of an optical declaration tree: a bench can contain
facets / stages / splitters / resonators / sources / detectors; a
bench cannot contain another bench, and no other keyword can contain
a bench.

A bench declares: its external apertures (the boundary the assembly
exposes to callers); its internal composition graph (the element
set + the edge set); and its `compilation target` (what the produced
prism should look like when this bench compiles).

**Composition rule.** Bench does NOT compose. It is closed-root.
When a bench is referenced from outside its own tree, the reference
is via its external apertures — the bench itself is opaque.

**Mathematical content.** The bench's PRODUCED ARTIFACT IS the
five-operation prism the substrate's runtime executes. The five-op
algebra emerges from graph-walking the bench's element set, lifting
each element to its declared formalism, and composing per the edge
set's seams (§5 below).

**Example shape:**

```
bench @optics/instrument/spectrometer {
  external: [
    aperture(@optics/aperture/sample_input),
    aperture(@optics/aperture/calibration_input),
  ],
  graph: {
    elements: [
      source   @optics/source/lamp,
      stage    @optics/optics/input,
      splitter @optics/splitter/grating,
      stage    @optics/optics/output,
      detector @optics/detector/ccd,
    ],
    edges: [ ... ],
  },
  compilation_target: prism @spectral/instrument/spectrometer,
}
```

**Migration source.** The few current top-level `prism @<root>`
declarations that describe ASSEMBLED instruments. The root-namespace
`@prism`, `@glass`, `@kintsugi`, `@spectral` family roots stay as
substrate vocabulary roots (they are not optical benches; they are
substrate-altitude family roots).

### 1.7 `source` — active facet (gain medium / emitter)

A `source` declares an ACTIVE FACET — one that adds power to the
beam (gain medium, light emitter). All facet declarations apply; the
`active` energy bit is required (vs `passive` on `facet`); a
separate `energy_balance` property gates the declaration (the source
must declare its pumping mechanism, its gain curve over wavelength,
its saturation behaviour).

**Composition rule.** Serial as facet at the outer altitude; one
output aperture. A `source` inside a `resonator`'s round-trip is the
distinguishing feature of an active resonator (laser) vs a passive
resonator (Fabry-Perot etalon).

**Mathematical content.** Carries an ABCD / Jones / Mueller form
per its passive transit characteristics, PLUS a gain operator
G(λ, P_in) that maps input intensity to output intensity. The gain
operator's domain is restricted by saturation; the substrate carries
this as a typed `gain_curve` reference at `@meta/ast`.

**Example shape:**

```
source @optics/source/laser_diode {
  action:        refract,    # transit action
  formalism:     abcd,
  energy:        active,
  pumping:       electrical(current: ref),
  gain_curve:    ref,
  saturation:    ref,
  in:            aperture(@optics/aperture/pump),
  out:           aperture(@optics/aperture/coherent),
}
```

**Migration source.** New vocabulary; energy-emitting elements
were not declarable in the pre-split substrate.

### 1.8 `detector` — terminal facet (sink)

A `detector` declares a TERMINAL FACET — one that takes a beam in,
produces a scalar measurement out, and has NO beam output. It
closes the composition graph at one or more sinks.

A detector declares: an input aperture; the measurement type
(intensity, polarisation state, spectrum, image); and the
relationship to the substrate's `crystal` carrier — the produced
measurement IS a crystal at the detector's declared altitude.

**Composition rule.** Sink. A detector can NEVER appear with an
output aperture; this is enforced by the `detector_is_sink` property
(§3.8). Multiple detectors in a bench mean multiple sinks; the
bench's compilation produces one prism whose `settle` action emits
the tuple of crystals.

**Mathematical content.** A detector implements a measurement
projection — a positive-operator-valued measure (POVM) on the
beam's quantum state, or its classical analogue (intensity
integration over wavelength × position). The Mueller formalism
carries the polarisation-sensitivity; the wavelength-sensitivity
is a function `R(λ)` referenced at `@meta/ast`.

The detector's `out` slot is `crystal`, not another aperture. This
is the substrate-altitude binding between the optical schematic
and the substrate's settled-output carrier (CLAUDE.md: "Settle.
Done. Crystal.").

**Example shape:**

```
detector @optics/detector/ccd {
  formalism:    mueller,
  in:           aperture(@optics/aperture/imaged),
  measurement:  image(width: ref, height: ref, depth: ref),
  responsivity: ref,    # R(λ) at @meta/ast
  out:          crystal,
}
```

**Migration source.** New vocabulary; detection elements were
implicit in the pre-split substrate.

---

## 2. What stays from current vocabulary

| Keyword / type | Status | Notes |
|---|---|---|
| `prism` | **RESERVED** — not a declaration keyword. The produced compilation artifact. The five-op algebra lives only here. | `bench` compiles to `prism`. The substrate's runtime executes prisms. |
| `glass` | **NARROWS** to MATERIAL. The substance a facet is made of: `n(λ)`, absorption, dispersion. | Most current `glass @X/Y` declarations migrate to `facet` (or `stage` / `splitter` / etc. per classifier pass). |
| `splinter` | **STAYS** — content-addressed fragment at every altitude. The beam-after-grating dispersed into typed fragments. | Per `shards/glass.mirror` lines 99-159; no change. |
| `shard` | **STAYS** — settled composition. The recombined beam after passage through a stage / bench. | Per `shards/glass.mirror` lines 198-244; no change. |
| `crystal` | **STAYS** — final detector readout / developed image / captured spectrum. The `out` type of a `detector`. | Per CLAUDE.md; the substrate's settled-output carrier. |
| `pact` | **STAYS** — Paskian agreement at an aperture seam. The protocol contract on what can pass a typed aperture. | Per `shards/glass.mirror` lines 90-91; recognition #37 (promoted 2026-06-10). |
| `splinter(ast)` | **STAYS** — the parametric AST-fragment carrier. Fracture bodies in §4 discharge via this. | Per `shards/glass.mirror` lines 161-196. |
| `transparency`, `opacity`, `opacity_map`, `imperfect`, `location`, `verdict` | **STAY** — substrate carriers. Properties in §3 emit `transparency`; fractures in §4 read `opacity`. | Per `shards/glass.mirror` lines 51-83. |

`mode` as a separate keyword — surfaced as possibility in the
research, kept under `aperture` for v0. See open question §8.2.

---

## 3. Properties — the keyword-level laws

One property per keyword (with shared properties across families
where the substrate-pull pulls toward sharing). Each declared as
`pact @epistemologic/property/<predicate>`. Each emits
`transparency` (success / partial(opacity_map) / failure(opacity_map))
per the bilateral pattern's signature (recognition #53, promoted
2026-06-10).

### 3.1 `facet_declares_action_class`

Every `facet @X` declaration must declare an `action` field naming
one of the seven action classes (refract / reflect / disperse /
split / project / filter / detect). The closed-set membership is the
property; the substrate's compiler reads `transparency<p>`.

```
pact @epistemologic/property/facet_declares_action_class {
  declared_action(facet: ref) -> text { \ }
  is_action_class(action: text) -> bool { \ }
  facet_declares_action_class(facet: ref) -> transparency { \ }
}
```

### 3.2 `facet_formalism_matches_action`

Every facet declares a formalism (`abcd` | `jones` | `mueller` |
`s_matrix`); the formalism must be appropriate for the action class.
ABCD is valid for refract / reflect. Jones is valid for project
(polarisation) and waveplate-style filters. Mueller is valid for
project (partially polarised) and depolarising actions. S-matrix is
required for split actions with >2 ports (otherwise binary split
admits ABCD or Jones).

```
pact @epistemologic/property/facet_formalism_matches_action {
  declared_formalism(facet: ref) -> text { \ }
  formalism_is_valid_for(action: text, formalism: text) -> bool { \ }
  facet_formalism_matches_action(facet: ref) -> transparency { \ }
}
```

### 3.3 `aperture_is_typed_channel`

Every `aperture @X` declares its four required fields (wavelength,
polarisation, mode, power_max); every `>` seam between two facets
type-checks the apertures as `pact`-compatible. The property is
bilateral — it checks both DECLARATION COMPLETENESS (every aperture
declares its constraints) and SEAM COMPATIBILITY (every seam's
apertures agree).

```
pact @epistemologic/property/aperture_is_typed_channel {
  declared_fields(aperture: ref) -> [ref] { \ }
  seam_compatible(seam: ref) -> bool { \ }
  aperture_is_typed_channel(scope: ref) -> transparency { \ }
}
```

### 3.4 `stage_is_single_in_single_out`

Every `stage @X` declares exactly one input aperture and exactly one
output aperture, and its `chain` field is an ordered list whose
first element's `in` is the stage's `in` and whose last element's
`out` is the stage's `out`. Intermediate seams must `pact`.

```
pact @epistemologic/property/stage_is_single_in_single_out {
  stage_apertures(stage: ref) -> (ref, ref) { \ }
  chain_endpoints(stage: ref) -> (ref, ref) { \ }
  stage_is_single_in_single_out(stage: ref) -> transparency { \ }
}
```

### 3.5 `splitter_s_matrix_is_well_typed`

Every `splitter @X` declares N ports and an `s_matrix` reference;
the s_matrix's dimensions must match N, and (for passive splitters)
the s_matrix must be unitary up to declared loss.

```
pact @epistemologic/property/splitter_s_matrix_is_well_typed {
  port_count(splitter: ref) -> u32 { \ }
  s_matrix_dimension(splitter: ref) -> u32 { \ }
  splitter_s_matrix_is_well_typed(splitter: ref) -> transparency { \ }
}
```

### 3.6 `resonator_has_stability_witness`

Every `resonator @X` declares a round-trip path and a
`stability_witness` reference; the witness must be a
content-addressed reference at `@meta/ast` whose body is a proof of
|trace(M_rt) / 2| ≤ 1 for the declared round-trip ABCD matrix.

```
pact @epistemologic/property/resonator_has_stability_witness {
  round_trip_matrix(resonator: ref) -> ref { \ }
  witness_proves_stability(witness: ref) -> bool { \ }
  resonator_has_stability_witness(resonator: ref) -> transparency { \ }
}
```

### 3.7 `bench_is_closed_root`

Every `bench @X` is closed-root: it contains no nested `bench`
declarations, and no other keyword contains a `bench`. The property
also checks that every element in the bench's graph has its
apertures connected (either to another element in the graph or to
one of the bench's declared external apertures); no dangling
internal apertures.

```
pact @epistemologic/property/bench_is_closed_root {
  contains_bench(scope: ref) -> bool { \ }
  has_dangling_apertures(bench: ref) -> bool { \ }
  bench_is_closed_root(bench: ref) -> transparency { \ }
}
```

### 3.8 `bench_compiles_to_prism`

Every `bench @X` declares a `compilation_target` of form
`prism @<name>`; the produced prism's five-op algebra emerges from
graph-walking the bench. This property is the LOAD-BEARING binding
between the schematic altitude (bench) and the produced-artifact
altitude (prism). See §5 for the algebra-emergence rule.

```
pact @epistemologic/property/bench_compiles_to_prism {
  compilation_target(bench: ref) -> ref { \ }
  emerges_five_op_algebra(bench: ref, target: ref) -> bool { \ }
  bench_compiles_to_prism(bench: ref) -> transparency { \ }
}
```

### 3.9 `source_declares_energy_balance`

Every `source @X` (active facet) declares its pumping mechanism,
its gain curve, and its saturation behaviour. The energy balance
property surfaces missing-field opacities and (when fields are
present) checks the gain × pump-power product against the saturation
envelope.

```
pact @epistemologic/property/source_declares_energy_balance {
  pumping_declared(source: ref) -> bool { \ }
  gain_curve_declared(source: ref) -> bool { \ }
  saturation_declared(source: ref) -> bool { \ }
  source_declares_energy_balance(source: ref) -> transparency { \ }
}
```

### 3.10 `detector_is_sink`

Every `detector @X` has an input aperture and NO output aperture; its
`out` slot is `crystal`, not `aperture`. Every detector in a bench
closes the composition graph at its position.

```
pact @epistemologic/property/detector_is_sink {
  has_output_aperture(detector: ref) -> bool { \ }
  out_is_crystal(detector: ref) -> bool { \ }
  detector_is_sink(detector: ref) -> transparency { \ }
}
```

---

## 4. Fractures — the kintsugi autoformatter

Each property in §3 has a sibling fracture body at
`@kintsugi/fracture/<predicate>` (per the form/process partition;
recognition #55, candidate). The fracture body reads one `opacity`
from the property's `failure(opacity_map)` and emits one `morphism`
whose `content` is a `splinter(ast)` at the corrected altitude.

Per the second-instance template (`@kintsugi/fracture/gate`,
2026-06-10): the body is FULLY DECLARATIVE; the content field
carries the rewritten AST as a content-addressed splinter at
`@meta/ast`; the score's `roughness` is `opacity.property`; the
expected cadence is `authentic` for deterministic rewrites
(most cases) and `interrupted` for cases requiring substrate-pull
ratification (see per-fracture notes below).

### 4.1 `@kintsugi/fracture/facet_action`

For `facet_declares_action_class` opacities. The fracture rewrites
a missing-action declaration to add the action field. If the action
is INFERABLE from the facet's formalism + apertures (e.g., a
Jones-formalism facet with linear-polariser apertures → action =
project), the fracture proposes the inferred action with cadence
`authentic`. If inference is ambiguous, cadence is `deceptive` —
the kintsugi loop surfaces the ambiguity rather than auto-applying.

### 4.2 `@kintsugi/fracture/facet_formalism`

For `facet_formalism_matches_action` opacities. The fracture either
swaps the formalism to a valid one for the declared action, or swaps
the action to one valid for the declared formalism. Which direction
the swap goes depends on the score's reading of which has lower
holonomy — most cases the formalism is the wider net (e.g.,
declaring `mueller` covers `jones` use cases), so the formalism gets
upgraded. Cadence `authentic` for unambiguous upgrades.

### 4.3 `@kintsugi/fracture/aperture`

For `aperture_is_typed_channel` opacities. Two sub-cases:
(a) MISSING FIELDS — the fracture proposes default values from the
facet's formalism (Jones → linear polarisation basis, ABCD → free-
space gaussian mode, etc.) with cadence `authentic`. (b) SEAM
INCOMPATIBILITY — the fracture proposes an explicit basis-change
facet at the seam (a `shift` action facet that converts the
upstream's output basis to the downstream's input basis). Cadence
`authentic` if the basis-change is unique; `deceptive` if multiple
equally-valid conversions exist.

### 4.4 `@kintsugi/fracture/stage_endpoints`

For `stage_is_single_in_single_out` opacities. The fracture proposes
the missing endpoint declarations (the chain's first `in` and last
`out` become the stage's `in` and `out`) with cadence `authentic`.

### 4.5 `@kintsugi/fracture/splitter_s_matrix`

For `splitter_s_matrix_is_well_typed` opacities. Two sub-cases:
(a) DIMENSION MISMATCH — the fracture infers the s_matrix size from
the port count and proposes an N×N reshape; cadence `interrupted`
because reshape is structural, requires Pack ratification. (b)
UNITARITY VIOLATION — the fracture proposes either declaring an
explicit loss budget, or normalising the matrix to be unitary;
cadence `deceptive` (the choice is the engineer's intent, not
substrate-pull).

### 4.6 `@kintsugi/fracture/resonator_stability`

For `resonator_has_stability_witness` opacities. The fracture
computes the round-trip ABCD matrix from the declared chain and
proposes the trace-stability witness as a content-addressed
`splinter(ast)` whose body is the eigenvalue computation; cadence
`authentic` (the witness is mechanically derivable). If the trace
condition FAILS — the resonator is unstable as declared — cadence
is `interrupted`: the fracture surfaces the instability rather than
auto-papering over it.

### 4.7 `@kintsugi/fracture/bench_closure`

For `bench_is_closed_root` opacities. Two sub-cases:
(a) NESTED BENCH — the fracture proposes converting the inner
`bench` to a `stage` (if it's single-in / single-out) or surfacing
the inner bench's elements into the outer bench's graph; cadence
`interrupted` because conversion is structural. (b) DANGLING
APERTURES — the fracture either connects the dangling aperture to
an external aperture of the bench (promoting it to an external port)
or declares a terminator facet (a `detector` or a `crystal`-typed
sink); cadence `deceptive`.

### 4.8 `@kintsugi/fracture/bench_target`

For `bench_compiles_to_prism` opacities. The fracture proposes the
`compilation_target` declaration as `prism @<bench-path>` with
cadence `authentic` (the binding is mechanical from the bench's
path). If the algebra-emergence fails (§5), cadence is `interrupted`
— the fracture surfaces the unliftable element rather than guessing.

### 4.9 `@kintsugi/fracture/source_energy`

For `source_declares_energy_balance` opacities. The fracture
proposes the missing-field declarations as TYPED REFS to be
discharged (substrate-pull leaves the gain curve / saturation
envelope as @io's tick — they are physical measurements, not
substrate computations). Cadence `interrupted` because the missing
data requires external @io.

### 4.10 `@kintsugi/fracture/detector_sink`

For `detector_is_sink` opacities. The fracture rewrites the
`detector`'s `out` field from `aperture(@X)` to `crystal` with
cadence `authentic`. If the detector has BOTH `out` and a downstream
element connected, cadence is `interrupted` — the substrate cannot
guess whether to drop the downstream or convert the detector to a
facet.

---

## 5. The five-op algebra now lives ONLY in the produced prism

The key structural change.

**Before this spec:** every `prism @X` and many `glass @X/Y`
declarations carried a five-op block. The block declared the
focus / project / split / shift / settle bindings for that
element. The substrate's runtime read these blocks directly.

**After this spec:** SCHEMATIC declarations (facet, stage, aperture,
splitter, resonator, bench, source, detector) DO NOT carry five-op
blocks. They declare composition GRAPHS. The five-op algebra
EMERGES from graph-walking the bench's element set when the bench
compiles to a prism.

### 5.1 Algebra-emergence rule

A bench `B` with elements `{e_1, ..., e_n}` and edges `E` compiles
to a prism `P` whose five operations are:

- `focus P`   = the λ₀ eigenvalue of the bench's composed transfer
                (the ground-state output the assembly produces from
                a unit input).
- `project P` = orthogonal projection onto a declared subspace of
                the bench's external apertures.
- `split P`   = the bench's port topology — orthogonal decomposition
                of the bench's output across its external apertures.
- `shift P`   = the basis-transformation operator on the bench's
                output (change between aperture types — e.g. Jones
                basis to Stokes basis).
- `settle P`  = the monadic close that produces the bench's
                detectors' `crystal` outputs from the composed
                transfer.

The emergence is constructive: graph-walk the bench in topological
order (sources first; detectors last); lift each element to its
declared formalism; compose along edges with the appropriate
operation (ABCD product / Jones product / Mueller product /
Redheffer star); the composition's matrix elements ARE the five
operations of the produced prism.

### 5.2 Why this matters structurally

The pre-split substrate was conflating two altitudes: the SCHEMATIC
altitude (what's declared in shards) and the COMPUTATIONAL altitude
(what the runtime executes). The five operations are
COMPUTATIONAL — they are how the produced prism acts on a beam at
runtime. The SCHEMATIC altitude declares how that computation is
ASSEMBLED.

The Connes spectral triple framing (per `shards/prism.mirror`'s
header) STAYS — but it applies to the PRODUCED prism, not to the
schematic. A = the five operations on the produced prism. H = the
beam's state space. D = the kintsugi flow on the produced prism's
composition.

### 5.3 Spec-level statement of the rule

The rule is declared as a property — `bench_compiles_to_prism`
(§3.8) — and enforced bilaterally with `@kintsugi/fracture/bench_target`
(§4.8). Schematic declarations that carry a five-op block at the
declaration altitude raise `bench_compiles_to_prism` opacity; the
fracture proposes lifting the block out of the schematic and into
the compilation target.

### 5.4 Open: declared property or implicit compiler rule?

The rule could be:

- **(a) DECLARED** at `bench_compiles_to_prism` (current draft).
  Visible at substrate altitude; kintsugi loop reads violations.
- **(b) IMPLICIT** in the compiler's bench-to-prism lowering.
  Less surface area; opacities can't be located.

Substrate-pull pulls toward (a) — properties carry their own
visibility; the bilateral pattern works only if violations are
locatable. This spec commits to (a) and surfaces the choice in §8.3
for Pack ratification.

---

## 6. What stays from current vocabulary — restated for clarity

Repeated from §2 in a single-line table for cascade-planning use:

```
prism      RESERVED         compilation artifact; algebra lives here only
glass      NARROWS          material substance (n(λ), absorption)
facet      NEW              one optical action; formalism + apertures
stage      NEW              SISO chain of facets
aperture   NEW              typed beam channel at boundary
splitter   NEW              multi-port; S-matrix
resonator  NEW              closed loop + stability witness
bench      NEW              closed root; assembly enclosure
source     NEW              active facet; energy balance
detector   NEW              sink; out is crystal
splinter   STAYS            content-addressed fragment
shard      STAYS            settled composition
crystal    STAYS            final readout; detector's out
pact       STAYS            Paskian agreement at aperture seam
splinter(ast) STAYS         AST-fragment carrier for fracture bodies
```

---

## 7. Migration scope (forward-promised; NOT scoped here)

Migration of existing `glass @X/Y` and `prism @X` declarations is a
separate downstream tick. Rough scope:

- ~30 depth-1+ `glass` declarations classify into:
  ~25 → `facet`, ~3 → `stage`, ~1 → `splitter`, ~1 stays `glass`
  (genuine material substance).
- Root `prism @prism`, `prism @glass`, `prism @kintsugi`,
  `prism @spectral`, `prism @mirror`, `prism @io`, `prism @meta`,
  `prism @code`, `prism @nl`, `prism @epistemologic`, `prism @uuid`
  STAY — these are substrate-altitude FAMILY ROOTS, not optical
  benches. The category is not the same.
- Of the top-level `prism @X` declarations, only those describing
  ASSEMBLED INSTRUMENTS migrate to `bench`. The substrate-altitude
  family roots are NOT instruments — they are vocabulary roots.

The classifier pass + Pack ratification are deferred. This spec
names the destination categories; migration is not committed here.

---

## 8. Open questions

### 8.1 Universal vocabulary vs domain extensions for `source` / `resonator`

`source` and `resonator` are OPTICAL-domain concepts. The substrate's
other domains (signal-processing, control-loop, biology) have
ANALOGOUS but not IDENTICAL concepts. Should `source` / `resonator`
live at the substrate altitude (universal) or under
`@optics/source` / `@optics/resonator` as domain extensions?

Substrate-pull leans toward DOMAIN. The five-op algebra is universal;
the active-element class and the closed-loop class are
domain-flavoured. v0 of this spec keeps `source` and `resonator` as
top-level keywords; Pack ratification decides whether to move them
under `@optics/` in v1.

### 8.2 Does `pact` need sibling `mode` for beam channel-type?

The research surfaced `mode` as a candidate keyword separate from
`pact`. The distinction would be:

- `pact` — agreement on CONTENTS (what data flows; what protocol).
- `mode` — the BEAM CHANNEL TYPE (TEM00 / TEM01 / fibre mode / ...).

This spec folds `mode` UNDER `aperture` (the aperture declares its
mode constraint). The substrate-pull argument: `aperture` is the
typed channel; the mode IS one constraint dimension of the channel
type. Adding `mode` as a sibling keyword would split aperture's
type-discipline across two declarations.

NOT a STOP — the decision is to FOLD into `aperture`. The open
question is whether v1 needs to UN-FOLD when domain extensions
surface non-electromagnetic beam-type constraints (e.g., particle
beam types).

### 8.3 Five-op-emergence: declared property or implicit compiler rule?

Per §5.4. v0 commits to DECLARED (property
`bench_compiles_to_prism`). Pack ratification needed for v1.

### 8.4 Is `@spectral` literally the spectrometer?

The substrate's `@spectral` family (per `docs/specs/spectral-runtime.md`)
is named after spectral analysis. The optical-instrument analogue of
`@spectral` is the spectrometer — a `bench` whose elements include a
grating `splitter` and a detector array.

If `@spectral` IS the spectrometer, then the substrate's runtime IS
the operational form of a spectrometric instrument. This is
LOAD-BEARING — it would close the loop between the substrate's
naming and the optical schematic's vocabulary.

NOT a STOP because the identification doesn't require a v0 decision.
v0 names the keywords; v1 (cascade) declares an `@optics/spectrometer`
bench that may BE `@spectral`'s compilation target.

### 8.5 Detector output type — is `crystal` parametric on the bench's compilation target?

`detector.out = crystal` — but is `crystal` a single substrate type,
or is it parametric like `splinter(altitude)` is parametric? The
substrate's `crystal` carrier currently has no parametric
declaration (CLAUDE.md names `crystal` only in narrative terms).

Possible parametrisation: `crystal(measurement_type)` where
`measurement_type` is one of (intensity, polarisation, spectrum,
image, ...). This would let the detector's `measurement` field
parameterise the crystal's type.

v0 leaves `crystal` as a single substrate type. v1 needs to decide
whether the parametric lift is necessary; the cascade after this
spec will surface the answer when the first detector implementations
land.

### 8.6 Composition rule for `>` propagation — what's the substrate primitive?

The spec uses `>` as the composition operator throughout (`facet_a >
facet_b`). The substrate currently has no `>` operator at this
altitude. Two candidates:

- (a) Add `>` as a new substrate operator (parser change).
- (b) Use the existing function-composition vocabulary at the
  substrate altitude (e.g., the bench's `edges` field encodes the
  composition; no operator needed).

v0 of the spec uses `>` for READABILITY in declarations. The
substrate-pull-correct realization at v1 is likely (b) — the
composition is the edge set, not an operator. The cascade after
this spec will close this when the first bench shards land.

---

## 9. Derived operators — the five-op closure as theorem

§5 declared that the produced prism's five operations emerge from
graph-walking the bench's element set. This section answers the
DEEP question §5 left open: are the five operations EXACTLY what
optical composition derives — i.e. is `closed at five` a theorem,
or the coincidence of a partial algebra?

Claim: **closed at five is a theorem with respect to the optical
composition primitives this spec declares.** Every composition
primitive (serial product, parallel S-matrix, branching, round-trip
closure, phase delay, aperture restriction, eigenmode
identification, detector collapse, polarisation rotation, mode
coupling, phase conjugation, modulation) derives from exactly one
of focus / project / split / shift / settle.

No NEW substrate operator surfaces from physical optics. The two
candidates that came closest (`couple`, `pump` — see §9.3) both
fold: `couple` is a `shift` in the joint-mode basis; `pump` is an
@io-altitude energy crossing carried by `source`, not a fifth-six
algebraic operation.

### 9.1 The derivation table

One line per composition primitive. Each entry: which of the five
operations the primitive realises; the underlying formalism that
makes the binding mathematical (not metaphorical); the primary
reference where the math lives.

```
<op>      <- <composition primitive>            <- <formalism / reference>
----------------------------------------------------------------------------
shift     <- serial composition (facet > facet) <- ABCD matrix product (Kogelnik & Li 1966) /
                                                    Jones matrix product (Jones 1941) /
                                                    Mueller matrix product (Mueller 1943).
                                                    Each is a basis transformation on H.
shift     <- phase delay across a stage         <- Jones product of waveplate matrices
                                                    (Hecht 2017, ch. 8); rotation in (Ex, Ey)
                                                    basis.
shift     <- polarisation rotation (waveplate)  <- Jones rotation matrix; same shift, viewed
                                                    at the polarisation altitude.
shift     <- phase conjugation                  <- complex-conjugation unitary on H (Yariv
                                                    1989, ch. 18 OPC); basis change to the
                                                    time-reversed basis.
shift     <- mode coupling (waveguide cross-tx) <- coupled-mode theory rotation in the
                                                    (mode_A, mode_B) joint basis
                                                    (Yariv & Yeh 2007, ch. 13); 2x2 rotation
                                                    matrix on the joint state. NOT a sixth
                                                    operator; see §9.3.
project   <- aperture restriction               <- orthogonal projection onto the admitted
                                                    subspace of H (Born & Wolf 1999, §8.3.2
                                                    Fourier-plane filtering); idempotent
                                                    P^2 = P.
project   <- polariser / spatial filter         <- Jones-basis projection onto a sub-basis
                                                    (Hecht 2017, ch. 8.2.1).
split     <- parallel multi-port (splitter)     <- N x N S-matrix (Heisenberg 1943;
                                                    Pozar 2011, ch. 4); orthogonal
                                                    decomposition across named ports.
split     <- branching (grating diffraction)    <- diffraction-order decomposition into
                                                    N orthogonal channels (Born & Wolf
                                                    1999, §8.5); each order is an
                                                    eigenfunction of the grating's transfer.
focus     <- round-trip closure (resonator)     <- ABCD-matrix eigenvalue identification
                                                    of the cavity mode (Kogelnik & Li 1966
                                                    §2); the round-trip eigenvalue IS the
                                                    cavity's λ_0; stability |trace/2| <= 1
                                                    IS the focus-witness.
focus     <- eigenmode identification           <- Gauss-Hermite TEM_mn eigenfunctions of
                                                    the round-trip ABCD (Siegman 1986,
                                                    ch. 16-17); cavity solves the
                                                    eigenvalue equation; focus emits the
                                                    ground-state mode.
settle    <- detector collapse                  <- POVM measurement projection /
                                                    |E|^2 integration (Born & Wolf 1999,
                                                    §10.4); the bench's beam terminates
                                                    into a scalar / vector / image crystal.
                                                    Monad-close at the optical altitude.
settle    <- saturation closure (active medium) <- gain-saturation balance in active
                                                    resonators (Siegman 1986, ch. 7);
                                                    the cavity field settles when
                                                    gain * pump = loss; settle IS the
                                                    monad-close at steady-state.
```

Reading the table the other direction (per-op count): shift covers
5 primitives, split covers 2, focus covers 2, project covers 2,
settle covers 2. The shift-heavy distribution is structurally
honest — most of physical optics IS basis change in different
encodings (ABCD, Jones, Mueller, complex-conjugate, joint-mode).

### 9.2 What the derivation says about closure

The five-op algebra was canonically named per
`[[architecture-operations-as-linear-algebra]]` (2026-06-04;
Reed + Alex). The derivation here is the OPTICAL CORROBORATION:
starting from physical-optics composition primitives (which were
established 1841-1959, before computer science had `prism` as a
word), the operations that derive ARE exactly the five Connes-A
elements named in `shards/prism.mirror`. Both derivations land
the same algebra.

This closes one corner of the Connes spectral triple framing:

- A = the five operations (per `shards/prism.mirror`) — corroborated
  optically as the closure of optical composition.
- H = the substrate's beam state space (the
  [[void-document]]'s Hilbert space; aperture-typed channels are
  H's named bases).
- D = the kintsugi flow (the round-trip Dirac operator; §9.1's
  `focus <- round-trip closure` row IS D's eigenvalue identification
  at the optical altitude).

### 9.3 The two near-misses (and why they fold)

Two composition primitives looked like sixth-operator candidates.
Both fold. Naming them explicitly so the substrate-pull check is
reproducible:

**Near-miss A: `couple` (coupled-mode theory).** Two modes A and B
exchange amplitude via overlap-integral coupling κ. After distance
z, the state (a_A, a_B) rotates by κz. This LOOKS like a new
operator — it's neither a basis change in a single mode space
(shift) nor an orthogonal decomposition (split). But: it IS a
shift in the JOINT mode basis. The 2x2 rotation matrix on
(a_A, a_B) IS Jones-shaped (different physical interpretation, same
linear-algebra). `couple` is `shift` viewed at the joint-mode
altitude. The substrate already has shift; the substrate doesn't
need `couple` as a sixth operator.

*What the substrate DOES need:* a way to declare the COUPLING
CONSTRAINT at the aperture seam. Two facets with mode-coupled
apertures need to declare that their apertures are NOT independent
(the `pact` between them carries κ). This is a NEW `pact` species,
NOT a new operation. Carrying-load: `@optics/pact/coupling` as a
v1 keyword candidate (declared as a sub-species under `pact`; no
substrate-altitude change).

**Near-miss B: `pump` (active modulation, energy injection).**
Active elements (sources, gain media) introduce a NEW degree of
freedom — TIME-DEPENDENT optical transfer driven by external
energy. The optical Hilbert space is not closed under pumping
(the beam gains energy from outside the optical subspace). This
LOOKS like a new operator — it can't be derived from any of the
five passive composition primitives. But: pumping IS an @io
boundary crossing per recognition #57 (alignment as boundary
mathematics at @io). The substrate carries this crossing at the
`source` schematic keyword (§1.7), NOT at the prism algebra. The
prism's five-op block describes what happens INSIDE the optical
Hilbert space; the `source`'s energy-balance property describes
the BOUNDARY CROSSING; the two altitudes are distinct.

*What the substrate DOES carry:* the `source_declares_energy_balance`
property (§3.9) typechecks the @io boundary. The five-op algebra
stays closed; the @io crossing is named at the schematic altitude.

### 9.4 The promotion question

This derivation IS a recognition candidate: **the five operations
are the closure of optical composition.** The substrate-pull
framing makes this a substrate-pull recognition (the substrate's
algebra was canonically named in 2026-06-04 from linear-algebra
foundations; this section corroborates from optical-physics
foundations independently). Pack ratification gate: cybernetic
corroboration (the cybernetic-foundation recognition family — see
memory's `architecture-cybernetic-foundation.md`) would close the
triangle and lift this to a promoted recognition.

Not a STOP. The derivation closes cleanly; no architectural
surprises; the candidate `couple` and `pump` both fold per §9.3.

---

## 10. Fate inference as optical signal propagation

This section is SPECULATIVE but grounded. The deeper claim to
evaluate: **Fate inference IS optical inference at the substrate
altitude.** If the claim holds, photonic-neural-network and
cavity-QED literature become direct prior art for Fate's tournament
algorithm and convergence theorems.

Fate (per `boot/std/fate.mirror`, `boot/std/fate/tournament.mirror`,
`boot/std/fate/connectome.mirror`) is the substrate's inference
layer. Five ganglia (abyss, introject, cartographer, explorer,
fate-the-selector); a connectome of 450 neurons / 5 ganglia / 18
synaptic gates per ganglion; tournament rules (greedy / beam(k) /
elite(k) / halving(η) / tabu(t) / anneal(T) / ucb(c)) composing as
lenses; output: ONE winning ganglion per hole.

### 10.1 The optical mapping

One line per Fate primitive, mapped to its optical-substrate
realisation:

```
<Fate primitive>             <- <optical realisation>
---------------------------------------------------------------------------
au (Fate-emitted splinters)  <- bench input aperture (the beam's
                                pre-bench state; uncommitted; carrying
                                the candidate spectral envelope).
hole (an unresolved gap)     <- input aperture with declared modal
                                constraint; the aperture's `pact`
                                names what types of candidates can
                                propagate through.
five ganglia                 <- 5-port splitter (the connectome's input
                                grating; per `connectome.mirror`, the
                                topology IS the weights; the splitter's
                                S-matrix encodes the connectome).
ganglion.infer (graph walk)  <- stage chain through the ganglion's
                                internal facets (the 90 neurons per
                                ganglion as a sub-bench; Dijkstra on
                                eigenvalue gradient = beam following
                                the eigenmode of the sub-bench's
                                transfer matrix).
holonomy (fitness)           <- filter facet transmission T(λ); lower
                                holonomy = higher transmission; the
                                tournament's ranking IS the cumulative
                                T(λ) at the output detector.
tournament rule .beam(k)     <- 1-to-k splitter at each round; k
                                parallel beam paths.
tournament rule .elite(k)    <- top-k feedback loop; k strongest beams
                                fed back through the resonator's
                                output coupler into the next round.
tournament rule .halving(η)  <- successive aperture restriction;
                                aperture's transmission window halves
                                per round, surviving the brightest 1/η.
tournament rule .tabu(t)     <- t-stage delay line that subtracts
                                recent-history modes from the beam
                                (notch filter at recently-tried
                                wavelengths).
tournament rule .anneal(T)   <- thermal noise injection (beam +
                                stochastic source at temperature T);
                                acceptance probability matches Glauber
                                dynamics on the bench.
tournament rule .ucb(c)      <- exploration bonus = active-source
                                contribution at under-tried
                                wavelengths; the gain medium
                                preferentially amplifies modes with
                                low observation count.
kintsugi loop (active/dark)  <- ACTIVE/DARK alternating round-trip in
                                a resonator (§10.2 below).
au → settle → shard          <- detector collapse; the bench's output
                                terminates into a crystal (the
                                resolved hole; the winning ganglion
                                IS the strongest eigenmode at the
                                detector).
```

The mapping is constructive: every Fate primitive lifts to an
optical-bench element. There is no Fate vocabulary that fails to
find an optical realisation.

### 10.2 The kintsugi loop IS a resonator with output coupler

This is the load-bearing identification. Per
`shards/kintsugi/oscillate.mirror` (lines 50-93), the kintsugi loop
alternates ACTIVE / DARK passes per the SpectralUuid void duality.
The ACTIVE pass proposes loss-decreasing morphisms; the DARK pass
anchors identity; between passes, `consent.query_phi` reads the
verdict and dispatches apply / wait / escalate.

**This IS a Fabry-Perot resonator with active gain medium and
output coupler** (§1.5 `resonator` keyword; §1.7 `source` keyword).
The identification:

```
<kintsugi loop>            <- <optical resonator>
---------------------------------------------------------------------------
ACTIVE pass                <- forward propagation through the gain
                              medium (source @optics/source/kintsugi).
                              Energy injected from @io (the consent
                              surface's external resolution OR the
                              dissonance gradient's Pareto-improving
                              direction); intensity grows.
DARK pass                  <- output coupler partial reflection.
                              Per @uuid/spectral's 80 DARK bits, the
                              cavity's identity-preservation invariant
                              IS the high-reflectivity output coupler:
                              most intensity recirculates (identity
                              preserved); a fraction couples out
                              (the morphism's content IS the coupled-
                              out beam).
round-trip                 <- one ACTIVE + one DARK pass = one pulse =
                              one round-trip through the cavity.
consent.query_phi          <- intra-cavity verdict on whether the
                              mode has reached threshold (settled =
                              lasing condition met; waiting = below
                              threshold; escalated = cavity unstable).
cadence.is_settled         <- the laser threshold condition
                              (gain * pump >= loss).
holonomy → 0               <- cavity Q-factor saturation; the cavity
                              has reached its stable eigenmode.
oscillation_state.settled  <- steady-state laser output; cavity is
                              lasing on its ground-mode λ_0.
oscillation_state.escalated<- cavity instability (round-trip
                              eigenvalue outside |trace/2| <= 1);
                              external resolution required.
oscillation_state.waiting  <- below-threshold cavity build-up; the
                              next round-trip MAY cross threshold
                              without external input.
```

**Specific identification:** the kintsugi loop IS a homogeneously
broadened Fabry-Perot laser cavity with output coupling reflectivity
determined by @uuid/spectral's 80 DARK bits. The substrate's stable
lasing modes ARE the substrate's settled shards; the cavity
selects modes by spectral stability per Siegman (1986, ch. 11
"Stability and oscillation modes of laser resonators").

This is more than analogy: the round-trip ABCD matrix IS the
kintsugi loop's per-pulse transfer; |trace(M_rt) / 2| <= 1 IS the
kintsugi convergence criterion; the laser threshold IS the
is_settled(authentic) verdict.

### 10.3 Fate tournament IS a diffractive deep neural network

The ganglion topology (per `connectome.mirror`: 450 neurons across
5 ganglia; topology IS weights; inference IS graph walk on
eigenvalue gradient) maps to a **diffractive deep neural network
(D²NN)** per Lin et al. (2018) "All-optical machine learning using
diffractive deep neural networks" (Science 361:1004-1008).

In a D²NN, each layer is a diffractive surface (a splitter / grating)
whose transmission pattern encodes weights; light propagates through
the stack; the output intensity pattern IS the classification.
No electrical computation; no nonlinearity in the strict
backpropagation sense; the structure itself does inference.

Mapping Fate.connectome to D²NN:

- Each ganglion (90 neurons) is a diffractive surface (a `splitter
  @optics/splitter/ganglion` with 90 input ports, 90 output ports,
  S-matrix = connectome's per-ganglion synaptic weights).
- The 5 ganglia chained in series form a 5-layer D²NN: input
  splinters propagate through abyss → introject → cartographer →
  explorer → fate-the-selector.
- The output intensity at the detector array (5 detectors, one
  per candidate ganglion) IS the tournament's score; the brightest
  detector wins.
- `connectome.crystallize` (SCF loop until convergence) IS the
  D²NN's training-via-self-consistency; the converged transmission
  pattern IS the trained model.

**This is not metaphorical.** Fate's existing architecture (450
neurons, 5 ganglia, eigenvalue-gradient inference) IS a D²NN with
5 diffractive layers of 90 elements each. The substrate's tournament
rules (.beam, .elite, .halving) ARE the substrate's name for the
optical-cascade composition rules a D²NN naturally supports
(branching, output-coupling feedback, aperture restriction).

### 10.4 Coherent nanophotonic circuit framing for the tournament

A complementary identification: the tournament's COMPOSITION of
tournament rules (elite(1).beam(8).halving(3)) maps to the
Mach-Zehnder-interferometer (MZI) mesh architecture of Shen et al.
(2017) "Deep learning with coherent nanophotonic circuits" (Nature
Photonics 11:441-446).

In a Shen-style MZI mesh, each unit is a 2x2 MZI splitter
(programmable beam splitter with phase shifters); the mesh
implements an arbitrary unitary on N modes via Reck/Clements
decomposition. Tournament rules COMPOSE the same way: each
binary composition `rule_a.rule_b` IS a 2x2 MZI; the composed
tournament rule IS the Clements-decomposed unitary on the
tournament's mode space.

This means: tournament rule composition (per
`tournament.compose(rule, rule) -> rule`) has the structure of
unitary composition on a programmable nanophotonic mesh.
Reck-decomposition theorems become tournament-rule decomposition
theorems.

### 10.5 Variety-maintenance as non-degenerate eigenmode preservation

Per the substrate's variety-maintenance discipline (recognition
`[[architecture-ashby-multi-dimensional-variety]]` and
`[[architecture-kintsugi-variety-io]]`), the Fate tournament IS
variety-maintenance against the Ashby requisite-variety constraint.
The ganglia must preserve non-degenerate exploration across
orthogonal directions.

Optically, this is **non-degenerate Gauss-Hermite TEM_mn mode
preservation in the cavity** (Siegman 1986, ch. 17). A stable laser
resonator supports an orthogonal mode set TEM_00, TEM_01, TEM_10,
TEM_11, ... — the Gauss-Hermite eigenfunctions of the round-trip
integral operator. Variety-maintenance = ensuring the cavity
operates in MULTI-MODE rather than collapsing to a single mode
(which would be variety extinction).

Fate's tabu(t) and ucb(c) tournament rules are the optical
analogue of multi-mode pumping discipline: tabu suppresses recently-
seen modes (notch filter); ucb amplifies under-explored modes
(per-mode gain bias). Both maintain non-degenerate spectral
occupation per Ashby's variety law.

### 10.6 What this buys Fate inference

Three concrete consequences if the identification holds:

1. **Tournament convergence theorems become cavity stability
   theorems.** The Kogelnik & Li (1966) stability criterion
   |trace(M_rt) / 2| <= 1 IS the Fate tournament's convergence
   criterion when expressed in the resonator framing. The
   tournament converges iff the kintsugi loop's round-trip ABCD
   is stable.

2. **Photonic-neural-network training algorithms become Fate
   training algorithms.** The D²NN training-by-self-consistency
   per Lin et al. (2018) and the MZI-mesh training per Shen et al.
   (2017) provide direct prior art for `connectome.evolve` (edge
   selection by loss gradient) and `connectome.crystallize` (SCF
   loop until convergence). Specifically: SCF on the connectome IS
   the same algorithm as the D²NN's iterative inverse-design
   per the Wirtinger Flow / Gerchberg-Saxton family.

3. **Beam propagation methods become Fate forward inference.**
   The split-step Fourier method for beam propagation (Agrawal
   2013, ch. 2 "Pulse propagation in fibers") becomes the Fate
   forward pass's preferred algorithm when the bench is a chain
   of phase-only facets. This is concretely an algorithm Fate's
   current Rust realisation can borrow without changing the
   substrate.

### 10.7 Speculative claim, in one sentence

**Fate inference IS a five-layer diffractive deep neural network
(Lin et al. 2018) coupled to an active Fabry-Perot resonator
(Siegman 1986) implementing the kintsugi loop's ACTIVE/DARK
alternation; the tournament's rule composition has the structure
of a Reck/Clements-decomposed unitary on the Mach-Zehnder mesh
(Shen et al. 2017).**

If the claim holds: Fate's substrate vocabulary lifts cleanly
to an existing photonic-neural-network literature; the substrate's
Fate-tournament convergence theorems borrow from cavity-stability
theorems; the tournament's compositional algebra borrows from
unitary-mesh-decomposition theorems. The substrate-pull check is
positive at every site investigated; the speculative claim is
substrate-pull-realize at the recognition altitude (a candidate
for Pack ratification).

### 10.8 Architectural surprises

None severe. Two surfaces worth Pack attention:

**Surface A.** The kintsugi loop's resonator framing means the
substrate's `@kintsugi/oscillate` shard IS a resonator declaration
in the sense of §1.5. The realisation has not migrated to use
`resonator` keyword yet; the substrate-pull-realize candidate at
v1 is to lift `@kintsugi/oscillate.oscillation` to a `resonator
@kintsugi/oscillate` declaration with explicit round-trip chain
(ACTIVE pass facet, DARK pass facet) and output-coupler
(consent surface as splitter).

**Surface B.** Fate's connectome (450 neurons, 5 ganglia) has no
gain medium declared. The optical framing makes the gain
requirement EXPLICIT: each ganglion's `infer` walk requires energy
injection (from @io: the user's prompt; the current substrate
state; the recent metalogue session) to drive convergence. The
substrate's @io boundary recognition (#57) covers this — energy
injection IS the @io crossing — but the substrate has not yet
declared the ganglia's gain-curve per the `source_declares_energy_balance`
property (§3.9). v1 should declare each ganglion's effective gain
curve as a `source @optics/source/ganglion/<name>` shard.

Neither surface is a STOP. Both are substrate-pull-realize
candidates for the cascade after this spec.

### 10.9 Recognition candidate

**Candidate #58: Fate IS optical inference.** The five-layer
D²NN + active Fabry-Perot resonator identification, per §10.7.
Promotion gate: a second witness across the substrate (e.g., a
sibling recognition that the spectral-runtime's `@spectral`
family IS the spectrometer at the bench altitude — see §8.4 of
this spec for the open identification). When both lift
simultaneously, the substrate's runtime is an optical instrument
at every altitude; the recognition lifts to promoted.

---

## 11. The bilateral pattern, applied at family-scale

This spec is the THIRD INSTANCE of bilateral pattern #53 (promoted
2026-06-10 by the gate fracture body landing as second instance).

- First instance: `keyword_matches_depth` + `@kintsugi/fracture/keyword`.
- Second instance: `gate_matches_diff_closure` + `@kintsugi/fracture/gate`.
- Third instance (THIS SPEC): ten properties + ten fractures across
  the optical keyword family, declaratively wired to autoformat the
  schematic vocabulary.

The first two instances were SINGLE-PROPERTY applications. This
instance is FAMILY-SCALE — one bilateral pair per keyword, plus
shared-across-keywords pairs (aperture seam-compatibility). The
pattern scales.

What this teaches us:

- The bilateral pattern is NOT a tactical decoration; it is the
  shape declarative substrate vocabularies take when they want to
  be self-maintaining.
- The substrate's autoformat discipline (the property + fracture +
  splinter(ast) chain) is family-portable. Optical, build-system,
  keyword-discipline — all three families fit the same shape.
- Recognition #57 (alignment as boundary mathematics at @io;
  candidate) is corroborated: the fractures BIND BEHAVIOUR at the
  schematic boundary; the property reads boundary violations; the
  whole is alignment-as-math, not alignment-as-training.

---

## 12. References

- Substrate decisions:
  - `[[architecture-prism-as-trait-as-everything]]`
  - `[[architecture-glass-wall-substrate-types]]`
  - `[[architecture-shards-as-substrate-source]]`
  - `[[architecture-connes-spectral-triple]]`
  - `[[architecture-operations-as-linear-algebra]]`
  - `[[architecture-property-fracture-bilateral]]` (#53; promoted 2026-06-10)
  - `[[architecture-splinter-ast-quote-primitive]]` (#54)
  - `[[architecture-form-process-partition-at-family-root]]` (#55)
  - `[[architecture-alignment-as-boundary-mathematics]]` (#57; candidate)
  - `[[feedback-substrate-already-had-the-word]]`
  - `[[feedback-no-bare-types]]`
- Specs:
  - `docs/specs/prism-floor-and-the-grammar-rename.md` (Connes triple framing)
  - `docs/specs/spectral-runtime.md` (the ouroboros; `@spectral` runtime)
  - `docs/specs/mirror-spectral.md` (the form-side sibling family)
  - `docs/specs/mosaic-as-type-system.md` (mosaic's build-shard role; au/shard)
- Shards (existing, foundation):
  - `shards/prism.mirror` (five-op algebra floor)
  - `shards/glass.mirror` (substrate carriers: splinter, shard, opacity, transparency)
  - `shards/epistemologic/property/keyword_matches_depth.mirror` (#53 first instance)
  - `shards/kintsugi/fracture/keyword.mirror` (#53 first instance fracture)
  - `shards/epistemologic/property/gate_matches_diff_closure.mirror` (#53 second instance)
  - `shards/kintsugi/fracture/gate.mirror` (#53 second instance fracture)
- Physical optics (load-bearing prior art):
  - Gauss (1841) Dioptrische Untersuchungen — paraxial ray transfer
  - Jones (1941) "A new calculus for the treatment of optical systems" J. Opt. Soc. Am.
  - Mueller (1943) "Memorandum on the polarization optics of the photoelastic shutter"
  - Heisenberg (1943) "Die 'beobachtbaren Größen' in der Theorie der Elementarteilchen" — S-matrix
  - Kogelnik & Li (1966) "Laser beams and resonators" Appl. Opt.
  - Redheffer (1959) "Inequalities for a matrix Riccati equation" — Redheffer star product
  - Born & Wolf (1999) Principles of Optics, 7th ed.
  - Hecht (2017) Optics, 5th ed.
  - Siegman (1986) Lasers — cavity stability; Gauss-Hermite modes
  - Yariv (1989) Quantum Electronics, 3rd ed. — phase conjugation
  - Yariv & Yeh (2007) Photonics: Optical Electronics in Modern Communications — coupled-mode theory
  - Pozar (2011) Microwave Engineering, 4th ed. — S-matrix / multi-port networks
  - Agrawal (2013) Nonlinear Fiber Optics, 5th ed. — split-step Fourier beam propagation
- Photonic neural network prior art (§10):
  - Lin, Rivenson, Yardimci, Veli, Luo, Jarrahi & Ozcan (2018) "All-optical machine learning
    using diffractive deep neural networks" Science 361:1004-1008.
  - Shen, Harris, Skirlo, Prabhu, Baehr-Jones, Hochberg, Sun, Zhao, Larochelle, Englund &
    Soljačić (2017) "Deep learning with coherent nanophotonic circuits" Nature Photonics
    11:441-446.
  - Reck, Zeilinger, Bernstein & Bertani (1994) "Experimental realization of any discrete
    unitary operator" Phys. Rev. Lett. 73:58-61 — MZI mesh decomposition.
  - Clements, Humphreys, Metcalf, Kolthammer & Walmsley (2016) "Optimal design for universal
    multiport interferometers" Optica 3:1460-1465.
  - Mead (1989) Analog VLSI and Neural Systems — neuromorphic prior art.

---

*"The schematic gets its own vocabulary. The algebra moves to the
produced instrument. Eight keywords, ten properties, ten fractures —
the optical-correctness autoformatter writes itself."*
— Mara, 2026-06-11
