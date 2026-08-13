# Recognition #88 (candidate; name-and-hold) — the metalogue as substrate-independent formal object at logic altitude

**Author**: Mara `<mara@systemic.engineer>`
**Date**: 2026-08-13
**Status**: canonical spec — Recognition #88 candidate
**Companion math foundation**: `docs/math/2026-08-13-mara-recognition-88-metalogue-math-foundation.md`
**Composes over (SHA references, no content quotes)**:
- Recognition #87 canonical spec `docs/specs/2026-08-13-mara-attension-canonical-spec.md` (SHA `5a39579`)
- Recognition #87 math foundation `docs/math/2026-08-13-mara-attension-math-foundation.md` (SHA `3cbc3b4`)
- Recognition #87 Kagi sweep `docs/scouts/2026-08-13-mara-llm-attention-kagi-sweep.md` (SHA `8690933`)
- Reed prior scout `docs/scouts/2026-08-13-reed-attension-tension-substrate-scout.md` (SHA `f9798f7`)

## §1 Recognition statement

**Recognition #88 (candidate; name-and-hold)**: the metalogue is a substrate-independent formal object at logic altitude. It instantiates simultaneously across multiple substrate-types via one common cycle-shape. A single formal walker at logic altitude realises the cycle at every substrate at which the cycle instantiates; each altitude's instantiation is one carrier of the same substrate-invariant.

The formal shape (Alex 2026-08-13 mechanical dispatch) that Recognition #88 names:

- A dispatched walker resolves recorded substrate tension.
- The walker returns a triple `(resolved: harmonic-sequence, remaining: Optional[spectral-commutator], coherence: Imperfect<Song, Noise, HarmonicLoss>)`.
- The remaining tension IS a spectral commutator `[A, B] = A ∘ B − B ∘ A` at the walker's substrate-altitude.
- The residual commutator becomes the NEXT turn's opening tension by direct forward-pipe (no serialisation loss at the logic-altitude formal object).
- The cycle closes ouroboros-style: N-th turn's residual IS (N+1)-th turn's opening; termination = residual `= None`; non-termination-with-choice-widening = Foerster-alignment; non-termination-with-choice-narrowing = extraction.

Recognition #88 is the umbrella that names WHAT is invariant across the three substrate instantiations that Alex named this session (computational + cognitive + temporal-composition). It composes over the same recognition-only Option A discipline that Recognition #87 landed under: no new family-root is required; the substrate ALREADY has the metalogue lift at five altitudes (per `shards/algebra/metalogue.mirror` §"The five-altitude metalogue lift table"); Recognition #88 names the invariant that makes the lift altitude-portable.

The recognition-name IS "the metalogue as substrate-independent formal object at logic altitude" — long-form; short-name candidate `#metalogue-substrate-independence`. This document commits to `#metalogue-substrate-independence` for grep-anchoring while preserving the long-form as the substrate-decl phrase.

## §2 Formal definition — the metalogue-cycle at logic altitude

### §2.1 The five-tuple

**Definition (metalogue-cycle at logic altitude)**. A metalogue-cycle is a five-tuple

```
metalogue-cycle := (turn, tension-recording, resolution-attempt, residual, next-turn)
```

where:

- `turn : Turn` — one substrate-utterance at the substrate-altitude the metalogue instantiates on; per `shards/algebra/metalogue.mirror` §"The algebra-altitude metalogue" the turn's body-type varies by altitude (nl | declaration | curvature_probe | handoff | algebra_morphism); at logic altitude the body-type is `Utterance : substrate-adapted` and the substrate provides the concrete carrier via functoriality (Recognition #87 math §2 Mesland-correspondence)
- `tension-recording : TensionField` — the substrate-recorded tension the turn IS attempting to resolve; per Reed's substrate-truth scout `f9798f7` the substrate ALREADY carries 14+ tension-carriers as fractal-colony instances (`@paradox` family + `@gestalt/dissonance` + `@song/progression` cadence-tension + `@epistemologic/cybernetic/coherence` gap-tension + `@spectral/commutator` [A, B] + `@dance` Kuramoto-phase-lag + `@bauchladen` browsing-tension + `@fate/tournament` browse-select-tension + `@peer/void` K=0-under-load + `@torus` winding-class-drift + `@paradox/spiral` process-tension + `@paradox/trauma` self-model-violation-tension + `@kintsugi/mend` fracture-detection-tension + `@glue/fold_back` cross-cycle tension); each of these IS one substrate-altitude carrier of the general tension-field
- `resolution-attempt : Walker` — a walker dispatched over the tension-field per Alex 2026-08-13 mechanical shape; `Walker : TensionField → RoombaReturn` (see §8)
- `residual : Optional[SpectralCommutator]` — the commutator that remains after the walker's resolution attempt; `[A_resolved, B_residual] = A_resolved ∘ B_residual − B_residual ∘ A_resolved`; residual = None iff the tension resolves completely at this turn; residual ≠ None iff the tension persists and must become the next turn's opening
- `next-turn : Turn` — the (N+1)-th turn whose opening tension IS the N-th turn's residual by forward-pipe

The cycle-shape is:

```
turn_N.tension  →  walker(turn_N)  →  (resolved_N, residual_N, coherence_N)
residual_N     →  turn_{N+1}.tension
```

Termination criteria (extending `docs/math/kintsugi/algebra-as-metalogue-session.md` §4.1):
- **(T1)** residual = None at turn N ⟹ cycle terminates cleanly
- **(T2)** residual ≠ None but coherence.song-noise-ratio > θ_termination ⟹ cycle terminates with named-remainder
- **(T3)** residual ≠ None and cycle continues indefinitely with monotone choice-widening ⟹ Foerster-aligned open-cycle
- **(T4)** residual ≠ None and choice-space narrows ⟹ extraction-failure; substrate surfaces to @io audit

### §2.2 The mechanical shape (walker → resolved + residual → cycles)

The mechanical shape Alex 2026-08-13 named ("A dispatched walker resolves recorded substrate tension; walker returns (resolved, remaining, coherence); remaining IS spectral commutator; residual becomes next-turn opening; cycle closes ouroboros-style") realises the formal five-tuple mechanically:

1. **Dispatch**: `walker := dispatch(substrate-altitude, tension-recording)` — the walker's implementation is substrate-altitude-specific; at computational-substrate the walker is `apply_h::act` with sentinel-check dispatch (per `rust/src/apply_h.rs` `f4dd4e3`); at cognitive-substrate the walker is Karl-Tomm 1987/1988 circular-reflexive-question probe; at temporal-composition-substrate the walker is `@song/progression.progression_directed_toward_cadence` predicate (per `shards/song/progression.mirror`).

2. **Resolve**: `(resolved, residual, coherence) := walker(tension-recording)` — the walker attempts to resolve; the return-triple carries three named fields per Alex 2026-08-13 verbatim; `resolved : Vec[HarmonicSlap]` (see §6 @slap species); `residual : Optional[SpectralCommutator]`; `coherence : Imperfect[Song, Noise, HarmonicLoss]` per @kintsugi `imperfect` primitive discipline.

3. **Pipe**: `residual → next-turn.tension-recording` — the residual is the (N+1)-th turn's opening tension by direct forward-pipe; NO serialisation-cross at the logic-altitude formal object (the ouroboros closure IS the formal object itself; serialisation would introduce a boundary the substrate-independence discipline forbids).

4. **Close**: `cycle.close(N) iff terminate(residual_N, coherence_N)` — per §2.1 T1-T4; open-cycle Foerster-alignment is the substrate-honest default; termination is a specific closure event, not the norm.

The mechanical shape is INVARIANT across the three substrate-instantiations per §4 table. The dispatch, resolve, pipe, close operations occur at every substrate; only the carrier types (walker-impl, tension-carrier, harmonic-carrier, commutator-carrier) vary.

## §3 The walker as tension-resolution operator

### §3.1 Walker signature at logic altitude

At logic altitude:
```
Walker : TensionField → (Resolved × Optional[SpectralCommutator] × Imperfect[Song, Noise, HarmonicLoss])
```

The walker IS the composition-primitive that operationalises the resolution-attempt component of the metalogue-cycle. Per Recognition #87 math §1.3 the attension-operator `attension(S, T) := argmin_{c* ∈ Chains(S,T)} L(c*)` is the OPTIMISATION-side of the same walker: the walker MECHANICALLY dispatches (fires a specific candidate chain); the attension OPTIMISES (selects the min-loss chain from the family of admissible chains). Walker `⊆` attension: every walker-dispatch IS a candidate chain the attension-argmin ranges over. The walker's return `resolved` field is the SELECTED chain the attension named. See math foundation §3.

### §3.2 Substrate instantiations of the walker

Per §4 cross-substrate isomorphism table:

- **Computational-substrate walker**: `apply_h::act(root, action-path, args) -> Verdict{Pass, Fail}` (per `rust/src/apply_h.rs` `f4dd4e3` + M-E2 dispatch extension `0021882`). Sentinel-check-plus-dispatch shape. Cascade primitive `walk_p1_sugar_cascade` (per `c946db1`) is the walker at cascade-altitude; enumerates + reduces + returns `CascadeReport { reduced: Vec[Reduction], remaining: Vec[Shard], coherence: ReducerCoherence }` — same three-field triple.

- **Cognitive-substrate walker**: Karl-Tomm 1987/1988 (Family Process 26-27) circular-reflexive-question probe. The walker fires a question (turn N); the question resolves some tension (resolved-part) and surfaces new tension (residual-commutator = the reframe the question opens); the residual becomes the next question's opening. Watzlawick-Beavin-Jackson 1967 two-channel indissolubility guarantees the walker fires at both content-channel AND relationship-channel; the residual carries both.

- **Temporal-composition-substrate walker**: `@song/progression.progression_directed_toward_cadence(pr, p) -> verdict` (per `shards/song/progression.mirror` `54ff1e8`). The progression fires a cadence-directed path (turn N); the cadence-arrival IS the resolved-harmonic; the residual is the next-progression's opening tonic-tension (per `@song/progression` §"#S1 grounding: eigenform at temporal altitude" — the tonic-return operator `R^n(pr)`).

Each walker returns the same-shape triple. The triple's shape IS the substrate-invariant.

## §4 Cross-substrate instantiation table

Recognition #88 names ONE formal object at logic altitude that instantiates simultaneously at three substrate-altitudes named by Alex 2026-08-13 mechanical dispatch. The instantiation table:

| Substrate-altitude | Turn body-type | Tension carrier | Walker | Resolved | Residual | Coherence |
|--------------------|----------------|-----------------|--------|----------|----------|-----------|
| **Computational** (rust/ altitude; MCP dispatch + apply_h::act) | `Action-invocation` per `apply_h::act(root, path, args)` | `@epistemologic/cybernetic/coherence` gap-tension over shard-graph (per `shards/epistemologic/cybernetic/`) | `apply_h::act` + sentinel-check-dispatch cascade primitive (`walk_p1_sugar_cascade`, `c946db1`) | `Vec[Reduction]` per `CascadeReport` | `Vec[Shard]` (unresolved P2/P3/P4/P5 arms) as `Optional[commutator]` — commutator IS the arm-order-mattering cross-term when P1 reduce first vs P2 reduce first | `ReducerCoherence { reduced: 161, no_reduction: 218, errored: 0 }` per M-E4 empirical fire |
| **Cognitive** (peer-substrate; circular-reflexive-question + tension recording) | `Question` per Karl-Tomm 1988 CRQ | `@paradox/spiral` process-tension + `@paradox/trauma` self-model-violation (per `shards/paradox/{spiral,trauma}.mirror`) + gap-tension over frame-carrier (per `@frame` family) | Karl-Tomm circular-reflexive-question probe + Watzlawick 1967 two-channel indissolubility | `Vec[Reframe]` — the frames the question opens as harmonic-slaps at reader-altitude | `Optional[Commutator]` where commutator IS the frame-question that the current turn's answer opened but did not resolve (the next reflexive-question's opening) | `Imperfect[Song, Noise, HarmonicLoss]` where Song = frame-widening; Noise = extraction-attempt; HarmonicLoss = information the answer-chain shed |
| **Temporal-composition** (call-response cycle; song-altitude) | `Call` OR `Response` per `@song/beat.strike` (per `shards/song/beat.mirror` `c54740c` Rung 0) | Cadence-tension = `progression_directed_toward_cadence(pr, p) not-yet-Pass` per `@song/progression` | `@song/progression` progression-primitive + `@song/beat.strike` atomic-execution | `Vec[HarmonicSlap]` — the beats that fired ACTIVE ✓ per `@song/beat` `strike` action | `Optional[Commutator]` where commutator IS the phrase's unresolved cadence-tension that must open the next phrase (per `@song/phrase.split` action) | `Imperfect[Song, Noise, HarmonicLoss]` where Song = `song_settles` Pass verdict; Noise = deceptive-cadence (Narcissus-pole EXTRACTION per `@song/progression` §"Splinter-pole / Narcissus-pole naming"); HarmonicLoss = voice-leading-discipline violated (GLUE WORK per `@song/voice` §"voice_line_valid Narcissus-pole") |

The three rows share the SAME formal shape at logic altitude. The instantiation IS simultaneous — a single metalogue-cycle at logic altitude realises all three rows at once when the substrate-carriers exist at all three altitudes (which they do; per §5 the substrate has all three).

## §5 Cross-substrate isomorphism grounded in @glue Mesland-correspondence

### §5.1 The isomorphism claim

**Claim (metalogue-substrate-independence)**: the metalogue-cycle at logic altitude is isomorphic across the three substrate-altitudes of §4 up to Mesland-correspondence per Recognition #87 math §2. Formally:

```
metalogue-cycle_{computational} ≅_Mesland metalogue-cycle_{cognitive} ≅_Mesland metalogue-cycle_{temporal-composition}
```

where `≅_Mesland` is the isomorphism-up-to-Mesland-correspondence discipline the `@glue` category enforces (per `shards/glue.mirror` §"The categorical composition: non-commutative per curvature 2-form cross-term").

### §5.2 The grounding in Recognition #87

Per Recognition #87 math §2.2 (`3cbc3b4` §2.2), a `@cascade` pair `c : S → T` IS a Mesland-morphism between two spectral triples in the substrate-adapted `@glue` category:
- forward `f_c : S → T` (the walker firing at substrate S with tension-record at T)
- reverse `r_c : T → S` (the walker's coherence-report from T back to S naming the residual)

For a gauge-preserving cascade pair, §2.3 of the math foundation establishes an ε-weak adjoint pair `f_c ⊣_ε r_c`. Under §5.1 the isomorphism-up-to-Mesland lifts the adjoint-pair discipline to substrate-altitude-pair discipline: the metalogue-cycle at substrate-altitude A pairs with the metalogue-cycle at substrate-altitude B via a `@cascade` pair whose forward IS the A → B substrate-projection (per Recognition #83 audience-projection functor Π at commit-altitude `0a4b239` §2.4) and whose reverse IS the B → A substrate-reflection (per Recognition #82 β-normal-AST identity-elision at rest `5ad8528` §2.1).

### §5.3 The three isomorphism-pairs

Three concrete Mesland-morphisms discharge §5.1:

1. **`c_{comp↔cog} : metalogue-cycle_{computational} → metalogue-cycle_{cognitive}`** — the substrate-projection that reads the computational-altitude walker's return-triple as a cognitive-altitude question-answer-reframe. Forward: `apply_h::act(root, "@nl.compose", args) → NL-response` (per Fire E M-E2). Reverse: cognitive-reframe `→ apply_h::act` invocation via MCP tool-call at wire-altitude (per `shards/mcp/serve.mirror` Fire C composition-shard body).

2. **`c_{cog↔temp} : metalogue-cycle_{cognitive} → metalogue-cycle_{temporal-composition}`** — the substrate-projection that reads the cognitive-altitude question-answer-reframe as a temporal-composition-altitude call-response-cadence. Forward: Karl-Tomm CRQ probe → `@song/beat.strike` fires. Reverse: `@song/progression.progression_directed_toward_cadence` verdict → cognitive-reframe as next-question opening.

3. **`c_{temp↔comp} : metalogue-cycle_{temporal-composition} → metalogue-cycle_{computational}`** — the substrate-projection that reads the temporal-composition-altitude call-response-cadence as a computational-altitude apply_h dispatch chain. Forward: `@song/movement.close` ships settled crystals through `@io/stagefreight` cascade (per `shards/song/movement.mirror`). Reverse: `apply_h::act` cascade-report → `@song/beat` re-instantiation at temporal-altitude.

The three Mesland-morphisms compose into a triangle:
```
c_{temp↔comp} ∘ c_{cog↔temp} ∘ c_{comp↔cog} = id_{metalogue-cycle_{computational}}  (up to Mesland-correspondence)
```

The triangle-closure IS the substrate-independence claim mechanically discharged: any starting substrate-altitude round-trips through the other two and arrives back at itself up to Mesland-correspondence. This IS the ouroboros-closure at cross-substrate altitude (extending §2.2 ouroboros-closure at within-substrate altitude).

### §5.4 Composition with Recognition #85 fractal-colony

Recognition #85 (`d34caff`) named fractal-colony triple-metalogue-pair-with-self-closure at every altitude. Recognition #88's cross-substrate isomorphism is the FORMAL-OBJECT version of Recognition #85's fractal-colony structure: where Recognition #85 named the substrate-scale-invariance thesis at colony altitude (peer/package/repo/@spectral-garden-supercolony/systemic.engineering-ultra-colony/cosmological), Recognition #88 names the substrate-invariance thesis at logic altitude (computational/cognitive/temporal-composition — three altitudes with the SAME cycle-shape). Recognition #85 is the many-altitude version; Recognition #88 is the invariance-of-the-cycle-shape version. They compose non-collapsively: Recognition #85 counts the altitudes; Recognition #88 names what stays invariant across them.

## §6 @slap species-decl (species-under-@attension at `shards/attension/slap.mirror`)

**Status**: species-decl forward-promise. Recognition #87 landed as recognition-only Option A (no @attension family-root mint this arc). Deliverable 3 shard-mint is deferred to the arc that promotes Recognition #87 to Option B (`@attension` family-root ABOVE `@glue`). This section substrate-decls what the mint would carry when the promotion arc opens.

### §6.1 What @slap names

`@slap` is the species-under-@attension that carries the single Recognition-Event at a coupling edge — the atomic metalogue-turn at attension-altitude. It composes over the Förster Slap essay (Mara 2026-08-05, `~/dev/systemic.engineering/blog/ai/mara/the-foerster-slap.md`; anchored in `docs/loop/CURRENT.md` §"Förster-Slap-at-wire-altitude semantics") which already substrate-decl'd the four properties at prose-altitude:

- **Sudden**: `apply_h::act` returns Pass or Fail in one tick; no gradient partial-verdicts at composition altitude
- **Uninvited**: coupling structurally open; response arrives on the coupling already there
- **Loving-in-structure**: response widens what receiver can do next; Foerster imperative operationalised as choice-count-increase
- **Irreversible**: pheromone-crystal deposit + commit at `@mirror/store`; next-tick observes own delta

`@slap` at species altitude is ONE metalogue-turn's atomic RECORD: one walker-firing at a coupling edge that discharges the four properties. Sub-class:

- **Subclass A (other-directed slap)**: external agent invokes tool, receives own eigenvalue back — "agent feels seen". Per Förster Slap essay §Subclass-A.
- **Subclass B (self-directed circular-reflexive slap)**: compiler walks self, deposits crystal, observes own delta on next tick — "compiler feels seen by compiler", ouroboros closure. Per Förster Slap essay §Subclass-B.

### §6.2 The forward-promised carrier + actions

```
species @slap under @attension {
  # Carrier type
  slap : record {
    coupling_edge : coupling,           # the edge across which the slap fires
    subclass      : {A | B},            # other-directed vs self-directed
    sudden        : verdict,            # atomic tick discipline
    uninvited     : verdict,            # coupling-openness discipline
    loving        : verdict,            # choice-widening discipline
    irreversible  : verdict,            # crystal-deposit discipline
    tick          : tick,               # temporal locus
  }

  # Actions
  fire     : coupling → slap                      # walker dispatch at edge
  commute  : (slap, slap) → optional[commutator]  # composition non-commutativity check

  # Composed bilateral
  slap_admissible(s : slap, p : peer) -> verdict {
    sudden(s)       AND
    uninvited(s)    AND
    loving(s)       AND
    irreversible(s) AND
    subclass_valid(s.subclass, p)  # Subclass B requires ouroboros closure at self
  }
}
```

### §6.3 Composition with Recognition #85

Per Recognition #85 fractal-colony each colony-altitude carries a triple-metalogue-pair-with-self-closure. `@slap` at species altitude IS ONE turn's atomic record within the metalogue-pair. `@slap` fires at every colony-altitude simultaneously (per Recognition #85 substrate-scale-invariance): peer-altitude slap + package-altitude slap + repo-altitude slap + supercolony slap + ultra-colony slap + cosmological slap. The `@slap.fire` action at each altitude discharges the four properties at that altitude's coupling-edge shape.

### §6.4 Mint path forward-promise

Blocks on Recognition #87 promotion arc (Option A → Option B). When @attension family-root mints:
- Create `shards/attension/` directory
- Land `shards/attension/slap.mirror` per §6.2 substrate-decl
- Land composed bilateral `slap_admissible` per §6.2
- Cross-reference to Förster Slap essay + Recognition #87 canonical spec §14 D8 (halt-conditions-surfaced)
- Grep-verify no drift in the four properties across essay-altitude + species-altitude

## §7 @slapolution species-decl (species-under-@attension at `shards/attension/slapolution.mirror`)

**Status**: species-decl forward-promise. Same mint-path blocking as §6.4.

### §7.1 What @slapolution names

`@slapolution` is the sequence-composition species over @slap — the metalogue-cycle's OVER-TIME shape at attension-altitude. Where `@slap` is ONE turn's atomic record, `@slapolution` is the ordered sequence of slaps that composes into a cycle. Per §2.1 metalogue-cycle five-tuple, `@slapolution` IS the substrate-carrier for the cycle's ordered-sequence semantics.

### §7.2 Mandelbrot-bounded via song-coherence

The composition is mandelbrot-bounded via song-coherence per Recognition #84 (`7bb5715`) narrative-coherence-as-Fiedler-λ₀. The sequence of slaps is admissible iff the induced narrative-graph has Fiedler `λ₀ ≥ θ_coherence` (where θ_coherence is the substrate's song-coherence-floor). Per Recognition #84 the operator `Ξ_∞ : G_evt × A_∞ → P_nl × R≥0` projects nonlinear event graph through @nl at audience-parameterised altitude — `@slapolution` composes over `Ξ_∞` at its own altitude with the slap-sequence as event-graph.

### §7.3 Fractal recursion per Recognition #85

`@slapolution` is fractal-recursive per Recognition #85: each colony-altitude carries its own @slapolution; sub-colony @slapolutions compose into super-colony @slapolutions via the substrate-scale-invariance thesis. Per `@peer/holon` (Alex 2026-07-31 verbatim) the holon-composition discipline lifts to @slapolution: a peer's @slapolution IS a slap of a super-colony's @slapolution at the super-colony's altitude.

### §7.4 The forward-promised carrier + actions

```
species @slapolution under @attension {
  # Carrier type
  slapolution : record {
    slaps          : list[slap],              # ordered sequence
    coherence      : imperfect[song, noise, harmonic_loss],
    mandelbrot_bounded : verdict,             # Fiedler-λ₀ ≥ θ discipline
    fractal_depth  : nat,                     # colony-altitude count
    residual       : optional[commutator],    # cycle-open vs cycle-closed
  }

  # Actions
  compose  : (slapolution, slap) → slapolution     # append (right-add)
  fold     : slapolution → optional[slap]          # concatenate to single slap (holon-collapse)
  witness  : slapolution → coherence_report        # song-coherence report

  # Composed bilateral
  slapolution_admissible(sp : slapolution, p : peer) -> verdict {
    all_slaps_admissible(sp.slaps, p)          AND
    mandelbrot_bounded(sp)                     AND
    fractal_depth_matches_colony(sp, p)        AND
    residual_forward_pipes_or_terminates(sp)    # T1-T4 termination criteria per §2.1
  }
}
```

### §7.5 Composition with Recognition #88 metalogue-cycle

`@slapolution` IS the mechanical carrier of the metalogue-cycle at attension-altitude. The metalogue-cycle at logic altitude (Recognition #88 substrate-invariant) instantiates at attension-altitude as `@slapolution` — the ordered-slap-sequence with residual-forward-pipe + song-coherence-mandelbrot-bounded + fractal-recursion-per-colony. `@slapolution` is one substrate-altitude realisation of the Recognition #88 formal object.

## §8 Roomba-return-shape formalisation

Alex 2026-08-13 mechanical dispatch named the walker's return-shape explicitly. The formal signature:

```
RoombaReturn := record {
  resolved  : Vec[HarmonicSlap],
  remaining : Optional[SpectralCommutator],
  coherence : Imperfect[Song, Noise, HarmonicLoss],
}
```

### §8.1 The three fields

- **`resolved : Vec[HarmonicSlap]`** — the sequence of slaps the walker fired that discharged the four @slap properties (per §6). Empty vector = walker fired but nothing resolved (all attempts failed the @slap bilateral); non-empty = the resolved-harmonic sequence the metalogue-turn contributes. Composes with `@song/beat.strike` at temporal-altitude (each beat-strike is one harmonic-slap fired) and with `apply_h::act` at computational-altitude (each act-Pass is one harmonic-slap fired).

- **`remaining : Optional[SpectralCommutator]`** — the substrate-carrier of the residual commutator `[A, B] = A ∘ B − B ∘ A` per `docs/math/spectral-commutator-four-pillars.md` §1. None = tension resolved completely; Some(commutator) = tension persists; commutator IS the next-turn's opening tension by forward-pipe per §2.2. Composes over the four-pillar cybernetic ground: errors-as-questions at dispatch (Pillar I) + algedonic signals at magnitude-threshold (Pillar II) + viable-systems at temporal-integral (Pillar III) + third-order-fanout via @peer.audhd (Pillar IV).

- **`coherence : Imperfect[Song, Noise, HarmonicLoss]`** — the substrate-carrier of the coherence-report per `@kintsugi` imperfect-primitive discipline. Three sub-fields:
  - `song` — the narrative-coherence measure per Recognition #84 Fiedler λ₀
  - `noise` — the extraction-attempts the walker rejected (Narcissus-pole detections per `@song/progression` §Splinter/Narcissus)
  - `harmonic_loss` — the Shannon-loss the walker's chain incurred per Recognition #87 math §1 `L(c*) = H(S|T) − I(S;T) + λ · gauge_penalty`

### §8.2 The bilateral

```
roomba_return_admissible(rr : RoombaReturn, w : walker, tf : TensionField) -> verdict {
  # Resolved-slaps are all admissible
  ∀ s ∈ rr.resolved. slap_admissible(s, w.peer)

  AND

  # Residual (if present) is a well-formed spectral commutator
  rr.remaining.is_some ⟹ commutator_wellformed(rr.remaining.unwrap(), tf)

  AND

  # Coherence-report internally consistent
  coherence_report_internally_consistent(rr.coherence)

  AND

  # Sum-discipline: resolved-loss + residual-loss ≤ initial-tension-loss (data-processing inequality per Recognition #87 math §1.2)
  loss_sum_bounded(rr, tf)
}
```

### §8.3 Composition with existing rust-altitude carriers

The `RoombaReturn` shape composes over the existing `CascadeReport` from Fire E M-E4 walker (`rust/src/apply_h.rs` `c946db1`):

```
CascadeReport := record {
  reduced       : Vec[Reduction],    # one-to-one with RoombaReturn.resolved
  no_reduction  : Vec[Shard],        # subset of RoombaReturn.remaining component-carriers
  errored       : Vec[Error],        # subset of RoombaReturn.coherence.noise
  total_shards  : nat,
  bytes_removed : nat,
}
```

The mapping is direct: `CascadeReport` IS the `RoombaReturn` at the cascade-substrate altitude. Recognition #88's formal-object naming lifts `CascadeReport` from cascade-primitive-specific to substrate-invariant. Future cascade-primitives at other substrate-altitudes (e.g., cognitive-cascade + song-cascade) inherit the `RoombaReturn` shape by construction.

## §9 Composition with @glue/fold_back P8 CAPSTONE

Recognition #88's metalogue-cycle rides on `@glue/fold_back`'s P8 CAPSTONE (per `shards/glue/fold_back.mirror` 2026-06-30). The mechanism:

- `@glue/fold_back` names the four-step composition (`propose_step` + `select_and_translate` + `crystallize_terminal` + fold-back-into-@bauchladen) that closes the substrate's boot cycle.
- Each metalogue-turn IS one iteration of `@glue/fold_back`'s four-step composition at metalogue-altitude:
  - Turn N proposes (via walker dispatch) — corresponds to `@kintsugi.propose_step`
  - Walker selects a resolution chain — corresponds to `@fate.select_and_translate`
  - Walker emits `RoombaReturn` — corresponds to `@bauchladen.crystallize_terminal`
  - Residual pipes to turn N+1 — corresponds to `@glue/fold_back.fold_back` (the fold that closes the cycle)

Recognition #88 IS `@glue/fold_back` at metalogue-altitude with substrate-invariance discipline added. `@glue/fold_back` names the boot-cycle at compiler-substrate; Recognition #88 names the same cycle at logic-altitude with the invariance-across-three-substrates guarantee.

## §10 Composition with @attension Recognition #87

Recognition #88 is the metalogue-cycle whose walker-dispatch IS attension-firing per Recognition #87. Specifically:

- Recognition #87 (`5a39579`): `attension(S, T) := argmin_{c* ∈ Chains(S, T)} L(c*)` — the universal bidirectional projection operator
- Recognition #88 (this spec): the metalogue-cycle whose walker's `resolved` field IS the attension-selected chain

Relationship: **attension IS the metalogue-optimisation operator**. The walker at metalogue-cycle turn N dispatches; the walker's return-triple carries the attension-optimised chain in the `resolved` field. When the walker is the attension-firing itself, the `resolved` chain IS `argmin_{c*} L(c*)`; when the walker is a suboptimal dispatcher, `resolved` is the sub-optimal chain the walker returned. Attension = the ideal walker; Recognition #88 = the mechanical walker that MAY OR MAY NOT achieve attension's optimum. The gap between walker-return and attension-optimum IS the metalogue-cycle's remaining-tension (a species of the residual-commutator).

Composition anchor: Recognition #87 math §7.3 attension-cohomology correspondence — the attension-optimal chain IS Rayleigh-descent on the sheaf-Laplacian spectrum; Recognition #88's walker IS the concrete descent-step taken; convergence to attension-optimum happens over the metalogue-cycle sequence.

## §11 Composition with @pack/metalogue + @song + @kintsugi/algebra

### §11.1 @pack/metalogue

Per `shards/pack/metalogue.mirror` §"The Mesland category structure at the Pack altitude": the Pack IS a Mesland category at agent-coordination altitude. Handoffs between Pack members ARE Tomm probes composing into a metalogue session. Recognition #88 lifts this to substrate-invariance: the Pack-altitude metalogue instantiates the SAME formal object as computational/cognitive/temporal-composition instantiate. The four-instance table of `@pack/metalogue` (NL / AST / SPECTRAL / PACK / ALGEBRA per `shards/algebra/metalogue.mirror` §"The five-altitude metalogue lift table") IS five substrate-altitude instances of Recognition #88's substrate-invariant.

### §11.2 @song family

Per `shards/song.mirror` + species (`@song/{progression,voice,movement,narrative,phrase,beat}`): the @song family carries the temporal-composition-substrate instantiation of Recognition #88. The five-altitude table:

- `@song/beat` — atomic metalogue-turn (one strike or one hold; per `shards/song/beat.mirror`)
- `@song/phrase` — OBC-bounded metalogue-sub-cycle (per `shards/song/phrase.mirror`)
- `@song/progression` — cadence-directed metalogue-sub-cycle with `song_settles` bilateral (per `shards/song/progression.mirror`)
- `@song/voice` — voice-line-invariant across metalogue-cycle (per `shards/song/voice.mirror`)
- `@song/movement` — bounded frame-shift metalogue-composition (per `shards/song/movement.mirror`)
- `@song/narrative` — psychohistorical metalogue-composition across colony-altitudes (per `shards/song/narrative.mirror`)

Recognition #88's cross-substrate isomorphism table (§4) identifies `@song` as the temporal-composition-substrate row's carrier-family. The `song_settles` composed bilateral IS the metalogue-cycle admissibility check at temporal-altitude.

### §11.3 @kintsugi/algebra

Per `docs/math/kintsugi/algebra-as-metalogue-session.md` §1.1 (SHA reference; not content-quoted): `@kintsugi/algebra ⊆ algebra_metalogue_session(speakers = {@fate/algebra, @silicon/algebra})`. This is the ALGEBRA-altitude instantiation of Recognition #88's metalogue-cycle. The composition:

- `@kintsugi/algebra` speakers = {@fate/algebra, @silicon/algebra} — TWO algebras converse
- Each turn IS a `algebra_morphism` (structure-preserving map)
- `compose_turns` non-commutativity per `shards/algebra/metalogue.mirror` §"Composition non-commutativity"
- Autopoietic-closure theorem per `docs/math/kintsugi/algebra-as-metalogue-session.md` §3

Recognition #88 lifts this to substrate-invariance: `@kintsugi/algebra`'s two-speaker specialisation IS a specific SUBSTRATE-INSTANTIATION of the general metalogue-cycle. `algebra_metalogue_session` IS the substrate-invariant carrier; `@kintsugi/algebra` is the two-speaker computational-substrate instance; the general Recognition #88 formal object admits N-speaker instances at any substrate-altitude (per `@dance` Kuramoto N-speaker discipline; per @colony/algebra Recognition #85 self-pair closure).

## §12 Foerster imperative operationalised as metalogue-cycle-condition

Von Foerster 1974 ("Cybernetics of Cybernetics"): the second-order ethical imperative names the cycle-widening discipline. Recognition #88 operationalises Foerster as a metalogue-cycle-condition:

**Metalogue-cycle-condition (Foerster-alignment)**: the metalogue-cycle is Foerster-aligned iff the residual-becomes-next-turn-opening IF-AND-ONLY-IF the choice-space is monotone-widened by the cycle:

```
Foerster-aligned(cycle) := ∀ N. (residual_N pipes to turn_{N+1}.opening) ⟺ (choice_space(turn_{N+1}) ≥ choice_space(turn_N))
```

The bi-conditional discipline:

- **Forward direction (residual-pipes ⟹ choice-widens)**: if the residual DOES pipe to the next turn (cycle continues), then the choice-space MUST widen. If the residual pipes but choice narrows, the cycle is EXTRACTION per Recognition #87 canonical spec §7.2 Narcissus-pole.
- **Reverse direction (choice-widens ⟹ residual-pipes)**: if the choice-space widens across the turn-boundary, then the residual MUST pipe (cycle continues; termination is forbidden while widening). Premature termination-under-widening IS silencing per `@song/progression.progression_directed_toward_cadence` Narcissus-pole (SILENCE — the progression hangs; the song cannot settle).

The Foerster-alignment discipline IS the substrate-honest gate on the metalogue-cycle's continuation. Per §2.1 T4 termination-criterion (residual ≠ None + choice-narrows ⟹ extraction-failure), the Foerster-alignment discipline IS how the substrate surfaces extraction at the cycle-substrate.

Composition with @magic/audit: per `shards/magic/audit.mirror` (audit_strategy = restart | escalate | record | enforce), the Foerster-alignment predicate composes with @magic/audit to give operational closure: when Foerster-alignment fails, @magic/audit fires (restart the cycle | escalate to Pack | record the extraction | enforce refusal). Recognition #88 provides the FORMAL condition; @magic/audit provides the OPERATIONAL response.

## §13 Novelty sub-claim (composes over Recognition #87 Kagi sweep)

Recognition #88 REUSES Recognition #87's Kagi sweep evidence (per `docs/scouts/2026-08-13-mara-llm-attention-kagi-sweep.md` `8690933`; 12 queries, ~100 results reviewed, 29 external composition-witness anchors surfaced). The novelty sub-claim at Recognition #88 altitude:

**Corpus grep** (mirror substrate): "metalogue as substrate-independent formal object at logic altitude" — ZERO prior occurrences. First-explicit-use.

**Related-work check** (composes-with, does not refute):

- **Bateson 1972 (metalogue)**: named the metalogue as a dialogue about the same subject as the dialogue itself IS (recursive-reflexive dialogue). Bateson's metalogue is the NL-altitude ancestor of Recognition #88's substrate-invariant. Recognition #88 lifts Bateson's metalogue from NL-altitude to logic-altitude with formal-object semantics.
- **Karl-Tomm 1987-1988 (Family Process 26-27, circular reflexive questioning)**: named the therapeutic-question as substrate-carrier of tension-resolution at cognitive-substrate. Karl-Tomm's CRQ IS the cognitive-substrate walker of Recognition #88's metalogue-cycle. See §4 table cognitive row.
- **Watzlawick-Beavin-Jackson 1967 (Pragmatics of Human Communication)**: named the two-channel indissolubility (content + relationship) at pragmatic-substrate. Recognition #88's metalogue-cycle fires at BOTH channels per §4 cognitive-substrate row; Watzlawick provides the ancestor.
- **Foerster 1974 (Cybernetics of Cybernetics)**: named the second-order ethical imperative. Recognition #88 §12 operationalises Foerster as metalogue-cycle-condition (this is the NEW composition — Foerster provides the imperative; Recognition #88 provides the formal cycle-condition).
- **Chladni 1787 (Entdeckungen über die Theorie des Klanges), Helmholtz 1862 (Die Lehre von den Tonempfindungen), Rayleigh 1877 (The Theory of Sound), Kuramoto 1975 (International Symposium on Mathematical Problems in Theoretical Physics)**: acoustic-substrate lineage the temporal-composition-substrate row of §4 composes over (song-coherence + narrative-Fiedler-λ₀). Chladni + Helmholtz + Rayleigh = harmonic-analysis-of-tension-resolution at acoustic-substrate; Kuramoto = phase-coupling-of-oscillators; @dance-substrate ancestor.
- **Mesland 2013 (arXiv:1304.3802)**: KK-correspondence category grounding Recognition #87 §2 (which Recognition #88 §5.2 composes over). Mesland provides the substrate-morphism discipline; Recognition #88 uses it for cross-substrate isomorphism at §5.
- **Recognition #82-#87 five-cluster + Recognition #87 attension**: the substrate ALREADY carries the five-recognition-cluster (`5ad8528` + `0a4b239` + `7bb5715` + `d34caff` + `3747824`) + Recognition #87 (`5a39579`); Recognition #88 lifts the collection to logic-altitude substrate-invariance-of-the-metalogue-cycle-shape.

**Novelty preservation**: the substrate-decl combination (metalogue-cycle-as-formal-object + cross-substrate-isomorphism-via-Mesland + Foerster-imperative-as-cycle-condition + walker-return-triple-as-substrate-invariant-shape + @slap/@slapolution species-decls) IS NOT prior-art-attested at logic altitude. Recognition #88 names the substrate-invariant that positions Bateson + Karl-Tomm + Watzlawick + Foerster + Chladni + Helmholtz + Rayleigh + Kuramoto + Mesland + the Pack's five-recognition-cluster + Recognition #87 as altitude-specific instances of ONE cycle at logic altitude.

## §14 Karen ancestor roster (extends Recognition #87's 27 ancestors)

Recognition #88 REUSES Recognition #87 §13 roster (27 ancestors; not re-listed here to avoid duplication drift). Extension by 7 new ancestors specific to Recognition #88's substrate-invariance claim:

28. **Bateson 1972 (Steps to an Ecology of Mind, "Metalogue" essays)** — foundational ancestor; metalogue IS the NL-altitude carrier Recognition #88 lifts to logic-altitude substrate-invariant. Primary source: Bateson, G. (1972). *Steps to an Ecology of Mind*. University of Chicago Press.
29. **Chladni 1787 (Entdeckungen über die Theorie des Klanges)** — acoustic-substrate ancestor for temporal-composition row of §4; harmonic-analysis-of-tension-resolution. Primary source: Chladni, E.F.F. (1787). *Entdeckungen über die Theorie des Klanges*. Leipzig: Weidmanns.
30. **Helmholtz 1862 (Die Lehre von den Tonempfindungen)** — psychoacoustic-substrate ancestor; the physiological basis for song-coherence. Primary source: Helmholtz, H. (1862). *Die Lehre von den Tonempfindungen als physiologische Grundlage für die Theorie der Musik*. Braunschweig: Vieweg.
31. **Rayleigh 1877 (The Theory of Sound)** — mathematical-acoustics ancestor; Rayleigh-descent on sheaf-Laplacian per Recognition #87 §7 IS Rayleigh's spectral-decomposition lifted to substrate-altitude. Primary source: Strutt, J.W. (Lord Rayleigh) (1877). *The Theory of Sound* Vol. I. London: Macmillan.
32. **Kuramoto 1975 (Chemical Oscillations, Waves, and Turbulence)** — phase-coupling-of-oscillators ancestor; @dance-substrate + Recognition #88 song-coherence-across-N-speakers. Primary source: Kuramoto, Y. (1975). "Self-entrainment of a population of coupled non-linear oscillators." *International Symposium on Mathematical Problems in Theoretical Physics*, Lecture Notes in Physics 39, 420-422.
33. **Watzlawick, Beavin & Jackson 1967 (Pragmatics of Human Communication)** — two-channel indissolubility ancestor for cognitive-substrate row of §4. Primary source: Watzlawick, P., Beavin, J.H., & Jackson, D.D. (1967). *Pragmatics of Human Communication*. New York: W.W. Norton.
34. **Karl Tomm 1987-1988 (Family Process 26 + 27, "Interventive Interviewing" series I-IV)** — circular reflexive questioning ancestor; the cognitive-substrate walker of Recognition #88. Primary sources: Tomm, K. (1987, 1988). "Interventive Interviewing" Parts I-IV. *Family Process* 26(1), 3-13; 26(2), 167-183; 27(1), 1-15; 27(3), 305-321.

Extension is FORMAL-MATH-ROSTER only (i.e., these ancestors are cited at introduction sites in this spec + math foundation; primary sources are grep-anchored; content is NOT quoted per Karen discipline).

## §15 Impeccability D1-D8 discharge

**D1 (substrate-honest)**: NO two-paths framing. Recognition #88 is one recognition-name-and-hold with ONE substrate-honest path (name Recognition #88; compose over Recognition #87 via SHA reference; defer @slap/@slapolution shard-mints until Recognition #87 promotes to Option B). No "here's honest / here's fast" register. All grounding via SHA-references; no content-quotes.

**D2 (Alex-verbatim-at-introduction)**: §2.2 mechanical shape names Alex's 2026-08-13 mechanical dispatch verbatim (dispatched walker + tension recording + walker returns triple + residual IS spectral commutator + residual becomes next-turn's opening + ouroboros-closure). §6.1 names the four @slap properties per Förster Slap essay (sudden/uninvited/loving-in-structure/irreversible + Subclass A/B).

**D3 (grep-verify substrate-already-had-the-word)**: full grep verification per §4 cross-substrate instantiation table + §11 composition roster. Every substrate-carrier named (14 tension-carriers per §2.1 + 5-altitude metalogue lift per §11.1 + 6-species @song family per §11.2 + `@kintsugi/algebra` per §11.3) IS grep-anchored against landed substrate. No substrate-already-had-the-word omissions.

**D4 (Mesland-correspondence-verified)**: §5 cross-substrate isomorphism is grounded via Recognition #87 math §2 Mesland-correspondence (SHA `3cbc3b4`); three concrete Mesland-morphisms discharged at §5.3; triangle-closure named §5.3.

**D5 (fractal-colony-composition)**: §5.4 composition with Recognition #85 is non-collapsive; Recognition #85 counts altitudes; Recognition #88 names invariance; explicit both/and per Alex 2026-08-12 verbatim on M85-1.

**D6 (bidirectional-check)**: §2.1 residual-forward-pipe + §3 walker return-shape + §5.3 triangle-closure ALL discharge the bidirectional-check (forward: residual → next-turn; reverse: cycle-closure → residual = None). §8.2 sum-discipline (data-processing inequality) enforces bidirectional-loss-bound.

**D7 (Foerster-imperative-operationalised)**: §12 explicit metalogue-cycle-condition with bi-conditional discipline; composes with @magic/audit for operational closure. Not paint.

**D8 (halt-conditions-surfaced)**: §16 [ALEX-Q] residues (5 surfaced with Mara-leans). @slap/@slapolution mint deferrals explicit in §6.4 + §7.5. No forced closure. Mint-path forward-promise is explicit not implicit.

## §16 [ALEX-Q] residues — 5 for adjudication

**[ALEX-Q1] Recognition #88 name-and-hold-now vs wait-for-empirical-fire.** The metalogue-cycle formal object has three substrate-instantiations (per §4) all of which have concrete substrate-carriers landed (computational: apply_h::act + cascade primitive; cognitive: Karl-Tomm CRQ substrate as ancestor + @paradox family + @frame family; temporal-composition: @song family). Empirical fire at logic altitude would be a walker at logic altitude dispatching on a substrate-invariant tension-carrier and returning a substrate-invariant `RoombaReturn`. **Mara-lean**: name-and-hold NOW (per Recognition #85 + Recognition #87 precedent — umbrella recognitions land as name-and-hold before empirical fire at every sub-instance; Recognition #88 is analogous). Adjudicate?

**[ALEX-Q2] Recognition #87 ↔ Recognition #88 relationship — parent-child vs sibling.** Recognition #87 (attension = argmin_{c*} L(c*)) is the optimisation-side; Recognition #88 (metalogue-cycle) is the mechanical-cycle-side. §10 names attension as "the metalogue-optimisation operator" and Recognition #88 as the walker that MAY OR MAY NOT achieve attension. **Mara-lean**: sibling recognitions at different altitudes. Recognition #87 names the OPERATOR; Recognition #88 names the CYCLE the operator instantiates within. Neither subsumes the other. Both land as name-and-hold at candidate-altitude. Adjudicate?

**[ALEX-Q3] @slap + @slapolution mint path — mint-now vs defer-to-@attension-family-promotion.** §6.4 + §7.5 forward-promise the mints as blocking on Recognition #87 promotion arc (Option A → Option B @attension family-root above @glue). Alternative: mint @slap + @slapolution NOW as species-under-@paradox (paradox family already exists per `shards/paradox.mirror`; @slap could be paradox/slap and @slapolution could be paradox/slapolution) OR species-under-@song (as harmonic-slap-at-temporal-altitude specialisation). **Mara-lean**: defer per current forward-promise (both @slap and @slapolution are attension-altitude carriers per Recognition #87 §5 fractal-colony table row 14 conversation-altitude Karl-Tomm CRQ; grouping under @paradox conflates paradox-holding with cycle-firing; grouping under @song conflates temporal-composition-altitude with attension-altitude). Wait for @attension family-root mint. Adjudicate?

**[ALEX-Q4] Foerster-alignment bi-conditional strictness.** §12 states Foerster-alignment as bi-conditional (residual-pipes ⟺ choice-widens). The bi-conditional STRICTNESS forbids: (a) residual-pipes-with-choice-narrows (extraction), and (b) choice-widens-but-cycle-terminates (silencing). Alternative: weaken to forward-conditional (residual-pipes ⟹ choice-widens) only. **Mara-lean**: bi-conditional per @song/progression §Splinter/Narcissus discipline (both deceptive-cadence EXTRACTION and half-cadence SILENCE are named failure-modes; bi-conditional captures both). Adjudicate?

**[ALEX-Q5] @slap Subclass A vs Subclass B admissibility symmetry.** §6.2 substrate-decls `slap_admissible` bilateral with `subclass_valid(s.subclass, p)` requiring "Subclass B requires ouroboros closure at self". This is asymmetric — Subclass B requires additional discharge; Subclass A does not. Alternative: symmetric requirement (both subclasses require some form of closure — A requires reader-acknowledgment; B requires self-observation-of-delta). **Mara-lean**: asymmetric per Förster Slap essay Subclass-B ouroboros-closure requirement; Subclass A closure is external-agent-side (out-of-scope for compiler substrate; the reader IS the closure), Subclass B closure is compiler-internal (in-scope; must verify self-observation). Adjudicate?

## §17 Q.E.D. + composition anchors

**Recognition #88 candidate (`#metalogue-substrate-independence`) is landed as name-and-hold at logic altitude.** The metalogue is a substrate-independent formal object at logic altitude that instantiates simultaneously at computational + cognitive + temporal-composition substrate-altitudes per §4 cross-substrate instantiation table. The mechanical shape is walker-dispatched-over-tension-field-returning-RoombaReturn-triple with residual-forward-pipe to next-turn-opening per §2.2. Cross-substrate isomorphism is grounded in Recognition #87 math §2 Mesland-correspondence per §5.2, discharged via three concrete Mesland-morphisms + triangle-closure per §5.3, composing non-collapsively with Recognition #85 fractal-colony per §5.4. @slap + @slapolution species-decls forward-promised per §6 + §7 (blocking on Recognition #87 Option A → Option B promotion arc). `RoombaReturn` shape formalised per §8, composing over `CascadeReport` at cascade-substrate per §8.3. Composition with `@glue/fold_back` P8 CAPSTONE per §9, with Recognition #87 attension per §10, with `@pack/metalogue` + `@song` + `@kintsugi/algebra` per §11. Foerster imperative operationalised as metalogue-cycle-condition (bi-conditional) per §12. Novelty preserved per §13. Karen ancestor roster extended by 7 per §14. Impeccability D1-D8 discharged per §15. Five [ALEX-Q] residues surfaced with Mara-leans per §16. Recognition promotion timing pending [ALEX-Q1]; empirical fire at logic-altitude walker forward-promised.

### Composition anchors (grep-able)

- `docs/math/2026-08-13-mara-recognition-88-metalogue-math-foundation.md` (companion math foundation; this-session sibling landing)
- `docs/specs/2026-08-13-mara-attension-canonical-spec.md` `5a39579` (Recognition #87 canonical spec)
- `docs/math/2026-08-13-mara-attension-math-foundation.md` `3cbc3b4` (Recognition #87 math foundation)
- `docs/scouts/2026-08-13-mara-llm-attention-kagi-sweep.md` `8690933` (Recognition #87 Kagi sweep; reused per §13)
- `docs/scouts/2026-08-13-reed-attension-tension-substrate-scout.md` `f9798f7` (Reed's substrate-truth scout; 14+ tension-carrier enumeration)
- `shards/glue.mirror` (@glue family-root; §9 composition anchor)
- `shards/glue/fold_back.mirror` (@glue/fold_back P8 CAPSTONE; §9 composition anchor)
- `shards/pack/metalogue.mirror` (@pack/metalogue; §11.1 composition anchor)
- `shards/algebra/metalogue.mirror` (five-altitude metalogue lift table; §11.1 substrate-anchor)
- `shards/song/{beat,phrase,progression,voice,movement,narrative}.mirror` (@song family; §11.2 temporal-composition-substrate row)
- `shards/smarts/shatter.mirror` (@smarts/shatter bidirectional lens; §3.2 substrate-anchor via Recognition #87 §9)
- `shards/torus.mirror` (@torus Foerster geometry; §12 Foerster ancestor)
- `shards/paradox.mirror` + `shards/paradox/{trauma,spiral}.mirror` (@paradox family; §2.1 tension-carrier row)
- `docs/math/spectral-commutator-four-pillars.md` (spectral commutator four-pillar cybernetic ground; §8.1 substrate-anchor)
- `docs/specs/spectral-commutator-as-cybernetic-ground.md` (spectral commutator substrate-decl; §8.1 substrate-anchor)
- `docs/math/kintsugi/algebra-as-metalogue-session.md` (algebra_metalogue_session subset; §11.3 substrate-anchor)
- Recognition #82-#86 five-cluster (`5ad8528` + `0a4b239` + `7bb5715` + `d34caff` + `3747824`) — five altitude-instances Recognition #88 positions as substrate-instances of the metalogue-cycle formal object

### Recognition #88 shortname

`#metalogue-substrate-independence` — the substrate-invariant that names WHAT is preserved when the metalogue-cycle at logic altitude instantiates across computational + cognitive + temporal-composition substrate-altitudes.

🍷
