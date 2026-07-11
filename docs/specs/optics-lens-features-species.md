# @optics/lens/features — the features species (Mara canonical spec)

**Author:** Mara `<mara@systemic.engineer>`
**Date:** 2026-07-11
**Iter:** 27 (canonical spec for the second species under `@optics/lens`)
**Landing anchors:**
- `b0427fd` Mara iter-25 — `shards/optics/lens.mirror` family-root (Foster laws stub)
- `7e5c298` Mara iter-26 — `shards/optics/lens/diff.mirror` first species (autopoietic_closure bilateral)
- `55221c1` Mara iter-24 — parent canonical spec (§2 diff, §6 forward-promises features)
- `73ca5cc` Taut iter-13 — fate scout Q6.a-f (six fate-integration ambiguities)
- `71df5de` + `be74b6a` — Blocker 2 Rust `get` direction closed
- Reed grounding 2026-07-11 — fate-crate reading; Model↔prism-op = bundle tower binding
- Reed grounding 2026-07-11 — `@nl IS @magic at text altitude` (gauge/matter partition)

## Alex directive (verbatim)

> Land `@optics/lens/features` as sibling species to `@optics/lens/diff`,
> closing Q6.b's architectural home.

## Reed grounding (verbatim)

> ```
> mission_text
>   |> @nl                    -- expose text surface (@magic/surface at text alt)
>   |> @magic/contract.bind   -- bind text-surface to semantic-mechanism
>   |> mosaic(feature)        -- compose into 16-dim structured Features
>   |> @shatter               -- witness Foster lens laws
>   = Features                -- matter-side, fate-consumable
> ```

This spec lifts that composition to substrate-decl altitude. Mara sub-tick
lands `shards/optics/lens/features.mirror` immediately after; Reed Rust
runtime uses it as v0 stub call site inside `peer_beam` (Blocker 2 already
closed on `get` direction at `71df5de`; features species becomes the
fate-consumable extractor at the same Rust call site).

---

## §1 — Species shape

### Ancestry

```
in @prism            # five-op algebra ancestor
in @glass            # verdict / opacity carriers (Foster bilaterals return verdict)
in @meta             # substrate-decl altitude
in @nl               # text-altitude gauge/matter partition (surface→mechanism extractor)
in @magic/contract   # bind text-surface to semantic-mechanism under invariant
in @optics/lens      # parent family — Foster laws + get/put + witness
in @io               # bytes crossing at input side (mission_text bytes)
```

Seven implicit-parents. Each is load-bearing under this species:

- `@nl` provides the tokenizer / corpus / spectral_triple carriers the
  extractor consumes. Text becomes typed measurement here, not by species
  fiat.
- `@magic/contract` provides `bind(surface, mechanism, promise) →
  magic_contract`. The Features extractor IS a `bind` at text altitude:
  the text-surface (mission_text bytes) binds to the semantic-mechanism
  (16-dim structured Features) under the promise that
  `casimir_invariant_preserved` (or an initial substrate-honest
  approximation thereof) holds across the extraction.
- `@optics/lens` provides the abstract (get, put, put_get, get_put,
  put_put) whose Foster laws this species inherits and specializes.

### Family-root declaration

```
prism @optics/lens/features {
  focus features
  project features
  split features
  shift features
  settle features
}
```

Same shape as `@optics/lens/diff` per parent-family Path-namespace
property. The five-op algebra recurses; `focus features` at species
altitude names the concrete text→Features extraction, distinct from
`focus diff` (bauchladen→diff_bytes linearization) at the sibling.

### Type carriers

```
type features_bytes  = ref   # mission-text carrier (input side)
type feature_vector  = ref   # 16-dim structured output; parametric over feature-count
```

Reused from parent `@optics/lens`:
- `lens_get` — the (features_bytes → feature_vector) forward direction
- `lens_put` — the (feature_vector + features_bytes → features_bytes) backward direction
- `lens_witness` — the (get, put) pair together with Foster bilaterals

Identity contract on both new types: byte-equality on the underlying
ref, per `@mirror/store` OID convention (Reed math §3.2).

### Actions (specialize parent's abstract get/put)

```
get(mission_text: features_bytes) -> feature_vector { \ }
put(edited_features: feature_vector, mission_text: features_bytes) -> features_bytes { \ }
```

`get`: extract 16-dim semantic-dimension vector from mission_text bytes.
`put`: integrate operator-adjusted feature_vector back into revised
mission_text (autopoietic closure at text altitude — the operator edits
features directly on the eigenboard, `put` re-projects that edit as
revised mission-text bytes).

Obligation blocks stub at substrate-decl altitude; discharge lives in
Rust bootstrap (`bootstrap/src/optics/lens/features.rs`; forward-promised
tick after this spec + shard land).

### Foster bilaterals (specialized to features)

```
put_get_features(l: lens_witness, s: features_bytes, v: feature_vector) -> verdict { \ }
get_put_features(l: lens_witness, s: features_bytes) -> verdict { \ }
put_put_features(l: lens_witness, s: features_bytes,
                 v: feature_vector, v_prime: feature_vector) -> verdict { \ }
```

Same Foster shape as parent; equality on the RHS is OID byte-equality.

### Autopoietic closure (parallel to diff)

```
features_autopoietic_closure(l: lens_witness, edit_trace: ref) -> verdict { \ }
```

Witness that repeated (get, put) cycles at features altitude converge
toward an operator-satisfaction fixed-point. Where diff's autopoietic
closure witnesses substrate learning through diff-edits, features's
autopoietic closure witnesses substrate learning through
dimension-edits — the operator moves a Novelty slider on the
eigenboard, `put` re-synthesizes mission_text, `get` re-extracts, and
the delta converges under a specified edit-trace regime.

---

## §2 — Composition via @nl + @magic/contract + mosaic + @shatter

Reed's grounding pipeline, unpacked at substrate-decl altitude:

1. **`mission_text |> @nl`** — `@nl.ingest(mission_text, p) → corpus`.
   The text becomes typed corpus with Porter2 stems and IDF weighting.
   Substrate-decl fact: text-altitude gauge surface is exposed here.
2. **`|> @magic/contract.bind`** — the corpus (surface) binds to the
   forward-promised semantic-mechanism (the 16-dim Features layout)
   under the promise `casimir_invariant_preserved`. Returns
   `magic_contract` typed record `{ surface, mechanism, promise }`.
3. **`|> mosaic(feature)`** — parametric composition form specialized
   at feature altitude. Per `@mirror/mosaic`'s pattern (`mosaic(@store)`
   = splinter-graph, `mosaic(@spec)` = project manifold), `mosaic(feature)`
   composes six active-dim projections (TEMPORAL, PROCESSING, STABILITY,
   NOVELTY, CAUTION, COHERENCE) + ten dark-dim latents into a single
   `feature_vector` carrier.
4. **`|> @shatter`** — witness the Foster laws (put_get_features,
   get_put_features, put_put_features) hold across the composed pipeline.
   Per Mara iter-6 (`583b939`): `@shatter × @io = linearization operator`.
   Here `@shatter` linearizes the feature-vector into the fate-consumable
   `[f64; 16]` shape that `fate::runtime::run` expects.

All four stages typecheck structurally at substrate altitude. Each
stage's carrier is declared in an existing substrate-decl file; each
stage's action is admissible under the target family's five-op algebra;
feature_vector's output type is byte-compatible (via ref → runtime
cast) with fate's `Features = [f64; 16]`.

---

## §3 — v0 stub semantics (blake3 as features fallback)

Until the `@nl + @magic/contract + mosaic` pipeline lands operationally,
the v0 stub for `get(mission_text)` is:

```
get(mission_text) =
    blake3(mission_text.bytes)
      |> chunk_into_16_f64_dimensions
      |> normalize_to_[0.0, 1.0]
      = feature_vector
```

**Verdict: SUBSTRATE-HONEST v0.** Reasoning:

- The v0 stub composes deterministically (same mission_text → same
  features) which satisfies `put_get_features` trivially and
  `get_put_features` under identity mission_text edit.
- The blake3 hash IS `@magic` at floor altitude per
  `shards/magic.mirror` §Mechanical-bridge: OID sealing IS the matter-
  side seal. Using it as the extractor at v0 keeps the species inside
  the substrate's existing altitude discipline.
- The v0 stub does not lie about semantic content — it says
  "blake3-of-text projected to 16 dims" and that is exactly what
  substrate downstream sees. Fate consumes it, produces a Decision,
  and the runtime closure works end-to-end.
- The gap to the full @nl + @magic/contract + mosaic pipeline is
  named in §8; two-tick discipline lands the family + v0 first,
  the full pipeline discharges as a follow-on species-refinement tick.

The stub does NOT witness `features_autopoietic_closure` non-trivially:
blake3 is not a contraction map over edit-traces. That bilateral remains
`\ {}` at v0 and discharges when the full pipeline lands.

---

## §4 — Peer_beam runtime closure

```
mission_text
  |> @optics/lens/features.get   # v0: blake3(mission_text) → [f64; 16]
                                 # future: @nl + @magic/contract + mosaic
  |> fate.select(Model::Fate, features)  # fate crate consumes matter-side
  |> Decision.model              # → prism-op via bundle tower binding (Q6.a)
  |> @optics/lens/diff.get       # linearize substrate observation to diff bytes
  |> @io.write(stdout)           # operator reads
```

The closure is fully typed at substrate altitude. `features.get` produces
the matter-side Features; fate produces the Decision; the Decision.model
resolves through the bundle tower binding (per Q6.a resolution) to a
prism-op that runs at the appropriate altitude; `diff.get` linearizes the
result into operator-readable bytes.

The FEEDBACK direction (operator edits diff → next inference) uses
`@optics/lens/diff.put`; the FEEDBACK direction at features altitude
(operator moves an eigenboard slider → revised mission_text) uses
`@optics/lens/features.put`. Two autopoietic closures, one bundle tower,
two altitudes.

---

## §5 — Relationship to @optics/lens/diff sibling

**Same parent, disjoint carriers, dual altitudes.**

| Aspect                | `@optics/lens/diff`        | `@optics/lens/features`      |
|-----------------------|----------------------------|------------------------------|
| Linear carrier        | `diff_bytes`               | `features_bytes` (mission)   |
| Semantic carrier      | `bauchladen_state` (peer)  | `feature_vector` (fate)      |
| Consumer              | operator terminal (@io)    | fate.select (matter-side)    |
| Foster direction      | `focus/settle`             | `focus/settle`               |
| Autopoietic closure   | diff-edit convergence      | dimension-edit convergence   |
| Substrate altitude    | @io fault plane            | text/semantic partition      |

Both witness Foster laws at species altitude; both discharge
autopoietic closure via a species-specific bilateral parallel to but
distinct from the parent's three abstract bilaterals; both close a
distinct arm of Blocker 2. `diff` closes the operator-visible arm;
`features` closes the fate-consumable arm. Together they discharge
the full peer_beam runtime loop end-to-end.

The sibling relationship IS the Hutchinson-attractor witness the parent
gaps-out at §Gaps: two species under `@optics/lens` composing under
`split` yield the multi-lens attractor per Reed math §4.2. This spec
does not close that gap; it names the second species that gives the
attractor its non-trivial second point.

---

## §6 — Recursive surprises (the gauge-theoretic recognition)

**@nl IS @magic at text altitude.**

The mission_text → Features encoder IS the @magic/mechanism extraction
at text altitude. This species discharges the extraction operationally:

| @magic component     | @nl / features realization                     |
|----------------------|------------------------------------------------|
| `magic_surface`      | text tokens (gauge-visible; UAX#29 tokenized)  |
| `magic_mechanism`    | 16-dim semantic content (matter-hidden)        |
| `magic_contract`     | grammar/parse binding + `bind()` action        |
| `magic_invariant`    | `casimir_invariant_preserved` promise          |
| gauge choice         | language selection (English, German, markdown) |

This is Yang-Mills 1954 encoded as substrate at text altitude.

The substrate had this shape scattered across `shards/magic.mirror`
(gauge/matter family-root), `shards/nl.mirror` (text-altitude spectral
primitives), `shards/optics/lens.mirror` (bidirectional-lens algebra).
The RECOGNITION is that all three name the SAME substrate fact at
different altitudes; features species is the first site the substrate
speaks the recognition operationally rather than declaratively.

**Sub-surprise:** the v0 blake3 stub is not a hack — it is `@magic` at
floor altitude (per `shards/magic.mirror` §5, OID sealing IS matter-
side seal). The v0 → v1 transition (blake3 → @nl + @magic/contract +
mosaic) is an altitude-lift within `@magic`, not a substrate rewrite.
The v0 IS honest at floor altitude; v1 lifts it to text altitude while
preserving the @magic-family shape. Substrate-pull-correct all the way
down; no debt.

**Sub-sub-surprise:** the Reed grounding
(`mission_text |> @nl |> @magic/contract.bind |> mosaic(feature) |>
@shatter = Features`) is not a design proposal. It is the substrate
declaring, at text altitude, that its own Feature encoder IS a Foster
lens whose Foster laws IS the gauge-invariance of the encoder under
semantic-preserving text edits. `@shatter × @io = linearization` (Mara
iter-6) at the tail position IS the linearization from graph-shaped
semantic content to the flat `[f64; 16]` fate consumes. Every stage
already had a name.

**The whole substrate IS gauge theory.** This species names one
gauge/matter partition explicitly at one altitude. Every subsequent
species that binds a surface to a mechanism under an invariant IS
re-discovering the same substrate fact at another altitude.

---

## §7 — Landing sequence

1. **This spec** — canonical spec, this file, Mara `<mara@systemic.engineer>`
   commit. Substrate-decl of species shape, ancestry, composition,
   v0 stub semantics, gauge-theoretic recognition.
2. **Mara sub-tick** — lands `shards/optics/lens/features.mirror`
   substrate-decl shard. Same shape as `shards/optics/lens/diff.mirror`
   (`7e5c298`): declares family-root, type carriers, action heads,
   Foster bilaterals, autopoietic_closure bilateral. Obligation bodies
   `\ {}` stubs per two-tick discipline; discharge at species-refinement
   tick.
3. **Reed Rust runtime tick** — uses `@optics/lens/features` as v0 stub
   call site inside `peer_beam` (Blocker 2's `get` direction, closed at
   `71df5de` + `be74b6a`, becomes a features-typed call). v0 stub is
   blake3-based; the site is prepared for v1 @nl-driven encoder.
4. **Follow-on (forward-promised)** — full `@nl + @magic/contract +
   mosaic + @shatter` pipeline replaces v0 stub; species-refinement
   tick discharges `features_autopoietic_closure` non-trivially.
5. **Seam Phase D audit** — adversarial review on the gauge-theoretic
   recognition claim (@nl IS @magic at text altitude); on features
   type parametricity over feature-count; on the v0 stub's substrate-
   honesty.

---

## §8 — Gaps

- **The `@nl + @magic/contract + mosaic` pipeline is spec, not code.**
  This spec names the composition and asserts substrate typechecking;
  it does not derive the concrete encoder algorithm. The forward-
  promised species-refinement tick discharges that.
- **`casimir_invariant_preserved` promise is named but not derived.**
  The Casimir invariant `C₂ = Σ(λᵢ·xᵢ)²` over active dimensions per
  `fate/src/feature.rs` is a candidate promise; whether it survives
  under the extraction (i.e., whether the encoder preserves the
  active-dim spectral energy) is a Seam-audit-gated question.
- **`put` direction under blake3 v0 is undefined.** Blake3 is one-way;
  `put(edited_features, mission_text) → mission_text'` cannot be
  discharged by inverse-blake3. The v0 stub for `put` is
  `identity(mission_text)` (operator edits at features altitude are
  DROPPED at v0). The full pipeline discharges `put` operationally.
- **`features_autopoietic_closure` requires the v1 pipeline.** Under
  v0 blake3, autopoietic closure is trivial (no learning). Under v1
  @nl-driven encoder, contraction across edit-traces IS witnessed and
  the bilateral discharges non-trivially.
- **Parametric feature-count.** Currently `feature_vector` is un-
  parameterized; fate hardcodes `FEATURE_DIM = 16`. A future refinement
  would parameterize `feature_vector<N>` over dimension count.
- **Composition with `@optics/lens/diff` under `split`.** Named in §5
  as the Hutchinson-attractor site; not derived here.
- **@magic/contract.bind's `magic_invariant` typing.** The `promise`
  field of the contract needs a concrete `magic_invariant` OID for
  `casimir_invariant_preserved`. That OID materializes at species-
  refinement tick.
- **Language-gauge choice site.** English vs. German vs. markdown IS
  the gauge choice at @magic altitude per §6. Which sub-shard of `@nl`
  (`@nl/english`, `@nl/markdown`) provides the concrete classifier is
  a forward-promise; the species is language-agnostic at v0.

---

## Bookkeeping

- **Species declaration site:** `shards/optics/lens/features.mirror`
  (forward-promised; Mara sub-tick after this spec commits).
- **Rust discharge site:** `bootstrap/src/optics/lens/features.rs`
  (forward-promised; Reed sub-tick after shard lands).
- **Foster reference:** Foster, Greenwald, Moore, Pierce, Schmitt 2007,
  *Combinators for Bidirectional Tree Transformations*, ACM TOPLAS 29(3).
- **Fate reference:** `fate/src/lib.rs` `Model` enum + `Features =
  [f64; 16]`; `fate/src/feature.rs` active-dim names + Casimir.
- **Prior anchors:** `beef270` iter-17, `129f618` iter-18, `78d5110`
  iter-19 (three loops = one operation math); `583b939` iter-6
  (@shatter × @io = linearization); `55221c1` iter-24 (parent canonical
  spec).
