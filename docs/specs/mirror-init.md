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

Three more primitives compose at the edges; named for completeness
because subsequent ticks will reference them.

#### 3.2.1 `WitnessedSingularity` / `NakedSingularity` — the shippable crystal

`fragmentation/src/singularity.rs:78` + `fragmentation/src/naked.rs:25`.

The substrate role: `WitnessedSingularity` collapses content under a
witness, producing a commit whose tree IS a `Lens` back to the
original (content-address invariant under observer; commit varies
with witness). `NakedSingularity` carries content + witness + dual
CIDs as a self-contained bundle — the "shippable crystal" shape.

`mirror init` does NOT compose these in v0 (no peer-export wire
exists yet). They're named here because the inter-peer crystal
exchange forward-promised in §9.1 will consume them: when a peer
exports a crystal to another peer's `.git/mirror/`, the carrier is
`NakedSingularity`.

#### 3.2.2 `ShardRef` + `HamiltonScheduler` — the typed session handle

`fragmentation/src/shard_ref.rs:80` + `fragmentation/src/
hamilton_scheduler.rs:33`.

The substrate role: `ShardRef { id: SpectralUuid, context:
ShardContext, budget_bytes: BudgetBytes }` is the typed session
handle per `[[architecture-shard-ref-as-prism]]`. `HamiltonScheduler`
is the priority-budgeted scheduler named for Margaret Hamilton
(per `[[architecture-hamilton-scheduler]]`).

`mirror init` v0 does NOT compose these — the v0 indexer runs as a
single synchronous walk with no scheduling. v1 will pass a
`ShardRef` to the indexer so per-peer budgets land on the indexing
work (the librarian's tick budget per
`spectral-db-as-autopoietic-memory.md` §6.4). Forward-promised in
§9.3.

#### 3.2.3 `append_note` / `read_notes` — the out-of-band metadata channel

`fragmentation/vcs/git/src/notes.rs:48`.

The substrate role: append-only git-notes under
`refs/spectral/notes/<topic>` — out-of-band metadata that survives
git operations without polluting trees. The substrate's natural
home for "things observed about the indexed surface but not part of
the surface itself."

`mirror init` admits an OPTIONAL composition here: after indexing,
append a note under `refs/spectral/notes/mirror-init/<run-oid>`
recording the run's envelope (the same JSON emitted to stdout per
§4.7). The note becomes the substrate's persistent record of the
init operation — the librarian can read it later without re-walking
the repo. v0 ships WITHOUT this composition (the envelope-to-stdout
suffices); v1.5 lands the note per §9.4.

### 3.3 What `mirror init` does NOT compose

Per Taut's §3.7 grep-verified "what is NOT present" list:

- **No `walk_repo` / `ingest_repo` primitive in fragmentation
  today.** `mirror init` synthesizes one (path (a) above) OR drives
  `project::project` from a synthesized manifest (path (b) above).
  v0 recommendation: (b).
- **No git-hook installer in fragmentation today.** `mirror init
  --install-hooks` writes hooks via mirror-side `std::fs::write`
  with a shell template (§6).
- **No `mirror init`-shaped primitive at fragmentation altitude.**
  Correct per Taut §3.7: init is mirror's altitude, not
  fragmentation's. This spec lives at mirror's altitude and
  composes fragmentation's primitives at the right altitude.

The "what's NOT present" list is structurally important. It tells us
the mirror-side glue is REAL work (~200 LOC); it's not free; but it's
bounded. The substrate-pull-honest path is composition, not
construction — but composition with the right ~200 LOC of mirror
glue. §4 enumerates that glue.

---

## §4 — The init operation flow

The substrate operation. Per Taut's §5 inventory + the §3 primitives
above, six steps. Each step names the carriers; the steps compose
into one envelope-yielding operation; the envelope is the v0
surface.

### 4.1 Step 1 — Cargo edge

The missing edge. `mirror/bootstrap/Cargo.toml` gains:

```toml
[dependencies]
fragmentation = {
    path = "../../fragmentation",
    features = ["concurrent", "singularity", "project", "supervision"],
    default-features = false,
}
fragmentation-git = {
    path = "../../fragmentation/vcs/git",
    default-features = false,
}
```

Two crates: `fragmentation` (the core; `concurrent` feature for
`ConcurrentStore`'s DashMap backing; `singularity` for
`WitnessedSingularity`; `project` for `project::project`;
`supervision` reserved for §7's spawn↔recall↔init composition);
`fragmentation-git` (the git wire layer; the
`NamespacedGitStore` lives here per Taut §3.2). `fragmentation-git`
transitively pulls `git2 = "0.19"` + `dashmap = "6"` — the binary-size
posture risk Taut flagged as R2 (addressed in §8.2 below).

The edge is one diff. The diff IS the Phase A unblock for everything
downstream. Per `mirror-store.md` §4.6: "the minimum to unblock F-2
is Cuts 1 and 2; both can land in a single fragmentation commit." —
both cuts have landed (the `concurrent` feature exists today per
`fragmentation/Cargo.toml`'s `features` block; the
`Fragmentable`/`ContentAddressed`/`TreeShaped` trait split also
exists today). Mirror's side of the F-2 work is THIS Cargo edge.

The Cargo edge is **NOT a runtime composition**; it's a build-graph
composition. The fragmentation crate becomes available to mirror's
bootstrap; subsequent steps (§4.2–§4.7) compose the runtime
primitives.

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
ProjectedFile { content, oid }> }`. Each `ProjectedFile` is a
`Vec<u8>` + a blob OID. The set is **deterministic** given the same
git state (because `git ls-files` is deterministic; because file
contents are deterministic; because `blob_oid_bytes` is the canonical
git-shape blob hash).

Determinism matters for §4.5 idempotency.

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

- **The `encode_splinter_as_fractal` adapter is mirror-side glue.**
  fragmentation's `Fractal<String>` is the store's value type; mirror's
  `Splinter<H>` is the crystal value type. The bridge is one
  encode-decode pair; lives in mirror's `bootstrap/src/crystallize.rs`
  or in a sibling module. ~20–30 LOC. Out of v0 scope for THIS spec to
  pin the API; the bridge is forward-promised under the bilateral
  pattern (declaration in `shards/`, fracture body in mirror Rust).
- **The `Uncrystallized` verdict is expected at Tick A.** R1's
  adversarial concern (§8.1): the `Crystallizations` floor is empty
  per `floor_crystallizations` at line 510 of `crystallize.rs`; every
  splinter lands `Uncrystallized`. This is NOT a blocker for `mirror
  init` — the crystals are stored regardless; downstream consumers
  (the librarian; `mirror recall`) can re-dispatch when the floor
  populates.

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

**(A) `.git/mirror/`** — `NamespacedGitStore::open(repo_path,
"mirror")`'s default. Per Taut §3.2: the store lives inside `.git/`
so it doesn't pollute the working tree; it uses the `.frgmnt` file
format (fan-out objects + refs) rather than git's own ODB; it
travels with `git` operations on `.git/` and survives clone IF the
refs are pushed.

**(B) `.spectral/db/<peer-oid>/`** — the dotted-directory pattern
mentioned in `CLAUDE.md` (project instructions) for spectral session
state: `gestalt/`, `sessions/`, `crystals/`, `HEAD`, `log`. A
peer-scoped subtree per the substrate-decl pattern emerging in
peer-ACL §10.

**(C) `~/.mirror/`** — `mirror-store.md` §10.5's user-scoped canonical
location: a bare fragmentation repo per `frgmnt_store.rs`,
content-addressed, per-user (not per-project), independent of `$PWD`.

### 5.2 The substrate-pull-honest call

**(A) `.git/mirror/` is the v0 location.** Three reasons stack:

**Reason 1 — fragmentation already declares the path.**
`NamespacedGitStore::open(repo_path, "mirror")` constructs
`.git/mirror/{objects,refs}/`. The substrate-decl side picked the
location; not picking it would be inventing a new shape. Per
`[[feedback-substrate-already-had-the-word]]`: when the substrate
already declares a location, don't invent a competitor.

**Reason 2 — `.git/<namespace>/` carries the git-clone semantics for
free.** A crystal indexed at `.git/mirror/objects/<2hex>/<rest>`
travels with `git clone` (no special-case code) so long as the
`refs/mirror/<name>` refs are pushed. The substrate's
spawn↔recall↔init triple (§7) consumes this: when a peer's home
repo is cloned, its indexed crystals come along; the consumer can
read `store.get_ref("HEAD")` to get the indexed surface immediately.
`.spectral/db/` and `~/.mirror/` would each need a separate transport
mechanism; `.git/mirror/` doesn't.

**Reason 3 — the `repo IS a store` isomorphism per Alex 2026-06-17.**
`spectral-db-as-autopoietic-memory.md`'s correction sequence: each
repo IS a store; the substrate has one boundary unit, named two ways
for two roles. `.git/mirror/` makes this isomorphism architecturally
visible: the store lives **inside** the repo's `.git/`; opening the
store IS opening the repo's storage role.

### 5.3 What `.spectral/db/` and `~/.mirror/` ARE (so the call doesn't
collide)

Both candidates have legitimate roles **at different altitudes**;
naming them avoids future collision:

**`.spectral/db/<peer-oid>/`** is the **session-state** location, not
the crystal-store location. Per `CLAUDE.md` (project instructions):
"Like `.git/` but for graphs. `gestalt/` crystals (user understanding
state), `sessions/` session data, `HEAD` current session timestamp,
`log` tick log." The session-state altitude is operational
intermediate data (the user's understanding-in-progress), not the
content-addressed crystal accumulation. `.git/mirror/` is the
storage altitude; `.spectral/db/` is the session-state altitude.
Both can coexist in the same repo.

**`~/.mirror/`** is the **per-user crystal-pool** location, per
`mirror-store.md` §10.5. A bare fragmentation repo at the user's
home directory; content-addressed; shared across all of the user's
project checkouts. Same OID → same blob → one copy on disk. The
relationship to `.git/mirror/`:

- `.git/mirror/` is **per-repo**. Crystals for files in THIS repo.
- `~/.mirror/` is **per-user**. Crystals shared across all repos.

The two compose: `mirror init` writes to `.git/mirror/` (per-repo);
a separate `mirror sync` (forward-promised; not in this spec's scope)
mirrors crystals from `.git/mirror/` to `~/.mirror/` so they survive
clean checkouts. The librarian per
`spectral-db-as-autopoietic-memory.md` lives at `~/.mirror/`'s root
supervisor altitude; it reads `~/.mirror/` AND it consolidates from
the per-repo `.git/mirror/` stores it supervises.

The architectural picture (forward-promised; not the v0 surface):

```
~/.mirror/                              ← librarian's catalog
  └── (per-user crystal pool)
       ↑ (sync; forward-promised)
.git/mirror/                            ← THIS spec's v0 location
  ├── objects/<2hex>/<rest>             ← content-addressed crystals
  └── refs/
       └── HEAD                         ← root OID of last init

.spectral/db/<peer-oid>/                ← session-state (separate altitude)
  ├── gestalt/
  ├── sessions/
  └── HEAD
```

### 5.4 The collision-avoidance pin

To ensure future ticks don't re-conflate the three locations, the
v0 surface declares:

- **`mirror init` writes ONLY to `.git/mirror/`.** Not
  `.spectral/db/`. Not `~/.mirror/`. The other two locations are
  out of v0 scope.
- **`mirror init` is idempotent against `.git/mirror/`** (§4.5). No
  observable change on re-run against the same git state.
- **`mirror init` does NOT touch `.spectral/db/`** (left for the
  session-state command surface, currently un-spec'd at this
  altitude).
- **`mirror init` does NOT sync to `~/.mirror/`** (left for `mirror
  sync`, forward-promised).

The three location boundaries hold per altitude; the v0 surface
honors the first altitude only.

### 5.5 The selection between path (a) and path (b) revisited

Per §3.1.5, the file-set comes from either:

- (a) a new `walk_repo` primitive in fragmentation (~30 LOC), OR
- (b) a `git ls-files`-driven manifest synthesized in mirror.

The substrate-pull-honest call for v0: **path (b)**. Three reasons:

1. **No fragmentation-side commit required.** v0 ships with one
   diff (the Cargo edge §4.1) + ~200 LOC of mirror glue. Path (a)
   adds a fragmentation commit; the v0 timeline benefits from
   keeping changes mirror-side.
2. **`git ls-files` IS what the user expects.** Files tracked by
   git ARE the indexable surface; untracked files (build artifacts,
   `target/`, editor state) are correctly excluded by default.
   Path (a)'s `Tree::walk` over the head tree achieves the same
   set; the substrate doesn't gain by re-implementing it.
3. **Path (a) lands later for the historical walk.** When `mirror
   init --history` (v1; §9.2) wants commit-by-commit indexing,
   `walk_commits_following` is already there; path (a)'s `walk_repo`
   adds value at that altitude. Deferring path (a) doesn't lose
   future capability.

The path (b) decision is REVERSIBLE: a later tick that adds
`walk_repo` to fragmentation can swap mirror's manifest-synthesis
for a direct `walk_repo` call; the envelope shape (§4.7) doesn't
change; downstream consumers are unaffected.

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

The hooks invoke `mirror reindex`, NOT `mirror init`. The
distinction:

- **`mirror init`** initializes the store (creates the namespace,
  walks the full file set, writes the root OID, sets up the
  `.git/mirror/` shape). One-shot or idempotent rerun.
- **`mirror reindex`** updates the store incrementally (walks
  ONLY the delta — staged changes, or commit-range diffs — and
  updates the existing crystals). Per-commit or per-edit cadence.

`mirror reindex` is forward-promised; not in this spec's scope.
Per `[[feedback-substrate-already-had-the-word]]`: the delta
mechanism already exists at fragmentation altitude
(`walk_commits_following` for commit ranges; `git diff --cached
--name-only` for staged changes); `mirror reindex` composes them
the same way `mirror init` composes the full-walk primitives.

### 6.3 Hook semantics: what re-indexing means

A re-index event is the moment the substrate's view of the repo
catches up with the working tree's view. Per §4.5 idempotency, this
is **structurally equivalent to running `init` and discarding the
unchanged crystals** — but operationally cheaper, because the delta
machinery avoids re-hashing unchanged files.

The substrate's semantic claim: **the crystals at `.git/mirror/HEAD`
represent the indexed projection of the repo as of the last
re-index event**. The librarian (when it lives) reads
`store.get_ref("HEAD")`; the read is correct IFF the last re-index
event was post-current-state. Stale crystals are visible to the
librarian as "the indexed surface at root OID X" — the librarian
itself does not know whether the working tree has drifted; that's
the inhabitant's housekeeping.

This matches `[[architecture-jurisdiction-sets-gates-inhabitant-
chooses-housekeeping]]`: the jurisdiction (the librarian's altitude)
sets the GATE (you must give me a root OID; I trust the root OID's
addressed bytes); the inhabitant (the repo) chooses the
HOUSEKEEPING (hooks for auto-reindex; or manual `mirror reindex`;
or accept stale crystals).

### 6.4 The push semantics (forward-promised)

The brief named: ".git/mirror/" travels with clones IF refs are
pushed. v0 ships WITHOUT pushing `refs/mirror/*` by default; the
inhabitant adds a `refspec` to `.git/config` if they want their
indexed crystals to travel:

```ini
[remote "origin"]
    fetch = +refs/heads/*:refs/remotes/origin/*
    fetch = +refs/mirror/*:refs/remotes/origin/mirror/*
    push = refs/heads/*:refs/heads/*
    push = refs/mirror/*:refs/mirror/*
```

This is OPT-IN. The substrate's posture (per §6.3): the inhabitant
chooses whether their crystals are visible to clones. The
`--install-hooks` flag does NOT modify `.git/config`; that's
a separate `mirror init --enable-push-refs` flag (forward-promised;
not in v0 scope).

The architectural reason for opt-in: the indexed crystals carry
content fingerprints; pushing them is a transparency act; not every
inhabitant wants every clone to know every file's BLAKE3. The
choice belongs to the inhabitant per the consent geometry per
`[[architecture-geometric-consent-projection]]`.

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
