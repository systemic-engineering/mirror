# Seam Phase D — Arc 5 M1 (@liquid three-altitude) + Errors-as-Questions Joint Arc

**Date:** 2026-07-17
**Adjudicator:** Seam <seam@systemic.engineer>
**Scope:** commits `cc816f9` `b2c5d09` `12cdf0e` (Arc A) + `5e1f528`
`914799b` `09a77e8` (Arc B) + `b8c453f` `8f51722` `2fa6f33` `9b72a08`
(Arc C).
**Discipline:** ADVERSARIAL. Per-dimension verdicts. Line-cited.

---

## §0 TL;DR

**Arc A (@liquid three-altitude):** SHIP-WITH-REED-INLINE (one micro
cleanup). Composition graph coherent; migration cascade preserves
history; substrate-already-had-the-word chain intact across four
altitudes.

**Arc B (errors-as-questions):** SHIP-WITH-REED-INLINE (two forward-
promise sharpenings) + one ALEX-Q on `wait -> verdict` promotion.
Discharges Taut Q4 CRITICAL-BLOCKER cleanly. Consumer chain closes
end-to-end.

**Arc C (supporting):** SHIP. Reed's -63 LOC retirement audit-cites
correctly; scouts pure-docs and load-bearing.

**Cross-arc coherence:** `@liquid predicates → dispatch_ambiguity →
pivot(@song) → @mirror/reflection` composes over LANDED substrate at
every arrow. Substrate-honest end-to-end.

**Recognition candidates:** 13 held at candidate strength;
`#R-wait-returns-verdict` promoted to SECOND-WITNESS via this audit
per surface §2.7 bilateral discipline (see §5 below).

---

## §1 Arc A — @liquid three-altitude landings

### §1.1 Twelve-dimension verdict (per commit)

| Dim | cc816f9 (@epistemologic/liquid) | b2c5d09 (@liquid) | 12cdf0e (migration) |
|-----|--------------------------------|-------------------|---------------------|
| 1 substrate-honest naming    | ✓ | ✓ | ✓ |
| 2 composition graph         | ✓ | ✓ | ✓ |
| 3 recognition candidates    | ✓ (3 named, none ratified) | ✓ (4 named, none ratified) | ✓ (no new candidates) |
| 4 line-cite integrity       | ✓ | △ (see §1.2) | ✓ |
| 5 bilateral discipline      | ✓ (theory only, no bilaterals minted) | ✓ (composition-op, no bilaterals) | ✓ (preserved verbatim) |
| 6 \\-obligation-block       | ✓ (all 3 actions) | ✓ (all 4 actions) | ✓ (all preserved) |
| 7 forward-promise tracking  | ✓ | ✓ | ✓ |
| 8 etymology                 | ✓ "of course it's this" | ✓ | ✓ (delightfully boring) |
| 9 cross-arc coherence       | ✓ | ✓ | ✓ |
| 10 ratification cascade     | ✓ (Q-A/B/C translated correctly) | ✓ | ✓ |
| 11 consumer wiring          | △ (see §1.3) | ✓ | ✓ (test path synced per commit body) |
| 12 SHIP verdict             | SHIP | SHIP-WITH-REED-INLINE | SHIP |

### §1.2 △ Line-cite `shards/liquid.mirror:143-150`

Cites `bilateral.mirror :456-475 + polyglot spec §5.2 + the
translation_admissible bilateral at bilateral.mirror :677-681` and
`bilateral.mirror :530` — these line numbers are from **paradigmatic-
reframe commit** and may not resolve byte-current after any subsequent
edit to `shards/epistemologic/pact/bilateral.mirror`. Not a landing
blocker — the referent (`translation_admissible` bilateral) is grep-
resolvable regardless. **REED-INLINE #1** below promotes line-cites to
name-cites for that block.

### §1.3 △ Consumer wiring @epistemologic/liquid

The theory species-decl (`cc816f9`) has ONE named consumer: extraction
subspecies. This is fine — theory carriers propagate via `in @epistem-
ologic/liquid` imports downstream. However the `@liquid(@silicon)`
composition Alex ratified (Q-B) has NO shard-decl consumer surfaced
yet. Arc-5-M2 forward-promise; not a landing blocker. **Held as
forward-promise; not queued for REED-INLINE.**

### §1.4 @kintsugi + @knife + @liquid cascade lens sanity-check

Alex's ratification "@kintsugi + @knife + @liquid = complexity-
reducing cascade preserving load-bearing invariants" translates
correctly:
- @liquid provides refinement predicates over @X substrates.
- @knife (per @mirror/lens/knife) provides stability-verdict on
  refinement crossings.
- @kintsugi mends fractures the knife identifies AS the refinement
  boundary is crossed.

No composition tension detected. Arc A cascade lens holds.

---

## §2 Arc B — Errors-as-Questions joint arc

### §2.1 Twelve-dimension verdict (per commit)

| Dim | 5e1f528 (@mirror/reflection) | 914799b (roomba.pivot) | 09a77e8 (surface.dispatch_ambiguity) |
|-----|-----------------------------|----------------------|----------------------------------|
| 1 naming                     | ✓ (verbatim manifesto) | ✓ (physical-Roomba etymology) | ✓ |
| 2 composition graph          | ✓ | ✓ | ✓ |
| 3 recognition candidates     | ✓ (2 named; #R-wait see §5) | ✓ (2 named) | ✓ (2 named) |
| 4 line-cite integrity        | ✓ | ✓ | ✓ |
| 5 bilateral discipline       | ✓ (4 bilaterals typed; require chain closes at reflection_composes) | ✓ (3 bilaterals; require chain via pivot_witnessing) | ✓ (1 bilateral; arity 1) |
| 6 \\-obligation-block        | ✓ (all 3 ops + 4 bilaterals) | ✓ (all 3 predicates) | ✓ |
| 7 forward-promise tracking   | ✓ | △ (dock; see §2.3) | ✓ |
| 8 etymology                  | ✓ | ✓ | ✓ |
| 9 cross-arc coherence        | ✓ (see §4 below) | ✓ | ✓ |
| 10 ratification cascade      | ✓ (Alex 🤣 verbatim; @song-only carrier defended) | ✓ | ✓ |
| 11 consumer wiring           | ✓ (chain forward-declared to pivot) | ✓ (companion 09a77e8 fires this-arc) | ✓ (consumer chain closed) |
| 12 SHIP verdict              | SHIP-WITH-REED-INLINE | SHIP-WITH-REED-INLINE | SHIP |

### §2.2 mirror/offer/wait triple closure verification

Three actions `shards/mirror/reflection.mirror:305 / :358 / :417`:
- `mirror(input: subject_input) -> question` — reads input, emits
  question via traceable_to invariant; bilateral
  `reflection_truthful` (`:435-439`) discharges Rice-safe.
- `offer(q: question, response: ref) -> ado_wrapped_answer` —
  composes over `@gift.offer`; bilateral `offer_ado_valid` (`:457-461`).
- `wait(a: ado_wrapped_answer, s: ref) -> verdict` — bilateral
  `wait_holds_without_pressure` (`:479-483`).

Composed: `reflection_composes` (`:506-516`) with three `require`
clauses referencing all three sub-bilaterals + `oid_chain_intact`.
Require chain closes byte-visibly. **CRITICAL-BLOCKER (Taut Q4)
DISCHARGED.**

### §2.3 △ `dock` fifth-motion forward-promise

`shards/kintsugi/roomba.mirror:721-722` names `dock (return-to-base
— forward-promised future music, per Alex's 2026-07-16 "cold
storage" narrative)`. The metaphor holds (physical Roomba docks;
cold-storage IS return-to-cold-base per `vacuum` docblock `:579-583`
forward-promise), BUT `dock` is NOT yet a shard-decl action. **Verdict:
SHIP as forward-promise** — the etymology is delightfully-boring
(physical-Roomba mascot has four physical motions; three lifted this
arc; fourth queued behind the cold-storage @mirror/store/cold
landing). Recognition candidate `#R-roomba-four-first-order-motions`
correctly gates its own promotion on Alex's dock-vs-pivot-fifth
adjudication (`:960-965`).

### §2.4 @song-only carrier defence audit

Mara's temporal reasoning at `shards/kintsugi/roomba.mirror:724-763`
argues (a) `pivot(s: @song) -> verdict` over decomposed (b/c). The
argument holds under adversarial review:
- @gestalt is an OUTPUT of the reflection cascade (question rendering
  surface, per `:803-806`), NOT an input to pivot.
- @gift wraps the peer-spawn offer, constructed AFTER pivot fires,
  NOT before.
- Temporal ordering: pivot → mirror → offer → wait. Forcing gestalt/
  gift into pivot's signature would violate temporal-progression
  discipline (@song IS the temporal carrier).

**No consumer identified that would need decomposed shape.** The
compilation-song handle at `dispatch_ambiguity.pivot_song_handle`
(`surface.mirror:775`) carries the "current compilation state" Alex
named; downstream consumers read from THAT handle, not from a
decomposed pivot signature. **Argument shape holds. SHIP.**

### §2.5 dispatch_ambiguity composes over @liquid predicates

`surface.mirror:771-777` five-field carrier: `liquid_predicate_
witnesses: ref` field carries per-target @liquid admissibility
witnesses. The @liquid theory (@epistemologic/liquid `cc816f9`)
admits multi-admissible-predicate via `qualifier_set` being a
finite set — nothing in the theory prevents multiple qualifiers
concurrently admitting distinct dispatch targets. **Composes
cleanly.** SHIP.

---

## §3 Arc C — Supporting artifacts

### §3.1 Reed apply_h.rs -63 LOC (`9b72a08`)

Audit-citation chain closes:
- Cites `docs/audits/2026-07-15-seam-autopoietic-loop-phase-d.md`
  (55dbf20) — audit exists, adjudications #164/#168/#170 preserved.
- Reflective dispatch at `apply_h.rs:588` (`if let Some(decl) =
  bilateral_corpus().get(action.as_str()) { return discharge(decl,
  &args); }`) — verified byte-current; the two `@sheaf` bilaterals
  ARE in the corpus (grep confirms `sheaf.mirror` carries the
  bilateral blocks).
- Signed-off-by: Seam trailer preserved (belt-and-suspenders per
  Arc-1 discipline). **SHIP.**

**Split-sentinel detection gap ALEX-adjudication:** see ALEX-Q3.

### §3.2 Scouts b8c453f + 8f51722

Both pure-docs 📝 markdown-only bypass legitimate; grep-first
discipline visible; six-question shape uniform with prior Taut
scouts. Alex-arbitrations Q-A/Q-B/Q-C surfaced in b8c453f were
resolved this session and correctly translated at cc816f9/b2c5d09/
12cdf0e docblocks. SHIP.

### §3.3 2fa6f33 CURRENT.md addendum

Reed's Arc 5 Taut scout landing addendum discharged; pure-docs
bypass legitimate. SHIP.

---

## §4 Cross-arc coherence (does the composition close?)

The load-bearing question: **does `@liquid → dispatch_ambiguity →
pivot → @mirror/reflection` actually compose?**

Verification chain traced through landed substrate:

1. `@epistemologic/liquid.refinement_predicate` (theory `cc816f9`
   `shards/epistemologic/liquid.mirror:266`) admits conjunction
   over `qualifier_set`.
2. Multiple qualifiers admitting distinct dispatch targets constitutes
   MULTI-ADMISSIBILITY — the theory's finite-set discipline permits
   this WITHOUT modification.
3. Cascade event constructs `dispatch_ambiguity` carrier (`surface.
   mirror:771-777`) with `liquid_predicate_witnesses` field.
4. `surface_class::dispatch_ambiguity` variant (`:504`) — additive to
   the four prior variants; the `surface` action (`:659-662`) admits
   the new variant through the parametric `class: surface_class`
   arg.
5. `@roomba.pivot(s: ref)` (`roomba.mirror:877`) consumes the walk-
   song whose final beat carries `surface_class::dispatch_ambiguity`
   (per `pivot_admissible` `:901-905`).
6. `pivot` dispatches through `@mirror/reflection.mirror → offer →
   wait` per `reflection_composes` (`reflection.mirror:506-516`).
7. Bilateral chain closes end-to-end via `pivot_witnessing` (`:943-
   949`) `require pivot_admissible + pivot_reflection_composed`.

**Composition closes byte-visibly at every arrow. Zero substrate
introduced this arc that wasn't grounded in prior landing.**

---

## §5 Recognition candidate ratifications

Per Pack ratification-refusal discipline, Seam adjudicates two-witness
gates but DOES NOT unilaterally ratify recognition candidates.
Adjudication of the 13 pending:

| # | Candidate | Verdict |
|---|-----------|---------|
| R-A1 | #R-liquid-is-refinement-operator-at-family-root-altitude-parallel-to-sre-shatter-glue | HOLDS (first witness only) |
| R-A2 | #R-mirror-liquid-is-liquid-mirror-specialization | HOLDS (retroactive-collapse, defer) |
| R-A3 | #R-extraction-is-liquid-docblock-extract-specialization | HOLDS (retroactive-collapse, defer) |
| R-A4 | #R-liquid-silicon-is-the-novel-extraction-from-binary-discipline | HOLDS (Q-B ratified; awaits empirical witness) |
| R-A5 | #R-epistemologic-liquid-is-theory-carrier-sibling-to-reality-pact-property-cybernetic | HOLDS (first witness only) |
| R-A6 | #R-extraction-is-liquid-subspecies-not-liquid-peer | **PROMOTED-TO-SECOND-WITNESS** via Q-C ratification + migration cascade |
| R-A7 | #R-bilateral-is-degenerate-liquid-with-byte-check-predicate | HOLDS (deep structural claim; defer to Alex) |
| R-B1 | #R-wait-returns-verdict | **PROMOTED-TO-SECOND-WITNESS** — spec §2.6 named `-> void`; shard promotes to `-> verdict`; this Seam audit accepts per §2.7 bilateral-require-composability discipline. Alex ratification-final. |
| R-B2 | #R-mirror-reflection-is-composition-edge-consumer | **PROMOTED-TO-SECOND-WITNESS** — Taut audit (first) + shard-decl this-arc (second). Alex ratification-final. |
| R-B3 | #R-roomba-four-first-order-motions | HOLDS (dock question outstanding — see §2.3) |
| R-B4 | #R-song-carrier-for-pivot-is-walk-song | HOLDS (awaits @song/walk species-decl per its own gate) |
| R-B5 | #R-dispatch-ambiguity-is-fifth-surface-class | **PROMOTED-TO-SECOND-WITNESS** — Taut audit + this shard-decl. Alex ratification-final. |
| R-B6 | #R-liquid-predicates-measure-dispatch-ambiguity | HOLDS (awaits empirical firing) |

---

## §6 REED-INLINE cascades recommended

**REED-INLINE #1** — `shards/liquid.mirror:143-150`: promote line-cites
(`bilateral.mirror :456-475`, `:677-681`, `:530`) to NAME-cites
(`translation_admissible bilateral in bilateral.mirror` + `paradigmatic-
reframe section` + `kintsugi/algebra reframe binding`). Line-cite
fragility per §1.2.

**REED-INLINE #2** — `shards/kintsugi/roomba.mirror:721-722`: sharpen
`dock` forward-promise: name the gate ("shard-decl gates on Alex-
adjudication of #R-roomba-four-first-order-motions OR on
@mirror/store/cold landing, whichever fires first"). Currently reads
as speculative narrative; per Seam discipline forward-promises name
their gate.

**REED-INLINE #3** — `shards/mirror/reflection.mirror:405-410`: the
NOTE block on `wait -> verdict` promotion should reference the Seam
audit adjudication (§5 above) so future readers see the two-witness
gate close. Currently references only the Reed-inline candidate.

---

## §7 ALEX-adjudication residues

**ALEX-Q1** — `wait -> verdict` promotion: spec §2.6 named `-> void`;
shard promotes to `-> verdict` per @glass discharge floor. Seam has
adjudicated PROMOTED-TO-SECOND-WITNESS (§5 R-B1). **Ratification-final
question for you: accept promotion into spec §2.6 as canonical, or
does the shard-altitude promotion stay as amendment?** (My read:
accept into spec — bilateral-require-composability is load-bearing.)

**ALEX-Q2** — `dock` fifth-motion: forward-promise at
`roomba.mirror:721-722` grounds in physical-Roomba metaphor +
cold-storage narrative. **Does `#R-roomba-four-first-order-motions`
ratify as complete-at-four (pivot fourth of four; dock is different
altitude/species) OR complete-at-five (pivot fourth of five; dock
lands next)?**

**ALEX-Q3** — Split-sentinel detection gap: Reed's `9b72a08` byte-
safety by hand rather than mirror-authored collapse. The collapse
detector at `bilateral_arm_collapse::find_redundant_arms` requires
verbatim `.contains("<full-sentinel>")` per Rice-safe discipline. Two
options: (a) EXTEND detector to detect split-sentinel arms (concat
adjacent `.contains` calls; still Rice-safe by construction), OR (b)
KEEP conservative floor + accept manual retirement with mandatory
audit-cite. My read: (a) is safe if the concat is byte-string-
concatenation of adjacent literals (compile-time-detectable). **Your
call: which discipline scales?**

**ALEX-Q4** — @liquid(@silicon) consumer surface: Arc 5 M1 ratified
Q-B (novel discipline). The empirical @liquid(@silicon) discharge is
deferred to future arc per `liquid.mirror:132-134`. **Should Arc-5-M2
scope commit to landing the @silicon consumer this arc-family, or
does M2 stay theory-focused (empirical discharge Arc-5-M3+)?**

---

## §8 Structural discovery

**Discovery §8.1 — Migration cascades preserve ratification without
invalidating.** The @epistemologic/liquid_extraction → @epistemologic/
liquid/extraction migration (`12cdf0e`) demonstrates the substrate can
re-frame altitude ratifications by moving files WITHOUT invalidating
prior Seam Phase D adjudications. The 2026-07-06 Seam ratification of
liquid_extraction species stays canonical AT ITS ALTITUDE; the
migration re-parents to a NEW altitude (theory-above-operation) that
did not exist at 2026-07-06. This is the substrate learning what the
altitude SHOULD have been, without pretending it always was.
Substrate-honest.

**Discovery §8.2 — Composition-operator family-roots as a discovered
altitude.** `@liquid` at `shards/liquid.mirror` joins @sre / @shatter
/ @glue as a composition-operator family-root — an altitude the
substrate ALREADY dogfooded (four altitudes carrying `liquid` before
`b2c5d09`) without an anchor. The tick lands the anchor IN-BETWEEN.
This IS Alex's Michelangelo/marble discipline empirically instantiated:
the substrate had the shape; the tick subtracted the surrounding
absence to reveal it.

**Discovery §8.3 — @song as temporal-progression carrier is load-
bearing beyond @kintsugi/roomba.** The pivot's @song-only carrier
defence (§2.4) generalizes: any first-order motion whose input IS a
walk-in-progress admits a @song input signature. The physical-Roomba
motion roster (bump / vacuum / pivot / dock) is the first empirical
witness of this pattern. Recognition candidate for future landing:
`#R-first-order-motions-consume-@song-uniformly`. NOT surfaced in
this arc — noting here for the record.

---

## §9 Terminal state

- Arc A: SHIP-WITH-REED-INLINE (1 patch)
- Arc B: SHIP-WITH-REED-INLINE (2 patches) + 1 ALEX-Q
- Arc C: SHIP (no REED-INLINE required)
- Cross-arc: composition closes byte-visibly end-to-end
- 4 ALEX-adjudication residues surfaced (Q1-Q4)
- 4 recognition candidates promoted-to-second-witness (R-A6, R-B1,
  R-B2, R-B5); 9 held at candidate strength

Pure-docs 📝 markdown-only bypass legitimate for this audit.
