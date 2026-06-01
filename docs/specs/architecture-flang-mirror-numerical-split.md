# architecture-flang-mirror-numerical-split — who runs the numbers, who composes the operations

*2026-05-28. Mara. Spec — architecture, not implementation.*

Status: **Yellow** (architectural recognition; resolves the dangling
`[[architecture-flang-mirror-numerical-split]]` wikilink referenced from
`substrate-native-fate-tournament.md`. Structural-hypothesis, not formally
verified; inherits the speculative-mapping hedge of its sources.)

Depends on:
- `docs/specs/substrate-native-fate-tournament.md` — the 16×16 Fate fiber,
  `au` as spectral coordinate, conductivity IS Laplacian.
- `docs/specs/numerical-substrate-via-fortran.md` — the flang LLVM-IR
  pathway; the `@code/fortran` grammar; Fortran Fate as a package.
- `docs/specs/prism-core-as-spectral-triple.md` — `prism/core` as the
  spectral triple's algebra; the bootstrap as its evaluator.
- `~/.reed/practice/insights/cosmology/eventually-consistent-universe.md`
  §4.4 — the 16 = 12 gauge + 4 Higgs reading; SSB as eigenvalue splitting.

---

## Thesis

There are two altitudes of numerical work in the substrate, and they belong
to two different runtimes:

- **flang runs the 16×16 weight inference.** The Fate fiber is 16-dimensional.
  Dense eigendecomposition, the SCF settlement loop, the LAPACK calls — the
  per-element floating-point work over the connection fiber — is compiled
  Fortran, emitted as LLVM IR by flang, consumed through `@code/llvm/ir`.
- **mirror composes the 5×5 fiber/eigenvalue scaling.** The five-operation
  Prism base (focus, project, split, zoom, refract) is where the substrate
  *composes* — restriction maps, the eigenvalue scaling between operations,
  the holonomy bookkeeping. That composition is mirror grammar, not Fortran.

The lift between them — `16 → 5` — is not an arbitrary projection. It is the
**monadic lift = SSB / the spectral action**: the 16-dimensional fiber's
degenerate spectrum splits, and the splitting selects which degrees of
freedom become observable as the five-operation base while the rest stay in
the bulk. (Per the source, the precise Standard-Model identification — 12
gauge + 4 Higgs = 16 — is speculative; what is established is the structural
shape: a degenerate fiber spectrum that splits, with the split *being* the
lift.)

## The split as a spectral triple

The split is the spectral triple `(A, H, D)`, assigned across the two
runtimes:

| Component | Spectral-triple role | Runtime | What it is |
|-----------|----------------------|---------|------------|
| `A` — algebra | the operations you compose | **mirror** | the five-op Prism algebra; `prism/core` |
| `D` — Dirac operator | the metric; what measures distance | **flang** | `D = d + d*`; eigendecomposition over the 16×16 fiber |
| `H` — Hilbert space | the state being acted on | **data** | the eigenvector/eigenvalue records `au` settles onto |

`A` is composition; it stays in grammar. `D` is the numerically dense
operator; it goes to compiled Fortran. `H` is neither code nor compiler —
it is the settled state, content-addressed as a git blob.

## The lift, typed

The lift is `16 → 5`. Honoring no-bare-types, neither end is a bare `[f64]`:

```mirror
-- the 16-dimensional connection fiber (flang's domain)
type fiber_state = record {
  connection: connection_fiber,     -- newtype over the 16×16, NOT [[f64; 16]; 16]
  spectrum:   fiber_spectrum,       -- the 16 eigenvalues, post-SCF; NOT [f64]
}

-- the five-operation base (mirror's domain)
type base_state = record {
  trajectory: eigenvector,          -- lives in eigenspace, NOT bare [f64]
  scaling:    eigenvalue_scaling,   -- the 5×5 inter-operation scaling
}

-- the lift IS spontaneous symmetry breaking / the spectral action
action lift(fiber_state) -> base_state = \    -- body parked; resolved via Fate
  -- the 16-dim spectrum splits; the split selects the observable five-op base.
```

The hole `\` is honest: the lift's body is the spectral action, resolved
through Fate's tournament, not hand-written here. The *types* commit to the
shape; the body settles later.

## Why the split, not one runtime

- **flang for `D`.** Dense eigendecomposition over a 16×16 fiber is exactly
  what LAPACK-backed Fortran is for. The LLVM-IR pathway (`numerical-substrate-via-fortran`)
  keeps this content-addressed: flang emits IR, mirror consumes it, the IR
  enters the same OID pipeline as mirror's own. The numerical floor is
  substrate, not `@io`.
- **mirror for `A`.** The five-op composition is holonomy bookkeeping —
  restriction maps on the eigenboard sheaf, the gutter's green/amber/red.
  That is what the substrate exists to *describe of itself*; it is never
  Fortran. Pushing it into flang would be capability-in-the-floor — the
  inverse of the FROZEN discipline.
- **The lift is the seam.** `16 → 5` is the one morphism that touches both:
  flang produces the settled 16-spectrum; mirror reads the split as its
  five-op base. The seam is where `au` is born — the four coordinates
  (`eigenvector`, `eigenvalue`, `fiedler`, `eigengap`) read off at the SCF
  fixed point.

## Boundary note

The flang side is **boundary/floor Rust + Fortran**, not capability Rust: the
FFI `extern` declarations, the `build.rs` flang invocation, the LLVM-IR link
step. Those commits carry `[substrate-pull:realize]` (paired with `🔧`, per
AGENTS.md — standalone boundary work is not a red/green pair). The mirror side
is pure grammar. The line between them is exactly the line the FROZEN policy
draws: `A` is grammar; `D`'s *invocation* is floor.

## Open

- The `16 → 5` lift body (`lift(fiber_state)`) is parked (`\`). Resolving it
  is downstream of Fate tournament settlement over the fiber.
- Whether `connection_fiber` is `16×16` dense or factored is a
  `numerical-substrate-via-fortran` §10 concern, not settled here.
- The Standard-Model identification (12 gauge + 4 Higgs) stays a hedge; this
  spec asserts only the structural shape (degenerate spectrum → split → lift).
