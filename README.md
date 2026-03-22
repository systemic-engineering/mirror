# conversation

> **Pre-v0.1** — This project is in active development. APIs, file formats,
> and architecture may change without notice. Not yet suitable for production use.

![Coverage](https://img.shields.io/badge/coverage-100%25_lines-brightgreen)
![Tests](https://img.shields.io/badge/tests-599_passing-brightgreen)
![License](https://img.shields.io/badge/license-systemic.engineering-blue)

Stories over trees.

A typed transformation pipeline language. `.conv` files describe desired state.
The Rust parser produces a `Prism<AstNode>`. The BEAM runtime converges toward it.

## Architecture

```
main.conv ──→ [Rust] ──→ Prism (projection matrix)
                              │
                    git commit to .git/refs/conversation/<branch>
                    author: conversation@systemic.engineering
                              │
                             OID ──→ [BEAM]
                              │
                    [Fortran NIF] ← matmul, preview, review, modify, compose
```

**Rust** (`src/`): Parses `.conv` source into a `Prism<AstNode>` (projection matrix),
generates the Fortran wiring + NIF bridge for matrix operations, commits the Prism
to `.git/refs/conversation/<branch>` as `conversation@systemic.engineering`, and
hands the OID to the BEAM. The branch is a Lens in the Prism pointing to HEAD.

**Fortran**: Basic linear algebra on the Prism. The matrix never leaves its native
representation — no ETF, no JSON, no lossy intermediate format. Rust generates the
`.f90` and the NIF glue.

**BEAM** (`beam/`): Gleam. Receives the OID. Operates on the Prism through the
Fortran NIF. Owns resolution, compilation, package discovery, and actor lifecycle.
The `@conversation` actor extends `@compiler`.

### Rust modules

| Module | Purpose |
|--------|---------|
| `kernel.rs` | Oid, Trace, Vector, ContentAddressed — core algebra |
| `ast.rs` | AstNode, Kind enum — the AST types |
| `parse.rs` | `.conv` source → `Prism<AstNode>` |
| `prism.rs` | Content-addressed tree structure |
| `ffi.rs` | `conv_parse`, `conv_compile_grammar` — FFI entry points |
| `compile.rs` | EAF emission — grammar → BEAM module bytecode |
| `domain/` | Domain implementations (filesystem) |
| `filter.rs` | Tree filtering (hash, sign, encrypt) |
| `generate.rs` | Type derivation from grammar registries |

Modules being superseded by BEAM (do not extend):
`resolve.rs`, `packages.rs`, `property.rs`

### BEAM modules (`beam/src/conversation/`)

| Module | Purpose |
|--------|---------|
| `compiler.gleam` | @compiler actor — compiles grammars, loads BEAM modules |
| `supervisor.gleam` | Static supervisor — @compiler + garden (RestForOne) |
| `garden.gleam` | Factory supervisor — domain server lifecycle |
| `boot.gleam` | Boot orchestration — imperative and supervised paths |
| `domain.gleam` | FFI bindings to domain_server.erl |
| `loader.gleam` | BEAM module loading (ETF → forms → binary) |
| `trace.gleam` | Witnessed, signed records |
| `oid.gleam` | Content-addressed identity (SHA-256) |
| `key.gleam` | Ed25519 actor identity |
| `ref.gleam` | Scoped content-addressed references |
| `prism.gleam` | Projection matrix (Fortran NIF) |
| `nif.gleam` | Bridge to Rust parser |
| `protocol.gleam` | Spec types — desired BEAM state |
| `runtime.gleam` | Convergence engine — delta computation |

### Supervision

```
conversation_supervisor (static_supervisor, RestForOne)
├── @compiler              — compiles grammars, loads BEAM modules, returns traces
└── garden (factory_sup)   — dynamic domain server lifecycle
```

**@compiler** starts first. Compiles `.conv` source → calls Rust NIF → loads BEAM
module → returns `Trace(CompiledDomain)`. In supervised mode, does not start domain
servers directly.

**garden** starts second. Factory supervisor that manages domain servers as dynamic
children. When a grammar is compiled, `garden.start_domain(name, domain)` starts
its GenServer under the factory. If a domain server crashes, the garden restarts it.

**RestForOne**: if @compiler crashes, the garden and all domain servers restart
(clean slate). If a single domain server crashes, the factory restarts just that one.

Two boot paths:
- **Imperative** (`boot.boot_from_files`): starts its own supervisor + @compiler,
  compiles grammars, starts domain servers inline. For standalone use.
- **Supervised** (`boot.supervised_boot_from_files`): compiles through an
  already-running named @compiler, starts domains through the garden. For embedding
  in a larger supervision tree.

### Packages

`@conversation` lives in this repo. It is the meta-grammar — it describes the
language's own type system. The `@conversation` actor reads this definition to
bootstrap.

Other packages live in the garden (`garden/public/@name/`). Each package that
declares `in @actor` becomes a spawned actor on the BEAM.

### main.conv

The entry point. Parsed by Rust into the Prism that crosses the FFI threshold.

## Building

```sh
# Everything (Rust lint + test + coverage + Gleam tests)
just check

# Rust only
nix develop -c cargo test
nix develop -c cargo llvm-cov --package conversation --fail-under-lines 100

# BEAM only
just beam-test

# Individual commands
nix develop -c cargo build                    # Rust parser
cd beam && gleam build                         # Gleam modules
cd beam && gleam test                          # Gleam tests
```

## License

systemic.engineering License v1.0
