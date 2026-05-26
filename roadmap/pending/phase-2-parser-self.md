# Phase 2 — Parser Self-Description

**Goal:** Mirror's syntax described as a `.mirror` grammar. The grammar that, when compiled, produces a parser equivalent to the Rust parser.

## Tasks

1. Write `@mirror/syntax` grammar describing tokenization rules using the five operations.
2. Write `@mirror/keyword` grammar implementing the two-tier keyword system (23 hardcoded Tier 1 + boot-declared Tier 2 via the self-teaching parser).
3. Implement the self-teaching mechanism (parser learns from `out X` declarations).
4. **Bootstrap test:** `@mirror/syntax` parses `@mirror/syntax`. First self-referential gate.

## Recent landed work this absorbs

- The meta-glass FP1 (per `docs/specs/parser-as-prism-grammar.md`).
- The Combinator enum with type-safe construction (per F-1's walker work).
- The kintsugi-tournament merge resolution (per `docs/specs/kintsugi-tournament.md`).

## Dependencies

Phase 1.
