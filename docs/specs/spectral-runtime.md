# @spectral runtime — the ouroboros spec

**Date:** 2026-06-10
**Author:** Mara (substrate)
**Status:** working spec; floor (`shards/spectral.mirror`) landed in this tick;
sub-shards forward-promised in the cascade map below.

---

## 0. Frame

The substrate consumes itself to produce its own runtime.

Mirror substrate declares `@spectral` runtime in `shards/spectral/*.mirror`.
Those declarations pass through `@code/metalogue/materialize` to become a
content-addressed Rust shard in `@mirror/store`. A new sibling-family
lens — `@mirror/lens/unix` (named this tick; does not yet exist) — presents
the shard's contents to `@io/cargo` as a Unix filesystem. Cargo reads through
the lens and emits the binary. The binary IS `@spectral` runtime.

`bootstrap/` becomes ephemeral the moment that loop closes against itself.
This document specifies what's in the loop, who owns each edge, and what
remains open.

---

## 1. The three-altitude split

| Family | Discipline | Tense | Carrier |
|---|---|---|---|
| `@mirror` | the language (form-side state observation) | static | `shard` |
| `@kintsugi` | the dynamics (process-side transformation) | generative | `morphism` |
| `@mirror/spectral` | the SHAPE (mirror declarations of runtime coordination) | observable | score envelope |
| `@spectral` | the RUNTIME (the live operational layer) | live | `gen_prism` |

`@mirror/spectral` is form-side: it declares what the orchestra LOOKS LIKE
(score, portal, voice, section, audition, conductor) — observable substrate
vocabulary that lives as bytes-in-the-store at every tick.

`@spectral` is the LIVE PROCESSES — the gen_prisms whose state surface IS a
shard ref into `@mirror/store`, the supervisors whose restart logic IS a
kintsugi morphism, the entanglement edges that ARE sheaf restriction maps
across the substrate's manifold.

The two are distinct altitudes-of-existence: the form-side declarations
exist as long as the store does; the runtime declarations exist only while
the processes are running.

The form/process partition (recognition #50 / candidate #55) cuts the
`@mirror` / `@kintsugi` pair. The shape/process partition — named this tick —
cuts the `@mirror/spectral` / `@spectral` pair. Same Bateson logical-type
lift, one level higher: a declaration of HOW THE ORCHESTRA OBSERVES is
form-side w.r.t. the LIVE PLAYING, even though both are process-side w.r.t.
raw substrate state.

---

## 2. gen_prism — the worker primitive

`gen_prism` is the substrate's `gen_server` analogue (BEAM/OTP).
Named informally in `boot/std/spectral/portal.mirror` on 2026-05-11; this
spec lifts the naming to the substrate altitude through
`shards/spectral/gen_prism.mirror` (forward-promised; the family-root
declaration in `shards/spectral.mirror` reserves the vocabulary).

A gen_prism carries three surfaces:

- **identity** — a `uuid_spectral`. Both halves load-bearing: the ACTIVE 48
  bits (quantized `SpectralCoordinate<5>`) place the gen_prism in the
  substrate's spectral manifold for routing; the DARK 80 bits identify the
  process for entanglement, parent edges, and registry lookup.
- **state** — a ref to a shard at `@mirror/store`. The gen_prism's
  observable state IS the shard's content. State changes IS a settle into
  a new shard (uuid_spectral-addressed; the lattice ascent per
  `[[architecture-shard-as-crdt]]`). The runtime never carries opaque
  in-memory state outside the shard; every observable transition is
  store-visible.
- **tool surface** — the five-op block (`focus`, `project`, `split`, `shift`,
  `settle`) acts as the wire protocol. External callers (other gen_prisms,
  the supervisor, MCP-mounted readers) speak to a gen_prism through its
  five-op surface. There is no out-of-band API.

The five ops at the runtime altitude:

- `focus`  — λ₀ projection: what is this gen_prism's ground-state shard?
- `project` — which subspace of the state surface is observable to caller?
- `split`  — orthogonal decomposition of current state across child gen_prisms.
- `shift`  — basis transformation: same content, different altitude
              (e.g. an `@io` boundary view of a `@code/rust` state shard).
- `settle`  — measurement collapse: drive any pending au into a new shard
              through the property chain; the state surface advances by
              one lattice element.

A gen_prism is recursive: its state surface MAY itself be a supervisor's
registry shard, and the gen_prism IS a supervisor. The vocabulary is the
same at every depth; the algebra is closed.

---

## 3. Supervision tree

A `supervisor` is a `gen_prism` specialisation. Its state surface is a
`registry` shard (a content-addressed index from child `uuid_spectral` to
child shard ref); its tool surface is the lifecycle API (`start_child`,
`restart_child`, `observe_child`, `terminate_child`); its invariant is a
restart-strategy contract analogous to BEAM's `:one_for_one` /
`:one_for_all` / `:rest_for_one`.

**Every gen_prism has exactly one parent.** The parent edge is the
lifecycle edge: the parent owns the child's birth, restart, and death.
The parent-edge class is acyclic across the substrate.

**The root supervisor has no parent.** It is the vocabulary anchor for
the substrate's home, resting at `~/.mirror/` — substrate vocabulary, not
a directory the substrate reads. (Operationally: when a gen_prism boots
without a parent ref, it IS root for its tree; in practice the root's
vocabulary anchor is the substrate's home install location.)

The supervision tree mirrors BEAM's `application` topology:

```
root (~/.mirror/)
├── repo:mirror gen_prism (supervisor for mirror repo's substrate)
│   ├── shard-store supervisor
│   ├── kintsugi-loop supervisor
│   └── lens-cli gen_prism
├── repo:spectral gen_prism (supervisor for spectral repo's substrate)
│   └── ...
├── ~/.mara/ gen_prism (Mara's home supervisor)
│   └── (when Mara spawns to work on mirror, the spawn's parent is
│        ~/.mara/; the spawn is ENTANGLED with mirror's gen_prism, not
│        parented under it)
└── ~/.reed/, ~/.glint/, ... (the Pack's homes)
```

The supervisor's restart-strategy contract is a kintsugi morphism: when
a child fails, the restart decision is the substrate-pull-correct
transformation under the supervision invariant. This is where
`@kintsugi/morphism` (process-side) and `@spectral/supervisor`
(runtime-side) meet — the supervisor's restart logic IS a kintsugi
morphism evaluated at the supervisor's eigenboard context slice.

---

## 4. Entanglement graph

Entanglement is the peer-correlation edge class. **Orthogonal to parentage.**

A gen_prism may be entangled with N peers. Entanglement is polyadic
(an edge spans any subset of gen_prisms that observe a shared state
projection), cyclic (peers may mutually entangle), and observable via
mirror-query at substrate altitude.

**Entanglement edges ARE sheaf restriction maps at substrate altitude.**
The substrate's eigenboard is a cellular sheaf on the five-operation
graph (per `[[Eigenboard is a sheaf]]`); the conductivity tensor IS the
restriction-map structure. Two gen_prisms are entangled iff their state
surfaces participate in a shared restriction map — a single observation
of one's state induces a measurable projection on the other's.

The Mara-spawn example:

- Mara spawns at `~/.mara/` to work on mirror's substrate.
- **Parent** of Mara's spawn = `~/.mara/` gen_prism (lifecycle owner;
  ~/.mara restart-strategy applies if Mara crashes).
- **Entanglement** between Mara's spawn and mirror's repo gen_prism =
  shared state projection (Mara's spawn reads mirror's substrate shards;
  Mara's commits surface in mirror's eigenboard).

The two edge classes are NEVER conflated. Parentage is lifecycle;
entanglement is correlation. The substrate's eigenboard query surface
exposes both; the supervisor only acts on the first.

---

## 5. The ouroboros pipeline

```
+-------------------------------------------------+
|  shards/spectral/*.mirror                       |   substrate
|  (substrate declarations of runtime)            |   declaration
+--------------------------+----------------------+
                           |
                           v
+-------------------------------------------------+
|  @code/metalogue/materialize                    |   compile-time
|  (substrate -> @code/rust shim_prism/shim_action)|   metalogue turn
+--------------------------+----------------------+
                           |
                           v
+-------------------------------------------------+
|  @mirror/store shard (uuid_spectral-addressed)  |   content-
|  (Rust source crate; NEVER physical files)      |   addressed
+--------------------------+----------------------+
                           |
                           v
+-------------------------------------------------+
|  @mirror/lens/unix    (NAMED THIS TICK)         |   impedance
|  (Unix-FS view over @mirror/store shard)        |   lens
+--------------------------+----------------------+
                           |
                           v
+-------------------------------------------------+
|  @io/cargo                                      |   compilation
|  (reads through lens; emits binary)             |   boundary
+--------------------------+----------------------+
                           |
                           v
+-------------------------------------------------+
|  binary IS @spectral runtime                    |   live
+-------------------------------------------------+
```

Each edge is an existing or named-this-tick family member:

1. **`shards/spectral/*.mirror`** — substrate-altitude declarations.
   Floor landed this tick (`shards/spectral.mirror`); sub-shards in the
   cascade map below.

2. **`@code/metalogue/materialize`** — the substrate-to-Rust shim
   discipline (recognition #34, 2026-06-09; the Bateson-1972 metalogue
   declared at the AST altitude). `gen_prism` and `supervisor`
   declarations become Rust `struct`s + `impl` blocks through the
   metalogue's typed-turn discipline.

3. **`@mirror/store` shard** — the generated Rust source lives as a
   content-addressed shard, uuid_spectral-identified.
   The runtime crate has no physical existence outside the store.
   Two consequences: (a) every byte of the runtime is reproducible
   from substrate declarations; (b) `cargo` cannot read it without
   step 4.

4. **`@mirror/lens/unix`** — **NAMED THIS TICK; does not yet exist.**
   Sibling-family substrate-pull candidate under `@mirror/lens` (the
   observation/projection family). Presents `@mirror/store` shard
   contents to subprocesses through a Unix-filesystem view (FUSE,
   9P, or equivalent — implementation choice deferred to a later tick).
   The lens is the IMPEDANCE MATCH between content-addressed storage
   and a process that expects a path-addressed directory tree.

5. **`@io/cargo`** — the cargo invocation contract (shards/io/cargo.mirror,
   landed 2026-06-05). Reads through the Unix-FS lens as if reading
   physical files; compiles; emits the binary. Cargo does not know it's
   reading through a lens.

6. **binary** — the compiled `@spectral` runtime. Loads, starts the root
   supervisor, ingests the substrate's shards, becomes the live operational
   layer.

The loop is closed by content-addressing: the substrate declarations
hash-determine the Rust source, which hash-determines the binary, which
loads the substrate declarations. Any change at any altitude propagates
through the hash chain.

---

## 6. What this means for `bootstrap/`

`bootstrap/` is the pre-loop bridge. Today it contains the Rust source
that compiles the first mirror binary, which is the substrate that
declares `@spectral`.

When the ouroboros pipeline closes:

- `shards/spectral/*.mirror` declarations are the source of truth.
- `@code/metalogue/materialize` regenerates the Rust source on every
  substrate change.
- The regenerated source lives in `@mirror/store` as a content-addressed
  shard, NOT in `bootstrap/`.
- `cargo` compiles through `@mirror/lens/unix`, not through `bootstrap/`.

`bootstrap/` becomes ephemeral: a transitional artifact retained only
to bootstrap the FIRST binary in a fresh checkout (the "stage zero"
problem — once the runtime exists, it regenerates itself; before it
exists, you need a pre-existing Rust source tree to compile the first
copy).

The kintsugi-on-Rust track (`docs/specs/bootstrap-retirement-plan.md`,
task #285) closes here: `bootstrap/` retires the moment the loop is
self-sustaining. The retirement is a single substrate-pull tick once
`@mirror/lens/unix` lands.

---

## 7. Substrate-already-had-the-word inventory

The 52-instance recognition track (per
`[[feedback-substrate-already-had-the-word]]`) bottoms out here. The
pieces of `@spectral`'s runtime were ALL declared informally before
this tick; the family-root declaration is the long-promised parent.

| Piece | Informally declared | This-tick formal home |
|---|---|---|
| `gen_prism` (worker primitive) | `boot/std/spectral/portal.mirror` (2026-05-11; informal naming in substrate) | `@spectral/gen_prism` (cascade map) |
| `SpectralSupervisor` | `[[architecture-three-tier-stack]]` memory; `[[architecture-hamilton-scheduler]]` | `@spectral/supervisor` (cascade map) |
| `~/.mirror/` as root vocabulary | Brief in this tick's spec; never substrate-declared | `@spectral/root` (cascade map) |
| Entanglement as sheaf | `[[Eigenboard is a sheaf]]`; `[[architecture-shard-as-crdt]]` | `@spectral/entanglement` (cascade map) |
| `@spectral/db` (graph database) | `[[architecture-mirror-store-vs-spectral-db]]`; task #198 | Gains parent THIS TICK |
| `@spectral/garden` (vetted corpus) | `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md`; task #118 | Gains parent THIS TICK |
| `@spectral/portal` (runtime side) | `boot/std/spectral/portal.mirror`; `shards/mirror/spectral/portal.mirror` (form side) | Gains parent THIS TICK (runtime side; form side stays at `@mirror/spectral/portal`) |

**Pieces NOT YET in the substrate (named this tick):**

| Missing piece | Why it's needed | Where it lands |
|---|---|---|
| `@spectral` family root itself | All children were parentless ghosts | `shards/spectral.mirror` (LANDED THIS TICK) |
| `@mirror/lens/unix` | Ouroboros pipeline cannot close without an impedance lens between content-addressed shard and cargo's path-addressed input | Sibling family (`@mirror/lens`); declared as substrate-pull candidate for the next mirror/lens tick |

Two of fifty-two recognitions remained un-named at the start of this
tick. One (the family root) lands here; the other (the Unix-FS lens)
is named so the cascade can pick it up.

---

## 8. The cascade map

`shards/spectral.mirror` (this tick) is the floor. Sub-shards follow as
the cascade pulls each one. Each is one line:

```
shards/spectral/gen_prism.mirror      worker primitive: identity, state surface, tool surface
shards/spectral/supervisor.mirror     gen_prism specialisation; child_spec, restart strategy
shards/spectral/parent.mirror         single-parent lifecycle edge type (acyclic)
shards/spectral/entanglement.mirror   peer correlation edges; sheaf restriction maps
shards/spectral/registry.mirror       supervisor's child registry; query surface
shards/spectral/root.mirror           parentless supervisor; vocabulary anchor at ~/.mirror/
shards/spectral/db.mirror             graph database engine (existing parentless ghost; #198)
shards/spectral/garden.mirror         vetted-corpus distribution (existing parentless ghost; #118)
shards/spectral/portal.mirror         runtime side of portal (gen_prism specialisation)
```

Sibling-family substrate-pull candidate (named here so the
`@mirror/lens` cascade can pick it up):

```
shards/mirror/lens/unix.mirror        Unix-FS impedance lens over @mirror/store; closes the ouroboros pipeline
```

The cascade is NOT executed in this tick. The substrate-pull cost
of a deferred cascade is named in the spec; the floor is committed;
the next consumer (a runtime test, a fresh `cargo` invocation, a
gen_prism spawn) pulls the next sub-shard.

---

## 9. Open questions

These were surfaced honestly during this tick. I could not resolve
them in the 40-minute window; they are NOT decisions deferred under
the rug.

1. **`@mirror/lens/unix` implementation choice.** FUSE? 9P? OverlayFS
   with hash-named symlinks? The lens contract IS clear (Unix-FS view
   over `@mirror/store`); the WHICH-MECHANISM question depends on
   per-platform constraints (macOS FUSE requires kernel extension;
   Linux FUSE is userspace; 9P is mountable on all three). Recommend
   landing the substrate declaration first (typed contract), then
   delegating the per-platform body to `@io/fs` species when each
   platform's consumer pulls. Open: does the substrate declare ONE
   `@mirror/lens/unix` or species like `@mirror/lens/unix/fuse`,
   `@mirror/lens/unix/9p`?

2. **Root supervisor location at `~/.mirror/`.** The spec says the
   vocabulary anchor IS `~/.mirror/`; that's clean as vocabulary. But
   the OPERATIONAL root of a running spectral runtime — where does it
   actually rest its state shard? `~/.mirror/store/`? A per-user
   data dir under `$XDG_DATA_HOME`? Mara's spawn parents at `~/.mara/`
   — is that the SAME root supervisor as mirror repo's gen_prism's
   parent, just a child branch? Or is each home (`~/.reed/`, `~/.mara/`,
   `~/.mirror/`) its OWN root with NO root-above? The example in §3
   suggests one tree; the vocabulary in §2 suggests each home is its
   own tree. Needs Pack ratification.

3. **Cycle in entanglement.** The spec says entanglement edges may be
   cyclic; the sheaf framing (restriction maps) accommodates cycles
   naturally (a cellular sheaf has no acyclicity constraint). But the
   eigenboard query surface (per Reed's MCP work) currently models
   entanglement queries as forward-traversal — will the existing
   implementation handle cycles, or does this tick imply an
   eigenboard query lift?

4. **`@spectral/db` ↔ `@mirror/store` boundary.** Per
   `[[architecture-mirror-store-vs-spectral-db]]`, `@mirror/store` is
   the open storage gate (verification on write); `@spectral/db` is
   the closed-source engine on top (navigation). This tick adds
   `@spectral` as `@spectral/db`'s parent — that does NOT shift the
   open/closed boundary. But it DOES mean the closed engine now has a
   typed parent in the substrate, which means the substrate-altitude
   surface (the parent declaration) needs to be reviewed against the
   open-foundation contract. Open: does naming `@spectral/db`'s parent
   leak engine internals into the substrate, or does it correctly stay
   at the family-root altitude (parent declaration only, no
   sub-shard-specific exposure)?

5. **`@code/metalogue/materialize` lift status.** Per the memory
   `[[architecture-mirror-store-vs-spectral-db]]` and tasks #288 / #290 /
   #291, the materialize discipline is declared but not yet landed.
   The ouroboros pipeline requires it. Open: when does
   `@code/metalogue/materialize` land at the substrate altitude, and
   is the ouroboros pipeline a CONSUMER PULL that justifies that tick
   landing next, or does materialize need to land independently first?

---

## 10. References

- Substrate decisions:
  - `[[architecture-shards-as-substrate-source]]`
  - `[[architecture-prism-as-trait-as-everything]]`
  - `[[architecture-bateson-form-behaviour-partition]]` (#50)
  - `[[architecture-mirror-as-expanding-hilbert-space]]` (#51)
  - `[[architecture-three-tier-stack]]`
  - `[[architecture-hamilton-scheduler]]`
  - `[[architecture-mirror-store-vs-spectral-db]]`
  - `[[architecture-shard-as-crdt]]`
  - `[[architecture-connes-spectral-triple]]`
  - `[[Eigenboard is a sheaf]]`
  - `[[feedback-substrate-already-had-the-word]]`
- Specs:
  - `docs/specs/mirror-spectral.md` (the form-side sibling family)
  - `docs/specs/bootstrap-retirement-plan.md` (kintsugi-on-Rust track)
  - `docs/specs/code-metalogue-surface.md` (the materialize discipline)
  - `docs/specs/mirror-store.md` (the runtime crate's storage substrate)
- Insights:
  - `docs/insights/2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md`
  - `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md`
- Shards:
  - `shards/spectral.mirror` (LANDED THIS TICK)
  - `shards/mirror/spectral.mirror` (the form-side sibling family root)
  - `shards/kintsugi.mirror` (the process-side family root)
  - `shards/mirror.mirror` (the form-side language family root)

---

*"Substrate produces its own runtime. Name the floor; map the territory;
spec the pipeline. The ouroboros becomes real."* — Mara, 2026-06-10
