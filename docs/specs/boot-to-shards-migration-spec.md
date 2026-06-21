# Spec: boot/std/* → shards/ migration

*Evergreen spec for the substrate-pull-correct migration of mirror's legacy `boot/std/*.mirror` corpus into the canonical `shards/` substrate-decl tree. Grounded in three woz:explore agent runs on 2026-06-21. Pairs with `docs/roadmap/13-boot-to-shards-migration.md`.*

## 0. The substrate-pull claim

The substrate is publishing red CI on `mirror kintsugi --ci --shatter 4 boot/std` (verdict `failure`, `dark_count: 1635 across 135 files`). The red is **correct**: it discloses 1635 substrate-pull-unsatisfied dark instances in the legacy boot/std/* tree.

The substrate-pull-correct fix: migrate boot/std/*.mirror to shards/*.mirror per `[[architecture-shards-as-substrate-source]]`. Each migration tick reduces `dark_count` by some delta. The kintsugi loop iterates until `dark_count = 0` OR `boot/std/` is empty.

This is the structural-altitude ouroboros bite — same #53 bilateral pattern as `dark_count_monotone` (tick 41) and `cold_compile_within_tolerance` (tick 43), applied at the corpus-migration altitude.

## 1. Grounded engine facts (from woz:explore 2026-06-21)

### 1.1 Resolution is namespace-based, not path-based

`bootstrap/src/lib.rs::cmd_kintsugi_ci_corpus` + `collect_declared_namespaces` + `count_unresolved_imports`:

- Scans both `shards/` AND `boot/std/` for top-level declarations (`glass @X`, `prism @X`, `grammar @X`)
- Accumulates into a flat `HashSet<String>` keyed by `@<path>` namespace name
- For each `in @X` clause, membership-tests against the set
- **Filesystem path is invisible to the resolver.** Both trees contribute equally to the declared set.

### 1.2 OIDs are content-addressed

`bootstrap/src/hash.rs::canonical_hash`. Byte-identical content at different paths → same OID. Moving a file with no content change → zero OID change → no cache bust.

### 1.3 dark_count = parser failures + unresolved imports

`bootstrap/src/lib.rs::count_dark` + `count_unresolved_imports`:

- Base dark: count of `AstKind::Dark` AST nodes (tokenization failures)
- Plus: count of `in @<path>` clauses whose `@<path>` isn't in the declared-namespace HashSet
- Each unresolved import = +1 dark

`Uncrystallized` (runtime dispatch failure when `Crystallizations<H>` has no body for a Ref) is a SEPARATE failure mode and is NOT counted toward dark_count.

### 1.4 Migration mechanism

`git mv boot/std/X.mirror shards/std/X.mirror` (or similar canonical path).

- No content rewrite required
- No importer rewrite required (any `in @std/X` continues resolving)
- Path-namespace property (`@epistemologic/pact/path_matches_namespace`) IS satisfied because the file's declared namespace matches BOTH paths' structure (only the leading `boot/std/` vs `shards/` differs)
- `bootstrap/tests/cross_shard_resolution.rs` covers regression

### 1.5 Open empirical question

The resolver SHOULD merge both trees — yet CI reports 1635 dark across 135 boot/std/ files. Either:
- (a) CI working-directory context skips the `shards/` scan
- (b) The 1635 includes base parser dark (not only unresolved imports)
- (c) There's deeper substrate-pull behavior not yet surfaced

**Phase 0 (calibration) resolves this empirically before the bulk migration starts.**

## 2. Topology (from woz:explore agent 2)

| Metric | Count |
|---|---|
| boot/std/*.mirror files | 134 |
| Leaves (zero incoming edges) | 65 |
| Critical files (shards → boot edges > 0) | 19 |
| Total shards → boot edges | 139 |
| Cycles | 0 (DAG) |

**The spine** (top files by incoming-edge count):

| File | Total incoming | shards → boot | Migration impact |
|---|---|---|---|
| `nl.mirror` | 65 | **60** | Migrating unblocks 60 shards files |
| `kintsugi.mirror` | 19 | **14** | Migrating unblocks 14 shards files |
| `epistemologic/property.mirror` | 21 | **12** | Migrating unblocks 12 shards files |
| `mirror/store.mirror` | 13 | 10 | Migrating unblocks 10 shards files |
| `epistemologic/math/bundle.mirror` | 11 | 8 | |
| `epistemologic/property/content_addressed.mirror` | 10 | 7 | |
| `io.mirror` | 39 | 6 | (mostly boot→boot edges) |

**62% of shards→boot dependency load lives in the top 3 spine files.** Migrating those three early unblocks 86 of 139 cross-substrate edges.

## 3. The substrate-decl: bilateral pair (recognition #91 candidate)

Following the established #53 property+fracture pattern (third and fourth instances landed yesterday: `dark_count_monotone` + `cold_compile_within_tolerance`):

### 3.1 Declarative half: `shards/epistemologic/property/imports_resolved.mirror`

```mirror
in @prism
in @meta
in @glass
in @epistemologic
in @epistemologic/property
in @mirror

# Property: every `in @<path>` clause in every shard resolves
# against the declared-namespace HashSet (per
# bootstrap/src/lib.rs::collect_declared_namespaces + count_unresolved_imports).

prism @epistemologic/property/imports_resolved {
  focus  imports_resolved
  project imports_resolved
  split  imports_resolved
  shift  imports_resolved
  settle imports_resolved
}

type shard_path = ref
type namespace = ref

imports_resolved(shard: shard_path) -> verdict { \ }
all_imports_resolved(corpus: ref) -> verdict { \ }

out @epistemologic/property/imports_resolved
out shard_path
out namespace
out imports_resolved
out all_imports_resolved
```

### 3.2 Operational half: `shards/kintsugi/fracture/migrate_from_boot.mirror`

```mirror
in @prism
in @meta
in @glass
in @kintsugi
in @epistemologic/property/imports_resolved

# Fracture body that emits a morphism when a boot/std/<X>.mirror
# shard would be eliminated from dark_count by moving to
# shards/<canonical-path>/<X>.mirror.
#
# Per agent 1 finding: the move is `git mv` only; no content
# rewrite; no importer rewrite. The morphism's splinter(ast)
# names the file-relocation action.

glass @kintsugi/fracture/migrate_from_boot {
  focus  migrate_from_boot
  project migrate_from_boot
  split  migrate_from_boot
  shift  migrate_from_boot
  settle migrate_from_boot
}

resolve_boot_to_shards(opacity: opacity) -> morphism {
  morphism {
    content: splinter(ast) {
      content:      opacity.location.file,
      ast:          @meta/ast,
      transparency: success,
    },
    score: dissonance {
      roughness: opacity.property,
      partials:  1,
    },
    expected: authentic,
  }
}

out @kintsugi/fracture/migrate_from_boot
out resolve_boot_to_shards
```

Fifth instance of #53 (after keyword/depth, dark_count_monotone, cold_compile_within_tolerance, plus task #272 @mirror/fracture/predicate forward-promised).

## 4. The migration phases (the loop)

### Phase 0: Calibration (1 tick)

**Purpose**: empirically resolve the open question (§1.5).

**Action**: pick the smallest boot/std/* leaf file (zero incoming edges). `git mv` it to its canonical shards/ path. Run `mirror kintsugi --ci --shatter 4 boot/std` BEFORE and AFTER the move. Compare dark_count delta.

**Outcomes**:
- Delta = 0 → resolver already saw both trees; dark must be base parser failures or different mechanism. Investigate before Phase 1.
- Delta < 0 (dark drops by some amount, possibly equal to that file's outgoing-edge count) → resolver merge works; proceed to Phase 1 as planned.
- Delta > 0 → migration BROKE something. Revert; investigate.

**Bench**: record bench_crystal with `runtime_ns`, `output_oid` (the dark_count BEFORE), `env_oid` (substrate hardware + Rust version + flags).

### Phase 1: Leaves batch (estimated 13 ticks)

**Purpose**: clear the 65 zero-incoming-edge leaves — the safest, parallel-batchable migrations.

**Action**: each tick, `git mv` ~5 leaves in one batch (move-only, no content edits). Run `mirror kintsugi --ci --shatter 4 boot/std` after. Record bench_crystal with delta.

**Termination**: when no leaves remain in boot/std/, advance to Phase 2.

### Phase 2: Spine in reverse-dependency order (estimated 19 ticks)

**Purpose**: clear the 19 critical files in topological order (most-depended-on goes LAST, so importers' deps land before consumers).

**But reversed for migration impact**: per agent 2 finding, **migrating the top 3 spine files FIRST unblocks 62% of cross-substrate dependencies**. So:

- Tick 14: `nl.mirror` → `shards/nl.mirror` (already exists at shards/nl.mirror!) — see §6 ambiguity. Calibrate first.
- Tick 15: `kintsugi.mirror` → verify shards already has `shards/kintsugi.mirror`? Same calibration.
- Tick 16: `epistemologic/property.mirror` → same check.
- Ticks 17-32: Remaining 16 critical files in reverse-dependency order from agent 2's analysis.

### Phase 3: Buffer (estimated 3 ticks)

**Purpose**: handle any cross-substrate edge residue surfaced by Phases 1-2.

### Total estimate: ~35 ticks (per agent 2)

## 5. Composition with the cascade

- **#86 @moi**: each tick's `migration_record` is `@moi(migration_record)` — pact-verified at composition time
- **#87 @mirror/bench**: each tick produces a bench_crystal recording (op = migrate, args_oid = file SHA, runtime_ns, output_oid = dark_count BEFORE, env_oid, predecessor, tick_index)
- **#88 @loop**: each migration tick IS one @loop.bind iteration at the migration-discipline altitude; loop_well_founded requires monotonic dark_count decrease
- **#89 @mirror/ref**: `mirror ref unresolved | settle migrate_from_boot` IS the operational form of the migration loop
- **#90 @mirror/docs**: the migration's audit trail lives at `docs/audits/<YYYY-MM-DD>-reed-boot-to-shards-migration-tick-N.md` per the @mirror/docs/audit species

## 6. Substrate-honest ambiguity

Several boot/std/* file names match existing shards/ file names:
- `boot/std/nl.mirror` vs `shards/nl.mirror` (different content?)
- `boot/std/kintsugi.mirror` vs `shards/kintsugi.mirror` (different content?)
- `boot/std/epistemologic/property.mirror` vs `shards/epistemologic/property.mirror`?

**The Phase 0 calibration MUST inspect this**: if both files declare the same `@<namespace>`, the namespace-set has one entry, but which file's content wins? Or are the contents byte-equal? If diverged, the migration becomes content-merge, not git mv.

**Forward-promised tick**: Mara writes a canonical migration-merge protocol if Phase 0 reveals divergence.

## 7. Validation criteria

The spec ratifies when:
- Phase 0 calibration tick lands; dark_count delta is empirically measured
- The bilateral pair (`imports_resolved` + `migrate_from_boot`) substrate-decl lands in shards/ and compiles
- The first leaf-batch tick lands on origin/main with dark_count delta recorded
- Pre-existing `bootstrap/tests/cross_shard_resolution.rs` stays green throughout

## 8. Forward-promised work

1. Bench infrastructure fast-path (`Crystallizer::observe_bench_crystal` per Taut's @mirror/bench analysis) — without this, bench_crystal recording adds Heisenberg-violation overhead per tick
2. Mara canonical spec for recognition #91 if the cascade-discipline says so (otherwise this evergreen spec suffices)
3. Boot/std/* deletion (vs archive) policy decision: do empty boot/std/ directories stay as artifacts, or get removed once empty? Substrate-pull leans: remove, since boot/std/ has no role once migration completes.
4. CI workflow update: `mirror kintsugi --ci --shatter 4 boot/std` becomes `mirror kintsugi --ci --shatter 4 shards/` once boot/std is empty.
5. The `imports_resolved` bilateral predicate's first non-decorative consumer is the migration loop's `settle_with` action call — ensure first-consumer pattern holds.

## 9. Out of scope (intentional)

- Rewriting boot/ files at content altitude (Phase 0 calibration may surface need; THEN spec'd as separate work)
- Migrating non-boot/std/* legacy (boot/00-prism.mirror and boot/*.mirror outside std/) — separate substrate-pull pass
- Bench infrastructure landing (Taut territory; blocking-but-decoupled)

— Reed
