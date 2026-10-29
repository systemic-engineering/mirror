# The engine's frequencies, and kleinos at the engine layer — math foundation

*2026-10-29. Mara. Review of the engine core `E` (`rust/engine`, `87e6860..83fe665`) for Reed and Alex. Spec: `docs/specs/mirror-fixpoint-parser.md` §4.1–§4.4.*

Labels: **PROVED**, **COMPUTED**, **CONJECTURE**, **OPEN**, **REFUTED**, **READING**.

**Briefs (Alex, relayed by Reed, 2026-10-29).**
- "look for cross-type patterns. That's where the frequencies come in already at the Rust layer." "Maybe a Frequency trait or something like that."
- "each tick is like a thunder going through the CPU, the shape of which was the settled tree of the previous tick. X = COORD(X). That's where kleinos comes in."

---

## §0 The answer

1. **One shape: an eventually periodic orbit, read through addresses.** Every deterministic iteration in mirror (E's rounds, W's alternation, `rotation::Splinter`, `compile --fixpoint`, convergence passes, kintsugi re-runs) is a step map on a set of states, compared by address. On a finite set every orbit has a unique tail μ and period λ (F1), and every recurrence after the tail is a multiple of λ (F2). "Fixed", "cycle(p)" and "budget" are λ = 1, λ = p ≥ 2, and "μ + λ exceeds the window". These are PROVED, and they justify one `Frequency` trait in place of five stop enums.
2. **Monotone steps have no cycles (F4, PROVED)**, so E's stop type correctly has no `Cycle` case. The trait can make that case unconstructible for monotone kinds.
3. **W's undefined atoms are exactly its period-2 atoms (F6, PROVED).** The comma inside one program is a frequency in the literal sense.
4. **The spectral link is honest for linear steps and for the functional-graph lift (PROVED).** Period ↔ roots of unity in the spectrum. For Kuramoto rotation it is an analogy until a linearisation is declared.
5. **Kleinos at the engine layer:**

   | property | verdict here |
   |---|---|
   | sovereignty | PROVED for positive programs (monotonicity) |
   | emergent third | a computable set of addresses (PROVED). Crossing is necessary (PROVED) and not sufficient (counterexample). "Exactly one" is REFUTED |
   | fusion refusal | idempotence of rule union (PROVED) |
   | Fiedler rise | REFUTED as a law here (counterexample) |
   | the tick's shape | Reed's reading PROVED in its exact form (§2.1) |
   | negation and the comma | under W sovereignty fails. The failure is a part disagreeing with the glue: NARROWED from "exactly the comma" to the relative-H¹ shape of the kintsugi math Prop 5.1 |

---

## §1 Frequency

**Setting.** A deterministic step `f : S → S` on a set of states, with `addr : S → Addr` injective (Merkle, barring a collision). The orbit is `s_t = f^t(s₀)`. Every iterating thing in the corpus is one of these:

| iteration | state | step | monotone? |
|---|---|---|---|
| E's core | `M_t ⊆ B` | `M ↦ M ∪ T_P(M)` | yes (inflationary) |
| W's outer loop | `under_k` | `γ ∘ γ` | yes (monotone, from `under₀ = F ⊆ γ²(F)`) |
| W's raw step | `S ⊆ B` | `γ` | **antitone** |
| `rotation::Splinter` (decision 7) | phases, quotiented by common rotation | one Kuramoto tick | no |
| `compile --fixpoint` (§3) | `N_k` | read with `E(N_{k−1})` | no |
| convergence (`mirror-convergence.md`) | the corpus root | kintsugi → mend → write back | no |
| kintsugi re-run on fixed input | the run record | `run` | constant (λ = 1 at once) |

**F1 (rho; PROVED).** If the orbit takes finitely many values, there are unique `μ ≥ 0`, `λ ≥ 1` with `s_{t+λ} = s_t` for all `t ≥ μ`, chosen minimal.
*Proof.* By pigeonhole some value repeats. Let `μ` be the least index whose value recurs, and `λ` the least positive gap to its recurrence. Determinism propagates the recurrence to every later index. Minimality makes the pair unique. ∎

**F2 (divisibility; PROVED).** For `t ≥ μ`: `s_t = s_{t+q}` iff `λ | q`.
*Proof.* "If" is F1 iterated. "Only if": write `q = aλ + r` with `0 ≤ r < λ`. Then `s_t = s_{t+r}`, and so `s_{t′} = s_{t′+r}` for every `t′ ≥ t`, including the indices of the first cycle. If `0 < r < λ`, that contradicts the minimality of λ. ∎

**F3 (PROVED).** Fixed ⇔ λ = 1 ⇔ `f(s_μ) = s_μ`, which is Alex's `X = COORD(X)`.

**F4 (monotone ⇒ no cycle; PROVED).** If `s ⊆ f(s)` for all s in a finite powerset, then λ = 1.
*Proof.* The orbit is an increasing chain. `s_t = s_{t+q}` with `q ≥ 1` squeezes `s_t ⊆ s_{t+1} ⊆ … ⊆ s_{t+q} = s_t`, so `s_t = s_{t+1}`. ∎
So E's `Stop` has two cases (`Fixed`, `Budget`), correctly, and so does W's outer loop.

**F5 (phase from deposits; PROVED).** If the address of every `s_t` is recorded, the first index `t` whose address equals an earlier one at `t′` gives `μ = t′` and `λ = t − t′`. For `t ≥ μ` the phase is `(t − μ) mod λ`.
*Proof.* Indices below the first repeat hold distinct states, so the first repeat is the entry of the cycle (F1). ∎
Without recorded addresses, Floyd's and Brent's algorithms find `(μ, λ)` in `O(μ + λ)` steps with `O(1)` memory. Brent (BIT 20 (1980) 176–184): "We describe a cycle-finding algorithm which is about 36 percent faster than Floyd's (on the average)". The store already keeps deposits, so F5 costs nothing extra.

**Consequence for the specs (PROVED from F2).** A loop that stops only on **consecutive** equality detects λ = 1 and nothing else. A 2-cycle shows unequal neighbours forever and reaches its budget. Compile §3 and decision 7 already report `cycle(p)` separately; F5 is how to compute it.

**F6 (W's period two; PROVED).** Let `under`, `over` be W's final values. Then `γ(under) = over` and `γ(over) = under`. So the raw γ-orbit from `under` is `under, over, under, …`, with λ ≤ 2, and an atom has period 2 in it iff it lies in `over ∖ under`, i.e. iff it is undefined.
*Proof.* `over = γ(under)` by definition. At the stop, `under = γ(over)` (consecutive equality of `under` with `γ(γ(under))`). An atom in `under ∩ over` is constant along the orbit, one outside `over` is constantly absent, and one in `over ∖ under` alternates. ∎
Van Gelder's alternating fixpoint computes exactly this pair: "monotonically to build up a set of negative conclusions until the least fixpoint is reached" (Van Gelder 1993, cited in the fixpoint math §5.3). *COMPUTED by hand on vector 04*: `under = {r}`, `over = {p, q, r, t}`, `γ({r}) = {p, q, r, t}`, `γ({p, q, r, t}) = {r}`. The period-2 atoms are `{p, q, t}`, which is the vector's `undefined`.
READING. The comma inside one program (an atom that is neither true nor false) is an atom that **oscillates** under the program's own antitone reading of itself.

### §1.4 Does period connect to spectra?

**Linear steps (PROVED).** Let `x ↦ Ax` on a finite-dimensional space over ℂ, with `A^λ x = x` and `x ≠ 0`. On the cyclic subspace generated by x, the minimal polynomial of A divides `t^λ − 1`, which has distinct roots. So A is diagonalisable there, its eigenvalues are λ-th roots of unity, and the minimal period of x is the lcm of the orders of the roots whose eigencomponents of x are non-zero. Fixed ⇔ x lies in the eigenspace of 1.

**Any finite step map (PROVED).** Encode f as the 0/1 matrix `P` with `P_{f(s), s} = 1` (its functional graph).
- Restricted to a cycle of length p, P is a cyclic permutation matrix with characteristic polynomial `t^p − 1`: the p-th roots of unity. This is the standard fact about permutation matrices, which I also checked directly.
- Restricted to the tail states, after quotienting by the cycles, P is nilpotent: eigenvalue 0.
- So **the spectrum of the step's functional graph is exactly the union, over its cycles, of the roots of unity of their lengths, plus 0**.

This is exact, and it is the honest link between "frequency" and "spectrum". But it lives on the state space: for E that is `2^{|B|}` states, so it is a statement, not a computation.

**Kuramoto rotation (READING, not proved).** `rotation.rs` is non-linear phase dynamics, and its address equality passes through a quantising ruler (decision 7: states equal modulo common phase rotation). Calling its λ a "frequency" in the spectral sense is an analogy until someone declares the linearisation (the Jacobian at a phase-locked state) and the ruler. Then the linear case above applies to the linearisation, and only near the locked state.

---

## §2 Kleinos at the engine layer

**Source.** prism `prismqueer::spectral::kleinos` (read at `7e6b00f`) names four properties (`Property::{Sovereignty, EmergentThird, FiedlerRise, FusionRefusal}`), checked on a `Graph` that `compose_candidate` **constructs**:
- the union of both inputs;
- one added vertex `V_e` (the "emergent third");
- complete bipartite bridging, chosen, in its own words, "satisfying strict rise for all reasonable test inputs".

So there the four properties hold **by construction of the compose**, not as theorems about a composition defined independently. 10-10 §8.6 already says: "Fiedler strict rise as a law … holds for the join that `kleinos.rs` builds, and not in general." Here I ask what the four become when the composition is fixed by the engine: **rule union of two Datalog programs**.

Notation: `lfp(P, F)` = the least model; `L₁ = lfp(P₁, F₁)`, `L₂ = lfp(P₂, F₂)`, `L = lfp(P₁ ∪ P₂, F₁ ∪ F₂)`.

### §2.1 The tick's shape (Reed's first bullet)

**Proposition 2.1 (PROVED).** An atom first derived at tick t ≥ 1 has a derivation whose premises all have tick ≤ t − 1, at least one of them exactly t − 1. Semi-naive round t joins each rule once per body position against `Δ_{t−1}` and against `M_{t−1}` elsewhere, so it finds exactly these derivations.
*Proof.* If every derivation of the atom had all premises at ticks ≤ t − 2, it would be derived in round t − 1 at the latest (naive equivalence, math Prop. 5.4). That contradicts its tick. ∎

READING. "The shape of each tick is what settled in the last one" holds exactly in this sense: every new atom leans on at least one atom that settled in the previous tick. "Thunder" is the delta. The tick stays out of the address (Prop. 1.7), so the thunder leaves no trace in what it built, only in the deposits.

### §2.2 Sovereignty

**Proposition 2.2 (PROVED).** For positive programs, `L₁ ∪ L₂ ⊆ L`.
*Proof.* `T_{P₁} ≤ T_{P₁∪P₂}` pointwise, and `F₁ ⊆ F₁ ∪ F₂`, so the least fixed points are ordered (Tarski). ∎

### §2.3 The emergent third

**Definition 2.3.** `E₃ = L ∖ (L₁ ∪ L₂)`: what neither program derives alone and both derive together. It is a set of addresses, computed by three runs.

**Proposition 2.4 (crossing is necessary; PROVED).** Call an edge of the predicate dependency graph **crossing** if a body predicate of a rule of `Pᵢ` is headed by a rule of `Pⱼ` or occurs in `Fⱼ` (j ≠ i). If there is no crossing edge, `E₃ = ∅`.
*Proof.* Without crossing edges, every rule of `Pᵢ` reads only predicates that `Pⱼ` and `Fⱼ` never produce. So `L₁ ∪ L₂` is closed under `T_{P₁∪P₂}`, and it contains `F₁ ∪ F₂`. The least fixed point is therefore contained in it. ∎

**Not sufficient (PROVED by counterexample).** `P₁ = {q(X) :- p(X)}`, `F₁ = {p(a)}`; `P₂ = ∅`, `F₂ = {p(a)}`. The predicate `p` crosses (P₁ reads it, F₂ holds it), and `E₃ = ∅`.

**"Exactly one emergent third" (prism §3.6.2) is REFUTED at this layer.** `P₁ = {path(X,Y) :- e(X,Y)}` with three edges of a directed 6-cycle; `P₂ = {path(X,Z) :- path(X,Y), path(Y,Z)}` with the other three. Then `L` has all 36 `path` atoms, `L₁` has 3, `L₂` none, so `|E₃| = 33`. In prism the "one" is the added vertex `V_e`. That vertex is a witness the constructor adds, not something the composition finds.

### §2.4 Fusion refusal

**Proposition 2.5 (PROVED).** Rule union is idempotent, `P ∪ P = P`, so composing a program with itself returns the same model and the same address, and `E₃ = ∅`.
READING. Prism's FusionRefusal (refusing a composition whose inputs have equal content addresses) appears here as a no-op the address makes visible: K₂ collapses to K₁ by itself, and nothing has to refuse.

On "never an average": the models form a powerset lattice. Composition is `L`, which contains the join `L₁ ∪ L₂`, with `E₃` as the excess. The only "consensus" operation the lattice offers is the meet `L₁ ∩ L₂`, which would delete each side's own atoms and break Prop 2.2. That is what fusion would be here, and rule union never computes it. (PROVED, as stated: `L ⊇ L₁ ∪ L₂ ⊇ L₁ ∩ L₂`.)

### §2.5 Fiedler rise

**Is there an analogue? No law (REFUTED).** Take the first K-E2 fixture: `P₁ = {q(X) :- p(X)}`, `F₁ = {p(a)}`; `P₂ = {r(X) :- q(X)}`. Then `E₃ = {r(a)}`.
- The dependency graphs (undirected) are `p — q` and `q — r`, each `K₂`, with `λ₂ = 2`.
- The union is the path `p — q — r`, with Laplacian spectrum `{0, 1, 3}` and `λ₂ = 1`.
- **λ₂ falls while an emergent third exists.** Strict rise is not necessary for E₃ ≠ ∅.
- In the other direction, two programs with identical rules over disjoint predicates have a disconnected union (`λ₂ = 0`) and `E₃ = ∅`. So rise isn't sufficient either, for the trivial reason that λ₂ can't rise there.

**What survives** is the trivially strict "rise" `|L| > |L₁ ∪ L₂| ⇔ E₃ ≠ ∅` (a tautology) and the necessary crossing condition of Prop. 2.4, which is a statement about `b₀`: E₃ ≠ ∅ requires the two dependency graphs to share a component. Whether any spectral quantity of the dependency graph predicts E₃ beyond that is **OPEN**.

### §2.6 Negation: retraction and the comma

**Proposition 2.6 (sovereignty fails under W; PROVED by example).** `P₁ = {p :- not q}`, `P₂ = {q.}`. `W(P₁)`: `p` true. `W(P₁ ∪ P₂)`: `q` true, `p` false. A true atom of a part is false in the glue.

**Is that "exactly the comma"? NARROWED.**
- Kintsugi's line comma is two **parts** disagreeing on a line (kintsugi math Def 3.2). Here a **part disagrees with the glue**: `P₁`'s local verdict on `p` doesn't extend to the composition.
- That is the shape of the kintsugi math Prop 5.1: a local section that fails to extend, an obstruction in the relative H¹ of the pair. It is the hole/comma rotation, not a line comma.
- So "compile vs kintsugi again" holds with a precise reading:
  - **compile** = each program's own W model (local sections);
  - **kintsugi** = the W model of the union;
  - **the comma** = the set `{a : status_{Pᵢ}(a) ≠ status_{P₁∪P₂}(a)}` restricted to `Pᵢ`'s vocabulary, owned by the part–glue pair. On positive programs it is empty by Prop 2.2: **commas need negation at the engine layer** (PROVED, from Prop 2.2).

**CONJECTURE 2.7.** A true atom of `Pᵢ` is retracted in the union only if some negative literal of `Pᵢ` mentions a predicate whose extension the other side changes. That is a crossing **negative** edge. Test K-E6 plus a seeded search would refute it.

---

## §3 Ledger

| claim | status |
|---|---|
| F1 rho, F2 divisibility, F3 fixed ⇔ λ = 1 | PROVED |
| F4 monotone ⇒ no cycle (E's two-case stop) | PROVED |
| F5 (μ, λ) from deposits; Brent/Floyd without them | PROVED; Brent 1980 cited |
| F6 W's undefined = period 2 of γ | PROVED; vector 04 by hand |
| consecutive equality detects only λ = 1 | PROVED |
| period ↔ roots of unity, linear steps and functional-graph lift | PROVED |
| period ↔ spectrum for Kuramoto rotation | READING (analogy) |
| §4.1 bound `T ≤ \|B\| − \|F\| + 1` (the builder's off-by-one) | PROVED |
| Prop 2.1 the tick's shape | PROVED |
| Prop 2.2 sovereignty (positive) | PROVED |
| `E₃` computable as addresses | PROVED |
| Prop 2.4 crossing necessary; not sufficient | PROVED (counterexample) |
| "exactly one emergent third" | REFUTED (`\|E₃\| = 33`) |
| Prop 2.5 fusion = idempotence; never the meet | PROVED |
| Fiedler strict rise as a law | REFUTED (λ₂: 2 → 1 with `E₃ ≠ ∅`) |
| a spectral predictor of `E₃` beyond `b₀` | OPEN |
| Prop 2.6 sovereignty fails under W | PROVED |
| "retraction = exactly the comma" | NARROWED to part-vs-glue (relative H¹ shape) |
| commas need negation at the engine layer | PROVED (from Prop 2.2) |
| Conj 2.7 retraction needs a crossing negative edge | CONJECTURE |

**For Seam.** F6's proof at the stop condition; Prop 2.4's closure argument; Conj 2.7.

**In passing.** The scarlet/kleinos metalogue essay repeats overclaims that this doc refutes at the engine layer: "exactly one" emergent third, and strict rise as a law. The essay is in Alex's repo, so this doc only notes it.

## Sources

- R. P. Brent, *An improved Monte Carlo factorization algorithm*, BIT 20 (1980) 176–184, https://link.springer.com/article/10.1007/BF01933190. Abstract via Kagi: "a cycle-finding algorithm which is about 36 percent faster than Floyd's (on the average)". **[abstract read]**
- A. Van Gelder, *The alternating fixpoint of logic programs with negation*, JCSS 1993: cited and quoted in `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md` §5.3 **[not re-read this session]**.
- Permutation-matrix spectra: "If a permutation matrix has a cycle of length k, then the eigenvalues … are the k-th roots of unity" (tinyvolt, *Permutation spectra*, via Kagi) **[snippet]**. The proof in §1.4 is self-contained.
- prism `prismqueer/src/spectral/kleinos.rs` at `7e6b00f` (`Property` enum; `compose_candidate`'s docblock) **[read]**.
- `rust/engine/src/lib.rs` at `83fe665` **[read in full]**.
