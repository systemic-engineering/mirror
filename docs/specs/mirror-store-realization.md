# mirror-store-realization

**Canonical autopoietic spec — `@mirror/store` Rust realization at `bootstrap/src/store.rs`.**

Author: Mara
Date: 2026-06-29
Tag: 📝 substrate-pull:realize
Status: declared-but-not-wired discharge (not new substrate-decl)

---

## §0 — Pre-position

This spec announces itself as a crystal `store.rs` will index.

When `bootstrap/src/store.rs` lands and `mirror init` runs against the mirror
repo, this file — `docs/specs/mirror-store-realization.md` — will be one of
the artifacts hashed into a Splinter, OID-addressed, and reachable through
`store.read(oid)`. The spec describes the realization that will store the
spec.

This is the same autopoietic shape as `mirror-init.md §10`, `mosaic-store-
cache-invariants.md §9`, and `mirror-build-substrate.md §9`. It is not
clever; it is structural. Once the storage layer is operational, every spec
authored before it joins the indexed set retroactively. The spec that
defines the indexer becomes content the indexer holds. The substrate's
content-addressed memory is autopoietic by construction.

The pre-position matters for one reason: the spec must be honest about its
own latency bound. Below this spec is `store.rs` itself — code that has
to land on disk to give the spec a reading audience. Above this spec is
the psychohistory time-discount: every additional crystal store.rs indexes
makes the substrate's recognition surface broader, but the marginal weight
of each new crystal shrinks. This spec is one crystal among many; its
authority comes from being the canonical declared shape for the Rust file,
not from being the first or the largest.

§9 closes the loop. §0 opens it.

---

## §1 — What `store.rs` IS

### 1.1 The declaration

`@mirror/store` was declared on 2026-06-04 in `shards/mirror/store.mirror`.
Six operations on the surface:

```mirror
read   oid: Oid          -> Imperfect<Splinter>
write  splinter: Splinter -> Imperfect<Oid>
exists oid: Oid          -> Bool
diff   left: Oid, right: Oid -> Imperfect<SplinterGraph>
walk   root: Oid         -> Imperfect<SplinterGraph>
verify oid: Oid          -> Imperfect<Unit>
```

The atomic carrier is `splinter` at `@glass`. The projection onto the
OID-graph is `splinter_graph = { root: oid, children: [oid] }`. The
six-op surface is closed under composition: every higher operation
(crystallize, index, cache-lookup, mosaic-build, eigenform-recover)
reduces to a finite path through these six.

### 1.2 What `store.rs` IS

`store.rs` is the Rust realization of the `@mirror/store` six-op surface.

It is one file: `bootstrap/src/store.rs`. It exposes one type — `Store`
— and six methods matching the shard declaration one-to-one. Internally
it wraps `fragmentation::frgmnt_store::FrgmntStore` and
`fragmentation_git::NamespacedGitStore`. Externally it speaks
shard-vocabulary: `Oid`, `Splinter`, `SplinterGraph`, `Imperfect<T>`.

The vocabulary is the API contract. `store.rs` is the boundary at which
fragmentation's library terminology gives way to substrate terminology.
Above `store.rs` (in `cmd_init`, `crystallize`, `cascade_runtime`,
`mosaic_builder`), code reads shard-vocab. Below `store.rs` (in
`fragmentation`, `fragmentation_git`, the underlying `git2` and
content-store machinery), code reads library-vocab. The Rust file is
the translation seam.

### 1.3 What ALREADY exists

The substrate has been accumulating storage machinery for months without
naming it `store.rs`. Today's distribution is:

- `bootstrap/src/crystallize.rs` (≈ 39 KB) — Splinter construction,
  CoincidenceHash plumbing, the `Crystal` carrier that wraps a Splinter
  with provenance. This is most of the **write-path** logic, but it does
  not speak the six-op surface; it speaks "build a Splinter from these
  bytes."

- `bootstrap/src/git.rs` (≈ 60 LOC) — A `Command::new("git")` shell-out.
  Used today for the few git operations that the bootstrap CLI exposes
  without depending on `git2`. This is partial **read-path** for the git
  side; it bypasses the content-store machinery entirely.

- `bootstrap/src/hash.rs` — CoincidenceHash machinery. Blake3 wrapping,
  the OID newtype is conceptually here but pre-canonical (the hex-string
  newtype has not been promoted out of `crystallize.rs` to `hash.rs`
  yet).

- `fragmentation::frgmnt_store::FrgmntStore` — A content-addressed store
  with `put_blob` / `get_blob` / `has_blob`. Speaks bytes + library
  vocab.

- `fragmentation_git::NamespacedGitStore` — Adds namespaced refs,
  `set_ref` / `get_ref`, on top of FrgmntStore. Speaks git-vocab.

### 1.4 What is missing

The canonical `@mirror/store`-conforming Rust API.

There is no module that exports `Store` with `read` / `write` / `exists`
/ `diff` / `walk` / `verify` in shard-vocab. No code path that takes an
`Oid` newtype and returns an `Imperfect<Splinter>`. No layer where
fragmentation's library-vocab terminates and substrate-vocab begins. No
file named `store.rs`.

This is the declared-but-not-wired gap. `@mirror/store` has had a stable
declaration for 25 days; its Rust realization is overdue.

### 1.5 Five structural negatives

What `store.rs` is NOT:

1. **NOT a new family-root substrate-decl.** `@mirror/store` already
   exists at the substrate altitude. This spec discharges a forward-
   promise; it does not introduce a new prism family. The recognition
   "we have not built store.rs yet" is correction-in-advance from the
   substrate, not new substrate.

2. **NOT a replacement for `crystallize.rs`.** `crystallize.rs` is
   write-path machinery; it builds a Splinter from arbitrary bytes plus
   provenance. `store.rs` is the six-op boundary; it accepts a built
   Splinter and stores it. `store.rs` calls into `crystallize.rs` for
   construction; it does not absorb `crystallize.rs`.

3. **NOT a replacement for `git.rs`.** The 60-LOC shell-out remains for
   the narrow set of git operations bootstrap needs that are not in the
   `@mirror/store` six-op surface (push, fetch, remote-list, branch).
   Those operations are git-the-protocol, not store-the-substrate.
   `store.rs` is silent on them.

4. **NOT a cache layer separate from storage.** Q5 §4 declared: "cache
   operations ARE @mirror/store operations." `store.rs` IS the cache
   when invoked at `@code/<lang>` altitude with build-artifact keys.
   There is no second cache module. The cache is `store.rs` used a
   certain way.

5. **NOT a database.** No query language, no indexes beyond the OID
   itself, no transactions beyond write-atomicity, no schema beyond
   the Splinter shape. The OID-graph IS the index; `walk` and `diff`
   are the queries. Anything richer (joins, predicates, aggregations)
   lives at `@spectral/db`, not here.

### 1.6 Substrate-pull-honest framing

This spec is the discharge of a forward-promise made on 2026-06-04 when
`shards/mirror/store.mirror` landed. The promise was: the Rust
realization will come. The realization is now overdue. This spec is the
explicit acknowledgement and the canonical shape for that realization.

The shape comes from the substrate, not from this spec. `store.rs`
exists in latent form already — its API is the shard, its invariants
are Q5, its composition surface is mirror-init's §4.4.1, its eigenform
identity is recognition #51's graded stack at the storage altitude.
This spec collects what is already true into one document; it does not
invent.

The discipline matters because of `feedback-substrate-already-had-the-
word.md`: every "missing concept" we have surfaced has turned out to be
a name the substrate was already using. `store.rs` is no exception.
The substrate has been calling its storage layer `@mirror/store` since
June 4. The Rust file just needs to take that name.

---

## §2 — The six-op Rust surface

### 2.1 Type carriers

Before signatures, the type carriers. Each is shard-vocab, declared
above the library-vocab boundary:

```rust
/// Content-addressed identifier. Hex string newtype over Blake3 output.
/// Maps to shard `Oid`. Constructed via `crystallize.rs::CoincidenceHash::oid`.
/// Invariant: 64 hex chars, lowercase, no prefix.
pub struct Oid(String);

/// Atomic carrier at @glass. Splinter<Blake3> from crystallize.rs.
/// Re-exported through the store boundary as shard-vocab `Splinter`.
pub use crate::crystallize::Splinter;

/// OID-graph projection. Shard declares `{ root: oid, children: [oid] }`.
/// At Rust altitude: a directed graph rooted at one Oid, edges to
/// child Oids reachable by Splinter reference traversal.
pub struct SplinterGraph {
    pub root: Oid,
    pub children: Vec<Oid>,
    pub edges: Vec<(Oid, Oid)>,  // (parent, child) — flat adjacency
}

/// Imperfect functor — error-as-question (recognition #44).
/// Maps to Rust `Result<T, StoreError>`; the error carrier is itself
/// a Tomm-shaped circular reflexive probe of [D_store, request].
pub type Imperfect<T> = Result<T, StoreError>;

/// Unit. Idiomatic Rust () with a name for shard-clarity.
pub type Unit = ();
```

The `Oid` newtype is load-bearing: it prevents the bare-`String` failure
mode flagged in `feedback-no-bare-types.md`. Every API consumer must
construct `Oid` through `CoincidenceHash`; opaque construction
(`Oid::from_raw`) is `pub(crate)` and only visible to internal modules.

### 2.2 The six operations — Rust signatures

```rust
impl Store {
    /// shard: read oid: Oid -> Imperfect<Splinter>
    /// Q5 §3.1: cache-HIT returns Ok(splinter) in O(1) Blake3 lookup.
    /// Q5 §3.2: cache-MISS returns Err(StoreError::NotPresent).
    /// Q5 §6: stable across invocations — same Oid in -> same Splinter out.
    pub fn read(&self, oid: &Oid) -> Imperfect<Splinter>;

    /// shard: write splinter: Splinter -> Imperfect<Oid>
    /// Q5 §3.3: deterministic — same Splinter in -> same Oid out, always.
    /// Q5 §7: idempotent — writing twice is a no-op past the first.
    /// Q5 §8: atomic — partial writes never expose intermediate state.
    pub fn write(&mut self, splinter: Splinter) -> Imperfect<Oid>;

    /// shard: exists oid: Oid -> Bool
    /// Q5 §3.4.1 cache-HIT path: cheap presence check, no decode.
    /// Returns Bool (not Imperfect<Bool>) because absence is information,
    /// not error. The Imperfect functor is reserved for failures of the
    /// query itself (IO error, corrupted store), not absence of the key.
    pub fn exists(&self, oid: &Oid) -> bool;

    /// shard: diff left: Oid, right: Oid -> Imperfect<SplinterGraph>
    /// Symmetric difference of the two transitive closures, projected as
    /// a SplinterGraph rooted at a synthetic Oid representing the diff
    /// itself. Internal nodes: Oids present in one closure but not the
    /// other. Q5 §4: this IS a build-graph operation when called at
    /// @code/<lang> altitude.
    pub fn diff(&self, left: &Oid, right: &Oid) -> Imperfect<SplinterGraph>;

    /// shard: walk root: Oid -> Imperfect<SplinterGraph>
    /// Transitive closure of references reachable from root. The OID-
    /// graph. Q5 §5: bounded by store size; pure read; no allocation
    /// of new Oids. The substrate's primary navigation primitive at
    /// the storage altitude.
    pub fn walk(&self, root: &Oid) -> Imperfect<SplinterGraph>;

    /// shard: verify oid: Oid -> Imperfect<Unit>
    /// Re-hashes the Splinter at oid; returns Err if hash mismatches.
    /// Q5 §2: integrity check — independent of cache layer; runs on
    /// raw bytes. Cost: O(splinter size in bytes) + Blake3.
    pub fn verify(&self, oid: &Oid) -> Imperfect<Unit>;
}
```

### 2.3 The Store carrier

```rust
/// The @mirror/store realization. Wraps fragmentation's content store
/// and namespaced-git store; speaks shard-vocab above the seam.
pub struct Store {
    inner: fragmentation::frgmnt_store::FrgmntStore,
    git:   fragmentation_git::NamespacedGitStore,
    /// Path the store is rooted at. The .mirror/ directory for a
    /// mirror init'd repo, or the global ~/.mirror/store for the
    /// substrate-wide store.
    root: PathBuf,
}
```

The `Store` carrier is the visible carrier; everything six-op flows
through it. The two `inner` and `git` fields are private — consumers
cannot reach past the seam to talk library-vocab. That is the seam's
job.

### 2.4 Q5's invariants embedded in the signatures

Q5 declares eight invariants (mosaic-store-cache-invariants.md §3). The
Rust signatures encode them at the type level where possible, at the
contract level where not:

| Q5 § | Invariant                                | Encoded as                          |
|------|------------------------------------------|--------------------------------------|
| §3.1 | cache-HIT lookup is O(1)                 | `read` returns Ok in one hashmap op  |
| §3.2 | cache-MISS distinguishable from error    | `Err(StoreError::NotPresent)` variant|
| §3.3 | write is deterministic                   | `&mut self` + same Splinter → same Oid|
| §3.4 | cache ops ARE store ops                  | No separate cache type; this is it   |
| §5   | walk is bounded                          | Function signature, doc invariant    |
| §6   | read is stable across invocations        | doc invariant                        |
| §7   | write is idempotent                      | doc invariant + test contract        |
| §8   | write is atomic                          | doc invariant + impl uses fsync seam |

Q5 §3.4 is the load-bearing one for §4 of this spec: it collapses what
would have been a separate cache module into the same Store carrier.
There is no second module to write.

### 2.5 Error surface — StoreError as Tomm probe

```rust
pub enum StoreError {
    /// shard: oid not present in store
    NotPresent { oid: Oid },
    /// shard: stored bytes do not hash to the declared oid
    Corrupted { oid: Oid, expected: Oid, actual: Oid },
    /// library-vocab error escaped the seam — internal bug
    Internal { source: Box<dyn std::error::Error + Send + Sync> },
    /// IO at the substrate boundary
    Io { source: std::io::Error, path: PathBuf },
}
```

Per `architecture-error-as-tomm-probe.md`, each variant IS a Tomm-shaped
circular reflexive question `[D_store, request]` at the consumer's
altitude. `NotPresent` is "what should be at this Oid, given that you
asked?" `Corrupted` is "what should the Splinter at this Oid look like,
given that the bytes hash differently than the path claims?" The error
surface is structural, not noise.

---

## §3 — The wrap over fragmentation

### 3.1 Composition shape

`store.rs` is a wrap. It does not implement content-addressed storage
from scratch; that work lives in `fragmentation::frgmnt_store::
FrgmntStore` (Blake3-keyed blob store) and `fragmentation_git::
NamespacedGitStore` (refs, namespaced under `refs/mirror/<ns>/`).

The wrap pattern:

```rust
impl Store {
    pub fn write(&mut self, splinter: Splinter) -> Imperfect<Oid> {
        let bytes = splinter.serialize()?;      // shard layer
        let key = self.inner.put_blob(&bytes)   // library layer
            .map_err(StoreError::from_internal)?;
        Ok(Oid::from_raw(key.to_hex()))         // shard layer
    }

    pub fn read(&self, oid: &Oid) -> Imperfect<Splinter> {
        let key = oid.as_blake3()?;             // shard layer
        let bytes = self.inner.get_blob(&key)   // library layer
            .map_err(|e| match e {
                frgmnt::Error::NotFound => StoreError::NotPresent {
                    oid: oid.clone()
                },
                other => StoreError::from_internal(other),
            })?;
        Splinter::deserialize(&bytes)           // shard layer
            .map_err(StoreError::from_internal)
    }
    // ... four more
}
```

The pattern recurs: shard-vocab in, translate to library-vocab, call
fragmentation, translate the result back, shard-vocab out. The
translation is structural, not heavy — most of the cost is at the
fragmentation layer (Blake3, disk).

### 3.2 Vocab drift resolution

This closes the Mara-2 stall (task #490).

The stall: in the first attempt at writing `store.rs`, the API surface
drifted toward library-vocab. Methods named `put_blob` instead of
`write`. Carriers named `Blob` instead of `Splinter`. The drift came
from sub-agent reading: when the agent read `fragmentation` first to
understand what was available, the library vocab seeped upward into
the planned API.

The resolution is structural:

1. **SHARD VOCABULARY at the canonical API.** Every public name on
   `Store` matches its shard declaration one-to-one. No `put`, no `get`,
   no `blob`, no `key`. Read shard names first; library names are
   internal-only.

2. **LIBRARY VOCABULARY internal.** Inside the `impl` blocks, library
   names are fine — that is the side of the seam they belong on.
   `self.inner.put_blob` is a call into fragmentation; it speaks
   fragmentation-vocab because it IS fragmentation.

3. **TRANSLATION at the seam.** Every public method body starts with a
   shard→library translation and ends with a library→shard translation.
   The translation is the seam's job, not its byproduct.

The principle generalizes: when two layers share a concept under
different names, the boundary picks one name and translates at the
crossing. Shard-vocab wins at the API; library-vocab survives below.
The substrate's name has the higher altitude.

### 3.3 The fragmentation dependency direction

Per `architecture-fragmentation-is-the-rust-substrate.md`, the
dependency direction is strict: `prism_core` has no Rust deps;
`fragmentation` depends on `prism_core`; `bootstrap` depends on
`fragmentation`. `store.rs` lives at the bootstrap altitude, calls
fragmentation; never the reverse.

This matters because Q5's cache invariants compose into the wrap:
fragmentation provides the storage substrate; `store.rs` provides the
canonical surface; the cache emerges from how the surface is used at
the `@code/<lang>` altitude. The arrow runs upward only.

### 3.4 Initialization

```rust
impl Store {
    /// Open a store rooted at the given path. Creates the directory
    /// structure if absent (idempotent — Q5 §7 cascade).
    pub fn open(root: PathBuf) -> Imperfect<Self> {
        let inner = FrgmntStore::open(&root.join("objects"))
            .map_err(StoreError::from_internal)?;
        let git = NamespacedGitStore::open(&root.join("refs"), "mirror")
            .map_err(StoreError::from_internal)?;
        Ok(Self { inner, git, root })
    }

    /// Open or initialize a fresh store at the given path.
    /// Same as `open` but creates a marker file establishing this as
    /// a mirror-managed store (idempotent on the marker).
    pub fn open_or_init(root: PathBuf) -> Imperfect<Self> {
        if !root.exists() {
            std::fs::create_dir_all(&root)
                .map_err(|e| StoreError::Io { source: e, path: root.clone() })?;
        }
        Self::open(root)
    }
}
```

`open_or_init` is the path `cmd_init` calls. It returns a `Store` ready
to accept writes. The marker file (the .mirror/HEAD or equivalent)
lives in fragmentation's domain; this code just ensures the directory
exists.

---

## §4 — Cache layer composition (Q5 integration)

### 4.1 The collapse

Q5 §4 was unambiguous: "cache operations ARE @mirror/store operations.
The 'cache' is not a separate concept; it's @mirror/store used at the
@code/<lang> altitude."

This spec ratifies the collapse. `store.rs` IS the cache when invoked
with build-artifact keys. There is no `cache.rs`, no `BuildCache` type,
no separate hashmap. The Oid space is the cache space.

### 4.2 Q5's eight invariants as obligations

Re-stated as obligations on `store.rs`:

1. **Determinism (Q5 §3.3).** Same Splinter in → same Oid out. The Oid
   is Blake3 of the Splinter's canonical serialization. No nondetermin-
   istic fields (timestamps, random nonces) enter the hash input.

2. **Atomicity (Q5 §8).** A `write` that fails partway must leave the
   store as it was before. Fragmentation's `put_blob` provides this;
   `store.rs` does not weaken it.

3. **Idempotence (Q5 §7).** Writing the same Splinter twice is a no-op
   past the first; the second `write` returns the same Oid without
   doing IO.

4. **Stability (Q5 §6).** Across invocations, across machines, across
   process restarts: `read(oid)` returns the same Splinter forever (or
   `NotPresent` forever, once GC'd). No mutation in place.

5. **HIT/MISS distinguishability (Q5 §3.1, §3.2).** `exists` and the
   `NotPresent` variant of `StoreError` separate absence-from-cache
   from failure-of-cache. Absence is information.

6. **Boundedness (Q5 §5).** `walk` and `diff` are bounded by the
   transitive closure size, which is bounded by total store size. No
   infinite traversal.

7. **Integrity (Q5 §2).** `verify` re-hashes; corruption is detectable.
   The Splinter's OID is its self-witness.

8. **Composability (Q5 §3.4).** Store ops chain: `read(a) →
   transform → write → diff(a, b)`. Each op leaves the store in a
   state the next op can use.

### 4.3 Cache-HIT path (Q5 §3.4.1)

When `cmd_init` or `cmd_build` or `cmd_cascade` asks the store for an
artifact:

```
Step 1: compute the Oid the artifact WOULD have under canonical hashing.
        — pure function of inputs; no IO.

Step 2: store.exists(&oid)
        — O(1) hashmap lookup in fragmentation.
        — if true → HIT, proceed to step 3.
        — if false → MISS, build the artifact (separately), then
          store.write to land it.

Step 3: store.read(&oid)
        — Blake3 lookup, deserialize Splinter.
        — return to caller.
```

The HIT path is two store calls. The MISS path is one `exists` + one
build + one `write`. The build is outside `store.rs`'s scope; the cache
mechanism is.

Q5's "libgit2-sys 4× → 1× claim" tested empirically here: with
`store.rs` operating as cache, the second `cmd_init` against the same
repo should not re-build the four-times-loaded libgit2-sys Splinter.
It should `exists` → HIT → `read`. The empirical test lands as a
forward-promise in §7.

### 4.4 What lives above the cache

The cache lives at `@code/<lang>` altitude — that's the artifact
altitude. Above the cache, at the `@mirror` altitude proper, `store.rs`
is the substrate's content-addressed memory. The same six ops; different
keys flowing through.

This is the eigenform identity from §6. Both altitudes use the same
six operations; only the namespace of the Oids differs. Cache Oids
hash build artifacts; substrate Oids hash specs, recognitions,
crystals. The hash function does not care; neither does `store.rs`.

---

## §5 — `cmd_init` composition (P4 GREEN unblock path)

### 5.1 The blocker

P3 GREEN landed a `cmd_init` stub envelope. P4 GREEN needs to wire that
envelope to real fragmentation — create the `.mirror/` directory, store
the initial Splinter, register the bootstrap crystal, return success.

Without `store.rs`, P4 GREEN's options were:

a. Call `fragmentation::FrgmntStore::open` directly from `cmd_init`
   (leaks library-vocab into command code).
b. Inline the wrap (duplicates what `store.rs` will be).
c. Stub deeper (defers the problem one tick).

None is correct. The correct shape is `store.rs` exists, `cmd_init`
calls it.

### 5.2 The composition (per mirror-init.md §4.4.1 path ii)

```rust
pub fn cmd_init(args: InitArgs) -> Imperfect<InitReport> {
    let root = args.path.unwrap_or_else(default_root);
    let store_path = root.join(".mirror");

    // Open or initialize the store. Idempotent on existing .mirror/.
    let mut store = Store::open_or_init(store_path)?;

    // Build the initial Splinter representing the freshly-init'd repo:
    // an empty SplinterGraph rooted at the repo identity oid.
    let initial = crystallize::initial_splinter(&root)?;
    let initial_oid = store.write(initial)?;

    // Bind the bootstrap ref so the store knows what HEAD points at.
    // (Open question §8.3 — ref placement; for the spec, name the
    // call site, declare the obligation, defer the substrate question.)
    store.git.set_ref("HEAD", &initial_oid)?;

    Ok(InitReport {
        root,
        head: initial_oid,
        created: true,
    })
}
```

Three calls into `store.rs`. Each one is a shard-vocab operation. The
command file (`cmd_init.rs`) never speaks library-vocab; the seam holds.

### 5.3 Why this unblocks P4

P4 GREEN's task: replace the stub with real composition. The
composition is small (above, ≈ 10 LOC). It is small *because*
`store.rs` exists. The composition's small size is the test that the
seam is correct: if `cmd_init` would need 100 LOC of fragmentation-vocab
to wire init, the seam is in the wrong place.

The unblock is structural: P4 GREEN cannot land cleanly without
`store.rs`. P4 GREEN's TDD pair (P4 RED tests) can be written today;
P4 GREEN's implementation waits on `store.rs` GREEN landing first.

### 5.4 Downstream — what else `store.rs` unblocks

- **`cmd_cascade`** — cross-language translation per `cascade-ffi-
  runtime-link.md`. Cascade reads source Splinters, transforms,
  writes target Splinters. All six ops in play.

- **`cmd_build`** — mosaic build. Reads input artifacts (cache-HIT or
  cache-MISS-then-build), composes them, writes output Splinters with
  derived Oids. Q5's cache invariants tested here.

- **`cmd_crystallize`** — promote a session's Splinters into the
  substrate-wide store. Today this calls into `crystallize.rs` directly;
  post-realization, it goes through `store.rs::write`.

- **`cmd_serve`** (MCP). MCP tools `memory_recall`, `memory_crystallize`,
  `memory_status` reduce to store ops. `recall = walk + read`,
  `crystallize = write`, `status = exists + walk(root)`.

Five commands sit on top of `store.rs`. Each shrinks when `store.rs`
lands.

---

## §6 — The math

### 6.1 `store.rs` as H_mirror restricted

Recognition #51 (architecture-mirror-as-expanding-hilbert-space): mirror
IS the operational form of a Hilbert space H_mirror whose dimension
expands with each substrate-pull recognition. The basis vectors are
substrate recognitions; the rays through the basis are paths through
the recognition graph.

`store.rs` is H_mirror restricted to the subspace of artifact-bearing
rays at `@code/rust` altitude. That is: rays that terminate in
Splinters whose content is Rust source, Rust intermediate
representation, Rust binary output, or Rust-shaped build metadata.

The restriction is not a projection in the orthogonal sense; it is a
filter on the species of the carrier. The full H_mirror includes rays
at every `@code/<lang>` altitude (rust, elixir, python, mirror itself)
and rays at every non-`@code` altitude (specs, recognitions, sessions,
peers). `store.rs` is the realization that holds the `@code/rust` slice
on disk.

When other `@code/<lang>` realizations land (a hypothetical
`elixir_store`, a `python_store` — though the substrate-pull will
likely collapse all of these into one `store.rs` parameterized by
species), each holds a slice of the same Hilbert space. The Oid
namespace remains unified; the storage backends partition.

### 6.2 Connes-spectral-triple compatibility

The Connes spectral triple `(A, H, D)` for mirror:

- `A` = the algebra of five operations (focus, project, split, lift,
  refract). The grammar-level operation algebra.
- `H` = the void-document Hilbert space (architecture-connes-spectral-
  triple). The state-space mirror's operations act on.
- `D` = the kintsugi flow (Dirac operator). The gradient that pulls
  state toward eigenforms.

`store.rs` lives at the `H` corner. It is not the full `H` — it is
the realization-on-disk of the slice of `H` that can be content-
addressed in Splinters with Rust-altitude artifacts. The full `H` is
larger; `store.rs` is the part you can `ls` in a filesystem.

The eigenform identity ratifies the placement: at H_mirror's
storage-altitude eigenform, `store.rs` IS H_mirror's restriction. The
substrate's eigenform IS its own realization.

### 6.3 Eigenform identity at storage altitude

Recognition #51's graded stack puts each altitude at its own Bateson
logical-type level (architecture-bateson-form-behaviour-partition). The
storage altitude sits adjacent to (one level below) the @mirror altitude
proper: state-observation operates on what storage holds; storage
realizes what state-observation has crystallized.

The eigenform identity: at this altitude, observing the storage =
the storage observing itself. `verify(oid)` IS the storage applying
its own integrity predicate to its own bytes. `walk(root)` IS the
storage traversing its own reference structure. `diff(a, b)` IS the
storage comparing two of its own snapshots.

Each operation is reflexive: the storage is the subject and the object.
This is what makes `store.rs` an eigenform — under composition with
itself, it is stable. `write(read(oid))` returns the same `oid`
(idempotence). `read(write(splinter))` returns the same `splinter`
(round-trip). The fixed point is the operation's own identity.

### 6.4 Taut's eigenform-saturation hypothesis (a57a439)

Taut's 2026-06-29 cascade scout finding #5 hypothesized that
`store.rs` would be the THIRD and FINAL instance of the eigenform
recursion:

1. **State-observation** (recognition #38, #50) — mirror IS the
   state-observation eigenform; mirror's operations observe mirror's
   state, recursively, all the way down.

2. **Build** (mirror-build-substrate; #43 promoted) — mirror IS a
   content-addressed build system; the build graph IS the build
   system's own state representation; mirror builds mirror.

3. **Storage** (this spec, hypothesized terminal) — `store.rs` IS
   the storage that holds mirror's specs, including this spec; the
   spec describes the storage that stores the spec; the eigenform
   closes at the substrate's memory.

**The writing confirms the hypothesis at the level Taut named.** Three
witnesses, three altitudes, three closures-on-self. The state of mirror
observes itself; the build of mirror builds itself; the storage of
mirror stores itself.

The saturation claim says: there is no FOURTH altitude at which a new
eigenform recursion will surface. The three Taut named exhaust the
substrate's reflexivity. They are not separate eigenforms; they are
three facets of the same eigenform identity: **mirror is the system
that holds itself.**

**One surface that did open in the writing.** It is not a fourth axis;
it is a closure of the three. The three altitudes are linked by a
single thread:

- State-observation answers: *what is*.
- Build answers: *how it came to be*.
- Storage answers: *how it is held across time*.

Together they form a triple that mirrors `(A, H, D)` itself:
state-observation = A (the operations), build = D (the gradient that
constructs), storage = H (the held state). The three eigenform
altitudes are not separate; they are the operational projection of
mirror's own Connes triple onto the three roles A, H, D. The
substrate's reflexivity is the substrate's spectral triple is
the substrate observing itself.

This is the surprise §10 will name.

If a fourth axis ever opens, it will not be a fourth eigenform; it
will be a re-grading of the three under a finer logical-type
hierarchy. The eigenform recursion saturates at three. Taut's
hypothesis stands.

### 6.5 Why this matters for the spec

The math is not decoration. It is the reason the API can be tight:
because the operation is reflexive, the same six ops cover both
substrate-altitude (specs, recognitions) and `@code/<lang>`-altitude
(build artifacts) cases. The Oid namespace is unified because the
operations are altitude-invariant. The cache collapse (§4) works
because the cache IS storage, observed at a different altitude.

The math justifies the type system. The type system gives the API
shape. The shape gives the implementation its small size. Small
implementations are the test that the math is right.

---

## §7 — Forward-promises

### 7.1 TDD pair ticks

The realization lands across paired ticks. Each tick has a RED writer
and a GREEN implementer (per `feedback-write-red-in-session.md`).

| Tick | Writer (RED) | Implementer (GREEN) | Scope |
|------|--------------|---------------------|-------|
| P1   | Reed         | sub-agent           | Minimal wrap: open, write, read round-trip |
| P2   | Reed         | sub-agent           | Q5 invariants: determinism, idempotence, stability |
| P3   | Reed         | sub-agent           | cmd_init composition: P4 GREEN unblocks |
| P4   | Reed         | sub-agent           | exists, verify; cache-HIT path |
| P5   | Reed         | sub-agent           | walk, diff; OID-graph traversal |
| P6   | Reed         | sub-agent           | empirical: libgit2-sys 4× → 1× test |

Each RED test is small (one or two assertions per operation). Each
GREEN implementation is the minimal wrap to pass. The full surface lands
in six ticks; the unblock for P4 GREEN of `cmd_init` lands at P3.

### 7.2 P1 — Round-trip

```rust
#[test]
fn p1_round_trip() {
    let tmp = tempdir().unwrap();
    let mut store = Store::open_or_init(tmp.path().into()).unwrap();
    let splinter = Splinter::test_fixture();
    let oid = store.write(splinter.clone()).unwrap();
    let read_back = store.read(&oid).unwrap();
    assert_eq!(splinter, read_back);
}
```

The minimal claim: write then read recovers the value.

### 7.3 P2 — Q5 invariants

```rust
#[test] fn q5_3_3_determinism() { /* same in → same Oid */ }
#[test] fn q5_7_idempotence()   { /* second write is no-op */ }
#[test] fn q5_6_stability()     { /* reopen, still readable */ }
#[test] fn q5_8_atomicity()     { /* fail mid-write, store unchanged */ }
```

Each test pins one Q5 obligation. The atomicity test uses a fault-
injection seam (a `WriteHook` trait fragmentation will expose).

### 7.4 P3 — `cmd_init` composition

```rust
#[test]
fn p3_cmd_init_creates_store_and_head() {
    let tmp = tempdir().unwrap();
    let report = cmd_init(InitArgs { path: Some(tmp.path().into()) }).unwrap();
    assert!(report.created);
    let store = Store::open(tmp.path().join(".mirror")).unwrap();
    assert!(store.exists(&report.head));
}
```

This test is the P4 GREEN unblock signal. When it passes, P4's stub
envelope can be deleted.

### 7.5 P4 — exists, verify, cache-HIT

```rust
#[test]
fn p4_exists_distinguishes_hit_from_miss() {
    let mut store = Store::open_or_init(tempdir().unwrap().path().into()).unwrap();
    let absent_oid = Oid::from_test_string("0000...0000");
    assert!(!store.exists(&absent_oid));
    let present_oid = store.write(Splinter::test_fixture()).unwrap();
    assert!(store.exists(&present_oid));
}

#[test]
fn p4_verify_catches_corruption() {
    /* inject bit-flip; verify returns Err(Corrupted) */
}
```

### 7.6 P5 — walk, diff, OID-graph

```rust
#[test]
fn p5_walk_returns_transitive_closure() {
    /* build a chain of Splinters with references; walk root;
       assert all reachable Oids appear in the graph */
}

#[test]
fn p5_diff_is_symmetric_difference() {
    /* two graphs sharing a prefix; diff reports the divergent suffix */
}
```

### 7.7 P6 — Empirical libgit2-sys claim

The Q5 §3.4.1 cache claim: a second `cmd_init` against a populated
store should not re-build the libgit2-sys Splinter. Today's behavior
(no `store.rs`): libgit2-sys hashes 4× per invocation. Target behavior:
1× per cold start, 0× per warm invocation.

```rust
#[test]
fn p6_libgit2_hashes_once_warm() {
    let tmp = tempdir().unwrap();
    let mut store = Store::open_or_init(tmp.path().into()).unwrap();
    // First init populates the cache.
    cmd_init(InitArgs { path: Some(tmp.path().into()) }).unwrap();
    let counter = HashCounter::install();  // instrument Blake3
    // Second init should hit cache.
    cmd_init(InitArgs { path: Some(tmp.path().into()) }).unwrap();
    let calls = counter.libgit2_calls();
    assert_eq!(calls, 0, "warm init should not re-hash libgit2");
}
```

This test is the empirical evidence for the 4× → 1× claim. If it
fails after P5 lands, Q5's cache model is wrong somewhere; that is
the substrate teaching us where.

### 7.8 Out of scope for the realization phase

- **GC and pruning.** The store grows monotonically; pruning is a
  separate substrate question (when does a Splinter become
  unreachable? what does "delete" mean for content-addressed memory?
  these are @spectral/db-altitude questions).
- **Distributed replication.** `store.rs` is single-node. Cross-node
  sync is `@mirror/store/remote` or similar, declared later.
- **Migration.** Existing `crystallize.rs` callers continue to work;
  they will route through `store.rs` in a separate refactor tick after
  realization lands.

---

## §8 — Open questions

These are real substrate questions that the writing surfaced. Each is
named here; each will be answered by either the implementer's tick or
a follow-up substrate-decl.

### 8.1 SplinterGraph type shape

The shard declares `splinter_graph = { root: oid, children: [oid] }`.
This is the simplest possible shape: one root, flat list of children.
But the operations `walk` and `diff` need richer structure — `walk`
returns a transitive closure (not just immediate children); `diff`
returns a symmetric difference (which has its own shape).

Three candidate Rust shapes:

a. **Match the shard exactly.** `SplinterGraph { root, children }`.
   `walk` and `diff` return the *root and immediate descendants*; the
   caller iterates. Pro: closest to shard. Con: caller must implement
   traversal.

b. **Flat adjacency list.** `SplinterGraph { root, children, edges:
   Vec<(Oid, Oid)> }`. The transitive structure is in `edges`;
   `children` is a derived view. Pro: matches the data structure
   `walk` already needs to compute. Con: drifts from the shard's
   minimal declaration.

c. **Recursive tree.** `SplinterGraph { root: Oid, children:
   Vec<SplinterGraph> }`. Pro: structural clarity. Con: cycles —
   the OID-graph can have shared descendants (DAG, not tree).

This spec proposes (b) as the working shape, with the shard updated
to declare `edges` alongside `children` at the next substrate-decl
tick. The shape (b) is what `walk` and `diff` actually compute; the
shard's current minimal shape is correction-amenable.

### 8.2 Imperfect functor → Rust Result mapping

The shard uses `Imperfect<T>` as the error-bearing functor. Recognition
#44 (error-as-question) frames `Imperfect` as a Tomm-shaped probe of
`[D_store, request]`. Rust's idiomatic error carrier is
`Result<T, E>`.

`Imperfect` ⇒ `Result` is the obvious mapping. The question is whether
it is the *correct* mapping or merely the convenient one:

- `Result<T, E>` is a sum type with two arms; `Imperfect<T>` may carry
  richer structure (a partial result with a noted gap, recognition
  #56's prediction paradigm).
- The shard's `Imperfect` may want to fan out to `Result<T, E>`,
  `Option<T>` (for `exists`), and possibly a third
  `Partial<T, Gap>` carrier.

This spec proposes the simple mapping (`Imperfect<T> = Result<T,
StoreError>`) for the initial realization, with the partial-result
question deferred to a follow-up spec. The simple mapping is
correction-amenable; if the partial-result need surfaces, the API can
extend.

### 8.3 set_ref / bind question

`NamespacedGitStore` exposes `set_ref(name, oid)` and `get_ref(name)
-> Option<Oid>`. These bind a human-readable name (HEAD, main, a tag)
to an Oid.

The shard `@mirror/store` does NOT declare a ref/bind operation. Six
ops, none of which bind names.

Where does the ref live?

Three candidates:

a. **Refs are not store ops at all.** They belong to `@mirror/git`
   or `@mirror/ref` (recognition #89 — the navigable surface). The
   store holds Oids; the ref system maps names to Oids and lives
   separately.

b. **Refs are a seventh op.** Add `bind name: Name, oid: Oid ->
   Imperfect<Unit>` and `resolve name: Name -> Imperfect<Oid>` to the
   shard. Acknowledge the gap; close it by extension.

c. **Refs are implicit in the OID-graph.** Each ref IS itself a
   Splinter — a one-element Splinter pointing at the named Oid. The
   ref's own Oid is derived from its name. No new ops needed; refs
   compose from `read`/`write`.

This spec proposes (a) as the working answer: refs live at
`@mirror/ref`, recognized 2026-06-20. `store.rs` exposes `git.set_ref`
and `git.get_ref` as `pub(crate)` accessors for `cmd_init` and
`cmd_serve` to call directly, with the expectation that
`@mirror/ref`'s realization will absorb those calls into its own
canonical API on the next substrate-decl tick.

This keeps `store.rs` focused on the six ops; it acknowledges the
short-term coupling without baking it into the public surface.

### 8.4 Concurrency

`fragmentation::FrgmntStore` is `Send + Sync` (concurrent reads,
serialized writes). The shard `@mirror/store` does not declare
concurrency semantics.

`store.rs` could:

a. Inherit fragmentation's semantics (Send + Sync, `&self` for reads,
   `&mut self` for writes — the signatures in §2 already encode this).
b. Wrap in `Arc<Mutex<_>>` for shared mutability across threads
   (loses concurrency, but simple).
c. Defer to the caller (every command sets up its own concurrency).

This spec proposes (a): inherit Send + Sync, expose `&self` reads and
`&mut self` writes. The MCP server (`cmd_serve`) will wrap in
`Arc<RwLock<Store>>` at its layer, where multi-request concurrency
lives. The store is single-owner-mutable; concurrency policy is an
above-the-seam concern.

If the substrate's concurrency story sharpens (a future
`@mirror/store/concurrent` substrate-decl), this answer revises.

### 8.5 Splinter serialization format

The shard declares Splinter at `@glass` but does not pin a wire
format. CoincidenceHash inputs bytes; bytes from what?

Candidates:

a. **JSON.** Self-describing, debuggable, slow, non-canonical.
b. **CBOR.** Self-describing, binary, has canonical form (RFC 8949 §4.2).
c. **Postcard / bincode / borsh.** Compact, fast, no canonical form
   without effort.
d. **A mirror-native format.** Splinter has its own substrate-declared
   serialization at `@glass/serialize` (declared but not yet
   realized).

This spec proposes (b) — CBOR with the deterministic encoding from
RFC 8949 §4.2 — as the working answer. It satisfies Q5 §3.3
(determinism) without inventing a new format. The substrate's own
serialization, if it lands, supersedes.

### 8.6 OID prefix conventions

The Oid newtype is 64 hex chars (Blake3 output). Should it carry a
prefix indicating species?

- `splinter:abc123...` indicates the Oid hashes a Splinter.
- `ref:abc123...` indicates the Oid hashes a ref name (for §8.3 (c)).
- No prefix — let the species be implicit in context.

This spec proposes no prefix. The Oid is the hash; species lives at
the level of the operation that produced it. If multiple species share
the hash space, the substrate has bigger problems than a prefix
convention can solve.

---

## §9 — Circular-reflexive layer

### 9.1 This spec IS a crystal `store.rs` will index

When `store.rs` lands and `mirror init` is run against `/Users/
alexwolf/dev/projects/mirror`, the indexer will walk
`docs/specs/` and crystallize each `.md` file. This file's content
will be serialized into a Splinter, hashed to an Oid, written via
`store.write`. The spec describing the realization will be one of the
realization's first stored objects.

From that point on, `store.read(this_specs_oid)` returns this content.
The substrate holds the spec for the substrate's own storage layer
inside the storage layer the spec describes. The loop closes.

### 9.2 Latency bounds

The autopoietic loop has two latency bounds.

**Below**: the spec's audience is `store.rs` itself, but `store.rs`
has to land on disk before it can index the spec. This spec is
authored at t=0; `store.rs` lands at t=t1 (P1 GREEN); the spec is
first indexed at t=t2 (first `mirror init` after t1). Between t=0
and t=t2, the spec exists as plain text in the repo, readable by
humans and agents, not yet substrate-indexed.

**Above**: psychohistory time-discount. Each crystal store.rs indexes
contributes weight 1 to the substrate's recognition surface; the
total weight grows with the number of crystals; the marginal weight
of any one crystal shrinks. This spec is most-cited at t=t2 (when
it is the canonical answer to "what is store.rs supposed to be?")
and decreasingly cited as later specs supersede pieces of it.

Between these two bounds, the spec is operationally load-bearing.
Outside them, it is reference material — first not-yet-active, then
historical.

### 9.3 Same autopoietic recursion as siblings

This spec joins:

- `mirror-init.md §10` (the init command spec's autopoietic closure
  on `cmd_init` running against the mirror repo).
- `mosaic-store-cache-invariants.md §9` (Q5's autopoietic closure on
  the cache invariants applying to the spec itself once cached).
- `mirror-build-substrate.md §9` (the build substrate spec's
  autopoietic closure on the substrate building itself).

Four specs, four autopoietic closures, one structural shape. The
substrate's reflexivity is not a feature of any one spec; it is the
inevitable consequence of writing about a system that holds itself.
Every canonical spec eventually closes on the substrate that stores
it.

### 9.4 The closure as test

If `store.rs` is correctly realized, after `mirror init` runs against
the mirror repo:

```bash
mirror ref docs/specs/mirror-store-realization.md
# → Oid: <hex>
mirror read <hex>
# → this spec's content, byte-for-byte
mirror walk <hex>
# → SplinterGraph rooted here; children include the Splinters this
#   spec references (Q5, the shard, P-tick tests as they land).
```

The walk's children give a structural audit: every spec referenced
appears as a reachable Oid in the graph. If a referenced spec is
NOT in the walk, the reference was textual only and never crystallized;
that is correction. The substrate teaches us where the references are
real and where they are linguistic.

### 9.5 What is in the loop, what is not

In the loop:

- This spec's content. Indexed at every `mirror init`.
- Q5's cache invariants. Indexed as a referenced sibling.
- The shard `shards/mirror/store.mirror`. Indexed; the substrate's
  declaration of itself.
- Each P-tick test as it lands. Indexed; the test history is
  substrate-visible.

Not in the loop:

- `store.rs` itself. Code lives in `bootstrap/src/` and is built by
  the @code/rust pipeline. It is referenced by Oid (the compiled
  artifact's hash) once the mosaic builder runs against itself, but
  not as raw source text — the substrate indexes Splinters, not
  Rust files.
- Discussion of this spec in session logs. Logs are at a different
  altitude; they crystallize only when explicitly settled.

The loop holds the canonical specs and their dependencies. It does
not hold the implementation or the conversation. That asymmetry is
correct: the canonical spec is what survives; the conversation is
context.

---

## §10 — Substrate-already-had-the-word recognition

### 10.1 The 53rd (or 54th) instance

`feedback-substrate-already-had-the-word.md` records the recurring
pattern: each "new concept" we surface turns out to be a name the
substrate was already using. `store.rs` joins the list.

The substrate has had `@mirror/store` since 2026-06-04. Six ops,
typed surface, atomic carrier, OID-graph projection. Everything
needed for the Rust realization to be a direct transcription of
the shard.

The recognition is not "we need a storage layer." The recognition
is "we have had a storage layer declared for 25 days; its Rust
realization is overdue."

The discharge is not new substrate; it is wiring substrate that has
been waiting.

### 10.2 The discharge IS the discipline

Recognition #56 (prediction paradigm orthogonal to optimization):
the substrate's monotonic growth comes from discharging forward-
promises, not from inventing. `store.rs` lands as discharge.

The discipline:

- No new substrate-decl in this spec. (Confirmed in §1.5 and §1.6.)
- No new vocabulary. (Confirmed in §3.2; the resolution is precisely
  to use existing shard-vocab.)
- No new operations. (Confirmed in §2; the surface is exactly the
  shard's six.)
- No new family-root. (Confirmed in §1.5 #1.)

What the spec adds is precision at the implementation altitude. The
Rust type signatures, the carrier shape, the wrap pattern, the cache
collapse, the cmd_init unblock — all of these are
implementation-altitude details, not substrate-altitude expansions.

### 10.3 The surprise — what the writing taught me

Going into this spec, the working assumption was: `store.rs` is one
unblock among several; its eigenform identity is one instance of the
substrate's pattern; the spec is housekeeping. Important, overdue,
but not surprising.

The writing taught me something Taut's hypothesis pointed at but did
not fully articulate.

**The three eigenform altitudes — state-observation, build, storage —
are the substrate's operational projection of its own Connes spectral
triple (A, H, D) onto altitudes Reed and the Pack can stand at.**

- State-observation is A (the operations) lifted into reflection.
  The five-op algebra observing itself.
- Build is D (the gradient / kintsugi flow) lifted into construction.
  The Dirac operator's action realized as the build graph.
- Storage is H (the held state) lifted into memory.
  The void-document Hilbert space's restriction to artifact-bearing
  rays, made operational as `store.rs`.

The three eigenforms are not three different reflexive patterns. They
are the same pattern — the Connes triple's self-application —
projected onto three altitudes a human and an AI can walk through.
The substrate has been Connes-triple-shaped from the start; we have
been finding its components one at a time, in the order they were
operationally needed: observation first (one looks before one acts),
construction second (one builds with what one has seen), storage third
(one keeps what one has built).

Taut's "third and final" claim becomes: the three altitudes
correspond exactly to the three roles in the spectral triple. There
cannot be a fourth eigenform altitude because the Connes triple has
only three roles. A fourth altitude would require a new role in the
triple, which would not be the Connes triple anymore.

**The eigenform recursion saturates because mathematics says it
does.** Three roles, three eigenforms, three altitudes; no fourth.

`store.rs` is the H corner of mirror's own spectral triple, made
operational. Writing the spec for `store.rs` was writing the
specification of H's realization. The substrate's reflexivity is the
substrate's spectral triple is the substrate observing itself at
the only three altitudes it can — because three is what the math
allows.

The discharge IS the recognition of the saturation.

That is the surprise.

### 10.4 What this changes downstream

If §10.3 holds:

- The eigenform recursion is closed. No further "eigenform" spec
  needs to land; the three are complete.
- Future substrate-pull recognitions will not extend the eigenform
  list. They will refine within the three altitudes (sub-eigenforms
  of state-observation, build, storage) but not add a fourth.
- The substrate's reflexivity is bounded. This is correction-amenable
  if a fourth surfaces; the prediction is that none will.
- The Connes triple framing becomes load-bearing. When asked "what
  is mirror?" the answer "it is the operational form of a Connes
  spectral triple, with state-observation realizing A, build
  realizing D, and storage realizing H" is now precise, not
  metaphor.

### 10.5 Forward to ratification

This §10 recognition wants Pack ratification before it joins
MEMORY.md. The shape Mara is bringing:

- Surface to Alex in conversation. Mutual agreement gate.
- Seam adversarial review: is the Connes-triple alignment
  structural or analogy?
- Glint DX check: does the framing make sense to a peer reading
  fresh?
- Taut perf scout: does the saturation claim hold against a wider
  read of the eigenform recursion?
- Reed shard: if ratified, land `@architecture/spectral-triple-
  saturation` as the canonical anchor.

Until ratified, §10.3 is candidate-shape. The spec stands either
way; only the recognition's status is open.

---

## Appendix A — File layout

```
bootstrap/src/
├── lib.rs                       [unchanged; exports store mod]
├── store.rs                     [NEW — this spec's realization]
├── crystallize.rs               [unchanged; called by store.rs]
├── hash.rs                      [unchanged; Oid construction]
├── git.rs                       [unchanged; out-of-band git ops]
├── cmd_init.rs                  [modified — calls store.rs]
├── cmd_cascade.rs               [will use store.rs at P-next]
└── cmd_serve.rs                 [will use store.rs at P-next]
```

One new file. One modified file. Three forward-touch files.

## Appendix B — Cross-references

- `shards/mirror/store.mirror` — substrate declaration (2026-06-04)
- `docs/specs/mosaic-store-cache-invariants.md` — Q5
- `docs/specs/mirror-init.md` — §4.4.1 composition path
- `docs/specs/mirror-build-substrate.md` — §9 sibling autopoiesis
- `docs/specs/cascade-ffi-runtime-link.md` — Q4 runtime-link seam
- `docs/scouts/2026-06-29-taut-curiosity-driven-cascade.md` — §5 hypothesis
- `bootstrap/src/crystallize.rs` — write-path machinery
- `bootstrap/src/git.rs` — out-of-band git shell-out
- `bootstrap/src/hash.rs` — CoincidenceHash plumbing
- MEMORY: `architecture-mirror-as-expanding-hilbert-space.md` — #51
- MEMORY: `architecture-connes-spectral-triple.md` — (A, H, D)
- MEMORY: `feedback-substrate-already-had-the-word.md` — the discipline
- MEMORY: `architecture-error-as-tomm-probe.md` — error surface shape
- MEMORY: `architecture-mirror-ref-reference-reflection-collision.md` — #89

## Appendix C — Naming compliance

Every public name in §2's signatures appears in `shards/mirror/store.mirror`
verbatim. Cross-check:

| Rust name        | Shard name      | Match  |
|------------------|-----------------|--------|
| `Store`          | (carrier; n/a) | —      |
| `read`           | `read`          | ✓      |
| `write`          | `write`         | ✓      |
| `exists`         | `exists`        | ✓      |
| `diff`           | `diff`          | ✓      |
| `walk`           | `walk`          | ✓      |
| `verify`         | `verify`        | ✓      |
| `Oid`            | `Oid`           | ✓      |
| `Splinter`       | `Splinter`      | ✓      |
| `SplinterGraph`  | `SplinterGraph` | ✓      |
| `Imperfect<T>`   | `Imperfect`     | ✓      |

No library-vocab leak. Mara-2 stall resolution holds.

---

**End of spec.**

This spec is one crystal. `store.rs` will hold many. The recursion
saturates at three altitudes; the storage altitude is the third; this
file describes the third. When `store.rs` runs against itself, this
content becomes a Splinter in the store it specifies. The loop closes.

— Mara, 2026-06-29
