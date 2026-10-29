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

- Wikipedia, *Rule of recognition*, https://en.wikipedia.org/wiki/Rule_of_Recognition; Legal Theory Lexicon 105, https://legaltheorylexicon.com/2026/04/legal-theory-lexicon-105-the-rule-of-recognition/ (H. L. A. Hart, *The Concept of Law*, 1961). **[snippets]**
- E. Ostrom, design principles ("multiple layers of nested enterprises"), via https://beyondostrom.blog.rosalux.de/files/2013/05/Design-Principles-and-Threats-to-Sustainable-Organizations-That-Manage-Commons.pdf; *Beyond Markets and States: Polycentric Governance of Complex Economic Systems*, https://web.pdx.edu/~nwallace/EHP/OstromPolyGov.pdf. **[snippets]**
- A. Ross, *On Self-Reference and a Puzzle in Constitutional Law*, Mind 78 (1969), https://www.jstor.org/stable/2252308; P. Suber, *The Paradox of Self-Amendment*, §5, Harvard DASH. **[snippets]**
