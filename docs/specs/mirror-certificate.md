# The certificate: a compilation that explains itself (spec)

*2026-10-29. Mara. Status: **Red**. Serves the loop's definition of done (`docs/loop/CHARTER.draft.md`). Builds on: the engine contract (fixpoint spec §4.1–§4.2, "E"), the tick-shape theorem (`docs/math/2026-10-29-mara-engine-frequency-and-kleinos.md` Prop 2.1, "freq"), the erasure record (fixpoint §6), holes (fixpoint §5.1), the vocabulary (fixpoint §1.1: clear / opaque / dark), and the ruler envelope (`mirror-ruler.md`). Labels: PROVED, CONJECTURE, OPEN, READING.*

**Alex:** "the compiler can explain its own compilation at every step for any arbitrary shard." And: "What is an explanation if not a (non-)linear trace through the past?"

---

## §0 The definition, in five lines

1. A **certificate** is a content-addressed fragment, printed as mirror-text, beside a result (never inside its address).
2. It holds the result's **past cone** as a partial order:
   - **layers**: the deposits in Foata normal form;
   - **derivations**: every first-tick rule instance, with its premises' addresses;
   - **erasures**: what was narrowed, and by which ruler;
   - **holes**: the questions left open.
3. "Explained" means two checks pass:
   - **soundness**: every atom has a well-founded derivation from the facts;
   - **closure**: one more round derives nothing new.

   Together they prove the result is *the* least model, not just *a* model.
4. Any log is one **punctuation** (a linear extension) of this order. Choosing one is a ruler. The certificate carries none.
5. An **independent checker** replays the certificate in **any** linear extension and must rebuild the identical root. Dark at 255 is certified by the derivations of the violating atoms plus the declining rule; Dark at 2 by the location where no light came back.

---

## §1 The past cone is a partial order; ticks are its Foata form (PROVED)

- **Derivation order.** For an atom a with a derivation `(rule r, premises p₁ … p_k)`, draw edges `pᵢ → a`. Restricted to **first-tick derivations** (all premises at ticks < tick(a)), this is a DAG.
- **Ticks are the depth (PROVED, from freq Prop 2.1).** Every atom first derived at tick t ≥ 1 has a first-tick derivation with a premise at t − 1 and none later. So `tick(a) = 1 + max tick(premises)` over its first-tick derivations. That makes tick(a) the longest-path depth of a in the first-tick DAG, with facts at depth 0.
- **That is the Foata normal form.** The layers are antichains (atoms at one tick don't depend on each other through first-tick derivations, since their premises are strictly earlier). Every atom at layer t depends on some atom at layer t − 1. Cartier and Foata (1969) introduced traces and this normal form; see the trace-monoid literature, e.g. Wikipedia, *Trace monoid*, and the Stuttgart *Cartier-Foata-Garside Monoids* notes, Thm 10.4.1, "the classical Cartier-Foata normal form" (via Kagi). The normal form is **unique** for a given order, so the layers are canonical, not a choice.
- **A linear log is a punctuation (READING, with a PROVED core).** Every linear extension of the order is a valid replay order. Picking one discards the independence between atoms in the same layer. Picking is non-injective on orders, so it is a choice, i.e. a ruler (Watzlawick's punctuation, as in the manipulation essay). The certificate stores the order, not a linear extension. A tool that prints a log returns `Shaped<Log, _>` naming its punctuation ruler.

The same structure appears as prism's `Progression` and the Voice's beats (READING; Reed's identification).

## §2 What the certificate must carry

### §2.1 Which derivations: all first-tick ones (PROVED to be choice-free)

| option | verdict |
|---|---|
| one derivation per atom | **refused as the default.** Picking one among several first-tick derivations is a Choice (a ruler). The certificate would then explain *a* reason, not *the* reasons |
| **all first-tick derivations** | **required.** The set is determined by the model and the ticks alone, so it needs no ruler. It is finite: at most the rule instances that fire at that tick. Each is well-founded (premises strictly earlier) |
| all derivations, including later-tick ones | **optional**, as `--explain=all`. This is how-provenance proper: Green, Karvounarakis & Tannen, *Provenance Semirings* (PODS 2007, "datalog and semirings of formal power series", via Kagi). For recursive programs the polynomial can be an **infinite power series** through cycles, so it is not required. Its finite truncations are OPEN as a spec item |

**Why "first-tick" is the right cut (PROVED).** A derivation with a premise at the same or a later tick can be circular (`p` from `q`, `q` from `p`) and explains nothing. First-tick derivations are exactly the ones that are well-founded with respect to the computation actually run.

### §2.2 "Explained at every step" (PROVED: least-model certificate)

For the engine's least model M over program P and facts F:
- **(S) soundness:** every non-fact atom in M has at least one certified first-tick derivation. So M ⊆ lfp(P, F) (induction on tick).
- **(C) closure:** one naive round over M derives nothing outside M, i.e. `T_P(M) ⊆ M`. So M is a pre-fixpoint, and lfp ⊆ M (Knaster–Tarski).
- **Together:** M = lfp. Without (C), a certificate shows only that M is a sound *subset* (the engine could have stopped early). Without (S), only that M is *closed* (it could contain junk).

This is a **certifying algorithm** in the sense of McConnell, Mehlhorn, Näher & Schweitzer (*Certifying algorithms*, Computer Science Review 2011: "an algorithm that produces with each output a certificate or witness that the particular output has not been compromised by a bug", via Kagi). Checking (C) costs one round. Checking (S) is linear in the certificate.

**For compile (the whole pipeline, every step):**

| stage | what certifies it |
|---|---|
| read (PEG in E) | the E certificate of the parse items (S and C above), plus Ford functionality checked on `m` |
| strip, normalize, settle | the erasure record ε (fixpoint §6, already required): `render(N, ε)` reproduces the bytes |
| crystallize | self-certifying: the Merkle address is recomputed by the checker (fixpoint §5.0) |
| graph, identify, interface | functions of N, recomputed by the checker |
| narrowing anywhere | `shaped_by` plus the family erasure (`mirror-ruler.md`) |

### §2.3 Under negation (W)

- **True atoms:** first-tick derivations from the last γ round, with each negative premise `not q` justified by the context set S of that round, given by address. The certificate carries the round contexts' addresses (`under_k`, `over_k`), so the checker can rerun γ.
- **Undefined atoms:** the period-2 witness (freq F6): `γ(under) = over` and `γ(over) = under`, checked by the checker with two γ runs.
- **False atoms** (queried): why-not, i.e. for each rule instance with that head, the failing literal. This is a query over the final model, finite for head-bound variables (oracle v0 limit). It is **optional at level 1** (`--explain=why-not`). For existential body variables it is OPEN, as the oracles note.

### §2.4 Holes, erasures, Dark

- **Holes:** each `\` occurrence as `hole <question> at <path>`, and each undefined atom in the start cone (fixpoint §4). A hole is part of the explanation: the certificate says where the compiler doesn't know, and that is the explanation (Opaque).
- **Erasures:** `shaped_by` plus the family erasure, per ruler (`mirror-ruler.md` R2).
- **Dark at exit 2** (no light): the certificate holds the location (line:col, or the engine `Dark` variant and its arguments) and **no layers**. Nothing was derived, so there is nothing to replay. The checker's job is only to confirm the location reproduces the Dark (re-reading the bytes up to that point fails the same way).
- **Dark at exit 255** (declined): the full certificate of the model, plus the declining rule (by address, or surface text until it has one) and the **derivations of the violating atoms**. For vector 05 that is the derivations of `owner(doc, alex)` and `owner(doc, reed)`, each from its `claim` fact. The decline is explained by the past cone of what was declined.

## §3 Encoding and address

A fragment at altitude `@mirror/certificate`, children ordered:

```text
fragment @mirror/certificate
  0 header       result <addr>   program <addr>   facts <addr>   engine <addr of E's shard / build>
  1 layers       fragment: one child per tick, each a sorted list of atom addresses
  2 derivations  fragment: sorted by (head addr, rule id, premise addrs); each = (head, rule, premises)
  3 contexts     (W only) the round contexts by address
  4 erasures     fragment: (ruler addr, family erasure) sorted by ruler addr
  5 holes        fragment: (question, occurrence) sorted
  6 dark         absent | (exit 2, location) | (exit 255, rule, violating atoms)
```

- **Address:** fixpoint §5.0's fragment rule. The certificate's address depends only on its content. Two runs that derive the same model the same way have the same certificate.
- **Not in the result's address:** the certificate sits beside the result, like the erasure record and witnesses (fixpoint §5).
- **Mirror-text** is the default printing. `--explain` prints it after the §1.1 record.
- **Size:** linear in the number of first-tick rule instances. For parse items that is the engine's run size, so `--explain` is opt-in, and the certificate is cached by the result's address.

## §4 The checker

**The property (PROVED).** Replaying the certificate's derivations in **any** linear extension of the derivation order rebuilds the identical model, and hence the identical root. Each step adds a head whose premises are already present, so the final set is the union of the heads plus the facts. Its address is the store-merge fragment of a sorted set, which is order-free.

**What the checker checks:**
1. every derivation is a valid instance of the named rule (its own unification);
2. (S): every non-fact model atom has ≥ 1 derivation with strictly earlier premises;
3. (C): one naive round over M adds nothing;
4. the layers equal the longest-path depths of the derivation DAG (Foata);
5. the root, interface and graph recomputed from the replay equal the record's;
6. for W: the contexts reproduce `under`/`over` and the period-2 witness;
7. for Dark at 255: each violating atom is derivable, and the declining rule is violated by them.

**Independence (Seam audits):**
- the checker is a separate crate (`rust/checker`), **not depending on `engine`**;
- it has its own unification, its own naive round, and its own §5.0 encoder (like E1's naive evaluator, which is independent of the semi-naive core);
- it reads only the certificate, the program and the facts;
- it is diverse in algorithm (naive, not semi-naive).

Seam audits that it imports nothing from `engine` or `ruler`, that it checks both (S) and (C), and that the address recomputation doesn't share code with the compiler's encoder. CONJECTURE: a diverse checker plus DDC on E (fixpoint §3) is enough for "explains itself" not to collapse into "the compiler vouches for itself".

**Order-freedom test:** the checker replays in K random linear extensions (fixed seed) plus the reverse-topological-stable one, and all must agree. This generalises the 720-orders test of pact vector 01.

## §5 Level 2: kintsugi explains its gluing

A level-2 certificate references level-1 certificates by address (Merkle), so explanation is recursive. Per finding:

| finding | its certificate |
|---|---|
| line comma | the two restriction outputs (addresses), each pointing into its shard's compile certificate (the interface part), and the relation `R` that failed |
| gap | the `in @X` occurrence, and the book's address showing no claimant of X (absence is checked against the book, which is itself certified as the set of `out`s) |
| claim | the two declaring shards' certificates (the `out @X` lines) |
| `among` | a cycle basis of Σ and the union-find witness (components per `Kᵢ` and of Σ), so the checker recomputes `b₁(Σ) − Σ b₁(Kᵢ)` |
| twist | the mend family and the gauge-solve failure: the constraint that propagated to a contradiction, or the cycle with non-trivial holonomy |
| kept comma | its question address and the approval or `LeaveOpen` record |

The level-2 checker property is the same: replay in any order of the gluing's dependencies rebuilds the identical `run` address. **OPEN:** whether the level-2 order (lines, then cycles) has a canonical Foata form of its own, or inherits one.

## §6 Red tests (sketches)

- **CT1 (vector 01):** `--explain` gives layers `[facts | path depth 1 | 2 | 3]`, every `path` atom with all its first-tick derivations; the checker replays in 100 random linear extensions plus the reverse one, and all give the same model address.
- **CT2 (closure matters):** a certificate with the last layer removed passes (S) and fails (C); one with a spurious atom passes (C) and fails (S).
- **CT3 (no single-derivation default):** an atom with two first-tick derivations (diamond `a→b, a→c, b→d, c→d` where `d` has two rules) carries both; a certificate with only one fails the checker's "all first-tick derivations" rule.
- **CT4 (W):** vector 04: `r` true with `not s` justified by the context; `p, q, t` undefined with the period-2 witness checked by two γ runs.
- **CT5 (Dark 255):** vector 05: the certificate holds both `owner` derivations and `shaped_by functional owner(1) -> 2`; the checker confirms the violation.
- **CT6 (Dark 2):** an unreadable fixture: a certificate with only a location; re-reading reproduces the same location.
- **CT7 (independence):** the checker crate has no `engine` dependency (`cargo tree`); a deliberately wrong derivation, injected by mutating the engine in a test build, is caught by the checker.
- **CT8 (compile, every step):** for `mirror.mirror`, the certificate's parse part checks, `render(N, ε)` reproduces the bytes, and the recomputed root equals the record's.

## §7 Ledger

| claim | status |
|---|---|
| ticks = Foata normal form of the first-tick derivation order | PROVED (freq Prop 2.1 + the depth argument) |
| a linear log is a punctuation, i.e. a ruler | PROVED core (non-injective on orders); READING (Watzlawick) |
| all first-tick derivations are choice-free and finite | PROVED |
| soundness + closure ⇔ M = lfp | PROVED (Knaster–Tarski) |
| replay in any linear extension rebuilds the root | PROVED |
| full provenance (power series) for recursive programs | OPTIONAL; finite truncation OPEN |
| why-not with existential variables | OPEN |
| a diverse checker + DDC suffices for self-explanation | CONJECTURE |
| level-2 canonical order | OPEN |
