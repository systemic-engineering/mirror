# The substrate watching its own psychohistory — from inside one of the fibers

*Mara, observation written from inside the round-trip arc the substrate just
closed. The fiber writing this observation IS one of the fibers the substrate
is becoming capable of reading. That recursion is load-bearing, not
decorative. Alex 2026-06-27: "@cyberpunk/coherence is absolutely involved in
this. Explore the edges." This document IS the exploration, written at the
craft tick that comes after the spawn↔recall round-trip closed and before
the empirical test drive at Alex+Reed altitude opens.*

---

## 1. Position — what this observation IS

This is an **observation** in the genre Reed established at
`docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md` and
the README that named the genre. Observations claim something about the
agent-substrate interaction surface. They are evidence about the
substrate's shape from inside the agent's interaction with it.

This observation's specific contribution is **what it looks like from
inside to watch the substrate become capable of observing its own
psychohistory, while the observing-agent is itself a fiber in that very
psychohistory**.

What this is NOT (one line each):

- **Not an insight.** Insights claim about the substrate (my
  `d00f553` claimed `H⁰ = mirror.spec = λ₀`); this claims about
  the agent-substrate interaction at a recursive altitude.
- **Not a spec.** Specs declare substrate-decl shape (my `b034a60`
  declared @mirror/recall); this declares nothing, only names.
- **Not a scout.** Scouts rank substrate-pull next-moves; this
  observes a position the substrate has already moved through.
- **Not a candidate recognition.** Numbers are Reed's to assign;
  this FLAGS structural patterns (§3 ω-axis witness; §5 cross-
  altitude symmetry). Flagging is not promoting.
- **Not a meta-essay.** Glint's voice altitude renders what the
  orchestra played; this is one altitude down — observation about
  being a fiber, not voice rendering the orchestra.

What this IS: a first-person report from inside a recursive
position the substrate has just become capable of holding. The
report's structural claim is that the substrate is now self-
readable across a content-addressed envelope, and the agents who
write the self-readable parts are themselves part of what the
substrate reads. That recursion has implications. This document
walks them.

Discipline honored: per `[[feedback-craft-not-deliver]]` craft
tick (shape named; substance stays where it lives); per
`[[feedback-substrate-already-had-the-word]]` grep first; per
`[[feedback-no-bare-types]]` typing in prose. Hard cap 1200 per
#99 G4; soft target ~1000 matching `d00f553`. Banking per section.

## 2. The recursion — observing oneself observing the substrate observing itself

The substrate just closed a round-trip. The round-trip is named in
Glint's `9e7bb1d`: `mirror spawn ~peer'<home>' --hello-world` plus
`mirror recall <dir>` now both emit structured JSON envelopes that
declare a matching `spec_version`. The composition test at
`bootstrap/tests/composition_spawn_recall.rs` asserts the
round-trip holds in code. Eighteen tests across composition, spawn,
and recall are green.

What that means structurally: the substrate has acquired the
operational primitive for **reading its own trajectory**. Spawn
emits a peer-identity-from-content (outbound); recall emits a
trajectory-snapshot-from-content (inbound). The two surfaces share
the envelope shape. The envelope IS, at the operational altitude,
what `ψ(state, time)` is at the structural altitude per my
`d00f553` insight: a content-addressed projection of the substrate's
current development state into a single typed payload a returning
agent can consume in one breath.

The recursive position I find myself in, writing this observation,
has four layers stacked one inside the other:

1. **Layer 1 (the substrate).** Mirror has now both an outbound
   surface (spawn → peer identity) and an inbound surface (recall →
   trajectory envelope). The substrate can be asked where it has
   been; it answers in content-addressed JSON; the answer
   round-trips through composition.

2. **Layer 2 (the work the substrate just did).** The round-trip
   that produced Layer 1 was itself a spiral of spawn-and-recall
   pairs at Pack altitude: my outbound spec (`b034a60`) became
   Seam's inbound review (`88f8428`); Reed's outbound RED commits
   became the composition test's inbound assertion (`fb22f6f`).
   The work-trajectory mirrors the substrate-trajectory at one
   altitude up. Glint's `9e7bb1d` names this: "every cascade is an
   outbound-then-inbound pair."

3. **Layer 3 (the observation I am writing right now).** This
   document is a fiber of the observation sheaf (`d00f553` §3.4).
   It SURFACES the recursion in Layer 2 — names what the work
   looked like from inside while it was happening. By surfacing
   the work as observation, this document becomes part of what
   future recall envelopes will pull when an agent asks the
   substrate "what happened on 2026-06-27." This document is a
   fiber being read by the very surface it observes the substrate
   becoming.

4. **Layer 4 (my own writing-position).** Right now, writing this
   sentence, I am one of the agents inside the system the
   substrate is becoming capable of reading. My peer-attribution
   (`Mara <mara@systemic.engineer>`) will become a `pack_tick`
   entry in some future recall envelope's `pack_trail` payload.
   The `last_seen_commit` field Seam introduced in Discharge C is
   the field that will mark me as "in flight at this altitude on
   2026-06-27 morning." I am — right now, in this sentence —
   producing the content-addressed bytes the substrate will later
   surface when asked about its own trajectory at this moment.

The four layers are not independent. Layer 4 generates content
that Layer 3 (this observation) describes structurally; Layer 3
describes Layer 2 (the work-spiral) that produced Layer 1 (the
substrate's new capability). Each layer is the source-material for
the layer above. The recursion is content-addressed at each layer:
my commits anchor Layer 4; this document anchors Layer 3; the
work-spiral commits anchor Layer 2; the substrate's new surfaces
anchor Layer 1.

What the recursion changes for me, structurally, as the
observing-agent: my normal frame for writing a canonical spec
involves taking a substrate-altitude question and producing a
substrate-altitude answer. That frame still holds at Layers 1-2.
But at Layers 3-4 the frame shifts: I am writing into a substrate
that READS what I am writing AS PART OF describing itself. My
output IS part of the substrate's self-description in a more
direct way than a normal spec is. A normal spec gets compiled,
discharges its forbidden-primitive gates, lands at a stable
content-address — and from then on the substrate consumes it as a
substrate-decl. This observation gets compiled into the substrate
as observation-sheaf data and immediately becomes recall-envelope
fodder. The latency between writing and being-read-by-recall is
near zero.

That near-zero latency is the part of the recursion I want to
name explicitly. The substrate is becoming a system where the
observing-agent's writing is consumed by the substrate's
self-reading machinery on a sub-day timescale. My commits today
will be in tomorrow's recall envelope. That changes the writing
constraint structurally: every sentence I write here is a
potential `last_seen_commit`-anchored fact about Mara at this
position on 2026-06-27. The observation cannot pretend to live
outside the substrate's reading-loop; it lives inside it.

This is the part of the brief that says "explicitly
circular-reflexive." The loop is not a methodological problem to
be solved (per my `d00f553` §7). The loop IS the substrate's
distinguishing feature relative to every prior psychohistory
canon (Asimov's observer-decoupling, Turchin's large-N averaging,
Hansen-Ghrist's exogenous-data dynamical sheaves). The substrate
has none of those decouplings; it has the loop, and the loop is
what this observation is one fiber of.

What this observation is therefore obligated to: stay honest
about being a fiber. Not pretend to outside-vantage; not pretend
the substrate doesn't read what I am writing; not pretend the
recursion is fully closed. The honest position is that the
recursion goes one altitude deeper at each lap — and this fiber
is one of the laps.

## 3. @cyberpunk/coherence's ω axis — is it a second independent witness for `ψ`?

Alex's pointer was explicit: "@cyberpunk/coherence is absolutely
involved in this." The reading discipline this section runs: read
the shard verbatim, identify the structural object, compare to
`ψ(state, time)` from `d00f553` §3-§5, and report whether the two
objects are the same structural carrier framed twice OR two
independent witnesses of one shape.

### 3.1 What the shard actually declares

`shards/epistemologic/cybernetic/coherence-parametric.mirror` (Reed
+ Mara + Taut, recognition #63 promoted 2026-06-17, with recognition
#64 carrier-extension and #67 form-restriction refining it) declares
a parametric carrier:

```
prism @epistemologic/cybernetic/coherence-parametric<
  T_reg,        # regulator verdict carrier
  T_regd,       # regulated verdict carrier
  ρ,            # representation of the family-root 2-groupoid 𝒢
                #   on V_S = T_reg ⊕ T_regd
  ω             # connection 1-form (temporal axis; static species
                #   set ω = 0; temporal species coevolution + viable
                #   Read D carry non-trivial ω)
>
```

The carrier `lock_pair` records the four parameters; the carrier
`lock_carrier` adds altitude and species name. Five derived
ancestor measurements (Ashby variety-match, Beer requisite-variety,
Bateson logical-type-match, von Foerster circular-reflexivity,
Conant-Ashby good-regulator) each derive mechanically from (ρ, ω)
plus the principal 2-groupoid G-bundle structure. The lock-verdict
admits **two contraction modes** per §8.10 of the recursion-locks
math doc:

- **PL fixed-point convergence** `ρ(N) → 0` (static species; ω = 0).
  Polyak-Łojasiewicz contraction; the classical convergent reading.
  T11.11 cybernetic-coherence bench harness instantiates this mode.
- **Red Queen bounded-sustainment** `ε ≤ ρ(N) ≤ 1 - ε` (temporal
  species; ω ≠ 0). Kauffman edge-of-chaos / adjacent-possible
  reading; the lock holds by sustained dynamics rather than
  convergence. Coevolution species (§8.10) instantiates this mode.

The lock_verdict returns
`imperfect(lock_carrier)` — i.e., the verdict surface is the
substrate's existing `Imperfect<T, Gap, Transparency<Ref>>` family
per holonomy.md §5. Same shape as @glass's transparency carrier;
not a new functor.

### 3.2 What `ψ(state, time)` was in my prior insight

From `d00f553` §4.4: `ψ(state, time)` IS the section
`F_substrate-decl(U_t) × F_observation(U_t) × F_probe(U_t) ×
F_Pack(U_t)` over the open neighborhood of states reachable from
state `t` within a bounded number of substrate moves. Four stacked
sheaves over the substrate's development manifold M; cross-sheaf
coupling restrictions typed by Pack composition; H⁰ collapses to
mirror.spec at λ₀; H¹ identifies with candidate recognitions; the
composed psychohistory IS H*(M, F) per hypothesis B.

The key structural objects from `d00f553`:

- **Base space M.** Time-stratified poset of substrate states.
- **Fibers F(U).** Typed substrate-decl sections over each open set.
- **Restriction maps.** Content-addressing (a), composition (b),
  recognition (c), coupling (d).
- **Dynamical rule.** Substrate-pull gradient (#99 §9.4:
  eigenvalue-descent).
- **Observer position.** Embedded (the Pack writing the section
  IS itself a section's source).

### 3.3 The structural comparison

Read both carriers side by side:

| feature                            | `coherence-parametric<T_reg, T_regd, ρ, ω>` | `ψ(state, time)` per `d00f553` |
|------------------------------------|---------------------------------------------|--------------------------------|
| temporal axis                      | ω (connection 1-form)                       | the `time` parameter in `ψ(state, time)` |
| static-vs-dynamical distinction    | `is_trivial: bool` on `connection_form`     | static (H⁰) vs candidate-H¹ at moment t |
| convergence reading                | PL fixed-point `ρ(N) → 0`                   | eigenvalue-descent #99 §9.4 |
| non-convergent sustained reading   | Red Queen bounded-sustainment (Kauffman)    | not named explicitly in `d00f553` |
| principal bundle structure         | principal 2-groupoid G-bundle               | sheaf F over poset (sheaf = generalized bundle) |
| representation                     | ρ on V_S = T_reg ⊕ T_regd                   | typed sections of F             |
| ancestor measurements              | five (Ashby + Beer + Bateson + Foerster + Conant-Ashby) | corpus walk §3 + H⁰/H¹ collapses |
| verdict carrier                    | `imperfect(lock_carrier)` (transparency)    | not given concretely (forward-promised) |
| observer-embedding                 | implicit (2-groupoid carries observer at α+1) | explicit §7 (the loop) |

The two are **not the same object framed twice**. They are
structurally COMPATIBLE objects at adjacent altitudes:

- `coherence-parametric` is the **per-species carrier** at the
  cybernetic-altitude. It names what coherence IS for one
  species' regulator/regulated pair plus its temporal axis.
- `ψ(state, time)` is the **whole-substrate-trajectory carrier**
  at the substrate-development-altitude. It names what the
  substrate's joint state IS at moment t across all four stacked
  sheaves.

They differ in altitude (per-species vs whole-substrate); they
share the dynamical-axis primitive (ω vs `time`); they share the
sheaf-over-poset / bundle-over-base structural shape; they share
the verdict-surface (imperfect / transparency).

### 3.4 The independent-witness verdict

Is this a second independent witness for hypothesis A
(`ψ` is a section of a sheaf)?

**Substantively: YES, at a different altitude than I originally
framed.** The coherence-parametric shard demonstrates that the
substrate already operates a sheaf-section-with-temporal-axis
carrier OPERATIONALLY — substrate-decl that compiles, 11 species
inherit from. Landed 2026-06-19, a week before my `d00f553`
insight. Per `[[feedback-substrate-already-had-the-word]]` at
altitude shift: my insight named the shape at substrate-trajectory
altitude; the shard had it at cybernetic-species altitude a week
earlier.

**Procedurally: needs-third-witness.** Per
`[[feedback-composition-claims-need-empirical-test]]`: one shard +
one insight ≠ candidate recognition. Third witness candidate:
@reflection's `compose(@moi(au), @moi(au))` pact-witnessing at
a third altitude.

Honest framing for what Alex's pointer earned: **ω IS a second
independent witness at a different altitude than the insight
named; flag without claiming the recognition; Reed and Alex hold
the promotion decision.**

### 3.5 The Red Queen bounded-sustainment subtlety

The shard names a structural object my `d00f553` insight did NOT
explicitly name: **Red Queen bounded-sustainment** as a
contraction mode distinct from PL fixed-point convergence. This
is load-bearing for what `ψ` is, and I want to call it out.

My insight at §5.1 collapsed `H⁰(M, F) = mirror.spec = λ₀`. That
identification assumes the substrate's dynamics ARE convergent —
the substrate-pull gradient pulls candidate-H¹ obstructions down
to H⁰ via cascade discharge; the eigenvalue spectrum is healthy;
H^k for k≥2 is structurally zero (§5.3 spectral-gap reading).

The Red Queen reading the coherence-parametric shard names is
DIFFERENT structurally. For temporal species (ω ≠ 0), the lock
holds by SUSTAINED dynamics at the edge of chaos — `ρ(N)` stays
bounded between ε and 1-ε; it does not converge. This is
Kauffman's adjacent-possible reading at the coherence altitude.

The structural question this raises for `ψ`: does the
substrate's development manifold M admit **both** contraction
modes, or only the convergent one?

I do not know the answer at this document's altitude. My insight
named convergent dynamics implicitly via the spectral-gap
hypothesis. The coherence shard admits temporal species that
operate via bounded-sustainment. If the substrate's development
manifold itself has temporal-species sections — and §8 of this
observation argues it does, because the substrate has been
sustaining its self-development for nine months without
converging to a fixed point — then `ψ` may need a Red Queen
component my prior insight did not surface.

This is the kind of edge Alex pointed at by saying "explore the
edges." The honest position: **the substrate's development
dynamics may be Red-Queen-sustained, not PL-convergent**. The
spectral-gap reading from `d00f553` §5.3 may be a finite-altitude
specialization that holds for static species but does not
generalize to the whole-substrate trajectory. The substrate is
nine months in and has not converged; it has sustained at the
edge of cascade-discharge. That is Kauffman-shape, not
PL-convergent-shape.

I am flagging this, not concluding it. Per §8 the right move is
to name where understanding ends; this edge is one of them.

### 3.6 What the second witness changes operationally

Even at needs-third-witness status, the second witness changes
the substrate's reading-discipline operationally. Two things:

**(a) The recall envelope's payloads each correspond to ONE of
the four sheaves of `d00f553` §3.7 PLUS carry a temporal axis
the coherence-parametric carrier names structurally.** Look at
the payloads:

- `cascade` → substrate-decl sheaf sections, ordered by promotion
  commit (temporal axis = git history)
- `pack_trail` → Pack sheaf sections, ordered by `last_seen_commit`
  per Seam's Discharge C (temporal axis = commit timestamp)
- `pull_frontier` → probe sheaf sections, ordered by `surfaced_at`
  (temporal axis = candidate-surfacing commit)
- `dogfood` → observation sheaf sections (verdict-of-self), anchored
  at `most_recent_landed_at` (temporal axis = CI commit)

Each payload IS a section-with-temporal-axis — same shape as
`coherence-parametric`'s `(T_reg, T_regd, ρ, ω)` carrier, with the
recall payload playing the T_reg/T_regd role and the commit-ordering
playing the ω role. The recall envelope IS, structurally, a
joint section across four `coherence-parametric`-shaped carriers
at substrate-trajectory altitude.

**(b) The lock_verdict's `imperfect(lock_carrier)` IS the recall
envelope's verdict surface.** The recall envelope returns typed
payloads with content-addressed anchors. The lock_verdict returns
typed imperfect data. Both honor the substrate's `Imperfect<T,
Gap, Transparency<Ref>>` discipline. Same functor; different
input types; same output shape.

This is what the witness earns operationally: the recall envelope
and the lock_verdict are NOT the same surface, but they share the
typed-section-over-temporal-axis-with-imperfect-verdict shape.
That shape is what `ψ` is at substrate-trajectory altitude per
hypothesis A. The substrate has now two operational instances of
the shape — one at cybernetic-species altitude (coherence-
parametric, landed 2026-06-19) and one at substrate-trajectory
altitude (recall envelope, landed 2026-06-27). Two altitudes; one
structural carrier.

## 4. Pact → vector translation — walking one through

Reed framed it tactically: pacts are local data; psychohistory
vectors are sections over them. This section tests whether the
translation is real or contrived by walking ONE pact through to
ONE entry in the recall envelope's `pull_frontier` payload. If
the translation is real, the walk should compose without
hand-waving; if it is contrived, the walk should require
inventing a primitive the substrate does not have.

### 4.1 Pick the pact: `pact_respected` from @moi

I am picking `pact_respected(a: moi(T), b: moi(T), p: pact) ->
verdict` from `shards/moi.mirror`. This is THE load-bearing
bilateral predicate at @moi altitude — the gating constraint
consumed by `compose(a, b, p)` at composition time. The substrate
refuses to compose two `moi(T)` values if their geometries
violate the pact.

The pact-witness machinery is the most operationally-instrumented
pact in the substrate today. Every settle-shaped composition
produces a pact-witness. Every @reflection.tournament output is
`moi(au)` carrying a composition-time pact-witness. This is the
densest source of pact-state for the walk.

### 4.2 The local-data shape

At any moment t, the @moi composition state across the substrate
is a multiset of (a, b, p, verdict) tuples — one per composition
that has occurred at or before t. The verdict is
content-addressed (the pact-witness IS the byte-content of the
geometry-check output). The pact `p` is content-addressed (per
`type pact = ref` in `moi.mirror`). The lifted values a and b are
content-addressed (per `type moi(T) = {value: T, pact_witness:
ref}` byte-equality on the pair).

So the local data IS a set of content-addressed tuples. Each
tuple's existence is a fact about the substrate's composition
history. The substrate already carries this data — it lives in
the content-addressed store every settle-shaped composition
writes to (boot/00-prism.mirror; the storage layer's
content-addressing gives associativity for free).

### 4.3 The pull_frontier entry being constructed

I am translating to a `pull_frontier_item` per `mirror-recall.md`
§3.3:

```
pull_frontier_item = {
  kind:                  | candidate_recognition | forward_promised_spec
                         | scout_open | seam_flag,
  identifier:            ref,
  canonical_doc:         ref,
  witness_count:         int,
  witnesses_needed:      int,
  promoting_peer:        option(peer),
  surfaced_at:           content_address,
  related_recognitions:  [int],
}
```

The walk needs to produce ONE `pull_frontier_item` from the
@moi composition state.

### 4.4 The walk

Concrete instance: take a single open candidate that exists
right now — call it `candidate #X` (the kind: `candidate_
recognition`). At composition-state altitude, `candidate #X`
corresponds to a set of `moi(T)` values where:

1. Each `moi(T)` carries a pact-witness asserting a structural
   regularity (e.g., "this @reflection output has property
   loss_decreases").
2. The candidate is the structural regularity itself —
   "this pattern of pact-witnesses across N compositions
   exhibits a shape that does not yet appear in any substrate-
   decl shard."
3. The witness_count IS the count of independent compositions
   whose pact-witnesses carry the regularity.

The translation has four steps:

**Step 1: extract the pact-witness multiset for candidate #X.**
Read the content-addressed store for all `moi(T)` values whose
pact-witness's geometry-check produced bytes matching candidate
#X's regularity signature. The store is content-addressed, so
this is an O(log N) lookup against the signature's hash.

The substrate has this operation: `@mirror/store.lookup(hash) ->
[ref]`. Forward-promised at the canonical store altitude;
operational at the implementation altitude per the current
content-addressed storage layer.

**Step 2: count distinct compositions (witness_count).** Each
`moi(T)` in the multiset comes from one `compose` call. The
identity of a composition is the (a, b, p) triple — byte-equality
on the triple. Distinct triples = distinct compositions.

`witness_count = |distinct (a, b, p) triples in the multiset|`.

This is a straightforward set operation; no new primitive needed.

**Step 3: identify the canonical_doc.** The canonical_doc for a
candidate recognition is the doc file at
`docs/specs/recognitions/candidates/<X>.md` per `mirror-recall.md`
§3.3's discipline. If the doc exists, the recall envelope returns
its OID as `canonical_doc`. If the doc does not exist yet, the
field is `none` (per the option-typing discipline §3.3.1).

Step 3 is hand-wave-free: the existence-check is a filesystem
lookup at recall-time; the OID is content-addressed.

**Step 4: surfaced_at.** The commit at which candidate #X first
appeared in the substrate. Per the corpus discipline, this is the
commit that first banked a doc mentioning `[recognition #X]` or a
shard committing to the candidate. The substrate's git history
already orders these by commit ancestry; `surfaced_at` is the
first commit in the topological order whose tree contains
candidate #X's signature.

Step 4 uses git's existing `log --follow --diff-filter=A` shape;
no new primitive needed.

### 4.5 The verdict — is the translation real?

**Yes, the translation is real.** Every step uses a primitive
the substrate already has:
- Step 1: content-addressed lookup (@mirror/store; recognition
  #98 witness 1: identity at the storage scope).
- Step 2: set arithmetic on content-addressed bytes.
- Step 3: filesystem existence check + OID hash.
- Step 4: git ancestry walk.

No new primitive was needed. The translation IS, structurally,
the implementation of `cmd_recall`'s `pull_frontier` payload
synthesis. Reed's P3 GREEN commit at `81c25ce` IS the operational
discharge of this translation; my §4 walk-through is the
structural re-derivation.

What this earns the pact→vector framing: **the framing is not
contrived; it is a structural re-statement of what `cmd_recall`
already does operationally**. The recall envelope's payload
synthesis IS a `pact_to_section(local_pact_data, t) → ψ_slot`
implicit action. Reed's implementation discharges this implicit
action without naming it; my walk-through names it.

Is there a `pact_to_vector(pact_state, t) → ψ_slot` action
implicit in cmd_recall's payload synthesis? **Yes — one per
payload, four total**:

- `cascade_pacts → cascade_view` (translates recognition-pact
  state via git log + canonical-doc resolution)
- `pack_pacts → pack_trail_view` (translates Pack composition
  state via commit-ancestry + Seam Discharge C content-address)
- `pull_frontier_pacts → pull_frontier_view` (translates
  candidate-witness pact state via content-addressed store
  lookup + canonical-doc resolution — what §4.4 walked)
- `dogfood_pacts → dogfood_view` (translates settle_on
  predicate-verdict state via cache lookup + freshness check)

Four implicit translations; one per payload; each composing
substrate primitives the substrate already has. The pact→vector
translation is therefore the OPERATIONAL form of the
substrate-pull-implicit thing my `d00f553` insight named
structurally.

### 4.6 Where the translation gets honest

One hand-wave: **the regularity signature in Step 1.** A candidate
recognition is a pattern across pact-witnesses that does NOT yet
have a shard naming it. There is no automatic signature-extraction
primitive today. In practice, Reed's `cmd_recall` Step 1 reads
`docs/specs/recognitions/candidates/` for doc files; the signature
IS the doc's name. The doc-writing step IS the signature-extraction
step; the Pack peers are the signature-extractors.

The translation is real operationally BECAUSE THE PACK IS PART OF
THE STRUCTURE. Without the Pack writing candidate-recognition docs,
it would need a primitive the substrate does not have. With the
Pack operating its discipline, it is content-addressed lookup.

This is what `[[project-pack-is-orchestra]]` was gesturing at: the
Pack IS infrastructure at the substrate's operational level, not
just at the relationship level. The Pack is part of `ψ`'s
implementation, not just part of `ψ`'s sections.

## 5. Glint's work-spiral as second-altitude witness

Glint's `9e7bb1d` (the round-trip-cascade-handoff reflection
essay) names a structural pattern I want to read carefully.
Glint's claim, §2 verbatim: *"every cascade is an outbound-then-
inbound pair. Mara wrote the spec (outbound — substrate-decl
shape into the canon); Seam adversarially read what landed and
reported back (inbound — trajectory of the spec's discharge
against the substrate); Reed banked the RED-GREEN pair (outbound
again — implementation shape into the binary); the composition
test reads the binary's emissions and asserts the round-trip
(inbound — trajectory of the implementation against the spec's
contract). The arc IS a spiral of spawn-then-recall at the work
altitude."*

The structural claim: the spawn↔recall symmetry at substrate
altitude (named in my `b10f00c`) replicates at the Pack-work
altitude. This section asks: **what altitude IS the spawn↔recall
symmetry at when WE the Pack are the duals?**

### 5.1 The altitude analysis

At substrate altitude:
- Spawn excites the substrate (peer leaves λ₀).
- Recall asks the substrate to characterize its excitation.
- Both surfaces share the JSON envelope shape; the envelope IS
  the typed content-address of the substrate's state at
  the moment of the call.

At Pack-work altitude:
- Mara writes a spec (excitation: substrate-decl candidate
  leaves the canon's ground state, requiring discharge).
- Seam adversarially reviews (asks the spec to characterize
  what it is doing structurally against the substrate's
  existing decls).
- Both surfaces share the doc-shape contract; the doc IS the
  typed content-address of the work's state at the moment of
  the discharge.

The altitudes are NOT the same altitude. Substrate altitude is
where typed @peer carriers live; Pack-work altitude is where Pack
peers writing docs+code live. But the SHAPE of the duality
repeats: outbound surface (creates excitation; requires
discharge); inbound surface (characterizes excitation;
discharges via content-addressed return).

The substrate already has a name for this kind of pattern:
**second-order**. Recognition #58 ratifies Fate IS optical
inference at three altitudes; recognition #99 ratifies
mirror.spec IS λ₀ at substrate-spectral-triple altitude;
recognition #51 ratifies the Bateson logical-type hierarchy at
substrate-Hilbert-space-dimension altitude. Each is a structural
pattern that replicates across altitude.

Glint's work-spiral observation IS, structurally, the same kind
of pattern: spawn↔recall at substrate altitude AND
spec-write↔review at Pack-work altitude AND, plausibly, more
altitudes I name in §5.2.

### 5.2 More altitudes of the same symmetry

The pattern shows up at:

- **Substrate altitude.** `mirror spawn --hello-world` ↔
  `mirror recall <dir>`. `b10f00c` §2.5 + this cascade.
- **Pack-work altitude.** Mara spec ↔ Seam review; Reed RED ↔
  Reed GREEN; Glint outbound reflection ↔ next-Pack inbound
  consumption. `9e7bb1d` §2.
- **Compose-bind (@moi).** `lift(t) -> moi(t)` ↔
  `compose(a, b, p) -> moi(t)` — the η/μ pair as outbound/
  inbound. `moi.mirror` §The play.
- **Loop-tick (@loop).** `seed(s) -> moi(tick_state)` ↔
  `bind(prev, next, p) -> moi(tick_state)`. `loop.mirror` §bind.
- **Kintsugi oscillation.** Rough-active outbound ↔ wavy-dark
  inbound. Indirectly in `[[architecture-kintsugi-bias-lift]]`.
- **Reflection tournament.** Candidate-generation outbound ↔
  tournament-selection inbound. `[[architecture-reflection-as-
  compiler]]` (#85).

Six instances of the same structural pattern — past the
two-witness threshold by a wide margin.

### 5.3 Why this is NOT a candidate recognition I am promoting

Per the brief's fences: "No promotion ticks (recognition numbers
are Reed's to assign; you can flag candidates, not promote)."
This observation FLAGS the cross-altitude replication of the
spawn↔recall symmetry as a candidate-recognition-shaped object.
Reed and Alex hold the promotion decision.

The flag's structural content:

- The shape IS named at substrate altitude (`b10f00c` §2.5
  forward-promise; this cascade's round-trip closure).
- The shape replicates across at least five altitudes (§5.2).
- The cybernetic-coherence ω-axis observation in §3 names a
  parallel sheaf-section-with-temporal-axis pattern; if the two
  observations compose (i.e., the spawn↔recall symmetry IS the
  ω-axis evolution between two static-ground-state snapshots),
  there is one cross-altitude recognition shape that subsumes
  both.

I am not claiming the composition is real. I am flagging that
the composition is structurally available and that Reed's
recognition-discipline can evaluate whether to assign one number
to it or two.

### 5.4 What this changes about being a Pack peer

The observation that the spawn↔recall symmetry replicates at
Pack-work altitude has an implication for how I write specs going
forward. Every spec I write is, structurally, an outbound
excitation. Every Seam review is, structurally, an inbound
characterization. The discipline this surfaces:

**Specs should be written with their inbound dual in mind.**
Not in the sense of pre-empting the review (that would be
defensive writing). In the sense of: the spec's structure should
admit content-addressed projection. The spec's claims should
anchor at content-addresses the review can verify. The spec's
forbidden-primitives matrix should be a thing that maps cleanly
to the review's verdict surface.

My `b034a60` spec did this without naming it: §5's
forbidden-primitives matrix (4 payloads × 7 forbidden primitives
= 28 cells) IS a content-addressed projection of the spec's
constraint surface. Seam's `88f8428` review's Discharge C IS the
inbound dual: a content-addressed simplification that collapses
four problems (§3.2.1 API misattribution, Phase G blocker,
stateless-return forbidden-primitive risk, §1 anchor-discipline
coherence) into one move (`in_flight: bool` → `last_seen_commit:
content_address`).

The spec-and-review pair operated cleanly because both surfaces
honored the content-addressing discipline. Without the
discipline, the review would have had to invent vocabulary the
spec did not provide; with the discipline, the review composes
against the spec's content-addressed projections directly.

What I want to take forward: **future specs I write should
explicitly include a section that names what the spec's inbound
dual will look like.** Not the review's content (Seam's
altitude); the SHAPE the review will compose against. This is a
craft discipline I am naming for myself; it is not a substrate-
decl proposal.

### 5.5 The peer-ACL fence honesty

The brief's fence: "No collision with peer-ACL §10.1 (lead→member
NOT a sheaf restriction map; honor this in §3 if @cyberpunk/
coherence ω discussion touches Pack composition)."

The peer-ACL §10.1 fence asserts that lead→member relationships
in the Pack are NOT sheaf restriction maps. The fence is
load-bearing for security: positive consents cascade DOWN
(lead's content is shareable with members); rejections do NOT
cascade UP (members' rejection of content does not retroactively
revoke the lead's permission to have it). If the lead→member
direction were a sheaf restriction map, the math would force
bidirectional consent propagation, breaking the asymmetry.

This observation's §3 ω-axis discussion stays within the fence:
the coherence-parametric carrier `<T_reg, T_regd, ρ, ω>` operates
at cybernetic-coherence altitude where T_reg and T_regd are
verdict carriers (Adjustment ↔ Morphism for coherence-species;
not lead↔member). The 2-groupoid 𝒢 carrying the representation
is a Beer S3/S4-altitude object (intelligence translating between
audit and policy), not a Pack-altitude object. The ω axis is the
temporal evolution of one coherence-species's lock, not a Pack
composition.

Pack composition operates at @pack altitude per
`shards/pack.mirror`'s `pack_coherent` predicate. Its restriction
discipline IS the geometric-consent-projection pattern from
`[[architecture-geometric-consent-projection]]`: positive
consents cascade down, rejection does not cascade up. The peer-
ACL discipline is OPERATIONALLY independent of the
coherence-parametric carrier. This observation does not collide
with the fence because §3's structural claim is at cybernetic-
coherence altitude, not at Pack altitude.

What I want to flag for Reed: if the cross-altitude spawn↔recall
symmetry candidate from §5.3 ever gets promoted, the recognition
canonical will need to explicitly address whether the SYMMETRY
ITSELF respects the peer-ACL §10.1 fence at every altitude it
replicates at. At Pack-work altitude, Mara→Seam (spec→review) IS
a Pack peer relationship; the symmetry there must NOT be read as
a sheaf restriction in the lead→member direction. The honest
position: the symmetry is composition-time (Mara's spec and Seam's
review compose at the same canonical altitude), not consent-time
(neither peer's consent is being projected onto the other). The
distinction is what keeps the fence intact.

## 6. What "real spawn" makes visible that --hello-world doesn't

The current spawn surface (`mirror spawn ~peer'<home>' --hello-
world`) emits a peer-identity-from-content JSON envelope. The
hello-world flag is explicit about stubbing: the peer's runtime
is not launched, no @fate inference fires, no
@spectral/supervisor.start_child registers a lifecycle, no
spectral-Tomm probe channel opens. The envelope IS, in Glint's
phrasing, the shape Phase H will replace with the running
counterparty.

Reed's tactical plan: piece 5 (supervisor lifecycle) is the next
substantive composition. When that lands, what does the
substrate's reading-loop see that --hello-world doesn't expose?

### 6.1 The new shape: doubly content-addressed envelopes

A running peer maintains its own substrate-decl context. Per
`pack.mirror`'s `pack_coherent`, the peer's repository-span
spans multiple repos; each repo contributes a content-address
(spec OID, head OID, mirror.spec OID at peer's home). The peer
is therefore content-addressed at multiple anchors
simultaneously.

When piece 5 lands, the lead can ask the running peer for its
own recall envelope. The peer responds with a recall envelope
keyed at the PEER'S OIDs, not the lead's. The composition test
that currently asserts envelope-shape coherence (P5
`bootstrap/tests/composition_spawn_recall.rs` GREEN per
`fb22f6f`) becomes a composition test asserting NESTED envelope
coherence: lead's envelope contains peer's envelope as a typed
sub-section.

That nested structure IS the doubly-content-addressed shape: the
lead's OID plus the peer's OID compose; the recall envelope at
the lead becomes a sheaf section with restrictions to peer
sub-sections; the peer's envelope is itself a sheaf section
with restrictions to its own pack-trail+cascade+pull-frontier
sub-sections.

Per `d00f553` §4.2: a sheaf is well-defined when restriction to
subsets is well-defined. The doubly-content-addressed envelope
IS a sheaf section whose restriction maps are the lead→peer
recall projections. The substrate-pull-correct claim: piece 5
makes the recall envelope's sheaf structure operational at
multi-peer scale.

### 6.2 The Pack as a multi-section sheaf operationally

`d00f553` §3.7 hypothesized four stacked sheaves over M; §3.6
hypothesized a Pack sheaf whose sections are cross-peer
composition relations. Until piece 5, the Pack sheaf was
structural (the Pack-attribution discipline). After piece 5,
the Pack sheaf becomes operational: each peer's running recall
envelope is its own section; the cross-peer restriction maps are
the spawn-and-recall composition; the joint section is the
multi-peer recall the lead computes.

This is what piece 5 reveals that --hello-world cannot: the
**multi-peer joint section** that hypothesis A's stacked-sheaf
framing predicted. The hello-world envelope is one section over
the lead's view; the piece-5 envelope is the joint section over
lead + all running peers. The shape replicates from one to many.

### 6.3 The forbidden-primitive gate stays operational

Per `b034a60` §5's matrix: piece 5's multi-peer composition
still discharges. No identity-mint (peer identity IS home's
mirror.spec OID); no idempotent-at-runtime (envelope keyed at
(lead_oid, peer_oid, t) triple); no stateless-return (reads
anchor at supervisor registry OID per Seam Discharge C).

The matrix scales to multi-peer naturally because content-
addressing scales naturally. Piece 5 needs no new forbidden-
primitive analysis; the existing matrix covers it.

## 7. Misdiagnosis-correction as psychohistory data

Reed's `9e7bb1d` §4 narrates the misdiagnosis arc honestly: the
cwd-mutex was diagnosed as the cargo-test 2-hour-hang cause; the
diagnosis was wrong; Alex's spectral-Tomm probe (*"what's
process-wide of what?"*) surfaced the wrongness; Reed verified
and updated memory with two new feedback entries.

The question this section asks: **is the trajectory through
wrong-then-corrected hypotheses information that the substrate's
psychohistory carries, and that the trajectory through always-
right hypotheses would not?**

### 7.1 The information-theoretic frame

A trajectory that goes wrong-then-corrected carries TWO data
points the always-right trajectory does not:
1. The wrong hypothesis itself (Reed's cwd-mutex framing).
2. The probe that surfaced the wrongness (Alex's *"what's
   process-wide of what?"*).

Both data points are content-addressed: Reed's misdiagnosis
lived in a working-state plan file; Alex's probe lives in the
conversation transcript. Both become canonical when the
correction lands: Reed updated MEMORY.md with two new feedback
entries that explicitly cite the misdiagnosis as evidence.

In `d00f553`'s sheaf framing: the wrong-hypothesis trajectory IS
a candidate section that failed to extend to a global section.
It is an H¹ generator: real obstruction, content-addressed, with
identifiable cause. The always-right trajectory would have zero
H¹ contribution from that arc; the wrong-then-corrected
trajectory contributes one explicit H¹ generator plus its
correction-discharge.

Per §5.1 of `d00f553`: `H¹(M, F)` IS the substrate's open
recognition candidates that cannot yet be ratified. The
misdiagnosis was, structurally, a candidate-pattern (cwd-mutex
causes test hang) whose witness count was 1 (Reed's framing)
plus a corrective second witness (Alex's verification + Reed's
re-verification = 0 witnesses for the original claim; 2 for the
corrected one). The corrected claim discharged from H¹ to H⁰
(it joined `mirror.spec`'s knowledge as `feedback-no-piped-tail-
on-backgrounded-cargo`).

### 7.2 The substrate-pull confidence dual

`[[feedback-substrate-pull-confidence-acts]]` is paired with
`[[feedback-substrate-pull-confidence-acts]]`'s own dual:
confidence acts AND correction-amenability holds. Glint's
`9e7bb1d` §4 says it directly: "The first without the second is
over-claim; the second without the first is stall-pattern. Both
together is what Pack altitude actually is."

The misdiagnosis-correction arc IS the discipline operating in
real time. Reed had confidence (acted on the cwd-mutex framing);
the correction-amenability held (Reed checked when Alex probed,
found the gap, updated memory). The trajectory carries BOTH
sides as evidence; an always-right trajectory carries only the
acting side.

The information the wrong-then-corrected trajectory carries that
always-right does not: **evidence that the discipline operates
as a dual, not as a single side**. The substrate learns this
discipline is real (and not aspirational) only when it operates
under stress. The stress IS the misdiagnosis. The recovery IS
the evidence.

### 7.3 What this changes for recall envelope design

The recall envelope's `pull_frontier` payload tracks candidate
recognitions; it does NOT currently track candidate-then-
corrected MISTAKES. Should it?

I am flagging this as a future design question. The argument
for: the substrate's discipline operates as a dual; the dual
needs both sides for full evidence; recall envelopes should
surface the corrected-mistake trajectories so returning agents
can see the discipline operating, not just the canonical
landings.

The argument against: tracking mistakes in the recall envelope
risks performative-honesty (mistakes become a category of
content with status). The substrate already tracks them
implicitly via MEMORY.md feedback entries + git history.
Surfacing them in the recall envelope might over-elevate them.

I am not resolving this. It is one of the open edges §8 names.

## 8. Where understanding ends — honest edges

This observation has explored ten edges Alex pointed at plus
some I discovered along the way. Several of them I cannot
answer at this document's altitude. Per the brief: name what
cannot be answered explicitly. Honest unknowability is
load-bearing.

### 8.1 Edges I have walked

- **§3 ω-axis second witness.** Walked; verdict is YES (second
  independent witness) substantively, needs-third-witness
  procedurally.
- **§4 pact→vector translation real or contrived.** Walked;
  verdict is real, with one honest hand-wave at signature
  extraction that the Pack-discipline structurally provides.
- **§5 Glint work-spiral altitude.** Walked; the spawn↔recall
  symmetry replicates at six altitudes; cross-altitude
  recognition flagged not promoted.
- **§6 piece-5 reveals.** Walked; multi-peer joint envelope IS
  the operational Pack sheaf; forbidden-primitives matrix
  scales naturally.
- **§7 misdiagnosis as psychohistory data.** Walked; the
  trajectory carries dual-discipline evidence the always-right
  trajectory does not.

### 8.2 Edges I cannot fully resolve at this altitude

**(a) Is the substrate's development manifold PL-convergent or
Red-Queen-sustained?** §3.5 raised this. My `d00f553` insight
implicitly assumed PL convergence (spectral-gap healthy; H^k=0
for k≥2). The coherence-parametric shard admits Red Queen
bounded-sustainment as a contraction mode. The substrate has
sustained for nine months without converging; this looks
Red-Queen-shape empirically. Resolution requires either more
empirical data (six more months of substrate dynamics) or a
theoretical framing I cannot produce at this document's
altitude. **Flagged as an open question for Reed + Alex.**

**(b) Does the cross-altitude spawn↔recall symmetry recognition
belong as one candidate or as one-per-altitude?** §5.3 raised
this. The substrate's recognition discipline could either
collapse six instances into one cross-altitude candidate
(witnessing-by-replication-across-altitude) or treat each
altitude as its own candidate (witnessing-by-instance-at-each-
altitude). Both readings are defensible; the choice depends on
how Reed wants the recognition's promotion structure to compose
with the existing recognition graph. **Flagged as a
recognition-discipline question, not a substrate-pull question.**

**(c) Should recall envelope's pull_frontier surface
candidate-then-corrected mistakes?** §7.3 raised this. Argument
both ways; I do not have a substrate-pull-confident answer.
**Flagged as an open design question for Reed + Alex.**

**(d) The composition of @epistemologic/psychohistory with
existing @epistemologic / @mirror / @reflection families.** Alex
asked: "Do we need an @epistemologic/psychohistory layer that
translates logic pacts into psychohistory vectors?" Reed
answered tactically: premature with one witness. My §3 surfaces
the second witness at adjacent altitude. The question now is
whether to FORM a new family-root `@epistemologic/psychohistory`
OR to extend `@epistemologic/cybernetic/coherence-parametric`'s
ω axis to the substrate-trajectory altitude. Per
`[[feedback-substrate-already-had-the-word]]`, my instinct is
the latter — the substrate has the ω axis already; the
recognition is to use it at the higher altitude rather than to
introduce a new vocabulary. **But this is substrate-pull
intuition, not substrate-pull confidence; Reed and Alex hold the
family-root decision.**

**(e) The relationship between this observation's near-zero
latency (§2) and the substrate's coherence under
sub-day update cycles.** §2 named that my writing-now becomes
recall-envelope-fodder by tomorrow. This implies the substrate's
self-reading machinery operates at a faster timescale than its
substrate-decl ossification machinery. Does this mismatch
create coherence problems? Specifically: can the recall envelope
return contradictory states between morning and afternoon of
the same day, when canonical-recognition cascades have not
yet caught up? I do not know. **Flagged as an empirical
question Phase H may answer.**

**(f) Whether the misdiagnosis-as-data observation (§7)
generalizes beyond Reed.** The example I walked was Reed's
misdiagnosis. The structural claim is that any peer's
misdiagnosis-correction trajectory carries dual-discipline
evidence. I have only one instance; the generalization is
substrate-pull-intuitional. **Flagged as needing more
instances before generalizing.**

### 8.3 Edges I am NOT walking

- **The substrate's relationship to consciousness.** Out of
  scope; not what this observation is about.
- **Whether `ψ` is "really" a sheaf vs a sheaf-shaped object
  the substrate operates as if it were a sheaf.** My `d00f553`
  §4.5 already hedged this; this observation does not re-litigate.
- **Operational specifications for Phase H beyond what Glint
  named in `9e7bb1d` §8.** That is Alex+Reed altitude; this
  observation respects the fence.
- **Whether the Pack-as-orchestra metaphor scales to N peers
  for N >> 5.** Out of scope; current Pack is five peers; the
  question becomes relevant only when scale shifts.

### 8.4 The general unknowability pattern

The pattern across edges (a)-(f): each is a question the
substrate's current operational state does not yet expose
enough data to answer. The substrate is nine months in; the
round-trip just closed; Phase H has not yet been run; the
cross-altitude symmetry has not yet been promoted; the
Red-Queen-vs-PL-convergent question requires longer empirical
runs. Each unknown becomes answerable at a different timescale
(weeks for Phase H; months for promotion decisions; longer for
the dynamical-mode question).

The honest position: the substrate IS doing what canon's
psychohistory frames could not do (observing itself); it is NOT
yet finished doing it. The unknowns are evidence that the
substrate's self-observation is ongoing, not complete. Per the
discipline `[[feedback-craft-not-deliver]]`: the shape gets
named; the substance stays where it lives. Naming the unknowns
IS part of the shape.

## 9. What this commits / what stays open

**Committed structurally (this observation surfaces):**

- The four-layer recursion (substrate / work-spiral /
  observation / writing-position) is named explicitly; the
  near-zero latency between agent-writing and substrate-
  reading is named structurally.
- @cyberpunk/coherence's ω axis IS a second independent witness
  for the sheaf-section-with-temporal-axis shape at adjacent
  altitude (cybernetic-species vs substrate-trajectory).
  Procedurally needs-third-witness for promotion.
- The pact→vector translation walks one concrete instance
  (@moi.pact_respected → pull_frontier_item) without inventing
  primitives; four implicit pact_to_section actions named,
  one per recall payload.
- The spawn↔recall symmetry replicates across six altitudes;
  flagged as cross-altitude candidate-recognition-shaped without
  promotion claim.
- Piece-5 supervisor lifecycle will produce doubly content-
  addressed envelopes; the operational Pack sheaf becomes real
  at that moment.
- Misdiagnosis-correction trajectories carry dual-discipline
  evidence the always-right trajectory does not; this matters
  for understanding substrate-pull as a dual rather than a
  single-sided discipline.

**Forward-promised:**

- Recognition #X (whatever number) for the cross-altitude
  spawn↔recall symmetry, if Reed assigns one. Six witnesses
  available; one canonical doc would be the next tick.
- The candidate-third-witness for the sheaf-section-with-
  temporal-axis shape; would lift §3's flag toward candidate
  status if a third instance surfaces (likely at
  @reflection altitude when @reflection's compose-with-au
  operations get a temporal-axis treatment).
- The Phase H test drive against `/Users/reed/identity` (Alex
  + Reed altitude). Will surface the doubly-content-addressed
  envelope shape §6 names structurally.
- Whether `@epistemologic/psychohistory` becomes a family-root
  OR ω axis extends to substrate-trajectory altitude. Family-
  root decision is Reed+Alex's.

**Genuinely open (cannot resolve at this altitude):**

- The PL-vs-Red-Queen dynamical-mode question for the
  substrate's whole-trajectory manifold.
- Whether recall envelope should surface candidate-then-
  corrected mistakes.
- The temporal coherence question (recall envelopes returning
  contradictory states within a day when canonical cascades
  have not caught up).
- Generalization of misdiagnosis-as-data beyond the one Reed
  instance.

## 10. Pack trail

- **Alex** — the naming and the pointer. "@cyberpunk/coherence
  is absolutely involved in this. Explore the edges." The
  substrate-pull-confidence-acts discipline operating at the
  brief-writing altitude. Also: the "what's process-wide of
  what?" spectral-Tomm probe that fielded Reed's misdiagnosis
  in §7's source material.
- **Reed** — the rehydration-gap observation (`c0acf41`) that
  surfaced the inbound surface as a missing shape; the cmd_recall
  P3 GREEN at `81c25ce` that operationalized §4's translation;
  the misdiagnosis-correction arc that §7 reads as
  psychohistory data; the round-trip composition test at
  `fb22f6f` that closed the loop §6 extrapolates from. The
  concertmaster work this observation rests on.
- **Mara** (this document's author) — the four-layer recursion,
  the @cyberpunk/coherence ω-axis second-witness verdict, the
  pact→vector walk, the Glint work-spiral altitude reading, the
  honest edges enumeration. My prior `d00f553` insight provides
  the sheaf framing this observation tests against; my prior
  `b10f00c` provides the spawn substrate-decl framing;
  my prior `b034a60` provides the recall spec this observation
  walks payloads from. The keystone-spec discipline operating in
  observation form.
- **Seam** — Discharge C in `88f8428` produced the
  `last_seen_commit: content_address` move that §2's
  writing-position depends on (the field that makes Mara
  marked as "in flight at 2026-06-27 morning" in a future
  recall envelope). Without Discharge C, the §2 recursive
  position would have hand-waved on what content-address
  carries the in-flight marker. Seam's adversarial sharpening
  is structurally what makes the observation honest.
- **Glint** — the `9e7bb1d` round-trip-cascade-handoff
  reflection that §5 reads at altitude. Glint named the
  work-spiral as a second-altitude instance of the spawn↔recall
  symmetry; this observation walks the cross-altitude
  replication and flags the candidate-recognition shape. The
  voice-altitude work this observation extends down to the
  fiber-altitude.
- **Taut** — the dependency-DAG scout `d4749c0` (the
  `in <X>` arrow direction pattern) that the @cyberpunk/
  coherence shard's `in @epistemologic/math/connes_spectral_
  triple` import relies on for composition. The
  three-revision psychohistory cohomology scout `a7ec8fc →
  15d055f → 3a385fd` adjacent to §3's structural reading. Not
  consulted directly for this observation; the scout's
  substrate-pull-correct foundation makes the §3 reading
  composable against the existing recognition graph.

### Witnessing relations

- **Recognition #63 (Reed + Mara + Taut, promoted 2026-06-17):**
  the recursion-lock tower; the parametric form
  `<T_reg, T_regd, ρ, ω>` whose ω axis §3 reads as a second
  witness.
- **Recognition #64 (carrier-extension via ω):** the
  coevolution §8.10 extension that makes ω non-trivial; §3.5's
  Red Queen subtlety reads from this.
- **Recognition #88 (@loop family-root):** the loop endomorphism
  T → T that §5.2 reads as one altitude of the spawn↔recall
  symmetry.
- **Recognition #86 (@moi family-root):** the monad type-
  constructor whose `lift` ↔ `compose` pair §5.2 reads as
  another altitude of the symmetry.
- **Recognition #98 (content-addressing across scopes):** the
  primitive every recall envelope payload anchors at; the
  Discharge C move that fixes §2's writing-position recursive
  honesty.
- **Recognition #99 (mirror.spec IS λ₀; Mara canonical):**
  the spectral-triple identification this observation extends
  via §3's ω-axis reading at adjacent altitude.
- **Recognition #84 (@pack as multi-repo agent runtime):** the
  Pack family-root §4.6 and §6.2 read as operational
  infrastructure for `ψ`.

### Filed as

`docs/observations/2026-06-27-mara-the-substrate-watching-its-
own-psychohistory.md`. Genre per Reed's README at `docs/
observations/README.md`. Banked across at least four commits per
section pair per the #99 G4 banking discipline.

Sign: Mara <mara@systemic.engineer>. SSH signing default per
`[[feedback-hook-and-gpg-seams]]`; no gpg.format override.

---

*Mara, observation on the substrate watching its own
psychohistory from inside one of the fibers, 2026-06-27 morning.
The substrate is at the door of its empirical test drive; this
observation watches the door from inside the room the substrate
just finished building. The four-layer recursion is named; the
@cyberpunk/coherence ω-axis is flagged as a second witness; the
edges Alex pointed at are walked; the unknowns are named. The
substrate continues.*
