# The @spin cluster — mathematical grounding

*The Clifford thread the substrate has been carrying, formalized at
math altitude in preparation for the axiom shards at
`shards/epistemologic/math/spin/`.*

This directory grounds the mathematical content that @spin depends on.
Canonical spec: `docs/specs/spin-as-clifford-thread.md` (candidate #114,
2026-07-01).

Structure:

- `clifford-thread.md` — the Clifford thread through the bundle tower;
  Clifford algebras Cl(p,q), Spin(N) groups, spinor bundles, the spin
  connection, and how it composes across altitudes via `spin_lift`.
  Feeds §6.1 (`clifford_relations`) and §6.6 (`spinor_bundle_representation`)
  axiom shards.

- `cpt-recursion.md` — the fourth-order CPT-preservation witness math
  derivation. Wigner classification, spin-statistics theorem, CPT theorem
  proofs at substrate-decl altitude. Feeds §6.2 (`wigner_classification`),
  §6.3 (`spin_statistics_theorem`), §6.4 (`cpt_theorem`), §6.5
  (`pauli_exclusion`) axiom shards. Grounds §7's
  `cpt_preserved_across_recursion` witness.

- `bibliography.md` — the paper hunt bibliography (Kagi + ArXiv). Lists
  every source cited across the spec + math cluster, plus adjacent-pull
  candidates for future consideration. Marks the papers Alex might want
  to download for deeper reading.

## Landing order

1. Spec `docs/specs/spin-as-clifford-thread.md` (2026-07-01, this tick).
2. Math cluster `docs/math/spin/` (2026-07-01, this tick).
3. Pack ratification (forward-promised).
4. Marker shard `shards/spin.mirror` (forward-promised).
5. Axiom shards `shards/epistemologic/math/spin/*.mirror` (forward-promised).
6. Species shards `shards/spin/*.mirror` (per consumer pull).
7. Reality shards `shards/epistemologic/reality/spin/*.mirror` (per
   consumer pull; forward-promised).

## Cross-references

- `docs/math/the-tower/principal-bundles.md` — the principal-bundle
  primitives the Clifford thread stitches together.
- `docs/math/the-tower/spectral-triples.md` — (A, H, D) at each altitude;
  @spin extends to (A, H, D, J, γ) with Cl-module typing.
- `docs/math/the-tower/curvature-and-tomm.md` — [D, a] = curvature 2-form.
  At spin altitude, [D_spin, a] = spin curvature.
- `shards/epistemologic/cybernetic/chirality.mirror` — #101 γ at
  cybernetic altitude; @spin imports as ancestor.
- `shards/epistemologic/cybernetic/charge_conjugation.mirror` — #102 J at
  cybernetic altitude; @spin imports as ancestor.
- `shards/algebra.mirror` — #104 P5's A of the Connes real spectral
  triple; @spin extends J and γ with Clifford typing.
