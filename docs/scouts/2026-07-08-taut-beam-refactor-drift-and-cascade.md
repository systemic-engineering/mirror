# Taut Scout — `mirror spawn` → `mirror peer beam` Refactor: Drift + Cascade

*2026-07-08. Grep-first, read-only. Under 300 words of prose in the summary.*

## VERDICT

**RENAME LANDABLE WITH FAULT-PLANE SHIFTS + CASCADE (multi-tick).**

Not one-tick. The rename requires:
1. **A NEW altitude-4 substrate-already-had-the-word admission** — `beam` is not just three altitudes (Prismqueer / Erlang BEAM / optical). It is FOUR: mirror already declares `type beam(t)`, `grammar @beam`, and `type target = beam | wasm | metal | native` in `boot/`. That's altitude 4 — and altitudes 4, 5, 6, ...
2. **A substrate-level grammar extension** at `@mirror/lens/cli` — subcommand nesting is NOT declared as first-class today; the current `command(name) -> prism` grammar has no nested `command` slot.
3. **A cli-block cascade** touching mirror.spec + bin/mirror-mcp + docs/loop/CURRENT.md + the substrate-decl at `shards/mirror/spawn.mirror` and its cli hookup.
4. **A stance reversal on `@pack.spawn`** — the load-bearing @pack primitive is named `spawn` (`shards/pack.mirror:263`); the cli-surface rename does not (need to) propagate that name, but the two names diverge, which is the fault-plane shift.

---

## TASK 1 — Existing `beam` usages (substrate-already-had-the-word × 4, not × 3)

**The 4th altitude Alex's insight missed: mirror ITSELF already declared `beam` as substrate.**

### Altitude 1 — Prismqueer's Beam (Optic algebraic carrier)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:35` — `pub trait Beam: Sized { type In; type Out; type Error; type Loss; ... }` (23.8KB).
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:129-133` — `Beam<In=...>` bounds on the five-op Prism trait's focus/project/settle.
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs:110` — `Beam<In = Self::State>` bound on `Connection: Fiber` per spectral-triple-grammar §Gap 1.

### Altitude 2 — Erlang BEAM (Reed's body / hot-code + supervision)
- `shards/code/beam.mirror` — full `prism @code/beam` family-root (module_version, code_change_msg, supervisor, gen_server_state, swap_module). Landed tick 8 per Seam C2/C4.
- `/Users/reed/body/` — the Elixir/BEAM substrate itself (mentioned in `shards/pack.mirror:61`, `shards/code/beam.mirror` header).
- 20+ shard cross-references cite `@code/beam` (magic/reveal, magic/audit, code/wasm, cyberpunk, spectral, smarts, pack).

### Altitude 3 — Optical beam (Recognition #58, Fate optical inference)
- Prose across `shards/optics.mirror:114` ("typed beam channel at a boundary"), `shards/cogito.mirror:204-227` ("emit at @beam altitude"), `shards/mirror/spectral/portal.mirror` (`project_eigenvalue(stream: beam<oid>) -> beam<eigenvalue>`).
- `shards/mirror/lens/cli/sh.mirror` + `.../shatter.mirror` — cli-adjacent optical prose ("stage produces the beam; the detector collapses it").

### Altitude 4 — MIRROR'S OWN `@beam` GRAMMAR + `beam` AS TARGET-TRIPLE (was missed)
- `boot/std/beam.mirror` — declares `grammar @beam` with 12 abstract actions (emit, observe, compare, profile, baseline, sample, suite, lens, query, start, call, supervise, ...) AND `type beam(t) { luminosity, path, wavelength, holes, topology, loss, emitted, duration, fractures }`. This IS the mirror-substrate carrier for pipeline observation — NOT a rename target, an existing binding.
- `boot/07b-package-spec.mirror:6` — `type target = beam | wasm | metal | native`. **BEAM IS A COMPILATION TARGET at the package-spec grammar.** `mirror spawn` typing `mirror <verb> beam` would collide with `target beam { ... }` in mirror.spec's grammar at the parse-ambiguity altitude.
- `boot/std/fate/tournament.mirror:22` — `type rule = greedy | beam(u64) | ...`. Beam-search algorithm as tournament rule (`elite(1).beam(8).halving(3)`). NOT semantically related to optical beam — this is Ginsburg/Reddy 1966+ beam search. But the token collides.
- `boot/std/craft.mirror`, `boot/std/mcp.mirror`, `boot/std/mirror/compile.mirror`, `boot/std/mirror/liquid.mirror`, `boot/std/mirror/serve.mirror`, `boot/std/mirror/lsp.mirror`, `boot/std/kintsugi/shatter.mirror`, `boot/std/epistemologic/property/laws/causality.mirror`, `boot/std/epistemologic/property/total_classification.mirror`, `boot/std/spectral/portal.mirror`, `boot/std/epistemologic/math/spectral-triple.mirror`, `boot/std/mirror/glass/ast/shape.mirror`, `boot/std/mirror/glass/ast/shape/fixed.mirror` — ALL cite `in @beam` or reference `beam.observe`/`beam.emit`/`beam(t)`. That's a full-substrate dependency graph rooted at `@beam` as an existing grammar.

### Signal:

The rename claim is **substrate-already-had-the-word × 4** — but altitude 4 was not just NOT considered, it was ALREADY the substrate's canonical `beam` binding. Adding `mirror peer beam <peer_home>` as a cli verb introduces a **fifth altitude** (cli invocation surface for peer-persistent-identity-through-torus), not a re-use of altitudes 1-3.

Whether adding altitude 5 is drift or clarification depends on whether the substrate's mission is polysemy-under-controlled-ambiguity (Alex's × 3 count reads that way) or altitude-purity (the boot/ grammar reads that way). **Substrate-honest posture: name that altitude 4 exists, name that altitude 5 would be a NEW binding, and let Mara's canonical spec name the ambiguity discipline** (e.g., is `@beam` still one grammar or does `@mirror/peer/beam` become a lens species?).

---

## TASK 2 — cli-block subcommand nesting: NOT substrate-decl'd today

Per policy: I do NOT re-scout admissibility (Alex named geometric ground truth). Reporting current state.

`shards/mirror/lens/cli.mirror` declares:

```
command(name) -> prism { \ }
arg(name, t: type) -> prism { \ }
flag(name, t: type) -> prism { \ }
```

Line 78-89. **NO nested `command` slot inside `command`'s body.** The grammar treats `command X { ... }` as a leaf sub-prism whose body admits `arg`, `flag`, and `#` nl-literals only. `command Y { ... }` inside another `command X { ... }` block is NOT declared admissible.

`docs/specs/cli-as-prism.md` §3 depicts nested files (`shards/mirror/lens/cli/<verb>/<sub>.mirror` for depth-2), but §3.2 explicitly says depth-2 is **"reserved but not minted"** — the file layout admits it; the grammar at `@mirror/lens/cli` does NOT surface a `command` head inside another `command` body.

The eight sub-stage shards (`shards/mirror/lens/cli/{compile,kintsugi,shatter,bootstrap,sh,reflect,time,crack}.mirror`) each declare their OWN `stage @mirror/lens/cli/<verb>` prism with five ops — but these are SUB-STAGES at the FILE-STRUCTURE altitude, not nested `command` blocks at the `@mirror/lens/cli` grammar altitude.

### Substrate landing needed (grammar-extension forward-promise, NOT a .mirror body sketch)

The grammar at `shards/mirror/lens/cli.mirror` MUST gain either:
- A `command`-inside-`command` recursion admissibility (a `command(name) -> prism` that admits `command Y { ... }` nested in its body), OR
- A `subcommand(name) -> prism` head declared alongside `command` at the same altitude that IS the sub-glass declaration.

Either shape names subcommand nesting as first-class per Alex's ground-truth. Mara's canonical spec should choose the shape; the naming candidate is likely `command` recursion (per cli-as-prism.md §1.2 "sub-stage" framing) rather than a new `subcommand` head, because "sub-glass IS a glass" is a five-op algebra fixpoint (same head, recursively).

Consumers cascade-updating when this lands: `mirror.spec`'s `cli { command X { ... } }` blocks would gain optional nested `command Y { ... }` slots; `bootstrap/src/lib.rs` arg-parse would gain a positional-parse-depth loop. Two-tick discipline: land the grammar admissibility first (one tick), lift consumers when a consumer needs depth ≥ 2 (subsequent ticks).

---

## TASK 3 — Rename cascade projection: `mirror spawn` → `mirror peer beam`

### Direct cli-surface consumers (must update per rename, one-tick per file)

1. `mirror.spec:143` — `command spawn { arg peer_home: ~d; flag hello_world: bool = false; flag mission: ~f }`. Would become `command peer { command beam { ... } }` per subcommand-nesting-admissibility.
2. `bin/mirror-mcp` — 6-tool MCP schema advertises `"mirror_spawn"` name (line ~30-40 of the JSON), dispatches at line ~104-113 (`"mirror_spawn") spawn_args=("spawn" "$peer_home") ...`). Two updates: tool name (external contract with claude harness) + binary flag positional.
3. `bootstrap/src/lib.rs` — 47 matches. The relevant sites: line 621 usage message ("commands: ... spawn [--hello-world] [--mission ...] <peer-home>"), line 3050-3068 dispatch match (`"spawn" => match positional { ... cmd_spawn(...) }`), line 3835 `fn cmd_spawn(peer_home, hello_world, task, ctx)`. Would need `"peer" => match positional { Some("beam") => match positional... }`. Full dispatch surgery.
4. `bootstrap/tests/spawn_mission_shard.rs`, `bootstrap/tests/spawn_storage.rs`, `bootstrap/tests/spawn_task_shard.rs` — 17 + 40 + 17 matches. Test-name-and-invocation cascade.

### Substrate-decl consumers (deeper cascade)

5. `shards/mirror/spawn.mirror` — the substrate-decl itself. 14.8KB. Path-namespace property forces this file to be RENAMED to `shards/mirror/peer/beam.mirror` OR to `shards/mirror/beam.mirror` (depending on whether the cli surface reads `peer beam` as a two-word command sequence or `peer` as a first sub-command with `beam` nested). Either move breaks 15+ cross-references in the shard's own header (`@song/movement.enter`, `@spectral/supervisor.start_child`, mirror_spawn_request → mirror_peer_beam_request).
6. `shards/pack.mirror:263` — `spawn(pe: peer, f: frame, r: repository, pk: pack, p: perturbation) -> runtime` at line 263-270. THE LOAD-BEARING @pack primitive. **The rename PROPOSAL does NOT touch this name** — @pack.spawn stays as the substrate primitive; @mirror/spawn (cli-surface wrapper) becomes @mirror/peer/beam. But this creates a naming divergence between the substrate primitive (`spawn`) and the cli verb (`peer beam`) at consumer altitude, which may drift the "cli verbs ARE species-altitude actions" binding from spawn.mirror line ~110 ("The cli verb `mirror spawn` IS the species-altitude action `enter` at cli altitude. Same operation, two altitudes."). Fault-plane shift #1.

### Docs cascade (all → mention the rename or update wording)

7. `docs/loop/CURRENT.md` — 21 matches. Session-handoff prose, iteration landings, forward-work items — all use `mirror spawn`. Update to `mirror peer beam`.
8. `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` — 17 matches. Mara's spawn-as-λ₀-excitation insight. **The prose title stays** (spawn IS still the @pack primitive), but internal references to cli-surface invocations would update. Fault-plane shift #2: the historical arc names spawn-at-cli-altitude; the rename retroactively refactors that history.
9. `docs/specs/mcp-spec-song-collapse.md` — 15 matches. The M-cascade wiring §10 (M2 = spawn return type upgrade). Update wording.
10. `docs/audits/2026-07-06-seam-phase-d-m2-tick-1-spawn-song-return.md`, `docs/audits/2026-07-02-seam-spawn-as-loop-monad.md`, `docs/audits/2026-07-06-seam-phase-d-m-clean-tick-1-spawn-fate-hinge.md`, `docs/audits/2026-06-28-seam-mirror-build-substrate-composite.md` — audit trail. The audits are dated substrate-history; renaming them or their references is fault-plane shift #3 (do audits reflect the substrate at their date, or the substrate at read-time?).
11. `docs/math/spawn/spawn-as-loop-monad.md` + `docs/math/spawn/README.md` — canonical math directory named `docs/math/spawn/`. Rename directory to `docs/math/peer-beam/` OR leave as historical anchor.
12. `docs/scouts/2026-07-08-taut-pain-driven-bounded-ontological-navigator-projection.md` — 9 matches. Fresh scout from THIS SAME AFTERNOON that names `mirror spawn` throughout. Rename would immediately dirty scout that's not yet 24 hours old.

### Verdict on cascade shape

**Multi-tick, not one-tick.** Roughly:
- Tick 1: subcommand nesting admissibility in `shards/mirror/lens/cli.mirror` grammar.
- Tick 2: `shards/mirror/spawn.mirror` → `shards/mirror/peer/beam.mirror` (path-namespace forces the move); update `@song/movement.enter` composition binding, `@spectral/supervisor.start_child` composition binding, mirror_spawn_request → mirror_peer_beam_request.
- Tick 3: `mirror.spec` cli-block update (`command peer { command beam { ... } }`) + `bootstrap/src/lib.rs` cmd_spawn → cmd_peer_beam dispatch + tests rename + `bin/mirror-mcp` tool schema rename (external contract with claude harness).
- Tick 4: docs sweep across 12+ docs consumers.
- Tick 5 (optional): `mirror beam <mission>` top-level anonymous primitive as a peer-less variant.
- Two-tick discipline for `spawn` retention: keep `spawn` as backward-compat alias at the binary dispatch until a Reed-audited sunset tick lands.

---

## TASK 4 — Silent-conflict check

### 4a. `@song/movement.enter` at spawn.mirror composition — PRESERVED

`shards/mirror/spawn.mirror` §"Composition with @song/movement" declares "mirror spawn ~peer'<home>' IS @song/movement.enter at cli altitude — the frame-entry action of a temporal-bounded epoch at runtime." A rename to `mirror peer beam` PRESERVES the composition semantically (frame-entry through a peer's torus is still frame-entry), but breaks the wording of the composition claim. Fault-plane shift #4: the composition binding needs restatement ("mirror peer beam IS @song/movement.enter" or "mirror beam IS @song/movement.enter, peer-scoped when peer is given"). Substrate-honest wording lift; not structural break.

### 4b. `@spectral/supervisor.start_child` alongside spawn — PRESERVED

`shards/spectral/supervisor.mirror:445` declares `start_child(s: supervisor, spec: child_spec) -> gen_prism`. `shards/mirror/spawn.mirror` §"Composition with @spectral/supervisor" declares "@mirror/spawn KICKS the spawn through @spectral/supervisor.start_child rather than directly instantiating." **The fire-alongside contract is PRESERVED under rename** — `peer beam` still kicks the supervisor start_child; only the outer verb changes. No structural break.

### 4c. Recognition #43 (mirror IS content-addressed build system) — PRESERVED

`shards/mirror/spawn.mirror` cites #43 as "spawn IS the second first-order consumer of the Apache-2.0 floor." Under rename, the cli-verb changes but the CAS-advance semantics stay identical (empty session ref → peer-initialized ref). Prose update; no structural break.

### 4d. Recognition #58 (Fate IS optical inference) — PROMOTED BY RENAME

`beam` verb naming makes the optical altitude EXPLICIT at the cli surface. Recognition #58 currently lives in prose citations (`shards/mirror/spawn.mirror` §"Recognition ancestry"); the rename promotes it to cli-surface-visible. **This is the ONE structural upgrade the rename buys**: cli-surface polysemy on `beam` binds the four altitudes (Prismqueer / Erlang / optical / mirror @beam grammar) into one word that means "a persistent-identity-carrying photonic-shaped runtime traversal." Fault-plane shift #5 (upward): the cli-surface names the optical altitude publicly.

### 4e. Recognition #34 (@metalogue) — `pack beam` DOES fit as a metalogue turn shape

Cross-referencing `shards/pack/metalogue.mirror` §"handoff carrier" (line 302-323) + `shards/algebra/metalogue.mirror` §"algebra_turn carrier" (line 181-193): the metalogue lift-table (NL / AST / SPECTRAL / PACK / ALGEBRA) uses a `turn` body typed per altitude. At Pack altitude the body is `handoff` (source-agent, target-agent, content). Adding a "pack beam" cli surface that composes with `@pack/metalogue.handoff` would fit — `pack beam ~peer'X'` = "the pack sends a peer-directed handoff-shaped beam through peer X's torus." The composition is coherent under Mesland-category framing. Signal: candidate landing for a subsequent tick, not this rename's scope.

### 4f. `@peer-has-a-torus` (candidate, 2026-07-07) — CLARIFIED BY RENAME

The candidate `@peer-has-a-torus` recognition names the peer's persistent-identity-substrate as a torus (double closure per Foerster; `docs/insights/2026-07-08-torus-double-closure-empirical.md`). "Peer beam" reads as "a beam traversing a peer's torus" — the persistent-identity ontology becomes cli-visible. Fault-plane shift #6 (upward): the candidate becomes easier to promote. Substrate-honest: the rename doesn't PROVE @peer-has-a-torus; it makes the cli-surface consistent with it if it lands.

---

## Top-3 signals

1. **Substrate-already-had-the-word × 4, not × 3.** `boot/std/beam.mirror` + `boot/std/fate/tournament.mirror`'s `beam(u64)` tournament rule + `boot/07b-package-spec.mirror`'s `target = beam | ...` are ALREADY canonical `beam` bindings in the substrate. Alex named three altitudes; there are four. Adding a cli surface introduces a fifth. Whether that's clarification or drift is Mara's call at the canonical-spec altitude — Taut names the count.
2. **Subcommand nesting is not substrate-decl'd today.** `shards/mirror/lens/cli.mirror` line 78-89 admits only `command`, `arg`, `flag`, `default` heads inside `cli { ... }`; nested `command` inside `command` is not declared. Per Alex's ground-truth policy, this is a substrate landing to name (grammar-extension forward-promise), not a question to leave open. Mara's canonical spec should extend `command(name) -> prism` to admit nested `command Y { ... }` in its body (or declare a `subcommand` head).
3. **Multi-tick cascade, roughly 5 ticks.** Substrate grammar (tick 1) → substrate-decl move (tick 2) → mirror.spec + Rust + MCP + tests (tick 3) → docs sweep across 12+ consumers (tick 4) → optional `mirror beam` top-level anonymous variant (tick 5). Two-tick discipline for `spawn` as backward-compat alias until Reed-audited sunset. Not a one-tick swap.

---

## Summary (under 300 words)

Alex's proposal — rename `mirror spawn <peer_home>` to `mirror peer beam <peer_home>` and add `mirror beam <mission>` as top-level primitive — is LANDABLE but not one-tick. The substrate has FOUR pre-existing `beam` bindings, not three: (1) Prismqueer's `pub trait Beam` at `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:35`, (2) Erlang BEAM at `shards/code/beam.mirror` + `/Users/reed/body/`, (3) optical beam in Recognition #58 prose across ~10 shards, (4) mirror's own `grammar @beam` at `boot/std/beam.mirror` + `target = beam | wasm | metal | native` at `boot/07b-package-spec.mirror:6` + `beam(u64)` tournament rule at `boot/std/fate/tournament.mirror:22`. Adding a cli surface introduces a fifth altitude; substrate-honest posture names that.

Subcommand nesting is NOT substrate-decl'd at `shards/mirror/lens/cli.mirror` today; the grammar's `command(name) -> prism` head admits no nested `command` in its body (line 78-89). Per Alex's ground-truth policy, this is a substrate landing (grammar extension forward-promise), not an admissibility question.

Cascade touches 12+ consumers across substrate + Rust + MCP + tests + docs + audits. Estimated 5-tick cascade with `spawn` retained as backward-compat alias. Silent conflicts on `@song/movement.enter` composition, `@spectral/supervisor.start_child` fire-alongside contract, and Recognitions #43/#58/#34/@peer-has-a-torus all PRESERVED or PROMOTED under rename — no structural breaks, just prose lifts. Rename PROMOTES Recognition #58 (Fate IS optical inference) to cli-surface visibility. `@pack.spawn` substrate primitive (`shards/pack.mirror:263`) stays as-is; naming divergence between substrate primitive and cli verb is fault-plane shift #1.

Recommendation: LANDABLE WITH FAULT-PLANE SHIFTS + CASCADE. Let Mara's canonical spec name the altitude-4 admission and choose the subcommand-nesting grammar shape.
