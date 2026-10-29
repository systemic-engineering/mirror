# The certificate: a compilation that explains itself (spec)

*2026-10-29. Mara. Status: **Red**. Second version: folds Seam's checker audit (`docs/audits/2026-10-29-seam-checker.md`, script `tests/checker/oracles/certificate_crowbar.py`; four wrong certificates passed v1's checks). Changes are marked *[C §n]* with the audit section. Serves the loop's definition of done (`docs/loop/CHARTER.draft.md`). Builds on:*
- *the engine contract (fixpoint spec §4.1–§4.2, "E");*
- *the tick-shape theorem (`docs/math/2026-10-29-mara-engine-frequency-and-kleinos.md` Prop 2.1, "freq");*
- *the erasure record (fixpoint §6) and holes (fixpoint §5.1);*
- *the vocabulary (fixpoint §1.1: clear / opaque / dark);*
- *the ruler envelope (`mirror-ruler.md`).*

*Labels: PROVED, CONJECTURE, OPEN, READING.*

**Alex:** "the compiler can explain its own compilation at every step for any arbitrary shard." And: "What is an explanation if not a (non-)linear trace through the past?"

---

## §0 The definition, in five lines (v2)

1. A **certificate** is a content-addressed fragment, printed as mirror-text, that sits beside a result and never inside its address.
2. It holds the result's past cone as a partial order:
   - **layers**: Foata, i.e. the ticks;
   - **all first-tick derivations**;
   - **W's whole context sequence**, from the initial context on;
   - **erasures** and **holes**.

   It never holds the program, the facts or the verdict as truth. Those are re-derived.
3. "Explained" means the **checker**, from the **source** alone, re-derives P, F, well-formedness, the model, the layers, every declaration check, N and the **verdict**, and the certificate agrees with each. The derivation-level checks are soundness (S), closure (C) and per-layer completeness (check 8).
4. A log is a punctuation of the order, and therefore a ruler. Order-freedom is a **structural** property: acyclic, premises in earlier layers, the model addressed from the sorted set, negatives judged against fixed contexts. It is not sampled.
5. The checker is independent:
   - Python at `tests/checker/`, stdlib + pinned `blake3`;
   - no repo imports, no process, no JSON;
   - written by neither the engine's author nor the auditor;
   - verified against §5.0 known-answer vectors written by a third party;
   - pinned by tree address; when over budget it says **UNKNOWN**, never pass.

**What changed from v1:**
- (a) v1 read P, F and the verdict from the certificate. v2 re-derives them from the source.
- (b) v1 trusted the period-2 witness for W. v2 checks every γ transition from the start.
- (c) v1 had no completeness check. v2 adds check 8.
- (d) v1 ran functional checks only when Dark was claimed. v2 runs them on every result.
- (e) v1 certified N by `render(N, ε)`. v2 recomputes N.
- (f) v1 sampled order-freedom with K. v2 makes it structural, and K is a determinism test only.

---

## §1 The past cone is a partial order; ticks are its Foata form (PROVED)

- **Derivation order.** For an atom a with a derivation `(rule r, premises p₁ … p_k)`, draw edges `pᵢ → a`. Restricted to **first-tick derivations** (all positive premises at ticks < tick(a)), this is a DAG.
- **Ticks are the depth (PROVED, from freq Prop 2.1).** `tick(a) = 1 + max tick(positive premises)` over its first-tick derivations, i.e. the longest-path depth with facts at 0. A tick is also the naive stage `min{n : a ∈ T_Pⁿ(F)}`, intrinsic to (P, F) (Seam verdict 2).
- **Pinned conventions** *[C §verdict 4]*:
  - **No positive premises.** The depth of an atom derived by a rule with no positive premise (`a.` as a rule, or `r :- not s`) is `1 + max ∅ = 1`: it fires in round 1, as the engine already does ("a rule with no positive body fires once, in round 1"). Facts are depth 0.
  - **W's layers.** True atoms take their layers from the **final** `under = γ(over_K)` run. Undefined atoms take theirs from the final `over = γ(under_K)` run and are marked as `over`-layers. Negative premises in either run are judged against that run's **fixed** context (`over_K` or `under_K`).
- **Foata.** Layers are antichains, and every atom at layer t depends on some atom at t − 1. This is the unique Cartier–Foata normal form (Cartier & Foata 1969; Wikipedia, *Trace monoid*; the Stuttgart *Cartier-Foata-Garside Monoids* notes, Thm 10.4.1; via Kagi), so the layers are canonical.
- **A linear log is a punctuation** (PROVED core: picking one extension is non-injective on orders; READING: Watzlawick). The certificate stores the order. A tool printing a log returns `Shaped<Log, _>` naming its punctuation ruler.

## §2 What the certificate carries, and what it never carries

### §2.1 Which derivations: all first-tick ones (PROVED choice-free)

| option | verdict |
|---|---|
| one derivation per atom | refused: picking is a Choice (a ruler) |
| **all first-tick derivations** | **required**: determined by the model and ticks, finite, well-founded. **Checked by check 8** *[C §1.2]* |
| all derivations | optional (`--explain=all`): how-provenance (Green, Karvounarakis & Tannen, PODS 2007, power series for recursive programs). Finite truncation OPEN |

### §2.2 Never carried as truth *[C §1.3, §1.6]*

- **P and F.** The checker re-derives them from the source: its own reader, and for compile its own PEG → Horn translation and byte → fact encoding. It compares their addresses with the header's. The header is a claim, never an input. (Seam case 4: a spurious fact passes against the certificate's facts and fails against the source's.)
- **The verdict and exit code.** The checker derives them: `clear` / `opaque` / `dark` at 2 or 255 (fixpoint §1.1). The record's verdict is compared, never read. (Seam case 3: a `clear` hiding a functional violation.)
- **N.** For compile, the checker recomputes N with its own `strip` and `nf` from the certified parse items. `render(N, ε) = bytes` is then an **additional** check, never the certificate of N. (Seam §1.5: `N = ∅` with ε = the whole file round-trips.)
- **Erasures under a ruler.** The ruler's content is a positive program (sealed rulers, D3 lean), so the checker evaluates it and compares the erasure, rather than trusting `shaped_by` + erasure.

### §2.3 Under negation (W) *[C §1.1]*

- The certificate carries **every** round context `under₀ = F, over₀, under₁, over₁, …` up to consecutive equality.
- The checker verifies **every transition** `over_k = γ(under_k)` and `under_{k+1} = γ(over_k)` from the start. That is 2 · rounds γ-runs, a recomputation.
- **Leastness follows from the iteration.** The period-2 property at the end holds automatically and is **not** evidence. It is true of every fixpoint of γ², stable models included: Seam case 1, vector 04 with "q, r true, nothing undefined", passes the witness and fails the iteration.
- True atoms carry their first-tick derivations in the final under-run. Undefined atoms carry theirs in the final over-run (§1 conventions).
- Why-not for false queried atoms is optional (head-bound only; existential variables OPEN).

### §2.4 Holes, erasures, Dark *[C §1.4]*

- **Holes:** `hole <question> at <path>`, plus the undefined atoms in the start cone. They are part of the explanation (Opaque).
- **Erasures:** `shaped_by` plus the family erasure, re-evaluated by the checker (§2.2).
- **Dark at exit 2** (no light):
  - the certificate holds a location (line:col for the reader; or the engine `Dark` variant and its arguments) and no layers;
  - the checker re-reads with **its own reader**, never the compiler's, and re-checks engine `Dark` directly from the program text (the unsafe variable doesn't occur in a positive literal, an arity mismatch, a declaration position out of range);
  - it accepts iff the claimed location is **a** genuine error;
  - **The ordering ruler for "the" location** (several errors are several valid Dark reports): `@mirror/dark-order`, a declared Choice ruler. Its content:
    - reader errors by least byte offset;
    - then engine checks in the engine's declared order (arity, then safety, then declarations), each within by sorted rule id or predicate.

    The record's `shaped_by` for a Dark at 2 stays `none` (no rule declined the light). The canonical location is a punctuation, named in the certificate header as `dark-order <addr>`.
- **The converse:** the checker re-runs every well-formedness check on **every** input. A program that should be Dark, for which a model was printed, fails.
- **Dark at exit 255** (declined): the full model certificate, plus the declining rule, plus the derivations of the violating atoms. The checker evaluates **every** `functional` declaration on **every** result (over `true` under W, matching E), so a missing decline is caught too.

## §3 Encoding and address

A fragment at altitude `@mirror/certificate`, children ordered:

```text
0 header       result <addr>  program <addr, claimed>  facts <addr, claimed>  engine <addr>  dark-order <addr | none>
1 layers       one child per tick: sorted atom addresses (W: under-layers, then over-layers)
2 derivations  sorted by (head, rule id, premise addrs)
3 contexts     (W) the whole sequence under₀, over₀, under₁, … by address
4 erasures     (ruler addr, family erasure), sorted
5 holes        (question, occurrence), sorted
6 dark         absent | (2, location) | (255, rule, violating atoms)
```

The address follows the fixpoint §5.0 fragment rule. The certificate sits beside the result, never in its address. It is printed as mirror-text after the §1.1 record with `--explain`, and cached by the result's address.

## §4 The checker

### §4.1 Checks *[C §1, §3]*

1. **Re-derive P and F** from the source. Compare with the header's addresses.
2. **Well-formedness** on every input: arity, safety, declaration positions (and Ford WF for grammars).
3. **(S) soundness:** every derivation is a valid instance of its rule (own unification); every non-fact model atom has at least one derivation with positive premises in strictly earlier layers.
4. **Layers** equal the longest-path depths (§1 conventions).
5. **(C) closure:** one naive round over M adds nothing. For a budget-truncated run this is false by design, and the result must be shaped by a `Count` ruler, with check 8 plus "no layer > k" in place of (C).
6. **W:** every transition of the context sequence from the start (§2.3).
7. **Every `functional` declaration** on every result. The checker **derives the verdict and exit** and compares them with the record's.
8. **Per-layer completeness** *[C §1.2]:* for every atom a at layer t, the set of rule instances with head a, positive premises in layers < t, and negatives true under the run's context, **equals** the certificate's set for a.
9. **Compile:** N by the checker's own strip/nf; then `render(N, ε) = bytes`; root, interface, graph and uuid by its own encoder; ruler erasures by evaluating the ruler.
10. **Dark:** at 2, the location is reproduced by the checker's own reader or checks; at 255, the rule is genuinely violated by the derived atoms.

### §4.2 Order-freedom, structurally *[C §3]*

Replaying in any linear extension rebuilds the root (PROVED: the result is `F ∪ heads`, addressed from the sorted set). The **structural** conditions that cover every extension:
- the derivation graph is acyclic, with premises in strictly earlier layers (checks 3, 4);
- the model address is computed from the sorted set by the checker's own encoder;
- W's negative premises are judged against the fixed round context, never the replay's current set.

Replaying in K random extensions plus the reverse one remains a **determinism test of the checker's own implementation** (K = 100). It is **not** part of "done". The charter's "order-free" means the three conditions above.

### §4.3 Independence and admissible shape *[C §2, §4]*

- **Shape** (Seam ADMIT-NARROWED): `tests/checker/checker.py <source> <certificate> <record>`. Python stdlib plus `blake3`, pinned in the script's inline metadata and run with `uv run`. It prints mirror-text `pass` | `fail <check> <detail>` | `unreadable <location>` and exits 0 / 1 / 2.
- **It may not:**
  - import repo modules or other oracles;
  - call the `mirror` binary or any process;
  - use the network;
  - use `json` or any serializer (it reads mirror-text with its own line reader);
  - read any file but its three arguments;
  - sample;
  - take P, F or the verdict from the certificate.
- **Author:** neither the engine's author (Reed) nor the auditor (Seam). **Mara-lean: Mara writes it** (the spec's author, so it reads the spec as intended). **Decision for Alex.**
- **Known-answer vectors for fixpoint §5.0:** byte-level examples (input → encoded bytes → BLAKE3 digest) for an atom, a fragment, a lens, `τ` with and without a grammar address, and a ruler (`mirror-ruler.md` §1.1). They are written from the spec text by someone who wrote **neither the engine nor the checker**. **Mara-lean: Taut** (grep-first, read-only on both). **Decision for Alex.** Both implementations must match them. A disagreement is a spec defect, resolved by Mara and logged.
- **Semantic vectors:** `tests/pact/oracles.md` and the adversarial set (§5), never derived from either implementation's output.
- **Budget:** a shard the checker can't finish within the loop's budget ruler is **UNKNOWN**, never pass.
- **Staging:** level 1 starts with **pact-text** certificates (the reader is small). The compile parts (PEG → Horn, strip/nf) arrive with T1, each as its own commit under Seam's checker audit.
- **`rust/checker`:** refused as the trust root. It is admissible later only as a second, faster checker, after the Python checker is frozen and measured too slow (a comma for Alex).
- **Frozen:** the charter pins the checker tree, this spec's commit and the vectors. The fix procedure is Seam's checker audit §5 (red fixture independent of the compiler, Seam amends, the checker's author fixes, Alex signs the new pin, re-measure, re-open every "done" claimed under a false-accepting pin).

## §5 Red tests: the adversarial set is the ground truth *[C §6]*

Each is a certificate the checker must **reject**, landing as a fixture with the check it exercises. Cases 1–4 are computed in `tests/checker/oracles/certificate_crowbar.out`.

| CT | adversarial certificate | caught by |
|---|---|---|
| CT-A1 | a stable model with a period-2 witness (vector 04, "q, r true") | 6 (full context sequence) |
| CT-A2 | a missing first-tick derivation (the diamond, d via c omitted) | 8 |
| CT-A3 | `clear` hiding a functional violation (vector 05 shape) | 7 |
| CT-A4 | a spurious fact in the certificate (`edge(a, z)`) | 1 |
| CT-A5 | a Dark location that is readable | 10 |
| CT-A6 | an unsafe program with a model | 2 |
| CT-A7 | `N = ∅`, ε = the whole file | 9 |
| CT-A8 | a layer off by one | 4 |
| CT-A9 | a circular derivation pair | 3 |
| CT-A10 | a budget-truncated run claiming the fixed point | 5 + 8 |
| CT-A11 | a correct certificate whose model address used a different sort order | 9 / encoder (known-answer vectors) |

Positive tests (must **pass**):
- CT1: vector 01, plus the determinism test (K = 100 extensions);
- CT4: vector 04 with its full context sequence;
- CT5: vector 05 as Dark at 255;
- CT6: an unreadable fixture as Dark at 2, located by `@mirror/dark-order`;
- CT8: `mirror.mirror` end to end (with T1).

## §6 Level 2: kintsugi explains its gluing

Unchanged from v1 in structure. Level-2 certificates reference level-1 certificates by address. The same rule holds: the checker re-derives each finding from the compiled interfaces and the book, never reading a finding as truth. Per finding:
- line comma: both restriction outputs, and the relation R;
- gap: the occurrence, and the certified book;
- claim: both declarers;
- `among`: a cycle basis, plus union-find;
- twist: the gauge-solve contradiction, or the holonomy cycle;
- kept comma: the approval or `LeaveOpen` record.

The level-2 canonical order is OPEN.

## §7 Ledger

| claim | status |
|---|---|
| ticks = Foata of the first-tick order (with the pinned conventions) | PROVED |
| a linear log is a punctuation | PROVED core; READING (Watzlawick) |
| all first-tick derivations are choice-free; completeness checkable (check 8) | PROVED |
| (S) + (C) ⇔ M = lfp(P, F), **relative to P and F re-derived from source** | PROVED |
| the period-2 witness certifies W | **REFUTED** (Seam case 1); the full transition sequence does (PROVED) |
| `render(N, ε)` certifies N | **REFUTED** (Seam §1.5); recomputing N does |
| replay in any extension rebuilds the root | PROVED; covered by the structural conditions, so sampling isn't needed |
| a diverse checker + DDC suffices | NARROWED: plus known-answer vectors, a different author, a different language |
| why-not with existential variables; full provenance truncation; level-2 order | OPEN |
