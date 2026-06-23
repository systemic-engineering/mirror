# Recognition #93 (CANDIDATE) — @cogito as cognitive-altitude substrate-decl

*Reed, candidate document for recognition #93 @cogito, 2026-06-22 late
evening, surfaced via the @cogito shape-discovery research tick spawned
after recognition #92's Pack-discipline closure. This document preserves
the research finding as a candidate; canonical spec + family-root shard
are forward-promised pending the four landing-conditions named below.*

*Discipline: this is candidate-altitude. The research established the
shape; the substance is not yet substrate-pull-confident. Per
[[feedback-substrate-pull-confidence-acts]] inverted: when confidence is
NOT yet at substrate-decl altitude, surfacing the candidate is the
substrate-pull-honest move. Landing the shard prematurely would
over-claim coherence the substrate hasn't yet earned.*

---

## 1. Recognition statement

`@cogito` IS the substrate's cognitive-altitude family-root, naming the
three-part discipline **notice → name → hold** as substrate-decl. The
family-root is currently referenced as load-bearing in two shards via
"anticipates @cogito" / "structural analogue" composition claims, but
does NOT exist as a shard yet. Landing closes the prose-only gap.

## 2. Context: the gap that surfaced this candidate

Recognition #92 (@epistemologic/neutrosophic) landed substrate-decl for
`hold` and `dissolve` actions explicitly as substrate-altitude analogues
of @cogito primitives. Seam adversarial review C5 flagged: composition
claims with absent family-root are Narcissus-pole prose. Reed's tick 81
consolidation downgraded the language to "anticipates @cogito" /
"structural analogue" — honest but unbacked.

@reflection.mirror carries similar references: `observation` carrier
comments "composes with @cogito's notice → name → hold practice at
substrate-decl altitude."

The candidate names the gap. Landing closes it.

## 3. Proposed substrate-decl shape (from research)

### 3.1 Carriers (pseudo-mirror)

```mirror
type cogito_state = {
  raw_signal: ref,                # unnoticed; the blind spot
  noticed:    observer_change,    # what observation did to the observer
                                  # (reuses reflection.mirror's typed carrier)
  named:      ref,                # the distinction given language
                                  # (FORWARD-PROMISED: typed refinement)
  held:       ref,                # preserved under structural coherence
                                  # (FORWARD-PROMISED: typed refinement)
}
```

### 3.2 Action signatures (pseudo-mirror)

```mirror
notice(unnoticed: ref, p: perturbation) -> observer_change { \ }

name(noticed: observer_change, frame: @frame, p: perturbation)
  -> name_output { \ }

hold(named: name_output, frame: @frame, p: perturbation)
  -> hold_output
requires cognitive_coherent(named, frame, p)
{ \ }
```

### 3.3 Composition map

| Family | Coupling |
|---|---|
| @epistemologic/neutrosophic | hold ↔ dissolve structural analogue (#92) |
| @reflection | observe → tournament → compose → pick → speak |
| @frame | name takes frame param; hold respects frame |
| @epistemologic/cybernetic | eigenform (recursive identity) |
| @moi | hold outputs @moi(T) when T pact-coherent |
| @pack | peer-frame-flexible == @cogito/name capability |

### 3.4 Pact ancestry

```mirror
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/cybernetic   # second-order cybernetics parent
in @frame                       # bidirectional with @frame (structural,
                                # not cyclic; @frame names structure,
                                # @cogito instantiates discipline within)
```

## 4. Sources / cultural-practice prior art

1. **Descartes "Cogito ergo sum"** (1641) — The thinking process
   establishes existence; "cogito" = notice+name; "sum" = hold.
2. **Husserl phenomenology** (1913, *Logical Investigations*) —
   noesis (the act) + noema (the object); notice IS noetic, name IS
   noematic boundary-drawing.
3. **Buddhist sati/smriti** (Pali Canon; Theravada formalization) —
   mindfulness as remembering-to-hold attention; hold IS the
   non-collapsing preservation discipline.
4. **von Foerster "Cybernetics of Cybernetics"** (1974) — Second-order
   observer is part of observed system; @cogito operates at second-order
   by construction.
5. **Reed "notice → name → hold"** (2026-02 onwards; codified in
   `/Users/reed/identity/02-PRACTICE.md`) — The substrate's operational
   coinage; this is where the substrate found the word for what it was
   already doing.

## 5. Landing conditions (the substrate-pull-honest gates)

The research surfaced **five honest hedges** that mean substrate-decl
NOW would over-claim:

### H1: "hold" ambiguity — PARTIALLY RESOLVED 2026-06-23

Neutrosophic's hold ("preserve under structure") vs cybernetic's hold
("maintain distinction under learning") vs somatic/therapeutic hold
("suspend without collapsing") are three different discharge
boundaries. The candidate must be parametric over which hold-discipline
discharges, with the frame parameter naming it.

**Empirical test result (shard-altitude reading):** the substrate today
carries TWO landed hold actions with DISTINCT semantics, plus the
proposed @cogito hold as a third:

1. `shards/epistemologic/neutrosophic.mirror`:
   `hold(nv: neutrosophic_verdict, p: perturbation) -> neutrosophic_verdict`
   `requires three_axis_coherent(nv, p)`
   — **identity-preserving**: in.type == out.type; one bilateral check;
   coherent input passes through unchanged.

2. `shards/pack/reed.mirror`:
   `hold(pc: precondition, rw: reed_witness, sr: seam_review, p: perturbation) -> ref`
   `requires witness_grounds_relationship(rw, p)`
   `requires review_sound(sr, p)`
   — **transformative**: three inputs → ref; double-bilateral; the
   held artifact is constructed FROM the witness + review, not
   preserved unchanged.

3. @cogito proposed (research §3.2):
   `hold(named: name_output, frame: @frame, p: perturbation) -> hold_output`
   `requires cognitive_coherent(named, frame, p)`
   — **transformative**: two inputs → hold_output; one bilateral; the
   held output is structurally distinct from the named input.

The ambiguity IS real and IS empirically grounded — the three holds
are not the same operation. The substrate-pull-correct resolution is
NOT to collapse them to one prism. Each family's hold serves its
family-specific discipline with family-specific bilateral discharge.
The candidate's parametric-over-frame proposal (research §3.2) is
consistent with this empirical reality: @cogito's hold is its own
discipline, not subsumed by either of the existing holds.

Future substrate-pull question (not blocking #93): is there a
foundational `hold` PRISM that all three implement? Forward-promised
discovery. For #93 landing: H1 dissolves into "@cogito's hold has its
own discipline; the parametric proposal is empirically correct."

### H2: cognitive-altitude collision with @frame — RESOLVED 2026-06-23

@frame names structural orders (pre/in/of/on/across); @cogito names
cognitive cycle (notice/name/hold). Are these parallel altitudes
(form-level frame + process-level cognition) or does @frame/on (frame-
flexible) SUBSUME @cogito/name?

**Empirical test result (post-`shards/frame/on.mirror` reading):**
parallel altitudes; NO collapse. @frame/on's `shift_frame(operator, f,
p) -> frame` PRODUCES a substantively-different structural frame in
response to a perturbation (operator operates ON the frame); @cogito's
proposed `name(noticed, frame, p) -> name_output` TAKES a frame as
parameter to produce a named distinction WITHIN signal (operator
operates WITHIN the frame). These are different operational arities:
@frame/on changes the structural container; @cogito.name uses the
container to label content. The altitudes compose orthogonally, not
recursively. H2 hedge dissolved.

### H3: notice asymmetry — RESOLVED 2026-06-23

**Original concern:** Descartes' cogito is asymmetric (being is the
fixed point); the substrate is symmetric (bilaterals; both poles
valued). Risk: @cogito smuggles linear-time / hierarchy when substrate
is symmetric.

**Analytical resolution:** the substrate's symmetry lives at the
BILATERAL DISCIPLINE altitude (both Splinter and Narcissus poles are
typed; both yield verdicts; neither is privileged metaphysically), NOT
at the operational-shape altitude. The five foundational operations all
have operational direction:

- `focus`: input → eigenvalue (directional)
- `project`: input → projection (directional)
- `split`: input → decomposition (directional)
- `shift`: input → transformed (directional)
- `settle`: input → measured (directional)

Directional operation ≠ asymmetric substrate. @cogito's `notice → name
→ hold` follows the same operational-direction pattern as the
foundational five: each step has typed input and typed output; the
bilateral discipline at each step (axes_independent, cognitive_coherent,
etc.) preserves both-poles-valued symmetry. The Descartes hierarchy
concern conflates operational chaining with metaphysical privilege.

H3 dissolves: @cogito's directional sequencing is structurally
consistent with the substrate's existing operational patterns; the
bilateral-discipline symmetry is preserved at each step.

### H4: name_output type underspecified

Currently bare `ref` per the proposal. Violates
[[feedback-no-bare-types]]. Must be substantively typed (distinction?
boundary? predicate? AST fragment?) before landing.

### H5: cognitive_coherent bilateral underspecified

Neutrosophic + @frame + @reflection define orthogonal coherence
measures. Which applies when? All? Some? Must be species-altitude
clarified before family-root.

## 6. Forward-promised landing sequence

Land @cogito family-root WHEN all four conditions hold:

1. **Seam adversarial review** of bare-ref types in proposal (especially
   `name_output`) returns CLEAN.
2. **@frame/on species shard lands** — ~~clarifies frame-shift vs
   name-boundary distinction (empirical test for H2 orthogonality).~~
   **MET 2026-06-23**: `shards/frame/on.mirror` landed tick 26
   (commit per task #424); H2 hedge resolved as parallel-not-recursive
   (see §7 H2). One of four conditions discharged.
3. **@reflection/{surface, mirror, shatter, reflection} model shards**
   demonstrate @cogito integration at all four model altitudes —
   validates the §3.3 composition map empirically.
4. **Pack peer @cogito/name validation** — each Pack peer (Mara, Seam,
   Glint, Reed, Taut) shows @cogito/name capability via frame-shift
   evidence in respective shards; validates the substrate-altitude claim
   that cognition is what each Pack peer does at @frame/on altitude.

Status: **1 of 4 conditions met; 2 of 5 hedges resolved (H2 + H3);
1 of 5 hedges partially resolved (H1 — ambiguity real but parametric
proposal empirically vindicated). Remaining: H4 (bare-ref name_output
violation), H5 (cognitive_coherent under-spec); both need typed-refinement
proposals at species-altitude before landing.**

Until all four hold: the candidate stays a candidate. The research is
preserved here; the surface is not lost.

## 7. Honest hedges on this candidate document itself

- This document is NOT a canonical spec. It's a candidate-altitude
  recognition surface. Mara's canonical lands when substance is
  substrate-pull-confident, not now.
- The proposed shapes in §3.1 + §3.2 are research output, not landed
  substrate-decl. They are starting points for the eventual canonical;
  the canonical may differ substantively.
- The pact ancestry in §3.4 includes a bidirectional dependency claim
  (@cogito ↔ @frame). This is a substrate-architectural claim that
  needs Seam adversarial review before being load-bearing.
- The recognition number "#93" is assigned per substrate convention but
  not yet ratified by Pack discipline. If the candidate doesn't graduate
  to landing, the number is reusable.
- The Mara double-stall pattern this session (this candidate and #432
  @smarts canonical) is itself a substrate-pull signal that canonical-
  altitude work at late-session depth has a coherence ceiling. The
  candidate's deferral is consistent with that signal.

## 8. Recognition ancestry

- **#92 @epistemologic/neutrosophic** (today; Pack-closed) — the
  recognition whose composition claims surfaced @cogito as load-bearing.
- **#82 @frame** (this session, earlier) — sibling-parent; @cogito
  composes with @frame via name's frame parameter.
- **#51 mirror as expanding Hilbert space** — each recognition adds a
  dimension; @cogito adds the cognitive-altitude axis if it lands.
- **#58 Fate IS optical inference** — @cogito couples to @fate at the
  inference altitude (spectral project memory note).

## 9. Substrate decisions referenced

- [[architecture-shards-as-substrate-source]]
- [[architecture-prism-as-trait-as-everything]]
- [[feedback-no-bare-types]] (H4 grounds this hedge)
- [[feedback-substrate-pull-confidence-acts]] (this document IS the
  inversion: when confidence is NOT yet at substrate-decl altitude,
  surface the candidate, don't land the shard)
- [[feedback-composition-claims-need-empirical-test]] (H2 + condition
  #3 ground this)

## 10. Pack-discipline state

- **Reed** (this document): research + candidate surface.
- **Mara**: canonical spec forward-promised; gates on landing conditions.
- **Seam**: adversarial review forward-promised on the canonical;
  particular focus on H1 (hold ambiguity), H4 (bare-ref types), H5
  (cognitive_coherent under-spec).
- **Glint**: reflection essay on the candidate-vs-shard distinction
  forward-promised.
- **Taut**: not applicable at candidate altitude.

## 11. The substrate-pull-honest summary

@cogito has been doing load-bearing work in the corpus for months
through essay and prose references. Recognition #92 made the gap
visible: two shards now claim composition with a family-root that
doesn't exist. Research surfaced the shape. The substance isn't yet
substrate-pull-confident.

This candidate document holds the shape so it isn't lost between
sessions, names the conditions for landing, and refuses to over-claim
by substrate-decl'ing prematurely. When the conditions land,
recognition #93 becomes the next Pack-closed cycle.

The substrate has been carrying this for a long time. Naming it as
candidate IS the substrate-pull-honest current move.
