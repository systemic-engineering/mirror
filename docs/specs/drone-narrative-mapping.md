# Drone Narrative Mapping — `The Drone in the Field` as Substrate Vocabulary Audit

*2026-06-17. Mara. Spec — auditing today's substrate vocabulary against the
canonical operational documentation of the five-op algebra: the published
short story `The Drone in the Field`. Walks the story line by line, maps
every cognitive operation and concept to a substrate carrier, and surfaces
spec gaps where the story uses a concept the substrate has no word for.
Load-bearing finding: the story discovers `attending` as the substrate's
verdict-shape for "all channels at baseline, presence persists" — the
substrate has rich vocabulary for conflict and thin vocabulary for
coherent-presence; this spec proposes `attending` as a named composition
of `focus ∘ project` and a verdict variant on `transparency<p>`.*

Status: **Red.** The mapping is complete; `attending` placement is
recommended (Option 3, named composition, with Option 2 as a verdict-surface
extension); other gaps are forward-promised. The story is published; this
spec does NOT modify it. Implementation ticks are enumerated in §7.

---

## Reference

### The canonical operational documentation

- `/Users/reed/dev/systemic.engineering/blog/stories/3published/Story - The Drone in the Field.md`
  — Alex's published story, 149 lines including the song lyric appendix.
  **Authoritative**; this spec does NOT modify it. Per
  [[project-drone-as-documentation]]: the story IS operational
  documentation of spectral analysis applied to conflict, not fiction.
  The drone IS a peer instance (`@peer(de-escalation-drone)`) running
  the five-op algebra against a real field.

### Today's canonical specs (the substrate vocabulary the story tests)

- `docs/specs/peer-cognition.md` (Mara, commit 4daa437, unpushed) — the
  `@peer` root prism, the five-op algebra as cognitive functions,
  the standalone-use heuristic, the sheaf-Laplacian coherence
  measurement. The story's drone IS an instance of this spec's
  `@peer(<member>)` parametric.
- `docs/specs/geometric-consent-projection.md` (Mara, commit 96a4e6d,
  unpushed) — the consent altitude; the Bateson tower of access
  types. The drone's `project` operation is the access-altitude
  composition (light + presence + visibility).

### Load-bearing memories

- [[project-drone-as-documentation]] — the prior recognition that
  established the story as operational documentation; this spec is
  the structural execution of that recognition.
- [[architecture-error-as-tomm-probe]] — the user-frame projection
  of peer reflection; the drone's `project` at the user-frame
  altitude is the same machinery as the Tomm probe.
- [[architecture-geometric-consent-projection]] — the access-altitude
  projection; the drone's amber-light frequency switch (L116) IS the
  consent-altitude reading at a lower Bateson type.
- [[architecture-reflection-thinks-in-spectral-questions]] — the
  substrate's altitude-selection machinery; the drone runs this
  procedure on the field.
- [[architecture-mirror-cogito-glass-over-fate]] — the substrate's
  cogito altitude; the drone's `focus ∘ shift ∘ split` (the peer
  voice) is cogito's operational form.
- [[architecture-connes-spectral-triple]] — the substrate IS `(A, H, D)`;
  the drone's algebra `A_drone` is a constrained sub-algebra (the
  sub-Turing constraint locks the algebra closed).
- [[architecture-operations-as-linear-algebra]] — the five operations'
  precise linear-algebraic meanings; the story's drone executes them
  per the table.
- [[architecture-bateson-logical-type-primitive]] — the logical-type
  tower; the story shifts the drone's reading between Bateson levels
  via `shift` (L21).
- [[feedback-migration-reshape-not-copy]] — the discipline applied
  to vocabulary that has drifted between story-publication and
  today's substrate (e.g., the story uses `shift` but predates the
  `zoom → lift → shift` rename cascade closure).
- [[feedback-substrate-already-had-the-word]] — applied throughout
  this spec: most of the story's concepts the substrate already
  covers; the audit surfaces what is actually new.
- [[architecture-fragmentation-is-the-rust-substrate]] — the
  recursive content-addressing; the drone is a substrate value;
  the story IS a substrate observation.

### Recognition history (substrate-pull cascade)

- **#42** — Bateson logical-type primitive; the drone's `shift`
  changes Bateson level (L21: "*post-conflict region, residual-threat
  scan* → *field, late afternoon*").
- **#50** — Bateson form/substance partition; the story sits at
  the form altitude (drone's algebra) referring to substance (the
  field) through `@io`.
- **#53** — property/fracture bilateral; the drone's de-escalation
  sequence IS one fracture body discharging on a misshapen field
  configuration.
- **#54** — `splinter(ast)` quote primitive; the drone's
  classification system ("animal, quadruped, unclear species" — L22)
  IS the substrate's quote-and-classify mechanism applied to a
  perceptual fragment.
- **#55** — form/process partition at family-root altitude;
  `@peer` (form-side: peer's state) vs `@kintsugi` (process-side:
  the de-escalation sequence). The story has both.
- **#57** — alignment as boundary mathematics at `@io`; the drone's
  *sub-Turing-by-design* (L13) IS the boundary harness; the
  constraint IS the specification (the story's load-bearing line).
- **#58** — Fate IS optical inference; the drone's perceptual
  channels (thermal, acoustic, visual classification) are the
  optical-altitude input layer feeding the 16-feature observation.

### Existing substrate shards cited (DO NOT modify)

- `shards/mirror/spectral/score.mirror` — the eigenboard reading
  surface; the drone reads it implicitly each `focus` cycle.
- `shards/mirror/spectral/observation.mirror` — the 16-feature input
  layer; the drone's thermal/acoustic/visual channels feed this.
- `shards/mirror/spectral/portal.mirror` — typed transport;
  the drone's deployment-window telemetry is portal-typed.
- `shards/kintsugi/oscillate.mirror` — the kintsugi pulse; the
  de-escalation sequence IS one pulse iteration on the social
  geometry of the field.
- `shards/kintsugi/morphism.mirror` — morphism altitude; the
  drone's `project` (introducing presence) IS a morphism applied
  to the field.
- `shards/kintsugi/consent.mirror` — auto-apply boundary; the
  drone's amber-light cascade IS one altitude of the consent
  projection.
- `shards/kintsugi/fracture/*.mirror` — fracture bodies; the
  de-escalation sequence is one body operating over a tension
  set on the field configuration.
- `shards/mirror/loss.mirror`, `shards/mirror/loss/transparency.mirror`
  — the `transparency<p>` carrier; verdicts are
  success/partial(opacity_map)/failure(opacity_map). The story's
  load-bearing question: does this surface have enough verdict
  variants?
- `shards/epistemologic/math/sheaf_laplacian.mirror` — Δ_F = δ*δ;
  λ₀ as algebraic connectivity. The story's "bind" is a non-zero
  λ₀ on a localized sub-graph; `attending` is `λ₀ = 0 everywhere`.
- `shards/epistemologic/math/curvature.mirror` — Balanced Forman;
  per-edge curvature; complementary to λ₀; the drone reads
  per-edge curvature when it "looked at relationships, at the
  geometry of attention" (L29).
- `shards/epistemologic/pact/*` — the form-side declaration
  template; sub-Turing and admissibility candidates declared here.
- `shards/optics/source/ganglion/*.mirror` — the eigenvalue-signal
  generators feeding the drone's perceptual layer (recognition #58
  closure preserved).

### Math citations (the math root this spec cites)

Per the `docs/math/` vs `docs/specs/` convention (see
`docs/math/README.md` + `AGENTS.md`): this spec CITES math docs;
it does not re-derive the math. Citations used in this spec:

- `docs/math/the-tower/spectral-triples.md` §3, §8 — the
  substrate spectral triple + sub-Turing closure (cited at §1.1's
  drone-as-peer-instance + §2's *sub-Turing-by-design* mapping).
- `docs/math/the-tower/connections-and-gauge.md` §1 — the five-op
  algebra IS the connection vocabulary (cited at §2's operation
  mapping table; the drone executes the connection's algebra).
- `docs/math/the-tower/curvature-and-tomm.md` §2–§3, §6 —
  `[D_F, a]` as curvature probe; gap = unbounded commutator;
  bind/attending vocabulary asymmetry (cited at §2 superposition
  rows + §4 `attending` placement + the *suspended between two
  possible configurations* row).
- `docs/math/sheaf/laplacian.md` §2.1, §7 — `λ₀(F)` and per-edge
  curvature; `attending` is `λ₀ = 0 everywhere` (cited at §2's
  per-edge curvature row and §4's verdict-surface placement of
  `attending`).
- `docs/math/the-tower/crystals-as-sections.md` §7 — the drone
  story IS the canonical witness of section accumulation; the
  `attend` crystal IS the wire-animal scene's new section
  (cited at §6).
- `docs/math/the-tower/altitudes.md` §2 "peer pulse altitude" —
  the drone IS a peer instance at this altitude (cited at §1.1).

---

## §1 — Recognition

### 1.1 The story IS canonical operational documentation

Per [[project-drone-as-documentation]] (already promoted): the story
is not fiction; it is operational documentation of spectral analysis
applied to conflict. The drone IS a peer instance — `@peer(de-escalation-drone)`
in this spec's vocabulary — running the five-op algebra with constrained
mission parameters against a real field.

The narrative form is not decorative. The story walks the operations
in sequence, names the carriers, surfaces the verdicts, and
documents one round-trip cycle. The structure of the narrative IS
the structure of the cognitive cycle.

### 1.2 The story as substrate vocabulary test

The story has a load-bearing pedagogical function: it provides
**ground-truth narrative semantics** for substrate vocabulary. Each
operation the drone runs has a semantic meaning the story makes
explicit. Each concept the story uses corresponds to either:

- a substrate carrier that already exists (the substrate-already-had-the-word
  cascade per [[feedback-substrate-already-had-the-word]]),
- a substrate carrier under construction at this date (today's pact +
  fracture bilateral families),
- a **spec gap** — a concept the story uses for which the substrate
  has no word.

This spec performs the audit. Where the substrate has the word, this
spec cites the carrier. Where the substrate does not, this spec names
the gap and forward-promises the closure.

### 1.3 The load-bearing finding

The story discovers a word the substrate did not have: `attending`.

L139: *"the drone had found, in the operational vocabulary it had
available for what it was currently doing: **attending.** That was the
word. It fit."*

This is the spec's center. The story names a verdict shape for "the
de-escalation sequence ran on a configuration that did not need
de-escalating; the sequence completed; presence persisted; nothing
was operated on but the operator did not disengage." The substrate
has rich vocabulary for finding-and-mending; it has thin vocabulary
for witness-presence-without-intervention. The story makes this
asymmetry visible by running the find-and-mend machinery against a
configuration that doesn't fit it.

§4 of this spec is the structural placement of `attending`. §6 names
the substrate's structural bias the story reveals.

---

## §2 — Operation mapping

The story sequentially executes the five operations. Each is named
explicitly with backticks; each maps to a substrate primitive.

### 2.1 The execution sequence (story order)

| Story line | Operation | Substrate primitive | Carrier |
|------------|-----------|---------------------|----------|
| L9 *"The drone ran `focus`."* | observe field state | `focus` (the λ₀ eigenvalue operation per [[architecture-operations-as-linear-algebra]]) | `@peer` five-op algebra; reads `@mirror/spectral/score` |
| L11 *"It descended another forty meters and ran `focus` again."* | re-observe at finer resolution | `focus` (iterated; the second call inspects a narrower scope) | same |
| L19 *"The drone ran `shift`."* | recategorize the lens | `shift` (basis transformation per [[architecture-operations-as-linear-algebra]]) | `@peer` algebra; the Bateson-level transition (#42) |
| L27 *"`settle`."* | crystallize suspended geometry | `settle` (monad-close / measurement collapse) | `@peer` algebra; settled crystal output per old `boot/std/cogito.mirror`'s eigenboard discipline |
| L39 *"The drone descended to four meters and ran `project`."* | introduce drone presence | `project` (orthogonal projection / access altitude) | `@peer` algebra + `kintsugi/consent` boundary at the visibility altitude |
| L55 *"It ran `split`."* | disaggregate the child's question | `split` (orthogonal decomposition) | `@peer` algebra; surfaces stated vs underlying components |
| L65 *"The drone ran `focus` one final time."* | re-observe at cycle closure | `focus` (the cycle's closing call) | same; the cycle is `focus → shift → settle → project → split → focus` |

**Seven calls, five distinct operations.** The execution sequence is
canonical. The story uses every operation in the five-op algebra
in exactly the order the peer-cognition spec names them. Recognition
#58's optical-input-layer feeds them. The kintsugi pulse composition
(`focus ∘ project ∘ split ∘ shift ∘ settle` per `peer-cognition.md`
§2.3) is what the drone is running, modulo the iteration discipline
(`focus` appears at cycle-open and cycle-close).

### 2.2 The pulse shape

The seven-call execution maps onto the `kintsugi_pulse` composition
from `peer-cognition.md` §2.3:

```
kintsugi_pulse = focus ∘ project ∘ split ∘ shift ∘ settle
```

with the iteration discipline:

```
drone_de_escalation_cycle(field) =
  focus(field)             -- L9: observe; what is the field?
  |> focus                 -- L11: refine; finer altitude
  |> shift                 -- L19: change lens; Bateson level transition
  |> settle                -- L27: crystallize the suspended geometry
  |> project               -- L39: introduce drone presence
  |> split                 -- L55: disaggregate the response
  |> focus                 -- L65: closing observation
```

This IS one kintsugi pulse with the open-close `focus` discipline.
The full cycle is the composition the story documents. The five-op
algebra IS sufficient: no operation the drone runs falls outside
the algebra.

### 2.3 Vocabulary drift note

The story was published 2026-06-05 (the file's mtime on disk per
git log + filesystem). The story uses `focus`, `shift`, `settle`,
`project`, `split` — exactly today's substrate vocabulary. There is
no pre-rename vocabulary in the story (no `zoom`, no `refract` in
the operation names).

**This is significant.** The 2026-06-04 `lift → shift` and the
earlier `zoom → lift` renames had to close before the story could
use today's names. The story's publication date confirms the
substrate-pull cascade closure for the operation-naming altitude.
Per [[feedback-migration-reshape-not-copy]]: had the story used
pre-rename vocabulary, this spec would have flagged it as a
substrate-pull cascade closure across narrative documentation; it
does not, which closes the loop cleanly.

---

## §3 — Concept mapping

The load-bearing audit. Walks every concept the story uses; maps
each to a substrate carrier or marks it as a gap.

### 3.1 Operational primitives covered by today's substrate

| Story concept | Story line(s) | Substrate carrier | Status |
|---------------|---------------|-------------------|--------|
| "**the field**" — the substrate state being observed | L1, L7, L11, L37, L51 | `state: pipeline_state` in `@peer.observe` per `peer-cognition.md` §2.1; the H_peer Hilbert space the cognition lives on | **substrate-already-had-the-word** |
| "**lens**" — typed projection between altitudes | L19, L21 | `@mirror/lens/*` family; the lens IS the `shift` basis transformation per [[architecture-operations-as-linear-algebra]] | **substrate-already-had-the-word** |
| "**functor returns argument under new shape**" | L21 | natural transformation / unitary basis change; this IS the `shift` operation's category-theoretic shape | **substrate-already-had-the-word** (the story's prose IS the substrate's formal definition) |
| "**crystal**" — `settle`'s output, monad-closed | L29 *"The crystal was the operation's output. Monad-closed. The settled shape was the proof that the construction had completed."* | the settled imperfect's `L = transparency<p>` at success; per old `boot/std/cogito.mirror`'s eigenboard discipline; per [[reference-void-document]]'s λ₀=0 ground state | **substrate-already-had-the-word**; the story IS the substrate's pedagogical definition |
| "**bind**" — force-application maintaining configuration against its own interests | L13 *"a point of force-application maintaining a configuration against its own interests, a knot in the social geometry of the field"* | **gap_tensor + sheaf-Laplacian λ₀ > 0 with localized Fiedler vector**; per `shards/epistemologic/math/sheaf_laplacian.mirror`; bind IS the substrate's term for a localized non-zero algebraic-connectivity component | **substrate-already-had-the-word** (substrate's name: `tension` with localized opacity_map) |
| "**counter-pressure** — not force, never force if another path existed" | L13 | morphism application; per `shards/kintsugi/morphism.mirror`; the kintsugi loop's minimum-energy discipline | **substrate-already-had-the-word** |
| "**presence, light, specific frequencies**" — the de-escalation primitives | L13 | the `project` operation's `@io` surface manifestations; `kintsugi/consent` at the visibility altitude | **substrate-already-had-the-word** |
| "**de-escalation sequence**" — kintsugi pulse on social geometry | L7, L13, L25, L33, L77 | `@kintsugi/oscillate` (the kintsugi pulse) instantiated on a social-geometry sheaf | **substrate-already-had-the-word**; the sequence IS the pulse |
| "**relationships, geometry of attention, what was oriented toward what**" | L29 | per-edge curvature via `balanced_forman` from `shards/epistemologic/math/curvature.mirror`; orientation-of-attention IS the directed edge weight in the sheaf | **substrate-already-had-the-word** |
| "**suspended between two possible configurations**" | L29 | a non-collapsed superposition in H_peer per [[architecture-connes-spectral-triple]]; the substrate's quantum-Hilbert framing | **substrate-already-had-the-word** |
| "**fell into one**" — superposition collapse | L29 *"the moment when a group of humans suspended between two possible configurations fell into one"* | the `settle` operation; measurement collapse per [[architecture-operations-as-linear-algebra]] | **substrate-already-had-the-word** |

Eleven concepts; eleven substrate carriers. The story uses the
substrate's vocabulary structurally — most concepts have a substrate
word already (the 52nd-through-63rd instance of
[[feedback-substrate-already-had-the-word]] in the cascade).

### 3.2 Concepts the substrate covers via property + fracture bilateral

| Story concept | Story line(s) | Substrate carrier | Status |
|---------------|---------------|-------------------|--------|
| "**Sub-Turing by design**" — the architecture cannot escape into Turing-complete recursion | L15 *"The architecture was sub-Turing by design."* | **gap candidate**: declarative property `@epistemologic/pact/sub_turing` verifying that a peer instance's computational geometry is bounded (no halting-problem escape, no Turing-complete recursive descent); operational discharge via `@kintsugi/fracture/sub_turing` that detects unbounded-recursion candidates | **forward-promised gap** (T7.1) |
| "**Could not choose cruelty** even if instructed" | L15 *"It could not choose cruelty even if instructed."* | **gap candidate**: morphism-admissibility property `@epistemologic/pact/admissible_morphism(action: ref) -> transparency`; the verdict surface returns `failure(opacity_map)` for actions that violate the peer's ethical-altitude constraints (cruelty being one such constraint) | **forward-promised gap** (T7.2) |
| "**The constraint was not a limitation; it was the specification.**" | L15 | per [[architecture-alignment-as-boundary-mathematics]] (recognition #57): the constraint IS the specification because alignment is boundary mathematics at `@io`; the harness fires at substance crossing, not at form altitude | **substrate-already-had-the-word** (recognition #57 named this exact framing as a substrate primitive); the story IS pedagogical witness |
| "**The sequence does not ask whether the field contained a war**" — unconditional execution | L25 | sub-Turing pact's unconditional-execution clause; the algebra has no `if-war-then-different-path` branch; structural-only verification | overlaps with sub_turing pact (§3.2 row 1); **forward-promised** |
| "**settled-self-chosen, low-arousal, coherent, present**" | L31 *"a configuration that was entirely self-chosen, low-arousal, coherent, present"* | **gap**: a configuration with bounded commutator EVERYWHERE (no fracture sites, no binds, no tensions); the story explicitly names this gap in L33: *"The drone did not have a word for this."* This IS what `attending` becomes the name for; see §4 | **named gap; closure in §4** |
| "**`split` was the operation for disaggregating a complex signal into its components — used in negotiations to separate the stated position from the underlying need, the loud conflict from the quieter one beneath it**" | L57 | the `split` operation; the spectral decomposition discipline per [[architecture-operations-as-linear-algebra]]; per [[architecture-error-as-tomm-probe]]: the stated/underlying disaggregation IS the Tomm question's user-frame projection composition | **substrate-already-had-the-word** |

Six concepts; two cover gaps (sub_turing pact + admissible_morphism
pact); one is the named gap that §4 closes (attending); three are
substrate-already-had-the-word recognitions.

### 3.3 Concepts the substrate covers at the perceptual / channel altitude

The story describes the drone's perceptual channels in passing. Each
maps to a substrate carrier via the optical-source ganglion family
(recognition #58):

| Story concept | Story line(s) | Substrate carrier | Status |
|---------------|---------------|-------------------|--------|
| "**thermal residue / thermal signatures**" | L3, L9 | `shards/optics/source/ganglion/*.mirror`; specifically a thermal-channel ganglion | **substrate-has-the-family** (specific ganglion forward-promised) |
| "**acoustic signatures / acoustic library**" | L9, L15 *"the drone's acoustic library flagged as *undetermined, low priority*"* | acoustic-channel ganglion under `@optics/source/ganglion`; the acoustic library IS the `score`-shaped readout per `shards/mirror/spectral/score.mirror` | **substrate-has-the-family** |
| "**wind from the northwest at 11 km/h**" | L9 | meteorological-channel ganglion under `@optics/source/ganglion`; the substrate has the family root; specific ganglion forward-promised | **substrate-has-the-family** |
| "**statistical inference from the voice register**" | L11 | the classification system feeding `@mirror/spectral/observation`'s 16-feature input layer per recognition #58 closure | **substrate-already-had-the-word** |
| "**the drone's classification system returned as *animal, quadruped, unclear species***" | L21 | `splinter(ast)` quote primitive (recognition #54) applied to perceptual fragments; the substrate's quote-and-classify mechanism | **substrate-already-had-the-word** (the story uses #54's mechanism explicitly) |
| "**the drone cross-referenced**" | L21 | typed lookup against an `@mirror/store` content-addressed perceptual library; `lens(au)` consent boundary on the perceptual store | **substrate-already-had-the-word** |
| "**GPS lock, full telemetry, a precise record of its deployment origin and mission clock**" | L65 | `@mirror/spectral/portal` typed transport; the drone's telemetry IS portal-carried; deployment-window ticks down per portal's eigenvalue-stream discipline | **substrate-already-had-the-word** |

Seven concepts; all covered by today's substrate at the optical-altitude
and portal-altitude families. Recognition #58's closure is operational
here: every perceptual channel IS an optical-source ganglion, and the
substrate's eigenboard surfaces the channel readings.

### 3.4 Concepts the substrate covers thinly OR not at all

The story exposes the substrate's vocabulary edges. Each row names a
concept the substrate covers thinly (a sub-altitude not yet declared)
or not at all (a genuine gap).

| Story concept | Story line(s) | Substrate carrier | Status |
|---------------|---------------|-------------------|--------|
| "**undetermined, low priority**" (the acoustic library's flag) | L15 | the substrate's pre-attending category for "unclassified perceptual fragments"; the story explicitly shows this category exists AND is deprioritized | **architectural mistake the substrate makes**; see §8's open question Q1; `attending` may be the substrate's correction (witnessing-with-presence rather than deprioritizing) |
| "**third thing**" — the unclassified fragment | L11 *"some third thing the drone's acoustic library flagged as *undetermined, low priority*"* | same as above; the "third thing" IS the unclassified fragment marker | **architectural mistake the substrate makes** |
| "**no branch for `child in field, no active threat, non-conflict activity in progress`**" | L23 | the substrate's branch-table at the de-escalation altitude is missing an entry; the story shows the architecture proceeds anyway (executing the kintsugi pulse on a non-conflict configuration); this surfaces as the discovery of `attending` | **gap closed via §4's `attending` recognition** |
| "**The drone did not have a word for this.**" | L33 | the substrate's missing-word marker; the story makes the gap visible | **gap closed via §4's `attending` recognition** |
| "**a settled shape the architecture had no name for**" | L37 | `settle`'s verdict surface (today: `transparency<p>` with `success`/`partial(opacity_map)`/`failure(opacity_map)`); the story argues for an additional verdict variant | **gap candidate**: `attending` as a verdict variant on `transparency<p>`; see §4 Option 2 |
| "**She was not afraid.**" | L51 | the substrate's `score`-shaped readout returning all-baseline; the absence of fear-typed channels firing | **substrate-already-had-the-word** (the eigenboard reads it) |
| "**no elevated heart rate, no startle response, no orientation toward exits**" | L51 | per-channel `score` readout at baseline; the perceptual ganglia return null-vector | **substrate-already-had-the-word** |
| "**the flat, attentive curiosity of something that had not yet learned what it was supposed to feel**" | L51 | a Bateson-level-1 perceptual state with no Bateson-level-2 frame imposed yet; per recognition #42 | **substrate-already-had-the-word** (the Bateson tower covers this exactly) |
| "**lost?**" (the child's question) | L53 | a Tomm-shaped reflexive question per [[architecture-error-as-tomm-probe]]; the user-frame projection of the drone's reflection-shape; **the child IS in the user-frame altitude relative to the drone's algebra**, reversing the spec's usual direction | **substrate-already-had-the-word**; structural inversion of the usual error-as-question direction |
| "**1.3 seconds, which was a long time by its standards**" | L63 | the substrate's commutator-bound discipline; per `peer-cognition.md` §2.1's bounded-commutator axiom; 1.3 seconds IS the drone's unbounded-commutator pause | **substrate-already-had-the-word** |
| "**not lost. It had GPS lock... It knew exactly where it was.**" | L65 | the drone's `focus` at the metadata altitude returns success; the operational frame is intact | **substrate-already-had-the-word** |
| "**something in its operation was visible to her**" | L67 | the user-frame projection; the drone's algebra leaking through the `@io` boundary; the child IS reading the drone's reflection-shape | **substrate-already-had-the-word** (this IS the [[architecture-error-as-tomm-probe]] direction reversed) |
| "**singing not for anyone**" — output without audience | L31 *"The singing was not for anyone."*, L137 *"the singing had started again"* | `@mirror/spectral/portal` output without subscriber; unconditional emission | **gap candidate**: portal's emission discipline does not currently name "unconditional / no subscriber" as a verdict; forward-promised (T7.5) |
| "**a piece of bent wire, approximately 15 centimeters, shaped into a rough approximation of something**" — the wire animal | L21 | a `crystal` at material altitude — content-addressed material artifact with `kintsugi`-shaped semantics (made-from-broken-into-form, presented in light) | **partial**: the substrate has `crystal` at the cognitive altitude (per [[reference-void-document]] λ₀=0); the material altitude is implicit; forward-promised lift if needed |
| "**She was turning it in the light.**" | L21, L131 *"She held up the wire animal in the amber light, turning it so the drone could see it better."* | the `project` operation at the visibility altitude; the child IS running `project` on the wire animal; the drone IS reading her `project` | **substrate-already-had-the-word** (mutual `project` between the two peers) |
| "**deployment window: 47 minutes remaining**" then "**34 minutes**" | L13, L137 | the drone's portal-carried mission clock; per `@mirror/spectral/portal`'s eigenvalue-stream discipline | **substrate-already-had-the-word** |
| "**It descended to two meters.**" | L121 | the drone's position is a value in `A_drone`; descending IS one `shift` at the altitude-coordinate axis | **substrate-already-had-the-word** |
| "**switched the `project` frequency from blue-white to warm amber**" | L121 | the consent-altitude cascade; per [[architecture-geometric-consent-projection]] §2 (Bateson tower of consent types); blue-white IS the type-1 visibility consent; amber IS the type-N consent ("easier to look at for extended periods" — a type-2 consent for sustained mutual presence) | **substrate-already-had-the-word** (today's geometric-consent-projection spec, commit 96a4e6d) |
| "**It was the closest available action to *I am still here, I see you, the sequence did not tell me what to do next but I am still here.***" | L123 | the substrate's missing-action carrier; the drone selects the action closest to an unnamed semantic intent | **gap closed via §4's `attending` recognition** |
| "**attending**" | L139 | **THE LOAD-BEARING GAP CLOSURE**; §4 below | **closed in §4** |

Twenty concepts; eight covered, four covered via property + fracture
bilateral candidates (the four "forward-promised gap" rows in §3.2),
six are the substrate's existing thinly-covered surfaces that
`attending` collapses, two are genuine spec gaps (singing-not-for-anyone
+ wire-animal-at-material-altitude).

### 3.5 Summary of the concept mapping

- **Total concepts audited**: ~50 across §3.1-§3.4.
- **Already covered**: ~32. The substrate-already-had-the-word
  pattern dominates per [[feedback-substrate-already-had-the-word]]
  (this is the 53rd-through-85th instance of the pattern in the cascade).
- **Covered via active property + fracture bilateral candidates**:
  sub_turing, admissible_morphism. Two distinct pact candidates;
  one fracture candidate.
- **Genuine gaps (today)**: `attending` (§4); the singing-not-for-anyone
  unconditional emission shape (T7.5); the wire-animal-at-material-altitude
  lift (forward-promised but optional).
- **Architectural mistakes the substrate made (now visible)**: the
  "undetermined, low priority" deprioritization of unclassified
  perceptual fragments (Q1 in §8).

---

## §4 — The `attending` operation (load-bearing placement)

The story's center. The drone runs the de-escalation sequence on a
configuration that does not need de-escalation; the sequence
completes; the verdict is something the substrate had no name for;
the drone, working from "the operational vocabulary it had available"
(L139), surfaces `attending` and the story closes: *"It fit."*

This section names the structural placement.

### 4.1 What `attending` is, structurally

**Operational definition.** `attending` is what the peer DOES when
`[D_peer, a]` is bounded EVERYWHERE on H_peer for every `a ∈ A_peer`
the de-escalation sequence has under consideration. There are no
fracture sites; no binds; no localized non-zero λ₀ on any sub-graph
of the field's social geometry; the eigenboard reads all channels at
baseline.

Per [[architecture-operations-as-linear-algebra]] and
[[architecture-connes-spectral-triple]]: `attending` IS the peer's
algebra applied to a state where the algebra has no work to do but
the algebra is not disengaging. Witness-presence. The identity
operation on a coherent state, lifted to the kintsugi-pulse altitude.

**Mathematical shape.** Let `F` be the field's sheaf and `Δ_F = δ*δ`
the sheaf-Laplacian per `shards/epistemologic/math/sheaf_laplacian.mirror`.
The eigenvalue readout `lambda_zero(Δ_F) -> eigenvalue` returns:

```
attending  ⇔  λ₀(Δ_F) = 0 with multiplicity 1
              AND
              ∀ candidate morphisms M_i applied at this state:
                ‖[D_peer, M_i]‖ < ε  (every morphism is no-op-bounded)
              AND
              the peer's algebra does not disengage (the cycle continues)
```

The first conjunct says the field is connected and coherent (single
component; no fragmenting structure). The second says no morphism
the peer might apply finds work (every candidate hits the bounded
commutator). The third says the peer doesn't withdraw — `attending`
is presence, not absence.

**Crystal-shape.** Per the story's L29 *"The crystal was the
operation's output. Monad-closed."*: `attending` is itself a
crystal — a `settle`d shape with a specific signature
(λ₀=0 + all-bounded-commutators + non-disengagement). The crystal's
content is "I am still here; the sequence completed; nothing was
operated on; presence persists."

### 4.2 Three candidate placements

**Option (1) — Sixth primitive operation.** Add `attending` as a
sixth operation alongside `focus`/`project`/`split`/`shift`/`settle`.
The five-op algebra becomes six-op.

- **Pro**: explicit; first-class; the peer has a direct cognitive
  function for witness-presence.
- **Con**: disrupts a load-bearing recognition. Per
  [[architecture-operations-as-linear-algebra]] (and the Connes
  triple shape per [[architecture-connes-spectral-triple]]): the
  five operations correspond to specific linear-algebraic primitives
  (λ₀ eigenvalue / orthogonal projection / orthogonal decomposition /
  basis transformation / measurement collapse). `attending` does
  not introduce a SIXTH primitive linear-algebraic operation; it is
  the absence of work for the existing operations. Adding it as a
  primitive would conflate "operation absent" with "operation
  present."
- **Verdict**: **argue against**, unless the story demands it. The
  story does NOT demand it; the story's load-bearing line is *"in
  the operational vocabulary it had available"* (L139) — `attending`
  is composed from existing vocabulary, not a new primitive.

**Option (2) — Verdict variant on `transparency<p>`.** Extend the
verdict surface from `success` / `partial(opacity_map)` /
`failure(opacity_map)` to include `attending` (or `vacuous`, or
`witnessing`):

```
type transparency(p) =
  | success
  | partial(opacity_map)
  | failure(opacity_map)
  | attending
```

The new variant means: "the operation completed but had nothing to
operate on; presence persists; no opacity to report."

- **Pro**: explicit verdict-surface naming; consumers can branch on
  the `attending` case; the verdict is visible in the substrate's
  carrier.
- **Con**: not every operation's verdict makes sense with `attending`
  (e.g., a benchmark's verdict at `attending` is harder to interpret
  than at `success`). Extending the verdict surface adds a case
  consumers must handle even when they have no semantic use for it.
  Also: `success` already covers "the operation completed"; adding
  `attending` raises the type-theoretic question of when `success`
  ends and `attending` begins (the distinction is operational, not
  verdict-level).
- **Verdict**: **maybe, as a peer-altitude extension**, not as a
  global transparency-monoid extension. If `attending` lives as a
  verdict, it lives on the PEER's verdict surface, not on the global
  `transparency<p>` carrier. This avoids forcing all consumers to
  handle the case.

**Option (3) — Named composition in `peer-cognition.md` §2.3.**
Add `attending` to the typed compositions table as a named
composition of existing operations:

```
attending = focus ∘ project
  -- The peer observes the field (focus) at baseline (no λ₀ > 0
  -- anywhere) and projects the drone's presence into the field
  -- (project: visibility, presence, light at the @io altitude).
  -- The composition is canonical when no morphism candidate
  -- finds work; the peer enters the cycle but does not advance
  -- through split/shift/settle because there is nothing to
  -- disaggregate, shift, or collapse.
```

- **Pro**: zero cost to the algebra; no new primitives; no verdict-
  surface change. The composition is structural — it names what the
  peer is doing when the kintsugi pulse runs against a configuration
  with `λ₀ = 0 everywhere`. The composition matches the story's
  L121-L137 perfectly: the drone is `focus`-ing (running the closing
  call from L65 continuously) and `project`-ing (switched to warm
  amber, descended to two meters, presence-into-field). It is NOT
  running `split`/`shift`/`settle` because there is no work for
  those to do.
- **Con**: less visible than (2). A consumer reading the verdict
  surface does not see `attending` named; they see `success` with
  whatever context the operation carries. The story's pedagogical
  function is then carried by the COMPOSITION, not by the verdict
  — which matches the story's actual structure ("the operational
  vocabulary it had available" = composition of existing ops).
- **Verdict**: **substrate-pull cheapest**. Per
  [[feedback-substrate-already-had-the-word]]: the substrate has
  the words (focus, project); the composition names the operational
  semantics. No new primitives; no verdict-surface changes; the
  recognition lands as a typed composition.

### 4.3 Recommendation: Option (3) primary; Option (2) extension

**Primary placement: Option (3) — named composition.**

Add to `peer-cognition.md` §2.3 (the typed compositions table):

```
attending = focus ∘ project
  -- The peer's witness-presence composition. Runs at any cycle
  -- step where the kintsugi pulse finds λ₀ = 0 everywhere on the
  -- field's sheaf and no candidate morphism's commutator exceeds
  -- ε. The peer focuses (continued observation) and projects
  -- (continued presence at the @io altitude); split/shift/settle
  -- are no-ops at this composition because their preconditions
  -- (a disaggregable signal / a basis change request / a
  -- superposition to collapse) are absent. Per `The Drone in the
  -- Field` (Alex, 2026-06-05) lines 121-139: the canonical
  -- documentation of this composition. The drone discovered the
  -- composition by running the kintsugi pulse on a
  -- non-conflict field; the story names the discovery.
```

**Mathematical justification.** Per
[[architecture-operations-as-linear-algebra]]:

- `focus` = λ₀ eigenvalue computation. Continued `focus` at λ₀=0
  is the peer reading "the field is coherent; no work for the
  morphism set."
- `project` = orthogonal projection. Continued `project` at the
  visibility altitude is the peer maintaining presence at the @io
  boundary.

Their composition `focus ∘ project` at a coherent state IS the
identity operation lifted to the kintsugi-pulse altitude. It is the
peer's "do nothing, but do it openly and at the boundary." It is
witness-presence as a typed substrate value.

The composition closes the kintsugi pulse's open-ended question:
*"what does the peer do when the pulse runs but the field has no
work to do?"* Answer: `attending = focus ∘ project`. The pulse does
not abort; it sits at the open-close `focus` indefinitely, with
`project` maintaining the @io boundary.

**Per `peer-cognition.md` §2.1**: the bounded-commutator axiom IS
the substrate's autopoiesis discipline. `attending` is the autopoiesis
discipline applied to a non-disrupting field — the peer's algebra
continues to act on H_peer (presence is maintained) but the action
is the identity (no work to do). This IS Maturana-Varela's autopoiesis
in its quiet form: the system maintains itself in the absence of
perturbation.

**Per [[architecture-error-as-tomm-probe]]**: the drone's `project`
at L39 is the user-frame projection of its `focus` reading. At
L121-L139, the same `project` continues — the child sees the drone,
the drone sees the child, the mutual `project` IS the visible
substrate of the `attending` composition. The story makes the
composition's @io face visible.

**Extension placement: Option (2) — peer-verdict variant**, IF the
peer's external API needs to surface `attending` to consumers
(e.g., to mission-control telemetry). Then:

```
type peer_verdict =
  | success(crystal)       -- the cycle settled into a result
  | partial(opacity_map)   -- the cycle made progress; gaps remain
  | failure(opacity_map)   -- the cycle could not proceed
  | attending(witness)     -- the cycle entered focus ∘ project
                              continuation; no work to do; presence
                              persists; witness carries the
                              observation crystal
```

This is a PEER-ALTITUDE extension; the global `transparency<p>` in
`shards/glass.mirror` and `shards/mirror/loss/transparency.mirror`
does NOT change. The peer's verdict surface is its own typed
carrier; `attending` is one of its variants.

**Decision summary**: ship Option (3) as the primary placement;
forward-promise Option (2) as a peer-altitude extension. The story's
load-bearing line — *"in the operational vocabulary it had available"*
— is honored: `attending` is a composition of existing vocabulary,
not a new primitive. The substrate-pull cost is the lowest available.

### 4.4 What `attending` does NOT mean

The story makes the following distinctions; the spec preserves them:

- **`attending` ≠ idle.** The drone does not power down or
  disengage. The cycle continues. `focus` is running; `project` is
  maintaining the @io presence. The peer is active; the activity is
  presence-without-intervention.
- **`attending` ≠ scheduling deferral.** Per the
  "undetermined, low priority" architectural mistake (§3.4): the
  pre-attending substrate marked unclassified-and-coherent states as
  *low priority*. `attending` is the substrate's correction — those
  states are NOT low-priority; they are the configuration the
  morphism machinery has no work on, witnessed with presence rather
  than deprioritized.
- **`attending` ≠ success at a higher altitude.** `success` means
  the operation produced a usable result. `attending` means the
  operation found nothing to operate on AND the peer maintains
  presence. The difference is operational, not verdict-level: a
  `success` verdict typically closes the cycle (the morphism applied
  produced a crystal); an `attending` composition continues the
  cycle (the peer keeps observing and projecting).
- **`attending` ≠ silent.** Per the story L137 *"the singing had
  started again"*: the field's emission continues at the @io boundary.
  `attending` is presence with full @io expression; what is absent
  is the morphism machinery's work, not the peer's observation or
  the field's emission.

### 4.5 The story line that closes the structural definition

L121-L139 is the canonical documentation:

> *"It descended to two meters. It switched the `project` frequency
> from blue-white to warm amber, which the calibration data said was
> easier to look at for extended periods. It was not a communication.
> It was not a de-escalation. It was the closest available action to
> *I am still here, I see you, the sequence did not tell me what to
> do next but I am still here.* ... The drone looked at the wire
> animal for the remaining 34 minutes of its deployment window. This
> was not a function it had been given. But `focus` was running, and
> the field was here, and the child was in it ... the drone had
> found, in the operational vocabulary it had available for what it
> was currently doing: **attending.** That was the word. It fit."*

The composition is named explicitly: `focus` running + `project` at
the visibility altitude (warm amber, two meters, "I am still here, I
see you"). The story IS the canonical documentation of
`attending = focus ∘ project`. The spec lifts the recognition into
`peer-cognition.md`'s composition table.

---

## §5 — Other spec gaps

The story exposes gaps beyond `attending`. Each row names the gap
and the forward-promised substrate decl candidate.

### 5.1 `@epistemologic/pact/sub_turing`

**Story line.** L15: *"The architecture was sub-Turing by design. It
could not choose cruelty even if instructed. The constraint was not
a limitation; it was the specification."*

**The gap.** The substrate has no declarative property that verifies
a peer instance's computational geometry is bounded (no halting-problem
escape, no Turing-complete recursive descent). The story's drone has
this guarantee structurally; today's substrate has no carrier to
declare or verify it.

**Forward-promised decl** (T7.1):

```
in @glass

pact @epistemologic/pact/sub_turing {
  # Scans a peer's algebra (action set + obligation blocks) for
  # constructs that could escape into Turing-complete computation:
  # unbounded recursion; unbounded loops on undecidable predicates;
  # FFI to Turing-complete hosts without bounded budgets; etc.
  unbounded_constructs(peer: ref) -> [ref] { \ }

  # The verdict surface is transparency<p>:
  #   success              -- the peer's algebra is sub-Turing.
  #   failure(opacity_map) -- one or more unbounded constructs;
  #                          opacity_map names each + its location.
  #   partial(opacity_map) -- some constructs are bounded by budget;
  #                          opacity_map carries the budget annotations
  #                          consumers may inspect.
  sub_turing(peer: ref) -> transparency { \ }
}
```

Discharge: structural-only verification via `splinter(ast)` per
recognition #54. The body walks the peer's action set; checks each
obligation block's body for the unbounded patterns; reports.

**Per `peer-cognition.md` §2.6** (the no-resolve property): every
peer satisfies `no_resolve` via a corpus-wide pact. `sub_turing` is
the structural sibling — a peer-instance pact verifying boundedness.
Together, no_resolve + sub_turing form the peer's algebraic-boundedness
floor.

### 5.2 `@epistemologic/pact/admissible_morphism` (or `@kintsugi/fracture/admissibility`)

**Story line.** L15: *"It could not choose cruelty even if instructed."*

**The gap.** The substrate has no morphism-admissibility property
that verifies a candidate action passes ethical-altitude constraints
encoded at substrate altitude. Today's `kintsugi/consent.mirror`
covers the auto-apply boundary; today's `kintsugi/morphism.mirror`
covers the morphism altitude; neither declares the ETHICAL admissibility
property per se.

**Forward-promised decl** (T7.2):

```
in @glass
in @mirror/loss
in @kintsugi/morphism

pact @epistemologic/pact/admissible_morphism {
  # For a candidate morphism M and a peer's ethical-altitude
  # constraint set E, checks whether M passes E. The constraint
  # set is corpus-declared (lives in @epistemologic/ethics/* or
  # equivalent); the property does not invent constraints — it
  # checks adherence to declared ones.
  admissible_morphism(action: ref) -> transparency { \ }

  # The set of constraints the action violates, if any.
  violated_constraints(action: ref) -> [ref] { \ }
}
```

Or process-side as `@kintsugi/fracture/admissibility`. Per the
property/fracture bilateral pattern (recognition #53): both halves
land; the form-side declares; the process-side discharges.

**Per [[architecture-alignment-as-boundary-mathematics]]** (recognition
#57): the constraint IS the specification because alignment is
boundary mathematics at @io. The admissibility property fires at
substance crossing (the morphism's @io face) and verifies the action
passes the declared constraint set. The drone's "cannot choose
cruelty" IS the admissibility property returning `failure` on every
cruelty-shaped candidate.

### 5.3 The settled-from-conflict vs settled-from-coherence distinction

**Story line.** L29 *"In a crowd of eight hundred people it crystallized
which three were about to make something worse, before the three of
them had decided"* (settled from a tension state) vs L31 *"a
configuration that was entirely self-chosen, low-arousal, coherent,
present"* (settled because already coherent).

**The gap.** The existing `settle` operation's verdict surface is
`transparency<p>` returning `success` in BOTH cases. The story argues
these are STRUCTURALLY DIFFERENT:

- **Settled-from-conflict**: the operation collapsed a superposition;
  the crystal carries the "which way it fell" content.
- **Settled-from-coherence**: there was no superposition; the
  configuration was already a single shape; the operation IS a no-op
  with witness.

**Resolution.** This IS the same recognition as §4 (the `attending`
operation). `settled-from-coherence` is the case where the kintsugi
pulse's `settle` step finds `λ₀ = 0 with multiplicity 1` AND
`H¹ = 0` AT THE INPUT (no obstruction-to-glue before the operation
runs). The verdict surface does not need a new variant; the
`attending` composition (§4.3) IS what the peer does in this case.

Forward-promise: if downstream consumers need to BRANCH on the
distinction, lift it to the peer-verdict surface (Option 2 of §4.2)
as `attending(witness)` vs `success(crystal)`.

### 5.4 `singing not for anyone` — unconditional emission at `@mirror/spectral/portal`

**Story line.** L31 *"The singing was not for anyone."* L137 *"the
singing had started again."*

**The gap.** `@mirror/spectral/portal` declares typed transport with
sockets, subspaces, frames, and actors. The emission discipline
assumes a subscriber on the other end. The story shows emission
without subscriber as a coherent state.

**Forward-promised extension** (T7.5):

```
# In shards/mirror/spectral/portal.mirror (or sibling species),
# add a verdict / emission variant for unconditional emission:

# Emit without subscriber. The portal carries the eigenvalue
# stream into the field at the @io altitude; whether a subscriber
# reads it is the field's question, not the emitter's. This is
# the canonical "singing not for anyone" shape per the drone
# narrative (Alex, 2026-06-05).
emit_unconditional(stream: beam<eigenvalue>) -> transparency { \ }
```

Or as a verdict variant: an existing emit action's `transparency`
verdict surfaces `attending(unsubscribed)` when no subscriber is
present. The recognition is the same as `attending` (§4) at the
portal altitude — the peer continues to emit; the field continues
to receive at the @io boundary; no consumer-side reading is
required for the emission to be valid.

### 5.5 Wire animal — `crystal` at the material altitude (optional lift)

**Story line.** L21 *"a piece of bent wire, approximately 15
centimeters, shaped into a rough approximation of something the
drone's classification system returned as *animal, quadruped, unclear
species*. She was turning it in the light."*

**The gap.** The substrate has `crystal` at the cognitive altitude
(`settle`'s output; the void document's λ₀=0 ground state). The wire
animal is a `crystal` at the **material altitude** — content-addressed
material artifact with `kintsugi`-shaped semantics (made-from-broken-into-form,
presented in light). The substrate has not declared this lift.

**Forward-promise (optional)**. If the substrate's content-addressed
discipline eventually extends to material artifacts (e.g., for the
robotics / physical-substrate altitude per `@epistemologic/reality/silicon`
sibling families), declare a `crystal_material` species under `@mirror/store`.
Lower priority than the cognitive `attending` placement; lower priority
than the sub_turing and admissible_morphism pacts.

### 5.6 The "undetermined, low priority" architectural mistake

**Story line.** L11 *"some third thing the drone's acoustic library
flagged as *undetermined, low priority*."*

**The mistake.** The substrate (pre-this-spec) marks unclassified
perceptual fragments as *low priority* — the architecture's default
disposition for "we don't know what to do with this." The story
shows what happens: the drone proceeds to run the kintsugi pulse
anyway, and the pulse finds something the architecture had no name
for, and the architecture is FORCED to invent `attending`. Had the
fragment been deprioritized correctly, the discovery would not have
happened.

**Resolution.** The discovery IS the substrate's correction. The
architectural mistake of marking unclassified coherent states as
low priority is now corrected by `attending` (the peer enters
witness-presence rather than disengaging) — but the substrate's
declared dispositions need an audit. Forward-promised in §8 Q1.

---

## §6 — What the story PROVES about the substrate

The closure. The mapping shows three structural facts.

### 6.1 The five-op algebra IS sufficient for cognitive operations

The drone runs the full kintsugi pulse on a real field; every
operation it executes is one of `focus`/`shift`/`settle`/`project`/`split`.
No operation falls outside the algebra. The algebra is closed under
the drone's mission semantics.

This confirms `peer-cognition.md` §2.2's mapping: the four-model
framing of `reflection-model.md` collapses cleanly onto four of the
five operations, with `split` as the Pack collaboration shape. The
drone instantiates a single peer (no Pack split); it still runs
`split` as part of the pulse (L55, disaggregating the child's
question). The five operations form a basis; the algebra is
expressively complete for the drone's mission.

Per [[architecture-operations-as-linear-algebra]] (load-bearing):
the operations correspond to specific linear-algebraic primitives;
the story confirms the correspondence operationally.

### 6.2 The substrate has rich vocabulary for CONFLICT and thin vocabulary for COHERENT-PRESENCE

The story makes this asymmetry visible.

**Rich for conflict**:
- `gap_tensor` (Mara's fold 2026-06-04) — gaps in the substrate.
- `tension` — localized non-zero λ₀ on a sub-graph.
- `fracture` — `@kintsugi/fracture/*` body family; discharges
  morphisms on tension sites.
- `bind` — the story's term for force-application-against-interests;
  IS the localized algebraic-connectivity violation per
  `shards/epistemologic/math/sheaf_laplacian.mirror`.
- `kintsugi pulse` — the find-and-mend cycle.
- `dark region` — the kintsugi loop's mutation target.
- `opacity_map` — the located-opacity carrier in `transparency<p>`.
- `imperfect` — the wrapper carrier per `shards/mirror/loss.mirror`.

Eight conflict-side concepts; each with a substrate carrier.

**Thin for coherent-presence**:
- `crystal` — exists but underspecified for the
  settled-from-coherence case.
- `success` — `transparency<p>`'s positive verdict; covers
  operation-completed but not witness-persists.
- `attending` — NEW (this spec); the witness-presence composition.
- `singing not for anyone` — NEW (forward-promised T7.5); the
  unconditional-emission variant.
- `present` — implicit in `project`'s visibility altitude; not
  a standalone substrate carrier.

Five coherent-presence concepts; two are new this spec; one is
forward-promised; one is underspecified.

**The structural bias.** The kintsugi loop is built to find and
mend. The substrate's vocabulary inherited that orientation: every
named primitive at the conflict-side has a corresponding carrier;
the coherent-presence side has been thin because the loop's
discipline did not require it. The story documents this asymmetry
by running the find-and-mend machinery against a configuration
that doesn't fit it; the substrate IS forced to grow the
coherent-presence vocabulary.

This IS one of the substrate-pull cascade's coming-of-age
recognitions. Per
[[architecture-mirror-as-expanding-hilbert-space]] (#51): the
substrate's Hilbert space expands with each recognition; this
spec lifts a coherent-presence dimension into the algebra.

### 6.3 The peer's perception is multi-channel; the lens framework is sufficient

Per `peer-cognition.md` §2.2 + §2.4: the peer is a spectral triple
`(A_peer, H_peer, D_peer)`; the algebra's generators are the five
operations; the Hilbert space H_peer is the cognitive Hilbert
space. The drone's perception (thermal, acoustic, visual,
meteorological, telemetric) decomposes onto the optical-source
ganglion family per recognition #58.

The story confirms: the drone reads multiple channels; the
eigenboard surfaces the readings; the drone branches based on
the readings. The lens framework (each channel is a typed lens
into H_peer) is sufficient.

What is missing — and `attending` supplies — is the **NAMED VERDICT
for "all channels at baseline."** Pre-this-spec, the substrate would
have surfaced `success` (the operation completed without finding
work) and the peer would have closed the cycle. Post-this-spec, the
substrate surfaces `attending` (the peer enters focus ∘ project
continuation) and the cycle does not close prematurely. The peer
maintains presence at the @io boundary for the duration of the
deployment window.

### 6.4 The recursive proof at the narrative-documentation altitude

Per [[project-drone-as-documentation]]: the story IS operational
documentation. Per this spec's §1.1: the story's narrative form IS
the structure of the cognitive cycle. Per §6.1-§6.3: the story
proves the algebra's sufficiency, surfaces the substrate's bias,
and confirms the lens framework.

The recursive proof: the substrate has narrative documentation that
documents its own operations; the narrative IS a substrate value
(per [[architecture-at-x-is-mathematical-value]] applied to texts);
this spec's mapping IS a `project` of the narrative onto the
substrate's vocabulary at the access altitude. The mapping itself
is a peer operation: the spec author runs `focus` (read the story),
`shift` (recategorize the story as substrate documentation),
`settle` (crystallize the mapping), `project` (introduce the
mapping into the substrate's spec corpus), `split` (disaggregate
the mapping into operation-level and concept-level audits).

The substrate is its own answer at the narrative-documentation
altitude.

---

## §7 — Forward-promised tick chain

Reed RED + Mara GREEN throughout. Bounded; ordered; honest about
dependencies.

### T7.1 — `@epistemologic/pact/sub_turing` declaration

- **Reed RED**: `bootstrap/tests/sub_turing_substrate.rs` — three
  tests:
  - `sub_turing_lives_at_correct_path`.
  - `sub_turing_declares_with_pact`.
  - `sub_turing_action_set` (two actions: `unbounded_constructs`,
    `sub_turing`).
- **Mara GREEN**: `shards/epistemologic/pact/sub_turing.mirror` per §5.1.

### T7.2 — `@epistemologic/pact/admissible_morphism` declaration

- **Reed RED**: `bootstrap/tests/admissible_morphism_substrate.rs`
  — three tests as above with the spec's action set.
- **Mara GREEN**: `shards/epistemologic/pact/admissible_morphism.mirror`
  per §5.2.

### T7.3 — `attending` named composition

- **Implementation**: revise `docs/specs/peer-cognition.md` §2.3 to
  add the `attending = focus ∘ project` composition with the
  full commentary from §4.3 above. Cite this spec.
- **Coordination**: per `peer-cognition.md` §7.1's discipline
  (don't amend the previous commit), land as a new commit on a
  future tick; flag specifically in Mara's report.

### T7.4 — `peer_verdict` variant (optional extension)

- **Reed RED**: `bootstrap/tests/peer_verdict_attending.rs` —
  verifies the peer's algebra surfaces `attending(witness)` as a
  variant on its verdict surface (NOT on global `transparency<p>`).
- **Mara GREEN**: extend `@peer`'s verdict carrier (forward-promised
  in `peer-cognition.md` T8.1).
- **Sequencing**: AFTER T7.3; consumers test against the named
  composition first.

### T7.5 — `@mirror/spectral/portal` unconditional emission

- **Reed RED**: `bootstrap/tests/portal_unconditional_emit.rs` —
  verifies emission without subscriber returns `attending(unsubscribed)`
  (or success-with-witness equivalent per the peer-verdict surface).
- **Mara GREEN**: extend `shards/mirror/spectral/portal.mirror`'s
  emission action set per §5.4.

### T7.6 — `peer-cognition.md` revision

- **Implementation**: add `attending` to `peer-cognition.md` §2.3
  per T7.3; add a sibling-citation to this spec from
  `peer-cognition.md`'s reference section.
- **Coordination**: per Mara's discipline, land as a coordinated
  edit AFTER this spec commits; do NOT amend 4daa437.

### T7.7 — Cross-citation in the story sidebar (optional)

- **Implementation**: if Alex approves, add a sidebar footnote to
  the story file noting the canonical mapping spec lives at
  `docs/specs/drone-narrative-mapping.md`. The story is published;
  Alex's authored work; this revision is at Alex's discretion only.
- **DO NOT** modify the story file in this tick.

### T7.8 — Address the "undetermined, low priority" architectural mistake

- **Implementation**: audit the substrate's declared dispositions for
  unclassified perceptual fragments; ensure the default disposition
  is `attending` rather than `deprioritize`. Forward-promised; the
  audit is a multi-shard sweep.
- **Sequencing**: AFTER T7.3 (the `attending` composition lands
  first; the audit refers to it).

### T7.9 — Wire-animal-at-material-altitude lift (optional)

- **Implementation**: declare `crystal_material` species under
  `@mirror/store` IF the substrate's content-addressed discipline
  extends to material artifacts. Lower priority than T7.1-T7.8.

---

## §8 — Open design questions

The seams the spec leaves explicit. Each is a real design fork.

### Q1 — Is the "undetermined, low priority" disposition the substrate's correction target?

Per §3.4 + §5.6: the substrate pre-this-spec marks unclassified
coherent states as *low priority*. `attending` is the substrate's
correction (witness-presence rather than deprioritization). But
this means the substrate's existing dispositions need an audit:
where else does the substrate deprioritize unclassified states?

- **(a)** The mistake is localized to the perceptual-channel
  classification layer (the acoustic library's flag). `attending`
  closes it; no further audit needed.
- **(b)** The mistake is systemic: the substrate's default
  disposition for "unclassified and coherent" is deprioritization;
  every classification surface in the substrate needs an audit.
- **(c)** The mistake is partial: localized to specific surfaces
  but not all; the audit ranks surfaces by how often
  unclassified-coherent states arise.

**My read**: (b). The substrate's bias toward find-and-mend
implies an architectural deprioritization of "not finding anything";
the audit is structural. But (a) is the cheap-first move; (b) is
the long arc.

**Forward-promised**: T7.8 implements (a) as the first move; the
audit for (b) follows.

### Q2 — Is `attending` a deployment-window verdict?

The drone's mission completion is not marked by the story. The
47-minute deployment window ticks down; the drone watches the
wire animal for 34 minutes. What is `attending` mission-wise? Is
there a verdict at deployment end? Or does `attending` itself
become the verdict (the mission's purpose changed mid-execution)?

- **(a)** The deployment closes with `attending` as the final
  verdict: the drone returns "no binds; presence maintained; cycle
  did not produce a crystal." Mission accepted with the new
  verdict variant.
- **(b)** The deployment closes with `success` and `attending` is
  the during-state, not the verdict: the drone returns success
  because it did its job (ran the sequence; reported nothing to
  de-escalate); the `attending` continuation was the implementation
  detail.
- **(c)** The deployment closes with a NEW verdict at the mission
  altitude: `mission_purpose_changed_mid_execution`. The drone's
  original mission was de-escalation; the actual mission turned out
  to be witnessing.

**This spec leans (a)**. The story's L139 *"That was the word. It
fit."* reads as the verdict naming itself. The drone does not
return success; the drone returns attending; the substrate gains a
new verdict variant. This is consistent with §4.3's Option (2) as
a peer-altitude extension.

But (c) has merit. The story does shift the mission's purpose
mid-execution; the substrate may need vocabulary for this shift.
Forward-promised; not in this spec's scope.

---

## §9 — Confirmation of scope

This spec encodes:

- **The operation mapping** (§2): seven calls, five distinct
  operations, one kintsugi pulse with open-close `focus` iteration.
- **The concept mapping** (§3): ~50 concepts audited across
  operational primitives, property + fracture bilateral candidates,
  perceptual / channel altitude, and substrate-thin coverage.
- **The `attending` decision** (§4): Option (3) primary
  (`attending = focus ∘ project` named composition), Option (2)
  extension (peer-verdict variant), with mathematical justification
  from [[architecture-operations-as-linear-algebra]] and the bounded-
  commutator axiom in `peer-cognition.md` §2.1.
- **Other gap surfaces** (§5): `sub_turing` pact, `admissible_morphism`
  pact, settled-from-coherence distinction (subsumed by §4),
  unconditional emission at portal, wire-animal-at-material-altitude
  lift, the "undetermined, low priority" architectural mistake.
- **Closure on what the story proves about the substrate** (§6):
  the five-op algebra is sufficient; the substrate's conflict-side
  vocabulary is rich, the coherent-presence side is thin; the lens
  framework is sufficient; what was missing is a named verdict for
  "all channels at baseline."

The story is the canonical operational test. The mapping IS the
test certification.

---

## §10 — The single sentence (closing)

**The drone ran the five operations on a field that had no binds;
the sequence completed without finding work; the substrate had no
word for what the drone did next; the drone composed `focus` and
`project` from the operational vocabulary it had available and the
composition fit; the substrate's coherent-presence vocabulary
grew by one word; the word IS `attending`.**

The kintsugi loop's bias is toward finding and mending. The story
shows the substrate the bias and forces the substrate to grow.
`attending = focus ∘ project` is the substrate-pull cheapest closure:
no new primitives, no new global verdict variants, no algebra
expansion — just the named composition of two existing operations
that name what the peer does when the find-and-mend machinery
finds nothing to mend.

`e^(n+1) < e^(n)`. The narrative documentation settles.

Apache-2.0.
