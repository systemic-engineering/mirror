# conversation

Stories over trees.

A typed transformation pipeline language. `.conv` files describe desired state.
The Rust parser produces a `Prism<AstNode>`. The BEAM runtime converges toward it.

## Architecture

```
main.conv → [Rust: parse] → Prism → [Fortran: matmul] → Matrix → [git commit] → OID → [BEAM]
```

**Rust** (`src/`): Parses `.conv` source into a `Prism<AstNode>`, extracts the type
vocabulary, encodes it as an n×n projection matrix via Fortran, and commits the matrix
to `.git/refs/conversation/<branch>` as `conversation@systemic.engineering`. The OID
crosses the FFI threshold to the BEAM. The branch is a Lens in the Prism pointing to HEAD.

**Fortran** (`beam/native/prism.f90`): Basic linear algebra on the projection matrix —
preview, review, modify, compose. Linked statically into the Rust binary via `build.rs`.
The matrix never leaves its native representation — no ETF, no JSON, no lossy intermediate.

**BEAM** (`beam/`): Gleam. Receives the OID. Loads the committed matrix. Owns resolution,
compilation, package discovery, and actor lifecycle. The `@conversation` actor extends
`@compiler`.

### Rust modules

| Module | Purpose |
|--------|---------|
| `kernel.rs` | Oid, Trace, Vector, ContentAddressed — core algebra |
| `ast.rs` | AstNode, Kind enum — the AST types |
| `parse.rs` | `.conv` source → `Prism<AstNode>` |
| `prism.rs` | Content-addressed tree structure |
| `matrix.rs` | Fortran FFI, vocabulary extraction, Prism→Matrix encoding |
| `ffi.rs` | `conv_parse` — the FFI entry point, matrix commit pipeline |
| `domain/` | Domain implementations (filesystem) |
| `filter.rs` | Tree filtering |
| `generate.rs` | Tree generation |

Modules being superseded by BEAM (do not extend):
`resolve.rs`, `packages.rs`, `compile.rs`, `property.rs`

### BEAM modules (`beam/src/conversation/`)

| Module | Purpose |
|--------|---------|
| `oid.gleam` | Content-addressed identity (SHA-256) |
| `key.gleam` | Ed25519 actor identity |
| `trace.gleam` | Witnessed, signed records |
| `ref.gleam` | Scoped content-addressed references |
| `prism.gleam` | Projection matrix (Fortran NIF) |
| `nif.gleam` | Bridge to Rust parser |
| `compiler.gleam` | @compiler actor |
| `protocol.gleam` | Spec types — desired BEAM state |
| `runtime.gleam` | Convergence engine — delta computation |

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
# Rust parser
nix develop -c cargo build

# BEAM runtime
cd beam && gleam build

# Tests
nix develop -c cargo test                                    # Rust
nix develop -c cargo llvm-cov --lib --fail-under-lines 100   # coverage
cd beam && gleam test                                         # Gleam
```

## License

systemic.engineering License v1.0
