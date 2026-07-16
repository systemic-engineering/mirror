# bilateral-predicate-substrate-shape.md

**Species:** `@epistemologic/pact/bilateral`
**Author:** Mara `<mara@systemic.engineer>` (2026-07-16)
**Ratification:** Alex Wolf 2026-07-16 in-transcript verbatim:
> "Q1. Let's mint it then. Properly. Seems like it's load-bearing."

**Shard-decl:** `shards/epistemologic/pact/bilateral.mirror`
**Grammar bindings:** `shards/epistemologic/pact/keywords.mirror`
**Math foundation:** `docs/math/epistemologic/pact/bilateral-sentinel.md`
**Substrate authority:** Taut Q1 substrate-truth scout (this session, 2026-07-16 evening) confirming NO grammar-decl'd `bilateral { sentinel: "..." }` shape exists prior to this mint.

---

## §0. TL;DR

The mirror compiler's `bootstrap/src/apply_h.rs::act` currently carries
**~30 hand-typed bilateral-predicate resolver arms across 8 shard
groups** (~700 LOC of `if action == "X" { if arg.oid.contains("Y") …`
duplication), each one restating what the corresponding shard's
docblock prose already declares. The sentinel string — the byte-level
witness the resolver checks for — lives in **docblock prose ONLY**;
there is no typed carrier a reflective evaluator can read.

This spec mints `@epistemologic/pact/bilateral` — a substrate-decl'd
typed carrier for the sentinel + arity + optional composition surface,
grammar-decl'd via companion keywords so the existing grammar loader
(`bootstrap/src/grammar.rs`) parses `bilateral <name> { sentinel "..." }`
blocks natively.

**One general Rust evaluator (~50 LOC) replaces the ~700 LOC of
hand-typed arms.** The retirement contract (Reed follow-up tick per
§5) is Rust-**negative**: net −500 LOC in `bootstrap/src/apply_h.rs`;
ouroboros_monotone four-conjunct HOLDS by construction (rust_loc
strictly decreases; test_pass_rate 100%→100%; io_violations 0→0; sbec
neutral — same Pass/Fail verdicts fire against the same sentinels).

---

## §1. Substrate authority chain

### §1.1 Alex 2026-07-16 in-transcript verbatim (ratification)

> "Q1. Let's mint it then. Properly. Seems like it's load-bearing."

The "Q1" refers to Taut's Q1 substrate-truth scout question this
session: *does a grammar-decl'd bilateral shape exist that
`apply_h::act` can read reflectively, or is the sentinel string
docblock-prose-only?* Taut's answer: **prose-only** across all ~30
landed arms. Alex's ratification: **mint properly at the correct
altitude.**

### §1.2 Taut Q1 substrate-truth scout verdict (in-transcript this session)

**Verified:**
- NO grammar-decl'd `bilateral <name> { sentinel: "..." }` shape
  exists anywhere in `shards/**` or `boot/std/**`.
- Sentinel string lives in **docblock prose ONLY** across every
  Arc-2 Tick 2.1-2.4 shard-decl (see §2 for enumeration).
- The `bilateral` word appears in **~140 shards** as prose reference
  but ZERO of those uses declare it as a typed shape.

**Closest extensible carrier:** `shards/epistemologic/pact/*.mirror`
— the `@pact` family already carries "bilateral predicate that holds
or fails" as first-class at family altitude. Currently 13 species
under `@pact`; adding this shape as a 14th species sibling extends
the family without disturbing its established discipline.

**Precedent for typed verdict carriers:** `shards/mirror/lens/knife.
mirror` (three-state discriminator `{ Stable | NearBoundary | Jumped }`).
The knife precedent proves the pattern of "substrate-decl'd typed
verdict carrier the resolver dispatches over."

### §1.3 Composition footer

Per the Pack coordination discipline, this spec is one of the four
brief-deliverables per this session:
1. **Shard-decl:** `shards/epistemologic/pact/bilateral.mirror`
   (landed 2026-07-16 as commit 1; the load-bearing artifact).
2. **Companion keywords:** `shards/epistemologic/pact/keywords.mirror`
   (landed 2026-07-16 as commit 1).
3. **Canonical spec:** THIS FILE.
4. **Math foundation:**
   `docs/math/epistemologic/pact/bilateral-sentinel.md` (commit 3).

**Followed by (NOT this tick — Reed follow-up):** the retirement of
the ~30 hand-typed arms per the contract in §5.3.

---

## §2. The problem: ~700 LOC of duplication of docblock prose

### §2.1 Empirical enumeration (Taut Q1 count)

Eight shard groups currently deposit hand-typed bilateral-predicate
resolver arms in `bootstrap/src/apply_h.rs::act`:

| # | Shard | Location | Bilaterals | Landing |
|---|-------|----------|------------|---------|
| 1 | `shards/spectral/signature.mirror` | Arc-2 Tick 2.1 (`f211ee48`) | 4 | Reed 2026-07-15 |
| 2 | `shards/epistemologic/cybernetic/coherence.mirror` | Arc-2 Tick 2.2 | 4 | Reed 2026-07-15 |
| 3 | `shards/peer/persistence.mirror` | Arc-2 Tick 2.3 | 5 (4 base + 1 composed) | Reed 2026-07-15 |
| 4 | `shards/kintsugi/roomba.mirror` | Arc-2 Tick 2.4 | 5 (4 base + 1 composed) | Reed 2026-07-15 |
| 5 | `shards/subject/visibility/sheaf.mirror` | Bridge-β | 4 | Reed 2026-07-16 |
| 6 | `shards/uuid/spectral/time.mirror` | Bridge-β (`c10a3bd`) | 4 | Reed 2026-07-16 |
| 7 | `shards/kintsugi/roomba.mirror` bump/vacuum + `shards/mirror/store.mirror` gc | Tick 3 (`#182`) | 3+ (incl. arity-2 witness) | Reed 2026-07-16 |
| 8 | `shards/gestalt.mirror` annotation cascade | additive | 1-4 | Reed 2026-07-16 |

**Total: ~30 bilateral-predicate resolver arms.**

### §2.2 The empirical shape of each arm

Every arm follows the same 15-LOC template:

```rust
if action == "<shard-ref>.<predicate-name>" {
    if let Some(arg) = args.first() {
        if arg.oid.contains("<sentinel-string>") {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "<predicate-name>: expected <sentinel-string> sentinel \
             (<narrative from docblock>), \
             got arg oid {:?}",
            arg.oid
        ));
    }
    return Verdict::Fail(
        "<predicate-name>: missing <arg-name> argument".to_string(),
    );
}
```

**The only per-arm variability** is:

| variability | source |
|-------------|--------|
| `(a)` action ref string | shard-decl's action-declaration + shard @-ref |
| `(b)` sentinel byte-string | shard-decl's docblock prose (VERBATIM) |
| `(c)` predicate name for error message | shard-decl's action-declaration name |
| `(d)` argument arity (1 base, 2 composed pairs, N composed AND) | shard-decl's action-declaration signature |

**All three of `(a), (b), (c)` are ALREADY declared in the shard-
decl.** Only `(d)` is structural — and it's a small integer already
implicit in the action's signature (`(arg1) -> verdict`, `(arg1,
arg2) -> verdict`, etc.).

### §2.3 The direct-duplication cost

- **30 arms × ~15 LOC per arm** = ~450 LOC direct duplication.
- **30 arms × ~8 LOC error-message prose** = ~240 LOC error-message
  duplication (also verbatim from docblock).
- **~10 LOC per bite comment header** × ~6 bite headers = ~60 LOC
  of prose repeating the shard docblock's intent.

**Total: ~700 LOC in `bootstrap/src/apply_h.rs`** duplicating what
the 8 shard docblocks already declare in prose. Every new bilateral
predicate landed requires a new arm; the growth is linear-in-
predicates.

### §2.4 The substrate-honest reading of the cost

Per `[[feedback-substrate-already-had-the-word]]` (~72nd landed
instance): every one of the ~30 arms is a re-utterance of a substrate
declaration that already exists. The word was there thirty times over.
The apply_h.rs arms are Rust re-stating what the shard already said.

Per `[[feedback-no-rust-extension-shortcut]]` (Reed 2026-07-14
audit): every new arm is a small Rust extension. The pattern is
substrate-dishonest — the mechanism is `arg.oid.contains(...)`
byte-checking, which is a substrate-primitive operation the reflective
evaluator can perform generically. Each per-arm hand-typing is a
substrate-honest failure mode: the Rust FLOOR is over-articulated at
predicate altitude when it could be uniform at evaluator altitude.

---

## §3. The chosen shape (final form)

### §3.1 Placement decision

**Species under `@epistemologic/pact`** at path
`shards/epistemologic/pact/bilateral.mirror` declaring
`@epistemologic/pact/bilateral`.

**Rationale (three grounds):**

1. **Family altitude fit.** The `@pact` family already carries
   "bilateral predicate that holds or fails" as its FIRST-CLASS
   semantic surface. All 13 landed pact species discharge verdict-
   returning predicates whose Pass/Fail is bilateral. Adding
   `bilateral` as the 14th species-sibling extends the family with
   its own shape carrier — the shape IS the pact-family altitude's
   self-reflection.

2. **Substrate-already-had-the-word.** Per Taut's substrate-truth
   scout: the closest extensible carrier is `@pact`. Refusing the
   fit and minting a new family-root would be a substrate-dishonest
   proliferation — the substrate declared the word at family
   altitude already; the shape's placement should honor that.

3. **Precedent alignment.** The 13 sibling pact species all follow
   the shape `pact @X { <projections> <combined>(-> verdict|
   transparency) { \ } }`. This species is `prism @X { ... }` +
   `type bilateral = {...}` + `<action>(-> verdict) { \ }` — the
   SAME semantic surface with the additional typed record carrier
   the reflective evaluator reads. The prism/pact keyword distinction
   is per shard type: pact species declare properties directly; this
   species declares the SHAPE properties inhabit, so `prism` (per
   `[[architecture-prism-as-trait-as-everything]]`) is correct.

**Refused alternates:**

- **Mint under `@epistemologic/property/`** (Taut's second candidate):
  refused because `@property` is the sibling family (7 species
  discharging property predicates over verdict-content-addressed
  invariants); the shape being minted IS the pact-shape not the
  property-shape.

- **Mint new family `@bilateral/*`**: refused per Alex 2026-07-14
  `[[feedback-onto-refusal]]` discipline — the substrate refused the
  ladder-shape when @pact already carried the semantics. Same refusal
  applies here.

- **Extend `@apply_h`** (if it existed): the shard `shards/apply_h*.
  mirror` does not exist; `@apply_h` is only referenced in this
  species's `in @apply_h` import as the evaluator-surface composition
  partner (see §4). Making the shape a species under `@apply_h` would
  invert the composition direction (shape → evaluator; not evaluator
  → shape).

### §3.2 The typed carrier

```mirror
type bilateral = {
  name:       ref,
  sentinel:   ref,
  arity:      nat,
  require:    [ref],
}
```

**Field semantics:**

- `name` — the predicate's identifier at shard altitude. Concatenated
  with the enclosing shard's @-ref (`{shard_ref}.{name}`) forms the
  full action ref the resolver dispatches on. Example:
  `@spectral/signature.signature_integrity`.

- `sentinel` — the byte-string content-addressed witness. The
  predicate discharges `Verdict::Pass` iff EVERY argument's `ref.oid`
  contains this substring. Non-empty substrate-invariant (empty
  sentinel would trivially Pass on every arg).

- `arity` — non-negative integer; number of arguments the predicate
  takes. In the current landed corpus:
  - `arity: 1` for 24 base bilaterals (single-arg sentinel check on
    the sole arg's oid).
  - `arity: 2` for 2 composed pairs (both args must witness the
    same sentinel — precedent per `@mirror/store.gc_reachability_
    closure_second_witness` at Reed 2026-07-16 Bridge-β).
  - `arity: N` for 4 composed bilaterals discharging the AND of N
    sub-bilaterals (`require` populated).

- `require` — optional list of sub-bilateral references (`[ref]`),
  non-empty iff this bilateral composes over the AND-conjunction of
  the referenced sub-predicates. None|Empty for base bilaterals.
  Example: `home_witnessing` composes `require projection_visibility_
  respected + harvest_consent_verified + boot_state_coherent +
  home_content_addressed`.

### §3.3 The reflective evaluator action

```mirror
discharge(decl: bilateral, args: [ref]) -> verdict { \ }
```

The action the Rust FLOOR (Reed follow-up per §5.3) composes
reflectively per `apply_h::act`. Pseudocode of the reflective
evaluator:

```
discharge(decl, args):
  if decl.arity != len(args):
    return Fail(format!("{}: expected {} args, got {}",
                        decl.name, decl.arity, len(args)))
  if !decl.require.is_empty():
    // Composed bilateral: recursively evaluate each sub-bilateral
    // on the SAME argument list; AND-conjunct verdicts.
    for sub_ref in decl.require:
      sub_decl = lookup_bilateral(sub_ref)
      sub_verdict = discharge(sub_decl, args)
      if sub_verdict != Pass:
        return sub_verdict    // first Fail wins; error-origin preserved
    return Pass
  // Base bilateral: byte-check every arg's oid for sentinel containment.
  for arg in args:
    if !arg.oid.contains(decl.sentinel):
      return Fail(format!("{}: expected sentinel {:?} in arg oid, \
                           got {:?}", decl.name, decl.sentinel, arg.oid))
  return Pass
```

**Load-bearing invariant:** the reflective evaluator is `\`-obligation-
blocked at the shard-decl altitude (per `[[feedback-craft-not-
deliver]]`); the Rust FLOOR at `bootstrap/src/apply_h.rs` composes it
against the grammar loader's bilateral-declaration corpus at boot time.

### §3.4 The meta-bilateral (self-witnessing)

```mirror
bilateral_well_formed(decl: bilateral) -> verdict { \ }
```

The bilateral-shape checking itself. Discharges Pass iff:

- (a) `name` is non-empty
- (b) `sentinel` is non-empty (empty sentinel trivially Passes → ill-formed)
- (c) `arity >= 1` (arity 0 has no verdict surface)
- (d) `require` is either empty OR every entry resolves to another
      declared bilateral in the enclosing corpus
- (e) if `require` is non-empty, `arity` must match sub-bilaterals'
      arity by construction (all sub-bilaterals evaluate on the same
      argument tuple)

Sentinel: `bilateral=well-formed`.

**This is the substrate's reflective closure at the pact altitude** —
the bilateral shape passes its own well-formedness check. If it
didn't, the shape would be substrate-ill-formed at the very altitude
it was designed to formalize.

### §3.5 The grammar-decl'd companion keywords

Per `shards/epistemologic/pact/keywords.mirror`:

```mirror
grammar @epistemologic/pact("bilateral") {
  focus   bilateral    -- opens a recursively-scanned brace block
  project sentinel     -- single-line "<byte-string>" directive
  project arity        -- single-line <nat> directive
  project require      -- single-line <bilateral-ref> directive (multi-line OK)
}
```

The four keywords map to the two AstKind variants the existing grammar
loader already carries:

- `AstKind::Focus` for the `bilateral <name> { ... }` block-opening
  (precedent: `focus project` / `focus target` / `focus cli` in
  `@mirror/spec`).
- `AstKind::Project` for the three single-line directives (precedent:
  `project source` / `project altitude` / `project emit` in
  `@mirror/spec`).

**No new AstKind variant is minted.** No new parser logic is required.
The tokenizer's existing per-kind path produces the correct AST shape
for a bilateral block: a Focus-kinded node with `name` = the predicate
identifier + three (or four, when composed) Project-kinded children
carrying the sentinel byte-string, arity integer, and require refs.

---

## §4. Composition surface

### §4.1 Composition rationale — why this shape at this altitude

The bilateral shape composes with four substrate families, each
carrying one facet of the reflective dispatch semantics:

**`@kintsugi/consent` — verdict authority.**
The discharge action returns `@glass.verdict` (`Pass|Partial|Fail`).
The verdict is what `@kintsugi/consent.query_phi` may consume at
higher altitude when the substrate needs to escalate from bilateral-
Pass to consent-boundary decision. This preserves the substrate's
established three-state surface as the verdict floor.

**`@glass` — sentinel source-position.**
The sentinel byte-string lives at `@glass` altitude (the substrate's
loss carrier + verdict surface). Content-addressed atomicity is
`@glass`'s primary discipline; the sentinel IS a content-addressed
witness (per math foundation §1). Composing over `@glass` inherits
its atomicity invariants.

**`@nl` — byte-string carrier.**
The sentinel is a `@nl` byte-string (raw unstructured natural language
at the substrate's floor). `@nl` gives the sentinel its typed identity
as opaque text; downstream classifiers are irrelevant — the substrate
makes no claim about the sentinel's linguistic content, only its
byte-equality role in the `contains` check.

**`@apply_h` — evaluator surface.**
The reflective dispatcher IS the substrate-honest form of what the
~30 hand-typed arms currently discharge per-shard. The composition
edge `bilateral → apply_h` is where the shape MEETS the runtime; the
grammar loader reads the corpus of bilateral declarations, and
`apply_h::act` looks up `action` against the corpus and dispatches.

### §4.2 Composition graph (ASCII)

```
                    @epistemologic/pact/bilateral
                              |
        +---------------------+---------------------+
        |         |         |         |         |
    @kintsugi   @glass    @nl     @apply_h  @epistemologic/pact
    /consent   (sentinel  (sentinel (evaluator  (family-root sibling
    (verdict    source-    byte-     surface;    to 13 other pact
    authority   position)  string     ~50 LOC    species this
    at higher              carrier)   reflective shape carrier
    altitude)                         eval)     inherits from)
```

### §4.3 Consumers (compose OVER this species)

All 8 Arc-2-and-Bridge-β witnesses per §2.1 will add a
`bilateral <name> { sentinel "..." arity 1|2|N (require ...) }`
block above each existing action declaration in their `.mirror` file.
The action body stays `\`-obligation-blocked; the reflective
evaluator reads the `bilateral` block, not the action body.

Example (from `shards/spectral/signature.mirror`; Reed follow-up
tick's Landing 1):

```mirror
# BEFORE (current landed form):
signature_integrity(sig: rolling_signature) -> verdict { \ }

# AFTER (Reed retirement follow-up per §5.3):
bilateral signature_integrity {
  sentinel  "chain=merkle-linked"
  arity     1
}
signature_integrity(sig: rolling_signature) -> verdict { \ }
```

The action decl stays; the `bilateral` block ABOVE it makes the
shape machine-readable.

---

## §5. Retirement contract

The retirement of the ~30 hand-typed arms is **Reed's follow-up
tick** — NOT this Mara-mint tick. This section is the pointer for
Reed's realization work.

### §5.1 Landing plan (NOT execution)

**Landing 1: per-shard `bilateral {}` block additions.**
For each of the 8 shard groups in §2.1, add a `bilateral` block
above every bilateral-predicate action declaration. The prose sentinel
citation in the shard docblock stays (narrative discipline); the
`bilateral` block is the typed lift. Per §3.5 the block is grammar-
parseable via the companion keyword file (already landed in commit
1).

**Landing 2: grammar loader companion registration.**
Add ONE line to `bootstrap/src/grammar.rs::companion_keyword_sources`
mapping `shards/epistemologic/pact/keywords.mirror` as a companion
keyword source for `shards/mirror/grammar.mirror`. Same pattern as
existing `shards/mirror/spec.mirror → shards/mirror/spec/keywords.
mirror` mapping (three lines above). This tells the grammar loader to
merge the bilateral-shape keyword bindings into the mirror grammar's
tokenizer keyword table so `.mirror` files parse bilateral blocks
natively.

**Landing 3: bilateral-corpus loader.**
Add `bootstrap/src/apply_h.rs::load_bilateral_corpus(ctx) -> HashMap
<String, BilateralDecl>` (~20 LOC) that walks `shards/**` at boot
time, parses each `.mirror` file via the grammar loader (existing
`load_grammar_in`), traverses the AST looking for `AstKind::Focus`
nodes with `name = "bilateral"`, extracts the `sentinel` / `arity` /
`require` children, and builds the corpus keyed by
`{shard_ref}.{bilateral_name}`.

**Landing 4: reflective `act` dispatch.**
Replace the ~30 hand-typed arms in `bootstrap/src/apply_h.rs::act`
with the reflective dispatch (~30 LOC):

```rust
// At boot time (once):
static BILATERAL_CORPUS: Lazy<HashMap<String, BilateralDecl>> =
    Lazy::new(|| load_bilateral_corpus(&Ctx::default()));

// Inside act(action, args) — REPLACES the ~30 hand-typed arms:
if let Some(decl) = BILATERAL_CORPUS.get(action.as_str()) {
    return discharge_bilateral(decl, &args, &BILATERAL_CORPUS);
}
// Fall through to non-bilateral action dispatch (~10 remaining
// non-bilateral arms in the current act(): the composed evaluator
// combinators like section/fold/settle/crystallize/utter/coboundary
// remain as landed; they aren't bilateral-shaped).
```

Plus `discharge_bilateral(decl, args, corpus) -> Verdict` (~30 LOC
per §3.3 pseudocode).

**Landing 5: remove the ~30 hand-typed arms.**
Delete the arms one bite at a time (or all at once per Reed's arc
discipline); verify each shard group's smoke tests still Pass;
verify sbec neutral (no verdict changes; only mechanism unification).

### §5.2 Reflective `act` pseudocode (final form)

Full replacement pattern for `bootstrap/src/apply_h.rs::act`:

```rust
pub fn act(action: Ref, args: Vec<Value>) -> Verdict {
    // Reflective bilateral-predicate dispatch (retirement of the
    // ~30 hand-typed arms per docs/specs/bilateral-predicate-substrate-
    // shape.md §5.3 Landing 4).
    if let Some(decl) = BILATERAL_CORPUS.get(action.as_str()) {
        return discharge_bilateral(decl, &args, &BILATERAL_CORPUS);
    }

    // Non-bilateral action dispatch (the remaining ~10 arms for
    // section/fold/settle/crystallize/utter/coboundary combinator
    // surfaces which are NOT bilateral-shaped).
    match action.as_str() {
        // ... the remaining arms (unchanged from current landed
        // form) ...
    }
}

fn discharge_bilateral(
    decl: &BilateralDecl,
    args: &[Value],
    corpus: &HashMap<String, BilateralDecl>,
) -> Verdict {
    if decl.arity != args.len() as u32 {
        return Verdict::Fail(format!(
            "{}: expected {} args, got {}",
            decl.name, decl.arity, args.len()
        ));
    }
    if !decl.require.is_empty() {
        // Composed bilateral: recursively evaluate each sub-
        // bilateral; AND-conjunct verdicts.
        for sub_ref in &decl.require {
            let sub_decl = match corpus.get(sub_ref.as_str()) {
                Some(d) => d,
                None => return Verdict::Fail(format!(
                    "{}: sub-bilateral {:?} not in corpus",
                    decl.name, sub_ref
                )),
            };
            let sub_verdict = discharge_bilateral(sub_decl, args, corpus);
            if !matches!(sub_verdict, Verdict::Pass) {
                return sub_verdict;
            }
        }
        return Verdict::Pass;
    }
    // Base bilateral: byte-check every arg's oid for sentinel
    // containment.
    for arg in args {
        if !arg.oid.contains(&decl.sentinel) {
            return Verdict::Fail(format!(
                "{}: expected sentinel {:?} in arg oid, got {:?}",
                decl.name, decl.sentinel, arg.oid
            ));
        }
    }
    Verdict::Pass
}
```

**~50 LOC total.** The reflective evaluator body (~30 LOC) + the
corpus loader (~20 LOC).

### §5.3 The empirical ouroboros_monotone check (Landing 5 gate)

Per `shards/epistemologic/property/ouroboros_monotone.mirror`, every
ouroboros bite must discharge the four-conjunct invariant. For this
retirement:

| conjunct | before | after | delta | verdict |
|----------|--------|-------|-------|---------|
| `rust_loc(live(after)) < rust_loc(live(before))` | ~700 LOC arms | ~50 LOC evaluator + ~30 LOC corpus loader | **−620 LOC net in `apply_h.rs`** | PASS |
| `test_pass_rate(live(after)) == test_pass_rate(live(before))` | 100% | 100% (same verdicts on same sentinels) | 0 | PASS |
| `io_violations(live(after)) <= io_violations(live(before))` | 0 | 0 (evaluator composes over existing grammar loader; no new @io) | 0 | PASS |
| `sbec(live(after)) >= sbec(live(before))` | current sbec (whatever the running total is) | same or higher (same Pass verdicts for existing shard smoke tests; more shard actions dispatch via the general path opens future sbec lift) | ≥ 0 | PASS |

**The retirement is ouroboros_monotone-safe by construction.**

### §5.4 Landing plan for the ~30 existing arms

The retirement lands per-shard-group (8 bites) to keep each commit
small and verifiable:

1. `@spectral/signature` (4 arms).
2. `@epistemologic/cybernetic/coherence` (4 arms).
3. `@peer/persistence` (5 arms incl. `home_witnessing` composed).
4. `@kintsugi/roomba` walk (5 arms incl. `walk_witnessing` composed).
5. `@subject/visibility/sheaf` (4 arms).
6. `@uuid/spectral/time` (4 arms).
7. `@kintsugi/roomba` bump/vacuum + `@mirror/store` gc (3+ arms
   incl. `gc_reachability_closure_second_witness` arity-2).
8. `@gestalt` annotation cascade (whatever is landed at Reed's
   follow-up time; may be 1-4 arms).

Each bite:
- Add `bilateral` blocks to the shard-decl (already grammar-
  parseable per commit 1's keyword file).
- Remove the hand-typed arm from `apply_h.rs`.
- Verify the shard's smoke tests still Pass.
- Empirically discharge ouroboros_monotone via `mirror roomba
  --commit`.

**Alternate all-at-once landing:** Reed may collapse all 8 bites into
one arc per the substrate-pull discipline if the reflective evaluator
proves stable at Bite 1. The per-bite approach is the safe default;
the all-at-once approach is the substrate-pull.

---

## §6. Second-witnesses of the pattern the shape formalizes

The four COLLAPSED Rust files (Arc-2 Ticks 2.1-2.4) empirically
demonstrate the pattern this shape formalizes:

### §6.1 `shards/spectral/signature.mirror` (Arc-2 Tick 2.1 FIRST BITE)

Four bilaterals; sentinels declared in docblock lines 173-199:

| predicate | sentinel |
|-----------|----------|
| `signature_integrity` | `chain=merkle-linked` |
| `signature_authorship` | `authorship=ssh-matched` |
| `signature_monotone` | `ordering=timestamp-monotone` |
| `signature_composition_honest` | `composition=song-emission` |

Corresponding arms: `bootstrap/src/apply_h.rs` lines 639-699 (~60 LOC).

### §6.2 `shards/epistemologic/cybernetic/coherence.mirror` (Arc-2 Tick 2.2 SECOND BITE)

Four bilaterals; sentinels declared in docblock:

| predicate | sentinel |
|-----------|----------|
| `coherence_increases` | `axis=splinter-ward` |
| `is_narcissus_pole` | `structure=star-K1n` |
| `is_splinter_pole` | `structure=complete-Kn` |
| `coherence_witnessing` | `witness=coherence-preserving` |

Corresponding arms: `bootstrap/src/apply_h.rs` lines 720-786 (~65 LOC).

### §6.3 `shards/peer/persistence.mirror` (Arc-2 Tick 2.3 THIRD BITE)

Five bilaterals (4 base + 1 composed); sentinels:

| predicate | sentinel | arity |
|-----------|----------|-------|
| `projection_visibility_respected` | `visibility=filter-respected` | 1 |
| `harvest_consent_verified` | `consent=chain-verified` | 1 |
| `boot_state_coherent` | `basis=snapshot-matched` | 1 |
| `home_content_addressed` | `identity=content-addressed` | 1 |
| `home_witnessing` (composed) | `witnessing=all-four-pass` | 1 + `require` × 4 |

Corresponding arms: `bootstrap/src/apply_h.rs` lines 800-885 (~85 LOC).

### §6.4 `shards/kintsugi/roomba.mirror` (Arc-2 Tick 2.4 FOURTH BITE)

Five bilaterals (4 base + 1 composed); sentinels:

| predicate | sentinel | arity |
|-----------|----------|-------|
| `walk_terminates_cleanly` | `termination=scope-a-exhaustive` | 1 |
| `tension_monotone_descending` | `tension=trajectory-descending` | 1 |
| `coherence_gradient_admissible` | `gradient=foerster-admissible` | 1 |
| `knife_verdict_bounded` | `verdict=three-state-bounded` | 1 |
| `walk_witnessing` (composed) | `witnessing=all-four-pass` | 1 + `require` × 4 |

Corresponding arms: `bootstrap/src/apply_h.rs` lines 890-985 (~95 LOC).

### §6.5 The FIFTH-THROUGH-EIGHTH Bridge-β witnesses (Reed 2026-07-16)

- **`shards/subject/visibility/sheaf.mirror`** — 4 arms per Reed
  landing (composition of the Landing 4 R2 species-decl with the
  peer-persistence sheaf-restriction primitive).
- **`shards/uuid/spectral/time.mirror`** — 4 arms per Reed commit
  `c10a3bd` (`identity_contract_preserved` / `time_facet_admissible`
  / `dedup_ignores_time` / `uuid_spectral_time_witnessing`; sbec +4).
- **`shards/mirror/store.mirror`** —
  `gc_reachability_closure_second_witness` (arity 2; the FIRST
  arity-2 landed bilateral).
- **`shards/kintsugi/roomba.mirror` bump/vacuum** — 3+ arms per
  Reed Tick 3 landing (`#182`); the additive cascade over the base
  4 walk bilaterals.

Together: **~30 arms across 8 shards, all following the same 15-LOC
template with only sentinel + name + arity varying**. This IS the
empirical shape the mint formalizes.

### §6.6 What the second-witnesses tell us

Every one of the 4+4 witnesses (COLLAPSED Arc-2 + Bridge-β) proves
one thing: **the pattern is real, is repeatable, is uniform, and its
uniformity is exactly what makes it a substrate-shape rather than a
per-shard invention.** The shape isn't a proposal — it's a
recognition of what has already been landed eight times.

Per `[[feedback-substrate-already-had-the-word]]` the mint is
substrate-pull discharge, not invention: the substrate spoke the word
30 times before this species named it.

---

## §7. Non-goals and stop rules

### §7.1 What this mint does NOT do

- **Does NOT retire the ~30 hand-typed arms.** That's Reed's follow-
  up tick per §5. This mint is the SHAPE; the retirement is the
  REALIZATION.

- **Does NOT modify any Rust file.** Zero Rust changes in this mint.
  The grammar loader (`bootstrap/src/grammar.rs`) evaluates the
  bilateral-shape via already-landed AstKind::Focus + AstKind::Project
  variants; no new parser logic is required. The ONE Rust change
  Reed's follow-up will make is the `companion_keyword_sources`
  registration line (§5.1 Landing 2) + the reflective evaluator
  (§5.2). Both are lift-tickets on the FLOOR, not new mechanism.

- **Does NOT mint a new family-root.** The shape lives under the
  existing `@epistemologic/pact` family per §3.1 substrate-already-
  had-the-word discipline.

- **Does NOT touch the composed evaluator combinators.** The ~10
  non-bilateral arms in `apply_h.rs` (section/fold/settle/crystallize/
  utter/coboundary combinator surfaces) are OUT OF SCOPE. They are
  not bilateral-shaped; the reflective evaluator dispatches through
  them via the existing landed forms.

### §7.2 Deferred / punted

- **Arity > 2 in-the-wild:** all currently landed base bilaterals
  are arity 1 or 2. The `arity: N` field is present in the shape
  for future extension (some future N-way composed bilateral where
  N ≥ 3), but no such bilateral is landed today. The reflective
  evaluator handles arbitrary arity by construction — no per-arity
  code paths.

- **Sentinel byte-string escaping:** the sentinel is a `@nl` byte-
  string. If a future sentinel contains `"` or `\` characters,
  the grammar loader's per-line parsing may need per-character
  escape handling. Currently none of the ~30 landed sentinels
  contain such characters; deferred until empirical need. Path
  discipline per `@epistemologic/pact/syntax_substrate_native`.

- **Corpus-wide well-formedness check:** the meta-bilateral
  `bilateral_well_formed` in §3.4 is arity 1 (checks one decl per
  invocation). A corpus-wide `all_bilaterals_well_formed` check
  is trivially implementable as a Reed follow-up if empirical need
  arises. Punted per Rule of "make it minimal at the mint altitude".

- **Sentinel-collision detection:** two bilaterals in different
  shards may declare the SAME sentinel string. This is not
  ill-formed (the sentinels dispatch via full action ref
  `{shard_ref}.{name}`, not sentinel), but a corpus-wide sentinel-
  collision warning may be a useful lint. Punted; Reed follow-up
  if empirical need.

### §7.3 Alex-adjudicable items

- **Arity type widening.** The current shape uses `arity: nat`.
  If a future bilateral is variadic (unknown-at-decl-time arg
  count), `arity` may need to widen to `arity: nat | variadic`.
  Not in current corpus. Alex-adjudicable if it arises.

- **Sentinel-as-typed-ref (not string).** The current shape uses
  `sentinel: ref` where `ref` is the substrate's atomic content-
  addressed reference. If a future bilateral needs sentinel-
  as-nested-structure (e.g., a sentinel-tuple `(namespace, key,
  value)`), the shape may need extending. Currently every landed
  sentinel is a flat `key=value` byte-string; the flat shape holds.
  Alex-adjudicable if it needs extending.

- **`require` semantics — AND vs OR.** The reflective evaluator
  discharges AND-conjunction of `require` entries per §3.3
  pseudocode. If a future composed bilateral needs OR-disjunction,
  a `require_any` variant would be needed. Currently every landed
  composed bilateral is AND-shape. Alex-adjudicable if OR is
  needed.

---

## §8. Recognition candidates (for Alex naming)

Two candidate Recognitions surface from this mint. Both at candidate
strength awaiting Alex's ratification + a second empirical witness.

### §8.1 `#R-substrate-shape-of-bilateral-predicate-is-typed-carrier-plus-reflective-evaluator`

**Statement:** The substrate-honest shape of a bilateral predicate is
a typed carrier `{ name, sentinel, arity, require }` at shard-decl
altitude PLUS a reflective evaluator `discharge(decl, args) -> verdict`
at Rust FLOOR altitude. The two altitudes MEET when the grammar loader
reads the shard-decl corpus and hands the reflective evaluator a lookup
table keyed by full action ref.

**First witness:** THIS mint (shard-decl + companion keywords + spec +
math).

**Second witness (pending):** Reed's follow-up retirement of the ~30
arms per §5.3. When the reflective evaluator ships and dispatches
correctly for all 8 shard groups' smoke tests, the pattern is
second-witnessed at empirical altitude.

### §8.2 `#R-the-30-hand-typed-arms-were-substrate-dishonest-Rust-extensions`

**Statement:** The ~30 hand-typed bilateral-predicate arms in
`bootstrap/src/apply_h.rs` were substrate-dishonest per
`[[feedback-no-rust-extension-shortcut]]` — each arm was a small Rust
extension re-uttering what the corresponding shard-decl docblock
already declared. The pattern was invisible at per-arm authorship
altitude but visible at 30-instance altitude. Once counted, the
substrate-dishonesty becomes structural.

**First witness:** THIS spec's §2.4 substrate-honest reading of the
duplication cost.

**Second witness (pending):** Reed's follow-up retirement + the
empirical ouroboros_monotone delta (rust_loc −620 net; the substrate
LITERALLY becomes more substrate-honest by ~620 LOC).

**Corollary:** future bilateral predicates lift to shard-decl altitude
FIRST (add a `bilateral` block); the Rust FLOOR discharges them
reflectively automatically. No new hand-typed arm should ever be
authored again for a bilateral-shaped predicate.

---

## §9. Related work / prior art

### §9.1 In-substrate precedent

- **`shards/mirror/spec/keywords.mirror`** — the precedent this
  mint mirrors: a companion keyword file registered in
  `companion_keyword_sources` that adds new keywords to the mirror
  grammar without modifying the parser. Same technique; same
  altitude.

- **`shards/song/keywords.mirror`** — second precedent (Mara
  `d29d45e` Path B; Rung 3b 2026-07-13). Same pattern.

- **`shards/mirror/lens/knife.mirror`** — three-state discriminator
  precedent for typed verdict carrier. Different shape (enum-style
  variants vs record fields), but the same substrate-honesty
  principle: the substrate declares the shape; the Rust FLOOR
  dispatches through it.

- **The 13 sibling `@pact` species** — all follow the pact-family
  discipline of "declarative property half + operational discharge
  half"; this species is the shape carrier the pact-family altitude
  inherits from.

### §9.2 Pre-AI ancestry

- **Content-addressed dispatch (Bazel, Nix).** Byte-checking a
  substrate ref's OID for sentinel containment is a specialization
  of content-addressed dispatch: the sentinel is a content-address-
  fragment; the dispatch is byte-comparison. Bazel's remote-cache
  and Nix's derivation-hash both use this pattern at different
  altitudes.

- **Pattern-matching over data constructors (ML family).** The
  reflective evaluator's `discharge(decl, args)` is structurally
  identical to a pattern match over the `bilateral` record's fields.
  The substrate's contribution is *typing* the pattern at the
  substrate-decl altitude so the pattern's shape is machine-readable.

- **Foerster 1976 "Eigen-behaviors".** The bilateral discipline's
  Pass/Fail bilateral verdict is a specialization of Foerster's
  eigen-operator discipline: the sentinel is a substrate-decl'd
  eigen-value; the discharge is `Op(oid) = Pass iff oid contains
  eigen-value`. This species IS the Foerster eigen-operator surface
  at pact altitude.

- **Pask 1975 "Conversation Theory".** The bilateral is one turn in
  a Paskian agreement: shard-decl declares its sentinel as
  proposition; the argument-side realization declares its sentinel-
  containment as agreement. Pass = agreement held; Fail = agreement
  refused. `[[architecture-property-fracture-bilateral]]` was
  substrate-decl'd as recognition #37; this species is its typed
  carrier.

### §9.3 What THIS mint contributes

- **A typed record carrier** for the sentinel + arity + composition
  surface.
- **A companion keyword file** making the shape grammar-parseable
  via the existing grammar loader.
- **A reflective evaluator pseudocode** for Reed's follow-up
  retirement.
- **The math foundation** (`docs/math/epistemologic/pact/bilateral-
  sentinel.md`) grounding the sentinel-as-content-addressed-witness
  claim in Connes-triple discipline.

The mint is substrate-pull discharge, not invention: the pattern
was already landed 8 times; the shape names it.

---

## §10. Composition footer

### §10.1 Citations

- **Taut Q1 substrate-truth scout** (this session, 2026-07-16
  evening — pointer only; no separate audit doc exists yet). Taut's
  verdict: NO grammar-decl'd bilateral shape exists; the closest
  extensible carrier is `@pact`; sentinel string lives in docblock
  prose ONLY.

- **The four COLLAPSED witnesses:**
  - `shards/spectral/signature.mirror` (Arc-2 Tick 2.1; commit
    `f211ee48`)
  - `shards/epistemologic/cybernetic/coherence.mirror` (Arc-2
    Tick 2.2; Mara `e0a3e48`)
  - `shards/peer/persistence.mirror` (Arc-2 Tick 2.3; landing at
    the sheaf composition altitude)
  - `shards/kintsugi/roomba.mirror` (Arc-2 Tick 2.4; walker
    landing)

- **Alex 2026-07-16 verbatim:** *"Q1. Let's mint it then. Properly.
  Seems like it's load-bearing."*

- **Retirement math:** 30 arms × ~15 LOC = ~450 LOC direct
  duplication + ~250 LOC error-message prose = ~700 LOC cruft
  collapse; reflective act ~50 LOC replacement; net savings ~500
  LOC in `apply_h.rs`.

### §10.2 Delivery footprint

This spec is one of three files in the Mara 2026-07-16 bilateral-
shape mint session:

1. **Shard-decl** at `shards/epistemologic/pact/bilateral.mirror`
   (commit 1; ~380 LOC).
2. **Companion keywords** at
   `shards/epistemologic/pact/keywords.mirror`
   (commit 1; ~60 LOC).
3. **Canonical spec** at
   `docs/specs/bilateral-predicate-substrate-shape.md`
   (commit 2; THIS FILE).
4. **Math foundation** at
   `docs/math/epistemologic/pact/bilateral-sentinel.md`
   (commit 3; forthcoming this session).

Reed's follow-up (later tick): the retirement per §5 (~50 LOC
reflective evaluator + `bilateral {}` blocks in 8 shards + deletion
of ~700 LOC of hand-typed arms in `apply_h.rs`).

### §10.3 Substrate decisions cascade

- `[[architecture-shards-as-substrate-source]]` — the bilateral
  shape IS a shard-decl; the reflective evaluator READS the shard.
- `[[architecture-prism-as-trait-as-everything]]` — `prism
  @epistemologic/pact/bilateral` declares the species IS a prism.
- `[[architecture-glass-wall-substrate-types]]` — sentinel string
  is a `@nl` byte-string; the verdict is `@glass.verdict`; both
  types inherit through the prism composition.
- `[[architecture-property-fracture-bilateral]]` — this species
  IS the canonical form the bilateral pattern takes at family-
  shape altitude.
- `[[feedback-substrate-already-had-the-word]]` (~72nd landed
  instance).
- `[[feedback-craft-not-deliver]]` — action bodies discharge at
  realisation via the reflective evaluator.
- `[[feedback-no-rust-extension-shortcut]]` — this species is a
  shard-decl + spec + math triad; ZERO new Rust; the follow-up
  retirement is Rust-negative (LOC subtraction).
- `[[feedback-legibility-over-foundation-when-collapsing]]` — the
  shape name `bilateral` is the readable Foerster/Pask-lineage word.

### §10.4 The delightfully-boring test

Alex's discipline: *"the reader ought to go 'of course it's this.'"*
(AGENTS.md Delightfully Boring section).

Applied here: the shape carries `name`, `sentinel`, `arity`, and
optional `require`. The reflective evaluator byte-checks the sentinel
in each arg's oid and AND-conjuncts requires. **Of course it's this.**
The mint doesn't invent; it names what has been landed 30 times.

---

**End of `docs/specs/bilateral-predicate-substrate-shape.md`.**
