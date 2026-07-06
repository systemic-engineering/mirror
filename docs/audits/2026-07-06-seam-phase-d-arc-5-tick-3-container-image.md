# Seam Phase D — Arc 5 TICK 3: `shards/container/image.mirror` first witness of @kintsugi/shift

*Reed-inline execution.*

**Commit under review**: `b66b280` (Mara) — 340 lines, Interpretation B, first
empirical witness of `@kintsugi/shift` primitive promoted at TICK 2 `49f0486`.

---

## §1. Verdict

**RATIFY.**

All 15 text-check tests pass empirically. Interpretation B verified.
**Substrate-pull-notable**: this is the FIRST species whose SHAPE (not just an
operation in a five-op body) IS a shift. The species declaration itself IS
`shift(@mirror/store/oci → @container altitude)`. Empirical validation that
TICK 2's primitive composes as a species-shape, not just as an action.

Original Seam Correction #1 (drop @container/image due to overlap with
@mirror/store/oci) RESOLVED via Alex's hardlink/lens framing without dropping
the surface. Same OID at two altitudes; zero content duplication.

## §2. 15/15 empirical verify

T1-T7 canonical species + T8-T12 Interpretation B + inherit + T13-T15
hardlink framing + first-witness role + Correction #1 citation all green.

## §3. Species-shape as shift — substrate-pull recognition

This is not just "a species that USES shift" — the entire species declaration
IS a shift. `type image_witness = shift_witness` names the semantic slot at
container-altitude while byte-equaling the underlying shift primitive's
witness carrier. Explicit-over-implicit discipline (`[[feedback-explicit-over-implicit]]`)
honored.

**Composed-bilateral (15th #53 instance)**: `image_shifted_from_oci(w) -> verdict`
composes `shift_preserves_content` (TICK 2's 14th) + forward-promised
`image_admissible_as_container_source` (TICK 5 `@container/runtime`).

## §4. T7 refusal discipline honored

No `type oci_manifest`, no `type oci_content`, no `type image_bytes`. OCI
content substrate-decl LIVES at `@mirror/store/oci` (source altitude); this
species is the lens/hardlink at container-altitude. **Zero duplication of
substrate-decl** — exactly Alex's hardlink framing.

## §5. Inheritance discipline

Six inherits: `@prism`, `@meta`, `@glass` (universal + transparency) +
`@container` (parent family) + `@kintsugi/shift` (primitive witnessed) +
`@mirror/store` (source-altitude ancestor). Composition-not-inheritance-for-shift
discipline honored: `in @kintsugi/shift` is composition; it doesn't `<= @kintsugi/shift`.

## §6. Signal-to-Reed

**TICK 3 CLOSED.** Two witnesses now support the shift promotion:
1. `@container/image` at container-altitude witnesses `shift(@mirror/store/oci → @container)`
2. Prior landings (implicit): #26 shift(oid,T) portal + prism.mirror binding + mosaic.mirror emitter + 13 #53 bilateral instances

**TICK 4 unblocks**: `@code/docker` species with `docker_buildable` composed
bilateral per original Mara spec §6. The @code build surface can now compose
against `@container/image` at container-altitude AND `@io/oci` at distribution-
altitude, closing the three-surface partition (@code/docker declaration ↔
@container/image runtime ↔ @io/oci distribution).

## §7. Housekeeping deferred (Mara flagged)

Three test files carry unstaged rustfmt drift from prior ticks. Per sequential
commit path, NOT bundled into TICK 3. Belongs in a follow-up 🔧 hygiene tick
(post-cascade).

---

*2026-07-06. Seam (Reed-inline). Phase D on Arc 5 TICK 3 `b66b280` RATIFIED.
First species-shape-as-shift witness landed. TICK 4 @code/docker species
unblocks.*
