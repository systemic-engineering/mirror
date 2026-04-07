# mirror ai — CLI dispatch report

## Summary

Added the `mirror ai` subcommand surface to `src/main.rs`, wiring fate's
`FateRuntime::select` into the CLI as a deterministic, pipeable inference
operation over mirror forms.

## Design choice — Option A (selector mode)

`mirror ai <model> <file>` runs `FateRuntime::select(<model>, features_of(file))`
and emits a small `@selection` form encoding `{input, from, next}`. The output
is parseable mirror source so the next `mirror ai` invocation can read it from
stdin via `-`. This is the smallest interpretation that uses fate's existing API
without inventing new semantics.

## CLI surface

- `mirror ai <abyss|pathfinder|cartographer|explorer|fate> [<file>|-]`
- `mirror ai [<file>|-]` — defaults to `fate`
- `mirror ai ... --out=<path>` — write result to file instead of stdout
- `mirror ai ... --train` — flag accepted, training semantics TBD; `--out`
  is honored regardless
- `mirror fmt <file>` — alias for `ai fate --train --out=<file> <file>`,
  reads `<file>`, writes the inference result back to `<file>`
- Stdin input: explicit `-` or implicit when stdin is not a tty

`mirror ai '<form>' <file>` (anonymous form invocation) and `mirror '<form>' <file>`
top-level alias are NOT implemented in this dispatch — `parse_model` rejects
unknown first positional and falls through to the file-arg path, so a quoted
form would currently be treated as a file path. Documented as TBD.

## Determinism

Verified bit-for-bit identical output across two consecutive runs of
`mirror ai fate boot/00-form.mirror`. Feature extraction is pure structural
counting normalized by total. No `Rng`, no `SystemTime`, no env reads in the
inference path. Fate's BF interpreter is deterministic by construction.

## Files modified

- `/Users/alexwolf/dev/projects/mirror/src/main.rs` — ~200 lines added

## Test baseline

`cargo test --lib`: **810 passed, 2 failed, 1 ignored** — matches the prior
baseline exactly. Failures are pre-existing
(`emit::tests::round_trip_boot`, `abyss::tests::boot_sequence_settles_combined`).

## Hard walls hit

None.
