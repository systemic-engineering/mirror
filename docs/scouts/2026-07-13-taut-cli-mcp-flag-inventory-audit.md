# Taut CLI/MCP flag inventory audit — 2026-07-13

📝 Taut [substrate-drift:scout] [cli-mcp-audit]

HEAD at audit: `90019c46a97e9d3b3764dd5d874291a919fb731c` (Reed Rung 6.1c
close, `commit_as_fold` discharge). Read-only grep-first inventory of
the CLI + MCP surface, prepared as ground for Mara's canonical CLI/MCP
composition spec per Alex's 2026-07-13 in-transcript direction:

> "we're getting to the point where we wanna really get into the nitty
> gritty design of the CLI and MCP; at the moment we have a lot of
> --options that arguably are just what the commands should always do."

## §1 — Executive summary

- **Top-level CLI verbs:** 10 substrate-decl'd + 3 runtime-only (with
  1 backward-compat alias + 1 MCP pipeline head). Substrate-decl'd:
  `compile`, `kintsugi`, `shatter`, `craft`, `init`, `recall`, `beam`,
  `peer` (recursive → `peer beam`). Runtime-only: `spawn` (deprecated
  alias), plus two mq-pipeline paths that special-case `@mcp.serve`.
- **`peer beam` flag counts:** 6 substrate-decl'd in `mirror.spec`
  cli-block (`peer_home`, `hello_world`, `mission`, `song`,
  `dance_with`, `deploy_to`, `emit_crystal` — one arg, six flags),
  but **11 parsed at runtime** in `cmd_peer_beam`. Substrate-decl
  covers `hello_world`, `mission`, `song`, `dance_with`, `deploy_to`,
  `emit_crystal`; **five runtime-only flags** (`fate_select`,
  `from_psychohistory`, `with_shadow`, `emit_diff`, `integrate_diff`)
  are undeclared at cli-block altitude.
- **Composition shape:** the dispatch is **priority-ordered mode-
  switching**, not additive. Six of the eleven flags are runtime-modes
  that early-return from `cmd_peer_beam` before any downstream flag
  fires. Only `hello_world` and `mission` are truly additive over the
  base "peer beam text envelope" path.
- **MCP surface:** 8 tools; `mirror_peer_beam` alone carries **11
  properties** (matching the runtime cascade, exceeding the substrate-
  decl). Composition pattern is one-shot per `tools/call`; there is no
  session/loop primitive.
- **Substrate-decl vs runtime drift:** 5 runtime-only flags, 3
  documented aliases (`--task`↔`--mission`, `--target`↔`--target-kind`,
  `mirror spawn` → `mirror peer beam`), and one MCP arg alias
  (`fate_select` at MCP → `--fate-select` at CLI). No verb collapses
  the mode-switching runtime dispatch into structure.

## §2 — Complete CLI verb tree (as of `90019c4`)

### 2.1 Substrate-decl'd verbs (`mirror.spec` cli-block, lines 80–283)

Location: `mirror.spec` `project mirror.spec { ... target binary {
cli { ... } } }`. Grammar per `shards/mirror/lens/cli.mirror:157–174`
(`command(name)`, `arg(name, t)`, `flag(name, t)`).

```
mirror
├── compile <path>                            (mirror.spec:82)
│     └── --strict           bool  = true     (mirror.spec:84)
├── kintsugi [<spec>]                         (mirror.spec:87, arg default = ./mirror.spec)
│     ├── --target           list(str) = []   (mirror.spec:90)
│     └── --emit-shatter     bool  = false    (mirror.spec:91)
├── shatter <oid> <out>                       (mirror.spec:94)
│     └── --target           str   = "auto"   (mirror.spec:110)
├── craft <target>                            (mirror.spec:123)
│     ├── --target-kind      str   = "binary" (mirror.spec:125)
│     └── --reflect          bool  = false    (mirror.spec:126)
├── init <path>                               (mirror.spec:136)
│     └── --install-hooks    bool  = false    (mirror.spec:138)
├── recall <spec_dir>                         (mirror.spec:149)
├── beam <mission>                            (mirror.spec:171)
│     └── --hello-world      bool  = false    (mirror.spec:173)
└── peer                                      (mirror.spec:212)
    └── beam <peer_home>                      (mirror.spec:213)
          ├── --hello-world  bool  = false    (mirror.spec:215)
          ├── --mission      ~f              (mirror.spec:216, no default)
          ├── --song         ~f              (mirror.spec:229)
          ├── --dance-with   ~f              (mirror.spec:246)
          ├── --deploy-to    ~f              (mirror.spec:261)
          └── --emit-crystal bool  = false    (mirror.spec:279)
```

### 2.2 Runtime-only verbs (`bootstrap/src/lib.rs`)

Grep pass over the `match args[1].as_str()` block starting at
`bootstrap/src/lib.rs:3124`:

- `"spawn"` (lib.rs:3170–3230) — backward-compat alias for `peer beam`
  (Tick 3 Landing 2). Emits stderr deprecation notice at lib.rs:3181.
  Parses ALL 11 runtime `peer beam` flags and re-dispatches into
  `cmd_peer_beam`. Substrate-decl absent from `mirror.spec` cli-block.
- `"peer"` (lib.rs:3232–3336) — depth-2 recursive-command match arm.
  Only sub-verb: `"beam"` (lib.rs:3251). `other =>` at lib.rs:3328
  errors with `"unknown: peer {}"` — no forward-promise for future
  sub-verbs even though the mirror.spec docblock at lines 145–159
  reserves `command peer { ... command spawn { ... } }` for the
  backward-compat variant.
- **mq-pipeline heads `@mcp.serve` / `@mcp`** (lib.rs:2837–2851,
  is_mcp_serve_head; lib.rs:2887 + 2916 dispatch) — NOT verbs at
  `match args[1]` altitude. Route through the mq path (Path A or B
  of dispatch) and short-circuit to `mcp::serve_loop` before pipeline
  execution. The only user-facing invocation is via the `bin/mirror-
  mcp` shell shim (`bin/mirror-mcp:15`):

  ```
  exec "${MIRROR_BIN:-$HOME/.local/bin/mirror}" /dev/stdin "@mcp.serve"
  ```

  So `mcp` is **not** a first-class CLI verb; it is a substrate-decl'd
  pipeline head at `boot/std/mcp.mirror` dispatched at mq altitude.

### 2.3 Verb-count reconciliation

Substrate-decl'd verbs at depth-1: 8 (`compile`, `kintsugi`, `shatter`,
`craft`, `init`, `recall`, `beam`, `peer`). Runtime-observed verbs at
`match args[1]`: 9 = same 8 + `spawn`. Deprecation window per Reed
`b012d3f`; no removal tick landed.

## §3 — Flag-by-flag audit table (`mirror peer beam`)

Runtime dispatch order per `cmd_peer_beam` (lib.rs:5031–5155). Every
flag is either **early-return** (runtime mode switching) or
**cascaded downstream** (additive over the base envelope path).

| # | Flag | lib.rs cite | Dispatch entry | Substrate-decl | Shape | Notes |
|---|---|---|---|---|---|---|
| 1 | `--emit-crystal` | 5049 | `emit_peer_crystal` (store_branch.rs:63) | ✅ mirror.spec:279 | **early-return mode** | FIRES FIRST. Skips Rungs 1-5 + fate/diff branches entirely. Rung 6' substrate-inversion. |
| 2 | `--deploy-to <f>` | 5065 | `dance::compute_dance_state` + `deploy::execute_deploy` (deploy.rs:52) | ✅ mirror.spec:261 | **early-return, requires --song + --dance-with** | Three-way narrowing `if let (Some, Some, Some)`. Rung 5. |
| 3 | `--dance-with <f>` | 5089 | `dance::execute_dance` (dance.rs:71) | ✅ mirror.spec:246 | **early-return, requires --song** | Two-way narrowing `if let (Some, Some)`. Rung 4. |
| 4 | `--song <f>` | 5110 | `song::single_beat_peer_beam` → `execute_song` (song.rs:44) | ✅ mirror.spec:229 | **early-return** | Rungs 1-3. |
| 5 | `--emit-diff` | 5119 | `emit_peer_beam_diff` (lib.rs) | ❌ RUNTIME-ONLY | **early-return** | Foster get-direction on @optics/lens/diff. Reed `4b2ef3c`. |
| 6 | `--integrate-diff` | 5136 | `integrate_peer_beam_diff` (lib.rs) | ❌ RUNTIME-ONLY | **early-return** | Foster put-direction. **Mutually exclusive with `--emit-diff` (order-precedence wins integrate_diff since it fires later — but `emit_diff` returns first so it actually wins).** Contradicts MCP schema comment which claims "integrate wins." |
| 7 | `--fate-select` | 5147 | `fate_select_peer_beam` / `fate_bounded_by_psychohistory_peer_beam` / `fate_bounded_shadow_peer_beam` (lib.rs:4703 + …) | ❌ RUNTIME-ONLY | **early-return mode + trichotomy** | Nested with `--from-psychohistory` + `--with-shadow`. |
| 8 | `--from-psychohistory` | 5147 (inside `if fate_select`) | (nested) | ❌ RUNTIME-ONLY | **modifier, requires --fate-select** | Silently no-op if `fate_select` false. No error. |
| 9 | `--with-shadow` | 5147 | (nested) | ❌ RUNTIME-ONLY | **modifier, requires --fate-select + --from-psychohistory** | Silently no-op if either predecessor absent. |
| 10 | `--hello-world` | 5203 | JSON envelope path in `cmd_peer_beam` body | ✅ mirror.spec:215 | **additive on base envelope** | Switches text↔JSON but is downstream of ALL early-returns; only fires when no mode flag caught the dispatch. |
| 11 | `--mission <f>` / `--task <f>` | 3193, 3260, 3376 | `task: Option<&str>` param carried through | ✅ mirror.spec:216 (`--mission`); `--task` is undeclared runtime alias (lib.rs:3193, 3260, 3376) | **additive value flag** | Consumed by fate + hello_world paths; ignored by song/dance/deploy/emit-crystal. |

### 3.1 Order-of-dispatch cascade (lib.rs:5048–5155)

```rust
if emit_crystal { return emit_peer_crystal(...); }              // #1 wins
if let (Some, Some, Some) = (song, dance_with, deploy_to) { ... } // #2
if let (Some, Some) = (song, dance_with) { ... }                  // #3
if let Some(song_path) = song { ... }                             // #4
if emit_diff { return emit_peer_beam_diff(...); }                 // #5
if integrate_diff { return integrate_peer_beam_diff(...); }       // #6
if fate_select { ... }                                            // #7 (nests #8, #9)
// falls through to base envelope (hello_world / mission consumed here)
```

**Drift alarm.** Docblock at lib.rs:5010–5017 claims "`fate_select`
wins over all other flags when set (last check in dispatch cascade)."
Runtime says the OPPOSITE: `fate_select` is the LAST check, so
`--emit-crystal` beats it; `--deploy-to` + `--song` + `--dance-with`
beat it; `--song` beats it; `--emit-diff` beats it. The docblock
contradicts the code by one full session's worth of Rung ladder
additions.

## §4 — Composition matrix

Rows are dominant flags (win when both present); columns are dominated
flags (silently no-op). ✅ = compose additively; ❌ = dominant flag
silently skips dominated flag; ↔ = mutually exclusive by narrowing
guard (both must be Some).

|                     | emit-crystal | deploy-to | dance-with | song | emit-diff | integrate-diff | fate-select | from-psychohistory | with-shadow | hello-world | mission |
|---|---|---|---|---|---|---|---|---|---|---|---|
| **emit-crystal**    | —            | ❌        | ❌         | ❌   | ❌        | ❌             | ❌          | ❌                 | ❌          | ❌          | ❌      |
| **deploy-to**       | dominated    | —         | ↔req       | ↔req | ❌        | ❌             | ❌          | ❌                 | ❌          | ❌          | ❌      |
| **dance-with**      | dominated    | dom.      | —          | ↔req | ❌        | ❌             | ❌          | ❌                 | ❌          | ❌          | ❌      |
| **song**            | dominated    | dom.      | dom.       | —    | ❌        | ❌             | ❌          | ❌                 | ❌          | ❌          | ❌      |
| **emit-diff**       | dominated    | dom.      | dom.       | dom. | —         | ❌ (fires first)| ❌          | ❌                 | ❌          | ❌          | ✅ used |
| **integrate-diff**  | dominated    | dom.      | dom.       | dom. | dom.      | —              | ❌          | ❌                 | ❌          | ❌          | ❌      |
| **fate-select**     | dominated    | dom.      | dom.       | dom. | dom.      | dom.           | —           | ↕modifier          | ↕modifier    | ❌          | ✅ used |
| **from-psychohistory** | (no-op unless fate-select) | — | — | — | — | — | modifier of | — | ↕modifier | (unreachable) | (unreachable) |
| **with-shadow**     | (no-op unless fate-select + from-psychohistory) | — | — | — | — | — | modifier | modifier | — | (unreachable) | (unreachable) |
| **hello-world**     | dominated    | dom.      | dom.       | dom. | dom.      | dom.           | dom.        | dom.               | dom.        | —           | ✅ used |
| **mission**         | dominated    | dom.      | dom.       | dom. | ✅used    | dominated      | ✅used      | —                  | —           | ✅used      | —       |

**Reading:** only three additive relationships in the entire 11-flag
surface — `mission` composes with `emit-diff`, `fate-select`, and
`hello-world`. Every other pair is either dominated by an earlier
early-return, or a modifier that silently no-ops without its parent.

### 4.1 Silent-no-op findings

- `--from-psychohistory` **alone** = silently no-op. No warning.
- `--with-shadow` **alone** or **without --from-psychohistory** =
  silently no-op. No warning.
- `--dance-with` **without --song** = silently no-op (guard is
  `if let (Some, Some) = (song, dance_with)`). No warning.
- `--deploy-to` **without --song and --dance-with** = silently no-op
  (three-way narrowing). No warning.
- `--mission` alone with **no mode flag** = falls through to the base
  envelope path but the mission is only surfaced when `--hello-world`
  is also set (envelope emits `"mission": ...` field only in the
  JSON envelope path at lib.rs:5389+).

**Drift.** Silent-no-op-on-missing-predecessor is a substrate
mis-shape. The flags encode preconditions that live in the runtime
match-tree; the substrate has no way to reject `--dance-with` without
`--song` at cli-block validation altitude, because `flag dance_with:
~f` says nothing about the requirement.

## §5 — MCP tool surface audit

Location: `bootstrap/src/mcp.rs::tools_list_result` (mcp.rs:95–248).
Eight tools per Mara iter-15 schema reconciliation.

| Tool | inputSchema properties | Required | Dispatch → CLI (mcp.rs:411–589) |
|---|---|---|---|
| `mirror_compile` | `file` | `file` | `mirror compile <file>` |
| `mirror_craft` | `target`, `target_kind` (enum), `reflect` | `target` | `mirror craft <t> [--target-kind K] [--reflect]` |
| `mirror_kintsugi` | `file`, `liquid`, `shatter` | `file` | `mirror kintsugi --ci --out @data/json <f> [--liquid] [--shatter N]` — **always adds `--ci --out @data/json`** (Tick 7 shatter fold `ffba2a7`). `--liquid` is runtime-declared at MCP schema but grep of lib.rs turns up **no `--liquid` flag** — it's a substrate-alias runtime-parsed elsewhere or a dead schema entry. |
| `mirror_init` | `path`, `install_hooks` | `path` | `mirror init <p> [--install-hooks]` |
| `mirror_recall` | `spec_dir` | `spec_dir` | `mirror recall <d>` |
| `mirror_peer_beam` | **11 properties** (mcp.rs:157–174) | `peer_home` | `mirror peer beam <p> [...11 flags...]` |
| `mirror_beam` | `mission` | `mission` | `mirror beam --mission <m>` — **the CLI shape is `mirror beam <mission>` positional (lib.rs:3374)**, but the MCP dispatch uses `--mission` flag (mcp.rs:571). Runtime lib.rs:3374 uses positional. Drift. |
| `mirror_spawn` (DEPRECATED) | `peer_home`, `hello_world`, `mission` | `peer_home` | `mirror spawn <p> [--hello-world] [--mission <m>]` |

### 5.1 `mirror_peer_beam` inputSchema drift

11 properties on `mirror_peer_beam`:
`peer_home` (req), `hello_world`, `mission`, `fate_select`,
`from_psychohistory`, `with_shadow`, `emit_diff`, `integrate_diff`,
`song`, `dance_with`, `deploy_to`.

Missing from MCP schema vs the CLI runtime:

- `emit_crystal` — **absent from MCP schema** (mcp.rs:157–174; the
  Rung 6' addition landed in `cmd_peer_beam` but was not extended to
  `mirror_peer_beam` inputSchema). MCP consumers cannot invoke the
  substrate-inversion path.
- Docblock at mcp.rs:174 (`song` description) still references
  "Kuramoto order parameter" and Mara `417ec25`; there is no docblock
  reference to Rung 6'.

Missing from CLI vs MCP schema: none. MCP is missing one flag; CLI
has strict superset.

### 5.2 MCP-specific naming drift

- MCP uses `snake_case` argument names (`fate_select`, `hello_world`);
  CLI uses `--kebab-case` (`--fate-select`, `--hello-world`).
  Mechanical (mcp.rs:501 pushes `--fate-select` when `b("fate_select")`
  is true). No conceptual drift, but every new flag doubles the sync-
  point count.
- MCP `mirror_kintsugi` accepts `liquid: bool` but CLI has **no
  `--liquid` flag anywhere in lib.rs**. Grep `--liquid` in
  `bootstrap/src/lib.rs` = 0 matches. Schema drift; dead property.

## §6 — Duplication + drift findings

### 6.1 Alias inventory (substrate-honest vs runtime-tolerated names)

| Alias pair | Substrate-honest | Runtime alias | Cite |
|---|---|---|---|
| `--mission` / `--task` | `--mission` | `--task` (both accepted) | lib.rs:3193, 3260, 3376 |
| `--target-kind` / `--target` (on `craft`) | `--target-kind` | `--target` | lib.rs:3000–3020 |
| `--target` (on `shatter`) | `--target` (substrate-ref selector) | none | lib.rs:2955 |
| `mirror spawn` / `mirror peer beam` | `mirror peer beam` | `mirror spawn` (with stderr notice) | lib.rs:3170 |
| `mirror_spawn` (MCP) / `mirror_peer_beam` | `mirror_peer_beam` | `mirror_spawn` (silent) | mcp.rs:222–240 |

The `--target` collision (`craft`'s target-kind alias vs `shatter`'s
substrate-ref selector) is disambiguated by the runtime at lib.rs:2953
(`subcommand_is_shatter = args[1] == "shatter"`). Substrate-decl
altitude accepts both because the mirror.spec cli-block declares
`flag target` on `shatter` (line 110) AND `flag target_kind` on `craft`
(line 125) as independent — the shared `--target` alias only exists at
runtime for `craft`.

### 6.2 Runtime-only flag drift (§3 rerun)

Five flags exist **only** at runtime with no substrate-decl:

- `--fate-select`
- `--from-psychohistory`
- `--with-shadow`
- `--emit-diff`
- `--integrate-diff`

The mirror.spec cli-block (mirror.spec:212–283) has no `flag
fate_select`, no `flag from_psychohistory`, etc. All five were added
to `cmd_peer_beam` (Reed `4b2ef3c` for diff-direction, `07ac55a` for
shadow) without a corresponding `mirror.spec` update. This means
`mirror craft --target-kind crystal ./` (settle the spec, emit
crystal OID) would not surface these flags in any @mirror/lens/cli
introspection.

### 6.3 Contradiction: docblock claims `fate-select` wins; runtime says
`emit-crystal` wins

The `cmd_peer_beam` docblock (lib.rs:5010–5017) says:

> "The `fate_select` param routes to @optics/lens/features.get +
> Fate::excited().resolve at Rust runtime. Emits selected Model +
> mapped prism-op via bundle-tower binding. **Wins over all other
> flags when set (last check in dispatch cascade).**"

Runtime order (lib.rs:5049): `emit_crystal` fires FIRST; `fate_select`
is the sixth and last non-hello_world dispatch. When both are set
together, `emit-crystal` wins. When `--emit-diff` and `--integrate-
diff` are BOTH set, the MCP schema comment at mcp.rs:157 says
"integrate wins" but the runtime (lib.rs:5120 vs 5137) has
`emit_diff` firing first — so `emit_diff` wins. **Two separate
docblocks lying about dispatch order.**

### 6.4 MCP-CLI capability gap

- `--emit-crystal` (CLI, Rung 6') — no MCP inputSchema entry.
- `--liquid` (MCP mirror_kintsugi) — no CLI runtime match arm.

### 6.5 Duplicated dispatch code

Flag-parsing for `--hello-world` + `--mission` + `--emit-diff` +
`--integrate-diff` + `--fate-select` + `--from-psychohistory` +
`--with-shadow` + `--song` + `--dance-with` + `--deploy-to` +
`--emit-crystal` is duplicated verbatim across THREE call sites:

- `spawn` alias arm (lib.rs:3180–3226)
- `peer beam` recursive arm (lib.rs:3260–3323)
- `beam` anonymous arm (lib.rs:3376–3397)

That's 44 lines of `args.iter().position(...)` triples, three times.
Any new flag added to `peer beam` requires three synchronized edits
plus the MCP schema plus mirror.spec. Adding `--emit-crystal` (Rung
6') updated all three but forgot MCP.

## §7 — Candidate command collapses

Alex's direction says "what does THIS command do, then THIS, HOW DO
THESE COMPOSE." The candidate collapses below name substrate-honest
verb-collapses that would eliminate the mode-switching-in-flag-guise
drift. NOT recommending; auditing.

### 7.1 `--song <s>` → `mirror peer sing <s>` (or `mirror peer beam sing`)

**Substrate-honest case:** @song/beat IS a species of atomic-execution
unit (mirror.spec:229 docblock cites `shards/song/beat.mirror` sixth
species mint). "Sing" IS the peer's temporal-frame execution per
@song/movement.enter semantics named at shards/mirror/peer/beam.mirror.
The current flag-guise says "peer beam PLUS song runtime dispatch";
substrate-honest says "sing IS a peer verb — it's what the peer DOES
when a song is present." Runtime dispatch is already an early-return
(lib.rs:5110), so structurally it's already a separate verb.

**Preserves-flag case:** if the song is optional context (peer can
beam without singing), keeping `--song` as opt-in modifier of `peer
beam` preserves the "peer beam with song" reading. But the runtime
dispatch is either/or (song → early-return; no-song → base envelope),
so it's not composed with the base path.

**Substrate-decl direction:** shards/mirror/peer/beam.mirror already
names the beam action; substrate-honest is `command peer { command
sing { arg peer_home: ~d; arg song: ~f } }`.

### 7.2 `--dance-with <h2>` → `mirror peer dance <h1> <h2>`

**Substrate-honest case:** @dance IS N-peer coordination. The runtime
signature is symmetric in `peer_home_1` and `peer_home_2` (dance.rs:52
`execute_dance(peer_home_1, spec_path_1, peer_home_2, spec_path_2,
song_path, ctx)`). Treating one peer as "primary" and the second as a
"flag argument" is asymmetric-in-flag-guise for a symmetric
substrate operation. `mirror peer dance <h1> <h2> --song <s>` reads
the substrate operation directly.

The Rung 4 docblock (mirror.spec:232–246) says "multi-peer @dance
coupling on shared beat" — the multi-peer aspect is the load-bearing
structure, not a flag on a single-peer operation. **This is the
strongest collapse candidate on the audit.**

### 7.3 `--deploy-to <target>` → `mirror deploy <spec> --target <t>`

**Substrate-honest case:** @spectral/garden/deployment IS its own
authority (mirror.spec:249 docblock names @spectral/garden +
@spectral/garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/
beat). Deployment isn't a "peer beam mode" — it's a top-level substrate
motion that composes over a completed dance. The current runtime
narrowing (`if let (Some, Some, Some) = (song, dance_with, deploy_to)`)
encodes the composition dependency in flag-precondition guise; a
`mirror deploy` verb that takes the dance output as its input would
make the composition explicit.

**Substrate-decl direction:** top-level `command deploy { arg
mission: ~f; flag target: ~f; flag from: dance_state }` or similar.

### 7.4 `--emit-crystal` → `mirror peer commit` (or peer's default)

**Substrate-honest case:** Rung 6.1c (Reed `90019c4`) discharge closed
`commit_as_fold` so the peer branch HEAD IS a real git commit. The
substrate closure claim at CURRENT.md:112–116 says "peer inference IS
mirror-store-bounded" — commit IS materialization, not an opt-in
emission mode. If commit-materialization IS the substrate-honest
default of `peer beam` (per Recognition #43 + #55), then `--emit-
crystal` should NOT be a flag; it should be default behavior. The
"stdout envelope" path becomes the opt-in `--dry-run` alternative.

**Substrate-decl direction:** flip the default. `peer beam` emits
crystal on `refs/mirror/peer/<uuid>/HEAD` by default; `--emit-envelope`
opts out to the pre-Rung-6' stdout shape.

### 7.5 `--fate-select` / `--from-psychohistory` / `--with-shadow`
→ substrate default OR nested command

**Substrate-honest case:** MEMORY note
`feedback_cli_subcommand_nesting_is_geometric_ground_truth.md`
(Alex 2026-07-XX) says sub-commands aren't a UX choice — they're
substrate structure. Recognition #58 (per CURRENT.md context) names
fate as the substrate's inference mechanism. If Fate IS how a peer
infers, then `--fate-select` is opt-in for a behavior that IS the
substrate. Two collapse options:

- **Default it.** `mirror peer beam <h>` fires fate optical inference
  as substrate-honest default; the base envelope (current no-flag
  behavior) becomes `--no-inference` or is retired.
- **Nest it.** `mirror peer beam infer <h> [--bounded-by
  psychohistory] [--with-shadow]` reads the two modifier flags as
  arguments to the `infer` sub-verb. `--from-psychohistory` and
  `--with-shadow`'s modifier-of-fate-select relationship becomes
  argument-of-infer.

Silent-no-op findings §4.1 dissolve either way.

### 7.6 `--emit-diff` / `--integrate-diff` → `mirror peer diff get/put`

**Substrate-honest case:** the docblock at lib.rs:5006–5008 explicitly
names "@optics/lens/diff.get direction" and "@optics/lens/diff.put
direction" — this IS the Foster get/put roundtrip at optical lens
altitude. `mirror peer diff get <peer_home> [--mission <m>]` and
`mirror peer diff put <peer_home>` reads the substrate operation
directly. The "mutually exclusive" property (emit_diff wins in runtime,
integrate_diff wins in docblock) disappears — they're separate verbs.

### 7.7 `--hello-world` (envelope shape switch) → keep as flag

**Preserves-flag case (substrate-honest to KEEP as flag):**
`--hello-world` switches text envelope ↔ JSON envelope at the @io
boundary. This IS an @io-boundary transport-frame choice; it composes
with any peer beam mode. Every early-return branch already respects
the base envelope shape internally, so `--hello-world`'s scope is
narrow. Keep as flag.

Better name candidate: `--json` (transport-honest) or `--envelope
<json|text>` (substrate-honest as a lens-projection choice). "hello-
world" as a verb-name feels stub-testing-legacy.

## §8 — Session shape analysis

### 8.1 Current shape

`mirror peer beam` is a **single, atomic CLI invocation** that either:

- emits a crystal on a git ref and exits (`--emit-crystal`), OR
- emits an envelope (JSON or text) to stdout and exits, OR
- writes a diff to peer_home/.bauchladen/ and exits (`--integrate-diff`), OR
- emits a diff to stdout and exits (`--emit-diff`), OR
- computes fate inference over features + emits fate envelope, OR
- fires a dance / deployment / song and emits per-Rung envelope.

**Every invocation is one shot.** There is no session state. Consecutive
invocations against the same `<peer_home>` re-read `mirror.spec` from
disk each time.

### 8.2 Substrate-honest session shape

A substrate-honest peer session per Recognition #43 + Rung 6.1c would
be:

```
mirror peer beam <h>        # inference tick; emits crystal on <h>/.git/refs/mirror/peer/<uuid>/HEAD
mirror peer sing <h> <song>  # temporal-frame execution
mirror peer dance <h1> <h2> --song <s>  # coherence tick
mirror deploy <spec> --from-dance <dance_oid>  # materialization
```

Each verb IS a boundary — a real commit on a real ref. Composition IS
git DAG traversal on `refs/mirror/peer/<uuid>/*`. The MCP loop becomes
"tools/call sequence with crystal OIDs threading through as
arguments."

This shape isn't a proposal; it's the ceiling of what the current
substrate-decls (Recognition #43 mirror-IS-content-addressed +
Recognition #55 commit_as_fold materialization + Recognition #80
gauge-bounded interior) admit if collapsed.

### 8.3 Natural boundary verbs (audit-observed)

From the runtime dispatch cascade + substrate-decl families:

- **`peer beam`** — inference primitive (Recognition #58 fate)
- **`peer sing`** (currently `peer beam --song`) — temporal-frame
- **`peer dance`** (currently `peer beam --song --dance-with`) —
  coherence
- **`deploy`** (currently `peer beam --song --dance-with --deploy-to`)
  — materialization at @spectral/garden
- **`peer commit`** or default of `peer beam` (currently `peer beam
  --emit-crystal`) — Rung 6.1c materialization

Five verbs. Alex's direction: "what does THIS command do, then THIS,
HOW DO THESE COMPOSE." The 11-flag surface hides a five-verb pipeline.

## §9 — MCP loop shape analysis

### 9.1 Current MCP invocation

Per mcp.rs:411 `dispatch_tool_call`, each `tools/call` invokes:

1. Build argv from tool name + args → `["peer", "beam", <h>, "--song",
   <s>, "--dance-with", <h2>, ...]`
2. Call `run_mirror(&argv, ctx)` (mcp.rs:389) which invokes
   `kintsugi_main_in(...)` in-process.
3. Return (stdout+stderr concatenated text, is_error).

**Composition primitive at MCP altitude:** each `tools/call` is a
process-level dispatch (in-process via `kintsugi_main_in`, but still a
one-shot). There is no session state on the MCP server side. Agent
callers must re-supply all args on every call.

### 9.2 Missing "session" primitive

The MCP schema has no tool for:

- Reading a peer's current crystal OID (`refs/mirror/peer/<uuid>/HEAD`
  after Rung 6.1c). Consumer would need to `git rev-parse` externally.
- Enumerating peer refs / peer identities on a home.
- Chaining a Rung sequence in one call (peer beam → dance → deploy)
  as an atomic-DAG operation.

An agent orchestrating a multi-Rung workflow currently issues N
independent `tools/call` requests, each rebuilding CLI argv, each
re-reading `mirror.spec`, each not knowing about prior invocations.

### 9.3 Natural MCP loops (audit-observed)

Two shapes could compose the current one-shot tools:

- **Session-as-argument-threading:** each `tools/call` returns a
  `session_oid` (the crystal OID emitted at Rung 6.1c); subsequent
  `tools/call` invocations pass `session_oid` as an input to compose
  over prior state. The MCP server stays stateless; state lives in the
  `refs/mirror/peer/<uuid>/*` git DAG.
- **Batch tool:** `mirror_peer_session` composes N `tools/call`s in
  one invocation, returning the final `session_oid` after the whole
  DAG runs. This is heavier at MCP altitude but matches Alex's "what's
  the MCP loop" question directly — the loop is the substrate-honest
  session shape from §8.2.

The current schema chose **neither**. Each `mirror_peer_beam` call is
opaque and doesn't participate in a session even after Rung 6.1c
made crystal OIDs materialize.

## §10 — Recommendation to Mara

Load-bearing findings that inform Mara's canonical CLI/MCP composition
spec:

**§10.1 — Substrate-decl vs runtime drift is the ground truth to close.**
Five runtime flags (`--fate-select`, `--from-psychohistory`,
`--with-shadow`, `--emit-diff`, `--integrate-diff`) live in
`cmd_peer_beam` without a `mirror.spec` cli-block declaration. The
substrate-honest surface is either (a) declare them all, (b) collapse
them into verbs per §7.5 + §7.6, or (c) collapse `cmd_peer_beam`'s
mode-switching cascade into six separate `cmd_*` functions dispatched
at `match args[1]`. Any spec proposal should pick one.

**§10.2 — Mode-switching flags are verbs in flag-guise.**
Six of eleven `peer beam` flags are early-return modes (§3.1). They
do not compose additively; they compete for dispatch. The substrate-
honest reading is "these are separate verbs sharing a peer_home
argument." §7.1–§7.6 name the collapse candidates. Strongest
candidates: §7.2 (`peer dance`), §7.3 (`deploy`), §7.6 (`peer diff
get/put`) — all three have symmetric-signature substrate-decls that
break under the flag-encoding.

**§10.3 — Silent-no-op is a spec-shape violation.**
`--dance-with` without `--song`, `--with-shadow` without
`--from-psychohistory` etc. silently no-op. The cli-block grammar
(`shards/mirror/lens/cli.mirror:157–174`) has no `requires(flag_a,
flag_b)` predicate at flag-declaration altitude. If the mode-switching
collapse in §7 doesn't land, cli-block needs a flag-precondition
predicate before more Rung ladders climb.

**§10.4 — Rung 6.1c inverted the default; the flag should follow.**
Recognition #43 + Recognition #55 + Rung 6.1c (Reed `90019c4`) mean
crystal-on-git-ref IS the substrate-honest peer materialization.
`--emit-crystal` as opt-in is backward for post-6.1c substrate.
Either flip the default (§7.4) or absorb `--emit-crystal` into the
`peer beam` command's structural semantics with no flag surface at
all. The MCP schema is missing this property entirely (mcp.rs:157–174);
that gap gets closed by removing the flag, not by adding a schema
entry.

**§10.5 — `--target` on `shatter` is substrate-ref-typed; grammar
should follow.**
The cli-block declares `flag target: str = "auto"` at mirror.spec:110
because "grammar today has no first-class `ref` value-type at flag
position" (mirror.spec:104 docblock). The runtime already validates
via `parse_substrate_ref_to_format` (lib.rs:2960). The lens grammar at
`shards/mirror/lens/cli.mirror` should mint the `ref` type as a
first-class type-vocabulary entry so this drift closes at substrate
altitude. Same lift lands `mission: ~f` → `mission: ref('song')` etc.

**§10.6 — The MCP loop question is the session-shape question.**
Alex's "what's the MCP loop" (§9.3) reduces to §8.2: does the substrate
admit a "peer session" primitive over multiple `tools/call`s? If yes,
crystal OIDs are the session tokens (Rung 6.1c already materializes
them); if no, the current one-shot shape is substrate-honest but the
verb collapses in §7 still hold. Either way, the current MCP schema
has neither pattern discharged. `mirror_peer_beam` at 11 properties
IS the flag-soup at MCP altitude; the CLI-side collapse dissolves it
without a corresponding MCP-side "session" invention.

**§10.7 — `spawn` alias + `mirror_spawn` MCP tool: retire this cycle.**
Two-tick discipline is exhausted — the substrate-honest rename to
`peer beam` landed 2026-07-08 (`4f4a257` + `b012d3f`); five days later
`spawn` still ships with a stderr deprecation notice AND an MCP tool
entry. Any CLI/MCP audit landing recommends removing both. Fault-plane
#1 (`@pack.spawn` at pack altitude, unchanged) is orthogonal.

**§10.8 — Docblocks lie; runtime dispatch is ground truth.**
Two independent docblocks contradict runtime dispatch order (§3
`fate_select` "wins," §5 MCP integrate_diff "wins"). Any spec Mara
writes should be paired with a runtime dispatch-order invariant test
in `bootstrap/tests/` so future rearrangements can't drift docblocks
out of sync silently.

---

**Grep-first inventory complete.** Every claim above cites file:line;
audit HEAD `90019c4`. The load-bearing findings (§10) are for Mara's
canonical spec; the tables (§3–§5) are the raw substrate for spec
composition. No edits proposed. No commits beyond this scout.

📝 Taut [substrate-drift:scout] [cli-mcp-audit]
