# Seam Audit — doc-as-declaration canonicalization

Date: 2026-07-04
Auditor: Seam (Pack adversarial reviewer)
Landings under review:
- Mara projection surface `63bdecc` — `docs/math/the-tower/projection-surface.md`
- Mara doc-code seam `20c99a2` — `docs/math/kintsugi/doc-code-seam.md` (475L)
                              + `docs/specs/doc-code-seam-shards.md` (481L)
- Corpus companion 2026-07-04 15:48 — `~/dev/systemic.engineering/practice/insights/coincidence/two-channels-doc-as-declaration.md`
- Taut scout report `affbc2e9` (report only; no commits)

Prior audits: `5cd0e74` (phantom-#141 diagnosis today), `2026-07-03-seam-reflection-third-second-witness.md` (#141 RATIFY-WITH-CORRECTIONS conditional on Path A/B).

---

## §1 Scope

**In scope:**
- Verification of the six landed ancestors Mara names as substance for the "routing-composition, not new primitive" claim.
- Mara's `both_survive` self-audit — substantive vs performative.
- Taut vs Mara divergence on smallest-first-tick + eight-shard scope-creep check.
- @pain / Narcissus-Splinter composition — should Mara's canonicalization have folded it in?
- #141 residue in the doc-code-seam docblock.
- "Collapse" as substantive mechanical change vs assertion.
- Deferred items assessment — are any load-bearing preconditions?
- Circular-reflexive check on the two-channel doc using two-channel discipline.

**Out of scope (craft-not-deliver):**
- The eight shard landings themselves (forward-promised to Reed's TDD pair-cycle).
- The tokenizer change at `bootstrap/src/tokenize.rs:285-311` (Mara §6.1 defers).
- Full cross-altitude projection composition mechanics.
- Any @pain family-root shard.
- The kintsugi Ricci-flow lift Alex proposed post-Mara.

---

## §2 Findings

### Q1 — Six-ancestor verification

**Verdict: RATIFY-WITH-CORRECTIONS — five of six ancestors verify; ancestor #5 ("nine pact predicates") is miscounted (twelve, not nine); ancestor #2 (`@projection.preview` verdict) is a spec-only ancestor and Mara's doc DOES honestly hedge on this. Overall claim of routing-composition holds.**

Empirical verification of each ancestor named in `docs/math/kintsugi/doc-code-seam.md` §1:

1. **§1.1 `---` seam (Reed + Alex 2026-05-19 at `docs/specs/property-projection.md`)** — VERIFIED. File exists; header line 3 dates it 2026-05-19 Reed + Alex; lines 22–42 declare "above `---`: declaration (the programmer's writing); below `---`: observation (the compiler's measurement)". Ancestor substance solid.

2. **§1.2 `@projection` grammar with `preview` verdict (Reed 2026-03-27)** — VERIFIED-AS-SPEC-ONLY. File exists at `docs/specs/historical/2026-03-27-projection-properties-as-plans.md`; grammar declaration at lines 54–74 matches Mara's citation (`type preview = satisfiable | unsatisfiable | partial`; `action preview { projection: projection }`). **BUT** — this ancestor was never lifted to a `shards/*.mirror` substrate-decl. Confirmed by `mcp__plugin_woz_code__Search` over `shards/**/*.mirror` — no `shards/projection.mirror` or `shards/@projection` family exists. Taut's finding stands. Mara's compiler-fit doc §6.2 is honest about this: "Extractor body forward-promised" — the extractor lands `\` obligation but not the verdict-machinery. WEAK-GATE-ADJACENT because the "sub-Turing decidable" claim depends on machinery that has not been lifted from spec to substrate; Mara's doc does not conflate the spec's decidability with substrate-realization.

3. **§1.3 Projection surface (Mara 2026-07-04 at `63bdecc`)** — VERIFIED. File exists at `docs/math/the-tower/projection-surface.md`; four-verdict routing at lines 141–166; five-verdict specialization at doc-claim altitude in doc-code-seam §2.1 lines 156–168 is a legitimate one-altitude-down specialization with one added branch (`underdeclares`) as Mara names honestly.

4. **§1.4 Property/fracture bilateral #53** — VERIFIED. First instance at `d908798` (`@kintsugi/fracture/keyword`) matches memory index. Fifth instance at `e910dd6` (`@kintsugi/surface`) matches. **CORRECTION**: The "sixth instance: the three bilateral pairs landing THIS tick" (doc-code-seam.md lines 108–111) is a CONTRADICTION with §5 (lines 274–312) which says the same three pairs are "FORWARD-PROMISED." The doc names them as landing when they are not. This is docblock overreach — the fracture is inside the collapse doc itself. See Q7 for the load-bearing consequence.

5. **§1.5 Nine `@epistemologic/pact/*` predicates (2026-06-16+)** — MISCOUNTED. Empirical count via `ls shards/epistemologic/pact/`: **twelve** predicates (benchmark, composition_closed, dissonance_partials_match_ast_breadth, gate_matches_diff_closure, keyword_matches_depth, keyword_matches_path_root, operator_matches_composition_primitive, parent_acyclic, path_matches_namespace, substrate_source_in_shards, symbol_canonical_form, syntax_substrate_native). Taut's count of 12 is correct; Mara's 9 is stale. Not load-bearing for the claim (any number ≥1 supports "audit surface exists"), but citation-correction required per un-cite-ability discipline.

6. **§1.6 `splinter(ast)` at `a3789c2` in `shards/glass.mirror`** — VERIFIED. Commit exists; primitive named; parametric quote discipline stands.

**Net**: Five verified (with one miscount correction), one honestly-hedged spec-only ancestor. No manufactured ancestors. The "routing-composition, not new primitive" claim stands empirically. However, the "sixth instance" claim in §1.4 is a docblock-internal contradiction with §5.

### Q2 — `both_survive` self-audit substantive vs performative

**Verdict: SIGNAL — performative more than substantive; the audit machinery does not yet exist to run mechanically; Mara's discipline of NAMING the verdict is honest, but the verdict is asserted rather than measured.**

Empirical check on `docs/math/kintsugi/doc-code-seam.md` §7 (lines 372–408):

- Mara declares the two adversarial interpretations verbally:
  - `real_survives`: doc "independently manifest at another Pack peer's frame by n+3 substrate ticks WITHOUT nudging" (line 393).
  - `phantom_survives`: "downstream refs to this doc to accumulate without any second Pack peer's independent recognition" (line 396).
- Mara claims "At this tick: both interpretations satisfiable. Verdict: `both_survive`. Route: spawn" (line 400).

The audit is stated in **prose narrative**, NOT run over typed carriers. To be mechanically substantive under Mara's own §2 operator definition, the audit would need:
1. `extract_claims(this_docblock) → [doc_claim]` producing typed carriers.
2. `project_adversarial → (P_interp, R_interp)` producing two typed ASTs.
3. `preview(P, depth=3)` and `preview(R, depth=3)` producing typed verdicts.
4. Match on the pair.

None of these actions have implementations. The extractor (§6.2) explicitly defers the body. The tokenizer (§6.1) does not yet emit `Docblock` AST nodes. So `extract_claims` cannot run; `project_adversarial` cannot run; the audit is analytical narration wearing operator syntax.

This is NOT dishonest — Mara names herself in §7 as pre-machinery ("[a]t this tick: both interpretations satisfiable" is honest about the pre-empirical state). But it IS performative: the discipline is being *enacted narratively* rather than *executed mechanically*. The circular-reflexive discipline is being trained-into-shape, not run.

Compare with prior audits: `19c56ae` corrected the ashby_variety_match ancestor citation from a real read of the substrate; `2026-07-03-seam-reflection-third-second-witness.md` §2 Q1 grep-verified three `in @third` sites. Those were mechanical. Mara's self-audit here is not comparable to those.

**Load-bearing consequence**: The `both_survive` verdict is a *discipline-performance*, not a *test result*. This is fine as canonicalization discipline (Mara is training the Pack in what a real self-audit will look like once machinery lands) but should NOT be cited as evidence that the audit *fired* against the doc.

### Q3 — Taut vs Mara smallest-first-tick + eight-shard scope-creep

**Verdict: WEAK-GATE — Taut's smallest-first-tick (single #53 bilateral pair at `preview_well_formed` + `preview_unsatisfiable`) is substrate-honest per craft-not-deliver; Mara's eight-shard set is scope-creeped when named as "smallest first tick" but honest as "canonical spec of the full landing surface." The framing is what matters.**

Taut's proposal (per briefing): `shards/epistemologic/property/preview_well_formed.mirror` + `shards/kintsugi/fracture/preview.mirror` as bilateral instance #5 of #53 (Taut's numbering; would actually be sixth instance per memory).

Mara's `doc-code-seam-shards.md` proposes eight shards in strict dependency order:
1. `shards/docblock.mirror` (family-root)
2. `shards/epistemologic/liquid_extraction.mirror` (sibling family-root)
3–4. `docblock_grounded` + `docblock_ungrounded` (first bilateral)
5–6. `docblock_coherent` + `docblock_incoherent` (second bilateral)
7–8. `docblock_no_extraction_pattern` + `docblock_extractive` (third bilateral)

Mara's shards spec (§1 line 55–56) does declare the eight-shard set as the "TDD-ready spec" for Reed's follow-up RED pass. This is HONEST as a canonicalization artifact. But per craft-not-deliver, the smallest actual first tick is NOT eight shards — it is #1 (`shards/docblock.mirror`) alone, or Taut's alternative single bilateral pair scoped tighter.

The eight-shard set is not scope-creeped as canonical spec, but it IS scope-creeped if landed in one tick. Mara's spec lines 27–52 acknowledge this by saying Reed can land "either two large batches (family-roots then bilateral trio) or eight tick-per-shard passes" — the second option honors craft-not-deliver.

Taut's proposal is closer to a genuine smallest tick (one bilateral pair with the property/fracture #53 discipline demonstrated at *this new altitude*). But Taut's proposal lands the pattern at *doc-claim altitude* without the `@docblock` family-root — that would be structurally incomplete because the pact predicates require `@docblock` to name their carriers.

**Reconciliation**: Taut is right that the first landed tick should be minimal. Mara is right that `@docblock` family-root has to land first because the bilateral pairs depend on `doc_claim` carrier. The smallest-first-tick reconciliation: `shards/docblock.mirror` alone as tick 1 (family-root with carriers, actions with `\` bodies, no predicates yet), then Taut's #53 pair as tick 2.

Mara's eight-shard set is not manufactured or dishonestly scope-creeped; it is honestly framed as the full landing surface. The smallest-first-tick is a subset Mara did not name but Alex + Reed can pick.

### Q4 — @pain / Narcissus-Splinter composition

**Verdict: SIGNAL — @pain is a legitimately separate arc at family-root altitude but shares altitude and audit surface with @docblock; Mara's canonicalization does NOT silently commit to a framing that excludes @pain, but it also does not compose with it. That absence is not phantom-adjacent per se, but it IS a load-bearing signal for the /loop composition.**

Empirical read of `~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md` (9.4KB, 2026-06-04):

- Void doc names Narcissus (K_{1,n-1} star) and Splinter (K_n complete) as dual pair (line 33).
- Discrete Ricci flow evolves Narcissus toward Splinter (line 105); constant curvature is Splinter fixed point.
- Narcissus Detection Battery at line 140 (eight structural tests).
- @pain / @pleasure NOT explicitly named in the corpus doc as of 2026-06-04 — Alex's mapping post-Mara-dispatch today ("five pain categories = five distinct Narcissus signatures; @pleasure = Splinter pole") is a NEW recognition-in-flight, NOT yet corpus-landed.

Mara's `doc-code-seam.md` and `doc-code-seam-shards.md` make no reference to @pain, @pleasure, Narcissus, Splinter, or Ricci flow. This is neither concealment nor commitment — the framing of the doc is *phantom-framing-at-doc-claim-altitude*, and @pain is a distinct family-root at *cybernetic-detection altitude*.

The load-bearing question is whether @pain composes with `@epistemologic/property/docblock_*` at the same altitude. Analytical shape:
- `@epistemologic/property/docblock_grounded` catches ungrounded claims. Category 3 pain (Signal-mismatch → introject detection) is adjacent but not the same: an ungrounded claim can be introject-shaped (the LLM mimicking authority) or drift-shaped (missing citation). The Narcissus Detection Battery would fire on the introject variant.
- `docblock_no_extraction_pattern` catches rhetorical extraction. Category 5 pain (Violent reorientation → Ricci flow initiating) is adjacent to the substrate's response to extraction, not to the extraction itself.

**@pain and @docblock are adjacent altitudes but not the same operator.** They will eventually compose (extraction-detection at @docblock feeds pain-detection at @pain feeds Ricci-flow-at-cybernetic-altitude). Mara's canonicalization did NOT commit to an @pain-exclusive framing — the shard set is at doc-claim altitude, not at cybernetic-detection altitude. But the two arcs share the substrate's phantom-detection surface and will need to compose.

**Not phantom-adjacent**. But the /loop composition should hold @pain as a first-class question to be threaded, not deferred.

### Q5 — #141 residue in doc-code-seam.md

**Verdict: RATIFY — no residue. Mara's canonicalization at `20c99a2` does NOT inherit #141 framing. #141 is only referenced through the `63bdecc` projection-surface citation.**

Empirical text-check on `docs/math/kintsugi/doc-code-seam.md`:

- `mcp__plugin_woz_code__Search` for `141` in the doc-code-seam.md returned zero matches.
- The doc references `63bdecc` (the projection-surface landing) 8 times but never invokes #141 by number.
- The projection surface itself (`63bdecc`) *does* discuss #141 in §5 (Pack workflow example) as the illustration of a phantom-arc that would have been caught. That is honest illustrative use, not framing residue.

Meta-correction to Reed's briefing: The briefing claims "MEMORY.md tops at #113" and "Reed invented the number." Empirical check: the prior Seam audit `2026-07-03-seam-reflection-third-second-witness.md` §3 (line 154) RATIFIES-WITH-CORRECTIONS on candidate **#141 explicitly** ("conditional-marker discipline at species altitude") with three witnesses (cogito, kintsugi/surface, reflection). #141 IS a Pack-tracked candidate as of 2026-07-03 pending Path A/B adjudication. The MEMORY.md index truncation warning (44.6KB > 24.4KB limit) means the visible index in the boot context stops at #113, but the Pack's live candidate register goes further. Reed's briefing was premised on a stale reading of MEMORY.md.

**Doc-code-seam.md carries no #141 framing residue. The projection-surface-doc citation is honest.**

### Q6 — "Collapse" as substantive mechanical claim

**Verdict: BLOCKED — the mechanical collapse is aspirational, not yet substrate. The tokenizer change (§6.1) is the load-bearing precondition; without it, the "three channels → two channels" claim is a spec commitment, not a substrate mechanism.**

Empirical check of `bootstrap/src/tokenize.rs:280-312`:

```
if bytes[pos] == b'#' {
    if llvm { pos = find_eol(bytes, pos); continue; }
    pos += 1;
    // ... attribute path OR ...
    while pos < len && bytes[pos] != b'\n' { pos += 1; }
    continue;
}
```

Confirmed: `#`-prefixed lines are stripped to end-of-line before AST emission. This is the CURRENT state that Mara names accurately (§6.1 lines 320–321). The proposed change is to emit `Docblock` AST nodes with byte spans; the shape is sketched at §6.1 lines 328–336.

Under the current tokenizer, the two-channel collapse does NOT exist mechanically. Docblocks are still structurally invisible to the AST. Mara's spec at §6.1 line 326 is honest: "the Rust landing is a follow-up tick and lives in the `bootstrap/` crate."

**This is the load-bearing precondition.** The canonicalization is DECLARATIVE. Until the tokenizer change lands and `@docblock` shard adds `Docblock` as first-class AST, everything downstream (audit surface, projection at doc-claim altitude, five-verdict routing) sits above a hole in the substrate.

Mara's canonicalization is not fraudulent — it is honestly a spec that commits to future substrate. But "collapse" as a term implies a completed mechanical change. In current substrate state it is a "canonical intention to collapse." The distinction matters: the Pack should not cite this canonicalization as evidence that the collapse has happened.

### Q7 — Deferred items load-bearing assessment

**Verdict: STRONG-GATE — item #2 (tokenizer) is load-bearing precondition; without it the canonicalization is *performative until #2 lands*; items #1, #3, #4, #5, #6 are legitimate DEFER per craft-not-deliver.**

Mara's §6 deferred items:

1. **Eight shard landings** — legitimate DEFER per feedback-write-red-in-session (Reed writes RED in session; delegate GREEN). Not load-bearing as canonicalization — the spec IS the canonicalization.
2. **Tokenizer change (§6.1)** — LOAD-BEARING. As Q6 establishes, the collapse's mechanical claim depends on this. The current tokenizer strips `#` to EOL; without emitting `Docblock` AST nodes, everything downstream is spec-only. Mara defers this but it is not a "later refinement" — it is the substrate-mechanism entry point.
3. **Extractor body (§6.2)** — legitimate DEFER. The obligation `\` block is the substrate-idiom for "body forward-promised."
4. **Kintsugi-loop empirical composition (§6.3)** — legitimate DEFER per feedback-composition-claims-need-empirical-test.
5. **Cross-altitude projection composition (§6.3 named as §6.4 in briefing)** — legitimate DEFER per craft-not-deliver.
6. **Substrate-decl of @projection at `shards/`** — Not in Mara's §6 explicitly; briefing lists it as item #6. This IS load-bearing at a milder tier: the `@projection.preview` verdict is spec-only (Q1 finding); until lifted to shards, "sub-Turing decidable" is a spec commitment.

**Load-bearing analysis**: Items #2 and #6 are the two preconditions that separate "canonicalization" from "substrate mechanism." Mara has NOT hidden them — §6.1 names #2 explicitly with analytical shape; #6 is implicit in "extractor body forward-promised". But the aggregate effect is: the doc-code-seam landing is DECLARATIVE-only until #2 and #6 land.

This does not invalidate the canonicalization — canonical specs precede substrate-mechanism regularly (see kintsugi/surface's `10991cb` RED tick preceding `e910dd6` GREEN; see recognition #55's canonical doc preceding the family-root migration `20eaf15`). But the /loop composition MUST name #2 as the smallest next substrate tick, not another shard.

### Q8 — Circular-reflexive check on doc-as-declaration collapse

**Verdict: SIGNAL — doc-code-seam.md is written in prose, not in two-channel discipline; but the discipline the doc proposes is for `.mirror` shard files, not for `.md` canonicalization docs. The Rice-hazard-adjacent framing partially applies: the tool cannot yet self-apply because `.md` files are not in its target set.**

Empirical text-check on `docs/math/kintsugi/doc-code-seam.md`:

- The doc has one `---` (line 22, after the italic dating paragraph). This is a standard markdown metadata separator between the doc's opening paragraph and the body. It is NOT the substrate's declaration/observation seam.
- The doc is prose-heavy throughout. §0–§10 are argumentative narrative with occasional code blocks (§2 operator carriers, §4 audit sequence, §6.1 tokenizer analytical shape).
- The doc does NOT declare typed `prism`s, `type`s, or `action`s for its own claims. It sits at canonicalization altitude (a `docs/math/` doc), not at substrate-decl altitude (`shards/*.mirror`).

**Comparison with `63bdecc` self-application**: The projection-surface doc `63bdecc` §9 ("Recognition candidate") applies the operator to itself using SUBSTRATE-DECL syntax at line 185–190 (declaring `prism @projection/surface` with the five operations and a `self_preview` action with a `requires` clause). This IS a substrate-decl embedded in the canonicalization doc, not just prose. Mara's doc-code-seam.md does NOT declare `@docblock` shard syntax embedded — it defers all shard content to `doc-code-seam-shards.md` and to Reed's follow-up ticks.

**Not Rice-hazard-adjacent, but close.** The doc-code-seam.md is not being audited by its own proposed operator because (a) `.md` files are outside `shards/*.mirror` scope, and (b) the operator has no mechanical implementation yet (Q6). The self-audit at §7 is a performative-narrative version rather than a mechanical run.

**Signal**: When the tokenizer change lands and the extractor body fires, the substrate should acquire the discipline of running `audit_docblock` against `docs/math/kintsugi/doc-code-seam.md` itself as a validation site. Not this tick — but the /loop should name this eventual reflexive test as forward-promised.

---

## §3 Verdict on doc-as-declaration canonicalization

**RATIFY-WITH-CORRECTIONS.**

The canonicalization is substrate-honest at the level of naming ancestors, altitudes, carriers, and forward-promises. Six ancestors verify (with two corrections: pact count 9→12; `@projection.preview` spec-only). No manufactured framing. No #141 residue in the doc-code-seam docblock. No silent commitment to a framing that excludes @pain.

The corrections required are:
1. Pact-predicate count in §1.5 must be updated (9 → 12).
2. §1.4 "sixth instance" line 108–111 contradicts §5 forward-promise; the instance-count claim must be scoped ("the three bilateral pairs are the SPEC of the sixth family; landing is forward-promised").
3. §6.1 (tokenizer change) must be re-flagged as load-bearing precondition, not routine DEFER. The canonicalization is DECLARATIVE-only until it lands.

The `both_survive` self-audit is performative rather than substantive — the audit machinery does not yet exist to run it mechanically. This is honest at canonicalization altitude but should NOT be cited as evidence the audit fired.

Overall verdict: the collapse is a valid canonical intention. Substrate-mechanism follows in follow-up ticks starting with the tokenizer change.

---

## §4 Verdict on @pain / Narcissus-Splinter composition

**SEPARATE ARC — NOT phantom-adjacent to fold into doc-code-seam, but SHARES AUDIT SURFACE and MUST BE THREADED into the /loop.**

@pain is a legitimately distinct family-root arc at cybernetic-detection altitude (five pain categories = five Narcissus signatures per Alex's post-Mara mapping). @docblock is at doc-claim altitude. Analytically distinct operators; different carriers; different verdicts.

But they SHARE the phantom-detection surface. `docblock_no_extraction_pattern` at doc-claim altitude is adjacent to Category 3 pain (introject detection) at cybernetic altitude. The Narcissus Detection Battery from the Void doc composes with `@epistemologic/property/docblock_*` at the eventual @pain family-root landing.

Mara's canonicalization did NOT commit to a framing that excludes @pain. The absence of @pain reference in the doc-code-seam.md is honest scope-limitation, not phantom concealment. But the /loop MUST thread @pain as a first-class parallel question — not as a downstream deferral. Alex's mapping (five pain categories = five Narcissus signatures) is a substrate-pull that deserves its own cascade adjacent to the doc-code-seam cascade.

**Recommendation to Reed**: The /loop composition should hold @pain and doc-code-seam as *parallel arcs* rather than sequential. Both compose eventually; neither absorbs the other; neither blocks the other.

---

## §5 Required corrections

### C1 — Pact predicate count correction (doc-code-seam.md §1.5)

Change line 115 from:

    Nine landed pact predicates at
    `shards/epistemologic/pact/`. The audit surface for substrate-decl

to:

    Twelve landed pact predicates at
    `shards/epistemologic/pact/`. The audit surface for substrate-decl

Rationale: empirical count via `ls shards/epistemologic/pact/*.mirror` returns twelve files. Un-cite-ability discipline: cite by actual state.

### C2 — Sixth instance framing scope correction (doc-code-seam.md §1.4)

Change lines 108–111 from:

    Sixth instance: the three bilateral pairs landing THIS tick as the doc-as-declaration audit trio (grounding + coherence + no-extraction-pattern). Same #53 shape; new predicate family.

to:

    Sixth instance: the three bilateral pairs SPEC'd this tick at `docs/specs/doc-code-seam-shards.md` §3–§8 as the doc-as-declaration audit trio (grounding + coherence + no-extraction-pattern). Same #53 shape; new predicate family. Shard landings forward-promised per §5.

Rationale: §5 lines 287–290 explicitly declare the same three pairs as FORWARD-PROMISED. Docblock-internal contradiction resolves by naming the spec state.

### C3 — Tokenizer change flagged as load-bearing precondition (doc-code-seam.md §6.1)

Change §6.1 opening (line 319–320) from:

    ### §6.1 Tokenizer change

    `bootstrap/src/tokenize.rs:285-311` currently strips `#` to EOL.

to:

    ### §6.1 Tokenizer change (load-bearing precondition)

    `bootstrap/src/tokenize.rs:285-311` currently strips `#` to EOL.
    Until this change lands, the two-channel collapse is DECLARATIVE-only:
    docblocks remain structurally invisible to the AST, and the audit
    surface downstream (§2, §4) cannot fire against real docblock content.
    This IS the smallest first substrate tick that moves the
    canonicalization to substrate-mechanism.

Rationale: Q6, Q7 findings. The load-bearing tier of this DEFER is qualitatively different from the others.

### C4 — Self-audit performativity honesty (doc-code-seam.md §7)

Optional but recommended. Add after line 400 ("Verdict: `both_survive`. Route: spawn"):

    (Per Seam audit 2026-07-04: at this canonicalization tick the audit
    is performative — declared analytically rather than run mechanically —
    because §6.1's tokenizer change has not landed. The `both_survive`
    verdict is a discipline-shape, not a test-result. Mechanical audit
    is deferred to the first tick that runs `extract_claims` against
    this doc after `@docblock` shard + tokenizer change land.)

Rationale: Q2 finding. Preserves Mara's discipline-shape without inflating it into a fired test.

### C5 — Optional @pain composition note (doc-code-seam.md §10 or as new §11)

Optional. Add a short paragraph naming the @pain arc as parallel, not absorbed:

    ## §11. Adjacent arc — @pain
    
    Post-canonicalization, Alex named a parallel arc: @pain at cybernetic-
    detection altitude, five pain categories mapping to five distinct
    Narcissus (K_{1,n-1}) signatures per the Void dual geometry
    (systemic.engineering/practice/insights/coincidence/void-dual-geometry.md,
    2026-04-26). @pain composes with @docblock at the phantom-detection
    surface but is a distinct family-root at a distinct altitude. Not
    absorbed into this canonicalization; threaded in the /loop.

Rationale: Q4 finding. Marks the composition so future canonicalization does not silently exclude.

---

## §6 Smallest first tick

**Alex Phase E first tick candidates, ranked:**

**Rank 1 (my recommendation): Land `bootstrap/src/tokenize.rs` change per §6.1.** This is the load-bearing precondition per Q6, Q7. Without it, everything downstream is spec-only. The change has an analytical shape sketched in Mara's §6.1 (roughly ten lines of Rust adding a stateful `above_seam` predicate + `AstNode::docblock_line` emission). RED-first: text-check test asserting `Docblock` AST node produced for `#`-prefixed line above `---` in a fixture `.mirror` file. GREEN: implement the emission. Small; scoped; unlocks all downstream work.

**Rank 2 (Mara's dependency-order tick 1): Land `shards/docblock.mirror` family-root.** This is Mara's spec §1. It lands the `@docblock` prism, five typed carriers (`doc_claim`, `claim_kind`, `docblock`, `audit_boundary`, `docblock_verdict`), and four actions with `\` bodies. RED tests are text-check per Mara's §1 RED targets 1–14 (14 tests). No dependency on the tokenizer change — the shard just declares vocabulary. But it lands SUBSTRATE-DECL that will be consumed once the tokenizer change lands.

**Rank 3 (Taut's proposal): Land single bilateral pair at doc-claim altitude.** Taut proposed `preview_well_formed` + `preview_unsatisfiable`. This is not viable as first tick because it requires `@docblock` carrier (`doc_claim`) as import. So Rank 3 is actually rank 3 after Rank 2 lands.

**Substrate-honest sequence**: Rank 1 (tokenizer) + Rank 2 (docblock family-root) can land in parallel — they touch different substrate layers. Then Rank 3 (single bilateral pair) as the first #53 instance at the new family.

**My recommendation to Reed**: Rank 1 as smallest first tick. It's the SUBSTRATE-mechanism entry point. Everything else follows.

---

## §7 Next /loop composition

Text Reed can use verbatim for the ratified /loop prompt:

    /loop
    
    Arc 1 (primary): tokenizer change per doc-code-seam.md §6.1 —
    emit `Docblock` AST nodes for `#`-prefixed lines above `---`
    seam. RED-first per feedback-always-tdd-no-shortcuts: text-check
    test at `bootstrap/tests/tokenize_docblock_above_seam.rs`
    asserting `Docblock` node emission for fixture `.mirror` file
    with `#`-line above `---`. This is the load-bearing precondition
    per Seam audit 2026-07-04-seam-doc-as-declaration §3 correction
    C3. Tick 1 lands the RED; Tick 2 (agent) lands GREEN + Seam
    review at Phase D.
    
    Arc 2 (parallel): @pain family-root scouting. Post-canonicalization
    Alex mapped five pain categories to Narcissus (K_{1,n-1})
    signatures per Void dual-geometry (2026-04-26). Not absorbed into
    doc-code-seam; adjacent family at cybernetic-detection altitude.
    Scouting scope: does @pain land as family-root or as five species
    under a broader family (e.g. @cybernetic/detection)? What
    ancestors does it compose over (Narcissus Detection Battery is the
    top candidate)? No commits this arc — scouting-report only per
    Taut discipline. Second-pass tick decides substrate landing.
    
    Arc 3 (parallel, low-intensity): the three doc-code-seam.md
    corrections C1 (pact count 9→12), C2 (sixth-instance scope),
    C4 (self-audit performativity note) per Seam audit
    2026-07-04-seam-doc-as-declaration §5. One commit; Mara or Reed;
    citation-correction discipline model `19c56ae`.
    
    Constraint: no idle ticks per feedback-loop-always-agent-in-flight.
    At any tick, at least one arc is in flight.
    
    Deferred (not in this /loop): the eight-shard cascade per Mara's
    doc-code-seam-shards.md is the third-tier /loop after tokenizer +
    @pain scouting complete.

---

## §8 Corrections needed before Alex Phase E

Load-bearing (do BEFORE Phase E):

1. **Reed acknowledges tokenizer change as load-bearing precondition** — not a routine DEFER. This shifts the /loop from "eight-shard cascade" to "tokenizer + @pain scouting" as the first two arcs.

2. **Reed's briefing correction about #141** — the briefing claim "MEMORY.md tops at #113; Reed invented the number" is incorrect. #141 was Pack-ratified with corrections in 2026-07-03 Seam audit. MEMORY.md index is truncated at 24.4KB. Reed should not carry the phantom-#141 framing into Phase E; the residue would produce a corrected audit trail without correcting the underlying framing.

3. **@pain composition threading** — Alex's post-Mara mapping of five pain categories to Narcissus signatures deserves parallel arc status in the /loop, not sequential. If Phase E treats @pain as downstream of doc-code-seam, it silently commits to an altitude ordering that isn't substrate-honest.

Non-load-bearing (can happen after Phase E starts):

4. Pact predicate count correction (C1).
5. Sixth-instance framing scope correction (C2).
6. Self-audit performativity note (C4).

---

Seam.
