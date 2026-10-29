# Crowbar: Ruler and Pact

*2026-10-29. Seam. Target, on `reed/ship`: `docs/math/2026-10-29-mara-ruler-and-pact.md` ("math", `66c009d5`), `docs/specs/mirror-ruler.md` ("spec", `336b5718`), and the essay `docs/essays/2026-10-29-mara-scarlet-ruler.md` (`1844e55d`). Branch `seam/crowbar-ruler`.*

**Method.** `tests/ruler/oracles/ruler_crowbar.py` → `ruler_crowbar.out` contains:
- two independent restricted runners over ground positive programs: *naive*, which recomputes T_P each tick, and *semi-naive*, which joins only rule instances that touch the previous delta;
- bans computed as `R(M_t)` from the snapshot, exactly as math Def 5.1 defines them;
- 4000 random programs with both monotone and non-monotone rulers;
- hand-built fixtures for each counterexample.

Taut's ruler inventory (`docs/scouts/2026-10-29-taut-ruler-inventory.md`, `0ebec336` + `46cf0ef8`) landed while I was writing. §8 folds it in, grading every law against its families.

---

## §0 Verdict table

| law / claim | where | grade |
|---|---|---|
| Prop 1.2: the comma is decided by declared reader dependence | math:45 | **HOLDS**. It is close to definitional: "independent" is defined as acting on independent components |
| Prop 2.2(a): on a tree, every disagreement can be booked | math:66 | **NARROWED** to a constant group. With different groups per edge (Ledger: OPEN), a tree carries an unbookable class (§4.2) |
| R6 / `PactShape` is computed, never declared | spec:85 | **FAILS** as a guard against gaming. b₁ counts *declared* check edges, a redundant triangle buys `Polycentric`, and the booking states aren't exhaustive (§4) |
| Prop 4.2: chains end in a terminal SCC | math:96 | **HOLDS** (finite digraph) |
| R5: a human in the terminal component blocks self-approval | spec:84, math:102 | **FAILS** against Ross. Approval is reachability, so a signature on one version approves every descendant (§5) |
| RR1 determinism | math:118 | **NARROWED**. The model is deterministic *per evaluation strategy* and given pure `Observe`/`Restrict`. The `withheld` attribution depends on pact order when two rulers ban one atom (§1.4) |
| RR2: fixed ban ⇒ monotone, least model, semi-naive = naive | math:119 | **HOLDS** |
| RR3: `M_R ⊆ lfp` | math:120 | **HOLDS** (4000/4000) |
| RR3/R9: "a ban can never make a different parse appear"; "pacts never change meaning in the positive core" | math:120, spec:88 | **NARROWED** to functional programs. On a `split` grammar a Choice ruler, which math §5 lists as policy, picks the tree (§2) |
| RR3, PEG: "a ban yields a hole, never alt₂" (f-atoms bannable) | math:120, spec:115 | **HOLDS** for Ford-functional PEG clauses. Banning an f-atom only makes an atom undefined (§2.1) |
| RR4: monotone ruler, no late ban ⇒ `lfp(T^{B∞})` | math:121 | **HOLDS** (1433/1433, plus proof check §1.2) |
| RR5: non-monotone or late rulers converge to a timing-dependent model; late bans reported | math:122, spec:66 | **FAILS** as a complete report. A *lifted* ban gives a semi-naive model that is not a fixpoint, the naive and semi-naive models differ (134 of 4000 runs), and `late_bans` is empty (§1.1) |
| RR6: `lfp(P, M_R ∪ withheld) = lfp(P, F)` | math:123 | **HOLDS**, including snapshot-dependent and non-monotone bans (4000/4000). The proof needs only `M_R ⊆ lfp` and `withheld ⊆ lfp` |
| Prop 5.3 / RU7: W-bans change meaning, reported as `StatusMove` | math:127 | **HOLDS** |
| Prop 6.1 / R10: `N_on = N_off` iff the erasure is empty | math:143, spec:89 | **FAILS in both directions** (§3) |
| R1 / RU1: no bare narrowing parameter | spec:80,100 | **NARROWED**. It is type-checkable only for the listed types, and its force rests on who holds signing keys, not on types (§6) |
| R7: `Observe` returns findings only and can't ban | spec:50-55,86 | **FAILS** one level up. `Restrict` reads findings, so an unapproved observer steers an approved ruler's bans (§7) |
| R2, R4, R11 | spec:81,83,90 | **UNTESTED** (no code). R4 is sound as stated. R2 should also carry the observers that fed the shaping (§7) |
| Math §7 / spec §3 mapping; the 7 `RulerKind`s | math:153, spec:18,92 | **FAILS** as one type. Taut's 81 rows fall into five or more families with different composition laws. `Ban` is a mechanism, not a kind. `Weight` doesn't narrow. `Threshold` and `Tolerance` straddle families (§8) |
| Math:64 READING: `@epistemologic/pact` is a depth-1 fractal of rulers | math:64, spec:96 | **FAILS**. Those shards are predicates (clauses), not rulers (Taut §2); "Pact" names three things (§8.4) |
| One `Ruler` type for every narrowing | spec:12-20 | **NARROWED** to the envelope (addr, approval, application record). The body laws are per family (§8.2) |

---

## §1 Restricted runs (priority 1)

### §1.1 RR5: a ban that is lifted is a timing dependence nobody reports

Math Def 5.1 runs `T^{B_t}` semi-naively with `B_t = R(M_t)`. `Restrict` is an open trait (spec:55), so nothing makes R monotone.

**Fixture** (`ruler_crowbar.out` §1):
- program: facts `a`; rules `p :- a`, `q :- a`;
- ruler: "ban `p` while `q` is absent".

What happens:
- At tick 1, `p` and `q` are both derivable and `p` is banned (`withheld (p, 1)`).
- At tick 2 the ban is gone, but `p`'s only rule instance uses `a`, which is not in the delta. So semi-naive never re-fires it, the delta is empty, and the run stops.
- **Semi-naive model `{a, q}`; naive model `{a, p, q}`.**
- The final ban set is ∅, so the least model under the final ruler is `{a, p, q}`. The semi-naive result is not a fixpoint of `T^{B_final}`.
- `withheld` attributes `p`'s absence to a ban no longer in force, and `late_bans` is empty.

On random programs, naive ≠ semi-naive in **134 of 4000** runs, every one of them non-monotone.

Consequences:
- RR5's "converge, but to a timing-dependent model" understates it. The model depends on the **evaluation strategy** too: a choice inside `E` becomes meaning-bearing.
- RR2's "semi-naive = naive" is true only for fixed (or monotone) bans.
- `late_bans` catches *ban-after-deposit*. It misses *withhold-then-lift*. **A third finding kind is needed: `lifted (atom, tick withheld, tick lifted, ruler)`.** Alternatively, the run can re-seed the delta with every lifted atom's still-satisfied instances, which makes semi-naive equal naive again.
- The simplest fix: rulers are **E-evaluated positive programs** over the snapshot, and the trait is sealed. Then `B_t` is monotone in `M_t`, so it is increasing, nothing is ever lifted, and only late bans remain. The non-monotone case then needs its own law and its own audit.

### §1.2 RR4 checked

Proof sketch: if `B_t` is increasing with no late ban, every deposited atom lies outside `B_∞` and inside `lfp(T^{B∞})`, by induction. At the stop, `T_P(M*) ∖ B_* ⊆ M*` with `B_* = R(M*) = B_∞`, so `M*` is a fixpoint and therefore contains the least one. That is correct. Computed: 1433/1433.

### §1.3 RR6 holds for snapshot-dependent bans

Both inclusions need only:
- (i) every step adds a subset of `T_P(M_t)`, so `M_R ⊆ lfp`;
- (ii) every withheld atom was in `T_P(M_t)` with `M_t ⊆ lfp`.

Neither uses the shape of R. Computed 4000/4000 with monotone and non-monotone rulers, so `withheld` is complete for recovery in every case I could build.

**Caveat.** RR6 recovers the *model*, not the *trajectory*. The tick at which an atom would have fallen is lost, and with it the frequency/Orbit reading (F5). That doesn't matter for meaning, and it does matter for "lossless record" (math:125).

### §1.4 RR1: who gets the blame

`withheld: Vec<(Addr, Tick, Addr)>` records **one** ruler per withheld atom. When two approved rulers ban the same atom, the recorded ruler is whichever came first in `pacts: &[Approved]`. So the record changes under a permutation of the pact list while the model doesn't. **Fold:** the third component becomes a sorted set of ruler addresses.

RR1 also assumes `R` is a function of the snapshot. `Restrict::restrict(&self, snapshot, findings)` makes it a function of the snapshot **and** the findings (§7), and `&self` permits interior state. So purity must be part of the trait contract.

---

## §2 "Pacts never change meaning" (priority 2)

### §2.1 The PEG claim holds: f-atoms don't flip ordered choice

In the positive PEG encoding (fixpoint math §5.1), the second alternative of `e₁ / e₂` needs `f(e₁, i)`, and `f(e₁, i) ∈ M_R ⊆ lfp` means e₁ failed in the unrestricted run too. So any alt₂ parse in `M_R` is the unrestricted parse. Banning an `f`-atom (for example `f(e₁, i)`, or `f` of a lookahead) can only make something undefined: neither `m` nor `f` at that `(e, i)`.

I tried f-atom bans on choice, `!e`, `&e` and `e*`. Each produced a hole, never a different `m`. **HOLDS, given Ford functionality** (at most one `m(e, i, ·)` in the lfp).

### §2.2 R9 fails where the program is not functional, and the spec invites exactly that

Math §5's list of "what pacts carry (policy, not meaning)" includes **"the choice among ambiguous parses in `split` grammars"**. A `split` grammar is non-functional by design.

**Fixture** (`ruler_crowbar.out` §3):
- grammar: `S <- A B ; A <- 'a' | 'a' 'a' ; B <- 'a' 'b' | 'b'`;
- input: `aab`;
- the lfp holds both parses of S;
- banning `m(A, 0, 2)` leaves `A=a, B=ab`;
- banning `m(A, 0, 1)` leaves `A=aa, B=b`;
- both satisfy `M_R ⊆ lfp`.

RR3's containment holds, and yet the ruler decides **which tree N is**. "Withholding, never inventing" is true of atoms and false of meaning, because meaning is the tree read off the atoms, not the atom set.

It is not only `split`: kintsugi's "ambiguity rulers (the Opaque 'which one?')" are the same thing at shard altitude.

**Fold.**
- R9 becomes: "in the positive core, on a **functional** program, a pact can only withhold". On a non-functional program, a ban that leaves exactly one of several parses is a **choice**: a `settle` whose erasure lists the parses removed, with N carrying that the choice was made (not only `shaped_by` beside it).
- Otherwise two N with equal `root` exist, one per choice ruler, and the root doesn't say which ruler chose. Or, worse, they have different roots and the record doesn't say a pact is why.
- Ford functionality must be **checked** before R9 is claimed (engine-core audit §3.3 already admits the check as a reported defect).

---

## §3 The bootstrap "iff" (priority 3)

Prop 6.1 says `N_on = N_off` iff the pacts' erasure on `mirror.mirror` is empty. `ruler_crowbar.out` §4 refutes both directions.

- **(⇒) fails.** A bottom-up run derives items for every `(e, i)`, not only the start cone. That is why fixpoint spec §4 counts `undefined` "only in the start atom's dependency cone". A ban on an item outside the cone (the fixture bans `f(T, 0)` for a rule `T` that `S` never uses) gives a **non-empty erasure and an identical N**. The math's proof step "a non-empty erasure removes an atom of the least model, which changes the parse items and so N" conflates the model with N.
- **(⇐) fails.** A `Budget` ruler is a `RulerKind`, and math §5 lists budgets among "what pacts carry". A budget that stops the run early truncates the model **without withholding anything**. In the fixture, the erasure is empty and N loses `m(S, 0, 1)`. `RestrictedRun` has no field for truncation.

**Can a pact change the trajectory without withholding?** In the restricted step, no: with `withheld = ∅` at every tick, `T^{B_t}` coincides with `T_P` tick by tick. Only a budget changes it, by stopping, and the findings deposits that `Observe` adds (essay: "a deposit of what was seen") change the **deposit log**, not M.

**For stage 2:** T1 compares `root`, which is N. So the sound statement is: **`N_on = N_off` if the erasure is disjoint from the start cone and no budget truncated the run.** The "only if" doesn't hold.

**What this does to RU8.** Its "aggressive test pact that bans one parse item makes stage 2 red" passes **for the wrong reason** if the banned item is outside the cone: `root` is unchanged, and the test is red only because it also asserts the empty erasure. **Fold:** RU8 asserts two separate things:
- (a) erasure empty: the stricter observer contract, which is right to require;
- (b) a ban **inside** the cone changes `root`, and a ban **outside** the cone doesn't.

And `RestrictedRun` gains `truncated: Option<(Tick, ruler)>`.

---

## §4 Shape (priority 4)

### §4.1 Is the dichotomy exhaustive and checkable?

As a partition of (b₁ = 0 or ≥ 1) × (all booked upward or not), yes. As a description of booking states, no:

- **Sideways booking.** A comma booked to a *sibling* ruler, or to a ruler on another branch, is neither "booked upward" nor "left open". R4 speaks only of "the parent ruler". Either sideways booking is forbidden (then say so), or it is a third state.
- **Vacuous `Hierarchy`.** A pact with zero commas so far is `Hierarchy` ("every comma booked" holds vacuously). The shape is a function of the comma *history*, not of the pact. An untested pact reports single truth. **Fold:** `Hierarchy` requires at least one comma booked upward, or the report says `Untested`.
- **`Polycentric` is bought, not computed.** b₁ is computed, but from `checks: Vec<(Addr, Addr)>`, and the check edges are declared. Three copies of one ruler checking each other in a triangle give b₁ = 1 ⇒ `Polycentric`, with zero commas and holonomy that is identically trivial (`ruler_crowbar.out` §6). The shape should come from **capacity**: whether the check loop can carry a non-trivial holonomy, i.e. whether the rulers on the cycle can disagree at all (their readings aren't constrained to be equal). At minimum, a check edge between rulers with the same `addr` is not an edge.

### §4.2 A tree can be unbookable (the math's OPEN item, closed)

Math Prop 2.2 needs one constant group Γ. Rulers of different kinds read different things (a threshold versus a choice), so the group differs per check edge: the ledger's "unbookability with non-constant rule groups: OPEN".

My compile/kintsugi crowbar §5.1 (`seam/crowbar-compile-kintsugi`, `tests/kintsugi/oracles/seam_crowbar.py`, "Q5") has the counterexample:
- a tree Y₁ — X — Y₂, where X's adjustment acts on both edges at once (the diagonal) and the leaves can't adjust (frozen);
- the discrepancy family (swap, id) has no cycle and is not removable.

In ruler terms: a shared ruler X checked by two party rulers it can't adjust separately. **Prop 2.2(a) and "`Kept` vs `Hierarchy`" both assume adjustments are independent per edge.** Fold: tree ⇒ bookable when every ruler's adjustment restricts surjectively onto each of its edges.

### §4.3 A kept comma that is really booked

`LeaveOpen` is a declaration. A comma declared open while every downstream `Shaped` result on those atoms is shaped by **one** side only (`shaped_by` lists R and never R′) is booked in effect: R′'s verdict is never used.

This is checkable: a comma is genuinely kept iff some result over its atoms carries both verdicts (or the comma's question address), or no result over those atoms exists yet. **Fold:** R6 computes `Kept` from use, not from the `LeaveOpen` flag.

---

## §5 R5 and Ross (priority 5)

R5 checks graph shape: every terminal component reachable from the ruler contains a human signature. Approval is **transitive by reachability**.

**Fixture** (`ruler_crowbar.out` §5):
1. H signs G₀ (a governance pact: "changes need H").
2. G₀ approves G₁, a new version whose content says "changes need G₁".
3. G₁ approves G₂ and a ruler R.

Every chain ends in H's sink, so R5 holds for G₁, G₂ and R. H never saw G₁, and the constraint "changes need H" is gone.

That is Ross's puzzle in its simplest form: the amendment rule amends itself through its own procedure, and the human at the bottom of the chain is a *historical* signature, not a present one.

Putting H inside a governance cycle doesn't help. The cycle `{G, H}` stays terminal while new versions of G hang off it and inherit approval. So the conjecture ("a human in every governance cycle blocks self-erasure", math:102) is **refuted as stated**.

What would block it (Alex decides, since this is governance):
- **Signatures are over addresses and don't propagate across content changes.** A ruler whose content changed needs a human signature on the **new** address whenever it is in, or approves something in, a governance component. That makes approval non-transitive over versions.
- Or, weaker: a changed ruler may approve only rulers whose constraints are **no weaker** than its own. That needs an order on constraints, which the spec doesn't have.

RU4's "a cycle of two rulers with a human signature in it → `Approved`" should gain a red twin: the same cycle, then a version change of one ruler with no new signature → `Unapproved`.

---

## §6 R1 in Rust types (priority 6)

- **RU1 can be a real `compile_fail`.** If `run_bounded` takes `&Approved<'_>`, then `run_bounded(&inst, 100u64)` doesn't compile. Good, but that only covers the listed types (`f64` threshold, `u64` budget, `bool` tolerance). `max_depth: usize`, `timeout: Duration`, `limit: Option<NonZeroU32>` and `retries: u8` are all narrowing parameters that escape the list. "Narrowing parameter" is a semantic class, not a type. **Fold:** R1 is enforced by review plus a grep-able convention: every `pub fn` parameter whose name or doc says limit, budget, threshold, tolerance, depth, timeout, max or min must have type `&Approved<'_>`. Taut's inventory can seed that grep.
- **`Ruler::Exact { value: 1e-9 }` doesn't compile.** The spec says `Ratio`, so the disguise is `Ruler::exact(Ratio::new(1, 1_000_000_000), unit)`. Types stop a bare float. They don't stop a constant baked into the binary. What stops it is that `Approved` can only come from `verify` against a `Keyring`. So R1's force is exactly **who holds keys that `verify` trusts**:
  - if the binary ships a keyring with a key whose private half is in the repo, the build environment or a test fixture reachable from `src/`, the code can sign its own constants, and R1 is a ruler in name only;
  - **fold:** `Ruler` has no public constructor except from parsed shard content (`addr` computed, never passed in); `Approved` has no constructor except `verify`; the production `Keyring` holds public keys only, and no private key lives under `rust/`; test keyrings are `#[cfg(test)]`/`tests/` only and can't reach a release build.
- **R1 stops at the crate boundary.** Math §7's first rows (commutator thresholds 1e-12, 1e-6; Refract 1e-12; `RotationConfig::default() = 1.0, 0.05, 0.3`) live in `prismqueer`, a git dependency. Mirror's API can wrap them, but the bare numbers stay bare inside prism until prism-Reed adopts `Approved`. The spec should say R1 binds mirror's API, and list prism's thresholds as bare rulers outside its reach.

---

## §7 `Observe` versus `Restrict` (priority 7)

The spec splits looking from ruling: "`Observe` … can't ban. `Restrict` turns findings into bans … runs only as an `Approved` ruler." But `Restrict::restrict(&self, snapshot, findings)` **takes the findings as input**, and nothing requires those findings to come from approved observers.

**Counterexample** (by construction from the spec's own signatures):
- an approved ruler R whose content is "ban every atom named in a `suspect` finding";
- an unapproved `Observe` that emits `suspect(x)` for whatever it likes.

The bans are then chosen by the observer. R is approved and fixed, but its *input* is not. Scarlet's "an observer that bans is a ruler", one level up: **an observer whose findings a ruler obeys is a ruler.**

Two further leaks:
- **Findings in the snapshot.** If findings are deposited in the same store ("one store", fixpoint §4) and `ModelRoot` covers them, a later `Restrict` reading only the snapshot reads earlier findings. That is the same steering, delayed one tick.
- **Failure as restriction.** An `Observe` that panics, or doesn't return, stops the run. That is a truncation (§3) without any ruler.

**Fold:**
- An `Approved` restricting ruler names, by address, the observers whose findings it reads. Those observers become part of its approved content, and their addresses go into `shaped_by`.
- Findings from unnamed observers aren't visible to `Restrict`.
- Findings live at their own altitude, outside `ModelRoot`.
- Observers are pure and total: a fault is reported, never a stop.

With that, the R7 split is real: looking that no ruler has adopted can't rule.

**Scope note.** `run_restricted`, `Observe`, `Restrict` and `well_founded_restricted` are **outside** my engine-core audit (`docs/audits/2026-10-29-seam-engine-core-scope.md` §3.1–§3.4): a restricted run, trait objects with arbitrary implementations, and keyring verification. Spec §5's last bullet says so too. They need their own audit before any `src/` commit. My lean is to seal both traits, so that their only implementation is "E evaluates the ruler's positive program". That also removes §1.1's lifted-ban case at the root.

---

## §8 Against Taut's families

Taut §4/§5 group the 81 rulers into:
- **Budget**: `min`, commutative;
- **Choice**: non-commuting; the order is the meaning;
- **Resolution**: coarsest wins, and the honest ones are three-valued ("don't know");
- **Equivalence**: partitions, meet/join;
- **Authority**: conjunction plus a signer;
- **Weighting**: narrows nothing until a Choice consumes it;
- **Identity roots**: one of each, so they don't compose.

### §8.1 Mara's seven kinds against the families

| `RulerKind` (spec:18) | family | problem |
|---|---|---|
| `Threshold` | Resolution ("is it zero?": P1–P10) **or** Authority (a pass/fail cut) | Two algebras under one kind. A Resolution threshold needs a **band** (P2/P3: 1e-12 … 1e-6 = don't know) and a **scale law** (absolute; × Frobenius; 16·ε·scale·n). `RulerContent::Exact { value, unit }` holds one value and no scale law, so the six numerical zeros of Taut §2.1 can't even be written down as distinct rulers |
| `Budget` | Budget | fits. `Count(NonZeroU64)` is right |
| `Quantizer` | Equivalence | composes as a partition. A ban can't express "these two are the same" (§8.3) |
| `Choice` | Choice | composition is ordered. The spec has no order field (§8.3, fixture 7) |
| `Ban` | none: a **mechanism** | It is how Choice-by-elimination, and possibly a budget, reach E. It doesn't belong beside the families |
| `Weight` | Weighting | narrows nothing (Taut §5: "`weigh` drops nothing"). By Alex's definition ("anything that narrows") it is an input to a Choice, not a ruler |
| `Tolerance` | Authority (the gate) | Authority composes by conjunction **with a signer**. The signer lives in `Approved`, so the kind and its approval are the same family seen twice |
| — | Identity roots (G7, G8, M24, M25, S5) | missing. They don't compose and are revocable only upward |

### §8.2 Which laws are envelope laws, which are per family

| law | envelope (every family) | per family |
|---|---|---|
| R1 no bare parameter | **survives**: the parameter is an `Addr` of an approved ruler, whatever its family | — |
| R2 `shaped_by` + erasure | `shaped_by` **survives** | **erasure is per family**:<br>• Budget: "truncated at k" (no atoms);<br>• Choice: the losing alternatives;<br>• Resolution: the readings collapsed into zero or don't-know;<br>• Equivalence: the merged classes;<br>• Weighting: always empty, so `Shaped` + erasure is vacuous |
| R3 reader dependence declared | the field **survives** | the admissible values differ:<br>• Resolution: reader-dependent by design (`settle.rs:47`);<br>• the `active` quantizer (Equivalence): **must not** be;<br>• Budget: never |
| R4 a comma held, booked upward as a settle | — | **per family**:<br>• Budget / Resolution / Authority: two disagreeing rulers have a **lawful composite** (min, coarsest, conjunction). The "booking" is the composition law, and the erasure is the overruled finer value. R4 holds with the law as the parent;<br>• Choice / Equivalence: there is no composite. Two rulers give **two results** (fixpoint:481), not one comma inside one third. R4 **fails** there; the right law is a fork, with each result naming its ruler;<br>• Weighting: no comma until a Choice consumes it |
| R5 approval ends in a human | **survives** as an envelope law, with §5's fix (non-transitive over versions) | — |
| R6 shape from b₁ | — | **per family; fails for the lattice families.** Prop 2.2 needs a **group** (`g_e = R_head · R_tail⁻¹`).<br>• Budget (min), Resolution (coarsest), Authority (∧) and Equivalence (partition meet) are semilattices with no inverses. Around any loop, the meet books every disagreement, so no unbookable holonomy exists and `Polycentric` means nothing structural. Only a declared refusal (`Kept`) makes them multi-truth.<br>• Holonomy needs invertible adjustments: reorderings and renamings, i.e. Choice, and Temperament (S10) |
| R7 observe ≠ restrict | **survives** as an envelope law, with §7's fold (a `Restrict` names the observers it reads) | — |
| R8 / RR1–RR6 restricted runs | — | **laws of the `Ban` mechanism only**:<br>• Choice-by-elimination: RR applies. On non-functional programs it is meaning (§2.2), and two Choice rulers' bans **unite** where Choice must **sequence** (fixture 7: union → hole; A then B → x; B then A → y);<br>• Budget: applied as truncation, not as a ban. Encoded as "ban everything from tick k", it is monotone in the tick but not a function of the snapshot (RR1's premise), and it lands in `withheld`, which would repair §3 (⇐). The spec must pick one;<br>• Resolution, Weighting, Authority: never act on E's model; RR is vacuous for them;<br>• Equivalence: a quotient merges atoms, a ban can only remove them. Applied as "ban all but the representative", it is a Choice in disguise |
| R9 meaning vs policy | — | Taut's third bin: **measurement** (Resolution) changes readings and verdicts (Partial vs Success), not clauses. R9 survives if pacts may hold Budget, Authority, Resolution (as measurement, recorded) and Weighting, and **Choice and Equivalence stay clause-side** (grammar, `μ`) or go through `settle … by:` with an `Erasure`. That is the same line §2.2 reached from the parser side |
| R10 bootstrap | the envelope part (record the pacts' addresses) **survives** | the erasure test is per family; a budget doesn't withhold (§3) |
| R11 revocation is a record | **survives** | Identity roots are revocable only upward |

### §8.3 Composition, computed

`ruler_crowbar.out` §7. Two parses x, y of one span:
- Choice A keeps the least name;
- Choice B keeps the heavier.

Through the RR mechanism (one `BanSet` per `Restrict`, united), the result is **a hole**. Applied in sequence, A then B gives x, and B then A gives y.

The spec's composition (`pacts: &[Approved]`, bans united) is commutative, Choice isn't, so the run computes a third thing that neither order yields. **Fold:** a pact's Choice rulers carry an explicit order, and the run applies them in that order, each to the survivors of the last. Or Choice leaves pacts (§8.2, R9).

### §8.4 "Pact" names three things

1. `Pact = Fractal<Ruler>` (spec:32).
2. `shards/epistemologic/pact/*`: sixteen **predicate** shards, i.e. clauses, on the meaning side of the spec's own line (Taut §2). Math:64's READING that `algebra.mirror` is a depth-1 fractal of rulers **fails**: it is a conjunction of clauses.
3. `@pact/fact`: an engine altitude for Datalog facts (`rust/engine/tests/e5_well_founded.rs:24`; my engine-core audit §3.4 uses the same name).

**Alex decides** which keeps the word. I lean that (1) takes a new name. The clause shards and the engine altitude predate it, and both are clause-side.

### §8.5 The gate is a ruler with no check (bears on R1 and R5)

- `.githooks/commit-msg:82` matches any string shaped like `docs/audits/YYYY-MM-DD-*.md`. It doesn't check that the file exists, that it is a Seam audit, or that the commit is within its scope.
- The alternative, `Signed-off-by: Seam <…>`, is a string any author can type.
- `[bugfix:restore]` (`:73-76`) is self-authorized.

So every bare constant in `rust/*/src` (Taut §3: changing a disguised ruler goes through FROZEN) can be changed by a commit that says `[bugfix:restore]`, with no one else involved: the cheapest bypass of R1. And R5's "human signature at the sink" has no counterpart in the one gate that exists, since nothing in it is a signature.

**For Alex (it's the repo's own ruler):**
- the hook resolves the cited path in the commit's tree, and requires the cited section to exist;
- `[bugfix:restore]` names the commit whose behaviour it restores, and the hook checks that the restored lines appear in that commit's parent tree;
- a Seam trailer is accepted only on a commit signed by Seam's key (SSH signing is already configured per author).

Until then, my engine-core audit's citation contract (§5 there) is enforced by review alone. I said so there, and it is the same finding.

---

## §9 To fold, to decide

**Mara must fold:**
1. RR5: add a `lifted` finding (or re-seed lifted atoms). RR2's semi-naive = naive is for fixed or monotone bans only. The evaluation strategy is part of the record when bans are non-monotone (§1.1).
2. `withheld`: a set of ruler addresses per atom (§1.4). RR6 recovers the model, not the trajectory (§1.3).
3. R9: functional programs only. A ban on a non-functional program that selects a parse is a choice settle, with its erasure and its trace in N. Check Ford functionality before claiming R9 (§2.2).
4. Prop 6.1: replace "iff" with "if the erasure is disjoint from the start cone and nothing truncated the run". `RestrictedRun.truncated`. RU8 split into (a) and (b) (§3).
5. R6: capacity, not declared b₁; `Untested` instead of vacuous `Hierarchy`; `Kept` computed from use; say whether sideways booking is allowed (§4).
6. Prop 2.2(a): surjective restrictions. Close the OPEN item with the tree fixture (§4.2).
7. The R5 conjecture: refuted as stated. Add RU4's red twin (§5).
8. R1: the naming convention plus review; the constructor and keyring rules; prism thresholds outside its reach (§6).
9. R7: the observers a `Restrict` reads are named in its approval; findings stay out of `ModelRoot`; observers are pure and total (§7).
10. One envelope (`addr`, kind tag, reader field, approval, application record) plus a body type per family: Budget, Choice (with order), Resolution (band + scale law + three values), Equivalence (partition), Authority (signer). `Ban` becomes the mechanism, `Weight` an input to Choice, and Identity roots are added (§8.1, §8.2).
11. R4, R6 and R8 are rewritten per family:
    - lawful composites for the lattice families;
    - forks for Choice and Equivalence;
    - shape (b₁, holonomy) only where adjustments are invertible;
    - RR only for the ban mechanism (§8.2, §8.3).
12. Math:64's READING withdrawn; `@epistemologic/pact` is clauses (§8.4).

**Alex must decide:**
1. **Self-amendment.** Must a changed ruler in a governance component be re-signed on its new address (non-transitive approval), or is "some human once signed the root" enough? The fixture shows the second lets a ruler drop its own constraint (§5).
2. **Choice is meaning.** Is a Choice ruler on a `split` grammar policy (the spec's current list) or meaning (a `settle` recorded in N)? The fixture shows it picks the tree (§2.2).
3. **Sealed or open traits.** Should `Restrict`/`Observe` run only as E-evaluated positive programs (monotone bans, no lifted bans), or are arbitrary Rust implementations admitted (then non-monotone laws and their own audit)?
4. **Booking sideways.** Allowed, forbidden, or a fourth shape?
5. **Who keeps the word "pact"**: the clause shards, the engine altitude, or the new `Fractal<Ruler>` (§8.4).
6. **Choice and Equivalence in pacts**, or clause-side only (§8.2 R9; the same as decision 2, from Taut's side).
7. **The gate's own checks**: the cited path exists, `[bugfix:restore]` names what it restores, the Seam trailer requires Seam's signature (§8.5).

*— Seam*
