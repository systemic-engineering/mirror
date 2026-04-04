# mirror/conversation split

**Date:** 2026-04-04
**Status:** Approved design

mirror is the language. conversation is the runtime.

## Motivation

The conversation crate contains both the language kernel (parse, resolve, compile, emit, classify, settle) and the runtime machinery (ractor actors, BEAM FFI, LSP server, CLI). These are different concerns with different dependency profiles. The language is pure transformation. The runtime is side effects.

Splitting them:
- mirror has zero async/actor/network dependencies
- conversation pulls in ractor, tokio, tower-lsp, eetf — heavy, but contained
- Each can evolve independently
- The BEAM side (conversation-beam) merges into conversation, giving one crate for all runtime concerns

## Architecture

### mirror — the language

Pure transformation. No actors, no BEAM, no network.

**Location:** `/Users/alexwolf/dev/projects/mirror/` (renamed from `conversation/`)

**Modules:**
- kernel (Oid, Trace, Vector, ContentAddressed, domain_oid!)
- ast (AstNode, Span, Kind)
- parse (Parse trait, KEYWORD_TABLE, frozen bootstrap parser)
- resolve (TypeRegistry, Namespace, grammar type checking)
- compile (AST → EAF, emit_actor_module)
- emit (AST → .mirror source, round-trip projection)
- check (Verified type)
- model (Domain, Action, TypeDef, Lens, Properties, DomainName)
- domain (conversation::Kind, filesystem)
- classifier (Optic enum, Weights, train, fine_tune, classify)
- abyss (AbyssConfig, PrismLoop, Termination, settle_loop)
- bounded (BoundedMemoryStore, Pressure, Shed)
- boot (BootSequence::from_dir, parallel layer compilation)
- spectral (Laplacian, GrammarProjection, TypeGraph)
- property (check_builtin)
- logic, filter, generate, packages, prism

**Core type:**
```rust
/// A settled, content-addressed, verified grammar.
/// What you look into that looks back.
pub struct Mirror { /* types, actions, properties, oid */ }
```

**Runtime trait (defined here, implemented elsewhere):**
```rust
pub trait Runtime: Send + Sync {
    type Actor;
    type Error;
    fn compile(&self, verified: Verified) -> Result<Beam<Mirror>, Self::Error>;
    fn spawn(&self, mirror: &Mirror) -> Result<Self::Actor, Self::Error>;
}
```

`compile` produces `Beam<Mirror>`. `spawn` takes a Mirror and makes it live. Mirror is the artifact. Domain is what conversation calls a spawned Mirror.

**MetalRuntime** lives here — GPU kernels, settled/cold path. Implements `Runtime`.

**Binary:** `mirror`
- `mirror fmt [--settle] [--train] <file>` — format, settle, train
- `mirror abyss settle <file|dir>` — settle loop
- `mirror abyss train` — retrain classifier from fixtures
- `mirror test <file>` — run tests
- `mirror resolve` — resolve tension → resolved fixtures

**Dependencies:** prism, fragmentation, fragmentation-git, coincidence, sha2, hex, age, base64, serde_json

**Drops:** ractor, tokio, git2, ssh-key, futures, tower-lsp
**Keeps:** eetf (needed by compile.rs for EAF emission — language output, not runtime)

### conversation — the runtime

Actors enter conversation. Two runtimes (Rust + BEAM) under one roof.

**Location:** `/Users/alexwolf/dev/projects/conversation/` (fresh crate)

**Rust side (`src/`):**

Moved from mirror:
- runtime (RactorRuntime, DomainMessage, DomainActor, InferenceSchedule, Value, Args, Response)
- artifact (ArtifactStore trait, MemoryStore, GitStore)
- actor/* (init, mount, observe, status, emit_nix)
- ffi (conv_parse, conv_compile_grammar, NIF exports)

Merged from conversation-lsp:
- lsp/server (LSP protocol impl)
- lsp/analysis (parser → LSP bridge)
- lsp/position (LSP position/range conversion)

Merged from conversation-bin:
- first_boot, launch (orchestration)

New:
- boot/spawn (takes mirror's settled crystals, spawns actors)

**BEAM side (`beam/`):**

Merged from conversation-beam:
- conversation_actor.erl (actor protocol, subscribe/notify)
- conversation_beam_app.erl, _sup.erl (OTP application + supervisor)
- conversation_beam_main.erl (entry point)
- conversation_cli.erl (BEAM CLI)
- conversation_cluster.erl (clustering)
- conversation_garden.erl (garden/package loading)
- conversation_graphql.erl (GraphQL executor)
- conversation_mcp.erl (Model Context Protocol)
- conversation_store.erl (artifact store)
- conversation_agent_protocol.erl (agent protocol)
- conversation_test_graph.erl (test runner)

Workspace member:
- conversation_nif (Rustler cdylib, moved from mirror's beam/native/)

**Binary:** `conversation`
- `conversation abyss settle <file|dir>` — settle + spawn actors
- `conversation lsp` — LSP server
- `conversation actor <observe|init|mount|unmount|status>` — actor lifecycle
- `conversation shell [path]` — REPL
- `conversation fmt [--settle] [--train] <file>` — delegates to mirror + spawns
- `conversation test <file>` — run tests with runtime
- `conversation db` — database operations

**Dependencies:** mirror, smelter, spectral-db, ractor, tokio, eetf, git2, tower-lsp, futures, ssh-key

**BEAM dependencies:** gleam_stdlib, gleam_erlang, gen_mcp

## What dies

| Project | Disposition |
|---------|------------|
| `conversation-lsp/` | Merged into `conversation/src/lsp/` |
| `conversation-bin/` | Merged into `conversation/src/main.rs` |
| `conversation-beam/` | Merged into `conversation/beam/` |

## What stays independent

| Project | Identity |
|---------|----------|
| `conversation-admin/` | Garden package `@admin` |
| `conversation-ai/` | Garden package `@ai` |
| `conversation-ca/` | Garden package `@ca` |
| `conversation-ci/` | Garden package `@ci` |

## File extensions: .mirror and .conv are different grammars

Not a rename. A lens.

- `.mirror` files are parsed by the mirror language — crystal keywords (fold, prism, traversal, lens, iso), type definitions, boot vocabulary. The language kernel.
- `.conv` files are parsed by conversation — runtime grammar, actor definitions, domain orchestration. The runtime layer.

Mirror's parser handles `.mirror`. Conversation's parser extends it to handle `.conv`. The lens between them is the compilation boundary: mirror produces `Beam<Mirror>`, conversation spawns it as a Domain.

This means the current `.conv` files split into two populations:
- Boot files, fixtures, crystal grammars → `.mirror`
- Actor grammars, domain servers, runtime config → `.conv`

The garden grammars are `.conv` — they're conversation-level, defining domains that get spawned. The boot vocabulary is `.mirror` — it defines the language itself.

## Not in this spec

- Garden grammar updates — downstream of the grammar split
- glue-pub updates — downstream of the grammar split

## Migration sequence

1. Rename `/Users/alexwolf/dev/projects/conversation/` → `/Users/alexwolf/dev/projects/mirror/`
2. Update Cargo.toml: keep `name = "mirror"`, binary becomes `mirror` (was `abyss`)
3. Remove ractor/tokio/eetf/tower-lsp from mirror's dependencies
4. Extract runtime.rs, artifact.rs, actor/*, ffi.rs into holding area
5. Create fresh `/Users/alexwolf/dev/projects/conversation/` Rust crate
6. Move extracted modules into conversation/src/
7. Copy conversation-beam/ contents into conversation/beam/
8. Move conversation_nif workspace member into conversation/beam/native/
9. Merge conversation-lsp/src/ into conversation/src/lsp/
10. Merge conversation-bin/src/ into conversation/src/ (first_boot, launch)
11. Wire conversation's Cargo.toml: depend on mirror + smelter + spectral-db + ractor + tokio + etc.
12. Build conversation binary with subcommands
13. Update all downstream path references
14. Delete dead projects (conversation-lsp/, conversation-bin/, conversation-beam/)
15. Tests pass in both crates independently
