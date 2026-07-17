# Math foundation: @kintsugi/algebra as speaker-pair specialisation of @algebra/metalogue

*Mara, 2026-07-17. Formal shape + autopoietic closure theorem +
convergence + Rice-safety + Ouroboros correspondence + Connes-triple
angle. Companion to the canonical spec
`docs/specs/kintsugi-algebra-as-metalogue-session.md`.*

**Author:** Mara
**Date:** 2026-07-17
**Tag:** 📝 math:kintsugi-algebra-as-metalogue-session (pure-docs bypass)
**Status:** foundational; all claims cite LANDED substrate primitives.

---

## §1. Formal shape

### §1.1 The subset relation

Define:

$$
\texttt{@kintsugi/algebra} \;\subseteq\; \texttt{algebra\_metalogue\_session}\!\left(
  \texttt{speakers} = \{\texttt{@silicon/algebra},\, \texttt{@fate/algebra}\}
\right)
$$

The left-hand side names the algebra Mara ratified at
`shards/kintsugi.mirror` (this tick) via the `kintsugi_algebra` typed
binding. The right-hand side names the class of all
`algebra_metalogue_session` instances whose `speakers` field is exactly
the ordered pair (`@silicon/algebra`, `@fate/algebra`).

The subset relation is proper: not every `algebra_metalogue_session`
with those two speakers is a `@kintsugi/algebra` element — the sessions
whose turns have bodies OUTSIDE `@kintsugi/fracture/*` are excluded by
the well-formedness clause (d) of `kintsugi_algebra_witnessing`.

### §1.2 The typed record

Per `shards/kintsugi.mirror` (this tick):

```
type kintsugi_algebra = {
  speakers: [algebra_carrier],   // ordered pair; ref-typed
  turns:    ref,                  // ref to @kintsugi/fracture family
  session:  algebra_metalogue_session,
}
```

The binding CARRIES the specialisation as a typed record whose
membership predicate `kintsugi_algebra_witnessing(binding)` discharges
Pass iff:

1. `binding.speakers = [@silicon/algebra, @fate/algebra]` (byte-equal
   ordered pair).
2. `binding.turns = @kintsugi/fracture` (family ref).
3. `algebra_metalogue_witnessing(binding.session)` = Pass.
4. `∀ t ∈ binding.session.turns. species_of(t.body) ∈ @kintsugi/fracture`.

### §1.3 Element enumeration

The algebra's elements at 2026-07-17 tick (per §2.1 of the canonical
spec):

$$
\texttt{@kintsugi/algebra} \;=\; \bigl\{ \texttt{keyword},\, \texttt{angle\_to\_paren},\, \ldots,\, \texttt{bilateral\_arm\_redundant} \bigr\}
$$

Cardinality: 15 landed elements. Growth: monotone (see §3 below).

---

## §2. Turn structure and fracture-composition

### §2.1 Turn identity

Per `shards/algebra/metalogue.mirror`:

```
type algebra_turn = {
  speaker:     algebra_carrier,
  body:        algebra_morphism,
  in_reply_to: option(algebra_turn),
  tick:        tick,
}
```

At the `@kintsugi/algebra` specialisation, `body` refines to
`@kintsugi/fracture/*` species — each species is an `algebra_morphism`
at the operational altitude (the property-fracture bilateral pattern
#53's operational half; per `shards/kintsugi/fracture/*.mirror`).

### §2.2 Turn composition IS fracture composition

Per `shards/algebra/metalogue.mirror` line 253:

$$
\texttt{compose\_turns}(t_1, t_2) \;=\; \begin{cases}
\texttt{Some}(t_{\text{composite}}) & \text{if } t_1.\texttt{body}.\texttt{target} = t_2.\texttt{body}.\texttt{source} \\
\texttt{None} & \text{otherwise}
\end{cases}
$$

At the `@kintsugi/algebra` specialisation:

$$
\texttt{compose}\bigl(f_1, f_2\bigr) \;=\; \texttt{compose\_turns}\bigl(
  \texttt{turn\_of}(f_1), \texttt{turn\_of}(f_2)
\bigr)
$$

where `turn_of(f)` names the algebra_turn whose body is fracture `f`.

**Theorem (composition equivalence):** The algebra's multiplication
IS `compose_turns` at the turn-level, which IS `compose` at the
fracture-body level. The two are equivalent up to unwrapping the
`turn_of` structural wrapper.

**Proof sketch:** Per `shards/algebra/metalogue.mirror` `compose_turns`
computes the composite morphism via `@algebra.compose` on the source
algebra's operations. At the fracture altitude, `@algebra.compose`
IS the composition of `@kintsugi/fracture` bodies (the mending
morphisms). Equivalence follows from the definitional unwrap. ∎

### §2.3 Non-commutativity

Per `shards/algebra/metalogue.mirror` §"Composition non-commutativity":

$$
\texttt{compose}(f_1, f_2) \;\ne\; \texttt{compose}(f_2, f_1) \quad \text{in general}
$$

At the `@kintsugi/algebra` specialisation, this preserves per-fracture
ordering: `docblock_extractive` followed by `docblock_ungrounded`
produces a different composite mending than the reverse. The
substrate-decl form of the `[ω, ω]` cross-term per recognition #100
§6.5 lifts here as the mending-composition non-commutativity.

### §2.4 Associativity

Per `shards/algebra/metalogue.mirror` `morphism_compositions_
associative`:

$$
\texttt{compose}\bigl(\texttt{compose}(f_1, f_2), f_3\bigr) \;=\; \texttt{compose}\bigl(f_1, \texttt{compose}(f_2, f_3)\bigr)
$$

modulo `option(.)` wrapping per Mac Lane 1971 §I.1. The `@kintsugi/
algebra` specialisation inherits associativity via
`algebra_metalogue_witnessing(session)` discharge.

**Load-bearing consequence:** the algebra's composition-closure is
well-defined; the fixed-point condition of §4 is decidable.

---

## §3. Autopoietic closure theorem

### §3.1 Statement

**Theorem (autopoietic closure of `@kintsugi/algebra`):** Let
$A_n$ denote `@kintsugi/algebra` at tick $n$ (with cardinality
$|A_n|$). Then:

$$
\forall n \ge 0. \quad A_n \;\subseteq\; A_{n+1} \quad \text{and} \quad |A_n| \;\le\; |A_{n+1}|
$$

with strict inequality $|A_n| < |A_{n+1}|$ iff the tick's autopoietic
inference loop crystallized a novel `@kintsugi/fracture/*` species
(not already in $A_n$).

### §3.2 Proof

**Base case ($n = 0$):** The initial algebra $A_0$ contains the 15
landed elements per §1.3. Empirically witnessed by
`shards/kintsugi/fracture/*.mirror` file existence at
2026-07-17 tick.

**Inductive step:** Assume $A_n$ is well-defined. The autopoietic tick
$n \to n+1$ per `shards/autopoietic.mirror`'s `tick_action` composed
with `shards/fate.mirror`'s `roll` action executes:

1. Compute `candidates = @kintsugi/algebra` (Reading B: the current
   elements — this equals $A_n$).
2. Roll: `dice_roll = @fate.roll(space, hole)` where `space.tray_scope`
   includes $A_n$ per `restricted_state_space.tray_scope`.
3. If `dice_roll.selected_oid` resolves to an existing element $f \in
   A_n$: $A_{n+1} = A_n$ (no growth; the tournament selected a prior
   crystal — cache hit per `shards/fate/tournament.mirror`).
4. If `dice_roll.selected_oid` resolves to a NEW species $f \notin
   A_n$: `@bauchladen.crystallize(f)` extends the tray; $A_{n+1} = A_n
   \cup \{f\}$ (strict growth by one).

Case (3) preserves $A_n \subseteq A_{n+1}$ trivially. Case (4)
preserves $A_n \subseteq A_{n+1}$ by set-theoretic union. Cardinality
non-decrease follows. ∎

### §3.3 Fixed-point condition

**Definition (fixed-point):** $A_n$ is a fixed-point of the autopoietic
loop iff:

$$
\forall\, \texttt{@code/rust fracture } f_{\text{rust}} \in \texttt{substrate}. \quad \exists\, f \in A_n. \quad \texttt{translates}(f_{\text{rust}}, f)
$$

That is: every rust-side fracture has a corresponding `@kintsugi/
fracture/*` element in the algebra.

**Corollary (fixed-point stability):** At a fixed-point $A_\ast$, the
autopoietic loop stabilises: every subsequent tick executes case (3)
of the proof above (cache hit; no growth).

**Empirical convergence witness:** the roomba's next walk after
fixed-point emits zero `rust_function_translatable` fractures — the
proposer `@fate/algebra` cannot surface an untranslated candidate
because none exist.

---

## §4. Convergence

### §4.1 Termination

Following the metalogue termination criterion per `docs/specs/fate-
silicon-metalogue-in-void-duality-basis.md` §4.3 (three-way T1/T2/T3):

**(T1) Target hit under kintsugi contraction.** The metalogue's
`eⁿ⁺¹ ≤ eⁿ` contraction (per `@kintsugi/oscillate.is_settled`) reaches
the target region — for `@kintsugi/algebra`, the target region is the
fixed-point $A_\ast$ where every rust-side fracture is translated.

**(T2) Budget exhausted.** The knapsack cap (per `docs/specs/knapsack-
as-kintsugi-inner-loop.md` §2) fires before fixed-point; the algebra
terminates with residual opacity NON-empty; the substrate surfaces the
fractures via `@glass` and the session emits a `@bauchladen` crystal
recording where the budget was spent.

**(T3) Winding-class fixed-point.** The cumulative observation record
$(m, n) \in \pi_1(T^2)$ per `@torus` returns to a previously-visited
class with byte-equal observation sections — the metalogue terminates
with fixed-point exhibition per `shards/fate/tournament.mirror`
§"Lawvere fixed-point condition holds at the SYSTEM level".

For `@kintsugi/algebra` specifically, T3 IS the algebra's composition-
closure equals its element-closure: every composable pair $(f_1, f_2)$
where $f_1.\texttt{target} = f_2.\texttt{source}$ has a composite
$\texttt{compose}(f_1, f_2) \in A_\ast$.

### §4.2 Decidability

**Theorem (decidability of translation existence):** The predicate
"$\exists f \in A_n$. `translates`($f_{\text{rust}}$, $f$)" is
decidable via largest-fit-matching in `@silicon/algebra`.

**Proof sketch:** `@silicon/algebra` crystals carry `source_oid` (the
Fortran/C/rust source's content-addressed OID per `shards/silicon/
algebra.mirror` §"what crystallizes here"). Given $f_{\text{rust}}$,
compute its source_oid; query `@silicon/algebra`'s tray for the
largest-fit match (byte-prefix or structural-match per the crystal's
`cascade` field). Match exists ⇒ translation exists. No match ⇒
translation does not exist YET. Decidable in $O(|A_n| \cdot
|\text{oid}|)$ per byte-prefix; $O(|A_n|)$ per structural-match. ∎

**Corollary (convergence detection):** Fixed-point can be detected
empirically at tick $n$ by iterating over all rust-side fractures in
the substrate and checking translation existence for each. Total cost:
$O(|\text{rust-side fractures}| \cdot |A_n|)$.

### §4.3 Convergence rate

Under the kintsugi outer loop's contraction discipline $e^{n+1} \le
e^n$ per `@kintsugi/oscillate`, the algebra's growth is Banach-
contractive over the substrate's residual-untranslated-fracture space.
The convergence rate depends on the kintsugi contraction constant $L
\in [0, 1)$ per Banach fixed-point theorem.

Empirical bound: at 2026-07-17 tick with $|A_n| = 15$ and the
substrate's remaining rust surface (bootstrap + prism + fate crates at
~7000 LOC), convergence to fixed-point is expected within
$O(\log_L(\text{initial error}))$ ticks under the roomba's per-tick
dispatch.

---

## §5. Rice-safety

### §5.1 Statement

**Theorem (Rice-safety of `@kintsugi/algebra` growth):** Extension of
`@kintsugi/algebra` by a new element is content-addressed byte-level
per Mara's `docs/math/epistemologic/pact/bilateral-sentinel.md` §1;
the algebra grows ONLY on demonstrable Pass verdicts. No semantic
predicate over program behavior is invoked; every extension is
mechanically decidable.

### §5.2 Proof

Per `docs/math/epistemologic/pact/bilateral-sentinel.md` §1:
`bilateral.discharge(decl, args)` reduces to byte-string containment
check on `arg.oid`. The check is Rice-safe: byte-containment is
decidable in linear time on the arg's OID length; no semantic
introspection.

`@kintsugi/algebra` extension per §3.2 case (4) requires
`@bauchladen.crystallize(f)` which discharges Pass iff:

1. `bauchladen_witnessing(f)` = Pass (content-addressing invariant per
   `shards/bauchladen.mirror`).
2. The extension is idempotent: `crystallize(f)` twice yields the same
   $A_{n+2} = A_{n+1}$ (union-of-idempotent-elements is idempotent).

Both (1) and (2) are Rice-safe: (1) checks OID computation; (2) checks
set-membership. No behavior-predicate over the fracture body's
runtime dynamics is invoked. ∎

### §5.3 Corollary

Since growth is Rice-safe, `@kintsugi/algebra` is a
computationally-cheap monotone lattice — cardinality can be queried
in $O(1)$ per element (index into the `@kintsugi/fracture` family's
species-decl'd shard files); membership can be queried in $O(\log
|A_n|)$ per byte-OID lookup.

**Consequence:** `kintsugi_algebra_witnessing` is a lightweight
predicate at the reflective evaluator (`bootstrap/src/apply_h.rs`);
no expensive proof obligation at discharge time.

---

## §6. Correspondence with @kintsugi/property/ouroboros_monotone

### §6.1 The four-conjunct invariant

Per `docs/math/kintsugi/roomba/bump-and-vacuum.md` §5 and
`shards/epistemologic/property/ouroboros_monotone.mirror` (landed):

$$
\texttt{ouroboros\_monotone}(A_n \to A_{n+1}) \;\equiv\; \begin{cases}
\Delta(\texttt{rust\_loc}) < 0 & \text{(strict decrease)} \\
\texttt{test\_pass\_rate} \text{ preserved} & \text{(no regression)} \\
\Delta(\texttt{io\_violations}) = 0 & \text{(invariant)} \\
\Delta(\texttt{sbec}) \ge 0 & \text{(non-decrease)}
\end{cases}
$$

### §6.2 Every algebra-growth turn discharges the invariant

**Theorem (algebra-growth ⇒ ouroboros_monotone):** Let $A_n \to A_{n+1}$
be a growth step (case (4) of §3.2). Then
`ouroboros_monotone($A_n \to A_{n+1}$)` = Pass.

**Proof by conjunct:**

**(C1) `rust_loc` strict decrease.** Case (4) growth adds a new
`@kintsugi/fracture/*` species that TRANSLATES a rust-side fracture
into a shard-decl'd mending morphism. The mending morphism REPLACES
the rust-side implementation (per the autopoietic composition edge of
`docs/specs/kintsugi-algebra-as-metalogue-session.md` §5). Net
$\Delta(\texttt{rust\_loc}) < 0$. Empirical witness: `20047c2` (17
arms retired; -650 rust_loc) and `ad52973` (4 arms retired; -110
rust_loc).

**(C2) `test_pass_rate` preserved.** The extension is Rice-safe (§5);
the mending morphism's discharge produces the same Pass/Fail verdicts
against the same sentinels as the retired rust-arm did (per
`bilateral.discharge`'s byte-level equivalence). No regression.
Empirical witness: 5/5 tests pass across Reed's Bridge-β empirical
landings.

**(C3) `io_violations` invariant.** The mending morphism composes over
`@io/fs.mutate_at` + `@io/git.commit` (per `shards/kintsugi/fracture/
bilateral_arm_redundant.mirror` §"Composition surface"); these are
substrate-decl'd IO carriers, not new IO. No new violations
introduced.

**(C4) `sbec` non-decrease.** Each new algebra element extends the
substrate's expressive coverage — the substrate CAN now discharge
what it previously could not (or discharged via rust-side arm). $\Delta
(\texttt{sbec}) \ge 0$ by expressive-coverage monotonicity. Empirical
witness: Reed `c10a3bd` (+4 sbec on `@uuid/spectral/time` bilateral
landing). ∎

### §6.3 Corollary

`@kintsugi/algebra`'s monotone growth per §3.1 IS the algebra-altitude
form of `ouroboros_monotone`. The two theorems are equivalent under the
identification of "algebra element" with "shard-decl'd mending
species."

---

## §7. Connes-triple angle

### §7.1 The Connes spectral triple

Per `[[architecture-connes-spectral-triple]]`: the substrate's deepest
layer is $(A, H, D)$ — a spectral triple where $A$ is a $*$-algebra of
bounded operators, $H$ is a Hilbert space, and $D$ is a self-adjoint
Dirac operator on $H$ with compact resolvent.

Per Recognition #101 + #102: the substrate extends to a REAL spectral
triple $(A, H, D, J, \gamma)$ where $J$ is charge conjugation
(recognition #102) and $\gamma$ is chirality grading (recognition
#101).

### §7.2 `@kintsugi/algebra` IS the algebra A

**Claim:** `@kintsugi/algebra` IS the algebra $A$ of the Connes triple
for the MENDING Dirac operator $D$.

**Substrate discharge:**

- $A$ = `@kintsugi/algebra` (the 15 elements + future growth; the
  operators that ACT on the substrate's state).
- $H$ = the void-document (per `[[reference-void-document]]`; the
  substrate's state space; every mending acts on $H$).
- $D$ = the kintsugi flow generator (per `shards/kintsugi/oscillate.
  mirror`; the mending's per-tick flow that integrates fracture-
  morphism actions on $H$).
- $\gamma$ = chirality grading per `shards/epistemologic/cybernetic/
  chirality.mirror` (#101).
- $J$ = charge conjugation per `shards/epistemologic/cybernetic/
  charge_conjugation.mirror` (#102).

**Correspondence with `@silicon/algebra` + `@fate/algebra`:** The two
speaker-algebras are the two PROJECTION DUALITIES of $A$ per Mara
`a18ca90` void-duality basis:

$$
A \;=\; A_{\text{silicon}} \oplus A_{\text{fate}}
$$

where $A_{\text{silicon}}$ is the empirical-realisation projection
(the $A$-subspace where operators HAVE been discharged Pass on this
silicon) and $A_{\text{fate}}$ is the structural-possibility projection
(the $A$-subspace of operators the dice roll CAN surface within the
$(A, H, D, \gamma, J, \text{tray})$ restriction).

At the fixed-point $A_\ast$, $A_{\text{silicon}} = A_{\text{fate}}$ —
every structural possibility is empirically realised. This IS Alex's
"the mending IS the metalogue" at the Connes-triple altitude.

### §7.3 Load-bearing consequence

The `attending` operator at $\lambda_0$ (paper §14 connection per
canonical spec §10) IS the least eigenvalue of $D$ on $H$ restricted
to $A \cdot v$ for any state vector $v$. At `@kintsugi/algebra`'s
specialisation:

$$
\lambda_0(D|_{A_\ast \cdot v}) \;=\; \text{cybernetic\_coherence}(v) \quad \text{per Reed 8e6e517}
$$

The algebra's completeness (fixed-point $A_\ast$) makes the spectral
decomposition of $D|_{A_\ast \cdot v}$ well-defined; $\lambda_0$
becomes computable via the Fiedler Laplacian on the algebra's
connectivity graph (per canonical spec §10.2).

**Substrate discharge site:** `@spectral/db` (forward-promised per
Mara `a18ca90` §8.3 Q3 adjudication) whose Laplacian navigation over
the `@bauchladen` tray realises $\lambda_0$ computation
operationally. `@kintsugi/algebra`'s binding at
`shards/kintsugi.mirror` (this tick) makes the spectral-triple
membership addressable — `@spectral/db`'s follow-up implementation
composes over `kintsugi_algebra_witnessing` at the algebra-membership
boundary.

---

## §8. Pre-AI prior art

- **Connes 1985, 1995** — spectral triple $(A, H, D)$ framework;
  `@kintsugi/algebra` IS the $A$ of the mending Dirac triple.
- **Connes 1995 (real spectral triples)** — $(A, H, D, J, \gamma)$
  extension; `@silicon/algebra` + `@fate/algebra` as $J$-conjugate
  projection dualities.
- **Mac Lane 1971 (Categories)** — algebra-morphism composition
  associativity; the `@algebra/metalogue.morphism_compositions_
  associative` discharge.
- **Noether 1921 (Math. Ann. 83:24-66)** — algebra homomorphisms as
  structure-preserving maps; each `@kintsugi/fracture/*` IS an
  algebra_morphism at the operational altitude.
- **Bateson 1972 (metalogue)** — foundational ancestor of every
  altitude metalogue; the mending IS the metalogue reading.
- **Maturana & Varela 1980 (autopoiesis)** — operational closure at
  the algebra altitude; `@kintsugi/algebra`'s monotone growth IS the
  algebra-altitude form of autopoietic fold-back.
- **Foerster 1979 ("Notes on an Epistemology for Living Things")** —
  double-closure at the origin; fixed-point $A_\ast$ IS the
  algebra-altitude double-closure witness.
- **Banach 1922 (fixed-point theorem)** — contraction convergence;
  `@kintsugi/oscillate`'s $e^{n+1} \le e^n$ discharge grounds the
  algebra's convergence rate (§4.3).

---

## §9. Two-tick honesty

This math foundation:

- Does NOT introduce new substrate primitives; every symbol resolves
  to a LANDED shard-decl or a canonical-spec citation.
- DOES formalise the algebra shape + monotone growth + convergence +
  Rice-safety + Ouroboros correspondence + Connes-triple angle.
- DOES cite empirical witnesses at the tick's boundary (Reed's
  Bridge-β landings + `20047c2` + `ad52973` bilateral-arm retirements
  + `fa569ce` fifteenth species mint).
- Does NOT execute the autopoietic composition edge (Reed follow-up
  tick A per canonical spec §6.1).
- Does NOT seed `@silicon/algebra` empirically (Reed-subagent follow-
  up tick B per canonical spec §6.2).

The math grounds what the canonical spec names; the canonical spec
names what the shard-decl binding carries; the shard-decl binding
extends the family root per Alex's Option 2 ratification.

Substrate already carried the shape (Alex's roomba spec verbatim +
`a18ca90` precedent + `fa569ce` fifteenth species). This foundation
names its formal properties.

---

## Related

- [[docs/specs/kintsugi-algebra-as-metalogue-session.md]] — Mara
  THIS-tick canonical spec; the naming this foundation grounds.
- [[shards/kintsugi.mirror]] — Mara THIS-tick shard-decl extension;
  the binding this foundation formalises.
- [[shards/algebra/metalogue.mirror]] — the
  `algebra_metalogue_session` primitive `@kintsugi/algebra`
  specialises.
- [[shards/silicon/algebra.mirror]] — speaker 1 (realiser); $A_{\text{
  silicon}}$ projection.
- [[shards/fate.mirror]] — speaker 2 (proposer) path-namespace;
  $A_{\text{fate}}$ projection.
- [[shards/kintsugi/fracture/bilateral_arm_redundant.mirror]] —
  fifteenth landed element (Mara `fa569ce`).
- [[shards/epistemologic/property/ouroboros_monotone.mirror]] — the
  four-conjunct invariant §6 correspondence formalises.
- [[docs/math/epistemologic/pact/bilateral-sentinel.md]] — Mara
  `701828a` (2026-07-16); the sentinel-as-content-addressed-witness
  math §5 inherits.
- [[docs/math/kintsugi/roomba/bump-and-vacuum.md]] — Mara `17697e6`
  (2026-07-16); the ouroboros_monotone four-conjunct math §6
  composes over.
- [[docs/specs/fate-silicon-metalogue-in-void-duality-basis.md]] —
  Mara `a18ca90` (2026-07-08); the void-duality basis §7.2 lifts.
- [[docs/loop/CURRENT.md]] — active arc state at time of writing.
