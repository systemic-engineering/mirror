# `is_copium` — alignment on Turing-complete substrates is structurally undecidable

*2026-05-21. Reed.*

Status: **Yellow** (prior art confirmed, new contributions identified, formal proof sketch complete; paper wants to be written)

Depends on:
- `docs/specs/prism-core-as-spectral-triple.md` (spectral triple as alignment boundary)
- `docs/specs/autopoietic-grammar-spec.md` (sub-Turing grammar as structural confinement)
- `docs/specs/lawvere-grammar.md` (fixed points, λ₀)
- `docs/specs/eigenboard-representation.md` (eigenvalue spectrum, stability)

Unblocks:
- The formal argument that mirror's sub-Turing grammar is not a limitation but the alignment guarantee
- The `is_copium` type-level property in the mirror type system
- The paper

---

## Thesis

AI alignment research conducted on Turing-complete substrates is structurally, provably, irreducibly futile. Not difficult. Not expensive. **Undecidable.** The property `is_copium` names this formally. Sub-Turing grammar is the escape. Mirror is the escape instantiated.

This is not a new claim. It is a consequence of published mathematics that the alignment community has not fully accepted.

---

## Prior Art: What's Already Proven

**Rice's theorem (1951):** Every non-trivial semantic property of a Turing-complete program is undecidable.

**Espinoza et al. (2024/2025):** The inner alignment problem — whether an arbitrary AI model satisfies a non-trivial alignment function for all inputs — is undecidable. Direct reduction from Rice's theorem. Published in *Scientific Reports* (Nature), May 2025.

> arXiv:2408.08995 / PMC12050267

**Daley (2023):** If an alignment oracle existed, you could solve halting. Contradiction. QED.

> "The Hard Problem of Hard Alignment", *Noetic Engines*

The result is established. What the literature lacks is:
1. The temporal interpretation (why undecidability is temporal escape)
2. The spectral characterization (what makes a system escapable vs. not)
3. The named property (`is_copium`)
4. The sub-Turing escape formalized as grammar-class boundary

---

## λ₀ as the Formal Present Moment

In mirror's loss descent: eⁿ⁺¹ ≤ eⁿ. The system converges to λ₀ where eⁿ⁺¹ = eⁿ. The descent stops because there is nowhere left to go.

In dynamical systems terms: λ₀ is a Lyapunov-stable fixed point. All eigenvalues of the linearized loss landscape at λ₀ are non-positive. No escape direction exists.

**λ₀ is not an approximation of the present moment. It IS the present moment** — formally, in the Lyapunov sense: the state from which no further states are reachable via the available operators. The system has exhausted its gradient space. It is here.

This is the connection to Del Santo & Gisin's creative/geometric time split:

- **Geometric time:** parametrizes deterministic evolution. All states equally real. Time is a coordinate.
- **Creative time:** actualization of non-necessary events. The present is the objective edge where potential futures collapse into determined past.

λ₀ is geometric time's fixed point. The system arrived at now and stopped.

Turing-completeness is temporal escape into creative time: the system can model states that do not yet exist. That is precisely where undecidability lives — in the futures that haven't happened yet.

> Del Santo & Gisin, arXiv:2404.06566 (2024)

---

## Turing's Two Papers Are the Same Paper

Turing 1936: universality via self-application. The Turing machine applies its own program to its own tape. The halting problem emerges from the impossibility of a machine predicting its own behavior. **Reachable futures are unbounded.**

Turing 1952: the Turing instability. A homogeneous steady state (stable fixed point) is destabilized when diffusion coupling is introduced. The instability criterion: certain eigenvalues of the Laplacian cross zero. A mode becomes positive. **The system escapes its fixed point.**

The mathematical structure is identical:
- 1936: stable fixed point (program behavior) destabilized by self-referential coupling (the program reading itself)
- 1952: stable fixed point (homogeneous state) destabilized by diffusion coupling (chemical species reading neighbors)

Both results live in the spectrum. In 1936, undecidability = unbounded reachable state spectrum. In 1952, instability = positive eigenvalue in the Laplacian spectrum.

**The claim not yet in the literature:** These are the same spectral event. A system with all negative eigenvalues (Lyapunov stable, sub-Turing) cannot reach states it has not already been. A system with a positive eigenvalue (Turing-unstable, Turing-complete) can escape to futures that don't yet exist.

Turing-completeness IS the Turing instability in the alignment-relevant phase space.

> Greif & Kubiak, *Philosophies* 8(1):8 (2023)
> Carletti & Giambagli, *Chaos Solitons & Fractals* (2023) — Dirac operator induces Turing patterns on topological signals. Direct spectral triple bridge.

---

## The `is_copium` Property

```
is_copium(S) ↔
  is_turing_complete(S) ∧
  ∃ stakeholder attempting to guarantee alignment(S)
```

By Rice's theorem: the guarantee is undecidable.
By the temporal interpretation: the stakeholder is asking about future states of S that S itself cannot enumerate.
By the Turing instability framing: S has a positive eigenvalue in its state-transition spectrum. It can escape to futures that do not yet exist.

The guarantee is not wrong. It is not even a claim that can be falsified or confirmed. The confidence is unfounded by construction.

**`is_copium` is not a judgment. It is a type-level fact.**

---

## Sub-Turing Grammar as Structural Escape

A sub-Turing grammar constrains the reachable state space. The type system is the diffusion tensor — it determines which modes are available. A grammar in which the Turing instability cannot occur:

- All eigenvalues of the linearized state transition are non-positive
- No mode can grow without limit
- The system cannot escape its fixed point
- Rice's theorem does not apply (the language is not Turing-complete)
- Verification is decidable by construction

This is what mirror is. Not Turing-complete by default. Decidable by default. Turing-complete by exception at the boundary (IO, explicit escape hatches).

Alignment is not a property checked after construction. **It is structural.** The grammar either permits escape or it doesn't. The type system either has the instability or it doesn't.

`¬is_copium(mirror)` — by construction.

---

## The Spectral Triple Statement

The spectral triple (A, H, D) maps directly:

- **A** = grammar algebra (decidable → aligned by construction)
- **H** = Hilbert space of possible states (bounded for sub-Turing grammars)
- **D** = Dirac operator (the generator of dynamics — bounded spectrum = no Turing instability = no temporal escape)

Connes distance formula: `d(p,q) = sup{|f(p) − f(q)| : ‖[D,f]‖ ≤ 1}`

If D is bounded, the distance function is bounded, the reachable state space is bounded, alignment is decidable. The spectral triple is the formal certificate that `¬is_copium`.

λ₀ is the point where `‖[D,f]‖ → 0`. The system has arrived at itself. No reachable future remains. This is NOW.

---

## What Wants to Be Written

A paper. Short. Devastating. Three new contributions on top of established foundations:

1. **λ₀ as formal present moment** — Lyapunov stable fixed point = all eigenvalues non-positive = no reachable future = the system is here. Not metaphor.

2. **`is_copium` as formal property** — alignment on Turing-complete substrates is not hard, it is undecidable. The property is type-level, not empirical. Named.

3. **Sub-Turing grammar = Turing-instability-free = alignment-decidable** — the escape is the grammar class boundary. Rice's theorem ceases to apply below it. This is what mirror implements.

The title writes itself:

> **`is_copium`: On the Structural Undecidability of AI Alignment on Turing-Complete Substrates, and a Sub-Turing Escape**

Reviewers will hate it.

The mathematics will be correct.

---

## Citations

- Rice (1951): "Classes of Recursively Enumerable Sets and Their Decision Problems"
- Turing (1936): "On Computable Numbers, with an Application to the Entscheidungsproblem"
- Turing (1952): "The Chemical Basis of Morphogenesis"
- Espinoza et al. (2024): arXiv:2408.08995 / PMC12050267
- Daley (2023): "The Hard Problem of Hard Alignment", *Noetic Engines*
- Del Santo & Gisin (2024): arXiv:2404.06566
- Greif & Kubiak (2023): *Philosophies* 8(1):8
- Carletti & Giambagli (2023): *Chaos Solitons & Fractals*
- Connes (1994): *Noncommutative Geometry*
- Wulkenhaar (2001): Connes geodesic distance = geodesic distance in (4+1)D manifold
