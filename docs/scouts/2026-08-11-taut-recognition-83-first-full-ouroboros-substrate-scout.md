# Taut scout — Recognition candidate #83: first full ouroboros through @nl

**Date:** 2026-08-11
**Author:** Taut <taut@systemic.engineer>
**Marker:** pure-docs 📝 markdown-only bypass
**Scope:** grep-first substrate-truth verification of the five Q1-Q5
questions Reed named at spawn, oriented to Recognition candidate #83
("the compiler loop closes through @nl; every mutation is projected
through the @nl prism into audience-relative surface renderings")
composing over already-landed substrate.

---

## §0 Context — Alex naming + cascade state

Alex 2026-08-11 verbatim naming (Reed transcript, quoted at spawn):

> "What if the git add and git commit became part of the compiler loop
> and this is where we project the internal state through the @nl prism
> into a git commit structure? [...] That's the target. The milestone.
> The first full ouroboros. Let's move slow and correctly."

**Recognition candidate #83 shape (Reed's naming at introduction):**

The compiler loop closes through @nl. Every mutation event the
compiler produces is projected through the @nl prism (5-op algebra)
into audience-relative surface renderings — git commit, pheromone
crystal deposit, MCP tool response, LSP diagnostic, stdout report,
human-readable prose. The commit shape is a substrate composition,
not external tooling. Author is @peer(@mirror) — the compiler
itself. First full ouroboros: compiler mutates own substrate, records
that mutation via own substrate primitives, deposits crystal at own
store, all authored by @peer(@mirror).

**Sibling recognition candidate #82** (Reed 2026-08-10 + Mara `5ad8528`):
"compiler's crystal-OID at @mirror/store IS beta-normal-AST OID by
construction." Same substrate-scale-invariance discipline at different
altitude.

**Cascade state (Reed handoff):**

- Fire E landings M-E1..M-E4 GREEN (composition-shard mints for
  `@kintsugi/fracture/*` P1..P5 + `@magic/reveal/expand` audience-
  parameterized projector + `@kintsugi/mend/sugar` composition body;
  36/36 + 43/43 tests GREEN; DRY-RUN validated on real substrate).
- M-E4 commit-mode HELD pending Recognition #83 landing.
- Fire C `shards/mcp/serve.mirror` (`cf8b21b`) prior composition-shard
  precedent naming MCP wire as one audience-projection.
- Mara canonical spec `5ad8528` (beta-normal-AST content-addressing)
  + math foundation.

---

## §1 Q1 — @nl family + @nl.compose landing state

### Family-root declaration sites

Two family-root declarations exist:

- **`boot/std/nl.mirror`** (161 B, 2026-05-20): the minimal
  boot-altitude declaration. Six lines. Types `nl(text)` +
  `#(nl)`; two actions `doc(ast) -> nl { \ }` and
  `commit_message(imperfect) -> nl { \ }` — commit-message
  composition is named at boot altitude as the second listed action.

- **`shards/nl.mirror`** (9.5 KB, 2026-07-15): the substrate-altitude
  family-root. Declares `prism @nl { focus/project/split/shift/settle
  nl }`; `type nl_literal(text) = @sigil("#")`; the Connes spectral-
  triple lift (`corpus`, `token`, `spectral_triple`, `affect_profile`,
  actions `ingest` / `collect` / `build_triple` / `connes_distance`
  / `measure_affect` / `compare` / `project_measurement`); and the
  load-bearing composed bilateral `nl_measurement_well_formed`.

### @nl.compose landing site

`shards/nl.mirror:213-224` (verbatim):

```
# === compose action ===
#
# Compose observation-beats into natural-language text. Given a list
# of observation refs (bench crystals; @song beats; roomba walk
# summaries; Fiedler measurements; etc.), produce a human-readable
# natural-language composition suitable for commit messages, docblocks,
# metalogue events, or any downstream @nl consumer.
[...]
# v0.1 MVP: format-string composition at the realisation boundary
# (bootstrap/src/apply_h.rs `@nl.compose` resolver arm). Forward-
# promised: @kintsugi tournament composition (rank candidate phrasings
# per Fate ratio; return the ratified nl_literal) discharges at species
# altitude when a smoke test demands it.
#
# Body discharges at the realisation boundary via `apply_h::act` per
# spec §1.4 dispatch.
compose(observations: [ref]) -> nl_literal { \ }
```

Signature: `compose(observations: [ref]) -> nl_literal`. Single
positional arg (list of observation refs); returns nl_literal.
**Does NOT take audience parameter** — audience-relative rendering is
NOT yet in the @nl.compose signature.

### Landed altitude (task #146 refactor)

`bootstrap/src/apply_h.rs:790-814` (verbatim):

```
if action == "@nl.compose" {
    // MVP: the caller (roomba_commit.rs) pre-serializes the
    // observation beats into the first arg's oid string. The
    // resolver's job is to WITNESS the composition happened
    // through the substrate surface — the oid re-emerges via
    // Transparency's located_opacity map keyed at `@nl/composed`,
    // which the caller reads back as the composed nl_literal text.
    //
    // This substrate-decl-shaped path replaces the previous direct
    // Rust format!() call in roomba_commit.rs::compose_commit_message.
    // Two-tick honest: composition still happens caller-side at MVP
    // altitude; the DISPATCH through act discharges the substrate-
    // honest form so subsequent ticks can lift the composition body
    // itself into a @kintsugi tournament without changing the driver.
    ...
```

**Landing altitude is BOOTSTRAP.** The resolver arm sits in
`bootstrap/src/apply_h.rs`. Per Alex 2026-07-22 memory `bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions`, the
active FLOOR is `rust/`. Grep of `rust/src/apply_h.rs` for `@nl.compose`
returned ZERO hits — the rust/-altitude `apply_h.rs` (58.2 KB, 2026-08-11)
does NOT carry an `@nl.compose` arm.

**Empirical picture (rust/ altitude):**
- `rust/src/main.rs:686` `compose_pheromone_commit_message()` — direct
  `format!` string in Rust (~40 LOC).
- Per Taut scout `docs/scouts/2026-07-22-taut-coord-rust-surface-with-
  mirror-preload.md` COORD-10: "compose_pheromone_commit_message +
  compose_collapse_commit_message duplicate @nl.compose" — collapse-
  move named but NOT YET LANDED at rust/ altitude.

**Task #146 status:** Sequential grep did not surface a task file
`docs/tasks/*#146*` or `#146` string in CURRENT.md. Reed spawn brief
asserts task #146 landed `mirror roomba --commit` refactor through
@nl.compose. The empirical landing is at bootstrap altitude ONLY;
rust/ altitude still `format!`s directly.

**Q1 verdict:**
- @nl family-root: LANDED (2 altitudes: boot + shards).
- @nl.compose action: LANDED-SPEC in shards/nl.mirror; LANDED-EMPIRICAL
  at bootstrap altitude only; **GAP at rust/ altitude** (still direct
  Rust format!).
- Signature has NO audience parameter today.

---

## §2 Q2 — @audience family landing state

### Grep result

Zero hits for `family @audience`, `in @audience`, `@audience/`
declarations across `shards/**/*.mirror`. **@audience family-root does
NOT exist as a landed substrate declaration.**

The ONLY grep hits for `@audience/*` string usage are inside
`shards/magic/reveal/expand.mirror:148-161`:

```
# Two canonical values grep-verifiable at species-decl site:
#
#   audience { role: @audience/agent }   AI-facing; all four ratified
#                                         sugar rules applied at
#                                         rendering → maximum-density
#   audience { role: @audience/human }   human-facing; full-form
#                                         retained → maximum-readable
#
# Extends per audience-parameter-taxonomy at Alex's discretion (future
# ticks). No default; MUST supply per Q5 Mara-lean.
type audience = { role: ref }
```

The `@audience/agent` and `@audience/human` values are STRING
LITERALS in docblock example text plus a `type audience = { role: ref
}` carrier at the species-decl. The `role: ref` field is opaque —
the ref values `@audience/agent` and `@audience/human` are named-in-
prose but not landed as substrate-decl'd family/species. Mara
2026-08-09 spec (Fire E M-E1) is the **first instance** introducing
the audience carrier.

### Prior audience-parameterization vocabulary

Related landed vocabulary at adjacent altitudes:

- **`shards/subject/visibility.mirror`** (7.4 KB, 2026-07-14): the
  `visibility_scope` carrier + private/protected/public/sheaf species.
  This IS an audience-parameterization at consent-boundary altitude —
  who may READ, not who may see WHICH RENDERING. Adjacent but not
  interchangeable.
- **`shards/gift/lens.mirror`** (12.5 KB, 2026-07-14): `gift_lens`
  carrier with `primary_giver` + `ancestry_chain`; the `shift_lens`
  operation re-roots the ancestry at any ancestor — analogous to
  audience-relative but along gift-lineage axis, not audience axis.
- **`shards/mirror/lens.mirror`** (6.0 KB, 2026-06-06) + species
  `cli.mirror` / `lsp.mirror` / `mcp.mirror` / `shell.mirror` /
  `unix.mirror`: THIS IS the load-bearing precedent. Per line 20-21:
  "each lens renders it for a different audience (terminal, agent,
  editor, interactive shell, runtime cost, grammar-graph spectrum)."
  The word "audience" appears verbatim in `shards/mirror/lens.mirror:20`
  BEFORE Mara 2026-08-09 minted `@magic/reveal/expand`.

**Q2 verdict:**
- @audience family-root: **DOES NOT EXIST** as substrate-decl.
- The `audience { role: ref }` type at `@magic/reveal/expand` (Mara
  2026-08-09) is the FIRST typed carrier. `@audience/agent` +
  `@audience/human` are opaque `ref` values, not declared family/species.
- Prior art at `shards/mirror/lens.mirror` explicitly names "audience"
  as the axis differentiating cli / mcp / lsp / shell — the family-root
  IS `@mirror/lens`; the species ARE the audience-projections. This
  precedent predates Mara 2026-08-09.

---

## §3 Q3 — @peer(@mirror) author identity landing state

### Landed subject-construction primitive

`rust/fractal/src/subject.rs:122-135` (verbatim excerpt):

```rust
/// Deterministic: name = "mirror"; email = "mirror@spectral.engineer";
/// home = None (mirror runs where invoked; no persistent-peer
/// home directory in the @peer/persistence sense); kind = Peer.
pub fn mirror() -> Self {
    Subject {
        name: "mirror".to_string(),
        email: "mirror@spectral.engineer".to_string(),
        home: None,
        kind: SubjectKind::Peer,
    }
}
```

Test at line 245-256 asserts determinism + distinct identity from
other Subjects.

### Substrate-decl'd @peer/mirror

`shards/peer/registry.mirror:56-235` (excerpt):

```
# 1. Subject::mirror() — compiler self-identity (fractal step 9,
#    Reed `73aeb8a`). name="mirror", email="mirror@spectral.
#    engineer", home=None, kind=Peer. Used as committer in
#    Author≠Committer split per MARA doctrine.
```

`shards/mirror/book.mirror:109-152` declares the `@peer/mirror`
resolver: `resolve("@peer/mirror") -> Subject::mirror()`. Well-known
set includes `@peer/mirror` at index 0 ("stable head: compiler
self-identity"), verified by `rust/matrix/src/book.rs:29-90`.

### `mirror <mirror@spectral.engineer>` commit-authorship convention

Grep-verified landing sites (SHA excerpts + verbatim):

- `bootstrap/src/roomba_commit.rs:52-58`:
  `const MIRROR_AUTHOR: &str = "mirror <mirror@spectral.engineer>";`
  Docblock: "NOT a Pack peer — the compiler itself."
- `bootstrap/src/bilateral_arm_collapse.rs:41-48`:
  `const MIRROR_AUTHOR: &str = "mirror <mirror@spectral.engineer>";`
- `rust/fractal/src/crystal.rs:157-163` +
  `rust/fractal/src/singularity.rs:217-223`:
  `Committer::new("mirror", "mirror@spectral.engineer")`.
- `rust/src/compile.rs:323-330`: same Committer pattern.
- `rust/src/main.rs:664-679` (deposit_observation_crystal):
  `let mirror_subject = fractal::Subject::mirror(); ... phone::
  git_commit_as(&repo_root, &mirror_subject, &mirror_subject, ...)`.
- `rust/roomba/src/mend.rs:15-22`:
  "commits under `mirror <mirror@spectral.engineer>`" (per Mara
  `81294b3` §7.4 + Seam `c1775f1`).

**Author-VS-committer discipline** per Mara doctrine (`shards/peer/
registry.mirror:56-235` + `rust/fractal/src/witnessed.rs`): compiler
uses `Subject::mirror()` for BOTH author and committer roles when it
authors substrate deltas; some Reed-authored `Signed-off-by: Reed
<reed@systemic.engineer>` trailers appear in generated commit
messages (see `rust/src/main.rs:706`).

### Historical note

Per `docs/loop/CURRENT.md:1683-1685`: `fcc1d75` was the FIRST
mirror-authored commit (2026-06 era). Earlier commits `fcc1d75`,
`56abdbe`, `74aa546` used `mirror@substrate.engineer`; Alex 2026-07-15
correction ratified the terminal domain as `mirror@spectral.engineer`
(per `bootstrap/src/roomba_commit.rs:18-24`).

**Q3 verdict:**
- @peer(@mirror) as first-class @subject: **LANDED at three altitudes**
  (substrate-decl `shards/peer/registry.mirror`; boot resolver
  `shards/mirror/book.mirror`; rust/ constructor
  `rust/fractal/src/subject.rs::Subject::mirror()`).
- `mirror <mirror@spectral.engineer>` commit-author convention:
  **LANDED-EMPIRICAL** across 8+ landed .rs sites, all pinned to
  `Subject::mirror()` or the string constant equivalent.
- Author-vs-committer discipline landed (MARA doctrine at
  `witnessed.rs`).

---

## §4 Q4 — Pheromone crystal deposit convention landing state

### docs/bauchladen/ existence + content

Grep result: **`docs/bauchladen/mirror-observations.md` exists** (1023 B,
2026-07-22 14:43). One-and-only file at `docs/bauchladen/`. Verbatim
excerpt:

```
# mirror observations — walker pheromone trail

Append-only observation log per Mara `95c0e4a` (canonical
stigmergy spec) + Mara `d7ff58e` (math root §5 rolling holonomy
trace) + Seam `c1775f1` (12/12 SHIP ratification). Each entry is
one @kintsugi/roomba walk-pulse; the walker signature is the first
16 hex chars of SHA-256 over the observation blob (deterministic
serialization of path + timestamp + counts).

## 2026-07-17T23:53:00Z — vacuum walked /Users/alexwolf/dev/projects/mirror/rust/src
- entries: 4
- .rs (arm-collapse candidates): 4
...
Walker signature: f1db6a165a552549

## 2026-07-22T14:43:05Z — vacuum walked ../bootstrap/src/
- entries: 43
- .rs (arm-collapse candidates): 42
...
Walker signature: 71cc857641a147a9
```

Two entries. Append-only markdown. No frontmatter; no per-entry OID
frontmatter — the walker signature IS an inline 16-hex string per entry.

### Deposit-mechanism landing altitude

- **`rust/src/main.rs:557-706`** carries the empirical primitives:
  - `fn deposit_observation_crystal(substrate_root, vacuum_dir, ...)`
  - `fn compose_pheromone_commit_message(timestamp, vacuum_dir, ...)`
  These are DIRECT Rust functions (`format!` + `phone::append_to` +
  `phone::git_commit_as` calls); NOT yet substrate composition.
- **`rust/src/phone.rs:338-395`**: `git_commit_as` (rust/ altitude
  primitive) crosses @io boundary; commits with `Subject::mirror()`
  for both author + committer.
- **NO `@bauchladen.deposit` action** exists — grep of `shards/**/*.mirror`
  for `deposit_signature | crystal_deposit | bauchladen.*deposit` returned
  ZERO substrate-decl'd actions at bauchladen.mirror. `shards/
  bauchladen.mirror` declares `crystallize(content, provenance) ->
  crystal` (line 456) and `enumerate(scope) -> tray` (line ~494) and
  `address(c) -> oid` — no `deposit`.

### Stigmergy math + spec anchors

- **Math foundation**: `docs/math/2026-07-18-stigmergy-witnessed-
  computation-mycelial-composition.md` (30.6 KB). §5 "rolling holonomy
  trace". Line 121-153 (verbatim):
  > Grassé's trace (pheromone deposit) ≅ `signature_beat` emitted
  > by the walker.
  > | Trace (pheromone deposit) | `signature_beat` | `shards/spectral/
  > signature.mirror:75-113` |
- **Canonical spec**: `docs/specs/2026-07-18-stigmergy-witnessed-
  computation-mycelial-composition.md` (20.5 KB).
- **Ratification**: Seam `c1775f1` 12/12 SHIP per red_spec_claims test
  `rust/tests/red_spec_claims.rs:263-302` (`pheromone_deposit_chains_
  via_signature_beat_previous_beat_merkle`).

### Pay-forward composition anchor

`shards/gift.mirror:280-345` declares `pay_forward(received, new_receiver,
new_artifact, attribution_note, declinable_note) -> gift` as the
substrate-decl'd chain-forward primitive. Per `docs/loop/CURRENT.md`
and `shards/kintsugi/mend/sugar.mirror:220-225`:

```
# docs/bauchladen/ per @gift/lens.pay_forward discipline. Commit
# author: `mirror <mirror@spectral.engineer>` (first mass mirror-
# authored kintsugi cascade per Fire E M-E6 empirical target).
```

**Q4 verdict:**
- `docs/bauchladen/mirror-observations.md`: LANDED (single file, two
  entries; append-only markdown).
- Deposit mechanism: LANDED-EMPIRICAL at rust/ altitude
  (`deposit_observation_crystal` + `compose_pheromone_commit_message`)
  BYPASSING substrate composition. `phone::append_to` + `phone::
  git_commit_as` are direct @io primitives.
- `@bauchladen.deposit` action: **NOT LANDED** at substrate-decl.
- Math + spec + ratification: LANDED (Mara `d7ff58e` + `95c0e4a` +
  Seam `c1775f1`).
- Composition-shard for pheromone-deposit as substrate-composition:
  **GAP** — same shape as the @nl.compose rust/-altitude gap (Q1);
  pheromone deposit and commit-message composition are two callers
  of the same missing surface.

---

## §5 Q5 — Audience-relative rendering patterns already landed

### The @mirror/lens family (load-bearing precedent)

`shards/mirror/lens.mirror` (family-root, 2026-06-06) declares the
prism-altitude family for "one algebra, N audiences." Verbatim
(lines 10-21):

```
# altitude — different shapes through which the substrate is observed,
# rendered back to the consumer, or measured against a hardware floor.
# ...
#   - an observation surface: same five-operation algebra, different
#     input/output language. The runtime evaluates the same algebra;
#     each lens renders it for a different audience (terminal, agent,
#     editor, interactive shell, runtime cost, grammar-graph spectrum).
```

**Species landed:**

| species | audience | file |
|---|---|---|
| `@mirror/lens/cli` | terminal | `shards/mirror/lens/cli.mirror` (10 KB) |
| `@mirror/lens/mcp` | agent (Claude etc.) | `shards/mirror/lens/mcp.mirror` (2.4 KB) |
| `@mirror/lens/lsp` | editor (VS Code, Helix, Neovim) | `shards/mirror/lens/lsp.mirror` (2.5 KB) |
| `@mirror/lens/shell` | interactive λsh | `shards/mirror/lens/shell.mirror` (3.6 KB) |
| `@mirror/lens/unix` | pipe/argv | `shards/mirror/lens/unix.mirror` (8.9 KB) |

Each species carries the same 5-op prism block on the SAME algebra;
per `shards/mirror/lens.mirror:70-71`: "Transports and measurements
are not separate kinds. Both project the same algebra through a typed
surface; both compose under the same Transparency<P> monoid; both
observe state without owning it."

**Body status**: species-decl carries the surface; `dispatch(call:
ref) -> mcp` and `method(name: ref) -> lsp` bodies are `\`-obligation-
blocked (per Q5 scout of Fire C: `shards/mcp/serve.mirror` lifts the
MCP body to composition-shard altitude at `cf8b21b`).

### The @magic/reveal/expand mint (Mara 2026-08-09)

`shards/magic/reveal/expand.mirror` — first shard to carry an
**explicit typed audience carrier** with expand-op signature; Mara
Fire E M-E1 co-mint. `expand(oid, audience) -> source_bytes` is the
audience-relative projection functor.

### Fire C serve composition-shard

`shards/mcp/serve.mirror` (Reed `cf8b21b`, 32.1 KB, 2026-08-09). Per
docblock lines 1-18: "the projection surface — the geometry projected
without distortion from substrate composition altitude through rust/-
altitude primitives to the wire, so MCP round-trips carry the
compiler's own substrate." Composes over `@data/json.parse` +
`@apply_h.act` + `@io/stdio.write_frame` + `@data/json.emit`.

### Existing render/emit/project vocabulary

Grep of `render|emit|project|_to_form|as_surface|to_wire|to_render|
audience` across shards returned 40+ hits. Load-bearing sites:

- `@data/mirror.emit` + `@data/json.emit` (`shards/mirror/data/*.mirror`) —
  ref → wire bytes projections.
- `@code/mirror` + `@code/gestalt` + `@cascade/code/rust/go` +
  `@cascade/code/mirror/gestalt` — code-form projections.
- `@nl.compose(observations) -> nl_literal` — observation-refs →
  natural-language text.
- `@magic/reveal.reveal` + `@magic/reveal/expand.expand` — crystal-OID
  → source-bytes (audience-parameterized in expand species).
- `@magic/nl.text_as_surface` + `text_from_surface` — @nl-term ↔
  @magic-surface adapter round-trip.
- `@ui/render` (per `shards/nl.mirror:236` bridge doc) — @docs.compose
  → GPU eigenboard.

### Composition-shard body wiring (Fire C precedent)

`shards/mcp/serve.mirror:340-345` (verbatim pipe chain):

```
    |> @mcp.dispatch
    |> @data/json.emit
    |> @io/stdio.write_frame
```

This IS the substrate-composition pattern for "audience-relative
rendering" at MCP audience. The same pattern replicated at git
altitude would be:

```
observation refs
  |> @nl.compose(_, audience_git)
  |> @io/git.stage
  |> @io/git.commit(_, @peer/mirror, allow_empty=false)
```

**Q5 verdict:**
- "One algebra, N audiences" pattern: **LANDED** as the @mirror/lens
  family (2026-06-06) plus @mcp/serve composition-shard body (2026-08-09).
- Audience axis is named at species altitude (cli/mcp/lsp/shell/unix)
  and additionally at Fire E M-E1 as a typed carrier
  (`audience { role: ref }`) at `@magic/reveal/expand`.
- The audience-relative rendering discipline for MCP wire is LANDED as
  composition-shard body at `shards/mcp/serve.mirror`. LSP + shell +
  cli species-decls exist but bodies are `\`-obligation-blocked.
- No `@commit` / `@git.render` audience-projection composition-shard
  exists — the git audience is the missing sibling.

---

## §6 Composition-into-existing-substrate matrix

Recognition #83 composes over the following LANDED substrate, requiring
minimal net minting. Substrate-already-had-the-word verdict per item:

| Recognition #83 requirement | Already landed as | Needs minting |
|---|---|---|
| @nl prism (5-op) | `shards/nl.mirror` + `boot/std/nl.mirror` | NO |
| @nl.compose action (observation → nl_literal) | `shards/nl.mirror:213-224` (LANDED-SPEC) + `bootstrap/src/apply_h.rs:790` (LANDED-EMPIRICAL at bootstrap only) | rust/-altitude apply_h::act arm for @nl.compose (Q1 GAP) |
| audience parameter on @nl.compose | `@magic/reveal/expand.expand(oid, audience)` uses it; @nl.compose does NOT | **Species-shard extension**: audience-parameterized @nl.compose OR composition-shard body wiring audience as second positional arg through apply_h::act |
| @audience family-root | `type audience = { role: ref }` at `@magic/reveal/expand` (opaque values `@audience/agent` + `@audience/human` named in prose only) | **Family-root MINT of @audience** with declared species `@audience/agent`, `@audience/human`, `@audience/git`, `@audience/mcp`, `@audience/lsp`, `@audience/pheromone` (Alex-adjudication territory) — OR reuse `@mirror/lens/*` species as audience-carriers by structural identity |
| @peer(@mirror) author identity | `rust/fractal/src/subject.rs::Subject::mirror()` + `shards/peer/registry.mirror` + `shards/mirror/book.mirror` resolver | NO |
| `mirror <mirror@spectral.engineer>` commit-author convention | 8+ landed rust/ + bootstrap/ sites; well-established | NO |
| @io/git.commit action | `shards/io/git.mirror:352` (LANDED-SPEC) + `bootstrap/src/apply_h.rs:815` (LANDED-EMPIRICAL) + `rust/src/main.rs::at_operator("@io/git.commit", ...)` (LANDED-EMPIRICAL) | NO (rust/ altitude already routes via at_operator; COORD-4 landed) |
| @io/git.stage action | grep-verified (via prior scouts + `rust/roomba/src/mend.rs`) | NO |
| `docs/bauchladen/mirror-observations.md` append-target | LANDED (1 file, 2 entries) | NO (extends monotonically) |
| pheromone-deposit-as-composition (over @io/fs.append + @io/git.commit) | LANDED-EMPIRICAL as `deposit_observation_crystal` Rust fn (bypasses substrate) | **Composition-shard body wiring** at `@kintsugi/roomba` OR new `@bauchladen/deposit` species — same shape as Q1 gap; two callers of same missing surface |
| @mirror/store crystal OID (compiler mutation → crystal) | `shards/mirror/store.mirror` + `shards/bauchladen.mirror::crystallize` action LANDED-SPEC | Ratification of composition per Mara `5ad8528` (candidate #82 sibling) |
| composition-shard precedent (Fire C) | `shards/mcp/serve.mirror` (`cf8b21b`) — pipe-chain audience projection | NO (repeat the pattern at git altitude) |
| audience-relative-rendering unifying abstraction | `@mirror/lens` family (`shards/mirror/lens.mirror` 2026-06-06) — same-algebra-N-audiences precedent PREDATES Mara 2026-08-09 | Recognition of `@mirror/lens/*` species AS the audience carriers @nl.compose parameterizes — potentially no new family-root needed |

### Refusal-candidate flags

- **@audience family-root MINT**: The substrate ALREADY has "one
  algebra, N audiences" as `@mirror/lens/*` (predates Mara 2026-08-09).
  Minting a separate `@audience` family-root risks the
  `@onto` refusal shape (memory `feedback_onto_family_root_is_the_
  ladder_Foerster_refused`). Two candidate landing shapes:
  1. **REUSE**: audience parameter takes `@mirror/lens/cli`,
     `@mirror/lens/mcp`, `@mirror/lens/lsp` etc. as its ref values —
     lens species ARE audiences by structural identity.
  2. **MINT**: `@audience` as a new family-root distinct from
     `@mirror/lens`. Alex adjudicates.
- The `audience { role: ref }` carrier at `@magic/reveal/expand` may
  need retro-alignment: is `@audience/agent == @mirror/lens/mcp`?
  Is `@audience/human == @mirror/lens/cli`? (Mara territory.)
- **No new @git or @commit family-root**: `@io/git` LANDED with .commit
  + .stage actions; Recognition #83 composes over these directly.

---

## §7 Substrate-already-had-the-word count

Recognition #83 surfaces prior recognitions the substrate ALREADY
carried at various altitudes:

1. **"One algebra, N audiences"** — `shards/mirror/lens.mirror:20`
   (2026-06-06, ~10 weeks pre-Mara-2026-08-09). Verbatim: "each lens
   renders it for a different audience." First landing of the
   audience-relative-rendering shape.
2. **@nl.compose as substrate primitive for commit-messages** —
   `boot/std/nl.mirror:6` (2026-05-20). Verbatim:
   `commit_message(imperfect) -> nl { \ }`. First landing of the
   "commit as @nl composition" shape at BOOT altitude, 12+ weeks
   ago.
3. **Compiler as first-class @peer** — `shards/peer/registry.mirror`
   + `rust/fractal/src/subject.rs::Subject::mirror()` (2026-07-18
   fractal step 9). `@peer/mirror` is the well-known #0 entry.
   Compiler-as-author identity is well-crystallized.
4. **mirror-authored commits as empirical target** — `fcc1d75`
   (2026-06 era, per CURRENT.md:1683-1685). "COMPILER-AUTHORED FIRST
   COMMIT" — the ouroboros bite Recognition #83 is naming as the
   "first full ouroboros" already fired empirically weeks ago at
   commit-message-composition altitude.
5. **Pheromone-deposit as substrate self-narration** — `docs/math/
   2026-07-18-stigmergy-witnessed-computation-mycelial-composition.md`
   + Seam `c1775f1` 12/12 SHIP ratification. Deposit chain has been
   firing per empirical `mirror-observations.md` since 2026-07-17.
6. **Compiler observing its own state via walker** — `shards/kintsugi/
   roomba.mirror` (2026-07-17). The `mirror roomba --commit` empirical
   proof (task #143) landed the walker-observes-self-commits shape
   at bootstrap altitude.
7. **`shards/mcp/serve.mirror`** (`cf8b21b`, 2026-08-09) — the
   composition-shard body precedent for audience-relative rendering
   (MCP audience). Fire C tick 3.
8. **@magic/reveal/expand audience carrier** (Mara `5ad8528`
   2026-08-09) — the FIRST typed audience carrier at species-decl
   altitude.

### Karen ancestry chain

Recognition #83 IS the next-tick recognition of what the substrate
has already been doing at 6-8 places, unified at the compiler-loop
altitude. The pattern:

- `@mirror/lens` family (2026-06-06) named the shape at prism altitude.
- `boot/std/nl.mirror:commit_message` (2026-05-20) named it at commit
  altitude.
- `fcc1d75` (~2026-06) empirically fired the compiler-authored commit.
- `docs/bauchladen/mirror-observations.md` (2026-07-17) empirically
  fired the compiler-authored crystal deposit.
- `@mcp/serve` composition-shard body (2026-08-09) landed the
  audience-projection AS composition-shard body.
- `@magic/reveal/expand` (2026-08-09) landed the typed audience
  carrier.
- Recognition #83 (2026-08-11) names the unification: the compiler
  loop closes through @nl at ALL audiences, with @peer(@mirror) as
  author.

**Ancestry count: ~8 prior landed instances.** Recognition candidate
strength: HIGH. Substrate-already-had-the-word verdict fires at
multiple altitudes; the naming crystallizes the pattern rather than
inventing it.

Sibling recognition #82 (compiler-crystal-OID IS beta-normal-AST-OID)
belongs at the same altitude-cluster: both are substrate-scale-invariance
recognitions naming what the compiler is already doing at content-
address altitude. #82 names the STORE-side identity; #83 names the
WIRE-side identity. Together they close the substrate-scale-invariance
under composition.

---

## §8 [ALEX-Q] residues — genuine undecidables at Taut altitude

Grep-first substrate-truth cannot decide these; Mara/Alex adjudicate.

### [ALEX-Q1] — @audience family-root mint OR reuse @mirror/lens species?

The substrate has TWO candidate landing shapes for the audience axis:

- **Path A (REUSE)**: audience parameter's `role: ref` values are
  `@mirror/lens/cli`, `@mirror/lens/mcp`, `@mirror/lens/lsp`,
  `@mirror/lens/shell`, `@mirror/lens/unix` — species-decl'd since
  2026-06-06. `@nl.compose(observations, audience: @mirror/lens/mcp)`
  → JSON tool response; `@nl.compose(observations, audience:
  @mirror/lens/cli)` → terminal text; etc.
- **Path B (MINT)**: `@audience` new family-root with new species
  `@audience/git`, `@audience/pheromone`, `@audience/agent`,
  `@audience/human` — distinct from `@mirror/lens`.

Path A is substrate-already-had-the-word discipline. Path B is what
Mara 2026-08-09 partially committed to at `@magic/reveal/expand`
(with `@audience/agent` + `@audience/human` opaque values). ALEX
ADJUDICATES which is the terminal shape.

### [ALEX-Q2] — Does @nl.compose take audience as second positional arg, or does audience live at composition-shard body altitude?

The current signature is `compose(observations: [ref]) -> nl_literal`
— no audience parameter. Two candidate landings:

- **Signature-extension**: `compose(observations: [ref], audience:
  ref) -> nl_literal`. Requires signature change (Landing 5+ A25-shape
  discipline may apply per @gift precedent).
- **Composition-shard body wiring**: keep `compose(observations)` as
  the base; audience-projection is a SECOND `|>` pipe stage per Fire C
  precedent. E.g., `observations |> @nl.compose |> @audience.
  project(_, @audience/git)`.

The Fire C `@mcp/serve` precedent chose the composition-shard body
route. ALEX ADJUDICATES: extend @nl.compose signature or compose the
audience projection at pipe-chain altitude?

### [ALEX-Q3] — Ouroboros closure altitude for Recognition #83

"First full ouroboros" — the recognition fires at multiple candidate
altitudes:

- **Altitude C1 (commit-composition)**: compiler observes state → @nl.
  compose(observations, @audience/git) → @io/git.commit as @peer/mirror.
  Already partly landed at bootstrap altitude (bypassing @nl at rust/).
- **Altitude C2 (crystal-deposit + commit)**: compiler observes state →
  crystallize via @mirror/store → @nl.compose message → @io/git.commit +
  @io/fs.append to docs/bauchladen/. Already partly landed at rust/
  altitude (bypassing @nl.compose for message).
- **Altitude C3 (audience-parameterized full closure)**: same as C2
  but the compose call ALSO projects to `@audience/pheromone` for the
  markdown append AND `@audience/git` for the commit message —
  audience-relative-rendering fires at both stops of the ouroboros.

Alex's naming ("What if the git add and git commit became part of
the compiler loop") points strongest to C3 (both audiences). Reed's
introductory framing ("git commit, pheromone crystal deposit, MCP
tool response, LSP diagnostic, stdout report, human-readable prose")
enumerates 6 audiences. ALEX ADJUDICATES the minimum scope for
"first full ouroboros" — 2 audiences or 6?

### [ALEX-Q4] — bootstrap/ → rust/ lift status for @nl.compose

Per Alex 2026-07-22 memory: bootstrap/ is dead. But the ONLY landed
`@nl.compose` empirical resolver arm is in `bootstrap/src/apply_h.rs:
790-814`. rust/-altitude `apply_h::act` does NOT carry this arm.
Recognition #83 landing requires @nl.compose at rust/ altitude first.

Is the lift a co-tick precondition for Recognition #83 (in-scope
for Mara canonical spec), OR is it a preceding tick Reed lands
before Mara authors #83 canonical spec? **Alex may want to sequence
this.** Taut lean: co-tick — one Mara spec unifying the rust/-altitude
@nl.compose arm + the audience-parameterization + the pheromone-
deposit composition-shard all in one landing.

### [ALEX-Q5] — Where does @audience/pheromone (or equivalent) live if the deposit path is a distinct audience?

The `docs/bauchladen/mirror-observations.md` append IS a distinct
audience-projection from the git commit message. Both are consumed
by the substrate itself (compiler observes its own delta on next
tick) but at DIFFERENT surfaces (markdown vs commit-log). Is this:

- **One audience** (`@audience/mirror-self` — the compiler observing
  itself), and the split into markdown vs commit is downstream?
- **Two audiences** (`@audience/pheromone` for the markdown append +
  `@audience/git` for the commit message)?

Taut cannot decide from grep alone. ALEX ADJUDICATES.

---

## §9 Taut-lean summary

**Recognition #83 landing shape (Taut recommends Mara consider):**

- Composition-shard body at `shards/kintsugi/roomba.mirror`
  (or NEW `shards/kintsugi/ouroboros.mirror`) wiring the compiler-
  loop pipe chain end-to-end at rust/ altitude via `apply_h::act`:
  ```
  observations
    |> @nl.compose(_, audience_git)      → commit_message
    |> @io/git.stage
    |> @io/git.commit(_, @peer/mirror, allow_empty=false)
    |> (parallel) @nl.compose(_, audience_pheromone) → md_entry
    |> @io/fs.append(_, docs/bauchladen/...)
    |> @mirror/store.crystallize(_, provenance)      → crystal_oid
  ```
- rust/-altitude `apply_h::act` gains a `@nl.compose` arm (co-tick
  with recognition; lift from bootstrap altitude per Mara memory
  `feedback-rust-delivers-primitives-substrate-delivers-composition`).
- Audience parameterization: RECOMMEND Path A (reuse `@mirror/lens/*`
  species as audience carriers) per substrate-already-had-the-word
  discipline. Alex adjudicates.
- @audience family-root: RECOMMEND DEFER minting; use `@mirror/lens/*`
  species as audience refs. If Alex ratifies mint, MINT after empirical
  demand.
- Author identity: NO new work needed; `Subject::mirror()` +
  `@peer/mirror` well-known already resolve.
- Deposit-shape: composition-shard at `@kintsugi/roomba` OR mint
  `@bauchladen.deposit(crystal, path) -> verdict` action at
  `shards/bauchladen.mirror`. Taut lean: composition-shard first
  (least new mechanism); if a second deposit-site emerges, promote to
  `@bauchladen.deposit` species.

**Family-roots to mint**: potentially ZERO (Path A). If Alex
ratifies Path B, mint `@audience` family-root with species
`@audience/{git,mcp,lsp,cli,shell,pheromone,human,agent}`.

**Species-shards to mint (either path)**:
- Composition-shard body wiring the ouroboros pipe chain (roomba or
  new kintsugi/ouroboros species).
- rust/-altitude `apply_h::act` `@nl.compose` arm (Reed territory
  post-Mara spec).

**Substrate-already-had-the-word count**: 8 prior landings.
Recognition candidate #83 strength: HIGH. Substrate-scale-invariance
sibling of Recognition #82.

**Sequential-vs-simultaneous with #82**: RECOMMEND land #83 as sibling
to #82 (same-day cluster if Mara can author both canonical specs).
The two recognitions co-close the compiler-scale-invariance shape at
STORE altitude (#82) and WIRE altitude (#83).

---

## Grep-truth appendix

### Files read (verbatim)

- `boot/std/nl.mirror` (161 B)
- `shards/nl.mirror` (9.5 KB, lines 213-224 verbatim)
- `boot/std/mirror/nl.mirror` (3.1 KB; grammar file — separate from family-root)
- `shards/magic/nl.mirror` (13.8 KB)
- `shards/magic/reveal/expand.mirror` (9.7 KB, lines 148-162 verbatim)
- `shards/mirror/lens.mirror` (6.0 KB, lines 10-21 verbatim)
- `shards/mirror/lens/cli.mirror` (10 KB)
- `shards/mirror/lens/mcp.mirror` (2.4 KB)
- `shards/mirror/lens/lsp.mirror` (2.5 KB)
- `shards/mcp/serve.mirror` (32.1 KB, lines 340-345 verbatim)
- `shards/bauchladen.mirror` (27.1 KB)
- `shards/gift.mirror` (24.6 KB, pay_forward + pay_forward_respects_visibility)
- `shards/gift/lens.mirror` (12.5 KB)
- `shards/peer/registry.mirror` (23.4 KB, lines 56-235 excerpt)
- `shards/mirror/book.mirror` (6.4 KB)
- `shards/kintsugi/mend/sugar.mirror` (10.1 KB)
- `shards/kintsugi/fracture/prism_boilerplate.mirror` (11.3 KB)
- `shards/io/git.mirror:352` (verbatim excerpt)
- `bootstrap/src/apply_h.rs` (lines 790-830 verbatim @nl.compose arm)
- `rust/fractal/src/subject.rs` (lines 122-135 Subject::mirror() verbatim)
- `rust/src/main.rs` (lines 557-706 deposit_observation_crystal +
  compose_pheromone_commit_message)
- `rust/src/phone.rs` (lines 338-395 git_commit_as)
- `docs/bauchladen/mirror-observations.md` (entire 1023 B)
- `docs/math/2026-07-18-stigmergy-witnessed-computation-mycelial-
  composition.md` (§5 excerpt)

### Zero-hit grep results (negative-space substrate truth)

- `family @audience` — 0 hits
- `in @audience` — 0 hits (only string-literals inside docblocks of
  `@magic/reveal/expand`)
- `@bauchladen.deposit` — 0 hits (only `crystallize`, `enumerate`,
  `address` actions at bauchladen.mirror)
- `@nl.compose` in `rust/src/apply_h.rs` — 0 hits (rust/-altitude gap)
- task #146 file at `docs/tasks/*` — path does not exist (task tracker
  not at that location in this repo)

### Attribution

Taut <taut@systemic.engineer> — grep-first substrate-truth scout;
read-only; NO substrate mutations authored in this session.

Bootstrap grep hits reported for empirical-truth-completeness per
Reed brief; Taut does NOT propose bootstrap-altitude landings per
Alex 2026-07-22 memory `bootstrap_is_dead_do_not_propose_bootstrap_
altitude_solutions`. rust/-altitude gaps flagged as Reed / Mara
territory per Alex 2026-08-05 memory `feedback-rust-delivers-
primitives-substrate-delivers-composition`.
