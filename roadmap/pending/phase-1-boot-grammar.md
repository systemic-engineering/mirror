# Phase 1 — Boot Grammar Completion

**Headline (post-audit 2026-05-26):** Phase 1 as a gate is closer to done than the prior version of this document suggested. Genuinely open: `!=` tokenization, singularity types (spec missing). Everything else is either done or obsolete — the consumption tick (`7461782` → `b13c357`) and the 2026-05-08/09 compiler collapse paid debts this doc didn't know about.

**Goal:** Zero parse holonomy. Boot grammars coherent. The substrate-pull pattern that paid most of the original Phase 1 surface is real and verified end-to-end.

## Consumption surface (substrate-pull tick 2026-05-26)

Already-paid sub-tasks that the prior version of this document did not name. These are not aspirational; they are landed and OID-verifiable on `mara/shard-chain`:

- **Boot grammar declares its own vocabulary in `.mirror`** — `@mirror/glass/ast/token` declares `glass`, `lambda`, `fixed`, `property`, `shape`. (commit `7461782`)
- **Bootstrap consumes substrate declarations for keywords** — `bootstrap/src/grammar.rs::load_grammar` reads `<op> <keyword>` pairs from the companion `.mirror` source and merges them into the primary grammar's keyword table. (commit `29b4dee`)
- **Substrate-pull pattern end-to-end:** read → harvest → tokenize → re-emit. The dispatch logic stays Rust (Phase 2 retires that); the *vocabulary* is now substrate-declared.
- **Boot tree round-trips:** 143/160 byte-identical, 17 changed-and-causally-justified (every changed file uses one or more of the new keywords; the OID delta IS the substrate-pull landing in observable behavior). (commit `b13c357`)
- **OID-stability under substrate-vocabulary changes:** same-keyword/same-op = no-op; same-keyword/different-op = `io::Error` (stop-and-report). The conflict path is the structural guarantee that this mechanism cannot silently overwrite the primary grammar.

## Tasks

1. ~~Add 5 OpticOp DeclKind variants (`Unfold`, `Subset`, `Superset`, `Iso`, `NotIso`) — reduces holonomy by 5.0.~~ **OBSOLETE.** `DeclKind` was killed in the 2026-05-08/09 compiler collapse; the surviving `AstKind` enum in `bootstrap/src/ast.rs:4-46` has no `Unfold/Subset/Superset/Iso/NotIso` variants and is not the layer where this distinction lives. If the optic-op surface is needed it lands as a `.mirror`-declared shape, not as Rust enum variants on the bootstrap AST.
2. **OPEN.** Fix `!=` tokenization — reduces holonomy by ~4.0. Dark-visible under `--strict` post-Seam-T1.1; not yet parsed as an op token. `bootstrap/src/tokenize.rs` has no `!=` case.
3. ~~Fix `->` return type on all declaration kinds — reduces holonomy by 2.0.~~ **DONE.** `bootstrap/src/tokenize.rs:738-857` handles the action-decl + decorator forms (`name(...) -> typename { ... }` and `decorator name(...) -> typename { ... }`); the `-> typename` is captured into the `IoBinding` body for round-trip and the parametric form `T(U)` is admitted via a balanced `(...)` immediately following the type-name identifier.
4. **OPEN + SPEC-MISSING.** Land singularity types (`@human = singularity`, `@ai = naked-singularity`). The cited spec `SINGULARITY.md` does not exist in-tree; the spec the task points at is absent. Blocked on spec authoring before the implementation surface can be drawn.
5. ~~`kintsugi --rebase`: collapse `boot.alex/` onto canonical boot via `@kintsugi/migrate`.~~ **OBSOLETE (paid by other means).** `boot.alex/` directory does not exist in the repo. The substrate-pull tick (2026-05-26, commits `7461782` → `29b4dee` → `b13c357`) is the structurally equivalent migration mechanism: vocabulary now flows from `.mirror` companion sources into the bootstrap's grammar via additive harvesting, not via a Rust-side `--rebase` subcommand. The CLI dispatch in `main.rs:898-925` has no `--rebase` flag and does not need one.
6. ~~Unify `ast.rs` and `mirror_ast.rs` into one `MirrorAST`.~~ **DONE** (2026-05-08/09 compiler collapse). Confirmed: only `ast.rs` exists in `bootstrap/src/`; there is no `mirror_ast.rs` to unify. Follow-up moved to Q1 (regenerate Rust AST from `@mirror/ast`?) and is now Phase 4 work.
7. ~~Clean up `resolve.rs`: remove the `conversation`-era naming; one resolution path.~~ **OBSOLETE (no target).** `resolve.rs` does not exist in `bootstrap/src/`. The "two resolvers coexist" framing is from a code shape that no longer exists; the resolver collapse already landed.

## Exit criterion

`mirror compile boot/` produces zero holonomy. All boot grammars parse, resolve, verify. The remaining open work to clear the gate is narrow: `!=` tokenization (Task 2) and singularity types (Task 4, blocked on spec).

**This is the gate. Nothing moves until Phase 1 is green.**

## Parallel work within phase

Task 2 (tokenizer fix for `!=`) and Task 4 (singularity types — pending spec) are independent. Two contributors can work in parallel once Task 4's spec lands.
