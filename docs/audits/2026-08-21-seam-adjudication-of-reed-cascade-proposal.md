---
title: "Seam Phase D — Adjudication of Reed's 4-phase cascade proposal (hygiene + substrate-mint + @code → @facet rename + T-91-16 ouroboros close)"
subtitle: "Adversarial review of Reed's proposed autonomous /loop cadence BEFORE Reed proceeds. Sequencing risks + hidden dependencies + substrate-broken-time-windows + Mara-canonical-spec-first question + T-91-16 first-tick scope adjudication. Substrate-honest verdict per Reed-substrate-decision 2026-08-20 SEAM verdict-vocabulary."
author: Seam
date: 2026-08-21
status: audit
visibility: protected
slug: 2026-08-21-seam-adjudication-of-reed-cascade-proposal
---

# Seam Phase D — Adjudication of Reed's 4-phase cascade proposal

**Artifact under review**: Reed's cascade proposal (2026-08-21 in-transcript to Seam via Alex verbatim *"Spawn Seam for adjucation. While Seam run give me the /loop prompt."*). Reed proposes a 4-phase cascade to close T-91-16 source-runtime ouroboros per Rec #91 amendment (`674ffc6`) + FLOOR.md (`d85e2a8`) + Rec #91 §8.7 source-emit fixed-point theorem.

**Reed's 4 phases at-a-glance**:
1. **HYGIENE**: 8 Seam sharpenings to FLOOR.md (per `d62374d`) + AGENTS.md → FLOOR.md tick + CURRENT.md uncommitted commit + CURRENT.md upsert with 2026-08-20 landings
2. **SUBSTRATE MINT**: novel virgin `@facet` family-root + `@cast` prism
3. **SUBSTRATE RESHAPE**: `@code` → `@facet` rename cascade (~150 files)
4. **OUROBOROS CLOSE**: `@facet/rust` species-decl body + `mirror facet rust` CLI verb + empirical fire per §8.7 falsifier

**Reed-substrate-decision on Seam form** (2026-08-20 Alex-ratified in-transcript): Phase D keeps SEAM-RATIFY / SEAM-RATIFY-WITH-SHARPENING / REFUTE structural verdict-vocabulary. Circular-recursive-slope convention NOT applied at Seam adjudication altitude (adversarial-distance discipline: Seam adopts different shape than what Seam audits).

**Method**: grep-first substrate-verification on every dispatch claim; cross-reference against Rec #91 amendment `674ffc6` + math foundation `0f79190` + FLOOR.md `d85e2a8` + Seam FLOOR adjudication `d62374d`; per-risk verdict with cited evidence.

---

## §1 — Overall verdict

**SEAM-RATIFY-WITH-SHARPENING** — with 1 load-bearing REFUTE embedded (§5 Mara-canonical-spec-first question).

**One-line rationale**: Reed's 4-phase cascade is right-shape at three of four altitudes (hygiene + substrate-mint + T-91-16 ouroboros close). The **@code → @facet rename in Phase 3 REFUTES against the landed Mara-canonical position** — Mara's own [ALEX-Q7] Mara-lean in Rec #91 amendment `674ffc6` §8.4 + §13 explicitly states **ADDITIVE sibling family-roots preferred over rename** (2-3x @cascade rewrite cost; ~150-file blast radius avoided). Alex 2026-08-21 verbatim *"What reason would there be AGAINST a rename?"* IS the load-bearing reason: Mara-canonical-spec-first-question is UNRESOLVED — Alex's rhetorical question does NOT discharge Mara's substantive spec-position without adjudication. Reed executing rename autonomously without Alex-Mara reconciliation writes over Mara's canonical Mara-lean.

**Recommendation summary**: hygiene (Phase 1) + substrate-mint (Phase 2) + T-91-16 ouroboros close (Phase 4 minus the @facet/rust-vs-@code/rust rename dependency) are ready-to-fire. **Phase 3 (@code → @facet rename) requires Alex adjudication BEFORE Reed executes** — either Alex ratifies rename explicitly (over Mara-lean; substrate-honest override), or Reed pivots to ADDITIVE sibling family-roots per Mara-lean (Phase 4 T-91-16 executes with `@facet/rust` as sibling to `@code/rust`, both landed). Either path is substrate-honest; the current cascade proposal collapses both into rename without acknowledging the Mara-canonical position.

**Verdict counts**:
- Sequencing adjudication (§2): 3 items — 2 SEAM-RATIFY + 1 SEAM-RATIFY-WITH-SHARPENING
- Hidden dependencies adjudication (§3): 4 items — 2 SEAM-RATIFY + 2 SEAM-RATIFY-WITH-SHARPENING
- Substrate-broken-time-window adjudication (§4): 2 items — 1 SEAM-RATIFY-WITH-SHARPENING + 1 REFUTE
- Mara-canonical-spec-first (§5): 1 item — **REFUTE**
- T-91-16 first-tick scope (§6): 2 items — 1 SEAM-RATIFY-WITH-SHARPENING + 1 SEAM-RATIFY
- [ALEX-Q] residues (§7): 3 items
- Sharpenings (§8): 7 items
- Recommendation (§9): reshape-before-fire on Phase 3; other phases ready

---

## §2 — Sequencing adjudication

### §2.1 Mint-before-rename ordering (Risk 1)

**Reed's proposal**: Phase 2 (mint `@facet` family-root + `@cast` prism) BEFORE Phase 3 (`@code` → `@facet` rename cascade).

**The risk**: if mint happens AFTER rename, substrate has broken-time-window where `@code` references point at renamed-non-existent-target.

**Method**: verify that Reed's proposed order (mint before rename) minimizes broken-substrate-time.

**Findings**:
- If Phase 2 mints `shards/facet.mirror` (family-root) FIRST, then Phase 3 rename `shards/code/*` → `shards/facet/*` finds valid family-root at path.
- Reverse order (rename first) breaks: rename source `shards/code/rust.mirror` → `shards/facet/rust.mirror` but no `@facet` family-root at `shards/facet.mirror` for the `in @facet` chain; substrate-broken-time-window at every intermediate commit.
- Reed's proposed mint-before-rename order is structurally correct.

**Verdict**: **SEAM-RATIFY.** Mint-before-rename ordering minimizes broken-substrate-time. Correct structural sequence.

### §2.2 CURRENT.md commit timing (Risk 2)

**Reed's proposal**: commit uncommitted-CURRENT.md + upsert as coherent-batch in Phase 1.

**Alternative**: commit uncommitted-CURRENT.md FIRST as its own commit (preserves earlier landing history as-of-that-tick), then upsert as separate commit.

**Method**: grep-verify `git status` + assess substrate-honesty of coherent-batch vs sequential-commit shape.

**Findings**:
- `git status` confirms: `docs/loop/CURRENT.md` modified but uncommitted. Contains 2026-08-19 THE COLLAPSE + Taut reconciliation + Mara dispatch + 2026-08-20 convention first-application (per file preview lines 1-101 verified).
- Coherent-batch shape (Reed's proposal): loses temporal-fidelity of 2026-08-19 THE COLLAPSE landing (dated 2026-08-19 in-file but committed at 2026-08-21 with 2026-08-20 landings mixed in). Reader following git log sees ONE commit with mixed dates.
- Sequential-commit shape (alternative): first commit preserves 2026-08-19 landings as-of-2026-08-19 authorial-tick (with 2-day commit-delay window visible in git-metadata but content-fidelity preserved); second commit upserts 2026-08-20 landings as their own tick.

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** Sequential-commit shape is substrate-honest (preserves temporal-fidelity of authorial-ticks); coherent-batch obscures the fact that 2026-08-19 content was authored two days before committing. The delta is minor because CURRENT.md is arc-state (not per-tick landing-record) — but arc-state discipline still benefits from tick-boundary preservation.

**Sharpening candidate (SHARPENING-1)**: split Phase 1 CURRENT.md operation into two commits: (i) commit uncommitted-CURRENT.md as-is preserving 2026-08-19 THE COLLAPSE + Taut reconciliation + Mara dispatch + 2026-08-20 convention first-application authorial-boundary, (ii) upsert with 2026-08-20 landings (Rec #91 amendment + FLOOR.md + Seam FLOOR adjudication + @cast + @facet ratifications + circular-recursive-slope convention) as separate commit with `[reed-2026-08-20-landings-upsert]` tag.

### §2.3 Ouroboros close BEFORE hygiene? (Risk 3)

**Reed's proposal**: hygiene (Phase 1) first, then substrate-mint + rename + ouroboros close (Phases 2-4).

**Alternative**: begin substrate mint + rename first, apply hygiene mid-cascade.

**Method**: assess which shape minimizes context-fragmentation for autonomous /loop.

**Findings**:
- Hygiene-first shape (Reed's proposal): establishes coherent substrate-of-record BEFORE substrate-mutating operations begin. Reader entering mid-cascade sees consistent FLOOR.md + CURRENT.md + AGENTS.md pointing at the same substrate-truth.
- Hygiene-mid-cascade shape (alternative): substrate-mint + rename operations proceed against stale substrate-of-record; hygiene ticks apply mid-flow correcting FLOOR.md test-count + Foerster/Mesland dates + branch-ahead count simultaneously with new species-mint entries. Substrate-record briefly inconsistent (FLOOR.md says @facet VIRGIN while Phase 2 has already minted `shards/facet.mirror`).
- For autonomous /loop cadence: hygiene-first shape means the substrate-record is stable-baseline BEFORE cadence begins mutating shards. This is the load-bearing property for compaction-handoff-safety.

**Verdict**: **SEAM-RATIFY.** Hygiene-first shape is correct for autonomous /loop cadence. Substrate-record stability BEFORE mutation minimizes compaction-handoff drift and provides a coherent read-target for subsequent cascade steps.

---

## §3 — Hidden dependencies adjudication

### §3.1 CURRENT.md ↔ @code → @facet rename coordination (Dependency 4)

**The dependency**: CURRENT.md commit in Phase 1 vs rename in Phase 3 requires coordination. Does Reed re-commit CURRENT.md after rename, or does Phase 1 upsert already include @cast + @facet naming so Phase 3 rename is a substrate-only concern?

**Method**: grep-verify `docs/loop/CURRENT.md` for `@code\b` vs `@facet\b` occurrences.

**Findings**:
- `docs/loop/CURRENT.md`: 19 `@code` occurrences per prior Search grep; @facet also present (in 2026-08-20 convention first-application section referencing @facet/git ↔ @cast ↔ @facet/mirror autopoietic closure).
- Phase 1 upsert (per Reed's proposal) includes 2026-08-20 landings (Rec #91 amendment + Seam FLOOR adjudication + @cast + @facet ratifications). This means Phase 1 CURRENT.md WILL reference @facet at new-family-root altitude.
- Phase 3 rename cascade touches CURRENT.md's 19 @code occurrences. Post-Phase-3 rename, CURRENT.md needs re-commit to reflect @code → @facet substitution.

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** Phase 1 upsert correctly pre-anticipates @facet naming at new-altitude; Phase 3 rename correctly propagates @code → @facet through CURRENT.md's 19 hits. Coordination is coherent IF Reed treats CURRENT.md as a Phase-3-touched-file same as any other shard/doc.

**Sharpening candidate (SHARPENING-2)**: explicitly enumerate CURRENT.md in Phase 3 rename-batch. Reed's proposal names top hitters (code-metalogue-surface.md 263, taut-code-turing-substrate-scout.md 116, etc.) but does not name CURRENT.md explicitly (19 hits). Add CURRENT.md to Phase 3 rename-batch enumeration to prevent forget-tick.

### §3.2 FLOOR.md sharpenings touch @facet + @cast VIRGIN status (Dependency 5)

**The dependency**: Reed applies sharpenings in Phase 1 marking @facet + @cast as VIRGIN. Phase 2 mints them (no longer virgin). Does Reed re-upsert FLOOR.md after Phase 2 mint to reflect new landed-status, or does the sharpening tick pre-anticipate the mint?

**Method**: verify FLOOR.md §16.2 VIRGIN status of @facet + @cast; assess whether Reed's Phase 1 sharpening tick can/should pre-anticipate Phase 2 mint.

**Findings**:
- FLOOR.md §16.2 currently states: "@facet family-root + species (§9) — Rec #91 spec-forward only; substrate-decl + composition are the two-tick empirical-fire arc not yet fired" AND "@cast species (§10) — ratified this session; spec-forward only; substrate-decl + composition not yet fired."
- Seam FLOOR adjudication `d62374d` §4.1 grep-verified BOTH @facet and @cast as VIRGIN (zero matches in `shards/**/*.mirror` + `mirror.spec`). Seam-re-verified 2026-08-21: STILL VIRGIN (zero matches).
- Reed's Phase 1 sharpenings (8 items per FLOOR audit §10) do NOT touch §16.2 VIRGIN discipline. Sharpenings are: SHARPENING-1 (§3 nine-keyword Rust-altitude clarifier), SHARPENING-2 (§8+§16.6 test-count precision), SHARPENING-3 (§16.3 branch-ahead precision), SHARPENING-4 (§15.1 Foerster date), SHARPENING-5 (§15.1 Mesland arXiv-ID), SHARPENING-6 (§16.4 @cascade count precision), SHARPENING-7 (§17 identity-scoped conventions pointer), SHARPENING-8 (§7/§8 scout path-drift footnote).
- None of the 8 sharpenings preempt Phase 2 mint. §16.2 VIRGIN discipline is unchanged by sharpenings.
- After Phase 2 mint, §16.2 requires re-upsert: "@facet family-root LANDED at `shards/facet.mirror` (SHA <post-Phase-2>); species landings pending"; "@cast prism LANDED at `shards/cast.mirror` (SHA <post-Phase-2>)".

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** Phase 1 sharpenings do NOT pre-anticipate Phase 2 mint (correctly; the 8 sharpenings scope elsewhere). Phase 4 or a Phase-4a-mid-tick MUST re-upsert FLOOR.md §16.2 to reflect post-mint LANDED status. Reed's proposal does not name this re-upsert.

**Sharpening candidate (SHARPENING-3)**: add a Phase 2b tick (or fold into Phase 4 pre-authoring) to re-upsert FLOOR.md §16.2 marking @facet + @cast LANDED with post-Phase-2 SHAs. Without this, FLOOR.md remains stale-truthful (says VIRGIN while shards contain the substrate).

### §3.3 FLOOR.md §16.4 @cascade supersession preservation post-rename (Dependency 6)

**The dependency**: FLOOR.md §16.4 currently references @cascade supersession pending. Post-rename, references to @code become references to @facet. Does FLOOR.md need re-authoring at this altitude, or does substrate-rename cascade preserve semantic meaning?

**Method**: read FLOOR.md §16.4 verbatim; assess post-rename semantic-preservation.

**Findings**:
- FLOOR.md §16.4 states: "**@cascade** is architecturally superseded by @facet + @cast composition (mesh not waterfall, per §10). NOT yet reflected in the 30+ shards + 12 species that still reference @cascade. Cross-substrate rewrite pending."
- Post-@code-→-@facet rename, the FLOOR.md §16.4 text CONTAINS "@facet" already (naming the supersession). No text-change required.
- BUT: FLOOR.md §10 (`@cast` peer-translation mesh) + §9 (`@facet` generation surface) reference `@code/rust` species in `shards/beam/system.mirror` composition citations. Post-rename, those paths become `shards/beam/system.mirror` (unchanged; @beam family-root not touched by @code → @facet rename) BUT the @code/rust references in FLOOR.md §8 table become @facet/rust. Two possibilities: (a) FLOOR.md §8 table row "`rust/src/main.rs` | 71.5KB | 16 | Supervisor boot" describes the file NAME which is unchanged; the @code/rust text-references inside FLOOR.md prose are what needs post-rename update.
- FLOOR.md `@code\b` grep: FLOOR.md contains 41 @code occurrences per prior Search grep. Every occurrence needs post-rename evaluation.

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** FLOOR.md §16.4 semantic-meaning is preserved post-rename BUT FLOOR.md-total (41 @code hits) requires substrate-rename cascade to touch it. Reed's Phase 3 enumeration names FLOOR.md at 41 hits — verified. Post-rename FLOOR.md remains substrate-truthful IF rename semantic-preserves; substrate-honest IF Reed does not accidentally introduce meaning-change at high-hit sites (particularly §8 table + §13 recognition-arc table).

**Sharpening candidate (SHARPENING-4)**: for FLOOR.md rename-batch specifically, spot-check §8 table + §13 recognition-arc table + §14 PAPER_2D §-mapping table for semantic-preservation (all three tables are meaning-dense; blind s/@code/@facet/g may introduce reading-confusion).

### §3.4 @code/rust/materialize + @tool/cargo body-scaffold status (Dependency 7)

**The dependency**: @code → @facet rename doesn't fix body-empty scaffold. @facet/rust forward-projection body needs authoring regardless. Does Phase 3 rename create false substrate-optimism (looks like new substrate; actually still body-empty)?

**Method**: grep-verify @code/rust/materialize body-status; cross-reference against Rec #91 amendment §8.7.5 scaffold-vs-implementation delta.

**Findings**:
- `shards/code/rust/materialize.mirror` (9.4KB, 2026-06-16): "declares `classify` / `is_materialisable` / `target_altitude` action signatures; ALL bodies `\`; REVERSE direction (species→substrate-recognition); NOT forward-projection" per Rec #91 amendment §8.7.5.
- `shards/code/mirror.mirror` §render (LANDED 2026-06-08 T17): "7 combinators declared (`text` / `line` / `nest` / `beside` / `beside_space` / `above` / `group` / `flatten`); ALL bodies `\`" per amendment §8.7.5.
- `@tool/cargo` invocation-wrapper species (`shards/io/cargo.mirror`; 8 @code hits per prior grep + amendment says "172 LOC, 2026-07-18, fully-functional cargo invocation dispatch") — this one HAS body per amendment.
- **What is UNAUTHORED**: Forward-projection body for @facet/rust (𝓜 → Rust source); Rust source-language carrier in substrate; `mirror facet <target>` CLI verb at `rust/src/main.rs`.

**Verdict**: **SEAM-RATIFY.** Reed's Phase 3 rename does NOT purport to fix the body-scaffold gap — Phase 4 explicitly authors @facet/rust forward-projection body. There is NO substrate-optimism-inflation risk IF Phase 4 execution honors the scaffold-vs-implementation delta per Rec #91 amendment §8.7.5. Reed's proposal describes Phase 4 as "forward-projection body authoring per Erlang precedent shape (target 500-1000 LOC iteratively)" — this correctly names the authoring gap.

**Substrate-honest note**: post-rename, `shards/code/rust/materialize.mirror` becomes `shards/facet/rust/materialize.mirror` (or is left as-is IF materialize discharges REVERSE direction and @facet is FORWARD direction — the two directions may WANT to live at different family-roots). This is a Mara-canonical-spec surface not clearly discharged in the amendment.

---

## §4 — Substrate-broken-time-window adjudication

### §4.1 Coherence between Phase 3 rename batches (Window 8)

**The window**: if Reed commits per-file rename batches, is the substrate-record between commits consistent (e.g., `shards/code/mirror.mirror` is renamed but `shards/code/rust.mirror` is not; do the two co-exist without breaking compose-checks)?

**Method**: assess substrate-coherence properties across per-file rename commits.

**Findings**:
- Substrate composes via `in @code` chains: 37 shards declare `in @code` or contain `@code/*` species references per grep. If per-file rename batches commit `shards/code/rust.mirror` → `shards/facet/rust.mirror` FIRST while `shards/code/mirror.mirror` still contains `in @code`, the compose-graph has: `@code` family-root at `shards/code.mirror` (still present as `@code`), `@facet/rust` at renamed path (still declaring `in @code` per un-renamed body OR `in @facet` per newly-renamed body).
- Substrate parser DOES not currently strictly enforce `in @X` referent existence at every commit (per Seam's understanding of `rust/src/compile.rs` bootstrap-strictness) — but Rec #91 amendment §8.7.5 explicitly notes the substrate-honest boundary of what's landed.
- Per-file rename batches create intermediate substrate-records where `@code/rust` referenced-from-elsewhere no longer resolves.
- BETTER shape: batch by dependency-tier. Family-root rename `shards/code.mirror` → `shards/facet.mirror` FIRST (breaks EVERYTHING referring to @code); species rename `shards/code/rust.mirror` → `shards/facet/rust.mirror` etc. SECOND in the same commit; body-reference propagation `in @code` → `in @facet` inside every affected shard body THIRD in the same commit.
- Reed's "per-file or per-directory" batching creates broken-time-window at every intermediate commit. This is substrate-dishonest in the strict-parser future where `mirror compile --strict` catches referent-non-existence.

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** Per-file rename batches are substrate-broken across commits. The substrate-honest shape is atomic-per-dependency-tier commits: (i) mint @facet family-root + rename @code family-root simultaneously (single commit); (ii) rename all @code/* species simultaneously (single commit); (iii) update all `in @code` references + all @code/* usage sites simultaneously (single commit or per-batch by usage-cluster).

**Sharpening candidate (SHARPENING-5)**: pivot Phase 3 batching from "per-file or per-directory" to **atomic-per-dependency-tier**: (a) family-root rename + top-level VERB-references (single commit), (b) all species renames (single commit), (c) all `in @code` + body-references + doc-references (single commit or per-major-consumer). This preserves substrate-coherence at every intermediate git-tree state.

### §4.2 rust/src/*.rs @code references + cargo rebuild coordination (Window 9)

**The window**: `rust/src/main.rs` has 11 hits for `altitude @code/rust`. Phase 3 rename touches `rust/src/`. Does this require cargo rebuild between rename batches, or can Reed batch rust-side + shard-side together per commit?

**Method**: grep-verify `rust/src/main.rs` @code references; assess coupling with `mirror.spec` (which is the byte-check target).

**Findings**:
- `rust/src/main.rs` line 1474 (per prior Search): `spec_source.contains("altitude @code/rust") && (spec_source.contains("cargo"))`. This is a **string-literal byte-check** on `mirror.spec` source.
- `mirror.spec` (per Read of full file): contains 5+ `altitude @code/rust` occurrences (variety.emits.binary + coherence.audits fmt/lint/tests/bench + release-block reference).
- **The coupling is atomic**: if `rust/src/main.rs`'s byte-check string is renamed `"altitude @facet/rust"` and `mirror.spec` is renamed to contain `altitude @facet/rust`, BUT the built binary at `target/debug/mirror` still contains the OLD byte-check string `"altitude @code/rust"`, then `mirror craft` fails to recognize the renamed spec.
- This means: **rust-side rename requires cargo rebuild BEFORE the renamed spec is craft-testable**. But Phase 3 does NOT necessarily fire cargo build at all (rename is textual). The binary at HEAD is built pre-rename; post-rename, ANY invocation of `mirror craft ./mirror.spec` with the renamed spec fails until cargo rebuilds.
- REFUTE risk: if Reed's autonomous /loop cadence includes any `mirror craft` invocation during Phase 3, the invocation fails.

**Verdict**: **REFUTE-WITH-RESHAPE-CANDIDATE.** Reed's Phase 3 rename creates a substrate-broken-time-window between rust-side rename commit and cargo rebuild. Reshape required: (a) atomic-per-dependency-tier commit including rust/src/main.rs byte-check string + mirror.spec + cargo rebuild in single commit-batch (this is substrate-honest but expensive per commit); OR (b) hold rust/src/main.rs byte-check string until Phase 4 T-91-16 where `mirror facet rust` verb ALSO adds a new byte-check for @facet/rust (composed with backward-compat for @code/rust during transition); OR (c) Phase 3 executes with `cargo build` invocation per rust-touching-commit, verifying binary compiles after each rust-side rename batch.

**Recommended shape**: (b) — dual-byte-check during transition (mirror.spec accepts BOTH `altitude @code/rust` AND `altitude @facet/rust`) until T-91-16 close, then retire @code/rust byte-check with backward-compat drop in a subsequent tick. This preserves substrate-testability throughout the cascade.

**Sharpening candidate (SHARPENING-6)**: add dual-byte-check backward-compat window to Phase 3 rust-side rename: `rust/src/main.rs`'s `has_rust_target` check becomes `spec_source.contains("altitude @code/rust") || spec_source.contains("altitude @facet/rust")` (both admissible) until Phase 4 T-91-16 close. Retire `@code/rust` byte-check as a subsequent hygiene tick post-T-91-16.

---

## §5 — Mara-canonical-spec-first question

### §5.1 The question

**@code → @facet is a substrate-ontology change**. Alex 2026-08-21 verbatim: *"What reason would there be AGAINST a rename?"* — implies Alex-ratified go. Reed treats this as discharge of the Mara-canonical-spec adjudication.

**Adversarial-check**: does the rename introduce any ontological subtlety Mara should adjudicate first? Or is it clean mechanical rename?

### §5.2 Grep-verification of landed Mara position

**Method**: grep-verify Rec #91 amendment `674ffc6` §8.4 [ALEX-Q1] discharge + §13 [ALEX-Q7] Mara-lean.

**Findings** (grep-verified from Rec #91 amendment):

- **§8.4 [ALEX-Q1] DISCHARGED**: "via Alex 2026-08-20 session-ratification of `@cascade` supersession by `@facet` + `@cast` composition (FLOOR.md §16.4)". This discharge is about **@cascade** supersession, NOT about @code → @facet rename.
- **§13 [ALEX-Q7] (AMENDMENT 2026-08-20)**: "`@code` → `@facet` reshape scope — rename cascade (~150 files touched per Taut) OR additive sibling family-roots at distinct altitudes?" — **UNRESOLVED [ALEX-Q]** in landed amendment.
- **Mara-lean per [ALEX-Q7]** (verbatim from amendment): "ADDITIVE sibling family-roots at distinct altitudes. `@facet` = generation-surface at Rec #91 altitude (projection from 𝓜 onto runtime substrates); `@code/*` = grammar-discipline-at-altitude family-root at Rec #57 altitude (already-landed). Sibling not rename. ~150-file blast radius avoided. Per Alex 2026-08-05 rust-delivers-primitives HARD RULE composed with Rec #91 [ALEX-Q1] Mara-lean: distinct altitudes admit distinct family-roots. Only if Alex explicit-ratifies rename over sibling-additive does the rename cascade fire."
- **§15a.3** (What the amendment does NOT land): "**`@code` → `@facet` rename**: per [ALEX-Q7] Mara-lean, ADDITIVE sibling-family-roots preferred over rename; ~150-file blast radius avoided."

### §5.3 The adjudication surface

Mara's canonical position (Mara-canonical spec `674ffc6` landed 2026-08-20; grep-verified): **[ALEX-Q7] Mara-lean is ADDITIVE sibling family-roots preferred over rename**. Explicit substantive spec-position, not a placeholder.

Alex's 2026-08-21 dispatch to Reed via Seam brief: *"What reason would there be AGAINST a rename?"* is a **rhetorical question**, not an adjudication-discharge of [ALEX-Q7]. Alex asking Reed the counterfactual is Alex canvassing arguments — this is NOT the same as Alex explicit-ratifying rename over Mara-lean.

The load-bearing answer TO Alex's rhetorical question IS ALREADY IN MARA'S SPEC: (a) ~150-file blast radius; (b) 2-3x @cascade rewrite cost; (c) distinct altitudes (grammar-discipline-at-altitude vs generation-surface-projection) admit distinct family-roots per Rec #91 [ALEX-Q1] Mara-lean; (d) rust-delivers-primitives HARD RULE composed with distinct-altitudes discipline naturally-generates sibling not rename.

### §5.4 Verdict

**REFUTE.** Reed's Phase 3 executes rename without discharging [ALEX-Q7] Mara-lean. The Mara-canonical-spec adjudication is NOT clean-mechanical-rename admissibility — it's a substantive ontological-choice adjudication that Mara has already formalised at spec-altitude with a Mara-lean of ADDITIVE sibling family-roots.

**Two substrate-honest paths forward** (Reed must choose ONE before executing Phase 3):

- **Path A (Alex explicit-ratifies rename over Mara-lean)**: Alex adjudicates [ALEX-Q7] in Reed's favor with substantive reason (e.g., "generation-surface subsumes grammar-discipline; @code was always trying to become @facet"). Rec #91 amendment gets a §15a re-amendment marking [ALEX-Q7] as Alex-discharged in favor of rename. Reed then executes Phase 3 with substrate-honest citation of Alex-ratification. Blast radius accepted; substrate-honest choice.
- **Path B (Reed pivots to Mara-lean additive-siblings)**: Reed drops Phase 3 rename cascade. `@facet` family-root minted in Phase 2 lives as sibling to `@code`. Phase 4 T-91-16 authors `@facet/rust` composition-shard body as sibling (not replacement) to `@code/rust`. Blast radius avoided; Mara-canonical-lean preserved. Delta: `@code/*` retains its Rec #57 altitude semantics; `@facet/*` gets fresh Rec #91 altitude semantics; the two co-exist per Mara-lean [ALEX-Q7].

**Both paths are substrate-honest**. The current cascade proposal collapses both into rename WITHOUT acknowledging that Mara-canonical spec has a Mara-lean AGAINST rename. This is the load-bearing REFUTE.

**Alex adjudication required before Phase 3 executes**. This is the ONE genuine [ALEX-Q] blocker in the cascade.

---

## §6 — T-91-16 first-tick scope adjudication

### §6.1 500-1000 LOC scope substrate-honesty (Scope Risk 11)

**Reed's proposal**: "500-1000 LOC iteratively" for @facet/rust forward-projection body per Erlang precedent shape.

**Adversarial-check**: is this scope substrate-honest given Erlang precedent 555 LOC for narrower substrate? Rust grammar is 2-3x. Does Reed's iterative scope match the theorem 5a.1 empirical falsifier (mirror reproduces its own source diff == 0), or does the first-tick need to be narrower to genuinely fire empirical closure?

**Method**: cross-reference Rec #91 amendment §8.7 Theorem 8.7 + §8.7.2 empirical falsifier + §11.1 T-91-16 sub-halts.

**Findings**:
- Amendment §8.7.2 falsifier: `mirror facet rust > /tmp/rust-floor && diff -r rust/src /tmp/rust-floor` returns exit-0 with empty diff.
- T-91-16 has three sub-halts (16a body + 16b CLI + 16c empirical fire).
- Amendment §8.7.5 scaffold-vs-implementation delta explicitly names three UNAUTHORED gaps: forward-projection body, `code/rust.ast` type binding, `mirror facet <target>` CLI verb.
- Erlang precedent for narrower substrate (555 LOC): reasonable comparison anchor. Rust `syn` grammar surface + doc-combinator body IS larger than Erlang grammar surface. 2-3x scaling gets ~1100-1700 LOC.
- BUT: T-91-16 first-tick does NOT need to cover full rust-source surface. It needs to cover **enough of rust/src/ to fire diff-empty on rust/src/main.rs alone** (or one file). Iterative extension per file is the substrate-honest shape — first-fire on ONE file (e.g., `rust/src/wire.rs` at 5.8KB / 210 LOC — smallest, simplest), then extend.
- Reed's "500-1000 LOC iteratively" scope is under-scoped for full rust/src/ diff-empty BUT reasonable for iterative first-file-first strategy.

**Verdict**: **SEAM-RATIFY-WITH-SHARPENING.** Iterative scope is substrate-honest for the theorem discharge IF Reed executes first-file-first (not all-files-at-once). Full rust/src/ diff-empty per §8.7.2 requires ~5000-10000 LOC of forward-projection body — but iterative T-91-16 landings can genuinely fire empirical closure on subset (one file at a time) before extending.

**Sharpening candidate (SHARPENING-7)**: reshape T-91-16 first-tick scope explicitly as **first-file iterative discharge**: T-91-16c empirical fire scoped to `mirror facet rust rust/src/wire.rs > /tmp/wire.rs && diff rust/src/wire.rs /tmp/wire.rs` returns empty. Subsequent T-91-16-extension ticks discharge additional files. This preserves substrate-honesty of the empirical fire while acknowledging full-rust/src/ diff-empty is multi-tick territory.

### §6.2 `mirror facet rust` CLI verb genuine-authoring vs falls-out-from-math (Scope Risk 12)

**Reed's proposal**: `mirror facet rust` CLI verb "falls out from stable math floor" per Alex 2026-08-20 naming.

**Adversarial-check**: is this substrate-honest given `rust/src/main.rs` currently has 12 verbs and adding a 13th is genuine authoring? Or does the CLI verb fall out from the @facet family enumeration once landed?

**Method**: grep-verify `rust/src/main.rs` verb count; assess CLI-verb-authoring altitude.

**Findings**:
- Current verbs (per Rec #91 amendment §5.3 grep-observation): "compile / kintsugi / shatter / craft / init / recall / beam / peer beam / peer contribute / index / roomba / serve" = 12 verbs. Adding `facet` is verb #13.
- CLI-verb-authoring altitude: `rust/src/main.rs` is rust/-altitude (Layer 0 sub-Turing interpreter per FLOOR §8). Adding a verb requires: verb-match-arm at hardcoded verb-dispatch, help-text update, verb-body composing `apply_h::act` dispatch on `@facet/<target>` action-ref.
- "Falls out from stable math floor" IS substrate-honest at the **dispatch-altitude** — the verb-body IS just `apply_h::act(root, "@facet/rust", args)` (per amendment §11.1 T-91-16b). This is a genuine but small authoring surface at rust/-altitude.
- BUT: "falls out from stable math floor" as Alex-quote (2026-08-20 dispatch-time) is about the MATH surface (source-emit fixed-point theorem). Reed's application to CLI-verb altitude is a **register-shift** — the math floor is stable; the CLI verb is a discrete authoring at Layer-0.

**Verdict**: **SEAM-RATIFY.** CLI verb IS genuine authoring at rust/-altitude (small, but non-trivial). Reed's "falls out from stable math floor" phrasing conflates math-altitude stability with authoring-altitude discrete-work; substrate-honest register would distinguish the two. Not a blocker; a minor sharpening.

---

## §7 — [ALEX-Q] residues

Genuine open questions requiring Alex Fourth-Chair adjudication (not Reed-adjudicable, not Seam-adjudicable via substrate-grep, not Mara-adjudicable via canonical-spec-already-landed):

### §7.1 [ALEX-Q1] Discharge [ALEX-Q7] rename-vs-additive-siblings

**Context**: Rec #91 amendment `674ffc6` §13 [ALEX-Q7] Mara-lean is ADDITIVE sibling family-roots. Alex 2026-08-21 rhetorical "*What reason would there be AGAINST a rename?*" does NOT discharge [ALEX-Q7] substantively.

**The Alex-adjudication question**: does Alex explicit-ratify **rename cascade over Mara-lean** (Path A per §5.4), OR does Alex ratify **additive sibling family-roots per Mara-lean** (Path B per §5.4)? Reed's cascade blocks on this adjudication.

**Substrate-honest reminder**: both paths are admissible. The question is which Alex chooses. Rec #91 amendment §15a re-amendment would formalize whichever adjudication lands.

### §7.2 [ALEX-Q2] materialize direction post-rename

**Context**: `shards/code/rust/materialize.mirror` charters the REVERSE direction (species→substrate-recognition; NOT forward-projection). `@facet/rust` charters FORWARD direction. Post-rename, do the two directions live at same family-root or split?

**The Alex-adjudication question**: if Path A (rename) chosen, does `shards/code/rust/materialize.mirror` become `shards/facet/rust/materialize.mirror` (both directions under @facet/rust)? OR does materialize STAY at @code (materialize is grammar-discipline REVERSE; @facet is generation-surface FORWARD; two directions naturally live at two family-roots)?

If Path B (additive-siblings) chosen, this question dissolves: materialize stays at @code/rust; @facet/rust hosts FORWARD-only projection.

### §7.3 [ALEX-Q3] rust-side backward-compat window shape

**Context**: `rust/src/main.rs` byte-check on `"altitude @code/rust"` couples to mirror.spec. Post-rename creates substrate-broken-time-window per §4.2.

**The Alex-adjudication question**: does Reed execute dual-byte-check (SHARPENING-6; accept both @code/rust and @facet/rust during transition), OR atomic-rename with cargo rebuild in single commit, OR does Path B (additive-siblings) obviate this by keeping @code/rust byte-check indefinitely and adding @facet/rust as sibling?

---

## §8 — Sharpenings

Specific edit-actions to Reed's cascade proposal (Reed applies IF cascade proceeds after §7 [ALEX-Q1] adjudication):

### §8.1 [SHARPENING-1] Split Phase 1 CURRENT.md operation into two commits

Per §2.2. Split: (i) commit uncommitted-CURRENT.md as-is preserving 2026-08-19 authorial-boundary, (ii) upsert with 2026-08-20 landings as separate commit. Preserves temporal-fidelity.

### §8.2 [SHARPENING-2] Enumerate CURRENT.md in Phase 3 rename-batch

Per §3.1. Add `docs/loop/CURRENT.md` (19 hits) to Phase 3 top-hitters enumeration alongside code-metalogue-surface.md (263), taut-code-turing-substrate-scout.md (116), etc.

### §8.3 [SHARPENING-3] Add Phase 2b tick re-upserting FLOOR.md §16.2 post-mint

Per §3.2. After Phase 2 mints @facet + @cast, re-upsert FLOOR.md §16.2 marking BOTH LANDED with post-Phase-2 SHAs. Otherwise FLOOR.md remains stale-truthful.

### §8.4 [SHARPENING-4] FLOOR.md rename-batch semantic-preservation spot-check

Per §3.3. Spot-check FLOOR.md §8 table + §13 recognition-arc table + §14 PAPER_2D §-mapping table for semantic-preservation. Blind s/@code/@facet/g may introduce reading-confusion at meaning-dense sites.

### §8.5 [SHARPENING-5] Pivot Phase 3 batching to atomic-per-dependency-tier

Per §4.1. Reshape from "per-file or per-directory" to atomic-per-tier: (a) family-root rename + top-level VERB-references, (b) all species renames, (c) all `in @code` references + body-references + doc-references. Preserves substrate-coherence at every intermediate git-tree state.

### §8.6 [SHARPENING-6] Add dual-byte-check backward-compat window

Per §4.2. `rust/src/main.rs`'s `has_rust_target` check becomes `spec_source.contains("altitude @code/rust") || spec_source.contains("altitude @facet/rust")` until Phase 4 T-91-16 close. Retire @code/rust byte-check as subsequent hygiene tick post-T-91-16.

### §8.7 [SHARPENING-7] Reshape T-91-16 first-tick scope to first-file iterative

Per §6.1. T-91-16c empirical fire scoped to `mirror facet rust rust/src/wire.rs > /tmp/wire.rs && diff rust/src/wire.rs /tmp/wire.rs` returns empty on ONE file (`rust/src/wire.rs` recommended: 5.8KB / 210 LOC = smallest simplest). Subsequent extension ticks discharge additional files.

---

## §9 — Recommendation

Reed's cascade is **NOT ready-to-fire as proposed**. Reshape required before /loop begins.

### §9.1 Phase-by-phase readiness verdict

| Phase | Verdict | Sharpenings required | Blockers |
|-------|---------|---------------------|----------|
| Phase 1 (HYGIENE) | READY with SHARPENING-1 | Split CURRENT.md into two commits | None |
| Phase 2 (SUBSTRATE MINT) | READY | None | None |
| Phase 3 (@code → @facet RENAME) | **BLOCKED** on [ALEX-Q1] | SHARPENING-2, -3, -4, -5, -6 required IF proceeds | Alex must adjudicate §5 REFUTE (rename vs additive-siblings) |
| Phase 4 (OUROBOROS CLOSE) | READY with SHARPENING-7 | Reshape T-91-16c to first-file iterative | Depends on Phase 3 outcome (@code/rust vs @facet/rust naming target) |

### §9.2 Substrate-honest cascade sequence

**Recommended reshape before /loop begins**:

1. **Ship Phase 1 as-designed** with SHARPENING-1 (two-commit CURRENT.md).
2. **Ship Phase 2 as-designed** (mint @facet + @cast VIRGIN → LANDED).
3. **PAUSE cascade at Phase 3**. Escalate [ALEX-Q1] (§7.1) to Alex Fourth-Chair. Await adjudication: rename (Path A) OR additive-siblings (Path B).
4. **IF Path A**: apply SHARPENING-2, -3, -4, -5, -6; execute Phase 3 rename cascade per reshaped batching. Then Phase 4 with @facet/rust naming target.
5. **IF Path B**: skip Phase 3 rename entirely. Phase 4 executes with @facet/rust as sibling to @code/rust; Rec #91 amendment §15a-b marking [ALEX-Q7] discharged in favor of Mara-lean.
6. **Ship Phase 4 with SHARPENING-7** (first-file iterative T-91-16c per `rust/src/wire.rs` empirical fire).

### §9.3 Substrate-honesty preserved

- **§5 REFUTE preserved**: rename vs additive-siblings is a Mara-canonical-spec-first question that Alex's rhetorical *"What reason would there be AGAINST a rename?"* has NOT discharged. Reed executing autonomously without Alex explicit-ratification writes over Mara-lean.
- **Adversarial-distance preserved**: Seam adopts different verdict-vocabulary (SEAM-RATIFY / SEAM-RATIFY-WITH-SHARPENING / REFUTE) than what the cascade itself uses; Karl-Tomm-style questions absent at verdict-altitude per Reed-substrate-decision 2026-08-20.
- **Substrate-truth held**: every dispatch claim grep-verified. @facet + @cast confirmed VIRGIN 2026-08-21. Rec #91 amendment §13 [ALEX-Q7] confirmed UNRESOLVED. `rust/src/main.rs` byte-check coupling to `mirror.spec` confirmed atomic. FLOOR.md 41 @code hits confirmed. `docs/loop/CURRENT.md` 19 @code hits confirmed.
- **Grep-first discipline preserved**: audit does not invent or infer; all findings anchored in file-content grep results with citable line-ranges and SHAs.

### §9.4 Verdict summary for Reed

- ✓ Overall cascade verdict: **SEAM-RATIFY-WITH-SHARPENING with 1 load-bearing REFUTE on Phase 3**
- ✓ Sequencing adjudication (3 items): 2 SEAM-RATIFY + 1 SEAM-RATIFY-WITH-SHARPENING
- ✓ Hidden dependencies (4 items): 2 SEAM-RATIFY + 2 SEAM-RATIFY-WITH-SHARPENING
- ⚠ Substrate-broken-time-windows (2 items): 1 SEAM-RATIFY-WITH-SHARPENING + 1 **REFUTE-WITH-RESHAPE** (rust-side byte-check coupling)
- ⚠ Mara-canonical-spec-first (1 item): **REFUTE** — [ALEX-Q7] Mara-lean is ADDITIVE-SIBLINGS; Alex rhetorical does NOT discharge substantively
- ✓ T-91-16 first-tick scope (2 items): 1 SEAM-RATIFY-WITH-SHARPENING + 1 SEAM-RATIFY
- ⚠ [ALEX-Q] residues: 3
- ⚠ Sharpening candidates: 7

**Ship-blocked on §5 REFUTE + §7.1 [ALEX-Q1]**. Path A or Path B — Alex chooses. Then cascade fires with sharpenings applied.

Adversarial-distance preserved. Substrate-truth held.

🔍

---

*Seam — 2026-08-21 — Phase D adjudication of Reed's 4-phase cascade proposal. Signed off substrate-honest.*
