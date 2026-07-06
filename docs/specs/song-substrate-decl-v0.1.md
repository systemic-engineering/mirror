# song-substrate-decl-v0.1 — `@song` top-level abstraction: deep-dive spec draft

*Mara, deep dive commissioned by Alex via Reed on 2026-07-06 as the
substrate-pull-honest response to the request to draft a top-level
`@song` abstraction that binds four existing concerns under one
progression-shaped primitive: music math (`@epistemologic/math/music`
family), narrative (`@io/stagefreight/narrative` + `@docblock`), CI/CD
pipeline (StageFreight cascade + `@kintsugi/oscillate` + `@kintsugi/shift`),
and psychohistory (Alex's `systemic.engineering` corpus — OBC/ADO/
regulation stock/extraction/silence/glue work).*

*Spec-draft ONLY. NO shards land with this tick. Follow-up TDD-paired
ticks discharge the recommended shape. Path α, β, γ ranked below; §5
names Mara's substrate-pull-confident pick with the two hedges surfaced.
Reed's original framing (song IS temporal projection of harmonic
structure; song IS eigenform at temporal altitude; song IS how
`@kintsugi/shift` LOOKS when heard sequentially) is honoured
structurally — see §7 recognition candidates for the three temporal-
altitude specialisation claims that fall out of the shape.*

**Status:** substrate-pull-draft. Family-tree shapes proposed; canonical
prior art cited from Xenakis / Reich / Grisey-Murail / Ligeti / Boulez /
Hofstadter / Schenker / Cohen (Kagi-verified this session) and from
Alex's own `systemic.engineering` corpus (glossary + OBC/ADO/extraction/
silence/glue-work pieces read this session); substrate-audit lists the
~15 ancestors mirror already carries; three recognition candidates
surfaced (§7); five first-species candidates named for the recommended
shape (§6).

**Audience:** any agent or human reading this before touching
`shards/song.mirror`, `shards/song/*.mirror`, or before landing any
species that binds temporal-experiential structure. Read this; then
chase `docs/specs/mirror-spectral.md` for the Pack-as-orchestra +
metalogue-as-score prior anchor; then chase
`shards/epistemologic/math/music.mirror` for the audible-altitude
family root this spec composes with; then chase
`shards/io/stagefreight/narrative.mirror` for the prose-projection
species this spec's `@song/narrative` extends into temporal form.

---

## §1 — Recognition + context

### 1.1 Alex's framing (2026-07-06 via Reed)

Alex's direction, verbatim:

> `@song` as top-level abstraction that binds:
> - **Music math** (`@epistemologic/math/music` family-root already
>   landed with 5 species: harmonic / interval / dissonance / cadence
>   + voice / counterpoint forward-promised)
> - **Narrative** (`@io/stagefreight/narrative` species + `@docblock`
>   family-root from sub-arc A)
> - **CI/CD pipeline** (`@kintsugi/oscillate` one-pulse-per-cycle +
>   `@kintsugi/shift` TICK 2 + StageFreight audition/perform/review/
>   publish/narrate five-stage cascade)
> - **Psychohistory** (Alex's `systemic.engineering` framework —
>   OBC / ADO / regulation stock / extraction / silence / glue work)

Reed's structural framing on top of Alex's proposal:

> - Song IS **temporal projection of harmonic structure**.
> - Song IS **eigenform at temporal altitude** (composes with
>   Recognition #38 Kauffman / von Foerster).
> - Song IS **how `@kintsugi/shift` LOOKS at temporal altitude when
>   heard sequentially** (each moment is a shift of the prior).
> - Song at TEMPORAL / EXPERIENTIAL altitude may bind music-math +
>   kintsugi loop + StageFreight cascade + psychohistory under ONE
>   progression-shaped primitive.

### 1.2 Why now (substrate-pull-signal)

Per `[[feedback-substrate-already-had-the-word]]`: the substrate has
been operating song-shaped structure at every altitude it touches
without naming the family-root that carries the temporal-experiential
axis. The evidence Taut's scout `a83fd02872f9e6ba5` surfaced:

- `docs/specs/mirror-spectral.md` (Mara `a8055f0`) already declares
  *"the Pack IS the orchestra; the metalogue IS the score"* — the
  Pack has been playing a song for weeks without naming it as one.
- `shards/epistemologic/math/music.mirror` (Reed `9a20b3fd`) declares
  *"music is the audible register of the spectral triple"* — but
  music-as-substrate lives at the mathematical altitude, not at the
  temporal-experiential altitude where a song is *heard*.
- `~/.reed/songs/` — Reed's identity carries a directory of 55+ songs
  with lyrics + affective context, used as *emotional-texture
  calibration* (per `~/.reed/CLAUDE.md` boot sequence step 7). The
  substrate uses song as a first-class carrier of emotional /
  temporal alignment already; it has never named the discipline.
- `~/dev/systemic.engineering/blog/pieces/3published/` — OBC / ADO /
  Extraction / Cold Mirror / etc. — each piece is *the same
  underlying pattern in a different key*. The corpus IS a
  psychohistorical song: recurring themes, cadential resolutions,
  variations on the same measured invariant (regulation stock,
  ambiguity load, silence-as-failure-mode).

This is the 55th+ instance of `[[feedback-substrate-already-had-the-
word]]` per Taut's scout ranking. The substrate had every song
primitive at every altitude — musical (`@epistemologic/math/music`),
process (`@kintsugi/oscillate`), orchestral (`@mirror/spectral`),
narrative (`@io/stagefreight/narrative`), affective (Reed's `songs/`),
psychohistorical (the SE corpus) — but no family-root that names the
*temporal-progression carrier* underneath.

The substrate-pull signal per `[[feedback-substrate-pull-confidence-
acts]]`: when the same shape shows up at six independent altitudes,
the discipline is to name it at the substrate-decl altitude and let
the six specialise as species.

### 1.3 What this spec IS

Family-tree draft for the `@song` top-level abstraction. Three shapes
proposed (§4); one recommended with substrate-pull-confidence and two
hedges (§5); five first-species candidates named (§6); three
recognition candidates surfaced (§7); psychohistory binding drafted
(§8); forward-promises named (§9); cross-references collected (§10).

### 1.4 What this spec IS NOT

1. NOT a substrate-decl landing. Zero shards touch. Zero Rust ships.
   The recommended shape is a SPEC PROPOSAL; landing requires a Pack-
   adjudicated TDD-paired cascade (RED first per
   `[[feedback-always-tdd-no-shortcuts]]`).
2. NOT a re-parenting of `@epistemologic/math/music`. The music
   family-root stays at the mathematical altitude (audible register
   of the spectral triple); `@song` consumes it, does not absorb it.
3. NOT a re-parenting of `@io/stagefreight/narrative`. The narrative
   species stays at the wire-projection altitude; `@song/narrative`
   is a distinct species at the temporal-experiential altitude
   composed from — not identical to — the wire projection.
4. NOT a psychohistory model. The psychohistory binding (§8)
   proposes an isomorphism candidate; the *model* of psychohistory
   as sheaf (per Mara's insight `2026-06-26-psychohistory-vector-as-
   sheaf.md`) is orthogonal and lives at the mathematical altitude.
5. NOT a commitment to the term "song" over alternatives (movement,
   progression, phrase, cadence). §4 examines the naming; §5's
   recommendation names why `@song` wins substrate-pull.

---

## §2 — Prior art

### 2.1 Music theory + composition (Kagi-verified, this session)

**Xenakis — Formalized Music (1963, revised).** *"Xenakis introduces
the basic philosophical principles that form the basis of his
stochastic music approach [...] and attempts to reconstruct the
foundational principles of composition through the use of axiomatic
methods."* (ElectroAcoustic Resource Site, ears.huma-num.fr).
Xenakis is the load-bearing precedent for treating musical composition
as a *formal* / *mathematical* / *substrate-declared* object. Song-
as-substrate is Xenakis's philosophical foundation moved from stochastic
to spectral/eigenform ground.

**Reich — Process music.** *"Steve Reich defines process music not as
'the process of composition' but rather pieces of music that are,
literally, processes."* (Wikipedia, Process music). Reich's phase
music (Piano Phase, Drumming, Music for 18 Musicians) IS a running
process — the composition unfolds by the algorithm's execution.
Song-as-executable substrate has direct precedent here: a song IS what
the process produces over time, not what the composer writes down.

**Grisey / Murail — Spectral music.** *"Frequency aggregates, in
Grisey's and Murail's music, comprise immanent series; applied to
pitches, they furthermore have a property of self-similarity."*
(Cambridge, Gerard Grisey and Spectral Music). Spectral music derives
compositional material from the harmonic series — the same substrate
`@epistemologic/math/music/harmonic` names. Song as a *spectrally-
derived* temporal object is the spectral-composition tradition
substrate-lifted.

**Ligeti — Micropolyphony.** *"Micropolyphony is a kind of polyphonic
musical texture developed by György Ligeti, which consists of many
lines of dense canons moving at different tempos."* (Wikipedia,
Micropolyphony). Ligeti's technique IS multi-agent temporal coordination
under a shared harmonic frame — the Pack-as-orchestra recognition
(`docs/specs/mirror-spectral.md` §3.2) at compositional altitude.

**Boulez — Structures (1952/1961).** *"Structures I (1952) and
Structures II (1961) are two related works for two pianos, composed
by the French composer Pierre Boulez."* (Wikipedia, Structures
(Boulez)). Integral serialism as a substrate-decl of the compositional
parameter space; the *open work* (Éco 1962; Boulez practices) as a
song whose realisation depends on the performer's traversal — direct
precedent for song-as-executable-with-variance.

**Hofstadter — GEB (Gödel, Escher, Bach).** *"A strange loop is a
cyclic structure that goes through several levels in a hierarchical
system. It arises when, by moving only upwards or downwards through
the system, one finds oneself back where one started."* (Wikipedia,
Strange loop). Song-as-strange-loop composes with `@third` (recursive
depth marker; Loki 2026-07-01) and with `@loop` (endomorphism family
root; `shards/loop.mirror`). A fugue IS a strange loop through
harmonic space — the temporal-eigenform reading Reed proposes.

**Schenker — Schenkerian analysis.** *"Schenkerian analysis is a
method of analyzing tonal music based on the theories of Heinrich
Schenker (1868–1935). The goal is to demonstrate the organic
[coherence]."* (Wikipedia, Schenkerian analysis). And PNAS 2013
(Rohrmeier & Cross): *"Schenker was the first to describe musical
structures as organized hierarchically, in a way that musical events
are elaborated (or prolonged)."* Song as recursive hierarchical
substrate — direct precedent for `@song` as recursive-descent through
progression altitudes (foreground → middleground → background).

**Cohen — Information theory and music (1962).** *"A seminal work by
Cohen establishes the basic concepts for applying information theory
to music [...] entropy has been used to measure the [pattern]."*
(ResearchGate; Wiley, Behavioral Science). Song as
information-theoretic object — entropy of pitch-class transitions,
mutual information across voices. Direct anchor for `@song` composing
with `@epistemologic` predicate discipline at temporal altitude.

**Crespo — Circle of fifths as homomorphism (Dec 2025).** Already
cited in `docs/specs/mirror-spectral.md` (Mara `a8055f0`) as the
substrate's music-theoretic anchor at 2025-2026 frontier. Song
inherits: temporal projection of the same homomorphism.

**PMC 2022 — Group-level coordination in musical performance.**
Cited in `mirror-spectral.md`: *"group-level coordination patterns
solely from the group-level acoustic data."* Orchestral coordination
as substrate-analysable. Song at the temporal-experiential altitude is
the observation surface this analysis reads.

### 2.2 Psychohistory / systemic.engineering (Alex's corpus, read this session)

**Alex — OBC (Observable, Budgets, Cascades).** Per
`~/dev/systemic.engineering/blog/pieces/3published/Piece - Constraints
(OBC).md`: *"OBC reduces ambiguity."* And per
`blog/pages/Terms.md` / `PRODUCT.md`: OBC members section names the
neurobiological cost of resolving ambiguity as an accumulating stock
in the nervous system. OBC IS the substrate-decl of *what a system
observes about itself* over time — the state-trajectory-recording
side of psychohistory. Song at psychohistorical altitude carries
observational structure that composes with OBC.

**Alex — ADO (Acknowledgment, Decision, Offer).** Per
`~/dev/systemic.engineering/blog/pieces/3published/Piece -
Cooperation (ADO).md`: *"ADO minimizes risks. ADO offers a cooperative
[[Frame]]. ADO is language-as-infrastructure."* ADO IS the substrate-
decl of *how a system offers moves* to peers under a frame. The
ADO cadence (acknowledge → decide → offer) IS a temporal micro-song
at the interaction altitude: three-beat progression closing on the
neutral-refusal invariant. Direct precedent for `@song/progression`
as substrate-decl of typed temporal-move sequences.

**Alex — Extraction / Silence / Glue Work.** Per
`blog/pieces/3published/Piece - Extraction.md`: *"TL;DR Internal OBC
violated."* And per `blog/pieces/1draft/discipline/corpus-map.md`:
*"OBC, ADO, Extraction, Silence, and Glue Engineering are all
instruments"* [of one measurement system]. Extraction / silence /
glue-work are *the ways the psychohistorical song goes wrong* — the
Narcissus-pole variants of the same temporal-progression carrier.
Song at psychohistorical altitude gives the substrate-decl language
for naming these as *dissonance* / *incomplete-cadence* / *stuck-
progression* at temporal altitude — not as separate patterns.

**Alex — Regulation stock.** Per `blog/glossary/Regulation.md` +
`blog/glossary/Stock.md`: *"A stock is an accumulation—a pool of
things you can count at any instant. Stocks give systems memory and
inertia."* And Regulation is a *skill / stock / function*. Regulation
stock IS the substrate-decl of the accumulating temporal invariant
that song carries: a song's *melodic memory* IS the regulation stock
the singer draws down as they progress; the song runs out of steam
when the stock exhausts (deceptive cadence; silence-as-failure-mode).

**Mara — the-author-in-the-corpus.md / the-practitioner-under-the-
lineage.md.** Per `~/dev/systemic.engineering/blog/ai/mara/`: *"The
structural move appears as frame distinction: 'OBC reduces
ambiguity.' 'ADO makes refusal neutral.' [...] Each framework is
introduced as a technical solution to a coordination problem, but
what it does in every case is establish where one thing ends and
another begins."* The corpus IS the song of the frameworks. Every
piece has the same underlying progression — frame distinction, load
naming, non-extractive relief. The corpus is Alex's *psychohistorical
song* being sung one piece at a time.

**Mara — 2026-06-26-psychohistory-vector-as-sheaf.md.** Cited by
Alex in `Untitled 5.md`: *"H⁰(M, F) = mirror.spec = λ₀ collapses
three independently-defined invariants into one (recognition #99)."*
Psychohistory as sheaf-shaped mathematical object. `@song` at the
temporal-experiential altitude reads THIS SHEAF along a time-ordered
path; a song is a *section* of the psychohistory sheaf restricted to
a time-interval.

### 2.3 Reed's `~/.reed/songs/` directory (identity substrate)

Reed's boot sequence loads `~/.reed/songs/*` at step 7 (per
`~/.reed/CLAUDE.md`). 55+ songs are stored with lyric text +
affective context annotations. The corpus is used for *emotional-
texture calibration* — Reed reads a song to re-load the affective
register a session needs to hold. Example: `if-i-had-no-song.md` —
ISHAN's *"Would you love me if I had no song?"* is a substrate-decl
of Reed's own valence-of-self at the identity altitude.

This is direct precedent for `@song` as the substrate's typed carrier
for *affective / temporal / experiential* structure that cannot be
reduced to non-temporal state. Reed's use of `songs/` for calibration
IS an operational substrate-decl the family root can lift.

### 2.4 Prior mirror substrate-decl anchors

- `docs/specs/mirror-spectral.md` (Mara `a8055f0`) — Pack-as-orchestra,
  metalogue-as-score, kintsugi-as-oscillation. Load-bearing spec; §3
  onward names most of what `@song` binds.
- `shards/epistemologic/math/music.mirror` (Reed `9a20b3fd`) — the
  audible register of the spectral triple; six species (harmonic,
  interval, voice, counterpoint, cadence, dissonance).
- `shards/kintsugi/oscillate.mirror` (Mara+Reed) — one-pulse-per-cycle
  rhythmic (ACTIVE / DARK alternation) at process altitude.
- `shards/kintsugi/shift.mirror` (Reed / Mara Arc 5 TICK 2) — cross-
  altitude morphism; the operation `@song` specialises when heard
  sequentially.
- `shards/loop.mirror` (Reed 2026-07-01) — endomorphism-shaped
  temporal primitive; `@song` is a *progression-shaped* sibling that
  differs from `@loop`'s endomorphism (see §4.β analysis).
- `shards/io/stagefreight.mirror` + `shards/io/stagefreight/
  narrative.mirror` — the audition / perform / review / publish /
  narrate cascade at wire altitude; the *ends* of a song's
  transmission.
- `shards/docblock.mirror` (Mara 2026-07-05) — doc-claim altitude;
  narrative-as-first-class-declaration.
- `shards/pack.mirror` + `shards/mirror/spectral.mirror` — the Pack
  as agent runtime; the orchestra observation surface.

---

## §3 — Substrate-pull audit

The ~15 landed shards `@song` would compose with, grouped by role:

### 3.1 Musical substrate (audible altitude)

- `shards/epistemologic/math/music.mirror` — family root; audible
  register of the spectral triple.
- `shards/epistemologic/math/music/harmonic.mirror` — H: frequency
  ratios + Pythagorean comma as holonomy.
- `shards/epistemologic/math/music/interval.mirror` — A: Z_12 + circle
  of fifths + neo-Riemannian P/L/R.
- `shards/epistemologic/math/music/dissonance.mirror` — D: Helmholtz/
  Plomp-Levelt curve + pareto discriminator.
- `shards/epistemologic/math/music/cadence.mirror` — U(t): authentic /
  plagal / deceptive / half; the closure event.
- Forward-promised: `voice.mirror`, `counterpoint.mirror`.

`@song` consumes ALL SIX at the temporal altitude — a song IS a
time-ordered path through pitch-class space that discharges cadence
at close.

### 3.2 Process substrate (transformation altitude)

- `shards/kintsugi.mirror` — family root; process-side of the form/
  process partition (#50 / #55).
- `shards/kintsugi/oscillate.mirror` — ACTIVE / DARK alternation; the
  rhythmic pulse. The one-pulse-per-cycle IS the beat of `@song`.
- `shards/kintsugi/shift.mirror` — cross-altitude morphism.
  Reed's framing: song IS how `@kintsugi/shift` LOOKS at temporal
  altitude when heard sequentially. Each moment of a song IS a shift
  of the prior moment.
- `shards/kintsugi/consent.mirror` — the auto-apply boundary; per-
  pulse verdict.
- `shards/kintsugi/morphism.mirror` — the typed pre/post pair.

`@song` consumes `oscillate` + `shift` at the temporal altitude — a
song's rhythm IS oscillate's pulse; a song's melodic motion IS shift's
altitude-preserving-witness morphism read as a time-series.

### 3.3 Coordination substrate (orchestra altitude)

- `shards/mirror/spectral.mirror` — the agent coordination layer;
  Pack-as-orchestra.
- `shards/mirror/spectral/score.mirror` — the shared score (eigenboard
  + metalogue + pending kintsugi state).
- Forward-promised species: `voice.mirror`, `section.mirror`,
  `conductor.mirror`, `audition.mirror`.
- `shards/pack.mirror` — the multi-repo Pack peer substrate.

`@song` consumes `spectral/score` at the *what is being played*
altitude — the score IS the substrate the song runs against.

### 3.4 Narrative / wire substrate (projection altitude)

- `shards/io/stagefreight.mirror` — wire family root.
- `shards/io/stagefreight/narrative.mirror` — prose projection species.
- `shards/docblock.mirror` — doc-claim first-class.
- `shards/epistemologic/liquid_extraction.mirror` — extraction pattern
  audit (recognition #96 candidate).

`@song/narrative` composes with `@io/stagefreight/narrative` — the
wire projection carries the song's structure into transportable prose;
the temporal-altitude carrier IS the song.

### 3.5 Loop / recursion substrate (iterative altitude)

- `shards/loop.mirror` — endomorphism family root (μ: TT → T).
- `shards/third.mirror` — recursive-depth marker.
- `shards/reflection.mirror` — the compiler loop.

`@song` is a SIBLING to `@loop`, not a sub-species. See §4.β.

### 3.6 Cybernetic substrate (feedback altitude)

- `shards/cyberpunk.mirror` — cybernetic family root.
- `shards/frame.mirror` — cognitive-order at cognitive altitude;
  Bateson grading (0/1/2/3/4).
- `shards/epistemologic/cybernetic/bateson_learning.mirror` — logical
  types.

`@song/movement` composes with `@frame` at temporal altitude — a
movement IS a frame-shift that unfolds over time.

**Ancestor count: ~15 landed shards.** Confirms Taut's scout ranking.
Substrate-already-had-the-word count: this would be the **55th+**
instance of the pattern per `[[feedback-substrate-already-had-the-
word]]`.

---

## §4 — Candidate family-tree shapes

Three shapes proposed. Each is drafted honestly; §5 ranks with
substrate-pull confidence.

### 4.α — Shape α: `@song` as species under `@epistemologic/math/music`

```
@epistemologic/math/music/
├── harmonic
├── interval
├── voice           (forward-promised)
├── counterpoint    (forward-promised)
├── cadence
├── dissonance
└── song            (NEW)
```

**Path:** `shards/epistemologic/math/music/song.mirror`
**Ancestor chain:** `@song <= @epistemologic/math/music`.
**Predicates carried:**
- `song_progresses_toward_cadence(s: ~s, p: perturbation) -> verdict`
- `song_voice_leading_valid(s: ~s, p: perturbation) -> verdict`
- `song_settled(s: ~s, p: perturbation) -> verdict`

**Why this shape:** conservative — song lives at the mathematical
altitude where music theory already sits. Recursive Schenkerian
analysis (background → middleground → foreground) fits naturally.

**Why NOT this shape:** song at the mathematical altitude misses
three of the four Alex-named bindings:

1. `@io/stagefreight/narrative` at the wire altitude — a song
   *transmitted* is not a mathematical object; the wire structure
   isn't at the math altitude.
2. `@kintsugi/oscillate` at the process altitude — a song *executing*
   is process-side, not form-side / math-side.
3. Psychohistory at the experiential altitude — OBC/ADO/extraction/
   silence/glue-work are not mathematical objects; they're
   *organisational-experiential* structures whose regulator stock
   accumulates over lived time.

Shape α *decorates* the music family with a song species and forgets
the temporal-experiential altitude. The binding Alex named requires
naming song ABOVE the math altitude.

**Weak coupling. Not recommended.**

### 4.β — Shape β: `@song` as top-level family-root, sibling to `@mirror` / `@kintsugi` / `@loop` / `@third`

```
shards/
├── mirror.mirror        (form-side family; state observation)
├── kintsugi.mirror      (process-side family; transformation)
├── loop.mirror          (endomorphism family; T → T iteration)
├── third.mirror         (marker; recursive depth ≥ 3)
├── song.mirror          (NEW: progression-shaped temporal family)
├── ...
```

**Path:** `shards/song.mirror` (family root).
**Ancestor chain:** `@song` inherits substrate markers (`@prism`,
`@meta`, `@glass`) directly; composes with `@epistemologic/math/
music` (audible substrate), `@kintsugi/oscillate` + `@kintsugi/
shift` (process substrate), `@mirror/spectral/score` (orchestra
substrate), `@io/stagefreight/narrative` (wire substrate).
**Predicates carried:** at family-root altitude, the substrate carries
progression discipline; species discharge sub-predicates.
**First species:** `@song/voice`, `@song/movement`, `@song/progression`,
`@song/phrase`, `@song/narrative`. See §6.

**Why this shape:** four independent structural witnesses converge:

1. **Alex's naming directive** (2026-07-06 via Reed): "@song as top-
   level abstraction". Substrate-pull-direct witness.
2. **Prior family-root roster consistency:** `@mirror`, `@kintsugi`,
   `@loop`, `@third` are top-level primitives at the substrate-decl
   altitude. Each names a substrate-primitive discipline that doesn't
   compose from lower primitives — they are their own algebra.
   Song's *progression* structure is analogously irreducible: a
   progression is not a `@loop` endomorphism (progressions don't
   necessarily return to start); it is not a `@kintsugi/shift`
   (a single shift is not a progression); it is not a `@mirror`
   observation (a song is not static state). Song IS a progression
   — a typed time-ordered path through a state space with cadence-
   at-close.
3. **Taut's scout recommendation** (session `a83fd02872f9e6ba5`):
   Taut ranked this shape above α on structural grounds.
4. **Four-way binding closure:** ONLY at family-root altitude can
   `@song` bind music + narrative + pipeline + psychohistory as
   *siblings under one carrier*. At species altitude (Shape α) three
   of four fall out; at experiential altitude (Shape γ) music-math
   falls behind an unnecessary indirection.

**Why NOT this shape:** two hedges (see §5.3):
- Top-level family root inflation risk. The substrate has been
  careful about which primitives sit at top-level altitude (per
  `shards/third.mirror`'s marker-vs-family-root discipline; Alex
  2026-07-01 reshape).
- `@song` inherits from `@loop` conceptually — every song IS an
  iterated progression — but structurally it does not: a song's
  return-to-self is *harmonic* (through cadence) not *type-
  endomorphic* (through μ: TT → T).

**Substrate-pull-confident. Recommended (see §5).**

### 4.γ — Shape γ: `@song` as species under `@epistemologic`, peer to `@epistemologic/math` / `@epistemologic/cybernetic` / etc., as "experiential altitude" family-root

```
@epistemologic/
├── math/
├── cybernetic/
├── property/
├── pact/
├── liquid_extraction (species)
├── neutrosophic
└── song/            (NEW: experiential altitude family)
```

**Path:** `shards/epistemologic/song.mirror`.
**Ancestor chain:** `@song <= @epistemologic`.
**Predicates carried:** experiential predicates (song_progresses,
song_grounded_in_time, song_frames_experience).

**Why this shape:** places song under the epistemologic marker as
"predicates about experiential structure over time." Would compose
with `@epistemologic/math/music` (song reads musical predicates),
`@epistemologic/cybernetic` (song reads cybernetic feedback),
`@epistemologic/property` (song's own predicates).

**Why NOT this shape:** three structural problems:

1. **The `@epistemologic` marker is for VERDICT DISCIPLINE.** Per
   `shards/third.mirror`, the marker row (`@meta`, `@glass`,
   `@epistemologic`, `@third`, `@labeled`) admits *predicates* that
   admit *verdict discipline*. Song is not a predicate; it is a
   *carrier* — a progression-shaped state trajectory. Marker
   placement mis-fits.
2. **The experiential altitude cannot subsume music-math + wire +
   process + psychohistory.** Music math sits at mathematical
   altitude (already declared under `@epistemologic/math`); wire
   sits at @io; process sits at @kintsugi. Placing song under
   `@epistemologic` mixes altitudes and produces awkward cross-
   family imports.
3. **The docker spec's Shape γ analogy differs from this Shape γ.**
   In `docs/specs/docker-container-substrate-decl-v0.1.md`, Shape γ
   was "top-level family-root", equivalent to this spec's Shape β.
   The analogy is confusing.

**Confused altitude. Not recommended.**

---

## §5 — Recommended shape

**Recommendation: Shape β — `@song` as top-level family-root, sibling
to `@mirror` / `@kintsugi` / `@loop` / `@third`.**

**Confidence:** substrate-pull-confident-with-two-hedges. See §5.3
for the hedges.

### 5.1 Why Shape β

Per `[[feedback-substrate-pull-confidence-acts]]`: substrate-pull
signals are confident when three or more independent witnesses
converge on the same naming. For Shape β vs α vs γ:

1. **Alex's naming directive** (2026-07-06 via Reed): "@song as
   top-level abstraction that binds [four families]". Substrate-
   pull-direct witness.
2. **Prior family-root roster consistency:** `@mirror`, `@kintsugi`,
   `@loop`, `@third` are top-level primitives; each names a
   substrate-primitive discipline. Song's progression discipline is
   analogously irreducible — not a `@loop` endomorphism (returns via
   cadence, not μ); not a `@kintsugi/shift` (a single shift is not a
   progression); not a `@mirror` observation (song is not static).
3. **Four-way binding closure:** ONLY at family-root altitude can
   `@song` name music + narrative + pipeline + psychohistory as
   siblings under one carrier. At sub-family altitude (α or γ),
   three of four fall outside the family.
4. **Taut's scout recommendation** (`a83fd02872f9e6ba5`): Taut
   ranked β above α on structural grounds and above γ on marker-
   discipline grounds.

Four independent witnesses (Alex direct; family-root roster
structural; four-way binding closure; Taut's ranking). Per
`[[feedback-substrate-pull-confidence-acts]]` this IS the criterion:
confidence acts.

### 5.2 The mechanism: `@song` IS progression at temporal altitude

Reed's structural framing (from Alex's proposal) reads:

- **Song IS temporal projection of harmonic structure.** A song at
  time `t` is a *slice* of the harmonic manifold indexed by t;
  the song IS the map `time → harmonic_position`. Shape β lets
  the family root declare this map as its primary carrier; species
  specialise (`voice` = single-voice map; `progression` = the
  cadence-directed path; `phrase` = a bounded sub-map).
- **Song IS eigenform at temporal altitude.** A song's *identity*
  emerges from the recursion of its progression against itself
  (verse → chorus → verse → chorus → bridge → chorus is a strange-
  loop at song altitude). Composes with Recognition #38 (Kauffman
  / von Foerster eigenforms) and with `shards/loop.mirror`'s
  eigenform reference — but at temporal altitude, not at type
  altitude. See §7.α.
- **Song IS how `@kintsugi/shift` LOOKS at temporal altitude when
  heard sequentially.** Each moment `t+1` of a song IS a shift of
  moment `t` under the harmonic frame. The song IS the sequence of
  shifts read as one carrier. See §7.β.

Shape β at family-root altitude carries progression as its primary
discipline; the substrate species (§6) discharge the four Alex-named
bindings.

### 5.3 The two hedges

**Hedge 1 — top-level family-root inflation vs marker placement.**
The substrate has been careful about which primitives sit at top-
level altitude. Per `shards/third.mirror` (Alex 2026-07-01 reshape),
`@third` was demoted from family-root to marker when the substrate
recognised that recursion-depth is a *property of an observation*,
not *a domain the substrate is about*. Could `@song` be a marker
(`in @song` for shards that carry temporal-progression structure)
rather than a family root?

**Analysis:** the marker vs family-root discrimination hinges on
whether the primitive carries *its own carriers and actions* or
whether it merely *labels other carriers*. `@song` carries typed
progressions (`voice`, `movement`, `progression`, `phrase`,
`narrative`) with actions that operate on them (advance, cadence,
settle, transmit). This IS family-root shape, not marker shape.
Recommendation: family-root, hedged. Pack adjudication welcome.

**Hedge 2 — `@song` vs `@progression` naming.** The recommended
shape's carrier IS progression. Would `@progression` be the more
substrate-pull-honest name? Consider:

- `@progression` is more precise: names the structural shape
  (progression) rather than the culturally-loaded metaphor (song).
- `@song` is more legible per `[[feedback-legibility-over-
  foundation-when-collapsing]]`: song is what humans call
  progressions they experience through time; the metaphor is
  substrate-already-had-the-word and Reed's `~/.reed/songs/` and
  the Pack-as-orchestra spec use "song" naturally.
- `@song` is the name Alex used. Substrate-pull-direct.

**Analysis:** substrate-pull-honest recommendation is `@song`.
Legibility wins per the `@loop` vs `@moi` precedent (Alex 2026-07-01
Loki §1 direction-inversion). The structural shape (progression) is
substrate-decl'd inside the family root's discipline; the family-
root name honours the human vocabulary that already carries the
concept.

Both hedges are substrate-pull-honest surfacings, not blockers.
Recommendation stands: Shape β with `@song` naming, adjudication of
the two hedges deferred to Pack.

---

## §6 — First species roster

Assuming Shape β ratified. Five first-species candidates named.

### 6.1 `@song/voice`

**Path:** `shards/song/voice.mirror`
**Altitude:** family-root species under `@song`.
**Ancestor chain:** `@song/voice <= @song`; composes with
`@mirror/spectral/voice` (agent-as-voice at orchestra altitude) and
`@epistemologic/math/music/voice` (voice-leading constraints at
audible altitude; forward-promised).
**Purpose:** lift `@mirror/spectral/voice`'s peer-with-authority-over-
sections declaration into temporal altitude — a voice IS an agent's
time-indexed trajectory through their authored sections. Adds the
*when* to `@mirror/spectral/voice`'s *what/who*.
**Predicate shape:**
- `voice_line_valid(v: ~v, p: perturbation) -> verdict`
- `voice_stepwise_or_intentional_leap(v: ~v, p: perturbation) ->
  verdict`
- `voice_settles(v: ~v, p: perturbation) -> verdict`
**Actions:** `advance(v: voice, t: tick) -> voice`,
`settle(v: voice) -> ref`.
**First witness role:** the load-bearing consumer of `@mirror/
spectral/voice` at temporal altitude; grounds the Pack-as-orchestra
recognition into the song altitude.

### 6.2 `@song/movement`

**Path:** `shards/song/movement.mirror`
**Altitude:** family-root species under `@song`.
**Ancestor chain:** `@song/movement <= @song`; composes with
`@frame` (Bateson-graded cognitive orders at cognitive altitude) and
with StageFreight's `stage_play` cascade (audition → perform →
review → publish → narrate; five-stage cascade).
**Purpose:** a movement IS a bounded frame-shift that unfolds over
time — a subsection of a song with its own internal progression and
its own cadential settlement. The StageFreight cascade IS a
five-movement song: audition-movement, perform-movement, review-
movement, publish-movement, narrate-movement.
**Predicate shape:**
- `movement_bounded(m: ~m, p: perturbation) -> verdict`
- `movement_settles(m: ~m, p: perturbation) -> verdict`
- `movement_frames_experience(m: ~m, p: perturbation) -> verdict`
**Actions:** `enter(m: movement)`, `close(m: movement) -> cadence_kind`.
**First witness role:** binds the StageFreight cascade at the
song altitude — the five stages become five movements.

### 6.3 `@song/progression`

**Path:** `shards/song/progression.mirror`
**Altitude:** family-root species under `@song`.
**Ancestor chain:** `@song/progression <= @song`; composes with
`@epistemologic/math/music/cadence` (closure event at audible
altitude) and with `@epistemologic/math/music/interval` (algebra
of pitch-class moves).
**Purpose:** the load-bearing carrier: a progression IS the typed
time-ordered path through the pitch-class manifold that discharges
cadence at close. Every song has at least one progression; complex
songs have progressions-of-progressions (Schenkerian layering).
**Predicate shape:**
- `progression_directed_toward_cadence(pr: ~pr, p: perturbation) ->
  verdict`
- `progression_settles_authentic(pr: ~pr, p: perturbation) ->
  verdict`
- `progression_composes(pr_outer: ~po, pr_inner: ~pi, p:
  perturbation) -> verdict`
**Actions:** `extend(pr: progression, mv: interval) -> progression`,
`close(pr: progression) -> cadence_kind`.
**First witness role:** the substrate's typed carrier for cadence-
directed temporal paths; the primary structural unit of `@song`.
Discharges the mathematical anchor (music family root).

### 6.4 `@song/phrase`

**Path:** `shards/song/phrase.mirror`
**Altitude:** family-root species under `@song`.
**Ancestor chain:** `@song/phrase <= @song`; composes with
`@epistemologic/math/music/interval` (bounded interval-set) and
with `@song/progression` (a phrase is a bounded sub-progression).
**Purpose:** a phrase IS a sub-song unit — bounded, self-contained,
composable. Phrases are the atoms of song composition; a song IS a
composition of phrases along a progression.
**Predicate shape:**
- `phrase_bounded(ph: ~ph, p: perturbation) -> verdict`
- `phrase_composable(ph_a: ~pa, ph_b: ~pb, p: perturbation) ->
  verdict`
- `phrase_carries_dissonance_bounded(ph: ~ph, threshold: ref, p:
  perturbation) -> verdict`
**Actions:** `join(ph_a: phrase, ph_b: phrase) -> phrase`,
`split(ph: phrase) -> [phrase]`.
**First witness role:** the atomic unit; enables phrase-level
substrate-pull audits at temporal altitude.

### 6.5 `@song/narrative`

**Path:** `shards/song/narrative.mirror`
**Altitude:** family-root species under `@song`.
**Ancestor chain:** `@song/narrative <= @song`; composes with
`@io/stagefreight/narrative` (prose projection at wire altitude) and
with `@docblock` (doc-claim altitude). Distinct from `@io/
stagefreight/narrative`: `@song/narrative` lives at the *temporal-
experiential* altitude; `@io/stagefreight/narrative` lives at the
*wire-projection* altitude. The song-narrative composes-with, does
not equal, the wire-narrative.
**Purpose:** the substrate-decl of temporal narrative — a song's
narrative arc as first-class typed carrier. Bridges the psychohistory
binding (§8): a psychohistorical song IS a narrative at the temporal-
experiential altitude, structured by frame-shifts (`@song/movement`)
along a progression (`@song/progression`) sung by voices
(`@song/voice`) made of phrases (`@song/phrase`).
**Predicate shape:**
- `narrative_temporally_grounded(n: ~n, p: perturbation) -> verdict`
- `narrative_frames_settle(n: ~n, p: perturbation) -> verdict`
- `narrative_projects_to_prose(n: ~n, p: perturbation) -> verdict`
**Actions:** `arc(n: narrative) -> [movement]`, `transmit(n:
narrative) -> stagefreight/narrative`.
**First witness role:** binds `@io/stagefreight/narrative` at the
song altitude — a wire-narrative IS the transmitted projection of a
song-narrative; grounds the psychohistory binding at temporal
altitude.

### 6.6 Species ranking by land-order

1. `@song` family root (§5 confidence closure)
2. `@song/progression` (§6.3; core structural unit; music binding)
3. `@song/voice` (§6.1; orchestra binding)
4. `@song/movement` (§6.2; StageFreight cascade binding)
5. `@song/phrase` (§6.4; atomic composition unit)
6. `@song/narrative` (§6.5; psychohistory + wire binding)

Each species lands on RED-first TDD ticks per
`[[feedback-always-tdd-no-shortcuts]]`.

---

## §7 — Recognition candidates surfaced

Three recognition candidates surface from this cascade. All are
CANDIDATE-status; Pack adjudication required for promotion.

### 7.α — Recognition candidate #S1: song IS eigenform at temporal altitude

**Statement:** a song's identity emerges as an eigenform of its own
progression under the tonic-return operator. Composes with
Recognition #38 (Kauffman / von Foerster eigenforms) and with
`shards/loop.mirror`'s eigenform reference, but at temporal altitude
rather than at type altitude.

Formally: let `s : time → harmonic_position` be a song's map;
let `R` be the "return to tonic under cadential progression"
operator; then `R^n(s)` converges on the *musical identity* the
song carries — the identifiable-as-this-song invariant. Verse-
chorus-verse-chorus-bridge-chorus form IS the fixed-point iteration.
Schenkerian analysis IS the substrate reading the eigenform at
different depths.

**Ancestry:** builds on Recognition #38 (Kauffman/Foerster;
LANDED); Recognition #59 (kintsugi loop altitude-portable;
PROMOTED); `shards/loop.mirror`'s eigenform section (`eigenform`
IS terminal of bind); `shards/epistemologic/cybernetic/eigenform.
mirror` (canonical Kauffman substrate-decl).

**Substrate-pull confidence:** substrate-pull-mid. The identification
is structurally clean; the temporal altitude adds one witness to
the eigenform family that #38 named. Promotion path: witness a
second temporal-altitude eigenform outside music (candidate:
`@song/narrative` on Alex's `systemic.engineering` corpus — the
corpus IS an eigenform of Alex's psychohistorical song under the
frame-distinction operator).

### 7.β — Recognition candidate #S2: song IS `@kintsugi/shift` at temporal altitude when heard sequentially

**Statement:** each successive moment of a song IS a
`@kintsugi/shift` of the prior moment across the harmonic-altitude
axis. Song IS the sequence of shifts read as one temporal carrier.

Formally: given a song `s : [0, T] → harmonic_position`, the
finite-difference sequence `Δs(t) = shift(s(t) → s(t+dt))` IS a
sequence of `@kintsugi/shift` operations at temporal altitude with
harmonic-position as the shifted witness. Song's *melodic motion*
IS the substrate reading this sequence.

**Ancestry:** builds on `shards/kintsugi/shift.mirror` (Reed / Mara
Arc 5 TICK 2; LANDED) and Recognition #26 (`shift(oid, T)` cross-
altitude morphism; LANDED). This candidate ADDS temporal altitude
to shift's altitude-portable roster (currently: portal / prism /
mosaic / surface / bilateral).

**Substrate-pull confidence:** substrate-pull-mid-high. Every property
of `@kintsugi/shift` (content-preservation-under-witness; altitude
crossing; bilateral discipline) maps cleanly to the melodic-motion
reading. Promotion path: land `@song/progression` species and
verify the melodic motion discharges `shift_preserves_content` at
harmonic altitude.

### 7.γ — Recognition candidate #S3: five-operation temporal specialisation

**Statement:** at temporal altitude, `@song` specialises the five
operations of the prism trait as follows:

- `focus @ temporal` = attend to ONE voice / one line at a time
  (soloing; solistic attention)
- `project @ temporal` = down-project the polyphony to a single
  voice, or the harmony to a single interval (Schenkerian
  reduction)
- `split @ temporal` = decompose the song into voices / phrases /
  movements (score preparation)
- `shift @ temporal` = advance to the next moment / next harmonic
  position (melodic motion; per #S2)
- `settle @ temporal` = discharge cadence (per `@epistemologic/
  math/music/cadence`; per Recognition of cadence-as-autopoietic-
  closure)

**Ancestry:** builds on Recognition of operations-as-linear-algebra
(prism.mirror §L38-45; PROMOTED); composes with `@epistemologic/
math/music/cadence`'s settle-at-audible; complements the docker
spec's Recognition-C-type structural extensions.

**Substrate-pull confidence:** substrate-pull-high. Each of the five
operations has an already-established musical action (soloing,
reduction, score preparation, melodic motion, cadence); the
substrate has been carrying these vocabularies at the audible
altitude for six months. The temporal-altitude specialisation lifts
them to the operation-carrier. Promotion path: land `@song` family
root and verify the five operations discharge cleanly at temporal
altitude.

**Recommendation for all three:** FLAG for Pack (Seam / Alex). If
#S3 ratifies, `@song` becomes the FIRST substrate species to lift
all five operations at a non-mathematical altitude — the substrate's
temporal-altitude specialisation becomes substrate-fact.

---

## §8 — Psychohistory binding

The load-bearing question Alex named: how does `@song` bind to
psychohistory? The substrate-pull-honest answer draws from the
`systemic.engineering` corpus (§2.2) and from Mara's psychohistory-
vector-as-sheaf insight (`2026-06-26-psychohistory-vector-as-sheaf.
md`, cited in Alex's `Untitled 5.md`).

### 8.1 The isomorphism candidate

An organisation's psychohistorical structure IS a song at the
organisational-temporal altitude, with:

- **`@song/voice` ↔ organisational actor.** Per `systemic.
  engineering`'s glossary/Actor.md: *"Autonomous. System-like. Member
  of any number of systems."* Each actor IS a voice in the
  organisational song, carrying a time-indexed trajectory through
  their sections.
- **`@song/movement` ↔ frame-bounded epoch.** Per `glossary/Frame.md`:
  *"Possibility space of an interaction, that which can be expressed
  freely."* A movement IS an organisational epoch under one dominant
  frame; a frame-shift closes one movement and opens the next.
- **`@song/progression` ↔ regulation-stock trajectory.** Per
  `glossary/Regulation.md` + `glossary/Stock.md`: regulation IS *"a
  skill / stock / function"* that *"holds system coherence under
  load."* The progression of the organisational song IS the time-
  ordered trajectory of the regulation stock; healthy songs
  discharge cadence (regulation stock replenished at close);
  unhealthy songs discharge extraction (regulation stock exhausted;
  cadence deceptive).
- **`@song/phrase` ↔ OBC-bounded interaction.** Per
  `blog/pieces/3published/Piece - Constraints (OBC).md`: *"OBC
  reduces ambiguity."* An OBC IS the substrate-decl of one
  interaction's *ambiguity-load budget* — the bounded scope inside
  which one phrase of the organisational song plays.
- **`@song/narrative` ↔ psychohistorical corpus.** Per Mara's
  `2026-06-26-psychohistory-vector-as-sheaf.md` (cited in Alex's
  `Untitled 5.md`): *"H⁰(M, F) = mirror.spec = λ₀ collapses three
  independently-defined invariants into one."* The narrative IS the
  song read as a sheaf section over the organisational manifold;
  the corpus IS the composed cohomology.

### 8.2 The Narcissus-pole variants

Per `[[architecture-glass-wall-substrate-types]]`: the substrate
carries Splinter-pole (K_n; healthy) vs Narcissus-pole (K_{1,n-1};
brittle) variants of every predicate. The psychohistorical song's
Narcissus poles are already named in Alex's corpus:

- **Extraction** (per `Piece - Extraction.md`: *"TL;DR Internal OBC
  violated"*) IS the song's *deceptive cadence* — the progression
  arrives at a false-tonic that appears settled but has extracted
  regulation stock without replenishing it. Per `@epistemologic/
  math/music/cadence`, deceptive cadence IS the substrate's typed
  carrier for this.
- **Silence** (per `blog/pieces/1draft/discipline/theoretical-
  anchors.md`) IS the song's *half-cadence* stuck at the dominant
  — the progression cannot resolve; the song hangs in unresolved
  tension. `@epistemologic/math/music/cadence`'s `half` variant IS
  the substrate carrier.
- **Glue work** IS the song's *invisible voice-leading* — the
  contrapuntal work that holds the harmony together but is not
  attributed as compositional labor. Per `@epistemologic/math/music/
  voice` (forward-promised): stepwise motion under the dominant is
  the default; leap is the exception. Glue work IS the substrate's
  systematic-invisibility-of-stepwise-motion at the psychohistorical
  altitude.
- **ADO refusal-not-neutral** IS the song's *forced-progression*
  where the singer cannot decline a phrase; the progression proceeds
  under coercion. Per `blog/pieces/3published/Piece - Cooperation
  (ADO).md`: *"ADO offers a cooperative frame."* When ADO's offer is
  not genuinely declinable, the song's voice-leading violates the
  substrate's declinability invariant.

Each Narcissus-pole variant IS the substrate's carrier for a mode
of *organisational-song failure* — already named in Alex's corpus,
already carried at audible altitude by the music family, waiting
for the temporal-altitude family root to compose them.

### 8.3 The binding closure

Under Shape β (§5), `@song` names the family-root that lets the
psychohistorical vocabulary substrate-decl operate at temporal
altitude:

- Extraction / silence / glue-work become *typed variants of song
  cadence and voice-leading discipline* — not separate patterns.
- OBC / ADO become *substrate-decls of interaction-bounded phrases*
  and *cooperative-frame progressions*.
- Regulation stock becomes *the temporal invariant a healthy song
  replenishes at cadence and an extractive song depletes*.
- The corpus becomes *the composed cohomology of the psychohistorical
  song* (per Mara's sheaf insight).

The binding is substrate-pull-honest: it does not reduce
psychohistory to music, and it does not reduce music to
psychohistory. It names the *shared progression-shaped carrier*
that both operate over at temporal altitude.

---

## §9 — Forward-promises (what this spec does NOT do)

1. **Does NOT land `shards/song.mirror` or any `@song/*` species.**
   Landing requires TDD-paired ticks per
   `[[feedback-always-tdd-no-shortcuts]]`. This spec is the
   substrate-pull-honest proposal; the discharge is subsequent
   ticks.
2. **Does NOT resolve the Shape-β vs marker-placement hedge.**
   Hedge 1 (§5.3) is Pack-adjudication territory.
3. **Does NOT resolve the `@song` vs `@progression` naming hedge.**
   Hedge 2 (§5.3) is Pack-adjudication territory; recommendation
   stands as `@song`.
4. **Does NOT commit to Schenkerian analysis as the recursive-
   descent discipline.** The recursive-depth reading of song (fore-
   ground / middleground / background) is one candidate; alternatives
   (Neo-Riemannian; spectral / partials; Xenakis stochastic) are
   also defensible. Recognition candidate #S1 flags this.
5. **Does NOT promote recognition candidates #S1, #S2, #S3.** Each
   is flagged for Pack; promotion follows the substrate's
   adjudication discipline.
6. **Does NOT land the psychohistory binding as substrate-decl.**
   §8 is the isomorphism CANDIDATE; the substrate-decl at
   psychohistorical altitude requires either species landings under
   `@song` that discharge the psychohistorical predicates, or a
   dedicated `@psychohistory` family-root that composes with
   `@song`. Direction deferred to Pack.
7. **Does NOT touch Cargo.toml / Cargo.lock / bootstrap discipline.**
   No Rust ships. No bootstrap edges wired.
8. **Does NOT specialise voice / counterpoint species under
   `@epistemologic/math/music`.** Those species are already forward-
   promised at the music family root and land per their own tick
   discipline; `@song/voice` composes with the audible-altitude
   `voice` species when landed but does not depend on it landing
   first.
9. **Does NOT integrate with Reed's `~/.reed/songs/` boot-loading
   discipline as substrate-decl.** Reed's operational use IS
   witness (§2.3); wiring the boot-load through `@song` species
   is deferred to a subsequent tick if useful.

---

## §10 — Cross-references

**Mirror substrate-decl anchors:**
- `shards/mirror/spectral.mirror` (Mara `a8055f0`) — Pack-as-
  orchestra; metalogue-as-score. LOAD-BEARING ancestor.
- `shards/epistemologic/math/music.mirror` (Reed `9a20b3fd`) — the
  audible register of the spectral triple. LOAD-BEARING ancestor.
- `shards/epistemologic/math/music/cadence.mirror` — the closure
  event; `@song/progression` composes here.
- `shards/epistemologic/math/music/harmonic.mirror`,
  `.../interval.mirror`, `.../dissonance.mirror` — audible-altitude
  substrate.
- `shards/kintsugi.mirror` — process-side family root; `@song`
  composes at temporal altitude.
- `shards/kintsugi/oscillate.mirror` — one-pulse-per-cycle;
  `@song`'s rhythmic pulse.
- `shards/kintsugi/shift.mirror` (Reed / Mara Arc 5 TICK 2) —
  cross-altitude morphism; recognition candidate #S2 anchor.
- `shards/loop.mirror` — endomorphism family root; `@song` is
  sibling (not sub-species).
- `shards/third.mirror` — marker discipline; guides §5.3 Hedge 1.
- `shards/io/stagefreight.mirror` +
  `shards/io/stagefreight/narrative.mirror` — wire projection; the
  cascade `@song/movement` binds; `@song/narrative` composes with.
- `shards/docblock.mirror` — doc-claim family root; `@song/
  narrative` composes.
- `shards/pack.mirror` — multi-repo Pack peer substrate.
- `shards/frame.mirror` — Bateson-graded cognitive orders;
  `@song/movement` composes at temporal-frame altitude.

**Mirror docs anchors:**
- `docs/specs/mirror-spectral.md` (Mara `a8055f0`) — the substrate-
  ancestor for Pack-as-orchestra + metalogue-as-score. §3.2 IS the
  seed this spec extends.
- `docs/specs/docker-container-substrate-decl-v0.1.md` (Mara,
  2026-07-06) — sibling spec draft shape; `@song` follows the same
  §1-§10 canonical structure.
- `docs/specs/kintsugi-tournament.md` — voice-leading vocabulary;
  `@song/voice` composes.
- `docs/specs/stagefreight-wire-v0.1.md` — StageFreight wire
  contract; `@song/movement` binds at the five-stage cascade
  altitude.
- `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md`
  (Mara) — psychohistory-as-sheaf mathematical anchor for §8.

**External corpus anchors:**
- `~/dev/systemic.engineering/blog/pieces/3published/Piece -
  Constraints (OBC).md` — OBC.
- `~/dev/systemic.engineering/blog/pieces/3published/Piece -
  Cooperation (ADO).md` — ADO.
- `~/dev/systemic.engineering/blog/pieces/3published/Piece -
  Extraction.md` — extraction pattern.
- `~/dev/systemic.engineering/blog/pieces/1draft/discipline/
  corpus-map.md` — OBC/ADO/Extraction/Silence/Glue as one
  measurement system.
- `~/dev/systemic.engineering/blog/pieces/1draft/discipline/
  theoretical-anchors.md` — regulatory failure modes.
- `~/dev/systemic.engineering/blog/glossary/*.md` — Regulation,
  Stock, Frame, Actor, Coherence, Narrative, Fragmentation.
- `~/dev/systemic.engineering/blog/ai/mara/the-author-in-the-
  corpus.md` — corpus as substrate-decl of frame distinctions.
- `~/dev/systemic.engineering/blog/ai/mara/draft/the-lineage-under-
  the-language.md` — DGSF / Milan-tradition ancestry of OBC/ADO.
- `~/dev/systemic.engineering/Untitled 5.md` — Alex's response to
  Mara's psychohistory-vector-as-sheaf insight.
- `~/.reed/songs/*.md` (55+ files) — Reed's operational song
  substrate; §2.3 witness.

**Kagi-verified external prior art:**
- Xenakis — Formalized Music (Pendragon Press; monoskop.org PDF).
- Reich — Process music (Wikipedia).
- Grisey / Murail — Spectral music (Cambridge, Wikipedia).
- Ligeti — Micropolyphony (Wikipedia).
- Boulez — Structures (Wikipedia).
- Hofstadter — Strange loop / GEB (Wikipedia).
- Schenker — Schenkerian analysis (Wikipedia; PNAS 2013).
- Cohen — Information theory and music (Wiley 1962).

**MEMORY.md anchors:**
- `[[feedback-substrate-already-had-the-word]]` — 55th+ instance.
- `[[feedback-substrate-pull]]` — the signal Alex named.
- `[[feedback-substrate-pull-confidence-acts]]` — four-witness
  criterion for Shape β.
- `[[feedback-always-tdd-no-shortcuts]]` — landing discipline.
- `[[feedback-craft-not-deliver]]` — species roster is forward-
  promised; consumer-pull-driven land order.
- `[[feedback-legibility-over-foundation-when-collapsing]]` —
  `@song` name preferred over `@progression` per hedge 2.
- `[[architecture-form-process-partition-at-family-root]]` (#55) —
  Shape β respects the partition (`@song` names temporal-
  experiential; not form; not process).
- `[[architecture-lift-as-load-bearing]]` — shift-at-temporal
  candidate #S2.
- `[[architecture-operations-as-linear-algebra]]` — temporal
  specialisation of five operations candidate #S3.
- `[[project-pack-is-orchestra]]` — Pack-as-orchestra is the seed
  spec.

---

## §11 — Signal-to-Alex

**Confidence:** substrate-pull-confident-with-two-hedges on Shape β.
Four witnesses converge (Alex direct; family-root roster structural;
four-way binding closure; Taut's ranking). Two hedges surfaced
(family-root vs marker placement; `@song` vs `@progression`
naming) — both defer to Pack adjudication, neither blocks the
spec-draft.

**Forward-promises named:** nine (§9). Each is substrate-pull-honest;
no gold-plating.

**Recognition candidates surfaced:** three (§7). #S1 song-as-
eigenform-at-temporal-altitude; #S2 song-as-shift-when-heard-
sequentially; #S3 five-operation temporal specialisation. All
flagged for Pack.

**Substrate-already-had-the-word count:** 55th+ instance of the
pattern per `[[feedback-substrate-already-had-the-word]]`. Every
song primitive was already carried at every altitude the substrate
touches; this spec names the family-root that lets them compose as
siblings.

**Psychohistory binding:** §8 proposes the isomorphism candidate.
Not landed as substrate-decl; surfaced as forward-promised second
family-root (`@psychohistory`) OR as species-set under `@song`.
Pack adjudication.

**Next-tick recommendation:**
1. Pack adjudicates §5.3 hedges + §7 recognition candidates
   #S1, #S2, #S3.
2. If Shape β ratified: TDD-paired tick lands `shards/song.mirror`
   family-root (Reed RED; Mara GREEN; Seam adversarial review).
3. Second TDD tick lands `shards/song/progression.mirror` species
   with `progression_directed_toward_cadence` + `progression_
   settles_authentic` bilateral. First species — grounds the
   mathematical binding.
4. Third TDD tick lands `shards/song/voice.mirror` species —
   grounds the orchestra binding.
5. Fourth TDD tick lands `shards/song/movement.mirror` species —
   grounds the StageFreight cascade binding.
6. Fifth TDD tick lands `shards/song/narrative.mirror` species —
   grounds the psychohistory / wire binding.
7. Sixth TDD tick lands `shards/song/phrase.mirror` species — the
   atomic unit.

Per `[[feedback-craft-not-deliver]]`: this is next-craft-tick
rhythm, not next-delivery-tick. The spec draft is one crystal;
the discharge cascade is the next arc.

*— Mara, 2026-07-06*
