# `mirror convergence` — kintsugi, mended, until nothing moves (spec)

*2026-10-29. Mara. Status: **Red**. v0 is §3; v1 waits for Taut's flux scout and the grammar-spectrum math.*

**Siblings:** `docs/specs/mirror-kintsugi.md` (cited "kintsugi §n": finds the commas), `docs/specs/mirror-fixpoint-parser.md` (cited "compile §n"), math `docs/math/2026-10-29-mara-compile-kintsugi-local-sections-and-gluing.md` (cited "math §n"; §11 for the spectral uuid on fragments).

---

## §0 The loop (Alex, 2026-10-29)

```text
kintsugi ─▶ split (flux) ─▶ restrict (v1) ─▶ rotate into questions ─▶ approve ─▶ settle ─▶ write back ─┐
   ▲                                                                                                      │
   └──────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

| step | what | mirror mechanism |
|---|---|---|
| 1 | **kintsugi** finds commas, gaps, claims, Dark and Opaque lines | `mirror kintsugi` (v0: full run) |
| 2 | **flux**: each finding's candidate mends are held together, not chosen | `split`: one immutable root per candidate; the held set is a fragment with its own `addr` |
| 3 | **restrict**: narrow the candidates by frequency without perturbing them (a filter yields a new root; the old one stays) | **v1**, pending Taut's flux scout and the grammar-spectrum math |
| 4 | **rotate**: each comma becomes a question (Karl Tomm's two moves: what do I know is true; what happens when I question it) | the comma's `question` address (kintsugi §6) plus its candidates, handed to a human |
| 5 | **approve**: a human approves one candidate, or a ruler approved in advance does | §1, the guard |
| 6 | **settle**: infer into the AST | `settle … by: <ruler>`; what is dropped is logged in the erasure record |
| 7 | **write back** | `@io.write = settle \|> shift @io/write`: a new root, a git commit signed by `mirror <mirror@spectral.engineer>` |
| 8 | **kintsugi again** | |

**Converged** means a pass comes back unchanged: two consecutive passes print the same kintsugi `run` address. That is the loop's fixed point, under the same stop test as compile §3's stage iteration: consecutive equality. A recurrence at distance p > 1 is reported as `cycle(p)`, never as converged.

**The measure along the way is complexity reduction**, reported per pass and never used as the stop test: `claims`, `gaps`, `names`, `dark`, `unbound`, `opaque`, `commas`, `twists`, `fractured − expected`, and the number of files under `boot/`. **Convergence means fewer unresolved commas, not fewer readings**: the candidates that weren't chosen stay as roots in the flux fragment, addressed and recoverable.

---

## §1 The guard

Alex: **"The guard is the human approving the comma."**

- Nothing is written back unless a human has approved that comma's resolution.
- **The one exception** is a ruler declared in advance (`ruler @mirror/convergence/auto { … }` in `mirror.spec`) that names a class of mends it approves. That ruler is itself human-approved, signed, and its address appears in every commit it authorises.
- Everything else is **offered and waits**. A finding with no approval stays in the next pass's record, with `resolution offered …`.
- An approval is a splinter: `approval(question, occurrence, resolution, approver, signature)`. The write-back commit names it in a trailer (`Approved-by: <approver> <approval addr>`).
- This is rerere's rule (offered by question, applied by occurrence; compile §5.1) with the application step held by a person.

---

## §2 Three states: Fractured | Flux | Settled

From Alex's 2026-09-04 sum type (Reality = Fractured(Vec<Shard>) | Flux(Liquid<Shard>) | Settled(Crystal<Shard>)), read at the altitude of one convergence pass:

| state | in this loop | observable |
|---|---|---|
| **Fractured** | the shard set falls apart into compositions the spec didn't declare | `fractured ≠ expected` (kintsugi §4) |
| **Flux** | a finding's candidate mends, held as a superposition | the flux fragment: one root per candidate, none settled |
| **Settled** | a rule or shard at its fixed point, `x = COORD(x)` | declared `fixed` (Alex 10-29: "`settle` is the act, `fixed` is its result"); a converged pass |

Moving from flux to settled is always a `settle` with a declared ruler: the human's approval, or the advance ruler of §1. Nothing settles by default.

---

## §3 v0: the one-candidate mends

v0 offers only the mends that have **exactly one candidate**. Choosing among several is v1. Each is offered, approved (§1), settled and written back, and then the next pass checks it.

| finding | the one candidate | approval asks |
|---|---|---|
| **undeclared** `(S, X)` (Dark line), when X is claimed by exactly one shard | insert `in @X` into S's import block (`@kintsugi/fracture/inport`'s `discharge`, which the substrate already declared) | "S uses @X here. Add `in @X`?" |
| **`name?`** (a shard with no `out`) | add `out @<name>`. The path-derived name is offered **as a suggestion**: the human approves or types another, so identity never comes from the path (compile §2 item 6) | "This shard has no name. `@mirror/book`?" |
| **claim** `@X` between a `boot/` file and a `shards/` file | collapse onto the `shards/` claimant. The `boot/` file's interface is diffed against it; exports only `boot/` has are offered as additions, and conflicting exports become line commas for the next pass | "Collapse `boot/std/kintsugi.mirror` into `shards/kintsugi.mirror`? Two exports move, one conflicts." |
| **gap** `in @X` with no X | none in v0 (several candidates: delete the line, create X, or rename) | offered as a question only |

**The `boot/` → `shards/` migration is the first convergence run.** With `boot/` in the target, today's 20 claims (kintsugi K0b) are the offers of pass 1. Each approved collapse removes a claim, and often turns an Opaque line into a bound one, e.g. `roomba.mirror`'s `in @kintsugi` (kintsugi K0b). The per-pass measure shows `boot/` shrinking.

**Test C1 (v0).** On the K0b sandbox, approve every offered claim collapse in a scripted session that records one approval per offer. Then:
- the next pass has `claims` reduced by exactly the number approved;
- every write-back commit carries its `Approved-by` trailer and the `mirror@spectral.engineer` signature;
- the unchosen candidates are still addressable from the flux fragment.

**Test C2.** With no approvals, two consecutive passes give the same `run` address (converged at "nothing approved"), and nothing was written: the guard holds.

**Test C3.** An advance ruler that auto-approves only `in @X` insertions with a unique X: undeclared pairs with a unique X are mended with the ruler's address in the trailer; everything else still waits.

**Test C4.** A pass that writes back a mend creating a new comma reports it in the next pass. The loop never treats a write-back as success until kintsugi has re-run.

---

## §4 The spectral uuid on every `Fragment`: "you resolved it similarly in the past…"

Alex: "add spectral UUID on the `Fragment` and we can ask 'you resolved it similarly in the past…'".

- **Every fragment carries `uuid_spectral`**, not only the root: 80 dark bits from its own `addr`, 48 active bits from the spectrum of its own sub-DAG (compile §8; math §11, Def 11.1).
- **rerere offers at two distances, each labelled:**
  - **exact**: same question address. "You resolved *this*."
  - **similar**: a recorded resolution whose fragment lies within a declared radius of this one in the active coordinate. "You resolved something *similar* like this." The offer shows the distance and the rulers (metric, radius, active quantizer).
- **Similar offers need the `active` quantization `@ruler` signed** (compile §12, decision 11, still open). Until then every fragment is `active unsigned` and **only exact offers exist**.
- **Identity stays the dark half; merges stay on `addr`.** Similarity never decides identity. It only proposes.
- **The human approves either way** (§1). A similar offer is never auto-approved by default. An advance ruler could name a radius, but that is a human decision, recorded.

**PROVED** (math Prop 11.2):
- dark exists per fragment at no extra cost;
- active depends only on the fragment's sub-DAG, so it is cacheable by `addr`;
- active never decides identity (cospectral, non-isomorphic sub-DAGs exist).

**CONJECTURE** (math Conj 11.3): a near active coordinate predicts a transferable resolution, at a rate well above chance.

**Refutation test C5** (after the ruler): two fragments with the **same** active bits, each with a recorded resolution, where neither resolution, applied at the other, makes it glue or fill. Prop 11.2(c) guarantees that candidate pairs exist. The conjecture can only survive as a measured **rate** over the recorded resolutions, never as a rule. The test publishes the rate.

---

## §5 Convergence CI, mapped

`~/dev/systemic.engineering/blog/void/1draft/decision.md`, the section "Hyped: Convergence CI" (protected draft; quoted briefly). Its question: *"did the tension resolve, and who was asked?"*

| Convergence CI says | mirror mechanism |
|---|---|
| "Every decision is an edge with both ends." A decision without its question "comes back as a question: *Who asked?*" | a line is `in @X`: both ends, declared. An **undeclared** reference is a dependency with no asker on record, so it is Dark and comes back as a question (kintsugi §3) |
| "A pipeline declares the tensions it promises to resolve … Green means every promised one resolved." | declared tensions: tritones in `@epistemologic/math/music/dissonance` and `@song`. There is no `@pipeline` shard at `346253b`; the substrate's word for a promised tension is the dissonance shard's. The gate ruler (kintsugi §4) declares what must resolve |
| "Disagreement is an event, not noise … offered back … *Same again?* Offered, never applied behind your back." | rerere: a comma's question, a recorded resolution, offered by question, applied by occurrence, approved by a human (§1, §4) |
| "The whole sequence is kept. Append-only, hash-chained, replayable." | content-addressed roots (every pass, every flux candidate, every approval is a splinter) and git commits; later, the Voice (observed calls) |
| "When the system doesn't know, it says so … It asks." | hole (compile), gap, `name?` (kintsugi), Dark (undeclared, unbound: nothing looked), Opaque ("which one?") |

**Binds back:**
- **spectral**: the store. Convergence passes, flux fragments and approvals are crystals and events.
- **bullet-up**: the roomba asks before it decides. §1's guard is the same rule at compiler altitude.
- **Convergence CI**: green = the gate ruler's promised tensions resolved, not "the steps ran".

---

## §6 The record

One record per pass, then one per offer:

```text
pass        3
run         <kintsugi run addr>
previous    <run addr of pass 2>
converged   false
offered     17
approved    5
written     5
claims      15
gaps        1
names       9
dark        38
boot_files  <files under boot/>

offer       claim @kintsugi
candidates  1
flux        <flux fragment addr>
question    <addr>
resolution  offered exact <addr>
```

`converged true` iff `run = previous`. `cycle(p)` replaces it when a run recurs at distance p > 1.

---

## §7 v0 / v1

| | v0 | v1 |
|---|---|---|
| kintsugi | full run | incremental (kintsugi §5) |
| mends offered | one-candidate (§3) | several candidates, held as flux |
| narrowing | none | frequency restriction (a filter yields a new root) |
| rerere distance | exact | exact + similar (after the active ruler) |
| auto-approval | none, or an advance ruler (§1) | the same |

---

## §8 Decided and open

**Decided (Alex 2026-10-29, third round).** These supersede the matching leans in this spec, in `mirror-kintsugi.md` §10 and in compile §12; fold the body text in the next pass.
1. **`twist`** is the name for the record of mends whose holonomy doesn't close. `among` stays the topology count.
2. **The first job of convergence** is folding each `keywords.mirror` into its parent shard. One candidate per fold; it dissolves the K0a duplicate claims (`@mirror/spec`, `@song`).
3. **`in @X` covers exactly `@X`**, not `@X/…` (row 21).
4. **A gap is where convergence goes Dark.** `in @X` with no X anywhere has no candidate mend, so the loop stops and asks "where is X?": Dark, exit 2. Not a red failure, and not something a gate ruler tolerates.
5. **Frozen is part of the relationship, not of a shard.** The line between two voices is the emergent third: a comma lives in the relationship, and the relationship says what each side may write. A comma against a shard the running voice doesn't own is **offered to its owner**, never mended. Prism (pinned rev), crystals signed by another identity and `@std` are owned elsewhere; `boot/` is not (it is being migrated). The ownership map is the voice cover (`peer/colony`). This is where math Prop. 3.3′ (a single-line comma against a frozen side is an H¹ class) enters the record: such a comma's `resolution` is `offered @<owner>`.
6. **`name?`: the human picks.** The mend offers both the path-derived name and the first declaration's name; neither is applied without a choice.
7. **An approval is both** a signed commit trailer (`Approved-by:`) and a splinter in the store.
8. **Advance rulers are `@kintsugi/fracture` species where the compiler learns its own auto-resolutions.** Each species already carries detection (`focus/project/split/shift/settle`), a mend (`resolve_*` returning a morphism scored by `dissonance`) and, in most species, `@kintsugi/consent`. The learning loop:
   - commas get resolved with human approval, and rerere records each resolution;
   - when a class keeps being mended the same way (same fracture species, same morphism, approved each time), the compiler **proposes** promoting it to an auto-resolution in that species;
   - **the human signs the promotion** (the guard, one level up: the compiler never promotes itself);
   - a class is proposable only if every mend in it has exactly one candidate, is reversible, touches only the shard's own declarations, and changes no other shard's verdict except turning a Dark line visible (so: `inport` with a unique X qualifies; naming an `out` and collapsing claims don't);
   - **demotion is automatic**: one reverted auto-mend returns the class to "offer and wait";
   - every auto-mend cites its species and the signed promotion (`Approved-by: ruler <addr>`), and every write it made stays listed for review.

**Drift found while deciding 8:** `shards/kintsugi/fracture/out_derivable.mirror` mends in the opposite direction from decision 2 of `mirror-kintsugi.md` §10. It removes an `out @X` derivable from the path, which under the `name?` rule would create holes. Retire or invert it; propose it as the first convergence job after the `keywords.mirror` fold.

**Still open for Alex:** names for b₁-as-bugs and b₁-as-tasks; whether the voice cover gets its own words or carries prism's musical names (held lightly).

**For Taut (answered 2026-10-29, scout `26ed4c9` on `taut/sheaf-cohomology-scout`):** `Liquid<T>` never existed in code, and the held superposition exists only in math and script (the least model of the PEG-as-Datalog chart). Spectral restriction is non-mutating by addressing but **not lossless**: every filter except `ring` is a `settle` (declared ruler, erasure logged). v1's restriction is `split` = `(P x, (1−P) x)` then `settle` keeping the band. prism's `spectral::settle` + `Tail` already has that shape; the band projector is asked of prism-Reed in handoff `prism-004`. Decision 8 of compile §12 (choice as `split` + `settle`) is **still open** (prism-Reed).

**For Mara (next):** the grammar-spectrum math behind v1's frequency restriction.

**For Seam:**
- C4: can a write-back loop oscillate (`cycle(p)`) under one-candidate mends alone?
- C5: the similarity refutation test's design.
