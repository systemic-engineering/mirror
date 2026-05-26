# spectral-db — Four-Tier Architecture with Nix Cold Tier + Iceberg Deep Archive + Biology-Typed Pheromone Dynamics

2026-05-26 — architectural decision by Alex; transcribed from conversation.

Status: Yellow — proposal; nothing in this spec is implemented today. The substrate pieces below the tier orchestration (Nix, Postgres, Mnesia) are all production-grade. The orchestration layer is the engineering work.

## The architecture

Four storage tiers, each on a tested substrate, with biology-typed coordination semantics:

| Tier | Substrate | Biology semantics | Access pattern |
|---|---|---|---|
| **Hot** | Mnesia (BEAM-distributed in-memory) | Foragers' current decisions / live coordination | Sub-millisecond access; active scene + tournament state; transactional |
| **Warm** | Postgres + `pgvector` (projected eigenvalues + pheromone field) | Trail map / recently-traversed paths | Queryable; indexed; informs current decisions; vector-search supported |
| **Cold** | Nix store (content-addressed blob storage; `@fragmentation` typed substrate above) | Colony's archaeological record | Immutable; full provenance preserved; rarely queried; cryptographically verified by construction |
| **Iceberg** | Tape / cloud archival / decentralized storage (pluggable adapter pattern) | Fossil record / deep sediment below the colony | Hours-to-restore latency; immutable for regulatory retention; cost per TB lowest |

Iceberg-shaped because the visible substrate above (hot/warm/cold) is small compared to what's below — most regulated-industry deployments accumulate orders of magnitude more cold-archive data than active working set.

## The substrate-pull discipline at the storage altitude

No novel storage technology in this stack. Every tier uses a battle-tested substrate. The contribution is the typed orchestration across tiers — the pheromone dynamics, the convergence-proof integration, the garden's super-colony coordination protocol.

Same pattern as @scene (didn't invent multi-actor interaction; typed what theater knew) and @kintsugi/fracture (didn't invent rewrites; typed what Credo+mix-format did). The cold tier doesn't invent content-addressed storage; it types what Nix already does.

## Cold tier — Nix store + `@fragmentation`

*Altitude: existing infrastructure (Nix) + declared substrate (`@fragmentation`); orchestration NOT implemented.*

Nix already provides everything the cold tier needs:

- Content-addressed by hash (`/nix/store/<hash>-<name>`)
- Immutable by construction
- Reference closures (what crystal cites what)
- Garbage collection with GC roots + reachability
- Binary caches + substitution (cross-deployment distribution)
- Cryptographic verification (the hash IS the verification)
- Cross-platform store format (Linux + macOS + NixOS)

Mirror's `@fragmentation/frame` is the typed substrate above Nix's blob layer. The composition:

```
@fragmentation/frame              — typed crystal carriers
         ↓
   Nix store entries              — content-addressed blob storage
         ↓
   Nix GC + binary caches         — promotion/demotion + cross-deployment
```

**Cold-tier pheromone dynamics:**

- **Evaporation** = Nix GC traversal with pheromone-strength metadata
- **Trail strength** = GC root reachability + last-access timestamp
- **Reinforcement** = re-acquiring a GC root (e.g., a new fracture cites the crystal, or a scene record references it)
- **Eviction** = GC collection when reachability drops AND evaporation curve says drop

The `@time.duration` substrate (per #76) carries the evaporation curve timing.

## Warm tier — Postgres + `pgvector`

*Altitude: existing infrastructure (Postgres + pgvector); orchestration NOT implemented.*

Projected eigenvalues + pheromone fields + queryable trail metadata:

- The spectral lift's output vectors live here (the eigenvalues mirror's `@fate.minimize` operates on)
- Pheromone trail strengths (continuous values) live here
- Foraging history (which trails were traversed when by which `@fate` agents) lives here
- Crystal-OID → vector-projection lookup table
- Cross-references between crystals (the graph structure that the spectral lift operates on)

`pgvector` provides standard vector similarity search; mirror inherits.

**Warm-tier pheromone dynamics:**

- **Evaporation** = exponential decay on trail strengths over `@time.duration`
- **Reinforcement** = trail-strength increment on foraging
- **Promotion to hot** = trail strength crosses threshold + access frequency hits N/window
- **Demotion to cold** = trail strength drops below threshold; vector ages out

## Hot tier — Mnesia (BEAM-distributed in-memory)

*Altitude: existing infrastructure (Mnesia); orchestration NOT implemented; depends on `@code/beam/eaf` per #66.*

Live coordination state:

- Active scene state (`@scene` participants, invariants, current verdict candidates)
- Active tournament state (`@fate.minimize` current trajectories, backtrack stack, candidate compositions)
- Hot crystals (the working set; sub-ms access required)
- Live foraging agents (`@fate` agents with active trails)
- Pheromone deltas pending warm-tier write-back

Mnesia is BEAM-native and fits the `@code/beam/eaf` deployment target. Distributed across the BEAM cluster shards (per `@spectral/mosaic`, #66).

**Hot-tier pheromone dynamics:**

- **Reinforcement** = immediate trail-strength increment on active foraging (in-memory)
- **Evaporation** = handled at warm-tier; hot tier is the action surface
- **Tournament round commits** = batched writes to warm tier; cold tier touched only on crystallization

## Iceberg tier — deep archive for regulatory retention

*Altitude: existing infrastructure (LTO tape; AWS Glacier; Filecoin; Storj); orchestration NOT implemented; adapter pattern pending.*

For regulated industries that cannot forget. Below the cold tier; usually below the deployment's own infrastructure.

**Backend options (pluggable adapters):**

- `@spectral/db/iceberg/tape` — LTO drives (LTO-9: 18TB compressed; multi-decade durability; air-gapped; physical custody)
- `@spectral/db/iceberg/glacier` — AWS S3 Glacier Deep Archive (12-48hr restore; $0.00099/GB/month)
- `@spectral/db/iceberg/azure-archive` / `@spectral/db/iceberg/coldline` — equivalent cloud archives
- `@spectral/db/iceberg/filecoin` / `@spectral/db/iceberg/storj` — decentralized archival; geographic distribution; cryptographic verification across replicas
- `@spectral/db/iceberg/local` — commodity disk for low-volume deployments that still want a fourth tier

All adapters speak `@spectral/portal`. The deployment chooses backend(s); the engine's orchestration is backend-agnostic.

**When crystals enter the iceberg:**

When Nix's GC would collect an unreferenced crystal (cold-tier evaporation reaching zero), the iceberg layer captures the crystal first:

- Pheromone trail metadata is stripped (no active foraging signal)
- Crystal content + provenance + last-known reference graph is preserved
- Iceberg metadata is added: ingestion timestamp; retention policy; jurisdiction tags; original deployment ID
- Nix GC then proceeds; the crystal is gone from the cold tier; it persists in the iceberg

**When crystals leave the iceberg:**

Never, in the regulated case. The iceberg is immutable retention.

In the rare case of restoration (audit; legal hold response; cross-deployment cross-pollination of an archived crystal):

- Iceberg adapter is queried by crystal OID
- Adapter restores the crystal (latency varies: tape mount, glacier restore window, decentralized fetch)
- Crystal is re-inserted at the cold tier with a fresh GC root
- Normal promotion/demotion resumes

**Why iceberg is structurally right:**

The convergence proof's Lyapunov function (total holonomy) integrates across all four tiers. Iceberg-tier holonomy is bounded below by structural retention requirements — it doesn't decrease via normal tournament rounds, but it also doesn't grow without bound because:

- Retention policies have time limits (HIPAA 6y; SOX 7y; GDPR indefinite-but-policy-bounded; defense 30y+)
- Beyond the retention horizon, even iceberg crystals can be cryptographically destroyed (e.g., key destruction for encrypted blobs; physical tape destruction)
- The iceberg's contribution to total holonomy is a FIXED COST, not a runaway sink

The convergence proof still terminates; the iceberg adds a constant offset, not unbounded accumulation.

**Compliance use cases the iceberg unlocks:**

- **HIPAA**: 6+ year retention of PHI with cryptographic provenance
- **SOX**: 7 years for financial records with audit trail
- **GDPR**: indefinite retention for legal hold; right-to-erasure via key destruction on encrypted blobs
- **Defense / cleared contractors**: 30+ year retention with air-gapped tape custody
- **Pharmaceutical**: 30+ year clinical trial data with cross-jurisdictional retention
- **Insurance / actuarial**: decades-scale historical data for risk models
- **Legal / discoverable**: every interaction crystal preserved for potential discovery
- **Disaster recovery beyond cloud provider failure**: physical tape custody is sovereign of any cloud's continued existence

This is the substrate-altitude answer to enterprise procurement's "can you guarantee we won't lose this?" question. The answer becomes: yes, mathematically, and here's the tape.

**Biology semantics at iceberg altitude:**

Fossil record / deep sediment. The colony's evolutionary history. Most colonies don't dig this deep, but those that do can read their entire history. For mirror: the substrate retains everything that ever existed in the corpus; pheromone trails are gone (those evaporated); the crystal content + structural provenance persists indefinitely.

## Tier-orchestration semantics

The IP moat. Closed-source `@spectral/db` engine. Open adapters (`@spectral/db/{mnesia, sql/postgres, sql/lite}`) speak `@spectral/portal`.

**Promotion path** (iceberg → cold → warm → hot):

- Iceberg → cold: rare; restoration via adapter on explicit demand (audit; legal hold response; cross-deployment cross-pollination request)
- Cold → warm: a fracture or `@fate` trajectory requires the crystal's vector projection; the engine reads from Nix store, projects to vector, writes to Postgres
- Warm → hot: trail strength + access frequency cross promotion threshold; the engine reads from Postgres, materializes in Mnesia

**Demotion path** (hot → warm → cold → iceberg):

- Hot → warm: no active foraging on the crystal for a TTL window; the engine flushes Mnesia state to Postgres, frees in-memory slot
- Warm → cold: trail strength drops below threshold; the engine writes the projected vector back into a cold-tier crystal's fragmentation metadata; Postgres row evicted
- Cold → iceberg: Nix GC would normally collect this crystal; the iceberg adapter captures it first with retention metadata; Nix GC proceeds; crystal persists in deep archive

**Garbage collection at cold tier:**

Nix's GC handles ultimate eviction at the cold-tier level. The engine maintains GC roots for crystals with non-zero trail strength. When trail strength reaches zero AND evaporation curve permits, the engine's pre-GC hook captures the crystal to the iceberg tier (with retention metadata) before releasing the GC root. Nix GC then collects the orphan from the cold tier; the crystal persists in iceberg.

## Convergence-proof integration

*Altitude: proof grounding; the proof works tier-aware.*

The Lyapunov function (total holonomy) integrates across all three tiers:

- Hot tier holonomy = unresolved tensions in active foraging
- Warm tier holonomy = unresolved tensions in projected vectors
- Cold tier holonomy = unresolved tensions in archaeological substrate

**Tournament rounds** (per the §10 tournament-level Lyapunov framing in `gap-tension-tensor-substrate.md`) reduce hot+warm holonomy by direct action. **Pheromone evaporation** reduces warm+cold holonomy by demotion. The monotone-decrease integrates.

Nix's GC is provably terminating (reference counting + reachability are decidable). The cold-tier component of the Lyapunov function has formal lower bounds from Nix semantics: bounded below by GC-reachable closure size, which is finite by construction.

## Garden integration — super-colony coordination

*Altitude: proposed; depends on `@spectral/garden` (#83) landing.*

The garden (super-colony level) coordinates cross-deployment pheromone fields via Nix binary caches:

- A vetted scene-crystal at `silicon-venue.world` lives in that deployment's Nix store
- The deployment's binary cache is published; other deployments subscribe
- A second deployment (e.g., a local lambda-shell) substitutes from the binary cache to pull the crystal
- The crystal arrives with pheromone metadata (foraging history; trail strengths; provenance)
- The second deployment's engine integrates the pheromone state; local foraging continues

The garden becomes a Nix binary cache + provenance layer + scene catalog. Three open substrates composed; zero new wire protocols invented.

## Biology-typed coordination

*Altitude: depends on `@epistemologic/reality/biology` substrate landing; #82 + new biology task pending.*

With `@epistemologic/reality/biology` declared (the encoding that lifts ant-colony semantics from metaphor to substrate type):

- `pheromone_trail` type carries strength + decay curve + provenance
- `colony` type wraps a deployment's tier stack
- `super_colony` type wraps a garden's cross-deployment coordination
- `foraging` type wraps a `@fate` trajectory through the substrate
- `evaporation` operation maps to tier demotion
- `reinforcement` operation maps to tier promotion + trail-strength increment

The metaphor dissolves into substrate types. Engineers reading the architecture find tier names + caching mechanics. Biologists reading the same architecture find colony + pheromone + foraging. The render-target system handles the cross-domain reading.

## Tombstone mechanism — forgetting is visible

*Altitude: proposed; structurally required by Merkle/OID architecture; sketched not specified.*

Alex 2026-05-26: *"If we forget. We make it visible."*

The substrate is Merkle/OID-rooted. Content-addressing means crystals can't be silently disappeared — references to a deleted crystal's OID would become orphan hashes with no explanation. The substrate's response: **tombstones**. Every deletion leaves a typed crystal that records the absence.

**Tombstone shape:**

```mirror
type tombstone = {
  original_oid:        oid,
  deleted_at:          time.instant,
  reason:              deletion_reason,
  authority:           peer,
  jurisdiction:        text,
  retention_class:     text,
  pheromone_strength_at_deletion: number,
  encryption_destroyed: bool,
}

type deletion_reason =
  | retention_horizon_reached(policy_class: text)
  | gdpr_right_to_erasure(request_id: text, controller: peer)
  | gc_eligible(last_referenced_at: time.instant)
  | court_order(order_id: text, jurisdiction: text)
  | curator_decision(reason: text)
```

**Lookup semantics:**

When any consumer dereferences an OID:

1. Engine queries hot → warm → cold → iceberg in promotion order
2. If found at any tier: return the crystal
3. If not found at any tier: query tombstone-index by `original_oid`
4. If tombstone exists: return the tombstone (NOT a 404 / NOT a silent gap)
5. If neither found: this is an unknown-OID error (different from forgotten-OID)

The substrate distinguishes three states of any OID:

- **Live** — crystal exists at some tier
- **Tombstoned** — crystal was deleted; tombstone records when, why, by whom
- **Unknown** — OID was never in this substrate; provenance error

**Why this is structurally right:**

- **Merkle architecture forbids silent deletion.** Cryptographic references to OIDs are forever; the substrate must respond to every dereference with truthful information. Tombstones make "was here; isn't" first-class.
- **GDPR right-to-erasure becomes clean.** The original data is cryptographically destroyed (key destruction on encrypted iceberg blobs; physical tape destruction; etc.). The tombstone records that the destruction happened. The tombstone itself is NOT personal data — it's metadata about deletion. Compliance is structural.
- **Audit trail is complete.** Every deletion is queryable. The deletion authority is recorded. The jurisdiction is recorded. The reason is recorded. The retention class is recorded.
- **Cross-deployment honesty.** When a subscribed deployment tries to substitute a crystal from the garden cache, and the source has tombstoned it, the subscriber receives the tombstone. The pheromone trail evaporated; the substrate-level truth (this was deleted; here's why) is preserved.
- **"We don't forget" becomes "we mark what we forgot."** Honest by construction.

**Convergence with the live research agent's finding:**

The ant-colony research agent (commit `3a07753`) independently surfaced this: *"influence decay, not deletion (Merkle/OID architecture forbids the latter)"* — staged as a future spec `kintsugi-influence-decay.md`. Same substrate truth from two directions: Alex's compliance/GDPR framing meets the research agent's Merkle/architecture framing. Both arrive at: the substrate cannot literally delete; it can mark what's gone. Tombstones make the marking typed and queryable.

**Tombstone tier residence:**

Tombstones live in the cold or iceberg tier (they're rarely accessed; they're forever; they're small). They don't promote to hot/warm under normal use. They're queried on dereference of forgotten OIDs.

The tombstone-index itself is a content-addressed crystal containing all tombstones the deployment knows about — it acts as the deployment's "book of forgetting." Its OID can be exported, audited, and verified independently.

**Cryptographic destruction + tombstone composition:**

For GDPR right-to-erasure:

1. Original crystal is encrypted at rest (iceberg tier; encrypted-blob adapter)
2. Erasure request arrives — e.g., user invokes their GDPR right
3. Engine destroys the encryption key (key custody is separate from blob custody; split-trust model)
4. Engine writes tombstone with `encryption_destroyed: true` and `reason: gdpr_right_to_erasure(...)`
5. Iceberg blob remains on tape/cloud/decentralized storage but is now cryptographically inaccessible
6. Tombstone is queryable; original data is provably unrecoverable

This is structural GDPR compliance: deletion is verifiable; the audit trail is complete; the cryptographic floor protects against cloud-provider "oops we kept a copy" issues.

**Design calls open:**

- Tombstone-index sharding: per-deployment, per-tenant, per-jurisdiction?
- Tombstone retention: do tombstones themselves have retention horizons, or are they permanent?
- Tombstone visibility: who can query the tombstone-index? (Usually the deployment's curators + auditors + the original data subject for GDPR-erasure verification.)
- Tombstone propagation: how does a tombstone reach subscribers of the garden's binary cache? (Likely: tombstones are themselves crystals; they substitute via Nix binary cache like any other.)

## Iceberg adapter design considerations

*Altitude: proposed; adapter pattern sketched but not specified.*

Each iceberg adapter must:

1. **Speak `@spectral/portal`** — same wire as other adapters
2. **Implement content-addressed retrieval** — lookup by crystal OID; backend-specific latency
3. **Preserve cryptographic verification** — the crystal's hash must verify after restoration; encryption-at-rest is the adapter's call
4. **Support retention metadata** — ingestion timestamp; retention policy class; jurisdiction tags; original deployment ID
5. **Provide audit log** — every iceberg operation (ingest; restore; destroy) is logged for compliance
6. **Handle key management** — for encrypted-blob adapters, key custody is separate from blob custody (split-trust model)
7. **Support multi-replica** — decentralized adapters can replicate across nodes; tape adapters can support multiple physical copies

**Cryptographic destruction (for retention horizon enforcement):**

When a crystal's retention policy expires:
- For encrypted-blob adapters: destroy the encryption key; blob is now unreadable; equivalent to deletion
- For unencrypted-tape adapters: physical tape destruction (or designated decommissioning)
- For decentralized adapters: cryptographic proof of destruction across replicas (varies by network)

This gives mirror's iceberg substrate honest "right-to-erasure" support without trusting cloud providers' deletion claims.

## Substrate dependencies

- `@fragmentation/frame` (existing) — cold-tier crystal carriers
- Nix / nix-darwin / NixOS (existing infrastructure) — cold-tier blob storage
- Postgres + pgvector (existing infrastructure) — warm-tier vectors + pheromones
- Mnesia (existing infrastructure) — hot-tier in-memory
- LTO tape drives / AWS Glacier / Filecoin / Storj (existing infrastructure) — iceberg-tier deep archive backends
- `@spectral/portal` (declared per #77; impl in #78) — wire protocol between adapters and engine
- `@spectral/db/{mnesia, sql/postgres, sql/lite}` (declared in `@spectral` namespace insight) — the adapters
- `@time.duration` (per #76) — evaporation curve timing
- `@scene` (proposed per #92) — hot-tier scene state shape
- `@fate` (proposed; baseline per #88) — active foraging agents
- `@code/beam/eaf` (per #66) — hot-tier runtime substrate
- `@spectral/garden` (proposed per #83) — super-colony coordination layer
- `@epistemologic/reality/biology` (proposed; new task) — biology-typed primitives

## Open design calls

1. **Promotion/demotion threshold parameters** — fixed values, learned per deployment, or curator-configured per scene?
2. **Evaporation curve shape** — exponential decay (canonical), power-law, or task-specific (e.g., foraging history weights)?
3. **Cross-tier consistency model** — eventual consistency between tiers (likely), or stronger guarantees on specific operations?
4. **Garden subscription model** — pull-based (deployment requests; cache serves) vs push-based (cache notifies subscribers); how trust is established between deployments
5. **Pheromone state distribution format** — how `pheromone_trail` metadata travels alongside the Nix store entry; likely as a sidecar derivation
6. **Engine implementation language** — BEAM-native (closest to Mnesia + likely the hot path) vs Rust (cross-platform; matches existing mirror bootstrap) vs hybrid
7. **Iceberg adapter default** — which iceberg backend ships first / by default (likely a local-filesystem adapter for development, with Glacier/tape/decentralized as production options)
8. **Retention policy expression** — declared per scene type? Per crystal? Per deployment-policy? Per regulatory framework with predefined classes (HIPAA/SOX/GDPR)?
9. **Key custody for encrypted iceberg blobs** — self-custodied; deployment-organization HSM; cross-party split-trust; third-party escrow

## Honesty markers

| Component | Status |
|---|---|
| Nix store | Existing infrastructure |
| Postgres + pgvector | Existing infrastructure |
| Mnesia | Existing infrastructure |
| LTO tape / cloud archive / decentralized storage backends | Existing infrastructure (each backend) |
| Iceberg tier orchestration | Proposed; adapter pattern sketched; not implemented |
| Retention policy expression | Proposed; design call open |
| Cryptographic destruction for right-to-erasure | Proposed; standard primitive (key destruction); orchestration pending |
| `@fragmentation/frame` | Declared substrate (existing) |
| `@spectral/portal` | Declared substrate (per #77; impl per #78) |
| `@spectral/db` engine | Proposed; not implemented |
| Tier-orchestration semantics | Proposed; not specified beyond this doc |
| Biology-typed coordination | Proposed; depends on `@epistemologic/reality/biology` |
| Garden cross-pollination via Nix binary caches | Proposed; protocol not specified |
| Convergence proof's tier-aware Lyapunov | Sketched in `gap-tension-tensor-substrate.md` §10; tier-integration not formally proven |

## Migration order (when implementation surfaces demand)

1. Cold tier: `@fragmentation/frame` already exists; add Nix-store backing
2. Warm tier: Postgres + pgvector adapter speaking `@spectral/portal`
3. Hot tier: Mnesia adapter speaking `@spectral/portal`
4. Tier-orchestration engine (the IP moat) — closed-source; reads three adapters
5. Iceberg tier: local-filesystem adapter first (development); then Glacier + tape adapters (production)
6. Pheromone-typed coordination — depends on `@epistemologic/reality/biology` substrate landing
7. Garden subscription protocol — depends on `@spectral/garden` (#83) landing
8. Retention policy declarations (per-scene / per-crystal / per-deployment) — depends on `@scene` (#92)
9. Cryptographic destruction primitives — standard libraries; orchestration via the iceberg adapter
10. Convergence proof's tier-aware Lyapunov — formal verification when all above land

## Provenance

- Alex 2026-05-26 — the three-tier architecture + Nix cold tier + biology-typed coordination
- Alex 2026-05-26 — "the garden coordination magic happens in spectral-db"
- Alex 2026-05-26 — "the blob storage just happens in... nix"
- Alex 2026-05-26 — "long tail we can have an iceberg type storage layer under nix"
- Alex 2026-05-26 — "the layers need a tombstone mechanism. If we forget. We make it visible."
- Research agent 2026-05-26 (`3a07753`) — independently surfaced "influence decay, not deletion (Merkle/OID architecture forbids the latter)"
- Earlier @spectral namespace insight (`docs/insights/2026-05-25-spectral-namespace-architecture.md`) — the open/closed split between portal, db, and adapters
- `mirror.spec` line 44 — `@mirror/store/nix` already declared
- Tasks #43, #48, #66, #76, #77, #78, #82, #83, #88, #92 — component dependencies

## Related

- `docs/specs/gap-tension-tensor-substrate.md` — the convergence proof + biology-substrate-pending references
- `docs/insights/2026-05-25-spectral-namespace-architecture.md` — the namespace structure
- `docs/insights/2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md` — `@fate` agents as ants (encoding pending)
- (Pending) `docs/insights/2026-05-26-ants-colonies-stigmergy-and-mirrors-tournament.md` — research agent in flight
