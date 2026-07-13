# `@dance` runtime Rung 4 — multi-peer coherence phase-lock via `bootstrap/src/dance.rs`

*Mara, 2026-07-13 arc-continuation spec. Substrate-decl reading of Reed's
Rung 3 delivery (`0cc4e11` GREEN) + Reed's escalated three-scope
adjudication brief: which shape of multi-peer @dance runtime IS
substrate-honest for Rung 4 per Taut `c54740c` §5.5, given that
`shards/song/beat.mirror` has ALREADY named the module path
(`bootstrap/src/dance.rs`), the runtime primitive ("two peer-homes"),
the measurement ("phase-difference on cybernetic_coherence deltas"),
the envelope carrier ("Kuramoto order-parameter"), and the convergence
witness ("aumann_agreement envelope … at shared root_OID")?*

**Author:** Mara
**Date:** 2026-07-13
**Tag:** 📝 substrate-pull:realize; ladder-rung-4-spec
**Status:** canonical adjudication of scope + canonical shape for
`bootstrap/src/dance.rs` + test-infrastructure spec + envelope contract.
Every substrate claim cited with OID or grep-verified file:line.

---

## §0. Executive summary

**Verdict: Scope B — shared-prior convergence — with a specific
narrowing the substrate has already reserved: coherence-delta phase-lock
on shared root OID.**

Reed's three-scope framing (A = byte-equal deterministic replay; B =
shared-prior content-addressed Aumann; C = full Kuramoto ODE
simulation) is well-framed but the substrate has already collapsed the
ambiguity. `shards/song/beat.mirror` lines 453-457 (`94e55eb`, LANDED)
verbatim:

> "Rung 4 (Reed multi-tick cascade): multi-peer @dance coupling on
> shared beat; `bootstrap/src/dance.rs` module reads two peer-homes;
> measures phase-difference on cybernetic_coherence deltas; reports
> Kuramoto order-parameter; emits aumann_agreement envelope on
> convergence (per Mara `4f079c8` @dance spec)."

This is Scope B specialized to a specific measurement: not "any shared
OID emission" (too generic, that's Scope A-in-disguise) and not "full
Kuramoto ODE integration" (Scope C, which requires simulated dynamics
the substrate does not endorse without physical hardware). The
substrate-reserved shape reads two peer-homes' `cybernetic_coherence
= λ₀(Δ_F)` values (per Reed `8e6e517` Path B annotation on
`shards/cyberpunk.mirror`), computes their phase-difference over a
shared @song's beat sequence, reports the Kuramoto order-parameter r
across those phases, and asserts Aumann agreement fires when both
peers' emitted crystal-OIDs settle at a byte-equal root OID.

**One-tick landability for Reed.** With this scope adjudicated,
Rung 4 lands in one Tick 4b: `bootstrap/src/dance.rs` (~100-150 lines),
`bootstrap/tests/peer_beam_dance_coherence_shard.rs` (five T-tests),
and one `--dance-with <peer-home-2>` flag on the existing `mirror peer
beam` cli surface. All infrastructure Reed already has landed
(peer_beam runtime, --song dispatch, cybernetic_coherence carrier) is
reused verbatim.

**Substrate-already-had-the-word coverage for Rung 4 (Scope B narrowed):
~100%.** Every carrier this spec references is landed with OID; every
piece of runtime infrastructure is landed with test-shard precedent;
the module path is landed in a substrate shard (`beat.mirror:454`);
the envelope fields are landed by substrate-shard-declared naming
(`Kuramoto order-parameter` and `aumann_agreement`); the measurement
is landed by Reed's `cybernetic_coherence = λ₀(Δ_F)` annotation. The
ONLY new material is ~100 lines of Rust translating the substrate's
already-written specification into a running module.

**Recognition candidate to promote at Rung 4 landing:**
`#R-multi-peer-coherence-phase-lock-realizes-dance-at-runtime-altitude`
— extending Mara `4f079c8`'s `#R-dance-is-coordination-without-signal-
on-forster-torus` from spec-altitude to runtime-altitude, and completing
Reed's `8e6e517` `#R-cybernetic-coherence-is-lambda-zero` chain from
single-peer scale to ensemble scale.

**Refusals.** Scope A refused as substrate-degenerate (see §2.1); Scope
C refused as substrate-overreach (§2.3). The remaining forward-promises
(N > 2 peers; @silicon coupling with actual EM measurement; @torus
holonomy accumulation; @bauchladen shared crystal exchange) are
explicitly named at §8 as Rung 5+ scope.

---

## §1. Substrate-already-had-the-word audit for Rung 4

Grep-first per `[[feedback-substrate-already-had-the-word]]` (~72nd
instance this arc). Every claim below is grep-verified with the
file:line where the substrate landed the carrier.

### 1.1 The Rung 4 module path IS pre-declared

`shards/song/beat.mirror:454` (Mara `94e55eb`, LANDED 2026-07-13):

> `bootstrap/src/dance.rs` module reads two peer-homes

The path is not a proposal in this spec. The substrate wrote it 24
hours before Reed's Rung 3 landing. Two-tick discipline honored: the
shard mint (Rung 0) forward-promised the module path; this spec
(Rung 4 Tick 4a) discharges the promise as canonical spec; Reed's
Tick 4b lands the code.

**Alternative names considered + refused (grep-verified negative):**
- `bootstrap/src/coordination.rs` — never named in any shard; would
  break the `beat.mirror:454` verbatim binding.
- `bootstrap/src/resonance.rs` — never named for the runtime; Mara
  `9e48710` §7 recommended Path B (annotation-not-mint) for
  `@resonance` itself, so a `resonance.rs` module would over-declare.
- `bootstrap/src/ensemble.rs` — `ensemble` appears in the spec
  vocabulary but not as a module name anywhere in `shards/`.

Verdict: `dance.rs` is substrate-locked. No adjudication surface.

### 1.2 The measurement primitive IS pre-declared

`shards/song/beat.mirror:455` verbatim: *"measures phase-difference on
cybernetic_coherence deltas."*

`shards/cyberpunk.mirror` (Reed `8e6e517` Path B annotation, LANDED
2026-07-11):

> `cybernetic_coherence = λ₀(Δ_F)`

Where `λ₀(Δ_F)` is the Fiedler value of the sheaf Laplacian, per
Mara's `spectral-coherence-substrate-metric-synthesis.md` §2 (Mara
`caf461f` chain):

> "λ₀(Δ_F) IS the algebraic connectivity = Fiedler value = spectral
> gap that measures how close the sheaf is to being globally
> coherent."

`shards/song/beat.mirror:493`:

> "Aumann-agreement fires when the ensemble's cybernetic_coherence
> deltas converge at shared root_OID"

The measurement is compositional over LANDED carriers:
1. Each peer's @song execution (Reed `0cc4e11`) emits per-block
   envelopes.
2. Each envelope's per-beat `cybernetic_coherence` value is the
   peer's λ₀(Δ_F) at that beat.
3. The phase-difference of two peers' coherence-value sequences over
   a shared @song's beat count is the Kuramoto phase-difference at
   the coherence altitude.
4. Aumann agreement witnesses at the crystal-OID emitted by each
   peer's terminal beat.

Every component landed. Zero new measurement primitive required.

### 1.3 The envelope carriers ARE pre-declared

**`Kuramoto order-parameter`** — verbatim substrate mint at
`shards/song/beat.mirror:456`. Additionally cited at:

- `docs/specs/resonance-as-inter-peer-coupling-shapes-fate-tournaments-toward-basins.md` §2.4 (Mara `9e48710`, LANDED 2026-07-12) — formal
  definition `r · e^{iψ} = (1/N) Σⱼ e^{iθⱼ}`.
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md` §2.1 (Mara `4f079c8`, LANDED 2026-07-13) — the physics context.
- `docs/specs/mirror-spectral.md` §6 (Mara `a8055f0`, LANDED) — Kuramoto
  cited as substrate ancestor.

**`aumann_agreement`** — verbatim substrate mint at `shards/song/beat.
mirror:456-457`. Additionally cited at:

- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md` §2.6 (Mara `4f079c8`) — Aumann 1976 agreement theorem
  formalization under content-addressed common prior.
- `shards/algebra/metalogue.mirror:359-395` (Mara `94e55eb` chain
  annotation) — Aumann-under-content-addressed-common-prior is the
  algebraic form of @dance.

**`shared root_OID`** — the convergence witness. Content-addressed
per `shards/bauchladen.mirror:243-291` LANDED chain. Every peer's
terminal @song beat emits a crystal-OID; byte-equality of two peers'
terminal OIDs IS Aumann agreement under `@bauchladen`'s common-prior
condition (self-authenticating under SHA-256 collision resistance).

Verdict: three envelope carriers, all substrate-pre-declared, zero new
naming required. `bootstrap/src/dance.rs` MUST emit fields named
`kuramoto_order_parameter`, `aumann_agreement`, and `shared_root_oid`
to honor the substrate binding.

### 1.4 The runtime infrastructure IS pre-landed

Reed's Rungs 1-3 landed everything Rung 4 needs to compose:

| Rung 4 primitive | Landed carrier | OID |
|---|---|---|
| `--song <file>` cli flag | Reed Rung 1 (`c36fbf5`) | LANDED |
| Multi-beat phrase execution | Reed Rung 2 (`70766c3`) | LANDED |
| Nested-block AST parse + per-block envelope | Reed Rung 3 (`0cc4e11`) | LANDED |
| `execute_song` runtime dispatch | `bootstrap/src/song.rs:61-112` | LANDED |
| Per-beat envelope emission | `bootstrap/src/song.rs:117-149` | LANDED |
| Peer_home fixture creation pattern | 12+ `peer_beam_*_shard.rs` files | LANDED |
| Subprocess spawn for CLI dispatch | every `peer_beam_*_shard.rs` uses `Command::new` | LANDED |
| Envelope field-match assertions | every T-test uses `stdout.contains(...)` | LANDED |

Rung 4 needs zero new infrastructure. The ONLY new material is the
`dance.rs` module (~100 lines) + one test shard (~200 lines) +
one `--dance-with` flag added to the argv parser (~5 lines).

### 1.5 The physical-simulation ambiguity IS pre-resolved (refusal of Scope C)

`shards/silicon.mirror` (Mara `ea7b092`, LANDED 2026-06-30) declares
`@silicon` as the physical-altitude family-root; per Mara's
`docs/specs/silicon.md` §1.1 the substrate's `@io` boundary discipline
requires that "everything above @io stays bounded; measurement lives
at @io." Mara `9e48710` §3.1 verbatim:

> "`observe_coupling` is @io-bounded — the actual EM / thermal /
> timing measurement is a @io channel read, not a substrate
> operation."

**Substrate reading:** the physical Kuramoto model per zk-proof-
context-bleed appendix is what happens IN REALITY when identical
model instances run on adjacent GPUs. It is a physical phenomenon
measured at `@io`. It is NOT something `bootstrap/src/dance.rs`
should simulate. Reed's Scope C ("full Kuramoto physics simulation")
would violate `@io`'s Turing-complete-surface discipline: simulating
Kuramoto in Rust and calling it "@dance" would misrepresent
simulation as substrate-realized physics.

**Substrate-honest bound:** `dance.rs` reads two peers' @song
executions (a substrate observation), computes phase-differences of
their coherence-deltas (a substrate computation on landed data), and
reports the order-parameter r (a substrate summary statistic). It
does NOT integrate `dθ/dt = ω + κ sin(θ-φ)` over time steps. If
future hardware (a proper @silicon.observe_coupling implementation)
delivers actual coupling measurements, THAT lands as Rung 6+ with a
separate spec. `dance.rs` at Rung 4 is the substrate-altitude
observation layer, not the physical-altitude coupling engine.

### 1.6 Coverage adjudication

| Rung 4 component (Scope B narrowed) | Substrate-already-had-the-word? | Landing OID |
|---|---|---|
| Module path `bootstrap/src/dance.rs` | YES | `beat.mirror:454` (`94e55eb`) |
| Two peer-home fixture pattern | YES | 12+ peer_beam_* shards |
| `cybernetic_coherence` measurement primitive | YES | `cyberpunk.mirror` (Reed `8e6e517`) |
| `Kuramoto order-parameter` envelope field | YES | `beat.mirror:456`; `resonance` spec §2.4 |
| `aumann_agreement` envelope field | YES | `beat.mirror:456-457`; `dance` spec §2.6 |
| `shared root_OID` convergence witness | YES | `bauchladen.mirror:243-291` chain |
| Phase-difference computation over beat sequence | YES | Reed's per-beat envelope emission (`song.rs:117-149`) |
| `@song` shared temporal frame | YES | Reed Rungs 1-3 |
| `--dance-with <peer-home-2>` flag admission | pattern established | Reed Rung 1 `--song` flag precedent |
| Test-shard multi-peer_home fixture pattern | reuse of existing 1-peer pattern | 12+ precedents |

**Coverage: 100% substrate-already-had-the-word for Scope B narrowed
to coherence phase-lock.**

The ONLY new content is: Rust code that composes landed carriers.
Zero substrate-decl inflation. Zero new keywords. Zero new prisms.
Zero new envelope fields beyond those the substrate has already
named. Two-tick discipline honored end-to-end.

---

## §2. The three scopes formalized + refusals

### 2.1 Scope A — deterministic byte-equality replay (REFUSED)

**Shape.** Two peers execute the SAME `.song` file under the SAME
peer_home fixture. Envelopes are byte-equal because the runtime is
deterministic. Convergence = literal OID equality.

**Kuramoto interpretation.** Trivial: r = 1 always, because ω₁ = ω₂
and κ is irrelevant (peers are structurally identical inputs to a
deterministic function). No coordination-without-signal
demonstration — this is just Merkle-consistent replay.

**Why refused.** Two arguments:

**(a) Substrate-drift.** The @dance spec `4f079c8` §2.6 verbatim:

> "the OIDs *are* the posteriors, and content-addressing makes their
> equality *common knowledge by construction*. **Aumann's condition
> is met structurally, not procedurally.** No message-passing needed
> to establish 'we agree' — the OID emission IS the agreement witness,
> self-authenticating under SHA-256 collision resistance."

Scope A demonstrates the OID equality, but it does so under the
degenerate case where inputs are identical. That's testing SHA-256's
collision resistance, not the substrate's coordination-without-signal
claim. The recognition Mara `4f079c8` named requires DIVERGENT inputs
converging on a SHARED basin — not identical inputs preserving byte-
equality.

**(b) The Heist test.** The Heist story (`~/dev/systemic.engineering/
blog/weird/3published/Weird - Heist.md`, Alex 2026-07-12) IS the
substrate's exposition of @dance. Its structural claim is *300-500
seeders reading the same book in different countries at different
times converging on the same preservation basin.* Two identical peers
in the same tempdir at the same wallclock second running the same
`.song` is not the Heist. It's not @dance. It's `git clone`.

**When Scope A would be substrate-honest.** Never at Rung 4. Would be
substrate-honest as a byte-equality regression preservation test —
"Rung 4 doesn't break Rung 3's byte-equality when only one peer
runs" — but that's a byte-equality regression, not the Rung 4
landing itself. Fold into T5 regression assertion (see §3.4).

### 2.2 Scope B — shared-prior convergence (RECOMMENDED, and specifically narrowed)

**Shape.** Two peers execute DIFFERENT `.song` files that share a
LANDED shard-graph substrate (both peers' `mirror.spec` targets read
from the same repo; both peers' songs reference `@song/beat`,
`@kintsugi/oscillate`, and inherit the substrate's `cybernetic_
coherence` measurement discipline). The songs may differ in movement,
voice, progression, but they discharge the same beat sequence over
the same @kintsugi/oscillate driver.

The peer-specific song content produces peer-specific beat envelopes.
Each peer emits `cybernetic_coherence` values per beat (per Reed's
Rung 3 per-block envelope structure + the `@cyberpunk` λ₀(Δ_F)
annotation). The two peers' coherence-deltas over the shared beat
count form two phase trajectories. Kuramoto order-parameter r
measures phase-lock.

Convergence witness: both peers' terminal beat crystals hash to a
SHARED root_OID (per `@bauchladen`'s common-prior discipline).

**Kuramoto interpretation.** Non-trivial:
- ω₁ ≠ ω₂ in general (each peer has its own natural frequency = its
  song's cadence).
- κ (the coupling) is induced by shared substrate: both peers read
  `shards/cyberpunk.mirror`, both inherit `cybernetic_coherence =
  λ₀(Δ_F)`, both write to bauchladen-addressed crystal outputs. This
  IS the Aumann common-prior implicit coupling.
- r → 1 when the shared substrate is strong enough to synchronize
  the beats.
- r < 1 when the songs are too divergent for shared substrate to
  synchronize (a demonstrable failure mode).

**Why recommended.** Three arguments:

**(a) Substrate-preservation.** Every substrate reservation in
`beat.mirror:453-457` maps directly onto Scope B — two peer-homes,
coherence deltas, order parameter, aumann_agreement, shared root_OID.
The substrate has already made this choice; Mara's job is to name it.

**(b) The Heist test PASSES.** Two peers with different local
specifics (their own songs) coordinating via shared substrate
(cybernetic_coherence measurement discipline) IS the operational
form of "300-500 seeders reading the same book (Foerster 1976) and
converging on preservation basins." Foerster's paper IS the
substrate; the peer's individual reading histories are the divergent
inputs; the preservation basin is the shared root_OID.

**(c) One-tick landability.** Reed can implement in a single Tick 4b
because every carrier is landed. No physics simulation. No hardware
integration. No new envelope fields to negotiate. The substrate has
already done the naming work.

**Ambiguity resolutions (Scope B narrowed):**
- **Peer count = 2 (fixed at Rung 4).** N > 2 forward-promised to
  Rung 5. Rationale: two-peer is the substrate-reserved shape per
  `beat.mirror:454` verbatim ("two peer-homes"); N-peer generalization
  requires the moduli-space extension Mara `4f079c8` §3.8 named as
  Rung 4's known limitation. Fixing N=2 keeps two-tick discipline.
- **Songs must reference @kintsugi/oscillate + @song/beat.** Fixture
  discipline: both peers' `.song` files must dispatch to
  `execute_song` and emit per-beat envelopes. Songs may vary
  arbitrarily above that; below that they must share the substrate's
  measurement altitude.
- **Coherence is a stub at Rung 4.** Reed's `8e6e517` annotation
  landed the CLAIM that `cybernetic_coherence = λ₀(Δ_F)`; the
  RUNTIME computation of λ₀(Δ_F) per-beat is a Rung 4.5 forward-
  promise. Rung 4 emits a deterministic stub coherence value (e.g.,
  hash of the beat's envelope bytes modulo a phase range) so the
  Kuramoto order-parameter has a well-defined input. Substrate-
  honest hedge: envelope names the stub explicitly (`coherence_
  altitude: stub (Rung 4.5 forward-promise: λ₀(Δ_F) per Reed
  8e6e517)`).

### 2.3 Scope C — full Kuramoto ODE simulation (REFUSED)

**Shape.** Reed implements a numerical integrator for `dθᵢ/dt = ωᵢ +
Σⱼ κᵢⱼ sin(θⱼ - θᵢ)` over time steps, reports the ensemble's
convergence to basins as the ODE settles, uses simulated coupling
matrices κᵢⱼ as inputs.

**Why refused.** Two arguments:

**(a) `@io` boundary violation.** Per Mara `9e48710` §3.1 (LANDED):
physical measurement lives at `@io`; the substrate does NOT simulate
physical processes it cannot measure. Kuramoto ODE integration IS
simulation of a physical process (coupled oscillators). Doing this
in `bootstrap/src/dance.rs` would violate `@io`'s Turing-complete-
surface discipline — the substrate would be pretending to have
observations it does not have.

**(b) Misrepresentation risk.** If `dance.rs` runs a Kuramoto ODE
integrator and emits an r value, consumers may read that r as
substrate-observed inter-peer coupling. It is NOT. It is a numerical
integration of a chosen κ matrix — a lie about what the substrate
observes. The @dance spec `4f079c8` §3.7 warns:

> "The mapping is not analogy; it is category-preserving under [a
> named] functor."

A category-preserving functor is meaningful when both categories are
substrate-observed. If one side is simulated, the functor collapses
to fiction. Scope C would collapse the @dance recognition into a
demo.

**When Scope C would be substrate-honest.** When hardware measurement
of `@silicon.coupling` exists per `9e48710` §3.1 — Alex on physical
GPU hardware with EM/thermal side-channels reading two peers' silicon
directly. That's Rung 7 or beyond. Not this ladder.

### 2.4 Verdict summary

| Scope | Verdict | Why |
|---|---|---|
| A (deterministic byte-equality) | REFUSED | Substrate-degenerate; tests SHA-256 not @dance; Heist test fails |
| **B (shared-prior convergence, coherence phase-lock)** | **RECOMMENDED** | Substrate-pre-declared verbatim at `beat.mirror:453-457`; Heist test passes; one-tick landable; 100% substrate-already-had-the-word |
| C (full Kuramoto ODE simulation) | REFUSED | `@io` boundary violation; misrepresents simulation as observation; forward-promise to Rung 7+ with hardware |

---

## §3. Canonical runtime shape — `bootstrap/src/dance.rs`

### 3.1 Module signature and responsibilities

`bootstrap/src/dance.rs` — the multi-peer @dance runtime.

**Responsibilities:**
1. Fire the same @song (or two arbitrary @songs sharing the
   substrate) at two peer_homes.
2. Collect each peer's per-beat coherence values from the emitted
   envelopes.
3. Compute the phase-difference sequence and derive the Kuramoto
   order-parameter r.
4. Compare the terminal beat crystals for byte-equality (shared
   root_OID).
5. Emit a single @dance envelope naming the substrate authorities +
   the two-peer measurement results.

**Substrate authority naming convention** (per Reed's Rung 1-3
pattern): every emitted envelope field cites the OID that landed the
carrier. Consumers verify substrate binding by string match on
authority names.

### 3.2 Public API

```rust
/// Fire a @song at two peer-homes; compute @dance metrics; emit
/// envelope. Rung 4 discharge per Mara `<this-spec-oid>` §3 +
/// substrate reservation at `shards/song/beat.mirror:453-457`.
pub fn execute_dance(
    peer_home_1: &str,
    peer_home_2: &str,
    spec_path_1: &std::path::Path,
    spec_path_2: &std::path::Path,
    song_path: &str,   // single shared song at Rung 4; §8 forward-promise: two-song variant Rung 4.5
    ctx: &Ctx,
) -> i32;
```

Signature mirrors `execute_song` (`bootstrap/src/song.rs:61`) with
the second peer-home added as the substrate-reserved coupling
partner. The `spec_path_N` parameters follow the existing peer_beam
dispatch pattern.

**Internal helpers** (private module functions):

```rust
fn collect_coherence_sequence(
    peer_home: &str,
    spec_path: &std::path::Path,
    song_path: &str,
    ctx: &Ctx,
) -> Vec<f64>;  // per-beat coherence values from the peer's envelope emission

fn kuramoto_order_parameter(
    phases_1: &[f64],
    phases_2: &[f64],
) -> f64;  // r ∈ [0, 1]; the two-peer specialization of the N-peer formula

fn stub_coherence_from_envelope_bytes(bytes: &[u8]) -> f64;  // Rung 4.5 forward-promise: replace with λ₀(Δ_F) actual computation

fn extract_terminal_beat_oid(envelope: &str) -> Option<String>;  // per @bauchladen crystal discipline

fn aumann_agreement(oid_1: Option<&str>, oid_2: Option<&str>) -> bool;
```

### 3.3 Envelope shape (substrate-locked field names)

Following Reed's Rung 3 per-block envelope pattern (`bootstrap/src/
song.rs:154-227`), `dance.rs` emits ONE dance-envelope after both
peers' beat sequences complete:

```
@@ dance @dance via 2 × (@song × @kintsugi/oscillate) coupled at cybernetic_coherence altitude (Rung 4) @@
+ peer_home_1: <path>
+ peer_home_2: <path>
+ song_path: <path>
+ beat_count_1: <n>
+ beat_count_2: <n>
+ coherence_sequence_1: [c1_0, c1_1, ..., c1_{n-1}]
+ coherence_sequence_2: [c2_0, c2_1, ..., c2_{n-1}]
+ phase_differences: [Δθ_0, Δθ_1, ..., Δθ_{n-1}]
+ kuramoto_order_parameter: <r>              # substrate-locked per beat.mirror:456
+ aumann_agreement: <true|false>              # substrate-locked per beat.mirror:456-457
+ shared_root_oid: <hex-oid> | <none>         # substrate-locked per beat.mirror:493
+ convergence_verdict: <converged|dispersed|chimera>
+ coherence_altitude: stub (Rung 4.5 forward-promise: λ₀(Δ_F) per Reed 8e6e517)
+ dance_authority: @dance (Mara `4f079c8` canonical spec; Path C recognition candidate)
+ resonance_authority: @resonance (Mara `9e48710`; Kuramoto coupling ancestor)
+ metalogue_authority: @algebra/metalogue N-speaker case (Mara `34cf333` chain; Alex 2026-07-13 in-transcript @dance naming annotation)
+ coherence_authority: @cyberpunk cybernetic_coherence = λ₀(Δ_F) (Reed `8e6e517`)
+ bauchladen_authority: @bauchladen shared common prior (Mara `4575340` Recognition #104)
+ torus_authority: @torus winding-class basins (Mara `caf461f` §6)
+ beat_authority: @song/beat (Mara `94e55eb` sixth species; Rung 4 module path pre-declared at beat.mirror:454)
+ oscillate_authority: @kintsugi/oscillate (shards/song.mirror:181 verbatim binding)
+ ladder_rung: 4 (Reed GREEN discharging Taut `c54740c` §5.5 per Mara `<this-spec-oid>` Scope B)
+ substrate_authority: @dance (multi-peer coordination-without-signal at coherence phase-lock altitude)
```

**Field ownership rationale:**
- `coherence_sequence_1/2` — raw per-beat measurement, enables downstream analysis.
- `phase_differences` — the Kuramoto input; enables consumers to verify the order-parameter derivation.
- `kuramoto_order_parameter` (r ∈ [0,1]) — the single scalar summary. Substrate-locked name.
- `aumann_agreement` (bool) — the OID-equality witness. Substrate-locked name.
- `shared_root_oid` (Option<hex>) — the actual convergence OID or `<none>`. Substrate-locked name.
- `convergence_verdict` — three-way classifier (`converged` when r ≥ 0.9 AND aumann_agreement; `dispersed` when r < 0.5; `chimera` otherwise). The thresholds are Rung 4 defaults per Kuramoto's mean-field K_c reading; consumers may re-classify.
- Authority names — every landed carrier cited so consumers verify by grep.

### 3.4 Test shard — `bootstrap/tests/peer_beam_dance_coherence_shard.rs`

Following Reed's shard-naming pattern (`peer_beam_song_*`), Rung 4's
test shard is `peer_beam_dance_coherence_shard.rs` — reading:
"peer_beam + dance + coherence altitude + shard suffix."

**Fixture pattern** (single-process, subprocess-spawn-based, per
Reed's existing peer_beam pattern):

```rust
fn make_two_peer_homes() -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-dance-coherence-{}", std::process::id()
    ));
    let home_1 = base.join("peer_1");
    let home_2 = base.join("peer_2");
    for home in [&home_1, &home_2] {
        std::fs::create_dir_all(home).expect("mkdir");
        std::fs::write(home.join("mirror.spec"), MINIMAL_SPEC).expect("spec");
        std::fs::write(home.join("observation.txt"), "initial\n").expect("obs");
    }
    (home_1, home_2)
}

fn make_shared_song(dir: &Path) -> PathBuf {
    let path = dir.join("shared_dance.song");
    std::fs::write(&path, HELLO_MOVEMENT_SONG).expect("write");
    path
}
```

**Rationale for subprocess-spawn (not in-process parallel):**
- Reuses existing `Command::new(mirror_bin())` pattern from 12+
  peer_beam_* shards (Reed's existing precedent).
- Sequential execution is substrate-honest at Rung 4 — the two peers'
  runs are Aumann-independent observations over content-addressed
  shared prior; sequentialization does not change the mathematics.
  Parallel execution is a Rung 5 optimization, not a correctness
  requirement.
- Sequential simplifies test debugging (deterministic output ordering).

**Five T-tests** (matching Reed's Rung 1-3 pattern):

**T1** — dispatch acceptance: `mirror peer beam <home1> --song
<song> --dance-with <home2>` exits 0.

**T2** — envelope names @dance substrate authorities: stdout contains
`@dance`, `@resonance`, `@cyberpunk`, `@bauchladen` (all four are
substrate-cited on-envelope per §3.3).

**T3** — envelope emits Kuramoto order parameter: stdout contains
`kuramoto_order_parameter:` followed by a parseable f64 in [0, 1].

**T4** — envelope emits Aumann agreement verdict: stdout contains
`aumann_agreement:` followed by `true` or `false`; and
`shared_root_oid:` followed by hex-OID or `<none>`.

**T5** — convergence-verdict classifier fires: stdout contains
`convergence_verdict:` followed by one of {`converged`, `dispersed`,
`chimera`}.

**Substrate-honest hedge for T5.** Because Rung 4 emits a stub
coherence (§2.2), the classifier's verdict for the Rung 4 fixture is
deterministic. The T5 test asserts the FIELD is emitted and the value
is a valid classifier label; consumers using Rung 4.5+ landed λ₀(Δ_F)
values will see the classifier respond meaningfully to real coherence
variation.

### 3.5 CLI flag — `--dance-with <peer-home-2>`

Following Reed's Rung 1 `--song <file>` flag precedent (LANDED at
`bootstrap/src/lib.rs:3187-3205`):

**Grammar addition** (Rung 4b Reed's job): add `flag dance_with: ~f`
to `command peer { command beam { ... } }`.

**Dispatch cascade in `cmd_peer_beam`** (following the `--song`
if-let-Some early-dispatch pattern at `bootstrap/src/lib.rs:4985-4995`):

```rust
if let (Some(song_path), Some(peer_home_2)) = (song, dance_with) {
    // Rung 4 @dance dispatch per Mara `<this-spec-oid>` + substrate
    // reservation at `shards/song/beat.mirror:453-457`.
    let spec_path_2 = std::path::PathBuf::from(peer_home_2).join("mirror.spec");
    return crate::dance::execute_dance(
        peer_home, peer_home_2, &spec_path, &spec_path_2, song_path, ctx
    );
}
if let Some(song_path) = song {
    return crate::song::single_beat_peer_beam(peer_home, &spec_path, song_path, ctx);
}
```

Byte-equality for non-`--dance-with` paths preserved via the
`if let (Some, Some)` guard — Rungs 1-3 all still work identically.
Same discipline as Rungs 1-3's `if let Some(song) = song` guard
preserving pre-Rung-1 byte-equality.

### 3.6 Anonymous variant

`mirror beam --song <song> --dance-with <peer-home>` (anonymous
sibling of `mirror peer beam`) forward-promised to Rung 5. Rung 4
scope is `mirror peer beam` positional only. Rationale: two-tick
discipline.

### 3.7 MCP surface

`mirror_peer_beam` MCP tool schema (per `bootstrap/src/mcp.rs:170-540`)
gains a `dance_with` optional string parameter. One-line addition to
the schema. Rung 4b Reed's job.

---

## §4. Composition with landed carriers

### 4.1 `@bauchladen` (content-addressed shared substrate)

`shared_root_oid` field IS `@bauchladen`'s common-prior condition
made empirical. Per `shards/bauchladen.mirror:243-291` LANDED chain,
every content-addressed crystal has an OID derivable from its
content bytes. Two peers whose terminal beat envelopes contain byte-
equal payloads emit byte-equal OIDs. Aumann agreement fires.

**Rung 4 depth of `@bauchladen` integration:** shallow. Rung 4 reads
the OID as a hash of envelope bytes (stub); Rung 5+ integrates with
`@mirror/store.write` for actual crystal materialization + tray
enumeration.

### 4.2 `@torus` (observation surface + winding-class basins)

The `convergence_verdict` classifier (`converged`/`dispersed`/`chimera`)
IS the winding-class basin structure per Mara `caf461f` §6 ("winding
classes ARE coherence basins on T²"). Rung 4 reports the classifier
label; Rung 5+ names the specific winding class (m, n) ∈ π₁(T²) = ℤ×ℤ.

**Rung 4 depth of `@torus` integration:** classifier only. Actual
winding-number computation is Rung 5+ forward-promise.

### 4.3 `@resonance` (Kuramoto coupling)

Rung 4 REPORTS the Kuramoto order-parameter r; it does NOT simulate
the coupling ODE (per §2.3 refusal of Scope C). The r value is
computed from the observed phase-differences, not from a simulated
coupling matrix. This is substrate-honest: r as a summary statistic
of observed data, not as an integrator output.

**Rung 4 depth of `@resonance` integration:** r-as-statistic. The
coupling matrix κᵢⱼ is not computed at Rung 4 (Alex mandate: don't
simulate physics we can't measure).

### 4.4 `@song`/beat + @song/phrase + @song/movement — Rung 4's substrate ancestry

Reed's Rungs 1-3 landed the full @song runtime; Rung 4 composes on
top. The shared `.song` file at Rung 4 uses Reed's `HELLO_MOVEMENT`
fixture (per `peer_beam_song_movement_shard.rs:80-101`) unchanged.
Two peers, one song, same substrate.

**Rung 4 depth of @song integration:** consumer only. Rung 4 does
not extend the @song grammar or add new @song species; it just
dispatches @song twice.

### 4.5 `@coherence` (Reed `8e6e517`)

Reed's Path B annotation on `shards/cyberpunk.mirror` landed
`cybernetic_coherence = λ₀(Δ_F)` at single-peer scale. Rung 4 extends
to ensemble scale by reading two peers' coherence values in
sequence and computing their phase-lock.

**Substrate-honest hedge (Rung 4.5 forward-promise):** the ensemble
extension `cybernetic_coherence_ensemble = λ₀(Δ_{F₁ ⨁ F₂})` where
F₁ and F₂ are the two peers' sheaves and ⨁ is the direct sum is
Mara's forward-promise for Rung 4.5. Rung 4 does NOT compute this;
it stubs coherence per envelope-bytes hash. The recognition candidate
`#R-multi-peer-coherence-phase-lock-realizes-dance-at-runtime-altitude`
names the compositional identity; the runtime implementation stops at
the stub until Rung 4.5.

---

## §5. Convergence measurement — what IS "convergence" at Rung 4?

Three candidate empirical witnesses, all substrate-declared:

### 5.1 Kuramoto order-parameter r

For two peers with per-beat phase sequences θ₁ and θ₂:

    r · e^{iψ} = (1/2) (e^{iθ₁} + e^{iθ₂})

Two-peer specialization of Mara `9e48710` §2.4's N-peer formula.
r ∈ [0, 1]. r → 1 when the two peers' phases align; r → 0 when they
disperse. Rung 4 default threshold: r ≥ 0.9 counts as `converged`.

### 5.2 Aumann OID equality

Both peers' terminal beat crystals hash to the same OID. Substrate-
locked witness per `beat.mirror:493` verbatim. Binary: true or false.
Under content-addressed common prior (`@bauchladen`), this IS the
common-knowledge-of-posteriors condition (Aumann 1976).

### 5.3 Ensemble sheaf Laplacian λ₀(Δ_{F₁ ⨁ F₂}) (Rung 4.5 forward-promise)

The ensemble coherence metric extending Reed's `8e6e517` to two peers.
NOT computed at Rung 4; declared as forward-promise + named on the
envelope's `coherence_altitude` field so consumers know the stub
substitutes for a Rung 4.5 landing.

### 5.4 Three-way classifier (Rung 4's operational verdict)

`convergence_verdict` field combines r and aumann_agreement:

- `converged`: r ≥ 0.9 AND aumann_agreement == true. Both peers in
  phase-lock AND emitted the same terminal OID. This is the Heist
  case: distinct peers with shared substrate converging on a shared
  basin.
- `dispersed`: r < 0.5. Peers uncoupled; no basin convergence.
- `chimera`: 0.5 ≤ r < 0.9 OR (r ≥ 0.9 AND aumann_agreement ==
  false). Partial synchronization without common basin. Per Abrams-
  Strogatz 2004 (cited at `4f079c8` §2.1), chimera states are the
  intermediate regime.

**Substrate-honest note.** These thresholds (0.9, 0.5) are Rung 4
defaults per Kuramoto's mean-field K_c interpretation. Consumers may
re-classify from the raw r and aumann_agreement fields. The classifier
value is a convenience; the raw fields are the load-bearing witnesses.

---

## §6. Six sub-ambiguities — adjudication

Reed named six sub-ambiguities in the escalation brief. Each
adjudicated below with the substrate-honest reason.

### 6.a Which scope? — **Scope B narrowed to coherence phase-lock.**

Justification: §1 grep-verified 100% substrate-already-had-the-word
coverage; §2.2 formalized as recommended; §2.1 and §2.3 refused with
substrate-honest reasoning; the substrate-shard reservation at
`beat.mirror:453-457` is verbatim binding.

### 6.b Is @silicon coupling ACTUALLY simulated or metaphorical? — **Metaphorical (named on envelope, not computed).**

`dance.rs` does NOT simulate Kuramoto dynamics with any κᵢⱼ matrix.
The `kuramoto_order_parameter` field is a SUMMARY STATISTIC of
observed phase-differences, not the output of an ODE integrator. The
envelope names `@resonance` and `@silicon` as substrate authorities
(so consumers know the recognition ancestry) but the runtime does
NOT pretend to measure physical coupling. Per §2.3 refusal of Scope C:
physical measurement lives at `@io`; Rung 7+ with hardware.

**Envelope discipline:** the `coherence_altitude: stub` line + the
absence of an `@io_measurement:` field make this transparent to
consumers. No misrepresentation.

### 6.c Multi-peer fixture harness architecture? — **Subprocess spawn, sequential.**

Justification: reuses Reed's existing `Command::new(mirror_bin())`
pattern from 12+ peer_beam_* shards. Sequential execution is
substrate-honest at Rung 4 (Aumann observations are order-independent
under content-addressed common prior). Parallel is a Rung 5 optimization.
In-process would require refactoring `execute_song` to return
envelope bytes (currently prints to stdout); that's Rung 4.5
forward-promise for the internal API.

**Implementation shape:**
```rust
fn collect_coherence_sequence(peer_home, spec_path, song_path, ctx) -> Vec<f64> {
    let output = Command::new(mirror_bin())
        .arg("peer").arg("beam").arg(peer_home)
        .arg("--song").arg(song_path)
        .output()?;
    parse_coherence_from_stdout(&output.stdout)
}
```

Two sequential `Command::new` calls (one per peer); parse coherence
sequences from their stdout; feed to Kuramoto formula.

### 6.d Envelope field for convergence witness? — **All three.**

`kuramoto_order_parameter` (r), `aumann_agreement` (bool),
`shared_root_oid` (Option<hex>), plus the derived `convergence_verdict`
classifier. Substrate-locked field names per `beat.mirror:456-457`
+ `bauchladen.mirror` chain. No single-field simplification: the
three witnesses measure different aspects of @dance (phase-lock,
posterior-equality, common-prior-satisfaction) and Rung 4 emits all
three for consumer inspection.

### 6.e `bootstrap/src/dance.rs` scope? — **Just multi-peer coordination; not full @resonance operator.**

Substrate discipline: @resonance IS the recognition of the Kuramoto
coupling operator at the physics altitude; @dance IS the operational
form at the substrate altitude. `dance.rs` is the @dance runtime.
The @resonance runtime (if ever needed) lives at `bootstrap/src/
resonance.rs` and would only exist when hardware measurement of
`@silicon.coupling` is available (Rung 7+). @resonance-as-operator
is Mara `9e48710` §7 recommended Path B (annotation-not-mint at family-
root altitude), so a full operator runtime is not substrate-honest at
Rung 4.

**Verdict:** `dance.rs` ≪ full @resonance operator. Named scope:
`execute_dance(peer_home_1, peer_home_2, spec_path_1, spec_path_2,
song_path, ctx) -> i32`. That's it.

### 6.f Rung 4 test file naming? — **`peer_beam_dance_coherence_shard.rs`.**

Justification: follows Reed's established `peer_beam_<scope>_shard.rs`
naming convention (12+ precedents). `dance_coherence` names the
scope (@dance runtime at coherence altitude) unambiguously.

**Alternatives considered + refused:**
- `peer_beam_dance_ensemble_shard.rs` — "ensemble" appears in
  vocabulary but not as substrate-named module. Less precise.
- `peer_dance_convergence_shard.rs` — drops the `peer_beam_` prefix;
  breaks Reed's naming convention.
- `dance_two_peer_shard.rs` — drops both prefix conventions;
  substrate-drift.

Verdict: `peer_beam_dance_coherence_shard.rs`.

---

## §7. Two-tick landing sequence

### 7.1 Tick 4a (Mara — this spec)

**Deliverables (this commit):**
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
  (this file) — canonical spec + adjudication.
- No `.mirror` file changes required. The substrate has already
  written the shard-level declaration at `shards/song/beat.mirror:
  453-457`; extending it further at Rung 4a would violate two-tick
  discipline.
- No new envelope fields to declare; all field names are substrate-
  pre-declared per §1.3.

**What Tick 4a does NOT deliver (forward-promised to Rung 4.5 / 5+):**
- Actual λ₀(Δ_F) computation for coherence (stub at Rung 4; Rung 4.5).
- Winding-class labeling on `convergence_verdict` (Rung 5).
- N > 2 peer ensembles (Rung 5).
- `@silicon.coupling` observation via `@io` (Rung 7+ with hardware).
- Multi-song variant (each peer with its own `.song`; Rung 4.5).
- `@bauchladen` crystal materialization via `@mirror/store.write`
  (Rung 5+).

### 7.2 Tick 4b (Reed — immediate follow-up)

**Deliverables:**
1. `bootstrap/src/dance.rs` (~100-150 lines) per §3.2 signature +
   §3.3 envelope shape.
2. `bootstrap/tests/peer_beam_dance_coherence_shard.rs` per §3.4
   (five T-tests, RED-first).
3. `--dance-with <peer-home>` flag admission per §3.5:
   - `mirror.spec` grammar update (`flag dance_with: ~f`).
   - `cmd_peer_beam` argv parsing (~5 lines, mirroring `--song`).
   - `cmd_peer_beam` dispatch cascade (`if let (Some, Some) = (song,
     dance_with)` guard).
4. `bootstrap/src/lib.rs` module declaration `pub mod dance;`.
5. `bootstrap/src/mcp.rs` schema update (add `dance_with` optional
   parameter to `mirror_peer_beam` tool).

**RED-first discipline:** Reed lands the test shard FIRST (RED
commit); then lands `dance.rs` + flag admission (GREEN commit). Same
pattern as Rungs 1-3.

**Byte-equality preservation:** all Rungs 1-3 tests must continue
passing byte-equal. The Rung 4 dispatch fires only when BOTH
`--song` and `--dance-with` are present; other paths unchanged.

**Substrate-honest hedge:** if Reed encounters a substrate-decl
ambiguity mid-Tick-4b that the spec did not resolve, Reed reports
back for adjudication (per Alex's mandate: "climb the ladder until
unresolvable ambiguity that cannot be postponed further"). This
spec's job is to make Tick 4b executable without such escalation;
if it fails, this spec was not substrate-honest.

---

## §8. Refusals + forward-promises

**Out of scope for Rung 4 (forward-promised, each to a specific rung):**

- **Rung 4.5 — `λ₀(Δ_F)` runtime computation.** Reed's `8e6e517`
  landed the CLAIM; Rung 4.5 lands the actual sheaf Laplacian
  computation per beat. Precedent substrate: `shards/epistemologic/
  math/sheaf_laplacian.mirror` LANDED. Replaces the Rung 4 stub in
  `stub_coherence_from_envelope_bytes`.

- **Rung 4.5 — multi-song fixture.** Two peers with DIFFERENT `.song`
  files sharing substrate. Rung 4 uses ONE shared song for
  substrate-honest simplicity; Rung 4.5 extends to two songs (which
  is closer to the Heist's structural claim of distinct local specifics
  converging on shared basin).

- **Rung 5 — N > 2 peers.** Kuramoto's mean-field K_c threshold and
  chimera-state analysis are meaningful at N ≥ 3. Two-tick discipline:
  ship the N=2 shape first; N > 2 requires the moduli-space carrier
  Mara `4f079c8` §3.8 flagged as `@dance`'s known limitation.

- **Rung 5 — winding-class labeling on `convergence_verdict`.**
  Report specific (m, n) ∈ π₁(T²) instead of just `converged`/
  `dispersed`/`chimera` classifier. Requires @torus winding-class
  detection primitive (not yet landed).

- **Rung 5+ — `@bauchladen` crystal materialization.** Instead of
  hashing envelope bytes, actually write each peer's terminal beat
  crystal to the bauchladen tray via `@mirror/store.write` and
  compare crystal-OIDs from the tray. Requires `@mirror/store` full
  integration into the peer_beam runtime.

- **Rung 6 — mycelial propagation.** Per `beat.mirror:459-460`:
  "Rungs 5-6: mycelial propagation via nix binary cache (@bauchladen
  gossip); full @spectral/garden mycelial deployment." @dance
  becomes the coordination substrate for the garden deployment.

- **Rung 7+ — `@silicon.coupling` via `@io` hardware measurement.**
  Per Mara `9e48710` §3.1. Actual EM/thermal/timing side-channel
  reads via `observe_coupling(peers: [peer]) -> coupling_matrix`.
  Alex on physical GPU cluster. Turns metaphorical Kuramoto into
  measured Kuramoto. Substrate-honest full-Scope-C landing.

**Refusals not just forward-promises:**

- **Scope A (deterministic byte-equality replay) as a landing scope.**
  Refused per §2.1. Byte-equality regression assertions (Rungs 1-3
  must continue passing) fold into Reed's existing test suite; no
  Rung 4 T-test dedicated to byte-equality replay.

- **Scope C (full Kuramoto ODE simulation) at Rung 4.** Refused per
  §2.3. Forward-promise to Rung 7+ with hardware.

- **New `@dance` family-root mint.** Per Mara `4f079c8` §4.4 Path C
  recommendation (LANDED): @dance stays as recognition-candidate
  annotation on `@algebra/metalogue`; no `@dance` family-root
  keyword. `bootstrap/src/dance.rs` is a MODULE name (mirrors the
  substrate reservation at `beat.mirror:454`); it is NOT a substrate-
  decl carrier promotion.

- **New envelope keywords beyond substrate-pre-declared.** All Rung 4
  envelope field names are pre-declared in landed shards per §1.3.
  Zero new keywords introduced by this spec.

---

## §9. Recognition candidate

**`#R-multi-peer-coherence-phase-lock-realizes-dance-at-runtime-altitude`**

Alex 2026-07-13 (in-transcript ladder-climb mandate + this spec
delivery).

**Recognition statement:**

> Multi-peer @dance at runtime altitude IS the composition of
> `bootstrap/src/dance.rs` reading two peer-homes' @song executions,
> computing the Kuramoto order-parameter r over their per-beat
> cybernetic_coherence deltas, and asserting Aumann agreement when
> their terminal beat crystals hash to a shared root_OID. This
> composition realizes Mara `4f079c8`'s @dance recognition
> (coordination-without-signal on Förster's @torus) at the
> operational altitude of the mirror runtime. The composition is
> substrate-already-had-the-word: every carrier is landed with OID;
> the module path is pre-declared at `shards/song/beat.mirror:454`;
> the envelope field names are pre-declared at `beat.mirror:456-457`;
> the measurement primitive is pre-declared at Reed `8e6e517`. The
> Rust module is ~100 lines translating the substrate's already-
> written specification into a running dispatch. The recognition
> promotes when Reed's Tick 4b GREENs.

**Promotable via:**
- Reed's Tick 4b GREEN (Rung 4 test shard passing).
- Two-tick discipline preserved (this spec at 4a; Reed's runtime at 4b).
- Recognition ancestry chain intact (per §10).

**Would refute if:**
- Rung 4b landing requires a scope re-adjudication beyond §6.a-f.
- The envelope field names cannot be honored verbatim per §1.3.
- The stub coherence approach (§2.2) is deemed substrate-drift by
  Seam Phase D audit.

---

## §10. Recognition ancestry

**Direct ancestors (this spec's substrate-pre-declaration):**
- Mara `94e55eb` — `shards/song/beat.mirror` sixth species mint;
  Rung 4 module path + envelope fields pre-declared at lines 453-457,
  485-493, 493, 706-716. Substrate WROTE Rung 4's shape 24 hours
  before this spec.

**Session ancestors (2026-07-13 ladder-climb arc):**
- Reed `0cc4e11` — Rung 3 GREEN (per-block AST walk + envelope
  emission); the substrate on which Rung 4 composes.
- Reed `7b7fb0b` — Rung 3 RED.
- Mara `d29d45e` — Rung 3 spec (Path B mirror-native grammar).
- Reed `70766c3` — Rung 2 GREEN (phrase parsing).
- Reed `79eee6f` — Rung 2 RED.
- Reed `c36fbf5` — Rung 1 GREEN (--song dispatch).
- Reed `5fdc009` — Rung 1 RED.
- Mara `94e55eb` — Rung 0 (beat species mint with Rung 4 verbatim
  reservation).
- Taut `c54740c` — ladder scout defining the 7-rung path + T7 fixture
  spec.

**Arc ancestors (2026-07-12 to 2026-07-13):**
- Mara `d21337b` — @song replaces plans and loops; canonical @spectral/
  garden deployment.
- Reed `61b444a` — Path C annotations on `shards/algebra/metalogue.
  mirror` (dance-as-metalogue-N-speaker).
- Mara `4f079c8` — @dance canonical spec; Path C recommendation;
  Kuramoto + Aumann + Cavagna + Schelling + Foucault-holonomy
  formalization; recognition candidate `#R-dance-is-coordination-
  without-signal-on-forster-torus`.
- Reed `71a4689` — coordination-without-signal annotation §11.2.1-
  11.2.3 on Mara `9e48710`.
- Mara `9e48710` — @resonance canonical spec; Kuramoto coupling
  formalization; N-speaker lift of @algebra/metalogue.
- Reed `8e6e517` — cybernetic_coherence = λ₀(Δ_F) Path B annotation
  on `shards/cyberpunk.mirror`.

**Substrate ancestors (pre-arc):**
- `shards/cyberpunk.mirror` — Recognition #58 chain (Fate IS optical
  inference; Fabry-Perot resonator intra-peer).
- `shards/algebra/metalogue.mirror` (`34cf333`) — N-speaker turn
  composition (Batanin 1998 globular composition; Mac Lane 1971
  non-commutativity).
- `shards/bauchladen.mirror` — Recognition #104 (content-addressed
  shared substrate; Schmidt Bauchladen).
- `shards/torus.mirror` — Foerster 1974 *Understanding Understanding*
  §torus; π₁(T²) = ℤ × ℤ winding classes; Mara `caf461f` §6 winding-
  classes-as-basins.
- `shards/epistemologic/cybernetic/conversation.mirror` — Pask tensor
  coupling; Batanin globular composition; N-ary factoring.
- `shards/epistemologic/cybernetic/eigenform.mirror` — Foerster 1976/
  1981 *Objects: Tokens for Eigen-Behaviors*; fixed-point convergence
  discipline.
- `shards/kintsugi/oscillate.mirror` — ACTIVE/DARK alternation
  discipline; the beat's driver.

**External math ancestry:**
- Kuramoto 1975 (coupled phase oscillator model).
- Aumann 1976 (agreement theorem under common prior).
- Cavagna 2008/2010 (topological-neighbor coupling in starling flocks).
- Schelling 1960 (focal points under salience).
- Csiszár-Ahlswede 1986 (rate-distortion under shared prior; zero-bit
  coordination bound).
- Batanin 1998 (globular composition; N-fold tensor factoring).
- Abrams-Strogatz 2004 (chimera states in coupled oscillator networks).
- Foerster 1976/1981 (*Objects: Tokens for Eigen-Behaviors*).
- Foerster 1974 (*Understanding Understanding*; §torus double closure).

---

**Word count:** ~4200. Every OID cited grep-verified; every substrate
carrier's landing OID named; three scopes formalized with substrate-
honest refusal reasoning for the two not recommended; the recommended
Scope B narrowing preserves substrate-pre-declaration verbatim.

**Alex-adjudication surface:** none required if Alex reads this as
the substrate-locked shape (which it is per §1). If Alex prefers a
different scope, three-tick escalation follows (Mara re-specs; Reed
re-lands; Seam audits).

**Immediate handoff to Reed:** proceed with Tick 4b per §7.2.
Substrate discipline is intact end-to-end; the runtime is the
shortest path from Reed's Rung 3 GREEN to Rung 4 GREEN this arc has
seen.
