# `docs/audits/` — adversarial review records

Per the substrate-decl at `shards/mirror/docs/audit.mirror`
(`@mirror/docs/audit`, tick 47 / docs-ouroboros tick 3; landed in
the same session that consolidated `docs/reviews/` + `docs/review/`
into here). Re-consolidated 2026-06-27 (Alex) after two Seam
reviews drifted back into a re-created `docs/reviews/`; the Pack
settled option (a) — `audits/` is the single canonical genre for
ALL adversarial-review-shaped artifacts (retrospective sweeps AND
per-artifact reviews of in-flight specs/cascades/proposals).

## The convention

Adversarial review records are first-class substrate (per
`@pack/seam` tick 33; the adversarial-review-frame peer). They
live HERE — `docs/reviews/` and `docs/review/` are not canonical
and should not be re-created. New Seam reviews of in-flight work
land directly in `docs/audits/` under the dated naming convention
below.

### Naming

Ideal form: `YYYY-MM-DD-<author>-<scope>.md`

Examples:
- `2026-05-22-seam-mirror-post-meta-glass.md` — Seam reviewing the
  post-meta-glass substrate-decl on 2026-05-22
- `2026-05-25-peer-glass-audit.md` — peer-glass audit on 2026-05-25
- `2026-05-30-pre-merge-adversarial-review.md` — pre-merge adversarial
  review on 2026-05-30

### Backward-compatible (undated files)

Two files lack date prefixes:
- `option-result-audit.md` (predates the dating convention)
- `seam-ast-optics-review.md` (predates the dating convention)

These stay as-is per the substrate's no-retroactive-rewrite
discipline. The convention applies forward; honest-history
preserves the past.

### Authors

Author MUST be one of `@pack/peer` variants (typically `seam`, but
any peer's adversarial review is admissible). The Pack-as-orchestra
discipline grounds the author typing.

## What lives here

At tick 47 (the original consolidation) this subdir held 8 files
(3 already here + 5 moved from `reviews/` + `review/`):

**Already at canonical path**:
1. `2026-05-22-seam-mirror-post-meta-glass.md`
2. `2026-05-25-peer-glass-audit.md`
3. `option-result-audit.md` (undated; honest-history)

**Moved at tick 47** (via git mv, preserving content-addressed
history):
4. `2026-04-14-fold-operator-review.md` (from `reviews/`)
5. `2026-04-14-session-final-review.md` (from `reviews/`)
6. `2026-05-30-pre-merge-adversarial-review.md` (from `reviews/`)
7. `seam-ast-optics-review.md` (from `reviews/`; undated;
   honest-history)
8. `2026-05-20-seam-adversarial.md` (from `review/`)

**Added since tick 47**:
9. `stagefreight-seam-review-2026-06-22.md` (Seam, 2026-06-22)

**Re-consolidated 2026-06-27** (Mara; via git mv from a
re-created `docs/reviews/`):
10. `2026-06-24-seam-garden-pack-acl-review.md` (Seam)
11. `2026-06-26-seam-mirror-recall-spec-review.md` (Seam)

After the 2026-06-27 move: `docs/reviews/` and `docs/review/` are
empty/removed; future adversarial reviews land directly here.

## Why this convention

Per Reed-agent cleanup audit (`docs/cleanup-review-2026-06-20.md`)
S-3 finding: three subdir names for one job is naming-redundancy
the substrate's content-addressing discipline does not tolerate.
The Pack-vocabulary alignment (audit = `@pack/seam` work) settles
the canonical name as `audits/`.

## Cross-reference

- Substrate-decl: `shards/mirror/docs/audit.mirror`
- Parent family-root: `shards/mirror/docs.mirror` (#90)
- Peer-altitude predicate this composes with: `@pack/seam.review_sound`
- Cleanup audit grounding: `docs/cleanup-review-2026-06-20.md`
- Bilateral predicate: `@mirror/docs/audit.audit_in_canonical_subdir`
