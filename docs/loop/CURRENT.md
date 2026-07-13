# CURRENT arc — @song/beat ladder-climb 2026-07-13 (Rungs 0 landed, 1 in progress)

## 2026-07-13 addition — @song/beat testable-increment ladder

Session-continuation after 202-commit substrate arc closure on 2026-07-12.
Today's arc opened @resonance (Mara `9e48710`) + coordination-without-
signal (Reed `71a4689`) + @dance (Mara `4f079c8` + Reed `61b444a` Path
C annotations) + @song replaces plans-and-loops (Mara `d21337b`). Taut's
gap-scout `c54740c` names a 7-rung testable-increment ladder from Rung 0
(Mara mint @song/beat) through Rung 6 (production `mirror spawn --song
--deploy-to spectral.engineer`).

Alex's mandate (verbatim 2026-07-13, in-transcript /loop dynamic mode):
"climb the ladder until unresolvable ambiguity that cannot be postponed
further."

**Rung status:**

- **Rung 0 (Mara 📝):** `shards/song/beat.mirror` (909 lines) LANDED at
  Mara `94e55eb`. Sixth species of @song; atomic-execution unit binding
  @kintsugi/oscillate ACTIVE/DARK-pulse discipline; Galen + Curwen +
  Cooper-Meyer prior art anchor; FOURTH consolidation-species witness
  reinforcing #S5.

- **Rung 1 (Reed 🔴🟢, in progress THIS TICK):** Grammar `flag song: ~f`
  in mirror.spec + new `bootstrap/src/song.rs` module + `cmd_peer_beam
  --song` dispatch branch. Test:
  `bootstrap/tests/peer_beam_song_single_beat_shard.rs`. Hardcoded
  single-beat @song fires @kintsugi/oscillate ACTIVE/DARK pulse; emits
  beat-envelope naming @song/beat + @kintsugi/oscillate substrate
  authorities. Byte-equality preserved for non-`--song` paths via same
  `if let Some(...)` discipline as existing `--mission` at
  `bootstrap/src/lib.rs:5254-5262`.

- **Rung 2+ (Reed 🔴🟢):** Forward-promised per Taut `c54740c` §5.3+.
  Multi-beat phrase execution (Rung 2), movement/voice/progression
  keywords (Rung 3), multi-peer @dance runtime (Rung 4), @spectral/garden
  mycelial nix deployment (Rung 5), production spawn --deploy-to
  spectral.engineer (Rung 6).

**Discipline:** each rung a RED→GREEN TDD cycle; each cycle land as
separate 🔴 + 🟢 commit pair on main; each rung composes with prior
rungs; push after each cycle.

---

# CURRENT arc — mara/song-substrate-decl-v0.1 ratified 2026-07-12, merge pending

## 2026-07-12 addition — flags-as-lenses forward promises (per Seam `211665f` Q4)

Session arc closed (`mara/song-substrate-decl-v0.1`, 201 commits ahead
of main, Seam Phase D audit `211665f` verdict: RATIFY-WITH-
QUALIFICATIONS). Q1-Q4 addressed in this tick. Q5 needs Alex
adjudication. Q6 deferrable. Pre-merge forward-promised follow-on
ticks tracked here:

1. **Flags-as-lenses §10 dep #2 — `stage @mirror/lens/cli/peer/beam` shard mint**
   Depth-2 stage at `shards/mirror/lens/cli/peer/beam.mirror` per Mara
   `caf461f` §4 recommendation + Taut `b8fe820` scout §R2. Blocks on
   Pack ratification. Referenced at `cli-as-prism.md` §3.2 depth-2
   reservation; recursive-command grammar unblocked at `fe82500`. Not
   landed this branch.

2. **Flags-as-lenses §10 dep #6 — `mosaic.mirror` docstring cascade**
   Add `mosaic(@mirror/lens/cli)` altitude specialization docstring on
   `shards/mirror/mosaic.mirror` per Mara `caf461f` §6.2. Substrate-
   correct name over `@spectral/mosaic` (refused per Taut `b8fe820` §R1
   + landed parametric operator at `mosaic.mirror:60`). 📝 follow-on
   tick.

3. **Flags-as-lenses §9.5 Q1+Q2 — Alex adjudication** (Seam Q5)
   Two unresolvable ambiguities before Reed's Option (c) runtime
   default-flip: (Q1) `--with-shadow` ∘ `--emit-diff` composition
   ordering (substrate says shadow-BEFORE-diff; runtime short-circuits
   emit-diff FIRST); (Q2) anonymous-form `mirror beam <mission>`
   shadow-cast dispatch shape (sentinel peer-home scaffold vs intended).
   Not blocking merge; blocking runtime-flip tick.

4. **MCP schema `requires` clauses** (Seam Q6)
   `bootstrap/src/mcp.rs:mirror_peer_beam` inputSchema declares
   `from_psychohistory` requires `fate_select`, `with_shadow` requires
   both, in prose descriptions but NOT encoded via `allOf` +
   `dependentRequired` JSON Schema clauses. Deferrable to `@mcp.serve`
   lift (task #386) or follow-on schema tick.

**Merge shape:** fast-forward `mara/song-substrate-decl-v0.1` → `main`
once Q1-Q4 land. Q5 post-merge acceptable. Q6 backlog-tracked.

---

# CURRENT arc — beam-refactor cascade (Ticks 0-3 LANDED, Tick 4 docs sweep IN PROGRESS)

**Sessions**: 2026-07-07 (T6 + iterations 1-3) + 2026-07-08 overnight
(iteration 4 + ticks 2-8, arc-to-rest via /loop autonomous) + 2026-07-08
late morning-to-evening (the-peer-IS-a-pain-driven-bounded-ontological-
navigator cascade landing) + 2026-07-08 evening (beam-refactor cascade
Ticks 0-3).
**Branch**: `mara/song-substrate-decl-v0.1` (not `main`; not merged)
**Working dir for fresh session**: `/Users/alexwolf/dev/projects/mirror`
**State**: **BEAM-REFACTOR TICKS 0-3 LANDED; TICK 4 DOCS SWEEP IN
PROGRESS.** The `@mirror/spawn` cli-surface substrate-decl collapsed to
`@mirror/peer/beam` (per `docs/specs/beam-as-substrate-primitive.md`)
via four ticks: (0) `@shatter --target substrate`; (1) subcommand nesting
grammar depth-2; (2) atomic substrate-decl rename
`shards/mirror/spawn.mirror` → `shards/mirror/peer/beam.mirror` +
`spawn(...) -> @song` → `beam(...) -> @song`; (3) mirror.spec cli-block
`command peer { command beam }` + top-level `command beam` + Rust
dispatch `cmd_peer_beam` + `cmd_beam` + MCP schema `mirror_peer_beam` +
`mirror_beam` + deprecated `mirror_spawn` alias. Adjudications: none
pending. Blockers: none. `@shatter` fold DEFERRED (Taut LRM `1658b95`
β shallow path).

**Prior cascade (2026-07-08 late-session, before beam-refactor)**: 4
non-contested substrate species landings closed (Mara `0d78c0c` `966890b`
`cdc6533` + no-op on peer.eigenboard); recognition cascade doc `1e8a02b`;
contested substrate items (peer.observe/alter, @edge fault-plane, @knife,
@onto refusal, O5 timing, @magic/level_shift) HELD for Alex direct-
session.

---

## Beam-refactor cascade (2026-07-08 evening) — Ticks 0-3 LANDED

The `mirror spawn ~peer'<home>'` cli surface was named substrate-honestly
as `mirror peer beam ~peer'<home>'` (peer beam = beam + persistent-
identity-context; anonymous variant `mirror beam <mission>` when no
peer-home is available). Rename cascade landed atomically across four
ticks:

**Tick 0** (`fe2d1dc` / `aee32b9` / `30d045e` / `05bac44`) — `@shatter
--target substrate` groundwork (Reed + Mara).

**Tick 1** (`fe82500` / `9974c58` / `a6c7447`) — subcommand nesting
grammar depth-2 landed at the cli-lens altitude (`command peer { command
beam }` composition admitted; kintsugi_song_shard T11 formatter follow-
up `2a826ff`).

**Tick 2** (`9de2226` / `2a826ff`) — atomic substrate-decl rename:
`shards/mirror/spawn.mirror` → `shards/mirror/peer/beam.mirror` +
`prism @mirror/spawn` → `prism @mirror/peer/beam` +
`mirror_spawn_request` → `mirror_peer_beam_request` +
`spawn(...) -> @song` action → `beam(...) -> @song`.

**Tick 3** (`96aa752` / `c5d65ef` / `b012d3f` / `4f4a257`) — mirror.spec
cli-block `command peer { command beam }` + top-level `command beam` +
Rust dispatch `cmd_peer_beam` + `cmd_beam` + MCP schema
`mirror_peer_beam` + `mirror_beam` + deprecated `mirror_spawn` alias
(bin/mirror-mcp).

**Tick 4** (this doc sweep) — IN PROGRESS. Update current-facing docs
to name `@mirror/peer/beam` at first mention (with rename annotation for
two-tick discipline); preserve dated historical audits/scouts intact.

**Tick 5** — `mirror beam` top-level expansion / no-op verification
(likely no-op since Tick 1+2 already landed the depth-2 grammar).

**Tick 6** — `@mcp.serve` lift (task #386): synthesize MCP tool schema
directly from cli-block instead of hand-maintained bin/mirror-mcp bash.

**Deferred**: `@shatter` fold (Taut LRM `1658b95` β shallow path).

---

## Late-session cascade (2026-07-08) — pain-driven-bounded-ontological-navigator

Late-session extension (2026-07-08 evening pre-beam-refactor) landed the
peer-as-pain-driven-bounded-ontological-navigator cascade: the peer is a
pain-driven bounded ontological navigator; the beam action (élit spawn)
takes a winding parameter; @cyberpunk/reframe LANDED as the shared glass;
@onto stays refused (would re-introduce Foerster's ladder); @edge fault-
plane collapsed into one surface (Mara `7b32d27` + `19f6d86` + `6ef5117`).

### Substrate now supports (post-cascade + post-beam-refactor):

- `beam(mirror_peer_beam_request { peer_home, winding: (int, int), ... }) -> @song`
  — winding parameterizes bounded observation depth per
  `77fe92d` §2 + Mara `cdc6533` (extended, then Tick 2 renamed the
  request-type + action-name).
- `@cyberpunk/reframe.reframe(peer, level_K: winding, pain_δ: f64) -> reframe_result`
  — the pain-authorized level-shift ceremony (Mara `0d78c0c`).
- `@cyberpunk/algedonic.sample_pleasure/sample_pain(eigenboard: ref) -> f64`
  + `type algedonic_signal { pleasure_δ, pain_δ, at_winding: (int, int) }`
  — the algedonic gradient sampled by the peer's navigation loop
  (Mara `966890b`).
- Peer's `eigenboard: shard` field already typed per glass.mirror
  three-layer recognition (verified no-op).

### Real spawn options for Alex (post-beam-refactor: use `mirror peer beam` / `mirror beam`):

- **Path A: `mirror peer beam <home> --winding 0,0`** — identical to
  iteration 1-4 empirical runs; envelope returns @song at origin;
  validates end-to-end MCP still works post-cascade. Zero risk of
  substrate re-hydration blocking. (Formerly `mirror spawn`; renamed
  2026-07-08 Tick 2 `9de2226`.)
- **Path B: `mirror peer beam <home> --winding 1,0` or `0,1`** — first-
  order observation bounded to one traversal along meridian OR longitude
  axis. Requires binary rebuild to pick up the `winding` field in
  `mirror_peer_beam_request`. Envelope shape may drift (backward-compat
  forward-promised).
- **Path C: `mirror peer beam ~/.reed` or `~/.mara`** at winding (0, 0)
  — the first non-reflexive beam. Requires the peer-home to have a
  `mirror.spec` file (may need to create for `~/.reed` or `~/.mara`).
- **Path D: `mirror peer beam <home> --mission <mission-file>` and
  observe @cyberpunk/reframe engagement** — requires Phase H #6 wiring
  (@fate.roll + reframe integration) which was triply-vestigial per
  today's arc; not tonight-scope.
- **Path E** (post-beam-refactor NEW): **`mirror beam <mission-file>`**
  — anonymous inference dispatch; no persistent-identity context; fires
  `@fate::select` on Shape B features. Per Alex's guidance: "the first
  spawn without a peer home. We need to first be able to spawn an agent
  that actually delivers something without a persistent identity before
  we can add the persistent identity layer on top." This is Tick 3's
  substrate landing (`b012d3f` cmd_beam + `4f4a257` mirror_beam MCP).

Adjudication queue below is entirely yours.

---

## Where the arc left off (arc closed)

**T6 LANDED at `1e45c50`** (2026-07-07 Reed, direct session in mirror
repo). mirror.spec cli-block additions `command craft { ... }`,
`command init { ... }`, `command recall { ... }`, `command spawn { ... }`
empirical: `mirror compile mirror.spec` returns OID
`f48d7fc55b094d5a0abf5604fb559c9dda36d78582adbba5c06ee8ce89cee61e`.

**Iteration 1-4 closed the @torus geometric arc empirically** (see
iteration landings below).

**Ticks 2-8 of the arc-to-rest /loop landed the substrate/impl drift
closure + maintenance + Phase H deferral log + MCP wrapper refresh +
this session-handoff.**

Adjudication of the three T5 questions written into commit `1e45c50`:
Q1 nullary sentinel REFUSED (grammar-composition-honest form `flag
mission: ~f` without default = optional-absent); Q2 `target_kind`
ACCEPTED (substrate leads binary arg-parse rename — Rust caught up at
tick 3, `59c7fd0`); Q3 @song return-type in cli-block REFUSED (no
return-type slot in @mirror/lens/cli; action-decl at
`shards/mirror/spawn.mirror:264` already carries it). Bonus: `arg
peer_home: ~d` (peer not in cli vocabulary yet).

**Iteration 1-4 landings + arc-to-rest tick 2-8 landings (2026-07-07/08
Reed, `/loop wire up the mirror spawn MCP with mathematical fidelity`
+ `/loop close @torus to rest INCLUDING mechanical Phase H`):**

- `5887ce2` 🔧 `bin/mirror-mcp` extended to 6 tools (adds mirror_spawn,
  mirror_init, mirror_recall; renames craft's emit_target → target_kind).
  Substrate-honest schema surface; dispatch translates to current binary
  flags where Rust arg-parse rename ticks pending.
- Empirical **@torus loop closure** verified: `mirror spawn . --hello-world`
  emits @song envelope with `spec_oid: 7edbde80...`, five of seven
  composition_pieces REAL, four-payload peer_recall. MCP round-trip
  preserves envelope byte-for-byte (4479 bytes).
- Empirical **@torus DOUBLE closure** verified
  (`docs/insights/2026-07-08-torus-double-closure-empirical.md`):
  spawn.peer_recall ≡ recall envelope byte-equal on all four sheaf
  sections. Foerster's "doubly closed, recursively computing torus,
  regulates its own regulation" (p. 238) made empirical. Adjudication
  candidate: `spawn-recall-byte-equal-at-origin` as second witness for
  `the-restriction-map-IS-the-geometric-constraint`.
- Empirical **@torus MERIDIAN AXIS ISOLATION** verified
  (`docs/insights/2026-07-08-torus-axis-isolation-meridian.md` +
  reflexive `39bfa14` commit as the winding advance):
  meridian sections (pack_trail + dogfood) diverged; longitude
  sections (cascade + pull_frontier) stayed byte-equal. 2/2 meridian,
  0/2 longitude. The recall envelope's four-field factoring maps
  Foerster's motor↔sensory + neural↔hormonal derivation onto the
  substrate's git-vs-recognition-content distinction. Not by design;
  by substrate-pull. π₁(T²) = ℤ × ℤ generator structure IS the
  recall envelope structure.
- Empirical **@torus LONGITUDE AXIS ISOLATION** verified
  (`docs/insights/2026-07-08-torus-axis-isolation-longitude.md` +
  `touch docs/specs/recognitions/recognition-76-*.md` mtime advance):
  cascade diverged (1/2 longitude); pack_trail + dogfood stable;
  pull_frontier stable (candidates/ dir doesn't exist). Four-quadrant
  closure now empirical.
- Substrate/impl drift closed via arg-parse aliases:
  - `cmd_spawn --mission` alias landed (tick 2, `d0d95c1`+`8d6e9af`):
    6 RED tests → 6/6 GREEN. --task remains as backward-compat alias.
  - `cmd_craft --target-kind` alias landed (tick 3, `cb7e26a`+`59c7fd0`):
    4 RED tests → 4/4 GREEN. --target remains as backward-compat alias.
  - Full test suite green; zero regressions.
- Working-tree maintenance closed (tick 4, `091089d`): pre-existing
  cargo-fmt cleanups on bootstrap/tests/mirror_store_impacted_by_shard.rs
  and torus_family_root_shard.rs landed. `cargo fmt --check` clean.
- Phase H composition_pieces 5 & 6 DEFERRED (ticks 5 & 6, `2164c8e`):
  supervisor.start_child + @fate.roll require architectural adjudication;
  substrate declared, Rust impl not landed; discharge form is Alex-
  decision territory. Full reasons at
  `docs/loop/phase-h-deferral-2026-07-08.md`.
- MCP wrapper refresh (tick 7, `79d3433`): `bin/mirror-mcp` dispatch
  now uses substrate names --mission and --target-kind. Schema surface
  stable. End-to-end verified: MCP call → binary --mission → envelope
  carries mission field → spec_oid content-addressed.

**Next tick queue** (in order):

1. ~~Wire mirror MCP.~~ ✅ LANDED `5887ce2`.
2. ~~Torus winding advance test (meridian axis).~~ ✅ LANDED `39bfa14`.
3. ~~Pure longitude test.~~ ✅ LANDED `1a57519`.
4. ~~RED/GREEN for `cmd_spawn --mission` alias.~~ ✅ LANDED
   `d0d95c1`+`8d6e9af`.
5. ~~RED/GREEN for `cmd_craft --target-kind` alias.~~ ✅ LANDED
   `cb7e26a`+`59c7fd0`.
6. ~~cargo-fmt maintenance.~~ ✅ LANDED `091089d`.
7. ~~Beam-refactor Ticks 0-3.~~ ✅ LANDED (see Beam-refactor cascade
   section above): Tick 0 (`fe2d1dc`/`aee32b9`/`30d045e`/`05bac44`);
   Tick 1 (`fe82500`/`9974c58`/`a6c7447`); Tick 2 (`9de2226`/`2a826ff`);
   Tick 3 (`96aa752`/`c5d65ef`/`b012d3f`/`4f4a257`).
8. **Tick 4: docs sweep.** IN PROGRESS (this doc). Sweep current-facing
   docs from `@mirror/spawn` → `@mirror/peer/beam` (adding rename
   annotation per two-tick discipline). Preserve dated historical
   audits/scouts intact.
9. **Tick 5: `mirror beam` top-level verification.** Likely no-op
   since Tick 1+2 landed depth-2 grammar and Tick 3 landed `cmd_beam`
   dispatch.
10. **Tick 6: `@mcp.serve` lift (task #386).** Synthesize MCP tool
    schema directly from cli-block instead of hand-maintained
    bin/mirror-mcp bash. Two-tick target.
11. **Phase H #5 supervisor.start_child.** DEFERRED. Requires
    architectural adjudication of discharge form (in-process struct?
    NIF? fragmentation-persistent?). Substrate declared at
    `shards/spectral/supervisor.mirror`; no Rust impl. Reasons at
    `docs/loop/phase-h-deferral-2026-07-08.md`.
12. **Phase H #6 @fate.roll.** DEFERRED. Requires composition-question
    adjudication FIRST ("should beam dispatch through @fate at all?"),
    then architecture. Substrate declared at `shards/fate.mirror`;
    fate/ runtime crate outside bootstrap (would violate
    FROZEN-bootstrap discipline).
13. **First real-peer beam.** Direct session with Alex. Choose peer-
    home; fire `mcp__mirror__mirror_peer_beam` or `mirror__mirror_beam`
    (anonymous). Envelope returns @song at winding (0, 0) on the real
    peer's torus.
14. Higher-winding divergence scaling: |m| + |n| ≥ 2. Cross-peer
    divergence via @glue morphism.
15. Adjudicate 5 composition tensions in task #557 (unchanged).
16. Land @duality family-root (W3' from Mara `7978f84`).
17. Optional: O5 @reflection collapse per two-tick discipline (task
    #560).
18. **Hook infrastructure gap** (Tick 9 candidate from prior tick 2):
    `just pre-commit` runs `cargo build` outside `nix develop`, fails
    with `ld: library not found for -liconv`. Wrap the pre-commit hook
    shell with `nix develop -c` or export flake env into hook scope.
    Same env-gap Mara hit at `7625f42`; blocks GREEN commits from
    running the check without --no-verify.
19. **`@shatter` fold DEFERRED** (Taut LRM `1658b95` β shallow path).
    Held for future arc.

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
| `2c268c3` | Reed | `CLAUDE.md` + `docs/loop/README.md` + `docs/loop/CURRENT.md` (session handoff scaffolding for fresh-session pickup) |
| `1e45c50` | Reed | ♻️ `mirror.spec` cli-block additions — command blocks for `craft` / `init` / `recall` / `spawn`. T6 closure. `mirror compile` GREEN at OID `f48d7fc5`. |
| `4829dea` | Reed | 📝 `docs/loop/CURRENT.md` T6-landed cascade + next-tick queue |
| `5887ce2` | Reed | 🔧 `bin/mirror-mcp` — all 6 tools advertised; substrate-honest schema surface; mirror_spawn empirical loop closure (spec_oid `7edbde80`) |
| `0ab0040` | Reed | 📝 `docs/insights/2026-07-08-torus-double-closure-empirical.md` — spawn.peer_recall ≡ recall envelope byte-equal (Foerster's doubly-closed torus made empirical) |
| `39bfa14` | Reed | 📝 `docs/insights/2026-07-08-torus-axis-isolation-meridian.md` — meridian axis isolated (2/2 diverged, 0/2 longitude), reflexive commit as the winding advance |
| `eeac0a6` | Reed | 📝 CURRENT.md iteration 3 cascade |
| `1a57519` | Reed | 📝 `docs/insights/2026-07-08-torus-axis-isolation-longitude.md` — longitude axis isolated (1/2 diverged, 0/2 meridian), touch-without-commit as the mtime advance |
| `d0d95c1` | Reed | 🔴 `bootstrap/tests/spawn_mission_shard.rs` — RED for `cmd_spawn --mission` alias (4/6 fail as expected) |
| `8d6e9af` | Reed | 🟢 `bootstrap/src/lib.rs` cmd_spawn accepts `--mission` alias; --task backward-compat. 6/6 mission tests + 6/6 task tests pass |
| `cb7e26a` | Reed | 🔴 `bootstrap/tests/craft_target_kind_shard.rs` — RED for `cmd_craft --target-kind` alias (2/4 fail as expected) |
| `59c7fd0` | Reed | 🟢 `bootstrap/src/lib.rs` cmd_craft accepts `--target-kind` alias; --target backward-compat. 4/4 + full suite green |
| `091089d` | Reed | ♻️ cargo-fmt on pre-existing bootstrap/tests modifications; cargo fmt --check clean |
| `2164c8e` | Reed | 📝 `docs/loop/phase-h-deferral-2026-07-08.md` — supervisor.start_child + @fate.roll deferred with architectural friction identified |
| `79d3433` | Reed | 🔧 `bin/mirror-mcp` dispatch surface uses substrate names --mission and --target-kind |

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

## Bare-agent T5 diff (LANDED at `1e45c50`, T6 CLOSED)

**Drift closed**: `mirror.spec` cli-block declares 3 commands (`compile`,
`kintsugi`, `shatter`); binary implements 6 (`+ craft`, `+ init`,
`+ recall`, `+ spawn`). Diff adds 4 command blocks.

**Substrate-honest naming correction in the spawn block**:

```mirror
command spawn {  # renamed 2026-07-08 Tick 3 `96aa752` to `command peer { command beam }` + top-level `command beam`
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
3. Should `spawn`'s return-type `@song` (per then-`shards/mirror/spawn.mirror:270`;
   renamed 2026-07-08 Tick 2 `9de2226` to `shards/mirror/peer/beam.mirror`
   carrying `beam(...) -> @song`) surface in the cli-block? The cli-block
   has no return-type slot today.

**Full diff**: recoverable from session JSONL. The essential structural
move is above; per-line detail is in the transcript.

---

## Immediate next actions (in order)

1. ~~**T6**: inspect the bare-agent diff above. Adjudicate 3 questions.
   Apply the mirror.spec cli-block additions or refuse with reason.~~
   ✅ LANDED at `1e45c50`. Adjudication in commit body.
2. **Wire mirror MCP**: extend `bin/mirror-mcp` to advertise + dispatch
   `mirror_spawn` (also `mirror_init`, `mirror_recall` now that
   substrate declares them). Task #386's `@mcp.serve` lift reads the
   cli-block to synthesize this — two-tick target. Also open Claude Code
   from `/Users/alexwolf/dev/projects/mirror` so `.mcp.json` registers.
3. **cmd_spawn Rust evolution**: `--task <path>` → `--mission <path>`.
   Substrate leads (cli-block declares `mission`); RED first
   (`bootstrap/tests/spawn_mission_shard.rs`).
4. **cmd_craft Rust evolution**: `--target <kind>` → `--target-kind <kind>`.
   Substrate leads; RED first.
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

## Recognition cascade candidates (late-session 2026-07-08 — Reed catalog at `1e8a02b`)

Held for Alex numeric-ID assignment.

**Numeric-IDs assigned (Reed adjudication per Alex delegation):**

| ID | Candidate | Status |
|---|---|---|
| **#108** | `the-peer-IS-a-pain-driven-bounded-ontological-navigator` | META |
| **#109** | `algedonic-gradient-is-navigation-surface` | sub |
| **#110** | `depth-is-winding-is-spawn-parameter` | sub |
| **#111** | `@magic-is-level-shift-ceremony` | sub |
| **#112** | `@reflection-was-naming-artifact` | O5 consumption tick |

Taut `0fc8589` verified all 6 @reflection actions have LANDED
destinations. Alex-verbatim: "This IS what @reflection wanted to be."

**Adjacent promotions from prior arc (strengthened case, still candidates):**
- `@peer-has-a-torus` (this session 2026-07-07)
- `spawn-recall-byte-equal-at-origin` (iteration 2 empirical)
- `the-tournament-IS-multi-dim-Dijkstra-in-π₁(T²)-quotiented-@knapsack-space` (Mara `7e426bc`)

## Late-session (2026-07-08) landing chronology

| Commit | Author | Content |
|---|---|---|
| `7e426bc` | Mara | 📝 `docs/insights/2026-07-08-mara-geometric-dijkstra-tournament-topology.md` — 15-slot substrate audit; 14/15 filled; @edge sub-prism sketch |
| `e975e2f` | Taut | 📝 `docs/scouts/2026-07-08-taut-fate-silicon-metalogue-projection.md` — verdict LANDABLE WITH CASCADE; substrate-already-had-the-word discoveries (@algebra + @algebra/metalogue LANDED) |
| `a18ca90` | Mara | 📝 `docs/specs/fate-silicon-metalogue-in-void-duality-basis.md` — canonical spec at algebra altitude (@fate/algebra ⇔ @silicon/algebra IS @metalogue) |
| `77fe92d` | Mara | 📝 `docs/specs/peer-as-pain-driven-bounded-ontological-navigator.md` — canonical spec for the full closure (1072 lines) |
| `0fc8589` | Taut | 📝 `docs/scouts/2026-07-08-taut-pain-driven-bounded-ontological-navigator-projection.md` — verdict LANDABLE WITH CASCADE; 8 substrate-decl enrichments (4 non-contested, 4 adjudication-pending) |
| `1e8a02b` | Reed | 📝 `docs/insights/2026-07-08-recognition-cascade-peer-as-navigator.md` — META + 3 subs + O5 consumption catalog |
| `0d78c0c` | Mara | 🔧 `shards/epistemologic/cybernetic/reframe.mirror` NEW — the pain-authorized level-shift glass (204 lines; OID `6cd74265`) |
| `966890b` | Mara | ♻️ `shards/epistemologic/cybernetic/algedonic.mirror` EXTEND — sample_pleasure/sample_pain + algedonic_signal carrier (OID `237a3ec6`) |
| `cdc6533` | Mara | ♻️ `shards/mirror/spawn.mirror` EXTEND — mirror_spawn_request.winding field; `in @torus` composition (OID `045e9e20`). NOTE: `shards/mirror/spawn.mirror` renamed 2026-07-08 Tick 2 `9de2226` to `shards/mirror/peer/beam.mirror`; `mirror_spawn_request` → `mirror_peer_beam_request`. |
| — (no-op) | Mara | `boot/std/peer.mirror` — eigenboard already typed as `shard` per glass.mirror three-layer recognition; no edit needed |
| `eec00d0` | Reed | 📝 `docs/loop/CURRENT.md` — late-session cascade capture + real-spawn Path A/B/C/D options |
| `a823438` | Reed | 📝 cascade catalog update: numeric-IDs #108-112 + Reed adjudications A/B/C/D (Alex delegation "your call, I'm not interested in assigning numbers") |
| `7b32d27` | Mara | 🔧 `shards/edge.mirror` NEW — the collapse under @magic (Alex #4 "one surface"); 295 lines; 5 carrier fields per @magic species (OID `9a6226bc`) |
| `19f6d86` | Mara | ♻️ `shards/mirror/ref.mirror` — Path α collapse; edge_kind lifted from local decl to `in @edge` import (OID `c17dac11`) |
| `6ef5117` | Mara | ♻️ `shards/mirror/store.mirror` — typed-edges deferral removed; cites @edge as the fault-plane collapse (OID `0dc58ef4`) |

## Alex-adjudication queue — late-session cascade additions

Appended to the existing queue (which persists from the arc-to-rest session).
See `1e8a02b` § Alex-adjudication queue for full context.

**Alex-answered (2026-07-08 late session):**
1. ~~Numeric-ID assignment for META~~ ✅ Delegated to Reed; #108-112 assigned
2. ~~Numeric-ID assignments for 3 subs~~ ✅ Delegated; contiguous cluster
3. **O5 tick timing** — STILL OPEN. Land @reflection consumption alongside
   META promotion, or hold O5 for separate cleanup tick after substrate
   settles?
4. ~~Mara-Taut divergence on `peer.observe`/`peer.alter`~~ ✅ Alex "let's
   see what emerges"; Reed adjudication: hold; composition-discharge
5. ~~`@edge` fault-plane resolution~~ ✅ Alex "collapse into one surface";
   LANDED Mara `7b32d27` + `19f6d86` + `6ef5117`
6. ~~`@knife` substrate declaration~~ ✅ Alex delegated; Reed adjudication:
   doc-only Taut-primitive; NOT family-root; promote when second consumer
   PULLS
7. ~~`@onto` refusal confirmation~~ ✅ Feedback memory landed:
   `feedback-onto-family-root-is-the-ladder-Foerster-refused`
8. **Real spawn Path selection** (from Session-handoff above) — STILL OPEN.
   Alex answered: "the first spawn without a peer home. We need to first
   be able to spawn an agent that actually delivers something without a
   persistent identity before we can add the persistent identity layer on
   top." Reed asked for architectural clarification: Shape A (anonymous
   envelope only), Shape B (anonymous inference dispatch, blocked by fate
   API drift), Shape C (anonymous sub-agent dispatch via Agent tool),
   Shape D (split into `mirror invoke` + `mirror spawn` verbs). AWAITING
   Alex direction.

**Housekeeping still open:**
- Fate WIP stash at `stash@{0}` — keep as anchor for tomorrow, or drop?
- O5 tick timing (from #3 above)
- Real spawn architecture (from #8 above)

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
- `shards/mirror/peer/beam.mirror` (beam action, return type `@song`;
  formerly `shards/mirror/spawn.mirror`; renamed 2026-07-08 Tick 2
  `9de2226`)
- `shards/epistemologic/cybernetic/*` (13 species; the Foerster mapping
  lives here)
- `shards/magic.mirror` + 7 species

**Bootstrap Rust** (post-beam-refactor Ticks 0-3):
- `bootstrap/src/lib.rs` — `cmd_peer_beam` + `cmd_beam` dispatch
  (Tick 3 GREEN `b012d3f`). Formerly `cmd_spawn` at Phase G v0.
- `bootstrap/src/lib.rs` — `peer_beam` + `beam` match arms in top-level
  dispatch
- `bootstrap/tests/spawn_task_shard.rs` — 6 tests, all GREEN at `7625f42`
  (backward-compat alias preserved during rename)
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
