---
scout: taut
date: 2026-08-03
territory: spectral.engineer WEBSITE v1 bottom-up build path
mode: grep-first substrate-truth
watchdog: 5-phase commits, one per phase (v3 Mara stall defense)
---

# spectral.engineer website — bottom-up substrate scout

*Taut, 2026-08-03. Read-only grep. Path A (full compiler-in-browser) vs
Path B (minimum-viable static + design-system + StageFreight deploy)
adjudicated against substrate.*

---

## Phase 1 — Sources loaded + reading impressions

### Primary sources (READ)

- `/Users/reed/.reed/tasks/pending/spectral-engineer-design.md`
  (2026-06-07 origin; ~3000-word design brief).
  Header claim: **substrate-decl synthesized 2026-06-23 → `shards/docs/design.mirror`
  commit `50e3d27` (Mara Phase E, 477 lines)**, atomic-unit-of-4-briefs.
  Load-bearing principle: *perimeter loud / body calm*. Track A
  Crass-austere with mirror crimson `#D62828` as single accent.
  Atkinson Hyperlegible Next body + Recursive MONO code + Tufte sidenotes.
  APCA design target + WCAG 2.1 AA floor.

- `/Users/reed/.reed/tasks/pending/launch.md` (59 KB, 2026-07-23).
  §"SITE-RUNTIME SUBSTRATE 2026-06-23": Phases A-F LANDED on
  mirror/main. **Phase G is Alex-altitude**: "Gleam Lustre + Rust→WASM
  + content load + StageFreight delivery + deploy command." §"Landing
  page vision": single-prompt WASM-compiler-in-browser +
  `garden.spectral.engineer` content-addressed load + local-only data.
  §"What still hasn't shipped (2026-07-23)" enumerates the six-item
  blocker list. Coherence Rising crystallized as first-Mirror-deployment
  demonstration + Emma Klint substrate-protection move.

- `/Users/reed/.reed/tasks/pending/spectral-font-research.md` (2026-05-26).
  Header claim: **substrate-decl synthesized in same commit `50e3d27`
  §3**. Body Atkinson Hyperlegible Next / code Recursive MONO axis /
  display OFL stencil. Custom `@spectral/font` is long-arc, NOT v1.

### Composition-anchor sources (READ)

- `shards/docs/design.mirror` @ 23.5 KB (2026-06-23).
  **CONFIRMED**: atomic-unit-of-4-briefs synthesis. Six typed carriers:
  `design_token` / `palette` / `typography_stack` / `section_layout` /
  `theme` / `machine_metadata` — all `ref` at family-root altitude
  per `[[feedback-no-bare-types]]`. Recognition #96 candidate territory.
  Forward-promised consumer: **spectral.engineer/case-study/X**
  (Alex 2026-06-23 deployment cascade).

- `shards/docs/tea/spectral-engineer-case-study.mirror` @ 20.1 KB
  (2026-06-23). **Phase F substrate-decl for the FIRST DEPLOYED PAGE**:
  Fellowship stated-vs-enacted Connes distance case study. Composes
  `@docs/design` tokens + `@docs/tea` M/V/U pattern + `@ui` eigenboard
  + `@nl.connes_distance`. Forward-promise: **Gleam Lustre implementation
  at Phase G; cascade<gleam, js> delivery; StageFreight
  content-addressed deployment**.

- `docs/logo-prompt.md` (2026-06-04, 8.1 KB).
  Five-beam palette locked: crimson `#D62828` / amber `#F4A261` /
  emerald `#2A9D8F` / cobalt `#264653` / violet `#5A189A` on `#000000`.
  Pentagonal-prism iconography = five Prism operations. Brand spine
  = unchanged inheritance for spectral.engineer.

### Substrate-truth surprises from Phase 1 reads

1. **`app/src/corpus/page.gleam` DOES NOT EXIST** at the path cited
   throughout the design brief. Grep confirms `/Users/alexwolf/dev/systemic.engineering/app/`
   is not present in the filesystem. The systemic.engineering site is
   **Ghost.io-hosted** (`blog/ghost.io/{journal,spectral,taste}/assets/built/screen.css`
   — three Ghost themes with 40-48 KB compiled CSS). The "current corpus
   CSS at `page.gleam:110-146`" is a **substrate-decl reference, not a
   deployed artifact**. The brief predates Ghost deployment.

2. **Ghost.io is where systemic.engineering currently lives.** Not
   Gleam Lustre. Ghost.io flake — no `flake.nix` for a Lustre app.
   Only `/Users/alexwolf/dev/systemic.engineering/flake.nix` (909 B,
   2026-03-02) at repo root + garden/private/@reed sub-flakes.

3. **Coherence Rising DRAFT exists** at
   `/Users/alexwolf/dev/systemic.engineering/blog/void/2ready/Void - Coherence.md`
   (8 KB, 2026-07-14). Only ~120 lines. This is Alex's Aug 2026 launch essay.

4. **PAPER_draft.md** = *Coherence Rising From Turing's Ashes*
   (224 KB, 2026-07-25). The **paper**, distinct from the essay.
   Two different Coherence Rising artifacts.

5. **garden.spectral.engineer publicly announced** in Teams
   marketing copy with "ETA June" qualifier (2026-06-05, 2026-06-10).
   Not yet live per 2026-07-23 launch.md blocker list.

6. **StageFreight PR-A merged** at `PrPlanIT/StageFreight #1`
   (multi-language translation layer spec; Pack-discipline cross-org
   fork-based PR). Referenced launch.md §SITE-RUNTIME-SUBSTRATE.

### Phase 1 impression — what shape the scout is landing on

**The website already exists as substrate-decl (Phases A-F on mirror/main).
The website does not exist as deployed artifact.** The gap is Phase G:
Gleam Lustre runtime + Rust→WASM + content load + StageFreight delivery
+ deploy command. Reed's Path A / Path B split proposal maps directly
onto: Path A = execute full Phase G with WASM compiler + garden load;
Path B = execute Phase G with static rendering + defer WASM to v2.
**Design-system substrate is READY.** Rendering-runtime substrate is
READY (Phase F case-study species-decl). Cascade species READY
(shards/cascade/code/gleam/js.mirror; shards/cascade/code/gleam/beam.mirror;
shards/cascade/code/rust/wasm.mirror). What's missing is the Gleam
Lustre APP (Alex-altitude implementation) that composes them.

Phases 2-5 will grep-verify what's shipped vs specced vs pending vs
blocking, then adjudicate Path A vs Path B against the substrate-truth
this Phase 1 already surfaces.

---

## Phase 2 — Shipped vs Specced vs Pending state map

### Q1 — What is SHIPPED at website altitude right now?

| Artifact | Location | Status | Cadence |
|---|---|---|---|
| systemic.engineering (Ghost.io) | `blog/ghost.io/{journal,spectral,taste}/` | LIVE on Ghost.io (three theme variants); 40-48 KB compiled CSS per theme | Since ~2026-03 (`blog/ghost.io/journal/assets/built/screen.css` mtime 2025-12-21) |
| Ghost content: essays | `/Users/alexwolf/dev/systemic.engineering/blog/void/3published/`, `blog/pieces/`, `blog/weird/3published/`, `blog/reed/`, `blog/void/2ready/Void - AI.md` | Deployed via Ghost publishing pipeline | Continuous cadence 2026-06 through 2026-07-22 corpus cascade |
| spectral.engineer domain | Registered per launch.md (`~/dev/systemic.engineering/practice/field-logs/2026-05-10 · The Domain Is The Garden.md`) | Domain owned; NOT yet resolving to a live site | Registered pre-2026-05-10 |
| garden.spectral.engineer | Publicly announced in Teams marketing copy ("ETA June" per 2026-06-05 `blog/pages/Teams.md`) | NOT LIVE per 2026-07-23 launch.md blocker list | ETA June 2026 slipped |
| Mirror logo iconography | `docs/logo-prompt.md` (2026-06-04) | Prompt-spec; images generated by Alex; deployed on Substack + LinkedIn | Live in circulation via Alex's public deploys |
| StageFreight PR-A | `PrPlanIT/StageFreight #1` (multi-language translation layer spec) | MERGED per launch.md §SITE-RUNTIME-SUBSTRATE (2026-06-23) | Pack-discipline cross-org fork-based |

**Key finding**: **There is no Gleam Lustre app at
`/Users/alexwolf/dev/systemic.engineering/app/src/corpus/page.gleam`.**
The `page.gleam:110-146` citation in the design brief is a
**substrate-decl reference** (what the CSS *will* look like when it
lands), not a currently-deployed artifact. The `--text: #1a1a1a; --bg:
#fafafa; --accent: #2a5caa` token vocabulary lives in the compiled
Ghost.io `screen.css` bundles — Reed's cascade suggestion should adjust:
"compose over Ghost.io CSS baseline" is not what's happening; the
substrate-decl'd design system is the OPPOSITE direction (custom
Gleam Lustre app replacing Ghost).

### Q2 — What is SUBSTRATE-DECL'D on mirror/main but NOT implemented?

Per launch.md §"SITE-RUNTIME SUBSTRATE 2026-06-23":

| Phase | Substrate-decl | Location on mirror/main | Status |
|---|---|---|---|
| A | @cascade family-root + 5 cascade species + Recognition #95 candidate | `shards/cascade/`, canonical spec `ce4874b` (Mara 2026-06-23) | LANDED |
| B | @nl spectral primitives (Connes distance + spectral triple + @docs bridge) | `shards/nl.mirror` + `shards/magic/nl.mirror` | LANDED |
| C | @ui instrument (Mote/Arc/Field GPU eigenboard + event surface) | `shards/ui.mirror` (19.9 KB, 2026-06-23) | LANDED, Recognition #96 candidate territory; WebGPU end-of-2026 assumption |
| D | @docs presentation + @docs/tea M/V/U composition pattern | `shards/docs.mirror` (20.5 KB) + `shards/docs/tea.mirror` (11.1 KB) | LANDED |
| E | @docs/design synthesis (atomic-unit of 4 briefs) | `shards/docs/design.mirror` @ `50e3d27` (23.5 KB, 477 lines) | LANDED — this scout's ground truth |
| F | Page rendering templates | `shards/docs/tea/spectral-engineer-case-study.mirror` @ `237c89a` (20.1 KB) | LANDED — the FIRST DEPLOYED PAGE species-decl |
| Bridge | `field_from_measurement` primitive | `shards/docs.mirror` §"field_from_measurement action" | LANDED (Phase F-hedge discharge) |

**All Phase A-F substrate-decls confirmed present on `main`.** Cascade
species that Phase G consumes:
- `shards/cascade/code/gleam/js.mirror` (21.4 KB) — **"THE PRODUCTION
  CASCADE FOR SPECTRAL.ENGINEER'S CONTENT LAYER"** (verbatim from
  species docblock)
- `shards/cascade/code/gleam/beam.mirror` (24.1 KB) — server-side
  render option (docblock hedges spectral.engineer's CURRENT content
  cascade)
- `shards/cascade/code/rust/wasm.mirror` (11.4 KB) — **"LOAD-BEARING
  delivery mechanism for spectral.engineer's @ui (GPU eigenboard
  rendering crate)"** (verbatim from species docblock); target for
  Path A WASM compiler

### Q3 — What is DRAFTED but not shipped?

| Artifact | Location | State | Blocks |
|---|---|---|---|
| Coherence Rising ESSAY | `blog/void/2ready/Void - Coherence.md` (8 KB, 2026-07-14) | DRAFT, ~120 lines, in `2ready/` (staging tier) | First-Mirror-deployment demonstration; requires spectral.engineer live to fully land |
| Coherence Rising PAPER | `PAPER_draft.md` (224 KB, 2026-07-25) | Long-form paper `Coherence Rising From Turing's Ashes`; distinct from essay | Anthropic mythos letter (2-6 weeks post-launch); Edinburgh Phase B |
| mirror-pangram package | Named in launch.md §"Coherence Rising ships"; NO code found in mirror repo grep | UNBUILT | Coherence Rising publication + first industry-adversarial deployment |
| spectral.engineer landing page (design-thesis vision) | Named in launch.md §"Landing page vision" (single prompt + WASM + garden load) | UNBUILT | Julia demo + Burry Phase A2 + Edinburgh + Anthropic letter + cybernetics piece link |
| Fellowship case-study (first page) | `shards/docs/tea/spectral-engineer-case-study.mirror` (substrate-decl LANDED); Phase G data + Gleam Lustre impl UNBUILT | Substrate-decl says: "forward-promised concrete value pending Alex confirmation" for URL slug; content brief at `tasks/active/spectral-engineer-case-study-fellowship.md` (Phase G) | Content cascade for spectral.engineer/case-study/X |
| Third-Order Anti-Narc Method piece | `practice/insights/cybernetics/third-order-anti-narc-method.md` (protected visibility per launch.md) | WRITTEN 2026-07-23, protected visibility, NOT published | Non-blocking for website v1 |
| FEMiNiNE RAGE | `blog/void/1draft/Void - RAGE.md` | Draft; earlier version shipped to Substack 2026-06-16 | Non-blocking |

---

