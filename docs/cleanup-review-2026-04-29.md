# Mirror Cleanup Review -- 2026-04-29

Review agent: Reed (sub-agent, spectral worktree)
Branch under review: `glint/observation-grammar` (current HEAD)
Main HEAD: `e84dcbb`

---

## What Spectral Sees

### Where It Worked

`spectral focus` successfully parses `.mirror` grammar files and extracts structural nodes. For each file it returns a flat list of typed declarations: `grammar:`, `type:`, `action-def:`, `in:` (dependency), `out:` (export), `focus:`, `project:`, `split:`, `shift:`, `settle:`, `fold:`, `template:`, and `default:`.

The richest output came from `boot/01-meta.mirror` (84 nodes), which correctly surfaced the full AST type hierarchy: expression, declaration, pattern, type_ref subtrees, plus all five prism operations mapped to sigils (`focus:type`, `project:ref`, `split:|`, `fold:<`, `shift:|>`, `settle:..`). This is genuinely useful for understanding the meta-grammar's structure at a glance.

The prism grammar files (`prism/*.mirror`) produced clean dependency graphs. Examples:
- `@prism` defines the five operations and six type families
- `@observation` depends on `@coincidence` and `@ai`, declares gap/holonomy/dark_vector
- `@coincidence` depends on `@topology` and `@property`, declares measurement/verdict/spectrum
- `@topology` depends on `@property` and `@actor`
- `@gestalt` (in `prism/gestalt/gestalt.mirror`) depends on `@prism`, declares domain/node/gestalt/line/annotation/lens/meta

The fixture files also parsed. `fixtures/settle/composition.mirror` returned `fold:structure`, `prism:eigenvalues`, `traversal:nodes`, `lens:transform` -- the five-operation vocabulary surfacing from test data.

`spectral status` returned: 38 nodes, 301 edges, 0 crystals, loss 0.000 bits, tension 0.0647.
`spectral loss` returned: no loss data yet, fiedler 0.0733.
`memory_status` (MCP): 942 nodes, 2340 edges, 0 crystals.

### Where It Fell Short

1. **Rust source files produce nothing.** `spectral focus /path/to/lib.rs` returns "parse error: no recognized declarations found." This is the biggest gap. The tool that is supposed to help understand code structure cannot parse the language the project is written in. For a code review, this means spectral is blind to the actual implementation -- it only sees the grammar layer.

2. **`spectral project` returned identical output to `spectral focus`.** On `boot/01-meta.mirror`, both commands produced the same 84 nodes in the same format. The projection operation did not visibly filter, group, or prioritize anything. Either projection is not yet differentiated from focus, or the projection criteria are identity for this input.

3. **No cross-file analysis.** Each `spectral focus` call is a single-file parse. There is no way to ask "show me all grammars that depend on @actor" or "which types are defined but never referenced." The dependency graph is implicit in the `in:` declarations, but spectral doesn't traverse it. I had to manually read each file and mentally assemble the dependency tree.

4. **No Rust-to-grammar correlation.** The grammars declare types and actions. The Rust code implements them. There is no way to ask spectral "which grammar types have Rust implementations" or "which Rust modules correspond to which grammars." This is the gap between the grammar layer and the implementation layer.

5. **`gestalt_detect` was denied by permission.** Could not test directory-level analysis.

6. **`graph_query` was denied by permission.** Could not test pipe-forward queries against the concept graph.

7. **`prism/gestalt/document.mirror` failed to parse** with "unterminated block" -- a syntax issue in the grammar file itself.

8. **`prism/beam.mirror` and `prism/git.mirror` produced degenerate output** -- 1 node each (`type:` with empty value). These files appear to be stubs or have parse issues.

---

## Branch Inventory

| Branch | +ahead | -behind | Status | Contents |
|--------|--------|---------|--------|----------|
| `glint/observation-grammar` * | 0 | 0 | current, dirty | @gestalt hierarchy + check command (already merged to main) |
| `break/crypto` | 23 | 230 | stale, diverged | ECDLP spectral attack experiments -- 10 experiments, all negative results |
| `reed/prism-migration` | 24 | 230 | stale, diverged | Same crypto work + prism trait migration attempt |
| `reed/mirror-new` | 19 | 16 | active, close to main | `mirror new` + `mirror run` + Fate AI + tournament selection |
| `reed/inline-action-bodies` | 7 | 389 | stale | `in @domain` inline body parsing |
| `reed/syntactic-sugar-bootstrap` | 6 | 408 | stale | `extends` + construct-def syntax |
| `reed/gestalt-mirror-support` | 6 | 215 | stale | .mirror file discovery + visibility export enforcement |
| `mara/identity-keys-phase-0-1` | 5 | 111 | stale | Identity type + boot renumber |
| `mara/mirror-init-spec` | 5 | 85 | stale | `mirror init .` + LAPACK store init + test speedup |
| `reed/sha512-key-derivation` | 3 | 395 | stale | SHA256 -> SHA512 for domain keys |
| `taut/shared-boot-store` | 3 | 85 | stale | Zero parse holonomy + std library types |
| `taut/zero-parse-holonomy` | 3 | 85 | stale | Same as shared-boot-store minus 1 commit |
| `mara/minimum-viable-keywords` | 2 | 85 | stale | Keyword research + std library types |
| `mara/sel-license-properties` | 2 | 111 | stale | SEL property checks -- 4 violation detectors |
| `reed/actor-type-surface` | 2 | 450 | stale | Self-testing .conv packages |
| `reed/emit-code` | 1 | 10 | close to main | NL compound decomposition fix |
| `reed/emit-code-phase1` | 1 | 44 | stale | IoList type + stub emitter |
| `reed/first-compiler-commit` | 1 | 408 | stale | Historical: first compiler commit |
| `reed/garden-absorbs-packages` | 1 | 408 | stale | Same historical commit |
| `reed/prism-migration-2026-04-08` | 1 | 191 | stale | Prism trait migration |
| `reed/witness-ci-sign-encrypt` | 1 | 133 | stale | CI sign/encrypt integration test |
| `mara/prism-core-bridge` | 1 | 44 | stale | MirrorFragment uses Addressable |
| `mara/std-library` | 1 | 85 | stale | 9 standard library types |
| `agent/narcissus/...` | 0 | 9 | merged | Delete |
| `glint/boot-reorg` | 0 | 85 | merged | Delete |
| `glint/cogito-grammar` | 0 | 5 | merged | Delete |
| `mara/ca-merge` | 0 | 113 | merged | Delete |
| `mara/error-surface-spec` | 0 | 99 | merged | Delete |
| `mara/git-store-integration` | 0 | 65 | merged | Delete |
| `mara/spec-codegen` | 0 | 52 | merged | Delete |
| `reed/fold-operator` | 0 | 83 | merged | Delete |
| `reed/imperfect-dispatch` | 0 | 44 | merged | Delete |
| `reed/star-rename` | 0 | 6 | merged | Delete |
| `seam/block-unrecognized-loss` | 0 | 27 | merged | Delete |
| `seam/imperfect-parser` | 0 | 103 | merged | Delete |
| `seam/kill-form` | 0 | 58 | merged | Delete |
| `taut/kill-form` | 0 | 48 | merged | Delete |
| `remotes/origin/glint/mirror-optic` | 0 | 0 | remote-only, merged | Delete |

**14 branches are fully merged and should be deleted.**

---

## Uncommitted Work

On `glint/observation-grammar`, 13 files modified + 13 untracked source files + various untracked docs/configs.

### Modified (worth preserving):

- **`prism/ai.mirror`** -- Adds observation/proposal/crystal types and observe/propose/decide actions to @ai grammar. Clean extension. +39 lines.
- **`src/eigentest.rs`** -- Adds `eigentest_ai_grammar_parses_and_runs` test with good commentary about AST star-shape vs type-graph shape. +39 lines.
- **`src/mirror_runtime.rs`** -- Major rework: +219/-85. Without reading the full diff, this is substantial.
- **`src/emit_code.rs`** -- +38/-38 refactor.
- **`src/lambda_phases.rs`** -- +92 refactor.
- **`src/mirror_ast.rs`** -- +99 refactor.
- **`src/nl/*`** (compound.rs, mod.rs, stop_words.rs, token.rs) -- NL pipeline improvements. stop_words.rs alone adds 162 lines.
- **`tests/nl_integration.rs`** -- Updated for NL changes.
- **`src/lib.rs`** -- Adds `pub mod mirver;` (wires the mirver module).
- **`src/loss.rs`** -- Minor change (+5/-1).

### Untracked source files (NOT wired in lib.rs except mirver):

| File | Lines | Purpose |
|------|-------|---------|
| `src/compile.rs` | 451 | Compilation pipeline |
| `src/ffi.rs` | 852 | BEAM FFI bridge |
| `src/identity.rs` | 66 | Identity type |
| `src/license.rs` | 645 | SEL license checking |
| `src/matrix.rs` | 467 | Matrix operations |
| `src/mirver.rs` | 358 | Mirror versioning (wired via uncommitted lib.rs change) |
| `src/packages.rs` | 801 | Package registry |
| `src/resolve.rs` | 2875 | Name resolution (largest untracked file) |

These total ~5,515 lines of untracked, unwired Rust. They are either in-progress work from branches that were not fully merged, or preparatory modules for future features. Most are not compiled at all (not in lib.rs), so they could contain any number of errors.

### Verdict

The modified files represent real, coherent work. The ai.mirror extension and eigentest addition are clean and should be committed. The mirror_runtime/lambda_phases/mirror_ast changes need review (large refactor, hard to assess without detailed diff reading). The NL improvements look like genuine capability additions.

The untracked source files are a concern -- 5,500 lines of dead code sitting in the working tree. They should either be committed to a feature branch or removed.

---

## Compile Failures

**`cargo check` passes.** The library compiles without errors (only upstream warnings in fragmentation/fragmentation-git).

**`cargo test` fails to compile 3 test files:**

| Test file | Error | Fix difficulty |
|-----------|-------|---------------|
| `tests/butterfly.rs` | `cannot find function enumerate_curve in module super::curve` -- missing function in the curve module within the test | Easy: add the missing function or fix the reference |
| `tests/spectral_break.rs` | 8x `use of unresolved module spectral_db` + 3x type annotation errors | Medium: spectral_db was removed as a dependency. These tests need rewriting to use internal types (e.g., `mirror::loss::Convergence`) |
| `tests/first_ca_task.rs` | `couldn't read tests/../mirror.shatter: No such file or directory` | Easy: regenerate `mirror.shatter` via `mirror crystal mirror.shatter`, or delete the test if the artifact is no longer part of the workflow |

**`cargo test --test cli` -- 1 failure:**
- `compile_shatter_produces_oid` -- same root cause: `mirror.shatter` file is missing. The `.gitignore` does not exclude `.shatter` files, and a prior commit (`c6628db`) says "Delete and ignore .shatter files" but the gitignore was never updated.

**All other tests pass: 721 lib tests + 43 integration tests across 16 test files.**

---

## Grammar Inventory

### Boot grammars (loaded by compiler, order matters)

| File | Grammar | Depends on | Nodes | Notes |
|------|---------|-----------|-------|-------|
| `boot/00-prism.mirror` | `@prism` | (root) | 9 | Five operations as trait methods |
| `boot/01-meta.mirror` | `@meta` | `@prism` | 84 | Full AST type hierarchy -- the big one |
| `boot/01a-meta-actor.mirror` | `@actor` | `@prism`, `@meta` | 12 | actor/state/process/message |
| `boot/01b-meta-action.mirror` | `@action` | `@prism`, `@meta`, `@actor` | 9 | action/collapse types |
| `boot/01c-meta-io.mirror` | `@io` | `@prism`, `@meta`, `@actor` | 17 | effect/mut/path/content/channel |
| `boot/02-shatter.mirror` | `@shatter` | `@prism`, `@meta`, `@io` | 5 | Output format |
| `boot/03-code.mirror` | `@code` | `@prism`, `@meta` | 16 | LSP types: position/range/diagnostic |
| `boot/03a-code-rust.mirror` | `@code/rust` | `@code` | 3 | Rust code generation |
| `boot/03b-code-gleam.mirror` | `@code/gleam` | `@code` | 3 | Gleam code generation |
| `boot/04a-runtime.mirror` | `@runtime` | `@prism`, `@meta`, `@actor` | 9 | effect/runtime |
| `boot/05-property.mirror` | `@property` | (self-contained) | 6 | verdict/property_error/effect_pattern |
| `boot/06-package.mirror` | `@package` | `@prism`, `@meta` | 17 | version/semver/mirver/package |
| `boot/06a-package-git.mirror` | `@package/git` | `@package` | -- | Not tested |
| `boot/06b-package-spec.mirror` | `@package/spec` | `@package` | -- | Not tested |

### Boot standard library (`boot/std/`)

| File | Grammar | Depends on |
|------|---------|-----------|
| `beam.mirror` | `@beam` | `@prism`, `@meta`, `@actor` |
| `benchmark.mirror` | `@benchmark` | -- |
| `cli.mirror` | `@cli` | `@prism`, `@meta`, `@code`, `@spec`, `@shatter` |
| `file.mirror` | `@file` | -- |
| `mirror.mirror` | `@mirror` | -- |
| `properties.mirror` | `@properties` | -- |
| `runtime.mirror` | `@runtime` (std) | -- |
| `rust.mirror` | `@rust` | -- |
| `time.mirror` | `@time` | -- |
| `tui.mirror` | `@tui` | `@prism`, `@meta`, `@config`, `@code`, `@ci`, `@ca`, `@lsp`, `@time` |

### Prism grammars (domain-specific, in `prism/`)

| File | Grammar | Depends on | Wired to Rust? |
|------|---------|-----------|---------------|
| `prism.mirror` | `@prism` | (root) | Yes -- `src/prism.rs` |
| `actor.mirror` | `@actor` | `@reality` | Via boot grammar |
| `ai.mirror` | `@ai` | `@actor` | Eigentest only |
| `beam.mirror` | `@beam` | -- | Stub (1 node) |
| `ca.mirror` | `@ca` | `@ci` | No |
| `ci.mirror` | `@ci` | `@beam` | No |
| `cogito.mirror` | `@cogito` | `@ai` | No |
| `coincidence.mirror` | `@coincidence` | `@topology`, `@property` | Via coincidence crate |
| `compiler.mirror` | `@compiler` | `@actor` | No |
| `fate.mirror` | `@fate` | `@actor` | No |
| `gestalt.mirror` | `@gestalt` | `@language` | Superseded by `gestalt/gestalt.mirror` |
| `gestalt/gestalt.mirror` | `@gestalt` | `@prism` | Via spectral |
| `gestalt/document.mirror` | `@gestalt/document` | `@gestalt` | Parse error (unterminated block) |
| `gestalt/memory.mirror` | `@gestalt/memory` | `@gestalt` | Via spectral MCP |
| `git.mirror` | `@git` | -- | Stub (1 node) |
| `language.mirror` | `@language` | `@actor` | No |
| `mail.mirror` | `@mail` | -- | No |
| `nix.mirror` | `@nix` | `@beam` | No |
| `observation.mirror` | `@observation` | `@coincidence`, `@ai` | No |
| `projection.mirror` | `@projection` | `@property`, `@coincidence`, `@compiler` | No |
| `property.mirror` | `@property` | `@compiler` | Via boot grammar |
| `reality.mirror` | `@reality` | `@prism` | No |
| `topology.mirror` | `@topology` | `@property`, `@actor` | No |

### Staging grammars (`.staging/boot/`)

Draft versions of identity, identity-keys, actor, action, and property grammars. Not compiled.

### Fixtures (`fixtures/`)

~30 `.mirror` files used by tests. Covering: settle, fold, lens, prism, traversal, iso, noop, escalate, extractive patterns. All test-only.

### Test fixtures (`tests/fixtures/`)

8 `.mirror` files: cli_flags, error_cases, identity, property, purity, recover, sigil, subset.

---

## What's Valuable

### Load-bearing for spectral

1. **The parser** (`src/parse.rs`, `src/mirror_ast.rs`, `src/mirror_runtime.rs`) -- This is the core. It parses `.mirror` files into ASTs that spectral consumes. Everything depends on it.

2. **The eigentest** (`src/eigentest.rs`) -- Graph property verification (star detection, bipartiteness, connectivity). Used by `spectral check` and the `check` CLI command.

3. **The kernel** (`src/kernel.rs` + `kernel/` module) -- Content addressing (Oid, Trace, Vector, ContentAddressed). Foundation for the entire content-addressed store.

4. **The store** (`src/store.rs`, `src/shard.rs`) -- Content-addressed storage. Shards, MirrorOid, loss tracking.

5. **The lambda pipeline** (`src/lambda_phases.rs`) -- Parse.then(Resolve).then(Properties).then(Emit). This is the compiler pipeline that spectral's grammar processing depends on.

6. **The boot grammars** (`boot/00-prism.mirror` through `boot/06-package.mirror`) -- These define the type system that the compiler validates against.

7. **The prism grammars** (`prism/coincidence.mirror`, `prism/observation.mirror`, `prism/gestalt/`) -- These define the vocabulary spectral uses for its graph analysis.

8. **The NL module** (`src/nl/`) -- Natural language tokenization for concept extraction. Used by spectral's memory system.

9. **The loss module** (`src/loss.rs`) -- Shannon loss calculation. Core metric.

### Speculative / not yet integrated

1. **BEAM/FFI** (`src/ffi.rs`, `beam/`, BEAM-related grammars) -- Elixir integration. Not compiled. Future direction.
2. **Code emission** (`src/emit_code.rs`, `src/generate_crate.rs`) -- Rust/Gleam code generation from grammars. Partially wired.
3. **License checking** (`src/license.rs`) -- SEL property verification. Not wired.
4. **Package management** (`src/packages.rs`, `src/resolve.rs`) -- Name resolution and package registry. Not wired. 2,875 lines of unwired resolve.rs.
5. **Matrix operations** (`src/matrix.rs`) -- Standalone matrix math. Not wired.
6. **Crypto experiments** (`tests/butterfly.rs`, `tests/spectral_break.rs`, `tests/crypto_break.rs`) -- ECDLP spectral attack research. All negative results. crypto_break passes; the other two don't compile.
7. **`@fate`, `@cogito`, `@compiler`, `@language`** grammars -- Declared but not implemented in Rust.
8. **`@mail`, `@nix`** grammars -- Aspirational. No implementation.
9. **Staging grammars** (`.staging/`) -- Draft boot grammars for identity/actor/action/property. Not compiled.

---

## What's Stale or Outdated

### Branches (delete immediately -- fully merged)
14 branches with 0 commits ahead of main. See Branch Inventory table above.

### Branches (archive or delete -- too far behind)
- `reed/first-compiler-commit` -- 408 behind, 1 ahead (historical commit)
- `reed/garden-absorbs-packages` -- 408 behind, same historical commit
- `reed/actor-type-surface` -- 450 behind, 2 ahead (old .conv format)
- `reed/syntactic-sugar-bootstrap` -- 408 behind, 6 ahead (extends syntax, old format)
- `reed/sha512-key-derivation` -- 395 behind, 3 ahead (SHA change likely superseded)
- `reed/inline-action-bodies` -- 389 behind, 7 ahead (inline body parsing, old format)

### Files
- `mirror.shatter` -- deleted but tests still reference it. Either regenerate or fix tests.
- `erl_crash.dump` -- 1.9MB crash dump from Erlang. Should be in `.gitignore` (it is, but the file exists).
- `.staging/` -- Draft grammars that have been superseded by the boot/ versions. Review and delete or integrate.
- `prism/gestalt.mirror` -- Superseded by `prism/gestalt/gestalt.mirror` (the file itself says so in a comment).
- `prism/beam.mirror` and `prism/git.mirror` -- Stubs producing 1 degenerate node each.
- `CLI_AI_REPORT.md`, `CLI_REPORT.md`, `DOCS_REPORT.md`, `MIRROR_RUNTIME_REPORT.md`, `SPRING_CLEAN_REPORT.md` -- One-off reports from prior sessions. Not documentation. Either move to `docs/reviews/` or delete.
- `target-break/` -- Appears to be a separate build target directory from the `break/crypto` branch. Should be in `.gitignore`.
- `boot.wat/` -- Empty/minimal WebAssembly directory. Purpose unclear.

### Untracked source files not wired in lib.rs
`src/compile.rs`, `src/ffi.rs`, `src/identity.rs`, `src/license.rs`, `src/matrix.rs`, `src/packages.rs`, `src/resolve.rs` -- 5,500 lines of dead code. Either commit to a feature branch or delete from the working tree.

---

## Recommended Plan

### Fix first (immediate)

1. **Delete 14 fully-merged branches.** No information loss.
   ```
   git branch -d agent/narcissus/cc7ce70c-6d20-4b41-84c4-b9264ea9b3ff glint/boot-reorg glint/cogito-grammar mara/ca-merge mara/error-surface-spec mara/git-store-integration mara/spec-codegen reed/fold-operator reed/imperfect-dispatch reed/star-rename seam/block-unrecognized-loss seam/imperfect-parser seam/kill-form taut/kill-form
   ```

2. **Fix the 3 broken test files.**
   - `tests/first_ca_task.rs`: Either regenerate `mirror.shatter` or delete the test (the artifact workflow may be dead).
   - `tests/butterfly.rs`: Add the missing `enumerate_curve` function or fix the module reference.
   - `tests/spectral_break.rs`: Replace `spectral_db::scheduler::Convergence` with `mirror::loss::Convergence`. Fix the 3 type annotation errors (add explicit `f64` annotations on the `.max()` calls).

3. **Add to `.gitignore`:** `*.shatter`, `target-break/`, `erl_crash.dump` (already listed but verify).

4. **Fix `prism/gestalt/document.mirror`** -- unterminated block parse error.

### Merge next

5. **`reed/emit-code`** -- Only 1 commit ahead, 10 behind. NL compound decomposition fix. Cherry-pick.

6. **Commit the uncommitted work on `glint/observation-grammar`** -- The ai.mirror extension, eigentest addition, NL improvements, and mirver wiring are all coherent. The mirror_runtime/lambda_phases/mirror_ast refactor needs review but compiles clean.

7. **`reed/mirror-new`** -- 19 ahead, 16 behind. The `mirror new` + `mirror run` + Fate AI work is the most substantive unmerged feature branch. Needs rebase and review.

### Archive or delete

8. **Archive these branches** (tag them if the work matters, then delete):
   - `break/crypto` -- 23 commits of ECDLP research. All negative results are documented in RESULTS.md on main. The test files are the unique content. Tag as `archive/crypto-experiments` if desired.
   - `reed/prism-migration` -- Same crypto work + migration attempt. Superseded.
   - `reed/gestalt-mirror-support` -- Visibility export enforcement, 6 commits, 215 behind. Ideas may be worth extracting but the branch is too stale to merge.
   - `mara/identity-keys-phase-0-1` -- Identity type work, 5 commits, 111 behind.

9. **Delete without ceremony** (too old, too diverged, content is historical):
   - `reed/first-compiler-commit`, `reed/garden-absorbs-packages`, `reed/actor-type-surface`, `reed/syntactic-sugar-bootstrap`, `reed/sha512-key-derivation`, `reed/inline-action-bodies`, `reed/emit-code-phase1`, `reed/prism-migration-2026-04-08`, `reed/witness-ci-sign-encrypt`
   - `mara/prism-core-bridge`, `mara/sel-license-properties`, `mara/std-library`, `mara/minimum-viable-keywords`, `mara/mirror-init-spec`
   - `taut/shared-boot-store`, `taut/zero-parse-holonomy`

### Leave alone

10. **`glint/observation-grammar`** (current branch) -- after committing the uncommitted work, this branch is at main. Can be deleted after merge.

11. **The untracked source files** -- decide per-file: `src/resolve.rs` (2,875 lines) is the largest risk. Either it's needed for a feature or it's dead weight. Same for `src/ffi.rs` (852 lines) and `src/license.rs` (645 lines). These should be committed to feature branches or deleted from the working tree. They are invisible to git and at risk of loss.

### Cleanup the report files

12. Move `CLI_AI_REPORT.md`, `CLI_REPORT.md`, `DOCS_REPORT.md`, `MIRROR_RUNTIME_REPORT.md`, `SPRING_CLEAN_REPORT.md` to `docs/reviews/` or delete them.

---

## What Would Make Spectral More Useful Here

This section is direct product feedback from using spectral as a code review tool.

### 1. Parse Rust (and other source languages)

The single biggest limitation. `spectral focus` only handles `.mirror` files. For a Rust project, this means spectral is blind to ~95% of the codebase. Even a minimal Rust parser that extracted `pub mod`, `pub fn`, `pub struct`, `pub enum`, `impl Trait for Type`, and `use` declarations would make spectral useful for understanding code structure. The grammar layer is 10% of the project; the implementation layer is 90%.

### 2. Cross-file dependency queries

I wanted to ask: "What does @actor depend on, transitively?" and "Which grammars have no Rust implementation?" These are graph queries over the grammar dependency DAG. The `in:` declarations give the edges. Spectral should be able to traverse them.

Concretely, I wanted:
```
spectral query "find grammar |> walk in |> where root = @actor"
spectral query "find grammar |> where not has_impl"
```

### 3. Differentiate `focus` from `project`

Both returned identical output. `project` should filter: show only types, or only actions, or only dependencies. Something like:
```
spectral project --types /path/to/file.mirror
spectral project --deps /path/to/file.mirror
```

### 4. Directory-level analysis without MCP

`spectral focus /path/to/directory/` should work. Parse all `.mirror` files in a directory, build the combined graph, show the dependency tree. I had to `find | xargs` manually.

### 5. Output format options

The current output is a flat list of `kind:value` pairs. For review work, I needed:
- **Tree format** showing the grammar hierarchy (grammars -> types -> variants)
- **DOT format** for dependency visualization
- **JSON format** for programmatic analysis
- **Diff format** showing what changed between two versions of a grammar

### 6. Grammar validation

`spectral check` should catch what I found manually:
- `prism/gestalt/document.mirror` has an unterminated block
- `prism/beam.mirror` and `prism/git.mirror` are degenerate stubs
- `prism/gestalt.mirror` is superseded by `prism/gestalt/gestalt.mirror` (duplicate grammar name)

### 7. Integration with git

I fell back to `git log`, `git diff`, `git branch` for everything structural about the project. Spectral should be able to answer:
- "Which .mirror files changed since commit X?"
- "Which grammars were affected by this branch?"
- "Show me the grammar diff between main and this branch"

The `mirror.spec` file already declares `store { path = .git/mirror }` -- the git integration intent is there. Surfacing it in spectral's CLI would make it genuinely useful for review workflows.

### 8. Loss/tension should be meaningful without prior context

`spectral status` showed tension 0.0647 and loss 0.000. `spectral loss` showed fiedler 0.0733. These numbers mean nothing to a reviewer without context. What's a good tension? What does this fiedler value indicate about the graph? Even a qualitative label ("low tension", "well-connected", "sparse") would help.

### 9. Memory recall needs entry points

`memory_recall` requires an OID. But I don't know any OIDs. There should be a way to recall by concept name, file path, or grammar name. Something like:
```
spectral recall @actor
spectral recall /path/to/file.rs
```

### Summary of tool usage

| Task | Tool used | Should have been spectral? |
|------|-----------|--------------------------|
| Parse grammar structure | spectral focus | Yes (worked) |
| Parse Rust structure | grep + Read | Yes (spectral can't) |
| Dependency graph | Manual assembly from spectral output | Yes (spectral should traverse) |
| Branch inventory | git branch + git log | Maybe (git integration) |
| Compile status | cargo check/test | No (cargo's job) |
| File discovery | find, ls, Glob | Yes (spectral should scan dirs) |
| Uncommitted changes | git status + git diff | Maybe (git integration) |
| Cross-referencing grammar to Rust | grep | Yes (spectral should correlate) |
