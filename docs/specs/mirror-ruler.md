# `Ruler` and the fractal of rulers (spec)

*2026-10-29. Mara (with Scarlet). Status: **Red**. Second version: folds Seam's crowbar (`docs/audits/2026-10-29-seam-crowbar-ruler-pact.md`, all 12 items of §9; marked *[S n]*) and Taut's inventory (`docs/scouts/2026-10-29-taut-ruler-inventory.md`; marked *[T]*). Math: `docs/math/2026-10-29-mara-ruler-and-pact.md` (cited "math §n"; §8–§9 hold the fold). Builds on `mirror-fixpoint-parser.md` §4.1–§4.3, `mirror-settle.md`, `mirror-coupling.md`.*

**The lines this spec holds** *[T]*:
- **meaning** lives in the clauses (grammar, `μ`, programs);
- **measurement** (what a reading reports: the Resolution family) is recorded beside the reading;
- **policy** (budgets, gates, approvals) lives in the fractal of rulers.

A choice that crosses into meaning is paid for with a `settle` and its `Erasure`.

**Naming** *[S 12; decision D5]*:
- `@epistemologic/pact/*` are **clauses** (predicate shards on the meaning side), not rulers. The math's earlier READING that they form a depth-1 fractal of rulers is withdrawn.
- `@pact/fact` is an engine altitude.
- The fractal of rulers needs its own name. Alex names it. Placeholder in this spec: **`RulerFractal`**.

---

## §1 One envelope, a body per family *[S 10, 11; T §4–§5]*

```rust
/// Every ruler, whatever its family.
pub struct Ruler<B: Body> {
    addr: Addr,                    // computed from parsed shard content; never passed in [S 8]
    reader: ReaderDependence,      // declared; admissible values per family (table below)
    body: B,
}
pub enum ReaderDependence { Independent, Dependent { reader: ReaderRef } }

/// Approval: only `verify` builds it [S 8].
pub struct Approved<'r, B: Body> { ruler: &'r Ruler<B>, by: ApprovalChain }
pub struct Revocation { ruler: Addr, by: ApprovalChain, at: Round }   // a record

/// Every shaped result names its rulers and the observers that fed them [S 9].
#[must_use]
pub struct Shaped<T, E> { value: T, shaped_by: NonEmpty<Addr>, observed_by: Vec<Addr>, erasure: E }

/// Sealed: the bodies below are the only ones [D3].
pub trait Body: sealed::Sealed { type Erasure; type Composite; }
```

| family | body | composes by | erasure (R2) | reader (R3) | comma between two rulers (R4) | shape (R6) | acts on E's model? |
|---|---|---|---|---|---|---|---|
| **Budget** | `Count { n: NonZeroU64, unit: Unit }` | `min`; commutative, idempotent | `Truncated { at: Tick }`; no atoms | always `Independent` | lawful composite (`min`); the overruled larger budget is recorded | semilattice: `Untested` / `Hierarchy` / `Kept` only | **as truncation, not a ban**: `RestrictedRun.truncated` [S 4] |
| **Choice** | an **ordered** list of criteria, each a positive program over held alternatives | sequence; **non-commuting; the order is the meaning** | the losing alternatives | `Independent` or `Dependent { reader }`, declared | **a fork**: two results, each naming its ruler (fixpoint §10) | holonomy possible (reorderings form a group): `Polycentric` meaningful | clause-side (D2/D6 lean); picking is a `settle` with erasure, recorded **in N** [S 3] |
| **Resolution** (measurement) | `Band { zero_below: Ratio, nonzero_from: Ratio, scale: ScaleLaw }`, verdict `Zero \| DontKnow \| NonZero` | coarsest: `(min zero_below, max nonzero_from)`, **within one `ScaleLaw` only** | the readings collapsed into `Zero` or `DontKnow` | `Dependent` by design (`settle.rs:47`) | lawful composite within a scale law; across scale laws a **fork** (OPEN: a common scale) | semilattice: no `Polycentric` | never; recorded beside the reading |
| **Equivalence** | `Partition` (a quantizer into classes) | lattice: meet = common refinement, join | the merged classes | **must be `Independent`** where identity is near (`active`: every issuer) | a fork in practice: a composite changes identity (fixpoint §10) | lattice: no `Polycentric` | clause-side (D6 lean) |
| **Authority** | `Conditions` (a positive program) + a signer | conjunction | the refused items | who signs (declared) | lawful composite (∧) | semilattice: no `Polycentric` | never; gates and approvals |
| **Identity root** *[T]* | `Root { name, signer }` | doesn't compose: one of each | — | `Independent` | none | — | never; revocable only upward |

`ScaleLaw = Absolute | TimesScale { floor: Ratio } | EpsilonScaled { k: NonZeroU64, dim: Dim }`. All six numerical zeros Taut found *[T §2.1]* become distinct, comparable rulers:
- `instrument.rs` `1e-9 × scale`;
- the commutator band `1e-12 / 1e-6 × max(‖·‖_F, 1)`;
- `spectrum.rs` `1e-12` absolute;
- `settle.rs` `16·ε·scale·n`;
- `tension.rs` `1e-9` absolute;
- `coincidence.rs` `f64::EPSILON` absolute.

**Not families** *[S 10]*:
- **Ban** is the mechanism by which a Choice-by-elimination reaches E (§3).
- **Weighting** narrows nothing. It is `Weights`, an input that a Choice consumes, carried under the Choice's envelope (the coda's comma lives on that Choice's `reader` field).

**Lattice families have no inverses, hence no holonomy** (math §8.2, PROVED). For Budget, Resolution, Equivalence and Authority, every set of rulers has a lawful composite, so every disagreement can be booked to that composite. Multi-truth there is only ever `Kept` (a declared refusal used two-sidedly). `Polycentric` means something only where adjustments are invertible: Choice (reorderings) and Temperament (`mirror-settle.md` §4).

---

## §2 Laws

| # | law (folded) | changed from v1 |
|---|---|---|
| R1 | **no narrowing parameter without a ruler.** Enforced three ways: (a) a convention plus review: every `pub fn` parameter named or documented as limit, budget, threshold, tolerance, depth, timeout, max or min has type `&Approved<'_, _>` (Taut's inventory seeds the grep); (b) `Ruler` has no public constructor except from parsed shard content, `Approved` none except `verify`; (c) the production `Keyring` holds public keys only, no private key lives under `rust/`, and test keyrings are `#[cfg(test)]` or `tests/` only. R1 binds mirror's API. Prism's thresholds (P1–P13, P16–P19) stay bare until prism adopts `Approved`, and are listed as outside its reach | *[S 8]* was type-only |
| R2 | every result carries `shaped_by` (ruler addresses) and `observed_by` (the observers that fed them). Its erasure has the family's erasure type (§1 table) | *[S 9, 11]* erasure per family |
| R3 | reader dependence is declared, and its admissible values are per family (§1) | *[S 11]* |
| R4 | **lattice families:** a comma is booked to the lawful composite (min, coarsest, ∧, meet), with the overruled value recorded. **Choice / Equivalence:** no comma inside one third; two results, a fork, each naming its ruler. **Sideways booking** (to a sibling) only as a recorded act approved by both parties (D4 lean) | *[S 11]* was "booked upward as a settle" for all |
| R5 | every approval chain ends in a terminal component containing a human signature, **and approval is non-transitive across ruler versions**: a ruler whose content changed (new `addr`) needs a fresh human signature if it is in, or approves anything in, a governance component. A ruler never approves its successor's relaxation (D1 lean) | *[S 7]* the "human in every cycle" conjecture is refuted |
| R6 | **shape is computed from capacity and use, per family:** (a) a check edge counts only if its rulers have different addresses and readings that can disagree (capacity); (b) `Untested` when no comma has occurred; `Hierarchy` needs at least one comma booked to a composite or upward; (c) `Kept` is computed from use: some result over the comma's atoms carries both verdicts, or none exists yet. A `LeaveOpen` comma used one-sidedly reports as booked to the side used; (d) `Polycentric` only for Choice-family loops with non-trivial holonomy | *[S 5, 11]* was b₁ of declared edges |
| R7 | **`Observe` reads, `Restrict` rules, and a `Restrict` names (by address, in its approved content) the observers it reads.** Findings from unnamed observers are invisible to it. Findings live at their own altitude, **outside `ModelRoot`**. Observers are pure and total: a fault is reported, never a stop | *[S 9]* |
| R8 | restricted-run laws, for the **ban mechanism only** (Choice-by-elimination, §3): RR1–RR6 as folded in math §5 | *[S 1, 2, 11]* |
| R9 | **on a Ford-functional program** (functionality checked first), a ban only withholds meaning. On a non-functional program (`split` grammars, kintsugi's "which one?"), a ban that leaves one of several parses is a **Choice settle**: its erasure lists the removed parses, and N records that the choice was made | *[S 3]* was "the positive core" |
| R10 | **bootstrap:** stage 2 runs with the observation rulers on. `N_on = N_off` holds **if** the erasure is disjoint from the start cone and nothing truncated the run (not "iff"). RU8 asserts two things separately (§4) | *[S 4]* |
| R11 | revocation is a record; identity roots are revocable only upward | *[T]* roots added |

---

## §3 The restricted run (the ban mechanism)

```rust
pub fn run_restricted(inst: &Instance, rulers: &[Approved<'_, Choice>], budget: &Approved<'_, Budget>)
    -> Shaped<RestrictedRun, Vec<Withheld>>;

pub struct RestrictedRun {
    model: ModelRoot,
    deposits: Vec<Deposit>,
    withheld: Vec<Withheld>,
    late_bans: Vec<(Addr, Tick, BTreeSet<Addr>)>,          // ban arrived after the deposit
    lifted: Vec<(Addr, Tick, Tick, BTreeSet<Addr>)>,       // withheld at t, ban gone at t′  [S 1]
    truncated: Option<(Tick, Addr)>,                        // a Budget stopped the run  [S 4]
    orbit: Orbit<Monotone>,
}
/// RR1's attribution: every ruler that banned the atom, sorted [S 2].
pub struct Withheld { atom: Addr, tick: Tick, by: NonEmpty<Addr> }
```

- **Sealed rulers (D3 lean).** `Restrict` is implemented only as "E evaluates the ruler's positive program over the snapshot (and its named observers' findings)". Then `B_t = R(M_t)` is monotone in `M_t`, which grows, so `B_t` grows and **nothing is ever lifted** (math §8.1, PROVED). `lifted` stays as a field for the open-trait alternative, where it is the report Seam showed was missing.
- **Choices in order.** Several Choice rulers in one fractal carry an explicit order. The run applies them **in sequence**, each to the survivors of the last, never by uniting their bans *[S 11; fixture 7]*.
- **W-programs.** `well_founded_restricted` reports every `StatusMove` against the unrestricted model (unchanged from v1).

---

## §4 Red tests

- **RU1 (no bare narrowing; R1).** `compile_fail`: `run_bounded(&inst, 100u64)`, an `f64` threshold, a `Duration` timeout. And a grep test over `rust/**/src` for parameters named limit/budget/threshold/tolerance/depth/timeout/max/min without `&Approved`.
- **RU2 (the coda's comma).** The `Weights` of a Choice declared `Independent` commute with arrival; declared `Dependent { reader }`, they don't, and the result names its reader.
- **RU3 (shape from capacity and use; R6).**
  - three copies of one ruler in a triangle → not `Polycentric` (no capacity);
  - a fresh fractal with no commas → `Untested`, never `Hierarchy`;
  - a `LeaveOpen` comma whose results all carry one side → reported as booked to that side;
  - a Budget triangle → never `Polycentric` (lattice);
  - three Choice rulers whose reorderings have non-trivial holonomy → `Polycentric`.
- **RU4 (approval; R5).**
  - self-approval → `Unapproved`;
  - human sink → `Approved`;
  - a human-signed governance cycle → `Approved`.
  - **Red twin [S 7]:** the same cycle, then a content change of one member with no fresh signature → `Unapproved` (Seam's G₀ → G₁ → G₂ fixture).
- **RU5 (restricted run, functional; R8, R9).** As v1, plus:
  - Ford functionality checked first;
  - two rulers banning one atom → `Withheld.by` holds both, sorted, and is identical under every permutation of `rulers`.
- **RU5′ (non-functional; R9) [S 3].** `S <- A B ; A <- 'a' | 'a' 'a' ; B <- 'a' 'b' | 'b'` on `aab`. A ban leaving one parse is a Choice settle: the erasure lists the other parse, and N records the choice.
- **RU6 (late and lifted; RR5) [S 1].**
  - Late: `p :- a`, `q :- p`, ban `p` once `q` is seen → `late_bans`.
  - Lifted (open-trait alternative only): `p :- a`, `q :- a`, ban `p` while `q` is absent → `lifted (p, 1, 2)`. Also asserts that naive and semi-naive differ there, so the evaluation strategy is recorded.
  - Sealed: the same ruler can't be written as a monotone positive program, and is refused.
- **RU7 (W).** Unchanged.
- **RU8a (observer contract) [S 4].** T1 stage 2 with the observation rulers on: `withheld` empty, `truncated` none.
- **RU8b (cone) [S 4].** A ban **inside** the start cone changes `root`. A ban **outside** it (`f(T, 0)` for an unused rule T) doesn't.
- **RU9 (Choice order) [S 11].** Seam's fixture 7: two parses x, y. Choice A keeps the least name, B keeps the heavier. In sequence A then B gives x, B then A gives y. Uniting their bans would give a hole, and is refused.
- **RU10 (R7 one level up) [S 9].** An approved Restrict reading an **unnamed** observer's `suspect(x)` finding bans nothing. Naming the observer in its approved content makes it ban `x`, and `observed_by` lists the observer. Findings never appear in `ModelRoot`.
- **RU11 (Resolution) [T].** The six numerical zeros as six `Band`s. The commutator band reports `DontKnow` on `1e-9`. `tension.rs`'s ruler reports `Zero` there. The two disagree, and with different scale laws the result is a fork, never a silent pick.

---

## §6 Temperaments and intonations (Alex, 2026-10-29)

Math: math §10. Alex: "A single-truth is what makes it a useful CI pipeline. A multi-truth ought to be protected and may never be collapsed into a single-truth ruler."

- **Temperament (single truth):** a ruler in a lattice family (§1). When totally ordered it is a scalar metric. Gates read these.
- **Intonation (multi-truth):** a connection on the observer graph, i.e. exact transports between frames. It has no value per reading, and its holonomy is its comma.
- **Vector ruler (the middle):** a lattice, plural while unweighted (a Pareto front). Weighting it is a Choice, i.e. a temperament act.

> **Reshaped 2026-10-29 (Alex: "Isn't a `Ruler` a `Prism`, and then the whole distinction dissolves? A single-truth ruler settles to a single scalar value. A multi-truth ruler to a field."; math §11).** One trait. Every ruler is a prism whose `settle` yields a `Field<B, V>`. The single/multi distinction moves to the field's base and value algebra:
> - point: a scalar;
> - lattice-valued: lawfully collapsible by meet;
> - flat group-valued: collapsible up to gauge;
> - **curved group-valued: never collapsible, only temperable**.
>
> `Intonation` is now `Field<Graph, G>` with G a group, and it **is** a ruler ("Fifty" closed, math §11.2). I5 is replaced below. I1–I4, I6 and IN1–IN5 hold with the renaming.

```rust
/// A value on each cell of a base. A scalar is a field over one point.
pub struct Field<B: Base, V> { base: B, values: Vec<(Cell, V)> }
pub struct Point;                                         // K₁: the decree's base
pub struct Graph { /* observers as vertices, lines as edges; b₁ computed */ }
pub struct Discrete<const K: usize>;                      // k criteria, no lines between them
pub type Scalar<V> = Field<Point, V>;
pub type Intonation<G> = Field<Graph, G>;                  // G: LoopGroup, transports on lines
pub type Front<const K: usize> = Field<Discrete<K>, Ratio>;

/// One ruler trait: a prism (focus, project, settle) whose settled value is a field.
/// Provenance (addr, reader, approval) is the envelope of §1; `Body` stays sealed.
pub trait Ruler: Prism<Refracted = Optic<Self::Projected, Field<Self::Base, Self::Value>>> {
    type Base: Base;
    type Value;
}

/// Flatness is a runtime fact with a type-level witness.
pub struct Flat<G>(Intonation<G>);                        // only from `check_flat`
impl<G: LoopGroup> Intonation<G> {
    pub fn transport(&self, path: &Path) -> G;
    pub fn holonomy(&self, l: &Loop<G>) -> Holonomy<G>;
    pub fn ruler_free(&self) -> RulerFree<G>;            // its gauge class
    pub fn check_flat(&self) -> Result<Flat<G>, Curved<G>>;
}
impl<G: LoopGroup> Flat<G> {
    /// Lossless up to gauge: the gauge class of a flat field is one point.
    pub fn gauge_point(&self) -> Shaped<Scalar<GaugeClass>, Gauge<G>>;  // erasure: the gauge (who translates how)
}
// Deliberately absent: any From/Into from Field<Graph, G> to Field<Point, _>
// (flat or curved); PartialOrd/Ord; Sum; score/as_f64; numeric Display.

/// Temper: the only bridge from a curved field. Borrows, never consumes.
pub fn temper<G: LoopGroup>(i: &Intonation<G>, t: &Approved<'_, Choice>)
    -> Shaped<Flat<G>, Curvature<G>>;                      // #[must_use]; erasure = the curvature
pub struct Curvature<G> { commas: Vec<Holonomy<G>> }       // around a cycle basis; empty iff flat

/// A vector ruler stays a front until a declared weighting collapses it.
pub fn weigh<const K: usize>(f: &Front<K>, w: &Approved<'_, Choice>) -> Shaped<Ratio, Front<K>>;

/// CI gates read temperaments and the kept commas. No field can hold an Intonation.
pub struct GateInput<'a> {
    temperaments: &'a [Shaped<Scalar<Verdictable>, ErasureAny>],   // point-fields only
    kept: &'a [KeptComma],                    // commas between voices: "a question, not a red build"
}
pub fn gate(input: &GateInput<'_>, ruler: &Approved<'_, Authority>) -> Shaped<Verdict, Vec<KeptComma>>;
```

**Laws.**
- **I1 (no collapse by conversion):** `Intonation` has no conversion to a scalar, number, order or `Ruler`. Backed by math Prop 10.4: no structure-preserving collapse exists, so every collapse is a Choice.
- **I2 (`temper` borrows):** `temper` takes `&Intonation`. The intonation is unchanged and stays addressable after any number of temperings. The result names its temperament (`shaped_by`), and its erasure is the averaged-away comma (for abelian real Γ, the harmonic part, math Prop 10.6).
- **I3 (vector rulers):** a `Front` collapses only through `weigh` with an approved Choice. The erasure is the front itself, so the collapse is reversible by reading the erasure.
- **I4 (gates):** `GateInput` holds temperaments and kept commas only. A gate's verdict is single-truth; the kept commas ride alongside, unchanged, in its result.
- ~~**I5 (provenance without narrowing):** … not a `Ruler<B>`, because it narrows nothing.~~ **I5′ (one trait):** every ruler is a prism settling to a field. An intonation is a ruler whose field lives on a graph with group values. It narrows *possibilities* (which connection holds), not readings. ("Fifty" closed, math §11.2.)
- **I7 (the codomain decides):**
  - `Field<Point, _>` is a scalar;
  - a lattice-valued field collapses by meet, and picking among several lawful collapses is a Choice;
  - a flat group field collapses up to gauge through `Flat::gauge_point`, with the gauge as erasure;
  - a curved group field collapses only through `temper`, with a non-empty `Curvature` as erasure (math §11.3).
- **I8 (temper vs an edge settle):** for abelian real G, `temper` keeps the gradient part and erases the harmonic part. An edge-Laplacian settle keeps the harmonic part and fades the gradient part. They are complementary, so tempered field + curvature = the original (math §11.4). prism's `spectral::settle` is vertex-only today, so this pairs `temper` with a settle prism hasn't landed.
- **I6 (the ruler-free third):** `ruler_free()` returns the intonation's gauge class: holonomy for constant Γ, non-abelian H¹ for per-line groups. On a tree it is `Absorbable`, while the transports still exist.

**Red tests.**
- **IN1 (`compile_fail`):** `let x: f64 = intonation.into();`, `intonation.score()`, `intonation < other` and `let r: Ruler<Budget> = intonation.into();` all fail to compile.
- **IN2 (temper keeps the comma):** a 12-edge fifths intonation in `PitchClassRatio`.
  - `temper` with an approved equal-temperament Choice returns a `Temperament` whose erasure holds the Pythagorean comma `[−19, 12⟩`;
  - `intonation.addr` and `intonation.holonomy(circle)` are unchanged afterwards;
  - tempering twice with two different Choices gives two results, each naming its own Choice.
- **IN3 (vector ruler):**
  - a three-point `Front<2>` with two incomparable maxima stays a `Front` through every operation except `weigh`;
  - `weigh` without an `Approved<Choice>` doesn't compile;
  - with one, the result names it and its erasure is the full front.
- **IN4 (the gate can't read an intonation):**
  - `compile_fail`: constructing `GateInput` with an `Intonation`;
  - a gate over two temperaments and one kept comma passes or fails on the temperaments alone, and returns the kept comma unchanged.
- **IN5 (tree vs loop):** an intonation on one wire has `ruler_free() == Absorbable` and a non-identity `transport`; on a triangle with non-trivial holonomy, `Classes([Comma])`.
- **FP1 (a point-field is today's scalar):** a `Scalar<Budget>` ruler passes every RU test that a `Ruler<Budget>` passed: same composite (min), same truncation erasure, same `shaped_by`.
- **FP2 (flat tempers losslessly):** a triangle intonation in `Z<12>` with holonomy identity. `check_flat` is `Ok`; `temper` returns an empty `Curvature`; `gauge_point` returns the trivial class, with the transports in its erasure.
- **FP3 (curved tempers only with the comma):** the 12-fifths intonation in `PitchClassRatio`. `check_flat` is `Err(Curved)`; `temper` returns `Curvature` = `[−19, 12⟩` (non-empty); the intonation is unchanged afterwards.
- **FP4 (`compile_fail`):** `let s: Scalar<Ratio> = intonation.into();` and `let s: Scalar<_> = flat.into();` both fail to compile. The only routes are `temper` and `gauge_point`, each returning an erasure.
- **FP5 (temper + settle = original, abelian real):** on a 4-cycle with real edge values, the tempered flat field plus the harmonic part (computed by the test oracle's own edge-Laplacian projection, in rational arithmetic) reconstructs the input field exactly. prism's vertex-only `settle` is not used.

## §5 Open decisions (Reed's lean, pending Alex; the spec works either way)

*D2 and D6 are decided by §6. D1, D3, D4, D5 and D7 stay open.*

| # | decision | Reed's lean | the alternative, and what changes |
|---|---|---|---|
| D1 | approval across ruler versions | non-transitive: each new version in or approving a governance component needs a fresh human signature; a ruler never approves its successor's relaxation | transitive ("someone once signed the root"): R5 reverts to reachability, and RU4's red twin becomes green, which means a ruler can drop its own constraint |
| D2 | is Choice meaning? | **DECIDED (Alex, 2026-10-29, via §6):** picking a Choice is a temperament act (clause-side, a `settle` with erasure, recorded in N); the un-picked structure is an `Intonation` | — |
| D3 | sealed traits | sealed: ruler behaviour lives in mirror (E evaluates positive programs); Rust checks only the envelope | open traits: `lifted` becomes a live report, non-monotone laws need their own audit, and Seam's engine-scope audit doesn't cover it |
| D4 | sideways booking | allowed only as a recorded act approved by both parties | forbidden (R4 lists only composite or fork), or a fourth shape |
| D5 | the word "pact" | stays with the clause shards (`@epistemologic/pact`) and the engine altitude; the fractal gets a new name (Alex names; placeholder `RulerFractal`) | `Pact = RulerFractal`: rename the clause shards and the altitude |
| D6 | Choice and Equivalence clause-side | **DECIDED (via §6):** yes, clause-side; what they choose among stays an `Intonation` | — |
| D7 | the gate's own checks (`.githooks/commit-msg`) | the cited audit file must exist in the commit's tree, with the cited section; `[bugfix:restore]` names the guarantee it restores and its restoring test; a Seam trailer requires Seam's own signature (a derived Seam key) | the regex-only gate stays: R1's force then rests on review alone, and every bare constant can be changed by a self-authorized `[bugfix:restore]` |
