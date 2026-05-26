# Phase 3 — Resolver Self-Description

**Goal:** Mirror's type system described as a `.mirror` grammar.

## Tasks

1. Write `@mirror/resolve` grammar.
2. Express `TypeRegistry` as a `.mirror` type.
3. Express validation rules as `requires` / `invariant` properties.
4. **Bootstrap test:** `@mirror/resolve` resolves `@mirror/resolve`.

## Recent work this absorbs

- Grammar inheritance via `<` (per the mirror-native-vcs spec's open question; lands here cleanly).
- The `@data/*` vs `@nl/*` vs `@code/*` vs `@mirror/*` namespace discipline.

## Dependencies

Phase 2.
