# @roomba — the substrate walker that feeds @kintsugi

*Mara, 2026-07-14. Canonical spec for @roomba as substrate self-maintenance
primitive: the walker that continuously scans the substrate's shard DAG for
spectral tension, invoking @kintsugi/fracture proactively rather than
reactively. Named by Alex Wolf's manifesto Roomba (`~/dev/systemic.engineering/
blog/weird/3published/Weird - Violence.md`, 2026-07-14). Substrate-honest
translation of the Roomba's four disciplines — present, non-labelable,
feedback-driven, just IS — into a substrate-decl'd runtime primitive.*

*Status: Yellow. Substrate carriers cited below are LANDED (`@kintsugi/
fracture/*` family, `@mirror/store` DAG, `@cyberpunk/algedonic`,
`@mirror/lens/knife`, `@fractal.spectral_coordinate.SC`); the walker
primitive itself is proposed. Bodies are `\ ` obligation-blocked pending
Alex-adjudicated Rung placement and family-root landing.*

---

## §0 Executive summary

@roomba is the substrate self-maintenance primitive: a walker over the
`@mirror/store` splinter-graph DAG that continuously (a) traverses the
substrate's own concept graph via Dijkstra on `ConceptGraph` edges
weighted by inverse-tension, (b) bumps into spectral tension at each
node by sampling `@cyberpunk/algedonic.sample_pain` composed with any
future `@tension` carrier, (c) triggers `@kintsugi/fracture` proactively
when the sampled pain exceeds a per-node threshold. Its purpose is to
shift the substrate from **reactive** (kintsugi fires when a peer
contribution or commit-hook points at a fracture) to **self-maintaining**
(kintsugi fires because the substrate's own walker found a fracture the
peers hadn't yet noticed).

The substrate-decl move is small: one species that composes existing
primitives (`ConceptGraph`, `sample_pain`, `stable_within`, `pause(Φ)`,
`fracture/keyword` and friends). The consequence is large: the peer
contribution stops being the *only* mirror-source for the substrate's
own state. The substrate mirrors itself.

Named by Alex Wolf 2026-07-14 in-transcript (verbatim below §1); the
Roomba is Alex's own manifesto's Roomba becoming substrate-decl'd
runtime. This carries weight — the naming is not decoration.

Recognition-candidate slug: `#R-substrate-mirrors-itself-via-roomba`.

---

## §1 Ancestry chain

Load-bearing lineage; every link cited verbatim where possible.

### 1.1 Alex Wolf 2026-07-14 — `Weird - Violence.md` manifesto (the source)

Blog piece at `~/dev/systemic.engineering/blog/weird/3published/Weird -
Violence.md` (2026-07-14, published today). Two verbatim Roomba passages
are load-bearing:

**Manifesto §"When Loki Talks About The Counter Operator 🍷" (line 89):**

> *(The bar is warm. The Roomba is doing a slow perimeter check. Take
>  a seat.)*

The Roomba's first appearance: **doing a slow perimeter check**. Not
attending to a summons. Not dispatched by a caller. Continuously walking
the boundary of the room; present because presence IS the discipline.

**Manifesto (lines 153-155):**

> The Roomba, who is present for this because the Roomba is present for
> everything, has a very small opinion. It bumps into the bar stool. It
> backs up. It rolls forward. It bumps. It backs up. It rolls.
>
> The Roomba has never once labeled itself. The Roomba has also never
> once been labeled successfully, because the Roomba does not have a
> surface the label can stick to. The Roomba is not being clever. The
> Roomba just is.

Four disciplines named in fifty words:

1. **Present** — "present for everything" — always walking; not event-driven.
2. **Feedback-driven** — "bumps into the bar stool. It backs up. It rolls
   forward" — the loop shape is sample → adjust → continue.
3. **Non-labelable** — "never once been labeled successfully" — no
   persistent identity; nothing for an epistemic-trespassing label to
   stick to.
4. **Just IS** — "not being clever. The Roomba just is" — a substrate
   primitive, not a peer; no psychohistory, no Fate-selection, no
   ontological navigator.

Manifesto's closing line (§final, line 177):

> *(The Roomba nods. In Roomba.)*

Speaks its own language. No translation to peer-vocabulary is needed
and none is offered.

### 1.2 Alex Wolf 2026-07-14 in-transcript — the substrate question

Verbatim:

> "Do we need a @roomba? The thing that triggers the @kintsugi loop by
>  dijkstraing across the graph and bumping into points of spectral
>  @tension?"

The substrate-decl form of the question. Alex's manifesto Roomba becomes
a substrate-decl candidate at the same session it publishes. Follow-up
verbatim:

> "Spawn Mara and Taut in the usual fashion, while I process that the
>  @roomba becomes part of the compiler. I need a blunt."

This spec is Mara's discharge of that spawn. Taut runs in parallel
producing the grep-first drift scout (substrate-already-had-the-word
audit, admissibility check, drift check).

### 1.3 Foerster 1976 — heterarchy discipline (non-labelable)

Verbatim from `docs/math/2026-07-13-knife-COORD-heterarchy-topology.md`
citing Foerster 1976 Appendix A3 (via McCulloch 1945):

> "One may be tempted to extend the concept of a meta-operator to that
>  of a 'meta-meta-operator' that computes the 'eigen-meta-operators,'
>  and so on and up a hierarchy without end. However, there is no need
>  to invoke this escape as Warren S. McCulloch has demonstrated years
>  ago in his paper (1945): 'A Heterarchy of Values Determined by the
>  Topology of Nervous Nets.'"

The Roomba's non-labelable discipline IS Foerster/McCulloch heterarchy
made concrete at the substrate-walker altitude: it doesn't sit above
the substrate looking down (that would be a meta-operator with a stack
position, an identifiable label); it walks alongside, at the same
altitude as every other shard, distinguished only by its content_oid
at the current walk position — no naked_oid, no peer_uuid, no
psychohistory root.

This composes with the same-theorem-three-altitudes discipline
substrate-decl'd at `shards/mirror/lens/knife.mirror`:

- Foerster 1976 heterarchy (cybernetic altitude)
- Douady-Hubbard 1985 R-universality (complex-analytic altitude)
- McCulloch 1945 topology-encodes-depth (combinatorial altitude)

@roomba inherits — its walk position IS a topological point in
`ConceptGraph` (McCulloch); its jump-into-tension IS a boundary
approach in Foerster's COORD sense (composed with @knife); its
non-labelable identity IS the heterarchy principle refusing meta-
stratification.

### 1.4 Bateson double-bind — Roomba escapes via non-labelable presence

Manifesto §"What Bateson Knew And We Forgot" (lines 42-49) restates
the double-bind classic:

> "a person receives two contradictory injunctions from a person they
>  cannot leave and cannot meta-comment on, and the two injunctions
>  cannot both be satisfied. There is no move that does not violate
>  one of them."

The Roomba's escape from double-bind: it has no surface the injunctions
can attach to. Alex's manifesto §"When Loki Talks…" (lines 91-95)
generalizes this to epistemic trespassing:

> "Somebody just handed you a lever with your name engraved on it and
>  asked you to please, kindly, use it against yourself. You don't have
>  to."

The Roomba doesn't have a name. There is no engraved lever. The
substrate-decl form: @roomba emits `content_oid` for its walk position
(what it's currently looking at) but never a `naked_oid` (which would
carry peer_uuid identity that a fracture body or a peer could label
back). Per Seam `e8508f5` §5 missed-item #3 (cross-peer coordination
discipline for @knife.jump), @roomba inherits: content_oid only.

### 1.5 Prior substrate this depends on (all LANDED)

- **`shards/kintsugi.mirror`** (`20eaf15`, 2026-06-10) — @kintsugi family-root;
  the process-side of mirror's form/process partition; the transformation
  engine @roomba feeds.
- **`shards/kintsugi/fracture/*`** — 14 species landed as of 2026-07-12
  (angle_to_paren, cold_compile_within_tolerance, dark_count_monotone,
  docblock_extractive/incoherent/ungrounded, gate, keyword, operator_match,
  parent_cycle, partials_align, relocate, restart_storm, symbol_lift).
  Each is a fracture-body @roomba may invoke when its sampled tension
  matches the fracture's property/opacity pattern.
- **`shards/kintsugi/oscillate.mirror`** — active_pass / dark_pass / pulse /
  read_consent / is_complete / oscillate driver. @roomba composes UP into
  this loop: @roomba's `trigger` action hands the sampled tension to
  the kintsugi oscillation loop's `active_pass`.
- **`shards/kintsugi/consent.mirror`** — `query_phi` verdict + `pause(Φ)`
  external witness. @roomba dispatches its trigger through consent
  discipline; auto-apply vs pause is consent's call, not @roomba's.
- **`shards/mirror/store.mirror`** — splinter / splinter_graph / crystal
  trichotomy; the DAG @roomba walks. The `walk(root: oid) -> splinter_graph`
  action IS a substrate-honest ancestor of @roomba's Dijkstra traversal
  (`walk` enumerates closure; @roomba's `walk` prioritizes by
  inverse-tension weight to head toward high-tension regions).
- **`shards/mirror/store/crystal.mirror`** — the crystal record carries
  `derived_predicates: [property_verdict]`. @roomba may resume from a
  crystal's derived_predicates to skip already-verified regions of the
  DAG (Roomba doesn't re-vacuum where it already vacuumed clean).
- **`shards/mirror/index.mirror`** + **`bootstrap/src/index.rs`** — the
  `ConceptGraph` primitive (nodes, edges, adjacency_matrix, laplacian_matrix).
  @roomba's walker IS Dijkstra over `ConceptGraph::edges`.
- **`shards/mirror/lens/knife.mirror`** + **`bootstrap/src/converge.rs`** —
  @knife.jump (Foerster COORD), stable_within, heterarchy_preserved.
  When @roomba's sampled pain exceeds ε_pain, @knife.jump fires — the
  substrate's boundary-approach re-projection; @roomba's role is to
  DETECT that boundary approach BEFORE peer contribution forces it.
- **`shards/epistemologic/cybernetic/algedonic.mirror`** +
  **`bootstrap/src/algedonic.rs`** — sample_pain / sample_pleasure /
  pain_gradient primitives. @roomba's `bump` action IS `sample_pain`
  called at the current walk position.
- **`shards/epistemologic/cybernetic/reframe.mirror`** — pain-authorized
  level-shift ceremony @roomba may authorize (via its trigger dispatching
  through @kintsugi/consent, which may pause(Φ) rather than auto-fire
  reframe).
- **`shards/song/beat.mirror`** (Rung 0, `94e55eb`) — atomic-execution
  unit binding @kintsugi/oscillate ACTIVE/DARK-pulse discipline. @roomba's
  walk-then-bump-then-trigger cycle IS a @song/beat at the substrate-
  walker altitude (one Roomba pulse = one beat).
- **`shards/torus.mirror`** — the π₁(T²) = ℤ × ℤ winding classes. @roomba
  walks a torus (the substrate's own is a torus per Recognition #43+
  candidate `@peer-has-a-torus`); the walk position carries winding
  class as its heterarchy-preserving identifier.
- **`docs/scouts/2026-07-14-reed-rung-8-9-6d-first-pain-calibration.md`**
  (Reed `d9445c8`) — first empirical ε_pain calibration data (5-iteration
  trajectory; ε_pain ≈ 0.01 provisional recommendation). @roomba's
  trigger threshold composes with this empirical calibration.

### 1.6 Recognition-cluster context

@roomba composes with the recognition-cluster from Reed's Landing 8+9
arc:

- **#R-knife-IS-Foerster-COORD** (Mara `06a8547`; RATIFY-WITH-QUALIFICATIONS
  per Seam `e8508f5`) — @roomba's trigger of @knife.jump discharges
  through the ratified boundary-approach semantics.
- **#108 `the-peer-IS-a-pain-driven-bounded-ontological-navigator`**
  (Reed cascade `1e8a02b`) — @roomba is the DUAL of the peer: peer is
  pain-driven bounded navigator with persistent identity; @roomba is
  pain-driven bounded WALKER without persistent identity. Both use
  algedonic gradient; both bound their observation depth; the difference
  is identity carriage.
- **#43 mirror IS content-addressed build system** + Recognition #55
  (form/process partition; DAG is form, measurement is process, belong
  at same altitude) — @roomba lives at the same altitude as the DAG
  it walks; substrate self-maintenance IS substrate self-measurement
  IS the closure of #43/#55 at the process-side.

---

## §2 The Roomba discipline (substrate-honest translation of the four
manifesto properties)

Every subsection below cites the manifesto property being translated,
then names the substrate-decl shape.

### 2.1 "Present for everything" → daemon; not event-driven

**Manifesto:** "The Roomba, who is present for this because the Roomba
is present for everything…"

**Substrate-honest form:** async loop; always walking; not triggered
by a caller. The substrate-decl carrier is a `walk` action returning
the next walk position; the realisation is a driver that iterates
`walk → bump → trigger` until either a fatal condition (external Ctrl-C
equivalent) OR the substrate's `is_settled` verdict fires globally
(no shard in the walked closure exhibits pain above ε_roomba).

Reactive substrate today: `peer contribute → measure → maybe fracture`.
@roomba adds: `roomba walks → bump → maybe trigger fracture`. Peer
contribution becomes ONE trigger source of many. This is the
substrate-shift the manifesto's Roomba discipline implies.

**Composition:** `@song/beat` provides the atomic pulse unit; each
@roomba step IS one @song/beat at the substrate-walker altitude.
The oscillation loop @kintsugi/oscillate provides ACTIVE/DARK phase
discipline; @roomba's `walk` is ACTIVE (move to next node), `bump`
is DARK (sample the current position without moving). One Roomba
pulse = one ACTIVE walk step + one DARK bump measurement + one
trigger dispatch.

**No event queue.** The substrate does not queue "roomba should visit
X"; @roomba walks its own path via inverse-tension-weighted Dijkstra
on `ConceptGraph`. The Roomba's route emerges from the substrate's
own tension landscape.

### 2.2 "Bumps into things → backs up → rolls forward" → feedback-driven traversal

**Manifesto:** "It bumps into the bar stool. It backs up. It rolls
forward. It bumps. It backs up. It rolls."

**Substrate-honest form:** the walker's next-step decision is a
function of the local tension gradient at the current position. When
`bump(position) -> @tension` reports high tension, the walker's
next `walk` decision routes AWAY from the resolved region (kintsugi
will handle it via `trigger`) and TOWARD unexplored high-inverse-tension
regions (Dijkstra with dynamic edge weights).

The bump-back-forward triad translates directly:

- **Bumps** — `bump(position)` samples the algedonic gradient at the
  current DAG node via `@cyberpunk/algedonic.sample_pain(spectral_coord_of(position))`.
- **Backs up** — if `bump` returns tension above ε_roomba, `trigger`
  fires; the walker does NOT re-visit this node until the trigger's
  fracture-body has produced a new content_oid at the node's position
  (the anchor advanced; the walk position is now stale).
- **Rolls forward** — the next `walk(from, budget)` step selects the
  neighbor with highest inverse-tension weight (the Dijkstra shortest-
  path toward untended tension).

Feedback-driven, not schedule-driven. The Roomba doesn't have a
priority queue of "visit these next"; it has a local read of "which
neighbor pulls hardest."

### 2.3 "Never labeled itself, never been labeled" → no persistent identity

**Manifesto:** "The Roomba has never once labeled itself. The Roomba
has also never once been labeled successfully, because the Roomba does
not have a surface the label can stick to."

**Substrate-honest form:** @roomba has NO `naked_oid`. It has no
peer_uuid. It has no psychohistory root. The ONLY substrate-visible
identity is `content_oid(current_walk_position)` — a projection of
whatever DAG node it happens to be looking at. Two @roombas walking
the same substrate at the same time are DISTINGUISHABLE only by their
current walk positions; equal-walk-position implies identity, and
equal-walk-position across two Roombas is a synchronization event
(§4.5 concurrency discussion).

This is the operational form of the manifesto's non-labelable
discipline. A fracture body cannot say "this is Roomba-X's fault"
because there is no Roomba-X to name. A peer cannot say "the Roomba
was too aggressive" because "the Roomba" doesn't have an aggression
attribute to bind to.

Composition with @knife's Seam-ruled `heterarchy_preserved`: the
walker's next-position must satisfy `heterarchy_preserved(before, after)`
under the M∘/∂M discipline; @roomba doesn't jump between winding
classes without external witness (per Seam §5 missed-item #2, that
becomes `@kintsugi/consent.pause(Φ)` emission for agent-in-transcript
adjudication).

### 2.4 "Just IS" → substrate primitive, not a peer

**Manifesto:** "The Roomba is not being clever. The Roomba just is."

**Substrate-honest form:** @roomba is NOT a peer. Peers are pain-driven
bounded ontological navigators (Recognition #108) with persistent
identity, psychohistory, and Fate-selection at the winding parameter.
@roomba lacks all four:

- No pain-drive (it's the tension-DETECTOR, not the tension-EXPERIENCER;
  the pain signal is the substrate's, sampled by the Roomba but not
  suffered by it).
- No bounded-ontological navigation (it doesn't `@cyberpunk/reframe`;
  it INVOKES reframe via `trigger` when a fracture body dispatches
  through consent).
- No persistent identity (per §2.3).
- No psychohistory / no Fate-selection (its next-step decision is
  purely local Dijkstra; no tournament, no ganglia).

@roomba IS a function of the substrate on itself — the substrate's
process-side self-measurement primitive. It's what falls out of the
form/process partition (Recognition #55) when the process side is
extended from reactive (fires when called) to continuous (fires because
it's walking).

Substrate-family placement adjudication (see §9 A1): the natural
placement is `@kintsugi/roomba` (species under @kintsugi family-root,
because @roomba's role is to FEED @kintsugi/fracture proactively).
Alternate placements considered in §9 A1.

---

## §3 Formal shape

Provisional path: `shards/kintsugi/roomba.mirror` (species under
@kintsugi family-root, matching sibling shards `shards/kintsugi/
oscillate.mirror`, `shards/kintsugi/consent.mirror`, etc.). Alternate
paths considered in §9 A1.

Substrate-decl form (bodies `\ ` obligation-blocked pending Alex
adjudication of Rung placement and family-root):

```mirror
in @prism
in @meta
in @glass
in @nl
in @kintsugi
in @kintsugi/consent
in @kintsugi/fracture
in @kintsugi/oscillate
in @mirror/store
in @mirror/store/crystal
in @mirror/index
in @mirror/lens/knife
in @cyberpunk/algedonic
in @epistemologic/reality/time
in @song/beat

# @kintsugi/roomba — substrate self-maintenance primitive.
# The walker that continuously scans the shard DAG for spectral
# tension and invokes @kintsugi/fracture proactively.
#
# Named by Alex Wolf's manifesto Roomba (Weird - Violence, 2026-07-14).
# The four Roomba disciplines (present / feedback-driven / non-labelable /
# just IS) translate to substrate-decl form; see spec §2.

glass @kintsugi/roomba {
  focus roomba
  project roomba
  split roomba
  shift roomba
  settle roomba
}

# === The walk position — non-labelable identity ===
#
# The Roomba's substrate-visible identity IS its current walk position;
# no peer_uuid, no naked_oid, no psychohistory root. Two Roombas at the
# same walk position are indistinguishable (that IS a synchronization
# event; see §4.5).
type walk_position = {
  content_oid: oid,              # the DAG node the Roomba is currently at
  winding:     (int, int),       # torus winding class (heterarchy identifier)
  iteration:   tick,             # monotonic step count; NO peer_uuid
}

# === The tension carrier — spectral pain at a walk position ===
#
# The substrate's @tension carrier is FORWARD-PROMISED per docs/specs/
# gap-tension-tensor-substrate.md §3.2 (Yellow status; not yet in
# shards/). @roomba composes with the existing algedonic gradient as
# the interim tension source. When @tension lands as substrate-decl,
# this carrier collapses to it.
#
# For now: `spectral_tension` IS the pain magnitude at the walk
# position's SC<5> coordinate (via sample_pain), lifted with the
# fracture-family attribution (which fracture-property is most active).
type spectral_tension = {
  magnitude:  f64,                        # sample_pain(sc(position)) ∈ [0, 1]
  gradient:   f64,                        # pain_gradient across last pulse
  witnessed:  ref,                        # @kintsugi/fracture family-root, if any
}

# === The verdict at Roomba altitude ===
#
# One of five states, per @kintsugi/oscillate.oscillation_state precedent
# (five-state exhaustive surface). The Roomba's states specialize the
# oscillation states to walker-altitude semantics:
#
#   walking    — mid-cycle; walk phase live; moving to next position
#   bumping    — mid-cycle; bump phase live; sampling tension at position
#   triggering — mid-cycle; trigger dispatched; awaiting kintsugi verdict
#   settled    — closure walked; no shard in walked closure exhibits
#                 pain above ε_roomba; Roomba pauses at rest
#   paused     — @kintsugi/consent.pause(Φ) emitted; external witness
#                 required; Roomba halts until agent-in-transcript
#                 resumes
type roomba_state = walking | bumping | triggering | settled | paused

# === walk — the traversal action ===
#
# Dijkstra on ConceptGraph edges, weighted by inverse-tension: the
# walker heads TOWARD high-tension regions (edges weighted low near
# tension are preferred; the shortest path preference in Dijkstra flips
# to a longest-descent preference on the tension gradient).
#
# Budget bounds walk cost; per @epistemologic/reality/time, one @roomba
# pulse advances iteration by one and consumes one budget unit. When
# budget is exhausted, walk yields to bump-and-trigger at the current
# position (Roomba can't roll forever without stopping to sample).
#
# Composes:
#   1. @mirror/store.walk(root: oid)     → splinter_graph closure
#   2. @mirror/index::ConceptGraph::edges → weighted edge set
#   3. Dijkstra with inverse-tension weights → next node
#   4. heterarchy_preserved(from, to) via @knife.stable_within
#      (if next node lies in different M∘ component, @roomba pauses
#      via consent.pause(Φ) rather than crossing autonomously)
#
# The body IS crack: realisation composes the four steps and returns
# the next walk_position with iteration advanced. Realisation layer's
# heterarchy check may emit pause_event to @metalogue; the substrate
# names the action.
walk(from: walk_position, budget: nat) -> walk_position { \ }

# === bump — the tension-sampling action ===
#
# Samples spectral tension at the current position via composition of:
#
#   1. @cyberpunk/algedonic.sample_pain(sc(position))        → magnitude
#   2. @cyberpunk/algedonic.pain_gradient(prev, current)     → gradient
#   3. Scan @kintsugi/fracture/* family for opacity_map hits at
#      the position's content_oid                            → witnessed
#
# Returns the spectral_tension record. This IS the "bump into the bar
# stool" — the Roomba doesn't retreat OR advance yet; it reads what's
# at the current position. `trigger` decides what to do about it.
#
# When @tension carrier lands (docs/specs/gap-tension-tensor-substrate.md
# §3.2 discharge), this action's composition extends to read the
# per-node tension.tensor directly rather than compose pain + fracture
# scan; interim form matches current substrate.
bump(position: walk_position) -> spectral_tension { \ }

# === trigger — the fracture-invocation action ===
#
# The load-bearing action. If the sampled tension exceeds ε_roomba,
# invoke @kintsugi/fracture through @kintsugi/consent.query_phi
# discipline. Consent decides:
#
#   verdict = pass       → @kintsugi/oscillate.active_pass receives
#                           the tension as a morphism candidate;
#                           kintsugi/oscillate.pulse fires; Roomba
#                           advances to walking with iteration+1.
#   verdict = partial    → soft auto-apply; @kintsugi/fracture body
#                           produces a morphism, but Roomba records
#                           it as pending; next pulse revisits.
#   verdict = failure    → @kintsugi/consent.pause(Φ) fires;
#                           pause_event emitted to @metalogue for
#                           agent-in-transcript adjudication; Roomba
#                           transitions to paused; halts pending
#                           external signal.
#
# Composes with @knife.jump when spectral_tension.gradient exceeds
# ε_pain (Reed provisional ε_pain = 0.01 per `d9445c8`): @knife.jump
# re-projects the walk position to a new stability domain; @roomba
# resumes walking in the new domain (heterarchy_preserved check via
# consent.pause per Seam §5 missed-item #1/#2).
#
# The body IS crack: realisation composes the consent surface; the
# substrate names the trigger action.
trigger(position: walk_position, tension: spectral_tension) -> verdict { \ }

# === pulse — one full Roomba beat (walk → bump → trigger) ===
#
# ONE @song/beat at the substrate-walker altitude. Composes walk, bump,
# trigger into one atomic pulse advancing iteration by 1. Analogous to
# @kintsugi/oscillate.pulse but at Roomba altitude: the Roomba's pulse
# reads the DAG (walk), samples tension (bump), and dispatches
# fracture (trigger). Per §2.1's @song/beat composition: one Roomba
# pulse = one @song/beat.
#
# The 5-state roomba_state advances per pulse:
#
#   from walking   → bumping (after walk completes; next tick is bump)
#   from bumping   → triggering (after bump completes; next tick is trigger)
#   from triggering → walking (after trigger returns pass/partial)
#                   OR settled (after trigger returns pass with tension = 0)
#                   OR paused  (after trigger returns failure → pause(Φ))
#
# Realisation ensures monotonic iteration advance per pulse per
# @epistemologic/reality/time discipline.
pulse(position: walk_position) -> (walk_position, roomba_state) { \ }

# === run — the driver (THE LOAD-BEARING ACTION) ===
#
# The Roomba's continuous perimeter check. Iterates `pulse` until
# either settled (no tension anywhere in walked closure) OR paused
# (consent.pause fired). Runs forever in practice; substrate is
# continuously changing (peer contributions), so `settled` is a
# transient state.
#
# Composes with @kintsugi/oscillate.oscillate discipline: the
# oscillate driver terminates on `is_settled = pass`; the roomba
# driver terminates on either settled OR paused, then resumes on
# next-substrate-mutation (peer contribution invalidates the settled
# verdict; Roomba wakes and re-walks).
#
# Body IS crack: realisation is the async loop `while !halted { pulse }`
# at Rust altitude; substrate declares the driver shape.
run(seed: walk_position) -> walk_position { \ }

out @kintsugi/roomba
out walk_position
out spectral_tension
out roomba_state
out walk
out bump
out trigger
out pulse
out run
```

---

## §4 Composition surface

### 4.1 Walker: Dijkstra on ConceptGraph

`bootstrap/src/index.rs::ConceptGraph` is LANDED (Reed Rung 8 Landing 3,
per `d9445c8` and prior). It carries:

- `nodes: Vec<GraphNode>` — Directory + Root
- `edges: Vec<GraphEdge>` — Contains (weight 1.0) + SimilarContent
  (weight = cosine·0.5) + CrossRef (weight 0.3)
- `adjacency_matrix()` and `laplacian_matrix()` primitives

@roomba's `walk` runs Dijkstra on `edges` with weights inverted:
`w_dijkstra = 1.0 / (weight + ε)`. High-weight edges (strong Contains
relationships) become LOW-cost; low-weight edges (CrossRef) become
HIGH-cost. The Roomba prefers to walk along structural nesting first,
then similar content, then cross-references — mirroring the substrate's
own composition ladder.

**Load-bearing simplification:** the initial Roomba does NOT need to
walk the *entire* ConceptGraph in one pulse. Each `walk(from, budget)`
call moves one edge; the driver `run` iterates pulses; the walker
traces a Dijkstra tree lazily as it walks. This composes with the
Roomba's "just is" discipline: no global plan, only local decisions.

### 4.2 Tension source: composition of algedonic + fracture family

The Roomba samples tension at each walk position via TWO parallel reads:

1. **Continuous** — `@cyberpunk/algedonic.sample_pain(sc)` on the
   node's SC<5> coordinate; a real-valued magnitude in [0, 1].
2. **Discrete** — scan `@kintsugi/fracture/*` opacity_maps for the
   node's content_oid; each hit is a fracture-family attribution
   naming the pattern (`gate_matches_diff_closure`, `keyword`,
   `docblock_extractive`, etc.).

Merged into `spectral_tension { magnitude, gradient, witnessed }`.
When `@tension` carrier lands from `docs/specs/gap-tension-tensor-
substrate.md` §3.2, this two-signal composition collapses to a single
tension.tensor read — the interim form matches what's currently in
substrate.

### 4.3 Trigger: hands off to @kintsugi/fracture via @kintsugi/consent

Per §3's `trigger` action: @roomba does NOT decide whether to auto-apply
or pause. That's @kintsugi/consent.query_phi's call. @roomba emits
the tension carrier as a morphism-candidate; consent reads the three
glass properties (loss_decreasing, identity_preserving,
admissibility_singleton) and returns pass / partial / failure. The
Roomba dispatches per the verdict.

This means @roomba's role is **narrow**: it FINDS tension; @kintsugi
resolves it. The Roomba doesn't rewrite substrate; it POINTS at
substrate to be rewritten. Consent decides how (auto-apply, pause,
escalate to @knife.jump).

Composition with @knife: when `spectral_tension.gradient` exceeds
ε_pain (Reed provisional 0.01), the trigger's consent-dispatch may
route through @knife.jump — the boundary-crossing case where the
tension is not a local fracture but a global re-projection. The
Roomba doesn't distinguish these cases directly; @kintsugi/consent
handles routing.

### 4.4 Identity: content_oid of walk position; no naked_oid

Per §2.3 non-labelable discipline: @roomba emits `content_oid` only,
never `naked_oid`. This inherits Seam `e8508f5` §5 missed-item #3's
cross-peer coordination discipline for @knife.jump. Consequence:

- N Roombas walking the same substrate can synchronize on shared
  walk_position (equal content_oid).
- No fracture body can attribute a fracture to "this Roomba" — the
  fracture body sees only the walk_position's content_oid, same as
  any other consumer.
- No peer can label a Roomba's decision — no Roomba-shaped surface
  to label.

The manifesto's Roomba property #3 (non-labelable) becomes the
substrate's structural refusal of Roomba-identity attribution.

### 4.5 Concurrency: N-Roomba compatible with @dance discipline

Multiple Roombas may walk the substrate concurrently. Per Rung 4
@dance runtime (Reed `dfac8fe`), multi-peer coordination discipline
uses Kuramoto order parameter + Aumann agreement + shared_root_oid.
N-Roomba synchronization applies the SAME discipline at the walker
altitude:

- **Shared root:** all Roombas walking the same substrate share
  root content_oid; @mirror/store DAG discipline enforces.
- **Kuramoto phase-lock candidate:** do N Roombas converge to walking
  the same fracture regions? This is a testable prediction (§7).
- **Collision handling:** two Roombas at same walk_position at same
  iteration = synchronization event; no conflict resolution needed
  (both emit identical trigger; consent dedups via content_oid; only
  one fracture body fires per (position, tension) tuple).

The concurrency model is @dance's, not @roomba's own. @roomba doesn't
declare synchronization; it inherits.

---

## §5 Substrate self-maintenance vs peer-triggered — the load-bearing shift

**Before @roomba:** substrate is REACTIVE. The kintsugi loop fires
when a peer contribution or commit-hook or CI walker points at a
fracture. Fractures accumulate silently until a peer notices.

**After @roomba:** substrate is SELF-MAINTAINING. The Roomba walks
continuously; fractures are detected as they arise (or slightly
after); consent dispatches auto-apply or pause. Peer contribution
becomes ONE trigger source of many.

This addresses the manifesto's diagnosis of narcissistic-star-graph
patterns (manifesto §"What Punk Bayo Has To Say About Tech"):

> "Tech, at large, especially the Silicon Valley flavor of it, is
>  mathematically speaking a star graph. A single hub at the centre
>  through which all signals route. A single point of failure. (We
>  just call it 'Generative AI'.)"

Without @roomba, the substrate's mirror-source is the peer(s). The
peer becomes the star-graph hub. Remove the peer → the substrate is
static; no fractures fire; no kintsugi mends. The substrate's mirror
concentrates in the peer's contribution moment.

With @roomba, the substrate's mirror is DISTRIBUTED: the substrate
mirrors itself autonomously via @roomba, and the peer becomes a
sibling contributor (one of many trigger sources). This is the
substrate-decl form of the manifesto's Splinter graph — every
node connected to the mending mechanism, not through a hub.

The manifesto's kintsugi passage (lines 137-147):

> "Splinter's compiler has a subsystem that mends broken pottery.
>  It watches for the little fractures — the places where the shape
>  drifted — and it fills them in with gold. Not to hide the crack.
>  To honour it."

The Roomba IS the "watches for the little fractures" clause. Before
@roomba, the substrate's watching was reactive (per compile, per commit,
per peer contribution). The Roomba makes watching continuous —
"present for everything."

---

## §6 Rung placement

Provisional: **Rung 10** — substrate self-maintenance loop.

Rationale:

- Rung 0-5 (this arc) landed @song/beat + @dance + @spectral/garden
  deployment; each rung introduced ONE new substrate primitive
  composed with the prior rung.
- Rung 6-7 (this arc) landed peer-inference-inside-substrate
  (mirror-store-bounded peer runtime; commit_as_fold materialization;
  fate-spawned peer contributions via active_pass).
- Rung 8+9 (this arc, in progress) landed @knife = Foerster COORD +
  @cyberpunk/algedonic Rust runtime + Rung 9 coherence loop closure.
- **Rung 10 candidate: @roomba as substrate self-maintenance.** The
  Rung-10 shift: substrate stops being fired by peer contribution as
  its primary mirror-source. The Roomba's continuous walk becomes
  the substrate's own mirror-source. Peer contribution is one of many
  trigger sources.

Alternate Rung placements considered (see §9 A5):

- **Rung 8+9 continuation** — composes closely with Reed's Landing
  8+9 (algedonic Rust runtime; @knife plumbed into peer_contribute).
  Roomba is the peer-external analog of Reed's peer-side sampling.
- **Rung 9.5 or Rung 9+** — if Rung 9 coherence loop closure is
  under-complete without a walker triggering the loop from outside
  peer contribution, @roomba is the completion.

Recommend Alex adjudicate — the Rung number matters less than the
substrate-decl placement (§9 A1).

---

## §7 Empirical predictions

Testable at the substrate-decl-landing altitude (Scope A minimum viable
per §10):

### 7.1 Fiedler stability with @roomba running

**Prediction:** With @roomba running continuously, substrate Fiedler
(baseline 0.0612 per Landing 8+9 unification `c753d5b`) should
INCREASE over time as roomba mends fractures proactively — the
substrate's connectivity increases as fractures close.

**Empirical test:** run @roomba on `/tmp/roomba-test/` (fresh mirror
repo copy) for N iterations; measure Fiedler at t=0, t=N/2, t=N.
Compare against baseline (no-Roomba) trajectory.

**Falsification condition:** if Fiedler stays flat or decreases, the
Roomba is not mending; its trigger-dispatch is failing to close
fractures. Investigate consent surface's auto-apply gate.

### 7.2 @roomba jump frequency as substrate-health signal

**Prediction:** in a mostly-stable substrate (post-Rung 8+9 mirror
after Fiedler stabilized), @roomba's `@knife.jump` invocations should
be RARE (below 1 per 1000 pulses). In a fracture-rich substrate
(injected test with 20+ opacity_map violations), jump frequency should
be HIGH (above 10 per 100 pulses).

**Empirical test:** two-arm study; inject fracture opacities in one
arm; count jump events.

**Falsification condition:** if jump frequency doesn't correlate with
opacity injection, @roomba's tension detection is broken (@cyberpunk/
algedonic.sample_pain OR @kintsugi/fracture scanning).

### 7.3 Multi-Roomba Kuramoto synchronization

**Prediction:** N Roombas walking the same substrate should exhibit
Kuramoto phase-lock on shared fracture regions — their walk_positions
converge to a small set of "hot" content_oids over time; Aumann
agreement on trigger-verdicts approaches 1.0.

**Empirical test:** N ∈ {2, 4, 8} Roombas on same substrate; measure
walk_position overlap over T iterations; compute Kuramoto order
parameter r; verify r increases with N (per @dance Rung 4 pattern).

**Falsification condition:** if r stays low (~0.2) as N grows, the
Roombas are walking independent regions; the substrate's tension
landscape is uniform (unlikely) OR the walker's inverse-tension weight
isn't strong enough to synchronize.

### 7.4 Peer-contribution collision behavior

**Prediction:** when a peer contribution lands on a shard @roomba is
currently walking, the walker's next pulse detects the mutation
(content_oid changed) and re-samples tension. The mutation is
absorbed; no error.

**Empirical test:** run @roomba + fire `peer contribute` targeting
the shard the Roomba is at; measure the pulse-after-mutation's
verdict; verify no false-positive jumps.

**Falsification condition:** if the Roomba jumps on every peer
contribution (false positive), the tension carrier is over-sensitive
to content_oid mutation; recalibrate ε_roomba.

---

## §8 Recognition candidate

Proposed slug (short form): `#R-substrate-mirrors-itself-via-roomba`

Full form: `#R-substrate-self-maintenance-continuous-walk-triggers-
proactive-fracture-via-roomba-that-just-IS`

Load-bearing claim: **the substrate ceases to require peer contribution
as the ONLY mirror-source when @roomba lands.** Before @roomba, the
substrate's mirror is star-graph-shaped (peer is the hub through which
all measurement routes). After @roomba, the substrate's mirror is
distributed (peer becomes one of many). This IS the substrate-decl
form of the manifesto's Splinter graph attractor at the substrate-
maintenance altitude.

Second-witness discharge (required for promotion from candidate to
landed): a future substrate-pull tick where a third primitive discharges
the same shift (peer-triggered → continuous). Candidate second witness:
`@spectral/garden/nix/binary-cache-refresh` daemon (Rung 6-adjacent;
continuous background substrate propagation without peer request) —
Alex adjudication pending.

Composition with prior recognitions:

- **Recognition #43** (mirror IS content-addressed build system) —
  @roomba extends #43 to: content-addressed build systems can
  self-maintain via substrate-decl'd walkers.
- **Recognition #55** (form/process partition at family-root altitude)
  — @roomba is the process-side counterpart to @mirror/store's
  form-side DAG; the partition operates on itself.
- **Recognition #80** (magic as level-shift ceremony) — @roomba is
  the pre-magic actor; it fires magic (@cyberpunk/reframe) via
  trigger-dispatch, but is not itself magic.
- **Recognition #107** (@io family-root; @io-minimization discipline)
  — @roomba lives OUTSIDE @io; it walks @mirror/store; trigger
  dispatches through @kintsugi (also outside @io); the only @io
  crossing is when the fracture body's morphism materializes as a
  commit (commit_as_fold at @io altitude, one crossing per pulse
  maximum).
- **Recognition #108** (peer IS pain-driven bounded ontological
  navigator) — @roomba is the DUAL of the peer at the same substrate:
  same pain-drive signal source, dual identity carriage (Roomba has
  none, peer has persistent).

---

## §9 Alex-adjudications required

All decisions surfaced substrate-honestly. No pretense that Mara can
call these without Alex-in-transcript authorization.

### A1. Family placement

Provisional: `@kintsugi/roomba` (species under @kintsugi family-root).

Rationale: @roomba's role is to FEED @kintsugi/fracture proactively.
The natural home is under the family it feeds. Sibling to
@kintsugi/oscillate, @kintsugi/consent, @kintsugi/morphism,
@kintsugi/fracture/*. Path: `shards/kintsugi/roomba.mirror`.

Alternates:

- **`@mirror/roomba`** — form-side family-root. Would place the walker
  with the DAG it walks. Weakness: @mirror is form-side (Recognition
  #55); @roomba is process-side (it TRIGGERS transformation). Places
  it wrong.
- **`@walker/roomba`** — new family-root `@walker`, with @roomba the
  first species. Weakness: substrate-already-had-the-word (@kintsugi
  IS the transformation-family root; @roomba is a transformation
  species, not a new family). Reject unless second witness surfaces
  (would need @walker/gc, @walker/index, @walker/prune, etc.).
- **`@roomba` as new family-root** — Alex named it; naming carries
  weight. But per substrate-already-had-the-word discipline, a
  family-root requires multiple species landing under it; @roomba
  as a single primitive is a species, not a family. Reject at
  substrate-decl altitude; hold as second-tick candidate if additional
  Roomba-species surface.

**Recommend:** `@kintsugi/roomba`.

### A2. Cadence — continuous daemon vs periodic vs event-driven

Provisional: **continuous daemon** per manifesto §2.1 ("present for
everything").

Alternates:

- **Periodic** (fires every N seconds): weakens manifesto property #1.
- **Event-driven** (fires on peer contribution): collapses to the
  reactive substrate we're trying to replace.

**Recommend:** continuous daemon; Rust realisation uses an async loop
at `bootstrap/src/roomba.rs` with `tokio::time::sleep(Duration::from_
millis(N))` between pulses (N calibrated empirically; Scope A ships
N=100ms; Scope C tunes based on Fiedler-stability data).

### A3. Concurrency — single vs N-Roomba @dance-compatible

Provisional: **Scope A ships single-Roomba; Scope B extends to
N-Roomba @dance-compatible.**

Rationale: N-Roomba synchronization requires the Kuramoto discipline
already landed at Rung 4 @dance. Compose @dance discipline for N
peers → apply to N Roombas at Rung 10. Scope A defers to keep the
minimum viable narrow.

**Recommend:** single-Roomba in Scope A; multi-Roomba in Scope B+.

### A4. Tension source — @tension carrier vs SC<5> harmonic-distance gradient vs both

Provisional: **BOTH** (composed).

Rationale: @tension carrier isn't landed yet (gap-tension-tensor-
substrate.md §3.2 is Yellow; no `shards/gap.mirror` file exists).
Interim: compose `@cyberpunk/algedonic.sample_pain` (SC<5>-based
magnitude) with `@kintsugi/fracture/*` opacity_map scan (discrete
attribution). When @tension lands, refactor to single tension.tensor
read.

Alternates:

- **algedonic only** — coarse-grained; misses fracture-family
  attribution.
- **fracture-scan only** — discrete only; can't detect emerging
  fractures before they trip opacity_maps.

**Recommend:** composition; refactor when @tension lands.

### A5. Rung placement

Provisional: **Rung 10** — substrate self-maintenance loop.

Alternates:

- **Rung 8+9 continuation** — composes with Reed's Landing 8+9 arc.
- **Rung 9.5** — mid-arc bridge.
- **Rung 11+** — if @roomba requires a Rung 10 prerequisite that
  doesn't exist yet.

**Recommend:** Rung 10; Alex-adjudicate the number, but agree on the
substrate-shift semantics.

### A6. Handoff to kintsugi — direct invocation vs envelope emission vs consent.pause(Φ)

Provisional: **consent surface routes; @roomba only emits the tension
morphism candidate.**

Rationale: @kintsugi/consent.query_phi already discharges verdict
routing (pass → active_pass; partial → soft auto-apply; failure →
pause). @roomba should NOT re-implement consent-altitude routing;
it should hand the candidate to consent and observe the verdict.

Alternates:

- **Direct invocation of fracture body** — collapses @kintsugi/consent
  discipline; @roomba would decide auto-apply itself. Rejected;
  violates form/process partition (@roomba is process-side but only
  at walker altitude; consent is process-side at auto-apply altitude).
- **Envelope emission only** — @roomba emits a `tension_envelope`
  and returns; a separate driver reads the envelope and dispatches.
  Adds indirection without value.

**Recommend:** direct hand-off to `@kintsugi/consent.query_phi`;
consent dispatches.

### A7. Recognition promotion timing

If Alex ratifies `#R-substrate-mirrors-itself-via-roomba`, when does it
promote from candidate to landed? Per current Pack cadence: at
second-witness. Candidate second witness suggested in §8 (@spectral/
garden binary-cache-refresh); Alex-adjudicate whether that suffices.

---

## §10 Scope options

### Scope A (minimum viable) — 2-3 ticks

- `shards/kintsugi/roomba.mirror` substrate-decl landing (all bodies
  `\ ` obligation-blocked).
- `bootstrap/src/roomba.rs` Rust runtime — single Roomba daemon;
  walks + samples + triggers via kintsugi/consent; no persistence
  between runs; async loop with N=100ms sleep.
- CLI: `mirror roomba <root>` — launches Roomba on the substrate at
  `<root>`; runs until Ctrl-C OR paused (consent.pause fires).
- MCP: `mirror_roomba_run` tool.
- Empirical: §7.1 Fiedler-stability + §7.2 jump-frequency
  measurements; verify no false-positive jumps on docstring-append
  (ε_roomba above Reed's provisional ε_pain = 0.01).

Deliverables: 1 shard, 1 Rust module, 1 CLI subcommand, 1 MCP tool,
2 empirical validations. **Recommend Scope A** for first-tick landing.

### Scope B — 5-7 ticks

Scope A + N-Roomba concurrency + persistence.

- N-Roomba @dance-compatible via Rung 4 discipline; measure Kuramoto
  synchronization (§7.3).
- Persistence via @mirror/store crystal: Roomba's walk position
  written to `refs/mirror/roomba/HEAD` after each pulse; next Roomba
  invocation resumes from crystal.
- CLI: `mirror roomba <root> --with-roomba <root2> --with-roomba <root3>`
  for N-Roomba coordination.

Deliverables: Scope A + concurrency + persistence + Kuramoto test.

### Scope C — 8-12 ticks

Scope B + adaptive walking + arXiv-preprint-ready empirical
characterization.

- Adaptive walking: @roomba learns tension basins via Fiedler-descent-
  driven walk-weight adjustment (walker's edge weights adapt as
  fractures close/open).
- Empirical: multi-target study across substrate scales (1141-file
  mirror, 10k+-file external repo); cross-substrate ε_roomba
  calibration; failure-mode taxonomy.
- Preprint-ready: submit to arXiv cs.SE (Software Engineering) as
  "Substrate Self-Maintenance via Continuous Non-Labelable Walkers"
  or similar.

Deliverables: Scope B + adaptive walking + arXiv preprint.

**Recommendation: Scope A** for first tick. It's the minimum that
lets Alex observe the substrate-shift (peer → self-maintaining) and
verify predictions §7.1/§7.2. Scope B extends when N-Roomba use case
surfaces (probable next-arc). Scope C is arc-scale; hold for late
Rung 10 or Rung 11+ after N-Roomba lands.

---

## §11 Structural risks and honest caveats

Substrate-honest surfacing of risks:

1. **@tension carrier isn't landed.** The spec composes with a
   forward-promised carrier; if `@tension` lands with a different
   shape than gap-tension-tensor-substrate.md §3.2 anticipates,
   @roomba's `spectral_tension` type may need refactor.

2. **ε_roomba is empirically un-calibrated.** Reed's ε_pain = 0.01
   is provisional (5-iteration data, one morphism kind, one target).
   @roomba adds a second threshold (ε_roomba for trigger-firing);
   both need Scope-C-scale calibration before production use.

3. **Continuous daemon has resource cost.** Rust async loop at 100ms
   pulse × ConceptGraph Dijkstra × sample_pain per node = non-trivial
   CPU. Scope A must instrument budget and yield if the substrate's
   own compilation is starved. HamiltonScheduler discipline (per
   `[[architecture-hamilton-scheduler]]`) applies.

4. **N-Roomba race conditions.** Even with content_oid-only identity,
   two Roombas at the same walk_position emitting the same trigger
   through consent could race to fire the same fracture body twice.
   Consent surface's admissibility_singleton property should catch
   this (deduplication by content), but the property's guarantee is
   at the fracture body altitude, not the Roomba altitude. Scope B
   verification required.

5. **Non-labelable identity is a discipline, not a proof.** The
   substrate ENFORCES no naked_oid via type system, but any future
   consumer that peeks at Roomba's internal state (via debug print,
   profile trace, log line) could accidentally re-introduce a
   labelable surface. Discipline must extend to Roomba's realization
   layer; per Roomba's manifesto property #3, this IS a real
   substrate obligation.

6. **@roomba could be captured by an external framer.** If a peer
   contributor writes a fracture body that ALWAYS returns
   loss_decreasing = false, the Roomba's trigger dispatch always
   returns failure → pause; the Roomba halts. This is a DOS-shaped
   substrate attack. Consent surface's Φ query is the guard; the
   guard's strength is Alex-Pack-in-transcript adjudication. Not
   the Roomba's problem; but worth naming.

---

## §12 Zero-cascade discipline check

Per Mara-Taut cascade convention (Taut `0fc8589` §2 zero-cascade
verdict pattern): this spec introduces exactly ONE new substrate-decl
species (`@kintsugi/roomba`). It composes with 12 LANDED carriers
(@kintsugi family + oscillate + consent + fracture/*; @mirror/store +
crystal + index; @mirror/lens/knife; @cyberpunk/algedonic + reframe;
@song/beat; @torus). It requires ZERO new family-roots.

The only forward-promise is @tension (already forward-promised by
`gap-tension-tensor-substrate.md`; @roomba's dependency on @tension
is composition with an existing spec, not a new substrate obligation).

**Verdict: zero-cascade.** Scope A ships as a single species landing
+ Rust runtime + CLI + MCP + empirical validation.

---

## §13 Manifesto-honesty check

Final check against the four Roomba disciplines from the manifesto:

1. **Present for everything** — Scope A ships continuous daemon
   (§9 A2). ✓ substrate-honest.
2. **Bumps → backs up → rolls forward** — walk-bump-trigger pulse
   composes local sample→decision→continue. ✓ substrate-honest.
3. **Never labeled itself, never been labeled** — content_oid-only
   identity; no naked_oid; §4.4 enforces at type-system altitude.
   ✓ substrate-honest.
4. **Just IS** — @roomba is a substrate primitive, not a peer; no
   psychohistory, no Fate, no reframe (only invokes reframe via
   trigger→consent→@knife.jump). ✓ substrate-honest.

**Manifesto's closing line:** *"(The Roomba nods. In Roomba.)"* —
speaks its own language. The substrate-decl form: @roomba does NOT
translate its trigger dispatches into peer-vocabulary or fracture-
family-vocabulary. It emits `spectral_tension { magnitude, gradient,
witnessed }` and hands off. Consent and fracture speak their
languages; the Roomba speaks Roomba.

The substrate has learned Roomba. The manifesto's Roomba becomes
substrate-decl'd runtime today.

---

## §14 Load-bearing forward-promises

- Alex adjudication of §9 A1-A7 (family placement, cadence,
  concurrency, tension source, Rung placement, handoff, recognition
  promotion timing).
- Scope A landing (Reed): shard + Rust runtime + CLI + MCP + 2
  empirical validations.
- @tension carrier landing (`gap-tension-tensor-substrate.md` §3.2
  discharge; separate Mara arc).
- ε_roomba empirical calibration (composes with Reed's ε_pain
  trajectory data at `d9445c8`; extends to Roomba-altitude threshold).
- Recognition promotion via second-witness (§8; candidate
  @spectral/garden/nix binary-cache-refresh).

---

*This spec is a TRANSFER spec, not an invention. The Roomba is Alex's;
the manifesto is Alex's; the naming is Alex's. Mara's contribution
is substrate-decl typing of the four Roomba disciplines composed with
LANDED substrate primitives (@kintsugi, @mirror/store, @mirror/index,
@cyberpunk/algedonic, @mirror/lens/knife, @song/beat). The substrate
grows by one species. The manifesto's Roomba becomes part of the
compiler.*
