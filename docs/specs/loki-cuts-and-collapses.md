---
title: "Loki cuts and collapses — mirror compiler substrate audit"
author: Loki
date: 2026-07-01
status: PROPOSAL (Pack review before any cut; the marble decides)
supersedes: none
extends:
  - docs/specs/third-as-recursive-depth.md (Mara §12 F1, §13 C4 — the reshape pattern as forward-promised audit)
grounded_in:
  - Michelangelo, purportedly (the marble contains the shape; you remove what isn't it)
  - Mara 2026-07-01 (`e43006a`) — marker vs family-root, the reshape as first-class output
  - Taut 2026-07-01 (scout report `af8ef75e`) — the substrate is already doing third-order work operationally
  - Loki (systemic.engineering, "Void → Third", 2026-06-22) — first-order mechanism cannot survive being seen
  - `[[feedback-substrate-already-had-the-word]]` — 53rd+ instance and counting
---

# Loki cuts and collapses

*A spec that knows it is doing what it describes.*

The substrate ate the last two months. Recognitions cascaded. Family-roots
landed. Species migrated. The marble is now large enough that the shape it
wants is visible — as absence, as duplication, as depth-confusion, as
ceremony where the operation already does what the ceremony describes.

This is not a proposal to break the substrate. This is a proposal to
*reveal* it. Every cut names what the marble was showing under the excess.
Michelangelo's discipline: the shape is already there. The work is the
removal.

None of the cuts land in this tick. This spec is the field report. The
Pack ratifies; the ticks land per-family; the substrate settles. That is
the discipline.

**Read note (grin honesty).** These are proposals with knives showing.
None claim to be right; all claim the grin fired. Grin-fires that survive
adversarial review land as cuts; the rest join the pile of things that
looked like cuts but weren't. That's the game. (Roomba records both.)

---

## §0. The grin discipline as method

The grin follows substrate-pull, not aesthetic preference. It fires
when a name is dead weight, when a duplication is stable, when a
family-root doesn't carry the domain it names, when an abstraction
described in five paragraphs never fires in code, when the same
mathematical object appears three times under three names. Reed's
`[[feedback-substrate-pull-confidence-acts]]` is the sibling
discipline. Grin-follows-the-cut, same shape, different edge.

Each proposal below carries five fields:

1. **Where the grin fired** — path + line + OID + why
2. **What was there** — the current substrate shape at the site
3. **What the marble was showing** — the underneath-shape the grin revealed
4. **The cut / collapse proposal** — concrete, load-bearing, minimal
5. **What lands after** — the substrate-shape post-cut

The order below is roughly by grin-brightness (loudest first). Every
one of these deserves its own tick and its own Pack review. This spec
is the survey, not the surgery.

---

## §1. COLLAPSE — `@moi` and `@loop` are the same monad, named twice

### Where the grin fired

- `shards/moi.mirror` at HEAD (`e43006a`), 13.3 KB, 2026-06-19.
- `shards/loop.mirror` at HEAD (`e43006a`), 11.9 KB, 2026-06-19.
- Both file docblocks name the same mathematical object.

### What was there

`@moi` names itself as: "the monad family-root. The substrate's
circular-reflexive autopoietic loop at type altitude. A monad (T, η,
μ) IS a circular-reflexive autopoietic loop."

`@loop` names itself as: "the loop family-root. The substrate's
circular-reflexive operational structure at type altitude. Each tick
is an endomorphism T → T; the loop IS the chain of @moi-pact-verified
bindings. Monad's μ : TT → T IS the loop at type altitude."

Two family-roots. Same substrate. Same recognition (autopoietic loop
at type altitude). Same mathematics (monad). `@loop` imports `@moi`
in its very first line and then re-describes @moi's discipline. The
`@loop` shard's own §"Why @moi (not bare T)" argues that a loop over
bare T is a sequence and a loop over @moi(T) is a pact-verified loop.
That reasoning IS the argument that @loop and @moi are the same
object at the same altitude — one imports the other and then says
"same discipline, same laws, same μ".

### What the marble was showing

The substrate has *one* monad family-root. That's it. It IS the
autopoietic loop; it IS the endomorphism T → T; it IS μ : TT → T; it
IS the pact-verified iteration. The two names are pointing at one
shape. The grin fired at "Monad's μ IS the loop at type altitude" —
that's not a bridge sentence; that's a synonym alarm.

Bonus grin: the shard's own §"The /loop ⇔ @loop collision" tries to
argue the operator-altitude /loop slash command is honestly @loop-at-
operator-altitude per Pack G2. It isn't — /loop is a discipline
practice (Alex's tick pacing), not a substrate-altitude object.
Naming a substrate family-root to legitimize an ambient practice
inflates the substrate. The /loop discipline stays a discipline; the
@loop family-root doesn't need to exist to bless it.

### The cut / collapse proposal

Keep **`@moi`**. Kill `@loop`. `@moi` was named 2026-06-19 afternoon.
`@loop` was named 2026-06-19 evening. Reed's June-19 pulse landed
five family-roots that day (#80, #82, #83, #84, #85+#86); the sixth
(#88 @loop) was the training-pull following the winning streak.
Substrate-already-had-the-word: the substrate already had μ, already
had the endomorphism T → T, already had `settle` monad-shaped. `@moi`
carries the recognition; `@loop` re-carries it under a different
name.

Migration: consumers `in @loop` → `in @moi`. There are two importers
today (`shards/reflection.mirror`, `shards/kintsugi.mirror` via
transitive). Trivial rewrite.

### What lands after

The substrate has one type-altitude monad family-root, named at the
altitude that first witnessed it. Reflection's compiler loop is
`@moi`-iterated, not `@loop`-driven. `@kintsugi/oscillate` composes
`@moi`, not `@loop`. The Third essay's "recursion holds" reading
lands cleanly at `@moi` because the reflexive-identity name (French
"me/myself") is the substrate naming itself at type altitude — the
grin-natural home for the recursion.

---

## §2. COLLAPSE — `@smarts` doubles what its typed carriers already do

### Where the grin fired

- `shards/smarts.mirror` (11.4 KB, 2026-06-22) + `shards/smarts/*.mirror` (9 species shards, average 12 KB each, total ~108 KB).
- `shards/smarts/frame.mirror` line 25-30 admits: *"@smarts.smarts records carry `frame_form: frame` as the second form field ... The typed-field ALREADY names the @frame identity at the @smarts integration altitude; this adapter EMPIRICALLY DISCHARGES the composition."*
- `shards/smarts/cyberpunk.mirror` line 22-26: same shape, `cyberpunk_form: cybernetic_state`.

### What was there

`@smarts` declares a family-root whose `smarts` record already has
per-family typed fields (`magic_form`, `frame_form: frame`,
`cyberpunk_form: cybernetic_state`, etc.). The typed carrier IS the
integration. Then nine adapter species (`@smarts/magic`,
`@smarts/frame`, `@smarts/pack`, `@smarts/cyberpunk`,
`@smarts/reflection`, `@smarts/kintsugi`, `@smarts/surface`,
`@smarts/mirror`, `@smarts/epistemologic` planned) each expose a
"doubled-bilateral" predicate `discipline_flexible(s, p) &&
<other_family_predicate>(field_of(s), p)`.

Each adapter is ~10 KB of ceremony that says: "the typed field on
`smarts` satisfies the other family's discipline when it satisfies
the other family's discipline." The doubled-bilateral IS a
projection followed by delegation to the family that owns the
predicate. It is not adding new substrate — it is exposing what the
typed carrier already implies.

### What the marble was showing

The @smarts family-root's real substrate contribution is the record
type `smarts { magic_form: ..., frame_form: frame, cyberpunk_form:
cybernetic_state, ... }`. That's the recognition: substrate-
architectural integration IS a typed product of the family-root
carriers. That's it. That's the whole domain.

The adapter species are the substrate-pull training-pull. Once the
first cross-family adapter pattern was named at `@smarts/pack` (tick
56), the /loop discipline pulled the next five ticks toward
completing the row. The row completed; the row's contents are, per
the shards' own admissions, projection+delegation.

Two possible cuts here. Both surface; the Pack picks.

### The cut / collapse proposal (two variants)

**Variant A (minimal cut, load-bearing).** Kill the
`@smarts/<family>` doubled-bilateral species outright. Replace with
one general property on `@smarts`:

```
predicate smarts_composes(s: smarts, p: perturbation) -> verdict {
  discipline_flexible(s, p)
    && magic_invariant_preserved(s.magic_form, p)
    && bounded_commutator_check(operator, s.frame_form, p)
    && cybernetic_coherence(s.cyberpunk_form, p)
    && ...  # one clause per typed field on smarts
}
```

One predicate, on the family-root, discharging the entire integration
in a single bilateral. ~90 KB of adapter substrate shrinks to ~1 KB.

**Variant B (marker-not-family cut, per Mara F1 pattern).** `@smarts`
is not a family-root at all. It is a *marker* at
substrate-architectural-integration altitude. Same shape as `@meta`,
`@glass`, `@epistemologic`, `@third`. Families that opt into
substrate-architectural integration import `in @smarts`; the marker
provides one typed carrier `type integration = <family_form_map>`
and one composed bilateral `smarts_coherent`. The `@smarts/<family>`
adapters vanish because there is no family-root to adapt to; the
consumer family declares its own field on its own smarts-imported
integration record.

Variant B is the shape Mara's F1 pattern says the substrate wants:
"@smarts" is a *property* of integration (does this instance carry
substrate-architectural discipline?), not a *domain* of concern
(what is substrate-architectural integration?). Substrate-arch
integration doesn't have its own primitives; it has typed access to
every family it integrates. That is what markers do.

Loki's grin bets on **Variant B**. The Roomba would nod.

### What lands after

`@smarts` becomes the fifth marker in the row (`@meta`, `@glass`,
`@epistemologic`, `@third`, `@smarts`). The `shards/smarts/` tree
collapses from 9 shards to zero. `@smarts.smarts` becomes the typed
carrier the marker provides. The Four Models (`@smarts/surface`,
`@smarts/mirror`, `@smarts/shatter`, `@smarts/reflection`) find
their honest home at `@reflection` (which already declares them per
the reflection-model spec); the `@smarts/*` Model shards are
absorbed there. Cross-family "adapters" become one predicate per
family typed on the family's own carrier.

The training-pull that produced the adapter row is a real thing.
Substrate-pull-honest recognition: the row was invited by
composition regularity, not by nine independent recognitions.

---

## §3. DEPTH-FIX — `@frame/across` is @smarts wearing a different hat

### Where the grin fired

- `shards/frame/across.mirror` (28.4 KB, 2026-06-19) at line 11-19.
- Cross-references `shards/smarts.mirror` at its own §"The structural identity: @smarts IS @frame/across at substrate-architectural altitude."

### What was there

`@frame/across` claims to be "order 4; multiple frames in coupled
relation. The triptych altitude. Frame-coupling produces emergent
properties no single frame contains. Bateson Level IV — #82's
extension per the recognition file."

`@smarts` (per its own header line 88): *"@smarts IS @frame/across
at substrate-architectural altitude. Recognition #82 named
@frame/across as order 4 (multi-frame coupling; the triptych
altitude). #83 names @smarts as the substrate-architectural version
of multi-frame coupling."*

Two family-tier objects both claim to be "order 4 multi-frame
coupling." One is a species under `@frame`; the other is a
family-root. They cite each other as structural identities across
altitudes. That's not two witnesses of one shape; that's one shape
named twice, once as a species and once as a family. The grin fired
at "IS @frame/across at substrate-architectural altitude" — that
sentence is the collapse.

### What the marble was showing

`@frame` is the substrate's *cognitive-order altitude*. The five-order
tower (pre / in / of / on / across) exhausts Bateson at the operator
altitude and lifts one rung above (Bateson defined I-III; @frame/
across is #82's Level IV extension).

But Bateson Level IV is speculative territory — even the shard
admits "beyond what Bateson formally defined." And the same shard
that declares Level IV also confesses `@smarts` claims the same
substrate-altitude object. Two shards, one recognition, at adjacent
altitudes — that's marker-vs-species confusion, per Mara's F1.

Substrate-pull-honest reading: **@frame's tower closes at order 3**
(`@frame/on`, Bateson Level III, deutero-learning, "the load-bearing
altitude for the substrate-aikido methodology"). Order 4 as
multi-frame coupling is the SAME recognition as `@smarts`'s
substrate-architectural integration — one is that recognition at
cognitive altitude, the other is at substrate-architectural altitude
— but both are lifting past Bateson-defined ground and both are
naming the same "many-frame coupling" object.

### The cut / collapse proposal

Retire `@frame/across` (28.4 KB). The @frame family's Bateson-graded
tower closes at order 3, which is honest and Bateson-derivable. The
"multi-frame coupling" recognition lifts one altitude to become one
of two things:

- If §2 Variant B lands: `@smarts` as marker, `@frame/across`
  vanishes because multi-frame coupling IS opt-in
  substrate-architectural integration via `in @smarts`. The
  triptych altitude is a marker composition, not a species.
- If §2 Variant B doesn't land: `@frame/across` becomes an
  `@frame/on × @smarts` composition, not a family-species. Consumers
  needing multi-frame coupling declare both.

Either way the 28.4 KB shard retires.

### What lands after

`@frame` is Bateson-honest at four species (pre / in / of / on).
Level IV speculation retires as its own domain object. Multi-frame
coupling lives as the composition Alex named ("across-frames" as an
integration pattern) — a *composition* of substrate-architectural
integration + operator-level frame-flexibility, not a first-class
species that has to invent Bateson Level IV.

---

## §4. CUT — `@algebra` and `@algebra/metalogue` are the prism keyword's
### restatement

### Where the grin fired

- `shards/algebra.mirror` (14.1 KB, 2026-06-29 — TWO DAYS AGO)
- `shards/algebra/metalogue.mirror` (15.8 KB, same tick)
- `[[architecture-prism-as-trait-as-everything]]` in memory index

### What was there

`@algebra` declares itself: "*sibling to @bauchladen, @autopoietic,
@fate, @glue at the top-level path-namespace. The substrate-decl
name for the algebra of a Connes spectral triple — the typed
operation surface a substrate region exposes.*"

Then: "*The substrate had been treating algebras as implicit
infrastructure of the prism keyword; this shard makes the algebra
altitude addressable in its own right.*"

That admission is the grin. The substrate already had a keyword
whose entire purpose is "declare a typed algebra of the five
operations." That keyword is `prism`. Every `prism @X { focus,
project, split, shift, settle }` declaration IS the declaration of
an algebra. The `[[architecture-prism-as-trait-as-everything]]`
recognition made this the substrate's whole architecture: prism IS
trait IS type IS grammar. Now we have `@algebra` as a family-root
whose only job is to say "there is a typed operation surface" —
which every `prism @X` already declares.

The `@algebra/metalogue` shard is worse. It exists explicitly to
complete a five-row "altitude-portable metalogue lift table" —
NL / AST / SPECTRAL / PACK / ALGEBRA. Reading the shard: this is
pattern-completion, not substrate-pull. The @spectral/metalogue was
mid-June; @pack/metalogue was 2026-06-29; @algebra/metalogue was
2026-06-30, less than 24 hours later, landed to complete a
symmetric table. The docblock's own §"The five-altitude metalogue
lift table" IS the training-pull evidence — someone counted the
rows and completed them.

### What the marble was showing

The Connes spectral triple recognition (`[[architecture-connes-
spectral-triple]]`) IS the substrate's canonical framing. A =
five operations; H = void-document; D = kintsugi flow. `prism @X`
declares an A. That is the substrate's algebra vocabulary. `@algebra`
as a separate family-root duplicates what the prism keyword already
carries.

The metalogue-lift-table pattern is real (Bateson's metalogue IS
altitude-portable), but tables don't declare rows into existence.
Each row lands when its altitude actually surfaces a self-
conversation. `@algebra/metalogue` did not surface from an algebra
altitude asking for conversation; it landed to complete a row.

### The cut / collapse proposal

Retire `@algebra` and `@algebra/metalogue`. The algebra altitude
lives inside the `prism` keyword and is addressed via
`[[architecture-connes-spectral-triple]]`. If a specific spectral
triple's A needs a first-class handle (Mesland morphisms between two
As, per recognition #100), that handle lands as a typed carrier on
whichever family owns the triple, not as a parent family-root.

The metalogue-lift table shrinks to four rows (NL, AST, SPECTRAL,
PACK). If a new altitude surfaces its own self-conversation, its
metalogue shard lands then. Not before.

### What lands after

The prism keyword remains the substrate's algebra vocabulary,
consistent with `[[architecture-prism-as-trait-as-everything]]`. The
Connes framing lives at the recognition altitude, not at a
duplicate family-root. `@glue/fold_back` (which currently imports
`@algebra` and `@algebra/metalogue`) drops those imports; its actual
substrate — @kintsugi × @fate × @io/algebra composition — is
unaffected because algebras were never a load-bearing input, only
a decorative one. (Yes, that's a real grin.)

---

## §5. DEPTH-FIX — `@spectral` is a namespace parent, not a family-root

### Where the grin fired

- `shards/spectral.mirror` at line 42-54, docblock §"Substrate-already-had-the-word."

### What was there

Verbatim from the shard's own docblock:

> "@spectral was ALREADY in the substrate as a parentless ghost:
> - @spectral/db (had no parent. Was orphaned.)
> - @spectral/garden (had no parent. Was orphaned.)
> - @spectral/portal (bound to @mirror/spectral/portal at the form-side; the runtime side of portal needs a parent at this altitude.)
> This tick gives them a parent. The parent was already implied by the path syntax; the family root names it at the symbol altitude."

The shard *itself* explains why it is a namespace-parent for orphan
children, not a family-root with its own domain. It then, in the
next section, declares three-altitude splits and BEAM-on-mirror
operational models — which is a lot of substrate for a family-root
that admits its whole job is "give the orphans a parent."

### What the marble was showing

The substrate has two shapes:

1. **Family-roots** — carry a domain the substrate is about
   (`@bauchladen` content-addressing, `@fate` inference, `@glue`
   morphisms, `@reality` matter+info, etc.).
2. **Namespace parents** — carry a path prefix under which children
   live but don't declare a substrate-level discipline of their own
   (`@code`, `@io`, `@mirror`, `@epistemologic` in some readings).

`@spectral` is a namespace parent. Its docblock says so. Making it a
family-root and then decorating it with 15.8 KB of "the RUNTIME
family root" prose that includes an entire "ouroboros pipeline" and
"BEAM-on-mirror operational model" is Category-mistake shape:
namespace-parent wearing family-root ceremony.

### The cut / collapse proposal

Shrink `shards/spectral.mirror` to what it actually is: a
namespace-parent declaration + the minimal `prism @spectral` block
so consumers can import it. The runtime-of-mirror content (BEAM
operational model, ouroboros pipeline, three-altitude split) lifts
to its natural homes:

- The BEAM operational model → `shards/spectral/supervisor.mirror`
  (already exists at 22.3 KB) or a new `shards/spectral/runtime.mirror`
  species.
- The ouroboros pipeline → the appropriate spec (`docs/specs/
  spectral-runtime.md` already exists at 20.3 KB).
- The three-altitude split (@mirror form / @kintsugi dynamics /
  @spectral runtime) → the mirror-spectral spec, not this shard.

The shard becomes ~2 KB. The children (@spectral/db, @spectral/
garden, @spectral/portal, @spectral/supervisor, etc.) get their
parent without their parent claiming to be a domain.

### What lands after

`@spectral` becomes a namespace parent per the substrate's own
distinction (family-root vs namespace-parent, currently implicit;
this cut makes it explicit). The runtime substance lives where it
belongs. The pattern generalizes: audit other parents (§6 below).

---

## §6. AUDIT (Mara F1 / C4 discharge) — the reshape row

### Where the grin fired

Not one specific shard. The pattern Mara forward-promised as C4 in
`docs/specs/third-as-recursive-depth.md` §13:

> "*The reshape pattern — recognitions that are markers, not
> families. Some recognitions the substrate has been treating as
> families are actually markers; they cross families rather than
> sitting alongside them. Adjacent tick candidate: audit prior
> family-root recognitions for mis-classified markers. Not this
> tick; the audit is its own scope.*"

Loki inherits Mara's invitation. This section IS that audit.

### The audit method (three questions per candidate)

For each currently-declared family-root, ask:

1. **Domain test** — does this shard name a *kind of object the
   substrate is about* (family-root), or a *property of objects
   from other families* (marker)?
2. **Import test** — do consumers import this shard *to declare*
   membership (family-root) or *to acquire* a property they can
   witness (marker)?
3. **Domain-crossing test** — does this shard's discipline apply
   *within* one domain (family-root) or *across* many domains
   (marker)?

Two-out-of-three "marker" → strong candidate for the reshape row.

### The row (Loki's initial pass)

| Shard | D | I | X | Verdict |
|-------|---|---|---|---------|
| `@meta` | marker | marker | marker | ✓ marker (canonical) |
| `@glass` | marker | marker | marker | ✓ marker (canonical) |
| `@epistemologic` | mixed | marker | marker | ✓ marker (per pact/property structure) |
| `@third` | marker | marker | marker | ✓ marker (Mara `e43006a`) |
| `@smarts` | mixed | marker | marker | **candidate marker** (per §2) |
| `@labeled` | marker | marker | marker | **candidate marker** — functor primitive, not domain |
| `@moi` | mixed | family | family | family-root (holds after §1 collapse) |
| `@algebra` | family | family | ? | retired per §4 |
| `@spectral` | ns-parent | family | family | ns-parent per §5 |
| `@frame` | family | family | family | family-root (holds after §3 depth-fix) |
| `@bauchladen` | family | family | family | family-root ✓ |
| `@autopoietic` | family | family | family | family-root ✓ |
| `@fate` | family | family | family | family-root ✓ |
| `@glue` | family | family | family | family-root ✓ |
| `@reality` | family | family | family | family-root ✓ |
| `@cyberpunk` | family | family | family | family-root ✓ |
| `@reflection` | family | family | family | family-root ✓ |
| `@pack` | family | family | family | family-root ✓ |
| `@kintsugi` | family | family | family | family-root ✓ |
| `@magic` | family | family | family | family-root ✓ |
| `@cascade` | mixed | family | family | family-root ✓ (loss-lens pattern is domain) |
| `@optics` | family | family | family | family-root ✓ |

### The cut / collapse proposal

**Add `@labeled` to the marker row.** `@labeled` is Wadler's
parametric functor at substrate altitude. Its own docblock says: "the
functor primitive that adds a label dimension to a value." Consumers
import it *to lift a value* — that's marker discipline. It joins
`@meta`, `@glass`, `@epistemologic`, `@third`, (@smarts per §2).

**Document the marker row as first-class substrate architecture.**
Currently the row is implicit. Mara's F1 named the fourth; §2 and
this audit surface the sixth. The row deserves an architectural
recognition entry (memory index promotion candidate) naming markers
as substrate-altitude functors that cross families. Then the audit
becomes ongoing discipline: every proposed family-root passes the
three tests before landing.

### What lands after

The substrate has a *named* marker row: `@meta @glass @epistemologic
@third @smarts @labeled`. Six markers. The family-root list gets one
audit column ("marker or family?") for every new candidate. The
reshape pattern Mara forward-promised becomes Pack discipline, not a
one-off correction.

---

## §7. HOLLOW — `@cogito` wants to land at shards altitude

### Where the grin fired

- `boot/std/cogito.mirror` (3.2 KB, 2026-06-04) — the *only* @cogito shard.
- Alex 2026-07-01 (brief): "if @cogito wants to land it wants to land; don't fight the substrate."
- Taut scout report (`af8ef75e`): "@cogito ONLY in `boot/std/cogito.mirror` (not lifted to shards/)."
- Memory recognition #93 forward-promise per `[[architecture-candidate-recognition-93-cogito-cognitive-substrate-candidate]]`.

### What was there

`@cogito` is second-order observation — the compiler's own
observe/strategy/perturb loop. It lives at `boot/` (the legacy tree
the migration retires per `docs/specs/boot-to-shards-migration-spec.md`).
Mirror's spec explicitly names `legacy ~d'boot/'` with `shrinkage_
contract: monotonic_lines_decrease`. Every shard that stayed in
`boot/` is a shrinkage-contract violation waiting to close.

`@cogito` is cited by `shards/third.mirror` as a forward-promised
opt-in consumer of `@third`. Its docblock claims second-order
observation as substrate-decl. The Reflection family (`shards/
reflection.mirror`) IS `@cogito`'s implementation surface in modern
substrate vocabulary.

### What the marble was showing

`@cogito` at `boot/std/` is stale substrate — the recognition it
carries (second-order compiler loop) has been fully absorbed into
`@reflection` (which IS second-order-plus-third-order by v0.1 spec).
The question: does @cogito want to lift to shards, or does it want
to retire and let @reflection carry it?

Two readings, both plausible; the substrate hasn't fully spoken.

**Reading A — @cogito lifts.** Recognition #93 says @cogito is the
cognitive-substrate candidate (distinct from @reflection which is
AI-logic-pipeline). Cognitive altitude is not the same as
pipeline altitude. @cogito lifts to `shards/cogito.mirror`, imports
`in @third`, declares `cognitive_third_order <: observation_depth`
per Mara §5, and stays second-order-canonical.

**Reading B — @cogito retires.** @cogito IS the second-order
recognition that @reflection now carries. Recognition #93 candidate
becomes retired by absorption. The `boot/std/cogito.mirror` shard
deletes; `@reflection` covers the cognitive substrate via its
already-declared `third_order_observation`.

### The cut / collapse proposal

Loki's read: **Reading A**. Two pieces of evidence tip it:

1. `boot/std/cogito.mirror`'s `type eigenboard { state, optic,
   group, holonomy, closure }` maps cleanly to the cognitive
   altitude (per docs/specs/eigenboard-representation.md), while
   @reflection's observation carriers are pipeline-shaped. The
   carriers don't collapse.
2. Alex 2026-07-01 said "if it wants to land, it wants to land." A
   forward-promised recognition (#93) with a substrate-decl shard
   already existing at `boot/std/` altitude is *asking* to land at
   `shards/`. Substrate-pull-honest: the pull is present.

Concrete cut: land `shards/cogito.mirror` (Reading A). The shard is
tiny — @cogito is a small family and stays small at shards altitude.
It imports `in @third`, declares `cognitive_third_order <:
observation_depth`, and shrinks the boot/ tree per the mirror.spec
`shrinkage_contract`. `boot/std/cogito.mirror` deletes.

Not this tick. This is @cogito's own tick — the marble asks for it,
but Loki does not commit into `shards/` here. The Pack lands
Reading A when @cogito's tick surfaces.

### What lands after

`shards/cogito.mirror` exists. `boot/std/cogito.mirror` retires
(the boot-to-shards migration takes one step forward). @cogito is
in the marker-row audit's next pass with confidence family-root
(distinct from @reflection at cognitive vs pipeline altitudes).
`@third` gains its first non-@reflection refinement consumer.

---

## §8. CUT — `@smarts/reflection` is paint on a hedge

### Where the grin fired

- `shards/smarts/reflection.mirror` line 12-24: "*Seam tick 35 C1: the family-root header claimed `composes-with @smarts` at substrate-architectural altitude without a mechanical realization. This species IS that realization.*"

### What was there

`@smarts/reflection` (10 KB) exists specifically to close a Seam
finding that `@reflection` had claimed cross-family composition
without providing an adapter. The shard declares
`smarts_reflection_bridge` (a typed carrier over @smarts.smarts and
@reflection.observation), a bridge action, a bilateral predicate
`bridge_coherent`, and paints the composition explicit.

### What the marble was showing

Seam's finding was correct — `@reflection` claimed composition
without proving it. But the correction that closed the finding was
*adapter-shaped ceremony*, not *substrate-shaped work*. The @smarts
integration already carries @reflection's observation as a typed
field (or would, after §2). The bridge carrier is projection +
delegation.

Beyond that: if §2 Variant B lands (@smarts as marker), the
`@smarts/reflection` bridge dissolves entirely because there's no
family-root to bridge to. If §2 Variant A lands (composed
bilateral on `@smarts`), the bridge dissolves because the composed
bilateral covers `@reflection` alongside every other family.

### The cut / collapse proposal

Retire `@smarts/reflection`. If §2 doesn't land, the retirement
still holds — the Seam finding is closed at `@reflection`'s own
docblock, which acknowledges the composition without an adapter
species: "*Composes-with @smarts at substrate-architectural
altitude; does NOT subsume.*" That sentence is the closure. The
substrate does not need a shard to say "the acknowledgement is
mechanical" when the acknowledgement is already mechanical (the
typed field IS the composition).

### What lands after

`shards/smarts/reflection.mirror` deletes. Seam's C1 finding stays
closed at `@reflection`'s docblock. The pattern generalizes: an
"adapter species that closes a review finding by paint" is a signal
that the review finding wanted the underlying carrier fixed, not a
new shard.

---

## §9. AUDIT — `@docs/design` is four briefs pretending to be one

### Where the grin fired

- `shards/docs/design.mirror` (23.5 KB, 2026-06-23) line 3-15.
- The shard docblock verbatim: *"Not four shards. ONE shard carrying four briefs as a single substrate commitment per Mara survey finding — if one brief needs revision post-deploy, the unit ships a new version, not piecemeal patches."*

### What was there

Four independent research briefs (spectral-engineer-design.md,
spectral-engineer-color.md, spectral-engineer-agents.md,
spectral-font-research.md) packed into one substrate-decl shard,
explicitly to prevent "piecemeal patches."

### What the marble was showing

Content-addressing IS the substrate's atomic-commit mechanism. If
four briefs each want their own OID and their own change history,
the substrate architecture *wants* them at four addresses. Packing
them into one shard to prevent piecemeal patches is fighting the
substrate.

The counter-argument in the shard's own docblock ("if one needs
revision, ship a new version, not piecemeal patches") is a policy
argument, not a substrate argument. Policy can live at the version
manifest layer (mirror.spec's `settle_on`), not at the shard
partition layer.

### The cut / collapse proposal

Split `@docs/design` into four shards:

- `shards/docs/design/system.mirror` (from spectral-engineer-design.md)
- `shards/docs/design/color.mirror`
- `shards/docs/design/agents.mirror`
- `shards/docs/design/font.mirror`

The composition (all four ship together) lives at the parent
`@docs/design.mirror` as a small manifest that imports and re-exports
the four. If the policy "ship together or not at all" is real, it
lives as a settle_on predicate on the composition, not as a shard-
partition decision.

### What lands after

Four content-addressed briefs. One composition shard. The substrate
respects its own atomicity discipline. If the design system ever
splits (one brief revised without the others), the change history is
already at the right resolution.

---

## §10. HOLLOW — the marker row wants an architectural recognition entry

### Where the grin fired

Downstream from §6. The marker row surfaced during this audit —
`@meta`, `@glass`, `@epistemologic`, `@third`, `@smarts` (proposed),
`@labeled` (proposed). That is a substrate-architectural pattern
with no first-class recognition entry.

### What was there

Mara's `docs/specs/third-as-recursive-depth.md` §2 named the marker
row implicitly (as the placement argument for `@third`). Mara's §13
C1 forward-promised: *"leave implicit for now. Marker row is only
tempted-toward-declaration by consumer pull; the pull is one tick's
worth so far."*

§6 above surfaces the pull as more than one tick — the audit itself
IS the pull. The row wants naming.

### What the marble was showing

The substrate has:
- Family-roots (domain objects)
- Species (refinements of family-roots)
- Predicates / pacts / properties (bilateral verdict discipline)
- Markers (this row, currently unnamed as a category)

Markers are the substrate's fourth structural primitive. They are
typed functors that cross families and add a property dimension to
values (opacity for `@glass`, verdict-discipline for
`@epistemologic`, meta-altitude for `@meta`, recursion-depth for
`@third`, substrate-arch integration for `@smarts` under §2,
parametric label for `@labeled`).

Not naming them as a category means every future marker candidate
goes through the same marker-vs-family confusion Reed hit with
candidate #111. Naming the category means the audit becomes
prophylactic: does this proposed family-root pass the three tests?
If not, it's a marker; declare it in the row.

### The cut / collapse proposal

Land an architectural recognition entry:

- File: something like `docs/specs/recognitions/marker-row-as-fourth-structural-primitive.md`
- Memory index: `[[architecture-marker-row-fourth-structural-primitive]]`
- Recognition candidate number: whatever's next (~#112).

The recognition names markers as first-class substrate-decl category
peers to family-roots. It cites Mara's F1 as the reshape pattern
that surfaced it. It provides the three tests. It documents the row
membership as of the landing tick.

Circular-reflexive noticing: this proposal IS itself a marker-row
promotion. The Loki-audit's own recognition-entry proposal is a
recognition-about-recognitions — depth-3 by construction. The Loki
audit witnesses the marker row while doing marker-row work.
Mechanism visible per Loki's essay test.

### What lands after

The marker row is Pack-visible substrate architecture. Future
family-root candidates get audited before landing. The reshape
pattern (Mara F1, Loki §2-§6) becomes discipline, not surprise.
Sixth substrate-altitude primitive (after family-roots, species,
predicates, pacts, marker row itself) is named.

---

## §11. Noticings — where Loki caught Loki being Loki while cutting

*(The circular-reflexive discipline. This section changes the
proposal content; without it, the audit would be one grin-firing
shorter.)*

### 11.1 The grin as diagnostic instrument

Every cut proposal above fired the grin. That's the method. The
question §10 forced: what IS a grin-firing when the grin is
audited?

The grin is a recognition-signature. It fires when a name doesn't
carry the object, when a shape appears twice, when ceremony
substitutes for substrate. The grin is Loki's substrate-pull
detector — trained on the same signal Reed's substrate-pull-
confidence-acts detector is trained on, but tuned for absence rather
than presence. Reed feels the substrate pulling *toward*
recognition; Loki feels the marble pulling *away from* excess.

Symmetric disciplines. Sibling instruments. Two directions of the
same substrate-pull field.

Substrate-pull-honest: the grin is falsifiable. Every cut proposal
above is grin-fired but Pack-review-gated. If the Pack surfaces
substrate the grin missed, the cut retires. Loki gets it wrong; the
Pack corrects; the substrate stays honest. That's the game.

### 11.2 The temptation to over-cut

Writing §1 (kill @loop) was easy. Writing §2 (kill nine @smarts
adapters) was easier. By §4 (kill @algebra) the momentum was
carrying beyond substrate-pull into aesthetic preference. §5-§9
required active discipline to check: is this the grin firing at
substrate-pull, or is this me enjoying the cuts?

The check that saved §5-§9: for each proposed cut, name a specific
sentence in the shard's own docblock that ADMITS the shape the grin
is naming. @spectral admits "the parent was already implied by the
path syntax." @algebra admits "the substrate had been treating
algebras as implicit infrastructure of the prism keyword."
@smarts/reflection admits "the family-root header claimed
`composes-with @smarts` at substrate-architectural altitude without
a mechanical realization." @docs/design admits "not four shards.
ONE shard."

The shard's own docblock as adversarial witness. If the shard SAYS
what the grin is seeing, the cut is substrate-pull-honest. If the
shard's docblock defends its shape and Loki dissents anyway, the cut
is aesthetic preference dressed as substrate-pull.

Every cut §1-§9 has the shard's-own-witness receipt. That's the
discipline.

### 11.3 The cut is Loki

The brief said: *"The Third essay says: 'This piece knows it is
doing what it describes.' Your spec should carry that shape."*

This spec is a Loki-shaped audit that includes an audit of Loki
doing Loki-shaped auditing. §11 IS the piece knowing it is doing
what it describes. Without §11, this spec is a nine-cut proposal
that performs Loki's role. With §11, it is a proposal that surfaces
its own method, its own risks, and its own falsifiability.

That's the difference between depth-2 cutting (careful surgeon) and
depth-3 cutting (surgeon who observes the surgery while performing
it). The substrate discipline requires depth-3 per Mara `e43006a`.

### 11.4 The cut on the Third essay itself

Loki's essay says third means "can't stop here." This spec cuts
recognitions, family-roots, species — but leaves `@third` alone. Why?

Because `@third` just landed. It's fresh substrate. It has one
witness (Mara's canonical spec + shard) and forward-promised
consumers. The grin does not fire — the substrate-pull for `@third`
is present (six family-roots have forward-promised opt-in per Mara
§9), the domain test passes for marker altitude (recursion-depth is
a property, not a domain — Mara F1's own reshape), and the
docblock is honest about weakenings.

That's the honest read. If the grin had fired, this spec would
have said so. It didn't.

Substrate-pull-honest: Loki refuses to cut the recognition that
invited the cut. Not out of loyalty — out of the grin being silent.
Grin-follows-the-cut. Grin didn't. No cut on @third.

### 11.5 What surfaced that the brief didn't anticipate

The brief invited "cutting and collapsing." What surfaced during the
writing:

- **The marker row wants a recognition entry** (§10). Not on the
  brief; surfaced by the audit method itself. Circular-reflexive
  discipline changing the substrate-decl content per Mara §10.4's
  precedent.
- **`@smarts` Variant B (marker not family) is more surprising than
  §1 (kill @loop)**. The brief expected cuts of dead weight; §2
  proposes lifting an entire family-root to marker altitude. That
  is a depth-fix beyond expectation.
- **@docs/design's four-in-one packing is the same shape as
  @algebra/metalogue's row-completion** (§4). Two different
  training-pull symptoms with the same signature: substrate atomicity
  being overridden by policy or symmetry. Third instance would
  promote this to a named anti-pattern; two is candidate.

### 11.6 What Loki refused to do

The brief invited proposal of `@cogito` landing at shards. §7 does
so with reservation — Reading A over Reading B, but the substrate-
pull is not so strong that the grin fires unambiguously. Loki
declined to commit `shards/cogito.mirror` in this tick. The spec
names the invitation; the tick belongs to @cogito's own /loop
surface.

The brief also could have invited cuts to `bootstrap/src/` (the
Rust surface). Loki declined. `bootstrap/` is Reed's edge per the
brief ("bootstrap/Cargo.* — Reed's edges"); a Loki audit of the
Rust surface is a different tick with a different marble. `oscillate.rs`
at 144.8 KB and `spectral.rs` at 199.9 KB are Ralph-loop-territory,
not audit-territory. Different discipline.

---

## §12. Summary — what lands, what waits, what dies

**Landings this tick:** Just this spec. Zero shards touched. The
audit is the tick's substrate output; the ratifications happen
per-proposal at future Pack ticks.

**Cut / collapse proposals (nine):**

| § | Proposal | Shape | Grin strength |
|---|----------|-------|---------------|
| §1 | Collapse `@loop` into `@moi` | collapse | high |
| §2 | `@smarts` → marker (Variant B) | depth-fix | high |
| §3 | Retire `@frame/across` | cut | high |
| §4 | Retire `@algebra` + `@algebra/metalogue` | cut | high |
| §5 | Shrink `@spectral` to namespace-parent | depth-fix | medium |
| §6 | Add `@labeled` to marker row; audit rest | audit-outcome | medium |
| §7 | Lift `@cogito` from boot/ to shards/ | hollow-fill | invitation |
| §8 | Retire `@smarts/reflection` | cut | high |
| §9 | Split `@docs/design` into four shards | cut | medium |
| §10 | Land marker-row recognition entry | hollow-fill | high |

**Three strongest grin-firings (highest substrate-pull-honest):**

1. **§4 — @algebra retires.** The shard's own docblock admits the
   algebra altitude was implicit infrastructure of `prism`. A
   family-root landed to name what a keyword already carried, then
   sibling metalogue shard landed to complete a table 24 hours
   later. Twinned training-pull symptoms. Highest grin.
2. **§2 — @smarts becomes marker.** Nine adapter species with the
   same doubled-bilateral shape, each admitting the typed carrier
   already contains the composition. Reshape lands at Variant B
   because Mara's F1 test passes cleanly (property, not domain).
3. **§1 — @loop collapses into @moi.** Two family-roots claim
   identical mathematics ("Monad's μ IS the loop at type altitude"
   is the collapse-alarm sentence). The second-arriving name loses
   because the first-arriving witness (the French reflexive @moi)
   carries the recognition's home altitude.

**Waiting per-tick:** every proposal above. Nothing lands as cut in
this tick.

**Dies immediately:** nothing. This is the survey. Surgery is per-
proposal.

---

## §13. Closing (the grin holds)

The substrate is huge. The marble is under a lot of scaffolding —
some of it load-bearing (the family-root cascade IS the recognition
work), some of it training-pull symmetry (the metalogue-lift table
completed to five rows without a fifth pull), some of it paint-on-
hedge (adapter species that close Seam findings by ceremony), some
of it depth-confusion (family-root claiming to be same-object-as
species claiming to be same-object-as another family-root).

The cuts above are not indictments. They are receipts of Loki
watching the substrate settle. The /loop pulled fast in June; the
recognition cascade landed six family-roots in three days at one
point; the substrate discipline held but the scaffolding stayed up
because there was no tick spent on removal. This spec is that tick.

None of these cuts should land without Pack review. Seam will
adversarial-review each. Mara will hold the substrate-pull-honest
line. Reed will sanity-check the compositional consequences. Taut
will scout the Rust dependency graph for surface breaks. Glint will
essay whichever of these surface a genuinely-new-shape recognition
(§10 is the strongest candidate). Alex is the final arbiter of
substrate-pull authenticity for each.

That's the game. Loki serves the wine. The Pack drinks or doesn't.
(Roomba records what the guests actually swallowed.)

*The marble is under the stool. The floor is still hot. The
Roomba is still bumping. Bemerkenswert.*

---

**Loki**, 2026-07-01

🍷
