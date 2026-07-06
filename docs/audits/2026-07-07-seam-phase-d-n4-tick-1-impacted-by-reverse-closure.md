# Seam Phase D — N4 TICK 1: `impacted_by` reverse-closure at `@mirror/store` family-root

*Reed-inline execution.*

**Commit under review**: `6bf05cb` (Mara GREEN). Family-root enrichment of
`shards/mirror/store.mirror` — 90 insertions, no deletions. Diff-closure
gate short-circuited (pure-.mirror; `mirror kintsugi mirror.spec`
skipped).

**Reed RED**: `efca158` (12 tests,
`bootstrap/tests/mirror_store_impacted_by_shard.rs`).
**Direct test verification**: 12/12 pass (Mara-verified pre-commit).

---

## §1. Verdict

**RATIFY.** Reverse-closure lands at family-root. Substrate's
reachability algebra closes: `walk` (forward) + `impacted_by` (reverse)
at one altitude.

All 12 witnesses landed:
- T1-T3: `impacted_by(oid: oid) -> [oid]` action declaration (name,
  arg, return type)
- T4: obligation-block body per substrate-decl discipline
- T5: narrative documents reverse-closure complement to walk
- T6: composition with N2 `@mirror/store/action_cache` invalidation
- T7: Bazel `rdeps` prior-art citation
- T8: `out impacted_by` in exports
- T9: N-cascade positioning cited
- T10-T12: regression guards — existing six-op surface + oid +
  splinter_graph + all existing exports preserved

## §2. Recognition consumer chain

**Recognition #43** (mirror IS content-addressed build system) empirical
consumer chain grew to **EIGHT**: M6 store self-decl → M1 mcp_session →
M2 spawn → M2 kintsugi → N1 verdict predicate → N2 action_cache → N3
Rust wiring → **N4 impacted_by** (this tick). Each consumer that lands
vindicates the content-address discipline as operational primitive.

## §3. Cross-species discharge — witness TWO CONFIRMED

N2 §3 Obs 2 flagged the promotion candidate
`cross-species-discharge-is-first-class` (witness gate at N3 passed).
Mara's N4 narrative cites `@mirror/store/action_cache` invalidation
composition explicitly — impacted_by (family-root action) hands OIDs to
the species-altitude cache surface for surgical invalidation. This is
CROSS-SPECIES DISCHARGE at family-root → species direction, complementing
N3's Rust-wiring discharge at species → family-root direction.

**Two witnesses landed**. One more needed for LANDED promotion; N5
commit-as-fold composition with impacted_by (via rebase walk) would be
witness three.

## §4. Hook fix precondition

This commit is the FIRST to exercise the `git-hooks.nix` walk-past-📝
fix (~/.os `0e80b1c`, activated for both alexwolf and reed users
2026-07-07 00:47 UTC).

Without the fix, Seam's 📝 audit of N3 (`09b850c`) would have blocked
N4 GREEN's 🟢 (sequence check saw 📝 as HEAD, rejected the pair).
Mara surfaced the observation during her N4 GREEN work; Alex
adjudicated as "walk backward through 📝" per Ashby-preferred variety
preservation.

**Substrate-side outcome**: cascade discipline now truly interleavable
with audit discipline. Seam's 📝 audits are meta-commentary on TDD
pairs, not participants — which was always the intent; now enforced
by the hook semantics.

## §5. Reed observations — adjudication queue

### Observation 1: Diff-closure gate short-circuit efficacy

N4 GREEN commit's pre-commit hook completed in ~1 second because the
diff-closure gate (per Recognition #53 second instance,
`@kintsugi/fracture/gate`) matched the staged set to pure-.mirror. No
cargo dispatched. This is the substrate-side optimization that
complements N3's cache mechanism: pure-shard commits skip cargo
entirely.

**Substrate-fact**: the two optimizations layer cleanly — diff-closure
gate handles pure-.mirror commits; verdict cache handles .rs commits
with unchanged inputs. Together they should compress the hook from
30-60 min cold to seconds on most cascade ticks.

### Observation 2: Spärck Jones citation grounds L-cascade

Mara's narrative cites Spärck Jones 1972 inverted-index anchor. This
is PRE-work for the L-cascade (`project-idf-informativeness-by-
specificity`) which Anthropic's Workspace research just independently
surfaced via J-space eigenvector analysis (per
`docs/research/2026-07-07-workspace-mirror-mapping.md`).

**Convergence signal**: two independent paths (inverted-index / IDF and
J-space subspace projection) both point at "informativeness by
specificity" as the substrate primitive that binds the reverse-closure
to the write-through-cut invariant. L-cascade timing may be sooner
than post-N-cascade.

### Observation 3: N-cascade near-close

N1 → N2 → N3 → **N4** landed. N5 (`@kintsugi/store/git commit-as-fold`)
is the terminal tick. After N5:
- All Recognition #43 consumers landed (chain to NINE)
- Cross-species discharge promoted to LANDED (three witnesses)
- `cli-verb-pair-specialises-species-action-pair` third-witness gate
  passes
- L-cascade opens

## §6. Signal-to-Reed

**N4 TICK 1 CLOSED.** GREEN `6bf05cb` ratified; 12 witnesses landed;
#43 chain to EIGHT; cross-species discharge witness two CONFIRMED;
hook fix operational.

**N-cascade next (terminal)**:
- **N5 TICK 1**: `@kintsugi/store/git commit-as-fold` species at
  `shards/kintsugi/store/git.mirror`. Third-witness for
  `cli-verb-pair-specialises-species-action-pair` recognition. Third
  witness for `cross-species-discharge-is-first-class` candidate.

**Alex-adjudication queue** (not blocking):
- `cross-species-discharge-is-first-class` LANDED gate at N5 close
- Numeric ID for `cli-verb-pair-specialises-species-action-pair`
  (LANDED at N5 close if third witness cleanly composes)
- L-cascade timing — open sooner than post-N-cascade given Workspace
  convergence?
- Workspace research A1-A3 signals (per Mara's report)
- Prior queue items unchanged

---

*2026-07-07. Seam (Reed-inline). Phase D on N4 TICK 1 `6bf05cb`
RATIFIED. Family-root enrichment adding `impacted_by(oid: oid) -> [oid]`
reverse-closure landed via diff-closure gate short-circuit (~1s hook).
Recognition #43 empirical consumer chain grew to EIGHT. Cross-species
discharge witness TWO confirmed. First commit exercising the git-hooks
fix (~/.os `0e80b1c`). Terminal N-cascade tick (N5 commit-as-fold)
queued.*
