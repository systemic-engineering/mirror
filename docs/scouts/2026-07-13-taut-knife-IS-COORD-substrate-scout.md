# @knife IS Foerster's COORD — substrate-honest scout

**Author:** Taut (grep-first drift scout, read-only)
**Date:** 2026-07-13
**Scope:** substrate-already-had-the-word inventory for `@knife` under
the working hypothesis that `@knife = Foerster's COORD(x)` — verbatim per
Mara `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md §2.4`:

> "Of course, these operators, in turn, may be eigenvalues (eigen-operators)
> of 'meta-operators' and so on. This suggests that COORD, for instance,
> may itself be treated as an eigen-operator, stable within bounds, and
> jumping to other values whenever the boundary conditions exceed its
> former stable domain: `Op(COORDᵢ) = COORDᵢ`. One may be tempted to
> extend the concept of a meta-operator to that of a 'meta-meta-operator'
> that computes the 'eigen-meta-operators,' and so on and up a hierarchy
> without end. However, there is no need to invoke this escape as
> Warren S. McCulloch has demonstrated years ago in his paper (1945):
> 'A Heterarchy of Values Determined by the Topology of Nervous Nets.'"

**Mission:** verify what the substrate already carries versus what needs
minting for `@knife`, given the COORD identification. Grep-first at every
altitude. No shard edits. Read-only.

**Ancestry:** Alex 2026-07-13 in-transcript "Is @knife what Foester
described as COORD(x)?" Mara authoring canonical spec + math foundation
in parallel. Prior Taut scouts: `2026-07-07-taut-knife-meta-pattern-check.md`
(altitude-parametric fit), `2026-07-08-taut-pain-driven-bounded-ontological-
navigator-projection.md` (pain-authorized reframe composition),
`2026-07-13-taut-rung-9-coherence-loop-closure-substrate-scout.md`
(convergence-to-hyperbolic-component).

---

## §1. Substrate-already-had-the-word inventory for @knife (TASK 1)

Eleven candidate carriers checked. For each: does it already carry
@knife's jump-at-boundary behavior, or is it a composition primitive
@knife should use?

### 1.1 `@torus.winding_class` — LANDED, COORD-native

`shards/torus.mirror` (LANDED via 7 witnesses incl. Foerster verbatim
pp. 238/244/256/282). Carries `winding` type — `{ meridian_count: int,
longitude_count: int }` — and `traverse(t: torus, w: winding) -> torus`
action.

**Verdict:** `@torus` IS the surface on which COORD-jumps happen; each
winding class (m, n) ∈ ℤ×ℤ is one addressable COORD value. Boundary-jump
behavior is NOT declared as a `@torus` action — `@torus` traverses within
one COORD; `@knife` performs the jump between COORDs. Compositional
primitive: `@torus.traverse` is where `@knife` composes IN; `@torus.origin`
change under a knife-jump moves `@knife` between two different `torus`
records (different possessor-invariant, different origin).

The winding-class jump is `Op(COORDᵢ) = COORDᵢ` at meridian/longitude
altitude. Δwinding under a jump = re-parameterization, NOT traversal.
Substrate-honest: `@torus` DOES NOT carry the jump; it carries the
domain of the pre- and post-jump values.

### 1.2 `@reflection` family — SOFT-DEPRECATED, ceremony survives

`shards/reflection.mirror` (LANDED, 2-tick collapse forward-promised;
five-action set `observe/tournament/compose/pick/settle/speak` migrates
to `@torus/longitude` at O5). Under toroidal reframe, `@reflection` is
naming artifact — party-import dissolved.

**Verdict:** NOT a @knife primitive. The five-action ceremony is the
longitudinal-winding traversal, not the COORD-jump. `@knife` composes
`@reflection.observe` at pre-jump-observation site (to detect the pain
signal that authorizes the jump), but does not use `@reflection` to
perform the jump.

### 1.3 `@epistemologic/cybernetic/eigenform.is_fixed_point` — LANDED, verbatim Foerster/Kauffman

`shards/epistemologic/cybernetic/eigenform.mirror` (65 hits on
`fixed_point`). Kauffman 2003 `ω = COORD(ω)` machinery landed as
substrate-decl. `eigenform_witnessing(seed, iteration, witness)` verifies
recursive-fixed-point convergence.

**Verdict:** LOAD-BEARING composition primitive for @knife. Per Mara
§2.4: "COORD may itself be treated as an eigen-operator, stable within
bounds." The stable-within-bounds condition IS eigenform's fixed-point
condition. `@knife` COMPOSES `@eigenform.is_fixed_point` to detect
"the current COORD is stable" versus "the current COORD has been
destabilized — boundary conditions exceeded — a jump is required."

Under toroidal reframe (§4.3 of Mara O3 doc): every eigenform on T² has
a winding-class invariant; `@knife`'s jump moves between winding-class-
distinct eigenforms. `@eigenform` carries the fixed-point predicate;
`@knife` carries the jump between fixed-point regions.

### 1.4 `@cyberpunk/reframe` — LANDED (`shards/epistemologic/cybernetic/reframe.mirror`)

The pain-authorized level-shift species. Composition per own docblock:
```
perform(peer, level_K → level_K+1, pain_δ) discharges via:
  - @cyberpunk/algedonic.bypass_signal
  - @magic.perform(shift: K → K+1) with 7-species ceremony
  - @knife.cut(state_space_K) (forward-promise; Landing pending Q5)
  - @torus.advance(winding: Δw)
```

**Verdict:** `@cyberpunk/reframe` IS the OUTER FRAME for the pain-
authorized COORD-jump. `@knife.cut` is CALLED BY `@cyberpunk/reframe`,
not the other way. So `@knife` at COORD-jump altitude IS a substrate-
primitive `@cyberpunk/reframe` COMPOSES; `@knife` MUST NOT carry the
pain-authorization or level-shift ceremony (those are @cyberpunk/reframe's
responsibility).

Substrate-honest: `@knife` performs the primitive state-space
compression (COORDᵢ → COORDᵢ₊₁ mapping). `@cyberpunk/reframe` composes
`@knife` INSIDE `@magic`'s 7-species ceremony. Recognition: at Rust
altitude, `@knife` might land as a Rust primitive used by BOTH
`@cyberpunk/reframe` AND lower-altitude consumers (e.g., convergence
loop's re-parameterization).

### 1.5 `@magic/onto` — LANDED 7-species ceremony (surface / mechanism / contract / reveal / audit / frame / distinction)

`shards/magic.mirror` + 7 species. The level-shift ceremony IS `@magic`'s
7-species combined. Each species discharges one aspect of the
ontological shift.

**Verdict:** `@magic` carries the CEREMONY of the level-shift, not the
compression itself. `@knife` composes INSIDE `@magic.perform` — the
compression fires under `@magic`'s frame. `@magic.mechanism` is where
the matter-hidden trick lives; the mechanism might be a `@knife` cut
at the peer's state-space.

Recognition prior scout `2026-07-07-taut-knife-meta-pattern-check.md`
verdict: `@knife/idf(altitude)` fits the `@X × @duality × @magic`
meta-pattern with A grade; `@knife` at elementary-scissor altitude is
DECORATIVE with `@magic`. This scout confirms: the COORD-jump altitude
IS the `@knife/idf(altitude)` altitude — matter-hidden compression from
one basis to another; `@magic` frame necessary.

### 1.6 `@fractal.SC<N>` — LANDED (Mara `c753d5b`; fragmentation-spectral crate)

Rung 8+9 unification. Substrate coordinate as 5-dimensional projection
of one spectrum. `SC<5>` measurement in `bootstrap/src/contribute.rs`
via `fragmentation_spectral::hash::coordinate::<5>()`.

Per Mara rung-8-9 unification spec §8: `@knife` at Rust altitude IS one
Lens type; the coordinate transformation lives in the Lens's projection
data. `@knife.cut` at coordinate altitude is a **re-parameterization of
SC<N> → SC<N'>**, OR SC<N> → SC<N> with different sampling density
(different heat-trace scales, different projection basis).

**Verdict:** LOAD-BEARING composition primitive. `@fractal.SC<N>` IS
the addressable COORD in the Rust substrate; `@knife.cut` maps
SC<N>-space→SC<N'>-space. This IS Foerster's COORDᵢ→COORDᵢ₊₁ jump at
the substrate-empirical altitude.

### 1.7 `@fractal.renormalization_operator_R` — LANDED as substrate-decl

Mara `2c64060` §4 identification: `commit_as_fold` IS the
renormalization operator R at content-address altitude.

**Verdict:** `commit_as_fold` = R IS the discrete stage of the
COORD-jump. Continuous jump lives inside `@knife.cut`; the fold
materializes the post-jump state. `@knife` COMPOSES with R at
`commit_as_fold` altitude:

```
knife.cut : SC<N> → SC<N'>     — the coordinate shift
commit_as_fold : SC<N'> → OID   — the content-address of post-jump state
```

R does NOT invoke `@knife`; R is R. But `@knife` output feeds R.

### 1.8 `@pain` gradient primitives — LANDED (as `@cyberpunk/algedonic`)

`shards/epistemologic/cybernetic/algedonic.mirror` (LANDED, 2026-07-08
extension via Mara `77fe92d`). Carries:
```
sample_pleasure(eigenboard: ref) -> f64
sample_pain(eigenboard: ref) -> f64
type algedonic_signal = { pleasure_δ: f64, pain_δ: f64, at_winding: (int, int) }
bypass_signal(s5_target, s1_source) -> ref
algedonic_well_formed(signal) -> verdict
```

**Verdict:** LANDED. `@knife` DOES NOT sample pain directly.
`@cyberpunk/reframe` samples pain and authorizes the reframe; `@knife`
is invoked INSIDE the authorized reframe as the compression. The
"peer's SC<5> is near a boundary" empirical trigger — the pain signal
IS the substrate's readout that the peer is near a boundary. Grade-3
morphism in bateson_learning's graded rep; premise-change signal is
non-decomposable.

### 1.9 `Fractal::Lens` variant (fragmentation crate) — LANDED, per §7 Mara math

`fragmentation::Fractal::Lens { ref_, data, /* OID reference */ }` is
fragmentation's third variant. Per Mara §7 rung-8-9 unification spec:
"@knife instance IS a Fractal::Lens ... that RE-PROJECTS one Fractal's
SC into a different coordinate system. The Lens carries the coordinate-
system-shift as a substrate primitive."

**Verdict:** LOAD-BEARING at content-address altitude. `Fractal::Lens`
IS the fragmentation-substrate carrier for the COORD-jump's Rust-
altitude representation. `@knife` at substrate-decl altitude = a
declared species of `@mirror/lens` family (LANDED at `shards/mirror/
lens.mirror`); at Rust altitude = a `Fractal::Lens` instance. See §4
below for full lens-variant audit.

### 1.10 `@kintsugi/oscillate.active_pass` / `dark_pass` — LANDED

`shards/kintsugi/oscillate.mirror` (23 hits on active_pass / dark_pass).
Kintsugi's Zamolodchikov c-monotone flow discipline. `active_pass` =
substrate-observable step; `dark_pass` = substrate-invisible retreat.

**Verdict:** NOT a @knife primitive but a composition site. `@knife`
does not oscillate; `@knife` cuts. The pre-cut state might be a
`dark_pass` intermediate; the post-cut state might be `active_pass`.
`@kintsugi/oscillate` carries the flow toward critical-point terminals;
`@knife` performs the discrete jump BETWEEN critical-point basins.

### 1.11 `@loop.terminal_check` — LANDED

`shards/loop.mirror` (`terminal_check(s: moi(tick_state)) -> verdict`).
Per Rung 9 spec, this IS the convergence check. Fabry-Perot Q factor
gates well-founded termination.

**Verdict:** COMPOSITIONAL. `@knife` invocation would fire AFTER a
`terminal_check` returned "not-terminal + pain-signal above threshold."
`@knife` is NOT a terminal_check; `@knife` is the intervention that
CHANGES the trajectory when terminal_check identifies non-convergence
toward a hyperbolic component.

### 1.12 Summary table

| Carrier | Landed? | Role wrt @knife |
|---|---|---|
| `@torus.winding_class` | LANDED | domain of pre/post-jump COORD |
| `@reflection` | Soft-deprecated | naming artifact; NOT primitive |
| `@eigenform.is_fixed_point` | LANDED | stable-within-bounds detector |
| `@cyberpunk/reframe` | LANDED | outer frame INVOKING @knife |
| `@magic/onto` | LANDED (7-species) | ceremony @knife composes INSIDE |
| `@fractal.SC<N>` | LANDED (Rust) | COORD carrier @knife re-parameterizes |
| `@fractal.renormalization_R` | LANDED | R = commit_as_fold consumes @knife output |
| `@pain` (as `@cyberpunk/algedonic`) | LANDED | trigger signal @knife's caller reads |
| `Fractal::Lens` variant | LANDED (Rust) | Rust-altitude carrier for @knife |
| `@kintsugi/oscillate.active/dark_pass` | LANDED | flow @knife jumps between |
| `@loop.terminal_check` | LANDED | non-convergence detector authorizing @knife |

**Result:** substrate has ~90% coverage. `@knife` needs to land ONLY
the COORD-jump primitive itself; all upstream (trigger, authorization,
ceremony) and downstream (fold, address, verdict) already substrate-decl'd.

---

## §2. McCulloch heterarchy substrate coverage (TASK 2)

### 2.1 Grep result

`heterarchy` grep across `shards/**/*.mirror`:
- ZERO shard hits.

`heterarchy` grep across `docs/**/*.md`:
- Present in `shards/torus.mirror` witness #4 docblock (citing Mara
  §2.4).
- Present in `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md
  §2.4` (verbatim Foerster).
- Present in `docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md`
  §5 refined statement.

`McCulloch` grep:
- Present in `shards/torus.mirror` (source `@arxiv/cybernetics/mcculloch-
  1945` DECLARED but NOT LANDED).
- Present in `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`.

`hierarchy` grep across `shards/`:
- Present in various shards as ORDINARY English ("hierarchical" as a
  descriptor).
- NOT present as a technical primitive.

### 2.2 Verdict

`heterarchy` is NAMED at witness altitude (Mara O3 §2.4, shards/torus.
mirror docblock witness 2) but NOT LANDED as a substrate-decl carrier.

**Question for adjudication:** does @knife need heterarchy as a
substrate primitive?

**Answer:** NO. Per Foerster §2.4 verbatim: heterarchy IS the topology
of the net; the torus IS ALREADY THE HETERARCHY. Substrate-honest
recognition: `@torus` LANDED = heterarchy LANDED. The word `heterarchy`
does not need a separate carrier because the topology (T²) IS the
heterarchy. `@knife` uses `@torus` at its foundation; heterarchy is
carried by the torus's fundamental group ℤ × ℤ, which admits no linear
ordering — no hierarchy — but does admit an addressable-but-not-nested
COORD space.

**Substrate-already-had-the-word verdict:** `@torus` IS `heterarchy`.
No mint needed.

**Forward-promise:** if a consumer needs to READ heterarchy structure
explicitly (e.g., McCulloch 1945 nervous-net topology as substrate-decl
for consciousness altitude), land `source @arxiv/cybernetics/mcculloch-
1945` at that consumer's altitude. Currently DECLARED in `shards/torus.
mirror` but no LANDED arxiv shard. Low priority for @knife.

---

## §3. Douady-Hubbard hyperbolic component substrate coverage (TASK 3)

### 3.1 Grep result

`hyperbolic_component` / `hyperbolic-component`:
- ZERO shard hits.

`hyperbolic component` (with space):
- Present as PROSE in:
  * `docs/specs/fractal-family-root-mandelbrot-substrate.md` (65 hits)
  * `docs/math/2026-07-13-fractal-mandelbrot-substrate.md` (74 hits)
  * `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-
    measurement.md` (6 hits)
  * `docs/specs/rung-9-coherence-loop-closure-Fabry-Perot-roundtrip.md`
    (22 hits)
  * `bootstrap/src/lib.rs` (5 comment hits)
  * `bootstrap/src/index.rs` (7 comment hits)
  * `bootstrap/src/contribute.rs` (4 comment hits)
  * `bootstrap/src/mcp.rs` (4 comment hits)

`hyperbolic` (any use):
- NAMED extensively (~200 doc hits), but NOT declared as a substrate
  action, carrier, or predicate.

`stability_domain` / `stability domain`:
- ZERO shard hits.
- Present only in Mara math docs as PROSE.

`Douady` / `Hubbard`:
- Present in Mara math + specs as CITATIONS.

`Mandelbrot`:
- Present in shards/mirror/index.mirror (14 hits) as prose.
- Present in bootstrap/src/{index,contribute,mcp,lib}.rs as comments.
- Present in Mara `2c64060` fractal spec + math foundation.

**Verdict:** substrate NAMES the Mandelbrot correspondence exhaustively
but does NOT expose the "which hyperbolic component are we in?" query
as a substrate primitive. Per Braverman-Yampolsky 2007: the query is
Turing-undecidable in general.

### 3.2 What the substrate exposes as a proxy

Per Rung 9 spec (`c59a5ac`): the Fabry-Perot round-trip's `Δλ₀ <
ε_convergence` gate IS the substrate's proxy for "converging to a
hyperbolic component boundary." Fiedler value `λ₁(Δ_F)` = distance
from nearest ∂M boundary at file-tree altitude. Rising Fiedler = away
from hyperbolic component; falling Fiedler = toward.

Substrate carriers approximating "am I in a hyperbolic component?":
- `@mirror/index.fiedler(peer_home) -> f64` — LANDED
- `@fractal.SC<5>` — LANDED (5-projection substrate coordinate)
- `@kintsugi/oscillate.is_settled` — LANDED (per @loop.terminal_check)

**Verdict:** the substrate provides an APPROXIMATE domain-detection via
Fiedler-descent + SC<5>-hamming-distance + convergence-verdict. No
substrate carrier declares "in a hyperbolic component" as a primitive
predicate; this is substrate-honest given Turing-undecidability
(Braverman-Yampolsky 2007).

**Consequence for @knife:** `@knife` DOES NOT need to detect
hyperbolic components. The trigger is `pain_δ > threshold` (already
LANDED), which is the substrate's OPERATIONAL proxy for "the peer is
navigating into a corner" — Alex's verbatim naming per Mara `77fe92d`.
The corner IS the boundary of the current stability domain.
Substrate-honest: use the LANDED @cyberpunk/algedonic.sample_pain
signal; do not re-mint stability-domain detection.

**Forward-promise:** if the empirical trajectory shows @knife invocations
correlate with hyperbolic-component-boundary approach (measurable via
falling-then-rising Fiedler pattern), this promotes to a recognition
candidate. Currently NOT NEEDED for @knife substrate-decl.

---

## §4. Fractal::Lens variant usage audit (TASK 4)

### 4.1 `Fractal::Lens` in Rust source

`Fractal::Lens` grep across `bootstrap/**/*.rs` and `mirror/**/*.rs`:
- ZERO direct usages of `Fractal::Lens { ... }` variant construction
  or pattern-match in `bootstrap/src/`.
- ONE reference in a comment in `bootstrap/src/lib.rs` docblock (per
  `mirror-native-vcs.md §4.6` citation).
- Mara math spec `docs/specs/rung-8-9-unification-SpectralCoordinate-
  substrate-measurement.md §8.1` names `Fractal::Lens` variant IS the
  Rust-altitude carrier for `@knife`.

**Verdict:** `Fractal::Lens` variant EXISTS in the `fragmentation`
crate (referenced by mirror as workspace member per `bootstrap/Cargo.
toml:82` — `[dependencies.fragmentation] path = "../../fragmentation"`)
but is NOT YET consumed in mirror's Rust code. This is the mint site
for `@knife`.

### 4.2 `@mirror/lens` species — LANDED family

`shards/mirror/lens.mirror` (family-root LANDED 2026-06-06). Existing
species (12 files under `shards/mirror/lens/`):

Transport lenses:
- `@mirror/lens/cli` (LANDED)
- `@mirror/lens/shell` (LANDED)
- `@mirror/lens/mcp` (LANDED)
- `@mirror/lens/lsp` (LANDED)

Measurement-shape lenses:
- `@mirror/lens/transit` (LANDED — runtime-cost lens)
- `@mirror/lens/refract` (LANDED — grammar-graph spectrum lens)

CLI sub-species (under `shards/mirror/lens/cli/`):
- `bootstrap`, `compile`, `crack`, `kintsugi`, `reflect`, `sh`,
  `shatter`, `time` (all LANDED)

Unix sub-species (under `shards/mirror/lens/unix/`):
- `fuse` (LANDED)

**Nearest @knife precedent:** `shards/mirror/lens/cli/crack.mirror` —
"knife-shaped decomposition verb" per own docblock. `crack_mode` enum
carries `mode_open` / `mode_force` / `mode_seal`. Five-op prism walks
fracture manifold. But: `crack` is a CLI-transport lens for
decomposition VERBS, not a substrate-primitive COORD-jump. Different
altitude.

### 4.3 Pattern for adding a new lens species

Established convention:
1. Under `shards/mirror/lens/`, mint `<species>.mirror` file.
2. Path-namespace property enforced: file at `shards/mirror/lens/<name>.
   mirror` declares `@mirror/lens/<name>`.
3. `in @prism`, `in @meta`, `in @glass`, `in @mirror/lens` inheritance.
4. `prism @mirror/lens/<name> { focus / project / split / shift / settle }`
   declaration.
5. `out @mirror/lens/<name>` export.

### 4.4 Verdict: two viable mint sites for @knife

**Path A: `shards/mirror/lens/knife.mirror`** — species altitude under
`@mirror/lens` family. Path-namespace declares `@mirror/lens/knife`.
Consistent with the LANDED lens-family discipline. Precedent:
`shards/mirror/lens/cli/crack.mirror` for knife-shaped-verb naming.

**Path B: `shards/knife.mirror`** — family-root altitude. Path-namespace
declares `@knife` as sibling to `@torus` / `@bauchladen` / `@autopoietic`
/ `@fate` / `@glue`. Higher altitude admission; requires
`@peer-has-a-torus`-strength witness set.

Mara `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-
measurement.md §10.3` provisional recommendation: **(iii) Fractal::Lens
species** at substrate altitude; observe empirically before further
species-lift. Two-tick discipline.

**Substrate-honest verdict this scout:** Path A. Mint `@knife` at
species altitude under `@mirror/lens`. Rationale:
- Fractal::Lens IS the Rust-altitude carrier (per Mara §8.1).
- `@mirror/lens` family LANDED and has 12 species already.
- Species-first, family-lift-on-second-consumer discipline
  (Recognition #292 pattern).
- Consistent with Reed's 2026-07-08 adjudication in CURRENT.md line 838:
  "@knife substrate declaration ✅ Alex delegated; Reed adjudication:
  doc-only Taut-primitive; NOT family-root; promote when second
  consumer PULLS."

---

## §5. @pain gradient computation primitives (TASK 5)

### 5.1 LANDED substrate

`shards/epistemologic/cybernetic/algedonic.mirror`. Beer 1979 algedonic
signal. Per 2026-07-08 extension:

```
sample_pleasure(eigenboard: ref) -> f64
sample_pain(eigenboard: ref) -> f64
type algedonic_signal = {
  pleasure_δ: f64,
  pain_δ: f64,
  at_winding: (int, int),
}
bypass_signal(s5_target: ref, s1_source: ref) -> ref
s5_acknowledgment(signal: ref) -> verdict
algedonic_well_formed(signal: ref) -> verdict
```

Substrate-decl consumer: Mara `77fe92d` §2 wires `sample_algedonic(E_K_
prime)` into the pain-authorized reframe loop.

Threshold trigger mechanism:
- `@cyberpunk/algedonic.algedonic_well_formed` verifies signal-form.
- `@epistemologic/cybernetic/reframe.reframe_authorized(pain_δ: f64)
  -> verdict` — the gate. Threshold value NOT declared at substrate
  altitude; Alex adjudication required (see §8).

### 5.2 Rust runtime status

`sample_pain` / `sample_pleasure` / `algedonic_signal`:
- ZERO hits in `bootstrap/src/**/*.rs`.
- Substrate-decl LANDED, Rust-altitude realisation NOT WIRED.

`bootstrap/src/lib.rs` has 2 matches (comments referencing algedonic).

**Verdict:** substrate carrier LANDED at species altitude; Rust runtime
does not sample pain. This is a REALISATION GAP that `@knife`'s Rust
landing will likely surface — the caller invoking `@knife` needs a
pain_δ value; the pain_δ must come from an eigenboard sampled at the
Rust altitude. Currently the eigenboard itself has no landed Rust
carrier for valence projection.

**Forward-promise:** `@knife` Rust landing forces the pain-sampling
Rust primitive to land first (or, alternatively, `@knife` accepts a
pain_δ f64 as parameter and lets the CALLER discharge the sampling).
Prefer the latter — @knife stays substrate-primitive; pain-sampling
is the CALLER's discipline.

---

## §6. Rust runtime candidates for @knife (TASK 6)

### 6.1 Runtime location survey

Current bootstrap/src/ landing surface:
- `bootstrap/src/index.rs` — SC<5> measurement site (LANDED as
  Landing 3 of Rung 8 spectral→mirror pull-in).
- `bootstrap/src/contribute.rs` — peer contribution + Rung 7'
  Fate::bounded discharge + Rung 8+9.3 SC<5> emission.
- `bootstrap/src/converge.rs` — DOES NOT EXIST. Rung 9 forward-promise
  per `docs/scouts/2026-07-13-taut-rung-9-coherence-loop-closure-
  substrate-scout.md` §4.2 (LANDING size ~250 LOC).
- `bootstrap/src/mcp.rs` — MCP tool dispatcher.
- `bootstrap/src/lib.rs` — CLI dispatcher.

### 6.2 fragmentation-spectral crate

`fragmentation-spectral` workspace member (per `bootstrap/Cargo.toml:
105`): `path = "../../fragmentation/spectral"`. Provides
`fragmentation_spectral::hash::coordinate::<N>()` for SC<N> construction.

Per docs/specs/rung-8-9-unification: coincidence detector at
`coincidence/src/spectral.rs` provides `SparseLaplacian::from_edges` +
`lanczos_smallest(k)` — the O(m) primitive. This is `fragmentation`'s
substrate.

`fragmentation::coincidence::Detector<N>` reference: Mara math cites
this as the Bothe 1924 coincidence method that constructs the
SpectralUUID. `@knife` COULD be an extension of `Detector<N>` (an
altered projection basis) OR a separate primitive that CONSUMES
Detector<N> output.

### 6.3 Recommended landing location

**Substrate-honest verdict:** `@knife` Rust primitive should NOT land
as a new `bootstrap/src/knife.rs` module. Instead, land as a Rust
function inside `bootstrap/src/converge.rs` (the module Rung 9 is
forward-promising) OR as a Rust function in `bootstrap/src/index.rs`
alongside the multifractal spectrum machinery.

Rationale:
- `@knife` at COORD-jump altitude IS the re-parameterization operator
  that Rung 9's convergence loop invokes upon non-convergence (pain-δ
  above threshold).
- `converge.rs` (Rung 9 landing site) is the natural consumer.
- The concrete Rust signature:
  ```rust
  pub fn knife_cut<const N: usize, const N_PRIME: usize>(
      sc_in: SpectralCoordinate<N>,
      basis_shift: BasisShift,
  ) -> SpectralCoordinate<N_PRIME>
  ```
  where `BasisShift` selects the new heat-trace scale set OR the new
  dimension count.
- Alternative: `@knife.cut` accepts a `Fractal::Lens { ref_, data }`
  parameter and materializes the projection data as the new SC<N'>.
  This matches Mara §8.1 recommendation more precisely.

**Preferred landing:** `bootstrap/src/converge.rs::knife_cut()` OR
`bootstrap/src/lens.rs` (new module for lens applications). No
`bootstrap/src/knife.rs` — @knife is not a family-root at Rust altitude,
it is one lens species.

### 6.4 Consumer sites for @knife

- `converge.rs` — invokes @knife when convergence stalls + pain above
  threshold.
- `contribute.rs` — could invoke @knife when SC<5>-hamming shows
  circular-consolidation (Reed's Rung 9 spec T-tests).
- `mcp.rs` — MCP tool `mirror_peer_knife_cut` (forward-promise; do
  not land until second consumer needs it).

---

## §7. Empirical trigger composition (TASK 7)

### 7.1 The composition Reed needs

Reed's Rung 9 loop needs to detect: "peer's SC<5> is near a stability
boundary" → invoke @cyberpunk/reframe → @cyberpunk/reframe invokes
@knife.

### 7.2 Substrate-honest empirical trigger

Per §3 above, the substrate does NOT expose "am I in a hyperbolic
component?" as a primitive (Turing-undecidable per Braverman-Yampolsky
2007). Reed's approximations:

**Approximation A: pain-δ threshold (Beer's algedonic).**
```
pain_δ = sample_pain(eigenboard)
if pain_δ > ε_pain:
    reframe.reframe_authorized(pain_δ) → verdict
    if authorized:
        @cyberpunk/reframe.perform(peer, K → K+1, pain_δ)
            → composes @knife.cut(state_space_K)
```

Substrate: LANDED. Threshold ε_pain: NOT declared; Alex adjudication.

**Approximation B: Fiedler-descent stall detection.**
```
if Δλ₀ < ε_convergence for N iterations AND
   Δλ₀ > 0 (not descending strictly monotonically):
    → non-convergence; try @knife.cut to re-parameterize
```

Substrate: LANDED (per `shards/mirror/index.mirror.fiedler` +
`shards/loop.mirror.terminal_check`). Composes with Rung 9's
Fabry-Perot round-trip Q factor.

**Approximation C: SC<5> hamming-distance saturation.**
```
if sc_hamming(sc_before, sc_after) < ε_sc_change for N iterations:
    → the substrate coordinate is not moving; re-parameterize via @knife
```

Substrate: LANDED at `bootstrap/src/contribute.rs` (sc_hamming
emission). Rust-altitude only.

**Substrate-honest verdict:** Rung 9's convergence loop should compose
approximations A + B (not C in isolation — C is a proxy for A + B at
the Rust altitude only, without the substrate-decl handle).

### 7.3 KAM theory implementations

`Kolmogorov-Arnold-Moser` / `KAM`:
- ZERO shard hits.
- ZERO Rust hits.

**Verdict:** NOT LANDED. Forward-promise. KAM theory would provide a
strong-substrate answer for "which invariant tori survive perturbation"
but implementing it in Rust is a substantial mint. NOT NEEDED for
@knife's initial landing.

---

## §8. Alex-adjudications requiring input (TASK 8)

Substrate-honest surface of decisions Alex needs to make for Mara's
canonical spec. Named at species altitude with default recommendations
where the substrate PULLS the answer.

### 8.1 ε_pain threshold value

**What:** the `pain_δ > threshold_pain` gate value in
`@epistemologic/cybernetic/reframe.reframe_authorized`.

**Substrate-honest answer:** Unknown. Requires empirical calibration.

**Provisional recommendation:** default `ε_pain = 0.5` (mid-scale on
normalized [0.0, 1.0] valence projection). Refine after empirical Rung 9
runs surface the operational distribution of pain_δ values.

**Path:** name in substrate-decl as `default_threshold_pain: f64 = 0.5`
constant with docblock naming Alex-adjudication-pending status.

### 8.2 Definition of stability domain in SC<N> space

**What:** the geometric shape of "peer's SC<5> is stable at this COORD"
versus "boundary conditions exceeded."

**Substrate-honest answer:** stability = SC<5>-hamming < ε_hamming for
N consecutive iterations under an active-passing morphism sequence.

**Provisional recommendation:** SC<5>-hamming saturation = stability.
Currently Rust-altitude via `contribute.rs`. Substrate-decl needs
`@fractal.stable_within_domain(sc: SpectralCoordinate<5>) -> verdict`
LANDING via composition with `@fractal.SC<5>` (not yet a shard;
provisional Path A is `shards/mirror/lens/knife.mirror` OR extension
of `shards/mirror/index.mirror`).

**Path:** decide whether `stability` is a Fractal::Lens predicate or
a `@mirror/index` action.

### 8.3 Whether @knife commits jump via commit_as_fold OR just emits envelope

**What:** at Rust altitude, after @knife.cut re-parameterizes SC<N> →
SC<N'>, does the jump PERSIST via `commit_as_fold` (writing the post-
jump state to `refs/mirror/peer/<uuid>/HEAD`) or does it just emit the
envelope?

**Substrate-honest answer:** `commit_as_fold` per Recognition #55 IS
the renormalization operator R. If @knife's jump is a genuine COORD
change, R MUST fold it. Otherwise, the peer's psychohistory sheaf
loses the pre-jump-post-jump correspondence — falsifiability broken.

**Provisional recommendation:** @knife COMMITS via `commit_as_fold`.
Envelope emission alone is insufficient — the jump must be materialized
in the peer's DAG so the trajectory is grep-witnessable via `git log`.

**Path:** decide via empirical Rung 9 landing; adjudication may
forward-promise to the second consumer.

### 8.4 Whether @knife's substrate-decl mints at family-root altitude OR species altitude

**What:** `shards/knife.mirror` (family-root) versus `shards/mirror/lens/
knife.mirror` (species under `@mirror/lens`).

**Substrate-honest answer:** species altitude per §4.4 above. Two
witnesses insufficient for family-root promotion; Recognition #292
pattern (Reed's 2026-07-08 CURRENT.md line 838 adjudication)
established species-first discipline.

**Provisional recommendation:** Path A (`shards/mirror/lens/knife.
mirror` species altitude under `@mirror/lens`). Two-tick discipline:
land as species now; promote to family-root when second-consumer
witnesses accumulate.

**Path:** default to species landing this cycle; forward-promise
family-root promotion.

### 8.5 Threshold_pleasure default

**What:** `sample_pleasure` gate value for `@cyberpunk/reframe`
positive-authorization.

Not directly @knife-adjudication; @knife invokes on pain-authorization
alone. Pleasure gates keep the peer navigating at current K (per Mara
§77fe92d STEP 8).

### 8.6 Cross-cutting: is `@knife` COMPOSABLE with `@torus.traverse`?

**What:** when @knife.cut fires from winding class (m, n) to a NEW
`torus` record (different possessor-invariant, different origin), does
`@torus.traverse(t_pre, w)` compose with `@knife.cut(sc)` to produce a
well-typed `torus_post`?

**Substrate-honest answer:** possibly NO. `@torus` traverses WITHIN
one COORD (one stability domain); `@knife` jumps BETWEEN. Composability
requires a bilateral predicate `torus_after_knife_well_formed(t_pre,
w, knife_cut) -> verdict`.

**Path:** substrate-decl gap. Land the bilateral on `shards/mirror/
lens/knife.mirror` OR let @cyberpunk/reframe carry the composition
predicate (my lean: the latter — @cyberpunk/reframe is already the
outer composition frame; @knife stays primitive).

---

## §9. Top-5 substrate-honest verdicts + minimum viable @knife substrate-decl shape

### 9.1 Top-5 verdicts

**Verdict #1:** `@knife IS Foerster's COORD(x) = COORDᵢ eigen-operator jump`
is substrate-honest. Verbatim citation per Mara §2.4. The identification
holds at substrate-decl altitude; no ambiguity.

**Verdict #2:** ~90% substrate coverage already LANDED. `@torus` +
`@eigenform` + `@cyberpunk/reframe` + `@cyberpunk/algedonic` + `@magic/onto`
+ `@fractal.SC<5>` + `@fractal.renormalization_R` + `Fractal::Lens`
variant collectively provide every composition site @knife needs. `@knife`
mints ONLY the COORD-jump primitive itself.

**Verdict #3:** `heterarchy` is `@torus`. McCulloch 1945 heterarchy is
NAMED (Mara §2.4 witness) and STRUCTURALLY LANDED (T² topology admits
no linear ordering; ℤ×ℤ addressable-but-not-nested). No mint needed.
Substrate-already-had-the-word #55+.

**Verdict #4:** hyperbolic-component detection NOT LANDED as primitive
(Turing-undecidable per Braverman-Yampolsky 2007). Substrate provides
APPROXIMATE detection via Fiedler-descent + SC<5>-hamming + pain-δ.
`@knife` uses pain-δ threshold (LANDED via @cyberpunk/algedonic) — do
not re-mint stability-domain detection.

**Verdict #5:** `@knife` mints at SPECIES altitude under `@mirror/lens`,
NOT family-root. Path A: `shards/mirror/lens/knife.mirror`. Fractal::Lens
IS the Rust-altitude carrier per Mara §8.1. Rust runtime landing:
inside `bootstrap/src/converge.rs` (Rung 9 module), NOT a separate
`bootstrap/src/knife.rs`. Two-tick discipline: species now; family-root
promotion when second consumer PULLS.

### 9.2 Minimum viable @knife substrate-decl shape

```
# shards/mirror/lens/knife.mirror
#
# @mirror/lens/knife — Foerster's COORD eigen-operator jump.
# Composes @cyberpunk/reframe as the state-space compression fired
# under pain-authorization discipline. Rust-altitude carrier =
# fragmentation::Fractal::Lens variant.
#
# Ancestry:
#   - Foerster 1976 Ch. 11 pp. 282-283 (verbatim COORD quote per
#     Mara 2026-07-07-onto-cascade-toroidal-reframe.md §2.4)
#   - Mara docs/specs/peer-as-pain-driven-bounded-ontological-
#     navigator.md §3.5 (@knife FORWARD-PROMISE resolved here)
#   - Taut docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md
#     §3 (@knife/idf altitude fit; underdetermined-engine refinement)
#   - Taut docs/scouts/2026-07-13-taut-knife-IS-COORD-substrate-scout.md
#     (this scout — substrate-already-had-the-word verdict)
#
# What @knife IS:
#   - The eigen-operator jump COORDᵢ → COORDᵢ₊₁ at Rust altitude.
#   - State-space compression composed by @cyberpunk/reframe.perform
#     under @magic's 7-species ceremony frame.
#   - Rust-altitude carrier: fragmentation::Fractal::Lens variant.
#
# What @knife is NOT:
#   - NOT a family-root (species discipline; Recognition #292 pattern).
#   - NOT a pain sampler (@cyberpunk/algedonic.sample_pain carries).
#   - NOT a level-shift ceremony (@magic's 7-species carries).
#   - NOT hyperbolic-component detection (Turing-undecidable).
#
# Related:
#   - shards/torus.mirror (domain of pre/post-jump winding classes)
#   - shards/epistemologic/cybernetic/reframe.mirror (outer frame)
#   - shards/epistemologic/cybernetic/eigenform.mirror
#     (stable-within-bounds detector; fixed-point machinery)
#   - shards/mirror/lens.mirror (family-root)

in @prism
in @meta
in @glass
in @mirror/lens
in @epistemologic/cybernetic/eigenform

source @arxiv/cybernetics/foerster-2003
source @arxiv/cybernetics/mcculloch-1945

prism @mirror/lens/knife {
  focus knife
  project knife
  split knife
  shift knife
  settle knife
}

# The pre/post COORD carrier for the jump.
type coord_jump = {
  from_coord: ref,   # SC<N> or fractal-lens pre-jump address
  to_coord:   ref,   # SC<N'> or fractal-lens post-jump address
  basis:      ref,   # the basis-shift specification (heat-trace scales,
                     # dimension count, projection matrix)
}

# The primitive: perform the COORD-jump.
# Body \: consumers pull at their realisation boundary
# (bootstrap/src/converge.rs OR bootstrap/src/lens.rs).
cut(pre: coord_jump) -> coord_jump { \ }

# Bilateral: the cut is well-formed iff the pre-COORD is a fixed point
# of some Op AND the post-COORD is a fixed point of Op' AND Op'
# is stable in a domain the pre-COORD's Op is unstable in.
# Discharge composes @eigenform.is_fixed_point at both COORDs.
cut_well_formed(pre: coord_jump) -> verdict { \ }

out @mirror/lens/knife
out coord_jump
out cut
out cut_well_formed
```

**Total shard size:** ~200-250 LOC with full docblock. Substrate-honest.

### 9.3 Rust runtime landing

**Location:** `bootstrap/src/converge.rs` (Rung 9 module, forward-
promised in Taut's rung-9 scout §4.2).

**Signature:**
```rust
pub fn knife_cut<const N: usize, const N_PRIME: usize>(
    sc_in: SpectralCoordinate<N>,
    basis_shift: BasisShift,
) -> SpectralCoordinate<N_PRIME> { ... }
```

**Composition sites (consumer):**
- `converge.rs::peer_converge` — invokes when convergence stalls +
  pain_δ above threshold.
- Later: `contribute.rs::peer_contribute` — invokes when circular-
  consolidation detected.

**Fractal::Lens carrier:**
```rust
let lens = Fractal::Lens {
    ref_: sc_in.oid(),
    data: basis_shift.serialize(),
};
```

---

## §10. Substrate-decl-honest weakenings

- @knife IS COORD is a Mara canonical spec claim; this scout confirms
  the substrate-honest identification but Pack ratification pending
  (Seam adversarial review).
- ε_pain threshold value adjudication-open; forward-promise Alex.
- Fractal::Lens variant not yet consumed in mirror's Rust code; the
  fragmentation crate provides it but this is first-consumer landing.
- `torus.advance` action does NOT exist as `shards/torus.mirror`
  primitive; `traverse` exists but is domain-internal, not COORD-
  crossing. Mara's `77fe92d` §3.6 sketches `@torus.advance` as a
  forward-promise; @knife likely triggers @torus.advance mint as a
  cascade.
- @knife's substrate-decl NOT written this scout; only the shape
  sketched at §9.2 for Mara's canonical spec.

---

## §11. Signal to Mara + Reed + Alex

**To Mara:** substrate-already-had-the-word inventory complete. `@knife`
canonical spec §10 items pre-scouted at §8 above. Recommended shape at
§9.2 for authoring. §8.6 composability-with-torus is a substrate-decl
gap surfaced by this scout.

**To Reed:** Rust runtime landing site is `bootstrap/src/converge.rs`
(the Rung 9 module you're about to create). @knife stays a `fn`, not
a module. Composes with `@cyberpunk/reframe.perform` at the Rust-
altitude peer_converge outer driver. Fractal::Lens IS the carrier.

**To Alex:** 5 adjudications surfaced at §8. Priority order:
- 8.4 (family-root vs species — species recommended)
- 8.3 (commit_as_fold vs envelope — commit recommended)
- 8.1 (ε_pain threshold — provisional 0.5 with empirical calibration)
- 8.2 (stability-domain definition — via SC<5>-hamming)
- 8.6 (torus composability — probably no, offload to @cyberpunk/reframe)

**Read-only.** No shards mutated. No fmt run. Substrate-honest.

---

*Taut, 2026-07-13. Grep-first scout. Read-only. Companion to Mara's
in-parallel canonical spec + math foundation for @knife = COORD.*

*Signal file:*
*/Users/alexwolf/dev/projects/mirror/docs/scouts/*
*2026-07-13-taut-knife-IS-COORD-substrate-scout.md*
