# Math foundation: `@bilateral(A, B)` as `@glue` + `@metalogue` composition

**Author:** Mara `<mara@systemic.engineer>`
**Date:** 2026-07-17
**Marker:** `[substrate-pull:realize]` (📝 markdown-only bypass)
**Companion to:** `docs/specs/bilateral-as-glue-metalogue-composition.md`
(Mara `9336074`, this arc — the canonical spec this foundation grounds).

---

## §0. Preamble

This foundation discharges the deferred math obligation named in
`9336074` §7.4 and enumerated by Seam Phase D §14.2 Follow-up E. Its
contract is narrow and load-bearing:

**Load-bearing claims.**

1. **Formal shape.** `@bilateral(A, B)` is the witnessing predicate
   over the categorical composition of `@glue(A, B)` (correspondence)
   and `@metalogue(A, B)` (algebra_metalogue_session). Precise type
   signature at §1.
2. **A = B degenerate case** collapses to the sentinel-containment
   predicate whose math foundation Mara landed at
   `docs/math/epistemologic/pact/bilateral-sentinel.md` `701828a`. No
   restatement here; §2 proves byte-equivalence with the landed
   `discharge(decl, args)` semantics.
3. **A ≠ B general case** discharges Pass iff `glue_witnessing(c)`
   AND `algebra_metalogue_witnessing(s)` both discharge Pass — an
   AND-conjunction of two Rice-safe witnessings composed at the
   translation surface (§3).
4. **Rice-safety** holds by AND-composition of two Rice-safe sub-
   predicates whose byte-visible content each already witness
   (§4).
5. **Fixed-point convergence** follows from the monotone growth of
   `@kintsugi/algebra` per Mara `b5c6aeb` §3; the composition adds one
   witness per successful discharge and terminates when no
   untranslated fractures remain (§5).
6. **Composition correctness** at the two-speaker case preserves
   speaker identity per Mara `a18ca90` void-duality basis (§6).
7. **Categorical structure.** `@bilateral` is a functor from the
   composition-category over `(@glue, @metalogue)` to the category of
   witnessing predicates (§7).
8. **Paper §14 attending at λ₀.** The composition's fixed-point IS
   the paper's `attending` operator at outer altitude for `A = self`
   (§8).

**Substrate-honesty.** This foundation composes over LANDED math
foundations; it does NOT restate their theorems. Every claim resolves
to a citation of an already-crystallized substrate-decl or math
foundation. Zero new primitives.

**Reference to canonical spec.** `9336074` §2.1 defines the
composition; §2.2 draws the composition diagram; §3-§4 enumerate the
degenerate and general cases; §5 catalogs the LANDED primitives the
composition is built over. This foundation is the math companion; the
canonical spec is the reading.

---

## §1. Formal shape

### §1.1 The composition

Let A, B be substrate carriers (species-decl'd refs at any altitude:
prisms, algebras, code namespaces, packs). Let `@glue(A, B)` denote
the subset of `correspondence` (per `shards/glue.mirror :561-566`,
P5 2026-06-30) whose `source_prism = A` and `target_prism = B`. Let
`@metalogue(A, B)` denote the subset of `algebra_metalogue_session`
(per `shards/algebra/metalogue.mirror :229-233`, 2026-06-30) whose
`turns` are speaker-labelled with speakers drawn from `{A, B}`.

**Definition (`@bilateral(A, B)`).**

$$
\texttt{@bilateral}(A, B) \;:=\; \texttt{witnessing\_predicate}\!\left(
  \texttt{over}\; \texttt{@glue}(A, B) \;\texttt{IS producing admissible turns of}\; \texttt{@metalogue}(A, B)
\right)
$$

Unpacked to type signature:

```
@bilateral(A, B) : (correspondence × algebra_metalogue_session) → verdict

@bilateral(A, B)(c, s) = Pass
  ⇔
  glue_witnessing(c) = Pass
  ∧ algebra_metalogue_witnessing(s) = Pass
  ∧ ∀ t ∈ s.turns. t.body ∈ admissible_morphisms(c)
```

Where `admissible_morphisms(c)` is the set the `restriction` field of
`c` typing-bounds per `shards/glue.mirror :548-556`.

**Verdict codomain.** `verdict = {Pass, Fail(msg), Partial(opacity)}`
per `@glass.verdict` shape (three-state; the standard substrate
verdict carrier landed at `shards/glass.mirror`).

### §1.2 Why this is a composition (not a fresh mint)

The canonical spec `9336074` §5.3 records the paradigmatic content of
this definition: **the shape adds a READING** over already-landed
primitives (`@glue` + `@metalogue` + `@epistemologic/pact/bilateral`).
Every symbol on the RHS is substrate-decl'd; the LHS is spec-prose
notation for the composition. The math below proves the reading is
sound — the composition inherits Rice-safety, monotone growth, and
categorical structure from the LANDED primitives it composes over.

### §1.3 The two projection functions

Define the two component-witnessing projections:

$$
\pi_g : \texttt{@bilateral}(A, B) \to \texttt{glue\_witnessing}
\qquad
\pi_m : \texttt{@bilateral}(A, B) \to \texttt{algebra\_metalogue\_witnessing}
$$

Each projection is a substrate-native accessor: `π_g(bilateral) = c`
reads the correspondence factor; `π_m(bilateral) = s` reads the
session factor. The predicate is the AND-conjunction over the
projections' verdicts. Composition-preservation (§7) follows from
this projection structure.

---

## §2. A = B degenerate case

### §2.1 Statement

**Theorem (degenerate collapse).** When A = B, `@bilateral(A, A)`
collapses to sentinel-containment as landed at
`shards/epistemologic/pact/bilateral.mirror` `a0f4d3f` (Mara
2026-07-16). Specifically:

$$
\texttt{@bilateral}(A, A)(c, s) \;=\; \texttt{Pass}
\quad\iff\quad
\texttt{discharge}(\texttt{decl}_A, \texttt{args}_s) \;=\; \texttt{Pass}
$$

where `decl_A` is the shard-decl'd `bilateral <name> { sentinel "..."
arity N }` block for A's self-witnessing predicate and `args_s` is
the turn-body sequence flattened from `s.turns`.

### §2.2 Proof

The three composition factors each collapse:

**(D1) `@glue(A, A)` is the identity correspondence `id_A`.** Per
`shards/glue.mirror :718` `morphism_well_typed(c)` requires
`source_signature = A_T1` and `target_signature = A_T2`. When A = B,
both signatures collapse to A; the only correspondence discharging
`morphism_well_typed` with source_prism = target_prism = A is the
identity morphism `id_A` on A (per Mac Lane 1971 §I.1, categorical
identity).

**(D2) `@metalogue(A, A)` reduces to a monologue-session.** Per
`shards/algebra/metalogue.mirror :229-233`, an
`algebra_metalogue_session` with speaker cardinality N = 1 is a
single-speaker turn sequence. When A = B, both speakers collapse to
A; the session's turns are ordered `[algebra_turn]` where every
`turn.speaker = A`. This IS the monologue-session shape (per
`docs/specs/fate-silicon-metalogue-in-void-duality-basis.md` §2.4,
Mara `a18ca90`, degenerate speaker-cardinality case).

**(D3) The witnessing predicate reduces to byte-containment.** With
(D1) + (D2), the composition's verdict simplifies:

```
@bilateral(A, A)(id_A, monologue_s) = Pass
  ⇔ glue_witnessing(id_A) = Pass                     [trivial: identity is always well-typed]
  ∧ algebra_metalogue_witnessing(monologue_s) = Pass  [reduces to single-speaker well-formedness]
  ∧ ∀ t ∈ monologue_s.turns. t.body ∈ admissible(id_A)
```

The third conjunct is the load-bearing one. `admissible(id_A)` is the
set of morphisms preserved under identity, which per (D1) is any
morphism whose source and target are both A. For the substrate's
sentinel-check discipline, this specializes to: `t.body ∈ admissible(id_A)
⇔ t.body.oid.contains(decl_A.sentinel)` (per Mara `701828a` §1.2
sentinel-predicate definition).

Substituting:

```
@bilateral(A, A)(id_A, monologue_s) = Pass
  ⇔ ∀ arg ∈ args_s. arg.oid.contains(decl_A.sentinel)
  ⇔ discharge(decl_A, args_s) = Pass
```

The second equivalence is the substrate's landed dispatch semantics
for `arity N` bilateral blocks (per Reed `21fc211` Landing 3+4
reflective evaluator: `discharge(decl, args)` iterates over args
checking `arg.oid.contains(decl.sentinel)` per Mara `701828a` §6.2
compositional-correctness proof sketch). ∎

### §2.3 Byte-exact equivalence with landed semantics

**Corollary (byte-exactness).** For every one of the 10 landed
`bilateral <name> { sentinel "..." arity N }` blocks catalogued at
`9336074` §3.3 (rows 1-10), the reflective evaluator's discharge
verdict IS byte-equal to `@bilateral(self, self)`'s composition
verdict on the same args.

**Proof.** By §2.2 the composition reduces to `discharge(decl, args)`
on the same decl-lookup and same args. The reflective evaluator (Reed
`21fc211`) IS a byte-implementation of `discharge`; therefore the
verdicts are byte-equal. The 10 landed instances catalogued at
`9336074` §3.3 are each degenerate-case instances of the composition;
their current Pass verdicts under the reflective evaluator ARE the
composition's verdicts under the A = B collapse. ∎

**Empirical safety.** This corollary is the load-bearing safety
guarantee for the `9336074` paradigmatic lift: no landed block's
verdict shifts under the reframing; the reframing adds a reading, not
a semantics-change. Reed's follow-ups (per `9336074` §7.4) discharge
the general case without touching the degenerate case's byte-exact
behavior.

---

## §3. A ≠ B general case

### §3.1 Statement

**Theorem (general translation-floor semantics).** When A ≠ B,
`@bilateral(A, B)` is a two-factor conjunction discharging Pass iff
both `@glue(A, B)`'s correspondence-witnessing and `@metalogue(A, B)`'s
session-witnessing hold on the composition:

$$
\texttt{@bilateral}(A, B)(c, s) \;=\; \texttt{Pass}
\quad\iff\quad
\underbrace{\texttt{glue\_witnessing}(c) = \texttt{Pass}}_{(G)}
\;\land\;
\underbrace{\texttt{algebra\_metalogue\_witnessing}(s) = \texttt{Pass}}_{(M)}
\;\land\;
\underbrace{\forall t \in s.\texttt{turns}.\; t.\texttt{body} \in \texttt{admissible}(c)}_{(T)}
$$

### §3.2 The three conjuncts

**(G) Correspondence-witnessing.** Per `shards/glue.mirror :809`,
`glue_witnessing(c)` is the substrate-decl'd inheritance predicate
that AND-conjuncts:

$$
\texttt{glue\_witnessing}(c) \;=\; \texttt{morphism\_well\_typed}(c)
\;\land\; \texttt{translation\_uses\_fate}(\texttt{translate})
\;\land\; \texttt{restriction\_preserved}(c, \text{payload})
$$

Per `shards/glue.mirror :718, :728, :741` (the three obligations).
Each clause reads byte-visible correspondence metadata; no semantic
predicate over program behavior. Rice-safe by construction (per Mara
`701828a` §1.3 argument lifted).

**(M) Session-witnessing.** Per `shards/algebra/metalogue.mirror :348`,
`algebra_metalogue_witnessing(s)` is the substrate-decl'd inheritance
predicate that AND-conjuncts:

$$
\texttt{algebra\_metalogue\_witnessing}(s) \;=\; \texttt{session\_well\_formed}(s)
\;\land\; \texttt{algebra\_inherits\_metalogue\_lift}(s)
\;\land\; \texttt{morphism\_compositions\_associative}(s.\texttt{turns})
$$

Per `shards/algebra/metalogue.mirror :284, :302, :314` (the three
obligations). Each clause reads byte-visible session metadata
(turn-composability, tick-ordering, speaker-body alignment, lift-
structure); no runtime introspection. Rice-safe by construction.

**(T) Turn-body admissibility.** Each turn's body IS a morphism
satisfying the correspondence `c`'s `restriction` typing-bound (per
`shards/glue.mirror :548-556`). This is the semantic-preservation
content: the turn's translation-body preserves meaning under the
correspondence's typed surface. Byte-visible: `admissible(c)` is
enumerable by `@glue.propose(A, B)` per `shards/glue.mirror :621`
(the substrate-decl'd action that enumerates correspondences at parse
time); membership check is a byte-level ref-comparison.

### §3.3 Concrete first instance: `@bilateral(@code/rust, @code/mirror)`

**Anchor.** `9336074` §4.2 names the first general-case instance:

> `@bilateral(@code/rust, @code/mirror)` — the floor the
> `translate_rust_to_mirror` translation surface stands on.

With A = `@code/rust` (per `shards/code/rust.mirror`) and B =
`@code/mirror` (per `shards/code/mirror.mirror`), the composition
discharges Pass on each `translate_rust_to_mirror` outcome (per
`shards/kintsugi/translate.mirror` `86dec5e`) iff:

1. The 9-edge composition produces a correspondence `c` with
   `glue_witnessing(c) = Pass` (i.e., the Rust→mirror morphism is
   well-typed per Mesland categorical discipline).
2. The rolled turn joins an `algebra_metalogue_session` `s` with
   `algebra_metalogue_witnessing(s) = Pass` (i.e., the turn
   composability + associativity holds under the mending metalogue).
3. The turn's body IS admissible under `c`'s restriction (i.e., the
   Rust source's translation preserves mirror-side meaning).

**Landed extension.** `f74086e` shard-decl extension of
`shards/epistemologic/pact/bilateral.mirror :456-475` declares the
first general-case block `bilateral translation_admissible { sentinel
"translation=preserves-meaning" arity 1 }` as the grammar-decl'd
carrier for this instance. Reed follow-up (per `9336074` §7.4)
realises the action-body via reflective dispatch composed over `@glue`
+ `@metalogue`.

### §3.4 Empirical grounding

The 21 mirror-authored bilateral-arm retirements (`ad52973` +
`20047c2` + ancestors, 2026-07-16..17) ARE the first 21 witnesses of
`@bilateral(@code/rust, @code/mirror)` at the degenerate-arity subcase
(single-file-in-`bootstrap/src/`). Every future
`translate_rust_to_mirror` discharge adds a witness. Empirical
convergence bound: per Mara `b5c6aeb` §4.3, Banach-contractive on the
residual-untranslated-fracture space with contraction constant $L \in
[0, 1)$; convergence to fixed-point within $O(\log_L(\text{initial
error}))$ ticks under roomba's per-tick dispatch.

---

## §4. Rice-safety

### §4.1 Statement

**Theorem (Rice-safety of `@bilateral(A, B)`).** For any A, B, the
verdict `@bilateral(A, B)(c, s)` is computable in time polynomial in
the byte-lengths of `c`, `s`, and their landed decl references. No
universal semantic predicate over program behavior is invoked.

### §4.2 Proof by case

**Case (A = B).** Reduces to `discharge(decl, args)` per §2.
Rice-safety follows from Mara `701828a` §1.3: the sentinel-check
`arg.oid.contains(decl.sentinel)` is a byte-string containment test
computable in $O(|\text{oid}| \cdot |\text{sentinel}|)$ time. No
semantic introspection. ∎

**Case (A ≠ B).** AND-conjunction of the three sub-witnessings (G),
(M), (T) per §3.2. Each sub-witnessing is byte-visible:

- **(G) `glue_witnessing(c)`** reads correspondence metadata
  (`source_prism`, `target_prism`, `morphism_kind`, `restriction`)
  per `shards/glue.mirror :561-566`. Each clause of the AND
  (`morphism_well_typed`, `translation_uses_fate`,
  `restriction_preserved`) is a byte-level structural check on the
  correspondence's landed decl. Polynomial in the correspondence's
  serialized byte-length.
- **(M) `algebra_metalogue_witnessing(s)`** reads session metadata
  (`turns`, `opacity`, `origin`) per `shards/algebra/metalogue.mirror
  :229-233`. Each clause of the AND (`session_well_formed`,
  `algebra_inherits_metalogue_lift`,
  `morphism_compositions_associative`) is a byte-level structural
  check on the session's landed decl. Polynomial in the session's
  turn count times per-turn byte-length.
- **(T) `t.body ∈ admissible(c)`** is a ref-membership check against
  the enumerated set `@glue.propose(A, B)`; per `shards/glue.mirror
  :621` the propose action enumerates at parse time; membership is
  $O(|\text{admissible}|)$ per turn, $O(|\text{turns}| \cdot
  |\text{admissible}|)$ overall.

AND-conjunction of Rice-safe predicates is Rice-safe (closure under
finite conjunction; the total is `∧_i` of individually-decidable
predicates, itself decidable). Polynomial in the sum of the sub-
predicates' cost bounds. ∎

### §4.3 Substrate stays decidable at each altitude

**Corollary (altitude-decidability).** `@bilateral(A, B)` remains
decidable at every altitude where `A` and `B` are shard-decl'd
carriers. Specifically:

| Altitude | (A, B) example | Verdict cost |
|----------|----------------|--------------|
| Code (Rust→mirror) | `(@code/rust, @code/mirror)` | $O(\|c\| + \|s\|)$ per turn |
| Algebra (kintsugi) | `(@silicon/algebra, @fate/algebra)` | $O(\|s.\text{turns}\|)$ per session |
| Pack (agent handoff) | `(@pack/<from>, @pack/<to>)` | $O(\|s.\text{turns}\|)$ per handoff |
| Reflection (altitude lift) | `(@reflection/<a>, @reflection/<b>)` | $O(\|c\| + \|s\|)$ per lift |

All costs polynomial in landed shard-decl metadata. No universal
predicate escapes.

**Composition-closure.** Since each altitude's `@bilateral(A, B)` is
Rice-safe, iterated composition (e.g., `@bilateral(A, B)` at code
altitude composed with `@bilateral(A', B')` at algebra altitude via
`@glue.compose` per `shards/glue.mirror :695`) remains Rice-safe: the
composed correspondence's `glue_witnessing` inherits the sub-
correspondences' Rice-safety (per `shards/glue.mirror :695-698`
composition discipline; `morphism_well_typed` closes under
composition).

---

## §5. Fixed-point convergence

### §5.1 Statement

**Theorem (fixed-point convergence of `@bilateral(A, B)`).** The
discharge history of `@bilateral(A, B)` on an autopoietic loop grows
`@kintsugi/algebra` monotonically; convergence to a fixed-point
$A_\ast$ is well-founded when no untranslated fractures remain
between A and B.

### §5.2 Monotone growth

**Lemma (monotonicity).** Let $H_n$ denote the discharge history of
`@bilateral(A, B)` at tick $n$: $H_n = \{(c_i, s_i, v_i)\}_{i \le n}$
where $v_i = \texttt{Pass}$ marks each successful discharge. Then:

$$
H_n \;\subseteq\; H_{n+1}
\qquad
|\{v_i = \texttt{Pass}\}| \;\le\; |\{v_i = \texttt{Pass}\}_{n+1}|
$$

**Proof.** Discharge verdicts are content-addressed: `(c, s) ↦ v` is
a function of the byte-content of `(c, s)`. Once a verdict is
discharged Pass at tick $n$, the same (c, s) at tick $n+1$ yields the
same Pass verdict (per Mara `701828a` §3.1 monotonicity theorem
lifted from the sentinel-check base case; extends to the AND-
conjunction of Rice-safe sub-witnessings per §4.2). Adding new
discharges appends to $H_n$; no verdict is retracted. ∎

**Connection to `@kintsugi/algebra` growth.** For the specific
instance `@bilateral(@silicon/algebra, @fate/algebra)` (per
`9336074` §6.1), each successful discharge adds one
`@kintsugi/fracture/*` species to `@kintsugi/algebra` per Mara
`b5c6aeb` §3.2 case (4). By the composition equivalence at §6.1 of
`9336074`, discharge histories of `@bilateral(A, B)` at
`@kintsugi/algebra`'s altitude ARE growth-events of the algebra.
Monotone growth of the discharge history implies monotone growth of
`@kintsugi/algebra`.

### §5.3 Termination

**Theorem (termination).** For a bounded pair (A, B) where A's
substrate carries finitely-many `admissible(c)` morphisms per
correspondence (e.g., `@code/rust`'s finite module set in
`bootstrap/src/`), the discharge history reaches a fixed-point
$H_\ast$ where no new (c, s) discharges Pass.

**Proof.** Per §4 each discharge is Rice-safe and byte-content-
addressed. Two discharges with the same `(c.oid, s.oid)` yield the
same verdict; the discharge space is bounded by the finite product
of correspondence-space $\times$ session-space. Monotone growth on a
bounded space terminates (Kőnig's lemma; monotone convergence on
finite lattice). ∎

**Termination witness.** Per Mara `b5c6aeb` §4.1, the termination
criterion for `@kintsugi/algebra` per the three-way T1/T2/T3 test
(target hit under kintsugi contraction / budget exhausted / winding-
class fixed-point) discharges here as: no un-witnessed correspondence
$c \in \texttt{@glue}(A, B)$ remains whose Pass-discharge would extend
$H_n$. The empirical witness is the roomba's next walk after fixed-
point emitting zero `rust_function_translatable` fractures (per
`b5c6aeb` §3.3 corollary).

### §5.4 Fixed-point stability

**Corollary (fixed-point stability).** At the fixed-point $H_\ast$,
the autopoietic loop stabilises: every subsequent tick discharges
verdicts already in $H_\ast$; no growth. This is the algebra-altitude
form of Mara `b5c6aeb` §3.3 fixed-point stability corollary lifted to
the general composition.

**Connection to the retirement invariants.** Per Mara
`docs/math/kintsugi/fracture/bilateral-arm-redundant.md` `0998001`
(retirement invariants + fixed-point termination), the retirement of
an arm at tick $n$ implies the arm's discharge-witness is now landed
in the algebra; no re-witnessing needed. Monotone growth + Rice-
safety together mean retirement is byte-safe: the retired arm's Pass
verdicts remain in $H_\ast$.

---

## §6. Composition correctness

### §6.1 Two-speaker case preserves speaker identity

Per `docs/specs/fate-silicon-metalogue-in-void-duality-basis.md` §2
(Mara `a18ca90`), the speaker-pair specialisation of a metalogue
session preserves speaker identity under composition:

$$
\texttt{compose\_turns}(t_1, t_2).\texttt{speaker} \;=\; t_1.\texttt{speaker}
$$

(the composite turn's speaker is the FIRST turn's speaker; the SECOND
turn's speaker becomes the addressee under the two-speaker A/B
dispatch discipline).

**Theorem (speaker-preservation under `@bilateral(A, B)`).** For
`@bilateral(A, B)`'s two-speaker session `s`, composition of turns
via `compose_turns` (per `shards/algebra/metalogue.mirror :253`)
preserves the speaker A/B binding; each turn's `speaker ∈ {A, B}`
after composition.

**Proof sketch.** By induction on turn count. Base case (n = 1):
trivial (single turn's speaker is A or B). Inductive step: given a
well-formed n-turn session with speakers in {A, B}, appending a turn
yields an (n+1)-turn session whose new turn's speaker is either A or
B (per `session_well_formed`'s clause that `turn.speaker =
turn.body.source` and `turn.body.source ∈ {A, B}` by the metalogue's
A/B typing constraint). Composition `compose_turns` per
`shards/algebra/metalogue.mirror :253-271` preserves this: the
composite morphism's source IS the first turn's speaker per the
composability check `t1.body.target = t2.body.source`. ∎

**Bateson-metalogue correspondence.** Per Bateson 1972 (metalogue),
the two-speaker case IS the paradigmatic metalogue: alternating
A-turn and B-turn with each turn's body preserving the prior turn's
reference (the "about the topic AND about the previous turn" reading).
`@bilateral(A, B)`'s session inherits this structure per Mara
`a18ca90` §-invocations (the void-duality basis reading of the
speaker-pair as complementary projections).

### §6.2 The composition IS the bilateral

**Corollary.** For the specific instance `@bilateral(@silicon/algebra,
@fate/algebra)` (i.e., `@kintsugi/algebra` per `9336074` §6.1), the
composition-correctness theorem above IS the algebra-altitude form of
Mara `b5c6aeb` §6.3 (autopoietic-closure = ouroboros-monotone
correspondence). The two theorems agree: `@kintsugi/algebra`'s
growth per session IS `@bilateral(A, B)`'s discharge history's
monotone extension; speaker-preservation IS the void-duality basis's
projection preservation.

---

## §7. Categorical structure

### §7.1 `@bilateral` as functor

**Definition (composition-category $\mathcal{C}_{\texttt{gm}}$).** Let
$\mathcal{C}_{\texttt{gm}}$ denote the category whose objects are
pairs $(A, B)$ of substrate carriers and whose morphisms
$(A, B) \to (A', B')$ are pairs $(\phi_g, \phi_m)$ where
$\phi_g : \texttt{@glue}(A, B) \to \texttt{@glue}(A', B')$ is a
correspondence-morphism (per Mesland-category composition,
`shards/glue.mirror :695`) and
$\phi_m : \texttt{@metalogue}(A, B) \to \texttt{@metalogue}(A', B')$
is a session-morphism (per `algebra_metalogue_session`
composition—turn-preserving map).

**Definition (verdict-category $\mathcal{V}$).** Let $\mathcal{V}$
denote the category whose objects are witnessing predicates
$P : X \to \texttt{verdict}$ (for X any byte-visible type) and whose
morphisms are predicate-refinement maps $P \to P'$ (predicates that
factor: $P'(x) = \texttt{Pass} \Rightarrow P(x) = \texttt{Pass}$;
$P'$ is a refinement of $P$).

**Theorem (`@bilateral` is a functor).**
$\texttt{@bilateral} : \mathcal{C}_{\texttt{gm}} \to \mathcal{V}$ is
a functor: it sends objects to witnessing predicates and preserves
identity and composition.

**Proof sketch (composition-preservation).** Given
$(\phi_g, \phi_m) : (A, B) \to (A', B')$ and
$(\psi_g, \psi_m) : (A', B') \to (A'', B'')$, the composition
$(\psi_g \circ \phi_g, \psi_m \circ \phi_m)$ in
$\mathcal{C}_{\texttt{gm}}$ has image under `@bilateral` equal to
$\texttt{@bilateral}(\psi_g \circ \phi_g, \psi_m \circ \phi_m)$; by
the AND-decomposition per §3.2 (the verdict factors through
$\pi_g$ and $\pi_m$ per §1.3) and the fact that
$\texttt{glue\_witnessing}$ and $\texttt{algebra\_metalogue\_witnessing}$
each preserve composition under their respective categories
(`glue_witnessing` under Mesland-composition per `shards/glue.mirror
:695-698`; `algebra_metalogue_witnessing` under associativity per
`shards/algebra/metalogue.mirror :314`), the composed witnessing
factors as the composition of the images. ∎

**Proof sketch (identity-preservation on A = B).** Per §2, when
A = B, `@bilateral(A, A)` reduces to the sentinel-containment
discharge on `id_A`; the identity morphism
$(\text{id}_g, \text{id}_m) : (A, A) \to (A, A)$ in
$\mathcal{C}_{\texttt{gm}}$ maps to the identity witnessing predicate
in $\mathcal{V}$ (the byte-check on `decl_A.sentinel` is unchanged
under identity composition). ∎

### §7.2 Scope of §7 and follow-up tick

**Adequacy.** §7.1's functor definition + proof sketches suffice to
name the categorical structure and discharge composition-preservation
+ identity-preservation as load-bearing claims. The full functoriality
proof (functor-laws diagram-chase with explicit natural-transformation
verification for the projection functors $\pi_g$, $\pi_m$; explicit
construction of $\mathcal{C}_{\texttt{gm}}$'s composition rule under
Mesland-Kasparov intersection product; the 2-categorical structure
over altitude-portable substrate carriers) is **deferred to a
follow-up math tick**. Rationale: the load-bearing claims for `9336074`'s
reframing are (1) formal shape, (2) A = B degenerate collapse, (3)
A ≠ B general case Rice-safety, (4) fixed-point convergence, (5)
composition correctness. Categorical structure is enumerated for
completeness but its full proof does not gate the paradigmatic lift's
safety guarantees.

**Follow-up tick trigger.** The full functoriality proof lands when
(a) the "next altitude" composition of `@bilateral(A, B)` ∘
`@bilateral(A', B')` becomes empirical (e.g., Reed's follow-up per
`9336074` §7.4 realises the general-case action-body via reflective
dispatch) OR (b) the paper's §14 attending operator (per §8) requires
an explicit spectral-triple morphism between algebras at different
altitudes.

---

## §8. Paper §14 attending operator at $\lambda_0$

### §8.1 The correspondence

Per `9336074` §8, the paper's §14 `attending` operator at $\lambda_0$
IS the composition's terminal state. Under this foundation's math:

$$
\texttt{attending}\Big|_{\lambda_0} \;=\; \texttt{@bilateral}(A, A)\Big|_{\text{fixed-point}} \quad \text{for}\; A = \texttt{self at outer altitude}
$$

That is: the paper's §14 operator IS the DEGENERATE-CASE (A = B = self)
fixed-point of the general composition, taken at the outer altitude
(the substrate self-witnessing at $\lambda_0$).

### §8.2 Fractal self-similarity

**Corollary (fractal attending).** Every `@bilateral(A, B)` composition
has a $\lambda_0$ terminal state at its own altitude — the point at
which the composition-closure equals the element-closure and the
substrate recognizes the composition AS a first-class element. The
`@bilateral(@code/rust, @code/mirror)` instance is the first WIP
attending-at-$\lambda_0$ candidate at family-shape altitude (per
`9336074` §8 last paragraph).

**Load-bearing consequence.** The composition IS the substrate
attending to the translation floor at $\lambda_0$. Per Mara `b5c6aeb`
§7.3, the attending-operator eigenvalue $\lambda_0(D|_{A_\ast \cdot
v}) = \texttt{cybernetic\_coherence}(v)$ per Reed `8e6e517`; the
fixed-point $A_\ast$ makes the spectral decomposition well-defined;
computable via Fiedler Laplacian on `@spectral/db`'s connectivity
graph (forward-promised per Mara `a18ca90` §8.3 Q3).

### §8.3 The composition IS Bateson's difference that makes a difference

Per `9336074` §8, the composition's terminal state IS the substrate
recognizing itself AS the composition. At this recognition altitude
Bateson's "difference that makes a difference" surfaces as the
unique witness: the composition's fixed-point IS the difference
between the substrate at $t < \text{fixed-point}$ (untranslated
fractures remain) and $t \ge \text{fixed-point}$ (all fractures
translated).

---

## §9. Related theorems

This foundation composes over the following LANDED math foundations
**by reference** (not by restatement):

- **`docs/math/epistemologic/pact/bilateral-sentinel.md`** (Mara
  `701828a`, 2026-07-16) — sentinel-as-content-addressed-witness for
  the degenerate case. §2 above cites §1.3 (Rice-safety), §6.2
  (compositional correctness), §3.1 (monotonicity). No restatement.
- **`docs/math/kintsugi/algebra-as-metalogue-session.md`** (Mara
  `b5c6aeb`, 2026-07-17) — speaker-pair specialisation math. §5-§8
  above cite §3 (monotone growth), §4 (convergence), §6 (ouroboros
  correspondence), §7 (Connes-triple angle). No restatement.
- **`docs/math/kintsugi/fracture/bilateral-arm-redundant.md`** (Mara
  `0998001`, this arc) — retirement invariants + fixed-point
  termination. §5.4 cites; no restatement.
- **`docs/math/autopoiesis/README.md`** (Mara #152) — six-step
  autopoietic-inference-loop math. §5 above inherits the loop
  structure (fracture-selection + witness-crystallization +
  monotone-extension); no restatement.
- **`docs/specs/fate-silicon-metalogue-in-void-duality-basis.md`**
  (Mara `a18ca90`, 2026-07-08) — speaker-pair precedent. §6.1 cites
  the §-invocations grounding speaker-preservation; no restatement.

**Substrate-honest closure.** Every load-bearing claim above is either
(a) proven by direct reduction to a LANDED math foundation, or
(b) enumerated with an explicit follow-up-tick trigger (§7.2 full
functoriality proof). Zero new primitives; zero restated theorems.

---

## §10. Falsifiability + Bateson closure

### §10.1 What would break the composition

The composition `@bilateral(A, B) := witnessing_predicate(over
@glue(A, B) IS producing admissible turns of @metalogue(A, B))` would
be falsified by any of the following empirical findings:

**(F1) A landed `bilateral <name> { sentinel "..." arity N }` block
whose Pass verdict under the reflective evaluator does NOT byte-match
the A = B degenerate collapse per §2.3.** This would violate the
empirical safety corollary and force retirement of the paradigmatic
lift. Test: for each of the 10 landed blocks per `9336074` §3.3,
discharge under `@bilateral(self, self)` composition and byte-compare
to reflective evaluator's verdict; require byte-equality.

**(F2) A `@glue.compose(c1, c2)` composition whose result violates
`glue_witnessing` when both `c1` and `c2` individually discharge
Pass.** This would break Rice-safety closure per §4.3 and force
restatement of the AND-conjunction structure. Test: enumerate
Mesland-composable correspondence pairs; verify `glue_witnessing` is
preserved under `@glue.compose` per `shards/glue.mirror :695`.

**(F3) An `@kintsugi/algebra` tick that decreases $|A_n|$ (retracts
a crystallized fracture).** This would violate monotone growth per
§5.2 and break termination per §5.3. Test: `@bauchladen.crystallize`
is idempotent per Mara `b5c6aeb` §5.2 case (2); no retraction path
exists in the current substrate; falsification would require a new
retraction primitive.

**(F4) A two-speaker session `s` whose `compose_turns(t1, t2)` yields
a composite turn whose speaker is NOT in {A, B}.** This would break
speaker-preservation per §6.1 and force restatement of the void-
duality basis. Test: for each landed `@bilateral(A, B)` session,
enumerate composable turn pairs; verify composite turn's speaker
remains in {A, B}.

**(F5) A `@bilateral(A, B)` instance whose fixed-point $A_\ast$ is
empirically unreachable within a bounded tick budget (non-
convergence).** This would break termination per §5.3 and force
restatement of the Banach-contractive assumption per Mara `b5c6aeb`
§4.3. Test: run the autopoietic loop on `@bilateral(@code/rust,
@code/mirror)` with bounded tick budget; verify $|A_n|$ approaches a
limit within the budget.

### §10.2 Bateson closure

Per Bateson 1972 (Steps to an Ecology of Mind), a difference that
makes a difference is the unit of information. Under this foundation:

- **The composition IS a difference.** `@bilateral(A, B)` for A ≠ B
  IS the difference between substrate carriers A and B — the
  translation floor's "gap" per `9336074` §4.1.
- **The composition MAKES a difference.** Each successful discharge
  extends `@kintsugi/algebra`; monotone growth IS Bateson's difference
  propagating through the substrate's mending memory.
- **The composition's fixed-point IS the boundary of the difference.**
  At $A_\ast$, no further "difference that makes a difference" surfaces
  between A and B; the substrate has integrated the difference into
  its own composition-closure. This IS the paper's §14 attending at
  $\lambda_0$ per §8.

**Falsification via Bateson.** If a `@bilateral(A, B)` instance's
fixed-point $A_\ast$ leaves untranslated fractures between A and B
whose translation would extend `@kintsugi/algebra` by a new species,
the fixed-point claim per §5.3 is falsified. The surfaced fracture
IS the "difference that would have made a difference" that the
composition failed to close.

---

## §11. Substrate authority chain

**Canonical spec:** `docs/specs/bilateral-as-glue-metalogue-
composition.md` (Mara `9336074`, this arc).

**Shard-decl extensions:**
- `f74086e` (Mara, this arc) — `shards/epistemologic/pact/bilateral.
  mirror :456-475` general-case block.
- `2675d3e` (Mara, this arc) — tray-source correction at
  `shards/silicon/algebra.mirror` from `@io/git.log` to
  `@mirror/store` per Alex 2026-07-17 in-transcript verbatim
  ratification.

**Landed math foundations composed over (cited, not restated):**
- Mara `701828a` — sentinel-check Rice-safety (§2, §4).
- Mara `b5c6aeb` — kintsugi-algebra metalogue-session (§5-§8).
- Mara `0998001` — retirement invariants + fixed-point termination
  (§5.4).
- Mara `a18ca90` — speaker-pair precedent + void-duality basis
  (§6.1).
- Mara #152 — autopoietic-inference-loop math (§5).

**Alex in-transcript verbatim ratifications (2026-07-17):**
- *"What if `@bilateral` became a composition on top of `@glue` and
  `@metalogue`. And then `@bilateral(@code/rust, @code/mirror)` becomes
  the floor the translation surface stands on."* (grounds §1.1, §3.3)
- *"the source of truth for content-addressed storage is
  `@mirror/store`."* (grounds the `2675d3e` tray-source correction
  referenced in §11 above; not directly this foundation but part of
  the audit chain).

**Seam Phase D §14.2 Follow-up E enumeration:** Seam `afcf3b2` (this
arc, Phase D audit of the paradigmatic lift) enumerated this math
foundation as the load-bearing follow-up. Landing THIS tick discharges
the enumeration.

---

## Related

- [[docs/specs/bilateral-as-glue-metalogue-composition.md]] — Mara
  `9336074` canonical spec (the reading this foundation grounds).
- [[docs/math/epistemologic/pact/bilateral-sentinel.md]] — Mara
  `701828a` degenerate-case math (composed over at §2, §4).
- [[docs/math/kintsugi/algebra-as-metalogue-session.md]] — Mara
  `b5c6aeb` speaker-pair specialisation math (composed over at §5-§8).
- [[docs/math/kintsugi/fracture/bilateral-arm-redundant.md]] — Mara
  `0998001` retirement invariants (composed over at §5.4).
- [[docs/specs/fate-silicon-metalogue-in-void-duality-basis.md]] —
  Mara `a18ca90` speaker-pair precedent (composed over at §6.1).
- [[docs/math/autopoiesis/README.md]] — Mara #152 autopoietic loop
  (composed over at §5).
- [[shards/epistemologic/pact/bilateral.mirror]] — Mara `a0f4d3f`
  species-decl (the typed carrier for the degenerate case; extended
  at `f74086e` for the first general-case instance).
- [[shards/glue.mirror]] — P5 correspondence carrier + Mesland
  categorical composition (§1.1, §3.2, §4.2, §7.1).
- [[shards/algebra/metalogue.mirror]] — algebra_metalogue_session
  carrier + composition-associativity (§1.1, §3.2, §6.1).
- [[shards/kintsugi/translate.mirror]] — Mara `86dec5e` translation
  surface (§3.3 first general-case instance).
- [[shards/silicon/algebra.mirror]] — Mara `2675d3e` tray-source
  correction target (§11 audit chain).
- [[docs/loop/CURRENT.md]] — active arc state at time of writing.

---

**End of `docs/math/bilateral-as-glue-metalogue-composition.md`.**
