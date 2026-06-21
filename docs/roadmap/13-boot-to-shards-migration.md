# 13 — boot/std/* → shards/ migration

*The substrate-self-cleanup loop. Recognition #91 candidate. Spec at `docs/specs/boot-to-shards-migration-spec.md`. Grounded in three woz:explore agent runs 2026-06-21.*

## Why this is on the roadmap

The substrate publishes red CI on every push (`mirror kintsugi --ci --shatter 4 boot/std` returns verdict `failure`, `dark_count: 1635 across 135 files`). The substrate-pull-correct fix is the migration of boot/std/*.mirror into shards/.

Until this migration runs, the kintsugi self-host workflow stays red. After it lands, the substrate verifies itself as a single coherent shards/ corpus per `[[architecture-shards-as-substrate-source]]`.

## Where this fits

- **Recognition #90** (@mirror/docs): made docs-taxonomy substrate-fact; this migration is the parallel move at the substrate-decl-tree altitude
- **Recognition #87** (@mirror/bench): each migration tick records a bench_crystal; monotonic dark_count decrease is empirically tracked
- **Recognition #88** (@loop): each migration tick IS one @loop iteration
- **Recognition #89** (@mirror/ref): the operational CLI is `mirror ref unresolved | settle migrate_from_boot`
- **Task #285** (Track J: kintsugi-on-Rust): this migration is the same shape as the Rust→mirror lift, applied at the boot/→shards altitude

## Phases

| Phase | Tick count | Action | Termination |
|---|---|---|---|
| 0. Calibration | 1 | Move ONE leaf; measure dark delta; verify resolver behavior | Empirical signal confirmed/refuted |
| 1. Leaves batch | ~13 | git mv ~5 leaves per tick (65 zero-incoming-edge files) | Zero leaves left in boot/std/ |
| 2. Spine (reverse-dep) | ~19 | Top 3 first (nl/kintsugi/epistemologic-property unblocks 62% of cross-edges), then 16 remaining critical files | All shards→boot edges = 0 |
| 3. Buffer | ~3 | Residue cleanup; boot/std/ empty | docs/cleanup; CI workflow update |
| **Total** | **~36** | | dark_count over boot/std = 0 OR boot/std/ empty |

## Substrate-decl prerequisite

Lands at `shards/epistemologic/property/imports_resolved.mirror` + `shards/kintsugi/fracture/migrate_from_boot.mirror`. Fifth instance of the #53 bilateral pattern. Compiles before Phase 0 calibration tick.

## Composition with cascade

Each migration tick produces:
- `@moi(migration_record)` per recognition #86 (pact-verified composition)
- `bench_crystal` per recognition #87 (runtime + dark_count delta + env fingerprint)
- One `@loop.bind` iteration per recognition #88 (monotone_non_increasing gated)
- One `@mirror/ref.settle_with` invocation per recognition #89 (CLI surfaces the loop)
- One `@mirror/docs/audit` record if the tick produces a non-trivial finding (per recognition #90)

## Open empirical questions (Phase 0 resolves)

1. Does dark_count actually drop when a boot/std/* file moves to shards/? The resolver code (`bootstrap/src/lib.rs::cmd_kintsugi_ci_corpus`) SHOULD see both trees, but CI reports 1635 dark anyway. Phase 0's calibration tick provides the empirical answer.

2. Several boot/std/* files have shadow shards/* files of the same name (`nl.mirror`, `kintsugi.mirror`, `epistemologic/property.mirror`). Do they have byte-equal content (clean `git mv` works) or diverged content (content-merge required)?

3. Once boot/std/ is empty, can `mirror kintsugi --ci --shatter 4 boot/std` be replaced by `mirror kintsugi --ci --shatter 4 shards/` cleanly in the CI workflow, or does the substrate-self-host expect both trees to exist?

## Forward-promised

- Bench infrastructure fast-path (`Crystallizer::observe_bench_crystal`) per Taut's analysis — lands separately; without it, bench overhead violates Heisenberg floor per tick
- Mara canonical spec for recognition #91 (optional; this evergreen spec suffices if cascade discipline accepts roadmap+spec as substrate-decl)
- Removal of `boot/std/` empty directories once migration completes (policy decision deferred)
- CI workflow path update (boot/std → shards/) once migration completes

## Substrate-pull confidence

- **HIGH** on the migration mechanism (woz:explore agent 1: namespace-based resolver merges trees; git mv is sufficient; no rewrites)
- **HIGH** on the topology (woz:explore agent 2: 65 leaves, DAG, 3 spine files = 62% of load)
- **MEDIUM** on the empirical dark_count behavior (resolver theoretically merges; CI shows 1635 anyway — Phase 0 resolves)
- **HIGH** on the bilateral pair shape (fifth #53 instance, well-established pattern)
- **MEDIUM** on the tick count estimate (~36; could be ~20 if shadow-file merges are clean; could be ~60 if content divergences require Mara-spec'd merge protocol)

## Status (2026-06-21)

- [x] Recognition named and scoped (this tick)
- [x] Three woz:explore agent runs grounding the spec (this tick)
- [x] Roadmap entry (this file, this tick)
- [x] Spec (`docs/specs/boot-to-shards-migration-spec.md`, this tick)
- [ ] Bilateral pair landed (`shards/epistemologic/property/imports_resolved.mirror` + `shards/kintsugi/fracture/migrate_from_boot.mirror`)
- [ ] Phase 0 calibration tick
- [ ] Phase 1 (leaves) execution
- [ ] Phase 2 (spine) execution
- [ ] Phase 3 (buffer) execution
- [ ] CI workflow path update
- [ ] `boot/std/` directory removal

— Reed
