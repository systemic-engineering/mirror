# Store IS MerkleTree: Package Management as Tree Traversal

**Date:** 2026-04-15
**Status:** Research
**Authors:** Alex + Reed

## The Insight

Store doesn't *store* MerkleTrees. Store IS a MerkleTree. `Store: MerkleTree`.

The store has an Oid. The store has children. The store IS a node in a tree of stores. This is not a metaphor. It's a structural identity that collapses the package manager into tree operations.

```
Garden (Store + MerkleTree)
  ├── ProjectA (Store + MerkleTree)
  │     ├── @prism (MerkleTree)
  │     └── @list (MerkleTree)
  ├── ProjectB (Store + MerkleTree)
  │     ├── @prism (MerkleTree)  ← same Oid, shared
  │     └── @custom (MerkleTree)
```

## What Already Exists

The infrastructure for this is already built. It just hasn't been named.

**fragmentation's `Fragmentable` trait** (`fragment.rs`):
```rust
pub trait Fragmentable {
    type Data: Encode;
    type Hash: HashAlg;
    fn self_ref(&self) -> &Ref<Self::Hash>;
    fn data(&self) -> &Self::Data;
    fn children(&self) -> &[Self];
}
```

This IS a MerkleTree trait. `self_ref` = the node's Oid. `data` = the node's content. `children` = the node's children. `Fragmentable` is `MerkleTree` under a different name.

**fragmentation's `Fractal` enum** (`fragment.rs`):
```rust
pub enum Fractal<E, H: HashAlg> {
    Shard { ref_: Ref<H>, data: E },                          // leaf
    Fractal { ref_: Ref<H>, data: E, fractal: Vec<...> },     // branch
    Lens { ref_: Ref<H>, data: E, target: Vec<H> },           // edge reference
}
```

Shard = leaf node. Fractal = branch node. Lens = external reference by Oid. This IS the MerkleTree with three node kinds.

**fragmentation's `Repo` trait** (`repo.rs`):
```rust
pub trait Repo {
    fn write_tree(&mut self, node: &Self::Node) -> String;  // returns root Oid
    fn read_tree(&self, oid: &str) -> Option<Self::Node>;
}
```

`write_tree` recursively stores all nodes, returns the root content Oid. `read_tree` retrieves by Oid. This IS `put` and `get` on a MerkleTree.

**fragmentation's `diff`** (`diff.rs`):
```rust
pub fn diff<F: Fragmentable>(old: &F, new: &F) -> Vec<Change<F>>
```

Compares two trees by root hash. Same hash = unchanged. Different hash = walk children. This IS Merkle diff.

**mirror's `Store` trait** (`store.rs`):
```rust
pub trait Store {
    fn insert(&mut self, value: Self::Value) -> Imperfect<Self::Shard, ...>;
    fn get(&self, oid: &MirrorOid) -> Imperfect<Self::Shard, ...>;
}
```

insert = put. get = get. The store is content-addressed. The Shard carries the Oid.

The gap: mirror's `Store` stores flat values. fragmentation's `Repo` stores trees. The insight is that the Store IS a tree node in a tree of stores, which means `Store: Fragmentable`. One trait, not two.

## 1. Package Resolution as Tree Traversal

### Traditional

```
Cargo.toml declares: mirror = "0.5"
  → cargo reads crates.io index (git repo, ~400MB)
  → cargo runs SAT solver over version constraints
  → cargo resolves transitive dependency graph
  → cargo writes Cargo.lock
  → O(n * m) where n = packages, m = versions
```

### MerkleTree

```
use @garden/security-checks
  → garden.get("security-checks")     # tree traversal to child node
  → child IS a MerkleTree node
  → child.self_ref() IS the version    # Oid = version
  → child.children() ARE dependencies  # children = deps
  → O(log n)
```

No resolution algorithm. The tree IS the resolution. Walking from root to child IS finding the package. Reading the child's Oid IS reading the version. Reading the child's children IS reading the dependencies.

Why no SAT solver: SAT solvers exist because version ranges create a constraint satisfaction problem. ">=1.0, <2.0" intersected with ">=1.5" across a transitive graph is NP-hard in the general case. But a MerkleTree doesn't have version ranges. It has Oids. An Oid is exact. There's nothing to solve. The tree has one state, and that state IS the solution.

Version ranges are a compression artifact. When you can't address content directly, you describe it indirectly with ranges. Content addressing removes the indirection.

## 2. Dependency Resolution IS Merkle Diff

### Traditional

```
cargo update
  → fetch new index
  → re-run SAT solver
  → diff old Cargo.lock vs new Cargo.lock (text diff)
  → hope the text diff captures the semantic diff
```

### MerkleTree

```
Update @list in ProjectA:
  old_tree: ProjectA { @prism: Oid_p, @list: Oid_L1 }
  new_tree: ProjectA { @prism: Oid_p, @list: Oid_L2 }

  fragmentation::diff(&old_tree, &new_tree) →
    Unchanged(@prism)         # same Oid, nothing changed
    Modified(@list: L1 → L2)  # Oid changed, this is the update

  Recurse into @list's children:
    diff(&L1, &L2) →
      Added(@new_dep)         # L2 has a new transitive dep
      Unchanged(@shared_dep)  # same Oid, shared
```

The Merkle diff IS the dependency diff. It tells you exactly which nodes changed, which stayed the same, and which were added or removed. No lockfile parsing. No heuristics. The tree structure carries the information.

fragmentation already implements this in `diff.rs`. The `Change` enum (Added, Removed, Modified, Unchanged) is the dependency changelog.

## 3. Deduplication IS Dependency Sharing

### Traditional

```
~/.cargo/registry/cache/
  mirror-0.5.0.crate       # 2MB
  mirror-0.5.0/ (unpacked) # 8MB
  mirror-0.4.0.crate       # 2MB (different version, mostly same code)
  mirror-0.4.0/ (unpacked) # 8MB
```

Two projects using mirror-0.5.0: stored twice on disk (once per `target/` directory), plus the shared registry cache.

### MerkleTree

```
Garden
  ├── ProjectA → @prism: Oid_abc
  ├── ProjectB → @prism: Oid_abc   ← same Oid
```

Same Oid = same node. Stored once. The MerkleTree deduplicates by construction. No special sharing logic. No symlinks. No hardlinks. The hash does it.

fragmentation's `Repo::write_tree` already does this: `self.objects.entry(oid).or_insert_with(...)`. If the Oid exists, the write is a no-op. Two projects referencing the same grammar get the same physical storage.

This extends to sub-trees. If @prism depends on @meta, and @meta hasn't changed, the entire @meta subtree is shared. The deduplication is structural, not just per-file.

## 4. Publishing IS Put

### Traditional

```
cargo publish
  → cargo package (create .crate tarball)
  → cargo verify (build in temp dir)
  → HTTP PUT to crates.io API
  → crates.io updates git index
  → crates.io stores .crate in S3
  → crates.io runs background jobs
```

### MerkleTree

```
garden.write_tree(&project_tree)
  → recursively stores all nodes
  → returns root Oid
  → garden's root Oid changes (it's a MerkleTree, one child changed)
  → new garden root IS the new registry state
```

Publishing is `write_tree`. The project tree IS a `Fractal` node. Its children are its grammars and dependencies. The garden's root hash changes because a child changed. The new root IS the new garden state. Content-addressed, deterministic, verifiable.

No tarball. No HTTP. No background jobs. Just tree insertion.

## 5. Installation IS Get + Lens

### Traditional

```
cargo add mirror
  → update Cargo.toml
  → cargo fetch (download .crate, unpack to registry/src/)
  → cargo build (copy to target/, compile)
```

### MerkleTree

```
local_store.add_child(garden.read_tree("@mirror"))
  → Option 1: copy the Fractal node (data + Oid)
  → Option 2: create a Lens node (Oid reference only)
```

The `Lens` variant in fragmentation is the key. A Lens carries `target: Vec<H>` — Oid references to external trees. The data stays in the garden. The local store references it by Oid.

```rust
Fractal::Lens {
    ref_: local_ref,
    data: "@mirror".into(),
    target: vec![garden_oid],
}
```

No download. No unpack. No `target/` directory. When the compiler needs @mirror's types, it does `garden.read_tree(oid)`. The data is fetched on demand. The Lens is the "installed" state.

For offline operation: the Fractal variant (full copy) instead of Lens. Same Oid either way. The consumer doesn't know or care which storage strategy was used.

## 6. What Dies

| Traditional Concept | Why It Exists | MerkleTree Equivalent | Status |
|---|---|---|---|
| **Lockfile** (Cargo.lock) | Pin exact versions across builds | Project store's root Oid | The tree IS the lock |
| **Version resolver** (SAT solver) | Reconcile version ranges | Tree traversal | Ranges don't exist; Oids are exact |
| **Dependency conflicts** | Two ranges that don't intersect | — | One Oid per node; conflicts are structurally impossible |
| **`cargo update`** | Re-resolve with new constraints | Change child Oid, diff trees | Explicit Oid swap, diff shows exactly what changed |
| **Registry index** (crates.io-index) | Map names to versions to tarballs | Garden tree | The garden IS the index |
| **Lockfile diffing** | Understand what changed in an update | `fragmentation::diff` | Merkle diff IS the dependency diff |
| **`cargo publish`** | Package + upload + index update | `write_tree` | Tree insertion |
| **`cargo install`** | Download + unpack + build | `read_tree` or Lens reference | Tree read or Oid reference |
| **Version strings** ("1.5.0") | Human-readable version identity | Oid (SHA-512 hex) | Content IS identity |
| **Semver compatibility** | Guess if upgrade is safe | Merkle diff + grammar diff | Structural: same types = compatible |
| **Yanking** | Remove broken version from index | Remove child from garden tree | Tree mutation, new root Oid |
| **Features/optional deps** | Conditional compilation | Tree variants (different children = different Oid) | Each feature combo IS a different tree |

### What Dies Completely

- **Lockfiles.** The project's Oid IS the lock. Reproducibility is structural, not a side-file.
- **SAT solving.** No version ranges means no constraint satisfaction problem.
- **Dependency conflicts.** A tree node has exactly one Oid. Two "versions" of the same dep would be two different child nodes with different names, not a conflict.
- **Registry as separate infrastructure.** The garden IS the registry. No crates.io. No npm. No separate service.

### What Transforms

- **Semver** doesn't die, but it changes role. It becomes a human-readable annotation on a tree node, not the resolution key. The Oid resolves. Semver communicates intent. `mirver` in `06-package.mirror` already has this shape: `{ oid, semver }`.
- **`@package` grammar** (`06-package.mirror`) simplifies. `resolve`, `install`, `publish` become tree operations. `diff` becomes `fragmentation::diff`. `compatible` becomes grammar-level type comparison.

### What Stays

- **Naming.** Humans need names, not SHA-512 hashes. `@prism` is a name that points to an Oid in the tree. The name-to-Oid mapping is the tree structure itself.
- **Discovery.** "What packages exist?" is `garden.children()`. Listing is tree enumeration.
- **Trust.** Who can `write_tree` into the garden? Signing, permissions, governance. The MerkleTree handles integrity (content = hash). Trust is orthogonal.
- **Build.** The MerkleTree stores grammars and source. Compilation still happens. The input to the compiler is a tree traversal instead of a directory walk.

## 7. What This Means for spec.mirror

Current spec.mirror:
```mirror
grammar @mirror/project {
  store in fragmentation-git {
    path = .git/mirror
  }
  boot = [ 00-prism, 01-meta, ... ]
  deploy @cli
}
```

With Store = MerkleTree:
```mirror
grammar @mirror/project {
  -- the store IS the project tree. No separate declaration needed.
  -- .git/mirror IS a MerkleTree whose root Oid IS the project state.

  boot = [
    use @prism          # local_store.read_tree("@prism")
    use @meta           # local_store.read_tree("@meta")
    use @garden/checks  # garden.read_tree("@garden/checks") → Lens
  ]

  deploy @cli
}
```

Each `use` IS a tree traversal:
- `use @prism` → walk local store tree to @prism child → read its Oid → that IS the version → its children ARE its deps
- `use @garden/checks` → walk garden tree to checks child → create Lens in local store → Oid reference, not copy

The spec IS the dependency declaration. The store IS the resolver. One mechanism.

## 8. Implementation Path

### What fragmentation already provides

| Operation | fragmentation primitive | Package manager operation |
|---|---|---|
| Store content | `Repo::write_tree` | publish |
| Retrieve content | `Repo::read_tree` | install / fetch |
| Compare versions | `diff::diff` | update / audit |
| Deduplicate | `HashMap::entry` in Store | sharing |
| Reference external | `Fractal::Lens` | lazy install |
| Walk tree | `Fragmentable::children` | resolve |
| Verify integrity | `content_oid` | verify / audit |

### What needs to be built

1. **Garden as a `Repo`** — a shared store that multiple projects reference. Currently each project has its own `MirrorGitStore`. The garden is a `Repo` whose children are project trees.

2. **Lens resolution** — when a Lens target is accessed, fetch from the garden's `Repo`. fragmentation has the `Lens` variant but no resolution logic yet.

3. **Grammar-level compatibility** — two Oids are "compatible" if their grammars are type-compatible. This is `@package.compatible(mirver, mirver)` from `06-package.mirror`, but implemented as grammar tree comparison rather than semver range intersection.

4. **Name mapping** — `@prism` → Oid in the tree. Currently names are filesystem paths. The tree should carry names as node data, with Oids as identity.

### What does NOT need to be built

- SAT solver
- Version range parser
- Registry protocol (HTTP API, index format)
- Lockfile format
- Lockfile merge conflict resolution
- Yanking infrastructure
- Feature resolution algorithm

## 9. The Algebra

If Store implements Fragmentable:

```
Store: Fragmentable
  self_ref() → store's root Oid
  data()     → store metadata (name, config)
  children() → child stores / grammars
```

Then Store composes with every operation that works on Fragmentable:
- `diff(store_v1, store_v2)` — what changed between project states
- `write_tree(store)` — persist the entire project
- `content_oid(store)` — the project's identity hash
- `walk(store)` — enumerate all contents

And Store composes with Store:
- Garden is a Store whose children are Stores
- Project is a Store whose children are grammar trees
- Grammar tree is a Fractal whose children are type nodes

One trait. One algebra. The package manager is a special case of tree operations on content-addressed data. No new concepts. No new infrastructure. Just the recognition that the Store already IS the thing it stores.
