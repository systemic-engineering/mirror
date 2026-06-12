# CoincidenceHash collapse — map the two parameterizations, propose unification, fix Splinter Oid

---

## SUPERSEDED 2026-05-30

**This spec's RECOMMENDATION (the CHC collapse plan that unifies `<3>` and `<5,5>`
into a single canonical CoincidenceHash backing `@mirror/store`) is SUPERSEDED.**
The map, the usage audit, the diagnosis, and the existing-CoincidenceHash detail
below (§2–§4) all remain accurate as a historical / structural record. Only the
conclusion changed.

What happened: same day this spec landed (commit `e9c259b`, 2026-05-30), Reed and
Alex continued the conversation. The upstream framing spec
(`docs/specs/spectral-hash-design.md`, commit `c3a01e3`) recommended a composite
`ContentOid { storage, navigation }`. That recommendation in turn got LRM-collapsed
the same day into the **generic-over-hash cascade**:

1. **`@mirror/store`** is the open content-addressed storage gate. It uses
   `Merkle<BLAKE3>` — NOT CoincidenceHash. Standard, fast, no float dependency,
   sidesteps Attack 1 entirely. mirror MUST work without `@spectral/db`.
2. **`@spectral/db`** is the engine on top (potentially closed source). Its
   navigation primitive is **`VoidPointer`** — the spectral coordinate that
   `SpectralCoordStore` + `coord_oids` + `spectral_distance_eigen` already compute,
   renamed. NOT a hash function; a coordinate.
3. **The Merkle tree is generic over the hash algorithm.** `Splinter<H>`,
   `Content<H>`, `Body<H>`, `Crystallization<H>`, `Crystallizations<H>` (renamed
   from `Registry`). Hash-blind types (`Ref`, `CrystallizeError`) stay concrete.
   Each consumer picks its own `H`.
4. **Verification belongs to `@mirror/store`**, not `@spectral/db` (correcting
   c3a01e3's framing on the way through).

**Consequence for CHC:** the collapse plan in §7 (CHC-1 through CHC-5) is
**obsolete** in its original form. The consumer (storage) that motivated unifying
`<3>` and `<5,5>` has moved away — `@mirror/store` is no longer a CoincidenceHash
consumer. The two existing CoincidenceHash sites (`prism_core::Detector<3>` and
`bootstrap::canonical_hash` `<5,5>`) stay where they are; whether they ever unify
is a separate, now lower-priority concern. If they outlive their callers entirely,
they get retired in a future hygiene tick.

**Cross-references for the current architecture:**

- `docs/specs/store-vs-db-and-the-cascade.md` — the LRM-collapsed architecture,
  the landing-page spec to read first.
- `docs/specs/spectral-hash-design.md` — the upstream framing spec, also amended
  2026-05-30 with a top-of-file rewrite banner. §4–§5's research (LSH, spectral
  hashing, Motwani-Naor-Panigrahy impossibility) stands; the recommendation in §6
  is rewritten.
- `docs/specs/kintsugi-minimum-runnable.md` — carries the cascade renames
  (`Registry` → `Crystallizations`, `ActionPath` → `Ref`) in its amendment section.

The rest of this spec is preserved as written. Treat §2–§4 as the substrate-pull
map (still accurate); treat §5–§8 as the historical proposal (the recommendation
is obsolete; the analysis is not).

---

**Status:** draft — substrate-pull spec. Read-only investigation +
proposed migration ticks. Authored by Mara on `mara/shard-chain` after
Alex pushed back on a Tick A architectural call that Reed and I made
*without reading the code*. The corrective is this map.

**Scope:** the three content-addressing primitives that currently live
in the workspace —
`prism_core::coincidence::Detector<N>` (the `<3>` site),
`bootstrap::hash::canonical_hash` (the `<5,5>` site), and
`bootstrap::crystallize::Oid` (the raw-SHA-256 Splinter site landed in
Tick A) — plus every callsite of each.

**Touches no `.rs`.** Markdown only. Migration is staged as 🔴/🟢
ticks in §7; this spec is the map, not the migration.

---

## 1. Motivation — own the gap honestly

This spec exists because Mara and Reed produced an architectural call
without reading the code.

In Tick A (`mirror` commit `8f710ce`) I implemented `crystallize::Oid`
as a raw 32-byte SHA-256 digest of a Merkle node, **deliberately not
reusing** `prism_core::Oid` (which I described as "wraps
`CoincidenceHash<3>`") or `bootstrap::hash::canonical_hash` (which I
described as "`CoincidenceHash<5,5>` — the spectral-triple Dirac
action on AST nodes"). The justification carried into the module
docstring at `bootstrap/src/crystallize.rs#83-91`:

> The coincidence hash is the *spectral-triple Dirac action* on AST
> nodes — its inputs are eigenvalue-projected vectors in a 5-d basis,
> which is the wrong altitude for self-similar content addressing.
> Splinter gets its own Merkle SHA-256 — minimal floor, no overlap.

Reed echoed the framing. Alex pushed back:

> *"maybe you and Mara should've read the code before confidently
> producing an answer."*

The "different altitudes" framing is principled-sounding language for a
position that was never grounded in code. It survives this spec only
in the places where the code *actually* supports it. Where the code
does not support it, I name that explicitly and walk it back.

This spec is the map + the proposed collapse + Splinter's corrected
Oid. The discipline is named in the artifact because the violation
was the artifact.

---

## 2. The mapping — what is each, exactly

### 2.1 `prism_core::coincidence::Detector<N>` — the prism-core primitive

**File:** `/Users/alexwolf/dev/projects/prism/core/src/coincidence.rs`

**Type signature:**

```rust
pub struct Detector<const N: usize> {
    projections: Vec<Projection>,
    space: String,
}

impl<const N: usize> Detector<N> {
    pub fn canonical(space: impl Into<String>, dimension: usize) -> Self { ... }
    fn detect(&self, data: &[u8]) -> DetectionResult { ... }
}

impl<const N: usize> HashPrism for Detector<N> {
    type Input = [u8];
    type Output = String;
    fn review(&self, input: &[u8]) -> String { ... } // -> 64-char hex
}
```

**The shape:** `N` is a compile-time const generic (number of
independent projection observers). `dimension` is a *runtime* argument
to `Detector::canonical`. `space` is a *runtime* string label
(`"content"` in the canonical detector). Internally, projections store
entries in `BTreeMap<(String, String), f64>` keyed by stringified
`"d0"`, `"d1"`, …, `"d{dimension-1}"`. State vectors are sparse
`BTreeMap<String, OrderedF64>`.

**What goes in:** raw bytes.

**What comes out:** a 64-char hex string. Produced by:

1. Encode bytes into a `StateVector` over `{d0, …, d{dimension-1}}`
   via per-byte SHA-256 seeded random projection (`encode_into_basis`).
2. For each of `N` projections (whose entries are derived from
   seed `"coincidence:projection:{i}:{N}"`), apply to the state vector
   to get `focus_sv`.
3. Concatenate `b"coincidence:" || (N as u64).to_le_bytes() ||
   for each projection: dense_bytes(focus_sv)` into
   `eigenvalue_bytes`.
4. Final hash = `SHA-256(b"prism-core:coincidence:" ||
   eigenvalue_bytes)`, hex-encoded.
5. Dark fallback if state is zero or any projection collapses: `SHA-256(b"prism-core:dark:" || input)`, hex-encoded.

**The canonical instance:**

```rust
static CANONICAL: LazyLock<Detector<3>> =
    LazyLock::new(|| Detector::canonical("content", DEFAULT_DIMENSION));
```

where `DEFAULT_DIMENSION = 16`. So the canonical detector is
`Detector<3>` with `dimension=16`, `space="content"`. **Reed described
this as "CoincidenceHash<3>".** The notation is wrong in two ways:
(a) there is no `<N,M>` type-level parameterization here — only `<N>`,
with dimension as a runtime constructor argument; (b) the canonical
detector's full shape is `<3>` *plus dimension=16 plus space="content"*
at runtime. "`CoincidenceHash<3>`" elides the runtime config.

**Sha256 extractability:** the SHA-256 final-compression step is
logically separable (`HashPrism::review` does `Sha256::new() ||
b"prism-core:coincidence:" || hex::decode(eigenvalue_hex)`), but the
*input* to that SHA-256 is the projected eigenvalue bytes — there is
no path that bypasses the projection. The detector IS the projection;
the SHA-256 is its post-compression to a fixed-width address.

### 2.2 `bootstrap::hash::canonical_hash` — the bootstrap primitive

**File:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/hash.rs`

**Type signatures (there is no `CoincidenceHash` *type* in the code):**

```rust
pub const DIM: usize = 5;
pub const NUM_PROJECTIONS: usize = 5;
pub const EPSILON: f64 = 2.2204460492503131e-16;
pub const LEX_ORDER: [usize; DIM] = [0, 1, 2, 3, 4];

pub struct Projection {
    pub entries: [[f64; DIM]; DIM],     // dense 5×5
    pub present: [[bool; DIM]; DIM],
}

pub fn canonical_hash(data: &[u8]) -> String { ... }   // -> 64-char hex
pub fn hash_tagged(tag: &str, content: &[u8]) -> String { ... }
```

**The shape:** there is **no `CoincidenceHash<N,M>` type** in
`bootstrap/src/hash.rs`. The dimensions are module-level constants.
The doc-comment at the top calls this "CoincidenceHash<5,5>" —

> ```rust
> //! SHA-256 helpers + CoincidenceHash<5,5>.
> //!
> //! This is the Cluster D rewrite. The bootstrap now implements the geometry
> //! declared in `boot/std/hash/coincidence.mirror`:
> //!
> //!   DIM = 5             — one dimension per Prism operation
> //!   NUM_PROJECTIONS = 5 — one projection per gutter-lens duality
> ```

— but no `<N,M>` parameter appears anywhere in the source.

**What goes in:** raw bytes.

**What comes out:** a 64-char hex string. The structure mirrors
`Detector<5>::canonical(_, 5)` byte-for-byte conceptually, with three
concrete differences:

1. **Dense arrays instead of sparse maps.** `[f64; 5]`, `[[f64; 5]; 5]`
   instead of `BTreeMap<String, OrderedF64>` and
   `BTreeMap<(String, String), f64>`.
2. **No string `space` parameter.** Implicit space is the bootstrap's
   AST altitude; not labelled.
3. **`LEX_ORDER` is identity for 5 elements** — the order constant is
   `[0,1,2,3,4]`. The prism-core sparse BTreeMap variant achieves the
   same canonical traversal via its key-ordered iteration over
   `"d0".."d{n-1}"`.

The seed format `"coincidence:projection:{i}:{NUM_PROJECTIONS}"`
matches `Detector::canonical`'s seed format byte-for-byte (line 264
of `coincidence.rs`: `format!("coincidence:projection:{i}:{N}")`).
The `b"prism-core:coincidence:"` and `b"prism-core:dark:"` final-stage
tags match. The encode-into-basis loop matches up to the
dense-vs-sparse rep.

**This is the same algorithm with different default parameters and
different storage representation.**

**Sha256 extractability:** same as §2.1. The final `SHA-256(b"prism-core:coincidence:" || eigenvalue_bytes)` step is the only line that calls `Sha256::new` for the live path; the eigenvalue projection cannot be bypassed.

### 2.3 `prism_core::Oid` — the OID wrapper

**File:** `/Users/alexwolf/dev/projects/prism/core/src/oid.rs`

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Oid(String);

impl Oid {
    pub fn hash(bytes: &[u8]) -> Self {
        let hex_str = crate::coincidence::canonical_hash(bytes);
        Oid(hex_str)
    }
}
```

**The shape:** thin newtype around a `String`, where the string is the
64-char hex output of `prism_core::coincidence::canonical_hash` —
i.e., the canonical `Detector<3>` with `dimension=16` and
`space="content"` from §2.1.

**Reed's prior framing was "`prism_core::Oid` wraps
`CoincidenceHash<3>`".** This is almost right: the wrapped string
does come from `Detector<3>`. But the docstring on `Oid::hash` is also
fuzzy:

> ```
> /// Uses CoincidenceHash<3> — three independent projection observers
> /// in a 16-dimensional space. The shared eigenvalue becomes the content
> /// address, compressed through SHA-256 to a fixed 64-char hex string.
> ```

The "`CoincidenceHash<3>`" notation is mirror-internal shorthand for
"the `Detector<3>` canonical instance"; the runtime
`(space="content", dimension=16)` config is part of the identity.

### 2.4 `bootstrap::crystallize::Oid` (Tick A) — Splinter's address

**File:** `/Users/alexwolf/dev/projects/mirror/bootstrap/src/crystallize.rs`

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Oid([u8; 32]);

fn compute_oid(content: &Content) -> Oid {
    let mut h = Sha256::new();
    match content {
        Content::Text(t)    => { h.update(b"T"); h.update(u64_le(...)); h.update(...) }
        Content::Record(m)  => { h.update(b"R"); h.update(u64_le(...)); for (k, sub) in m { ...; h.update(sub.oid().bytes()) } }
        Content::List(it)   => { h.update(b"L"); h.update(u64_le(...)); for sub in it { h.update(sub.oid().bytes()) } }
    }
    let digest = h.finalize();
    Oid(...)
}
```

**The shape:** raw 32-byte SHA-256 over a Merkle-style framing. Each
level hashes the per-shape tag (`b"T"`/`b"R"`/`b"L"`) + length prefix +
shape-specific body that *includes children's OIDs rather than
children's content*. The Merkle structure is the load-bearing
property for fragmentation; the underlying hash is plain SHA-256.

**There is no coincidence/eigenvalue projection here at all.** This is
the call I made in Tick A, and the call this spec re-examines.

### 2.5 `bootstrap::hash::hash_tagged` — the tagged wrapper

Wrapper around `canonical_hash`:

```rust
pub fn hash_tagged(tag: &str, content: &[u8]) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(tag.len() + 1 + content.len());
    buf.extend_from_slice(tag.as_bytes());
    buf.push(b':');
    buf.extend_from_slice(content);
    canonical_hash(&buf)
}
```

Not a separate primitive — just `canonical_hash(tag || ":" || content)`.

### 2.6 The grammar declaration

**File:** `/Users/alexwolf/dev/projects/mirror/boot/std/hash/coincidence.mirror`

Declares the `@hash/coincidence` grammar with `dim=5`, `projections=5`,
the `dimension` and `duality` enums, the seed format, and the live /
dark tags. The grammar names the shape; the bootstrap `hash.rs`
realizes it. The grammar is consistent with `bootstrap/src/hash.rs`
but **not** consistent with `prism_core::coincidence`, which uses
`<3,16>`. The grammar says "this is the canonical address" — and the
bootstrap obeys; prism-core diverges.

### 2.7 What is NOT a primitive

- **`SpectralOid`** (`prism_core::spectral_oid::SpectralOid`) is a
  *truncation policy* on top of a raw OID string — it's a precision
  lens, not a hash function. Not relevant to this collapse.
- **`MerkleTree`** in `prism_core::merkle` is a tree of OIDs, not a
  hash primitive. Composes the addresses produced by §2.1.
- **`spectral.rs`'s `eigen_d`** is a power-iteration eigendecomposition
  routine over a *runtime-supplied* matrix; it does not itself compute
  a hash. The docstring claims "For mirror's `CoincidenceHash<5,5>`
  the operator is a 5×5 symmetric matrix in the canonical basis" but
  there is **no callsite in the bootstrap that feeds the
  `canonical_hash` projection matrix into `eigen_d`**. The 5×5
  scaffold is documentary; the spectral analysis path is unused at
  the bootstrap altitude today.

---

## 3. The usage audit — every callsite

Grouped by what is structurally similar. Rows are individual call
sites unless multiple identical calls in one function were collapsed.

### 3.1 `prism_core::Oid::hash` (→ `Detector<3>` `(dim=16, space="content")`)

| Caller | Parameterization | What it hashes | Why |
|---|---|---|---|
| `prism/core/src/oid.rs::Oid::hash` | `Detector<3>` canonical | arbitrary bytes, the public entrypoint | the substrate's content-address primitive |
| `prism/core/src/coincidence.rs::Detector<N> as Addressable` | `Detector<N>` self | `b"detector:{N}:{space}"` | detector's own identity OID |
| `prism/derive/src/lib.rs::derive(Prism)` | `Oid::hash` | `@name.as_bytes()` of the `#[oid("@…")]` attribute | every derive-generated `Addressable` impl |
| `prism/derive/src/lib.rs::derive(DeriveLambda)` | `Oid::hash` | `@name.as_bytes()` | every derive-generated `Lambda::abs(oid, ...)` |
| `prism/core/src/named.rs::tests` (benchmark only) | `Oid::hash` | 1k iterations | perf sanity for derive macros |
| `prism/core/tests/lambda_integration.rs` | `Oid::hash` (via derive) | `@parse` etc. | integration test |
| **spectral-db** seam — `Oid::hash` used by `make_ref` and `content_oid` in different ways | `Oid::hash` | shard content | per the seam finding in `spectral-db/tests/bootstrap.rs#25`: `make_ref` uses `sha::hash`, `content_oid` uses `Oid::hash` — *the spectral-db codebase already documents a parallel inconsistency* |

Note on the spectral-db finding: the bootstrap `tests/bootstrap.rs`
string corpus contains

> *"make_ref and content_oid use DIFFERENT hash functions. make_ref()
> calls sha::hash to produce a Ref (fragmentation SHA), while
> content_oid() calls Oid::hash which uses prism canonical_hash
> (CoincidenceHash<3>)."*

This is the same family of inconsistency we're documenting here at a
different altitude. The collapse this spec proposes does not directly
resolve spectral-db's seam, but it makes the canonical address scheme
unambiguous so downstream crates have one primitive to choose, not
three.

### 3.2 `bootstrap::hash::canonical_hash` (→ dense `<5,5>`)

| Caller | Parameterization | What it hashes | Why |
|---|---|---|---|
| `bootstrap/src/main.rs:cmd_compile` (3 call sites) | `<5,5>` | raw source bytes of a `.mirror` file | source-OID cache key for the git-crystal store |
| `bootstrap/src/main.rs:cmd_compile` (corpus aggregate) | `<5,5>` | per-file OIDs concatenated | top-level `crystal` digest for a multi-file build |
| `bootstrap/src/spectral.rs:compute_oid_inner` (every `AstKind` branch) | `<5,5>` (via `hash_tagged`) | AST kind tag + name + body + child-OIDs | content-OID of an AST node |
| `bootstrap/src/spectral.rs:combinator_tree_oid_hex` (every `Combinator` variant) | `<5,5>` (via `hash_tagged`) | combinator variant tag + body | content-OID of a parser combinator tree |
| `bootstrap/tests/oid_smoke.rs` pinned values | `<5,5>` | small test inputs | byte-stability pins |

**Observation:** `bootstrap::hash::canonical_hash` is the bootstrap's
entire content-addressing world — *for source bytes, AST nodes, and
combinator trees*. The 5×5 is not specialised to one altitude; it's
the bootstrap's only address-computing primitive. The same `<5,5>`
that hashes raw source text also hashes a single-byte tag literal.

**There is no callsite of `prism_core::Oid::hash` inside the bootstrap
crate.** The bootstrap reaches for `bootstrap::hash::canonical_hash`
for every content address. (`prism_core::Oid` *the type* is in scope
via `prism_core::Oid` import — but `crystallize.rs`'s OID is
`crystallize::Oid` (a different newtype, see §2.4), and AST OIDs are
plain `String` returned by `compute_content_oid`. The bootstrap does
not use `prism_core::Oid` as a value at all today.)

### 3.3 `bootstrap::crystallize::Oid` (Tick A, raw SHA-256)

| Caller | Parameterization | What it hashes | Why |
|---|---|---|---|
| `bootstrap/src/crystallize.rs::compute_oid` | raw SHA-256 | Splinter Merkle node | the Tick A call |
| `bootstrap/src/crystallize.rs::tests` | raw SHA-256 | test Splinters | Tick A test coverage |

One caller. New as of `8f710ce`. This is exactly what the spec
revisits.

### 3.4 What's not called by anyone

- `prism_core::coincidence_hash()` (the `Named<Detector<3>>` wrapper)
  has no in-tree callers outside its own tests. Substrate-facing
  surface only.
- `Detector::to_metal` is a documentary path for compiling the
  coincidence projection to the Metal Prism instruction stream;
  tests-only callers today.
- `eigen_d` in `bootstrap/src/spectral.rs` has tests-only callers.
  The 5×5 matrix it consumes is constructed inline in tests; the
  bootstrap never feeds `canonical_hash`'s projection matrix into it.

---

## 4. Principled vs evolutionary divergence

### 4.1 The diagnosis

**Evolutionary drift, dressed in principled language after the fact.**

The code shows:

1. `prism_core::coincidence::Detector<N>` was ported from
   `coincidence` crate at `prism/core/src/coincidence.rs#1-6`:
   > ```
   > //! Minimal coincidence hash — eigenvalue-based content addressing.
   > //!
   > //! Ported from the coincidence crate. This is the minimal code path:
   > //! bytes -> StateVector -> N projections -> Detection -> eigenvalue hex.
   > //!
   > //! N=3 is the canonical detector for Oid::hash(). Three independent observers,
   > //! deterministic projections from SHA-256 seeds.
   > ```
   The `<3, dim=16>` choice is inherited from the source crate's
   defaults. There is no document in `prism/core/` that derives
   `N=3` or `dim=16` from a principled position. The detector is
   parameterized to allow any `<N>` and any dimension; the *canonical*
   instance is `<3>, dim=16` by historical accident of the port.

2. `bootstrap/src/hash.rs` was rewritten in Cluster D from a prior
   C-era `<3,16>` implementation to `<5,5>` — but only in the
   bootstrap. The bootstrap docstring confirms (`hash.rs#1-21`):
   > ```
   > //! This is the Cluster D rewrite. The bootstrap now implements the geometry
   > //! declared in `boot/std/hash/coincidence.mirror`:
   > //!
   > //!   DIM = 5             — one dimension per Prism operation
   > //!   NUM_PROJECTIONS = 5 — one projection per gutter-lens duality
   > ```
   The grammar `boot/std/hash/coincidence.mirror` declares the `<5,5>`
   geometry as the *canonical* coincidence hash; the prism-core
   `<3,16>` was the pre-rewrite shape that survived in prism-core
   because nobody updated it after Cluster D.

3. The bootstrap's `oid_smoke.rs` tests confirm the pinned `<5,5>`
   values are the post-rewrite reference. The pinned
   `<3,16>` values from the pre-rewrite era are explicitly gone
   (`oid_smoke.rs` docstring at top: "the earlier pre-Cluster-D values
   were computed under CoincidenceHash<3,16>, the C-era seed; they
   are gone").

4. **The grammar already declares the bootstrap's `<5,5>` is
   canonical.** Per `boot/std/hash/coincidence.mirror`:
   > ```
   > # CoincidenceHash<5,5>. The hash IS the Void geometry:
   > #   5 dimensions — one per operation (focus, project, split, shift, settle)
   > #   5 projections — one per duality lens (entropy, spectral, cheeger, ricci, mixing)
   > #   λ₀ = 0       — the dark fallback. Void. The generative zero.
   > ```
   The 5×5 is grounded in the Prism shape (5 operations) and the
   gutter-lens dualities (5 dualities). This is principled. The
   `<3,16>` in prism-core has no equivalent grounding — it's the
   ported default.

5. **`eigenboard-representation.md` agrees** at §644-670:
   > ```
   > ### 5×5 conductivity tensor → connection matrix
   > `@hash/coincidence`'s tensor is the matrix representation of the
   > bundle's connection in the canonical basis.
   > ```
   The 5×5 is the conductivity tensor. The prism-core `<3,16>` is not
   mentioned in any structural document.

### 4.2 What this means for the collapse

The code shows two implementations of *the same algorithm* with
different default parameters. The bootstrap's `<5,5>` is the post-
rewrite, grammar-declared canonical shape; prism-core's `<3,16>` is
the pre-rewrite default that nobody updated.

**Reed's claim that the two are "AST-altitude Dirac vs 5×5
conductivity tensor" is wrong on the AST-altitude half.** The AST
altitude content-OID is computed by the *bootstrap's* `<5,5>`, not by
prism-core's `<3>`. The prism-core `<3>` is consumed by `Oid::hash`
which is the *substrate-side* primitive — used by `derive(Prism)`
generated impls and any prism-substrate code that wants a content
address (today: spectral-db's `content_oid` per the seam finding,
future: any prism-core consumer not in the bootstrap). The two
primitives are not separated by altitude in any clean sense; they are
separated by *crate boundary*, which is evolutionary.

**My Tick A justification for Splinter's parallel scheme is also
inaccurate on the same axis.** I wrote in `crystallize.rs#83-91`:

> The coincidence hash is the *spectral-triple Dirac action* on AST
> nodes — its inputs are eigenvalue-projected vectors in a 5-d basis,
> which is the wrong altitude for self-similar content addressing.

The bootstrap's `<5,5>` *does* compute content addresses for AST
nodes (per §3.2) — and not via a vector-input path. The input is
bytes; the eigenvalue projection is internal. Splinter is a Merkle
structure over its children, where each level needs to hash *bytes*
(the tag, the length, the children's OIDs) to produce a fixed-width
address. That is exactly what `canonical_hash` does. The
"wrong altitude" framing was rationalization for keeping Splinter
simple at Tick A — which is a real concern (see §6 on the migration
shape) but not a structural separation from `canonical_hash`.

**The collapse is justified.** Two parameterizations of the same
primitive, one declared by the grammar as canonical, the other a
legacy default in a separate crate.

---

## 5. The collapse — propose the unified primitive

### 5.1 Which shape survives

**The bootstrap's `<5,5>` survives as the canonical coincidence-hash
parameterization.** Two reasons grounded in code + spec:

1. The grammar `boot/std/hash/coincidence.mirror` declares `<5,5>` as
   canonical. The substrate-pull discipline says when the grammar
   declares a shape, the Rust converges to the grammar, not vice
   versa. `prism-core`'s `<3,16>` is the pre-rewrite default that
   nobody migrated.
2. `eigenboard-representation.md` couples the 5×5 to the Prism
   trait's five operations and the gutter-lens dualities. The
   `<3,16>` has no equivalent structural justification.

`Detector<N>` with `N` as a const generic and `dimension` as a runtime
argument **stays as a substrate-level abstraction** — the
coincidence-detector primitive is still useful at other parameterizations
for research and for the metal-compilation path (`to_metal` works
for any `N`). What changes is the **canonical** instance:

```rust
// before
static CANONICAL: LazyLock<Detector<3>> =
    LazyLock::new(|| Detector::canonical("content", 16));

// after
static CANONICAL: LazyLock<Detector<5>> =
    LazyLock::new(|| Detector::canonical("content", 5));
```

or — preferable per the next paragraph — `Oid::hash` delegates to a
named `coincidence_hash_5x5` that's a dense `<5,5>` implementation,
leaving the generic `Detector<N>` available for non-canonical use.

### 5.2 Which implementation backs the canonical address

The bootstrap's dense `<5,5>` is byte-stable and matches the grammar
byte-for-byte (the `oid_smoke.rs` pins). The prism-core sparse
`Detector<N>` produces a different byte sequence at `<5,5>` because:

- it uses `BTreeMap` keyed by `"d0".."d4"` strings, so `dense_bytes`
  iteration is by alphabetical key order, which for `"d0"`..`"d9"` is
  identical to numeric order. ✓
- it uses `i.to_le_bytes()` for the index where the bootstrap uses
  `u64_le(i, ...)` packing. Both are 8-byte little-endian; same bytes. ✓
- it omits *near-zero* entries from the `BTreeMap` when constructing
  projections, so the dense-bytes output of `focus_sv` reconstructs
  zeroes for missing labels. The bootstrap's dense arrays carry
  explicit zeroes at the same positions. **Same bytes by case
  analysis — but only if the EPSILON thresholds match exactly.**
  The bootstrap uses `EPSILON = 2.2204460492503131e-16` (machine
  epsilon); prism-core uses `f64::EPSILON` (same value). ✓

**Plausibly byte-stable across the two implementations at the same
parameters.** A byte-equality round-trip test on a small corpus is
required before the migration commits (this is a Tick B gate, named in
§7).

If the round-trip test passes: collapse `bootstrap/src/hash.rs` to a
thin shim that delegates to `prism_core::canonical_hash` (with the
canonical detector now at `<5,5>`). One implementation, two callsites
resolved.

If the round-trip test fails (some subtle byte-skew between dense and
sparse rep): keep the bootstrap's dense `<5,5>` as the canonical
implementation, and update prism-core's `Detector::canonical` to call
the bootstrap's `canonical_hash` via a re-export through a shared
crate (or invert the dependency so prism-core has its own dense
`<5,5>` path and the bootstrap delegates). Either way: one byte-stable
implementation, not two.

**Open decision for Alex** (§8): which crate hosts the unified
implementation. Prism-core is the lower-level crate; the bootstrap
depends on prism-core, not vice versa. Moving the canonical to
prism-core feels structurally right (the substrate hosts the
substrate-level primitive). But the grammar that declares `<5,5>`
lives in `boot/std/hash/coincidence.mirror`, which is mirror-internal;
until that grammar is exported as a prism-core idea, the substrate-
side `Detector` doesn't know about the `<5,5>` defaults.

### 5.3 What `Sha256` falls out as a separable primitive

The final-stage `SHA-256(b"prism-core:coincidence:" || eigenvalue_bytes)`
and `SHA-256(b"prism-core:dark:" || data)` are the only two places
the coincidence hash uses raw SHA-256. They're inside the canonical
hash pipeline; not separately exported.

**A separately useful primitive falls out:** plain Merkle SHA-256 over
byte-prefixed framings — exactly what `crystallize::compute_oid` does
today. This is needed by Splinter (see §6) and is the `sha::hash`
shape that spectral-db's `make_ref` uses. Naming it explicitly —
`bootstrap::hash::sha256_merkle` or moving to a shared
`prism_core::merkle_hash` — is a low-risk extraction.

---

## 6. Splinter's Oid in the collapsed world

This is the operational reason this spec exists. Re-examining the
Tick A call against the collapsed primitive.

### 6.1 The three options Reed surfaced

**(a)** Splinter wraps the shared coincidence hash (`canonical_hash`).
Cross-wall verification works by construction because Splinter's OIDs
are in the same address space as every other AST/source/combinator
OID.

**(b)** Splinter and substrate share a primitive but spectral-typed
wrappers diverge. Splinter has `crystallize::Oid` wrapping the same
bytes as `prism_core::Oid` but typed differently to enforce a domain
boundary; cross-wall verification needs a typed bridge.

**(c)** The split is genuinely load-bearing: Splinter needs raw
SHA-256 Merkle, not the coincidence hash, for code-level reasons that
the coincidence projection breaks.

### 6.2 What the code supports

**(c) is the strongest case.** Splinter's defining property is
self-similar Merkle: each level hashes its children's *OIDs*, not
their content. The OID of a Record is `SHA-256("R" || len(m) ||
for each (k, sub): len(k) || k || sub.oid().bytes())` — and
`sub.oid().bytes()` is required to be a **fixed-width byte sequence**
that unambiguously identifies the child. The crystallize Tick A
spec'd 32 bytes per child OID.

`canonical_hash` returns a 64-char hex *string*. Decoded, it's 32
bytes. So the byte-width match holds. But the input domain of
`canonical_hash` is `&[u8]` bytes, and the output domain is the
64-char hex address string. Splinter's Merkle parent needs to read
child OIDs as 32 raw bytes (not 64 hex chars) so the parent's hash
input is dense. Either:

- **Splinter uses `canonical_hash`** and the Merkle parent input is
  `hex::decode(child.oid().as_str())` — adds a hex<->bytes roundtrip
  per child, but works.
- **Splinter uses raw SHA-256 Merkle (Tick A status quo)** and
  cross-wall verification needs a bridge: a Splinter that wants to
  prove its identity against an AST OID has to compute its content
  via `canonical_hash` separately.

**(c) survives partially.** Splinter genuinely needs a Merkle-shaped
hash where children's OIDs are the *direct* input to the parent's
hash. The coincidence projection is byte-input, byte-output; it does
not pass children's OIDs through unchanged. To compose Splinters into
larger Splinters under `canonical_hash` you'd be projecting children's
OID bytes through the coincidence detector at every level, which is
not what Merkle wants (Merkle wants the parent hash to be a function
of the children's hashes byte-for-byte). The coincidence hash is the
right primitive for hashing *content* into an address; raw SHA-256 is
the right primitive for composing *addresses* into a parent address.

These are different jobs. The Tick A call was directionally right, for
the wrong reason.

### 6.3 The corrected framing for Tick A

Replace the docstring at `bootstrap/src/crystallize.rs#83-91`:

```text
## Adaptation note — Splinter OID source

The spec assumes plain `Sha256` for Splinter's Merkle OID computation
(a standard content hash, distinct from the bootstrap's existing
`canonical_hash` / `CoincidenceHash<5,5>` in `hash.rs`). The
coincidence hash is the *spectral-triple Dirac action* on AST nodes
— its inputs are eigenvalue-projected vectors in a 5-d basis, which
is the wrong altitude for self-similar content addressing. Splinter
gets its own Merkle SHA-256 — minimal floor, no overlap.
```

with:

```text
## Adaptation note — Splinter OID source

Splinter's OID is raw SHA-256 in a Merkle framing (per-shape tag +
length + children's OIDs-as-bytes). This is structurally different
from `canonical_hash` (the @hash/coincidence shape), which projects
byte input through the 5×5 conductivity tensor before SHA-256
compression. The two primitives do different jobs:

- `canonical_hash` hashes *content* (raw bytes) into an address.
- Splinter's `compute_oid` composes *child addresses* into a parent
  address.

For Splinters whose leaves carry content (`Content::Text`), the
leaf's hash input is the literal text bytes — coincidence-projection-
free by design, because Splinter is not a content-OID; it's a
Merkle-composition OID. A separate path (e.g. `splinter_with_content_oids`)
can compute coincidence-OIDs for the leaves if cross-wall
verification against `canonical_hash` is needed.
```

### 6.4 What changes in `bootstrap/src/crystallize.rs` after the collapse

Minimal. The Tick A surface stays:

- `Oid([u8; 32])` — unchanged. The 32-byte width is the Merkle
  framing's child-address payload; matches the post-decode width of
  `canonical_hash` output too.
- `compute_oid` — unchanged on the algorithm; docstring corrected
  per §6.3.
- New: a method `Splinter::content_oid(&self) -> String` that returns
  the coincidence-OID of the Splinter's flattened content bytes. This
  is the cross-wall bridge: any Splinter can prove its identity
  against any other address scheme by exposing its content under
  `canonical_hash`. Implementation is a flattening serializer
  (canonical encoding of `Content` → bytes) plus `canonical_hash`.
  Body is small; tick is short.

**My original Tick A call survives the read** for the structural
reason (Splinter is Merkle-composition, not content-hash), **but not
for the reason I gave**. The framing in the original docstring is
incorrect; the structural conclusion is correct.

---

## 7. Migration ticks

Ordered. Each tick is a single commit with marker. No tick reaches
for non-`.md` files; this spec stays markdown-only.

### Tick CHC-1 — 🔴 byte-stability test for `<5,5>` round-trip

**Scope:** `bootstrap/tests/coincidence_collapse.rs` (new). A failing
test that constructs
`prism_core::coincidence::Detector::<5>::canonical("content", 5)` and
asserts byte-equality with `bootstrap::hash::canonical_hash` for a
small fixed corpus (the `oid_smoke.rs` test inputs plus a few edge
cases — empty, single-byte, dark-fallback trigger).

**Marker:** 🔴 — the test fails today because prism-core's canonical
is `<3, dim=16>`, not `<5,5>`. The failure shape proves the
divergence is mechanical.

**Why first:** §5.2 hinges on byte-stability between dense and sparse
implementations. If the round-trip is byte-stable, the collapse is
trivial; if not, the collapse needs a single-implementation home
decision (§8). The test answers the question before §8 lands.

**Files touched:** `bootstrap/tests/coincidence_collapse.rs` (new).
Verification: `cargo test --test coincidence_collapse` exits non-zero
with a byte-diff in the failure message.

### Tick CHC-2 — 🟢 update prism-core's canonical to `<5,5>` `[substrate-pull:realize]`

**Scope:**

- `prism/core/src/coincidence.rs`: change `static CANONICAL:
  LazyLock<Detector<3>>` to `LazyLock<Detector<5>>` and the
  `DEFAULT_DIMENSION` constant from 16 to 5 (or pass 5 directly to
  preserve the constant for non-canonical use).
- Update `Detector<3>`-specific tests in `coincidence.rs` to
  `Detector<5>` where they were testing the canonical path; keep
  `Detector<2>`/`<3>` tests as parametric coverage.
- Update `oid.rs` test `oid_hash_cross_version_stable` pinned value
  (the current pin `"08f8e91d23…"` is computed under `<3,16>`; pin
  the new value under `<5,5>`).
- Update derive-macro doc comments referring to `CoincidenceHash<3>`
  to `<5,5>`.

**Marker:** 🟢 — Tick CHC-1's test must now pass.

**Verification:** `cargo test -p prism-core` green;
`cargo test --test coincidence_collapse` green.

**Substrate-pull justification:** the grammar
`boot/std/hash/coincidence.mirror` declared `<5,5>` as canonical; this
tick realizes that grammar in the substrate. AGENTS.md §"Boundary
Rust is not frozen capability" — `[substrate-pull:realize]`.

### Tick CHC-3 — 🟢 collapse `bootstrap::hash::canonical_hash` to delegate `[substrate-pull:realize]`

**Scope:**

- `bootstrap/src/hash.rs`: replace the local `canonical_hash` and
  `hash_tagged` implementations with delegations to
  `prism_core::canonical_hash`. Keep the file as a thin re-export
  surface so existing callers (`bootstrap/src/main.rs`, `spectral.rs`)
  don't change. Optionally retain `DIM`/`NUM_PROJECTIONS`/`LEX_ORDER`
  constants as a compat surface; the grammar `coincidence.mirror`
  exports them and the bootstrap may still want them for diagnostics.
- Update `bootstrap/tests/oid_smoke.rs` pinned values if Tick CHC-1
  surfaced any byte-diff (per §5.2 "if the round-trip test fails").

**Marker:** 🟢 — every existing test stays green.

**Verification:** `cargo test -p mirror` green across the bootstrap
suite; `cargo test --test oid_smoke` green with current or updated
pins; the substrate now has one canonical hash, not two.

### Tick CHC-4 — 🟢 extract `sha256_merkle` and correct Splinter docstring `[substrate-pull:realize]`

**Scope:**

- `bootstrap/src/crystallize.rs`: replace the inline `Sha256` calls in
  `compute_oid` with a named helper (`fn sha256_merkle(parts: ...)`)
  that makes the Merkle-composition shape explicit. The algorithm
  is byte-identical to today; this is a rename + docstring
  correction, not a behavioural change.
- Replace the misleading `## Adaptation note — Splinter OID source`
  docstring per §6.3.
- Add `Splinter::content_oid(&self) -> String` per §6.4 — flatten the
  Splinter to canonical bytes, hash via `prism_core::canonical_hash`.
  Test: a Splinter and its byte-flattened equivalent under
  `canonical_hash` agree. This is the cross-wall verification bridge.

**Marker:** 🟢 — `cargo test -p mirror crystallize::tests` stays
green, plus a new test for `content_oid`.

**Verification:** `cargo test -p mirror crystallize::tests` green;
new `splinter_content_oid_matches_canonical_hash` test green.

### Tick CHC-5 — 📝 doc updates

**Scope:**

- `docs/specs/eigenboard-representation.md`: confirm the
  conductivity-tensor framing is now consistent with one canonical
  hash.
- `docs/specs/kintsugi-minimum-runnable.md` §3.1
  `coincidence_matches`: confirm the property reads the unified
  canonical hash.
- `boot/std/hash/coincidence.mirror`: drop the now-stale comment
  about "the bootstrap currently implements CoincidenceHash<3,16>".
- This spec: mark `Status:` accepted; record the migration outcome.

**Marker:** 📝.

**Verification:** doc-only; no test changes.

---

## 8. Open decisions for Alex

1. **Which crate hosts the canonical implementation post-collapse.**
   §5.2 has two options:
   - Prism-core's `Detector<N>::canonical` becomes `<5>, dim=5`, and
     `bootstrap/src/hash.rs` becomes a thin re-export shim.
   - The bootstrap's dense `<5,5>` stays as the canonical, prism-core's
     canonical changes to delegate (or a shared lower crate hosts the
     primitive — but there isn't one today).
   Prism-core feels structurally right (substrate hosts substrate-
   level primitives), but the grammar that declares the canonical
   shape lives in mirror. Without a shared `boot/std/hash/` export
   mechanism, the substrate doesn't know about the `<5,5>` constraint.
   **Recommendation:** prism-core hosts. The substrate can have a
   canonical without knowing why `<5,5>` is canonical; the why is
   downstream in the grammar.

2. **Whether to keep `Detector<N>` parametric.** The metal-compilation
   path (`Detector::to_metal`) and the spectral-research path
   benefit from `Detector<N>` staying generic. The canonical changes
   to `<5>`; non-canonical use of `Detector<3>` or `Detector<7>`
   remains available. **Recommendation:** keep parametric. The
   canonical is one instance; the primitive is a family.

3. **Whether Splinter's `content_oid` (§6.4) should be a method or a
   free function.** A method couples Splinter to `prism_core` more
   tightly; a free function in a separate module keeps the
   crystallize floor minimal. **Recommendation:** free function in a
   new `bootstrap::splinter_content` module — preserves the floor's
   minimalism, defers the `prism_core` import until the cross-wall
   verification is actually used.

4. **Whether to keep the `<5,5>` notation in docs at all.** It's
   non-standard (`Detector` has `<N>` only; `<5,5>` was Cluster D
   shorthand for `(N=5, dim=5)`). After collapse, the explicit
   form `Detector<5>::canonical("content", 5)` is unambiguous and the
   docs can drop the `<N,M>` shorthand. **Recommendation:** drop the
   `<N,M>` notation in code comments after Tick CHC-3; keep it in
   research docs as historical reference.

5. **Whether to address the spectral-db `make_ref` vs `content_oid`
   seam in the same migration.** Per §3.1 the spectral-db codebase
   already documents that `make_ref` uses `sha::hash` and
   `content_oid` uses `Oid::hash` (= prism-core's coincidence). After
   collapse, both still exist as separate primitives — but their
   purposes are now clearly distinguishable: raw-SHA Merkle for
   composing addresses (§5.3, §6) vs coincidence hash for content
   addressing. The seam might be intentional (analogous to Splinter's
   §6.4). **Recommendation:** out of scope for this spec; surface
   for Reed/spectral-db cycle decision.

---

## Appendix A — Where Reed's prior description was inaccurate

Reed's framing during the Tick A call vs what the code shows:

| Reed said | Code shows |
|---|---|
| `prism_core::Oid` wraps `CoincidenceHash<3>` | Wraps the canonical `Detector<3>`, which has `(N=3, dim=16, space="content")`. The `<3>` shorthand elides the runtime config; the `<3>` is type-level, `dim=16` and `space="content"` are runtime. |
| `bootstrap::canonical_hash` is `CoincidenceHash<5,5>` (5×5 conductivity tensor) | Algorithmically yes, but there is no `CoincidenceHash<N,M>` *type* — `DIM=5` and `NUM_PROJECTIONS=5` are module constants. The 5×5 framing is in the docstring and the grammar, not in a type parameter. |
| The two are at "different altitudes — AST altitude (5×5) vs substrate altitude (3)" | Both compute content-OIDs over byte input. The bootstrap's `<5,5>` IS the AST altitude (used by `compute_oid_inner` for AST nodes — §3.2). Prism-core's `<3>` is *not* the AST altitude — it's the substrate's general-purpose content address, used by `derive(Prism)` and by spectral-db, not by the mirror bootstrap. The altitude separation is misleading; the crate separation is real. |
| The 5×5 is "a spectral-triple Dirac action" | The `eigenboard-representation.md` spec at §644-670 says the 5×5 tensor IS the connection matrix of the eigenboard's bundle; the spectral-triple framing in `spectral.rs` calls `apply_h(&ContentOidPrism, node)` the Dirac action, which is then byte-hashed via `canonical_hash`. The 5×5 matrix shape is real; the "Dirac action" is at one altitude up (the AST recursion is the Dirac action; the hash is its scalar projection). Reed's framing collapsed the two layers. |
| Splinter needs a different primitive because the coincidence hash is "the wrong altitude" | Splinter genuinely needs a different primitive (§6.2) because it's a Merkle-composition hash, not a content hash. The structural reason is right; the "altitude" framing is rationalization. |

My own Tick A docstring carried the "different altitudes" framing
forward into the codebase. §6.3 corrects it.

---

## Appendix B — What this spec did NOT investigate

Named explicitly so future readers don't assume coverage where there
is none:

- The `spectral.rs` `eigen_d` path is unused at the bootstrap altitude
  today (§2.7). Whether it should become live as part of the collapse
  is an open question for the eigenboard-representation track, not
  this spec.
- The spectral-db `make_ref` vs `content_oid` seam (§3.1, §8.5) is
  documented but not resolved here.
- Cosmos and lens crates have no callsites of any of these primitives
  in their current Rust source. If that changes, this spec needs
  re-evaluation; today it does not.
- The `Detector::to_metal` compilation path is touched by the
  canonical change (its output bytes differ between `<3, dim=16>` and
  `<5, dim=5>`), but is unused outside its own tests. Verifying its
  Metal-program shape under the new canonical is a Tick-CHC-2 detail,
  not a separate concern.

---

## Appendix C — Applications: CoincidenceHash as KDF-context

Added 2026-06-12 (Mara). The collapse-plan map above documents
CoincidenceHash as an identity-comparison primitive. The
spectral-db / spectral audit surfaced a second application that
belongs at the same altitude: **CoincidenceHash is a key-derivation
context that makes self-authenticating origin possible.**

### C.1 The construction

The session key for portal-altitude encryption derives from two
inputs composed at KDF time:

```
session_key = KDF(
  shared_secret  = ML-KEM key-encapsulation output,
  context        = coincidence_hash(state)        // <5,5> per §2.2
)
```

The `shared_secret` carries post-quantum confidentiality (the
KEM half). The `context` carries the *geometry of the moment*:
the CoincidenceHash of the eigenvalue state that produced the very
eigenvalues the session is about to encrypt.

### C.2 Why it works — self-authenticating origin

A standard KDF-context binds a session key to a domain string
("application/v1/session"). Substituting `coincidence_hash(state)`
for that domain string upgrades the binding from
*operator-declared* to *substrate-declared*:

- The key is no longer addressable except by an agent that observes
  the same eigenvalue state. The geometry IS the context.
- A compromised session-key reveals exactly one tick's eigenvalues —
  no more. Geometry rotates → hash rotates → key rotates. There is
  no key reuse across geometry-distinct ticks by construction.
- The encryption key is derived from the same hash function that
  addresses the content being encrypted. The same primitive that
  observes the content's identity gates access to it.

This is the property the spectral-db / spectral substrate calls
*self-authenticating origin*. The CoincidenceHash plays both roles
because both jobs need the same observation: which tick this is, in
the canonical 5×5 conductivity-tensor basis.

### C.3 Cross-reference

The §5.3 fall-out — plain SHA-256 Merkle as a separate primitive
for *composing* addresses — applies here too. The KDF context is
exactly the CoincidenceHash's content-addressing job; the Merkle
composition primitive is the wrong shape for it. The two roles
identified in §6 (content vs composition) cleanly partition both
storage and encryption: CoincidenceHash hashes content into
addresses *and* into KDF contexts; SHA-256 Merkle composes
addresses into parent addresses. The collapse plan's `<5,5>`
canonical is what the KDF context names; truncated CoincidenceHash
variants (per the Splinter / VoidPointer precision discussion) are
not appropriate KDF inputs — the KDF context wants the full 64-char
hex / 32-byte address, not a navigation-altitude truncation.
