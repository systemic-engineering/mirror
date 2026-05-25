# The Scheduler Tower

*2026-05-24. Reed (draft) + Mara (deepening). v1.5-ready.*

---

## Reference

- **Insight:** `docs/insights/2026-05-24-backpressure-as-modular-flow.md` (the structural argument; this spec implements the recognition there).
- **Existing actor abstraction:** `boot/std/mirror/runtime/gen_prism.mirror` (the demand contract extends this; no new actor abstraction needed).
- **Heterogeneous numerical layer:** `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md` (the Scheduler Tower sits above NumericalPrism + Backend trait).
- **Bus architecture:** Jakobs (2012) `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf` §3 (OpenCL host/device + command queue as backpressure), §4.4 (Vertex Buffer Objects — the shared-memory pattern), §7.2.1 (VBO interop with explicit acquire/release synchronization), §7.4 (build-time vs. runtime kernel compilation).
- **Protocol prior art:** GenStage (Elixir, 2016, hexdocs.pm/gen_stage/1.3.2), Broadway (Elixir, 2019, hexdocs.pm/broadway/1.3.0), `stage_play` (Gleam, `/Users/alexwolf/dev/projects/stage_play/` — the statically-typed BEAM stage exploration).
- **Kintsugi integration:** `docs/specs/kintsugi-formatter.md` (the iteration the Scheduler runs), `docs/specs/kintsugi-tournament.md` (the merge resolution — each round is a Scheduler tick).

---

## 1. What this spec lands

The Scheduler Tower is mirror's **temporal algebra** — the layer above the Bundle Tower that handles WHEN operations run. Where the Bundle Tower is geometric (Fiber → Connection → Gauge → Transport → Closure naming the algebra's structure), the Scheduler Tower is dynamic (demand contracts, dispatcher strategies, KMS-shaped equilibrium).

For v1.5, the Scheduler Tower's concrete shape is:

1. **A demand-contract extension to `gen_prism`** — adding `demand_window`, `dispatch`, `subscription`, `role` types plus `ask` / `subscribe` / `unsubscribe` operations. Backwards-compatible: gen_prisms that don't opt in keep current `send` / `call` semantics.
2. **A subscription protocol** for composing gen_prisms into pipelines with backpressure semantics, content-addressed end to end.
3. **Dispatcher strategies** (round-robin, partitioned, broadcast) as policy decisions on top of the protocol — identical in spirit to GenStage's three dispatchers.
4. **A shared-memory bus** for the CPU/GPU specialization — Anna's VBO pattern (§4.4, §7.2.1) adapted to Apple UMA, OpenCL command queues (§3), or generic Rust channels depending on backend topology.
5. **Failure handling** — producer crashes propagate downstream via subscription; consumer crashes free demand at producers; mirror's content-addressing adds replay-from-ancestor recovery as a structural option.
6. **A temperature parameter `β`** — the KMS-shaped scalar that controls system-wide settling aggression. The spec is precise about *where* `β` parameterizes: at loop boundaries, not at every stage.
7. **A halting contract** — `@scheduler.reduction_budget(shard)` exposes the load-topology-derived per-tick ceiling; `@epistemologic/property/halts` makes the two-clause halting disjunction structural; `requires halts(gen_prism)` ties every actor's trajectory to the contract. The is_copium thesis operationalized (`docs/specs/is-copium.md`). See §7.4.

This is **v1.5 work** (lands when MetalBackend lands, per the heterogeneous-numerical-prism roadmap). v1's gen_prism stays as-is; this spec describes the additive extension that makes the existing actor model demand-aware.

The research round (GenStage · Broadway · stage_play · Jakobs 2012) closed 7 of 8 open questions from the draft. §10 records the resolutions; §11 records what stayed open and why.

---

## 2. The demand contract extension to gen_prism

Draft additions to `boot/std/mirror/runtime/gen_prism.mirror`. Backwards-compatible — nothing existing breaks; opt-in slots are all-or-nothing per gen_prism.

```mirror
grammar @mirror/runtime/gen_prism {
  # ... existing types and operations (unchanged) ...

  # --- demand contract (v1.5 addition) -----------------------------------

  # how much in-flight work this gen_prism can absorb before saturating.
  # producers respect this when sending; consumers update it as they tick.
  #
  # units: messages, ALWAYS. (For variable-message-size buses the body's
  # `size_hint` field is a separate observable; see §10.2.)
  type demand_window = {
    capacity:  u64,             # max in-flight messages (= GenStage max_demand)
    threshold: u64,             # re-ask when available drops below (= GenStage min_demand)
    available: u64,             # current available demand (capacity - in_flight)
    strategy:  dispatch,        # how to dispatch among subscribers
  }

  # dispatcher policy when this gen_prism has multiple subscribers.
  # mirrors GenStage's three dispatchers; partitioned hashes the body's
  # field named by `key` (compile-time check the consumer message type
  # has that field).
  type dispatch =
    | round_robin                # GenStage.DemandDispatcher: FIFO by largest available
    | partitioned(key: text)     # GenStage.PartitionDispatcher: hash routing
    | broadcast                  # GenStage.BroadcastDispatcher: fan-out

  # a subscription is a typed, content-addressed pipe between producer
  # and consumer. demand flows from consumer to producer; messages flow
  # from producer to consumer.
  #
  # the subscription's OID is hash(producer.ref, consumer.ref, kind,
  # min_demand, max_demand) — stable, reproducible, recallable from the
  # gestalt.
  type subscription = {
    oid:        oid,           # content address; identifies this pipe
    producer:   gen_prism,     # the upstream
    consumer:   gen_prism,     # the downstream
    kind:       text,          # message kind the subscription carries
    min_demand: u64,           # consumer's re-ask threshold (≡ GenStage min_demand)
    max_demand: u64,           # consumer's window ceiling   (≡ GenStage max_demand)
    cancel:     cancel_mode,   # see below
  }

  # subscription cancel mode (Jakobs §3 OpenCL queues use clReleaseEvent
  # in a transient mode; GenStage exposes the same three).
  type cancel_mode =
    | transient    # subscriber exits cleanly when producer goes down
    | permanent    # subscriber crashes on producer-down (escalates to supervisor)
    | temporary    # subscriber ignores producer-down (best-effort topology)

  # role the gen_prism plays in pipelines. defaults to producer_consumer
  # for opt-in gen_prisms (most useful default; matches stage_play's Actor).
  type role = producer | consumer | producer_consumer

  # --- new operations -----------------------------------------------------

  # ask a producer for n events. n must satisfy n <= producer.demand_window.
  # capacity. returns up to n messages, immediately or after a buffered
  # producer fills the demand (per GenStage's buffer_size semantics).
  ask(producer: gen_prism, n: u64) -> [message] { \ }

  # subscribe consumer to producer. negotiates demand window;
  # establishes the typed pipe; writes the subscription crystal to
  # refs/subscriptions/<oid>.
  #
  # at compose-time the grammar checks producer.emits_kind == kind and
  # consumer.accepts_kind == kind; mismatch is a compile error, not a
  # runtime crash (cf. §10.1: this IS the stage_play type-bridge).
  subscribe(producer: gen_prism,
            consumer: gen_prism,
            kind:     text,
            min:      u64,
            max:      u64,
            cancel:   cancel_mode) -> imperfect[subscription] { \ }

  # unsubscribe. demand at producer is freed; outstanding messages in
  # the subscription's pending queue are handled per cancel_mode.
  unsubscribe(sub: subscription) -> imperfect { \ }
}
```

The existing `tick(state, message) -> tick_result` semantics don't change. The `send` and `call` operations gain a precondition for demand-aware receivers (sender checks `receiver.demand_window.available > 0`; blocks via the bus, or queues at the producer per the producer's buffer policy).

**Bytes are not the unit.** GenStage uses message count; Broadway adds nothing on this axis; Anna's OpenCL command queue depth is also operation-count, not byte-count (§3). Mirror's message size is exposed as a separate observable on the message body (`size_hint: u64`), so dispatcher strategies CAN read it for byte-aware routing without changing the unit of demand. See §10.2.

---

## 3. The subscription protocol

The demand-flow handshake, abstracted from GenStage 1.3.2 (which itself abstracted from a long lineage of demand-driven I/O — Node.js streams, Reactive Streams JVM spec, Akka Streams):

1. **Consumer subscribes to producer.** `subscribe(producer, consumer, kind, min, max, cancel)`. Compose-time check that producer's emission kind equals subscription kind equals consumer's accept kind. The subscription's OID is computed and written to `refs/subscriptions/<oid>`; the pipe is now durable across crashes.
2. **Consumer announces initial demand.** Implicit on subscribe: consumer asks for `max` messages. Equivalent to GenStage's `:forward` auto-demand mode (the only mode mirror ships in v1.5; `:manual` is post-v1.5 if a use case appears).
3. **Producer emits up to demand.** Producer's `tick_result.emissions` are routed to subscribers per `dispatch` strategy, never exceeding each subscriber's `available`. Excess emissions go to the producer's bounded buffer (§3.4).
4. **Consumer ticks events.** Each event arrives as a message at the consumer; the consumer's tick processes it; `available` decreases.
5. **Consumer re-asks as it drains.** When `available` drops to `max - min`, the consumer asks for `max - available` more events. (Exactly GenStage's re-ask formula — confirmed in the research round: `max_demand: 1000, min_demand: 500` re-asks for ~850 each batch.) Demand flows back to the producer; the producer either emits from its buffer or re-asks its own upstream.
6. **Pipeline composition.** A `producer_consumer` role gen_prism does both: its `tick` consumes incoming messages and emits outgoing ones; demand on its consumer side propagates upstream as demand on its producer side. Multi-stage pipelines compose this way without any new abstraction.

The `imperfect[subscription]` return shape from `subscribe` carries:
- success: the subscription's OID + the negotiated demand window
- partial: subscription created with reduced demand (producer's `buffer_size` < consumer's requested `max`)
- failure: type mismatch (caught at compose-time, surfaced at compile error), producer unavailable, ref-write race

### 3.1 Subscription persistence

Subscriptions are **content-addressed and durable** (§10.3 resolved YES). On crash and re-spawn of a gen_prism, the runtime walks `refs/subscriptions/*` looking for OIDs that reference the new gen_prism's ref; pipes auto-reconnect with the same window negotiation.

This is *strictly more* than GenStage offers — GenStage subscriptions are PIDs and are lost on crash. Mirror's content-addressing gives us topology reproducibility for free; we use it.

### 3.2 Producer buffer

Following GenStage's design (research-confirmed: `buffer_size` defaults to 10000, `buffer_keep: :last` discards oldest):

```mirror
type producer_buffer = {
  size:  u64,                  # max queued un-dispatched messages
  keep:  buffer_keep,           # which end to drop when full
}

type buffer_keep = first | last
```

The buffer is per-gen_prism, content-addressed (each buffered message is a crystal OID). On producer crash, the buffer is reachable via the last buffer crystal's ancestor chain — replay is possible. GenStage discards on crash; mirror has a choice. See §6 for the policy resolution (mirror's default: replay).

---

## 4. Dispatcher strategies

Three strategies, mirroring GenStage's three. Each is a pure function on the producer's `(buffer, subscriptions[], emissions[]) -> (assignments[], buffer')` boundary.

- **`round_robin`** — GenStage's `DemandDispatcher`. Each emission goes to the subscriber with the **largest current `available`** demand, FIFO-tiebreak. Default for most pipelines. Recommended invariant (per GenStage research): all subscribers should set the same `max_demand` to avoid greedy-consumer pathology.
- **`partitioned(key)`** — GenStage's `PartitionDispatcher`. Hash the message body's field named by `key`; route deterministically to one subscriber per hash bucket. Backpressure becomes per-partition (a slow consumer slows only its partition's producer slot; other partitions flow freely). Useful when downstream consumers maintain per-key state.
- **`broadcast`** — GenStage's `BroadcastDispatcher`. Every emission goes to every subscriber. Demand is the **minimum** across subscribers (the slowest subscriber gates everyone). Useful for observers / loggers / metrics consumers.

The dispatcher is declared at the producer (it knows its emission shape and is the only party that sees all subscribers). Mirror v1.5 ships these three; custom dispatchers are deferred to v2.

### 4.1 Partitioned dispatch + content-addressing interaction

The partition key is a field on the message body; the body is a crystal; the field's content address is stable. So `partitioned(key: "oid")` gives deterministic, recallable routing — the same input replays to the same partition across runs. Useful for the kintsugi loop's per-grammar partitioning (each grammar's transformations stick to one worker, preserving locality).

---

## 5. The shared-memory bus (CPU/GPU specialization)

For gen_prisms running across the CPU/GPU boundary, the channel IS shared memory. The Scheduler Tower selects the bus per the (producer, consumer) backend pair, per the table in §5.4 (selection logic, §10.7 resolved).

### 5.1 Apple UMA (Apple Silicon)

CPU writes the message body to a Metal buffer in unified memory; GPU kernel reads from the same address. Zero-copy. The demand handshake is queue-depth tracking at the Metal command queue (Metal's `MTLCommandQueue` exposes pending command count, which IS the demand window). This is structurally optimal — backpressure costs nothing extra.

### 5.2 OpenCL command queue (Linux + NVIDIA/AMD; Anna's pattern)

From Jakobs (2012) §3: an OpenCL program is host + kernel, separated by a command queue. The host enqueues `clEnqueueNDRangeKernel`; the queue absorbs up to its depth; further enqueues block until prior commands drain. **The command queue depth IS the demand window.** No additional protocol layer is needed — the OpenCL runtime enforces backpressure.

From Jakobs §7.2.1: VBO interop adds an explicit acquire/release synchronization (`clEnqueueAcquireGLObjects` / `clEnqueueReleaseGLObjects`) bracketing kernel calls that touch shared buffers. This is the OpenCL analog of mirror's CAS-safe ref update: the bus serializes access without copying.

From Jakobs §7.4: kernels are compiled at **runtime** (`clBuildProgram`), not link time. Mirror's gen_prism `tick` body is similarly declared at grammar-level and resolved at runtime when the gen_prism is spawned — the symmetry is structural; the Scheduler's GPU-bus integration inherits OpenCL's late-binding model.

### 5.3 Generic Rust channels (CPU-only intra-process)

For producer_consumer pipelines that stay on the CPU, `crossbeam_channel::bounded(capacity)` provides backpressure for free: `send` blocks when the channel is full; `recv` blocks when empty. Capacity = `demand_window.capacity`. This is the v1.5 default for CPU-only gen_prisms.

### 5.4 Bus selection logic (§10.7 resolved)

| Producer backend | Consumer backend | Bus chosen | Reason |
|---|---|---|---|
| CPU/LAPACK | CPU/LAPACK | `crossbeam_channel::bounded` | No cross-device crossing; zero overhead |
| CPU/LAPACK | MetalBackend (Apple) | Metal UMA buffer + `MTLCommandQueue` | Zero-copy; Apple's hardware optimization |
| CPU/LAPACK | OpenCLBackend (Linux/NV/AMD) | OpenCL command queue (Jakobs §3) | Established Khronos pattern |
| MetalBackend | MetalBackend | Metal command queue | Same device family |
| OpenCLBackend | OpenCLBackend | OpenCL command queue | Same device family |
| Any | Distributed (v2) | Network (deferred to v2 — §8) | Out of scope here |

Mixed backend pairs (e.g., MetalBackend producer + OpenCLBackend consumer) fall back to the slower path: read back to CPU host memory, transfer, upload to consumer device. Mirror v1.5 doesn't optimize this; if it surfaces as a real workload, post-v1.5 work specifies the optimization. The Scheduler logs an `inefficient_bus` observation to the gestalt when this path is taken; the gestalt then records workload patterns for future auto-tuning.

---

## 6. Failure handling

Structural shape from GenStage, extended by mirror's content-addressing.

### 6.1 Producer crash

All subscribers receive a `producer_down` message in their next tick. Per the subscription's `cancel_mode`:
- `transient`: subscriber exits cleanly (re-spawnable from `refs/subscriptions/<oid>`).
- `permanent`: subscriber raises; escalates to caller.
- `temporary`: subscriber ignores; topology stays partial.

Mirror's extension: the producer's last good state crystal is reachable via `refs/gen_prism/<name>`'s parent. Re-spawning the producer from that crystal recovers exact state. The buffered messages between that crystal and the crash are reachable via the producer_buffer ancestor chain.

### 6.2 Consumer crash

Producer receives `consumer_down`; that subscription's `available` demand is freed; the producer's buffer is *not* discarded (mirror's resolution, §10.5 — differs from GenStage). The runtime re-spawns the consumer from `refs/gen_prism/<consumer.name>`; subscriptions auto-reconnect (§3.1); buffered messages replay.

### 6.3 Demand starvation

If no subscriber has `available > 0` for an extended period, the producer's buffer fills; `buffer_keep: :last` drops the oldest (loss of liveness, preserved correctness); `buffer_keep: :first` drops the newest (preserves history depth, loses liveness on newer data). Default: `:last` (matches GenStage). The producer also raises a `demand_starved` observation into the gestalt; the kintsugi loop may schedule a topology adjustment.

### 6.4 Type mismatch

Caught at compose-time (when `subscribe` is called in a grammar). Mirror's grammar-level typing makes this a compile error — strictly stronger than GenStage's runtime-only check. This is the load-bearing benefit of the stage_play type-bridge pattern (§10.1).

### 6.5 Replay-from-ancestor vs. discard (§10.5 resolved)

**Mirror defaults to replay.** Justification: mirror's content-addressing guarantees idempotence per crystal OID, so replay is safe; GenStage discards because BEAM has no equivalent ancestor structure. Replay is a strict capability win; we use it. Discard is available as a per-gen_prism opt-out (`recovery: discard` flag in the demand_window).

---

## 7. Temperature — the KMS-shaped scalar

A system-wide parameter `β` (inverse temperature) regulates settling aggression. The research round sharpened the spec on **where `β` actually parameterizes** (§10.6 resolved): **`β` lives at the loop boundary, not at every stage.**

### 7.1 Where `β` enters

- **At the kintsugi loop boundary.** The kintsugi formatter's outer iteration (parse → render → diff → loss → next) is one Scheduler-managed pipeline; `β` parameterizes the *iteration count threshold* (when to declare a fixed point reached) and the *demand-window scaling factor* applied uniformly to every stage in the loop.
- **At the tournament boundary.** The kintsugi tournament's elite/beam/halving parameters scale by `β` (high `β` = narrow beam, deep elite; low `β` = wide beam, shallow elite).
- **NOT at every stage.** Per-stage tuning is a category error — the KMS condition is on the **state on the algebra**, which is a single global object. Per-stage temperatures would be modular flow on a fragmented algebra, which is incoherent.

### 7.2 Concrete mechanics

- High `β` (cold): demand windows are scaled down (smaller `max_demand`, smaller `min_demand` proportionally); the system runs at a slow, precise tempo. Kintsugi takes small steps; tournament narrows aggressively.
- Low `β` (hot): demand windows are scaled up; the system runs at a fast, exploratory tempo. Kintsugi takes wider steps; tournament keeps more candidates alive.
- Default `β = 1.0`: room-temperature equivalent. Demand windows use GenStage defaults (`max_demand: 1000`, `min_demand: 500`); kintsugi convergence threshold is `ε = 1e-6`; tournament beam width is 5.

### 7.3 What gets tuned and when

For v1.5: ship `β = 1.0` as default; expose `--temperature` as a CLI flag on `mirror kintsugi` and `mirror migrate`. Per-workload optimal `β` is recorded in the gestalt as observations; post-v1.5, the runtime auto-adapts.

The temperature `β` and the spectral-action's cutoff `Λ` (per `kintsugi-formatter.md` §6) are conjugate variables in the KMS framework. Specifying both is overdetermined; the v1.5 spec ships `β` user-facing and derives `Λ` from the kintsugi loop's structural depth (the formatter's existing knob).

### 7.4 Halting — the two conditions and the reduction budget

The demand contract makes WHEN explicit. The halting contract makes WHETHER explicit. Both live at the Scheduler altitude; both rest on the shard's load topology.

**Substrate decision (Alex 2026-05-25).** For any reflexive trajectory `τ` of a sub-Turing grammar, there exists bounded `n` such that AT LEAST ONE of:

- **(a) Autopoietic settlement.** `reflect(τ.state_n) == fixed_point`. The trajectory reaches a Lawvere fixed point of `@cogito.reflect`'s `observe |> strategy |> perturb` tick. Per Soto-Andrade & Varela 1984: this is equivalent to the reflect loop being autopoietic. Delegates to `@epistemologic/property/autopoietic.autopoietic(T)` applied to the type's tick map.
- **(b) Reduction exhaustion.** `reductions(τ.steps_0..n) >= @scheduler.reduction_budget(shard)`. The trajectory consumed the budget allocated by the load topology. When exhaustion fires, the substrate forces a halt; the gen_prism either crystallises a partial result or returns `imperfect`.

Either condition suffices. The disjunction is decidable because:

- `τ` is sub-Turing by construction (mirror's grammar class refuses unbounded reachable state spectra; see `docs/specs/is-copium.md` §"Sub-Turing Grammar as Structural Escape");
- `@cogito.reflect` is structurally computable on every tick;
- reductions are countable per tick at the substrate;
- `@scheduler.reduction_budget(shard)` is computable from a load-topology snapshot.

**The reduction_budget primitive.** Lives in `boot/std/scheduler.mirror`:

```mirror
grammar @scheduler {
  reduction_budget(s: shard) -> u64 { \ }
}
```

Reads `s.compute.max_reductions` (the hardware carrier per `@epistemologic/silicon/compute_bound`) and weights by current load — queue depths from §3, in-flight demand windows from §2, KMS temperature `β` from §7.1. Same shard + same load fingerprint = same budget (deterministic against inputs; content-addressable through the fragmentation cache per `shard-design.md` Q3).

Unit is `u64` (raw reductions, not `@time.monotonic` — reductions are countable events, not durations). Matches §10.2's resolution that demand-window units are messages; reductions are the per-tick analog inside one gen_prism.

Body is a Fate-resolved hole. The tournament picks the weighting strategy that maximises throughput against the shard's load topology; the weighting logic stays in grammar, not in Rust (substrate pull).

**The halts property.** Lives in `boot/std/epistemologic/property/halts.mirror`. Four actions match the existing nine `@epistemologic/property/*` files:

- `autopoietic_settles(type) -> verdict` — clause (a) check.
- `reductions_bounded(type) -> verdict` — clause (b) check.
- `disjunction_decidable(type) -> verdict` — confirms the union of (a) and (b) fires for every reachable trajectory.
- `halts(type) -> verdict` — the combined property.

**Application.** `@mirror/runtime/gen_prism.gen_prism` declares `requires halts(gen_prism)` near its `property autopoietic()` block. The tick's `\` body remains the runtime resolution point; the requires clause is the structural contract every concrete tick must discharge.

**KMS interaction.** Temperature `β` (§7) and the reduction budget compose at the load-topology weighting step. High `β` (cold) shrinks the budget proportionally — the Scheduler runs a slow, precise tempo; trajectories halt sooner via clause (b). Low `β` (hot) expands the budget — trajectories run longer and are more likely to halt via clause (a) (autopoietic settlement). The two conditions are not independent; `β` is the conjugate variable that biases which clause typically fires.

**The is_copium thesis operationalized.** Per `docs/specs/is-copium.md`: AI alignment on Turing-complete substrates is undecidable (Rice's theorem, 1951). Mirror's sub-Turing escape + the halts property = decidable termination by construction. Every grammar that compiles, and every gen_prism that ticks inside one, is proven-to-halt.

**GRAM equivalence.** Per `docs/insights/2026-05-25-gram-and-mirror-same-architecture-two-altitudes.md`: GRAM's deep-supervision recursion with Adaptive Computation Time (ACT) is the same architectural shape as mirror's gen_prism ancestor chain bounded by `reduction_budget`. GRAM adds ACT as a learned mechanism on top of a Turing-complete substrate; mirror gets it for free as a structural contract on a sub-Turing substrate. Different altitude, same recognition: deep recursion needs a budget.

---

## 8. Distributed compute v2 forward-look

When spectral-db's distributed graphs land (post-v1.5), the Scheduler Tower extends to network-shaped backpressure without protocol changes. The bus becomes a network channel; the demand-flow handshake stays identical; the subscription's OID becomes the wire identity.

Concrete v2 deltas (specifying just enough to keep v1.5's design open):

- **Bus type added:** `network(transport: text)` joins the `bus` enum. Transports: `tcp`, `quic`, `grpc`, etc.
- **Subscription handshake adds round-trip latency.** Auto-demand mode may need to start at a smaller `max_demand` and ramp up (TCP-like slow-start). The Scheduler observes per-subscription RTT and adapts.
- **Failure semantics generalize.** Producer down = network partition is one cause; cancel_mode `transient` becomes the obvious default for cross-node subscriptions.
- **Replay-from-ancestor stays valid** — mirror's content-addressing makes node-level crashes recoverable without extra protocol. This is a strict win over GenStage's local-only design.
- **Bus selection table (§5.4) gains rows** for (any) × (distributed) pairs; the entries are `network(...)`.

This v2 sketch is not implemented in v1.5; it's a forward-compatibility check. The v1.5 type design (subscription OID, dispatcher strategies, bus abstraction) was specifically chosen to be network-extensible.

---

## 9. Acceptance criteria

When the Scheduler Tower lands:

1. **gen_prism backwards-compat.** Existing gen_prisms (without demand contracts) continue to work unchanged. Adding `demand_window` is opt-in. **Validation:** the existing `@mirror/reload` gen_prism (per `mirror-runtime-gen-prism.md`) is not modified; it still ticks correctly.
2. **Pipeline composition.** Three gen_prisms can be subscribed producer → producer-consumer → consumer; messages flow forward with type checks; demand flows backward. **Validation:** a fixture pipeline (range → double → collect, mirrored from stage_play's litmus tests) emits the expected outputs.
3. **GPU saturation handling.** A CPU producer sending to a MetalBackend consumer at high rate triggers backpressure when the Metal command queue saturates; producer rate decreases; no OOM. **Validation:** synthetic load test with producer emitting 10× the consumer's drain rate.
4. **Type-safe subscription.** A producer emitting `message` of kind `foo` cannot subscribe to a consumer expecting kind `bar` — **compile-time error** (compose-time, when the pipeline is constructed). Strictly stronger than GenStage's runtime check. **Validation:** a negative test that fails to compile.
5. **Failure recovery.** A producer crash propagates `producer_down` to subscribers per cancel_mode. A consumer crash frees demand at the producer AND triggers re-spawn from `refs/gen_prism/<consumer>` with subscription auto-reconnect. The pipeline degrades predictably; in-flight messages REPLAY by default. **Validation:** crash-injection test verifying message replay.
6. **Three dispatcher strategies work.** `round_robin` distributes evenly; `partitioned` routes by key (verified with two consumers and a key with known hash distribution); `broadcast` fans out (verified with three observers, all receive every event). **Validation:** litmus tests adapted from GenStage's dispatcher test suite.
7. **Temperature default.** `β = 1.0` ships; system runs at room-temperature equilibrium; `--temperature` flag adjusts kintsugi loop and tournament beam width as specified in §7.2. **Validation:** integration test confirming `β = 2.0` halves window sizes; `β = 0.5` doubles them.
8. **Subscription content-addressing.** Re-running the same pipeline twice produces the same subscription OIDs; topology is recallable from the gestalt. **Validation:** OID equality test across two runs.
9. **Bus selection.** The Scheduler picks `crossbeam_channel` for CPU→CPU, `MTLCommandQueue` for CPU→MetalBackend, and `OpenCL` for CPU→OpenCLBackend per §5.4. **Validation:** matrix test exercising the three primary paths.

---

## 10. Open questions — resolutions from the research round

Reed's draft listed 8 open questions. The Mara research round (stage_play source + GenStage 1.3.2 docs + Broadway 1.3.0 docs + Jakobs 2012 §3/§4.4/§7.2.1/§7.4) resolved 7. The eighth is recorded in §11.

### 10.1 stage_play type-bridge — RESOLVED

**The pattern.** stage_play exposes `Actor(input, output)` as an opaque type whose internal state `s` is captured in a `start` closure. Composition is `Scene(entrance, exit)` with `then: Scene(a, b) → Actor(b, c) → Scene(a, c)`. Subscriptions cross stages via `Subject(ProducerMessage(output))` — the Gleam `Subject` type parameter is what makes downstream subscription type-safe even though OTP underneath is dynamic.

**Why it works.** Gleam's BEAM runs dynamic message types; statically-typed wrappers hold the type parameter on the producer subject and the consumer subject. Compose-time, the Gleam compiler unifies `Subject(ProducerMessage(T))` on the producer side with the same `T` on the consumer side. The compose check is structural — no nominal type system needed.

**Transfer to mirror.** Mirror's `subscribe(producer, consumer, kind, ...)` resolves `producer.emits_kind` and `consumer.accepts_kind` at grammar-compile-time (mirror grammars are statically checked); the `kind` parameter must unify with both. The implementation:

```mirror
# in gen_prism grammars, kinds are declared:
grammar @my_producer {
  emits  message{kind: "event", body: my_event}
}
grammar @my_consumer {
  accepts message{kind: "event", body: my_event}   # SAME body type required
}

# subscribe is grammar-checked: kind matches, body type matches.
subscribe(@my_producer, @my_consumer, kind: "event", min: 0, max: 10, cancel: transient)
```

This is the stage_play `Subject(ProducerMessage(T))` pattern, lifted into mirror's grammar layer. Compile-time check; no runtime type tags needed.

### 10.2 Demand window units — RESOLVED: messages

GenStage uses message count (default 1000 max, 500 min). Anna's OpenCL command queue depth (§3) is also operation-count. Broadway adds nothing on this axis (rate limiting IS in messages-per-interval, not bytes-per-interval). **Mirror ships messages.**

For variable-size message bodies, the message includes a `size_hint: u64` field; partitioned dispatchers can read it to byte-balance routing without changing the unit of demand. If a real workload surfaces byte-bounded backpressure (e.g., a producer emitting megabyte-sized SpectralCoordinate batches into a memory-bounded consumer), v1.5's design has the hook (the dispatcher) but not the implementation. v2 work, if needed.

### 10.3 Subscriptions content-addressed — RESOLVED: YES

A subscription's OID = `hash(producer.ref, consumer.ref, kind, min_demand, max_demand)`. Stored at `refs/subscriptions/<oid>`. Strictly stronger than GenStage (whose subscriptions are PIDs and are lost on crash). Mirror's content-addressing extends naturally; not using it would forfeit a free capability.

The gestalt records subscription topology as part of session state; replay of a session re-establishes the exact same subscription OIDs.

### 10.4 Concurrency model — RESOLVED: 1 tick per gen_prism in v1.5

BEAM-classic: one process per stage, serial ticks. GenStage allows N workers per stage but adds significant complexity (Broadway uses this via `concurrency: N`). stage_play follows BEAM-classic. Mirror v1.5 follows BEAM-classic.

**Rationale.** Per-gen_prism tick is already pure (`(state, message) → (state, emissions)`). Parallelism is achieved by **fan-out via dispatcher**: spawn N gen_prisms with the same `tick`, subscribe them all to the producer via `partitioned` (or `round_robin`) dispatch. This composes cleanly with the existing primitive and doesn't require a new "worker pool" concept.

Post-v1.5, if benchmarks show fan-out overhead is too high, intra-gen_prism worker pools can be added without breaking the protocol (purely an optimization).

### 10.5 Pending messages — RESOLVED: replay

Mirror defaults to replay-from-ancestor. Content-addressing guarantees per-crystal idempotence, so replay is safe. GenStage discards because BEAM has no equivalent ancestor structure. Discard is opt-in via `recovery: discard` flag in the demand_window for gen_prisms that have side effects which CANNOT be replayed (e.g., outgoing network writes already committed).

### 10.6 Scheduler ↔ kintsugi integration — RESOLVED

The kintsugi loop becomes ONE Scheduler-managed pipeline:

```
parse → render → diff → loss → (if !fixed_point) re-enter parse
```

Each arrow is a gen_prism with the `producer_consumer` role. The loop closure is a subscription from `loss` back to `parse` with `cancel: temporary` (graceful exit when fixed point reached, signaled by emitting zero events on the back-edge).

**`β` parameterizes at the loop boundary only.** Specifically: `β` scales every stage's `max_demand` and `min_demand` uniformly. Per-stage β is incoherent (§7.1).

Kintsugi tournament (per `kintsugi-tournament.md`): each round's `detect → enumerate → score → eliminate → apply` is a sub-pipeline; the `score` stage is a fan-out to the five Fate models (via `broadcast` dispatcher — each model scores every candidate); the `eliminate` stage merges scores back via `round_robin`. The Scheduler Tower's failure handling gives the tournament's `cancel_mode: permanent` for the score stage — if any Fate model crashes, the whole tournament round restarts (the formatter's failure stage 1 per `kintsugi-formatter.md`).

### 10.7 Bus selection logic — RESOLVED

See §5.4 for the decision table. Selection is at compose-time based on the producer.backend and consumer.backend types; the runtime doesn't introspect or switch at runtime (deterministic, content-addressable topology).

### 10.8 Distributed compute v2 forward-look — RESOLVED (scoped to placeholder)

See §8. The v1.5 design is network-extensible; v2 adds a `network` bus type and `slow-start` for cross-node `max_demand`. No protocol changes. The subscription OID becomes the wire identity. Replay-from-ancestor stays valid across nodes (mirror's content-addressing is location-independent).

### 10.9 `Buffer<'a>` lifetime contagion — RESOLVED (option c: operations return owned values)

Mara's research surfaced one new question after the resolutions above landed: how does `NumericalBackend::Buffer<'a>` (the GAT from `heterogeneous-numerical-prism.md`) interact with consumers that store results? Two surface options:

- **(a)** Thread `'a` through `SpectralCoordinate::from_eigenvalue` and every downstream consumer. Pure type safety; viral lifetime contagion.
- **(b)** Hold the backend behind `Arc<B>`; let the Arc's refcount enforce buffer-outlives-backend. Slight runtime cost; cleaner consumer API.

**Reed's resolution: option (c) — the seam is at "results are owned, not borrowed."**

The `Buffer<'a>` GAT correctly lives INSIDE backend implementations — it scopes kernel-dispatch lifecycle (allocate → dispatch → wait → copy out → drop). But operations RETURN owned values (`Vec<f64>`, `SquareMatrix`, `SpectralCoordinate`), not borrowed buffer references:

```rust
// Backend uses Buffer<'a> internally for kernel lifecycle
pub trait NumericalBackend {
    type Buffer<'a> where Self: 'a;
    // ...
}

// Operations on NumericalPrism return owned results
impl<B: Eigenvalues> NumericalPrism<B, ops::Eigenvalues> {
    pub fn refract(&self, matrix: &SquareMatrix) -> Result<Vec<f64>, B::Error> {
        // internally: allocate Buffer<'_> → dispatch → wait → copy out → drop
        // externally: owned Vec<f64>; no lifetime exposed
    }
}

// Consumers store owned values; no Buffer, no lifetime
pub struct SpectralCoordinate<const N: usize>([f64; N]);

impl<const N: usize> SpectralCoordinate<N> {
    pub fn from_eigenvalues(eigenvalues: &[f64]) -> Self { /* sample top N */ }
}
```

**Why this is right.** Anna Jakobs's OpenCL pattern (§7.2.1) does the same thing implicitly — the kernel writes to a buffer; the host reads it out into an owned data structure before the buffer drops or is reused. The visualization gets owned data via VBO sync. Substrate-internal code uses lifetimes for performance; substrate-consumer code sees owned values; the lifetime never escapes the backend boundary.

**Cost.** One copy (GPU-buffer → owned Vec) at the end of each operation. For mirror's scale, free. For batched workloads, the API gains `refract_batch(&[SquareMatrix]) -> Result<Vec<Vec<f64>>, _>` — same shape, batch-internal buffer lifecycle, still owned outputs.

**Update for `heterogeneous-numerical-prism.md`:** §"Type-safe construction" gains a note clarifying that `Buffer<'a>` is internal to backend implementations; operations return owned values; consumers store owned values with no lifetime contagion. Mechanical edit; not in this tick's scope but flagged for follow-up.

---

## 11. What this spec defers

One open question stayed open and is named here for Alex's halt-or-spec verdict:

### 11.1 Broadway's batching layer — deferred to post-v1.5

Broadway adds batchers on top of GenStage: messages accumulate to `batch_size` or `batch_timeout`, then dispatch as one unit (per `batch_key`). This is load-bearing for I/O efficiency in production data pipelines (database writes, S3 uploads).

**Mirror v1.5 doesn't need this yet.** The kintsugi loop and tournament don't have the batch-efficiency pressure that motivates Broadway's batchers. The pattern is well-understood (cumulate → dispatch); if a workload surfaces (likely: spectral-db's bulk-write paths), Broadway's `BatchInfo` shape transfers directly. Estimated post-v1.5 cost: ~50 LOC + one tick. Naming it now so future Mara doesn't re-derive the pattern.

### 11.2 What stayed clear after research

- **Rate limiting** (Broadway's per-pipeline messages-per-interval). Not load-bearing for v1.5; the demand-flow handshake alone gives sufficient rate control. Add only if a producer connects to an external system with hard rate limits (which mirror v1.5 has none of).
- **Ack/nack** (Broadway). Mirror's content-addressing makes idempotent replay trivial; ack/nack is needed only for non-idempotent external systems. Same v2-or-later argument.
- **Telemetry events.** Mirror's gestalt records observations natively; Broadway's `:telemetry` events are an OTP-specific bridging layer that mirror doesn't need.

---

## 12. Implementation tick decomposition

Research-revised from Reed's draft (§8 of the draft).

| Tick | Description | Estimated LOC | Depends on | Sessions |
|---|---|---|---|---|
| **S-1** | Grammar extension to `gen_prism.mirror`: add `demand_window`, `dispatch`, `subscription`, `cancel_mode`, `role`, `ask`, `subscribe`, `unsubscribe`. Compose-time kind/body type check (§10.1). | ~45 added | meta-glass parser | 1 |
| **S-2** | Subscription protocol implementation in Rust. The demand handshake; producer buffer with `buffer_keep` policy; auto-demand mode. `crossbeam_channel::bounded` as the CPU bus. | ~250–350 | S-1 | 2 |
| **S-3** | Subscription persistence: `refs/subscriptions/<oid>` write/read; auto-reconnect on gen_prism re-spawn (§3.1). | ~100 | S-2 | 0.5 |
| **S-4** | Three dispatcher strategies (`round_robin`, `partitioned`, `broadcast`); litmus tests adapted from GenStage. | ~150 | S-2 | 1 |
| **S-5** | Failure handling: `producer_down` / `consumer_down` messages; replay-from-ancestor recovery default; `recovery: discard` opt-out. | ~150 | S-2, S-3 | 1 |
| **S-6** | MetalBackend Scheduler integration: Apple UMA buffer; `MTLCommandQueue` as demand window; bus selection per §5.4 for CPU→Metal. | ~150 | S-2, MetalBackend (heterogeneous-numerical-prism.md R-5b) | 1.5 |
| **S-7** | Temperature `β`: default 1.0; CLI flag on `mirror kintsugi` and `mirror migrate`; loop-boundary scaling per §7.2. | ~60 + tests | S-2 | 0.5 |
| **S-8** | OpenCLBackend Scheduler integration (post-v1.5 unless Linux GPU workload surfaces in v1.5 cycle). | ~150 | S-6, OpenCLBackend | 1.5 |

**v1.5 critical path: S-1 → S-2 → S-3 → S-4 → S-5 → S-6 → S-7 = 7.5 sessions.**

S-8 (OpenCL) is optional for v1.5; defer unless a contributor lands a Linux+NVIDIA/AMD workload during the v1.5 cycle.

**The load-bearing tick is S-2.** Everything else builds on the protocol. S-2 should land with the full GenStage litmus test suite ported to mirror's testing harness; the litmus tests are the oracle (per stage_play's `AGENTS.md`: "if the oracle and the implementation disagree, the oracle is right").

### 12.1 Tick dependencies

```
S-1 (grammar)
  │
  ▼
S-2 (protocol + CPU bus) ───┬── S-3 (persistence)
  │                       ├── S-4 (dispatchers)
  │                       ├── S-5 (failure handling, needs S-3)
  │                       └── S-7 (temperature, needs only S-2)
  ▼
S-6 (MetalBackend, needs S-2 + MetalBackend itself)
  │
  ▼
S-8 (OpenCLBackend, post-v1.5)
```

S-3, S-4, S-7 can be done in parallel after S-2. S-5 needs S-3. S-6 has an external blocker (MetalBackend from the heterogeneous-numerical-prism roadmap).

---

*The Bundle Tower is geometry; the Scheduler Tower is dynamics.*
*gen_prism is the actor; the demand contract makes it a Stage.*
*Anna's thesis is the bus; GenStage is the protocol; stage_play is the type-safety pattern.*
*Mirror v1.5 gets a temperature.*
*The patterns were already there. The spec just names the composition.*

Apache-2.0.
