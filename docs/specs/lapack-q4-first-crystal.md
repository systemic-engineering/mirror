# lapack-q4-first-crystal — scope verification + smallest first empirical crystal spec (math ↔ silicon path via @mirror/store/git 31e7d45)

*Mara, 2026-07-01. Scope-verification research tick discharging the two
questions Reed named after P4 GREEN landed cmd_init's content-addressed
persistence path. Q1 verifies the "LAPACK Q4 already empirically
operational as first crystal" claim from `docs/specs/reality-algebra-
math-and-glue.md` at `805676e`. Q2 specs the smallest first empirical
crystal from math to silicon that the substrate can now discharge.*

**Author:** Mara
**Date:** 2026-07-01
**Tag:** 📝 substrate-pull:realize (spec-only; no shards, no Rust; the
discharge target for a follow-up TDD-paired tick)
**Status:** Q1 verdict landed; Q2 smallest-first-crystal path named;
implementation forward-promised.

---

## §0 — Why this spec exists

Two claims in `805676e` (Mara's canonical spec at
`docs/specs/reality-algebra-math-and-glue.md`) named v1.0
spectral.engineer as "closer than the brief framed it":

1. **math ↔ silicon via LAPACK Q4 case** — "operational at project
   level" (spec §5.3, §5.4, §10 closure).
2. **math ↔ nl via Glint's prose-cascade species at `939eca6f`** —
   "operational at substrate-pull level" (spec §5.3, §10 closure).

Per the status-drift catch pattern
([[feedback-status-drift-catch-pattern]]; three instances in 72h caught
by grep-first / scout / adversarial review), operational claims like
these need verification before the substrate builds against them.

Q1 verifies claim 1 by grepping the substrate for LAPACK Q4 evidence.

Q2 specs the smallest first empirical crystal — the minimum discharge
that would land math ↔ silicon as "operational as first crystal" in the
strict content-addressed sense (a crystal on disk under
`.git/mirror/`, addressable by oid, retrievable by `mirror ref`).

The context: P4 GREEN landed at `31e7d45` (Reed, 2026-07-01) — the
`cmd_init` composition per Mara's `@mirror/store/git` species discharge
map (`1de09a9`). Content-addressed persistence is REAL now:
`NamespacedGitStore::open` + git ls-files + Splinter per-file + BLAKE3
root_oid + `set_ref("HEAD", root)` + flush all compose end-to-end. The
persistence path for a LAPACK crystal is UNBLOCKED.

---

## §1 — Q1 verdict: **aspirational, not operational**

The claim "LAPACK Q4 already empirically operational as first crystal"
in `805676e` is **substrate-pull-honest as an intent statement** but
NOT operational-as-first-crystal in the content-addressed sense the
canonical spec's §5.4 v1.0 criterion requires.

### §1.1 — What EXISTS (the operational parts)

Grep against the mirror repo at `31e7d45` surfaces the following
LAPACK-related substrate that IS operational:

#### §1.1.1 — LAPACK FFI is real and running

`bootstrap/src/sheaf_laplacian.rs::lambda_zero` calls
`prismqueer::ffi::eigenvalues(n, &matrix)` — the LAPACK `dsyev`
symmetric-eigendecomposition path. Verified:

- `bootstrap/Cargo.toml` line 30: `prismqueer = { version = "0.1",
  features = ["bundle", "lapack"] }` — the `lapack` feature "wires
  prism-core's Fortran FFI (the dsyev / dgesvd wrappers over LAPACK;
  gfortran + Accelerate framework on darwin)".
- `bootstrap/Cargo.lock` line 501: `prismqueer 0.1.1` from crates.io
  registry — pinned dependency, live.
- `bootstrap/src/sheaf_laplacian.rs` lines 34-38: `lambda_zero` calls
  `prismqueer::ffi::eigenvalues` (LAPACK `dsyev`).
- `bootstrap/src/sheaf_laplacian.rs` line 296 (elided by lint):
  `let evals = match prismqueer::ffi::eigenvalues(n, &matrix)` —
  the actual FFI call site.
- `bootstrap/tests/fortran_ffi.rs` — proves Rust ↔ Fortran FFI works
  end to end via `extern "C" { fn dot5(a: *const f64, b: *const f64)
  -> f64; }` bound against `bootstrap/fortran/dot.f90` (compiled by
  `bootstrap/build.rs` via flang).
- `bootstrap/build.rs` — compiles Fortran source via flang; emits
  `cargo:rustc-link-lib=static=dot`.
- `bootstrap/src/realisation.rs` line 302: names "LAPACK FFI — dsyev /
  dgesvd over gfortran + Accelerate" as the realisation-layer entry
  for `sheaf_laplacian.rs`.

**Evidence verdict:** the LAPACK FFI pathway is operational at the
Rust altitude via `prismqueer`. `lambda_zero` really computes
eigenvalues via LAPACK `dsyev`. This is real, not aspirational.

#### §1.1.2 — Content-addressed persistence is real and running

Per Reed's P4 GREEN at `31e7d45`, `cmd_init` composes:

- `NamespacedGitStore::open(repo_path, "mirror")` — opens
  `.git/mirror/` namespace.
- `git ls-files` — enumerates the working-tree manifest.
- For each file: Splinter of bytes → BLAKE3 oid → `insert_persistent(
  store, format!("splinter:{oid}"), fractal_of(bytes), size_of(bytes))`.
- BLAKE3 root_oid via re-hashing the sorted `(path, oid)` list.
- `set_ref(store, "HEAD", root_oid)` + `flush()`.

15/15 init tests green at `31e7d45`. The persistence path is
end-to-end operational.

#### §1.1.3 — The forward-promises that name the intent

- `docs/specs/cascade-ffi-runtime-link.md` §7 — "The LAPACK case study
  (**forward-promise discharge target**)". The spec ITSELF names the
  LAPACK case as "the FIRST EMPIRICAL discharge target **when the
  substrate-decl shards land operationally**" (future tense; §7.3
  "First empirical discharge" is titled `first empirical discharge`
  and describes what "the empirical evidence the spec predicts" will
  look like, not what has been observed).
- `docs/specs/silicon.md` `27e9067` — names @silicon as the top-level
  autopoietic family-root; §0 says the spec IS "the first crystal;
  every subsequent @silicon emission is a later crystal" (autopoietic
  recursion at the spec altitude, not a content-addressed crystal
  under `@mirror/store/git`).
- `docs/specs/numerical-substrate-via-fortran.md` — the flang/mirror
  numerical-split spec (spec, not implementation).
- `roadmap/pending/phase-6-io-numerical-prism.md` — Phase 6
  forward-promises LapackBackend → `au(@code/fortran)` via flang;
  content-addressed in `@mirror/store`; **forward-promised, not
  landed**.

### §1.2 — What does NOT exist (the aspirational parts)

Grep against `shards/`, `docs/specs/`, and the git log surfaces the
following gaps that make the "operational as first crystal" claim
aspirational:

1. **No `shards/reality/algebra/silicon.mirror`.** The species-root at
   `shards/reality/algebra.mirror` lines 296-311 enumerates
   `@reality/algebra/silicon` as **"(forward-promised): subsumes
   today's @silicon work at `27e9067`"**. The species shard does not
   exist on disk (confirmed by absence in
   `shards/reality/algebra/` listing — only `math.mirror` present).
2. **No `@glue/math_silicon` sub-shard.** `shards/glue.mirror` enumerates
   the species roster; neither `math_silicon`, `math ↔ silicon`, nor
   `LAPACK` appear in `shards/glue.mirror` at any species-decl
   position. The `@glue/math_*` sub-shards are named as future species
   (spec §1.1 "Future @glue species (forward-promised, not landing
   this tick)") and are NOT present.
3. **No `@mirror/store/lapack` sub-species.** Only `@mirror/store/git`
   has landed; the species roster at `shards/mirror/store.mirror`
   forward-promises "other species when consumers pull" but does not
   enumerate a LAPACK persistence species.
4. **No content-addressed crystal on disk from a LAPACK computation.**
   `bootstrap/src/sheaf_laplacian.rs::lambda_zero` computes the
   eigenvalue and returns it as an in-memory `Eigenvalue` value; the
   computation's result IS NOT written to `.git/mirror/` as a
   content-addressed crystal. No LAPACK-result crystal has ever been
   persisted through `NamespacedGitStore::insert_persistent`.
5. **No LAPACK-specific spec.** `docs/specs/` has zero files named
   `lapack*` or `q4*`; the LAPACK case lives inside the broader
   `cascade-ffi-runtime-link.md` at §7 as a **forward-promise**, not
   as a discharged spec.

### §1.3 — What "operational as first crystal" would require (per the canonical spec's own criterion)

Per `805676e` §5.4 the v1.0 spectral.engineer criterion for "math ↔
silicon operational as first crystal" is:

> **The empirical discharges per §5.3 are operational**: at least
> math ↔ silicon (LAPACKPrism) AND math ↔ nl (Glint's prose-cascade
> species) MUST be operational as first crystals.

And per §5.3:

> **math ↔ silicon: LAPACKPrism via Q4's LAPACK case.** Per
> `docs/specs/cascade-ffi-runtime-link.md` §7 and per `reality.md`
> §3.2.1, the LAPACK case is the Q4 forward-promise: a math content
> (theorem about symmetric positive-definite matrices admitting LL^T
> decomposition) discharged via @glue/math_silicon to silicon content
> (LAPACKPrism invoking dpotrf + dpotrs). The discharge is operational
> at the project level; the substrate-decl form is forward-promised
> at the @glue/math_silicon shard landing.

The canonical spec ALREADY concedes that the substrate-decl form is
forward-promised at the @glue/math_silicon shard landing. The
"operational at project level" phrase is a soft-focus qualifier —
LAPACK is on the tin, `dsyev` runs, `lambda_zero` returns a number.
None of that is a first-crystal in the strict content-addressed sense.

### §1.4 — Verdict

**Aspirational, not operational.**

- LAPACK FFI is real (`prismqueer::ffi::eigenvalues`).
- The computation exists (`sheaf_laplacian::lambda_zero`).
- Content-addressed persistence is real (P4 GREEN at `31e7d45`).
- **BUT** the composition — a math statement crystallized as
  content-addressed data, discharged via `@glue/math_silicon` through
  LAPACK, and persisted as a crystal in `.git/mirror/` — has NEVER
  BEEN EXECUTED.

The canonical spec `805676e`'s claim is the strongest defensible read
of what's true today ("operational at project level"; every ingredient
of the chain exists) but is NOT a first-crystal-in-the-content-
addressed-sense claim. The distinction matters because §5.4's v1.0
criterion is content-addressed-crystal-strict.

**Status-drift diagnosis (per [[feedback-status-drift-catch-pattern]]):**
This is instance #4 (72h + 24h window). The pattern: analytical chain
looked complete, individual ingredients verified, composition never
empirically tested. Recovery: name the smallest first crystal (Q2
below) so the composition can be executed as one commit.

---

## §2 — Q2: The smallest first empirical crystal

### §2.1 — The mathematical input

**Theorem (Cholesky decomposition of the 2×2 unit-scaled positive-
definite matrix).**

> *For A = [[4, 12], [12, 37]] (symmetric positive-definite), there
> exists a lower-triangular L = [[2, 0], [6, 1]] such that A = L·Lᵀ.*

Rationale for the choice:

- **Smallest non-trivial case.** 2×2 is the smallest matrix size
  where Cholesky decomposition is non-degenerate (1×1 is just
  a square root).
- **Numerically exact.** The chosen A has integer entries and the
  decomposition L has integer entries; `dpotrf` will return byte-
  stable results across platforms (no floating-point rounding
  drift).
- **Textbook example.** A = [[4, 12], [12, 37]], L = [[2, 0], [6, 1]]
  is the canonical Cholesky worked example (verify: L·Lᵀ = [[4, 12],
  [12, 4+36+1]] = [[4, 12], [12, 41]]... correction: L = [[2, 0],
  [6, 1]], Lᵀ = [[2, 6], [0, 1]], L·Lᵀ = [[4, 12], [12, 37]] ✓).
- **Named in the canonical spec.** Per `805676e` §5.3 the LAPACK
  case is "a math content (theorem about symmetric positive-definite
  matrices admitting LL^T decomposition) discharged via
  @glue/math_silicon to silicon content (LAPACKPrism invoking dpotrf
  + dpotrs)". The theorem is the substrate's named target.

### §2.2 — The LAPACK routine: `dpotrf`

Per `805676e` §2.1 and §5.3 the LAPACK Q4 case names `dpotrf` (double-
precision positive-definite triangular factorization) as the math ↔
silicon discharge routine. `dpotrf` is:

- The reference LAPACK routine for Cholesky decomposition.
- Column-major, in-place (over-writes A with L in the lower triangle).
- Deterministic per platform.
- Available on darwin via Accelerate framework (the same LAPACK
  distribution `prismqueer` already links).

### §2.3 — The discharge path (end-to-end)

The chain, tick by tick:

```
math_statement (bytes: theorem serialization)
  → @reality/algebra/math                       [species; landed]
  → @glue/math_silicon.translate(...)           [FORWARD-PROMISED]
  → @reality/algebra/silicon                    [FORWARD-PROMISED]
  → LapackPrism.dpotrf(A)                       [Rust; FORWARD-PROMISED]
     → prismqueer::ffi::???                     [Rust; check if dpotrf exists]
     → LAPACK dpotrf (Fortran; via Accelerate)  [operational]
     → L (bytes: column-major f64 matrix)       [returned]
  → Splinter(L_bytes)                           [operational; P4 GREEN]
  → BLAKE3 oid of L                             [operational; P4 GREEN]
  → NamespacedGitStore::insert_persistent(...)  [operational; P4 GREEN]
  → set_ref(store, "crystal/lapack_q4_first",   [operational; P4 GREEN]
            L_oid)
  → flush()                                     [operational; P4 GREEN]
```

The persistence path (last five arrows) is REAL now. The FFI path
(LAPACK dpotrf) is REAL now (via prismqueer or via a thin new
wrapper). The discharge lambdas (@glue/math_silicon.translate;
@reality/algebra/silicon witness) are FORWARD-PROMISED.

**The smallest first crystal is the L matrix itself, persisted as
content-addressed data under a named ref.** The crystal's oid IS the
substrate's empirical claim: "math statement about Cholesky
decomposition of A ↔ silicon computation of L = [[2,0],[6,1]]; both
addressable, both retrievable, both content-addressed".

Retrieval verification:

```
mirror ref crystal/lapack_q4_first
  → oid: blake3:<64-hex-chars>

mirror-store get_persistent oid
  → 32 bytes: [2.0, 0.0, 6.0, 1.0] as column-major f64
```

The retrieval MUST match the theorem's L. That match IS the
"operational as first crystal" discharge.

### §2.4 — The math-side crystal (dual persistence)

For the discharge to be BILATERAL per `805676e` §2.1 (matter aspect ↔
information aspect gauge-equivalent under @reality), the MATH SIDE
must ALSO be persisted:

```
math_statement (bytes: theorem serialization)
  → Splinter(theorem_bytes)
  → BLAKE3 oid of theorem
  → NamespacedGitStore::insert_persistent(...)
  → set_ref(store, "theorem/cholesky_2x2", theorem_oid)
```

Then the discharge itself is a THIRD content-addressed object:

```
morphism_bytes (the @glue/math_silicon.translate invocation record):
  source_prism:  @reality/algebra/math
  target_prism:  @reality/algebra/silicon
  source_oid:    theorem_oid
  target_oid:    L_oid
  morphism_kind: @arxiv/math/cholesky_decomposition.lapack_realization
  restriction:   { precision: f64, conditioning: well-conditioned,
                   layout: column-major }
  → Splinter(morphism_bytes)
  → BLAKE3 oid of morphism record
  → insert_persistent(...)
  → set_ref(store, "morphism/math_silicon/lapack_q4_first", morphism_oid)
```

Three crystals: theorem, L, morphism. The morphism's persistence IS
the substrate's typed witness that the Mesland correspondence holds
at the LAPACK altitude. The morphism_oid IS the discharge's identity.

### §2.5 — Why this is the SMALLEST

- **One 2×2 matrix**, not a 100×100 matrix.
- **One LAPACK routine** (`dpotrf`), not the LAPACK routine family.
- **One theorem**, not a proof.
- **Three crystals** (theorem / L / morphism), the minimum for a
  bilateral gauge-equivalent discharge.
- **Zero new species-roots**. Reuses the existing `@reality/algebra`
  and `@glue` family-roots.
- **Zero new @io altitudes**. Reuses the existing `@io/silicon`
  (cargo/LLVM/LAPACK) altitude.
- **Zero new content-addressing machinery**. Reuses `NamespacedGitStore`
  + Splinter + BLAKE3.
- **No @fate consultation.** No @bauchladen accumulation. The @fate /
  @bauchladen infrastructure is not required for the first crystal
  (it becomes required when a SECOND LAPACK realization competes for
  the same theorem; the first crystal has one realization, so no
  selection is needed).

The smallest first crystal is one commit that lands three shards, one
Rust binding, and one integration test that persists three crystals and
verifies the L matrix retrieves byte-equal.

---

## §3 — What SHARDS need to land first

Three shards, in dependency order:

### §3.1 — `shards/reality/algebra/silicon.mirror` — the silicon species

The species-root at `shards/reality/algebra.mirror` lines 296-311
already forward-promises this shard. It IS the load-bearing missing
species for the discharge.

Shape (per the species pattern at `shards/reality/algebra.mirror` §3.3
and per `docs/specs/reality-algebra-math-and-glue.md` §2.1):

```mirror
in @reality/algebra

prism @reality/algebra/silicon <= @reality/algebra {
  type matter_carrier      = transistor_state
  type information_carrier = executable_algebra
  witness algebra_closure  <- closure_under_5op_at_silicon
  witness gauge_action     <- 5op_uniform_at_silicon
  witness matter_information_gauge_equivalent
                           <- reality_gauge_collapse_at_silicon
  witness io_discharge     <- @io/silicon

  # 5-op block (specialized from @reality/algebra's species_descriptor
  # to H_silicon carriers)
  focus operator            # LAPACK routine handle
  project operator          # subset of the LAPACK family
  split operator            # decomposition (Cholesky, QR, SVD)
  lift operator             # scale up to larger dimensions
  settle result             # numerical result (matrix, vector, scalar)

  # ... obligation blocks discharge at the @io/silicon boundary
}

out @reality/algebra/silicon
```

The shard MUST inherit `in @reality/algebra` (species-root already
landed). It MUST declare the four constituents per §3.3. It MUST NOT
declare the LAPACK routines directly — those live at the @glue seam
below.

### §3.2 — `shards/glue/math_silicon.mirror` — the math ↔ silicon @glue species

Per `shards/glue.mirror` this species is enumerated at the family-
root but not landed. This shard IS the correspondence carrier —
the Mesland morphism at the math ↔ silicon boundary.

Shape (per `docs/specs/reality-algebra-math-and-glue.md` §2.1 and the
@glue family-root pattern):

```mirror
in @glue

prism @glue/math_silicon <= @glue {
  type correspondence = mesland_morphism
  type source_prism   = @reality/algebra/math
  type target_prism   = @reality/algebra/silicon
  type morphism_kind  = ref     # e.g., @arxiv/math/cholesky.lapack_realization
  type restriction    = record  # precision + conditioning + layout

  focus correspondence
  project correspondence
  split correspondence
  lift correspondence
  settle correspondence

  translate(c: correspondence, source: source_prism)
    -> imperfect<target_prism, lapack_handle, transparency(precision_loss)>
    { \ }
}

out @glue/math_silicon
```

The obligation block discharges through the LAPACK FFI (§4 below).

### §3.3 — `shards/reality/algebra/math/cholesky.mirror` — the math content for the theorem

Optional but recommended: a math-side content shard declaring the
Cholesky theorem as `@reality/algebra/math` content addressable by
name. Alternative: encode the theorem inline in the integration test.
Recommended path: land the theorem shard so the crystal's math side
has a substrate-declared shape, not just an ad-hoc byte serialization.

### §3.4 — Optional: `shards/mirror/store/lapack.mirror` (defer)

The canonical `@mirror/store/git` species is sufficient for the first
crystal. A LAPACK-specific persistence species would be premature
optimization; defer until a second species pulls for it (e.g.,
zero-copy LAPACK-result persistence via memory-mapped Splinter).

---

## §4 — What RUST needs to land

### §4.1 — `prismqueer::ffi::cholesky` (or similar) — LAPACK `dpotrf` binding

**Current status:** `prismqueer` exposes `eigenvalues` (LAPACK
`dsyev`). Verify (in a follow-up scout) whether `prismqueer::ffi`
exposes `dpotrf` / `cholesky` already; if not, add it.

**Shape (if new):**

```rust
// prismqueer/src/ffi.rs (or a new module)
extern "C" {
    fn dpotrf_(
        uplo: *const c_char,     // 'L' for lower-triangular result
        n: *const i32,           // matrix dimension
        a: *mut f64,             // input matrix (column-major); overwritten with L
        lda: *const i32,         // leading dimension
        info: *mut i32,          // return code
    );
}

pub fn cholesky(n: usize, a: &mut [f64]) -> Result<(), LapackError> {
    let uplo = b'L' as c_char;
    let n_i32 = n as i32;
    let mut info: i32 = 0;
    unsafe {
        dpotrf_(&uplo, &n_i32, a.as_mut_ptr(), &n_i32, &mut info);
    }
    if info == 0 {
        Ok(())
    } else {
        Err(LapackError::NotPositiveDefinite(info))
    }
}
```

If `prismqueer` upstream doesn't ship `cholesky`, the smallest first
crystal can inline the FFI declaration in
`bootstrap/tests/lapack_q4_first_crystal.rs` (test-only) or in a new
`bootstrap/src/lapack_q4.rs` module. Preference: upstream to
`prismqueer` for reusability, BUT the smallest first crystal can
land with an inline FFI declaration in bootstrap to unblock without
waiting for a `prismqueer` release.

### §4.2 — `cmd_crystallize` (or extend `cmd_init`) — the persistence invocation

The Rust surface the integration test calls:

```rust
pub fn crystallize_lapack_q4_first(
    store: &mut NamespacedGitStore,
    theorem_bytes: &[u8],
    a: &[f64],           // 2×2 symmetric positive-definite matrix
) -> Result<CrystalTriple, ...> {
    // 1. Persist theorem
    let theorem_oid = blake3_of(theorem_bytes);
    let theorem_fractal = encode(theorem_bytes)?;
    store.insert_persistent(format!("splinter:{theorem_oid}"),
                            theorem_fractal, theorem_bytes.len())?;

    // 2. Compute L via LAPACK dpotrf
    let mut a_mut = a.to_vec();
    prismqueer::ffi::cholesky(2, &mut a_mut)?;
    let l_bytes: &[u8] = bytemuck::cast_slice(&a_mut);

    // 3. Persist L
    let l_oid = blake3_of(l_bytes);
    let l_fractal = encode(l_bytes)?;
    store.insert_persistent(format!("splinter:{l_oid}"),
                            l_fractal, l_bytes.len())?;

    // 4. Persist morphism record
    let morphism = MesladdMorphism {
        source_prism: "@reality/algebra/math",
        target_prism: "@reality/algebra/silicon",
        source_oid: theorem_oid,
        target_oid: l_oid,
        morphism_kind:
            "@arxiv/math/cholesky_decomposition.lapack_realization",
        restriction: Restriction {
            precision: "f64",
            conditioning: "well-conditioned",
            layout: "column-major",
        },
    };
    let morphism_bytes = serialize(&morphism)?;
    let morphism_oid = blake3_of(&morphism_bytes);
    let morphism_fractal = encode(&morphism_bytes)?;
    store.insert_persistent(format!("splinter:{morphism_oid}"),
                            morphism_fractal, morphism_bytes.len())?;

    // 5. Set refs
    store.set_ref("theorem/cholesky_2x2", theorem_oid)?;
    store.set_ref("crystal/lapack_q4_first", l_oid)?;
    store.set_ref("morphism/math_silicon/lapack_q4_first", morphism_oid)?;
    store.flush()?;

    Ok(CrystalTriple { theorem_oid, l_oid, morphism_oid })
}
```

Recommended: land as `bootstrap/src/lapack_q4.rs` or as a new command
`cmd_crystallize_lapack_q4` (subcommand-scoped for testability).

### §4.3 — Integration test — the empirical discharge

`bootstrap/tests/lapack_q4_first_crystal.rs`:

```rust
#[test]
fn lapack_q4_first_crystal_persists_bilateral_gauge_equivalent() {
    let tmp = tempdir().unwrap();
    let repo = init_git_repo(tmp.path());
    let mut store = NamespacedGitStore::open(&repo, "mirror").unwrap();

    let theorem = b"theorem: for A = [[4,12],[12,37]], \
                    A = L*L^T with L = [[2,0],[6,1]]";
    let a: [f64; 4] = [4.0, 12.0, 12.0, 37.0];  // column-major

    let triple = crystallize_lapack_q4_first(&mut store, theorem, &a)
        .unwrap();

    // Retrieval verification
    let l_fractal = store.get_persistent(
        &format!("splinter:{}", triple.l_oid)).unwrap();
    let l_bytes = decode(&l_fractal).unwrap();
    let l: &[f64] = bytemuck::cast_slice(&l_bytes);

    assert_eq!(l.len(), 4);
    assert!((l[0] - 2.0).abs() < 1e-12);   // L[0,0]
    assert!((l[1] - 6.0).abs() < 1e-12);   // L[1,0]  (column-major)
    assert!(l[2].abs() < 1e-12);           // L[0,1]  (zero; upper triangle)
    assert!((l[3] - 1.0).abs() < 1e-12);   // L[1,1]

    // Named-ref verification
    let head_l_oid = store.get_ref("crystal/lapack_q4_first").unwrap();
    assert_eq!(head_l_oid, triple.l_oid);

    // Morphism-record verification
    let morphism_oid = store.get_ref(
        "morphism/math_silicon/lapack_q4_first").unwrap();
    assert_eq!(morphism_oid, triple.morphism_oid);
}
```

**This test is the empirical discharge.** It runs LAPACK on real data,
persists the result as a content-addressed crystal, retrieves it, and
verifies byte-equal reconstruction. When this test passes, the
"LAPACK Q4 operational as first crystal" claim becomes
substrate-pull-honest.

---

## §5 — What's already available (nothing else needs to land)

- **Content-addressed persistence** (`NamespacedGitStore` +
  `insert_persistent` + `set_ref` + `flush`) — Reed's P4 GREEN at
  `31e7d45`. Battle-tested by 15/15 init tests.
- **Splinter + BLAKE3 encoding** (`fragmentation::encoding::encode` +
  `fragmentation::fragment::Fractal`) — the substrate-typed
  fragment carrier. Named at `shards/mirror/store/git.mirror`
  lines 305-320.
- **LAPACK FFI infrastructure** (`prismqueer::ffi::eigenvalues`
  proves the pattern works; `dpotrf` is a sibling routine in the
  same LAPACK distribution). `bootstrap/Cargo.toml`'s `lapack`
  feature already pulls the linkage.
- **Accelerate framework linkage** (via `prismqueer` on darwin).
  Zero new build configuration required.
- **The species pattern** at `shards/reality/algebra.mirror` §3.3
  gives the exact structural template `silicon.mirror` follows.
- **The @glue species pattern** at `shards/glue.mirror` gives the
  exact structural template `math_silicon.mirror` follows.
- **The math species** at `shards/reality/algebra/math.mirror`
  provides the source_prism the morphism binds against.

Nothing else needs to land. The five-line diff below is roughly the
scope:

```
+ shards/reality/algebra/silicon.mirror     (species; ~150 lines)
+ shards/glue/math_silicon.mirror           (species; ~200 lines)
+ bootstrap/src/lapack_q4.rs                (Rust; ~80 lines)
+ bootstrap/tests/lapack_q4_first_crystal.rs (test; ~60 lines)
+ shards/reality/algebra/math/cholesky.mirror (optional; ~40 lines)
```

Roughly ~530 lines to close the "operational as first crystal" gap.
Under one tick of substrate-pull-realize work.

---

## §6 — Autopoietic recursion (honest disclosure)

This spec itself will become a content-addressed crystal once
`cmd_init` runs against `docs/specs/`. Its own oid will be a stable
identity. Future @fate inferences at the math ↔ silicon altitude will
load it as prior-art. The autopoietic recursion is structural.

Per Mara-2's discipline (`silicon.md` §0): naming the autopoietic
recursion is honest reporting, not justification. The spec's content
holds on its merits — the Q1 verdict, the Q2 smallest-first-crystal
path, the shard shapes, the Rust surface, the integration test —
independently of the autopoietic recursion. The crystal is real when
the test passes.

---

## §7 — Report-back summary

**Q1 verdict:** aspirational, not operational. LAPACK FFI is real;
content-addressed persistence is real; the composition has never been
executed. Evidence: `bootstrap/src/sheaf_laplacian.rs` runs LAPACK
`dsyev` via `prismqueer::ffi::eigenvalues`; P4 GREEN at `31e7d45`
wires content-addressed persistence; but `shards/reality/algebra/
silicon.mirror` does not exist, `shards/glue/math_silicon.mirror`
does not exist, no LAPACK-result crystal has been persisted through
`NamespacedGitStore::insert_persistent`, and no LAPACK-specific spec
exists in `docs/specs/`. The canonical spec `805676e` §5.3 itself
qualifies with "the substrate-decl form is forward-promised at the
@glue/math_silicon shard landing".

**Q2 smallest first crystal path:** theorem about 2×2 Cholesky
decomposition (A = [[4,12],[12,37]] → L = [[2,0],[6,1]]) discharged
via LAPACK `dpotrf`; persisted as three content-addressed crystals
(theorem / L / morphism record) via P4 GREEN's `NamespacedGitStore`
composition; verified by byte-equal retrieval through
`get_persistent`.

**Rust dependencies:** `prismqueer::ffi::eigenvalues` (LAPACK
`dsyev`) is operational. `dpotrf` is a sibling routine in the same
LAPACK distribution; either upstream to `prismqueer` or inline the
FFI declaration in `bootstrap/src/lapack_q4.rs`. Zero new build
configuration required (Accelerate framework already linked).

**Missing shards:** `shards/reality/algebra/silicon.mirror` (the
silicon species; forward-promised at
`shards/reality/algebra.mirror` line 299); `shards/glue/
math_silicon.mirror` (the math ↔ silicon @glue species; NOT
enumerated at `shards/glue.mirror` — needs to be added to the
species roster too); optionally
`shards/reality/algebra/math/cholesky.mirror` (the theorem
content shard).

**Surprises about math ↔ silicon readiness:**

1. **The infrastructure is complete.** LAPACK FFI runs. Content-
   addressed persistence runs. All that's missing is the composition
   test that would tie them together.
2. **The canonical spec's own language is more honest than the claim.**
   `805676e` §5.3 says "the discharge is operational at the project
   level; the substrate-decl form is forward-promised". The "already
   empirically operational as first crystal" phrasing in the closure
   sections (§10) is slightly stronger than the body text supports.
   Status-drift instance #4 caught.
3. **Under one tick to close.** ~530 lines across five files gets
   the "operational as first crystal" claim to substrate-pull-honest.
   Reed can dispatch this immediately after this spec lands.

---

*End of scope-verification spec. Two questions discharged. The
composition test is the empirical discharge target for the next
TDD-paired tick — RED first, per [[feedback-always-tdd-no-shortcuts]].
When the test passes, `805676e` §5.4's math ↔ silicon criterion IS
operational as first crystal, in the strict content-addressed sense
the canonical spec's v1.0 criterion requires.*
