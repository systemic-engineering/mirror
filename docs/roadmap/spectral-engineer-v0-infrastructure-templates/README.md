# spectral.engineer v0.1 infrastructure templates — authored-not-deployed

**Authored**: 2026-08-03 by Reed per `/loop` orchestration + Alex 2026-08-03 roadmap + Reed shape-doc `~/.reed/tasks/pending/spectral-engineer-v0-build.md` `1e164ab`.

**Status**: authored-not-deployed. These templates realize shape-doc build-order steps 6-10 (Reed-orchestrated infrastructure). They do NOT deploy anything on their own; deployment requires the Alex-altitude Gleam Lustre app scaffold (shape-doc steps 1-5) as a build target.

**HARD BLOCK boundary** (per /loop discipline): Reed does NOT author `.gleam` files at Alex-altitude. These templates skeleton the infrastructure that composes AROUND the Alex-altitude scaffold.

## Files

| Template | Target path in app repo | Realizes |
|----------|-------------------------|----------|
| `flake.nix` | `flake.nix` at app repo root | shape-doc step 6: reproducible Gleam Lustre build via Nix |
| `stagefreight.yml` | `.stagefreight.yml` at app repo root | shape-doc step 7: content-addressed docker image dispatch |
| `fly.toml` | `fly.toml` at app repo root | shape-doc step 8-10: fly.io target-instance configuration |

## Substrate-truth correction (2026-08-03 post-authorship)

Grep-audit against `/Users/reed/dev/projects/StageFreight/` local checkout (2026-08-03 03:25) surfaced that the initial `stagefreight.yml` template (at commit `155a684`) fabricated an `apiVersion:` + `kind:FreightManifest` + `deploy.targets[].type:fly-io` schema shape that does NOT exist in landed StageFreight PR-A. Corrected shape uses the real `builds:` + `targets:` parallel-lists schema per StageFreight `README.md` + `docs/Docker.md` + `docs/Release.md`. Also surfaced: **StageFreight has NO native fly.io deploy target-kind**; fly.io deploy composition happens outside StageFreight via `flyctl` directly.

This surfaces **new [ALEX-Q-4]** on StageFreight↔fly.io composition (see corrected `stagefreight.yml` header).

## How to use

When Alex-altitude Gleam Lustre scaffold lands at `~/dev/systemic.engineering/app/` (or Alex-designated location):

1. Copy `flake.nix` to app repo root; adjust `packages.spectral-engineer` derivation to point at actual Gleam build artifact location per Alex-scaffold shape.
2. Copy `stagefreight.yml` to app repo root as `.stagefreight.yml`; verify [ALEX-Q-4] composition choice (a/b/c) + [ALEX-VERIFY] Nix↔StageFreight passthrough compatibility per header.
3. Copy `fly.toml` to app repo root; adjust `app` + `primary_region` + secrets as needed.

## Composition anchors

- Reed shape-doc: `~/.reed/tasks/pending/spectral-engineer-v0-build.md` (`1e164ab`)
- Mara canonical spec: `docs/specs/2026-08-03-mara-spectral-engineer-v0-canonical-spec.md` §6-§7 realization
- Mara math: `docs/math/2026-08-03-mara-spectral-engineer-web-altitude-formalization.md` §3 deploy-pipeline composition + §3.4 structural-property indissolubility (per R2 REED-INLINE)
- Seam Phase D: `docs/audits/2026-08-03-seam-phase-d-spectral-engineer-v0-web-altitude.md` §5 substrate-fidelity check + §6 design-token realization
- `shards/io/stagefreight.mirror` — delivery family (StageFreight PR-A merged 2026-06-22)
- Reed R1-R4 REED-INLINE cascades (`a6d6243`) — stagefreight symbol-drift fix + structural-property naming + falsifiability refactoring + Karen additions
- Nix flake pattern ancestors: NixOS `dockerTools.buildLayeredImage` (Dolstra 2004-present per math §8.5)
- fly.io pattern ancestors: fly.io team target-instance provisioning + Let's Encrypt cert integration (per math §8.5)

## [ALEX-VERIFY] markers

Templates contain `[ALEX-VERIFY]` markers where Reed authored reasonable-shape defaults that Alex should verify against actual Gleam Lustre scaffold shape + landed StageFreight PR-A schema + current fly.io v2 field grammar. These are load-bearing verification points, not TODOs.

---

🌱 Reed 2026-08-03
