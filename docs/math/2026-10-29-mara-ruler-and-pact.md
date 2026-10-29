# Ruler and Pact — math foundation

*2026-10-29. Mara (with Scarlet). Spec: `docs/specs/mirror-ruler.md`. Builds on the coupling math (`docs/math/2026-10-29-mara-coupling-and-relationship.md`, cited "coupling §n"), the settle/Loop math ("settle §n"), the engine math ("freq §n") and fixpoint spec §4.1 ("E").*

Labels: **PROVED**, **COMPUTED**, **CONJECTURE**, **OPEN**, **REFUTED**, **READING**.

**Alex's moves (2026-10-29, relayed by Reed):**
1. "Their disagreement is in the shape of the comma. It's both. And a ruler decides how it's weighted. We really need to make the `Ruler` explicit … probably parts of the Rust-side pact enforcement."
2. "What if what upholds the coupling is a `Fractal` of `Ruler`s and that's basically what a `Pact` is? In the single truth ontology there's only ever one ruler. In a multi-truth ontology there's a fractal of rulers and they don't need to agree on everything, which becomes commas within the emergent third."
3. "What if the parser uses the pacts to observe itself? … The pacts observe and restrict the statespace for the next tick."

Taut's ruler inventory (`taut/ruler-inventory`) had not landed when this was written. §7 maps the rulers already listed in fixpoint spec §10, and the mapping is to be re-checked against Taut's list.

> **Folded 2026-10-29 after Seam's crowbar** (`docs/audits/2026-10-29-seam-crowbar-ruler-pact.md`) **and Taut's inventory** (`docs/scouts/2026-10-29-taut-ruler-inventory.md`). The original claims stay visible; each correction is marked *[S n]* for Seam's §9 item n, or *[T]*. §8 holds the new math (families, lattices without holonomy, sealed rulers), and §9 the decisions pending Alex. Where §0 and the body differ, the folded text below wins.

---

## §0 The answer

1. **The Mara/Scarlet comma is settled by declaration, not by a winner** (PROVED, from coupling §6). A weighting ruler declares whether it is reader-dependent. If it is, `weigh` and `arrive` don't commute, Scarlet's reading holds, and the ruler must name the reader it is for. If it isn't, they commute and Mara's reading holds. Both are representable, and the comma is held by whichever ruler is declared.
2. **A `Ruler` is one narrowing decision with content:** Horn rules plus a declared kind. A **`Pact` is a fractal of rulers**, with a ruler graph whose edges are "checks". This is a definition. ~~The existing `@epistemologic/pact` family has this shape, but as a one-level tree~~ *[S 12; T §2]* **Withdrawn:** those shards are predicates, i.e. clauses, on the meaning side. "Pact" names three things, so the fractal of rulers needs its own name (Alex's call; placeholder `RulerFractal`). And a ruler is not one kind: it is **an envelope plus a body per family** (§8).
3. **"A tree of rulers is single truth": NARROWED.**
   - **PROVED:** on a tree of rulers, every disagreement between rulers *can* be booked to one ruler (gauge triviality, coupling Prop 2.4). With b₁ ≥ 1 among the rulers, a disagreement can exist that no single ruler owns.
   - **REFUTED** as stated: a tree can keep its line commas open. Mara and Scarlet's disagreement sits on one line (b₁ = 0) and is kept. So a tree is single-truth only together with **upward resolution**: every comma booked to the level above.
   - The exact statement: **single truth = every comma booked to some ruler. A tree makes booking always possible. A loop makes some booking impossible.** Multi-truth needs a loop, or a declared refusal to book.
   - *[S 5, 6, 11]* **Folded, three ways:**
     - "a tree makes booking always possible" holds only when each ruler's adjustment restricts **surjectively and independently** onto its edges. Seam's frozen tree (Y₁ — X — Y₂, X adjusting both edges at once) is unbookable with no loop, which closes the earlier OPEN item as REFUTED in general (§2);
     - for the **lattice families** (Budget, Resolution, Equivalence, Authority) there are no inverses, so no holonomy at all. Every disagreement has a lawful composite, and multi-truth is only ever a declared refusal (§8.2);
     - shape is computed from **capacity and use**, not from b₁ of declared check edges (§8.3).
4. **Where the regress stops (PROVED).** In a finite "who approves this ruler" graph, every chain of approvals ends in a terminal strongly connected component. That component is either a **sink** (a signed human approval: Alex's guard) or a **cycle** (a pact that governs its own changes). This is the graph-theoretic form of Hart's rule of recognition (a sink) and of self-amending constitutions (a cycle, Ross's puzzle). *[S 7]* But approval by reachability is transitive over **versions**: a human signature on G₀ approves G₁, G₂, … after G₁ drops "changes need H". The conjecture that a human in every cycle blocks self-erasure is **REFUTED** (Seam's fixture). Reed's lean, pending Alex: approval is non-transitive across versions (§4, §9).
5. **Restricted runs** (the parser observing itself at tick boundaries):
   - in the positive core, a ruler that bans atoms can only **withhold** meaning, never change it: `M_R ⊆ lfp` (PROVED);
   - semi-naive = naive still holds for a fixed or monotone ban (PROVED). *[S 1]* For non-monotone bans it fails: a **lifted** ban leaves a semi-naive model that is not a fixpoint (134 of 4000 random runs). Sealed rulers (positive programs over the snapshot) are monotone, so nothing is ever lifted (§8.1);
   - *[S 3]* "withhold, never invent" is about **atoms**. Meaning is the tree read off the atoms, so on a non-functional program a ban that leaves one parse is a Choice;
   - a ban that grows monotonically with what it observes converges (PROVED). A non-monotone or late one still converges (the step is inflationary), but to a timing-dependent model, so late bans are reported (PROVED);
   - the restricted model plus its erasures recovers the unrestricted model (PROVED);
   - under negation, a ban **can** change meaning (`p :- not q`, ban q ⇒ p true; PROVED), so restrictions on W-programs are reported as status changes, never silently.
6. **Bootstrap.** ~~iff their erasure on `mirror.mirror` is empty~~ *[S 4]* **REFUTED in both directions.** A ban outside the start cone changes the erasure and not N; a budget truncates N with an empty erasure. Corrected, and PROVED: **`N_on = N_off` if the erasure is disjoint from the start cone and nothing truncated the run** (§6). The seed needs no observation rulers, and their addresses go into the record.

---

## §1 Ruler

**Definition 1.1.** A **ruler** is a content-addressed declaration with:
- a **kind**: ~~threshold, budget, quantizer, choice, ban, weight, or tolerance~~ *[S 10; T §4–§5]* **a family**: Budget, Choice, Resolution, Equivalence, Authority, or Identity root (§8). Ban is a mechanism; Weight is an input to a Choice;
- **content**: Horn rules evaluated by E, or for numeric kinds, an exact value with its unit;
- a declared **reader dependence**: `ReaderIndependent`, or `ReaderDependent { reader }`;
- an **approval**: a signature, or a governance cycle (§4).

Applying a ruler is a `settle`: non-injective, with the ruler's address and an erasure (fixpoint math §2.3, §4.4).

**Proposition 1.2 (the comma by declaration; PROVED).** Let `weigh_R` be a weighting under ruler R, and `arrive_r` the event of reader r arriving.
- If R is reader-independent (its weights are a function of past resolutions only), `weigh_R ∘ arrive_r = arrive_r ∘ weigh_R`.
- If R's weights depend on r, the two orders give different results whenever `R(r) ≠ R(r₀)` for the reader r₀ assumed at weighing time.

*Proof.* In the first case the two maps act on independent components of the state. In the second, weighing first fixes `R(r₀)`, and weighing after arrival yields `R(r)`. ∎

So the coda's comma is "in the shape of the comma" (Alex): it is both, and which one holds is a field of the ruler. The spec requires the field.

---

## §2 Pact = Fractal of Rulers

**Definition 2.1.** A **pact** is a fractal of rulers:
- leaves are each party's own rulers;
- inner nodes are the rulers shared on a line (composites of their children);
- the **ruler graph** has an edge `R — R′` whenever one ruler checks the other: they read the same atoms, and their verdicts are compared.

The containment tree is the fractal. The check edges may go across. `b₁(ruler graph)` is the number of independent checking loops.

~~READING (grep): `shards/epistemologic/pact/algebra.mirror` composes fifteen `requires` over sibling pacts, a star of depth one. The existing family is a hierarchy-shaped fractal of rulers.~~ *[S 12]* **Withdrawn.** `algebra.mirror` is a conjunction of clauses (predicates), not a fractal of rulers.

**Proposition 2.2 (tree ⇒ bookable; loop ⇒ possibly unbookable; PROVED).** Model each ruler's reading of a shared quantity as an element of a constant group Γ, and the discrepancy along a check edge as `g_e = R_head · R_tail⁻¹`.
- (a) On a tree, every family `(g_e)` is the discrepancy family of some assignment of rulers. Equivalently, it can be removed by adjusting the rulers: every disagreement can be booked to one ruler per edge.
- (b) If b₁ ≥ 1, there are families whose holonomy around a cycle is non-trivial, and no adjustment of the rulers removes it.

*Proof.* Coupling Prop 2.4 / kintsugi Prop 3.5, applied to the ruler graph. ∎

*[S 6]* **(a) needs a hypothesis, now stated (PROVED):** each ruler's adjustment group must restrict **surjectively and independently** onto each of its incident edges. Without it, a tree can be unbookable: Seam's frozen tree Y₁ — X — Y₂ (kintsugi math Prop 3.6), where X's adjustment acts on both edges at once and the leaves are frozen. The ledger's "non-constant groups: OPEN" is closed: **tree ⇒ bookable is REFUTED in general; it holds under the surjectivity hypothesis.**

**Verdict on "a pact without loops is a hierarchy, and a hierarchy is one ruler wearing a fractal": NARROWED.**
- **What is PROVED:** without loops, every disagreement *can* be booked to a ruler, so a pact on a tree *that books every comma upward* is functionally one ruler: the composite at the root.
- **What is REFUTED:** "tree ⇒ single truth". A tree may decline to book. Mara and Scarlet's comma lives on one line, b₁ = 0, and is kept open by declaration (`LeaveOpen`). That is multi-truth without a loop.
- **The exact dichotomy:**
  - single truth = every comma booked;
  - multi-truth = some comma unbooked, either because it **can't** be booked (a loop, Prop 2.2(b): structural) or because it **isn't** (a declared refusal: kept).
- ~~**Consequence for the spec:** a tree-only pact is flagged `Hierarchy` only if it also books every comma upward. Otherwise it is `Kept`. A pact with b₁ ≥ 1 is `Polycentric`.~~ *[S 5]* Superseded by §8.3: shape from capacity and use, `Untested` before any comma, and `Polycentric` only for invertible (Choice) families.

Prior art. Ostrom's design principles include "Appropriation, provision, monitoring, enforcement, conflict resolution, and governance activities are organized in multiple layers of nested enterprises" (Ostrom, via Kagi), and her polycentric governance (*Beyond Markets and States*, AER 2010) has decision centres that check each other across levels. In this vocabulary that is a fractal of rulers with cross-checks, b₁ ≥ 1. The mapping is a READING; Ostrom's claims are empirical and are not re-proved here.

---

## §3 Commas between rulers

**Definition 3.1.** A **ruler comma** is a check edge whose two rulers give different verdicts on the same atoms. It is held exactly: both verdicts, both ruler addresses, and the question address `H("ruler-comma\0" ‖ addr R ‖ addr R′ ‖ addr(atoms))`.

**Law (spec).** A ruler comma is never absorbed upward silently. Booking it to a parent ruler is a `settle` with that parent's address and an erasure listing the overruled verdict. *[S 11]* **Per family:** for the lattice families the booking is the lawful composite (min, coarsest, ∧, meet). For Choice and Equivalence there is no comma inside one third: two rulers give two results, a fork (fixpoint spec §10). Sideways booking (to a sibling) only as a recorded act approved by both parties (Reed's lean, D4). This is Alex's "commas within the emergent third": the pact's shared rulers are the emergent third of the parties, and their unresolved commas stay inside it, visible.

---

## §4 Where the regress stops

**Definition 4.1.** The **approval graph** has an edge `R → A` when A approves R. A is a ruler (the pact's change rule) or a human signature.

**Proposition 4.2 (PROVED).** In a finite approval graph, every maximal chain `R → A₁ → A₂ → …` ends in a terminal strongly connected component (one with no edges leaving it). That component is either a single node with no approver (a **sink**) or contains a cycle (a **governance loop**).
*Proof.* The condensation of a finite digraph is a finite DAG, so every path ends in a sink of the condensation. A sink SCC is a single node without a self-loop, or it contains a cycle. ∎

**Reading against prior art.**
- **Sink = Hart's rule of recognition.** "a master meta-rule underlying any legal system that defines the common identifying test for legal validity" (Wikipedia, *Rule of recognition*). Secondary rules "specify how primary rules are created, changed, adjudicated, and identified" (Legal Theory Lexicon 105). Hart's rule of recognition is not validated by a further rule; it exists as accepted practice. Mirror's sink is a **human signature**: Alex's guard, "the guard is the human approving the comma". READING.
- **Cycle = self-amendment.** Alf Ross, *On Self-Reference and a Puzzle in Constitutional Law* (Mind, 1969): an amendment rule that amends itself is "the highest authority of the system", and Ross argued that amending it by its own procedure is paradoxical. Suber's *The Paradox of Self-Amendment* analyses the puzzle. In mirror, a governance cycle is a pact whose change rule is under the pact. Prop 4.2 says such a cycle is a legitimate place for the regress to stop. **Whether it is well-founded** (does a self-approved change ever approve its own removal?) is a fixed-point question Ross raises and this doc doesn't settle: OPEN.
- **Mirror's default (spec):** every approval chain must reach a human-signed sink. A governance cycle is allowed only if one of its members is human-signed. ~~(CONJECTURE that this blocks Ross-style self-erasure)~~ *[S 7]* **REFUTED.** Approval by reachability is transitive over versions. In Seam's fixture, H signs G₀; G₀ approves G₁, which says "changes need G₁"; G₁ approves G₂ and R. Every chain still ends at H, and H's constraint is gone. **Reed's lean (D1, pending Alex):** signatures are over addresses and don't propagate across content changes. A changed ruler in, or approving into, a governance component needs a fresh human signature, and a ruler never approves its successor's relaxation. *Alternative:* an order on constraints ("no weaker than"), which the spec doesn't yet have.

---

## §5 The parser observing itself: restricted runs

**Setting.** E runs ticks (fixpoint §4.1). At the boundary after tick t:
- the model `M_t` is an immutable content-addressed root;
- observation pacts read `M_t` (a snapshot read never perturbs the run: PROVED by construction, since the run continues from the same root);
- they return findings, and approved restricting rulers turn findings into a **ban set** `B_t`: atoms tick t+1 must not deposit. Looking and ruling are two different maps; the spec keeps them as two traits, at Scarlet's insistence.

Reed relates this to Anna Jakobs's thesis (observation at step boundaries, sequential, at a cost). I have not read the thesis, so that link is READING.

**Definition 5.1 (restricted step).** `T^B(M) = M ∪ (T_P(M) ∖ B)`, run semi-naively. With `B_t` depending on the observed snapshot, the run is `M_{t+1} = T^{B_t}(M_t)`, `B_t = R(M_t)`.

**Proposition 5.2 (restricted-run laws; PROVED).**
- **(RR1) Determinism.** If R is a **pure** function of the content-addressed snapshot (and of its named observers' findings, §8.4), the restricted *model* is deterministic per evaluation strategy. *[S 2]* **Attribution:** each withheld atom records the **set** of rulers that banned it (sorted), so the record is invariant under permutations of the ruler list.
- **(RR2) Fixed or monotone ban: least model and semi-naive = naive.** *[S 1]* For a non-monotone ban this fails (§8.1). For a fixed B, `T^B` is monotone, so `lfp(T^B)` exists. Semi-naive equals naive on it, because it is a positive program whose heads are filtered by a fixed set (freq/E Prop 5.4, unchanged). **Name: the least model under ruler R.**
- **(RR3) Soundness: withholding, never inventing.** `lfp(T^B) ⊆ lfp(T_P)`, since `T^B ≤ T_P` pointwise. In the PEG encoding (math §5.1), a ban can make an atom **undefined** (a hole) but can never make a different parse appear: failure is derived positively, so banning `m(alt₁, i, j)` doesn't produce `f(alt₁, i)`, and `alt₂` isn't tried. **Meaning lives in the clauses; policy lives in the pacts.** A pact can withhold meaning, and the withholding shows as holes and erasures. It can't substitute meaning.
- **(RR4) Monotone rulers converge.** If `B_t` is increasing in t (the ruler only adds bans as it sees more), it stabilises at some `B_∞` in a finite domain. If moreover no ban covers an atom deposited before the ban appeared (no late ban, RR5), the result equals `lfp(T^{B_∞})`, the least model under the final ruler.
- **(RR5) Non-monotone or late rulers converge, but to a timing-dependent model.** `T^B` keeps M (it is inflationary), so the run cannot cycle (F4), whatever R does. But if a ban appears only after the atom it covers was deposited, the atom stays. Example: rules `p :- a`, `q :- p`, fact `a`, and R bans `p` iff `q ∈ M_t`. Then `p` is deposited at tick 1, `q` at tick 2, and the ban at the boundary after tick 2 comes too late. The result is not the least model under any fixed ban. **Law (spec):** an atom already deposited that a later ban covers is reported as a **late ban** finding. Only retracting steps (W's outer loop) can cycle, and their `Orbit` is reported from deposits (F5).
  *[S 1]* **Incomplete as first stated: the lifted ban.** Facts `a`; rules `p :- a`, `q :- a`; R = "ban `p` while `q` is absent". At tick 1 `p` is withheld. At tick 2 the ban is gone, but semi-naive never re-fires `p :- a` (`a` isn't in the delta). Semi-naive gives `{a, q}`, naive gives `{a, p, q}`, and the semi-naive model is not a fixpoint of `T^{B_final}` (Seam: 134/4000 random runs). A third finding, **`lifted (atom, tick withheld, tick lifted, rulers)`**, is required, or re-seeding the delta with lifted atoms' instances. Sealed rulers make it impossible (§8.1).
- **(RR6) Recoverability.** With `X` the union of everything withheld (the erasure), `lfp(T_P, M_R ∪ X) = lfp(T_P, F)`.
  *Proof.* `F ⊆ M_R ∪ X ⊆ lfp(T_P, F)`, then monotonicity and idempotence. ∎
  The restricted model plus its erasure is a lossless record **of the model**. *[S 2]* Not of the trajectory: the tick at which a withheld atom would have fallen is lost, and with it the F5 frequency reading. Seam: holds for snapshot-dependent and non-monotone bans too (4000/4000); the proof uses only `M_R ⊆ lfp` and `withheld ⊆ lfp`.

**Proposition 5.3 (under negation a ban changes meaning; PROVED by example).** `P = {p :- not q}`, with a rule deriving `q`. W gives `q` true and `p` false. Ban `q`: W gives `p` true. A restriction on a negation program has derived an atom the unrestricted program doesn't. **Law (spec):** restrictions on W-programs are reported as status changes against the unrestricted run (each atom whose status moved). They are never applied silently. This is exactly the shear warning: a pact that changes meaning without a receipt.

**What pacts carry (policy, not meaning):** ~~the list as first written~~ *[S 3, 11; T]* corrected:
- budgets: as **truncation** (`RestrictedRun.truncated`), never as bans;
- functionality findings;
- frequency (the Orbit from deposits, F5);
- error recovery (which hole to report).

**Measurement** (Resolution) is a third bin. It is recorded beside readings, and never acts on E's model.

**Removed from the policy list:** "the choice among ambiguous parses in `split` grammars" and kintsugi's "which one?". On a non-functional program, a ban that leaves one of several parses picks **which tree N is** (Seam's fixture: `S <- A B ; A <- 'a' | 'a' 'a' ; B <- 'a' 'b' | 'b'` on `aab`). That is a **Choice**: meaning, a `settle` recorded in N (Reed's lean D2/D6: clause-side). RR3 is true of atoms, and of meaning only on **Ford-functional** programs, so functionality is checked first.

---

## §6 Bootstrap

~~**Proposition 6.1** … `N_on = N_off` iff the pacts' erasure on this parse is empty.~~ *[S 4]* **REFUTED in both directions.**
- (⇒) A ban on an item outside the start atom's dependency cone (a rule the start never uses) makes the erasure non-empty and leaves N unchanged. My proof conflated the model with N.
- (⇐) A budget truncates the run with an empty erasure and loses part of N.

**Proposition 6.1′ (PROVED).** If the erasure is disjoint from the start cone and no budget truncated the run, then `N_on = N_off`, and `addr(N_on) = addr(N_off)`.
*Proof.* The start cone is closed under premises: every derivation of a cone atom uses only cone atoms (fixpoint math §5.1, first-premise evaluation). So bans outside the cone can't change any cone atom, and without truncation the cone's derivations complete. N is read off the cone. ∎
The converse doesn't hold, so the test checks the sufficient condition and the observer contract separately (spec RU8a/b).

**Answer.** The seed does not need to include the pacts. T1's stage 2 at k = 1 must hold **with the pacts on**, and the test asserts that their erasure on `mirror.mirror` is empty and nothing was truncated: observers that took nothing (the stricter contract, RU8a). Separately, a ban inside the cone must change `root` and one outside must not (RU8b). A non-empty erasure means a pact changed the parser's reading of itself, and stage 2 is red. The observation pacts are shards too. They are parsed by the same `P₁`, and their own addresses are listed in the record (`shaped_by`).

---

## §7 Today's rulers, mapped

*[T]* **Superseded by Taut's inventory:** 81 rows; 40 disguised, all in code; only 5 of 64 code rulers record their own identity beside their outcome; 6 of 81 have a route across. The family assignment is Taut's §4 table (Budget, Choice, Resolution, Equivalence, Weighting, Authority, Identity roots), adopted in spec §1. The provisional table below is kept for the record.

From fixpoint spec §10 and this week's specs:

| ruler | kind | where | reader dependence |
|---|---|---|---|
| commutator thresholds 1e-12, 1e-6 | threshold | prism | independent |
| Refract cut-off 1e-12 | threshold | prism | independent |
| settle `Noise` | threshold | prism | independent |
| Kuramoto K, dt, omega_scale | parameter (threshold) | prism `rotation` | independent |
| rotation tick budget | budget | prism | independent |
| stage-iteration `k_max` | budget | compile §3 | independent |
| E's budget (`Budget`) | budget | engine | independent |
| the `active` quantizer, operator, bit split | quantizer | uuid (system-wide) | independent |
| prism `Precision` | threshold | prism | independent |
| JSON Unicode normalization (NFC) | choice | `μ_json` | independent |
| number type per μ position | choice | compile §9.1 | independent |
| `@mirror/gate` tolerance | tolerance | `mirror.spec` | independent |
| `@mirror/convergence/auto` | choice (auto-approval) | convergence §1 | independent |
| similar-offer radius and metric | threshold | convergence §4 | **dependent** (reader = the fragment asking) |
| `Futures` weighting | weight | settle §4 | declared per ruler (the coda's comma) |
| functional declarations | ban (finding) | engine | independent |

Every one of these is a bare number or an undeclared choice in today's code or specs. The spec makes each a `Ruler`.

---

## §8 The fold: families, lattices, sealed rulers

### §8.1 Sealed rulers lift nothing (PROVED)

Let every restricting ruler be a positive program `R_P` evaluated by E over the snapshot: `B_t = lfp(R_P, M_t)` restricted to a ban predicate. Positive programs are monotone in their input, and `M_t` is increasing (`T^B` is inflationary), so `B_t ⊆ B_{t+1}`. A ban, once present, stays: **nothing is lifted**, and RR4 applies when there are no late bans. Lifted bans need a non-monotone R, i.e. an open trait (decision D3).

### §8.2 Lattice families have no holonomy (PROVED)

Budget (`min`), Resolution (coarsest band within one scale law), Authority (`∧`) and Equivalence (partition meet) each compose by an operation that is associative, commutative and idempotent: a semilattice. Two consequences:
- (a) Every finite set of rulers in such a family has a composite (their meet), which is a lawful booking of every comma among them. No disagreement is structurally unbookable.
- (b) Holonomy needs `g_e = R_head · R_tail⁻¹`, i.e. inverses. In a meet-semilattice with identity e (its top), only e is invertible: if `x ∧ y = e`, then `e = x ∧ y ≤ x`, so `x = e`. So for these families b₁ measures nothing, and `Polycentric` means nothing structural. Multi-truth among them is only `Kept`: a declared refusal to compose, used two-sidedly.

Holonomy needs invertible adjustments: reorderings in the **Choice** family (permutations form a group), and `Temperament` (settle §4).

*Remark (Equivalence).* Partitions do have a lawful composite (the common refinement), but using it changes identity (addresses). Fixpoint spec §10 already rules that "the same meaning settled by two rulers gives two results", so in practice Equivalence forks. That is a design choice (D6), and READING here.

*Resolution across scale laws.* The coarsest-band composite is defined only within one `ScaleLaw`. Across scale laws (Taut's six numerical zeros use three conventions) a common scale is OPEN, so they fork.

### §8.3 Shape from capacity and use (folded R6)

- **Capacity:** a check edge carries a possible disagreement only if its rulers have different addresses and readings that can differ. Three copies of one ruler in a triangle have b₁ = 1 and zero capacity. Seam: they must not report `Polycentric`.
- **Use:** a comma declared `LeaveOpen` but used one-sidedly (every result over its atoms is shaped by one side) is booked in effect. `Kept` is computed from use.
- **Untested:** a fractal with no comma yet reports `Untested`, never `Hierarchy` (which would hold vacuously).
- **Sideways:** booking to a sibling only as a recorded act approved by both parties (D4 lean).

### §8.4 R7, one level up (folded)

A ruler that obeys an observer's findings is ruled by that observer (Seam: "an observer whose findings a ruler obeys is a ruler"). So an approved `Restrict` names, in its approved content, the observers it reads, and their addresses join `shaped_by`/`observed_by`. Findings from unnamed observers are invisible to it. Findings live outside `ModelRoot`, so they can't steer through the snapshot one tick later. Observers are pure and total: a fault is a reported finding, never a truncation without a ruler.

### §8.5 Choices compose in order (PROVED by Seam's fixture 7)

Two parses x, y. Choice A keeps the least name, B the heavier. Uniting their bans gives a hole. A then B gives x; B then A gives y. Choice composition is ordered, so a fractal's Choice rulers carry an explicit order and apply in sequence. The ban-union mechanism (RR) is correct only for a single Choice, or for the Budget-as-truncation case.

## §9 Decisions pending Alex (Reed's lean; the spec works either way)

D1 approval non-transitive across versions; D2 Choice is meaning (clause-side, picking is a settle); D3 sealed traits; D4 sideways booking only as a recorded act approved by both; D5 "pact" stays with the clause shards and the engine altitude, and the fractal of rulers gets a new name (placeholder `RulerFractal`); D6 Choice and Equivalence clause-side; D7 the gate's own checks. Each, with its alternative, is in spec §5.

**D2 and D6 are decided by §10** (Alex, 2026-10-29, saying yes to Reed's answer): picking a Choice is a temperament act (clause-side, a settle with its erasure), and the un-picked structure is an Intonation. D1, D3, D4, D5 and D7 stay open.

---

## §10 Temperaments and intonations

**The question (Reed, after §8.2).** Every lattice family always has a lawful single-truth composite, and with Choice moved clause-side the ruler fractal can never be structurally polycentric. Should mirror *guarantee* multi-truth, or *protect* the right to keep a comma open?

**Alex:** "A single-truth is what makes it a useful CI pipeline. A multi-truth ought to be protected and may never be collapsed into a single-truth ruler. Basically single-truth rulers are scalar metrics. Multi-truth rulers are … whatever they are, you tell me."

**Reed's answer (Alex: yes), made exact.**

### §10.1 Definitions

**Definition 10.1 (temperament).** A ruler whose readings land in a meet-semilattice with a lawful composite: the lattice families of §8.2. When it is **totally ordered** (a Budget, a single Resolution verdict, a gate's pass/fail), it is a **scalar metric**. A temperament reads each observer's state into one shared value space and composes. That is what makes a CI gate decidable.

**Definition 10.2 (intonation).** A **connection** on the observer graph:
- a frame for each observer;
- for each line `u — v`, an invertible transport `τ_{uv} ∈ Γ` ("how I get from your reading to mine"), with `τ_{vu} = τ_{uv}⁻¹`;
- Γ an exact group (`LoopGroup`, settle §1.1: ratios, renamings, `Z<N>`).

It assigns no value to any single reading. It relates frames and never picks one. Around a loop the transports compose to the holonomy, and where that isn't the identity, the comma is the content. Musically: just intonation keeps the transports exact; equal temperament lays a scalar grid over them.

**Definition 10.3 (vector ruler).** A ruler with values in a product of totally ordered scales, `ℚ^k` under the componentwise order: several criteria. It is a lattice (componentwise min), so it is a temperament family in the §8.2 sense. But it is not totally ordered, so its maxima form an antichain, the Pareto front.

### §10.2 No lawful collapse exists (PROVED)

**Proposition 10.4.** Every monoid homomorphism `h : Γ → L` from a group into a meet-semilattice with top `e` is constant at `e`.
*Proof.* For `g ∈ Γ`: `e = h(1) = h(g · g⁻¹) = h(g) ∧ h(g⁻¹) ≤ h(g)`, so `h(g) = e` (§8.2). ∎

**Corollary 10.5 ("never collapsed" is a theorem, not only a policy).** There is no structure-preserving map from an intonation to a temperament other than the trivial one, which forgets everything. Every non-trivial collapse fails to be a homomorphism. That is, it is a **choice**: non-injective and non-structural, a `settle` that needs a named ruler and returns an erasure. Alex's "may never be collapsed into a single-truth ruler" is enforced by algebra, before any type. The type law below only removes the non-lawful shortcuts (`From`, `Into`, summaries).

**Proposition 10.6 (what tempering erases; PROVED for abelian Γ ⊆ ℝ).** For Γ abelian and real-valued (log-ratios), choosing a scalar reading per observer means choosing a potential φ. The tempered transports are `φ(v) − φ(u)`, and the least-squares choice is the gradient part of the Hodge decomposition. What is lost is exactly the harmonic part, the comma, spread equally around each loop (the coda's result: equal temperament is the harmonic projection). For non-abelian Γ there is no Hodge decomposition, and the erasure is the holonomy representation itself (OPEN: a canonical erasure format beyond "the words around a cycle basis").

### §10.3 Is an intonation exactly a connection whose holonomy is the ruler-free third? NARROWED

- **PROVED (constant Γ, connected graph):** the gauge class of an intonation (connections modulo re-choosing frames) corresponds one-to-one with its holonomy representation `π₁ → Γ` modulo conjugation (kintsugi math Prop 3.5). So the **gauge-invariant content** of an intonation is exactly the coupling's ruler-free third (coupling Prop 2.4, spec `RulerFree::Classes`).
- **REFUTED as "exactly":**
  - an intonation carries more than its gauge class. Its transports themselves are real content ("how to get from your reading to mine"), even where they are gauge-trivial. On one wire (a tree), an intonation has no ruler-free third, and it still relates two frames. Echo and Narcissus have an intonation; it just holds nothing no ruler can absorb;
  - for non-constant Γ (different groups per line), the invariants are the non-abelian H¹ class, which can be non-trivial on a tree (Seam's tree twist), so not holonomy alone.
- **Exact statement:** an intonation is a connection. Its ruler-free third is its gauge class, which is its holonomy for constant Γ and its H¹ class in general. Its gauge-dependent part is the translation between frames: content, but bookable to a ruler.

### §10.4 The middle case: vector rulers (PROVED)

- A vector ruler is a lattice, so it has no holonomy (§8.2): it is never ruler-free.
- Its Pareto front is an antichain, so it is **plural but commensurable**: several incomparable maxima, each with a value in a shared space.
- It collapses to a scalar exactly when it is composed with a weighting `w : ℚ^k → ℚ` (a linear functional with positive weights), which totally orders the front. Distinct front points can then tie or be ordered, and either way the front's incomparability is erased.
- So a vector ruler is protected (`Kept`) **while unweighted**. A weighting is a Choice, i.e. a temperament act: declared, signed, with the erased front as its erasure.

### §10.5 Consistency with the earlier specs (PROVED, by inspection of the definitions)

- **`mirror-settle.md`:** `Loop`, `Holonomy` and `LoopGroup` are the intonation's machinery unchanged. `Temperament { kernel, ruler }` (settle §4) is a temperament of a ratio intonation: quotient by commas, with the kernel as erasure. "Never temper" (L3) reads precisely as **never temper without a named ruler and a returned erasure**, which settle Prop C.6 already said.
- **`mirror-coupling.md`:** `RulerFree` is the gauge class of the coupling's intonation (§10.3). `Thirds.order` (commutator words) is the intonation's non-abelian content. `Offered` strength is a temperament offered to the parties, not reported about them (C5), consistent with "the gate reads temperaments".
- **Convergence CI** (`decision.md`, "Hyped: Convergence CI"): the verdict is single-truth (temperaments: the gate), and the kept commas between voices ride alongside as "a question, not a red build". §10 is the type-level form of that sentence.

### §10.6 Ledger additions

| claim | status |
|---|---|
| 10.4 every homomorphism group → meet-semilattice is trivial | PROVED |
| 10.5 every non-trivial collapse of an intonation is a choice (a settle) | PROVED |
| 10.6 tempering erases the harmonic part (abelian real Γ) | PROVED |
| a canonical erasure for non-abelian tempering | OPEN |
| intonation = connection; its gauge class = the ruler-free third (constant Γ) | PROVED |
| "intonation is exactly its ruler-free third" | REFUTED (gauge-dependent transports; tree twist for non-constant Γ) |
| vector rulers: lattice, no holonomy, plural while unweighted, collapsed by a weighting (a Choice) | PROVED |
| D2, D6 decided: picking is temperament, the un-picked structure is the intonation | decided (Alex) |

---

## §11 A ruler is a prism; the distinction moves to the codomain

**Alex:** "Isn't a `Ruler` a `Prism`, and then the whole distinction dissolves? A single-truth ruler settles to a single scalar value. A multi-truth ruler to a field."

### §11.1 What prism's `Prism` is (read at the cache rev `7e6b00f`)

`prismqueer/src/lib.rs` defines `trait Prism` with **three** operations, `focus`, `project`, `settle`, typed as a chain of beams (`Input → Focused → Projected → Refracted`). The final beam carries an `Imperfect`: `Success(out)`, `Partial(out, loss)` or `Failure(err, loss)`.
- The **five** ops (`focus`, `project`, `split`, `shift`, `settle`) are mirror's shard-side declaration form (`prism @X { … }`), not the Rust trait.
- The pinned rev `26ed54b` isn't in my local cache. Taut's inventory reports only line shifts between the two revs, so the trait shape is taken from `7e6b00f`.
- One known tension: prism's docstring calls `project` "the lossy transformation", while fixpoint math §4.4 says only `settle` is non-injective (decision 12, owner prism-Reed). Below, all narrowing is placed in `settle`, which is consistent with mirror's math.

### §11.2 The unification (PROVED as definitions line up)

**Definition 11.1.** A **ruler** is a prism, i.e. a declared, signed `settle` (after `focus` and `project`), whose settled value is a **field** `Field<B, V>`: a value in V on each cell of a base B.
- **Scalar:** `Field<Point, V>`, a field over one point (K₁, the decree).
- **Intonation:** `Field<Γ-graph, G>` with G a group: a transport per line, i.e. a connection.
- **Lattice family ruler:** `Field<B, L>` with L a meet-semilattice.
- **Vector ruler:** `Field<Discrete(k), L>`, k criteria with no lines between them (no transports: the criteria are incommensurable axes).

Every family specified so far settles to one of these (Budget, Resolution, Authority: a point or a lattice field; Choice: a point, picked from held alternatives; Equivalence: a partition, a lattice value; Intonation: a group field). So "a ruler is a prism" holds for everything in spec §1 and §6. **Fifty dissolves on the name.** An intonation *settles*: it fixes which transports hold, out of all possible connections, and that narrows **possibilities**, not readings. Scarlet's "it doesn't narrow to a value" and Mara's "it is a ruler" are both true. The definition of a ruler moves from "narrows to a value" to "settles, to a field".

### §11.3 What does **not** dissolve: the codomain's algebra (PROVED)

The single/multi distinction relocates to the settled field. There are four cases, by base and value algebra:

1. **A field on a point.** Nothing to collapse.
2. **A lattice-valued field on any base.** The meet `∧ : L^B → L` is a lattice homomorphism, so a **lawful collapse to a point exists**.
   - If several exist (for a vector ruler: `min` over criteria, or any single projection), choosing one is a **Choice**, a temperament act.
   - This is exactly §8.2: single truth is always lawfully available here.
3. **A group-valued flat field** (trivial holonomy on every loop). Its gauge class is a single point (kintsugi Prop 3.5: flat on a connected graph ⇒ gauge-trivial), so it collapses **losslessly up to gauge**. The gauge-dependent transports (who translates how) are lost, and they are bookable to a ruler (§10.3). **"Temperament = a flat field"** holds in this sense: equal temperament is the flat field closest to the just one.
4. **A group-valued curved field** (some holonomy ≠ identity). **No lawful collapse exists.** Every monoid homomorphism from the group of such fields `G^E` into a meet-semilattice is constant at the top (Prop 10.4 applied to `G^E`). The only way down is `temper`: a declared settle onto flat fields or a point, whose erasure is the curvature.

So Alex's "the whole distinction dissolves" is **NARROWED**:
- the distinction between *kinds of ruler* dissolves: one trait;
- the distinction between *kinds of settled value* does not: curved group-fields (case 4) are the multi-truth that is never collapsed, by a theorem.

**"A scalar is a field over a single point" (PROVED, definitional).** The real parameters are then:
- the base: a point, or a graph with b₁ ≥ 0;
- the value algebra: lattice or group;
- for groups, the flatness.

### §11.4 `temper` versus prism's `settle` (PROVED, Hodge)

For abelian real G and a graph without 2-cells, the 1-cochains split orthogonally into gradient ⊕ harmonic (Lim, *Hodge Laplacians on Graphs*: edge flows converge to the harmonic projection `P_{H¹} u₀`).
- prism's spectral `settle` (`spectral/settle.rs`) runs the Laplacian flow and reports a `Tail { moving, faded, opaque }` against a `Noise` resolution. Its scope is vertices only (`settle.rs:17`). *[prism-Reed, prism-005, 2026-10-29]* But the edge flow already exists elsewhere in prism: `Instrument` works on C⁰ ⊕ C¹ (the Hodge–Dirac operator), and `Instrument::decay` **keeps** H¹, i.e. the harmonic part, and fades the rest (Lim's edge flow).
- `temper` does the **complement**: it keeps the gradient part (a flat field, potentials) and returns the harmonic part as the erasure (Prop 10.6).
- So `temper` and the edge-Laplacian settle are complementary orthogonal projections. A curved field equals its tempered flat field plus the curvature the edge flow would keep. Nothing is lost between them, and **what temper erases is exactly what an edge-Laplacian settle keeps** (abelian real case). ~~prism doesn't have that settle yet.~~ **The measuring dual of `temper` is `Instrument::decay`;** prism-005 step D makes the Hodge split explicit.
- For non-abelian G there is no orthogonal split, and the erasure is the holonomy words (OPEN: a canonical form).

### §11.5 Against everything already specified

- **Taut/Seam families:** the lattice families are exactly the prisms whose settled value is lattice-valued (cases 1–2). Choice settles to a point picked from held alternatives. The held, un-picked alternatives (an intonation over the choice space, or a Front) are case 2 or 4, depending on whether the alternatives carry invertible transports. **PROVED** by matching the definitions in spec §1.
- **`mirror-settle` `Loop` / `Holonomy` / `Temperament`:** `Loop` and `Holonomy` measure case 4's curvature. `Temperament { kernel, ruler }` is `temper` for a ratio-valued field. Consistent.
- **Coupling `RulerFree`:** the gauge class of the coupling's field (§10.3). It is a point in case 3 (`Absorbable`) and non-trivial in case 4 (`Classes`). Consistent.
- **Prop 10.6:** restated as §11.4. Consistent.
- **D-decisions:** D2/D6 (picking is temperament) are unchanged: picking is the settle from case 2 or 4 to case 1. D1, D3, D4, D5 and D7 are untouched.

### §11.6 READING: a spin field on a lattice

In physics, a spin configuration on a lattice is a field over a graph base. A lattice gauge field is a connection on links, with plaquette holonomy: Wilson's 1974 construction quantizes "a gauge field theory on a discrete lattice … preserving exact gauge invariance", and Wilson loops are its holonomies (Wilson, *Confinement of quarks*, Phys. Rev. D 10, 2445; Wikipedia, *Wilson loop*; via Kagi). Reed relates this shape to the spin field in Anna Jakobs's thesis. That is READING, about the shape only, not a claim the thesis makes, and I haven't read the thesis.

### §11.7 Ledger additions

| claim | status |
|---|---|
| prism's Rust `Prism` has three ops; the five are mirror's declaration form | PROVED (read, `7e6b00f`) |
| every specified ruler is a prism settling to a field | PROVED (definitions) |
| "Fifty" (is an intonation a ruler?) | dissolved: a ruler settles, possibly to a field |
| lattice-valued fields collapse lawfully (meet); choosing among collapses is a Choice | PROVED |
| flat group-fields collapse losslessly up to gauge | PROVED |
| curved group-fields have no lawful collapse (Prop 10.4 on `G^E`) | PROVED |
| "the whole distinction dissolves" | NARROWED: one trait; the codomain's algebra and curvature remain |
| `temper` and an edge-Laplacian settle are complementary Hodge projections (abelian real) | PROVED; the dual is `Instrument::decay` (prism-005 step D makes the split explicit) |
| canonical non-abelian erasure | OPEN |
| spin fields / lattice gauge fields as the same shape | READING (cited) |

## Ledger

| claim | status |
|---|---|
| 1.2 the Mara/Scarlet comma held by a declared reader dependence | PROVED |
| 2.1 fractal of rulers | definition. The `@epistemologic/pact` READING is withdrawn: those shards are clauses *[S 12]* |
| 2.2 tree ⇒ bookable (constant group, surjective independent adjustments); b₁ ≥ 1 ⇒ some unbookable | PROVED |
| tree ⇒ bookable without surjectivity | REFUTED (Seam's frozen tree) *[S 6]* |
| "tree of rulers = single truth" | NARROWED: tree + upward booking + surjectivity = single truth; tree alone REFUTED (a kept comma) |
| 4.2 every approval chain ends in a sink or a governance cycle | PROVED |
| sink = Hart's rule of recognition; cycle = self-amendment | READING (cited) |
| a human in every governance cycle blocks self-erasure | REFUTED (Seam's G₀→G₁→G₂) *[S 7]* |
| RR1 determinism per strategy; attribution as a set of rulers | PROVED (folded) |
| RR2 semi-naive = naive for fixed or monotone bans | PROVED; FAILS for non-monotone (lifted bans) *[S 1]* |
| RR3 `M_R ⊆ lfp`; meaning preserved on Ford-functional programs | PROVED; on non-functional programs a ban is a Choice *[S 3]* |
| RR4 monotone, no late ban ⇒ `lfp(T^{B∞})` | PROVED (Seam 1433/1433) |
| RR5 late bans **and lifted bans** reported | PROVED (folded) |
| RR6 recovers the model, not the trajectory | PROVED (Seam 4000/4000) |
| W-program bans can change meaning | PROVED (example) |
| 6.1 "iff empty erasure" | REFUTED both ways *[S 4]* |
| 6.1′ disjoint from the start cone + no truncation ⇒ `N_on = N_off` | PROVED |
| §8.1 sealed rulers lift nothing | PROVED |
| §8.2 lattice families have no holonomy; `Polycentric` meaningless there | PROVED |
| §8.5 Choice composes in order, not by union | PROVED (fixture) |
| Resolution composite across scale laws | OPEN |
| §7 mapping | superseded by Taut's inventory and spec §1 |
| the Anna Jakobs link | READING (thesis not read) |

## Sources (Kagi, this session)

- K. G. Wilson, *Confinement of quarks*, Phys. Rev. D 10 (1974) 2445, https://link.aps.org/doi/10.1103/PhysRevD.10.2445; Wikipedia, *Wilson loop*. **[abstract and snippet]**
- L.-H. Lim, *Hodge Laplacians on Graphs*, SIAM Review 2020, https://www.stat.uchicago.edu/~lekheng/work/psapm.pdf (edge flow converges to `P_{H¹} u₀`). **[snippet; also read in the 10-10 session]**

- Wikipedia, *Rule of recognition*, https://en.wikipedia.org/wiki/Rule_of_Recognition; Legal Theory Lexicon 105, https://legaltheorylexicon.com/2026/04/legal-theory-lexicon-105-the-rule-of-recognition/ (H. L. A. Hart, *The Concept of Law*, 1961). **[snippets]**
- E. Ostrom, design principles ("multiple layers of nested enterprises"), via https://beyondostrom.blog.rosalux.de/files/2013/05/Design-Principles-and-Threats-to-Sustainable-Organizations-That-Manage-Commons.pdf; *Beyond Markets and States: Polycentric Governance of Complex Economic Systems*, https://web.pdx.edu/~nwallace/EHP/OstromPolyGov.pdf. **[snippets]**
- A. Ross, *On Self-Reference and a Puzzle in Constitutional Law*, Mind 78 (1969), https://www.jstor.org/stable/2252308; P. Suber, *The Paradox of Self-Amendment*, §5, Harvard DASH. **[snippets]**
