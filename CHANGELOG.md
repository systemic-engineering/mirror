# Changelog

All notable changes to mirror will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
adapted to mirror's substrate-pull discipline:

- **Substrate** — substrate-decl shards landed, substrate vocabulary
  renamed, root-prism families introduced.
- **Recognitions** — substrate-pull recognitions promoted (with the
  `#NN` identifier; the canonical record lives in
  `~/.reed/` MEMORY entries).
- **Added** — user-observable surfaces (CLI subcommands, grammar
  primitives, output formats) that weren't there before.
- **Changed** — user-observable changes to existing surfaces.
- **Deprecated** — surfaces still present but slated for removal.
- **Removed** — surfaces gone.
- **Fixed** — bugs closed.

This project does not yet adhere to [Semantic Versioning](https://semver.org/);
the pre-v0.1 ground is still settling. v0.1.0 (the kintsugi-CI
release; see `docs/specs/kintsugi-ci-v0.1.md`) will be the first
tag where semver applies. v1.0.0 is the spectral.engineer cloud
deployment (per `roadmap/wip/v1-launch.md`).

## [Unreleased]

### Substrate
- Round-trip composition: `mirror spawn <home> --hello-world` composes
  peer's recall payloads in-process (commit `28943c1`, 2026-06-27).
- `@mirror/docs` family-root (recognition #90 candidate;
  `shards/mirror/docs.mirror`).
- `@mirror/spec IS λ₀` recognition #99 promoted (commit `d0b6519`).
- `@mirror/ref` family-root for reference⇔reflection collision
  (recognition #89; landed tick 44 on origin/main).

### Added
- `CONTRIBUTING.md` — contributor guide adapted from knigge's pattern
  to mirror's substrate-pull discipline + Pack-as-orchestra
  authoring structure (2026-06-27).
- `docs/insights/README.md`, `docs/scouts/README.md`,
  `docs/reviews/README.md` — genre-clarifying READMEs ahead of
  external contributor arrival (2026-06-27).
- `docs/observations/` — new doc genre for first-person agent
  reports of substrate interaction (Reed `c0acf41`).

### Changed
- README links CONTRIBUTING.md + adds a Contributing section naming
  the Pack-as-orchestra (2026-06-27).

---

*Earlier history is captured in commit log + insight docs at
`docs/insights/`; a structured CHANGELOG begins from this entry.
For grant verification numerics, see `docs/GRANTS.md` (refreshed
via `just docs-refresh`).*
