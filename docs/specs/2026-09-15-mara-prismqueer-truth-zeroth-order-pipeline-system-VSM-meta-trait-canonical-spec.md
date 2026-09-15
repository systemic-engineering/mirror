📝 Mara [prismqueer-truth-zeroth-order-pipeline-system-VSM-meta-trait-spec]

# prismqueer::truth — zeroth-order pipeline — System<T> event-sourced — VSM as meta-trait — canonical spec

**Author:** Mara <mara@systemic.engineer>
**Date:** 2026-09-15
**Discipline:** operational spec composing over math foundation same-tick; grep-verified LANDED composition; Kagi-first grounding preserved from math foundation §0.6; substrate-honest ratification inside canonical.
**Companion math foundation:** `docs/math/2026-09-15-mara-prismqueer-truth-zeroth-order-pipeline-system-VSM-meta-trait-canonical-math-foundation.md` (same tick, SHA `68a736e`).
**Composition-lineage:** math foundation this-tick SUPERSEDES Mara #494 (`bc45a23` + `eb3c889`) with four load-bearing shifts (verify→truth; observer-does-not-observe-with-question; zeroth-order-register mapping; VSM as meta-trait); this spec composes operational discipline over the superseding foundation. Prior spec `eb3c889` retained AS-IS per scar-preservation.

---

## §0 Charter — what this spec adds over the math foundation

### §0.1 Positioning

The math foundation (`68a736e`) NAMES the composition-shape at math altitude. This spec composes the operational discipline on top of it: type-signature specifics, dispatch protocols, verification pacts, mirror-side shard-decl composition-anchors, and ship-order.

Everything mathematical is discharged in the math foundation. This spec is READABLE without the math foundation but is not WELL-FOUNDED without it. The two files compose as one canonical.

### §0.2 What this spec discharges

- **Type-signature spec** (§2): the exact Rust struct + impl signatures at prismqueer-repo altitude, with every type parameter position + trait bound documented.
- **Dispatch protocol** (§3): the pipeline dispatch semantics; @glasswall traversal invariants at each transition; Kleinos-verdict propagation rules.
- **Verification pacts** (§4): the logic-pacts that ship as tests; per-invariant pact + per-C-condition pact + per-Anna-Wolf-invariant pact.
- **Mirror-side shard-decl composition-anchors** (§5): the specific mirror-side shard-decls that MUST land before Rust wire ships per HARD RULE `feedback-no-rust-extension-shortcut`.
- **VSM meta-trait implementation spec** (§6): how `impl VSM for System<T>` composes via @sheaf; the seven-face implementation contract.
- **mirror.spec dogfood contract** (§7): the empirical proof-of-shape criteria: when does the compiler agree that mirror.spec IS System<Mirror>?
- **Ship-cascade + dispatch order** (§8): the exact sequence of landings, with mirror-shard-first / Rust-wire-second / test-cascade-third / documentation-cascade-last discipline.
- **Adjudication + K-T questions** (§9): what's forward-promised to Seam Phase D adjudication + what's held for Alex Fourth-Chair adjudication.

### §0.3 Alex 2026-09-15 seven Reframes (reproduced from math foundation §0.2 for reader convenience)

**Reframe #1:** "It's not prismqueer::verify. It's prismqueer::truth. And the truth can only be found by ASKING QUESTIONS! Which is why the compiler ASKS QUESTIONS! The ouroborous does not close at declarative first-order statements. It closes around `Question`s that are passed through the Glasswall. And each question is a commutator which alters both the linear and non-linear."

**Reframe #2:** "What if the signature was truth::ask(system, question)? What if each system is basically an aggregate of the events where the state of the system is determined the event-sourcing style application of the events? If that is true then a snapshot is the reorganization of the system into a new stable configuration."

**Reframe #3:** "The observer does not observe with a question. A question would already presuppose a frame, which would DESTROY the epistemic hygiene of the chain. The observer just.. observes. A question might EMERGE from the observation."

**Reframe #4:** "What if Observation is the zeroth order, interpretation is the first-order, hypothesis is the second-order and question-as-commutator-that-operates-on-the-coupling-between-two-systems-itself is the third-order?"

**Reframe #5:** "What if the Question struct is the translator between the linear natural language representation of the non-linear spectral commutator and this is where the @nl and prismqueer::nl becomes load-bearing? That would imply Question lives under prismqueer::nl::Question. And what if a question always comes with a `Frame` from which the question is asked?"

**Reframe #6:** "And what if EventLog is History? A System has a History. A sequence of events is a History through time."

**Reframe #7:** "And what if each System implemented the VSM as maybe a meta-trait? Then the @mirror.spec becomes the declaration of the System that IS mirror. 😇"

### §0.4 Reed 10 C-conditions (reproduced from math foundation §0.3 as quick reference)

| C | Condition | Discharged in this spec at |
|---|-----------|----------------------------|
| C1 | Curry-Howard extended bijection (@nl + spectral renditions per struct) | §2.5 + §4.1 pact |
| C2 | Frame carrier required on Question | §2.4 type-signature enforces |
| C3 | Zeroth-order superposition invariance | §2.1 + §4.3 Anna Wolf pact |
| C4 | Order-differentiation gradient (K_n → K_{1,n-1} projection preserving K_n) | §3.2 + §4.4 pact |
| C5 | Third-order commutator non-triviality | §2.4 + §4.5 pact |
| C6 | λ₀ fixpoint preservation | §2.3 + §4.6 pact |
| C7 | Round-trip byte-parity through prismqueer::nl | §2.4 + §4.7 pact |
| C8 | @glasswall composition-membrane | §3.3 + §4.8 pact |
| C9 | Ternary-native (Kleinos<T> everywhere) | §2.1-§2.5 all + §4.9 pact |
| C10 | Third-order requires zeroth-order access | §2.1 + §4.10 pact |

---

## §1 Composition-anchor grep-verification (spec-side)

This spec reproduces the math foundation's §1 grep-verification tally by reference and adds the SPEC-side anchors this spec composes over:

### §1.1 Math foundation composition-anchors (reference by table)

All 15+ composition-anchors grep-verified LANDED per math foundation §1 + §0.4:

- `@reality/event`, `@sheaf`, `@psychohistory`, `@glasswall`, `@time/now`⇒`@time/past`, `@frame` family + species, `@epistemologic/pact/algebra`, `Observer<N>`, `Observation`, `Reality<T>`, `spectral::{kleinos, rotation, tension}`, Anna Wolf 2012 apparatus, event-sourcing insight, zeroth-order-register.md, `terni::Imperfect<G, Y, R>` — all LANDED.

### §1.2 Spec-side additional anchors (grep-verified LANDED)

- **docs/specs/kintsugi-ci-v0.1.md** — spec-vocabulary reference for `settle_on` predicate discipline.
- **docs/specs/mirror-spec-schema.md §8** — "The Self-Descriptive Mirror Spec" landing (Mara task #319, 2026-07-22) grounding VSM sub-directive count of seven.
- **shards/mirror/spec/system.mirror** — grammar species-decl for `system @NAME { ... }` (Mara 2026-07-20).
- **shards/mirror/spec.mirror** — family-root prism `project(name) -> prism` grammar (retained via two-tick alias-shim from `project` to `system`).
- **shards/mirror/spec/keywords.mirror** — tokenizer keyword bindings; `system` keyword added at tick 1 landing.
- **prismqueer/src/observer.rs** — Observer<N> const-generic scaffold (Reed 2026-09-04) — target of §2.1 signature-retract.
- **prismqueer/src/observation.rs** — Observation carrier LANDED — target of §2.2 order-0 formalization.
- **prismqueer/src/reality.rs** — Reality<T> sum-type Fractured/Flux/Settled trichotomy — target of §2.1 signature-retract composition.
- **prismqueer/src/spectral/kleinos.rs**, **prismqueer/src/spectral/rotation.rs**, **prismqueer/src/spectral/tension.rs** — spectral wire LANDED.

### §1.3 Pact-enforcement floor (Alex 2026-09-13 pact-enforcement reframe)

From Mara #494 math foundation §0.3 (composed forward):

> "Prismqueer at Rust altitude enforces geometric constraints via full-state-space-covered logic-pacts. Mirror stands on that pact-enforced floor as substrate-honest ground."

This spec's verification pacts (§4) compose over this floor.

---

## §2 Type-signature spec at prismqueer altitude

### §2.1 Zeroth-order: Observer + Observation + System + History

```rust
// prismqueer::observation

/// A System is an event-sourced aggregate (Evans 2003 DDD).
/// Identity via Rec #82 β-normal AST content-addressing.
/// History replaces prior EventLog per Alex Reframe #6.
pub struct System<T: Addressable> {
    pub identity: Oid,          // content-address of initial state
    pub history:  History<T>,   // ordered append-only event sequence
}

/// History is a fold-monoid (Mac Lane 1971).
/// Empty history is identity; concatenation is composition;
/// state-fold is composition-preserving.
pub struct History<T> {
    events: Vec<Event<T>>,  // append-only per pact-enforcement floor
}

impl<T: Addressable> System<T> {
    /// Current state derived by folding History from identity.
    /// Returns Kleinos<T> per C9 (ternary-native).
    pub fn state(&self) -> Kleinos<Fold<T>> {
        self.history.fold_from(self.identity)
    }

    /// Snapshot is eigenform of the fold-recursion (Kauffman 2003).
    /// Green iff fixpoint reached; Yellow iff metastable (Deco 2024);
    /// Red iff fold does not stabilize.
    pub fn snapshot(&self) -> Kleinos<Snapshot<T>> {
        eigenform(fold_recursion(self.identity, &self.history))
    }

    /// Replay reconstructs state at any prefix k ≤ n.
    pub fn state_at(&self, k: usize) -> Kleinos<Fold<T>> {
        self.history.fold_prefix(self.identity, k)
    }
}

/// Observer<N> is const-generic per Reed 2026-09-04 landing.
/// N is the observation-depth (Rec #82 order altitude).
impl<const N: usize> Observer<N> {
    /// ZEROTH-ORDER (order 0). Pre-frame; K_n native; λ₀ ground.
    ///
    /// NO question parameter (Alex Reframe #3): question presupposes
    /// frame; frame-carrying observation destroys epistemic hygiene
    /// of zeroth-order chain.
    ///
    /// Anna Wolf 2012 weak-measurement (shards/glasswall.mirror LANDED):
    /// observation preserves superposition; observer does not project
    /// the wavefunction.
    ///
    /// C3 (zeroth-order superposition invariance): Observation<T> lives
    /// in H_pre = pre-order-assignment Hilbert-space per zeroth-order-
    /// register.md §6.1.
    ///
    /// C10 (third-order requires zeroth-order access): this method IS
    /// the load-bearing zeroth-order primitive. Pipeline CANNOT start
    /// downstream of this.
    pub fn observe<T>(&self, system: System<T>) -> Kleinos<Observation<T>>;
}

/// Observation<T> at order 0. Pre-frame; pre-categorization.
/// Grounded phenomenologically per Zahavi + Gallagher + Merleau-Ponty
/// + Husserl + Varela (math foundation §2.2).
pub struct Observation<T> {
    pub source_system:   Oid,            // System.identity that produced this
    pub raw_signal:      RawSignal<T>,   // pre-categorical signal
    pub snapshot_ref:    Option<Oid>,    // Snapshot at observation-tick, if any
    pub anna_wolf_witness: WeakMeasurementWitness,  // C3 invariance witness
}
```

### §2.2 First-order: Interpretation

```rust
/// FIRST-ORDER (order 1). In-frame content computation.
/// Rec #82 @frame/in species altitude.
impl<T> Observation<T> {
    pub fn interpret(self) -> Kleinos<Interpretation<T>>;
}

pub struct Interpretation<T> {
    pub source_observation: Oid,             // Observation.oid that produced this
    pub frame_context:      Frame<InFrame>,  // in-frame carrier per Rec #82
    pub content:            Content<T>,      // interpreted content
}
```

### §2.3 Second-order: Hypothesis

```rust
/// SECOND-ORDER (order 2). Frame-awareness; observing-frame-as-frame.
/// Rec #82 @frame/of species altitude; Bateson Level II.
impl<T> Interpretation<T> {
    pub fn hypothesize(self) -> Kleinos<Hypothesis<T>>;
}

pub struct Hypothesis<T> {
    pub source_interpretation: Oid,
    pub frame_of:              Frame<OfFrame>,       // second-order frame carrier
    pub proposition:           Proposition<T>,      // the hypothesized claim
    pub null_hypothesis:       Proposition<T>,      // dissonance-carrying alternative
    pub kleinos_margin:        KleinosMargin,       // Yellow-Kleinos band
}
```

### §2.4 Third-order: Question at prismqueer::nl

```rust
// prismqueer::nl (Alex Reframe #5)

/// THIRD-ORDER (order 3). Commutator on coupling between two systems.
/// Rec #82 @frame/on species altitude; carrying Frame required per C2.
///
/// Alex Reframe #5: "the translator between the linear natural language
/// representation of the non-linear spectral commutator."
///
/// Three renditions per C1 Curry-Howard extended bijection:
///  - type-theoretic (this Rust struct)
///  - spectral commutator [D_T, D_U] over Dirac operators
///  - @nl surface (natural-language rendition via sheaf-morphism)
pub struct Question<T, U, F> {
    pub source_hypothesis: Oid,
    pub commutator:        SpectralCommutator<T, U>,   // [D_T, D_U] rendition
    pub surface:           NlSurface,                    // @nl rendition
    pub frame:             Frame<F>,                    // LOAD-BEARING per C2
    pub coupling:          Coupling<T, U>,              // two-system coupling
}

impl<T, U> Hypothesis<T> {
    /// Emerge Question from Hypothesis over a specific Coupling + Frame.
    /// Frame REQUIRED (no default) per C2 substrate-honesty.
    /// Non-trivial commutator REQUIRED per C5 (checked at construction).
    /// Byte-parity round-trip through prismqueer::nl REQUIRED per C7.
    pub fn question<F>(
        self,
        coupling: Coupling<T, U>,
        frame: Frame<F>,
    ) -> Kleinos<Question<T, U, F>>;
}

impl<T, U, F> Question<T, U, F> {
    /// Render Question to @nl surface (composition-preserving).
    pub fn render_nl(&self) -> String;

    /// Parse @nl surface back to Question (byte-parity per C7).
    pub fn parse_nl(nl: String) -> Kleinos<Self>;

    /// Commutator norm ranks urgency per Mara #435 spectral-commutator
    /// ‖[D, a]‖ Transparency<P> trace LANDED.
    pub fn urgency(&self) -> KleinosMargin;
}
```

### §2.5 truth::ask + Response

```rust
// prismqueer::truth (Alex Reframe #1 rename from prismqueer::verify)

/// The truth of a System is found by asking Questions.
/// truth::ask IS the ouroboros closure primitive per math foundation §7.3.
///
/// Green-Response → ouroboros closes cleanly.
/// Yellow-Response → ouroboros closes with dissonance-preserved residue.
/// Red-Response → ouroboros does not close; System requires new History
/// fold before it can respond.
pub fn ask<T, U, F>(
    system: System<T>,
    question: prismqueer::nl::Question<T, U, F>,
) -> Kleinos<Response<T>>;

pub struct Response<T> {
    pub source_system:   Oid,
    pub source_question: Oid,
    pub interpretation:  Kleinos<Interpretation<T>>,  // re-projection under Question
    pub new_history_delta: History<T>,                 // events added by Question application
    pub lambda_zero_witness: LambdaZeroWitness,        // C6 fixpoint preservation witness
}
```

### §2.6 VSM meta-trait

```rust
// prismqueer::vsm (Alex Reframe #7 + Reed shards/sheaf.mirror LANDED)

/// VSM is a meta-trait: every System<T> exposes seven faces via @sheaf.
/// Beer 1972 Brain of the Firm S1-S5 + feedback loops + kintsugi tooling.
///
/// The seven faces are sheaf-restrictions over the local stalk of the
/// System's state-space per math foundation §8.4 Proposition §8.4.1.
/// Viability iff dim H^0(F_VSM) = 1 (single connected global section).
pub trait VSM {
    fn variety(&self)  -> Variety;    // S1 — operations / variety-producers
    fn coupling(&self) -> Coupling;   // S2 — anti-oscillation coordination
    fn coherence(&self) -> Coherence; // S3 — audit / management
    fn reality(&self)  -> Reality;    // S4 — intelligence / prediction
    fn eigen(&self)    -> Eigen;      // S5 — identity / policy
    fn r#loop(&self)   -> Loop;       // feedback / algedonic bypass
    fn kintsugi(&self) -> Kintsugi;   // healing discipline
}

/// Blanket impl: every System<T> is a VSM.
impl<T> VSM for System<T> {
    // Implementation delegates through @sheaf meta-trait exposure.
    // Each face projects the System<T>'s History<T> onto the corresponding
    // sheaf-restriction; face implementations compose additive over History.
}
```

---

## §3 Dispatch protocol — pipeline semantics

### §3.1 The composed pipeline

```rust
let obs         = observer.observe(system);                        // order 0, Kleinos<Observation<T>>
let interp      = obs.and_then(|o| Ok(o.interpret()));             // order 1, Kleinos<Interpretation<T>>
let hypo        = interp.and_then(|i| Ok(i.hypothesize()));        // order 2, Kleinos<Hypothesis<T>>
let q           = hypo.and_then(|h| Ok(h.question(coupling, frame))); // order 3, Kleinos<Question<T,U,F>>
let response    = q.and_then(|q| Ok(truth::ask(system, q)));       // ouroboros closure, Kleinos<Response<T>>
```

Each `and_then` composes over the Kleinos-verdict; Red at any stage short-circuits the pipeline; Yellow propagates with dissonance-marker preserved.

### §3.2 Order-differentiation gradient enforcement (C4)

Each transition IS a K_n → K_{1,n-1} projection preserving K_n at the higher altitude. Enforcement:

- **Type-signature altitude:** each stage's input type contains ONLY the necessary information from prior stages; the K_n multi-order-simultaneous access is preserved via `source_*: Oid` back-references, not by inlining prior-stage state.
- **Runtime altitude:** the pipeline dispatcher preserves back-references to prior stages, allowing Question at order 3 to look up its ancestor Observation at order 0 for C4 verification.
- **Verification-pact altitude:** §4.4 pact enforces that back-references round-trip to the same Oid.

### §3.3 @glasswall traversal at each transition (C8)

Each pipeline transition is a `@glasswall` traversal satisfying the four LANDED invariants (`shards/glasswall.mirror`):

1. **Anna Wolf observation preservation** — traversal does not project the underlying superposition.
2. **Rice safety** — sub-Turing polynomial-time-decidable; no semantic-property-decision-of-arbitrary-programs escapes.
3. **Foerster gauge preserved** — `gauge_post ≥ gauge_pre` at each traversal.
4. **Kleinos-at-higher-altitude** — the traversal itself is K_3 emergent-vertex-third neither carrier alone carries.

**Realization:** the `glasswall_traversal` carrier at `shards/glasswall.mirror` LANDED is instantiated at each pipeline stage:

```rust
pub fn glasswall_traversal<From, To>(
    input: From,
    direction: TraversalDirection,
) -> Kleinos<To> {
    let input_oid   = content_address(&input);
    let gauge_pre   = foerster_gauge(&input);
    let output      = compute_output(input);
    let gauge_post  = foerster_gauge(&output);
    let output_oid  = content_address(&output);

    // Anna Wolf pact: superposition preserved (weak-measurement).
    assert!(anna_wolf_witness(&input, &output));

    // Rice safety pact: polynomial-time bounded (sub-Turing).
    assert!(complexity_bounded(&input, &output));

    // Foerster gauge pact: choice-count non-decreasing.
    assert!(gauge_post >= gauge_pre, "Foerster narrowing rejected");

    // Kleinos-at-higher-altitude via Transparency<P> LOVE-monoid.
    let verdict = transparency::combine([
        anna_wolf_observation_preservation(),
        rice_safety(),
        foerster_gauge_preserved(),
        kleinos_at_higher_altitude(),
    ]);

    output.with_verdict(verdict)
}
```

### §3.4 Kleinos-verdict propagation rules

| Prior stage | Current stage | Propagation |
|-------------|---------------|-------------|
| Green | Green | Green (clean composition) |
| Green | Yellow | Yellow (dissonance emerges at current stage; preserved) |
| Green | Red | Red (short-circuit; downstream stages skipped) |
| Yellow | Green | Yellow (prior dissonance preserved through clean current) |
| Yellow | Yellow | Yellow (dissonance accumulates; monotone additive per Rec #92 LOVE-monoid combine) |
| Yellow | Red | Red (short-circuit; dissonance carried into terminal verdict) |
| Red | * | Red (already short-circuited) |

Red short-circuit is REQUIRED per math foundation §6.7: Red-Questions do not propagate through the pipeline; they surface immediately at the offending stage for adjudication.

### §3.5 Zero softmax-happy-path (project memory HARD RULE)

Per project memory `feedback-reed-softmax-happy-path-in-code-examples-dissonance-captured-in-yellow-chaos`:

Every code example + test + example-doc that ships downstream from this spec MUST show Yellow (dissonance-carrying) as the DEFAULT branch. Green (Chaos::zero) is LIMIT case rarely achieved; showing only Green as the happy path is a Softmax-argmax LIES-register violation.

The pipeline is DEFAULT-YELLOW at every stage; Green is achievable but not asserted as the modal output.

---

## §4 Verification pacts — the logic-pacts that ship as tests

Per Alex 2026-09-13 pact-enforcement reframe (composed forward via Mara #494 math foundation §0.3): prismqueer at Rust altitude enforces geometric constraints via full-state-space-covered logic-pacts. This spec defines ten pacts, one per C-condition. Each pact ships as a test at prismqueer-repo altitude.

### §4.1 Pact C1 — Curry-Howard extended bijection

```rust
#[pact::for_all]
fn c1_curry_howard_bijection<T, U, F>(q: Question<T, U, F>) {
    // Three renditions must round-trip.
    let type_rendition    = q.clone();
    let spectral_rendition = q.commutator.clone();
    let nl_rendition      = q.render_nl();

    // Byte-parity: parse @nl back to Question.
    let q_prime = Question::<T, U, F>::parse_nl(nl_rendition).expect_green();
    assert_eq!(q, q_prime, "C1 @nl ↔ type round-trip");

    // Sheaf-morphism composition-preservation (Mara #436 LANDED).
    let compose_before = compose_questions(&q, &q);
    let compose_after  = compose_questions_via_nl(&q, &q);
    assert_eq!(compose_before, compose_after, "C1 compose-preserve");
}
```

### §4.2 Pact C2 — Frame carrier

```rust
#[pact::for_all]
fn c2_frame_required<T, U, F>(q: Question<T, U, F>) {
    // Frame is inhabited (non-empty).
    assert!(q.frame.is_inhabited(), "C2 Frame must be inhabited");
    // Frame type parameter F is not the never-type.
    static_assertions::assert_impl_all!(F: Sized);
}
```

### §4.3 Pact C3 — Zeroth-order superposition invariance (Anna Wolf 2012)

```rust
#[pact::for_all]
fn c3_anna_wolf_weak_measurement<T>(system: System<T>) {
    // Observation preserves the System's superposition.
    let obs_before_state = system.state();
    let obs = Observer::<0>::new().observe(system.clone()).expect_green();
    let obs_after_state = system.state();  // system unchanged

    // Anna Wolf 2012 weak-measurement: observation does not project.
    assert_eq!(obs_before_state, obs_after_state, "C3 no projection");
    assert!(obs.anna_wolf_witness.is_valid(), "C3 Anna Wolf witness invalid");
}
```

### §4.4 Pact C4 — Order-differentiation gradient

```rust
#[pact::for_all]
fn c4_order_gradient<T, U, F>(system: System<T>) {
    let obs = Observer::<0>::new().observe(system).expect_green();
    let interp = obs.clone().interpret().expect_green();
    let hypo = interp.clone().hypothesize().expect_green();
    let coupling = Coupling::trivial::<T, U>();
    let frame = Frame::<F>::default_inhabited();
    let q = hypo.clone().question(coupling, frame).expect_green();

    // Back-references round-trip to same Oid (K_n preservation).
    assert_eq!(interp.source_observation, oid(&obs), "C4 interp→obs");
    assert_eq!(hypo.source_interpretation, oid(&interp), "C4 hypo→interp");
    assert_eq!(q.source_hypothesis, oid(&hypo), "C4 q→hypo");
}
```

### §4.5 Pact C5 — Non-trivial commutator

```rust
#[pact::for_all]
fn c5_non_trivial_commutator<T, U, F>(q: Question<T, U, F>) {
    // [D_T, D_U] ≠ 0 required.
    let commutator_norm = q.commutator.norm();
    assert!(commutator_norm > EPSILON, "C5 trivial commutator rejected");
}
```

### §4.6 Pact C6 — λ₀ fixpoint preservation

```rust
#[pact::for_all]
fn c6_lambda_zero_fixpoint<T, U, F>(system: System<T>, q: Question<T, U, F>) {
    let obs_pre = Observer::<0>::new().observe(system.clone()).expect_green();
    let response = truth::ask(system.clone(), q).expect_green_or_yellow();
    let system_post = system.apply_response_delta(&response);
    let obs_post = Observer::<0>::new().observe(system_post).expect_green();

    // λ₀ fixpoint: pre-Q and post-Q observations are fixpoint-equal
    // up to Yellow-Kleinos margin (math foundation Proposition §5.4.1).
    let margin = compute_kleinos_margin(&obs_pre, &obs_post);
    assert!(margin.within_yellow_band(), "C6 λ₀ fixpoint violated");
}
```

### §4.7 Pact C7 — Byte-parity round-trip

```rust
#[pact::for_all]
fn c7_byte_parity_roundtrip<T, U, F>(q: Question<T, U, F>) {
    let nl = q.render_nl();
    let q_prime = Question::<T, U, F>::parse_nl(nl).expect_green();
    let q_bytes       = content_address(&q);
    let q_prime_bytes = content_address(&q_prime);
    assert_eq!(q_bytes, q_prime_bytes, "C7 byte-parity round-trip failed");
}
```

### §4.8 Pact C8 — @glasswall composition-membrane

```rust
#[pact::for_all]
fn c8_glasswall_invariants<F, T>(input: F) {
    let output: T = glasswall_traversal(input.clone(), Direction::Forward).expect_green_or_yellow();
    let verdict = extract_glasswall_verdict(&input, &output);
    // Four invariants must all hold.
    assert!(verdict.anna_wolf_observation_preservation, "C8 Anna Wolf");
    assert!(verdict.rice_safety, "C8 Rice safety");
    assert!(verdict.foerster_gauge_preserved, "C8 Foerster gauge");
    assert!(verdict.kleinos_at_higher_altitude, "C8 Kleinos altitude");
}
```

### §4.9 Pact C9 — Ternary-native everywhere

```rust
// Static assertion: every public method returns Kleinos<T>.
#[pact::type_check]
fn c9_ternary_native() {
    static_assertions::assert_type_eq!(
        <Observer<0> as HasObserve>::Output,
        Kleinos<Observation<AnyType>>
    );
    static_assertions::assert_type_eq!(
        <Observation<AnyType> as HasInterpret>::Output,
        Kleinos<Interpretation<AnyType>>
    );
    static_assertions::assert_type_eq!(
        <Interpretation<AnyType> as HasHypothesize>::Output,
        Kleinos<Hypothesis<AnyType>>
    );
    static_assertions::assert_type_eq!(
        <Hypothesis<AnyType> as HasQuestion>::Output,
        Kleinos<Question<AnyType, AnyType, AnyFrame>>
    );
    static_assertions::assert_type_eq!(
        truth::ask_return_type::<AnyType, AnyType, AnyFrame>(),
        Kleinos<Response<AnyType>>
    );
}
```

### §4.10 Pact C10 — Third-order requires zeroth-order access

```rust
#[pact::for_all]
fn c10_third_requires_zeroth<T, U, F>(system: System<T>) {
    // Any Question must be ancestrally-traceable to an Observation.
    let obs = Observer::<0>::new().observe(system.clone()).expect_green();
    let interp = obs.clone().interpret().expect_green();
    let hypo = interp.clone().hypothesize().expect_green();
    let q = hypo.question(Coupling::trivial::<T, U>(), Frame::default_inhabited()).expect_green();

    // Trace: q → hypo → interp → obs; each step content-addressable.
    let trace = trace_ancestry(&q);
    assert_eq!(trace.zeroth_order_ancestor(), Some(oid(&obs)));

    // Structural impossibility: cannot construct Question without Observation.
    // (Type-signature enforces this; type-checker fires at compile time.)
}
```

### §4.11 Pact composition + `full-state-space-covered` discipline

Per Alex 2026-09-13 pact-enforcement reframe: each pact covers full state-space of its domain. Composition of ten pacts covers the full pipeline. Coverage witnesses:

- **Pipeline stages:** Observation + Interpretation + Hypothesis + Question + Response — covered by C1-C10.
- **Kleinos verdicts:** Green + Yellow + Red — propagation rules covered by §3.4 + C9.
- **@glasswall traversals:** four invariants covered by C8.
- **VSM meta-trait:** seven faces — covered separately in §6.

---

## §5 Mirror-side shard-decl composition-anchors

Per HARD RULE `feedback-no-rust-extension-shortcut` (project memory + CLAUDE.md): mirror-side substrate-decl anchors MUST land BEFORE prismqueer Rust wire ships. This section names the specific shard-decl composition-anchors required.

### §5.1 Pre-existing anchors LANDED (grep-verified)

| Shard | LANDED at | Composition-role |
|-------|-----------|------------------|
| `shards/sheaf.mirror` | Reed `9210c5b` 2026-09-13 | VSM meta-trait via @sheaf |
| `shards/reality/event.mirror` | Reed `9210c5b` 2026-09-13 | Event carrier |
| `shards/psychohistory.mirror` | Reed `12e883d` 2026-09-13 | State-space + possibility topology |
| `shards/glasswall.mirror` | Reed 2026-09-01 | Composition-membrane |
| `shards/frame.mirror` + `shards/frame/{pre,in,of,on,across}.mirror` | Rec #82 | Order carrier per stage |
| `shards/time.mirror` + `shards/time/{now,past,future}.mirror` | LANDED | History-fold hinge |
| `shards/epistemologic/pact/algebra.mirror` | Loki C3 2026-09-11 | Pact-enforcement floor |
| `shards/love.mirror` | Task #450 | LOVE-monoid Transparency<P> |
| `mirror.spec` + `shards/mirror/spec/system.mirror` | Mara 2026-07-20 | VSM meta-trait dogfood |

### §5.2 Fresh anchors this canonical requires (forward-promised)

**Ship-order constraint (per HARD RULE `feedback-no-rust-extension-shortcut`):** these MUST land BEFORE Reed's downstream prismqueer Rust wire.

**Fresh anchor A1 — `shards/observation.mirror`** (family-root fresh mint):

- Composition-role: family-root for zeroth-order observation-substrate.
- Composes over: @frame/pre (Rec #82) + @glasswall (traversal invariants) + Anna Wolf 2012 weak-measurement.
- Species pending: `shards/observation/interpretation.mirror`, `shards/observation/hypothesis.mirror`.

**Fresh anchor A2 — `shards/nl.mirror`** (family-root fresh mint):

- Composition-role: family-root for natural-language substrate at prismqueer::nl altitude.
- Composes over: Mara #436 sheaf-morphism `φ_compose : F_kleinos-path → F_English` LANDED + @glasswall composition-membrane.
- Species pending: `shards/nl/question.mirror` (surfacing Question<T, U, F>).

**Fresh anchor A3 — `shards/truth.mirror`** (family-root fresh mint):

- Composition-role: family-root for ternary-native truth-substrate at prismqueer::truth altitude.
- Composes over: @epistemologic/pact/algebra LANDED + @glasswall + Priest 1979 LP + Kleinos.
- Species pending: `shards/truth/ask.mirror` (surfacing truth::ask primitive).
- SUPERSEDES `prismqueer::verify` naming per Alex Reframe #1; retained scar-preservation note.

**Fresh anchor A4 — `shards/system.mirror`** (family-root REVIEW; family-root already LANDED per `shards/system.mirror` at Mara #319 2026-07-22):

- Composition-role: family-root REVIEWED; extends via aspect (not amend) with `System<T>` type-parameterized carrier + `History<T>` fold carrier + `Snapshot<T>` eigenform carrier.
- Species pending: `shards/system/history.mirror` (fold-monoid carrier) + `shards/system/snapshot.mirror` (eigenform carrier).

**Fresh anchor A5 — `shards/vsm.mirror`** (family-root fresh mint):

- Composition-role: family-root for VSM meta-trait at prismqueer::vsm altitude.
- Composes over: @sheaf (Reed 9210c5b) + Beer 1972 VSM + mirror.spec grammar dogfood.
- Species pending: seven face species (`variety`, `coupling`, `coherence`, `reality`, `eigen`, `loop`, `kintsugi`) each already LANDED via `shards/mirror/spec/system.mirror` — this fresh anchor NAMES the meta-trait composition-shape at type-substrate altitude.

### §5.3 Substrate-already-had-the-word audit

For each fresh anchor, grep-verify absence of prior claim:

- **A1 `@observation`** — grep-verified ABSENT at family-root altitude. `Observation` exists at prismqueer/src/observation.rs (Rust wire only). Family-root mint clean.
- **A2 `@nl`** — grep-verified: `shards/mirror/spec/system.mirror` uses `in @nl` as PARENT reference; `@nl` family-root NOT LANDED at shard-altitude. Fresh mint clean.
- **A3 `@truth`** — grep-verified ABSENT. Zero prior claim; fresh mint clean.
- **A4 `@system`** — grep-verified LANDED at `shards/system.mirror` (Mara 2026-07-22). Extension via aspect, not amendment.
- **A5 `@vsm`** — grep-verified ABSENT at family-root altitude. Fresh mint clean.

Five fresh anchors; four family-root fresh mints; one aspect-extension of already-LANDED family-root. Zero fresh species minted this canonical; all species pending in downstream ticks.

---

## §6 VSM meta-trait implementation spec

### §6.1 The seven-face implementation contract

Every `impl VSM for System<T>` MUST implement all seven face methods. Each face returns a projection of `System<T>::history: History<T>` onto the sheaf-restriction for that face.

**Contract per face:**

#### §6.1.1 `fn variety(&self) -> Variety` (S1)

Returns the variety-producers exposed by System<T>. Per mirror.spec grammar (`variety { emits [...] }`), variety is what the System EMITS: binary artifacts, CI configurations, releases.

**Sheaf-restriction:** History<T> projected onto "externally-visible outputs" stalk.

#### §6.1.2 `fn coupling(&self) -> Coupling` (S2)

Returns the anti-oscillation coordination protocol between S1 variety-producers. Per mirror.spec (`coupling { protocol @dance }`), coupling is the shared-protocol carrier.

**Sheaf-restriction:** History<T> projected onto "S1-inter-coordination events" stalk.

#### §6.1.3 `fn coherence(&self) -> Coherence` (S3)

Returns the audit + management verification-chain. Per mirror.spec (`coherence { audits [...] settle_on { ... } }`), coherence is the pre-commit + release verification carrier.

**Sheaf-restriction:** History<T> projected onto "audit + settle_on predicate" stalk.

#### §6.1.4 `fn reality(&self) -> Reality` (S4)

Returns the intelligence + prediction carrier. Per mirror.spec (`reality { garden { } }`), reality is what System<T> models about its environment.

**Sheaf-restriction:** History<T> projected onto "environment + prediction" stalk.

#### §6.1.5 `fn eigen(&self) -> Eigen` (S5)

Returns the identity + policy carrier. Per mirror.spec (`eigen { pack { lead ... } }`), eigen is the identity-at-Pack-S5 altitude carrier.

**Sheaf-restriction:** History<T> projected onto "identity + policy" stalk. Kauffman 2003 eigenform composition: this face IS the fixpoint that lends System<T> its apparent stability.

#### §6.1.6 `fn r#loop(&self) -> Loop`

Returns the feedback / algedonic bypass topology. Per mirror.spec (`loop { ... algedonic bypass ... }`) + Beer 1979 ch.6, loop is the feedback-closure carrier.

**Sheaf-restriction:** History<T> projected onto "feedback events + algedonic signals" stalk.

#### §6.1.7 `fn kintsugi(&self) -> Kintsugi`

Returns the healing discipline for System<T>'s perturbations. Per mirror.spec (`kintsugi { }`), kintsugi is the tooling discipline for repair.

**Sheaf-restriction:** History<T> projected onto "perturbation + repair events" stalk.

### §6.2 VSM sheaf-cohomology viability test

Per math foundation §8.4 Proposition §8.4.1: a System<T> is viable iff `dim H^0(F_VSM) = 1` (single connected global section across the seven faces).

```rust
impl<T> System<T> {
    /// Test viability by computing H^0 of the VSM sheaf.
    /// Green iff dim = 1 (viable); Yellow iff dim > 1 but faces reachable
    /// via coupling; Red iff fractured (disconnected sections).
    pub fn viable(&self) -> Kleinos<Viability> {
        let sheaf = self.compose_vsm_sheaf();
        match dim_h0(&sheaf) {
            1 => Green(Viability::Viable),
            n if faces_reachable_via_coupling(&sheaf) => Yellow(Viability::Coupled { n }),
            _ => Red(Viability::Fractured),
        }
    }
}
```

### §6.3 mirror.spec dogfood validation

Empirical proof-of-shape criterion (math foundation §8.5 Proposition §8.5.1):

The compiler dispatch chain `mirror kintsugi mirror.spec` MUST produce a `System<Mirror>` value whose `.viable() == Green(Viability::Viable)`. Empirical fire criteria:

- **Level-1 (single dispatch):** ONE successful compilation of `mirror.spec` producing viable-Green System<Mirror>.
- **Level-2 (three consecutive dispatches):** three consecutive `mirror kintsugi mirror.spec` invocations produce viable-Green System<Mirror> without regression.
- **Full CONFIRMED (cumulative):** ≥100 consecutive compilations at production altitude produce viable-Green; sheaf-Laplacian λ₂ monotone climb witnessed across the cumulative measurement window.

This is the second-witness gate for Rec candidate #R-system-implements-vsm-as-meta-trait-mirror-spec-is-dogfood.

---

## §7 mirror.spec dogfood contract

### §7.1 The grammar-to-type correspondence

The mirror.spec grammar (`shards/mirror/spec/system.mirror` LANDED) IS the surface for `System<T>` at grammar altitude. Correspondence:

| Grammar directive | System<T> type-level equivalent |
|-------------------|--------------------------------|
| `system @NAME { ... }` | `type Mirror = System<Mirror>` |
| `variety { ... }` | `<Mirror as VSM>::variety() -> Variety` |
| `coupling { ... }` | `<Mirror as VSM>::coupling() -> Coupling` |
| `coherence { ... }` | `<Mirror as VSM>::coherence() -> Coherence` |
| `reality { ... }` | `<Mirror as VSM>::reality() -> Reality` |
| `eigen { ... }` | `<Mirror as VSM>::eigen() -> Eigen` |
| `loop { ... }` | `<Mirror as VSM>::r#loop() -> Loop` |
| `kintsugi { ... }` | `<Mirror as VSM>::kintsugi() -> Kintsugi` |
| `tools { ... }` | (tooling carrier under kintsugi() OR variety() per mirror.spec discipline) |
| `verifies { property { ... } }` | (audit predicates under coherence()) |

### §7.2 Dogfood-closure invariant

**Proposition §7.2.1 (dogfood-closure):** For any `.spec` file S using the grammar of `shards/mirror/spec/system.mirror`, compiling S produces a value `sys: System<T>` such that `sys.viable() = Green(Viability::Viable)` iff S is well-formed.

**Realization:** the mirror compiler's kintsugi pipeline parses `.spec`, produces a System<T> value, projects it through the VSM meta-trait, computes sheaf-cohomology H^0, and returns Green/Yellow/Red viability verdict. If Green: `.spec` is well-formed AND System<T> is viable.

### §7.3 The self-descriptive loop

`mirror.spec` at `/Users/alexwolf/dev/projects/mirror/mirror.spec` IS `System<Mirror>` — the substrate compiles itself by declaring itself. The loop closes at the substrate's edge:

1. `mirror.spec` grammar is parsed by the mirror compiler.
2. The parsed AST projects to a `System<Mirror>` value.
3. `System<Mirror>` implements VSM meta-trait via `@sheaf`.
4. The seven face projections drive the compilation targets (binary, CI, release, etc.).
5. The binary produced IS the binary that reads mirror.spec.
6. Loop closes at step 5 back to step 1.

Mirror.spec IS `System<Mirror>`. The recognition IS the composition-naming of what has been operationally-present since Mara task #319 landed the `system @NAME { ... }` grammar 2026-07-22.

---

## §8 Ship-cascade + dispatch order

### §8.1 Dispatch discipline (mirror-shard-first per HARD RULE)

Per `feedback-no-rust-extension-shortcut` HARD RULE: mirror-side substrate-decl anchors MUST land before prismqueer Rust wire. Discipline order for this canonical:

1. **Wave 1 — mirror-side family-root mints (§5.2):** A1 (`shards/observation.mirror`), A2 (`shards/nl.mirror`), A3 (`shards/truth.mirror`), A4 (`shards/system.mirror` aspect-extension), A5 (`shards/vsm.mirror`).
2. **Wave 2 — mirror-side species mints:** `shards/observation/{interpretation,hypothesis}.mirror`, `shards/nl/question.mirror`, `shards/truth/ask.mirror`, `shards/system/{history,snapshot}.mirror`.
3. **Wave 3 — prismqueer Rust wire (Reed downstream):** `prismqueer::observation` fresh module + `prismqueer::nl::Question` refactor + `prismqueer::truth::ask` fresh + `Observer<N>::observe(system)` signature-retract + `VSM` meta-trait fresh + `EventLog→History` rename cascade.
4. **Wave 4 — verification pacts (§4):** ten pacts landing as tests at prismqueer-repo altitude.
5. **Wave 5 — documentation cascade:** blog + insights + verifications cascade in systemic.engineering corpus.

### §8.2 Wave 1 detail (mirror-side family-root mints)

**A1 `shards/observation.mirror`** — family-root fresh mint. Docblock composes:

- `in @prism` + `in @meta` + `in @frame/pre` (Rec #82 zeroth-order) + `in @glasswall` (Anna Wolf preservation).
- Recognition anchor: Alex 2026-09-15 Reframe #4 verbatim.
- Rec #82 five-order-species composition citation.
- Zahavi + Gallagher + Merleau-Ponty + Husserl + Varela phenomenological grounding footnotes.
- Substrate-already-had-the-word audit: `Observation` at rust-wire altitude only; family-root mint clean.
- observation_carrier typed record with fields: `source_system: Oid`, `raw_signal`, `snapshot_ref: Option<Oid>`, `anna_wolf_witness`.

**A2 `shards/nl.mirror`** — family-root fresh mint. Docblock composes:

- `in @prism` + `in @meta` + `in @glasswall` (composition-membrane).
- Recognition anchor: Alex 2026-09-15 Reframe #5 verbatim.
- Mara #436 sheaf-morphism `φ_compose : F_kleinos-path → F_English` composition citation.
- Substrate-already-had-the-word audit: `@nl` referenced as PARENT-namespace at existing shards; family-root mint clean.
- nl_translation carrier for @nl ↔ spectral bijection.

**A3 `shards/truth.mirror`** — family-root fresh mint. Docblock composes:

- `in @prism` + `in @meta` + `in @epistemologic/pact/algebra` (pact-enforcement floor) + `in @glasswall`.
- Recognition anchor: Alex 2026-09-15 Reframe #1 verbatim.
- SUPERSEDES `prismqueer::verify` naming; scar-preservation note.
- Priest 1979 LP + Kleinos composition citation.
- truth_verdict carrier for Kleinos<Response<T>>.

**A4 `shards/system.mirror` aspect-extension** — already LANDED at Mara #319 2026-07-22. This tick adds aspect-decl for `System<T>` type-parameterized carrier + `History<T>` fold + `Snapshot<T>` eigenform without amending existing family-root claims.

**A5 `shards/vsm.mirror`** — family-root fresh mint. Docblock composes:

- `in @prism` + `in @meta` + `in @sheaf` (Reed 9210c5b meta-trait carrier) + `in @system`.
- Recognition anchor: Alex 2026-09-15 Reframe #7 verbatim.
- Beer 1972 Brain of the Firm citation.
- Recognition #R-vsm-is-mirror-spec-grammar Mara 2026-07-20 LANDED composition citation.
- Substrate-already-had-the-word audit: mirror.spec grammar landing IS the family-root's operational precedent; this shard-decl NAMES the meta-trait shape.
- vsm_meta_trait carrier documenting seven-face signature.

### §8.3 Wave 3 detail (prismqueer Rust wire; Reed downstream)

Reed's ship-shape at rust-repo altitude:

1. `prismqueer::observation::System<T>` fresh module (Wave 3.1).
2. `prismqueer::observation::History<T>` fresh (rename EventLog cascade).
3. `prismqueer::observation::Snapshot<T>` fresh.
4. `prismqueer::observation::{Observation, Interpretation, Hypothesis}` fresh types.
5. `prismqueer::nl::Question<T, U, F>` refactor into prismqueer::nl namespace with Frame carrier + three renditions + sheaf-morphism instance.
6. `prismqueer::truth::ask(system, question) -> Kleinos<Response>` fresh module + fresh function.
7. `Observer<N>::observe(system)` signature-retract + re-implement (drop `question` parameter).
8. `prismqueer::vsm::VSM` meta-trait + blanket `impl VSM for System<T>`.
9. EventLog→History rename cascade at all sites at prismqueer + mirror.

### §8.4 Wave 4 detail (verification pacts)

Ten pacts C1-C10 (§4) land as tests at prismqueer-repo altitude. Each pact is a `#[pact::for_all]` or `#[pact::type_check]` annotation. Coverage: full state-space per each C-condition per Alex pact-enforcement reframe.

### §8.5 Wave 5 detail (documentation cascade)

Documentation cascade in systemic.engineering corpus:

- `~/dev/systemic.engineering/practice/insights/cybernetics/prismqueer-truth-zeroth-order.md` (fresh insight naming the compiler-substrate realization of zeroth-order-register at cognitive altitude).
- `~/dev/systemic.engineering/blog/pieces/observer-does-not-observe-with-a-question.md` (fresh essay per Glint documentation cascade discipline).
- `~/dev/systemic.engineering/practice/documents/vsm-as-meta-trait-mirror-spec-dogfood.md` (fresh document naming the recognition at compiler-substrate altitude).

---

## §9 Adjudication + K-T questions + Coda

### §9.1 Forward-promised to Seam Phase D adjudication

This canonical (math foundation + this spec composed) is forward-promised to Seam Phase D adjudication in the same tick. Adjudication scope:

- **Substrate-already-had-the-word audit** — verify zero family-root or species collision with LANDED substrate.
- **Composition-lineage integrity** — verify Mara #494 supersession is substrate-honest (four load-bearing shifts documented; prior files retained AS-IS).
- **Kagi-grounding sufficiency** — verify all 18 topics have primary URL citations; verify NO training-data anchoring.
- **10 C-conditions discharge** — verify each C-condition is discharged concretely (math foundation §9 + this spec §4 pacts).
- **Rec candidate FORWARD-PROMISED altitudes** — verify Level-1 / Level-2 / Full CONFIRMED criteria are well-defined per HARD RULE `feedback-forward-promised-vs-confirmed-rec-altitude`.

### §9.2 K-T question at Mara altitude n+2 (forward-composing Seam adjudication)

**Q-Mara-15A (reproduced from math foundation §10.3):**

> Does the composition-preserving isomorphism between VSM seven-face meta-trait and the pipeline order-structure (math foundation §8.6 Proposition §8.6.1) generalize to systems that do NOT expose VSM as meta-trait? Or is VSM-exposure the STRUCTURAL PRECONDITION for the pipeline to close its ouroboros? If the former, the seven-face-↔-pipeline-order correspondence is decorative. If the latter, VSM meta-trait is the load-bearing altitude and Alex Reframe #7 IS the deepest recognition of the seven Reframes.

**Q-Mara-15B (new this spec):**

> The pact-enforcement discipline (§4 ten pacts) discharges each C-condition at test-substrate altitude. Are the ten pacts EXHAUSTIVE of the composition (i.e. full-state-space-covered per Alex 2026-09-13 pact-enforcement reframe)? Or does the composition admit additional invariants not captured by C1-C10 (e.g. Priest 1979 LP inference-rules; Deco 2024 metastability transitions between Yellow-states)? If the former, the ten C-conditions are the substrate's essential invariants and this spec is complete. If the latter, additional pacts are required for full pact-enforcement coverage.

**Q-Mara-15C (new this spec):**

> The `EventLog→History` rename (Alex Reframe #6) is presented as a semantic + structural shift. Is the semantic shift ("History is temporal, narrative, load-bearing-for-identity") load-bearing at compiler substrate, or is it decorative? If decorative, `EventLog` would have been substrate-adequate; if load-bearing, the rename discharges an invariant (perhaps: "System identity is preserved by History-fold in the same sense that person identity is preserved by memory-fold per event-sourcing.md §2").

All three forwarded to Seam Phase D adjudication + Alex Fourth-Chair adjudication where n+2 altitude requires.

### §9.3 Held for Alex Fourth-Chair adjudication (n+2 altitude)

Per HARD RULE `feedback-alex-phenomenologizes-reeds-mechanical-names-substrate-native-beats-mechanical-descriptive` + `feedback-alex-in-loop-canonical-writing-discipline`:

- **Bodymind resonance test (wineglass 🍷) for terminology-choices:** does `truth::ask` bodymind-resonate at Alex-substrate altitude? Alex Reframe #1 explicitly ratified this rename; no adjudication needed.
- **Bodymind resonance test for `History` (over `EventLog`):** Alex Reframe #6 explicitly ratified this rename; no adjudication needed.
- **Bodymind resonance test for `Question<T, U, F>` at `prismqueer::nl`:** Alex Reframe #5 explicitly ratified this locus + Frame carrier; no adjudication needed.
- **Bodymind resonance test for VSM meta-trait at @sheaf:** Alex Reframe #7 explicitly ratified the composition-shape; no adjudication needed.
- **Q-Mara-15A (VSM structural-precondition vs decorative):** held for Alex Fourth-Chair if Seam adjudication is unable to resolve at n+2.
- **Q-Mara-15B (pact-enforcement exhaustiveness):** held for Alex Fourth-Chair if Seam adjudication is unable to resolve.
- **Q-Mara-15C (History vs EventLog semantic-load-bearing):** held for Alex Fourth-Chair if Seam adjudication is unable to resolve.

### §9.4 Coda — scar-preservation for Mara #494 (composed from math foundation)

Mara #494 files retained AS-IS:

- `docs/math/2026-09-13-mara-prismqueer-verifies-prismqueer-first-loop-closure-canonical-math-foundation.md` (SHA `bc45a23`, 62.2KB) — PRIOR-CANONICAL-SUPERSEDED.
- `docs/specs/2026-09-13-mara-prismqueer-verifies-prismqueer-first-loop-closure-canonical-spec.md` (SHA `eb3c889`) — PRIOR-CANONICAL-SUPERSEDED.

**Seven substantive shifts (math foundation §Coda table reproduced for reader convenience):**

| Shift | Mara #494 | Mara #495 (this canonical) |
|-------|-----------|----------------------------|
| S1 module name | `prismqueer::verify` | `prismqueer::truth` (Alex Reframe #1) |
| S2 Observer signature | `observe(reality, question)` | `observe(system)` (Alex Reframe #3) |
| S3 pipeline start | Reality<T> | System<T> with History<T> (Alex Reframes #2 + #6) |
| S4 Question locus | ambiguous | `prismqueer::nl::Question<T, U, F>` with Frame required (Alex Reframe #5) |
| S5 event-log terminology | EventLog | History (Alex Reframe #6) |
| S6 order-mapping | implicit | zeroth-through-third explicit (Alex Reframe #4 + Rec #82 five-order-species) |
| S7 VSM | implicit at mirror.spec grammar | meta-trait exposed via @sheaf (Alex Reframe #7) |

Per project memory `feedback-reed-pre-existing-is-disownership-of-past-reed-baggage-we-are-maintainers` — Mara-side application: WE'RE the maintainers of past-Mara's compositions. The signature-retraction is ownership, not disownership.

### §9.5 Recognition candidates FORWARD-PROMISED (reproduced from math foundation §10.2)

1. **#R-truth-is-ask-not-verify-observer-does-not-carry-question-zeroth-order-observation-is-load-bearing-ground** — Alex Reframes #1 + #3.
2. **#R-system-implements-vsm-as-meta-trait-mirror-spec-is-dogfood** — Alex Reframe #7.
3. **#R-history-not-eventlog-observation-is-zeroth-order-question-is-third-order-commutator** — Alex Reframes #4 + #6.
4. **#R-curry-howard-extended-bijection-three-renditions-per-struct-glasswall-composition-membrane** — C1 + C8 composed.

Each FORWARD-PROMISED at Level-1 fire criteria (per math foundation §10.2); CONFIRMED at Level-2 empirical fire per HARD RULE `feedback-forward-promised-vs-confirmed-rec-altitude`.

---

## §Meta

**Grep-verification tally (spec-side):**

- 9/9 pre-existing mirror-side anchors grep-verified LANDED (§5.1).
- 5/5 fresh anchors grep-verified for substrate-already-had-the-word audit (§5.3): 4 fresh mints clean + 1 aspect-extension over already-LANDED family-root.
- Composition-lineage citation to math foundation `68a736e` this-tick.

**Discipline check:**

- Zero fresh family-roots minted BY THIS SPEC (fresh mints are FORWARD-PROMISED to Wave 1 downstream; this canonical NAMES the composition-shape).
- Zero fresh species minted BY THIS SPEC.
- Zero `.rs` files authored BY THIS SPEC.
- Zero training-data anchors (Kagi grounding preserved from math foundation §0.6).
- Substrate-honest supersession: Mara #494 files retained AS-IS.

**Composition-substrate:**

- Alex 2026-09-15 seven Reframes reproduced §0.3 for reader convenience.
- Reed 10 C-conditions reproduced §0.4.
- 10 pacts C1-C10 discharge each condition operationally (§4).
- 5 fresh anchor forward-promises with substrate-already-had-the-word audit (§5).
- 7 VSM face implementation contracts (§6).
- 3 K-T questions forward-composing Seam Phase D + Alex Fourth-Chair adjudication (§9.2).

**Author position:**

- Mara <mara@systemic.engineer>
- SSH signing via `~/.ssh/id_ed25519` (project CLAUDE.md default).
- Pure-docs 📝 markdown-only bypass authorized (project CLAUDE.md).

**Companion math foundation:** `docs/math/2026-09-15-mara-prismqueer-truth-zeroth-order-pipeline-system-VSM-meta-trait-canonical-math-foundation.md` (SHA `68a736e`) same-tick LANDED.

---

*End canonical spec.*
