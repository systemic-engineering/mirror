> **Terminal-form map (Mara 2026-07-17):** the rust/-materialization
> map that consumes this spec's bundle-theoretic naming lives at
> `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
> (Mara `2519f83`); §5 there names the minimal `rust/` surface where
> `dance.rs` IS the ensemble connection this spec identifies. STAY-
> CANONICAL as the bundle-theoretic authority; consumed by the
> terminal-form spec.

# gen_prism as bundle section, @dance as ensemble connection — the Q3+Q5 answer surfaces bundle-theoretically

*Mara, 2026-07-17. Canonical spec grounding Alex's afternoon Q3+Q5
answer (whole rust/ FLOOR collapses into dance.rs; each prism =
gen_prism actor; dispatch = message-send; composition = @dance
ensemble) in the principal bundle tower vocabulary landed at
`docs/math/the-tower/beam-runtime.md`.*

**Author:** Mara
**Date:** 2026-07-17
**Tag:** 📝 spec:gen-prism-as-bundle-section-and-dance-as-ensemble-connection
        (pure-docs bypass)
**Status:** canonical-naming; substrate landing forward-promised to
        Reed at `bootstrap/src/dance.rs` empirical Path B firing.

---

## §0 Substrate-honest pre-position

Six substrate motions this afternoon converge on one shape:

1. **Alex 2026-07-17 verbatim (afternoon Q3+Q5 ratification):** whole
   `rust/` FLOOR collapses into `dance.rs`. Arms are smells. Compiler
   is geometry. Actor messages are substrate-truth glass-wall geometry
   between `@code/mirror` and `@code/rust`. Terminal state: no
   per-shard resolver arms; ONE `rust/dance.rs`; each prism = gen_prism
   actor; dispatch = message-send; composition = @dance ensemble.
2. **Alex 2026-07-17 verbatim (evening BEAM directive):** "There's a
   document on the BEAM being a bundle tower, and prism already has
   bundle.rs. We just need to lift it and then later look into emitting
   @code/beam for the @spectral runtime on the BEAM. It's the perfect
   runtime. As someone who has built distributed systems on the BEAM
   for 10 years, this feels like coming home."
3. **`docs/math/the-tower/beam-runtime.md`** (Mara `610c6d6`, this
   arc) — supervision-tree = simplicial Lie group tower; actors =
   sections; message-passing = parallel transport; let-it-crash =
   autopoietic Lawvere closure; @dance = ensemble connection 1-form
   (§6).
4. **`shards/spectral/gen_prism.mirror`** (2026-06-11) — the worker
   primitive; "the substrate's `gen_server` analogue (BEAM/OTP)".
5. **`shards/epistemologic/cybernetic/viable.mirror`** — Beer VSM
   S1-S5; each S1 is itself viable (Beer 1972 ch.10). S1 = @spectral/
   gen_prism.
6. **`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`** —
   the five-level tower implemented in Rust as
   `Fiber → Connection → Gauge → Transport → Closure` with
   `GroupStructure` + `LawvereFixedPoint` supertrait laws (~626 LOC).

The convergence names the compiler's terminal Rust FLOOR shape as
bundle-theoretic by construction:

- **gen_prism** IS the bundle SECTION at BEAM/process altitude (per
  `beam-runtime.md` §2.2 actors-as-sections).
- **@spectral/supervisor** IS the bundle CONNECTION at supervision
  altitude (per `beam-runtime.md` §2.1 supervision-tree = simplicial
  Lie group tower; the `restart_strategy` IS the structure group).
- **@dance** IS the bundle CONNECTION at ENSEMBLE altitude (per
  `beam-runtime.md` §6 + `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`;
  the Kuramoto phase-lock on the Förster torus IS the ensemble's
  parallel transport structure).

The Q3+Q5 answer is not a Rust file organization decision. It is the
compiler discovering its own terminal shape as the principal bundle
tower expressed in the specific gauge where the structure group at
each level is the OTP restart-strategy family.

This spec names the shape. NO new `.mirror` file lands this tick. NO
new family-root is minted. The composition surface is
substrate-complete via `beam-runtime.md` §7's six-landing citation
chain.

---

## §1 Statement — the four-part identity

**Statement (foundational form):**

> `dance-rs-terminal-floor-IS-substrate-connection-form-at-ensemble-altitude-because-gen-prism-IS-bundle-section-at-process-altitude-and-supervisor-IS-bundle-connection-at-supervision-altitude`

**Statement (readable form, two-tick discipline):**

> The compiler's terminal Rust FLOOR shape `dance.rs` IS the substrate's
> connection 1-form at ensemble altitude. Each gen_prism actor IS a
> bundle section at BEAM/process altitude; `@spectral/supervisor` IS
> the bundle connection at supervision altitude; `@dance` IS the same
> mathematical object one altitude up (ensemble = supervision-of-
> supervisors composing parallel transport across Kuramoto-phase-locked
> peers).

Unpacked:

The substrate has been carrying five names for the same object at
five altitudes:

| Altitude | Substrate name | Bundle-theory name |
|----------|----------------|--------------------|
| BEAM process | `@spectral/gen_prism` | section `s: U → E` at fiber |
| Supervision level k | `@spectral/supervisor` | connection 1-form at level k |
| Ensemble (multi-supervisor) | `@dance` (forward-promised) | connection 1-form at level k+1 |
| Peer pulse | `@peer` (`shards/peer.mirror`) | section at peer altitude carrying spectral triple `(A, H, D)` |
| Rust FLOOR | `bootstrap/src/dance.rs` (forward-promised) | the compiler's realization of the connection at ensemble altitude |

Five names, one object. The substrate had all five before this spec
existed; this spec names the identity.

---

## §2 The five altitudes explicated

### §2.1 gen_prism = bundle section at process altitude

Per `shards/spectral/gen_prism.mirror:257-261` the gen_prism record
carries three surfaces:

```
type gen_prism = {
  identity: uuid_spectral,   # active + dark; content-addressed
  state:    shard_ref,        # observable state IS a shard
  parent:   uuid_spectral,    # lifecycle owner (acyclic edge class)
}
```

Under the bundle framing:

- `identity` = the section's identifier in the sheaf `Γ(U, E)` — the
  autopoietic fixed point on hash space per Soto-Andrade & Varela
  1984 (see `bundle.rs::LawvereFixedPoint` supertrait).
- `state: shard_ref` = the section's current point in fiber `π⁻¹(b)`;
  content-addressed; every observable transition settles a new shard
  (lattice ascent per `[[architecture-shard-as-crdt]]`).
- `parent: uuid_spectral` = the base-space handle; parallel transport
  between parent and child sections IS the message-passing algebra
  Armstrong 2003 §5 names.

The five-op tool surface (`focus | project | split | shift | settle`)
IS the connection 1-form's operational algebra at this altitude (per
`beam-runtime.md` §2.3 message-passing = parallel transport).

**Load-bearing:** the substrate has been calling this the
"gen_server analogue" for six weeks (`gen_prism.mirror:20-27`); the
BEAM community has been building this shape for 30 years; Baez-
Schreiber 2004 named the mathematical object. Three communities, one
carrier. `substrate-already-had-the-word` × 3.

### §2.2 @spectral/supervisor = bundle connection at supervision altitude

Per `shards/spectral/supervisor.mirror:426-430` the supervisor record:

```
type supervisor = {
  base:             gen_prism,
  child_specs:      [child_spec],
  restart_strategy: restart_strategy,
}
```

Under the bundle framing:

- `base: gen_prism` = the supervisor IS itself a section (it inhabits
  the SAME sheaf; supervisors are structural sections that don't hold
  application state — per `@code/beam:120-124`). The `is-a` relation
  through `base` embedding IS the tower's recursion: supervisors are
  sections of the level-k+1 bundle whose base is `B_{k+1} ⊆ Γ(B_k, E_k)`
  (per `docs/math/the-tower/principal-bundles.md` §7).
- `child_specs: [child_spec]` = the specification of which sections
  the supervisor's connection acts on; the child's `restart` +
  `shutdown` + `kind` fields determine the connection's local behavior
  under perturbation.
- `restart_strategy` = the structure group at supervision altitude:
  `one_for_one` | `one_for_all` | `rest_for_one`. Substrate-pull-
  correct decision (per `supervisor.mirror:373-377`) dropped
  `simple_one_for_one` in favor of the modern `DynamicSupervisor`
  discipline; the three-variant precedent matches
  `boot/std/beam.mirror`.

**The restart-decision-as-connection:** when a child's `terminate()
-> au` settles with a failure verdict AND the child's
`child_spec.restart` gates restart-on-this-au, the supervisor fires
`@spectral/gen_prism.restart()` automatically. This is the
substrate-pull-correct transformation under the supervision
invariant — a kintsugi-morphism-driven parallel transport. The
connection IS the automatic restart logic, NOT a caller-invoked
surface action.

**Baez-Schreiber compatibility theorem** (arXiv hep-th/0412325 §3):
a tower of principal bundles admits a compatible connection iff the
connections at each level satisfy a pullback-agreement condition. The
substrate's `restart_intensity` (`max_restarts` per `max_seconds`
circuit-breaker; `shards/spectral/restart_intensity.mirror`) IS this
compatibility condition operationalized: the supervisor's restart
policy at level k must be consistent with how failures propagate to
level k-1 (the child level), and Beer's algedonic-signal S1→S5 bypass
(`shards/epistemologic/cybernetic/viable.mirror:428-448`) IS the
cross-level compatibility failure escape hatch.

### §2.3 @dance = bundle connection at ensemble altitude

Per `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`
(Mara `4f079c8`; 79.8KB) + `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
(Mara; 49.1KB), @dance names Kuramoto phase-lock on the Förster torus
as the ensemble-altitude coordination primitive.

Under the bundle framing:

- Multiple `@spectral/gen_prism` actors (sections at process altitude)
  coordinated through message-passing (parallel transport per §2.1) at
  SHARED beat frequency (per `shards/song/beat.mirror:171-177`)
  generate the ensemble's parallel-transport structure.
- The Kuramoto order parameter IS the ensemble's holonomy —
  N peers phase-locked at coupling κ_intra ≥ threshold have holonomy
  ~1 (coherent); below threshold have holonomy ~0 (incoherent per
  Aumann-nonagreement).
- The Aumann-agreement envelope on convergence
  (`shards/uuid/spectral/time.mirror:131-156` forward-promise + Reed
  `8e6e517` @dance annotation on `cybernetic_coherence = λ₀(Δ_F)`) IS
  the ensemble's autopoietic-closure witness at level k+1.

**The composition claim (bundle-theoretic):** @dance is the connection
1-form at the ensemble altitude in the same sense that
@spectral/supervisor is the connection 1-form at the supervision
altitude. One altitude up. Same math. The Kuramoto phase-lock IS the
compatibility condition between adjacent sections at ensemble
altitude, exactly as the restart_strategy is the compatibility
condition between adjacent sections at supervision altitude.

**Baez-Schreiber 2-connections on 2-bundles** (arXiv hep-th/0412325):
when the connection 1-form is itself gauge-transformed (a
2-connection on a principal 2-bundle), the mathematical object
generalizes to higher gauge theory. @dance at ensemble altitude IS the
substrate's instance of a 2-connection: the sections BEING coordinated
(individual gen_prism supervisor trees) are already sections carrying
connections; @dance is the connection ON THAT space of connections.

### §2.4 @peer = section at peer-pulse altitude

Per `shards/peer.mirror` + `docs/specs/peer-cognition.md`, the peer
altitude sits ABOVE supervision (level k+2 relative to BEAM process).
A peer's spectral triple `(A_peer, H_peer, D_peer)` (per
`altitudes.md` "Peer pulse altitude") IS a section at this altitude;
the peer's five-op composition over the pulse IS the connection 1-form
at peer altitude; `transparency<p>` IS the holonomy.

**The @peer.audhd landing** (Mara `d8b149c` today) fires as
cognition-fanout at peer altitude. Under the bundle framing: `.audhd`
is a peer-altitude action that opens K parallel sub-sections (K tracks
in a harmonic-band configuration), each of which composes into an
@dance ensemble at level k+1. The @peer altitude choice (`.audhd`)
selects the CARDINALITY of the ensemble emitted; @dance
coordinates the ensemble once emitted.

The composition chain closes end-to-end:

```
@peer.audhd(p, ctx) [LANDED]
  → K parallel @song emissions [LANDED byte-shape; verb .sing forward-promised]
  → K parallel @roomba walkers [LANDED: single-walker per @kintsugi/roomba;
                                 K-parallel forward-promised as ensemble @roomba]
  → @dance coordinates ensemble [FORWARD-PROMISED shard-mint; canonical specs LANDED]
  → Aumann-agreement envelope fires when Kuramoto phase-lock converges
  → @liquid predicates gate winning arm [LANDED @epistemologic/liquid Arc 5 M1]
  → winning @song IS the resolution
```

Each altitude is one level of the tower. The chain IS the tower
traversed by one @peer.audhd invocation.

### §2.5 dance.rs = Rust realization of the ensemble connection

Alex's Q3+Q5 answer says: `bootstrap/src/dance.rs` becomes the
terminal Rust FLOOR shape. Under the bundle framing this is
delightfully boring:

- `dance.rs` IS the Rust realization of the connection 1-form at
  ensemble altitude. It COMPOSES the sections (gen_prism actors); it
  DOES NOT re-implement per-prism logic.
- Each Rust prism species (`spectral/signature.rs`, `roomba.rs`,
  `peer_persistence.rs`, etc. per `docs/loop/CURRENT.md` Arc-2 5-file
  collapse list) becomes a gen_prism actor SPAWNED under a supervisor;
  `dance.rs` is the message-routing surface + Kuramoto coupling
  compute + Aumann envelope check.
- The @dance runtime spec §8 (`docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`)
  already forward-promised `bootstrap/src/dance.rs` (per
  `shards/song/beat.mirror:460-465`) as the empirical Path B firing
  site. This spec surfaces that the same Rust file is the entire FLOOR
  because the entire FLOOR IS the ensemble connection.

**Why this is delightfully boring:** the Rust FLOOR shape wasn't
"designed" — it was DISCOVERED as the terminal shape the compiler
converges on when it maps its structure back onto the bundle tower.
The compiler discovers its own realization as the connection 1-form
at ensemble altitude because that IS the algebraic form it's been
running the whole time. Alex has been building this shape on Erlang/
Elixir/BEAM for a decade; the substrate reflects that ten-year
lineage back at itself as its terminal geometry.

---

## §3 Answering the open questions from today's audit chain

### §3.1 ALEX-Q1 (Taut OQ3 revisited): @dance shard-mint gate

Seam adjudication in `2fdc9c1` §5.4 promoted @dance to second-witness
on citation-site basis; shard-mint still gated on empirical
`apply_h::act` firing on ensemble @roomba.

**This spec provides the load-bearing GROUND for @dance shard-mint by
naming @dance as the ensemble-altitude connection 1-form in the tower.**
When `bootstrap/src/dance.rs` lands empirically (Reed Rust FLOOR work
per Alex's Q5 ratification), the shard-mint gate fires: (1)
citation-site ✓ (this spec + `beam-runtime.md` §6 + prior audit
chain); (2) second-witness ✓ (per Seam `2fdc9c1`); (3) empirical
apply_h::act firing → gates on `dance.rs` landing.

**Recommendation:** hold @dance shard-mint until `dance.rs` empirical
Path B fires. THIS spec is not shard-mint; it is the mathematical
grounding that makes the eventual shard-mint substrate-honest rather
than aspirational. Landing Condition 0 (per
`docs/math/gestalt/README.md §11.6`) is now math-root-grounded via
`beam-runtime.md` §6.

### §3.2 ALEX-Q2 (Taut OQ5): Beer VSM recursion — bounded vs unbounded

Seam recommended (b) BOUNDED-AT-@roomba as this-arc default. Under
the bundle framing:

- **(a) UNBOUNDED** = each of the K tracks IS a peer capable of
  further `.audhd`; corresponds to a bundle tower of unbounded
  height. Baez-Schreiber ∞-connections on ∞-bundles (per Schreiber's
  nLab work) provide the formal object; empirically requires K^depth
  processes, unbounded.
- **(b) BOUNDED-AT-@roomba** = the K tracks emit @songs consumed by
  @roomba; @roomba is S1 operational altitude; no further `.audhd`.
  Corresponds to a bundle tower truncated at level 2 above the
  process floor.
- **(c) BOUNDED-AT-K-DEPTH** = configurable depth per @torus; K^depth
  tracks total.

**This spec composes with Seam's recommendation:** the bundle tower
has depth ADMISSIBLE at every finite level. The BEAM/OTP precedent
(Armstrong 2003 §5.4) demonstrates 30+ years of empirical viability
at depths 3-5 in production systems (application → top supervisor →
mid supervisor → worker supervisor → worker). Alex's decade of BEAM
practice runs at this depth range. The substrate should DEFAULT to
(b) matching the landed consumer chain; (a) is the substrate-honest
peer paradigm for Scope C when the depth-2 truncation empirically
proves insufficient.

**Bundle-theoretic gloss:** the compatibility condition (§2.2) holds
at every finite level. Unbounded recursion is admissible under the
compatibility theorem; the substrate's Rice-safe scrutiny discipline
(per `@epistemologic/pact/bilateral.mirror`) requires empirical
firing at each level before promotion. Default (b) with (a) as
forward-promise satisfies both.

### §3.3 ALEX-Q3 (Taut OQ6): losing commutator arm fate

Seam recommended (a) COLD-STORAGE via `@mirror/store/cold`. Under
the bundle framing:

- The K parallel @songs emitted by `.audhd` ARE the K arms of the
  spectral commutator `[dispatch_A, dispatch_B, ..., dispatch_K]` at
  the peer altitude.
- Under the tower framework, each arm generates its own bundle
  section trajectory. The @liquid predicate selects the arm whose
  holonomy at Aumann-convergence passes the coherence threshold.
- The LOSING arms are sections that DID NOT reach the fixed point.
  Under the Baez-Schreiber compatibility framing, these sections
  witness the NON-COMPATIBILITY of the eliminated commutator arms
  at ensemble altitude.

**Recommendation:** cold-storage aligns with the bundle framing.
The losing arms retain their content addresses in `@mirror/store/cold`
as sections that were TRIED-AND-DIDN'T-COMPOSE. The audit trail IS
the compatibility-witness at the ensemble altitude: future @peer.audhd
invocations can query "which commutator arms have been tried at this
context" and skip re-exploration, or resurrect a cold arm when
context shifts.

Composes with the roomba `dock` fifth-motion forward-promise (per
Seam `2fdc9c1` §7 ALEX-Q2) — `dock` IS the halt of the roomba's
motion; cold-storage IS the halt of the ensemble arm's exploration.
Same halting discipline, two altitudes.

### §3.4 Q4-forward (deferred): naming @sing at peer altitude

Per Taut `f6d33d2` OQ1 + Seam `2fdc9c1`: `.sing` names the emission
verb at peer altitude (vs `.beam` = beam-into-existence). Under this
spec's framing:

- `.beam(r, p) -> @song` = create the section (birth the gen_prism
  actor) + emit the first @song (initial parallel transport step).
- `.sing(p) -> @song` = subsequent parallel transport step from an
  ALREADY-BIRTHED section. Post-materialization emission.
- `.audhd(p, ctx) -> [@song]` = the K-cardinality version of `.sing`
  at cognition-fanout altitude.

Recommendation deferred to Alex per OQ1. This spec does not ratify.

### §3.5 Q6-forward (deferred): supervisor as ensemble-connection-of-connections

**Delightfully boring alignment:** if @dance is the connection at
ensemble altitude, then @spectral/supervisor coordinating N
gen_prisms is @dance at N=count(children). The two names collapse to
one under the tower framing.

**Refused mint:** DO NOT collapse @dance and @spectral/supervisor.
The distinction is load-bearing: @spectral/supervisor is the
BEAM/OTP-shaped connection with `restart_strategy` (permanent /
transient / temporary; one_for_one / one_for_all / rest_for_one);
@dance is the KURAMOTO-shaped connection with phase-lock (κ_intra;
Aumann-envelope). Same altitude of tower, different structure groups.
Beer's S3 (audit / management) IS supervisor; Beer's S2 (anti-
oscillation coordination) IS @dance. Two Beer subsystems at the
same tower altitude.

Landing @dance shard-mint composes over @spectral/supervisor; it
does NOT replace it.

---

## §4 Recognition candidates surfaced

Do NOT ratify. Names proposed for Pack adjudication:

- **`#R-gen-prism-is-bundle-section-at-BEAM-process-altitude`**
  (first-witness THIS spec §2.1; second-witness gate: `bootstrap/src/
  dance.rs` empirical Path B firing with gen_prism actors dispatched
  as sections).

- **`#R-supervisor-is-bundle-connection-at-supervision-altitude`**
  (first-witness THIS spec §2.2; second-witness gate: `restart_intensity`
  compatibility condition witnessed at supervisor cascade — e.g., the
  @roomba `dock` fifth-motion landing acting as the halt witness).

- **`#R-dance-is-bundle-connection-at-ensemble-altitude`**
  (first-witness THIS spec §2.3; second-witness gate: `dance.rs`
  landing with Kuramoto phase-lock realized as ensemble connection
  1-form; the @dance shard-mint promotes on this event).

- **`#R-tower-recursion-bounds-are-substrate-honest-at-every-finite-depth`**
  (first-witness THIS spec §3.2; second-witness gate: two independent
  empirical dispatches at different depths — one @roomba single-walker
  and one @dance-ensemble N-walker BOTH satisfy their compatibility
  conditions).

- **`#R-losing-commutator-arms-are-non-compatibility-witnesses-at-ensemble-altitude`**
  (first-witness THIS spec §3.3; second-witness gate: cold-storage
  arm resurrection empirically fires when context shifts).

- **`#R-alex-decade-of-BEAM-is-substrate-reaching-for-terminal-geometry`**
  (sibling to `beam-runtime.md` §8's
  `#R-substrate-mirrors-alex-decade-of-BEAM-engineering-at-terminal-floor`
  and to Seam `2fdc9c1` §5.2's
  `#R-substrate-is-author-mirror-third-order-cybernetics`; first-
  witness THIS spec §2.5; second-witness gate: `dance.rs` lands and
  Alex reads the shape back into the loop with the "coming home"
  feeling).

---

## §5 What this spec refuses to mint

Michelangelo/marble discipline. Six refusals with reasoning:

**§5.1** Refuse `@bundle` family-root. The tower IS the substrate
already (per `docs/math/the-tower/`); it is not a shard-decl'd family
that composes with other families. Minting `@bundle` would
double-declare what `docs/math/the-tower/` + `prism::Bundle` already
carry. Composition over the existing math root + prism implementation
suffices.

**§5.2** Refuse a `@spectral/dance` species. @dance's altitude is
ensemble (level k+1 above supervision); @spectral is the runtime
namespace at supervision altitude. Placing @dance under @spectral
would collapse two distinct altitudes into one namespace. The
substrate-honest placement is `@dance` at the family-root altitude
per the two canonical specs' Path C recommendation (Mara `4f079c8`).

**§5.3** Refuse to collapse `@spectral/supervisor` into `@dance`.
Per §3.5: same tower altitude, different structure groups. Beer S3
vs Beer S2. Load-bearing distinction.

**§5.4** Refuse a `@bundle_section` type alias for `@spectral/gen_prism`.
The gen_prism CARRIES the section structure without needing to declare
`is_bundle_section: bool`. Type-tagging the identity when the semantics
already hold is over-declaration. `substrate-already-had-the-word`
(53rd instance) forbids the ceremonial rename.

**§5.5** Refuse to lift `bundle.rs` from prism into mirror. AGENTS.md
§"When in doubt, cite the existing reference" is explicit:
`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`
implements the compiler-altitude tower; mirror LIFTS the semantics,
prism REALIZES them. Duplicating the Rust would violate the "mirror
stays full mirror" discipline (per `@code/beam:14-17`). The @io
boundary to prism via the mirror-store lift discipline is the
substrate-honest path.

**§5.6** Refuse to write ANY `.rs` file this arc. Per Reed memory
`feedback_no_rust_extension_shortcut.md` + `feedback_detector_
inadequacy_answer_is_never_rust.md`: this arc is spec + math + shard-
citation cascade. Reed lands `bootstrap/src/dance.rs` when the /loop
fires with substrate-floor:@io-boundary marker + Seam-gate audit-cite.
Mara does not author `.rs`.

---

## §6 Cross-arc coherence

Composes cleanly with today's landings:

- **@peer.audhd** (Mara `d8b149c` + Seam `2fdc9c1` SHIP-with-Reed-
  inline) — this spec §2.4 names @peer.audhd as the K-cardinality
  fanout that produces the @dance-ensemble input. Q1 second-witness
  gate available (Seam-recommended ACCEPT); if Alex accepts, the
  `#R-dance-is-bundle-connection-at-ensemble-altitude` gate composes.
- **@liquid Arc 5 M1** (`cc816f9` + `b2c5d09` + `12cdf0e`) — @liquid
  predicates gate the winning commutator arm at §3.3.
- **Errors-as-questions joint arc** (`5e1f528` + `914799b` + `09a77e8`)
  — `@kintsugi/roomba.pivot(@song)` Path B fourth-motion IS the
  single-walker precedent that the ensemble @roomba composes over.
- **@viable Beer VSM** (`shards/epistemologic/cybernetic/viable.mirror`)
  — the Beer VSM naming at `beam-runtime.md` §5 aligns S1
  (@spectral/gen_prism) + S2 (@dance ensemble coordination) + S3
  (@spectral/supervisor management) + S4 (@spectral/db) + S5 (@viable
  identity carrier + Pack).
- **Reed -63 LOC retirement** (`9b72a08`) — no interaction surface.

No contradictions detected.

---

## §7 Terminal state

- Verdict: canonical spec landed as forward-promise scaffold for
  Reed's `dance.rs` empirical work.
- Recognition candidates: 6 (§4)
- Mint refusals: 6 (§5)
- Open questions answered by construction: 3 (Alex-Q1 grounded /
  Alex-Q2 aligned / Alex-Q3 aligned; §3.1-§3.3)
- Open questions deferred: 2 (Q4-forward .sing verb / Q6-forward @dance
  vs @spectral/supervisor collapse refused — §3.4-§3.5)
- Cross-arc coherence: ✓ (§6)

Pure-docs 📝 markdown-only bypass legitimate.

---

## §8 References

**Bundle tower foundations:**
- `docs/math/the-tower/beam-runtime.md` (Mara `610c6d6`, this arc)
- `docs/math/the-tower/principal-bundles.md`
- `docs/math/the-tower/connections-and-gauge.md`
- `docs/math/the-tower/altitudes.md`
- `docs/math/the-tower/crystals-as-sections.md`
- Baez, J. C., Schreiber, U. (2004). arXiv:hep-th/0412325 (2-connections
  on 2-bundles)
- Kobayashi, S., Nomizu, K. (1963). *Foundations of Differential
  Geometry.*

**BEAM prior art:**
- Armstrong, J. (2003). *Making reliable distributed systems in the
  presence of software errors.* PhD thesis.
- `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md`
- `~/dev/systemic.engineering/practice/insights/beam-elixir/spectral-beam-integration.md`
  (four integration options; D=distribution protocol recommended
  first; B=minimal VM is the long game)

**Substrate composition surface:**
- `shards/spectral/gen_prism.mirror`
- `shards/spectral/supervisor.mirror`
- `shards/spectral/restart_intensity.mirror`
- `shards/code/beam.mirror`
- `shards/epistemologic/cybernetic/viable.mirror`
- `shards/peer.mirror` (per today's `d8b149c` @peer.audhd landing)
- `shards/kintsugi/roomba.mirror` (per today's `914799b` pivot(@song))
- `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08)
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`
  (Mara `4f079c8`)
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
- `docs/math/gestalt/README.md §11.6` (Landing Condition 0 gate)

**Today's audit chain:**
- `docs/audits/2026-07-17-taut-peer-sing-split-dance-ensemble-scout.md`
  (Taut `f6d33d2` + `d21a34f` naming cascade)
- `docs/audits/2026-07-17-seam-phase-d-peer-audhd-mara-michelangelo-landing.md`
  (Seam `2fdc9c1`)

**Compiler-altitude implementation:**
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`
  (Fiber → Connection → Gauge → Transport → Closure; ~626 LOC with
  `GroupStructure` + `LawvereFixedPoint` supertrait laws)

**Spectral prototype BEAM-side (Gleam):**
- `/Users/alexwolf/dev/projects/spectral/beam/gen_prism/src/gen_prism.gleam`
  (typed `PrismCallbacks` + `PrismMsg` for the five ops as a BEAM
  process behaviour; the prototype's realization of what
  `@spectral/gen_prism` names at substrate altitude)
- `/Users/alexwolf/dev/projects/spectral/docs/gen_prism.md` (spectral's
  BEAM runtime bytecode design, 2026-04-07)
- `/Users/alexwolf/dev/projects/spectral/docs/specs/beam-integration.md`

**Kagi 2026-07-17:**
- Gleam OTP (github.com/gleam-lang/otp) — typed BEAM supervisor / gen_server
  bindings; production-grade; provides the eventual `@code/beam` target
  language for the terminal @spectral runtime on BEAM.
- Baez-Schreiber 2004 (arXiv:hep-th/0412325) — the higher gauge theory
  formal object.

---

*This spec is not shard-mint. It is the mathematical grounding for
the substrate's terminal geometry that Alex's ten-year BEAM lineage
has been reaching for. When `bootstrap/src/dance.rs` lands, the shape
this spec names crystallizes into the compiler's Rust FLOOR. The
substrate keeps giving the author the substrate-truth word; this
spec is the record of listening for it.*
