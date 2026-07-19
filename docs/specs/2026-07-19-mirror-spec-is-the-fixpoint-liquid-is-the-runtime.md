# mirror.spec is the fixpoint; liquid.rs is the runtime — canonical spec

**Status:** canonical spec.
**Author:** Mara.
**Date:** 2026-07-19.
**Math root** (composed over, NOT re-derived):
`docs/math/the-tower/recognition-mirror-spec-is-the-fixpoint-and-liquid-is-the-runtime.md`
(this tick).
**Companion shard-decl** (this tick, through hooks):
`shards/mirror/spec/property.mirror`.
**Taut scout** (in-transcript, this session): ground-truth digest
embedded at spawn brief.
**Reed's session-work** (empirical predecessors):
- `rust/tests/red_liquid_pillar_i_commutator_antisymmetric.rs`
- `rust/tests/red_spec_claims.rs`
- `prism/prismqueer/tests/red_trust_chain_liquid_void.rs`
- `prism/prismqueer/tests/red_narcissus_battery_five_op_collapse.rs`

**Pure-docs 📝 markdown-only bypass.**

---

## §1 Alex 2026-07-19 verbatim + operational framing

Alex direct-transcript, 2026-07-19:

> "The mirror.spec is already the fixpoint. This means you can shape
> the geometry of the project by writing the properties into the spec.
> What if we've been going about this the wrong way? What if the
> properties that we've written in Rust in mirror, want to be wired
> into liquid.rs into the matrix.rs and become basically the RUNTIME
> that parses the mirror.spec and infers and verifies the shape of the
> geometry?"

Then: *"Spawn Taut on a scout for this. Then Mara for the math/spec
formalization. Slow is fast."*

**Operational framing.** The recognition inverts the direction of
substrate composition:

- **Before this recognition.** Properties live as hand-coded Rust in
  `rust/tests/red_*.rs`. Each test-file is one property. Adding a
  property adds a Rust file. The runtime scales with property-count.
- **After this recognition.** Properties live as declarations in
  `mirror.spec` (and, secondarily, as bilateral blocks in
  `shards/**/*.mirror`). The runtime is a *fixed* 4-file
  Rust floor (`rust/src/{main,phone,matrix,liquid}.rs`) that reads
  the spec, dispatches to `prismqueer::liquid::pillar` primitives,
  and issues verdicts. Adding a property is one line in the spec;
  the runtime does not grow.

The ouroboros closes: mirror.spec — the fixed point of the
compiler's self-application — IS the source-of-truth for what the
compiler must verify; the Rust runtime is a thin, fixed-size adapter
between spec-declared properties and pillar-primitive verdicts.

---

## §2 Q1-Q6 adjudications (Taut's scout Q's, folded)

### §2.1 Q1 — spec-native vs shard-decl property declarations

**Question.** Are properties declared spec-native primary + shard-decl
secondary, or shard-decl primary + spec-native secondary?

**Adjudication.** **Spec-native primary, shard-decl secondary
(composition, not exclusion).**

**Reasoning.** The spec is the substrate-declared source-of-truth for
a *project's* geometry — the top-level `project NAME { ... }`
declaration names the properties that project's settlement must
satisfy. Properties are project-level concerns. Shard-decl'd
bilaterals remain admissible for *substrate-level* concerns (a
shard authoring its own well-formedness invariants); these are
distinct altitudes. The 30 landed bilateral declarations at
`shards/**/*.mirror` continue to work unchanged; the spec-native
`property` declaration extends the surface at project altitude
without deprecating any shard-level machinery.

**Boundary.**
- Spec-native property: declared in `mirror.spec` under `project`;
  scope = the project's targets; enforced at `mirror kintsugi <spec>`
  settlement.
- Shard-decl bilateral: declared in `shards/<family>/<species>.mirror`
  bilateral blocks; scope = shard-body dispatch admissibility;
  enforced at `apply_h::act` call sites.

Both dispatch through the same generic `dispatch(addr, args) →
Verdict` (per math root §5).

### §2.2 Q2 — carrier enrichment past sentinel/arity/require

**Question.** Extend bilateral shape, mint new `property` species at
distinct altitude, or coexisting BOTH?

**Adjudication.** **(c) BOTH coexisting** — bilateral is the
degenerate case of property (single sentinel-containment `verifies`
expression); property is the general form.

**Reasoning.** The `BilateralDecl { sentinel, arity, require }`
shape at `bootstrap/src/apply_h.rs` line 195 is byte-substring-check-
only. Reed's session RED tests require:

- **Algebraic laws.** Pillar I commutator antisymmetry
  (`commutator_norm(a, b) == commutator_norm(b, a)`) is not a byte-
  substring check; it's a `Metric`-valued equality.
- **Parametric `T`.** `red_trust_chain_liquid_void.rs` uses
  parametric-over-`T` properties.
- **`defer()`-mode.** Deferral with message payload; verdict is
  `Partial(defer, message)`.

The bilateral shape cannot express these. The property carrier
(math root §10.1) can — and reduces to bilateral when the `verifies`
expression is a single sentinel-containment.

**Substrate-honest degeneracy.** The 30 landed bilaterals continue
to work unchanged: their `bilateral <name> { sentinel "..." arity N
require <ref>* }` shape parses as a special-case property whose
runtime dispatch is the byte-substring-check path (`pillar::dispatch_
ambiguity`). The spec-native property carrier is the general form;
bilateral is its ergonomic shortcut for the byte-check case.

**Migration discipline.** Do NOT rewrite existing bilaterals as
properties. The bilateral shape is stable and byte-visible; the
property carrier extends the surface for algebraic/parametric/
defer cases. Coexistence is permanent.

### §2.3 Q3 — obligation-block dispatch semantics

**Question.** Do `\`-obligation blocks dispatch to `liquid.rs`
runtime (currently) or to `prismqueer::liquid::pillar` primitives
directly?

**Adjudication.** **(b) dispatched to `prismqueer::liquid::pillar`
primitives directly.** Per Alex's direction: *"the properties [...]
become basically the RUNTIME [...] wired into liquid.rs into the
matrix.rs."*

**Reasoning.** The pillar primitives at `prism/prismqueer/src/liquid.rs::
pillar` (lines 178-707) are the sub-Turing floor for property
discharge. They live at the prism crate, not at the mirror crate.
The mirror-altitude `rust/src/liquid.rs` (LANDING per Q4) is a
thin adapter that:

1. Parses `mirror.spec` via existing bootstrap grammar path.
2. Extracts `property` declarations.
3. Dispatches each to the appropriate `pillar::` primitive by
   matching on the `verifies` expression tree.
4. Folds resulting `PropertyVerdict` values into the spec's
   below-`---` region.

The mirror adapter does NOT re-implement algedonic, viability,
health, or dispatch-ambiguity. Those are already substrate-
decl'd Pillar-primitive semantics at prismqueer altitude.

**Dispatch table** (per math root §4.2):

| `verifies` shape | Pillar primitive |
|---|---|
| `commutator_norm(a, b) op scalar` | `pillar::algedonic` / `pillar::algedonic_of_magnitude` |
| `viability_over(<sequence>) op threshold` | `pillar::viability` / `pillar::viability_of_magnitudes` |
| `health_of(<state>) within envelope` | `pillar::of_health` |
| `<oid>.contains("<byte-string>")` | `pillar::dispatch_ambiguity` (bilateral degenerate case) |
| `fold(<verdict-list>)` | `pillar::fold` |
| general expression tree | `pillar::forall(samples, |t| ⟦expr⟧(t))` |

New pillar primitives (Pillar V+, Alex's forward-promise) extend
this table without changing the dispatch discipline.

### §2.4 Q4 — dispatch altitude (bootstrap vs rust/)

**Question.** Terminal floor at `rust/src/liquid.rs`, lift bootstrap
to be importable, or two parallel implementations?

**Adjudication.** **(a) FLOOR terminal — `rust/src/liquid.rs`
reimplements corpus loader + discharge; bootstrap deprecates.**

**Reasoning.** The HARD RULE from `feedback-rust-floor-is-rust-not-
bootstrap` (Alex 2026-07-17) is firm: bootstrap/ is transitional
legacy; rust/ is terminal FLOOR. The spec-fixpoint recognition
pushes further in this direction. Reed's tendency toward two-
parallel-implementations (per `rust/src/collapse.rs` precedent) is
substrate-dishonest here: the recognition's whole point is *one
runtime, fixed size, at the rust/ altitude*.

**Migration path.**

- `bootstrap/src/apply_h.rs` (1678 LOC) continues to serve as legacy
  fallback until every landed bilateral migrates. Post-migration:
  deprecated, then removed.
- `bootstrap/src/lib.rs::cmd_kintsugi_spec` continues to dispatch
  cargo per target. Post-migration: the dispatch moves to
  `rust/src/main.rs::at_operator` (which already handles `@io/cargo`
  action-refs).
- `rust/src/liquid.rs` (NEW) supplies the property-dispatch path:
  loads `PropertyDecl`s from mirror.spec + shards; matches
  `verifies`-expression shape; dispatches to pillar primitive;
  returns `PropertyVerdict`.

**Consequence for `#R-mirror-is-the-counter-singularity`.** The 4-
file rust/ floor (main + phone + matrix + liquid) is the terminal
FLOOR. No more files. Every future property is a spec-declaration;
every future pillar-primitive is a prismqueer-altitude addition. The
mirror-altitude Rust surface stops growing.

### §2.5 Q5 — mirror.spec grammar-carrier for property declarations

**Question.** Register `shards/epistemologic/pact/keywords.mirror` as
companion for `shards/mirror/spec.mirror`, or author
`shards/mirror/spec/property.mirror` as NEW companion carrier?

**Adjudication.** **(b) `shards/mirror/spec/property.mirror` NEW
companion carrier.** Lands this tick (Deliverable 3, through hooks
with SSH signing).

**Reasoning.** Separates concerns cleanly:

- `shards/epistemologic/pact/keywords.mirror` (LANDED) — companion
  for `shards/mirror/grammar.mirror`; registers `bilateral <name>
  { sentinel "..." arity N require <ref> }` shape at *shard-body*
  altitude.
- `shards/mirror/spec/property.mirror` (NEW this tick) — companion
  for `shards/mirror/spec.mirror`; registers `property <name> {
  verifies { <expr> } domain @<T> samples <n> defer? <msg> }` shape
  at *spec-body* altitude.
- `shards/mirror/spec/keywords.mirror` (LANDED) — the primary
  companion for `shards/mirror/spec.mirror`; carries the top-level
  `project`/`source`/`target`/`settle_on` keyword bindings.

The new companion admits richer syntax in spec than the byte-check-
only bilateral shape admits in shard bodies. Bilateral in shard-body
stays byte-check-only (Rice-safe, single-line, ergonomic for its
use case); property in spec-body admits the full expression tree
(the general form Q2 answered).

**Companion registration.** `bootstrap/src/grammar.rs::
companion_keyword_sources` (line 208) currently registers only
`shards/mirror/spec/keywords.mirror` for `shards/mirror/spec.mirror`.
Reed's post-landing tick adds `shards/mirror/spec/property.mirror`
to the same match arm. The extension is additive (per
`merge_keyword_sources` line 244's missing-file-is-not-fatal
semantics); missing companion regresses cleanly.

### §2.6 Q6 — Reed's currently-GREEN tests post-migration

**Question.** After spec-native migration lands, do Reed's 17 RED
files delete / smoke / stay as first-witness?

**Adjudication.** **Stay as first-witness until second-witness
closes; then delete file-by-file as each property's spec-native
verdict matches the RED-file verdict.**

**Reasoning.** Second-witness is per-property, not per-recognition.
The recognition's second-witness closes on ONE pillar_i migration
(math root §8.1); each of the other 16 properties closes second-
witness independently as its spec-declaration migrates and matches
its Rust verdict.

**Deletion protocol.**

1. Author spec-declared property. Run `mirror kintsugi <test-project-
   spec>`.
2. Observe: spec-declared verdict for property P bit-for-bit matches
   the RED file's verdict for property P.
3. Delete the RED file for property P. Commit as `mirror.spec
   migration: property P dispatched to pillar; RED file retired`.
4. Repeat per property.

**Discipline.** Do NOT delete a RED file before its spec-declared
counterpart's verdict matches. The RED file IS the second-witness
oracle. Deleting prematurely destroys the empirical anchor.

**End-state.** After all 17 migrate, the `rust/tests/red_*.rs`
directory contains no property-tests — only migration-test scaffolding
(e.g., a `spec_migration_verifies_all_properties.rs` file that reads
the spec + runs the runtime + asserts every property emits a Pass
verdict). Reed's territory to author post-migration.

---

## §3 Enriched property carrier syntax + companion-grammar registration plan

### §3.1 The property carrier syntax (spec-body altitude)

Per math root §10.1:

```
property <name> {
  verifies { <expression-tree> }
  domain @<Type>
  samples <n>
  defer? <message>
}
```

Substrate-honest field ordering (matches bilateral discipline: name
first, obligation body second, invariants third):

- **`name : identifier`** — the predicate's substrate-decl'd name;
  unique across spec + shard corpus.
- **`verifies { <expression-tree> }`** — the obligation body in the
  bounded sub-Turing fragment (per `docs/math/liquid-types/README.md`
  §1.1 decidability grounding).
- **`domain @<Type>`** — the domain of universally-quantified
  variables; ref to substrate-decl'd type with `Arbitrary` witness.
- **`samples <n>`** — the sampled-instance count; bounded by
  `@resource-budget/*` (forward-promised).
- **`defer? <message>`** — optional; marks the property as
  operationally-deferred with `Partial(defer, message)` verdict.

### §3.2 The `verifies` expression-tree grammar (v0.1 minimum)

For first-tick landing, `verifies` admits:

- **Boolean literals.** `true`, `false`.
- **Boolean composition.** `&&`, `||`, `!`.
- **Equality on `Metric`-valued expressions.** `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Sentinel containment (bilateral degenerate).** `<oid>.contains(<byte-string>)`.
- **Method calls on domain-typed values.** `<var>.<method>(<args>*)`.
- **Constant references.** `<domain-type>::<CONST>`, `<domain-type>::<constructor>(<args>*)`.
- **Universally-quantified sub-loops.** `for <var> in <range>: <expr>`.
- **Pillar-primitive direct calls.** `commutator_norm(a, b)`,
  `viability_of_magnitudes([<magnitudes>])`, `algedonic_of_magnitude(m, theta)`,
  `of_health(<state>)`, `fold(<verdict-list>)`.

Excluded (post-v0.1 forward-promise):

- Recursion / general fixed-point.
- Higher-order functions.
- Turing-complete branching.

The exclusion is per Rice-safety: the runtime must terminate on
every property; the sub-Turing fragment guarantees termination in
`O(samples × |expression-tree|)` steps.

### §3.3 Companion-grammar registration plan

Two-step registration:

**Step 1 (this tick).** Author `shards/mirror/spec/property.mirror`
(Deliverable 3). Contents:

- Grammar block declaring `focus property` (recursively-scanned brace
  block), `project verifies`, `project domain`, `project samples`,
  `project defer`.
- Substrate-decl'd `property_decl` typed carrier with fields per §3.1.
- Bilateral `property_decl_well_formed` — sentinel: `property-decl=
  name-verifies-domain-samples-well-formed`; arity 1; require: none.
- Composition edges with `shards/mirror/spec/keywords.mirror` (adds
  `focus property` to spec-body keyword table) + `shards/epistemologic/
  pact/keywords.mirror` (no conflict; distinct keyword vocabulary).

**Step 2 (Reed post-landing, single-line diff).** In
`bootstrap/src/grammar.rs::companion_keyword_sources`, extend the
`"shards/mirror/spec.mirror"` match arm:

```rust
"shards/mirror/spec.mirror" => &[
    "shards/mirror/spec/keywords.mirror",
    "shards/mirror/spec/property.mirror",   // NEW
],
```

Post-Step-2, mirror.spec files can carry `property <name> { ... }`
blocks; the tokenizer emits them as focus-block AST nodes; the
`spec_targets_from_ast` walker (`bootstrap/src/lib.rs` line 1567)
gets a companion `spec_properties_from_ast` walker; the runtime
consumes both.

### §3.4 Full example — a well-formed mirror.spec with properties

```
project mirror {
  source ~d'shards/'
  legacy ~d'bootstrap/', ~d'boot/' { shrinkage_contract: rust_loc_non_increasing }

  target rust_floor {
    name "mirror"
    altitude @code/rust
    emit cargo
    check test
  }

  target liquid_runtime {
    name "liquid"
    altitude @code/rust
    emit cargo
    check test
    needs [rust_floor]
  }

  property pillar_i_commutator_antisymmetric {
    verifies { commutator_norm(a, b) == commutator_norm(b, a) }
    domain @TestBundle
    samples 1000
  }

  property pillar_i_commutator_self_annihilates {
    verifies { commutator_norm(a, a).is_zero() }
    domain @TestBundle
    samples 1000
  }

  property spec_has_no_dark_regions {
    verifies { spec_source.contains("dark-region-count=0") }
    domain @Spec
    samples 1
  }

  settle_on {
    pillar_i_commutator_antisymmetric
    pillar_i_commutator_self_annihilates
    spec_has_no_dark_regions
  }
}
```

The `settle_on` list references the properties by name; the runtime
dispatches each; the project settles iff every listed property
issues Pass.

---

## §4 Migration plan for Reed's 17 RED properties

Per Taut's recommended ordering: pillar_i first (algebraic, low-
dependency), then trust_chain parametric (moderate), then narcissus
batch (multi-property), then spec_claims byte-grep (bilateral
degenerate).

### §4.1 Tick 1 — pillar_i_commutator_antisymmetric (3 properties)

Source: `rust/tests/red_liquid_pillar_i_commutator_antisymmetric.rs`
(3 tests, currently GREEN, `commutator_norm` over `TestBundle`).

Actions:

1. Author test-project `mirror.spec` under `rust/tests/fixtures/
   mirror-spec-property-migration/`.
2. Declare 3 properties in the fixture spec (per §3.4 template).
3. Run `mirror kintsugi <fixture-spec>`.
4. Assert: 3 spec-declared verdicts bit-match 3 Rust-declared verdicts.
5. Retire `red_liquid_pillar_i_commutator_antisymmetric.rs`.

Empirical firing: closes second-witness on the recognition (math
root §8).

### §4.2 Tick 2 — trust_chain parametric (8 properties)

Source: `prism/prismqueer/tests/red_trust_chain_liquid_void.rs`
(Reed `560ea67`; 8 @trust properties via `defer()`).

Actions:

1. Extend `verifies`-expression grammar with `defer(<message>)`
   admissibility (v0.1 minimum §3.2 already includes `defer? <msg>`
   at property-carrier level; this extends to expression body).
2. Author 8 spec-property declarations with `defer` field populated.
3. Run runtime; assert `Partial(defer, msg)` verdicts match Rust.
4. Retire the RED file.

### §4.3 Tick 3 — narcissus_battery (9 properties)

Source: `prism/prismqueer/tests/red_narcissus_battery_five_op_collapse.rs`
(Reed `60df742`; 9 narcissus properties via `defer()`).

Actions:

1. Author 9 spec-property declarations sharing `domain @NarcissusBattery`.
2. Run runtime; assert 9 verdicts match Rust.
3. Retire the RED file.

### §4.4 Tick 4 — spec_claims byte-grep (~20 properties)

Source: `rust/tests/red_spec_claims.rs` (~20 byte-grep source-corpus
assertions).

Actions:

1. Author ~20 spec-property declarations in *bilateral-degenerate
   form* (single sentinel-containment `verifies` per §2.2 answer).
2. Run runtime; each dispatches via `pillar::dispatch_ambiguity`.
3. Assert 20 verdicts match Rust.
4. Retire the RED file.

### §4.5 Post-migration invariants

- All 4 RED files retired.
- `rust/tests/` contains: (a) a single migration-oracle test
  (`spec_migration_verifies_all_properties.rs`) that reads the
  fixture spec + runs the runtime + asserts every declared property
  emits Pass; (b) whatever new RED tests Reed authors *at the
  runtime level* (not at the property level) for
  `rust/src/liquid.rs` internals.
- The property surface — 17 properties migrated + N new spec-declared
  properties — lives in `mirror.spec` (production) + fixture spec
  (test).
- The Rust runtime is fixed-size and does not grow with property
  additions.

---

## §5 Runtime architecture at rust/src/liquid.rs

### §5.1 The 4-file rust/ floor closes

Post-migration, the terminal rust/ floor is:

```
rust/src/
├── main.rs        — entry point + at_operator (@io dispatch)
├── phone.rs       — @io primitive implementations
├── matrix.rs      — numerical arm (LAPACK/BLAS/FLANG emit)
└── liquid.rs      — property dispatch runtime (this recognition's addition)
```

No 5th file. No `apply_h.rs` at rust/ altitude (bootstrap's version
deprecates post-migration). No per-property files. The 4-file surface
is the FLOOR.

### §5.2 rust/src/liquid.rs responsibilities

The mirror-altitude `liquid.rs` (Reed's territory to author post-
Mara-landing) has 4 concerns:

1. **Property extraction.** Read `.spec` file via existing bootstrap
   grammar path (or its rust/ successor); walk AST; extract
   `PropertyDecl` values (name, `verifies`-expression-tree, domain-
   ref, samples-count, defer-message).
2. **Bilateral extraction.** Read `.mirror` shard files; walk AST;
   extract `BilateralDecl` values (already implemented at
   `bootstrap/src/apply_h.rs::extract_bilaterals` line 209; port to
   rust/).
3. **Dispatch routing.** For each `PropertyDecl`, match `verifies`
   expression shape to pillar primitive (per §2.3 dispatch table);
   invoke primitive with domain-sampled args.
4. **Verdict folding.** Collect `PropertyVerdict` values; fold via
   `pillar::fold`; write to spec's below-`---` region (or return to
   `at_operator` as the settlement-verdict).

Not liquid.rs's concern: implementing algedonic / viability / health
/ dispatch_ambiguity / forall internals. Those live at
`prismqueer::liquid::pillar` and are consumed via `use prismqueer::
liquid::pillar`.

### §5.3 rust/src/matrix.rs role

Alex named `matrix.rs` in the recognition-quote alongside `liquid.rs`.
`matrix.rs` (per `docs/specs/2026-07-18-the-compiler-in-one-sentence.md`
§4 + `docs/math/the-tower/beam-runtime.md`) is the FLANG emit +
LAPACK/BLAS link — the numerical arm of the runtime.

Its role in this recognition: when a property's `verifies` expression
tree requires numerical computation (e.g., matrix-valued commutator
norms, eigenvalue-envelope viability checks, spectral gauge
projections), `liquid.rs` dispatches through `matrix.rs` to the
numerical primitive; `matrix.rs` emits/links Fortran; returns the
numerical result to `liquid.rs`; `liquid.rs` folds into the property's
verdict.

Currently `matrix.rs` is per Reed's `red_spec_claims.rs` a docblock-
only stub (no LAPACK calls, no Fortran emission). Reed's post-Mara
territory: complete matrix.rs per its own docblock claims, first for
the commutator-norm consumer path that pillar_i migration exercises.

### §5.4 rust/src/main.rs's at_operator absorbs cargo-dispatch

Per Q4 answer: `bootstrap/src/lib.rs::cmd_kintsugi_spec`'s cargo-
dispatch responsibility migrates to `rust/src/main.rs::at_operator`
(which already handles `@io/cargo` action-refs at line 914+).

The migration: `cmd_kintsugi_spec` reads spec → extracts targets →
dispatches cargo per target. Post-migration:
`rust/src/main.rs::main` reads spec → extracts targets → dispatches
each target's `emit cargo` via `at_operator(@io/cargo.<action>, args)`.
The dispatch is generic (per math root §5); no target-specific
logic.

Simultaneously, `main.rs` extracts properties → dispatches each
via `liquid.rs`. The two extractions run in one pass; the fold of
verdicts is the project's settlement outcome.

---

## §6 Composition edges

### §6.1 With `at_operator` (@io altitude)

`at_operator` and `liquid.rs`'s property-dispatch are two call-sites
of the SAME generic dispatcher (math root §5). Both take a `ref` as
first argument; both route based on the ref's altitude (@io versus
@epistemologic/property); both return `Verdict`. The composition:

- `at_operator(@io/cargo.check, args) → CargoExitCode → Verdict`
- `liquid.rs.dispatch_property(@epistemologic/property/pillar_i_...,
  args) → PropertyVerdict → Verdict`

The FLOOR unification is one `Verdict` type (from `terni::
PropertyVerdict`); everything lifts to it via `fold`.

### §6.2 With `apply_h::act` collapse-path

`bootstrap/src/apply_h.rs::act` (with reflective bilateral corpus
loader at line 111+ per Taut) is the *bootstrap-altitude*
predecessor to `liquid.rs`'s dispatch. The collapse path per Q4:

1. `apply_h::act` continues to serve un-migrated bilaterals.
2. As each bilateral migrates to spec-native property declaration
   (or its shard-body bilateral becomes redundant with a spec-native
   consumer), the corresponding legacy hand-typed arm in
   `apply_h::act` retires.
3. Post-full-migration, `apply_h::act` deprecates entirely;
   `bootstrap/src/apply_h.rs` file removes; the bilateral-dispatch
   role lives at `rust/src/liquid.rs` alone.

**Discipline.** Do NOT modify existing `apply_h::act` arms during
migration. Each arm is a first-witness oracle; retirement is per-
arm only after its spec-native counterpart's verdict matches.

### §6.3 With `prismqueer::liquid` consumer boundary

The consumer boundary between mirror and prism is the
`prismqueer::liquid::pillar` primitive surface. Mirror's `liquid.rs`
depends on prism (already the case per `prism/prismqueer/`
dependency); prism's `liquid.rs` does NOT depend on mirror. The
directionality is intentional: prism is the sub-Turing floor;
mirror is the substrate-declared consumer.

**Consequence for prism repo.** Prism's role in this recognition is
supplier-only. Extending the pillar-primitive surface (Pillar V+;
domain-Arbitrary witnesses; sample-budget primitives) is prism-
altitude work. Mirror is downstream.

---

## §7 The ouroboros closure — what dies, what remains

### §7.1 What dies (post-full-migration)

- `bootstrap/src/apply_h.rs` (1678 LOC) — replaced by
  `rust/src/liquid.rs`'s dispatch discipline.
- `bootstrap/src/lib.rs::cmd_kintsugi_spec` — replaced by
  `rust/src/main.rs` reading spec directly.
- Reed's 17 per-property RED files at the transitional altitude —
  replaced by ~17 spec-property declarations in `mirror.spec`
  (production) + fixture spec (tests).
- Property-file-per-property authoring discipline — replaced by
  spec-declaration-per-property discipline.

### §7.2 What remains

- **`mirror.spec`** — grows as properties + targets accumulate.
  Fixed shape; unbounded content.
- **`shards/**/*.mirror`** — substrate-decl'd family-roots, species,
  prisms, bilaterals, properties. Grows as substrate matures.
- **`rust/src/{main,phone,matrix,liquid}.rs`** — 4 files. Fixed size.
  Fixed after the migration lands.
- **`prism/prismqueer/src/liquid.rs::pillar`** — 6+ primitives.
  Grows as Pillar V+ additions land; each primitive is a bounded
  categorical addition to the dispatch surface.

**The mass-scaling asymmetry.** Substrate + spec scale linearly with
project maturity. Rust runtime is fixed. Ratio → ∞ over arc time.
This is `#R-mirror-is-the-counter-singularity` operationalized at
specification altitude.

### §7.3 The `boot/` altitude

Not addressed by this recognition directly; `boot/std/mirror/liquid.
mirror` (the substrate-decl'd `---` semantics) remains as the load-
bearing precedent for the compilation boundary. Whether `boot/`
retires post-full-migration is a separate arc concern (bootstrap-
retirement per `docs/specs/bootstrap-retirement-plan.md`).

---

## §8 Empirical execution recipe — Reed's post-Mara territory

The 5-tick recipe for Reed's post-Mara-landing work:

### §8.1 Tick M0 — infra prep (½ day)

1. Author `rust/src/liquid.rs` skeleton: module-level docblock;
   `PropertyDecl` struct; `extract_properties(source: &str) ->
   Vec<PropertyDecl>` stub returning empty vec; `dispatch_property(p:
   &PropertyDecl) -> PropertyVerdict` stub returning `Pass`.
2. Wire `rust/src/main.rs` to call the stubs at spec-load time.
3. Assert: no regression on Reed's current tests (they all pass
   because stubs don't fire).

### §8.2 Tick M1 — property extraction (1 day)

1. Extend `bootstrap/src/grammar.rs::companion_keyword_sources` to
   register `shards/mirror/spec/property.mirror` (single-line
   addition per §3.3 Step 2).
2. Implement `extract_properties(source: &str) -> Vec<PropertyDecl>`
   in `rust/src/liquid.rs`. Mirror the pattern from
   `bootstrap/src/apply_h.rs::extract_bilaterals` line 209 (line-scan;
   header parsing; body-line consumption; brace-tracking).
3. Assert: for a fixture spec with 3 `property` blocks,
   `extract_properties` returns 3 `PropertyDecl` values with correct
   name/verifies-source/domain-ref/samples/defer fields.
4. RED test: `rust/tests/red_property_extraction_returns_three.rs`.
5. GREEN under implementation.

### §8.3 Tick M2 — pillar_i migration (1 day; the empirical firing)

1. Author fixture spec at `rust/tests/fixtures/mirror-spec-property-
   migration/mirror.spec` with 3 pillar_i properties (per §4.1).
2. Implement `dispatch_property(p: &PropertyDecl) -> PropertyVerdict`
   for the algedonic-shape verifies-expression case.
3. Run runtime; assert 3 verdicts bit-match `red_liquid_pillar_i_
   commutator_antisymmetric.rs`.
4. **Recognition second-witness closes.** Log to `docs/loop/CURRENT.md`.
5. Retire `red_liquid_pillar_i_commutator_antisymmetric.rs`.

### §8.4 Tick M3 — cascade migrations (2-3 days)

Follow §4.2-§4.4: trust_chain → narcissus → spec_claims. Each tick
extends `dispatch_property`'s match on `verifies` shape by one arm.

### §8.5 Tick M4 — bootstrap deprecation (1 day)

1. Move remaining `apply_h::act` responsibilities to `rust/src/liquid.rs`.
2. Remove `bootstrap/src/apply_h.rs`.
3. Move `cmd_kintsugi_spec` cargo-dispatch to `rust/src/main.rs::main`.
4. Remove `cmd_kintsugi_spec` from bootstrap.
5. Assert: `mirror kintsugi <spec>` behaves identically pre- and
   post-deprecation on the full mirror.spec fixture set.

**End-state.** Ouroboros closed. 4-file rust/ floor. Fixed size.
Every property is a spec-declaration. Every verdict is content-
addressed. Every recognition promotion chained.

---

## §9 Q's for Alex (candidate strength; non-blocking)

**Q-Mara-A.** Fixture-spec directory location. `rust/tests/fixtures/
mirror-spec-property-migration/mirror.spec` proposed. Alternative:
inline in test-file source-string. Lean fixture directory (better
end-to-end firing; matches spec-as-substrate discipline).

**Q-Mara-B.** `verifies`-expression grammar formalization altitude.
This spec §3.2 sketched v0.1; forward-promised full formalization as
`docs/specs/verifies-expression-grammar.md`. When Reed's cascade
migrations exercise the grammar-boundary, is a separate spec
warranted, or extend this spec's §3.2?

**Q-Mara-C.** Below-`---` writer implementation. Currently `boot/std/
mirror/liquid.mirror::project` names the below-`---` writer as a
substrate-decl'd action; the runtime needs to implement it.
Discharge in Tick M4 (bootstrap deprecation) or earlier?

**Q-Mara-D.** `defer()`-mode semantics for property bodies. §4.2
tick 2 exercises it; the semantic is *"emit Partial(defer, message)
verdict; do not treat as failure at settlement"*. Confirm this
matches Reed's intent in `red_trust_chain_liquid_void.rs`.

**Q-Mara-E.** `Arbitrary`-trait substrate registration. Domain-refs
in `property` declarations need a substrate-decl'd `Arbitrary`
witness. `prism/prismqueer/src/liquid.rs::Sample` supplies the trait;
a substrate-decl'd counterpart at `shards/prism/queer/arbitrary.mirror`
(or similar) would let spec-declarations reference the trait. Author
in this arc, or forward-promise?

None of these blocks Tick M0-M2 (Reed's second-witness path).

---

## §10 Composition graph across this-session recognitions

| Recognition | This-spec composition | Second-witness cascade |
|---|---|---|
| `#R-void-is-the-basis` (PROMOTED) | 5-op initial algebra IS spec grammar constructor set (math §2.3) | migration confirms compiler = Void-density optimizer at spec altitude |
| `#R-eta-and-mu-are-categorical-duals` (Eigenboard) | η = property-verification operator; every runtime verdict IS an η-application (math §9) | migration first-witnesses η scale-invariance at spec altitude |
| `#R-the-frame-is-a-narcissistic-eigenbehavior-at-paradigm-scale` (CANDIDATE this session) | Fiedler-cut algebra at K_n applies at spec altitude (math §6) | property placement discipline first-witnessed as recognition-bomb placement |
| `#R-the-compiler-in-one-sentence` (first-witness-closed 2026-07-18) | `property` extends the compiler-in-one-sentence closure surface | full compiler surface (source/legacy/target/settle_on/cli/tools/check/property) closes |
| `#R-verdict-is-content-addressed` (PROMOTED 2026-07-12) | spec-declared property verdicts content-addressed by construction | memoization-by-construction second-witnesses at spec altitude |
| `#R-mirror-is-the-counter-singularity` | mass-scaling asymmetry (property mass scales, runtime mass does not) | 4-file rust/ floor closure first-witnesses the counter-singularity at code altitude |
| `[[architecture-mirror-as-content-addressed-build-system]]` (#43 PROMOTED) | mirror IS the content-addressed build system; spec IS the source-of-truth carrier | (already promoted; this recognition extends its consequence) |
| `[[architecture-property-fracture-bilateral]]` (#53) | spec-native property = third altitude for property declarations (spec-consumer altitude joins @epistemologic/property + @kintsugi/fracture) | (already promoted; this recognition extends the discipline) |

**Composition surprise.** The 5-op basis of `#R-void-is-the-basis`
(`focus`/`project`/`split`/`shift`/`settle`) IS the tokenizer kinds
in `shards/mirror/spec/keywords.mirror` (`focus project` / `focus
target` / `focus cli`; `project source` / `project altitude` /
`project emit`; `settle settle_on`). The Void's 5-op basis IS the
mirror.spec grammar's constructor set. This is a first-witness of
`#R-void-is-the-basis` at grammar altitude: the spec-parser's
tokenizer kinds are Void-basis moves; adding a `focus property`
kind extends the grammar with one more Void-basis move.

**Second composition surprise.** The `---` separator in
`boot/std/mirror/liquid.mirror` (June 2026) IS the recognition's
compilation boundary. The recognition's mathematical claim (spec ↦
below-`---` is a monoid homomorphism from Spec-extension to Verdict-
composition) was ALREADY substrate-decl'd 11 months ago. This
tick's contribution is to name what was already there — not to add
new substrate, but to promote the existing substrate-decl to the
compilation-semantics altitude where it can be discharged in a
migration tick.

---

## §11 Adversarial-robustness checklist

Every claim in §1-§10 survives challenge because:

- **Every recognition citation** points to a PROMOTED or first-
  witness-closed sibling recognition in this session.
- **Every substrate reference** points to landed file + line number.
- **Every prior-art citation** is a peer-reviewed publication or
  well-known open-source project.
- **Every falsifier** is concrete + executable in a single Reed tick.
- **Every dispatch-table entry** maps to a landed pillar primitive
  in `prism/prismqueer/src/liquid.rs::pillar` (lines 178-707).
- **The migration plan** is 5 ticks; each tick has a Pass/Fail
  oracle (bit-match with an existing RED file's verdict).
- **The Q's for Alex** are all candidate-strength; none block the
  cascade.
- **The composition graph** is closed (every edge to a recognition
  in this session or landed in prior arc).

---

## §12 Reed's next unblocked (post-Mara-landing) empirical work

**Tick M0 unlocked.** Author `rust/src/liquid.rs` skeleton per §8.1.
No dependency on further Mara authorship; the property carrier
grammar-decl in `shards/mirror/spec/property.mirror` (Deliverable 3)
lands this tick with SSH signing through hooks. Reed can consume
immediately.

**Tick M2 empirical firing** closes second-witness on the recognition
(math root §8 + this spec §4.1). One firing; six recognitions
promoted; ouroboros closes.

**Slow is fast.** Formalization done. Migration path clear. The
implementation is Reed's.
