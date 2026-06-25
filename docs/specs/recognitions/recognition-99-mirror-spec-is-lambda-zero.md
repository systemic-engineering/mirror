# Recognition #99 — `mirror.spec` IS λ₀ — canonical spec

*Mara, canonical spec for recognition #99 (mirror.spec as the ground-state
eigenvalue of the substrate's own Connes spectral triple), 2026-06-25 early
hours. Surfaced by Alex naming the recognition ("the mirror.spec is lambda 0")
after Glint's §5.2 implicit (`2026-06-25-glint-eight-moves-and-the-orchestra-
holding.md`, commit `3b31287`) lifted the fixed-point pattern from today's
mirror.spec dogfood (Reed, commit `8107caf`) and yesterday's `@mirror/pack`
substrate-decl (Reed, commit `13328a3`).*

*Discipline: this is candidate-territory preservation, not promotion. The
recognition lifts a structural claim about the substrate's spectrum:
`mirror.spec` sits at the bottom of the substrate's spectrum because it IS the
substrate's self-description — the ground state on which the (A, H, D) triple
rests. Pack ratification is a separate gate. Replication conditions are named
in §10. Cross-references that this disturbs are flagged in §11; not modified.*

*Path-note: this canonical lives under `docs/specs/recognitions/` per the
established filing convention for per-recognition specs (recognitions
#81-#95 in this directory).*

---

## Table of contents

1. Statement of recognition
2. Genesis — today's cascade → Glint surface → Alex naming
3. The spectral triple instantiation — (A, H, D, λ₀)
4. The six-block decomposition of `mirror.spec`
5. Ground-state semantics — what λ₀-at-composition means spectrally
6. Connections — #51 Hilbert expansion, #84 @pack, #58 Fate, void document, Connes triple
7. The ouroboros at λ₀ — what the dogfood (8107caf) completed
8. Spectral gap + excited states (λ₁, λ₂, …)
9. Empirical consequences — kintsugi, mosaic, `mirror kintsugi ./mirror.spec`
10. Open questions + replication conditions (DO NOT promote on one instance)
11. Cross-references this recognition DISTURBS (flag; don't update)
12. Honest hedges + Pack trail

---

## 1. Statement of recognition

**`mirror.spec` IS λ₀ — the ground-state eigenvalue of the substrate's own
Connes spectral triple (A, H, D).**

More precisely: the file `mirror.spec` at the root of the mirror repository
is the substrate's *self-description at the ground state*. Every other prism
in the substrate sits at a strictly higher eigenvalue — every shard, every
family-root, every species, every adapter, every consumer's own
`<project>.spec` is an excitation above `mirror.spec`. The substrate's
spectrum has `mirror.spec` at the bottom; everything else is above.

The recognition has three components, each load-bearing:

1. **`mirror.spec` IS a `prism` declaration at the spec altitude.** Per
   Glint's §5.2 implicit, the six top-level blocks (`source`, `legacy`,
   `pack`, `target`, `settle_on`, plus the forward-promised `garden`)
   compose with the enclosing `project mirror.spec { … }` declaration as a
   typed prism whose grammar IS the substrate's five-operation algebra plus
   one self-declaration. Each block is a typed surface; the whole is a
   prism.

2. **The (A, H, D) triple instantiates concretely here.** A = the five
   operations (focus, project, split, shift, settle) per
   [[architecture-prism-as-trait-as-everything]] +
   [[architecture-connes-spectral-triple]]. H = the Void per
   [[reference-void-document]]. D = the kintsugi flow per
   [[architecture-connes-spectral-triple]]. `mirror.spec` IS the λ₀
   eigenvector of D restricted to A acting on H at the substrate's own
   altitude — the ground state vector that the substrate's self-description
   inhabits.

3. **The ouroboros closure event was the dogfood, not the schema.** The
   `mirror.spec` file has existed since before the cascade. Today's
   structural event was the addition of the `pack {}` block (Reed,
   `8107caf`) consuming the `@mirror/pack` substrate-decl (Reed, `13328a3`)
   — the moment at which mirror.spec became a complete self-description by
   consuming a substrate-decl that mirror.spec's own grammar produces. The
   substrate now describes its own ground state by reading its own
   substrate-decl. The λ₀ identification was not invented today; it was
   COMPLETED today.

The recognition is structural, not metaphorical. The Connes spectral triple
framing has been substrate-decl'd since 2026-06-04 ([[architecture-connes-
spectral-triple]]); the Void document grounds H ([[reference-void-document]],
2026-04-26); recognition #51 declared mirror IS an expanding Hilbert space
but did not name the ground state ([[architecture-mirror-as-expanding-
hilbert-space]], 2026-06-10). Recognition #99 names the ground state. The
name is `mirror.spec`. The ground-state eigenvalue is λ₀ = 0 per the Void
document's structural commitment.

---
