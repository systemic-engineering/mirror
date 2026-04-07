# Spring Clean Report — mirror crate (2026-04-07)

## Files deleted
- `src/compile.rs` (~660 lines) — entire BEAM EAF emission module. Pattern preserved at `spectral/docs/gen_prism.md`.
- `src/boot.rs` (265 lines) — old boot orchestration. Zero callers in src/ under the new `mirror_runtime::compile_boot_dir` pipeline.
- `tests/compile_test.rs` (474 lines) — integration test for deleted compile.rs.

## Files modified
- `src/lib.rs` — removed `pub mod compile;` and `pub mod boot;`; updated module doc header (dropped compile bullet, added mirror_runtime bullet, dropped boot bullet).

## Lines deleted
~1,400 lines of source + tests.

## Cargo.toml dependencies removed
None. `eetf` is still used by `src/spectral.rs` (independent ETF encoding paths on lines 329–381, and in tests). Kept.

## Test output
```
test result: FAILED. 810 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
failures:
    abyss::tests::boot_sequence_settles_combined
    emit::tests::round_trip_boot
```
Baseline before cleanup: 822 passed, same 2 failures (pre-existing, unrelated — both fail on `form @property {` parse error in boot.backup fixtures). Delta: -12 tests (compile.rs unit tests removed with the file). No new failures.

## Build output
```
Compiling mirror v0.1.0 (/Users/alexwolf/dev/projects/mirror)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.32s
```

## Hard walls
None hit. boot.rs came out cleanly — it had no callers in src/ (only self-tests that loaded `boot.backup/` fixtures). main.rs, session.rs, training.rs do not reference it.

## Not touched (deferred, out of scope or risky)
- `boot.backup/` directory — still referenced by `src/property.rs` tests and was the basis of the 2 pre-existing failing tests. Leaving it as Pack's call.
- `bootstrap.mirror`, `main.mirror`, `systemic.engineering.mirror` at crate root — vestigial-looking but not confirmed dead; no src references but possibly used by external flows.
- Other potentially-dead modules (ghost, mirror_bf, classifier, features, gestalt) — would need a deeper pass with per-module caller analysis. Not done in this dispatch to stay within the "decisive but safe" mandate.
- `erl_crash.dump` at crate root — debris, but not source.
