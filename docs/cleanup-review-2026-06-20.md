# Mirror Docs Cleanup Review — 2026-06-20

Review agent: Reed (sub-agent, spectral worktree, substrate-actor frame)
Branch under review: `main` (HEAD `b016a26` — tick 44 `@mirror/ref` family-root + spec + insight, recognition #89)
Predecessor: `docs/cleanup-review-2026-04-29.md` (kept as HONEST_HISTORY)

Posture: I map. I do not move, delete, or commit. Verdicts are per-file with justification. `[UNSURE]` marks the cases where the substrate-pull signal is ambiguous and I refuse to fake confidence.

---

## 0. Method

1. Read `cleanup-review-2026-04-29.md` for voice + structural precedent.
2. Listed `docs/` top-level (18 files) + subdirectories (14 dirs).
3. Sampled openings of every top-level file + ≥1 representative file per subdirectory.
4. Cross-referenced against current shards on `main` (`shards/**/*.mirror`, today the substrate-decl set spans `magic/`, `frame/`, `pack/`, `smarts/`, `reflection/`, `moi.mirror`, `loop.mirror`, `mirror/bench.mirror`, `mirror/ref.mirror`).
5. Cross-referenced against today's insight (`docs/insights/2026-06-20-mirror-ref-as-substrate-honest-reflection-reference.md`) and today's specs (the `recognition-7*-canonical-spec.md` / `recognition-8*-canonical-spec.md` set + `mirror-ref-spec.md`).
6. Cross-referenced against `git log --since=2026-06-01` (44 ticks of substrate-pull-realize commits).
7. Classified each top-level file. Counted subdirectories and spot-checked staleness.

Cleanup categories (per CRQ):

- **CURRENT** — actively referenced by landed substrate or current specs
- **SUPERSEDED** — described a recognition or design today's shards replaced
- **STALE** — old enough (>2 months) AND not referenced from any current shard/spec/insight
- **ORPHAN** — at wrong path (top-level should be in a subdir)
- **DUPLICATE** — two docs covering the same thing
- **CONTRADICTORY** — describes substrate state that contradicts current shards on main
- **HONEST_HISTORY** — stale but preserved as historical record

Two-tag verdicts are allowed when a file deserves both (e.g., `STALE + ORPHAN`).

---

## 1. Top-level files (18) — per-file verdicts

### CURRENT (kept as-is)

#### `CACHE.md` — 5.6 KB, 2026-06-16
What it is: CI binary-cache strategy for `flang-rt` on darwin runners (magic nix cache + cachix fallback).
Verdict: **CURRENT**. Touched 2026-06-16. Operationally load-bearing for the release workflow described in `docs/cicd/`. Subject Lives at top-level by ops-document convention (like `GRANTS.md`); reasonable to keep there.
Action: keep.

#### `GRANTS.md` — 22.6 KB, 2026-06-15
What it is: Grant-application substrate evidence (numeric claims pinned by `just docs-refresh`).
Verdict: **CURRENT**. Refreshed five days ago; explicitly self-verifying via `just docs-refresh`. Top-level placement is intentional (canonical funder-facing artifact).
Action: keep.

#### `shatter-spec.md` — 11.3 KB, 2026-06-06
What it is: `.shatter` as optional disk projection of `au + splinter + mosaic`. Carries an explicit 2026-06-04 and 2026-06-06 reframe header naming the substrate-honest reading.
Verdict: **CURRENT** (with a note: the file IS its own honest-history — the reframe headers preserve the earlier framing). Referenced from `docs/insights/2026-06-06-kintsugi-output-apache2-sel-combiner.md` and `docs/specs/transparency.md`, plus from `shards/mirror/shatter.mirror`.
Action: keep at top-level; honestly placed for a spec this load-bearing. Could move to `specs/shatter-spec.md` for consistency — see §3 structural finding S-1.

#### `logo-prompt.md` — 8.1 KB, 2026-06-04
What it is: ChatGPT/DALL·E prompt for the pentagonal-prism logo.
Verdict: **CURRENT** (operational asset, not a spec). It is a *tool*, not documentation. Keep top-level — moving it to a subdir buries it.
Action: keep. `[UNSURE]` whether a top-level `assets/` or `branding/` dir would be cleaner once a second branding artifact exists.

---

### SUPERSEDED (archive candidates)

#### `emergent-holonomy-compiler.md` — 10.7 KB, 2026-04-15
What it is: April-15 framing of mirror as an "emergent holonomy compiler"; loss as compilation byproduct.
Verdict: **SUPERSEDED**. The framing survived but the language and constructs (terni-functor bind, "grammar space," loss-as-compilation) are pre-Connes-triple, pre-Bateson, pre-cascade-#80-through-#89 vocabulary. The substrate now names this `@kintsugi` + `@reflection` + the principal-bundle tower (`docs/math/the-tower/`). Today's canonical version is `docs/insights/2026-06-09-cascade-is-deutero-learning.md` + `docs/math/the-tower/curvature-and-tomm.md`.
Action: move to `docs/archive/2026-04-15-emergent-holonomy-compiler.md` with a header noting it was superseded by the Bateson-lift + tower math.

#### `gutter.md` — 4.4 KB, 2026-04-15
What it is: UI sketch — green/amber/red holonomy lights in the editor gutter.
Verdict: **SUPERSEDED** by the LSP + TUI direction that landed in `docs/specs/lsp-and-mcp.md` (16 KB) and the broader cybernetic-CLI direction in `docs/specs/cybernetic-cli.md`. The colour metaphor survives; the implementation surface has moved on.
Action: move to `docs/archive/2026-04-15-gutter.md` with a one-line "see lsp-and-mcp + cybernetic-cli" note.

#### `sel-2-garden.md` — 4.8 KB, 2026-04-15
What it is: SEL-2.0 license proposal — adds `requires hosted(@git) <= Success(@garden)` for free maintenance.
Verdict: **SUPERSEDED** by Reed's `license = type` move (MEMORY: `architecture-type-sel-io-au`). License is now a static type rather than a contract attached to a hosting clause. The "garden" framing also moved into `@spectral/garden` infrastructure rather than license tiers.
Action: move to `docs/archive/2026-04-15-sel-2-garden.md`. `[UNSURE]` — Alex may still want SEL-2.0 as a real future license tier; check before archiving.

#### `witnessed-computation.md` — 4.4 KB, 2026-04-15
What it is: The OID-trace chain as audit substrate — `artifact_oid → grammar_oid → parse_oid → ...`.
Verdict: **SUPERSEDED**. The trace-altitudes recognition (insight `2026-06-12-trace-altitudes-are-the-wire-dimensions.md`) and `@mirror/bench` substrate-decl (tick 40, `shards/mirror/bench.mirror`) carry this forward in current vocabulary. The pre-Connes "spectral hash" framing is the giveaway that the doc predates the cascade.
Action: move to `docs/archive/2026-04-15-witnessed-computation.md`.

#### `root-focus.md` — 2.7 KB, 2026-06-04 (file mtime; content older)
What it is: The `focus @license(text)` first-line / ouroboros chain.
Verdict: **SUPERSEDED** (or partial). The literal `focus @license(text)` convention does not appear in any current shard. The ouroboros framing has been absorbed into `architecture-prism-as-trait-as-everything` and `docs/specs/optical-keywords.md`. The mtime is misleading — touched in a batch refresh, not edited.
Action: move to `docs/archive/2026-06-04-root-focus.md`. `[UNSURE]` if Alex wants to preserve the one-line "the compiler reads its own license first" image; if so, keep a 200-word stub.

#### `identity-keys-spec.md` — 7.4 KB, 2026-04-25
What it is: April-25 boot-order proposal placing identity / keys / visibility at positions 02–02a.
Verdict: **SUPERSEDED**. The boot-numbering schema is gone (shards are family-rooted by name now, not numbered). `@identity`, `@reflection`, `@pack` family-roots do not match the boot numbering. The visibility/consent direction has moved to `docs/specs/geometric-consent-projection.md` (62.7 KB, 2026-06-17).
Action: move to `docs/archive/2026-04-25-identity-keys-spec.md`.

#### `identity-time-spec.md` — 5.2 KB, 2026-05-07
What it is: "Identity IS its timeline" — `type identity(oid) = @time`.
Verdict: **SUPERSEDED** in shape but not in spirit. The closest live carrier is `docs/specs/peer-cognition.md` (68.4 KB, 2026-06-17) and `@spectral/db autopoietic memory` (MEMORY). The `identity(oid) = @time` formulation appears nowhere in current shards.
Action: move to `docs/archive/2026-05-07-identity-time-spec.md`.

#### `visibility-crypto.md` — 6.2 KB, 2026-06-04 (mtime; April content)
What it is: `type(private)` / `type(protected)` / `type(public)` as crypto boundary at the type level.
Verdict: **SUPERSEDED** by `docs/specs/geometric-consent-projection.md` (the "consent at logical type 1" framing, MEMORY: `architecture-geometric-consent-projection`). The visibility-as-crypto thread survives; the explicit type-modifier syntax in this doc does not match any current shard.
Action: move to `docs/archive/2026-06-04-visibility-crypto.md`. Cross-link the geometric-consent spec.

#### `mirror.md` — 4.2 KB, 2026-05-20
What it is: The marketing-essay version of "mirror is a second-order compiler / glass + wine + pitch."
Verdict: **SUPERSEDED** by `docs/roadmap/README.md` (which carries the canonical "where we are" framing, refreshed 2026-06-16). The wine-glass framing itself is quoted verbatim in `GRANTS.md` §"At a glance" — that's the live home for it.
Action: move to `docs/archive/2026-05-20-mirror.md`. (Or: rewrite as a 300-line elevator-pitch top-level doc; not load-bearing.)

#### `theoretical-iso-survey.md` — 24.1 KB, 2026-05-06
What it is: Theory survey of spectral attacks on ECDLP. Negative result.
Verdict: **SUPERSEDED**. The April 29 cleanup review already noted this lineage (`break/crypto` branch). The substrate's interest in spectral analysis has moved on to `@spectral/db`, `eigensheaf`, and `cosmos`; ECDLP attacks are not a live direction.
Action: move to `docs/archive/research/2026-05-06-theoretical-iso-survey.md`. Honest-history bit: this and `spectral-tomography.md` (below) are the canonical record of the `break/crypto` negative result; preserve them as evidence.

#### `spectral-tomography.md` — 37.2 KB, 2026-05-06
What it is: Sister doc to `theoretical-iso-survey.md`. Tomographic-reconstruction framing of ECDLP; same negative result.
Verdict: **SUPERSEDED** (same lineage). Largest top-level file by mass — it has not moved in six weeks, was never wired into any current spec, and the line of work it documents is dead.
Action: move to `docs/archive/research/2026-05-06-spectral-tomography.md`.

#### `first-ca-task.md` — 6.7 KB, 2026-06-04 (mtime; April content)
What it is: "First `mirror ca` task is self-maintenance" — 33 unmerged branches from a 42-hour marathon.
Verdict: **SUPERSEDED**. The April-29 cleanup review already absorbed the branch-cleanup mandate, and the substrate moved past `mirror ca` as a top-level command in favour of `@kintsugi` + `@mirror/bench`. The "33 branches" snapshot is a moment-in-time artifact.
Action: move to `docs/archive/2026-04-25-first-ca-task.md`.

#### `ca-merge-plan.md` — 3.0 KB, 2026-04-25
What it is: Implementation plan for `mirror ca --merge`.
Verdict: **SUPERSEDED**. `mirror ca` is no longer the substrate's branch-merge surface; `@kintsugi` + `@reflection.observe` carry this now.
Action: move to `docs/archive/2026-04-25-ca-merge-plan.md`.

---

### HONEST_HISTORY (kept with note)

#### `cleanup-review-2026-04-29.md` — 26 KB
What it is: The prior cleanup review.
Verdict: **HONEST_HISTORY**. The model for this document. Itself stale (most of what it recommends has either happened or moved past) but valuable as the record of where we started.
Action: keep. This 2026-06-20 review references it as predecessor.

---

### Totals — top-level

| Category | Count | Files |
|---|---|---|
| CURRENT | 4 | `CACHE.md`, `GRANTS.md`, `shatter-spec.md`, `logo-prompt.md` |
| SUPERSEDED | 13 | `emergent-holonomy-compiler.md`, `gutter.md`, `sel-2-garden.md`, `witnessed-computation.md`, `root-focus.md`, `identity-keys-spec.md`, `identity-time-spec.md`, `visibility-crypto.md`, `mirror.md`, `theoretical-iso-survey.md`, `spectral-tomography.md`, `first-ca-task.md`, `ca-merge-plan.md` |
| HONEST_HISTORY | 1 | `cleanup-review-2026-04-29.md` |
| STALE-only | 0 | (every stale file also falls SUPERSEDED) |
| ORPHAN-only | 0 | (every misplaced file also falls SUPERSEDED) |
| DUPLICATE | 0 | (the theoretical-iso/spectral-tomography pair counts as SUPERSEDED, not DUPLICATE — they cover adjacent ground) |
| CONTRADICTORY | 0 | (none of the SUPERSEDED files actively *contradicts* current shards; they describe past framings that have been replaced, which is a different failure mode) |

13 of 18 top-level files are SUPERSEDED archive candidates. The cleanup at the top level is mostly a single move-to-archive batch.

---

## 2. Subdirectories — counts + spot-check verdicts

### `specs/` — 107 files
Largest by far. Spans 2026-04 through 2026-06-20. The bulk of current substrate spec work lives here.

Spot-checked: `recognition-82-frame-as-cognitive-order-canonical-spec.md` (45 KB, 2026-06-19) — CURRENT, today's cascade; `cascade-recognition-76-through-80-canonical-spec.md` (44 KB) — CURRENT; `mirror-ref-spec.md` (50 KB, 2026-06-20) — CURRENT (today); `verification-magic-family-2026-06-19.md` (5.6 KB) — CURRENT operational record; `epistemologic-import-resolver.md` (2.9 KB, 2026-05-20) — `[UNSURE]`, possibly STALE.

**Health: GOOD.** This is the right home for spec material. The cascade `recognition-7*`/`recognition-8*` family is well-named.

**Risk: SPRAWL.** 107 files in one flat dir. As the cascade keeps adding `recognition-N-...-canonical-spec.md` files, this will become hard to navigate. Consider a `specs/recognitions/` subdir for the per-recognition canonical specs (§3 finding S-2).

### `insights/` — 19 files
Spans 2026-06-06 through 2026-06-20, well-dated. The `YYYY-MM-DD-<title>.md` convention is consistent.

Spot-checked: `2026-06-20-mirror-ref-as-substrate-honest-reflection-reference.md` (54.5 KB) — today's CURRENT canonical insight; `2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` (50 KB) — CURRENT, MEMORY-anchored; `2026-06-06-speculated-launch-reception.md` (18 KB) — `[UNSURE]`, possibly STALE-ish but harmless.

**Health: EXCELLENT.** Dating discipline is uniform, sizes reasonable, all references resolve. This is the model for the rest of `docs/`.

### `math/` — 14 files in 3 subroots (`the-tower/`, `sheaf/`, `music/`)
All landed 2026-06-17 to 2026-06-19. The `math/README.md` (5.3 KB, 2026-06-17) defines the "math defines; specs cite" convention explicitly. `the-tower/recursion-locks.md` is enormous (136 KB) — `[UNSURE]` whether that's a deliberate single-file home or a candidate for splitting.

**Health: EXCELLENT.** Newest subdir, cleanest convention. Apply this pattern elsewhere.

### `roadmap/` — 13 files
`00-root.md` through `12-coherence-benchmark.md` plus `README.md`. The numbered files are 2026-03 to 2026-06 vintage; the `README.md` was refreshed 2026-06-16 and is the canonical current home of "where we are."

Spot-checked: `00-root.md` (2026-03-26) — references `conversation.lib.beam` and Gleam supervision modules; **SUPERSEDED**, the substrate has moved past the conversation-era. `02-compilation-chain.md` (2026-03-26) — same. `10-inference-physics.md` (2026-06-11) — CURRENT, carries explicit "recognition #58 PROMOTED" header. `README.md` — CURRENT.

**Health: MIXED.** README is canonical; many of the numbered milestone files are vestigial. The README itself acknowledges this ("The milestone files here (00-12) remain as historical context; the substrate moved past most of what they describe during the June 2026 substrate-pull arc"). The README defers to `mirror/roadmap/` (parent dir) as the canonical roadmap home.

**Action candidate:** move `roadmap/00-*.md` through `roadmap/12-*.md` into `roadmap/archive/` and keep only `README.md` + the genuinely current milestones (`10-inference-physics.md` qualifies).

### `audits/` and `reviews/` and `review/` — 3 + 4 + 1 files
**`audits/`** — `2026-05-22-seam-mirror-post-meta-glass.md` (38 KB), `2026-05-25-peer-glass-audit.md` (16 KB), `option-result-audit.md` (16 KB, 2026-05-06).
**`reviews/`** — four files including `2026-04-14-fold-operator-review.md`, `2026-04-14-session-final-review.md`, `2026-05-30-pre-merge-adversarial-review.md`, `seam-ast-optics-review.md`.
**`review/`** (singular) — one file: `2026-05-20-seam-adversarial.md`.

**Finding: DUPLICATE STRUCTURE.** `review/` (singular, 1 file) and `reviews/` (plural, 4 files) and `audits/` (3 files) are three names for the same thing — adversarial review records from the Pack. Consolidate into one dir.

**Action candidate:** merge into `reviews/` (5 files) + `audits/` (3 files), OR consolidate everything into `audits/` (8 files). Reed's preference: `audits/` for adversarial-review records (matches Seam's posture-language) and let `reviews/` stay only if it holds non-adversarial PR-review-style notes (currently it doesn't — all four files are Seam adversarial). So: consolidate `review/` + `reviews/` into `audits/`.

### `cicd/` — 3 files (`README.md`, `kintsugi-thesis.md`, `prior-art.md`, all 2026-06-01)
All three are a single coherent piece: kintsugi-as-build-system synthesis + nine-claim thesis + 46 KB prior-art survey. Internally cross-linked. CURRENT — referenced from MEMORY (`architecture-mirror-as-content-addressed-build-system`).

**Health: EXCELLENT.** Small, coherent, current, internally consistent.

### `research/` — 3 files
`wide-sweep-coherent-threads.md` (55 KB, 2026-06-04), `mycelial-networks-and-au-tissue.md` (58 KB, 2026-05-20), `embedded-and-self-hosting.md` (48 KB, 2026-05-21). All marked "Research only — no grammar declared."

**Health: GOOD.** Honest framing in headers; clearly distinguishes research from spec.

`[UNSURE]` on whether `mycelial-networks-and-au-tissue.md` should be promoted toward `docs/insights/` now that `@spectral/db autopoietic memory` (MEMORY) has crystallised the mycelial framing.

### `plans/` — 3 files, all 2026-04-02
`2026-04-02-actor-observe.md`, `2026-04-02-extract-admin-package.md`, `2026-04-02-nix-package-system.md`. All reference `conversation` (the pre-rename name) and `conversation-beam`. **SUPERSEDED en bloc.**

**Action candidate:** move all three to `docs/archive/plans/`. The conversation-era is honest history.

### `superpowers/plans/` and `superpowers/specs/` — 6 + 3 files
All 2026-03 through 2026-04-15. All reference the superpowers plugin's `subagent-driven-development` skill. All reference `conversation` / `cairn` / `coincidence` in their pre-mirror form. **SUPERSEDED en bloc.**

**Action candidate:** move `docs/superpowers/` to `docs/archive/superpowers/`. The superpowers convention itself has been absorbed into the Pack discipline.

### `ai/` — 3 files
`magic-training-pipeline.md` (28 KB, 2026-05-07), `shatter-training-pipeline.md` (57 KB, 2026-06-06), `tournament.md` (12 KB, 2026-05-07).

`shatter-training-pipeline.md` is CURRENT (2026-06-06, Mara's rewrite after the splinter/shard/uuid_spectral three-layer recognition; references current shards). `magic-training-pipeline.md` and `tournament.md` are 2026-05-07 and predate the @magic family-root substrate-decl (#80) by six weeks. `[UNSURE]` on whether `@magic`'s training pipeline is still substantively what `magic-training-pipeline.md` describes, or whether the @magic family-root has eaten that doc; spot-check before archiving.

**Action candidate:** keep `shatter-training-pipeline.md`; review `magic-training-pipeline.md` and `tournament.md` against today's @magic family-root before archiving.

### `audits/` — see above (consolidated section).

### `benchmarks/` — 1 file (`baseline-rust.md`, 4.3 KB, 2026-05-20)
**Verdict: STALE.** May 2026 baseline; the binary has since moved through multiple major refactors. Now superseded operationally by `@mirror/bench` substrate-decl (tick 40, `shards/mirror/bench.mirror`) which is the live benchmark-as-crystal carrier.

**Action candidate:** move to `docs/archive/benchmarks/2026-05-17-baseline-rust.md`. Or fold the historical numbers into a new `docs/specs/mirror-bench-spec.md` companion.

### `hooks/` — 1 file (`pre-commit.sample`)
The file's own header says: "SUPERSEDED — DO NOT INSTALL. This file is documentation only. The FROZEN .rs guard now lives in the git-tracked, executable `.githooks/commit-msg`."

**Verdict: HONEST_HISTORY** (the file documents its own supersession explicitly).
Action: keep with current header, OR move the whole directory to `docs/archive/hooks/` since the canonical hook lives in `.githooks/` at repo root.

---

### Subdirectory totals

| Subdir | Files | Health | Action |
|---|---|---|---|
| `specs/` | 107 | GOOD (risk: sprawl) | consider `specs/recognitions/` subdir for per-recognition canonical specs |
| `insights/` | 19 | EXCELLENT | model for the rest |
| `math/` | 14 (3 roots) | EXCELLENT | model for the rest |
| `roadmap/` | 13 | MIXED | archive `00-*` through `12-*` except `10`; keep README |
| `audits/` + `reviews/` + `review/` | 3+4+1 | DUPLICATE STRUCTURE | consolidate into `audits/` |
| `cicd/` | 3 | EXCELLENT | leave |
| `research/` | 3 | GOOD | leave |
| `plans/` | 3 | SUPERSEDED en bloc | move to `archive/plans/` |
| `superpowers/` | 9 | SUPERSEDED en bloc | move to `archive/superpowers/` |
| `ai/` | 3 | MIXED (1 CURRENT, 2 `[UNSURE]`) | review against @magic family-root |
| `benchmarks/` | 1 | STALE | move to `archive/benchmarks/` or fold into a new bench spec |
| `hooks/` | 1 | HONEST_HISTORY | keep with note OR move to `archive/hooks/` |

---

## 3. Structural findings

### S-1: Top-level vs subdirectory convention is inconsistent
13 of 18 top-level files belong in subdirectories (specs/ or archive/). Only 4 belong at top-level (CACHE, GRANTS, logo-prompt, README-equivalent material). The substrate's discipline elsewhere — `specs/`, `insights/`, `math/`, `cicd/` — is to subdir-by-kind. The top-level files are accreted history from before that discipline solidified.

**Proposed convention (ratify or revise):**
- Top-level `docs/` holds ONLY: cleanup reviews, grant artifacts, build/release ops docs (`CACHE.md`), branding assets (`logo-prompt.md`), and a top-level `README.md` if one exists.
- Everything else is `docs/<kind>/<file>`.
- Stale-but-historically-load-bearing material lives in `docs/archive/<original-path>` with a header noting supersession.

### S-2: Per-recognition specs will overflow `specs/` flat
The cascade is producing `recognition-N-...-canonical-spec.md` artifacts at a high rate (3 in the last 5 days; 7+ this month including the cascade-76-through-80 omnibus). Once #89's spec gets a "canonical" companion, plus the forward-promised landings, this will be 15+ files in a year.

**Proposed convention:** `docs/specs/recognitions/recognition-N-<slug>.md`. Keep non-recognition specs (e.g., `mirror-ref-spec.md`, `eigensheaf.md`) at `docs/specs/<slug>.md`.

### S-3: Three names for one thing (`audit` / `audits` / `review` / `reviews`)
`audits/` (3 files), `reviews/` (4 files), and `review/` (1 file, singular) are doing the same job. All eight files are Seam-authored adversarial reviews / audits of branches or surfaces. The Pack's vocabulary now uses "Seam review" consistently (per `shards/pack/seam.mirror` tick 33). Consolidate naming.

**Proposed convention:** `docs/audits/<YYYY-MM-DD>-<author>-<scope>.md`. Drop `reviews/` and `review/`. The naming pattern (`<date>-<author>-<scope>`) matches what's already in `audits/`.

### S-4: Dating conventions are MIXED
- `insights/YYYY-MM-DD-slug.md` — consistent.
- `audits/YYYY-MM-DD-slug.md` — consistent (when used).
- `specs/<slug>.md` — undated.
- `specs/recognition-N-...-canonical-spec.md` — recognition-numbered.
- `specs/<slug>-research-YYYY-MM-DD.md` — date suffix.
- Top-level — undated.

This is mostly fine — specs are tend-evergreen and shouldn't carry dates; insights and audits are snapshots and should. But the recognition specs would benefit from the date suffix too (next time #82's spec needs an update, the suffix tells the reader at a glance whether they're looking at the original landing or a re-spec).

**Proposed:** keep specs undated, add date suffix to recognition-canonical specs when re-spec'd: `recognition-82-...-canonical-spec-YYYY-MM-DD.md`.

---

## 4. Counts summary

### Top-level (18 files)

| Category | Count |
|---|---|
| CURRENT | 4 |
| SUPERSEDED | 13 |
| STALE-only | 0 |
| ORPHAN-only | 0 |
| DUPLICATE | 0 |
| CONTRADICTORY | 0 |
| HONEST_HISTORY | 1 |

### Subdirectories (13 dirs; ~177 files total under top-level + subdirs ≈ 195 files)

| Subdir | CURRENT-ish | SUPERSEDED-ish | Action |
|---|---|---|---|
| specs/ | ~95 | ~12 `[UNSURE]` | review individually next pass |
| insights/ | 19 | 0 | keep |
| math/ | 14 | 0 | keep |
| roadmap/ | 2 | 11 | archive most |
| audits/+reviews/+review/ | 8 (consolidate) | 0 | merge dirs |
| cicd/ | 3 | 0 | keep |
| research/ | 3 | 0 | keep |
| plans/ | 0 | 3 | archive |
| superpowers/ | 0 | 9 | archive |
| ai/ | 1 | 2 `[UNSURE]` | review |
| benchmarks/ | 0 | 1 | archive |
| hooks/ | 1 (self-noting) | 0 | keep with note |

---

## 5. What I am NOT confident about

`[UNSURE]` register, surfaced explicitly:

1. **`sel-2-garden.md`** — whether SEL-2.0 is dead or just dormant. Alex's call.
2. **`logo-prompt.md` placement** — top-level vs a future `assets/`/`branding/` dir. Defer until a second branding artifact exists.
3. **`ai/magic-training-pipeline.md` and `ai/tournament.md`** — whether the @magic family-root (tick 7+) has eaten these or extends them. Need to read `shards/magic/*.mirror` against these two docs side by side. Out of scope for this audit pass.
4. **`research/mycelial-networks-and-au-tissue.md`** — whether to promote to `insights/` given `@spectral/db autopoietic memory` MEMORY anchor.
5. **`math/the-tower/recursion-locks.md`** at 136 KB — single-file home or split candidate.
6. **`specs/epistemologic-import-resolver.md`** — small (3 KB), 2026-05-20, possibly STALE; worth a closer look.

I would not move any of these without checking with Alex.

---

## 6. Notes for the next pass

- `specs/` is 107 files. I spot-checked maybe 10. The next cleanup loop should focus there — especially the `kintsugi-*` family (8 specs spanning May-June, possibly with overlap) and the `mirror-*` family (`mirror-grammar-self-hosted`, `mirror-interpreter`, `mirror-new-command`, `mirror-runtime-gen-prism`, `mirror-spec-schema`, `mirror-spectral`, `mirror-store`, `mirror-ref-spec` — eight specs, likely some overlap).
- The `recognition-N-...-canonical-spec.md` family is in active production. Whatever convention gets ratified for those files needs to land before the next cascade adds three more.
- `docs/archive/` does not exist yet. If the proposal here lands, it should be created with a top-level `README.md` explaining "files here are preserved as historical record; substrate has moved past them."

---

*End of cleanup review 2026-06-20.*
