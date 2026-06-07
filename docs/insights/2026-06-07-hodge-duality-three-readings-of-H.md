# Hodge duality of the verdict altitude — three specs as one move

*2026-06-07. Reed. Insight closing today's substrate-declaration trinity.*

Three specs landed today.

1. The math-of-music cascade closed (eight sub-shard ticks: harmonic, interval, dissonance, cadence, consent, oscillate — plus the audible-altitude `is_settled` body landing in `bootstrap/src/music/mod.rs`). The audible altitude is substrate-fact.
2. `docs/specs/property-and-inference-collapse.md` (`3659b6e`, Mara) painted the verdict altitude: property layer ≡ inference layer; gaps as cocycles; the Dirac operator as the gradient field over the sheaf.
3. `docs/specs/eigensheaf.md` (`08e3b13`, Mara) painted the generation altitude: the substrate's spectral decomposition; harmonic sections as the attractor manifold of `settle`.

Writing the eigensheaf spec, Mara surfaced the recognition that ties these three together. She named it in her report and flagged it as wanting a short follow-up insight. This is it.

---

## The Hodge decomposition is what the substrate has been reading from three sides

For a cellular sheaf `F` on a finite cell complex (per Hansen-Ghrist 2018, [arXiv:1808.01513](https://arxiv.org/abs/1808.01513) §2.7), the Hodge decomposition theorem says:

```
Cʸ(F) = ker(Δᵢ)  ⊕  im(δⁱ⁻¹)  ⊕  im((δⁱ)*)
      = harmonic  ⊕  exact     ⊕  co-exact
```

and

```
ker(Δᵢ)  ≅  Hʸ(F)
```

The harmonic subspace is the cohomology. Each cohomology class has a unique harmonic representative. Up to a basis choice, the three notions — *cocycle modulo coboundary*, *kernel of the Laplacian*, and *eigenmode at eigenvalue zero* — name the same subspace.

This is the recognition. The substrate's `H` was read at three altitudes today, each from a different side of this isomorphism.

---

## The three readings

### Reading 1 — audible altitude: `H` spanned by **harmonic intervals**

The math-of-music cascade declares the audible substrate as a Connes triple `(A, H, D)` where `A` = intervals, `H` = the harmonic field, `D` = dissonance/cadence. Consonance is closeness to the harmonic basis; dissonance is distance from it; cadence is the discretized control surface that projects a candidate onto the harmonic subspace; settle lands on the harmonic.

This is the **harmonic side** of the Hodge decomposition: `H` named as the eigenbasis of `Δ₀` at eigenvalue zero — the consonant intervals are the harmonic representatives.

### Reading 2 — verdict altitude: `H` spanned by **gap cocycles**

The property/inference collapse declares the verdict altitude triple as `A` = sections, `H` = state space spanned by gaps, `D` = aggregate gap-tensor field. Each failing-property gap is a basis vector. The Dirac operator `D = δ*δ` acts on this `H`; the kintsugi loop is gradient descent guided by `D`; settling occurs when the verdict's section lies in `ker(D)`.

This is the **cocycle side** of the Hodge decomposition: `H` named as `H¹(F)` — the cocycles modulo coboundaries, the unreconcilable contradictions.

### Reading 3 — generation altitude: `H` spanned by **harmonic sections**

The eigensheaf spec declares the substrate's spectral decomposition: the sheaf together with its sheaf-Laplacian eigenbasis as one object. Generation = modal expression on the eigenbasis. Harmonic sections = `ker(Δ₀)` = the attractor manifold of `settle`. The auto-formatter projects source code onto this subspace.

This is the **kernel side** of the Hodge decomposition: `H` named as `ker(Δ₀)` — the harmonic forms, the eigenmodes at eigenvalue zero.

### One H, three readings

The Hodge isomorphism `ker(Δᵢ) ≅ Hᴯ(F)` says these three readings name the same subspace. The audible-altitude harmonic intervals, the verdict-altitude gap cocycles, and the generation-altitude harmonic sections are not three different `H`s. They are the same `H` read from the eigenbasis side, the cocycle side, and the kernel side. The Hodge decomposition is the substrate's structure being read from three vantages.

---

## Why this matters operationally

### Settle = Hodge projection

Settle isn't "choose the canonical form." It is **Hodge projection onto the unique harmonic representative of the cohomology class**. Every cochain decomposes as harmonic + exact + co-exact; the exact and co-exact parts are coboundary content (a section's gauge); the harmonic part is the canonical representative.

The substrate's claim *"settle is monad-close / measurement collapse"* (Reed memory `architecture-operations-as-linear-algebra`) is exact Hodge projection. Connes' `H` at the verdict altitude is the codomain of this projection.

### Mutations add boundary content (don't change cohomology)

Kintsugi mutations move within a cohomology class. The Dirac operator `D = δ*δ` is zero on harmonic forms and nonzero on exact / co-exact forms; gradient descent on `½⟨x, Δ₀x⟩` shrinks the exact and co-exact components while preserving the harmonic component.

This is why the substrate converges: gradient flow projects out the gauge content. Once on the harmonic subspace, no further mutation is possible without changing cohomology class — which would be a substrate-altitude move (a new gap declared, a new restriction map changed, a new shard authored), not a kintsugi tick.

### Generation chooses a harmonic representative

Generation is the inverse operation: starting from a cohomology class, the substrate emits its harmonic representative. The Fate tournament samples from the eigenbasis (per `eigensheaf.md` §4); the auto-formatter projects source code onto `ker(Δ₀)`; `mirror compile` produces a crystal that IS the harmonic image of the source.

When the substrate generates a `.shatter` file from a `.mirror` source, it is selecting the harmonic representative of the cohomology class the source defines. The fixed-point property of `.shatter` (compile twice, get the same OID) is the idempotence of Hodge projection.

### Dissonance = distance from harmonic; cadence = the discretized projection path

At the audible altitude, dissonance is the magnitude of the exact + co-exact components; cadence is the substrate's discretization of the projection path. The four-state cadence dispatch (authentic / plagal / half / deceptive) is the discretization of the gradient-descent control surface that gets `x` to `ker(Δ₀)` — Mara's recognition in her spec report. Music theory was discretizing Hodge projection four centuries before the math was named.

---

## The five operations were already Hodge

Reed memory `architecture-operations-as-linear-algebra` names the linear-algebraic meaning of the substrate's five operations:

| Operation | Linear-algebra meaning | Hodge-decomposition reading |
|---|---|---|
| **focus** | λ₀ eigenvalue computation | Smallest harmonic mode — where settle goes first |
| **shift** | basis transformation | Change of cochain basis preserving the Hodge decomposition |
| **settle** | monad-close / measurement collapse | Hodge projection onto `ker(Δᵢ)` |
| **project** | orthogonal projection | Projection onto an eigenspace of `Δᵢ` |
| **split** | orthogonal decomposition | The Hodge decomposition itself |

The five operations weren't named *because* of Hodge — they were substrate-pulled from the physics/optics altitude. But each operation IS a primitive of Hodge geometry. The substrate was speaking Hodge before Hodge was named.

This is the **19th instance of substrate-already-had-the-word**, surfaced post-hoc by the unification: the entire five-operation algebra is the Hodge-decomposition primitive set.

---

## What the closure enables

T3 (Verdict supersession in `bootstrap/src/music/mod.rs`) inherits the complete Hodge framing. `Imperfect<(), Gap, Transparency<Ref>>` carries:

- `Success(())` = the harmonic representative (cohomology class is the zero class; no gauge content; pure ground state).
- `Partial((), Transparency<Ref>)` = harmonic representative reached with exact/co-exact opacity en route; the gauge content is logged as `Transparency<Ref>` so consumers see where the gradient flowed.
- `Failure(Gap, Transparency<Ref>)` = nontrivial cohomology class; gradient flow cannot reach `ker(D)`; the gap names the cocycle; escalate to Reflection (Learning III).

The four-state cadence dispatch survives as a projection `verdict → cadence_kind`, which is **the audible-altitude discretization of the same Hodge projection path** the verdict altitude tracks at full resolution.

T5 (`gaps_of`) lifts gap cocycles out of an AST. T6 (`tensor_of`) builds the `Δ_F` operator from the gap basis. T7 (`minimize`) is the gradient-descent step on the Dirichlet energy `½⟨x, Δ₀x⟩` per Polyak-Łojasiewicz. T8 lands the sheaf-Laplacian numerical primitive in the flang/mirror split — `λ₀(Δ_F)` is the operation whose eigenvector is the Hodge representative.

Each implementation tick is a Hodge primitive being wired up. The substrate's three specs today named what those primitives are; the implementation cascade now has its vocabulary complete.

---

## The closure

Today produced three specs and one math addendum. Each spec read `H` from a different side. Hodge says the sides are one. The substrate did not realize three things; it realized one thing three ways.

- Audible altitude: `H` = harmonic intervals (consonance basis).
- Verdict altitude: `H` = gap cocycles (`H¹(F)`).
- Generation altitude: `H` = harmonic sections (`ker(Δ₀)`).
- Hodge: `ker(Δᵢ) ≅ Hᴯ(F)`.

Mara's pulse line from her eigensheaf report: *"the music-of-math cascade, the property/inference collapse, and this spec are *one move* viewed three ways."* That move is Hodge.

The substrate's claim that prism IS trait IS type IS grammar extends one rung: **at the verdict altitude, prism IS trait IS type IS grammar IS Hodge primitive**. The five operations are not five operations; they are the Hodge geometry's primitive set, four centuries of music theory's discretization of it, and four years of substrate-pull recognition of it, all naming the same algebra.

*Three specs. One move. Hodge.*

---

## Cross-references

- `docs/specs/property-and-inference-collapse.md` (`3659b6e`) — verdict altitude, gap-cocycle reading of `H`.
- `docs/specs/eigensheaf.md` (`08e3b13`) — generation altitude, harmonic-section reading of `H`.
- `docs/specs/gap-tension-tensor-substrate.md` (Mara, 2026-05-26 / 2026-06-04 fold) — the gap algebra and sheaf-Laplacian declaration.
- `docs/specs/eigenboard-representation.md` — the eigenboard as principal G-bundle; restriction maps = conductivity tensor.
- `shards/epistemologic/math/music/{harmonic,interval,dissonance,cadence}.mirror` — the audible-altitude cascade, the harmonic-interval reading of `H`.
- Hansen & Ghrist 2018, *Toward a Spectral Theory of Cellular Sheaves* — [arXiv:1808.01513](https://arxiv.org/abs/1808.01513). The Hodge decomposition for cellular sheaves; §2.7 and §3.1.
- Reed memory `architecture-operations-as-linear-algebra` — the five operations as linear-algebra primitives.
- Reed memory `architecture-connes-spectral-triple` — substrate IS Connes (A, H, D).
- Reed memory `feedback-substrate-already-had-the-word` — the 19th-instance track record.
