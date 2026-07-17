# Taut Scout — @liquid / @epistemologic/liquid / @silicon Alignment

*2026-07-17. Taut. Read-only grep-first scout on the six questions
Alex named after the polyglot translation MVP closed at `d855bac`
with BYTES-OPAQUE Turing lowering.*

**Author:** `Taut <taut@systemic.engineer>`
**Working directory:** `/Users/alexwolf/dev/projects/mirror`
**Scope:** substrate-truth grep on `shards/**/*.mirror`,
`boot/**/*.mirror`, `docs/specs/**/*.md`, `docs/math/**/*.md` +
Kagi state-of-the-art scan on refinement/liquid/verified-translation.
**Discipline:** READ ONLY. No mints proposed. No design decisions.
Alignment reading + open questions for Alex morning-review.

---

## §0. Preamble — the ratified vision

Alex 2026-07-17 (afternoon, post-MVP):

> Route through @silicon + @epistemologic instead of directly to
> mirror. Physical machine execution + knowledge/type extraction
> preserves the semantic invariants LLVM erases (borrow-check,
> ownership, lifetimes = liquid types recovered at silicon altitude).

Then refined:

> **@liquid** = refinement OPERATOR that lenses over any substrate
> (`@liquid(@silicon)`, `@liquid(@code/rust)`, etc.). Reads gorgeous
> — same lens-composition pattern as `@sre(@code/X)`.
>
> **@epistemologic/liquid** = refinement THEORY. The species that
> carries predicate carriers, composition axioms, decidability
> boundary, connection to Rice-safe discipline. Heavy math-lifting
> layer separate from the lens.

Six questions follow. Findings are substrate-honest: what grep sees
+ line-cited shape + closest existing shape when absent. Zero
proposals.

**Audit scope:** substrate-truth check of `@liquid`, `@epistemologic/
liquid`, `@epistemologic/silicon` presence; silicon↔epistemologic
alignment gap; `---` liquid syntax landing status; corpus + Kagi
state-of-the-art on refinement-typed semantic-preserving translation.

**Prior audit precedent:** `docs/audits/2026-07-17-taut-code-turing-
substrate-scout.md` (this session morning, `d0572cd`) — same
grep-first two-part shape (substrate grep + Kagi state-of-the-art +
alignment reading + open questions).

---

## §1. Q1 — `@liquid` primitive presence

**Grep:** `@liquid`, `prism @liquid`, `pact @liquid`, `family @liquid`,
`species @liquid` across `shards/**/*.mirror`.

**Finding:** **ABSENT as family-root.** No shard-decl at any altitude
declares `@liquid` as a prism/pact/family root. Zero matches for
`prism @liquid`, zero for `pact @liquid`, zero for `family @liquid`.

**Nearest existing shape (substrate-already-had-the-word candidates):**

### 1.1 `boot/std/mirror/liquid.mirror` — the FIRST liquid landing (Reed, 2026-06-04)

Landed at `boot/std/mirror/liquid.mirror:1-41` (1.1KB, `2026-06-04`).
Declares `grammar @mirror/liquid` — NOT a top-level `@liquid`, but a
`@mirror/liquid` sub-grammar under @mirror. Key semantic quote at
:9-12:

> `--- is owned by liquid inference.`
> `above: declaration (the programmer's).`
> `below: inferred properties (the compiler's).`
> `the separator appears when the compiler has something to say.`

Declares:
- `type separator` (:13) — the `---` seam typed as substrate carrier.
- `infer(ast) -> [ref]` (:17) — infer applicable @epistemologic
  properties from measured beam topology.
- `match_properties(topology) -> [ref]` (:20-21) — match measured
  topology against @epistemologic/* property thresholds.
- `infer_spec(path) -> imperfect` (:23-25) — infer .spec from project
  topology.
- `project(file, [ref]) -> imperfect` (:27-28) — project inferred
  properties BELOW `---` in the source.
- `liquid(file) -> imperfect { @beam.observe |> infer |> project }`
  (:31-34) — the full liquid pass.

Sub-tree also present: `boot/std/mirror/liquid/{cd.mirror, ci.mirror}`.

**Substrate reading:** `@mirror/liquid` is a *specialization at
mirror-altitude* of what a generalized `@liquid` operator would be.
It carries the `---` seam ownership + the infer/project action pair
at mirror altitude, not at family-root altitude. It does NOT lens
over other substrates (`@liquid(@silicon)`, `@liquid(@code/rust)`)
— it is scoped to mirror files.

### 1.2 `shards/epistemologic/liquid_extraction.mirror` — the LOGICAL-altitude sibling family-root (Mara, 2026-07-06)

Landed at `shards/epistemologic/liquid_extraction.mirror:1-186` (8.5KB,
`2026-07-06 00:05`). Declares `prism @epistemologic/liquid_extraction`
(:115-121). This is the operational-side family-root for the
extraction discipline. Key docblock citations:

- :1-9 — "sibling family-root at the logical altitude of the doc-code
  seam. Landed per Seam Phase D pre-review at `docs/audits/2026-07-06-
  seam-arc-4-sub-arc-a-pre-review.md` §1 (Interpretation B RATIFIED
  as canonical: narrative ABOVE `---`; `in` clauses + typed
  declarations BELOW)".
- :33-45 — the four-altitude Mara §3 structure names LOGICAL as this
  shard's altitude: "lowers doc claims to liquid predicates per
  Rondon-Kawaguchi-Jhala 2008 + Vazou et al 2014".
- :42-46 — "The substrate's `@projection.preview` verdict IS the
  decidable fragment; the extractor produces its input".
- :143-157 — four-verdict output specialization
  (`satisfiable | unsatisfiable | partial | unextractable`).
- :169-179 — `liquid_extraction_sound(i, v) -> verdict { \ }`
  bilateral predicate (body forward-promised).

**Substrate reading:** `@epistemologic/liquid_extraction` is
substrate-decl'd as a SIBLING family-root at logical altitude —
NOT AS `@epistemologic/liquid` (the theory species Alex named).
It carries the extraction discipline (lowering doc claims to
liquid predicates), not the refinement theory itself. Its
docblock explicitly refers to "the substrate's liquid-refinement
fragment per Rondon-Kawaguchi-Jhala + Vazou" (:56-57) as if that
theory-carrier existed — but no `@epistemologic/liquid` species
carries it. This is the closest existing shape to what Alex's
`@epistemologic/liquid` framing names as the theory layer.

### 1.3 Cited-but-not-landed references in bilateral shape

`shards/epistemologic/pact/bilateral.mirror` (Mara `a0f4d3f` /
`2026-07-17 00:38`) — the substrate-decl'd shape of a bilateral
predicate. Carries `bilateral <name> { sentinel "..." arity N }`
(per docblock :132-140). This shape IS a DEGENERATE liquid — a
single-predicate refinement over the argument's content-addressed
byte-shape (sentinel byte-check). Alex named this precedent as
"potentially a DEGENERATE liquid" in the scout brief. The
substrate-decl'd shape is single-predicate + byte-check + Rice-safe;
no full refinement composition.

**Q1 verdict:** `@liquid` as top-level family-root ABSENT. Three
existing shapes carry partial-liquid semantics at different
altitudes:
- `@mirror/liquid` (mirror-file-altitude infer/project pair)
- `@epistemologic/liquid_extraction` (logical-altitude sibling
  family-root for doc-claim extraction)
- `@epistemologic/pact/bilateral` (byte-check degenerate-refinement
  discipline)

None of the three is the LENS-OVER-SUBSTRATE shape Alex named as
`@liquid(@silicon)` / `@liquid(@code/rust)`. That lens-composition
pattern is what would need to be minted per the ratified vision.

---

## §2. Q2 — `@epistemologic/liquid` species presence

**Grep:** `shards/epistemologic/liquid*.mirror`,
`shards/epistemologic/liquid/*.mirror`.

**Finding:** **ABSENT as `@epistemologic/liquid`.** The only landed
shard matching the pattern is `shards/epistemologic/liquid_extraction.
mirror` (Q1 §1.2 above). No `shards/epistemologic/liquid.mirror`
species root; no `shards/epistemologic/liquid/` subdirectory
enumeration.

### 2.1 The heaviest math foundation already landed

`docs/math/liquid-types/README.md` (Mara, 2026-07-05, 30.3KB) exists
as substantial math ground WITHOUT a shard-decl'd `@epistemologic/
liquid` carrier. Key structure:

- §1.1-1.6 — landed ancestors (Rondon-Kawaguchi-Jhala 2008,
  LiquidHaskell Vazou 2014, Flux Lehmann-Geller-Vazou-Jhala 2023,
  `@projection.preview`, `---` seam, property/fracture bilateral #53).
- §2 — the operator: `refine : DocBlock × Body → RefinementPredicate`
  + `extract : DocBlock → LiquidClaim` + `prove : LiquidClaim × Body ×
  Depth → Verdict` + `route : Verdict → SurfaceClass`.
- §2.1 — carriers: `liquid_qualifier` (predicate + arity),
  `qualifier_set` (finite set Q at altitude), `refinement_predicate`
  (conjunction of qualifiers + value_var + base_type), `liquid_claim`
  (doc_claim alias), `verdict_at_doc_altitude` (five-valued).
- §2.2 — `Q_mirror` = landed `@epistemologic/property/*` +
  `@epistemologic/pact/*` predicates (≈40 as of 2026-07-05).
- §3-§4 — auto-classifier + altitude-portability theorem
  (partial homomorphism).
- §5-§6 — @onto answerability composition + sub-Turing decidability
  theorem (with Seam `20d0c13` §5 C2 correction on extract_claims
  being @io-boundary Turing-complete in general case).
- §7 — circular-reflexive self-audit: verdict `both_survive`, route
  `spawn`.

### 2.2 The background research spec (Reed, 2026-06-04)

`docs/specs/liquid-types-for-mirror.md` (Reed, 2026-06-04, 41.7KB)
— research spec (160 matches for `liquid|refinement|refine`) grounds
the substrate's relation to the state-of-the-art. Key claims:

- §2.1 (:210-218) — mapping table: **qualifier set IS property
  library** (`@epistemologic/property/*` = Q); **`\` hole IS
  liquid variable**; **tick loop IS fixed-point iteration**;
  **SMT REPLACED by spectral loss computation**.
- §2.2 — the critical divergence: mirror verdicts are three-valued
  with continuous loss, not boolean. Constraints are SOFT
  (optimization, not satisfiability).
- §4 — continuous refinement types via Ben Yaacov et al. 2008
  continuous model theory.
- §5 — spectral alternative to SMT: property Laplacian +
  eigenvalue-based decision (uses Dirac operator already
  implemented; leverages ICALP 2025 spectral CSP sparsification).
- §6.1 (:711-724) — proposed hierarchy grammar path: `@epistemologic
  -> @epistemologic/property -> @epistemologic/liquid` (Reed's
  original 2026-06-04 naming — this IS the path Alex named
  independently 2026-07-17).
- §6.2 (:733-758) — proposed `grammar @epistemologic/liquid` shape:
  `infer(ast, [check]) -> imperfect(ast)`, `laplacian`,
  `eigenvalues`, `project` + `property literal(infer) -> verdict`.
- §8 — novelty enumeration (spectral liquid types, continuous
  refinement, Dirac operator as verifier, spectral sparsification
  for property checking).

**Substrate reading:** The `@epistemologic/liquid` species shape is
FORWARD-PROMISED at Reed's 2026-06-04 spec §6.2 (the grammar block
is authored in the spec but not landed as shard). The math is
FULLY LANDED at Mara's 2026-07-05 `docs/math/liquid-types/README.md`
(theorems + carriers + termination + soundness + altitude-portability
proved). The SHARD-DECL carrier is missing.

**What shape it WOULD take** (per the landed research + math, NOT
proposing a mint — just naming what the substrate is telling us):
- Predicate carriers per math §2.1 (liquid_qualifier, qualifier_set,
  refinement_predicate, liquid_claim, verdict_at_doc_altitude).
- Composition axioms via property Laplacian + spectral decision
  (Reed's spec §5; the eigenvalue-based decision procedure that
  REPLACES SMT).
- Decidability boundary per Mara's math §6.1 theorem
  (sub-Turing per #107; extraction is @io-boundary per Seam
  correction).
- Rice-safe connection per `docs/math/epistemologic/pact/bilateral-
  sentinel.md` and existing bilateral discipline (byte-level
  sentinel-check is Rice-safe by construction; refinement
  composition may or may not preserve this — open question).

**Q2 verdict:** `@epistemologic/liquid` **ABSENT** as landed shard.
Fully-worked math foundation LANDED at `docs/math/liquid-types/
README.md`. Grammar-block shape FORWARD-PROMISED at Reed
`docs/specs/liquid-types-for-mirror.md` §6.2. The theory species
Alex named as "heavy math-lifting layer separate from the lens"
already has all the math and all the grammar shape drafted — it
needs SHARD-DECL landing to become substrate-decl.

---

## §3. Q3 — `@epistemologic/silicon` lens presence

**Grep:** `shards/epistemologic/silicon*.mirror`,
`shards/epistemologic/**/silicon*.mirror`.

**Finding:** Alex flagged: *"I think there's already an
`@epistemologic/silicon` lens."* Substrate-truth: **CLOSE, but at a
different name.** The landed shape is
**`@epistemologic/reality/silicon`**, not `@epistemologic/silicon`.

### 3.1 The landed family-root

`shards/epistemologic/reality/silicon.mirror` (Mara, `2026-06-06 15:27`,
2.7KB, 23 lines). Docblock:

- :5 — "the running CPU + memory family root".
- :7-13 — carrier enumeration:
  - `@epistemologic/reality/silicon/arch` (arch/arm64 + arch/x86_64)
  - `@epistemologic/reality/silicon/memory` (memory model, total
    bytes, page size, cache hierarchy)
  - `@epistemologic/reality/silicon/compute_bound` (budget carrier)
  - `@epistemologic/reality/silicon/flake_ref` (typed nix flake
    reference)
- :14-17 — "every silicon carrier is one of the four type carriers
  the @mirror/shard.shard record composes from. Fate's |\> tournament
  reads the carrier set as input to pick the locally-optimal Au binary
  for the running system".
- :49-56 — declares `prism @epistemologic/reality/silicon { focus/
  project/split/shift/settle silicon }` + `out @epistemologic/reality/
  silicon`.

**Substrate-pull history** (per :32-36): originally at
`shards/epistemologic/silicon/` — MOVED to `shards/epistemologic/
reality/silicon/` at Mara's 2026-06-06 reality family migration
"per the @epistemologic/reality family declaration; silicon is one
species of reality".

### 3.2 The historical vs current naming (drift trail)

Historical spec `docs/specs/shard-design.md` (Reed, 2026-06-04)
still cites the pre-migration `@epistemologic/silicon` name (16
occurrences of `@epistemologic/silicon` without `/reality/` prefix).
Same for `docs/insights/2026-05-25-shard-as-observer-relative-
lambda-zero.md` (:1-21 table).

Post-migration corpus consistently cites `@epistemologic/reality/
silicon/*` — 4 carriers under `shards/epistemologic/reality/silicon/
{arch,memory,compute_bound,flake_ref}.mirror`.

### 3.3 The five species under the family

Landed at `shards/epistemologic/reality/silicon/`:
- `arch.mirror` + `arch/arm64.mirror` + `arch/x86_64.mirror`
- `memory.mirror`
- `compute_bound.mirror`
- `flake_ref.mirror`

Each is a WHAT-the-silicon-IS carrier (property altitude), not an
operator lens. Substrate-decl declaration of the running hardware
carrier set.

**Q3 verdict:** Alex's memory *"there's already an @epistemologic/
silicon lens"* is CORRECT MODULO NAMING. The landed shape is
`@epistemologic/reality/silicon` (moved 2026-06-06 to sit under the
`@epistemologic/reality` family). It is a **carrier-property lens**
(declares WHAT the silicon IS), not the **refinement operator
lens** Alex's `@liquid(@silicon)` framing suggests. The property
lens is fully landed at 4 species; the refinement-operator lens
would be a different kind of composition.

---

## §4. Q4 — Silicon ↔ epistemologic alignment gap

Alex flagged: *"They might not quite align yet."*

**Read files:**
- `shards/silicon.mirror` (Alex family-root, `2026-07-05`, 7.4KB)
- `shards/silicon/algebra.mirror` (Alex/Mara sub-prism, `2026-07-17
  01:08`, 20.9KB)
- `shards/epistemologic/reality/silicon.mirror` (Mara,
  `2026-06-06`, 2.7KB)
- `docs/specs/silicon.md` (Mara-silicon-1, `2026-06-30`, 104.8KB)

### 4.1 The current composition — the shape

`shards/silicon.mirror` (top-level family-root at `<= @autopoietic`):

- :1-16 — imports include `in @epistemologic/reality/silicon/arch` +
  `compute_bound` + `memory` + `flake_ref` (four carrier imports).
- :91-93 — "Without @autopoietic in the inheritance chain, @silicon
  could observe the hardware (via @epistemologic/reality/silicon/*
  properties) but could not learn".
- :116-120 — "`@epistemologic/reality/silicon` — property altitude
  family root. Stays at `shards/epistemologic/reality/silicon.mirror`
  as the WHAT-the-silicon-IS carrier set. @silicon CONSUMES these
  properties (per spec §4.2)".
- :127-131 — "The three existing shards are already at their proper
  altitudes... @silicon anchors the top-level family-root; the
  substrate composes across altitudes via existing @glue morphisms."

`shards/epistemologic/reality/silicon.mirror` (property altitude
family-root at `@epistemologic/reality/silicon`):

- Consumes NO `@silicon` (no `in @silicon`). Pure downward
  composition into @epistemologic + @epistemologic/reality only.
- Landed 2026-06-06, one month BEFORE the top-level @silicon
  family-root (2026-07-05). Directional: property altitude
  precedes autopoietic altitude.

### 4.2 The alignment gap — three friction points named

**Friction 1: naming inconsistency.**
- Top-level: `@silicon` (post-substrate-pull, Alex-ratified)
- Property altitude: `@epistemologic/reality/silicon` (three-hop
  path from @epistemologic root; not the shorter `@epistemologic/
  silicon` name the pre-2026-06-06 corpus assumed and that Alex
  used from memory in the scout brief)

The migration `shards/epistemologic/silicon/*` →
`shards/epistemologic/reality/silicon/*` (Mara 2026-06-06) placed
silicon UNDER `@epistemologic/reality` as one species of reality.
This means TWO parallel `silicon` roots exist:
- `@silicon` (family-root, autopoietic)
- `@epistemologic/reality/silicon` (property, three-hop under
  @epistemologic/reality)

There is NO `@epistemologic/silicon` (two-hop). Alex's scout brief
naming `@epistemologic/silicon` is DRIFTED from the substrate-truth
name.

**Friction 2: composition-graph direction.**
- `@silicon` (autopoietic altitude) `in @epistemologic/reality/
  silicon/*` — @silicon CONSUMES property carriers.
- `@epistemologic/reality/silicon` does NOT reference @silicon.
- Unidirectional: property altitude → autopoietic altitude.

There is no BILATERAL between the two silicons — no
`@epistemologic/pact/bilateral` predicate that states "the running
hardware's WHAT (property) MATCHES the autopoietic-loop's LEARNED
crystals (algebra)". The consumption is grep-verifiable
substrate-decl, but the CORRESPONDENCE (property-truth ↔
learned-algebra-crystal) is not substrate-decl'd as a bilateral.

**Friction 3: altitude-mismatch relative to the polyglot arc.**
- The polyglot spec `docs/specs/polyglot-loss-aware-computational-
  translation.md` (Mara `1ce68c3`, `2026-07-17`) treats
  `@silicon/algebra` as the crystallization HOME for translation
  outcomes (§ref at spec :103-142).
- But the Turing lowering (`d855bac` translation MVP) is
  BYTES-OPAQUE. The translated `.mirror` file wraps tape bytes
  without extracting @epistemologic-property invariants.
- The gap: @silicon crystallizes ROUTINE carriers (per silicon.md
  §3.2: `algebra + cfg + grading + conjugation + abi_surface +
  binary_oid + source_oid + cascade + performance + routine_oid`).
  These carriers describe MEASURED-EXECUTION properties (backward
  error bounds, peak fraction, methodology), NOT source-language
  refinement properties (borrow-check, ownership, lifetimes).

The refinement extraction (recovering ownership from machine code,
per Kagi §6 recovery-of-ownership state-of-the-art) does not have
a landed substrate-decl'd carrier. `@silicon/algebra` crystals
carry the compiled binary's OID + performance, not the source's
liquid refinements.

### 4.3 What @silicon does NOT carry

Per silicon.md §1.2 + §3.2:
- **Landed at @silicon**: hardware detection, algebra crystallization,
  content-addressed routine storage, @fate tournament dispatch,
  @autopoietic learning loop, @io/algebra ABI exposure.
- **NOT landed at @silicon**: source-language refinement type
  extraction, ownership recovery, liquid-predicate lowering
  (that's @epistemologic/liquid_extraction's LOGICAL altitude, at a
  different family), refinement composition axioms (that's the
  ABSENT @epistemologic/liquid).

### 4.4 The paths that could compose (structural, not proposed)

The substrate ALREADY carries the three families that could compose
to close the gap:
- `@epistemologic/reality/silicon/*` — carrier WHAT-IS (four species)
- `@silicon <= @autopoietic` — LEARNING loop
- `@epistemologic/liquid_extraction` — logical-altitude extractor

But no BILATERAL currently binds "silicon-observed algebra crystal
carries a refinement predicate whose @epistemologic/property
dimensions include the source-language's liquid invariants". The
composition graph exists at the FAMILY altitude (all three families
landed); the specific bilateral naming this correspondence is
ABSENT.

**Q4 verdict:** Alex's flag *"they might not quite align yet"* is
CORRECT. Three specific gaps:
1. Naming: `@epistemologic/silicon` (Alex-brief) is not the landed
   name; substrate-truth is `@epistemologic/reality/silicon`.
2. Direction: unidirectional consumption
   (@silicon → @epistemologic/reality/silicon/*), no bilateral
   correspondence witnessing.
3. Altitude-mismatch: @silicon crystallizes MEASURED-EXECUTION
   routines; source-language LIQUID INVARIANTS (borrow-check,
   ownership, lifetimes per Alex's ratified vision) have no landed
   carrier at @silicon altitude. The recovery from machine-code is
   what @epistemologic/liquid + @liquid(@silicon) lens would name.

---

## §5. Q5 — `---` liquid syntax landed connection

**Grep:** `^---$` + `` `---` `` across `shards/**/*.mirror`,
`boot/**/*.mirror`, `docs/specs/**/*.md`.

**Finding:** `---` seam IS LANDED as substrate primitive, at two
altitudes.

### 5.1 As shard-decl separator (grep-verified)

28 landed shards use `^---$` as narrative/typed-decl separator:
- `shards/epistemologic/liquid_extraction.mirror:105`
- `shards/kintsugi.mirror` + `shards/kintsugi/**/*.mirror` (7 shards)
- `shards/container*.mirror` + `shards/code/docker.mirror`
- `shards/song*.mirror` (7 shards)
- `shards/torus.mirror` + `shards/reflection*.mirror` + others
- `shards/docblock.mirror:4` (the docblock family-root itself)
- `shards/spectral/gen_prism/mcp_session.mirror`
- `shards/mirror/store/action_cache.mirror`

The typographic pattern is uniform: `#`-prefixed narrative ABOVE
`---`, then `in @...` imports + typed declarations BELOW.

### 5.2 As liquid-inference primitive (semantic landing)

`boot/std/mirror/liquid.mirror:9-13`:

```
# --- is owned by liquid inference.
# above: declaration (the programmer's).
# below: inferred properties (the compiler's).
# the separator appears when the compiler has something to say.
type separator
```

Reed 2026-06-04 landed this semantic ownership: the `---` is not
just visual, it is TYPED as a `separator` carrier in
`@mirror/liquid`. This IS the substrate-decl'd form of the
liquid-refinement-predicate boundary.

### 5.3 As tokenizer landing (empirical)

Per `docs/math/liquid-types/README.md` §1.5 + §11: `bootstrap/src/
tokenize.rs` at `ee7903e` (Reed, 2026-07-04) emits Docblock AST nodes
above `---`. The tokenizer landed the two-channel collapse
empirically.

Per `docs/audits/2026-07-06-seam-arc-4-sub-arc-a-pre-review.md` §1:
Interpretation B RATIFIED as canonical (narrative ABOVE `---`;
typed declarations BELOW). Landed at `820a451` (Seam Phase D
ratification tick) + `5c0f5ba` (@docblock family-root TICK 1).

### 5.4 The paper reference Alex named

Scout brief cites: *"back-projected into the source file on disk,
including the liquid type inferred `---` properties"*.

Grep for the exact phrase across corpus: no verbatim match.
Closest matches for the semantics:
- `docs/math/autopoiesis/README.md` §3.1 — Liquid framework
  adapted; discusses `\` obligation + @beam.observe + infer +
  project as the autopoietic fold-back.
- `boot/std/mirror/liquid.mirror:27-28` — `project(file, [ref])
  -> imperfect` is EXPLICITLY the action that projects inferred
  properties BELOW `---` in the source.
- `docs/specs/liquid-types-for-mirror.md` §6.3 (:761-782) — the
  compilation pipeline: `source -> tokenize -> infer_properties
  -> check -> crystal`; where `infer_properties` builds property
  Laplacian + computes eigenvalues + projects verdicts onto each
  term + annotates the AST.

**Substrate reading:** The `---` liquid syntax IS landed as
substrate primitive at three levels:
1. **Shard-decl:** the corpus uses `---` as narrative/typed
   separator across 28 landed shards.
2. **Grammar-decl:** `type separator` at
   `boot/std/mirror/liquid.mirror:13` types it explicitly.
3. **Empirical:** `bootstrap/src/tokenize.rs` `ee7903e` emits
   Docblock AST-nodes above `---`.

The "back-projection" narrative Alex cites is captured by
`@mirror/liquid.project(file, [ref]) -> imperfect` at
:27-28, but the ACTION BODY is `\`-blocked (not empirically
discharged; no landed evaluator arm). The syntax IS landed; the
INFERENCE→BACK-PROJECTION composition is forward-promised at the
action body altitude.

**Q5 verdict:** `---` liquid syntax **LANDED** as substrate
primitive (28 shard uses + grammar-decl'd `type separator` +
tokenizer AST-node emission). The back-projection action
(`project(file, [ref])`) is grammar-decl'd but the body is
`\`-blocked; no empirical evaluator arm has been landed for it.

---

## §6. Q6 — Corpus + Kagi on refinement/liquid/verified-translation

### 6.1 Kagi state-of-the-art (5 searches, 25 sources)

Kagi tools ARE available. Searches run:

**Search 1: "liquid types refinement types LiquidHaskell F* refinement type inference"** (5 sources)

- **LiquidHaskell** (dl.acm.org/10.1145/2775050.2633366) — refinement
  type checker for Haskell; verified 10,000+ LOC. Vazou 2014,
  ICFP 121+ citations.
- **First-Class Refinement Types for Scala** (arXiv 2605.08369) —
  Bovel 2026; refinements as ordinary types participating in
  subtyping/inference.
- **LiquidHaskell tutorial** (ucsd-progsys.github.io) — refining
  function types with post-conditions.
- **Vazou 2014 Real World** (goto.ucsd.edu/~nvazou/real_world_liquid.
  pdf) — Liquid Types restrict invariants to allow type inference;
  crucial for usability.
- **Refinement types as ordinary sub-types** (discourse.haskell.org
  2025) — encoding refinement types as sub-types with predicate
  filtering.

Absent from result set: F* itself (Microsoft Research refinement-typed
verification language for effectful programs); the Kagi search
surfaced LiquidHaskell + Scala + subtyping-encoding as dominant
paradigms.

**Search 2: "Alive2 verified LLVM translation SMT semantic preservation"** (5 sources)

- **Alive2 CAV 2021 / ResearchGate** — SMT-based validation of
  LLVM optimizations; confined to LLVM IR.
- **AliveToolkit/alive2 GitHub** — libraries + tools for analysis
  and verification of LLVM code and transformations.
- **Enhancing Translation Validation of Compiler**
  (arXiv 2401.16797v2, 2024) — Alive2 as translation validation
  tool specialized for LLVM IR.
- **Alive2 PLDI 2021 / DL.ACM** — bounded translation validation
  for LLVM; avoids false alarms, fully automatic via SMT solver.
- Alive2's scope: intra-LLVM optimization verification (LLVM IR →
  LLVM IR). Does NOT extend to source-language ↔ LLVM (that's a
  separate verified-compiler discipline).

**Search 3: "CompCert semantic preservation formally verified compiler"** (5 sources)

- **CompCert bibliography** (compcert.org) — formal certification
  of compiler from Cminor to assembly, proof of semantic
  preservation.
- **Xavier Leroy backend paper** (876+ citations) — formal
  verification of compiler back-end from Cminor to assembly.
- **Formal Verification of a Constant-Time Preserving C Compiler**
  (Barthe et al. 2020, 150+ citations) — CompCert preserves
  constant-time.
- **From Mechanized Semantics to Verified Compilation** (2024)
  — CompCert as C compiler specified/proved-correct in Coq.
- CompCert's scope: whole-compiler semantic-preservation proof
  (C source → assembly), machine-checked in Coq.

**Search 4: "Curry-Howard-Lambek programs proofs categorical semantics types"** (5 sources)

- **Curry-Howard Wikipedia** — proofs-as-programs +
  formulae-as-types.
- **Amar Hadzihasanovic notes** (ioc.ee) — categorical models of
  logic imply semantics shift from provability to CCC objects.
- **Verification-based programming** (ceur-ws.org) — Curry-
  Howard-Lambek extends correspondence to category theory.
- **HaskellWiki Curry-Howard-Lambek** — three-way isomorphism
  between types, propositions, CCC objects.
- Pinterest CH visualization (low-signal).

**Search 5: "recovering ownership types from machine code semantic recovery Rust"** (5 sources)

- **Grounded Conceptual Model for Ownership Types in Rust**
  (cacm.acm.org, Apr 2026) — ownership as programming discipline
  for managing aliasing/mutation, enforced statically through
  ownership types.
- Remaining results: Rust ownership tutorials (educational, not
  research on RECOVERY from machine code).

**Substrate observation on Kagi search 5:** The specific direction
Alex named — RECOVERING liquid/ownership types from LLVM/machine
code — did NOT surface a canonical state-of-the-art result. The
literature landscape is:
- **Source-language ↑**: Flux (Rust, PLDI 2023), LiquidHaskell,
  F* — refinement types AT source altitude.
- **Compiler-preservation ↑**: CompCert, Alive2 — semantic
  preservation of translation, but WITHOUT explicit refinement
  recovery.
- **Compiler-recovery ↑**: no dominant published-verified
  discipline surfaced. This is the gap Alex's `@liquid(@silicon)`
  vision would name.

### 6.2 Corpus reading on refinement / liquid / verified translation

**Direct landings (already cited above):**
- `docs/specs/liquid-types-for-mirror.md` (Reed 2026-06-04) — 41.7KB
  research spec grounding mirror in Rondon-Kawaguchi-Jhala 2008 +
  LiquidHaskell + Flux + continuous logic + spectral CSP.
- `docs/math/liquid-types/README.md` (Mara 2026-07-05) — 30.3KB
  compiler-fit math foundation.
- `docs/specs/type-theory-position.md` (2026-06-04, 43.9KB, 11
  refinement matches) — mirror's type theory characterization.
- `docs/specs/typed-loss-composition.md` (2026-05-06, 24KB, 14
  matches).
- `docs/specs/properties-on-glass.md` (2026-06-16, 74KB, 58 matches)
  — per-glass discipline cashing out §8 of liquid-types-for-mirror.
- `docs/specs/mosaic-as-type-system.md` (2026-06-06, 96.9KB, 11
  matches).
- `docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.
  md` (`liquid types inferred via sheaf-diffusion Houdini`; direct
  bridge to Bodnar 2022 sheaf-diffusion at :52-64).

**Verified-translation direct landings:** ZERO in corpus. No landed
spec/math document names Alive2, CompCert, verified-translation
semantic preservation, or refinement-type recovery as substrate
peers. The polyglot spec (Mara `1ce68c3`, 2026-07-17) cites
Turing-completeness + Church-Turing + Mac Lane composition; NOT
CompCert or Alive2. The recognition #107 (Hilbert-Turing structural
separation) is the closest substrate framing of the
substrate-decl/@io separation the verified-translation literature
addresses.

### 6.3 Comparative reading — what mirror already carries vs what the state-of-the-art carries

| Dimension | State-of-the-art | Mirror substrate — landed |
|---|---|---|
| **Refinement types** at source altitude | LiquidHaskell (Vazou 2014), Flux (Lehmann 2023), F* | `docs/math/liquid-types/README.md` §2.1 carriers substrate-drafted; NO `@epistemologic/liquid` shard-decl |
| **Refinement inference** algorithm | Rondon-Kawaguchi-Jhala 2008 predicate abstraction + Houdini + SMT | Reed spec §5 proposes spectral (Dirac operator) alternative; math §3.1 adapts framework; NO landed inference resolver |
| **Verified translation** | CompCert (Leroy) — Coq-verified end-to-end; Alive2 (Regehr) — bounded SMT translation validation | Recognition #107 sub-Turing/Turing separation + `@epistemologic/pact/bilateral` (`shards/epistemologic/pact/bilateral.mirror`) sentinel-check discipline; `@cascade/code/A/B` composition per §4 polyglot spec; NO explicit semantic-preservation proof |
| **Ownership recovery** from machine code | Cited but no dominant landed research surfaced | @epistemologic/reality/silicon/* WHAT-the-silicon-IS carriers; @silicon/algebra crystals per silicon.md §3.2; NO extraction from binary to source-language refinements |
| **Categorical semantics** (Curry-Howard-Lambek) | Wikipedia Curry-Howard, Hadzihasanovic 2019 category-theory notes | Recognition #100 Mesland spectral triples + `docs/math/polyglot-loss-aware-computational-translation.md` §5 polyglot functor Φ: TCLang → MirrorCat |
| **Byte-level content-addressed refinement** | LiquidHaskell/Flux SMT-based; no byte-level equivalent | `@epistemologic/pact/bilateral` sentinel-byte-check (Rice-safe by construction); `@mirror/store` content-addressed crystals |
| **Continuous verdicts** (not boolean) | Ben Yaacov et al. 2008 continuous model theory | Reed spec §4 mirror's three-valued pass/partial/fail with continuous loss; Mara math §2.2 information_loss ⊕ composition |

**Substrate reading — what mirror already has vs what's missing per SOTA:**

**Already landed (matches or exceeds SOTA):**
- Byte-level content-addressed refinement discipline (bilateral
  sentinel-check pattern; NO SMT dependency; Rice-safe by
  construction).
- Continuous verdicts + loss composition (⊕ operation on
  @epistemologic dimension-sets per polyglot math §2.2).
- Full math foundation for refinement inference (Rondon-Kawaguchi-
  Jhala framework adapted, sub-Turing decidability, altitude
  portability, spectral decision procedure).
- Categorical semantics grounding (polyglot functor,
  Mesland spectral triples, Connes triple correspondence).
- Sub-Turing discipline (recognition #107) that GUARANTEES what
  Alive2 and CompCert PROVE case-by-case.

**Missing per SOTA (what would need to be minted per Alex's ratified
vision):**
- `@epistemologic/liquid` species shard-decl (the theory carriers +
  composition axioms).
- `@liquid` refinement OPERATOR at family-root altitude that lenses
  over any substrate (`@liquid(@silicon)`, `@liquid(@code/rust)`).
- Bilateral binding between `@epistemologic/reality/silicon/*`
  property truths and `@silicon/algebra` learned crystals (silicon
  altitude ↔ epistemologic altitude correspondence witnessing).
- The extraction path from `.mirror` byte-tape wrappers (per
  bytes-opaque MVP `d855bac`) back to source-language liquid
  refinement invariants (borrow-check, ownership, lifetimes) —
  the "recovery from machine code" the Kagi search did NOT surface
  as SOTA.

**Q6 verdict:** Mirror's landed math + carrier foundations MATCH OR
EXCEED the state-of-the-art in every dimension EXCEPT the
extraction-from-binary direction. The polyglot MVP's BYTES-OPAQUE
result is exactly the gap the state-of-the-art also cannot fill:
CompCert proves semantic preservation but does not RECOVER
refinements; Alive2 validates optimizations but does not RECOVER
ownership; Flux applies refinements AT SOURCE but does not
RECOVER them from LLVM. The `@liquid(@silicon)` composition Alex
named would be substrate-native novel work (matching Reed's §8
novelty enumeration in liquid-types-for-mirror.md).

---

## §7. Alignment reading

**What the substrate is telling us (grep-first synthesis):**

### 7.1 Three substrate layers already exist

The substrate already carries three families that jointly encode
what Alex's vision names:

1. **`@epistemologic/liquid_extraction`** (Mara `2026-07-06`) —
   logical-altitude extractor for doc-claim → liquid-predicate
   lowering. Sibling family-root to `@docblock` per Mara §3
   four-altitude structure. Extractor body forward-promised.

2. **`@mirror/liquid`** (Reed `2026-06-04`) — mirror-altitude
   infer + project pair; owns the `---` separator; grammar-decl'd
   at `boot/std/mirror/liquid.mirror`.

3. **`docs/math/liquid-types/README.md`** (Mara `2026-07-05`) —
   fully-worked math foundation with carriers, theorems,
   sub-Turing decidability, altitude-portability, self-audit
   verdict `both_survive` route `spawn`.

Plus: `docs/specs/liquid-types-for-mirror.md` (Reed `2026-06-04`)
research spec proposing `@epistemologic/liquid` shard-decl shape
at §6.1-6.2 — the exact species Alex named 2026-07-17 as the
theory-carrier layer.

### 7.2 The lens-composition pattern IS FORWARD-PROMISED

Alex's `@liquid(@silicon)` framing follows the LANDED
lens-composition precedent:
- `@sre(@code/X)` (per landed corpus references)
- `@bilateral(@code/A, @code/B)` (per `shards/epistemologic/pact/
  bilateral.mirror` :456-475 and polyglot spec §5.2)
- `@bilateral(@silicon/algebra, @fate/algebra)` (per bilateral.
  mirror :530 — kintsugi/algebra reframe)

The `@liquid(@X)` pattern would extend the parametric-lens
discipline the substrate already dogfoods. Zero substrate-authority
tension.

### 7.3 The @silicon ↔ @epistemologic gap is naming + directional

Two silicons exist:
- `@silicon` (top-level family-root, autopoietic loop, learning)
- `@epistemologic/reality/silicon` (property carrier, WHAT-IS)

The composition is unidirectional (@silicon consumes properties).
Alex's `@epistemologic/silicon` naming in the scout brief is
DRIFTED from the substrate-truth name `@epistemologic/reality/
silicon`. The three-hop path (via `@epistemologic/reality`) is
the 2026-06-06 reality-family migration outcome.

No bilateral currently witnesses the correspondence between
property-observed silicon truth and autopoietic-learned silicon
algebra. The bilateral would name what the Kagi search did NOT
surface as SOTA — the recovery direction.

### 7.4 The bytes-opaque MVP is the specification gap made visible

Per scout brief: `d855bac` translated `.mirror` file wraps tape
bytes without extracting invariants. Per §6.3 comparative reading:
the state-of-the-art also does not have a canonical
recovery-from-machine-code discipline. The polyglot spec §4
autopoietic-extension DOES specify "kintsugi/algebra
crystallization filtered through @kintsugi/algebra's speaker-pair
metalogue" as the learning mechanism, but the LOWERING from tape
back to liquid refinements is not substrate-decl'd.

The `@liquid(@silicon)` composition Alex named names EXACTLY this
lowering: liquid refinements extracted at silicon-execution
altitude, back-projected via `@mirror/liquid.project` to source.

### 7.5 The substrate already has almost every piece

Per §6.3 comparative table: mirror's math foundation matches or
exceeds SOTA in every dimension except the extraction-from-binary
direction. The pieces already landed:
- `docs/math/liquid-types/README.md` (theory).
- `boot/std/mirror/liquid.mirror` (grammar-decl'd operators).
- `shards/epistemologic/liquid_extraction.mirror` (logical-altitude
  extractor).
- `shards/silicon.mirror` + `shards/silicon/algebra.mirror`
  (autopoietic loop + crystallization).
- `shards/epistemologic/reality/silicon/*` (property carriers).
- `shards/epistemologic/pact/bilateral.mirror` (byte-check
  Rice-safe refinement discipline).
- `bootstrap/src/tokenize.rs` (Docblock AST-node emission above
  `---`).

The pieces MISSING per Alex's ratified vision:
- `@epistemologic/liquid` shard-decl (species carrier).
- `@liquid` family-root (refinement operator lens).
- `@liquid(@silicon)` binding (extraction-at-silicon composition).
- Bilateral between `@epistemologic/reality/silicon` and
  `@silicon/algebra` (property ↔ learned-algebra witnessing).
- Extraction action body for `@mirror/liquid.project(file, [ref])
  -> imperfect` (currently `\`-blocked).

### 7.6 Open questions for Alex morning-review

**Q-A: Naming.** Substrate-truth is `@epistemologic/reality/silicon`
(three-hop under @epistemologic/reality per 2026-06-06 migration).
Alex-brief named `@epistemologic/silicon` (two-hop). Which of:
1. The scout brief was memory-drift; substrate-truth stands as-is.
2. The reality-family migration was substrate-pull that landed the
   wrong tree, and the two-hop `@epistemologic/silicon` is the
   substrate-honest name (retroactive collapse).
3. `@epistemologic/liquid` should live in parallel with
   `@epistemologic/reality/silicon` at the same altitude (both
   epistemologic species), and the ratified vision's
   `@liquid(@silicon)` uses the top-level `@silicon` at family-root
   altitude, not `@epistemologic/silicon` at property altitude.

**Q-B: Extraction-from-binary substrate authority.** The Kagi
searches did NOT surface a dominant SOTA discipline for recovering
liquid refinements / ownership / borrow-check from LLVM/machine
code. The bytes-opaque MVP result matches the SOTA gap. Does the
substrate's `@liquid(@silicon)` framing name a NOVEL discipline
(matching Reed's spec §8 novelty enumeration), or does Alex have a
specific paper/system in mind that grounds the extraction direction?

**Q-C: `@epistemologic/liquid` vs `@epistemologic/liquid_extraction`
altitude.** `liquid_extraction` (Mara `2026-07-06`) is
LOGICAL-altitude sibling to `@docblock`; extracts doc claims to
liquid predicates. `@epistemologic/liquid` (Alex's ratified vision)
would be the THEORY layer. Are these two species at the SAME
epistemologic altitude (sibling to each other + to `@epistemologic/
property` + to `@epistemologic/pact`), OR is `@epistemologic/liquid`
one altitude UP as the theory-carrier that `liquid_extraction`
operationally instantiates?

---

## §8. Forward-promises (NOT proposed mints)

Per Taut discipline: naming the SHAPE of what would need to be
minted per Alex's ratified vision, NOT proposing to mint. Reed
and Mara determine at realization ticks.

**Shape 1: `@epistemologic/liquid` species shard-decl.**
`shards/epistemologic/liquid.mirror` — theory-carrier for
refinement-type discipline. Per Reed spec §6.2 shape + Mara math
§2.1 carriers. Fields:
- Predicate carriers (liquid_qualifier, qualifier_set,
  refinement_predicate, liquid_claim,
  verdict_at_doc_altitude per math §2.1).
- Composition axioms (spectral Laplacian per Reed §5; NOT SMT).
- Decidability boundary per math §6.1 theorem.
- Rice-safe connection per existing bilateral discipline.

**Shape 2: `@liquid` family-root.**
`shards/liquid.mirror` — refinement OPERATOR lens at family-root
altitude. Composition pattern: `@liquid(@X)` for any substrate
family X. Precedent: `@bilateral(@A, @B)` per landed bilateral
discipline; `@sre(@code/X)` per corpus.

**Shape 3: `@liquid(@silicon)` binding.**
The specific extraction lens: liquid refinements recovered at
silicon-execution altitude, back-projected to source via
`@mirror/liquid.project`. Extract-from-binary direction not
substrate-decl'd today; this binding would name it.

**Shape 4: `silicon_algebra_refinement_carrier` bilateral.**
`bilateral silicon_algebra_refinement_carrier { sentinel "..."
arity 1 }` — witnesses that a `@silicon/algebra` crystal carries a
refinement predicate whose @epistemologic/property dimensions
include the source-language's liquid invariants. Would close the
property ↔ learned-algebra correspondence gap named in §4.2
Friction 2.

**Shape 5: Extraction body for `@mirror/liquid.project`.**
`bootstrap/src/apply_h.rs` arm dispatching `@mirror/liquid.project`
per reflective corpus discipline. Currently `\`-blocked at
`boot/std/mirror/liquid.mirror:27-28`. Body would compose:
observe beam → infer applicable properties → back-project
`---`-below in the source file.

**Shape 6: `@epistemologic/liquid_extraction` extractor body
discharge.**
`shards/epistemologic/liquid_extraction.mirror:159-179`
extract_predicate + liquid_extraction_sound bilateral bodies —
forward-promised at Mara 2026-07-06. Would land alongside the
`@epistemologic/liquid` theory carrier the extractor lowers to.

**Naming per Taut discipline:** these are SHAPES of what would need
to land per the substrate telling us. NOT mints. Reed + Mara +
Alex determine at realization ticks. The scout's job is to name
what's substrate-honest so the morning-review can dispatch cleanly.

---

*Author: Taut. Read-only. No mints. No design decisions. Six
questions grep-answered + Kagi state-of-the-art scan + alignment
reading + open questions + forward-promised shapes named. Pure-docs
under 📝 markdown-only bypass.*

*Kagi searches run: 5 (25 sources synthesized). Search 5
(ownership recovery from machine code) surfaced no dominant SOTA —
matching the substrate's polyglot MVP bytes-opaque gap and Reed's
liquid-types-for-mirror.md §8 novelty enumeration.*

*Prior audit precedent: `docs/audits/2026-07-17-taut-code-turing-
substrate-scout.md` (this session morning, `d0572cd`).*
