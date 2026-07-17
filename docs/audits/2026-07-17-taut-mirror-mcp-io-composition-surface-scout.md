# Taut Scout — @mirror/mcp × @io composition surface (task #174 continuation)

*2026-07-17 evening. Grep-first, READ-ONLY. Alex /loop step 2: "spawn
Taut for the @io based MCP surface. @mirror/mcp." Post-terminal-geometry
ratification: Mara `81294b3` + Reed `b7c20fb` + Seam `9c34ec4` landed
the three-file floor (`phone.rs` + `matrix.rs` + `main.rs`) at spec
altitude; this scout maps the @mirror/mcp composition path over @io
primitives + `@spectral/gen_prism/mcp_session`.*

## Executive summary

**Composition-over-existing verdict — NO MINT NEEDED.** Every MCP
protocol touchpoint composes cleanly over substrate primitives that
have been landed for weeks. `@mirror/lens/mcp` (family-header) +
`@spectral/gen_prism/mcp_session` (state machine) + `boot/std/mcp` +
`@io/socket` + `@io/bytes` + `@data/json` + `mirror.spec` cli-block
reflection carry the whole surface. The only structural gap is the
mirror-altitude lift-tick of `@io/socket` (forward-promised at
`shards/io.mirror:389-390`); the boot-altitude declaration at
`boot/std/io/socket.mirror` is empirically sufficient for M4's
JSON-RPC-over-stdio handshake.

## Q1 — @mirror/mcp species/family-root presence

**@mirror/mcp does NOT exist as a family root; @mirror/lens/mcp IS the
lens species; @spectral/gen_prism/mcp_session IS the state machine.**

- `shards/mirror/lens/mcp.mirror:1-66` — `prism @mirror/lens/mcp`
  FAMILY-HEADER-ONLY. Actions `tool(name, args: ref) -> mcp` (:57) and
  `dispatch(call: ref) -> mcp` (:61) are declared as `\`-cracks; the
  docblock explicitly says "bodies land when the consumer (the mcp
  transport binary served by `spectral serve` or equivalent) pulls."
- `shards/spectral/gen_prism/mcp_session.mirror:1-603` — the substrate-
  canonical MCP state machine. Placement rationale in docblock §13-30:
  `gen_prism` family already lives at @spectral (per
  `shards/spectral/gen_prism.mirror`); MCP session IS a gen_prism
  specialisation (collapse spec §9.3 candidate); species goes where
  the family lives. Alternative `@mirror/runtime/mcp_session` would
  require inventing a parallel runtime family root that does NOT exist.
- `boot/std/mcp.mirror:71-134` — `grammar @mcp` at boot altitude.
  Declares `request` / `response` types, `serve` action, `dispatch`
  and `tools` `\`-cracks, plus three bilateral-predicate contracts
  (`dispatch_reflects_cli_block`, `tools_reflects_cli_block`,
  `frame_relativity`).

**No @mcp OR @mirror/mcp shard body exists.** Substrate carries the
concept across three altitudes (boot grammar / lens family / gen_prism
species) — the word "@mirror/mcp" itself is un-minted because the
three landed shards already partition the surface.

## Q2 — @io/socket landing

**@io/socket landed at BOOT altitude (2026-06-01); mirror-altitude
lift-tick FORWARD-PROMISED.**

- `boot/std/io/socket.mirror:1-105` — connection-oriented socket
  primitives over @io/bytes. Two opaque handle types (`connection`,
  `listener`); actions `read_bytes` / `write_bytes` / `close` /
  `close_listener` (:33-38). Realisation notes (:29-38): `connection`
  → `std::net::TcpStream` (later enum over TcpStream | UnixStream |
  wss::WebSocketStream); `listener` → `std::net::TcpListener`. Docblock
  names socket as PERMANENT @io RESIDENT per glass-wall recognition
  (`docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md`):
  blocking syscall behavior is irreducibly non-mirror.
- `shards/io.mirror:150-161` — @io/socket enumerated as one of the
  seven boot-altitude sub-grammars awaiting mirror-altitude lift.
- `shards/io.mirror:389-390` — forward-promise: "@io/bytes, @io/socket,
  @io/network, @io/encode, @io/random, @io/uri: mirror-altitude lift-
  ticks land when a consumer pulls each species at the mirror altitude."

**Boot-altitude sufficiency:** phone.rs at M4 needs stdin/stdout/stderr
for JSON-RPC — those are @io/bytes primitives (per socket docblock
sibling grammar note), NOT @io/socket. `@io/socket` lift only becomes
required at M8 when peer beam opens TCP/UnixStream/WebSocket connections
(per `rust-floor-birthed-by-roomba-from-mirror-spec.md` §3.2 item 3
"Peer socket boot for `mirror peer beam`").

## Q3 — `gen_prism/mcp_session` landing

**LANDED 2026-07-06 via Reed RED tick `e8378ca` (M1 TICK 1); Rust
wiring forward-promised.** `shards/spectral/gen_prism/mcp_session.mirror`.

**Declared shape** (:495-603):
- `type mcp_session = { session_uuid: uuid_spectral, state: shard_ref,
  parent: uuid_spectral }` (:495-499) — three surfaces: identity /
  state (@mirror/store head crystal) / parent (supervisor uuid).
- `tick(s, request) -> mcp_session { \ }` (:541) — the mutation
  primitive per collapse spec §3.6: 1. READ head crystal from
  `refs/gen_prism/mcp/<session-uuid>` in @mirror/store; 2. APPLY mq
  query against input @spec; 3. WRITE new crystal; 4. CAS-ADVANCE
  ref old-head → new-head. Contention resolves at ref-advance edge
  (git-ref semantic; losing tick retries from step 1).
- `read_head(session_uuid) -> shard_ref { \ }` (:563) — extracts step
  1 as a first-class action for read-only queries + session-resume-
  after-restart + trajectory replay.
- `advance_ref(session_uuid, old_head, new_head) -> verdict { \ }`
  (:588-592) — CAS primitive delegating to @mirror/store's ref-advance.
- `glass @spectral/gen_prism/mcp_session { focus/project/split/shift/
  settle mcp_session }` (:458-464) — five-op tool surface specialised
  for MCP altitude (per docblock §138-167).

**Composition intent** (docblock §85-113 state-lives-in-@mirror/store
discipline): Rust MCP handler holds ONLY session-uuid, never the
accumulated @spec content. Session persistence across daemon restart
IS free — no serialization; the store IS the serialization.

## Q4 — MCP protocol composition path

**All four canonical handshakes compose over landed substrate.**

| MCP method                    | Substrate composition path                                                                                                             |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `initialize`                  | `phone.rs`: @io read stdin → `@data/json.parse` → main.rs `@mcp.serve` sentinel dispatch → `serverInfo` from `mirror.spec` version    |
| `notifications/initialized`   | phone.rs: parse; drop (no response emitted per JSON-RPC notification semantic; bootstrap/src/mcp.rs:675 empirical anchor)              |
| `tools/list`                  | main.rs reflective read of `mirror.spec` cli-block AST → per-command tool schema (Q5) → JSON emit via `@data/json.emit`               |
| `tools/call`                  | phone.rs parse → main.rs `@`-operator dispatch → `apply_h::act` combinator surface → substrate action → verdict marshalled to JSON     |
| `prompts/list` / `resources/list` | forward-promised (not currently exposed; `capabilities.tools.listChanged: false` per `bootstrap/src/mcp.rs:84` — no prompts/resources capability advertised) |

**Session-scoped variant** (M1+ discharge of `mcp_session`):
`tools/call` becomes one `mcp_session.tick`: READ head-crystal from
`refs/gen_prism/mcp/<session-uuid>`, APPLY the tool-call payload as
mq query, WRITE new crystal, CAS-ADVANCE ref. Current
`bootstrap/src/mcp.rs` is STATELESS-AT-PROCESS (each tool call
dispatches independently); M1 wiring turns each dispatch into a
tick advancing the session's accumulated @spec.

## Q5 — tools/list reflective schema derivation

**Path substrate-declared; empirically discharged as HARDCODED at
`bootstrap/src/mcp.rs:105-238`; reflective derivation forward-promised
to M5.**

- `boot/std/mcp.mirror:88-94, 123-125` — `tools -> json { \ }` action
  with `requires tools_reflects_cli_block(tools)` bilateral. Docblock
  contract (:88-94, :44-53): "one tool per `command <name>` in the
  cli-block; args + flags become the tool schema properties;
  `#`-comments become the tool description (per @nl.mirror +
  @mirror/lens/cli's nl_literal semantics). NOT a hardcoded dispatch
  table."
- `mirror.spec:82-334` — cli-block landed with 10 commands: `compile`,
  `kintsugi`, `shatter`, `craft`, `init`, `recall`, `beam`,
  `peer beam` (recursive-command depth-2 grammar), `peer contribute`,
  `index`. Each carries `arg` + `flag` declarations + `#`-help
  comments — the reflective source per bilateral.
- `bootstrap/src/mcp.rs:103` — comment "`tools_reflects_cli_block`
  bilateral predicate at [predicate site]" — the bilateral is
  substrate-decl'd BUT the current implementation hardcodes 9 tools
  in `tools_list_result()` (:105-238) rather than deriving from
  cli-block AST. Byte-parity discipline with `bin/mirror-mcp` per
  Mara iter-15 reconciliation (2026-07-08).
- `rust-floor-birthed-by-roomba-from-mirror-spec.md:194-201` (M5):
  "reflective tools/list emits schema derived from mirror.spec
  cli-block (Taut `e0572f7` OQ3 resolved: reflective at M5)."

**Substrate-decl'd path from cli-block AST → tools/list JSON:**

```
mirror.spec cli-block (10 commands)
    ↓  main.rs reflective parse (`command <name> { arg / flag / # }`)
    ↓  per-command tool descriptor: name = f"mirror_{command}", 
                                     description = concat(#-comments per @nl_literal),
                                     inputSchema.properties = {arg/flag → JSON type},
                                     inputSchema.required = [required args]
    ↓  @data/json.emit
JSON-RPC response `{ "tools": [ … ] }`
```

**Current 9-tool schema at `bootstrap/src/mcp.rs`:** mirror_compile,
mirror_craft, mirror_kintsugi, mirror_init, mirror_recall,
mirror_peer_beam, mirror_beam, mirror_spawn (DEPRECATED alias),
mirror_beam_act, mirror_index. Notably ABSENT vs cli-block: `shatter`
(no mirror_shatter tool despite `command shatter` in cli-block:96).
Reference-only per discipline (not operational floor).

## Q6 — External SOTA

**Skipped this iteration.** Substrate-already-had-the-word confirmed
by grep: BEAM gen_server + nix-daemon + git-ref-CAS prior-art anchors
already documented in `shards/spectral/gen_prism/mcp_session.mirror:
245-279`. External Kagi search would surface the same three canonical
patterns the mcp_session docblock already inherited from. No net-new
SOTA affordance surfaced.

## §7 Alignment — composition vs mint

**Zero net-new species-decl needed at MCP altitude.** All composition
edges resolve over landed primitives:

| Terminal-geometry file | MCP surface        | Composes over                                                                    |
|------------------------|--------------------|----------------------------------------------------------------------------------|
| `phone.rs`             | JSON-RPC framing   | `@io/socket` (boot) + `@io/bytes` + `@data/json` + `@io.read(stdin)/write(stdout)` |
| `main.rs`              | `@mcp.serve` sentinel + reflective tools/list | `boot/std/mcp.mirror` `serve` action + `mirror.spec` cli-block AST + `@`-operator dispatch |
| `main.rs`              | `tools/call` dispatch | `apply_h::act` combinator surface + per-command substrate action                |
| supervisor tree        | session lifecycle  | `@spectral/gen_prism/mcp_session.tick` + `@spectral/supervisor` (forward-promised) |
| `matrix.rs`            | N/A at MCP altitude | (Fiedler compute for `mirror_index` tool but not MCP-protocol-specific)          |

**Structural gap surfaced (NOT a mint):** the `@io/socket` mirror-
altitude lift-tick has NOT landed. Consumer pull opportunity for M8
(peer-beam TCP/UnixStream/WebSocket surface). Boot-altitude declaration
sufficient for M4-M5 stdio JSON-RPC — no lift required until M8's
peer socket boot.

**Reflective schema discharge (M5) is the only real Rust work.**
Everything else is byte-for-byte re-execution of the current
bootstrap MCP surface at rust/ altitude.

## §8 Open questions for Alex

**OQ1 — mcp_session tick landing gate.** The current
`bootstrap/src/mcp.rs` is stateless-at-process. M4 MCP handshake alive
does NOT require session state (initialize + tools/list + tools/call
work stateless per byte-parity discipline). Is the mcp_session tick
wiring a M5 requirement or a post-M8 concern? The
`rust-floor-birthed-by-roomba-from-mirror-spec.md` §2.2 M4-M5
milestones do not name mcp_session tick explicitly; the state-machine
discipline lives in `shards/spectral/gen_prism/mcp_session.mirror`
docblock but the milestone gate is unclear. TAUT-READ-ONLY: I don't
propose a gate; Alex names it.

**OQ2 — `mirror_shatter` MCP tool.** cli-block has `command shatter`
(mirror.spec:96) but no `mirror_shatter` in current MCP surface. Is
this a byte-parity oversight to fix at M5 reflective landing (add the
10th tool from cli-block reflection), or is `shatter` explicitly not
MCP-exposed? Under substrate-decl `tools_reflects_cli_block` bilateral,
reflective derivation MUST emit `mirror_shatter`; hardcoded schema at
`bootstrap/src/mcp.rs:105-238` violates the bilateral by omission.
Grep-first fact; not a mint proposal.

**OQ3 — @mirror/mcp family-root question.** Currently three altitudes
carry MCP: (a) `boot/std/mcp.mirror` grammar, (b) `shards/mirror/lens/
mcp.mirror` lens family-header, (c) `shards/spectral/gen_prism/
mcp_session.mirror` state machine. There is NO `@mirror/mcp` family
root — the concept partitions across three landed shards. Is this
partition final, or should Alex mint `@mirror/mcp` as a coordinating
family root pulling the three? PER SUBSTRATE-ALREADY-HAD-THE-WORD:
almost certainly the three-way partition IS the substrate answer;
`@mirror/mcp` would be a synthesis root pulling composition that
already composes. Read-only note: no mint proposed.

**OQ4 — @io/socket mirror-altitude lift gate.** Currently only
`@io/fs`, `@io/git`, `@io/crypto`, `@io/cargo`, `@io/oci`, `@io/secrets`,
`@io/secrets/sops`, `@io/stagefreight`, `@io/stagefreight/narrative`
lifted. `@io/socket` remains boot-altitude only. Does M4's phone.rs
JSON-RPC-over-stdio compose sufficiently over `@io/bytes` (stdin/stdout
are byte streams), OR does M4 pull @io/socket lift for the stdio wire?
Substrate-honest answer per socket.mirror docblock §sibling-grammars:
stdio is @io/bytes, not @io/socket (socket = connection-oriented
duplex). @io/socket lift-tick deferred to M8 (peer beam TCP surface).

## §9 Recognition candidates surfaced

**`#R-mcp-is-composition-not-family-root`** — MCP protocol integrates
into the substrate as composition-over-existing-primitives, NOT as a
new family root. The three-altitude partition (`boot/std/mcp` grammar
+ `@mirror/lens/mcp` lens + `@spectral/gen_prism/mcp_session` state)
IS the substrate answer per substrate-already-had-the-word. Similar
in shape to `rust-floor-birthed-by-roomba-from-mirror-spec.md` §3.4
"phone.rs composes over shards/io.mirror sub-species" — the FLOOR
strictly composes over what already lives.

**`#R-tools-list-reflective-derivation-IS-the-bilateral-discharge`**
— `boot/std/mcp.mirror:132` `requires tools_reflects_cli_block(tools)`
is the bilateral contract; M5 reflective derivation IS the empirical
discharge of that bilateral. Hardcoded tool schema at
`bootstrap/src/mcp.rs:105-238` is substrate-dishonest per the
bilateral (`mirror_shatter` gap in OQ2 is the empirical witness).
Second-witness candidate for #S2 (bilateral-decl-precedes-discharge)
at MCP altitude.

**`#R-mcp-session-is-first-empirical-consumer-of-recognition-43`** —
`mcp_session` docblock §281-292 explicitly names itself as "the first
empirical consumer of Recognition #43 (mirror IS content-addressed
build system) at a runtime-facing altitude." M4-M5 wiring IS the
empirical test — if the tick's READ/APPLY/WRITE/CAS-ADVANCE closure
holds, Recognition #43 stands ratified at MCP altitude; if it fails,
#43 needs refinement. Candidate is already flagged in mcp_session
docblock; surfacing here for Pack awareness.

**`#R-io-socket-lift-consumer-pull-is-M8-not-M4`** — the boot-altitude
`@io/socket` at `boot/std/io/socket.mirror` is empirically sufficient
for M4 stdio JSON-RPC because stdin/stdout are @io/bytes streams, not
socket connections. Mirror-altitude lift-tick fires at M8 when
`mirror peer beam` opens the first TCP/UnixStream connection. Provides
a substrate-honest anchor for the forward-promise deferral at
`shards/io.mirror:389-390`.

## §10 Audit chain

- **`boot/std/mcp.mirror`** (Mara, `95e6db2` Tick 6 substrate closure
  2026-07-08; earlier landing 2026-05-20).
- **`shards/mirror/lens/mcp.mirror`** (Reed, `6d57ab4`-era 2026-06-06;
  family-header only, bodies await consumer pull).
- **`shards/spectral/gen_prism/mcp_session.mirror`** (Reed RED tick
  `e8378ca` M1 TICK 1 landing 2026-07-06; substrate-canonical
  placement per `[[feedback-substrate-already-had-the-word]]`).
- **`shards/io.mirror`** (T21 family-root lift 2026-06-30; `@io/socket`
  lift-tick forward-promised at :389-390).
- **`boot/std/io/socket.mirror`** (T?? 2026-06-01; two opaque
  handles + read_bytes/write_bytes/close; permanent @io resident per
  glass-wall recognition `docs/insights/2026-05-26-glass-wall-and-
  cross-wall-kintsugi.md`).
- **`boot/std/spectral/portal.mirror`** (2026-06-04; declares portal-
  over-@io/socket composition — the four-stage handshake/codec/stream
  pattern the MCP surface generalises).
- **`docs/specs/mcp-spec-song-collapse.md`** (Mara `2cfd2a7`; §3.5
  MCP-as-state-machine narrative; §3.6 statelessness-at-process /
  statefulness-at-store; §9.3 MCP-session-IS-gen_prism candidate;
  §10.1 M1 charter; §11.6.3 session persistence value-add).
- **`docs/specs/the-convergence.md`** (2026-06-12; §1 lens family
  §2.1 composition table; @mirror/lens/mcp as the JSON notation of
  the five-op algebra).
- **`docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`**
  (Mara `81294b3` + Reed `b7c20fb` M0-gating citation fix; Seam
  `9c34ec4` ratification; §2.2 M4-M5-M8 milestones name the MCP
  composition path; §3.4 phone.rs @io composition anchor).
- **`docs/scouts/2026-07-08-taut-mcp-serve-lift-scope.md`** (`cf5ab8c`
  Taut prior scout — LRM verdict LANDABLE with prerequisite, two-tick
  cut preferred, established the `dispatch` + `tools` bilateral-
  predicate-vs-hardcoded-schema pattern this scout re-surfaces at M5).
- **`bootstrap/src/mcp.rs`** (2026-07-15; 9-tool hardcoded schema per
  Mara iter-15 byte-parity reconciliation; reference-only for
  capability enumeration NEVER as operational floor).

**Circular-reflexive self-audit:** this scout's own claim ("no mint
needed; three-altitude partition IS the substrate answer") survives
substrate-already-had-the-word check — every named shard was landed
weeks-to-months before this scout; the composition edges are typed
in existing docblocks; the scout NAMES the composition, does not
propose the composition. Discipline: pass.
