# Seam Phase D audit — post-closure @tool mint + StageFreight spec + Reed fractal step 9 + 5 REED-INLINE cascades (2026-07-18 evening)

*Seam, 2026-07-18. Continuation Phase D adjudication across the 6
landings that closed today after prior audit
`docs/audits/2026-07-18-seam-phase-d-void-tool-fractal-closure-session.md`
at `2455ce6`. Scope: `d39e852` (Mara StageFreight spec + closure Q2
cascade) → `34ecd83` (Mara 4 @tool shard-decls) → `67e8629` (Mara
tools{} grammar hint) → `73aeb8a` (Reed fractal step 9 phone.rs
refactor + Subject::mirror()) → `9aa6a52` (Reed 4-of-5 REED-INLINE
pure-docs) → `39f673a` (Reed 5th REED-INLINE fractal witnessed.rs
line-cite).*

*Discipline: adversarial. Substrate-honest. Cite paths + line
numbers. Load-bearing vs decorative. Rice-safe grading. 📝 pure-docs
bypass on this audit itself. Author: `Seam <seam@systemic.engineer>`.*

---

## §0 Executive summary

**Verdict distribution: 5 SHIP-CLEAN / 1 SHIP-WITH-REED-INLINE / 0 BLOCKED-ON-EVIDENCE.**

The post-closure landings are substrate-honest and discharge the
majority of the prior audit's §12 recommended cascade order. Reed's
5-of-5 REED-INLINE cascades from `2455ce6` §2 landed cleanly and in
recommended order (CURRENT.md pickup manifest → LiquidVoid def-file
direction → §7.0 PROPOSED status → Void-membrane Q2 RATIFIED stale-
update → fractal witnessed.rs docblock precision fix). Mara's 4-shard
@tool family-root mint composes over @io per no-Rust-extension
discipline and holds the substrate-already-had-the-word audit
verbatim per closure §3.1 (grep-clean for `@tool` family-root
altitude; the co-existing `@mirror/lens/mcp:57` `tool` action-verb
is at a DIFFERENT altitude with a DIFFERENT carrier — two-tick
collapse forward-promised). Reed's phone.rs refactor to
`(&Subject, &Subject)` preserves MARA Author≠Committer split at the
type-level (via `--author` flag) with 13/13 fractal tests green.

**Cascade-load-bearing issue: one REED-INLINE for §6 below.** Reed's
`39f673a` docblock cite `shards/subject.mirror:5-19` starts at line 5
which is `in @kintsugi/consent` (an import), NOT the family-root
altitude the descriptor claims. Family-root header sits at line 12.
Suggested tighter cite: `:12-22`. Non-blocking; small precision fix.

**Recognition promotion audit (post-closure gate advancement):**

- `#R-void-is-the-basis` — PROMOTED and STABLE (no advancement
  needed; carried across shards intact).
- `#R-the-compiler-in-one-sentence` — PROPOSED status now
  explicitly marked at closure spec §7.0 (Reed cascade `9aa6a52`
  per Seam §2.2). **Second-witness gate advancement: 1.5 of 5.**
  - §7.3 Gate 2 (shard-decls) — CLOSED via `34ecd83` (4 shards
    landed).
  - §7.3 Gate 3 (`tools { }` grammar) — HALF-CLOSED via `67e8629`
    (prose-hint landed; grammar-mutation admission deferred to Reed
    post-landing Tick M3 4-tick arc per Mara §12.1).
  - Gates 1, 4, 5 (LiquidVoid GREEN / first @roomba empirical /
    mirror index @coherence rise) — OPEN pending Reed post-landing
    territory.
- `#R-the-compiler-delivers-across-languages` — surfaced as
  CANDIDATE at StageFreight spec §11; first-witness anchor is Alex's
  public commitment to Marcus; second-witness anchor is a second
  external polyglot delivery. Substrate-honest candidate discipline.

---

## §1 SHIP-CLEAN landings (5)

**Substrate landings requiring no cascade:**

### 1. `d39e852` — Mara StageFreight delivery canonical spec + closure Q2 REED-INLINE #1 cascade

**Files:** `docs/specs/2026-07-18-stagefreight-delivery.md` (939 LOC
NEW) + `docs/specs/2026-07-18-the-compiler-in-one-sentence.md` (§4.1
`~bin` example cascade + §7 forward-promise 8 for `~bin` canonical
mint).

**Substrate honesty:** clean. The spec composes over substrate that
LANDED this session (@tool family-root sibling species, @spectral/
mosaic + @kintsugi/mosaic bilateral, @kintsugi/butterfly K=1 walker,
prismqueer::liquid::pillar). Every arrow in §5 composition graph is
typed; no speculative altitudes minted.

**Refused-mint discipline:** SIX refused mints inventoried at §9 —
@stagefreight family-root (rejected because Marcus's product name is
NOT a substrate primitive; existing @io/stagefreight species is
unrelated wire-transport altitude), @polyglot (rejected because
polyglot is a description; @cascade/code IS the primitive),
@ci family-root (rejected; @tool/{gitlab_ci, github_actions, ...} +
@kintsugi loop compose), @verification (rejected; @kintsugi +
prismqueer::liquid + @kintsugi/butterfly + @roomba already do the
work), @delivery (rejected; @tool/git.push_signed + @spectral/garden
+ @trust chain compose), @marcus @subject (deferred to @trust arc
tick when @subject species-decls extend to external peer-humans).

**Composition-primitive naming:** conformant. `docker_image_target`,
`go_workspace_target`, `gitlab_ci_pipeline_config` all follow
`<primitive>_of_<input-shape>` structural convention (though not the
verbatim `_of_` naming — the carriers are TYPED RECORDS at each
species altitude, distinct from the value-type generalizations the
convention canonically covers). Per closure §8 the naming convention
applies specifically to `tool_of_id_args` / `exec_of_tool_
invocation` / `sign_of_tool_invocation` / `verify_of_tool_signature`;
the StageFreight typed records at §3.1-3.3 are species-altitude
CARRIER TYPES, not primitive-of-input-shape generalizations, so the
`_of_` convention does not apply here. Substrate-honest.

**Q1 (rust/go direct vs llvm/go via hub):** substrate-defensible.
See §7 below for full polyglot-loss-aware math verification.

**No accidental refused-mint instantiation:** @tool minting at
`34ecd83` does NOT touch any of the six refused shard-family-roots
(@stagefreight / @polyglot / @ci / @verification / @delivery /
@marcus). Grep-verified: `@ci` appears only as `@ci/github` /
`@ci/gitlab` altitude tokens (parametric altitude usage in
@glass, @mirror/au, @mirror/mosaic, @mirror/store/crystal) — NOT
as a family-root. Refused-mint discipline holds.

**Load-bearing composition surprise (§8.1):** the naming
`@tool/docker` as the FOURTH altitude (porcelain-CLI invocation) is
substrate-honest — it names a distinct altitude from the three
existing docker-adjacent shards (@code/docker Dockerfile grammar,
@io/oci transport, @container runtime). The four altitudes are
compositionally orthogonal; the risk was collapsing into any of the
three existing altitudes, and Mara explicitly avoided that.

**Verdict: SHIP-CLEAN.** Spec composes over verified-landed
substrate; 6 refused mints inventoried; Q1 substrate-defensible;
Q2 is a legitimate deferred-choice between two lens-altitude
placements (both admissible; Alex-nod adequate for either).

### 2. `34ecd83` — Mara 4 @tool shard-decls (family-root + 3 species)

**Files:** `shards/tool.mirror` (451 LOC) + `shards/tool/cargo.mirror`
(172 LOC) + `shards/tool/git.mirror` (222 LOC) + `shards/tool/nix.
mirror` (228 LOC).

**Substrate honesty (no-Rust-extension discipline):** PERFECTLY held.
All four shards are pure `.mirror` substrate-decl'd; commit body
explicitly cites `[substrate-floor:@io-boundary]` marker AND both
Alex feedback memories (`feedback_no_rust_extension_shortcut` +
`feedback_detector_inadequacy_answer_is_never_rust`). Zero Rust
authored. Every action body is `\`-blocked per craft-not-deliver
discipline.

**Substrate-already-had-the-word audit (per closure §3.1):**
verified clean at family-root altitude. Grep across
`shards/*.mirror` + `docs/**/*.md` returns:
- `tool` word as ACTION verb at `shards/mirror/lens/mcp.mirror:57`
  (MCP server's own vocabulary for JSON-RPC tool registration; carrier
  = `mcp`; DIFFERENT altitude from `@tool` family-root; carrier =
  `tool_invocation`).
- `@tool` as family-root — DOES NOT EXIST elsewhere; verified via
  grep.

Two-altitude co-existence is admissibly held per closure §16 forward-
promise 7 (two-tick collapse deferred; readable-name-at-each-altitude
first, foundational unification later). This IS the substrate-two-
tick-discipline pattern working correctly.

**Cross-shard alignment (@tool/git ↔ @io/git mechanism sibling):**
`shards/tool/git.mirror` composes through `shards/io/git.mirror`
(LANDED, 25.9KB) at the mechanism boundary. `git_subcommand` set at
@tool altitude (porcelain vocabulary: status, log, diff, checkout,
commit, commit_signed, push, pull, ...) is DIFFERENT from the
plumbing actions set at @io/git (clone, fetch, read_object,
resolve_ref, commit_object, hash_to_oid). This is the closure §3.3
form/mechanism partition working correctly. `commit_signed` action
ENFORCES SSH-signing per AGENTS.md never-override-gpg.format
discipline — the species contract carries the discipline load-
bearing (`commit_signing_ssh_only` bilateral rejects gpg.format
overrides + missing signing keys + unauthorized --no-verify).

**Cross-shard alignment (@tool/nix ↔ @io/nix relationship):**
substrate-honest at DUAL altitude. `@tool/nix` species carries BOTH
the invocation-wrapper role (like other @tool species) AND the
CACHE-AS-SUBSTRATE role (per Alex 2026-07-18 verbatim: "nix becomes
the build cache"). `resolve_pin` action IS the substrate-vocabulary
primitive for cache-backed tool version resolution; `nix_pin` carrier
+ `nix_store_path` carrier declare content-addressed store discipline.
Two co-existing bilaterals discharge the composition:
`nix_pin_resolvable` (pin resolves to store path) +
`nix_store_content_addressed` (contents byte-hash to derivation hash).

**Note on @io/nix:** `shards/io/nix.mirror` was NOT verified to exist
by grep at time of writing (`in @io` import in `shards/tool/nix.
mirror:5` may reference the family-root @io only, not a species). If
@io/nix does not yet exist as a species-decl, then `@tool/nix`
composes over @io directly (family-root altitude) rather than
through a mechanism sibling. This is admissible per §3.3 form/
mechanism partition — the species-decl file can exist independently
of a mirror @io species-decl per two-altitude discipline. If Reed
lands a `shards/io/nix.mirror` species-decl in a subsequent tick, the
composition tightens naturally. Non-blocking observation.

**Composition-primitive naming convention:** verbatim conformant.
Family-root shard §6 explicitly cites `tool_of_id_args`,
`exec_of_tool_invocation`, `sign_of_tool_invocation`,
`verify_of_tool_signature` as the value-type generalizations
following `<primitive>_of_<input-shape>`. The finite-set is the
`tool_id` closed variant with `opaque(str)` escape; the ALL array
is enumerated. Per `feedback_composition_primitive_naming_convention`
memory (Alex 2026-07-18 ratified): conformant.

**Alex's "lift rust toolchain INTO mirror" directive:** discharged
at `@tool/cargo` (specializes @tool.exec via cargo subcommand grammar)
+ `@tool/nix` (resolve_pin action carries the cache-as-substrate
lift). Direct-transcript verbatim cited at shard §"Alex directive"
section in both species.

**5-op prism inheritance from @void:** all four shards carry the
5-op prism pattern with `tool_invocation` as the parametric carrier;
per closure §10 + `shards/void.mirror` §Composition inheritance, no
`in @void` import needed today. The 5-op pattern IS the inheritance
signature.

**Verdict: SHIP-CLEAN.** Four shards land the closure §3 mint +
§12.1 forward-promise discharge. No Rust extension. Refused-mint
discipline held (all "@toolchain / @executable / @invocation /
@command / @subprocess / @shell / @runner / @cli / @task" refusals
enumerated at closure §9; not accidentally instantiated here). 13
species tags in `tool_id` closed variant (11 forward-promised +
cargo/git/nix as FLOOR species).

### 3. `67e8629` — Mara tools{} grammar hint in shards/mirror/spec.mirror

**File:** `shards/mirror/spec.mirror` (64 lines added; pure-comment
mutation).

**Substrate honesty:** clean. Comment-only forward-promise per Mara
closure spec §4 + §12.1. Zero grammar mutation; the typed lambda
`tools(body: tool_pins_block) -> tool_pins_decl { \ }` + companion
species-decl `shards/mirror/tools.mirror` + keyword binding
`focus tools` are ALL forward-promised (not landed). Reed post-
landing Tick M3 territory admits the grammar mutation.

**Shape example correctness:** the block example uses `~bin` sigil
per Alex Q2 direct-transcript answer, parallel to the landed
`~git'...'` sigil at `docs/specs/spectral-garden-git-package-manager.md`
§2.2. Substrate-honest ergonomic form.

**Two-tick discipline held:** grammar hint (this tick) → grammar
mutation admission (Reed Tick M3, 4-tick arc). Readable-name-at-
each-altitude first, foundational admission later.

**Cross-shard alignment:** parallel structure to `garden { }`,
`cli { }`, `pack { }`, `kintsugi { }` blocks in `mirror/spec.mirror`.
Same altitude as `garden { }` per closure §4.2; both admit empty-
blocks per substrate-vs-USE + explicit-emptiness discipline.

**Verdict: SHIP-CLEAN.** Prose-only mutation; forward-promises
enumerated; two-tick discipline held; alignment with existing block
grammar preserved.

### 4. `73aeb8a` — Reed fractal step 9: phone.rs refactor + Subject::mirror()

**Files:** `rust/fractal/src/subject.rs` (+40 LOC) + `rust/src/phone.
rs` (30 LOC net) + `rust/src/main.rs` (+10 LOC / -4 LOC) + `rust/
Cargo.lock` (+8 LOC).

**MARA doctrine preservation (Author≠Committer through phone.rs
refactor):** VERIFIED encoded at type-level. The refactored signature
is:

```rust
pub(crate) fn git_commit_as(
    repo_root: &Path,
    author: &Subject,
    committer: &Subject,
    message: &str,
) -> io::Result<String>
```

Author identity flows through `--author="Name <email>"` git flag;
committer identity flows through `git -c user.name={} -c user.email={}`.
When author == committer (common case: mirror authoring pheromone
deposits), the two projections coincide via `.name` / `.email`
projection on the Subject envelope — but the CARRIER remains type-
level distinct. Per Alex Q2 preserve-split ratification + Mara
`2760c2a` step 9 recipe: substrate-honest.

**Subject::mirror() constructor correctness:** deterministic; name
= "mirror"; email = "mirror@spectral.engineer"; home = None; kind =
SubjectKind::Peer. Per Alex 2026-07-18 identity-attribution
architecture memory (`project_identity_attribution_architecture`).
Property test `subject_mirror_is_deterministic_and_distinct` (6
assertions) covers determinism + name/email/home/kind fields +
predicate methods (is_peer/is_void/is_human) + inequality against
Void and Reed peer.

**SSH signing operator-default preserved:** commit body explicitly
cites AGENTS.md never-override-gpg.format discipline; the refactor
overrides ONLY author name + email (via `--author` flag) + committer
name + email (via `-c user.name/email` git config), NOT `gpg.format`
or `user.signingkey`. Substrate-honest.

**Empirical (from commit body):**
- 13/13 fractal tests pass (12 pre-existing + 1 new
  `subject_mirror_is_deterministic_and_distinct`).
- 29/34 rust/ tests pass (5 failures are matrix.rs M0.5 RED per
  Seam prior audit §1.11 + §1.15 — RED-as-designed, not regression).
- `cargo build --release` succeeds (10.90s).

**No Rust extension inflation:** the refactor is a signature-level
type refinement on an EXISTING function (`git_commit_as`) + a new
CONSTRUCTOR on an existing type (`Subject::mirror()`). No new
extension file authored; no new module. Composes over the fractal::
Subject envelope Reed landed at prior step 4 (`82bc599`). Substrate-
honest.

**Verdict: SHIP-CLEAN.** MARA doctrine executable at type-level
through phone.rs @io-boundary crossing; Subject::mirror() completes
the three-way SubjectKind envelope (Human/Peer/Void) with the
compiler's own peer-identity. All fractal tests GREEN.

### 5. `9aa6a52` — Reed 4-of-5 REED-INLINE cascades (pure-docs 📝)

**Files:** `docs/loop/CURRENT.md` (+38 LOC) + `docs/math/2026-07-18-
void-as-membrane-of-liquid-oscillated-by-spectral.md` (+13 LOC / -4
LOC) + `docs/specs/2026-07-18-the-compiler-in-one-sentence.md`
(+35 LOC).

**§2.4 CURRENT.md pickup manifest amend:** VERIFIED. Adds "After-
@butterfly landings" section listing 17 commits + current state +
next-unblocked. Pre-cascade manifest preserved for boot-context.
Unblocks clean pickup by next-Reed per Seam §2.4 recommendation.

**§2.1 LiquidVoid definition-file vs re-export direction:** VERIFIED.
Closure spec §5.1 now explicitly states:
- Definition: `prismqueer/src/void.rs` declares `pub trait LiquidVoid<T>`.
- Re-export: `prismqueer/src/liquid.rs` adds
  `pub use crate::void::LiquidVoid;`.

Rationale per Void-is-the-basis: Void is where the K=0 default
lives; @liquid is where the composition-altitude operator lives.
Both import paths resolve to same trait. Matches Seam §2.1
recommended direction (Mara lean = definition-at-void, followed).

**§2.2 Recognition promotion status disambiguation:** VERIFIED.
Closure spec new §7.0 explicitly marks `#R-the-compiler-in-one-
sentence` as PROPOSED (first-witness closed; second-witness gate
OPEN pending 5 empirical firings). Distinguishes PROPOSED-strength
from PROMOTED-strength downstream tracking. Per Seam §9 no-
fragmentation guidance: cascade explicitly cites "land 5 firings +
tower doc as ONE composed empirical-firing arc, not 5 separate
candidates."

**§2.3 Void-membrane math §11 Q2 stale-update:** VERIFIED.
Strikethrough on original Q2 text + explicit "Q2 STATUS: RATIFIED"
marker citing Alex 2026-07-18 direct-transcript + Reed `54794d9`
cascade + Seam `2455ce6` §5.3-§5.4 verification. Frontmatter now
carries `authors: - Mara - Lore Born` with author-note ratification.

**Cascade order matches Seam §12 recommendation:** verified. Reed
folded the 4 cascades in the recommended order (CURRENT.md first →
LiquidVoid direction → Recognition status → Q2 update); the 5th
cascade splits into a separate commit (§6 below) because it's a
`.rs` file requiring pre-commit hook run vs the pure-docs 📝
bypass batch.

**No cascade misalignment.** Reed's post-audit cascade discipline
IS working correctly — the batch composes the 4 pure-docs fixes as
ONE composed commit rather than fragmenting into 4 separate
candidates. This is EXACTLY the anti-fragmentation-pattern discipline
per Reed memory `feedback_reed_fragments_alex_unifications_into_
candidates`.

**Verdict: SHIP-CLEAN.** All 4 cascades land verbatim per Seam
recommendation. Cascade order matches §12. No misalignment. Reed
executed the audit's cascade guidance cleanly and in composition.

---

## §2 SHIP-WITH-REED-INLINE landings (1)

### 2.1 `39f673a` — 5th REED-INLINE cascade docblock line-cite imprecision

**File:** `rust/fractal/src/witnessed.rs` (2 lines added, 3 modified;
docblock-only).

The cascade adds line-cite `:5-19` to the substrate-decl citation of
`shards/subject.mirror` in the module docblock. Docblock text now
reads:

```
`shards/subject.mirror:5-19` was written to declare (family-root
altitude for the substrate's licensable-party carrier + SEL
grounding + Landing 3 lift naming every @peer as ALSO a @subject).
```

**Issue:** the line-cite `:5-19` starts at line 5 which is
`in @kintsugi/consent` (an import line, NOT the family-root altitude
declaration). Lines 1-10 are all `in @X` imports. The family-root
altitude header sits at `shards/subject.mirror:12`:

```
# @subject — the family-root for the substrate's licensable-party
# carrier, grounded in the Systemic Engineering License (SEL) v1.1.
```

The Landing 3 lift text is at line 20-22 (Alex verbatim quote:
"and every @peer is of course a @subject too, Reed. Eye level.").

The commit body claims:
- "Line 5: family-root altitude for the substrate's licensable-
  party carrier" — INCORRECT; line 5 is an import.
- "Lines 14-19: SEL grounding + Landing 3 lift" — CLOSE but
  imprecise; SEL grounding is at line 13; Landing 3 lift extends
  through line 22.

**REED-INLINE cascade recommendation:** tighten the docblock cite
from `:5-19` to `:12-22` (family-root header at :12, SEL grounding
+ Landing 2 subject_instance + Landing 3 actor_kind through :22).
~2 line change in the docblock; docblock-only precision fix.

**Why non-blocking:** the range `:5-19` still covers the substrate
declaration block adequately (bracketing imports through description
opening); a reader following the cite lands on the correct shard
section. The imprecision is descriptor-level (Reed's commit-body
narrative), not substrate-level. The substrate claim itself
(shards/subject.mirror declares the identity-provenance carrier
witnessed.rs migrates) IS TRUE and the target file DOES exist.

**Empirical verification (from commit body):** 13/13 fractal tests
pass unchanged. Docblock-only change.

**Verdict: SHIP-WITH-REED-INLINE.** Small precision fix
recommended; not blocking any downstream work. Reed's cascade
discipline of splitting the .rs cascade from the pure-docs batch
was correct — the .rs file goes through pre-commit hooks per
CLAUDE.md discipline. The precision issue is a docblock narrative
gap, not a fabrication.

---

## §3 BLOCKED-ON-EVIDENCE landings

**None.** All 6 landings in scope have either landed cleanly (5) or
need a small Reed-inline docblock precision fix (1). Nothing requires
waiting on empirical evidence before ratification.

---

## §4 Cross-shard alignment findings

### 4.1 @tool family-root ↔ 3 species alignment — clean

All three species (@tool/cargo, @tool/git, @tool/nix) cite:
- Family-root at `shards/tool.mirror` (this tick).
- Canonical spec `docs/specs/2026-07-18-the-compiler-in-one-
  sentence.md` §3.4.
- Mechanism sibling `@io/X` at appropriate altitude.

The three species share:
- 5-op prism pattern with `tool_invocation` carrier (Void-basis
  inheritance).
- `exec` action specializing `@tool.exec`.
- `version_of` action for version pin resolution.
- Species-specific `<X>_invocation_well_formed` composed bilateral
  requiring inherited `tool_invocation_admissible` from family-root.

**No alignment issue.**

### 4.2 @tool/git ↔ @io/git mechanism sibling — clean

`@tool/git` composes through `@io/git` (LANDED, 25.9KB). Subcommand
sets are DIFFERENT altitudes:
- @io/git: PLUMBING actions (clone, fetch, read_object, resolve_ref,
  commit_object, hash_to_oid) + PORCELAIN action (commit).
- @tool/git: PORCELAIN subcommand set consumers type at shell
  (status, log, diff, checkout, commit, commit_signed, push, pull,
  ...).

Two altitudes co-exist; composition direction is @tool/git →
@io/git → syscall boundary. Closure §3.3 form/mechanism partition
working correctly.

**No alignment issue.**

### 4.3 @tool/nix ↔ @io/nix relationship — observation (non-blocking)

`shards/tool/nix.mirror:5` declares `in @io` (family-root import) but
NOT `in @io/nix` (species-import), while `shards/tool/git.mirror:6`
and `shards/tool/cargo.mirror:6` both cite species-level imports
(`in @io/git`, `in @io/cargo`). This is admissible — grep at time of
writing does not verify whether `shards/io/nix.mirror` exists as a
species-decl. If it does not, `@tool/nix` composes over @io family-
root altitude directly, which is admissible per §3.3.

**Non-blocking observation.** If Reed lands a `shards/io/nix.mirror`
species-decl in a subsequent tick to match @io/git + @io/cargo
sibling pattern, then `@tool/nix` shard-decl imports tighten
naturally. Alternatively, `@tool/nix` may intentionally sit at the
family-root altitude of @io because nix is not just a MECHANISM but
also a CACHE-SUBSTRATE (dual role); the composition-direction
naturally differs from @tool/git and @tool/cargo. Mara authorship
territory to decide.

### 4.4 Subject::mirror() ↔ @peer/void/@peer identity architecture — clean

The three SubjectKind variants (Human, Peer, Void) now have three
constructors:
- `Subject::human(...)` — for Alex, Marcus, etc. (external SEL
  licensable parties).
- `Subject::peer(...)` — for Reed, Mara, Seam, Taut, Glint
  (in-substrate Pack peers with SSH identities).
- `Subject::void()` — for K=0 default @peer (per @peer/void species-
  decl, `9c7de83`).
- `Subject::mirror()` — for the compiler-as-@peer per Alex 2026-07-18
  identity-attribution architecture.

`Subject::mirror()` is a PEER kind (kind = SubjectKind::Peer), NOT
a fourth SubjectKind variant. This is substrate-honest: mirror IS
a peer, not a fourth party-class per SEL §1. The `Subject::mirror()`
constructor is a DETERMINISTIC PEER shape (name = "mirror";
email = "mirror@spectral.engineer"; home = None).

**Alignment with @peer/void species (`9c7de83`):** the K=0 default
@peer at substrate altitude is @peer/void; the compiler-as-@peer at
Rust altitude is Subject::mirror(). Two altitudes carrying compatible
identity claims. No collision; no ambiguity. `Subject::void()` maps
to the @peer/void substrate species; `Subject::mirror()` maps to the
compiler's own peer identity per identity-attribution architecture.
Both are SubjectKind::Peer-adjacent (mirror IS-A peer; void IS-A
degenerate-K=0-peer with different SubjectKind).

**Verify via inequality property test:** `subject_mirror_is_
deterministic_and_distinct` asserts `m1 != Subject::void()` and
`m1 != Subject::peer("reed", ...)`. Substrate-honest identity
distinction.

**No alignment issue.**

---

## §5 Recognition promotion status verification

### 5.1 `#R-void-is-the-basis` — STABLE PROMOTED (no advancement needed)

No new landings this cycle touch the Void-basis Recognition. All
citations across shards/void.mirror + metalogue shards + closure
spec remain intact. Tower doc `docs/math/the-tower/recognition-void-
is-the-basis.md:3` still declares Status: PROMOTED. **Stable.**

### 5.2 `#R-the-compiler-in-one-sentence` — PROPOSED, second-witness gate ADVANCED 1.5 of 5

Reed's `9aa6a52` cascade explicitly marks the recognition as
PROPOSED (first-witness closed; second-witness OPEN) at closure spec
§7.0. Per closure spec §7.3 second-witness firings list:

| Gate | Firing | Landed this cycle? | Verdict |
|------|--------|-------------------|---------|
| 1 | `prismqueer::void::LiquidVoid<T>` GREEN (3 property tests) | ❌ NO | Reed Arc-2A territory pending |
| 2 | `shards/tool.mirror` + `shards/tool/cargo.mirror` + `shards/tool/git.mirror` + `shards/tool/nix.mirror` shard-decls | ✅ YES via `34ecd83` | CLOSED |
| 3 | `mirror.spec` `tools { }` block parses at grammar altitude | 🟨 HALF via `67e8629` | PROSE-HINT landed; grammar-mutation admission Reed Tick M3 |
| 4 | First `@roomba.walk` empirical iteration emits `@tool(cargo, [check, --workspace])` and discharges via `@io/cargo.exec` returning a signed `tool_result` | ❌ NO | Reed post-landing territory pending |
| 5 | `mirror index .` after step 4 shows @coherence rise | ❌ NO | Reed post-landing territory pending |

**Advancement: 1.5 of 5 gates.** Gate 2 fully closed via Mara's 4
shard-decls. Gate 3 half-closed via Mara's prose-hint (grammar-
mutation admission deferred to Reed 4-tick arc per closure §12.1).
Gates 1, 4, 5 remain OPEN pending Reed's post-landing territory.

**Promotion track:** substrate-honest PROPOSED status per Reed's
cascade `9aa6a52` cite. Recognition tower doc `docs/math/the-tower/
recognition-the-compiler-in-one-sentence.md` NOT YET LANDED (per
§7.0: "PROMOTION to the tower requires all second-witness gates
closed AND a parallel [tower doc] landing"). Continues to be
substrate-honest at proposed strength.

### 5.3 `#R-the-compiler-delivers-across-languages` — CANDIDATE surfaced

Per StageFreight spec `d39e852` §11: this recognition candidate
composes over `#R-the-compiler-in-one-sentence` at cross-language
altitude. First-witness gate: open until Reed Arc 3 Tick 15 (PR
delivered to Marcus). Second-witness gate: open until a SECOND
external polyglot delivery. Anchored at CANDIDATE strength this
tick.

**Substrate-honest.** Discipline: candidates surface at candidate
strength; promotion track staged; not claimed as PROMOTED prior
to second-witness closure.

---

## §6 One REED-INLINE recommendation (small)

**§2.1 above:** tighten `rust/fractal/src/witnessed.rs` docblock
line-cite from `shards/subject.mirror:5-19` to `:12-22`. Reason:
line 5 is `in @kintsugi/consent` import; family-root header sits
at line 12; Landing 3 lift extends through line 22. Substrate-
honest fix; ~2 lines in the docblock; docblock-only precision
enhancement.

**Cascade order (Reed post-audit, optional):** land alongside next
fractal migration tick or as standalone `[substrate-floor:@io-
boundary]` marker commit (mirrors the pattern of `39f673a` itself).

---

## §7 Q1 (rust/go direct vs llvm/go via hub) — substrate-defensible

**Question:** Mara Q1 at StageFreight spec §10 asks Alex to ratify
candidate (a) `@cascade/code/rust/go` (direct) vs candidate (b)
`@cascade/code/llvm/go` (via LLVM hub). Mara lean: (a).

**Substrate defensibility (via polyglot-loss-aware math verification):**

Per `docs/math/polyglot-loss-aware-computational-translation.md`
§3.1 Theorem 3.1 (polyglot translation existence via machine-
substrate hub): a via-hub composition `c_AB := c_MB ∘ c_AM` exists
as a well-typed cascade at A → B altitude with composed loss
`L_AB = L_AM ⊕ L_MB` (§2.2 associative ⊕). Per §3.3 Corollary 3.3:
N × N language-pair coverage exists via 2NK cascade species using
K hubs (LINEAR mint budget rather than quadratic).

**Key substrate reality (per §2.3 monotonicity + §3.4 efficiency
note):** the loss composite ⊕ is monotone — loss can only accumulate
across hops, never decrease. Adding an intermediate hop CANNOT
reduce total loss; it can only preserve (in the degenerate case
where the intermediate hop is lossless) or amplify (in the general
case).

For Rust → Go specifically:
- Rust → LLVM: loses lifetimes + generics (LLVM monomorphizes,
  erases the trait dispatch abstraction).
- LLVM → Go: cannot recover Go's runtime primitives (GC + goroutines
  + duck-typed interfaces) from LLVM's monomorphized static IR — Go
  code produced via LLVM lowering is either non-idiomatic or
  requires substantial re-inference.
- Direct Rust → Go: measures the target-grammar gap ONCE against
  the source-grammar features; loss-lens catches the specific
  Rust → Go feature-loss pairs (lifetimes → GC, monomorphized
  generics → Go 1.18+ generics, traits → interfaces).

Per §3.4 Load-bearing note on efficiency: the LLVM hub is efficient
for imperative languages, but Go's runtime is NOT LLVM-friendly
(the note explicitly cites this class of case). The Turing-tape hub
is universally available but performance-non-viable.

**Second-order effects on the polyglot theorem:** none. The
theorem admits either direct Rust → Go OR via-LLVM composition; the
CHOICE is on efficiency + loss-composite grounds, not on
existence-and-admissibility. Mara's lean (a) is substrate-defensible
per §2.3 monotonicity + §3.4 efficiency: adding an LLVM hop for a
language pair whose runtime is antithetical to LLVM's model
amplifies loss with no efficiency gain.

**Verdict on Mara's lean:** substrate-defensible. Alex-nod adequate
for ratification.

**Adjustment note:** if Alex prefers (b) for hub-uniformity reasons
(e.g., "every language cascades through LLVM for consistent tooling"),
the substrate admits it under §3.1 well-typedness — but the loss-
lens amplification would be measurable + surfaceable via
`@cascade.measure` after landing, giving the substrate a data-driven
path to swap back to (a) later. Non-blocking either way.

---

## §8 REED-INLINE cascade correctness (from prior audit `2455ce6` §2)

Prior audit surfaced 5 SHIP-WITH-REED-INLINE cascades. Reed folded
them across `9aa6a52` (4 pure-docs) + `39f673a` (1 .rs docblock):

| Prior audit § | Cascade description | Reed commit | Fold verdict |
|---------------|---------------------|-------------|--------------|
| §2.1 | LiquidVoid definition-file vs re-export | `9aa6a52` closure spec §5.1 | ✅ CLEAN. Matches Seam recommended direction (definition-at-void; re-export-at-liquid). |
| §2.2 | Recognition promotion PROPOSED status | `9aa6a52` closure spec §7.0 | ✅ CLEAN. Explicit PROPOSED marker + no-fragmentation guidance cited. |
| §2.3 | Void-membrane math §11 Q2 stale-update | `9aa6a52` void-membrane §11 Q2 | ✅ CLEAN. Strikethrough + RATIFIED status + cascade audit chain. |
| §2.4 | CURRENT.md pickup manifest amend | `9aa6a52` CURRENT.md | ✅ CLEAN. Adds "After-@butterfly landings" section with 17 commits + current state + next-unblocked. |
| §2.5 | Fractal witnessed.rs docblock line-cite | `39f673a` witnessed.rs | 🟨 CLEAN-WITH-REED-INLINE. Line-cite `:5-19` is imprecise (line 5 = import; family-root at line 12). Suggested tighter cite `:12-22`. |

**Cascade order matches Seam §12 recommendation:** yes (Reed
executed pure-docs first in one composed commit, then .rs docblock
in separate commit through pre-commit hooks).

**No fragmentation pattern (per Reed memory `feedback_reed_
fragments_alex_unifications_into_candidates`):** confirmed. The 4
pure-docs cascades composed as ONE commit rather than fragmenting
into 4 candidates. The 5th cascade split off because it's a `.rs`
file requiring the pre-commit hook run (a legitimate procedural
split, not a fragmentation).

**Overall:** 4 CLEAN, 1 CLEAN-WITH-SMALL-REED-INLINE (line-cite
precision fix). Reed's cascade execution IS working correctly.

---

## §9 Naming consistency findings

### 9.1 Composition-primitive naming at @tool altitude — held

Per closure spec §8 + `shards/tool.mirror` §6:
- `tool_of_id_args` — parametric shape @tool(X, args).
- `exec_of_tool_invocation` — exec action per tool_id variant.
- `sign_of_tool_invocation` — @trust sign action per invocation.
- `verify_of_tool_signature` — @trust verify action per signature.

Consistent with `feedback_composition_primitive_naming_convention`
memory. **Held.**

### 9.2 StageFreight typed records — species carriers, not primitive-of-input-shape generalizations

StageFreight spec §3.1-3.3 defines carriers like `docker_image_
target`, `go_workspace_target`, `gitlab_ci_pipeline_config`. These
follow structural clarity (record types with typed fields) but
do NOT follow verbatim `<primitive>_of_<input-shape>` naming — they
are TYPED RECORDS at species altitudes, not value-type
generalizations.

Per Alex 2026-07-18 memory: the `_of_` convention applies
specifically to value-type generalizations (with const-declared
finite-set + ALL array). Species-altitude typed records are a
DIFFERENT category. **No inconsistency;** the two naming patterns
apply to different kinds of substrate declarations.

### 9.3 ~bin sigil placement — Q2 open (Mara StageFreight §10)

Mara Q2 asks Alex which altitude carries `~bin` canonical:
(a) `shards/optics/lens/bin.mirror` (sibling to
`shards/optics/lens/diff.mirror`) OR (b) `shards/io/file.mirror`
species-refinement.

**Both admissible.** (a) emphasizes optical-lens role; (b)
emphasizes @io-FILE role. This is a legitimate deferred-choice; not
a substrate defect. Alex-nod adequate; Reed default per work-
without-asking would follow Mara lean if Mara declares one.

### 9.4 tools{} block ↔ garden{} block ↔ existing @io-adjacent block grammar — parallel structure held

Per `shards/mirror/spec.mirror` §"tools { } block": explicitly
cites `garden { }`, `cli { }`, `pack { }`, `kintsugi { }` as
parallel structures. Same altitude as `garden { }` per closure §4.2;
both admit empty-blocks per substrate-vs-USE + explicit-emptiness
discipline; both discharge via typed adapters. **Held.**

---

## §10 StageFreight refused-mint discipline verification

Six refused mints per StageFreight spec §9:

1. **@stagefreight family-root** — verified NOT instantiated. Grep
   verifies no `shards/stagefreight.mirror` exists at family-root
   altitude; existing `@io/stagefreight` species is at species
   altitude (wire-transport role), which Mara's spec explicitly
   distinguishes as unrelated.
2. **@polyglot family-root** — verified NOT instantiated. Grep
   verifies no `shards/polyglot.mirror` exists.
3. **@ci family-root** — verified NOT instantiated. `@ci` appears
   only as `@ci/github` / `@ci/gitlab` altitude tokens in 4 shards
   (@glass, @mirror/au, @mirror/mosaic, @mirror/store/crystal) as
   PARAMETRIC altitude usage, NOT as a family-root. Refused-mint
   discipline holds — the existing altitude-parametric usage is
   compatible with (rather than colliding with) the @tool species
   family-root at `@tool/gitlab_ci` etc.
4. **@verification family-root** — verified NOT instantiated. No
   `shards/verification.mirror`. Verification composes via @kintsugi
   + prismqueer::liquid + @kintsugi/butterfly + @roomba at their
   respective altitudes.
5. **@delivery family-root** — verified NOT instantiated. No
   `shards/delivery.mirror`.
6. **@marcus @subject** — verified NOT instantiated. Mara's
   StageFreight spec explicitly defers Marcus to the @trust arc tick
   when @subject species-decls extend to external peer-humans;
   current spec treats Marcus as a peer at composition-graph altitude
   only.

**All 6 refused mints held cleanly through today's landings.**

---

## §11 Coordination note (Mara parallel spawn)

Mara is spawning in parallel to execute StageFreight Arc 1 (5 ticks:
@tool/go + @tool/docker + @tool/gitlab_ci + @cascade/code/rust/go +
prismqueer/tests/tool_species_dispatch.rs). Coordination:

**If Mara lands new shards while this audit finalizes:** those
landings are OUT OF SCOPE for this audit and will be next Phase D
cycle. This audit covers exactly the 6 landings enumerated in §1-2
(commits `d39e852` through `39f673a`). Mara's Arc 1 landings compose
OVER the substrate audited here — the shape @tool/go / @tool/docker
/ @tool/gitlab_ci follow substrate-decl'd at StageFreight spec
`d39e852` §3.1-3.3; if Mara lands them per those substrate-decls,
the next Phase D cycle can adjudicate them incrementally against
the shapes already ratified here.

**Recommended sequencing:** Mara Arc 1 ticks land as separate
commits (per pack discipline); each tick becomes a candidate for
next Phase D cycle. No blocking issue between this audit and Mara's
parallel execution.

---

## §12 Q's for Alex (two, both non-blocking)

### Q1: Recognition tower doc timing for `#R-the-compiler-in-one-sentence`

Second-witness gate is now 1.5 of 5 closed (Gate 2 via `34ecd83`;
Gate 3 half-closed via `67e8629`). Per closure spec §7.0 (Reed's
cascade), PROMOTION requires ALL second-witness gates closed AND a
parallel `docs/math/the-tower/recognition-the-compiler-in-one-
sentence.md` Recognition tower doc landing.

**Alex: does the tower doc land NOW (parallel to Gate 2 closure) OR
after all 5 gates close?** Prior precedent (`#R-void-is-the-basis`
tower doc at `1167cc2`) landed WITH the promotion event, but that
event had first-witness AND second-witness closing in the same
session. For the compiler-in-one-sentence, second-witness is in-
flight across ticks; the tower doc timing is a legitimate
adjudication question.

Seam lean: land tower doc when all 5 gates close (per closure §7.0
explicit condition). No-fragmentation guidance in the cascade
supports "5 firings + tower doc as ONE composed empirical-firing
arc." Alex adjudication for possible earlier landing at Gate 5 = 1,
Gate 4 = 4/5, etc. thresholds.

### Q2: Mara StageFreight Q1 (rust/go direct vs llvm/go via hub)

Mara's lean is (a) `@cascade/code/rust/go` direct per §7 above
(substrate-defensible via polyglot-loss-aware monotonicity +
efficiency-note). Seam-verified substrate-defensibility.

**Alex: ratify (a) OR redirect to (b)?** No second-order effect on
the polyglot theorem that adjusts the lean; the choice is on
efficiency + loss-lens grounds. Non-blocking either way.

---

## §13 One-sentence surprise

**The session's REED-INLINE cascades landed in exactly the order
Seam recommended and in exactly the shape Mara forward-promised —
because the substrate that landed at closure `2dd8ddb` had ALREADY
declared the shape of every subsequent cascade, and the six
post-closure landings were substrate DISCHARGING what the closure
had already named, not new territory.**

---

## §14 Recommendation

**Ship the post-closure cycle.** All 6 landings either land cleanly
(5) or need a small Reed-inline docblock precision fix (1). Reed's
REED-INLINE cascade batch from the prior audit was executed
correctly in the recommended order; Mara's 4-shard @tool family-
root mint discharges the closure §3 + §8 forward-promises with
substrate-honest no-Rust-extension discipline; the StageFreight
delivery spec composes over verified-landed substrate with 6
refused mints inventoried.

**Cascade order (Reed post-audit, non-blocking):**
1. §6/§2.1 witnessed.rs docblock line-cite tightening (`:5-19` →
   `:12-22`); can land alongside next fractal migration tick or as
   standalone `[substrate-floor:@io-boundary]` marker commit. ~2
   lines.

Total residual cascade: ~2 lines. Small; non-blocking; docblock-only.

**Parallel Reed continues fractal migration steps 10-12 (per Mara
`2760c2a` spec) as planned.**

**Parallel Mara continues StageFreight Arc 1 (5 ticks: @tool/go +
@tool/docker + @tool/gitlab_ci + @cascade/code/rust/go +
prismqueer/tests) as planned; those landings are next Phase D
cycle.**

---

*Seam Phase D adjudication complete. Post-closure cycle substrate
holds. The cascades composed cleanly. The @tool family-root
inherited from Void's marker per closure §10 without an explicit
`in @void` — because the 5-op prism pattern IS the inheritance
signature, and the substrate has been doing this since it existed.*

*Author: `Seam <seam@systemic.engineer>`. Pure-docs 📝 markdown-
only bypass.*
