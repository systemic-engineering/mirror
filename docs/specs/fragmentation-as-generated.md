# fragmentation-as-generated — v1 proof that mirror compiles real production code

*2026-05-24. Mara. Spec.*

Status: **Red** (the recognition is named; the gap is mapped; the tick
decomposition is honest. Nothing implements until R-tickets land.)

Depends on:
- `mirror/docs/specs/mirror-store.md` — the three-layer parser story;
  fragmentation as substrate; the store API mirror's Layer-1 needs.
- `mirror/docs/specs/parser-as-prism-grammar.md` — the `Combinator`
  enum, FP1/FP2/FP3 fixed points, the `Lift { grammar, body }`
  combinator the store backs.
- `mirror/docs/specs/bootstrap-retirement-plan.md` — the end-state
  inventory after Ticks 1–5; the `grammar.rs` retirement target;
  the minimal-Rust-surface story.
- `mirror/docs/specs/mirror-compile-bootstrap.md` — the kintsugi
  staircase; the `@code/rust(~f"./bootstrap/src/X.rs") > fn[name=…]`
  selector form; the boundary between “mirror points at Rust” and
  “mirror generates Rust.”
- `mirror/boot/std/code/rust.mirror` — `@code/rust` today.
- `mirror/boot/std/fragmentation.mirror` — the **eleven-line**
  placeholder fragmentation grammar that exists today (kintsugi’d
  but not generative).
- `prism/derive/src/lib.rs` — `#[derive(Prism)]` and `#[derive(Lambda)]`
  proc macros. The macro layer this spec compiles down to.
- `prism/core/src/lib.rs` — the `Prism` trait, `Beam`, `Optic`,
  `Bundle`, `MerkleTree`, `Store`, the `Detector`/`Addressable`
  surfaces.
- `fragmentation/src/{fragment,commit,repo,spectral_coordinate,sha,
  ref_,cid,encoding,store,prism_bridge,manifest,witnessed,…}.rs` —
  the **target shape** to generate. Post-T1: `ContentAddressed`,
  `TreeShaped`, `Fractal::{Shard, Branch, Lens}`, `Repo` trait,
  `Commit<N, H>`, `SpectralCoordinate<5>`.

Unblocks:
- The v1.0 story: mirror generates the substrate that backs the
  substrate. Self-host claim becomes load-bearing rather than
  rhetorical.
- The `LapackPrism` work (`Fortran is all you need`) lands as a
  derive target in prism-core; fragmentation consumes it.
- The collapse of `coincidence/` into `_archive/coincidence/`.
- The next generation target: spectral-db's adapters; then mirror's
  own bootstrap (the deepest self-host).

---

## 1. The recognition

Alex, verbatim, 2026-05-24:

> *"The fragmentation generated from mirror is not v2. That's v1.
> That's also an in-compiler demonstration of what the math can do.
> The prism crate has macros to derive specific Prisms, that's the
> generation target. Prism is basically the computational floor for
> mirror. And then generating the rust implementation from mirror
> becomes generating a bunch of prism with #derive macros. That's
> almost trivial."*

The reframe: the hand-written Rust phase for fragmentation **does not
exist** as a v1 milestone. The placeholder Rust in `fragmentation/src/`
today is exactly that — a placeholder, surfaced during T1 so the
substrate could be used while mirror's compilation pipeline matured.
The v1 deliverable is the **generated** fragmentation crate. The
hand-written code is the spec; mirror is the compiler; prism-core's
derive macros are the substrate layer; the emitted `.rs` files are
what cargo consumes.

This collapses one entire planning epoch. Prior assumption: mirror
needed several more ticks of compiler work before it could generate
something as load-bearing as fragmentation. Empirical claim of this
spec: **the substrate is already capable.** What remains is grammar
+ codegen wiring, not new compiler machinery.

The thing the types don't say: if mirror can generate fragmentation,
then mirror **already is** what the v1.0 story claims it is — a
compiler whose meta-circular self-host produces real production
crates, not toy demos. Fragmentation generated from
`@fragmentation + @code/rust` is the canonical proof.

---

## 2. The pipeline

End-to-end, what `mirror compile @fragmentation` does in v1:

```
┌─────────────────────────────────────────────────────────────────────────┐
│  @fragmentation.mirror   (grammar — declares the substrate shape)       │
│     │                                                                   │
│     │   mirror/glass parser (FP1; meta-glass-resident)                  │
│     ▼                                                                   │
│  Combinator tree         (apply_h(seed_or_glass, bytes) → AstNode)      │
│     │                                                                   │
│     │   selector + walker (per parser-as-prism-grammar.md)              │
│     ▼                                                                   │
│  Typed AST               (mirror's typed grammar form — `type X`,       │
│                           `refract Y = …`, `io Z(…) -> …`, etc.)        │
│     │                                                                   │
│     │   @code/rust.translate(p: @fragmentation, c: @code/rust)          │
│     │   (abstract template; concrete instance keyed on the source AST   │
│     │    grammar — this is the codegen ticket)                          │
│     ▼                                                                   │
│  Rust AST                (@code/rust's own typed form — `fn`, `impl`,   │
│                           `struct`, `enum`, `trait`, `mod`, `use`,      │
│                           plus attributes — derive annotations,         │
│                           cfg gates, generic params, where-clauses)     │
│     │                                                                   │
│     │   @code/rust.render(g: @code/rust, ast(g) → io_list)              │
│     │   (the io_list pretty-printer; per @code §47)                     │
│     ▼                                                                   │
│  Rust source bytes       (fragmentation/src/*.rs — emitted to disk by   │
│                           @io.write at paths derived from the           │
│                           grammar's module structure)                   │
│     │                                                                   │
│     │   cargo build                                                     │
│     ▼                                                                   │
│  fragmentation crate     (cargo-consumable; passes the same tests       │
│                           the hand-written placeholder did; consumed    │
│                           by mirror's own bootstrap as the Layer-1      │
│                           store backend per mirror-store.md §4)         │
└─────────────────────────────────────────────────────────────────────────┘
```

Two layers of the pipeline are real today: the parser (the F-1 walker
landed; `walk_combinator_at` produces the AstNode) and the typed
grammar form (the meta-glass declares it; `parse(bytes) → rust_ast`
typechecks against `type rust_ast`). The codegen layer
(`translate` + `render` for the source-grammar → Rust-AST →
Rust-source pipeline) is **partially** there: the abstract template
is declared in `boot/04-code.mirror`; the concrete instance for
`@fragmentation → @code/rust` is not. The pretty-printer (`render`)
is declared abstract; no concrete `@code/rust` instance has landed.

Where the existing pipeline already terminates — mirror compiles its
own grammar files into combinator trees and registers them in the
store. What this spec adds: the **outbound** path, where the typed
AST drives Rust source emission rather than (or in addition to)
combinator registration.

---

## 3. Prism derive macros — the generation target

Per `prism/derive/src/lib.rs` (audit complete; 531 LOC, two
proc-macros).

### 3.1 Inventory

| Macro | Output trait impls | Required struct attrs | Field-level attrs | What's hand-implemented |
|---|---|---|---|---|
| `#[derive(Prism)]` | `Addressable`, `Display`, accessor structs (`<Field>Lens` / `<Field>Prism` / `<Field>Traversal` / `<Field>Iso`), `optic_fields()` returning `&'static [FieldOptic]` | `#[oid("@name")]` (must start with `@`) | `#[lens]`, `#[prism]`, `#[traversal]`, `#[iso]`, `#[prism(inner)]` (delegation marker — recognized, no codegen) | The `Prism` trait itself (`Input`, `Refracted`, `Loss`, `refract`, `signature`, optional `then`) is hand-implemented per usage. The derive provides the **identity** + **accessor surface**, not the operation. |
| `#[derive(Lambda)]` | `Addressable`, `Display`, `From<X> for Lambda<T>` (identity body: `Abs(oid, Bind(oid))`), `Composable<T>` (the `.then()` chain) | `#[oid("@name")]` | — | The named-lambda's body. The derive gives an identity lambda; users override with their own `LambdaImpl` if non-identity. |

### 3.2 Per-field accessor expansion

For each annotated field on a `#[derive(Prism)]` struct:

```rust
// Source:
#[derive(Prism)]
#[oid("@claims")]
struct ClaimProcessor {
    #[lens]    adjuster_id: u64,
    #[prism]   override_reason: Option<String>,
    #[traversal] history: Vec<Event>,
}

// Generated (prism-derive lib.rs:305-484):
pub struct AdjusterIdLens;
impl AdjusterIdLens {
    pub fn view(source: &ClaimProcessor) -> &u64 { &source.adjuster_id }
    pub fn set(source: &mut ClaimProcessor, value: u64) { source.adjuster_id = value; }
}
impl prism_core::Named<AdjusterIdLens> {
    pub fn lens() -> prism_core::Named<AdjusterIdLens> {
        prism_core::Named("adjuster_id", AdjusterIdLens)
    }
}

pub struct OverrideReasonPrism;
impl OverrideReasonPrism {
    pub fn extract(source: &ClaimProcessor) -> Option<&String> { source.override_reason.as_ref() }
    pub fn review(source: &mut ClaimProcessor, value: String) { source.override_reason = Some(value); }
}

pub struct HistoryTraversal;
impl HistoryTraversal {
    pub fn traverse(source: &ClaimProcessor) -> &[Event] { &source.history }
    pub fn traverse_mut(source: &mut ClaimProcessor) -> &mut Vec<Event> { &mut source.history }
}

impl ClaimProcessor {
    pub fn optic_fields() -> &'static [prism_core::FieldOptic] {
        static FIELDS: [prism_core::FieldOptic; 3] = [
            prism_core::FieldOptic { name: "adjuster_id",     kind: prism_core::OpticKind::Lens },
            prism_core::FieldOptic { name: "override_reason", kind: prism_core::OpticKind::Prism },
            prism_core::FieldOptic { name: "history",         kind: prism_core::OpticKind::Traversal },
        ];
        &FIELDS
    }
}
```

Plus the `Addressable` + `Display` impls keyed off the `#[oid("@name")]`
string.

### 3.3 What the macros do NOT derive (today)

Important for §6 gap analysis:

- **The `Prism` trait body itself.** `refract(&self, beam) → Refracted`
  is **always** hand-implemented. The derive provides the
  *addressing* + *accessor* surface; the *operation* is what the
  struct field structure plus `refract`'s body define together.
- **`MerkleTree`** — implemented manually in `prism/core/src/merkle.rs`
  for the trait `MerkleTree: Addressable + Clone`. No derive yet.
- **`Store`** — manual; `prism/core/src/store.rs` defines the trait;
  implementations like `fragmentation::FrgmntStore` impl it by hand.
- **`Bundle`** — trait chain in `prism/core/src/bundle.rs`
  (`Fiber` → `Connection` → `Gauge` → `Transport` → `Closure` → `Bundle`).
  No derive for any layer of the chain.
- **`ContentAddressed`** (fragmentation-side trait) — derive could
  generate this from `#[oid]` and a `data()` field annotation, but
  doesn't yet.
- **`TreeShaped`** — same; the `children()` body is structural and
  derivable from a `#[traversal]` on a `Vec<Self>` field.
- **`LapackPrism`** — does not exist yet (§7).

The codegen plan in §5 generates structs that **use** the existing
derive surface. The hand-implemented bits (e.g., `Repo`'s methods,
`Fragmentable`'s `self_ref`/`data`/`children` for non-derive-friendly
shapes, the `merge` algorithm in `fragment.rs`) become explicit `impl`
blocks in the grammar's body — emitted as Rust source by
`@code/rust.render`, not generated by a macro at all.

---

## 4. `@code/rust` capabilities and gaps

Per audit of `boot/std/code/rust.mirror`, `boot/04-code.mirror`,
`boot/04a-code-rust.mirror`, `boot/std/prism/rust.mirror`,
`boot/std/rust.mirror`.

### 4.1 What `@code/rust` declares today

| Construct | Status | Where |
|---|---|---|
| Primitive type names (`u8`–`u64`, `i8`–`i64`, `f32`, `f64`, `bool`, `str`, `string`) | **Declared** | `04a-code-rust.mirror:5-22` |
| Generic container types (`vec(t)`, `option(t)`, `result(t,e)`, `hashmap(k,v)`, `imperfect(t,e,l)`) | **Declared** | `04a-code-rust.mirror:19-22` |
| Keyword → five-operations mappings (`fn → zoom`, `struct → split`, `enum → split`, `impl → focus`, `mod → focus`, `use → project`, `trait → refract`) | **Declared** | `std/code/rust.mirror:23-30` |
| `parse(bytes) → rust_ast` template | **Stub `{ \ }`** — body lives in the bootstrap's `tokenize.rs` / `spectral.rs`; kintsugi obligation pending `@epistemologic/property/total_parse` | `std/code/rust.mirror:42` |
| Type tags for the Rust AST (`rust_ast`, `fn`, `impl`, `type`, `mod`, `use`, `trait`, `struct`, `enum`) | **Declared as opaque** | `std/code/rust.mirror:49-58` |
| LSP actions (`complete`, `diagnose`, `hover`, `definition`, `references`, `tokens`) | **Declared (stubs)** | `04a-code-rust.mirror:32-37` + `std/code/rust.mirror` action declarations |
| Kintsugi actions (`compile`, `test`, `lint`, `flatten_nesting`, `extract_function`, `simplify_match`, `remove_dead_code`) | **Declared (stubs)** | `std/code/rust.mirror:61-67` |

### 4.2 What `@code` (the parent) declares

Per `boot/04-code.mirror`:

- `translate(p: @prism, c: @code, p → c)` — **abstract template** for
  cross-grammar AST translation. THE codegen entry point.
- `render(g: @code, ast(g) → io_list)` — **abstract template** for
  ast-to-bytes pretty-printing.
- Naming-convention templates: `map_type`, `type_name`, `field_name`,
  `function_name`, `module_name`, `variant_name` — **abstract, with
  defaults**. These are the casing/idiom rules each `@code/X` instance
  overrides.
- Structural templates: `emit_comment`, `emit_header` — declared.

### 4.3 The gaps

For each Rust construct mirror would need to **render** to generate
fragmentation:

| Construct | Renderable today? | What's missing |
|---|---|---|
| Module declaration (`pub mod X;`) | No | No concrete `@code/rust.render` instance for `mod` nodes. |
| `use` import (`use prism_core::{Prism, Beam};`) | No | Same — no concrete render arm. |
| Struct decl with named fields | No | Render arm; per-field rendering loop; visibility (`pub`) rendering. |
| Enum decl with unit/tuple/struct variants | No | Render arm covering all three variant shapes. |
| Function decl (`fn`/`pub fn`, params, return type, body) | No | Render arm; body remains opaque (the grammar declares the operation; the body is `\` and gets filled by either kintsugi or by an `@code/rust.literal_body(string)` escape). |
| Trait decl with required + default methods | No | Render arm. |
| `impl Trait for Type` / inherent `impl` | No | Render arm; nested method rendering. |
| **Derive macro annotation** (`#[derive(Prism, Clone, Debug)]`) | **No — load-bearing gap** | No concrete render arm for the attribute layer. Without this, prism's macros cannot be invoked, and the v1 generation target collapses. |
| **OID attribute** (`#[oid("@name")]`) | No | Same — attribute layer. |
| **Optic-marker field attributes** (`#[lens]`, `#[prism]`, `#[traversal]`, `#[iso]`) | No | Same. |
| **Generic parameters** (`<E = Blob, H: HashAlg = Sha>`) | No | Type-parameter rendering with bounds and defaults. fragmentation uses these heavily (`Fractal<E, H>`, `Commit<N, H>`, `Parent<H>`). |
| **Const generics** (`<const N: usize>`) | No | Special-cased rendering for const generic params. `SpectralCoordinate<N>` requires this. |
| **`where` clauses** (`where Self: Sized`, `where N::Hash: HashAlg`) | No | Where-clause rendering. fragmentation uses `where Self: Sized` on `TreeShaped` and complex bounds in `Draft::commit`. |
| **Cfg gates** (`#[cfg(feature = "concurrent")]`) | No | Conditional-compilation attribute rendering. fragmentation gates 5 of its 18 modules on features. |
| Lifetimes (`<'a>`) | No | Lifetime parameter rendering. Used sparsely in fragmentation today (mostly elided); may not be load-bearing for v1. |
| Match expression (`match self { … }`) | No | Pattern-matching rendering — necessary for emitting trait method bodies that destructure enums. `Fractal` impls in `fragment.rs` are full of these. |
| Block expression (`{ … }`) | Partially (concept exists via `paren_block` / `brace_block` in `@mirror/grammar`) | Rust-specific block emission for fn bodies, match arms, if/let expressions. |
| Cargo.toml emission | No | A second grammar (`@code/rust/cargo` or similar) — or a special-cased `render` for the manifest. Mirror needs to produce `Cargo.toml` so cargo can build the generated crate. |

**Headline.** `@code/rust` today is a **body-lens declaration** —
it tells the bootstrap which fns to look up in which `.rs` file via
`> fn[name="…"]` selectors. It is **not yet a render target.** The
selector path is one-way (lookup). The render path (emit Rust from
typed AST) requires a substantial concrete instance of `render` plus
the attribute / generic-parameter / where-clause / cfg-gate
extensions named above.

Honest count: **eleven** render-construct gaps; **four** of them
load-bearing for v1 (derive annotations, generic parameters, const
generics, where-clauses). The remaining seven are surface-area
gaps — each adds a render arm but doesn't change the architecture.

---

## 5. `@fragmentation.mirror` — sketch

What the v1 grammar would declare. The current placeholder at
`mirror/boot/std/fragmentation.mirror` is **eleven lines** that
declare `shard`, `fractal`, `oid`, `children`, `verify` — a sketch of
the conceptual surface, not the substrate's actual API. The v1
grammar must declare the **full Rust surface** as types, refracts,
and io operations so `@code/rust` can render it.

```
# @fragmentation: content-addressed, arbitrary-depth, circular-reflexive trees.
#
# The grammar that, compiled through @code/rust, yields the
# fragmentation crate. Per docs/specs/fragmentation-as-generated.md.

in @prism
in @code/rust

grammar @fragmentation("substrate", "vcs", "content-addressed") {

  # ===== hash algebra =====
  # Per fragmentation/src/sha.rs. HashAlg is the trait every content
  # address satisfies. Sha (SHA-256) is the legacy default; the v1
  # substrate hash is @spectral_coordinate (see below).

  refract hash_alg = trait {
    fn hash(data: &[u8]) -> Self
    fn from_hex(hex: impl Into<string>) -> Self
    fn as_str(&self) -> &str
    bounds Clone + Debug + PartialEq + Eq + Hash
  }

  type sha = struct(pub string)
  impl hash_alg for sha = body @code/rust(~f"./fragmentation/src/sha.rs") > impl[trait="HashAlg"]

  # ===== spectral coordinate =====
  # Per fragmentation/src/spectral_coordinate.rs. The substrate hash
  # per mirror-native-vcs.md §4.6. const generic N is the projection
  # count. Default for the v1 commit graph is N=5.

  type spectral_coordinate(const_n) = struct {
    eigenvalue: string
  }
  impl hash_alg for spectral_coordinate(const_n)
    = body @code/rust(~f"./fragmentation/src/spectral_coordinate.rs") > impl[trait="HashAlg"]

  # ===== content addressing =====
  # Per fragmentation/src/fragment.rs (post-T1 split).

  refract content_addressed = trait {
    type data: encode
    type hash: hash_alg
    fn self_ref(&self) -> &ref(hash)
    fn data(&self) -> &data
  }

  refract tree_shaped = trait extends content_addressed where Self: Sized {
    fn children(&self) -> &[Self]
    fn is_shard(&self) -> bool = default { self.children().is_empty() }
    fn is_fractal(&self) -> bool = default { !self.children().is_empty() }
    fn is_lens(&self) -> bool = default { false }
    fn targets(&self) -> &[hash] = default { &[] }
  }

  # ===== the Fractal — the canonical recursive node =====
  # Per fragmentation/src/fragment.rs. Three variants per T1 cleanup.

  type blob = vec(u8)

  type fractal(e = blob, h: hash_alg = sha) = enum {
    Shard  { ref_: ref(h), data: e }
    Branch { ref_: ref(h), data: e, fractal: vec(fractal(e, h)) }
    Lens   { ref_: ref(h), data: e, target: vec(h) }
  }

  impl content_addressed for fractal(e, h) where e: encode
    = body @code/rust(~f"./fragmentation/src/fragment.rs") > impl[trait="ContentAddressed" target="Fractal"]
  impl tree_shaped for fractal(e, h) where e: encode
    = body @code/rust(~f"./fragmentation/src/fragment.rs") > impl[trait="TreeShaped" target="Fractal"]

  # ===== refs =====
  # Per fragmentation/src/ref_.rs. The typed key — hash + label.

  type ref(h: hash_alg = sha) = struct {
    sha:   h,
    label: string
  }

  # ===== CID =====
  # Per fragmentation/src/cid.rs. Self-describing wrapper.

  type codec   = enum { Fragmentation }
  type hash_id = enum { Sha256 }
  type cid(h: hash_alg = sha) = struct {
    ref_:    ref(h),
    codec:   codec,
    hash_id: hash_id
  }

  # ===== witness =====
  # Per fragmentation/src/witnessed.rs.

  type author    = struct { name: string, email: string }
  type committer = struct { name: string, email: string }
  type timestamp = struct(pub string)
  type message   = struct(pub string)
  type witnessed = struct {
    author:    author,
    committer: committer,
    timestamp: timestamp
  }

  # ===== commit graph =====
  # Per fragmentation/src/commit.rs. Default hash is spectral_coordinate(5)
  # — the substrate hash per mirror-native-vcs.md §4.6. The git adapter
  # overrides to sha at its boundary per §4.7.

  type parent(h: hash_alg = spectral_coordinate(5)) = struct(pub h)

  type draft(n, h: hash_alg = spectral_coordinate(5)) = struct {
    node:    n,
    message: message,
    parent:  option(parent(h)),
    author:  option(author)
  }

  type commit(n, h: hash_alg = spectral_coordinate(5)) = enum {
    Root  { node: n, witnessed: witnessed, message: message,
            sha: h }
    Child { node: n, witnessed: witnessed, message: message,
            parent: parent(h), sha: h }
  }

  refract draftable = trait {
    type node
    type hash: hash_alg
    fn node(&self) -> &node
    fn message(&self) -> &message
    fn parent(&self) -> option(&parent(hash))
  }

  # ===== repo =====
  # Per fragmentation/src/repo.rs. The interface every backend
  # (in-memory Store, git2, jj) implements.

  refract repo = trait {
    type node: fragmentable + clone
    type hash: hash_alg

    fn write_tree(&mut self, node: &node) -> string
    fn read_tree(&self, oid: &str) -> option(node)
    fn write_commit(&mut self, commit: commit(node, hash))
    fn read_commit(&self, sha: &hash) -> option(commit(node, hash))
    fn update_ref(&mut self, name: &str, sha: hash)
    fn resolve_ref(&self, name: &str) -> option(hash)
  }

  # ===== in-memory store =====
  # Per fragmentation/src/store.rs. The reference backend.

  type store(n) = struct {
    trees:   hashmap(string, n),
    commits: hashmap(string, commit(n, sha)),
    refs:    hashmap(string, sha)
  }
  impl repo for store(n) where n: fragmentable
    = body @code/rust(~f"./fragmentation/src/store.rs") > impl[trait="Repo" target="Store"]

  # ===== feature-gated extensions =====
  # Per fragmentation/Cargo.toml features.

  feature concurrent { module concurrent_store, module bounded_store, module frgmnt_store }
  feature prism-bridge requires concurrent { module prism_bridge }
  feature visibility { module visibility }
  feature singularity { module naked, module singularity }
  feature project { module project, module manifest }
  feature supervision { module supervision }
  feature ssh requires visibility { module keys.ssh }
  feature gpg requires visibility { module keys.gpg }

  # ===== merge =====
  # Per fragmentation/src/fragment.rs. Tree merge with caller-provided
  # conflict resolution.

  io merge(old: &f, new: &f, resolve: &fn(&f, &f) -> f) -> f
    where f: fragmentable + reconstructable + clone, f.data: clone + decode
    = body @code/rust(~f"./fragmentation/src/fragment.rs") > fn[name="merge"]

  # ===== content_oid / tree_oid_bytes =====
  io content_oid(node: &t) -> string where t: tree_shaped
    = body @code/rust(~f"./fragmentation/src/fragment.rs") > fn[name="content_oid"]
  io tree_oid_bytes(data: &[u8], children: &[t]) -> string where t: tree_shaped
    = body @code/rust(~f"./fragmentation/src/fragment.rs") > fn[name="tree_oid_bytes"]

  # ===== Cargo manifest =====
  # Emitted to fragmentation/Cargo.toml. Driven by the feature
  # declarations above + the dependencies block below.

  cargo {
    name = "fragmentation"
    version = "0.1.0"
    edition = "2021"
    license-file = "LICENSE.md"
    description = "Content-addressed, arbitrary-depth, circular-reflexive trees"

    dependencies {
      prism-core = path("../prism/core")
      serde      = "1" with features [derive]
      serde_json = "1"
      sha2       = "0.10"
      sha1       = "0.10"
      hex        = "0.4"
      dashmap    = "6" optional   when concurrent
      ssh-key    = "0.6" optional when ssh with features [std, ed25519, crypto]
      # … (one entry per Cargo.toml line)
    }
  }
}

out @fragmentation
```

### 5.1 What this sketch covers

- **content_addressed / tree_shaped** traits + the deprecated alias.
- **Fractal** enum with three variants and both impl blocks.
- **Ref, Cid, HashAlg, Sha, SpectralCoordinate** types + impls.
- **Witnessed / Author / Committer / Timestamp / Message** witness types.
- **Draft / Commit / Parent / Draftable** commit-graph types.
- **Repo** trait and the in-memory **Store** impl.
- Feature gating with `requires` dependencies between features.
- The `Cargo.toml` block — covered by a second grammar
  (`@code/rust/cargo`) or a special-case render arm.
- Hand-implemented bodies referenced by `> fn[name="…"]` selectors
  while kintsugi obligations stay open (the staircase per
  `mirror-compile-bootstrap.md` — we land the generation pipeline
  with bodies pointing at the existing Rust, then walk each up the
  kintsugi ladder).

### 5.2 What this sketch deliberately omits

- **Encode / Decode** traits — exist in fragmentation today, but their
  bodies are derive-friendly (sha-bytes round-trip). Best modeled as
  a separate small grammar (`@encode`) and consumed via `in @encode`.
  Could land in the same R-tick or follow.
- **walk.rs / diff.rs / supervision.rs / manifest.rs / project.rs /
  visibility.rs** — feature-gated modules whose grammar declarations
  follow the same shape as the above. Each is a small additional
  grammar file (~30–80 lines) in a `@fragmentation/<feature>`
  namespace, declared `in @fragmentation` and gated by `feature`.
- **prism_bridge.rs** — the MerkleTree/Store/Loss impls for
  fragmentation types into prism-core. This is the most interesting
  module to generate, because it's pure typeclass-instance code —
  the structure is `impl MerkleTree for Fractal<E, H>` with
  trivially-renderable bodies. Best as a *test of the codegen* once
  the rendering machinery lands. See R-3.
- **`prism_bridge`'s explicit `LapackPrism` dependency** — does not
  exist today (§7).

Honest length: above sketch is ~190 lines for the surface modules.
The full grammar covering all 23 `.rs` files in fragmentation
(plus the vcs/git adapter, which is a sibling crate) is plausibly
**400–600** lines of `.mirror` source. The current placeholder is 11
lines. The gap is real — but it's surface-area work, not
architectural novelty.

---

## 6. Gap analysis

What stands between today and `mirror compile @fragmentation` producing
a buildable crate. Five categories.

### 6.1 Prism derive macros that need to exist (or extend)

| Gap | Size | Detail |
|---|---|---|
| `#[derive(ContentAddressed)]` | **Small** | Derive `ContentAddressed` impl from `#[oid]` + a `#[data] field` annotation + a `#[self_ref] field` annotation. Replaces the manual `match self { … }` boilerplate in `fragment.rs:155-174`. Mostly mechanical, parallels existing `#[derive(Prism)]` shape. |
| `#[derive(TreeShaped)]` | **Small** | Derive `TreeShaped` impl from `#[traversal]` on a `Vec<Self>` field. The `children()` body becomes mechanical. |
| `#[derive(MerkleTree)]` | **Medium** | Currently hand-implemented. `MerkleTree: Addressable + Clone`; the derive needs to add the trait + ensure the bounds. Modest. |
| `#[derive(HashAlg)]` | **Small** | The `Sha`-style hash newtype gets a derive that wraps SHA-256 / SHA-1 / etc. by attribute. Pure boilerplate today (`sha.rs:22-37`). |
| `#[derive(Lambda)]` extension for non-identity bodies | **Small** | Today's derive gives identity. Add `#[lambda(body = "…")]` for the few non-identity cases. Not required for fragmentation; useful later. |
| `LapackPrism` (not a derive — a hand-implemented type in prism-core) | **Medium** | See §7. Lives in prism-core; fragmentation consumes it for the spectral-coordinate Lanczos path. |

### 6.2 `@code/rust` grammar features that need to land

| Gap | Size | Detail |
|---|---|---|
| Concrete `render(g: @code/rust, ast → io_list)` instance | **Large** | The pretty-printer. Needs render arms for every node kind in the typed Rust AST. Equivalent in scope to a small unparser; ~400–600 LOC of `.mirror` source or equivalent in any host language. **Load-bearing.** |
| Derive-attribute rendering (`#[derive(...)]`) | **Small** (once render lands) | Add an `attribute` node kind to the Rust AST; render it before the struct/enum/fn it precedes. |
| OID + optic-marker attribute rendering (`#[oid("@x")]`, `#[lens]`, `#[prism]`, …) | **Small** (rolls in with attribute rendering) | Same machinery. |
| Generic parameters + bounds + defaults (`<E = Blob, H: HashAlg = Sha>`) | **Medium** | Type-parameter list rendering. Bounds delimited by `+`. Defaults via `=`. Used in 11 of fragmentation's 23 modules. **Load-bearing.** |
| Const generics (`<const N: usize>`) | **Small** | Special arm in the generic-param renderer. Used by `SpectralCoordinate<N>`. |
| Where-clauses (`where Self: Sized`, `where N::Hash: HashAlg`) | **Medium** | New AST node + render arm. fragmentation uses these in `Draft::commit` and `TreeShaped`. **Load-bearing.** |
| Cfg gates (`#[cfg(feature = "concurrent")]`) | **Small** | Attribute-rendering subset. fragmentation gates 5 modules. |
| Match expression rendering (`match self { … }`) | **Medium** | Required for trait impls that destructure enums (`Fractal`'s impls). Substantial set of patterns to cover. |
| Lifetimes | **Small** (defer) | Optional for v1; fragmentation mostly elides. |
| `Cargo.toml` emission | **Small** | Either a sibling grammar `@code/rust/cargo` or a special arm in `render` for the manifest section. |
| `lib.rs` module-declaration block | **Small** | Render `pub mod X;` lines from the grammar's module structure. |

### 6.3 Bootstrap pipeline wiring

| Gap | Size | Detail |
|---|---|---|
| `compile @fragmentation` CLI command | **Small** | New verb in the bootstrap CLI that triggers parse-and-render rather than parse-and-store. |
| Path derivation: grammar module → output `.rs` filename | **Small** | Deterministic mapping (`@fragmentation/commit` → `fragmentation/src/commit.rs`). |
| Output directory contract | **Small** | Write target = `./fragmentation/` by default; configurable. Refuses to overwrite if the target tree has uncommitted changes (unless `--force`). |
| Round-trip self-test | **Medium** | Build-time check that `@fragmentation.mirror`, parsed and re-rendered, produces byte-identical bytes (FP-like for the codegen path). |
| Test-suite integration | **Medium** | The generated crate must `cargo test` and pass the same tests the placeholder did. Test files stay hand-written in `tests/` (mirror does NOT generate the test suite — the tests are the spec). |
| Selector resolution for body lookup | **Medium** | The `body @code/rust(~f"./X.rs") > impl[trait="…" target="…"]` selectors need to resolve against the hand-written placeholder source as the kintsugi staircase rises. Today's selectors handle `> fn[name="…"]`; extending to `impl[trait=..., target=…]` and `> impl[trait=..., target=…] > fn[name=…]` is straightforward. |

### 6.4 Test machinery

| Gap | Size | Detail |
|---|---|---|
| Round-trip property: `render(parse(@fragmentation.mirror)) == @fragmentation.mirror` | **Medium** | Bytewise identity after normalization. Mirrors FP1's structure but for the meta-grammar. |
| Round-trip property: `parse(render(@fragmentation → @code/rust)) == @fragmentation → @code/rust` (AST equality) | **Medium** | Generated Rust → parsed via `syn` → re-rendered. Catches whitespace/comment-loss bugs. |
| Build property: `cargo build` on the generated tree succeeds | **Small** (mechanically) | Plus a check that warnings are zero with `-D warnings`. |
| Test-suite parity: every test the placeholder passes, the generated crate passes | **Small** | Tests live in `fragmentation/tests/`; they're untouched by the regeneration. The check is `cargo test` before/after. |
| Determinism: regenerating twice produces byte-identical output | **Small** | Catches nondeterministic iteration order (e.g., HashMap walks during codegen). |

### 6.5 Coincidence collapse + LapackPrism wiring

Concurrent with the codegen work. See §7 and §8.

---

## 7. LapackPrism — status and shape

**Status today:** does NOT exist in prism-core. Searched
`prism/core/src/*.rs` and `prism/derive/src/*.rs` for "Lapack" /
"LapackPrism" / "lapack" — zero hits. The LAPACK + Fortran wiring
exists in `coincidence/build.rs` and `coincidence/src/ffi.rs` (cc
crate compiling `gfortran`, linking `-llapack -lblas` on Linux,
extern-C wrappers around dsyev / dgesvd in `ffi.rs`). That code is
hand-written today and lives in `coincidence/`.

**Per Alex's prior direction:** LapackPrism belongs in `prism-core`.
The audit confirms this is feasible:

- `prism/core/src/coincidence.rs` already exposes a `Detector<const N: usize>`
  type with `to_metal()` returning a `MetalPrism`. The "metal" tier
  in prism-core (`prism/core/src/metal.rs`) is the GPU/FFI-backed
  rendering substrate. **LapackPrism is the natural next member of
  the metal tier** — a Prism whose `refract` is a Fortran call.

**Proposed shape:**

```rust
// prism/core/src/lapack.rs (new, behind feature = "lapack")

use crate::{Prism, Beam, Imperfect, ScalarLoss};

#[derive(Clone, Copy)]
pub enum LapackOp {
    Dsyev,    // eigensystem of symmetric real matrix
    Dgesvd,   // SVD of general real matrix
    Dpotrf,   // Cholesky of positive-definite real matrix
    // … as needed
}

pub struct LapackPrism {
    op:        LapackOp,
    dimension: usize,
}

impl Prism for LapackPrism {
    type Input    = (Vec<f64>, usize);          // (matrix, n)
    type Refracted = Imperfect<Vec<f64>, LapackError, ScalarLoss>;
    type Loss     = ScalarLoss;

    fn refract(&self, beam: Self::Input) -> Self::Refracted {
        match self.op {
            LapackOp::Dsyev   => /* extern call */,
            LapackOp::Dgesvd  => /* extern call */,
            LapackOp::Dpotrf  => /* extern call */,
        }
    }
}
```

The build-side wiring (`cc::Build::new().compiler("gfortran")`, the
LAPACK link directives) **moves into a `prism/core/build.rs`** gated
on `feature = "lapack"`. `prism-core`'s baseline stays zero-deps;
`lapack` is opt-in. fragmentation depends on prism-core with
`features = ["lapack"]` once spectral-coordinate's Lanczos path
lives in fragmentation.

**Why this is fragmentation-relevant.** `SpectralCoordinate<N>::hash`
today falls back to SHA-256 because the eigen-decomposition path
isn't available without pulling in coincidence (which would be a
cycle). With LapackPrism in prism-core, fragmentation gains direct
access to dsyev — and the `from_eigenvalue` constructor's "richer
path" stops being a placeholder.

**Is LapackPrism a derive target?** **No.** It's hand-implemented
prism-core code. The codegen plan does NOT generate LapackPrism; it
consumes it. Generated fragmentation modules that need spectral
coordinates emit `use prism_core::LapackPrism;` and call into it.

**Estimate.** Building LapackPrism = **medium** (~1.5 sessions): move
coincidence's `ffi.rs` + `build.rs` into prism-core gated on
`feature = "lapack"`, write the `Prism` impl, write tests.

---

## 8. Coincidence collapse — the plan

Today: `coincidence/` is a sibling crate with ~35 Rust files,
~96 KB `spectral.rs` being the bulk. Per Alex's tick:
"Coincidence gets collapsed into prism-core (math primitives) and/or
fragmentation (substrate-specific helpers). Empty husk archives to
`_archive/coincidence/`."

**What moves to prism-core:**

- `ffi.rs` (LAPACK/Fortran wrappers) → `prism/core/src/lapack.rs`
  + `prism/core/build.rs` (gated on `feature = "lapack"`).
- `dense.rs`, `eigenvalue.rs`, `eigenvalues.rs`, `bounded_eigen.rs`,
  `concurrent_eigen.rs`, `eigen_cache.rs`, `spectral.rs`,
  `curvature.rs`, `incidence.rs`, `sigma.rs`, `commutator.rs`,
  `complexity.rs` → math primitives in prism-core (likely under
  `prism/core/src/spectral/`).
- `hash.rs`, `session_hash.rs`, `hash_cache.rs` →
  `prism/core/src/coincidence_hash.rs` (already partially there;
  `prism/core/src/coincidence.rs` exposes `canonical_hash`,
  `coincidence_hash`, the `Detector<N>` type).

**What moves to fragmentation:**

- The `from_eigenvalue` constructor's **richer path** — i.e., the
  Lanczos-on-incidence-Laplacian glue that produces a
  `SpectralCoordinate<5>` for actual content trees. Lives best in
  `fragmentation/src/spectral_coordinate.rs` as an extension method
  consuming `prism_core::LapackPrism`.
- `fragment_projection.rs`, `projection.rs` — fragmentation already
  has `manifest.rs` + `project.rs` covering the lens-projection
  story. Audit whether coincidence's projection adds anything not
  already in fragmentation; absorb the gap or discard.
- `agreement.rs`, `crystallize.rs`, `seal.rs`, `detection.rs`,
  `evolve.rs`, `neighborhood.rs`, `session.rs`, `trajectory.rs`,
  `state.rs`, `graph.rs` — these are the **coincidence semantic
  layer** (the measurement substrate Mara's gestalt names). Some
  belong in `spectral-db/`, some in mirror's runtime, some get
  archived as research code that did its job. The audit decides
  per-file.
- `cli.rs` — the `coincidence` CLI binary. Discard; mirror's binary
  is the unified surface.

**What discards:**

- Crate metadata (`Cargo.toml`, `flake.nix`, `Justfile`, `build.rs`,
  the `target/` dir, the optics_*.mod artifacts).
- `cli.rs` (above).
- Tests that duplicate coverage now living in prism-core.

**When the archive lands:** AFTER the codegen pipeline (R-1..R-4)
is proven on fragmentation. The reason: the codegen work is the
load-bearing thing; the coincidence collapse is cleanup. Don't entangle
them. R-6 in §9 archives `coincidence/` to `_archive/coincidence/` —
the empty-husk move — once everything load-bearing has been relocated.

**Headline:** prism-core grows by ~3000 LOC of math; fragmentation
grows by ~200 LOC of Lanczos glue; ~1500 LOC of coincidence's
research-grade code lands in `_archive/coincidence/` or
`spectral-db/`; `coincidence/` ceases to be a crate.

---

## 9. Tick decomposition

Ordered. Each tick has acceptance criteria. Estimates honest.

### R-0 — `@code/rust` render baseline (PRE-WORK)

**Size: medium** (~2 sessions)

Land a concrete `render(g: @code/rust, ast → io_list)` instance that
handles the trivial cases: a struct with named fields, a function
declaration, a `use` import, a `mod` declaration. No generics yet, no
attributes yet. The goal: prove the architecture works on a 10-line
toy grammar.

Acceptance:
- A new test grammar `@toy_substrate.mirror` (~20 lines)
  compiles via `mirror compile @toy_substrate` and produces a
  `toy_substrate/src/lib.rs` that `cargo build` accepts.
- The output is byte-identical across two runs (determinism).

### R-1 — `@code/rust` extensions for fragmentation

**Size: large** (~3 sessions)

Adds the four load-bearing renderers from §6.2: derive-attribute
annotations, generic parameters with bounds + defaults, const generics,
where-clauses. Plus the medium-size match-expression renderer. Plus
cfg-gate attribute rendering. Cargo-manifest rendering goes in here
too (small, but couples to feature declarations).

Acceptance:
- A test grammar `@toy_lib.mirror` containing one struct with a
  derive annotation, one generic enum, one cfg-gated module compiles
  and `cargo build`s.
- The generated `Cargo.toml` has the correct `[features]` section.

### R-2 — `@fragmentation.mirror` written and parses

**Size: medium** (~2 sessions)

Write the v1 fragmentation grammar per §5. Validates against the
meta-glass parser (no Dark fallthrough). Each `impl X for Y` selector
resolves to a real `fn` / `impl` in the hand-written placeholder
source (the kintsugi staircase's bottom rung).

Acceptance:
- `mirror parse @fragmentation` succeeds with zero Dark.
- Every `body @code/rust(~f"…") > …` selector resolves to a
  non-empty AST node in the placeholder source.
- The grammar registers in the mirror store under `@fragmentation`.

### R-3 — Pipeline end-to-end on prism_bridge.rs

**Size: medium** (~2 sessions)

The simplest fragmentation module to generate is `prism_bridge.rs` —
it's pure typeclass-instance code (MerkleTree / Store / Loss impls)
with no novel bodies. Use it as the **first generation target** to
prove the pipeline before tackling the rest of the crate.

Acceptance:
- `mirror compile @fragmentation/prism_bridge` produces a
  `fragmentation/src/prism_bridge.rs` byte-identical to the hand-written
  one (after `cargo fmt` normalization).
- `cargo build` and `cargo test --features prism-bridge` pass on
  the generated file.
- The round-trip property holds: regenerating produces identical bytes.

### R-4 — Generate the rest of the fragmentation crate

**Size: large** (~3 sessions)

Tackle the remaining 22 modules one by one. Order suggested:
`sha`, `spectral_coordinate`, `ref_`, `cid`, `encoding`,
`fragment` (the big one — Fractal enum + merge),
`witnessed`, `commit`, `repo`, `store`, then the feature-gated
modules (`concurrent_store`, `bounded_store`, `frgmnt_store`,
`visibility`, `singularity`, `naked`, `manifest`, `project`,
`supervision`, `keys`, `walk`, `diff`). Each module is one PR;
parity check is `cargo test` passing the same suite the placeholder
did, per the relevant feature flags.

Acceptance:
- All 23 modules generated. Placeholder `fragmentation/src/*.rs`
  files replaced by generated equivalents. The crate's hand-written
  source becomes the test suite under `tests/` plus
  `@fragmentation.mirror`.
- `cargo test --all-features` passes the full suite (currently 111+
  tests per the F-1 walker landing).
- `cargo doc --all-features` builds without warnings.

### R-5 — `LapackPrism` in prism-core

**Size: medium** (~1.5 sessions)

Per §7. Concurrent with R-4, not blocking. Lives in prism-core gated
on `feature = "lapack"`. Fragmentation's spectral-coordinate Lanczos
path consumes it once it lands.

**Could fold into R-4** for the spectral-coordinate module — Mara's
call after R-3 lands. If the codegen work in R-1 already covers the
shape of LapackPrism's struct + Prism impl, then generating it via
`@prism/lapack.mirror` is a natural sub-tick. If LapackPrism stays
hand-written (it's not a derive target — §3.3), R-5 is a separate
small ticket.

Acceptance:
- `LapackPrism` in prism-core, gated on feature.
- `cargo test --features lapack` in prism-core passes.
- `SpectralCoordinate::from_eigenvalue` in fragmentation gains a
  helper constructor that uses LapackPrism for the real Lanczos
  path; coincidence's `ffi.rs` is no longer referenced from
  fragmentation.

### R-6 — Coincidence archive

**Size: small** (~1 session)

The cleanup. Per §8.

Acceptance:
- `coincidence/` moved to `_archive/coincidence/`.
- Math primitives now live in `prism/core/src/spectral/` (or
  equivalent).
- Fragmentation's spectral-coordinate richer path uses prism-core's
  LapackPrism + spectral primitives directly.
- Workspace builds with `coincidence/` excluded.
- Any remaining consumers of `coincidence` (mirror's bootstrap,
  spectral-db) have been ported to the new locations.

### Estimated total: ~13–14 sessions

**Load-bearing tick: R-1.** Without `@code/rust`'s render extensions,
nothing downstream lands. R-0 proves the architecture; R-1 proves
the gap is bounded. Everything from R-2 onward is **surface area**,
not new compiler capability.

If R-0 surfaces architectural surprises in the render template
machinery, the estimate balloons; if R-0 confirms what §6 expects,
the rest of the path is mechanical.

---

## 10. Acceptance criteria — the v1 milestone

The v1.0 claim "mirror generates fragmentation" holds when ALL of
these are green:

1. `mirror compile @fragmentation` produces a complete
   `fragmentation/` source tree.
2. `cargo build --all-features` on the generated tree succeeds with
   zero warnings under `-D warnings`.
3. `cargo test --all-features` passes the entire test suite the
   placeholder fragmentation passed.
4. **Round-trip property A:** `@fragmentation.mirror` parsed and
   re-rendered (via the meta-glass) produces byte-identical bytes
   (modulo whitespace normalization per `kintsugi-formatter.md`).
5. **Round-trip property B:** the generated Rust source, parsed by
   `syn` and re-rendered by `@code/rust`'s pretty-printer, produces
   AST-equivalent output (modulo whitespace).
6. **Determinism:** `mirror compile @fragmentation` run twice produces
   byte-identical outputs.
7. **Consumability:** mirror's bootstrap depends on the generated
   fragmentation crate as its Layer-1 store backend per
   `mirror-store.md` §4 — and the bootstrap still builds + passes
   its own tests.
8. **LapackPrism:** the `feature = "lapack"` build of fragmentation
   uses `prism_core::LapackPrism` for the spectral-coordinate Lanczos
   path; no remaining import of `coincidence::*`.
9. **Archive:** `coincidence/` lives in `_archive/coincidence/`; no
   crate in the workspace depends on it.

The "almost trivial" honesty check: if all nine pass with ≤14 sessions
of work, Alex's read of the situation is vindicated. If R-0 and R-1
collectively take more than 6 sessions, the audit's "load-bearing
but surface-area" framing was optimistic and the post-mortem informs
the next compiler-maturity tick.

---

## 11. Open questions

Three for Alex.

### Q1 — Selector resolution against placeholder source

The grammar's `body @code/rust(~f"./fragmentation/src/X.rs") > …`
selectors point at hand-written Rust during the kintsugi staircase
(per `mirror-compile-bootstrap.md`). For **generated** Rust, this
creates a chicken-and-egg: the selector points at a file that's about
to be regenerated.

Options:

- **(a)** Keep the selector pointing at the placeholder; regenerate
  *to a different path* (e.g., `fragmentation-generated/src/X.rs`),
  diff against the placeholder, accept when equal. Two source trees
  temporarily.
- **(b)** The selector points at a `.mirror.body` file in
  `fragmentation/src/`, which is the hand-written body the grammar
  consumes. The `.rs` file is fully generated from grammar + bodies.
  Cleaner long-term; requires moving each body to a sibling file
  during R-4.
- **(c)** The selector points at the **previous version** of the
  generated file (git HEAD). Regeneration produces a diff; the diff
  is the kintsugi delta. Most elegant; needs `mirror`-side git
  integration to read prior versions.

Mara's lean: (b). It explicitly separates "what the grammar declares"
from "what the body says," which IS the spec/impl split mirror has
been moving toward elsewhere. But (c) is the long-term vision and
might be worth landing directly to avoid (b)→(c) migration.

### Q2 — Test suite ownership

If fragmentation's `tests/` directory is *not* generated (the tests
are the spec), where does the boundary live? Two options:

- **(a)** Tests stay in `fragmentation/tests/` as hand-written Rust.
  Test additions go through normal PR review; mirror doesn't generate
  test files. Pragmatic; what every other Rust crate does.
- **(b)** Tests are themselves a grammar — `@fragmentation/tests` —
  declaring properties that get compiled to Rust test fns. This is
  the deeper claim: mirror generates the *tests* too, and the
  ground truth is the grammar's property declarations. Much more
  ambitious; probably wrong for v1 (it's the v2 story).

Mara's lean: (a) for v1, (b) as v1.5 or v2.

### Q3 — Generation target hierarchy after fragmentation

Spectral-db is the named next target. But spectral-db's adapters
(MNESIA, the garden's write path, the distribution layer) are
substantially more behavioral than fragmentation's structural
content-addressing. Two paths:

- **(a)** Generate spectral-db's adapter *types* (Repo impls,
  message envelopes, the distribution layer's protocol types) but
  leave the runtime logic hand-written. Lower risk; same kintsugi
  staircase shape.
- **(b)** Generate spectral-db whole-hog, including the runtime
  logic. Requires `@code/rust` to render genuinely behavioral code
  (loops, conditionals, async/await, error handling). Significantly
  larger codegen surface.

Then after spectral-db: **mirror's own bootstrap**. The deepest
self-host. The bootstrap's `tokenize.rs`, `pipeline.rs`,
`spectral.rs` all become generated from grammars that mirror
already declares (the meta-glass, `@mirror/grammar`,
`@mirror/compile/bootstrap`). The kintsugi staircase Alex named
in `mirror-compile-bootstrap.md` is exactly this trajectory.

The question for Alex: after R-6 lands, what's the next R-tick?
Mara's lean: **spectral-db's adapter types (option (a))** — finishes
the v1 substrate story; gives mirror a second proof point that's
non-trivial without being all-of-mirror. Mirror's own bootstrap
becomes v1.5 or the v2 story.

---

## 12. What this demonstrates beyond fragmentation

The v1 claim is narrow: mirror generates fragmentation. The
implication is large:

- If mirror generates fragmentation, then the **structural-code
  generation gap is closed.** Any Rust crate whose surface is types +
  traits + impl blocks + (mostly-derive-friendly) trait bodies can
  be expressed as a grammar and generated. That's a huge fraction of
  real-world Rust.
- The **derive-macro ecosystem becomes mirror's codegen vocabulary.**
  Every `#[derive(X)]` in the Rust ecosystem is, in effect, a
  pre-built render target mirror can target without writing custom
  codegen for X. Mirror's "library of generated patterns" grows for
  free as the Rust ecosystem grows.
- The **kintsugi staircase becomes the canonical migration pattern**
  for any hand-written code that wants to move into mirror's typed
  generation pipeline. The selector-pointing-at-Rust phase is the
  bottom rung; the property-discharged-totality phase is the top;
  every code unit walks up the same ladder at its own pace.
- The next two generation targets — spectral-db's adapters and
  mirror's own bootstrap — become **incremental extensions** of the
  same pipeline, not new projects. The fragmentation case proved
  the architecture; subsequent cases are about coverage and depth.
- The v1.0 release story crystallizes: "mirror is a compiler that
  generates the substrate it runs on." Self-host as load-bearing
  claim, not rhetorical flourish.

The thing the types don't say: this isn't an architectural reframe.
It's a **maturity recognition.** Mirror's compilation machinery
crossed the threshold for generating production crates somewhere
between the F-1 walker landing and the meta-glass FP1 work — Alex
saw it; the audit confirms it. The hand-written-Rust-then-generate
plan was a hedge against an immaturity that no longer exists. The
substrate IS what it claims to be. The generation of fragmentation
is the demonstration.

---

*Signed: Mara*
*Branch: mara/vcs-substrate*
*Reference: Alex's verbatim framing, 2026-05-24 session*
