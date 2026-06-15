# mirror-store — three-layer parser, fragmentation as substrate, FP1 at Layer 2

*2026-05-22. Mara. Updated 2026-06-04 (Reed + Alex). Spec.*

Status: **Red** (the three-layer architecture is named; the store API
is specified; the FP1 reframe is stated; the fragmentation audit
records a verdict; the boot sequence and Shift dispatch are pinned. No
code lands in this tick.)

> **2026-06-04 reframe (Reed + Alex, canonical).** The fragmentation
> store IS the canonical content-addressed substrate. **No deps;
> shards are self-contained crystals.** Splinter IS the structural
> lockfile. Composition by OID, not name resolution.
>
> **`.shatter` is one OPTIONAL projection format** of `au + splinter +
> mosaic` — see [[../shatter-spec]]. Other projection formats are
> possible (binary, IR, language-specific embeddings); the store does
> not require any particular on-disk format. Whatever a tool projects
> from the store, the store's content addressing is the canonical
> identity.
>
> `mirror shatter` is **plumbing** — direct content-store access
> (analogous to `git cat-file`). The porcelain (mosaic settlement,
> kintsugi-on-spec) is in [[kintsugi-ci-v0.1]].
>
> The substrate-pull arc: `shards/` is source of truth (per
> [[prism-floor-and-the-grammar-rename]]); the store backs `Shift`
> dispatch via OID composition; legacy `boot/` + `bootstrap/` use the
> store as their content backend.

Depends on:
- `mirror/docs/specs/parser-as-prism-grammar.md` — the `Combinator`
  enum, FP1/FP2/FP3 fixed points, the meta-glass self-parse equation,
  the `Shift { grammar, body }` combinator the store backs.
- `mirror/docs/specs/walker-contract.md` — Mara's F-1 contract; the
  walker's per-variant byte-consumption rules and the
  Checkpoint-C scope note that the Shift "registry is empty and Shift
  falls back to walking `body` structurally."
- `mirror/docs/specs/bootstrap-retirement-plan.md` — the end-state
  inventory after Ticks 1–5; the `grammar.rs` retirement target;
  the minimal-Rust-surface story.
- `mirror/docs/specs/combinator-optimization.md` — normalization
  before OID hashing; the property the store's `oid` operation
  depends on for FP1 robustness.
- `fragmentation/Cargo.toml` — the substrate candidate's manifest;
  the feature flags that gate the std touchpoints.
- `fragmentation/src/{fragment,store,bounded_store,frgmnt_store,sha,
  ref_,cid,encoding}.rs` — the API surface considered.
- `spectral/docs/specs/` — task #43 (spectral-db) is a separate
  project that consumes the same fragmentation substrate. This spec
  defines mirror's Layer-1 store — NOT spectral-db. spectral-db's
  own surface (distribution, deltas, conflict resolution, the
  MNESIA adapter, the garden's read+write path) lives in its own
  spec.

Unblocks:
- F-2 (next checkpoint after F-1): wire the Shift dispatch through a
  real store. Without it, `Shift` walks structurally and the
  cross-grammar dispatch is a no-op.
- Tick 4c of the retirement plan: `grammar.rs` retires once the
  store owns grammar loading.
- spectral-db / task #43: remains a separate project. Shares
  fragmentation as content-addressed substrate; does NOT inherit
  from mirror's store. spectral-db owns distribution, deltas,
  conflict resolution, the MNESIA adapter — the engine that
  powers the garden. Sharing a substrate is not the same as being
  the same artifact.
- The `@mirror/store` grammar declaration (the existing
  three-backend sketch in this file's prior revision) gets a
  concrete Layer-1 implementation it can dispatch over.

---

## Recognition

The F-1 walker landed real byte consumption in every arm. Five
commits (`b9118cb` → `62b8650`) put `Literal`, `Charset`, `Seq`,
`Repeat`, `Choice`, `Capture`, `BraceBlock`, `ParenBlock`, `Until`,
`Shift`, and `DarkFallback` on the bytes. 111 tests. The walker walks.

But the seed retreated. The original parser-as-prism story called
for the seed to encode the meta-glass — the grammar that parses
`glass.mirror` (the file that declares `.mirror`'s own form). That
seed-as-meta-glass produces FP1 as a load-bearing equation:
`apply_h(seed, glass.mirror.bytes) == seed` (OID-equal). The
combinator tree the seed encodes IS the combinator tree the seed
produces when it parses the file that declares it.

The current seed doesn't claim this. The seed today is a permissive
**balanced-bytes recognizer** — it accepts any well-formed
brace/paren-balanced stream. FP1 in its tautological form holds
(the seed accepts `glass.mirror.bytes` without Dark fallthrough),
but the *structural* FP1 — the OID equality — would require the
seed to be the meta-glass.

Three things are needed before the structural FP1 lands:

1. A **store** the walker can `fetch` `@<grammar>` references from.
   Today `Shift` walks `body` structurally because the registry is
   empty. With a store, `Shift { grammar: @mirror/glass, body }`
   dispatches `apply_h(store.fetch(@mirror/glass), body_bytes)`.
2. A **boot sequence** that populates the store with the loaded
   `.mirror` files at startup. The seed parses the meta-glass; the
   meta-glass parses every other `.mirror` file; each file's
   resulting Combinator tree is inserted into the store under its
   declared `@<ref>`.
3. A clean **layering** that distinguishes "what stays Rust because
   it must" from "what the store can deliver as data." The walker
   blurred this. The store names it.

This spec lands the architecture. The implementation follows in F-2.

The thing the types don't say: **the Shift registry IS the mirror
store.** Two names; one Layer-1 component, keyed on `@<ref>`, with
a small closed `Entry` enum (Combinator + AstNode + Bytes).
spectral-db (task #43) is a separate project that consumes the same
fragmentation primitives the mirror store consumes — but adds an
engine on top: distribution, deltas, conflict resolution, the
MNESIA adapter. Sharing fragmentation is not being the same
artifact. The car is not the fuel.

---

## 1. The three-layer architecture

The original parser-as-prism spec read implicitly as two layers: the
Rust seed (Layer 1), and the loaded grammars (Layer 2). The
self-hosting story (FP1) lived inside Layer 2 — the seed parses the
meta-glass and the meta-glass parses everything else.

That reading is incomplete. Real source files (`.mirror` files in
`mirror/`, `.rs` files in `bootstrap/src/`, `.ll` files in the
butterfly's output) are not the meta-glass. They are application
files. The meta-glass is the grammar; the source files are bytes
the grammar parses.

Three layers, named explicitly:

### Layer 1 — Rust substrate (~30 LOC seed + walker + store + @io)

The minimum Rust surface that cannot be expressed in mirror without
circularity. Stays Rust through v1. Four concerns:

- **Walker** — `walk_combinator_at` (per `walker-contract.md`). The
  evaluator. Real today; landed in F-1.
- **@io kernel** — `git.rs`, `exec.rs`, the file-read calls. The
  subprocess boundary. Stays for the butterfly's clang shell-out
  and the git crystal cache.
- **Store** — this spec. The content-addressed key/value layer
  keyed on `@<ref>`. New in F-2.
- **Seed** — `prism_seed()`. The smallest Combinator literal that
  parses `mirror/glass.mirror`. Per `parser-as-prism-grammar.md`'s
  4b.2/4b.3 trajectory, ~30 LOC.

Everything else retires into grammar declarations or into other
layers.

### Layer 2 — loaded grammars (`.mirror` files registered in the store)

The structural intelligence layer. `.mirror` files parsed by the
seed (for the meta-glass) or by the meta-glass (for every other
grammar), then registered in the store under their declared
`@<ref>`.

The boot sequence (§5) populates this layer. After boot, the store
contains:

- `@mirror/glass` — the meta-glass; the grammar that parses
  `.mirror` files.
- `@nl` — the natural-language grammar; bare comments + inline
  backticks + fenced blocks + doctest prompts.
- `@code/rust`, `@code/llvm/ir` — the host grammars for the
  butterfly's input/output.
- Every other grammar mirror needs (`@mirror/kintsugi`,
  `@mirror/butterfly`, `@hash/coincidence`, etc.) as they're
  declared.

### Layer 3 — application files (`.mirror`, `.rs`, `.ll` source)

Files that declare which grammar they consume and submit themselves
to that grammar. Two paths:

- **Self-declaring** (the common case): the file's first non-trivial
  token names its grammar. `mirror/glass.mirror` opens with
  `grammar @mirror/glass`. `mirror/00-prism.mirror` opens with
  `grammar @mirror/prism`. The seed reads enough to extract the
  declared `@<ref>`, then dispatches.
- **Extension-implied**: `.rs` files use `@code/rust`; `.ll` files
  use `@code/llvm/ir`. The mapping table is data in
  `mirror/code/<lang>.mirror` (the host grammar declares its file
  extensions).

Parsing a Layer-3 file is:
`apply_h(store.fetch(declared_grammar), file.bytes)`.

The three layers are not optional. The seed cannot parse a Layer-3
file in general — the seed is sized to the meta-glass, not to
arbitrary host grammars. The meta-glass cannot parse `code/rust.mirror`
itself (it would have to be a superset of every grammar). The store
makes the dispatch possible.

This is the deviation from `parser-as-prism-grammar.md`'s implicit
two-layer reading. The spec's "FP3 — every other grammar lifts the
same way" already names Layer 3 in fact; this spec names it in
architecture.

---

## 2. Where FP1 actually lives

The original FP1 was:

```rust
let seed = prism_seed();
let glass_bytes = read_file("mirror/glass.mirror");
let meta_glass = apply_h(seed, (glass_bytes, 0)).into_focus().unwrap();
assert_eq!(combinator_tree_oid(&seed),
           combinator_tree_oid(&meta_glass));
```

Single-layer claim: the seed parses the file that declares it; the
two trees hash byte-identical.

Under the three-layer split, this equation lives in two places, with
different load-bearing strengths.

### Layer-1 acceptance check (low bar)

```rust
let seed = prism_seed();
let glass_bytes = read_file("mirror/glass.mirror");
let result = apply_h(seed, (glass_bytes, 0));
assert!(result.is_success());                          // no Dark fallthrough
assert!(result.into_focus().is_some());                // structural parse
```

The seed's bytes are syntactically well-formed under the seed's own
recognizer. This is what the permissive balanced-bytes seed achieves
today. It says nothing about *what* the parse produced — only that
the parse didn't fall through to `DarkFallback` at the top level.

Useful as a smoke test. Cheap to maintain. Doesn't pin the
relationship between the seed's structure and the meta-glass's
structure.

### Layer-2 self-hosting fixed point (load-bearing)

```rust
let seed = prism_seed();
store.boot(["mirror/glass.mirror", "mirror/nl.mirror", ...])?;

// The structural assertion:
let meta_glass = store.fetch(reference!("@mirror/glass")).unwrap();
let glass_bytes = read_file("mirror/glass.mirror");
let reparsed = apply_h(&meta_glass, (glass_bytes, 0)).into_focus().unwrap();

assert_eq!(store.oid(reference!("@mirror/glass")).unwrap(),
           combinator_tree_oid(&reparsed));
```

Once the meta-glass is loaded and registered, the assertion is: the
loaded grammar parses the file that declares it to byte-identical
structure (after normalization, per `combinator-optimization.md`).
The store's `@mirror/glass` entry, applied to `glass.mirror`'s
bytes, produces a Combinator tree whose OID matches the store's
stored OID for `@mirror/glass`.

This is the real proof of self-hosting. Three things must hold for
it to pass:

1. The seed's parse of `glass.mirror` and the meta-glass's parse
   of `glass.mirror` agree (the seed is contained in the
   meta-glass's expressive range on that input).
2. Normalization is a fixed point — running it twice produces the
   same tree (Idempotency, per `combinator-optimization.md` §3).
3. The Merkle hash is canonical post-normalization (no
   structural sharing artifacts; same tree → same OID).

The original spec's claim moves from Layer 1 to Layer 2. The Layer-1
form survives as an acceptance check; the Layer-2 form is the
fixed-point equation the bootstrap stands on.

### FP2 / FP3 under the layering

FP2 (`@mirror/prism` lifts cleanly through the meta-glass) and FP3
(every other grammar lifts the same way) are Layer-2/Layer-3 well-
formedness assertions: each `.mirror` file in the boot tree parses
through the store-resident meta-glass without Dark fallthrough. The
assertions stay where `parser-as-prism-grammar.md` puts them; this
spec just makes explicit that they assume the store is populated.

---

## 3. The store API surface

Six functions. Total. The Entry type's enum is closed (in mirror,
not Rust — see §3.2).

```rust
// Layer-1 surface — Rust trait.
pub trait MirrorStore {
    /// Fetch the entry registered under `ref`. None if not present.
    fn fetch(&self, r: &Reference) -> Option<&Entry>;

    /// Insert `entry` under `ref`. Returns the entry's OID.
    /// Idempotent: inserting the same `(ref, entry)` returns the
    /// same OID and is a no-op on the second call.
    fn insert(&mut self, r: Reference, entry: Entry) -> Oid;

    /// True iff `fetch(r)` would return Some.
    fn contains(&self, r: &Reference) -> bool;

    /// Convenience: fetch `r`, then walk `input` through the
    /// resulting combinator. None if `r` is not a Combinator entry
    /// (or if `r` is not registered).
    fn walk_via(&self, r: &Reference, input: &[u8]) -> Option<Imperfect<Witness>>;

    /// Boot the store from a list of `.mirror` file paths. Loads
    /// the seed, applies it to each file, registers each resulting
    /// Combinator under its declared `@<ref>`. Order-aware (see §5).
    fn boot(&mut self, roots: &[Path]) -> Result<(), BootError>;

    /// The OID of `r`'s entry. None if `r` is not registered.
    /// `oid(r)` equals the OID returned by the last `insert(r, _)`
    /// call that didn't get superseded by a later `insert`.
    fn oid(&self, r: &Reference) -> Option<Oid>;
}
```

That's the total surface. Six functions. The store stays small
and Combinator-focused; it does NOT grow into spectral-db's
content types. spectral-db declares its own typed entry surface in
its own crate (see §7).

### 3.1 Reference, Oid, Entry, Witness

`Reference` is `@<path>`: the leading `@`, slash-separated
components, optional `(args)` tail. Today's representation in
`Combinator::Shift { grammar: String, ... }` is the string form; the
typed form is:

```rust
pub struct Reference {
    path: Vec<Cow<'static, str>>,        // ["mirror", "glass"]
    tags: Vec<Cow<'static, str>>,        // ("mirror", "spec") on grammar @mirror/grammar
}
```

`Oid` is `[u8; 32]` — the post-normalization Merkle hash of the
entry's encoded bytes. Aligns with
`combinator_tree_oid`'s current shape in `spectral.rs`.

`Witness` is `Imperfect<AstNode, Infallible, ScalarLoss>` — the
walker's return type today. Re-used so `walk_via` doesn't introduce
a parallel return shape.

### 3.2 Entry — closed in mirror, not Rust

The Entry type is a closed sum. v1 (this tick's scope, store-only):

```rust
pub enum Entry {
    Combinator(Combinator),     // grammar trees; the Shift registry's content
    AstNode(AstNode),           // parsed source trees (Layer-3 outputs)
    Bytes(Vec<u8>),             // raw byte blobs (.mirror file source, .rs/.ll source)
}
```

But this enum should NOT live in `bootstrap/src/`. It lives in
`mirror/store/entry.mirror` — declared in the grammar layer, parsed
by the meta-glass, evaluated by the bootstrap's `Fold5` over its
own kinds. The Rust side gets a generated enum (per the Tick 4c
plan; the keyword↔kind table is data, so is the entry-type table).

This keeps the closed-surface honesty consistent with
`parser-as-prism-grammar.md` §"The named vocabulary": every
extension to the closed surface is a spec change AND a grammar
change AND (for now) a Rust impl-match-arm change. When Tick 4c
lands, the Rust side becomes data-driven and only the spec +
grammar change.

The brief asks for forward-room for `Project`, `Crystal`, `Gestalt`.
Those are §7's concern — v1.5 and v2 extensions. The v1 surface is
the three variants above.

### 3.3 Why six and not more

The brief said "6–10 functions max." Six is the floor. Two
candidates considered and rejected:

- **`remove(r)`** — content-addressed stores don't remove. The OID
  is the identity. Removal would mean garbage collection, which is
  a separate concern owned by the @io kernel (the on-disk store
  has compaction; the in-memory store doesn't, since process exit
  is the GC).
- **`iter()` / `keys()`** — useful for diagnostics, not for the
  Shift dispatch. Defer until a concrete diagnostic use lands; not
  added speculatively for spectral-db (spectral-db has its own
  storage layer).

The `boot` function is the heavy lifter — it owns the load order
(§5), the seed-vs-meta-glass dispatch, and the failure modes
(§5.2). The other five are leaves.

---

## 4. Fragmentation as substrate — audit + decision

The brief: does fragmentation provide the right substrate for the
mirror store?

**Verdict: yes, with cleanup.**

Fragmentation already does content-addressed storage. The
primitives are right. The cleanup is real but bounded. The
estimate: 3 cuts before mirror can depend on it, ~1 session each.

### 4.1 What fragmentation provides

Per `fragmentation/Cargo.toml` and `src/lib.rs`:

- **`Fragmentable` trait** (`fragment.rs:12`) — `Data: Encode`,
  `Hash: HashAlg`, plus `is_shard`/`is_fractal`/`data`/`children`.
  Type-level: pluggable hash, encoding generic. This IS the
  shape the store needs. `Entry` becomes `Fragmentable`.
- **`Fractal<E, H>`** (`fragment.rs:42`) — the recursive
  node. `Shard` (terminal) + `Fractal` (recursive). Combinator
  trees map directly: `Seq`/`Choice`/`Repeat` etc. are recursive
  nodes; `Literal`/`Charset`/`LiteralKind` are terminal.
- **`HashAlg` trait** (`sha.rs:9`) — pluggable hash algorithm.
  Mirror already has `CoincidenceHash<5,5>` (the matrix-form
  Dirac). Plugging it in as the `HashAlg` for the mirror store
  is one impl block. `Sha::hash` uses SHA-256, which matches
  `combinator_tree_oid`'s current shape.
- **`ConcurrentStore<N, H>`** (`concurrent_store.rs`) — lock-free
  reads, shard-locked writes, `Send + Sync` unconditional. The
  in-memory store the mirror Layer-1 needs.
- **`FrgmntStore<N>`** (`frgmnt_store.rs`) — bounded cache +
  on-disk spillover under `.frgmnt/objects/`. Useful for mirror's
  own persistence when the boot set outgrows memory. spectral-db
  has its own persistence story (MNESIA + sync); not this spec's
  concern.
- **`BoundedStore<N>`** (`bounded_store.rs`) — size-bounded cache
  with LIFO eviction. For the gestalt-tier cache mirror's LSP
  surface wants.
- **`Cid<H>`** (`cid.rs`) — self-describing content identifier;
  wraps `Ref<H>` with codec + hash-algorithm metadata. The
  forward-compatibility envelope for future content types if
  mirror's `Entry` ever needs codec variation (it currently
  doesn't).
- **`Reconstructable` trait** (`fragment.rs:222`) — round-trip
  encode/decode. The serialization story for `boot()`'s persistence
  variant.

### 4.2 The boundary — what fragmentation owns vs what mirror owns

**fragmentation owns:**

- Content addressing primitives (`Fragmentable`, `Fractal`, `HashAlg`).
- Storage backends (`ConcurrentStore`, `FrgmntStore`, `BoundedStore`).
- Encoding/decoding round-trip (`Encode`, `Decode`).
- The `Cid<H>` envelope.
- Git-native persistence (`git.rs`, under the `git` feature).

**mirror owns:**

- The `Entry` enum and its `Fragmentable` impl.
- The `Reference` type (`@<path>`-shaped keys).
- The mapping `Reference` → content-store key (`Ref::label`).
- The `MirrorStore` trait — the six-function interface (§3).
- `boot()` — the parse-and-register loop (§5).
- `walk_via()` — the convenience composition of `fetch` + `apply_h`.

The boundary is clean: fragmentation knows nothing about
Combinators, AstNodes, or grammar references. Mirror knows nothing
about how the bytes get hashed or where they're stored.

### 4.3 The Cargo.toml line

```toml
[dependencies]
fragmentation = { path = "../fragmentation", default-features = false }
```

Default features: none. The git/ssh/gpg/fuse/cli features are not
needed for Layer 1. The `cli` binary is irrelevant; the mirror
bootstrap doesn't need a `frgmt` subcommand.

For any future on-disk mirror-store mode (mirror's own concern,
separate from spectral-db's persistence), we add:

```toml
fragmentation = { path = "../fragmentation",
                  default-features = false,
                  features = ["git"] }
```

— but that's a v1.5 concern, not v1.

### 4.4 `no_std + alloc` compatibility

`bootstrap-retirement-plan.md`'s no_std stretch wants the bootstrap
to compile `no_std + alloc`. Mirror only consumes a subset of
fragmentation (the in-memory primitives), so the audit question is:
does *that subset* compile no_std + alloc?

Per `fragmentation/Cargo.toml`:

- **Required dependencies**: `serde`, `serde_json`, `sha2`, `sha1`,
  `hex`, `dashmap`. All of these have `no_std` variants:
  - `serde` — `default-features = false` is no_std (alloc + derive).
  - `serde_json` — requires alloc, supports no_std.
  - `sha2`, `sha1` — RustCrypto, both no_std-compatible with
    `default-features = false`.
  - `hex` — supports no_std with `default-features = false`.
  - `dashmap` — **blocker**. Depends on `std::sync` and `parking_lot`.
    Not no_std-compatible today.
- **Optional dependencies** (git/ssh/gpg/fuse/cli) — all behind
  features. Disabled by default. None block no_std.

The `dashmap` blocker is real but scoped: `ConcurrentStore` and the
`BoundedStore` both use `dashmap::DashMap`. Mirror's in-memory store
needs concurrent reads (LSP's gestalt tier wants warm reads from
multiple worker threads). Three resolutions, ranked:

1. **Feature-gate `dashmap` behind a `concurrent` feature in
   fragmentation.** The `Send + Sync` story moves behind the
   feature. The single-threaded variant uses `RefCell<HashMap>` (or
   `alloc::collections::BTreeMap` for ordered iteration). Mirror's
   bootstrap is single-threaded today (the walker has no rayon, no
   tokio); single-threaded variant is fine for v1. Cleanup: §4.5.
2. **Replace `dashmap` with `hashbrown::HashMap` behind a
   `spin::Mutex`.** `hashbrown` is no_std-compatible; `spin::Mutex`
   is no_std-compatible. Concurrent reads cost a lock, but for a
   read-mostly store the contention is minimal. Cleanup: alternative
   form of (1).
3. **Leave `dashmap` and accept that the no_std stretch is std-only
   for `concurrent_store`.** Pragmatic if (1) and (2) take longer
   than the no_std stretch's payoff window.

The brief asked "verify that fragmentation can be a `no_std + alloc`
build for the embedded story." The answer is yes, with the
`dashmap` resolution in (1) or (2). The in-memory store mirror needs
becomes no_std-clean. The git-backed store stays std-only.

### 4.5 The cleanup — top three cuts

Ranked by load-bearing weight, with effort estimates:

#### Cut 1 — feature-gate `dashmap` behind `concurrent`

Effort: **medium** (~1 session)

`fragmentation/Cargo.toml`:

```toml
[features]
default = ["concurrent"]
concurrent = ["dep:dashmap"]
```

`src/concurrent_store.rs` and `src/bounded_store.rs` become
`#[cfg(feature = "concurrent")]`. A new
`src/single_threaded_store.rs` (or `src/local_store.rs`) provides
the `&mut self` API on `RefCell<HashMap>`.

Mirror's bootstrap depends without the `concurrent` feature in v1.
The LSP/spectral-db side opts in.

**Why first.** This is the no_std blocker. Without it, mirror can't
land the no_std stretch.

#### Cut 2 — narrow the `Fragmentable` trait surface

Effort: **small** (~half session)

`Fragmentable` today carries `Data: Encode`, `Hash: HashAlg`, plus
seven required methods. The mirror store only needs:

- `content_oid() -> String` (or the typed `Oid` form)
- `encode() -> Vec<u8>` (for serialization)

The other methods (`is_shard`, `is_fractal`, `data`, `children`,
`name`) make sense for `Fractal<E, H>` but bleed into the trait for
all impls. Mirror's `Entry::Combinator` doesn't have a "shard vs
fractal" distinction (a `Combinator::Literal` is terminal; a
`Combinator::Seq` is recursive; but they're variants of one enum,
not two type-level shapes).

**Resolution.** Split `Fragmentable` into a small required trait
(`ContentAddressed`) and an optional extension trait
(`TreeShaped`) that carries the tree-walking methods. `Fractal`
implements both. Mirror's `Entry` implements only the small one.

Backward-compatible: a blanket impl `impl<F: Fragmentable> TreeShaped
for F` keeps existing code working.

**Why second.** It's the biggest surface mismatch. The trait works
for fragmentation's internal users but doesn't fit `Entry`'s shape
cleanly. Without this cut, mirror either implements a bunch of
no-op trait methods or wraps `Entry` in a useless `Fractal`-shaped
container.

#### Cut 3 — rename `Fractal::Fractal` and related ambiguities

Effort: **small** (~quarter session)

`Fractal<E, H>::Fractal { … }` is the recursive variant of the
`Fractal` enum. `Fractal::Shard` is the terminal variant. The doubly-
named variant (`Fractal::Fractal`) is confusing in Rust code and in
documentation.

Suggestion: rename the recursive variant to `Branch`. The enum's
name (`Fractal`) stays — it's a shape claim, not a variant claim.

```rust
pub enum Fractal<E, H: HashAlg> {
    Shard { ref_: Ref<H>, data: E },
    Branch { ref_: Ref<H>, data: E, children: Vec<Self> },
    Lens { ref_: Ref<H>, data: E, children: Vec<Self>, targets: Vec<H> },
}
```

(`Lens` is already a recursive variant with cross-tree references;
the rename clarifies the family.)

**Why third.** Quality-of-life, not architecture. But mirror
depending on fragmentation means mirror's grep/docs surface includes
fragmentation's; the doubly-named variant becomes a recurring source
of confusion. Cheaper to fix once than to maintain explanations.

### 4.6 Where the cleanup lands

| Cut | Lands in | Tick scope |
|---|---|---|
| 1 — feature-gate dashmap | fragmentation | a fragmentation-side commit BEFORE mirror's F-2 |
| 2 — split `Fragmentable` | fragmentation | same fragmentation commit, OR in F-2's PR as a paired fragmentation/mirror change |
| 3 — rename `Fractal::Fractal` | fragmentation | a follow-up fragmentation tick; not blocking F-2 |

The minimum to unblock F-2 is Cuts 1 and 2. Both can land in a
single fragmentation commit (`reed/v1-floor` of fragmentation, paired
with mirror's F-2 branch). Cut 3 can come later; it's quality of
life.

The brief said: "if 'yes, with cleanup': spec the cleanup as its
own §X. List the cuts. Order them. Estimate effort." This is that.

---

## 5. The boot sequence

What happens at startup. The store goes from empty to fully
populated; the failure modes are pinned; the reproducibility
property is stated.

### 5.1 The happy path

```text
1. Mirror binary starts.
2. Rust initialises the store (empty).
3. The seed loads — Mara's existing permissive balanced-bytes
   recognizer (today) OR the meta-glass-encoding seed (after 4b.3
   in the parser-as-prism plan).
4. The bootstrap loads `mirror/<paths>/*.mirror` files one at a time.
   For each file F:
     a. Read F's bytes via @io.
     b. Apply the seed (or the previously-loaded meta-glass, once
        registered) to F's bytes:
          witness = apply_h(parser, (bytes, 0))
        where parser = seed if F is glass.mirror itself,
              parser = store.fetch(@mirror/glass) otherwise.
     c. Extract the declared `@<ref>` from F's parsed structure
        (the leading `grammar @<ref> { … }` form).
     d. Convert the witness's AST into a Combinator tree (the
        Fold5 instance that does this lives in spectral.rs; it's
        the inverse of `dump_ast`).
     e. Insert the Combinator into the store:
          store.insert(@<ref>, Entry::Combinator(tree))
5. After all files load, the store contains every grammar mirror
   uses. The seed itself is implicit — it lives in Rust; the store
   doesn't carry it. The meta-glass is in the store, but the seed
   bootstrapped it.
6. Subsequent apply_h calls dispatch through store.fetch(@<ref>)
   for Shift variants — see §6.
```

### 5.2 Load order

Three options:

- **Strict ordering**: the bootstrap declares a load order (e.g.,
  `[glass.mirror, nl.mirror, code/rust.mirror, ...]`). Each file
  loads only after every grammar it depends on is in the store.
- **Lazy resolution**: load files in any order; when a `Shift {
  grammar: @<ref>, ... }` resolves at parse time, if `@<ref>` is
  not yet in the store, treat the Shift as structural (per F-1's
  Checkpoint-C scope). After boot, re-walk to fill in resolutions.
- **Two-pass**: first pass loads all files into the store with
  Lifts unresolved (or with grammar references as opaque strings);
  second pass walks the store and resolves all Lifts.

**Recommendation: strict ordering.** Three reasons:

1. Reproducibility (5.4) is easier with deterministic order. Lazy
   resolution introduces ordering nondeterminism that the OID
   depends on.
2. Cycle detection becomes trivial: a strict topological sort fails
   loudly if the dependency graph has cycles.
3. The boot config (which files to load, in what order) becomes a
   declared spec — a `mirror.boot` file in the project root, or a
   default list in the bootstrap.

The trade-off: strict ordering requires the boot config to know the
dependency graph. For the v1 boot set (~10 `.mirror` files), this is
manual and fine. For larger sets (third-party grammars, project-
specific declarations), `mirror.boot` declares the order, OR a
`mirror boot --resolve` subcommand computes it via topological sort
of `in @<ref>` declarations.

### 5.3 Failure modes

Three failure modes the boot sequence must surface explicitly:

1. **A file fails to parse with the seed (or meta-glass).** The
   walker emits Dark fallthrough. The boot fails loudly. Diagnostic:
   the file path + the Dark span + the seed's apparent
   misunderstanding. Exit code 2 (matches `--strict` mode in
   today's `cmd_compile`).
2. **A file declares a grammar reference whose `@<ref>` is not yet
   declared.** Two sub-cases:
   a. The reference points at another file in the boot set that
      hasn't loaded yet → boot order bug → loud failure with
      "consider reordering" diagnostic.
   b. The reference points at a non-existent file → user error →
      loud failure with "no grammar declares @<ref>" diagnostic.
3. **Cyclic references between grammars.** Detected by the
   topological sort in `mirror boot --resolve`, OR by reachability
   analysis during strict-ordering boot. Diagnostic: the cycle in
   `@<ref> → @<ref> → ... → @<ref>` form.

All three are `BootError` variants. The Result type of `boot()`
carries them.

### 5.4 Reproducibility

**Property.** For a fixed boot set (same files, same bytes, same
order), `store.oid(@<ref>)` is byte-identical across runs.

This holds iff:

- The seed is deterministic (no environment dependence, no random
  source). Already true — `prism_seed()` is a constant.
- The walker is deterministic (no `HashMap`-iteration order leakage,
  no thread interleaving). Today's single-threaded walker satisfies
  this. The eventual concurrent path must use ordered insertion.
- Normalization is deterministic (per `combinator-optimization.md`
  §3 Idempotency + Canonicalization properties). Today's
  normalization passes #1 and #2 satisfy this.
- The Merkle hash is canonical post-normalization. Today's
  `combinator_tree_oid_hex` is canonical because normalization
  precedes hashing in the entry point.

Verification: a smoke test that boots the store twice in the same
process, then `assert_eq!(store.oid(r), store.oid(r))` for every
`r` in the boot set. A second smoke test that boots, serializes,
restarts, deserializes, and re-boots; the OIDs match.

### 5.5 The integration with the existing `@mirror/store` grammar

The current `mirror-store.md` (which this spec replaces) declared
the `@mirror/store` grammar contract:

```mirror
in @prism

grammar @mirror/store {
  store(crystal) -> oid { \ }
  fetch(oid) -> imperfect { \ }
  exists(oid) -> bool { \ }
}
```

That contract is the **outer interface** — the grammar declaration
of what a content-addressed store does. The Rust `MirrorStore`
trait (§3) is the **inner implementation** of one backend (the
in-memory + on-disk variant fragmentation provides). The two
relate as:

- `@mirror/store/memory` (new backend, the default for the
  bootstrap): wraps the Rust `MirrorStore` trait. Lives in the
  store layer; not user-facing.
- `@mirror/store/git`, `@mirror/store/nix`, `@mirror/store/spectral-db`
  — the existing three backends, declared in the previous
  revision. They stay as user-facing storage targets, each declared
  as a grammar in `mirror/store/<backend>.mirror`. They all
  implement the same outer grammar contract.

The mapping: the Shift registry is `@mirror/store/memory` restricted
to `Combinator`-typed entries (with AstNode + Bytes as v1
conveniences). spectral-db's storage layer is NOT a backend of
mirror's store — it's a peer system that consumes the same
fragmentation substrate. The naming overlap (`@mirror/store/...`
backend grammars vs spectral-db's storage) is historical; future
revisions may rename the mirror-store backend variants so the
substrate distinction is named in code.

The correct unification: the Shift registry IS the mirror store —
two names, one Layer-1 component. spectral-db shares fragmentation
with mirror's store; it does not share its definition.

---

## 6. The Shift dispatch wiring

Today, per `walker-contract.md` and `parser-as-prism-grammar.md`:

```rust
Combinator::Shift { grammar, body }:
  - In Checkpoint A/B (today): walk `body` structurally with no
    grammar resolution. Witness = Shift { grammar, body: walked body },
    offset = body's offset, success = body's success.
  - In Checkpoint C: resolve `grammar` via a registry. Recursive
    apply over the extracted body bytes.
```

The store makes Checkpoint C concrete. The dispatch:

```rust
fn walk_lift(
    grammar_ref: &Reference,
    body: &Combinator,
    source: &[u8],
    offset: usize,
    store: &dyn MirrorStore,
) -> WalkOut {
    // 1. Walk body at the current offset to extract the body span.
    let body_walk = walk_combinator_at(body, source, offset, depth + 1);
    if !body_walk.success {
        return WalkOut::dark_at(offset);
    }
    let body_bytes = &source[offset..body_walk.offset];

    // 2. Resolve the target grammar.
    let target = match store.fetch(grammar_ref) {
        Some(Entry::Combinator(c)) => c,
        Some(_) => return WalkOut::dark_at(offset),     // wrong kind
        None => {
            // Layer-1 fallback: Shift walks body structurally (the
            // Checkpoint-A/B behavior). Useful before the store is
            // populated (e.g. during the boot of glass.mirror itself,
            // before @nl is loaded).
            return WalkOut {
                witness: Combinator::Shift {
                    grammar: grammar_ref.clone(),
                    body: Box::new(body_walk.witness),
                },
                offset: body_walk.offset,
                success: true,
            };
        }
    };

    // 3. Apply the target grammar to the body bytes.
    let sub_walk = walk_combinator_at(target, body_bytes, 0, depth + 1);
    if !sub_walk.success {
        return WalkOut::dark_at(offset);
    }

    // 4. Produce the Shift witness wrapping the sub-walk.
    WalkOut {
        witness: Combinator::Shift {
            grammar: grammar_ref.clone(),
            body: Box::new(sub_walk.witness),
        },
        offset: body_walk.offset,        // outer offset advanced by body
        success: true,
    }
}
```

### 6.1 Recursion handling

Each Shift descends into `walk_combinator_at` again with `depth + 1`.
The `MAX_DEPTH = 1024` bound from `walker-contract.md` §Termination
applies. Cyclic grammars are bounded by depth; pathological cycles
emit Dark at depth 1024.

### 6.2 Failure modes — Shift produces Dark

Three places Shift can fall through:

1. **Body fails** — the body extractor (`Until`, `Capture`, etc.)
   emits Dark. The outer Shift inherits Dark.
2. **Grammar ref not in store, AND no Layer-1 fallback** — in
   strict mode (the bootstrap's eventual default), an unresolved
   `@<ref>` is a hard failure. In permissive mode (today, while the
   store is still being built), the fallback walks body structurally.
3. **Target grammar produces Dark on body bytes** — the target
   grammar accepted some prefix but couldn't get past the rest. The
   outer Shift inherits the inner Dark.

### 6.3 The OID-preservation invariant under Shift

FP1's OID-preservation invariant (per `walker-contract.md`) said: for
the seed and `grammar.mirror`, the witness OID matches the seed OID.
With Shift dispatching through the store, the invariant extends:

For any `(@<ref>, source)` pair where the file declares `@<ref>`
and the file's bytes are the file's source, the witness OID of
`apply_h(store.fetch(@<ref>), source)` matches the store OID of
`@<ref>` (post-normalization, modulo the file's own content
addressing — they're different OIDs, but they're each other's
fixed point under the normalization).

This is the structural FP1 from §2 written in terms of the store.

### 6.4 What the F-2 commit does

F-2 — the next Mara tick after this spec — implements:

1. The `MirrorStore` trait (Rust, in `bootstrap/src/`).
2. The `Entry` enum (Rust, generated from the grammar declaration
   if time permits; otherwise hand-coded with a TODO to generate).
3. The `boot()` function with strict ordering and the failure
   modes from §5.3.
4. The Shift dispatch from §6.
5. Cross-validation: every `.mirror` file in the boot set loads
   without Dark fallthrough; the resulting Combinator trees'
   OIDs are stable across runs.
6. The Layer-2 FP1 assertion from §2.

F-2 does NOT implement:

- On-disk persistence for mirror's store (§7; separate from
  spectral-db's persistence). v1.5.
- LSP integration (separate spec).
- The `mirror boot --resolve` subcommand (separate tick).

---

## 7. Relationship to spectral-db

**The mirror store is NOT spectral-db.** It's a small Layer-1
component of mirror's bootstrap. spectral-db (task #43) is a
separate project that powers the garden, with its own scope:
distribution, deltas, conflict resolution, the MNESIA adapter,
and the read+write surface the systemic.engineering garden runs
on. Earlier drafts of this spec collapsed the two; that collapse
was wrong. The car is not the fuel.

What they share: **fragmentation as the content-addressed
substrate.** Both depend on `fragmentation`'s `Shard` + `Fractal`
primitives for content-OID computation and storage. Cut 1 (feature-
gate `dashmap`) and Cut 2 (split `Fragmentable` into
`ContentAddressed` + `TreeShaped`) land in fragmentation regardless
of which consumer triggered them — they benefit both.

What they don't share: the consumption pattern. Mirror's store is
consumed by the walker via `apply_h`, restricted to a small closed
set of entry types. spectral-db is consumed by the network, the
CLI, and the garden — with its own typed surface, replication
protocol, conflict-resolution machinery, and persistence model.

### 7.1 The mirror store's Entry surface (stable, small)

```rust
pub enum Entry {
    Combinator(Combinator),  // Shift registry; loaded grammars
    AstNode(AstNode),        // parsed source caches
    Bytes(Vec<u8>),          // raw source / opaque content
}
```

This is the full surface. The mirror store does NOT grow `Project`
/ `Crystal` / `Gestalt` / `Session` / `Eigenboard` variants — those
are spectral-db's content types, declared in spectral-db's own
crate on top of the same fragmentation primitives. The mirror
store stays Combinator-focused; that focus is the point.

New API surface beyond the six functions (`list_by_kind`,
`crystallize`, etc.) belongs in spectral-db, not here.

### 7.2 What spectral-db owns (out of scope for this spec)

The list, for honest naming — none of these belong in mirror's
store; all of them belong in spectral-db's own scope:

- **Distribution.** Multi-node replication; the protocol for
  eventually-consistent state across peers.
- **Deltas.** Snapshot-to-snapshot diff at content-addressed
  granularity; the bandwidth story for sync.
- **Conflict resolution.** Likely a kintsugi-tournament shape
  applied to data conflicts (distinct from file collisions in
  `kintsugi-tournament.md`'s scope).
- **The MNESIA adapter.** BEAM persistence — the garden's
  read-traffic backend.
- **The `spectral *` CLI surface.** `focus` / `project` / `split`
  / `shift` / `settle` dispatch into spectral-db's API, not into
  mirror's store.
- **Gestalt navigation, session state, eigenboard storage.**
- **The garden's read+write path.** The thing the public
  systemic.engineering surface is built on.

When spectral-db's spec lands (`spectral/docs/specs/spectral-db.md`),
this spec gets a one-line back-reference. They're peers.

### 7.3 Implication for the spec hierarchy

- `mirror/docs/specs/mirror-store.md` (this spec) — mirror's
  Layer-1 store. Small. Combinator-focused.
- `spectral/docs/specs/spectral-db.md` (to be written) —
  spectral-db's full surface: distribution, deltas, conflict
  resolution, MNESIA, CLI, the garden.
- `fragmentation/docs/…` — the shared substrate's surface,
  consumed by both.

Three artifacts. Three scopes. One shared dependency.

Task #43 stays a separate project. Earlier framing in this spec
that had it "collapse" or "absorb" into mirror's store retires
with this revision.

---

## 8. The minimal Rust surface — final count

After the store lands (F-2), what stays Rust?

Per `bootstrap-retirement-plan.md`'s end-state inventory, updated
for this spec:

```text
bootstrap/
├── Cargo.toml             (workspace; std default; fragmentation dep)
├── src/
│   ├── lib.rs             (~10 LOC: cfg + extern crate alloc)
│   ├── ast.rs             (~140 LOC, stable)
│   ├── exec.rs            (~30 LOC, behind `std` feature)
│   ├── git.rs             (~60 LOC, behind `std` feature)
│   ├── hash.rs            (~270 LOC; CoincidenceHash<5,5>)
│   ├── main.rs            (~250 LOC, CLI shell + diagnostics)
│   ├── spectral.rs        (~1100 LOC; evaluator + walker + store + seed)
│   ├── store.rs           NEW: ~150 LOC; MirrorStore trait + boot()
│   └── (RETIRED: content, render, tokenize, grammar, pipeline)
```

### What stays Rust — final

1. **Walker** — `walk_combinator_at` in `spectral.rs`. Real today.
2. **@io kernel** — `git.rs`, `exec.rs`. Behind `std` feature for
   no_std stretch.
3. **Store** — `store.rs` (this spec). New in F-2.
4. **Seed** — `prism_seed()` in `spectral.rs`. ~30 LOC after 4b.3.
5. **Hash** — `hash.rs`. The concrete D in matrix form. Stable.
6. **AST type** — `ast.rs`. The state type for H. Stable.
7. **Spectral primitives** — `compose_a`, `apply_h`, `eigen_d`,
   `Fold5`, `ContentOidPrism`. Stable.

### What retires (per `bootstrap-retirement-plan.md`, with store
context)

- `tokenize.rs` — retires in Tick 4c. With the store, the
  retirement completes: the structural-self walker arms for
  `IoBinding`/`MatchArm`/`SelectVariant`/`KeywordFormBody` become
  real Shift dispatches through `store.fetch(@<ref>)`. Today
  these arms are placeholders; with the store, they're full
  dispatches into the relevant grammar.
- `grammar.rs` — retires in Tick 4c. Its job (`parse_grammar`,
  `load_grammar`, `grammar_path_for_ref`, `grammar_ref_from_path`,
  `is_skip_word`) IS what `store.boot()` does. The keyword tables
  become combinator-data in the loaded grammars. `is_skip_word`
  becomes a charset declaration in `code/rust.mirror`.
- `Fold5::on_other` reducer — retires per 4c. With the store
  populated, the IoBinding/Match/Select arms produce canonical
  shapes; the catch-all dissolves.
- `content.rs`, `render.rs`, `pipeline.rs` — already retired in
  earlier ticks; unaffected by the store.

### LOC delta from this spec

| Change | LOC delta |
|---|---|
| New `store.rs` | +150 |
| Shift dispatch in `walk_combinator_at` (in `spectral.rs`) | +30 |
| `grammar.rs` retires (Tick 4c) | -232 |
| `tokenize.rs` IoBinding/Match/Select arms become real | net ~0 |
| `Entry` enum + Fragmentable impl | +50 |
| `fragmentation` as dep — no LOC, but +1 line in Cargo.toml | +1 |

Net: -2 LOC. The store costs nothing on net — it's the substrate
that lets `grammar.rs` retire.

### Binary size delta

| Configuration | TEXT | stripped total |
|---|---|---|
| Today (post-F-1, std) | ~344 KB | ~388 KB |
| + Tick 4c (grammar.rs retires) | -10 KB | -10 KB |
| + Store (this spec, F-2) | +5 KB | +5 KB |
| + fragmentation dep (in-memory only) | +15 KB | +15 KB |
| Net | ~354 KB | ~398 KB |

The fragmentation dependency adds ~15KB for the in-memory store
plus its serde+sha2 surface. Mirror was going to need sha2 anyway
for the OID computation; the actual new bytes are ~10KB of
`Fragmentable` + `ConcurrentStore` machinery. Justified by what
retires (the `grammar.rs` keyword-table machinery + the
walker's structural-self Shift fallback) and by the spectral-db
unlock.

---

## 9. Open questions

Six. The honest list, ranked by load-bearing weight.

### 9.1 Reference resolution semantics

Do we support relative references (`@./local`, `@../shared/foo`)
or only absolute ones (`@mirror/glass`)?

**Tension.** Absolute is simpler; relative is more useful for
project-local grammar declarations (a user's `myproject.mirror`
might want to declare `@./parser` for an internal grammar without
having to invent a globally unique `@<org>/<project>/parser` name).

**Cyclic-ref policy.** Strict-ordering boot rejects cycles. Lazy
resolution would tolerate them (a Shift to `@a` that resolves through
`@b` that shifts back to `@a` is fine if both bottom out via
`Until`/`Capture` boundaries). The strict ordering simplifies
reasoning but forbids genuine recursive grammars.

**Provisional answer.** v1: absolute only, no cycles. v1.5+:
relative references, with cycles allowed inside a single
declaration but not across files. Spec belongs in a follow-up
`grammar-reference-resolution.md`.

### 9.2 Cache invalidation under disk changes

How does the store learn that a `.mirror` file on disk has changed?

**Options.**

- **Per-boot rebuild.** Every `mirror` invocation re-runs `boot()`.
  Cheap when the boot set is small (~10 files); expensive when it
  grows.
- **mtime caching.** The on-disk store records file mtimes; boot
  reloads only changed files. Sensitive to clock skew; misses
  semantic-equivalent edits (whitespace changes that don't change
  the OID).
- **Content-address caching.** The on-disk store keys by file
  content OID. Boot reads the file, hashes it, dispatches to the
  cached parse if the hash matches. Always correct; pays one hash
  per file per boot.

**Provisional answer.** Per-boot rebuild for v1 (the boot set is
small). Content-address caching for v1.5 once the boot set grows.
mtime caching: never (clock-skew bugs).

This is THE most load-bearing open question for the user-facing
performance story. Worth a careful answer before F-2 lands.

### 9.3 Persistence — in-memory only, or backed to disk

The store today: in-memory only. The brief mentions persistence to
`~/.spectral/` or fragmentation's `~/.coincidence` store.

**Trade-offs.**

- **In-memory only** — simpler. Lower-latency for the bootstrap.
  Every invocation rebuilds the store from scratch. Fine for
  ~10-file boot sets.
- **On-disk** — `~/.spectral/store/` keyed by OID. Survives across
  invocations. Pays serialization on insert + deserialization on
  fetch. Required for the LSP gestalt-tier cache (per Mara's
  `eigenboard.spec` — `tier gestalt { budget = 60MB }`).
- **Hybrid** — in-memory for the boot set; on-disk for project-
  level grammars and spectral-db's session state. The
  `FrgmntStore` in fragmentation already implements this exact
  hybrid.

**Provisional answer.** v1 (F-2): in-memory only via
`ConcurrentStore` (or its single-threaded equivalent post-Cut 1).
v1.5 (mirror-store side): optional hybrid via `FrgmntStore` for
large boot sets that outgrow memory. spectral-db's persistence
(session/crystal/gestalt entries) is OUT of scope here — those
live in spectral-db's own storage.

### 9.4 Concurrent access — read-during-write, snapshot semantics

The mirror bootstrap is single-threaded today. The LSP is not. When
the LSP serves "go to definition" through the store while a
file-watcher is reloading a grammar, what does the LSP see?

**Options.**

- **Lock-free reads, single-writer.** `dashmap`'s default. Readers
  see either the pre-insert state or the post-insert state, never
  a torn read.
- **Snapshot-at-fetch.** The store hands out `Arc<Entry>` clones;
  the reader keeps its snapshot until it's done. Writers don't
  affect in-flight readers.
- **Pessimistic locking.** A global RwLock guards the whole store.
  Slow; not what mirror wants.

**Provisional answer.** Lock-free reads via fragmentation's
`ConcurrentStore` (post-Cut 1 cleanup). Single-writer via the
store's `&mut self` API (the boot loop holds the unique reference;
the LSP's file-watcher pipeline serializes its writes).

### 9.5 Type-erased fetch vs typed fetch

`fetch(@<ref>) -> Option<&Entry>` is type-erased. The caller does:

```rust
match store.fetch(reference!("@mirror/glass")) {
    Some(Entry::Combinator(c)) => c,
    Some(_) => panic!("expected Combinator"),
    None => panic!("missing"),
}
```

Alternative: `fetch::<Combinator>(@<ref>) -> Option<&Combinator>`,
typed. Cleaner at call sites; requires either reflection or a
per-variant `FromEntry` trait.

**Provisional answer.** Type-erased for v1 (matches today's
`Entry` enum). Typed via a `FromEntry` trait for v1.5 if call sites
get noisy. The Cut 2 split of `Fragmentable` makes the typed form
cleaner: `ContentAddressed::content_oid` is variant-agnostic;
`TreeShaped` is variant-specific.

### 9.6 The exact split of "fragmentation cleanup"

The brief asked: "in fragmentation, in mirror, or both?"

**Provisional answer.**

- **Cuts 1 (feature-gate dashmap) and 2 (split Fragmentable):**
  in fragmentation. They land as a paired fragmentation/mirror PR
  on `reed/v1-floor` of both repos.
- **Cut 3 (rename Fractal::Fractal):** in fragmentation, as a
  follow-up tick. Not blocking F-2.
- **Mirror-side glue** (`Entry` enum, `Reference` type,
  `MirrorStore` trait, `ContentAddressed` impl for `Entry`): in
  mirror, in F-2.

The fragmentation cleanup PR can land first (no mirror dependency);
F-2 then depends on the cleanup.

---

## 10. What the existing specs need updated to match this one

Not in this tick's scope, but flagging for the follow-up:

1. **`mirror-store.md`** — this file. Replaced.
2. **`parser-as-prism-grammar.md`** — §"Bootstrap loop" reads as
   two-layer (seed + meta-glass + downstream grammars). Add a
   pointer to this spec's §1 for the three-layer naming and §2 for
   the FP1 reframe. FP1's load-bearing equation moves from
   `combinator_tree_oid(&seed) == combinator_tree_oid(&meta_glass)`
   (Layer-1 acceptance) to the Layer-2 store-mediated form.
3. **`walker-contract.md`** — §Shift's "Checkpoint C" footnote
   becomes a pointer to this spec's §6. The "registry" is the
   store; the wiring is here.
4. **`bootstrap-retirement-plan.md`** — Tick 4c's `grammar.rs`
   retirement gets the store as its successor. End-state file list
   gains `store.rs`. The Cargo.toml dependency table gains
   `fragmentation`.
5. **`spectral/docs/specs/spectral-db.md`** (to be written, by the
   spectral side) — task #43 is a separate project; its spec
   declares the distribution, delta, conflict-resolution, and
   MNESIA surface, and notes that it shares fragmentation with
   mirror's store. NOT a collapse.

None of those updates happen in this tick. They're follow-up work
once F-2 lands.

---

## 10.5 Canonical on-disk location

When the store is materialized to disk (the persistent backend; the
in-memory backend has no path), it lives at a fixed, user-scoped
canonical path:

```
~/.mirror
```

This is the single agreed-on default across consumers. The shape:

- A **bare fragmentation repo** (per `fragmentation/src/frgmnt_store.rs`).
  Content-addressed; not a working tree.
- Per-user, not per-project. Multiple project checkouts share one
  store. Same OID → same blob → one copy on disk.
- Stable across nixpkgs / toolchain bumps. Independent of `$PWD`,
  `$CARGO_HOME`, and `target/`.

No override env var is defined at this altitude (no `MIRROR_HOME`).
The path is one fact; consumers (CLI, CI, lenses) read `~/.mirror`
directly. If a future tick demands relocation, the override lands
then — not preemptively.

This matters for **CI cache stability**. GitHub Actions caches
`~/.mirror` between runs keyed on shard / boot / spec content hashes
(per `kintsugi-ci-v0.1.md` and `.github/workflows/kintsugi.yml`). The
build doesn't re-download or re-materialize crystals that were already
resolved on a previous run. Recognition #43 — mirror IS a
content-addressed build system — wires through CI exactly here.

The bootstrap binary doesn't read this path today (the on-disk
backend isn't wired into the bootstrap dispatcher — see §6). When it
is, this is the path.

---

## 11. References

- `mirror/docs/specs/parser-as-prism-grammar.md` — the Combinator
  surface this spec stores; the FP1 equation this spec reframes;
  the Shift combinator this spec dispatches.
- `mirror/docs/specs/walker-contract.md` — Mara's F-1 contract; the
  per-variant byte consumption that the store dispatches through.
- `mirror/docs/specs/bootstrap-retirement-plan.md` — the end-state
  this spec contributes to (post-Tick-4c).
- `mirror/docs/specs/combinator-optimization.md` — normalization
  before OID hashing; the property the store's OID equality
  depends on.
- `mirror/bootstrap/src/spectral.rs` (F-1 head, commit `62b8650`)
  — the Combinator enum, `prism_seed()`, `walk_combinator_at`,
  `combinator_tree_oid`, `normalize`.
- `fragmentation/Cargo.toml` — the substrate's feature flags.
- `fragmentation/src/fragment.rs` — `Fragmentable`, `Fractal<E, H>`,
  `Blob`, `merge`, `Reconstructable`.
- `fragmentation/src/concurrent_store.rs` — `ConcurrentStore<N, H>`;
  the in-memory store mirror consumes.
- `fragmentation/src/frgmnt_store.rs` — `FrgmntStore<N>`; the
  hybrid in-memory + on-disk store. Available to both mirror (if
  the boot set outgrows memory) and spectral-db (as one persistence
  option among MNESIA and others) — each as a separate consumer.
- `fragmentation/src/bounded_store.rs` — `BoundedStore<N>`; the
  LRU-bounded cache for the gestalt tier.
- `fragmentation/src/sha.rs` — `HashAlg` trait; pluggable hash.
- `fragmentation/src/cid.rs` — `Cid<H>`; self-describing content
  identifier (v1.5 envelope).
- Reed memory: `project-mirror-compile-staircase` — the order to
  land these moves in.
- Mara memory: `eigenboard.spec` — the three-tier cache budget
  the store backs (eigenvalue: 12.8MB; gestalt: 60MB; vector:
  300MB).

---

*Same shape. Three layers. The Shift registry IS the mirror store —
two names, one Layer-1 component. spectral-db shares fragmentation
with the mirror store; it does not share its definition.
fragmentation is the substrate they both stand on. The thing the
types don't say: a shared substrate is not a shared artifact.*

*Apache-2.0.*
