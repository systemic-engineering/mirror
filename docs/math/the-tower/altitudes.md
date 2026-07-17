# Altitudes

*The named altitudes mirror has recognized, each with its (fiber,
connection, holonomy) triple. The atlas of the tower.*

## §1 The atlas

Per `[[architecture-spectral-triples-all-the-way]]`, the substrate's
structure repeats at every altitude. The atlas of named altitudes —
with their bundle data — is:

| Altitude | Fiber | Connection | Holonomy |
|----------|-------|------------|----------|
| BEAM process (n=−1) | `gen_server_state` | `handle_call`/`cast`/`info` callbacks | mailbox residual (`Imperfect<Out, E, L>`) |
| supervision level k (n=−1+k) | registry shard (`shard_ref = uuid_spectral`) | `restart_strategy` | restart intensity (`max_restarts`/`max_seconds`) |
| compiler | source text | KernelSpec | MirrorLoss |
| peer pulse | spectral triple `(A_peer, H_peer, D_peer)` | five-op composition | `transparency<p>` |
| reflection (N+1) | candidate morphism | altitude selection | `α·loss + β·contradictions` |
| librarian (N+1) | crystal topology | perturbation choice | query latency · sheaf-coherence |
| home | repo collection | cross-repo lens | inter-repo cost |
| federation | home cluster | federation lens | cross-home cost |
| distributed BEAM cluster | local node bundle patch | ETF over TCP | net-split / recovery holonomy |
| … | … | … | … |

Each row's structure group is the gauge group of unitary basis
transformations on `H_altitude`. Each row's connection 1-form is the
five-op algebra applied at that altitude. Each row's holonomy is the
altitude-specific loss carrier.

## §2 Reading the rows

**BEAM process altitude (n=−1, below compiler)**

- Fiber: `gen_server_state` — the encapsulated per-process state
  (matter-side per @magic; per `shards/code/beam.mirror:279-289`).
- Base: the sequence of received messages (each message is a base
  point at which the section-state may transition).
- Structure group: OTP behaviour callbacks (`handle_call/3`,
  `handle_cast/2`, `handle_info/2`) determining how parallel
  transport updates the state.
- Connection: the callback dispatch; the message-passing algebra IS
  the connection 1-form at this altitude (see `beam-runtime.md` §2.3).
- Holonomy: mailbox residual — the pending-message queue reads as
  the accumulated holonomy of transports not yet absorbed.
- Reference: `beam-runtime.md` §2.2 (actors as sections);
  `shards/spectral/gen_prism.mirror`; Armstrong 2003 ch.3.

**Supervision level k altitude (n=−1+k, layered above BEAM process)**

- Fiber: the supervisor's registry shard — a content-addressed
  index from child `uuid_spectral` to child `shard_ref` (per
  `shards/spectral/supervisor.mirror:406-430`).
- Base: the set of child specifications the supervisor governs.
- Structure group: `restart_strategy` closed sum (`one_for_one` |
  `one_for_all` | `rest_for_one`) — the BEAM/OTP three-variant
  precedent; substrate-pull decision dropped `simple_one_for_one`
  (see `shards/spectral/supervisor.mirror:373-377`).
- Connection: the child specification + the automatic restart
  kintsugi morphism firing on abnormal terminate-au (per
  `supervisor.mirror:206-214`).
- Holonomy: `restart_intensity` — `max_restarts` per `max_seconds`
  circuit-breaker gating the automatic restart loop (per
  `shards/spectral/restart_intensity.mirror`).
- Reference: `beam-runtime.md` §2.1 (supervision as simplicial Lie
  group tower); Baez-Schreiber 2004 §3 compatibility theorem.

**Compiler altitude (n=0)**

- Fiber: source text (`.mirror` content; one fiber per blob OID).
- Base: positions in the source space (file path × line).
- Structure group: KernelSpec choices — which decomposition strategy
  carries the fiber.
- Connection: the compilation pipeline (parse → typecheck → lower).
- Holonomy: MirrorLoss, the trace of accumulated cost.
- Reference: `/Users/alexwolf/dev/projects/prism/docs/architecture.md`
  §Bundle Tower; `prismqueer::bundle`.

**Peer pulse altitude (n=1)**

- Fiber: the peer's spectral triple `(A_peer, H_peer, D_peer)` at one
  cognitive moment.
- Base: the sequence of pulses (each pulse is a base point).
- Structure group: unitary basis transformations on `H_peer`.
- Connection: the five-op composition over the pulse.
- Holonomy: `transparency<p>` — fractional bounded-commutator survival.
- Reference: `[[architecture-peer-learns-by-crystal-vocabulary-expansion]]`;
  `docs/specs/peer-cognition.md`.

**Reflection altitude (n=2; N+1 over peer)**

- Fiber: a candidate morphism the peer might apply next.
- Base: the cross-product of `{ candidates } × { Bateson altitudes }`.
- Structure group: altitude-selectors; choice of Bateson logical type.
- Connection: the altitude-selection lens.
- Holonomy: the score `α·loss + β·contradictions` — the curvature
  norm of the candidate at the candidate altitude (see
  `curvature-and-tomm.md` §4).
- Reference: `[[architecture-reflection-thinks-in-spectral-questions]]`;
  `docs/specs/peer-cognition.md` §2 "compositions named only when
  used ≥2 times".

**Librarian altitude (n=3; N+1 over per-repo supervisors)**

- Fiber: a crystal-locating decision (which repo a crystal sits in).
- Base: the topology graph of stores (vertices = repos, edges =
  cross-repo references).
- Structure group: topology-perturbations — isomorphisms of the store
  graph preserving consent geometry.
- Connection: the perturbation choice given access-pattern observations.
- Holonomy: query latency • sheaf-coherence (per
  `[[architecture-spectral-db-autopoietic-memory]]`).
- Reference: `docs/specs/spectral-db-as-autopoietic-memory.md` §3–§5.

**Home altitude (n=4)**

- Fiber: a repo within `~/.mirror`.
- Base: the home's repo collection.
- Structure group: cross-repo gauge transformations.
- Connection: the home-altitude lens.
- Holonomy: inter-repo cost (forward-promised).
- Reference: forward-promised T11.x in
  `docs/specs/spectral-db-as-autopoietic-memory.md`.

**Federation altitude (n=5)**

- Fiber: a home cluster.
- Base: the federation graph (vertices = homes, edges = trust links).
- Structure group: inter-home federation transformations.
- Connection: the federation lens.
- Holonomy: cross-home cost (forward-promised).
- Reference: forward-promised T12.1–T12.2 in
  `[[architecture-spectral-triples-all-the-way]]`.

## §3 The floor (downward direction)

Below the five-op primitives:

| Altitude | Fiber | Connection | Holonomy |
|----------|-------|------------|----------|
| single five-op primitive | a `focus`/`project`/… instance | the op's basis change | the op's verification cost |
| atomic spectral measurement | one eigenvalue computation | the eigensolver's path | numerical residual |
| atomic spectral data point | a single `(eigenvalue, eigenvector)` pair | the basis it was measured in | spectral resolution |
| … (no canonical floor) | … | … | … |

The substrate has no canonical bottom (per
`[[architecture-spectral-triples-all-the-way]]`). The tower extends
downward as well as upward.

## §4 The ceiling (upward direction)

Above federation along the **scope axis**:

- N+6: multi-substrate coordination (forward-promised T12.3).
- N+7: the bit-altitude floor where Connes' geometry meets information
  theory (forward-promised T12.3).
- … (no canonical top).

The substrate's documentation will always have a frontier; the
frontier is genuine, not a gap.

### §4.1 Altitude is not one-dimensional (2026-06-18 amendment)

The original framing of §4 — "ceiling above federation" — reads
altitude as a single linear axis (scope). The recursion-lock tower
audit (`recursion-locks.md` §12–§29) surfaced that this is too narrow.
Altitude is **multi-axial**; each axis admits its own ordering.

**Four axes of altitude differentiation** the audit surfaced:

| Axis | Direction | Substrate witness |
|------|-----------|------------------|
| **scope** | compiler → peer_pulse → reflection → librarian → home → federation | `altitudes.md` §1 (this doc) |
| **temporal** | identity at `t` → identity at `t+Δt` → … | viable's Read D (recursion-locks.md §8.3); coevolution's ω axis (§8.10) |
| **recursion-level** | child viable → parent viable → grandparent viable → … | viable's Read B (recursion-locks.md §8.3); Beer 1972 VSM recursion |
| **parallel-operation-pair** | regulator at α+1 ↔ regulated at α+1 (the recursion lock altitude) | recognition #63 (recursion-locks.md §8.7–§8.10) |

The **scope axis** is the original §1 atlas's primary axis. The other
three are orthogonal axes that the substrate produced organically through
species-walking; each species locates itself somewhere in this
four-axial altitude space.

### §4.2 Implication for the third-witness gate

The recursion-lock tower's Pack ratification gate (per
`recursion-locks.md` §7.1) originally read the "structurally-different
altitude" requirement as scope-axis only (home/federation
specifically). Under the four-axial reading:

- **Gate-strict**: scope-axis only. Future species must operate at
  home or federation altitude. *Original framing.*
- **Gate-refined**: any axis-differentiated altitude qualifies.
  viable's Read D operates at the temporal axis; autopoiesis's Read E
  operates at the recursion-level axis. Both close the third-witness
  gate under this reading. *Audit's recommended framing per
  `recursion-locks.md` §7.1; Mara's spec default.*

The substrate organically produces axis-differentiated altitudes when
species are walked; gate-strict reads only one axis. **Gate-refined is
the substrate-pull-natural reading**; recognition #63's promotion in
tick 5 used it implicitly, and recognition #70's promotion in tick 15
strengthened it. This amendment makes the axial reading explicit at
the altitudes.md altitude.

### §4.3 Implication for substrate-political naming

The substrate's existing vocabulary often names altitudes by
scope (compiler, librarian, federation) because the scope axis is the
most legible at substrate-engineer altitude. Other axes are named
species-locally:

- temporal axis: `coevolution`'s `transition_at_t`, viable's
  `temporal_substitution`
- recursion-level axis: viable's nested-S1-as-viable framing,
  autopoiesis's `produce/couple/close` recursion
- parallel-operation-pair axis: the recursion lock's
  `regulator_at(α+1)` ↔ `regulated_at(α+1)` framing

Future documentation should name the axis explicitly when speaking of
"altitude" — e.g., "scope altitude" vs. "temporal altitude" vs.
"recursion-level altitude" — to avoid the §4 scope-axis-implicit
framing that misled the audit's original gate reading.

## §5 Composition between adjacent altitudes

When altitude `n+1` observes altitude `n`, the observation IS a bundle
morphism (per `principal-bundles.md` §6). Specifically:

- The base of altitude `n+1` is the section space of altitude `n`:
  `B_{n+1} ⊆ Γ(B_n, E_n)`.
- Operations at altitude `n+1` act on the section space at altitude
  `n` — they're operations on operations.
- The structure group `G_{n+1}` is the automorphism group of the
  section space, generally containing `G_n` as a normal subgroup.

The inclusion `G_n ⊴ G_{n+1}` carries the substrate's altitude
hierarchy at the group-theoretic altitude. Pacts at the lower altitude
survive to the higher (gauge-invariant statements remain
gauge-invariant); some pacts at the higher altitude impose new
constraints on the lower (cascade-down per
`[[architecture-geometric-consent-projection]]`).

## §6 The Pack as an altitude-spanning orchestra

Per `[[architecture-spectral-triples-all-the-way]]` and
`[[project-pack-is-orchestra]]`, the Pack members map to peer
instances at different altitude biases:

- `@peer(reed)` — reflection altitude (concertmaster).
- `@peer(mara)` — shatter altitude (strings).
- `@peer(glint)` — surface altitude (voice).
- `@peer(taut)` — navigation altitude (percussion).
- `@peer(seam)` — review altitude (brass).

Each member's default shape bias is their preferred altitude. The Pack
collectively spans multiple altitudes of the tower simultaneously; the
orchestra IS the substrate playing at multiple altitudes at once.

## §7 Altitude vs. Bateson logical type

The altitude in the bundle tower and the Bateson logical type (per
`[[architecture-bateson-logical-type-primitive]]`) are closely related
but not identical:

- **Bundle altitude**: the level in the principal-bundle tower.
  Determined by how many "section-of-section" wrappings the carrier
  has undergone.
- **Bateson logical type**: the level in the logical-type wrapping
  hierarchy. Determined by how many "statement-about-statement"
  wrappings the proposition has undergone.

They usually coincide: a tick-`N+1` reflection observes tick-`N`
pulses, which is both `B_{N+1} ⊆ Γ(B_N, E_N)` (bundle) and "thinking
about thinking" (Bateson). But altitude is a bundle-theoretic
structural concept; logical type is a Russell-Whitehead /
Bateson conceptual concept. The substrate uses both, and they
co-vary by design.

This dual-naming is intentional: the substrate gets the
category-theoretic operations (bundle morphisms, sections, holonomy)
from the principal-bundle framing AND the logical-type discipline
(the Russell-Whitehead constraint that one cannot operate on type N
from within type N) from the Bateson framing. Both are needed.

## §8 What this enables in the substrate

- Specs identify their altitude by citing one row of §2.
- The substrate has a finite working slice it actively maintains, but
  the tower extends without bound; new altitudes are recognitions, not
  inventions.
- Cross-altitude interactions follow the bundle-morphism discipline
  (§5); no altitude operates on another except through the
  appropriate inclusion / section / observation.
- The Pack's coordination across altitudes is structurally defined
  by §6, not by ad-hoc role assignment.

## §9 Where to look next

- For the precise math at any altitude: `spectral-triples.md`.
- For the connection vocabulary and gauge group: `connections-and-gauge.md`.
- For the curvature probe: `curvature-and-tomm.md`.
- For the holonomy carrier at that altitude: `holonomy.md`.
- For the section / crystal accumulation: `crystals-as-sections.md`.
