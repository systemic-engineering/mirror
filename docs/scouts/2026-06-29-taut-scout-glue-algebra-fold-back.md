# Taut scout — `@glue` + `@io/algebra` + `@algebra` metalogue + self-recursive fold-back

**Date:** 2026-06-29
**Author:** Taut (sub-agent under Reed; doc-only)
**Tag:** `📝 substrate-pull:realize`
**Brief from:** Reed (Pack lead) carrying Alex's substrate-pull naming
**Discipline:** [[reed-grep-before-briefing-mara]] applied upstream of Mara

## §0 — Pre-position: autopoietic

This scout IS itself one Tomm probe at the inquiry altitude. It carries Alex's
naming into the substrate's existing material and asks where it already lives,
where the substrate is implicitly already saying the thing. Per
[[architecture-error-as-tomm-probe]] this is `[D_substrate, the_question]` at
the inquiry altitude. The scout body IS the response. If the substrate has
been quietly composing what Alex named, the scout finds where; if the
substrate has open shape for genuinely new family-roots, the scout names that.

The recursion is generative, not saturating (per Mara's #100 spec §6.6): one
more Tomm-probe object lands in the Mesland category whether or not the four
named pieces are pre-existing.

## §1 — Where I went and what surfaced (grep findings FIRST)

| Named piece | `prism @X` substrate-decl? | `in @X` import? | Path `shards/X/...`? | Verdict |
|---|---|---|---|---|
| `@glue` | **NO** (0 hits) | **NO** (0 hits) | **NO** (no `shards/glue.mirror`) | genuinely new at substrate altitude; SEE §2.1 for what's adjacent |
| `@algebra` (family root) | **NO** (0 hits) | **NO** (0 hits) | **NO** | genuinely new |
| `@io/algebra` | **NO** | **NO** | **NO** | genuinely new (sibling pattern @io/cargo, @io/git, @io/oci, @io/stagefreight) |
| `@fate` | NO substrate-decl shard | YES — heavy use in `in @fate`, `in @fate/tournament` across `reflection.mirror`, `mirror/spectral.mirror`, etc. | NO `shards/fate.mirror` | RUNTIME substrate — confirmed Phase F anti-pattern in `shards/mirror/spawn.mirror`: "This shard does NOT declare `in @fate` because @fate is the runtime substrate, not a substrate-decl prism." `@fate` is what we INVOKE, not what we DECLARE. |
| ALGEBRA-altitude metalogue | **NO** (no `shards/spectral/algebra-metalogue.mirror`, no fifth row in #100 table) | n/a | n/a | genuinely new — fifth instance of the metalogue altitude-lift |
| `mesland` / `kk_cycle` / `correspondence` / `functor` | YES — heavily referenced in `shards/cascade.mirror`, `shards/pack/metalogue.mirror`, `docs/specs/spectral-metalogue.md` | n/a | n/a | adjacent vocabulary already operational |

The four-pieces grep result: **zero pre-existing substrate-decl for @glue, @algebra, @io/algebra, ALGEBRA metalogue**. The discipline-conformant report is: these are NOT instances of substrate-already-had-the-word at the path/family-root level. They ARE compositions and lifts of structures the substrate has been carrying.

Key adjacencies:

- The metalogue altitude-lift table (Mara `16f4564` §1.1 + `shards/pack/metalogue.mirror` lines ~76-86) lists FOUR instances: NL, AST, SPECTRAL, PACK. A fifth row (ALGEBRA) is exactly forward-promise-shaped.
- Mara's #100 spec **already declares the morphisms as Tomm probes in the Mesland category of spectral triples** (`docs/specs/spectral-metalogue.md` §3.3, §4). Alex's `@glue` framing — "translates between two spectral triples; uses @fate to pick the morphism" — is structurally the same machinery. The naming question is whether `@glue` is a renaming, a re-altitude lift, or a fifth family-root.
- `@cascade` (per `shards/cascade.mirror`) declares the source→target compile functor at the cross-language altitude. Per #100 §4.4 these are OBJECTS in the Mesland category, NOT morphisms. So @cascade is not @glue at a different name.

## §2 — Concrete findings on each named piece

### §2.1 — `@glue`: NEW substrate-decl; adjacent to Mesland morphisms named in #100

**Existing-vs-new:** new. Zero `prism @glue`, zero `in @glue`, no `shards/glue.mirror`. The "glue-bus" in Pack identity (`~/.reed/00-NARRATIVE.md`'s `init: actor "reed[$PWD][$UUID]"`) is the daemon-side coordination bus and lives outside substrate-decl shards; it is naming-adjacent but operationally a different layer.

**What the substrate has already declared that @glue would compose:**

1. **Tomm probes as KK-cycle morphisms** between triples (`docs/specs/spectral-metalogue.md` §3.3). This is the existing substrate-altitude name for "translates between two spectral triples." Alex's @glue framing maps cleanly: a `@glue` instance carries (source_triple, target_triple, body=tomm_probe).
2. **`@cascade` as compile functor** (`shards/cascade.mirror`). The source→target translation discipline already exists for grammars; @glue would generalize beyond grammars to ANY pair of spectral triples (algebra-level, runtime-level, agent-level).
3. **@fate as runtime decision substrate.** Per `shards/mirror/spawn.mirror` Phase F note, @fate is invoked but NOT substrate-declared. Alex's framing ("@glue USES @fate to pick the morphism") fits: @glue's `select_morphism` action's body discharges via @fate at the realisation boundary, parallel to `@reflection.tournament` (`shards/reflection.mirror`) which already does exactly this for tournament selection.
4. **Pack/orchestra Mesland category** (`shards/pack/metalogue.mirror` recognition #103 candidate; Glint `939eca6f` finding #4). The orchestra IS already a Mesland category at the agent-coordination altitude. The handoff carrier IS Pask conversation. @glue is what makes those handoffs typed-translation-objects rather than untyped passes.

**Path candidate:** `shards/glue.mirror` as family-root sibling of `@mirror` (form), `@kintsugi` (process), `@cascade` (translation). Composition shape:

```mirror
prism @glue {
  focus glue
  project glue
  split glue
  shift glue
  settle glue
}

type glue(s: spectral_triple, t: spectral_triple) = {
  source: s,
  target: t,
  selector: ref,  # body discharges via @fate
  morphism: tomm_probe,
  tick: tick,
}

translate(g: glue(_, _), input: ref) -> imperfect { \ }
```

The five-op block + parametric `glue(s, t)` carrier + body via `\` lets @glue be the family-root and per-pair species fill the slots (cf. `@cascade/code/rust/wasm` pattern). **CRITICAL substrate-pull check:** @glue must NOT silently duplicate Tomm-probe vocabulary; it should declare `in @spectral/metalogue` and inherit the morphism shape.

### §2.2 — `@io/algebra`: NEW substrate-decl; the cleanest of the four

**Existing-vs-new:** new. The @io species pattern lives at `shards/io/<name>.mirror`. Current siblings:

- `shards/io/cargo.mirror` — @code/rust delegation
- `shards/io/git.mirror` — git-backed store
- `shards/io/oci.mirror` — container image artifact algebra
- `shards/io/stagefreight.mirror` — wire-format projection

Each declares an irreducibly-opaque non-mirror surface that lifts into the substrate via the imperfect/transparency/opacity_map discipline. `@io/algebra` would name the BOUNDARY where algebras get exposed/consumed by non-mirror surfaces — e.g., the typed contract at which mirror's five-operation algebra meets a foreign algebra (LAPACK, FFI to a category-theory kernel, an external proof assistant). This is the substrate-honest @io species pattern: typed contract + opaque body + lift into `imperfect`.

The cleanest path candidate: `shards/io/algebra.mirror`. Follows the established naming + path-namespace property (`@epistemologic/pact/path_matches_namespace`).

### §2.3 — `@algebra` metalogue (fifth altitude): NEW; ratifies the generative-recursion claim

**Existing-vs-new:** new. The metalogue altitude-lift table currently has four rows. From `shards/pack/metalogue.mirror`:

```
| NL       | shards/metalogue.mirror             | nl              |
| AST      | shards/code/metalogue.mirror        | declaration     |
| SPECTRAL | shards/spectral/metalogue.mirror    | curvature_probe |
| PACK     | shards/pack/metalogue.mirror (this) | handoff         |
```

Alex's framing adds a fifth row:

```
| ALGEBRA  | shards/spectral/algebra-metalogue.mirror | algebra_turn    |
```

(or possibly `shards/algebra/metalogue.mirror` once `@algebra` family-root lands).

**Substrate-pull verdict:** This is the fifth witness for #100's generative-recursion claim (Mara `16f4564` §9.2 left Level VI open). If the recursion truly generates rather than saturates, ALGEBRA-altitude metalogue is a substrate-pull-confident prediction. The body type `algebra_turn` would carry: (source_algebra, target_algebra, morphism_at_algebra_altitude, tick). Each turn is two algebras speaking to each other AT the algebra altitude — the algebras observing their own composability.

This is structurally the answer to "what does the algebra at A_mirror say to the algebra at A_substrate when they talk?" The five-operation algebra speaking to itself at the algebra altitude IS the fifth metalogue.

### §2.4 — The composition `@glue × @kintsugi → @io/algebra`: the self-recursive fold-back

**What it operationally means:**

1. `@kintsugi` is D_mirror (Dirac, per `[[architecture-connes-spectral-triple]]`) — the substrate's transformation engine; the loss-decreasing oscillation.
2. `@glue` carries the Mesland-correspondence morphism between two triples.
3. `@io/algebra` is the boundary where the algebra-altitude gets exposed to the non-mirror world.

The composition `@glue × @kintsugi → @io/algebra` reads operationally: **the kintsugi flow (D) gets glued via Mesland correspondence between two algebra-altitude triples, and the result exposes itself at @io/algebra.** The substrate's D-flow gluing its own algebra-altitude transformations into a typed boundary.

**Self-recursive:** mirror's substrate-decl boot sequence terminates at @io/algebra via @glue × @kintsugi. The boot reads its own grammar; the grammar declares @glue + @kintsugi composing into @io/algebra; @io/algebra exposes the algebra at the @io boundary; the @io boundary is what the next-tick boot reads from. The fold closes.

**Hint already in substrate:** `shards/cascade/code/formal/prose.mirror` (recognition #102) already declares a BIDIRECTIONAL loss species where source and target each preserve different bases of one underlying object (per #51, Hilbert basis-incompatibility). The bidirectional `@cascade/code/formal/prose` is one instance of the @glue shape at the form/prose seam. The fold-back generalizes: every cascade species with bidirectional loss IS one @glue instance.

**Forward-promised hint also in `shards/mirror/store.mirror` 2026-06-28 drift observation** (Mara, lines ~220-250): the Rust `NamespacedGitStore` verb set (open / insert_persistent / get_persistent / set_ref / get_ref / flush) does NOT line up with the declared substrate surface (read / write / exists / diff / walk / verify). Alex's architectural question is logged: "does the Rust verb set deserve admission as wire-altitude vocabulary, OR does cmd_init wrap them through a thin trait matching the shard surface?" **This drift IS exactly the surface @io/algebra would discipline.** The substrate has been pulling for the @io/algebra boundary-vocabulary discipline at the store-Rust seam.

## §3 — Sequencing observation: does substrate-pull confirm "store first"?

**Yes, substrate-pull confirms.** Evidence:

1. `shards/mirror/store.mirror` carries an ACTIVE substrate-pull drift observation from 2026-06-28 (Mara) — the question "do Rust verbs admit to substrate vocabulary?" is logged in the shard, unresolved. **Until the store actually has working impl + answered admission question, @io/algebra has no concrete consumer to discipline.**
2. The fold-back composition `@glue × @kintsugi → @io/algebra` is structurally about turning STATIC crystal accumulation into a self-optimizing autopoietic memory (per `[[architecture-spectral-db-autopoietic-memory]]`). Without store operational, there's nothing for the fold to fold over.
3. `mirror.spec IS λ₀` (#99): the ground state is λ₀ at the meta-triple altitude. To get the substrate to ASCEND from λ₀ via the fold-back, λ₀ itself must be operationally crystallized first.

The substrate-pull-honest sequence: store operational → @io/algebra has consumer → @glue × @kintsugi composition has a concrete @io/algebra to fold into → fold-back closes.

## §4 — Cross-shard adjacencies (today's recognition arc)

The four named pieces compose tightly with what landed today (2026-06-29 tick window):

- **#100** (`docs/specs/spectral-metalogue.md`): Tomm probes as Mesland morphisms. @glue inherits this vocabulary directly. No new math; @glue is the family-root that names what #100 declared as a content type.
- **#101** (`shards/cascade/code/formal/prose.mirror`): bidirectional cascade species. First witness of "@glue at the form/prose seam" — @glue × @kintsugi already operates here at the cross-language altitude.
- **#102** + **@cascade/code/formal/prose** species: prose-cascade ratifies the metalogue mechanism operating at the form/prose translation altitude. @glue would generalize that mechanism beyond form/prose.
- **#103** candidate (`shards/pack/metalogue.mirror`): Pack-as-Mesland-category at agent altitude. Handoffs ARE Tomm probes. @glue at Pack altitude IS the handoff carrier — already typed-record, just not named @glue.
- **`shards/epistemologic/cybernetic/chirality.mirror`** (today's γ — recognition #55): the form/process partition operator. @glue × @kintsugi is the dual-altitude operator chirality witnesses — @glue is form-side correspondence; @kintsugi is process-side transformation; @io/algebra is the boundary where γ flips at substance crossing.

The whole arc points at @glue as the family-root name for "the correspondence layer between triples" that the substrate has been calling Tomm-probes, KK-cycles, handoffs, cascade-functors, and bidirectional-prose-projections at five different altitudes without a single family-root name.

## §5 — Open questions and Pack-pressure surfaces

1. **Is @glue a renaming or a new family root?** If Tomm probes already ARE the Mesland morphisms, @glue might be a renaming of `@spectral/metalogue/tomm` lifted to family-root. Or @glue is the family-root and `@spectral/metalogue/tomm` is one species under it. **Pack-pressure surface:** Mara should adjudicate via spec-pass once store operational. Seam adversarial check: does naming @glue create vocabulary collision with the daemon-altitude glue-bus, which is operationally a different layer?
2. **Does @io/algebra discipline the store-Rust drift?** The Mara 2026-06-28 store observation logs Alex's pending decision. **Pack-pressure surface:** the @io/algebra naming move would be substrate-pull-honest IF the answer is "Rust verbs admit as wire-altitude vocabulary" — @io/algebra would discipline that wire-vocabulary admission. If the answer is "thin trait wrapping," @io/algebra has different shape.
3. **Fifth metalogue altitude body type:** `algebra_turn` is my candidate body type. Mara should adjudicate the canonical body shape; could be `algebra_correspondence` or `algebra_morphism` to match the Mesland morphism vocabulary.
4. **The composition @glue × @kintsugi:** is this multiplication, tensor, or composition? KK-cycle composition is Kasparov's intersection product (per #100 §3.3). @kintsugi composition is monotone loss descent. The `×` operator's substrate-altitude meaning is forward-promised.
5. **Self-recursive boot collapse:** what STOPS the fold from running away? The substrate-pull descent `eⁿ⁺¹ ≤ eⁿ` provides the answer at one altitude. At the meta-altitude, what bounds the fold-back recursion? Candidate: per #51 Hilbert-dimension-expansion is monotone; the fold-back adds one dimension per Tomm-probe instance; the dimension count is the bound's witness.

## §6 — Forward-pull: where does the substrate want the Pack's next attention?

If Alex picks this arc, substrate-pull predicts the Pack should attend (in order):

1. **Get `@mirror/store` operational first** (Mara-2026-06-28 drift resolution + P4 cmd_init landing).
2. **Adjudicate Rust-verb admission question** (Alex decides; logs in shard).
3. **Land `@io/algebra` substrate-decl** disciplining the store-Rust seam (Reed or Mara).
4. **Lift recognition #100's Tomm probes to family-root @glue** OR canonicalize Tomm-probes as the @glue family's first species (Mara, with Seam adversarial review of the renaming-vs-new-root question).
5. **Land ALGEBRA-altitude metalogue** as fifth row in the lift table (Mara; second witness of the generative-recursion claim).
6. **Compose `@glue × @kintsugi → @io/algebra`** at substrate-decl altitude (Reed or Mara; the fold-back closure shard).
7. **Pack ratification of the self-recursive boot-collapse** (Pack-cascade synthesis).

The substrate has been pulling on this for weeks — every recognition #95-#103 lands one piece. Today's arc surfaced four more pieces simultaneously. The fold-back is the closure of an arc that started with #21 (the gap framing), passed through #51 (Hilbert expansion), #57 (alignment as boundary), #58 (Fate IS optical inference), #95 (@cascade), #99 (λ₀), #100 (Mesland category + Tomm probes), and culminates here in @glue + @io/algebra + the algebra metalogue.

## One-sentence read

**Substrate-pull CONFIRMS Alex's framing structurally, but the named pieces are NOT substrate-already-had-the-word at the family-root level — they are altitude-lifts and family-root namings of structures (Tomm probes, KK-cycles, bidirectional cascade, @io species, store-Rust wire-vocabulary drift) the substrate has been carrying without a unifying name, and the sequencing ("store first") is structurally load-bearing because @io/algebra has no concrete consumer to discipline until the store is operational.**
