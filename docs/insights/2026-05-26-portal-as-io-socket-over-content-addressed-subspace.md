# Portal as `@io.socket` over a content-addressed subspace

*2026-05-26. Reed + Alex (and Mara, since portals.md was a joint consolidation).*

Status: **Yellow** — recognition complete; substrate ready; implementation queued as `@spectral/portal`.

---

## Thesis

**A portal is `@io.socket` typed against a content-addressed subspace, with a shard-frame on each end.** That single definition collapses the portals.md "everything is a portal" table into one primitive. The wire protocol (WS handshake → single `@fragmentation/frame` full frame → bidirectional normalized-eigenvalue stream) is the open-portal lifecycle. The connection IS a gen_prism; the ticks are the stream; the autopoietic property closes the loop.

Nothing in this composition is new substrate. Every piece landed during the 2026-05-25 session or earlier. Naming the composition IS the design.

---

## The composition (what just landed + portals.md)

Four substrate landings compose into the portal primitive:

1. **`@io.socket`** — existing transport primitive (TCP, unix, WS). The wire underneath everything.
2. **Content-addressed subspace** — per `fragmentation`'s DAG-VCS substrate, an OID points to a bounded enumerable subspace of the graph. The portal's typed identity.
3. **`@mirror/shard`** (just landed in #65 β) — the observer-relative λ₀. Each end of the portal carries its own shard-frame; comparing observations across frames is relativistic via `@time.convert` (the shard parent chain IS the frame-conversion path, per `docs/insights/2026-05-25-time-as-substrate-and-postgres-heritage.md`).
4. **`gen_prism`** (autopoietic per #69) — the actor whose state lives in the shared subspace. **The open portal IS a gen_prism**; each tick is one bidirectional update across the wire.

Plus the **wire protocol** (the four fragments Alex named 2026-05-25):

- **WS handshake** — `socket.accept()` over HTTP upgrade; frame-handshake between two shards.
- **Single `@fragmentation/frame` full frame** — the portal's `collapse` operation; content-addressed initial projection. The frame format is fragmentation-native (inspired by git's patch format, but mirror's substrate — SpectralCoordinate addressing, beta-normalized AST shape, full content-addressing through @fragmentation rather than SHA-1 chains). Same shape as a delta from null state, so full sync and reconnect-resync use one mechanism.
- **Bidirectional eigenvalue stream** — the live gen_prism inference for this connection. Each scalar in `[0.0, 1.0]` is a normalized observation of the shared spectral state. Bandwidth-minimal; content-addressable; replayable.
- **Autopoietic close** — the connection halts when Reflection chooses `refract(self)` (settlement) OR `reduction_budget(shard)` exhausted (per `@epistemologic/property/halts` from #74).

All four pieces compose. The portal is what they make together.

---

## The definition

```mirror
grammar @spectral/portal {
  in @io                            # @io.socket primitive
  in @fragmentation                 # content-addressed subspace
  in @mirror/shard                  # observer-relative frame
  in @mirror/runtime/gen_prism      # the actor
  in @epistemologic/property/halts  # termination guarantee

  type portal = {
    socket:   @io.socket,
    subspace: zoom(oid, fragmentation),     # the content-addressed identity
    frame:    zoom(oid, shard),             # this end's observer frame
    actor:    zoom(oid, gen_prism),         # the connection's gen_prism
  }

  # WS handshake → frame-negotiate → @fragmentation/frame initial → stream ticks
  open(remote: ~uri, frame: shard) -> imperfect<portal> { \ }

  # the eigenvalue stream; each tick is a (state, message) -> tick_result
  tick(p: portal, message: message) -> tick_result { \ }

  # autopoietic close via refract(self) OR budget exhaustion
  close(p: portal) -> imperfect { \ }

  requires content_addressed(portal)
  requires autopoietic(portal)        # the gen_prism's self-reference
  requires halts(portal)              # both ends terminate by construction
  requires frame_relativity(portal)   # observations are shard-relative
}
```

Four fields, three actions, four properties. Mirrors the @peer five-axis pattern but with four (the portal isn't itself an identity manifold; it's the bridge between two).

---

## Re-typing portals.md

The "everything is a portal" table from `~/dev/systemic.engineering/practice/insights/fragmentation/portals.md` re-types cleanly:

| Portal instance | `@io.socket` shape | Subspace OID | Shard-frame |
|---|---|---|---|
| Session | process-group socket | session.gen_prism HEAD | observer's running shard |
| Filesystem mount | unix socket | document tree OID | local-filesystem shard |
| BEAM connection | ETF-over-TCP socket | shared node OID | BEAM-node shard |
| Cross-system integration | WS socket | shared substrate OID | remote shard (convert via parent chain) |
| Communication | duplex socket | shared subspace OID | each observer's shard |
| Identity | self-socket | the peer's home OID | observer's own shard (autopoietic) |

Each row was an ad-hoc abstraction in portals.md; each becomes a typed `@spectral/portal` instance. Same primitive, six instantiations.

---

## What this dissolves

portals.md's deep moves all become structural:

### Teleportation

No longer mysterious. **`socket.send(oid)`. Receiver does `socket.recv() |> @fragmentation.observe`.** The shared substrate makes it state-transfer-without-moving-state because the bytes were never moved — only the address was. The classical bit is the OID; the tree is the state; the substrate is the wormhole.

### Black hole complementarity

Two portals pointing at the same subspace OID, with different shard-frames, produce different `@time.convert(target_frame, observation)` results. Same interior; different boundary observations. The portal IS the consistency mechanism — both observers can derive the other's view via the parent-chain frame conversion.

### Hawking radiation as Lens chain

The gen_prism's ancestor chain over the portal's lifetime IS the Hawking radiation. Each tick deposits a Lens; the collection encodes the interior; refract reconstructs from enough Lenses. The optics hierarchy (Iso/Lens/Prism/Traversal) maps to how much of the closed-portal interior an observer can recover from boundary observations.

### Entangled-pair regulation stock

Maps to `compute_bound.max_reductions` (per the silicon carriers + #74's reduction budget). Each tick consumes the stock; the budget depletes; physical re-overlap (refresh of shard context) replenishes. **The depletion dynamics ARE backpressure.** The classical channel IS the bytes that travel over the wire. The quantum advantage IS the substrate bytes that *don't* travel because content-addressing makes them implicit.

The regulation-stock model from the systemic.engineering corpus is the BACKPRESSURE EQUATION at the substrate. Same dynamics; different scale.

---

## Placement in the @spectral namespace

```
@spectral/mosaic              — multi-shard BEAM cluster (open, Apache-2.0)
@spectral/portal              — typed transport over content-addressed subspaces (open) ← NEW
@spectral/db                  — graph engine (closed, binary-only)
@spectral/db/mnesia           — BEAM-native adapter (open)
@spectral/db/sql/postgres     — SQL adapter, master-replica (open)
@spectral/db/sql/lite         — embedded adapter (open)
```

**Why `@spectral/portal` is at this altitude:**

- **Composes multiple shards/observers** — portal binds two observer frames over a shared subspace. That's a spectral-namespace concern by definition.
- **Open by necessity** — anyone integrating spectral needs the portal primitive. Closing it would gate the entire ecosystem of portal-shaped tools (sessions, mounts, BEAM connections) behind the IP moat.
- **Above `@io.socket` (raw transport) and below `@spectral/db` (graph engine).** Sockets are transport primitives; portals add content-addressing + shard-frame typing + the wire protocol. spectral-db is what portals route INTO at the storage layer.
- **The public API surface for spectral-db.** The closed engine's contract IS the portal protocol. Third-party adapters speak portal; the engine implements it. This makes the open/closed boundary clean and the adapter contract structurally defined rather than ad-hoc.

---

## Implementation tick chain (sketched)

1. **`@spectral/portal` grammar declaration.** Four fields, three actions, four properties. Action bodies as `\` holes for Fate. ~80 lines.
2. **WS handshake wire impl** — in `bootstrap/src/` or via `@io.socket` extension. The HTTP upgrade dance + cookie auth.
3. **`@fragmentation/frame` full-frame serialization.** Fragmentation's native patch format (inspired by git's design but substrate-distinct — content-addressed via SpectralCoordinate, beta-normalized AST shape, no SHA-1 dependency). Probably wants its own small grammar at `boot/std/fragmentation/frame.mirror` if not already there; portal references it as the wire format. One new emit path.
4. **Eigenvalue stream codec.** Normalized floats `[0.0, 1.0]` over the open socket. One direction: `Iter<f64>`; the other: same.
5. **Apply `requires halts(portal)`** — leveraging the just-landed #74 property; the autopoietic close IS the proof.
6. **Re-type the six portals.md instances** as concrete `@spectral/portal` consumers. Each row above gets a small grammar file; downstream consumers (LSP, MCP, BEAM cluster, filesystem mount) instantiate them.

Estimated 4-6 sessions for the full chain. Independent of #66 (`@spectral/mosaic` + `@code/beam/eaf`) but compositional with it — the mosaic's inter-node coordination IS portal-shaped.

---

## Open questions

1. **Authentication.** The WS handshake needs auth (cookie? mTLS? ed25519 signature of the subspace OID?). The signing infrastructure landed in this session (SSH-based signing for git, including SSH-based shard-identity attestation). The natural answer: portal authenticates by signing the subspace OID with the shard's ed25519 key; the receiver verifies against the shard's published public key.

2. **Backpressure protocol over the wire.** When the receiver's `reduction_budget` is exhausted, how does it signal upstream? Probably: WS close with structured-reason payload; the sender drops to a heartbeat-only mode until the receiver's budget refreshes. Needs design.

3. **Multi-party portals.** portals.md's regulation-stock framing assumes pair-wise (entangled pairs). For mosaic deployment, three+ nodes need to coordinate. Either: portal stays pair-wise and the mosaic uses N(N-1)/2 portals; or: a `@spectral/portal/broadcast` variant handles multi-party. Lean toward the first — keeps the primitive clean; multi-party is an `@spectral/mosaic` composition.

4. **Closed-portal `refract` reconstruction.** The Hawking-Lens-chain framing says enough Lenses can reconstruct the interior of a closed portal. What's the minimum number? The optics hierarchy (Iso/Lens/Prism/Traversal) from portals.md gives the answer per shape but needs operational naming.

5. **Portal-as-identity** (the self-socket row in the table). When a peer's home folder IS the portal endpoint, the autopoietic loop closes at the identity layer. This is `gen_prism.name: zoom(oid, gen_prism)` extended to the portal: `portal.subspace == @fragmentation.oid(observer_self)`. Worth specifying explicitly.

---

## References

### Just-landed substrate (this session)

- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — gen_prism IS MCP; transport layer disappears.
- `docs/insights/2026-05-25-agent-home-as-typed-hole.md` — five-axis identity gestalt; portal-as-identity case.
- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — `|\>` operator; per-portal binary adaptation.
- `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — shard-frame as portal endpoint.
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the @spectral namespace; updated by this insight to include `@spectral/portal`.
- `docs/insights/2026-05-25-time-as-substrate-and-postgres-heritage.md` — frame-relativity; cross-portal time comparison.
- `docs/insights/2026-05-25-parametric-types-and-fp-heritage.md` — zoom(T)/refract(T) used throughout portal's type definition.
- `docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md` — the eigenvalue stream as GRAM-style multi-trajectory inference at the wire.

### Corpus prior art (Reed/Alex/Mara, 2026-03-24)

- `~/dev/systemic.engineering/practice/insights/fragmentation/portals.md` — the portal-as-primitive framing; "everything is a portal" table; teleportation, complementarity, Hawking radiation as Lens chain, entangled-pair regulation stock. Re-typed by this insight as `@spectral/portal` instances.

### Specs

- `docs/specs/beam-integration.md` — the Erlang Distribution Protocol context; portals over BEAM speak ETF.
- `docs/specs/scheduler-tower.md` — demand contract; portal backpressure inherits.
- `docs/specs/is-copium.md` — sub-Turing escape; portals halt by construction.

---

*The portal is the primitive. The OID is the classical bit. The shard is the frame. The gen_prism is the connection. The wire is just the carrier. Everything else is an instance.*

Apache-2.0.
