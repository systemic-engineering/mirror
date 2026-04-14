# `grammar @mirror < @runtime` — The Runtime Spec

**Author:** Glint
**Date:** 2026-04-14
**Status:** Specification. The delta between what exists and what closes the loop.

---

## 1. What Exists (Inventory)

### Mirror crate (`/Users/alexwolf/dev/projects/mirror/`)

#### Implemented and tested

| File | What it is | Status |
|------|-----------|--------|
| `src/mirror_runtime.rs` | Parser (`parse_form`), `Form` tree, `Shatter` (Prism impl), `MirrorRegistry` (FrgmntStore-backed), boot pipeline, `materialize_crystal`, `compile_file`, `compile_source` | **Solid.** 549+ tests. Full parse-resolve-emit pipeline. Content-addressed round-trips proven. |
| `src/cli.rs` | `Cli::open`, `Cli::dispatch`, 17 commands (compile, crystal, ci, ca, ai, kintsugi, verify, init, repl, focus/project/split/zoom/refract, bench, registry) | **Solid.** 20+ e2e tests. Dispatch is a match table, not grammar-driven. |
| `src/store.rs` | `Store` trait, `MirrorOid`, `Shard<V>`, `ForeignKey` trait | **Solid.** Trait-level, in-memory test impl. No disk store in this module (disk is FrgmntStore in mirror_runtime). |
| `src/declaration.rs` | `DeclKind` (23 variants), `OpticOp` (10 variants including Subset/Superset/NotIso/Unfold), `MirrorData`, `MirrorFragment`, `fragment()` builder | **Solid.** Encode/Decode round-trips. All variants tested. |
| `src/loss.rs` | `MirrorLoss` (four-fold: ParseLoss, ResolutionLoss, PropertyLoss, EmitLoss), `Convergence`, `PhaseRecord`, `Phase` | **Solid.** Implements `Loss` trait. Holonomy is sum of all four folds plus convergence penalty. |
| `src/dispatch.rs` | `Value`, `Args`, `Response` — runtime dispatch types | **Implemented.** Enum-based, no serialization yet. |
| `src/runtime.rs` | `MetalRuntime` trait — compile Prism to Metal instructions | **Implemented.** Trait only. The Metal backend compiles prism values to flat instruction sequences. |
| `src/session.rs` | `Session` state machine (Idle -> Focused -> Projected -> Forked -> Merged -> Trained), `GestaltProfile`, fork/merge/zoom/train/refract | **Implemented.** Full lifecycle tested. |
| `src/parse.rs` | `Parse` struct implementing `Vector` — tokenizer and AST builder | **Implemented.** Used by spectral's `optic_cmd`. |
| `src/ast.rs` | `Ast`, `AstNode` — the raw parse tree | **Implemented.** |
| `src/ast_prism.rs` | Prism over AST nodes | **Implemented.** |
| `src/bounded.rs` | Bounded storage with pressure-based eviction | **Implemented.** |
| `src/bundle.rs` | `MirrorCompiler` — grammar compilation to shard | **Implemented.** |
| `src/shard.rs` | `Shard` — compiled artifact with rank, target, grammar_oid | **Implemented.** |
| `src/kernel.rs` | `Oid`, `TraceOid`, `ContentAddressed`, `Vector`, `Trace`, `Composed`, `Latent`, `Setting`, `Addressable` | **Solid.** Foundation types, macro-first module. |
| `src/lsp/` | Grammar generation from tree-sitter `node-types.json` + LSP capability probing | **Implemented.** Generates `@code/<lang>` grammars. NOT an LSP server. NOT a language server protocol implementation. This is a grammar *learner*, not a grammar *server*. |
| `src/domain/` | `filesystem` (Filesystem Setting), `conversation` (Script Setting) | **Implemented.** Trait-level domain bindings. |
| `src/gestalt.rs` | `GestaltProfile` — reader model with loss tracking | **Implemented.** |
| `src/optic.rs` | Optic CLI dispatch for the five operations | **Implemented.** |
| `src/sign.rs` | Ed25519 signing (`#[cfg(feature = "git")]`) | **Implemented.** |
| `src/git_store.rs` | Git-backed store (`#[cfg(feature = "git")]`) | **Implemented.** |
| `src/classifier.rs` | Type classifier | **Implemented.** |
| `src/filter.rs` | AST filtering | **Implemented.** |
| `src/abyss.rs` | Abyss loop (convergence) | **Implemented.** |
| `src/prism.rs` | Mirror's Prism | **Implemented.** |
| `src/mirror_bf.rs` | Brainfuck-style interpreter (Metal test bed) | **Implemented.** |

#### Boot kernel (`boot/`)

| File | What it declares | Status |
|------|-----------------|--------|
| `00-prism.mirror` | `@prism` — identity, five operations, `in`/`out` | **Crystal.** The axiom. |
| `01-meta.mirror` | `@meta` — type/ref/operators, pure/real, imperfect, beam, precision, grammar, abstract | **Crystal.** The meta-language. |
| `01a-meta-action.mirror` | Action extensions to `@meta` | **Crystal.** |
| `01b-meta-io.mirror` | IO extensions to `@meta` | **Crystal.** |
| `02-shatter.mirror` | `@shatter` — materialize, crystallize, learn | **Crystal.** |
| `03-code.mirror` | `@code` — position, range, severity, diagnostic, completion, token, `abstract grammar @code` with 6 abstract actions | **Crystal.** The LSP type surface. |
| `03a-code-rust.mirror` | Rust-specific `@code/rust` | **Crystal.** |
| `04-actor.mirror` | `@actor` — actor, state, process, message, `abstract grammar @actor` with start/send/stop | **Crystal.** The actor abstraction. |
| `05-property.mirror` | `@property` — verdict, property_error, property_loss, templates vs properties | **Crystal.** |
| `06-package.mirror` | `@package` — version, semver, mirver, package, resolve/install/publish/diff/compatible | **Crystal.** |

#### Boot std (`boot/std/`)

| File | What it declares | Status |
|------|-----------------|--------|
| `beam.mirror` | `@beam` — luminosity, beam, process, message, callbacks, gen_server, supervisor, strategy, `grammar @beam` with start/call/cast/supervise/stop | **Crystal.** The BEAM runtime grammar. Declares `in @actor`. |
| `cli.mirror` | `@cli` — exit_code, command, flags-as-optics, commands-as-compositions, 25 actions, recover/rescue | **Crystal.** The CLI grammar. |
| `mirror.mirror` | `@mirror` — templates (unique_variants, every_type_reachable, etc.), invariants (idempotent, deterministic, pure), ensures (always_halts) | **Crystal.** The self-referential grammar. |
| `properties.mirror` | Templates (iso-observing) and properties (verdict-producing), boundary properties, security properties | **Crystal.** |
| `time.mirror` | `@time` — tick, snapshot, timeline, cursor, mutation, delta, replay, fork, `grammar @time` with enter/restore/browse/compare/replay/fork/step/present | **Crystal.** |
| `tui.mirror` | `@tui` — key, chord, binding, panel, color, dot, gutter_line, `grammar @tui` with render/layout/gutter/dots/timeline/scroll/prompt/complete, keybindings | **Crystal.** |
| `benchmark.mirror` | Benchmark grammar | **Crystal.** |

#### Docs/specs (today's session)

| File | What it is |
|------|-----------|
| `compiler-surface-plan.md` | Compiler surface design |
| `error-surface-spec.md` | Error surface spec |
| `i18n-feature-spec.md` | i18n feature spec |
| `reflection-model.md` | Reflection model |
| `2026-04-02-*.md` | CI/CA/package design specs |
| `2026-04-14-kintsugi-plan.md` | Kintsugi canonical ordering plan |

### Spectral crate (`/Users/alexwolf/dev/projects/spectral/`)

#### Implemented

| File | What it is | Status |
|------|-----------|--------|
| `src/main.rs` | CLI: 5 optic commands, init, repl (stub), tick/tock/shatter (stubs), diff, log, blame (stub), `mirror` delegate, `memory` subcommand, `serve` MCP server | **Partial.** Five optics work via `mirror::parse::Parse`. tick/tock/shatter are stubs. repl prints "(not yet wired)". |
| `src/serve.rs` | MCP JSON-RPC server over stdio. Scans `.mirror/.conv` for grammar actions, exposes as MCP tools. Memory tools stubbed ("not yet wired"). | **Partial.** Protocol skeleton works. Tool dispatch returns errors because lens is disabled. |
| `src/apache2/runtime.rs` | `Runtime` trait: `tick(&mut self, signal) -> Imperfect<State, Error, Loss>` | **Trait only.** No implementors in spectral. |
| `src/apache2/signal.rs` | `SignalKind` (Init, Tick, Tock, Crystal, Observe), `Signal` | **Implemented.** |
| `src/apache2/observe.rs` | `Observation` with `measure()` returning `Imperfect` | **Implemented.** |
| `src/apache2/identity.rs` | `Name`, `NamingLoss`, `BiasChain` | **Implemented.** |
| `src/apache2/init.rs` | `init_identity()` — reads `.mirror` files, derives bias chain | **Implemented.** |
| `src/apache2/loss.rs` | `InitLoss`, `ObserveLoss` + re-exports from terni | **Implemented.** |
| `src/memory.rs` | CLI memory commands (store, recall, crystallize, export, ingest, status) | **Implemented.** |
| `src/session.rs` | `.spectral/` directory management | **Implemented.** |
| `src/diff.rs` | Diff primitives | **Implemented.** |
| `src/log.rs` | Log formatting | **Implemented.** |
| `src/refs.rs` | Reference resolution | **Implemented.** |

#### BEAM side (`beam/`)

| File | What it is | Status |
|------|-----------|--------|
| `beam/src/conversation/runtime.gleam` | `converge(spec) -> List(Delta)` — evaluates Spec against BEAM state, returns StartProcess/UpdateState/StopProcess deltas | **Implemented.** Pattern matching stubs (wildcards always match, guards always apply). |
| `beam/src/conversation/protocol.gleam` | `Spec`, `Arm`, `Pattern`, `Op` — the protocol contract between AST and BEAM runtime | **Implemented.** |
| `beam/src/conversation/supervisor.gleam` | Static supervision tree: `@compiler -> garden`. RestForOne. | **Implemented.** |
| `beam/src/conversation/compiler.gleam` | Grammar compiler actor (NIF-backed) | **Implemented.** |
| `beam/src/conversation/garden.gleam` | Factory supervisor for domain servers | **Implemented.** |
| `beam/src/conversation/boot.gleam` | Boot orchestrator — loads garden files, compiles | **Implemented.** |
| `beam/src/domain_server.erl` | gen_server with `exec` primitive (apply/3) | **Implemented.** |

#### Spectral docs

| File | What it is |
|------|-----------|
| `docs/plan.md` | Two-tick plan. Tick 1 complete (mirror delivers). Tick 2 in progress (spectral consumes). |
| `docs/care-model.md` | Heath's care model. Clinical surface decisions waiting for Heike. |
| `docs/threat-model.md` | Seam's threat model. Five adversaries, five defenses, five gaps. |
| `docs/garden-surface.md` | The garden as a live surface. Grammar IS API IS demo. |
| `docs/specs/absorb-lens.md` | Absorb lens crate into spectral. |
| `docs/specs/glint-prism.md` | Glint: the linter that thinks. Interrupt privilege. |
| `docs/specs/sign-verify.md` | Dual signature (Ed25519 + spectral). |
| `docs/specs/spec-files.md` | `.spec` files — constraints, budgets, SLOs. |
| `docs/specs/tui-v0.md` | TUI v0 — readline prompt loop. |

#### Disabled dependencies

```toml
# Temporarily disabled in spectral/Cargo.toml:
# lens = { path = "../lens" }
# spectral-db = { path = "../spectral-db" }
# conversation = { path = "../conversation" }
```

These are disabled because fate needs prism-core migration. The dependency chain is broken at `fate -> prism-core`.

---

## 2. The @runtime Grammar

### What the architecture session established

```mirror
-- boot/04-actor.mirror already declares the actor abstraction:
abstract grammar @actor {
  abstract action start(actor) -> process
  abstract action send(process, message) -> imperfect
  abstract action stop(process) -> imperfect
}

-- boot/std/beam.mirror already declares the BEAM implementation:
grammar @beam {
  in @actor
  abstract action start(callbacks, state) -> imperfect(gen_server, beam_error, loss)
  abstract action call(gen_server, ref) -> imperfect(ref, beam_error, loss)
  abstract action cast(gen_server, ref) -> imperfect
  abstract action supervise([process], strategy) -> imperfect(supervisor, beam_error, loss)
  abstract action stop(process) -> imperfect
}
```

**The gap:** `@actor` defines the abstraction. `@beam` implements it for OTP. But there is no `@runtime` that bridges `@actor` to the process lifecycle that mirror and spectral need. And there is no `grammar @mirror` that extends a runtime.

Currently `@mirror` in `boot/std/mirror.mirror` declares only structural properties (unique_variants, deterministic, pure, etc.). It does not declare itself as a runtime. It does not have `spawn`.

### The @runtime grammar (new boot file)

```mirror
in @prism
in @meta
in @actor

-- the runtime contract: what any runtime must implement
-- a runtime turns actors into processes via spawn
-- spawn is the primitive. everything else composes from spawn.

type effect(actor)
type runtime(grammar)

abstract grammar @runtime {
  in @actor

  -- the single primitive: turn an actor + effect into a process
  abstract action spawn(actor, effect) => process

  -- observe a running process
  abstract action observe(process) -> imperfect

  -- stop a running process
  abstract action halt(process) -> imperfect
}
```

**Why `spawn` and not `start`:** `@actor.start` takes an actor and returns a process. That is the actor contract. `@runtime.spawn` takes an actor AND an effect — what the actor should DO. The runtime dispatches the effect. The actor is the identity. The effect is the behavior. The process is the result.

### grammar @mirror < @runtime (extend mirror.mirror)

```mirror
grammar @mirror < @runtime {
  in @meta
  in @property
  in @code

  -- mirror IS a runtime. it spawns these actors:

  -- the compiler: parse, resolve, emit
  action spawn(@compiler, compile(source)) => process

  -- the LSP: long-running, serves the editor
  action spawn(@lsp, serve(editor)) => process

  -- an agent: Claude Code subagent doing a task
  action spawn(@agent, task(description)) => process

  -- the CLI: one-shot command dispatch
  action spawn(@command, effect(args)) => process

  -- structural properties (already in mirror.mirror)
  requires unique_variants
  requires every_type_reachable
  requires no_dead_variants
  requires types_lowercase
  requires canonical_order

  invariant idempotent
  invariant deterministic
  invariant pure
  invariant no_cycles
  ensures always_halts
}
```

**The key insight:** mirror's CLI commands are already spawn dispatches in disguise.

- `mirror compile foo.mirror` = `@mirror.spawn(@compiler, compile("foo.mirror"))`
- `mirror lsp` = `@mirror.spawn(@lsp, serve(editor))`
- `mirror ai --commit` = `@mirror.spawn(@agent, task("commit"))`
- `mirror ca --merge` = `@mirror.spawn(@agent, task("merge"))`

The dispatch table in `cli.rs` is an untyped, hardcoded version of what `@runtime.spawn` would be if it were grammar-driven.

### grammar @spectral < @runtime

```mirror
grammar @spectral < @runtime {
  in @time
  in @code/spectral

  -- spectral IS a runtime. it spawns these actors:

  -- the tick/tock loop: advance the graph
  action spawn(@graph, tick(signal)) => process
  action spawn(@graph, tock()) => process

  -- the MCP server: long-running, serves Claude Code
  action spawn(@mcp, serve(project)) => process

  -- agent memory: store, recall, crystallize
  action spawn(@memory, effect(args)) => process

  -- the observation layer
  action spawn(@observer, observe(path)) => process
}
```

### grammar @beam < @runtime (already exists, needs `< @runtime`)

```mirror
grammar @beam < @runtime {
  in @actor

  -- @beam already implements the actor contract
  -- adding < @runtime formalizes: beam IS a runtime
  -- spawn delegates to start + supervision

  action spawn(actor, effect) => process {
    let callbacks = derive_callbacks(actor, effect)
    let state = initial_state(effect)
    @beam.start(callbacks, state)
  }
}
```

---

## 3. The Delta

### Delta 1: @runtime boot file

**What exists:** `04-actor.mirror` declares `abstract grammar @actor` with start/send/stop. `beam.mirror` declares `grammar @beam` with OTP primitives. No `@runtime` exists.

**What's needed:** A boot file declaring `abstract grammar @runtime` with `spawn(actor, effect) => process` as the single primitive.

**What to build:**
- New file: `boot/04a-runtime.mirror` (between actor and property in boot order)
- Contents: `in @actor`, `type effect(actor)`, `type runtime(grammar)`, `abstract grammar @runtime` with spawn/observe/halt
- Update: `DeclKind` does not need changes — `grammar`, `abstract`, `action`, `type` all parse already
- Update: `OpticOp::Subset` (`<`) already parses. The `grammar @mirror < @runtime` syntax needs the parser to recognize `<` after a grammar name as inheritance, not as a comparison. **This is the first real implementation task.**

**Parser work:** Currently `parse_decl` handles `grammar @name { ... }`. It needs to handle `grammar @name < @parent { ... }`. The `<` token is already in `OpticOp::Subset`. The parser needs to:
1. After reading the grammar name, check for `<`
2. If present, read the parent grammar name
3. Store it in `Form` (new field: `parent_grammar: Option<String>`)
4. Resolution checks that the parent grammar exists in the registry

This is a small parser change. The `Form` struct already has `grammar_ref` for actions — `parent_grammar` follows the same pattern.

### Delta 2: Mirror as a runtime (update mirror.mirror)

**What exists:** `boot/std/mirror.mirror` declares `grammar @mirror` with structural properties only. No actions. No spawn. No `< @runtime`.

**What's needed:** `grammar @mirror < @runtime` with spawn as the dispatch primitive.

**What to build:**
1. Update `boot/std/mirror.mirror` to add `in @runtime` and four spawn action declarations
2. Add `< @runtime` to the grammar declaration (requires Delta 1's parser change)
3. The CLI dispatch in `cli.rs` does not need to change immediately — it continues to work as a hardcoded match table. The grammar declaration makes the contract explicit. The grammar-driven dispatch is a future tick.

**What this does NOT require:**
- A process manager. Mirror spawns are currently function calls, not OS processes.
- A supervision tree. That comes when mirror runs on the BEAM.
- Async. The CLI is synchronous. The LSP will need async, but that is Delta 3.

### Delta 3: The LSP — what exists vs what's needed

**What exists:** `src/lsp/` contains grammar *generation* from tree-sitter — `mirror lsp learn @code/python` reads `node-types.json` and generates a `.mirror` file. This is NOT an LSP server. It is a one-shot tool that produces grammars.

**What's needed:** An actual Language Server Protocol implementation that serves the editor. This is `@mirror.spawn(@lsp, serve(editor))`.

**What the LSP needs from mirror:**
- `parse_form(source)` — already exists, returns `Imperfect<Form, Error, MirrorLoss>`
- `MirrorLoss` — already exists, maps directly to LSP diagnostics
- `ParseLoss.unrecognized` -> LSP warning diagnostics
- `ResolutionLoss.unresolved_refs` -> LSP error diagnostics
- `PropertyLoss.verdicts` -> LSP info/warning diagnostics
- `Convergence` -> LSP status bar information
- `DeclKind` + `OpticOp` -> completion items
- `Form` tree -> document symbols, go-to-definition, hover

**What the LSP needs from spectral:**
- Nothing for v0. The LSP is a mirror feature, not a spectral feature.
- v1: spectral-db integration for cross-file resolution (which grammar does `in @X` resolve to?)

**What to build:**
1. New module: `src/lsp/server.rs` — LSP JSON-RPC over stdio (same pattern as spectral's `serve.rs`)
2. `textDocument/didOpen` + `textDocument/didChange` -> `parse_form(source)` -> cache the `Imperfect` result
3. `textDocument/publishDiagnostics` <- `MirrorLoss` fields mapped to LSP Diagnostic
4. `textDocument/completion` <- `DeclKind` variants + registered grammar names from the registry
5. `textDocument/hover` <- `Form` node information
6. `textDocument/documentSymbol` <- `Form` tree
7. CLI: `mirror lsp` (no args) starts the server. `mirror lsp learn` (existing) generates grammars.

**The mapping (MirrorLoss -> LSP Diagnostic):**

| MirrorLoss field | LSP Diagnostic |
|-----------------|----------------|
| `parse.unrecognized[i]` | Warning at line `i.line`: "unrecognized keyword: {i.keyword}" |
| `resolution.unresolved_refs[i]` | Error: "unresolved reference: {i.0}" |
| `properties.verdicts[i]` where Failure | Error: "property {i.property} failed: {observation}" |
| `properties.verdicts[i]` where Partial | Warning: "property {i.property} partial (loss: {loss})" |
| `convergence == BudgetExhausted` | Error: "compilation budget exhausted" |
| `convergence == Oscillating(n)` | Warning: "oscillating between {n} attractors" |

**This is the most valuable delta.** An LSP that reports MirrorLoss as diagnostics turns every editor into a mirror-aware compiler. The grammar IS the language server. The loss IS the diagnostic.

### Delta 4: `mirror ai --commit`

**What exists:** `cli.rs` has `cmd_ai` which dispatches to a fate model. The `ai` command takes a model name and optional path.

**What's needed:** `mirror ai --commit` that:
1. Reads the current git diff
2. Compiles changed `.mirror` files to check holonomy
3. If holonomy is zero (crystal), generates a commit message from the changes
4. If holonomy is nonzero, reports the loss and refuses to commit

**What this needs from mirror:**
- `compile_source` — already exists
- `MirrorLoss.holonomy()` — already exists
- `git2` — already in deps
- Commit message generation — requires an LLM call or a template

**What to build:**
1. In `cmd_ai`, detect `--commit` flag
2. Use `git2` to read the working tree diff
3. For each changed `.mirror` file, compile and check holonomy
4. If all holonomy == 0: generate commit message, create commit
5. If any holonomy > 0: print the loss, exit 1

**The commit message template:**

```
<kind>: <summary from form names>

holonomy: 0.000 (crystal)
files: <list of changed .mirror files>
crystal: <crystal OID>
```

The kind is derived from the changed `DeclKind` variants: new types = "type", new grammar = "grammar", property changes = "property", etc.

### Delta 5: `mirror merge --ai` (currently `ca --merge`)

**What exists:** `cli.rs` has `cmd_ca_merge` (behind `ca --merge`). The help text says `ca <path> [--enforce] -- observe, suggest, enforce`.

**What's needed:** AI-assisted merge that:
1. Lists branches with their holonomy
2. For each branch, compiles its `.mirror` files and checks for conflicts
3. Reports which branches are crystal (safe to merge) vs partial (need review)
4. Optionally performs the merge for crystal branches

**What this needs from mirror:**
- `compile_source` — already exists
- `MirrorLoss.holonomy()` — already exists
- `git2` — already in deps
- Branch enumeration — `git2::Repository::branches()`

**What to build:**
1. Rename or alias: `mirror merge --ai` alongside `mirror ca --merge`
2. `git2::Repository::branches()` -> for each branch, compile `.mirror` files
3. Report: branch name, commits ahead, holonomy, conflicts (yes/no)
4. With `--apply`: fast-forward merge crystal branches, skip partial branches
5. With `--force`: merge everything, report combined holonomy

**The merge report:**

```
branch                  commits  holonomy  conflicts  status
glint/tick-1            3        0.000     no         crystal (safe to merge)
mara/spectral-db        7        0.034     no         partial (review needed)
seam/threat-model       2        0.000     no         crystal (safe to merge)
```

### Delta 6: Spectral consuming mirror as runtime

**What exists:** Spectral depends on `mirror` as a path dep. It uses `mirror::parse::Parse` for optic commands. It delegates `spectral mirror <cmd>` to the `mirror` binary. Its `Runtime` trait in `apache2/runtime.rs` has `tick` but no `spawn`.

**What's needed:**
1. Spectral's `Runtime` trait needs to align with `@runtime.spawn`
2. Spectral's MCP server needs to use the grammar-driven tool dispatch, not stub errors
3. The `tick`/`tock` commands need to wire through mirror's compilation pipeline

**What to build (in dependency order):**

1. **Update spectral's `Runtime` trait:**
   ```rust
   pub trait Runtime {
       type Actor;
       type Effect;
       type Process;
       type Error;
       type L: Loss;

       fn spawn(&mut self, actor: Self::Actor, effect: Self::Effect)
           -> Imperfect<Self::Process, Self::Error, Self::L>;
       fn observe(&self, process: &Self::Process)
           -> Imperfect<(), Self::Error, Self::L>;
       fn halt(&mut self, process: &Self::Process)
           -> Imperfect<(), Self::Error, Self::L>;
   }
   ```

2. **Wire the REPL through mirror:** `shard>` input -> `mirror::parse::Parse.trace(input)` -> print result. Currently prints "(not yet wired)".

3. **Wire tick/tock through mirror:** `spectral tick` -> compile all `.mirror` files in the session -> update the session's crystal OID. `spectral tock` -> check convergence (has the crystal OID stabilized?).

4. **Re-enable disabled deps:** This requires fixing the fate -> prism-core migration. The dependency chain is: `spectral -> lens -> spectral-db -> prism-core`. Lens is being absorbed (see `absorb-lens.md`). spectral-db and conversation are disabled.

### Summary: dependency order

```
1. Parser: grammar @name < @parent     (small parser change in mirror_runtime.rs)
2. Boot: 04a-runtime.mirror            (new boot file, abstract grammar @runtime)
3. Boot: update mirror.mirror          (grammar @mirror < @runtime + spawn actions)
4. Boot: update beam.mirror            (grammar @beam < @runtime)
5. LSP: src/lsp/server.rs              (MirrorLoss -> LSP Diagnostic mapping)
6. CLI: mirror ai --commit             (git diff -> compile -> holonomy check -> commit)
7. CLI: mirror merge --ai              (branch -> compile -> holonomy report -> merge)
8. Spectral: Runtime trait update      (spawn/observe/halt instead of tick)
9. Spectral: wire REPL                 (shard> -> mirror parse -> result)
10. Spectral: wire tick/tock           (compile .mirror files -> session crystal)
```

Items 1-4 are grammar-level. No Rust implementation changes needed beyond the parser.
Item 5 is the highest-value implementation work.
Items 6-7 are CLI features that compose existing pieces.
Items 8-10 are spectral consuming what mirror provides.

---

## What is real and what is aspiration

**Real:**
- The parser, the four-fold loss, the content-addressed pipeline, the boot kernel — all implemented and tested.
- The grammar hierarchy (`@prism -> @meta -> @actor -> @code -> @beam`) — declared in boot files, enforced by the resolver.
- The CLI dispatch — 17 commands, working, tested.
- The five optic commands — working end-to-end through spectral.
- The MCP server skeleton — protocol works, tool dispatch stubbed.
- The BEAM-side runtime — converge/delta/supervisor chain works, pattern matching stubbed.

**Aspiration (designed but not built):**
- Grammar inheritance (`grammar @X < @Y`) — the parser does not handle `<` after grammar names yet.
- `@runtime` as a grammar — does not exist as a boot file.
- `@mirror` as a runtime — the grammar says properties, not spawn.
- The LSP server — `src/lsp/` generates grammars, does not serve them.
- `mirror ai --commit` — the `ai` command exists but `--commit` is not implemented.
- `mirror merge --ai` — `ca --merge` exists as a dispatch target but the implementation is a stub.
- Grammar-driven CLI dispatch — the CLI is a hardcoded match table, not spawn dispatch.
- Spectral consuming mirror's Runtime — spectral has its own `Runtime` trait that doesn't match.

**The glass wall:**
- Mirror side: `.mirror` source -> `Form` tree -> `MirrorFragment` -> `Shatter` crystal. This pipeline is solid.
- Spectral side: stub commands, disabled deps, MCP tools returning "not yet wired".
- Between them: `mirror::parse::Parse` is the only bridge that works end-to-end.
- The BEAM side: supervision tree works, convergence engine works, but the NIF bridge (`conversation` crate) is disabled in spectral's Cargo.toml.

The loop does not close yet. The grammar declares it. The parser compiles it. The runtime does not execute it. The delta is the execution layer.

---

*The grammar IS the spec. The spec IS the delta. The delta IS the work.*
*What exists compiles. What's needed is declared. What to build is ordered.*
*One primitive: spawn. Everything else composes from spawn.*
