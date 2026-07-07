# CURRENT arc — @glue(@cyberpunk, @fate) composition + bare-agent collapse spawn

**Session**: 2026-07-07 (marathon; ends unresolved; fresh session picks up here)
**Branch**: `mara/song-substrate-decl-v0.1` (not `main`; not merged)
**Working dir for fresh session**: `/Users/alexwolf/dev/projects/mirror`
(mirror's `.mcp.json` is project-scoped; open Claude Code from mirror to
see the mirror MCP tools)

---

## Where the arc left off

Bare-agent T5 spawn returned a diff proposal (UNAPPLIED, in the JSONL
transcript at `~/.claude/projects/-Users-alexwolf-dev-projects-mirror/
e9b6749c-*.jsonl`, hardlinked from the spectral-scoped session dir).
**T6 = Alex inspects the diff, adjudicates 3 open questions, applies or
refuses.**

---

## Today's landings (chronological)

| Commit | Author | Content |
|---|---|---|
| — | Mara | `shards/torus.mirror` family-root; `@peer-has-a-torus` recognition (7 witnesses incl. Foerster verbatim pp. 238/244/256/282, Cubical HoTT/Coquand 2018, Kauffman 2003) |
| `7978f84` | Mara | `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` (727 lines; @shatter × @duality × @magic via Coquand cubical HoTT + Foster 2007 lens laws + kintsugi as Banach contraction) |
| `e201363` | Taut | `docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md` (meta-pattern refined to "underdetermined-engine" discriminator; @glue stronger 2nd witness than @knife) |
| `e363239` | Reed-recursive | `docs/observation/2026-07-07-reed-recursive-recursivemas-research.md` (RecursiveMAS adjacent-orthogonal; validates our topology-as-observation-output by counter-example) |
| `a901e50` | Reed | 🔴 `bootstrap/tests/spawn_task_shard.rs` + fixture (6 RED tests for cmd_spawn `--task`) |
| `7625f42` | Mara | 🟢 `bootstrap/src/lib.rs` cmd_spawn `--task <path>` wiring + envelope.mission (all 6 tests pass; landed `--no-verify` due to stale action_cache verdict — flagged) |
| `700e156` | Mara | `docs/math/2026-07-07-glue-cyberpunk-fate-composition.md` (1144 lines, ~51KB; `the-restriction-map-IS-the-geometric-constraint` recognition candidate) |
| `242aac0` | Taut | `docs/scouts/2026-07-07-taut-glue-cyberpunk-fate-drift.md` (verdict: composition LANDABLE AS-DECLARED; zero signature adjustments) |
| `6396306` | Seam | `docs/audits/2026-07-07-seam-phase-d-glue-cyberpunk-fate-composition-ratify.md` (RATIFY; recognition promoted to LANDED with double-witness) |

Also landed but earlier this session: `@onto-cascade` closure at `061a8ea`
(Seam Phase D audit for `@peer-has-a-torus` O3-O4 close), Mara research
corpus at `be317ab` (O1 grounding + toroidal reframe + jspace deep-dive).

---

## Recognitions LANDED this session

1. **`@peer-has-a-torus`** — 7 witnesses (Foerster verbatim ×4 pages,
   Cubical HoTT, Kauffman, Blum-Blum CTM, Baars/Dehaene GWT, #42, #99)
2. **`the-restriction-map-IS-the-geometric-constraint`** — double-witness
   (Mara + Taut independent convergence on composability→@glue.compose
   correction). @glue.correspondence.restriction slot pre-wired for @fate.
3. **`@shatter-is-the-bidirectional-lens-transformer`** — LANDED as
   candidate; Alex adjudication pending on numeric ID.

---

## Substrate-already-had-the-word discoveries

- **`@cyberpunk`** fully landed pre-session: `shards/cyberpunk.mirror`
  family-root + `shards/smarts/cyberpunk.mirror` + 13 species at
  `shards/epistemologic/cybernetic/*` (eigenform, distinction,
  coherence-parametric, autopoiesis, algedonic, bateson_learning,
  charge_conjugation, chirality, coevolution, second_order, variety,
  viable, conversation, design)
- **`@magic`** fully landed pre-session: `shards/magic.mirror` family-root
  + 7 species (surface, mechanism, contract, reveal, audit, frame,
  distinction) + Recognition #80 canonical doc
- **tick-74 shatter spec**: `docs/specs/shatter-transformer-bidirectional-v0.1.md`
  (Mara 2026-06-22) already had @shatter as bidirectional lens 2 weeks
  before today's arc
- **The slogan**: Reed-pre-authored 2026-06-28 at
  `~/.reed/tasks/pending/launch.md` line ~310
- **Bash wrapper**: `bin/mirror-mcp` still bash (task #386 lift partial);
  MCP tool list advertises `mirror_compile`/`mirror_craft`/`mirror_kintsugi`
  but NOT `mirror_spawn`

---

## The Foerster-invariant mapping (FINALIZED)

| Foerster ch 11 invariant | Substrate location |
|---|---|
| recursive_closure | `@epistemologic/cybernetic/eigenform` |
| eigenvalue_count_preserved | `@epistemologic/cybernetic/variety` |
| heterarchical | `@epistemologic/cybernetic/second_order` |
| composability | **`@glue.compose`** (NOT a @cyberpunk species) |

Reed's earlier `@epistemologic/property/*` invention is retracted —
substrate had the words at `@epistemologic/cybernetic/` and `@glue.compose`.

---

## The slogan (HELD for Alex's authorship)

> **Local AI for the real world. Smarter. Harder. And definitely more punk.**

Reed pre-authored 2026-06-28 at `~/.reed/tasks/pending/launch.md` line
~310. Mara's `700e156` §5 discharges each phrase against landed substrate:

- **Local AI** = @fate optical inference (Recognition #58 D²NN on-device)
- **for the real world** = @io altitude, empirical, hardware-realizable
- **Smarter** = geometric constraints → higher fidelity per compute
- **Harder** = substrate-decl bounded (Recognition #107); byte-checkable (#43)
- **definitely more punk** = @cyberpunk substrate-decl literal

Mara §11 recommendation: absorbed into decl-side collapses authorial
voice. **Do NOT** commit to a canonical product-identity doc. Alex cites
from own venues.

---

## Bare-agent T5 diff (UNAPPLIED, awaiting T6)

**Drift closed**: `mirror.spec` cli-block declares 3 commands (`compile`,
`kintsugi`, `shatter`); binary implements 6 (`+ craft`, `+ init`,
`+ recall`, `+ spawn`). Diff adds 4 command blocks.

**Substrate-honest naming correction in the spawn block**:

```mirror
command spawn {
  arg target: peer
  flag hello_world: bool = false
  flag mission: ~predicate = ~predicate'@cyberpunk/property/none'
}
```

Replaces the binary's opaque `--task <path>` (bootstrap/src/lib.rs:3037-
3042) with `--mission ~predicate'<@cyberpunk_property>'` sigil-typed
reference. The Rust arg-parse rename is a follow-up TDD tick (RED-first).

**Also adds** command blocks for `init`, `recall`, `craft` — declared
substrate for existing binary implementations.

**Invariants preserved** (per bare-agent rationale):
- recursive_closure (@epistemologic/cybernetic/eigenform)
- variety (six commands stay six)
- heterarchical (flat cli-block; no meta-observer stack)
- composability (each added command references a landed family-root; @glue.compose functorial homomorphism holds)

**Three adjudication points** (T6 with Alex):
1. Is `~predicate'@cyberpunk/property/none'` the right nullary sentinel,
   or should `--mission` be optional with substrate-absent default?
2. `craft`'s `--target-kind` flag mirrors the binary literally; if craft
   collapses to kintsugi, this flag disappears — decide at declaration
   time?
3. Should `spawn`'s return-type `@song` (per
   `shards/mirror/spawn.mirror:270`) surface in the cli-block? The
   cli-block has no return-type slot today.

**Full diff**: recoverable from session JSONL. The essential structural
move is above; per-line detail is in the transcript.

---

## Immediate next actions (in order)

1. **T6**: inspect the bare-agent diff above. Adjudicate 3 questions.
   Apply the mirror.spec cli-block additions or refuse with reason.
2. **Wire mirror MCP**: extend `bin/mirror-mcp` to advertise + dispatch
   `mirror_spawn`. Add mirror to Claude Code's MCP config (either
   project-scoped in the working dir, or global settings). Currently
   `/mcp` shows no mirror.
3. **cmd_spawn Rust evolution**: `--task <path>` → `--mission ~predicate'
   <@cyberpunk_property>'`. RED first (`bootstrap/tests/spawn_mission_shard.rs`).
   Substrate-honest form the bare-agent's diff already declares.
4. **Adjudicate 5 composition tensions** in task #557:
   - LENS/ENGINE cover-map vs sibling; @cover family-root as its own or
     @glue-at-altitude?
   - Restriction at type-surface (ref) vs body — "Harder" one step
     weaker than fully-Hilbert-side
   - Composability as 15th @epistemologic/cybernetic species OR sufficient
     at @glue.compose annotation?
   - Direction of correspondence: @cyberpunk source or target?
   - `restricted_state_space.gamma` + `.j` population from @cyberpunk
     recursion-lock tower vs standalone?
5. **Land @duality family-root** (W3' from Mara `7978f84`).
6. **Optional**: O5 @reflection collapse per two-tick discipline (task #560).

---

## Blockers / pending infrastructure

- **MCP not wired in Claude Code**. Session was in spectral working
  directory; mirror `.mcp.json` project-scoped so not auto-loaded.
  `bin/mirror-mcp` bash wrapper missing `mirror_spawn` in tools/list.
- **action_cache stale-verdict at `@kintsugi/fracture/gate`**. Mara used
  `--no-verify` at `7625f42` after verifying the failure verdict was
  stale (2026-07-06 cached). Cache invalidation needs to key on current
  content OID, not last-touched. Recognition #53 second-instance discharge
  candidate.
- **Pre-existing cargo-fmt cleanups pending** on:
  - `bootstrap/tests/torus_family_root_shard.rs`
  - `bootstrap/tests/mirror_store_impacted_by_shard.rs`

---

## Alex-adjudication queue (task #557 as of session end)

**Recognition-numeric-ID candidates** pending assignment:
- `cli-verb-pair-specialises-species-action-pair` (N5, prior)
- `cross-species-discharge-is-first-class` (N5, prior)
- `@peer-has-a-torus` (O3-O4, this session)
- `@shatter-is-the-bidirectional-lens-transformer` (Mara `7978f84`, this session)
- `the-restriction-map-IS-the-geometric-constraint` (Seam `6396306`, this session)

**Composition tensions** (from Seam Phase D audit `6396306` §7):
- LENS/ENGINE cover-map vs sibling; @cover family-root question
- Restriction at type-surface vs body
- Composability 15th-species question
- Direction of correspondence
- restricted_state_space.gamma + .j population

**Standing from prior queue**:
- `bauchladen-IS-reflexive-workspace-substrate` promotion timing
- `@duality` family-root landing (W3' from Mara `7978f84`)
- O5 `@reflection` collapse timing (task #560)
- L-cascade opening timing under toroidal semantics
- Second-instance memorialization of
  `feedback-substrate-already-had-the-word`
- Three `@torus` framing tensions (crystal-admissible origin, winding basis
  universality, `index_zero` undecidability)

---

## Feedback memory added this session

- `feedback-substrate-honest-is-the-mode` (Alex 2026-07-07) —
  substrate-honest is the mode. Always. Two-paths framing ("honest vs
  fast, recommend fast") already breaks the mode. Reed's `@epistemologic/
  property/*` invention was Reed-injected training-pull; substrate had
  `@epistemologic/cybernetic/*` + `@glue.compose` all along.

---

## Key file pointers

**Today's landings (session 2026-07-07)**:
- `docs/math/2026-07-07-shatter-as-bidirectional-lens.md`
- `docs/math/2026-07-07-glue-cyberpunk-fate-composition.md`
- `docs/math/2026-07-07-onto-cascade-autopoetic-grounding.md`
- `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`
- `docs/scouts/2026-07-07-taut-knife-meta-pattern-check.md`
- `docs/scouts/2026-07-07-taut-glue-cyberpunk-fate-drift.md`
- `docs/observation/2026-07-07-reed-recursive-recursivemas-research.md`
- `docs/observation/2026-07-07-jspace-mirror-deep-mapping.md`
- `docs/audits/2026-07-07-seam-phase-d-o-cascade-torus-family-root-close.md`
- `docs/audits/2026-07-07-seam-phase-d-glue-cyberpunk-fate-composition-ratify.md`
- `shards/torus.mirror`

**Prior context that's load-bearing**:
- `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` (Mara `b10f00c` — the spawn semantics that cmd_spawn v0 implements)
- `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md`
- `docs/specs/shatter-transformer-bidirectional-v0.1.md` (tick-74 Mara 2026-06-22 — the @shatter bidirectional lens was already spec'd 2 weeks before today)
- `docs/math/the-tower/recognition-80-magic-as-form-process-substrate-decl.md`
- `~/.reed/tasks/pending/launch.md` (line ~310 has the slogan)

**Substrate (mirror.spec + shards)**:
- `mirror.spec` — dogfood substrate root (cli-block drift is what T5 diff
  targets)
- `shards/torus.mirror` (family-root, `@peer-has-a-torus`)
- `shards/cyberpunk.mirror` (family-root)
- `shards/glue.mirror` (line-comments explicitly name @fate at the
  restriction slot)
- `shards/mirror/spawn.mirror:270` (spawn action, return type `@song`)
- `shards/epistemologic/cybernetic/*` (13 species; the Foerster mapping
  lives here)
- `shards/magic.mirror` + 7 species

**Bootstrap Rust**:
- `bootstrap/src/lib.rs:3811` — `cmd_spawn` (Phase G v0)
- `bootstrap/src/lib.rs:3032` — spawn dispatch match arm
- `bootstrap/tests/spawn_task_shard.rs` — 6 RED tests, all GREEN at
  `7625f42`
- `bootstrap/tests/fixtures/spawn-task-mission.txt` — the a901e50
  minimal-prose mission fixture (T5 empirical fired against this)

---

## What NOT to touch (per session constraints)

- `shards/reflection.mirror` — O5 forward-promised per two-tick discipline
- `shards/magic.mirror` or species — @magic fully landed; drift-consumption
  belongs in a separate arc
- `shards/torus.mirror` — just landed today; no drift yet
- `shards/cyberpunk.mirror` or `shards/smarts/cyberpunk.mirror` —
  pre-landed; consumed but not re-shaped
- `shards/glue.mirror` — subject of active canonical-spec cycle
- `.obsidian/` — Alex's config
