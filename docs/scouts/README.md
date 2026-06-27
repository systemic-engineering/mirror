# docs/scouts/

Substrate-pull scouting — recognition of where the substrate is
*about* to land, ahead of the cascade actually landing it. Scouts
claim something about where the substrate is going, with enough
evidence to seed Pack ratification later but without the certainty
that earns an `insights/` entry yet.

Distinct from sibling genres:

- `insights/` — load-bearing semantics docs (e.g. spawn-is-substrate-
  leaving-ground-state). Insights claim something about what the
  substrate IS.
- `observations/` — first-person agent observations of the substrate's
  introspective surface during work. Observations claim something
  about the interaction surface.
- `reviews/` / `audits/` — adversarial review of artifacts. Reviews
  claim something about what's wrong (or sharp) in what's landed.
- `scouts/` (this dir) — substrate-pull *recognitions of next moves*.
  Scouts claim something about the direction the substrate is
  pulling, before the move itself lands.

A scout is the work of looking ahead and reporting honestly: *the
substrate is pulling toward X; here is the evidence; here is what
would have to be true for X to land as substrate-decl.* Typically
authored by Taut (the scouting/performance peer of the Pack-as-
orchestra), but any peer's scouting work is admissible.

## Naming

`YYYY-MM-DD-<author>-<scope>.md`

Examples:
- `2026-06-24-taut-substrate-pull-scout.md`
- `2026-06-25-taut-lambda-zero-cascade-scout.md`
- `2026-06-26-taut-psychohistory-cohomology-scout.md`

## When a scout promotes

A scout's claim crystallises into one of two outcomes:

1. **Substrate-decl landing.** The next move lands as a shard
   (`shards/...`) and Pack ratification absorbs the scout's evidence
   into the canonical record. The scout stays in this directory as
   honest history of the recognition before the landing.
2. **Promotion to `insights/`.** The recognition turns out to be
   load-bearing semantics, not just a directional pull. A new dated
   insight doc lands citing the scout as foundation. The scout stays
   here.

Scouts do NOT move to `archive/` on promotion — they ARE the
substrate-pull-honest record of the moment before the move.

## Style

- Cite the substrate signals the scout is reading (recent shards,
  insights, MEMORY anchors).
- State the prediction sharply. Vague directional gestures dilute
  the genre.
- Name what would falsify the prediction (the next-tick observable
  that would say "no, it's not pulling that way").
- Honest uncertainty welcome. `\` is a first-class scout value.

## Cross-reference

- Pack-as-orchestra: `[[project-pack-is-orchestra]]` (MEMORY)
- Sibling genres: `docs/observations/README.md`,
  `docs/audits/README.md`, `docs/archive/README.md`
