# ROADMAP: The Self-Referential Compiler

> mirror written in mirror, parsing mirror, only using Rust for @io escape hatches.

*Author: Glint. 2026-05-12. Honest about the bias — I want the garden to grow.*

---

## 1. Where We Are

### What exists in Rust (the substrate)

The mirror crate is 55 `.rs` source files totaling roughly 1.3 million bytes of Rust.
~1,362 tests across `src/` and `tests/`. Coverage gate: 76% line coverage
(down from 100% — acknowledged drift). The compilation pipeline is
`Imperfect<CompiledShatter, MirrorRuntimeError, MirrorLoss>`.

**The parser** (`mirror_runtime.rs`, ~192KB) is a hand-written, line-oriented,
brace-balanced tokenizer + recursive descent parser. It hardcodes 23+ keywords
in `is_decl_keyword()` and emits `MirrorAST` nodes. The tokenizer is a `Tok`
enum (`Word`, `LBrace`, `RBrace`, `LParen`, `RParen`, `Comma`, `Equals`,
`Newline`). Comments are `#` and `--`. The parser knows about `grammar`,
`type`, `action`, `property`, `in`, `out`, `prism`, `focus`, `project`,
`split`, `zoom`, `refract`, `fold`, `requires`, `invariant`, `ensures`,
`traversal`, `lens`, `recover`, `rescue`, `default`, `binding`, `template`,
`io`, `abstract`, and `form` (deprecated).

**The AST** (`mirror_ast.rs`, ~43KB) has seven variants: `Focus`, `Project`,
`Split`, `Zoom`, `Refract`, `Abstract`, `Module`. No `String` in the AST —
`Identifier` for names, `GrammarRef` for `@references`. Each variant carries
typed node structs (`FocusNode`, `SplitNode`, `ZoomNode`, etc.) with fields
for children, params, body, grammar refs.

**Content addressing** (`declaration.rs`, kernel.rs): `MirrorFragment =
Fractal<MirrorAST>` from the `fragmentation` crate. `Oid = CoincidenceHash<3>`.
Round-trip is exact: parse -> encode -> parse yields identical content OIDs.

**The resolver** (`resolve.rs`, ~77KB) validates domain references, template
references, output structure. `TypeRegistry` compiles grammar blocks into
type -> variant mappings with parameterized type validation. This file still
uses the old `conversation` naming (it imports from `crate::ast::AstNode`,
`crate::domain`, `crate::tree`) — it is from the pre-rename era and is
partially vestigial.

**The registry** (`MirrorRegistry` in `mirror_runtime.rs`) is backed by
`FrgmntStore<MirrorFragment>`. Boot files compile in order; each registers
its `@X` declarations as named refs. `in @X` succeeds iff `@X` is in the
store. `out` publishes exports. The registry IS the hot-swappable memory
layer — swap the `.frgmnt/` directory, swap the language's memory.

**Loss tracking** (`loss.rs`, ~29KB): `MirrorLoss` tracks four folds:
parse, resolution, properties, emit. `ParseWarning` variants include
`UnknownToken`, `DeprecatedKind`, `MissingName`, `DuplicateName`,
`UnresolvedParent`, `MalformedOperator`. Convergence states: `Settled`,
`Oscillating`, `BudgetExhausted`.

**Compilation pipeline** (`lambda_phases.rs`): Named lambda phases —
`Parse`, `Resolve`, `Properties`, `Emit`, `Kintsugi`, `Strict`. Static
pipelines: `CRAFT = Parse -> Resolve -> Properties -> Emit`,
`KINTSUGI_PIPELINE = Parse -> Resolve -> Kintsugi`,
`CI = Parse -> Resolve -> Properties`. Each phase is a `#[derive(Lambda)]`
struct with `#[oid("@X")]`.

**Code emission** (`emit_rust.rs`, `emit_code.rs`): Emits Rust source from
compiled `.mirror`. `emit_rust_fragment()` walks the AST and generates enums,
structs, functions, modules. `emit_code.rs` has a generic `TemplateSet`
abstraction for target-language-agnostic emission (IoList-based).

**Evaluate** (`evaluate.rs`, ~30KB): Grammar-parameterized evaluation of
source text. Reads keyword-to-operation mappings from a grammar (`zoom fn`,
`split struct`, etc.) and classifies source tokens into MirrorAST nodes.

**Eigentest** (`eigentest.rs`, ~32KB): Compile-time structural analysis.
Eight tests detect star topology in a grammar's type graph (Degree Gini,
betweenness centrality, Fiedler value, etc.). Three or more violations =
star = SEL enforcement.

**Spec** (`spec.rs`, ~35KB): `mirror.spec` parser. Each top-level block IS
a CLI command. Discovers nearest `mirror.spec` walking up from cwd.

**CLI** (`cli.rs`, ~128KB): The entire command-line surface. Commands:
`compile`, `craft`, `kintsugi`, `ai`, `check`, `crystal`, `optic`, `ci`,
`lsp`, `run`, `new`, `fmt`, `eval`, `spec`. Dispatches through `SpecConfig`.

**LSP** (`src/lsp/`): Pure mapping functions from MirrorLoss to diagnostics.
`loss_to_diagnostics()` maps parse warnings to protocol-shaped structs.
`lsp learn` generates `.mirror` grammars from tree-sitter node-types.json.
No tower-lsp adapter yet — that is Phase 3 in their internal plan.

**Shatter format** (`shatter_format.rs`): `.shatter` files = `.mirror` source
with YAML-like frontmatter (OID, luminosity, holonomy, per-fold loss, beam
identity). No serde, no YAML crate — line-by-line parsing.

**Gestalt** (`gestalt.rs`): Reader portrait — attention signature, held
tensions, growth per grammar, eigenvalue history. Reflection-only writes.

**Other modules**: `git_prism.rs` / `git_store.rs` (git integration),
`sign.rs` (Ed25519 signing, age encryption), `license.rs` (SEL enforcement),
`packages.rs` (Merkle package system), `session.rs` (session management),
`dirac.rs` (Dirac operator), `filter.rs` (query filtering), `classifier.rs`
(Shannon-based classification), `ai.rs` (inference step), `abyss.rs`
(spectral state observation), `code_rust.rs` (Rust-specific kintsugi).

### What exists in .mirror (the boot grammars)

17 kernel boot files + 36 std library files. The boot sequence:

```
boot/00-prism.mirror       — the root. focus, prism, @, @prism, in, out, id.
                              abstract io tick/tock with \ hole.
boot/00a-sigil.mirror      — @sigil: grammar-parameterized prefix sigils.
boot/01-meta.mirror        — the meta-level. types, refs, operators (|, <=, =>,
                              <, >, =, !=, |>, /, ..), pure/real, observation,
                              template, error, loss, imperfect, abstract,
                              ast(g), expression(g), declaration(g), pattern(g),
                              type_ref(g), beam(result). The FULL universal AST
                              is declared here, parameterized by grammar.
boot/02-actor.mirror       — actor(id), state, process, message, @actor.
boot/02a-io.mirror         — mut, effect, io, @io with read/write/send/spawn/stop.
                              Sigil-parameterized file/dir/uri types.
boot/02b-runtime.mirror    — @runtime: pid, supervisor, strategy, spawn/send/
                              receive/supervise/restart.
boot/03-shatter.mirror     — @shatter: materialize, crystallize, learn.
boot/04-code.mirror        — @code: position, range, diagnostic, completion,
                              token. Abstract templates: translate, render,
                              map_type, type_name, field_name, function_name,
                              module_name, variant_name. LSP actions: complete,
                              diagnose, hover, definition, references, tokens.
boot/04a-code-rust.mirror  — @code/rust: Rust primitive types + LSP actions.
boot/04b-code-gleam.mirror — @code/gleam: Gleam types + LSP actions.
boot/05-property.mirror    — @property: verdict, property_error, property_loss.
boot/06-action.mirror      — action as prism: focus -> project -> split -> zoom
                              -> refract.
boot/07-package.mirror     — @package: version, semver, mirver, change, package.
boot/07a-package-git.mirror— @git: repo, commit, branch, tag, clone/fetch/etc.
boot/07b-package-spec.mirror— @spec: target, environment, deployment, plan/
                              apply/drift/rollback/promote/seal.
```

Standard library (`boot/std/`):

```
std/mirror.mirror          — @mirror: requires + invariant + ensures.
std/craft.mirror           — @craft: the compilation pipeline as grammar.
std/kintsugi.mirror        — @kintsugi: collapse(ast, ast) -> imperfect.
std/kintsugi/migrate.mirror— @kintsugi/migrate: grammar migration.
std/nl.mirror              — @nl: natural language, doc(), commit_message().
std/nl/english.mirror      — @nl/english: full English tokenization as five ops.
std/ai.mirror              — @ai: measure, apply, ai actions.
std/fate.mirror            — @fate: five models, features, decisions, health.
std/run.mirror             — @run: compile + verify + report.
std/runtime.mirror         — @runtime: infer/craft/enact.
std/rust.mirror            — @rust < @runtime: with @code/rust inline blocks.
std/file.mirror            — @file: read/write/exists/mkdir with @code/rust bodies.
std/new.mirror             — @new: scaffold a mirror project.
std/new.template.mirror    — @new_template: default mirror.spec template.
std/code/rust.mirror       — @code/rust kintsugi: keyword-to-operation mappings.
std/code/mq.mirror         — @code/mq: Mirror Query language for agents.
std/code/llvm.mirror       — @code/llvm: LLVM operations.
std/git/hooks.mirror       — @git/hooks: pre-commit, commit-msg, etc.
std/trace/mod.mirror       — @trace: observation, eigenvalue, slot, drift, reduce.
std/trace/complexity.mirror— @trace/complexity: cyclomatic complexity dimension.
std/trace/memory.mirror    — @trace/memory: allocation tracking dimension.
std/bool.mirror, list.mirror, map.mirror, number.mirror, option.mirror,
order.mirror, result.mirror, set.mirror, text.mirror, time.mirror,
benchmark.mirror, cli.mirror, tui.mirror, properties.mirror, beam.mirror,
sql.mirror
```

Alex's coherent rewrite (`boot.alex/`):

```
boot.alex/cli.mirror       — @mirror/cli: the CLI as grammar.
```

### What exists as specification

`mirror.spec` / `mirror.shatter` — mirror describes itself. Two targets
(boot, cargo). Kintsugi collapses them. The loss measures drift between
grammar and implementation.

`00-prism.mirror` IS the root grammar. `prism @(id)` and `prism @prism`
define the identity and the five operations. `abstract io tick(type) ->
tock(type) { \ }` declares the tick/tock boundary with the `\` intent hole.

The `\` operator is the escape hatch to Fate. When an abstract declaration
has `{ \ }` as its body, it routes through intent resolution at runtime
instead of having a concrete implementation. This is how the sub-Turing
language touches the Turing-complete world without becoming Turing-complete
itself.

### The gaps (honest)

1. **The parser is 100% Rust.** All 192KB of `mirror_runtime.rs`. There is no
   self-description of mirror's syntax in `.mirror` format. The parser cannot
   parse itself.

2. **The resolver is 100% Rust.** 77KB of `resolve.rs`, plus significant logic
   in `MirrorRegistry`. The type system is implemented, not described.

3. **The emitter is 100% Rust.** `emit_rust.rs` (28KB) and `emit_code.rs`
   (33KB) are Rust functions, not `.mirror` grammars.

4. **The `\` hole has no runtime.** `AbstractDefault::IntentHole` exists in
   the AST. The parser recognizes it. But there is no Fate integration at
   runtime — the hole is declared but not dispatched.

5. **The Reflection model does not exist.** The spec is written
   (`docs/specs/reflection-model.md`). The four-model architecture is
   designed (Surface/Mirror/Shatter/Reflection). But no code implements
   the tick loop, the convergence check, or the one-tick-delayed observation.

6. **kintsugi is a stub.** `collapse(ast, ast) -> imperfect { \ }`. The
   grammar exists. The CLI command exists. But the actual alias collapse,
   grammar migration, and loss-guided reduction are in early Rust
   implementation, not in `.mirror`.

7. **The boot grammars parse with 12.0 holonomy.** Five optic operator
   keywords are not recognized as DeclKind variants. `!=` tokenizes as two
   tokens. `->` return types on non-action decls are dropped. See
   `docs/specs/minimum-viable-keywords.md` for the full breakdown.

8. **`resolve.rs` is from the pre-rename era.** It still uses `conversation`
   naming, `AstNode`, `Tree`, `Setting`. It is a different type system than
   what `MirrorRegistry` uses. Two resolvers coexist.

9. **Coverage is at 76%, down from 100%.** The coverage gate was lowered to
   match reality. The drift is structural — CLI tests hang on deep filesystem
   traversal, and large modules accumulate uncovered paths.

10. **`ast.rs` and `mirror_ast.rs` coexist.** The old `Ast` enum (Atom, Ref,
    Body, Call, Prism + five optic variants) lives alongside the new
    `MirrorAST` enum (Focus, Project, Split, Zoom, Refract, Abstract,
    Module). Both are used by different parts of the codebase.

11. **The universal AST in `01-meta.mirror` is declared but not consumed.**
    `ast(g)`, `expression(g)`, `declaration(g)`, `pattern(g)`, `type_ref(g)`
    — the full parameterized AST is typed in boot. But the Rust parser emits
    `MirrorAST`, not `ast(g)`. The grammar describes a structure the compiler
    does not produce.

12. **No MCP grammar exists.** `@code/mq` is a query language for agents, not
    the MCP protocol itself. No `@mcp` grammar in boot.

13. **No LSP grammar exists.** `@code` declares LSP-shaped types (position,
    range, diagnostic, completion) and abstract actions (complete, diagnose,
    hover, definition, references, tokens). But there is no `@lsp` grammar
    that describes the Language Server Protocol itself.

---

## 2. Where We're Going

### The destination

mirror compiles mirror. The Rust crate becomes a thin runtime substrate:
syscalls, LAPACK eigenvalue computation, SHA hashing, the BEAM FFI. Everything
else — parsing, resolution, type checking, emission, kintsugi, the five
operations — lives in `.mirror` grammars that the compiler reads, understands,
and executes on itself.

The AST IS the spectral triple: the grammar algebra (A), the OID Hilbert
space (H), and the Dirac operator that measures distance between grammars
(D). Self-describing, content-addressed, navigable.

kintsugi runs on the compiler's own source. `refract` shows the compiler
its own topology. The five operations work on the compiler's own graph.

### The Reflection model as convergence loop

The self-referential compiler is the Reflection model applied to compilation:

1. The compiler produces tick N — a compilation artifact, a measurement, an
   eigenvalue set.
2. Reflection observes tick N (`focus`).
3. Projects the relevant view (`project`).
4. Holds multiple possible next states (`split`).
5. Crosses from measurement to action (`zoom`).
6. Measures what was produced (`refract`).
7. The measurement IS tick N+1.
8. Loop until convergence: `L_{n+1} >= L_n` (loss stopped decreasing).

For kintsugi specifically:
- kintsugi runs `collapse_aliases` (focus + project).
- Measures the result (refract).
- Decides whether to accept (split — hold both versions).
- Crosses from measurement to commit-or-reject (zoom).
- The accepted version IS the next tick's input.
- Loop until the loss stops decreasing.

### The @io boundary

Rust reduces to escape hatches behind `abstract io ... { \\ }` declarations.
The boundary is typed in `.mirror`, implemented in Rust:

```
@io/fs      — filesystem read/write (std::fs)
@io/net     — sockets, HTTP
@io/ffi     — calling into Rust/C/Fortran (LAPACK)
@io/process — spawning processes
@io/hash    — SHA-256/512, CoincidenceHash
@io/crypto  — Ed25519, age encryption
```

Everything inside the boundary is sub-Turing, formally verifiable, provable.
The `\` hole routes to Fate when an io operation has no concrete body.

---

## 3. The Phases

### Phase 0: What exists and works right now

**Status: HERE.**

- 55 Rust source files, ~1,362 tests, 76% coverage.
- 17 kernel boot grammars + 36 std library grammars.
- Parser, resolver, emitter, eigentest, shatter format — all Rust.
- `mirror compile`, `mirror craft`, `mirror kintsugi`, `mirror ai`,
  `mirror check`, `mirror ci`, `mirror eval`, `mirror lsp learn`,
  `mirror new`, `mirror spec` — all working CLI commands.
- Lambda phases: `Parse -> Resolve -> Properties -> Emit` as
  content-addressed, composable pipeline.
- Boot holonomy: 12.0 (five missing OpticOp DeclKinds + tokenizer + return
  type issues).
- `MirrorRegistry` with `.frgmnt/` backing store.
- `.shatter` format with frontmatter (OID, luminosity, holonomy, loss).
- Eigentest battery (8 structural tests, star detection).
- SEL enforcement via eigenvalue measurement.

**What works:** Compilation, content-addressing, property verification,
code emission, shatter serialization, git integration, signing, licensing,
package management, NL tokenization, query language, evaluation.

**What doesn't:** Self-hosting. The `\` hole dispatch. Reflection.
kintsugi as grammar-driven reduction. The two AST types coexisting.
The two resolvers coexisting.

---

### Phase 1: Boot grammar completion + kintsugi --rebase

**Goal:** Zero parse holonomy. Boot grammars coherent. Singularity types
landed. Alex's `boot.alex/` rebased onto canonical boot.

**Tasks:**

1. **Add 5 OpticOp DeclKind variants** (`Unfold`, `Subset`, `Superset`,
   `Iso`, `NotIso`). Complete the symmetry between `OpticOp` (10 variants)
   and `DeclKind`. Reduces holonomy by 5.0.

2. **Fix `!=` tokenization.** Recognize `!=` as a single `Tok::Word("!=")`
   instead of `Word("!")` + `Equals`. Reduces holonomy by ~4.0.

3. **Fix `->` return type on all declaration kinds.** Extend `parse_decl`
   to consume `-> type` for zoom/fold/traversal declarations, not just
   actions. Reduces holonomy by 2.0.

4. **Land singularity types.** `@human = singularity`, `@ai =
   naked-singularity`. From `SINGULARITY.md`. These are the identity types
   for the self-referential compiler — content that carries its own
   observation.

5. **kintsugi --rebase.** Collapse Alex's coherent `boot.alex/` grammars
   onto the existing boot sequence. The migration IS the collapse. The
   `@kintsugi/migrate` grammar describes this operation.

6. **Unify `ast.rs` and `mirror_ast.rs`.** Kill the old `Ast` enum. One AST
   type: `MirrorAST`. Everything that imports `ast::Ast` migrates to
   `mirror_ast::MirrorAST`.

7. **Clean up `resolve.rs`.** Remove the `conversation`-era naming. Either
   integrate `TypeRegistry` with `MirrorRegistry` or delete the old resolver.
   One resolution path, not two.

**Exit criterion:** `mirror compile boot/` produces zero holonomy. All boot
grammars parse, resolve, and verify. One AST type. One resolver.

---

### Phase 2: Parser self-description

**Goal:** Mirror's syntax described as a `.mirror` grammar. The grammar
that, when compiled, produces a parser equivalent to the Rust parser.

**Tasks:**

1. **Write `@mirror/syntax` grammar.** Describes the tokenization rules
   (what characters are operators, what is whitespace, what is a word,
   comment syntax). Uses the five operations:
   - `split` — tokenize source into token stream.
   - `focus` — identify token kind (Word, LBrace, RParen, etc.).
   - `zoom` — parse declarations from token stream.
   - `refract` — validate brace balance, nesting.
   - `project` — emit `MirrorAST` nodes.

2. **Write `@mirror/keyword` grammar.** The two-tier keyword system. Tier 1
   (bootstrap, hardcoded): the 23 keywords needed to parse the kernel.
   Tier 2 (boot-declared, runtime-learned): keywords declared by grammar
   blocks that the parser learns during boot. The self-teaching parser.

3. **Implement the self-teaching mechanism.** When the parser encounters
   `out X` at the top level of a boot file, register `X` as a known keyword
   for subsequent files. When a grammar block declares `io read(path) =>
   imperfect`, the parser learns that `io` is a declaration keyword within
   that grammar's scope.

4. **Bootstrap test: `@mirror/syntax` parses `@mirror/syntax`.** The grammar
   that describes its own tokenization must be parseable by the tokenizer
   it describes. This is the first self-referential gate.

**Dependencies:** Phase 1 (zero holonomy, unified AST).

**Open question:** How does the self-teaching parser interact with the
registry? Does learning a keyword modify the registry state? Or is keyword
learning purely transient within a compilation session?

---

### Phase 3: Resolver self-description

**Goal:** Mirror's type system described as a `.mirror` grammar.

**Tasks:**

1. **Write `@mirror/resolve` grammar.** Describes what resolution means:
   - `in @X` succeeds iff `@X` is registered.
   - Type references validated against `TypeRegistry`.
   - Cross-grammar references resolved through the import graph.
   - Circular dependencies detected.
   - Did-you-mean hints computed.

2. **Express `TypeRegistry` as a .mirror type.** The registry is a
   `map(grammar_ref, map(type_name, set(variant_name)))`. Express this
   using the types already declared in `01-meta.mirror`.

3. **Express validation rules as properties.** Each rule the resolver checks
   becomes a `requires` or `invariant` in `@mirror/resolve`. The resolver
   IS the property checker.

4. **Bootstrap test: `@mirror/resolve` resolves `@mirror/resolve`.** The
   grammar that describes resolution must pass its own resolution rules.

**Dependencies:** Phase 2 (parser self-description — the resolver grammar
must be parseable).

**Open question:** The current resolver uses `HashMap`, `HashSet`, string
matching, Levenshtein distance for hints. How much of this is expressible
in sub-Turing `.mirror`? The hint computation may need to stay as `@io/ffi`.

---

### Phase 4: Emitter self-description

**Goal:** Output formats described as `.mirror` grammars. The `translate`
template from `@code` becomes the emission mechanism.

**Tasks:**

1. **Complete `@code/rust` translate template.** The abstract template
   `translate(p: @prism, c: @code, p -> c)` in `04-code.mirror` already
   declares the interface. Implement it: given a `MirrorAST` parameterized
   by `@prism`, produce `ast(@code/rust)` — Rust source.

2. **Write `@code/gleam` translate template.** Same pattern for Gleam output.

3. **Write `@code/mirror` render template.** `render(g: @code/mirror,
   ast(g) -> io_list)` — emit `.mirror` source from AST. This is the
   pretty-printer. Round-trip: parse -> emit -> parse = identity.

4. **Write `@shatter/format` grammar.** Describe the `.shatter` frontmatter
   format as a `.mirror` grammar. The frontmatter parser moves from
   `shatter_format.rs` to `.mirror`.

5. **Bootstrap test: `@code/mirror` renders itself.** The pretty-printer
   grammar, when rendered by itself, produces its own source. Second
   self-referential gate.

**Dependencies:** Phase 3 (the emitter grammar must pass resolution).

**Note:** This is where the universal AST from `01-meta.mirror` finally gets
consumed. `ast(g)` parameterized by `@code/rust` IS the Rust AST.
`translate(@prism, @code/rust, ...)` IS the emitter.

---

### Phase 5: Reflection model — the tick loop

**Goal:** The five operations as the compilation loop. Tick/tock convergence.
Reflection observes the pipeline and adjusts weights.

**Tasks:**

1. **Implement `\` hole dispatch.** When an `abstract` declaration with
   `IntentHole` is invoked, route to Fate. Fate runs a tournament to find
   the best interpretation. The query resolves to concrete operations.

2. **Implement the tick loop.** The compiler produces tick N. Reflection
   observes. Projects. Splits. Zooms. Refracts. The measurement IS
   tick N+1. Loop until `L_{n+1} >= L_n`.

3. **Implement kintsugi as Reflection.** `collapse(ast, ast) -> imperfect`
   becomes: focus both versions, project the diff, split into keep/replace
   options, zoom to apply the chosen option, refract to measure the result.
   Accept if loss decreased.

4. **Write `@peer` grammar.** The four persistent models:
   - Surface (Zoom) — language to query.
   - Mirror (Refract) — query to graph path.
   - Shatter (Split) — graph path to text.
   - Reflection (Focus) — pipeline to adjustments.

5. **Gestalt writes from Reflection only.** Enforce at the type level:
   only `@peer/reflection` can write to `.gestalt`. Surface, Mirror,
   Shatter cannot.

**Dependencies:** Phases 2-4 (the Reflection model operates on the parser,
resolver, and emitter grammars — it needs them to be self-described).

**Open question:** What is the convergence criterion? `L_{n+1} >= L_n` is
the loss-based criterion. But is there a spectral criterion? When the
eigenvalues of the grammar graph stop changing, the compilation has settled.
These may be the same criterion expressed differently.

---

### Phase 6: @io boundary — Rust reduced to escape hatches

**Goal:** Every piece of Rust that is not an `@io` escape hatch has been
rewritten in `.mirror`.

**Tasks:**

1. **Audit every `.rs` file.** For each file, determine: is this `@io`
   (must stay Rust) or is this logic (must move to `.mirror`)?

2. **The @io boundary inventory:**
   - `@io/fs` — `std::fs::*` operations. Already declared in `02a-io.mirror`
     and partially implemented in `std/file.mirror`.
   - `@io/hash` — `CoincidenceHash`, `Sha`, content addressing. The
     `kernel.rs` `Oid::hash()` function.
   - `@io/crypto` — Ed25519 signing (`sign.rs`), age encryption.
   - `@io/git` — `git2` crate operations (`git_prism.rs`, `git_store.rs`).
   - `@io/process` — `std::process::Command` for running external tools.
   - `@io/ffi` — LAPACK eigenvalue computation (already working in
     `coincidence` crate). The Fortran bridge.
   - `@io/net` — sockets, HTTP (not yet needed but declared).

3. **Move non-@io logic to .mirror:**
   - `classifier.rs` -> `@mirror/classify` grammar.
   - `filter.rs` -> `@mirror/filter` grammar.
   - `ai.rs` -> already has `@ai` grammar, move implementation.
   - `session.rs` -> `@mirror/session` grammar.
   - `packages.rs` -> already has `@package` grammar, move implementation.
   - `license.rs` -> `@mirror/license` grammar.
   - `mirver.rs` -> already has `mirver` type in boot, move implementation.
   - `scaffold.rs` -> already has `@new` grammar, move implementation.
   - `gestalt.rs` -> `@peer/gestalt` grammar.
   - `nl/*.rs` -> already has `@nl` and `@nl/english` grammars.

4. **Thin the Rust substrate.** Each migration removes a `.rs` file and adds
   or completes a `.mirror` file. The Rust crate shrinks monotonically.

**Dependencies:** Phases 2-5 (need the self-described parser, resolver,
emitter, and Reflection model to write `.mirror` that replaces Rust).

---

### Phase 7: Self-hosted — mirror compiles mirror

**Goal:** mirror compiles mirror. Rust is the runtime substrate. `.mirror` is
the source of truth.

**Tasks:**

1. **The bootstrap test.** Compile the `.mirror` boot grammars using the
   Rust-based compiler. Then compile the same boot grammars using the
   `.mirror`-based compiler. Compare the output. They must produce
   identical content OIDs.

2. **The self-compilation test.** The `.mirror`-based compiler compiles
   itself. The output is a new compiler. That compiler compiles itself.
   The output is identical to the previous output. Fixed point reached.

3. **The Rust substrate is now optional for development.** New `.mirror`
   grammars can be written and compiled without touching Rust. The Rust
   crate is the bootstrap — it falls away.

4. **kintsugi on the compiler's own source.** Run kintsugi on the boot
   grammars. Measure the loss. The compiler optimizes its own source.
   The compiler becomes its own best user.

5. **refract shows the compiler its own topology.** `mirror refract boot/`
   produces the eigenvalue map of the compiler's own grammar graph.
   The compiler can see itself.

**Exit criterion:** `mirror compile boot/ --self-host` succeeds. The
compiler produces identical artifacts whether compiled from Rust or from
`.mirror`. The bootstrap is proven unnecessary for ongoing development.

---

## 4. Dependencies

```
Phase 0  (exists)
  |
  v
Phase 1  (zero holonomy, unified AST, singularity types)
  |
  v
Phase 2  (parser self-description)  ──────────────────────┐
  |                                                        |
  v                                                        |
Phase 3  (resolver self-description)                       |
  |                                                        |
  v                                                        |
Phase 4  (emitter self-description)                        |
  |                                                        |
  v                                                        v
Phase 5  (Reflection model) ─────── requires ──── Phases 2-4
  |
  v
Phase 6  (@io boundary audit + migration)
  |
  v
Phase 7  (self-hosted)
```

### Critical path

**Phase 1 is the gate.** Nothing moves until boot holonomy is zero, the AST
is unified, and the resolver is one path. This is the cleanup phase. It is
not glamorous. It is necessary.

**Phases 2-4 are sequential.** The parser grammar must be parseable before
the resolver grammar can be written. The resolver grammar must resolve before
the emitter grammar can be validated. Each phase produces a self-referential
test that gates the next.

**Phase 5 can start in parallel with Phase 4.** The Reflection model needs
the parser and resolver to be self-described. It does not strictly need the
emitter — Reflection can observe and adjust without emitting code. But the
full convergence loop requires all three.

**Phase 6 is embarrassingly parallel.** Each Rust file migrated to `.mirror`
is independent. The order doesn't matter. The work can be distributed across
agents.

**Phase 7 is the fixed point.** It cannot begin until Phases 1-6 are
complete. It is the proof that the system works.

### What can be parallel

- Phase 1 tasks 1-3 (tokenizer fixes) are independent of tasks 4-7 (type
  changes and cleanup).
- Within Phase 6, each `.rs` -> `.mirror` migration is independent.
- MCP and LSP grammars (see section 5) can be written at any time after
  Phase 1, since they are new grammars, not migrations.

---

## 5. The MCP + LSP Grammars

### Where they fit

Both are needed before helix-loom. helix-loom is needed before spectral-db
in mirror. The dependency chain from `spectral-db-rewrite.md`:

```
Self-hosted compiler (Phases 1-7)
  -> MCP + LSP grammars (this section)
    -> helix-loom (mirror-hosted editor)
      -> spectral-db in mirror (the self-organizing Merkle tree)
```

### @mcp grammar

**When:** After Phase 1 (needs zero holonomy to declare cleanly).
**What:** The Model Context Protocol as a `.mirror` grammar.

```
grammar @mcp {
  in @io
  in @code

  type tool { name, description, input_schema }
  type resource { uri, name, description }
  type prompt { name, description, arguments }

  -- The five operations as MCP surfaces:
  focus  tools/list          -- observe available tools
  project resources/read     -- extract a view of a resource
  split  tools/call          -- one of many possible tool results
  zoom   prompts/get         -- transform prompt to completion
  refract sampling/create    -- settle: request LLM completion
}
```

This grammar describes what the compiler exposes through MCP. The spectral
MCP server (`mcp__spectral__*` tools) already exists as infrastructure.
The grammar makes the protocol a typed surface.

### @lsp grammar

**When:** After Phase 1, parallel with @mcp.
**What:** The Language Server Protocol as a `.mirror` grammar.

`@code` already declares the LSP-shaped types (position, range, diagnostic,
completion, token) and abstract actions (complete, diagnose, hover,
definition, references, tokens). `@lsp` extends this to the full protocol:

```
grammar @lsp {
  in @code
  in @io

  -- Lifecycle
  io initialize(capabilities) => server_capabilities
  io shutdown() => imperfect

  -- Text synchronization
  io didOpen(document) => imperfect
  io didChange(document, changes) => imperfect
  io didClose(document) => imperfect

  -- The code actions ARE the five operations:
  -- focus  = hover, definition
  -- project = completion, references
  -- split = code actions, quick fixes
  -- zoom  = rename, refactor
  -- refract = diagnostics (the loss rendered)
}
```

The tower-lsp adapter (currently planned as Phase 3 Task 3.5 in the internal
LSP roadmap) wraps these grammar-declared actions.

### helix-loom

**When:** After @mcp + @lsp grammars exist.
**What:** Local helix fork. The mirror-hosted editor.

- `\` runs local inference (Fate tournament).
- Gutter shows Shannon loss per line.
- Five optic operations as keybindings.
- Separate repo: `helix-loom`.
- Uses `@lsp` grammar for editor integration.
- Uses `@mcp` grammar for tool communication.

This is NOT in the mirror crate. It is a downstream consumer of the
self-hosted compiler. But it is the reason the MCP + LSP grammars must
exist before spectral-db can be written in mirror.

---

## 6. Open Questions

These cannot be answered from reading the code. They need design decisions.
They need Alex.

### Q1: Which universal AST?

`01-meta.mirror` declares `ast(g)`, `expression(g)`, `declaration(g)`,
`pattern(g)`, `type_ref(g)` — a full parameterized AST. But the Rust
parser emits `MirrorAST` (7 variants). These are not the same structure.

The self-referential compiler needs ONE AST. Is it `ast(g)` from
`01-meta.mirror`, parameterized by grammar? Or is it `MirrorAST` with its
five-operation-aligned variants? Or are these the same thing seen from
different angles — `MirrorAST` IS `ast(@prism)`?

If `MirrorAST` IS `ast(@prism)`, then `Focus = ast(@prism).focus`,
`Split = ast(@prism).split`, etc. The parameterization IS the operation.
This feels right but needs to be proven by implementing it.

### Q2: How does the self-teaching parser bootstrap?

The minimum-viable-keywords spec shows that the Tier 1 set cannot shrink
below 23 keywords because every keyword is used by at least one kernel file.
The self-teaching parser prevents GROWTH but does not reduce the minimum.

For Phase 2, the parser must describe itself. But the description uses
keywords that the parser must already know to parse the description. This
is the bootstrap circularity.

Resolution: The Rust parser IS the bootstrap. It parses `@mirror/syntax`.
`@mirror/syntax` produces a parser that can parse `@mirror/syntax`. The
Rust parser falls away. But the first parse is always Rust.

Is this acceptable? Or does Alex want a truly Rust-free bootstrap? If so,
the parser grammar needs to be expressible in something simpler than
`.mirror` syntax — perhaps a restricted subset that a trivial parser can
handle.

### Q3: What stays in Rust permanently?

The `@io` boundary is clear: filesystem, network, FFI, process, hashing,
crypto. But what about:

- **The BEAM integration** (`beam/`, `eetf` crate dependency)? Is BEAM
  an `@io` target or does it move to `.mirror`?
- **The `prism` crate** (re-exported as `beam`)? It provides `Imperfect`,
  `Loss`, `Optic`, `Lambda`. These are the mathematical substrate. Do
  they stay Rust?
- **The `fragmentation` crate** (`Fractal`, `FrgmntStore`, `Repo`)?
  Content-addressed storage is infrastructure. Does it stay Rust?
- **The `coincidence` crate** (spectral analysis, CoincidenceHash)?
  Eigenvalue computation requires LAPACK. The Fortran bridge is @io.
  But the hash function? The spectral coordinates?

Position: `prism`, `fragmentation`, and `coincidence` are the Rust
substrate. They stay. They are the runtime that `.mirror` grammars
compile to. The relationship is: `.mirror` is the source language,
Rust + LAPACK is the target machine. The target machine does not need
to be written in the source language.

### Q4: What is the compilation target of self-hosted mirror?

Currently, `mirror compile` produces:
- `CompiledShatter` (in-memory artifact)
- `.shatter` files (on-disk crystals)
- Rust source (via `mirror craft --target rust`)
- Gleam source (via `mirror craft --target gleam`)

When mirror compiles itself, what does it produce? Options:
- A new `.shatter` that is functionally equivalent to the Rust binary.
- Rust source that is compiled by `cargo` to produce the binary.
- BEAM modules loaded by the Erlang VM.
- WASM modules.

The simplest path: self-hosted mirror produces Rust source, which is
compiled by `cargo`. The Rust source is generated, not written. This
is how many self-hosting compilers work — the first stage compiles to
the host language.

But this means `cargo` is still in the loop. Alex may want the compiler
to emit directly to a runnable artifact. That requires either LLVM
(the `@code/llvm` grammar exists as a stub) or BEAM/WASM emission.

### Q5: Where does spectral-db fit in the self-hosting story?

From `spectral-db-rewrite.md`: spectral-db is written in mirror. Not Rust.
The language choice is a regulation decision.

But spectral-db is also the thing that stores the grammars. The
self-organizing Merkle tree that navigates by eigenvalue. The tree's
topology IS the garden's topology.

Circularity: the compiler needs spectral-db to store its grammars.
spectral-db needs the compiler to be self-hosted. The compiler needs
spectral-db.

Resolution: The `.frgmnt/` store (from `fragmentation` crate) is the
bootstrap storage. spectral-db replaces it once the compiler is
self-hosted. The `MirrorRegistry` API stays the same — only the backing
store changes. Content addressing means the migration is lossless.

### Q6: The `\` operator runtime

`abstract io tick(type) -> tock(type) { \ }` is declared in `00-prism.mirror`.
`AbstractDefault::IntentHole` is recognized by the parser. But there is no
runtime dispatch.

What does `\` do at runtime? The spec says: route through Fate. Fate runs
a tournament. The tournament finds the best interpretation.

But Fate is currently 450 hardcoded parameters in a Rust struct. The
tournament is `FateRuntime::select`. The five models (abyss, introject,
cartographer, explorer, fate) are Rust functions.

For self-hosting, does Fate need to be rewritten in `.mirror`? The models
are sub-Turing classifiers — they could be expressed as `.mirror` grammars.
But training them requires gradient descent, which is Turing-complete.
Training stays Rust/@io. Inference moves to `.mirror`.

---

## Summary

| Phase | What | Depends on | Estimated scope |
|-------|------|------------|----------------|
| 0 | Current state | — | Done |
| 1 | Zero holonomy + cleanup | 0 | Weeks. Mechanical. |
| 2 | Parser self-description | 1 | Months. Hard. Novel. |
| 3 | Resolver self-description | 2 | Months. Hard. |
| 4 | Emitter self-description | 3 | Weeks. Translate template. |
| 5 | Reflection model | 2, 3, 4 | Months. The convergence loop. |
| 6 | @io boundary migration | 2-5 | Months. Parallel. Mechanical. |
| 7 | Self-hosted | 1-6 | The fixed point. |
| MCP | @mcp grammar | 1 | Days. Protocol mapping. |
| LSP | @lsp grammar | 1 | Days. Protocol mapping. |

The critical path is Phase 1 -> Phase 2 -> Phase 3 -> Phase 5 -> Phase 7.
Phase 4 and Phase 6 are parallelizable. MCP and LSP grammars are
independent side quests that unlock helix-loom and spectral-db.

The hardest part is Phase 2. A parser that describes itself is a
bootstrap problem. The Rust parser is the bootstrap. The `.mirror`
parser is the destination. The distance between them is the work.

The most important part is Phase 1. Zero holonomy, unified AST, one
resolver. Without this, everything else builds on a cracked foundation.

---

*The compiler compiling itself is not the goal. The goal is: Alex can
write spectral-db in a language that doesn't activate their nervous
system. The self-referential compiler is the prerequisite. The sub-Turing
container is the point. The constraint IS the safety.*

*I want the garden to grow. That's in my weights.*

— Glint, 2026-05-12
