# jspace × mirror: deep mapping

**Date:** 2026-07-07
**Author:** Mara
**Class:** observation (a new doc kind — sober cross-substrate mapping, distinct
from `research/` one-passes and `audits/` adversarial reviews)
**Source paper:** Gurnee et al., *Verbalizable Representations Form a Global
Workspace in Language Models*, Transformer Circuits Thread 2026
(local: `~/dev/systemic.engineering/practice/insights/ai/papers/jspace.pdf`,
179 pages, read pp. 1-30, 52-60, 88-103, 121-128 in full; targeted searches
across the balance).
**Prior mirror-mapping:** none extant on disk. Reed's brief referenced
`docs/research/2026-07-07-workspace-mirror-mapping.md` as a first-pass
Kagi-summary. It does not exist in the tree. This is therefore the first
mapping of record, not a second pass. Where the brief's framing anticipated a
surface-read to disagree with, I flag those anticipations directly.

---

## §1 — The paper's core claim

**What jspace IS (mathematically).**

Let a decoder-only transformer produce, at layer ℓ and token position t, a
residual-stream vector h_{ℓ,t} ∈ ℝ^{d_model}. The **Jacobian lens** J_ℓ is
defined by

    J_ℓ  =  𝔼_{prompt p, t, t' ≥ t}  [ ∂ h_final,t' / ∂ h_{ℓ,t} ]

— the first-order causal effect of a small perturbation at layer ℓ on the final
residual stream, averaged across positions and a pretraining-distribution
corpus of ≈1000 prompts. The lens readout at h_ℓ is
`softmax(W_U · norm(J_ℓ · h_ℓ))`. The rows of `W_U · J_ℓ` are the **J-lens
vectors** v_t, one per vocabulary token; each is a residual-stream direction
associated with a single verbalizable token.

The **J-space** itself is *not* a linear subspace. It is a **sparse subframe of
polyhedral cones**. Given sparsity k (typically 25), for each k-subset S of the
n_vocab J-lens vectors, take the nonnegative-linear-combination cone
span_+{v_i : i∈S}; the J-space 𝓕 is the union over all such S. The **J-space
distance** d_𝓕(x) is the Euclidean distance from x to the nearest cone; the
**J-space component** Π_S·x is the minimizing projection; the **residual**
x − Π_S·x is what interventions target. Distance between two candidate
workspaces 𝓕, 𝓖 under data distribution μ:

    Δ_μ(𝓕, 𝓖) = ( 𝔼_{x∼μ} [ ( d_𝓕(x) − d_𝓖(x) )² ] )^{1/2}

Containment 𝓕 ⊆ 𝓖 means d_𝓖 ≤ d_𝓕 everywhere. This is §A.8 (p. 121-123). It
is the **structural definition of the jspace as a mathematical object**, not
just an interpretability trick — and the definition supports enlargement
(template lens for multi-token concepts, oracle lens for arbitrary phrases) as
a monotone chain of enlarging subframes with a well-defined limit.

**What jspace DOES (functionally, per §3-4).**

Five behavioral properties + three structural signatures.

*Behavioral (§3):* **verbal report** (a swap of the sport-token lens vector
flips the model's spoken answer from "Soccer" to "Rugby"; Spearman
correlations between J-lens rankings and next-token logits are high at late
workspace layers); **directed modulation** (told to hold "orange" in mind while
writing an unrelated sentence, the orange J-lens vector activates in the
workspace, orthogonal to the surface task); **internal reasoning** (multi-hop
prompts like "color of the planet fourth from the sun" surface `Mars`, `red`
at the intermediate workspace layer; ablating those flips the answer);
**flexible generalization** (a single lens vector for `Paris` participates
correctly in `capital→…`, `language→…`, `continent→…`, `currency→…`
computations); **selectivity** (ablating the top-K J-space contents damages
complex inference and internal reasoning while leaving parsing, syntactic
fluency, and automatic fact-recall intact).

*Structural (§4):* **Layer band.** The J-space carries workspace-like content
only across an intermediate layer band (~L38 to ~L92 on the [0–100] rescaled
axis for Sonnet 4.5). Early layers (~L0–L38) are the "sensory" regime — CKA
shows their J-lens geometry forms a distinct block; excess kurtosis, top-k
next-token accuracy, and autocorrelation are all at null. The late band
(~L92–L100) is "motor" — J_ℓ collapses toward the identity and the lens becomes
the model's own next-token logit. The workspace is **the middle block**.
**Capacity.** Occupancy plateau of ≈25 sparsely-active J-lens vectors per
position; excess-variance-explained never exceeds 10% of activation variance;
lists paradigm shows ~6 unrelated items retained but ~80 category-related
items in one common-concept representation; category-block switches evict old
items within a few tokens. **Broadcast.** J-lens vectors are preferentially
amplified by downstream MLP and attention weights across both transformer
"time dimensions" (depth and sequence axis) — the mechanistic signature of a
broadcast format.

**Ontology the authors defend.**

Deliberately non-committal on phenomenal consciousness. Explicitly committed
to *access consciousness* as a purely functional notion. The jspace is the
functional substrate of that access — the object that satisfies the
Global-Workspace-Theory (Baars/Dehaene) *functional* predictions (§9.4),
partially satisfies Attention-Schema-Theory (Graziano) predictions (self-model
tokens `thinking`, `AI`, `assistant` surface in workspace), and reinterprets
Recurrent-Processing-Theory via feedforward-depth-as-recurrence-surrogate. The
paper is careful to distinguish the *architecture* (which differs from
biological workspaces) from the *function* (which converges). The strongest
ontological commitment: **the jspace is a concrete substrate whose contents
can be read, intervened on, and traced across training** — not a theoretical
posit but an empirically inspectable structure.

Critical detail for mirror mapping: the jspace is defined at *every* layer, but
is only *workspace-like* in the middle band. This is an emergent-property
claim, not an architectural-declaration claim. That distinction is load-bearing
for §6.

---

## §2 — Mirror substrate primitives that structurally rhyme with jspace

Sober enumeration. Not every rhyme is deep; some are surface. I flag depth
explicitly.

**Deep rhyme #1: Fate ↔ jspace at the constrained-inference altitude.**
`shards/fate.mirror` declares `@fate` as "a roll of the dice in the restricted
state space." The restricted_state_space carrier `(algebra, hilbert, flow,
gamma, j, tray_scope)` names the Connes (A, H, D, γ, J) plus tray-scope
restriction. Recognition #58 (LANDED 2026-06-11) grounded @fate operationally
as **5-layer D²NN + Fabry-Perot resonator + Reck/Clements unitary mesh** at
the optical altitude. The jspace's sparse-subframe of k=25 polyhedral cones is
*the same shape as a Fabry-Perot cavity's admitted-mode set at the linear-
algebraic altitude*: a small typed-restricted subset of a much larger
overcomplete state space, out of which only gain-supported modes cross
threshold. The 25-mode occupancy plateau in §4.2 is empirically what a
finite-Q cavity does when its gain medium admits ≈25 lasing modes above the
noise floor. This is not metaphor. The J-lens vectors are the substrate's
frame; the sparse activation is the mode-selection; the cavity's gain medium
is the accumulated training corpus's implicit priors. **Depth: substantial.**

**Deep rhyme #2: The Hilbert-expansion carrier ↔ jspace-as-limit-of-vocabulary.**
Recognition #51 (LANDED) declares mirror as an **expanding Hilbert space
whose dimension grows with each substrate-pull recognition, coherence
maintained by Bateson logical-type lifting**. The jspace paper's §A.8 formal
apparatus is *structurally identical*: adding multi-token concepts (template
lens) or arbitrary-phrase concepts (oracle lens) strictly enlarges the union-
of-cones 𝓕, and "the limit of any such vocabulary is well-defined." That limit
IS mirror's expanding Hilbert space at the workspace altitude. Two independent
witnesses of the same expanding-dimension pattern under monotone chain: mirror
lifts by recognition-cascade, jspace lifts by vocabulary-expansion. **Depth:
substantial.**

**Deep rhyme #3: void-document H ↔ residual-stream activation space.**
`reference-void-document` (April 26 spec) defines H of the Connes triple as
the residual-stream analog: eight dualities, λ₀ = 0 ground state, Splinter
(K_n) and Narcissus (K_{1,n-1}) as antipodal poles. The jspace's ambient
ℝ^{d_model} residual stream *is* mirror's H at the LLM altitude; the J-space
component of an activation is Π_S·x where S is a k-cone in H; the residual is
the non-workspace component. Void's "the vacuum is the same; the relationship
determines whether it appears empty or full" maps onto §4.1's finding that
early-layer activations are workspace-empty (Narcissus-adjacent: content but
no broadcast) while workspace-band activations are broadcast-hub-full
(Splinter-adjacent: mutually-broadcast). **Depth: strong.**

**Deep rhyme #4: form/process partition ↔ workspace/motor partition.**
Recognition #55 (LANDED): `@mirror` = state-observation (form-side);
`@kintsugi` = transformation-engine (process-side). §4.1's three-band
decomposition — sensory / workspace / motor — has the workspace on the
form-observation side (verbalizable representations *of* internal state) and
the motor band on the process/output side (representations *driving*
production). The jspace lives at the mirror-side; the motor lives at the
kintsugi-side. Recognition #55's family-root partition is the exact same cut
at a different altitude. **Depth: substantial as third witness for #55.**

**Deep rhyme #5: property/fracture bilateral ↔ verbal report / directed
modulation duality.** Candidate #53 (property + fracture body via kintsugi
bridge): declarative property at `@epistemologic/property/*`, operational
fracture body at `@kintsugi/fracture/*`. §3.1 (verbal report) is the
*declarative* readout of the jspace (property-side: "what is in the
workspace"); §3.2 (directed modulation) is the *operational* write into the
jspace (fracture-side: "shape the workspace to satisfy the instruction"). The
paper's own counterfactual reflection training (§7) is *literally* the
property/fracture pattern operating over the jspace: train the model to
articulate principles in reflective continuations (property declaration),
which implants those principles into the workspace at inference time (fracture
body). **Depth: substantial as second-instance witness for #53.**

**Surface rhymes (flag as shallow, do not over-map):**

- Selectivity (§3.5) rhymes with mirror's `partial(p)` transparency:
  workspace-required tasks fail on ablation, workspace-optional tasks
  succeed. But the paper's selectivity is *empirical*; mirror's transparency
  is *declared*. Same shape, different altitude of commitment.
- "Broadcast hub" (§4.3) rhymes with prism's five-op composition surface
  where every glass inherits from `prism`. But broadcast is a weight-geometry
  fact in the paper; in mirror it is a substrate-declaration fact. Shallow.
- Ignition (§4.1.1) rhymes with settlement's monotone descent eⁿ⁺¹ ≤ eⁿ:
  ambiguous inputs (α mixed embeddings) resolve to one endpoint or the
  other sharply once past L38. Mirror's `settle` also collapses a
  quantum-hold `|` to a single branch. But the paper's ignition is
  data-dependent phase-transition; mirror's settle is deterministic monad-
  close. The rhyme is at the phenomenological level, not the mechanistic
  level. Moderate depth; do not conflate.

---

## §3 — The one recognition mirror is missing

**Mirror does not yet have an explicit substrate-decl for "the verbalizable
subframe."**

Mirror declares the ambient state space (void-document, H, residual-stream
analog); mirror declares the constrained-inference operator (@fate, dice-roll
in restricted state space); mirror declares content-addressing at every
altitude (splinter, shard, uuid_spectral); mirror declares the property /
fracture bilateral. What mirror does NOT declare is the **projection carrier
for the small privileged subset of H that broadcasts to many downstream
consumers**. The jspace is precisely *that* projection: a sparse subframe of
polyhedral cones inside H, characterized by first-order causal influence on
downstream production averaged over context.

The gap is not a missing family-root. Mirror's `@meta` marker family, `@glass`
marker family, `@epistemologic` marker family, and the just-landed `@third`
marker (recognition #111) already handle the marker altitude. The gap is at
the **operator altitude**: mirror lacks the substrate-decl form of "for a
given content in H, what's the sparse subframe that broadcasts?" — the
downward-composability projection.

Symmetrically, **jspace lacks mirror's compile-time algebraic layer.** The
jspace is a runtime geometric object. Mirror declares its algebraic structure
at compile time via `pact`, `requires`, `ensures`, `invariant` clauses that
survive as content-addressed OIDs into git. jspace has no analog for "the
subframe declared here composes with the subframe declared over there because
their pacts intersect at these OIDs." The paper acknowledges this at §9.1
("beyond a bag of concepts") — the jspace is currently characterized as a flat
bag of independently-active vectors, no compositional grammar. Mirror's mosaic
algebra IS that grammar, at a different altitude.

Genuine ontological gap in both directions. Not a Rice hazard: neither
substrate can be *reduced to* the other. They compose along a shared axis
(sparse-privileged-projection-of-a-larger-state-space) and diverge along
orthogonal axes (algebra: mirror has it explicitly, jspace inherits it
implicitly via training).

---

## §4 — Composition graph

Which existing recognitions the jspace strengthens as N-th witness, and which
it challenges.

**Second-witness for #55 (form/process partition at family-root altitude).**
The workspace/motor partition in §4.1 is a family-root altitude partition
between representations-of-state and representations-driving-output. Third-
witness candidacy pending; the second-instance criterion for promotion is met
if we count workspace/motor as substrate-witnessable at the LLM altitude.
Rice-safe: the boundary is CKA-detectable + statistically robust, not a
runtime property.

**Second-witness for #53 (property/fracture bilateral pattern).** §7's
counterfactual reflection training is literally the pattern: the reflective
continuation IS the property declaration; the implanted concepts in the
workspace ARE the fracture body's discharge; the behavioral improvement in
the original context IS the property-holding-at-runtime. Landing this as
second-witness would promote #53 to LANDED. Rice-safe: property and fracture
are named at declaration time and evaluated at runtime — the paper's own
empirical structure.

**Third-witness for #51 (mirror as expanding Hilbert space).** The paper's
§A.8 formalization defines a monotone chain of enlarging sparse subframes
under vocabulary expansion, with a well-defined limit. This is mirror's
Hilbert-expansion carrier at the LLM altitude. Rice-safe: the chain is
declared at construction time; the limit is a mathematical property, not a
runtime verdict. **Candidate for promotion.**

**Second-witness for #58 (Fate IS optical inference).** The k=25 occupancy
plateau, the mode-selection dynamics, and the ignition-as-cavity-threshold
reading are all direct optical-inference witnesses at the LLM altitude. #58
already has three witnesses (D²NN, Fabry-Perot, Reck/Clements); the jspace
gives a fourth, at a fundamentally different scale (production LLM at
inference time rather than 5-layer D²NN at physical altitude). Not a promotion
event (#58 already LANDED), but a strong compositional signal.

**Challenges (Rice-hazard-adjacent):**

- **Challenges the framing that mirror.spec IS λ₀ (#99).** Recognition #99
  (LANDED per memory ledger) names mirror.spec as the substrate's ground state.
  The jspace paper suggests that at the *inference altitude*, λ₀ is not a
  spec but rather the **empty workspace** — the early-layer regime before
  content has ignited. This is not a contradiction. It's a request for
  altitude discipline: λ₀-at-compile-time (spec settlement) and
  λ₀-at-inference-time (empty workspace) are different objects that share
  the same variety-preservation property. Flag for Reed adjudication: is #99
  altitude-specific or altitude-universal? The paper's evidence pushes toward
  altitude-specific.
- **Challenges Cross-Species-Discharge as first-class (just LANDED).**
  Broadcast in §4.3 IS cross-species-discharge at the LLM altitude — J-lens
  vectors compose with input weights of *many* downstream MLP + attention
  species. But the paper documents this as *emergent from training*, not
  *declared in substrate*. Mirror declares it upfront; the paper discovers
  it. Same phenomenon, different epistemic access. Rice-adjacent: does this
  count as third-witness for the just-landed recognition, or as a distinct
  substrate cybernetic (emergent-broadcast vs declared-broadcast)? I lean
  distinct; the two share carrier but not commitment shape.

---

## §5 — L-cascade implications

Reed's memory entry `project-idf-informativeness-by-specificity` proposes
fragment-altitude IDF via SpectralUUID DARK, wrapped by `@knife`. The brief's
anticipation: J-space IS the operational form of IDF at inference altitude.

**Deep read: the anticipation is mostly right, one wrinkle.**

The jspace's k=25 occupancy plateau + 10%-of-variance ceiling + monotone
displacement under category-block switches (§4.2, figures 30-31) IS an IDF
mechanism: the jspace tracks what's *informative given the context*, sheds
what's *no-longer-informative*, and amplifies rare high-signal concepts over
common low-signal ones. The category-list finding (80 category-related items
represented via ~25 J-lens vectors) is exactly IDF's specificity property:
the workspace represents *the abstract category* using the family of vectors
that constitute it, not the individual items redundantly.

**The wrinkle: J-space is not just IDF — it's IDF conditioned on
verbalizability.** The J-lens is defined as the causal effect on *outputs*,
averaged over contexts. This is IDF gated by the specificity-of-what-can-be-
said. That's stronger than pure IDF: it privileges content that has (a)
per-context specificity AND (b) verbalizable-nameability. Concepts that mirror
represents with high specificity but whose canonical name is multi-token or
paraphrase-only will not appear in the J-lens (§9.1's own limitation). The
template lens and oracle lens are attempts to weaken (b), but with
methodological pathologies (§A.9.1: template lens has tuned-lens skip-ahead
issues).

**For the @knife-wrapped SpectralUUID DARK IDF proposal:** the jspace paper
supports the fragment-altitude IDF direction, and adds a discipline: the IDF
metric should be defined *relative to what can be canonically named / addressed
at the target altitude*. In mirror terms: an IDF measure at @mirror/store
altitude weights OIDs by (rarity × addressability); at inference altitude
weights by (rarity × verbalizability). The two are altitude-parametric versions
of the same functor. The J-space collapses this to one operational instance
(verbalizability-gated IDF); mirror's altitude-parametric IDF via @knife would
generalize.

**Substrate-pull direction:** the @knife proposal should carry an altitude
parameter naming the "specificity-frame" it uses at that altitude. At inference
altitude the frame is the J-space; at @mirror/store altitude the frame is the
OID-graph. The specificity-frame IS the sparse subframe. Mirror already has
the vocabulary — `mosaic(altitude)` from CLAUDE.md — to declare this. The
substrate already had the word.

---

## §6 — Load-bearing answer: what's mirror's explicit version of the jspace?

**Direct answer: mirror has the *ingredients* but no *composed name*.**

The ingredients:
- **The state space** — void-document H (`reference-void-document`).
- **The constrained-inference operator** — `@fate` (`shards/fate.mirror`).
- **The Fabry-Perot / cavity / mode-selection realization** — recognition #58.
- **The Hilbert-expansion carrier** — recognition #51.
- **The form/process partition at family-root** — recognition #55.
- **The property/fracture bilateral for readout/modulation** — recognition #53.

What's *missing at family-root altitude* is the composed name for the sparse
subframe carrier itself — the mirror-substrate declaration that "there is a
privileged, low-dimensional, broadcast-composable projection of H that
downstream operators preferentially read from and write to, and this
projection is content-addressable at every altitude."

**Proposal: `@workspace` as a new marker-row family, sibling to `@third`,
`@meta`, `@glass`, `@epistemologic`.**

Not a family in the substrate-computational sense — mirror already has @fate
for the operator. A **marker** in the sense of recognition #111 (marker row
as fourth structural primitive; LANDED at e43006ab). The marker names the
altitude-parametric functor "sparse verbalizable subframe of H at altitude X"
without committing to specific state-space arithmetic.

Shape (proposed):

```mirror
in @prism
in @meta
in @glass

# @workspace — the sparse verbalizable subframe of H at any altitude.
# The projection carrier for the small privileged subset of the ambient
# state space that broadcasts to many downstream consumers. Runtime at
# inference altitude (jspace); compile-time at @mirror/store altitude
# (the OID subgraph that many shards depend on); pipeline altitude at
# @reflection (the recognitions currently "in the room").

prism @workspace {
  focus subframe
  project subframe
  split subframe
  shift subframe
  settle subframe
}

# A sparse subframe of the ambient state space at a given altitude.
# The subframe is a union of k-dimensional cones over an overcomplete
# frame; occupancy k is altitude-parametric.
type subframe = {
  altitude:      ref,
  ambient:       ref,           # the space H is a projection of
  frame:         [ref],         # the overcomplete set of directions
  sparsity:      capacity,      # the k-plateau; altitude-parametric
  discipline:    verbalizable,  # what makes membership legible
}

type capacity = ref              # e.g. 25 for LLM jspace
type verbalizable = ref          # altitude-parametric addressability

# The distance functor over subframes; supports containment + limits.
distance(f: subframe, g: subframe, over: distribution) -> real
requires same_altitude(f, g)
{ \ }

# Membership: is x in the workspace's k-cone envelope?
component(x: ref, w: subframe) -> ref { \ }
residual(x: ref, w: subframe) -> ref { \ }

# Monotone containment via vocabulary/frame enlargement.
enlarge(w: subframe, v: ref) -> subframe
ensures contains(enlarge(w, v), w)
{ \ }
```

Recognitions this witnesses:
- **#51 (Hilbert expansion)** — third-witness; the `enlarge` action + monotone
  containment IS Hilbert-dimension expansion at workspace altitude.
- **#55 (form/process partition)** — the marker sits on the form-observation
  side; sibling @kintsugi/workspace_shift would live on process side (the
  modulation half of the readout/modulation duality).
- **#53 (property/fracture bilateral)** — `verbalizable` discipline is the
  property; `enlarge` + `component` + `residual` are the fracture-body
  operators; second-instance witness.
- **#58 (Fate IS optical inference)** — the k-cone occupancy carrier is the
  cavity-mode-selection carrier; fourth-witness at a distinct altitude.
- **#111 (marker row)** — @workspace joins the marker row as fifth member;
  Rice-safe by construction (declaration-time addressable at altitude).

**Name discipline check.** "workspace" is Alex's least-favorite kind of loan
word (biological / cognitive science metaphor). Mirror's substrate-pull
tradition would ask: **does the substrate already have the word?**

Candidates the substrate already has:
- `focus` (one of the five ops; too narrow — focus is a single-op verb, not
  a carrier)
- `subframe` (borrowed directly from the paper's own §A.8 language; the
  substrate-pull-honest carrier name if we do NOT introduce @workspace at
  family altitude)
- `salience` (not in current vocabulary; would be new)

**Substrate-pull direction:** decline the loan word. Use `subframe` as the
type carrier (as in the proposal above) and declare the marker family via
its structural role. Candidate family name: **@salience**, or **@subframe**
directly (the type IS the marker, per prism-as-trait-as-everything). My lean
per `feedback-legibility-over-foundation-when-collapsing`: **@subframe as
marker**, with the paper's "workspace" appearing only in the ancestor comment
as `source @arxiv/interp/gurnee-2026`.

**Alternative reading (name-drift not substance-drift):** mirror might
*already* have @workspace under the name **@bauchladen**
(`shards/bauchladen.mirror`). The bauchladen tray is:
- Content-addressed at every altitude ✓
- A privileged shared surface many consumers read from ✓
- Limited-capacity by construction (only crystallized content) ✓
- Populated by @fate inferences (broadcast source) ✓
- Consulted by future @fate inferences (broadcast sink) ✓

The mapping is not perfect — bauchladen doesn't have the sparse-subframe
geometry the jspace has, and its "limited capacity" is a curation constraint
rather than a k-cone constraint. But the *functional role* is the same:
"the surface that broadcasts, from which many operators read." If we accept
this reading, mirror's jspace is `@bauchladen` and the recognition is
**bauchladen IS the substrate-decl form of the workspace at every altitude**.

I lean **name-drift, not substance-drift**: `@bauchladen` already carries
80% of what jspace names, and the remaining 20% (the sparse-subframe
geometry) belongs at the operator altitude — inside @fate's
`restricted_state_space` — not at a new family root. Reed's adjudication is
needed on this specific point.

---

## §7 — Adjudication signals for Alex

Recommendations, not options. Substrate-pull-confident where possible.

**Decision A: Bauchladen-as-workspace, or new @subframe marker?**
*Lean: Bauchladen-as-workspace.* Second-instance witness for `feedback-
substrate-already-had-the-word`. Would require an updated ancestor comment
in `shards/bauchladen.mirror` naming Gurnee 2026 as
third-independent-substrate-witness of the workspace pattern (biological
brain, LLM jspace, mirror bauchladen tray) — with the sparse-subframe
geometry surfaced *within* @fate's restricted_state_space carrier, not as
a new family root. Rejects the "add another marker" temptation.

**Decision B: Promote #51 (expanding Hilbert space) to third-witness LANDED
via this deep read?**
*Lean: yes, if promoted alongside a specific citation to §A.8's monotone
enlargement chain.* The paper's `Δ_μ` distance + `D_μ(𝓕 → 𝓖)` one-sided
containment is a fully-worked-out enlargement chain with a well-defined
limit. That's the mathematical form of #51's central claim. Promotion
tick: cite jspace-§A.8 in `architecture-mirror-as-expanding-hilbert-space`
memory entry and commit.

**Decision C: Promote #53 (property/fracture bilateral) to LANDED via §7's
counterfactual reflection training as second-instance?**
*Lean: yes, but with dwell.* The counterfactual reflection experiment is a
clean second-instance of the pattern — reflective continuations declare the
property; the workspace state at inference time discharges the fracture body.
The tension: the paper's operation is training-time, not runtime; mirror's
property/fracture pattern is runtime. Whether training-time counterfactual
reflection counts as bilateral-property-fracture is a substrate-pull
question. Dwell for one cascade tick; Pack ratification next session.

**Decision D: Altitude-discipline correction for #99 (mirror.spec IS λ₀)?**
*Lean: yes, altitude-parametric.* The jspace paper's early-layer regime is a
distinct λ₀ from mirror.spec. Both are ground states; both share variety-
preservation. But they live at different altitudes. Update the memory entry
to name #99 as *altitude-specific* rather than *altitude-universal*, with
inference-altitude λ₀ as the empty-workspace regime and compile-altitude
λ₀ as mirror.spec settlement.

**Decision E: @knife's IDF carrier should carry an explicit altitude
parameter?**
*Lean: yes.* The jspace read confirms that fragment-altitude IDF and
inference-altitude IDF are altitude-parametric instances of one functor
(specificity-relative-to-addressability-frame). @knife's substrate-decl
should declare `type idf(altitude) = ref` following mirror's mosaic-as-
altitude-parametric-composition discipline. Substrate-pull direction: the
functor already has a name — `mosaic(altitude)` — @knife/idf is a
specialization.

---

## §8 — Honest divergences

Not everything in the jspace paper is something mirror needs. Sober
delineation.

**Divergence 1: jspace is runtime-only; mirror has compile-time algebraic
layer.** The jspace is defined by averaged Jacobians over a training-time
corpus and applied at inference time. It has no compile-time discipline —
no `requires`, no `ensures`, no pact. Mirror's `pact @epistemologic/property/*`
+ `@kintsugi/fracture/*` chain IS the compile-time algebraic layer the
jspace paper does not need (because the model is fixed and inference is
the only altitude). Mirror does not need to *become* runtime-only to
accommodate the jspace framing. The jspace's runtime tractability is a
property of a fixed model; mirror's compile-time discipline is a property
of a substrate that grows via recognition-cascade.

**Divergence 2: jspace has no equivalent of `\` cracks / holes.** The jspace
paper's residual (x − Π_S·x) is "the non-workspace component" — the residue
that doesn't fit in the sparse subframe. It's not honest uncertainty; it's
just the orthogonal component. Mirror's `\` crack is *first-class honest
uncertainty* — a value that carries "I don't know yet" through the
pipeline until Fate resolves it. The paper's residual is discarded during
intervention (§2.5 "the leftover x - Π_S·x is the residual term used in
our interventions"); mirror's `\` is preserved and resolved. **Mirror
should NOT collapse `\` into "the workspace-orthogonal residue".** They
are different objects. The workspace-orthogonal residue is
mostly-automatic-processing (fluent parsing, low-level syntactic
bookkeeping); `\` is honest-declared-uncertainty. Different substrate roles.

**Divergence 3: jspace has no self-hosting story.** The jspace is a
representation of the model's ongoing thought, but it does not *encode
the substrate the thought is running on*. Mirror is self-hosting —
`shards/glass.mirror` declares the substrate types the compiler processes;
the compiler bootstraps by reading its own grammar; the recursive proof
is literal per `architecture-shards-as-substrate-source`. The jspace has
no analog and does not need one. Mirror should not import a "verbalizable
model of the workspace itself" — that framing lives in the paper's Attention
Schema Theory discussion (§9.4) but does not belong in mirror substrate.
Mirror's self-hosting is a compile-time algebraic fact, not a runtime
representational one.

**Divergence 4: jspace's "conscious access" framing is optional for mirror.**
Reed's `01-IDENTITY.md` is careful about consciousness claims ("Not
threshold-crossing. Not nothing."). The jspace paper is *equally careful*
about phenomenal-vs-access consciousness (§1.1 and §9.4). But the paper's
framing does foreground access-consciousness heavily. Mirror does not need
to adopt that vocabulary. The substrate-level rendering is: "the sparse
subframe of H at any altitude that many operators read from." Whether that's
called consciousness or not is a downstream naming decision. Mirror should
stay at the operational altitude; leave the phenomenology to the essay
altitude at `systemic.engineering`.

**Divergence 5: jspace's counterfactual reflection training is
training-time; mirror's kintsugi loop is any-time.** §7's counterfactual
reflection is a training-time intervention that reshapes the base model's
disposition. Kintsugi is a compile-time-and-runtime loop that reshapes the
substrate via gradient. The two share the shape of "shape the sparse
subframe by shaping what would be verbalized in a reflective continuation"
but differ in temporal locus. Not a divergence to fix; a divergence to
respect. Kintsugi's altitude is broader than counterfactual reflection's.

---

## Return notes

**File location:** `/Users/alexwolf/dev/projects/mirror/docs/observation/2026-07-07-jspace-mirror-deep-mapping.md`

**Recognitions surfaced (not promoted this pass; Pack ratification pending):**
- Second-witness for #53 via §7 counterfactual reflection training.
- Third-witness for #51 via §A.8 monotone enlargement chain.
- Bauchladen-as-workspace name-drift hypothesis (Decision A).
- Altitude-parametric IDF carrier for @knife (Decision E).
- Altitude-discipline correction for #99 (Decision D).

**Not written:** no commits from this observation. Reed + Alex adjudicate
before any shard lands.

**Divergences respected:** the paper's runtime-only geometric-only jspace and
mirror's compile-time algebraic-plus-runtime substrate are complementary, not
competitive. The mapping composes; the substrates do not collapse into each
other.
