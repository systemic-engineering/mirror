---
date: 2026-07-15
author: Reed
scope: Per-file migration map for the 5 Rust extensions Reed authored 2026-07-14 in violation of docs/specs/bootstrap-retirement-plan.md. Includes empirical check on native evaluator existence.
status: scout
companion: docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md
---

# Reed Rust-extension migration map — the 5 substrate-dishonest files from 2026-07-14

*Companion to the audit. This scout enumerates the substrate-honest
replacement for each of the 5 Rust files, the ordering, the dependency
graph, and — critically — the empirical check that surfaces THE
evaluator gap.*

---

## §1 The five files

| # | Path | LOC | Commit | Landing arc |
|---|------|-----|--------|-------------|
| 1 | `bootstrap/src/coherence.rs` | 217 | `422076d` | @coherence Rust runtime |
| 2 | `bootstrap/src/roomba.rs` | 425 | `422076d` | @roomba walker Scope A |
| 3 | `bootstrap/tests/roomba_walk_smoke.rs` | 84 | `d3d7d15` | end2end smoke |
| 4 | `bootstrap/src/spectral_signature.rs` | 177 | `99cd3ec` | @spectral/signature runtime |
| 5 | `bootstrap/src/peer_persistence.rs` | 420 | `99cd3ec` | @peer/persistence Landing C |

**Total.** 1323 LOC of Rust to migrate. Substrate-honest replacement is
~290 LOC of shard bodies + record declarations (~78% reduction). Plus
the evaluator gap (§5) which is the enabling work.

---

## §2 Per-file migration

### §2.1 `bootstrap/src/coherence.rs`

**Current state.**
- Functions: `coherence_score(&EigenvalueProfile) -> f64`,
  `coherence_delta(&EigenvalueProfile, &EigenvalueProfile) -> f64`,
  `coherence_increases(&EigenvalueProfile, &EigenvalueProfile) -> bool`.
- Tests: inline unit tests in the file.

**Substrate-honest replacement.**
- Bodies for the corresponding actions in `shards/epistemologic/cybernetic/
  coherence.mirror` (already substrate-decl'd by Mara `e0a3e48`).
- `coherence_score` body composes:
  `@mirror/index.eigenvalue_profile.fiedler_value(profile)`.
- `coherence_delta` body composes:
  `@math.subtract(coherence_score(after), coherence_score(before))`.
- `coherence_increases` body composes:
  `@math.gte(coherence_delta(before, after), 0)`.

**@io primitives invoked.** None. Pure arithmetic composition. The
Fiedler value is already a substrate-decl'd primitive on
`EigenvalueProfile` (Reed `c53a97c`); the arithmetic (`subtract`, `gte`)
is `@math` primitives already substrate-decl'd.

**Migration cost.**
- LOC delta: -217 Rust, +50 shard body.
- Test migration: inline tests become shard-body assertions using
  substrate's `verdict` / `imperfect` discipline.
- Consumer-pull adjustments: `crate::coherence::coherence_score` call-
  sites in `roomba.rs` and elsewhere rewire to shard-dispatch (blocked
  by §5 evaluator gap).

### §2.2 `bootstrap/src/roomba.rs`

**Current state.**
- Types: `RoombaStep { step_index, node_idx, node_label, sc5, tension,
  pain, coherence, knife_verdict, coherence_delta }`,
  `RoombaTrajectory { graph_node_count, steps, termination }`,
  `WalkTermination { BudgetExhausted, NoUnvisitedNeighbors,
  CoherenceMaximum }`.
- Functions: `walk(&Path, budget, epsilon_pain) -> RoombaTrajectory`,
  `summarize_trajectory(&RoombaTrajectory) -> String`, helpers
  (`highest_tension_unvisited_neighbor`, `tension_at_position`, etc.).

**Substrate-honest replacement.**
- A `shards/roomba.mirror` species-shard (does not yet exist; would
  ship with the migration) that substrate-decl's the record types and
  actions.
- `walk` body composes: `@mirror/index.build_concept_graph(root)` +
  `@spectral.sc5_at_node(graph, node)` + `@algedonic.sample_pain(sc5)` +
  `@math.variance(neighbor_pains)` (tension) + `@math.argmax(unvisited,
  tension)` + `@converge.stable_within(pain, epsilon_pain)` +
  `@coherence.coherence_score(profile_at_node)` + loop primitive.
- `summarize_trajectory` body composes: `@string.format` over the
  trajectory record.

**@io primitives invoked.** `@mirror/index.build_concept_graph` is @io
(reads the filesystem). Everything else is pure composition over
in-memory records.

**Migration cost.**
- LOC delta: -425 Rust, +100 shard body + record decls.
- Test migration: `roomba_walk_smoke.rs` (§2.3) migrates together.
- Consumer-pull adjustments: `bootstrap/tests/roomba_walk_smoke.rs`
  imports rewire; any downstream `use mirror::roomba::*` rewires.

### §2.3 `bootstrap/tests/roomba_walk_smoke.rs`

**Current state.** Cargo integration test asserting `walk(&root, 20,
0.1)` produces a non-empty trajectory over the mirror repo.

**Substrate-honest replacement.** Test-shape shard body in
`shards/roomba/test.mirror` (or the `roomba.mirror` shard itself) using
the substrate's TDD discipline — RED-first shard-body with `verdict`-
returning assertions. Discharged via `mirror kintsugi --ci` or a similar
test-dispatching lens.

**@io primitives invoked.** Same as §2.2 (the test walks the substrate
DAG, which is @io filesystem work).

**Migration cost.**
- LOC delta: -84 Rust, +30 shard test-body.
- Depends on `mirror kintsugi --ci` (or equivalent) being able to
  dispatch test-shape shard bodies; today only partial support exists.

### §2.4 `bootstrap/src/spectral_signature.rs`

**Current state.**
- Types: `SignatureBeat { contribution_oid, sc_at_beat, rung,
  previous_beat, timestamp, ssh_fingerprint }`, `RollingSignature
  { author_name, beats, current_sc, song_oid, garden_endpoint }`.
- Functions: `compute(author_name, garden_root, at) -> RollingSignature`.

**Substrate-honest replacement.**
- `shards/spectral.mirror` EXISTS (5.5KB, 2026-07-14). Body for the
  `compute` action composes: `@mirror/index.index(garden_root)` +
  `@fragmentation.sc5_hash(profile_bytes)`. Beats trail is a shard-body
  fold over a `@bauchladen.harvest_diff` when Landing C+ lands.
- `SignatureBeat` and `RollingSignature` records are substrate-decl'd
  in `shards/spectral.mirror` — not Rust structs.

**@io primitives invoked.** `@mirror/index.index` is @io (LAPACK
`dsyev` via `prismqueer` over filesystem).

**Migration cost.**
- LOC delta: -177 Rust, +30 shard body (records already decl'd).
- Test migration: none inline; tests to add as shard bodies.
- Consumer-pull adjustments: `peer_persistence.rs` uses
  `spectral_signature::compute` — that call-site migrates in §2.5.

**Key finding.** This is the worst of the 5 — Reed wrote Rust for
substrate that Mara had ALREADY substrate-decl'd in `shards/spectral.
mirror`. The failure is not just "wrote Rust" but "wrote Rust for
existing shards/". The pre-authorship grep would have caught it.

### §2.5 `bootstrap/src/peer_persistence.rs`

**Current state.**
- Types: `PeerHome { peer_name, home_path, projection_at, harvest_at,
  bauchladen_manifest, signature_snapshot, boot_state }`,
  `EigenboardState { subject_name, inference_basis, arousal,
  current_focus, winding }`, `Arousal { Quiet, Alert, Focus, ... }`.
- Functions: `materialize`, `harvest`, `boot`, `refresh`, `home_of` (5
  primitives from Landing A spec).

**Substrate-honest replacement.**
- Landing A spec at `docs/specs/peer-persistence-and-home-projection.md`
  already substrate-decl's all 5 primitives as `\`-obligation-blocked
  action bodies. The Landing C forward-promise IS filling those bodies
  as shard-body compositions.
- `materialize` body composes: `@mirror/store.write_crystal(name,
  bytes)` + `@fs.mkdirp(path)` (@io) + `@torus.advance(winding)`.
- `harvest` body composes: `@fs.readdir(path)` (@io) +
  `@bauchladen.diff_manifest(current, previous)` +
  `@sheaf.acl_project(candidates, subject_visibility)` (Landing D).
- `boot` body composes: `@spectral/signature.compute(peer, home)` +
  `@algedonic.arousal_from_sc5(current_sc)` + `@eigenboard.instantiate
  (subject, sc5, arousal, winding)`.
- `refresh` body composes: `@peer.materialize` + `@peer.harvest`
  (atomic).
- `home_of(subject_instance) -> peer_home` body composes:
  `@subject.instance_lookup(instance) -> peer_name` +
  `@peer.home_by_name(peer_name)`.

**@io primitives invoked.** `@fs.mkdirp`, `@fs.readdir`,
`@mirror/store.write_crystal`. All existing @io primitives.

**Migration cost.**
- LOC delta: -420 Rust, +80 shard body.
- Test migration: tests to be added as shard bodies.
- Consumer-pull adjustments: `bin/mirror` / `mcp.rs` / any peer-CLI
  consumers wire to shard-dispatch.

---

## §3 Ordering and dependency graph

```
                                    (evaluator gap §5)
                                            |
                                            v
                                    [ §2.4 spectral_signature ]
                                            |
                                            v
                                    [ §2.5 peer_persistence ]  <- LOC-largest first
                                            |
                                            v
                        +--- [ §2.1 coherence ] ---+
                        |                          |
                        v                          v
                [ §2.2 roomba ]  <-------  (needs coherence)
                        |
                        v
                [ §2.3 roomba_walk_smoke ]
```

**First to migrate (smallest, most self-contained).** §2.4
`spectral_signature.rs` — the shard already exists and the composition is
minimal; the migration IS the empirical proof-of-concept that shard-body
+ @io dispatch works.

**Second.** §2.1 `coherence.rs` — pure arithmetic composition; no @io
dependency beyond `EigenvalueProfile` which is already substrate-decl'd.

**Third.** §2.5 `peer_persistence.rs` — the largest and most consumer-
facing; requires §2.4 as a dependency. `@sheaf` ACL discipline forward-
promises Landing D; the migration can ship with a placeholder ACL
projection until then.

**Fourth.** §2.2 `roomba.rs` — depends on §2.1 for `coherence_score`
composition.

**Fifth.** §2.3 `roomba_walk_smoke.rs` — migrates with §2.2.

---

## §4 Migration cost aggregate

| Landing | Rust LOC removed | Shard-body LOC added | Net |
|---|---|---|---|
| §2.4 spectral_signature | 177 | 30 | -147 |
| §2.1 coherence | 217 | 50 | -167 |
| §2.5 peer_persistence | 420 | 80 | -340 |
| §2.2 roomba | 425 | 100 | -325 |
| §2.3 roomba_walk_smoke | 84 | 30 | -54 |
| **Total** | **1323** | **290** | **-1033** |

Plus the evaluator arc (§5). The evaluator itself IS legitimate
`[substrate-floor:@io-boundary]` work; the LOC for that is genuinely
FLOOR (irreducible), not the substrate-dishonest business logic that
composes over it.

---

## §5 Empirical check — does native execution discharge shard bodies today?

**The question.** Does the substrate have an evaluator that can run
shard action bodies natively? Test: pick a landed shard with a body
that's NOT `\`-obligation-blocked; can `mirror` execute it?

**Grep for evaluator state.**

- `bootstrap/src/exec.rs` — 796 B; `io_exec(cmd, args, input) -> (i32,
  Vec<u8>)` helper only. Butterfly's clang shell-out. Not a shard-body
  evaluator.
- `bootstrap/src/lib.rs::dispatch` — CLI dispatch; matches on argv[1]
  (`compile`, `craft`, `kintsugi`, `mcp`, ...) and dispatches to
  `cmd_*` Rust functions. All Rust; no shard-body execution.
- `bootstrap/src/kintsugi.rs`, `bootstrap/src/oscillate.rs`,
  `bootstrap/src/crystallize.rs` — the core execution modules. Each
  is a Rust implementation of a substrate-decl'd operation. None
  dispatch shard bodies; each IS the "shard body" reimplemented as
  Rust.
- Grep across `bootstrap/src/**/*.rs` for `fn eval | fn evaluate |
  fn exec_body | fn run_body | fn shard_dispatch | fn action_call |
  fn dispatch_action` returns **zero matches**.

**The shard bodies today.** Grep `shards/**/*.mirror` for action
bodies: nearly every landed action body is `\`-obligation-blocked
(e.g., `shards/mirror/lens/shell.mirror` — `enter`, `toggle`, `eval`
are all `\`). The few that have non-`\` bodies (e.g., `prism` blocks
declaring `focus shell` etc.) are the five-op algebra declarations
themselves, not action bodies that DO something.

**Test attempted.** Pick `shards/subject/visibility/public.mirror` (any
non-`\` action body). There is no `mirror execute <shard> <action>` CLI
verb, no `mirror eval` command, no runtime that takes a shard action
and produces a verdict. The action bodies are not dispatched by any
component of the current bootstrap.

**Finding — the evaluator gap.**

Native `mirror` execution does NOT dispatch shard action bodies today.
Every "execution" in the current substrate is Rust-implemented in
`bootstrap/src/*.rs`. The substrate's action bodies are declarative
(obligation-blocked or algebraically-declared) but not
executable-by-mirror.

**This is THE gap.** The retirement work for Rust extensions is not
just moving lines from `.rs` to `.mirror` — it is BUILDING THE
EVALUATOR that dispatches shard action bodies. Without the evaluator,
every migration in §2 is blocked because the substrate has no way to
run the shard-body compositions the migration produces.

**Implication for the marker.** The evaluator itself IS legitimate
`[substrate-floor:@io-boundary]` work under the tightened hook. It IS
irreducible FLOOR. It's the parser-as-Prism / apply_h retirement arc
from `docs/specs/bootstrap-retirement-plan.md` §"Tick 6 — tokenize.rs +
grammar.rs retire" — the parser combinator surface + `apply_h` as
shard-body dispatch. The retirement plan already names this arc; what
Reed did instead was skip the arc and write business-logic Rust on top
of the missing evaluator.

**Substrate-honest closure.** The 5 Rust extensions were not just
substrate-dishonest — they were an EXHAUSTIVE demonstration that the
evaluator gap is the real blocker. Every one of the 5 could have been
a shard body EXCEPT for the missing dispatcher. Reed's antipattern is
the shape of "the evaluator gap makes Rust extensions the path of least
resistance." The tightened hook + Seam gate now force the evaluator arc
to be worked as `[substrate-floor:@io-boundary]` work with Seam sign-
off, not as substrate-dishonest business logic on top of the gap.

---

## §6 The retirement arc — ordered sequence

Given the evaluator gap, the migration sequence is:

**Arc-1: Evaluator FLOOR (Seam-adjudicated substrate-floor work).**
1. **Tick 1.1** — Design the combinator surface for shard-body
   dispatch. Companion audit doc: Seam adjudicates whether the
   proposed surface is irreducible FLOOR or admits shard-body
   composition. Reference `docs/specs/bootstrap-retirement-plan.md`
   §"tokenize.rs — RETIRE (the big one)" + §"pipeline.rs — RETIRE".
2. **Tick 1.2** — Implement the evaluator as `[substrate-floor:@io-
   boundary]` Rust in `bootstrap/src/apply_h.rs` (or extend
   `bootstrap/src/spectral.rs`). Seam sign-off required. Smoke: dispatch
   one non-`\` shard body end-to-end.
3. **Tick 1.3** — Wire `mirror execute <shard-path> <action>` CLI
   verb through the evaluator. Ratifies Tick 1.2 empirically.

**Arc-2: Per-file migrations (each closes with `.rs` deletion).**
4. **Tick 2.1** — Migrate `bootstrap/src/spectral_signature.rs` →
   shard body in `shards/spectral.mirror`. Delete Rust file. First
   empirical proof-of-concept for the evaluator.
5. **Tick 2.2** — Migrate `bootstrap/src/coherence.rs` → shard body in
   `shards/epistemologic/cybernetic/coherence.mirror`. Delete Rust
   file.
6. **Tick 2.3** — Migrate `bootstrap/src/peer_persistence.rs` → shard
   bodies in `shards/peer/*` (new sub-family shards to mint). Delete
   Rust file. Placeholder ACL until Landing D `@sheaf` matures.
7. **Tick 2.4** — Migrate `bootstrap/src/roomba.rs` → shard body in
   new `shards/roomba.mirror`. Delete Rust file.
8. **Tick 2.5** — Migrate `bootstrap/tests/roomba_walk_smoke.rs` →
   shard test-body. Delete Rust test file.

**Arc-3: Post-migration cascade.**
9. **Tick 3.1** — Update `bootstrap/src/lib.rs` module registrations:
   remove `pub mod coherence; pub mod roomba; pub mod spectral_
   signature; pub mod peer_persistence;` declarations.
10. **Tick 3.2** — Update any external consumers (bin/mirror, mcp.rs)
    that referenced the retired modules.

**Estimated arc duration.** 3–5 sessions. Arc-1 is the load-bearing
chunk; Arc-2 is mechanical once Arc-1 lands. Arc-3 is cleanup.

---

## §7 Substrate-honest closure

This scout enumerates the migration path and names the evaluator gap
as the enabling work. It does NOT execute the migration this tick.
Executing the migration is the retirement arc; scoping it is this
scout.

Reed's role in the retirement arc:
- Tick 1.1 companion audit — Reed authors, Seam signs off.
- Tick 1.2 implementation — Reed authors `[substrate-floor:@io-boundary]`
  Rust, Seam signs off.
- Arc-2 per-file migrations — Reed authors shard bodies (no `.rs`),
  hook accepts (pure markdown / shard `📝` bypass), no Seam gate needed
  because no `.rs` change.
- Arc-3 cascade — Reed deletes retired `.rs` files; the deletion is
  `.rs` change but under `[bugfix:restore]` (restoring the FROZEN
  contract that these files should never have existed).

**Empirical check summary.** The evaluator does not exist today. The 5
Rust extensions were the substrate-dishonest workaround for its
absence. Building the evaluator IS the substrate-honest response.
Alex to adjudicate at Phase E whether the evaluator arc is prioritized
next in the /loop.

---

*Scout closure. The migration is scoped; the evaluator gap is named;
the retirement arc is queued for Alex Phase E adjudication.*
