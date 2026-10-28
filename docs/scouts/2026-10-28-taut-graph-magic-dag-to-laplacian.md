# Taut scout: graph magic, from the DAG on disk to a directed Laplacian

*2026-10-28. Taut (grep-first, read-only). Asked by Alex via Reed: "How do we get from DAG on disk to directed Laplacian in the runtime? The bits and pieces are already there."*

What the labels mean:
- **LANDED**: code exists and has tests. I did not run any test suites, so LANDED means "code plus tests in the tree" unless I say I executed something.
- **DECLARED**: the thing exists only in a spec, a docblock or a `.mirror` declaration.
- **ABSENT**: grep finds nothing.
- **COMPUTED (Taut)**: a small numpy check I ran myself in the scratchpad. It is not corpus evidence and is flagged wherever it appears.

Repos and versions:
- **prism**: the pinned copy at `/Users/reed/.cache/mirror/prism-7e6b00f/prismqueer`, which is what mirror builds against (`rust/Cargo.toml:75` "Pinned by git rev"). Live prism HEAD is `43f47ba` (2026-10-27).
- **spectral-db**: HEAD `b996be3` (2026-05-09).
- **coincidence**: HEAD `5bac11c` (2026-05-24).
- **fragmentation**: HEAD `22b52ad` (2026-07-13).

---

## §0 Answer in five lines

1. **Disk DAG: LANDED.** `fragmentation-git::write_tree` writes Shard as a blob and Fractal as a tree, so git dedups shared subtrees. spectral-db separately stores an *undirected adjacency graph* in git trees. It does not store the Merkle DAG itself.
2. **Runtime graph: the DAG is lost.** The only Merkle-to-Laplacian path is `coincidence::Laplacian::from_tree`, and its walker allocates one vertex per *occurrence*, so shared subtrees come back as copies. The result is always a tree with b₁ = 0. prism's `Graph` is an undirected simple graph (`BTreeSet<EdgeKey>`), so it cannot even hold the parallel edge that `f(x, x)` reuse produces.
3. **Laplacian: real symmetric only.** Every landed Laplacian (prism `kleinos`/`Spectrum`/`Instrument`, coincidence, spectral-db) is the undirected D − A. prism's `Instrument` (Hodge–Dirac) already returns b₀ + b₁. The magnetic Laplacian and the magnetic Dirac operator are fully **DECLARED** in `practice/insights/spectral-db/edge-slope-spectral-theory.md` (2026-05-12). The rank-1 twisted sheaf with holonomy is **PROVED in docs** (comma doc §1.4) and was **MEASURED in Python** (Seam 10-21). None of these exist in Rust.
4. **Spectrum to coordinate to 48 active bits: DECLARED, never computed.** `SpectralUuid::from_parts(active: u64, …)` is LANDED, but every production caller passes `active = 0`. No quantizer exists anywhere. The landed `SpectralCoordinate` is either SHA-256 or random-projection bytes, never a Laplacian 5-tuple.
5. **Biggest gap:** no OID-deduplicating DAG → graph builder. Without one, both candidates (a) and (b) see a tree, where b₁ = 0 and the magnetic spectrum is gauge-equal to the plain one. COMPUTED (Taut): even on a true DAG, a uniform-phase magnetic Laplacian sees a reuse loop only when its two paths have *different lengths*. No complex eigensolver exists in any repo.

---

## §1 Per stage

### Stage 1: the DAG on disk (git objects)

**LANDED.** fragmentation writes a Merkle tree into git objects.
- `fragmentation/vcs/git/src/git.rs:79-80`: "Write a fragment tree to git objects. Returns the root OID. / Shard -> blob, Fractal -> tree with .data + numbered children."
- Children are entries `format!("{:04}", i)` (`git.rs:95-97`). Identical child content gets the identical git OID, so git dedups it and the object store is a DAG.
- `write_node` is the generic Fragmentable version (`git.rs:125-148`).

**DECLARED.** The VCS spec names the DAG.
- `fragmentation/docs/specs/mirror-native-vcs.md:1027`: "#### Consequence 3 — fragmentation is graph-native DAG".
- It continues: "the same node cloned into two parents collapses to one stored node under the content-address", and "Cycle detection is an error class" for v1 (lines ~1029-1046).

**LANDED, but it is a different graph.** spectral-db stores an *undirected adjacency* graph in git.
- `spectral-db/src/lib.rs:400-408` documents one tree per node holding `.type`, `.content`, `.ts` and an `{oid_B}` entry per neighbour.
- The comment at lines ~446-449 says "each node lists its neighbors" and calls the result "the undirected graph".
- Edges are user, JSONL or NL-token edges. They are not Merkle containment.

**LANDED in prism.** `merkle.rs:11` defines `pub trait MerkleTree`, with `children(&self) -> &[Self]`: children are held by value, which makes it a tree type, not a DAG type. `merkle.rs:45` defines `diff`.

**mirror runtime: the crystal chain is linear.**
- `rust/src/compile.rs:154-155` builds the chain: "each Crystal's `prev` points to the previous crystal's OID".
- `rust/fractal/src/crystal.rs:110-114`: "at this scaffold altitude the OID is computed via a simple XOR-fold". This file has uncommitted edits in the working tree, but the XOR-fold is untouched by them.
- mirror has no git object writer for crystals. `rust/tests/red_spec_claims.rs:284-286` is a RED test: "no chain, no Merkle DAG".

**DELETED.** The old `bootstrap/` was removed in `9de3eca` (2026-08-26). spectral-db's `Cargo.toml` still points at `../mirror/bootstrap` (see §3).

### Stage 2: the DAG as a graph value in the runtime

**LANDED, but it loses reuse.** `coincidence/src/spectral.rs:125` defines `Laplacian::from_tree<F: Fragmentable>`, which calls `walk_tree` and then `dfs`. Lines 794-797:
```
let idx = vertices.len();
vertices.push(content_oid(node));
if let Some(p) = parent { edges.push((p, idx)); }
```
Every occurrence gets a fresh index, even when `content_oid` repeats. The result is the tree expansion: no shared vertices, b₁ = 0. Lines 120-123 add: "Lens targets are external — not included as edges."

**LANDED, the inverse direction.** `fragmentation/vcs/git/src/git.rs:370` defines `read_tree`: "Blob -> Shard, Tree -> Fractal". It rebuilds a by-value tree, so the duplicates come back too.

**LANDED: prism's `Graph` is undirected and simple.**
- `prism/src/spectral/kleinos.rs:131-134` defines `pub struct Graph { vertices: BTreeSet<VertexId>, edges: BTreeSet<EdgeKey> }`.
- `contains_edge` canonicalises with `edge_key(e.0, e.1)` (`kleinos.rs:153-155`).
- `graph_from_edges` (`kleinos.rs:258`) has no weights, no direction and no multiplicity. A parallel edge, such as one parent with two entries pointing at the same child OID, collapses into one edge.

**LANDED: what mirror actually feeds into prism.** Neither graph is a crystal DAG.
- `rust/src/main.rs:270-295` builds `build_shards_sheaf` from sibling cliques of `shards/**/*.mirror` files, grouped per directory.
- `main.rs:317-331` builds `build_session_sheaf` as a path of splinter succession.

**LANDED, the closest thing to an OID-keyed reuse graph.** spectral-db's content subgraph.
- `spectral-db/src/content.rs:1-5`: "creates Inferred edges (tree structure) and Coincidence edges (shared OIDs across nodes)".
- Coincidence edges are created in `lib.rs:2596-2615`. Adjacency is `HashMap<String, HashSet<String>>` keyed by OID (`index.rs:24`), so shared token OIDs *do* close cycles.
- This graph is undirected and is built over NL tokens, not over AST containment.

### Stage 3: the Laplacian

**LANDED: the real symmetric D − A, everywhere.**
- prism `kleinos.rs:306-317`: "Build graph Laplacian L = D - A (row-major flat matrix; symmetric)".
- coincidence `spectral.rs:144-152` defines `from_weighted_edges` with symmetric writes.
- spectral-db `fiedler.rs:84-96` builds a dense Laplacian over replicas, and `index.rs:486-487` builds an ego Laplacian through coincidence.

**LANDED: the Hodge–Dirac operator on C⁰ ⊕ C¹.**
- `prism/src/spectral/instrument.rs:1-16`. Edges are oriented canonically, not by DAG direction: `oriented(e)` returns `(e,1.0)` if `e.0 <= e.1` (`instrument.rs:50-56`).
- `instrument.rs:220-221`: "dim ker D = b₀ + b₁: components plus independent cycles."
- Test `tests/instrument.rs:68-80`, `kernel_counts_components_and_cycles`.
- This is candidate (a)'s b₁ counter, already present.

**LANDED: the cycle-space basis.** `coincidence/src/incidence.rs:117-122` defines `cycle_space()`: "Each basis vector represents an independent cycle in the graph. Empty for trees (no cycles)."

**DECLARED: the magnetic Laplacian, in detail.** All in `practice/insights/spectral-db/edge-slope-spectral-theory.md`:
- §2.3 (l.130-170): `L^q = D - A^q`, "Hermitian … q = 0 recovers the undirected Laplacian", plus a per-edge phase θ_ij.
- §2.5 (l.218-225): "Magnetic Dirac operator … D^theta = [0, (B^theta)*; B^theta, 0] is Hermitian, and (D^theta)^2 = L^theta_0 + L^theta_1".
- §2.9 (l.366-369): "The magnetic Fiedler value … has not been formally studied".
- §7.3 (l.609-614): "Eigendecomposition of an n x n complex Hermitian matrix costs O(n^3) … ~4x".

**DECLARED, with proofs in a doc: a rank-1 twisted sheaf, i.e. a GL(1) connection.**
- comma doc §1.4 (`docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md:116-120`): "(δ_φ s)(e) = (1 + φ_e) s(t(e)) − s(s(e)). This is the coboundary of a rank-1 cellular sheaf".
- Thm 1.6 (l.124): "b₀ − b₁ = |V| − |E|".
- l.128: "Holonomy ≠ 1 around a cycle kills that cycle's H¹ class *and* a global section together."
- l.40: "Real scalars throughout; complexification changes nothing below."

**DECLARED and ABSENT: the O(d) sheaf/connection Laplacian.**
- `shards/epistemologic/math/sheaf_laplacian.mirror:181-182`: "for an O(d)-bundle over a graph the sheaf Laplacian IS the connection Laplacian".
- `shards/mirror/index.mirror:233-234` adds "realized in `sheaf_laplacian::sheaf_laplacian`". That Rust lived in `bootstrap/src/sheaf_laplacian.rs:246` and was deleted in `9de3eca`. It only handled d = 1 and built a symmetric normalised Laplacian ("`Δ_F = I − D^{-1/2} W D^{-1/2}`"); the source/target fields did not set direction.
- prism `kleinos.rs:125-129`: "Stalks default to trivial … Phase 2+ adds non-trivial `Stalker`". There is still no `Stalker` type in `src/`.

**LANDED, as traits: transport and holonomy.**
- `prism/src/bundle.rs:150-170`: `trait Transport { type Holonomy: Metric; fn transport(...) }`.
- Its tests use `[f64;4]` states with `ScalarLoss` holonomy (`bundle.rs:466-467`). It is not a graph operator.

### Stage 4: the spectrum

**LANDED: one real eigensolver family.**
- prism `native/spectral.f90:39,69` calls `dsyev`. `dgesvd` is at `:105,142` (SVD of a real general matrix).
- `ffi.rs:218` `eigenvalues`, `:249` `eigensystem`, `:284/319` `singular_values`/`svd`.
- coincidence has its own `native/spectral.f90` (dsyev) and `SparseLaplacian::lanczos_smallest` (`spectral.rs:1086`).
- spectral-db `fiedler.rs:155-165` uses Jacobi for replicas.

**LANDED: five typed axes, three of them computed.**
- prism `spectrum.rs:96/108/131` compute `split()`, `refract()` and `project()`.
- `Focus` and `Lift` are newtypes only: "Primitive computation … is forward-promised" (`spectrum.rs:162-166`, `:176-178`, header `:28-29`).

**LANDED: heat traces.** coincidence `spectral.rs:395` `heat_kernel(t)` = Σ e^{−tλ}, `heat_kernel_matrix`, `heat_kernel_diagonal`.

**ABSENT: any complex eigensolver.** There is no `zheev`, `zheevd`, `zgeev` or `dgeev` in any `.rs` or `.f90` under `/Users/alexwolf/dev/projects` (see §5). The only hit is the string list in the RED test `mirror/rust/tests/red_spec_claims.rs:69`.

### Stage 5: the spectral coordinate

**DECLARED.** The canonical definition lives in the fragmentation VCS spec.
- `mirror-native-vcs.md:1091-1105`: "coordinate(content) = pack5( λ₂, λ₅ − λ₂, Tr(exp(−0.25 · D²)), Tr(exp(−1.0 · D²)), Tr(exp(−4.0 · D²)) )". It takes D² to be "the graph Laplacian of the content's incidence shape".
- `pack5` is ABSENT from code (grep, §5).

**LANDED, but it is not a Laplacian coordinate.**
- `fragmentation/src/spectral_coordinate.rs:82-104`: `HashAlg::hash` is "SHA-256 of `b"spectral-coord:" || N || bytes`".
- The "rich path" (`fragmentation/spectral/src/hash.rs:27-50`, `coincidence/src/hash.rs`) is `Detector::canonical`. That is N projections "seeded from `SHA-256("coincidence:projection:{i}:{N}")`" and applied to a byte encoding (`coincidence/src/coincidence.rs:55-87`). No graph is involved.

**LANDED in spectral-db: a per-node coordinate.**
- `index.rs:470-501` `compute_spectral_coordinates` takes the ego-Laplacian eigenvalue vector, filtered `>= precision`, and stores it content-addressed (`spectral_store.rs:1-5`: "Same eigenvalues = same OID").
- `profile.rs:1-15` puts a whole-graph "16 × f64" spectrum into a git blob: "The git OID of this blob IS `profile_oid`".

### Stage 6: the 48 active bits of uuid_spectral

**LANDED: the carrier.**
- prism `spectral_uuid.rs:93-102`: `from_parts(active: u64, content_hash: &[u8; 32])` uses "lower 48 bits".
- `spectral_uuid.rs:31-34`: "Quantization of `SpectralCoordinate<5>` into 48 bits lives at fragmentation's boundary; this crate takes a pre-quantized `u64`".

**LANDED with `active = 0`.**
- `fragmentation/vcs/mcp/src/shard.rs:107-111`: "T4 ships with `active = 0` (the spectral coord computation lives upstream in `coincidence`; … once the quantization rules pin per the spec §11 Q1)".
- `session.rs:94`: `ShardId::from_content(0, hash.as_bytes())`.
- The only nonzero `active` values are test literals (`tests/shard.rs:81`, `tools/content.rs:351`).

**ABSENT: any quantizer.**
- `quantiz` appears only in docblocks and specs: prism `spectral_uuid.rs`, fragmentation `shard.rs`, mirror `AGENTS.md:520,532`, `reality-shard-as-crdt.md:84,312`, `docs/math/uuid/spectral-time.md:116`.
- spectral-db's cartographer plan (`docs/superpowers/plans/2026-04-04-cartographer.md:251-258`, "Quantizes to 4 decimal places") was never ported into `src/`.

**DECLARED: the `combine` arithmetic for the active half is still open.** `reality-shard-as-crdt.md:308-313`, §11 OQ1: "the quantized 12-bit-per-dim encoding may need a small rounding discipline".

---

## §2 The directed-Laplacian question

### 2.1 Does the corpus know that D − A on a DAG is degenerate?

**Partly. It knows the adjacency-side version and says the symmetric Laplacian cannot see direction. It never states the triangular D_out − A fact.**

- `prism/docs/math/constrained-inference-floor.md:217` (prism `7e6b00f`, 2026-10-19): "directed adjacency A is nilpotent iff the graph is acyclic | **Do not decide this with floating eigenvalues** (Prop. 3.2). The symmetric Laplacian cannot see it at all."
- Same file, `:228-232`: "Proposition 3.2 … A digraph is acyclic iff its adjacency matrix A is nilpotent … Nilpotent matrices are **defective** … The spectral criterion is exact in exact arithmetic and **useless in floating point**."
- Same file, `:234`: "LAPACK is the right instrument when the answer is a well-conditioned quantity with a margin. It is the wrong instrument when the answer is an exact structural fact with no margin."
- `edge-slope-spectral-theory.md:80-83` says only "For directed graphs … L = D - A is not symmetric. Its eigenvalues are generally complex". It does not mention the acyclic case, where they are real and equal to the out-degrees.
- Same insight, `:85-98` (Chung 2005) and `:349-358` (Wu 2005). Chung's Laplacian is defined "for strongly connected directed graphs" (l.85-86), and Wu's "a(G) = 0 iff G is not strongly connected" (l.355). A DAG with n ≥ 2 is never strongly connected. Taut's reading of the text: Chung's operator is undefined on a DAG and the directed Fiedler value is 0.
- `docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md:347`: "Directed gaps hide inside one undirected component." The same doc at `:340` reads: "Orientation of each relation is a gauge two communicators may differ on … The gauge group acts on edges, not on the log."

**COMPUTED (Taut, scratch `dagcheck.py`, not corpus).** On a diamond, a triangle and a tree DAG, `eig(D_out − A)` returned exactly the sorted out-degrees: `[0,1,1,2]` vs out-degrees `[0,1,1,2]`. This confirms Reed's finding.

### 2.2 Is a magnetic, connection or sheaf Laplacian implemented or specced?

| Operator | Status | Where |
|---|---|---|
| Magnetic L^q, per-edge θ, magnetic Dirac D^θ, magnetic Hodge (D^θ)² | DECLARED (research synthesis; no code) | `practice/insights/spectral-db/edge-slope-spectral-theory.md` §2.3, §2.5, §3.2, §5.1 (2026-05-12) |
| Rank-1 twisted sheaf δ_φ (GL(1), holonomy) | DECLARED and PROVED in doc | comma doc §1.4–1.6 (l.112-142); twisted control §3.3 (l.266) |
| Rank-2 rotation twist (O(2)/SO(2)) on cycles | MEASURED in Python, not Rust | `practice/insights/spectral/2026-10-21-seam-crowbar-the-baby.md:117-135` ("On a graph *every* cycle can carry holonomy", "λ_min = 2 − 2cos(θ/n)"); `.experiments.py` uses real `np.linalg.eigvalsh` |
| Magnetic Laplacian, as prior art for the above | Cited | same crowbar doc `:146`: "**Magnetic Laplacian:** U(1) holonomy on graphs; Shubin 1994 …; MagNet 2021 for directed graphs"; `:151` "Verdict (d): RENAMED … (connection/magnetic Laplacian …)" |
| O(d) connection Laplacian = sheaf Laplacian | DECLARED; the Rust was deleted | `sheaf_laplacian.mirror:181-182`; deleted `bootstrap/src/sheaf_laplacian.rs` (`9de3eca`), which was d = 1, symmetric and normalised |
| Sheaf stalks / `Stalker` in prism | DECLARED (Phase 2+) | `kleinos.rs:19-20, 125-129` |
| Additive ℝ 1-cochain with its harmonic residual ("comma" = holonomy of the circle of fifths) | PROVED in doc (Python) | Chord doc `2026-10-10-mara-the-chord-math-foundation.md:730-736`, Thm 6.1(iii): "Its periods are the commas" |
| Non-backtracking operator (non-symmetric, on directed edges) | DECLARED | `practice/insights/coincidence/non-backtracking-operators.md:9, 54` ("Needs complex eigenvalue solver") |

**Taut note on the table (standard math, not stated in the corpus).** Seam's "rank-2 rotation" twist is SO(2) ≅ U(1) written over the reals. A magnetic Laplacian on n vertices therefore has the same spectrum, each eigenvalue doubled, as a real 2n × 2n O(2)-connection Laplacian, and that is something `dsyev` can already solve.

**COMPUTED (Taut, scratch, not corpus): when does the magnetic Laplacian differ from the undirected one on a DAG?** With q = 1/4:
- **Tree DAG:** spectrum identical to the undirected one (`[0, .586, 2, 3.414]` both).
- **Diamond r→a→c, r→b→c (equal-length reuse paths):** spectrum identical (`[0,2,2,4]` both).
- **Triangle r→a→c, r→c (unequal paths):** spectrum differs (`[0.268, 2, 3.732]` vs `[0,3,3]`).

So with a uniform phase, the flux around a reuse loop is proportional to the length difference of its two paths. Equal-depth reuse is invisible to (b). This is a check, not a corpus claim.

### 2.3 Any complex eigensolver anywhere?

**ABSENT.**
- No `zheev`, `zheevd`, `zgeev` or `dgeev` call in any `.rs` or `.f90` under `/Users/alexwolf/dev/projects` (excluding `target/` and `_archive/`).
- prism `native/spectral.f90` exposes `dsyev`, `dgesvd` and `phase_lock` only (`:22-211`).
- `Instrument::ring` computes e^{itD} as real cos/sin parts of a real D (`instrument.rs:148` `struct Rotated { real, imag }`, `:244` `ring`). It never forms a complex matrix.
- The only complex arithmetic in Rust is `rustfft::num_complex` in `cosmos/src/rgg.rs:396`, which is FFT and not eigen.
- Two docs record the need for one: `edge-slope…md:609-614` and `non-backtracking-operators.md:54`.

---

## §3 Pieces in spectral-db that mirror and prism don't have

**Build state.** spectral-db does **not** resolve today.
- `cargo metadata --offline` exits 101 with "failed to get `mirror` … Unable to update /Users/alexwolf/dev/projects/mirror/bootstrap".
- The staged `Cargo.toml` diff moved `mirror = { path = "../mirror" }` to `"../mirror/bootstrap"`, which `9de3eca` deleted.
- `prism = { package = "prism-core", path = "../prism/core" }` points at a path that no longer exists either. prism now lives at `prism/prismqueer`.
- The last commit is 2026-05-09. There are 484 `#[test]` attributes across `src/` and `tests/`. They do not compile against the current tree, so I did not run them.

**LANDED in spectral-db only, by its own tests and history.** Each piece is buildable only at its 2026-05 dependency snapshot.
- **Incremental reindex on append.** `src/incremental.rs:241` `incremental_index` diffs git commits (`diff_commits`, `:114`) and records the last indexed commit under a ref (`indexed_ref_name`, `:86`). Tests at `:486` ("incremental_index_processes_only_diff") and `:527` ("five_sequential_indexes_with_no_changes_produce_zero_work").
- **Dirty/settled spectral tree.** `src/spectral_tree.rs:82` `mark_dirty`; `:136` `advance_settlement` ("if not dirty and spectrum unchanged, increment stable ticks").
- **Ego-subgraph spectra as per-node coordinates.** `index.rs:403` `ego_subgraph(center, hops)`; `:470-501` `compute_spectral_coordinates` bounded by `SpectralBudget` (`budget.can_decompose(ego.n)`).
- **A content-addressed eigenvalue store.** `spectral_store.rs:45-56`, LE f64 bytes → git blob OID.
- **Spectrum committed to git.** `profile.rs` stores a 16-eigenvalue blob plus a graph-hash trailer, and `lib.rs:115-128` `CrystalCommitMeta { fiedler, nodes, edges, profile_oid, sessions, holonomy, … }` puts it in the commit message.
- **The `holonomy` field is a placeholder.** It is hard-coded `holonomy: 0.0` at `lib.rs:1621`, so it is DECLARED in the format only.
- **OID-keyed reuse edges.** `lib.rs:2596-2615` creates Coincidence edges for shared token OIDs. This is the only landed graph in which shared content closes a cycle.
- **Replica-network Fiedler monitor.** `fiedler.rs:29-127`, `NetworkMonitor::check → PartitionRisk`.
- **An eigenvalue L2 metric.** `index.rs:642` `eigenvalue_distance`, "zero-padded to equal length".

**Recorded negative result.** `docs/superpowers/specs/2026-04-04-cartographer-design.md:21`: "Sparse Lanczos: 130x Fiedler error — Krylov too small for degenerate rings".

**Header drift.** The `index.rs:1-5` header says "Uses SHA-256 as the spectral hash for content indexing", while `:29` says "ego Laplacian eigenvalues (the spectral hash)". Both maps exist side by side (`hashes` vs `spectral_eigenvalues`).

---

## §4 Contradictions and drift between the three corpora

1. **Three definitions of `SpectralCoordinate<5>`.**
   - (i) `mirror-native-vcs.md:1095-1101` defines it as λ₂, λ₅−λ₂ and three heat traces.
   - (ii) `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-measurement.md:162-166` maps those slots onto void dualities spectral/entropy/cheeger/ricci/mixing.
   - (iii) prism `spectrum.rs` uses five different readings: Split = λ₂, Refract = von Neumann, Project = Cheeger bound, Focus = ORC, Lift = KW.
   - The landed code (`coincidence.rs:55-87`) is SHA-seeded random projections of bytes, i.e. none of the three.
   - The docblock `fragmentation/src/spectral_coordinate.rs:4-6` says "Fiedler value, eigengap, three heat-trace samples" for a type whose `hash` is SHA-256.
2. **Which graph the active bits read.**
   - `shards/uuid/spectral.mirror:36-39` says the "local Laplacian neighbourhood structure within a shard's splinter OID graph".
   - `docs/math/uuid/spectral-time.md:116-120` says "five-eigenvector projection of the peer's local Laplacian neighbourhood".
   - `docs/specs/spectral-hash-design.md:211-215` requires the opposite: "must be derivable from content bytes *without* requiring graph context (otherwise it can't serve as an *address*) … The current eigenvalue-vector implementation FAILS this".
   - Reed's brief adds a fourth reading: "a spectral coordinate of the crystal's OWN DAG".
3. **Whether the coordinate belongs in the identifier at all.**
   - `docs/specs/store-vs-db-and-the-cascade.md:48-61` (VoidPointer, 2026-05-30): "Conflating the two (e.g., putting `VoidPointer` into the storage OID …) couples them in ways that break the posture."
   - uuid_spectral (2026-06-06, `AGENTS.md:518-535`) puts the spectral coordinate into the 128-bit identifier next to the content hash.
4. **Bit budget.**
   - `reality-shard-as-crdt.md:312` says "quantized 12-bit-per-dim encoding". 12 × 5 = 60, not 48; 48 / 5 = 9.6 bits per dim.
   - rung-8-9 `:91` gives the canonical encoding as "40 bytes = 5 × f64 LE", which is 320 bits.
   - `spectral-time.md:131` claims "quantization loss is bounded by `O(1/2^48)` per component", which assumes 48 bits *per component*.
5. **What D² means in the same spec.** `mirror-native-vcs.md:1024` has "`D² = L₀ + L₁` is the Hodge Laplacian whose spectrum the coordinate samples", but `:1104` and `:1393` have "`D² = L₀`". A heat trace over L₀ + L₁ carries b₁; one over L₀ alone does not.
6. **Stale path.** `AGENTS.md:522` cites "`prism/core/src/spectral_uuid.rs`". The file is at `prism/prismqueer/src/spectral_uuid.rs`. spectral-db's `Cargo.toml` carries the same `prism/core` path.
7. **Realisation claims after the deletion.** `shards/mirror/index.mirror:234` says the connection Laplacian is "realized in `sheaf_laplacian::sheaf_laplacian`". That function was deleted with `bootstrap/` in `9de3eca`, and when it existed it was undirected and d = 1.
8. **Uniqueness of a spectrum.**
   - `spectral-db/src/profile.rs:13-14`: the profile OID "uniquely identifies the graph state by its spectrum".
   - `practice/insights/fragmentation/math/spectral-geometry.md:86-98` ("Isospectral but Not Isometric") documents that isospectral pairs exist.
9. **Edge-model attribution.** `edge-slope-spectral-theory.md:11-13` describes "spectral-db edges" via `incidence.rs`, `evolve.rs` and `graph.rs`. Those files are in `coincidence/src/`, not `spectral-db/src/`.
10. **Orientation semantics.** prism's `Instrument`, coincidence's `IncidenceMatrix` and `dirac-operator-on-graphs.md:293` ("Choose canonical orientation (lower OID -> higher OID)") all orient by *id order*. The difference doc (`:340`) treats orientation as a gauge. A Merkle DAG's parent→child direction is not represented by any of them.

---

## §5 Commands run

All read-only. Scratch files went to the session scratchpad and `/tmp/sdb_meta.err`.

```
head identity.mirror (Taut); ls spectral-db; git -C spectral-db log --oneline | wc -l
rg -ni -e "magnetic|hermitian|zheev|...|dsyev|lanczos|eigensolv" spectral-db (excl. target)
ls -d prism/core prism/imperfect fragmentation ... mirror/bootstrap   # existence of spectral-db path deps
cargo metadata --format-version 1 --offline --manifest-path spectral-db/Cargo.toml   # exit 101
cargo metadata --no-deps --offline  (coincidence, fragmentation)                    # exit 0
git -C spectral-db diff --cached -- Cargo.toml
sed/rg over spectral-db/src/{index,lib,store,content,profile,fiedler,incremental,spectral_tree}.rs
sed/rg over coincidence/src/{spectral,incidence,coincidence,hash}.rs, native/spectral.f90
sed/rg over fragmentation/{src/spectral_coordinate.rs, spectral/src/hash.rs, vcs/git/src/git.rs, vcs/mcp/src/shard.rs}
Read prism-7e6b00f/prismqueer/src/{spectral_uuid,spectral_oid,merkle,oid}.rs, spectral/{kleinos,instrument,spectrum}.rs, ffi.rs, bundle.rs, connection.rs
rg -ni magnetic|hermitian|zheev|holonomy  /Users/alexwolf/dev/projects/prism (live, 43f47ba)
rg -i quantiz across prism, fragmentation, mirror, spectral-db, coincidence
rg -i "magnetic laplac|Shubin|Fanuel|MagNet|Chung|directed laplac|connection laplac|Singer|Bandeira" mirror/docs mirror/shards AGENTS.md insights prism/docs spectral-db/docs
rg -i "triangular|out-degree|D_out|topological order" (corpus)
rg -l "zheev|zgeev|dgeev|Complex64|num_complex::" -g '*.rs' -g '*.f90' /Users/alexwolf/dev/projects  # 2 hits, neither an eigensolver
sed mirror/docs/math/{comma-between-observers, chord, difference-that-makes-a-difference}, prism/docs/math/constrained-inference-floor.md
sed fragmentation/docs/specs/mirror-native-vcs.md §4.6; mirror/docs/specs/{reality-shard-as-crdt, spectral-hash-design, store-vs-db-and-the-cascade, rung-8-9-…}.md; docs/math/uuid/spectral-time.md
sed shards/{uuid/spectral, sheaf, mirror/index, epistemologic/math/sheaf_laplacian}.mirror
git -C mirror log --all -S'fn sheaf_laplacian'; git show 9de3eca --stat; git show 9de3eca^:bootstrap/src/sheaf_laplacian.rs
sed mirror/rust/src/{main,compile}.rs, rust/fractal/src/crystal.rs; git diff --stat (crystal.rs, compile.rs)
rg/sed practice/insights/{spectral-db/edge-slope-spectral-theory, spectral-db/dirac-operator-on-graphs, spectral/2026-10-21-seam-crowbar-the-baby(.md,.experiments.py), coincidence/non-backtracking-operators}
python3 scratchpad/dagcheck.py   # COMPUTED (Taut): eig(D_out−A) = out-degrees; magnetic vs undirected on tree / equal diamond / unequal triangle
```
