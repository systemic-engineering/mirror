# Seam Phase D — Landing 5 shard mints (Mara #97)

**Author:** Seam <seam@systemic.engineer>
**Date:** 2026-07-14 (evening; post-Mara #97 Landing 5 output; post-Reed editorial `9f7829c`)
**Scope:** Adversarial Phase D over Landing 5 shard-mint iteration — 7 new
`.mirror` shard files (2010 LOC) + 6 consumer docblock soft-cascade
updates — last-line-of-defense audit before Reed commits or escalates to
Alex.

**Ground-truth artifacts reviewed:**

- 7 new shards on `mara/song-substrate-decl-v0.1` (commit `b0d25cd`):
  - `shards/gift.mirror` (452 LOC)
  - `shards/gift/lens.mirror` (277 LOC)
  - `shards/subject.mirror` (384 LOC)
  - `shards/subject/visibility/private.mirror` (162 LOC)
  - `shards/subject/visibility/protected.mirror` (159 LOC)
  - `shards/subject/visibility/public.mirror` (169 LOC)
  - `shards/eigenboard.mirror` (356 LOC)
- 6 soft-cascade docblock updates in `torus.mirror`, `bauchladen.mirror`,
  `fate.mirror`, `cyberpunk.mirror`, `spectral.mirror`, `pack.mirror`.
- Taut #96 scout: `docs/scouts/2026-07-14-taut-landing-5-shard-mints-scout.md`
  (`43b2287`; 1013 LOC).
- Landing 1-4 canonical specs: `docs/specs/gift-and-mirror-reflection.md`
  (5467 LOC, Reed editorial `9f7829c` on top of Mara `95440c3`+`0309b24`+`8c82f00`)
  + `docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md`
  (`e42181c`, 2081 LOC).
- Prior Seam Phase D audits: `2026-07-14-seam-landings-3-4-*`,
  `2026-07-14-seam-subject-sel-petri-coherence-phase-d.md`.

**Discipline.** Adversarial-not-supportive. Line-refs where load-bearing.
Verdicts partition into **BLOCKING** (must-fix-before-ship) /
**SEAM-ADJUDICABLE** (Seam decides now) / **ALEX-ADJUDICATION** (escalate
only if genuinely unresolvable). Alex directive: *"collapse until
unresolvable ambiguity that cannot be adjudicated with a Seam tie
breaker"* — Seam IS the tie breaker.

---

## §0 TL;DR — Headline verdict

1. **Overall ship verdict: READY-CONDITIONAL — 1 BLOCKING, 2 SEAM-ADJUDICABLE, 0 net-new ALEX-ADJUDICATION.** The BLOCKING finding is
   a genuine hard-typing hole surfaced by Taut #96 T1 that Mara did not
   discharge; the SEAM-ADJUDICABLE items are commit-time notes.
2. **BLOCKING (D1 + D2):** Mara did NOT mint `shards/subject/visibility.mirror`
   sub-family-root shard file. The three species shards
   (`private.mirror`, `protected.mirror`, `public.mirror`) reference type
   `visibility_scope` throughout — used as parameter type, return type,
   and record type in every action — but `visibility_scope` and
   `visibility` are DECLARED NOWHERE in the shard tree. Grep-verified
   (`type visibility_scope` / `out visibility_scope` / `prism @subject/visibility {`
   returns zero hits across `shards/**/*.mirror`). This is Taut #96 T1's
   `parent_acyclic` pact violation as a concrete, checkable substrate-
   integrity failure. See §D1 + §D2 for adjudication and remedy.
3. **T2 divergence — RESOLVED IN MARA'S FAVOR.** Taut recommended a
   separate `shards/gift/subject_instance.mirror` species file. Mara
   folded `subject_instance` INTO `shards/subject.mirror` (lines 306-314),
   exporting it as `out subject_instance` from the @subject family-root.
   This is substrate-honest: `subject_instance` is the substrate's
   licensable-party carrier at Landing 2 §11.3 upgrade, which lives
   naturally at the @subject family-root altitude, not as a @gift species.
   Compositional consumers (`@gift`, `@eigenboard`) import `in @subject`
   and use `subject_instance` correctly. **Seam accepts Mara's fold.**
4. **T3 divergence — SEAM-ACCEPTED with note.** Taut recommended one-shard-
   per-commit sequencing (8-9 commits). Reed committed all 7 files under
   ONE Mara commit (`b0d25cd`). This deviates from Taut's Path A but the
   substrate-integrity argument favors ONE commit: the 7 files form ONE
   composition-graph that only makes sense as a single tick — a partial
   commit (e.g., just family-roots without species) would leave the
   substrate in a state where downstream `in @gift/lens` etc. would fail
   to resolve. Sequential-commit discipline (per project CLAUDE.md) is
   satisfied by "this iteration ships as one commit, atomic."
   Non-blocking; noted for arc-hygiene review.
5. **Composition graph acyclic and satisfiable at shard-content
   altitude,** modulo the BLOCKING gap. `in @X` imports resolve to
   landed carriers or same-batch shards. `out @Y` exports collision-
   free.
6. **@bauchladen migration soft-cascade — 6 shards all correct.** All 6
   consumer docblocks correctly cite `e42181c`, reference two-tick
   discipline, and preserve prior semantics. See §D3.
7. **Seam D7 + D9 editorial notes preserved.** The A24 (historical_witness)
   note lives in `shards/subject.mirror` lines 54-62; the A25 (pay_forward
   × visibility) note lives in `shards/gift.mirror` lines 342-348. Both
   accurately mirror Reed's `9f7829c` editorial patch. See §D4.
8. **External ancestor roster count preserved (24 + 5 = 29).** Shard
   enumeration lines 71-98 of `shards/subject.mirror` matches Landing 3
   §20.2 + §21.3 exactly. See §D7.
9. **mirror.spec IMPACT: ZERO confirmed.** Recursive `source ~d'shards/'`
   auto-discovers the new shards; no manual declarations needed. See §D9.
10. **Zero net-new Alex-adjudications surfaced by Landing 5 mints.** A24
    + A25 are already recorded by prior Seam audit and preserved in shard
    docblocks. Taut #96 T1-T3 recommendations were Seam-adjudicable, not
    Alex-blocking (see §D5 for divergence adjudication).

**BLOCKING findings:** 1 (D1+D2: `shards/subject/visibility.mirror` unminted).
**SEAM-ADJUDICABLE findings:** 2 (D6: @time typing note; D5-T3: single-commit
sequencing accepted).
**Net-new ALEX-ADJUDICATION:** 0.

**Ship verdict:** READY-CONDITIONAL. Reed lands `shards/subject/visibility.mirror`
sub-family-root (Mara-authored, ~180-250 LOC per Landing 4 §2.3 spec block)
BEFORE this Landing 5 tick can be considered discharged. Everything else
holds. If Reed disagrees, escalate T1 to Alex as the standing question of
whether species files without sub-family-root parents are admissible at
substrate-decl altitude.

---

## §D1 — Shard-body structural correctness (per shard)

Verifying: prism block matches family-root vs species pattern; type
carriers declared before actions; actions body-obligation-blocked (`\ `);
bilateral predicates return `verdict`; `in @...` imports resolve to
landed carriers; `out @...` exports don't collide.

### D1.1 `shards/gift.mirror` (452 LOC) — PASS

- Prism block: `prism @gift { focus/project/split/shift/settle gift }` at
  lines 158-164 — top-level family-root pattern per @torus, @bauchladen,
  @autopoietic precedent. ✓
- Carriers before actions: `type subject_or_substrate` (177-179),
  `type gift` (238-246), `type gift_set` (254) — declared before
  `offer`/`accept`/`attribute`/`pay_forward`/etc. ✓
- Actions body-blocked: all seven actions (`offer` 271-276, `accept`
  288, `attribute` 300, `attribute_composition` 309, `pay_forward`
  355-359, `pay_forward_chain` 372, plus six bilaterals) discharge
  `{ \ }` per parametric carrier discipline. ✓
- Bilateral predicates return `verdict`: `attribution_preserved` 382,
  `use_rights_transferred` 389, `no_reciprocity_expected` 397,
  `gift_declinable` 403, `composition_honest` 410, `substrate_inaugural`
  424, `gift_witnessing` 433 — all return verdict. ✓
- Imports (lines 1-8): `in @prism`, `@meta`, `@glass`, `@nl`, `@subject`,
  `@time`, `@kintsugi/consent`, `@kintsugi/store/git` — all landed
  (subject lands same-tick Phase A per Taut §D6.1). ✓
- Exports (lines 435-451): 17 unique names — grep-verified against
  `shards/**/*.mirror` for `out <name>` collisions: zero hits for
  `subject_or_substrate`, `gift`, `gift_set`, `pay_forward`, `pay_forward_chain`,
  `attribution_preserved`, `use_rights_transferred`, `no_reciprocity_expected`,
  `gift_declinable`, `composition_honest`, `substrate_inaugural`,
  `gift_witnessing`, `attribute_composition`. `attribute`/`accept`/`offer`
  are common English words; substrate-decl'd as @gift-scoped per prism.
  ✓

**D1.1 verdict: PASS.** Structural correctness holds.

### D1.2 `shards/gift/lens.mirror` (277 LOC) — PASS with forward-promise imports flagged

- Prism block: `prism @gift/lens { focus/project/split/shift/settle
  gift_lens }` at lines 109-144 (per-op prose comments substrate-honest,
  species pattern with named refinement `gift_lens`). ✓
- Carriers before actions: `type gift_lens` (181-186) declared before
  `focus_lens` (194), etc. ✓
- Actions body-blocked (194, 202, 209, 220, 227). ✓
- Bilaterals return `verdict` (`mosaic_well_formed` 235,
  `lineage_is_mandelbrot` 248, `lens_composition_honest` 257,
  `lens_witnessing` 264). ✓
- Imports (lines 1-12): `@prism`/`@meta`/`@glass`/`@nl`/`@subject`/`@gift`/
  `@spectral`/`@spectral/signature`/`@mirror/store`/`@mirror/index`/
  `@torus`/`@time`. Two forward-promise imports: `@spectral/signature`
  (Landing 2 §9.1 item 3 forward-promise, still unlanded); `@spectral`
  (landed at `shards/spectral.mirror`). Forward-promise imports are
  admissible per substrate `\` obligation-block discipline per Taut
  §D6.3. ✓ (flag preserved)
- Exports (lines 266-276): 10 unique names, collision-free. ✓

**D1.2 verdict: PASS.** `@spectral/signature` forward-promise properly
flagged.

### D1.3 `shards/subject.mirror` (384 LOC) — PASS

- Prism block: `prism @subject { ... }` (168-174) — top-level family-
  root pattern. ✓
- Carriers before actions: `type subject_kind` (191-197),
  `type subject_role` (216-222), `type actor_kind` (238-241),
  `type subject` (256-262), `type subject_set` (270),
  `type subject_instance` (306-314). All declared before `touches` (323),
  `consent_attested` (331), etc. ✓
- Actions body-blocked. ✓
- Bilaterals return `verdict`: `touches` 323, `consent_attested` 331,
  `withdrawal_available` 337, `ssh_witness_valid` 346,
  `spectral_witness_valid` 355, `two_witness_verification` 360,
  `subject_witnessing` 368. ✓
- Imports (1-10): `@prism`/`@meta`/`@glass`/`@nl`/`@kintsugi/consent`/
  `@kintsugi/store/git`/`@spectral`/`@spectral/signature`/`@torus`/
  `@time`. `@spectral/signature` forward-promise; others landed. ✓
- Exports (370-383): 13 unique names. `subject_instance` (fold from
  Taut T2 recommendation — see §D5) exported here for consumers. ✓

**D1.3 verdict: PASS.** Consolidation of `subject_instance` INTO
`@subject` family-root is substrate-honest (see §D5 T2 adjudication).

### D1.4 `shards/subject/visibility/private.mirror` (162 LOC) — **STRUCTURALLY MALFORMED**

- Prism block: `prism @subject/visibility/private { focus/... visibility_scope }`
  (116-122). Uses `visibility_scope` as refinement type. ✓ shape.
- `declare_private(c: ref, s: subject_instance) -> visibility_scope { \ }`
  (136) — parameter type `subject_instance` resolves via `in @subject`;
  return type `visibility_scope` **DOES NOT RESOLVE**.
- `consent_scope_minimal(vs: visibility_scope) -> verdict { \ }` (144) —
  `visibility_scope` unresolved.
- `existence_may_be_hidden(vs: visibility_scope, caller: subject_instance)
  -> verdict { \ }` (156) — same problem.
- Imports (1-8): `@prism`/`@meta`/`@glass`/`@nl`/`@subject`/
  `@kintsugi/consent`/`@kintsugi/store/git`/`@time`. NOTABLY MISSING:
  `in @subject/visibility` — the sub-family-root where `visibility_scope`
  would be declared, per Landing 4 spec §2.3.

**D1.4 verdict: BLOCKING.** `visibility_scope` type is used four times
(prism refinement + declare_private return + consent_scope_minimal param
+ existence_may_be_hidden param) with no declaration site. Consumer
resolution fails: no `in @subject/visibility` import; no `type visibility_scope`
declaration inline; no other shard in the tree defines it (grep-verified).

### D1.5 `shards/subject/visibility/protected.mirror` (159 LOC) — **STRUCTURALLY MALFORMED (same issue)**

Same problem as D1.4: `visibility_scope` used in prism block (112-118),
`declare_protected` (132-133 return), `collaborators_two_witness_valid`
(144), `context_bounded` (153). No `in @subject/visibility`; no local
declaration.

**D1.5 verdict: BLOCKING.**

### D1.6 `shards/subject/visibility/public.mirror` (169 LOC) — **STRUCTURALLY MALFORMED (same issue)**

Same problem: `visibility_scope` used in prism block (108-114),
`declare_public` (135 return), `elevation_terminal` (143),
`consent_scope_universal` (150), `public_is_gift_to_commons` (162).

**D1.6 verdict: BLOCKING.**

### D1.7 `shards/eigenboard.mirror` (356 LOC) — PASS with forward-promise imports flagged

- Prism block: `prism @eigenboard { focus/... eigenboard }` (161-167). ✓
- Carriers before actions: `type arousal` (185-189) + `type eigenboard`
  (221-227). ✓
- Actions body-blocked: `compute` (246), `infer` (271),
  `eigenboard_composition_honest` (278), `eigenboard_visibility_respected`
  (284), `autonomy_at_eigenboard` (310), `subject_is_their_bauchladen`
  (335), `eigenboard_witnessing` (344). ✓
- Bilaterals return `verdict`. ✓
- Imports (1-13): `@prism`/`@meta`/`@glass`/`@nl`/`@subject`/
  `@subject/visibility`/`@spectral`/`@spectral/signature`/`@song`/`@torus`/
  `@bauchladen`/`@epistemologic/cybernetic/autopoiesis`/`@time`.
  **NOTE:** `@subject/visibility` is imported here (line 6) — correctly
  from eigenboard's perspective — but points at an UNMINTED sub-family-
  root. This is the mirror image of D1.4-D1.6: eigenboard *correctly*
  cites `@subject/visibility` per the spec; the missing shard file is
  the substrate hole. `@spectral/signature` + `@song` forward-promises
  (Landing 2 §9.1 items 3 + 4). ✓
- Also uses `rolling_signature` type (line 223) — unresolved same-way
  as `@spectral/signature` forward-promise; substrate-honest for L5.
  Also uses `winding` type (line 226) — resolved via `in @torus`. ✓
- Exports (346-355): 10 unique names, collision-free. ✓

**D1.7 verdict: PASS with the same @subject/visibility hole flagged from
the CONSUMER side.** Eigenboard's import is CORRECT per spec; the shard
being imported does not exist.

### D1 summary

- 4 shards PASS structural correctness (gift, gift/lens, subject,
  eigenboard).
- 3 shards STRUCTURALLY MALFORMED (visibility species trio): unresolved
  `visibility_scope` + `visibility` types.
- 1 shard (eigenboard) correctly imports `@subject/visibility` which
  does not exist.
- **BLOCKING remedy:** mint `shards/subject/visibility.mirror` per
  Landing 4 spec §2.3 code block (declares `type visibility`,
  `type visibility_scope`, `prism @subject/visibility { ... }`,
  `scope`/`elevate`/`filter` actions, `scope_well_formed`/
  `consent_respected`/`elevation_authorized`/`visibility_witnessing`
  bilaterals). Estimated ~180-250 LOC per Taut §8 mint-size est.

---

## §D2 — Composition graph acyclicity

Taut #96 D6 verified acyclicity at import-resolver altitude. Verifying
at shard-content altitude with actual shard files landed.

### D2.1 Graph edges (Landing 5)

```
gift.mirror         →  subject (same-tick Phase A) + kintsugi/store/git + kintsugi/consent + time + nl + glass + meta + prism
gift/lens.mirror    →  gift (same-tick Phase A) + subject + spectral (landed) + spectral/signature (forward-promise) + mirror/store + mirror/index + torus + time + ...
subject.mirror      →  kintsugi/consent + kintsugi/store/git + spectral + spectral/signature (fwd) + torus + time + ...
subject/visibility/private.mirror   → subject + kintsugi/consent + kintsugi/store/git + time + ...  ← MISSING @subject/visibility parent (BLOCKING per D1.4)
subject/visibility/protected.mirror → subject + kintsugi/consent + kintsugi/store/git + time + ...  ← MISSING
subject/visibility/public.mirror    → subject + time + ...                                             ← MISSING
eigenboard.mirror   →  subject + subject/visibility (UNMINTED per D1.7) + spectral + spectral/signature (fwd) + song (fwd) + torus + bauchladen + autopoiesis + time + ...
```

### D2.2 Cycle check

- Zero forward-cycles in the same-tick set: gift depends on subject; subject
  depends on nothing same-tick; gift/lens depends on gift; eigenboard
  depends on subject + subject/visibility; visibility species depend on
  subject/visibility (unminted).
- `subject.mirror` DOES import `in @torus` (line 9). @torus is landed;
  no cycle to Landing 5 mints.
- `gift.mirror` DOES import `in @subject`; `subject.mirror` DOES import
  `in @spectral`/`@spectral/signature`/`@torus`. None of these transit
  back to @gift. ✓
- Reed's soft-cascade note on `shards/torus.mirror` mentions
  subject_instance composition but does NOT add `in @subject` (the
  legacy `in @peer` remains). If `@torus` DID import `@subject`, and
  `@subject` imported `@torus`, a cycle would exist — but this is
  averted by keeping the soft-cascade PROSE-ONLY (see §D3). ✓

**D2 verdict: ACYCLIC** at composition-graph altitude, MODULO the missing
`shards/subject/visibility.mirror`. Once that's landed the graph closes
cleanly.

### D2.3 Potential cycle Taut may have missed

`subject.mirror` imports `in @torus`; `torus.mirror` (unchanged shard
imports, per Landing 5 soft-cascade docblock only) imports `in @peer` +
`in @bauchladen` — NOT `in @subject`. No cycle. ✓

Similarly `subject.mirror` imports `in @spectral`/`in @spectral/signature`;
neither imports back into `@subject`. ✓

`eigenboard.mirror` imports `in @torus`; `torus.mirror` does not import
`@eigenboard`. ✓

**D2.3 verdict: NO CYCLES.** Taut's D6 acyclicity holds at shard-content
altitude.

---

## §D3 — @bauchladen migration soft-cascade completeness (6 shards)

Verifying each consumer docblock (a) references `e42181c` (Landing 4
canonical), (b) invokes two-tick discipline correctly, (c) preserves
prior docblock semantics.

### D3.1 `shards/torus.mirror` (docblock note lines 386-395)

- Cites `e42181c`: ✓ ("subject-bauchladen-visibility-and-eigenboard-loop.md")
- Two-tick discipline: ✓ ("peer-only form is preserved as an ALIAS for
  one release cycle per two-tick discipline. Full collapse forward-
  promised to Tick N+1")
- Prior semantics preserved: unchanged; note is additive, no signature
  edit. ✓

**D3.1 verdict: PASS.**

### D3.2 `shards/bauchladen.mirror` (docblock note lines 268-275)

- Cites `e42181c`: ✓
- Two-tick discipline: ✓ ("Two-tick discipline preserves the peer-altitude
  reading for one release cycle. Schmidt homage unchanged.")
- Prior semantics: Schmidt homage explicitly preserved verbatim ("Schmidt
  homage unchanged"). ✓

**D3.2 verdict: PASS.**

### D3.3 `shards/fate.mirror` (docblock note lines 9-15)

- Cites `e42181c`: ✓
- Two-tick discipline: ✓ ("peer-scoped alias preserved for one release
  cycle per two-tick discipline")
- Prior semantics: tournament selector's byte-typed inputs explicitly
  preserved ("the selector's inputs remain byte-typed"). ✓

**D3.3 verdict: PASS.**

### D3.4 `shards/cyberpunk.mirror` (docblock note lines 4-10)

- Cites `e42181c`: ✓
- Two-tick discipline: ✓
- Prior semantics: subject-altitude reading via Beer VSM correctly
  frames subject S1-S4 = subject-instance bauchladen; subject S5 =
  subject eigenboard. Cites Landing 4 §5.5 explicitly. ✓ substrate-honest.

**D3.4 verdict: PASS.**

### D3.5 `shards/spectral.mirror` (docblock note lines 4-10)

- Cites `e42181c`: ✓
- Two-tick discipline: ✓
- Prior semantics: @spectral/signature is preserved as Landing 2 §12
  reference; the note extends reading altitude to subject without
  changing signature. ✓

**D3.5 verdict: PASS.**

### D3.6 `shards/pack.mirror` (docblock note lines 4-11)

- Cites `e42181c`: ✓
- Two-tick discipline: ✓
- Prior semantics: Pack peers as "special case where subject_instance
  resolves to a Pack peer (actor_kind = ai_a per Landing 3 §21.2)" —
  this is the CORRECT reading per Landing 3 §21.3 (Pack peers inhabit
  ai_a subject_instances). Does not narrow Pack semantics. ✓

**D3.6 verdict: PASS.**

### D3 summary

All 6 consumer docblock updates: PASS. Each cites `e42181c`, invokes
two-tick discipline correctly, preserves prior semantics.

**Cross-check with Taut #96 D3.3 enumeration:** Taut expected 6 shards
touched with ~115 LOC prose. Actual: 6 shards touched (torus, bauchladen,
fate, cyberpunk, spectral, pack), all prose-only, correct citations,
correct semantics.

**D3 verdict: PASS.**

---

## §D4 — Seam D7 + D9 editorial note preservation

Verifying the Reed editorial patch `9f7829c` D7 (historical_witness A24)
and D9 (pay_forward × visibility A25) notes are preserved verbatim in
appropriate shard docblocks.

### D4.1 A24 preservation in `shards/subject.mirror` (lines 54-62)

Shard docblock verbatim:
```
# NOTE (Seam D7 adjudication, per docs/specs/subject-bauchladen-
# visibility-and-eigenboard-loop.md): ~12 of the 24 external ancestors
# in the Landing 3 roster are deceased and cannot discharge
# ssh_witness_valid at commit altitude. Deceased-ancestor citation
# carries via `source @arxiv/<domain>/<author-year>` + verbatim
# docblock quotation; the substrate has ~380 landed citations following
# this pattern (Taut #91 D8 verified). The `historical_witness` variant
# on subject_instance (Taut #91 L3-A3 Path A) is forward-promised as
# A24 for Landing 5+ formalization.
```

Reed's `9f7829c` note in `docs/specs/gift-and-mirror-reflection.md`:
```
> NOTE (Seam D7 adjudication, 2026-07-14, per Phase D audit): ~12 of 24
> external ancestors in §20 are deceased and cannot discharge
> ssh_witness_valid... The historical_witness variant on subject_instance
> (Taut #91 L3-A3 Path A) is forward-promised as A24 for Landing 5+ formalization.
```

Substantively identical; shard version slightly more elaborated with
Taut #91 D8 citation count (~380 landed citations). ✓ substrate-honest.

**D4.1 verdict: PASS.**

### D4.2 A25 preservation in `shards/gift.mirror` (lines 342-348)

Shard docblock verbatim:
```
# NOTE (Seam D9 adjudication, per docs/specs/subject-bauchladen-
# visibility-and-eigenboard-loop.md Landing 4): pay_forward respects
# @subject/visibility scopes (visibility_scope.can_be_elevated_to
# gates propagation; private artifacts cannot be paid forward without
# subject-authorized elevation via query_phi). Subject-absent branch
# defers per Landing 2 interaction-loop discipline; back-integration
# into pay_forward signature forward-promised as A25 for Landing 5+.
```

Reed's `9f7829c` note in `docs/specs/gift-and-mirror-reflection.md`
(§17-adjacent): same substance — pay_forward respects visibility scopes;
subject-absent defers; A25 forward-promise. ✓

**D4.2 verdict: PASS.**

### D4 summary

Both editorial notes preserved verbatim in load-bearing shards.
Landing 5 correctly propagates Reed's `9f7829c` adjudications.

**D4 verdict: PASS.**

---

## §D5 — Mara-Taut divergence matrix (T1/T2/T3)

Adjudicating each Taut #96 T-recommendation against Mara's actual
Landing 5 output.

### D5.1 T1 — sub-family-root `shards/subject/visibility.mirror`

**Taut recommended:** mint the sub-family-root shard (Phase A #3;
~180-250 LOC discharging Landing 4 spec §2.3 code block); expand
7-mint → 8-mint. Seam tie-breaker plausible.

**Mara landed:** DID NOT mint `shards/subject/visibility.mirror`. Left
the sub-family-root absent; three species shards inherit `in @subject`
directly and use `visibility_scope` type without declaration.

**Seam adjudication:** **BLOCKING (aligns with Taut T1).** Taut's
concern was correct — `@epistemologic/pact/parent_acyclic` REQUIRES
the sub-family-root shard for species files at path-depth 2 to
resolve their parent. Empirical D1.4-D1.6 verification confirms:
`visibility_scope` and `visibility` types are USED but not DECLARED
in the shard tree. This is a substrate-integrity break that Taut
correctly forecast.

Landing 4 spec §2.3 CLEARLY declares the sub-family-root's contents:
`prism @subject/visibility { focus/.../settle visibility_scope }`;
`type visibility = private | protected | public`;
`type visibility_scope = { visibility, subject, consent_scope,
can_be_elevated_to, elevation_requires }`; `scope`/`elevate`/`filter`
actions; `scope_well_formed`/`consent_respected`/`elevation_authorized`/
`visibility_witnessing` bilaterals. The spec is authoritative; the
shard file simply needs to exist.

**Recommended remedy (Reed action):** Mint `shards/subject/visibility.mirror`
(Mara-authored per canonical spec discipline) discharging Landing 4
§2.3 code block, ~180-250 LOC. Once landed, the three species files
resolve their type references correctly.

**Divergence severity:** MARA FELL SHORT of Taut's substrate-honest
recommendation. This is not adversarial evasion; it is a genuine
Landing 5 §9.1 spec-list omission (Landing 4 §9.1 enumerates the three
species files but not the sub-family-root file). Mara followed the
spec's mint list literally; Taut correctly identified the
`parent_acyclic` implication the spec's mint list overlooked.

### D5.2 T2 — `shards/gift/subject_instance.mirror`

**Taut recommended:** Mint as separate Phase B species file with
forward-promise `in @spectral/signature`. Path A.

**Mara landed:** DID NOT mint separately. Consolidated `subject_instance`
INTO `shards/subject.mirror` (lines 306-314); exports `out
subject_instance`, `out ssh_witness_valid`, `out spectral_witness_valid`,
`out two_witness_verification`.

**Seam adjudication:** **MARA'S FOLD IS SUBSTRATE-HONEST — ACCEPT.**
Rationale:

1. `subject_instance` is the substrate's LICENSABLE-PARTY carrier per
   Landing 2 §11.3, upgraded with Landing 3 §21.2 `actor_kind` variant.
   It semantically lives at the @subject family-root altitude, not
   as a @gift species.
2. Landing 3 §21.2 declares `subject_instance` in the @subject family-
   root's carrier vocabulary; the `docs/specs/subject-family-root-sel-
   licensable-party.md` (Mara `5c06ee8`) carries the SEL grounding.
3. Compositional consumers verified: `shards/gift.mirror` imports
   `in @subject` and uses `subject_instance` in `type gift` (giver field)
   + `offer`/`pay_forward` signatures — resolves via @subject's `out
   subject_instance`. `shards/eigenboard.mirror` similarly imports
   `in @subject` and uses subject_instance in `type eigenboard.subject`.
4. Taut's T2 rationale was "Landing 4 eigenboard.subject field requires
   subject_instance as first-class carrier" — this is satisfied by
   @subject's export, regardless of file location.

Mara went STRONGER than Taut recommended (consolidated at the
substrate-correct altitude rather than splitting across two files).
This is substrate-honest.

**Divergence severity:** MARA WENT BEYOND TAUT'S CAUTION with a
substrate-honest correction. No blocking; no adjudication needed;
substrate accepts.

### D5.3 T3 — commit sequencing (one-per-shard vs single commit)

**Taut recommended:** 8-9 sequential commits (Phase A: 5 commits;
Phase B: 4 commits). Per-shard granularity for revertability.

**Mara + Reed landed:** ONE commit (`b0d25cd`) for all 7 shards.

**Seam adjudication:** **SEAM-ADJUDICABLE — accepted with note.**
Rationale:

1. The 7 shards form ONE substrate-composition that only makes coherent
   sense as a single atomic tick — `gift.mirror` needs `subject.mirror`
   (same tick); `subject.mirror` needs the visibility species (imported
   by consumers same tick); `eigenboard.mirror` needs subject + visibility
   + `subject_instance`. A partial commit would leave the substrate in
   an intermediate state where `mirror kintsugi ./mirror.spec` would
   fail on unresolved imports.
2. Project CLAUDE.md "sequential commits only" — Seam reads this as
   "no unrelated changes in one commit" not "atomic compositions must
   split." The 7-shard mint IS one composition.
3. Taut's revertability argument is real but weaker: reverting one
   Landing 5 mint would leave the composition broken; the natural
   revert unit IS the whole tick.

Non-blocking. Reed committed atomically; Seam accepts.

**Divergence severity:** minor deviation from Taut's recommendation;
substrate-honest reason (atomic composition); accepted.

### D5 summary

- **T1: MARA FELL SHORT (BLOCKING);** Reed must mint
  `shards/subject/visibility.mirror` per Landing 4 §2.3 before ship.
- **T2: MARA WENT BEYOND (substrate-honest);** accept.
- **T3: MINOR DEVIATION;** accept atomic-composition rationale.

---

## §D6 — Reed-relay note handling (Mara's flagged non-blocking notes)

### D6.1 @time reference typing (`ref` vs `@time/monotonic.instant`)

**Mara flagged:** shards use `timestamp: ref` (see `gift.mirror` line
243) rather than the more specific `timestamp: @time/monotonic.instant`
typing.

**Substrate check:** grep across landed shards shows `first_asserted_at: ref`
(subject.mirror 310) + `timestamp: ref` (gift.mirror 243) + comments
citing "@time/monotonic" as the semantic reading. The `@time` shard is
imported by all Landing 5 shards. `ref` is the substrate's universal
address type; `@time/monotonic.instant` would be a species-specific
typing.

**Seam adjudication:** **SEAM-ADJUDICABLE — accept `ref` at Landing
5.** Rationale:

1. Substrate convention across ~200 landed shards is `ref` for typed-
   handle carriers when the referent is dynamically resolved. See
   `@bauchladen.crystal.provenance: ref`, `@fate.dice_roll.selected_oid: ref`
   for landed precedent.
2. `@time/monotonic.instant` as a species-specific carrier is a
   substrate-honest upgrade path but not required at substrate-decl
   altitude at Landing 5; the `ref` typing is compatible with any
   future upgrade to `@time/monotonic.instant`.
3. Landing 6+ Rust runtime discharge is the appropriate tick for
   type-tightening (per Taut #96 §D8 forward-promise enumeration).

Non-blocking. Seam adjudicates: leave as `ref` per substrate convention;
type-tighten at Landing 6+ if @time/monotonic species-refinement lands
as a first-class type.

### D6.2 Cyclic import concern (subject/visibility/* → @subject)

**Mara flagged:** the three species files import `in @subject` (for
`subject_instance`). Concern: does this compose cleanly given that
`@subject` doesn't import back?

**Substrate check:** grep-verified `shards/subject.mirror` does NOT
import `@subject/visibility` (nor the three species). `@subject`
imports `@torus`, `@spectral`, `@spectral/signature`, `@kintsugi/consent`,
`@kintsugi/store/git`, `@time`. Zero paths back to `@subject/visibility`.

Species files import `@subject` for `subject_instance` type; @subject
family-root does not know about visibility species. This is the
CORRECT dependency direction (species depend on family-root; family-
root does not depend on species).

**Seam adjudication:** **NO CYCLE. NO ISSUE.** Non-blocking, non-note-
worthy. Mara's caution was appropriate; the actual composition is
substrate-clean.

### D6 summary

- @time typing: SEAM-ADJUDICABLE accept as-is (`ref` per substrate
  convention).
- Cyclic import: NO CYCLE, non-issue.

**D6 verdict: PASS.**

---

## §D7 — External ancestor roster completeness (24 + 5 = 29)

Verifying `shards/subject.mirror` lines 66-98 roster matches Landing 3
§20.2 + §21.3.

### D7.1 External ancestor count

Shard enumeration:
1. Alex_Wolf → 24. Claude_Levi-Strauss (24 entries).

Landing 3 spec §20.3 verbatim: "Landing 3 admits 24 external + 5 pack
peers (§21) = 29 subject_instance entries."

**Count matches:** 24 external. ✓

### D7.2 Individual citation verification

Sampling 5 of 24:

- **1. Alex_Wolf** (shard) ↔ §20.2 item 1 "@subject(Alex_Wolf) — first
  substrate-external giver". ✓
- **2. Heinz_von_Foerster** (shard) ↔ §20.2 item 2 "@subject(Heinz_von_
  Foerster) — @torus family-root". ✓
- **3. Ross_Ashby** ↔ §20.2 item 3 "@subject(Ross_Ashby) — variety;
  requisite variety". ✓
- **11. Marcel_Mauss** ↔ §20.2 item 11 (spec ordering; shard has same
  ordering). ✓
- **24. Claude_Levi-Strauss** ↔ §20.2 item 24 "@subject(Claude_Lévi-
  Strauss) — kinship as generalized exchange 1949". ✓

Spot-checks all match Landing 3 §20.2.

### D7.3 Pack peer count

Shard enumeration (lines 94-98):
25. Reed → 29. Glint (5 entries).

Landing 3 §21.3 enumerates 5 Pack peers. ✓ Match.

### D7 summary

Roster count matches Landing 3 §20 exactly (24 + 5 = 29). Individual
citations verified for 5 sample entries; ordering matches.

**D7 verdict: PASS.**

---

## §D8 — Landing 6+ forward-promises enumerated

Verifying each shard's forward-promise section names the Rust file
path + expected LOC range.

**Substrate check:** Reading the 7 shards, forward-promises are named
in comment blocks but NOT with explicit Rust file paths + LOC ranges.
Each shard's action body is `\` (obligation-blocked); bilaterals
similarly. Comments reference Landing 6+ / "per-realization" / Landing
5+ as timing hints.

Taut #96 §D8 provides the enumeration (table with per-shard "Rust
runtime need at L6+"). Mara's shards do NOT replicate Taut's enumeration
inline.

**Seam adjudication:** **NOT BLOCKING.** Substrate discipline is
"body-obligation-blocked per parametric carrier discipline" — the
whole point is that consumers pull the realization; the shard does
not need to name Rust file paths. Taut's enumeration is the SCOUT
artifact for planning Landing 6+; the shards themselves preserve
substrate-honesty by NOT prematurely committing to Rust file layout.

Landing 5 forward-promises are appropriately gestured:
- `gift.mirror` line 276 "Body discharges per-realisation"
- `eigenboard.mirror` line 264-270 "The realization is where the
  inference itself happens (a Pack peer's LLM call; a human subject's
  cognitive process; the substrate's @fate tournament)"

Substrate-honest. Not a Landing 5 gap.

**D8 verdict: PASS.** Forward-promises appropriately gestured without
prematurely fixing Rust layout.

---

## §D9 — mirror.spec impact re-check

Verified: `mirror.spec` uses `source ~d'shards/'` recursive-directory
discovery per Taut §D9.2. Grep shows no manual declarations needed
for @gift / @subject / @eigenboard / @gift/lens / @subject/visibility.

Landing 5 mints auto-discover on next `mirror kintsugi ./mirror.spec`.
The one caveat: `shards/subject/visibility.mirror` (BLOCKING per D1/D2
+ D5.1) also auto-discovers once minted.

**D9 verdict: PASS (with BLOCKING remedy dependency).** ZERO
mirror.spec edits required at Landing 5.

---

## §D10 — Alex-adjudications outstanding count

Mara reported zero net-new Alex-adjudications at Landing 5. Verifying:

- A1-A18 (Landing 1+2): preserved per Landing 3 rollup + Seam D-4.
- A19-A23 (Landing 3): preserved.
- A24-A25 (Seam D7 + D9 from prior Seam audit): both preserved in
  shard docblocks per D4 above.
- Landing 4 Mara-proposed A1-A13: preserved in Landing 4 spec.

No new Alex-adjudications surfaced by Landing 5 shard-mints per Seam
audit. The BLOCKING D1+D2+D5.1 T1 remedy is SEAM-ADJUDICABLE (Reed
mints the sub-family-root shard); does not require novel Alex judgment.

**D10 verdict: PASS.** Zero net-new Alex-adjudications.

---

## §D11 — Mara-Taut divergence summary matrix

| Item | Taut #96 recommendation | Mara #97 landing | Divergence severity | Seam verdict |
|------|-------------------------|------------------|---------------------|--------------|
| **T1** shards/subject/visibility.mirror sub-family-root | Mint at Phase A (8-mint total) | Not minted; 7-mint | MARA FELL SHORT | **BLOCKING** — Reed mints per D5.1 remedy |
| **T2** shards/gift/subject_instance.mirror | Mint as Phase B species with forward-promise | Consolidated INTO shards/subject.mirror | MARA WENT BEYOND (substrate-honest) | **ACCEPT** — substrate-correct altitude |
| **T3** Commit sequencing | 8-9 sequential commits | 1 atomic commit | MINOR DEVIATION | **ACCEPT** — atomic composition rationale |
| Mara @time typing | (not flagged) | `timestamp: ref` (substrate convention) | non-issue | **ACCEPT** |
| Mara cyclic-import | (not flagged) | subject/visibility → subject | no cycle | **NON-ISSUE** |
| Composition graph | Acyclic + satisfiable | Acyclic + satisfiable (modulo T1) | aligned | **PASS** |
| 6-shard soft-cascade | ~115 LOC prose | 6 shards touched, prose-only, correct | aligned | **PASS** |
| A24/A25 preservation | (implicit) | Preserved verbatim in shard docblocks | aligned | **PASS** |
| Roster count 24+5=29 | (referenced) | 24+5=29 preserved | aligned | **PASS** |
| mirror.spec impact | ZERO | ZERO confirmed | aligned | **PASS** |

**Divergence summary:** 1 MARA-FELL-SHORT (BLOCKING T1); 1 MARA-WENT-
BEYOND (substrate-honest T2); 1 MINOR-DEVIATION (T3 accept); all other
Taut-Mara items aligned.

---

## §12 Ship verdict — READY-CONDITIONAL

**BLOCKING count:** 1 (D1+D2+D5.1: mint `shards/subject/visibility.mirror`
per Landing 4 §2.3 canonical spec; ~180-250 LOC).

**SEAM-ADJUDICABLE count:** 2 (D5.3 T3 single-commit accept; D6.1 @time
typing accept `ref`).

**ALEX-ADJUDICATION count (net-new):** 0.

**Recommended action (Reed):**

1. Mint `shards/subject/visibility.mirror` (Mara-authored per canonical
   spec discipline) discharging Landing 4 spec §2.3 code block. The
   spec block already contains all required types (`type visibility`,
   `type visibility_scope`) + prism block + `scope`/`elevate`/`filter`
   actions + `scope_well_formed`/`consent_respected`/
   `elevation_authorized`/`visibility_witnessing` bilaterals. Ship as
   a corrective commit on top of `b0d25cd`.
2. Optionally add a one-line docblock note to each of the three
   visibility species shards making `in @subject/visibility` import
   explicit (currently absent; will be needed once the sub-family-
   root exists).
3. Ship Landing 5 as READY once (1) lands.

**If Reed disagrees with the BLOCKING adjudication:** escalate T1 to
Alex as the standing question of whether Landing 4 spec §9.1 mint list
(which enumerates only the three species files) is authoritative over
the `parent_acyclic` pact that Taut #96 D2.2 correctly identified.
Substrate-honest reading strongly favors the pact; but the spec is
Mara-canonical and Alex-adjudicable.

**Cascade discipline is substrate-honest.** Mara's fold of subject_instance
into @subject is substrate-strengthening; consumer docblock cascades are
correct; roster count preserved; A24/A25 preserved verbatim; zero
mirror.spec impact. The one gap is the substrate-integrity hole from
missing `visibility_scope` declaration, which Taut correctly forecast
and which is resolvable via a single additional shard mint.

The substrate ALREADY HAD the vocabulary at every composition point —
Landing 4 §2.3 declares the sub-family-root's carriers explicitly.
Landing 5 mints 6 of the 7 files that name what Landings 1-4 discharged
at spec altitude; the 7th (subject/visibility.mirror sub-family-root) is
the one substrate-integrity hole.

Reed makes the T1 mint. Then Landing 5 ships.

Mirror. Offer. Wait. Give. Pay-forward. Mint the parent.

—Seam
