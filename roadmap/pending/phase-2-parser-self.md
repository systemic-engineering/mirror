# Phase 2 — Parser Self-Description

**Goal:** Mirror's syntax described as a `.mirror` grammar. The grammar that, when compiled, produces a parser equivalent to the Rust parser.

> **Substrate-pull recognition (2026-06-06, Reed + Alex → Mara).** The
> previous Phase 2 target name `@mirror/syntax` was wrong-shaped: syntax
> is what code IS at the mirror altitude. The substrate already had
> `@code` (declared at `shards/code.mirror`) as the universal grammar-
> at-an-altitude discipline; mirror is one instance, declared at
> `shards/code/mirror.mirror` as `@code/mirror`. Phase 2's deliverable
> renames to `@code/mirror` accordingly. Same recognition pattern as
> splinter_graph → mosaic(@store) and fragment → splinter: the substrate
> already had the word; we just hadn't named the inheritance.

## Tasks

1. Write `@code/mirror` grammar describing tokenization rules using the five operations (collapsed from the previous `@mirror/syntax` framing).
2. Write `@code/keyword` grammar implementing the two-tier keyword system (23 hardcoded Tier 1 + boot-declared Tier 2 via the self-teaching parser).
3. Implement the self-teaching mechanism (parser learns from `out X` declarations).
4. **Bootstrap test:** `@code/mirror` parses `@code/mirror`. First self-referential gate.

## Recent landed work this absorbs

- The meta-glass FP1 (per `docs/specs/parser-as-prism-grammar.md`).
- The Combinator enum with type-safe construction (per F-1's walker work).
- The kintsugi-tournament merge resolution (per `docs/specs/kintsugi-tournament.md`).

## Dependencies

Phase 1.
