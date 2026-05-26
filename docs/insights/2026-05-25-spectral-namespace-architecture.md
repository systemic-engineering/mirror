# The `@spectral` namespace: mosaic, not legion; open adapters, closed engine

*2026-05-25. Reed + Alex.*

Status: **Yellow** — architectural recognition complete; namespace shape ready to encode in grammar; the closed-source boundary is the business model decision.

---

## Thesis

The `@spectral` namespace separates four layers cleanly:

- **`@spectral/mosaic`** (open, Apache-2.0): the multi-shard BEAM-cluster deployment grammar. Composes individual mirror-binary shards into a coherent cluster. Compiles to `@code/beam/eaf`. Heterogeneous-tiles-make-a-picture, not legion-of-clones.
- **`@spectral/portal`** (open, Apache-2.0): typed transport over content-addressed subspaces. The portal primitive — `@io.socket` + content-addressed subspace OID + shard-frame on each end. The wire protocol (WS handshake → `@fragmentation/frame` full frame → bidirectional eigenvalue stream) is the open-portal lifecycle. Public API surface for `@spectral/db`. See `docs/insights/2026-05-26-portal-as-io-socket-over-content-addressed-subspace.md`.
- **`@spectral/db`** (closed, binary-only): the proprietary graph engine. Eigenvalue compute, fragmentation, kintsugi tournament implementation, conductivity tensors. The IP moat. *Speaks the `@spectral/portal` protocol at its public boundary.*
- **`@spectral/db/{mnesia, sql/postgres, sql/lite}`** (open adapters): wrappers between the closed engine and existing storage substrates. Third-party adapters welcome.

Each layer has a different relationship to commodity vs differentiation. The namespace honors the distinction. **The portal layer is what makes the adapter contract structurally defined rather than ad-hoc** — every adapter speaks portal; the engine implements portal; third-party tools (LSP, MCP, BEAM nodes, filesystem mounts) consume portal.

---

## Mosaic, not Legion

A mosaic is heterogeneous tiles fitted into a coherent image. A legion is uniform clones marching in formation.

For multi-shard deployment:
- Each shard has its own λ₀ (its own hardware, peer composition, compute budget)
- Tiles fit together = shards interoperate via BEAM cluster
- The mosaic is observable as a coherent whole = the cluster has unified semantic view despite physical heterogeneity
- Mosaic = art form = honest about the design intentionality

Naming the framing as "mosaic" instead of "legion" is structurally honest about what the cluster actually is: different hardware, different load, different agents per tile — fitted into pattern, not forced into uniformity.

---

## The Stack

```
@io.socket                           → raw transport primitive (TCP/unix/WS)
@spectral/portal                     → typed transport: socket + subspace + frame
@code/llvm                           → per-shard mirror binary (locally optimal via |\>)
@code/beam/eaf                       → multi-shard mosaic runtime (BEAM application format)
@spectral/mosaic                     → grammar composing shards into a deployment
```

The portal layer sits between raw transport and graph-engine concerns: every adapter, every cross-shard message, every spectral-db connection rides on `@spectral/portal`. The closed `@spectral/db` engine speaks portal at its public boundary; the open adapters speak portal at the storage end. Portal is the seam.

Mirror compiles to LLVM for the per-shard binary; spectral compiles to BEAM for the cluster orchestration. The runtime layers are honest about what BEAM was designed for: distributed, fault-tolerant, multi-node coordination. Erlang/OTP/Mnesia have spent four decades getting this right; don't reimplement.

`@code/beam/eaf` is new substrate work — mirror needs an emit path to BEAM (Erlang Application Format), parallel to the existing `@code/llvm` path. EAF is conceptually the OTP `.app` / release format; exact target shape wants design.

---

## The Closed-Source Boundary

```
Open (Apache-2.0):
  mirror              # the compiler + grammars + @peer + @glue
  @spectral/mosaic    # multi-shard BEAM cluster grammar
  @spectral/db/mnesia       # BEAM-native distributed adapter
  @spectral/db/sql/postgres # SQL adapter, master-replica
  @spectral/db/sql/lite     # embedded single-node adapter

Closed (binary-only):
  @spectral/db        # the graph engine — eigenvalue compute,
                      # fragmentation, kintsugi, conductivity tensors
```

This boundary is coherent with the `feedback-no-paywall-in-compiler` discipline. The compiler is free (compilers are commodities; pedagogy wins). The graph engine is closed (the spectral algebra at scale is hard-won; it's the moat). The grammars stay readable; the proofs stay verifiable; the binary stays your business.

The math is published. The algorithms are documented. The proofs are inspectable. What's closed is the production-grade implementation of those proofs at scale.

---

## Adapter Architecture

The engine doesn't reinvent storage. It sits on whichever backing tier fits the deployment:

| Adapter | Properties | Use case |
|---|---|---|
| `@spectral/db/sql/lite` | Embedded, single-node, ACID | Local dev, tests, single-machine |
| `@spectral/db/sql/postgres` | Master-replica, ACID, ops-friendly | Mid-scale, single-master clusters |
| `@spectral/db/mnesia` | BEAM-native, multi-master, eventual consistency | spectral.engineer, cluster-first |

Same engine, different persistence tier. Best-of-breed at each layer.

The adapter contract (what protocol the closed binary speaks to the open adapters) is the public boundary — versioning and stability matter here specifically. The discoverability extension point welcomes third-party adapters: `@spectral/db/dynamo`, `@spectral/db/redis`, `@spectral/db/sqlserver` are all reasonable next steps.

---

## Implications

- **License model** — per-deployment? per-org? per-shard? Open question; needs to land before v1.0.
- **Binary distribution** — given the shard-as-nix-flake recognition, `@spectral/db` ships as a flake whose source is sealed. The `|\>` operator still applies; closed source doesn't mean fixed bytes. Fate can still resolve the locally-optimal binary per shard's hardware.
- **Phase 7 spectral.engineer deployment** = a mosaic of mirror-binary tiles, coordinated on BEAM, backed by `@spectral/db` (licensed) using whichever adapter the deployment composes in. The license is what spectral.engineer charges for.
- **Self-hosting story** — fully possible; users license the spectral-db binary and run the rest themselves.

---

## Connections

- `mirror-supersedes-daemon` — mirror is the local-node substrate; spectral is the cluster substrate. Same separation principle.
- `pipe-hole-and-au-binary` — `|\>` works the same for closed binaries; the AST is verifiable, the binary is locally optimal, the engine just ships as a sealed-source flake.
- `shard-as-observer-relative-lambda-zero` — the shard composes adapters; the mosaic composes shards; the namespace separates the layers.
- `2026-05-26-portal-as-io-socket-over-content-addressed-subspace` — names `@spectral/portal` as the typed transport primitive; the public API surface this namespace's closed engine speaks at its boundary.

---

*The compiler is free because compilers are commodities. The graph engine is closed because the spectral algebra at scale is hard-won. The grammars stay readable; the proofs stay verifiable; the binary stays your business.*

Apache-2.0 (this insight document).
