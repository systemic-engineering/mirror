# Bundle Tower Refactor: mirror → prism-core

The principal bundle tower from differential geometry IS the type hierarchy in
mirror. Each layer wraps a Prism. Each layer adds one concept. `refract` peels
one layer.

```
Layer       prism-core type             what it adds
─────────   ─────────────────────       ──────────────────────
Fiber       P: Prism                    the raw transformation
Connection  Named<P>                    label + OID
Gauge       Crystal<Named<P>>           luminosity (settlement)
Transport   Mirror<P> (crystal + store) persistence
Closure     Shard<P> (mirror + target)  compilation target
```

The six primitives landed in prism-core:
- `Oid`, `Addressable` (oid.rs)
- `Luminosity` (luminosity.rs)
- `Crystal<P>` (crystal.rs)
- `Named<P>` (named.rs)
- `Store` trait (store.rs)
- `Fiber`, `Connection`, `Gauge`, `Transport`, `Closure`, `Bundle` (bundle.rs)

This spec maps each mirror type to its prism-core equivalent and classifies
the migration as Delete, Adapt, or Keep.

---

## Type Mapping

### 1. `mirror::kernel::Oid` → `prism_core::Oid`

**Classification: ADAPT**

Mirror's Oid uses SHA-512 (via the `sha2` crate). Prism-core's Oid uses a
double-hash of `DefaultHasher` (placeholder, spec says "replace with SHA-256
behind a feature flag later"). These are not the same hash function.

Mirror's Oid also has an incremental `OidHasher` that prism-core lacks.

**What changes:**
- Prism-core's `Oid::hash()` must be upgraded to SHA-512 (or made generic
  over hash algorithm) before mirror can adopt it.
- Mirror's `OidHasher` (incremental SHA-512) needs to be ported into
  prism-core or kept as a mirror extension.
- `Oid::dark()` and `Oid::is_dark()` exist only in prism-core. Mirror does
  not use them but they are compatible.
- Once hash functions align, mirror's `kernel::Oid` becomes a re-export of
  `prism_core::Oid`.

**Migration order:** This blocks everything else. Oid is the atom.

### 2. `mirror::kernel::TraceOid` → no prism-core equivalent

**Classification: KEEP**

`TraceOid` is a newtype over `Oid` with a different semantic role (trace
identity vs content identity). Prism-core has no trace concept. Mirror keeps
this type but builds it on `prism_core::Oid` once Oid is unified.

### 3. `mirror::kernel::ContentAddressed` → `prism_core::Addressable`

**Classification: DELETE (after Oid unification)**

Mirror's `ContentAddressed` trait:
```rust
pub trait ContentAddressed {
    type Oid: Clone + PartialEq + Eq + Hash + Debug + Display + AsRef<str> + Into<TraceOid>;
    fn content_oid(&self) -> Self::Oid;
}
```

Prism-core's `Addressable` trait:
```rust
pub trait Addressable {
    fn oid(&self) -> Oid;
}
```

Mirror's version is more general (associated Oid type). Prism-core's version
is simpler (concrete `Oid`). The associated type exists because mirror has
`TraceOid` and `MirrorOid` as distinct OID types. If those become newtypes
over `prism_core::Oid`, the associated type is no longer needed.

**What changes:**
- `ContentAddressed` becomes `Addressable` from prism-core.
- Types that impl `ContentAddressed` with `type Oid = kernel::Oid` switch to
  `impl Addressable`.
- Types that use `type Oid = TraceOid` or domain OIDs keep a local trait or
  use `Into<prism_core::Oid>` adapters.

### 4. `mirror::shatter_format::Luminosity` → `prism_core::Luminosity`

**Classification: ADAPT**

Mirror's Luminosity:
```rust
pub enum Luminosity { Light, Dimmed, Dark }
```

Prism-core's Luminosity:
```rust
pub enum Luminosity { Light, Dimmed(f64), Dark }
```

Mirror's version is a classification enum (derived from `MirrorLoss`).
Prism-core's version carries a holonomy value in `Dimmed(f64)`.

Mirror's `Luminosity::from_loss(&MirrorLoss)` computes the classification.
This maps cleanly: `Light` stays, `Dimmed` gains a `holonomy()` value from
the loss, `Dark` stays.

**What changes:**
- Delete `mirror::shatter_format::Luminosity`.
- Replace with `prism_core::Luminosity`.
- `Luminosity::from_loss()` becomes a standalone function that returns
  `prism_core::Luminosity::Dimmed(loss.holonomy())` instead of bare `Dimmed`.
- `ShatterMeta`, `ShatterNotification`, and `MirrorLspBackend` all use the
  prism-core type.
- `Luminosity::parse()` / `as_str()` must be added to prism-core or kept as
  an extension trait in mirror for serialization.

### 5. `mirror::store::MirrorOid` → `prism_core::Oid`

**Classification: DELETE (after Oid unification)**

`MirrorOid` is a newtype over `mirror::kernel::Oid`. Once kernel Oid is
unified with prism-core Oid, MirrorOid becomes either:
- A direct re-export of `prism_core::Oid`, or
- Deleted entirely, with call sites using `prism_core::Oid` directly.

### 6. `mirror::store::Shard<V>` → no direct prism-core equivalent

**Classification: KEEP**

Mirror's `store::Shard<V>` is a `(value, MirrorOid)` pair — a value with its
content address. This is not the same as `mirror::shard::Shard` (the
compilation artifact). Prism-core has no generic value+address wrapper.

This type is small and domain-specific. Keep it in mirror, built on
`prism_core::Oid`.

### 7. `mirror::store::Store` → `prism_core::Store`

**Classification: ADAPT**

Mirror's Store trait:
```rust
pub trait Store {
    type Value;
    type Shard: ContentAddressed;
    type Error;
    type Loss: Loss;
    fn insert(&mut self, value: Self::Value) -> Imperfect<Self::Shard, ...>;
    fn get(&self, oid: &MirrorOid) -> Imperfect<Self::Shard, ...>;
}
```

Prism-core's Store trait:
```rust
pub trait Store {
    type Error;
    type L: Loss;
    fn get(&self, oid: &Oid) -> Imperfect<Vec<u8>, ...>;
    fn put(&mut self, oid: Oid, data: Vec<u8>) -> Imperfect<Oid, ...>;
    fn has(&self, oid: &Oid) -> Imperfect<Luminosity, ...>;
}
```

These are different interfaces. Mirror's Store is typed (generic over Value
and Shard). Prism-core's Store is byte-oriented (raw `Vec<u8>`). Mirror's
Store computes OIDs internally on insert. Prism-core's Store takes OIDs as
arguments to `put`.

**What changes:**
- Mirror's `Store` trait does NOT delete. It wraps prism-core's `Store` as
  the persistence backend but adds the typed surface (Value → Shard with
  computed OID).
- `MirrorGitStore` does NOT implement `prism_core::Store` directly — it uses
  `fragmentation_git` underneath, not raw byte storage.
- The `has()` method from prism-core's Store is useful and should be adopted
  by mirror's Store trait.
- Eventually: mirror's Store becomes `TypedStore<S: prism_core::Store>`, a
  typed wrapper over the raw byte store.

### 8. `mirror::shard::Shard` → no direct prism-core equivalent

**Classification: KEEP**

Mirror's `shard::Shard` is a compilation artifact: `(grammar_oid, KernelSpec,
Target)`. This is the `Closure` layer of the bundle tower — domain-specific
to the mirror compiler. Prism-core provides the `Closure` trait but not the
concrete artifact type.

### 9. `mirror::bundle::MirrorCompiler` → already uses prism-core's Bundle

**Classification: KEEP (already integrated)**

`MirrorCompiler` already implements `Fiber`, `Connection`, `Gauge`,
`Transport`, and `Closure` from prism-core's bundle module. It IS the bundle
tower instantiation. No change needed except updating the Oid references if
kernel Oid is unified.

### 10. `mirror::bundle::Target` → no prism-core equivalent

**Classification: KEEP**

`Target` (Beam/Wasm/Metal) is mirror's domain-specific gauge group. Prism-core
provides the `Gauge` trait; mirror provides the concrete `Target` enum.

### 11. `mirror::mirror_runtime::Form` → no prism-core equivalent

**Classification: KEEP**

`Form` is the parsed AST node of the `.mirror` grammar. It carries kind, name,
params, variants, children, and action metadata. This is pure domain content.
Prism-core has nothing equivalent.

`Form` could impl `prism_core::Addressable` (it already has `to_fragment()`
which computes a content hash), but that is an enhancement, not a replacement.

### 12. `mirror::mirror_runtime::CompiledShatter` → `Crystal<Named<MirrorData>>`?

**Classification: ADAPT (partial)**

`CompiledShatter` is `(Form, MirrorFragment)` — the parsed form plus its
content-addressed fragment. This is structurally similar to
`Crystal<Named<MirrorData>>`:

- `Named<MirrorData>` = the data with a name (the form name) and OID
- `Crystal<Named<MirrorData>>` = the named data with luminosity

But `CompiledShatter` carries the full `Form` tree (with children, actions,
body text), not just `MirrorData`. And the luminosity is carried separately
in `MirrorLoss`, not inside `CompiledShatter`.

**Verdict:** CompiledShatter does not reduce to `Crystal<Named<MirrorData>>`.
The types are related but not equivalent. CompiledShatter keeps its structure;
it could gain an `impl Addressable` that delegates to `fragment.oid()`.

### 13. `mirror::loss::MirrorLoss` → no prism-core equivalent

**Classification: KEEP**

`MirrorLoss` is mirror's domain-specific `Loss` implementation with four
sub-folds (parse, resolution, properties, emit). It already implements
`prism::Loss`. No prism-core type replaces it.

### 14. `mirror::shatter_format::ShatterMeta` → does Crystal carry this?

**Classification: KEEP**

`ShatterMeta` carries frontmatter for `.shatter` files: oid, luminosity,
holonomy, per-phase loss breakdown, and beam identity. `Crystal<P>` carries
`(P, Luminosity)` — just the prism and its settlement state. Crystal does NOT
carry the loss breakdown or beam identity.

ShatterMeta is a serialization format. Crystal is a runtime wrapper. Different
purposes.

### 15. `mirror::git_store::MirrorGitStore` → impl `prism_core::Store`?

**Classification: ADAPT (future)**

`MirrorGitStore` wraps `NamespacedGitStore` from `fragmentation-git`. It could
impl `prism_core::Store` by mapping `get/put/has` to its existing methods, but
the interface mismatch (typed crystals vs raw bytes) means this is a future
task, not a current deletion.

### 16. `mirror::optic::MirrorOptic` → no prism-core equivalent

**Classification: KEEP**

Domain-specific: loads a compiled grammar and extracts named actions. No
prism-core analog.

### 17. `mirror::lsp::server::MirrorLspBackend` → no prism-core equivalent

**Classification: KEEP**

Domain-specific LSP mapping layer. Uses Luminosity (which should switch to
prism-core's version) but is otherwise independent.

---

## Summary Table

```
mirror type                    prism-core type           action    blocks
────────────────────────────   ───────────────────────   ──────    ──────
kernel::Oid                    Oid                       ADAPT     nothing (root)
kernel::TraceOid               (none)                    KEEP      Oid
kernel::ContentAddressed       Addressable               DELETE    Oid
shatter_format::Luminosity     Luminosity                ADAPT     nothing
store::MirrorOid               Oid                       DELETE    Oid
store::Shard<V>                (none)                    KEEP      Oid
store::Store                   Store (wraps)             ADAPT     Oid, Luminosity
store::ForeignKey              (none)                    KEEP      —
shard::Shard                   (none)                    KEEP      —
bundle::MirrorCompiler         Bundle (already impl'd)   KEEP      —
bundle::Target                 (none)                    KEEP      —
mirror_runtime::Form           (none)                    KEEP      —
mirror_runtime::CompiledShatter (none)                   KEEP      —
loss::MirrorLoss               (none)                    KEEP      —
shatter_format::ShatterMeta    (none)                    KEEP      —
git_store::MirrorGitStore      Store (future impl)       ADAPT     Store
optic::MirrorOptic             (none)                    KEEP      —
lsp::MirrorLspBackend          (none)                    KEEP      —
```

---

## Migration Order

### Phase 0: Oid alignment (prerequisite)

Prism-core's `Oid::hash()` uses `DefaultHasher` (non-cryptographic, unstable
across Rust versions). Mirror's uses SHA-512. These MUST converge before any
type unification.

Options:
1. Upgrade prism-core's `Oid::hash()` to SHA-512 (preferred — matches mirror
   and conversation crate).
2. Make `Oid` generic over hash algorithm (over-engineered for now).
3. Feature-gate: `Oid::hash()` uses SHA-512 when `sha2` feature is enabled.

**Recommendation:** Option 1. One hash function. SHA-512. Prism-core gains a
`sha2` dependency.

### Phase 1: Luminosity unification

1. Add `parse()` / `as_str()` methods to `prism_core::Luminosity` (or an
   extension trait in mirror).
2. Add `from_holonomy()` to prism-core (already exists).
3. Replace `mirror::shatter_format::Luminosity` with `prism_core::Luminosity`
   everywhere.
4. `Luminosity::from_loss(&MirrorLoss)` becomes a free function returning
   `prism_core::Luminosity`.

**Tests affected:** `shatter_format::tests`, `lsp::server::tests` — all
Luminosity comparisons change from `Luminosity::Dimmed` (unit) to
`Luminosity::Dimmed(f64)` (carrying holonomy value). Pattern matches must
account for the inner value.

### Phase 2: Oid unification

1. After Phase 0, replace `mirror::kernel::Oid` with `prism_core::Oid`.
2. Delete `mirror::store::MirrorOid` (replace with `prism_core::Oid`).
3. Delete `mirror::kernel::ContentAddressed` (replace with
   `prism_core::Addressable`).
4. Keep `TraceOid` as a newtype over `prism_core::Oid`.
5. Update `domain_oid!` macro to produce types wrapping `prism_core::Oid`.

**Tests affected:** Every test that constructs `Oid::new(...)` or
`MirrorOid::hash(...)` — roughly 40+ test functions across `store.rs`,
`loss.rs`, `bundle.rs`, `shatter_format.rs`, `git_store.rs`. Method
signatures are compatible (`new`, `hash`, `as_ref`), so most changes are
import path updates.

### Phase 3: Store trait alignment

1. Add `has()` method to mirror's `Store` trait.
2. Consider `TypedStore<S: prism_core::Store>` wrapper.
3. `MirrorGitStore` gains `impl prism_core::Store` for raw byte access.

**Tests affected:** `store::tests`, `git_store::tests`.

### Phase 4: Addressable adoption

1. `Form` gains `impl prism_core::Addressable`.
2. `CompiledShatter` gains `impl prism_core::Addressable` (delegates to
   `fragment.oid()`).
3. Any type that previously impl'd `ContentAddressed` with
   `type Oid = kernel::Oid` switches to `impl Addressable`.

**Tests affected:** Minimal — additive change.

---

## Derive Macro

**Not needed yet.**

The `#[derive(Prism)]` macro would generate `Fiber`, `Connection`, `Gauge`,
`Transport`, `Closure` implementations. Currently only `MirrorCompiler`
implements the full tower, and it has domain-specific logic in each impl
(especially `Transport::transport`). A derive macro would need to be
opinionated about what each layer does generically, and we do not yet have
enough concrete instances to identify the pattern.

**When it becomes needed:** When a second or third type implements the full
bundle tower. At that point the shared structure will be visible and the
macro can codify it.

---

## What Does NOT Map

Some mirror types exist in a different conceptual space than prism-core:

- **Form** — AST, not optic. Prism-core operates on beams and transformations.
  Forms are the *content* that flows through prisms, not prisms themselves.
- **CompiledShatter** — compilation artifact, not a Crystal. A Crystal is a
  settled prism. A CompiledShatter is a settled *compilation*. Related but not
  identical.
- **MirrorLoss** — domain-specific loss. Prism-core provides the `Loss` trait;
  mirror provides the four-fold implementation. This is the intended
  relationship.
- **MirrorGitStore** — implementation, not abstraction. Prism-core's `Store`
  is the abstraction; `MirrorGitStore` is one implementation among potentially
  many.

---

## Risk Assessment

**Low risk (import path changes only):**
- Luminosity unification (Phase 1)
- Addressable adoption (Phase 4)

**Medium risk (hash function change):**
- Oid alignment (Phase 0) — changing the hash function in prism-core affects
  all content addresses. Existing OIDs stored by prism-core consumers will
  be invalidated. If prism-core has no persistent stores yet, this is safe.

**Higher risk (interface change):**
- Store trait alignment (Phase 3) — the typed/untyped split means mirror's
  Store may need to become a two-layer design. This touches `MirrorGitStore`,
  which has filesystem side effects and integration tests.

---

*Seam found: five types delete, four adapt, nine keep. The tower is already
standing in bundle.rs. The seams are in the foundation (Oid hash function)
and the glass between serialization (Luminosity enum shape). Fix the
foundation first.*
