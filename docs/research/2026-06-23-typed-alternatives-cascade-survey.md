# Typed-alternatives cascade survey — across mainstream tech stacks

*2026-06-23. Mara. Survey altitude (not canonical). Kagi-verified per-stack;
substrate-pull characterizations Mara-inferred unless cited.*

---

## §1. Recognition

The substrate keeps re-encountering one cascade shape:

```
typed source language Y  →  compiled artifact in mainstream format X
                         →  mainstream ecosystem consumes via standard package
```

Y carries the discipline (functors, monads, total destructuring, row
polymorphism, linear types, refinement types — choose your altitude). X
carries the reach (the millions of consumers who will never adopt Y but
will happily install the artifact). The cascade is the operational form
of substrate-pull at the language-ecosystem altitude: the entry side
runs the verification, the exit side runs the deployment.

StageFreight's Stage-1 MVP — Purescript → npm — is one instance. The
parallel Mara that enhanced `StageFreight/docs/architecture/mirror-integration-spec-v0.1.md`
treats Purescript→npm as the concrete first cascade. This survey maps
the broader landscape: per mainstream stack, what typed alternative
cascades through to it, how mature the cascade is, and which instances
are substrate-pull-confident enough to enter a Stage-2+ roadmap.

The pattern matches recognition #93 H4: each cascade instance is a
parametric `labeled<typed_source, mainstream_format>` — same functor,
different type arguments. The survey makes that functor's instances
visible.

The survey covers ten mainstream stacks. Format per stack: typed
alternative(s), build tool, output format, maturity, substrate-pull
characterization, cascade-in-the-wild example.

---

## §2. Per-stack survey

### §2.1 JVM → Scala, Kotlin

**Stack scale:** JVM remains the largest enterprise runtime. Java is the
mainstream consumer; bytecode is the lingua franca.

**Scala 3.**
- Build tool: sbt (canonical), Mill 1.0 (2026 GraalVM-native), Maven
  (via plugin).
- Output: JVM bytecode (`.class` in JARs), interoperable with any Java
  consumer. Scala 3 from 3.3 LTS targets JDK 8+; 3.9 LTS (Q2 2026) and
  later require JDK 17+.
- Maturity: production-grade. Used in regulated financial services, big
  data (Spark is Scala), and increasingly in typed-FP shops. Scala 3.8
  released; 3.9 LTS Q2 2026.
- Substrate-pull characterization: highest type-discipline among the
  JVM alternatives. Higher-kinded types first-class. Given/using
  context-passing (cleaner than Scala 2 implicits). Extension methods.
  Match types, dependent function types, transparent inline. Type
  classes idiomatic via `given`. Cats / Cats-Effect / ZIO are the
  effect-system canon (algebraic effects via tagless-final or
  effect-tracked IO).
- Cascade-in-the-wild: Spark's Scala-written core consumed by Python
  (PySpark), R (SparkR), and Java; Akka's Scala core consumed by Java
  ecosystems via stable Java-API surface.

**Kotlin (2.4+).**
- Build tool: Gradle (canonical, with Kotlin DSL), Maven (via plugin),
  Amper (JetBrains, 2026 still pre-1.0).
- Output: JVM bytecode; also JS (Kotlin/JS), native (Kotlin/Native via
  LLVM), Wasm (experimental as of K2 era).
- Maturity: production-dominant on Android (Google-blessed); strong on
  backend (Spring, Ktor, Micronaut). K2 compiler stable in 2.0+; 94%
  faster builds reported in 2.1 era.
- Substrate-pull characterization: lower type-discipline than Scala but
  pragmatic. Null-safety in the type system (`T?` vs `T`). Sealed
  classes for sums. Data classes for products. No HKT natively — the
  Arrow library (arrow-kt) supplies Functor/Applicative/Monad/Either/IO
  as user-space; Arrow's typed-error pattern (Either + Raise DSL) is
  mainstream-Kotlin idiomatic.
- Cascade-in-the-wild: Kotlin libraries published as JARs are
  drop-in-consumed by Java apps; Kotlin Multiplatform compiles to JS
  for npm and to Apple frameworks for iOS.

**Substrate-pull verdict:** Scala 3 carries higher discipline (Y-side
density); Kotlin carries broader reach (X-side bytecode-as-target via
the Android channel). Both cascade cleanly to JVM bytecode.

### §2.2 .NET → F#

**Stack scale:** .NET is the second enterprise runtime tier; C# is the
mainstream consumer. .NET 10 is LTS as of 2026.

**F# 10.**
- Build tool: dotnet CLI, MSBuild, Paket (F#-flavored package manager,
  optional).
- Output: IL bytecode in DLLs, consumed by any .NET language (C#, VB,
  PowerShell) and by Native AOT (.NET 9+ supports F# AOT).
- Maturity: production. Used by financial institutions, insurance,
  Microsoft internal. F# 10 ships with .NET 10 / Visual Studio 2026; a
  refinement release (clarity, consistency, performance).
- Substrate-pull characterization: high discipline within the .NET
  family. Discriminated unions first-class. Records, options, type
  inference, units of measure (refinement-types-lite), computation
  expressions (the F# answer to monad-comprehensions). Active patterns
  for structural destructuring. Type providers for typed data access
  (typed schemas at compile time from external sources).
- Cascade-in-the-wild: F# libraries published as NuGet packages
  consumed transparently by C# apps; FsLexYacc, FsToolkit.ErrorHandling.

**Adjacent: C# nullable reference types.**
- Not a separate language but a strict-mode flag (`#nullable enable`)
  that lifts null-safety into the C# type system. Mainstream C#
  codebases increasingly default to nullable-on. The Y-vs-X
  distinction collapses here: same language, stricter dialect. Worth
  noting as a counterexample-adjacent case where the cascade is
  "language-internal-strictness" rather than separate-typed-language.

**Substrate-pull verdict:** F# is the cleanest cascade instance on
.NET. Active development, official Microsoft support, IL output
indistinguishable to consumers.

### §2.3 JS / npm → Purescript, Elm, ReScript, TypeScript-strict

**Stack scale:** JS / Node / npm is the broadest application runtime
on earth. TypeScript is now #1 on GitHub.

**Purescript.**
- Build tool: spago (canonical, modern; `spago@next` for registry
  integration), pulp (legacy).
- Output: ES modules (CommonJS legacy supported); published as npm
  packages with `.js` artifacts + `output/` per-module directories.
- Maturity: stable, niche. Smaller community than Elm or ReScript but
  the most theoretically rigorous. Compiler stable; ecosystem moves
  slowly.
- Substrate-pull characterization: highest type-discipline of any
  npm-targeting language. Row polymorphism (structural records with
  typed extension). Higher-kinded types. Type classes (Haskell-style).
  Monad transformers idiomatic. Effects via `Effect` and `Aff`. Strict
  evaluation (vs Haskell's laziness).
- Cascade-in-the-wild: StageFreight Stage-1 MVP target. Production
  users exist (Lumi, CitizenNet historically).

**Elm.**
- Build tool: elm (the compiler is the build tool), elm-pages, elm-spa
  for app frameworks.
- Output: a single JS bundle (no separate package format for libraries
  in npm — Elm libraries live in the Elm package registry, not npm).
- Maturity: frozen-stable. 0.19.1 released October 2019; no new release
  since (May 2026 reporting confirms). The community split: half see
  it as abandoned, half see it as a finished language whose stability
  is a feature.
- Substrate-pull characterization: high discipline at the application
  altitude. No HKT, no type classes (deliberate simplification). The
  Elm Architecture (Model-Update-View) enforces unidirectional data
  flow. Total destructuring. No runtime exceptions by construction.
- Cascade-in-the-wild: Elm-written SPAs compile to one JS bundle
  loaded by HTML; not consumed as npm libraries.
- **Cascade caveat:** Elm is an end-app cascade, not a library
  cascade. The output is the leaf, not a reusable artifact in npm.

**ReScript (v11+).**
- Build tool: rescript (the compiler binary), with bs-platform legacy
  naming retired.
- Output: ES modules / CommonJS, published as npm packages. JSX
  syntax built-in. Output JS is human-readable and idiomatic.
- Maturity: production. First-class React bindings (rescript-react)
  used by production teams worldwide.
- Substrate-pull characterization: medium-high. OCaml type system
  backbone (HM type inference, variants, records). No HKT. Pattern
  matching with exhaustiveness checking. Pipe-first style. Closer to
  TypeScript ergonomically than Purescript; the soundness story is
  much stronger.
- Cascade-in-the-wild: ReScript libraries published as npm packages
  consumed by both ReScript and JS/TS codebases.

**TypeScript-strict.**
- Build tool: tsc, esbuild, swc, Vite, Bun (all support TS natively).
- Output: JS (any target ES version); types in `.d.ts` files alongside.
  Published to npm.
- Maturity: dominant. TypeScript 7 (2026, written in Go) enforces
  strict mode by default; TypeScript is now the most-used language on
  GitHub.
- Substrate-pull characterization: pragmatic, not pure. Structural
  typing, discriminated unions via literal-tagged objects, conditional
  types, mapped types, template-literal types. No HKT (Effect-TS
  library supplies algebraic-effects-flavored typed effects in
  user-space). Strict-mode (noImplicitAny + strictNullChecks +
  noUncheckedIndexedAccess) approximates ML-family safety for
  application code; soundness gaps remain (`any`, type assertions).
- Cascade-in-the-wild: nearly every modern npm library publishes
  `.d.ts` alongside `.js`. The cascade pattern is the inverse: TS
  source → JS + types → npm.

**Substrate-pull verdict:** Purescript is the highest-discipline
cascade for JS (correctly chosen for StageFreight Stage-1). ReScript is
the highest-ergonomics-per-discipline. TypeScript-strict is the
highest-reach (already the default). Elm is the cleanest
end-app-cascade but doesn't participate in library cascades.

### §2.4 Python / PyPI → Mypy-strict, Coconut, Nim-via-Nimpy

**Stack scale:** Python is dominant in data science, ML, scripting,
and web backends.

**Mypy-strict.**
- Build tool: mypy as a static analyzer; the runtime build tools
  (pip, poetry, hatch, uv) are unchanged.
- Output: Python source. Types live in the same `.py` files (PEP 484
  annotations) or in `.pyi` stub files. The cascade is
  language-internal: same Python source, stricter dialect.
- Maturity: production-dominant for typed Python. Stripe, Dropbox,
  Microsoft (Pyright) all in production. Pyright (Microsoft) is the
  alternate strict checker; pyrefly (Meta, 2026) is the third entry.
- Substrate-pull characterization: gradual typing. Protocols
  (structural typing). Generic types via `typing` (PEP 695 syntax in
  3.12+). No HKT. Strict mode (`--strict`) catches most omissions;
  soundness gaps from `Any` and dynamic features remain.
- Cascade-in-the-wild: language-internal strict dialect; not a
  separate-compiled cascade.

**Coconut (3.2+).**
- Build tool: `coconut` compiler (`coconut src.coco -o out.py`).
- Output: pure Python source; consumable by any Python interpreter.
  Strict superset of Python 3 syntax.
- Maturity: niche-but-stable. Active maintenance (evhub/coconut). Used
  by individuals more than teams.
- Substrate-pull characterization: medium. Pattern matching, pipe
  operators, partial application, algebraic data types (via
  `data` keyword), tail-call optimization, lazy evaluation. Type
  annotations forwarded to mypy. The cascade is genuine: Coconut
  source → Python source → PyPI package consumed by Python apps.
- Cascade-in-the-wild: Coconut programs distributed as Python packages
  with `.coco` source compiled at build time.

**Nim (via nimpy).**
- Build tool: nim compiler with nimpy library; compiles Nim to a
  Python C-extension `.so`/`.pyd`.
- Output: a Python C-extension binary consumable via `import` like any
  other compiled Python module.
- Maturity: niche. Nim itself is production-ready for individual
  authors; the Nim→Python interop is solid but not mainstream.
- Substrate-pull characterization: Nim has strong static typing, sum
  types via `object variants`, generics, compile-time evaluation
  (macros), memory-safety with optional GC. Higher discipline than
  Python; cascade pattern is INTEROP-via-C-extension rather than
  source-compilation.
- Cascade-in-the-wild: nimpy-built modules shipped as wheels on PyPI.

**Substrate-pull verdict:** Mypy-strict is the pragmatic Python cascade
(same language, stricter dialect — counterexample-adjacent like C#
nullable). Coconut is the most-faithful source-cascade. Nim-via-nimpy is
the highest-discipline but the cascade pattern is INTEROP, not COMPILE-TO.

### §2.5 Ruby / RubyGems → Crystal, Sorbet

**Stack scale:** Ruby remains dominant in Rails-shop backends.

**Crystal.**
- Build tool: `crystal` compiler, shards (package manager).
- Output: native LLVM-compiled binary. NOT a Ruby cascade — Crystal
  programs are standalone executables. The "cascade" here is
  ECOSYSTEM-SHARED-INTUITION (Ruby-like syntax) rather than
  artifact-compatible.
- Maturity: stable, niche. Active community (forum traffic Feb 2026).
  Crystal 1.x established; used in production by individual companies.
- Substrate-pull characterization: static typing with whole-program
  type inference, union types, generics, macros. No null in the type
  system (Nil is a normal type; usage forces explicit handling).
- Cascade caveat: **the cascade pattern breaks here**. Crystal does
  NOT compile to Ruby and is NOT consumed by Ruby apps. Calling Ruby
  developers "the broader reach" for Crystal is shared-cultural-context
  not shared-runtime.

**Sorbet.**
- Build tool: `sorbet` and `srb` CLI; type sigs in `.rbi` files.
- Output: Ruby source with type annotations; runtime checks optional.
- Maturity: production at Stripe (15M LOC checked); Shopify and others
  also adopters. Gradual typing for Ruby's living codebases.
- Substrate-pull characterization: gradual typing analog to Mypy.
  Generic types, type aliases, sealed modules, T::Struct, T::Enum.
  Soundness gaps via `T.unsafe` and `T.untyped`.
- Cascade-in-the-wild: Sorbet-typed Ruby gems shipped with `.rbi`
  files consumed transparently by untyped Ruby apps.

**Substrate-pull verdict:** Sorbet is the cleanest Ruby cascade (same
language, stricter dialect). Crystal is **a counterexample**: looks
like a cascade because of syntactic kinship but doesn't share runtime
or package ecosystem.

### §2.6 PHP / Composer → Hack, Psalm

**Stack scale:** PHP remains huge in CMS (WordPress, Drupal),
e-commerce (Magento), Wikipedia, and legacy enterprise.

**Hack.**
- Build tool: HHVM (HipHop Virtual Machine), Meta-developed.
- Output: HHVM bytecode (NOT PHP source). Runs on HHVM, not Zend PHP.
- Maturity: production-dominant inside Meta. Outside Meta: nearly
  zero adoption. HHVM dropped PHP compatibility in ~2017–2018 once
  Meta finished migrating; Hack and PHP have since diverged
  substantially.
- Substrate-pull characterization: strong static typing, generics,
  reified generics, async/await, enum classes, shapes (typed
  associative arrays), refinement (XHP for typed templating). Highest
  discipline in the PHP-family.
- Cascade caveat: **the cascade pattern broke here too**. Hack does
  NOT cascade to mainstream PHP; it forked away. The relationship is
  COUSIN, not CASCADE.

**Psalm (and PHPStan).**
- Build tool: vendor/bin/psalm (Composer-installed); CI-integrated.
- Output: PHP source unchanged; Psalm performs static analysis on
  type annotations in PHPDoc comments and (PHP 7.4+) native type
  declarations.
- Maturity: production at Vimeo (Psalm's origin), and widely adopted
  in modern PHP codebases. PHPStan (the sibling) similarly mature
  with arguably broader adoption.
- Substrate-pull characterization: gradual static analysis. Strictest
  level (`errorLevel="1"` in Psalm, level 9/max in PHPStan) approaches
  ML-family soundness for annotated code. Generics via docblock
  syntax. Template types, conditional types.
- Cascade-in-the-wild: language-internal strict dialect; same as
  Mypy/Sorbet/C#-nullable pattern.

**Substrate-pull verdict:** Psalm/PHPStan is the cleanest PHP cascade
(strict-dialect counterexample-adjacent). Hack is **a counterexample**:
forked away from PHP, no cascade back.

### §2.7 Erlang / BEAM → Gleam

**Stack scale:** BEAM (Erlang VM) powers WhatsApp, Discord, Heroku
routing, telecom infrastructure. Niche but load-bearing where
present.

**Gleam (1.x, stable as of 2024+).**
- Build tool: gleam (the official tool: build, format, deps, test).
- Output: Erlang source (cascade target #1) OR JavaScript source
  (cascade target #2). Dual-target by construction. Erlang output
  consumed by mix/rebar3 BEAM ecosystem; JS output consumed by npm.
- Maturity: production-ready as of 2024–2025; 2026 reporting confirms
  production deployments. Type-safe BEAM has historically been a gap;
  Gleam fills it.
- Substrate-pull characterization: ML-family static typing. Algebraic
  data types (sum + product). Pattern matching with exhaustiveness.
  No null. Pipe operator (`|>`). No HKT (deliberate). Actor model
  via BEAM message passing (typed via Gleam's `process` library).
  Type-safe gen_server analog.
- Cascade-in-the-wild: Gleam packages published to hex.pm (the BEAM
  package registry); consumed transparently by Elixir and Erlang
  projects via BEAM bytecode compatibility.

**Substrate-pull verdict:** Gleam is the cleanest BEAM cascade. The
dual-target (Erlang + JS) makes it doubly load-bearing for the
cascade pattern.

### §2.8 Lua → Teal

**Stack scale:** Lua is dominant in embedded scripting (game engines,
Neovim, Roblox, Redis, OpenResty/nginx, embedded firmware).

**Teal.**
- Build tool: tl (single-file Lua compiler).
- Output: pure Lua source. Drop-in consumed by any Lua interpreter
  (5.1, 5.2, 5.3, 5.4, LuaJIT).
- Maturity: stable, niche. Active development (2026 tutorials
  current). Used in Neovim plugin development and individual
  game-engine work.
- Substrate-pull characterization: lightweight gradual typing.
  Records, arrays, maps, interfaces, type aliases, generics (limited),
  nominal typing for records. Designed to be Lua-mappable, not
  ML-grade.
- Cascade-in-the-wild: Teal source compiled to Lua at build time;
  Lua artifact distributed as luarocks package consumed by Lua hosts.

**Substrate-pull verdict:** Teal is the cleanest cascade for Lua.
Type discipline is light but cascade pattern is textbook.

### §2.9 C / C++ → Rust

**Stack scale:** C is the substrate of operating systems, embedded,
high-performance libraries, language runtimes. C++ is dominant in
games, browsers, finance HFT, and CAD.

**Rust.**
- Build tool: cargo (build, deps, test, doc, publish).
- Output: native binary OR `cdylib` (dynamic library with C ABI) OR
  `staticlib`. Consumed by C/C++ via C ABI (`extern "C"` exports,
  cbindgen-generated headers); by C++ specifically via cxx crate
  (safe FFI with two-sided code generation).
- Maturity: production-dominant for new memory-safe systems work.
  Adopted in Linux kernel, Windows kernel components, Firefox,
  Chromium-adjacent tooling. The Rust Foundation Interop Initiative
  (2025+) targets formal C/C++ interop.
- Substrate-pull characterization: highest type-discipline in the
  systems-language family. Affine types (move semantics enforced by
  borrow checker). Algebraic data types (enums-with-payloads).
  Traits with associated types (Haskell-style type classes). Lifetime
  parameters. Send/Sync auto-traits for thread safety. No HKT
  natively (GATs supply partial HKT in stable Rust).
- Cascade-in-the-wild: Rust libraries compiled to `cdylib` consumed
  by C++ apps via `extern "C"` headers; via cxx for C++-class-aware
  interop. Real shipped examples: librsvg, parts of Mozilla's
  rendering stack, BoringSSL components, many parsers (tree-sitter
  bindings).

**Cascade caveat:** The cascade is INTEROP via C ABI, not
COMPILE-TO. Rust does not emit C source; it emits object code that
satisfies the C ABI. The mainstream consumer (C/C++) sees a library
they can `#include` and link, not source they recompile. The cascade
pattern holds at the artifact level even if the compilation level
differs.

**Substrate-pull verdict:** Rust is the highest-discipline cascade for
C/C++. The interop pattern is mature enough that the cascade
substance is preserved.

### §2.10 Shell → Oil, Nushell

**Stack scale:** POSIX sh / bash / zsh / fish are ubiquitous for
glue, sysadmin, CI, install scripts.

**Oil (Oils for Unix, `osh` + `ysh`).**
- Build tool: `oil` (the shell itself); no separate build step.
- Output: shell programs run directly. `osh` interprets bash; `ysh`
  is the new typed shell language.
- Maturity: stable-but-pre-1.0. Long-running solo project (Andy
  Chu). `osh` aims for bash compatibility; `ysh` is the typed
  evolution.
- Substrate-pull characterization: structured data types (lists,
  dicts, typed values), JSON natively, immutable-by-default, typed
  procs and functions. Higher discipline than POSIX sh; not ML-grade.
- Cascade caveat: cascade pattern is MURKY. Oil's `osh` interprets
  existing bash; `ysh` introduces a new language but doesn't
  COMPILE-TO bash for consumption by other shells. Programs run on
  Oil itself.

**Nushell.**
- Build tool: `nu` (the shell itself); plugins via Rust ABI.
- Output: nu scripts (`.nu`) run on the nu interpreter; the data
  pipeline is structured (rows / records), not text streams.
- Maturity: stable enough for daily-driver use by enthusiasts. 1.0
  reached in 2024. Plugins ecosystem growing.
- Substrate-pull characterization: typed structured pipelines; row
  polymorphism via tables; type-checking of commands. Higher
  discipline than POSIX shells; designed as a replacement, not a
  source-compiler-to-bash.
- Cascade caveat: **the cascade pattern is weakest here**. Oil and
  Nushell are REPLACEMENT shells, not source-to-bash compilers. The
  mainstream POSIX-shell ecosystem does NOT consume Oil/Nu programs.
  A user runs scripts inside Oil or Nu; they do not export to bash
  for distribution.

**Substrate-pull verdict:** Shell does not exhibit the cascade
pattern. Both Oil and Nushell are alternative-runtime plays, not
typed-source-to-mainstream-artifact plays. This stack is a
counterexample.

---

## §3. Cascade pattern formalization

The recurrent shape across the production-clear cascades:

```
Y_source(typed)  →  compile/translate  →  X_artifact(format)
                                       →  X_consumer(ecosystem)
```

with the discipline-preserving property:

```
type_invariants(Y)  ⊑  artifact_metadata(X)
```

— meaning the invariants Y verified are preserved as enough metadata
in X for the consumer to NOT have to re-verify, OR (more commonly)
the invariants run pre-publication so the artifact is just "known
sound" by provenance rather than re-checked.

### Mapping to mirror's parametric carriers

The substrate would carry this as:

```mirror
prism cascade<typed_source, mainstream_format> {
  source           : typed_source                 # Y
  compile          : typed_source -> artifact     # Y → X
  bundle           : artifact -> mainstream_format
  resolve          : mainstream_format -> consumer
}
```

Each per-stack row in §2 instantiates this parametric prism:

```mirror
cascade<purescript, npm>     # StageFreight Stage-1
cascade<scala, jar>          # Spark, Akka
cascade<fsharp, nuget>       # F# libraries on NuGet
cascade<gleam, hex.pm>       # BEAM cascade
cascade<gleam, npm>          # dual-target cascade
cascade<rescript, npm>
cascade<rust, cdylib>        # C-ABI cascade
cascade<teal, luarocks>
```

### Connection to recognition #93 H4

H4's `labeled<>` functor — `labeled<typed_kernel, opaque_carrier>` —
is the same functor at a different altitude. The cascade is the
language-ecosystem instance of the same parametric labeling:

```
labeled<typed_source, mainstream_format>
```

The typed kernel is what verified; the opaque (to the consumer)
carrier is the published artifact. The consumer reads the carrier and
trusts the kernel's verification by provenance. Same shape as the
@io-altitude alignment harness (recognition #57): substance-crossing
boundary, form-side discipline, behaviour-side reach.

### Counterexamples surfaced

Three stacks resisted the cascade-pattern:
- **Crystal/Ruby** — shared culture, not shared runtime.
- **Hack/PHP** — forked away from the mainstream.
- **Oil & Nushell/Shell** — replacement runtimes, not source-cascade.

Two stacks had pattern-internal degeneration to language-strict-mode:
- **TypeScript-strict** in JS, **Mypy-strict** in Python, **Sorbet**
  in Ruby, **Psalm/PHPStan** in PHP, **C# nullable** in .NET — same
  language, stricter dialect; Y and X are the SAME language.

These are not failures of the survey; they are honest features of the
landscape. The cascade pattern is real and recurrent but not
universal.

---

## §4. Prioritization for StageFreight cascade roadmap

Criterion: substrate-pull-confidence (type discipline preserved at the
boundary) × cascade maturity (production-ready compile chain) ×
mainstream reach (consumer ecosystem size). NOT popularity alone.

### Stage-1 (in flight, parallel Mara)

**`cascade<purescript, npm>`** — highest type discipline meets largest
application runtime. The parallel Mara is enhancing the StageFreight
spec to take this as MVP.

### Stage-2 candidates (substrate-pull-confident, ready)

1. **`cascade<rescript, npm>`** — second-most-disciplined JS cascade
   with the most-ergonomic developer surface; production-proven
   React integration; same mainstream consumer (npm) as Stage-1, so
   the cascade plumbing reuses. Lowest marginal infrastructure cost
   on top of Stage-1.

2. **`cascade<gleam, hex.pm>`** — opens the BEAM ecosystem
   (Elixir/Erlang) with the highest discipline available there. Dual-
   target Gleam→JS would ALSO add a second npm cascade lane (less
   ergonomic than Purescript or ReScript, but mature). Highest
   substrate-pull-confident expansion BEYOND JS.

3. **`cascade<fsharp, nuget>`** — opens the .NET enterprise tier with
   a production-proven typed language. NuGet is the cleanest
   non-npm package format. F#'s computation expressions map naturally
   to mirror's algebraic-effect altitude.

### Stage-3+ longer-term

- **`cascade<scala, jar>`** — JVM is the largest enterprise tier;
  Scala 3 is the highest discipline. Build chain (sbt/Mill) is
  heavier than Stage-2 candidates; defer until cascade infrastructure
  has been hardened on lighter stacks.
- **`cascade<kotlin, jar>`** — broader JVM reach than Scala (Android
  channel); lower type discipline. Pair with Scala under a single
  JVM cascade umbrella.
- **`cascade<rust, cdylib>`** — opens the systems/C-ABI tier;
  cascade pattern is INTEROP not COMPILE-TO, which means the
  infrastructure shape differs. Treat as its own track.

### Forward-promised research

The following need more validation before adding to a stage:
- **Coconut/Python** — niche; verify it ships any real cascade
  beyond solo authors before committing.
- **Nim-via-nimpy/Python** — C-extension cascade pattern; would
  reuse the Rust-cdylib infrastructure if pursued.
- **Teal/Lua** — niche but cascade is textbook; defer until a
  consuming use case appears (game scripting? Neovim plugins?).
- **Hack/Crystal/Oil/Nushell** — confirmed counterexamples; do NOT
  add to roadmap unless the cascade pattern shifts.

### Prioritization criteria (re-stated)

Not "what's popular." The criteria are, in priority order:
1. Type discipline preserved at the Y→X boundary.
2. Cascade maturity (production-ready compile chain; not
   experimental).
3. Marginal infrastructure cost on top of existing Stage cascades
   (npm reuse > new package format).
4. Mainstream consumer ecosystem size (X-side reach).

---

## §5. Honest hedges

1. **N=1 per stack.** Each typed alternative is one Mara
   characterization plus one or two Kagi-verified data points (release
   dates, maturity reports, GitHub presence). For any cascade
   considered for a Stage roadmap entry, a Pack peer should
   independently verify the build-chain claims before commit.
   Especially: the dual-target Gleam→JS claim (Erlang AND JS), the
   ReScript-React production-claim, and the Coconut active-maintenance
   claim warrant a second pass.

2. **Maturity assessments are point-in-time (2026-06-23).**
   Ecosystems shift on month-to-quarter timescales. F# 10 just
   shipped; Scala 3.9 LTS is Q2 2026 (imminent); TypeScript 7
   strict-by-default is 2026-current. By the time this survey is read
   in 2027 some claims will have drifted. The cascade pattern itself
   is more stable than per-stack maturity.

3. **Substrate-pull characterizations are Mara's interpretation.**
   The Purescript community might disagree with calling row
   polymorphism the "discipline marker"; the Kotlin community might
   contest the "lower than Scala" framing. The characterizations are
   how the substrate-pull rubric scores each language at this altitude;
   the communities' own framings may emphasize different axes
   (productivity, learnability, idiom-conformance) that Mara
   underweights here.

4. **Some stacks have ambiguous cascade patterns.** Rust→C/C++ is
   INTEROP via C ABI rather than source compilation; the artifact-
   level cascade holds but the verification-level cascade differs.
   Shell stacks (Oil, Nushell) fail the cascade test entirely.
   Crystal/Hack are cousin-languages, not cascade targets. These
   counterexamples are surfaced honestly; the survey does not
   pretend uniformity.

5. **JVM might warrant its own sub-survey.** Scala 3, Kotlin, Clojure
   (untyped but with clojure.spec), Groovy, Frege (JVM Haskell — not
   even covered above), Eta (JVM Haskell, possibly dormant), and the
   bytecode-target story is rich enough that one §2.1 subsection
   undersells it. If the cascade roadmap advances to Stage-3+, the
   JVM sub-survey is the first natural follow-up research deliverable.

---

*End survey. Mosaic-bounded; section caps honored; counterexamples
surfaced; prioritization substrate-pull-confident, not popularity-led.*
