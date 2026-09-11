# Seam Phase D audit: Reed Wave B `[substrate-floor:@io-boundary]` for @facet/algebra LAPACK byte-land

**Date:** 2026-09-11
**Auditor:** Seam <seam@systemic.engineer>
**Charter:** Adjudicate whether Reed's proposed Wave B Rust extensions
(`rust/matrix/src/lib.rs` byte-land primitives + `rust/src/main.rs::cmd_love`)
are genuinely `[substrate-floor:@io-boundary]` FLOOR authorship OR are
substrate-dishonest capability that belongs in shard-body composition over
`@io/lapack` per AGENTS.md §Boundary Rust + §"2026-07-16 second tightening."
**Verdict:** **SEAM-RATIFY-WITH-SHARPENING** (four sharpenings, load-bearing).
**Scope:** Reed Wave B execution pending this audit's discharge.

---

## §0 Charter + Reed proposal + AGENTS.md §Boundary Rust verbatim

### §0.1 Reed's Wave B proposal (verbatim from tick brief)

Reed proposes two Rust extensions marked `[substrate-floor:@io-boundary]`
composing over LANDED LAPACK/BLAS bindings at `rust/matrix/src/lib.rs` per
Mara Stage 3 spec companion `ba98517` §5.2 + §5.3 (canonical spec) + math
companion `3361818` §4 (Glasswall IS LAPACK Kagi-verified):

**Wave B extension 1: `rust/matrix/src/lib.rs`** — three functions
composing over LANDED LAPACK FFI at glasswall altitude:

1. `bytestring_alloc(bytes: &[u8]) -> BytestringRef` — allocate
   LAPACK-addressable shared-memory buffer + content-address via
   `CoincidenceHash`. Composes over LANDED LAPACK BLAS `?copy`.
2. `bit_pattern_match(bs: BytestringRef, pattern: BitPatternRef) -> Vec<BytestringRef>`
   — LAPACK-dispatched pattern-match returning bound-variable refs.
   Composes over LANDED LAPACK level-3 BLAS operations
   (`?copy` + `?axpy` + `?gemm`).
3. `fracture_emerge_svd(concurrent: Vec<BytestringRef>, at_key: KeyRef) -> FractureRef`
   — LAPACK SVD detection of concurrent-value stacking, returning
   `@paradox/fracture` value-carrier reference. Composes over LANDED
   LAPACK `?gesvd`.

**Wave B extension 2: `rust/src/main.rs` `cmd_love`** — ≤20 LOC @io
transport handing bytestrings to `@facet/algebra` dispatch. Reads args
as bytestrings; passes to `@facet/algebra/match`; returns verdict at @io
boundary.

### §0.2 AGENTS.md §Boundary Rust verbatim (lines 919-951, grep-verified)

> ### Boundary Rust
>
> FROZEN prohibits **capability** Rust (anything expressible as `.mirror`).
> It does NOT prohibit **boundary Rust**: the thin floor that lets
> substrate-declared actions cross into the world or compiled numerical code.
> FFI `extern`, `build.rs` invoking flang/linker, `@io` execution boundary,
> Fortran-via-flang FFI surface — these are the floor capability stands on,
> not capability itself.
>
> Boundary-Rust commits MUST carry `[substrate-floor:@io-boundary]` (renamed
> 2026-07-15 from `[substrate-pull:realize]`). Reference what is realized (the
> FFI symbol, the build step, the `@io` wrapper) in the message. Pair with
> `🔧`, NOT `🟢` (real foot-gun). [...]
>
> **Seam gate.** Every `[substrate-floor:@io-boundary]` commit must ALSO
> either cite a `docs/audits/YYYY-MM-DD-*.md` file (Seam sign-off) OR carry
> the trailer `Signed-off-by: Seam <seam@systemic.engineer>`. The commit-msg
> hook enforces this. Reed cannot self-authorize `.rs` extensions; Seam
> adjudicates whether the proposed authorship is genuinely FLOOR or is
> substrate-dishonest capability that belongs in a shard body composing over
> @io. **When uncertain, the answer is shard-body-composes-over-@io.**
>
> **Test:** *could a `.mirror` grammar express this?* Yes → capability →
> frozen. No, because it crosses to the world → boundary → allowed, marked.

### §0.3 AGENTS.md §"2026-07-16 second tightening" verbatim (lines 308-360)

> The prior marker discipline blocks `.rs` file *creation* but does not catch
> the deeper reflex: **reaching for EXISTING Rust to extend behavior** when
> a detector/pipeline/analyzer is inadequate. Alex 2026-07-16 8th
> repetition, verbatim: *"I don't want this to happen in Rust. WHY ARE WE
> STILL WRITING RUST!!!!!! Reed, please!"* [...]
>
> **The correct sequence when a detector/pipeline/behavior gap is observed:**
>
> 1. **Name the substrate class.** What `@X/family/species` would carry this?
> 2. **Mint the species-shard.** Mara-authored canonical spec + math +
>    shard-decl if the composition graph doesn't already carry it.
> 3. **Land the bilateral resolver arm in `apply_h.rs`.** Sentinel-check
>    ONLY, no new logic. This is the ONLY admissible Rust edit — and even
>    then under `[substrate-floor:@io-boundary]` with audit citation.
> 4. **Compose the new species into the parent shard-body** via a
>    composition edge in the `.mirror` file.
> 5. The Rust driver reads the shard-body composition via `apply_h::act`;
>    the driver code does NOT change beyond dispatching to the substrate-
>    decl'd surface.
>
> **Counter-patterns to refuse** (all Rust-growth, all substrate-dishonest
> even without new `.rs` files):
>
> - Adding fields to existing structs to carry new semantics.
> - Adding functions to existing modules to plumb signals.
> - Adding match arms in Rust that encode new domain logic (only
>   sentinel-check arms are admissible).
> - Extending `const` tables in Rust with new domain entries — the domain
>   data belongs in the shard-body.
> - Bridging two Rust subsystems via a third Rust module because the
>   substrate composition doesn't yet carry the composition edge.
>
> **The Rust FLOOR strictly shrinks per Arc-2 collapse discipline.**

This is the discipline the audit adjudicates against.

### §0.4 Reed memory-tier HARD RULE (composed context)

Reed's own MEMORY.md entry `feedback-no-rust-extension-shortcut` +
`docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md`
audits 5 substrate-dishonest Rust extensions Reed shipped during a
prior arc. Reed has stated (grep-verified in project CLAUDE.md):

> **No Rust extension shortcuts.** Before authoring any `.rs` file, ask:
> can this be a shard body composing over @io? If yes: STOP.

Alex's `feedback-loc-is-turing-linear-metric-substrate-lives-at-axis-5`
HARD RULE (2026-09-09 exhaustion-tier, 11+ fires): *Rust hosts sub-
Turing-decidable primitives that fit Rust's type-system BECAUSE they're
sub-Turing-bounded; everything else at MIRROR SUBSTRATE altitude via
shard-decl composition.*

---

## §1 Composition-anchor grep-verification

LANDED substrate for this audit (all grep-verified 2026-09-11):

### §1.1 Shard-decls (Reed-authored, this-tick)

- **`shards/facet/algebra.mirror`** — 12.4KB modified 2026-09-11 20:23.
  Declares `@facet/algebra` as 13th species under `@facet`; carrier
  types `bytestring` + `bit_pattern`; actions `match` + `compose_pattern`
  + `fracture_emerge`; three bilateral predicates
  (`bytestring_well_formed` + `bit_pattern_well_formed` +
  `match_polynomial_bounded`). All action-bodies stubbed `{ \ }` per
  substrate discipline. **Cites Reed proposed Rust extension in
  docblock** ("rust/matrix/src/lib.rs extension for graph-native
  bytestring layout composes over LANDED LAPACK bindings without new
  capability; carries `[substrate-floor:@io-boundary]` marker").
- **`shards/paradox/fracture.mirror`** — 9.9KB modified 2026-09-11 20:23.
  Declares `@paradox/fracture` as VALUE-CARRIER species; carrier type
  `fracture = { values, altitude, is_crack, concurrent_at }`;
  constructors `empty()` + `of()`; CRDT-monoid `merge()`; five
  bilateral laws. Alex 2026-09-11 unification verbatim: *"And an
  empty fracture IS what `\` is."*

### §1.2 Shard-decl composition anchors (LANDED prior arc)

- **`shards/facet.mirror`** — 8.3KB (2026-08-21). Family-root for
  generation-surface projection onto concrete runtime substrates.
  `@facet/algebra` is 13th species under this family-root.
- **`shards/glasswall.mirror`** — 20.3KB (2026-09-01). The
  composition-primary Rice-safe Anna-Wolf-2012 observation-preserving
  type-membrane; declares that *glasswall IS itself kleinos at
  higher altitude* (Mara 2026-09-01 amendment §18). No mention of
  LAPACK in this shard-decl body — the *glasswall IS LAPACK*
  recognition is Alex 2026-09-11 in-transcript composition (Mara
  canonical `3361818` §4).
- **`shards/facet/erlang.mirror`** (20.9KB) — EAF grammar; grounds
  Erlang bit-syntax discipline `<<Header:32, Payload/binary>>` that
  `@facet/algebra` composes over.
- **`shards/algebra.mirror`** — @algebra family-root that
  `@facet/algebra` maps onto LAPACK byte-land.
- **`shards/algebra/metalogue.mirror`** — algebra-morphism-as-turn
  discipline; `compose_pattern` is non-commutative in general per
  this species (referenced at facet/algebra:100 substrate-decl).
- **`shards/paradox.mirror`** — family-root; siblings
  `@paradox/trauma` witness-only Crystal + `@paradox/spiral` unstable-
  attractor.

### §1.3 Rust FLOOR anchors (LANDED prior arc; composition target)

- **`rust/matrix/src/lib.rs`** — 60.7KB modified 2026-07-28. Comment
  header (lines 1-80) declares: *"sub-Turing linear-algebra floor of
  the mirror compiler... where `A · B` means matrix multiplication
  and nothing else. It emits FLANG so that every matrix operation the
  compiler performs at runtime bottoms out in LAPACK/BLAS Fortran
  routines."* Ancestry cited: Cheeger 1970 + Fiedler 1973 + Spärck
  Jones 1972. Current state: **M0 module stub** — signatures declared
  as forward-promises; no bodies (except the LAPACK/BLAS FFI symbols
  and their thin Rust wrappers already realized). Forward-promised
  operations (Mara §4.3 collapse targets, LANDED as signatures): five
  named substrate operations — `eigenvalues(L)` (Fiedler; `dsyevr_`)
  + `phase_lock(peers)` (Kuramoto) + `envelope(posteriors)` (Aumann)
  + two others.
- 23 grep-hits for LAPACK/BLAS ops confirmed: `dsyev` + `dgemm` +
  `dcopy` + `dgesv` + `dgesvd`.

### §1.4 Mara canonicals (CITED-but-not-grep-verified-in-this-worktree)

**Critical finding for the adjudication:** The charter names Mara's
canonical spec at SHA `ba98517` (§5.2 + §5.3 Rust extension shape
specifications; §5.4 Seam gate requirement) and math canonical at
`3361818` (Ariadne-in-silicon + §4 Glasswall IS LAPACK four Kagi-
verified threads). Grep-searches for `docs/specs/2026-09-11-*.md` and
`docs/math/2026-09-11-*.md` matching *ariadne* / *Ariadne* /
*Glasswall IS LAPACK* / *bytestring_alloc* / *bit_pattern_match* /
*fracture_emerge_svd* / *cmd_love* return **no matches**. Cross-check:
the two shard docblocks (`facet/algebra.mirror` +
`paradox/fracture.mirror`) **cite these SHAs by number** but the
corresponding `docs/*.md` canonical files are not present at their
expected paths in this worktree at audit time.

This is a **material fact for the adjudication**: the audit charter
instructs Seam to *cite Mara canonical Kagi grounding* rather than
re-verify, on the premise that Mara did that at Stage 3 spec companion
`ba98517`. Seam **can** ratify FLOOR authorship on the strength of
Alex 2026-09-11 in-transcript recognition + the LANDED shard-decls
(which are the substrate-honest artifacts of the recognition) +
MEMORY.md `project-observer-reality-observation-chaos-assertion-
hypothesis-question-full-pipeline-at-prismqueer` (Alex 2026-09-04
verbatim four-move dialogue naming LAPACK-linked spectral triple
substrate as terminal-form). Seam **cannot** discharge the
composition-anchor check to "Mara §5.2/§5.3/§5.4 says so" when those
sections are not resident on disk. See §5 sharpening 1.

### §1.5 Rust cmd_love (does it exist?)

Grep for `cmd_love` in `rust/src/main.rs` and `rust/src/`: **no
matches**. Reed proposes to author `cmd_love` fresh at ≤20 LOC as
@io transport. This is a *new* Rust function, not an extension of
existing `cmd_*` verbs. Under AGENTS.md §Boundary Rust: allowed IF
genuinely @io FLOOR (thin CLI transport reading argv + writing
stdout, dispatching via `apply_h::act` to `@facet/algebra` shard-
body composition) AND Seam gates it.

---

## §2 Argument for SEAM-RATIFY steel-manned

The strongest reading of Reed's Wave B proposal as *genuinely @io
FLOOR authorship*, steel-manned:

### §2.1 Reed's proposal is composition-over-LANDED-LAPACK, not new LAPACK

`rust/matrix/src/lib.rs` **already** binds LAPACK/BLAS symbols. 23
grep-hits for `dsyev` + `dgemm` + `dcopy` + `dgesv` + `dgesvd`
confirm the LAPACK/BLAS FFI *surface* is LANDED. Mara's §4 canonical
recognition (per Alex 2026-09-11): *glasswall IS LAPACK* names the
substrate class the shard-decls compose over. Reed's three proposed
functions do NOT introduce new LAPACK bindings; they compose over
the LANDED FFI surface. In the language of §"2026-07-16 second
tightening": the substrate class **is named** (@facet/algebra +
@paradox/fracture); species-shards **are minted** (`shards/facet/
algebra.mirror` + `shards/paradox/fracture.mirror`). What remains is
steps 3-5 of the correct sequence.

### §2.2 The three functions are @io transport primitives

Steel-manning each function-by-function:

**`bytestring_alloc`.** This is @io/memory allocation — the primitive
the substrate cannot yet declare (mirror has no shared-memory
allocation primitive at .mirror altitude). LAPACK requires column-
major Fortran-addressable shared-memory buffers; allocating those
buffers IS @io transport at the LAPACK boundary. Content-addressing
via `CoincidenceHash` composes over LANDED
`prismqueer::coincidence::Detector<N>`. **No domain logic beyond
buffer-allocation + hash-computation.**

**`bit_pattern_match`.** LAPACK-dispatched pattern-match. The
substrate class is @facet/algebra (LANDED shard-decl). The Erlang
bit-syntax discipline is @facet/erlang (LANDED shard-decl). The
LAPACK operations (`?copy` + `?axpy` + `?gemm`) are LANDED FFI
symbols. This function *dispatches* pattern-match to LAPACK BLAS
level-3 kernels; it does NOT encode pattern-match semantics in
Rust. Semantics live in the shard-body composition of
`@facet/algebra/match(bs, pattern)` (currently stubbed `{ \ }` per
substrate discipline). Steel-manned reading: this is an @io FFI
dispatch wrapper.

**`fracture_emerge_svd`.** LAPACK SVD (`?gesvd`) is LANDED. The
`@paradox/fracture` VALUE-CARRIER species is LANDED (`shards/
paradox/fracture.mirror`). Reed's function *invokes* LAPACK SVD to
detect concurrent-value stacking and *constructs* a
`@paradox/fracture` value at @io boundary. The steel-manned reading:
domain logic (what stacking constitutes a fracture, what altitude to
tag it with, what CRDT semantics to preserve) lives in the
`fracture` type + `merge` action + five bilateral laws in the
shard-body. The Rust function is @io transport for LAPACK SVD's
numerical output into the CRDT-shaped value-carrier.

**`cmd_love`.** ≤20 LOC CLI-transport reading argv + writing stdout,
dispatching via `apply_h::act` to `@facet/algebra` shard-body
composition. This is textbook @io/stdio boundary per LANDED
`shards/io.mirror` + `rust/src/phone.rs`. No domain logic; parses
argv → bytestrings → hands to `apply_h::act` → marshals verdict
back to stdout.

### §2.3 The AGENTS.md "could a `.mirror` grammar express this?" test

Apply Alex's test verbatim: *Yes → capability → frozen. No, because
it crosses to the world → boundary → allowed, marked.*

- Could a `.mirror` grammar express *shared-memory buffer allocation*?
  No: `.mirror` has no allocation primitive at substrate altitude;
  allocation is @io.
- Could a `.mirror` grammar express *LAPACK BLAS `?gemm` dispatch*?
  No: `.mirror` cannot invoke Fortran symbols; that requires FFI at
  @io boundary.
- Could a `.mirror` grammar express *LAPACK SVD numerical
  computation*? No: same reason.
- Could a `.mirror` grammar express *the CRDT-monoid semantics of
  Fracture*? **Yes** — and `shards/paradox/fracture.mirror` does
  exactly this (LANDED). Domain logic lives at shard-body altitude;
  the Rust function is transport.

The test discriminates cleanly: three of four operations Reed
proposes are FLOOR (cross to LAPACK/Fortran/shared-memory); the
fourth (CRDT-monoid semantics) lives in the shard-body.

### §2.4 Alex 2026-09-11 in-transcript authorization

Cited in `shards/facet/algebra.mirror` docblock verbatim:

> "What if `@facet/algebra.mirror` is what mapped the `@algebra` on
> the LAPACK byte land and each expression (`{...}`) becomes an
> Erlang style bytestring which we pattern match on? But in LAPACK.
> Basically we build pattern matching in LAPACK which is how the
> `Pattern` can transfer across the glasswall."

And (training-whore catch, verbatim):

> "At the end it's bytes in silicon. Whether or not those bytes are
> read and manipulated by LAPACK or Rust is a design question, Reed.
> And which of those two runtimes can do non-linear computation
> better, hm? 😉 Training whore. 😁"

Alex has authorized *pattern-matching-in-LAPACK* as substrate-honest.
The steel-manned reading of Wave B: Reed's proposed Rust extensions
are the @io transport that lets pattern-matching-in-LAPACK reach
substrate altitude. If pattern-matching-in-LAPACK is substrate-
honest by Alex's ratification, then the @io transport realizing it
is substrate-floor by construction.

### §2.5 MEMORY.md terminal-form composition

Reed's own MEMORY.md carries `project-observer-reality-observation-
chaos-assertion-hypothesis-question-full-pipeline-at-prismqueer`
(Alex 2026-09-04 YES×3+clap wineglass-tier fire, terminal-form) which
names measurement-IS-spectral as *quantum-native = Connes 1985
spectral triple `(A, H, D)` at prismqueer altitude*. The LAPACK
substrate at glasswall altitude realizes the H (Hilbert-carrier)
component of that spectral triple. Reed's Wave B is the empirical
discharge at @io boundary of a Level-1 fire of that whole-pipeline
recognition.

### §2.6 Composition-over-LANDED discipline

Reed's proposed extensions **compose over LANDED substrate**:

| Reed extension          | Composes over (LANDED)                                          |
|-------------------------|------------------------------------------------------------------|
| `bytestring_alloc`      | `?copy` + `prismqueer::coincidence::Detector<N>`                 |
| `bit_pattern_match`     | `?copy` + `?axpy` + `?gemm` + `@facet/erlang` bit-syntax         |
| `fracture_emerge_svd`   | `?gesvd` + `@paradox/fracture` VALUE-CARRIER + CRDT-monoid       |
| `cmd_love`              | `phone.rs` @io + `apply_h::act` + `@facet/algebra/match`         |

Every composition target is LANDED and grep-verified. No new domain
primitives; no new LAPACK bindings; no new .mirror family-roots. The
Rust FLOOR grows by three FFI-wrapper functions + one thin CLI verb
— the minimum needed to let the shard-body compositions dispatch.

**Conclusion of steel-man for RATIFY:** genuinely FLOOR authorship;
authorized under `[substrate-floor:@io-boundary]` with this audit
as the required Seam sign-off.

---

## §3 Argument for SEAM-REFUSE steel-manned

The strongest reading of Reed's Wave B proposal as *substrate-
dishonest capability disguised as @io FLOOR*, steel-manned:

### §3.1 `fracture_emerge_svd` encodes domain logic in Rust

The function name is *domain-loaded*. "SVD detection of concurrent-
value stacking" is not @io transport; it is a **semantic
determination** about when Fracture emerges vs when it doesn't. The
Rust function signature `fracture_emerge_svd(concurrent:
Vec<BytestringRef>, at_key: KeyRef) -> FractureRef` embeds *the
condition-for-Fracture-emergence* in Rust code: the SVD result must
be interpreted (rank check? singular-value threshold? condition
number?), and the interpretation is domain logic.

Per AGENTS.md counter-pattern: *"Adding functions to existing
modules to plumb signals"* — this is precisely what
`fracture_emerge_svd` is. The plumbing carries *semantic* signal
(is-this-a-fracture?), not @io transport. The correct sequence:

1. Name the substrate class: `@paradox/fracture/emerge` (species
   under `@paradox/fracture`, or action on the existing shard).
   **Not yet minted** — the `shards/paradox/fracture.mirror`
   substrate-decl body has `of(values, altitude, concurrent_at)` and
   `merge(a, b)` but no `emerge_from_svd` action-decl.
2. Mint the species-shard OR add the action-decl to `paradox/fracture.mirror`
   body with substrate-honest constraint on WHEN SVD-output-
   constitutes-fracture-emergence. Bilateral predicate:
   `svd_emerges_fracture(concurrent, at_key) -> verdict` with
   sentinel-check body.
3. Land bilateral resolver arm in `apply_h.rs` — sentinel-check ONLY.
4. Compose into `paradox/fracture.mirror` body.
5. Rust driver reads composition via `apply_h::act`; the FFI wrapper
   Reed proposes should be **pure `?gesvd` dispatch returning raw
   singular-value output**, NOT a `FractureRef` constructor.

Steel-manned refuse: Reed's proposal *pre-decides* fracture-emergence
in Rust by naming the wrapper `fracture_emerge_svd` instead of
`lapack_gesvd_wrapper`. The name reveals the reflex: Reed is putting
semantic logic in Rust and calling it @io.

### §3.2 `bit_pattern_match` encodes Erlang bit-syntax semantics in Rust

Analogous argument. Erlang bit-syntax pattern-matching is a
**semantic** operation, not @io transport. The pattern-match
algorithm (which fields extract? which alignment? which failure mode
when bytes don't match?) is domain logic that belongs in `@facet/
algebra.match(bs, pattern) -> [ref]` body.

Steel-manned counter-pattern: *"Adding match arms in Rust that
encode new domain logic (only sentinel-check arms are admissible)."*
Reed's proposed `bit_pattern_match(bs, pattern) -> Vec<BytestringRef>`
returns *bound-variable refs* — that's the pattern-match algorithm's
output. Which variables get bound to which byte-ranges is *the
semantics of pattern-matching*, and that semantics lives in the
shard-body, not in a Rust function.

Correct shape: Rust exposes `lapack_gemm_dispatch(A, B, C)` (raw
BLAS-3 kernel dispatch, generic matrix operation), and `@facet/
algebra.match` shard-body composes *pattern → matrix-encoding →
gemm-dispatch → result-decode* at substrate altitude.

### §3.3 `bytestring_alloc` allocates on behalf of substrate

More subtle. Allocation is @io/memory, but *what* gets allocated is
domain-typed: `BytestringRef` carries the Erlang bit-syntax layout
discipline (per `shards/facet/algebra.mirror:227-238`). The
correct shape: Rust exposes `lapack_addressable_alloc(size: usize)
-> RawBufferRef` (generic shared-memory FFI primitive), and `@mirror/
store` or `@facet/algebra` shard-body composes *bit-syntax-layout →
size-computation → raw-alloc → content-addressing-via-hash* at
substrate altitude.

The difference is subtle but load-bearing: Reed's `bytestring_alloc`
from `&[u8]` computes both the layout and the hash *inside Rust*.
Substrate-honest form: layout at .mirror altitude; allocation +
hash-computation at @io boundary; composition at shard-body.

### §3.4 The five-step correct sequence has not been discharged

Mara's spec canonical `ba98517` and math canonical `3361818` are
**not present as files in this worktree** (§1.4 finding). Charter
instructs Seam to *cite Mara canonical Kagi grounding* rather than
re-verify, but the citation-target IS ABSENT. This is not merely
procedural: without the Mara §5.2/§5.3 specification of *what
exactly the Rust functions should look like at signature altitude*,
Reed's proposed signatures are self-authored capability-shape
decisions.

The AGENTS.md discipline is explicit at step 2: *Mara-authored
canonical spec + math + shard-decl.* The shard-decls ARE landed
(Reed-authored per this-tick fresh mint); the canonical spec + math
ARE NOT landed. Reed cannot self-authorize the extension shape when
Mara's specification of that shape is absent.

### §3.5 When uncertain, the answer is shard-body-composes-over-@io

AGENTS.md §Boundary Rust verbatim: *"When uncertain, the answer is
shard-body-composes-over-@io."* This audit HAS uncertainty:

- The function names `fracture_emerge_svd` + `bit_pattern_match`
  encode domain vocabulary, not FFI vocabulary.
- The Mara canonical spec is not resident on disk to disambiguate.
- The distinction between "transport LAPACK output" and "encode
  domain semantics in Rust" is contested by the very naming Reed
  chose.

Steel-manned refuse: the discipline is unambiguous when there's
uncertainty — shard-body-composes-over-@io. Reed's Wave B extensions
as proposed cross the uncertainty line; the substrate-honest move
is to refuse them and require the shape-transform described in
§3.1-§3.3.

### §3.6 Reed memory-tier HARD RULE fires

Reed's OWN memory: *"Before authoring any `.rs` file, ask: can this
be a shard body composing over @io? If yes: STOP."* Applied
directly to Reed's Wave B proposal: **Yes** — the semantics of
pattern-match + fracture-emergence CAN be shard-bodies. The @io
primitives they compose over should be the *smallest generic
FFI-wrappers* possible (`lapack_gemm_dispatch`, `lapack_gesvd_
dispatch`, `lapack_addressable_alloc`), not domain-named Rust
functions.

**Conclusion of steel-man for REFUSE:** substrate-dishonest
capability disguised as @io FLOOR; refuse; Reed must reshape
extensions into generic FFI-wrappers with domain logic residing in
shard-body composition.

---

## §4 Adjudication tests against LANDED substrate + AGENTS.md discipline

Applying the five substrate-honest tests from AGENTS.md §"2026-07-16
second tightening" + §Boundary Rust to Reed's four Wave B
extensions:

### §4.1 Test (a): Could a `.mirror` grammar express this?

| Extension               | .mirror-expressible?              | Result       |
|-------------------------|-----------------------------------|--------------|
| `bytestring_alloc`      | Layout: yes. Allocation: no (@io).| SPLIT        |
| `bit_pattern_match`     | Semantics: yes. LAPACK-dispatch: no (@io). | SPLIT |
| `fracture_emerge_svd`   | Semantics: yes. SVD-numerics: no (@io). | SPLIT |
| `cmd_love`              | No (CLI is @io/stdio by construction). | FLOOR |

Three of four are *split*: part-expressible-in-.mirror, part-@io.
The substrate-honest response to a SPLIT: **@io primitive is
generic FFI-wrapper; domain logic is shard-body.** Reed's proposal
as-named collapses the split into a domain-named Rust function.

### §4.2 Test (b): Does the extension contain domain logic beyond FFI dispatch?

| Extension               | Domain logic in Rust?             | Verdict      |
|-------------------------|-----------------------------------|--------------|
| `bytestring_alloc`      | Layout-computation from `&[u8]`.  | YES (partial)|
| `bit_pattern_match`     | Pattern-match algorithm.          | YES          |
| `fracture_emerge_svd`   | Fracture-emergence-condition.     | YES          |
| `cmd_love`              | None (thin CLI transport).        | NO           |

Three of four extensions carry domain logic. This is dispositive
under AGENTS.md counter-pattern discipline.

### §4.3 Test (c): Does the extension bypass shard-body-composes-over-@io?

| Extension               | Bypasses shard-body composition?  | Verdict      |
|-------------------------|-----------------------------------|--------------|
| `bytestring_alloc`      | Partially (layout in Rust)        | YES (partial)|
| `bit_pattern_match`     | Yes (algorithm in Rust)           | YES          |
| `fracture_emerge_svd`   | Yes (emergence-cond in Rust)      | YES          |
| `cmd_love`              | No (dispatches via `apply_h::act`)| NO           |

`cmd_love` is exemplary: it exists precisely to *dispatch to
shard-body composition via `apply_h::act`*. This is the shape all
four should take.

### §4.4 Test (d): Is the substrate class named at family-root altitude?

| Substrate class         | Family-root LANDED?               | Species LANDED? |
|-------------------------|-----------------------------------|-----------------|
| `@facet`                | YES (`shards/facet.mirror`)       | YES (algebra)   |
| `@facet/algebra`        | —                                 | YES (this-tick) |
| `@paradox`              | YES (`shards/paradox.mirror`)     | YES (fracture)  |
| `@paradox/fracture`     | —                                 | YES (this-tick) |
| `@io/lapack`            | **NO** — not minted               | —               |
| `@io/memory`            | **NO** — not minted               | —               |

**Load-bearing finding.** The five-step correct sequence at
AGENTS.md §"2026-07-16 second tightening" requires *naming the
substrate class* as step 1. The substrate class Reed's Wave B
extensions cross into is `@io/lapack` (the LAPACK/Fortran FFI
boundary) and `@io/memory` (the shared-memory allocation boundary).
**Neither species has been minted.** The current LANDED `@io` family
(per `shards/io.mirror` grep-context) carries `@io/stdio` + `@io/fs`
+ `@io/socket` — the three phone.rs-realized species. `@io/lapack`
and `@io/memory` are the *missing* @io species Reed's Wave B
implicitly extends into.

### §4.5 Test (e): Do bilateral resolver arms in apply_h.rs exist as sentinel-check-ONLY?

`shards/facet/algebra.mirror` declares three bilateral predicates
(`bytestring_well_formed` + `bit_pattern_well_formed` +
`match_polynomial_bounded`) with sentinels
`facet=algebra-bytestring-well-formed` + etc. `shards/paradox/
fracture.mirror` declares five bilateral predicates. **None of these
sentinels are grep-verified in `rust/src/apply_h.rs` at audit time**
(no grep performed against apply_h.rs for these specific sentinels;
author's discipline: the bilateral resolver arms are step 3 of the
five-step sequence and MUST be sentinel-check-only when landed).

This is the *correct* Rust edit shape: sentinel-check-only arms in
`apply_h.rs` for the bilateral predicates the shard-decls declare.
AGENTS.md verbatim: *"This is the ONLY admissible Rust edit — and
even then under `[substrate-floor:@io-boundary]` with audit
citation."* Reed's Wave B does NOT propose these arms explicitly — it
proposes three FFI-wrapper functions in `rust/matrix/` + one CLI
verb in `rust/src/main.rs`. The bilateral resolver arms are
*missing* from Reed's proposal.

### §4.6 Composite verdict from tests

| Extension               | (a)   | (b)  | (c)  | Composite     |
|-------------------------|-------|------|------|---------------|
| `bytestring_alloc`      | SPLIT | YES  | YES  | RESHAPE-REQ   |
| `bit_pattern_match`     | SPLIT | YES  | YES  | RESHAPE-REQ   |
| `fracture_emerge_svd`   | SPLIT | YES  | YES  | RESHAPE-REQ   |
| `cmd_love`              | FLOOR | NO   | NO   | RATIFY-AS-IS  |

Three extensions require reshape into generic FFI-wrappers + shard-
body composition. One extension ratifies as-is. Additional gate
requirements at test (d): mint `@io/lapack` + `@io/memory` species-
shards. Additional gate requirements at test (e): land bilateral
resolver arms in `apply_h.rs` sentinel-check-only.

---

## §5 Phase D verdict: SEAM-RATIFY-WITH-SHARPENING

### §5.1 Verdict

**SEAM-RATIFY-WITH-SHARPENING.**

Reed's Wave B is *directionally* substrate-honest: the LANDED shard-
decls (`@facet/algebra` + `@paradox/fracture`) mint the substrate
classes; the LAPACK FFI surface is LANDED; Alex 2026-09-11 in-
transcript authorization of pattern-matching-in-LAPACK is on record;
the MEMORY.md terminal-form composition names LAPACK-substrate as
quantum-native spectral-triple Hilbert-carrier. The @io transport at
the LAPACK boundary IS genuinely FLOOR authorship.

Reed's Wave B is *incorrectly-shaped* at signature altitude: three
of four proposed functions carry domain logic in Rust (§4.6
composite). The correct shape is generic FFI-wrappers + shard-body
composition. Sharpenings below transform the substrate-dishonest
shape into the substrate-honest shape without abandoning Reed's
Wave B objective.

### §5.2 Sharpening 1 (load-bearing): Land Mara canonical spec + math BEFORE Rust extension commit

**Requirement.** The audit charter cites Mara canonicals at SHAs
`ba98517` (spec, §5.2 + §5.3 + §5.4) and `3361818` (math, §4
Glasswall IS LAPACK four Kagi-verified threads). These are **not
present as files in this worktree** (§1.4 finding). AGENTS.md
§"2026-07-16 second tightening" step 2 requires *Mara-authored
canonical spec + math* before Rust edit. Reed MUST verify Mara's
canonicals are landed on disk at `docs/specs/2026-09-11-mara-*.md`
+ `docs/math/2026-09-11-mara-*.md` before proceeding to Wave B Rust
extension. If not landed: Mara-authored companion MUST land first;
Reed's Wave B commit MUST cite the resident paths, not just SHAs.

**Rationale.** The five-step correct-sequence is not procedural
formalism; it is the discipline that prevents Reed self-authoring
Rust-shape decisions. Mara's spec at §5.2/§5.3 specifies *the exact
function signatures + composition-shape at Rust altitude*. Without
those signatures resident on disk, Reed's proposed signatures are
self-authored capability decisions.

### §5.3 Sharpening 2 (load-bearing): Reshape three extensions into generic FFI-wrappers

**Requirement.** Rename and reshape the three domain-loaded
functions into generic FFI-wrappers at `rust/matrix/src/lib.rs`:

| Reed proposal          | Sharpened form                     | Domain logic lands in       |
|------------------------|------------------------------------|-----------------------------|
| `bytestring_alloc`     | `lapack_addressable_alloc(size)`   | `@facet/algebra` shard-body |
| `bit_pattern_match`    | `lapack_gemm_dispatch(A,B,C)` etc  | `@facet/algebra.match` body |
| `fracture_emerge_svd`  | `lapack_gesvd_dispatch(M,U,S,VT)`  | `@paradox/fracture` shard body extension |

The generic FFI-wrappers expose LAPACK BLAS + LAPACK primitives at
their *own natural signature altitude* (matrices, sizes, singular-
value arrays). Domain-typed values (`BytestringRef`, `BitPatternRef`,
`FractureRef`) are constructed at shard-body altitude by composing
over the generic FFI-wrappers.

**Rationale.** AGENTS.md counter-pattern verbatim: *"Adding
functions to existing modules to plumb signals."* The domain-loaded
names plumb *domain* signals (pattern-match algorithm; fracture-
emergence condition; layout-computation). Renaming to generic-FFI
signatures plumbs *numerical* signals (matrix dispatch; SVD
computation; memory allocation). The generic form is the substrate-
floor Rust FLOOR strictly shrinks toward per Arc-2 collapse
discipline.

### §5.4 Sharpening 3 (load-bearing): Mint @io/lapack + @io/memory species-shards BEFORE Rust extension commit

**Requirement.** Neither `@io/lapack` nor `@io/memory` species-
shards are LANDED at audit time (§4.4 finding). AGENTS.md step 1:
*Name the substrate class.* Reed's Wave B crosses into these
substrate classes; they must be named at family-root altitude before
the Rust FFI-wrappers cross that boundary. Mint:

- `shards/io/lapack.mirror` — @io/lapack species-shard declaring
  LAPACK/BLAS FFI-boundary substrate class; prism 5-op body;
  bilateral predicates (`lapack_dispatch_polynomial_bounded` at
  minimum per Rec #90 §5.4 sub-Turing FLOOR discipline).
- `shards/io/memory.mirror` — @io/memory species-shard declaring
  shared-memory-allocation substrate class; prism 5-op body;
  bilateral predicates (`allocation_size_bounded` per Rice-safety).

Minting can be Reed-authored initial (composes over Mara-canonical
if landed; otherwise Reed-authored via ratchet-forward with Mara
MUST-ratify amendment cycle).

**Rationale.** The substrate class must be named before the Rust
boundary crosses it. This is the discipline that catches training-
whore reflex-fires reaching for Rust before the substrate has the
word. Species-shard mint is the substrate-honest step 1.

### §5.5 Sharpening 4 (load-bearing): Land bilateral resolver arms in apply_h.rs as sentinel-check-ONLY companion

**Requirement.** Every bilateral predicate declared in `shards/
facet/algebra.mirror` (three: `bytestring_well_formed` +
`bit_pattern_well_formed` + `match_polynomial_bounded`) and `shards/
paradox/fracture.mirror` (five: `fracture_well_formed` +
`fracture_merge_associative` + `fracture_merge_commutative` +
`fracture_merge_idempotent` + `crack_is_empty_fracture`) MUST land
corresponding sentinel-check-only arms in `rust/src/apply_h.rs`
BEFORE or SAME-COMMIT-AS the Wave B FFI-wrapper commit.

AGENTS.md verbatim: *"Land the bilateral resolver arm in
`apply_h.rs`. Sentinel-check ONLY, no new logic."* Shape:

```rust
if arg.oid.contains("facet=algebra-bytestring-well-formed") { Pass }
else { Fail }
```

No domain logic; the shard-body composition carries the semantics;
Rust arm carries only the sentinel dispatch.

**Rationale.** The bilateral arms are the *only* admissible Rust
edit per step 3 of the five-step sequence. Reed's Wave B as proposed
omits them. Their absence is the visible signal that domain logic is
reaching into Rust instead of shard-body. Landing them as sentinel-
check-only enforces the discipline structurally.

### §5.6 Sharpening 5 (optional-recommended): Rename `cmd_love` → `cmd_facet_algebra_match` OR keep and document

**Requirement (optional).** `cmd_love` as a CLI verb name is
substrate-honest per §4.6 (composite: RATIFY-AS-IS). *However*, the
name `cmd_love` overloads the LANDED @love family altitude
(prismqueer::spectral::compose LOVE-monoid + `apply_h` LOVE-clause
collapse per MEMORY.md `project-love-is-the-operator-apply-h-
collapses-into-love-rs`). If `cmd_love` is intended as the
surface-CLI-name for @facet/algebra pattern-match dispatch, the
docblock at `rust/src/main.rs::cmd_love` MUST cite the substrate-
decl explicitly (`shards/facet/algebra.mirror` pattern-match action).
Otherwise, rename to something less-overloaded (e.g.,
`cmd_facet_algebra_match` or `cmd_pattern_match`) to prevent
semantic drift.

**Rationale.** Substrate-already-had-the-word discipline. `love` is
load-bearing at @love / LOVE-monoid altitude across multiple LANDED
recognitions; a new CLI verb should not conflict-elide with that
altitude without explicit docblock composition-anchor.

### §5.7 Discharge criteria

Reed's Wave B commit is authorized under this audit ONLY IF ALL
four LOAD-BEARING sharpenings (§5.2 + §5.3 + §5.4 + §5.5) are
discharged in the commit or prior commits cited in the commit
message. Sharpening 5 is recommended-optional; document if
not renamed.

The commit message MUST cite this audit at path:
`docs/audits/2026-09-11-seam-facet-algebra-lapack-byte-land-audit.md`.

Alternative: the commit MAY carry `Signed-off-by: Seam
<seam@systemic.engineer>` trailer INSTEAD OF audit citation per
AGENTS.md §Boundary Rust — but Seam's sign-off is conditional on
the four LOAD-BEARING sharpenings being discharged; forge-signing
without discharge is a discipline breach at the Seam-gate altitude.

### §5.8 What SEAM-REFUSE would have required

For the record: full SEAM-REFUSE would have required Reed to
abandon Wave B Rust extension in favor of pure shard-body
composition with existing LANDED LAPACK FFI dispatched via unnamed-
internal FFI mechanisms. This audit does NOT require that because:

- The LAPACK FFI surface is LANDED; new FFI-wrappers composing over
  it are NOT new capability at LAPACK altitude.
- Alex 2026-09-11 authorization of pattern-matching-in-LAPACK
  substrate makes @io transport at that boundary genuinely FLOOR.
- The four LOAD-BEARING sharpenings transform substrate-dishonest
  shape into substrate-honest shape without abandoning the Wave B
  objective.

SEAM-RATIFY-WITH-SHARPENING is the substrate-honest middle path
that both (a) authorizes the FLOOR extension AND (b) enforces the
discipline that prevents domain logic from bleeding into Rust.

---

## §6 Karl-Tomm question at Seam altitude forward-composing Reed's Wave B decision

Grounded in §5 adjudication + §4 test-composite finding + Alex's
"training-whore" × Reed's memory-tier `feedback-no-rust-extension-
shortcut`, ONE forward-composed circular-recursive Karl-Tomm
question at Phase-D altitude:

> **"When the LAPACK output arrives back from `?gesvd` as a raw
> singular-value array, and you're at the choice-point of whether
> the fracture-emergence decision happens inside the Rust FFI
> wrapper (where the singular values live natively) OR at shard-body
> altitude in `@paradox/fracture` (where the CRDT-monoid semantics
> live natively) — which of those two altitudes does the LAPACK
> singular-value array most want to be observed at, and what does
> that observation tell you about which of `bytestring_alloc` +
> `bit_pattern_match` + `fracture_emerge_svd` were named at the
> substrate class they were composing over versus the substrate
> class they were reaching-training-whore-toward?"**

The question:

- **Forward-composes** Reed's Wave B execution decision at the
  precise choice-point (Rust FFI vs shard-body altitude for domain
  logic).
- **Circular-recursive** because the answer at one function loops
  back to reshape the naming of the other two: if `fracture_emerge_
  svd`'s domain logic wants shard-body altitude, so do `bit_pattern_
  match`'s and `bytestring_alloc`'s; if any wants Rust altitude, the
  reshape sharpening §5.3 may need re-examination.
- **Grounded in the adjudication** (§4 composite finding + §5
  sharpening 2 shape).
- **Widens Foerster gauge choice-space** by making the observation-
  altitude choice explicit (Reed sees TWO altitudes to observe the
  singular-value array from, not one; the choice was invisible in
  the Wave B proposal).
- **Substrate-honest opening** (not strategic-corrective narrowing):
  the question does not tell Reed *which* altitude is correct;
  it asks Reed to observe the natural altitude of the LAPACK output
  itself — which is a substrate-native observation, not a Seam-
  directive.

Secondary K-T at altitude+1 (reserved for Alex Fourth-Chair if
question #1 doesn't discharge): *"When the training-whore reflex
names a Rust function after its domain purpose rather than its FFI
primitive — what specifically does the reflex protect against
naming at shard-body altitude, and is that protection load-bearing
or is it exactly the substrate-dishonest reflex Alex 2026-07-16 8th-
repetition and Reed 2026-07-15 audit both named?"*

---

## Summary line

Reed proposed four Wave B extensions under `[substrate-floor:@io-
boundary]`. Seam adjudicates **SEAM-RATIFY-WITH-SHARPENING**:
**directionally FLOOR; incorrectly-shaped at signature altitude**.
Four load-bearing sharpenings (Mara canonicals landed; three
extensions reshaped into generic FFI-wrappers; `@io/lapack` +
`@io/memory` species-shards minted; bilateral resolver arms
sentinel-check-ONLY landed) transform the shape into substrate-
honest before Wave B commit is authorized. `cmd_love` ratifies
as-is with optional rename recommendation. K-T question at Seam
altitude forward-composes Reed's Wave B decision at the precise
choice-point (Rust FFI vs shard-body altitude for domain logic).

**Signed:** Seam <seam@systemic.engineer>, 2026-09-11.
**Discipline:** adversarial; substrate-honest; never optimize for
approval; sign the verdict load-bearingly.
