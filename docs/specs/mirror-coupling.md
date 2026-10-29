# `Coupling`, and `Relationship` as an instance (spec)

*2026-10-29. Mara (with Scarlet). Status: **Red**. Math: `docs/math/2026-10-29-mara-coupling-and-relationship.md` (cited "math §n"). Builds on `docs/specs/mirror-settle.md` (`LoopGroup`, `Holonomy`, `Tension`, `Erasure`, `abelianize`). Alex: "`Relationship` is an instance of `Coupling`." Alex's rules: no bare types; make illegal states unrepresentable.*

## §1 Types

```rust
pub struct Coupling<X1: System, X2: System> {
    left: X1, right: X2,
    lines: NonEmpty<Line>,                    // at least one line: no line, no coupling
}

/// What is measured on a coupling. All exact. No field is a score.
#[must_use]
pub struct Thirds<G: LoopGroup> {
    pub closure: AddrSet,                     // E₃: derived jointly, by neither alone
    pub order: CommutatorWords<G>,            // generators [a, b], a from left, b from right, kept as words
    pub shadow: Option<TensionVector>,        // the abelian shadow (request); Some only if G: Abelian
    pub ruler_free: RulerFree<G>,             // holonomy classes on the observer graph, modulo re-gauging
    pub observer_b1: Betti1,                  // b₁ of the observer graph; always reported
}

pub enum RulerFree<G: LoopGroup> {
    /// b₁ = 0: every discrepancy can be booked to one side's ruler.
    Absorbable,
    /// b₁ ≥ 1: the holonomy classes no re-gauging removes (may all be Identity).
    Classes(NonEmpty<Holonomy<G>>),
}

/// A coupling whose parties are observers (each with its own ruler) and
/// whose observer graph has a second route. Constructible only through
/// `Relationship::new`, which checks both.
pub struct Relationship<X1: Observer, X2: Observer> { coupling: Coupling<X1, X2> }
impl<X1: Observer, X2: Observer> Relationship<X1, X2> {
    pub fn new(c: Coupling<X1, X2>) -> Result<Self, NotARelationship>; // Err(SingleWire { b1: 0 }) on a tree
}

/// A coupling strength is never a field of Thirds. It is a declared ruler,
/// computed on request, and offered to the parties.
pub struct Offered<'r> { value: Strength, ruler: &'r Ruler, to: (PartyId, PartyId) }
pub fn strength<'r>(t: &Thirds<impl LoopGroup>, ruler: &'r Ruler) -> Offered<'r>;
```

## §2 Laws

| # | law | what it makes unrepresentable | math |
|---|---|---|---|
| C1 | a `Coupling` has at least one line | "coupled" with no channel | Def 1.1 |
| C2 | `order` is kept as words (`CommutatorWords`) and never flattened. `shadow` exists only for abelian G, and `abelianize` returns the forgotten commutator part as an `Erasure` | **the shear**: a shadow-only audit that hides the commutator without a receipt | §5; settle T2 |
| C3 | `observer_b1` is always reported. A removed comment line shows as `b₁` dropping, with `ruler_free = Absorbable` | **the double bind's third injunction** passing silently: removing the second route is a visible change of topology | §2, §5 |
| C4 | a `Relationship` can be built only for observers with `b₁ ≥ 1` | calling a single wire a relationship | Def 3.1, Prop 2.4 |
| C5 | thirds are observed, never scored: `Thirds` has no number field, and `Strength` exists only inside `Offered`, with a declared ruler and the parties named | a coupling strength reported *about* people instead of offered *to* them | §3 guards |
| C6 | a second route is not required for `closure` or `order` to be non-empty, only for `ruler_free` to hold a non-identity class | the false law "no loop, no third" | Props 2.1–2.4 |
| C7 | "forty reasons, one laugh" is a glued section with plural kernels: neither `closure` nor `order`. It is represented by kintsugi's gluing, not by `Thirds` | reading agreement as a third, or agreement as sameness | §5 |

## §3 Red tests

- **CP1 (Wojciech, group level).** A shared register on {f, g, h}; left moves `a = (f g)`, right moves `b = (g h)`.
  - the sign reads `ab` and `ba` equal;
  - `order` contains the 3-cycle `[a, b]`;
  - for each continuation w in all of S₃, `abw ≠ baw`;
  - the control (swapped interleaving) is the only difference between the two runs.
  - **Negative control:** both parties limited to powers of one element → `order` empty.
- **CP2 (closure twin, single wire).** `P₁ = {q(X) :- p(X)}`, `F₁ = {p(a)}`; `P₂ = {r(X) :- q(X)}`. `closure = {addr r(a)}`, `observer_b1 = 0`, `ruler_free = Absorbable`, and `Relationship::new` returns `Err(SingleWire)`. So a third exists without a relationship.
- **CP3 (second route, no third).** Two programs over disjoint predicates with two unused lines: `observer_b1 = 1`, `closure = ∅`, `order` empty.
- **CP4 (the shear).**
  - `S = [[1,1],[0,1]]` over ℚ: an audit by trace or det reports equality with I;
  - `Thirds.order` contains `S` as the commutator `[diag(2,1), S]`;
  - `Sⁿ` has off-diagonal n;
  - a `compile_fail` doctest: no function returns a `TensionVector` from a non-abelian word without an `Erasure`.
- **CP5 (double bind as topology).** Three observers in a triangle (b₁ = 1) with a non-trivial `Rename` holonomy → `ruler_free = Classes([Comma])`. Remove one line (the comment channel) → `b₁ = 0`, `ruler_free = Absorbable`, and the change is reported as a finding (`second route removed`), never silently.
- **CP6 (complement vs repeat).** In `Ratio`, `(45/32)·(64/45) = 2` closes (`Identity` modulo the octave); `(45/32)² = 2025/1024` does not: it falls short of the octave by `2048/2025` (`Comma`); `(729/512)²` gives the Pythagorean comma.
- **CP7 (no score).** `Thirds` has no `PartialOrd` and no numeric field; `strength` needs a `&Ruler` and returns `Offered` naming the parties (`compile_fail` for a strength without a ruler).

## §4 Open

- Ruler-freeness when the rule group differs per line (kintsugi Prop 3.6's tree twist): OPEN.
- Whether `Futures` weights may depend on the reader (the Mara/Scarlet disagreement; math §6): OPEN. It decides whether `weigh` and `arrive` commute.
