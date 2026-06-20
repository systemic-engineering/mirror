# `docs/specs/recognitions/` — per-recognition canonical specs

Per the substrate-decl at `shards/mirror/docs/spec.mirror`
(`@mirror/docs/spec`, tick 46 / docs-ouroboros tick 2; landed in
the same session that landed `shards/mirror/docs.mirror` (#90
candidate)).

## The convention

A spec is a **recognition_spec** (per `spec_kind` variant) when its
filename matches one of:
- `recognition-NN-<topic>-canonical-spec.md` — the canonical spec
  for recognition #NN (where NN is the recognition number)
- `recognition-NN-research-<YYYY-MM-DD>.md` — research-altitude
  snapshot grounding recognition #NN at a specific tick
- `cascade-recognition-NN-through-MM-canonical-spec.md` — a
  cascade spec spanning recognitions #NN through #MM
- `<topic>-tower-research-<YYYY-MM-DD>.md` — cross-cutting
  research that grounded multiple recognitions

These files live HERE (not at `docs/specs/` top level). The
family-root's `spec_in_canonical_subdir` bilateral predicate gates
on this convention.

Evergreen specs (rewritten in place; no recognition number) stay at
`docs/specs/<topic>.md`.

## What lives here at tick 46 landing

Reed's docs-ouroboros tick 2 relocates six files from
`docs/specs/` flat to here:

1. `cascade-recognition-76-through-80-canonical-spec.md`
2. `recognition-76-research-2026-06-18.md`
3. `recognition-79-research-2026-06-18.md`
4. `recognition-81-runtime-magic-canonical-spec.md`
5. `recognition-82-frame-as-cognitive-order-canonical-spec.md`
6. `string-theory-tower-research-2026-06-18.md`

## Why this convention

Per Reed-agent cleanup audit (docs/cleanup-review-2026-06-20.md)
S-2 finding: `docs/specs/` had 107 flat files at audit time with
three new recognition-canonical-specs landed in the prior 5 days.
Without the `recognitions/` subdir, `docs/specs/` becomes
unfindable as the cascade grows.

The convention preserves the substrate's auditable recognition
history: every recognition has a canonical spec; specs accumulate
rather than being rewritten; subdir consolidation keeps the
evergreen-spec workspace navigable.

## Cross-reference

- Substrate-decl: `shards/mirror/docs/spec.mirror`
- Parent family-root: `shards/mirror/docs.mirror` (#90)
- Cleanup audit grounding: `docs/cleanup-review-2026-06-20.md`
- Bilateral predicate: `@mirror/docs/spec.spec_in_canonical_subdir`
