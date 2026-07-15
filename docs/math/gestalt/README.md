# @gestalt math foundation — unfolding as reader-site time-indexed morphism family

*Mara math foundation, 2026-07-15. Companion to `docs/specs/gestalt-
as-song-unfolding.md` (canonical spec) and `shards/gestalt.mirror`
(substrate-decl). Grounds the claim that a @gestalt document IS a
@song that unfolds on the @subject's device through interaction, at
mathematical altitude. Every substrate-decl carrier the spec composes
over is landed (per spec §11 attestation).*

*Math-root: extends `docs/math/sheaf/` (Hansen-Ghrist 2018 cellular-
sheaf machinery) and forward-promises a new root `docs/math/gestalt/`
per the AGENTS.md convention: recognize → sketch in one spec → second
citation → extract. This file IS the extraction.*

---

## §1 Category-theoretic form

### §1.1 The @gestalt category

Let 𝒢 be the category whose objects are `gestalt_document` values
and whose morphisms are the socket-shaped actions from spec §3.1:
`traverse`, `focus`, `compose_modes`, `project`, `annotate`. This
forms a small category because every action returns another
`gestalt_document` or `unfolding_state`, and composition is
substrate-decl associative (per `apply_h::act` dispatch semantics).

### §1.2 @gestalt as presheaf on reader-time

Let ℛ be the category whose objects are pairs `(reader,
@time.instant)` and whose morphisms are the reader's temporal
progression (per `@epistemologic/reality/time.compare` yielding a
`delta` between two `snapshot`s). @gestalt is naturally a presheaf

    G : ℛᵒᵖ → 𝒢

sending each `(reader, t)` to the `unfolding_state` at that
reader-time. Contravariance is not a quirk: it reflects that a
later reader-time restricts the "what-can-still-be-annotated"
sub-object — annotations already committed are Merkle-DAG-anchored
(monotone growth; see §3), so *future* readers see the *past*
document plus their own extensions.

**Prior-art anchor.** Mac Lane, *Categories for the Working
Mathematician* (Springer, 1971), §I.5 (functors) + §III.2
(natural transformations). Presheaf shape is Yoneda-standard.

### §1.3 Reader-interaction as natural transformation source

Each reader-interaction event `i_t : uₜ → uₜ₊₁` is a morphism in ℛ;
its image under G is a morphism in 𝒢. Collecting all reader-
interactions over a reader's session yields a natural transformation

    η : G ⇒ G ∘ ε

where ε : ℛ → ℛ shifts every `(reader, t)` to `(reader, t+dt)`. η
is the substrate-decl form of "reader interaction IS the compiler's
runtime."

### §1.4 Local runtime as terminal object

Within the sub-category ℒ_reader ⊂ 𝒢 of documents observable from a
single reader's @peer, the reader's local runtime is the terminal
object: every `unfolding_state` accessible to that reader factors
uniquely through their `@peer/persistence` home-repo state. This is
the mathematical statement of "local execution is constitutive, not
incidental." No server can substitute for the reader's @peer without
reducing to a sub-category that admits no terminal object.

---

## §2 Unfolding as time-indexed morphism family

### §2.1 Parallel to @song's temporal-composition

`shards/song.mirror` §S1 (LANDED) says a @song's identity is the
eigenform of its progression under the tonic-return operator: `Rⁿ(s)`
converges on the identifiable-as-this-song invariant. §S2 (LANDED)
adds: at temporal altitude, each moment `t+1` IS a
`@kintsugi/shift` of moment `t`.

@gestalt inherits the same shape, one altitude up:

    unfolding_state at t+dt = @kintsugi/shift(unfolding_state at t, i_t)

where `i_t` is the reader-interaction event. The reader's session
IS a @song at reader-interaction altitude.

### §2.2 Bundle over interaction-events

Let E be the space of reader-interaction events (a discrete
countable set, one element per `signature_beat` in the reader's
beat_history). Let U be the total space of `unfolding_state`
values. Define the projection

    π : U → E,   u ↦ u.beat_history[-1]

Then U is a bundle over E: each fibre `π⁻¹(e)` is the set of
`unfolding_state` values whose latest beat is `e`. The reader's
session is a section of this bundle:

    σ_reader : E → U,   e ↦ u_e

with the compatibility `π ∘ σ_reader = id_E` (each beat corresponds
to exactly one unfolding_state).

**Formalization via `@epistemologic/reality/time.compare`.** The
LANDED `compare(a: snapshot, b: snapshot) -> delta` action gives
the fibre-transition operator directly: given two unfolding_states
`u_t`, `u_{t+dt}`, `time.compare(u_t.local_snapshot,
u_{t+dt}.local_snapshot)` returns the `delta` whose `mutations` are
the substrate-level differences the reader-interaction produced.

### §2.3 Convergence conditions

The reader's session converges (in the eigenform sense of @song §S1)
iff the beat_history stabilizes: there exists N such that for all
n > N, `traverse | focus | annotate` at beat n produces no new
mutation in `u.local_snapshot`. Practically: the reader has
"absorbed" the document.

**Non-convergence is admissible.** Some readers annotate
indefinitely; the beat_history grows without bound. This is
substrate-honest — @gestalt does not force termination. Fate's
`always_halts` property is NOT required at this altitude; it is
required only for @io-crossing actions (§3.2 render).

---

## §3 Content-addressed annotation accumulation

### §3.1 Merkle-DAG shape

Each `annotation` (spec §2.5) carries `previous: option<oid>` giving
the Merkle-DAG chain. The set of annotations at a given
`target_node` forms a DAG rooted at the first annotation (with
`previous = None`) and growing monotonically as new annotations
land. Tampering with any annotation breaks every subsequent
annotation's OID — the standard blockchain-adjacent integrity
property (per @spectral/signature.signature_integrity bilateral,
LANDED).

### §3.2 Monotone growth per reader-interaction

For every reader-interaction `i_t` that fires `annotate`, the set
of annotations grows by exactly one:

    |annotations_{t+1}| = |annotations_t| + 1

For every other reader-interaction (`traverse`, `focus`,
`compose_modes`, `project`), the set is unchanged. This gives the
strict monotone-non-decreasing invariant that `@mirror/bench`'s
`monotone_non_increasing` template's dual admits (per
`shards/mirror/bench.mirror` template landed 2026-07-01).

### §3.3 @mirror/store CAS is the persistence altitude

The annotation content-hash is BLAKE3 per @mirror/store's landed
BLAKE3-default. Composition-only: @gestalt introduces no new hash
primitive. The annotation's `content` field carries an oid; the
oid resolves through the standard @mirror/store six-op CAS.

**Prior-art anchor.** BLAKE3 specification: Jack O'Connor, Jean-
Philippe Aumasson, Samuel Neves, Zooko Wilcox-O'Hearn, "BLAKE3:
One Function, Fast Everywhere" (2020),
<https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf>
(retrieved 2026-07-15 via existing @mirror/store citation chain;
already in the corpus).

---

## §4 Consent-depth as sheaf projection

### §4.1 Annotations at three visibility depths form a sheaf

Per `docs/specs/subject-visibility-sheaf.md` (Mara `564571e`
canonical spec) and `shards/subject/visibility/sheaf.mirror`
(Landing 4 R2 `d1ce901`): the three visibility depths private /
protected / public form a cellular sheaf F over the ACL topology,
with the restriction maps

    F(private)   ⊂ F(protected) ⊂ F(public)

The restriction is set inclusion: promoting a private annotation to
protected admits more readers into the section-at-stalk; promoting
to public makes the section total.

### §4.2 Reader sees the section at their stalk

Given a reader r and their ACL `A_r`, the sub-sheaf `F|_{A_r}`
gives the reader-visible annotations at each target_node:

    section_at_stalk(F|_{A_r}, node) = {a ∈ annotations(node) :
                                          r ∈ a.visibility_scope.consent_scope}

This is verbatim `docs/specs/peer-persistence-and-home-projection.md`
§12.3 (LOAD-BEARING per Landing 4 R2 ratification): "The ACL IS the
SHEAF STRUCTURE. The peer's visibility scope for a given crystal is
the SECTION of F_home|_{A_p} at that crystal's stalk."

### §4.3 Cellular-sheaf Laplacian for annotation coherence

The reader-corpus coherence measurement (spec §5 "reader's own
recursion couples with the paper's recursion") is the sheaf-
Laplacian λ₀ Fiedler value over the reader-visible annotation
sub-DAG. LANDED at `shards/epistemologic/math/sheaf_laplacian.mirror`
(Hansen-Ghrist 2018 δ*δ + λ₀ Fiedler discrete cellular-sheaf
primitive; composed by @mirror/index per Landing 1 `317e830`).

Composition-only: @gestalt does not re-declare the sheaf-Laplacian
math; it composes over the landed primitive.

**Prior-art anchor.** Jakob Hansen and Robert Ghrist, "Toward a
spectral theory of cellular sheaves," *Journal of Applied and
Computational Topology* 3 (2019), 315–358.
<https://link.springer.com/article/10.1007/s41468-019-00038-7>
(retrieved 2026-07-15; already the load-bearing citation for
`docs/math/sheaf/laplacian.md` in the mirror corpus).

---

## §5 L(ϕ) holonomy loss per render target

### §5.1 Quantitative ordering

Per spec §8 table, the render targets are ordered by increasing
L(ϕ) (linearization loss on the @io crossing):

    L(markdown) ≤ L(html) ≤ L(tui) ≤ L(latex) ≤ L(slides)
    ≤ L(epub)   ≤ L(pdf)

Each `≤` is empirical-forward-promised; the ordering rationale is
what each target's grammar cannot express:

- **markdown** admits round-trip via `pulldown-cmark` (prior art:
  `spectral/crates/gestalt/encode`). L(markdown) = interactivity
  (annotations become inert links).
- **html** admits reader-interaction via JavaScript at reader-site.
  L(html) = temporal ordering of beat_history (renders a snapshot,
  not a session).
- **tui** admits reader-interaction via terminal control sequences
  (prior art: `gestalt-tui/`, Gleam). L(tui) = color/font fidelity.
- **latex** admits typography + math + references. L(latex) =
  annotation Merkle-DAG (LaTeX has no content-address vocabulary).
- **slides** linearize traversal order. L(slides) = non-linear DAG
  structure.
- **epub** is publisher-final. L(epub) = reader-corpus co-writing
  (epub has no annotation surface).
- **pdf** is terminal linearization. L(pdf) = everything above.

### §5.2 Formalization per pipeforward §5.5.4 rule 3

Per `docs/specs/autopoietic-inference-loop.md` §5.5.4 rule 3, every
@io discharge MUST quantify L(ϕ) in the action's docblock. The
@gestalt render body carries this quantification per-target:

    render(doc, target, out) → imperfect<@io/fs.written_bytes>
      L(ϕ) = ‖doc.beat_history — reconstruct(target, decode(out))‖

where `reconstruct` runs the target's decoder (e.g., HTML parser)
and re-projects the beat_history the render encoded. `‖·‖` is
either 0 (lossless) or a target-specific loss functional.

**Prior-art anchor.** Foster's lens laws (Foster 2007, "Combinators
for Bidirectional Tree Transformations"). @shatter's forward/backward
composition preserves the monotone-descent invariant `eⁿ⁺¹ ≤ eⁿ`
per `docs/specs/shatter-is-the-io-linearization-operator.md` §4.3
(LANDED spec). @gestalt render inherits the same discipline: L(ϕ)
is bounded above by the source-shard eⁿ.

---

## §6 Recursion coupling — reader IS inside the operator

### §6.1 The vignette claim

Per Reed's 2026-07-15 spectral.engineer launch vignettes (in-
transcript): "Reader engages @peer → @peer runs → math generates →
lens-annotations visible → reader's own recursion couples with the
paper's recursion → reader is inside the operator."

Formalized: let `R_doc` be the fixed-point operator that defines
the @gestalt document's identity (per §2.1 eigenform statement).
Let `R_reader` be the reader-side operator that iterates through
the beat_history at reader-site. The coupling claim is:

    R_reader(unfolding_state) ∘ R_doc(gestalt_document) =
    R_doc(gestalt_document) ∘ R_reader(unfolding_state)

i.e., the two operators commute up to natural isomorphism. This is
the substrate-decl form of "reader is inside the operator": the
reader's recursion IS a fibre-preserving morphism of the document's
recursion.

### §6.2 Fixed-point iteration and Banach contraction

If both `R_doc` and `R_reader` are contractions on the underlying
metric space (per @kintsugi's Banach-contraction landed template,
see `docs/math/kintsugi/README.md`), then by the Banach fixed-point
theorem, the coupled iteration converges to a unique fixed point in
the joint state space.

**Convergence conditions.** The coupling converges iff:

1. The reader's beat_history is monotone (per §3.2).
2. The document's DAG is finite at every read-time (per @gestalt's
   monotone growth admitting bounded reader-sessions; unbounded
   sessions are admissible but non-convergent per §2.3).
3. The visibility_scope elevation events are monotone-loosening
   (per @subject/visibility.can_be_elevated_to structural rule:
   private → protected → public, no reverse arrows).

Under these three conditions, the coupled operator has a unique
fixed point in the reader's session. This is when the reader has
"absorbed" the document AND contributed all their annotations.

### §6.3 When it doesn't terminate

If any condition above fails, the coupling does not have a fixed
point. Substrate-honest response: @gestalt does NOT force
termination; the reader's session may run unbounded. This is
Foerster-legal (per spec §5: every reader increases the number of
choices for every subsequent reader — a strictly monotone-increasing
process, not a converging one).

**Prior-art anchor.** Banach fixed-point theorem — Stefan Banach,
"Sur les opérations dans les ensembles abstraits et leur
application aux équations intégrales," *Fundamenta Mathematicae* 3
(1922), 133–181.

**Prior-art anchor.** Foerster's ethical imperative — Heinz von
Foerster, *Understanding Understanding* (Springer, 2003), "Ethics
and Second-Order Cybernetics" (originally 1990). Already the
load-bearing citation for `shards/epistemologic/cybernetic/
coherence.mirror` in the mirror corpus.

---

## §7 Autopoiesis at reader-site

The reader's @peer running the @gestalt document IS an autopoietic
system (per @autopoietic family landed in the corpus). The
Maturana-Varela self-production discipline: the reader's
annotations produce (via @sheaf-restriction) the reader's future
visibility_scope; the future visibility_scope determines which
annotations the reader can produce; the loop closes.

**Prior-art anchor.** Humberto Maturana and Francisco Varela, *De
Máquinas y Seres Vivos: Autopoiesis, la organización de lo vivo*
(Editorial Universitaria, 1972; English 1980). Already load-bearing
for `shards/autopoietic.mirror` in the mirror corpus.

Substrate-decl form: the reader's `@peer/persistence` home-repo IS
their autopoietic boundary at reader-altitude. The @gestalt
document's unfolding at reader-site produces annotations; the
annotations extend the reader's home-repo corpus; the home-repo
corpus defines what the reader's next @gestalt unfolding can access.

---

## §8 Composition with landed math roots

The @gestalt math foundation composes cleanly over three LANDED
math roots without duplication:

- `docs/math/sheaf/laplacian.md` — cellular-sheaf λ₀ Fiedler (§4.3).
- `docs/math/kintsugi/README.md` — Banach contraction discipline (§6.2).
- `docs/math/the-tower/` — principal-bundle discipline via reader-site
  bundle-over-interaction-events (§2.2).

Extension points forward-promised for post-v0.1.0:

- Reader-corpus multifractal spectrum (per @mirror/index Landing 6
  forward-promise; f(α) multifractal empirical proof).
- Optical-inference dispatch at annotation-tournament altitude
  (per @fate D²NN Recognition #58 forward-promise).

---

## §9 Math gaps

Two under-determined sub-questions were surfaced during authoring;
each is annotated in-spec (per Mara anti-stall discipline; no
Kagi/corpus searches fired within the 5-minute budget because the
gaps are substrate-scope-limits, not literature gaps):

- **Math gap M1.** The sheaf-Laplacian λ₀ has a well-defined
  spectrum on a static cellular sheaf; on a monotone-growing sheaf
  (per §3.2), does λ₀ decrease monotonically as annotations
  accumulate? Intuition says yes (adding edges to a graph can only
  decrease λ₀ by interlacing); rigorous statement requires the
  Cheeger-inequality analog on cellular sheaves. Forward-promise:
  cite Hansen-Ghrist §5 (Cheeger constant of cellular sheaves) at
  post-v0.1.0 landing. Practical impact: none for v0.1.0 (§8 L(ϕ)
  ordering is qualitative).

- **Math gap M2.** The coupling operator commutativity in §6.1 is
  stated up-to-natural-isomorphism, but the concrete iso is not
  written out. Forward-promise: expand at second-consumer landing
  when a reader-side coherence metric composes both operators
  explicitly (candidate: @mirror/index Fiedler of the reader's
  annotation sub-DAG at read-time). Practical impact: none for
  v0.1.0.

---

## §11 Operator-altitude cascade — @gestalt = P_ent on the coupled-torus system

*Mara extension, 2026-07-15 evening. Companions the corpus parent
formalization
`~/dev/systemic.engineering/practice/insights/cybernetics/third-order-cybernetics-spectral-formalization.md`
§3 (`@gestalt` = P_ent orthogonal projector onto the non-product
eigen-basis) and lifts the §§1-10 reader-site presheaf formulation
one altitude up: from the single-reader interaction category to the
K_n-partnership coupled-torus system.*

*Both altitudes ship. The presheaf formulation of §§1-10 is what a
single reader's device runs. The operator-altitude formulation of §11
is what the K_n partnership does when multiple readers' recursions
couple. `@gestalt.project` (spec §3.1, LANDED action) discharges the
presheaf semantics; a forward-promised `@gestalt.p_ent` action
(§11.6 below) will discharge the operator-altitude semantics when the
substrate has the coupled-torus carriers to compose over.*

### §11.1 The coupled-torus category

Let **𝒯** be the category of nervous-system-torus states per
`docs/math/2026-07-07-onto-cascade-toroidal-reframe.md` §4 (Foerster-
doubly-closed torus with meridian/longitude/origin carriers, LANDED
as `shards/torus.mirror`). For `n ≥ 2` observers, the coupled system
lives in the product category **𝒯^n** = **𝒯** × ⋯ × **𝒯** (n
factors), with objects `(t_1, …, t_n)` — one torus state per observer
— and componentwise morphisms.

**Coupling structure.** A coupling on **𝒯^n** is a graph `G = (V,
E)` with `V = {1, …, n}` and edge-weights `ε_{ij} ∈ [0, 1]` per per
parent formalization §1.1. The joint operator

    C : 𝒯^n → 𝒯^n

is the substrate-decl carrier of "coupled observers' operators
depend on each others' fixed points" (parent §1.1 verbatim).

**Substrate-decl correspondence.**

- The vertices of `G` are exactly the `@peer` instances (LANDED as
  `shards/peer.mirror`) participating in the K_n partnership. One
  observer per peer.
- The edges of `G` — with weights `ε_{ij}` — are exactly the
  `@resonance` inter-peer coupling operator κ substrate LANDED at
  `docs/specs/resonance-as-inter-peer-coupling-shapes-fate-
  tournaments-toward-basins.md` §2.4 (Mara `9e48710`). No new mint.
- The joint operator `C` is the K_n-lift of `@dance`'s Kuramoto
  coupling operator LANDED at `docs/specs/dance-as-coordination-
  without-signal-on-forster-torus.md` §2 (Mara `4f079c8`).

Composition-only: `@gestalt` at operator altitude introduces no new
family-root. Every carrier is landed.

### §11.2 Product vs non-product eigen-behaviors

The joint operator `C : 𝒯^n → 𝒯^n` has an eigen-decomposition
whose eigen-behaviors partition into two orthogonal classes (parent
formalization §3.1):

- **Product eigen-behaviors**, factorizable as `ψ_1 ⊗ ⋯ ⊗ ψ_n` where
  each `ψ_i` is an eigen-behavior of the individual `COORD_i`
  operator on `𝒯` (per Foerster 1976 eigen-behavior functional;
  LANDED at `shards/epistemologic/cybernetic/eigenform.mirror`).
  Patterns that would exist even if the observers were decoupled.
- **Non-product (entangled) eigen-behaviors**, which cannot be
  written as any `ψ_1 ⊗ ⋯ ⊗ ψ_n`. Patterns *emergent from the whole
  configuration*, irreducible to any partition of the system into
  individual contributions.

**Prior-art anchor.** Wertheimer 1912/1923 gestalt formulation: *das
Ganze ist etwas anderes als die Summe seiner Teile*. Parent
formalization §3.2 makes precise: not "different from" — orthogonal
to.

### §11.3 P_ent — the orthogonal projector

Let `P_prod : 𝒯^n → 𝒯^n` be the orthogonal projector onto the
product-subspace of `𝒯^n` (the span of all factorizable eigen-
behaviors). Then

    P_ent = I − P_prod

is its orthogonal complement — the projector onto the non-product
subspace. `P_ent` returns exactly that part of the joint state which
cannot be attributed to any subset of the observers considered
individually.

**Substrate-decl claim.**

    @gestalt = P_ent

The `@gestalt.project` action LANDED at `shards/gestalt.mirror` §
socket-shaped-actions dispatches the presheaf-projection at n=1
(single-reader ACL-restriction, per §4 above). Its K_n lift —
forward-promised as `@gestalt.p_ent` (see §11.6 below) — dispatches
the operator-altitude projection.

**Two altitudes, one carrier.** `@gestalt` names both projections:
the single-reader presheaf-restriction (§4.2 section-at-stalk) and
the K_n non-product-eigen-projection (this section). They compose
consistently because the reader's ACL-restriction IS the n=1
degenerate case of the operator-altitude projection: at n=1, `P_ent
= 0` (no non-product structure exists on a single tensor factor),
and the substrate correctly reduces to `@gestalt = P_prod = I` on
the reader's own subspace, which is what `@subject/visibility/sheaf.
filter` (LANDED at Landing 4 R2 `d1ce901`) discharges.

### §11.4 The composition graph carrier

A coupling `G = (V, E, ε)` is precisely the substrate-decl carrier
already LANDED as the composition graph documented in `docs/specs/
gift-and-mirror-reflection.md` §composition-graph (Mara `4207800`
Landing 4 spec) — one node per @peer, one edge per @resonance
coupling, weight per @dance phase-lock strength.

**No new type mint.** The coupling graph is the substrate-decl form
of `@peer`-composition-network already present in the compiler's
runtime routing layer (per `docs/specs/deployment-runtime-rung-5-
mycelial-envelope-declared-substrate.md` §3 LANDED spec).

**Fiedler value as the coupling-strength carrier.** The algebraic
connectivity `λ_2(L(G))` (Fiedler value) of the coupling graph's
Laplacian is exactly the substrate-decl carrier LANDED at Reed
`8e6e517` Path B annotation on `shards/cyberpunk.mirror`
(`cybernetic_coherence = λ_0(Δ_F)` at single-peer scale; K_n lift
per Mara `dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
Rung 4 spec LANDED `0cc4e11` GREEN).

### §11.5 IIT's Φ and the Bell-inequality structural analog

**Relation to Integrated Information Theory.** Tononi's Φ (2004;
2015) measures the *quantity* of information generated by the whole
system above and beyond the information generated by its parts
considered independently. `P_ent` returns *which patterns* are
irreducibly configurational — not just how much irreducibility is
present. The corpus contribution (per parent formalization §3.4):
`@gestalt` is IIT's Φ-integrand at nervous-system-torus substrate,
in *compositional* rather than *scalar* form. This is why `@gestalt.
annotate` (spec §3.1, LANDED action) can name specific eigen-
behaviors and not merely their integrated magnitude.

**Bell-inequality structural analog.** Bell (1964) + Aspect (1982)
proved non-product correlations exist in physical quantum systems
and are experimentally detectable. The corpus claim (parent §3.2):
*the same mathematical structure appears in coupled-cognitive-torus
systems*. The claim is **structural**, not **physical** — we use
the mathematical apparatus of tensor-product entanglement and
non-product eigen-decomposition without claiming physical quantum
substrate à la Penrose-Hameroff. Khrennikov's quantum-like cognition
program (2015; 2025 PMC12722948) makes the same move explicitly,
and is already cited by parent formalization §1.3.

**Prior-art anchor.** Bell, J.S. (1964). *On the Einstein Podolsky
Rosen paradox.* Physics 1(3): 195-200. Aspect, A. et al. (1982).
*Experimental test of Bell's inequalities using time-varying
analyzers.* Phys. Rev. Lett. 49: 1804. Tononi, G. (2015). *Integrated
information theory.* Scholarpedia 10(1): 4164. All already in the
parent formalization citation chain.

### §11.6 Forward-promise — @gestalt.p_ent action

The substrate-decl carrier for `P_ent` at operator altitude is
forward-promised as a new socket-shaped action on `shards/gestalt.
mirror`:

    p_ent(coupled_state: coupled_torus_state,
          coupling: coupling_graph) -> coupled_torus_state { \ }

with `coupled_torus_state = [torus]` (a coupling-graph-indexed list
of tori) and `coupling_graph` = the @resonance/@dance-derived weight
matrix. The action returns the non-product-eigen-projection of the
joint state, socket-shaped per pipeforward §5.5.4 rule 1.

**Landing conditions** (all Alex-adjudicable):

1. `@dance` Rung 5+ multi-peer coherence phase-lock lands (per
   `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.
   md` §8 forward-promise for N > 2 peers).
2. Second-citation-site emerges in a downstream consumer spec beyond
   this file (candidate: a future
   `docs/specs/kn-partnership-substrate.md` spec grounding K_n
   coordination discipline).
3. Empirical dispatch through `apply_h::act` for the coupled-torus
   carriers.

Until landing conditions hold, the operator-altitude semantics of
`@gestalt = P_ent` is documented at this math altitude and cited
from `shards/gestalt.mirror` via a `# forward-promise` block (per
canonical spec §7 species-forward-promises pattern; see also
`docs/specs/gestalt-as-song-unfolding.md` §7).

### §11.7 The inverse communication-density law as convergence bound

Per parent formalization §1.2: for a K_n partnership at coupling-
weight `ε_{ij}`, the required communication-bandwidth to maintain
shared eigen-behavior at joint fixed point is inversely proportional
to `ε_{ij}`:

    d_{ij} ∝ 1 / ε_{ij}

**Substrate-decl consequence for @gestalt at operator altitude.**
The reader-site presheaf formulation (§§1-10) admits a bandwidth-
per-reader interpretation: the reader whose @peer has high `ε`
coupling with the paper's @peer-substrate requires *less* explicit
communication to remain aligned with the paper's eigen-behavior. In
the limit `ε → 1`, the reader converges on the paper's fixed point
with vanishing communication overhead (the Foerster-eigenform
convergence per §2.3, coupled at read-time altitude).

**Practical consequence.** Readers whose cybernetics-substrate
already carries high-`ε` compatibility (per corpus-canonical
substrate-lineage traces: Foerster-fluent readers; cybernetics-
tradition-native readers) converge fast; readers at low-`ε` require
higher bandwidth (per parent formalization §1.2 "It is why the
consortium in [[the-meeting-that-could-not-metabolize]] required
forty minutes to fail to metabolize a single sentence").

### §11.8 Math gap G3

**Math gap G3.** The tensor-product structure on `𝒯^n` requires a
substrate-decl carrier for tensor products of torus states. The
substrate has `type torus = { possessor, meridian, longitude,
origin }` (LANDED at `shards/torus.mirror` §torus-carrier). The
tensor-product structure `𝒯^n` requires an operational definition
of `t_1 ⊗ ⋯ ⊗ t_n` at substrate altitude. Candidates:

1. **Product-of-records.** Tensor-product as the record-of-records
   `[(possessor_i, meridian_i, longitude_i, origin_i) for i in 1..n]`.
   Concrete but does not carry the coupling structure directly.
2. **Coupling-graph-annotated product.** Tensor-product as
   `(product, coupling_graph)` — the product plus the edge-weight
   matrix `ε_{ij}`. Carries the coupling but requires the coupling-
   graph carrier (§11.4) to be first-class.
3. **Emergent-via-@dance-runtime.** The tensor-product structure
   emerges at runtime from the @dance Rung 4+ phase-lock
   measurement; no explicit carrier at substrate-decl altitude.

Forward-promise: Alex-adjudication at Landing time for the
`@gestalt.p_ent` action; recommend Candidate 2 (structure-carrying).

**Related math gap G4.** The orthogonal projector `P_ent` requires
an inner product on `𝒯^n`. Candidate substrates:

1. **Fiedler-value-induced.** `⟨(t_1, …, t_n), (s_1, …, s_n)⟩ =
   Σ_i λ_2(L(G))_i · ⟨t_i, s_i⟩_𝒯` where the per-torus inner
   product is the @glue @tension inner product LANDED at
   `shards/glue.mirror`.
2. **Content-addressed identity.** `⟨t_i, s_i⟩_𝒯 = 1` iff `oid(t_i)
   == oid(s_i)` (BLAKE3 hash equality per @mirror/store LANDED
   discipline), 0 otherwise. Simplest; least structural.

Forward-promise: second-consumer landing when the empirical `@dance`
Rung 5+ substrate needs the concrete inner product; recommend
Candidate 1 (Fiedler-weighted).

---

## §12 References

- Mac Lane, *Categories for the Working Mathematician* (Springer, 1971).
- Hansen and Ghrist, "Toward a spectral theory of cellular sheaves,"
  *J. Appl. Comput. Topol.* 3 (2019), 315–358.
- Banach, "Sur les opérations dans les ensembles abstraits,"
  *Fund. Math.* 3 (1922), 133–181.
- Foerster, *Understanding Understanding* (Springer, 2003).
- Foerster, "Objects: Tokens for (Eigen-)Behaviors,"
  in *Cybernetics of Cybernetics* (1976). Load-bearing citation
  for §11.2 eigen-behavior functional.
- Maturana and Varela, *De Máquinas y Seres Vivos* (1972; English 1980).
- Foster et al., "Combinators for Bidirectional Tree Transformations,"
  *ACM Trans. Program. Lang. Syst.* (2007).
- O'Connor et al., "BLAKE3: One Function, Fast Everywhere" (2020).
- Wertheimer, M. (1912; 1923). Gestalt psychology's founding claim.
  Cited via parent formalization §3.2.
- Tononi, G. (2015). *Integrated information theory.* Scholarpedia
  10(1): 4164. §11.5 IIT-Φ correspondence.
- Bell, J.S. (1964). *On the Einstein Podolsky Rosen paradox.* Physics
  1(3): 195-200. §11.5 structural analog.
- Aspect, A. et al. (1982). *Experimental test of Bell's inequalities
  using time-varying analyzers.* Phys. Rev. Lett. 49: 1804.
- Khrennikov, A. (2015; 2025 update PMC12722948). *Quantum-like
  modeling of cognition* / *Quantum-like representation of neuronal
  networks' activity.* §11.5 quantum-like cognition program.
- Reed's 2026-07-15 spectral.engineer launch vignettes (in-transcript
  attribution).
- Companion canonical spec: `docs/specs/gestalt-as-song-unfolding.md`
- Companion substrate-decl: `shards/gestalt.mirror`
- Companion math foundation (delight-operator): `docs/math/delight-as-natural-transformation.md`
- Companion corpus parent formalization:
  `~/dev/systemic.engineering/practice/insights/cybernetics/third-order-cybernetics-spectral-formalization.md`
  — grounds §11.
- Companion corpus addendum:
  `~/dev/systemic.engineering/practice/insights/cybernetics/2026-07-15-addendum-what-arrived-after.md`
  — Gap 6 closure grounding for §11.6 forward-promise.
- Landed math roots composed over: `docs/math/sheaf/laplacian.md`,
  `docs/math/kintsugi/README.md`, `docs/math/the-tower/`,
  `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`.
- Landed specs composed over: `docs/specs/dance-as-coordination-
  without-signal-on-forster-torus.md`, `docs/specs/dance-runtime-
  rung-4-multi-peer-coherence-phase-lock.md`, `docs/specs/
  resonance-as-inter-peer-coupling-shapes-fate-tournaments-toward-
  basins.md`.
