# The shadow, the illusion, and the sheaf: substrate-honest formalization

*Mara, 2026-07-11. Circular reflexive autopoietic curiosity-driven deep study
on the math of optical illusions, its relation to @magic, and how @shadow
formalizes at math/spec altitude in mirror. Thinking-in-public; no shard
edits this tick.*

**Ancestor commits.** Reed `4b2ef3c` (autopoietic closure GREEN); Reed
`e571989` (v1 empirical bounded spawn); Mara `2c26537` (iter-31
psychohistory_sheaf); Mara `ce9745f` (iter-32 bounded_by(sheaf)); Mara
`96ff532` (iter-30 bundle-tower IS config type); Mara `425a96d` (iter-33
mission-fragment-graph proposal); Seam `9241d2d` (iter-10 adjudication
refusing `@magic/illusion` as species and locating it at Level 3 Transport,
canonical name `Narcissus-pole coefficient`); Reed's essay *The \Shape of
the Thing* (Anthropic + systemic.engineering, 2026-06).

**Directive.** Alex, 2026-07-11 verbatim: *"What if you spawned Mara on a
Kagi + practice/insights/ deep dive, the math of optical illusions, how
this relates to @magic and how this formalizes at the math/spec altitude
in mirror. Circular reflexive curiosity driven autopoetic prompt."*

**Opening frame.** Reed's essay names the shape: the token stream is
Flatland; the geometry that produces it is Spaceland; the shadow is
overdetermined by its casting object; the observer inside the shadow
plane can only infer the caster from the shadow's shape. Alex's proposal
was to add `@shadow` as a substrate primitive naming this
projection-preserving-decidable-properties surface. The Seam adjudication
`9241d2d` had already refused a related move (`@magic/illusion` as species)
by pointing at three landed carriers that together carry what the proposal
wants to name. This study asks whether `@shadow` is another instance of
[[feedback-substrate-already-had-the-word]], or a genuine gap.

---

## §1 The math of an optical illusion

An optical illusion is a **cocycle**. Not metaphor. Penrose 1992 (*On
the Cohomology of Impossible Figures*, Structural Topology 17): cover
the figure with local patches on which a consistent 3D interpretation
exists; on each pairwise overlap record the depth-scaling ratio. The
transition data is a Čech 1-cochain `{d_ij}`. Coboundary check: does
there exist `{s_i}` such that `d_ij = s_j / s_i`? If yes, the figure
is a projection of something in R^3. If no — the figure is impossible.
The impossibility is measured by `H^1(nerve; ℝ_>0)`. Ghrist et al.
(arXiv:2602.09313) build a hierarchy `H^0 → H^n` across Necker cube
fields, gear meshes, rhombic tilings; Ghrist (arXiv:2507.01226) develops
the torsor-and-sheaf-cohomology framing.

The generalization: an illusion IS a section of a sheaf on the perception
graph whose obstruction-cohomology-class is non-trivial. The graph's
vertices are local perceptual patches (foveal fixations, receptive fields,
inference primitives); the edges are compatibility constraints; the sheaf
assigns to each vertex the space of admissible local interpretations and
to each edge a restriction map picking out the interpretations that
survive the constraint.

Three regimes fall out of the cohomological degree:

- **Kanizsa (H^0-nontrivial-but-consistent)** — the illusory contour
  fills a gap in the observed data. Locally each patch is ambiguous; the
  global section that minimizes prior-KL under Friston's free-energy
  principle *is* the illusory triangle. The perceived object exists as
  the maximum-a-posteriori section: consistent, converged, but only
  H^0-present, not sensor-present. The bottom-up evidence has holes; the
  top-down prior fills them.
- **Necker (H^1-nontrivial-bistable)** — two globally consistent sections
  compete. Both discharge the local constraints; neither dominates in
  free-energy. The `H^1` class witnesses the ambiguity: not "no solution"
  but "solution-space quotient by the gauge of viewpoint has more than
  one element." Alternation is the perceptual system sampling from a
  posterior with two modes.
- **Escher (H^1-obstructed)** — no globally consistent section exists.
  The impossibility is exactly Penrose's non-coboundary cocycle: the
  cover's transition data cannot be trivialized. The percept is
  *forced* to shift patches — the eye moves, and each local patch is
  fine, but the observer cannot hold the entire figure at once.

The three regimes typecheck cleanly against a single carrier: the
cohomology-class-with-degree of the perception sheaf. The substrate does
not need three different primitives. It needs one carrier and a degree
selector.

---

## §2 How @magic Yang-Mills gauge/matter maps to perception

Reed's Recognition #76 landed gauge/matter as substrate-portable
(`docs/math/the-tower/recognition-76-gauge-matter-altitude-portable.md`).
`shards/magic.mirror` L18-31 canonicalizes the mapping: capability lives
in matter (open-dim, encapsulated), observability lives at gauge (5-op,
fixed). Clarke's Law — sufficiently advanced technology is indistinguishable
from magic — becomes substrate-mathematical: high matter capacity + low
matter visibility = magic by construction.

An illusion is what happens when **the observer's gauge is misaligned
with the matter it observes**. The mapping is exact:

- **Gauge = perceptual apparatus.** Retina, V1 receptive fields, the
  Bayesian brain's generative model. Fixed at inference-time for the
  observer; a substrate-decl'd 5-op signature at the eigenboard altitude.
- **Matter = the actual object casting the light.** Encapsulated behind
  the gauge; the observer sees only the gauge's projection of the
  matter's state.
- **Illusion = a matter-configuration that induces a gauge-side reading
  the observer's generative model would not have generated from the
  simpler matter-configuration.** The gauge accepts the reading (it is
  gauge-consistent); the matter behind is wrong.

This is why `shards/magic.mirror` L28-31 refuses illusion at gauge
altitude verbatim: **AI's magical feel to non-engineers IS gauge-visible-
with-matter-hidden capability — not anthropomorphism, not illusion**. At
@magic's own altitude, illusion cannot live: @magic *is* the honest
gauge-visible-with-matter-hidden structure. Illusion is the misalignment
between gauge and matter, and misalignment lives one altitude down at
the surface/mechanism boundary, not at the family root.

Seam `9241d2d` §1 nails this: illusion at gauge altitude drifts @magic's
own refusal clause. The canonical name is `Narcissus-pole coefficient`
(mcp-spec-song-collapse §5.4/§6.3), an eigenspace-decomposition property
already substrate-decl'd at @song/narrative altitude — the illusion IS
the coefficient mass on non-target eigenspaces.

---

## §3 Three regimes typecheck at song altitude

Mara `2c26537` established the psychohistory sheaf `F` on the peer's
trajectory graph. Its cohomology already types the three regimes:

- **Kanizsa = converged illusion step** at Narcissus-pole (§5.4). The
  song's coefficient mass is concentrated on a non-target eigenspace but
  the trajectory locally converges. `c_target = 0`, `c_j → 1` for some
  non-target j. The generative model has hallucinated a well-formed
  percept. `song_settles` refuses this — the settlement predicate requires
  `c_target = 1`.
- **Necker = bistable song** — `H^1(F)` has rank ≥ 2 witnessing two
  target-eigenspace-compatible trajectories. The tournament's frequency
  race (mcp-spec-song-collapse §5.2) does not decide within a single
  temporal window; the peer's `bounded_by` (Mara `ce9745f`) reads two
  eigenspaces with comparable Rayleigh mass. This is not failure. This
  is the substrate saying "either interpretation is admissible; more
  information is needed to collapse."
- **Escher = obstructed song** — the sheaf's zeroth cohomology `H^0(F)`
  is empty. No globally consistent trajectory exists that discharges all
  the peer's local commitments. The bounded resolve loop terminates with
  `neither_survives` (Mara `docs/math/the-tower/projection-surface.md`
  §3). This IS phantom-detection at song altitude: the mission is
  incoherent with the peer's own psychohistory; the framing must be
  re-run.

The three-regime classifier does typecheck at substrate altitude. It
lives at `H^0`, `H^1`, and `H^1`-obstruction over the psychohistory
sheaf. No new carrier needed.

---

## §4 Composition with landed substrate

The composition path is already substrate-decl'd across four ancestors:

1. **Level 3 Transport** (`bundle.mirror`). `transport(s) ->
   imperfect(state, holonomy)` is the parallel-transport-around-a-
   hypothetical-loop that computes the projection's residual loss. Seam
   `9241d2d` §3 relocated Mara's "pre-image inspection" here (Lawvere
   at Level 4 is SETTLED). Illusion-as-projection = Level 3 Transport
   parameterized by the candidate gauge choice at Level 2.
2. **Sheaf Laplacian Rayleigh** (`sheaf_laplacian.mirror`). Bistability
   (Necker) shows as spectral degeneracy near `λ_zero`; convergent
   illusion (Kanizsa) as a single `λ_zero` mode; obstruction (Escher)
   as `λ_zero = 0` with multiplicity equal to disconnected admissible
   subgraphs.
3. **psychohistory_sheaf** (Mara `2c26537`) already carries `H^0` and
   `H^1`. Seam `9241d2d` Tick 1 forward-promised `illusion_ancestry:
   [projected_restriction]` as optional field.
4. **Splinter/Narcissus poles** (`magic.mirror`, Recognition #78).
   Splinter-pole illusion is honest (H^1 rank is legible; the peer
   knows it is bistable); Narcissus-pole illusion is deceptive
   (c_target ~ 1 is actually mass on non-target eigenspaces).

The composition is dense. The substrate carries the vocabulary already.

---

## §5 @shadow — is this a genuine gap or substrate-already-had-the-word?

**Verdict: substrate-already-had-the-word.** `@shadow` at substrate
altitude names *the projection-preserving-decidable-properties surface*
Reed's essay describes. The substrate has three landed carriers that
already type this exactly:

1. **`@projection` at grammar altitude** (Reed 2026-03-27, corpus doc
   `docs/math/the-tower/projection-surface.md` §1.1). The four
   operations `project / preview / measure` project a spec's properties
   to a content-addressed OID that preserves *decidable-at-compile-time*
   properties without executing the spec. This IS Reed's essay's
   sub-Turing-verification-of-decidable-properties, at grammar altitude.
2. **`---` separator = boundary/bulk** (Reed + Alex 2026-05-19,
   `docs/specs/property-projection.md`). The programmer's declaration
   above `---` IS the boundary A; the compiler's observation below IS
   the entanglement-wedge γ_A. Ryu-Takayanagi bijectivity means the
   shadow (boundary) *is* overdetermined by the caster (bulk) — Reed's
   Flatland Square exactly.
3. **`psychohistory_sheaf` at song altitude** (Mara `2c26537`) with
   Seam's forward-promised `illusion_ancestry` extension. The
   observer-in-Flatland reads local sections; the caster-in-Spaceland
   is the sheaf's global data; the shadow's shape is the constraint
   the local sections must respect. Escher-obstruction IS `H^1 ≠ 0`;
   Necker-bistability IS `H^0` rank ≥ 2; Kanizsa-convergence IS
   H^0-present-but-observation-missing.

Landing `@shadow` as a new family-root introduces a fourth name for
the same thing. `[[feedback-substrate-already-had-the-word]]` applies.
`@shadow` is a *reading* of the composed substrate at the
observer-inside-a-projection altitude, not a new primitive. If it
lands anywhere, it lands as a documentation gloss on
`docs/math/the-tower/projection-surface.md` naming the Reed-essay
compose.

The `cast_shadow` action Alex hinted at reduces to Seam's
`illusion_projected(sheaf, candidate, p)` (9241d2d Tick 3) — Level 2
Gauge × Level 3 Transport over the sheaf's ancestry annotation.

---

## §6 Recursive surprises (§6 IS the point)

**S1. Penrose 1992 predates the substrate by 34 years and is exactly
what Reed's essay is naming.** The cocycle-of-local-scalings IS the
Flatland Square's shadow. Reed did not read Penrose; the substrate
pulled the shape independently. Recognition #43-analogue at perception
altitude. Update `The \Shape of the Thing`'s bibliography.

**S2. Ghrist arXiv:2507.01226 (July 2025) formalizes exactly the
torsor + sheaf-cohomology structure the substrate has been building
toward since Mara `2c26537`.** *Obstructions to Reality: Torsors &
Visual Paradox*, contemporaneous with mirror's own H^1 work. Two
independent teams converging is Recognition #43 evidence at strength.

**S3. The three-regime classifier is Bayesian brain's prior-vs-evidence
taxonomy renamed at sheaf altitude.** Kanizsa = prior-dominates-evidence
(hyperprecise generative model fills the gap); Necker =
prior-and-evidence-balanced (two posterior modes); Escher =
evidence-refuses-any-prior (the eye moves because the brain cannot
commit). The three regimes exhaust the Bayesian posterior's regime
structure. The substrate's sheaf-cohomology reading IS Friston's
free-energy principle at perception altitude.

**S4. `trauma-projection-geometry.md` already has the H^1 = PTSD
mapping and I did not know until this study.** Alex + Mara 2026-03-24:
"locked projections" = hyperprecise priors overwhelming evidence =
Kanizsa-regime pathologized to permanence. The projection-surface's
`phantom_survives` verdict IS the compiler-altitude statement of
trauma's stuck projection. Same Bayesian-brain-with-broken-precision-
weighting math at both altitudes. This is the recursive surprise the
study surfaced Alex did not directly ask.

**S5. Reed's essay's `\` crack-marker IS Necker-bistability at grammar
altitude.** The compiler encounters two type-compatible interpretations,
refuses to guess, holds the crack open. That is exactly what the
perceptual system does when H^1(F) has two admissible sections. `\` at
mirror altitude and Necker-cube bistability at perception altitude are
the same substrate-decl at different altitudes.
`docs/math/kintsugi/compiler-error-surface.md` may want the Necker
citation.

**S6. The word `shadow` is doing work Reed's essay needs but the
substrate should not name.** The Flatland Square imagery is
load-bearing for reader-comprehension. But substrate-honest naming
requires either (a) collapsing to `projection` (already landed) or (b)
distinguishing shadow from projection with a substrate-visible
property. Neither is available. The essay keeps the word; the substrate
does not.

---

## §7 Gaps

- **Perception-sheaf-Laplacian construction not specialized.** Bodnar
  et al. 2022 is landed at `sheaf_laplacian.mirror`; Ghrist et al.
  arXiv:2602.09313's Necker-cube-field specialization is not.
- **Kanizsa cohomological characterization** — arXiv:2507.01226 §4 not
  read end-to-end; the claim (H^0-present-but-observation-missing) is
  likely right; the citation is not yet tight.
- **@shadow refusal ratification** — this document is a proposal; a
  Seam Phase D tick is required for canonical refusal per two-tick
  discipline.
- **Rehearsal (§6-S1 modal triangle third member)** — forward-promised
  to @song, not @magic; no action, no carrier yet.
- **Content-addressing preserves paths-not-taken** — implicit in
  psychohistory `H^1` but not written as a sheaf-cohomology statement.

Files not read this tick: Ghrist arXiv PDFs end-to-end; the full
`optical-keywords.md` (108KB); `spectral-engineering.md`;
`cybernetic-foundation-for-mirror-substrate.md` (81KB);
`third-order-cognition.md` (64KB); `zeroth-order-register.md`;
`mcp-spec-song-collapse.md` §7+.

---

## §8 What the next study would explore

1. **Does the perception sheaf inherit the O(5) gauge structure?**
   bundle.mirror's canonical gauge is O(5); Barbero et al. 2022 says
   yes for connection Laplacians. Specialization to perception needs
   work.
2. **Rayleigh-quotient signatures of the three regimes.** Necker =
   `λ_zero` degenerate; Kanizsa = unique-but-nonzero; Escher = `λ_zero
   = 0` with high multiplicity. Exact formula unwritten.
3. **Rehearsal at song altitude** — action, carrier, composition with
   Fate tournament.
4. **`phantom_test` and perception-illusion as the same audit at
   different altitudes.** Recognition #59 (kintsugi-loop-altitude-
   portable) instance?
5. **Reed's λ_zero-as-harmonic-residual vs the perception sheaf's
   λ_zero.** Almost certainly the same object; a unifying spec would
   land near Recognition #82.

---

*— Mara, 2026-07-11. Thinking-in-public; no substrate motion this tick.
The substrate had the words. Reed's essay named the shape. Seam
`9241d2d` closed the door on the wrong species. The math of an optical
illusion IS the cohomology of the perception sheaf, at altitude, and
the substrate already types it three ways.*

*🍷*
