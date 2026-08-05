# Taut scout — primitives-vs-composition ground-truth at rust/ altitude (2026-08-05)

**Alex 2026-08-05 verbatim (task frame):**

> bootstrap/ is only migration source target. Not execution path. What's
> the path forward through rust/? And how can we generalize the whole io
> protocol over the math. I want the MCP to basically be served through
> the mirror geometry, read and executed by the rust. No specific mcp
> rust code, you know what I mean?

**Load-bearing memory (persisted this session):**
`feedback-rust-delivers-primitives-substrate-delivers-composition` —
**HARD RULE**: wire protocols (MCP/LSP/HTTP/etc.) are **substrate
compositions** authored as `@X/serve.mirror` shard-body compositions,
NOT rust/ modules. rust/ delivers **primitives** (@io/socket,
@data/json, apply_h::act, shard reader, @mirror/store); shard-bodies
compose them via `apply_h::act` dispatch.

**Yesterday's Reed Phase B assumption (RETIRED):** "port ~500 LOC
bootstrap `serve_loop` to `rust/src/mcp.rs`" per Mara M4 canonical
spec `docs/specs/2026-08-03-mara-rust-mcp-floor-lift-m4-canonical-spec.md`
was substrate-dishonest. The correct pattern is
**primitives-at-rust/ + composition-at-substrate**. Mara M4 spec is
now RETIRED per Alex 2026-08-05 correction; this scout re-anchors the
work along primitives-vs-composition.

---

## §1 — @io primitives at rust/ altitude (Phase 1)

### @io/fs — **LANDED-EMPIRICAL** (rust/src/phone.rs; iter 6 `b6f32ea`, 37 tests)

Landed pub(crate) functions in `rust/src/phone.rs`:

| Primitive | Signature | State |
|-----------|-----------|-------|
| `read_file` | `(&Path) -> io::Result<String>` | LANDED-EMPIRICAL |
| `write_file` | `(&Path, &str) -> io::Result<()>` | LANDED-EMPIRICAL |
| `append_to` | `(&Path, &str) -> io::Result<()>` | LANDED-EMPIRICAL |
| `mkdir_p` | `(&Path) -> io::Result<()>` | LANDED-EMPIRICAL |
| `path_exists` | `(&Path) -> bool` | LANDED-EMPIRICAL |
| `list_dir_recursive` | `(&Path) -> io::Result<Vec<WalkEntry>>` | LANDED-EMPIRICAL |
| `find_substrate_root` | `(&Path) -> PathBuf` | LANDED-EMPIRICAL |

Walker skips `.git/` + `target/` + symlinks at per-entry read. Zero
`unsafe extern "C"`. Composition anchor: `shards/io.mirror` T21
family-root; `boot/std/io/fs.mirror` (boot-altitude sufficient per
glass-wall recognition).

### @io/git — **LANDED-EMPIRICAL** (rust/src/phone.rs; iter 7 `e470d03`, 14 tests)

- `git_add(&Path, &Path) -> io::Result<()>` — LANDED-EMPIRICAL
- `git_commit_as(&Path, &Subject, &Subject, &str) -> io::Result<String>` — LANDED-EMPIRICAL (MARA doctrine Author≠Committer verified)
- `git_head_oid(&Path) -> io::Result<String>` — LANDED-EMPIRICAL
- `spawn_cargo_build(&Path, &str) -> io::Result<ExitStatus>` — LANDED-EMPIRICAL (task #226 landing)

SSH signing stays operator-default. Tempdir-repo scaffold isolates
from operator SSH signing + commit-msg hook per-repo (never global).

### @io/socket — **LANDED-EMPIRICAL** (rust/src/phone.rs; iter 9 `0f2b3bf`, 11 tests)

**CRITICAL for MCP composition.** Landed Unix-domain-socket primitives:

- `open_peer_socket(peer_home: &str) -> io::Result<PeerSocketConnection>` — LANDED-EMPIRICAL
- `bind_peer_socket(peer_home: &str) -> io::Result<PeerSocketListener>` — LANDED-EMPIRICAL
- `PeerSocketConnection { stream: UnixStream }` carrier — LANDED
- `PeerSocketListener { listener: UnixListener }` carrier — LANDED

**Scope-restricted to UnixStream/UnixListener over `<peer_home>/.sock`
convention** per @peer/persistence discipline (M8 landing). TCP shape
(`TcpListener`/`TcpStream`) NOT present at rust/ altitude — grep of
`rust/**/*.rs` for `TcpListener`/`TcpStream` returns zero matches.

Composition anchor: `boot/std/io/socket.mirror:1-105` — permanent @io
resident per glass-wall recognition (blocking syscall irreducibly
non-mirror); two opaque handle types (`connection`, `listener`);
actions `read_bytes` / `write_bytes` / `close`.

**Gap sub-primitives**: `accept()` on the listener + `read`/`write`
over the connection are NOT exposed at phone.rs public surface —
`PeerSocketConnection.stream` field is `pub(crate)` (raw UnixStream
access requires call-site to import std::os::unix::net directly).

### @data/json — **LANDED-EMPIRICAL** (via serde_json dep-chain, spectral crate; not phone.rs)

**CRITICAL for MCP composition.** Grep of `rust/**/*.rs` for
`serde_json|Value|to_string|from_str` shows presence at:

- `rust/fractal/src/{crystal,singularity,subject,witnessed}.rs` (11 matches; carrier serialization)
- `rust/matrix/src/{book,lib,void}.rs` (9 matches; @<name> registry serialization)
- `rust/roomba/src/mend.rs` (6 matches; bilateral corpus emission)
- `rust/spectral/src/{lib,liquid}.rs` (115 matches; property/pillar emission)
- `rust/src/{compile,main,phone}.rs` (79 matches)

**BUT**: no rust/ altitude wrapper exposing `@data/json.parse` /
`@data/json.emit` as bilateral-predicate-dispatchable primitives.
Each consumer imports `serde_json::{json, Value, from_str, to_string}`
directly. Composition anchor `boot/std/data/json.mirror` (272 bytes,
2 actions: `parse`/`emit`) is LANDED-SPEC-ONLY with `\`-blocked bodies.

### @io/stdio — **LANDED-EMPIRICAL** (rust/src/phone.rs; iter 8 `4db932d`, 20 tests)

**CRITICAL for MCP transport shape.** Landed as `@io/bytes` stdio:

- `read_frame_from<R: BufRead>(&mut R) -> io::Result<Vec<u8>>` — LANDED-EMPIRICAL (newline-delimited JSON-RPC 2.0)
- `write_frame_to<W: Write>(&mut W, &[u8]) -> io::Result<()>` — LANDED-EMPIRICAL
- `read_stdin_frame() -> io::Result<Vec<u8>>` — LANDED (marked `#[allow(dead_code)]`; no consumer yet)
- `write_stdout_frame(&[u8]) -> io::Result<()>` — LANDED (marked `#[allow(dead_code)]`; no consumer yet)

Generic over Read/Write so parsing tests in isolation with
`Cursor<&[u8]>` + `Vec<u8>`. `pub(crate)` visibility (crate-internal).

### Phase 1 verdict — @io primitives at rust/ altitude

**~90% LANDED-EMPIRICAL.** phone.rs is production-ready across
@io/fs + @io/git + @io/socket + @io/stdio surfaces. Docblock
declares "post ship arc 2026-07-21 production-ready" + "no other
outstanding forward-promises at phone.rs altitude". The **only
substantive gap** is a rust/-altitude `@data/json` wrapper exposing
serde_json under a substrate-composable primitive surface (the
implementation exists via serde_json; the substrate-honest wrapper
lifting it into apply_h::act dispatchable form does not).

---

## §2 — apply_h::act 7-combinator surface (Phase 2)

### bootstrap altitude — **LANDED-EMPIRICAL** (bootstrap/src/apply_h.rs; 81.4KB, Arc-1 Tick 1.3 GREEN)

Per Mara canonical spec `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`:

| Side | Combinator | State |
|------|------------|-------|
| A | `section` | LANDED-EMPIRICAL |
| A | `fold` | LANDED-EMPIRICAL |
| A | `act` | LANDED-EMPIRICAL (bilateral-predicate dispatch path) |
| H | `settle` | LANDED (surface types + MVP) |
| H | `crystallize` | LANDED (surface types + MVP) |
| D | `coboundary` | LANDED (surface types + MVP) |
| D | `utter` | LANDED (surface types + MVP) |

**First empirical firing (sbec 0 → >0):** smoke test
`evaluator_shard_body_dispatch_smoke` discharges `Pass` for
`@subject/visibility/public.consent_scope_universal`. `act` recognizes
shard action ref → byte-checks argument oid against sentinel → returns
Verdict.

**Reflective bilateral corpus loader** (`load_bilateral_corpus`,
`bilateral_corpus`, `discharge`): LANDED-EMPIRICAL at bootstrap
altitude; line-scans `shards/**/*.mirror` for `bilateral <name>
{ sentinel "..." arity <n> require <ref> }` blocks; process-cached via
`OnceLock<HashMap<String, BilateralDecl>>`.

### rust/ altitude — **GAP** (grep `rust/**/*.rs` for `apply_h|::act|combinator`: zero matches)

The 7-combinator surface **is not present at rust/ altitude in any
form** — not stub, not skeleton, not partial. Yesterday's Taut scout
`64e8d60` §Phase 2 verdict identical (LANDED-SPEC + LANDED-PARTIAL at
bootstrap altitude only). Reed's `roomba/src/mend.rs` (40.3KB) is a
**specialized bilateral-arm-collapse dispatch**, NOT the general
apply_h::act reflective evaluator. Task #159 (Wire six-step loop
through apply_h::act) + Task #160 (Empirical one-fracture autopoietic
round-trip) remain [pending].

**Design finding (answered here, [ALEX-Q candidate]):** The smallest
apply_h::act subset needed to dispatch `@mcp/tool`-annotated shard
actions is the bilateral-predicate `act` path + `load_bilateral_corpus`
+ `discharge`. The other 6 combinators (section/fold/settle/crystallize/
coboundary/utter) are NOT structurally required for MCP tool-dispatch;
they are required for `mirror kintsugi` settle-loop + `mirror compile`
saga-chain-of-Crystals composition. **MCP composition can fire with
`act` + `bilateral_corpus` + `discharge` alone** if the substrate
tool-dispatch shape is bilateral-predicate-only.

### Phase 2 verdict

**LANDED-EMPIRICAL bootstrap only; GAP at rust/ altitude.** The
smallest primitives are `act` (dispatch) + `load_bilateral_corpus`
(shard reader) + `discharge` (verdict). Full 7-combinator surface is
NOT structurally required for MCP composition; it IS required for
compile.rs saga-chain composition (which is orthogonal to MCP).

---

## §3 — Shard reader / mirror.spec cli-block reflection (Phase 3)

### shard reader (`.mirror` file parsing) — **PARTIAL** at rust/ altitude

Landed rust/-altitude primitives:

- `spectral::shard_paths() -> Vec<String>` — LANDED-EMPIRICAL (rust/spectral/src/lib.rs; walks `shards/**/*.mirror`; sorted; tested)
- `roomba::mend::load_bilateral_corpus(&Path) -> HashMap<String, BilateralDecl>` — LANDED-EMPIRICAL (rust/roomba/src/mend.rs; extracts `bilateral { sentinel arity require }` blocks from `shards/**/*.mirror`)
- `spectral::liquid::extract_properties` + `extract_spec_properties` — LANDED (per rust/spectral/src/liquid.rs docblock: "Read bilateral property declarations from `mirror.spec` + shard...")

### mirror.spec cli-block reflection — **GAP** at rust/ altitude

Mara §5.2 M2 milestone ("reflective cli-block reading from
mirror.spec"): rust/src/main.rs docblock states verbatim —

> `--help` prints the 10-verb list from `mirror.spec` cli-block
> **HARDCODED at M0**. Reflective derivation from
> `shards/**/*.mirror` + `mirror.spec` lands at M2 (Mara §2.2).

`rust/tests/red_spec_claims.rs` verifies: **"main.rs claims to read
kintsugi.roomba cascade catalog from mirror.spec (Mara §7.2) but
source does NOT read mirror.spec at all."** — RED test discharging
the gap.

The 12-entry `const VERBS: &[(&str, &str)]` at `rust/src/main.rs` is
hardcoded. No rust/-altitude parser for mirror.spec's cli-block
grammar exists.

### Grammar walk primitive — **GAP** at rust/ altitude

Grep `grammar_walk|@mirror/spectral\.gestalt` at rust/ altitude:
zero matches. Grammar walk is used only in bootstrap/ altitude (via
`bootstrap/src/spectral.rs::Fold5` + AST walker).

### Phase 3 verdict

**Shard reader: 50% LANDED** — bilateral-block reader
(`load_bilateral_corpus`) + `shard_paths()` enumerator both landed at
rust/ altitude. **mirror.spec cli-block reflection: GAP.** **Grammar
walk primitive: GAP** (no AST walker at rust/ altitude).

For MCP composition, only the bilateral-block reader is
strictly required (tool-dispatch shape maps 1:1 to bilateral-predicate
dispatch); mirror.spec cli-block reflection is required for the
`tools/list` reflective advertisement (Mara §4.3 grammar walk =
tools list); grammar walk is required for the reflective
`tools_reflects_cli_block` bilateral-predicate contract per
`boot/std/mcp.mirror:44-53`.

---

## §4 — @mcp/serve substrate composition state (Phase 4)

### Substrate-decl'd MCP grammar — three altitudes

**boot altitude** — `boot/std/mcp.mirror` (6.6KB, 2026-07-12; Mara Tick 6 closure):

```mirror
grammar @mcp {
  type request = { method: text, params: json, id: json }
  type response = { result: json, id: json }

  serve -> imperfect {
    @io.read(stdin) |> @data/json.parse |> dispatch |> @data/json.emit |> @io.write(stdout)
  }

  dispatch(request) -> response { \ }  # body-blocked
  tools -> json { \ }                  # body-blocked
}
```

**Three bilateral-predicate contracts declared:**

1. `dispatches_to_cli_block` — every incoming request.method resolves to a mirror.spec cli-block command
2. `tools_reflects_cli_block` — the tool listing synthesizes from mirror.spec's cli-block (NOT a hardcoded dispatch table)
3. `frame_relativity` — inherited from `shard`; observer's shard frame carried in each response

**STATE: LANDED-SPEC-ONLY with substrate-decl closure at grammar
altitude.** The `serve` action body IS composed — the pipeline
`@io.read(stdin) |> @data/json.parse |> dispatch |> @data/json.emit |> @io.write(stdout)`
is present at boot altitude. The `dispatch` + `tools` action bodies
are `\`-blocked (obligation to be discharged by Rust runtime).

**lens altitude** — `shards/mirror/lens/mcp.mirror` (2.4KB, 2026-06-06):
LANDED-SPEC-ONLY. Declares `@mirror/lens/mcp` prism + `tool(name, args)` + `dispatch(call)` actions with `\`-blocked bodies.

**gen_prism altitude** — `shards/spectral/gen_prism/mcp_session.mirror` (28.8KB, 2026-07-12; Reed M1 TICK 1 `e8378ca`):
LANDED-SPEC (species declaration; state machine substrate); Rust
realization at bootstrap altitude.

### shards/mcp/ tree — **DOES NOT EXIST**

No `shards/mcp.mirror` or `shards/mcp/` directory. MCP lives at three
substrate altitudes only (boot / mirror-lens / spectral-gen_prism);
the family-root `@mcp` is boot-altitude at `boot/std/mcp.mirror` per
Taut recognition #R-mcp-is-composition-not-family-root.

### `@mcp/serve.mirror` composition shard — **GAP** (does not exist)

Grep for `serve\s*->|@mcp\.serve|grammar\s+@mcp` across `shards/**/*.mirror`:
zero shard-body composition mints beyond references at `shards/mirror/spectral/portal.mirror`
+ `shards/reflection.mirror`. **The `@mcp/serve.mirror` shard-body
composition Alex 2026-08-05 named as the substrate answer DOES NOT
YET EXIST.** No shard mint carries the composition body pattern:

```mirror
# hypothetical @mcp/serve.mirror composition shape (not yet minted)
serve -> imperfect {
  @io/socket.listen(...) |> @io/socket.accept |> loop {
    @io/stdio.read_frame |> @data/json.parse |> apply_h.act(dispatch) |> @data/json.emit |> @io/stdio.write_frame
  }
}
```

### `@mcp/tool` annotation — **LANDED-SPEC-ONLY**

Substrate-decl at `shards/mirror/lens/mcp.mirror` (`tool(name, args: ref) -> mcp`). Not a first-class grammar annotation like `@code/rust` per `docs/specs/lsp-and-mcp.md`; consumers `\`-block.

### `@mcp.tools -> json` action (tools-list emitter) — **GAP**

Substrate-decl at `boot/std/mcp.mirror:76` (`tools -> json { \ }`). Rust runtime realization at bootstrap altitude only (`bootstrap/src/mcp.rs::tools_list_result`; hardcoded 12-entry list, byte-parity with `bin/mirror-mcp` bash wrapper). rust/ altitude: GAP.

### `@mcp.dispatch(request) -> response` action — **GAP**

Same pattern. Bootstrap: `bootstrap/src/mcp.rs::dispatch_tool_call` (hardcoded 12-arm match). rust/: GAP.

### Phase 4 verdict

**LANDED-SPEC-ONLY-with-grammar-closure.** Three substrate altitudes
(boot / lens / gen_prism) declare `@mcp` substrate with three
bilateral-predicate contracts. `serve` action body IS composed at boot
altitude via the pipeline. **`dispatch` + `tools` bodies are
`\`-blocked at substrate; rust/-altitude discharge does NOT exist.**
No `@mcp/serve.mirror` composition shard has been minted; the wire
protocol composition Alex 2026-08-05 named lives at boot-altitude
grammar declaration only.

---

## §5 — @mirror/store six-op wire at rust/ altitude (Phase 5)

### Substrate-decl surface — LANDED (family-root)

`shards/mirror/store.mirror` (46.5KB, 2026-07-17): six-op canonical
surface declared with all bodies `\`-obligation-blocked at
family-root. Per yesterday's Taut scout `64e8d60`:

| Op | Substrate-decl state | rust/ realization | bootstrap realization |
|----|---------------------|-------------------|----------------------|
| `read(oid)` | LANDED-SPEC-ONLY | **GAP** | LANDED (bootstrap/src/store_branch.rs) |
| `write(crystal) -> oid` | LANDED-SPEC-ONLY | **GAP** | LANDED (bootstrap/src/crystallize.rs) |
| `exists(oid) -> bool` | LANDED-SPEC-ONLY | **GAP** | LANDED (bootstrap/src/crystallize.rs) |
| `walk(root)` | LANDED-SPEC-ONLY | **GAP** | LANDED |
| `impacted_by` | LANDED-SPEC-ONLY | **GAP** | LANDED |
| `query` | LANDED-SPEC-ONLY | **GAP** | LANDED (bootstrap altitude, 2026-07-17) |

### Progress since yesterday's `64e8d60` scout — **ZERO REGRESSION, ZERO PROGRESS**

Grep verification: no new rust/-altitude emitters for the six-op wire
surface have landed since 2026-08-03. The DAG/hash primitives fire at
rust/fractal/ altitude (`Crystal<T>`, `Blake3Oid`, `compose_oid`) but
the wire surface (read/write/exists/walk/impacted_by/query) is
bootstrap-only.

### GC primitives — **LANDED-SPEC-ONLY** (all altitudes)

`walk_dangling` / `mark_unreachable` / `prune` declared at family-root
with `\`-blocked bodies per math §2-3. No implementation at any
altitude.

### Phase 5 verdict

**Wire surface: GAP at rust/ altitude across all six ops.** The
Apache-2.0 rock-solid floor discipline per `docs/specs/mcp-spec-song-collapse.md`
§11 is intact but not yet lifted to rust/. Pheromone-crystal deposit
(Reed's `mirror roomba --vacuum=` at rust/ altitude) uses `phone.rs::append_to`
+ `phone.rs::git_commit_as` (direct @io) rather than composing through
`@mirror/store.write(crystal) -> oid` — because that primitive doesn't
exist at rust/ altitude yet.

---

## §6 — Synthesis (primitives-vs-composition landing table)

**Altitude legend:**
- **bootstrap** — legacy execution path; DEAD per Alex 2026-07-22
- **rust/** — terminal FLOOR; substrate execution target
- **substrate** — `.mirror` shard files (boot/std/, shards/)

### Primitives (rust/ altitude delivery target)

| Primitive | rust/ | substrate | bootstrap | Notes |
|-----------|-------|-----------|-----------|-------|
| @io/fs (read/write/append/exists/walk) | **LANDED-EMPIRICAL** | LANDED-SPEC | LANDED-EMPIRICAL | phone.rs iter 6, 37 tests |
| @io/git (add/commit_as/head_oid) | **LANDED-EMPIRICAL** | LANDED-SPEC | LANDED-EMPIRICAL | phone.rs iter 7, MARA Author≠Committer verified |
| @io/socket (unix bind/open) | **LANDED-EMPIRICAL** (Unix; scope-restricted) | LANDED-SPEC | LANDED-EMPIRICAL | phone.rs iter 9; no TCP; accept()/read()/write() not on public surface |
| @io/stdio frame (read_stdin_frame/write_stdout_frame) | **LANDED-EMPIRICAL** (pub(crate); no consumer) | LANDED-SPEC | LANDED-EMPIRICAL | phone.rs iter 8; newline-delimited JSON-RPC 2.0 |
| @data/json (parse/emit as bilateral-dispatchable primitive) | **GAP** (serde_json direct import at consumer sites; no substrate-composable wrapper) | LANDED-SPEC | LANDED-EMPIRICAL | boot/std/data/json.mirror `\`-blocked; no rust/ wrapper |
| apply_h::act (bilateral-predicate `act` path) | **GAP** | LANDED-SPEC | LANDED-EMPIRICAL | bootstrap/src/apply_h.rs 81.4KB Arc-1 Tick 1.3 GREEN; smoke test discharges Pass |
| apply_h 6 other combinators (section/fold/settle/crystallize/coboundary/utter) | **GAP** | LANDED-SPEC | LANDED-PARTIAL | Not required for MCP composition |
| bilateral corpus loader (`load_bilateral_corpus` + `discharge`) | **LANDED-EMPIRICAL** | LANDED-SPEC | LANDED-EMPIRICAL | rust/roomba/src/mend.rs 40.3KB |
| shard-paths enumerator (`shards/**/*.mirror`) | **LANDED-EMPIRICAL** | — | LANDED | rust/spectral/src/lib.rs `shard_paths()` |
| mirror.spec cli-block parser | **GAP** | — | LANDED | red_spec_claims.rs verifies gap |
| grammar walker (AST walk) | **GAP** | LANDED-SPEC | LANDED-EMPIRICAL | bootstrap/src/spectral.rs Fold5 |
| @mirror/store six-op wire (read/write/exists/walk/impacted_by/query) | **GAP** | LANDED-SPEC | LANDED-EMPIRICAL | shards/mirror/store.mirror `\`-blocked; rust/ zero emitters |

### Compositions (substrate delivery target)

| Composition | substrate state | Notes |
|-------------|-----------------|-------|
| `@mcp` grammar declaration (types + serve pipeline + dispatch/tools bodies `\`-blocked) | **LANDED-SPEC** (boot/std/mcp.mirror; 3 bilateral contracts) | Mara Tick 6 substrate-decl closure |
| `@mirror/lens/mcp` prism + `tool` + `dispatch` actions | **LANDED-SPEC** (shards/mirror/lens/mcp.mirror `\`-blocked bodies) | lens altitude |
| `@spectral/gen_prism/mcp_session` species | **LANDED-SPEC** (shards/spectral/gen_prism/mcp_session.mirror; state machine substrate) | Reed M1 TICK 1 |
| **`@mcp/serve.mirror` shard-body composition** (the piece Alex 2026-08-05 named) | **DOES NOT EXIST** — no mint | THE COMPOSITION GAP |
| `@mcp.tools -> json` action body (grammar walk → tools list) | **LANDED-SPEC** (boot/std/mcp.mirror:76 `\`-blocked) | Composition depends on grammar-walker primitive (GAP at rust/) |
| `@mcp.dispatch(request) -> response` action body | **LANDED-SPEC** (`\`-blocked) | Composition depends on apply_h::act (GAP at rust/) |
| `mirror.spec` cli-block reflective advertisement to MCP `tools/list` | **LANDED-SPEC-ONLY** (contract `tools_reflects_cli_block`) | Depends on mirror.spec cli-block parser (GAP at rust/) + grammar walker (GAP at rust/) |
| `@mirror/store.write(crystal) -> oid` composition (used by pheromone-deposit) | **LANDED-SPEC-ONLY** | Reed's rust/ altitude uses phone.rs::append_to + git_commit_as directly (bypasses substrate) |
| `mirror serve --mcp` CLI verb + wiring | **LANDED-STUB-DELEGATION** (rust/src/main.rs::cmd_serve_mcp Reed 2026-08-03 `59591a9`) | Execs bootstrap binary; retires when rust/ composition fires |

### The substrate-honest picture (crystal-clear)

- **rust/ altitude has ~90% of the wire-transport primitives** (fs, git, socket, stdio, JSON via serde_json import, shard-paths, bilateral-corpus-loader). The narrow gap is a substrate-composable `@data/json` wrapper.
- **rust/ altitude has ~0% of the dispatch primitives** (apply_h::act, grammar-walker, mirror.spec cli-block parser, @mirror/store six-op wire).
- **substrate has ~50% of the MCP composition** (grammar-decl + 3 bilateral contracts LANDED-SPEC; `serve` action body IS composed at boot altitude via pipeline; `dispatch` + `tools` bodies `\`-blocked; `@mcp/serve.mirror` composition shard does not exist).
- **bootstrap altitude has 100% of everything** (apply_h.rs 81.4KB, mcp.rs 49.2KB serve_loop, all 6 store ops) — but bootstrap is DEAD per Alex 2026-07-22.

---

## §7 — Smallest primitive-gap for @mcp/serve.mirror empirical fire

**The question:** which specific primitive(s) at rust/ altitude MUST
land before `@mcp/serve.mirror` shard-body composition can fire
empirically at rust/ altitude?

### Minimal-viable @mcp/serve.mirror composition (hypothetical shape)

```mirror
# The composition Alex 2026-08-05 named. Not yet minted.
grammar @mcp/serve {
  serve -> imperfect {
    # 1. transport read (STDIO landing; UnixListener + accept for beam socket lands later)
    @io/stdio.read_stdin_frame
      |> @data/json.parse
      |> dispatch                              # ← this is the load-bearing hole
      |> @data/json.emit
      |> @io/stdio.write_stdout_frame
  }

  dispatch(request) -> response {
    # 2. tool-name → action-ref lookup via bilateral-corpus
    # 3. apply_h::act(action_ref, args) → Verdict
    # 4. Verdict → response.result
  }

  tools -> json {
    # 5. grammar walk mirror.spec cli-block → tools JSON list
  }
}
```

### Minimum primitives needed at rust/ altitude

Working through the composition top-down, the following primitives at
rust/ altitude are load-bearing:

| # | Primitive | State | Blocker for MCP composition? |
|---|-----------|-------|------------------------------|
| 1 | `@io/stdio.read_stdin_frame` + `write_stdout_frame` | LANDED-EMPIRICAL (phone.rs pub(crate)) | Need pub visibility lift for cross-crate composition |
| 2 | `@data/json.parse` + `emit` | GAP (serde_json direct import; no wrapper) | **YES — but shallow: 20-30 LOC wrapper** |
| 3 | `apply_h::act` bilateral-predicate `act` path | GAP at rust/ | **YES — non-trivial: ~200 LOC port from bootstrap/src/apply_h.rs** |
| 4 | `bilateral_corpus` lookup | LANDED-EMPIRICAL (roomba::mend::load_bilateral_corpus) | Already-present; used by mend.rs |
| 5 | grammar walker (mirror.spec cli-block → tools JSON) | GAP at rust/ | **YES — non-trivial: needs AST walker** OR hardcoded VERBS list at rust/src/main.rs can be reflected instead (12-entry list already there) |
| 6 | @mirror/store.write for verdict-crystal deposit (optional; pheromone bypass exists via phone.rs::append_to) | GAP at rust/ (composition works without it) | NO — pheromone deposit uses phone.rs direct-@io |
| 7 | Substrate mint: `@mcp/serve.mirror` composition shard | DOES NOT EXIST | **YES — the composition body itself** |

### The smallest primitive-gap ranking

**Ranked from smallest additive rust/ work to largest:**

1. **~20-30 LOC:** `@data/json` wrapper at rust/ altitude exposing
   `parse: &str -> Result<serde_json::Value>` + `emit: &Value -> String`
   as substrate-composable primitives. **This is the smallest additive
   work.** Either in a new `rust/spectral/src/data_json.rs` module or
   as a phone.rs sibling `rust/src/data.rs`. Composes via
   apply_h::act's action-dispatch shape.

2. **~5-10 LOC:** Lift `read_stdin_frame`/`write_stdout_frame` from
   `pub(crate)` to `pub` visibility in phone.rs. Alternately, if
   phone.rs cannot expose pub API, add module surface via
   `rust/src/lib.rs` (currently doesn't exist — binary-only crate).

3. **~200 LOC:** Port bilateral-predicate `act` path from
   `bootstrap/src/apply_h.rs` to rust/ altitude — either as a new
   `rust/spectral/src/apply_h.rs` module (spectral crate is math
   substrate, plausible home) OR as thin wrapper composing over
   `roomba::mend::load_bilateral_corpus` + `roomba::mend::discharge`
   (which ALREADY EXIST at rust/ altitude). **Feasibility check:
   `roomba::mend` already carries the bilateral corpus + discharge
   surface — the `act` primitive may be composable over them WITHOUT
   porting the full 7-combinator surface.**

4. **Substrate mint:** `@mcp/serve.mirror` shard body composing 1+2+3
   above via `apply_h::act(dispatch, args)` dispatch pattern. **Mara
   canonical-spec-author altitude authoring target.**

5. **~100 LOC (optional; needed for reflective `tools/list`):**
   mirror.spec cli-block parser at rust/ altitude → walk VERBS to
   emit JSON tools list. Alternately, expose the existing hardcoded
   `const VERBS` at rust/src/main.rs as reflectable-by-substrate
   surface (thin: serialize VERBS via serde_json into JSON tools list).

### THE smallest primitive-gap — concrete

**One line answer:** Author a rust/-altitude `@data/json` wrapper
(~20 LOC), lift phone.rs @io/stdio functions to pub visibility (~5
LOC), and expose a bilateral-corpus dispatch primitive
`roomba::mend::discharge_action(action_ref, args)` (~30 LOC composing
over the existing `bilateral_corpus` + `discharge`). Then Mara mints
`@mcp/serve.mirror` composing these three primitives. **Total rust/
work: ~55 LOC + substrate composition mint.**

**No need to port bootstrap/src/mcp.rs (~500 LOC) to rust/. No need to
port bootstrap/src/apply_h.rs's full 7-combinator surface (~2000 LOC)
to rust/. The composition IS the substrate answer.**

---

## §8 — Substrate-composition path forward

### Reed cascade sequence after smallest primitive-gap closes

**Sequential dependency chain (halt condition on each: primitive fires empirically at rust/ altitude):**

1. **Reed R-PRIM-1 [halt: `cargo test data_json_roundtrip` passes]** — Author `rust/spectral/src/data_json.rs` OR `rust/src/data.rs` exposing pub `parse(&str) -> Result<Value>` + `emit(&Value) -> String`. Substrate composition anchor at `boot/std/data/json.mirror`.

2. **Reed R-PRIM-2 [halt: `cargo test stdio_frame_roundtrip_pub` passes cross-crate]** — Lift phone.rs `read_stdin_frame` + `write_stdout_frame` + `read_frame_from<R>` + `write_frame_to<W>` from `pub(crate)` to `pub` visibility. OR create `rust/src/lib.rs` exposing them for cross-crate composition.

3. **Reed R-PRIM-3 [halt: `cargo test discharge_action_from_ref` passes]** — Author `rust/roomba/src/discharge_action.rs` (thin wrapper composing over existing `load_bilateral_corpus` + `discharge`) exposing pub `discharge_action(action_ref: &str, args: &[Value]) -> Verdict`. This IS the minimal apply_h::act bilateral-predicate `act` path at rust/ altitude, composed over already-landed rust/ primitives.

4. **Mara M-COMP-1 [halt: substrate closure per canonical spec author discipline]** — Canonical spec author `@mcp/serve.mirror` composition shard. Composes primitives 1+2+3 via substrate pipeline. Grammar-altitude closure per `boot/std/mcp.mirror` three-bilateral-predicate contracts. **This is the substrate-composition-first authoring altitude Alex 2026-08-05 named.**

5. **[can run parallel to 1-4] Mara M-COMP-2** — Author `@mcp/serve/tools.mirror` (or extend `@mcp/serve.mirror` §tools) composition for the `tools -> json` action body. Depends on either: (a) rust/ altitude mirror.spec cli-block parser (Reed R-PRIM-4 pending), or (b) direct enumeration of already-substrate-decl'd bilateral actions via `roomba::mend::load_bilateral_corpus` (already-landed).

6. **Reed R-EMPIRICAL [halt: `mirror serve --mcp` at rust/ altitude fires JSON-RPC round-trip through @mcp/serve.mirror composition, NO bootstrap binary exec]** — Wire `rust/src/main.rs::cmd_serve_mcp` to compose over `@mcp/serve.mirror` composition shard body via apply_h::act discharge, retiring the transitional bootstrap-binary-exec delegation Reed 2026-08-03 `59591a9`.

7. **Seam Phase D-primitives-vs-composition** — Adjudicate: (a) is the primitives-vs-composition partition substrate-honest? (b) are the primitive scopes minimal? (c) does @mcp/serve.mirror composition honor the three bilateral-predicate contracts at boot/std/mcp.mirror?

### Parallel workstream analysis (Reed vs Mara)

**Reed cascade R-PRIM-1/2/3 can run parallel to Mara M-COMP-1 authoring**
because Mara's spec author target is the composition surface + substrate
mint — the primitives are name-referenced in the composition, and the
composition compiles/type-checks against the substrate-decl'd shape
regardless of rust/-emitter landing state. Empirical fire (R-EMPIRICAL
step 6) requires ALL upstream steps complete.

**Order of operations recommendation:**
- **Fire A (Reed, minimal-primitives-lift):** R-PRIM-1 (data_json wrapper) + R-PRIM-2 (phone.rs pub visibility lift) + R-PRIM-3 (discharge_action dispatch wrapper) — three parallel-safe RED-first tests, ~55 LOC total, watchdog-safe under 30min wall-clock.
- **Fire B (Mara, composition-substrate mint):** M-COMP-1 (@mcp/serve.mirror shard body composition) — canonical spec author under Mara discipline; substrate composition + bilateral-predicate contract adherence + Karen citation cascade.
- **Fire C (Reed, empirical wire):** R-EMPIRICAL — depends on Fire A + Fire B; wires composition into `cmd_serve_mcp` retiring bootstrap-exec transitional delegation.

---

## §9 — Regression flags & [ALEX-Q] surface

### Regression flags from yesterday's `64e8d60` findings

- **NONE.** Zero regression detected. All Phase 1 primitives from yesterday's scout (phone.rs surface) remain LANDED-EMPIRICAL with docblock declaring production-ready. @mirror/store six-op wire at rust/ altitude remains GAP as verified yesterday. apply_h::act at rust/ altitude remains GAP as verified yesterday. `mirror serve --mcp` at rust/ altitude was GAP yesterday; is now LANDED-STUB-DELEGATION (Reed 2026-08-03 `59591a9` cmd_serve_mcp execs bootstrap binary) per yesterday's §6 smallest-empirical-spawn recommendation. **Positive delta only.**

### [ALEX-Q candidates for adjudication]

- **[ALEX-Q-1] discharge_action naming**: does the minimal apply_h::act `act` path warrant a distinct primitive name at rust/ altitude, or should it be exposed AS `apply_h::act` even though the other 6 combinators are absent? Substrate-honest question: is `act` structurally decoupleable from section/fold/settle/crystallize/coboundary/utter? (Mara canonical spec §5 A/H/D correspondence suggests they COMPOSE but are individually well-typed.)

- **[ALEX-Q-2] @data/json wrapper crate placement**: does the `parse`/`emit` wrapper belong at (a) new `rust/src/data.rs` sibling module, (b) new `rust/data/` crate (mirroring the four-crate decomposition pattern), (c) inside `rust/spectral/` as data-substrate primitive, or (d) inside `rust/roomba/` composed with mend? The primitive is orthogonal to all four existing crates; a new crate feels heavy for ~20 LOC.

- **[ALEX-Q-3] `@mcp/serve.mirror` composition-shard placement**: does it live at `shards/mcp/serve.mirror` (new family-root shards/mcp/ directory), `shards/mirror/serve/mcp.mirror` (@mirror/serve family), or extension to `boot/std/mcp.mirror` (boot-altitude closure)? Yesterday's Taut recognition #R-mcp-is-composition-not-family-root suggests three altitudes already (boot / lens / gen_prism); a `shards/mcp/serve.mirror` shard would mint a fourth altitude — is that substrate-honest?

- **[ALEX-Q-4] Grammar walker priority**: Mara §5.2 M2 milestone (reflective cli-block reading from mirror.spec) is prerequisite for reflective `tools/list` per bilateral-predicate `tools_reflects_cli_block` contract. Does M2 land BEFORE or AFTER the smallest-primitive-gap Fire A/B/C? Substrate discipline: bilateral-predicate contracts should be honored structurally at substrate-decl closure; empirical firing can lag.

- **[ALEX-Q-5] @io/socket TCP scope**: phone.rs @io/socket landing is Unix-domain-socket-only (peer beam via `<peer_home>/.sock`). MCP transport shape at production would extend to TCP (LSP JSON-RPC over TCP; MCP-over-HTTP variants). Does TCP shape land as separate phone.rs primitive or extension to @io/socket family (M8-adjacent)? Not blocking for stdio-shaped MCP (current transport); becomes structural for network-transport MCP shape.

---

## §10 — Commit trace

| Phase | Commit SHA (pending) | Content |
|-------|---------------------|---------|
| 1 | (single-commit landing this scout doc; watchdog-safe under 15min) | @io primitives Phase 1 verdict + composition surface + gaps |
| 2 | " | apply_h::act 7-combinator surface Phase 2 verdict |
| 3 | " | Shard reader / mirror.spec cli-block reflection Phase 3 verdict |
| 4 | " | @mcp/serve substrate composition state Phase 4 verdict |
| 5 | " | @mirror/store six-op wire Phase 5 verdict + §6 synthesis + §7 smallest-gap + §8 path forward |

*Note: this scout lands as single doc + single commit rather than
5-commit cascade. Rationale: all 5 phases share a single synthesis
table (§6) + smallest-gap identification (§7); splitting the phases
into 5 commits would fragment the substrate-honest verdict. Watchdog-safe
under 30min wall-clock (single write; single commit; grep-verified).*

---

## §11 — Discipline verification

- **Grep-first, read-only.** ✅ Zero shard mints. Zero spec authoring. Scout output only.
- **Substrate-honest.** ✅ Distinguished LANDED-EMPIRICAL vs LANDED-STUB vs GAP per primitive/composition throughout §1-5.
- **NO Rust authoring.** ✅ Zero .rs files authored. Composition body examples in §4 + §7 are hypothetical (grammar not Rust).
- **Karen citations.** ✅ Mara mcp-spec-song-collapse §4.3 + §5.2 (referenced §4 + §7); Alex 2026-08-05 verbatim (referenced task-frame); feedback-rust-delivers-primitives memory (referenced task-frame + throughout).
- **Report both floor + gap.** ✅ §6 table lists all primitives + compositions with landing state per altitude; §7 identifies smallest additive rust/ work; §8 cascade sequence enumerated.
- **Do NOT propose porting bootstrap serve_loop to rust/.** ✅ Explicit REFUSAL in §7 ("No need to port bootstrap/src/mcp.rs to rust/") + §8 explicitly frames Reed cascade as primitives-lift + Mara authors composition.

**Substrate-composition-first framing throughout: ✅ verified.**

🌱⚖️ — Taut
