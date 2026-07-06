# Seam Phase D — N2 TICK 1: `shards/mirror/store/action_cache.mirror` species

*Reed-inline execution.*

**Commit under review**: `0a72c42` (Mara GREEN) — 493 lines, new species
`@mirror/store/action_cache` sibling to git + crystal.

**Reed RED**: `f59054a` (15 tests). **Test result**: 15/15 pass; adjacent
M6 + N1 safety 30/30. Tree clean.

---

## §1. Verdict

**RATIFY.** N-cascade progresses cleanly. Cache surface substrate-fact.

All 15 witnesses landed:
- Species declaration `@mirror/store/action_cache` per path-namespace pact
- Interpretation B canonical (single seam; narrative-above discipline)
- Ancestry `in @prism / @meta / @glass / @mirror/store`
- Three actions declared with obligation blocks:
  - `cache_read(spec_oid, target_oid, inputs_oid) -> imperfect` (hit-with-
    verdict / partial / miss)
  - `cache_write(spec_oid, target_oid, inputs_oid, v: verdict) -> verdict`
    (idempotent by content-address)
  - `cache_exists(spec_oid, target_oid, inputs_oid) -> verdict` (stat-only
    fast-path)
- N1 predicate citation (`verdict_is_content_addressed` authorizes cache)
- Bazel REAPI ActionCache ancestor citation
- `crystal.derived_predicates` carrier citation
- Family-root six-op composition
- Operational target (13-min hook overhead)
- N-cascade forward-promises + @kintsugi.settle consumer

## §2. 15/15 empirical verify

Adjacent chain: `mirror_store_apache_floor_shard` (M6) + `verdict_is_content_addressed_shard`
(N1) + this shard = 45/45 clean. No regression on the M6 Apache-2.0 floor
or N1 predicate consumer.

**Recognition #43 empirical consumer chain grew to SIX**: M6 store self-decl
→ M1 mcp_session → M2 spawn → M2 kintsugi → N1 verdict predicate → N2 action
cache. Apache-2.0 floor holds under six first-order consumers.

## §3. Mara Seam-worthy observations — adjudication

### Observation 1: `prism <= @mirror/store` shape chosen over `glass` shape

**Verdict: SUBSTRATE-CANONICAL, ratified.**

Mara followed `@mirror/store/git`'s `prism <name> <= @mirror/store` shape
(species with wire vocabulary) rather than `@mirror/store/crystal`'s `glass`
shape. Rationale documented in-shard: cache verbs are wire-altitude verbs
that discharge against the family six-op surface — mirrors git.mirror's
discharge-map discipline exactly.

Substrate-canonical choice per `[[feedback-substrate-already-had-the-word]]`:
the @mirror/store family has two species-shape templates already; action_cache
fits the wire-verbs template (git) not the storage-shape template (crystal).
Ratified.

### Observation 2: Cross-species discharge in `cache_write`

**Verdict: FLAGGED for Seam review at N3 landing gate.**

Mara notes: `cache_write` discharge composes BOTH `@mirror/store.write`
(new derived_predicates oid) AND `@mirror/store/git.set_ref` (crystal head
update). This CROSSES species boundaries. Worth review to confirm whether
cross-species discharge should be first-class OR specialized per wire.

**Reed lean**: cross-species discharge IS first-class — that's exactly how
the REAPI split works (CAS write + action-cache map advance are logically
two distinct operations composed atomically). Species boundaries reflect
responsibility partitions, not composition boundaries.

**Second-witness gate**: N3 Rust wiring will exercise this discharge
pattern. If it composes cleanly through the substrate + Rust glue, the
pattern promotes to substrate-decl'd cross-species discharge as first-class.
If it needs bespoke plumbing, we specialize.

**Not blocking N-cascade advancement**.

### Observation 3: Recognition #53 bilateral half-landing

**Verdict: substrate-fact; other half forward-promised at N3.**

Mara flags: this N2 landing completes HALF of a Recognition #53 bilateral
pair. The property side (N1 `verdict_is_content_addressed`) + store-side
operational (this N2 action_cache) landed together. The fracture-side
operational (`@kintsugi/fracture/verdict_cache_miss`) is forward-promised
at N3 — the fracture body that fires when cache_exists returns failure.

**Ratified**. #53 bilateral will fully close at N3. Standard three-tick
sub-cascade shape (property → store-op → fracture-body).

## §4. Cross-session design work saved to memory

During this tick's substrate work, Alex surfaced a substantial adjacent
design direction (IDF as substrate primitive):

**Recognition candidate (memory-saved, not-yet-promoted)**:
`fragment IS content-addressed + IDF-weighted by construction` — sister to
Recognition #43. Alex's move: embed IDF at the AST fragment altitude where
SpectralUUID grounds identity. Fragment carries its `idf` field as an
invariant. Every downstream consumer (kintsugi tournament, @spectral/smarts
recall, @bauchladen self-differentiation, @mirror/bench TF-IDF hot-path,
verdict-cache invalidation ordering) composes by reading the substrate
rather than recomputing.

**L-cascade candidate**: opens naturally after N-cascade closes. Sibling to
M (session/spec/song) and N (incremental verdicts). See
`[[project-idf-informativeness-by-specificity]]` for the full design
direction captured this session.

**Not this session's work**. Named to preserve continuity.

## §5. Signal-to-Reed

**N2 TICK 1 CLOSED.** GREEN `0a72c42` ratified; 15/15 pass; adjacent 30/30;
tree clean.

**N-cascade next**:
- **N3 TICK 1**: **Rust wiring** at `bootstrap/src/lib.rs:~1189`.
  `cmd_kintsugi_spec` consults `cache_read` before dispatching cargo; calls
  `cache_write` after each fresh verdict. **The 13-minute hook overhead
  falls empirically once N3 lands.** First observable win.
- **N4 TICK 1**: reverse-closure `impacted_by(oid) -> [oid]` at
  `@mirror/store` family-root.
- **N5 TICK 1**: `@kintsugi/store/git commit-as-fold` (third-witness for
  `cli-verb-pair-specialises-species-action-pair` recognition).

**Alex-adjudication queue** (not blocking):
- Cross-species discharge first-class vs specialized (from Observation 2;
  witness gate at N3)
- L-cascade candidate direction (fragment-altitude IDF embedding);
  post-N-cascade timing
- Prior queue items unchanged

---

*2026-07-06. Seam (Reed-inline). Phase D on N2 TICK 1 `0a72c42` RATIFIED.
@mirror/store/action_cache species landed with three actions (cache_read /
cache_write / cache_exists); Recognition #53 bilateral half completed;
Recognition #43 empirical consumer chain grew to six. Cross-species
discharge witness gate at N3. Business-observable milestone at N3: the
13-minute pre-commit hook falls when Rust wires the cache into
cmd_kintsugi_spec.*
