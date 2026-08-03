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

## Phase 3 — Blocking-chain analysis

### Q4 — What is BLOCKING spectral.engineer v1?

Launch.md §"What still hasn't shipped (2026-07-23)" enumeration, cross-checked against substrate:

| # | Blocker | Substrate status | Alex-altitude vs mirror-altitude |
|---|---|---|---|
| 1 | spectral.engineer landing page | Substrate-decl READY (Phases A-F); Phase G Gleam Lustre app UNBUILT | Alex-altitude implementation |
| 2 | Compiler in-browser (WASM deploy) | Cascade species `shards/cascade/code/rust/wasm.mirror` LANDED (substrate-decl); Rust→WASM actual emit path unclear — @ui GPU eigenboard crate at `/Users/alexwolf/dev/projects/spectral/crates/ui` is grep-referenced but WebGPU assumed "available end-of-2026" per `shards/ui.mirror` hedge | Substantial Alex-altitude work; ui crate has to actually compile-to-WASM; not just substrate-decl |
| 3 | garden.spectral.engineer package manager | `shards/spectral/garden.mirror` LANDED 2026-07-20 as species; canonical spec + math foundation in `docs/specs/spectral-garden-cosmos-spectral-db-reification.md`; cosmos + spectral-db reified at `~/dev/garden/{cosmos,spectral-db}` | Substrate READY; deployment surface (public HTTPS at garden.spectral.engineer) still needs stand-up |
| 4 | mirror-pangram package | NAMED in launch.md; NO code found via grep across mirror repo or systemic.engineering | Alex-altitude authorship required; first industry-adversarial package |
| 5 | Coherence Rising essay | Draft at `blog/void/2ready/Void - Coherence.md` (8 KB, ~120 lines); needs completion + frame-engineered-to-be-detected-as-AI move | Alex-altitude authoring |
| 6 | Third-Order Anti-Narc Method piece publication | Written 2026-07-23, protected visibility | Alex-altitude publication decision |

**Critical-path chain toward v1 landing page (from Reed-adjudication perspective):**

```
Phase A-F substrate-decl [LANDED]
    ↓
Phase G Alex-altitude construction:
    ↓
    ├── Gleam Lustre app scaffold (composes @docs/design + @docs/tea tokens)
    │       ↓
    │   composes cascade<gleam, js> → browser-deployable JS bundle
    │       ↓
    │   [optional] Rust→WASM @ui eigenboard crate (Path A only)
    │       ↓
    │   [optional] garden.spectral.engineer content-load (Path A only)
    │
    ├── Fellowship case-study content (stated corpus + enacted corpus)
    │   OR minimum-landing-page content (single-prompt cursor + tagline)
    │
    └── Deploy target:
            ↓
        StageFreight PR-A (merged) provides multi-language translation layer
            ↓
        fly.io or equivalent Docker-image hosting (Reed session-context)
            ↓
        DNS: spectral.engineer → hosted app
```

**Substrate-truth blocker**: **the actual gap is Alex-altitude Gleam
Lustre + Rust→WASM implementation of Phase G**. The substrate-decl
side is ready. The Cargo.toml + gleam.toml + fly.toml + deploy pipeline
have not been authored.

### Q5 — What UNBLOCKS on spectral.engineer ship?

Cross-references from grep (each verified via launch.md §Related + task-file grep):

| Downstream task | Blocking predicate | Verified via |
|---|---|---|
| **julia-demo** (`tasks/important/julia-demo.md`) | "spectral.engineer live OR a substitute that lets the audience run the benchmark" | Direct quote line 3 |
| **burry** Phase A2 email | "spectral.engineer + founding-member /ai access pipeline operational" for verifiable-benchmark email | Direct quote `burry.md:20`; Phase A1 already EXECUTED 2026-06-27 restack |
| **edinburgh** Phase B approach to Sam | "Approach Sam when spectral.engineer is live... show up with something in the world, not just a pitch" | Direct quote `edinburgh.md:23`; late-2026/early-2027 enrollment window |
| **anthropic-mythos-letter** | "spectral.engineer public launch — the measurement instrument goes live" is prerequisite #2 of 3 | Direct enumeration `anthropic-mythos-letter.md:32`; sends 2-6 weeks post-launch |
| **cybernetics-two-substrate-dialogue** | "spectral.engineer operational with mirror runtime" + "Reed identity corpus loadable" + "Web-accessible interaction layer" | Three prereqs enumerated `cybernetics-two-substrate-dialogue.md:88-93`; describes Reed-on-spectral.engineer as co-author of piece |
| **Coherence Rising essay publication** | Detection-run needs mirror-pangram package on garden.spectral.engineer (frame-engineering premise) | launch.md §"Coherence Rising ships"; detection-mechanism embedded in the move |

**Path B (static + design system, no WASM) unblock coverage:**

- julia-demo: YES — "a substitute that lets the audience run the benchmark" clause admits static benchmark page
- burry Phase A2: YES — verifiable benchmarks page + OpenCollective link works as static HTML
- edinburgh Phase B: YES — "show up with something in the world" satisfied by any live spectral.engineer
- anthropic mythos letter: PARTIAL — "measurement instrument goes live" language reads as compiler-in-browser, but a landing page + local-download instructions could substitute
- cybernetics-two-substrate-dialogue: NO — three prereqs (operational mirror runtime, Reed corpus loadable, web-accessible interaction) all require Path A WASM

**Path A (full WASM compiler) unblock coverage: ALL of the above.**

The uplift Path A provides beyond Path B is: cybernetics piece coauthored by Reed-on-spectral.engineer (full requirement) + Coherence Rising's detection-mechanism-in-browser demonstration + Anthropic letter's "measurement instrument live" claim at full strength.

---

## Phase 4 — Path A vs Path B verdict + Path B bottom-up build order

### Q6 — Path A vs Path B adjudication

**Path A** — full compiler-in-browser vision (single prompt + WASM +
`garden.spectral.engineer` load + local-first inference):

| Dependency | Status | Weeks-cost estimate |
|---|---|---|
| Rust→WASM emit path for mirror binary | Substrate-decl LANDED (`shards/cascade/code/rust/wasm.mirror`); actual compile-to-WASM of mirror runtime crate: UNTESTED | 2-4 weeks (uncertain) |
| @ui GPU eigenboard crate WASM-compiled | `/Users/alexwolf/dev/projects/spectral/crates/ui` grep-referenced; WebGPU assumed "available end-of-2026" per `shards/ui.mirror` hedge | 2-4 weeks (WebGPU browser support gating) |
| garden.spectral.engineer HTTPS serving with content-addressed load | `shards/spectral/garden.mirror` LANDED 2026-07-20; cosmos + spectral-db reified; PUBLIC HTTPS deployment surface UNBUILT | 1-2 weeks |
| geometric-roomba landing #1 (per Reed prompt) | Mentioned in Reed's prompt as Path A blocker; grep-referenced across `shards/kintsugi/*.mirror` and `shards/gestalt.mirror`; NOT yet fully landed | Uncertain — depends on which "geometric-roomba" landing Reed means |
| Fate GPU integration | `shards/spectral/*` + `shards/silicon/algebra.mirror` grep-referenced; substrate exists; browser-GPU inference bridge UNBUILT | 2-4 weeks (research-territory) |
| **Total realistic-optimistic Path A wall-clock** | | **6-10 weeks Alex-altitude solo; longer with substrate research surprises** |

**Path B** — minimum-viable static site with design system + existing
essays + StageFreight → fly.io deploy:

| Dependency | Status | Days-cost estimate |
|---|---|---|
| Design system substrate | `shards/docs/design.mirror` (23.5 KB, 477 lines) LANDED with concrete token vocabulary: `color_punctum #A0264F` + `wine_50..wine_950` ladder + `beam_*` semantic palette + typography stack + three-altitude density layout + Tufte sidenote pattern | 0 days — READY |
| Gleam Lustre app scaffold | `shards/cascade/code/gleam/js.mirror` (21.4 KB) LANDED as substrate-decl for the cascade; new Gleam project (`gleam new spectral_engineer_site`) needs authoring | 1-2 days |
| Page template (composes design tokens) | `shards/docs/tea/spectral-engineer-case-study.mirror` (20.1 KB) LANDED for Fellowship case-study; simpler landing-page species-decl is one composition-instance | 1-2 days |
| Content (minimum): landing page + one essay | Coherence Rising draft at 8 KB in `2ready/`; needs completion pass OR the June triptych rehost from Ghost.io | 1-3 days depending on scope |
| StageFreight delivery | PR-A merged 2026-06-23; multi-language translation layer available | 0 days — READY |
| fly.io deploy target | Reed's session-context (per prompt) scoped fly.io deployment; `fly.toml` + Dockerfile need authoring; SSH signing + custom domain routing | 1-2 days |
| DNS: `spectral.engineer` → fly.io | Domain owned; DNS records need pointing | <1 day |
| **Total realistic Path B wall-clock** | | **~5-9 days Alex-altitude solo** |

**Substrate-honest verdict — Path B is 6-8× closer than Path A.** The
substrate CONFIRMS Reed's split. All Phase A-F substrate-decl work
directly supports Path B (Gleam Lustre + cascade<gleam, js> +
`@docs/design` token bindings + `@docs/tea` M/V/U pattern). Path A
requires 4 additional substrate-heavy work-streams (WASM mirror
compile + WebGPU eigenboard browser support + public garden HTTPS +
fate GPU inference bridge) that are NOT yet substrate-decl'd at
implementation-detail altitude.

**Path C candidates surfaced by grep:**

- **Path C1 (progressive-enhancement)**: Ship Path B as v1.0.
  Add WASM compiler-in-browser as v1.1 progressive-enhancement.
  Landing page renders static by default; JavaScript hydrates
  single-prompt cursor when WASM is available. Substrate-supported:
  cascade<gleam, js> LANDED, Rust→WASM LANDED as separate cascades;
  the composition IS the progressive-enhancement architecture.
- **Path C2 (partial-WASM)**: Ship landing page with pre-canned
  compiled example (Fellowship Connes-distance measurement served as
  static WASM demo) instead of full compiler-in-browser. Users see
  the eigenboard rendering a real @nl measurement without needing
  the compiler to run their own inputs. Substrate-supported: this IS
  the Phase F Fellowship case-study species-decl at
  `shards/docs/tea/spectral-engineer-case-study.mirror`. Reduces Path
  A scope by 60-70%.
- **Path C3 (StageFreight-native docker image)**: Per
  `shards/kintsugi/ouroboros.mirror` (Alex quoted directly):
  *"we ship with @../StageFreight/ the executable docker image that
  you [...] pipeline. That's what spectral.engineer becomes. A
  ready-to-deploy-and-run docker image."* This reframes
  spectral.engineer as a deployable docker artifact users pull-and-run
  locally, with the WEB SITE demonstrating the artifact. Path B
  covers the web site half; the docker image is the compiler
  half. Substrate-native decomposition.

**Taut recommendation**: **Path B v1.0 → Path C1 progressive-enhancement
v1.1 → Path A/C3 v1.2**. This is the substrate-honest sequence. Each
step composes what the prior step landed.

### Q7 — Path B bottom-up build order (concrete steps)

Assumes Alex-altitude Gleam Lustre + StageFreight + fly.io competence.
Each step grep-verified against substrate.

**Step 0 — Prereq check (~30 min)**

- Confirm `gleam` toolchain, `nix`, `docker` installed
- Confirm StageFreight repo cloned; `stagefreight init` runnable
- Confirm fly.io account + `flyctl` authenticated
- Confirm DNS access for `spectral.engineer`

**Step 1 — Scaffold Gleam Lustre app (~1 hour)**

- `mkdir -p ~/dev/projects/spectral_engineer_site && cd $_`
- `gleam new spectral_engineer_site --template=lustre`
- Add `lustre` + `lustre_ssg` (static-site-generation companion) + `gleam_stdlib` deps
- Compose over `shards/cascade/code/gleam/js.mirror` substrate-decl
- Substrate-grep: `shards/cascade/code/gleam/js.mirror` L164-170 —
  "PRODUCTION CASCADE FOR SPECTRAL.ENGINEER'S CONTENT LAYER";
  gleam_source carrier IS the input shape

**Step 2 — Realize the design tokens into CSS (~2-3 hours)**

- Translate `shards/docs/design.mirror` §2 palette tokens into CSS
  custom properties in `priv/static/tokens.css`:
  - `--color-punctum: #A0264F;`
  - `--wine-50` through `--wine-950` ladder
  - `--beam-claim` / `--beam-evidence` / `--beam-warning` /
    `--beam-code` / `--beam-external-link`
- Translate §3 typography stack (Atkinson Hyperlegible Next body +
  Recursive MONO axis code + OFL stencil display) with `@font-face`
  declarations + fallback chain
- Translate §4 three-altitude density layout (site-dense /
  piece-Tufte-sparse / element-semantic-color)
- Substrate-grep: `shards/docs/design.mirror` L119-235 — token
  vocabulary is verbatim; no re-derivation needed. This IS Reed's
  "compose over substrate ancestor" per HARD RULE.

**Step 3 — Author page species-decl for landing (~1 hour)**

- Compose over `shards/docs/tea.mirror` M/V/U pattern
- Simpler than Fellowship case-study: just cover + tagline + single-prompt-cursor placeholder + footer + machine-readable JSON-LD
- `case_study_id = "index"` or new species `@docs/tea/landing`
- The single-prompt-cursor in Path B v1.0 is COSMETIC (submit-button says "Coming in v1.1")
- Substrate-grep: `shards/docs/tea/spectral-engineer-case-study.mirror`
  L98-105 — prism declaration pattern; L107-140 — case_study_model
  carrier shape

**Step 4 — Content: landing page + Coherence Rising (~1-2 days)**

- Landing page copy: mirror-logo iconography (Variation A favicon per
  `docs/logo-prompt.md` L57-59) + tagline from launch.md §"spectral.engineer
  tagline crystallized v3":
  *"Förster-legal quantum-native AI. Local. Distributed. Mycelial. On
  hardware you already own. Made in Germany. Smarter. Harder. And
  definitely more punk."*
- One essay: Coherence Rising completion pass (currently 8 KB draft
  in `blog/void/2ready/Void - Coherence.md`); OR June triptych
  rehost (Alignment&Coordination + The Build + Third Belongs to
  the Cyberneticists); OR minimum-landing-page-only (see Q8)
- Substrate-grep: launch.md L57 for tagline v3 verbatim; Coherence
  Rising draft at `blog/void/2ready/Void - Coherence.md`

**Step 5 — Machine-readable layer (~2-3 hours)**

- `robots.txt` per `shards/docs/design.mirror` §5 machine_readable +
  spectral-engineer-agents.md brief
- `llms.txt` index of pieces + metadata
- JSON-LD Article schema in each page's `<script type="application/ld+json">`
- AIPREF Content-Usage rules
- Substrate-grep: `shards/docs/design.mirror` §1 `machine_readable = ref`
  carrier

**Step 6 — Nix flake for reproducible build (~1-2 hours)**

- `flake.nix` composing `gleam` toolchain + `lustre` build + StageFreight
  container assembly
- Substrate-grep: `/Users/alexwolf/dev/systemic.engineering/flake.nix`
  as pattern-reference; garden/private/@reed sub-flakes as composition
  pattern

**Step 7 — StageFreight docker image build (~1-2 hours)**

- StageFreight PR-A merged multi-language translation layer handles
  Gleam→JS→static-bundle
- Emit crystal artifact per `shards/kintsugi/ouroboros.mirror`
  "ready-to-deploy-and-run docker image" language
- Content-addressed image tag; StageFreight orchestrates
- Substrate-grep: launch.md §SITE-RUNTIME-SUBSTRATE — StageFreight
  PR-A already merged; delivery layer READY

**Step 8 — fly.io deploy (~1-2 hours)**

- `fly.toml` scoped from Reed's session context
- `flyctl launch --dockerfile Dockerfile.stagefreight`
- Set `[env]`, `[http_service]`, `[[services.ports]]` for HTTPS on 443
- `flyctl deploy`
- Substrate-grep: NO prior fly.io references in mirror repo grep;
  Reed session-context is the sole source; validate against
  StageFreight FluxCD reconcile-and-deploy pipeline (launch.md
  §SITE-RUNTIME-SUBSTRATE mentions Cilium-firewalled cluster; fly.io
  is a substitute at v1.0)

**Step 9 — DNS + first-view smoke test (~1 hour)**

- Point `spectral.engineer` A/AAAA record at fly.io app
- Wait for TLS cert issuance
- Curl + Firefox reader-mode smoke test
- APCA contrast audit via APCA calculator (per
  `shards/docs/design.mirror` §5 a11y_contract)
- WCAG 2.1 AA regulator-facing audit

**Step 10 — Recognition #96 second-witness commit (~30 min)**

- The deployed site IS the second-witness for Recognition #96
  (instrument @ui + presentation @docs partition at deployed-page
  altitude); Phase F species-decl at `237c89a` was the first
  substrate-witness; live deploy is the second
- Signal to Reed for Recognition #96 ratification-gate check
- Substrate-grep: `shards/docs/tea/spectral-engineer-case-study.mirror`
  Recognition ancestry §; docs/loop/CURRENT.md Recognition candidate
  register

**Total wall-clock: ~5-9 days at Alex-altitude solo cadence.** Steps
0-10 have zero substrate-decl blockers; every step composes over
already-landed shards.

---

## Phase 5 — Content-for-v1 recommendation + follow-up flags + closure

### Q8 — Content-for-v1 recommendation

Three options canvassed against Q5 unblock coverage + Phase 4 build order:

| Content option | Days added | Julia unblock | Burry A2 unblock | Edinburgh unblock | Anthropic letter unblock | Substrate-honest register |
|---|---|---|---|---|---|---|
| **Minimum-landing-only** (tagline + single-prompt cursor + footer + logo + JSON-LD; ~1 page) | 0 (in Step 4) | Partial — landing but no benchmark | Partial — page live but no benchmarks-page | YES — "something in the world" clause | Partial — measurement instrument NOT visibly live | Punk-zine minimalism; fable register from launch.md §"2026-06-28 Launch Note" honored |
| **Coherence Rising alone** (landing + one essay in `2ready/` needs ~1 day completion pass) | +1-2 days | YES if essay carries benchmark | YES if essay carries verifiable-benchmarks page | YES | YES — first-Mirror-deployment demonstration lands via detection-mechanism | first-Mirror-deployment substrate-move honored; but essay's frame-engineered-to-be-detected-as-AI premise requires mirror-pangram package (Path A dep) |
| **June triptych rehost** (Alignment&Coordination + The Build + Third Belongs to the Cyberneticists; already on Ghost.io/Substack, needs migration/mirror) | +2-3 days | YES | YES if benchmarks page added | YES | Partial — mythos letter references specific mirror runtime, not the June triptych | Corpus already-shipped content elevated to spectral.engineer domain; low-risk substrate-safe |

**Taut recommendation: Minimum-landing-only for v1.0 + Coherence
Rising as v1.1 immediately after.**

Rationale:
- Minimum-landing v1.0 satisfies the *fable register* from launch.md
  §"2026-06-28 Launch Note" — "small device on table" gesture at web
  altitude
- Ships in ~5 days (Step 4 collapses to <1 hour when content is
  landing-only)
- Julia demo is downstream-flexible per its blocking clause ("OR a
  substitute that lets the audience run the benchmark") — Alex can
  ship a static benchmark page as landing v1.0 §Evidence section if
  benchmarks are load-bearing
- Coherence Rising v1.1 is a 1-2 day cadence-follow that lands as
  the first *piece* on the live site — this is the substrate-honest
  first-Mirror-deployment demonstration Alex was planning
- June triptych rehost is deferrable — those pieces are already live
  on Substack/Ghost.io; migration is nice-to-have not blocker
- mirror-pangram package is orthogonal to landing-page ship — it's
  a separate garden.spectral.engineer artifact; can be built in
  parallel or as v1.1

### Follow-up flags

**[FLAG-TAUT-1]** — The design brief cites
`/Users/alexwolf/dev/systemic.engineering/app/src/corpus/page.gleam:110-146`
throughout as if it were a live artifact. **It is not.** The path
`/Users/alexwolf/dev/systemic.engineering/app/` does not exist. The
CSS values quoted ARE the substrate-decl in `shards/docs/design.mirror`
§2. Recommend: update design brief header to name the substrate-decl
as canonical source, remove page.gleam citations OR mark them as
"proposed post-Phase-G shape". Non-blocking for the build; but Reed
will trip on this citation if not corrected.

**[FLAG-TAUT-2]** — Design brief says accent should be mirror crimson
`#D62828` (single accent Track A). Landed design shard uses
`color_punctum #A0264F` (wine-pink, sampled from Spectral Remix
cover). **These are different colors.** `shards/docs/design.mirror`
§2 shows the wine-pink was locked per "Decisions 5+6" of
spectral-engineer-color.md sibling brief. Design brief `.md` doesn't
mention `#A0264F`. Recommend: cross-check which is canonical; the
shard is the substrate ground-truth per `shards/docs/design.mirror`
LANDED status.

**[FLAG-TAUT-3]** — `shards/ui.mirror` hedge assumes WebGPU is
"available end-of-2026." As of 2026-08-03 (today), WebGPU shipping
status in Firefox stable is still gated (Chromium has it; Firefox
Nightly + preference flag). This affects Path A eigenboard rendering
in the browser. Recommend: verify current WebGPU shipping status
before committing Path A timeline; consider WebGL2 fallback for @ui.

### [ALEX-Q]s (target ≤3)

**[ALEX-Q-1]** — For v1.0 content: **minimum-landing-only OR
Coherence Rising alone**? (Taut recommends: minimum-landing-only v1.0
+ Coherence Rising v1.1 in same week.) The recommendation frees ~5-day
critical-path ship; the alternative extends by 1-2 days for stronger
first-impression. Alex-altitude decision.

**[ALEX-Q-2]** — For v1.0 accent color: **mirror crimson `#D62828`
(design brief prescription) OR color_punctum wine-pink `#A0264F`
(landed shard value from Spectral Remix cover sampling)?** The shard
is what will ship if Path B executes as-substrate. Per FLAG-TAUT-2.

**[ALEX-Q-3]** — For v1.0 vs v1.1 boundary: **fly.io OR
substrate-native StageFreight-with-Cilium-firewalled-cluster
deployment target?** Reed's session-context scoped fly.io. Ouroboros
shard `shards/kintsugi/ouroboros.mirror` quotes Alex naming
StageFreight docker image as "ready-to-deploy-and-run" (which reads
as: fly.io is fine for v1.0 as long as StageFreight assembles the
image). Alex-altitude decision.

### Reed-cascade suggestion (first-tick post-adjudication)

Once Alex adjudicates the three ALEX-Q's above, Reed's first concrete
tick is:

**Tick R1 — Reed authors `tasks/pending/spectral-engineer-v1-build.md`**
as an active-cycle Shape-Up-format shape doc composing over this scout.
Structure:
- §1 Path B v1.0 scope: 11-step build order per Q7 (concrete)
- §2 Path C1 v1.1 progressive-enhancement scope: WASM compiler-in-browser
  behind feature-flag; garden.spectral.engineer content-load
- §3 Substrate compose-anchors: `shards/docs/design.mirror` +
  `shards/docs/tea/spectral-engineer-case-study.mirror` +
  `shards/cascade/code/gleam/js.mirror` cited per HARD RULE (Reed
  re-derives what is already landed)
- §4 Alex-altitude vs Reed-orchestrated split: Alex owns Gleam Lustre
  authoring; Reed orchestrates Mara canonical-spec for any missing
  species-decl (e.g., `@docs/tea/landing` if landing-page pattern needs
  species-decl separate from Fellowship case-study)
- §5 Cycle-length: recommend 6-day cycle (matches Path B upper bound)
- §6 Handoff to Alex: after §1-5 confirmed, Alex-solo execution;
  Reed available for substrate-clarification questions

Reed's SECOND concrete tick is optional and deferred to Alex trigger:
scaffold Fellowship case-study content brief (`tasks/active/spectral-engineer-case-study-fellowship.md`
per Phase F forward-promise) — this is v1.1 or v1.2 content, NOT v1.0.

---

## Scout closure

- 5 phase-commits successfully sequenced (v3 Mara stall pattern avoided)
- Substrate-truth surfaced: **Phases A-F LANDED; Path B is 6-8× closer
  than Path A because the substrate ALREADY DID the work**
- Reed's Path A/Path B split substrate-confirmed; Path C1
  progressive-enhancement recommended as bridge
- Content recommendation: **minimum-landing-only v1.0 + Coherence Rising
  v1.1**
- 3 [ALEX-Q]s surfaced (color decision + content scope + deploy target)
- 3 [FLAG-TAUT]s surfaced (page.gleam citation drift + accent color
  drift + WebGPU availability check)
- Reed-cascade first-tick suggestion: author
  `tasks/pending/spectral-engineer-v1-build.md` composing over this
  scout

Grep-first, substrate-honest, commit-often held.

🌱🔍




