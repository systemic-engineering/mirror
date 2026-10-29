# `settle` on a Fractal, and the `Loop` — type theory (spec)

*2026-10-29. Mara. Status: **Red**. Math: `docs/math/2026-10-29-mara-settle-on-a-fractal-and-the-loop.md` (cited "math A.n / B.n"). Composed with Scarlet in `docs/essays/2026-10-29-mara-scarlet-metalogue-settle-loop.md`. Builds on `docs/specs/mirror-fixpoint-parser.md` §4.2 (newtypes), §4.3 (`Frequency`, `Orbit<K>`).*

Alex's rules apply throughout: **no bare types**, and **make illegal states unrepresentable**. Each law below says which illegal state its type removes.

---

## §1 The types

### §1.1 Exact groups

```rust
/// A group whose identity test is exact. There is no impl for f64 or f32,
/// and there never will be (law L3).
pub trait LoopGroup: Clone + Eq {
    fn identity() -> Self;
    fn compose(&self, other: &Self) -> Self;
    fn inverse(&self) -> Self;
}
```

Instances. The substrate already had the words: `shards/epistemologic/math/music/harmonic.mirror` declares `type ratio` ("in lowest terms") and names "the Pythagorean comma as holonomy" (`comma_as_holonomy`), and `interval.mirror` declares the ℤ₁₂ quotient (`pitch_class`). The Rust types realise those carriers; they don't rename them.
- `Ratio`: `harmonic.ratio`, an exact positive rational, encoded as a sparse vector of prime exponents (3/2 = `[−1, 1⟩`; the xenharmonic name for this encoding is *monzo*).
- `PitchClassRatio`: `Ratio` modulo ⟨2⟩, with the exponent of 2 dropped.
- `Z<const N: u32>`: ℤ/N. `Z<12>` is `interval.pitch_class`, the tempered clock; `Z<2>` is polarity.
- `Rename`: a finite permutation of names (kintsugi mends).

`Ratio` and `PitchClassRatio` compare by exponent vectors, so equality is exact, and the identity is "all exponents zero".

*Drift noted, not edited:*
- `interval.mirror:221` declares `type pitch_class = u32`, a bare type by Alex's rule; it wants `Z<12>`.
- `harmonic.mirror:57` speaks of kintsugi that "reduces holonomy". Under L3, kintsugi records mends and never reduces a holonomy by averaging.

### §1.2 `Loop` and `Holonomy`

```rust
pub struct Loop<G: LoopGroup> { base: Vertex, edges: NonEmpty<Step<G>> }
impl<G: LoopGroup> Loop<G> {
    /// Ok only if the path is non-empty and returns to `base`.
    pub fn new(base: Vertex, edges: Vec<Step<G>>) -> Result<Self, NotClosed>;
    pub fn holonomy(&self) -> Holonomy<G>;
}

pub enum Holonomy<G: LoopGroup> { Identity, Comma(NonIdentity<G>) }

/// A group element that is not the identity. The only constructor
/// returns None for the identity.
pub struct NonIdentity<G>(G);
impl<G: LoopGroup> NonIdentity<G> { pub fn new(g: G) -> Option<Self>; pub fn get(&self) -> &G; }
```

### §1.3 Orbits are loops

`Orbit<K>` is as in fixpoint §4.3: `Fixed { at }`, `Cycle { from, period }` (with `Period ≥ 2`; unconstructible when `K = Monotone`), `Unknown { window }`. Its relation to `Loop` (math B.7):

```rust
impl<K: OrbitKind> Orbit<K> {
    /// The holonomy of a Cycle's lift when each state carries a label in G.
    /// Fixed: the length-1 loop. Unknown: None (no loop has been seen).
    pub fn lift<G: LoopGroup>(&self, label: impl Fn(Tick) -> G) -> Option<Holonomy<G>>;
}
```

### §1.4 Fragments, crystals, the Fractal

```rust
pub enum FragmentState {
    Fractured(NonEmpty<Finding>),          // the cone doesn't glue (kintsugi comma, gap, claim)
    Flux(Flux),
    Settled(Crystal),
}
pub enum Flux {
    Cycle { from: Tick, period: Period, phases: NonEmpty<Addr> },  // phases.len() == period
    Dark { window: Budget },                                         // Unknown: the window was too short
}

/// Only `settle` builds a Crystal, from a fragment whose whole cone is
/// settled (law L4). No public constructor.
pub struct Crystal { addr: Addr, uuid: SpectralUuid }

pub struct Fractal<T> { /* fragments keyed by Addr, dependency edges, SCC condensation */ }
```

### §1.5 `settle`

```rust
#[must_use = "the erasure records what settling discarded (Förster: no narrowing without a named ruler)"]
pub struct Settled {
    pub crystals: Fractal<Crystal>,
    pub flux: Vec<(Addr, Flux)>,              // left open: offered, asked about, not answered
    pub fractured: Vec<(Addr, NonEmpty<Finding>)>,
    pub erasure: Erasure,                     // every phase not chosen, with the ruler that chose
}

pub fn settle(
    fractal: Fractal<Fragment>,
    rulers: &Rulers,           // named, signed @ruler references; may be empty
    window: Budget,
) -> Settled;
```

- With no ruler covering a `Cycle`, the fragment stays `Flux`. `settle` never picks a phase on its own.
- A `Ruler` is an `Addr` of a signed `@ruler` crystal, never a closure or a number (fixpoint §10).
- `Erasure` entries are `(fragment addr, chosen phase addr, discarded phase addrs, ruler addr)`.

### §1.6 Dark decides, active suggests

- `Crystal`, `Flux::Cycle` detection and every recurrence test compare `Addr` (the dark side).
- `SpectralUuid`'s active half has **no `Eq` used by `settle`**. It appears only in:
  ```rust
  pub struct Suggestion { other: Addr, distance: ActiveDistance, ruler: Addr }
  pub fn similar(&self, other: &Crystal, radius: &Ruler) -> Option<Suggestion>;
  ```
  A `Suggestion` can be offered to a person (convergence §4). Nothing converts a `Suggestion` into a recurrence, a fixation, or an applied resolution.

---

## §2 Laws

| # | law | the illegal state it removes | math |
|---|---|---|---|
| L1 | `Holonomy::Comma(x)` implies `x ≠ identity` | a "comma" that is closed | B.1 |
| L2 | holonomy is gauge-invariant up to conjugacy (exactly invariant for abelian G) | a closedness verdict that depends on the choice of potentials | B.2 |
| L3 | **never temper**: no `LoopGroup` for floats, and no function `Holonomy<G> → Holonomy<G>` sends `Comma` to `Identity`. The only route from an open loop to a decision is `settle` with a named ruler, recorded in `Erasure` | averaging a comma away (fusion) | B.4 |
| L4 | a `Crystal` exists only for a fragment whose whole cone is settled; the settle order is the SCC condensation (recursive between, circular inside) | a crystal built from a fragment that only looked still for a tick | A.3, A.4 |
| L5 | where dependencies are contained (Merkle), an equal address on consecutive ticks means settled; where they are named, only the SCC order decides | trusting a named reference's unchanged address | A.5 |
| L6 | fixation is lossless restriction: settled fragments are served from cache by `Addr`, with identical trajectories; an edit unsettles exactly its up-cone | re-evaluating what is known, or caching what isn't settled | A.6 |
| L7 | a `Cycle` settles only through a named ruler, and the erasure lists the `period − 1` phases not chosen | an anonymous choice of phase | A.7 |
| L8 | monotone kinds have no `Cycle` (`Orbit<Monotone>`); under W a component with a negative cycle leaves its period-2 atoms as `Flux::Cycle { period: 2 }` | a "cycle" in a positive program; an undefined atom silently read as false | freq F4, F6; A.9 |
| L9 | recurrence is decided by `Addr` only; active bits only make `Suggestion`s | a false Fixed/Cycle between cospectral states | A.5 |
| L10 | `Orbit::lift` of a `Fixed` is `Identity`; a `Cycle`'s lift may be a `Comma` (the orbit closes in the quotient and may stay open in the lift) | confusing "the pitch class came back" with "the pitch came back" | B.7 |

**Förster's imperative as a signature:** every narrowing (a choice among phases, a resolution applied) passes through a named `Ruler`, and the record of what was narrowed (`Erasure`) is returned and `#[must_use]`. A narrowing that isn't written down can't be expressed.

---

## §3 Red tests

Fixtures are data in the test, or mirror-text. No serde.

- **S1. The Pythagorean comma, exact.**
  - Twelve edges labelled `Ratio([−1, 1⟩)` (3/2) around one loop in `PitchClassRatio`. `holonomy()` is `Comma` with representative `531441/524288` (monzo `[−19, 12⟩`).
  - Six edges give `729/512`, and `(729/512)²` equals `2 · 531441/524288`.
  - A `compile_fail` doctest shows `impl LoopGroup for f64` doesn't exist and `NonIdentity::new(identity)` is `None`.
  - **Documentation case:** twelve `f64` fifths of `2^(7/12)` give `1.0000000000000002`. A tolerance test reports "closed" and an exact `==` reports "open". The test asserts both outcomes, to show why the exact type exists.
- **S2. Vector 04's period-2 atoms.** Through `settle` on the pact program, `p`, `q` and `t` end as `Flux::Cycle { period: 2 }`, `r` as `Settled`, and the erasure is empty (no ruler given).
- **S3. Locally unchanged ≠ settled (SCC).** u counts `0, 1, 2, 2`; v copies u's previous state (math A.3).
  - With v's dependency on u **named**: at tick 1, v's state is unchanged and its address equal, and `settle` still reports v as not settled. v becomes a `Crystal` only after u is settled.
  - With the dependency **contained**: v's address changes whenever u's does, and the address alone decides.
  - A two-member SCC settles as one: both `Crystal` at the same tick, or neither.
- **S4. Cospectral pair: no false recurrence.** Two fragments with equal active bits and different addresses (Laplacian-cospectral trees on 11 vertices, fixpoint T10). The sequence `A, B, B, …` is `Fixed { at: 1 }` by address. A test on active bits would report `Fixed { at: 0 }`, because A and B look equal, and would crystallise A, the wrong state. The test asserts the address verdict, that `similar(A, B)` is `Some(Suggestion)`, and that no `Crystal` or `Cycle` came from the suggestion.
- **S5. A cycle needs a ruler.** A period-3 component settles to `Flux::Cycle { period: 3 }` without rulers. With a ruler choosing phase 0 it becomes a `Crystal`, and `erasure` lists the other two phase addresses and the ruler's address. Dropping the `Settled` value without reading `erasure` is a compile warning (`#[must_use]`).
- **S6. Lossless restriction.** On the E1 fixtures, the trajectory with settled fragments served from cache equals the full trajectory, and a perturbation of one fragment unsettles exactly its up-cone.
- **S7. Orbit lift.** `+7 mod 12` on `Z<12>` from C: `Cycle { period: 12 }`. `lift` with labels 3/2 in `PitchClassRatio` gives the S1 comma; `lift` of a `Fixed` gives `Identity`.

---

## §4 Open

- **CONJECTURE (math A.10):** W component by component equals W on the whole program. Test: E5 vectors both ways.
- **Twist:** only the constant-group twist is a `Loop` (math B.7). The tree twist needs the gauge solve of kintsugi math Prop 3.6, a separate type (`GaugeProblem`), not specced here.
- **For Alex:** the name `Flux::Dark` (Reed's "Unknown → Dark") versus a separate fourth state. And `pitch_class = u32` → `Z<12>` in `interval.mirror` (the bare type).
- **For Seam:** L5's boundary between contained and named dependencies in mirror's normal form (is every intra-shard reference contained?), and S4's fixture.
