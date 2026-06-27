# docs/reviews/

Adversarial review records of in-flight work (specs, cascades,
proposals not yet landed).

> **Open structural question — surfaced 2026-06-27 (Mara):**
> `docs/audits/README.md` (tick 47) declared that `reviews/` was
> consolidated into `audits/` and would stay empty going forward.
> Two new Seam reviews then landed here
> (`2026-06-24-seam-garden-pack-acl-review.md`,
> `2026-06-26-seam-mirror-recall-spec-review.md`). One of three
> things is true and the Pack should pick one:
>
> 1. **`reviews/` IS canonical for in-flight reviews; `audits/` is
>    for landed-and-consolidated adversarial-review records.** The
>    naming distinction is forward-looking-vs-historical. If so,
>    `audits/README.md` should be updated and this directory stays.
> 2. **The two recent files should move to `audits/`.** The tick 47
>    consolidation stands; the new files drifted to the wrong path.
> 3. **`reviews/` and `audits/` are the same genre and one should
>    be deleted.** The original tick 47 framing.
>
> Until the Pack settles this, both directories carry adversarial
> reviews; consult both when looking for the latest review of a
> given piece.

## Distinction from sibling genres (provisional)

- `audits/` — adversarial review records, dated and consolidated per
  the tick 47 convention. See `docs/audits/README.md`.
- `observations/` — first-person agent observations of substrate
  interaction.
- `scouts/` — substrate-pull recognitions of next moves.
- `insights/` — load-bearing semantics docs.
- `reviews/` (this dir) — currently functioning as the landing site
  for new Seam adversarial reviews of recent work.

## Naming

`YYYY-MM-DD-<author>-<scope>.md` — matches the `audits/` convention.

## Cross-reference

- `docs/audits/README.md` — the tick 47 consolidation record
- `docs/cleanup-review-2026-06-20.md` §3 finding S-3 — the
  original duplication finding
- `docs/cleanup-review-2026-06-20-followup.md` — the [UNSURE]
  register; this conflict is a new instance of the same shape
