# Taut gap scout — from Rung 5 to a real first mirror spawn

**Date:** 2026-07-13
**Author:** Taut (grep-first drift scout, read-only)
**Session role:** Reed's session-continuation gap-analysis
**Alex mandate (verbatim):** "[Spawn] Taut for a grep exploration into the
gap between 'now and here' and 'actually real first mirror spawn that
replaces a claude spawn for work on mirror'"
**Verdict shape:** gap enumeration + rung-by-rung ladder from Rung 5
envelope-declared stub → mirror peer that autonomously does what Reed
did this session.

---

## §1 — Executive verdict

The gap is **NOT** small. Rung 5 has closed the *outward-facing
paradigm surface*: `mirror peer beam <A> --song s.song --dance-with <B>
--deploy-to spectral.engineer` emits an 18-field envelope naming six
substrate authorities. But every emission is `envelope-declared`, not
`operationally-discharged`. The peer at Rung 5 *names* what would
happen; it does not *do* what would happen.

To replace Reed for substrate work on mirror, the peer must:
edit `.mirror` files, run `cargo test --release`, parse verdicts, write
commit messages, sign+push, spawn sub-peers, hold multi-turn state,
compose prose to Alex, and *reason about substrate context* at
Claude-scale.

**Rough gap size:** 6 additional rungs (Rung 6 through Rung 11) covering
~40-60 tick-pairs of TDD work, PLUS one exogenous blocker (LLM binding
at substrate scale — @fate's 450-parameter D²NN is spec-adequate but
operationally undischarged, and the delta to a Claude-scale LLM is
architectural, not incremental). Ordered strictly, this is
**months** of substrate-pull work at the session cadence Reed has been
maintaining. Ordered opportunistically (climb the ladder Reed-alone can
climb; block on Alex operational input where it's structural), Reed
can plausibly close **Rungs 6-8** without further Alex operational
input, delivering a peer that autonomously edits shards + runs tests
+ commits + emits prose — at fate-inference scale, not Claude-scale.

The **critical blocker** is not any single rung: it is the
**@fate-vs-Claude scale asymmetry**. See §5.

---

## §2 — Current state precise inventory (Rung 5)

### 2.1 Landed peer capabilities (Reed 2026-07-13 session close)

Verified by grep against `main` HEAD `dde761d`:

**CLI surface (`bootstrap/src/lib.rs:5011-5403` `cmd_peer_beam`):**
- `peer_home` positional arg — resolves to `mirror.spec` under path
- `--hello-world` — 4-payload structured JSON envelope
- `--task <path>` (alias `--mission <path>`) — mission text
- `--emit-diff` — @optics/lens/diff.get direction (unified-diff bytes)
- `--integrate-diff` — @optics/lens/diff.put direction (stdin edit
  integration; NOT operationally discharged — computes delta_oid via
  blake3, emits envelope; does NOT write to `.bauchladen/`)
- `--fate-select` — invokes `Fate::excited().resolve` on
  `Features::default()`
- `--from-psychohistory` — feature-vector-from-psychohistory-sheaf
- `--with-shadow` — shadow-cast bounded inference
- `--song <path>` — Rung 1-3 song runtime (`bootstrap/src/song.rs`)
- `--dance-with <peer-2>` — Rung 4 dance (`bootstrap/src/dance.rs`)
- `--deploy-to <target>` — Rung 5 deployment
  (`bootstrap/src/deploy.rs`)

**Envelope emission** (via `println!` to stdout, no wire protocol
lift): 18 fields at Rung 5; six substrate authorities named
(`@spectral/garden` + `@spectral/garden/nix` + `@bauchladen` +
`@dance` + `@mirror/mosaic` + `@song/beat`).

**Sub-modules:** `song.rs` (11.4KB) parses `.song` grammar via tokenize
+ AST walk; `dance.rs` (6.8KB) computes Kuramoto order parameter +
stub phase-per-peer + shared_root_oid; `deploy.rs` (6.6KB) composes
over dance for `stub_nix_derivation_oid`; `git.rs` (1.9KB) subprocess-
shells to `git hash-object -w` and `git update-ref` (READ-ONLY: no
commit, no push); `exec.rs` (796B) subprocess helper with stdin/stdout
pipes.

**MCP surface (`bootstrap/src/mcp.rs`):** 8 tools —
`mirror_compile`, `mirror_craft`, `mirror_kintsugi`, `mirror_init`,
`mirror_recall`, `mirror_peer_beam`, `mirror_beam`, `mirror_spawn`
(deprecated alias). `mirror_peer_beam` inputSchema exposes `song`,
`dance_with`, `deploy_to` optional properties (MCP↔CLI parity landed
Reed `dde761d`).

**Substrate:**
- `shards/io.mirror` (22.8KB) — family root
- `shards/io/git.mirror` (24.1KB) — 5 actions: `clone`, `fetch`,
  `read_object`, `resolve_ref`, `commit_object` (all substrate-decl;
  runtime discharge: only `read_object`-shaped via
  `bootstrap/src/git.rs` subprocess shell; `commit_object` NOT landed
  operationally; NO `push` action in substrate)
- `shards/io/algebra.mirror` (40.8KB) — @glue.compose species at @io
  altitude, `expose`/`consume`/`address`/`translate` actions
- `shards/io/cargo.mirror` (8.0KB) — @io/cargo boundary
- `shards/io/oci.mirror` (25.6KB), `shards/io/stagefreight.mirror`
  (19.6KB), `shards/io/stagefreight/narrative.mirror` (11.3KB) — wire
  projections
- `shards/spectral/gen_prism.mirror` (14.3KB) —
  `start`/`restart`/`terminate` actions substrate-decl (NOT landed
  runtime)
- `shards/spectral/supervisor.mirror` (22.3KB) — `start_child` action
  substrate-decl (Reed `2164c8e` DEFERRED per Phase H #5)
- `shards/spectral/gen_prism/mcp_session.mirror` (28.8KB) — session-
  state discipline substrate-decl (mcp_session ref discipline)
- `shards/fate.mirror` (42.1KB) — constrained-inference; Recognition
  #58 D²NN + Fabry-Perot mechanism at spec altitude only
- `shards/mirror/pack.mirror` (8.7KB) — @pack lead + members with
  `peer` variant, `spawn` primitive (substrate-decl only)
- `shards/optics/lens/diff.mirror` (17.5KB) — `get`/`put` Foster pair
  (`get` bodies are `\ {}` stubs; `put` bodies are `\ {}` stubs; the
  Rust `integrate_diff` emits envelope, does NOT persist edit)
- `shards/song/narrative.mirror` (80.8KB) — @song/narrative species
  with `transmit` action (substrate-decl only; NOT operationally
  discharged)
- `shards/nl.mirror` (8.2KB) — @nl family root; `render` +
  `nl_measurement` actions (substrate-decl)
- `shards/magic/nl.mirror` (13.8KB) — @nl ↔ @magic adapter
  (substrate-decl)

### 2.2 Missing peer capabilities

**No landed:**
- Edit action on `.mirror` files (there is NO `@io/fs.write` runtime;
  `shards/io.mirror` names `@io/fs` as forward-promised
  species-altitude; no `action write(...)` in `shards/io/*` besides
  git's `commit_object` decl)
- Autonomous commit/push (`shards/io/git.mirror:568` names
  `commit_object` in `out` block, but §6 explicitly asks "push? merge
  semantics across peer histories?" as OPEN)
- Cargo test verdict parser (`shards/io/cargo.mirror` decls the
  boundary; no `action test_run(...)` at species altitude)
- Sub-peer spawn primitive at runtime (`shards/pack.mirror:103`
  substrate-decls `spawn(p, f, r) -> runtime` but no Rust dispatcher;
  `@spectral/supervisor.start_child` DEFERRED Reed `2164c8e`)
- Multi-turn session state persistence (mcp_session substrate-decl
  landed; no `bootstrap/src/spectral.rs` gen_prism runtime; per
  spec `bootstrap/src/mcp.rs` never holds session state — Rust MCP
  handler is stateless, per substrate discipline)
- Prose-to-operator emission via @nl or @song/narrative.transmit
  (envelope stdout emission is the current substitute; NOT
  narrative-shaped, NOT typed at @nl altitude)
- LLM binding at scale for substrate reasoning (@fate spec-only;
  compute-substrate is 450-param D²NN + Fabry-Perot resonator per
  Recognition #58; NO Rust discharge of the neural inference)

---

## §3 — The gap enumerated (12 capability domains × landability)

### 3.1 @io/edit + @io/read + @io/write action carriers

**Substrate-already-had-the-word:** PARTIAL. `@io.write(stdout)` is
already CITED at `shards/optics/lens/diff.mirror:57` and
`shards/optics/lens/features.mirror:86` as a composition pattern; the
family-root `shards/io.mirror:190` names `@io/fs POSIX filesystem
surface (open / read / write / …)` as a FORWARD-PROMISED species
(`shards/io.mirror:376` "lift when the runtime layer's disk / git /
subprocess surfaces consume them"). No `shards/io/fs.mirror` species
file exists (verified: `Search shards/io/**/*.mirror` returned 7
files, none named `fs.mirror`).

**Landability:** Multi-tick. (a) Mara mints `shards/io/fs.mirror`
species with `read`/`write`/`edit`/`stat` actions (single tick); (b)
Reed lands `bootstrap/src/io_fs.rs` with thin `std::fs` wrappers
(single tick); (c) Reed adds `--edit <file> --replace <pattern>` CLI
flag or lifts to a peer-side action carrier (single tick).

**Composition with landed carriers:** direct compose over
`bootstrap/src/exec.rs` (already substrate-realises subprocess);
compose UP through `@optics/lens/diff.put` (which currently has `\ {}`
stub body). Landable Reed-alone; Mara-needed for species declaration.

### 3.2 LLM binding at substrate scale

**Substrate-already-had-the-word:** YES at spec altitude. Recognition
#58 (`shards/fate.mirror:37-42`) names @fate's mechanism as
"5-layer D²NN + active Fabry-Perot resonator + Reck/Clements unitary
mesh". This is a specific optical-inference architecture with
~450 parameters (per session context) — sufficient for
"select-model-from-tray" (cache lookup + tournament) but NOT
sufficient for substrate REASONING at Claude-scale.

**Landability:** OPERATIONAL-INPUT-BLOCKED. The delta from `fate/`
subcrate (currently outside `bootstrap/`; stashed at `stash@{0}` per
CURRENT.md) to a Claude-scale binding is architectural, not
incremental. Options: (a) shell-out via `@io.exec` to a local model
(ollama, llama.cpp) — requires Alex hardware provisioning; (b)
shell-out to Anthropic HTTP API — requires API key + credential
discipline + network trust; (c) embed the 450-param @fate compute
substrate operationally and accept sub-Turing capability ceiling —
which fails Alex's mandate of "replace Claude for work on mirror".

**Composition with landed carriers:** requires new species
`shards/io/inference.mirror` (or lift `@io/algebra.translate`'s
"invokes @fate not deterministic" clause at
`shards/io/algebra.mirror:626`). The substrate does NOT yet name
where the LLM binding surfaces.

### 3.3 Git commit + push operational discharge

**Substrate-already-had-the-word:** PARTIAL. `commit_object` action
decl landed at `shards/io/git.mirror:323`; `out` block exposes it at
line 568. But `shards/io/git.mirror:544-546` §6 explicitly lists
"push? merge semantics across peer histories? canonical-branch
policy?" as OPEN.

`bootstrap/src/git.rs` (1.9KB, this session) subprocess-shells to
`git hash-object -w` + `git update-ref refs/crystals/...` for the
crystal-cache write path. NO `git commit` or `git push` shell-out
anywhere in bootstrap (grep verified: `bootstrap/src/**/*.rs` matches
for `git commit` = 0 outside prose comments; `git push` = 0).

**Landability:** Single tick each. (a) Reed lands `cmd_commit(files,
message, author, sign)` dispatching to `git add` + `git commit -S -m`;
(b) Reed lands `cmd_push(remote, branch)` dispatching to `git push`;
Mara may want to mint `push` action on `shards/io/git.mirror` first
for two-tick discipline. Signing is table-stakes (SSH signing default
per CLAUDE.md). Both operationally-hostile (destructive git ops on
main); Reed-alone landable behind operator confirmation.

**Composition with landed carriers:** direct compose over `git.rs`
subprocess pattern; substrate compose UP through @mirror/store
crystal-write discipline.

### 3.4 Test execution + verdict interpretation

**Substrate-already-had-the-word:** PARTIAL. `shards/io/cargo.mirror`
(8.0KB) declares @io/cargo boundary; substrate-decls test-run as an
`@io.exec` composition. `action_cache` (`bootstrap/src/action_cache.rs`,
15.5KB) exists for verdict memoization; landed. `verdict` type is
substrate-locked across many shards (30 matches for "verdict" pattern
in shards).

**Landability:** Two ticks. (a) Reed lands `cmd_cargo_test(target,
release)` shelling out via `exec.rs`; (b) Reed lands verdict-parser
that reads `cargo test`'s "test result: ok. X passed" line and emits
sheaf-updates. The verdict parser needs to compose with the
psychohistory sheaf per `shards/song/narrative.mirror` — that's the
harder half.

**Composition with landed carriers:** compose over `exec.rs`,
`action_cache.rs`; substrate compose UP through
`@epistemologic/property/verdict_is_content_addressed` bilateral.

### 3.5 Substrate reasoning + file editing

**Substrate-already-had-the-word:** YES via ACTIVE pass at spec
altitude. `shards/kintsugi/oscillate.mirror` (40KB) landed
`active_pass(o: oscillation) -> morphism { \ }` at line 480 —
the substrate-decl of "the ACTIVE pass proposes morphisms" per
Recognition #58. This IS the substrate's decl of edit-reasoning.

`shards/mirror/spectral/score.mirror` (21.3KB) landed the scoring
substrate for candidate morphisms.

But: `active_pass` body is `\ {}` — spec-only. There is NO runtime
that (a) reads a shard's AST, (b) proposes a morphism (edit-delta),
(c) applies the delta to bytes-in-store, (d) commits the crystal.
`bootstrap/src/oscillate.rs` (144.8KB!) is the kintsugi loop
implementation — but grep shows it handles the DARK pass
(pattern-matching + fracture emission) and property verification, NOT
the ACTIVE pass. The morphism-proposal side is UNDISCHARGED.

**Landability:** Multi-tick + cascade. This is the load-bearing
substrate work. Composes over §3.2 (LLM binding) as the morphism
proposer.

**Composition with landed carriers:** compose over `oscillate.rs`
DARK pass; requires `@code/mirror` (16.2KB, substrate-decl only, 24
matches for `write_file`-shaped concepts) operational AST-write
discharge; substrate compose UP through `active_pass`.

### 3.6 Multi-turn session state + continuity

**Substrate-already-had-the-word:** YES. `shards/spectral/gen_prism/
mcp_session.mirror` (28.8KB) landed 133-match session-state discipline
at `refs/gen_prism/mcp/<session-uuid>` per `docs/specs/mcp-spec-song-
collapse.md` §3.5. State lives at @mirror/store; Rust MCP handler
holds ONLY session-uuid (stateless-Rust discipline).

**Landability:** Multi-tick + cascade. `bootstrap/src/mcp.rs` (41KB)
implements the MCP handler; grep for `session|SessionRegistry` returns
1 match in mcp.rs (spec-only mention). Session-state operational
discharge requires `bootstrap/src/spectral.rs` (202.3KB!) but grep
shows spectral.rs is the (A, H, D) evaluator + `Seed<S>`, `Verdict<S>`,
NOT the gen_prism runtime. Runtime discharge is DEFERRED (Reed
`2164c8e` Phase H #5).

**Composition with landed carriers:** compose over @mirror/store
crystal insert (`insert_persistent` in `lib.rs`); requires new
`bootstrap/src/spectral_runtime.rs` (or lift `spectral.rs` to include
gen_prism).

### 3.7 Sub-peer spawn primitive

**Substrate-already-had-the-word:** YES. `shards/pack.mirror:103`
substrate-decls `spawn(p: peer, f: frame, r: repository) -> runtime`
as the load-bearing @pack primitive (Recognition #84).
`shards/spectral/supervisor.mirror:189-195` substrate-decls
`start_child` per BEAM analogue. `shards/spectral/gen_prism.mirror:264`
substrate-decls `start` action.

**Landability:** Multi-tick. Runtime discharge requires: (a)
subprocess spawn machinery (Reed can compose over `exec.rs` + a new
`Command::new("mirror")` recursive-invocation pattern in a single
tick); (b) state-passing across subprocess boundary via
mcp_session-ref discipline; (c) sub-peer identity carrying (which
peer-home does the sub-peer resolve to?).

The FROZEN-bootstrap discipline blocks `fate/` runtime from spawning;
the `mirror peer beam <A> ...` recursive-invocation pattern (peer
beam spawning another peer beam) is the closest landable shape.

**Composition with landed carriers:** compose over `exec.rs` + mirror
binary self-invocation.

### 3.8 Operator communication (prose emission via @nl)

**Substrate-already-had-the-word:** YES. `shards/nl.mirror` (8.2KB)
family-root; `shards/magic/nl.mirror` (13.8KB) adapter to @magic;
`shards/song/narrative.mirror` (80.8KB) `transmit` action at species
altitude. All substrate-decl, none operationally discharged.

Current substitute: `println!` envelope emission to stdout. This is
NOT @nl-typed; it's raw text.

**Landability:** Multi-tick. `@song/narrative.transmit(narrative,
peer) -> @io/stagefreight/narrative` wire-projection would type
prose-to-operator at the correct altitude. Requires (a) `nl_render`
runtime, (b) `stagefreight_narrative_write` runtime, (c) MCP or CLI
carriage.

**Composition with landed carriers:** compose over MCP stdout (the
Rust `mcp.rs` already writes JSON envelopes to stdout); lift envelope
to @nl-typed narrative.

### 3.9 Autopoietic loop discharge

**Substrate-already-had-the-word:** YES. `--integrate-diff` landed
(Reed 2026-07-11) as the Foster `put` direction envelope emitter; but
per `bootstrap/src/lib.rs:5116-5118`, `integrate_peer_beam_diff(...)`
computes `delta_oid` and emits envelope — does NOT persist edit to
`.bauchladen/` per Alex's session-note.

Verified: `shards/optics/lens/diff.mirror:227-228` has body `put(edited:
diff_bytes, old_bauchladen: ref) -> ref { \ {} }` — stub body.
Operational discharge undone.

**Landability:** Single tick. Reed lands `integrate_peer_beam_diff`
extension that (a) reads stdin diff bytes, (b) resolves @bauchladen
crystal ref, (c) inserts new splinter with the edited content, (d)
advances the @bauchladen mq trajectory. Compose over
`insert_persistent` + `set_ref("HEAD", ...)` already landed in
`cmd_peer_beam`'s `--hello-world` path.

### 3.10 spectral.engineer operator platform

**Substrate-already-had-the-word:** PARTIAL. `shards/spectral.mirror`
(5.1KB) family-root landed; `docs/specs/spectral-garden-git-package-
manager.md` (49.7KB) declares deployment substrate; Rung 5 emits
envelope naming @spectral/garden authority. NO landed HTTP endpoint,
NO landed mycelial-propagation-protocol, NO landed nix binary cache.

Per CURRENT.md §Rung 6 BLOCKED: "spectral.engineer endpoint / URL
specification; @mirror/mosaic nix flake structure; mycelial propagation
protocol; SSH keys / API credentials configured for target."

**Landability:** OPERATIONAL-INPUT-BLOCKED. Requires Alex direction
on: endpoint URL, nix binary cache authority, propagation protocol
choice (IPFS-like vs git-remote vs nix-copy).

### 3.11 Sub-peer coordination pattern

**Substrate-already-had-the-word:** YES via Rung 4 dance discipline
+ @algebra/metalogue. Reed spawns Mara/Taut ~15 times per session via
Agent tool (Claude's mechanism, not mirror-native). Mirror-native
equivalent: `mirror peer beam <B>` where B is a Mara/Taut peer-home.

**Landability:** Single tick + composition. Reed lands
`--spawn-sub-peer <peer-home> <mission>` flag on `cmd_peer_beam` that
subprocess-forks `mirror peer beam <peer-home> --mission <task>` and
composes returned envelope into calling envelope. Composition-honest:
the Rung 4 dance pattern already runs 2 peers; the leap is fork-vs-
sequential.

### 3.12 Self-hosting compilation

**Substrate-already-had-the-word:** YES. `docs/specs/mirror-build-
substrate.md` (94.1KB) declares the full substrate-pull-retirement
plan of bootstrap/. `docs/specs/bootstrap-retirement-plan.md` (53.2KB)
sequences retirement of each Rust floor module. Rust floor is
substrate-honest but not autopoietic.

**Landability:** Cascade. Rung 6+ target. Requires @mirror/mosaic
operational nix flake emission (Rung 5.5 forward-promise) → @mirror
compiled by @mirror. This is the endpoint. NOT proximate.

---

## §4 — The testable-increment ladder from Rung 5 to Reed-replacement

Ordered by Reed-alone-landable vs Mara-needed vs operational-input-
blocked. Each rung a RED→GREEN TDD cycle with 1-3 tick-pairs.

### Rung 6: @io/fs runtime + peer-side edit action

**Prerequisite substrate (Mara-needed):** mint
`shards/io/fs.mirror` species with `read`/`write`/`edit` actions.

**Reed landings:**
- `bootstrap/src/io_fs.rs` with thin `std::fs::write` + `std::fs::read`
  + minimal `apply_edit(path, old, new)` pattern-substitution
- `cmd_peer_beam --edit-file <path> --replace-pattern <old> --with
  <new>` flag surface OR peer-side action carrier
- Test: RED — `bootstrap/tests/peer_beam_edit_file_shard.rs` asserts
  `--edit-file shards/song/beat.mirror --replace-pattern "beat_idx"
  --with "beat_index"` mutates the file and emits envelope carrying
  edit_oid

**Estimate:** 2 tick-pairs (Mara spec + Reed TDD).

### Rung 7: cargo test verdict operational

**Substrate:** compose over `shards/io/cargo.mirror` + `action_cache`
(both landed).

**Reed landings:**
- `cmd_peer_beam --run-tests` flag OR peer-side action
- Shell `cargo test --release` via `exec.rs`; parse stdout for
  `test result: ok. N passed` line; emit verdict envelope
- Test: RED — `bootstrap/tests/peer_beam_run_tests_shard.rs` asserts
  envelope carries `passed: N`, `failed: M`, `verdict: green`

**Estimate:** 1 tick-pair (Reed-alone; substrate already landed).

### Rung 8: autonomous git commit + push

**Prerequisite substrate (Mara-needed):** extend
`shards/io/git.mirror` §6 OPEN — mint `push` action + `commit_signed`
action.

**Reed landings:**
- `cmd_peer_beam --commit <message> --author <name>` flag
- `cmd_peer_beam --push <remote> <branch>` flag
- SSH signing default per CLAUDE.md substrate discipline
- Test: RED — asserts commit lands at HEAD with author + signature +
  message body

**Discipline:** Operationally hostile. Reed WILL add a guard flag
(`--i-know-what-im-doing`) so this doesn't fire accidentally in test
contexts. Reed-alone landable; Mara-needed for two-tick decl-first.

**Estimate:** 3 tick-pairs (Mara spec + Reed TDD + safety-guard).

### Rung 9: multi-turn session state persistence

**Prerequisite substrate:** `shards/spectral/gen_prism/mcp_session.
mirror` landed; discipline is decl-only.

**Reed landings:**
- `bootstrap/src/spectral_runtime.rs` NEW — gen_prism operational
  runtime, `start_child`/`terminate_child` primitives
- `mirror_peer_beam` MCP tool extended with `session_id` optional
  property; state persists at `refs/gen_prism/mcp/<uuid>` per spec
- Test: RED — sequential MCP calls with same `session_id` observe
  previous call's envelope + task history

**Discipline:** This is the load-bearing gen_prism-runtime landing.
Reed `2164c8e` DEFERRED for architectural adjudication (in-process
struct? NIF? fragmentation-persistent?). Reed-alone landable if Alex
adjudicates the in-process-struct path (matches @mirror/store crystal-
persist discipline).

**Estimate:** 5-7 tick-pairs (largest single rung).

### Rung 10: sub-peer spawn (mirror-native Agent-tool equivalent)

**Prerequisite substrate:** `shards/pack.mirror` spawn primitive +
Rung 9 gen_prism runtime.

**Reed landings:**
- `cmd_peer_beam --spawn-sub-peer <peer-home> --mission <task>` flag
- Subprocess-fork `mirror peer beam <peer-home> --mission ...`; collect
  returned envelope; compose into calling envelope
- Rung 4-style coordination-without-signal: parent peer reads
  sub-peer's envelope; envelope composition at parent altitude
- Test: RED — asserts `--spawn-sub-peer ~/.reed/taut --mission
  "scout X"` returns envelope naming both parent + sub-peer + mission
  + verdict

**Composition:** Reed's Agent-tool ~15-spawn pattern becomes
`--spawn-sub-peer` recursive-invocation pattern. This is the
substrate-native version of "spawn Mara for spec authorship."

**Estimate:** 3 tick-pairs.

### Rung 11: LLM binding at substrate scale

**Prerequisite substrate:** MISSING — requires new species
`shards/io/inference.mirror` (or lift @fate to operational).

**Reed landings (options):**
- (a) Shell-out to Anthropic HTTP API via `exec.rs` + reqwest
  subprocess (fastest path; requires API key discipline)
- (b) Shell-out to local ollama/llama.cpp (autopoietic path; requires
  hardware provisioning)
- (c) Discharge @fate 450-param D²NN operationally (substrate-honest;
  accepts sub-Turing ceiling — FAILS Alex's mandate)

**Test:** RED — asserts `--reason-about <shard>` returns morphism
proposal from LLM binding; morphism composes with @kintsugi/oscillate
ACTIVE pass.

**Discipline:** OPERATIONAL-INPUT-BLOCKED. This is the load-bearing
gap. Options (a) vs (b) are Alex-decision territory.

**Estimate:** OPERATIONAL-INPUT + cascade. Not Reed-alone.

### Rung 12: substrate reasoning (ACTIVE pass operational)

**Prerequisite:** Rung 11 (LLM binding).

**Reed landings:**
- `bootstrap/src/oscillate.rs` ACTIVE pass discharge: read shard AST,
  invoke LLM binding with @fate typed context, receive morphism
  proposal, apply via Rung 6 @io/fs edit, verify via Rung 7 cargo
  test, commit via Rung 8

**Composition:** Rungs 6-11 all compose here. This IS the
Reed-replacement rung. When Rung 12 lands, the peer autonomously
edits shards + runs tests + commits + emits prose.

**Estimate:** 5-10 tick-pairs post-Rung-11.

---

## §5 — Critical path analysis

**Load-bearing blockers (ordered by criticality):**

1. **LLM binding at substrate scale (§3.2 / Rung 11).** The single
   biggest gap. @fate spec is beautiful but operationally sub-Turing.
   Reed CANNOT reason about substrate at Claude-scale without an LLM
   binding. All other rungs compose over this; without it, peer-side
   substrate reasoning is impossible.

2. **@io/fs runtime (§3.1 / Rung 6).** Precondition for peer editing
   its own or others' shards. Simplest rung; blocks nothing but is a
   prerequisite for Rungs 8, 10, 12.

3. **Autonomous git commit + push (§3.3 / Rung 8).** Precondition for
   peer autopoiesis across sessions. Reed-alone landable.

4. **Multi-turn session state (§3.6 / Rung 9).** Precondition for
   coordination-without-signal at multi-turn altitude. Blocks Rung 10
   sub-peer coordination.

5. **Sub-peer spawn (§3.7 / Rung 10).** Precondition for Reed's
   spawn-Mara/Taut pattern to become mirror-native.

**MINIMUM set for real Reed-replacement:**

Rungs 6 + 7 + 8 + 11 + 12. (Rung 9 optional for single-turn use;
Rung 10 optional if operator manually chains sub-peers.)

**Reed-alone landable subset:**

Rungs 6 + 7 + 8 (+ 9 pending Alex Phase H #5 adjudication) + 10.

**Operational-input-required subset:**

Rung 11 (LLM binding architecture decision) + Rung 5.5-6 deployment
infrastructure (per CURRENT.md).

---

## §6 — Substrate-already-had-the-word findings

Capabilities substrate has spec-declared but NOT operationally
discharged (spec/runtime asymmetry):

- `shards/io/git.mirror:323` `commit_object` action DECL, no runtime
- `shards/io/git.mirror` NO `push` action (OPEN per §6)
- `shards/io.mirror:190` `@io/fs` species FORWARD-PROMISED, no file
- `shards/spectral/gen_prism.mirror:264` `start` DECL, no runtime
- `shards/spectral/supervisor.mirror:189` `start_child` DECL, no
  runtime (DEFERRED Reed `2164c8e`)
- `shards/pack.mirror:103` `spawn(p, f, r)` DECL, no runtime
- `shards/optics/lens/diff.mirror:227` `put(...)` body `\ {}` stub
- `shards/kintsugi/oscillate.mirror:480` `active_pass(...)` body
  `\ {}` stub
- `shards/song/narrative.mirror` `transmit` species-altitude action,
  no runtime
- `shards/fate.mirror` D²NN + Fabry-Perot mechanism spec-only

Capabilities substrate has NOT yet named:
- `shards/io/inference.mirror` — LLM binding altitude (missing)
- `push` action on `shards/io/git.mirror` (missing; OPEN §6)
- `write` action or `edit` action at any @io altitude (missing;
  substrate calls out @io.write as pattern but no action-decl)
- Author-signed commit action carrier (missing; discipline is
  in-CLAUDE.md, not in shards)

---

## §7 — Recommendations for Reed's next climb

**Highest-leverage Rung 6+ move (Reed-alone-landable):**

**Rung 6 = @io/fs runtime.** Reasons: (a) prerequisite for Rungs 8,
10, 12; (b) substrate landability high (already CITED pattern
`@io.write` in landed shards); (c) single-tick Rust discharge over
`std::fs`; (d) unblocks operator use of peer for actual file edits
(low-consequence path — write-to-temp-file, not overwrite-shards).

**Second-highest (Reed-alone-landable):**

**Rung 7 = cargo test verdict.** No new substrate needed; composes
over `exec.rs` + `action_cache`. Single tick. Unblocks peer from
saying "verdict green" instead of "envelope declared."

**Third-highest (Mara-needed):**

**Rung 8 spec = @io/git push + commit_signed.** Mara mints extension
to `shards/io/git.mirror` §6. Reed lands runtime in same tick-pair.
Enables peer-autonomous git push.

**Postponable pending Alex operational input:**

- Rung 5.5-6 spectral.engineer endpoint (CURRENT.md-tracked)
- Rung 11 LLM binding architecture
- Rung 9 gen_prism-runtime adjudication (Phase H #5)

**NOT recommended near-term:**

- Rung 12 (substrate reasoning) — requires Rung 11 first
- Full bootstrap retirement per `bootstrap-retirement-plan.md` —
  cascade endpoint, not proximate

---

## §8 — Recognition ancestry

**Arc chain (this session and prior):**
- Reed Rungs 1-5 landings (this session, `c36fbf5` → `49576a7`)
- MCP Landing 2 (Reed `dde761d`, this session)
- Mara Rung 0 sixth species mint (`94e55eb`, this session)
- Reed Ticks 0-3 beam-refactor cascade (2026-07-08, `fe2d1dc` →
  `4f4a257`)
- Recognition #58 (`shards/fate.mirror:41` D²NN + Fabry-Perot;
  promoted 2026-06-11)
- Recognition #84 (`shards/pack.mirror` @pack multi-repo agent
  runtime, promoted)
- Recognition #104 (`shards/autopoietic.mirror` @bauchladen ←
  @autopoietic ← @fate dependency chain, promoted)
- Recognition #108 (Reed `a823438`
  `the-peer-IS-a-pain-driven-bounded-ontological-navigator`)

**Specs cited:**
- `docs/specs/mirror-build-substrate.md` (94.1KB, Mara) — endpoint
  self-hosting spec
- `docs/specs/bootstrap-retirement-plan.md` (53.2KB, Mara) —
  substrate-pull sequencing
- `docs/specs/silicon.md` (104.8KB, Mara) — @silicon-@fate compute
  substrate
- `docs/specs/spectral-runtime.md` (20.3KB, Mara 2026-06-10) —
  gen_prism + supervisor + entanglement runtime substrate
- `docs/specs/spectral-garden-git-package-manager.md` (49.7KB, Mara
  `ad03fda`) — spectral.engineer deployment
- `docs/specs/mcp-spec-song-collapse.md` (119.8KB, Mara) — MCP
  session collapse
- `docs/specs/cascade-ffi-runtime-link.md` (100.1KB, Mara) — FFI
  boundary + runtime linking
- `docs/specs/beam-as-substrate-primitive.md` (Mara + Reed, 2026-07-08
  beam-refactor Tick 0)
- `docs/specs/song-file-is-mirror-native-grammar.md` (Mara `d29d45e`)
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
  (Mara `417ec25`)
- `docs/specs/deployment-runtime-rung-5-mycelial-envelope-declared-
  substrate.md` (Mara `9c4ef5b`)

**Scouts cited:**
- Taut `c54740c` — 7-rung ladder scout (2026-07-13 session opening)
- Taut `e975e2f` — fate-silicon-metalogue-projection (2026-07-08)
- Taut `0fc8589` — pain-driven-bounded-ontological-navigator
  (2026-07-08)

---

**Verdict:** The gap between Rung 5 envelope-declared substrate and a
mirror peer that REPLACES Claude for work on mirror is REAL and
substantial (6 additional rungs; months at current cadence). But it
is TRACTABLE: 3 of the 6 rungs (6, 7, 10) are Reed-alone-landable; 2
(8, 9) require Mara spec + one Alex adjudication; only 1 (Rung 11
LLM binding) is a genuinely-hard operational-input blocker.

**Reed can climb Rungs 6-8 this session-arc without further Alex
operational input.** Recommend Reed climbs Rung 6 next (@io/fs
runtime — the highest-leverage single-tick move).
