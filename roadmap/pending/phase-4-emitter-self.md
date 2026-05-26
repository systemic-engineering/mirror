# Phase 4 — Emitter Self-Description + Fragmentation Generation Proof

**Goal:** Output formats described as `.mirror` grammars. **Fragmentation's Rust source generated from `@fragmentation + @code/rust`** — the in-compiler demonstration that mirror compiles production code.

## Tasks

1. Complete the `@code/rust` translate template per `docs/specs/fragmentation-as-generated.md`. The R-tick decomposition there (R-0 through R-6) is the implementation path.
2. Write the `@fragmentation.mirror` grammar (sketch in §3 of fragmentation-as-generated.md; ~400–600 lines projected).
3. Generate `fragmentation/src/` from `@fragmentation + @code/rust`. Replace the hand-written Rust with the generated version.
4. Write the `@code/mirror` render template (the pretty-printer). Round-trip: parse → emit → parse = identity.
5. Write the `@shatter/format` grammar.
6. **Bootstrap tests:** `@code/mirror` renders itself; `@fragmentation + @code/rust` produces a fragmentation crate that passes all of today's fragmentation tests.

**This is THE Phase 4 demonstration vehicle.** Fragmentation as a generated production crate is the concrete proof that mirror's compilation pipeline crossed the maturity threshold for self-hosted production code.

## Dependencies

Phase 3.
