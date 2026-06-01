# kintsugi-thesis — nine reproducibility claims, scored honestly

*2026-06-01. The engineering pitch. Kintsugi-as-build-system claims
Nix-grade reproducibility AND determinism — including for its
AI-inferred au column. The claim is decomposed into nine load-bearing
properties. Each is either supported by what's already landed, or named
as work yet to do. No marketing language. No novelty pitch.*

Read [[README]] for the bar and [[prior-art]] for the lineage. This
file is the engineering claim and where it stands today.

---

## 1. The thesis, stated as engineering

```
Kintsugi-as-build-system is reproducible and deterministic, including
its au-bound joints, because every input — including the model that
did the inferring — is named by its content hash, and the inference
function is local-by-construction and seed-pinned by substrate law.
```

The thesis is testable. For each au-bound joint produced by a
kintsugi build:

- **Reproducibility test.** Two builders on different machines, given
  the same source tree, the same substrate, the same Crystallizations
  table (with the same model OID pinned), produce byte-identical
  output artifacts. Verified by `Splinter::verify` returning true on
  both machines' final-artifact OIDs being equal.
- **Determinism test.** Re-running the build on the same machine, in
  any order, with any concurrency setting, produces the same output.
  Verified by repeated `kintsugi_tick` invocations on the same input
  producing the same `Imperfect` verdict carrier (same Pass/Partial/Fail
  shape, same Transparency map by content).

If either test fails on a non-trivial corpus, the thesis is broken at
the point where the test fails, and the broken claim below names the
specific leak.

The pottery metaphor is in service of the engineering: the gold
filling the cracks is content-addressed gold, not "AI gold". A crack
that gets filled stays filled across builds because the fill is named
by the same hash on every machine that does the filling.

---

## 2. The nine claims

The thesis decomposes into nine claims, each of which can be
inspected independently. The claims are numbered, scored against
today's substrate, and named honestly: ✅ landed, ⚠️ partial, ❌ yet
to do.

### Claim 1 — Content-addressing all the way down ✅

**Property.** Every value the substrate exchanges has a content
address (`Splinter<H>::oid()`). The address is computed from the
canonical bytes of the value; two byte-identical values have
byte-identical OIDs; two OIDs that compare equal name the same value.

**Where it lives.** `bootstrap/src/crystallize.rs` — the `Splinter<H>`
type, the `MerkleHash` trait, the `compute_oid` Merkle encoding
(`b"T" || u64_le(len) || bytes`, `b"R" || u64_le(n) || sorted
(key, child-OID) pairs`, `b"L" || u64_le(n) || child-OID list`).

**Where it could leak.** Nowhere structural today. The encoding is
fixed; tested
([`splinter_oid_deterministic`](../../bootstrap/src/crystallize.rs),
[`splinter_record_merkle`](../../bootstrap/src/crystallize.rs),
[`splinter_list_merkle`](../../bootstrap/src/crystallize.rs)). The
default backend (BLAKE3, `Blake3Oid`) is content-deterministic. The
generic-over-hash cascade lets the engine use a different `H` without
disturbing the encoding shape.

**Verdict.** ✅ Landed. The Venti/Nix lesson is inherited without
compromise.

### Claim 2 — Hermetic by mathematical invariant, not by sandbox ✅

**Property.** Inference (the source of au values) does not reach off
the local machine. The substrate refuses remote inference by
mathematical law, not by network policy.

**Where it lives.** `AGENTS.md` §"`local` is a universal property of
@fate". `@fate` carries `local` as a universal property by
construction. Any inference routed through `@fate` satisfies the
local guarantee. Remote inference goes through `@spectral/garden/*`
packages — a *different namespace*, with explicit provenance and
audit trails. The substrate does not prevent remote routing; it
refuses to pretend the guarantees hold across the wire.

**Why this is stronger than Bazel's sandbox.** Bazel's sandbox can be
punched (`--config=` flags, `local_test_environment`, sandbox
exceptions for specific tools). The local-by-construction invariant
cannot. A grammar that imports `@spectral/garden/foo` is *visibly* in
the audited-cross-wire path; the property check `glass_wall(g)` would
refuse a grammar that pretends to be `@fate`-shaped while reaching
across the wire.

**Where it could leak.** A future `@fate` implementation that
silently delegates to a network endpoint behind the local API would
break the invariant. The structural defense is that `@fate`'s body
must satisfy the `local` property at compile time; today this is a
declared property with a `\` body (parked). The property check is
formally substrate-decidable (the namespace check is one substring
operation) but isn't yet enforced. **Yet-to-do flag: the `glass_wall`
property check for `@fate`-resident bodies wants to land as part of
the Tick C minimum-runnable engine landing.**

**Verdict.** ✅ Landed as substrate law; ⚠️ enforcement is by audit,
not yet by compile-time property check.

### Claim 3 — Pinned model: weights are content-addressed ⚠️

**Property.** The Fate model's weights are bytes in `@mirror/store`.
The bytes have a `Splinter<H>` OID. That OID is part of the cache key
for any au-typed value the model produces. Upgrading the model
changes the OID; the OID change is a cache miss; the cache miss is
an explicit rebuild.

**Where it lives.** `@mirror/store` is the content-addressed gate
([[../specs/store-vs-db-and-the-cascade]] §1.1). The store accepts
any bytes; the model's weights are bytes. The store gives them an
OID. That much is landed.

**Where it could leak / what's missing.**

- **Cache key composition.** Today, fate's resolutions are stored
  at `refs/fate/<hole_oid>` ([[../specs/au-and-conductivity]]).
  The hole-OID is in the key. The model OID is **not yet** in the
  key. A model upgrade with the same hole-OID would silently return
  a stale resolution.
- **The fix.** The cache key becomes `(model_oid, hole_oid,
  temperature, seed, sampling_policy_oid)`. This is one substrate-level
  change to `@fate.infer`'s signature: the action takes (hole, model,
  config) instead of just (hole); the model's OID is computed from
  its weights' bytes; the config's OID covers temperature, seed, and
  sampling policy.
- **Substrate impact.** The `@fate` grammar's `infer` action grows
  parameters. Existing call sites are few (the kintsugi engine and
  the auto-scheduler tests). The change is localised.

**Verdict.** ⚠️ Partial. The store gives model weights an OID; the
cache-key composition that includes the model OID is the missing
piece. Named as a discrete substrate change.

### Claim 4 — Pinned seed, fixed temperature, deterministic sampling ❌

**Property.** `@fate.infer`'s inference function is a pure function
of its inputs. Same (hole_oid, model_oid, temperature, seed,
sampling_policy_oid) → same au value, byte-identical.

**Where it lives.** Not yet. `boot/std/fate.mirror` declares the five
models as `io tick(features) => imperfect`, with bodies parked. The
tournament rules (`elite(1).beam(8).halving(3)`) compose
deterministically *given* deterministic candidate generation. The
candidate generation itself is not yet seed-pinned.

**Where it could leak / what's missing.**

- **Temperature is implicit.** Today there's no declared temperature
  parameter. The `@fate/tournament.anneal(T)` action declares a
  temperature for the simulated-annealing rule, but the *base
  inference* doesn't have one declared. The substrate change is to
  add `temperature: f64` to `@fate.infer`'s signature, default to
  zero (greedy), and forbid temperature ≠ 0 in build contexts (it
  can be allowed in interactive contexts; the build vs interactive
  distinction is `@io`-flag territory).
- **Seed is implicit.** Same story. Add `seed: u64` to the
  signature; the seed is part of the cache key.
- **Sampling policy is implicit.** Same story. Today the policy is
  hard-coded as `elite(1).beam(8).halving(3)`. The policy's OID
  (computed from the rule expression's bytes) belongs in the cache
  key.
- **Kernel determinism.** Even with all four pinned, a
  matrix-multiplication kernel that does parallel reductions can
  produce different bit patterns across runs (the floating-point
  associativity issue). The structural answer is the property
  `requires deterministic(@fate.infer)`: the model checker refuses
  to register a fate model whose underlying kernel cannot guarantee
  byte-identical output. CPU paths typically satisfy this; GPU
  paths typically don't, and need explicit work (Triton's `tl.fma`
  in deterministic mode, cuBLAS with `CUBLAS_PEDANTIC_MATH`, etc.).

**Verdict.** ❌ Yet to do. This is the load-bearing block of the
reproducibility chain for the au column. Three substrate-level
changes (`@fate.infer` signature; build/interactive flag in `@io`;
`requires deterministic` property) and one engineering-discipline
ladder (CPU-only by default; GPU with explicit deterministic
configuration). All localised; none architecturally novel.

### Claim 5 — Au is content-addressed by (hole, model, config) ⚠️

**Property.** An au-typed value's OID is computed from (hole_oid,
model_oid, temperature, seed, sampling_policy_oid, the inference
function's output bytes). The OID is recomputable from the inputs.
Cache hits are safe.

**Where it lives.** `Splinter<H>` provides the OID; the au value is a
splinter; the splinter's OID is computed from its canonical bytes.
That part is direct from Claim 1.

**Where it could leak / what's missing.** The composition of the
cache key depends on Claims 3 and 4 landing. Until then, an au
value's OID is computed only from its bytes; the inputs that
*produced* those bytes are not bound into the cache key. If the
model drifts (and Claim 3's hole-only key is in effect), the same
hole produces a different au value, and the cache returns the new
one without flagging the input change.

**Verdict.** ⚠️ Depends on 3 and 4. The OID mechanism is landed; the
inputs that get hashed into it are not yet the right inputs.

### Claim 6 — Verdict semantics are deterministic by definition ✅

**Property.** `Transparency<P>::combine` and
`PropertyVerdict::merge_with` are pure functions. Same inputs → same
outputs. Order-independent (commutative monoid under combine on
non-colliding paths; well-defined merge on colliding paths).

**Where it lives.** `prism/imperfect/src/transparency.rs`. The merge
semantics are tested
([`merge_with_fail_dominates_partial`](../../prism/imperfect/tests/transparency.rs),
[`merge_with_partials_unions_diagnostics_min_confidence`](../../prism/imperfect/tests/transparency.rs),
[`merge_with_pass_is_neutral`](../../prism/imperfect/tests/transparency.rs)).

The shape:

- **Fail** dominates anything (the "dirty bit" propagates).
- **Partial** + **Partial** → **Partial** with the minimum confidence
  and union of diagnostics in declaration order.
- **Pass** is neutral.

This is structural. The verdict for a build is the composition of
verdicts at each crystallized joint; that composition is a pure
function of the per-joint verdicts; the per-joint verdicts are pure
functions of the inputs (modulo the property-implementation
discipline of Claim 7).

**Where it could leak.** Nowhere structurally. The `OpacityMap`
newtype prevents forging an empty Opaque (which would be the
catastrophic absorbing element); the merge semantics are tested.

**Verdict.** ✅ Landed. The verdict composition is deterministic by
the type system, not by discipline.

### Claim 7 — Property checks are deterministic ⚠️

**Property.** Each `@epistemologic/property/*` check is a pure
function of its input AST. Same AST → same verdict.

**Where it lives.** `boot/std/epistemologic/property/*.mirror`.
`coincidence_matches`, `total_classification`, `glass_wall` are the
properties the kintsugi minimum-runnable engine composes at the
kintsugi altitude
([[../specs/kintsugi-minimum-runnable]] §3).

**Where it could leak / what's missing.**

- **The property bodies are `\` (parked).** The discharge happens in
  Rust today (`bootstrap/src/main.rs::count_dark` for
  `total_classification`; the bootstrap's hash-coincidence check for
  `coincidence_matches`; the `@mirror/grammar.is_mirror` ref check
  for `glass_wall`). The Rust bodies have been audited (no
  HashMap-in-loop, no clock reads, no PID reads); the audit is by
  reading, not by property check.
- **The structural defense** is `requires deterministic(check)` on
  each property's substrate declaration. Like Claim 4's seed
  property, this is a substrate-level annotation that the model
  checker can use to refuse non-deterministic property implementations.
- **The yet-to-do.** Land the `requires deterministic` property and
  apply it across all `@epistemologic/property/*` declarations. This
  is one substrate change plus an audit pass; both substrate-pull,
  both small.

**Verdict.** ⚠️ Partial. The properties are deterministic by audit;
they aren't yet deterministic by property check. The fix is
mechanical.

### Claim 8 — DAG traversal is stable ❌

**Property.** Build graph topological sort produces the same order
on every machine. Tie-breaks are deterministic; iteration order
across the crystallizations table is deterministic.

**Where it lives.** Partially. `Splinter<H>`'s Record encoding
iterates a `BTreeMap`, which is sorted-by-key by construction — the
canonical bytes are stable. `Splinter<H>`'s List encoding iterates a
`Vec`, which is order-preserving — also stable.

**Where it could leak / what's missing.**

- **`Crystallizations<H>` uses a `HashMap<Ref, Body<H>>` internally**
  (`bootstrap/src/crystallize.rs:434`). Hash-map iteration order is
  not deterministic across runs. As long as dispatch is by lookup
  (not iteration), this does not affect the verdict; but any future
  addition that iterates the registry (e.g. "list all registered
  refs", "audit registrations") would surface non-determinism.
- **The fix.** Switch the internal representation to `BTreeMap<Ref,
  Body<H>>` ordered by `Ref`'s string bytes. `Ref` is a hash-blind
  newtype with a string representation; `Ord` is straightforward.
  The change is one type swap in `crystallize.rs`.
- **Build-graph traversal order** is not yet defined because the
  build graph isn't yet wired beyond `kintsugi_tick`. When it
  lands (per `kintsugi-minimum-runnable.md` §9 ticks), the
  traversal must use a topological sort with `Ref`-bytes as the
  tie-break. This is the standard discipline (Nix's derivation
  evaluation is the reference shape).

**Verdict.** ❌ Yet to do. Two small changes: BTreeMap in the
registry, topological sort with Ref-bytes tie-break in the build
walker.

### Claim 9 — @io boundary discipline enforces determinism ❌

**Property.** When a body wraps an external tool (cargo / rustc / a
file-system operation), the wrapping enforces determinism: relevant
flags set (`-C codegen-units=1`, `SOURCE_DATE_EPOCH=0`), no
`~/.cache` leaks, hermetic toolchain.

**Where it lives.** `AGENTS.md` §"The Glass Wall" defines `@io` as
the only legitimate non-mirror surface; the
`@epistemologic/property/glass_wall` check enforces that non-mirror
substrate lives under `@io`. The discipline that *within `@io`,
external tool invocations are deterministic* is named in
[[../specs/au-and-conductivity]] as "stage 2" of the kintsugi
ladder but not yet enforced.

**Where it could leak / what's missing.**

- **The Rust toolchain on the host machine** is the deepest residue.
  Even with `SOURCE_DATE_EPOCH` and `-C codegen-units=1`, the rustc
  binary itself is whatever the system has. Cross-machine
  reproducibility requires either (a) pinning the rustc binary's
  OID in the substrate (the Nix flake answer), or (b) accepting
  that the *bootstrap* is per-machine and the kintsugi-built
  artifacts above it are cross-machine. The second is what mirror
  has today; the first is the v2.0+ aspiration.
- **The structural defense** is `@io` wrappers that *declare* the
  determinism flags they pass. A wrapper for `rustc` whose
  declaration is `requires deterministic(rustc, flags = {
  codegen-units: 1, source-date-epoch: 0, ... })` is auditable; the
  model checker can refuse calls that don't include the flags.
- **Cargo's `build.rs`** is the biggest leak in this category
  (per [[prior-art]] §1.8). Mirror's substrate cannot directly
  contain a Rust `build.rs`; if one is needed (e.g. for FFI), it
  lives in `@io` and the same determinism-flags discipline applies.

**Verdict.** ❌ Yet to do. The `@io` wrappers exist; the
determinism-flag declaration on them does not. This is the largest
single piece of yet-to-do work in the chain; for v1.0 (mirror's
bootstrap is per-machine) it may be acceptable as-is, with the
cross-machine claim deferred to v1.x.

---

## 3. The reproducibility chain — what stands today vs what's owed

Going up the chain from input bytes to final artifact:

| Layer | Property | Today |
|---|---|---|
| Source bytes | Content-addressed via Splinter OID | ✅ Landed |
| Hash function | BLAKE3 default; generic-over-hash via MerkleHash | ✅ Landed |
| Substrate declarations | Content-addressed (declaration OID) | ✅ Landed |
| Fracture inputs | Content-addressed via candidate OID | ✅ Landed |
| Loss verdict composition | Deterministic by PropertyVerdict::merge_with | ✅ Landed |
| Property check determinism | By audit; not yet by property check | ⚠️ Partial |
| @fate model weights | OID in store | ✅ Landed |
| @fate cache key includes model OID | Single substrate change | ⚠️ Partial |
| @fate inference seed-pinned | Three substrate changes | ❌ Yet to do |
| Au value cache key composition | Depends on the above | ⚠️ Partial |
| Crystallizations iteration order | BTreeMap swap | ❌ Yet to do |
| @io tool wrapper determinism flags | Substrate-level wrapper change | ❌ Yet to do |
| Cross-machine toolchain reproducibility | Per-machine in v1.0; flake-shape in v1.x | ❌ Deferred |

The chain is half landed. The work to close it is small and named:
six substrate-level changes plus one audit pass. None of them is
architecturally novel; all of them are localised.

The honest read: **kintsugi can hit the reproducibility bar for
hand-written bodies today** (because hand-written bodies inherit Nix's
discipline directly). **Kintsugi cannot yet hit the bar for the au
column** because the four-piece pinning (model OID, seed, temperature,
sampling policy) isn't yet wired. The pinning is the work; nothing
about it is research.

---

## 4. What the metaphor models, and where it stops

Kintsugi (Japanese, *"golden joinery"*) is the pottery practice of
repairing broken ceramics with lacquer mixed with powdered gold. The
metaphor maps cleanly to the engineering for three reasons:

1. **The gold is structural, not cosmetic.** A kintsugi-repaired bowl
   holds water through the gold-filled cracks; the gold is the
   bowl's load-bearing material at the joint. An au-bound joint is
   the build's load-bearing material at the place where no
   hand-written body exists.
2. **The repair is visible, by design.** Kintsugi pottery is more
   valuable than the unbroken original because its history is
   legible in the joint. An au-bound build artifact is *more
   honest* than a build whose every body was pre-declared, because
   the type system carries the fact of inference, and the build
   report names which joints needed it.
3. **The repair is content-addressed.** Each crack is filled with
   gold-of-this-mix, not generic gold; remix the recipe and the
   repair changes. Content-addressing au values by (hole, model,
   config) is the same discipline.

Where the metaphor stops:

- **Kintsugi pottery is non-reversible.** Once the lacquer cures,
  the bowl is the bowl. The build artifact is reproducible; you can
  rebuild it from the recipe at any time. The metaphor is about the
  joint, not about persistence.
- **Kintsugi pottery has one repair per crack.** A build might
  re-resolve the same hole differently across model upgrades. The
  metaphor's "one repair forever" is not the build's "same repair
  given same inputs". The reproducibility property is the second,
  not the first.
- **Kintsugi pottery has a craftsperson.** The metaphor's "gold
  filling the crack" is the work of a human artisan. The build's
  "gold filling the crack" is `@fate.infer` — a substrate-declared
  inference function whose discipline is what makes the gold the
  same gold every time. The artisan is the substrate.

The metaphor is useful because it names the architectural shape
(visible repair as load-bearing material). It is not useful as a
correctness argument; correctness comes from the nine claims above.

---

## 5. What kintsugi inherits, and where it owes

| Inheritance | Source | What kintsugi takes |
|---|---|---|
| Hash-of-recipe → hash-of-output | Nix (Dolstra 2006) | `Splinter<H>` is the derivation hash; crystallization OID is the derivation. |
| Content-addressed storage | Venti (Quinlan & Dorward 2002) | `@mirror/store` is Venti for substrate values. |
| Hermeticity by construction | Bazel (Google 2007) | The local-by-construction invariant on `@fate` is the same shape, made mathematical. |
| Marketplace as body registry | GitHub Actions (2018) | `Crystallizations` is the typed marketplace; SHA-pinning is the default. |
| Verb / noun split | Tekton (2019) | `Crystallization<H>` ⇆ Task; crystallize call ⇆ TaskRun. |
| Reconciliation loop | ArgoCD / Flux (2016–2018) | `kintsugi_tick` is the reconciler; OID-tree equality is the convergence check. |
| Function composition as build | Dagger (2022) | `Body<H>: Fn(…) -> Imperfect<…>`. |
| Pinned-seed inferred output | Halide auto-scheduler (2019) | Existence proof that AI-inferred outputs can be reproducible given pinned cost model + seed. |
| Synthesis under typed contracts | Sketch / Synquid (2008 / 2016) | The `requires` properties are the contract; @fate is the synthesis. |

What kintsugi owes (the yet-to-do):

- **Pin model OID in cache key** (Claim 3).
- **Pin seed, temperature, sampling policy in cache key** (Claim 4).
- **Land `requires deterministic` property for @fate inference**
  (Claim 4).
- **Land `requires deterministic` property for @epistemologic
  property implementations** (Claim 7).
- **Switch `Crystallizations<H>`'s internal representation to
  `BTreeMap`** (Claim 8).
- **Add determinism-flag declarations to @io tool wrappers**
  (Claim 9).
- **Cross-machine toolchain reproducibility (flake-shape)** — deferred
  to v1.x.

Each is a localised substrate change. None depends on the others
landing first (except 5, which depends on 3 and 4 for the cache key
composition). The work is sequencable and small.

---

## 6. The honest test

The thesis is testable. The test that would prove or refute it is:

1. Pick a non-trivial corpus (e.g. the boot tree, ~160 .mirror files).
2. Build it on machine A. Capture the final artifact's OID. Capture
   every au-bound joint's OID.
3. Build it on machine B (different OS, different libc, different
   CPU model). Capture the same OIDs.
4. Build it on machine A six months later. Capture the same OIDs.
5. The three OID sets must be byte-identical for the thesis to hold.

Today, the thesis would partially pass:

- Hand-written bodies' outputs are reproducible across the three
  builds (the bootstrap's Rust + `SOURCE_DATE_EPOCH` + `-C
  codegen-units=1` discipline gets us to per-machine
  reproducibility; cross-machine wants Claim 9's flake-shape
  toolchain).
- Au-bound joints would *not* be reproducible across the three
  builds (because Claim 3's model-OID cache key isn't yet wired
  and Claim 4's seed isn't pinned).

The honest claim today is: **kintsugi's hand-written path is
reproducible-with-discipline; the au column is not yet.** The work to
close the gap is the six changes in §5. None is novel; none is large.

When all six land, the thesis becomes a falsifiable engineering
property that the test above checks. Until then, the claim is
*"kintsugi can be reproducible if we do X, Y, Z; today X is in place,
Y is partial, Z is named."*

That is the substance.

---

*Reproducibility is engineering. Determinism is engineering. The
metaphor is the model; the claim is the test. The gold conducts when
its inputs — including the model that mixed it — are named by hash
and refuse to drift.*

Apache-2.0.
