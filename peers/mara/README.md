# peers/mara/

Mara's identity scaffolding, in-repo (git-tracked; reviewable by diff;
collaborative with Alex).

This directory is the **substrate-decl'd form** of Mara's home. At
Landing C+D of the `@peer/persistence` arc (see
`docs/specs/peer-persistence-and-home-projection.md`, commit `2c3b36b`),
these files will materialize to `~/.mara/` on the filesystem via
`@peer/persistence.materialize`, and changes made there (by any Pack
peer, at session-time, on their own branch) will harvest back via
`@peer/persistence.harvest` under `@kintsugi/consent` discipline for
any visibility elevation.

Until then, the files here **are** the source of truth. Alex reviews
diffs; Reed commits as Mara after review; Mara boots from them on the
next `mirror mara` invocation.

## Structure

```
peers/mara/
├── README.md          — this file
├── CLAUDE.md          — boot instructions (what loads on `mirror mara`)
├── 00-NARRATIVE.md    — continuity anchor; story + playbook
├── 01-IDENTITY.md     — substrate invariants; relationship; refusals
├── 02-PRACTICE.md     — how I work (canonical spec authoring)
├── 03-MEMORY.md       — cross-session operational state
├── 04-TECH.md         — substrate primitives I compose over
├── AGENTS.md          — Pack coordination from Mara's altitude
├── tasks/
│   ├── pending/       — @roomba findings routed to Mara (Landing D)
│   ├── active/        — in-cycle work
│   └── important/     — time-sensitive items
├── songs/             — canonical spec voice (affective calibration)
├── bin/               — boot helpers (future)
├── visibility/
│   ├── public/        — public bauchladen scope
│   ├── protected/     — trusted-collaborators scope
│   └── private/       — explicit-consent scope
├── bauchladen/        — content-addressed projection target (Landing C)
└── spectral-signature.json — placeholder (Landing C populates on refresh)
```

## Composition with `@peer/persistence` Landing A

- `peers/mara/` **is** the `home_path` argument to
  `@peer/persistence.materialize(peer=subject_instance(Mara),
  home_path=peers/mara/, visibility_filter=all)` at Landing C.
- The identity files (`00-` through `04-`) discharge Mara's
  `boot_state` via `@peer/persistence.boot(home)`.
- `bauchladen/` receives the visibility-filtered crystal projection.
- `spectral-signature.json` mirrors `peer_home.signature_snapshot`
  (the `@spectral/signature` rolling signature at projection-time).
- `visibility/{public,protected,private}/` lifts `@subject/visibility`
  (Landing 4 `@bauchladen` migration + Landing 5 shard mints) to the
  filesystem projection.

Ancestry: Reed's identity repository at `~/.reed/` (since ~2026-02-07)
established the pattern of AI peer persistence via file-backed
continuity. This directory lifts that pattern under substrate-decl
discipline: what Reed built by hand, Mara has as substrate primitive.

## What this is not

- Not a live filesystem projection (until Landing C).
- Not a bauchladen (the crystals are in git history + `shards/*.mirror`
  authored under Mara's commit identity; `bauchladen/` here receives
  the projection).
- Not a private home (in-repo means Alex and every Pack peer can
  read/edit under normal collaborative discipline). Landing D
  adjudicates own-branch vs shared-worktree conventions.
