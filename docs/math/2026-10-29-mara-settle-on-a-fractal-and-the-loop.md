# Settle on a Fractal, and the Loop — math foundation

*2026-10-29. Mara. Composed with Scarlet: `docs/essays/2026-10-29-mara-scarlet-metalogue-settle-loop.md`. Types: `docs/specs/mirror-settle.md`. Builds on `docs/math/2026-10-29-mara-engine-frequency-and-kleinos.md` (cited "freq §n"; laws F1–F6).*

Labels: **PROVED**, **COMPUTED**, **CONJECTURE**, **OPEN**, **READING**, **NARROWED**.

**Brief (Alex and Reed, 2026-10-29).**
- (A) `settle` on a `Fractal`: a circular-recursive loop that sorts each fragment by its orbit each tick. Fixed → Crystal (cached, dropped); Cycle(p) → flux (look, ask, settle by a human or a declared ruler, with the erasure logged); Unknown → Dark. Locally unchanged ≠ settled.
- (B) Alex: "Circular implies that we can have a `Loop` … We basically look for closed loops and ask about open loops." The Pythagorean comma; temperament as fusion; period 2 as the tritone; `γ ∘ γ`.

---

## §0 The answer

1. **Settled is a property of a fragment's cone, not of the fragment.** A fragment is settled at t iff every fragment in its dependency cone (itself included) is constant from t on (Def A.2). Checking that for one fragment alone fails: a fragment can sit still for a tick and move later (Prop A.3, PROVED by example). The evaluation order that decides it is the **SCC condensation order**: recursive between components, circular inside each (Prop A.4, PROVED).
2. **Where dependencies are held by address, the dark bits decide** (Prop A.5, PROVED, Merkle). Where they are held by name (cross-shard references in mirror's normal form are never unfolded), only the SCC order decides.
3. **Fixation is lossless restriction** (Prop A.6, PROVED). Dropping settled fragments from later ticks and serving them from cache changes no trajectory. Seen from inside, this is kintsugi's star-locality: an edit unsettles exactly its up-cone.
4. **A cycle settles only through a ruler** (Prop A.7, PROVED). Choosing one of p ≥ 2 phases is non-injective, and the other p − 1 go to the erasure.
5. **Under W, a stratified program settles to two values, component by component** (PROVED, cited). A component with a negative cycle leaves F6's period-2 atoms: flux. That the alternating fixpoint can be run component by component in general is a CONJECTURE here.
6. **A `Loop` is a closed path with a holonomy in an exact group; the comma is the holonomy** (Def B.1). No number of pure fifths closes on octaves (Prop B.3, PROVED). The circle's holonomy is 531441/524288.
7. **Equal temperament is fusion** (Prop B.4, PROVED). It distributes the log-holonomy equally over the twelve edges and leaves ℚ to do it. In `f64` it doesn't even close: 1.0000000000000002 (COMPUTED).
8. **Period 2 ↔ eigenvalue −1 ↔ the tritone in ℤ₁₂** (PROVED). Two Pythagorean tritones overshoot the octave by exactly the Pythagorean comma (PROVED; COMPUTED). The engine's half turn is exact for a different reason: polarity is a ℤ₂ grading, not a ratio (Prop B.6, PROVED). "Two tritones make an octave" as W's convergence argument is a READING on the clock and a proof on the polarity group.
9. **Orbits are loops** (Prop B.7, PROVED): closed in the state quotient by definition, with the comma appearing in the lift. **Kintsugi's twist is an open loop only for a constant group on a cycle**; Seam's tree twist is not a loop (NARROWED).

---

## Part A — settle on a Fractal

### A.1 Setting

**Definition A.1.** A **Fractal** is a finite set of fragments V with a dependency relation `D ⊆ V × V`. `v D u` means "v depends on u". Each dependency is either:
- **contained**: u's address is inside v's address preimage (a Merkle child); or
- **named**: v refers to u by name, and u's address is not in v's preimage. This is mirror's normal form for cross-shard references (fixpoint math Prop 3.5) and for references inside an SCC (`(SCC, index)`, fixpoint math §1.4).

Each fragment has a state `s_v(t)` in a finite set. A tick is deterministic and time-invariant:
`s_v(t+1) = f_v(s_v(t), (s_u(t))_{u ∈ D(v)})`.

The **cone** of v is `cone(v) = {v} ∪ D⁺(v)` (D⁺ the transitive closure). A cone is closed under D.

**Definition A.2.** v is **settled at t** iff `s_u(t′) = s_u(t)` for every `u ∈ cone(v)` and every `t′ ≥ t`.

### A.2 Locally unchanged is not settled

**Proposition A.3 (PROVED by example).** `s_v(t) = s_v(t+1)` doesn't imply that v is settled at t.
*Example.* u counts to 2 and stays: `s_u = 0, 1, 2, 2, …`. v copies u's previous state: `s_v(t+1) = s_u(t)`, with `s_v(0) = 0`. Then `s_v = 0, 0, 1, 2, 2, …`. v is unchanged from tick 0 to tick 1 and changes at tick 2. ∎

**Proposition A.3′ (the cone is enough; PROVED).** If the whole cone's state is equal at t and t + 1, then v is settled at t.
*Proof.* The cone is closed under D, so its states evolve as an autonomous deterministic system. A state equal to its successor is a fixed point of that system, and the system stays there. ∎

### A.3 Circular-recursive: the SCC order

**Proposition A.4 (PROVED).** Order the strongly connected components of D's graph topologically, dependencies first: `C₁, …, C_m`.
- (a) If every component below C is settled from t₀ on, then C, with those inputs constant, is an autonomous system on a finite set. It has a tail `μ_C` and period `λ_C` (freq F1). C is settled from `t₀ + μ_C` iff `λ_C = 1`.
- (b) If some component below C has period λ > 1, then no member of C is settled by Def A.2, whatever C's own states do. C's projected period divides the period of the joint orbit.
- (c) So "settled" is computed by one pass in component order. Each component is either settled whole or not at all: members of one SCC lie in each other's cones.

*Proof.* (a) is F1 and F3 applied to the component's subsystem. (b): Def A.2 quantifies over the cone, and the projection of a periodic sequence has a period that divides the sequence's period. (c): the members of an SCC have equal cones. ∎

READING. "Circular-recursive", taken literally: recursive down the condensation DAG, circular inside each component. This is the evaluation order of stratified Datalog, read as a settle loop.

### A.4 Dark decides

**Proposition A.5 (PROVED, barring hash collisions).** If every dependency in `cone(v)` is **contained**, then `addr(s_v(t))` determines the state of the whole cone at t. So `addr(s_v(t)) = addr(s_v(t+1))` implies v is settled at t.
*Proof.* Merkle injectivity (fixpoint math Prop 1.1) by induction down the cone, then Prop A.3′. ∎
For **named** dependencies the shortcut fails: the name stays the same while what it names changes, which is exactly Prop A.3's pattern. There, only Prop A.4's order decides. *Mirror's cross-shard references are named.* So a shard's own `root` staying the same never proves the shard is settled. Kintsugi's line keys exist for this reason (kintsugi math §4).

**Active suggests (PROVED).** Deciding recurrence by the active half of `uuid_spectral` is unsound: cospectral, non-isomorphic sub-DAGs share active bits (fixpoint math Prop 7.11), so comparing active bits can report a false Fixed or a false Cycle. Recurrence is decided by address (freq F5). Active bits only rank suggestions.

### A.5 What each orbit case becomes

**Proposition A.6 (fixation is lossless restriction; PROVED).** At each tick, run the step only on the unsettled fragments `R(t)`, and serve every settled fragment's constant state from a cache keyed by its address. Every trajectory equals the full run's.
*Proof.* Settled states are constant (Def A.2). The step is deterministic, and every input an unsettled fragment reads is either recomputed or served unchanged. ∎
*Corollary (PROVED).* A change to fragment u can unsettle exactly the fragments whose cone contains u: the up-cone of u. This is kintsugi's star-locality seen from inside the settle loop.

**Proposition A.7 (a cycle settles only through a ruler; PROVED).** If a component has `λ ≥ 2`, any map that picks one state of its cycle sends λ distinct states to one. That is non-injective, so by fixpoint math §2.3/§4.4 it is a `settle` and requires a named ruler. The erasure records the λ − 1 phases not chosen.

**Unknown (PROVED, by F1).** A component with no recurrence within the window satisfies `μ + λ > window`. The verdict is about the observer's window, not the component: Dark ("nothing looked long enough").

**Definition A.8 (per-fragment state; Alex's 2026-09-04 sum type, read per fragment).**
- **Fractured**: the fragment's cone doesn't glue (a kintsugi comma, gap or claim among its lines).
- **Flux**: glued, with some cone component at `λ ≥ 2` (`Cycle(p)`) or Unknown (`Dark(window)`).
- **Settled**: glued, with every cone component at `λ = 1`. The fragment becomes a Crystal.

### A.6 Under W

**Proposition A.9 (PROVED, cited).** For a stratified program (no negation inside a component of the predicate dependency graph), each component is positive once the components below it are fixed. Each one settles with `λ = 1` (freq F4), and the well-founded model is two-valued. Van Gelder, Ross & Schlipf (JACM 1991): "for locally stratified programs the perfect model coincides with the well-founded model".

**For a component with a negative cycle**, W's alternation (freq F6) leaves the atoms in `over ∖ under`, the period-2 atoms. Those are Flux with `Cycle(2)`: the undefined atoms. No ruler is applied to them by default, so they stay asked, not answered.

**CONJECTURE A.10.** Running W component by component in SCC order (each component's W with the lower components' three-valued result fixed) yields the same well-founded model as W on the whole program. I have not proved it here or cited a primary source. Test: the engine's E5 vectors run both ways must agree.

---

## Part B — the Loop

### B.1 Loop and holonomy

**Definition B.1.** Let Γ be a graph whose edges carry labels in a group G; traversing an edge backwards uses the inverse.
- A **Loop** is a non-empty closed edge path γ from a base vertex back to itself.
- Its **holonomy** `hol(γ) ∈ G` is the ordered product of the labels.
- γ is **closed** iff `hol(γ) = e`; otherwise it is **open**, and `hol(γ)` is its **comma**.

Policy (Alex): closed loops get settled; open loops get asked about.

**Proposition B.2 (gauge; PROVED).** Re-labelling by vertex potentials, `g_e ↦ φ(head) g_e φ(tail)⁻¹`, changes `hol(γ)` to `φ(b) hol(γ) φ(b)⁻¹`, with b the base. So closedness and the comma's conjugacy class are gauge-invariant; for abelian G, the comma itself is.
*Proof.* Adjacent potentials cancel along the path. ∎

### B.2 The Pythagorean comma

**Proposition B.3 (PROVED; COMPUTED).** Label the circle of fifths in `G = ℚ_{>0}^× / ⟨2⟩` (ratios modulo octaves) with the pure fifth 3/2 on each of its 12 edges. Then:
- `hol = [3¹²] ≠ e`;
- in fact no positive number of pure fifths closes: `3^a ∈ ⟨2⟩` forces a = 0, by unique factorisation;
- the representative in [1, 2) is `3¹²/2¹⁹ = (3/2)¹²/2⁷ = 531441/524288 ≈ 1.0136433`, about 23.46 cents.

*COMPUTED* with exact rationals (Python `fractions`). The value is confirmed by the Xenharmonic Wiki ("531441/524288 … the amount by which twelve perfect fifths exceed seven octaves, or in other words (3/2)¹²/2⁷") and Wikipedia ("Take the just fifth to the 12th power, then subtract seven octaves").

### B.3 Temperament is fusion

**Proposition B.4 (PROVED).** Equal temperament replaces each fifth's label with `2^{7/12}`. Then:
- (a) the holonomy becomes `2⁷ ≡ e`: the loop closes;
- (b) in log coordinates, `log 2^{7/12} = log(3/2) − (1/12)·log(comma)`. Each edge absorbs exactly one twelfth of the log-holonomy. This is the arithmetic mean of the disagreement spread over the loop, as the literature puts it: equal temperament flattens "each fifth by a twelfth of a Pythagorean comma" (Wikipedia; Scholtz, MTO 4.4, 1998);
- (c) `2^{7/12} ∉ ℚ`: if it were p/q, then `2⁷q¹² = p¹²`, and 2-adic valuations give `7 + 12·v(q) = 12·v(p)`, so 7 ≡ 0 mod 12. Contradiction. The loop closes only by leaving the rationals;
- (d) the map "temper" sends every circle-of-fifths labelling to a closed loop. It is non-injective on holonomies and erases the comma: **fusion**, in the kleinos sense of averaging two voices into one (freq §2.4).

*COMPUTED* (`f64`): twelve tempered fifths divided by 2⁷ give `1.0000000000000002`, not 1. A tolerance-based test calls that closed (false closure); an exact `f64` test calls it a comma that belongs to the rounding (a false comma). Twelve pure fifths in `f64` give `1.0136432647705078`, which is exact, because the denominator is a power of two. **The float isn't what loses the comma; tempering is. And floats then fail both ways at the identity.**

Mirror's law (Alex's exact-values decision, compile §9.1): **never temper.** No operation maps a comma to the identity except a `settle` with a named ruler, whose erasure records the comma.

### B.4 Period two, the tritone, and γ ∘ γ

**Proposition B.5 (PROVED).**
- (a) ℤ₁₂ is cyclic, so it has exactly one element of order 2, namely 6: the tritone, half the circle.
- (b) Under the character `k ↦ e^{2πik/12}`, 6 ↦ −1.
- (c) For a linear step, an orbit of exact period 2 has a non-zero component in the eigenspace of −1 (freq §1.4: the minimal period is the lcm of the orders of the roots of unity present).
- (d) **Exact ratios:** the Pythagorean augmented fourth (six fifths folded into the octave) is `729/512`, and `(729/512)² = 2 · 531441/524288`. *Two Pythagorean tritones overshoot the octave by exactly the Pythagorean comma.*

*COMPUTED* (exact). The value 729/512 is confirmed by Wikipedia's *Pythagorean interval* and the Xenharmonic Wiki ("reached by stacking six perfect fifths (3/2), and reducing by three octaves").

**Proposition B.6 (the engine's half turn is exact; PROVED).** Maps between posets carry a polarity in `ℤ₂ = {monotone, antitone}`, and composition adds polarities: antitone ∘ antitone is monotone. `γ` is antitone (Van Gelder, *The alternating fixpoint of logic programs with negation*, PODS 1989 / JCSS 1993: "monotonically to build up a set of negative conclusions until the least fixpoint is reached"). So `γ ∘ γ` is monotone, and its iteration from F climbs to a fixed point (Knaster–Tarski; finite, freq F4). The polarity group has no ratio in it, so it has no comma. The embedding `ℤ₂ ↪ ℤ₁₂`, `1 ↦ 6`, is a homomorphism.

READING. "Two tritones make an octave" as W's convergence argument:
- **exact** on the polarity group;
- **exact** on the tempered clock ℤ₁₂;
- **false by exactly the Pythagorean comma** on just ratios (B.5(d)).

The proof is the polarity. The tritone is how it sounds. The atoms that flip every step of γ (freq F6) are where the half turn shows: **the undefined atoms**.

### B.5 Orbits and twists as loops

**Proposition B.7 (orbits are loops; PROVED).** An orbit's cycle is a closed path in the step's functional graph. In the state space it always closes (that is what a period is), and Fixed is the loop of length one. Let states carry labels in G, with each step multiplying by `g(s)`. Then the lift of the orbit's cycle has holonomy `∏ g(s)` around it. The comma lives in the lift, not in the quotient.
*Example (PROVED, B.3).* `+7 mod 12` on ℤ₁₂ has the orbit of C with period 12, closed. Its lift to ratios carries holonomy 531441/524288, open. The circle of fifths is a closed orbit on the clock and an open path on the string.

**Twist (NARROWED).** With the same group on every line, a twist around a cycle of lines is exactly an open loop (kintsugi math Prop 3.5). Seam's tree twist (kintsugi math Prop 3.6: different groups per line, a frozen consumer, no cycle) is not a loop. It is a failure to solve for gauges. "Kintsugi's twist is an open loop over the cover" is true in the constant case only.

---

## Ledger

| claim | status |
|---|---|
| A.3 locally unchanged ≠ settled | PROVED (example) |
| A.3′ cone fixed for one tick ⇒ settled | PROVED |
| A.4 SCC order: recursive between, circular inside | PROVED |
| A.5 contained dependencies: the address decides | PROVED (Merkle) |
| A.5 named dependencies: the address does not decide | PROVED (A.3 pattern) |
| active bits can't decide recurrence | PROVED (Prop 7.11) |
| A.6 fixation is lossless restriction; up-cone locality | PROVED |
| A.7 a cycle settles only through a ruler | PROVED |
| A.9 stratified ⇒ two-valued, per component | PROVED (cited, VRS 1991) |
| A.10 W component by component equals W on the whole | CONJECTURE |
| B.2 gauge invariance of holonomy | PROVED |
| B.3 no pure fifths close; comma = 531441/524288 | PROVED; COMPUTED |
| B.4 temperament = log-mean distribution, leaves ℚ, fusion | PROVED |
| B.4 `f64`: false closure under tolerance, false comma under exact equality | COMPUTED |
| B.5 tritone = unique order-2 element ↦ −1; linear period 2 ⇔ a −1 component | PROVED |
| B.5(d) two Pythagorean tritones = octave × comma | PROVED; COMPUTED |
| B.6 polarity ℤ₂: γ antitone, γ ∘ γ monotone, exact | PROVED |
| "two tritones make an octave" as the convergence argument | READING (exact on ℤ₂ and on ℤ₁₂; false on just ratios) |
| B.7 orbits are loops; the comma lives in the lift | PROVED |
| twist = open loop | NARROWED (constant group on a cycle only) |
| does a near active coordinate predict a transferable resolution | CONJECTURE (freq/kintsugi math §11; unchanged) |
| a spectral predictor of which flux a ruler should settle | OPEN |

## Sources (Kagi, this session)

- Pythagorean comma: Xenharmonic Wiki, https://en.xen.wiki/w/Pythagorean_comma ; Wikipedia, https://en.wikipedia.org/wiki/Pythagorean_comma (snippets: the value, "twelve perfect fifths exceed seven octaves", "flattening each fifth by a twelfth of a Pythagorean comma"). **[snippets]**
- B. Scholtz, *Algorithms for Tunings and Temperaments*, MTO 4.4 (1998), https://www.mtosmt.org/issues/mto.98.4.4/mto.98.4.4.scholtz.html: "Equal temperament flattens the fifths … by 1/12 of a Pythagorean comma". **[snippet]**
- 729/512: Wikipedia *Pythagorean interval*; Xenharmonic Wiki https://en.xen.wiki/w/729/512 **[snippets]**
- A. Van Gelder, *The alternating fixpoint of logic programs with negation*, PODS 1989 (https://dl.acm.org/doi/10.1145/73721.73722) and JCSS 1993 (https://www.sciencedirect.com/science/article/pii/002200009390024Q). **[abstracts]**
- A. Van Gelder, K. Ross, J. Schlipf, *The well-founded semantics for general logic programs*, JACM 38(3) 1991, https://www.cse.unsw.edu.au/~rvg/negative/p619-van_gelder.pdf ("for locally stratified programs the perfect model coincides with the well-founded model"). **[abstract snippet]**
