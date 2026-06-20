# `docs/archive/` — superseded documentation preserved as honest history

Per the substrate's no-delete-without-pact discipline + the
`@mirror/docs` family-root substrate-decl (recognition #90 candidate;
landed at `shards/mirror/docs.mirror`).

## The archive convention

A document is archived (NOT deleted) when its `supersession_status`
is `superseded` per the `@mirror/docs.classify` action. The move
preserves the substrate's honest-history discipline:

```
docs/<original-path>  →  docs/archive/<original-path>
```

With a supersession header prepended to the file explaining:
- WHY it's archived (which recognition/shard supersedes it)
- WHAT replaces it (cite the active substrate)
- WHEN it was archived (date + commit SHA)

## What lives here

The Reed-agent cleanup audit (2026-06-20) identified 13 top-level
SUPERSEDED candidates that will populate this archive in tick 4 of
the docs/ ouroboros loop. The audit also identified `roadmap/00-12/*`
(except `10-inference-physics.md`), `plans/`, and `superpowers/` as
en-bloc archive candidates in tick 5.

What is currently empty is intentional — this README skeleton is
tick 1's deliverable; population follows per-tick.

## What does NOT live here

- HONEST_HISTORY docs (e.g., `docs/cleanup-review-2026-04-29.md`)
  stay at their original path with their dated filename serving as
  the historical marker. They are NOT superseded; they ARE the
  history.
- CURRENT docs (actively referenced by landed substrate) stay at
  their original path.
- The archive is for the substrate's PAST that the substrate's
  PRESENT renders inactive but that the substrate's HONESTY
  preserves.

## Cross-reference

- Family-root: `shards/mirror/docs.mirror` (the substrate-decl
  declaring this convention)
- Cleanup audit: `docs/cleanup-review-2026-06-20.md` (the Reed-agent
  map that grounds this tick's work)
- Bilateral predicate: `@mirror/docs.doc_well_classified` (the
  substrate's archive-correctness check)
