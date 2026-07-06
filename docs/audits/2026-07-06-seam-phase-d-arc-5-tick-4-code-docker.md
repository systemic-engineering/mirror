# Seam Phase D — Arc 5 TICK 4: `shards/code/docker.mirror`

*Reed-inline execution.*

**Commit under review**: `a1fb4bd` (Mara) — 371 lines, Interpretation B, @code/docker
species per Mara spec `ec636d3` §6.1.

---

## §1. Verdict

**RATIFY.**

All 15 text-check tests pass. Three-surface partition closed (@code/docker
declaration ↔ @container/image runtime ↔ @io/oci distribution).

**16th #53 composed-bilateral** landed: `docker_buildable(d) -> verdict` composes
`dockerfile_parses` + `dockerfile_llb_emittable` + `shift_preserves_content`
(bridging to TICK 2 `49f0486`). Structural bilateral cascade extending.

## §2. 15/15 empirical verify

T1-T7 canonical + T8-T12 Interpretation B + inherit + T13-T15 three-surface
grounding all green.

## §3. Substrate-honest reuse of @container/image carrier

Mara imports `image_witness` from `@container/image` rather than inventing a
parallel carrier. `build(d: dockerfile) -> image_witness` — the code-altitude
action returns the container-altitude witness directly, making the cross-altitude
fold structural, not narrative.

**The build IS a shift** through `@kintsugi/shift`. Second empirical witness of
TICK 2's primitive (TICK 3 @container/image was first; TICK 4 @code/docker is
second — species-shape witness vs carrier-reuse witness).

## §4. Three-surface partition closure

First substrate-decl closure of the docker/OCI substrate:

| Altitude | Species | Role |
|----------|---------|------|
| code | `@code/docker` | Dockerfile as declarative code (this TICK) |
| container | `@container/image` (TICK 3 `b66b280`) | Runtime lens via shift |
| distribution | `@io/oci` (pre-existing) | Registry protocol |

@code/docker's `build` action folds code → container altitude via `@kintsugi/shift`.
Registry side (@io/oci) is composition point for distribution.

## §5. Signal-to-Reed

**TICK 4 CLOSED.**

**TICK 5 unblocks**: `@container/runtime` with `runtime_daemon_absent` predicate
per Mara spec §6.3 + §10.4. This DIRECTLY resolves the StageFreight-daemon
blocker (task #540) via the Splinter-pole path (podman/buildah/containerd-direct
daemonless dispatch).

**Arc 6 candidate (Taut scout `a83fd02872f9e6ba5`)**: `@song` top-level abstraction
as Arc-level pivot binding music-math + kintsugi loop + StageFreight cascade +
psychohistory. 55th+ instance of substrate-already-had-the-word. Arc 5 TICK 5
lands StageFreight-daemon resolution first; Arc 6 opens post-Arc-5 (or Alex
adjudicates pivot ordering).

---

*2026-07-06. Seam (Reed-inline). Phase D on Arc 5 TICK 4 `a1fb4bd` RATIFIED.
Three-surface partition closed. Second empirical witness of @kintsugi/shift.
TICK 5 @container/runtime unblocks StageFreight-daemon resolution.*
