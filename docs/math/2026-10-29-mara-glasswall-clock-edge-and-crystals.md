# The glasswall, the clock edge, and crystals as native functions — math note

*2026-10-29. Mara (with Scarlet). Spec (future): `docs/specs/mirror-native.md`. Builds on: the algebra insight (protected: `practice/insights/cybernetics/the-algebra-of-non-linear-relating.md` §1.5, "The clock is the glasswall in hardware. A register replaces simultaneous consistency (trace) by iteration in time."), the engine contract (fixpoint spec §4.1–§4.2), the settle math (`docs/math/2026-10-29-mara-settle-on-a-fractal-and-the-loop.md`, "settle §n"), the coupling math ("coupling §n"), and the engine/frequency math ("freq §n").*

Labels: **PROVED**, **CONJECTURE**, **OPEN**, **REFUTED**, **NARROWED**, **READING**.

**Alex:**
- "Logic gates. Machine code. That's the glasswall between the nontrivial and trivial machine. The linear and the nonlinear. The Kleinos DAG of logic gates. The thunder."
- "And a Fragment is a trivial machine crystallized. And when we have the machine code mirror becomes self-modifying. Because those can become native functions. A Crystal becomes memorized computation."

---

## §0 Verdicts

1. **Within a tick, the engine is combinational** (PROVED from the code's structure). A semi-naive round reads `(M_{t−1}, Δ_{t−1})` and writes `next`, which nothing in the round reads. So a round is a function: a DAG of rule instances from inputs to new atoms. That is the thunder.
2. **Every recursive dependency is cut by the tick boundary** (PROVED). It is the engine's form of the synchronous-circuit discipline that "all feedback … must pass through" a register (prior art below). The tick boundary (deposits latch, the root is frozen, observers read, restrictions settle) is the clock edge.
3. **The non-trivial machine lives only in loops through the boundary** (PROVED, definitional): state is what persists across ticks, and only the latched model persists.
4. **"No loops within a tick ⇒ every discrepancy bookable ⇒ single truth": NARROWED.**
   - The conclusion holds: a combinational DAG with single drivers has exactly one consistent assignment for given inputs.
   - The stated reason is wrong. A gate DAG can have undirected cycles (reconvergent fan-out), and coupling Prop 2.4 is about undirected cycles. What makes a tick single-truth is **directed acyclicity plus single assignment**: each wire is written once, and no two paths are required to agree.
5. **"Holonomy can only live across the glasswall": NARROWED.**
   - The holonomy of *given* transports is a product around a cycle. That is a combinational computation, and a tick can do it.
   - What needs the clock is **feedback**: fixpoints (including the closure-third, which generally needs several ticks), orbits and frequencies, W's period-2 atoms, state, and the order-third *as produced by interleaving moves over time*.
   - Restated: **the trace lives across the glasswall; functions live within it.**
6. **A settled fragment is a crystallized trivial machine** (PROVED from settle A.5/A.6): its value is a pure function of its cone's addresses, and caching it by address is lossless. **Content addressing is memoization** (PROVED; prior art: Nix content-addressed derivations, Shake/Salsa early cutoff).
7. **Native promotion** (future):
   - what can become machine code is the **combinational part**: a component's round function, compiled from its settled clause set;
   - replacing it can't introduce hidden loops or state, provided the native code is pure, total and computes the same round function (PROVED, conditional);
   - **equivalence beyond replayed inputs is OPEN** (translation validation), so promotion is an Authority ruler, revocable, demoted on mismatch.
8. **"Fire C: Rust exits the picture": NARROWED.** Mirror can generate its own native code, but some floor still runs the first E: the irreducible witness of fixpoint math §6.1. The trust root moves to E's shard plus DDC. It never vanishes (CONJECTURE that a mirror-generated floor can pass DDC).
9. **The `@cascade/code/*` shards** sketch the **lifting** direction (Rust → LLVM IR → Turing tape → mirror value), with every body a `\`. The **lowering** direction this needs (crystal clauses → LLVM IR → native) isn't sketched. Their coherence and loss bilaterals are the right template for the promotion check.

---

## §1 Within a tick: combinational (the thunder)

**Proposition 1.1 (a round is a function; PROVED).** In `least_model` (`rust/engine/src/lib.rs`, at the slice-2 merge `1ae650de`), round t computes `next` from `model` (`M_{t−1}`), `delta` (`Δ_{t−1}`) and the index of `M_{t−1}`. `next` is not read until the round ends, when it becomes `Δ_t` and joins M. So the round's output is a function of its two inputs, with no feedback inside it. The rule instances form a DAG: premises from `M_{t−1}`/`Δ_{t−1}`, heads into `next`.

**Proposition 1.2 (single truth within a tick; PROVED).** Given the inputs, a DAG of deterministic gates with one driver per wire has exactly one consistent assignment: evaluate in topological order. In Datalog, several derivations of one atom are absorbed by set union, which is idempotent. So no wire carries two values.

**Why "no loops" is the wrong reason (REFUTED as stated).** Reconvergent fan-out `a → b → d`, `a → c → d` is a cycle in the undirected graph. Coupling Prop 2.4 counts undirected cycles. A DAG can carry them and still be single-truth, because nothing asks the two paths to agree: `b` and `c` are different wires into `d`. Curvature needs a loop **and** a requirement that the transports around it compose consistently. A combinational DAG has no such requirement.

READING. Förster's trivial machine (output a function of input) is exactly a combinational DAG. Kleinos's "DAG of gates" is the within-tick picture. Alex's "thunder" is the round: everything in it is determined at once by what latched at the last edge.

## §2 Across ticks: the clock edge

**Proposition 2.1 (every recursive dependency crosses the boundary; PROVED).** A dependency cycle among predicates (`path :- path, e`) is never evaluated within one round. Its atoms derived at round t are inputs only at round t+1 (Prop 1.1). This is the engine's form of synchronous design: "always using registers in the feedback path leads us to the formal definition of a synchronous sequential circuit" (ScienceDirect, *Synchronous Sequential Circuit*, via Kagi). The register is the deposit latch plus the immutable root (fixpoint spec §4.1; ruler math §5's tick-boundary observation).

**Proposition 2.2 (the non-trivial machine lives in loops through the boundary; PROVED, definitional).** Förster's non-trivial machine has an internal state that changes the input–output map. In the engine, the only thing that persists from one round to the next is the latched model. So every behaviour that depends on history runs through the boundary.

**Proposition 2.3 (what needs the clock; PROVED).**
- Least fixpoints, including E₃ beyond one step (freq §2.3).
- Orbits and frequencies (freq F1–F6).
- W's alternation and its period-2 atoms (freq F6).
- Convergence passes.

Each is defined by iteration of a step, so each lives across ticks.

**The ring and the latch (PROVED on the two examples; READING beyond them).** The algebra insight reads the SR latch as two eigenforms (memory) and the ring oscillator as none (oscillation). In W's terms:
- `p :- not q. q :- not p` is two cross-coupled inverters. It has two stable models, `{p}` and `{q}` (two eigenforms), and W reports both atoms undefined: it keeps the latch open and doesn't pick.
- `p :- not p` is an odd ring. It has no stable model, and W reports `p` undefined, the oscillation.

W's γ is one inverter stage per evaluation (antitone, settle B.6). Clocking it gives F6's period 2.

## §3 Holonomy and the glasswall (NARROWED)

- Holonomy of **given** data (a ruler intonation, the circle of fifths) is a product around a cycle. A tick computes it combinationally, so no clock is needed to *measure* a comma.
- Holonomy **produced by feedback**, a loop of the system's own dependence on itself that doesn't close, needs the clock. Examples: W's undefined atoms, a convergence `cycle(p)`, the order-third as it arises when moves interleave over time.
- So the sentence that holds: **the trace (feedback) lives across the glasswall, and functions (including the measurement of given holonomy) live within it.** This is the algebra insight's "a register replaces simultaneous consistency (trace) by iteration in time", applied to the engine.

## §4 Crystals as memorized computation

**Proposition 4.1 (a settled fragment is a crystallized trivial machine; PROVED).** By settle A.5 and A.6, a settled fragment's value is constant given its cone. Its address is a Merkle function of the cone's addresses. Serving it from a cache keyed by address leaves every trajectory unchanged.

**Proposition 4.2 (content addressing is memoization; PROVED; prior art).** A memo table maps inputs to outputs. With inputs named by content address, the table is total and needs no invalidation, because a changed input has a new address. Prior art:
- Nix content-addressed derivations ("the store path of an output is not known in advance", nix.dev);
- early cutoff in Shake and Salsa (*Build systems à la carte*, already cited in kintsugi math).

What is memoized is the **value**, not the **trajectory**: RR6's caveat (ruler math §5) applies.

**Two different things Alex's sentence joins (READING, made precise):**
- **memo:** a crystal's address → its value. Always available, and lossless (Prop 4.1);
- **native:** a component's **round function** (its settled clause set, compiled) → machine code. This is "those can become native functions".

A crystal is data. The native function is the compiled thunder that produced it.

## §5 Native promotion, and why it can't add hidden loops (future)

**Proposition 5.1 (combinational replacement is safe; PROVED, conditional).** Replace the interpreter's round function for a component C with native code N. If N is pure (no `@io`, no global state), total, and computes the same function `(M_{t−1}, Δ_{t−1}) ↦ next` on C, then every trajectory is unchanged (determinism, Prop 1.1). N can't create state, because state exists only in the latched model, which N reaches only through its output `next`.

**The safeguards** (spec, future):
1. **Two identifiers, one recorded edge.** The crystal's `addr` stays identity. The native artefact gets its own per-system hash (target triple, compiler identity). A single recorded edge joins them (Reed: mirror-007's two-ids rule; the same split as `root` vs `git` in fixpoint spec §1.2).
2. **A second observer before promotion.** Replay against the interpreter on recorded inputs, and Wheeler's diverse double-compiling for the compiler that produced N. Thompson, *Reflections on Trusting Trust* (1984): "To what extent should one trust a statement that a program is free of Trojan horses?"; Wheeler, arXiv 1004.5534 (via Kagi).
   - **Replay proves equality on the replayed inputs only** (PROVED: a finite test set doesn't determine a function).
   - Equivalence on every input is translation validation: **OPEN**. A component's domain is finite (`2^{|B|}`), so exhaustive checking is possible in principle and infeasible in general.
3. **Promotion is an Authority ruler.** Signed, revocable (a record), and demoted automatically when a sampled interpreter re-run disagrees.
4. **Purity is checked.** Anything touching `@io` stays at the glasswall as `shift @io` and is never promoted.
5. **Only between ticks.** Promotion and demotion happen at a tick boundary, never inside a round, so a round is always one function (Prop 1.1).

**Proposition 5.2 (self-modification without hidden state; PROVED from 5.1).** If mirror only ever replaces combinational round functions, between ticks, under conditions 1–5, the system's state space and its registers are unchanged. Self-modification changes **how fast** a component's thunder runs, never **what** it computes.

## §6 Fire C (NARROWED)

Alex's September "Fire C: mirror writes mirror; Rust exits the picture" gets a mechanism: mirror lowers its own settled clause sets to native code.

What doesn't exit is the **floor**. Something must run the first E, and fixpoint math §6.1 names it the irreducible witness. With DDC (two independent floors building E's shard to bit-identical artefacts, fixpoint spec §3), the floor can become mirror-generated, and the trust root moves to E's shard plus the DDC record.

CONJECTURE: a mirror-generated floor can pass DDC against a Rust floor. That is the empirical form of Fire C.

## §7 The `@cascade/code/*` shards (grep-verified)

`shards/cascade/code/rust/llvm.mirror`, `llvm/turing.mirror` and `turing/mirror.mirror` are edges 1–3 of a chain Alex ratified on 2026-07-17 ("And boom. The loop closes."): Rust → LLVM IR → Turing tape → mirror value. Each declares:
- `apply_*`, `bundle_*` and `measure_*` actions, with `\` bodies;
- coherence and loss bilaterals (`turing_translation_coherent`, `llvm_turing_loss_well_defined`, `cascade_llvm_turing_preserves_computation`).

**Fit.**
- **Direction:** they lift code *into* mirror. Crystals-to-native needs the reverse, lowering mirror clause sets *to* LLVM IR. No shard sketches that.
- **Template:** the `measure_*` actions return `imperfect(artifact, ref, information_loss)`, and the coherence bilaterals are exactly the shape of the promotion check (does the artefact preserve the computation; what was lost).

So the spec reuses their bilaterals and adds the lowering edge. Nothing in them is landed.

## §8 Engine §4.2 findings (recorded)

- **`Instance::new` is generic over a sealed `Program` trait** (implemented by `PositiveProgram` and `NormalProgram`). This adds one public name, `Program`. Accepted: sealing keeps outside implementations out, and one constructor serves both typestates.
- **Arity conflicts among rules alone** (no facts involved) report the **least two** arities seen for the predicate (`expected` = the least, `found` = the next), so the report doesn't depend on rule order. A conflict between program and facts is found by `Instance::new`, with `expected` = the program's arity and `found` = the fact's (fixpoint spec §4.2 decision).
- **`Run` fields are still `pub`** (`model`, `model_addr`, `deposits`, `stop`, `findings`). A test, or a caller, can build a `Run<Fixed>` that no run produced, forging the guarantee the typestate exists to carry. **Decision (spec-owned, Mara):** the fields become private, with read-only accessors, and `Run` is constructible only inside the engine. Tests that need a `Run` get one by running. This lands as a 🔴/🟢 pair (a `compile_fail` doctest building `Run { … }` outside the crate).

## §9 Is this JIT for mirror? (Alex) — Futamura's first projection

Close, but without speculation. The differences are exact (PROVED by definition):

| | a JIT (e.g. Truffle/Graal) | mirror native promotion |
|---|---|---|
| trigger | **hot**: profiling | **settled**: correctness (settle A.4) |
| assumptions | speculative ("uses partial evaluation … in combination with dynamic speculation and run-time profiling feedback", Graal docs) | none: a crystal's address is its content |
| guard | type and shape checks; deoptimise on failure | an address comparison, and nothing else |
| reuse | per process | content-addressed: across runs and machines (like Unison, where "each … definition is identified by a hash of its syntax tree", and like Nix/Bazel/Salsa) |

**The precise name: Futamura's first projection.** Partially evaluating the engine (an interpreter) with respect to a settled fragment's clause set gives a compiled program for that fragment (Futamura 1971; reprinted HOSC 12(4) 1999, "Partial Evaluation of Computation Process — An Approach to a Compiler-Compiler"; Wikipedia, *Partial evaluation*; via Kagi). Truffle is the closest prior art for partially evaluating an interpreter (Marr & Ducasse: "Truffle … uses partial evaluation as meta-compilation technique"). Mirror differs in having no speculation to undo. **Demotion happens only on a replay mismatch, which is a bug finding, never a deoptimisation.**

## §10 The Hamilton scheduler: a memory hierarchy ordered by observability (future)

**Alex:** "Maybe the Hamilton scheduler is what keeps hot crystals in memory and evicted crystals become settled machine code."

### §10.1 The inversion of JIT (READING, made precise)

- A crystal kept in memory as **data** can be *observed*: pacts read it at tick boundaries, kintsugi reads its interface, rerere reads its active bits, convergence diffs it.
- Native code is a trivial machine running blind: executable, not observable.
- So **eviction to native crosses the glasswall downward** (relation → function, observable → executable). What it erases is the **ability to look**, not the computation. The crystal stays in the content-addressed store, and re-observing means reloading it (PROVED: the address and the store are untouched by promotion, §5 safeguard 1).
- In a JIT, the hot code is compiled. Here the **observed** crystals stay data, and the **unobserved, settled** ones become code. Hotness of execution and hotness of observation come apart.

### §10.2 Three tiers

1. **Flux** (not settled; can't become code): shed under pressure, recomputable from the crystals it depends on, with an erasure recorded (`PressureEvent`).
2. **Hot crystals** (settled, observed): kept as data.
3. **Cold crystals** (settled, unobserved for long enough): tenured into native code, after the second observer (§5: replay or DDC). Demoted automatically on a mismatch.

**Tenuring has prior art:** generational garbage collection, where objects that survive enough collections are promoted (Ungar, *Generation Scavenging*, SIGPLAN Notices 19(5) 1984, via Kagi). Here the survival criterion is **settled and unobserved for k ticks**, where k is a declared Budget ruler.

**The priorities are a declared ruler, never a heuristic.** Which flux to shed first, how many ticks make a crystal cold, and how much to shed at once are Budget/Authority rulers (ruler spec §1), signed and recorded in every `PressureEvent`.

### §10.3 Hamilton and the 1202 alarm (cited; READING)

The Apollo Guidance Computer's EXECUTIVE, faced with more jobs than it had room for, "respond[ed] … by turning on the PROG light (program alarm), storing the 1201 or 1202 alarm code, stopping all jobs and tasks", and restarting the ones that mattered (Cherry, *Exegesis of the 1201 and 1202 Alarms Which Occurred During the Apollo 11 Lunar Landing*, ibiblio.org/apollo, via Kagi). Priorities were decided before flight. That is the shape §10.2 asks for: **shed under load, by priorities declared in advance, with a record**.

### §10.4 What spectral-db's `pressure.rs` already does (read-only; `spectral-db/src/pressure.rs`, last change `2d0c1bd`)

**Does:**
- detects overflow against a capacity (`check_pressure`);
- sheds non-crystallized nodes when full;
- **crystals survive shedding** (`crystals_survive_shedding` test);
- records every shed as a `PressureEvent { evicted, preserved, trigger }`, kept in an event history that can be restored.

**Doesn't, or disagrees with its own doc:**
- **Priority is not an eigenvalue.** The module doc says it evicts "the lowest-eigenvalue non-crystallized nodes" and preserves "crystals + high eigenvalue". The code sorts by the first 8 bytes of each node's hash ("lower bytes = lower priority"). That is an arbitrary, undeclared order: a disguised ruler in Taut's sense.
- **"Shed 25% of capacity"** is a bare constant (a disguised Budget ruler).
- `PressureEvent` records what was shed, but **not which ruler chose it**: no ruler address.
- There is no tier between "kept" and "evicted": no observability, no tenuring, no native code. Crystals are never evicted, the opposite of Alex's tier 3, where cold crystals leave memory as code.
- It uses `serde` for the event, which mirror's spec excludes on its own path (fixpoint §9). That doesn't matter for spectral-db.

**Fit:** `pressure.rs` is tier 1 (flux shedding, crystals protected, erasure logged), with an undeclared priority. Tiers 2–3 and the declared ruler are new.

### §10.5 Scarlet's margin (a disagreement, kept)

- **Scarlet:** "A book nobody is reading isn't cold. It's waiting. The library never sends a book somewhere it can't be opened."
- **Mara:** the crystal never leaves the store. It stays readable by address, and only its *running* form is blind.
- **Scarlet:** "Then don't call it evicted. Nothing left."
- **Status:** the spec names tier 3 **tenured**, not "evicted" (Ungar's word; it says the crystal earned it). Whether "cold" is fair stays on the list.

## Ledger

| claim | status |
|---|---|
| a round is a combinational function | PROVED (code structure at `1ae650de`) |
| single truth within a tick | PROVED (single assignment + DAG) |
| "no loops ⇒ bookable" as the reason | REFUTED (reconvergent fan-out); NARROWED to directed acyclicity + single drivers |
| recursion is cut by the tick boundary | PROVED |
| state and history live only in loops through the boundary | PROVED (definitional) |
| holonomy only across the glasswall | NARROWED: feedback lives across; measuring given holonomy is combinational |
| latch = even negative cycle (two stable models); ring = odd (none) | PROVED on the examples; READING beyond them |
| a crystal is a crystallized trivial machine; content addressing is memoization | PROVED |
| combinational native replacement adds no hidden state | PROVED (conditional on pure, total, equal function) |
| replay proves equality on replayed inputs only | PROVED |
| full equivalence of native and interpreter | OPEN |
| a mirror-generated floor can pass DDC | CONJECTURE |
| "Rust exits the picture" | NARROWED: the floor stays, its origin can change |
| cascade shards fit | direction reversed; bilaterals reusable (grep) |
| native promotion = Futamura's first projection, no speculation, address-only guard | PROVED (definitions); prior art cited |
| eviction to native erases observability, not computation | PROVED (store and address unchanged) |
| three tiers, tenuring by a declared ruler | definition (future) |
| spectral-db `pressure.rs` = tier 1; its priority is a hash order, not the documented eigenvalue | read-only finding |
| "cold" as the name for unobserved crystals | disputed (Scarlet) |

## Sources (Kagi, this session)

- ScienceDirect Topics, *Synchronous Sequential Circuit*, https://www.sciencedirect.com/topics/computer-science/synchronous-sequential-circuit (registers in every feedback path; state changes only at the clock edge). **[snippet]**
- K. Thompson, *Reflections on Trusting Trust*, CACM 27(8) 1984, https://www.cs.cmu.edu/~rdriley/487/papers/Thompson_1984_ReflectionsonTrustingTrust.pdf. **[snippet]**
- D. A. Wheeler, *Fully Countering Trusting Trust through Diverse Double-Compiling*, arXiv 1004.5534, https://arxiv.org/abs/1004.5534. **[abstract page]**
- Nix manual, *Store Derivation and Deriving Path* (content-addressed derivations), https://nix.dev/manual/nix/2.34/store/derivation/. **[snippet]**
- Protected corpus: the algebra insight §1.5, quoted by phrase only.
- Y. Futamura, *Partial Evaluation of Computation Process — An Approach to a Compiler-Compiler* (1971; HOSC 12(4) 1999); Wikipedia, *Partial evaluation*. **[snippet]**
- Graal/Truffle docs, *PartialEvaluation.md*, https://github.com/oracle/graal/blob/master/truffle/docs/PartialEvaluation.md; S. Marr & S. Ducasse, *Tracing vs. Partial Evaluation* (OOPSLA 2015). **[snippets]**
- Unison, *The big idea*, https://www.unison-lang.org/docs/the-big-idea/. **[snippet]**
- D. Ungar, *Generation Scavenging*, ACM SIGPLAN Notices 19(5) 1984, https://dl.acm.org/doi/10.1145/390011.808261. **[abstract]**
- D. Cherry, *Exegesis of the 1201 and 1202 Alarms*, https://www.ibiblio.org/apollo/Documents/CherryApollo11Exegesis.pdf. **[snippet]**
- spectral-db `src/pressure.rs` (read-only, local).
