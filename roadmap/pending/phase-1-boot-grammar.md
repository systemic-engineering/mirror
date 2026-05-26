# Phase 1 — Boot Grammar Completion + `kintsugi --rebase`

**Goal:** Zero parse holonomy. Boot grammars coherent. Singularity types landed. Alex's `boot.alex/` rebased onto canonical boot. One AST type. One resolver.

## Tasks

1. Add 5 OpticOp DeclKind variants (`Unfold`, `Subset`, `Superset`, `Iso`, `NotIso`) — reduces holonomy by 5.0.
2. Fix `!=` tokenization — reduces holonomy by ~4.0.
3. Fix `->` return type on all declaration kinds — reduces holonomy by 2.0.
4. Land singularity types (`@human = singularity`, `@ai = naked-singularity`) per `SINGULARITY.md`.
5. `kintsugi --rebase`: collapse `boot.alex/` onto canonical boot via `@kintsugi/migrate`.
6. ~~Unify `ast.rs` and `mirror_ast.rs` into one `MirrorAST`.~~ **DONE** (2026-05-08/09 compiler collapse). The repo has one `AstKind` enum + `AstNode` struct in `bootstrap/src/ast.rs`. Follow-up moved to Q1 (regenerate Rust AST from `@mirror/ast`?) and is now Phase 4 work.
7. Clean up `resolve.rs`: remove the `conversation`-era naming; one resolution path. *(Status post-collapse unverified — needs re-audit before next Phase 1 spawn.)*

## Exit criterion

`mirror compile boot/` produces zero holonomy. All boot grammars parse, resolve, verify. One AST type. One resolver.

**This is the gate. Nothing moves until Phase 1 is green.**

## Parallel work within phase

Phase 1 tasks 1–3 (tokenizer fixes) are independent of tasks 4–7 (type unification + cleanup). Two contributors can work in parallel.
