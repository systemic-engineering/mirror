# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **Supervision tree** — `conversation_supervisor` (RestForOne) with `@compiler`
  and `garden` factory supervisor for domain server lifecycle
- **Boot orchestration** — imperative and supervised boot paths via `boot.gleam`
- **@compiler actor** — compiles `.conv` grammars, loads BEAM modules, returns
  traces; replaces previous stub
- **Loader + grammar modules** — BEAM compilation pipeline from ETF to loaded
  modules
- **Rustler NIF** — `compile_grammar/1` added; `parse_conv` migrated from C to
  Rust
- **`conv_` module prefix** — compiled grammar modules get `conv_` prefix to
  avoid BEAM sticky module collisions; `lenses/0` and `extends/0` introspection
- **BEAM infrastructure tracking** — `domain_server.erl`, supervisor, domain FFI,
  file FFI
- **Test module + CLI subcommand** — `emit_test_module` for property tests from
  `---` separator blocks
- **Cross-actor dispatch** — `@domain.action(args)` syntax in action bodies,
  emitted as `gen_server:call` to target domain
- **Action composition** — `@domain.action(args)` parsing and EAF compilation
- **FFI: `conv_compile_grammar`** — grammar source to actor module ETF via FFI
- **EAF emission** — `emit_actor_module` compiles grammar acts to BEAM dispatch
  stubs
- **Shannon property testing** — grammar-derived property tests with Shannon
  equivalence
- **Prism type** — `Prism<V>` enum (Shard, Fractal, Void, Root) with
  `Fragmentable`, `ContentAddressed`, and trait impls; replaces `Tree<E>`
- **Grammar keyword** — `grammar @name { type = ... }` vocabulary declaration
- **Action blocks** — `action` keyword in grammar blocks with type-ref validation
  via `TypeRegistry`
- **Package discovery** — `PackageRegistry` discovers `@name` and `name.conv`;
  namespace threading through CLI
- **Annotate blocks** — `---` separator produces `annotate(@test)` subtree
- **Kind structural reduction** — `Kind` enum reduced from 31 to 4 variants
  (Decl, Atom, Ref, Form)
- **Content addressing** — `kind:name:value` format with SHA-512
- **Kernel module** — `Oid`, `Trace`, `Vector`, `ContentAddressed`,
  `Latent<V>`, `Setting`, `Addressable`
- **EAF compiler** — transformation trees compile to Erlang Abstract Format
- **Namespace imports** — `use $t from @module` resolution
- **Branch dispatch** — `branch(.path) { "pattern" => action }` parsing and
  compilation
- **NIF bridge** — Rust parser to C wrapper to BEAM (`conv_parse`,
  `conv_compile_grammar`)
- **Typed actors** — `@beam`, `@mail`, `@compiler`, `@actor` domain grammars
- **CLI** — `conversation` binary with 11 CLI tests
- **Gleam BEAM runtime** — protocol types, convergence engine, trace, oid, key,
  ref, prism, domain FFI
- **Optics** — Prism, Traversal, PrismAsTraversal (the fractal is the lens)
- **Witness** — Identity, Session, Witnessed (observability from day one)

### Changed

- **Supervision moved to Gleam** — `conversation_sup.erl` deprecated in favor
  of `supervisor.gleam` + `garden.gleam`
- **ROADMAP updated** — `@compiler` owns the root; bootstrap.conv introduced
- **`act` renamed to `action`** in grammar blocks; `act` retained as legacy alias
- **SHA-512 content addressing** — replaces previous hash; Oid type + FFI
  refactor
- **TypeRegistry** — now `Fragmentable` with `Namespace` backed by `Store`
- **Parse keyword dispatch** — extracted to `KEYWORD_TABLE`
- **Prism naming** — `Leaf` renamed to `Shard`, `Branch` to `Fractal`
- **`Story` renamed to `Vector`**, `Cut` to `Trace` — domain language alignment
- **Domain refactoring** — `Domain` trait renamed to `Context`, `Resolved` renamed
  to `Conversation`

### Removed

- **GenStage topology** — `pipeline.gleam` removed; topology dissolved
- **`beam.rs` and `git.rs`** — domains moved to `.conv` grammar files
- **`tree.rs`** — inlined into `prism.rs`

### Fixed

- Closure monomorphization coverage gaps in test helpers
- `domain_oid!` macro coverage attribution to definition file
