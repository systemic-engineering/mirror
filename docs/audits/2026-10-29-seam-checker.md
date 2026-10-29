# Audit: the certificate checker, trust root of "done"

*2026-10-29. Seam. Pre-charter. Branch `seam/audit-checker` from `reed/ship` @ `d77a4a07`. Target: `docs/specs/mirror-certificate.md` (Mara, `d77a4a07`), `docs/loop/CHARTER.draft.md`, `docs/loop/STATE.md`.*

**Method.** `tests/checker/oracles/certificate_crowbar.py` → `certificate_crowbar.out`. It is a literal implementation of the certificate spec's §4 check list (checks 1, 2, 3, 4, 6, 7) over ground programs with negation. Each case is a certificate that passes those checks while the result is wrong, or a demonstration of what a check can't see.

---

## Verdicts

| # | claim (spec line) | grade |
|---|---|---|
| 1 | (S) + (C) ⇒ M = lfp (§2.2, :48-53) | **HOLDS** for positive programs, **relative to P and F**. It certifies nothing if P or F come from the certificate or the compiler rather than from the source (case 4). For compile, P is the grammar's Horn clauses, and their translation isn't certified (§1.6) |
| 2 | first-tick derivations are choice-free (§2.1) | **HOLDS**. A tick is the naive stage `min{n : a ∈ T_Pⁿ(F)}`, intrinsic to (P, F) |
| 3 | … and "all first-tick derivations" are required (§2.1, CT3) | **FAILS as specified.** §4's check list has no completeness check, so CT3 can't go red (case 2) |
| 4 | ticks = Foata normal form (§1) | **HOLDS** (longest-path layering of the first-tick order; unique). Two conventions must be pinned: depth of a derivation with no positive premise (`r :- not s`), and which γ run's ticks W's layers are |
| 5 | W: round contexts + period-2 witness (§2.3, §4 item 6) | **FAILS.** A stable model passes the witness. Vector 04 with "q, r true, nothing undefined" passes every §4 check (case 1) |
| 6 | Dark at 2 by location only (§2.4) | **NARROWED.** It is sound only if the checker re-reads with **its own** reader. "The" location is a choice among errors (a ruler). The converse, a wrong non-dark result on an ill-formed program, needs the checker to re-run well-formedness on every result (§1.4) |
| 7 | Dark at 255 by derivations + declining rule (§2.4) | **HOLDS** for "the violation is real". **FAILS for the converse**: a `clear` result hiding a functional violation passes, because check 7 runs only when 255 is claimed (case 3) |
| 8 | "every step": strip/normalize/settle via `render(N, ε)` (§2.2 table) | **FAILS.** `N = ∅`, `ε = the whole file as trivia` round-trips the bytes (§1.5) |
| 9 | replay in any linear extension rebuilds the root (§4) | **HOLDS, and it is vacuous as a test.** It is a theorem about sets: one acyclicity check covers every extension (case 5: 6 extensions, 1 result) |
| 10 | a diverse checker + DDC suffices (§4 CONJECTURE) | **NARROWED.** Algorithmic diversity doesn't cover shared spec readings, so add known-answer vectors, a different author and a different language (§2) |
| 11 | Order-freedom by K random extensions + reverse (§4) | K is **not** a ruler of done. It is a test-effort parameter for the checker's own determinism. The structural check is §3 |

---

## §1 Certificates that pass while the result is wrong

### §1.1 W: the period-2 witness doesn't certify leastness (case 1)

The program is vector 04: `p :- not q`, `q :- not p`, `r :- not s`, `t :- p`. The well-founded model is `under = {r}`, `over = {p, q, r, t}`, so p, q and t are undefined.

A certificate claiming `{q, r}` true and nothing undefined, with contexts `(under, over) = ({q, r}, {q, r})`, passes every §4 check:
- γ({q, r}) = {q, r}: a stable model is a fixpoint of γ, so a fortiori of γ²;
- the derivations of q and r are valid under that context, and closure holds under it.

F6 says the *final* pair of W's iteration has period 2. Its converse ("any period-2 pair is W's") is false: every fixpoint of γ² has the property, and W's pair is the **least**.

**Fold:** the certificate carries **every** round context from the initial one (`under₀ = F`, or as E fixes it), and the checker verifies each transition `over_k = γ(under_k)` and `under_{k+1} = γ(over_k)` from the start, up to consecutive equality. Leastness then follows from the iteration, not from the witness. The cost is 2 · rounds γ-runs, which is a recomputation, and that is fine for a checker. The period-2 check becomes redundant (it holds at the end automatically), and the certificate shouldn't present it as the evidence.

### §1.2 "All first-tick derivations" is required but not checked (case 2)

The diamond `a→b, a→c, b→d, c→d`. A certificate omitting d's derivation via c passes checks 1–7. CT3 expects a failure, but nothing in §4 produces one.

**Fold:** check 8, **completeness per layer**. For every atom a at layer t, the set of rule instances with head a whose positive premises all lie in layers < t (and whose negative premises hold under the round context) equals the certificate's set for a. This is one naive round restricted per layer, and it subsumes (C) for all but the top layer. It also covers **budget-truncated** runs, where (C) is false by design: then M must equal the naive stage k exactly, which is what per-layer completeness plus "no layer > k" says.

### §1.3 A `clear` that should be Dark at 255 (case 3)

The vector-05 shape: `owner(doc, alex)` and `owner(doc, reed)` under `functional owner(1) -> 2`. A certificate with verdict `clear` passes. Check 7 runs only when the certificate claims 255.

**Fold:** the checker evaluates **every** `functional` declaration over the certified model (over `true` only, under W, matching E's A5) for **every** result. Then it derives the verdict itself: `clear`/`opaque`/`dark` and the exit code. The certificate's verdict is a claim to compare, never an input.

### §1.4 Dark at 2, and its converse

- **"Re-reading the bytes up to that point fails the same way" with which reader?** If it is the compiler's reader, a compiler that rejects a valid file passes its own check. The checker needs **its own reader** for the grammar in question. For pact-text that is §1.2 of my pact-eval audit, re-implemented. For compile it is the checker's own reading of `mirror.mirror`'s grammar (§1.6).
- **"The" location is a choice.** A file with two errors, or a program that is both unsafe and arity-inconsistent, has several valid Dark reports. Which one is printed is an order of checks, i.e. a ruler (a punctuation, in the spec's own §1 terms). The checker accepts the certificate iff the claimed location is **a** genuine error, and the spec names the ordering ruler for the canonical one (e.g. least byte offset; engine checks in a fixed declared order).
- **Engine `Dark`** (`Unsafe { rule, var }`, `Arity`, `Functional`) is re-checkable directly from the program text: the var doesn't occur in a positive body literal, and so on. That is cheap and independent.
- **The converse.** A program that should be Dark (an unsafe rule), for which the compiler printed a model anyway. Without its own safety and arity checks, the checker's naive round over an unsafe rule either mis-evaluates or inherits the compiler's leniency. **Fold:** the checker re-runs every well-formedness check on every input, not only when Dark is claimed.

### §1.5 `render(N, ε)` doesn't certify N

The §2.2 table certifies strip/normalize/settle by "`render(N, ε)` reproduces the bytes". Take `N = ∅` (or any N) and `ε =` the bytes of the file recorded as trivia: the round trip is perfect and N is wrong. Round-tripping shows that (N, ε) jointly **encode** the bytes, not that N is the normal form.

**Fold:** the checker recomputes N itself, as its own `strip` and `nf` applied to the certified parse items. Then it checks `render(N, ε) = bytes` as an *additional* property. For `settle` under a ruler: the ruler's content is a positive program (sealed rulers, D3 lean), so the checker evaluates it and compares the erasure, rather than trusting `shaped_by` + erasure as given.

### §1.6 What the certificate can't express: the inputs to (S) and (C)

(S) and (C) prove `M = lfp(P, F)`. For `mirror compile`:
- **P** is the Horn-clause translation of the grammar (fixpoint math §5.1);
- **F** is the bytes as `char(i, c)` facts.

A bug in either translation is invisible to the certificate: the model would be the correct least model of the wrong program (case 4 shows the fact-side version, where checking against the certificate's own fact list passes and checking against the source fails).

**Fold:** the checker **re-derives P and F from the source** (its own PEG → Horn translation and its own byte → fact encoding), compares their addresses with the header's, and never reads P or F from the certificate.

---

## §2 Independence

**Algorithmic diversity is necessary and not sufficient.** The checker and the engine can fail together through a **shared reading of one text**:
- **§5.0 bytes.** Both implement `enc`, `τ` and the atom-content encoding from the same prose. If the prose is ambiguous, both can resolve it the same wrong way. The root then agrees and is wrong against the spec, so the identity scheme drifts silently.
- **Semantics.** Closed world for facts only; `not` on a predicate with no rules; safety; arity of predicates that occur only in facts; what counts as "intensional" for printing; Ford's functionality as "at most one value" over `true` only. Each is a line of spec two implementers can read alike and wrongly.

Mitigations, all required:
1. **Known-answer vectors for §5.0.** Byte-level worked examples (input → encoded bytes → hash) in the spec, produced by an implementation written from the spec text by someone who wrote neither the engine nor the checker. These are fixtures both must match. A disagreement is a spec defect, resolved by Mara and logged.
2. **Semantic vectors** (`tests/pact/oracles.md`, plus the adversarial set in §6 here) as the shared ground truth, never derived from either implementation's output.
3. **A different author.** Not Reed (engine), and not Seam (auditor). Mara (spec author) or another Pack member writes the checker. The checker reads the spec as its author intended, which is what it should check.
4. **A different language.** **Python** (stdlib plus a pinned `blake3`), at `tests/checker/`. The reasons:
   - it removes the shared toolchain, std collections and crate graph (Wheeler's DDC argues for diverse compilers, and a checker is the same argument one level up);
   - it keeps the checker out of FROZEN's `.rs` gate, where no marker fits it honestly (§4);
   - the repo already runs Python oracles through `uv` (`tests/kintsugi/oracles/*`, this file's script).

   The cost is speed. The checker's naive closure round over a large parse-item model may be slow. **That is measured, not assumed:** the loop's budget ruler decides, and a shard the checker can't finish within budget is reported **UNKNOWN**, never passed.

**What the checker re-derives vs. trusts.**

| re-derives (never reads from the certificate or the compiler) | trusts |
|---|---|
| P and F from the source; well-formedness (arity, safety, declarations); the model by (S), (C) and per-layer completeness; the layers (depth); W's whole context sequence; every `functional` check; the verdict and exit; N via its own strip/nf; the erasure by evaluating the ruler; root, interface, graph and uuid via its own encoder; Dark locations via its own reader | the spec text (pinned by commit); the known-answer vectors; BLAKE3 (the primitive, not our risk); the OS's file read; the Python interpreter |

---

## §3 Order-freedom: what covers every linear extension

The spec's property (replay in any linear extension rebuilds the root) is a theorem. Each replay step adds a head whose premises are present, so every complete replay ends at `F ∪ {heads}`. The address is computed from the **sorted** set, so it doesn't depend on order. Case 5 enumerates all 6 extensions of the diamond: 1 result.

So replaying in K random extensions can't find a defect in a **certificate**. It can find one only in the **checker's implementation** (hidden order dependence: iteration order, early exits, mutation during replay).

**The structural check that covers all extensions:**
1. the derivation graph is acyclic, with every premise in a strictly earlier layer (checks 2 + 4 already imply it);
2. the model address is computed from the sorted set by the checker's encoder;
3. under W, negative premises are judged against the **fixed** round context, never against the replay's current set.

Given these three, "every linear extension" is proved, not sampled.

**K is therefore not a ruler of done.** It is a test parameter of the checker's own property test, which still runs (reverse order + K seeded shuffles; K = 100 as in CT1 is fine). It must not appear in the charter's done-property. "Order-free" in the charter means the three structural conditions.

---

## §4 Admissibility and citation contract for building it

**Shape: ADMIT-NARROWED, Python at `tests/checker/`.**
- **Entry point:** `checker.py <source> <certificate> <record>`. It prints a mirror-text verdict (`pass` | `fail <check> <detail>` | `unreadable <location>`) and exits 0 / 1 / 2.
- **Dependencies:** Python stdlib and `blake3` (version pinned in the script's inline metadata, run with `uv run`). Nothing else.
- **It may not:**
  - import any repo module, or any other oracle;
  - call the `mirror` binary or any process;
  - use the network;
  - use `json` or any serializer (it reads mirror-text with its own line reader, the same HARD RULE in spirit);
  - read any file but its three arguments;
  - sample (every derivation, every layer, every declaration is checked);
  - read P, F or the verdict from the certificate as truth.
- **It must implement:**
  - checks 1–7, corrected per §1: W's whole context sequence (§1.1), check 8 per-layer completeness (§1.2), functional checks on every result (§1.3), its own reader and well-formedness checks (§1.4), its own strip/nf for compile (§1.5);
  - P and F re-derived from the source (§1.6);
  - its own §5.0 encoder, verified against the known-answer vectors.
- **Staging.** Level 1 starts with pact-text certificates (the reader is small), and the compile certificate parts (PEG → Horn, strip/nf) arrive when T1 lands, each as its own commit under this audit.

**`rust/checker`: REFUSE as the trust root.** It is a separate crate and could be made independent, but it shares the toolchain and the language with the engine, and it needs a FROZEN marker that would be untrue (it isn't floor, and it adds no capability). It is admissible later **only** as a second, faster checker, after the Python checker is frozen and measured too slow for the corpus within budget. That is a comma for Alex, under its own audit.

**Citation contract** (commits touching `tests/checker/**` before the freeze). The hook doesn't gate non-`.rs` files, so this is enforced by review:
1. `Checker: within docs/audits/2026-10-29-seam-checker.md §<n>`, naming the check(s) implemented;
2. the red certificate fixture that the commit turns into a correct pass or fail. Each check lands with at least one **wrong** certificate it rejects: the §6 adversarial set;
3. an author who is not the engine's author;
4. `No repo imports. No json. No subprocess. stdlib + blake3 only.`

**Outside this audit (needs a new one):**
- any dependency beyond stdlib + blake3;
- any import of repo code;
- calling the compiler;
- trusting a certificate field as input;
- sampling;
- a Rust checker;
- any change after the freeze, except by §5.

---

## §5 What "frozen" means, and how a checker bug gets fixed

**Frozen, technically.**
- The charter pins the checker by **tree address**: `git rev-parse <commit>:tests/checker`.
- It also pins the **certificate spec** by commit. The checker implements a format, and a spec change is a checker change.
- The known-answer vectors are pinned the same way.
- Before every measurement, the loop recomputes those addresses. Any difference stops the loop with a comma, and no measurement is reported.
- The loop may read the checker and run it. It may never write under `tests/checker/`, the spec, or the vectors.

**Two kinds of checker bug:**
- **False reject** (a correct certificate fails). The loop sees red on output it believes correct. It **must not** patch the checker. It reports the shard as failed, and files a comma with the certificate and its argument.
- **False accept** (a wrong certificate passes). Found by Seam, an adversarial fixture or a reviewer. It is the dangerous one, because "done" may already have been claimed on it.

**The fix procedure (both kinds):**
1. **A red fixture independent of the compiler.** It is a hand-built certificate, citing the spec clause the checker misreads. "The compiler's output fails it" is **never** sufficient grounds: that is the loop teaching the test to the compiler.
2. **Seam amends this audit** (📝) with the bug, the clause and the fixture.
3. **The checker's author fixes it.** Not the loop, and not the engine's author.
4. **Alex signs the new pin**: a signed commit, or an approval recorded in-transcript with its date, updating the charter's checker address. Nobody else can move the pin.
5. **Re-measure the whole corpus** with the new checker. Results under the old checker stay in `STATE.md`'s history, marked with the old pin, never rewritten (R11's rule for revocation, applied to the checker).
6. **A false-accept fix also re-opens every "done" claimed under the old pin.** A level is done only under the checker that's pinned now.

**Watch item.** A checker change that turns previously red compiler outputs green, and only those, is the signature of the test bending to the compiler. Seam reviews every such diff, and its audit amendment must show the spec reading, not the compiler's behaviour, as the reason.

---

## §6 The adversarial set (seeds; to land as fixtures with the checker)

Each is a certificate the checker must **reject**:
1. A stable model with a period-2 witness (§1.1).
2. A missing first-tick derivation (§1.2).
3. `clear` hiding a functional violation (§1.3).
4. A spurious fact in the certificate (§1.6).
5. A Dark location that is readable.
6. An unsafe program with a model.
7. `N = ∅`, with ε = everything (§1.5).
8. A layer off by one.
9. A circular derivation pair.
10. A budget-truncated run claiming the fixed point.
11. A correct certificate whose model address was computed over a different sort order.

Cases 1–4 are computed in `certificate_crowbar.out`. The rest are specified here and land as fixtures with the checks they exercise.

## §7 For Mara, Reed and Alex

- **Mara (spec):**
  - check 8 (per-layer completeness);
  - W's full context sequence instead of the period-2 witness as evidence;
  - functional and well-formedness checks on every result, with the verdict derived by the checker;
  - N recomputed (not only round-tripped);
  - P and F re-derived from source;
  - the Dark-location ordering ruler;
  - the depth convention for premise-free rules and W's layers;
  - §5.0 known-answer vectors;
  - "order-free" defined structurally (§3).
- **Reed (charter draft):** pin the checker tree, the certificate spec commit and the vectors. Remove K from the done-property. Add the §5 fix procedure and "UNKNOWN when over budget, never pass".
- **Alex:** who writes the checker (not Reed, not Seam). Python as the trust root (my lean) versus a Rust second checker later. Signing the checker pin is yours alone.

*— Seam*
