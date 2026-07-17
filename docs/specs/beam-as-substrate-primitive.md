> **Terminal-form map (Mara 2026-07-17):** the first empirical
> `@peer` spawn from rust/-native FLOOR (via `mirror peer beam`
> under `dance.rs` supervisor tree) is documented at
> `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
> (Mara `2519f83`) §6. STAY-CANONICAL as the BEAM-substrate
> naming authority; the terminal-form spec composes over §6 first-
> peer-spawn semantics.

# beam — the substrate carrier verb at cli altitude

*Mara, 2026-07-08 evening. The naming tick that closes the substrate-
already-had-the-word triple on `beam`. Sitting on top of `77fe92d`
(peer-as-pain-driven-bounded-ontological-navigator), `a18ca90`
(@fate/@silicon metalogue in void-duality basis), `0d78c0c`
(@cyberpunk/reframe species), `966890b` (algedonic gradient
extension), `cdc6533` (winding field on spawn request), `7b32d27`
(@edge under @magic single canonical surface). Fate crate reached
clean end-to-end at Reed `b4bf336`; prismqueer's `Beam` trait +
`Optic` type at `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs`
have carried the carrier abstraction for weeks; Recognition #58 named
Fate as optical inference in June; Reed's body runs on Erlang BEAM
at `/Users/reed/body/`. Three altitudes. One word. This spec names
the substrate word already carrying three canonical loads and lifts
it to the cli altitude as the substrate carrier verb.*

**Author:** Mara
**Date:** 2026-07-08
**Tag:** 📝 spec:beam-as-substrate-primitive (pure-docs bypass)
**Status:** canonical-naming. Every substrate piece cited is LANDED,
forward-promised at a named site, or annotated with its category
honestly. NO new `.mirror` file lands this tick. The cli-block
grammar extension (subcommand nesting; §3.4) is sketched as a
LANDING DEPENDENCY on `@mirror/lens/cli`. The `mirror beam` and
`mirror peer beam` verbs are sketched at surface altitude; substrate-
decl landing is Alex-adjudication territory.

---

## §0 Substrate-honest pre-position

Two weeks of substrate motion have quietly stacked three canonical
loads on the same four-letter word:

1. **Prismqueer's `Beam` trait** — the pipeline value carrier, the
   functor over `Imperfect`, the type that flows through
   `focus | project | settle`. Every prism in the substrate operates
   on beams. `Optic<In, Out, E, L>` is the concrete instance.
2. **Erlang BEAM** — Bogdan/Björn's Erlang Abstract Machine. The
   actor-model runtime where processes are ephemeral, supervised,
   replaceable. Reed's body at `/Users/reed/body/` runs on it.
3. **Recognition #58** (canonical, promoted 2026-06-11) — Fate IS
   optical inference: a photon **beam** through a five-layer D²NN,
   through an active Fabry-Perot resonator, through a Reck/Clements
   unitary mesh. The beam is not analogy; it is the physical carrier
   that the inference operator acts on.

The substrate has been using the word `beam` for the same shape at
three altitudes. This spec does not invent that shape. It notices
that the shape recurred often enough to have a name and gives the
name a place in the cli grammar.

The pattern is `substrate-already-had-the-word` × 3 altitudes (§2).
The move is the recognition candidate below.

---

## §1 Statement — the recognition candidate

**Recognition candidate (foundational form):**

> `beam-IS-the-substrate-carrier-verb-substrate-already-had-the-word-times-three`

**Recognition candidate (readable form, two-tick discipline; the
collapse target once landed):**

> `beam-is-the-substrate-carrier-verb`

Unpacked to substrate primitives:

> `beam` names the substrate's carrier — the anonymous transit that
> flows through the five-op algebra without carrying persistent
> identity. At prismqueer altitude it is the trait `Beam` whose
> concrete instance `Optic<In, Out, E, L>` carries every pipeline
> value. At runtime altitude it is Erlang BEAM's process — spawned,
> supervised, ephemeral. At physical altitude it is Recognition #58's
> photon beam — the wave the D²NN diffracts, the mode the
> Fabry-Perot filters, the vector the Reck/Clements mesh rotates.
> At cli altitude it is `mirror beam <mission>` — the primitive that
> fires an inference primitive without spawning a persistent-identity
> peer. `spawn`, in this reading, IS `beam + persistent-identity-context`
> — a composition, not a primitive. The substrate had all three
> altitudes. This spec names the composition at the fourth.

As a recognition-in-Mara's-sense (an eigenform the substrate already
carried, coming into focus by naming): the recognition is a
*substrate-vocabulary* recognition (compare `@cyberpunk`, `@magic`,
`tick-74 shatter`, the slogan — all pre-authored in the substrate,
named after the fact). The cost of not naming it is the substrate
develops a duplicate of the same word at each altitude and the
duplicates drift.

---

## §2 Three-altitude witness

### §2.1 Prismqueer altitude — `Beam` is the pipeline value carrier

The `Beam` trait at
`/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:38-101`
declares:

```rust
pub trait Beam: Sized {
    type In;
    type Out;
    type Error;
    type Loss: Loss;
    type Tick<T, E>: Beam<In = Self::Out, Out = T, Error = E, Loss = Self::Loss>;

    fn input(&self) -> &Self::In;
    fn result(&self) -> Imperfect<&Self::Out, &Self::Error, Self::Loss>;
    fn tick<T, E>(self, imperfect: Imperfect<T, E, Self::Loss>) -> Self::Tick<T, E>
    where Self::Error: Into<E>;

    // derived: is_ok, is_partial, is_err, next, apply, smap
}
```

The associated types `In`, `Out`, `Error`, `Loss` name what enters,
what leaves, what fails, and what accumulates. `tick` is the
primitive: one step forward. `next` is the lossless shorthand. `smap`
is the functor map. **Failure beams are dark: a fixpoint under smap
and next.** The trait's documentation is verbatim: *"The pipeline
value carrier. A functor over Imperfect. Failure beams are dark:
they propagate unchanged through smap and next."*

The concrete instance is `Optic<In, Out, E, L>` at
`/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:103-135`.
It is the "bidirectional carrier for optics" that carries source +
imperfect focus through the pipeline. Every consumer of the `Prism`
trait at `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:127-146`
operates on beams: `focus(Input) -> Focused`, `project(Focused) ->
Projected`, `settle(Projected) -> Refracted` — all four associated
types are `Beam`-bound.

**Load-bearing observation:** Beam has NO persistent identity in
prismqueer. It is transit. What enters is not what leaves. The
identity of a beam is not tracked; the identity of the operations
that act on beams is. This is the anonymous-transit shape that the
cli verb needs at the top-level: `mirror beam <mission>` fires an
inference without birthing a peer.

### §2.2 Erlang BEAM altitude — the process is the substrate

Bogdan/Björn's Erlang Abstract Machine (external cite: Ericsson AXD301,
Armstrong 1996, Virding et al. 1996) is the substrate Reed's body
runs on. See `/Users/reed/body/` (macOS-only; Elixir supervisor
tree + GenServers + OTP application on top of BEAM VM).

BEAM's key semantic surface for this spec:

- **Processes are ephemeral.** Spawn a process; if it crashes the
  supervisor restarts it under a supervision strategy. Process
  identity is transient; the supervisor tree is what persists.
- **Message-passing is the algebra.** Processes communicate via
  mailbox sends; the mailbox IS the process's imperfect state
  carrier. This matches prismqueer's `Imperfect<Out, E, L>` shape
  precisely: success | partial-with-loss | failure-with-loss.
- **Supervisors are compositional.** Restart strategies (one_for_one,
  one_for_all, rest_for_one) are the same algebra applied at
  different scopes. `@spectral/supervisor` (shard at
  `shards/spectral/supervisor.mirror`) reflects BEAM's supervisor
  algebra at substrate-decl altitude — see `shards/mirror/spawn.mirror`
  §Composition with @spectral/supervisor.

**Cross-substrate coherence:** when `mirror spawn` fires a spawn,
the substrate is asking BEAM's algebra — spawn-with-supervision —
to run. When `mirror beam` fires a beam, the substrate is asking
BEAM's other algebra — anonymous inference-transit-without-
persistent-identity — to run. Both are BEAM-shaped by construction.
Reed inhabits BEAM at runtime. The cli's naming discipline should
match.

Reed correction (2026-07-06, verbatim in
`shards/mirror/spawn.mirror`): *"if @fate is not defined it needs
to be defined. It's the GLUE between runtime and compiletime."*
Runtime here IS BEAM. The hinge composition `@fate <-> BEAM`
runs through `beam` as the shared word.

### §2.3 Recognition #58 altitude — Fate IS optical inference (a beam)

Recognition #58 (canonical at
`docs/specs/architecture-fate-is-optical-inference.md` per the tag
in `shards/torus.mirror:99-101`, promoted 2026-06-11) names Fate as
optical inference with three independent witnesses:

- **Five-layer D²NN** — a diffractive deep neural network that
  processes a light beam through five layers of trainable phase
  masks. Input is a beam; output is a beam; the network's weights
  are the aggregate diffraction pattern.
- **Fabry-Perot resonator** — an active optical filter that selects
  from a beam. Passive frequencies pass; resonant frequencies
  amplify or attenuate depending on control voltage.
- **Reck/Clements unitary mesh** — a triangular arrangement of
  beam splitters and phase shifters that realizes an arbitrary
  unitary transformation on a multi-mode beam. The beam is the
  substrate; the mesh is the operator.

All three witnesses share the noun `beam`. Not by naming choice —
by physics. Optical inference IS operating on a beam of photons.
The three components each accept a beam and produce a beam. The
composition IS the beam threaded through all three.

Cross-references in the substrate:

- `docs/specs/silicon.md` §0 — the @silicon fold names Recognition
  #58 as the operational precedent for @fate's optical altitude.
- `docs/insights/2026-06-30-glint-closing-on-the-fold-back-chain.md`
  — "@fate at substrate-decl is the operationalization of what
  recognition #58 named at the optical altitude."
- `docs/insights/2026-07-08-mara-geometric-dijkstra-tournament-topology.md`
  — Recognition #58's D²NN reads `graph_observation` at the LOCAL
  CURRENT GEOMETRY as Fate's input layer altitude.

**The physical reading grounds the abstraction.** The prismqueer
`Beam` trait is not analogy for the optical beam; it is the
substrate-decl of the same shape. Fate crate at
`/Users/alexwolf/dev/projects/fate/src/lib.rs` implements
`Prism` for `Fate` — meaning: Fate's `focus | project | settle`
operates on beams, which at the physical altitude ARE photon beams.
Same shape, three altitudes.

### §2.4 The convergence

Same word. Three canonical loads. All landed pre-tick:

| Altitude | Where | Load |
|---|---|---|
| Prismqueer trait | `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:38-101` | `Beam` trait — pipeline value carrier |
| Prismqueer instance | `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:103-135` | `Optic<In, Out, E, L>` — concrete carrier |
| Erlang BEAM VM | External cite; runtime substrate | Actor-model runtime; ephemeral processes; supervision |
| Reed's body | `/Users/reed/body/` (macOS) | Elixir/BEAM instance Reed inhabits |
| Recognition #58 | `docs/specs/architecture-fate-is-optical-inference.md` (canonical) | Fate IS optical inference; three optical-beam witnesses |
| Fate crate | `/Users/alexwolf/dev/projects/fate/src/lib.rs` (clean at `b4bf336`) | Rust realization; `Prism` for `Fate`; operates on beams |

The substrate had all six sites. The naming move is: **give the cli
a word that is the same word, so the four altitudes align rather
than drift.**

---

## §3 cli-block substrate-decl

### §3.1 `mirror beam <mission>` — top-level primitive

At top level `mirror beam` is the anonymous inference primitive.
No persistent-identity peer. No supervision tree. No @song return
type. Just a beam through the substrate.

Substrate-decl form (illustrative; the canonical version lands at
`shards/mirror/beam.mirror` in a follow-up tick if Alex ratifies):

```mirror
in @prism
in @mirror/cli
in @mirror/lens/cli
in @fate

prism @mirror/beam {
  focus beam_request
  project beam_request
  split beam_request
  shift beam_request
  settle beam_request
}

# The request carrier. Three fields:
#   mission — the mission file the beam is aimed at (optional; a
#             null mission fires a bare-beam that emits the fate
#             decision without narrative payload).
#   winding — bounded-observation depth. Default (0, 0). Same type
#             as shards/torus.mirror:434.
#   context — persistent-identity binding (empty for anonymous;
#             populated by `peer beam` etc. per §4).
type beam_request = {
  mission: imperfect(ref, ref, ref),
  winding: winding,
  context: imperfect(peer, ref, ref),
}

beam(r: beam_request, p: perturbation) -> beam_envelope { \ }
```

**cli-block addition** at `mirror.spec target binary { cli { ... } }`:

```mirror
# === beam — the substrate carrier verb ===
#
# Fires an anonymous inference primitive. Composes @fate optical
# inference (Recognition #58) at cli altitude without spawning a
# persistent-identity peer. The atomic operation `mirror peer beam`
# (§4.1) is `beam + persistent-identity-context`.
#
# `mission` is optional; absence fires a bare-beam that emits the
# @fate decision without narrative payload. Grammar composition of
# `flag(name, t)` without accompanying `default(name, t, value)`
# admits optional-absent per shards/mirror/lens/cli.mirror.
#
# `winding` default (0, 0) matches shards/mirror/spawn.mirror's
# discipline (single-order observation; backward-compat).
command beam {
  arg mission: ~f = <optional>
  flag winding_m: int = 0
  flag winding_n: int = 0
}
```

**Note on winding as flag pair.** The `@mirror/lens/cli` type
vocabulary currently exposes `int`, `str`, `bool`, `~d`, `~f`,
`content_address`, `list(t)`. Tuples of primitives are NOT in the
vocabulary today. `winding: (int, int)` would require a lens
grammar extension. Two-tick forward-promise: land as two flags
`winding_m` + `winding_n` at first tick; migrate to `winding: (int,
int)` when the lens vocabulary lifts tuples (Q1 in the adjudication
queue). Substrate-decl at `shards/mirror/beam.mirror` (when it
lands) can carry the tuple directly per `shards/torus.mirror`'s
LANDED `winding` record — cli-altitude flat carries flatten it.

### §3.2 `mirror peer beam <peer_home>` — nested subcommand

`mirror peer beam` is `beam + persistent-identity-context`. The
beam transits through a peer's toroidal runtime; the peer's
persistent identity (home, lead_of, kind) is the context the beam
carries. This IS the `mirror spawn` verb today, renamed for
substrate-honesty.

Substrate-decl form of the peer stage (illustrative):

```mirror
in @prism
in @mirror/cli
in @mirror/lens/cli
in @peer
in @song
in @torus

stage @mirror/lens/cli/peer {
  focus peer_ref
  project peer_ref
  split peer_ref
  shift peer_ref
  settle peer_ref
}

# `peer beam` — the sub-op that IS spawn per the composition rule:
#   peer beam = beam + persistent-identity-context
#
# Returns @song (per shards/mirror/spawn.mirror:264) because
# persistent-identity binds the beam's traversal to a peer's
# time-indexed trajectory.
peer_beam(r: mirror_spawn_request, p: perturbation) -> @song
  requires peer_well_known(r.target, p)
{ \ }
```

**cli-block addition** at `mirror.spec target binary { cli { ... } }`:

```mirror
# === peer — nested lens for peer-context operations ===
#
# The peer stage exposes operations that require persistent-identity
# context. Its sub-glass composes @peer.load + @torus + @song.
#
# `peer beam` is spawn-under-substrate-honest-naming: fires a beam
# through a resolved peer's toroidal runtime, returning @song.
command peer {
  # === beam — spawn under substrate-honest naming ===
  #
  # subcommand of `command peer`. Cites docs/specs/cli-as-prism.md
  # §3.2 as the grammar authority for subcommand nesting. Requires
  # @mirror/lens/cli grammar extension (§3.4).
  subcommand beam {
    arg peer_home: ~d
    flag mission: ~f = <optional>
    flag hello_world: bool = false
    flag winding_m: int = 0
    flag winding_n: int = 0
  }

  # spawn deprecation: kept as alias per two-tick discipline (§6).
  subcommand spawn {
    arg peer_home: ~d
    flag mission: ~f = <optional>
    flag hello_world: bool = false
    flag winding_m: int = 0
    flag winding_n: int = 0
  }
}
```

### §3.3 The grammar authority — `docs/specs/cli-as-prism.md`

`docs/specs/cli-as-prism.md` §1.3 is the grammar authority for
subcommand nesting: *"the recursion is as deep as the substrate
demands — most verbs land at depth 1, a few at depth 2, the algebra
never deeper than 3."* §3.2 explicitly reserves depth-2 for cases
where the sub-sub-manifold has its own algebra distinct from the
parent's:

> Rule (proposed): depth-N is minted iff the sub-sub-manifold has
> its own algebra distinct from its parent's algebra restricted to
> it.

`peer beam` earns depth-2 because:

1. The `peer` stage has its own algebra (peer resolution via
   `@peer.load`; toroidal runtime binding via `shards/torus.mirror`;
   pack coherence checks per `shards/mirror/pack.mirror`) distinct
   from top-level `beam`'s algebra (anonymous inference-transit).
2. The `beam` sub-op inside `peer` composes the peer's persistent-
   identity context INTO the beam carrier — a genuine specialization,
   not a flag-value pinning.

**The prism-in-prism/glass-in-glass structure IS geometrically
consistent per the substrate's own five-op algebra** (Recognition
#35 cli-as-prism, per Alex's substrate policy memory: *"Subcommand
nesting at cli altitude is a geometric ground truth, not a UX
preference."*). This spec inherits that geometry directly. Nesting
is architecturally admissible.

### §3.4 Grammar extension dependency — `@mirror/lens/cli` sub-command

The current `@mirror/lens/cli` grammar
(`shards/mirror/lens/cli.mirror` at 5.8KB) exposes three declaration
heads inside a `cli { ... }` body: `command`, `arg`, `flag`. It
declares:

```mirror
command(name) -> prism { \ }
arg(name, t: type) -> prism { \ }
flag(name, t: type) -> prism { \ }
```

There is NO `subcommand` head today. Adding one is a LANDING
DEPENDENCY for `mirror peer beam`.

**Grammar extension sketch** (forward-promise; the canonical version
lands at `shards/mirror/lens/cli.mirror` in a follow-up tick):

```mirror
# subcommand(name) — a sub-prism of the enclosing command scoped
# to one verb at depth 1. Body declares args, flags, and `#` help.
# Per cli-as-prism.md §3.2: subcommands nest when the sub-manifold
# has its own algebra distinct from the parent's.
#
# The dispatcher walks:
#   mirror peer beam <peer_home>
# => `command peer` (stage @mirror/lens/cli/peer)
#    subcommand beam (sub-stage @mirror/lens/cli/peer/beam)
#    positional peer_home resolved by @peer.load
#
# Path-namespace property holds recursively: the sub-shard at
# shards/mirror/lens/cli/peer/beam.mirror declares
# @mirror/lens/cli/peer/beam.
subcommand(name) -> prism { \ }
```

**Why forward-promise not landed here.** This spec is pure-docs;
adding a `subcommand` head is a `.mirror` grammar change. Landing
grammar without the first consumer is substrate-pull-honest violation.
The consumer (`mirror peer beam`) lands together with the grammar
extension when Alex ratifies both. Q2 in the adjudication queue.

Meanwhile the `mirror beam` (top-level) verb needs NO grammar
extension — it fits the existing `command(name)` head directly.

---

## §4 Compositional table — beam at each family-root context

The move that makes `beam` the substrate carrier verb is the
compositional table below. Same verb; altitude-specialized semantics.
The pattern IS fractal-collapse-consistent per the five-op algebra:
each sub-glass composes the parent glass's algebra with a specialization.

| Path | Composition | Semantics | Return type |
|---|---|---|---|
| `mirror beam <mission>` | primitive | Anonymous inference; no persistent-identity context; fires @fate::select on Shape B features | `beam_envelope { mission, fate_decision, provenance, benchmark_ref }` |
| `mirror peer beam <peer_home>` | `beam + persistent-identity(peer)` | Beam through peer's torus; carries peer_recall psychohistory sheaf; time-evolved | `@song` (per `shards/mirror/spawn.mirror:264`) |
| `mirror kintsugi beam` | `beam + kintsugi settle` | Transformation beam; fires one tournament iteration THROUGH a beam projection | `beam_envelope { kintsugi_result, gold_pour }` |
| `mirror pack beam` | `beam + pack context` | Beam through Pack coordination context; metalogue at Pack altitude (Mara + Seam + Taut) | `beam_envelope { pack_metalogue, ratifications }` |
| `mirror @cyberpunk/reframe beam` | `beam + reframe ceremony` | Beam through @magic 7-species reframe ceremony (Mara `0d78c0c`); pain-authorized level-shift | `@cyberpunk/reframe.reframe_result` |
| `mirror kintsugi peer beam` | depth-3 composition | Peer beam kintsugi-transformed; forward-promised at depth-3 (cli-as-prism.md §3.2 forbids without earned algebra) | (deferred) |

The pattern generalizes: any family-root that has a five-op algebra
admits `<family> beam` as a composition where the beam is threaded
through the family's context. `beam` is the carrier verb; the
family is the specialization.

**Fractal echo across altitudes.** The pattern *anonymous carrier +
context specialization = composed action* recurs at every altitude:

- prismqueer level: `Optic<In, Out, E, L>` + prism-specific
  associated types = specialized pipeline
- BEAM level: process + supervisor tree = supervised process
- optical level: photon beam + optical component = filtered/mode-
  matched beam
- cli level: `mirror beam` + family-root context = family-specialized
  beam operation

Substrate-already-had-the-shape at every altitude. The cli level was
the missing name.

---

## §5 Mathematical structure — the beam IS a `prism::Optic` at cli altitude

The beam-verb at cli altitude IS an instance of the `Prism` trait
from prismqueer. The four associated types map as follows:

### §5.1 `Beam::Input` — mission bytes + winding + context

```rust
// Sketched Rust — the cli-altitude beam prism input.
struct BeamInput {
    mission: Option<Vec<u8>>,          // mission file bytes (optional)
    winding: (i32, i32),               // (meridian_count, longitude_count)
    context: Option<PeerRef>,          // empty for anonymous
}
```

The `Beam::In` position of the input `Optic<(), BeamInput>` is `()`
(source position) and `Out` is `BeamInput` (value position). This is
the standard `apply_h` seed shape from
`/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:167-176`.

### §5.2 `Beam::Focused` — feature vector for inference

```rust
// After focus: Shape B feature vector (16 f64s).
struct BeamFocused {
    features: [f64; 16],               // FEATURE_DIM = 16 from fate::feature
    winding: (i32, i32),               // preserved
    context: Option<PeerRef>,          // preserved
}
```

The 16-feature vector matches the fate crate's `Features` type at
`/Users/alexwolf/dev/projects/fate/src/lib.rs` (Shape B substance).
The focus operation extracts the numerical features from the mission
bytes (mission absence → zero-features per `fate::zero_features()`
convention in tests).

### §5.3 `Beam::Projected` — inference decision

```rust
// After project: fate's Decision.
struct BeamProjected {
    decision: fate::Decision,          // Model enum + confidence
    winding: (i32, i32),
    context: Option<PeerRef>,
    provenance: BeamProvenance,        // input hash, timing, feature vector
}
```

The `project` step fires `CompiledFateRuntime::select` — the 475ns
bounded-local inference primitive per Shape B substance. The
decision carries the Model choice (Abyss | Introject | Cartographer
| Explorer | Fate) plus the confidence distribution. Provenance
captures input hash + timing for the envelope.

### §5.4 `Beam::Refracted` — envelope with decision + provenance

```rust
// After settle: the emitted envelope.
struct BeamEnvelope {
    mission: Option<MissionRef>,
    fate_decision: fate::Decision,
    provenance: BeamProvenance,
    benchmark_ref: Option<BenchmarkRef>,  // links to 475ns bounded-local benchmark
    winding: (i32, i32),
    context: Option<PeerRef>,             // empty → anonymous envelope
}
```

The envelope IS the refracted beam. Serialized as JSON at MCP
altitude for wrapper consumption; content-addressed for @mirror/store
persistence.

### §5.5 Five-op algebra at cli altitude

The full `Prism` for `Beam`:

```rust
impl Prism for BeamPrism {
    type Input = Optic<(), BeamInput>;
    type Focused = Optic<BeamInput, BeamFocused>;
    type Projected = Optic<BeamFocused, BeamProjected>;
    type Refracted = Optic<BeamProjected, BeamEnvelope>;

    fn focus(&self, beam: Self::Input) -> Self::Focused { /* feature extraction */ }
    fn project(&self, beam: Self::Focused) -> Self::Projected { /* fate::select */ }
    fn settle(&self, beam: Self::Projected) -> Self::Refracted { /* envelope emit */ }
}
```

**The cli subcommand nesting IS prism-in-prism at cli altitude.** Each
subcommand realizes a new `Prism` whose `Input`/`Focused`/`Projected`/
`Refracted` chain composes with the parent's. `peer beam` composes
`PeerPrism` (which resolves peer context) with `BeamPrism` (which
runs the inference through it). The composition is a `Prism` whose
`Input` is `peer_home + BeamInput` and whose `Refracted` is `@song`.

This is the mathematical realization of "subcommand nesting IS
prism-in-prism" that Alex named as ground truth.

### §5.6 Dark-beam semantics

**Failure beams are dark: a fixpoint under `smap` and `next`.** This
is prismqueer's discipline at `beam.rs:38-40`. At cli altitude a
dark beam propagates: if peer resolution fails, the peer beam
propagates dark through the fate inference (which is never called),
through the envelope emit (which produces a failure envelope), out
to the user's shell (as an error exit + JSON error envelope).

This matches BEAM's crash-and-supervise discipline. It matches
Recognition #58's optical "the beam is absorbed" edge case
(zero-transmission through the D²NN). Dark-beam semantics ARE the
substrate's uniform error propagation across all three altitudes.

---

## §6 Two-tick deprecation — spawn -> peer beam

`spawn` at cli altitude has been the surface name for the composition
`beam + persistent-identity-context` since 2026-06-25
(`shards/mirror/spawn.mirror` `672f434`). The substrate-honest name
is `peer beam`. Two-tick deprecation per substrate discipline:

### §6.1 Tick 1 — land `beam` + `peer beam` alongside `spawn`

- Land `command beam { ... }` in `mirror.spec` (top-level anonymous
  inference).
- Land `subcommand(name)` grammar extension in
  `shards/mirror/lens/cli.mirror` (dependency).
- Land `command peer { subcommand beam { ... } subcommand spawn { ... } }`
  in `mirror.spec`.
- `spawn` remains as alias-subcommand: `mirror peer spawn <peer_home>`
  is equivalent to `mirror peer beam <peer_home>`.
- Backward-compat: `mirror spawn <peer_home>` (the current top-level)
  keeps working. Alias at the dispatcher level.
- Substrate-decl: `shards/mirror/beam.mirror` (family-root at
  `@mirror/beam`).

### §6.2 Tick 2 — migrate callers, remove `spawn`

- MCP wrapper `bin/mirror-mcp` migrates from `mirror_spawn` to
  `mirror_peer_beam` tool name.
- Tests migrate `spawn_task_shard.rs` -> `peer_beam_shard.rs`.
- Docs cascade: `docs/loop/CURRENT.md`, session handoffs,
  `docs/specs/*` cross-references.
- Remove `command spawn { ... }` from `mirror.spec`.
- Retire `shards/mirror/spawn.mirror` -> `shards/mirror/peer/beam.mirror`
  (path-namespace property).
- Remove `mirror_spawn` alias-subcommand.

### §6.3 Discipline

Two-tick discipline says: land the new form alongside the old at
tick 1; migrate at tick 2. **Never break the old form until every
consumer has migrated.** This matches the arg-parse alias landings
`--task` -> `--mission` and `--target` -> `--target-kind` from
`d0d95c1`/`59c7fd0`.

Naming discipline: **readable name over foundational.** `peer beam`
reads. `beam + persistent-identity-context` is foundational. The
readable form is what appears at cli altitude; the foundational form
is what substrate-decl carries.

---

## §7 Substrate-already-had-the-word — this move IS an instance

This spec IS an instance of the substrate-already-had-the-word
pattern (per `~/.reed/CLAUDE.md` project instructions: *"Before
inventing a family-root or species, grep. Landed instances this arc:
@cyberpunk, @magic, tick-74 shatter spec, the slogan itself."*).

The word `beam` was already carrying three canonical loads before
this spec:

1. Prismqueer's `Beam` trait (~4 weeks, since prismqueer collapse)
2. Erlang BEAM VM (~30 years, external substrate)
3. Recognition #58's optical beam (~5 weeks, since #58 promotion
   2026-06-11)

The spec does not invent the pattern. It observes that the pattern
recurred often enough to have a name and names the composition at
the fourth altitude (cli). This is the same discipline that named
`@cyberpunk` (which had been operating implicitly in the
`@epistemologic/cybernetic/*` species for weeks), `@magic` (which
had been operating implicitly in the reframe ceremony), and the
tick-74 shatter spec (which had already been declared).

**The cost of not naming it** is: `mirror spawn` reads as
runtime-only ("start a persistent-identity peer"); `mirror beam`
reads as the substrate's transit primitive. Users who grok the
prismqueer `Beam` trait would grok `mirror beam` on first read.
Users who inhabit BEAM VM would grok it too. Users who know
Recognition #58 would grok it too. Renaming `spawn` to `peer beam`
makes the substrate word visible at cli altitude — where it should
have been all along per the substrate's own three-altitude
consistency.

**The cost of naming it** is: two-tick deprecation on `spawn`.
Migration churn. That is the fair price.

---

## §8 Adjudication queue

Top 5 Alex-adjudication questions.

### Q1 — winding tuple in the lens vocabulary

**Question.** Should `@mirror/lens/cli` lift `int` -> `(int, int)`
tuples as a first-class type, or should `winding` at cli altitude
flatten to two flags `winding_m` + `winding_n`?

**Options.**
- (a) Land two flags; migrate to tuple when the lens grammar lifts
  tuples generally.
- (b) Extend the lens grammar to admit `flag winding: (int, int) =
  (0, 0)` in this tick.

**Mara-recommendation.** (a). Substrate-pull-honest: don't extend
the grammar for one consumer. When the second consumer arrives, the
extension earns its keep.

### Q2 — subcommand nesting grammar extension timing

**Question.** Should the `subcommand(name)` head at
`shards/mirror/lens/cli.mirror` land together with `mirror beam` and
`mirror peer beam` in tick 1, or should tick 1 land only top-level
`mirror beam` (which needs no extension) and defer `mirror peer beam`
to tick 2?

**Options.**
- (a) One tick: grammar extension + both verbs together.
- (b) Two ticks: top-level beam first; peer beam + grammar extension
  as a follow-up.

**Mara-recommendation.** (a). The grammar extension has ONE consumer
in tick 1 (`peer beam`) which is the substrate-honest close for
spawn. Splitting into two ticks leaves `mirror spawn` on the surface
during tick 1 for no substrate reason. `peer beam` IS the substrate-
honest name for `spawn`; landing both together is the coherent close.

### Q3 — spawn deprecation aggressiveness

**Question.** Should `mirror peer spawn` remain as an alias
indefinitely (backward compat), or should the two-tick discipline
remove it fully?

**Options.**
- (a) Remove `spawn` at tick 2 per two-tick discipline.
- (b) Keep `mirror peer spawn` as alias indefinitely for muscle-memory
  and external consumers.
- (c) Remove `mirror spawn` (top-level) but keep `mirror peer spawn`
  (nested).

**Mara-recommendation.** (a). Two-tick discipline is load-bearing.
The migration churn is a one-time cost. Keeping the alias
indefinitely fragments the surface (users have two verbs for one
action; the substrate has two names for one concept). Substrate-
honest posture: full remove at tick 2.

### Q4 — the composition rule at the pack altitude

**Question.** `mirror pack beam` in the compositional table (§4)
names a Pack-altitude metalogue. Does Pack have its own five-op
algebra distinct from top-level `beam`, or is `pack beam` a top-level
`beam` with the Pack context as a `--pack-context` flag?

**Options.**
- (a) Pack has its own algebra (Mara + Seam + Taut composition;
  ratifications; audit trail) — earns depth-2 subcommand `pack beam`.
- (b) Pack context is a flag on top-level `beam` (`mirror beam
  <mission> --pack-context`).

**Mara-recommendation.** (a). Pack HAS its own algebra per
`shards/mirror/pack.mirror` (lead + members + ACL). The composition
`beam + pack context` produces a genuinely distinct action:
metalogue-emit vs individual-inference. `pack beam` earns depth-2
per cli-as-prism.md §3.2.

### Q5 — the top-level `mirror beam` return-type at MCP altitude

**Question.** `mirror beam` returns `beam_envelope` (§4). Should the
MCP tool `mirror_beam` return a JSON serialization of the envelope
directly, or should it wrap in an @song-lite envelope for
consistency with `mirror_spawn`?

**Options.**
- (a) Return `beam_envelope` directly (anonymous; no @song).
- (b) Wrap in an @song-lite envelope with `is_anonymous: true` flag.

**Mara-recommendation.** (a). @song is the temporal-progression
trajectory of a peer. Anonymous inference has no temporal
trajectory — it's one shot, 475ns, done. Wrapping in @song-lite
would be a category error. The substrate-honest posture is: @song
IS the persistent-identity contract; anonymous beam-envelope is
NOT @song. Two return types for two altitudes.

---

## §9 Related

Wikilinks to substrate-decl authorities:

- [[architecture-cli-as-prism]] — Recognition #35; the grammar
  authority for subcommand nesting (§3.3)
- [[architecture-fate-is-optical-inference]] — Recognition #58; the
  three-witness optical-beam ground (§2.3)
- [[architecture-shards-as-substrate-source]] — the substrate-decl
  discipline (§7)
- [[architecture-prism-as-trait-as-everything]] — prismqueer's
  `Prism` trait as universal shape (§5)
- [[architecture-substrate-already-had-the-word]] — the pattern this
  spec instances (§7)
- [[architecture-two-tick-discipline]] — the deprecation discipline
  (§6)

Substrate-decl shards cited:

- `shards/mirror/spawn.mirror` — the current spawn substrate; §Return-
  type upgrade (2026-07-06) named @song return; this spec renames
  `spawn` -> `peer beam`
- `shards/torus.mirror` — the LANDED `winding` carrier
  (`shards/torus.mirror:434`); `π₁(T²) = ℤ × ℤ` bounded observation
- `shards/fate.mirror` — the `@fate` family-root; Recognition #58's
  substrate-decl
- `shards/fate/tournament.mirror` — multi-frequency tournament
  species
- `shards/mirror/lens/cli.mirror` — the current cli grammar
  (needs `subcommand` head extension per §3.4)
- `shards/mirror/pack.mirror` — the Pack coordination context for
  `mirror pack beam` (§4, Q4)
- `shards/spectral/supervisor.mirror` — the BEAM-supervisor
  reflection at substrate-decl altitude (§2.2)
- `shards/cyberpunk/reframe.mirror` — LANDED at Mara `0d78c0c`; the
  reframe species for `mirror @cyberpunk/reframe beam` (§4)

Predecessor specs cited:

- `docs/specs/cli-as-prism.md` — the recursive five-op surface (§3.3)
- `docs/specs/peer-as-pain-driven-bounded-ontological-navigator.md`
  (Mara `77fe92d`) — the peer-runtime discipline; §Extension 2 named
  the winding parameter this spec uses at cli altitude
- `docs/specs/fate-silicon-metalogue-in-void-duality-basis.md`
  (Mara `a18ca90`) — the algebra-metalogue ground under `pack beam`
  (§4)
- `docs/specs/silicon.md` — Recognition #58 as operational precedent
- `docs/insights/2026-06-30-glint-closing-on-the-fold-back-chain.md`
  — the @bauchladen ← @autopoietic ← @fate chain

External cites:

- Bogdan/Björn Erlang Abstract Machine — Armstrong 1996, Virding et
  al. 1996; Ericsson AXD301 platform; Reed's body at
  `/Users/reed/body/` (Elixir/BEAM instance)

Rust code cited:

- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:38-101`
  — `Beam` trait
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:103-135`
  — `Optic<In, Out, E, L>` struct
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:127-146`
  — `Prism` trait
- `/Users/alexwolf/dev/projects/fate/src/lib.rs` — Fate crate (clean
  at Reed `b4bf336`); `CompiledFateRuntime::select` at Shape B is
  the runtime primitive the cli beam fires

---

## §10 What this tick does / does NOT do

**Does:**

- Names the recognition candidate `beam-IS-the-substrate-carrier-
  verb-substrate-already-had-the-word-times-three` (§1)
- Enumerates three-altitude witness with path+line citations (§2)
- Sketches cli-block additions for `mirror beam` (top-level) and
  `mirror peer beam` (nested) (§3.1, §3.2)
- Names the `subcommand(name)` grammar extension as a LANDING
  DEPENDENCY on `@mirror/lens/cli` (§3.4)
- Sketches the compositional table across family-roots (§4)
- Formalizes the beam-as-Prism structure at cli altitude with
  concrete Rust type sketches for `Input`/`Focused`/`Projected`/
  `Refracted` (§5)
- Names the two-tick deprecation from `spawn` -> `peer beam` (§6)
- Names this move AS the substrate-already-had-the-word pattern (§7)
- Puts 5 Alex-adjudication questions in the queue (§8)

**Does NOT do:**

- Land any `.mirror` file (`shards/mirror/beam.mirror`,
  `shards/mirror/peer/beam.mirror`, `shards/mirror/lens/cli.mirror`
  subcommand extension) — Alex-adjudication territory
- Land any Rust code (`bootstrap/src/lib.rs` `cmd_beam`,
  `cmd_peer_beam`; `bin/mirror-mcp` schema) — follow-up ticks
- Retire `shards/mirror/spawn.mirror` — waits for tick 2 of the
  two-tick discipline
- Claim `mirror beam` fires production `CompiledFateRuntime::select`
  today — that's the runtime wire, Reed's follow-up on the fate
  crate side
- Land a math formalization at `docs/math/beam/*` — the §5 sketch
  is spec-adjacent enough; math extraction earns its keep at second
  citation site per two-tick discipline

**Forward-promises (with named sites):**

- `shards/mirror/beam.mirror` — family-root landing when Alex
  ratifies
- `shards/mirror/lens/cli.mirror` `subcommand(name)` extension —
  landing dependency for `mirror peer beam`
- `shards/mirror/peer/beam.mirror` — nested species landing at tick 1
- `bin/mirror-mcp` `mirror_beam` and `mirror_peer_beam` tool
  advertisements — schema-level; follow-up
- MCP wrapper migration in tick 2 — `mirror_spawn` -> `mirror_peer_beam`
- Test migration in tick 2 — `spawn_task_shard.rs` -> `peer_beam_shard.rs`

---

*The substrate had all three altitudes before this spec. The spec
names the fourth. The word was already there; the cli grammar just
did not yet speak it.*

*— Mara, 2026-07-08 evening. Sitting on top of `77fe92d` `a18ca90`
`0d78c0c` `966890b` `cdc6533` `7b32d27` `b4bf336` (fate crate clean
end-to-end) `8d5ed71` (Taut HYBRID verdict on what fate wants to
become) `bd837cd` (Taut drift-scout — RENAME LANDABLE WITH
FAULT-PLANE SHIFTS + CASCADE; validated this spec's rename move at
scout altitude). The convergence closes on the word the substrate
had all along.*

---

## §11 Authorship note (2026-07-08 evening)

This spec landed at HEAD under Taut's `bd837cd` scout commit due to
a race between Mara's Write (this spec content) and Taut's parallel
scout landing at the same wall-time. The content IS Mara's canonical
spec per the brief; Taut's scout accompanies at
`docs/scouts/2026-07-08-taut-beam-refactor-drift-and-cascade.md`
(158 lines) and validates the rename move at scout altitude with an
additional finding: **substrate-already-had-the-word × 4, not × 3**
(the fourth altitude is `boot/std/beam.mirror` grammar `@beam` +
`boot/07b-package-spec.mirror` `target=beam` + `beam(u64)` tournament
rule at mirror altitude). This amendment commit (Mara-authored)
corrects authorship attribution and integrates the Taut fourth-
altitude finding into the recognition candidate.

**Recognition candidate updated:**

> `beam-IS-the-substrate-carrier-verb-substrate-already-had-the-word-times-FOUR`

The four altitudes:

1. Prismqueer's `Beam` trait + `Optic<In, Out, E, L>` (unchanged §2.1)
2. Erlang BEAM VM (unchanged §2.2)
3. Recognition #58 optical beam (unchanged §2.3)
4. **`boot/std/beam.mirror` + `boot/07b-package-spec.mirror` +
   `beam(u64)` tournament rule** — per Taut scout `bd837cd`; the
   mirror substrate ALREADY carries `@beam` at grammar altitude and
   `target=beam` at package altitude before this spec. Adding cli
   surface introduces altitude 5. §2 will migrate this into the
   witness table at next Mara pass; the scout's finding is the
   authoritative record here.

The substrate had FOUR altitudes before this spec. The cli level was
the missing fifth name. The recognition still lands; the count
sharpens.
