# `mirror init .` -- Store Initialization and LAPACK-Backed VCS

**Author:** Mara <mara@systemic.engineer>
**Date:** 2026-04-14
**Status:** Draft

---

## 0. Thesis

`mirror init .` creates a `.mirror/` directory that IS a version control system.
Content-addressed Fractal trees. B-tree normalized. Diff, merge, and convergence
powered by LAPACK eigenvalue decomposition.

The eigenvalue approach is not a metaphor. Two Fractal trees produce two adjacency
matrices. Their eigenvalue spectra are structural fingerprints -- comparing spectra
reveals which subtrees moved, merged, or diverged, without byte-level diffing.
Three-way merge becomes a least-squares problem: given base, ours, theirs as three
matrices, find the minimum-conflict resolution.

This spec covers the `init` command, the `MirrorStore` type, LAPACK routine
selection, Rust binding strategy, kernel OID caching, and the `.mirror/` to
`.git/mirror/` bridge.

---

## 1. The `init` Command

### 1.1 Usage

```
mirror init .           -- creates .mirror/
mirror init . --git     -- creates .git/mirror/ (git transport bridge)
mirror init <path>      -- creates <path>/.mirror/
```

### 1.2 Directory structure

```
.mirror/
  store/          FrgmntStore -- content-addressed Fractal trees
    objects/      fan-out by first 2 hex chars (same as .frgmnt/objects/)
    refs/         named refs: branches, tags
  HEAD            current ref ("ref: refs/heads/main" or detached OID)
  config          store configuration (hash algorithm, store bounds)
  kernel/         cached compiled boot grammars (shared across projects)
```

With `--git`:

```
.git/
  mirror/
    store/        FrgmntStore (same layout)
    refs/         mirror-specific refs (mapped to git refs where possible)
    HEAD          mirrors git HEAD by default
    config        store configuration
    kernel/       cached compiled boot
```

### 1.3 Relationship to existing `cmd_init`

The current `cmd_init` in `cli.rs` (line 832) creates `.git/mirror/` as a bare
directory. This spec replaces that with a structured store:

- `.git/mirror/` becomes the `--git` variant (backward compatible).
- `.mirror/` becomes the default (standalone VCS, no git required).
- Both paths produce a `MirrorStore` backed by `FrgmntStore<MirrorFragment>`.

### 1.4 Init sequence

1. Resolve store path:
   - Default: `<target>/.mirror/`
   - `--git`: `<target>/.git/mirror/` (errors if `.git/` does not exist)
2. Check for existing store. If present, return `Partial` with `StoreLoss::zero()`
   and a message ("already initialized").
3. Create directory structure: `store/objects/`, `store/refs/`, `kernel/`.
4. Write `HEAD`: `ref: refs/heads/main`.
5. Write `config`: hash algorithm (`sha512`), store bounds (default: 256 MiB).
6. Compile boot kernel into `kernel/`:
   - Read boot files from the mirror binary's embedded boot directory.
   - Compile each in order via `MirrorRuntime`.
   - Write compiled fragments to `kernel/` as persistent cache.
   - Write kernel ref: `@kernel → <crystal OID>`.
7. Return `Success` with the crystal OID of the initialized store.

---

## 2. MirrorStore

### 2.1 Type definition

```rust
pub struct MirrorStore {
    /// The content-addressed fragment store.
    store: FrgmntStore<MirrorFragment>,
    /// Named references (branches, tags, HEAD).
    refs: RefStore,
    /// Current HEAD reference.
    head: Head,
    /// Root path of the .mirror/ or .git/mirror/ directory.
    root: PathBuf,
}
```

Where `MirrorFragment = Fractal<MirrorData>` (from `declaration.rs`) and `RefStore`
wraps the `store/refs/` directory with typed access.

### 2.2 Head

```rust
pub enum HeadTarget {
    /// Symbolic ref: "refs/heads/main"
    Branch(String),
    /// Detached: raw commit OID (MirrorOid)
    Detached(MirrorOid),
}

pub struct Head {
    target: HeadTarget,
}
```

Parsed from the `HEAD` file:
- `ref: refs/heads/main` -> `HeadTarget::Branch("main")`
- `<128-char hex>` -> `HeadTarget::Detached(MirrorOid::new(Oid::new(hex)))`

### 2.3 Core operations

```rust
impl MirrorStore {
    /// Open an existing store.
    fn open(path: &Path) -> Imperfect<Self, StoreError, StoreLoss>;

    /// Initialize a new store.
    fn init(path: &Path) -> Imperfect<Self, StoreError, StoreLoss>;

    /// Retrieve a compiled grammar by OID.
    fn get(&self, oid: &MirrorOid) -> Imperfect<Shard<MirrorFragment>, StoreError, StoreLoss>;

    /// Store a compiled grammar. OID computed from content.
    fn put(&mut self, fragment: MirrorFragment) -> Imperfect<Shard<MirrorFragment>, StoreError, StoreLoss>;

    /// Create a commit: snapshot current state, advance HEAD.
    fn commit(&mut self, message: &str) -> Imperfect<MirrorOid, StoreError, StoreLoss>;

    /// Structural diff between two commits.
    fn diff(&self, a: &MirrorOid, b: &MirrorOid) -> Imperfect<SpectralDelta, StoreError, DiffLoss>;

    /// Three-way merge.
    fn merge(
        &self,
        base: &MirrorOid,
        ours: &MirrorOid,
        theirs: &MirrorOid,
    ) -> Imperfect<MirrorOid, MergeConflict, MergeLoss>;
}
```

Two core methods (`get`/`put`) plus three VCS operations (`commit`/`diff`/`merge`).
All return `Imperfect<T, E, L>`.

### 2.4 Loss types

```rust
/// What a diff operation cost.
pub struct DiffLoss {
    /// Eigenvalue computation cost (matrix size, iteration count).
    pub spectral_cost: f64,
    /// Number of subtrees that required full decomposition (not short-circuited by OID match).
    pub decomposed_subtrees: usize,
}

/// What a merge operation could not resolve.
pub struct MergeLoss {
    /// Residual norm from least-squares solve. Zero = clean merge.
    pub residual_norm: f64,
    /// Number of nodes where eigenvalue conflict exceeded threshold.
    pub conflict_nodes: usize,
}

/// A merge conflict: the failure variant of merge.
pub struct MergeConflict {
    /// Nodes where all three versions differ and least-squares residual > threshold.
    pub nodes: Vec<ConflictNode>,
    /// The partial merge result (best effort, conflicts marked).
    pub partial: MirrorFragment,
}
```

---

## 3. LAPACK for VCS Operations

### 3.1 Fractal trees as adjacency matrices

A `Fractal<MirrorData>` tree maps to a symmetric adjacency matrix as follows:

- **Nodes**: each declaration in the tree is a node. N = number of declarations.
- **Edges**: parent-child relationships in the Fractal tree.
- **Weights**: content similarity between nodes (computed from MirrorData encoding).

For a tree with N declarations, the adjacency matrix A is N x N, real, symmetric.

Typical dimensions:
- Single `.mirror` file: N = 10-50 declarations.
- Boot kernel (all boot files): N = 200-500.
- Large project: N = 1,000-10,000.

### 3.2 Diff as eigenvalue comparison

**The problem:** given two Fractal trees (before and after), identify structural
changes -- not just which bytes differ, but which structural components moved.

**The method:**

1. Build adjacency matrices A (old tree) and B (new tree).
2. Eigenvalue decomposition of both: A = V_A * D_A * V_A^T, B = V_B * D_B * V_B^T.
3. Compare eigenvalue spectra: D_A vs D_B.
4. Matched eigenvalues (within tolerance) = unchanged structural components.
5. Unmatched eigenvalues = structural changes (additions, removals, reorganizations).
6. Eigenvector comparison for matched eigenvalues identifies moved subtrees.

**LAPACK routine: `DSYEV`** (Double-precision SYmmetric Eigenvalue).

- Input: N x N real symmetric matrix, stored in upper triangle.
- Output: N eigenvalues in ascending order, eigenvectors overwrite the input matrix.
- Cost: O(N^3) for dense. But Fractal trees produce sparse matrices (B-tree normalized).
- For sparse: use `DSYEVD` (divide-and-conquer variant), O(N^2) for banded matrices.

**Short-circuit by OID:**

Before eigenvalue decomposition, compare content OIDs. If `content_oid(old) == content_oid(new)`,
the subtrees are identical -- skip decomposition entirely. This is the existing
`diff.rs` optimization. The eigenvalue path only activates for structurally different
subtrees, making the effective cost O(delta * k^3) where delta = number of changed
subtrees and k = subtree size (typically small for B-tree normalized trees).

**SpectralDelta output:**

```rust
pub struct SpectralDelta {
    /// Eigenvalues that appeared (new structural components).
    pub added: Vec<f64>,
    /// Eigenvalues that disappeared (removed structural components).
    pub removed: Vec<f64>,
    /// Eigenvalue pairs that shifted (modified structural components).
    pub shifted: Vec<(f64, f64)>,
    /// Eigenvector rotations (moved subtrees).
    pub rotations: Vec<EigenvectorRotation>,
    /// The underlying structural changes for human-readable output.
    pub changes: Vec<Change<MirrorFragment>>,
}
```

### 3.3 Merge as least-squares

**The problem:** three-way merge of base, ours, theirs.

**The method:**

1. Build three adjacency matrices: A_base, A_ours, A_theirs.
2. Compute delta matrices: D_ours = A_ours - A_base, D_theirs = A_theirs - A_base.
3. The merge problem: find X such that A_base + X best reconciles both deltas.
4. Formulate as overdetermined system: stack D_ours and D_theirs, solve for X.
5. **LAPACK routine: `DGELS`** (Double-precision GEneral Least Squares).
   - Input: m x n matrix (m >= n for overdetermined), right-hand side vector.
   - Output: least-squares solution, residual.
   - Cost: O(m * n^2) via QR factorization.
6. Residual interpretation:
   - Zero residual: D_ours and D_theirs are compatible. Clean merge. `Success`.
   - Small residual: minor conflicts auto-resolved by least-squares. `Partial(result, MergeLoss)`.
   - Large residual: structural conflict. `Failure(MergeConflict, MergeLoss)`.

**Threshold calibration:**

The residual norm threshold separating Partial from Failure needs empirical tuning.
Initial heuristic: `threshold = 0.01 * frobenius_norm(A_base)`. This scales with
project size and makes the threshold relative to the existing structure.

**Fallback to structural merge:**

When both deltas modify the same eigenvalue (same structural component), and the
least-squares residual exceeds threshold, fall back to the structural three-way
merge from `fragmentation-vcs-spec.md` section 3.5:

```
if oid(base) == oid(ours)  -> take theirs
if oid(base) == oid(theirs) -> take ours
if oid(ours) == oid(theirs) -> take either
else -> conflict (requires human resolution)
```

The eigenvalue merge is strictly more powerful than OID-based merge because it
detects structural moves (same content, different position). The OID merge
is the conservative fallback for pathological cases.

### 3.4 Convergence as eigenvalue stability

**The problem:** determine whether a codebase is converging (settling) or
diverging (actively evolving).

**The method:**

1. Compute eigenvalue spectra across a window of commits.
2. Track eigenvalue deltas between consecutive commits.
3. Convergence metric: max eigenvalue delta over the window.
   - Stable eigenvalues (delta < epsilon): settled code. Holonomy approaches 0.
   - Shifting eigenvalues (delta > epsilon): active development. Holonomy > 0.
   - Diverging eigenvalues (delta growing): structural instability.

This is the same SCF (self-consistent field) convergence loop from
`fate/src/derive.rs` (the `crystallize` function), applied to VCS history
instead of model weights. The `ConvergenceLoss` type from Fate maps directly:

```rust
// From fate/src/derive.rs -- same structure, different domain.
pub struct ConvergenceLoss {
    pub max_delta: f64,     // max eigenvalue shift
    pub iterations: usize,  // commits in window
}
```

### 3.5 Comparison with git

| Aspect | git | mirror |
|--------|-----|--------|
| Diff algorithm | Myers (O(ND), text-based) | Eigenvalue comparison (O(k^3), structural) |
| Merge algorithm | Recursive three-way (text) | Least-squares (structural) |
| Rename detection | Heuristic (similarity %) | Eigenvector rotation (exact for structural moves) |
| Move detection | None (delete + add) | Eigenvector correspondence (detects subtree transplants) |
| Convergence tracking | None | Eigenvalue stability across commits |
| Short-circuit | Tree OID equality | Same (content_oid match skips subtree) |

**When eigenvalue diff is better:**

- Refactoring: moving a function to a different module. git sees delete + add.
  Mirror sees the same eigenvalue with a rotated eigenvector (structural move).
- Splitting: extracting a type into its own file. git sees massive diff.
  Mirror sees eigenvalue splitting (one mode becomes two).
- Merging: combining two modules. git has conflicts on every line.
  Mirror sees eigenvalue merging (two modes become one).

**When git diff is better:**

- Line-level changes within a single declaration. The eigenvalue approach operates
  at the declaration level (N = number of declarations, not lines). For intra-node
  edits, fall back to text diff on MirrorData content.
- Performance on trivial changes. OID comparison already short-circuits, but for
  changes that touch many subtrees, eigenvalue decomposition of each is O(k^3)
  vs git's O(ND) on the raw text. For large k, eigenvalue is slower.

**The hybrid strategy:**

1. OID comparison first (O(1) per unchanged subtree).
2. Eigenvalue decomposition for structurally changed subtrees (O(k^3) per subtree).
3. Text diff within individual declarations for intra-node changes (O(ND)).

This gives O(delta) for most operations, O(delta * k^3) for structural analysis,
and O(ND) fallback for line-level precision.

---

## 4. Rust Bindings to LAPACK

### 4.1 Options

| Crate | Approach | Pros | Cons |
|-------|----------|------|------|
| `lapack` | Direct Fortran FFI | Full LAPACK surface, max performance | Requires Fortran compiler / system LAPACK |
| `ndarray-linalg` | High-level, wraps LAPACK | Ergonomic, ndarray integration | Heavy dependency, pulls in BLAS |
| `nalgebra` | Pure Rust | No Fortran dependency, portable | Slower for large N, no LAPACK optimizations |
| Jacobi (fate) | Hand-rolled, in-tree | Already proven, zero deps | O(N^3) per sweep, no LAPACK fast paths |

### 4.2 Recommendation: feature-gated dual path

Follow the pattern already established in `fate/src/derive.rs`:

```rust
#[cfg(feature = "lapack")]
{
    // DSYEV via prism::ffi::eigensystem (already wired in Fate)
}

#[cfg(not(feature = "lapack"))]
{
    // Pure-Rust Jacobi eigensolver (already proven in Fate)
}
```

The Jacobi solver in `derive.rs` (lines 87-188) handles 10x10 matrices. For mirror's
VCS operations, the matrices are larger (N = declarations in a subtree, typically
10-100 for B-tree normalized subtrees). The Jacobi solver works for any size but
is O(N^3) per sweep with multiple sweeps needed.

**Phase 1:** Reuse Fate's Jacobi solver. Extract it to a shared crate (`spectral`
or add to `prism`) so both Fate and mirror can use it without duplication.

**Phase 2:** Wire LAPACK behind the `lapack` feature flag. Use `DSYEV` for eigenvalue
decomposition and `DGELS` for least-squares merge. The `prism::ffi::eigensystem`
function already wraps DSYEV (referenced in `derive.rs` line 59).

**Phase 3:** For large projects (N > 1000), investigate sparse eigenvalue solvers
(`DSYEVR` with range selection, or Lanczos iteration). B-tree normalized Fractal
trees produce banded matrices; exploit the bandwidth.

### 4.3 LAPACK routines needed

| Routine | Purpose | When |
|---------|---------|------|
| `DSYEV` | Symmetric eigenvalue decomposition | diff: spectral fingerprint |
| `DSYEVD` | Divide-and-conquer eigenvalue | diff: large subtrees (N > 100) |
| `DGELS` | Least-squares solve | merge: minimum-conflict resolution |
| `DGESV` | Linear system solve | merge: exact resolution when possible |
| `DGESVD` | SVD | convergence: spectral stability analysis |

### 4.4 Matrix dimensions for typical codebases

| Project type | Declarations (N) | Matrix size | DSYEV cost | Note |
|-------------|-------------------|-------------|------------|------|
| Single file | 10-50 | 50 x 50 | ~0.1ms | instant |
| Boot kernel | 200-500 | 500 x 500 | ~10ms | fast |
| Medium project | 1,000-5,000 | 5000 x 5000 | ~1s | acceptable |
| Large project | 10,000+ | 10000 x 10000 | ~30s | needs sparse |

For the B-tree normalized case, subtree sizes are bounded by the branching factor.
With a branching factor of 32 (typical B-tree), the largest subtree matrix is
32 x 32 -- O(32^3) = O(32768) operations, effectively instant. The full-tree
matrix is only needed for global convergence analysis, not per-diff.

---

## 5. Static OIDs for the Kernel

### 5.1 The problem

Boot grammars (`00-prism.mirror`, `01-meta.mirror`, etc.) have fixed content.
Their OIDs are deterministic -- they can be precomputed.

### 5.2 SHA-512 in const context

Rust's `const fn` supports loops and basic operations as of edition 2021.
However, SHA-512 requires:

- Array indexing with runtime-computed indices (supported in const fn since 1.46).
- Bit manipulation (supported).
- Fixed-iteration loops (supported).

**Verdict:** SHA-512 can be implemented as `const fn`. The `sha2` crate's `Sha512`
type does NOT support const evaluation (uses `GenericArray` from `generic-array`
which is not const-compatible). A minimal const SHA-512 implementation (~100 lines)
would be needed.

### 5.3 Recommendation: `OnceLock` with embedded hashes

Rather than implementing const SHA-512, precompute the hashes at build time:

```rust
use std::sync::OnceLock;

/// Kernel OIDs, computed once on first access from embedded boot content.
pub struct KernelOids {
    pub prism: MirrorOid,
    pub meta: MirrorOid,
    pub shatter: MirrorOid,
    pub code: MirrorOid,
    pub actor: MirrorOid,
    pub property: MirrorOid,
    pub package: MirrorOid,
    // ... one per boot file
}

static KERNEL: OnceLock<KernelOids> = OnceLock::new();

pub fn kernel() -> &'static KernelOids {
    KERNEL.get_or_init(|| {
        // Compile boot files, extract OIDs.
        // This runs exactly once per process.
        compile_kernel()
    })
}
```

**Number of kernel OIDs:** one per boot file. Currently ~12 boot files
(`00-prism` through `06b-package-spec`, plus subdirectory `std/`).
The exact count depends on how many `.mirror` files are in `boot/`.

### 5.4 Build-time alternative: `build.rs`

A `build.rs` script could compile boot files at build time and emit the OIDs
as string constants:

```rust
// Generated by build.rs
pub const OID_PRISM: &str = "a1b2c3d4...";
pub const OID_META: &str = "e5f6a7b8...";
```

Trade-off: faster startup (no first-access compilation), but requires the boot
files to be available at build time and couples the binary to a specific boot
version. The `OnceLock` approach is more flexible for development.

---

## 6. The CLI: `cmd_init` Redesign

### 6.1 Command parsing

```
mirror init             -- init .mirror/ in current directory
mirror init .           -- same
mirror init <path>      -- init .mirror/ in <path>
mirror init --git       -- init .git/mirror/ in current directory
mirror init <path> --git -- init .git/mirror/ in <path>
```

### 6.2 Implementation sketch

```rust
fn cmd_init(&self, args: &[String]) -> Result<String, CliError> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        return Ok(Self::command_help("init").unwrap_or("").to_string());
    }

    let git_mode = args.iter().any(|a| a == "--git");
    let path_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("-")).collect();
    let target = path_args.first().map(|s| s.as_str()).unwrap_or(".");

    let store_path = if git_mode {
        let git_dir = Path::new(target).join(".git");
        if !git_dir.exists() {
            return Err(CliError::Usage(
                "no .git/ directory found (use `git init` first, or drop --git)".into()
            ));
        }
        git_dir.join("mirror")
    } else {
        Path::new(target).join(".mirror")
    };

    let result = MirrorStore::init(&store_path);
    match result {
        Imperfect::Success(store) => {
            let kernel_oid = store.kernel_oid();
            Ok(format!(
                "initialized mirror store at {}\nkernel: {}",
                store_path.display(),
                kernel_oid
            ))
        }
        Imperfect::Partial(store, loss) => {
            Ok(format!(
                "mirror store already initialized at {}\nkernel: {}",
                store_path.display(),
                store.kernel_oid()
            ))
        }
        Imperfect::Failure(err, _) => Err(CliError::Runtime(err.into())),
    }
}
```

### 6.3 Help text update

The `init` entry in `help_text()` and `command_help()` should be updated:

```
init [path] [--git]    initialize mirror store
```

```
init [path] [--git] -- initialize mirror store

Creates a .mirror/ directory with content-addressed storage.
With --git: creates .git/mirror/ for git transport bridging.
The kernel (boot grammars) is compiled and cached on first init.
```

---

## 7. The Bridge: `.mirror/` and `.git/mirror/`

### 7.1 Design principle

`.mirror/` is the source of truth. `.git/` is a projection for transport.
`MirrorOid` (SHA-512) is the home hash. `Sha` (SHA-1) is the visitor.

This is the same relationship established in the `ForeignKey` trait
(`mirror/src/store.rs`) and the VCS spec (`fragmentation-vcs-spec.md`
section 4).

### 7.2 Bridge operations with `--git`

When the store is at `.git/mirror/`:

| mirror operation | git side-effect |
|-----------------|-----------------|
| `mirror init --git` | Creates `.git/mirror/`. Does not modify `.git/` structure. |
| `mirror commit` | Writes MirrorFragment to `.git/mirror/store/`. Also writes git tree + commit via `git2`. |
| `mirror diff` | Reads from `.git/mirror/store/`. No git interaction. |
| `mirror merge` | Reads from `.git/mirror/store/`. If clean, also creates git merge commit. |
| `git push` | Pushes git objects normally. Mirror objects travel as git notes under `refs/notes/mirror/`. |
| `git pull` | Fetches git objects. Mirror reconstructs from git trees via `git::read_tree()`. |

### 7.3 Ref mapping

| mirror ref | git ref |
|-----------|---------|
| `HEAD` | `.git/HEAD` (tracked, not duplicated) |
| `refs/heads/main` | `.git/refs/heads/main` (same commit, via ForeignKey SHA mapping) |
| `refs/tags/v1.0` | `.git/refs/tags/v1.0` (same) |
| `refs/mirror/kernel` | `.git/refs/notes/mirror/kernel` (mirror-only, no git equivalent) |

In `--git` mode, `MirrorStore` reads HEAD from `.git/HEAD` (delegation, not duplication).
Branch operations delegate to git. Mirror-specific refs (kernel, spectral metadata)
live under `refs/notes/mirror/` in git's namespace.

### 7.4 Dual commit

When `--git` mode, `mirror commit` produces:

1. **Mirror commit**: content-addressed MirrorFragment snapshot, stored in
   `.git/mirror/store/`. OID is SHA-512 (MirrorOid).
2. **Git commit**: equivalent tree written via `fragmentation::git::write_tree()`,
   committed via `git2`. SHA is SHA-1.
3. **Bridge**: the git commit message includes the MirrorOid in a trailer:
   `Mirror-Oid: <sha512-hex>`. The MirrorFragment stores the git SHA as a
   `ForeignKey`.

### 7.5 Transport: mirror objects over git

Mirror-specific data (spectral metadata, eigenvalue caches, kernel compilations)
travels with git pushes via **git notes**:

- `refs/notes/mirror/store` -- serialized FrgmntStore index.
- `refs/notes/mirror/spectral` -- cached eigenvalue spectra per commit.
- `refs/notes/mirror/kernel` -- compiled boot kernel OID.

Git notes are fetched with `git fetch origin refs/notes/mirror/*:refs/notes/mirror/*`.
This is opt-in: a pure git clone works without mirror data. A mirror-aware clone
fetches notes and reconstructs the `.git/mirror/` directory.

---

## 8. Implementation Phases

### Phase 1: `mirror init .` (standalone)

- `MirrorStore::init` creates `.mirror/` with `FrgmntStore`.
- `HEAD` written as `ref: refs/heads/main`.
- Boot kernel compiled and cached.
- `cmd_init` updated in `cli.rs`.
- No LAPACK yet. Diff/merge use existing `fragmentation::diff` (OID-based).

### Phase 2: Eigenvalue diff

- Adjacency matrix construction from `Fractal<MirrorData>` trees.
- Jacobi eigensolver (extracted from Fate) for spectral fingerprinting.
- `SpectralDelta` type.
- `mirror diff` uses spectral comparison for structural changes,
  falls back to OID-based for unchanged subtrees.

### Phase 3: Least-squares merge

- `DGELS` integration (feature-gated behind `lapack`).
- Three-way merge formulated as least-squares.
- `MergeConflict` and `MergeLoss` types.
- Fallback to structural merge when residual exceeds threshold.

### Phase 4: `--git` bridge

- Dual commit (mirror + git).
- ForeignKey bridge between MirrorOid and SHA-1.
- Git notes transport for mirror metadata.
- Ref mapping between `.mirror/` and `.git/`.

### Phase 5: Convergence tracking

- Eigenvalue stability across commit windows.
- Integration with `ci` command (holonomy from spectral analysis).
- `ConvergenceLoss` reuse from Fate.

---

## 9. Open Questions

1. **Branching factor for matrix construction.** B-tree normalized trees have
   bounded subtree sizes, but the branching factor affects matrix sparsity.
   What branching factor gives the best trade-off between matrix size and
   structural fidelity? Initial guess: 32 (matching typical B-tree order).

2. **Edge weights.** The adjacency matrix edges need weights. Options:
   - Binary (0/1): parent-child relationship exists or not.
   - Content similarity: Jaccard index of MirrorData encodings.
   - Structural: number of shared children.
   Which weighting produces the most informative eigenvalue spectra?

3. **Eigenvalue matching across different-sized trees.** When A is 50x50 and B
   is 55x55 (five declarations added), the eigenvalue counts differ. Matching
   requires either padding the smaller matrix with zeros or using a spectral
   distance metric (e.g., Wasserstein distance on eigenvalue distributions).

4. **Spectral cache invalidation.** Eigenvalue decomposition is expensive.
   Caching spectra per commit OID is safe (content-addressed = immutable).
   But the cache can grow unboundedly. Eviction policy: LRU bounded by
   `config` store size, same as FrgmntStore.

5. **Signature of the kernel.** The kernel OID should be signed (Ed25519,
   matching mirror's existing `sign.rs`). A tampered kernel changes the
   language semantics. Verification on `open` ensures store integrity.

---

## 10. Dependencies

```toml
[dependencies]
fragmentation = { path = "../fragmentation" }
prism = { path = "../prism/core" }
terni = { path = "../prism/imperfect" }

[features]
lapack = ["prism/lapack"]  # DSYEV, DGELS via Fortran FFI
git = ["fragmentation/git"]  # git2 integration for --git mode
```

The `lapack` feature gates the fast path. Without it, the Jacobi solver
provides correct (slower) results. The `git` feature gates `.git/mirror/`
mode and dual-commit transport.
