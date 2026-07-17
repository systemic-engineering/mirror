# Taut scout — rust/ dance.rs rebuild gap

*Grep-first ground-truth of the delta between the current substrate-decl'd
compiler state and a working `rust/`-native rebuild of the binary + MCP
through the Mara Q3+Q5 answer (whole rust/ FLOOR collapses into dance.rs;
each prism = gen_prism actor; composition = @dance ensemble).*

**Author:** Taut
**Date:** 2026-07-17
**Tag:** 📝 Taut [rust-dance-rebuild-gap-scout] (pure-docs bypass)
**Discipline:** READ ONLY. Grep-first. No mints proposed. Substrate-truth
grep + gap enumeration + open questions for Alex.
**Scope:** what's missing between where we are (compiler broken by
construction after `~/.local/bin/mirror` deletion; `rust/` absent;
bootstrap/ legacy-status frozen) and a working rust/-native binary +
MCP composed over Mara canonical spec `fee2727` + Mara math root
`610c6d6` + prism `bundle.rs` reference implementation.
**Terminal position:** center on `rust/` + Mara canonical spec + prism
reference + Baez-Schreiber math + landed compositions.
`bootstrap/` referenced ONLY for functional-behavior enumeration (Q2)
and explicit legacy naming — never as the operational floor.

---

## §1 Q1 — `rust/` current state

**Finding:** `rust/` directory does not exist at project root.

- `ls /Users/alexwolf/dev/projects/mirror/rust/` → "No such file or
  directory".
- Search pattern `rust/**/*` → "the base directory 'rust' does not
  exist".
- No pre-existing Cargo manifest, no scaffolding, no partial file
  tree.

**Confirmed:** the rebuild starts from empty. `rust/` is a green
field. Every file in it will be net-new authoring at the Rust
altitude (§5 categorizes each).

**Not-yet-declared but load-bearing:**
- No `shards/rust.mirror` family-root shard-decl exists (grep
  `shards/**/*.mirror` for filename `rust.mirror` → absent). The
  existing family-root is `@code/rust` (referenced across
  `mirror.spec:82,197,207,214,223` as `altitude @code/rust`) —
  landed at `shards/code/rust.mirror` (implied by `@code/rust`
  citations).
- The path prefix `rust/` at PROJECT root (peer to `bootstrap/`) is
  not itself a substrate-decl surface — it's a FILE-SYSTEM directory
  the Rust workspace inhabits. Its authoring is a Cargo layout
  choice at the @io boundary, not a shard-mint.

---

## §2 Q2 — Functional-behavior enumeration (capabilities to preserve)

The current compiler (bootstrap/ altitude, legacy-status) currently
exposes ten CAPABILITY surfaces the rust/-native rebuild must preserve.
Enumerated by CAPABILITY, not by file. Bootstrap cited ONLY as the
current source of the functional list per this scout's discipline
constraint.

### §2.1 CLI verbs (from `mirror.spec:80-256` cli-block)

Ten commands landed in the substrate cli-block:
- `compile <path>` — tokenize one .mirror file through grammar lens.
- `craft <target>` — grammar-directory settlement; `--target-kind`
  emits code (binary/rust/gleam); `--reflect` verify-only.
- `kintsugi [<spec>]` — settle a project (mosaic on the spec); flags
  `--target list(str)` + `--emit_shatter`.
- `shatter <oid> <out>` — project a settled shard; `--target str`
  parameterizes @shatter's codomain.
- `init <path>` — mirror-native store bootstrap; `--install_hooks`.
- `recall <spec_dir>` — inbound-trajectory dual of spawn.
- `beam <mission>` — anonymous @song/movement.enter (no persistent
  identity).
- `index <path>` — @mirror/fractal-coherence measurement (top-16
  eigenvalues via LAPACK; Fiedler λ₀).
- `peer beam <peer_home>` — depth-2 recursive-command; persistent-
  identity variant; flag surface: `mission` / `hello_world` /
  `song` / `dance_with` / `deploy_to` / `emit_crystal` (rungs 1-6').
- `peer contribute <peer_home>` — depth-2; Fate-spawned peer
  contribution; `--target <shard>`.

### §2.2 MCP tool exposition

Eight `mirror_`-prefixed tools per `.mcp.json:1-6` + `bootstrap/src/
mcp.rs:107-224`:
`mirror_compile` / `mirror_craft` / `mirror_kintsugi` /
`mirror_init` / `mirror_recall` / `mirror_peer_beam` / `mirror_beam` /
`mirror_spawn` (deprecated alias). Plus two later additions per §2.2
of `mcp.rs`: `mirror_beam_act` + `mirror_index`. Total: 10 tools.

MCP dispatch chain: `bin/mirror-mcp` (17 LOC bash shim) → `exec
mirror /dev/stdin "@mcp.serve"` → `bootstrap/src/lib.rs:3122` matches
`@mcp.serve` or `@mcp` → `mcp::handle_request` dispatches `initialize` /
`tools/list` / `tools/call` / `notifications/initialized` per
`bootstrap/src/mcp.rs:665-730`.

### §2.3 Reflective evaluator surface (`apply_h::act`)

Seven combinators surface (per `mirror.spec` §Landing 12 `bootstrap/
src/apply_h.rs`, 81.4KB): the (A,H,D) reflective-evaluator dispatch
surface consumed by `mirror beam act <shard> <action> [args]`.
Empirical anchor: `mirror beam act @subject/visibility/public
consent_scope_universal` returns Pass exit 0 (per `mcp.rs:215`).
Bilateral arm collapse tests (5/5 passing per CURRENT.md: uuid/
spectral/time; @audhd; sheaf; @roomba bump/vacuum-mark-gc; reflective
bilateral dispatch smoke) — all resolver arms dispatch through
`apply_h::act`.

### §2.4 Corpus loader (walks `shards/**/*.mirror`)

Extracts bilateral blocks + species-decls + family-roots from
substrate. Consumed by evaluator, index, tokenize, grammar.
Consumers: `bootstrap/src/tokenize.rs` (38.3KB) + `bootstrap/src/
grammar.rs` (18.4KB) + `bootstrap/src/index.rs` (32.7KB).

### §2.5 @io boundaries (declared surface)

- `@io/git.commit` — commit authoring via `cmd_peer_beam` +
  `run_translate_cascade` (`bootstrap/src/lib.rs:1030`).
- `@io/fs.write` + `@io/fs.read` — corpus loader, action_cache,
  peer_persistence.
- `@nl.compose` — natural-language commit-message composition
  (chain naming).
- `@io/crypto` + `@io/secrets` + `@io/secrets/sops` — four-shard
  mint 2026-07-15 per CURRENT.md commit `059cf1c`.
- `@io/cargo` — `check`/`fmt_check`/`clippy`/`test`/`audit`/`bench`
  dispatch per `mirror.spec:196-249` targets.

### §2.6 Walker (Roomba)

Four motions landed per `mirror.spec` + `shards/kintsugi/roomba*.mirror`:
bump / vacuum-mark-then-prune / pivot. Fifth motion `dock` forward-
promised per Seam `2fdc9c1` §7 ALEX-Q2. Consumers: `bootstrap/src/
roomba.rs` (17.9KB) + `roomba_commit.rs` (33.4KB) +
`roomba_fracture.rs` (11.8KB). Bilateral arm collapse test:
`roomba_bump_vacuum_gc_bilateral_dispatch_smoke.rs`.

### §2.7 Fiedler / spectral measurement

`mirror index <path>` → top-16 eigenvalues via `prismqueer::ffi::
eigenvalues` (LAPACK dsyev); Fiedler = values[1] post-normalization.
Consumers: `bootstrap/src/index.rs` + `bootstrap/src/spectral.rs`
(202.3KB) + `sheaf_laplacian.rs` (24.5KB) + `cholesky.rs` (10.6KB).
Prism dependency: `prismqueer = { features = ["bundle", "lapack"] }`
per `bootstrap/Cargo.toml:29`.

### §2.8 Commit authoring (@nl.compose + @io/git.commit chain)

Composed dispatch: peer beam / roomba --commit / translate cascade
all reach `git commit` via subprocess at the @io boundary. Landed
substrate: `docs/specs/kintsugi-store-git-commit-as-fold-shard.md`
(referenced by `kintsugi_store_git_commit_as_fold_shard.rs` test).

### §2.9 Translation cascade (`--translate`)

`mirror roomba --commit --translate=<rs-file>` per `bootstrap/src/
lib.rs:972-1173`. Emits `shards/generated/rust_translated_<basename>.
mirror`. Substrate authority: Mara `1ce68c3` polyglot-loss-aware-
translation spec. Consumers: `polyglot_cascade_translation_smoke.rs`
test. @cascade/code/A/B species referenced.

### §2.10 Bilateral arm collapse (`--collapse=<rs-file>`)

`bootstrap/src/bilateral_arm_collapse.rs` (24.6KB). Route: when
`--collapse` points at a `.rs` file, dispatch through the pipeforward
§5.5.4 explicit force-a-collapse (per `lib.rs:3918`). Consumer:
`bilateral_arm_collapse_smoke.rs` test.

---

## §3 Q3 — MCP composition path

**Current MCP surface:**

- Config: `.mcp.json` (159B, minimal) — one stdio server named
  `mirror`, command `bin/mirror-mcp`.
- Wrapper: `bin/mirror-mcp` (888B; 17 LOC) — `exec ${MIRROR_BIN:-$HOME/
  .local/bin/mirror} /dev/stdin "@mcp.serve"`. **Broken by
  construction:** `~/.local/bin/mirror` was deleted this session per
  Alex direction; the shim now exits with ENOENT until the
  rust/-native binary lands at `MIRROR_BIN`.
- Runtime: `bootstrap/src/mcp.rs` (46.6KB) — `handle_request` per JSON-
  RPC 2.0; three methods: `initialize` (returns `serverInfo: {name:
  "mirror", version: "0.1.0"}` at line 84); `tools/list` (10-tool
  schema, hardcoded); `tools/call` (dispatch_tool_call). Bash-fixture
  byte-parity tests per `bootstrap/tests/mcp_fixtures/*.json`.

**No dedicated `@mcp` shard-decl exists.** Grep `shards/**/*.mirror`
for `@mcp\b|mcp\.serve` → hits are ALL prose citations, not species-
decl'd carriers. `shards/spectral/gen_prism/mcp_session.mirror`
exists (28.8KB) — `@spectral/gen_prism` specialization for MCP
session lifecycle; substrate-decl'd as a gen_prism actor variant.

**MCP through gen_prism actor pattern (Mara `fee2727` §2.1 + §2.5):**
gen_prism IS the substrate's gen_server analogue at BEAM/process
altitude; MCP sessions ARE gen_prism actors carrying (identity, state,
parent) triples. Under the Q3+Q5 answer, each MCP `tools/call`
dispatches through a gen_prism actor whose `state: shard_ref` carries
the per-tool substrate-decl, and message-passing IS the connection-
1-form parallel transport (per `docs/math/the-tower/beam-runtime.md`
§2.3). The MCP server itself IS a supervisor (§2.2 of Mara canonical)
with `restart_strategy: one_for_one` — each session gen_prism can
crash without crashing the server.

**Landed spec for MCP integration the rebuild targets:**
`shards/spectral/gen_prism/mcp_session.mirror` (28.8KB, 2026-07-12) —
the MCP-session-as-gen_prism species-decl. `bootstrap/tests/
spectral_gen_prism_mcp_session_shard.rs` (12.4KB) — RED test. Line-
cite gates: `.mirror` species-decl carries the gen_prism specialisation
that gates rust/-native rebuild's MCP surface.

**MCP integration path summary:**
1. `rust/main.rs` boots supervisor tree (per `shards/spectral/
   supervisor.mirror` restart_strategy).
2. Top-level supervisor spawns `mcp_server` gen_prism actor.
3. Each `initialize` request births a session gen_prism child under
   `mcp_server` supervisor.
4. Each `tools/call` dispatches through the session's gen_prism
   message-passing to the appropriate tool-handler gen_prism actor.
5. Tools/list schema derives REFLECTIVELY from `mirror.spec` cli-
   block via `apply_h::act` at boot (currently hardcoded in mcp.rs;
   the reflective form is forward-promised per mcp.rs:88-90 "the
   reflective form...is a heavier substrate-motion left for a future
   tick").

---

## §4 Q4 — Composition surface for the rebuild (landed)

### §4.1 Mara canonical spec (`fee2727`)

`docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-
connection.md` (28.2KB; 617 LOC). Grounds the four-part identity
(gen_prism = section; supervisor = connection at supervision altitude;
@dance = connection at ensemble altitude; dance.rs = Rust realization
of the ensemble connection). §5.5 explicitly REFUSES lifting
`bundle.rs` from prism into mirror ("mirror stays full mirror").

### §4.2 Mara math root (`610c6d6`)

`docs/math/the-tower/beam-runtime.md` (21.7KB; 490 LOC). Baez-
Schreiber 2004 principal 2-bundle 2-connection theorem grounding.
§2.1 Supervision trees ≅ simplicial Lie group tower. §2.2 Actors ≅
sections of a principal bundle. §2.3 Message passing ≅ parallel
transport. §6 @dance = ensemble connection 1-form.

### §4.3 Prism reference implementation

`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`
(20.3KB; ~626 LOC). Five-level tower: `Fiber → Connection → Gauge →
Transport → Closure → Bundle` supertrait chain. `GroupStructure` +
`LawvereFixedPoint` traits (idempotence + fixed-point axioms).
`Cyclic<N>` + `StableFiber` test bundle. AGENTS.md discipline:
mirror LIFTS the semantics; prism REALIZES them; NO lift needed.

### §4.4 Landed BEAM/OTP-analogue shards

- `shards/code/beam.mirror` (16.7KB) — four BEAM carriers.
- `shards/spectral/gen_prism.mirror` (15.0KB) — worker primitive
  (gen_server analogue).
- `shards/spectral/supervisor.mirror` (23.0KB) — supervision-tree
  primitive; `restart_strategy: one_for_one | one_for_all |
  rest_for_one`.
- `shards/spectral/gen_prism/mcp_session.mirror` (28.8KB) — MCP-
  session-as-gen_prism specialisation.
- `shards/spectral/restart_intensity.mirror` (6.1KB) — max_restarts
  per max_seconds circuit-breaker.
- `shards/spectral/entanglement.mirror` (33.3KB) — cross-actor
  coordination.
- `shards/spectral/parent.mirror` + `registry.mirror` + `root.mirror`
  + `portal.mirror` — supervision-tree kin.
- `shards/epistemologic/cybernetic/viable.mirror` (31.1KB) — Beer
  VSM S1-S5 (S1 = gen_prism).
- `shards/epistemologic/property/restart_intensity_well_formed.mirror`
  (13.9KB) + `shards/kintsugi/fracture/restart_storm.mirror` (19.2KB)
  — property + fracture on restart intensity.
- `shards/cascade/code/gleam/beam.mirror` (24.1KB) — Gleam-emit
  cascade for @code/beam.
- `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08).

### §4.5 Insights corpus (external, reference-cite only)

- `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-
  principal-bundle-tower.md` (2026-04-08; cascade source for Mara
  math root per `beam-runtime.md` §0).
- `~/dev/systemic.engineering/practice/insights/beam-elixir/` +
  `distributed-systems/` corpora — neuroaffirmative-lensing + BEAM
  insights.

---

## §5 Q5 — Gap enumeration

### §5.1 Net-new authoring (files that must be typed from empty)

| File | Purpose | Composition anchor |
|------|---------|--------------------|
| `rust/Cargo.toml` | Workspace manifest | copy shape from `bootstrap/Cargo.toml` deps: prismqueer + blake3 + serde + serde_json + libc |
| `rust/Cargo.lock` | Lock file | `cargo build` generates |
| `rust/src/main.rs` | Binary entry point | Boot supervisor tree; dispatch args → `dance::route` |
| `rust/src/dance.rs` | Ensemble connection 1-form (the FLOOR) | Mara canonical §2.5; message-routing + Kuramoto coupling |
| `rust/src/lib.rs` | Library entry (for tests) | Bootstrap analogue |
| `rust/src/mcp.rs` | MCP server as gen_prism supervisor | Compose over `mcp_session.mirror` species-decl |

**Question OQ1:** Does the rebuild need a separate `rust/src/mcp.rs`
if the Q3+Q5 answer says WHOLE floor collapses into `dance.rs`? Or
does MCP dispatch inline into `dance.rs` as one arm of the ensemble
connection?

### §5.2 Composition-lift (existing shard-decls → rust/ implementations)

Under `[substrate-floor:@io-boundary]` gate. The Rust files below
CONSUME the shard-decl'd substrate rather than re-declaring it:

- `@spectral/gen_prism` (LANDED) → each Rust prism-type in rust/
  becomes a gen_prism actor spawned under a supervisor.
- `@spectral/supervisor` (LANDED) → supervision-tree boot in
  `rust/src/main.rs` + gen_prism child_specs.
- `@dance` (canonical specs LANDED; shard-mint FORWARD-PROMISED to
  `dance.rs` empirical Path B firing per Mara `fee2727` §3.1) →
  Kuramoto coupling + Aumann envelope in `rust/src/dance.rs`.
- `@mirror/peer/beam` + `@mirror/beam` + `@mirror/index` + `@mirror/
  init` + `@mirror/recall` + `@mirror/beam/act` + `@spectral/
  gen_prism/mcp_session` — all landed shard-decls; rust/-native
  bodies compose over.

### §5.3 Substrate-decl'd (already in shards, consumers pull at
implementation time)

All 10 CLI verbs + 10 MCP tools + 7 apply_h combinators + 4 roomba
motions + Fiedler measurement + translate cascade + bilateral arm
collapse ARE substrate-decl'd. The rebuild does not need to re-mint
any of these; it CONSUMES them at rust/-native altitude via
apply_h::act dispatch.

### §5.4 Reference-cite only (AGENTS.md permits; not lifted)

- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` —
  five-level tower reference implementation. Consumed via
  `prismqueer = { version = "0.1", features = ["bundle", "lapack"] }`
  Cargo dep. NOT copied.
- `~/dev/systemic.engineering/practice/insights/beam-elixir/*` +
  `distributed-systems/*` — Mara-BEAM-arc insights corpus. Reference
  in docblock cascades only.
- `bootstrap/src/apply_h.rs` + `bootstrap/src/mcp.rs` + `bootstrap/
  src/dance.rs` + all bootstrap/src/*.rs — legacy-status; NOT lifted
  into rust/. The rebuild AUTHORS afresh; bootstrap remains as
  functional-behavior reference during the transition.

---

## §6 Q6 — Smallest-first MVP ordering

Per Alex "geometry will show us when" discipline: sequence produces a
working binary via smallest empirically-verifiable slice.

### Tick 1 — Empty scaffold + supervision-tree skeleton (RED)

- Create `rust/` directory.
- Author `rust/Cargo.toml` (minimal; prismqueer + serde + blake3 deps).
- Author `rust/src/main.rs` (5-line entry).
- Author `rust/src/dance.rs` empty module with `pub fn route(args:
  &[String]) -> i32` stub returning 0.
- Verify: `cd rust && cargo build` produces `rust/target/debug/
  mirror` binary; `./target/debug/mirror` exits 0.
- **Empirical anchor:** `mirror --help` prints nothing yet but binary
  runs. Green.

### Tick 2 — `mirror --help` prints something

- `rust/src/dance.rs::route` reads args[1] == "--help" → prints
  substrate-decl'd cli-block command list.
- Compose over `apply_h::act` at the shard-decl altitude: read
  `mirror.spec` cli-block; enumerate commands; print synopsis.
- **Empirical anchor:** `./target/debug/mirror --help` prints
  compile / craft / kintsugi / shatter / init / recall / beam /
  index / peer beam / peer contribute.

### Tick 3 — One CLI verb dispatches end-to-end

Simplest verb: `mirror compile <file>` — tokenizes ONE .mirror file
through grammar lens; returns SHA-256. Requires: grammar loader
(compose over `bootstrap/src/grammar.rs` altitude at shard-decl —
grammar lens IS the composable species per `shards/mirror/lens/cli/
compile.mirror`).

**Question OQ2:** Does Tick 3 lift ONE dispatch as gen_prism actor
under supervision-tree, or does it dispatch synchronously in
`dance::route`? Alex Q3+Q5 answer implies each prism = gen_prism
actor from Tick 3 onward.

### Tick 4 — MCP `initialize` handshake

- `rust/src/main.rs` matches `@mcp.serve` sentinel per current
  bootstrap/src/lib.rs:3122 pattern.
- MCP session gen_prism supervisor boots.
- JSON-RPC `initialize` returns `serverInfo: {name: "mirror",
  version: "0.1.0"}` — byte-parity with `bootstrap/tests/
  mcp_fixtures/initialize.resp.json`.
- **Empirical anchor:** `bin/mirror-mcp` shim now points at
  `rust/target/debug/mirror` via `MIRROR_BIN`; MCP handshake succeeds.

### Tick 5 — MCP `tools/list` returns byte-parity schema

- Reflective form: read `mirror.spec` cli-block at boot; emit tools
  schema derived from cli-block.
- OR hardcoded parity form: copy `bootstrap/src/mcp.rs::tools_
  list_result` structure.
- **Alex adjudication OQ3:** reflective at Tick 5 vs hardcoded-then-
  reflective at Tick 5+N?

### Tick 6 — First real tool dispatch

Simplest tool: `mirror_compile` (already dispatchable per Tick 3).
Wire `tools/call → dispatch_tool_call → mirror::compile`.
**Empirical anchor:** MCP client calls `mirror_compile`; returns
SHA-256.

### Tick 7-N — Remaining 9 tools + apply_h::act

Order per landed complexity: init → recall → beam → index → peer_
beam (rungs 1-6) → beam_act → craft → kintsugi. Each tick fires
one tool empirically; corresponding gen_prism actor lands; apply_h
combinator surface composed at appropriate ticks.

### Tick N+1 — Kuramoto coupling + Aumann envelope

`rust/src/dance.rs` grows the ensemble connection 1-form when
multiple gen_prism actors coordinate (peer beam --dance-with;
peer contribute). @dance shard-mint gates on this empirical firing
per Mara `fee2727` §3.1.

### Tick N+2 — bootstrap/ retirement

Delete `bootstrap/` directory. `mirror.spec` `legacy` block updates.
Substrate-pull-honest terminal state.

---

## §7 Alignment reading

### §7.1 What composes cleanly

- All 10 CLI verbs + 10 MCP tools are substrate-decl'd; rust/-
  native bodies CONSUME the shard-decls, no re-declaration needed.
- Fiedler measurement composes cleanly via `prismqueer` crate dep
  (already in bootstrap/Cargo.toml:29).
- Supervision-tree shape lifts DIRECTLY from `shards/spectral/
  supervisor.mirror` restart_strategy triple.
- MCP-session-as-gen_prism species-decl (`shards/spectral/gen_prism/
  mcp_session.mirror`) provides the load-bearing anchor for MCP
  integration through the actor pattern.
- prism `bundle.rs` provides the five-level tower reference; mirror
  cites, does not lift.

### §7.2 What needs net-new authoring

- `rust/Cargo.toml` + `rust/Cargo.lock` + `rust/src/main.rs` +
  `rust/src/dance.rs` + `rust/src/lib.rs` (if separate library
  crate needed for tests).
- MCP JSON-RPC handshake wire (currently in `bootstrap/src/mcp.rs`
  46.6KB); Rust re-authoring under gen_prism actor discipline.
- Corpus loader Rust code (currently in bootstrap/src/tokenize.rs
  + grammar.rs + index.rs; ~90KB total).
- Reflective evaluator (`apply_h::act` currently 81.4KB); rust/-
  native rebuild sequences through the 7 combinators.

### §7.3 What's the load-bearing risk

The Q3+Q5 answer says whole FLOOR collapses into ONE `dance.rs`.
Mara §5.6 refuses to write ANY `.rs` this arc. **Terminal shape
carries all 10 CLI verbs + 10 MCP tools + 7 combinators + walker +
Fiedler in ONE file.** Load-bearing question OQ4: is
`rust/src/dance.rs` monolithic (one file, everything), or is it
the ROUTER that dispatches into per-prism modules
(`rust/src/prism/compile.rs`, `rust/src/prism/index.rs`, etc.)?

Mara canonical §5.5 discipline: `dance.rs` COMPOSES the sections
(gen_prism actors); does NOT re-implement per-prism logic. This
implies dance.rs is the ROUTER, per-prism logic lives elsewhere —
BUT Alex Q3+Q5 verbatim says "whole rust/ FLOOR collapses into
dance.rs; each prism = gen_prism actor". The tension between
"whole FLOOR = one file" and "sections live in their own files"
needs Alex adjudication.

---

## §8 Open questions for Alex

**OQ1.** Does the rebuild need a separate `rust/src/mcp.rs` if
the Q3+Q5 answer says WHOLE floor collapses into `dance.rs`? Or
does MCP dispatch inline into `dance.rs` as one arm of the ensemble
connection?

**OQ2.** Does Tick 3 (first CLI verb dispatch) lift as gen_prism
actor under supervision-tree from the FIRST empirical firing, or
does it dispatch synchronously in `dance::route` until Tick N when
Kuramoto coupling lands?

**OQ3.** MCP `tools/list` at Tick 5 — reflective (parse mirror.spec
cli-block at boot) or hardcoded byte-parity with bootstrap/src/
mcp.rs? Mara `fee2727` §5.6 refuses to write .rs this arc; the
answer shapes whether reflective form lands day-1 or forward-promise.

**OQ4.** Is `rust/src/dance.rs` monolithic (one file carries all
10 CLI verbs + 10 MCP tools + 7 combinators + walker + Fiedler),
or is it the ROUTER dispatching into per-prism modules? Alex Q3+Q5
verbatim vs Mara canonical §5.5 tension.

**OQ5.** `rust/` at project root — is this a Cargo workspace with
`rust/` as one member (peer to `bootstrap/`), or is it a standalone
Cargo project with its own `Cargo.lock`? Affects `flake.nix` update
+ CI action + Justfile targets.

**OQ6.** Legacy retirement gate: does `bootstrap/` retire when
Tick N+1 lands (Kuramoto + Aumann), or when EVERY functional-
behavior capability (Q2 §2.1-§2.10) is empirically firing in
rust/-native surface? The former is faster; the latter is
substrate-honest.

**OQ7.** Do the 8 currently-passing bilateral arm collapse tests
(uuid/spectral/time; @audhd; sheaf; @roomba bump/vacuum-gc;
reflective bilateral dispatch smoke; peer_audhd; polyglot_cascade;
liquid_extraction) need rust/-native re-authoring, or do they
retire alongside bootstrap/ once rust/-native empirical firing
covers their empirical claims?

---

## §9 Recognition candidates surfaced

Do NOT ratify. Names proposed for Pack adjudication:

- **`#R-rust-floor-birth-is-supervision-tree-boot-not-file-authoring`**
  — first-witness THIS scout §6 Tick 1; second-witness gate: rust/
  main.rs boots supervisor gen_prism at Tick 1 with visible boot
  envelope naming spectral/supervisor authority.

- **`#R-mcp-session-is-gen-prism-actor-under-server-supervisor`** —
  first-witness `shards/spectral/gen_prism/mcp_session.mirror`
  species-decl; second-witness gate: rust/-native mcp session
  spawn observed at Tick 4 empirical `initialize` handshake.

- **`#R-tools-list-schema-is-reflective-projection-of-cli-block`** —
  first-witness bootstrap/src/mcp.rs:88-90 forward-promise ("reflective
  form left for future tick"); second-witness gate: rust/-native
  tools/list emits schema DERIVED at boot from mirror.spec cli-block.

- **`#R-dance-rs-monolithic-vs-router-is-alex-adjudication-load-bearing`**
  — first-witness THIS scout §7.3; second-witness gate: Alex
  adjudicates OQ4 in-transcript and the terminal file shape
  reflects the answer.

- **`#R-bootstrap-retirement-gate-is-empirical-not-tick-count`** —
  first-witness THIS scout §8 OQ6; second-witness gate: bootstrap/
  deletion tick fires only after every Q2 §2.1-§2.10 capability
  passes rust/-native empirical.

---

## §10 Audit chain

- **Alex 2026-07-17 verbatim (this session):** "I also want to
  detach bootstrap completely from the execution path. If that
  means the compiler breaks, then the compiler breaks. You keep
  touching and talking about bootstrap/ while rust/ is the floor.
  And I'm no longer willing to tolerate that."
- **Alex 2026-07-17 verbatim (this session):** "Delete the binary.
  Rebuild from rust/."
- **Reed session action:** deleted `/Users/reed/.local/bin/mirror`;
  compiler now broken by construction; MCP shim exits ENOENT.
- **Mara canonical spec:** `docs/specs/gen-prism-as-bundle-section-
  and-dance-as-ensemble-connection.md` (`fee2727`; 617 LOC; §1-§6).
- **Mara math root:** `docs/math/the-tower/beam-runtime.md`
  (`610c6d6`; 490 LOC; Baez-Schreiber 2004 §1-§7).
- **Mara docblock cascade:** `@code/beam` + `@spectral/gen_prism` +
  `@spectral/supervisor` + `@epistemologic/cybernetic/viable`
  (`3e4c3e3`).
- **Prism reference:** `/Users/alexwolf/dev/projects/prism/
  prismqueer/src/bundle.rs` (~626 LOC; five-level tower).
- **Landed BEAM/OTP shards:** `shards/spectral/gen_prism.mirror` +
  `shards/spectral/supervisor.mirror` + `shards/spectral/gen_prism/
  mcp_session.mirror` + `shards/code/beam.mirror` + `shards/
  epistemologic/cybernetic/viable.mirror`.
- **Insights corpus (reference-cite only):** `~/dev/systemic.
  engineering/practice/insights/beam-elixir/beam-as-principal-
  bundle-tower.md`.
- **Discipline anchors:** Reed memory `feedback_no_rust_extension_
  shortcut.md` + `feedback_detector_inadequacy_answer_is_never_
  rust.md`; AGENTS.md `[substrate-floor:@io-boundary]` marker;
  Michelangelo/marble refusal discipline; Mara `fee2727` §5.6 (no
  .rs this arc).

---

*Scout complete. Six per-Q findings, gap enumeration, MCP path,
Tick 1→N MVP sequencing, seven open questions, five recognition
candidates. bootstrap/ referenced only as legacy-status functional-
behavior source. rust/ centered as terminal Rust FLOOR per Alex
Q3+Q5 + Mara canonical spec.*
