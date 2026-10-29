# `mirror native` — crystals as native functions (spec, **future**)

*2026-10-29. Mara (with Scarlet). Status: **Future**. Nothing here is built before T1, kintsugi v0 and convergence v0. Math: `docs/math/2026-10-29-mara-glasswall-clock-edge-and-crystals.md` (cited "math §n").*

**The line:**
- within a tick, the engine is combinational (a function);
- the tick boundary is the clock edge;
- state lives only in what latches there.

Native promotion replaces **combinational** round functions with machine code, **between ticks**, and nothing else. This is Futamura's first projection (math §9), not a JIT: the trigger is *settled*, not *hot*, and the only guard is an address comparison.

## §1 Types (sketch)

```rust
/// A settled component's round function, compiled. Identity stays the crystal's addr.
pub struct Native {
    crystal: Addr,                  // identity: the clause set's address
    artefact: ArtefactId,           // per-system hash: target triple + compiler identity + bytes
    promoted_by: Addr,              // the Authority ruler that promoted it (revocable)
    witness: PromotionWitness,      // Replay { inputs: NonEmpty<Addr> } | Ddc { floors: (Addr, Addr) }
}

/// The observability tiers (math §10), governed by a declared ruler.
pub enum Tier { Flux, Hot, Tenured(Native) }

pub struct Hamilton<'r> {
    shed_order: &'r Approved<'r, Choice>,       // which flux is shed first: declared, never a hash order
    cold_after: &'r Approved<'r, Budget>,       // ticks settled and unobserved before tenuring
    shed_amount: &'r Approved<'r, Budget>,      // how much to shed at once (spectral-db: a bare 25%)
}
#[must_use]
pub struct PressureEvent { shed: Vec<Addr>, kept: Vec<Addr>, tenured: Vec<Addr>, trigger: Trigger, by: NonEmpty<Addr> }
```

## §2 Laws

- **N1 (combinational only):** a `Native` implements exactly one component's round function `(M_{t−1}, Δ_{t−1}) ↦ next`. It is pure (no `@io`, no global state) and total. Anything touching `@io` stays at the glasswall as `shift @io` and is never promoted (math Prop 5.1).
- **N2 (between ticks):** promotion and demotion happen only at a tick boundary.
- **N3 (two ids, one edge):** `crystal` is identity. `artefact` is a separate per-system hash. The edge between them is recorded once. No code path compares the two as identities (cf. fixpoint §1.2).
- **N4 (second observer):** promotion needs a `PromotionWitness`: replay against the interpreter on recorded inputs, or DDC for the compiler that built the artefact. Replay proves equality on the replayed inputs only (PROVED), so a witness is evidence, not a proof. Full equivalence is OPEN.
- **N5 (Authority, revocable, demoted on mismatch):** promotion is an Authority ruler (ruler spec §1). A sampled interpreter re-run that disagrees demotes automatically and records a bug finding, never a deoptimisation.
- **N6 (Hamilton is a declared ruler):** shed order, cold-after and shed amount are signed rulers. Every `PressureEvent` names them (`by`).
- **N7 (tiers by observability):**
  - flux is shed (recomputable from its crystals) with the erasure recorded;
  - observed crystals stay as data;
  - unobserved settled crystals may be **tenured** into native code;
  - tenuring erases observability, never content: the crystal stays in the store, readable by address.
- **N8 (Fire C, narrowed):** mirror may generate the floor that runs E only after DDC against an independent floor. The trust root is E's shard plus the DDC record, never empty.

## §3 Prior art and the corpus

- `shards/cascade/code/{rust/llvm, llvm/turing, turing/mirror}.mirror` sketch the **lifting** chain into mirror, with `\` bodies. The **lowering** edge (clause set → LLVM IR) is new. Their coherence and loss bilaterals (`*_translation_coherent`, `*_loss_well_defined`, `*_preserves_computation`) are reused as the shape of N4's check.
- spectral-db `src/pressure.rs` is tier 1:
  - it sheds non-crystals, protects crystals, and logs `PressureEvent { evicted, preserved, trigger }`;
  - its priority is a hash-byte order, not the eigenvalue its doc claims;
  - its shed amount is a bare 25%;
  - it has no ruler in the event.

  N6 replaces all three.

## §4 Red tests (sketches; not for now)

- **NT1:** a promoted round function and the interpreter produce equal `next` on every replayed input, and a deliberately miscompiled artefact is demoted on the first sampled mismatch, with a finding.
- **NT2:** an `@io`-touching component is refused promotion.
- **NT3:** promotion inside a round doesn't type-check (only the boundary API takes a `Native`).
- **NT4:** `crystal` stays the identity. Two systems produce different `artefact`s for one `crystal`, and both are accepted.
- **NT5:** a Hamilton without a declared shed order doesn't compile. A `PressureEvent` lists the rulers that chose.
- **NT6:** a tenured crystal is still readable by address, and re-observing it reloads the data tier.

## §5 Engine §4.2 findings, recorded (applies now, not future)

See fixpoint spec §4.2 and math §8:
- `Instance::new` is generic over a sealed `Program` trait (one extra public name, accepted);
- arity conflicts among rules alone report the least two arities;
- **`Run` fields become private**, with read-only accessors and no outside construction (a 🔴/🟢 pair).
