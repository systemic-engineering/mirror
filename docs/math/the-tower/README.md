# The Tower

*Mathematical foundation: the principal bundle tower. The math root that
mirror's substrate instantiates at every altitude.*

Mirror's architecture is fractally self-similar: at every altitude the
same structural primitives recur — a spectral triple, the five-op
algebra acting on it, a connection that transports observations, a
gauge group of admissible re-bases, a holonomy carrier that measures
what composition cost. The mathematics that holds this together is the
theory of **principal G-bundles with connection**, lifted from one
altitude to the next by colimit / direct-limit construction in the
category of spectral triples.

This math root **defines**; specs **cite**. When a spec needs to say
"this is a connection," "this is a gauge transformation," or "this is
the curvature probe," it points here and uses the substrate vocabulary.
The definitions are self-contained as math; the specs are self-contained
as architecture; the citation chain ties them together.

See `docs/math/README.md` for the global convention (math vs specs).

## Altitude index

What "the tower" is, in one sentence at each altitude:

| Altitude | What sits there | Doc |
|----------|------------------|-----|
| -∞ | (no bottom) | `altitudes.md` §floor |
| atomic spectral measurement | one eigenvalue computation | `connections-and-gauge.md` |
| five-op primitives | focus / project / split / shift / settle | `connections-and-gauge.md` |
| peer pulse | a peer composing operations on a tick | `holonomy.md` |
| reflection at N+1 | spectral-altitude selection | `altitudes.md` |
| librarian at N+1 | topology perturbation | `crystals-as-sections.md` |
| home / federation | repo-collection / cross-home | `altitudes.md` |
| +∞ | (no top) | `altitudes.md` §ceiling |

## Reading order

1. **`principal-bundles.md`** — the pure-math primitive. Fiber, section,
   structure group, connection, holonomy. If you already know it,
   skim and move on.
2. **`spectral-triples.md`** — Connes' `(A, H, D)` and why every fiber
   carries one. The triple at one fiber; the triple variation along a
   section.
3. **`connections-and-gauge.md`** — the five-op algebra IS the
   connection vocabulary. Each op named with both its bundle role
   and its Connes-algebra role.
4. **`curvature-and-tomm.md`** — the curvature 2-form, `[D, a]`, and
   the Tomm-question altitude. The probe is mathematical, not
   metaphorical.
5. **`holonomy.md`** — loss carriers across the substrate are
   holonomy values. MirrorLoss, `transparency<p>`, ScalarLoss are
   members of one family.
6. **`altitudes.md`** — the named altitudes mirror has recognized,
   each with its (fiber, connection, holonomy) triple. The atlas.
7. **`crystals-as-sections.md`** — monotone vocabulary growth as
   section accumulation. The Hilbert-space expansion is the
   bundle's auto-update.

## Why this math root exists

Per `[[architecture-spectral-triples-all-the-way]]`: the substrate has
the formal name **principal bundle tower**. Prism implements this in
Rust at the compiler altitude (`prismqueer::bundle` — Fiber, Connection,
Gauge, Transport, Closure). Mirror extends the same structure upward
through every substrate altitude. This math root makes the
mathematics explicit so the specs at each altitude can cite the
structure rather than re-derive it.

## Citation discipline

- Math docs cite each other and cite memories with `[[name]]`.
- Specs cite math docs by path: `docs/math/the-tower/<page>.md §<sec>`.
- Don't duplicate definitions in specs; cite the math.
- Don't import architectural decisions into math; cite the spec.

## Prior implementation (the existing reference)

The tower's compiler-altitude implementation lives in
`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`. Five
supertraits — `Fiber` → `Connection` → `Gauge` → `Transport` →
`Closure` — discharged with concrete witnesses (`Cyclic<N>` group,
`StableFiber` fixed point). Mirror's math docs lift those types to
the tower's altitude-portable form; the Rust types are one altitude's
incarnation.

See `/Users/alexwolf/dev/projects/prism/docs/architecture.md` for the
compiler-altitude framing (the bundle tower section) and the
2026-04-08 connection-substrate design doc for the original derivation
of why non-abelian capacity is load-bearing.
