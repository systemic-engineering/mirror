# kintsugi self-hosting spec

`mirror kintsugi src/ boot/ --target @mirror --out boot.mirror/`

The command that takes the mirror compiler — Rust source and boot grammars —
and produces a pure-mirror equivalent. Kintsugi navigates the space of all
implementations satisfying the same contracts (in/out boundaries) and finds
the fiber closest to the ground state.

This spec maps what exists, what's missing, and the critical path.

---

## 1. What EXISTS

### Dependencies (Cargo.toml)

External crates used by the mirror binary:

| Crate | Purpose | @io boundary? |
|-------|---------|---------------|
| `prism-core` | Five operations, Lambda, Loss, Oid, Beam, Optic, Metal | **Stays in Rust** — the substrate |
| `fragmentation` | Content-addressed fragment trees, Encode/Decode, Repo | **Stays in Rust** — storage substrate |
| `fragmentation-git` | Git-native SHA-1 bridging | **@io** — git |
| `coincidence` | CoincidenceHash spectral hashing | **Stays in Rust** — math substrate |
| `serde_json` | JSON serialization | **@io** — serialization |
| `sha2` | SHA-256 hashing | **Stays in Rust** — crypto primitive |
| `hex` | Hex encoding/decoding | Can be grammar (trivial) |
| `age` | Age encryption (SSH keys) | **@io** — crypto |
| `base64` | Base64 encoding | Can be grammar (trivial) |
| `eetf` | Erlang External Term Format | **@io** — BEAM interop |
| `git2` | libgit2 bindings | **@io** — git |
| `ssh-key` | SSH key handling | **@io** — crypto |
| `unicode-segmentation` | Unicode grapheme clustering | **@io** — text processing |
| `rust-stemmers` | Natural language stemming | **@io** — NLP |

---

## 2. Component Map

### mirror_ast.rs (src/mirror_ast.rs)
**Contract:** in: nothing (type definitions), out: `MirrorAST` enum (7 variants), `Identifier`, `GrammarRef`, `TypeBody`, `Field`, `AbstractDefault`
**Status:** stays in Rust
**Current:** The 7-variant AST: Focus, Project, Split, Zoom, Refract, Abstract, Module. Plus Identifier (not String), GrammarRef (always @-prefixed), TypeBody (Enum/Struct/Alias/Unit), Field (name+type_ref). Implements Encode/Decode, content_oid via CoincidenceHash, MerkleTree for prism, Addressable.
**Target:** This IS the substrate. The AST definition stays in Rust because it IS the @io boundary between mirror grammars and the Rust type system. The five operations are defined here structurally.
**Gap:** None. This is the glass wall. It stays.
**Migration:** N/A — this is the anchor point everything else migrates toward.

---

### kernel.rs (src/kernel.rs)
**Contract:** in: bytes, out: `Oid` (content address), `TraceOid`, `Trace<T,E>`, `Vector<A,B>`, `Composed`, `Latent`, `ContentAddressed` trait, `Setting` trait
**Status:** stays in Rust
**Current:** Content addressing via CoincidenceHash<3>. OidHasher for incremental hashing. Vector trait for composable transformations with traced results. Latent for cached evaluation backed by fragmentation Repo. ContentAddressed impls for String, Vec, Option, tuple, i32, f64, Fractal, Prism, serde_json::Value.
**Target:** Stays. This is the vector algebra substrate. Content addressing, composition, and caching are fundamental operations that define the runtime.
**Gap:** None.
**Migration:** N/A — substrate.

---

### declaration.rs (src/declaration.rs)
**Contract:** in: `MirrorAST`, out: `MirrorFragment` (content-addressed AST tree), `OpticOp` (operator tokens)
**Status:** can be grammar NOW
**Current:** OpticOp enum (Iso, Fold, Split, Focus, Zoom, Refract, Subset, Superset, NotIso, Unfold) with from_token/as_str. MirrorFragment type alias (`Fractal<MirrorAST>`). MirrorFragmentExt trait for accessing ast/children/hash. `fragment()` helper builds Fractal from AST + children.
**Target:** OpticOp is already declared in `boot/01-meta.mirror` as operator tokens. The fragment construction is thin glue over fragmentation crate types.
**Gap:** OpticOp's from_token/as_str mapping is pure logic that could be a grammar lookup table. The fragment() constructor is 10 lines of Fractal construction — stays as @io glue.
**Migration:**
1. OpticOp from_token → grammar mapping table in `@meta` (already partially there)
2. MirrorFragment/MirrorFragmentExt → stays as Rust type alias + trait (thin @io)
3. fragment() constructor → stays as @io (calls fragmentation crate)

---

### loss.rs (src/loss.rs)
**Contract:** in: compilation events, out: `MirrorLoss` (four-fold compilation trace), `ParseLoss`, `ResolutionLoss`, `PropertyLoss`, `EmitLoss`, `Convergence`, `ParseWarning`, `PhaseRecord`
**Status:** can be grammar NOW
**Current:** MirrorLoss is a structured Loss implementation with four sub-losses (parse, resolution, properties, emit) plus convergence status, crystal OID, and recovered flag. Implements the prism Loss trait (zero, total, is_zero, combine). holonomy() sums all sub-losses plus convergence penalty. Each sub-loss has its own holonomy/is_zero/combine.
**Target:**
```
# @mirror/loss grammar
grammar @mirror/loss {
  in @prism
  in @meta

  type parse_warning = unknown_token | deprecated_kind | missing_name | duplicate_name | unresolved_parent | malformed_operator
  type parse_loss { warnings: [parse_warning] }
  type resolution_loss { unresolved_refs: [(text, oid)], resolution_ratio: f64 }
  type property_loss { verdicts: [property_verdict] }
  type emit_loss { phases: [phase_record], staleness: u32, dark_dims: [u32] }
  type convergence = converging(u32) | settled | oscillating(u32) | budget_exhausted

  type mirror_loss {
    parse: parse_loss,
    resolution: resolution_loss,
    properties: property_loss,
    emit: emit_loss,
    convergence: convergence,
    crystal: option(oid),
    recovered: bool,
  }

  action holonomy(mirror_loss) -> f64 {
    parse.holonomy + resolution.holonomy + properties.holonomy + emit.holonomy + convergence_penalty
  }
}
```
**Gap:** The Loss trait impl (zero, total, is_zero, combine) needs the prism crate's trait. The grammar can declare the types and the holonomy logic. The trait impl stays as thin Rust glue.
**Migration:**
1. Declare all loss types in `@mirror/loss` grammar
2. Implement holonomy/combine as grammar actions
3. Keep `impl Loss for MirrorLoss` as 20 lines of Rust delegating to grammar

---

### lambda_phases.rs (src/lambda_phases.rs)
**Contract:** in: `SourceText(String)`, out: `ParsedAst`, `ResolvedAst`, `CheckedAst`, `EmittedCode(String)`. Pipeline: Parse -> Resolve -> Properties -> Emit.
**Status:** can be grammar NOW
**Current:** Six phase structs (Parse, Resolve, Emit, Kintsugi, Strict, Properties) each with `#[derive(DeriveLambda)]` and `#[oid("@X")]`. Three static pipelines (CRAFT, KINTSUGI_PIPELINE, CI). Typed LambdaFn implementations for Parse, Resolve, Properties, Emit with Input/Output type checking.
**Target:** This is ALREADY a grammar — it's declared in `boot/std/craft.mirror`:
```
grammar @craft {
  craft(target) -> crystal {
    focus(target) |> split |> zoom |> refract |> project
  }
}
```
The Rust structs are the substrate implementation of what the grammar declares. The phase composition IS the grammar composition.
**Gap:** The LambdaFn::reduce implementations contain the actual work: Parse calls parse_form, Resolve opens MirrorRegistry, Properties is pass-through, Emit calls emit_code_fragment. These are @io calls. The composition logic (`.then()`) is in prism-core.
**Migration:**
1. Phase declarations → already grammar (`@craft`)
2. Pipeline composition → already grammar (`focus |> split |> zoom |> refract |> project`)
3. LambdaFn::reduce implementations → stay as Rust @io (they call the parser, registry, emitter)
4. The newtypes (SourceText, ParsedAst, etc.) → grammar types

---

### mirror_runtime.rs (src/mirror_runtime.rs)
**Contract:** in: `.mirror` source text, out: `MirrorFragment` (compiled), `Shatter` (artifact), `MirrorRegistry` (boot state). Also: `parse_form()` (the parser), tokenizer, MirrorRuntime struct, CompiledShatter.
**Status:** needs @io boundary (partially can be grammar)
**Current:** The largest file (~1600 lines). Contains:
- **Tokenizer**: `tokenize()` — lexes `.mirror` source into tokens. Pure logic.
- **Parser**: `parse_form()` → `Imperfect<MirrorFragment, Error, MirrorLoss>` — the main parser. Pure logic operating on tokens, building MirrorAST. Line-oriented, brace-balanced.
- **Declaration keyword table**: `is_decl_keyword()` — the hardcoded keyword set. Pure data.
- **Shatter**: Prism implementation for compilation artifacts. Thin.
- **MirrorRegistry**: Wraps `FrgmntStore<MirrorFragment>` — resolves `in @X` references. @io (disk).
- **MirrorRuntime**: Holds registry, compiles sources, boots grammars. @io (disk).
- **Boot sequence**: `compile_boot_dir()` — reads boot/ directory in order. @io (filesystem).

**Target:** The big prize. Most of this file IS grammar logic trapped in Rust:
- Tokenizer → grammar-parameterized tokenization (evaluate.rs already exists for this)
- Parser → the parser IS the grammar evaluator. The keyword table IS the boot grammars.
- Shatter → already a grammar (`@shatter`)
- Registry → @io boundary (thin Rust over FrgmntStore)
- Runtime → @io boundary (thin Rust orchestration)

**Gap:**
- The tokenizer is ~200 lines of pure Rust string processing. This can be a grammar action.
- The parser is ~500 lines of recursive descent. The grammar IS the parser — but the bootstrap requires having a parser before grammars exist. This is the chicken-and-egg.
- MirrorRegistry.resolve/register are @io (disk operations via FrgmntStore).
- Boot sequence is @io (fs::read_dir, fs::read_to_string).

**Migration:**
1. **Keyword table** → boot/00-prism.mirror + boot/01-meta.mirror (already done)
2. **Tokenizer** → `@mirror/tokenize` grammar action. Pure logic, no @io.
3. **Parser** → The parser stays in Rust for bootstrap. BUT: once boot grammars are loaded, the evaluate.rs grammar-parameterized evaluation replaces the hardcoded parser for user files. The parser bootstraps itself out of existence.
4. **Shatter** → `@shatter` grammar (already done)
5. **MirrorRegistry** → `@io` boundary: 4 operations (open, resolve_ref, register, list_refs)
6. **MirrorRuntime** → `@io` boundary: compile_source, boot, the orchestrator
7. **parse_form** → stays as Rust entry point, but delegates to grammar evaluation once booted

---

### cli.rs (src/cli.rs)
**Contract:** in: command name + args, out: `Imperfect<String, CliError, MirrorLoss>`. Dispatch table for all CLI commands.
**Status:** can be grammar NOW (partially)
**Current:** ~1000 lines. Cli struct wraps MirrorRuntime + SpecConfig. dispatch() routes by command name. ~20 command handlers (compile, crystal, ai, ci, lsp, ca, merge, bench, verify, init, repl, kintsugi, check, focus/project/split/zoom/refract, craft, registry, git, query). Help text generation from spec blocks.
**Target:** Already partially grammar — `boot.alex/cli.mirror` declares `@mirror/cli` with command types. `mirror.spec` IS the dispatch table as grammar. The Cli struct's dispatch_handler() is a match on command names → this IS an evaluate() over `@mirror/cli`.
**Gap:**
- Each cmd_* handler contains @io operations (fs reads, git operations, compilation orchestration).
- The dispatch table itself is pure logic already captured in mirror.spec.
- Help text generation is pure string formatting.
**Migration:**
1. **dispatch table** → `@mirror/cli` grammar + mirror.spec (already done)
2. **help text** → `@nl` grammar action (doc generation)
3. **cmd_compile** → grammar action calling @io(read_file) + @parse + @emit
4. **cmd_crystal** → grammar action calling @io(read_dir) + boot sequence
5. **cmd_kintsugi** → grammar action calling @code/rust kintsugi operations
6. **cmd_focus/project/split/zoom/refract** → grammar actions (already the five operations)
7. **cmd_git** → @io boundary (libgit2)
8. **cmd_ai** → @io boundary (model invocation)
9. **cmd_lsp** → @io boundary (stdio server)

---

### code_rust.rs (src/code_rust.rs)
**Contract:** in: Rust source text, out: `Vec<RustItem>` (parsed Rust items), `MirrorAST` (converted), `Ast` (base AST for kintsugi)
**Status:** can be grammar NOW
**Current:** ~1500 lines. Three layers:
1. **Lightweight Rust parser**: `parse_rust_items()` — scans for keywords (fn, struct, enum, impl, use, trait, mod), matches braces, extracts names/params/fields. Pure string processing.
2. **MirrorAST conversion**: `item_to_mirror_ast()` — maps RustItem → MirrorAST nodes (fn→Zoom, struct→Split, impl→Focus, use→Project, trait→Refract, mod→Module).
3. **Base AST conversion**: `rust_to_base_ast()` — maps to base AST for kintsugi operations (eliminate_dead, collapse_aliases, flatten_wrappers).
4. **Metrics**: node_count, depth, fn/type/impl/use/trait counts.

**Target:** This IS `@code/rust` — the grammar already exists in `boot/std/code/rust.mirror`:
```
grammar @code/rust {
  zoom fn
  split struct
  split enum
  focus impl
  focus mod
  project use
  refract trait
}
```
The Rust parser and conversions are the implementation of that grammar. The keyword→operation mappings are already grammar declarations. evaluate.rs already has the grammar-parameterized evaluation that reads these mappings.
**Gap:** The lightweight Rust parser (brace matching, keyword scanning) is ~800 lines of pure string processing. This is the @io boundary for reading Rust source — it needs to exist somewhere, but it's pure logic that could be a grammar action.
**Migration:**
1. **Keyword→operation mappings** → already grammar (`@code/rust`)
2. **Rust parser** → `@code/rust` action `parse(source) -> [rust_item]` — pure logic, stays as Rust @io initially, migrates to grammar as evaluate.rs matures
3. **MirrorAST conversion** → this IS evaluate.rs applied to `@code/rust`
4. **Kintsugi operations** → `@kintsugi` grammar (already declared)
5. **Metrics** → `@trace/complexity` grammar (already exists)

---

### evaluate.rs (src/evaluate.rs)
**Contract:** in: compiled grammar fragment + source text, out: `MirrorAST` (evaluated per grammar rules)
**Status:** already grammar
**Current:** Grammar-parameterized evaluation. Extracts keyword→operation mappings from a compiled grammar fragment, then tokenizes source text and builds MirrorAST nodes according to the grammar's rules. This IS the grammar evaluator — the thing that makes `@code/rust { zoom fn }` actually work.
**Target:** This is the key mechanism. When evaluate.rs is complete, every `@code/X` grammar self-describes its parser. No more hardcoded Rust parsing — the grammar IS the parser.
**Gap:** Currently only handles simple keyword→operation mappings. Needs to handle:
- Brace matching (structural)
- Parameter extraction
- Type parsing
- Nested scopes
**Migration:** evaluate.rs IS the migration. As it grows, more of mirror_runtime.rs's parser becomes redundant.

---

### parse.rs (src/parse.rs)
**Contract:** in: `.mirror` source text, out: `Prism<AstNode>` (content-addressed parse tree)
**Status:** needs @io boundary
**Current:** Implements `Vector<String, Prism<AstNode>>`. Maps MirrorFragment tree to Prism tree of AstNode (Kind + name + value). Used by spectral and external consumers. The bridge between internal AST and external content-addressed interface.
**Target:** This is the @io boundary between mirror's internal types and spectral's external consumption. Stays as thin Rust.
**Gap:** Minimal — it's already thin glue.
**Migration:** Keep as @io boundary. The tag_to_ast() mapping table could move to grammar.

---

### emit_code.rs (src/emit_code.rs)
**Contract:** in: `MirrorFragment`, out: `IoList` (tree of byte slices for target language code)
**Status:** can be grammar NOW
**Current:** IoList type (Chunk/Nested/Empty — tree of byte slices, like Erlang iolist). TemplateSet struct with closures for each code generation template (map_type, type_name, field_name, emit_enum, emit_struct, emit_unit_type, emit_function, emit_property, emit_module, emit_header, emit_comment). CodeGrammar wraps TemplateSet with a grammar name. `emit_code_fragment()` walks the MirrorFragment tree and applies templates.
**Target:** This IS `@code`'s `render` template — already declared in `boot/04-code.mirror`:
```
abstract template render(g: @code, ast(g) -> io_list)
```
Each target language provides its own TemplateSet. The Rust templates produce Rust source.
**Gap:** The TemplateSet closures are pure Rust logic (string formatting, case conversion). These are exactly the kind of thing that should be grammar templates.
**Migration:**
1. **IoList** → stays as Rust type (it's the @io output format)
2. **TemplateSet** → grammar templates in `@code/rust`, `@code/gleam`, etc.
3. **emit_code_fragment** → grammar action `render` dispatching to target templates
4. **Case conversion** → grammar actions (type_name, field_name, etc.)

---

### emit_rust.rs (src/emit_rust.rs)
**Contract:** in: `MirrorFragment`, out: Rust source code (String)
**Status:** can be grammar NOW
**Current:** The Rust-specific TemplateSet implementation. PascalCase/snake_case conversion. Type mapping (mirror types → Rust types). Enum/struct/function emission. Derive macros, visibility, documentation comments.
**Target:** This IS the `@code/rust render` implementation. Grammar templates.
**Gap:** Pure string formatting logic. No @io.
**Migration:**
1. Move all template logic to `@code/rust` grammar actions
2. Keep IoList as Rust @io type
3. The grammar actions produce IoList, Rust renders it to bytes

---

### spec.rs (src/spec.rs)
**Contract:** in: `mirror.spec` file content, out: `SpecConfig` (command registry)
**Status:** can be grammar NOW
**Current:** Parses `mirror.spec` files into SpecConfig with SpecBlock entries. Each block is a CLI command with flags and settings. discover() walks up from cwd looking for mirror.spec. resolve_command() finds blocks by name. help_text() renders help.
**Target:** mirror.spec IS already grammar — it uses mirror syntax. The parser for .spec files should be parse_form() applied to the spec grammar.
**Gap:** The spec parser is a separate ad-hoc parser (~200 lines) instead of using mirror's own parser. This is a bootstrap artifact.
**Migration:**
1. Parse .spec files with parse_form() + a `@spec` grammar
2. SpecConfig becomes a grammar-derived type
3. discover() stays as @io (filesystem walking)

---

### ast.rs (src/ast.rs)
**Contract:** in: nothing (type defs), out: `Ast` enum (Atom, Ref, Body, Call, Prism + five optics)
**Status:** stays in Rust
**Current:** The base AST — simpler than MirrorAST. Five syntax variants (Atom, Ref, Body, Call, Prism) plus five optic variants. Used by kintsugi operations (eliminate_dead, collapse_aliases, flatten_wrappers) and the base evaluation pipeline.
**Target:** Stays as Rust substrate. The base AST is the foundation the kintsugi operations work on.
**Gap:** None — this is substrate.
**Migration:** N/A.

---

### abyss.rs (src/abyss.rs)
**Contract:** in: Prism + initial beam, out: `Termination` (Settled/BudgetExhausted/Oscillation)
**Status:** can be grammar NOW
**Current:** The core loop. AbyssConfig (max_cycles, precision, oscillation_window). PrismLoop trait (fold from projection back to focused form). The convergence detection via spectral hash comparison.
**Target:** The Abyss IS Fate. The loop logic is pure — no @io. This is exactly what `@kintsugi { collapse(ast, ast) -> imperfect { \ } }` declares.
**Gap:** The PrismLoop trait requires Prism trait implementations. The loop logic itself is pure.
**Migration:**
1. AbyssConfig → grammar type in `@kintsugi`
2. Convergence detection → grammar action
3. PrismLoop → trait stays in Rust, loop body becomes grammar

---

### dirac.rs (src/dirac.rs)
**Contract:** in: weighted graph (adjacency), out: eigenvalues, Connes distance, spectral gap
**Status:** stays in Rust
**Current:** Sparse matrix in CSR format. Signed weighted incidence matrix. Block Dirac operator [[0, B^T], [B, 0]]. Jacobi eigenvalue solver. Connes distance via Dijkstra. All pure math, no external deps.
**Gap:** This is pure computation. Could theoretically be grammar, but the numerical linear algebra is the kind of thing that benefits from Rust's performance. This is the spectral triple math.
**Migration:** Stays in Rust. The math IS the substrate. Future: @code/fortran for GPU acceleration.

---

### session.rs (src/session.rs)
**Contract:** in: reader identity + gestalt path, out: Session (state machine + gestalt + forks)
**Status:** can be grammar NOW
**Current:** Session state machine (Idle → Focused → Projected → Forked → Merged → Trained). Fork struct for exploration branches. GestaltProfile loading from disk.
**Target:** Session states ARE the five operations applied to encounters:
```
Idle → focus(question) → Focused
Focused → project → Projected
Projected → split(fork) → Forked
Forked → zoom(merge) → Merged
Merged → refract(train) → Trained
```
**Gap:** GestaltProfile.load is @io (disk). State machine logic is pure.
**Migration:**
1. Session state machine → grammar (state transitions as optic composition)
2. GestaltProfile loading → @io boundary
3. Fork management → grammar actions

---

### store.rs (src/store.rs)
**Contract:** in: value, out: `Shard<V>` (value + content address), `MirrorOid`, `ForeignKey`
**Status:** stays in Rust
**Current:** MirrorOid newtype over Oid. Shard pairs value with content address. ForeignKey bridges hash domains (coincidence → git SHA-1). MirrorStore trait with put/get returning Imperfect.
**Target:** Stays as @io boundary. Store operations ARE io operations.
**Gap:** None.
**Migration:** N/A — @io substrate.

---

### lsp/ (src/lsp/*.rs)
**Contract:** in: LSP protocol messages, out: LSP protocol responses
**Status:** needs @io boundary
**Current:** LSP server implementation. generate.rs (tree-sitter grammar generation), language.rs (language config), node_types.rs (node type mappings), server.rs (stdio JSON-RPC server).
**Target:** The LSP actions are already declared in `@code`:
```
abstract action complete(position) -> [completion]
abstract action diagnose(range) -> [diagnostic]
abstract action hover(position) -> string
abstract action definition(position) -> position
abstract action references(position) -> [position]
abstract action tokens(range) -> [token]
```
**Gap:** The server loop is @io (stdio, JSON-RPC). The LSP actions themselves are grammar actions already declared.
**Migration:**
1. LSP types → already grammar (`@code` types: position, range, diagnostic, etc.)
2. LSP actions → grammar actions (already declared)
3. Server loop → @io boundary (stdio transport)
4. Tree-sitter grammar generation → grammar action

---

### Various remaining files

| File | Status | Notes |
|------|--------|-------|
| `main.rs` | @io | 44 lines. CLI entry point. Stays. |
| `ast_prism.rs` | can be grammar | AST-as-Prism implementation |
| `bounded.rs` | stays in Rust | Pressure-based bounded storage |
| `build.rs` | @io | Build script |
| `bundle.rs` | can be grammar | Bundle/package management |
| `classifier.rs` | can be grammar | Declaration classification |
| `dispatch.rs` | can be grammar | Value/Args/Response types |
| `domain/` | partially grammar | conversation.rs (Kind enum), filesystem.rs (@io) |
| `eigentest.rs` | stays in Rust | Eigenvalue testing (math) |
| `fate_bridge.rs` | @io boundary | Fate model invocation |
| `filter.rs` | can be grammar | Declaration filtering |
| `generate_crate.rs` | can be grammar | Rust crate generation |
| `generated.rs` | can be grammar | Code generation utilities |
| `gestalt.rs` | can be grammar | Gestalt profile management |
| `git_prism.rs` | @io | Git read operations via libgit2 |
| `git_store.rs` | @io | Git-backed fragment store |
| `grammar_regions.rs` | can be grammar | Grammar region detection |
| `license.rs` | can be grammar | License type system |
| `mirror_bf.rs` | stays in Rust | Brainfuck interpreter (Fate substrate) |
| `mirver.rs` | can be grammar | Version comparison |
| `nl/` | @io + grammar | Natural language processing |
| `optic.rs` | can be grammar | Optic composition helpers |
| `packages.rs` | can be grammar | Package resolution |
| `prism.rs` | stays in Rust | Prism fragment types |
| `resolve.rs` | partially grammar | Type registry, resolution |
| `run.rs` | @io | Process execution |
| `runtime.rs` | stays in Rust | MetalRuntime trait (42 lines) |
| `scaffold.rs` | can be grammar | Project scaffolding |
| `shard.rs` | stays in Rust | Shard types |
| `shatter_blob.rs` | @io | Binary serialization |
| `shatter_format.rs` | can be grammar | .shatter format |
| `shell.rs` | @io | REPL/shell |
| `sign.rs` | @io | Ed25519 signing |
| `test.rs` | can be grammar | Test utilities |

---

### Boot grammars (boot/)

| File | Status | What it defines |
|------|--------|-----------------|
| `00-prism.mirror` | already grammar | The five operations, `in`/`out`, `@prism`, `abstract io tick -> tock` |
| `00a-sigil.mirror` | already grammar | Sigil system |
| `01-meta.mirror` | already grammar | Type system, operators, `@meta`, ast(g), expression(g), declaration(g), pattern(g), beam(result) |
| `02-actor.mirror` | already grammar | actor, state, process, message, `@actor` |
| `02a-io.mirror` | already grammar | mut, effect, io, `@io` (read/write/send/spawn/stop) |
| `02b-runtime.mirror` | already grammar | `@runtime` (pid, supervisor, strategy, spawn/send/receive/supervise) |
| `03-shatter.mirror` | already grammar | `@shatter` (materialize/crystallize/learn) |
| `04-code.mirror` | already grammar | `@code` (translate/render templates, LSP actions, position/range/diagnostic types) |
| `04a-code-rust.mirror` | already grammar | `@code/rust` (primitive types, LSP action overrides) |
| `04b-code-gleam.mirror` | already grammar | `@code/gleam` |
| `05-property.mirror` | already grammar | `@property` (verdict, property_error, property_loss) |
| `06-action.mirror` | already grammar | action prism, action declaration |
| `07-package.mirror` | already grammar | `@package` (version, semver, mirver, change, package) |
| `07a-package-git.mirror` | already grammar | `@git` (repo, commit, branch, tag) |
| `07b-package-spec.mirror` | already grammar | `@spec` (target, environment, deployment) |

### Boot standard library (boot/std/)

| File | Status | What it defines |
|------|--------|-----------------|
| `kintsugi.mirror` | already grammar | `@kintsugi { collapse(ast, ast) -> imperfect { \ } }` |
| `kintsugi/translate.mirror` | already grammar | `@kintsugi/translate { translate(ast, grammar) -> imperfect { \ } }` |
| `kintsugi/migrate.mirror` | already grammar | `@kintsugi/migrate { migrate(ast) -> imperfect { \ } }` |
| `craft.mirror` | already grammar | `@craft { craft(target) -> crystal { focus |> split |> zoom |> refract |> project } }` |
| `nl.mirror` | already grammar | `@nl { type nl(text), type #(nl), doc, commit_message }` |
| `code/rust.mirror` | already grammar | `@code/rust { zoom fn, split struct/enum, focus impl/mod, project use, refract trait }` |
| `code/llvm.mirror` | already grammar | `@code/llvm { zoom define, split type, focus module, project declare, refract verify }` |
| `git/hooks.mirror` | already grammar | `@git/hooks { hook types, check, format }` |
| Various std/*.mirror | already grammar | bool, list, map, number, option, order, result, run, set, text, time, etc. |

---

## 3. The Categories

### Already grammar (ready)
- All boot/ and boot/std/ files
- mirror.spec
- boot.alex/cli.mirror
- evaluate.rs (the grammar evaluator itself)

### Can be grammar NOW (pure logic, no @io)
- `declaration.rs` — OpticOp mapping table
- `loss.rs` — loss types and holonomy computation
- `lambda_phases.rs` — phase declarations and pipeline composition
- `emit_code.rs` — template set and code emission
- `emit_rust.rs` — Rust-specific templates
- `spec.rs` — spec parsing (should use mirror's own parser)
- `code_rust.rs` — keyword mappings (already grammar), conversion logic
- `abyss.rs` — convergence detection loop
- `session.rs` — state machine transitions
- `classifier.rs`, `filter.rs`, `optic.rs`, `grammar_regions.rs`
- `license.rs`, `mirver.rs`, `shatter_format.rs`, `scaffold.rs`
- `bundle.rs`, `packages.rs`, `dispatch.rs`, `gestalt.rs`

### Needs @io boundary (filesystem, network, process, FFI)
- `mirror_runtime.rs` — MirrorRegistry (FrgmntStore), boot sequence (filesystem)
- `cli.rs` — command handlers (filesystem, git, process)
- `parse.rs` — Vector implementation (bridges to external consumers)
- `lsp/` — stdio JSON-RPC server
- `git_prism.rs`, `git_store.rs` — libgit2
- `sign.rs` — Ed25519 (age crate)
- `shatter_blob.rs` — binary serialization (bincode)
- `shell.rs` — REPL (stdin/stdout)
- `run.rs` — process execution
- `nl/` — unicode segmentation, stemming
- `fate_bridge.rs` — model invocation
- `main.rs` — CLI entry point

### Stays in Rust (the substrate)
- `mirror_ast.rs` — the 7-variant AST definition (the glass wall)
- `ast.rs` — the base AST (kintsugi substrate)
- `kernel.rs` — Oid, Trace, Vector, ContentAddressed, Composed, Latent
- `store.rs` — MirrorOid, Shard, ForeignKey
- `dirac.rs` — spectral triple math
- `eigentest.rs` — eigenvalue testing
- `runtime.rs` — MetalRuntime trait (42 lines)
- `prism.rs` — Prism fragment types
- `shard.rs` — Shard types
- `bounded.rs` — pressure-based bounded storage
- `mirror_bf.rs` — brainfuck interpreter (Fate substrate)

---

## 4. The Critical Path

### Phase 1: Loss types and emit templates (unlocks self-description)

**What:** Translate loss.rs types and emit_rust.rs templates to grammar.

**Why first:** These are pure logic with zero @io dependencies. They prove that grammar can express the compiler's own type system and code generation. The compiler starts describing itself.

1. `@mirror/loss` grammar — all loss types, holonomy computation
2. `@code/rust` render templates — PascalCase, snake_case, emit_enum, emit_struct
3. Keep Rust impls as thin delegation to grammar-defined logic

### Phase 2: evaluate.rs completion (unlocks grammar-driven parsing)

**What:** Complete the grammar-parameterized evaluator to handle:
- Brace matching (structural scope)
- Parameter extraction (typed fields)
- Nested scopes (grammar children)
- Type body parsing (enum/struct/alias/unit)

**Why second:** This is the mechanism. Once evaluate.rs handles full mirror syntax, every `@code/X` grammar becomes a self-contained parser. The hardcoded parser in mirror_runtime.rs becomes redundant for user-facing compilation. evaluate.rs bootstraps the parser out of existence.

### Phase 3: mirror_runtime.rs decomposition (the big one)

**What:** Split mirror_runtime.rs into:
- `@mirror/tokenize` — tokenizer as grammar action
- `@mirror/parse` — parser as grammar action (delegates to evaluate.rs)
- `@mirror/registry` — thin @io boundary (4 operations over FrgmntStore)
- `@mirror/boot` — boot sequence as grammar action + @io(read_dir)

**Why third:** This is the heart of the compiler. Once decomposed, the compiler's core logic is grammar, and only the @io boundaries remain in Rust.

### Phase 4: cli.rs grammar dispatch (unlocks full self-hosting)

**What:** Route all CLI commands through grammar evaluation:
- `mirror.spec` parsed by mirror's own parser
- Each command handler becomes a grammar action
- @io operations isolated into `@io` boundary functions

**Why fourth:** With phases 1-3 done, the CLI is just dispatch. The grammar IS the CLI. The Rust code becomes ~200 lines of @io glue.

### Phase 5: code_rust.rs via evaluate.rs (the proof)

**What:** Replace code_rust.rs's lightweight Rust parser with evaluate.rs applied to `@code/rust` grammar.

**Why last:** This proves the system works end-to-end. A grammar (`@code/rust`) that declares keyword→operation mappings, combined with evaluate.rs, replaces 1500 lines of handwritten Rust parser. The grammar IS the parser.

---

## 5. The Residual Rust

After all phases, what remains in Rust:

```
src/
├── main.rs              # 44 lines — CLI entry point, @io
├── mirror_ast.rs        # ~830 lines — the AST definition (the glass wall)
├── ast.rs               # ~300 lines — base AST (kintsugi substrate)
├── kernel.rs            # ~430 lines — Oid, Vector, Composed, Latent
├── dirac.rs             # ~400 lines — spectral triple math
├── eigentest.rs         # ~200 lines — eigenvalue testing
├── mirror_bf.rs         # ~100 lines — brainfuck (Fate substrate)
├── runtime.rs           # 42 lines — MetalRuntime trait
├── store.rs             # ~150 lines — MirrorOid, Shard, ForeignKey
├── prism.rs             # ~100 lines — Prism fragment types
├── shard.rs             # ~50 lines — Shard types
├── bounded.rs           # ~200 lines — bounded storage
├── @io boundaries:
│   ├── mirror_registry.rs  # ~100 lines — FrgmntStore wrapper
│   ├── boot.rs             # ~50 lines — read boot dir
│   ├── lsp_server.rs       # ~200 lines — stdio JSON-RPC
│   ├── git_io.rs           # ~200 lines — libgit2 operations
│   ├── sign_io.rs          # ~50 lines — Ed25519
│   ├── shell_io.rs         # ~50 lines — REPL stdin/stdout
│   └── run_io.rs           # ~30 lines — process execution
```

Total residual Rust: ~3300 lines (down from ~15000+).
Everything else: grammar in `boot.mirror/`.

---

## 6. The equation

```
boot.mirror/ = kintsugi(src/ + boot/, @mirror)
```

Where:
- `src/` is the Rust implementation
- `boot/` is the existing grammar declarations
- `@mirror` is the target grammar (mirror describing itself)
- `kintsugi` finds the ground state fiber in H (Hilbert space of all implementations)
- The ground state minimizes D (the Dirac operator = information loss)

The output `boot.mirror/` is the mirror compiler expressed purely in mirror,
with @io boundaries as the thinnest possible Rust substrate.

The proof is: `mirror craft boot.mirror/ --target @mirror` produces the same
binary behavior as `cargo build`. The content addresses match. The crystal
is the same crystal. Different fiber, same eigenvalue.

`e^{n+1} < e^n`. The errors get smaller. The growth is monotonically
non-decreasing. By convexity.
