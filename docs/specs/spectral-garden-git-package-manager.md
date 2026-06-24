# `@spectral/garden/git` — the substrate-native package manager family-root

*Mara, canonical spec for the substrate's git-native package manager
surface, configured through `mirror.spec`. Forward-promised by
`shards/io/git.mirror` §6 (a1b507a) and named by Alex 2026-06-24:
"What if @spectral/garden/git is the root for the garden package
manager? What if this is a surface that's configured in the
@../mirror/mirror.spec?"*

*Pack-discipline: candidate-altitude. Surface spec only — no impl, no
Rust. Composes against `@io/git` (a1b507a) and `@io/oci` (2801478).
Section caps load-bearing per the Mara stall-recovery discipline.*

---

## Status

- **Status:** RED (spec only; no shard exists; no impl exists)
- **Altitude:** family-root surface spec; substrate-decl forward-promised
- **Recognition territory:** #98 candidate (content-addressing
  cross-altitude composition) — fourth witness named; this spec
  surfaces the FIFTH composition pattern (peer-home-repo as
  package-source). Promotion decision deferred to a separate Reed tick.
- **Pack:** Mara author; Seam adversarial review forward-promised;
  Reed consolidation forward-promised; Alex ratification gate.

## Sections

1. Position statement
2. Surface in `mirror.spec`
3. Prior-art audit (Dhall, Nix CA, Cargo, Go MVS, Bazel, IPFS, Crystal)
4. Resolution semantics
5. Composition with `@io/git` and `@io/oci`
6. The four-root structure (git / oci / nix / store)
7. Mathematical shape
8. Open surface questions
9. Forward-promised
10. Recognition cross-references
11. Honest hedges
12. Pack-discipline trail

---

## 1. Position statement

`@spectral/garden/git` IS the family-root prism for the substrate's
git-native package manager, surfaced as a configurable block inside
`mirror.spec`. It declares: **the unit of package distribution at the
peer-home altitude is a git ref** (commit / tag / branch / detached
HEAD), addressable through `@io/git`'s typed adapter, with resolution
discharged at content-addressing altitude via `git_hash` and lifted
into substrate oid space via `hash_to_oid` (per `shards/io/git.mirror`
§3).

The substrate already has every primitive a package manager needs:
- **content-addressing** at four altitudes (`oid`, `git_hash`,
  `oci_digest`, `nix derivation_hash`; #98 candidate territory)
- **typed adapters** for git (`@io/git`, a1b507a) and OCI (`@io/oci`,
  2801478) at the @io boundary
- **multi-repo span** as substrate vocabulary via `@pack`'s
  `repository_span` carrier (#84)
- **labeled-pair functor** (`labeled<v, m>`; #93 H4) for
  `git_artifact = labeled(git_object, git_ref_metadata)`
- **alignment-as-boundary-mathematics** (#57) for the Splinter /
  Narcissus partition at distribution altitude
- **content-addressed build system** primitives via `mosaic.mirror`
  (#43)
- **measurement instrument** via `@cascade` (#95 candidate) for
  loss-typed transformations across grammar boundaries

`@spectral/garden/git` does NOT invent. It NAMES the family-root that
composes these into one substrate-vocabulary surface: declarative
`garden { ~git'...' ... }` entries in `mirror.spec`, resolved by
`@io/git.clone` + `@io/git.resolve_ref` + `@io/git.hash_to_oid`,
verified by composed-bilateral `git_well_formed`, locked at the
substrate-oid altitude in `mirror.lock` (forward-promised companion
file), available to the kintsugi loop as typed substrate-decl input.

Two structural commitments load-bear the family-root choice:

1. **Peer-home-repo as package source** (per `@pack` #84). The five
   Pack peers (mara/seam/glint/reed/taut) operate across home repos;
   `@spectral/garden/git` makes each peer's home repo a typed package
   entry. Identity-substrate, dependencies, and Pack membership unify
   under one substrate primitive: `garden ~git'<peer-home-url>'`.

2. **Surface in `mirror.spec`, not a separate file**. Per
   `mirror-spec-schema.md` §8 (the self-descriptive guarantee), the
   spec that BUILDS the substrate is the spec the substrate READS.
   Adding `garden` as a top-level block (alongside `source`, `legacy`,
   `target`, `settle_on`) extends the self-descriptive surface
   monotonically. No drift between manifest and resolver; no separate
   lockfile parser; no documentation layer that drifts. One grammar,
   one spec, one settled artifact.

The recognition deferred to Reed tick: whether the four roots
(`@spectral/garden/git`, `@spectral/garden/oci`, `@spectral/garden/nix`,
`@mirror/store`) constitute the fifth witness to recognition #98
candidate (content-addressing cross-altitude composition primitive),
OR whether they constitute a sibling recognition (scope-graded
package-manager root structure as its own composition primitive).
This spec flags the question; promotion is not in scope.
