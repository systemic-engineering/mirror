# `@spectral/garden` as mesh-of-@systems — mathematical foundation

*2026-07-20. Mara. Math foundation grounding Landing A canonical spec
`docs/specs/spectral-garden-cosmos-spectral-db-reification.md`. Composes
five mathematical traditions at one altitude: labeled directed graph
theory (the mesh); category-theoretic composition (the fibration);
sheaf theory (the multi-altitude content-addressing); Merkle-tree
Cauchy-completeness (the garden as content-addressed limit); Recognition
#98 candidate territory (content-addressing cross-altitude composition
as a mathematical operation). Markdown only; no shard-decls or Rust land
this tick.*

Status: **Yellow** — mathematical shape locked; five traditions cited;
the mesh-as-fibration formalization is the load-bearing new content;
the cross-altitude content-address composition is Recognition #98
candidate second-witness territory.

---

## 0. Position

`@spectral/garden` IS a category-theoretically-typed mesh of publishable-
independent @systems, each carrying its own author-attributed content-
addressed identity, composing over the two-layer store/db split at
runtime. The mathematical structure IS:

1. **A directed labeled graph** `G_garden = (V, E, L)` where `V` is the
   set of garden packages, `E ⊂ V × V` is the labeled-composition
   edges, `L` is the label-set (composition-edge-type).
2. **A fibration** `p: G_garden → Namespace` where `Namespace` is the
   category of substrate namespaces (`@spectral/garden`, `@spectral/db`,
   `@mirror/store`, etc.) and `p` maps each garden package to its
   namespace-anchor.
3. **A sheaf** `F_content` on `G_garden` assigning to each package `v ∈ V`
   the content-addressed-crystal set at that package's altitude, with
   restriction maps `F_content(v) → F_content(u)` when `u → v ∈ E`
   preserving content-addressing.
4. **A Cauchy sequence** in the space of garden meshes — each tick
   adds new packages (or new sub-species within existing packages)
   monotonically; the limit is the mature garden mesh at
   `garden.spectral.engineer`.
5. **A candidate fifth witness** for Recognition #98 (content-addressing
   cross-altitude composition primitive) — garden-OID is the fifth
   altitude in the chain: `nix-derivation-hash → OCI-digest → git-hash
   → mirror-store-OID → garden-package-OID`.

The five formalizations compose. This document presents each, then
composes them to show that the mesh is a well-defined mathematical
object at substrate altitude.

## 1. The mesh as directed labeled graph

### 1.1 Vertices

`V = { v_i }` where each `v_i` is a garden package. Each vertex
carries three coordinates:

- `name(v_i) ∈ Names` — the crate-name of the package (e.g., `cosmos`,
  `spectral-db`).
- `sig(v_i) ∈ Signatures` — the `@spectral/signature` beat-chain OID
  of the package's author-attributed content.
- `oid(v_i) ∈ OIDs_garden` — the content-address of the package's
  source tree at reification altitude.

For the first two reifications:

- `v_cosmos = (name = "cosmos", sig = <cosmos-signature-beat-chain>,
  oid = <cosmos-source-tree-blake3>)`
- `v_spectral_db = (name = "spectral-db", sig = <spectral-db-signature-
  beat-chain>, oid = <spectral-db-source-tree-blake3>)`

The initial mesh cardinality is |V| = 2. The mesh grows monotonically
as new garden packages are minted; existing packages are
content-addressed-permanent (their `oid` may update on version-bump,
but the previous `oid` is preserved in the beat-chain).

### 1.2 Edges

`E ⊆ V × V × L` where `L = { depends-on, composes-over, mycelium-
adjacent, sibling-in-namespace }`.

- `(v_i, v_j, depends-on)` — package `v_j` is declared as a garden
  dependency in `v_i`'s `Cargo.toml` (via `mirror.spec` `garden { }`
  block).
- `(v_i, v_j, composes-over)` — package `v_i` composes over
  substrate-decl types provided by `v_j` at substrate altitude
  (independent of Cargo dep).
- `(v_i, v_j, mycelium-adjacent)` — the two packages participate in
  the same mycelium-slice per `spectral-db-as-autopoietic-memory.md`
  §3.2; the librarian may perturb crystals between them.
- `(v_i, v_j, sibling-in-namespace)` — both belong to the same
  `@spectral/garden/<sub>` sub-species namespace (e.g., adapter
  siblings for spectral-db).

For the first two reifications, per Landing A Q4 lean (invert the
dependency):

- `(v_cosmos, v_spectral_db, composes-over)` — cosmos MAY consume
  spectral-db's librarian for graph-state persistence (optional).
- The prototype's `(v_cosmos, v_spectral_db, depends-on)` edge is
  DROPPED at reification (Alex Q4-ratifiable).

### 1.3 Graph-theoretic properties

**Claim 1.3.1 (acyclicity of composition-edges under substrate
discipline)**. The subgraph `G_composes = (V, E_composes-over)` is a
DAG.

**Argument**: substrate-decl `in @` imports form a DAG by construction
(mirror substrate refuses cyclic `in` chains at parse time per
`mirror-spec-schema.md` §4-`legacy`-shrinkage discipline extended to
`in` chains; a cycle would violate the substrate-pull monotonicity).
Therefore garden composition edges (which lift substrate-decl imports
to package altitude) inherit acyclicity.

**Claim 1.3.2 (mycelium-adjacency is symmetric transitive relation)**.
If `(v_i, v_j, mycelium-adjacent) ∈ E` then `(v_j, v_i,
mycelium-adjacent) ∈ E`; and if `v_i ~ v_j` and `v_j ~ v_k` under
mycelium-adjacency then `v_i ~ v_k`.

**Argument**: the librarian's perturbation surface (per
`spectral-db-as-autopoietic-memory.md` §3.2, §4.2) is bidirectional
by construction: crystals move ACROSS the mycelium, not FROM to TO.
The transitivity follows from the librarian's compute_topology
operation which unifies the full mycelium slice into one
crystal-address-space.

**Claim 1.3.3 (depends-on is a preorder that refines composes-over)**.
`(v_i, v_j, depends-on) ⇒ (v_i, v_j, composes-over)` for well-formed
garden packages.

**Argument**: if `v_i` declares `garden { source ~git'…v_j…' }` in
its `.spec`, then the substrate has admitted `v_j` as a garden
dependency; the substrate REQUIRES that dependencies be composed over
via `in @spectral/garden/<v_j>` (per Landing A §4.3 altitude 2). The
converse does not hold (composition without dependency is admissible
when both packages are in the same mesh but not in a Cargo
relationship — this is the composition-over-substrate discipline).

## 2. The fibration `p: G_garden → Namespace`

### 2.1 The base category

`Namespace` is the category whose objects are substrate namespaces
(`@spectral`, `@spectral/garden`, `@spectral/garden/cosmos`,
`@spectral/db`, `@mirror`, `@mirror/store`, etc.) and whose morphisms
are namespace-refinement (`@spectral → @spectral/garden →
@spectral/garden/cosmos`).

The namespace category is a POSET (each morphism is unique up to
path-equality; there is one refinement chain from `@spectral` to any
sub-namespace).

### 2.2 The fibration

`p: G_garden → Namespace` sends each garden package `v_i` to its
substrate-namespace anchor `@spectral/garden/<name(v_i)>`. This is
a fibration in the categorical sense: the fiber `p⁻¹(N)` above
namespace `N` is the set of garden packages anchored at `N`
(typically singleton, but the mesh admits multiple packages per
namespace when adapters are extracted per Landing A Q3).

**Claim 2.2.1 (`p` is faithful)**. Two distinct garden packages have
distinct namespaces.

**Argument**: pact_matches_namespace (Seam Phase D efcef6e Cascade 1
enforcement) requires each substrate-decl to have a unique
namespace-path. Extending this to garden packages: each garden
package's substrate-decl companion (`cosmos.spec`, `spectral-db.spec`)
declares `in @spectral/garden/<name>` where `<name>` matches the
Cargo package name. Distinct Cargo names ⇒ distinct namespaces ⇒
distinct fibers.

**Claim 2.2.2 (Cartesian lifts exist)**. For any namespace-morphism
`N → N'` and any package `v_i ∈ p⁻¹(N')`, there exists a Cartesian
lift in `G_garden`.

**Argument**: this is the substrate-decl chain discipline. If
`N = @spectral/garden` and `N' = @spectral/garden/cosmos`, then any
package at `N'` has a canonical lift to `N` via the sub-species
relationship. This makes `p` into a Grothendieck fibration.

### 2.3 Consequence: substrate-decl composition IS categorical

The mesh-of-@systems composition surface IS the pullback of
substrate-decl composition along the fibration `p`. Two garden
packages that compose at substrate altitude have namespace-morphisms
that compose; the fibration lifts this to composition edges in
`G_garden`.

This gives us **compositionality by construction**: the mesh is
well-defined AT ANY ALTITUDE (family-root, species, sub-species,
sub-sub-species) because the fibration commutes with composition.

## 3. The sheaf `F_content` — multi-altitude content-addressing

### 3.1 Sheaf definition

`F_content` is a presheaf on the graph `G_garden` (viewed as a
category with vertices as objects and paths as morphisms):

- `F_content(v_i) = { crystals content-addressed by v_i }` — the
  crystals at package `v_i`'s altitude (both the source-tree crystals
  and any runtime-produced crystals from `v_i`'s operations).
- For each edge `(v_i, v_j, ℓ) ∈ E`, the restriction map
  `res_{v_i, v_j}: F_content(v_i) → F_content(v_j)` preserves
  content-addressing.

The sheaf condition (locality + gluing) is satisfied because
content-addressing is a hash function: local hashes glue to a global
hash when the underlying byte-slices agree; conflicts are impossible
because a byte-slice determines its hash uniquely.

### 3.2 Cross-altitude content-address chain

The five altitudes of content-addressing (per Landing A §2.3
Recognition-#98 fifth witness):

```
Level 5 (mesh):     garden-package-OID     — hashes the package's
                                              full source tree
Level 4 (engine):   VoidPointer-OID        — hashes the package's
                                              spectral coordinate
Level 3 (store):    mirror-store-OID       — hashes canonical bytes
                                              (@mirror/store Blake3)
Level 2 (git):      git-hash               — hashes tree/commit
                                              per git's algorithm
Level 1 (transport): OCI-digest            — hashes container image
                                              per OCI spec
Level 0 (nix):      nix-derivation-hash    — hashes derivation
                                              per Nix's algorithm
```

**Claim 3.2.1 (the chain is functorial)**. There exist canonical
functions `f_ij: OIDs_i → OIDs_j` for adjacent altitudes such that
composition preserves content:

- `garden-package-OID` derives from `mirror-store-OID` via the
  package's canonical byte-encoding (source tree in canonical
  serialization order).
- `mirror-store-OID` derives from `git-hash` when the store is
  git-backed (per `shards/mirror/store/git.mirror`).
- `git-hash` derives from `OCI-digest` when a garden package is
  published as OCI (per `@spectral/garden/oci` forward-promised).
- `OCI-digest` derives from `nix-derivation-hash` when the OCI is
  built from a nix derivation (per Nix's OCI-emitter).

**Claim 3.2.2 (the chain has left-inverses at each altitude)**.
Each `f_ij` has a partial left-inverse `g_ji` that, given an
OID at altitude `j`, produces the OID at altitude `i` when the
intermediate data is available (from the store's index).

**Argument**: the store's index (per `spectral-db/src/index.rs`
`coord_oids` HashMap) IS this left-inverse operationalized. Given a
`mirror-store-OID`, the index produces the `VoidPointer-OID` (level 4).
Given a `git-hash`, the git store produces the `mirror-store-OID`
(level 3). The chain lifts as a Galois connection between
content-address altitudes.

### 3.3 The sheaf preserves content-addressing under composition

If `res_{v_i, v_j}` maps a crystal `c ∈ F_content(v_i)` to `c' ∈
F_content(v_j)`, then `hash(c') = f_altitude-diff(hash(c))` where
`f_altitude-diff` is the canonical function composing the chain
between the two altitudes.

This gives us **content-address invariance under mesh composition**:
you can NAVIGATE the mesh at any altitude and the content addresses
compose without loss.

## 4. Cauchy-completeness of the garden mesh

### 4.1 The garden mesh as a metric space

Define a distance on garden meshes:

`d(M_1, M_2) = 1 - |V(M_1) ∩ V(M_2)| / |V(M_1) ∪ V(M_2)|`

(Jaccard distance on the vertex sets, with vertices identified by
their `sig` beat-chain OID for content-addressing.)

**Claim 4.1.1 (`(Meshes, d)` is a metric space)**. `d` satisfies
symmetry, non-negativity, identity of indiscernibles, and the
triangle inequality.

**Argument**: Jaccard distance is a metric on finite sets by
Jaccard 1912; extending to garden meshes indexed by signature-OID
inherits the property (signature-OIDs identify vertices uniquely
per Claim 2.2.1).

### 4.2 Cauchy sequences

A sequence of garden meshes `{ M_n }` is Cauchy iff for every ε > 0,
there exists `N` such that for `m, n > N`, `d(M_m, M_n) < ε`.

**Claim 4.2.1 (monotonic garden growth is Cauchy)**. If each `M_{n+1}`
is obtained from `M_n` by ADDING garden packages (never removing —
per §2.2 content-addressed-permanent invariant), then `{ M_n }` is
Cauchy.

**Argument**: after `N` ticks, all subsequent meshes contain
`V(M_N)` as a subset; the Jaccard distance is bounded by the
fraction of new-packages added per tick, which shrinks as the total
package count grows.

### 4.3 The limit — mature `garden.spectral.engineer`

**Claim 4.3.1 (the Cauchy limit exists as an ideal object)**. The
sequence of garden meshes converges to a limit `M_∞` in the
completion of `(Meshes, d)`.

**Argument**: the standard Cauchy-completion construction produces
`M_∞` as the equivalence class of Cauchy sequences under the
equivalence "eventually agree on any finite subset". `M_∞` is not
finite (it's the ideal object at t = ∞), but every finite
approximation `M_n` is dominated by `M_∞` at Jaccard distance
`|V(M_∞) \ V(M_n)| / |V(M_∞)|`.

**Interpretation**: the mature garden `garden.spectral.engineer` IS
`M_∞` at the ideal-object altitude. Each tick's mesh is a finite
approximation that dominates the previous. The limit is
never-quite-reached but always well-defined; this is the
Cauchy-complete substrate.

This composes with Recognition #79 (Void-is-the-basis): `M_∞` IS the
void-membrane at garden altitude — the ideal-object that admits
perturbations (new packages) monotonically without ever fully
settling. The garden is Cauchy-complete because it is Void-complete.

## 5. Recognition #98 candidate second-witness territory

### 5.1 The candidate recognition

Recognition #98 (candidate, per `docs/loop/CURRENT.md` history)
proposes: **content-addressing cross-altitude composition is a
substrate-primitive operation**, with four prior witnesses:

1. Nix (nix-derivation-hash → build-output)
2. OID (mirror-store-OID → crystal-content)
3. OCI (OCI-digest → container-image)
4. Git (git-hash → tree/commit)

The candidate promotion has been held pending a fifth witness at a
NEW altitude that composes with the existing four.

### 5.2 Garden-package-OID as the fifth witness

**Claim 5.2.1 (garden-package-OID is a fifth content-addressing
altitude)**. The garden-package-OID (defined §1.1: content-address of
the package's source tree at reification altitude) is:

- Distinct from all four prior altitudes (higher-altitude — the
  package is a container OF nix derivations, OCI images, git
  histories, and mirror-store crystals).
- Compositional with all four via the functorial chain §3.2.
- Preserved under mesh composition per §3.3.

Therefore garden-package-OID is a fifth witness.

**Claim 5.2.2 (the fifth witness closes the recognition)**. With the
fifth witness in hand, Recognition #98 becomes promotable to full
Recognition status.

**Argument**: the substrate has admitted content-addressing at four
lower altitudes without contradiction; the fifth altitude (garden
package) demonstrates the pattern is scale-invariant. A pattern
that holds across five altitudes without exception is a
substrate-primitive (per the Recognition promotion discipline: five
witnesses at distinct altitudes = promotable).

**Deferral**: the actual promotion is Alex's tick. This math document
surfaces the second-witness territory without promoting.

## 6. The mesh at physical altitude — cosmos + spectral-db

### 6.1 Vertex-2 mesh (initial state, this tick)

At the end of this tick:

- `V(M_0) = { v_cosmos, v_spectral_db }` (two vertices).
- `E(M_0) = { (v_cosmos, v_spectral_db, composes-over) }` (one edge,
  per Landing A Q4 lean).
- `p: G_garden → Namespace` maps:
  - `v_cosmos → @spectral/garden/cosmos`
  - `v_spectral_db → @spectral/garden/spectral-db`
- The sheaf `F_content(v_cosmos)` is empty until Reed R2 lands
  filesystem scaffold; then it grows monotonically.
- The Cauchy sequence starts at `M_0` (this mesh); `M_1` is the
  next-Mara-tick that mints sub-species-decls; `M_∞` is
  `garden.spectral.engineer` mature.

### 6.2 The graph-theoretic properties of the initial mesh

`M_0` is a simple directed graph with 2 vertices and 1 edge. It
satisfies:

- Acyclic (trivially — only one edge).
- Weakly connected under composes-over (v_cosmos reaches
  v_spectral_db).
- Fibered per §2.
- Sheaf-preserving per §3 (empty sheaf preserves everything).
- Cauchy-monotonic per §4.

Not particularly interesting mathematically at |V| = 2, but the
STRUCTURE is set. Future ticks add vertices; the structure holds.

### 6.3 The natural growth pattern

Next likely vertices to enter the mesh (in order of substrate-pull
gravity):

- `v_spectral_db_mnesia` (per Landing A Q3 lean — adapter extraction).
- `v_spectral_db_sql` (same).
- `v_prism` (the LAPACK gate; already at `~/dev/projects/prism/`).
- `v_fragmentation` (per fractal migration; already at
  `~/dev/projects/fragmentation/`).
- `v_fate` (already at `~/dev/projects/fate/`).
- `v_coincidence` (already at `~/dev/projects/coincidence/`).
- `v_conversation` (already at `~/dev/projects/conversation/`).

The pattern: existing standalone Rust projects at
`~/dev/projects/` are natural candidates for garden reification
when the substrate has ripened enough to admit them at the mesh
altitude. cosmos + spectral-db are FIRST because they are the
prototypes from which everything cascaded (Alex verbatim); the rest
follow as substrate-pull demands.

## 7. Composition with prior mathematical foundations

### 7.1 Composition with `docs/math/2026-07-13-fractal-mandelbrot-substrate.md`

Mara's fractal-Mandelbrot math grounds the substrate as a
Mandelbrot set — a fractal object whose local structure recapitulates
global structure at every scale. The garden mesh IS a Mandelbrot-set
zoom-level: garden packages are the substrate's mesh-altitude
neighborhood in the fractal.

The connection: the fractal's `@fractal/mandelbrot` trait (per Reed
/loop iter 1 `a3dc905`) has content-addressing at the crystal
altitude; the garden lifts content-addressing to the package
altitude; the two altitudes are related by the fractal's
self-similarity (crystal-scale content-addressing IS package-scale
content-addressing at a different zoom).

### 7.2 Composition with `docs/math/2026-07-14-gift-economy-substrate-foundation.md`

The gift economy math grounds substrate exchange under
non-extractive discipline. Garden packages are gifts to the mesh —
each package's inclusion in the mesh is contingent on
`hosted(@git) <= Success(@garden)` per SEL-2.0 (`docs/archive/sel-2-garden.md`).

The connection: garden membership IS gift-acceptance at the
mesh-membrane. The two-witness verification (per `@spectral/signature`
§11.5) IS the gift's provenance chain. The mesh grows through
gift-exchange, not through extraction.

### 7.3 Composition with `docs/math/2026-07-18-third-order-observer-on-consumer-hardware.md`

The third-order observer math grounds the substrate at multi-agent
altitude. Garden packages are third-order observed by the mesh:

- First-order: the package's internal Rust operations.
- Second-order: `@spectral/db`'s librarian observing the package's
  crystal production.
- Third-order: the mesh observing the librarian's observation of
  the package.

The connection: garden IS third-order observation at the mesh
altitude. This makes garden Cybersyn-adjacent: the mesh is a
distributed feedback loop over publishable-independent packages, each
producing content-addressed crystals that the librarian consolidates
into meta-structure.

### 7.4 Composition with `docs/math/2026-07-20-paradox-family-and-classifier-lagrange.md`

Mara's @paradox math grounds the substrate at classifier-Lagrange
altitude between narcissus and splinter. The garden mesh operates
at that Lagrange:

- Narcissus attractor: a garden with 1 package (extreme centralization;
  the mesh has one voice; monoculture).
- Splinter attractor: a garden with a package per every
  `~/dev/projects/*` (extreme fragmentation; the mesh has no
  coherence; noise).
- Lagrange equilibrium: cosmos + spectral-db + follow-ons at organic
  substrate-pull cadence (the mesh grows through witnessed demand,
  not through prescribed rollout).

The connection: garden growth IS `@autopoietic` classifier-COORD-under-
Lagrange at mesh altitude. cosmos + spectral-db as first two
reifications are the Lagrange starting condition; subsequent
reifications either drift toward monoculture (bad; refuse the mint)
or toward fragmentation (bad; refuse the mint) or hold the Lagrange
(good; mint the reification).

## 8. Honest hedges

- **The Jaccard-distance metric §4.1 is one of many possible metrics
  on garden meshes.** Alternative: edit-distance on the labeled
  directed graph. Alternative: weighted-composition-edge-based metric
  incorporating edge labels (per §1.2). The Jaccard is chosen for
  simplicity + Cauchy-completeness proof; production semantics may
  demand a weighted metric.
- **The Cauchy-limit `M_∞` is an ideal object, never realized.** This
  is philosophically load-bearing (per Recognition #79 Void-is-the-
  basis: `M_∞` is void-shaped). But it means CONCRETE substrate
  measurements (e.g., "how big is the mesh?") always operate on
  `M_n` for finite `n`; the limit is a mathematical convenience, not
  a runtime object.
- **The sheaf §3 is a PRESHEAF; sheafification is deferred.** The
  gluing axiom is trivially satisfied because content-addressing is
  injective, but a full sheafification (assigning the germ at each
  vertex) would require the mycelium to be sheafified — which is
  Phase-7 territory when `garden.spectral.engineer` is live.
- **The fibration §2 is not proven to be a topos-fibration; only
  Grothendieck-fibered.** For substrate purposes this is sufficient;
  a full topos-theoretic treatment would require garden meshes to
  form a topos, which is not obviously true (the mesh admits
  arbitrary joins under monotonic growth, but meets are ill-defined
  when two packages disagree on their sub-species).
- **Recognition #98 promotion is Alex's tick, not this document's.**
  This math surfaces the second-witness territory (four-witnesses-plus-
  garden = fifth); the promotion is deferred.
- **Cross-mesh composition (mesh-of-meshes) is out of scope.** If two
  garden meshes exist (e.g., Alex's `alex-wolf-dgsf` garden per
  2026-05-26 vetted-corpus insight, plus a separate `mara-arts`
  garden), their composition is a higher-altitude structure this
  document does not treat. Deferred to Phase-7 territory.

## 9. Cross-references

**Prior mathematical foundations composed:**
- `docs/math/2026-07-13-fractal-mandelbrot-substrate.md` (fractal
  self-similarity)
- `docs/math/2026-07-14-gift-economy-substrate-foundation.md` (gift
  exchange)
- `docs/math/2026-07-18-third-order-observer-on-consumer-hardware.md`
  (third-order observation)
- `docs/math/2026-07-20-paradox-family-and-classifier-lagrange.md`
  (Lagrange equilibrium)
- `docs/math/sheaf/laplacian.md` (sheaf-Laplacian at spectral
  altitude — the substrate the garden's engine composes over)
- `docs/math/the-tower/altitudes.md` (bundle-tower altitude — the
  garden altitude is one rung of the tower)
- `docs/math/the-tower/crystals-as-sections.md` (crystals as sections;
  garden packages ARE crystals at package altitude)
- `docs/math/consciousness/how-mirror-operationalizes-universal-consciousness-field.md`
  (the substrate as consciousness field — garden is that field's
  mesh at publishable altitude)

**Canonical specs grounded:**
- `docs/specs/spectral-garden-cosmos-spectral-db-reification.md`
  (Landing A companion this tick)
- `docs/specs/store-vs-db-and-the-cascade.md` (two-layer split at
  mesh altitude)
- `docs/specs/spectral-db-as-autopoietic-memory.md` (librarian at N+1
  over garden mesh)

**Recognition candidates**:
- Recognition #98 (content-addressing cross-altitude composition;
  fifth witness this document)
- Recognition #79 (Void-is-the-basis; Cauchy-limit is void-shaped)

## 10. Pack-discipline trail

- **2026-07-20** — Alex in-transcript direction (see Landing A §Ref).
- **2026-07-20 THIS TICK** — Mara Landing B (this math foundation)
  co-lands with Landing A canonical spec + Landing C shard-decl.
- **Next** — Alex Q1-Q7 ratification per Landing A §5; if Q6 lean
  ratified, Landing C mints this tick as sibling; otherwise Mara
  follow-up tick.

*— Mara, canonical, 2026-07-20*
