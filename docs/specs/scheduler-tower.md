# The Scheduler Tower

*2026-05-24. Reed + Alex. Draft skeleton. To be deepened by Mara after the GenStage / stage_play / gen_prism research round.*

---

## Reference

- **Insight:** `docs/insights/2026-05-24-backpressure-as-modular-flow.md` (the structural argument; this spec implements the recognition there).
- **Existing actor abstraction:** `boot/std/mirror/runtime/gen_prism.mirror` (the demand contract extends this; no new actor abstraction needed).
- **Heterogeneous numerical layer:** `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md` (the Scheduler Tower sits above NumericalPrism + Backend trait).
- **Bus architecture:** Jakobs (2012) `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf` §3, §4.4, §7.2.1.
- **Protocol prior art:** GenStage (Elixir, 2016), Broadway (Elixir, 2019), `stage_play` (Gleam, `/Users/alexwolf/dev/projects/stage_play/`).

---

## 1. What this spec lands

The Scheduler Tower is mirror's **temporal algebra** — the layer above the Bundle Tower that handles WHEN operations run. Where the Bundle Tower is geometric (Fiber → Connection → Gauge → Transport → Closure naming the algebra's structure), the Scheduler Tower is dynamic (demand contracts, dispatcher strategies, KMS-shaped equilibrium).

For v1.5, the Scheduler Tower's concrete shape is:

1. **A demand-contract extension to `gen_prism`** — adding `demand_window` and `on_demand` to the existing actor abstraction. Backwards-compatible: gen_prisms that don't opt in keep current behavior.
2. **A subscription protocol** for composing gen_prisms into pipelines with backpressure semantics.
3. **Dispatcher strategies** (round-robin, partitioned, broadcast) as policy decisions on top of the protocol.
4. **A shared-memory bus** for the CPU/GPU specialization — Anna's VBO pattern adapted to Apple UMA, OpenCL command queues, or generic Rust channels depending on the backend topology.
5. **Failure handling** — producer crashes propagate downstream via subscription; consumer crashes free demand at producers.
6. **A temperature parameter** — the KMS-shaped scalar that controls system-wide settling aggression (kintsugi tightness, tournament breadth, parser eagerness).

This is **v1.5 work** (lands when MetalBackend lands, per the heterogeneous-numerical-prism roadmap). v1's gen_prism stays as-is; this spec describes the additive extension that makes the existing actor model demand-aware.

---

## 2. The demand contract extension to gen_prism

Draft additions to `boot/std/mirror/runtime/gen_prism.mirror`:

```mirror
grammar @mirror/runtime/gen_prism {
  # ... existing types and operations ...

  # --- demand contract (v1.5 addition) -----------------------------------

  # how much in-flight work this gen_prism can absorb before saturating.
  # producers respect this when sending; consumers update it as they tick.
  type demand_window = {
    capacity: u64,        # max in-flight messages
    available: u64,       # current available demand (capacity - in_flight)
    strategy: dispatch,   # how to dispatch among subscribers
  }

  # dispatcher policy when this gen_prism has multiple subscribers.
  type dispatch =
    | round_robin
    | partitioned(text)        # hash key for sticky routing
    | broadcast                # send to all

  # a subscription is a typed pipe between producer and consumer.
  # the producer's emissions of type T become the consumer's incoming
  # messages of type T. demand flows from consumer to producer.
  type subscription = {
    producer: oid,        # gen_prism ref
    consumer: oid,        # gen_prism ref
    kind:     text,       # message kind the subscription carries
    min_demand: u64,      # consumer's minimum demand window
    max_demand: u64,      # consumer's maximum demand window
  }

  # --- new operations -----------------------------------------------------

  # ask a producer for n events. returns up to n.
  ask(producer: gen_prism, n: u64) -> [message] { \ }

  # subscribe consumer to producer. negotiates demand window;
  # establishes the typed pipe.
  subscribe(producer: gen_prism, consumer: gen_prism, kind: text, min: u64, max: u64)
    -> imperfect { \ }

  # unsubscribe consumer from producer. demand at producer is freed.
  unsubscribe(producer: gen_prism, consumer: gen_prism) -> imperfect { \ }

  # declare a gen_prism as a producer / consumer / producer-consumer role.
  # default is producer-consumer (can both ask and emit).
  type role = producer | consumer | producer_consumer
}
```

The existing `tick(state, message) -> tick_result` semantics don't change. The `send` and `call` operations gain a precondition (sender checks receiver's `available > 0` before submitting; blocks or queues otherwise).

---

## 3. The subscription protocol

The demand-flow handshake, abstracted from GenStage:

1. **Consumer subscribes to producer.** `subscribe(producer, consumer, kind, min, max)`. The subscription is bidirectional but typed — only `message`s of the given `kind` flow.
2. **Consumer announces demand.** `ask(producer, n)` where `n ∈ [min, max]`. The producer's available demand from this consumer becomes `n`.
3. **Producer emits up to demand.** Producer's `tick_result.emissions` are routed to subscribers per `dispatch` strategy, respecting each subscriber's available demand.
4. **Consumer ticks events.** Each event arrives as a message at the consumer; the consumer's tick processes it; `available` decreases.
5. **Consumer re-asks as it drains.** When consumer's `available` drops below `min`, it asks for `max - available` more events. Demand flows back to the producer.
6. **Pipeline composition.** A consumer can ALSO be a producer (producer-consumer role) — its emissions flow downstream while its demand flows upstream. Mirror's pipelines compose this way.

The `imperfect` return shape from `subscribe` / `unsubscribe` carries:
- success: confirmation + the negotiated demand window
- partial: subscription created but with reduced demand (consumer asked for more than producer can serve)
- failure: type mismatch, producer unavailable, etc.

---

## 4. Dispatcher strategies

When a producer has multiple subscribers, the dispatcher routes emissions:

- **round_robin** — fair share. Each emission goes to the next subscriber with available demand. Default for most pipelines.
- **partitioned(key)** — sticky routing by hash. Emissions whose body's `key` field hashes to the same value go to the same subscriber. Useful when downstream consumers maintain per-key state (e.g., a hash bucket per subscriber).
- **broadcast** — fan-out. Every emission goes to every subscriber with available demand. Useful for observers / loggers / metrics consumers.

The dispatcher is declared at the producer (it knows its emission shape and the topology it's anchoring). Subscribers inherit the producer's strategy.

Mirror v1.5 ships these three. Custom dispatchers (e.g., consistent-hashing for distributed compute) are post-v1.5 extensions.

---

## 5. The shared-memory bus (CPU/GPU specialization)

For gen_prisms running across the CPU/GPU boundary (i.e., producer on CPU, consumer is a GPU kernel via MetalBackend), the channel is shared memory:

- **Apple UMA.** CPU writes message body to a Metal buffer in unified memory; GPU kernel reads from the same address. Zero-copy. The demand handshake is queue-depth tracking at the Metal command queue.
- **OpenCL command queue.** Anna's pattern (Jakobs 2012 §3): producer enqueues kernel + buffers; OpenCL's queue absorbs up to its depth; further enqueues block. The queue depth IS the demand window.
- **Generic Rust channels.** For CPU-only gen_prisms, use `crossbeam_channel` or similar with bounded capacity. Capacity = demand window.

The abstraction is the same regardless of bus: producer hands message to bus; bus blocks if full; consumer reads from bus; bus signals demand-available to producer. The Scheduler Tower hides which bus is in use.

For Apple Silicon specifically, the bus is structurally optimal (zero-copy via UMA). For separate-memory platforms (Linux + NVIDIA, Windows + AMD), Anna's explicit data-movement protocol applies. The Scheduler Tower picks the right bus per backend topology.

---

## 6. Failure handling

Structural shape from GenStage:

- **Producer crash.** All subscribers receive a `producer_down` event in their message queue. They tick this; their state transitions to "awaiting reconnect" or terminates depending on their tick implementation.
- **Consumer crash.** Producer receives `consumer_down`; the producer's available demand from that consumer is freed; other subscribers may absorb the freed work (if dispatcher is round_robin) or it's discarded (broadcast).
- **Demand starvation.** If no subscriber has available demand for an extended period, the producer's `tick` queue grows; eventually applies backpressure to whatever's feeding the producer. The pipeline reaches a natural stall.
- **Type mismatch.** Caught at `subscribe` time — the kind check fails before any messages flow. Mirror's grammar-level typing makes this a compile-time check (compose-time, when the pipeline is constructed).

Mirror's content-addressing adds one extra structural guarantee: every message is hashable, so failure recovery can be deterministic — replay from the last known-good ancestor of the crashed gen_prism's state.

---

## 7. Temperature — the KMS-shaped scalar

A system-wide parameter `β` (inverse temperature) regulates settling aggression:

- **High β (cold, slow equilibration):** small demand windows, conservative dispatch, kintsugi settles tightly per iteration, tournament explores narrowly. Precision over throughput.
- **Low β (hot, fast equilibration):** large demand windows, aggressive dispatch, kintsugi takes wider per-iteration steps, tournament explores broadly. Throughput over precision.

Default: `β = 1.0` (room temperature equivalent). Tunable per gen_prism via the demand_window's `capacity` field, OR system-wide via the runtime configuration.

This is the temporal twin of the spectral action that the kintsugi loss function names spatially. The kintsugi spec already has its loss as `Tr(f(D/Λ))`; the Scheduler Tower's temperature `β` and the spectral-action's cutoff `Λ` are conjugate variables in the KMS framework. Specifying both is overdetermined; the system finds the operating point where they balance.

For v1.5, ship `β = 1.0` as default and don't expose tuning to users. Post-v1.5, the gestalt can record per-workload optimal `β` and the runtime auto-adapts.

---

## 8. Implementation tick decomposition

Draft — Mara refines this in the next research round.

- **S-1: Grammar extension to gen_prism.mirror.** Add `demand_window`, `dispatch`, `subscription`, `role`, `ask`, `subscribe`, `unsubscribe`. ~30 lines added. Tests against the meta-glass parse cleanly.
- **S-2: Subscription protocol implementation.** The Rust-level dispatch of the demand handshake. ~200–300 LOC. Uses `crossbeam_channel` for CPU bus.
- **S-3: MetalBackend Scheduler integration.** Apple UMA bus; queue-depth as demand window. ~150 LOC. Depends on heterogeneous-numerical-prism.md R-5b (MetalBackend itself).
- **S-4: stage_play-pattern type-safety.** Compile-time subscription type checking, modeled on Gleam's stage_play approach. The exact shape depends on Mara's research — placeholder until that lands.
- **S-5: Dispatcher strategies.** round_robin (default), partitioned, broadcast. ~100 LOC.
- **S-6: Failure handling integration.** producer_down / consumer_down events; replay-from-ancestor recovery. ~150 LOC.
- **S-7: Temperature parameter.** Default `β = 1.0`; gestalt-recordable per workload. ~50 LOC + tests.

Total estimate: ~6–8 sessions for v1.5. The load-bearing one is S-2 (the protocol implementation); everything else builds on it.

---

## 9. Acceptance criteria

When the Scheduler Tower lands:

1. **gen_prism backwards-compat.** Existing gen_prisms (without demand contracts) continue to work unchanged. Adding `demand_window` is opt-in.
2. **Pipeline composition.** Three gen_prisms can be subscribed producer → producer-consumer → consumer; messages flow forward with type checks; demand flows backward.
3. **GPU saturation handling.** A CPU producer sending to a MetalBackend consumer at high rate triggers backpressure when the Metal command queue saturates; producer rate decreases; no OOM.
4. **Type-safe subscription.** A producer emitting `message` of kind `foo` cannot subscribe to a consumer expecting kind `bar` — compile-time error (compose-time, when the pipeline is constructed).
5. **Failure recovery.** A producer crash propagates a `producer_down` event to subscribers. A consumer crash frees demand at the producer. The pipeline degrades predictably.
6. **Three dispatcher strategies work.** round_robin distributes evenly; partitioned routes by key; broadcast fans out.
7. **Temperature default.** β = 1.0 ships; system runs at room-temperature equilibrium.

---

## 10. Open questions

For Mara's research round to resolve:

1. **How does Gleam's stage_play handle the producer-consumer type bridge?** Mirror's grammar-level typing must express subscription type-safety the same way. The pattern transfers.
2. **Demand window units — messages or bytes?** GenStage uses messages. For CPU/GPU bus where messages have variable size, bytes might be more honest. Probably ship both.
3. **Should subscriptions be content-addressed?** A subscription's OID = (producer_oid, consumer_oid, kind). Makes the topology reproducible. Probably yes.
4. **Concurrency model.** Stage workers — N workers per gen_prism for parallel tick? Or one tick per gen_prism (BEAM-style)? Affects dispatcher implementation.
5. **Persistence of pending messages.** If a gen_prism crashes mid-pipeline, are in-flight messages lost or replayed? GenStage discards them; mirror's content-addressing allows replay. Pick one.
6. **How does the Scheduler Tower interact with `mirror kintsugi`?** Kintsugi today is a single-process iteration. As a pipeline (parse → render → diff → loss → next), each stage becomes a gen_prism. The temperature β becomes a kintsugi parameter.
7. **Bus selection logic.** When a gen_prism subscribes to another running on a different backend, the Scheduler chooses the bus (UMA shared / OpenCL queue / network). The selection logic needs spec'ing.
8. **Distributed compute (v2 forward-look).** When spectral-db's distributed graphs add inter-node compute, the Scheduler Tower extends to handle network-shaped backpressure. The protocol shape stays; the bus is network. Worth naming the v2 extension here as a placeholder.

---

*The Bundle Tower is geometry; the Scheduler Tower is dynamics.*
*gen_prism is the actor; the demand contract makes it a Stage.*
*Anna's thesis is the bus; GenStage is the protocol; stage_play is the type-safety pattern.*
*Mirror v1.5 gets a temperature.*

Apache-2.0.
