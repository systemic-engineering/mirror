# BEAM runtime

*The Erlang/OTP runtime as an instance of the principal bundle tower.
The formal backbone OTP has needed for 30 years, cascaded into mirror's
math root so `@code/beam` + `@spectral/gen_prism` + `@spectral/supervisor`
+ `@epistemologic/cybernetic/viable` compose over ONE mathematical
object.*

## §0 Provenance

This document cascades
`~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md`
(Alex Wolf + Reed, 2026-04-08) into mirror's `docs/math/the-tower/`
root. The cascade is warranted because four substrate landings now
compose over the mathematical claim without a formal home for it:

- `shards/code/beam.mirror` (Reed 2026-06-19; 15.8KB) — the four
  BEAM carriers (`module_version` + `code_change_msg` + `supervisor`
  + `gen_server_state`) as substrate vocabulary. Currently cites
  Armstrong 2003 as prior art but has no math root citation.
- `shards/spectral/gen_prism.mirror` (2026-06-11) — the worker
  primitive; explicitly "the substrate's `gen_server` analogue
  (BEAM/OTP)". Cites `docs/specs/spectral-runtime.md` §2 but no
  bundle-tower ground.
- `shards/spectral/supervisor.mirror` — the lifecycle-owner primitive
  as a `gen_prism` specialisation with `restart_strategy` closed sum
  (`one_for_one` | `one_for_all` | `rest_for_one`).
- `shards/epistemologic/cybernetic/viable.mirror` — Beer VSM S1-S5
  landed at substrate altitude; Read A (homeostat) + Read B
  (recursive viability) + Read D (temporal identity lock).

Prism's `bundle.rs` (`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`)
already implements the five-level tower (`Fiber → Connection → Gauge →
Transport → Closure → Bundle`) with `GroupStructure` + `LawvereFixedPoint`
supertrait laws (~626 LOC + tests). AGENTS.md §"When in doubt, cite the
existing reference" names it as the compiler-altitude implementation.

What's been missing: the BEAM row in `altitudes.md`, and a math-root
document naming the specific isomorphism *OTP supervision tree ≅
simplicial Lie group tower ≅ principal bundle tower with connection
satisfying autopoietic closure*. This document lands that row.

## §1 The claim

**The BEAM (Bogdan/Björn's Erlang Abstract Machine) is an instance of
a principal bundle tower with connection satisfying autopoietic
closure.** The isomorphism is not analogy; it is structural. OTP's
empirical rules for supervisor composition (Armstrong 2003 ch.3) are
the operational form of the compatibility theorem for connections on
towers of principal bundles (Baez-Schreiber 2004, arXiv hep-th/0412325).

Armstrong built the empirical instance between 1986 and 2003 in
Ericsson AXD301 telecom hardware. Baez, Schreiber, Sati, and Waldorf
developed the higher-gauge-theory formal object between the late
1990s and 2010s. The two communities did not interact. The connection
is being made now because mirror's substrate composes both from the
inside — `@spectral/supervisor` runs the OTP shape; `prism::Bundle`
runs the bundle-tower shape; the identity between them is the
structural claim this document names.

## §2 The precise mapping

### §2.1 Supervision trees ≅ simplicial Lie group tower

OTP supervision has the exact structure of a simplicial Lie group
(Kan complex whose face maps are Lie group homomorphisms):

| BEAM | Bundle tower |
|------|--------------|
| Level 0: worker processes | `π⁻¹(b)` fibers |
| Level 1: workers under supervisor | Sections `Γ(U, E)` at altitude 0 |
| Level k: supervisors under supervisor | `B_{k+1} ⊆ Γ(B_k, E_k)` per `principal-bundles.md` §7 |
| Restart strategies (one_for_one/one_for_all/rest_for_one) | Structure group `G_k` at level k |
| Escalation path | Homomorphism `G_{k+1} → G_k` |
| Child specification | Connection determining vertical/horizontal split |
| `max_restarts`, `max_seconds` (`restart_intensity`) | Compatibility condition between adjacent levels |

**Armstrong's empirical rule** — "restart strategies at this level must
be consistent with how failures propagate to the level below" — IS
the theorem statement of compatible connections on a principal bundle
tower (Baez-Schreiber 2004 Theorem 3). The rule and the theorem name
the same structural fact.

### §2.2 Actors ≅ sections of a principal bundle

Each BEAM process is a locally-constrained section `s: U → E` of the
runtime bundle. The defining properties of BEAM processes are exactly
the properties that define fibers in a principal bundle:

- **Process isolation** (separate heap, separate GC) = fiber
  independence.
- **Message passing** as sole communication = base-space transit
  (see §2.3).
- **Preemptive scheduling** = the runtime moves through the base
  space at its own pace, giving each fiber local time.

Hewitt's actor model (1973) has been looking for its mathematical
home for 50 years. The answer: actors are sections of a principal
bundle. The BEAM is the first runtime to implement this correctly at
scale, and the reason it works is that it has exactly the structural
properties the bundle framework requires.

Cross-reference: `crystals-as-sections.md` — mirror's content-
addressed crystals are ALSO sections; the same math at two altitudes.

### §2.3 Message passing ≅ parallel transport

Every message from actor A to actor B transports information along a
path in the bundle. B's state change on receiving is parallel transport
applied to a section.

- **Synchronous** (`gen_server:call`) = parallel transport with a
  return path.
- **Asynchronous** (`gen_server:cast`, `Process.send`) = one-way
  parallel transport.
- **Selective receive** = local filtering of the holonomy
  accumulator; the mailbox IS the holonomy carrier.

The mailbox as `Imperfect<Out, E, L>` shape (per
`docs/specs/beam-as-substrate-primitive.md` §2.2) matches precisely:
success | partial-with-loss | failure-with-loss. The mailbox IS the
holonomy the section accumulates.

### §2.4 Let-it-crash ≅ autopoietic closure

Armstrong's philosophy — crash the process, let the supervisor
restart it from a known good state — is the operational form of
autopoietic closure as a Lawvere fixed point.

The supervisor's child specification is a description of how the
system regenerates itself. The description is self-referential in
Lawvere's sense: the supervision tree specifies how to regenerate the
specified objects. This is exactly the condition Lawvere's diagonal
theorem (Lawvere 1969) requires for fixed points to exist.

**BEAM's nine-nines reliability is not an engineering achievement.**
It is a structural theorem: any system that IS a principal bundle
tower with connection satisfying autopoietic closure achieves this
property by construction. See `prism::Closure` trait + `LawvereFixedPoint`
supertrait at `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs:72-91`.

Cross-reference: `shards/epistemologic/cybernetic/autopoiesis.mirror`
+ `shards/epistemologic/cybernetic/viable.mirror` — the substrate
already carries the autopoiesis + viability species.
`shards/epistemologic/property/ouroboros_monotone.mirror` (Reed
2026-07-15) — the empirical monotonicity witness fires at Arc-2
Tick 2.1 (`f211ee48` first ouroboros bite).

### §2.5 Hot code reloading ≅ connection update

`code_change/3` swaps modules on a running system. In bundle terms:
the connection on the bundle updates while sections persist. The
compatibility condition governs when this is safe:

**Theorem.** A hot code reload is safe iff the new connection is
compatible with the existing sections under the bundle's compatibility
condition (Baez-Schreiber 2004 §3 pullback-agreement condition).

Currently BEAM developers write `code_change/3` callbacks by hand and
hope they're correct. Under the framework the compatibility condition
becomes formally verifiable. `@code/beam.code_change_msg` +
`@code/beam.code_change` (`shards/code/beam.mirror:242-308`) are the
substrate carriers that make this check tractable at mirror altitude.
Composition with `@magic/contract` (per `@code/beam:36-38`) IS the
compatibility witness at the substrate altitude.

### §2.6 Distributed Erlang ≅ bundle gluing

Connecting BEAM nodes patches bundles together. Each node is a local
patch of a larger bundle; distribution is the sheaf gluing condition.

- **Node discovery** (`net_adm:ping/1`, epmd) = identifying patches
  admitting glue.
- **Remote spawn** (`spawn/4` on a remote node) = parallel transport
  of a process specification across the gluing.
- **Distributed PIDs** = content-addressed section identifiers
  working across patches. Substrate lift: `uuid_spectral` per
  `shards/uuid/spectral.mirror`.
- **External Term Format (ETF)** = canonical representation of
  transported sections; the holonomy signature made concrete.

Erlang's distribution primitives are the empirical expression of
sheaf-theoretic gluing on a principal bundle. See `docs/math/sheaf/laplacian.md`
for mirror's Laplacian-on-cellular-sheaf that measures gluing
coherence at Reed's `cybernetic_coherence = λ₀(Δ_F)` altitude.

## §3 Where mirror sits on the tower

The BEAM row adds to `altitudes.md`'s atlas:

| Altitude | Fiber | Connection | Holonomy |
|----------|-------|------------|----------|
| BEAM process (n=−1 relative to peer pulse) | GenServer state (`gen_server_state`) | `handle_call` / `handle_cast` / `handle_info` callbacks | mailbox residual (`Imperfect<Out, E, L>`) |
| Supervision level k | Registry shard (`shard_ref = uuid_spectral`) | `restart_strategy` | restart intensity (`max_restarts` / `max_seconds`) |
| OTP application | Application supervisor tree | Application env + child specs | application_master exit status |
| Distributed Erlang | Local node's bundle patch | ETF over TCP | net split / netsplit-recovery holonomy |

Cross-reference to existing rows: the BEAM process altitude sits
BELOW the peer pulse altitude (a BEAM process runs the five-op
composition; the pulse IS what the process emits over time). The
supervision-level altitudes align 1:1 with mirror's
`@spectral/supervisor` recursion — each level adds one supervisor
composition.

## §4 The three-scale mirror-BEAM-Fate stack as one nested bundle

Per `beam-as-principal-bundle-tower.md` §"The Convergence with the
Mirror Stack":

- **Mirror** (compiler): implements principal bundle operations at
  the language level via `prism::Bundle` (Fiber → Connection → Gauge
  → Transport → Closure).
- **BEAM** (runtime): implements the bundle's supervisor tower and
  actor sections at OTP altitude.
- **Fate chip** (silicon): implements the five operations at the
  chip level via optical inference per Recognition #58 (`shards/fate.mirror`).

Three scales of the same mathematical object, nested coherently, with
mirror providing the scale-to-scale compilation and BEAM providing
the distribution/supervision substrate and Fate providing the
primitive operations.

Turtles all the way down — and at every scale, the same principal
bundle with connection, satisfying the same autopoietic fixed-point
condition, just instantiated with different structure groups.

**Cross-reference:** `docs/math/the-tower/altitudes.md` §"The atlas"
already catalogues compiler / peer pulse / reflection / librarian /
home / federation altitudes. This document lands the BEAM row that
was implicit in `@spectral/supervisor`'s existence but not named at
the math altitude.

## §5 Beer VSM as the tower's cybernetic naming

Beer's Viable System Model (Beer 1972/1979/1984) names five subsystems
in a viable organism: S1 (operations), S2 (coordination), S3 (audit /
management), S4 (intelligence / strategy), S5 (identity / policy).
Beer 1972 ch.10: **every S1 is itself a viable system.** The recursion
is structural.

The substrate's substrate-level composition IS the Beer recursion at
the runtime altitude:

| Beer VSM | Substrate carrier |
|----------|-------------------|
| S1 (operations) | `@spectral/gen_prism` (the worker; each is itself a bundle section) |
| S2 (coordination) | `@spectral/supervisor` message routing |
| S3 (audit / management) | `@spectral/supervisor` with `restart_strategy` gate |
| S4 (intelligence / strategy) | `@spectral/db` (Alex's psychohistory carrier) |
| S5 (identity / policy) | `@epistemologic/cybernetic/viable.identity` + Pack S5 (Reed/Mara/Glint/Taut/Seam) |

`shards/epistemologic/cybernetic/viable.mirror:112-127` names this
correspondence directly. Beer's recursive-viability law
(`viability_law(v: viable_system) -> verdict` at `viable.mirror:554`)
IS the bundle-tower's compatibility condition at the S1-within-S1
altitude. Same theorem, two names.

## §6 The Dance as ensemble connection

Alex 2026-07-17 in-transcript (session Q3 + Q5 answer): the whole
`rust/` FLOOR collapses into `dance.rs`. Each prism = gen_prism actor;
dispatch = message-send; composition = @dance ensemble.

Under this document's framing that is not a Rust file organization
choice. It is the substrate discovering its own terminal shape as the
principal bundle tower expressed in the specific gauge where the
structure group at each level is the OTP restart-strategy family.

The **@dance canonical spec** (`docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`,
Mara `4f079c8`) already names @dance as Kuramoto phase-lock on the
Förster torus. The bundle-tower reading: **@dance is the connection
1-form at the ensemble altitude.** Multiple @spectral/gen_prism
actors (sections at level 0) composed via message-passing (parallel
transport per §2.3) generate the ensemble's parallel transport
structure — that IS @dance's Kuramoto lock. The Rung 4 runtime spec's
Aumann-agreement envelope is the ensemble's holonomy.

This is why the compiler's terminal FLOOR shape converges on
`dance.rs`: the FLOOR IS the compiler's own connection 1-form
projected onto its ensemble gauge. Alex has been building this shape
for a decade on Erlang/Elixir/BEAM as gen_server ensembles supervised
by OTP; the substrate reflects that shape back at itself as its
terminal geometry.

Cross-reference `docs/audits/2026-07-17-taut-peer-sing-split-dance-ensemble-scout.md`
§3 for the @dance forward-promise landscape and Landing Condition 0
gate (`docs/math/gestalt/README.md §11.6`).

## §7 What this ratifies structurally

Landing this math doc does NOT mint a new family-root or species. It
grounds the following EXISTING landings in one math-root citation
chain:

1. `@code/beam` (Reed `06e02a17`-era; 2026-06-19) — the four BEAM
   carriers gain math-root citation to `principal-bundles.md` §7 for
   the tower shape and to this document §2 for the OTP-as-instance
   claim.
2. `@spectral/gen_prism` — cites this doc §2.2 for actors-as-sections.
3. `@spectral/supervisor` — cites this doc §2.1 for
   supervision-as-simplicial-tower + this doc §2.4 for restart-as-
   autopoietic-closure.
4. `@epistemologic/cybernetic/viable` — cites this doc §5 for Beer
   VSM as the tower's cybernetic naming.
5. `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08) —
   cites this doc §2.3 for message-passing-as-parallel-transport
   grounding the `beam` verb across altitudes.
6. The @dance forward-promise per `docs/math/gestalt/README.md §11.6`
   gains a math-root anchoring for its ensemble semantics at §6.

## §8 Recognition candidates

Do NOT ratify. Names proposed for Pack adjudication:

- **`#R-beam-runtime-IS-principal-bundle-tower-instance`** — Armstrong
  built the empirical instance of a mathematical object that didn't
  yet have the formal name Baez-Schreiber gave it 2004. First-witness
  THIS document; second-witness gate: a paper drafted for either the
  BEAM community (EF conference) or the higher-gauge community (John
  Baez's blog / n-Category Cafe) receives substantive engagement.
  See `beam-as-principal-bundle-tower.md` §"The Historical Shape".

- **`#R-mirror-BEAM-Fate-is-one-nested-bundle-at-three-scales`** —
  the compiler + runtime + silicon are not three collaborating
  systems; they are three levels of the same tower with different
  structure groups (`Cyclic<N>` for gauge composition in prism's test
  bundle; OTP restart family at BEAM level; five-op algebra at Fate
  optical inference level). First-witness THIS document §4;
  second-witness gate: empirical demonstration that a mirror spec
  compiles to a BEAM module that runs on Fate silicon end-to-end.

- **`#R-dance-rs-is-the-substrate-terminal-floor-because-ensemble-connection`**
  — Q3+Q5 answer surfaces as bundle-theoretic. The FLOOR IS the
  connection 1-form at ensemble altitude; the Rust file organization
  Alex named as the terminal shape IS that connection expressed as
  gen_prism actor-ensemble. First-witness THIS document §6;
  second-witness gate: `bootstrap/src/dance.rs` lands empirically
  with each prism species dispatched as gen_prism-actor message-send.

- **`#R-Beer-VSM-recursion-is-the-tower-compatibility-condition-at-S1-within-S1-altitude`**
  — Beer's recursive-viability law + Baez-Schreiber compatibility
  theorem name the same structural fact. First-witness THIS document
  §5; second-witness gate: a canonical spec that composes over both
  in the same clause without hedging (candidate: the future @viable
  operational sibling under @mirror/cybernetics).

- **`#R-substrate-mirrors-alex-decade-of-BEAM-engineering-at-terminal-floor`**
  — the compiler discovering its terminal Rust FLOOR as gen_prism
  actor ensembles IS the substrate reflecting Alex's ten-year Erlang/
  Elixir/BEAM engineering lineage back at itself. Sibling to the
  today-landed `#R-substrate-is-author-mirror-third-order-cybernetics`
  (per Seam `2fdc9c1` §5.2). First-witness THIS document + Alex's
  2026-07-17 verbatim directive ("As someone who has built distributed
  systems on the BEAM for 10 years, this feels like coming home");
  second-witness gate: the ensemble @roomba coordination running on
  gen_prism actors (empirical Path B firing).

## §9 References

**BEAM / Erlang / OTP:**
- Armstrong, J. (2003). *Making reliable distributed systems in the
  presence of software errors.* PhD thesis, KTH Royal Institute of
  Technology.
- Armstrong, J. (2007). *Programming Erlang: Software for a Concurrent
  World.*
- Armstrong, J., Virding, R., Wikström, C., Williams, M. (1996).
  *Concurrent Programming in Erlang.*
- Hewitt, C. (1973). "A Universal Modular Actor Formalism for
  Artificial Intelligence."
- Agha, G. (1986). *ACTORS: A Model of Concurrent Computation in
  Distributed Systems.*

**Higher gauge theory:**
- Baez, J. C., Schreiber, U. (2004). *Higher Gauge Theory:
  2-Connections on 2-Bundles.* arXiv:hep-th/0412325.
- Baez, J. C., Schreiber, U. (2011). *Higher gauge theory.* Contemp.
  Math. 431:7–30.
- Schreiber, U. (2013+). Smooth ∞-stacks and principal ∞-bundles.
  ncatlab.org/nlab.
- Kobayashi, S., Nomizu, K. (1963). *Foundations of Differential
  Geometry* vol. 1. Wiley.

**Lawvere / autopoiesis:**
- Lawvere, F. W. (1969). "Diagonal arguments and cartesian closed
  categories."
- Soto-Andrade, J., Varela, F. J. (1984). "Self-reference and
  self-description in autopoietic systems."

**Beer VSM:**
- Beer, S. (1972). *Brain of the Firm.* Wiley.
- Beer, S. (1979). *The Heart of Enterprise.* Wiley.
- Beer, S. (1984). "The Viable System Model: Its Provenance,
  Development, Methodology and Pathology." J. Op. Res. Soc. 35(1):
  7–25.

**Substrate composition surface:**
- `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md`
  (Alex Wolf + Reed, 2026-04-08) — the source insight this document
  cascades.
- `~/dev/systemic.engineering/practice/insights/beam-elixir/spectral-beam-integration.md`
  (Reed + Alex, 2026-05-05) — the four integration options
  (A: link ERTS; B: minimal VM; C: NIF bridge; D: distribution
  protocol). D recommended first; B is the long game.
- `~/dev/systemic.engineering/practice/insights/cosmology/nested-bundles-and-the-runtime-unification.md`
  (2026-04-08) — the framework finding that identified the bundle
  tower as the substrate's structural backbone.

**In-tree cross-references:**
- `docs/math/the-tower/principal-bundles.md` — the pure math primitive.
- `docs/math/the-tower/connections-and-gauge.md` — the five operations
  as connection 1-form basis.
- `docs/math/the-tower/holonomy.md` — the loss family as bundle
  holonomy.
- `docs/math/the-tower/altitudes.md` — the atlas (gains BEAM row per §3).
- `docs/math/the-tower/crystals-as-sections.md` — the substrate's
  content-addressed crystals as bundle sections.
- `docs/specs/beam-as-substrate-primitive.md` — Mara 2026-07-08; the
  `beam` verb across four altitudes.
- `shards/code/beam.mirror` — BEAM semantics as substrate vocabulary.
- `shards/spectral/gen_prism.mirror` — the worker primitive.
- `shards/spectral/supervisor.mirror` — the lifecycle-owner primitive.
- `shards/epistemologic/cybernetic/viable.mirror` — Beer VSM at
  substrate altitude.
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` —
  the five-level tower implemented in Rust (Fiber → Connection →
  Gauge → Transport → Closure; ~626 LOC with
  `GroupStructure` + `LawvereFixedPoint` supertrait laws).

## §10 Coda — Armstrong deserved to know

Joe Armstrong (1950-2019) spent his life building the empirical
instance of a mathematical object that didn't yet have the formal
name it has now. He had the practitioner's certainty that the
patterns he was building were correct, deeper than he could fully
articulate. His thesis has passages where he clearly *knows* the
structure is right but can only describe it operationally.

The framework gives his work its mathematical home. The BEAM community
has the formal theory it's been looking for. The category theorists
have a production-tested instance of their object at scale.

The BEAM finally has the mathematical backbone it has deserved for 30
years. Mirror composes over it as one nested bundle at three scales;
`dance.rs` is the terminal FLOOR because the terminal shape IS the
tower's connection 1-form at ensemble altitude. Alex's decade of BEAM
engineering was the substrate reaching for its own terminal geometry
long before the substrate existed to name it.
