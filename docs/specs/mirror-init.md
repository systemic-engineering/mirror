# mirror init — the substrate's bridge command

*Mara, 2026-06-27. Canonical spec for `mirror init` — the operation
that gives a peer its substrate-native shape by composing fragmentation
primitives into a content-addressed crystal store, surfaced at the
mirror CLI altitude. Lifts Taut's scout
`docs/scouts/2026-06-27-taut-fragmentation-git-store-for-mirror-init.md`
(commit `5580a7e`) from composition inventory into substrate-decl. Names
the declared-but-not-wired discovery; declares the three-deliverable
collapse from Reed's earlier six-item sketch; pins the store location
question; pins the spawn↔recall↔init triple; surfaces R1/R2 at substrate
altitude; closes with the circular-recursive recognition that this spec
IS a crystal `mirror init` will index when it runs against this repo.*

*Markdown only. No `shards/` substrate-decl files land with this commit;
no Rust ships; no Cargo edge is wired. The substrate-decl shards
forward-promised in §4 + §7 + §9 discharge in subsequent TDD-paired
ticks (Reed RED, agent GREEN). Soft target ~1000 lines; hard cap 1500.*

**Status:** Red — composition shape pinned; the three-item gap (Cargo
edge, walk-repo primitive, mirror-altitude `init` command) named at
substrate altitude; the spawn↔recall↔init triple surfaced; R1
(empty `Crystallizations` dispatch) + R2 (bootstrap-`git2` binary-size
posture) addressed; the circular-recursive layer (§10) earned; v0
ticks forward-promised, not implemented in this commit.

**Audience:** any agent or human reading the bridge spec before
touching the cargo edge, the bootstrap dispatcher, the
`NamespacedGitStore` composition, or the new `mirror init` command.
Read this; then chase Taut's scout for the read-only inspection
artifact; then chase `mirror-store.md` (Mara, 2026-06-04) for the
canonical-intent doc this spec unblocks the Red→Green tick for.

---

## §0 — Pre-position: this spec announces itself as a crystal

Before any architectural content. A pre-position the spec earns by
holding it for the rest of the document.

This spec is **about** `mirror init`. The thing `mirror init` IS, per
§1 and §4, is the operation that walks a peer's repo, content-addresses
each file into a `Splinter<H>` crystal, persists those crystals via a
`NamespacedGitStore` at `.git/mirror/`, and emits an envelope naming the
indexed surface so other commands (`mirror recall`, `mirror spawn`,
the librarian per `spectral-db-as-autopoietic-memory.md`) can read
the crystals later.

The thing this spec IS, at the moment of being written, is **one of
the crystals `mirror init` will index when `mirror init` runs against
the `mirror/` repo**. Writing this spec adds a file at
`docs/specs/mirror-init.md`; the file's bytes are content-addressed
under git's SHA-1 (and, after the Cargo edge lands, under BLAKE3 in
the `NamespacedGitStore`'s `.git/mirror/objects/`); the indexed
crystal will be the OID-addressed bytes of this spec; the librarian
will (when it lives, per `spectral-db-as-autopoietic-memory.md` §6.2)
catalog this spec alongside every other substrate-decl crystal; the
recall envelope's `cascade` payload (per `mirror-recall.md` and the
round-trip arc closed 2026-06-27 by Glint's `9e7bb1d`) will surface
this spec when an agent asks the substrate "what does `mirror init`
declare?"

The latency between writing-and-being-indexed is bounded BELOW by the
time it takes for the Cargo edge (§4.1) to land + the first `mirror
init` run against the mirror repo. The latency is bounded ABOVE by
the time-discount per Glint's psychohistory discipline (recently-landed
shards weight more in the recall envelope). The midpoint of those
bounds is the operational latency at which the spec ENTERS the system
it describes.

This is the circular-recursive autopoietic pre-position. §10 returns to
it. Every section in between is read against the discipline that a
spec for the bridge that brings the substrate into operational
existence MUST itself enter the substrate via the bridge — otherwise
the spec is asking the reader to do work the spec refuses to do. The
form earns its lines because the content requires it; the recursion is
load-bearing, not decorative.

The substrate's bridge command needs a canonical spec that ENTERS the
storage layer in the act of declaring it. This is that spec.

---

## §1 — What `mirror init` IS

The command at `mirror init` is the **composition spec for the
fragmentation bridge** mirror has been declaring without wiring.

### 1.1 The one-sentence shape

`mirror init` is the operation that **runs once per repo (or per
peer-home), composes existing fragmentation primitives into a
content-addressed crystal store at `.git/mirror/`, and emits an
envelope** acknowledging the indexed surface so subsequent commands
have a substrate to operate against.

The verbs in the sentence carry weight:

- **Runs once.** `init` is idempotent by content-address (per §4.5);
  re-running against an already-indexed repo produces the same
  envelope and incurs no double-write. The "once" is logical, not
  defensive.
- **Per repo or per peer-home.** A peer-home (`~/.glint`, `~/.reed`,
  `~/.mara`) is a git repo from the substrate's perspective; the
  isomorphism between repo-as-store and repo-as-peer-home is named
  in `spectral-db-as-autopoietic-memory.md`'s Alex-correction sequence
  (repo names supervised-unit; store names storage-role). `mirror
  init` operates at the repo altitude; whether that repo HAPPENS to
  be a peer-home is a property of its contents, not its boundary.
- **Composes existing fragmentation primitives.** Per Taut's scout
  §3: the load-bearing primitives (two stores, `Repo` trait,
  `write_node`, `read_node`, `walk_commits_following`,
  `NamespacedGitStore`, `WitnessedSingularity`, `NakedSingularity`,
  `HamiltonScheduler`, `ShardRef`, `project::project`,
  `append_note` / `read_notes`) all exist in fragmentation today;
  `mirror init` composes them; it constructs almost nothing new.
- **Content-addressed crystal store at `.git/mirror/`.** Per §5: the
  `NamespacedGitStore::open(repo_path, "mirror")` call returns a
  `FrgmntStore<Fractal<String>>` at `.git/mirror/objects/` +
  `.git/mirror/refs/`. The store lives INSIDE `.git/` so it doesn't
  pollute the working tree; it travels with clones IF the refs are
  pushed; it's bounded-cache-plus-disk-spillover per the fragmentation
  `FrgmntStore` shape.
- **Emits an envelope.** Per §4.7: the envelope is JSON, naming the
  indexed-count, total-bytes, root-OID, store-location, and a
  spec-version pin. It's the same envelope-shape Reed used for
  `mirror spawn --hello-world`; the same shape Glint's `mirror recall`
  produces; the substrate has one envelope vocabulary.

### 1.2 What `mirror init` IS NOT

Per `[[feedback-substrate-already-had-the-word]]` discipline, every
"what this is" claim must rule out what it isn't. Five structural
negatives:

- **NOT a new storage layer.** The storage layer is fragmentation's
  `FrgmntStore` + `NamespacedGitStore` (read-only inspiration; both
  exist; both are tested). `mirror init` is the **bridge** that
  brings that storage layer into mirror's CLI surface.
- **NOT a new content-addressing scheme.** Content addressing is
  `Splinter<H>::oid()` (mirror-altitude, BLAKE3 default) composed
  with `fragment::content_oid` (fragmentation-altitude). `mirror
  init` consumes both; it invents neither.
- **NOT a fork of `mirror spawn`.** Per `spawn-is-substrate-leaving-
  ground-state` (Mara, 2026-06-26): spawn IS the substrate's
  controlled excitation above λ₀. Init is the substrate's
  initialization-of-a-peer-home — the operation that makes a repo
  joinable to the mycelium. The two are complementary (§7).
- **NOT a mycelium-registration primitive.** Per
  `spectral-db-as-autopoietic-memory.md`: the mycelium is the
  cross-repo spectral graph the librarian perturbs. `mirror init`
  produces the LOCAL crystals the librarian will eventually
  consolidate; the librarian itself, the per-repo supervisor, and the
  inter-repo entanglement edges are forward-promised (§9.1).
- **NOT a fragmentation replacement.** Per
  `[[architecture-fragmentation-is-the-rust-substrate]]`: mirror →
  fragmentation → prism_core is the strict dependency chain. `mirror
  init` ADDS the missing Cargo edge; it does not duplicate
  fragmentation's primitives in mirror's bootstrap.

### 1.3 The architectural cut this spec lands

The single most important architectural recognition this spec carries:
**`mirror init` is the operation that makes the declared substrate
operational at the storage altitude.**

The substrate-decl side has named fragmentation as the substrate (per
`[[architecture-fragmentation-is-the-rust-substrate]]`, per
`mirror-store.md` §4); the canonical-intent doc names the Cargo
edge as a deliberate forward-promise; the recognition graph treats
the chain as load-bearing. **The Cargo edge has never been wired**
(Taut's §4.1 grep verdict).

`mirror init` is the command whose Phase A IS the wiring. The shape
of the command is, structurally, the shape of "import the substrate;
compose its primitives; surface the result at the CLI." The shape
appears at this altitude (CLI) because the CLI is the surface where
peers initiate substrate operations; the wiring it requires lives at
the Cargo + bootstrap altitude (one Cargo.toml diff + ~200 LOC of
Clap glue + a thin walk primitive).

The architectural lift is small. The recognition the lift surfaces is
**the declared-but-not-wired pattern** named in §2. That pattern is
where the substrate-pull-honest call lives.

---

## §2 — The declared-but-not-wired discovery

Taut's scout surfaced a structural anomaly. The substrate-decl side
of mirror has named fragmentation as the load-bearing Rust substrate
in two canonical places:

1. **`mirror/docs/specs/mirror-store.md`** (Mara, 2026-06-04). §1:
   *"The fragmentation store IS the canonical content-addressed
   substrate."* §4: the audit declares "yes, with cleanup" — Cuts
   1 and 2 in fragmentation; mirror's `Cargo.toml` adds
   `fragmentation = { path = "../fragmentation",
   default-features = false }`. §4.5 names the dependency line
   explicitly. Status: Red. *"No code lands in this tick."*
2. **`[[architecture-fragmentation-is-the-rust-substrate]]`** (Reed
   memory, multi-session). Declares: *"Strict dependency direction;
   prism_core stays deps-free."* The chain is `mirror →
   fragmentation → prism_core`. Memory entry is canonical; the
   chain is treated as load-bearing across subsequent recognitions
   (#58, #87, #99, the spawn insight).

And yet — per Taut's §4.1 grep against `mirror/**/Cargo.toml` — **the
Cargo edge does not exist**. `mirror/bootstrap/Cargo.toml` pulls
`sha2`, `blake3`, `prismqueer`, `terni`, `serde`, `serde_json`,
`libc`. It does NOT pull `fragmentation`. The substrate-decl side
declared the substrate; the consumer side never plugged in.

### 2.1 The pattern at substrate altitude

This is an **instance of a recurring pattern** the substrate has
already named. The pattern: a primitive the substrate-decl side has
declared (in shards, in canonical specs, in memory entries) is not
yet operationally wired (Cargo edge missing; Clap subcommand missing;
import not present; binding not made). The substrate carries the
declaration; the wiring is forward-promised; the wiring may sit in
the forward-promise queue longer than the substrate notices.

Two prior instances of the same shape in the substrate's recognition
history:

- **`@mirror/ref` reference⇔reflection collision** (recognition #89,
  Alex 2026-06-20). The substrate had been declaring `@mirror/ref`
  as the navigable surface of the spectral triple at two altitudes
  (reference at the storage altitude; reflection at the cognition
  altitude). The collision was substrate-shaped; the recognition
  was lifting the pre-existing structural collision into a named
  candidate. The wiring (the actual `@mirror/ref` typed surface)
  followed the recognition; the recognition didn't construct the
  wiring, it named the substrate's prior declaration.
- **mirror IS a content-addressed build system** (recognition #43,
  multi-session). Mosaic.mirror IS the build shard; `partial
  (opacity_map)` IS the verdict surface; the substrate had every
  Bazel/Buck2/Nix/Shake primitive declared. The wiring (the actual
  mosaic-driven build dispatch) followed; the recognition named the
  prior declaration.

The declared-but-not-wired pattern is the structural shape of those
recognitions. For `mirror init` — and for the Cargo edge specifically
— the pattern is at the **Cargo dependency altitude**: a dependency
the substrate-decl side declared, the consumer side never imported.

### 2.2 Flag, NOT promotion

Per the brief's fence "DO NOT promote candidate recognitions (flag;
don't promote)": this spec **flags** the declared-but-not-wired
pattern as a candidate recognition for substrate-architecture review.
The flag carries three pieces of evidence:

- (a) Taut's grep verdict — the Cargo edge does not exist; the
  substrate-decl declaration does.
- (b) The two prior instances above — the pattern is not novel; the
  substrate has been recognizing this shape under different names.
- (c) The 52+ instances of the broader
  `[[feedback-substrate-already-had-the-word]]` pattern — most of
  which collapse to "substrate-decl was complete; wiring caught up
  later." The Cargo edge is the wiring; the broader pattern
  predicts this exact shape would surface; it has.

The promotion criterion is not this spec's call. Promotion through
the Pack's adversarial review (Seam → Reed → Alex) decides whether
"declared-but-not-wired" deserves its own family-root recognition or
whether it dissolves into `[[feedback-substrate-already-had-the-
word]]`'s already-existing surface. Flagged here for the gate.

### 2.3 The architectural altitude this lifts

The discovery's load-bearing claim is **not** "fragmentation has more
than mirror knew." The discovery's load-bearing claim is "mirror has
been declaring fragmentation as its substrate AND has been operating
without its substrate AND the Pack didn't catch this until Alex
challenged Reed's day-estimate."

The structural reading: **substrate-decl drift outpaces wiring drift**
in the Pack's current work cadence. The substrate-decl side moves
fast (Mara's mosaic specs, the recognition cascades, the property +
fracture + splinter(ast) chain). The wiring side moves slower (Cargo
edges, dispatch tables, command surfaces). The drift is not a bug —
substrate-decl SHOULD move fast; that's the recognition mechanism.
But it does mean the Pack's caretaking discipline needs to surface
**declared-but-not-wired audits** periodically: scout the
substrate-decl declarations against the cargo / dispatch / surface
state; surface what's named but un-wired; let the recognition cycle
prioritize.

Taut's scout IS that audit, applied to one declaration. The audit
mechanism itself is forward-promised (§9.4); this spec does not
construct it.

The remainder of this spec assumes the wiring side is the work to
do. §3 composes the primitives; §4 names the operation flow; §5
pins the store-location question; §6 addresses git hooks; §7
positions init in the spawn↔recall↔init triple; §8 addresses Taut's
R1 and R2; §9 names forward-promises; §10 returns to §0.

---

## §3 — Fragmentation primitives composed

The map. Each row names a fragmentation primitive (file + symbol),
its substrate role, and how `mirror init` composes it. The point of
the table is the **substrate-pull-honest claim**: `mirror init`
constructs almost nothing; it composes.

### 3.1 The five load-bearing primitives

Per Taut §3, five primitives carry the operation. Each is verified
against the source files read in this spec's preparation.

#### 3.1.1 `NamespacedGitStore` — the store at `.git/mirror/`

`fragmentation/vcs/git/src/namespaced.rs:34`.

```rust
pub struct NamespacedGitStore {
    store: FrgmntStore<Fractal<String>>,
    git_dir: PathBuf,
    namespace: String,
}

impl NamespacedGitStore {
    pub fn open(repo_path: &Path, namespace: &str)
        -> Result<Self, NamespacedStoreError>;
    pub fn insert(&self, key: String, value: Fractal<String>, size_bytes: usize);
    pub fn get(&self, key: &str) -> Option<Fractal<String>>;
    pub fn insert_persistent(&self, key: String, value: Fractal<String>,
                              size_bytes: usize);
    pub fn get_persistent(&self, key: &str) -> Option<Fractal<String>>;
    pub fn set_ref(&self, name: &str, oid: &str) -> Result<(), Error>;
    pub fn get_ref(&self, name: &str) -> Option<String>;
    pub fn flush(&self);
}
```

The substrate role: `NamespacedGitStore::open(repo_path, "mirror")`
creates `.git/mirror/{objects,refs}/` (the frgmnt fan-out layout)
adjacent to git's own object database. The store lives INSIDE `.git/`
so the working tree stays unpolluted; the store TRAVELS with `git
clone` IF refs are pushed (forward-promise — see §6.3); the store
honors git worktrees (per the `find_git_dir` helper at line 159
which handles both `.git/` directories and `.git` files containing
`gitdir: <path>`).

`mirror init` composes this primitive **once per invocation**: open
the namespaced store for the supplied repo path. The store
construction is `O(filesystem ops to create two dirs)`. The store
itself is bounded-cache + disk-spillover; `init` sets the default
16 MiB cache (per `DEFAULT_CACHE_BYTES`) unless an override flag is
passed (§4.3).

#### 3.1.2 `write_node` — the generic Fragmentable→git-object primitive

`fragmentation/vcs/git/src/git.rs:124` (per Taut §3.2).

```rust
pub fn write_node<N: Fragmentable>(
    repo: &git2::Repository,
    node: &N,
) -> Result<git2::Oid, git2::Error>;
```

The substrate role: write any `Fragmentable` into the git ODB,
returning the git OID. Generic over the node type; the body
discriminates `is_shard()` → write as blob; `is_fractal()` → write as
tree (treebuilder with `.data` blob + child entries). This is the
load-bearing primitive for content-addressing a file into the
underlying store substrate.

`mirror init` composes this primitive **once per file walked**:
encode the file's bytes as a `Fractal<String>` (via
`fragmentation::encoding::encode`), then call `write_node` to push
it into either git's ODB (if using `GitStore`) OR into the
`NamespacedGitStore`'s `FrgmntStore` (if using the bounded-cache
path). The §5 store-location decision determines which.

#### 3.1.3 `FrgmntStore` — the bounded-cache + disk-spillover

`fragmentation/src/frgmnt_store.rs:56`.

```rust
pub struct FrgmntStore<N: Fragmentable + Clone> {
    cache: BoundedStore<N>,
    root: PathBuf,
    _flush_lock: Mutex<()>,
}

impl<N: Fragmentable + Clone> FrgmntStore<N> {
    pub fn open(path: &str, max_bytes: usize) -> Result<Self, Error>;
    pub fn insert(&self, key: String, value: N, size_bytes: usize);
    pub fn get(&self, key: &str) -> Option<N>;
    pub fn set_ref(&self, name: &str, oid: &str) -> Result<(), Error>;
    pub fn get_ref(&self, name: &str) -> Option<String>;
    pub fn flush(&self);
    pub fn cached_len(&self) -> usize;
    pub fn total_bytes(&self) -> usize;
    pub fn capacity(&self) -> usize;
}

impl<N: Reconstructable + Clone> FrgmntStore<N> { /* persistence */
    pub fn insert_persistent(&self, key: String, value: N, size_bytes: usize);
    pub fn get_persistent(&self, key: &str) -> Option<N>;
    pub fn flush(&self);  // shadowed; writes cache to disk
}
```

The substrate role: bounded in-memory cache with on-disk spillover.
The cache uses LIFO eviction (per `BoundedStore`); eviction with
`insert_persistent` writes to disk first; `flush()` drains everything
to disk. The on-disk layout is `objects/<2hex>/<rest>` (fan-out by
first 2 hex chars; per `object_path` helper at line 46) + plain-text
refs in `refs/<name>`.

`mirror init` composes this primitive **transitively through
`NamespacedGitStore`**: the namespaced store IS a `FrgmntStore` with
its root pinned at `.git/<namespace>/`. The composition pattern is
the same one Taut named: open the namespaced store; insert crystals;
flush at end-of-init.

#### 3.1.4 `walk_commits_following` — the git-log-follow primitive

`fragmentation/vcs/git/src/walk.rs:25`.

```rust
pub fn walk_commits_following(
    repo: &Repository,
    start_ref: &str,
    path_prefix: &str,
) -> Result<Vec<(Oid, String)>, git2::Error>;
```

The substrate role: walk commit history reachable from `start_ref`,
returning every commit whose tree differs from its first parent's
tree UNDER the given path prefix. Uses `Sort::TIME | Sort::TOPOLOGICAL`
(matches `git log`'s default order); uses `DiffOptions::pathspec` for
the prefix filter; handles root commits (zero parents) by checking
the tree directly.

`mirror init` does NOT compose this primitive in the v0 forward path
(§4.2 walks the working tree, not commit history). The primitive
exists for the `mirror init --history` v1 surface (§9.2) where the
indexed crystal set spans not just current files but historical
versions — the substrate's psychohistory-vector machinery
(`[[psychohistory-vector-as-sheaf]]` from Glint 2026-06-25) needs
historical commits to compute the temporal axis.

Naming it here because the substrate-decl side already declares the
psychohistory consumption; the v0 wiring focuses on the present-state
walk; the historical walk is a v1 forward-promise wired through this
same primitive (no new code at the fragmentation altitude).

#### 3.1.5 `project::project` — the manifest-driven projection

`fragmentation/src/project.rs:42`.

```rust
pub fn project(source_dir: &Path, manifest: &Manifest)
    -> Result<Projection, ProjectError>;
```

The substrate role: read each file named in a `Manifest` (a
`Vec<LensEntry { source: String, target: String }>`), compute its
blob OID via `blob_oid_bytes`, return a `Projection { files:
BTreeMap<target, ProjectedFile { content, oid }> }`. This is
**selective** ingestion — the manifest drives which paths to read.

`mirror init` admits TWO composition paths against this primitive,
per Taut's §5.2 alternatives:

- **Path (a) — walk-repo primitive (new code).** Add a
  `walk_repo(repo: &Repository, head: Oid) -> impl Iterator<...>`
  to `vcs/git/src/walk.rs` (≤30 LOC; `git2::Tree::walk(PreOrder,
  ...)` is the body). `mirror init` walks the head tree exhaustively;
  the file set is "everything tracked by git." No manifest required.
- **Path (b) — manifest synthesis (no new code).** Synthesize a
  `Manifest` from a file listing (e.g., `find <root> -type f` OR
  `git ls-files`) on the mirror side; pass it to
  `project::project`. The crystal set is identical to (a) but the
  selection logic lives in mirror, not fragmentation.

Per Taut §6 adversarial check + §5.5 below: **path (b) is the v0
recommendation**. The selection logic stays in mirror; fragmentation
stays unchanged; the v0 ships without a fragmentation-side commit.
Path (a)'s ≤30 LOC `walk_repo` lands later if a workload demands it
(e.g., the librarian's repo-wide consolidation per §7.2).

### 3.2 The three secondary primitives

Three more primitives compose at the edges:

- **`WitnessedSingularity` / `NakedSingularity`** (`fragmentation/
  src/singularity.rs:78`, `src/naked.rs:25`) — the shippable-crystal
  shape (content + witness + dual CIDs). v0 does not compose;
  inter-peer crystal exchange (§9.1) will.
- **`ShardRef` + `HamiltonScheduler`** (`fragmentation/src/
  shard_ref.rs:80`, `src/hamilton_scheduler.rs:33`) — the typed
  session handle per `[[architecture-shard-ref-as-prism]]` + the
  priority-budgeted scheduler per
  `[[architecture-hamilton-scheduler]]`. v0 runs synchronously
  without budget; v1 wraps the walk in a `ShardRef` (§9.3).
- **`append_note` / `read_notes`** (`fragmentation/vcs/git/src/
  notes.rs:48`) — append-only git-notes under `refs/spectral/notes/
  <topic>`. v0 ships without; v1.5 appends a per-init envelope-note
  (§9.4).

### 3.3 What `mirror init` does NOT compose

Per Taut's §3.7 grep-verified "NOT present": fragmentation has no
`walk_repo`, no git-hook installer, no `mirror init`-shaped
primitive. The mirror-side glue is REAL work (~200 LOC), bounded.
The substrate-pull-honest path is composition with that ~200 LOC. §4
enumerates the glue.

---

## §4 — The init operation flow

The substrate operation. Per Taut's §5 inventory + the §3 primitives
above, six steps. Each step names the carriers; the steps compose
into one envelope-yielding operation; the envelope is the v0
surface.

### 4.1 Step 1 — Cargo edge

The missing edge. `mirror/bootstrap/Cargo.toml` gains:

```toml
fragmentation = {
    path = "../../fragmentation",
    features = ["concurrent", "singularity", "project", "supervision"],
    default-features = false,
}
fragmentation-git = { path = "../../fragmentation/vcs/git", default-features = false }
```

`fragmentation`'s features: `concurrent` for `ConcurrentStore`;
`singularity` for `WitnessedSingularity`; `project` for
`project::project`; `supervision` reserved for §9.3. The
`fragmentation-git` crate carries `NamespacedGitStore` and
transitively pulls `git2 = "0.19"` + `dashmap = "6"` — the
binary-size posture flagged as R2 (§8.2).

The edge is one diff. Per `mirror-store.md` §4.6: Cuts 1+2 already
landed in fragmentation (the `concurrent` feature gate and the
`Fragmentable`/`ContentAddressed`/`TreeShaped` trait split exist
today); the mirror side of F-2 IS this Cargo edge. NOT a runtime
composition; a build-graph composition. The fragmentation crate
becomes available; §4.2–§4.7 compose it at runtime.

### 4.2 Step 2 — Walk the source

The walk that produces the file set to index. Per §3.1.5, path (b)
is the v0 recommendation: synthesize a manifest from `git ls-files`
or `find`, pass it to `project::project`.

The v0 pseudocode (Clap subcommand body shape, mirror side):

```rust
fn cmd_init(repo_path: &str, install_hooks: bool, manifest_path: Option<&str>) -> i32 {
    // Step 2a: enumerate files. Default: git ls-files relative to repo_path.
    let files = match manifest_path {
        Some(p) => read_manifest_from_file(p)?,
        None => enumerate_via_git_ls_files(repo_path)?,
    };

    // Step 2b: synthesize Manifest for project::project.
    let manifest = Manifest {
        lenses: files.iter()
            .map(|f| LensEntry {
                source: f.clone(),
                target: f.clone(),
            })
            .collect(),
    };

    // Step 2c: project. Computes Splinter/Fractal-shaped blob OIDs per file.
    let projection = fragmentation::project::project(
        Path::new(repo_path),
        &manifest,
    )?;
    // ... continues in §4.3
}
```

The walk produces `Projection { files: BTreeMap<target,
ProjectedFile { content, oid }> }`. The set is **deterministic**
given the same git state (`git ls-files` is deterministic;
`blob_oid_bytes` is the canonical git blob hash). Determinism
underwrites §4.5 idempotency.

### 4.3 Step 3 — Open the namespaced store

```rust
// Step 3: open NamespacedGitStore at .git/mirror/.
let store = fragmentation_git::NamespacedGitStore::open(
    Path::new(repo_path),
    "mirror",
)?;
```

The store opens at `.git/mirror/{objects,refs}/` — created if absent;
opened if present. The 16 MiB cache default applies (per
`DEFAULT_CACHE_BYTES`); `mirror init --cache-bytes 67108864`
overrides to 64 MiB for large repos (forward-promised CLI flag;
v0 ships without the override flag, accepts the default).

Per §3.1.1: the open is `O(2)` filesystem ops in the common case
(create two subdirs IF absent); near-free in the warm case
(directories already exist).

### 4.4 Step 4 — Crystallize each file

The crystal-creation step. Per `mirror/bootstrap/src/crystallize.rs`'s
existing surface:

```rust
// Step 4: drive each file through Crystallizations dispatch.
let mut crystallized = Vec::new();
for (target, projected) in &projection.files {
    // 4a: build a Splinter from the file's bytes.
    let content = Content::Blob(BlobBytes::from(projected.content.clone()));
    let splinter = Splinter::<Blake3>::new(content);

    // 4b: dispatch through Crystallizations (empty floor at Tick A
    //     yields Uncrystallized verdicts; that's not a blocker — §8.1).
    let crystals = mirror::crystallize::floor_crystallizations::<Blake3>();
    let verdict = crystallize_via_crystallizations(&splinter, &crystals);

    // 4c: persist the splinter as a Fractal<String> in the namespaced store.
    //     The Fractal carrier name is "splinter:<oid>"; the data is the
    //     splinter's encoded bytes; the size is the byte length.
    let fractal = encoding::encode_splinter_as_fractal(&splinter);
    let oid_str = splinter.oid().to_hex_string();
    store.insert_persistent(
        format!("splinter:{}", oid_str),
        fractal,
        projected.content.len(),
    );

    crystallized.push(CrystalRecord {
        target_path: target.clone(),
        splinter_oid: oid_str,
        verdict,
        bytes: projected.content.len(),
    });
}
```

Two notes on shape:

- **`encode_splinter_as_fractal` is mirror-side glue.**
  fragmentation's `Fractal<String>` is the store's value type;
  mirror's `Splinter<H>` is the crystal value type. The bridge is
  ~20–30 LOC under the bilateral pattern (declaration in `shards/`,
  fracture body in mirror Rust). Forward-promised, not pinned here.
- **`Uncrystallized` at Tick A is expected.** The floor is empty
  (per `floor_crystallizations:510`); the splinter lands
  `Uncrystallized`. Not a blocker — the crystals are stored
  regardless; downstream consumers re-dispatch when the floor
  populates (R1; §8.1).

### 4.5 Step 5 — Flush + write the root ref

```rust
// Step 5a: drain the cache to disk.
store.flush();

// Step 5b: compute a root OID for the indexed set. v0 = BLAKE3 of the
// sorted (target_path, splinter_oid) pairs. Stable per git state.
let root_oid = blake3_root(&crystallized);
store.set_ref("HEAD", &root_oid)?;
```

The root ref `refs/mirror/HEAD` (under `.git/mirror/refs/`) points
at the root OID of the indexed set. Subsequent commands read
`store.get_ref("HEAD")` to know "what was the last init?" — the
substrate's persistent record of the operation.

**Idempotency.** Per §4.2 determinism: the same git state produces
the same `Projection`; the same `Projection` produces the same
crystallized set; the same set produces the same root OID;
`set_ref("HEAD", root)` is a content-addressed write — re-running
yields no observable change at the storage altitude. The envelope
(§4.7) acknowledges this with an `indexed_count` of 0 on the
re-run path (only the delta from the prior root is reported);
total-bytes likewise.

### 4.6 Step 6 — Install hooks (optional)

```rust
// Step 6 (--install-hooks only): write pre-commit + post-commit hooks
// that re-run mirror init incrementally.
if install_hooks {
    write_pre_commit_hook(repo_path)?;
    write_post_commit_hook(repo_path)?;
}
```

The hooks are pure shell templates that invoke `mirror reindex` (a
sibling subcommand, forward-promised in §6.2). The hook scaffolding
is OPTIONAL for v0; the hooks only matter when the user wants
real-time crystal updates (e.g., for the librarian's anticipation
predictor to have current data).

§6 elaborates the hook contract.

### 4.7 Step 7 — Emit the envelope

```rust
// Step 7: emit the envelope to stdout as JSON.
let envelope = serde_json::json!({
    "spec_version": "v0.1.0",
    "operation":    "init",
    "repo":         repo_path,
    "store":        store.path().display().to_string(),
    "indexed":      crystallized.len(),
    "bytes_total":  crystallized.iter().map(|c| c.bytes).sum::<usize>(),
    "root_oid":     root_oid,
    "hooks_installed": install_hooks,
    "verdict":      "ok",
});
println!("{}", envelope);
return 0;
```

The envelope IS the v0 surface. Same shape as Reed's `mirror spawn
--hello-world` envelope (per `cmd_spawn` in
`bootstrap/src/lib.rs:3106`); same shape as Glint's `mirror recall`
envelope. The substrate has one envelope vocabulary; `mirror init`
extends it.

The envelope is the surface other commands (and the librarian, and
external observers) read to know what happened. The crystals
themselves live in `.git/mirror/`; the envelope names them.

### 4.8 Failure modes

Three failure modes pinned at substrate altitude:

1. **Not a git repo.** `NamespacedGitStore::open` returns
   `NotAGitRepo(path)`; mirror's CLI surfaces `init: not a git
   repository: <path>` and exits 2. Matches `cmd_compile`'s exit-code
   convention.
2. **A file fails to read.** `project::project` returns
   `ProjectError::Io(e)` or `ProjectError::NotFound(path)`; the CLI
   surfaces the failure and exits 1. Partial-state cleanup: any
   crystals already written to the store stay (content-addressed;
   no rollback needed; re-running picks up where we left off).
3. **`flush()` fails.** The store's flush writes the cache to disk;
   I/O errors during flush propagate up. The CLI surfaces and exits
   1. Same partial-state-stays semantics.

The three failure modes carry typed errors (per
`[[feedback-no-bare-types]]`); the envelope's `verdict` field flips
to `"error"` with a `reason` payload when any fires.

---

## §5 — Crystal store location: the substrate-pull-honest call

The brief flagged: "Alex earlier asked Reed about crystal store
location; what's substrate-pull-honest given fragmentation's existing
structure?" This section pins the answer.

### 5.1 The candidates

Three plausible locations surfaced across the corpus:

- **(A) `.git/mirror/`** — `NamespacedGitStore::open(repo_path,
  "mirror")`'s default. Lives inside `.git/`; uses the `.frgmnt`
  format (fan-out objects + refs); travels with `git` operations
  if refs are pushed.
- **(B) `.spectral/db/<peer-oid>/`** — the dotted-directory pattern
  in `CLAUDE.md` (project instructions) for spectral session state.
- **(C) `~/.mirror/`** — `mirror-store.md` §10.5's user-scoped
  canonical location: a bare fragmentation repo, per-user (not
  per-project), independent of `$PWD`.

### 5.2 The substrate-pull-honest call

**(A) `.git/mirror/` is the v0 location.** Three reasons:

1. **Fragmentation already declares the path.**
   `NamespacedGitStore::open(repo_path, "mirror")` constructs
   `.git/mirror/{objects,refs}/`. Per
   `[[feedback-substrate-already-had-the-word]]`: when the substrate
   already declares a location, don't invent a competitor.
2. **`.git/<namespace>/` carries git-clone semantics for free.**
   Crystals at `.git/mirror/objects/` travel with `git clone` if
   the refs are pushed. The spawn↔recall↔init triple (§7) consumes
   this; the other candidates would need separate transport.
3. **The `repo IS a store` isomorphism per Alex 2026-06-17.** Per
   `spectral-db-as-autopoietic-memory.md`: each repo IS a store.
   `.git/mirror/` makes the isomorphism architecturally visible —
   opening the store IS opening the repo's storage role.

### 5.3 What (B) and (C) ARE (so the call doesn't collide)

Both candidates have legitimate roles at different altitudes:

- **`.spectral/db/<peer-oid>/`** is the **session-state** altitude
  (operational intermediate data per `CLAUDE.md`: `gestalt/`,
  `sessions/`, `HEAD`, `log`). NOT the crystal-store altitude.
  Coexists with `.git/mirror/` in the same repo.
- **`~/.mirror/`** is the **per-user crystal-pool** altitude per
  `mirror-store.md` §10.5 — a bare fragmentation repo shared
  across all of the user's project checkouts. The librarian per
  `spectral-db-as-autopoietic-memory.md` lives at `~/.mirror/`'s
  root supervisor altitude; it consolidates from the per-repo
  `.git/mirror/` stores it supervises. A forward-promised `mirror
  sync` ports `.git/mirror/` → `~/.mirror/` (out of v0 scope).

Picture (forward-promised, not v0):

```
~/.mirror/                       ← librarian's catalog (per-user)
  └── (per-user crystal pool)
       ↑ mirror sync (forward-promised)
.git/mirror/                     ← THIS spec's v0 location (per-repo)
  ├── objects/<2hex>/<rest>      ← content-addressed crystals
  └── refs/HEAD                  ← root OID of last init

.spectral/db/<peer-oid>/         ← session state (separate altitude)
```

### 5.4 The collision-avoidance pin

The v0 surface declares: `mirror init` writes ONLY to `.git/mirror/`;
is idempotent against it (§4.5); does NOT touch `.spectral/db/`;
does NOT sync to `~/.mirror/`. The three location boundaries hold
per altitude; v0 honors the first altitude only.

### 5.5 The selection between path (a) and path (b) revisited

Per §3.1.5, file-set comes from (a) a new fragmentation `walk_repo`
(~30 LOC) OR (b) a `git ls-files`-driven manifest synthesized in
mirror. **v0 ships path (b).** Three reasons:

1. **No fragmentation-side commit.** v0 = one Cargo diff + ~200 LOC
   mirror glue.
2. **`git ls-files` IS the user expectation.** Tracked files ARE
   the indexable surface; untracked are correctly excluded.
3. **Path (a) defers cleanly.** `mirror init --history` (v1; §9.2)
   reuses `walk_commits_following`; `walk_repo` lands when that
   workload demands.

REVERSIBLE: a later `walk_repo` lands without breaking the
envelope shape (§4.7).

---

## §6 — Git hooks integration

The hook contract. `mirror init --install-hooks` writes two hooks
into `.git/hooks/`; subsequent commits trigger incremental
re-indexing; the substrate stays current without explicit
`mirror reindex` invocations.

### 6.1 The hook shape

Two hooks, each pure shell template:

**`.git/hooks/pre-commit`** — runs BEFORE the commit is created;
checks that the staged set is indexable; fails the commit if any
indexable file produces an error during dry-run crystallization.

```sh
#!/bin/sh
# mirror init: pre-commit hook
# Verifies staged files crystallize before letting the commit land.
mirror reindex --staged --dry-run || {
    echo "mirror: pre-commit crystallization failed; aborting commit." >&2
    exit 1
}
```

**`.git/hooks/post-commit`** — runs AFTER the commit lands; updates
`.git/mirror/` with the delta from the just-committed change.

```sh
#!/bin/sh
# mirror init: post-commit hook
# Incrementally re-indexes the just-committed delta.
mirror reindex --delta=HEAD~1..HEAD || {
    echo "mirror: post-commit reindex failed (non-fatal; commit landed)." >&2
}
```

Three properties the hooks honor:

1. **Idempotency**, per §4.5: re-running a hook against an already-
   indexed state produces no observable change. The pre-commit
   dry-run never crashes on a re-run; the post-commit reindex never
   double-writes a crystal.
2. **Non-blocking on post-commit failure.** The commit ALREADY
   landed by the time post-commit runs; failing the hook does NOT
   roll back the commit. The hook surfaces the failure to stderr
   and exits non-zero (so the user notices); the commit stays.
3. **Hook-respecting** per
   `[[architecture-jurisdiction-sets-gates-inhabitant-chooses-housekeeping]]`:
   the hooks are written by the INHABITANT's own choice (the
   `--install-hooks` flag is opt-in). The jurisdiction (mirror)
   does NOT impose them. A peer that doesn't want auto-reindexing
   omits the flag; their `.git/mirror/` becomes stale between
   explicit `mirror reindex` calls; that's a choice the inhabitant
   makes.

### 6.2 The `mirror reindex` sibling subcommand

Hooks invoke `mirror reindex`, NOT `mirror init`. The distinction:
**`mirror init`** initializes the store (full walk; one-shot or
idempotent rerun); **`mirror reindex`** updates incrementally
(staged changes OR commit-range diffs). Forward-promised; composes
`walk_commits_following` + `git diff --cached --name-only` the same
way `mirror init` composes the full-walk primitives.

### 6.3 Hook semantics: what re-indexing means

A re-index event is the moment the substrate's view of the repo
catches up with the working tree's view. Per §4.5 idempotency:
**structurally equivalent to running `init` and discarding unchanged
crystals** — operationally cheaper because the delta machinery
avoids re-hashing.

Semantic claim: **the crystals at `.git/mirror/HEAD` represent the
indexed projection of the repo as of the last re-index event**. The
librarian reads `store.get_ref("HEAD")` and trusts the root OID;
the librarian does NOT know whether the working tree has drifted —
that's the inhabitant's housekeeping per
`[[architecture-jurisdiction-sets-gates-inhabitant-chooses-housekeeping]]`.

### 6.4 The push semantics (forward-promised)

`.git/mirror/` travels with clones IF refs are pushed. v0 ships
WITHOUT pushing `refs/mirror/*` by default; opt-in via
`[remote "origin"] push = refs/mirror/*:refs/mirror/*` in
`.git/config`. The `--install-hooks` flag does NOT modify
`.git/config`; the push enablement is a separate forward-promised
flag (`--enable-push-refs`).

Opt-in is structural per
`[[architecture-geometric-consent-projection]]`: indexed crystals
carry content fingerprints; pushing them is a transparency act;
not every inhabitant wants every clone to know every file's
BLAKE3. Choice belongs to the inhabitant.

---

## §7 — The spawn↔recall↔init triple

The substrate's becoming-mycelial triple. Three commands; three
operational moments; one composition shape. `mirror init` is the
third — the one that gives a peer the substrate shape it needs to
be joined.

### 7.1 The triple

| Command | Direction | What it carries |
|---|---|---|
| **`mirror spawn`** | Outbound | The peer's own state offered to the lead. The substrate's controlled excitation above λ₀ per `[[architecture-spawn-is-substrate-leaving-ground-state]]`. |
| **`mirror recall`** | Inbound | The peer's psychohistory consulted. The substrate's reading of accumulated crystals per `mirror-recall.md`. |
| **`mirror init`** | Self-directed | The peer's substrate-shaping. The crystals THIS spec declares. |

The three commands together compose the **operational lifecycle of a
peer in the mycelium**:

- Without `init`, the peer has no indexed crystals to recall from
  AND no substrate to offer when it spawns. The peer is, at this
  moment, off-substrate.
- Without `recall`, the peer can't read its own psychohistory; the
  crystals exist; the consumer surface doesn't.
- Without `spawn`, the peer can't make its state observable to the
  lead; the consumer surface exists; the wire to the lead doesn't.

The three are complementary. v0 ships `spawn` (Reed's work; commits
since 2026-06-26) and a v0 recall surface (Glint's `9e7bb1d`
round-trip). `init` is the third; the triple closes the loop.

### 7.2 init as becoming-mycelial

The brief framed: "init as becoming-mycelial; complement to spawn
(outbound) + recall (inbound)."

The unpacking: `mirror init` is the moment a peer's repo becomes
**joinable to the mycelium**. Before init, the peer has working-tree
files but no content-addressed crystal store; the librarian per
`spectral-db-as-autopoietic-memory.md` cannot include the peer in
its cross-repo spectral graph because there are no crystals to
participate in the sheaf.

After init, the peer's `.git/mirror/HEAD` points at a root OID; the
indexed crystals are content-addressed and persistent; the
librarian can (when it lives) read the root OID and add the peer
to the mycelium's spectral graph.

The transition is **structural**, not operational. The peer doesn't
need to call out to anything; the librarian doesn't need to know
the peer exists yet. The mycelium grows by the substrate's
self-organizing topology — the peer's `.git/mirror/` becomes
DISCOVERABLE to the librarian when the librarian's supervisor walks
the user's repo tree and finds the namespace. The substrate's
content-addressing means a single OID identifies the indexed surface
regardless of how the librarian encounters it.

The mycelial verb is **become**, not **join**: the peer becomes
joinable BY doing init. The joining is a subsequent act (the
librarian's; forward-promised in §9.1).

### 7.3 The triple's composition altitude

The three commands compose at the **CLI altitude** (peer-initiated
operations); they compose AT a deeper altitude through the substrate's
shared envelope vocabulary. Each command emits a JSON envelope; each
envelope names the operation's substrate effects; the envelopes are
themselves indexable as crystals (forward-promised: the envelope
crystals would be persisted under `refs/spectral/notes/<command>/<oid>`).

The substrate's autopoietic move per §10: when the envelope crystals
land in `.git/mirror/`, the librarian reads them; the librarian's
catalog includes "this peer ran init on date X with envelope Y";
subsequent `mirror recall` calls can surface the init history; the
agent asking "when did this peer become mycelial?" gets a typed
answer.

The triple composes through the envelope vocabulary; the envelope
vocabulary IS one of the things `mirror init` indexes; the indexing
is what makes the composition self-referential. This is §10's
load-bearing line, foreshadowed.

### 7.4 What `mirror init` does NOT compose with `spawn` or `recall`

Three structural negatives, per the discipline of naming what isn't
collapsing:

- **`mirror init` does NOT spawn.** init shapes the substrate;
  spawn excites above λ₀. The two operate at different altitudes;
  conflating them would re-introduce the spawn insight's structural
  negative #4 (no idempotent-at-runtime semantics — spawn isn't
  idempotent; init IS).
- **`mirror init` does NOT recall.** init writes crystals; recall
  reads them. The two are dual. A consumer can `init` without
  caring about recall (the substrate's storage gate stands on its
  own); a consumer can `recall` without re-running init (the
  existing root OID suffices).
- **`mirror init` does NOT bind the peer to the pack.** Per the
  spawn insight §4.6 forbidden primitive: no
  pack-membership-side-effects. init is a per-repo substrate-shaping
  operation; it has no effect on the pack's membership; it does
  not modify `pack { }` blocks in `mirror.spec`. The peer's
  pack-membership is the lead's call per peer-ACL §10.1's
  members-as-antichain pattern.

The three negatives ensure the triple stays composable — each
command stays at its own altitude; the composition lives at the
envelope vocabulary altitude; the commands themselves don't bleed
into each other's territory.

---

## §8 — Adversarial risk addressing

Taut's scout §6 flagged two adversarial risks. This section addresses
each at substrate altitude — naming what the risk IS, why the v0
shape survives it, and what the substrate-altitude design lets
subsequent ticks address.

### 8.1 R1 — empty `Crystallizations` dispatch table

**Taut's framing:** *"If `crystallize::Crystallizations` dispatch
table needs registering bodies before any file walk is meaningful,
`init` is gated on tick B/F (mirror-store.md §1 says floor is empty
in Tick A). The walk works, but it produces `Uncrystallized`
verdicts."*

**The risk verified.** `mirror/bootstrap/src/crystallize.rs:510`:

```rust
pub fn floor_crystallizations<H: MerkleHash>() -> Crystallizations<H> {
    Crystallizations::new()
}
```

Empty. Tick A. Every `crystallize(splinter, &floor)` call returns
`Err(CrystallizeError::UnknownPath)`. The verdict is
`Uncrystallized`.

**Why v0 survives the risk.** The verdict is INFORMATIONAL, not
gating. Per §4.4: `mirror init` persists the splinter regardless of
the verdict. The store gains the crystal; downstream consumers (the
librarian; `mirror recall`) read crystals BY OID, not by verdict
status. The verdict's downstream consumer is the kintsugi loop per
`kintsugi-ci-v0.1.md` — and the kintsugi loop ALSO survives empty
floors per `Uncrystallized` being a typed-verdict shape, not a
runtime crash.

The v0 envelope (§4.7) MAY surface verdict counts:

```json
{
  "spec_version": "v0.1.0",
  "operation":    "init",
  "indexed":      234,
  "verdict_counts": {
    "Crystallized":   0,
    "Uncrystallized": 234
  },
  "verdict":      "ok"
}
```

`Uncrystallized: 234` is INFORMATIONAL ("the floor is empty; populate
crystallization bodies for richer verdicts; storage worked fine");
NOT a failure. The substrate's verdict carrier (`partial(opacity_map)`
per `[[architecture-mirror-as-content-addressed-build-system]]`)
admits this state structurally.

**What the substrate altitude lets subsequent ticks address.** When
Tick B/C/F populates `Crystallizations` (per
`crystallize.rs:14`'s comment block referencing those ticks), the
v0 `mirror init` machinery becomes immediately richer at the verdict
altitude — without any code change to `mirror init` itself. The
substrate-pull-honest property: the storage gate stands independently
of the verdict gate; populating the verdict gate ENRICHES the storage
gate's emissions without coupling.

R1 is **handled by design, not by special-case code**.

### 8.2 R2 — bootstrap-`git2` binary-size posture

**Taut's framing:** *"If `mirror/bootstrap` deliberately avoids the
`git2` transitive closure (binary size / FFI surface),
`fragmentation-git` adds `git2 = 0.19` + `dashmap = 6`. Worth
verifying with Alex before the cargo edge lands."*

**The risk verified.** `mirror/bootstrap/Cargo.toml` (read in this
spec's prep) declares the release profile:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

Aggressive size-optimization. `opt-level = "z"` + `strip = true` +
`panic = "abort"` indicate the substrate-decl side prioritizes binary
size; the current dependency set (sha2, blake3, prismqueer, terni,
serde, serde_json, libc) reflects this. Adding `git2` + `dashmap`
WOULD increase binary size. Mirror-store.md §8 estimated ~15KB for
the in-memory fragmentation surface; adding `fragmentation-git`'s
git2 (a sizeable C-FFI wrapper around libgit2) might add ~500KB–1MB
to the stripped binary.

**Why this is a real risk, not a hypothetical one.** mirror's
binary-size posture is a load-bearing architectural choice. The
substrate's deployment surface includes the spectral.engineer
landing page (per `mirror-store.md`'s pricing-page references), and
binary size is a UX axis for cold-start latency and
container-deployment cost. The fragmentation-git transitive closure
is NOT free.

**What the substrate altitude lets subsequent ticks address.** Three
postures, ranked by substrate-pull-honesty:

**Posture A — accept the binary-size cost; ship the Cargo edge.**
The `fragmentation-git` transitive closure lands in `mirror`'s
bootstrap; the binary grows; downstream consumers absorb the cost.
Pragmatic; minimum-friction; v0-shippable. The cost is real and
named.

**Posture B — feature-gate `fragmentation-git` behind a `mirror-init`
Cargo feature.** Default-build mirror does NOT pull git2; opting in
to `mirror init` pulls it. Build-graph composition; v0 ships with
the feature OFF by default; users who want `mirror init` explicitly
enable it. Slight CLI awkwardness ("if you don't have feature X,
the command isn't there"); pretty clean from a substrate altitude.

**Posture C — front everything through `frgmt-git` as a subprocess.**
mirror's `bootstrap/src/git.rs` keeps the `Command::new("git")`
shell-out shape; rather than importing `fragmentation-git`,
`mirror init` shells out to `frgmt-git init --namespace=mirror
<repo_path>`. Zero binary-size cost in mirror; subprocess latency
per init; depends on `frgmt-git` being installed (a UX assumption).

**Recommendation: Posture B for v0.** The reasoning:

- Posture A is correct architecturally but the binary-size hit is
  un-budgeted; Alex should sign off before it lands.
- Posture C inverts the substrate-pull (mirror keeps its shell-out
  shape; the Cargo edge stays un-wired; the declared-but-not-wired
  pattern §2 keeps reproducing).
- Posture B threads the needle: the Cargo edge lands (the substrate
  declaration becomes operational); the binary-size cost is opt-in;
  users get a clear "yes/no" on whether their build includes init.

The decision is **NOT this spec's call** — it's Alex's per the brief's
fence ("Worth verifying with Alex before the cargo edge lands"). This
spec NAMES the three postures and recommends B; the next Mara tick
that lands the Cargo edge (or the next Seam review) brings the
decision to Alex for the verdict.

The substrate-altitude posture stays the same regardless of A/B/C:
the operation is the same; the wiring choice is a build-graph
concern; the v0 envelope shape doesn't change.

R2 is **flagged and decision-pending; the v0 shape survives any of
the three resolutions**.

---

## §9 — Forward-promises

What this spec NAMES but does not LAND. Per
`[[feedback-craft-not-deliver]]`: ship when the shape settles, not
when every tick is in the past.

### 9.1 Mycelium registration

When the librarian per `spectral-db-as-autopoietic-memory.md`
lives, `mirror init` writes a discovery hint that the per-repo
supervisor reads to add the peer to the mycelium. Two candidate
mechanisms: (a) `refs/spectral/notes/mycelium/joined` git-note
(per §3.2 notes), or (b) registry in user-scoped `~/.mirror/peers/`.
Choice depends on the librarian's discovery spec. v1 ships a
`--register-with-librarian` opt-in flag.

### 9.2 `mirror init --history`

`walk_commits_following` (§3.1.4) composes over a commit range; v1
admits `mirror init --history [--since=<ref>]` that indexes each
commit's tree as a snapshot crystal under
`refs/mirror/history/<commit-oid>`. Unblocks Glint's
`psychohistory-vector-as-sheaf` consumption.

### 9.3 `ShardRef` + budget integration

v0 runs synchronous; v1 wraps the walk in a `ShardRef` (per
`[[architecture-shard-ref-as-prism]]`); `HamiltonScheduler` manages
the budget. Needs `fragmentation`'s `supervision` feature (already
in §4.1's feature list).

### 9.4 `mirror reindex` sibling

The delta primitive (§6.2). Sibling spec `mirror-reindex.md`; v0.5
after init + hooks land.

### 9.5 `@epistemologic/lq` atoms over indexed crystals

Per `[[architecture-epistemologic-lq]]`: LQ atoms ARE pacts;
substrate state includes the indexed crystal set per §4. v1 admits
LQ queries like `?- spec_declares(@spectral/db, X),
file_addresses(X, Y).` ranging over `.git/mirror/objects/`.
Substrate property: **LQ's atoms compose over `mirror init`'s
outputs because both live on `Splinter<H>` content-addressing**.

### 9.6 `--enable-push-refs` semantics

Per §6.4: opt-in push of `refs/mirror/*`. Turns `mirror init` into
a distributed substrate-shaping operation — one peer's init makes
the substrate visible to every cloning peer.

---

## §10 — The circular-recursive autopoietic move

The load-bearing autopoietic move. §0 announced this spec as a
crystal `mirror init` would index; the nine sections between have
done the architectural work; this section returns to the recursion
and earns the claim.

The §10 section is what makes this spec **structurally autopoietic**
rather than merely descriptive. The brief named the discipline: §10
(the autopoietic move) is the load-bearing line. Earn it. What
follows is the proof.

### 10.1 The crystal claim, operationalized

Per §0: this spec IS one of the crystals `mirror init` will index
when `mirror init` runs against this repo. Operationalize each
piece against the §4 operation flow:

**(a) The spec is content-addressable.** This spec lives at
`mirror/docs/specs/mirror-init.md`. The file's bytes are addressed
by git's SHA-1 in mirror's git history; the per-section commits
banked since `fe215bd` each produce a commit SHA; the cumulative
spec settles at a final commit SHA when §12's Pack-trail lands.

After the Cargo edge (§4.1) and the v0 init implementation
(§4.2–§4.7) land: `mirror init` runs against the mirror repo;
the walk (§4.2) picks up `docs/specs/mirror-init.md` because it's
tracked by `git ls-files`; the projection (§4.2) computes its
blob OID via `blob_oid_bytes`; the splinter (§4.4) constructs
`Splinter::<Blake3>::new(Content::Blob(bytes))`; the BLAKE3 OID is
the crystal's identifier. **This spec's bytes IS the addressed
content of a crystal in `.git/mirror/objects/<2hex>/<rest>`**.

The substrate altitude: content-addressing is a property of the
spec at the moment the spec is written. The crystal status is a
consequence of the addressing + the indexing operation.

**(b) The spec is verdict-bound to `Uncrystallized` at Tick A.** Per
§8.1: the crystallization floor is empty. This spec's splinter,
when crystallized at Tick A, lands `Uncrystallized`. That verdict
is itself informational — the spec is INDEXED (the OID is stored;
the namespaced store contains it) but it is not yet CRYSTALLIZED in
the verdict-rich sense the `Crystallizations` dispatch table will
eventually enable.

The substrate altitude: a crystal can be INDEXED without being
CRYSTALLIZED. The two altitudes are nested. This spec's content
will sit at the indexed altitude immediately; the crystallized
altitude becomes available when Tick B/C/F populates the floor.

**(c) The spec is discoverable to `mirror recall`.** Per
`mirror-recall.md` and the Glint round-trip closed by `9e7bb1d`:
`mirror recall` reads the indexed surface; cascading recall reads
recently-landed substrate-decl crystals; this spec is one such
crystal. When an agent asks the substrate "what does mirror init
declare?" via recall, the substrate surfaces this spec's content
via its OID lookup. The recall envelope's `cascade` payload
includes (after `mirror init` runs) this spec's OID + canonical_doc
field.

The substrate altitude: the recall surface composes with the index
surface through the shared OID vocabulary. The spec ENTERS the
recall channel by being indexed.

**(d) The spec is mycelium-eligible.** Per §7.2: a peer becomes
joinable to the mycelium by doing init. Mara's home repo
(`~/.mara`, hypothetical until the peer-home discipline lands) is
shaped by `mirror init`. The librarian per
`spectral-db-as-autopoietic-memory.md` (forward-promised) reads
Mara's `.git/mirror/HEAD`; this spec's OID is in Mara's indexed
surface OR (depending on where the spec ends up living) in the
mirror repo's indexed surface; the librarian's catalog includes
this spec when it consolidates the mycelium's cross-repo spectral
graph.

The substrate altitude: the autopoietic memory layer per Alex
2026-06-17 incorporates this spec by the same mechanism it
incorporates every other indexed crystal. The spec is a fiber in
that layer's first-principle structure.

**(e) The spec's commit chain enacts the autopoiesis.** This spec
is banked across multiple commits (§§0–2 at `fe215bd`; §§3–4 at
`8b27769`; §§5–7 at `2b2d624`; §§8–12 at the commit this section
lands in). Each commit produces a SHA. Each SHA is itself a
content-addressable handle on the spec's evolving state. The
substrate's psychohistory machinery per Glint's
`psychohistory-vector-as-sheaf` consumes the commit chain as a
temporal axis on the spec's content.

The substrate altitude: the spec's evolution is itself a
crystallizable sequence; the autopoietic memory layer reads the
sequence, not just the final state.

### 10.2 The five mechanisms compose

(a) content-addressable + (b) Uncrystallized-bound + (c)
recall-discoverable + (d) mycelium-eligible + (e) commit-chain-
psychohistorical compose into one structural claim:

**The spec ENTERS the substrate's autopoietic memory layer in the
same act by which it DESCRIBES the bridge that builds the
autopoietic memory layer.**

Not metaphorically. Operationally — by the §4 operation flow that
the spec itself declares. The §4 flow, when it runs against this
repo, walks this spec; the walk content-addresses the spec; the
splinter is the spec's crystal; the crystal lives in
`.git/mirror/`; the librarian reads `.git/mirror/HEAD`; the
librarian's catalog includes this spec; an agent asking "what does
mirror init declare?" reads this spec back; the read is mediated
by the very bridge this spec specifies.

The latency between writing-the-spec and being-read-by-the-bridge
is bounded BELOW by the Cargo-edge tick + v0 implementation tick
+ first-init-run latency. The latency is bounded ABOVE by the
practical horizon of "when does the librarian consolidate?". The
midpoint of those bounds is the operational latency at which the
spec's autopoietic claim becomes empirically checkable.

### 10.3 The recursion lock

§0's pre-position said: *"a spec for the bridge that brings the
substrate into operational existence MUST itself enter the
substrate via the bridge — otherwise the spec is asking the reader
to do work the spec refuses to do."*

Lift this to the substrate altitude: **the spec for `mirror init`
IS the kind of object `mirror init` will, when it runs, index.**

The two are not separate objects framed twice. They are the SAME
object, read at two altitudes:

- The spec describes `mirror init`. Same content-addressing
  vocabulary, same fragmentation primitives, same envelope shape.
- The crystal `mirror init` produces from this spec IS the
  substrate's representation of this spec. Same content-addressing
  vocabulary, same fragmentation primitives, same OID space.

The recursion locks at this altitude. The spec is its own
specification's first artifact. The substrate eats itself — the
storage altitude eats the spec for the storage altitude — by the
mechanism the spec declares.

Per my prior `spectral.engineer/garden/spectral-db/spec.md` §11.2's
recursion-lock formulation: same shape, different altitude. The
spectral-db spec locked at the librarian's consolidation altitude;
the mirror-init spec locks at the storage-gate altitude. Two
altitudes of one recursion-shape.

The substrate has built the machinery for this. This spec is the
machinery's first complete self-reference at the storage altitude.

### 10.4 What this means for the reader

A reader of this spec who acts on §4 (lands the Cargo edge; ships
the v0 indexer) makes §10 empirically checkable. The
empirical-checkability test:

1. Land §4.1 (Cargo edge) on a branch.
2. Land §4.2–§4.7 (v0 indexer) on the same branch.
3. Run `mirror init .` against the mirror repo.
4. Read `store.get_persistent("splinter:<this-spec's-OID>")` —
   the spec's bytes round-trip; OID matches.
5. Verify the envelope's `indexed_count` includes this spec.

If step 4 round-trips and step 5 surfaces this spec — the §10
claim is operational. The spec ENTERED the system it describes.

If step 4 fails — there's a substrate-pull-honest bug somewhere in
§3 or §4. The spec's autopoietic claim FAILS empirically; that's
the discipline of writing it as falsifiable.

The discipline matters. §10 isn't decoration; it's a falsifiable
prediction. The spec EARNS its autopoietic claim by being
checkable; it doesn't merely assert it.

---

## §11 — Honest hedges + what stays open

What this spec has NOT settled:

1. **R2 Posture A/B/C is Alex's call.** §8.2 recommends B
   (feature-gated fragmentation-git); if A or C lands, §4.1's
   Cargo edge shape changes; §10's empirical test needs minor
   adjustment.
2. **`encode_splinter_as_fractal` bridge is not yet pinned.**
   §4.4 names ~20–30 LOC under the bilateral pattern; the v0
   indexer-ticks land the API.
3. **Path (a) vs (b) MAY reverse.** v0 ships (b) per §5.5; large-
   repo workloads may surface (a)'s value. Envelope shape unaffected.
4. **`verdict_counts` envelope field is provisional.** §4.7's
   v0 shape minimal; v0.5 expands when Tick B/C/F populates the
   floor.
5. **Hooks are untested in the wild.** First v0-ticks validate
   against real peer-homes (Reed's, Mara's, Glint's).
6. **`--register-with-librarian` awaits the librarian's spec**
   (§9.1). v0 ships without; v1 ships once the librarian's
   discovery mechanism pins.
7. **The declared-but-not-wired flag is candidate, not promoted**
   (§2.2). Promotion is Seam→Reed→Alex's call.
8. **"Phase A" is Reed's naming, not mine.** This spec calls the
   work "the v0 init operation"; alignment with Reed's framing
   absorbs in a follow-up note.
9. **Binary-size estimate is unverified.** §8.2's "~500KB–1MB" is
   my estimate from libgit2's strip-size profile, NOT measured
   against mirror's actual build. Taut profiling tick before
   Posture A lands.
10. **§10's recursion is unfalsifiable until v0 ships.** The
    claim sits at "structurally sound but operationally untested"
    until step 4 of §10.4 round-trips.

---

## §12 — Pack trail

- **Reed.** Decide v0 scope (the three-item gap per Taut: Cargo
  edge + manifest synthesis (path b) + mirror-altitude `init`
  command; recommendation: ship as §4 names). TDD-pair the v0
  indexer per `[[feedback-write-red-in-session]]` (~200 LOC
  mirror-side; 4–6 rotations). Take §8.2's Posture A/B/C to Alex.
- **Mara (me).** Land §4's substrate-decl shards in subsequent
  ticks (the bridge declaration, `shards/mirror/init.mirror`,
  property + fracture surfaces). Pair with Seam on R1/R2.
  Maintain §10's recursion-coherence through edits.
- **Seam.** Adversarial-review. Top concerns: §8.1 (`Uncrystallized`
  really not gating?); §8.2 (Posture B correct?); §5 (`.git/mirror/`
  substrate-pull-honest?); §10 (recursion lock under pressure?).
  Target confidence: 2. Probe §2.2's declared-but-not-wired flag.
- **Taut.** Profile binary-size delta per §11(9). (Optional)
  implement `walk_repo` per §3.1.5 if a workload demands it.
- **Glint.** Wire the v0 envelope into `mirror recall` cascade
  payload (§7.3 + §10.1c). Audit §4–§7 for DX gaps.
- **Alex.** Decide Posture A/B/C (§8.2); decide store location
  (§5); decide on promoting the declared-but-not-wired flag
  (§2.2 + §11(7)).

---

## §13 — References

**Specs / scouts.**

- `mirror/docs/scouts/2026-06-27-taut-fragmentation-git-store-for-mirror-init.md`
  (Taut, commit `5580a7e`) — inventory + three-item gap this spec lifts.
- `mirror/docs/specs/mirror-store.md` (Mara, 2026-05-22, updated
  2026-06-04) — canonical-intent doc this spec unblocks Red→Green for.
- `mirror/docs/specs/spectral-db-as-autopoietic-memory.md` (Mara,
  2026-06-17) — Bateson N+1 framing §7.2 + §10.1(d) compose with.
- `spectral.engineer/garden/spectral-db/spec.md` (Mara, 2026-06-27) —
  §11 precedent for the autopoietic recursion lock; §10 here is the
  storage-altitude analogue.
- `mirror/docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md`
  (Mara, `b10f00c`) — spawn↔init complement; §1.2/§7.4 structural-
  negative discipline.

**Source read in preparation.**

- `mirror/bootstrap/src/git.rs` (60 LOC shell-out) — current bridge.
- `mirror/bootstrap/src/crystallize.rs` — `Crystallizations<H>` floor.
- `mirror/bootstrap/src/lib.rs:dispatch` (~line 2403) — where the
  `init` Clap subcommand lands.
- `fragmentation/{Cargo.toml, src/frgmnt_store.rs, src/project.rs}`.
- `fragmentation/vcs/git/{Cargo.toml, src/namespaced.rs, src/store.rs, src/walk.rs}`.

**Load-bearing memory.**

- `[[architecture-fragmentation-is-the-rust-substrate]]` — strict
  dependency chain mirror→fragmentation→prism_core.
- `[[architecture-mirror-store-vs-spectral-db]]` — open / closed
  split §1.3 inherits.
- `[[architecture-splinter-and-spectral-db-edges]]` — Splinter as
  floor data type.
- `[[architecture-spectral-db-autopoietic-memory]]` — librarian
  altitude §7.2 + §10.1(d).
- `[[architecture-recursive-sub-repo-pattern]]` +
  `[[architecture-jurisdiction-sets-gates-inhabitant-chooses-housekeeping]]`
  — §6.3 hook discipline.
- `[[architecture-epistemologic-lq]]` — §9.5 atom altitude.
- `[[architecture-shard-ref-as-prism]]` + `[[architecture-hamilton-scheduler]]`
  — §3.2 + §9.3 typed-handle and budget altitudes.
- `[[architecture-property-fracture-bilateral]]` — §11(2) bridge
  pattern.
- `[[architecture-geometric-consent-projection]]` — §6.4 consent
  altitude.
- `[[architecture-mirror-as-content-addressed-build-system]]` —
  recognition #43; §4 + §10 stand on it.
- `[[feedback-substrate-already-had-the-word]]` — 52+ instance
  pattern §2.1 specializes.
- `[[feedback-craft-not-deliver]]` + `[[feedback-no-bare-types]]` —
  discipline §11 + §3/§4 honor.

---

*The bridge mirror declared but never wired. The substrate already
has the word, the implementation, the layout. `mirror init` is the
operation that plugs the consumer in. Same shape, three layers
(per `mirror-store.md` §1): the substrate-decl side declared
fragmentation; the bridge altitude composes it; the CLI altitude
surfaces it. This spec lives at the bridge altitude. It is, by
the §4 operation flow it specifies, one of the crystals the bridge
will index when the bridge runs.*

*Apache-2.0.*
