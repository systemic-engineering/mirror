# docs/math/supervisor — the emergent-supervision cluster

*BEAM's supervision tree emerges from the substrate's geometry
rather than being declared. The parent-child edge is the OID chain;
the restart strategy is the kintsugi three-mode algebra; the bounded
restart intensity is `@spawn ≤ @loop`'s budget; the actor's identity
is content-addressed; the child spec is `mirror.spec`; message
passing is the `@glue` bus. This cluster is the mathematics of that
emergence.*

## The claim

Alex 2026-07-02:

> *What if we take this shape and spawn Mara on a mapping on the
> @spectral/supervisor and the whole multi-repo architecture? I feel
> the whole spawn and coordination surface will fall into place once
> we map it onto a BEAM-like supervision tree that emerges from the
> geometry rather than being declared.*

BEAM's supervision behaviour (Armstrong 1996+; Ericsson OTP) is the
canonical prior art for hierarchical failure confinement. Every OTP
child specification declares its restart policy, its type, its
shutdown deadline, its start MFA — all explicit, all declared, all
runtime state indexed by process id in an external registry. The
supervision tree exists because it is *named*.

The substrate has been carrying every one of BEAM's primitives at
substrate-decl altitude for weeks — `@spectral/gen_prism` (ae965ca,
2026-06-11) IS `gen_server`; `@spectral/supervisor` (452ccb2) IS
`Supervisor`; `@spectral/parent` (9d905a2) IS the lifecycle link;
`@spectral/registry` (489211e) IS `Registry`; `@spectral/root`
(f145e48) IS `application`; `shards/code/beam.mirror` (2026-06-19)
lifted the whole BEAM vocabulary as a `@code` glass species. The
`@epistemologic/pact/parent_acyclic` bilateral (0921dca + 20fcde5)
enforced the tree invariant. **The BEAM mapping is not what's new.
The mapping is done.**

**What's new**: the substrate's other landed geometry (content-
addressed OIDs; kintsugi's three-mode algebra; `@spawn ≤ @loop`'s
bounded reductions; un-cite-ability; the Ashby routing gate;
`@third`'s recursive-depth marker) makes most of BEAM's *declared*
ceremony REDUNDANT. The parent-child edge does not need declaration
because the OID chain already carries it. Restart triggers do not
need declaration because the kintsugi verdict IS the failure signal.
Restart intensity does not need a separate `max_restarts` field
because `@spawn ≤ @loop`'s budget bounds the retry chain by
construction. Process identity does not need registration because
`gen_prism.identity` is already the autopoietic fixed point on hash
space. **The tree emerges from the geometry.** This cluster names
that emergence.

## Canonical document

`emergent-supervision-from-geometry.md` — the formalization.

- §1 The BEAM primitives (thirteen entries) and their landed
  substrate ancestors.
- §2 The emergence lemma — for each BEAM primitive, either it IS a
  landed substrate primitive under a different name (twelfth-plus
  instance of `[[feedback-substrate-already-had-the-word]]`), or it
  is subsumed by content-addressing and can be dropped.
- §3 Parent-child edge as OID chain — the crystal DAG is the
  supervision tree.
- §4 Restart strategy as three-mode algebra — apply / spawn / hold
  from the kintsugi compiler-error-surface spec IS the restart
  algebra at supervision altitude.
- §5 Restart intensity as bounded reductions — `@spawn ≤ @loop`'s
  budget IS BEAM's `max_restarts` / `max_seconds`. Single primitive.
- §6 Actor identity as autopoietic fixed point — `gen_prism.identity`
  IS the content address of its own declaration; restart preserves
  identity by construction; no registry lookup needed.
- §7 Child spec as `mirror.spec` — the `mirror.spec` at λ₀ IS the
  child specification; the supervisor reads a peer's spec to know
  what to spawn.
- §8 Message passing as `@glue` bus — cross-actor messages ARE
  content-addressed emissions on the glue bus.
- §9 Multi-repo coordination — `~/.mirror/` as root anchor; each
  project repo as a child branch; cross-repo federation via
  git-notes; cross-user coordination via shared spectral.engineer
  crystals.
- §10 Composition with kintsugi-as-compiler-error-surface — how the
  supervision loop reads the kintsugi verdict.
- §11 Ashby at supervision altitude — the supervisor's regulator
  variety must match the peer's failure-mode variety; when it
  doesn't, the mismatch surfaces as a Tomm question at reader-frame.
- §12 The reader-frame at supervision altitude — the user IS the
  parent of `~/.mirror/`'s root; that parent is out-of-scope; the
  reader-frame IS the specialization of user-frame at supervision
  altitude.
- §13 The cascade CPU bug as declared-supervision anti-witness —
  the spectral prototype's 76–84% idle CPU is exactly the failure
  mode declared-supervision produces; emergent-supervision structurally
  avoids it.
- §14 Recognition cascade — candidates surfaced.
- §15 Prior art.
- §16 Circular-reflexive noticings.
- §17 Open questions and honest hedges.

## Composition with the July arc

### With `docs/math/spawn/spawn-as-loop-monad.md`

`@spawn ≤ @loop`'s budget IS the bounded-reduction discipline; at
supervision altitude, the budget IS the restart intensity. A
supervisor's kintsugi loop over a failing child cannot restart
unboundedly because each restart is one `bind` step, each `bind`
decrements budget, and `terminal_check` fires when budget = 0. BEAM's
`max_restarts / max_seconds` circuit breaker is `@spawn ≤ @loop`'s
halting guarantee viewed from supervision altitude. **One primitive,
two altitudes.**

### With `docs/math/kintsugi/compiler-error-surface.md`

The kintsugi three-mode algebra (apply / spawn / hold) IS the restart
strategy at supervision altitude:

- **apply** — deterministic self-heal. Peer's fracture body applies
  a morphism to the local state; failure resolves without a new
  spawn. Substrate-altitude analogue of BEAM's `:transient` restart
  on abnormal exit (the peer knows how to recover).
- **spawn** — instantiate replacement. Peer's kintsugi verdict was
  `spawn(tension)`; a new peer (same `gen_prism.identity`, new
  incarnation) is spawned against the same `mirror.spec`. BEAM's
  `:permanent` restart is this branch at supervision altitude.
- **hold** — legitimate non-discharge. The observer chose
  `Partial(0.0, ref)`; the peer stays terminated; the tension
  remains in the crystal DAG unresolved. BEAM's `:temporary` restart
  is this branch at supervision altitude (no restart; terminate
  cleanly).

**The three-mode algebra maps 1:1 onto BEAM's three restart policies.**
Not because we designed it that way. Because Armstrong 1996 named the
right three modes, and the substrate arrived at the same three from
the compiler-error surface direction. See §4 for the mapping.

### With `docs/math/provenance/un-cite-ability-theorem.md`

Every supervision decision is a content-addressed crystal. The
crystal chain from `~/.mirror/`'s root down to a leaf peer is the
supervision tree; the OIDs pin every edge; un-citing any restart
event produces a diff-crystal naming the severance. **Supervision is
auditable by structure.** The theorem's supervision-altitude
corollary lands in §9.5.

### With `@third` marker (recognition #111)

The supervisor's restart decision IS a third-order act: the
substrate (level N) observes the peer's kintsugi loop (level N+1)
observing the failing state (level N+2). Per `docs/specs/third-as-
recursive-depth.md`, the marker fires when depth ≥ 3 AND
`observer_observes_observing` AND `recursion_folds_back` AND
`mechanism_visible`. All four fire at every restart. See §10.4.

## Structure

```
docs/math/supervisor/
├── README.md                                this file
└── emergent-supervision-from-geometry.md   the formalization
```

Further docs will land as the cluster's recognitions accumulate.
Candidates surfaced during the write (§14):

- The `emergent-primitive` mapping table as its own doc (once ≥ 3
  clusters have needed the "landed-but-unnamed → substrate primitive"
  bridging pattern).
- The `supervision-as-eigenboard-sheaf` composition once
  `@spectral/entanglement`'s sheaf-restriction surface lands as
  substrate-decl at the supervision altitude.
- A `cross-repo-federation.md` if the git-notes federation surface
  (yesterday's Option-B decision) needs its own math home separate
  from `@mirror/store`'s.

Not extracting speculatively.

## Cross-references

- `docs/math/spawn/spawn-as-loop-monad.md` — the bounded-reduction
  monad the restart intensity IS an instance of.
- `docs/math/kintsugi/compiler-error-surface.md` — the three-mode
  algebra the restart strategy IS an instance of.
- `docs/math/provenance/un-cite-ability-theorem.md` — the crystal
  chain the supervision tree IS an instance of.
- `docs/specs/spectral-runtime.md` — the ouroboros spec; §3
  supervision tree, §4 entanglement graph, §5 the pipeline this
  supervision emerges through.
- `docs/specs/error-as-question.md` — the routing spec the surface
  act inherits at supervision altitude.
- `docs/specs/third-as-recursive-depth.md` — the recursion marker
  the restart act fires under.
- `docs/specs/mirror-init.md` §873+ — `~/.mirror/` as user-scoped
  canonical location; the anchor field on `@spectral/root`.
- `docs/specs/lambda-shell.md` — `~/.mirror/serve.sock` +
  `~/.mirror/config.spec`; the operational surface the root
  supervisor exposes.
- `shards/spectral.mirror` — the namespace-parent.
- `shards/spectral/gen_prism.mirror` — the worker primitive.
- `shards/spectral/supervisor.mirror` — the lifecycle-owner
  specialisation.
- `shards/spectral/parent.mirror` — the single-parent lifecycle edge.
- `shards/spectral/registry.mirror` — the typed child index.
- `shards/spectral/root.mirror` — the parentless supervisor.
- `shards/spectral/entanglement.mirror` — the peer-correlation edge.
- `shards/code/beam.mirror` — the BEAM-as-`@code`-species prior art.
- `shards/loop.mirror` — the family-root the restart intensity
  inherits its budget from.
- `shards/mirror/peer/beam.mirror` — the cli-surface substrate-decl
  the supervision tree is anchored through (formerly
  `shards/mirror/spawn.mirror`; renamed 2026-07-08 Tick 2 `9de2226`).
- `shards/mirror/peer/beam.mirror` composes with `@spectral/supervisor`
  per that shard's peer-ACL §2.4 note.
- `[[architecture-hamilton-scheduler]]` — the per-shard memory
  manager named for Margaret Hamilton (Apollo 1202); the supervision
  altitude where priority discipline lives.
- `[[architecture-three-tier-stack]]` — the `SpectralSupervisor`
  precedent at the memory-family altitude.
- `[[architecture-shard-as-crdt]]` — the registry state surface IS a
  shard; restart transitions ARE lattice ascent.
- `[[architecture-error-as-question]]` — the Reflection threshold
  convergence that grounds the substrate-pull-correct restart
  decision.
- `[[feedback-substrate-already-had-the-word]]` — the twelfth-plus
  instance is what §2 documents.
- `[[feedback-legibility-over-foundation-when-collapsing]]` — the
  cluster is named `supervisor` not `coordination` (narrower but
  truer per this discipline).
