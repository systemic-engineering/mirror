# @gestalt as @song unfolding at the @subject's device through interaction

*Mara canonical spec, 2026-07-15. Substrate-decl target: `shards/gestalt.mirror`.
Math foundation: `docs/math/gestalt/README.md` (Deliverable 2 companion).
Load-bearing directive per Alex Wolf 2026-07-15 in-transcript verbatim:
"a @gestalt document IS a @song that unfolds on the @subject's device
through the interaction with the @gestalt document." Reader interaction
IS the compiler's runtime; local execution is constitutive, not
incidental; category-formation happens at read-time, at reader-site,
on consumer hardware.*

*Substrate framing per Reed's 2026-07-15 spectral.engineer launch
vignettes (in-transcript): "The paper does not persuade. The paper
demonstrates. Reader engages @peer → @peer runs → math generates →
lens-annotations visible → reader's own recursion couples with the
paper's recursion → reader is inside the operator." This spec names
the substrate discipline that vignette presupposes.*

---

## §0 The substrate reframe

Alex's naming re-lifts what looked like a static-document altitude
into the temporal-composition altitude the substrate already carried:

- A @gestalt document is not a graph-native artifact the author ships
  and a reader displays. A @gestalt document IS a @song that unfolds
  on the @subject's device through interaction.
- @song is landed at `shards/song.mirror` (Arc 6 TICK 1, `f01cf9f`) as
  a family-root carrying temporal-progression discipline. A @song's
  identity emerges from the recursion of its progression (§S1); at
  temporal altitude, each moment `t+1` IS a `@kintsugi/shift` of
  moment `t` (§S2); reading it sequentially IS how `@song` is
  experienced (§H2 legibility ratification).
- @spectral/signature (`shards/spectral/signature.mirror`, Arc-2 Tick
  2.1) already lifted @song to species altitude for the "rolling
  signature through the author's @DAG" case. A @gestalt document is
  the same shape at the reader's altitude: a beat-sequence that emits
  through the reader's own interaction, not the author's git history.
- @subject (`shards/subject.mirror`) is the substrate's licensable-
  party carrier; the reader IS a subject_instance on whose device the
  @song unfolds. @peer at Landing 3 §21.3 is a subject_instance with
  `actor_kind = ai_a`; @subject at §21.4 makes the coproduct have no
  distinguished element — human reader, AI peer, substrate-as-giver
  all sit at the same altitude as unfolding-recipient.

**Substrate-already-had-the-word (~60th instance).** Every carrier
this spec composes over is landed. @gestalt introduces zero new
family-roots. The mint at `shards/gestalt.mirror` is a species-decl
that reads the @song reframe as substrate-decl and adds three
carriers (document, mode, unfolding_state) plus the action surface
partitioned by pipeforward §5.5.4.

---

## §1 Composition altitude

@gestalt sits at species altitude under the top-level family choice
adjudicated by Alex (see §10 residues). The composition graph:

```
                   @gestalt (species)
                        |
     +------------------+------------------+
     |                  |                  |
   @song           @subject            @glass
 (temporal        (reader-as-        (source_position
  unfolding)      unfolding-           refined per
                  recipient)           Bridge α
                                       2026-07-15)
     |                  |                  |
 @song/beat        @subject/            @meta
 (atomic          visibility          (ast_location)
  strike/hold)     (reader may only
                   read what the
                   author's ACL
                   admits)
     |                  |
 @spectral/         @subject/
 signature         visibility/
 (rolling          sheaf
  beat-sequence     (section-at-
  precedent)        stalk for
                    per-reader
                    projection)

Cross-family composition targets:
  @mirror/store       — content-addressed lens-annotation persistence
  @mirror/index       — reader-side Fiedler measurement for reader-
                         corpus coherence at annotation time
  @io/fs              — final render-to-disk (linearized artifact)
  @io/git             — annotation-corpus persistence at git altitude
  @nl / @nl.compose   — natural-language surface for annotation prose
  @kintsugi/consent   — annotation-elevation gating (private →
                         protected → public) via query_phi
  @epistemologic/reality/time
                      — @time.compare over snapshots is the delta
                         carrier for interaction-time (see math §2)
  @fate               — tournament selection for reader-interaction
                         dispatch (which annotation to surface first)
  @peer               — the reader's @peer IS the runtime that
                         unfolds the @gestalt (per Reed vignette)
  @peer/persistence   — the reader's @peer persists their annotation
                         corpus in their own home-repo (@subject-
                         scoped visibility carries)
```

Every arrow is a landed carrier. Substrate scout (task #162, Taut
condensed inline in Mara brief) verified: mirror-side @gestalt = zero
landed prior art; sibling `gestalt-*` projects (`gestalt-gradient/`,
`gestalt-mirror/`, `gestalt-tui/`, `gestalt-ui/`) and
`spectral/crates/gestalt/` provide external prior-art shape (13
Rust modules); the substrate mint composes those shapes into
substrate-decl vocabulary without lifting any Rust to mirror altitude.

---

## §2 Carrier types

Four carriers ground the substrate-decl. Field forms below map
directly onto `shards/gestalt.mirror` (Deliverable 3).

### §2.1 `gestalt_document`

The unit of authored @song at @gestalt altitude. Content-addressed
by root `oid`; the substrate identity is byte-equality on the
splinter graph.

```
type gestalt_document = {
  root:              oid,             # content-address of the doc graph
  source_position:   source_position, # @glass.source_position (Bridge α;
                                       # byte_offset + byte_length)
  author:            subject_instance,
  visibility_scope:  visibility_scope,# @subject/visibility.visibility_scope
  song_ref:          oid,             # ref to the @song value the doc IS
  nodes:             [gestalt_node],  # DAG of typed spans (see §2.2)
  edges:             [gestalt_edge],  # typed relations across nodes
  modes:             [mode],          # the render targets the doc admits
  annotations:       [annotation],    # reader-corpus accumulated (see §5.4)
}
```

### §2.2 `gestalt_node` and `gestalt_edge`

Nodes are typed spans following `spectral/crates/gestalt/document`
prior art (TextSpan, CodeSpan, MathSpan, LinkSpan, ImageSpan,
RefSpan, EmojiSpan, SpoilerSpan, HardBreak) lifted to substrate
vocabulary. Each node carries `source_position` so the reader's
interaction can byte-locate what it references.

```
type gestalt_node = {
  kind:            node_kind,           # variant per span-kind above
  content:         oid,                 # content-addressed body
  source_position: source_position,     # @glass byte-slice
  visibility:      visibility,          # per-node scope (allows @sheaf
                                         # section-at-stalk restriction)
}

type gestalt_edge = {
  from:            oid,                 # source node's content-hash
  to:              oid,                 # target node's content-hash
  kind:            edge_kind,           # reference | fold | annotation |
                                         # ancestor | reader-lens
}
```

### §2.3 `mode`

Render targets the document admits. Not all modes must be admitted;
authors choose which linearization surfaces they permit. Each mode
carries its L(ϕ) profile per pipeforward §5.5.4 rule 3.

```
type mode = | markdown | html | latex | slides | pdf | epub | tui
```

L(ϕ) per mode: see §8 Alex-adjudicable render-target table.

### §2.4 `unfolding_state`

The reader-site runtime state as the @song unfolds. This carrier
IS what makes reader-interaction the compiler's runtime: the
substrate-decl records the reader's beat-sequence through the
document, and annotation contributions extend the sequence
monotonically (see math §3 Merkle-DAG accumulation).

```
type unfolding_state = {
  document:         gestalt_document,
  reader:           subject_instance,
  reader_visibility: visibility_scope, # what THIS reader may see
  focus_node:       oid,               # current node under attention
  beat_history:     [signature_beat],  # sequence of interactions,
                                        # per @spectral/signature model
  local_snapshot:   snapshot,          # @time snapshot at unfolding-
                                        # start; @time.compare gives
                                        # the delta at any moment
  annotation_draft: option<annotation>, # in-flight reader contribution
}
```

Composition-only: `signature_beat` is the same carrier
`shards/spectral/signature.mirror` uses; the reader's beat-history
is a rolling_signature over the reader's engagement corpus, not the
author's DAG.

### §2.5 `annotation`

The reader-side contribution unit. Content-addressed; monotone
growth; visibility-scoped per @sheaf discipline (readers see the
section at their stalk).

```
type annotation = {
  contributor:      subject_instance,   # the reader (any actor_kind)
  target_node:      oid,                # what is being annotated
  content:          oid,                # nl_literal or ref
  visibility_scope: visibility_scope,   # reader's chosen scope
  previous:         option<oid>,        # Merkle-DAG chain to prior
                                         # annotations at this node
  timestamp:        @time.instant,
  ssh_fingerprint:  ref,                # per @spectral/signature model
}
```

---

## §3 Action surface (partitioned per pipeforward §5.5.4)

The pipeforward hard gate (ratified Alex 2026-07-15 at `7181f5c`
in `docs/specs/autopoietic-inference-loop.md` §5.5.4) is load-bearing
on every action below. Rule 1: designed for socket-forwarded
composition first. Rule 2: return a nonlinear ref by default. Rule
3: @io discharge quantifies L(ϕ). Rule 4: `--collapse` escape hatch
is explicit, never silent.

### §3.1 Socket-shaped actions (stay nonlinear across the pipe)

Each returns a substrate-typed value that can compose over
`~/.mirror/serve.sock` without linearization.

```
unfold_at_reader_site(
  doc:      gestalt_document,
  reader:   subject_instance,
  at:       @time.instant,
) -> unfolding_state
```
Constructs the reader-site runtime state. NO @io crossing: the
substrate returns a nonlinear ref; the reader's @peer runs the
unfolding locally. Composes with `@peer/persistence` for annotation-
corpus home-repo binding.

```
traverse(
  state:    unfolding_state,
  next:     oid,
) -> unfolding_state
```
Advance the reader's `focus_node` to `next`. Emits one
`signature_beat` in the reader's beat_history. Composition-only
over @song/beat's `strike` at reader-site altitude.

```
focus(
  state:    unfolding_state,
  node:     oid,
) -> unfolding_state
```
Zoom the reader's attention onto one node without traversal. Emits a
`hold`-beat (per @song/beat vocabulary): temporal advance without
firing an action against the graph.

```
compose_modes(
  doc:      gestalt_document,
  modes:    [mode],
) -> gestalt_document
```
Declare which render targets the document admits. Nonlinear:
returns the augmented document ref. No @io crossing until §3.2's
`render` fires.

```
project(
  doc:      gestalt_document,
  viewer:   subject_instance,
) -> gestalt_document
```
Return the sub-document the viewer's ACL admits, via @sheaf's
`filter(bauchladen, visibility, viewer)` section-at-stalk. Nonlinear:
returns the projected document as ref. Composes @subject/visibility
+ @subject/visibility/sheaf for the restriction discipline.

```
annotate(
  state:    unfolding_state,
  content:  ref,
  vs:       visibility_scope,
) -> unfolding_state
```
Reader contributes an annotation to the current focus_node. The
annotation is content-addressed and Merkle-DAG-chained per §2.5.
Nonlinear: returns the extended unfolding_state; annotation is not
committed to @io/git until `settle_annotation` (below) fires as a
`--collapse`-authorized @io discharge OR as an authorized
socket-forwarded @kintsugi/store/git compose.

### §3.2 @io discharge action (linearization; L(ϕ) declared)

Every render is an @io crossing per pipeforward §5.5.4 rule 3. The
substrate MUST NOT render silently; render happens iff (a) the
consumer is not another mirror process, OR (b) the caller passes
`--collapse` at the CLI surface.

```
render(
  doc:      gestalt_document,
  target:   mode,
  out:      @io/fs.path,
) -> imperfect<@io/fs.written_bytes>
```

L(ϕ) profile per target — see §8 table for the quantitative shape
each mode discards.

The `imperfect<>` carrier is per @glass discipline: `Pass` iff the
render round-trips under the mode's own decoder; `Partial(loss)`
when the mode discards structural information (e.g., html discards
the beat-history's temporal ordering); `Fail` when a required
carrier is missing (e.g., PDF render without a `mode = pdf` in
`doc.modes`).

---

## §4 The unfolding law

The mathematics of "unfolds on the @subject's device through
interaction" is worked in `docs/math/gestalt/README.md` (Deliverable
2 companion). The substrate-decl statement:

> **@gestalt law of unfolding.** A `gestalt_document` `d` unfolds
> as a beat-sequence in the reader's `unfolding_state` `s`. Each
> reader-interaction `i_t` extends the beat_history by one beat;
> the beat is emitted by @song's landed `strike`/`hold` at
> `@song/beat` species altitude; the sequence is content-addressed
> by @spectral/signature's rolling_signature; category-formation
> happens at read-time because reader-interaction is the
> morphism-generator for the document's DAG-growth.

Formalized in math §1 (functor / co-algebra shape), math §2
(interaction as time-indexed morphism family per
`@epistemologic/reality/time.compare`), math §3 (Merkle-DAG
accumulation), and math §6 (recursion coupling — the "reader is
inside the operator" claim from Reed's vignette).

The substrate-decl carriers `unfolding_state` + `signature_beat`
and the action `traverse`/`focus`/`annotate` are the primitives the
theorem quantifies over.

---

## §5 Category-formation at read-time

The composition graph the vignette described:

```
reader opens spectral.engineer URL
    ↓
reader's @peer runs locally (constitutive; no server compute)
    ↓
@gestalt.unfold_at_reader_site(doc, reader, now)
    → unfolding_state u₀
    ↓
reader interacts (traverse | focus | annotate) → u₁, u₂, ..., uₙ
    ↓
each interaction extends beat_history (monotone; per math §3)
    ↓
annotations accumulate content-addressed
    (per @mirror/store CAS; per @spectral/signature rolling-signature model)
    ↓
gestalt_document (as @song) GROWS via reader-corpus co-writing
    (per @subject/visibility sheaf-restriction: readers see the
     section at their stalk; consent_scope elevation via
     @kintsugi/consent.query_phi)
```

**Reader-corpus co-writing at content-addressed identity-provenance.**
Every annotation carries `contributor: subject_instance` + `ssh_
fingerprint` + `previous: option<oid>` so the reader-corpus is
Merkle-DAG-verifiable at any @subject's stalk. This is the
substrate-decl form of "the category grows via the exact substrate
that resists it" — resistance-to-extraction lives in the @sheaf
sections; the co-writing lives in the DAG growth.

**Foerster-legal at maximum recursion.** Reader-interaction as
morphism-generator IS Foerster's ethical imperative operationalized:
every reader increases the number of choices (annotation options)
for every subsequent reader. Formalized in math §6.

---

## §6 Pipeforward alignment

Every action in §3 either stays socket-shaped or discharges via @io
with declared L(ϕ). The three composition modes per
`docs/specs/autopoietic-inference-loop.md` §5.5.2:

- **Mode A (all-nonlinear reader-site chain):** `unfold_at_reader_site
  |> traverse |> focus |> annotate` — the reader's entire session
  stays in nonlinear substrate; zero @io crossings until settle.
- **Mode B (mirror socket-forwarded across peers):** the reader's
  @peer forwards annotation contributions via
  `~/.mirror/serve.sock` to the author's @peer or to a downstream
  reader's @peer; content-addressed refs cross the socket, not
  linearized bytes; single L(ϕ) on final settle. Discharges
  Recognition #R-mirror-compiler-is-operationally-closed at reader-
  altitude.
- **Mode C (`--collapse` at CLI):** `mirror gestalt render doc.gst
  --target html --collapse=./out.html` — caller explicitly authorizes
  the @io crossing. The L(ϕ) crystal is logged per rule 3.

Multi-render batches (e.g., `mirror gestalt render doc.gst --target
html,pdf,slides`) SHOULD amortize the traversal cost: one
unfolding_state, three linearizations.

---

## §7 Forward-promises

The following are declared at spec-authoring altitude but await
downstream landings:

- **`@io/socket` species-decl mint.** Per pipeforward §5.5.4
  Reed-inline-4 note: `boot/std/io/socket.mirror` FLOOR landed;
  `shards/io/socket.mirror` species-decl NOT YET LANDED. Mode B
  reader-to-reader socket-forwarding requires this landing.
- **`@subject/visibility/sheaf.filter` empirical discharge.** The
  per-reader section-at-stalk projection substrate-decl is landed
  (Landing 4 R2 `d1ce901`); the empirical dispatch through
  `apply_h::act` awaits Arc-1 evaluator FLOOR (Tick 1.1-1.4).
- **`render` per-mode L(ϕ) empirical measurements.** §8 gives
  ordering + rationale; empirical L(ϕ) values await
  implementation ticks (Alex-adjudicable at §10 R3).
- **@peer optical inference for annotation-tournament dispatch.**
  `@fate/tournament` composition target named in §1; empirical
  discharge awaits @fate optical-inference landings (Recognition
  #58 on-device D²NN forward-promised).

---

## §8 Alex-adjudicable render-target table (v0.1.0 residue)

Per math §5 L(ϕ) holonomy loss per render target. Rough ordering
below; refine at empirical measurement time.

| target   | L(ϕ) magnitude | what is discarded                                                      |
|----------|----------------|------------------------------------------------------------------------|
| markdown | lowest         | interactivity (annotations become inert links to nowhere)              |
| html     | low            | temporal ordering of beat_history (renders a snapshot, not a session)  |
| tui      | low            | color/font (terminal fidelity constraint) but retains reader-interaction|
| latex    | medium         | annotation Merkle-DAG (LaTeX has no content-address vocabulary)        |
| slides   | medium-high    | non-linear DAG structure (slides linearize traversal order)            |
| epub     | high           | reader-corpus co-writing (epub is publisher-final; no annotation surface)|
| pdf      | highest        | everything above; PDF is terminal linearization + typography-final     |

**v0.1.0 recommendation (Alex adjudicates):** ship with markdown +
html render targets. tui + latex + slides + epub + pdf deferred to
subsequent ticks (post-v0.1.0). Rationale: markdown preserves
substrate-authoring roundtrip (per `spectral/crates/gestalt/encode`
pulldown-cmark prior art); html preserves reader-interaction at
lowest L(ϕ) for non-mirror-consuming readers on the web (Reed
vignette site: spectral.engineer).

---

## §9 v0.1.0 render targets and scope

- Carrier mints in `shards/gestalt.mirror` (Deliverable 3): all §2
  types, all §3 actions with `\` obligation bodies.
- `render` bodies for `markdown` and `html` targets forward-promised
  via `[substrate-floor:@io-boundary]` marker when the reader-site
  runtime lands (post-Arc-1 evaluator FLOOR).
- Reader-interaction dispatch (traverse / focus / annotate) discharges
  through `apply_h::act` per Arc-2 evaluator pattern.
- `@fate` tournament dispatch for annotation-surface ordering:
  forward-promised (Alex-adjudicable at §10 R4).

---

## §10 Alex-adjudicable residues

R1. **@gestalt family placement.** Options: (a) top-level family-
    root `shards/gestalt.mirror`; (b) species under @song
    (`shards/song/gestalt.mirror`); (c) species under @subject
    (`shards/subject/gestalt.mirror` — reads "the reader IS a
    subject, gestalt IS a species of subject-facing @song").
    Mara-recommendation: **(b) species under @song** — the reframe
    verbatim says "a @gestalt document IS a @song"; @song is the
    already-landed family-root; species-under-@song matches
    @spectral/signature precedent (Arc-2 Tick 2.1) which lifted
    @song at a different consumer-altitude. Two-tick discipline:
    if a second consumer emerges at non-@song altitude (e.g., a
    @subject-facing UI that isn't a @song), promote to family-root
    then. Not now.

R2. **v0.1.0 scope.** Options: (a) markdown + html only (Mara-
    recommendation, §8); (b) markdown + html + tui (adds terminal
    surface but requires `gestalt-tui/` Gleam prior-art lift); (c)
    all seven targets (would blow the v0.1.0 scope; DEFER).

R3. **Empirical L(ϕ) measurement approach.** Options: (a) per-target
    round-trip discharger + Fiedler-delta measurement (composes
    @mirror/index); (b) qualitative-only per §8 table (defer
    quantitative measurement to post-v0.1.0). Mara-recommendation:
    **(a)** — the substrate already has @mirror/index landed
    (Landing 1 `317e830`) and the round-trip discharge template
    exists at `@shatter` linearization spec (§4.3 monotone descent
    preservation).

R4. **Annotation-tournament dispatch (recursive-document settlement
    shape).** When N readers annotate the same node with
    conflicting visibility_scopes, which annotation surfaces first
    for reader N+1? Options: (a) @fate tournament by
    reader-corpus-affinity (reader N+1 sees annotations from
    subjects their @peer/persistence corpus resonates with — the
    "reader recursion couples with paper recursion" vignette
    formalization); (b) chronological monotone (simplest; matches
    @spectral/signature ordering); (c) reader-selects (annotation-
    lens toggle per Reed vignette Kai-scenario). Mara-recommendation:
    **(c) reader-selects** as v0.1.0 default; **(a) @fate tournament**
    forward-promised for post-v0.1.0.

R5. **Recursive-document settlement shape.** When a reader annotates
    an annotation, does the graph grow at the original node or fork
    into a new document? Options: (a) grow at original (annotations
    form a sub-DAG under target_node); (b) fork (each annotation
    thread becomes a new gestalt_document with the parent as
    ancestor). Mara-recommendation: **(a) grow at original** —
    matches @mirror/store CAS composition; matches
    @spectral/signature Merkle-DAG shape; matches Reed's vignette
    "annotations toggle on/off" (not "annotations spawn documents").

R6. **Vocabulary residue: `unfold_at_reader_site` naming.** Options:
    (a) `unfold_at_reader_site` (current spec — verbose, self-
    documenting); (b) `unfold` (delightfully-boring; the reader-site
    is implicit because @gestalt IS reader-site); (c) `open`
    (matches user vocabulary "open a document"). Mara-recommendation:
    **(b) `unfold`** — two-tick discipline: readable name over
    foundational. The reader-site is structural; naming it in the
    action-verb is redundant.

---

## §11 Substrate-honesty attestation

- Every family-root cited in §1 composition graph is LANDED.
- Every species-decl carrier cited in §2 is LANDED.
- Zero Rust extension shortcuts. All action bodies are `\` obligation
  bodies that dispatch through `apply_h::act` per Arc-2 evaluator
  pattern per Mara-B §6.2 recognition candidate.
- Pipeforward §5.5.4 hard gate satisfied: §3.1 socket-shaped surface
  is the default; §3.2 render is the only @io discharge; `--collapse`
  is the explicit escape hatch.
- Substrate-already-had-the-word verified: @song (family-root landed);
  @subject / @subject/visibility / @subject/visibility/sheaf
  (landed); @spectral/signature (species landed); @peer/persistence
  (species landed); @glass source_position (Bridge α landed
  2026-07-15); @mirror/store six-op CAS (landed); @io/fs (landed);
  @io/git (landed).
- The word "workaround" appears nowhere in this spec.

---

## §12 References

- `shards/song.mirror` (Arc 6 TICK 1 landing `f01cf9f`; family-root)
- `shards/song/beat.mirror` (Rung 0 landing `94e55eb`; atomic-execution
  species)
- `shards/spectral/signature.mirror` (Arc-2 Tick 2.1 landing
  `f211ee48`; rolling-signature precedent for beat-sequence-as-song)
- `shards/subject.mirror` (Landing 3 landing; subject_instance carrier)
- `shards/subject/visibility.mirror` (Landing 5 landing;
  visibility_scope carrier)
- `shards/subject/visibility/sheaf.mirror` (Landing 4 R2 `d1ce901`;
  sheaf-restriction species)
- `shards/peer/persistence.mirror` (Arc-2 Tick 2.3 landing;
  peer-home-projection species)
- `shards/glass.mirror` (Bridge α source_position landing 2026-07-15)
- `shards/mirror/store.mirror` (@mirror/store CAS family-root)
- `shards/io/fs.mirror` (@io/fs family-root)
- `shards/io/git.mirror` (@io/git family-root)
- `shards/epistemologic/reality/time.mirror` (@time.compare over
  snapshots for interaction-time delta — `compare(a: snapshot, b:
  snapshot) -> delta` verified landed as exported action at
  `shards/epistemologic/reality/time.mirror:151` + `out compare` at
  line 200, per Seam Phase D adjudication of this landing)
- `docs/specs/autopoietic-inference-loop.md` §5.5.4 pipeforward hard
  gate (Alex ratification `7181f5c` 2026-07-15)
- `docs/specs/shatter-is-the-io-linearization-operator.md` (Mara
  2026-07-08; §4.3 monotone descent preservation across linearization)
- `docs/specs/subject-visibility-sheaf.md` (Mara `564571e`; @sheaf
  canonical spec)
- `docs/specs/gift-and-mirror-reflection.md` §11-§24 (Landing 3
  subject_instance + Landing 5 A24 historical_witness)
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara-B
  Arc-2 Tick 2.1 ouroboros_monotone template)
- `docs/scouts/2026-07-15-taut-nix-iconv-root-cause-not-decay.md`
  (Taut root-cause discipline referenced per Mara brief)
- Reed's 2026-07-15 spectral.engineer launch vignettes (in-transcript
  attribution; load-bearing "why this substrate" reference for §5)
- Sibling prior-art projects: `/Users/alexwolf/dev/projects/gestalt-
  gradient/` (Rust POC), `/Users/alexwolf/dev/projects/gestalt-mirror/`
  (schema + /protected + /public partitions),
  `/Users/alexwolf/dev/projects/gestalt-tui/` (Gleam TUI),
  `/Users/alexwolf/dev/projects/gestalt-ui/` (Gleam web surface),
  `/Users/alexwolf/dev/projects/spectral/crates/gestalt/` (13
  Rust modules; document / domain / encode / dom / panel / token
  / form / spectral / detect / graph / semantic / mirror_domain)
- Companion math: `docs/math/gestalt/README.md` (Deliverable 2)
- Companion substrate-decl: `shards/gestalt.mirror` (Deliverable 3)
