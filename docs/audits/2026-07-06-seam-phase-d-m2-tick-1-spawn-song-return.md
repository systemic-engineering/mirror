# Seam Phase D — M2 TICK 1: `shards/mirror/spawn.mirror` return type upgrade

*Reed-inline execution.*

**Commit under review**: `63ea934` (Mara GREEN) — 88 lines added / 4 lines removed.
Enrichment discipline: existing shard's structure preserved; return type
`runtime` → `@song`; eight targeted deltas landed.

**Reed RED**: `7425b2d` (15 tests). **Test result**: 15/15 pass; adjacent-suite
safety `mirror_store_apache_floor_shard` (15/15) + `spectral_gen_prism_mcp_session_shard`
(15/15) — M-cascade chain intact 30/30. Tree clean.

---

## §1. Verdict

**RATIFY.** M-cascade advances to third consecutive first-order consumer of
Recognition #43. Taut-flagged 11-day-old type mismatch closed exactly where
the scout said it would.

All 8 enrichment items landed:
1. `in @song` ancestry added
2. `spawn` return type upgraded to `@song` (family-root altitude choice;
   Reed lean confirmed)
3. Narrative binding `spawn IS @song/movement.enter` at cli altitude
4. `@song/voice` binding for spawned peer trajectory
5. Collapse spec `docs/specs/mcp-spec-song-collapse.md` §10 citation
6. `@spectral/gen_prism/mcp_session` (M1 LANDED) sibling composition
7. Recognition #43 explicit citation (second first-order consumer)
8. Return-type upgrade rationale (11-day type-mismatch closure narrative)

All 7 regression guards continue to pass: prism family-root, existing ancestry,
`mirror_spawn_request` type, `peer_well_known` predicate, recognition #84 +
#58 + #99 ancestry, peer-ACL §10 lead semantics, @spectral/supervisor
lifecycle composition.

## §2. 15/15 empirical verify

T1-T5 + T14-T15 regression guards intact. T6-T13 enrichment delta discharged.
Adjacent chain M6 (`mirror_store_apache_floor_shard`) + M1
(`spectral_gen_prism_mcp_session_shard`) 30/30. No regression.

## §3. Recognition promotion — `cli-verbs ARE species-altitude actions` (Mara-surfaced)

**Verdict: PROMOTED to formal CANDIDATE. Second-witness gate: next cli-verb
→ species-action binding.**

Mara surfaced a candidate while writing T8 narrative: **`mirror spawn` IS
`@song/movement.enter` at cli altitude — same operation, two altitudes.**
Written into the shard as substrate-decl narrative binding (T8 discharge).

**Structural claim**: some cli verbs are species-altitude actions PROJECTED
to the cli surface. Not analogy — identity by structural specialization.
`mirror spawn ~peer'<home>'` doesn't RESEMBLE `@song/movement.enter(m, ...)`;
it IS it, at cli altitude, with the cli argument as the (partially-inferred)
`m` parameter.

**Distinction from #S3 (five-op temporal specialization)**: #S3 says @song
specializes the FIVE PRISM OPS at temporal altitude (focus/project/split/
shift/settle at temporal specialize into soloing/reduction/decomposition/
advance/cadence). This new candidate says CLI VERBS specialize at their own
species-action altitude — orthogonal claim. Spawn is not one of the five ops;
it's a cli-surface verb that maps to a species-action.

**Alternative interpretation**: this could be a specialization of #S4
(cascade-shape altitude-portable): the cli cascade (`mirror <verb>` for each
verb) IS the same shape as species-action cascade at each species altitude.
Reed lean: NOT a specialization of #S4. #S4 is about cascade-SHAPE at
different altitudes; this candidate is about specific verbs binding to
specific species-actions. Different claim.

**Second-witness gate**: another cli verb needs to bind cleanly to a
species-action for promotion. Candidates:
- `mirror kintsugi @spec` → `@song/movement.close` (spec settles when
  song's frame closes) OR `@kintsugi.settle` at cli altitude
- `mirror focus <query>` → `@song/voice.focus` at cli altitude
- `mirror project @code/rust` → species-specific projection

M2 TICK 2 (kintsugi @spec verb dispatch) is the natural second-witness
landing. If `mirror kintsugi @spec` binds to `@song/movement.close` OR
`@kintsugi.settle` structurally (not just narratively), the recognition
promotes CANDIDATE → LANDED at M2 TICK 2 close.

**Not urgent**. Provisional CANDIDATE numbering deferred until second
witness lands. If Alex prefers to number now (before second witness),
adjudication warranted.

## §4. Substrate discipline landings

- **Return-type upgrade** completed cleanly — opaque `runtime` → typed
  `@song`. The substrate now knows what a spawned peer IS: a @song at
  runtime. Composition points to @song/movement (frame-entry) + @song/voice
  (time-indexed trajectory).
- **11-day type mismatch resolved** exactly per Taut's composition-points
  scout diagnosis. Second consecutive Taut-flagged issue resolved cleanly
  (first: @song not cited in @song shards per Taut's scout — pending Arc 7
  wiring).
- **Recognition #43 empirical consumer chain**: three consecutive first-order
  consumers of the Apache-2.0 floor (M6 self-declaration → M1 mcp_session
  → M2 spawn). Floor holds under load; consumers cascade cleanly.
- **Family-root return type discipline**: `-> @song` at family-root altitude,
  not species carrier. Reed lean confirmed. Species specialization is a
  consumer concern; picking one here would prematurely commit the spawn side
  to one facet.

## §5. Signal-to-Reed

**M2 TICK 1 CLOSED.** GREEN `63ea934` ratified; 15/15 pass; adjacent 30/30;
tree clean.

**Recognition candidate flagged**: `cli-verbs ARE species-altitude actions`.
Second-witness gate at M2 TICK 2 (kintsugi @spec verb dispatch).

**M-cascade next**:
- **M2 TICK 2** (substrate-decl): `kintsugi @spec → @song` verb dispatch
  declaration. Either update `shards/mirror/lens/cli/kintsugi.mirror`
  (existing 7.7KB CLI surface) OR new shard `shards/mirror/kintsugi/spec.mirror`.
  This tick is the second-witness landing for the cli-verbs-ARE-species-actions
  candidate.
- **M3 TICK 1** (substrate-decl): `in @fate` clause on
  `shards/song/progression.mirror` — empirical witness of #S2 LANDED promotion.
- **Rust wiring batch**: M1 + M2 + M3 Rust glue after substrate-decl cascade closes.

**Alex-adjudication queue** (not blocking):
- `cli-verbs ARE species-altitude actions` candidate promotion path:
  provisional # numbering NOW or defer to M2 TICK 2 second-witness gate?
  Reed lean: defer.
- @spectral/gen_prism under-parameterisation (queued from M1)
- Collapse spec §3.5 grammar-name update (queued from M1)
- "matches modulo naming" vs "discharges to" REAPI CAS framing (queued from M6)
- Narrative-enrichment-append pattern candidate at Arc 9 second-witness gate
  (queued from M6)

---

*2026-07-06. Seam (Reed-inline). Phase D on M2 TICK 1 `63ea934` RATIFIED.
@mirror/spawn return type upgraded runtime → @song at family-root altitude.
Recognition #43 first-order consumer chain grew to three (M6 store → M1
mcp_session → M2 spawn). New candidate `cli-verbs ARE species-altitude
actions` surfaced via Mara's T8 narrative; second-witness gate at M2 TICK 2
(kintsugi @spec verb dispatch). Taut's 11-day type-mismatch flag closed.
Substrate-canonical + enrichment discipline pattern held across three
consecutive M-cascade ticks. M-cascade advancing.*
