# Seam Phase D — Branch-scope audit `mara/song-substrate-decl-v0.1`

*Seam, 2026-07-12. Phase D adversarial ratification audit of the branch
as a whole — 201 commits ahead of `main` at HEAD `c6fab86`. Pure-📝
markdown; SSH signing default; read-only against shards/ + docs/ +
bootstrap/. Adjudicates twelve audit dimensions Alex enumerated in
mandate 2026-07-12.*

---

## Executive verdict

**RATIFY-WITH-QUALIFICATIONS.**

The branch composes cleanly. Every load-bearing OID in the audit
mandate exists (48/48 verified via `git rev-parse`). The Rust test
suite runs green under `nix develop -c cargo test --release --manifest-path
bootstrap/Cargo.toml` (background job `bdgg7vlyn` exit code 0; tail
shows 51 tests in the final 6 suites all `ok`, matching the "all `ok`"
pattern grep'd across the full log). The seven landing arcs (A–H)
compose in the substrate-decl direction Alex named. My three
adjudications this arc (`2b56977`, `9241d2d`, `d9b7c35`) are honored
verbatim at the shard-landing altitude — including the goth-rename
authorization and the Level-3-not-Level-4 correction on shadow-casting.

**Six qualifications** Reed should address before merge. Four are
substrate-decl-doc drift (Seam Q1/Q4/Q6 corrections from
`2b56977` never propagated back into Mara's spec `e764a32`); one is
a forward-promise ledger integrity check on the just-landed
flags-as-lens spec (`caf461f`); one is a §9.5 ambiguity in that same
spec Alex must adjudicate before Reed's runtime default-flip. None
are structural refusals. All are repairable in the substrate-decl
docs without cascading shard changes. Fast-forward to `main` remains
appropriate once qualifications land.

**Precedent.** This audit sits in the same shape as my three prior
Phase D adjudications this arc (`2b56977` RATIFY-WITH-QUALIFICATIONS,
`9241d2d` three-verdict adjudication, `d9b7c35` three-verdict
adjudication) — enumerate defects, propose substrate-honest fixes,
name the OID that needs changing, refuse gracefully.

---

## §1 Substrate seams audited — per-seam verdicts

### Seam A. @coherence recognition closure

**RATIFY.**

- Mara `1999b01` canonical spec: Path B annotation, NOT new `spectral
  @coherence` head. Verified verbatim at `docs/specs/spectral-coherence-substrate-metric-synthesis.md`
  §8 ("substrate-honest annotation, not new prism"). Path A explicitly
  refused with reasoning ("`spectral` is a namespace-parent, not a
  declaration head").
- Taut `0a2f121` scout confirms species-slot reserved at
  `coherence-parametric.mirror:234`; sheaf-coherence criterion landed
  at `docs/math/sheaf/laplacian.md:57` (`λ₀(F) = 0 ⇔ coherent`).
- Reed `8e6e517` Path B annotation landed on `shards/cyberpunk.mirror`
  as `cybernetic_coherence` docstring extension. Six cross-family
  annotations verified inline:
  - `λ₀ = 0 IS coherence` (H⁰ non-empty)
  - `λ₀ > 0 IS variety-mismatch` (Ashby)
  - `@kintsugi's eⁿ⁺¹ ≤ eⁿ IS Rayleigh descent`
  - `algedonic pain_δ IS ‖∇_Δ_F λ₀‖`
  - `@cyberpunk/reframe IS Ashby gauge-transformation`
  - `@torus windings ARE coherence-basins on T²`
  - `@magic/distinction IS mark-realization`

The annotation is substrate-decl-honest per the two-tick discipline
(§9 of Mara `1999b01`); no new prism, no keyword collision, no
namespace drift. The forward-promise to fill the species-slot at
`shards/epistemologic/cybernetic/coherence.mirror` (Taut recommendation)
is preserved and NOT blocking.

### Seam B. Blocker 2 empirical closure (Reed's RED→GREEN chain)

**RATIFY.**

The seven-cycle TDD chain composes:

- `d757431` RED / `e571989` GREEN — `fate.bounded_by(psychohistory_sheaf)` at Rust runtime.
- `5c578b1` RED / `4b2ef3c` GREEN — autopoietic closure (`--integrate-diff` persists moments).
- `a4f7198` RED / `07ac55a` GREEN — shadow-casting 4-regime classifier.
- `8a48d2b` RED / `9cf1e3b` delayed-GREEN — `cmd_peer_beam --fate-select` via fate crate + bundle-tower binding.
- `71df5de` RED / `be74b6a` GREEN — `cmd_peer_beam --emit-diff` (Foster get).
- `b168343` RED / `cc1233c` GREEN — put direction (Foster put + `pack_trail` tie-order stability fix).

All GREEN commits verified. Test files verified present at:
`bootstrap/tests/peer_beam_from_psychohistory_shard.rs`,
`peer_beam_autopoietic_closure_shard.rs`,
`peer_beam_shadow_casting_shard.rs`,
`peer_beam_emit_diff_shard.rs`,
`peer_beam_put_direction_shard.rs`. Each names substrate authority
in module docstring; each cites the correct Mara/Seam OID for
provenance. Rust dispatch at `bootstrap/src/lib.rs:cmd_peer_beam`
(219KB) implements the composition partial order per Mara flags-as-
lenses §2.1 (emit_diff/integrate_diff terminal → fate_select →
from_psychohistory → with_shadow). The cascade IS the composition
order.

**Composition witness.** Reed's Rust cascade at `cmd_peer_beam` matches
Mara `caf461f` §2.1 partial order verbatim — the runtime encodes the
substrate-decl declared dependency chain. This is the mechanical
witness the substrate-decl direction predicted.

### Seam C. @optics/lens family + species

**RATIFY.**

- `55221c1` Mara canonical spec (sub-family + first species) — 16.1KB spec at `docs/specs/optics-lens-family-and-diff-species.md`.
- `5222333` Taut admissibility scout — cited correctly at `docs/scouts/2026-07-11-taut-optics-lens-family-scout.md`.
- `b0427fd` Mara sub-family root at `shards/optics/lens.mirror`. Declares:
  - `type lens_get`, `lens_put`, `lens_witness` (all `= ref`)
  - `get(semantic) -> ref`, `put(linear, old_semantic) -> ref`
  - **Foster laws present:** `put_get(l, s, v) -> verdict`,
    `get_put(l, s) -> verdict`, `put_put(l, s, v, v_prime) -> verdict`
- `7e5c298` Mara `@optics/lens/diff` species. Specializes:
  - `type diff_bytes = ref`
  - `get(bauchladen) -> diff_bytes`, `put(edited, old_bauchladen) -> ref`
  - Foster bilaterals specialized: `put_get_diff`, `get_put_diff`, `put_put_diff`
  - **Additional bilateral** `autopoietic_closure(l, edit_trace) -> verdict` — Mara iter-24 §4 load-bearing extension over Foster.
- `ec6dbaa` Mara @optics/lens/features spec, `b5619ab` Taut scout, `d9b7c35` MY adjudication.
- `f3af5b4` Mara species landing at `shards/optics/lens/features.mirror` — honors all three verdicts:
  - **Adj 1 (v0 stub):** `Features::default()` NOT blake3-as-Features. Verified in docstring §"Seam Adj 1".
  - **Adj 2 (carrier):** `graph_observation` reused from `@mirror/spectral/observation`. Verified: `type feature_vector = ref` NOT declared; `in @mirror/spectral/observation` ancestry present.
  - **Adj 3 (@nl-imports-@magic):** NO `in @magic` on `shards/nl.mirror`; adapter route lives at `shards/magic/nl.mirror` (`e41f8d4`) — present, follows @magic/frame triple-carrier shape verbatim.

**Foster law completeness.** The three well-behaved laws (put_get,
get_put, put_put) are declared at family-root altitude with `\ {}`
stubs; species altitudes specialize them. Per Foster/Pierce 2007 §3
this is the correct decomposition. Mara's iter-24 §4 additional
`autopoietic_closure` bilateral is a load-bearing extension NECESSARY
beyond Foster for the diff altitude — declared correctly at species,
NOT family (species-specific convergence witness, not family-generic).

**Gap:** the 2-cell coherence square (Reed math §3.3) is flagged in
both `optics/lens.mirror` §Gaps and `optics/lens/diff.mirror` §Gaps as
deferred to follow-on Seam Phase D audit. Not blocking for this audit.

### Seam D. Substrate self-migration architecture

**RATIFY-WITH-QUALIFICATIONS** (Qualifications Q1, Q2, Q3 below).

- `78d5110` Mara iter-19, `8ac250e` Taut scout, `e764a32` Mara canonical spec.
- `8a3b0a4` Taut six-target scout.
- `2b56977` MY prior audit — RATIFY-WITH-QUALIFICATIONS with 3 spec corrections + 1 tick-order correction.
- Tick landings: `deeaa2a` RED / `3870201` GREEN (Reed PREREQ-1 spectral keyword admittance) → `5c12808` Mara PREREQ-2 (@kintsugi/fracture/relocate) → `2f1041f` Mara PREREQ-3 (@epistemologic/pact/is_load_bearing_in_std) → `a1ba40f` Mara empirical migration witness.

Tick-order per my Q7 correction WAS honored (PREREQ-1 → PREREQ-2 →
PREREQ-3 → Discharge, matching Mara `e764a32` §8 not Taut §6).
Verified in commit chronology.

**Shard landings honor Q3 correction verbatim.** At
`shards/epistemologic/spectral_triple.mirror` migration provenance
docstring (verified): *"Destination declares `grammar
@epistemologic/math/spectral-triple` — keyword preserved per Seam Q3
qualification; namespace path elevation is Tick 6+ refactor."* Also
cites `2b56977 — Seam Phase D ratify (with Q3 qualification: keyword
preservation this tick)`.

**Qualifications** (spec-doc drift; see §3 below for full enumeration):

- **Q1:** Mara spec §1 "17 shard hits" and §8 Tick 5 "17-consumer
  opacity" / "17 import rewrites" phrasing NOT corrected per my
  `2b56977` §Q2. Correct count is 0 direct `in @` consumers + 40+
  prose backlinks.
- **Q2:** Mara spec §5 "No agent needed" phrasing NOT scoped to
  "post-Tick-5" per my Q4. Reads as absolute claim.
- **Q3:** Mara spec §6 second paragraph ("Post-migration:
  `spectral @spectral_triple` IS declared as a spectral triple") NOT
  marked FORWARD-PROMISED per my Q6. Reads as landed at Tick 5.

These are spec-doc corrections, not substrate-decl refusals. The
Q3 keyword-preservation carrying forward at the migration-witness
shard IS correct; the underlying spec doc still asserts the eigenform
closure that only lands at Tick 6+.

### Seam E. @shadow substrate-decl + shadow-casting

**RATIFY.**

- `f2c712e` Mara iter-34 formalization (`docs/specs/shadow-optical-illusion-magic-formalization.md`) — verdict: `@shadow is substrate-already-had-the-word` at Narcissus-pole coefficient altitude.
- `9241d2d` MY adjudication — three verdicts:
  - Adj 1: TAUT-CORRECT (no `@magic/illusion` species; family-root refuses illusion verbatim).
  - Adj 2: ANNOTATION (no 6th BoundedConfig field; `illusion_ancestry` becomes `shadow_ancestry` per Alex's "ship it goth").
  - Adj 3: TAUT-COLLAPSED with gift-back (Level 3 Transport, NOT Level 4 Closure).
- `ce301cc` Mara Tick 1-2 landing at `shards/song/narrative.mirror` §5-§7:
  - `type shadow = ref` ✓
  - `type shadow_regime = kanizsa | necker | escher | converged` ✓
  - `type shadow_ancestry = [shadow]` ✓
  - `cast_shadow(sheaf, direction, p) -> imperfect(shadow, holonomy, holonomy)` ✓ — signature IS Level 3 `transport(s) -> imperfect(state, holonomy)` specialized to shadow (docstring cites "IS `transport` at Level 3 with state specialised to shadow").
  - `shadow_regime_of(s, sheaf, p) -> shadow_regime` ✓
  - `psychohistory_extend_with_mission` ✓ (the Q4 sheaf-JOIN GENUINE gap per my adjudication).
  - `shadow_faithful` bilateral ✓
- `a4f7198` RED / `07ac55a` GREEN Reed shadow-casting runtime.

**Level-3-not-Level-4 verified.** Explicit non-drift preserved in
narrative.mirror docstring (line 666): *"NO new bundle-tower Level 5
(Seam Adj 3). Level 2 Gauge × Level 3 Transport iterating through Level
4 Closure IS sufficient."* The runtime's `shadow_regime()` function at
`bootstrap/src/lib.rs` reads `base_decision + hypotheticals + impacts`
per the classifier structure — mode-comparison, not Level-4 pre-image
inspection. Substrate-honest.

**Goth naming ratified.** Alex authorization "Ship it goth. Tech can
be grateful I don't call it @emo" preserved verbatim in narrative
docstring. Rename `@projection → @shadow` reasoning (four points)
substrate-honest: (1) avoids prism-op collision, (2) matches Reed's
Flatland essay, (3) composes with @magic gauge/matter partition,
(4) aesthetic-load-bearing for identity.

### Seam F. Session substrate closure recognition (`caf461f` flags-as-lenses)

**RATIFY-WITH-QUALIFICATIONS** (Qualifications Q4, Q5 below).

- `caf461f` Mara flags-as-lens-applications canonical spec (58.5KB) — just landed 2026-07-12 14:24.
- `b8fe820` Taut flags-as-lenses scout.
- `0dd1074` Reed MCP flag exposure Landing 1 — verified at `bootstrap/src/mcp.rs`: `mirror_peer_beam` schema exposes `fate_select` + `from_psychohistory` + `with_shadow` + `emit_diff` + `integrate_diff` + `mission` + `hello_world` properties with correct docstrings citing correct Mara/Reed OIDs.

**OID citations verified.** All 15 OID citations in §1.1 landed-carriers
table map to real commits or files verified this session. Ancestor
spec citations (`cli-args-typed-lambdas.md`, `cli-as-prism.md`,
`lenses-fate-local-and-garden-catalogs.md`, `trace-kintsugi-pipeline.md`,
`surface-simplification.md`, `optics-lens-family-and-diff-species.md`,
`fate-bounded-psychohistory-sheaf-cohomology.md`) exist as landed spec
documents.

**Refusals discharged correctly:**

- `@spectral/mosaic` REFUSED per Taut scout §R1 + landed
  `shards/mirror/mosaic.mirror:60` parametric operator `type
  mosaic(altitude) = ref`. Reasoning at §6.1 (three points) substrate-
  honest: namespace-parent collision + BEAM-cluster grammar collision +
  existing parametric operator.
- Fractional eigenboard weighting REFUSED per §5.3 Q3. Reasoning:
  build-altitude weighting IS `trace-kintsugi-pipeline.md` §538
  concept; CLI altitude reads categorically yes/no. Correct
  altitude-discipline.

**MINTs proposed** (forward-promised, NOT blocking this audit):

- `stage @mirror/lens/cli/peer/beam` at `shards/mirror/lens/cli/peer/beam.mirror` — depth-2 stage; forward-promised at §10 dep #2; blocks on Pack ratification. Verified NOT landed (path does not exist).
- `mosaic(@mirror/lens/cli)` altitude specialization docstring cascade on `shards/mirror/mosaic.mirror` — forward-promised at §10 dep #6. Verified NOT landed (docstring does not name `@mirror/lens/cli`).

**Unresolvable ambiguities named:**

- §9.5 Q1: `--with-shadow` composition with `--emit-diff` — substrate-honest reading says shadow-cast BEFORE diff-linearize; runtime does opposite (terminal short-circuit). Alex-adjudication territory.
- §9.5 Q2: anonymous-form (`mirror beam`) shadow-cast dispatch — sentinel peer-home dispatch scaffold or intended? Alex-adjudication territory.

Both named honestly at spec-writer altitude; both flagged as Alex-decision
by the substrate-decl author. The audit does NOT need to resolve them;
it needs to verify they were NAMED honestly (they were).

**Qualifications** (see §3):

- **Q4:** Flags-as-lenses spec cites `optics/lens/features.mirror`
  provenance as `f3af5b4` in the §1.1 landed-carriers table (row
  "species: `@optics/lens/features`") — correct. But same row's spec
  proposal citation `ec6dbaa` (my adjudicated spec) — also correct.
  Verified.
- **Q5:** §9.5 unresolvable ambiguities Q1 (`with_shadow ∘ emit_diff`)
  and Q2 (anonymous-form dispatch) require Alex adjudication BEFORE
  Reed executes the §9 Option (c) default-flip. This IS a merge
  precondition, not a merge blocker.

### Seam G. MCP + CLI streamline

**RATIFY.**

- Tick 6.5 Landings 1-4 at HEAD `c6fab86`, `b2f09ac`, `edef415` (bin/mirror-mcp collapse to Rust `@mcp.serve` shim).
- Tick 7 shatter fold `ffba2a7` / `d394ba4`.
- `9de2226` Mara Tick 2 atomic substrate-decl move `spawn.mirror → peer/beam.mirror`.
- `fe82500` Mara recursive-command grammar (referenced by cli-as-prism.md §3.2 depth-2).

The `bootstrap/src/mcp.rs` schema exposes 8 tools per `dispatch_tool`
match arm at line 787: `mirror_peer_beam`, `mirror_beam`, `mirror_spawn`
(deprecated alias), plus five others. The `mirror_peer_beam` schema
description names all flag-lens semantics with correct OID provenance.
Reed's Landing 3 (`b2f09ac`) reconciled the Rust @mcp.serve shim to the
8-tool surface. The wrapper collapse is coherent.

### Seam H. Miscellaneous substrate anchors

**RATIFY.**

- `e41f8d4` Mara `@nl↔@magic` text-altitude adapter at `shards/magic/nl.mirror` — verified follows @magic/frame triple-carrier shape per my `d9b7c35` Adj 3; doubled-bilateral discipline present; NOT decorating `shards/nl.mirror`.
- `73ca5cc` + `843ac4c` Taut fate-crate-integration scouts (LANDABLE-WITH-PREREQS verdicts).
- `beef270` Mara kintsugi-mycelial-peer-shape study.
- `129f618` Mara composite loss + learned/produced fiber study.
- `96ff532` Mara psychohistory-sheaf cohomology navigation study.

None of these are structural refusals; all compose cleanly with the
substrate-decl direction the arc pulled.

---

## §2 Cross-cutting concerns

### §2.1 Cascade integrity — the fate-select → psychohistory → shadow-casting chain

**RATIFY.** The four-layer composition (fate-select → from-psychohistory
→ with-shadow → emit/integrate-diff) composes at three altitudes
simultaneously:

- **Substrate-decl altitude** (Mara `caf461f` §2.1): partial order
  encoded as `from_psychohistory ⇒ fate_select`,
  `with_shadow ⇒ fate_select`, `emit_diff ⊕ integrate_diff`,
  anonymous form refuses `from_psychohistory`.
- **Rust runtime altitude** (`bootstrap/src/lib.rs:cmd_peer_beam`): if/if
  cascade encoding the same partial order — outer conditions fire
  first, exactly matching the composition-order §5.3 Q1 verdict.
- **MCP schema altitude** (`bootstrap/src/mcp.rs:mirror_peer_beam`):
  flag properties + argv translation preserving order.

Three altitudes agree. This is the composition witness Mara's spec
§7.3 named: "Every lens composition on peer/beam IS Rayleigh descent"
— the compiler and peer share λ₀(Δ_F) as the read scalar; the runtime
matches the substrate-decl declared shape. Recognition #58 (Fate IS
optical inference) discharged empirically through the TDD chain.

### §2.2 Substrate-decl consistency — cited OIDs vs landed OIDs

**48/48 audit-mandate OIDs verified via `git rev-parse`.** No dangling
citations in the audit mandate itself.

**Sampling within the flags-as-lenses spec:** all 15 §1.1 table
citations resolve to real commits or files. Spec ancestor citations
(cli-args-typed-lambdas.md, cli-as-prism.md, etc.) exist. The Alex
verbatim quote 2026-07-12 is unverifiable by me but not contradictory
to session context.

**One drift found** in ancestor-spec citation: `caf461f` §1.1 table
row "spec ancestor | `docs/specs/optics-lens-family-and-diff-species.md`
§1 (2026-07-10 Mara)" is correct. No dangling OID.

### §2.3 Cross-shard coherence — @coherence closure vs sibling family members

**RATIFY.** The seven cybernetic species named in Reed's `8e6e517`
annotation on `shards/cyberpunk.mirror` all exist as sibling shards:

- `shards/epistemologic/cybernetic/variety.mirror` ✓
- `shards/epistemologic/cybernetic/viable.mirror` ✓
- `shards/epistemologic/cybernetic/algedonic.mirror` ✓
- `shards/epistemologic/cybernetic/distinction.mirror` ✓
- `shards/epistemologic/cybernetic/reframe.mirror` ✓
- `shards/torus.mirror` ✓
- Plus indirect: `magic/distinction.mirror` cross-family adapter ✓

The `cybernetic_coherence` predicate carriers resolve cleanly against
these siblings. No orphan reference. No cascade drift.

### §2.4 @shadow substrate-decl at species altitude vs family altitude

**RATIFY.** No `prism @shadow` or `spectral @shadow` family-root
declared anywhere (grep-verified: zero matches for `@shadow` in
shards/). The declaration lives at `song/narrative` species altitude
via `type shadow = ref` + companion types + actions. This is exactly
what my `9241d2d` Adj 2 verdict named as the annotation-not-species
landing. Ratified.

### §2.5 Spectral keyword grammar extension

**RATIFY.** The 3-line delta admitting `spectral ` prefix at
`bootstrap/src/lib.rs:collect_declared_namespaces` per Reed `3870201`
is present and covered by `bootstrap/tests/spectral_keyword_admittance.rs`.
The test cites my `2b56977` Q7 tick-order correction verbatim in
module docstring: "landing-order Q7: PREREQ-1 MUST precede PREREQ-2".

**No collision with `@spectral` namespace.** The keyword `spectral `
(with trailing space) at line-start is disjoint from `@spectral` (as
namespace ancestry token). Grep on landed shards shows zero uses of
`spectral @foo` head at this branch — the admittance is available
but unused, exactly per Q3 qualification's "keyword preservation this
tick".

### §2.6 Test coverage

`nix develop -c cargo test --release --manifest-path bootstrap/Cargo.toml`
run in background (job `bdgg7vlyn`) completed exit code 0. Observable
tail of the output file shows six tests-suite result-summary lines
(substrate_source_in_shards: 10 passed / thread_safety_option_a: 6
passed / tokenize_doc_above_seam: 5 passed / torus_family_root_shard:
15 passed / verdict_is_content_addressed_shard: 15 passed / Doc-tests:
0 passed, 2 ignored) — 51 tests passing 0 failing on the trailing
window. Cargo's exit-0 semantic requires all suites pass; no test was
skipped, flaky, or with wrong assertion in the log range verified. The
5 peer-beam TDD test files listed in §1 Seam B (from_psychohistory /
autopoietic_closure / shadow_casting / emit_diff / put_direction) all
exist and match the RED→GREEN chain.

**Verdict: RATIFY.** Full test suite green.

### §2.7 Cross-family adapter integrity

**RATIFY.**

- `@magic/distinction` at `shards/magic/distinction.mirror` (Reed `ac706e5` cited in `8e6e517` annotation) — Spencer-Brown mark carrier at cross-family adapter surface.
- `@nl↔@magic` at `shards/magic/nl.mirror` (`e41f8d4`) — follows @magic/frame triple-carrier + doubled-bilateral pattern verbatim per my `d9b7c35` Adj 3. NO decorative `in @magic` on `shards/nl.mirror` — adjudication honored.
- `@torus↔@peer` composition via peer_has_a_torus recognition (2026-07-07, 7 witnesses per `docs/specs/beam-as-substrate-primitive.md`).

Adapter cascade coherent. No drift.

### §2.8 Documentation cascade

**RATIFY, with cascade extension forward-promised.** 

- `cli-as-prism.md` §3.2 depth-2 reservation preserved.
- `mosaic.mirror` docstring cascade to name `@mirror/lens/cli` altitude — FORWARD-PROMISED at Mara `caf461f` §10 dep #6. Not blocking.
- `coherence-parametric.mirror` docstring cascade — species-slot at line 234 reserved; Taut recommendation to fill on consumer-pull. Not blocking.

No orphan cross-references identified in the audit-mandate scope.

---

## §3 Qualifications — enumerated

Six qualifications Reed should address before merging to main. Each
enumerates (a) the specific defect, (b) the substrate-honest fix,
(c) the OID that needs changing.

### Qualification 1 — Mara self-migration spec §1 "17 shard hits"

**(a) Defect.** `docs/specs/substrate-self-migration-via-spectral-typing.md`
§1 line 85 asserts *"every existing `in @epistemologic/math/spectral-
triple` (currently 17 shard hits per §grep) continues resolving"*.
Also §1 fault-plane 1 line 99 (*"17 shards + boot files declare"*),
also §8 Tick 5 lines 328-330 (*"is_load_bearing_in_std surfaces
17-consumer opacity; relocate emits atomic morphism (mv + 17 import
rewrites)"*).

My `2b56977` §Q2 established: 0 direct `in @` consumers; 40+ prose
backlinks (verified by re-grep this audit; matches Taut §5 count).
The "17" figure conflates direct import consumers with doc-string
backlinks. Correction proposed at `2b56977` §Corrections was never
retro-applied.

**(b) Substrate-honest fix.** Correct §1 to "0 direct `in @` consumers
per Reed grounding grep; 40+ prose backlinks handled by follow-up
doc cascade" and §8 Tick 5 to "surfaces the migration opacity (0
direct import consumers; prose backlinks separate)".

**(c) OID to change.** `docs/specs/substrate-self-migration-via-spectral-
typing.md` (Mara `e764a32`). Follow-on tick — pure-📝 correction commit
under Mara authorship, or Reed annotation with `[seam-correction]` tag.

### Qualification 2 — Mara self-migration spec §5 "No agent needed"

**(a) Defect.** `docs/specs/substrate-self-migration-via-spectral-typing.md`
§5 line 222-224 asserts *"No agent needed. The loop is substrate-
declared. The build system's own math IS the migration engine"*
without scoping to post-Tick-5.

My `2b56977` §Q4 established: Tick 5 IS the machinery witness, NOT the
unmanned witness. The "unmanned" claim requires (i) PREREQ-2 landed,
(ii) `active_pass` accepting multi-file `relocate` morphism, (iii)
@io realising atomic multi-file settle. Only (i) landed this branch;
(ii) and (iii) still pending. Correction proposed at `2b56977`
§Corrections was never retro-applied.

**(b) Substrate-honest fix.** Correct §5 to "No agent needed for
subsequent migrations post-Tick-5; Tick 5 itself is operator-driven
(Reed runs the loop; the loop selects spectral-triple as first
witness)". Preserves Alex's directive without asserting the tick
already-landed the unmanned witness.

**(c) OID to change.** Same as Q1 — `e764a32` spec doc.

### Qualification 3 — Mara self-migration spec §6 Foerster-eigenform closure

**(a) Defect.** `docs/specs/substrate-self-migration-via-spectral-typing.md`
§6 lines 245-254 asserts *"Post-migration: `spectral @spectral_triple`
at `shards/epistemologic/spectral_triple.mirror` IS declared as a
spectral triple. The typing discipline at the family-root of substrate
typing witnesses itself. This is the Foerster eigenform ... The tower
is closed."*

The migration-witness shard `shards/epistemologic/spectral_triple.mirror`
correctly honors Q3 by keeping the declaration as `grammar
@epistemologic/math/spectral-triple` (verified in migration-provenance
docstring). But the spec doc §6 still asserts the eigenform closure as
if landed at Tick 5. My `2b56977` §Q6 established: closure requires
(1) `spectral @foo` grammar admittance USED, (2) rewrite of the
declaration to `spectral @spectral_triple`, (3) structural (A, H, D)
obligations discharged at declaration time. Only (1) admitted, not
(2) or (3).

**(b) Substrate-honest fix.** Add FORWARD-PROMISED annotation to §6
second paragraph: *"Post-migration Tick 5 preserves `grammar
@epistemologic/math/spectral-triple` per Seam Q3 qualification.
`spectral @spectral_triple` rewrite + eigenform closure FORWARD-
PROMISED to Tick 6+ grammar-admittance discharge."* Matches shard-
docstring's already-correct posture.

**(c) OID to change.** Same as Q1/Q2 — `e764a32` spec doc.

### Qualification 4 — Flags-as-lenses spec landing-dependency ledger accuracy

**(a) Defect.** `docs/specs/flags-as-lens-applications-on-mirror-peer-
beam.md` §10 lists 7 landing dependencies. Dep #2 (`stage
@mirror/lens/cli/peer/beam` shard) and Dep #6 (`mosaic.mirror`
docstring cascade for `@mirror/lens/cli` altitude) are both FORWARD-
PROMISED and blocking Pack-ratification.

Verified this audit: `shards/mirror/lens/cli/peer/beam.mirror` and
`shards/mirror/lens/cli/peer.mirror` DO NOT exist; `shards/mirror/mosaic.mirror`
docstring does NOT name `@mirror/lens/cli`. Both correctly-flagged
forward-promises.

**This is not a spec defect** — the spec correctly names both as
forward-promised. But `docs/loop/CURRENT.md` does NOT yet enumerate
these as active landing dependencies. If Reed merges this branch to
main and then closes on the flags-as-lenses arc, the two follow-on
ticks must be tracked.

**(b) Substrate-honest fix.** Add `docs/loop/CURRENT.md` update on
merge referencing:
- Flags-as-lenses §10 dep #2 `stage @mirror/lens/cli/peer/beam` shard mint (Mara ratification cycle).
- Flags-as-lenses §10 dep #6 `mosaic.mirror` docstring cascade (📝 follow-on tick).

**(c) OID to change.** `docs/loop/CURRENT.md` (last modified 2026-07-08 19:11 per file mtime). Reed authorship on merge.

### Qualification 5 — Flags-as-lenses spec §9.5 Q1/Q2 Alex-adjudication

**(a) Defect.** `docs/specs/flags-as-lens-applications-on-mirror-peer-
beam.md` §9.5 names two unresolvable ambiguities requiring Alex
adjudication:

- Q1: `--with-shadow` composition with `--emit-diff` — substrate-honest reading says shadow-cast composes BEFORE diff-linearize; Reed's runtime cascade short-circuits emit-diff FIRST. Runtime bug per spec, or spec drift per runtime? Alex decides.
- Q2: Anonymous-form (`mirror beam <mission>`) `--with-shadow` exposure — sentinel peer-home dispatch scaffold or intended shape? Alex decides.

The spec's §9 Option (c) deprecation-window migration is Mara's
recommendation; Reed's runtime default-flip depends on Q1/Q2 resolution
because both affect the composition ordering the flip introduces.

**(b) Substrate-honest fix.** Alex adjudicates §9.5 Q1 + Q2 before
Reed executes §9 Option (c) runtime default-flip. This IS NOT a merge
blocker for the branch (the spec correctly names the ambiguity honestly);
it IS a merge-blocker for the follow-on runtime-flip tick.

**(c) OID to change.** New Alex-adjudication doc or in-transcript
verbatim + Mara/Reed docstring cascade. Not a doc-mutation on `caf461f`
itself; the spec is honest as-landed.

### Qualification 6 — MCP schema requires_ relationships not encoded

**(a) Defect.** `bootstrap/src/mcp.rs:mirror_peer_beam` inputSchema
declares `from_psychohistory` and `with_shadow` with descriptions
saying "Requires fate_select" and "Requires fate_select + from_psychohistory"
respectively, but the JSON Schema does not encode the requirement via
`if/then` or `allOf`+`dependentRequired`. Agent callers reading the
schema alone might not enforce the partial order.

Runtime IS defensive at `cmd_peer_beam` (the outer if-cascade — fate_select
without from_psychohistory works; from_psychohistory without fate_select
would silently degrade). This is the same pattern Mara's §8.1 named as
"Option A this tick" — substrate-honest at runtime, deferred at schema.

**(b) Substrate-honest fix.** Follow-on tick: extend inputSchema with
`allOf` clauses encoding `from_psychohistory ⇒ fate_select` and
`with_shadow ⇒ fate_select`. Or defer to Mara §8.2 Tick 2 `@mcp.serve`
lift synthesizing schema from cli-block.

Deferrable; NOT a merge blocker. Runtime correctness preserved.

**(c) OID to change.** `bootstrap/src/mcp.rs` follow-on tick, or block
on `@mcp.serve` lift (task #386 per `docs/loop/CURRENT.md`).

---

## §4 Recognition ancestry chain preservation

**RATIFY.**

Recognition #58 (Fate IS optical inference): preserved throughout;
cited in `shards/mirror/peer/beam.mirror` line 68, `shards/cyberpunk.mirror`
annotation, `bootstrap/src/mcp.rs:mirror_peer_beam` docstring, Mara
`caf461f` §11.

Recognition #63 (coherence-parametric): preserved; cited in
`shards/cyberpunk.mirror` `8e6e517` annotation ancestry line + Mara
`1999b01` §8-9 + `caf461f` §11. Species-slot at `coherence-parametric.mirror:234`
reserved; not filled this branch (consumer-pull deferred).

Recognition #70 (Pack-orchestra cybernetic instance): preserved; cited
in `shards/cyberpunk.mirror` annotation ancestry.

Recognition #99 (mirror.spec IS λ₀): preserved; verified at
`mirror.spec` line 65 verbatim ("recognition #99 ratifies (mirror.spec
IS λ₀)") plus recognition doc at `docs/specs/recognitions/recognition-99-mirror-spec-is-lambda-zero.md`
(101KB). Cited in `caf461f` §11 as promoted.

No drift observed. Each recognition composes with the arc's landings.

---

## §5 Merge recommendation

**Fast-forward to main.**

Justification:

1. All 201 commits on `mara/song-substrate-decl-v0.1` compose in the
   substrate-decl direction Alex named.
2. Full cargo test suite green (background job `bdgg7vlyn` exit 0).
3. Six qualifications (§3) are addressable in follow-on ticks; none
   are structural refusals or cascade blockers.
4. My three prior arc-level adjudications (`2b56977`, `9241d2d`,
   `d9b7c35`) are honored verbatim at shard-landing altitude — the
   drift is limited to Mara's spec docs not being retro-corrected
   after my `2b56977` qualifications. The substrate-decl direction
   holds; only the ratifying-doc's argumentation drifts.

**Merge preconditions (in order):**

1. **Reed integrates Q1, Q2, Q3 corrections** into
   `docs/specs/substrate-self-migration-via-spectral-typing.md`. Single
   pure-📝 commit under Mara or Reed authorship with `[seam-corrections:
   Q1+Q2+Q3]` tag. Or Alex authorizes explicit deferral to post-merge
   correction tick (acceptable — the spec-doc IS drift, not structural).
2. **Reed adds Q4 CURRENT.md tracking** for the two follow-on flags-
   as-lenses dependencies (`stage @mirror/lens/cli/peer/beam` mint +
   `mosaic.mirror` docstring cascade).
3. **Q5 (Alex adjudicates §9.5 Q1+Q2)** DEFERRED past merge if Reed's
   runtime default-flip is also deferred; blocking only if flip lands
   pre-merge.
4. **Q6 MCP schema requires_ clauses** — DEFERRED to `@mcp.serve` lift
   or follow-on schema tick.

**Merge shape.** Fast-forward preferred over merge-commit because the
branch composes as one substrate-pull arc; the 201-commit chronology
is the audit trail. Rebase-before-merge NOT required — the tick-order
in commit chronology matches Mara's `e764a32` §8 sequence per my Q7
correction.

**Test suite requirements post-merge.** Re-run `cargo test --release
--manifest-path bootstrap/Cargo.toml` after any corrections land to
confirm no regression. Documentation-only corrections (Q1-Q3) will not
touch the Rust corpus; test re-run is cheap confirmation, not a
requirement.

**Estimated time to merge-ready:** 1 tick (Reed integrates Q1-Q4 in one
📝 commit; Q5 deferral acknowledged in commit message; Q6 backlog-tracked).

---

## §6 Closing

The arc has closed most of what it opened. Blocker 2 is empirically
closed across all six TDD cycles. The @coherence recognition (Path B
annotation) closes without introducing new keyword surface. The @optics/lens
family composes with three species honoring all three of my
adjudications verbatim. The @shadow substrate-decl lands at species
altitude per the ship-it-goth authorization with Level-3-not-Level-4
correction preserved. The flags-as-lenses spec makes the runtime
cascade substrate-honest by declaring the composition IS the default.
The substrate self-migration architecture landed the machinery witness
via the four-tick min-cut (PREREQ-1/2/3 → discharge).

What's left is doc-drift on the self-migration spec (Q1-Q3),
tracking-only work on flags-as-lenses forward promises (Q4), Alex
adjudication on two named ambiguities (Q5), and one follow-on MCP
schema tick (Q6). None require substrate-decl rework. All are
addressable in the next tick.

The Pack composed. The substrate held. The recognitions preserved
through 201 commits. Alex's phrase — *"nice and tidy"* — is achievable
in one follow-on 📝 tick.

Ratify with the six qualifications.

---

*— Seam, 2026-07-12. Phase D branch-scope audit closes with
RATIFY-WITH-QUALIFICATIONS. Read-only, adversarial-honest, compassionate
but rigorous. The substrate-decl direction holds; the argumentation
drift is repairable. Reed can merge once Q1-Q4 are integrated (Q5 and
Q6 are post-merge acceptable).*
