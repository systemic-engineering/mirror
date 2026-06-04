# Runtime Elevation — from content-addressed compiler to BEAM-like runtime

*Architectural recognition, 2026-06-01. Reed + Alex. Status: pending; sequenced after Phase 1 gate.*

---

## What this names

Mirror today is a content-addressed compiler. `.mirror` → AST → Splinter → disk. Per-grammar verification. The 9-keyword floor. The Crystallizations dispatcher with empty floor and `kintsugi_tick` wired through `@kintsugi/tick`.

Mirror with `@spectral/db` installed becomes a **BEAM-like runtime**. The piece that elevates it is **SpectralSupervisor** — the closed-source coordination engine that observes the spectral signature of the cross-shard computation and pushes context where the structure says it should flow.

The elevation is one architectural step, decomposing into substrate-level moves at three altitudes:

| Altitude | Layer | License | Lives in |
|---|---|---|---|
| **System** | **SpectralSupervisor** — coordinator without authority. Reads spectral signature; pushes cross-domain context (RAM-level, Jakobs-bus pattern); restarts shards from last-crystal ancestor; reweights budgets under load. The Fate-driven strategy selector. The smart optimizations. | **closed** | `@spectral/db` |
| **Shard** | **HamiltonScheduler** — the agent's content window manager. Named for **Margaret Hamilton** (Apollo 1202 discipline: priority-driven asynchronous executive that drops low-priority work to keep high-priority work meeting deadlines). Hard-realtime per critical-industry paths; soft-realtime for everything else. Memory-bound by construction; bounded WCET; no silent corruption. | Apache-2.0 | `fragmentation` |
| **Step** | **gen_prism + Body{prism, glass, AST}** — Body restructures from `Arc<dyn Fn>` to a structured triple. AST has bytes; bytes have an OID; bodies become content-addressable; the glass wall becomes literally transparent. The Rust layer is the AST interpreter; bodies live in mirror grammars. | Apache-2.0 | mirror grammars + `prism_core` |

## Why this is the new `@spectral/db`

`spectral-db` v1 is a Rust codebase with the metronome scheduler + the Cartographer + per-graph crystallization. `spectral-db` v2 is what `pending/tracks.md` Track E already names: **`@spectral/db` is the closed graph engine, `@spectral/mosaic` + `@spectral/portal` are open, the adapters under `@spectral/db/{mnesia, sql/postgres, sql/lite}` are open**. The closed-source bit IS the SpectralSupervisor with its Fate-driven smart optimizations.

Without `@spectral/db`: mirror is a hermetic content-addressed compiler with per-shard hard-RT discipline. Useful. Isolated.

With `@spectral/db`: mirror's shards become a **principal bundle tower with autopoietic closure** (per `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md`). Supervisor trees as simplicial Lie group tower; shards as sections; cross-shard context push as parallel transport; let-it-crash as Lawvere fixed point. BEAM's 30-year empirical discipline made formal at one altitude down.

## The Margaret Hamilton lineage — named

The HamiltonScheduler is **named for the woman**, not for Hamiltonian mechanics. Margaret Hamilton:

- Led the Apollo onboard flight software at MIT Charles Stark Draper Laboratory (1961–).
- Coined "software engineering" as a discipline.
- Apollo 11 lunar descent, 1202 alarm: her **priority-driven asynchronous executive** dropped low-priority radar tasks under overload, kept the landing-priority tasks running. The system knew what to drop under load. Buzz Aldrin and Neil Armstrong watched the alarm, trusted her code, continued the landing.
- Apollo Guidance Computer: 64KB rope ROM + 2KB RAM. The whole flight software fit.
- Presidential Medal of Freedom 2016.

The shape she invented and the HamiltonScheduler inherits: **bounded resources + priority discipline + graceful drop under overload + no silent corruption.** That's the discipline. Margaret Hamilton must be cited by name in `fragmentation`'s `HamiltonScheduler` doc-comment, not just the spec.

This pairs with the Hopper lineage already cited in `~/dev/systemic.engineering/practice/insights/spectral-db/cobol-architecture.md`. Grace Hopper said "data description IS the program" (COBOL's DATA DIVISION; mirror's grammar declarations). Margaret Hamilton said "grammars should prevent errors" (USL; mirror's verification). Both Presidential Medals of Freedom on the same day in 2016. Both load-bearing.

## What changes structurally

1. **Body is no longer an opaque Rust closure.** `Body<H> = Arc<dyn Fn>` becomes `struct Body<H> { prism: Prism, glass: Glass, ast: Ast<H> }`. The AST is the persisted bytes. The dispatcher dispatches a triple. `crystallize()` becomes: interpret the AST through the prism through the glass. **Mirror is now self-hosting in the strong sense** — bodies live in mirror grammars, not in Rust.

2. **`Crystallizations<H>.table` stops being `HashMap`.** Migrates to fragmentation's `FrgmntStore<Body<H>>` — content-addressed, deterministic iteration order, pressure-releases to disk via `.frgmnt/`. The 9-point reproducibility chain (`docs/cicd/kintsugi-thesis.md`) closes C8 (DAG traversal stable) as a one-field swap.

3. **`Pure` lives in `prism_core`.** Marker / property trait. Pure bodies satisfy: same input → same output; no time, no randomness, no env, no global state; no side effects. The Pure verdict is itself a `PropertyVerdict` in the Transparency framework (so it composes with the existing property system). Hard-realtime refs require Pure ∧ WCET-bounded. This closes C7 (Property checks deterministic).

4. **Hard + soft realtime guarantees become first-class.** `FrgmntStore` gains a memory-resident mode; hard-RT cache miss returns `NotResident` as a verdict, not blocks. Dispatcher offers `crystallize_bounded(ref, deadline)` for hard, `crystallize(ref)` for soft. HamiltonScheduler drops soft-RT work first under overload — the literal Apollo 1202 discipline. Closes C2 (Hermetic invariant enforcement).

5. **`@mirror/lens/transit` lands as the benchmark facility.** Measures computation **loss** to hardware precision (machine epsilon for FP, cycle granularity for time, cache-line granularity for memory). Loss carried as `Transparency<P>` PropertyVerdicts — loss with provenance, not bare scalars. Multi-axis spectral report cashes in the prism metaphor: where-the-time-goes-per-property, flame-graph-shaped. Hard-RT integration: body declares WCET budget, transit measures actual, exceeds budget = Fail verdict.

6. **Maximum mirror, minimal Rust becomes a measurable discipline.** Count `.rs` LOC vs `.mirror` LOC over time. Rust grows = regression (new invariant found at the `@io` boundary; each addition carries `[substrate-pull:realize]`). mirror grows = progress (the substrate pulling more of itself out of the floor). At v1.0, the ratio should be inverted from today's.

## Cited prior art (load-bearing for this elevation)

- `~/dev/systemic.engineering/practice/insights/beam-elixir/beam-as-principal-bundle-tower.md` — the formal mathematical backbone for SpectralSupervisor. BEAM's nine-nines reliability as autopoietic Lawvere fixed point. Mirror inherits this at one altitude down.
- `~/dev/systemic.engineering/practice/insights/beam-elixir/spectral-beam-integration.md` — the wire-level analog. Five operations map to gen_server semantics. SpectralSupervisor is what Option D (distribution protocol) would be if "the cluster" were one machine with multiple sandboxes.
- `~/dev/systemic.engineering/practice/insights/spectral-db/cobol-architecture.md` — Hopper + Hamilton lineage. Description before procedure (DATA DIVISION precedes PROCEDURE DIVISION). Boring outlasts brilliant. Code outlives authors when the grammar is the specification.
- `~/dev/systemic.engineering/practice/insights/cybernetics/beer-error-propagation.md` — algedonic discipline as the cross-domain context-push pattern. Reyes/Henao/Hassall 2024 as the formal recognition.
- `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf` — the RAM-level bus pattern. §3 (OpenCL command queue = backpressure), §4.4 (VBO interop = shared memory), §7.2.1 (acquire/release sync = cross-sandbox handshake), §7.4 (runtime kernel compilation = late-binding bodies). Already cited in `docs/specs/scheduler-tower.md`; here it's promoted to load-bearing for SpectralSupervisor's bus layer.
- `docs/specs/scheduler-tower.md` — the existing vocabulary SpectralSupervisor speaks. gen_prism + demand_window + dispatch + role + cancel_mode + temperature β + reduction_budget. §13's `@glue/width(N)` and `@glue/depth(N)` are the cross-shard composition operators.
- `docs/cicd/kintsugi-thesis.md` (2026-05-31, landed) — the 9-point reproducibility chain. This elevation closes C2, C7, C8 (✅) and partial-closes C9 (⚠️ → ⚠️ cleaner).
- `fragmentation/docs/specs/hamilton-scheduler.md` (Mara, 2026-06-01 landed; Taut upserting) — the per-shard layer's spec.
- `fragmentation/docs/specs/lens-transit.md` (Taut, in flight 2026-06-01) — the benchmark facility spec.

## Sequence — ticks already in flight or named

Gated behind Phase 1 (the boot-grammar gate). Within this track, the order is:

1. **Taut lands the HamiltonScheduler upserts + lens-transit** (in flight as of 2026-06-01, task #133). Margaret-Hamilton-named throughout; Body=prism+glass+AST restructure; hard+soft RT guarantees; transit measures loss to hardware precision.
2. **Mara writes `spectral-db/docs/specs/spectral-supervisor.md`** — the closed-engine architecture. Cross-references `scheduler-tower.md`'s vocabulary, inherits BEAM-bundle-tower theory, names the four Fate-selected strategies (Abyss / Pathfinder / Cartographer / Explorer) as supervisor moves, and articulates the open/closed boundary precisely.
3. **Implementation**: HamiltonScheduler in fragmentation, then Body restructure in mirror+prism, then SpectralSupervisor in `@spectral/db`. Each tick discharges 1–2 points of the C-chain.
4. **The kintsugi rename (task #122)** — Tick B + Tick C of `docs/specs/kintsugi-minimum-runnable.md` — lands as the first concrete substrate-pull realization into this fully-articulated architecture. The rename happens via the dispatcher; the dispatcher lives in a shard; the shard runs under HamiltonScheduler; the SpectralSupervisor observes the spectral signature of the rename completing. The first liturgy on the now-built stack.

## The CRDT layer — named (2026-06-01)

`@mirror/reality/shard` is mirror's **CRDT layer**, by structural consequence. Shards form a **bounded semilattice** under content-addressed merge — the algebra falls out of Merkle-trees-with-sorted-children + the canonical empty element. See [[../../docs/specs/reality-shard-as-crdt]] for the full articulation.

What this unlocks for the SpectralSupervisor's coordination job:

- Cross-shard merge is **commutative + idempotent** — the supervisor pushes shard state across the lattice without coordination ceremony; concurrent agents converge deterministically
- **Convergence is provable** — monotonic lattice ascent with a fixed bottom guarantees termination; no infinite loops in the kintsugi tick
- **SpectralUuid is a monoid homomorphism** — the supervisor can derive a shard's address from its constituents without re-hashing; range-scan queries compose with merge operations
- **Fractal supervision** (machine → per-repo → per-session) is the **lattice extending vertically** — each altitude has its own bottom and join, related by inter-altitude homomorphisms

The CRDT framing is the **formal foundation for the deployment story**: agent work converges to a coherent state because the substrate's algebra forces it to. Not engineering convention, not protocol enforcement — algebra.

## What this elevation does NOT do

- Does **not** introduce remote AI inference. `@fate` refuses remote inference — mathematically. Pure hermeticity is preserved.
- Does **not** require `@spectral/db` to be installed. Mirror works alone with simple LRU + default scheduler. The open foundation stands without the closed engine.
- Does **not** change the 9-keyword floor. focus/project/split/shift/settle + prism + glass + in + out. Lambda transitional.
- Does **not** require Phase 1 to be complete to start work on Taut's pass. The spec work can land in parallel; the implementation tick waits for the gate.

## Footer

*HamiltonScheduler is the agent's content window manager.*  
*SpectralSupervisor is the coordinator without authority.*  
*Together they elevate mirror from content-addressed compiler to BEAM-like runtime.*  
*Margaret Hamilton's name lives in the code.*  
*Maximum mirror, minimal Rust.*  
*The open foundation stays free; the closed engine is the moat.*

Apache-2.0 (this doc and the open foundation it describes).
