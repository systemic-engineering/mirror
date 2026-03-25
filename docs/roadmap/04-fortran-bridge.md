# 04 — Fortran Bridge

Three crates. Three concerns. One content-addressed pipeline.

```
conversation    — proof + coordination  (BEAM)
coincidence     — numerics + compilation (Fortran)
fragmentation   — storage + addressing   (git)
```

---

## The Split

Coincidence splits into core and domain modules:

**coincidence-core** — the Fortran bridge. Domain-agnostic. Takes any graph,
produces content-addressed Fortran vectors. Eigendecomposition via LAPACK
dsyev, heat kernel, spectral analysis, EigenCache keyed by matrix blob OID.
`native/spectral.f90` wraps dsyev with `bind(c)` interface, build.rs compiles
with gfortran, links Accelerate (macOS) or lapack+blas (Linux). Compile once,
resolve forever.

**coincidence-cosmo** — cosmology domain. Galaxy catalogs, sigma, Lambda_eff, Local
Group evolution, differential clock rates. Provides the graph, core does the
math.

**coincidence-org** — organizational domain. Email networks, Enron-style
analysis. Same core, different graphs.

Additional domain modules follow the same pattern. The core doesn't know what
the graph represents. The domain doesn't know how eigenvalues are computed.

---

## The Compiler Flow

```
Graph → Laplacian → matrix_hash → cache lookup
  ├─ HIT  → EigenSystem from disk (content-addressed shard)
  └─ MISS → Fortran dsyev → EigenSystem → store to fragmentation
              ↓
         Spectrum, Crystal, heat kernel, evolution
```

EigenSystem implements Fragmentable — same matrix, same eigensystem, same OID.
Laplacian implements Singularity (Iso): `collapse()` computes the eigensystem,
`refract()` reconstructs via V × diag(lambda) × V^T. The optics map:

```
Singularity (Iso):   Laplacian ←→ EigenSystem    (lossless)
Singularity (Prism): Laplacian  → Spectrum        (eigenvalues only)
Singularity (Lens):  Tree       → Crystal         (observer-dependent)
Fragmentable:        EigenSystem (shard)           (content-addressed)
```

---

## Integration with Conversation

Grammar trees meet `Laplacian::from_tree()`. The compiler already produces
content-addressed trees. When conversation needs to compute — state space
distances, domain similarity, translation cost — it calls through
coincidence-core for the numerics.

Conversation says what to compute. Coincidence compiles it to Fortran vectors.
Fragmentation stores the results. The NIF boundary is already there.

A scientist can `git clone` the fragmentation repo, read the Fortran vectors,
and reproduce the entire computation without touching Rust or Gleam.
