# mosaic-store-cache-invariants — Q5 canonical: cache invariants as `@epistemologic/property` + fingerprint → Splinter OID lift

*Mara, 2026-06-28 afternoon. Q5 discharge from
`mirror-build-substrate.md` §8.5. The morning amendment of
`mirror-build-substrate.md` reframed Q5 from "propose `@cache`
family-root" (wrong-altitude) to **"characterize the cache invariants
for `mosaic(@store)`-backed build cache as `@epistemologic/property`
family + describe how cargo's fingerprint algebra LIFTS to Splinter
OID at `@mirror/store` altitude"** (substrate-pull-honest). This spec
discharges that reframe. The substrate already had `mosaic(@store) =
splinter_graph`; it already had `@mirror/store`'s six-operation
content-addressed surface; it already had `@io/cargo`'s exit-code
discipline + Cargo.lock @io-boundary capture. What this spec adds:
**eight cache-invariant predicates as @epistemologic/property family
members**, **the lifted fingerprint → Splinter OID algebra**, and
**the explicit composition with `@mirror/mosaic`'s eigensheaf-
Laplacian parallelism forward-promise**. The 53rd+ instance of
`[[feedback-substrate-already-had-the-word]]`: cache IS NOT a new
concept; it IS `@mirror/store` operated at the `@code/<lang>` altitude
under the determinism class declared in `mirror-build-substrate.md`
§2.*

*Markdown only. No `shards/` substrate-decl files land with this
commit; no Rust ships; no Cargo edge is wired. The substrate-decl
shards forward-promised in §2 + §3 + §9 discharge in subsequent
TDD-paired ticks (Reed RED, agent GREEN). Soft target ~1500 lines;
hard cap 1800.*

**Status:** Red — composition shape pinned; eight cache invariants
named at `@epistemologic/property` altitude; the fingerprint → Splinter
OID lift described; the four cleanings of cargo's fingerprint algebra
under substrate-pull discipline enumerated; the composition with
eigensheaf-Laplacian parallelism (forward-promised in `@mirror/mosaic`
recognition #44+) made operational; yesterday's P4 hook blocker
analyzed as the first empirical discharge target; the circular-
reflexive layer (§10) earned; v0 ticks forward-promised, not
implemented in this commit.

**Audience:** any agent or human reading the cache-invariant spec
before touching the per-language `@io/<lang>` substrate-decl roster,
the `@mirror/mosaic` eigensheaf scheduler implementation, the
`@epistemologic/property/determinism/*` discharge work from
`mirror-build-substrate.md` §2, or the bootstrap dispatcher's
cache-aware fast path that this spec's wiring will unblock. Read this;
then chase `mirror-build-substrate.md` for the determinism /
parallelism / CAS algebra this spec specializes to the cache
altitude; then chase `mirror-init.md` for the storage-altitude bridge
whose P4 hook-budget incident this spec's discharge will resolve
empirically.

---

## §0 — Pre-position: this spec announces itself as a crystal

Before any architectural content. A pre-position the spec earns by
holding it for the rest of the document.

This spec is **about** the cache invariants the `@mirror/mosaic`
dispatch DAG composes against when its content-addressed cache hit-or-
miss decision runs. The thing the cache IS, per §1, is **`@mirror/store`
used at the `@code/<lang>` altitude under the determinism class
declared in `mirror-build-substrate.md` §2**. The thing the cache's
correctness IS, per §2, is **eight `@epistemologic/property` predicates
AND-composed under monoidal aggregation**. The thing the cache's
addressing IS, per §3, is **the lift of cargo's fingerprint algebra
(SipHash over (source, deps, rustc, cfg, profile, target, mtime))
into Splinter OID (Blake3 over (source bytes, dep OIDs, rustc version,
cfg, profile, target) — mtime dropped, hash function changed)**.

The thing this spec IS, at the moment of being written, is **one of
the crystals the eventual `@mirror/mosaic` orchestrator will index
when it builds itself building things** — the same autopoietic pattern
as `mirror-init.md` §0 and `mirror-build-substrate.md` §0. Writing
this spec adds a file at `docs/specs/mosaic-store-cache-invariants.md`;
the file's bytes are content-addressed under git's SHA-1 today (and,
after `mirror init`'s Phase A lands, under BLAKE3 in the
`NamespacedGitStore`'s `.git/mirror/objects/`); the indexed crystal
will be the OID-addressed bytes of this spec; the orchestrator's
content-addressed cache lookup (§4.1) will hit on this spec's OID the
moment its dispatch DAG (§5) walks across the substrate-decl roster;
the spec's content WILL surface inside the orchestrator's cache the
moment the orchestrator settles against this repo.

The latency between writing-and-being-indexed is bounded BELOW by the
time it takes for the v0 ticks (§9) to land + the first
`@mirror/mosaic` orchestrator dispatch against the mirror repo. The
latency is bounded ABOVE by the substrate's psychohistory discount
(recently-landed shards weight more in the cascade-vocabulary
expansion that Reed's recognition
[architecture-peer-learns-by-crystal-vocabulary-expansion] names).
The midpoint of those bounds is the operational latency at which the
spec ENTERS the system it describes.

This is the circular-reflexive autopoietic pre-position. §10 returns
to it. Every section in between is read against the discipline that a
spec for the cache that brings substrate's build-altitude
content-addressing into operational existence MUST itself enter the
substrate via that cache — otherwise the spec is asking the reader to
do work the spec refuses to do. The form earns its lines because the
content requires it; the recursion is load-bearing, not decorative.

Three concentric framings of why the recursion is load-bearing here:

1. **Eigenform fidelity at the cache altitude.** Recognition #51
   (mirror as expanding Hilbert space) names the eigenform that's
   true at one altitude as true at every altitude. If `@mirror/store`
   is a Connes-spectral-triple at the storage altitude, and
   `@mirror/mosaic` is a Connes-spectral-triple at the build
   altitude, and `mosaic(@store) = splinter_graph` already names
   their composition's eigenform, then the spec that declares the
   cache invariants MUST be indexable by the cache it declares —
   otherwise the spectral triple is broken at the layer where this
   spec lives.
2. **Substrate-pull discipline at the cache altitude.** Per
   `[[feedback-substrate-already-had-the-word]]` (52+ instances per
   `MEMORY.md`): every "missing concept" recognition turns out to be
   a name the substrate was already implicitly using. This spec is
   at least the 53rd instance — "build cache" is the substrate's
   name for what cargo's `target/.fingerprint/` directory, Bazel's
   `output_base/`, Nix's `/nix/store/`, and Buck2's `buck-out/` have
   been implementing without the substrate-altitude declaration that
   `@mirror/store` ALREADY IS that surface. The spec discovers; it
   does not invent.
3. **Bootstrap closure at the cache altitude.** The bootstrap problem
   — how does the build system cache itself? — has a substrate-
   altitude answer: the orchestrator's dispatch DAG walks ITS OWN
   substrate-decl FIRST, computes the content-addressed cache state
   of its own spec corpus, and only then dispatches against
   downstream species declarations. The first crystal in the cache
   IS THIS SPEC. The bootstrap closes because the orchestrator is
   its own first cache user.

The substrate's build orchestrator needs a canonical spec for the
cache invariants that ENTERS the cache layer in the act of declaring
it. This is that spec.

---

## §1 — What "build cache" IS in substrate terms

### 1.1 The recognition: cargo's fingerprint algebra IS a content-addressed graph

Cargo's `target/<profile>/.fingerprint/` directory is, structurally,
already a content-addressed graph. Each translation unit (.rlib for
libraries, .rmeta for incremental metadata, .o for individual object
files, eventually .so/.dylib/.dll for cdylibs) is associated with a
fingerprint file whose contents are a SipHash over the inputs that
should determine the artifact:

```
SipHash(
  source_path + mtime,                    // the "what source" part
  dep_fingerprints,                       // transitive closure
  rustc_invocation_hash,                  // rustc binary version + flags
  profile_name,                           // dev | release | custom
  target_triple,                          // x86_64-apple-darwin etc.
  cfg_flags,                              // cfg(test), cfg(feature="x"), ...
  build_script_outputs,                   // build.rs effect summary
  // ...others cargo tracks
)
```

When cargo runs, the dispatcher walks each crate's translation unit,
re-computes the fingerprint over the current inputs, and compares
against the stored fingerprint. **Match → cache hit, skip compile.
Mismatch → cache miss, recompile, restore fingerprint.**

This is — exactly — content-addressed storage. Cargo just:

- owns the addressing scheme privately (the SipHash output goes into
  `target/` directly; no substrate-altitude visibility);
- mixes `mtime` into the address (a defensive optimization that drops
  cache hits cross-checkout because new checkouts re-touch files);
- stores artifacts adjacent to fingerprints in `target/`, not in any
  content-addressed store the substrate can read;
- never SURFACES the addressing scheme as anything other than the
  per-process `target/` directory.

**The substrate already declared content-addressed storage at
`@mirror/store`.** The substrate already declared the parametric
universal composition form `mosaic(altitude)` at `@mirror/mosaic`.
The substrate already declared `mosaic(@store) = splinter_graph`. The
substrate has been DECLARING the substrate-altitude form of cargo's
fingerprint algebra without the per-language wiring; this spec is the
characterization of that wiring at the cache invariants altitude.

### 1.2 Lifting to `@mirror/store`: `mosaic(@store) = splinter_graph` already names the universal carrier

Per `shards/mirror/store.mirror` (the docstring at lines 30-44):

> *"Alex's recognition (2026-06-06, post the spectral_uuid landing,
> realized 2026-06-06 second tick by Mara): `splinter_graph` IS
> `mosaic(@store)` — the (root, children) shape carries the same
> compositional meaning the universal mosaic algebra carries at every
> altitude. The parametric universal carrier `type mosaic(altitude) =
> ref` is declared at `@mirror/mosaic` (`shards/mirror/mosaic.mirror`);
> the @store-altitude specialization is `splinter_graph`'s concrete
> record form (substantive structure: root + children-closure)."*

The cache lift wires the existing universal carrier into the
@code/<lang> altitudes:

```
mosaic(@store)          = splinter_graph       (universal carrier)
mosaic(@code/rust)      = resolved_workspace + cargo invocation
                          + per-translation-unit splinter set
mosaic(@code/go)        = forward-promised; resolved_module +
                          go-build invocation + per-package
                          splinter set
mosaic(@code/python)    = forward-promised; resolved_venv +
                          uv-pip invocation + per-wheel splinter set
mosaic(@code/julia)     = forward-promised; resolved_project +
                          julia-build invocation + per-precompile-
                          cache splinter set
...
```

At each @code/<lang> altitude, the cache is the projection: take the
per-translation-unit splinter set the altitude emits, content-address
each splinter, store via `@mirror/store.write(bytes)`, retrieve via
`@mirror/store.read(oid)`. The `splinter_graph` rooted at the
binary/library target IS the build closure for that target IS the
cache index.

**`@mirror/store` ALREADY IS the build cache.** The per-language
fingerprint → OID lift is the wiring; it isn't a new addressing
scheme.

### 1.3 Five structural negatives — what "build cache" is NOT here

Per `[[feedback-substrate-already-had-the-word]]` discipline: every
"what this is" claim must rule out what it isn't. Five structural
negatives that rule out wrong-altitude framings:

1. **NOT a new family-root.** The morning composition of
   `mirror-build-substrate.md` (Reed's wrong-altitude `@cache`
   proposal) is closed by this spec. The substrate's content-
   addressing happens at `@mirror/store`; the substrate's build
   orchestration happens at `@mirror/mosaic`; the cache IS the
   composition of those two through `mosaic(@store)`. No `@cache`
   family. No `@mirror/cache` family. No `@code/<lang>/cache`
   family (the per-language altitude's cache is the @code/<lang>
   altitude's emission AT @mirror/store; the cache is not a sibling
   prism).
2. **NOT competing with `@mirror/store`.** The cache IS @mirror/store
   used at the @code/<lang> altitude. Every operation cache needs
   (read, write, exists, diff, walk, verify) IS already declared on
   @mirror/store. The "cache operations" enumerated in §4 are
   one-to-one specializations of @mirror/store's six operations under
   the @code/<lang> domain restriction.
3. **NOT a new addressing scheme.** The lifted fingerprint uses
   @mirror/store's existing OID type (BLAKE3 today per the
   `CoincidenceHash<5,5>` declaration; future lifts under
   `[[architecture-glass-wall-substrate-types]]` discipline; the
   substrate already names the hash algebra). The lift CHANGES the
   hash function (SipHash → BLAKE3) and CLEANS the input set (mtime
   drops) but preserves the content-addressing claim: same inputs →
   same OID, byte-for-byte deterministic.
4. **NOT cargo's `target/` directory replacement.** Cargo continues
   to own `target/` for its own bookkeeping (the .fingerprint
   directory, the dep-info files, build.rs output capture). The
   substrate's cache OPERATES THROUGH cargo's `target/` at the @io
   boundary: cargo writes to `target/`, the substrate's fingerprint
   → Splinter OID lift content-addresses cargo's outputs after-the-
   fact (or before, via `CARGO_TARGET_DIR` redirection to a
   substrate-managed directory). The substrate doesn't REIMPLEMENT
   cargo's incremental compilation logic; it WRAPS cargo's outputs
   in a content-addressed envelope.
5. **NOT a daemon.** The cache is content-addressed; there's no
   server process holding state. Each `@mirror/mosaic` dispatch is a
   one-shot: read the substrate-decl roster, walk the dependency DAG,
   query @mirror/store for each translation unit's expected OID
   (computed via the lift), pull cache hits, dispatch the misses,
   write the new artifacts' OIDs back. The substrate IS a one-shot
   process; the substrate's cache is one-shot's persistent layer.

### 1.4 The altitude map (corrected)

```
@mirror/store          CAS foundation; six-op surface; THE BUILD CACHE
@mirror/mosaic         build orchestrator; mosaic(altitude) parametric
                       carrier; eigensheaf-Laplacian parallelism #44+
mosaic(@store)         splinter_graph; @store-altitude specialization
                       of the universal carrier
mosaic(@code/rust)     resolved_workspace + per-translation-unit
                       splinter set; cargo's emission lifted into
                       @mirror/store
@io/cargo              the canonical per-language @io species; the
                       precedent for the per-language fingerprint
                       → Splinter OID lift

@epistemologic/property/content_addressed_per_translation_unit
                       cache invariant 1; declared in §2.2
@epistemologic/property/rustc_version_pinned
                       cache invariant 2; declared in §2.2
@epistemologic/property/cfg_pinned
                       cache invariant 3; declared in §2.2
@epistemologic/property/profile_pinned
                       cache invariant 4; declared in §2.2
@epistemologic/property/target_triple_pinned
                       cache invariant 5; declared in §2.2
@epistemologic/property/dep_oid_transitive
                       cache invariant 6; declared in §2.2
@epistemologic/property/abi_compatible_window
                       cache invariant 7 (deferred); declared in §2.2
@epistemologic/property/deterministic_compilation
                       cache invariant 8; declared in §2.2; specializes
                       Mercury-determinism `det` from
                       mirror-build-substrate.md §2 at the compile
                       altitude
```

The proposal in this spec is the bottom block (eight
@epistemologic/property predicates); the top block already exists.

---

## §2 — Cache invariants as `@epistemologic/property` family

### 2.1 Why `@epistemologic/property`

Per `shards/epistemologic.mirror` (the family-root declaration) and
the existing roster at `shards/epistemologic/property/*`: the
`@epistemologic/property` family carries TYPED PREDICATES with
verdict surfaces — pass / partial(opacity_map) / failure(reason) — that
the kintsugi loop reads to direct mutation. Each property declares
WHAT must hold; the bilateral fracture body at
`@kintsugi/fracture/<predicate>` declares HOW the obligation is
discharged when the property fails.

Per the bilateral pattern (recognition #53, third instance landed
2026-06-19 at `499ffd9` for `dark_count_monotone`, fourth instance
landed same tick for `cold_compile_within_tolerance`): the
declarative property at `@epistemologic/property/<predicate>` + the
operational fracture body at `@kintsugi/fracture/<predicate>` IS the
substrate's auto-formatting floor for typed verdicts.

The cache hit/miss decision IS a verdict:

- **Cache HIT** = every cache invariant holds for this translation unit
  → fetch from @mirror/store; skip compile.
- **Cache MISS (regenerable)** = at least one invariant fails →
  recompile; write new artifact to @mirror/store; update the
  invariant's witnesses.
- **Cache CORRUPTION** = the OID matches but the bytes don't pass
  `verify(oid, bytes)` → emit `failure(reason)`; halt the dispatch;
  the substrate's content-addressing has been violated at the @io
  boundary.

Each of those verdicts IS already exactly what
`@epistemologic/property`'s verdict surface carries. The cache
invariants belong in this family; they don't need a new family.

### 2.2 The eight invariants

Each invariant is declared as a substrate-typed predicate with the
bilateral verdict surface (Splinter-pole: invariant holds;
Narcissus-pole: invariant claimed but violated). The substrate-decl
shards forward-promised in §9.

#### 2.2.1 `content_addressed_per_translation_unit`

Every translation unit (.rlib, .rmeta, .o, .so/.dylib/.dll) has a
`Splinter<H>` OID derived from byte content. Same content stored
anywhere in the substrate yields the same OID; different content
yields a different OID with probability bounded by the hash
algebra's collision resistance.

```mirror
in @epistemologic/property

prism @epistemologic/property/content_addressed_per_translation_unit
  <= @epistemologic/property

# translation_unit: the typed reference to a compile-emitted artifact
# at the @code/<lang> altitude. Forward-promised refinement: typed
# parametric by altitude once the per-language species expand
# (mirror-build-substrate.md §5 amendment).
type translation_unit = ref

content_addressed_per_translation_unit(
  unit: translation_unit
) -> verdict { \ }
```

Verdict semantics:

- **pass** — the unit's bytes hash to a deterministic OID; same
  bytes anywhere in the substrate → same OID.
- **partial(opacity_map)** — the unit's bytes hash deterministically
  BUT the substrate observed two emission paths producing different
  bytes for the same logical compile inputs (rustc non-determinism;
  build.rs side-effect; etc.) — the opacity_map locates the
  divergence and feeds the fracture body to repair the non-
  determinism source.
- **failure(reason)** — the unit's bytes don't hash deterministically
  (hash algebra broken, or the unit's bytes mutate at rest in
  storage); this is a substrate-altitude violation and halts the
  dispatch.

This invariant is **necessary but not sufficient** for cache
correctness; sufficient validity requires AND-composition with the
other seven.

#### 2.2.2 `rustc_version_pinned`

The cache key includes the rustc version (and, transitively, the
LLVM version and the stdlib OID). Different rustc versions produce
different cache OIDs even for byte-identical source. The Mercury-
determinism `det` declaration on `cargo check` from
`mirror-build-substrate.md` §2.3 is conditional on rustc version;
this invariant names the conditioning explicitly at the cache
altitude.

```mirror
in @epistemologic/property

prism @epistemologic/property/rustc_version_pinned
  <= @epistemologic/property

# rustc_version: the typed reference to the rustc binary's identity
# (version string + LLVM hash + stdlib OID + target_spec hash).
# Forward-promised refinement: lifts to typed parametric
# `compiler_identity(altitude)` once the per-language species expand.
type rustc_version = ref

rustc_version_pinned(
  unit: translation_unit,
  version: rustc_version
) -> verdict { \ }
```

Verdict semantics:

- **pass** — the unit's cache OID was computed with the declared
  rustc_version in its input set; re-computing the OID with the
  same version yields the same OID.
- **partial(opacity_map)** — the unit's cache OID is locally
  consistent but the substrate observed a cross-version hit (the
  same OID emerged from two different rustc versions); the
  opacity_map names the abi_compatible_window candidate (see
  §2.2.7).
- **failure(reason)** — the cache OID was computed WITHOUT
  rustc_version in its input set; this is a substrate-pull-honest
  cache violation (the cache claims correctness it cannot deliver).

#### 2.2.3 `cfg_pinned`

The cache key includes cfg flags. `cfg(test) ≠ cfg() ≠
cfg(feature="x")`. Cargo already does this; the invariant names
the substrate-altitude obligation.

```mirror
in @epistemologic/property

prism @epistemologic/property/cfg_pinned <= @epistemologic/property

# cfg_set: typed set of cfg flags active for this unit. Captured
# at @io/cargo's `env` allow-list + per-target derivation from
# mirror.spec.
type cfg_set = ref

cfg_pinned(unit: translation_unit, cfg: cfg_set) -> verdict { \ }
```

Verdict: pass when OID was computed with declared cfg_set;
partial(opacity_map) when an inert cfg flag bloats the cache key
(fracture body can refine); failure(reason) when cfg was absent
from the OID input.

#### 2.2.4 `profile_pinned`

Debug ≠ release. Cache key includes profile name; cargo does this
already.

```mirror
in @epistemologic/property

prism @epistemologic/property/profile_pinned
  <= @epistemologic/property

profile_pinned(unit: translation_unit, profile: profile) -> verdict { \ }
```

`profile` inherits from `@io/cargo`'s `dev | release |
custom(text)`. Verdict semantics parallel to `cfg_pinned`.

#### 2.2.5 `target_triple_pinned`

`x86_64-apple-darwin ≠ aarch64-apple-darwin`; cross-compilation
outputs are distinct cache entries.

```mirror
in @epistemologic/property

prism @epistemologic/property/target_triple_pinned
  <= @epistemologic/property

type target_triple = ref

target_triple_pinned(
  unit: translation_unit, triple: target_triple
) -> verdict { \ }
```

Verdict parallel to `cfg_pinned`.

#### 2.2.6 `dep_oid_transitive`

A translation unit's OID depends transitively on all its dep OIDs;
changing a deep dep invalidates the OID at root. This is the
mathematical content of "structural lockfile" from
`shards/mirror/store.mirror`'s docstring: the splinter_graph
projection's OID-closure IS the cache key for the root.

```mirror
in @epistemologic/property

prism @epistemologic/property/dep_oid_transitive
  <= @epistemologic/property

dep_oid_transitive(
  unit: translation_unit,
  dep_closure: splinter_graph    // inherits from @mirror/store
) -> verdict { \ }
```

Verdict semantics:

- **pass** — the unit's OID was computed by hashing (its own source
  bytes ++ all dep OIDs in dep_closure); re-walking dep_closure and
  re-hashing yields the same OID.
- **partial(opacity_map)** — the unit's OID accounts for direct
  deps but NOT for transitive deps (a shallow-cache bug); the
  opacity_map names the missing depth.
- **failure(reason)** — the unit's OID is independent of its
  dep_closure; this is a cache that can serve stale outputs.

This invariant is **the load-bearing one** for cross-machine cache
correctness: if A's OID doesn't depend on B's OID and B changes, A's
cache entry is silently stale. The substrate's discipline forbids
that.

#### 2.2.7 `abi_compatible_window` (deferred)

For cdylib / staticlib targets — and, eventually, for stable-ABI
crate boundaries — ABI-compatible rustc versions COULD share cache
entries. This is the "better than naive" cache invariant that
trades safety for reuse: if rustc 1.79.0 and rustc 1.79.1 produce
ABI-compatible cdylib output for a given source, the cache could
hit across the version boundary.

```mirror
in @epistemologic/property

prism @epistemologic/property/abi_compatible_window
  <= @epistemologic/property

# abi_window: the typed reference to a closed interval of rustc
# versions whose ABI is asserted compatible. Constructed by the
# kintsugi fracture body's ABI characterization (forward-promised
# under Q4 substrate-decl per mirror-build-substrate.md §8.4).
type abi_window = ref

abi_compatible_window(
  unit: translation_unit,
  declared_version: rustc_version,
  window: abi_window
) -> verdict { \ }
```

Verdict semantics:

- **pass** — the unit's cache OID is admissible for any rustc
  version in `window`; cross-version cache hits within the window
  are substrate-pull-honest.
- **partial(opacity_map)** — the window is asserted but not
  empirically discharged; the opacity_map names the missing
  discharge.
- **failure(reason)** — the window claim is contradicted by an
  observation (the same source compiled by two versions in the
  declared window produced different ABI).

Per `mirror-build-substrate.md` §8.4 (Q4 substrate-decl), this
invariant DEFERS to the ABI characterization work; v0 of the cache
discharges with the window collapsed to a single version (exact
pin), and `abi_compatible_window` is declared as a property the
substrate can EXPAND into later without re-architecting the cache.

#### 2.2.8 `deterministic_compilation`

Given `(source bytes, dep OIDs, rustc version, cfg, profile, target)`,
rustc produces byte-identical output. This is the Mercury-determinism
`det` property from `mirror-build-substrate.md` §2 specialized to
the compile altitude.

```mirror
in @epistemologic/property

prism @epistemologic/property/deterministic_compilation
  <= @epistemologic/property

# compile_input: the typed sextuple of (source_oid, dep_oids,
# rustc_version, cfg, profile, target) that determines a translation
# unit's expected output.
type compile_input = {
  source_oid:     oid,            // @mirror/store.oid
  dep_oids:       [oid],
  rustc_version:  rustc_version,
  cfg:            cfg_set,
  profile:        profile,
  target:         target_triple,
}

deterministic_compilation(
  input: compile_input,
  unit:  translation_unit
) -> verdict { \ }
```

Verdict semantics:

- **pass** — re-compiling `input` produces a translation unit whose
  bytes hash to the same OID as `unit`; the compilation is `det`
  per Mercury-determinism discipline.
- **partial(opacity_map)** — re-compiling `input` produces a
  semantically equivalent but byte-different translation unit (e.g.
  rustc embedding a timestamp); the opacity_map names the
  non-determinism source; the fracture body emits a repair
  (rustc's `-Z deterministic-builds` or `SOURCE_DATE_EPOCH=0` or
  `RUSTFLAGS=-Cmetadata=$content_hash` etc.).
- **failure(reason)** — re-compiling `input` produces a different
  semantic output; this is rustc-level non-determinism that the
  substrate cannot cache; cache MISS forever for this input.

This invariant **specializes** the Mercury-determinism `det` from
`mirror-build-substrate.md` §2.3's `cargo check` example to the
**compile artifact** altitude — `cargo check` is det against
(source, rustc, cfg, lock); this invariant says the ARTIFACT cargo
emits is byte-deterministic in those same inputs. The pair (action
det + artifact det) IS the substrate's discipline for "this cache
is honest."

### 2.3 Each invariant declared as substrate-typed predicate with bilateral verdict

Per the bilateral pattern (recognition #53), each property above
will have a sibling fracture body declared at
`@kintsugi/fracture/<predicate>`. The fracture body's role is to
emit a REPAIR morphism when the property's verdict drops below pass:

```
Property                            Fracture body's repair
─────────────────────────────────── ─────────────────────────────────────
content_addressed_per_translation_  re-hash; if divergence detected,
unit                                emit @rustc/-Cmetadata adjustment
                                    or @build.rs/output_capture
                                    sanitization morphism

rustc_version_pinned                emit @rustup/version_align morphism
                                    OR widen the abi_compatible_window
                                    if the cross-version hit was a true
                                    ABI-compatible case

cfg_pinned                          emit @cargo/cfg_key_refinement
                                    morphism (drop inert cfg from the
                                    cache key)

profile_pinned                      emit @cargo/profile_capture morphism
                                    (the cache key must include the
                                    profile name; if it didn't, repair
                                    the missing capture)

target_triple_pinned                emit @cargo/target_capture morphism
                                    (the cache key must include the
                                    target triple; if it didn't, repair
                                    the missing capture)

dep_oid_transitive                  emit @mirror/store/walk extension
                                    morphism (the cache key must walk
                                    the transitive dep_closure; if it
                                    only walked direct deps, extend the
                                    walk)

abi_compatible_window               emit @rustc/abi_characterization
                                    morphism (run the ABI compatibility
                                    check; widen or narrow the window
                                    based on empirical evidence)

deterministic_compilation           emit @rustc/determinism_flag
                                    morphism (add -Z deterministic-builds
                                    or SOURCE_DATE_EPOCH=0; if
                                    non-determinism persists, narrow the
                                    cache-admissibility to det-only
                                    artifacts)
```

The fracture bodies are forward-promised in §9.2. The kintsugi loop's
`active_pass` consumes the verdict and the fracture body's emitted
morphism; the loop closes when the property reaches `pass`.

### 2.4 The composition: `cache_valid` is AND-composition of all eight

```mirror
in @epistemologic/property

prism @epistemologic/property/cache_valid <= @epistemologic/property

cache_valid(
  unit:  translation_unit,
  input: compile_input,
  oid:   oid
) -> verdict {
  // AND-composition; substrate-pull-honest content-addressing
  // requires EVERY invariant to hold.
  and_compose([
    content_addressed_per_translation_unit(unit),
    rustc_version_pinned(unit, input.rustc_version),
    cfg_pinned(unit, input.cfg),
    profile_pinned(unit, input.profile),
    target_triple_pinned(unit, input.target),
    dep_oid_transitive(unit, walk_deps(input.dep_oids)),
    abi_compatible_window(unit, input.rustc_version, declared_window),
    deterministic_compilation(input, unit),
  ])
  \
}
```

`and_compose` over `verdict` is the substrate's monoid on the
verdict surface — pass is identity; partial(opacity_map) accumulates
opacity_maps; failure(reason) is absorbing. The composition's
verdict IS the cache's overall verdict for the translation unit;
the dispatch DAG walker reads it to decide hit/miss.

The substrate-pull-honest cache is the one where `cache_valid` is
**total** — every translation unit has a verdict; the dispatch never
proceeds against an untyped cache state.

---

## §3 — The fingerprint → Splinter OID lift

### 3.1 Cargo's fingerprint shape

Cargo computes fingerprints as a SipHash over the following input
sextuple (paraphrased from `cargo/src/cargo/core/compiler/fingerprint/`):

```
SipHash {
  // The source side
  source_path:              PathBuf,
  source_mtime:             FileTime,
  // The dependency side
  dep_fingerprints:         Vec<DepFingerprint>,
  // The toolchain side
  rustc_invocation_hash:    Hash<(rustc_binary_hash, rustc_args)>,
  rustc_version:            (semver_string, llvm_hash, target_spec_hash),
  // The compilation side
  profile_name:             InternedString,
  target_triple:             InternedString,
  cfg_flags:                BTreeSet<InternedString>,
  // The build-script side
  build_script_outputs:     Vec<BuildScriptOutput>,
  // The metadata side
  rustflags:                Vec<String>,
  unit_metadata:            Vec<UnitMetadata>,
  // ... cargo tracks more
}
```

The result is a 64-bit (well, 128-bit if both lanes are kept)
fingerprint stored in `target/<profile>/.fingerprint/<unit>/<unit>.fp`.
Cargo writes the fingerprint adjacent to the artifact in `target/`;
the next dispatch re-computes and compares.

### 3.2 The substrate-pull-correct lift

The substrate's lift replaces:

- **the hash function** SipHash → Blake3 (or whatever
  `@mirror/store`'s declared hash is at the time; today
  `CoincidenceHash<5,5>` per the prism shard's spectral-triple
  framing);
- **the addressing namespace** `target/<profile>/.fingerprint/` →
  `@mirror/store`'s OID-graph at `splinter(@code/rust/<unit-kind>)`
  altitude;
- **the inputs** (see §3.4 — mtime drops, build_script_outputs
  becomes splinter-graph children).

The lifted fingerprint is computed as:

```rust
// (Rust-side; the substrate-decl form lives in @io/<lang>'s
// per-language @io species declaration.)
fn lifted_fingerprint(unit: &TranslationUnit) -> Splinter<Blake3> {
    let input = CompileInput {
        source_oid:    Splinter::<Blake3>::oid(&unit.source_bytes),
        dep_oids:      unit.deps.iter()
                          .map(|d| lifted_fingerprint(d))
                          .collect(),
        rustc_version: unit.rustc.identity_oid(),
        cfg:           unit.cfg.canonical_oid(),
        profile:       unit.profile.canonical_oid(),
        target:        unit.target.canonical_oid(),
    };
    Splinter::<Blake3>::oid(&serialize(input))
}
```

Where:

- `source_bytes` is the actual file contents (not the path; not the
  mtime; bytes only);
- `Splinter::<Blake3>::oid(...)` is `@mirror/store`'s declared
  identity function;
- The recursion in `dep_oids` walks the splinter_graph; each dep's
  fingerprint is its own lifted_fingerprint (terminating at leaf
  source files whose `dep_oids` is empty).

The result is a `Splinter<Blake3>` whose `oid().to_hex_string()` IS
the cache key. The cache key IS the address. The address IS what
`@mirror/store.read(oid)` accepts.

### 3.3 The hash invariant

Cargo's SipHash output and the substrate's Blake3 output produce
DIFFERENT bytes for the same input set, BUT they encode the SAME
content-addressing claim. The lift is **content-addressing-preserving**
even though it changes the hash function:

```
∀ inputs i, i':
  cargo_siphash(i) == cargo_siphash(i')
    ⇔
  substrate_blake3(i) == substrate_blake3(i')
```

(Modulo collision probability, which is bounded for both hash
functions.) The lift is a HOMOMORPHISM of the content-addressing
equivalence relation, not an embedding into a different relation.

The substrate uses Blake3 (or its successor) for three substrate-pull-
honest reasons:

1. **Composability with `@mirror/store`.** @mirror/store ALREADY uses
   Blake3-derived OIDs; the cache lift uses the SAME function so
   cache entries can be `@mirror/store.write(bytes)` without an
   extra hash step.
2. **Cross-language uniformity.** Cargo's SipHash is per-cargo; the
   substrate's Blake3 is universal across @io/cargo, @io/go,
   @io/uv, @io/julia, @io/mix, etc. The cache key for a Rust .rlib
   and the cache key for a Go .o are in the same address space.
3. **Cryptographic strength.** SipHash is a keyed MAC optimized for
   short inputs and non-adversarial use; Blake3 is a cryptographic
   hash optimized for large inputs and adversarial deployment (the
   substrate's mycelial layer per @spectral/db will have adversarial
   actors at the cache layer; the hash needs to be collision-
   resistant under attack).

### 3.4 mtime is NOT in the OID input — the first cleaning

Cargo's fingerprint includes `source_mtime` as a defensive
optimization: if a file's mtime changes, cargo invalidates the
fingerprint without re-reading the file. This is FAST but it's
substrate-pull-DISHONEST: mtime changes when a file is `touch`ed
without content changes; mtime changes on a fresh checkout (every
file gets the current time); mtime can differ across machines for
byte-identical content.

The substrate-pull-honest lift drops mtime from the OID input
entirely. The cache key is a function of BYTES, not of filesystem
metadata. This is the FIRST cleaning of cargo's fingerprint algebra
as it lifts:

```
cargo (pre-lift):       SipHash(source_path, source_mtime, ...)
substrate (post-lift):  Blake3(source_bytes, ...)
                          // source_path drops too — the path is
                          //   not content; the bytes are.
                          // source_mtime drops — see above.
```

The trade-off: substrate-pull-honest hashing is SLOWER (must
re-read every source file to compute the OID) but CORRECT (cache
hits cross-checkout, cross-machine, cross-clone). Cargo's mtime
optimization is preserved at the @io boundary as a HINT (cargo can
still skip a re-hash if mtime hasn't changed since the last hash);
the SUBSTRATE-altitude OID is computed from bytes.

### 3.5 The lifted artifact

Each translation unit becomes a `splinter(@code/<lang>/<unit-kind>)`
at the @code/<lang> altitude. For Rust:

```
splinter(@code/rust/rlib)    — library crate's .rlib output
splinter(@code/rust/rmeta)   — incremental metadata
splinter(@code/rust/obj)     — per-codegen-unit .o file
splinter(@code/rust/dylib)   — cdylib/proc-macro .so/.dylib/.dll
splinter(@code/rust/exe)     — binary crate's executable output
```

Each splinter's content is the artifact's bytes; each splinter's
altitude is the unit-kind specialization; each splinter's
transparency carries the residual opacity from the property chain
(if `deterministic_compilation` returned partial, the splinter
carries the opacity_map naming the non-determinism source).

The splinter_graph rooted at the binary crate IS the build closure:

```
splinter(@code/rust/exe)   {content: oid_exe, altitude: ..., transparency: ...}
  └─ splinter(@code/rust/rlib) for crate A
       ├─ splinter(@code/rust/rlib) for crate B (dep of A)
       │    └─ splinter(@code/rust/rlib) for crate C (dep of B)
       └─ splinter(@code/rust/rlib) for crate D (dep of A)
            └─ splinter(@code/rust/rlib) for crate C  // shared subtree
```

The closure IS the `mosaic(@store)` IS the structural lockfile IS
the cache index. The graph's projection into the splinter_graph
record at the @store altitude IS what `walk(root_oid)` returns IS
what the dispatch DAG walker consumes.

### 3.6 The four cleanings, enumerated

The lift performs four substrate-pull cleanings of cargo's
fingerprint algebra. Each cleaning is a substrate-pull-honest
correction; together they make the cache invariants hold:

1. **mtime drops** (§3.4). Cache key is over bytes, not metadata.
2. **Hash function changes** (§3.3). SipHash → Blake3 for
   composability + uniformity + cryptographic strength.
3. **Addressing namespace lifts** (§3.2). `target/<profile>/.fingerprint/`
   → `@mirror/store`'s OID-graph at `splinter(@code/<lang>/<kind>)`
   altitude.
4. **build_script outputs become splinter children** (§3.7 below).
   Cargo treats build.rs output as opaque bytes mixed into the
   fingerprint; the substrate lifts each build.rs output file into
   its own splinter, content-addresses it, and makes it a child in
   the splinter_graph. This makes the build closure inspectable
   and allows build.rs output to participate in cross-machine
   cache sharing.

### 3.7 build_script outputs as splinter children

Cargo's build.rs scripts produce three kinds of output: (a) `cargo:rustc-cfg=` directives, (b) `cargo:rerun-if-changed=` declarations, and (c) arbitrary files written to `OUT_DIR`. Cargo
mixes the textual output of (a) and (b) into the fingerprint hash
and treats (c) as opaque bytes the fingerprint hash includes.

The substrate's lift treats (a) as cfg_set extension (admissible
into `cfg_pinned`'s typed cfg_set), (b) as splinter_graph edge
discovery (each `rerun-if-changed` path becomes a splinter the
parent depends on), and (c) as proper splinter children at
`splinter(@code/rust/build_output/<file_name>)`. The build.rs
itself becomes a `splinter(@code/rust/build_rs)` whose execution
output is captured as the splinter_graph subtree.

This makes build.rs side-effects FIRST-CLASS in the cache. Two
crates that depend on a build.rs producing identical outputs (e.g.
`syn`-derived code generation) can share the build.rs output's
splinter; the dispatch DAG walker can dedupe across the splinter_graph.

### 3.8 Cross-language uniformity

The lift's shape is per-language but the OID's shape is universal.
Each `@io/<lang>` species declares its own `lifted_fingerprint`
function (whose input sextuple may differ — Go's includes the
`GOFLAGS`; Python's includes the interpreter version; Julia's
includes the precompile cache version), but the OUTPUT is always a
`Splinter<Blake3>` at the @code/<lang> altitude. The cache
operations (§4) don't know about per-language fingerprint inputs;
they only see OIDs.

This is the substrate-pull dividend: the universal carrier
`mosaic(altitude)` from `@mirror/mosaic` and the universal address
`oid` from `@mirror/store` compose so the cache operations are
language-agnostic. The per-language species expansion (forward-
promised in `mirror-build-substrate.md` §5 amendment) DECLARES the
fingerprint inputs per language; the cache machinery doesn't need
to change.

---

## §4 — The cache operations (against `@mirror/store`'s six)

Per §1.3 negative #2: cache operations ARE @mirror/store operations.
The "cache surface" enumerated below is one-to-one with @mirror/store's
six operations under the @code/<lang> domain restriction. The
substrate had this already; this section makes the correspondence
explicit so downstream readers don't reach for a duplicate API.

### 4.1 `cache_read(oid) -> imperfect<bytes, error, loss>`

Same shape as `@mirror/store.read(o: oid) -> imperfect`. A cache hit
returns the stored translation unit's bytes; a cache miss returns
`failure(reason)` (oid not present); a cache present-but-drift returns
`partial(opacity_map)` (oid present but verify failed).

```mirror
in @mirror/store
in @code/rust    // or whatever @code/<lang> altitude

cache_read(o: oid) -> imperfect {
  @mirror/store.read(o)
  \
}
```

The cache layer is THIN — it's a re-exposure of @mirror/store.read at
the @code/<lang> altitude. The @code/<lang> altitude adds typing
discipline (the returned bytes ARE expected to be a translation unit
at this altitude); the underlying operation is unchanged.

### 4.2 `cache_write(bytes) -> oid`

Same shape as `@mirror/store.write(content: bytes) -> oid`. The
translation unit's bytes are content-addressed and stored;
idempotent by construction (writing the same bytes twice returns the
same oid and is a no-op on the second call).

```mirror
in @mirror/store
in @code/rust

cache_write(content: bytes) -> oid {
  @mirror/store.write(content)
  \
}
```

After `cache_write` succeeds, the dispatch DAG walker registers the
returned OID against the translation unit's compile_input (§2.2.8)
so the next dispatch can `cache_read(expected_oid)` for the hit.

### 4.3 `cache_exists(oid) -> verdict`

Same shape as `@mirror/store.exists(o: oid) -> verdict`. Used by the
dispatch DAG walker BEFORE attempting `cache_read` to avoid a wasted
read on a cold cache.

```mirror
in @mirror/store
in @code/rust

cache_exists(o: oid) -> verdict {
  @mirror/store.exists(o)
  \
}
```

`pass` — cache hit available; proceed to cache_read.
`partial(confidence)` — oid is in a cache layer but the canonical
backend hasn't confirmed; the dispatcher may speculatively proceed
or wait for confirmation.
`failure(reason)` — cache miss; the dispatcher MUST dispatch the
compile.

### 4.4 `cache_verify(oid, bytes) -> verdict`

Same shape as `@mirror/store.verify(o: oid, content: bytes) -> verdict`.
The re-hash check; substrate's integrity primitive. Used in the
`cache_valid` composition's `content_addressed_per_translation_unit`
discharge.

```mirror
in @mirror/store
in @code/rust

cache_verify(o: oid, content: bytes) -> verdict {
  @mirror/store.verify(o, content)
  \
}
```

Run on every `cache_read` if `@epistemologic/property/effect/storage_
integrity` is required at the @code/<lang> altitude; can be elided in
trusted-storage regimes for speed.

### 4.5 `cache_walk(root) -> splinter_graph`

Same shape as `@mirror/store.walk(root: oid) -> splinter_graph`.
Enumerates the build closure rooted at the binary/library target's
OID. This IS what the dispatch DAG walker consumes to discover the
transitive dep_closure for `dep_oid_transitive`.

```mirror
in @mirror/store
in @code/rust

cache_walk(root: oid) -> splinter_graph {
  @mirror/store.walk(root)
  \
}
```

The returned splinter_graph IS the cache index for the target IS
the structural lockfile.

### 4.6 `cache_diff(a, b) -> imperfect`

Same shape as `@mirror/store.diff(a: oid, b: oid) -> imperfect`. What
changed between two builds? Used by the dispatcher for incremental
DAG decisions and by `kintsugi` for substrate-pull repair morphism
selection (the diff names where the cache was invalidated; the
fracture body's repair targets exactly that locus).

```mirror
in @mirror/store
in @code/rust

cache_diff(a: oid, b: oid) -> imperfect {
  @mirror/store.diff(a, b)
  \
}
```

### 4.7 KEY INSIGHT: cache operations ARE @mirror/store operations

The "cache" is not a separate concept; it's `@mirror/store` used at
the `@code/<lang>` altitude. The substrate had this already. The
operations in §4.1-4.6 are thin per-altitude re-exports; the
substrate's surface is `@mirror/store`'s six operations.

This is the 53rd+ instance of `[[feedback-substrate-already-had-the-
word]]` named in §1.3 negative #1, made concrete at the operations
altitude: every cache operation has a substrate-altitude name; the
spec doesn't invent a single new operation. The cache IS the existing
storage gate operated under the determinism class declared in
`mirror-build-substrate.md` §2 at the per-language altitude.

The implication: ANY @mirror/store extension (e.g. cross-repo
mycelial sharing per @spectral/db) IMMEDIATELY becomes a cache
extension. The cache inherits @mirror/store's evolution for free.

---

## §5 — Composition with `@mirror/mosaic`'s eigensheaf parallelism

### 5.1 The forward-promise

Per `shards/mirror/mosaic.mirror` (the per-target action dispatch
docstring, lines 161-168):

> *"Per-target action dispatch (insight #43, 2026-06-09): when a
> target's `check <action>` directive (declared in
> shards/mirror/spec.mirror, v1+) names a specific @io/cargo action
> — fmt_check, clippy, test, audit, check, build — shift routes to
> that action instead of the altitude's default `build`. The
> pre-commit chain is five such settlements composed under
> transparency<p>; the bootstrap dispatcher walks them in
> declaration order today, eigensheaf-Laplacian parallelism
> analysis lands at recognition #44+."*

The forward-promise: eigensheaf-Laplacian parallelism analysis
identifies which targets settle CONCURRENTLY. The pre-commit chain
of five settlements (fmt_check → check → clippy → test → audit) is
declared as a linear chain in the bootstrap; the substrate-pull-
honest analysis is that they form a partial-order DAG whose
eigensheaf decomposition reveals the parallelism structure.

The mathematical machinery for the analysis lives in
`mirror-build-substrate.md` §6.4 (the parent spec): the build DAG
is a cellular sheaf over the eigensheaf-Laplacian operator; the
parallel groups are the eigenspaces of the operator's zero
eigenvalue; the dispatcher walks the DAG by eigenspace tier,
dispatching each tier in parallel and waiting on the tier boundary.

### 5.2 How the cache changes the eigensheaf decomposition

With content-addressed cache: targets whose `cache_read(oid)`
succeeds **short-circuit** — no compilation needed, just retrieval
from @mirror/store. The cache hit changes the DAG's effective
shape: a cache-hit target is "settled" instantly; its dependents
become eligible immediately; the eigenvalue spectrum collapses to
fewer non-trivial tiers.

The cache-aware dispatch algorithm:

```
1. Compute the full splinter_graph for the requested targets
   (cache_walk(root) for each target).
2. For each translation unit in the splinter_graph, compute its
   expected OID via lifted_fingerprint (§3.2).
3. For each expected OID, cache_exists(oid):
   - pass:    mark the unit "settled, source = cache".
   - failure: mark the unit "pending, source = compile".
4. Recompute the eigensheaf-Laplacian decomposition over the
   PRUNED DAG (settled units removed; their successors' parents
   set updated).
5. Dispatch the parallel tiers of the pruned DAG in order; each
   tier's units compile in parallel; cache_write(bytes) on each
   success.
6. Verify cache_valid for the dispatched units after settlement;
   emit transparency<p> for any partial verdicts.
```

The key insight: **the cache HIT pruning happens BEFORE the
eigensheaf decomposition.** This means cache hits reduce the
dispatch DAG's depth (cached targets disappear from the DAG, not
just from the dispatch); the eigenvalue spectrum collapses to
fewer tiers; the residual non-cached work parallelizes more.

### 5.3 Cache hit changes the DAG; the eigenvalue spectrum collapses

A worked example. The bootstrap's pre-commit chain at the mosaic-
target altitude is currently declared as:

```
fmt_check → check → clippy → test → audit
```

Linear; five sequential settlements. Total wall-clock time = sum of
individual settle times.

With the eigensheaf decomposition applied to the underlying
translation-unit DAG (rather than the action DAG): each action is
itself a DAG over translation units; multiple actions can share
translation-unit DAG nodes; the cross-action DAG has parallelism
where actions read disjoint translation-unit subtrees.

```
fmt_check reads: src/**/*.rs (formatter input)
check     reads: src/**/*.rs + Cargo.toml (type checker input)
clippy    reads: src/**/*.rs + Cargo.toml (lint input)
test      reads: src/**/*.rs + Cargo.toml + tests/**/*.rs + deps
audit     reads: Cargo.lock + advisory DB
```

The eigensheaf analysis reveals: `audit` has no translation-unit
overlap with `fmt_check`/`check`/`clippy`/`test`; it can run in
parallel. `fmt_check` has no overlap with `check`/`clippy`/`test`
beyond shared input (which is read-only); it can run in parallel
with them. `check` and `clippy` share most of their work
(type-check pass); the eigensheaf decomposition reveals the shared
substructure and dispatches the shared part once.

Without cache: the parallelism is real but every action still
compiles its translation units from scratch. Wall-clock = max(action
times) ≈ test's time (the longest action).

With cache: the SECOND pre-commit run hits 100% on all five
actions' translation units (no source has changed since the last
hit). The eigensheaf decomposition collapses to a trivial DAG (all
nodes settled); wall-clock = O(cache_exists checks + cache_read
retrievals) ≈ 100ms for the whole chain.

This is the operational dividend. The cache changes the eigenvalue
spectrum from "five tiers, each ~30s" to "one tier, ~100ms" for a
no-source-change pre-commit hit.

### 5.4 Why the "like nix but better" claim cashes out

Nix's per-derivation cache hits at the granularity of "did the
inputs to this derivation change?" — where a "derivation" is roughly
a `Cargo.toml` workspace + lockfile + rustc version. A single source
file change invalidates the WHOLE derivation; cargo's incremental
compilation kicks in WITHIN the derivation but doesn't share across
derivations.

The substrate's cache hits at the granularity of `splinter(@code/
rust/rlib)` — per-translation-unit. A single source file change
invalidates ONLY the units that transitively depend on that source
file's OID; all other units' OIDs remain valid; their cache entries
stay live.

Combined with eigensheaf-Laplacian parallelism: targets whose
cache_read succeeds short-circuit; the remaining targets parallelize
optimally based on their transitive dep_closure. This is sub-second
incremental builds where the GRANULARITY is .rlib, not derivation.

Quantitatively, for a workspace of N translation units where one
source changes:

- **Nix (per-derivation):** invalidates the workspace's derivation;
  rebuilds N units (subject to cargo's per-process incremental).
- **Cargo alone (per-unit, local):** invalidates the changed unit
  and its dependents (~k units); rebuilds k.
- **Substrate cache (per-unit, content-addressed, cross-machine):**
  invalidates the changed unit and its dependents; rebuilds k;
  publishes each rebuild's OID to @mirror/store; the NEXT machine
  that wants the same OIDs (CI runner, peer agent, librarian
  consolidation) hits cache for the (N-k) unchanged units AND the
  k rebuilt units (cross-machine sharing).

The "better than nix" claim is: same per-unit granularity that
cargo has, PLUS cross-machine sharing that nix-style content-
addressing provides, PLUS the eigensheaf-Laplacian parallelism
that substrate-altitude analysis enables. Three properties; one
substrate-decl.

### 5.5 Algorithmic complexity

For a build DAG of N translation units with depth D and hit-rate h:

- Cache lookup: O(N) cache_exists; cheap via Bloom filter.
- Pruned DAG: O((1-h)·N) units to dispatch.
- Eigensheaf decomposition: O((1-h)·N · D); bounded by depth.
- Parallel dispatch: O(D' · max_unit_time); D' ≤ D shrinks as h
  grows.

High-hit-rate (h ≈ 1.0): wall-clock collapses to O(milliseconds).
Cold (h ≈ 0.0): matches cargo baseline + cache_write overhead.
Cache strictly dominates for h > 0; cost is per-unit hash overhead
at compile.

---

## §6 — How yesterday's P4 hook blocker disappears

### 6.1 Yesterday's incident

Per `MEMORY.md` and the P4 hook blocker context: the
`fragmentation-git` Cargo edge introduces `libgit2-sys` as a
transitive dependency. The pre-commit hook chain runs four cargo
subcommands in sequence: `cargo check`, `cargo clippy`, `cargo
test`, and (in some configurations) `cargo build --release`. Each
subcommand triggers a fresh cargo dispatch; each dispatch sees
`libgit2-sys` as an unbuilt translation unit and triggers its
~2-3 minute cold compile.

Net effect: libgit2-sys recompiled 4× per hook run. Total cold-
compile budget exceeds the hook's signal-15 timeout; the hook
kills the chain; the commit fails.

The blocker is OPERATIONAL (libgit2-sys is genuinely expensive to
compile) but the WASTE is structural — the substrate has no
mechanism to share libgit2-sys's compile output across cargo
subcommand invocations in the same hook run.

### 6.2 With this spec landed operationally

Once the v0 ticks discharge (§9):

1. The first `cargo check` invocation triggers libgit2-sys's cold
   compile. The output (.rlib) is content-addressed via the
   lifted_fingerprint (§3.2). The OID is stored in @mirror/store
   via `cache_write(bytes)`.
2. The second invocation (`cargo clippy`) runs lifted_fingerprint
   over libgit2-sys's source bytes + dep OIDs + rustc version +
   cfg + profile + target. The result is the SAME OID as the
   first invocation (cfg might differ between check and clippy;
   if so, two cache entries; but typically the cfg is identical).
3. `cache_exists(oid)` returns pass; `cache_read(oid)` returns the
   stored .rlib bytes; cargo's invocation skips the libgit2-sys
   compile and proceeds.
4. Repeat for `cargo test` and `cargo build --release`.

### 6.3 Quantitative analysis

- **libgit2-sys cold compile:** ~2-3 minutes per invocation.
- **libgit2-sys cache hit (cache_exists + cache_read):** ~10ms.
- **4× cold (current state):** ~10 minutes total libgit2-sys
  budget.
- **1× cold + 3× hit (post-discharge):** ~3 minutes (the first
  cold) + ~30ms (the three hits) ≈ 3 minutes total. **~7 minutes
  saved per hook run.**
- For hook runs after the FIRST hook run on the same source tree
  (no libgit2-sys source change): 4× hit ≈ 40ms. **~10 minutes
  saved per hook run.**

The pre-commit hook's signal-15 budget is 5 minutes (default). The
~7-10 minute savings move the hook from "fails reliably" to
"succeeds with margin."

### 6.4 This IS the substrate-pull-honest unblock

Task #488 (per `MEMORY.md` referenceable) currently has the P4
hook blocker open. The substrate-pull-honest unblock is NOT to:

- raise the hook timeout (papers over the structural waste);
- exclude libgit2-sys from the pre-commit chain (loses the
  verification);
- skip cargo subcommands (loses the verification);
- use sccache as a workaround (sccache is a content-addressed
  cache OPERATING OUTSIDE the substrate; using it bypasses
  substrate-decl rather than discharging it).

The substrate-pull-honest unblock IS to discharge this spec's v0
ticks (§9) — the cache invariants land, the fingerprint → OID lift
lands, the cache_read/cache_write/cache_exists operations are
wired through @io/cargo, the pre-commit hook chain operates against
the cache. libgit2-sys is recompiled ONCE per (rustc, cfg, profile,
target); the substrate's discipline absorbs sccache's role natively.

The cross-reference to task #488 is explicit so the next pass over
the P4 hook blocker sees the substrate-altitude resolution path
named, not silently routed around with a workaround.

### 6.5 Generalization beyond libgit2-sys

Every heavy transitive dependency benefits from the same mechanism:
`tokio` (~1-2min cold), `regex` (~30s), `serde_derive` (~30s + per-
use), `clap` (~30s), `prost` (~30s + per-proto), `cxx` (~1min). For
a typical CLI dep set (clap + tokio + serde + regex + tracing): cold
budget ≈ 4-5min; warm hit budget ≈ 50ms. Hook chain savings scale
with heavy-dep count.

### 6.6 Empirical discharge target

The first empirical discharge for this spec is to re-run yesterday's
P4 commit attempt with the cache landed. The expected observation:

- First invocation (cache cold): libgit2-sys cold compile observed
  via tracing (e.g. `RUSTC_LOG=info`); ~2-3 min observed; cache_write
  succeeds; OID logged.
- Second invocation (cargo clippy): cache_exists returns pass;
  cache_read returns bytes; libgit2-sys NOT recompiled; cargo
  proceeds directly to the dependent compile.
- Hook chain wall-clock ≤ 5 minutes; hook succeeds; commit lands.

The empirical witness IS the substrate-pull-correct discharge of
the bilateral pair: the property `cache_valid` holds (verdict =
pass) for libgit2-sys's translation unit across the four
subcommand invocations; the fracture body never fires (no repair
needed); the cache invariants hold operationally.

---

## §7 — The math

### 7.1 Cache as equalizer

In category-theoretic terms: the cache is the EQUALIZER of two
arrows from the source-set to the artifact-set:

```
                compile
source_set ────────────────► artifact_set
                retrieve
source_set ────────────────► artifact_set
```

where:

- `compile(input)` = run rustc on input, get bytes;
- `retrieve(input)` = `cache_read(lifted_fingerprint(input))`.

The equalizer is the subset of `source_set × artifact_set` where
the two arrows produce byte-equal results. Content-addressing IS
the proof that this equalizer exists and is well-defined:
deterministic compilation (§2.2.8) guarantees that for any
`input`, `compile(input)` produces a unique byte-output; the
fingerprint algebra produces a unique OID for the same `input`;
the cache stores `(OID, bytes)`; `retrieve(input)` returns the
stored bytes; the two arrows agree.

When `deterministic_compilation` returns partial or failure, the
equalizer is partial or empty — the cache claim is invalid; the
substrate must dispatch the compile (cache MISS forever for that
input, per §2.2.8 failure semantics).

### 7.2 Cache as functor

The compilation operation `compile: (source, env) → artifact` is
**functorial** at the substrate altitude:

- It maps objects (source files + env) to objects (artifact bytes).
- It maps morphisms (source dependencies + env composition) to
  morphisms (artifact dependencies + linker composition).
- It preserves identity (empty input → empty output) and composition
  (compile(compose(s1, s2)) = compose(compile(s1), compile(s2))
  modulo linker concerns).

Content-addressing IS the proof that the functor is well-defined:
same inputs → same output, byte-for-byte. The functor's well-
definedness IS the cache's correctness; the cache's correctness IS
the functor's well-definedness.

When the functor is not well-defined (non-deterministic rustc, e.g.
embedding timestamps), the cache cannot be sound. The
`deterministic_compilation` invariant from §2.2.8 IS the substrate's
discipline enforcing well-definedness.

### 7.3 Cache invariants as `@epistemologic/property` monoid

AND-composition of the eight invariants is **monoidal** over the
verdict surface: identity = pass; associative; absorbing element =
failure; commutative; partial verdicts accumulate opacity_maps
under union (per @glass's opacity_map vocabulary).

The monoid means `cache_valid` is computed incrementally: fold
each invariant's verdict; short-circuit on failure (absorbing);
otherwise continue. Algebraic discipline for "is this cache entry
valid?" reduced to a fold over eight predicates.

### 7.4 Sheaf restriction

Cache hits across machines are **sheaf restriction maps** in the
sheaf-theoretic sense: each machine carries a local section of the
substrate's content-addressed store; the global section is the
union of all machines' local sections; restriction maps are
@mirror/store's read/write/exists operations across the network.

The gluing condition: any two machines that both have an OID in
their local section MUST agree on the bytes at that OID
(content-addressing guarantees this; the `cache_verify` invariant
witnesses it). Where machines disagree on bytes for the same OID,
the substrate has a content-addressing violation and must
quarantine the divergent storage.

The DISTRIBUTED cache — cross-machine, cross-repo, peer-to-peer
sharing — IS the gluing of these local sections into a global
section. Per `[[architecture-mirror-store-vs-spectral-db]]` and
Anna's-thesis-style "open foundation closes engine":

- **@mirror/store** provides the LOCAL section + the restriction
  maps (read/write/exists/verify operations) — open foundation;
  per-machine; substrate-decl ground.
- **@spectral/db** provides the GLOBAL gluing + the supervisor
  topology — closed engine; cross-machine; the librarian per
  `spectral-db-as-autopoietic-memory.md`'s autopoietic memory
  consolidation.

This spec is about the LOCAL section (the @mirror/store side); the
DISTRIBUTED cache (the @spectral/db side) is forward-promised in
§8.4 and lives in the closed-engine layer. The substrate-decl
honesty: the open foundation's cache works in isolation; the closed
engine ENHANCES it with distribution.

### 7.5 Cache as adjunction (forward-promise)

A future refinement: the cache + recompile pair forms an ADJUNCTION
between source-set and artifact-set. `compile` is left adjoint;
`decompile_dep_closure` (walk the splinter_graph backward to the
minimal source) is right adjoint. The unit is the source's
transitive dep_closure; the counit is the cache hit. Speculative;
left as forward-promise; would guide cache eviction policy.

---

## §8 — Open questions

### 8.1 RUSTC_VERSION exact match vs ABI window

Per §2.2.7, the cache currently pins rustc_version exactly; this is
the safe choice with low reuse (a rustc bump invalidates the whole
cache). The substrate has the `abi_compatible_window` invariant
declared as a property the cache CAN expand into, but the
empirical ABI characterization that closes the window is forward-
promised under Q4 substrate-decl (`mirror-build-substrate.md` §8.4).

Two paths v1+ may take:

1. **Conservative:** keep exact pin; let users opt in to wider
   windows via `mirror.spec` declaration; the substrate witnesses
   the opt-in but doesn't widen the window proactively.
2. **Empirical:** the kintsugi fracture body for
   `abi_compatible_window` runs ABI compatibility scans on rustc
   bumps; if the scan passes, the substrate widens the window; if
   it fails, the substrate narrows or invalidates.

Path 2 requires the ABI scan implementation (rustc plugin or
external tool); path 1 is shipping-ready as soon as the v0 ticks
discharge. The conservative path is the v0 floor; the empirical
path is the v1+ refinement.

### 8.2 Per-language fingerprint logic location

Where does the per-language fingerprint logic live? Three
candidates:

1. **`@code/<lang>/cache`** — sub-prism under each @code/<lang>
   altitude; co-locates fingerprint with the language's
   substrate-decl.
2. **`@io/<lang>/cache`** — sub-prism under each @io/<lang>
   species; co-locates fingerprint with the @io tool's
   substrate-decl.
3. **`@mirror/mosaic/cache/<lang>`** — sub-prism under
   @mirror/mosaic; co-locates fingerprint with the orchestrator.

The substrate-pull-honest argument for option 2 (`@io/<lang>/
cache`): the fingerprint inputs are the @io tool's invocation
inputs (env vars, command-line flags, manifest contents); the @io
species ALREADY declares those inputs (per
`shards/io/cargo.mirror`); the fingerprint is the canonical
content-address of those inputs. The fingerprint belongs at the
@io altitude where the inputs are already typed.

The substrate-pull-honest argument for option 1 (`@code/<lang>/
cache`): the fingerprint includes language-altitude facts (rustc
version is a @code/rust fact; rustc cfg is a @code/rust fact)
that may not be visible at the @io altitude. The fingerprint
belongs at the @code/<lang> altitude where the language facts
live.

Resolution forward-promised to the per-language species expansion
(`mirror-build-substrate.md` §5 amendment) — when @io/go and
@io/uv and @io/julia land, the location decision will be made
based on which altitude carries the fingerprint inputs most
naturally. v0 ships with @io/cargo carrying the fingerprint; v1+
may refactor based on the per-language landings.

### 8.3 Garbage collection

@mirror/store's content-addressed storage grows unboundedly as
cache entries accumulate. Cache entries that aren't referenced
from any live splinter_graph root are candidates for garbage
collection. The GC policy is substrate-decl-able as an
@epistemologic/property:

```mirror
in @epistemologic/property

prism @epistemologic/property/cache_gc_admissible
  <= @epistemologic/property

cache_gc_admissible(
  oid: oid,
  live_roots: [oid]
) -> verdict { \ }
```

Verdict semantics:

- **pass** — `oid` is NOT reachable from any live root via
  `cache_walk`; admissible for GC.
- **partial(opacity_map)** — `oid` is reachable from some roots
  but not others; partial admissibility; GC policy may retain or
  evict based on age + access frequency.
- **failure(reason)** — `oid` IS reachable from at least one live
  root; not admissible for GC.

The GC policy is forward-promised; v0 ships without GC (the cache
grows unboundedly; disk pressure is the operator's concern); v1+
will discharge the policy property.

### 8.4 Cross-machine cache sharing — @spectral/db territory

Per §7.4 sheaf restriction analysis and
`[[architecture-mirror-store-vs-spectral-db]]`: the local section
(per-machine cache) is @mirror/store's open foundation; the global
gluing (cross-machine cache sharing) is @spectral/db's closed
engine. This spec is about the local section.

The substrate-decl honesty: this spec does NOT propose cross-machine
cache sharing as part of @mirror/store. The per-machine cache works
in isolation; cross-machine sharing is a NEXT-altitude composition
(@spectral/db) that builds on the per-machine substrate.

The closed engine's role:

- consolidate per-machine cache entries into a federated mycelial
  cache;
- decide which OIDs to replicate across the federation
  (popularity-based, peer-trust-weighted, locality-aware);
- handle the gluing condition operationally (Byzantine-fault-
  tolerant consensus on byte-equality at OIDs);
- provide the librarian (per `spectral-db-as-autopoietic-memory.md`)
  with the cross-machine view.

This is forward-promise; this spec doesn't substrate-decl any of
it. Per Anna's-thesis discipline: the open foundation gives the
per-machine cache for free; the closed engine sells the cross-
machine consolidation. The business model and the architecture
align at the @mirror/store / @spectral/db boundary.

### 8.5 Cache invariants under cross-compilation pipelines

When building for a target_triple different from the host (e.g.
building aarch64-unknown-linux-musl on x86_64-apple-darwin), the
cache key includes the target_triple (§2.2.5) but the COMPILER
HOST also differs. Two situations:

1. **Host-portable rustc** (rustup-managed toolchain that supports
   multiple targets): the rustc_version pins the host-version +
   target-version pair; cache entries are host+target-keyed.
2. **Host-specific rustc** (cross-compiler installed separately):
   the rustc_version pins the cross-compiler's identity; cache
   entries are specific to that cross-compiler.

Case 1 is well-served by the current invariants; case 2 needs the
rustc_version to capture the cross-compiler's distinct identity
(not just the version number). This is a sub-tightening of the
`rustc_version_pinned` invariant; forward-promised under the v1+
cross-compilation polish.

### 8.6 Cache invariants under sandbox escape

Cache correctness depends on compilation being SANDBOXED — read
set bounded by declared inputs. If rustc reads outside the input
set (`/etc/...`, env-derived paths), the cache key is incomplete.
`deterministic_compilation` (§2.2.8) catches this REACTIVELY post-
hoc; a proactive
`@epistemologic/property/effect/filesystem_read_set_bounded` would
catch it at dispatch. Forward-promised; v0 relies on reactive
verdict.

---

## §9 — Forward-promises

### 9.1 Rust-side implementation

The fingerprint → Splinter OID lift lands in the mirror Rust crate
(specifically, the per-language @io species crates — `fragmentation-
cargo` if it exists, otherwise a new `mirror-cache` crate that
wraps @io/cargo's invocation surface). The implementation is
roughly:

```rust
pub struct CacheKey(Splinter<Blake3>);

pub struct CompileInput {
    pub source_oid:    Splinter<Blake3>,
    pub dep_oids:      Vec<CacheKey>,
    pub rustc_version: RustcIdentity,
    pub cfg:           CfgSet,
    pub profile:       Profile,
    pub target:        TargetTriple,
}

pub fn lifted_fingerprint(input: &CompileInput) -> CacheKey {
    let serialized = bincode::serialize(input).unwrap();
    CacheKey(Splinter::<Blake3>::oid(&serialized))
}

pub fn cache_lookup(
    store: &dyn MirrorStore,
    key: &CacheKey,
) -> Imperfect<Vec<u8>, CacheError, OpacityMap> {
    store.read(&key.0.oid())
}

pub fn cache_publish(
    store: &mut dyn MirrorStore,
    key: &CacheKey,
    bytes: &[u8],
) -> Verdict {
    let stored_oid = store.write(bytes);
    if stored_oid == key.0.oid() {
        Verdict::Pass
    } else {
        Verdict::Failure(Reason::OidMismatch {
            expected: key.0.oid(),
            actual:   stored_oid,
        })
    }
}
```

The crate's public surface mirrors the §4 cache operations one-for-
one; the per-language fingerprint input set is constructed by the
@io/<lang> caller and passed in. The crate lives BELOW substrate-
decl altitude; the substrate-decl shards (§9.2) are the contract;
the crate is the realization.

### 9.2 Substrate-decl shards forward-promised

Per the bilateral pattern (§2.3), each invariant gets two shards:

```
shards/epistemologic/property/cache/content_addressed_per_translation_unit.mirror
shards/epistemologic/property/cache/rustc_version_pinned.mirror
shards/epistemologic/property/cache/cfg_pinned.mirror
shards/epistemologic/property/cache/profile_pinned.mirror
shards/epistemologic/property/cache/target_triple_pinned.mirror
shards/epistemologic/property/cache/dep_oid_transitive.mirror
shards/epistemologic/property/cache/abi_compatible_window.mirror
shards/epistemologic/property/cache/deterministic_compilation.mirror
shards/epistemologic/property/cache/cache_valid.mirror

shards/kintsugi/fracture/cache/content_addressed_per_translation_unit.mirror
shards/kintsugi/fracture/cache/rustc_version_pinned.mirror
shards/kintsugi/fracture/cache/cfg_pinned.mirror
shards/kintsugi/fracture/cache/profile_pinned.mirror
shards/kintsugi/fracture/cache/target_triple_pinned.mirror
shards/kintsugi/fracture/cache/dep_oid_transitive.mirror
shards/kintsugi/fracture/cache/abi_compatible_window.mirror
shards/kintsugi/fracture/cache/deterministic_compilation.mirror
```

Sixteen shards (eight property + eight fracture); landing tick-by-
tick. Each tick is a TDD-paired Red→Green pair: Reed writes the
RED test, an agent writes the GREEN substrate-decl + fracture body.

### 9.3 First empirical cache discharge

Re-run yesterday's P4 commit attempt with this spec's v0 ticks
landed; verify libgit2-sys cache hit per §6.3 expected observation.
The empirical witness IS the bilateral pair's discharge; the
substrate-pull-correct unblock for task #488.

### 9.4 Per-language cache extensions

As @io/<lang> species expand per `mirror-build-substrate.md` §5
amendment, each species declares its own fingerprint input set:

```
@io/cargo:    (source_bytes, dep_oids, rustc_version, cfg, profile,
               target, build_script_outputs)
@io/go:       (source_bytes, dep_oids, go_version, goflags, gocache_key,
               build_id)
@io/uv:       (source_bytes, dep_oids, python_version, wheel_metadata,
               build_isolation_state)
@io/julia:    (source_bytes, dep_oids, julia_version, precompile_cache_key,
               manifest_oid)
@io/mix:      (source_bytes, dep_oids, elixir_version, otp_version,
               mix_lock_oid, compile_env)
@io/make:     (source_bytes, dep_oids, makefile_oid, env_set,
               command_oid)
```

Each species' fingerprint feeds into the universal cache_valid
composition via the cache invariants; the per-language species
DECLARES the inputs; the cache machinery is unchanged.

### 9.5 Bazel + Nix absorption

The substrate absorbs Bazel-style hermetic actions + Nix-style
derivations by composing per-language species under each
runtime's sandbox boundary. Bazel `ctx.execute` becomes an @io
species whose fingerprint includes hermetic inputs; Nix
`mkDerivation` becomes an @io species whose fingerprint includes
derivation input store paths. The cache HITS across all runtimes
when fingerprints agree — universal OID address space unifies what
are today disjoint namespaces. Forward-promised with @io/<lang>
expansion.

### 9.6 Cache + @spectral/db distributed gluing

Per §8.4: distributed cache is @spectral/db's closed-engine
territory. This spec's forward-promise to @spectral/db is the
INTERFACE: cache exposes its local section via @mirror/store's
six ops; @spectral/db federates. Interface stability is the
commitment.

### 9.7 Cache instrumentation + telemetry

Each cache_read/write/exists operation emits a telemetry event
(hit, miss, cache_valid verdict per invariant) consumed by the
forward-promised @mirror/trace family. Opt-in at the @io/<lang>
caller boundary; observation not enforcement.

---

## §10 — Circular-reflexive layer

### 10.1 The recursion this spec earns by writing itself

This spec describes the cache that mirror's eventual build
orchestrator (@mirror/mosaic) will use when it builds itself
building things. The spec ITSELF is a content-addressed crystal in
mirror's store (just like every other .md file in the repo); the
spec ABOUT cache will LIVE IN the cache the spec describes; the
recursion closes when `mirror init` runs against the mirror repo
and the cache walks the documents in `docs/specs/` and indexes
them content-addressed.

The four nested closures:

1. **The spec is content-addressed.** This file's bytes hash to a
   BLAKE3 OID once `mirror init`'s Phase A lands the substrate-pull
   from git's SHA-1 to @mirror/store's BLAKE3. The OID is the
   spec's identity in the substrate; the OID enters the cache when
   the substrate indexes the docs/specs corpus.
2. **The cache the spec describes USES the same OID for its own
   spec.** When @mirror/mosaic's dispatch walks the substrate's
   own substrate-decl roster (the bootstrap closure of §0 framing
   3), this spec's OID is in the walk. The walker computes
   cache_exists(this_spec_oid); if pass, cache_read returns this
   spec's bytes; the walker dispatches against the bytes.
3. **The walker's dispatch DAG includes cache discharge as a
   first-class target.** Per §9.2, the bilateral pair landings are
   tracked as forward-promises. The walker, dispatching against
   the substrate-decl, sees this spec's forward-promises in its
   work queue; it dispatches them; the cache's discharge happens
   as ONE step in the walker's dispatch.
4. **The dispatch's cache hit feeds back into THIS spec's
   propagation.** Once the v0 ticks discharge and the cache is
   operational, this spec's OID is one of the FIRST cache entries
   the dispatcher writes (the substrate's own substrate-decl is
   indexed first per §0 framing 3). Subsequent dispatches that
   walk past this spec's OID hit the cache; the spec propagates
   through the substrate at cache-read speed; the substrate's
   knowledge of its own cache discipline accelerates.

The recursion is load-bearing in the sense of §0 framings 1-3:
eigenform fidelity (spec carries the same content-addressing
discipline at the document altitude that it declares at the
artifact altitude), substrate-pull discipline (the spec doesn't
invent; it documents what the substrate already had), and bootstrap
closure (the cache's first user is its own spec).

### 10.2 What this spec's existence WITNESSES

Spec existence ≠ implementation existence. Three substrate-pull-
honest facts witnessed at 2026-06-28:

1. **@mirror/store ALREADY IS the build cache.** The substrate-decl
   from 2026-06-04 declared content-addressed storage; the
   recognition that this IS the cache is today's. The substrate WAS
   already the cache; this spec names the pre-existing structure.
2. **mosaic(@store) = splinter_graph ALREADY IS the cache index.**
   The universal carrier from 2026-06-06; the recognition today.
   The substrate WAS already carrying the index.
3. **@io/cargo ALREADY IS the per-language precedent.** The
   invocation inputs ARE the cache key inputs; the substrate WAS
   already typing them; the spec lifts the typing to substrate-decl.

Three pre-existing structures; one substrate-pull-honest
documentation. The spec writes nothing the substrate doesn't
already do.

### 10.3 The crystal-self-ingest gesture

When `mirror init` runs against the mirror repo and indexes this
spec, the cache's verification recurses: the spec describes the
invariants; the cache asserts them on the spec; the OID
demonstrates `content_addressed_per_translation_unit`; the git
predecessor chain demonstrates `dep_oid_transitive` at document
altitude; `deterministic_compilation` becomes deterministic-
indexing. Autopoietic closure: substrate's documentation IS
substrate's discipline IS substrate's documentation.

### 10.4 What this spec REFUSES

- A new family-root for cache (`@cache` etc.) — the morning
  framing is closed.
- Reinventing content-addressing — @mirror/store's OID is the
  universal address.
- Competing with cargo's incremental compilation — the cache
  WRAPS cargo's target/ at the @io boundary.
- Cross-machine cache as part of @mirror/store — that's
  @spectral/db's territory per Anna's-thesis open/closed.
- A workaround for the P4 hook blocker that doesn't discharge at
  substrate altitude.

Each refusal is load-bearing; each makes the spec substrate-pull-
honest. The substrate names every refused alternative; the spec
uses the substrate's names.

### 10.5 The closure

The spec returns to §0 with four nested closures earned. The cache
IS @mirror/store at @code/<lang>; the invariants ARE
@epistemologic/property predicates; the fingerprint LIFTS to
Splinter OID via four cleanings; discharge IS the bilateral pair
landings per §9.2.

The spec enters the substrate via the cache it describes; the
cache verifies the spec; discipline propagates bidirectionally;
the bootstrap closes; the build orchestrator has substrate-decl
ground for cache; the P4 blocker resolves. The spec earns its
lines because the substrate earned its discipline first. The 53rd+
instance of `[[feedback-substrate-already-had-the-word]]`
discharges in the act of writing.

---

## §11 — Recognition log

### 11.1 What this spec recognizes

This spec is the 53rd+ instance of `[[feedback-substrate-already-
had-the-word]]`. The substrate had:

- `@mirror/store` (the cache);
- `mosaic(@store) = splinter_graph` (the cache index);
- `@io/cargo` (the per-language precedent);
- `@epistemologic/property` family (the invariant declaration form);
- the bilateral pattern `@epistemologic/property/<predicate>` +
  `@kintsugi/fracture/<predicate>` (the auto-formatting floor);
- the determinism class from `mirror-build-substrate.md` §2 (the
  per-action discipline the per-artifact discipline specializes).

The spec discovers that the substrate had a "build cache" all
along; the discovery is at the operations + invariants altitude;
the substrate's pre-existing surface is named at the cache altitude.

### 11.2 Open candidate

**Recognition candidate (not yet promoted; flagged for Pack
ratification):** the substrate-pull pattern of "X is Y operated at
altitude Z" recurs at the cache layer. Each such recurrence is one
instance of `[[architecture-form-process-kinship-at-sub-shard-
altitude]]` (recognition #61 PROMOTED): the form-side
(@mirror/store as a storage prism) and the process-side (@io/cargo
as an invocation prism) compose at one sub-shard altitude (the
cache layer) without either becoming the other. The cache IS
@mirror/store under @io/cargo's invocation discipline; the
composition is altitude-local.

If the pattern recurs at one more altitude (e.g. @spectral/db
gluing as @mirror/store federated across @io/network — the same
form/process composition at the distributed-cache altitude), the
recognition graduates from sub-pattern to substrate-decl primitive.
Forward-promised observation; not promoted in this spec.

### 11.3 Substrate-pull witnesses

Three substrate-pull-honest facts at composition altitude:

1. The 4× libgit2-sys waste IS the substrate's algedonic signal
   that cache discipline is operationally missing though declared
   substrate-decl-ally. The signal-15 hook kill IS the substrate
   naming the discharge as blocker.
2. @mirror/mosaic recognition #44+ (eigensheaf parallelism) +
   this spec's invariants compose into operational dispatch.
   Neither alone unblocks the budget; together they do.
3. The bilateral pattern's two recent instances
   (dark_count_monotone + cold_compile_within_tolerance, both
   2026-06-19) extend to cache invariants without new machinery;
   the spec adds eight instances of the same pattern at the cache
   altitude.

The witnesses ARE the spec's warrant: the discipline declared IS
the discipline the substrate was pulling toward.

---

*End of spec. Markdown only. 1500-line target met; soft cap held.
The v0 ticks (§9) discharge in subsequent TDD-paired commits;
the P4 hook blocker unblock per §6.4 is the first empirical
discharge target. The substrate-pull continues.*
