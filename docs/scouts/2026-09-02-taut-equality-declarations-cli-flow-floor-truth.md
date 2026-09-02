# Taut 2026-09-02 — Equality-declarations + CLI-flow floor-truth scout

**Author:** Taut <taut@systemic.engineer>
**Charter:** Alex 2026-09-02 verbatim — find other `=` equality-declarations
in shards (Alex recalls `au = io = sel`); find floor-truth on three
CLI-flow shapes Reed needs.
**Mode:** grep-first, read-only, cite everything.

---

## Scope A — `=` equality-declarations in shards

**Alex's recalled `au = io = sel`: FOUND (with sharpening).**

The equality is declared in TWO places as an *architectural decision reference* named
`[[architecture-type-sel-io-au]]`, with body `sel = io + au`:

- `shards/container.mirror:246` — verbatim `Per \`[[architecture-type-sel-io-au]]\` (sel = io + au static type)`
- `shards/io/oci.mirror:218` — verbatim `Per [[architecture-type-sel-io-au]] (sel = io + au static type):`

Additional adjacent citations (SEL boundary compositions):

- `shards/io/git.mirror:552-556` — `Auth at SEL boundary per [[architecture-type-sel-io-au]]`
- `shards/io/oci.mirror:530` — `Auth refinement at SEL boundary (per [[architecture-type-sel-io-au]])`
- `shards/io/secrets.mirror:122-124` — SEL-boundary key/crypto composition citations

**Historical grounding:** `docs/audits/2026-07-14-seam-subject-sel-petri-coherence-phase-d.md:634-654`
audits `type sel = @io + @au` at compositional-typing altitude
under a section titled `D6 — \`type sel = @io + @au\` composition-typing bilateral`
(SEAM-Phase-D audit). Alex-adjudication residue A2 (line 962) named it "family-root placement".

**Semantic (my read):** NOT a `type sel = ...` declaration in any current
`.mirror` shard body. It is a **substrate-decision reference** (double-bracket citation)
naming a compositional-typing decision that the SEL boundary IS the SUM
of the `@io` boundary + the `@au` static type. The `+` is disjoint-union
type composition, not addition. `au = io = sel` (as Alex phrased it) is
the deeper reading: all three are aliases for the SAME boundary observed
from three altitudes (silicon, information, licensable-party).

**No `type sel = ...` shard-decl currently exists** — the composition is
referenced but the type is not declared as a top-level shard type.

**Other `=` patterns in shards** (grep of `^type [a-z_]+ = `): pervasive
type-alias declarations (e.g. `shards/glass.mirror:251 type reason = @nl`;
`shards/mq.mirror:581 type mq_literal(text) = @sigil("mq")`;
`shards/nl.mirror:34 type nl_literal(text) = @sigil("#")`). These are
standard TYPE-ALIASING via `=`, not identity/equality declarations. Distinct
altitude from Alex's `au = io = sel` observation.

## Scope B — `shards/kintsugi/mend/sugar.mirror`

**Full read:** 236 lines.

Declares `@kintsugi/mend/sugar` sub-species via
`prism @kintsugi/mend/sugar { focus/project/split/shift/settle sugar_event }`
(lines 133-139). Composes over four @kintsugi/fracture/* detection species +
@magic/reveal/expand resolution + @mirror/store crystallize/load + Reed
Fire A rust/-altitude primitives (phone::@io/fs + wire::parse/emit +
apply_h::act dispatch).

**Sugar-desugar primitives (lines 173-235):**
- `type sugar_event` — 6-field record (shard_path, oid_pre, oid_post, fractures_found, bytes_removed, witness)
- `bilateral sugar_witnessing` (line 200) — round-trip fidelity contract; sentinel `"sugar-bit-parity-round-trip = pass-content-addressed"`
- `mend(shard_path: ref) -> sugar_event` (line 226) — composition-shard body; body `\`-obligation-blocked
- `walk_cascade(scope: ref) -> ref` (line 235) — Fire E M-E4 walker driver

**Pipeline (lines 47-78 docblock):** Write-path (source→store): `read_file |> parse |> detect_all_fracturable |> omit_fractures |> crystallize`. Read-path (store→source): `load(oid) |> expand(_, aud) |> emit |> write_file`. Round-trip identity asserts `oid_pre == oid_post` per Mara math §2 identity functor `resugar∘sugar = id`.

**Relationship to imports (lines 1-22):** `in @data` + `in @data/mirror` are consumed as `wire::parse`/`wire::emit` primitives at @data/mirror altitude (Reed Fire A R-PRIM-1 landings).

## Scope C — `shards/mirror/data.mirror`

**Full read:** 195 lines.

**Family-root** for mirror's data-grammar. Declares `@mirror/data` (line 195: `out @mirror/data`) as the parent under which `@mirror/data/json`, `@mirror/data/yaml`, `@mirror/data/toml`, `@mirror/data/text` sit as parallel species.

**No 5-op prism block landed here** — the shard is a docblock-only family-root declaration; species files under `shards/mirror/data/` (json, yaml, toml — 3 species landed) carry the prism blocks. The docblock names data-grammar-IS-a-lens (lines 20-32) as load-bearing recognition.

**Relationship to @data/mirror duality:** the shard names @mirror/data as a "lift" from grammar-altitude `@data/json` (still landed at `boot/std/data/json.mirror`) to mirror-altitude `@mirror/data/json` (species file at `shards/mirror/data/json.mirror:154 type json = null | bool(bool) | ...`). Sibling to `@facet/X` (language-grammar) family; both are grammar-as-lens at mirror altitude.

## Scope D — `<=` semantic in prism-inheritance-form (13 sites)

Representative sites read:

- `shards/silicon.mirror:153 prism @silicon <= @autopoietic { focus/project/split/shift/settle silicon }`
- `shards/reality.mirror:363 prism @reality <= @autopoietic { ... }` (docblock at 195: `"<= @autopoietic" — what makes @reality a` [reality-family]; 323: `via <= @autopoietic; fold-back permission`)
- `shards/reality/algebra.mirror:202 prism @reality/algebra <= @reality { ... }`
- `shards/reality/algebra/math.mirror:361 prism @reality/algebra/math <= @reality/algebra { ... }`
- `shards/mirror/store/git.mirror:270 prism @mirror/store/git <= @mirror/store { focus/project/split/shift/settle git_store }`
- `shards/glue/math_silicon.mirror:332 prism @glue/math_silicon <= @glue { ... }` — line 339 docblock: `The species inherits from @glue via \`<= @glue\`; the correspondence, morphism, and translation_outcome carriers are re-used from the family-root`

**Semantic (my read):** `<=` is **prism-species inheritance** (child inherits parent's carriers + surface actions; child specializes the 5-op block to its own focus type). Not alias, not delegation. The child species RE-USES the family-root's carrier types (correspondence, morphism, translation_outcome — see `glue/math_silicon.mirror:339`) and SPECIALIZES the 5-op block for its altitude.

Evidence chain:
- `shards/loop.mirror:401` docblock: `"\`@spawn <= @loop\` inherits every action above and adds four"`
- `shards/silicon.mirror:96` docblock: `"@silicon <= @autopoietic <= @bauchladen"` — transitive chain
- `shards/silicon.mirror:100` docblock: `"The @fate composition is via \`in @fate\`, not \`<= @fate\`"` — explicit distinction between inheritance (`<=`) vs composition-import (`in`)

Sub-species mints inherit the parent prism's carriers and specialise the focus.

## Scope E — @io/socket landed primitives

**LANDED. Iter 9 (`0f2b3bf`); Mara §3 socket-handover altitude.**

- `rust/src/phone.rs:1-4` docblock: `"phone.rs — the @io socket-handover altitude"`
- `rust/src/phone.rs:47-56` — `@io/socket (iter 9 0f2b3bf; 11 tests) — M8 landed. open_peer_socket + bind_peer_socket + PeerSocketConnection + PeerSocketListener carriers over std::os::unix::net::{UnixStream, UnixListener}. Peer socket path convention <peer_home>/.sock per @peer/persistence discipline`
- `rust/src/phone.rs:201-210` — `pub(crate) fn open_peer_socket(peer_home: &str) -> io::Result<PeerSocketConnection>`
- `rust/src/phone.rs:227-238` — `pub(crate) fn bind_peer_socket(peer_home: &str) -> io::Result<PeerSocketListener>`
- `rust/src/phone.rs:255-266` — carrier structs `PeerSocketConnection { stream: UnixStream }` and `PeerSocketListener { listener: UnixListener }`
- `rust/src/phone.rs:1668-1908` — 11 property tests covering lifecycle + roundtrip + concurrent + UTF-8 + 64KB spans

**Substrate-decl anchors:**
- `shards/io.mirror:151-155` — `@io/socket connection-oriented duplex streams over @io/bytes. Two opaque handles (connection, listener); read_bytes / write_bytes / close`
- `shards/mirror/phone.mirror:34-36` — `@io/socket — TCP/UDS connect + accept + read + write (MCP stdio transport surface)`

**Handover semantics:** peer-to-peer via `<peer_home>/.sock` unix socket. Stream reads/writes at byte altitude via `UnixStream`. Bind clears stale socket files.

**No landed precedent for "state persistence between separate mirror-invocations via socket."** The sockets exist for peer-beam (persistent-identity peer-to-peer), NOT for cross-invocation state handover. See Scope G.

## Scope F — CLI-verb shapes for ask/infer/nl-input

**`ask` verb: NOT LANDED** (grep of `rust/src/main.rs:153-171 const VERBS`).
**`infer` verb: NOT LANDED** as CLI verb.
**@nl-string-shaped input verb: NOT LANDED as top-level verb.**

Current VERBS array (`rust/src/main.rs:153-171` verbatim):
```
compile, kintsugi, shatter, craft, init, recall, beam,
peer beam, peer contribute, index, roomba, serve, bumblebee, bench
```
(14 verbs, not 10 despite header comment.)

**infer at rust altitude:** `rust/src/main.rs:86` imports `infer_via_rotation` from `prismqueer::spectral`; used internally by `roomba` cascade (line 610). Not a CLI verb, an internal primitive.

**CLI verb species landed under `shards/mirror/lens/cli/*`:**
`compile.mirror`, `kintsugi.mirror`, `shatter.mirror`, `bootstrap.mirror`, `sh.mirror`, `reflect.mirror`, `time.mirror`, `crack.mirror` (8 species; per `compile.mirror:12-15` "eight verb sub-stages forward-promised"). No `ask.mirror`, no `infer.mirror`, no NL-input-shaped verb.

**NL sigil landed:** `shards/nl.mirror:34 type nl_literal(text) = @sigil("#")` — the `#` sigil produces an @nl term. Available as substrate type but NOT wired into CLI as `mirror ask "..."` shape.

## Scope G — substrate-state-handover-between-invocations precedent

**Landed session/persistence primitives:**

- `shards/spectral/gen_prism/mcp_session.mirror:100-106` — `"session persistence across daemon restart IS free (no serialization; the store IS the serialization)"`
- `shards/spectral/gen_prism/mcp_session.mirror:203-207` — `"session persistence IS an agentic value-add of the MCP-as-gen_prism collapse. Because the session state lives in @mirror/store (not in-process), a daemon restart does not lose the accumulated @spec"`
- `shards/spectral/gen_prism/mcp_session.mirror:482-486` — `"DARK 80 bits identify it content-addressably for session ref lookup and cross-restart persistence"`

**Landed wavefunction-collapse discipline (@io boundary):**

- `shards/io.mirror:432-470` — `"@io as wavefunction-collapse discharge boundary … Every @io crossing is a MEASUREMENT EVENT"`
- `shards/io.mirror:509-511` — `"@io = discharge = wavefunction collapse = event-horizon crossing. Three names, one substrate"`
- `shards/eigen.mirror:15-19` — `"Liquid splinters settle into content-addressed crystals by holding the quantum wavefunction @coherent as long as possible"`

**Landed SCM_RIGHTS handoff precedent (T20 forward-promised):**

- `shards/mirror/data.mirror:129 T20 — portal handoff via SCM_RIGHTS at CLI pipe boundary`
- `shards/mirror/spectral/portal.mirror:37-42` — `"SCM_RIGHTS needs (OS-layer fd passing) … the substrate has been declaring the typed-capability primitive since boot/std/spectral/portal.mirror"`
- `shards/mirror/spectral/portal.mirror:250-255` — `"T20 SCM_RIGHTS pipe handoff … the substrate's shift(oid, T) triple flows over the wire without re-serialisation"`
- `shards/spectral/portal.mirror:218-232` — `"T20 SCM_RIGHTS pipe handoff at CLI pipe boundary; task #263 lands the OS-layer realisation … the substrate state never serialises to bytes between mirror processes"`

**Cross-invocation state handover: SUBSTRATE-DECL-LANDED, IMPLEMENTATION FORWARD-PROMISED.** The discipline exists (T20 SCM_RIGHTS portal handoff via CLI pipe boundary; content-addressed OID surviving handoff via receiver-side @mirror/store lookup). Rust implementation NOT landed. Task #263 named in shard `spectral/portal.mirror:220`.

Wavefunction-collapse-avoidance discipline: content-addressed OID handoff over pipe means `shift(oid, T)` triple flows without re-serialization — the OID survives the SCM_RIGHTS transfer because both sides resolve the same content-address through their own @mirror/store.

---

## Punch-list for the three CLI-flow shapes

**1. `mirror ask "What are my tasks today?"` (@nl question-shaped input):**
- verb `ask`: NOT LANDED (`rust/src/main.rs:153-171` VERBS array)
- `type nl_literal = @sigil("#")` LANDED (`shards/nl.mirror:34`)
- @nl.compose action LANDED (`shards/nl.mirror`, forward-promised body); apply_h::act dispatch precedent EXISTS
- gap: no CLI-verb entry point taking bare @nl string; no dispatch from question-shape to substrate query

**2. `mirror '@tasks |> filter .date = @time/today' | mirror @gestalt/calendar` (jq-style pipe with socket handover):**
- @mq family-root LANDED (`shards/mq.mirror:1-890`) — mirror's algebra as query language
- `|>` pipe-composition FORMALIZED as typed action per Mara §15 recommendations (mq.mirror docblock line 42)
- `@time/today` reference: `@time` shard LANDED; `today` species: not verified this scout
- `@tasks` family-root: not verified this scout
- socket handover between `|` piped invocations: NOT LANDED. T20 SCM_RIGHTS is forward-promised (task #263). NO current wire for two `mirror` invocations to share state without wavefunction collapse.
- CLI verb accepting bare MQ expression as first arg (not behind a subcommand): NOT LANDED. `mirror serve` accepts `mirror_query` MQ expressions via MCP tool; standalone CLI MQ execution absent.

**3. `mirror infer "any @nl string"`:**
- verb `infer`: NOT LANDED as CLI verb (`rust/src/main.rs:153-171`)
- `infer_via_rotation` from `prismqueer::spectral` LANDED as internal primitive (main.rs:86, called from roomba)
- gap: no CLI-verb entry point; no @nl-string→infer dispatch

---

## Verdict: `prism @data = @mirror` grammar shape

**GRAMMAR-EXTENSION, not grammar-recognition.**

The `=` operator IS landed as:
- type-alias in `type <name> = <expr>` form (pervasive; `shards/glass.mirror:251`, `shards/mq.mirror:581`, `shards/nl.mirror:34`, etc.)
- reference-inside-docblock naming compositional-typing decisions (`sel = io + au` at `shards/container.mirror:246` + `shards/io/oci.mirror:218`)

The `=` operator IS NOT landed as:
- top-level prism-alias operator (e.g. `prism @data = @mirror` as a syntactic form declaring `@data` and `@mirror` are the SAME prism)
- identity-declaration between prisms at family-root altitude

Prism-inheritance uses `<=` (13 landed sites), NOT `=`. Prism identity (compose-over) uses `in @X` (imports), NOT `=`.

Reed's fabricated proposal `prism @data = @mirror` would require:
(a) new grammar production for prism-level `=` operator, OR
(b) reinterpretation of `<=` as bidirectional-identity (currently unidirectional-inheritance per `shards/silicon.mirror:100` `<= @fate` vs `in @fate` distinction).

Neither is landed. Alex's `au = io = sel` observation refers to the compositional-typing DECISION `[[architecture-type-sel-io-au]]` at docblock altitude (`shards/container.mirror:246`), NOT a landed prism-`=` operator.

---

**Scout complete. Read-only mode preserved. No proposal authored.**
