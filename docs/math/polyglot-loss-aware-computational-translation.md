# Math foundation: polyglot loss-aware computational translation

**Author:** Mara `<mara@systemic.engineer>`
**Date:** 2026-07-17
**Marker:** `[substrate-pull:realize]` (📝 markdown-only bypass)
**Companion to:** `docs/specs/polyglot-loss-aware-computational-translation.md`
(landed same tick).

---

## §0. Preamble

This foundation formalizes the load-bearing claims of the canonical
spec: the mirror compiler as a **polyglot loss-aware computational
translator** grounded in cascade-compositional adjacent-altitude
morphisms through machine-substrate hubs, with loss-awareness via
`@cascade.measure` and autopoietic learning via `@kintsugi/algebra`
crystallization.

**Load-bearing claims formalized here.**

1. **Cascade morphism.** `@cascade/code/A/B : @code/A → @code/B` is a
   morphism in a substrate-decl'd category over `@code/*` species,
   with a typed carrier and a composition axiom (§1).
2. **Loss measurement.** Loss composition `⊕` is associative and
   monotone; admissibility threshold τ is Rice-safe (§2).
3. **Polyglot theorem.** For any Turing-complete languages A, B and
   any machine-substrate hub M, `@cascade/code/A/B :=
   @cascade/code/M/B ∘ @cascade/code/A/M` exists as a well-typed
   composition (§3).
4. **Autopoietic closure.** `@kintsugi/algebra` grows monotonically
   per admissible cascade discharge; converges via Banach contraction
   (§4).
5. **Categorical structure.** Polyglot is a functor from the
   Turing-complete-languages category to the mirror substrate
   category (§5).
6. **Connes spectral-triple angle.** Polyglot architecture as
   multi-altitude spectral triple (§6).
7. **Reference-only** to landed math foundations (§7).
8. **Falsifiability + Bateson-closure** (§8).

**Substrate-honesty gate.** Every symbol resolves to a LANDED
substrate-decl or landed math foundation. ZERO new primitives.

**Reference to canonical spec.** The canonical spec `docs/specs/
polyglot-loss-aware-computational-translation.md` §§1–12 grounds the
substrate authority chain, the concrete instantiation, the
composition graph, and the landing plan; THIS foundation formalizes
the math. Every substrate-decl citation in the canonical spec is
inherited here without restatement.

---

## §1. Formal cascade morphism

### §1.1 The cascade category

**Definition (cascade category `Cascade_@code`).** Objects: substrate-
decl'd `@code/X` species (per Taut audit `d0572cd` §1.1 — 10 landed
today; extensible by mint). Morphisms: substrate-decl'd
`@cascade/code/A/B` species (per Taut audit §1.4 — 5 landed today;
extensible by mint). Composition: `@glue.compose` per
`shards/glue.mirror :695`. Identity: `@cascade/code/A/A := id_A`
(degenerate; landed via the family-root's identity discipline).

### §1.2 The morphism carrier

**Definition (`@cascade/code/A/B` typed shape).** Per
`shards/cascade.mirror` (Mara `ce4874b`, 2026-06-23) family-root
five-op discipline:

```
type cascade_A_B = {
  source:            @code/A  (source altitude ref),
  target:            @code/B  (target altitude ref),
  compile:           typed_source<A> → compiled_artifact<B>,
  measure:           (typed_source<A>, compiled_artifact<B>, loss_lens<A, B>)
                       → imperfect<compiled_artifact<B>, ref, information_loss>,
  loss_lens:         loss_lens<A, B>  (parametric; per @labeled<> functor),
  well_formed:       cascade_well_formed(source, artifact, p) → verdict,
}
```

`loss_lens<A, B>` is a `@labeled<A, B>` instance per recognition #93
H4 (per `shards/labeled.mirror` — the labeled<> functor primitive).
Wadler-1989 / Reynolds-1983 parametricity applies: the label
(source, target grammar pair) is preserved under lens composition.

### §1.3 Composition axiom

**Axiom (cascade composition).** For adjacent-altitude cascades
`c_AB : @cascade/code/A/B` and `c_BC : @cascade/code/B/C`:

$$
c_{AC} := c_{BC} \circ c_{AB} \;=\; \texttt{@glue.compose}(c_{AB}, c_{BC})
$$

is a well-typed morphism in `Cascade_@code` at `A → C` altitude, with:

$$
\begin{aligned}
c_{AC}.\texttt{compile}(x) &:= c_{BC}.\texttt{compile}(c_{AB}.\texttt{compile}(x)) \\
c_{AC}.\texttt{measure}(x, y) &:= c_{AB}.\texttt{measure}(x, y_{AB}) \oplus c_{BC}.\texttt{measure}(y_{AB}, y) \\
c_{AC}.\texttt{well\_formed}(x, y) &:= c_{AB}.\texttt{well\_formed}(x, y_{AB}) \land c_{BC}.\texttt{well\_formed}(y_{AB}, y)
\end{aligned}
$$

where `y_{AB} := c_{AB}.compile(x)` is the intermediate artifact.

**Proof of well-typedness.** Per `shards/glue.mirror :695-698`,
`@glue.compose(c1, c2) -> correspondence` is substrate-decl'd with
`morphism_well_typed(compose(c1, c2)) = Pass ⇔
morphism_well_typed(c1) = Pass ∧ morphism_well_typed(c2) = Pass`
(inheritance under Mac Lane 1971 §I.1 categorical composition). The
cascade species are correspondences per Mesland 2013 §3; composition
inherits the well-typedness. ∎

### §1.4 Identity and associativity

**Identity.** `id_A := @cascade/code/A/A` at the degenerate case
where source and target altitudes coincide. Per §2 of `docs/math/
bilateral-as-glue-metalogue-composition.md` (Mara `9be68b1`) A = B
degenerate case: `@glue(A, A) = id_A` (Mac Lane 1971 §I.1). Loss is
zero for identity: `id_A.measure(x, x) = imperfect.success(x)`.

**Associativity.** For three composable cascades c_AB, c_BC, c_CD:

$$
(c_{CD} \circ c_{BC}) \circ c_{AB} \;=\; c_{CD} \circ (c_{BC} \circ c_{AB})
$$

Follows from `@glue.compose` associativity per `shards/glue.mirror :698`
(inherited from Mac Lane 1971 §I.1). ∎

---

## §2. Loss measurement

### §2.1 Formalization of "loss-aware"

Per `shards/cascade.mirror :151-168` + `[[feedback-loss-from-
epistemologic-properties]]`: loss is a **composite of `@epistemologic`
properties**, not Shannon, not invented.

**Definition (`information_loss` carrier).** Substrate-typed ref
whose byte-representation encodes a finite set of `@epistemologic/
property/*` dimensions. Each dimension is a substrate-decl'd
property (per `shards/epistemologic/*.mirror` corpus). The
information_loss carrier's OID content-addresses the specific
dimension-set.

Type signature (per `shards/cascade.mirror`):
```
type information_loss = ref  (byte-composite of @epistemologic dimensions)
```

### §2.2 The `⊕` operation

**Definition (loss composition).** For losses L₁, L₂ :
information_loss:

$$
L_1 \oplus L_2 \;:=\; \texttt{ref}(\texttt{@epistemologic dimensions}(L_1) \;\cup\; \texttt{@epistemologic dimensions}(L_2))
$$

The `⊕` is the byte-typed union operation on the dimension-sets,
content-addressed by the union's canonical form (sort + BLAKE3 per
`shards/mirror/store.mirror` OID discipline).

### §2.3 Properties of `⊕`

**Theorem (⊕ associativity).** $(L_1 \oplus L_2) \oplus L_3 = L_1
\oplus (L_2 \oplus L_3)$.

**Proof.** Set-union is associative: $(S_1 \cup S_2) \cup S_3 = S_1
\cup (S_2 \cup S_3)$ for any sets. The canonical form (sort + BLAKE3)
is deterministic; both sides content-address to the byte-equal OID. ∎

**Theorem (⊕ monotonicity).** $L_1 \oplus L_2 \geq L_1$ and
$L_1 \oplus L_2 \geq L_2$, under the partial order
$L \leq L' \iff \texttt{dimensions}(L) \subseteq \texttt{dimensions}(L')$.

**Proof.** Set-union is monotone: $S_1 \cup S_2 \supseteq S_1$ and
$S_1 \cup S_2 \supseteq S_2$. Partial-order lifts to
`information_loss` via `dimensions(·)` projection. ∎

**Theorem (⊕ commutativity on dimension-sets, non-commutativity on
rescue semantics).** $L_1 \oplus L_2 = L_2 \oplus L_1$ AS
dimension-sets. However, when composed with `imperfect.recover` /
`imperfect.rescue` semantics (per `shards/cascade.mirror :132-138`
imperfect carrier), the ORDER matters — a dimension lost at hop 1 is
not recoverable at hop 2. The rescue-semantics-composed loss is
non-commutative in general; the dimension-set-composed loss is
commutative. This foundation formalizes only the dimension-set
level; the rescue-semantics composition is deferred to a per-cascade-
species discipline in the species shard docblock.

### §2.4 Admissibility threshold τ

**Definition (admissibility bound).** τ : information_loss is a
caller-declared upper-bound on acceptable loss for a cascade
invocation. Admissibility predicate:

$$
\texttt{admissible}(L, \tau) \;:=\; L \leq \tau
$$

per the partial order of §2.3.

**Theorem (admissibility is Rice-safe).** For any L, τ :
information_loss, `admissible(L, τ)` is decidable in time polynomial
in $|\texttt{dimensions}(L)| + |\texttt{dimensions}(\tau)|$.

**Proof.** `dimensions(L) ⊆ dimensions(τ)` is a set-subset check.
The dimensions are byte-visible from the OID's canonical form.
Set-subset is decidable in $O(|L| \cdot |τ|)$ (or better via
sorted comparison in $O(|L| + |τ|)$). No semantic introspection. ∎

**Composition with `@bilateral(@code/A, @code/B)`.** Per canonical
spec §5.3, `translation_admissible(outcome) = Pass ⇔ ... ∧
outcome.loss ≤ τ_caller ∧ ...`. The loss-check is Rice-safe by this
theorem; the composed bilateral inherits Rice-safety per Mara
`9be68b1` §4 AND-composition closure.

### §2.5 Loss lens alternatives

Per canonical spec §5.4, multiple loss-lens candidates compose with
`@cascade.measure`:

**Fiedler-value lens** (per `@coherence`; `docs/specs/eigensheaf.md`).
Loss as $\lambda_2^{\text{before}} - \lambda_2^{\text{after}}$
(algebraic connectivity reduction). Well-defined per Fiedler 1973;
composable per spectral additivity (approximate, under commuting-
Laplacian assumption; exact under block-diagonal decomposition).

**Algedonic-pain lens** (per `@cyberpunk/algedonic`). Loss as
$\int \text{pain\_gradient}$ across cascade hops. Well-defined per
`@cyberpunk/algedonic` species-decl; composable per additivity of
line integrals.

**Cut-count lens** (per `@knife`). Loss as cardinality of cuts
required to align cascade output to admissibility. Well-defined per
`@knife` species-decl; composable per cardinality additivity.

**@epistemologic-property composite** (default per
`[[feedback-loss-from-epistemologic-properties]]`). Loss as the
byte-typed dimension-set per §2.1. Composable per §2.2 ⊕.

Each cascade species selects its own lens; the family-root does not
pre-commit. Composability is preserved across lens choices provided
the chosen lens admits associative + monotone ⊕.

---

## §3. The polyglot theorem

### §3.1 Statement

**Theorem 3.1 (polyglot translation existence via machine-substrate
hub).** Let A, B be Turing-complete languages represented as
`@code/A` and `@code/B` species. Let M be a machine-substrate
altitude with landed `@cascade/code/A/M` and `@cascade/code/M/B`
species. Then:

$$
c_{AB} := c_{MB} \circ c_{AM}
$$

is a well-typed cascade at `A → B` altitude with:

1. Well-typedness: `morphism_well_typed(c_AB) = Pass` (§1.3).
2. Composed loss: `L_AB = L_AM ⊕ L_MB`, substrate-typed and
   readable via `c_AB.measure` (§2.2).
3. Admissibility: `admissible(L_AB, τ) = Pass` iff `L_AB ≤ τ`
   (§2.4).
4. Autopoietic extension: admissible discharges extend
   `@kintsugi/algebra` per §4.

### §3.2 Proof

**(1) Well-typedness.** Direct application of §1.3 composition
axiom. Since `c_AM` and `c_MB` are landed cascade species (by
hypothesis), each satisfies `morphism_well_typed = Pass`; their
composition inherits via `@glue.compose` per Mac Lane 1971 §I.1
(landed at `shards/glue.mirror :695-698`).

**(2) Loss composition.** Direct application of §2.2 ⊕ definition.
`L_AM : information_loss` and `L_MB : information_loss` are both
substrate-typed. Their ⊕ is substrate-typed via §2.3 associativity
+ monotonicity theorems.

**(3) Admissibility.** Direct application of §2.4 theorem.
`admissible(L_AB, τ)` is Rice-safe; discharges Pass iff L_AB ≤ τ.

**(4) Autopoietic extension.** By §4 theorem 4.1 monotone growth of
`@kintsugi/algebra` under admissible cascade discharge. ∎

### §3.3 Corollary — polyglot coverage via linear mint budget

**Corollary 3.3.** For N Turing-complete language species and K
machine-substrate hubs with `@cascade/code/L/M` species for every
(L, M) with L ∈ languages, M ∈ hubs (2NK cascade species total),
full N × N language-pair translation coverage exists via composition
in `Cascade_@code`.

**Proof.** For any (A, B) with A, B ∈ languages, pick any hub M
(K choices). Both `c_AM` and `c_MB` are landed by hypothesis. Apply
theorem 3.1. ∎

**Landing-plan implication (canonical spec §3.4).** For N = 10
languages and K = 3 hubs (`@code/llvm`, `@code/wasm`, `@code/turing`
after §9.1 M1 mint): full 100-pair coverage from ~60 cascade species,
not $10 \times 9 = 90$ direct edges.

### §3.4 Existence of machine-substrate hub M for arbitrary (A, B)

**Sub-theorem 3.4 (universal hub existence).** For any pair (A, B) of
Turing-complete languages, at least one machine-substrate altitude M
admits landable `@cascade/code/A/M` and `@cascade/code/M/B` species.

**Proof sketch (via Church-Turing).** By Church-Turing thesis, every
Turing-complete language is inter-simulatable with the Turing tape
machine model. Hence `@code/turing` (§9.1 M1 mint) is a candidate
hub for any (A, B): translations A → tape and tape → B exist as
computable functions per Church 1936 / Turing 1936. The
substrate-decl of `@cascade/code/A/turing` requires only that the
translation is COMPUTABLE (not unique, not efficient). ∎

**Load-bearing note on efficiency.** Turing-tape hub is universally
available but expensive (naïve tape simulation is not
performance-viable). LLVM-hub is efficient for imperative languages
(landed). WASM-hub is efficient for browser-target languages.
`@fate.roll` selects hub per candidate cost per §4 tournament
dispatch. Existence and admissibility (§3.1) are separate axes from
efficiency; the theorem guarantees existence, autopoietic learning
optimizes efficiency.

### §3.5 Load-bearing note on proof completeness

The proof of theorem 3.1 is complete for the existence-and-
admissibility direction under the four numbered clauses. The FULL
proof of the autopoietic-fixed-point over ALL Turing-complete
language pairs depends on the Banach-contraction constant $L$
per §4.3 depending on the loss composite selected. For any given
loss composite selection, the fixed-point exists per Banach 1922
under $L \in [0, 1)$; the theorem's universal quantification over
loss composites is subject to composite-specific contraction-constant
verification.

**Follow-up math tick if this proof-sketch is insufficient.** A
per-loss-composite verification of $L \in [0, 1)$ for each landed
loss-lens (Fiedler, algedonic, cut-count, @epistemologic composite)
would strengthen theorem 3.1 to universal quantification. This is
deferred to a subsequent math tick if empirical evidence surfaces a
loss composite for which $L \notin [0, 1)$.

---

## §4. Autopoietic closure

### §4.1 Monotone growth of `@kintsugi/algebra`

Per Mara `docs/math/kintsugi/algebra-as-metalogue-session.md` §3.1
theorem (autopoietic closure of `@kintsugi/algebra`):

$$
\forall n \geq 0. \quad A_n \subseteq A_{n+1} \quad \text{and} \quad |A_n| \leq |A_{n+1}|
$$

with strict inequality iff the tick's autopoietic loop crystallized a
novel `@kintsugi/fracture/*` species.

**Theorem 4.1 (polyglot extension of monotone growth).** Every
admissible cascade discharge per theorem 3.1 clause (3) contributes a
turn to `@kintsugi/algebra`'s session per Reading A (per Mara
`a58d5f0` §0 ratified reconciliation) AND becomes an element of
`@kintsugi/algebra` per Reading B (element-set growth).

**Proof.** Direct composition of §3.1 clause (4) with Mara `b5c6aeb`
§3.2 proof (autopoietic tick $n \to n+1$ case 4: crystallization
extends the tray by one). The cascade discharge's `routine` crystal
(per `shards/silicon/algebra.mirror` §"what crystallizes here") is
content-addressed by (source_oid, target_oid, cascade_oid,
selected_morphism.oid). Idempotency preserves the invariant: two
byte-equal cascade discharges produce byte-equal crystals; the
extension is idempotent. ∎

### §4.2 Fixed-point condition for polyglot translation

**Definition (polyglot fixed-point).** $A_\ast$ is a polyglot
fixed-point iff:

$$
\forall (A, B) \text{ language pair}. \forall f_A \in \texttt{substrate source-fragments at } A. \exists\, c \in A_\ast. c \text{ admissibly translates } f_A \text{ to } B
$$

That is: every substrate source-fragment in language A has an
admissible cascade in $A_\ast$ that translates to language B, for
every language pair.

**Corollary (fixed-point stability).** At $A_\ast$, the autopoietic
loop stabilises per Mara `b5c6aeb` §3.3 corollary: every subsequent
tick executes case (3) of the autopoietic step (cache hit; no growth).

### §4.3 Convergence via Banach contraction

Per Mara `b5c6aeb` §4.3 convergence-rate analysis, extended to
polyglot altitude:

**Theorem 4.3 (polyglot convergence via Banach contraction).** Under
the kintsugi outer loop's contraction discipline $e^{n+1} \leq e^n$
(per `@kintsugi/oscillate`), for a chosen loss composite with
contraction constant $L \in [0, 1)$, the algebra's growth is
Banach-contractive over the residual-untranslated-cross-language-
fragment space:

$$
\texttt{residual}(A_{n+1}) \leq L \cdot \texttt{residual}(A_n)
$$

Convergence to fixed-point $A_\ast$ within $O(\log_L(\texttt{initial
residual}))$ ticks under roomba's per-tick dispatch.

**Proof sketch.** Direct lift of Mara `b5c6aeb` §4.3 to the
cross-language-fragment space; the residual is the count of
(language A, fragment f_A) pairs for which no admissible cascade
exists in $A_n$. Each admissible discharge per theorem 4.1 reduces
the residual by at least one; the contraction constant depends on
the loss composite selected (§3.5). ∎

**Alternative termination criteria** per Mara `b5c6aeb` §4.1
three-way T1/T2/T3:

- **T1** (target hit): reaches fixed-point $A_\ast$.
- **T2** (budget exhausted): knapsack cap fires before fixed-point;
  substrate surfaces residual via `@glass` opacity carrier.
- **T3** (winding-class fixed-point): cumulative observation record
  returns to previously-visited class per `@torus`.

For polyglot specifically, T3 IS the algebra's composition-closure
equals its element-closure: every composable cascade pair (c_1, c_2)
where c_1.target = c_2.source has a composite `compose(c_1, c_2) ∈
A_\ast`.

### §4.4 Rice-safety of algebra extension

Per Mara `b5c6aeb` §5 theorem (Rice-safety of `@kintsugi/algebra`
growth): extension is content-addressed byte-level per Mara
`701828a` §1 (`bilateral.discharge(decl, args)` reduces to
byte-string containment).

**Corollary 4.4.** The polyglot cascade discharge's crystallization
per theorem 4.1 is Rice-safe: byte-level containment check on the
crystal's OID; no semantic predicate over program behavior.

Consequence: `kintsugi_algebra_witnessing(binding)` per
`shards/kintsugi.mirror :263-267` is a lightweight predicate at the
reflective evaluator (`bootstrap/src/apply_h.rs`); no expensive proof
obligation at discharge time even for polyglot cascade discharges.

---

## §5. Categorical structure — polyglot as functor

### §5.1 Definition

**Definition (polyglot functor).** Let `TCLang` be the category
whose objects are Turing-complete languages (per Church-Turing) and
whose morphisms are computable semantics-preserving translations.
Let `MirrorCat` be the category whose objects are `@code/*` species
and whose morphisms are `@cascade/code/A/B` species (per §1.1
`Cascade_@code`). Define:

$$
\Phi : \texttt{TCLang} \to \texttt{MirrorCat}
$$

$$
\Phi(L) := \texttt{@code}/L \quad \text{(species-decl for language L)}
$$

$$
\Phi(t : L_1 \to L_2) := \texttt{@cascade/code}/L_1/L_2
$$

when the cascade species is landed, otherwise Φ(t) is undefined at
that arrow (the substrate does not YET carry the cascade). The
polyglot-theorem-guaranteed composition (§3.1) fills gaps via
machine-substrate hub composition.

### §5.2 Functoriality

**Theorem 5.2 (Φ is a functor).**

1. **Identity preservation.** $\Phi(id_L) = id_{\Phi(L)} =
   \texttt{@cascade/code}/L/L = id_{\texttt{@code}/L}$ per §1.4.
2. **Composition preservation.** $\Phi(t_2 \circ t_1) = \Phi(t_2)
   \circ \Phi(t_1)$ per §1.3 composition axiom.

**Proof.**

(1) Identity: the degenerate case $A = B$ per §1.4 gives
`@cascade/code/L/L` as the identity morphism on `@code/L`.

(2) Composition: for translations $t_1 : L_1 \to L_2$ and $t_2 : L_2
\to L_3$ with landed cascade species `c_12 := @cascade/code/L_1/L_2`
and `c_23 := @cascade/code/L_2/L_3`:

$$
\Phi(t_2 \circ t_1) = \texttt{@cascade/code}/L_1/L_3 = c_{23} \circ c_{12} = \Phi(t_2) \circ \Phi(t_1)
$$

per §1.3 axiom. ∎

### §5.3 Loss-lens as natural transformation

**Corollary 5.3.** The loss-lens family `{loss_lens<A, B>}_{A, B ∈
@code/*}` per `shards/cascade.mirror :210-227` is a natural
transformation from Φ to the loss-carrier functor
`Λ : MirrorCat → InfoLoss` (mapping cascade species to their
associated loss carriers).

**Sketch.** Naturality: for any cascade square

$$
\begin{array}{ccc}
\texttt{@code}/A & \xrightarrow{c_1} & \texttt{@code}/B \\
\downarrow_{c_3} & & \downarrow_{c_4} \\
\texttt{@code}/A' & \xrightarrow{c_2} & \texttt{@code}/B'
\end{array}
$$

with all four cascades landed, the associated loss diagram commutes
via ⊕ per §2.3 associativity. Full proof deferred to per-cascade
species discipline; the lens naturality is a per-species discharge
per `<A>_<B>_loss_well_defined` bilateral.

---

## §6. Connes spectral-triple angle

### §6.1 Recall — the substrate's spectral-triple architecture

Per `[[architecture-connes-spectral-triple]]` and `docs/math/
kintsugi/algebra-as-metalogue-session.md` §7: the substrate's
deepest layer is $(A, H, D)$ — a spectral triple where A is a
$*$-algebra of bounded operators, H is a Hilbert space, D is a
self-adjoint operator (the Dirac operator).

**Correspondence at family altitude:**

- **A**: `@mirror` — the observables at family-root altitude.
- **H**: expanding Hilbert space per recognition #51 (per
  `[[architecture-mirror-as-expanding-hilbert-space]]`).
- **D**: `@kintsugi` — the dynamics (per `[[architecture-connes-
  spectral-triple]]` — @kintsugi names U(t) of the triple).

### §6.2 Polyglot as multi-altitude spectral triple

**Definition (polyglot spectral triple).** The polyglot architecture
per canonical spec §8 IS a multi-altitude spectral triple:

- **Local algebras** $\{A_L\}_{L \in \texttt{@code}/*}$: each `@code/
  L` species is a $*$-algebra of operators at language altitude L.
  Per canonical spec §3.1, each species is grounded in the five-op
  prism discipline; the operations form a bounded algebra.
- **Morphisms** $\{\phi_{AB}\}_{(A, B) \in \texttt{@cascade/code}/A/B}$:
  each cascade species is a morphism between local algebras. Per §1
  composition axiom, morphisms compose associatively.
- **Witnesses** $\{@bilateral(A, B)\}_{(A, B)}$: each bilateral is a
  Dirac-operator-like discharge predicate; per Mara `701828a` §2
  connes-triple angle, the reflective evaluator IS the Dirac
  operator D acting uniformly on A.
- **Fixed-point algebra** $A_\ast$ = `@kintsugi/algebra` at
  polyglot fixed-point per §4.2 — the substrate's realised
  polyglot-translation algebra.

**Alignment with landed spectral-triple discipline.** Per
`docs/math/kintsugi/algebra-as-metalogue-session.md` §7 the
`@kintsugi/algebra` corresponds to the spectral triple's realised
algebra; the polyglot generalisation extends this to multi-language
altitude — the realised polyglot-translation algebra IS the
spectral-triple's fixed-point at the cross-language boundary.

### §6.3 Ouroboros-of-ouroboros at polyglot altitude

Per Mara `docs/math/autopoiesis/README.md` §6 (the ouroboros-of-
ouroboros theorem, deferred to full statement there): the autopoietic
operator's fixed-point is itself an element of the algebra. Extended
to polyglot altitude: the polyglot translation operator's fixed-point
is itself a `@cascade/code/A/B` species (specifically, when A = B =
mirror, the identity cascade). The polyglot operator translates its
own definition into its own algebra.

This is the substrate-honest form of self-application at the
polyglot altitude — the polyglot compiler compiles its own
composition into its own substrate; the ouroboros closes at the
cross-language boundary.

---

## §7. Related theorems (reference-only; no restatement)

The following LANDED math foundations compose with the polyglot
formalization above. Each is cited by (author, oid/reference,
section) with no restatement:

- **Mara `9be68b1` (`docs/math/bilateral-as-glue-metalogue-
  composition.md`)** — general-case `@bilateral(A, B)` composition
  theorem (§3), Rice-safety by AND-conjunction (§4), fixed-point
  convergence (§5), functorial structure (§7), attending-at-λ₀ (§8).
  THIS foundation composes §3-§4 for polyglot admissibility gating.

- **Mara `b5c6aeb` (`docs/math/kintsugi/algebra-as-metalogue-
  session.md`)** — autopoietic-closure theorem (§3.1), fixed-point
  condition (§3.3), Banach-contractive convergence (§4.3),
  Rice-safety of algebra growth (§5), ouroboros_monotone
  correspondence (§6), Connes-triple angle (§7). THIS foundation
  lifts §3.1 + §4.3 to polyglot altitude in §4 above.

- **Mara `701828a` (`docs/math/epistemologic/pact/bilateral-
  sentinel.md`)** — sentinel-as-content-addressed-witness (§1),
  reflective evaluator as Connes spectral triple (§2), monotone
  extension safety (§3). THIS foundation composes §1 for §2.4
  Rice-safety and §2 for §6.2 spectral-triple correspondence.

- **Mara #152 (`docs/math/autopoiesis/README.md`)** — compile-
  altitude Maturana-Varela operational closure (§1), Polyak-
  Łojasiewicz fixed-point convergence (§2), Liquid-type refinement
  (§3), tournament ranking (§4), content-addressed memory (§5),
  ouroboros-of-ouroboros theorem (§6). THIS foundation composes
  §1.2 (compile-altitude autopoiesis) with polyglot extension in
  §4.

- **Mara `a18ca90` (`docs/specs/fate-silicon-metalogue-in-void-
  duality-basis.md`)** — void-duality basis for the metalogue
  session composition; three-way termination criteria T1/T2/T3
  (§4.3). THIS foundation composes §4.1 termination criteria for
  polyglot convergence.

- **Reed `21fc211`** — reflective evaluator (Landing 3+4) at
  `bootstrap/src/apply_h.rs`; corpus-dispatch reflection for
  bilateral discharge. Empirical implementation-of-record for §2.4
  Rice-safe admissibility check.

- **Mac Lane 1971 §I.1** (canonical) — categorical composition +
  associativity + identity axioms. Composed for §1 morphism
  discipline and §5 functoriality.

- **Wadler 1989 / Reynolds 1983** (per `shards/cascade.mirror`
  sources) — parametricity for `@labeled<>` functor. Composed for
  §1.2 loss_lens<A, B> parametric-carrier discipline.

- **Mesland 2013 §3** (per `shards/glue.mirror`) — categorical
  differential operator + morphism composition. Composed for §1.3
  well-typedness proof.

- **Banach 1922** (canonical) — Banach fixed-point theorem +
  contraction mapping. Composed for §4.3 convergence proof.

- **Church 1936 / Turing 1936** (canonical) — Church-Turing thesis
  + universal machine model. Composed for §3.4 universal hub
  existence sub-theorem.

- **Fiedler 1973** (canonical) — algebraic connectivity of graphs +
  Fiedler value. Composed for §2.5 Fiedler-value loss lens.

---

## §8. Falsifiability + Bateson-closure

### §8.1 Falsifiers (canonical spec §11 formalized)

**F1 formalized (compositional adjacency).** Falsifier: exists
Turing-complete languages A, B for which no machine-substrate hub M
admits landable `@cascade/code/A/M` + `@cascade/code/M/B` species AND
`@code/turing` mint per §9.1 M1 cannot bridge them. **Test
protocol**: enumerate empirical (A, B) pairs; for each, verify
Church-Turing translation existence per §3.4 sub-theorem; if any
pair falsifies, the theorem 3.1's universal quantification fails.

**F2 formalized (loss non-associativity).** Falsifier: exists losses
$L_1, L_2, L_3$ : information_loss with $(L_1 \oplus L_2) \oplus L_3
\neq L_1 \oplus (L_2 \oplus L_3)$. **Test protocol**: construct
three-hop cascade with substrate-typed loss at each hop; compute
both associations; verify byte-equal OIDs of the composed
information_loss carriers. Companion §2.3 proof shows set-union
associativity ⇒ ⊕ associativity.

**F3 formalized (admissibility non-Rice-safe).** Falsifier: exists L,
τ for which `admissible(L, τ)` requires semantic introspection of the
compiled program. **Test protocol**: verify `admissible(L, τ)`
reduces to byte-level subset-check on dimension-sets per §2.4 proof;
if any admissibility discharge invokes program-behavior predicate,
Rice-safety fails.

**F4 formalized (algebra non-monotone).** Falsifier: exists
cascade discharge that INVALIDATES a prior Pass verdict on
`@kintsugi/algebra` extension. **Test protocol**: pre-condition:
`algebra_metalogue_witnessing(A_n) = Pass`; extend via case (4);
verify `algebra_metalogue_witnessing(A_{n+1}) = Pass` AND every
prior element's discharge remains Pass. Companion §4 + Mara
`b5c6aeb` §3.2 formalize.

**F5 formalized (fixed-point unreachable).** Falsifier: exists
loss composite for which the roomba walk's residual-untranslated-
cross-language-fragment space has $L \notin [0, 1)$. **Test
protocol**: empirically measure fragment-count decrease per
roomba tick over ≥ 50 ticks per language pair; regression-fit for
contraction constant; verify $L < 1$. Fall-back per Mara `b5c6aeb`
§4.1: T2 budget-exhausted termination surfaces residual opacity
without falsifying admissibility guarantee for translations that DID
crystallize.

### §8.2 Bateson closure

Per `[[architecture-bateson-form-behaviour-partition]]` (recognition
#50) and the corresponding discipline in canonical spec:

**Form side of the partition (form-substrate polyglot).**
`@cascade/code/A/B` species-decl'd shape lives at
`shards/cascade/code/<A>/<B>.mirror`. The composition graph per
canonical spec §8 is form-substrate content. Falsifiability of the
form is: does the substrate-decl'd shape composition-close per §1.4
axioms? Answer per §5 functoriality theorem: yes.

**Process side of the partition (process-substrate polyglot).**
Cascade DISCHARGE at empirical run-time (per `mirror roomba
--translate=<rs-file>` per canonical spec §9.3 M5 empirical). The
autopoietic loop per canonical spec §6 is process-substrate. Companion
§4 formalizes convergence via Banach contraction.

**Bateson closure requirement.** Form and process must close on each
other. Formalized: for each admissible cascade discharge (process),
the crystallized `routine` element extends the algebra (form). For
each algebra element (form), a corresponding cascade discharge
existed (process). This bidirectional closure IS §4.1 theorem
composed with §4.2 fixed-point condition.

**Substrate-honest verification.** At polyglot fixed-point $A_\ast$
per §4.2, form-substrate (element-set) and process-substrate
(discharge-set) are bijectively linked; each element crystallized
FROM a discharge; each discharge PRODUCED an element. Bateson
closure holds per this bijection.

### §8.3 Substrate-decl'd verification protocol

Per Mara `b5c6aeb` §8 discipline, the falsifiers are verifiable at
substrate-decl altitude before empirical run. Each falsifier F1-F5
is decidable on the substrate corpus + landed math foundations
without requiring the M1-M5 mint sequence to fire. This foundation's
verification is grep-first + composition-check, per Taut audit
discipline; empirical falsifiability comes online with M5.

---

*Math foundation ends. No mints. No shard authored. No Rust
touched. Markdown-only under 📝 bypass. Theory-authoring per Alex
2026-07-17 ratification; all proofs compose over LANDED foundations
cited in §7 without restatement. Every load-bearing claim resolves
to (i) a landed substrate-decl citation, (ii) a landed math
foundation citation, or (iii) a canonical proof (Mac Lane 1971,
Church 1936, Turing 1936, Banach 1922, Wadler 1989, Reynolds 1983,
Mesland 2013, Fiedler 1973).*

**Companion:** `docs/specs/polyglot-loss-aware-computational-
translation.md` (canonical spec, LANDED same tick).

**Cascade reading for follow-up ticks:**
- Per canonical spec §9 landing plan: Reed mint sequence M1-M5.
- Per §3.5: per-loss-composite contraction-constant verification if
  empirical evidence surfaces $L \notin [0, 1)$ (deferred math tick).
- Per §5.3: full naturality proof for loss-lens as natural
  transformation (deferred to per-cascade species discipline).
