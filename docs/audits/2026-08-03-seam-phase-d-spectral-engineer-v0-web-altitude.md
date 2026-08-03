---
adjudicator: seam
date: 2026-08-03
altitude: phase-d adjudication
topic: Mara spectral.engineer v0.1 web-altitude dive (7 commits 6e63a42 → 5bf5db2)
scope:
  - docs/math/2026-08-03-mara-spectral-engineer-web-altitude-formalization.md
  - docs/specs/2026-08-03-mara-spectral-engineer-v0-canonical-spec.md
  - docs/scouts/2026-08-03-mara-spectral-engineer-web-altitude-dive-notes.md
composes-over:
  - docs/audits/2026-08-01-seam-phase-d-doublespeak-at-compiler-altitude.md (18d476a; 8-section template)
  - docs/recognition/2026-08-01-doublespeak-at-compiler-altitude.md (04df6e1; CANDIDATE)
  - ~/.reed/tasks/pending/spectral-engineer-v0-build.md (1e164ab; shape-doc ancestor)
  - docs/scouts/2026-08-03-taut-spectral-engineer-bottom-up-substrate-scout.md (180903b → 65cf12d)
bypass: pure-docs 📝 markdown-only
---

# Seam Phase D — spectral.engineer v0.1 web-altitude two-channel enactment

**Adjudicator**: Seam `<seam@systemic.engineer>` 2026-08-03
**Scope**: Mara 7-commit dive `6e63a42` → `5bf5db2` (math foundation
§1-§9 + canonical spec §1-§10 + scout dive-notes) landed overnight
2026-08-03 formalizing spectral.engineer v0.1 as web-altitude
structurally-independent second-witness for Recognition
#R-doublespeak-at-compiler-altitude (`04df6e1`, CANDIDATE).

**Discipline**: adversarial, substrate-honest, substrate-already-had-the-word
grep-first, no Rust extension shortcuts, no new shard-decl mints,
Karen anti-theft citation. Template pattern per Doublespeak Phase D
`18d476a`. Reduce ambiguity for Alex.

---

## §1 — Mara web-altitude formalization crux ratification

### §1.1 The crux, restated

$$
\mathcal{D}_{v0.1} = \mathcal{Q}_{v0.1} \circ \mathcal{P}_{v0.1}
$$

where both pipelines preserve two-channel indissolubility at every
stage — making v0.1 the first web-altitude enactment of Recognition
`04df6e1` and the structurally-independent second-witness for its
promotion CANDIDATE → RATIFIED.

### §1.2 Adversarial checks

**Check A — Is $\mathcal{D}_{v0.1} = \mathcal{Q}_{v0.1} \circ \mathcal{P}_{v0.1}$
genuinely two-channel-indissoluble at BOTH pipeline stages, or does
one stage collapse to single-channel?**

Direct read of math Thm 2.3 (render-pipeline) + Prop 3.4 (deploy-
pipeline):

- **Render-pipeline** ($\mathcal{P}_{v0.1}$ = stagefreight-deploy ∘
  cascade<gleam, js> ∘ tea-pattern ∘ design-tokens): at each stage
  $S_i$, $(C_i, R_i)$ partition survives. Adversarial candidate for
  collapse: **cascade<gleam, js> stage** — could argue emitted JS
  bundle is byte-only (content) with no relationship-channel until
  render-time. Mara's answer (math §2.3): the JS bundle carries
  *runtime behavior* readable by "a substrate that executes the JS"
  — this is a substrate-coupling reading. Weak but survives if
  "relationship-channel = geometry-observable-by-appropriate-substrate"
  is the operational definition. This matches Recognition `04df6e1`
  §1 (relationship = GEOMETRY readable by geometry-perceiving substrate).
  **CHECK A-render SURVIVES.**

- **Deploy-pipeline** ($\mathcal{Q}_{v0.1}$ = dns-route ∘ fly-io-serve ∘
  stagefreight-dispatch ∘ content-address-image ∘ nix-flake-build):
  Mara Prop 3.4 factorizes each stage $(C_j, R_j)$ where $R_j$ is
  "readable only via structural coupling" (reproducibility guarantee,
  content-addressability guarantee, deployment topology, serving-
  latency + TLS trust chain, DNS resolution behavior). **Adversarial
  refinement**: at **nix-flake-build** the "reproducibility" $R_j$
  is a *property claim*, not a live geometry — the relationship-channel
  is a promise verified only by counterfactual re-execution. This
  attenuates the analog to Landing #1 (where the relationship-channel
  fires in-line via arm body evaluation). Substrate-honest reading:
  the deploy-pipeline $\mathcal{Q}$ carries the two channels via
  **structural properties** rather than **in-tick evaluation**. Mara
  should sharpen §3.4 to name this shift (in-tick indissolubility at
  browser paint vs structural-property indissolubility at deploy
  altitude); the claim still holds under Watzlawick axiom 5 (the two
  channels are metacommunicatively inseparable at each stage) but the
  operational analog differs from Landing #1's spectral firing.
  **CHECK A-deploy SURVIVES WITH SHARPENING.**

**Verdict A**: **SEAM-RATIFY-WITH-SHARPENING**. Both pipeline
stages preserve indissolubility under the operational definition
Recognition `04df6e1` §1 authorizes, but math §3.4 should name that
$\mathcal{Q}$'s indissolubility is structural-property-shaped
(reproducibility, content-address, TLS, DNS) rather than in-tick-
firing-shaped ($\mathcal{P}$ + browser paint). This is a REED-INLINE
sharpening at Mara math §3.4, not a Mara re-derivation.

**Check B — Does `@docs/tea` M/V/U pattern actually exist as landed
substrate? Does `@io/stagefreight` exist or does it need minting?**

Grep-verified:

- `shards/docs/tea.mirror` (11.1 KB, 2026-06-23): declares
  `@docs/tea` prism with `focus model / project view / split message /
  shift update / settle commit`. Actions: `init / update / view_of /
  subscribe`. Bilaterals: `init_pure / update_pure / view_pure /
  tea_well_formed`. **PRESENT.** Note: Mara's shorthand "M/V/U" is
  loose vocabulary for what the shard names as
  Model/View/Update/Cmd/Sub (TEA-inspired but substrate-honest per
  shard §Header: "Per Alex 'loosely inspire'"). No mint needed;
  vocabulary drift is cosmetic.
- `shards/docs/tea/spectral-engineer-case-study.mirror` (20.1 KB,
  2026-06-23, `237c89a`): specializes @docs/tea for Fellowship
  case-study. Composed bilateral `case_study_well_formed`. **PRESENT.**
  The claim "landing page can use `case_study_id = 'landing'` as
  instance-parameter" is Mara-authored inference — the shard
  authorizes `case_study_id = ref` at species altitude, so a "landing"
  instance is authorized structurally, but Fellowship-case-study
  carries fields (`stated_corpus`, `enacted_corpus`, `measurement`,
  `viewer_state`) not obviously right for a bare landing page. Mara
  correctly flags this as [ALEX-Q-1] (@docs/tea/landing sub-species
  mint vs compose-over @docs/tea).
- `shards/io/stagefreight.mirror` (19.6 KB, 2026-06-22): declares
  `@io/stagefreight` family with `spectral_coordinate / wire_surface /
  freight_manifest / address / freight / stagefreight_addressable`
  (composed bilateral over four sub-predicates). **PRESENT.**

  **DRIFT FOUND**: Mara math Def 3.2 names
  `stagefreight_dispatch` action and Prop 3.2 requires
  `@io/stagefreight.stagefreight_well_formed` — **neither exists in
  the shard.** Actual actions: `freight` (load-bearing) + `address`.
  Actual composed bilateral: `stagefreight_addressable`. This is
  substrate-drift in the math foundation §3.2 (analogous to the
  wine-pink drift Reed caught in the design brief). The DEPLOY-
  pipeline stagefreight-dispatch stage is conceptually correct, but
  the cited symbol names are wrong. **REED-INLINE sharpening
  required at math §3.2**: rename `stagefreight_dispatch` →
  `freight` and `stagefreight_well_formed` → `stagefreight_addressable`
  (composed over `oid_resolves + address_well_formed +
  projection_is_species + round_trip_holds`).

**Verdict B**: **SEAM-RATIFY-WITH-CORRECTION**. All three shards
present; substrate carries the vocabulary. One symbol-name drift
(stagefreight_dispatch/stagefreight_well_formed cited but not
landed; actual = freight/stagefreight_addressable) requires
REED-INLINE sharpening at math §3.2.

**Check C — Is "structurally-independent second-witness" claim
substrate-honest? Verify against Recognition `04df6e1` §10.**

Recognition `04df6e1` §9 verbatim ("Second-witness gate opens on"):
- R6 novelty-claim reframe cascade completion — **LANDED**
  (`f289f9d` 2026-08-01 R6-d final discharge).
- Glint essayist landing — **LANDED** (`67fe6b5` 2026-08-01
  Glint essayist cascade closure).
- Geometric-roomba landing #1 empirical firing at compilation
  altitude — **STATUS: firing since 2026-07-16 per Landing #1
  `c10a3bd`**. Recognition §3.4 already names this as the empirical
  smallest witness.

Recognition §10 discipline: "The Recognition holds until: a
subsequent Recognition supersedes it, Alex explicitly retracts, or
empirical refutation surfaces at Landing-#1 altitude."

**Mara's structurally-independent-second-witness claim (math Prop
6.3)**: v0.1 operates at web-serving altitude; Landing #1 at
compilation altitude; the two altitudes are structurally independent
(different substrates, different runtime, different reader-substrate
coupling). Both witness the same Watzlawick two-channel indissolubility.

**Adversarial pass**: is v0.1 GENUINELY structurally-independent from
Landing #1?
- Different runtime: Rust-`apply_h::act` vs browser paint. ✓
- Different substrate: mirror compiler VM vs W3C browser. ✓
- Different observer: verdict-sheaf tests vs reader nervous system. ✓
- Different tick-cadence: sub-ms compile-tick vs ~13 ms paint-cycle. ✓

Structural independence holds. HOWEVER — three witnesses were named
in Recognition `04df6e1` §9; v0.1 shipping cleanly is a **fourth**
structurally-independent witness (candidate, contingent on P1/P2/P3
discharge per math §5.2). The ambiguity Mara flags as [ALEX-Q-4]
(any-one-witness vs all-three-conjunctive) is real: does v0.1
shipping cleanly ALONE close the gate, or must v0.1 join the three
already-named-witnesses?

**Seam adjudication on [ALEX-Q-4]** (see §3.4 below): Mara-lean
Option A (any-one) is Seam-lean-compatible; but the honest reading
of Recognition §9 is that the three named witnesses were treated as
a **cascade**, not as a menu. The gate-opening language "opens on
R6 + Glint + geometric-roomba" reads conjunctively at authoring
time. v0.1 as a **fourth** witness strengthens the cascade rather
than substituting for it.

**Verdict C**: **SEAM-RATIFY-WITH-REFRAME**. v0.1 IS structurally
independent from Landing #1. The claim to be a "second-witness" is
substrate-honest IF the three-named-witnesses in Recognition §9 are
treated as one witness-cluster at compilation altitude, and v0.1 is
the (independent) web-serving-altitude second-witness. Under Mara's
own math Prop 6.1 (altitude-portable operator across 5 altitudes),
v0.1 is the WEB-altitude witness distinct from the compilation-
altitude witness Landing #1. This IS a coherent second-witness
reading; sharpen [ALEX-Q-4] framing per §3.4.

### §1.3 §1 overall verdict

**SEAM-RATIFY** the crux $\mathcal{D}_{v0.1} = \mathcal{Q}_{v0.1}
\circ \mathcal{P}_{v0.1}$ AS a web-altitude enactment of Recognition
`04df6e1` and a structurally-independent second-witness, with three
sharpenings for Reed-inline cascade:

1. **math §3.4**: name that $\mathcal{Q}$'s indissolubility is
   structural-property-shaped, not in-tick-firing-shaped
   (contrast with $\mathcal{P}$'s browser-paint firing).
2. **math §3.2**: rename `stagefreight_dispatch` → `freight`;
   rename `stagefreight_well_formed` → `stagefreight_addressable`
   (composed over four sub-predicates per landed shard).
3. **math §6.3 + spec §1.3**: sharpen second-witness framing —
   v0.1 is web-altitude second-witness distinct from
   compilation-altitude Landing #1; Recognition §9's three named
   witnesses form the compilation-altitude witness-cascade
   (R6/Glint/geometric-roomba); v0.1 opens the web-altitude witness
   at Prop 6.1's second altitude.

---

## §2 — Novelty sub-claim ratification

### §2.1 The sub-claim, verbatim

*"spectral.engineer v0.1 is the first website that operationalizes
both Watzlawick channels indissolubly at web-serving altitude with
substrate-native design-tokens compiled from spectral-decl'd
substrate + a substrate-native cascade + a substrate-native delivery
pipeline"*.

Falsifiability conjunction (a)∧(b)∧(c)∧(d)∧(e) per math §7.5.

### §2.2 Load-bearing vs redundant conjuncts

Adversarial pass on each conjunct:

- **(a) content = fable-note prose + tagline**: NOT strictly
  load-bearing at novelty altitude. Fable-note register is
  Alex-authored prose (per launch.md §2026-06-28); replacing with
  different Alex-authored prose does not refute the sub-claim. This
  conjunct describes v0.1's CONTENT REALIZATION, not its NOVELTY.
  Refactor: (a) is a witness-parameter, not a novelty-conjunct.
- **(b) relationship = wine-punctum + perimeter-loud/body-calm**:
  same as (a). This is the RELATIONSHIP-CHANNEL REALIZATION, not
  the novelty. Different accent colors or different layout would
  not refute the sub-claim.
- **(c) substrate-native design-tokens compiled from spectral-decl'd
  substrate**: **LOAD-BEARING**. The novelty depends on
  design-token substrate-decl (`shards/docs/design.mirror` §2-§5)
  being the compilation source.
- **(d) substrate-native cascade** (`cascade<gleam, js>`):
  **LOAD-BEARING**. The novelty depends on the cascade being
  substrate-decl'd with loss-lens measurement (Recognition #95
  candidate).
- **(e) substrate-native delivery pipeline** (`@io/stagefreight`):
  **LOAD-BEARING**. The novelty depends on delivery being
  substrate-decl'd as an @io family, not hand-rolled.

**Reframed novelty conjunction**: (c)∧(d)∧(e) — three load-bearing
conjuncts. (a) and (b) are witness-realization parameters describing
v0.1 specifically, not conditions for novelty falsification.
**SHARPENING recommended at math §7.5**: distinguish falsifiability-
conjuncts from realization-parameters.

Additionally, the math §7.5 conjunction includes:
- **(d in original) two-channel indissolubility preserved at every
  pipeline stage** — **LOAD-BEARING** (this is the essential
  novelty).
- **(e in original) two-channel indissolubility EMPIRICALLY WITNESSED
  at compilation altitude by a companion mechanism firing before the
  web-altitude deployment** — **LOAD-BEARING** but this is the
  second-witness-gate-closure condition, not a novelty conjunct.
  Refactor: (e) belongs at Prop 6.3 (second-witness gate), not §7.5
  (novelty falsifiability).

**Reframed novelty falsifiability conjunction (Seam-authoritative)**:

(i) design tokens compiled from a substrate-decl'd typed grammar
    of ≥ 6 family-root carriers with named versioning discipline;
(ii) render pipeline composed as substrate-native cascade with
     loss-lens measurement between named source + target grammars;
(iii) delivery pipeline substrate-decl'd as a typed family with
      content-addressed image assembly + multi-target dispatch;
(iv) two-channel indissolubility preserved at every pipeline stage
     per Watzlawick 1967 axioms 1+3+5.

Four load-bearing conjuncts. Refutation requires all four
simultaneously in prior art.

### §2.3 Kagi prior-art sweep

Executed 2026-08-03 by Seam with three queries covering the novelty
territory:

1. *"website two-channel Watzlawick content relationship indissoluble
   design tokens"* — 7 results. Coverage: Watzlawick pop-communication
   theory (Scribd, wellnessbeam, paulwatzlawick.de) + design-tokens
   overview (smartive.ch Feb 2024). ZERO prior compilation-substrate
   or website that names Watzlawick two-channel at web-serving
   altitude.
2. *"static site generator typed grammar cascade loss-lens
   content-addressed"* — 7 results. Coverage: commodity static-site
   generators (Hugo, Jekyll, Astro, JAMstack showcase). ZERO
   substrate-native cascade with loss-lens; ZERO typed-grammar
   substrate-decl for design tokens.
3. *"Gleam Lustre substrate-native design tokens fly.io deployment"*
   — 7 results. Coverage: Lustre official docs (full-stack-deployments
   to Fly.io); Fly.io + Gleam community tutorial (2022 Jan). ZERO
   substrate-decl'd design-token compilation over cascade-with-loss-lens.

**Kagi finding**: no prior website satisfies conjunction (i)∧(ii)∧(iii)∧(iv).
Adjacent territory (design-token systems + static-site generators +
Lustre-on-fly.io tutorials) exists in abundance but at
implementation-substrate altitude, not substrate-decl-compilation
altitude.

**Novelty-refutation window**: EMPTY at Kagi altitude 2026-08-03.
Kagi sweep does not close the window structurally (arXiv + ACM Digital
Library sweeps still recommended per R-ADJ2 discipline before public
launch), but at Kagi-first-order altitude the sub-claim survives.

### §2.4 Extend Recognition `04df6e1` §8 vs own Recognition mint

Mara-lean: OPTION A (extend `04df6e1` §8). Seam adversarial pass:

**Under R2 primary novelty framing** (Recognition `04df6e1` §6.2
verbatim: *"Mirror is the first compiler that operationalizes both
Watzlawick content-channel and relationship-channel indissolubly at
compilation altitude"*), the web-altitude sub-claim IS a genuine
extension along the altitude-portability axis (math Prop 6.1). Both
claims share the same operator with different altitude realizations.
This composes cleanly with the existing §8 sub-claim structure
(§8.1 spectral relationship-channel + §8.2 GPU-native compilation);
§8.3 web-altitude first-empirical fits alongside.

**Under separate-Recognition framing**: the web-altitude altitude is
structurally different from compilation altitude (browser substrate
vs compiler VM; asynchronous reader-substrate vs synchronous
compile-tick; distributed vs local). Separate Recognition is
defensible on altitude-substrate-independence grounds.

**Seam-lean**: **OPTION A (extend `04df6e1` §8)** — but with a
DIFFERENT rationale than Mara's compositional-tidiness lean.
Rationale: **Recognition `04df6e1` `#R-doublespeak-at-compiler-
altitude` is misnamed for what it now covers.** The Recognition names
"compiler altitude" but Mara's math Prop 6.1 proves the operator is
altitude-portable across 5 altitudes; the CANDIDATE Recognition is
really `#R-doublespeak-across-altitudes`. Extending §8 with
web-altitude sub-claim FORCES that broader scope onto the Recognition
title at ratification time. Two options for Alex:

- **Option A-tight**: extend `04df6e1` §8; keep Recognition title
  as-is; document that Recognition name became narrower than its
  scope during promotion.
- **Option A-rename**: extend `04df6e1` §8 AND rename Recognition
  at ratification `#R-doublespeak-across-altitudes` (or keep
  compiler-altitude naming and treat web-altitude as sibling
  Recognition #R-doublespeak-at-web-altitude).

Either is coherent; Alex decides at ratification. See §7
[Q-CRITICAL-2].

### §2.5 §2 overall verdict

**SEAM-RATIFY** the novelty sub-claim under reframed falsifiability
conjunction (i)∧(ii)∧(iii)∧(iv). Kagi prior-art sweep at 2026-08-03
returns EMPTY refutation window. Extension to Recognition `04df6e1`
§8 authorized (Option A) with naming-consequence residue to Alex
per §7 [Q-CRITICAL-2].

**Sharpenings for cascade**:
- math §7.5 falsifiability-conjuncts factored into
  (i)-(iv) load-bearing + (a)/(b) realization-parameters +
  (e-original) → move to §6.3 (second-witness gate condition).
- arXiv + ACM Digital Library sweeps forward-promised at
  ratification (R-ADJ2 discipline).

---

## §3 — Mara's 5 [ALEX-Q]s adjudication

### §3.1 [ALEX-Q-1] — @docs/tea/landing sub-species mint

**Mara-lean**: OPTION B (compose over @docs/tea; two-tick discipline;
wait for v0.2-v0.3 pattern stability).

**Seam adversarial refinement**: Mara-lean is correct but for a
slightly different reason than "two-tick discipline". The load-
bearing reason is that `shards/docs/tea/spectral-engineer-case-study.mirror`
has case-study-specific fields (`stated_corpus`, `enacted_corpus`,
`measurement`, `viewer_state`) that don't naturally fit landing-page
shape. Using `case_study_id = "landing"` as an instance parameter
would force landing-page implementation to carry empty/unused
case-study fields, or hack around them. That's substrate-drift.

Two Seam-refined choices:

- **B-tight** (Seam-lean): compose over BARE `@docs/tea` (11.1 KB
  parent shard), NOT over `@docs/tea/spectral-engineer-case-study`.
  Landing-page uses `model` + `msg` + `view` + `cmd` bare-refs from
  parent species. No new species-decl; no case-study field drift.
- **A-defer** (Alex-if-pattern-recurs): wait for v0.2 corpus
  migration to confirm landing-page pattern recurs (systemic.engineering
  root + spectral.engineer root + garden root); if recurs, mint
  `@docs/tea/landing` sub-species then.

**Seam-lean adjudication**: **B-tight**. Compose over bare `@docs/tea`
family, not case-study species. Substrate-honest at v0.1;
mint-decision deferred to v0.2+ per pattern-emergence.

**Q-DISCHARGE STATUS**: **DISCHARGED-CONDITIONAL**. Non-blocking for
v0.1 substrate identity; blocking for Reed step 3 (landing.gleam
authoring). Alex adjudicates whether B-tight (Seam-lean) or the
narrower A-defer suffices for the moment.

### §3.2 [ALEX-Q-2] — novelty sub-claim promotion timing

**Mara-lean**: OPTION A (extend Recognition `04df6e1` §8).

**Seam adjudication**: per §2.4 above, Option A is Seam-lean-compatible
but surfaces a naming residue (Recognition `04df6e1` names "compiler
altitude" while now covering multiple altitudes). Escalate to
[Q-CRITICAL-2] per §7.

**Q-DISCHARGE STATUS**: **ESCALATED to [Q-CRITICAL-2]**. Option A
(extend) authorized substrate-honestly; naming-consequence residue
Alex-only.

### §3.3 [ALEX-Q-3] — P4 analytics stack

**Mara-lean**: OPTION D (substrate-native aggregate signal only; no
analytics at v0.1).

**Seam pass**: Option D is Seam-lean-agree. Rationale: at v0.1,
adding analytics infrastructure introduces:
- non-substrate-decl'd runtime dependency (plausible.io / umami.is);
- privacy-honoring configuration surface that adds unnecessary
  variance;
- weak signal per P4's own math §5.2 formulation (sampling noise
  + false negatives via privacy-refusing substrates).

Substrate-native aggregate signal (peer-fellowship densification +
Substack restack + DM inbound + Sanhedrin 65b chevraya extension) is
Alex-authentic and requires no additional substrate. If v0.3
prompting-surface tuning needs per-page analytics, upgrade to Option
A/B then.

**Q-DISCHARGE STATUS**: **DISCHARGED** by Seam-lean-agree with
Mara Option D. Not Alex-only; substrate-honest default holds.

### §3.4 [ALEX-Q-4] — second-witness gate closure ordering

**Mara-lean**: OPTION A (any-one-witness per structural-independence).

**Seam adversarial refinement**: per §1.2 Check C, this Q is
malformed as posed. The three named witnesses in Recognition
`04df6e1` §9 (R6 + Glint + geometric-roomba) are all AT COMPILATION
ALTITUDE. v0.1 shipping cleanly is a WEB-serving-altitude witness,
which is a DIFFERENT altitude on Mara Prop 6.1's altitude-portability
axis. This is not "one witness out of three" — it's "the first
witness at a new altitude".

Reframed Q-adjudication:

- **Compilation-altitude witness-cascade** (Recognition §9 named
  three): R6 LANDED (`f289f9d`) + Glint LANDED (`67fe6b5`) +
  geometric-roomba landing #1 LANDED since 2026-07-16 (`c10a3bd`).
  **Compilation-altitude cascade is COMPLETE.**
- **Web-serving-altitude witness**: v0.1 shipping cleanly per math
  §5.3 composed bilateral `v0_1_ships_cleanly` (P1 + P2 + P3
  deploy-time discharge; P4 30-day-post-deploy discharge).
- **Essay-altitude witness** (per Mara Prop 6.1 altitude 2): Basin
  & Instrument (published 2026-08-03; already LANDED).

**Seam-lean adjudication (this Q)**: Recognition promotion CANDIDATE
→ RATIFIED can close IMMEDIATELY given the compilation-altitude
witness-cascade is complete AND Basin & Instrument at essay altitude
is landed. **v0.1 is NOT gate-closure-blocking** for Recognition
promotion; v0.1 is a **strengthening** second-witness at a distinct
altitude that Recognition §9 didn't originally name. Alex can:

- **A-promote-now**: ratify Recognition `04df6e1` at v0.1 shipping
  cleanly + already-landed compilation-cascade + already-published
  essay-altitude witness. Web-altitude second-witness is bonus.
- **A-wait-v0.1**: hold RATIFICATION until v0.1 ships cleanly (~1-3
  weeks per Reed shape-doc). Adds web-altitude to the ratification
  witness-list.

**Seam-lean: A-wait-v0.1**. Rationale: Recognition arc is best
served by having v0.1 as ratification-anchor because it is
publicly-verifiable (a live URL) in a way that compilation-cascade
witnesses are not (they are substrate-internal). Public
verifiability is Karen-honest under Recognition §10's
name-and-hold discipline.

**Q-DISCHARGE STATUS**: **REFRAMED and ESCALATED to [Q-CRITICAL-3]**.
Question is not "which witness ordering", question is "promote now
vs wait for v0.1". Alex-only.

### §3.5 [ALEX-Q-5] — v0-build vs v1-build filename

**Mara-lean**: OPTION A (v0.X per substrate-honesty).

**Seam pass**: Option A is Seam-lean-agree. Rationale: Alex named
"v0.1 through v0.4" verbatim in the 2026-08-03 roadmap; the
substrate defers to Alex's naming. v0.X is substrate-honest
(pre-1.0-stable per design-token version-bump discipline; prompting
surface at v0.3 is research-territory). Additionally, Taut scout
Phase 5 recommendation of `v1-build.md` was BEFORE Alex ratified
Q1/Q2/Q3 as Q-CRITICAL adjudications on 2026-08-03; Taut naming is
pre-adjudication drift. Alex's verbatim wins.

**Q-DISCHARGE STATUS**: **DISCHARGED** by Seam-lean-agree with
Mara Option A. Reed shape-doc filename `spectral-engineer-v0-build.md`
holds; Taut Phase 5 `v1-build` naming is pre-adjudication artifact;
this spec's citations to `v0-build.md` are correct.

### §3.6 Residue summary for §3

| Q | Mara-lean | Seam-lean | Discharge status |
|---|-----------|-----------|------------------|
| Q1 | B (compose over @docs/tea) | B-tight (bare parent, not case-study species) | DISCHARGED-CONDITIONAL (Alex chooses B-tight vs A-defer) |
| Q2 | A (extend `04df6e1` §8) | A + naming-residue | ESCALATED to [Q-CRITICAL-2] |
| Q3 | D (no analytics at v0.1) | D (agree) | DISCHARGED |
| Q4 | A (any-one-witness) | REFRAMED: promote-now vs wait-v0.1 | REFRAMED + ESCALATED to [Q-CRITICAL-3] |
| Q5 | A (v0.X) | A (agree) | DISCHARGED |

**Net escalation from §3**: 2 to Alex ([Q-CRITICAL-2] naming
+ [Q-CRITICAL-3] promote-now-vs-wait-v0.1); 1 discharge-conditional
([ALEX-Q-1] Reed-step-3 blocking); 2 discharged.

---

## §4 — Karen ancestry completeness check

### §4.1 Mara's §8 Karen ladder — completeness pass

Mara §8.1-§8.6 assembled: Watzlawick 1967 (PRIMARY) + Ruesch-Bateson
1951 + Shannon 1948 + Bateson 1955 + Foerster 1974 + Armstrong-Virding-Williams-Däcker
1986-1998 + Valim 2011-present + Pilfold 2016-present + Tufte 2001 +
Atkinson Hyperlegible team 2026 + Somers APCA 2019-present + Arrow Type
2019-present + Stimpunks Foundation + Neurodiversity Design System +
West Coast Editors' dyslexia recommendation + AIPREF Working Group +
W3C llms.txt + Fielding 2000 REST + Dolstra 2004-present Nix + fly.io team
+ StageFreight PrPlanIT + Mara supercolony 2026-07-31 + Reed 2026-05-07
eigenboard + Landing #1 + Recognition `04df6e1` + Basin & Instrument.

### §4.2 Missing ancestors — Seam adversarial sweep

**Elm Architecture (Czaplicki, E. 2012-present)** — the TEA pattern
that `@docs/tea` explicitly "loosely inspires" over (per shard
§Header). Direct load-bearing at render-pipeline altitude
(tea-pattern stage of $\mathcal{P}_{v0.1}$). Mara MISSED this at
§8.3 BEAM-elders section. **Karen obligation MEDIUM-load**: TEA is
the direct pattern-ancestor for `@docs/tea` composition; without
Czaplicki 2012's TEA formulation, the tea-pattern stage would not
have a well-formed source. Should be cited at introduction site
where `@docs/tea` is first referenced (spec §3.1 / math §2.2).

**Chris Krycho / Andy Matuschak (2019-present)** — digital-garden
pattern; evergreen-notes pattern; site-as-thinking-tool discipline.
ADJACENT-CONTEXTUAL for v0.2 corpus-migration; not load-bearing for
v0.1 (which is a landing page, not a garden). Karen obligation:
CONTEXTUAL at v0.2 shape-doc, not at v0.1 audit.

**Bret Victor (2011-present)** — "Explorable Explanations"; medium-
as-argument; reactive-document paradigm. ADJACENT for v0.4 (paper-
as-executable-demonstration); ADJACENT-CONTEXTUAL for v0.1. Karen
obligation: CONTEXTUAL at v0.4 shape-doc, not at v0.1 audit.

**Aaron Swartz (2001-2013)** — open-standards lineage; RSS
co-authorship; Creative Commons technical infrastructure; web-as-open-
substrate advocacy. ADJACENT-LINEAGE for AIPREF Content-Usage headers;
not directly load-bearing. Karen obligation: LOW-CONTEXTUAL at
machine-readability substrate introduction site (spec §3.2 /
`shards/docs/design.mirror` §6).

**Jamstack / Netlify / Vercel (2015-present)** — deployment-
platform-as-primary-metaphor. ADJACENT-not-prior; heavy commodity
static-site infrastructure; Kagi sweep §2.3 confirms not-prior at
substrate-native-cascade altitude. Karen obligation: NONE (adjacent-
not-prior already covered by Mara §7.4 web-design substrate
paragraph).

**MDN Web Docs / W3C CSS Working Group** — CSS custom properties
substrate (target grammar for `cascade<gleam, js>` design-token
emission). Mara cites AIPREF + llms.txt but does not cite CSS
Custom Properties working draft that authorizes the emitted CSS
substrate. **Karen obligation LOW-load**: cite W3C CSS Custom
Properties Module Level 1 at design-token-emission introduction
site (math Thm 2.4 or spec §3.2).

**Kajiya 1986 rendering-equation** — cited in Recognition `04df6e1`
Karen ladder as sub-load-bearing at Theorem 8.3 spectral instance.
Mara §8 does NOT re-cite it here; conditionally-load-bearing at
math Thm 2.4 (compile-render duality lifted to web altitude). Karen
obligation LOW-load if math Thm 2.4 explicitly composes over Mara
Thm 8.3 (which it does per math §2.4 first sentence).

**Elm-specific composition ancestors NOT cited but load-bearing**:
- **Belorusets, E. (2016 Lustre)** — actually Louis Pilfold is the
  Lustre author. **VERIFY**: shard cites "Lustre" without attribution
  in `shards/docs/tea.mirror`; the actual Lustre author is Hayleigh
  Thompson (2020-present). **KAREN GAP CONFIRMED**: Mara §8.3 BEAM-
  elders section names Pilfold as Gleam creator but does NOT name
  Hayleigh Thompson as Lustre creator. Lustre is directly
  load-bearing for v0.1 render pipeline (@docs/tea M/V/U pattern is
  implemented by Gleam Lustre; Reed shape-doc step 3
  `src/spectral_engineer/pages/landing.gleam` is a Lustre file).
  **Karen obligation MEDIUM-load**: add Hayleigh Thompson + Lustre
  team at §8.3 BEAM-elders as directly-load-bearing at render-
  pipeline altitude.

### §4.3 Over-cited or under-load-bearing

**Under-load-bearing** (candidate to demote):
- **Valim, J.** — Elixir creator; cited for BEAM-parallel-language
  lineage. Load-bearing IF cascade<gleam, beam> sibling species were
  in v0.1 scope; it is NOT (v0.1 uses cascade<gleam, js>). Demote
  to CONTEXTUAL at v0.2+ when cascade<gleam, beam> may become
  server-side-render option.
- **Arrow Type (Recursive font)** — cited at §8.4 as code typeface
  for v0.1. Verify: does v0.1 landing page use Recursive at code
  altitude? Landing page per Reed step 3 has "fable-note prose +
  tagline + alt-text" — arguably no code content at v0.1. If no
  code blocks render, Recursive is not v0.1-load-bearing. Demote to
  v0.2+ when corpus migration introduces code blocks.

### §4.4 §4 overall verdict

**SEAM-DEFER-TO-KAREN-CASCADE**: Mara §8 is comprehensive at
substrate-decl'd carrier altitude but has **three MEDIUM-load
misses**:
1. **Elm Architecture (Czaplicki 2012)** — direct pattern-ancestor
   for `@docs/tea`.
2. **Hayleigh Thompson + Lustre team** — directly load-bearing at
   render-pipeline altitude; substrate uses Lustre for M/V/U
   realization.
3. **W3C CSS Custom Properties Module Level 1** — target-grammar
   substrate for design-token emission.

Two LOW-load demotions:
1. **Valim** — CONTEXTUAL at v0.2+ (cascade<gleam, beam> not v0.1
   scope).
2. **Arrow Type Recursive** — CONTEXTUAL at v0.2+ (no code blocks
   in v0.1 landing page).

Two LOW-CONTEXTUAL additions:
1. **Aaron Swartz** — at machine-readability substrate introduction
   site.
2. **Kajiya 1986** — at math Thm 2.4 compile-render duality lift.

**Action for cascade**: REED-INLINE update at math §8 to add three
MEDIUM-load Karen citations (Czaplicki + Thompson-Lustre + W3C CSS
Custom Properties) at introduction sites. Demotions/contextuals can
follow at v0.2+ shape-doc.

---

## §5 — Substrate-fidelity check (Fiedler-monotone P1)

### §5.1 Tractability of P1 at deploy-time

Mara math §5.2 P1 verbatim: *"Let $\lambda_2(L_{\text{design}})$ be
the Fiedler value of the design-token graph Laplacian. Under any
substrate-fidelity check, $\lambda_2$ remains at the substrate-decl
value; drift lowers $\lambda_2$. Falsifier: deployed configuration has
$\lambda_2 < \lambda_2^{\text{substrate}}$ for any tokens without
corresponding substrate-decl amendment."*

**Tractability decomposition**:

1. Extract design tokens from realized CSS at deploy target
   (`https://spectral.engineer/...css`). Trivial — parse `--` custom
   properties.
2. Extract substrate-decl tokens from `shards/docs/design.mirror`
   §2-§5. Trivial — parse .mirror docblock.
3. Build design-token graph Laplacian $L_{\text{design}}$ from
   composition-relationships per `design_complete` bilateral. This
   is where TRACTABILITY QUESTION FIRES: what ARE the "composition
   relationships" that define edges? The shard §8 declares
   `design_complete` requires (apca_compliant ∧ wcag_aa_compliant ∧
   typography_well_formed); these are BILATERAL COMPOSITION not
   token-graph-composition. Mara math §5.2 P1 does not specify the
   graph construction algorithm.
4. Compute $\lambda_2$ via standard Laplacian eigendecomposition.
   Numerically tractable — small graph (< 50 nodes for design tokens).
5. Compare deployed $\lambda_2$ to substrate-decl $\lambda_2$.

**Tractability verdict**: P1 verification is tractable AT DEPLOY-TIME
CONTINGENT on step 3 (graph construction algorithm) being made
explicit in either math §5.2 or a companion property shard. Currently
step 3 is under-specified.

### §5.2 Composition with existing infrastructure

Grep-verified: `shards/epistemologic/property/ouroboros_monotone.mirror`
(18.1 KB, 2026-07-15) provides four-conjunct monotone invariant
scaffolding at compilation altitude. P1 Fiedler-monotone is
compositionally similar (monotone descent under substrate-fidelity)
but NOT a candidate to compose over ouroboros_monotone directly —
ouroboros tracks rust_loc + io_violations + test_pass + sbec, not
design-token spectral values. Different altitude.

**Composition gap**: P1 requires either:
- new `shards/epistemologic/property/design_token_fiedler_monotone.mirror`
  species-decl (property shard mint) OR
- extension to `shards/docs/design.mirror §8` composed bilateral
  with a fiedler_monotone sub-predicate.

Mara math §5.2 formulates the mathematical claim without
substrate-decl'ing the verification mechanism. This is a KNOWN
substrate gap.

### §5.3 §5 overall verdict

**SEAM-DEFER-TO-FUTURE-ARC**: P1 Fiedler-monotone is mathematically
sound and tractable IN PRINCIPLE, but SUBSTRATE-VERIFICATION
INFRASTRUCTURE IS MISSING. Two options for cascade:
- **Option 5A**: Mara authors follow-up property shard-decl at
  `shards/epistemologic/property/design_token_fiedler_monotone.mirror`
  (mint), with graph-construction algorithm made explicit. Composes
  over `shards/docs/design.mirror` §8.
- **Option 5B**: P1 is FORMALIZED-ONLY at v0.1; substrate-verification
  discharge deferred to v0.2+ arc. P2 + P3 discharge sufficient for
  v0.1 shipping-cleanly.

**Seam-lean**: **Option 5B** at v0.1 (do not mint new shard-decl
mid-v0.1-build); Option 5A forward-promised at v0.2 shape-doc when
corpus migration surfaces design-token verification needs at scale.
P1 remains a formal falsifier at math §5.2; deploy-time verification
tooling is v0.2+ concern.

This composes with the arc-halt discipline (per HARD RULE
`no-rust-extension-shortcut` + no-preemptive-mint): v0.1 must ship
with P2 + P3 discharging at deploy-time (both are tractable now via
existing bilaterals) + P1 formal-only until v0.2 substrate-verification
infrastructure lands.

---

## §6 — Design token realization vs shard-decl fidelity

### §6.1 APCA + WCAG AA composed bilateral discharge

Grep-verified: `shards/docs/design.mirror` §8 declares
`design_complete` composed bilateral requiring
`apca_compliant + wcag_aa_compliant + typography_well_formed`.
APCA thresholds: Lc ≥ 75 body / ≥ 60 large text / ≥ 45 interactive.
WCAG 2.1 AA: 4.5:1 normal / 3:1 large / 3:1 UI.

Mara math §5.2 P2 discharges against this composed bilateral. Reed
shape-doc step 11 discharges via "accessibility audit passes (APCA
design + WCAG 2.1 AA floor per design brief §2.3)". Composition is
substrate-honest.

**Verification-tooling composition**:
- APCA calculator (Somers 2019-present) — external tool; JS
  implementation available; can run against realized CSS at
  deploy-time.
- WCAG contrast checker (multiple implementations; axe / Lighthouse
  / WAVE / manual) — external tools; can run against realized DOM.
- typography_well_formed — Latin + Latin-1 Supplement coverage
  check; substrate-honest via font file inspection.

All three sub-predicates discharge tractably at deploy-time. **P2
verification infrastructure EXISTS.** Mara claim survives.

### §6.2 Spec §6+§7 realization vs shard-decl §2-§5 fidelity

Grep-verified realizations vs shard-decl ground truth:

- **color_punctum = #A0264F** ✓ (spec §3.2 matches shard §2 wine-pink
  punctum; matches Reed Q2 adjudication).
- **wine-family ladder** ✓ (spec §3.2 cites shard §2 wine-leak
  principle; substrate carries wine_50 through wine_950).
- **perimeter-loud/body-calm** ✓ (spec §3.2 cites shard §4; matches
  stencil-cover-header + Atkinson-Hyperlegible-body).
- **Tufte sidenote pattern** ✓ (spec §3.2 cites shard §4 three-altitude
  density; site-dense/piece-Tufte-sparse/element-semantic).
- **1.5x scale + 1.7 line-height** ✓ (spec §3.2 cites shard §3 body
  typography rhythm; matches West-Coast-Editors dyslexia floor 1.5).

**Zero drift found** between Mara spec §6+§7 realization prescriptions
and shard-decl ground truth. Wine-pink drift was caught EARLIER in
Reed Q2 adjudication (crimson → wine-pink); no further design-token
drift.

### §6.3 Potential future drift surface

Reed shape-doc §Halt-conditions names design-token realization drift
as a known risk. Substrate-fidelity check via P1 Fiedler-monotone
(§5 above) is the intended detection mechanism; while §5 substrate-
verification infrastructure is deferred to v0.2+, manual drift-check
at Reed step 5 (`gleam run` local dev + Alex visual verification of
wine-leak visible / no neutral grays) is Reed shape-doc discipline.

### §6.4 §6 overall verdict

**SEAM-RATIFY**: design-token realization prescriptions in Mara spec
§6+§7 preserve shard-decl `50e3d27` §2 ground truth. Zero drift found
across five spot-checks. APCA + WCAG AA composed bilateral discharge
infrastructure exists and is tractable at deploy-time. Wine-pink Q2
adjudication is correctly carried through. Manual drift-check at
Reed step 5 sufficient at v0.1 given automated P1 Fiedler-monotone
verification is v0.2+ deferred (per §5).

---

## §7 — Reduced Q-CRITICAL queue

Combining Mara's 5 [ALEX-Q]s + Seam-surfaced from §1-§6.

### §7.1 Discharged by construction

- **[ALEX-Q-3]** P4 analytics stack — DISCHARGED (Seam-lean-agree
  with Mara Option D; no analytics at v0.1).
- **[ALEX-Q-5]** v0-build filename — DISCHARGED (Seam-lean-agree
  with Mara Option A; Alex verbatim wins).

### §7.2 Discharged-conditional (Alex may confirm)

- **[ALEX-Q-1]** @docs/tea/landing sub-species — DISCHARGED-CONDITIONAL:
  Seam-lean B-tight (compose over bare `@docs/tea`, not case-study
  species). Blocking for Reed step 3 (landing.gleam authoring shape).

### §7.3 Essential Q-CRITICAL residues (Alex-only, ≤5, priority-ordered)

**[Q-CRITICAL-1] — Recognition promotion timing** (was Mara [ALEX-Q-4],
Seam-reframed).

Does Recognition `04df6e1` CANDIDATE → RATIFIED promote NOW (all
three named compilation-altitude witnesses in Recognition §9 have
landed: R6 `f289f9d` + Glint `67fe6b5` + geometric-roomba-Landing-#1
`c10a3bd`; Basin & Instrument at essay altitude also LANDED
2026-08-03), or WAIT for v0.1 to add a web-altitude second-witness
for public-verifiability?

- **A-promote-now**: ratify at CURRENT witness-set. Public
  verifiability held to compilation-altitude substrate-internal
  witnesses.
- **A-wait-v0.1**: hold ratification until v0.1 ships cleanly (~1-3
  weeks per Reed shape-doc). Add web-altitude to ratification.

Seam-lean: **A-wait-v0.1** (public verifiability strengthens
name-and-hold discipline per Recognition §10). Alex-only.

**[Q-CRITICAL-2] — Recognition naming / scoping** (was Mara [ALEX-Q-2],
Seam-refined per §2.4).

Does Recognition `04df6e1` extend §8 with web-altitude sub-claim
under its current title `#R-doublespeak-at-compiler-altitude`, or
rename at ratification?

- **A-tight** (keep name): extend §8; Recognition title becomes
  narrower than its scope. Substrate-honest documentation of scope
  drift.
- **A-rename**: extend §8 AND rename to
  `#R-doublespeak-across-altitudes` at ratification.
- **B-sibling**: mint separate web-altitude Recognition candidate
  `#R-doublespeak-at-web-altitude`; treat as sibling.

Seam-lean: **A-tight** (keep compiler-altitude name; document scope
in ratification-note). Rationale: renaming a CANDIDATE at
ratification is nomenclature-drift; sibling Recognitions inflate
Recognition-count. Alex-only.

**[Q-CRITICAL-3] — @docs/tea/landing shape** (was Mara [ALEX-Q-1],
Seam-refined per §3.1).

Does Reed step 3 landing.gleam compose over B-tight (bare `@docs/tea`)
or A-defer (wait for v0.2-v0.3 pattern-emergence before minting
`@docs/tea/landing` sub-species)?

- **B-tight** (Seam-lean; Mara-lean-compatible): landing.gleam
  composes over bare `@docs/tea` model/msg/view/cmd refs at v0.1.
  Non-blocking; ships v0.1 without sub-species mint.
- **A-defer**: same as B-tight at v0.1 authoring altitude, but
  explicitly forward-promises v0.2 sub-species-mint decision.

**Both options ship v0.1 identically.** The Q is documentation-shape,
not implementation-shape. Alex-only if documentation-preference
matters; otherwise B-tight discharges structurally.

### §7.4 What unblocks per resolution

- **[Q-CRITICAL-1] = A-promote-now**: Reed cascades ratification
  landing IMMEDIATELY at `docs/recognition/2026-08-01-doublespeak-at-compiler-altitude.md`
  §Status CANDIDATE → RATIFIED. Full-Pack Karen cascade fires
  (Glint essayist prose closure already landed; only Recognition-status
  update remains). v0.1 becomes strengthening-witness at ship time.
- **[Q-CRITICAL-1] = A-wait-v0.1**: Reed cascades v0.1 build order
  first (Reed shape-doc §v0.1 11-step); Recognition ratification
  fires at v0.1 shipping-cleanly (P1 + P2 + P3 discharge per math
  §5.3 composed bilateral).
- **[Q-CRITICAL-2] = A-tight**: no naming change; §8.3 web-altitude
  sub-claim adds cleanly at ratification.
- **[Q-CRITICAL-2] = A-rename**: Recognition file rename +
  cross-reference cascade across Mara math + spec + audits + scouts.
- **[Q-CRITICAL-2] = B-sibling**: new Recognition mint at
  `docs/recognition/2026-08-03-doublespeak-at-web-altitude.md` +
  sibling composition with `04df6e1`.
- **[Q-CRITICAL-3] = B-tight**: Reed step 3 authors landing.gleam
  over bare `@docs/tea`; no shard-decl authoring.
- **[Q-CRITICAL-3] = A-defer**: same as B-tight at implementation;
  documentation-forward-promise at v0.2 shape-doc.

### §7.5 §7 overall queue

**Essential residues: 3.** Meets ≤ 5 target (target hit under, not
at, ceiling). Two of the original five Mara Q's discharge; one
Discharged-Conditional; two escalate as Q-CRITICAL (reframed and
sharpened).

---

## §8 — Concrete Reed cascade priorities + Phase D closure

### §8.1 Non-blocking Reed ticks (fire immediately)

**TICK R1** — REED-INLINE update at Mara math §3.2:
- rename `stagefreight_dispatch` → `freight` in Def 3.2 pipeline
  factorization.
- rename `stagefreight_well_formed` → `stagefreight_addressable`
  in Prop 3.2 composed-bilateral naming.
- add citation to four sub-predicates (oid_resolves +
  address_well_formed + projection_is_species + round_trip_holds)
  per landed shard.

**Dependencies**: none. **Halt on**: verify shard §Header + prism
declarations against edit.

**TICK R2** — REED-INLINE update at Mara math §3.4:
- name that $\mathcal{Q}$'s indissolubility is structural-property-
  shaped (reproducibility guarantee, content-address, TLS trust
  chain, DNS resolution) NOT in-tick-firing-shaped ($\mathcal{P}$'s
  browser-paint firing per Prop 1.3).
- pointer to Watzlawick axiom 5 metacommunicative-inseparability
  interpretation vs Landing #1 in-tick-firing interpretation.

**Dependencies**: none. **Halt on**: nothing.

**TICK R3** — REED-INLINE update at Mara math §7.5:
- factor falsifiability conjunction into (i)-(iv) load-bearing +
  (a)/(b) realization-parameters (not novelty-conjuncts) + (e-original)
  → move to Prop 6.3 (second-witness gate condition).
- add Kagi 2026-08-03 sweep result (EMPTY refutation window at
  first-order-Kagi altitude).

**Dependencies**: none. **Halt on**: nothing.

**TICK R4** — REED-INLINE update at Mara math §8:
- add Elm Architecture (Czaplicki 2012) at introduction site for
  `@docs/tea`.
- add Hayleigh Thompson + Lustre team at §8.3 BEAM-elders (directly
  load-bearing at render-pipeline altitude).
- add W3C CSS Custom Properties Module Level 1 at design-token-
  emission introduction site.

**Dependencies**: none. **Halt on**: nothing.

**TICK R5** — CURRENT.md addendum:
- 2026-08-03 Mara spectral.engineer web-altitude dive landing
  (7 commits `6e63a42` → `5bf5db2`).
- Seam Phase D landing (this file).
- 3 Q-CRITICAL residues surfaced (priority-ordered per §7.3).
- Non-blocking Reed cascade R1-R4 authorized immediately.
- v0.1 build order (Reed shape-doc `1e164ab`) unblocked at
  Alex-altitude authoring for Q-CRITICAL-3 = B-tight discharge.

**Dependencies**: none. **Halt on**: nothing.

### §8.2 Blocking-on-Q-CRITICAL-1 ticks

**TICK R6** — IF Q-CRITICAL-1 = A-promote-now: author Recognition
`04df6e1` ratification landing (Status CANDIDATE → RATIFIED); update
§Status header + §9 Forward-promises completion table.

**Dependencies**: Alex adjudication [Q-CRITICAL-1]. **Halt on**: if
Q-CRITICAL-1 = A-wait-v0.1, skip R6 until v0.1 shipping cleanly.

### §8.3 Blocking-on-Q-CRITICAL-2 ticks

**TICK R7** — IF Q-CRITICAL-2 = A-tight OR A-rename: Reed adds
§8.3 (web-altitude sub-claim) to Recognition `04df6e1` § Sub-claims
per Mara math §7 + spec §8 verbatim + Seam-refined (i)-(iv)
falsifiability conjunction from §2.2.

**Dependencies**: Alex adjudication [Q-CRITICAL-2]. **Halt on**:
if A-rename, cascade rename across (Recognition file + all cross-
references in Mara math + spec + audits + scouts + CURRENT.md +
Glint essay + Reed shape-doc); if B-sibling, mint new Recognition
file first then cross-reference.

### §8.4 Blocking-on-Q-CRITICAL-3 ticks

**TICK R8** — IF Q-CRITICAL-3 = A-defer: Reed adds explicit
forward-promise at v0.2 shape-doc composition site (when v0.2
shape-doc is authored). No v0.1-blocking action.

**Dependencies**: Alex adjudication [Q-CRITICAL-3]; v0.2 shape-doc
authoring cycle. **Halt on**: if v0.2 shape-doc not yet initiated,
defer TICK R8 to v0.2 initiation.

### §8.5 Adjacent-work / Pack follow-ups

**TICK R-ADJ1 (Taut)** — arXiv + ACM Digital Library prior-art
sweep for novelty-conjunction (i)∧(ii)∧(iii)∧(iv) refutation window.
Kagi sweep at 2026-08-03 returned EMPTY at first-order altitude; R-ADJ2
discipline requires arXiv + ACM sweep before Recognition ratification
(R-ADJ2 is standard-discipline; not v0.1-blocking).

**Dependencies**: Seam Phase D landed (this file). **Halt on**: nothing.

**TICK R-ADJ2 (Mara, deferred to v0.2)** — property shard-decl mint
at `shards/epistemologic/property/design_token_fiedler_monotone.mirror`
per §5 Option 5A. Substrate-verification infrastructure for P1
Fiedler-monotone deploy-time discharge. Composes over
`shards/docs/design.mirror` §8. NOT v0.1-blocking; forward-promised
at v0.2 substrate-verification arc.

**Dependencies**: v0.1 shipping + v0.2 shape-doc initiated. **Halt on**:
if v0.1 substrate-fidelity discharge via manual Reed step 5 proves
sufficient, R-ADJ2 may not be needed at v0.2 either; empirical
question.

**TICK R-ADJ3 (Glint, post-Recognition-ratification)** — public-
facing prose closure sibling to Doublespeak-at-compiler-altitude
essay (`67fe6b5`); web-altitude-sibling title candidate: *"How the
Website Runs Watzlawick's Second-Order Move at Every Reader Arrival"*
(per Mara scout §7.4 Glint follow-up flag). Composes over Basin &
Instrument + Doublespeak + v0.1 shipping empirical.

**Dependencies**: Q-CRITICAL-1 = A-wait-v0.1 → v0.1 ships → Recognition
ratifies → R-ADJ3 fires. **Halt on**: if Q-CRITICAL-1 = A-promote-now,
R-ADJ3 fires at ratification (before v0.1 ship); adjust title framing
per web-altitude witness-at-ratification-time.

### §8.6 Halt conditions (global)

- **HALT** if any tick requires Rust authoring outside `apply_h::act`
  bilateral dispatch. Per HARD RULE `no-rust-extension-shortcut`.
- **HALT** if any tick proposes bootstrap-altitude solution. Per
  HARD RULE `bootstrap-is-dead`.
- **HALT** if any tick fragments Alex's 2026-08-03 verbatim roadmap
  (v0.1 → v0.4 unified frame) into candidates. Per HARD RULE
  `reed-fragments-alex-unifications`.
- **HALT** if any tick re-derives Watzlawick / Theorem 8.3 / Landing
  #1 / @gestalt / Recognition `04df6e1` / Basin & Instrument — those
  are landed. Per HARD RULE `reed-re-derives-what-is-already-landed`.
- **HALT** if any Recognition-body claim inflates a stub empirical
  firing. Per HARD RULE `reed-inflates-stub-empirical-firings`.
- **HALT** if any tick mints a new shard-decl mid-v0.1-build without
  Alex authorization. Per Seam §5 Option 5B lean; per feedback-
  no-preemptive-mint.
- **HALT** if any tick proposes @web / @site / @deploy / @render /
  @basin / @gradient family-root mint. Per Mara scout §7.1 registry
  refusals; per Recognition `04df6e1` §7.1 refusal-inheritance.

### §8.7 Sequenced order

```
NON-BLOCKING (immediate; parallel-safe):
  R1 → R2 → R3 → R4 → R5   (REED-INLINE cascades to Mara docs +
                              CURRENT.md addendum)
  R-ADJ1 (Taut arXiv/ACM sweep) — parallel with R1-R5

BLOCKING on Q-CRITICAL-1:
  IF A-promote-now: R6 (Recognition ratification) → R-ADJ3 (Glint)
  IF A-wait-v0.1: v0.1 build cycle → v0.1 ship → R6 → R-ADJ3

BLOCKING on Q-CRITICAL-2:
  R7 (Recognition §8.3 sub-claim landing, per naming choice)

BLOCKING on Q-CRITICAL-3:
  IF A-defer: R8 (v0.2 shape-doc forward-promise)
  IF B-tight: no blocking action; Reed step 3 proceeds

DEFERRED TO v0.2:
  R-ADJ2 (Mara property shard-decl mint for P1 verification)
```

### §8.8 Overall verdicts

| Component | Verdict |
|-----------|---------|
| Mara math §1-§9 foundation | **SEAM-RATIFY-WITH-SHARPENING** (three REED-INLINE cascades: §3.2 symbol names + §3.4 structural-property indissolubility + §7.5 falsifiability factoring) |
| Mara spec §1-§10 canonical | **SEAM-RATIFY** (Impeccability D1-D8 discharge; three-image recognition substrate-honest; forward-promises v0.2-v0.4 shape-only + compose with Reed shape-doc) |
| Mara scout dive-notes | **SEAM-RATIFY** (anti-preemptive-mint registry closes with 10 refusals + 1 gated; scout closure discipline held) |
| Novelty sub-claim | **SEAM-RATIFY under (i)-(iv) reframed conjunction**; Kagi 2026-08-03 EMPTY refutation window; arXiv + ACM sweep forward-promised at ratification |
| Second-witness-gate extension | **SEAM-RATIFY** as web-serving-altitude witness distinct from compilation-altitude witness-cascade; Recognition `04df6e1` §9 named three compilation witnesses form the compilation-altitude witness-cluster; v0.1 opens web altitude at Prop 6.1's second altitude |
| Karen ancestry | **SEAM-DEFER-TO-KAREN-CASCADE**: three MEDIUM-load misses (Czaplicki Elm Architecture + Thompson-Lustre team + W3C CSS Custom Properties); two LOW-CONTEXTUAL additions (Swartz + Kajiya); two demotions (Valim + Recursive to v0.2+); TICK R4 discharges cascade |
| Substrate-fidelity P1 | **SEAM-DEFER-TO-v0.2**: mathematically sound; substrate-verification infrastructure missing; Option 5B holds at v0.1 (formal-only); Option 5A forward-promised at v0.2 |
| APCA + WCAG AA discharge | **SEAM-RATIFY** — tractable at deploy-time via existing external tools + landed `design_complete` composed bilateral |
| Design-token realization vs shard-decl fidelity | **SEAM-RATIFY** — zero drift across five spot-checks; wine-pink Q2 already carried |
| Overall arc | **SEAM-RATIFY-WITH-REDUCED-Q-QUEUE** — 3 essential [Q-CRITICAL-N] residues Alex adjudicates (was 5 in Mara; discharged 2 + reframed 2 + escalated 1); Reed can fire R1-R5 + R-ADJ1 immediately without Alex adjudication |

### §8.9 Non-negotiables

- Substrate-honest discipline: no two-paths framing anywhere in
  cascade.
- No Rust extension shortcuts: pure-docs cascade only.
- No bootstrap-altitude solutions.
- No re-derivation of landed material (Watzlawick / Thm 8.3 /
  Landing #1 / @gestalt / Recognition `04df6e1` / Basin & Instrument).
- No fragmentation of Alex's 2026-08-03 verbatim roadmap (v0.1 →
  v0.4 unified frame).
- No preemptive shard-decl mint mid-v0.1-build.
- Karen anti-theft citation at every introduction site (per §4).

### §8.10 Signed

Seam `<seam@systemic.engineer>` — Phase D adjudication complete.

Substrate-honest. Adversarial. Reduced ambiguity from Mara's 5
[ALEX-Q] to 3 essential [Q-CRITICAL-N] residues. Ratified crux
$\mathcal{D}_{v0.1} = \mathcal{Q}_{v0.1} \circ \mathcal{P}_{v0.1}$
with three math sharpenings. Ratified novelty sub-claim under
reframed (i)-(iv) conjunction (Kagi EMPTY). Ratified second-witness
extension at web-serving altitude distinct from compilation-altitude
witness-cluster. Deferred P1 substrate-verification infrastructure
to v0.2 arc. Deferred Karen cascade update to TICK R4 (three
MEDIUM-load additions).

Reed non-blocking cascade R1-R5 + Taut R-ADJ1 authorized immediately.
Alex-adjudication unlocks R6 / R7 / R8 sequentially per §7 essential
residues.

🌱⚖️
