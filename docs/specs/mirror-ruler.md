# `Ruler` and `Pact` (spec)

*2026-10-29. Mara (with Scarlet). Status: **Red**. Math: `docs/math/2026-10-29-mara-ruler-and-pact.md` (cited "math §n"). Builds on `docs/specs/mirror-fixpoint-parser.md` §4.1–§4.3 (E, newtypes, `Frequency`), `docs/specs/mirror-settle.md` (`settle`, `Erasure`), `docs/specs/mirror-coupling.md` (`Thirds`, b₁). Alex: "We really need to make the `Ruler` explicit … probably parts of the Rust-side pact enforcement."*

**The line this spec holds: meaning lives in the clauses, policy lives in the pacts.**

## §1 Types

```rust
/// One narrowing decision. Content-addressed; its address is what every
/// shaped result carries.
pub struct Ruler {
    addr: Addr,
    kind: RulerKind,
    content: RulerContent,              // Horn rules for E, or an exact value with its unit
    reader: ReaderDependence,           // declared, never inferred
}
pub enum RulerKind { Threshold, Budget, Quantizer, Choice, Ban, Weight, Tolerance }
pub enum RulerContent { Rules(PositiveProgram), Exact { value: Ratio, unit: Unit }, Count(NonZeroU64) }
pub enum ReaderDependence { Independent, Dependent { reader: ReaderRef } }

/// A ruler someone may apply. Only constructible by verifying a signature
/// (or a governance cycle that reaches a human signature, §2 R5).
pub struct Approved<'r> { ruler: &'r Ruler, by: ApprovalChain }
impl Approved<'_> { pub fn verify(r: &Ruler, chain: ApprovalChain, keys: &Keyring) -> Result<Self, Unapproved>; }

/// Revocation is a record, not a deletion.
pub struct Revocation { ruler: Addr, by: ApprovalChain, at: Round }

/// A fractal of rulers: leaves are each party's rulers, inner nodes are
/// shared rulers on a line; check edges may cross.
pub struct Pact {
    tree: Fractal<Ruler>,
    checks: Vec<(Addr, Addr)>,          // ruler-to-ruler checks
    b1: Betti1,                         // of the ruler graph; always reported
    shape: PactShape,                   // computed, never declared
    commas: Vec<RulerComma>,            // held exactly
}
pub enum PactShape { Hierarchy, Kept, Polycentric }  // math §2 verdict

pub struct RulerComma { question: Addr, left: (Addr, Verdict), right: (Addr, Verdict), atoms: Addr }

/// Every result shaped by rulers says which.
pub struct Shaped<T> { value: T, shaped_by: NonEmpty<Addr>, erasure: Erasure }
```

**Engine-side (E with pacts; math §5):**

```rust
/// Looking: reads an immutable snapshot and returns findings. It can't ban.
pub trait Observe { fn observe(&self, snapshot: &ModelRoot) -> Vec<Finding>; }
/// Ruling: returns bans for the next tick. Every Restrict is a ruler and
/// runs only as an Approved one. (Split at Scarlet's insistence: an
/// observer that bans is a ruler, not a reader.)
pub trait Restrict { fn restrict(&self, snapshot: &ModelRoot, findings: &[Finding]) -> BanSet; }

pub fn run_restricted(
    inst: &Instance,
    pacts: &[Approved<'_>],
) -> Shaped<RestrictedRun>;           // #[must_use]

pub struct RestrictedRun {
    model: ModelRoot,
    deposits: Vec<Deposit>,
    withheld: Vec<(Addr, Tick, Addr)>,   // (atom, tick, ruler): the erasure
    late_bans: Vec<(Addr, Tick, Addr)>,  // atoms deposited before a ban covered them
    orbit: Orbit<Monotone>,
}

/// For programs with negation: restrictions are reported as status moves
/// against the unrestricted W model, never applied silently.
pub fn well_founded_restricted(inst: &Instance, pacts: &[Approved<'_>])
    -> Shaped<(WellFounded, Vec<StatusMove>)>;
```

## §2 Laws

| # | law | what it removes | math |
|---|---|---|---|
| R1 | **no narrowing parameter without a ruler**: no API takes a bare `f64` threshold, `u64` budget or `bool` tolerance. Narrowing parameters are `&Approved<Ruler>` | silent policy | §1, fixpoint §4.2 |
| R2 | **every shaped result carries the shaping ruler's address** (`Shaped.shaped_by`) and its erasure (`#[must_use]`) | results whose policy can't be traced | settle L3/L7 |
| R3 | **reader dependence is declared.** A `Weight` ruler with `Dependent { reader }` names its reader, and a result shaped by it is valid for that reader only | the coda's comma decided by default instead of by declaration | §1, Prop 1.2 |
| R4 | **a ruler comma is held exactly** (both verdicts, both addresses). Booking it upward is a `settle` with the parent ruler's address and an erasure listing the overruled verdict | a silent absorption upward | §3 |
| R5 | **every approval chain ends in a sink or a governance cycle**, and in mirror at least one member of that terminal component is a human signature. `Approved` can't be built otherwise | rulers that approve themselves with no one at the end of the chain | Prop 4.2 |
| R6 | **`PactShape` is computed, never declared**: `Hierarchy` iff b₁ = 0 and every comma booked upward; `Kept` iff b₁ = 0 with some comma left open; `Polycentric` iff b₁ ≥ 1. `b1` is always reported | a single-truth regime presented as a fractal | Prop 2.2 verdict |
| R7 | **observation is at tick boundaries, on an immutable root**. `Observe` reads `ModelRoot` and returns findings only. `Restrict` turns findings into bans, and runs only as an `Approved` ruler. Neither can write the model | perturbation by observation; looking passed off as ruling | §5 |
| R8 | **restricted runs:** deterministic (RR1); for a fixed ban, the least model under the ruler, with semi-naive = naive (RR2); sound, `M_R ⊆ lfp`, withholding never inventing (RR3); late bans reported (RR5); recoverable, `lfp(P, M_R ∪ withheld) = lfp(P, F)` (RR6) | restricted runs that can't be audited or undone | §5 |
| R9 | **pacts never change meaning in the positive core** (RR3). In W-programs every restriction is reported as status moves (Prop 5.3) | **the shear**: policy changing meaning without a receipt | §5 |
| R10 | **bootstrap:** T1 stage 2 runs with the observation pacts on, and asserts that their erasure on `mirror.mirror` is empty | a pact silently reshaping the parser's reading of itself | Prop 6.1 |
| R11 | **revocation is a record.** A revoked ruler's past results keep its address and are marked, never rewritten | history edited by policy | — |

## §3 Today's rulers

Each row of math §7 becomes a `Ruler` with a kind and declared reader dependence: the prism thresholds, Kuramoto parameters, budgets (`k_max`, E's budget, rotation ticks), the `active` quantizer (system-wide), prism `Precision`, NFC, the μ number types, the gate, convergence auto-approval, the similar-offer radius (reader-dependent), `Futures` weights (declared per ruler), and functional declarations. **To re-check against Taut's inventory** (`docs/scouts/2026-10-29-taut-ruler-inventory.md`, not landed at writing).

The existing `@epistemologic/pact` family is a depth-1 tree: `algebra.mirror` requires fifteen leaves. Under R6 it reports `Hierarchy` or `Kept`, never `Polycentric`, until a cross-check exists.

## §4 Red tests

- **RU1 (no bare narrowing).** A `compile_fail` doctest: `run_bounded(&inst, 100u64)` and any threshold taking `f64` don't compile. With an `Approved` ruler they do, and the result's `shaped_by` contains the ruler's address.
- **RU2 (the coda's comma, both ways).** A `Weight` ruler declared `Independent`: `weigh` then `arrive` equals `arrive` then `weigh`. Declared `Dependent { reader: r }` with weights that differ for r₀ and r: the two orders differ, and the result says which reader it is for.
- **RU3 (shape is computed).**
  - a star of rulers that books every comma up → `Hierarchy`;
  - the same star with one comma `LeaveOpen` → `Kept`;
  - three rulers checking each other in a triangle with non-trivial holonomy → `Polycentric`, with a ruler comma that no single ruler can absorb (the gauge solve fails).
- **RU4 (approval chains).**
  - a ruler approved by itself only → `Unapproved`;
  - a cycle of two rulers with a human signature in it → `Approved`;
  - a chain ending in a human signature → `Approved`;
  - revoking a ruler → past results keep its address, marked revoked.
- **RU5 (restricted run, positive core).**
  - E1's chain with a fixed ban on one `p` atom: the result is `lfp(T^B)` and semi-naive equals naive on it;
  - `M_R ⊆ lfp`;
  - `lfp(P, M_R ∪ withheld) = lfp(P, F)`;
  - the PEG grammar `S <- 'a' / 'b'` on `a` with `m(alt₁, 0, 1)` banned: `(S, 0)` becomes undefined, and never `m(alt₂, 0, 1)`.
- **RU6 (late ban).** Rules `p :- a`, `q :- p`, fact `a`, and a ruler that bans `p` once `q` is seen: `p` stays, and `late_bans` contains `(p, 1, ruler)`.
- **RU7 (W restriction reported).** `p :- not q` with `q` derivable; banning `q` gives `StatusMove { p: False → True }`, reported, never silent.
- **RU8 (bootstrap).** T1 stage 2 with the observation pacts on: equal `root`, and `withheld` empty on `mirror.mirror`. A deliberately aggressive test pact that bans one parse item makes stage 2 red.

## §5 Open

- Unbookability when rule groups differ per check edge (math: OPEN).
- Whether a human in every governance cycle blocks Ross-style self-erasure (CONJECTURE).
- The mapping of §3 against Taut's inventory.
- Where Rust-side pact enforcement ends and shard-side pacts begin: R1–R11 are API laws, and the pacts' content is shards evaluated by E (Seam's engine-scope audit governs what floor Rust may hold).
