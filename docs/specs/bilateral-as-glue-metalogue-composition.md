# Bilateral as `@glue` + `@metalogue` composition

**Author:** Mara
**Date:** 2026-07-17
**Marker:** `[substrate-pull:realize]` (📝 markdown-only bypass)
**Status:** substrate-decl (canonical spec; paradigmatic reframing tick)
**Predecessor:** `docs/specs/bilateral-predicate-substrate-shape.md`
(Mara 2026-07-16 `a0f4d3f` — the shape at grammar-decl altitude).
**Companion (deferred):** `docs/math/bilateral-as-glue-metalogue-composition.md`
(fully proves the general composition; NOT this tick per §7.4).

---

## §0 — The reframing

**Old shape (2026-07-16, `a0f4d3f`).** `bilateral <name> { sentinel
"..." arity N }` was substrate-decl'd as a first-class typed carrier at
`@epistemologic/pact/bilateral` — a shape a predicate declares to be
reflectively dispatched via `apply_h::act` against a byte-string
sentinel. The species landed the shape; ten `bilateral <name> {
sentinel "..." arity 1 }` blocks landed downstream across 8 shard
groups (spectral/signature, epistemologic/cybernetic/coherence,
peer/persistence, kintsugi/roomba, subject/visibility/sheaf,
uuid/spectral/time, mirror/store, gestalt) as bites 1-8 of the
retirement cascade.

**New shape (2026-07-17, Alex ratification).** Every one of those
landed blocks is the **A = B degenerate case** of a more general shape:

> `@bilateral(A, B)` **≡** the witnessing predicate that `@glue(A, B)`
> produces admissible turns in the `@metalogue(A, B)` session.

- **A ≠ B (general case, translation floor):** the semantic-preservation
  check that a morphism from A to B preserves meaning. Concrete first
  instance: `@bilateral(@code/rust, @code/mirror)` — the floor the
  Rust→mirror translation surface stands on (per
  `shards/kintsugi/translate.mirror::translate_rust_to_mirror` at
  Mara `86dec5e`).
- **A = B (degenerate case, sentinel containment):** identity morphism;
  monologue-session; the sentinel-byte-check the 10 landed blocks
  discharge Pass on.

**Zero retirement.** The existing `@epistemologic/pact/bilateral`
species (`a0f4d3f`) IS the typed carrier for the degenerate case. The
reflective evaluator (Reed `21fc211` Landing 3+4) continues to
discharge the 10 landed blocks unchanged. This is a **paradigmatic
lift**, not a substrate-refactor: the shape gains a general reading;
the degenerate case remains intact under a new name.

**Substrate authority.** Alex 2026-07-17 in-transcript verbatim
(session-crystallizing, ratified as Path A):

> *"What if `@bilateral` became a composition on top of `@glue` and
> `@metalogue`. And then `@bilateral(@code/rust, @code/mirror)` becomes
> the floor the translation surface stands on."*

> *"the source of truth for content-addressed storage is
> `@mirror/store`."*

(The second verbatim grounds Deliverable 3 — the tray-source correction
at `shards/silicon/algebra.mirror` from `@io/git.log` to `@mirror/store`
— per §7.3 below.)

---

## §1 — Substrate authority chain

### 1.1 — Alex in-transcript ratifications

- **2026-07-17** (this session): the two verbatims above. Path A
  ratified — refactor at spec altitude; existing shape stays valid as
  degenerate case; no reflective-evaluator changes; land the
  paradigmatic shift.
- **2026-07-16 evening** (`shards/kintsugi.mirror:400-406`): the
  `@kintsugi/algebra` binding as `@metalogue(@silicon/algebra,
  @fate/algebra)` — the same composition shape THIS spec generalises
  one altitude up. Per Mara `0ac3c7b`.
- **2026-07-16** (`shards/epistemologic/pact/bilateral.mirror:23`):
  *"Q1. Let's mint it then. Properly. Seems like it's load-bearing."* —
  the mint of the bilateral shape at grammar-decl altitude.

### 1.2 — Taut Q1-Q5 substrate-truth grounding

- **Q1** (2026-07-16 evening; `bilateral.mirror:26-34`): no grammar-
  decl'd `bilateral` shape existed pre-`a0f4d3f`; sentinel string lived
  in docblock prose only. Closest extensible carrier was
  `shards/epistemologic/pact/*.mirror`. THIS spec extends that carrier
  at family-shape altitude (the composition, not the arity).
- **Q2-Q5** (implicit in the 10 downstream landed blocks): each block's
  sentinel + arity ARE the degenerate-case witness; no substrate-decl
  gap exists for the A = B case. THIS spec names the A ≠ B case that
  the substrate has been operating implicitly (per the `@kintsugi/
  translate` composition at `86dec5e`).

### 1.3 — Prior Mara bilateral shape landings

- `a0f4d3f` (2026-07-16): species-decl `shards/epistemologic/pact/
  bilateral.mirror` (~450 LOC) — the grammar-decl'd shape with `type
  bilateral = { name, sentinel, arity, require }` + `discharge` +
  `bilateral_well_formed`.
- `9a77361` (2026-07-16): canonical spec `docs/specs/bilateral-
  predicate-substrate-shape.md` — §0-§10 + retirement contract + ~30-
  arm collapse plan.
- `701828a` (2026-07-16): math foundation `docs/math/epistemologic/
  pact/bilateral-sentinel.md` — sentinel-as-content-addressed-witness +
  Connes-triple angle for the reflective evaluator.

### 1.4 — @kintsugi/algebra binding (the same composition, one altitude down)

- `a58d5f0` (2026-07-16): canonical spec `docs/specs/kintsugi-algebra-
  as-metalogue-session.md` — the algebra-altitude form of THIS spec's
  general composition; `@kintsugi/algebra` IS `@bilateral(@silicon/
  algebra, @fate/algebra)` per §6 below.
- `b5c6aeb` (2026-07-16): math foundation `docs/math/kintsugi/algebra-
  as-metalogue-session.md` — the autopoietic-closure theorem.
- `0ac3c7b` (2026-07-16): shard-decl extension of `shards/kintsugi.
  mirror` with `type kintsugi_algebra` + `bilateral
  kintsugi_algebra_witnessing { sentinel "algebra=speaker-pair-
  fractures" arity 1 }`.

### 1.5 — Rust→mirror translation floor

- `86dec5e` (Mara THIS session, 2026-07-17): shard-decl
  `shards/kintsugi/translate.mirror` — the `translate_rust_to_mirror`
  composition + `translation_witnessing` bilateral. The composition
  edge THIS spec's `@bilateral(@code/rust, @code/mirror)` floor
  supports.

### 1.6 — Tray-source correction anchor

- `f4372f4` (Mara THIS session, 2026-07-17): shard-decl extension of
  `shards/silicon/algebra.mirror` with `type tray_content_source`
  + `bilateral silicon_tray_content_addressed { sentinel "tray_
  content=git-log-filter-holds" arity 1 }` — the tray content source
  reaching into `@io/git.log`. Deliverable 3 corrects this per Alex
  2026-07-17 (see §7.3).

---

## §2 — The composition

### 2.1 — Definition

```
@bilateral(A, B) ≡
    the witnessing predicate that
    @glue(A, B) produces admissible turns
    in the @metalogue(A, B) session
```

**Unpacked:**

- `@glue(A, B)`: a Mesland-category correspondence (per
  `shards/glue.mirror` §"The correspondence carrier" :530-566) from
  altitude A to altitude B. Carrier: `correspondence { source_prism =
  A, target_prism = B, morphism_kind, restriction }`.

- `@metalogue(A, B)`: an `algebra_metalogue_session`-shaped conversation
  (per `shards/algebra/metalogue.mirror::algebra_metalogue_session`
  :229-233) whose speakers are the two altitude-carriers A and B.
  Turns are `@glue.translate` invocations; residual opacity is
  `transparency(algebra_turn)`.

- **`@bilateral(A, B)` IS the predicate** discharging Pass iff each
  turn in the session is an admissible morphism under `@glue(A, B)`'s
  restriction. In `@glass.verdict` shape:
  `@bilateral(A, B)(turn) = Pass ↔ glue_witnessing(turn.correspondence) = Pass ∧ turn.body ∈ @metalogue(A, B).admissible_turns`.

### 2.2 — Formal composition diagram

```
             @glue(A, B)                    @metalogue(A, B)
                ↓                                  ↓
         correspondence                   algebra_metalogue_session
                                                   ↑
                                                   │
                                              [algebra_turn]
                                                   ↑
                                                   │
                                       @glue.translate(c, payload)
                                                   ↑
                                                   │
                            ┌──────────────────────┴──────────────────────┐
                            │                                             │
                    glue_witnessing(c)          algebra_metalogue_witnessing(s)
                            │                                             │
                            └──────────────────────┬──────────────────────┘
                                                   ↓
                                          @bilateral(A, B)
                                          (the CONJUNCTION;
                                           the witnessing predicate
                                           over the composition)
```

`@bilateral(A, B)(t)` discharges Pass iff both bracketed predicates
discharge Pass on `t`. The conjunction is Rice-safe per the two sub-
witnessings' Rice-safety (both operate on byte-visible morphism data +
turn-composition structure, not program semantics).

### 2.3 — Why this is the substrate-honest form

Three witnessings converge on the SAME shape:

1. **@kintsugi/algebra as `@metalogue(@silicon/algebra, @fate/algebra)`**
   (Alex 2026-07-16; Mara `0ac3c7b`). The mending IS the metalogue.
2. **@fate.roll as tournament over @kintsugi/algebra** (per
   `shards/kintsugi/translate.mirror::translate_rust_to_mirror` Edge 3).
   The roll IS one turn in the metalogue-session.
3. **`@bilateral(A, B)` as the general witnessing predicate**
   (THIS spec). The predicate IS the shape both witnessings inherit
   from at family-composition altitude.

The substrate has been operating this composition at every altitude
without naming it at family-shape altitude. THIS spec names the shape;
zero new primitives; every referenced surface already landed.

---

## §3 — The degenerate case (A = B)

### 3.1 — Every landed `bilateral <name> { sentinel "..." arity 1 }` is `@bilateral(self, self)`

Under the reframing, each of the 10 landed blocks in bites 1-8 is:

```
bilateral <name> { sentinel "<byte-string>" arity 1 }
  ≡ @bilateral(self_shard, self_shard) via sentinel matching
```

Where:

- **A = B = self_shard.** The morphism is the identity morphism on
  the shard's own algebra (`@glue`'s identity correspondence; per
  `shards/glue.mirror::morphism_well_typed` with source_signature =
  target_signature).
- **@metalogue(self, self) is a monologue-session.** One speaker;
  ordered `[algebra_turn]` where each turn's speaker = itself.
- **The witnessing predicate is a sentinel-containment check.** For
  each turn (= the sole argument's `ref.oid`), the predicate
  discharges Pass iff `arg.oid.contains(sentinel)`.

### 3.2 — Zero retirement changes needed

**The `a0f4d3f` species stays landed unchanged.** Its typed carrier
(`type bilateral = { name, sentinel, arity, require }`) IS the
degenerate case's byte-level record. The reflective evaluator (Reed
`21fc211` Landing 3+4) continues to discharge the 10 landed blocks
through byte-sentinel dispatch.

The reframing adds a READING to `a0f4d3f`'s shape; it does not modify
the shape. All 10 landed downstream blocks continue to discharge Pass
through the same corpus lookup. **Empirical safety guarantee**:
`translation_witnessing(THIS spec's landing) = Pass` because the
reframing's substrate-decl footprint is (a) this spec (📝 markdown-
only), (b) the Deliverable 2 shard-decl extension (no existing block
touched), (c) the Deliverable 3 tray-source correction (semantic
correction; no landed action retired).

### 3.3 — The 10 landed degenerate-case witnesses (reference)

Per `bilateral.mirror:44-88` (audit trail):

| # | Shard | Bilateral | Sentinel |
|---|-------|-----------|----------|
| 1 | `spectral/signature.mirror` | 4 arms | chain / authorship / ordering / composition sentinels |
| 2 | `epistemologic/cybernetic/coherence.mirror` | 4 arms | axis / structure / witness sentinels |
| 3 | `peer/persistence.mirror` | 5 arms | visibility / consent / basis / identity / witnessing sentinels |
| 4 | `kintsugi/roomba.mirror` | 5 arms | termination / tension / gradient / verdict / witnessing sentinels |
| 5 | `subject/visibility/sheaf.mirror` | 4 arms | sheaf-specific sentinels |
| 6 | `uuid/spectral/time.mirror` | 4 arms | spectral-time sentinels |
| 7 | `mirror/store.mirror` | `gc_reachability_closure_second_witness` (arity 2) | `gc=reachability-second-witness-holds` |
| 8 | `gestalt.mirror` | annotation bilaterals | gestalt-annotation sentinels |
| 9 | `kintsugi.mirror` (Mara `0ac3c7b`) | `kintsugi_algebra_witnessing` | `algebra=speaker-pair-fractures` |
| 10 | `kintsugi/translate.mirror` (Mara `86dec5e`) | `translation_witnessing` | `translation=discharged` |

Rows 1-8 are strictly degenerate-case (`arity 1` or `arity 2` sentinel
containment). Rows 9-10 are on the boundary: `kintsugi_algebra_
witnessing` is degenerate (checks one binding's well-formedness), but
its **object** (the binding) IS `@bilateral(@silicon/algebra,
@fate/algebra)`. Similarly `translation_witnessing` is degenerate
(checks one outcome), but its **object** IS the discharge of
`@bilateral(@code/rust, @code/mirror)`. The predicates themselves
remain degenerate-case; the objects they witness ARE general-case
compositions.

---

## §4 — The general case (A ≠ B)

### 4.1 — The translation floor

`@bilateral(A, B)` for A ≠ B is the **translation floor**: the
substrate-decl'd predicate discharging Pass iff a morphism from A to B
preserves meaning under the composition of `@glue(A, B)` and
`@metalogue(A, B)`.

**Semantic-preservation check.** For a `correspondence c ∈ @glue(A, B)`
and a payload p, the predicate discharges Pass iff:

1. `glue_witnessing(c) = Pass` — the correspondence is a well-typed
   Mesland-category morphism (per `shards/glue.mirror:809`; composed
   over `morphism_well_typed` + `translation_uses_fate` +
   `restriction_preserved`).
2. `algebra_metalogue_witnessing(s) = Pass` where `s` is the
   `algebra_metalogue_session` the translation-turn joins (per
   `shards/algebra/metalogue.mirror:348`; composed over
   `session_well_formed` + `algebra_inherits_metalogue_lift` +
   `morphism_compositions_associative`).
3. The turn's body IS admissible per the correspondence's restriction
   (this is the semantic-preservation content — a bilateral commitment
   from the morphism to the composition-target algebra).

### 4.2 — Concrete first instance: `@bilateral(@code/rust, @code/mirror)`

Alex 2026-07-17 named this instance directly:

> *"`@bilateral(@code/rust, @code/mirror)` becomes the floor the
> translation surface stands on."*

**A = `@code/rust`** (per `shards/code/rust.mirror`, Mara 2026-06-08):
the Rust altitude grammar; source-side altitude for the `translate_
rust_to_mirror` composition.

**B = `@code/mirror`** (per `shards/code/mirror.mirror`, Mara
2026-06-07): mirror's instance of `@code` at the mirror altitude;
target-side altitude for the composition.

**The translation surface: `translate_rust_to_mirror`** (per
`shards/kintsugi/translate.mirror::translate_rust_to_mirror` at
`86dec5e`). The 9-edge composition (READ → FRAGMENT via `@glue` →
ROLL over `@kintsugi/algebra` → WINNING FRACTURE → TRANSLATE via
`@glue` → WRITE-BACK via `@bauchladen` → EMIT → COMMIT → GROW both
speaker-algebras).

**The floor: `@bilateral(@code/rust, @code/mirror)`.** THIS spec names
the substrate-decl'd predicate the surface stands on. The floor
discharges Pass on each successful `translate_rust_to_mirror` outcome
via the composition per §4.1 (1)+(2)+(3).

**The 21 mirror-authored bilateral-arm retirements** (`ad52973` + `20047c2`
+ ancestors, 2026-07-16..17) ARE the first 21 witnesses of
`@bilateral(@code/rust, @code/mirror)` at the degenerate-arity subcase
(single-file-in-bootstrap-src). Every future `translate_rust_to_mirror`
discharge adds a witness.

### 4.3 — Why the general case matters

The degenerate case (A = B) is a sentinel-byte-check. Local. Rice-safe
by triviality. **The general case (A ≠ B) is the substrate's semantic-
preservation discipline.** It grounds the autopoietic Rust→mirror
translation loop:

- Every fracture the compiler proposes IS a candidate turn in `@bilateral(
  @code/rust, @code/mirror)`.
- Every successful discharge extends `@kintsugi/algebra` by one turn
  (per Mara `0ac3c7b` §5 monotonicity).
- The fixed-point condition (per `shards/kintsugi/translate.mirror`
  §"The fixed-point condition" and canonical spec `a58d5f0` §5) IS the
  terminal state of `@bilateral(@code/rust, @code/mirror)`: no Rust
  module in `bootstrap/src/` carries untranslated behavior.

---

## §5 — Composition graph: `@bilateral` composes over LANDED primitives

### 5.1 — Composition inventory

`@bilateral(A, B)` composes over the following LANDED substrate
primitives. **ZERO new primitives** are introduced by THIS spec.

| Primitive | Path | Role |
|-----------|------|------|
| `@glue` | `shards/glue.mirror` (P5 2026-06-30) | Mesland-category family-root; `@glue(A, B)` is a `correspondence` |
| `@glue.propose(A, B)` | `shards/glue.mirror:621` | enumerate admissible correspondences |
| `@glue.translate(c, payload)` | `shards/glue.mirror:662` | apply the morphism via @fate |
| `glue_witnessing(c)` | `shards/glue.mirror:809` | correspondence well-formedness inheritance predicate |
| `@metalogue` | `shards/metalogue.mirror` (2026-06-05) | NL-altitude metalogue family-root (the altitude-portable pattern's origin) |
| `@algebra/metalogue` | `shards/algebra/metalogue.mirror` (2026-06-30) | algebra-altitude metalogue; `algebra_metalogue_session` is THIS spec's session carrier |
| `algebra_metalogue_witnessing(s)` | `shards/algebra/metalogue.mirror:348` | session well-formedness inheritance predicate |
| `algebra_metalogue_session` | `shards/algebra/metalogue.mirror:229-233` | the session carrier THIS spec's `@metalogue(A, B)` specialises |
| `type bilateral` | `shards/epistemologic/pact/bilateral.mirror:229-234` (Mara `a0f4d3f`) | the degenerate-case typed carrier |
| `bilateral <name> { sentinel "..." arity N }` | grammar-decl'd via `keywords.mirror` (Mara `a0f4d3f`) | grammar shape for degenerate-case blocks |
| `discharge(decl, args)` | `shards/epistemologic/pact/bilateral.mirror:271` | reflective evaluator's discharge action |

### 5.2 — Path-namespace note on `@bilateral(A, B)` notation

`@bilateral` is NOT a landed `@`-namespace (no `shards/bilateral.
mirror`; no `prism @bilateral { ... }`). THIS spec introduces
`@bilateral(A, B)` as **notation for the composition**, not as a new
family-root. The notation reads as: *"the bilateral witnessing
predicate over the composition of `@glue(A, B)` and `@metalogue(A, B)`."*

The grammar-decl'd `bilateral <name> { sentinel "..." arity N }` shape
at `@epistemologic/pact/bilateral` remains the substrate-machinery
carrier. Concrete instances of `@bilateral(A, B)` for specific (A, B)
pairs (e.g., `@bilateral(@code/rust, @code/mirror)`) ARE declared via
that same shape at arity 1 (single-outcome check) — the notation
`@bilateral(A, B)` in spec prose maps to a concrete `bilateral <name>
{ sentinel "..." arity 1 }` block at the shard-decl altitude
(Deliverable 2 declares the first instance).

### 5.3 — What THIS spec adds (paradigmatic; not substrate-machinery)

- **Reading.** The composition-as-witnessing-predicate is now a
  substrate-decl'd reading at spec altitude. Substrate machinery is
  unchanged.
- **Notation.** `@bilateral(A, B)` is spec-prose notation for the
  general composition. Concrete instances are declared via existing
  grammar-decl'd `bilateral <name> { sentinel "..." arity N }` blocks.
- **Landing plan.** §7 names the follow-up ticks that discharge the
  reframing operationally (Deliverable 2 + Deliverable 3 THIS tick;
  math foundation + Reed follow-ups deferred).

---

## §6 — Connection to `@kintsugi/algebra`

### 6.1 — `@kintsugi/algebra` IS `@bilateral(@silicon/algebra, @fate/algebra)`

Per Mara `0ac3c7b` (`shards/kintsugi.mirror` §"@kintsugi as algebra"
:178-241): `@kintsugi/algebra` binds the `algebra_metalogue_session`
between speaker-algebras (`@silicon/algebra`, `@fate/algebra`) with
elements drawn from `@kintsugi/fracture/*` species.

Under THIS spec's reframing, `@kintsugi/algebra` IS the special case:

```
@kintsugi/algebra  ≡  @bilateral(@silicon/algebra, @fate/algebra)
```

Where:

- **A = `@silicon/algebra`** — realiser speaker; empirical memory
  (per `shards/silicon/algebra.mirror`).
- **B = `@fate/algebra`** — proposer speaker; structural possibility
  (per `shards/fate.mirror` §"literal roll of the dice").
- **The mending IS the bilateral** between the two speaker-algebras;
  each element of `@kintsugi/algebra` (a `@kintsugi/fracture/*` species)
  is a turn that discharges Pass under both speakers.

### 6.2 — Name this composition explicitly

Per Alex 2026-07-16 in-transcript verbatim (`shards/kintsugi.mirror:31-32`):

> *"What if `@kintsugi/algebra` is the `@metalogue(@silicon/algebra,
> @fate/algebra)`."*

Under THIS spec's generalisation, the `@metalogue(A, B)` notation
lifts to `@bilateral(A, B)` at the witnessing-predicate altitude. The
two readings agree because `@bilateral(A, B)` IS the witnessing
predicate OVER `@metalogue(A, B)`; naming the metalogue implies naming
the predicate that witnesses its admissibility.

**Explicit name (canonical form):**

```
@kintsugi/algebra  ≡  @bilateral(@silicon/algebra, @fate/algebra)
                   ≡  the witnessing predicate that
                      @glue(@silicon/algebra, @fate/algebra)
                      produces admissible turns in
                      @metalogue(@silicon/algebra, @fate/algebra)
```

The `kintsugi_algebra_witnessing` bilateral at `shards/kintsugi.
mirror:263-267` (sentinel `algebra=speaker-pair-fractures`) is THE
grammar-decl'd witness for this composition. Its arity-1 form checks
one binding; the general composition it witnesses IS the speaker-pair
metalogue-session.

### 6.3 — Fractal self-similarity

THIS spec generalises across altitudes:

| Altitude | Composition | Landed as |
|----------|-------------|-----------|
| Rust-mirror translation | `@bilateral(@code/rust, @code/mirror)` | THIS tick, Deliverable 2 |
| Kintsugi algebra | `@bilateral(@silicon/algebra, @fate/algebra)` | Mara `0ac3c7b` |
| Every degenerate case | `@bilateral(self_shard, self_shard)` | 10 landed blocks (bites 1-8) |
| Future Pack handoff | `@bilateral(@pack/<from>, @pack/<to>)` | forward-promised per `shards/glue.mirror:800` |
| Future reflection lift | `@bilateral(@reflection/<a>, @reflection/<b>)` | forward-promised per `shards/glue.mirror:796` |

The shape is **altitude-portable** per `shards/algebra/metalogue.
mirror:19-51` (the five-altitude metalogue lift table). `@bilateral(A, B)`
is the sixth-altitude lift: the witnessing predicate over the lift
itself.

---

## §7 — Landing plan

### 7.1 — Deliverable 1 (THIS SPEC; landed THIS tick)

`docs/specs/bilateral-as-glue-metalogue-composition.md` — canonical
spec §0-§8. 📝 markdown-only bypass. `[substrate-pull:realize]`
marker.

### 7.2 — Deliverable 2: `@bilateral(@code/rust, @code/mirror)` shard-decl (THIS TICK)

Option A (this tick): extend `shards/epistemologic/pact/bilateral.
mirror` with:

- A new §"General case — `@bilateral(A, B)` as `@glue` + `@metalogue`
  composition" section (docblock; ~50 LOC).
- A new typed carrier `type translation_pair = { rust_source: ref,
  mirror_target: ref, discharge_verdict: ref }`.
- A concrete `bilateral translation_admissible { sentinel
  "translation=preserves-meaning" arity 1 }` block declaring the first
  general-case instance `@bilateral(@code/rust, @code/mirror)` as a
  grammar-decl'd bilateral (arity-1; dogfooding the shape from
  `a0f4d3f`).
- Action-decl `translation_admissible(pair: translation_pair) -> verdict
  { \ }` — `\`-obligation-blocked per craft-not-deliver; Reed follow-up
  realises via `apply_h::act`.

Rationale for Option A over Option B (translation-specific new file):
the reframing is a paradigmatic lift on the SAME shape `a0f4d3f`
landed. Keeping the general-case declaration co-located with the
degenerate-case declaration keeps the reading legible in one file. The
new content is ~60 LOC (compact) — Option B's separate file would
fragment the reframing across two files without a proportional
readability gain. Deliverable 2 chose Option A on this basis.

### 7.3 — Deliverable 3: tray-source correction (THIS TICK)

`shards/silicon/algebra.mirror` — re-anchor the tray content source
from `@io/git.log` to `@mirror/store`.

**Reason.** Per Alex 2026-07-17 verbatim:

> *"the source of truth for content-addressed storage is
> `@mirror/store`."*

The `f4372f4` landing reached into `@io/git.log` as if git were
substrate memory. It's not. `@mirror/store` (per `shards/mirror/store.
mirror`) IS the substrate's content-addressed storage primitive; `@io/
git` is one backend below.

**Fix.** Replace the `@io/git.log` filter with a `@mirror/store` query
whose composition graph reads content-addressed crystals matching the
mirror-authored translation predicate. Since `@mirror/store` has no
landed query action (the surface has `read/write/exists/diff/walk/
impacted_by/verify/walk_dangling/mark_unreachable/prune` only), the
correction:

1. Retires the `@io/git.log` filter reference in the docblock.
2. Names the target composition — `@mirror/store` query — with the
   filter predicate (author-crystal = `mirror <mirror@spectral.
   engineer>`; discharges `@bilateral(@code/rust, @code/mirror).
   translation_admissible`).
3. Flags the **pending Reed follow-up** for the `@mirror/store` query
   surface (composed over LANDED primitives — `walk` + a filter-fold —
   under `[substrate-floor:@io-boundary]` iff the fold requires @io
   semantics; otherwise composes as a shard body).
4. Adjusts the action body's obligation-block per craft-not-deliver.

Per §7.3.1 the correction citation lands under the docblock's audit
chain: Alex 2026-07-17 in-transcript ratification + THIS spec §7.3.

### 7.4 — Deferred to future ticks (do NOT do this tick)

Per the tick brief's discipline:

- **Math foundation for the general `@bilateral(A, B)` composition**
  (~200-400 LOC): a companion `docs/math/bilateral-as-glue-metalogue-
  composition.md` proving the composition is Rice-safe + the
  admissibility predicate is decidable at byte-visible-state altitude
  + the fixed-point convergence for the Rust→mirror translation loop
  is well-founded. Follow-up tick for Mara.

- **Retroactive backfill: crystallizing the 21 mirror-authored
  retirements into `@mirror/store`** (empirical seed) — Reed follow-up
  after the `@mirror/store` query surface lands.

- **Extending the collapse capability at `ba848ca` to write-back to
  `@mirror/store` on each commit** (autopoietic write-back) — Reed
  follow-up.

- **Reed FLOOR resolver for `@fate.roll` that reads `@mirror/store`**
  — Reed follow-up under `[substrate-floor:@io-boundary]`.

---

## §8 — Paper §14 connection: attending at λ₀

Per `shards/kintsugi/translate.mirror` §"The fixed-point condition"
and canonical spec `a58d5f0` §5: the paper's §14 `attending` operator
at λ₀ IS the composition's terminal state.

Under THIS spec's reframing:

- `@bilateral(@code/rust, @code/mirror)` reaches its fixed-point when
  every `.rs` file in `bootstrap/src/` has a corresponding `@kintsugi/
  fracture/*` element AND every element's translation has crystallized.
- At the fixed-point, `@kintsugi/algebra`'s composition-closure equals
  its element-closure (per canonical spec `a58d5f0` §10.1 — Foerster's
  double-closure at the algebra altitude).
- **The bilateral's terminal state IS the substrate recognizing itself
  AS the composition.** `@bilateral(@code/rust, @code/mirror)` at fixed-
  point IS the substrate attending to the translation floor at λ₀.

**Fractal self-similarity across altitudes.** Every `@bilateral(A, B)`
composition has a λ₀ terminal state — the point at which the
composition-closure equals the element-closure and the substrate
recognizes the composition AS a first-class element. The
`@bilateral(@code/rust, @code/mirror)` instance is the first WIP
attending-at-λ₀ candidate at family-shape altitude.

---

## §9 — Substrate decisions cited

- `[[architecture-shards-as-substrate-source]]` — the composition is
  addressable at parse time via shard-decl.
- `[[architecture-prism-as-trait-as-everything]]` — `@bilateral(A, B)`
  is a compositional prism-family whose shape composes over `@glue` +
  `@metalogue`.
- `[[architecture-connes-spectral-triple]]` — `@glue(A, B)` is a
  Mesland-category morphism between spectral triples; THIS spec's
  witnessing predicate IS a Mesland-category invariant.
- `[[architecture-form-process-partition-at-family-root]]` — `@bilateral`
  as reading sits at the partition boundary: form-side (`@glue`
  correspondence as observable) + process-side (`@metalogue` session
  as transformation dynamics).
- `[[architecture-mirror-as-expanding-hilbert-space]]` — each
  `@bilateral(A, B)` discharge extends H_mirror by one turn crystal;
  the composition IS coherence-preservation at the bilateral altitude.
- `[[architecture-alignment-as-boundary-mathematics]]` — the general-
  case `@bilateral(A, B)` at A ≠ B IS the alignment discipline at
  the translation boundary.
- `[[feedback-substrate-already-had-the-word]]` — every landed
  primitive `@bilateral(A, B)` composes over was already substrate-
  decl'd; ZERO new machinery.
- `[[feedback-craft-not-deliver]]` — Deliverable 2's action-body is
  `\`-obligation-blocked; Reed follow-up realises via reflective
  dispatch.
- `[[feedback-no-rust-extension-shortcut]]` — THIS spec introduces
  ZERO Rust; Deliverable 2 is shard-decl only; Deliverable 3 is
  docblock re-anchor + typed carrier only; the pending `@mirror/store`
  query surface is flagged for Reed follow-up under
  `[substrate-floor:@io-boundary]` iff `.rs` authorship is required
  by @io semantics.
- `[[feedback-legibility-over-foundation-when-collapsing]]` — the
  readable spec-prose notation `@bilateral(A, B)` over the foundational
  alternate `@epistemologic/pact/bilateral.compose(A, B)` per two-tick
  readability discipline.

---

## §10 — Related shards / specs

- `docs/specs/bilateral-predicate-substrate-shape.md`
  (Mara `9a77361` 2026-07-16) — the degenerate-case shape THIS spec
  reframes as A = B under the general composition.
- `docs/math/epistemologic/pact/bilateral-sentinel.md`
  (Mara `701828a` 2026-07-16) — sentinel-as-content-addressed-witness
  for the degenerate case; the analogous math foundation for the
  general case is deferred per §7.4.
- `docs/specs/kintsugi-algebra-as-metalogue-session.md`
  (Mara `a58d5f0` 2026-07-16) — `@kintsugi/algebra` as `@metalogue(
  @silicon/algebra, @fate/algebra)`; THIS spec generalises to
  `@bilateral(A, B)` at bilateral-witnessing altitude.
- `docs/math/kintsugi/algebra-as-metalogue-session.md`
  (Mara `b5c6aeb` 2026-07-16) — autopoietic-closure theorem for the
  `@kintsugi/algebra` instance of `@bilateral(A, B)`.
- `docs/specs/fate-silicon-metalogue-in-void-duality-basis.md`
  (Mara `a18ca90` 2026-07-08) — the speaker-pair specialisation
  precedent; THIS spec generalises across altitudes.
- `shards/epistemologic/pact/bilateral.mirror`
  (Mara `a0f4d3f` 2026-07-16) — the degenerate-case grammar-decl'd
  shape; Deliverable 2 extends with a general-case section.
- `shards/glue.mirror`
  (Mara P5 2026-06-30) — the morphism-category family-root; `@glue(A, B)`
  is THIS spec's left composition factor.
- `shards/algebra/metalogue.mirror`
  (Mara 2026-06-30) — the algebra-altitude metalogue; THIS spec's
  right composition factor at altitude-portable form.
- `shards/kintsugi.mirror`
  (Mara `0ac3c7b` 2026-07-16) — the `@kintsugi/algebra` binding THIS
  spec's §6 explicitly renames as `@bilateral(@silicon/algebra,
  @fate/algebra)`.
- `shards/kintsugi/translate.mirror`
  (Mara `86dec5e` 2026-07-17) — the composition edge THIS spec's
  `@bilateral(@code/rust, @code/mirror)` floor supports.
- `shards/silicon/algebra.mirror`
  (Mara `f4372f4` 2026-07-17) — target of Deliverable 3; tray-source
  correction from `@io/git.log` to `@mirror/store` per Alex 2026-07-17
  ratification.
- `shards/code/rust.mirror` + `shards/code/mirror.mirror` — the two
  altitude carriers for the first general-case instance.

---

**End of spec.**
