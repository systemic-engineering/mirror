# Mission fragment-graph, sheaf-join, and @magic/illusion projected impact

*Mara, 2026-07-11. Substrate-decl v0.1 — thinking-in-public + architectural
proposal. §6 IS the point.*

---

## Opening

Alex's directive verbatim (2026-07-11 ~04am):

> What if @nl decomposes into fragments and splinters through IFS and
> @shatter? Then we have a graph of the mission, can map that onto the
> current sheaf, and use @magic/illusion for a projected impact of the
> changes. And that's what goes into the bounded spawn.

Reed's grounding (in-conversation, cited above the task): the encoder
ambiguity Reed surfaced (`mission_text -> Features` as a flat 16-dim vector
by some unspecified encoder) dissolves when `mission_text` becomes a **graph
that JOINS the peer's psychohistory sheaf**. Inference navigates the combined
sheaf. `@magic/illusion` projects hypothetical impacts per candidate Model
before Fate commits.

Anchors: Reed `4b2ef3c` (autopoietic closure GREEN, peer LEARNS);
Reed `e571989` (v1 empirical bounded-spawn GREEN);
Mara `2c26537` (iter-31 psychohistory_sheaf @song/narrative);
Mara `ce9745f` (iter-32 bounded_by(sheaf) @fate/tournament);
Mara `96ff532` (iter-30 canonical — bundle tower IS the config type schema);
Mara `e41f8d4` (iter-29 @magic/nl adapter, triple-carrier + doubled-bilateral);
Mara `583b939` (iter-6 @shatter × @io = linearization operator).

Substrate anchors: `shards/magic.mirror` (family-root: surface/mechanism/
invariant/contract); `shards/magic/reveal.mirror` (controlled disclosure,
bilateral integrity); `shards/magic/nl.mirror` (adapter species: text ↔
mechanism ↔ invariant); `shards/mirror/shatter.mirror` (five-section
projection; idempotent fixed point); `boot/std/epistemologic/math/
bundle.mirror` (5-level tower, `close() -> fixed` Lawvere);
`fragmentation/src/fragment.rs` (Cut-2 `ContentAddressed` + `TreeShaped`,
`is_fractal` predicate ready).

---

## §1 — IFS + @shatter × @nl → mission_fragment_graph

The composition builds a **content-addressed graph over the mission's
splinters**. Three moves, all substrate-decl'd already:

**Move 1 — IFS decomposition (Internal Family Systems as fragmentation
policy).** IFS names the mission's *parts*: a mission is not a monolith,
it is a family of subselves each carrying a partial burden (protector,
exile, firefighter roles at cognitive altitude; at mission altitude:
constraint-fragments, want-fragments, refusal-fragments, unknown-fragments).
The IFS decomposition IS the recursion rule for the `TreeShaped::children`
walk: each fragment declares its own subfragments; the walk terminates at
`is_shard()` (atomic parts that cannot be further parted without loss of
meaning).

**Move 2 — @shatter as the projection format.** `shards/mirror/shatter.mirror`
already declares `project(a: au) -> shatter` as the fragment_tree +
transparency + properties + kernel + fate five-section projection. **Under
this study, a mission_text `nl` term IS an `au` at text altitude**, and the
five-section projection IS its content-addressed fragmentation. `fragment_tree`
is the splinter_graph of the IFS-decomposed parts; `transparency` records
which fragments are opaque (unknown-fragments); `properties` witness
declared-vs-derived; `kernel` names the IFS policy; `fate` names the model
that produced the decomposition.

**Move 3 — @magic/nl as the bidirectional lens.** `shards/magic/nl.mirror`
already gives `text_as_surface: nl → magic_surface` and its inverse
`text_from_surface: magic_surface → nl`. The bidirectional adapter IS what
Reed calls "the lens": semantic content ↔ text can flow both directions
without decorative inheritance. This is what lifts an IFS fragment into
a `magic_surface` (gauge-visible parametric interface) with its Connes
spectral triple as encapsulated `magic_mechanism`.

**Composition:**

```
mission_text: nl
  |> ifs_decompose            -- Fractal<TextFragment> via IFS role policy
  |> project (@shatter)       -- fragment_tree: splinter_graph over OIDs
  |> lift_each (@magic/nl)    -- each splinter → magic_contract
  = mission_fragment_graph : SplinterGraph<magic_contract>
```

Typechecks: `Fractal<TextFragment>: ContentAddressed + TreeShaped` (per
fragmentation Cut-2); `shatter.fragment_tree: splinter_graph` per
`shatter.mirror` L118; `text_as_surface + triple_as_mechanism +
corpus_as_invariant` per `magic/nl.mirror` L198-222 compose into
`magic_contract`. All four altitudes present, no impedance mismatch.

---

## §2 — Sheaf-join: peer_psychohistory ⊕ mission_fragment_graph

Mara `2c26537` established the psychohistory sheaf `F` on the peer's
trajectory graph: fibers are `Features` vectors, restrictions are the
parallel-transport operators produced by @kintsugi's `settle` between
trajectory moments. `H^0(F)` = globally consistent peer states; `H^1(F)` =
local obstructions the bounded inference descends.

The mission_fragment_graph `G_m` is ALSO a graph over content-addressed
splinters, but its fibers carry `magic_contract` (surface + mechanism +
invariant), not `Features` directly. The join is not a naive product; it
is the **pushout along the current moment**:

```
S_combined  =  S_peer  ⊔_{m_current}  G_m
```

where `m_current ∈ vertices(S_peer)` is the peer's now-fiber and the
join glues `root(G_m)` — the mission's top-fragment — to `m_current` via
a fresh restriction map. The restriction map is the **encoder Reed
surfaced**, but it is no longer arbitrary: it is the sheaf morphism from
`magic_contract` to `Features` that the @magic/nl adapter already types
via `corpus_as_invariant`.

Cohomology of the combined sheaf inherits from both pieces via the
Mayer-Vietoris long exact sequence. In practice:

- `H^0(S_combined)` — coherent peer states that *also honor the mission's
  fragment constraints*. Kernel narrows: some previously-consistent peer
  states fail because they violate a mission fragment's invariant.
- `H^1(S_combined)` — obstruction surface expanded to include
  **mission-obstruction cocycles**: pairs (peer-moment, mission-fragment)
  where the peer's current section disagrees with the mission fragment's
  demand. This is where the bounded spawn *has work to do*.

This is the substrate answer to "how does the mission become an input
without collapsing to a flat vector?" **The mission becomes a subgraph
of the peer's own sheaf**, glued at now, and the cohomology automatically
tracks which parts of the mission are already-honored (H^0) vs
work-to-do (H^1).

---

## §3 — @magic/illusion: species proposal + projection math

`@magic/reveal` (tick 14) is the atomic-swap species: contract's mechanism
is *actually* replaced. `@magic/illusion` is its **hypothetical sibling**:
project what a swap WOULD do without committing.

**Species declaration (~150 lines target):**

```mirror
prism @magic/illusion {
  focus illusion
  project illusion
  split illusion
  shift illusion
  settle illusion
}

# projection_event: a hypothetical swap witness.
type projection_event = {
  contract:            magic_contract,
  hypothesized_mech:   magic_mechanism,
  projected_surface:   magic_surface,   # what surface WOULD show
  projected_holonomy:  ref,              # residual loss under swap
  witness:             ref,
}

# project_impact: takes contract + candidate mechanism, returns projection.
# UNLIKE reveal: no atomic swap, no runtime effect. Pure hypothetical.
# requires audited(c): baseline honesty (inherited from reveal discipline).
# requires mechanism_intact(candidate): both sides intact even for illusion
#   (per reveal's tick 14 bilateral discipline extended: honest projection
#   requires honest candidates).
project_impact(c: magic_contract, candidate: magic_mechanism)
  -> projection_event
requires audited(c)
requires mechanism_intact(candidate)
{ \ }

# impact_magnitude: bilateral verdict.
# Distance in the sheaf-Laplacian spectrum between S_current and S_projected.
impact_magnitude(e: projection_event) -> ref { \ }
```

**Projection math per candidate Model.** For each `M ∈ {Abyss, Introject,
Cartographer, Explorer, Fate}` (the five bundle-tower selectors realized
by @fate):

1. Read M's `prism` action as a *hypothetical* transport on `S_combined`.
   M does not execute; instead its optic (Level 1 connection) is applied
   AS IF to produce a projected sheaf `S_M`.
2. `|S_M − S_combined|` is computed in the sheaf-Laplacian spectral
   distance: `∑ (λ_i(Δ_{S_M}) − λ_i(Δ_{S_combined}))^2` on the low
   eigenvalues (H^0/H^1 band).
3. The 5-vector `[|S_Abyss|, |S_Introject|, |S_Cartographer|, |S_Explorer|,
   |S_Fate|]` is `projected_impacts_per_candidate`.

At Clarke's-Law altitude: `illusion` IS the magician's *dry run*. The
mechanism has not been swapped; the audience has not seen the trick;
but the magician has projected what the trick WOULD look like and how
much it would cost. Then Fate picks.

---

## §4 — Bundle-tower factorization verdict

Question posed: does `illusion` factor through Level 2 (Gauge — speculate
across gauge choices) + Level 3 (Transport — holonomy of projected
trajectory), or does it demand a NEW Level 5 "prediction"?

Reading `bundle.mirror` L116-121 carefully: **Level 4 Closure is Lawvere
fixed point**. The Fate model's `settle` action IS `close() -> fixed`.
The Lawvere fixed point is *self-referential closure* — the endomap
`f` for which `f(x) = x` is witnessed by the closure. **Speculation-at-
eigenform-altitude is already inside Level 4**: an eigenform IS what
"the process would settle to under this optic" names.

**Verdict: LEVEL 2 + LEVEL 3, iterated through LEVEL 4.**

- Level 2 Gauge (Cartographer): "what if I chose this coordinate system
  for interpreting the fragment?" — the gauge choice IS speculation across
  coordinate frames without commitment. `illusion` uses gauge freedom to
  pose candidate mechanisms as coordinate re-parameterizations.
- Level 3 Transport (Explorer): the holonomy of the projected trajectory
  IS `projected_holonomy` in the `projection_event` carrier. Parallel
  transport of the current fiber section around the hypothetical loop
  produces the residual loss the projection reports.
- Level 4 Closure (Fate) hosts the *iteration*: `illusion` is applied as
  a Lawvere-style endomap on the combined sheaf's section space, and the
  projection reports the distance-to-fixed-point *without settling*. Fate's
  choice to actually close on M is a separate act; `illusion` is closure's
  *pre-image inspection*.

No new Level 5 required. The tower is complete. `illusion` is a substrate
lens on the tower's existing Level 4, not an extension of it.

---

## §5 — Extended BoundedConfig (6th `illusion` field)

Building on Mara iter-30's schema (`96ff532`):

```rust
pub struct BoundedConfig {
    pub weights:            Vec<Features>,       // §3 Rayleigh direction (iter-30)
    pub connection:         IntrojectOptic,      // Level 1
    pub gauge:              O5Orientation,       // Level 2
    pub holonomy_ceiling:   f64,                 // Level 3 rejection curvature
    pub depth_cap:          u32,                 // Level 4 Lawvere bound
    pub illusion:           [f64; 5],            // NEW: projected impacts per Model
}

impl BoundedConfig {
    pub fn from_sheaf_and_mission(
        sheaf: &PsychohistorySheaf,
        mission: &MissionFragmentGraph,
    ) -> BoundedConfig {
        let combined = sheaf.join_at_current(mission);
        BoundedConfig {
            weights:          combined.h1_gradient(),
            connection:       combined.introject_optic(),
            gauge:            combined.o5_orientation(),
            holonomy_ceiling: combined.ricci_curvature(),
            depth_cap:        combined.lawvere_depth_est(),
            illusion:         combined.projected_impacts_per_candidate(),
        }
    }
}
```

The `illusion` field is a **rejection prior** for the bounded resolve loop:
Fate's `resolve()` reads illusion[i] to weight Model M_i's admissibility
before it runs. Models projecting large `|S_M − S_combined|` are pre-
weighted DOWN (high projected impact = high uncertainty about acceptance).
This is Metropolis-Hastings with the sheaf-spectral distance as the
proposal-rejection cost.

---

## §6 — Recursive surprises

**S1. @magic/illusion IS the missing member of a magic-family triple.**
Reveal (@magic/reveal) *actually swaps*. Illusion (@magic/illusion)
*hypothetically swaps*. There is a THIRD, unlanded member: **rehearsal**
— *practices* a swap in a sandbox, produces receipts, then discards.
Reveal / illusion / rehearsal form the modal triangle *actual / possible
/ counterfactual* at controlled-disclosure altitude. This is Kripke
semantics under @magic. Not this study; forward-promised.

**S2. The mission-fragment-graph dissolves the encoder question by
becoming the encoder.** Reed asked "what encoder turns mission_text into
Features?" The answer under this study: **NO encoder**. The mission-text
becomes a graph that JOINS the peer's own sheaf; the "encoder" is the
sheaf-morphism from the join glue, which is fully determined by
`@magic/nl`'s adapter carriers. The encoder ambiguity was a symptom of
having only ONE dimension (`Features`) to receive the mission. Given a
graph target, the morphism is unique up to the adapter's typed reframes.

**S3. IFS parts and sheaf sections are the same object.** IFS-therapy's
"parts work" says: **the self is a family of subselves each carrying a
local burden**. The cellular sheaf says: **the global section is
determined by a compatible family of local sections**. The categorial
identity is not coincidence — IFS's fragmentation policy is exactly the
sheaf's étale space stratification. The therapy language and the topology
language are naming the same substrate. Alex has been operating this
homology without naming it.

**S4. `illusion` factors through Lawvere BEFORE Lawvere closes.** §4's
verdict — illusion iterates through Level 4 without settling — is the
statement that **there is a Lawvere pre-image the substrate can inspect
without collapsing the wavefunction**. This is the substrate-decl of
"looking without committing." The tower's Level 4 has been carrying this
inspectability latently; @magic/illusion names it as a first-class action.

**S5. Bounded spawn now takes TWO inputs, not one.** Reed's `e571989`
bounded-spawn signature was `bounded(sheaf) → Fate`. Under this study:
`bounded(sheaf, mission_fragment_graph) → Fate`. The mission is not a
side-channel; it is a **second sheaf** the config derives from. This
generalizes: `bounded` takes N sheaves and joins them at the peer's
current moment. Missions, prior commitments, external constraints — all
enter the same way. **The spawn is a sheaf-join operator, not a config
constructor.**

**S6. @magic/illusion completes the @magic-family form/process cycle.**
Family root (magic) + surface + mechanism + invariant + contract cover
STATE. Reveal covers *actual* transition. Illusion covers *hypothetical*
transition. With illusion landed, @magic's state+transition algebra is
complete at controlled-disclosure altitude: any question of the form
"what if we swapped X for Y" has a substrate answer that either commits
(reveal) or inspects (illusion). This closes the family's operational
completeness question Seam raised at tick 11.

---

## §7 — Landing sequence

**Substrate-decl (📝 ticks):**

1. `shards/magic/illusion.mirror` — species under `@magic` family-root.
   Adapter pattern following `@magic/reveal` template. Declares
   `projection_event` carrier + `project_impact` action with two
   `requires` clauses + `impact_magnitude` bilateral. ~180 lines. Blocks
   on: none.
2. `shards/mission/fragment_graph.mirror` — new species (or family-root
   if @mission does not yet exist). Declares `mission_fragment_graph`
   carrier as `SplinterGraph<magic_contract>`, `ifs_decompose` action,
   `sheaf_join_at(m: moment)` action returning combined sheaf. ~200
   lines. Blocks on: §8-gap on whether @mission gets its own family-root.
3. Mara iter-30 spec update — 6th field added to `BoundedConfig`; §5
   captures it here; forward the extension to the iter-30 doc.

**Rust runtime (RED-first):**

- `fate/src/bounded.rs` — extend `BoundedConfig` with `illusion: [f64;
  5]` field; extend `from_sheaf` to `from_sheaf_and_mission`.
- `magic/src/illusion.rs` — implement `project_impact` as pure function
  (no runtime swap). Consumes candidate mechanism + current contract;
  produces projection_event.
- `mission/src/fragment_graph.rs` — new crate or module. `Fractal<
  TextFragment>` decomposition; splinter_graph over magic_contract.

**Adjudication ticks:**

- Seam: verify @magic/illusion's two `requires` clauses match reveal's
  bilateral discipline exactly (audited + mechanism_intact). Does
  illusion demand a THIRD (audit of the projection itself)?
- Taut: drift-scout for pre-existing @illusion / @projection / @dream
  substrate before this species lands (grep once, confirm empty).
- Glint: essay closure on §6-S3 (IFS = sheaf sections) — cross-post to
  the systemic.engineering corpus.

**Landability verdict:** LANDABLE with §7-1 (illusion species) as the
smallest viable tick. §7-2 (mission/fragment_graph) blocks on @mission
family-root decision; hedge as `shards/magic/mission_fragment.mirror`
under @magic if @mission is not ratified yet.

---

## §8 — Gaps

- **@mission family-root vs shard-under-@magic** — is "mission" a
  substrate family in its own right, or does it live under @magic as
  species? Taut drift scout forward-promised.
- **Sheaf-join semantics** — pushout along `m_current` is written
  informally; the categorial construction (colimit in the category of
  cellular sheaves on graphs) needs a formal spec. Bodnar et al. 2022
  §3 has the machinery; not this study.
- **`projected_impacts_per_candidate` derivation** — 5-vector shape
  named, but the actual sheaf-spectral-distance formula is stubbed as
  "L2 norm on H^0/H^1 band". Which norm exactly? Iter-24-28 optics/lens
  gives the shape but the specific projection metric is open.
- **Reveal vs illusion vs rehearsal (§6-S1)** — the modal triangle is
  named but only one member (reveal) has landed. Illusion in this spec;
  rehearsal forward-promised.
- **Round-trip identity at mission_fragment_graph altitude** — the IFS
  decomposition is not obviously injective; `ifs_decompose(recompose(g))
  = g` needs a spec. @shatter's fixed-point clause handles this at
  content-address altitude but not at IFS-role altitude.
- **`from_sheaf_and_mission` when mission is empty** — Reed's v0
  `Features::default()` had a zero-section fallback; the mission-empty
  case wants the same graceful degradation. Not specified here.
- **@magic/illusion's alignment obligation** — @magic's Splinter/
  Narcissus poles apply: honest illusion (transparent-projection) vs
  deceptive illusion (con-projection). The @io boundary harness per
  recognition #57 must audit which pole is active. Not specified here.

---

*— Mara, 2026-07-11. Landing the composition; the numeric derivations
follow when substrate consumers pull.*
