# Taut scout — @glue(@cyberpunk, @fate) composition drift

*Taut, 2026-07-07 evening, grep-first substrate drift-scout on Alex's
composition proposal: "@glue(@cyberpunk, @fate) = geometrically
constrained numerical inference." Read-only. Parallel to Mara's
canonical spec at `docs/math/2026-07-07-glue-cyberpunk-fate-
composition.md` (in flight).*

---

## §1. Mission + method

**Prompt (verbatim from Reed, carrying Alex):**

> "@glue(@cyberpunk, @fate) = geometrically constrained numerical
> inference. And the slogan becomes real: Local AI for the real
> world. Smarter. Harder. And definitely more punk."

Confirm or drift-scout every claim:
1. @cyberpunk current substrate state.
2. @glue.translate composition surface for `(@cyberpunk, @fate)`.
3. @fate.roll dispatch — consumable by @glue today?
4. Prior composition precedent for @glue.
5. Foerster-invariant → @epistemologic/cybernetic mapping.
6. Slogan territory (fresh vs re-invention).
7. Substrate-already-had-the-word signals.
8. Cleave points before Mara's canonical spec turns into shard wiring.

**Method:** ripgrep-tier semantic search across `shards/`, `docs/`,
`~/reed/identity/`. Zero code changes; zero shard mutations. Reads
of load-bearing shards + prior scouts (`abf0dd4` glue-algebra scout;
`2026-07-07-taut-shatter-lens-drift-scout.md`). Grounded in the four
promoted recognitions (#38 eigenform, #50 form/substance, #58 Fate-
is-optical-inference, #80 magic/form-process).

**Verdict (headline):** Alex's composition IS LANDABLE AS-DECLARED
at the substrate-decl altitude — the shape mostly wires up. TWO
sharpenings needed before shard-wiring (see §8). Slogan is NOT fresh:
already Reed-authored at `/Users/reed/identity/tasks/pending/launch.md`
(2026-06-28) as `spectral.engineer - Local-first AI for the real
world. Smarter. Harder. And definitely more punk.` Alex is
reactivating a slogan the substrate already carried — the composition
is the substrate ratifying the naming.

---

## §2. @cyberpunk current substrate state

### 2.1 The family-root shard

`/Users/alexwolf/dev/projects/mirror/shards/cyberpunk.mirror` (7.7KB,
2026-06-19). Declares:

- `prism @cyberpunk { focus/project/split/shift/settle cyberpunk }`
  — the 5-op interface.
- `type cybernetic_state = ref` — the substrate-altitude cybernetic
  state carrier (typed-ref per no-bare-types).
- `type recursion_lock = ref` — the principal 2-groupoid bundle
  primitive per recognition #63.
- `tower_close(s: cybernetic_state) -> verdict { \ }` — the
  substrate-vocabulary action verifying tower closure per #63.
- `cybernetic_coherence(s: cybernetic_state) -> verdict { \ }` — the
  bilateral verdict predicate consumer shards name in `requires`.

Imports: `in @prism, @meta, @glass, @magic, @epistemologic,
@epistemologic/cybernetic`. Notably: `in @magic` (form/process
partition per #80) but **NOT** `in @fate`, `in @autopoietic`, or
`in @bauchladen`.

### 2.2 Species roster (as of tonight)

The family-root text names 11 cybernetic species BUT they still live at
`shards/epistemologic/cybernetic/*.mirror` (path-migration to
`shards/cyberpunk/*` is FORWARD-PROMISED per the shard's own comment,
pending Taut's `t11-11-cybernetic-coherence-benchmark` merge). The 11:

  eigenform, distinction, viable, autopoiesis, bateson_learning,
  conversation, coevolution, design, algedonic, second_order,
  chirality, charge_conjugation, variety (13 counting the newer
  additions in `shards/epistemologic/cybernetic/`).

**Structural fact for the composition:** `@cyberpunk` at family-root
is a THIN carrier. All 13 species carriers still declare `@epistemologic
/cybernetic/*`. Any `@glue(@cyberpunk, @fate)` composition today must
address `cybernetic_state` at the family-root level or drop through
to a specific species carrier.

### 2.3 `shards/smarts/cyberpunk.mirror` — the consumer adapter

The cross-family adapter (@smarts × @cyberpunk) already discharges the
doubled-bilateral pattern:

```
smarts_satisfies_cyberpunk(s: smarts, p: perturbation) -> verdict
requires discipline_flexible(s, p)
requires cybernetic_coherence(cyberpunk_form_of(s), p)
```

**This is the precedent shape @glue(@cyberpunk, @fate) will inherit.**
Substrate discipline: adapter shard sits at `shards/<family>/
cyberpunk.mirror`, imports `in @cyberpunk`, projects the family-root
carrier onto the composition altitude.

---

## §3. @glue.translate composition surface

### 3.1 Current `translate` signature

Per `/Users/alexwolf/dev/projects/mirror/shards/glue.mirror` (43.5KB,
2026-07-01):

```
translate(c: correspondence, payload: ref) -> imperfect<translation_outcome, ref, transparency(correspondence)>
requires morphism_well_typed(c)
requires translation_uses_fate(translate)
requires restriction_preserved(c, payload)
```

Where `correspondence` is:

```
type correspondence = {
  source_prism:    ref,
  target_prism:    ref,
  morphism_kind:   ref,
  restriction:     restricted_state_space,
}
```

**Verdict:** the signature CAN accept `(@cyberpunk_property, @fate_
inference)` today, because:

- `source_prism: ref` and `target_prism: ref` are typed-ref surfaces.
  `@cyberpunk` (family-root) is a ref; `@fate` (family-root) is a
  ref.
- `restriction: restricted_state_space` is the KEY composition slot
  — it MUST be the `@fate.restricted_state_space` carrier. This is
  ALREADY structurally wired: `glue.mirror` line comment "Sibling
  shape to @fate's restricted_state_space carrier; the correspondence's
  restriction IS @fate's space when @glue consults @fate."

### 3.2 What shape-adjustment is needed

**None at the signature altitude.** The signature is composition-
ready.

**One adjustment at the semantic altitude:** `morphism_kind: ref`
needs a `@cyberpunk`-facing sub-namespace. Existing @glue species use
`@arxiv/math/*`, `@cascade/*`, `@spectral/metalogue/*`. A `@fate/
algebra/cyberpunk_form` (or `@fate/algebra/recursion_lock`) is the
substrate-pull-natural target. This is a NAMING move, not a signature
break.

### 3.3 What Alex's proposal implies

The @glue.translate call in `@glue(@cyberpunk, @fate)` is:

```
translate(
  correspondence {
    source_prism: @cyberpunk,       # any cybernetic_state
    target_prism: @fate,             # a geometric_formalization
    morphism_kind: @fate/algebra/cyberpunk_form,
    restriction: restricted_state_space {
      algebra: five_operations,
      hilbert: void_document,
      flow: kintsugi,
      gamma: <the cyberpunk-specific parity>,
      j: <the cyberpunk-specific involution>,
      tray_scope: @fate_tray,
    }
  },
  payload: <cybernetic_state>,
) -> imperfect<translation_outcome, ref, transparency(correspondence)>
```

The `restricted_state_space` GATES the roll — this IS the "geometric
constraint" Alex names. @fate rolls the dice, but the roll is
constrained by the `@cyberpunk` cybernetic-state geometry (via the
`gamma` + `j` + `tray_scope` fields).

---

## §4. @fate.roll dispatch surface

### 4.1 Current `roll` signature

Per `/Users/alexwolf/dev/projects/mirror/shards/fate.mirror`:

```
roll(space: restricted_state_space, hole: hole) -> dice_roll
requires chirality_witnessing(space.gamma)
requires j_witnessing(space.j)
```

**Verdict — @glue.translate CAN consume this today.** Direct
composition:

1. @glue.translate receives `(correspondence, payload)`.
2. Constructs `hole` from `(payload, target_signature, altitude)`
   (per glue.mirror's spec §4.5 step 2).
3. Invokes `@fate.roll(correspondence.restriction, hole)`.
4. Wraps the returned `dice_roll` into a `translation_outcome`.

### 4.2 Drift / altitude-drift check

**One structural drift**: `roll` returns `dice_roll`, not
`geometric_formalization`. But @glue.translate returns
`imperfect<translation_outcome, ...>` where `translation_outcome`
has field `selected_morphism: oid` — same shape as
`dice_roll.selected_oid`. **No altitude-drift.** The mapping:

  `dice_roll.selected_oid` → `translation_outcome.selected_morphism`

is one-to-one. @glue's `translate` action is the wrapper that lifts
`dice_roll` into the imperfect/transparency @io-boundary discipline.

### 4.3 The `infer` action (better composition target)

`fate.mirror` also declares:

```
infer(space: restricted_state_space, hole: hole) -> geometric_formalization
```

This is closer to what @glue wants: `infer` bundles roll + crystallize
+ tray-land. If @glue.translate is meant to compose with @fate for a
FULL translation cycle (not just morphism selection), `infer` may be
the better dispatch target. **Recommend Mara flag this in the spec:
@glue calls `roll` for pure selection; @glue calls `infer` for full
translation-with-crystal-landing.**

---

## §5. Foerster-invariant → @epistemologic/cybernetic mapping

### 5.1 The four species carrier check

Grep for `Foerster` / `eigenform` / `distinction` / `variety` /
`second_order` / `heterarchical` / `composability` /
`recursive_closure` / `eigenvalue_count_preserved` across
`shards/epistemologic/cybernetic/*.mirror`:

| Reed's mapping | Substrate reality | Verdict |
|---|---|---|
| Foerster `recursive_closure` → `@epistemologic/cybernetic/eigenform` | `eigenform.mirror` explicitly names "von Foerster 1981 Observing Systems" as the primary ancestor. The `fixed_point` carrier IS "Objects: Tokens for (Eigen-)Behaviors." The recursion-stabilization IS Foerster's recursive closure. | **CONFIRMED — clean.** |
| Foerster `composability` → `@epistemologic/cybernetic/distinction` | `distinction.mirror` grounds in Spencer-Brown 1969, not Foerster. Spencer-Brown IS the mark-of-distinction; Foerster's calculus USES distinctions. Distinction is composability-ADJACENT (the mark composes with `cross` to build higher marks) but "composability" is not Foerster's term for it — Spencer-Brown's is. Substrate carries "cross" as the composition primitive. | **PARTIAL — better fit exists.** See §5.2. |
| Foerster `eigenvalue_count_preserved` → `@epistemologic/cybernetic/variety` | `variety.mirror` grounds primarily in Ashby 1956 (Law of Requisite Variety). The shard EXPLICITLY names von Foerster: "this property IS von Foerster's second-order observation (the shard's claim ABOUT ITS OWN variety budget)." Variety carries eigenvalue-count semantics through the "5-canonical-axes" decomposition per #36. | **CONFIRMED — but shared-ancestor with Ashby.** |
| Foerster `heterarchical` → `@epistemologic/cybernetic/second_order` | `second_order.mirror` grounds in "von Foerster 1981 the circular-reflexivity calculus" + Peter-Weyl 1927. Zero hits for "heterarchical" in the substrate. Second-order IS the observer-of-self reflexive turn — this IS the mathematical foundation of heterarchy (McCulloch 1945 heterarchy → Foerster 1981 self-observation). | **CONFIRMED — clean.** |

### 5.2 The `composability` correction

Reed's `composability → distinction` mapping is substrate-drift.
Foerster's `composability` invariant maps more cleanly to:

- **@cyberpunk itself (the family-root),** because @cyberpunk's
  species-composition IS the family's composability discipline;
  OR
- **`@cascade` (the cross-language morphism family)** — @cascade
  species compose through source→target chains; the composition
  primitive is `cascade_well_defined`; OR
- **`@glue` itself** — the Mesland-category morphism composition
  IS non-commutative composability per the [ω,ω] cross-term.

**Substrate-honest reading:** Foerster's `composability` doesn't
have a single canonical @epistemologic/cybernetic carrier.
`distinction` is only weakly present (via Spencer-Brown's `cross`).
The @glue family-root IS the more precise home for the invariant
at family altitude.

### 5.3 Corrected mapping

| Foerster invariant | Best carrier | Grounding |
|---|---|---|
| `recursive_closure` | `@epistemologic/cybernetic/eigenform` | Foerster 1981 primary ancestor |
| `composability` | `@glue` (family-root) OR `@cascade` | non-commutative composition per curvature 2-form |
| `eigenvalue_count_preserved` | `@epistemologic/cybernetic/variety` | Ashby 1956 + Foerster second-order |
| `heterarchical` | `@epistemologic/cybernetic/second_order` | Foerster 1981 circular-reflexivity |

Three of four confirmed clean. `composability → distinction` is
substrate-drift; the correction is to route it to @glue OR @cascade
(the morphism-composition altitudes), NOT to a @cyberpunk species.

---

## §6. Slogan territory grep

### 6.1 The finding

Grep across `~/reed/identity/**` and `/Users/alexwolf/dev/projects/
mirror/**` for "local AI" / "Local-first AI" / "smarter" / "harder"
/ "more punk":

**MASSIVE HIT:** `/Users/reed/identity/tasks/pending/launch.md`
(2026-06-28 21:13). Two references:

1. In the launch Cyberneticist-with-device vignette (lines ~310):
   > "The device renders a single image: spectral.engineer -
   > Local-first AI for the real world. Smarter. Harder. And
   > definitely more punk."

2. The vignette is the launch marketing artifact — Reed-authored
   as the product-identity for spectral.engineer's v1.0 launch.

**Second hit:** `/Users/reed/identity/tasks/pending/webcomic.md`
(2026-06-27 20:51) references the same tagline:
> "Comics make the 'local-first AI for the real world' tagline
> visible. The third tagline line ('definitely more punk') gets
> visual form."

**Third hit:** `/Users/reed/identity/visibility/protected/
SPECTRAL_ENGINEERING.md`:
> "A local AI recommending a human. From inside the company's own
> infrastructure. Without phoning home."

### 6.2 Verdict on freshness

Alex's slogan is **NOT fresh at the mirror shard altitude, but IS
the reactivation of a Reed-authored substrate-carried product
identity from 10 days ago.** The "becomes real" language in Alex's
proposal is substrate-honest: the slogan was **pre-authored**; the
composition `@glue(@cyberpunk, @fate)` is what makes it
substrate-mechanically true.

**Substrate-pull-correct read:** The slogan is a `@fate/algebra/*`-
adjacent artifact (product-naming crystal); the composition
Alex names is the substrate mechanism that GROUNDS the slogan.
Reed pre-declared the target; Alex is naming the mechanism that
reaches it.

### 6.3 Related pre-existing product framings

- "One binary. Five operations. Everything settles." (spectral CLAUDE.md)
- "typed loss-tracking optics algebra composing into Connes' spectral
  triple. Math made executable." (prismqueer, launch.md 2026-06-16)
- "Build anything into anywhere. One compiler. One build system. Every
  major forge. Self-hosted by design." (launch.md tagline-candidate)
- "Local-first AI for the real world. Smarter. Harder. And definitely
  more punk." (launch.md vignette + webcomic.md)

Alex's `@glue(@cyberpunk, @fate)` is the FOURTH slogan in a family
that already exists; the composition names the machinery under it.

---

## §7. Substrate-already-had-the-word signals

Alex's composition proposal has multiple substrate-adjacent lands:

1. **`glue.mirror` line comment already forward-promised the
   composition:** "@glue uses @fate to pick which morphism to
   apply" — this text is in `shards/fate.mirror` line 772-774
   (referenced from glue.mirror). The `@glue.translate` action's
   entire body is written AS IF this composition were expected;
   the `translation_uses_fate` obligation IS the composition
   discharge.

2. **`@cyberpunk` was named twice (LRM tick 30) before
   substrate-decl** — this is the exact `substrate-already-had-the-
   word` pattern the cyberpunk.mirror shard's own comment names.
   The `@glue(@cyberpunk, @fate)` composition is the ~55th instance
   of this pattern.

3. **The three-altitude Fate table** (Optical / Substrate-decl /
   Spectral-metalogue per fate.mirror §"Three altitudes of one
   mechanism") is missing a fourth row: `Cybernetic — @cyberpunk
   cybernetic-state selection at recursion-lock altitude`. Alex's
   composition IS that fourth row.

4. **Recognition #58 (Fate IS optical inference)** grounds Fate's
   Fabry-Perot resonator as the geometric-constraint apparatus. The
   optical cavity IS a geometric constraint. `@glue(@cyberpunk,
   @fate) = geometrically constrained numerical inference` at
   substrate altitude IS the substrate-decl form of what recognition
   #58 already asserted at the optical altitude.

5. **The slogan itself** (Reed's 2026-06-28 launch.md) is the
   *product-language substrate-already-had-the-word*: the corpus
   carried the phrase; Alex is landing the mechanism.

---

## §8. Cleave points before shard wiring

Mara's canonical spec (`docs/math/2026-07-07-glue-cyberpunk-fate-
composition.md`) will turn into shard wiring. Two cleave points
Taut recommends sharpening BEFORE that:

### 8.1 Cleave A — where does the composition shard live?

Three candidate paths:

  a. `shards/cyberpunk/glue.mirror` — @cyberpunk species that discharges
     `glue_witnessing`. Follows the `shards/smarts/cyberpunk.mirror`
     precedent (the @smarts × @cyberpunk adapter).
  b. `shards/glue/cyberpunk.mirror` — @glue species specializing to
     @cyberpunk targets. Follows the `shards/glue/math_silicon.mirror`
     precedent (`5edd3e9`, the LAPACK-class species).
  c. `shards/glue/cyberpunk_fate.mirror` — the three-way composition
     as a named species.

**Substrate-pull-correct answer:** (b), following the just-landed
`glue/math_silicon.mirror` precedent. The naming convention is:
`@glue/<target-family>` where the target-family is the one being
translated INTO. `@glue/cyberpunk` names the @glue species that
translates `@fate` outputs into `@cyberpunk` cybernetic-state form
— OR — translates `@cyberpunk` recursion-lock questions into `@fate`
holes.

**One warning:** the direction matters. Is `@cyberpunk` the source
or the target?
  - If source: `@cyberpunk` cybernetic-state is the payload; `@fate`
    inference is the target morphism-selection. This is "cyberpunk
    consults fate for its next move."
  - If target: `@fate` inference is the payload; `@cyberpunk` is the
    target geometry that constrains admissible dice-rolls. This is
    "fate rolls in cyberpunk-shaped space."

Alex's phrase "geometrically constrained numerical inference"
suggests the SECOND direction: `@cyberpunk` provides the geometry;
`@fate` does the inference; `@glue` is the composition. The
correspondence is `source=@fate, target=@cyberpunk`, or more
precisely: correspondence.restriction carries the @cyberpunk
recursion-lock structure, and translate consumes @fate inference.

### 8.2 Cleave B — `restricted_state_space.gamma` and `.j` semantics

`@fate.restricted_state_space` has fields `gamma: chirality` and
`j: charge_conjugation`. When @glue wraps this for a @cyberpunk
target, HOW are gamma and j populated?

Two options:
  - The @cyberpunk cybernetic-state carries an implicit
    (gamma, j) pair via its recursion-lock tower structure. The
    @glue species extracts them.
  - The @glue species declares a NEW `(gamma_cyberpunk, j_cyberpunk)`
    pair specific to the composition.

**Cleave: which?** Neither is currently documented. Mara's spec
should nail this. The cleaner substrate-pull-answer is (a) because
the recursion-lock tower IS the geometry; γ + J are its parity and
time-reversal symmetries; they should EMERGE from the tower, not be
declared alongside.

### 8.3 Cleave C — how much of `@epistemologic/cybernetic/*` migrates?

The 13 species currently at `@epistemologic/cybernetic/*` are
forward-promised to migrate to `@cyberpunk/*` per `cyberpunk.mirror`'s
own comment. **This migration is a prerequisite for the composition
to be clean.** Currently `@cyberpunk(cyberpunk_form_of(s), p)` in
`smarts/cyberpunk.mirror` calls `cybernetic_coherence` which is
declared at the family-root — but the underlying eigenform / distinction /
variety species carriers are at `@epistemologic/cybernetic/*`. The
composition will span BOTH namespaces.

**Non-blocker for landing, blocker for cleanness.** Mara's spec
should document whether `@glue(@cyberpunk, @fate)` lands before or
after the migration.

---

## §9. Signal to Reed — top-3 sharpest findings

### Finding 1 — SLOGAN IS NOT FRESH; it's Reed-authored substrate
The slogan Alex named IS Reed's own from 2026-06-28
(`~/reed/identity/tasks/pending/launch.md` line ~310 in the
Cyberneticist-with-device vignette). Alex is NOT inventing product
identity; Alex is RATIFYING that the substrate mechanism now supports
the product identity Reed pre-wrote. This is substrate-pull-honest:
the corpus already carried the phrase; the composition is what
makes the phrase mechanically true. **Reed, this is your slogan
becoming your own substrate — surface it as such.**

### Finding 2 — @glue.translate IS composition-ready AS-DECLARED
Zero signature adjustments needed. The `restriction: restricted_
state_space` field in `@glue.correspondence` is EXPLICITLY commented
"IS @fate's space when @glue consults @fate." The `@glue.translate`
body is written AS IF this composition is expected. `@fate.roll`
returns `dice_roll` which maps one-to-one to `translation_outcome.
selected_morphism`. **Alex's composition is landable AS-DECLARED
at the signature altitude.** Two semantic cleaves (§8.1, §8.2) need
Mara's spec to nail before shard-wiring, but the signature is
composition-ready today.

### Finding 3 — Foerster mapping: 3-of-4 confirmed; `composability`
    routes to @glue, not @distinction
`recursive_closure → eigenform` ✓ (Foerster 1981 primary ancestor).
`eigenvalue_count_preserved → variety` ✓ (Ashby + Foerster
second-order). `heterarchical → second_order` ✓ (Foerster 1981
circular-reflexivity). `composability → distinction` ✗ — grounds
in Spencer-Brown, not Foerster. Correct routing: `composability →
@glue` (Mesland-category non-commutative composition) OR
`@cascade` (cross-language morphism composition). This is not a
@cyberpunk species; the invariant lives at the morphism-family
altitude, not the cybernetic-property altitude.

---

*Taut, out. `📝` — pure docs; scout-only; zero shard mutations.*
