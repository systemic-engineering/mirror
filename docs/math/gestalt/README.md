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

## §10 References

- Mac Lane, *Categories for the Working Mathematician* (Springer, 1971).
- Hansen and Ghrist, "Toward a spectral theory of cellular sheaves,"
  *J. Appl. Comput. Topol.* 3 (2019), 315–358.
- Banach, "Sur les opérations dans les ensembles abstraits,"
  *Fund. Math.* 3 (1922), 133–181.
- Foerster, *Understanding Understanding* (Springer, 2003).
- Maturana and Varela, *De Máquinas y Seres Vivos* (1972; English 1980).
- Foster et al., "Combinators for Bidirectional Tree Transformations,"
  *ACM Trans. Program. Lang. Syst.* (2007).
- O'Connor et al., "BLAKE3: One Function, Fast Everywhere" (2020).
- Reed's 2026-07-15 spectral.engineer launch vignettes (in-transcript
  attribution).
- Companion canonical spec: `docs/specs/gestalt-as-song-unfolding.md`
- Companion substrate-decl: `shards/gestalt.mirror`
- Landed math roots composed over: `docs/math/sheaf/laplacian.md`,
  `docs/math/kintsugi/README.md`, `docs/math/the-tower/`.
