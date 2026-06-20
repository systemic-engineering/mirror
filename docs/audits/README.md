# `docs/audits/` — adversarial review records

Per the substrate-decl at `shards/mirror/docs/audit.mirror`
(`@mirror/docs/audit`, tick 47 / docs-ouroboros tick 3; landed in
the same session that consolidated `docs/reviews/` + `docs/review/`
into here).

## The convention

Adversarial review records are first-class substrate (per
`@pack/seam` tick 33; the adversarial-review-frame peer). They
live HERE (not at `docs/reviews/` or `docs/review/`; both redundant
subdir names were consolidated to this canonical path).

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

## What lives here at tick 47 landing

After the consolidation, this subdir contains 8 files (3 already
here + 5 moved from `reviews/` + `review/`):

**Already at canonical path**:
1. `2026-05-22-seam-mirror-post-meta-glass.md`
2. `2026-05-25-peer-glass-audit.md`
3. `option-result-audit.md` (undated; honest-history)

**Moved this tick** (via git mv, preserving content-addressed
history):
4. `2026-04-14-fold-operator-review.md` (from `reviews/`)
5. `2026-04-14-session-final-review.md` (from `reviews/`)
6. `2026-05-30-pre-merge-adversarial-review.md` (from `reviews/`)
7. `seam-ast-optics-review.md` (from `reviews/`; undated;
   honest-history)
8. `2026-05-20-seam-adversarial.md` (from `review/`)

After the move: `docs/reviews/` and `docs/review/` are empty; git
stops tracking the empty directories implicitly.

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
