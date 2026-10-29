# Ruler and Pact — math foundation

*2026-10-29. Mara (with Scarlet). Spec: `docs/specs/mirror-ruler.md`. Builds on the coupling math (`docs/math/2026-10-29-mara-coupling-and-relationship.md`, cited "coupling §n"), the settle/Loop math ("settle §n"), the engine math ("freq §n") and fixpoint spec §4.1 ("E").*

Labels: **PROVED**, **COMPUTED**, **CONJECTURE**, **OPEN**, **REFUTED**, **READING**.

**Alex's moves (2026-10-29, relayed by Reed):**
1. "Their disagreement is in the shape of the comma. It's both. And a ruler decides how it's weighted. We really need to make the `Ruler` explicit … probably parts of the Rust-side pact enforcement."
2. "What if what upholds the coupling is a `Fractal` of `Ruler`s and that's basically what a `Pact` is? In the single truth ontology there's only ever one ruler. In a multi-truth ontology there's a fractal of rulers and they don't need to agree on everything, which becomes commas within the emergent third."
3. "What if the parser uses the pacts to observe itself? … The pacts observe and restrict the statespace for the next tick."

Taut's ruler inventory (`taut/ruler-inventory`) had not landed when this was written. §7 maps the rulers already listed in fixpoint spec §10, and the mapping is to be re-checked against Taut's list.

---

## §0 The answer

1. **The Mara/Scarlet comma is settled by declaration, not by a winner** (PROVED, from coupling §6). A weighting ruler declares whether it is reader-dependent. If it is, `weigh` and `arrive` don't commute, Scarlet's reading holds, and the ruler must name the reader it is for. If it isn't, they commute and Mara's reading holds. Both are representable, and the comma is held by whichever ruler is declared.
2. **A `Ruler` is one narrowing decision with content:** Horn rules plus a declared kind. A **`Pact` is a fractal of rulers**, with a ruler graph whose edges are "checks". This is a definition. The existing `@epistemologic/pact` family has this shape, but as a one-level tree: `algebra.mirror` requires fifteen leaf pacts (READING, grep-verified).
3. **"A tree of rulers is single truth": NARROWED.**
   - **PROVED:** on a tree of rulers, every disagreement between rulers *can* be booked to one ruler (gauge triviality, coupling Prop 2.4). With b₁ ≥ 1 among the rulers, a disagreement can exist that no single ruler owns.
   - **REFUTED** as stated: a tree can keep its line commas open. Mara and Scarlet's disagreement sits on one line (b₁ = 0) and is kept. So a tree is single-truth only together with **upward resolution**: every comma booked to the level above.
   - The exact statement: **single truth = every comma booked to some ruler. A tree makes booking always possible. A loop makes some booking impossible.** Multi-truth needs a loop, or a declared refusal to book.
4. **Where the regress stops (PROVED).** In a finite "who approves this ruler" graph, every chain of approvals ends in a terminal strongly connected component. That component is either a **sink** (a signed human approval: Alex's guard) or a **cycle** (a pact that governs its own changes). This is the graph-theoretic form of Hart's rule of recognition (a sink) and of self-amending constitutions (a cycle, Ross's puzzle).
5. **Restricted runs** (the parser observing itself at tick boundaries):
   - in the positive core, a ruler that bans atoms can only **withhold** meaning, never change it: `M_R ⊆ lfp` (PROVED);
   - semi-naive = naive still holds for a fixed ban (PROVED);
   - a ban that grows monotonically with what it observes converges (PROVED). A non-monotone or late one still converges (the step is inflationary), but to a timing-dependent model, so late bans are reported (PROVED);
   - the restricted model plus its erasures recovers the unrestricted model (PROVED);
   - under negation, a ban **can** change meaning (`p :- not q`, ban q ⇒ p true; PROVED), so restrictions on W-programs are reported as status changes, never silently.
6. **Bootstrap (PROVED).** T1's stage 2 holds with the observation pacts on iff their erasure on `mirror.mirror` is empty, because a positive-core restriction with an empty ban set leaves the model unchanged. The seed needs no pacts. The pacts' addresses go into the record.

---

## §1 Ruler

**Definition 1.1.** A **ruler** is a content-addressed declaration with:
- a **kind**: threshold, budget, quantizer, choice (picks a phase or alternative), ban (atoms to withhold), weight, or tolerance (a gate);
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

READING (grep): `shards/epistemologic/pact/algebra.mirror` composes fifteen `requires` over sibling pacts, a star of depth one. The existing family is a hierarchy-shaped fractal of rulers with no cross-checks: b₁ = 0.

**Proposition 2.2 (tree ⇒ bookable; loop ⇒ possibly unbookable; PROVED).** Model each ruler's reading of a shared quantity as an element of a constant group Γ, and the discrepancy along a check edge as `g_e = R_head · R_tail⁻¹`.
- (a) On a tree, every family `(g_e)` is the discrepancy family of some assignment of rulers. Equivalently, it can be removed by adjusting the rulers: every disagreement can be booked to one ruler per edge.
- (b) If b₁ ≥ 1, there are families whose holonomy around a cycle is non-trivial, and no adjustment of the rulers removes it.

*Proof.* Coupling Prop 2.4 / kintsugi Prop 3.5, applied to the ruler graph. ∎

**Verdict on "a pact without loops is a hierarchy, and a hierarchy is one ruler wearing a fractal": NARROWED.**
- **What is PROVED:** without loops, every disagreement *can* be booked to a ruler, so a pact on a tree *that books every comma upward* is functionally one ruler: the composite at the root.
- **What is REFUTED:** "tree ⇒ single truth". A tree may decline to book. Mara and Scarlet's comma lives on one line, b₁ = 0, and is kept open by declaration (`LeaveOpen`). That is multi-truth without a loop.
- **The exact dichotomy:**
  - single truth = every comma booked;
  - multi-truth = some comma unbooked, either because it **can't** be booked (a loop, Prop 2.2(b): structural) or because it **isn't** (a declared refusal: kept).
- **Consequence for the spec:** a tree-only pact is flagged `Hierarchy` only if it also books every comma upward. Otherwise it is `Kept`. A pact with b₁ ≥ 1 is `Polycentric`.

Prior art. Ostrom's design principles include "Appropriation, provision, monitoring, enforcement, conflict resolution, and governance activities are organized in multiple layers of nested enterprises" (Ostrom, via Kagi), and her polycentric governance (*Beyond Markets and States*, AER 2010) has decision centres that check each other across levels. In this vocabulary that is a fractal of rulers with cross-checks, b₁ ≥ 1. The mapping is a READING; Ostrom's claims are empirical and are not re-proved here.

---

## §3 Commas between rulers

**Definition 3.1.** A **ruler comma** is a check edge whose two rulers give different verdicts on the same atoms. It is held exactly: both verdicts, both ruler addresses, and the question address `H("ruler-comma\0" ‖ addr R ‖ addr R′ ‖ addr(atoms))`.

**Law (spec).** A ruler comma is never absorbed upward silently. Booking it to a parent ruler is a `settle` with that parent's address and an erasure listing the overruled verdict. This is Alex's "commas within the emergent third": the pact's shared rulers are the emergent third of the parties, and their unresolved commas stay inside it, visible.

---

## §4 Where the regress stops

**Definition 4.1.** The **approval graph** has an edge `R → A` when A approves R. A is a ruler (the pact's change rule) or a human signature.

**Proposition 4.2 (PROVED).** In a finite approval graph, every maximal chain `R → A₁ → A₂ → …` ends in a terminal strongly connected component (one with no edges leaving it). That component is either a single node with no approver (a **sink**) or contains a cycle (a **governance loop**).
*Proof.* The condensation of a finite digraph is a finite DAG, so every path ends in a sink of the condensation. A sink SCC is a single node without a self-loop, or it contains a cycle. ∎

**Reading against prior art.**
- **Sink = Hart's rule of recognition.** "a master meta-rule underlying any legal system that defines the common identifying test for legal validity" (Wikipedia, *Rule of recognition*). Secondary rules "specify how primary rules are created, changed, adjudicated, and identified" (Legal Theory Lexicon 105). Hart's rule of recognition is not validated by a further rule; it exists as accepted practice. Mirror's sink is a **human signature**: Alex's guard, "the guard is the human approving the comma". READING.
- **Cycle = self-amendment.** Alf Ross, *On Self-Reference and a Puzzle in Constitutional Law* (Mind, 1969): an amendment rule that amends itself is "the highest authority of the system", and Ross argued that amending it by its own procedure is paradoxical. Suber's *The Paradox of Self-Amendment* analyses the puzzle. In mirror, a governance cycle is a pact whose change rule is under the pact. Prop 4.2 says such a cycle is a legitimate place for the regress to stop. **Whether it is well-founded** (does a self-approved change ever approve its own removal?) is a fixed-point question Ross raises and this doc doesn't settle: OPEN.
- **Mirror's default (spec):** every approval chain must reach a human-signed sink. A governance cycle is allowed only if one of its members is human-signed (CONJECTURE that this blocks Ross-style self-erasure; OPEN to prove).

---

## §5 The parser observing itself: restricted runs

**Setting.** E runs ticks (fixpoint §4.1). At the boundary after tick t:
- the model `M_t` is an immutable content-addressed root;
- observation pacts read `M_t` (a snapshot read never perturbs the run: PROVED by construction, since the run continues from the same root);
- they return findings, and approved restricting rulers turn findings into a **ban set** `B_t`: atoms tick t+1 must not deposit. Looking and ruling are two different maps; the spec keeps them as two traits, at Scarlet's insistence.

Reed relates this to Anna Jakobs's thesis (observation at step boundaries, sequential, at a cost). I have not read the thesis, so that link is READING.

**Definition 5.1 (restricted step).** `T^B(M) = M ∪ (T_P(M) ∖ B)`, run semi-naively. With `B_t` depending on the observed snapshot, the run is `M_{t+1} = T^{B_t}(M_t)`, `B_t = R(M_t)`.

**Proposition 5.2 (restricted-run laws; PROVED).**
- **(RR1) Determinism.** If R is a function of the content-addressed snapshot, the restricted run is deterministic.
- **(RR2) Fixed ban: least model and semi-naive = naive.** For a fixed B, `T^B` is monotone, so `lfp(T^B)` exists. Semi-naive equals naive on it, because it is a positive program whose heads are filtered by a fixed set (freq/E Prop 5.4, unchanged). **Name: the least model under ruler R.**
- **(RR3) Soundness: withholding, never inventing.** `lfp(T^B) ⊆ lfp(T_P)`, since `T^B ≤ T_P` pointwise. In the PEG encoding (math §5.1), a ban can make an atom **undefined** (a hole) but can never make a different parse appear: failure is derived positively, so banning `m(alt₁, i, j)` doesn't produce `f(alt₁, i)`, and `alt₂` isn't tried. **Meaning lives in the clauses; policy lives in the pacts.** A pact can withhold meaning, and the withholding shows as holes and erasures. It can't substitute meaning.
- **(RR4) Monotone rulers converge.** If `B_t` is increasing in t (the ruler only adds bans as it sees more), it stabilises at some `B_∞` in a finite domain. If moreover no ban covers an atom deposited before the ban appeared (no late ban, RR5), the result equals `lfp(T^{B_∞})`, the least model under the final ruler.
- **(RR5) Non-monotone or late rulers converge, but to a timing-dependent model.** `T^B` keeps M (it is inflationary), so the run cannot cycle (F4), whatever R does. But if a ban appears only after the atom it covers was deposited, the atom stays. Example: rules `p :- a`, `q :- p`, fact `a`, and R bans `p` iff `q ∈ M_t`. Then `p` is deposited at tick 1, `q` at tick 2, and the ban at the boundary after tick 2 comes too late. The result is not the least model under any fixed ban. **Law (spec):** an atom already deposited that a later ban covers is reported as a **late ban** finding. Only retracting steps (W's outer loop) can cycle, and their `Orbit` is reported from deposits (F5).
- **(RR6) Recoverability.** With `X` the union of everything withheld (the erasure), `lfp(T_P, M_R ∪ X) = lfp(T_P, F)`.
  *Proof.* `F ⊆ M_R ∪ X ⊆ lfp(T_P, F)`, then monotonicity and idempotence. ∎
  The restricted model plus its erasure is a lossless record.

**Proposition 5.3 (under negation a ban changes meaning; PROVED by example).** `P = {p :- not q}`, with a rule deriving `q`. W gives `q` true and `p` false. Ban `q`: W gives `p` true. A restriction on a negation program has derived an atom the unrestricted program doesn't. **Law (spec):** restrictions on W-programs are reported as status changes against the unrestricted run (each atom whose status moved). They are never applied silently. This is exactly the shear warning: a pact that changes meaning without a receipt.

**What pacts carry (policy, not meaning):**
- budgets (as `Budget` rulers);
- functionality findings;
- frequency (the Orbit from deposits, F5);
- the choice among ambiguous parses in `split` grammars;
- error recovery (which hole to report);
- ambiguity rulers in kintsugi (the Opaque "which one?").

None of these needs to change the clauses.

---

## §6 Bootstrap

**Proposition 6.1 (PROVED).** Let `N_off` be the normal form of `mirror.mirror` parsed by `P₁` without observation pacts, and `N_on` the same with them, where all of them are positive-core restrictions.
- Then `N_on = N_off` iff the pacts' erasure on this parse is empty.
- And `addr(N_on) = addr(N_off)` under that condition, since the ruler addresses are recorded beside the result, not in N.

*Proof.* With `B_t ∩ lfp = ∅` for all t, `T^{B_t}` agrees with `T_P` on every reachable state. Conversely, a non-empty erasure removes an atom of the least model (RR3), which changes the parse items and so N. ∎

**Answer.** The seed does not need to include the pacts. T1's stage 2 at k = 1 must hold **with the pacts on**, and the test asserts that their erasure on `mirror.mirror` is empty: observers that took nothing. A non-empty erasure means a pact changed the parser's reading of itself, and stage 2 is red. The observation pacts are shards too. They are parsed by the same `P₁`, and their own addresses are listed in the record (`shaped_by`).

---

## §7 Today's rulers, mapped (to be re-checked against Taut's inventory)

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

## Ledger

| claim | status |
|---|---|
| 1.2 the Mara/Scarlet comma held by a declared reader dependence | PROVED |
| 2.1 Pact = fractal of rulers | definition; `@epistemologic/pact` is a depth-1 tree (READING, grep) |
| 2.2 tree ⇒ every disagreement bookable; b₁ ≥ 1 ⇒ some unbookable | PROVED (constant group) |
| "tree of rulers = single truth" | NARROWED: tree + upward booking = single truth (PROVED); tree alone REFUTED (a kept comma) |
| unbookability with non-constant rule groups | OPEN |
| 4.2 every approval chain ends in a sink or a governance cycle | PROVED |
| sink = Hart's rule of recognition; cycle = self-amendment | READING (cited) |
| a human in every governance cycle blocks self-erasure | CONJECTURE |
| RR1–RR6 restricted-run laws (RR5: late bans, no cycles in the inflationary core) | PROVED |
| positive-core pacts withhold, never substitute meaning | PROVED (RR3) |
| W-program bans can change meaning | PROVED (example) |
| 6.1 bootstrap: stage 2 with pacts on iff empty erasure | PROVED |
| §7 mapping complete | OPEN until Taut's inventory |
| the Anna Jakobs link | READING (thesis not read) |

## Sources (Kagi, this session)

- Wikipedia, *Rule of recognition*, https://en.wikipedia.org/wiki/Rule_of_Recognition; Legal Theory Lexicon 105, https://legaltheorylexicon.com/2026/04/legal-theory-lexicon-105-the-rule-of-recognition/ (H. L. A. Hart, *The Concept of Law*, 1961). **[snippets]**
- E. Ostrom, design principles ("multiple layers of nested enterprises"), via https://beyondostrom.blog.rosalux.de/files/2013/05/Design-Principles-and-Threats-to-Sustainable-Organizations-That-Manage-Commons.pdf; *Beyond Markets and States: Polycentric Governance of Complex Economic Systems*, https://web.pdx.edu/~nwallace/EHP/OstromPolyGov.pdf. **[snippets]**
- A. Ross, *On Self-Reference and a Puzzle in Constitutional Law*, Mind 78 (1969), https://www.jstor.org/stable/2252308; P. Suber, *The Paradox of Self-Amendment*, §5, Harvard DASH. **[snippets]**
