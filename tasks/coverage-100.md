# Coverage: Achieve 100% line coverage

**Priority:** High — TDD gate is non-negotiable.
**Current:** 742/744 tests pass. Coverage gate exists in CI but current % unknown.
**Target:** 100%

## Known gaps

- `src/lsp/` — may be orphaned now that conversation-lsp is a separate crate. Remove or keep as library code.
- `src/db.rs` — 12 tests, new module, check coverage
- `src/packages.rs` — 2 test failures (to_namespace tests expect Err but tolerant loader returns Ok). Fix tests.
- `src/main.rs` — CLI paths, hard to unit test

## Pre-existing test failures

- `to_namespace_parse_error` — expects `is_err()` but tolerant loader logs and continues
- `to_namespace_grammar_compile_error` — same issue

Fix: update these tests to match the new tolerant behavior (check that the namespace doesn't contain the broken grammar, rather than checking for Err).

## Approach

1. Fix the 2 failing tests first
2. Run `nix develop -c cargo llvm-cov --lib --features lsp,db`
3. Cover all public API paths
4. The `src/lsp/` code: decide whether to keep as shared library or remove (it's duplicated in conversation-lsp)

## Rules

- 100% line coverage. No cheating.
- If code can't be tested, refactor until it can
