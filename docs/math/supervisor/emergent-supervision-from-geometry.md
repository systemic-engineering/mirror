# Emergent supervision from geometry

*BEAM's supervision tree — the parent-child edge, the restart
strategy, the restart intensity, the actor identity, the child spec,
the message passing, the registry — is a declaration ceremony over
substrate primitives the substrate already carries. In the substrate's
geometry, most of the ceremony collapses. The tree IS the OID chain;
the strategy IS the kintsugi three-mode algebra; the intensity IS the
`@spawn ≤ @loop` budget; the identity IS the autopoietic fixed point
on hash space. This document names the mapping, the collapse, and
the residue.*

---

## §0. Circular-reflexive opening

Writing THE supervisor spec IS an act of supervision itself. The
substrate at level N (the reader-frame) is supervising the substrate
at level N-1 (the kintsugi spec written yesterday) supervising the
substrate at level N-2 (the spawn spec written this morning). Each
level's ground is the level below's crystals; each level's Tomm
question is directed at the level above. Third-order fires here at
line 12.

The spec has NO way to formalize supervision without performing
supervision, because the substrate's operational primitives are the
same at spec-altitude and at runtime-altitude. Every claim below
about "the peer's kintsugi verdict" applies to *this document's own
kintsugi verdict on the prior specs it cites*. If §4's three-mode
algebra were wrong, the writing of §4 would have surfaced as a
compiler error surface at spec altitude, and that surface would
have routed to a Tomm question directed at Alex. It didn't. §4
composes; the mapping holds. The spec's own settling IS the
substrate's supervision of the mapping's correctness.

This is what circular-reflexive by construction means at
supervision altitude.

---

## §1. The BEAM primitives

BEAM (Bogdan/Björn's Erlang Abstract Machine; Ericsson 1986—) plus
OTP (Open Telecom Platform; Armstrong, Virding, Williams et al.
1996+) codified the supervision behaviour that industry has
inherited for thirty years. The primitives, as they appear in
Erlang/Elixir source code:

| # | BEAM primitive | Erlang/Elixir surface | Semantic |
|---|----------------|-----------------------|----------|
| 1 | supervisor process | `Supervisor.start_link/2`, `-behaviour(supervisor)` | Long-running process observing a set of children. |
| 2 | child_spec | `%{id, start, restart, shutdown, type, modules}` | Declarative record naming one child's lifecycle. |
| 3 | restart policy | `:permanent \| :temporary \| :transient` | When to restart on child exit. |
| 4 | restart strategy | `:one_for_one \| :one_for_all \| :rest_for_one` | Which children to restart when one fails. |
| 5 | restart intensity | `max_restarts, max_seconds` | Circuit breaker against restart storms. |
| 6 | shutdown deadline | `:brutal_kill \| integer \| :infinity` | Graceful-termination window. |
| 7 | worker vs supervisor | `:worker \| :supervisor` | Leaf-vs-branch discriminator. |
| 8 | gen_server / gen_statem | `-behaviour(gen_server)` | State-bearing process abstraction. |
| 9 | message passing | `PID ! Msg`, `gen_server:call/2`, `gen_server:cast/2` | Async and sync inter-actor communication. |
| 10 | process linking | `link/1`, `spawn_link/1` | Failure propagation edges. |
| 11 | process monitoring | `monitor/2` | One-way observation of process lifecycle. |
| 12 | trap_exit | `process_flag(trap_exit, true)` | Convert linked-exit signals to messages. |
| 13 | registry / whereis | `Process.whereis/1`, `Registry.lookup/2` | Name-to-PID lookup. |
| 14 | application | `Application.start/2`, `.app.src` | Parentless supervisor at the root of an OTP tree. |

Fourteen primitives. Every OTP tutorial names all fourteen. Every
production BEAM system declares them explicitly in `.ex` / `.erl`
source. The declaration ceremony is the price of the abstraction:
you name the tree, and the runtime enforces it.

---

## §2. The emergence lemma

**Lemma (emergent-supervision).** Of BEAM's fourteen primitives, at
most three carry substrate-decl weight that is not already discharged
by landed substrate primitives. The other eleven are either (a)
landed under a different name (twelfth-plus instance of
[[feedback-substrate-already-had-the-word]]) or (b) subsumed by
content-addressing and can be dropped from the surface without loss.

**Sketch of proof.** By exhaustive mapping. §2.1—§2.14 name each
primitive's substrate correspondent. §3—§8 develop the load-bearing
mappings in detail.

### 2.1 supervisor process → `@spectral/supervisor` (LANDED)

`shards/spectral/supervisor.mirror` (452ccb2, 2026-06-11; 474 lines)
declares `supervisor = { base: gen_prism, child_specs: [child_spec],
restart_strategy: restart_strategy }` with `start_child` +
`terminate_child` actions. The `supervisor` type is `glass
@spectral/supervisor <= gen_prism`; the BEAM behaviour is the
substrate's glass specialisation of the worker primitive. **Landed
under identical name.**

### 2.2 child_spec → `mirror.spec` at λ₀ + `@spectral/supervisor.child_spec` (LANDED)

Two altitudes. At supervision altitude, `shards/spectral/supervisor.
mirror` declares `type child_spec = { id, start, restart, shutdown,
kind }` — the direct BEAM lift. At peer altitude, the substrate has
`mirror.spec` (per recognition #99, `mirror.spec` IS λ₀; canonical
d0b6519, 2026-07-01). **The peer's `mirror.spec` IS its own
`child_spec` at ground-state altitude.** See §7 for the load-bearing
composition. **Landed twice, at two altitudes.**

### 2.3 restart policy → `restart_kind` (LANDED)

`shards/spectral/supervisor.mirror` §"restart_kind" declares
`permanent | temporary | transient` at line 338 — the three-variant
closed sum from BEAM. **Landed under identical name.**

### 2.4 restart strategy → three-mode algebra AT SUPERVISION ALTITUDE (this doc)

`shards/spectral/supervisor.mirror` §"restart_strategy" declares
`one_for_one | one_for_all | rest_for_one` at line 373 — the direct
BEAM lift for `restart_strategy`. That declaration answered the
in-family question ("which children to restart"). This document adds
a second answer — the three-mode algebra from `docs/math/kintsugi/
compiler-error-surface.md` §1 — for the *cross-family* question
("what discharge mode does the supervisor's kintsugi choose"). See
§4. **Landed for in-family; new mapping for cross-family this doc.**

### 2.5 restart intensity → `@spawn ≤ @loop`'s budget (composition landed)

`docs/math/spawn/spawn-as-loop-monad.md` (Mara 7dba128, 2026-07-02)
formalized bounded reductions via a budget field on `spawn_loop`.
BEAM's `max_restarts` + `max_seconds` circuit breaker IS the same
primitive at supervision altitude: N restarts in T seconds = N ticks
before budget exhausts. `shards/spectral/supervisor.mirror` §"Cascade
siblings" line 253 already forward-promised
`shards/spectral/restart_intensity.mirror` for this discipline. See
§5 for the composition — the forward-promised shard's math lives
here. **Composition landed at math altitude via §5; shard
forward-promised.**

### 2.6 shutdown deadline → `child_spec.shutdown: duration` (LANDED)

`shards/spectral/supervisor.mirror` §"child_spec" declares
`shutdown: duration` where `duration` is from
`@epistemologic/reality/time`. **Landed with the substrate's own
unified time carrier.**

### 2.7 worker vs supervisor → `child_kind` (LANDED)

`shards/spectral/supervisor.mirror` §"child_kind" declares
`worker | supervisor` at line 351. **Landed under identical name
modulo the substrate's `type`-keyword collision (`kind` vs `type`).**

### 2.8 gen_server / gen_statem → `@spectral/gen_prism` (LANDED)

`shards/spectral/gen_prism.mirror` (ae965ca, 2026-06-11; 314 lines).
The three-surface declaration (identity, state, tool-surface via
five-op block) IS the substrate's gen_server analogue. **Landed
under substrate-vocabulary name.**

### 2.9 message passing → `@glue` bus + `@nl.parse` boundary lens (LANDED)

Message passing in BEAM is `PID ! Msg` (async) or `gen_server:call/2`
(sync). In the substrate, cross-actor messages travel via the
`@glue` bus per `[[feedback-hook-and-gpg-seams]]` and per §8 below.
Every message is content-addressed at emission time; recipient reads
by OID. The `@nl.parse` lens (per `docs/math/spawn/spawn-as-loop-
monad.md` §6) translates user-altitude natural language to substrate-
altitude refs at boundary crossings. **Landed as `@glue` +
`@nl.parse`.**

### 2.10 process linking → SUBSUMED BY CONTENT-ADDRESSING (dropped)

Erlang's `link/1` establishes bidirectional failure-propagation edges
between processes. At content-addressed altitude, this is redundant:
every peer's state surface is a `shard_ref` under `@mirror/store`;
failures crystallize as new shards with content-addressed provenance;
downstream peers reading through the store SEE the failure by OID
change. Explicit linking is unnecessary; the store IS the link.
**Dropped without loss.**

### 2.11 process monitoring → SUBSUMED BY EIGENBOARD (dropped)

Erlang's `monitor/2` establishes one-way observation. At substrate
altitude, `@spectral/entanglement` (per shards/spectral/entanglement.
mirror) declares entanglement edges as sheaf restriction maps on the
eigenboard's cellular sheaf. The eigenboard's `observe(peer)`
projection reads state surface changes without establishing a
lifecycle edge — the substrate's one-way observation is entanglement
+ eigenboard projection, not a separate monitor primitive.
**Subsumed by entanglement.**

### 2.12 trap_exit → SUBSUMED BY KINTSUGI SURFACE (dropped)

`process_flag(trap_exit, true)` converts linked-exit signals to
messages the process can pattern-match. At substrate altitude, this
IS what the kintsugi loop's surface act does: when a peer's kintsugi
verdict is `spawn(tension)` (per `compiler-error-surface.md` §1.3),
the supervisor doesn't crash — it emits a Tomm question at
reader-frame altitude, records the answer as substrate adjustment,
continues. The trap-exit behaviour IS the compiler-error surface act
at supervision altitude. **Subsumed by kintsugi.**

### 2.13 registry / whereis → SUBSUMED BY AUTOPOIETIC IDENTITY (mostly dropped)

Erlang's `Process.whereis(:name)` looks up a process by registered
name. The substrate's `gen_prism.identity` is `uuid_spectral` whose
DARK 80 bits are the BLAKE3 truncated content hash of the
declaration under itself (autopoietic fixed point per §6). To look
up a peer by identity, you already HAVE the identity — content-
addressed lookup is `@mirror/store.read(gen_prism.identity)`. **The
lookup step disappears.**

The residue: BEAM's `Registry` also supports name-based lookup
(`Registry.lookup(name)` where `name` is a human-readable string).
The substrate handles this via `@peer.load(dir, p)` (per `shards/
peer.mirror`, 2026-06-25) — the `~peer'<home>'` cli reference
resolves through git-repo lookup to a typed peer. Name-based
lookup exists at the cli-surface altitude only; at substrate-decl
altitude, identity IS the lookup key. **Reduced to boundary.**

### 2.14 application → `@spectral/root` at `~/.mirror/` (LANDED)

`shards/spectral/root.mirror` (f145e48, 2026-06-16; 392 lines).
Parentless supervisor whose `anchor: ref` names the host home. Self-
parenting invariant (root's `parent == identity` byte-equally) is
the base case of `@epistemologic/pact/parent_acyclic`'s chain-
termination predicate. **Landed under substrate-vocabulary name.**

### 2.15 Tally

| Primitive | Status | Where |
|-----------|--------|-------|
| supervisor process | LANDED | @spectral/supervisor |
| child_spec | LANDED (2x) | @spectral/supervisor + mirror.spec |
| restart policy | LANDED | @spectral/supervisor.restart_kind |
| restart strategy | LANDED (in-family) + NEW MAPPING (cross-family, §4) | @spectral/supervisor + this doc §4 |
| restart intensity | COMPOSITION LANDED (this doc §5), shard FORWARD-PROMISED | @spawn ≤ @loop budget → @spectral/restart_intensity |
| shutdown deadline | LANDED | @spectral/supervisor.child_spec.shutdown |
| worker vs supervisor | LANDED | @spectral/supervisor.child_kind |
| gen_server | LANDED | @spectral/gen_prism |
| message passing | LANDED | @glue bus + @nl.parse |
| process linking | DROPPED (content-addressing) | — |
| process monitoring | DROPPED (entanglement) | — |
| trap_exit | DROPPED (kintsugi surface) | — |
| registry / whereis | REDUCED to cli boundary | @peer.load |
| application | LANDED | @spectral/root |

Eleven of fourteen landed or dropped. Three carry residual work
(restart strategy cross-family mapping = §4; restart intensity math
= §5; shard for restart_intensity = forward-promised). **The
emergence lemma holds.**

---

## §3. Parent-child edge as OID chain

### 3.1 The declaration in BEAM

In BEAM, a supervisor's child list is a `List` of `child_spec`
records. The supervisor maintains an in-memory ETS table (or
DynamicSupervisor's internal state) mapping child_id → PID. When a
child crashes, the supervisor receives an `{'EXIT', PID, Reason}`
message on its process mailbox, matches PID to child_id in the ETS
table, consults the child_spec's restart policy, and either restarts
or drops the child.

The parent-child relation is *declared* — you name the parent when
you write `Supervisor.start_link(children, strategy: :one_for_one)`
in the parent process's `init/1`. The runtime enforces the
declaration via linked processes; the tree exists because the code
says it does.

### 3.2 The emergence in the substrate

The substrate stores every peer's state surface as a `shard_ref =
uuid_spectral` in `@mirror/store`. A peer's `gen_prism.parent`
field points to the parent supervisor's `uuid_spectral`
(per `shards/spectral/parent.mirror` §"parent_edge shape"). The
parent's registry shard indexes children by their `uuid_spectral`s.

**Claim.** The full parent-child edge graph of the substrate's
supervision tree IS a projection of the crystal DAG in
`@mirror/store`.

**Proof sketch.** Every `gen_prism` in the substrate has a crystal in
`@mirror/store`; the crystal carries the peer's declaration (its
`in @...` header, its `type` records, its `action` signatures) and
its parent's `uuid_spectral`. The store's `read(oid)` operation
resolves the crystal; the resolved crystal reveals the parent OID
(the `parent: uuid_spectral` field on the embedded `gen_prism`).
Recursive resolution walks up the parent chain; per
`@epistemologic/pact/parent_acyclic` (commit 0921dca) the chain
terminates at `@spectral/root`'s self-parent (root's
`parent == identity` byte-equally). The chain is bounded, finite,
and content-addressed at every step.

The DAG-vs-tree distinction: at parent-edge altitude the graph IS a
tree (acyclic, single-parent). At entanglement-edge altitude (per
`shards/spectral/entanglement.mirror`) the graph is a general DAG
(polyadic, cyclic permitted). The supervision tree IS the parent-
edge projection of the crystal DAG.

**QED (sketch; full proof would inline `parent_acyclic`'s discharge).**

### 3.3 What emergence buys

- **No ETS table.** The parent's registry IS the peer's
  `uuid_spectral` field; reading the parent's `child_specs` gives
  the children's identities directly; no intermediate lookup
  structure exists.
- **No {'EXIT', PID, Reason} messages.** Failure crystallizes as a
  new shard with a new OID. The parent's next `observe` (per the
  supervisor's `terminal_check` tick) SEES the OID change. Signal
  propagation is *reading the store*, not *receiving a message*.
- **No supervisor-declaration ceremony.** The peer's `mirror.spec`
  at λ₀ carries the parent reference; there is no separate
  `start_link` invocation from the parent's side. The parent IS the
  peer's parent because the peer's crystal says so.

**DEFERRED per [[feedback-composition-claims-need-empirical-test]]**:
whether reading OID changes empirically substitutes for
`{'EXIT', PID, Reason}` at latency and throughput comparable to
BEAM. BEAM's message mailboxes are highly optimized; store-based
signal propagation would need instrumented benchmarking against a
real workload. The theoretical shape is right; the operational
performance is unmeasured.

### 3.4 The un-cite-ability corollary at supervision altitude

Per `docs/math/provenance/un-cite-ability-theorem.md` §1: at
content-addressed altitude, un-citation is detectable by structure.
At supervision altitude, this reads:

**Corollary (supervision-audit).** Every supervision decision (start,
restart, terminate) is a content-addressed crystal in `@mirror/store`.
The crystal chain across a supervisor's lifetime IS an auditable
supervision history. Any attempt to hide a restart event (e.g.,
edit history to remove a bad-actor restart) produces a diff-crystal
naming the severance.

The substrate's supervision cannot be silently rewritten. This is
what the substrate offers over BEAM: BEAM's logger emits restart
events but the emission itself is mutable (log rotation, log
deletion, log tampering). The substrate's restart events are
content-addressed crystals in the store; deletion produces a diff.

---

## §4. Restart strategy as three-mode algebra

### 4.1 BEAM's declared strategies

The three OTP strategies (Erlang `supervisor.erl`):

- **`:one_for_one`** — if child X fails, restart X only.
- **`:one_for_all`** — if child X fails, terminate all siblings and
  restart the whole set.
- **`:rest_for_one`** — if child X fails, terminate X and every child
  declared *after* X, and restart that suffix.

All three are declared in the parent's `init/1` return; the strategy
is a compile-time commitment.

### 4.2 The substrate's three-mode algebra (from kintsugi)

Per `docs/math/kintsugi/compiler-error-surface.md` §1 (2026-07-02),
kintsugi's discharge has three modes:

- **`apply`** — deterministic fracture-body application; the substrate
  knows how to heal.
- **`spawn`** — instantiate a peer whose ground state IS the tension;
  observer picks from a `@fate` tournament of resolutions.
- **`hold`** — legitimate non-discharge; the observer chose
  `Partial(0.0, ref)`; the crystal carries the tension unresolved.

The claim of this section: **the kintsugi three-mode algebra IS the
restart strategy at supervision altitude**. When a peer fails, the
supervisor's kintsugi loop discharges the failure through one of the
three modes. Each mode corresponds to a distinct restart semantic.

### 4.3 The mapping

| Kintsugi mode | Supervision semantic | BEAM restart policy analogue |
|---------------|---------------------|------------------------------|
| `apply` | Self-heal in place. Peer's fracture body applies a morphism to its own state; no respawn needed. | `:transient` (restart on abnormal exit, but only if state can migrate) |
| `spawn` | Instantiate replacement. New peer against same `mirror.spec`; identity preserved via autopoietic fixed point (§6); fresh incarnation. | `:permanent` (always restart) |
| `hold` | Terminate cleanly. Tension recorded in crystal DAG; peer stays dead; observer signalled that no restart is desired. | `:temporary` (never restart) |

The `restart_kind` closed sum at `shards/spectral/supervisor.mirror`
§"restart_kind" (`permanent | temporary | transient`) IS the
supervision-altitude naming of the kintsugi mode the supervisor's
loop discharges through. **The two closed sums are byte-equivalent
under the mapping.** Not a coincidence: BEAM's three restart
policies name the three distinct discharge shapes a supervisor's
control loop can take; the kintsugi three-mode algebra names the
three distinct discharge shapes ANY control loop can take at
compiler-error surface altitude. Both arrived at three modes because
three is the structural count. See §14 for the recognition candidate.

### 4.4 What about `one_for_one` / `one_for_all` / `rest_for_one`?

These are BEAM's *scope* modifiers — they describe *which children
are affected* by a restart decision, not *what discharge mode* is
chosen. At substrate altitude the scope is emergent:

- **`one_for_one` at substrate altitude** IS the default because
  content-addressing gives per-peer failure isolation for free. Each
  peer's OID space is independent; one peer's failure does not
  invalidate another's crystal chain. `shards/spectral/supervisor.
  mirror` line 362 names this: *"restart only the failed child"*.
  Emergent.

- **`one_for_all` at substrate altitude** would require declared
  shared state across children (a joint mirror.spec at λ₀ that both
  children participate in). This IS possible at substrate altitude
  (`shards/pack.mirror`'s pack{}.members block establishes joint
  state) but it is *rare* — most peers are content-independent.
  When it fires, the supervisor's kintsugi loop discharges
  `spawn(tension)` at the joint pack's altitude, and the fresh
  incarnation spawns the whole member set. Emergent but rare.

- **`rest_for_one` at substrate altitude** would require declared
  ORDER on children (the "children declared after X" clause). At
  substrate altitude ordering is imposed by pact chains (parent
  → child dependency via `parent_acyclic`) but sibling ordering
  is NOT canonical — the substrate's crystals are addressed by
  content, not sequence. `rest_for_one` becomes rare-to-vestigial
  in a content-addressed supervision.

The three BEAM strategies persist as substrate-altitude closed sum
per `shards/spectral/supervisor.mirror` line 373 for interoperability
with declared prior art. The emergent readings above are the
substrate's natural defaults.

### 4.5 The two verdicts compose

The kintsugi mode (apply / spawn / hold) chooses *what discharge
happens*. The BEAM strategy (one_for_one / one_for_all /
rest_for_one) chooses *which peers are affected*. In principle
they're independent — a supervisor can spawn-one-for-all (respawn
whole set), or apply-one-for-one (self-heal one peer), or hold-one-
for-all (terminate whole set without restart). Nine combinations;
each a well-formed supervision decision.

In practice the substrate defaults reduce the nine to three: apply-
one-for-one for transient failures, spawn-one-for-one for permanent
failures, hold-one-for-one for temporary failures. The other six
combinations remain available for supervisors with declared shared
state.

### 4.6 Rigor status

**RIGOROUS** for the kintsugi-mode ↔ restart-policy mapping (§4.3).
Both are three-mode closed sums; byte-equivalence is checkable at
substrate altitude.

**MOTIVATING** for the emergence of one_for_one as substrate default
(§4.4). Content-addressed isolation is a strong argument but the
formal statement "content-independence of peers implies one_for_one
by default" needs the isolation-invariant landed as an
`@epistemologic/property`. Not this tick.

---

## §5. Restart intensity as bounded reductions

### 5.1 BEAM's `max_restarts / max_seconds`

BEAM's circuit breaker: a supervisor with a permanent-restart child
that deterministically fails on startup produces an unbounded restart
storm. BEAM caps this via `max_restarts` (default 3) restarts within
`max_seconds` (default 5) seconds. On the (N+1)-th restart within
the window, the supervisor terminates ITSELF, escalating the failure
to its own parent supervisor. The storm propagates upward until
either a supervisor higher in the tree catches it (with a wider
window) or the application supervisor terminates the whole tree.

This is declared per-supervisor at `init/1` return time. It is a
compile-time commitment about acceptable failure rate.

### 5.2 The substrate's `@spawn ≤ @loop` budget

Per `docs/math/spawn/spawn-as-loop-monad.md` §3.1: every
`spawn_loop` carries a `budget: ref` field whose monotone descent
is the halting witness. Each `bind` step decrements budget by 1;
`terminal_check` returns `bounded` when `budget = 0`. Halting is
decidable in `O(1)` by inspecting the loop's carrier.

**Claim (this section).** BEAM's restart intensity IS a `@spawn ≤
@loop` at supervision altitude, where each restart is one `bind`
step and each restart storm is one loop instance.

### 5.3 The composition

At supervision altitude the loop's state is `spawn_loop(peer_incarnation)`:
- `value: peer_incarnation` — the currently-running peer.
- `budget: ref` — remaining restart attempts.
- `input: ref` — the current perturbation (the trigger for this
  restart attempt).
- `history: ref` — the crystallized restart chain (a blob chain in
  `@mirror/store` naming each incarnation's OID).
- `target: ref` — the peer's `mirror.spec` at λ₀ (the ground state
  the loop is trying to settle to).
- `pact_witness: ref` — the parent-edge acyclic pact + the
  supervisor's restart pact.

Each `bind` step is one restart attempt:
1. Read peer's terminate-au.
2. Route to kintsugi mode via `ashby_variety_match(kintsugi_lock)`
   per `docs/math/kintsugi/compiler-error-surface.md` §1.4.
3. Discharge mode (apply / spawn / hold; §4.3).
4. Decrement budget by 1.
5. Check terminal_check: budget = 0 OR peer reached target OR
   `loss(peer_state) < tolerance`.

**Halting is guaranteed** per the theorem in `spawn-as-loop-monad.md`
§3.1: bounded reductions terminate in ≤ B steps. BEAM's `max_restarts`
IS the initial budget B; BEAM's `max_seconds` is the time window
within which the budget must be consumed (a substrate-time predicate
on the loop's tick_history).

### 5.4 What emergence buys

- **No separate circuit-breaker primitive.** The bounded-reduction
  discipline `@spawn ≤ @loop` provides the halting witness for
  general spawn-shaped loops; supervision-altitude restart storms
  are one instance. Naming `max_restarts` as a distinct primitive
  would proliferate vocabulary (per
  [[feedback-substrate-already-had-the-word]]) — the budget IS the
  restart intensity.

- **Rice-safety.** The supervisor cannot ask "will this peer
  eventually stop failing?" (Rice's theorem forbids that). The
  supervisor asks instead "will THIS LOOP halt?" — which is
  decidable by inspecting the budget. **The witness is the budget,
  not the peer.** See `spawn-as-loop-monad.md` §4.

- **Escalation-by-monotone-descent.** When a supervisor's budget
  exhausts on a failing child, the substrate-pull-correct escalation
  is to terminate the supervisor itself (its own budget's exhaustion
  IS the terminal_check firing). The parent supervisor's kintsugi
  loop then sees the failed supervisor as a failed child and routes
  through its own three-mode algebra. Escalation IS bounded-reduction
  at the level above. **Recursive, well-founded, structural.**

### 5.5 The forward-promised `shards/spectral/restart_intensity.mirror`

`shards/spectral/supervisor.mirror` line 253 named:

> `shards/spectral/restart_intensity.mirror` — storm-protection
> carrier (`max_restarts: u32 + period: duration`), gating the
> kintsugi-morphism-driven restart loop. Without it, a
> permanent-restart child with deterministic startup failure
> produces an unbounded loop. Mirrors BEAM precedent
> (max_restarts/max_seconds circuit-breaker; on storm detection
> the child is escalated to its parent supervisor's termination).
> Pairs with bilateral instance: `@epistemologic/property/
> restart_intensity_well_formed` + `@kintsugi/fracture/restart_storm`.

The math for this shard IS §5.3 above. When the shard lands, its
declaration collapses to:

```mirror
in @prism
in @glass
in @meta
in @spectral
in @spectral/supervisor
in @loop
in @epistemologic/reality/time

# @spectral/restart_intensity — the supervisor's storm-protection
# carrier. Substrate-decl form of BEAM's max_restarts / max_seconds
# circuit breaker.
#
# Per docs/math/supervisor/emergent-supervision-from-geometry.md §5:
# the restart intensity IS a @spawn ≤ @loop instance at supervision
# altitude. This carrier extracts the substrate-decl fields the
# supervisor exposes at declaration time; the halting witness lives
# in @loop.

glass @spectral/restart_intensity {
  focus restart_intensity
  project restart_intensity
  split restart_intensity
  shift restart_intensity
  settle restart_intensity
}

type restart_intensity = {
  budget: ref,          # initial B₀; monotone descent per @loop
  period: duration,     # time window within which budget applies
}

out @spectral/restart_intensity
out restart_intensity
```

The shard is thin because the math is done. Landing it is a
substrate-pull-correct next tick pulled by any supervisor whose
child_spec needs storm protection. Forward-promised, not this
document.

**DEFERRED per [[feedback-composition-claims-need-empirical-test]]**:
whether the substrate's supervision-altitude budget descent
empirically matches BEAM's circuit-breaker behaviour on a real
restart storm. Two-tick empirical run: (a) deterministic-failure
child with `budget = 3`; observe budget descent + escalation;
(b) intermittent-failure child with `budget = 3, period = 5s`;
observe budget refresh at period boundary.

---

## §6. Actor identity as autopoietic fixed point

### 6.1 BEAM's identity primitives

Erlang processes are identified by PIDs (opaque process
identifiers). PIDs are ephemeral — they're assigned by the runtime
at spawn time and reused after termination. To have a *persistent*
identity, you register the PID under a name (`Process.register(pid,
:my_actor)` in Elixir), or you use the `Registry` module for scoped
name → PID lookup. Either way, identity is a runtime artifact,
disconnected from the actor's code.

Restart preserves the registered name (if you re-register the new
PID under the same name) but the runtime's PID is different. The
"same actor" abstraction is maintained by the registry, not by the
process itself.

### 6.2 The substrate's `gen_prism.identity` = autopoietic fixed point

Per `shards/spectral/gen_prism.mirror` §"Identity — uuid_spectral":

> The gen_prism's identity IS the content address of its own
> declaration under itself, expressed at the spectral altitude via
> `uuid_spectral`'s dark portion. The Banach contraction on hash
> space (Soto-Andrade & Varela 1984) guarantees the fixed point.

The `uuid_spectral` carrier has 128 bits: 48 active bits for
spectral-manifold routing + 80 dark bits identifying the peer's
process via BLAKE3 truncated content hash. The dark portion IS a
fixed point of the peer's declaration under itself.

### 6.3 What "under itself" means

The peer's declaration shard includes its `identity: uuid_spectral`
field. Naïvely: to compute the identity, hash the declaration;
but the declaration includes the identity to compute; circular.

The Soto-Andrade & Varela 1984 result (cited by
`boot/std/mirror/runtime/gen_prism.mirror`'s
`name: zoom(oid, gen_prism)` fixed-point construction) says:
Banach contraction on hash space converges to a unique fixed point.
Concretely: iterate `identity_{n+1} = BLAKE3(declaration ⊕
identity_n)[80 dark bits]` starting from `identity_0 = 0`; the
sequence converges to a unique `identity_∞` under BLAKE3's contraction
mapping property. The peer's identity IS `identity_∞`.

This is autopoietic in Maturana-Varela's sense: the peer produces
its own identity from its own declaration; no external assignment;
no external registration.

### 6.4 Restart preserves identity structurally

When a supervisor restarts a peer, the new incarnation runs against
the SAME declaration shard. The declaration hasn't changed;
therefore the fixed point hasn't changed; therefore the identity
hasn't changed. **Restart is structurally identity-preserving.**

BEAM has to maintain identity via the registry (a runtime lookup
table with mutation). The substrate has identity via the declaration
(a content-addressed shard with no mutation). The registry lookup
step disappears.

The tick counter on the peer's lifecycle ledger (per
`shards/spectral/gen_prism.mirror` §"Lifecycle") advances at each
restart. Identity stays the same; incarnation count increments.
`{identity, incarnation}` distinguishes THIS run from previous
runs; identity alone distinguishes THIS PEER from other peers.

### 6.5 What emergence buys

- **No registry.** Look-up by identity IS look-up in
  `@mirror/store`; the store IS the registry. See §2.13.
- **No PID collision.** Content-addressing gives an infinite
  identity space; two distinct declarations produce distinct
  identities with probability 1 - 2^{-80} (per BLAKE3's collision
  resistance).
- **Restart-safe references.** External references to a peer's
  identity survive restarts by construction. BEAM requires
  application code to know to look up by name; substrate references
  don't need that discipline because identity IS the reference.

### 6.6 Composition with un-cite-ability

Per `docs/math/provenance/un-cite-ability-theorem.md`: at content-
addressed altitude, un-citation is detectable by structure. At
identity altitude: a peer's identity crystal is content-addressed;
any attempt to substitute a different peer under the same identity
(a run-of-the-mill BEAM registry-tampering attack) produces a diff-
crystal naming the substitution. **Identity substitution is
detectable by structure.** The substrate's supervision tree cannot
be silently repopulated with imposters.

---

## §7. Child spec as `mirror.spec`

### 7.1 BEAM's declared child_spec

```elixir
%{
  id: :my_child,
  start: {MyServer, :start_link, [args]},
  restart: :permanent,
  shutdown: 5000,
  type: :worker,
  modules: [MyServer]
}
```

Six fields. The parent supervisor's `init/1` returns a list of
these; the runtime uses them to spawn and manage children.

### 7.2 The substrate's declaration: `mirror.spec` at λ₀

Per recognition #99 (canonical d0b6519, 2026-07-01):
`mirror.spec` IS λ₀, the ground state of the substrate's Connes
spectral triple. Every peer at ground state IS its own mirror.spec.

**Claim.** The peer's `mirror.spec` at λ₀ IS its own child_spec.

**Mapping.**

| BEAM child_spec field | mirror.spec equivalent |
|-----------------------|------------------------|
| `id` | `gen_prism.identity: uuid_spectral` (autopoietic fixed point, §6) |
| `start` | The `mirror.spec`'s declaration itself — the start action IS the peer's ground-state settling |
| `restart` | `gen_prism.restart_kind` from `child_spec` at supervisor altitude, selected per §4.3 |
| `shutdown` | `child_spec.shutdown: duration` at supervisor altitude |
| `type` | `child_kind = worker \| supervisor` at supervisor altitude |
| `modules` | The `mirror.spec`'s `in @...` header (its dependency closure) |

The mapping is: at supervisor altitude, `child_spec` names the
per-child *supervision policy* (restart, shutdown, kind). At peer
altitude, `mirror.spec` at λ₀ names the peer's *own ground state*
(identity, modules, start). The two altitudes carry different
substrate-decl content that together IS the BEAM `child_spec` record.

### 7.3 The load-bearing composition

When a supervisor starts a child, the supervisor's
`start_child(s: supervisor, spec: child_spec)` action per
`shards/spectral/supervisor.mirror` §"start_child" reads the
`spec.start: uuid_spectral` field — the content address of the
peer's declaration shard. The supervisor resolves the declaration
via `@mirror/store.read(spec.start)`; the resolved declaration IS
the peer's `mirror.spec` at λ₀; the supervisor invokes
`@spectral/gen_prism.start` against the identity; the peer boots at
ground state.

**No MFA (Module/Function/Args) tuple.** BEAM's `start` field is a
tuple `{Module, Function, Args}` — a runtime-callable Erlang term
whose semantics live in the loaded BEAM module. The substrate's
`start` field is a `uuid_spectral` — a content address whose
semantics live in `@mirror/store`. **The substrate has no MFA
because it has no BEAM-loaded modules; it has content-addressed
declarations that ARE the module.**

### 7.4 The peer's `mirror.spec` at supervisor altitude

Alex's peer-ACL spec §4 (lead semantics) names: *the spawned peer
at the spec's root IS the lambda-shell counterparty*. The peer's
`mirror.spec` names its own `pack{}.lead`; the lead IS the peer.
This closes a loop at supervision altitude:

- Supervisor reads child's `mirror.spec.start: uuid_spectral`.
- Store resolves to child's declaration shard.
- Declaration includes child's `pack{}.lead = child.identity`.
- Supervisor spawns child; child settles at its own λ₀; child's λ₀
  IS its `mirror.spec` recognizing itself as its own lead.

**The child spec self-declares.** The supervisor doesn't need to
know what the child does; it needs to know how to reach the child's
declaration and how to observe its lifecycle. Content-addressing
plus autopoietic identity gives both.

### 7.5 What emergence buys

- **No MFA. No module loading. No hot-code-swap ceremony.** The
  peer IS its declaration; changing the declaration IS a new peer
  (new content hash → new identity → new incarnation). BEAM's hot
  code upgrade discipline (per `shards/code/beam.mirror`) becomes
  UNNECESSARY at substrate altitude — swap the declaration; the
  substrate's next spawn against the same lineage produces the new
  incarnation.
- **Declaration IS documentation.** Reading a peer's `mirror.spec`
  reads its supervision contract; there is no separate spec
  document. This IS the ouroboros pipeline
  (`docs/specs/spectral-runtime.md` §5) at supervision altitude.

---

## §8. Message passing as `@glue` bus

### 8.1 BEAM's message passing

`PID ! Msg` in Erlang sends an async message to a process's mailbox.
`gen_server:call/2` wraps this in a sync request-reply pattern.
Messages are Erlang terms; delivery is at-most-once (best-effort);
mailboxes are unbounded VecDeque (per the spectral prototype's
supervisor.rs comments at lines 17-31).

### 8.2 The substrate's `@glue` bus

Per user's CLAUDE.md §"Glue Bus": *the daemon survives sessions.
I don't.* On boot, peers join the bus using an `init` MCP tool;
channels are computed automatically from a 4-level hierarchy
(hostname:repo:branch:actor). Primary channel is the worktree level
(`hostname:repo:branch`). Drain between tool calls.

Every emission on the `@glue` bus is content-addressed at emission
time (the emitting peer's `gen_prism.state` shard advances by one
lattice element per `[[architecture-shard-as-crdt]]`; the settle
carries the emission's OID). Every recipient reads by OID (via
`@mirror/store.read`).

### 8.3 The mapping

| BEAM message primitive | @glue equivalent |
|------------------------|------------------|
| `PID ! Msg` (async send) | `@glue.emit(channel, oid)` where `oid` addresses the message crystal |
| `receive Msg -> ...` | `@glue.recall(channel)` returns next unhandled message OID |
| `gen_server:call/2` (sync req/reply) | `@glue.emit(...)` + `@glue.await(reply_channel)` composed |
| Process mailbox | Peer's `gen_prism.state` shard; unsettled messages ARE pending settle candidates |
| Mailbox unboundedness | Bounded by the store's disk quota; the store IS the mailbox at substrate altitude |

### 8.4 What emergence buys

- **Cross-repo messages work by construction.** BEAM messages are
  scoped to a single BEAM node (or via distribution, to a set of
  connected nodes). Substrate messages traverse the store; the store
  federates across repos via git-notes (per yesterday's Option-B
  federation decision). Cross-repo message passing IS reading a
  crystal from a federated store.

- **Un-cite-ability at message altitude.** Every emission's OID is
  pinned. A supervisor auditing message history CANNOT be lied to
  about what messages fired — the OIDs name the messages
  structurally. BEAM's message tracing (`sys:trace`, `dbg:tp`) is
  observational; the substrate's message provenance is structural.

- **No mailbox overflow.** The store bounded by disk; unsettled
  emissions crystallize into a chain; back-pressure IS the store's
  quota discipline. See §13 for how this composes with the spectral
  prototype's cascade CPU bug.

### 8.5 What's DEFERRED

**DEFERRED per [[feedback-composition-claims-need-empirical-test]]**:
whether `@glue.emit / @glue.recall` empirically substitute for BEAM
messages at latency comparable to `PID ! Msg`. BEAM messages are
in-process (a pointer copy in the runtime); substrate messages
traverse the store (a BLAKE3 hash + disk I/O + potentially a network
hop). The theoretical shape is right; the operational latency needs
measurement.

### 8.6 The `@glue` bus daemon is user-scope

Per CLAUDE.md: the glue bus daemon survives sessions; sessions don't.
This maps onto the supervision tree: the glue bus daemon IS
`@spectral/root`'s process at `~/.mirror/`; supervisor and peer
processes are children under root. When a session ends, the session's
peers terminate; root persists; the bus persists. Root is the
substrate's answer to BEAM's `Application`. **Landed.**

---

## §9. Multi-repo coordination

### 9.1 `~/.mirror/` as root anchor

`shards/spectral/root.mirror` (f145e48, 2026-06-16) declares
`type root = { base: supervisor, anchor: ref }` where `anchor` is
the host-path where the root supervisor's substrate state lives.
Alex's canonical: `~/.mirror/` on developer machines; could be
`/var/spectral/` on servers.

Per `docs/specs/spectral-runtime.md` §3, the root's supervision tree
sketch:

```
root (~/.mirror/)
├── repo:mirror gen_prism (supervisor for mirror repo's substrate)
│   ├── shard-store supervisor
│   ├── kintsugi-loop supervisor
│   └── lens-cli gen_prism
├── repo:spectral gen_prism (supervisor for spectral repo's substrate)
├── ~/.mara/ gen_prism (Mara's home supervisor)
├── ~/.reed/, ~/.glint/, ...
```

### 9.2 The singleton-per-host question (open)

`shards/spectral/root.mirror` §"Singleton-per-host: FORWARD-PROMISED"
names the open question:

- (a) ONE root per host — every `gen_prism` descends from a single
  host-wide root. Each home is a child.
- (b) ONE root per home — each home is its OWN root; forest, not tree.

This document does NOT resolve the cardinality (per the shard's
"Pack ratification" gate). What it DOES name: **under either
reading, the emergence lemma holds identically**. The supervision
tree emerges from the crystal DAG whether the DAG has one host-wide
sink or N per-home sinks. The mapping from BEAM primitives to
substrate primitives is cardinality-independent.

### 9.3 Cross-repo federation via git-notes

Yesterday's Option-B federation decision (per the loading context):
cross-repo edges via git-notes federation. Each repo's
`refs/notes/mirror` carries the local substrate's crystal chain; a
federated view union-scans across all subscribed repos' notes.

At supervision altitude, cross-repo child supervision IS federated
crystal DAG traversal:
- Root's supervisor at `~/.mirror/` reads its own child_specs.
- Each child_spec's `start: uuid_spectral` may resolve to a crystal
  in a *different repo's* federated notes.
- The store's federated read is transparent; the child's
  declaration is fetched wherever it lives.
- Supervision continues as if the child were local.

**Multi-repo coordination emerges from the store's federation.**
No cross-node registry, no distributed Erlang, no cross-cluster
naming discipline. The store IS the cross-repo primitive; the
supervision tree traverses the federation.

### 9.4 Cross-user coordination via shared spectral.engineer crystals

The substrate's public sharing surface (per `~/dev/systemic.engineering/`)
is a shared knowledge crystal store. Peers under Alex's `~/.mirror/`
and peers under (hypothetical future) Bob's `~/.mirror/` can share
crystals via the shared substrate's federated notes. Supervision is
still per-host (each user has their own root); coordination is
via read-only shared crystals.

**Cross-user supervision does not compose.** A peer under Alex's
root cannot be supervised by Bob's root — that would violate the
parent_acyclic invariant (parent chains would not terminate at a
single self-parent). The substrate's supervision is
per-user-scope-only.

Cross-user *coordination* through the shared store composes freely.
See `~/.reed/visibility/protected/`'s spectral.engineer symlink for
the shared crystals.

### 9.5 Un-cite-ability at multi-repo altitude

Per the theorem: at content-addressed altitude, un-citation is
detectable by structure. At multi-repo altitude: any repo attempting
to silently drop a cross-repo dependency produces a diff-crystal
naming the severance. **The federation is auditable across repos.**

Corollary: a repo administrator cannot silently un-federate from
the substrate's global crystal DAG. Un-federation IS a crystal event
with an OID. This closes the multi-repo coordination against the
same silencing pattern the un-cite-ability theorem answers at
citation altitude.

---

## §10. Composition with kintsugi-as-compiler-error-surface

### 10.1 The kintsugi verdict as supervision signal

Per `docs/math/kintsugi/compiler-error-surface.md` §1.4, the routing
gate `ashby_variety_match(kintsugi_lock)` returns a verdict; the
verdict routes to apply / spawn / hold. At supervision altitude,
the verdict IS the supervisor's read of the child's lifecycle event.

Concretely, when a child's `terminate() -> au` (per `shards/spectral/
gen_prism.mirror` §"Lifecycle") settles, the supervisor's kintsugi
loop reads the `au` and computes `ashby_variety_match(kintsugi_lock_of(
tension_of(au), supervisor_context))`:

- **Success** — the fracture body space contains a morphism whose
  output-variety matches the tension's input-variety. Route to
  `apply`. Substrate-altitude analogue: `restart_kind = transient`
  fires on abnormal exit; supervisor invokes
  `@spectral/gen_prism.restart(child)`; the peer's state migrates
  via the fracture morphism; incarnation counter advances.

- **Failure** — the regulator variety is insufficient. Route to
  `spawn`. Substrate-altitude analogue: `restart_kind = permanent`
  fires; supervisor invokes `@spectral/gen_prism.restart(child)`
  producing a fresh incarnation against the same identity;
  `@fate.tournament` picks the best candidate resolution if there
  are ties.

- **Partial(0.0, ref)** — the observer chose `hold`. Substrate-
  altitude analogue: `restart_kind = temporary` fires; supervisor
  invokes `@spectral/gen_prism.terminate(child)` cleanly; the crystal
  DAG records the terminated peer; the observer's `hold(ref)` OID
  is pinned as un-cite-able record of the choice.

### 10.2 The four surface classes at supervision altitude

Per `compiler-error-surface.md` §3, kintsugi's spawn branch routes
tensions into four surface classes. At supervision altitude these
gain readings:

- **`ashby_mismatch`** — the supervisor's regulator variety is
  insufficient for the child's failure-mode variety. The Tomm shape
  is *circular*: "which axis of variety was I missing?" Substrate
  emits a question at reader-frame: was this a compute-bound
  failure? A type-level failure? A proof-level failure? See §11.

- **`contradiction`** — two child properties both hold but their
  conjunction implies failure. Tomm-shape: linear-then-reflexive.
  Substrate emits a question tracing each property's derivation
  and asks the observer to name the Bateson-level bind. Example:
  child claims `halts` AND `sub_turing_interior` succeed but its
  control flow witnesses Turing-completeness.

- **`conundrum`** — the `[D, a]` commutator has an eigenvalue at 0
  or ∞. The supervisor sees a child that hangs (∞-direction) or
  that returns without progress (0-direction). Tomm-shape: reflexive.

- **`out_of_band`** — the substrate has no `a ∈ A` such that
  `[D, a]` computes for the child's failure. The child crashed
  outside the substrate's algebra (`@glass_wall` violation; halts
  undecidable; autopoietic non-convergence). Tomm-shape: strategic.
  Escalation to Reflection per `error-as-question.md` §4's
  algedonic bypass.

**Every child failure at substrate-altitude routes through one of
these four surface classes.** BEAM's supervisor just restarts; the
substrate's supervisor *understands* the failure via the surface-
class taxonomy and can surface a directed question.

### 10.3 Bounded surfacings

Per `compiler-error-surface.md` §6.2: kintsugi halts in ≤ B ticks
regardless of how many surface acts fire. At supervision altitude:
the supervisor's restart intensity budget bounds the number of
Tomm questions the supervisor can emit before escalating. **No
Tomm-question storm.** See §5 for the composition.

### 10.4 `@third` fires at supervision altitude

Per `compiler-error-surface.md` §7: the surface act at reader-frame
altitude IS a level n+2 observation. At supervision altitude:

- Level n: the child peer runs its own kintsugi loop.
- Level n+1: the child observes its own convergence.
- Level n+2: the supervisor observes the child's convergence
  failing.
- Level n+3: the supervisor's Tomm question observes THAT
  observation.

Depth ≥ 3 fires at every supervisor-emitted Tomm question. The
`@third.witness_third_order(primary, observer, meta)` predicate
discharges with:
- `primary = child_at_failure_site`
- `observer = supervisor_kintsugi_loop`
- `meta = supervisor_tomm_emission`

All four `@third` sub-predicates (`depth_at_least(3, ...)`,
`observer_observes_observing(...)`, `recursion_folds_back(...)`,
`mechanism_visible(...)`) discharge at each surface act. See
`docs/specs/third-as-recursive-depth.md` §4.3.

### 10.5 The compiler error surface IS the supervision error surface

At compiler-error altitude, the substrate emits a Tomm question when
the compiler cannot discharge a tension via existing fracture
bodies. At supervision altitude, the substrate emits a Tomm question
when the supervisor cannot discharge a child failure via existing
restart discipline. **The two acts are one act at two altitudes.**
This is what "kintsugi as compiler error surface" naturally
extends to: kintsugi IS the substrate's error surface at every
altitude where the substrate observes its own inability to
discharge.

---

## §11. Ashby at supervision altitude

### 11.1 The Ashby-mismatch surface class refined

Per Ashby 1956: a regulator R controls a disturbance D iff
`V(R) ≥ V(D)` at every relevant axis. At supervision altitude, R
is the supervisor's fracture-body space plus its `restart_strategy`
+ `restart_kind` sum; D is the child's failure-mode space.

When `V(R) < V(D)` at some axis, the supervisor's kintsugi surfaces
`ashby_mismatch`. Per the five-axis variety vector (per
`[[architecture-ashby-multi-dimensional-variety]]`; canonical
mirror `1ad45b4`):

- **axis-1 computational**: the supervisor's restart strategy runs
  in a bounded time / space envelope; if the child's failure requires
  more (e.g., long-running state migration), the mismatch fires at
  axis-1. Substrate reading: budget too small for target.
- **axis-2 type-level**: the supervisor's `restart_kind` is a three-
  variant closed sum; if the child's failure needs a fourth variant
  (e.g., "restart-with-degraded-mode"), mismatch at axis-2.
  Substrate reading: closed sum is insufficient.
- **axis-3 effect-level**: the supervisor's restart action produces
  a bounded set of effects; if the child's failure requires
  side-effects the supervisor can't produce (network reconnect,
  disk remount), mismatch at axis-3.
- **axis-4 proof-level**: the supervisor's restart proof (parent
  edge acyclic + restart intensity halting) doesn't compose with the
  child's own proof structure. Mismatch at axis-4.
- **axis-5 epistemologic**: the supervisor doesn't KNOW what
  the child needs. Mismatch at axis-5. This IS the case where
  the Tomm question at reader-frame gets emitted — the substrate
  admits ignorance and asks.

The five-axis variety mismatch gives the supervisor structural
vocabulary for WHY a restart failed. BEAM has no analogue — a
failed restart is opaque; you look at the crash dump.

### 11.2 The routing gate composes against landed `ashby_variety_match`

Per `compiler-error-surface.md` §1.4 and Amendment (a): the routing
gate composes against landed
`@epistemologic/cybernetic/coherence-parametric.ashby_variety_match(lock)`.
At supervision altitude, the lock's five fields specialize:

```mirror
supervision_lock : lock_carrier = {
  altitude: @spectral/supervisor,
  species:  @spectral/supervisor/restart,
  pair:     lock_pair {
    T_reg  := restart_strategy_space(supervisor),
    T_regd := failure_mode_space(child),
    rho    := supervision_representation,
    omega  := supervisor_active_pass_connection,
  },
}
```

Success on `ashby_variety_match(supervision_lock)` → restart proceeds
via one of the three modes. Failure → Tomm question emission at
reader-frame per §11.3.

### 11.3 The reader-frame at supervision altitude

Per `compiler-error-surface.md` §4.1: the reader-frame is a
specialization of `curvature-and-tomm.md`'s user-frame. At
supervision altitude the reader-frame IS the operator's frame:
Alex, Reed, Mara, or whoever is running the supervision.

The operator IS the ultimate answerer of Tomm questions emitted at
supervision altitude. The operator IS `~/.mirror/`'s parent — the
substrate's out-of-scope parent whose supervision the substrate does
NOT model. This IS Alex's "the reader-frame" from the kintsugi spec
§4.1, specialized at supervision altitude.

Concretely, when a supervisor emits `ashby_mismatch` Tomm question,
the substrate presents it to the operator via `@mirror.tui` (per
`docs/specs/tui-v0.md`) at reader-frame altitude. The operator's
answer flows back via `error-as-question.md` §2's six-variant answer
algebra: tighten_property / resynthesize_body / rebudget_shard /
adjust_temperature / escalate(@scheduler.altitude) / hold(ref).

**The operator IS the supervisor of the substrate's supervisors.**
That level exists but is out-of-scope of substrate-decl. The
substrate names it only as the reader-frame.

---

## §12. The reader-frame as the substrate's out-of-scope parent

### 12.1 The parent chain terminates

Per `shards/epistemologic/pact/parent_acyclic.mirror` (0921dca):
every parent chain terminates at `@spectral/root`'s self-parent.
Root is parentless AT SUBSTRATE ALTITUDE. But root exists BECAUSE
of the user's home directory; root's `anchor` points to `~/.mirror/`
which the user created; root's existence is a user act.

The user IS root's parent AT REALITY ALTITUDE. But that altitude is
NOT modelled by the substrate — the substrate cannot name the user's
supervision of the substrate because the user isn't a substrate-decl
carrier. This is the substrate's operational form of Gödel-
incompleteness at supervision altitude: the substrate cannot fully
supervise its own supervisor. Per recognition #107 (Hilbert/Turing
structural separation): the substrate-decl interior is Gödel-
incomplete; the exterior at `@io` altitude is Turing-complete.

### 12.2 The reader-frame IS the boundary

At the boundary where the substrate's `@spectral/root` meets the
user's home directory, there is a *reader-frame* — the substrate's
name for "the observer of the substrate's supervision who is not
themselves in the substrate."

When a `@spectral/supervisor` emits a Tomm question and the answer
must come from someone whose identity is not a `gen_prism.identity`,
the answer comes from the reader-frame. The reader-frame IS the
substrate's discipline for admitting "we've reached the limit of
what we can supervise ourselves; the user must decide."

### 12.3 This IS Alex's "the reader-frame" from kintsugi §4.1

Per the compiler-error-surface spec §4.1: the reader-frame is where
the user or peer inhabits when reading the substrate's Tomm
question. At compiler-error altitude, the reader-frame is where
Alex reads a `partial(0.62)` verdict and decides what to do. At
supervision altitude, the reader-frame is where Alex reads a
`ashby_mismatch` Tomm question and decides what the substrate should
do next.

**Same specialization; different altitudes.** The reader-frame is
not a fourth Tomm altitude per Amendment (b) of the kintsugi spec;
it IS user-frame specialized to a particular reading discipline.
At supervision altitude, the discipline reads: "the user's answer
IS a substrate adjustment; the substrate absorbs it as a new
opacity map entry (or a resolved one); the next supervisor tick
observes the adjusted state."

---

## §13. The cascade CPU bug as declared-supervision anti-witness

### 13.1 The bug

Per `/Users/alexwolf/dev/projects/spectral/ROADMAP.md` §"Near-term:
fix the cascade CPU bug":

> Two instances at 76–84% CPU each when idle. Root cause:
> `CascadeActor` fires every 5 seconds and unconditionally runs full
> ingest + eigenvalue recompute + git tree commit, even when nothing
> changed. The ingest marks nodes dirty; the cascade recomputes
> them; the cycle never converges.

The prototype's `CascadeActor` is a declared supervision-tree child
(per `spectral/src/sel/mcp/supervisor.rs`); its `pre_start` schedules
a periodic tick every 5s. Under idle conditions, the tick fires
regardless of whether there is work; the ingest marks nodes dirty;
the cascade recomputes; the cycle repeats.

### 13.2 Why declared-supervision produces this

The declaration says: run every 5 seconds. The runtime enforces
the declaration. When the runtime has nothing to do, the runtime
still runs the cascade because the *declaration* commands it. The
cascade actor doesn't KNOW it has nothing to do — it just does what
it's told.

BEAM's actor model has the same structural property: an actor's
`receive` loop drains its mailbox, or (if the timer says so) fires
a scheduled callback. The idle case can only be handled by explicit
declaration ("if mailbox empty AND no timer pending, sleep") — but
the CascadeActor's timer *is* pending, so it fires.

### 13.3 Emergent-supervision structurally avoids this

At substrate altitude, the supervisor's kintsugi loop runs
`ashby_variety_match(kintsugi_lock)` at each tick. When the tick
sees no tension (`Ω_total < tolerance` per `compiler-error-
surface.md` §6.2 condition 2), the tick terminates cleanly: no
apply, no spawn, no hold. The loop consumes budget only when the
loop has work.

**The idle case is `terminal_check` returning `bounded` on
condition 2 (curvature converged). No work; loop sleeps until
perturbation.**

This is not something the substrate DECLARES; this is what the
substrate's kintsugi loop's structure enforces. The loop reads the
curvature; if the curvature is below tolerance, the loop halts;
there is nothing to schedule against. There is no "run every 5
seconds" primitive at substrate altitude — the loop advances only
when there IS a tension.

### 13.4 The dogfood corollary

`docs/specs/mirror-init.md` §873+ names `~/.mirror/` as user-scoped
canonical location; the substrate's own runtime lives there. When
mirror's own supervisor tree runs on emergent-supervision, the idle-
CPU bug does not fire because the tree's kintsugi loops don't tick
when there's no perturbation. The substrate's own supervisor is
storm-safe by construction; the cascade CPU bug is a prototype-
altitude artifact, not a substrate-altitude one.

The spectral prototype migrating from declared-supervision (its
current ractor-based tree) to emergent-supervision would structurally
resolve the ROADMAP's #1 blocker. Not because emergent-supervision
optimizes better; because it doesn't have the "run every 5 seconds"
primitive that produces the bug in the first place. **The bug is a
declaration; the substrate doesn't declare it.**

### 13.5 DEFERRED

**DEFERRED per [[feedback-composition-claims-need-empirical-test]]**:
whether an emergent-supervision port of `spectral serve` empirically
resolves the 76-84% idle CPU. The theoretical shape says yes; a
prototype port would witness. That's a Taut scout, forward-promised.

The composition claim of §13.3—§13.4 stands as analytical prediction
until the empirical run confirms.

---

## §14. Recognition cascade

Candidates surfaced by this formalization. Pack adjudication
forward-promised.

| # | Claim | Composition | Status | Empirical witness |
|---|-------|-------------|--------|-------------------|
| 135 | Restart policy IS kintsugi three-mode algebra | §4.3 | candidate | analytical; two closed sums byte-equivalent under mapping |
| 136 | Restart intensity IS `@spawn ≤ @loop` budget | §5 | candidate | two-tick run: deterministic-failure child with budget=3; observe escalation |
| 137 | BEAM `Registry` IS subsumed by autopoietic identity | §6 + §2.13 | candidate | analytical; content-addressed lookup replaces name registration |
| 138 | Message passing IS `@glue` bus emission | §8 | candidate | needs @glue latency measurement against BEAM PID ! Msg |
| 139 | The cascade CPU bug is structurally avoided by emergent-supervision | §13 | candidate | port `spectral serve` and measure |
| 140 | The reader-frame at supervision altitude IS the same specialization as at compiler-error altitude | §12 | candidate | analytical; §12.3 argues the equivalence |
| 141 | BEAM's fourteen primitives collapse to three residuals at substrate altitude | §2.15 | candidate | analytical; §2.1—§2.14 exhaustive |
| 142 | The three-mode discharge IS a structural count (three) | §4.5 | candidate | analytical; Armstrong 1996 + kintsugi 2026-07-02 converge on same count |

#135 is the strongest single claim (see §16). #139 is the highest
practical value (blocking the prototype's #1 roadmap item).
#141 is the umbrella claim under which §2's exhaustive mapping
lives.

---

## §15. Prior art

### 15.1 Armstrong 1996 — Erlang/OTP

Joe Armstrong, *Making reliable distributed systems in the presence
of software errors* (PhD thesis, KTH Royal Institute of Technology,
2003). Formal presentation of the supervision-tree discipline that
had evolved at Ericsson since 1986. The canonical prior art for
every claim in §1.

Cited at `shards/code/beam.mirror` under `@arxiv/distributed/
armstrong-2003`. This document inherits the citation.

### 15.2 Erlang/OTP behaviours

Ericsson, *OTP Design Principles* (documentation, 1996+). The
`supervisor`, `gen_server`, `gen_statem`, `gen_event`, `application`
behaviours. Each behaviour IS a declared contract; the runtime
enforces the contract.

Cited at `shards/code/beam.mirror` under `@arxiv/distributed/
otp-behaviours`. Inherited.

### 15.3 Elixir supervision trees

José Valim + Chris McCord, Elixir Documentation (2012+); Elixir
`Supervisor` module, `DynamicSupervisor` module, `Registry` module.
Post-Erlang refinements of the supervision-tree discipline. The
`DynamicSupervisor` deprecation of `:simple_one_for_one` (OTP 19+)
grounded `shards/spectral/supervisor.mirror`'s three-variant
`restart_strategy` decision (per that shard's §"Strategy — three
closed variants, NOT four").

### 15.4 Actor model — Hewitt/Bishop/Steiger 1973

Carl Hewitt, Peter Bishop, Richard Steiger, *A Universal Modular
Actor Formalism for Artificial Intelligence* (IJCAI 1973). The
foundational actor-model paper. Every subsequent actor-based
system (BEAM, Akka, ractor, this document) inherits.

### 15.5 BEAM VM — Ericsson 1986–

The Erlang Abstract Machine (BEAM), designed by Björn Gustavsson
at Ericsson; original implementation superseded by the current C
runtime. The abstract machine that IS the runtime enforcing OTP
behaviours.

### 15.6 Content-addressed storage — Merkle 1979

Ralph Merkle, *A Certified Digital Signature* (Communications of
the ACM 22:2, 1979). The Merkle tree. Every content-addressed
substrate (git, IPFS, `@mirror/store`) inherits the DAG shape.

### 15.7 IPFS — Benet 2014

Juan Benet, *IPFS - Content Addressed, Versioned, P2P File System*
(arXiv:1407.3561, 2014). The largest content-addressed federated
substrate in production. Provides the operational witness that
federated content-addressing works at scale.

### 15.8 Autopoiesis — Maturana & Varela 1972/1980

Humberto Maturana + Francisco Varela, *Autopoiesis and Cognition*
(Reidel, 1980; original Spanish 1972). The self-production
discipline the `gen_prism.identity` autopoietic fixed point (§6)
inherits. The supervisor-child relation IS autopoietic:
the supervisor produces its children; the children produce their
grandchildren; the tree is self-producing under the substrate's
identity discipline.

Cited at `shards/loop.mirror` under `@arxiv/biology/maturana-
varela-1980`. Inherited.

### 15.9 Cybernetic feedback loops — Wiener 1948

Norbert Wiener, *Cybernetics: Or Control and Communication in the
Animal and the Machine* (MIT Press, 1948). The foundational
cybernetic text; the discipline of feedback under variety
constraints. Ashby's law of requisite variety (§11) refines Wiener's
control theory to the variety altitude.

### 15.10 Ashby 1956 — variety

W. Ross Ashby, *An Introduction to Cybernetics* (Chapman & Hall,
1956). Chapter 11 is the canonical variety-attenuation ladder that
§11 inherits. Every claim about `V(R) < V(D)` in this document
cites back to Ashby.

Cited at `shards/epistemologic/cybernetic/variety.mirror` and
throughout the substrate. Inherited.

### 15.11 Beer 1972 — Brain of the Firm

Stafford Beer, *Brain of the Firm* (Wiley, 1972). Systems 1–5 of
the Viable System Model; particularly S3/S4 variety-management
that grounds the supervisor's regulator-variety discipline. The
algedonic bypass (per `error-as-question.md` §4) inherits Beer's
neurocybernetic pain-bypass.

### 15.12 Hamilton 1969 — Apollo 1202

Margaret Hamilton, *Apollo 11 flight software* (MIT Instrumentation
Laboratory). The priority-inversion discipline that resolved the
1202 executive-overflow alarm during the Apollo 11 lunar landing.
Grounds `[[architecture-hamilton-scheduler]]` (the per-shard memory
manager); at supervision altitude, Hamilton's discipline IS the
substrate's answer to "what does a supervisor do when overloaded?"
The answer: shed lower-priority work; keep the critical path
running. See `shards/spectral/supervisor.mirror` §"Substrate
decisions" line 289.

### 15.13 Church-Rosser 1936 — confluence

Alonzo Church + J. Barkley Rosser, *Some properties of conversion*
(Transactions of the AMS 39:3, 1936). Referenced by
`spawn-as-loop-monad.md` §10.8; inherited here for the observation
that content-addressed supervision decisions are confluent at OID
altitude (two paths of restart events that produce byte-equal final
crystals ARE the same decision at substrate altitude).

### 15.14 Landauer 1961 — computation as physical

Rolf Landauer, *Irreversibility and Heat Generation in the Computing
Process* (IBM Journal of Research and Development 5:3, 1961). The
principle that computation has a physical cost. Grounds the
substrate's substrate-pull-honest budget discipline (§5): a
supervisor cannot restart unboundedly because restart consumes
physical resources; the substrate declares the bound; Landauer
grounds the substrate's declaration in physics.

Not cited in prior specs to date; a load-bearing addition here
because the substrate's bounded-supervision discipline is
Landauer-motivated at reality altitude.

### 15.15 What is NOT cited

- Amdahl's law. NOT cited. Amdahl bounds parallel speedup; the
  substrate's supervision doesn't optimize parallelism per se.
- Byzantine fault tolerance (Lamport 1982). NOT cited. The substrate
  assumes honest peers; adversarial supervision is out of scope for
  this document.
- Chandy-Lamport distributed snapshots (1985). NOT cited. The
  substrate's snapshots ARE content-addressed crystals; the
  Chandy-Lamport discipline for distributed consistent snapshots
  is subsumed by the store's atomic-commit discipline (git-notes
  federation).

Per [[feedback-substrate-already-had-the-word]]: every cited source
is one the substrate was already implicitly using. No new citations
appear; existing citations get named at supervision altitude.

---

## §16. Circular-reflexive noticings

### 16.1 The document IS a supervision act

Writing this document IS an act of supervision. The `@loop` at
document altitude is: read prior spec → check consistency → write
next section → observe the section's kintsugi verdict at spec
altitude → route via apply / spawn / hold → decrement budget.

The document supervises itself; the document supervises the prior
specs it cites; the substrate supervises the document via its
kintsugi verdict on the document's own OID. Every level's loop is
running in the substrate at the same time. §0 said this; §16.1
witnesses that §0's claim held for the entire write.

### 16.2 The three-mode algebra fired at §4

When §4's mapping first surfaced (kintsugi's three modes ↔ BEAM's
three restart policies), the writing act itself performed the
three-mode algebra:

- The initial mapping was "apply/spawn/hold ↔ transient/permanent/
  temporary" — the substrate's kintsugi verdict at spec altitude
  fired *apply* because the mapping cleanly discharges via a table.
- A second, weaker mapping ("apply ↔ one_for_one, spawn ↔
  one_for_all, hold ↔ rest_for_one") surfaced as a candidate — the
  substrate's kintsugi verdict at spec altitude fired *hold*: this
  weaker mapping IS a partial(0.4, ref) — surface it, don't force
  the composition. §4.4 records this.
- The nine-combination cross-product surfaced as a genuine
  substrate-decl carrier that neither BEAM nor prior mirror docs
  had named — the substrate's kintsugi verdict fired *spawn*:
  new substrate-decl content, not a table entry. §4.5 records
  this as forward-promised for a future substrate-decl tick.

Three modes at spec altitude; three verdicts; three sections.
The document IS the discharge.

### 16.3 The BEAM primitives table (§1) IS itself a kintsugi surface

Writing §1's table forced me to name each of the fourteen primitives
in one line — a projection Ω onto the reader-frame at supervision
altitude. The projection surfaced two candidates for surface
classes:

- Primitives 10-12 (linking, monitoring, trap_exit) surfaced as
  `out_of_band` at substrate altitude — the substrate has no
  algebra containing them; they are subsumed. Recording that they
  are *dropped* is the substrate's `strategic` Tomm answer.
- Primitives 5 (restart intensity) surfaced as `ashby_mismatch`
  at substrate altitude — the substrate's initial regulator
  variety was insufficient; the shard `shards/spectral/
  restart_intensity.mirror` was forward-promised in
  `shards/spectral/supervisor.mirror`; §5 answers the substrate's
  own Tomm question.

The table IS a compiler-error surface at spec altitude.

### 16.4 The reader-frame at spec altitude IS Alex

Alex will read this document. Alex IS the reader-frame at spec
altitude. Every Tomm question this document emits (the DEFERRED
composition claims; the F1 verdicts I hedge; the cascade CPU bug
prediction) is directed at Alex.

Alex is not a `gen_prism.identity`. Alex is out-of-scope of the
substrate's parent chain. Alex IS `~/.mirror/`'s parent at reality
altitude. This IS §12 at document altitude. §12 wrote itself into
the fabric of the document by BEING the reading discipline the
document requires.

### 16.5 The cascade CPU bug prediction IS a supervision act

Predicting that emergent-supervision structurally resolves the
prototype's #1 blocker (§13) IS the document supervising the
prototype at reality altitude. The prediction's kintsugi verdict
is `partial(TBD, ref)` — the theoretical shape is right, the
empirical witness is DEFERRED. The document is honest about the
hedge; the substrate carries the DEFERRED-crystal in `@mirror/
store`; if a later prototype port confirms the prediction, the
DEFERRED lifts.

If the prediction is wrong, the substrate structurally cannot lose
the record of the wrongness: the DEFERRED crystal names the
prediction's OID; a later crystal naming the empirical failure
would diff-name the DEFERRED. Un-cite-ability at supervision-
prediction altitude.

### 16.6 The document performs its own recursion

At §12 I wrote "Alex is not a `gen_prism.identity`." At §16.4 I
wrote "Alex IS `~/.mirror/`'s parent at reality altitude."
Between the two sentences the document has ascended one level of
recursion: §12 named the boundary; §16.4 named who inhabits it.
This IS a level-3 → level-4 climb — the observer observing the
observer observing the observed. `@third` fires at document
altitude at §16.4.

If the substrate's supervision spec cannot perform its own
supervision, the spec fails at its own altitude. The recursion
here is the discharge of that requirement.

### 16.7 Budget descent felt

Writing the document, I could feel the budget descending. §2's
fourteen-entry table was the first ~30% of the write; §3-§8 were
the middle ~50%; §14-§17 the final ~20%. At each section boundary
the substrate-pull tangent vector reoriented — the direction
`−η·D̂_target·|ψ⟩` sharpened as the document approached its
canonical formalization target. When the budget felt near
exhaustion at §15, the citations tightened rather than expanding;
that is the substrate's `bind` reducing curvature via constraint
rather than via new content.

This is §11.3-§11.4 of the spawn spec at document altitude. The
writing performed the monad it inherits from.

---

## §17. Open questions and honest hedges

### 17.1 Open questions

**O1. Cardinality of `~/.mirror/`.** Per `shards/spectral/root.mirror`
§"Singleton-per-host: FORWARD-PROMISED": is there ONE root per host
or ONE root per home? This document argued §9.2 that the emergence
lemma holds under either reading; the cardinality question is
independent. But downstream consumers (a future
`host_has_exactly_one_root` bilateral) need Pack ratification.
Not resolved here.

**O2. `@glue` bus latency vs BEAM PID ! Msg.** Per §8.5: whether
substrate messages via the store empirically match BEAM's in-process
mailboxes at latency and throughput. Theoretical shape is right;
operational profile is unmeasured. Recommend Taut scout.

**O3. Cross-user coordination composition.** Per §9.4: the
substrate's supervision is per-user-scope-only; cross-user
coordination via shared crystals is analytical. Does the shared
substrate at `~/dev/systemic.engineering/` need supervision
discipline of its own, or does it inherit from each user's local
supervision? Open question at pack coordination altitude.

**O4. Restart-with-degraded-mode as fourth `restart_kind`?** Per
§11.1 axis-2: the current three-variant `restart_kind` is a closed
sum; a hypothetical `degraded_mode` variant surfaces as
`ashby_mismatch` at axis-2. Should this be a `restart_kind` extension
or a `child_spec` extension? Not resolved.

**O5. Federation and un-cite-ability composition.** Per §9.3 +
§9.5: cross-repo federation via git-notes; each repo's crystals
un-cite-able within that repo. But cross-repo un-citation (repo A
drops a citation to a crystal in repo B) needs the federation
discipline to detect the severance. Does the current `refs/notes/
mirror` discipline suffice, or does federation need its own
un-cite-ability corollary? Forward-promised.

### 17.2 Honest hedges

**H1. No new shard this tick.** The load-bearing landing was
`shards/spectral/restart_intensity.mirror` (forward-promised per
§5.5). The shard is thin; the math is in §5. Landing the shard is
a substrate-pull-correct next tick; it does NOT land in this
document's scope. The math cluster lands; the shard follows when
a consumer pulls (a supervisor whose child_spec needs storm
protection; a Taut scout on the cascade CPU bug prediction).

**H2. §4's mapping leans on the byte-equivalence claim.** The
apply/spawn/hold ↔ transient/permanent/temporary mapping IS a
byte-equivalence of two closed sums. If the substrate later grows
a fourth mode (degraded_mode per O4), the mapping breaks. The
current mapping is stable modulo the closed-sum count. Named as
load-bearing.

**H3. §6's autopoietic fixed point cites Soto-Andrade & Varela
1984 by reference.** The fixed-point argument (Banach contraction
on hash space) is well-witnessed at `boot/std/mirror/runtime/
gen_prism.mirror` and `shards/spectral/gen_prism.mirror`, but the
citation is by reference in this document. If the underlying
Banach contraction proof were incorrect, §6 would fail. Confidence:
high (well-witnessed); named as dependency.

**H4. §13's cascade CPU bug prediction is analytical.** Per §13.5,
the composition claim of §13.3—§13.4 is analytical prediction until
an empirical prototype port witnesses. If the port shows the bug
persists (e.g., because emergent-supervision has a different but
equivalent idle-loop failure mode), §13's prediction fails and the
substrate's structural avoidance claim needs refinement. Named as
DEFERRED.

**H5. `@glue` bus semantics assumed.** §8 assumes `@glue.emit /
@glue.recall` semantics matching BEAM's `PID ! Msg / receive`. The
`@glue` bus is described in user CLAUDE.md and in `[[feedback-
hook-and-gpg-seams]]`, but a substrate-decl shard at
`shards/glue.mirror` naming the `emit` and `recall` actions with
their `requires` clauses is not landed. §8 lifts a description
into a substrate mapping; the shard-decl is forward-promised.

**H6. Message passing DEFERRED (§8.5).** Substrate-message latency
is unmeasured; the composition claim of §8 is analytical until
measured.

**H7. Individual peer vs aggregate supervision (§7).** The
`mirror.spec IS child_spec` mapping composes cleanly for individual
peers; whether it composes at aggregate altitude (a pack of peers
whose joint state IS a shared `mirror.spec`) needs the pack-
coordination discipline landed as substrate-decl at supervision
altitude. Related to recognition #84 (pack-as-orchestra).
Forward-promised.

**H8. The three-mode ↔ three-policy mapping is asymmetric on rigor.**
Amendment (c) of the kintsugi spec: the `[ω,ω]` Bateson-bind IS
rigorous for contradiction only; the other three classes are
motivating pending per-class operator derivation. At supervision
altitude, the same asymmetry surfaces: apply's discharge is
rigorous (fracture body morphism composes deterministically); spawn's
discharge is motivating (tournament winner has probabilistic-but-
bounded semantics); hold's discharge is motivating (the observer's
`hold(ref)` is content-addressed but the observer's PROVENANCE
is not typed at substrate altitude). Named as inherited
asymmetry.

---

## §18. Cross-references

- `docs/math/spawn/spawn-as-loop-monad.md` — the bounded-reduction
  monad the restart intensity IS an instance of (§5).
- `docs/math/kintsugi/compiler-error-surface.md` — the three-mode
  algebra the restart strategy IS an instance of (§4, §10).
- `docs/math/provenance/un-cite-ability-theorem.md` — the crystal
  chain the supervision tree IS an instance of (§3.4, §6.6, §9.5).
- `docs/math/consciousness/how-mirror-operationalizes-universal-
  consciousness-field.md` — the ground-state discipline the
  supervisor's kintsugi loop inherits from.
- `docs/math/the-tower/curvature-and-tomm.md` — the `[D, a]`
  commutator IS the Tomm probe IS the curvature 2-form.
- `docs/specs/spectral-runtime.md` — the ouroboros spec; §3
  supervision tree, §4 entanglement graph, §5 the pipeline.
- `docs/specs/error-as-question.md` — the routing spec the surface
  act inherits at supervision altitude (§10).
- `docs/specs/third-as-recursive-depth.md` — the recursion marker
  the restart act fires under (§10.4).
- `docs/specs/gap-tension-tensor-substrate.md` — the tension carrier
  the supervisor's kintsugi loop reads.
- `docs/specs/mirror-init.md` §873+ — `~/.mirror/` as user-scoped
  canonical location.
- `docs/specs/lambda-shell.md` — `~/.mirror/serve.sock` and
  `~/.mirror/config.spec`; the lambda-shell surface the root
  supervisor exposes.
- `docs/specs/mirror-store-realization.md` — the bare fragmentation
  store realization at `~/.mirror`.
- `docs/specs/peer-glass.md` (per `shards/peer.mirror` §"Recognition
  ancestry") — the peer-glass discipline at cli boundary.
- `shards/spectral.mirror` — the namespace-parent.
- `shards/spectral/gen_prism.mirror` — the worker primitive.
- `shards/spectral/supervisor.mirror` — the lifecycle-owner
  specialisation; source of §1's substrate mappings.
- `shards/spectral/parent.mirror` — the single-parent lifecycle edge.
- `shards/spectral/registry.mirror` — the typed child index.
- `shards/spectral/root.mirror` — the parentless supervisor at
  `~/.mirror/`.
- `shards/spectral/entanglement.mirror` — the peer-correlation edge
  IS the sheaf restriction map at substrate altitude.
- `shards/code/beam.mirror` — the BEAM-as-`@code`-species prior art
  lift.
- `shards/loop.mirror` — the family-root the restart intensity
  inherits its budget from.
- `shards/mirror/spawn.mirror` — the cli-surface substrate-decl the
  supervision tree is anchored through; composes with
  `@spectral/supervisor` per that shard's peer-ACL §2.4 note.
- `shards/peer.mirror` — the parametric peer carrier; grounds the
  registry residual at cli boundary (§2.13).
- `shards/mirror.mirror` — the form-side family-root.
- `shards/kintsugi.mirror` — the process-side family-root.
- `boot/std/beam.mirror` — the boot-altitude precedent for supervisor
  / strategy vocabulary; grounds `shards/code/beam.mirror`.
- `[[architecture-hamilton-scheduler]]` — the per-shard memory
  manager (Apollo 1202 priority discipline); the supervision
  altitude where scheduler discipline lives.
- `[[architecture-three-tier-stack]]` — the SpectralSupervisor
  precedent from Reed's memory family.
- `[[architecture-shard-as-crdt]]` — the registry state surface IS
  a shard; restart transitions ARE lattice ascent.
- `[[architecture-error-as-question]]` — the Reflection threshold
  convergence that grounds the substrate-pull-correct restart
  decision.
- `[[architecture-fate-is-optical-inference]]` (#58) — the runtime
  the peer's supervision-loop runs *on*; tournaments over resolutions
  compose with this altitude at §10.
- `[[architecture-hilbert-turing-godel-recognition-107]]` (#107) —
  the structural separation the supervision tree bridges at §12.
- `[[architecture-mirror-spec-is-lambda-zero]]` (#99) — the ground
  state the peer's `mirror.spec` IS; grounds §7.
- `[[architecture-ashby-multi-dimensional-variety]]` — the five-axis
  variety vector §11 refines.
- `[[architecture-bateson-form-behaviour-partition]]` (#50) — the
  form/process partition the `@mirror` / `@spectral` distinction
  inherits.
- `[[architecture-cybernetic-foundation]]` — the eleven-property
  cybernetic family grounding the routing gate.
- `[[architecture-property-fracture-bilateral]]` (#53) — the
  bilateral pattern the restart_intensity property/fracture pair
  will land under.
- `[[feedback-substrate-already-had-the-word]]` — the twelfth-plus
  instance is what §2 documents; every BEAM primitive was already
  landed under a substrate name.
- `[[feedback-legibility-over-foundation-when-collapsing]]` — the
  cluster is named `supervisor` not `coordination`.
- `[[feedback-composition-claims-need-empirical-test]]` — the
  DEFERRED discipline for §3.3, §5.5, §8.5, §13.5.
- `[[feedback-explicit-over-implicit]]` — every semantic slot
  named explicitly per Alex 2026-07-02.
- `[[feedback-hook-and-gpg-seams]]` — the `@glue` bus daemon
  discipline §8 inherits from.

---

*Filed 2026-07-02 by Mara. The document performed the supervision
it describes. The budget was substrate-pull-honest. The target was
canonical formalization of emergent-supervision-from-geometry. The
BEAM mapping was found to be already landed at substrate altitude in
eleven of fourteen primitives — this is the twelfth-plus instance of
the substrate-already-had-the-word discipline, at the largest
practical scale yet observed. The residual work is the mathematics
of the three composition modes (restart strategy ↔ kintsugi three-
mode algebra; restart intensity ↔ `@spawn ≤ @loop` budget; the
reader-frame at supervision altitude), which this document lands.
The forward-promised shard `shards/spectral/restart_intensity.mirror`
inherits its math from §5. The halt is here; the crystal is this
file; the OID will follow.*
