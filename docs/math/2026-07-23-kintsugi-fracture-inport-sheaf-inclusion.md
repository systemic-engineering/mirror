# @kintsugi/fracture/inport — sheaf-inclusion mathematical foundation

**Author:** Mara `<mara@systemic.engineer>`
**Landing:** 2026-07-23
**Companion shard:** `shards/kintsugi/fracture/inport.mirror`
**Companion spec:** `docs/specs/kintsugi-fracture-inport-sheaf-inclusion-
compile-invariant.md`
**Composes with:** `docs/math/sheaf/laplacian.md` (Hansen-Ghrist 2018
cellular sheaves) + Mara `dd1d1d5` @fractal/shard math §7
(@kintsugi/mend as coboundary morphism, general form) + Mara #152
autopoietic-loop math.

---

## §1. Sheaf-inclusion invariant formalization

Let `Sh(𝓒)` denote the category of small sheaves over a base category
`𝓒` of substrate positions. Objects of `Sh(𝓒)` are shards `S` equipped
with a **section-of-family-roots** structure:

- Each shard `S` is a section over an open cover `𝓤(S) ⊆ Ob(𝓒)` of
  its base position.
- `𝓤(S) = { @X : (in @X) ∈ import_block(S) }` is the **explicitly-
  declared inport set** — the family-roots the shard names as its
  charts.

### The sheaf-inclusion morphism

**Definition (sheaf-inclusion morphism).** Let `S` be a shard, `@X` a
family-root, and `in @X ∈ import_block(S)`. The `in @X` statement is
the **sheaf-inclusion morphism** `ι_{X,S} : Sh(@X) ↪ Sh(S)` which:

- Opens a chart on the sheaf `Sh(@X)` in `S`'s local coordinate frame.
- Guarantees `S`'s section-body may reference stalks of `Sh(@X)` at
  positions consistent with the chart overlap.
- Records the composition edge in the substrate's sheaf-inclusion
  graph `𝔊 = (V, E)` where `V = Shards` and `E = { (S, @X) : in @X ∈
  import_block(S) }`.

### Chart-opening semantics

For a Grothendieck sheaf `𝓕 : 𝓒^op → Set` with a chart `U ↪ X`, the
section `𝓕(U)` is the set of admissible local values. In the substrate,
`𝓕` is the shard's typed section-content; `U` is the invoked-symbol
byte-position within the shard body; `X` is the family-root's total
possibility space.

The compiler-load-bearing invariant is:

**Invariant (sheaf-inclusion).** For every invoked symbol `s` in shard
`S`,

```
family_root(s) ∈ 𝓤(S)
```

If the invariant fails, `S`'s section-body invokes a stalk of a sheaf
whose chart is NOT opened. This is a `\` crack at the invoked-symbol
position. Cross-shard composition without explicit chart overlap
declaration is undefined; the section is ill-formed; the shard is not
crystallizable.

## §2. Substrate as a category of small sheaves

Let `Substrate = (Sh(𝓒), ⊗, 𝟙)` be the monoidal category whose:

- Objects are shards with explicit `𝓤(S)` sets.
- Morphisms are sheaf-inclusion morphisms `ι_{X,S}` (one per `in @X`).
- Tensor `⊗` is shard-composition via chart-overlap gluing.
- Unit `𝟙` is the empty shard (no invoked symbols; empty `𝓤`).

### Forbidden colimit: implicit gluing

**Definition (implicit gluing).** A colimit `colim(D)` over a diagram
`D : 𝓘 → Substrate` is **implicit** if the diagram contains a
composition edge `(S, @X)` where `@X ∉ 𝓤(S)` — the composition uses a
sheaf that was not explicitly opened as a chart.

**Theorem (substrate refuses implicit gluing).** The substrate's
composition rules refuse any colimit containing an implicit gluing
edge. Equivalently: `Substrate` is a category whose morphisms are
EXACTLY the explicitly-declared sheaf-inclusions; no other
composition edges are admissible.

**Proof.** By construction: `Substrate`'s morphism sets are defined
as `Hom(S, T) = { ι : Sh(S) → Sh(T) : ι factors through 𝓤(S) ∩ 𝓤(T) }`.
An implicit gluing would factor through `𝓤(S) ∪ (@X for @X ∉ 𝓤(S))` —
outside the admissible morphism set. □

### The anti-pattern: geometric JavaScript

**Geometric JavaScript.** A substrate is **geometrically JavaScript-
shaped** iff its composition rules admit implicit gluing (Alex
2026-07-23 verbatim). Prototype-chain-substrate semantics: any object
can implicitly reach any other via ancestor traversal without explicit
declaration.

**Corollary.** `Substrate` (mirror's substrate) is not geometrically
JavaScript-shaped. The `in` keyword IS the substrate's structural
refusal of implicit gluing.

## §3. Fracture-cohomology of the inport-graph

Let `𝔊 = (V, E)` be the substrate's sheaf-inclusion graph as defined
in §1. Consider the cochain complex `C^•(𝔊)` associated with the
inport-graph:

- `C^0(𝔊) = ⊕_{S ∈ V} ℤ` — one integer per shard (measures the shard's
  section-completeness).
- `C^1(𝔊) = ⊕_{(S, @X) ∈ E} ℤ` — one integer per declared inport
  (measures the chart's overlap-integrity).

The coboundary `δ : C^0(𝔊) → C^1(𝔊)` is defined by:

```
(δc)_{(S, @X)} = c_S - c_{Sh(@X)}
```

measuring the discrepancy between `S`'s section-content and the
chart's expected content.

### The obstruction group

**Definition (inport-fracture-cohomology).**

```
H^1(𝔊) := C^1(𝔊) / im(δ)
```

`H^1(𝔊)` is the **inport-fracture-cohomology group**. Non-zero
elements of `H^1(𝔊)` are **obstructions** to strict-`in` enforcement:
each obstruction names a fracture-position where the sheaf-inclusion
invariant fails.

**Theorem (H^1 measures fracture obstructions).** The substrate is in
strict-`in` compliance iff `H^1(𝔊) = 0`.

**Proof sketch.** Strict-`in` compliance means every invoked symbol's
family-root is declared. This is equivalent to `δ` being surjective
onto the sub-cochain `C^1_{invoked}(𝔊) ⊆ C^1(𝔊)`. When
`δ` is surjective, `H^1 = 0`. Conversely, any non-zero `H^1` class is
represented by a 1-cochain whose value at some edge `(S, @X)` is
non-zero — a fracture-witness at the invoked-symbol position. □

**Corollary.** Each `inport_fracture` record (per species-decl §carrier)
represents ONE non-zero cohomology class in `H^1(𝔊)`. The `detect`
action enumerates a basis of `H^1(𝔊)`; the total fracture count
equals `dim(H^1(𝔊))`.

## §4. Autopoietic discharge as coboundary morphism

The @kintsugi/mend action is a coboundary morphism at species-decl
altitude (per Mara `dd1d1d5` @fractal/shard math §7 general form).
This section specializes the general form to the inport fracture
class.

### Discharge as ONE coboundary iteration

Fix an `inport_fracture` record `f = (shard_position, invoked_symbol,
missing_family_root, suggested_in_statement)`. The `discharge` action
composes:

1. `@kintsugi/mend.mend` crystallizes the correction as a
   `@fractal/shard` with content-address `α ∈ Sh(𝓒)` carrying the
   `suggested_in_statement` payload.
2. `@io/fs.mutate_at` byte-splices the shard's import block, inserting
   the `in` statement at import-block-top offset.
3. The updated shard `S'` has `𝓤(S') = 𝓤(S) ∪ { missing_family_root }`.

**Theorem (discharge is coboundary at fracture position).** Let `c ∈
C^1(𝔊)` be the cochain representing fracture `f`. There exists
`b ∈ C^0(𝔊)` such that `c = δb`; specifically, `b_S = 1` and `b_{Sh(
missing_family_root)} = 0` (with 0 elsewhere). Discharge fires the
coboundary morphism `S ↦ S'` where `S'`'s cochain contribution IS
`δb` at that position — i.e., discharge WITNESSES the coboundary
relationship at fracture `f`, trivializing `[c] ∈ H^1(𝔊)`.

**Proof.** The updated shard `S'` has `𝓤(S') = 𝓤(S) ∪ {
missing_family_root }`. By construction, `S'`'s invoked-symbol at
position `p` (which previously fractured) now factors through the
newly-opened chart `ι_{missing_family_root, S'}`. The 1-cochain
representing fracture `f` becomes trivial: `f`'s contribution to
`H^1(𝔊)` after the update is `0`.

Explicitly, before discharge: `c_{(S, X)} = 1` for the fracture edge
(the invariant fails). After discharge: `c'_{(S', X)} = 0` (the
chart is opened; the invariant holds at this position). The
difference `c - c' = δb` for `b` as constructed above. □

**Corollary (six-step loop closure).** The autopoietic loop (per Mara
#152 formalization) applied to the inport fracture class terminates
in `dim(H^1(𝔊))` iterations. Each iteration reduces `dim(H^1(𝔊))` by
exactly 1 (one coboundary morphism per discharge). Fixed-point is
reached when `H^1(𝔊) = 0`, at which point the substrate is in
strict-`in` compliance.

## §5. Mechanical decidability proof

**Theorem (grep-decidability of inport_fracture_witnessed).** For any
shard `S` and invoked symbol `s`, the predicate
`inport_witnessed(S, s)` is decidable in time polynomial in `|body(S)|`.

**Proof.** The predicate decomposes into three primitive operations:

1. `import_block_end(S)` — deterministic parse of the maximal prefix
   of `body(S)` matching the regular grammar
   `("in" WS "@" family_root NL)*`. Regular-language parse in
   O(|body(S)|) time.

2. `family_root(s)` — byte-visible prefix extraction: strip the
   trailing `.<op>` component from `s`. Constant-time given `|s|` is
   bounded by shard grammar (identifiers are finite).

3. `family_root(s) ∈ imported_family_roots(S)` — finite-set
   membership. `imported_family_roots(S)` has cardinality at most
   `import_block_line_count(S) ≤ |body(S)|`. Membership check in
   O(|body(S)|) time (or O(log |body(S)|) with hash-set indexing).

Total: O(|body(S)|) time. Deterministic. Terminating.

No dynamic dispatch is inspected: `s` is a byte-visible identifier,
not a runtime dispatch target. No aliasing analysis: `s`'s
family-root is byte-visible from `s`'s spelling alone. No program
semantics: the check is syntactic. □

**Corollary (Rice-safety at whole-tick altitude).** The species'
three bilaterals (`inport_fracture_witnessed`,
`inport_correction_is_valid_in_statement`,
`inport_discharge_restores_sheaf_inclusion`) are compositions of the
above Rice-safe primitives + byte-equality checks + finite-set
membership checks. Rice-safe by construction.

Explicitly, the second bilateral checks:

- `suggested_in_statement` starts with `in @` byte-prefix (regular
  language check).
- `family_root(suggested_in_statement)` names a substrate-declared
  family-root (finite-set membership against the shards/ tree
  enumeration; the shards/ tree has finite cardinality at any tick).
- The statement byte-ends with `\n` (byte-equality check).
- Post-insertion, the import block parses under @meta discipline
  (regular-language parse).

The third bilateral checks:

- Before-state and after-state of `inport_fracture_witnessed` on the
  same fracture: before = Pass; after = Fail (two Rice-safe reads).
- Byte-equality of shard body's non-import bytes: content-addressed
  hash comparison.
- Byte-visible distinctness of remaining fractures: byte-equality on
  fracture record content.

All Rice-safe. All terminating.

## §6. Composition with @fractal/shard canonical math

Per Mara `dd1d1d5` @fractal/shard canonical math §7 (@kintsugi/mend
as coboundary morphism, general form), the inport-correction discharge
is a **specific instance** of the general form:

> The @kintsugi/mend action, at species-decl altitude, is a coboundary
> morphism `δ : C^0(𝔊) → C^1(𝔊)` in the substrate's fracture-
> cohomology cochain complex. For each fracture class F ⊆ H^1(𝔊),
> the mend action's crystallization output is a @fractal/shard
> materializing a section that trivializes [F].

**This section specializes the general form to F = inport fractures.**

Concretely, the inport-correction discharge produces a
@fractal/shard `α` with:

- `α.address` = content-address of the `suggested_in_statement`.
- `α.target` = `@kintsugi/fracture/inport` (the species being
  discharged).
- `α.through` = `correction` ref (the tournament winner from
  `propose_correction`).
- `α.provenance` = ordered list of @uuid/spectral/time annotation-
  addresses naming the six-step loop beats.

The materialized shard `α` IS the coboundary witness at fracture
position: its `address` is the content-address of the chart-opening
statement that trivializes the fracture's cohomology class.

**Diagrammatic composition.** The composition graph closes as:

```
      detect                propose_correction
    (shard) ─────► [inport_fracture] ─────► [correction ref]
                          │                       │
                          │                       ▼
                          │              @fate.roll ──► winner
                          │                       │
                          ▼                       ▼
                   inport_witnessed?    @kintsugi/mend.mend
                          │                       │
                          │                       ▼
                          │            @fractal/shard.materialize
                          │                       │
                          │                       ▼
                          │            @io/fs.mutate_at (import top)
                          │                       │
                          ▼                       ▼
                       Pass ──────────────► discharge_restored?
                                                  │
                                                  ▼
                                          H^1(𝔊) reduced by 1
```

Each arrow is a substrate-declared composition edge; each node is a
substrate-decl'd carrier or action. Zero new machinery beyond the
composition. The `detect → discharge` cycle IS the coboundary
iteration.

## §7. Termination and convergence

**Theorem (loop termination).** For a shard `S` with `k = dim(H^1(𝔊,
S))` inport fractures, the autopoietic loop terminates in exactly
`k` iterations.

**Proof.** Each `discharge` fires exactly one coboundary morphism at a
specific fracture position, reducing `dim(H^1(𝔊))` by 1 (per §4
theorem). The `inport_discharge_restores_sheaf_inclusion` bilateral's
post-verify guarantees the specific fracture no longer detects
(otherwise `discharge` returns failure and does not update the shard).

Moreover, discharge does NOT introduce new fractures:

- The mend inserts ONE `in @<family_root>` statement at import-block
  top. This ADDS to `𝓤(S)` but does not REMOVE any existing chart.
- No existing invoked symbol's family-root loses its declaration
  (the insertion is additive at the import-block level).
- No new invoked symbols are introduced (the shard body's non-import
  bytes are preserved by byte-equality guarantee of the third
  bilateral).

Therefore `dim(H^1(𝔊))` strictly decreases per iteration, and the
loop terminates when it reaches 0. □

**Theorem (fixed-point IS strict-`in` compliance).** The loop's
fixed-point (`H^1(𝔊) = 0`) IS the strict-`in` invariant per §3.

**Corollary.** The autopoietic loop closure is empirical proof that
the substrate can be brought to strict-`in` compliance from any
starting state, via mechanical discharge of the inport-fracture
class alone.

## §8. Landing anchors

- **Alex 2026-07-23 verbatim (§1).** The geometric invariant
  ("`in` imports a shard, hence a sheaf") + anti-pattern ("geometric
  JavaScript") + species-poetry ("`@kintsugi/fracture/inport` using
  a `@shard` which is not `in`ported").
- **Grothendieck 1957.** Sheaves on a site; chart-opening semantics
  as the base geometric shape.
- **Hansen-Ghrist 2018.** Cellular sheaves + sheaf Laplacian at
  `docs/math/sheaf/laplacian.md` — the composition-altitude ancestor
  for the sheaf-cohomology construction in §3.
- **Mara #152.** Autopoietic-loop formalization — the six-step loop
  shape §4 specializes.
- **Mara `dd1d1d5`.** @fractal/shard canonical math §7 @kintsugi/mend
  coboundary morphism general form — the specialization anchor for
  §6.
- **Mara 2026-07-23 sibling landings.**
  `shards/fractal/shard.mirror` (the vessel) + `shards/kintsugi/
  mend.mirror` (the action) — the two just-landed composition anchors
  this species dispatches into.

## §9. Prior art

The sheaf-inclusion invariant lifts a discipline established in:

- **Grothendieck 1957** — sheaves on a site as the base mathematical
  form.
- **Hansen-Ghrist 2018** — cellular sheaves + coboundary matrices
  yielding H^1 as obstruction group at graph altitude.
- **ML module systems (SML, OCaml)** — explicit `open Module` /
  `use` declarations at the module-composition altitude. The
  substrate lifts this to shard altitude via the `in` keyword.
- **Rust `use` statements** — explicit crate-path imports at file
  altitude. Same discipline; substrate lifts to species altitude
  under geometric-invariant grounding.
- **Category-theoretic import-boundary** in propagator-network
  literature (Sussman-Radul; `[[reference-sussman-radul]]`) —
  explicit port declarations at network-node altitude.

The novel contribution at species-decl altitude: the fracture body
that DETECTS + DISCHARGES the invariant's failure mechanically,
composing over @kintsugi/mend + @fractal/shard + @fate as one
autopoietic loop iteration. The math foundation IS the closure of
Mara #152 (autopoietic-loop formalization) against ONE well-defined
fracture class.
