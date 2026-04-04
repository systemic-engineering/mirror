# mirror/conversation split — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split the monolithic `mirror` crate into two: `mirror` (pure language) and `conversation` (runtime + actors + LSP + CLI). Rename `Domain` → `Mirror` as the core type.

**Architecture:** The current `/Users/alexwolf/dev/projects/conversation/` directory (crate name `mirror`) gets renamed to `/Users/alexwolf/dev/projects/mirror/`. Runtime modules are extracted into a fresh `/Users/alexwolf/dev/projects/conversation/` crate. conversation-lsp, conversation-bin, and conversation-beam merge into the new conversation crate.

**Tech Stack:** Rust, ractor, tokio, tower-lsp, eetf, Gleam/Erlang (BEAM side)

**Key files reference:**
- Current crate: `/Users/alexwolf/dev/projects/conversation/` (crate name: `mirror`, binary: `abyss`)
- Runtime trait + impl: `src/runtime.rs` (567 lines)
- Artifact store: `src/artifact.rs` (434 lines)
- Actor lifecycle: `src/actor/*.rs` (1542 lines)
- FFI/NIF bridge: `src/ffi.rs` (966 lines)
- Core type: `src/model.rs:237` — `pub struct Domain`
- conversation-lsp: `/Users/alexwolf/dev/projects/conversation-lsp/src/` (710 lines)
- conversation-bin: `/Users/alexwolf/dev/projects/conversation-bin/src/` (256 lines)
- conversation-beam: `/Users/alexwolf/dev/projects/conversation-beam/` (Gleam/Erlang)

**Test commands:**
- mirror: `nix develop -c cargo test --lib --test settle_test`
- conversation (after creation): `nix develop -c cargo test`
- Coverage: `nix develop -c cargo llvm-cov --fail-under-lines 100`

---

### Task 1: Rename directory and binary

Move the filesystem directory. Update the binary name from `abyss` to `mirror`.

**Files:**
- Modify: `/Users/alexwolf/dev/projects/conversation/Cargo.toml` → moves to `/Users/alexwolf/dev/projects/mirror/Cargo.toml`

- [ ] **Step 1: Rename the directory**

```bash
cd /Users/alexwolf/dev/projects
mv conversation mirror
```

- [ ] **Step 2: Update Cargo.toml binary name**

In `/Users/alexwolf/dev/projects/mirror/Cargo.toml`, change:

```toml
[[bin]]
name = "mirror"
path = "src/main.rs"
```

(was `name = "abyss"`)

- [ ] **Step 3: Update main.rs help text**

In `/Users/alexwolf/dev/projects/mirror/src/main.rs`, replace all `"abyss"` references in help/error strings with `"mirror"`. The eprintln help block at the top and the `settle_cmd` error messages.

- [ ] **Step 4: Verify it builds**

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo build
```

Expected: compiles with warnings only (no errors).

- [ ] **Step 5: Run tests**

```bash
nix develop -c cargo test --lib --test settle_test
```

Expected: 870 passed, 0 failed.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "rename: conversation/ → mirror/, binary abyss → mirror"
```

---

### Task 2: Rename Domain → Mirror

The core type rename. `pub struct Domain` becomes `pub struct Mirror`. Update all references within the mirror crate.

**Files:**
- Modify: `/Users/alexwolf/dev/projects/mirror/src/model.rs`
- Modify: `/Users/alexwolf/dev/projects/mirror/src/lib.rs`
- Modify: all files that `use crate::model::Domain` or `mirror::model::Domain`

- [ ] **Step 1: Rename the struct in model.rs**

In `/Users/alexwolf/dev/projects/mirror/src/model.rs`:

```rust
// line 237: was pub struct Domain
/// A settled, content-addressed, verified grammar.
/// What you look into that looks back.
pub struct Mirror {
    pub name: DomainName,
    pub types: Vec<TypeDef>,
    pub actions: Vec<Action>,
    pub lenses: Vec<Lens>,
    pub extends: Vec<DomainName>,
    pub calls: Vec<ActionCall>,
    pub properties: Properties,
}
```

Also rename:
- `DomainOid` → `MirrorOid`
- `DomainSpectrum` → keep (it describes the spectrum of a domain, which is a runtime concept — but it's used by `Verified` in check.rs, so keep it for now)
- `DomainComplexity` → keep (same reason)
- `impl Encode for Domain` → `impl Encode for Mirror`
- `impl ContentAddressed for Domain` → `impl ContentAddressed for Mirror`
- `Domain::from_grammar` → `Mirror::from_grammar`
- All `impl Domain` blocks → `impl Mirror`

- [ ] **Step 2: Update lib.rs re-export**

In `/Users/alexwolf/dev/projects/mirror/src/lib.rs`:

```rust
// was: pub use model::Domain;
pub use model::Mirror;
```

- [ ] **Step 3: Update all internal references**

Run a project-wide find-and-replace within `/Users/alexwolf/dev/projects/mirror/src/`:
- `use crate::model::Domain;` → `use crate::model::Mirror;`  (but NOT DomainName, DomainMessage, etc.)
- `Domain::from_grammar` → `Mirror::from_grammar`
- `Beam<Domain>` → `Beam<Mirror>`
- `-> Domain` → `-> Mirror` (in check.rs: `into_domain()` → `into_mirror()`)
- `domain: Domain` → `mirror: Mirror` in struct fields (in runtime.rs DomainActorState)
- `fn domain(` → context-dependent, check each

Key files to update:
- `src/runtime.rs` — `type Artifact = Domain` → `type Artifact = Mirror`, `DomainActorState { domain: Domain }` → `DomainActorState { mirror: Mirror }`
- `src/boot.rs` — `Beam<Domain>` → `Beam<Mirror>`
- `src/compile.rs` — references in tests
- `src/check.rs` — `into_domain()` → `into_mirror()`, return type
- `src/packages.rs` — `Domain` references
- `src/logic.rs` — `Domain` references
- `src/generate.rs` — `Domain` references
- `src/property.rs` — `Domain` references
- `src/spectral.rs` — `Domain` references
- `src/db.rs` — `Domain` references
- `src/main.rs` — `mirror::model::Domain` → `mirror::model::Mirror`

Do NOT rename `DomainName`, `DomainMessage`, `DomainActor` yet — those are runtime concepts that will move to conversation.

- [ ] **Step 4: Build and test**

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test --lib --test settle_test
```

Expected: 870 passed, 0 failed.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "rename: Domain → Mirror — the crystal type"
```

---

### Task 3: Split Runtime trait from RactorRuntime

Separate the trait definition (stays in mirror) from the ractor implementation (moves out). This is the clean cut.

**Files:**
- Modify: `/Users/alexwolf/dev/projects/mirror/src/runtime.rs` — keep only trait + types
- Create: `/Users/alexwolf/dev/projects/mirror/src/runtime_types.rs` — if needed for Value/Args/Response

- [ ] **Step 1: Extract the Runtime trait into a clean module**

Rewrite `/Users/alexwolf/dev/projects/mirror/src/runtime.rs` to contain ONLY:

```rust
//! Runtime trait — the compilation backend interface.
//!
//! Mirror defines the contract. Implementations live elsewhere.
//! - MetalRuntime: GPU kernels (in mirror, settled/cold path)
//! - RactorRuntime: ractor actors (in conversation, hot path)

use std::fmt;

use crate::check::Verified;
use crate::model::Mirror;

/// The compiler backend. Two operations:
/// - compile: Verified → Mirror (pure, storable)
/// - spawn: Mirror → Handle (side effect, ephemeral)
#[allow(async_fn_in_trait)]
pub trait Runtime: Send + Sync {
    type Actor;
    type Error: fmt::Display + Send;

    async fn compile(&self, verified: Verified) -> Result<prism::Beam<Mirror>, Self::Error>;
    async fn spawn(&self, mirror: &Mirror) -> Result<Self::Actor, Self::Error>;
}
```

- [ ] **Step 2: Move runtime types to their own module**

Create `/Users/alexwolf/dev/projects/mirror/src/dispatch.rs`:

```rust
//! Dispatch types — Value, Args, Response for action dispatch.
//! Used by both mirror (compile-time validation) and conversation (runtime dispatch).

use std::collections::BTreeMap;
use std::fmt;

use crate::model::ActionName;
use crate::Oid;

#[derive(Clone, Debug)]
pub enum Value {
    Text(String),
    Bytes(Vec<u8>),
    Oid(Oid),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

#[derive(Clone, Debug)]
pub enum Args {
    Empty,
    Single(Value),
    Named(BTreeMap<ActionName, Value>),
}

#[derive(Clone, Debug)]
pub enum Response {
    Ok(Value),
    Error(String),
}

#[derive(Debug)]
pub struct RuntimeError(pub String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
```

- [ ] **Step 3: Move InferenceSchedule into dispatch.rs or its own module**

Add to `/Users/alexwolf/dev/projects/mirror/src/dispatch.rs`:

```rust
use crate::model::DomainComplexity;
use crate::check::Verified;

pub enum InferenceSchedule {
    Immediate,
    Diffusion(coincidence::eigenvalues::Eigenvalues),
}

impl InferenceSchedule {
    pub fn from_verified(verified: &Verified) -> Self {
        match verified.complexity() {
            DomainComplexity::Trivial => InferenceSchedule::Immediate,
            DomainComplexity::Spectrum(spectrum) => {
                InferenceSchedule::Diffusion(spectrum.eigenvalues().clone())
            }
        }
    }

    pub fn temperature(&self, context_complexity: f64) -> f64 {
        match self {
            InferenceSchedule::Immediate => 0.0,
            InferenceSchedule::Diffusion(eigenvalues) => {
                let t = eigenvalues.diffusion_time(context_complexity);
                eigenvalues.temperature_at(t)
            }
        }
    }
}
```

- [ ] **Step 4: Save the ractor code to a holding file**

Save the RactorRuntime, DomainMessage, DomainActor, DomainActorState, and all ractor tests to `/Users/alexwolf/dev/projects/mirror/extracted/ractor_runtime.rs` for later use in conversation.

```bash
mkdir -p /Users/alexwolf/dev/projects/mirror/extracted
```

Copy lines 127-567 of the old runtime.rs (everything from `DomainMessage` through the tests) into `extracted/ractor_runtime.rs`.

- [ ] **Step 5: Update lib.rs**

In `/Users/alexwolf/dev/projects/mirror/src/lib.rs`, add:

```rust
pub mod dispatch;
```

Update re-exports:
```rust
pub use dispatch::{Args, InferenceSchedule, Response, RuntimeError, Value};
```

- [ ] **Step 6: Remove ractor and tokio from Cargo.toml**

In `/Users/alexwolf/dev/projects/mirror/Cargo.toml`:
- Remove `ractor = "0.15"`
- Remove `tokio = { version = "1", features = ["full"] }`
- Remove `futures = "0.3"`
- Remove `git2 = "0.19"`
- Remove `ssh-key = { ... }`
- Keep `eetf` (needed by compile.rs)

Also remove from `[dev-dependencies]`:
- Remove `tokio = { version = "1", features = ["full", "test-util"] }`

- [ ] **Step 7: Fix compilation errors**

The removal of ractor/tokio will cause errors in:
- `src/runtime.rs` — already cleaned up
- `src/actor/` — will be removed in next task
- `src/boot.rs` — uses `futures::join_all` — replace with sequential or remove async
- `src/main.rs` — may use tokio runtime

Fix each: remove async where not needed, or gate behind a feature flag.

- [ ] **Step 8: Build and test**

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test --lib --test settle_test
```

Expected: all mirror-only tests pass. Runtime tests will have been removed (they live in extracted/).

- [ ] **Step 9: Commit**

```bash
git add -A
git commit -m "split: extract Runtime trait, remove ractor/tokio from mirror"
```

---

### Task 4: Remove runtime modules from mirror

Extract actor/*, artifact.rs, ffi.rs from mirror. They move to conversation.

**Files:**
- Remove from mirror: `src/actor/` (entire directory)
- Remove from mirror: `src/artifact.rs`
- Remove from mirror: `src/ffi.rs`
- Modify: `src/lib.rs` — remove module declarations
- Save: `extracted/` — for conversation to pick up

- [ ] **Step 1: Move files to extracted/**

```bash
cd /Users/alexwolf/dev/projects/mirror
cp -r src/actor extracted/actor
cp src/artifact.rs extracted/artifact.rs
cp src/ffi.rs extracted/ffi.rs
```

- [ ] **Step 2: Remove from mirror src/**

```bash
rm -rf src/actor
rm src/artifact.rs
rm src/ffi.rs
```

- [ ] **Step 3: Update lib.rs**

Remove these lines from `/Users/alexwolf/dev/projects/mirror/src/lib.rs`:

```rust
// Remove:
pub mod actor;
// Remove (if artifact was declared):
pub mod artifact;  // not always present as pub mod
// Remove:
pub mod ffi;
```

- [ ] **Step 4: Remove the workspace member**

In `/Users/alexwolf/dev/projects/mirror/Cargo.toml`, change:

```toml
[workspace]
members = ["."]
```

(Remove `"beam/native/conversation_nif"` from members — it moves to conversation.)

- [ ] **Step 5: Fix all compilation errors**

Files that imported actor/artifact/ffi modules will break. For each:
- If it's `src/main.rs` referencing actor commands: remove the actor subcommand (it moves to conversation)
- If it's `src/lib.rs` re-exporting: remove the re-export
- If it's a test that depends on runtime: move the test to extracted/

- [ ] **Step 6: Remove db module if it depends on runtime**

Check `src/db.rs` — if it depends on ractor/tokio/artifact, move it to extracted/ too. It's feature-gated (`#[cfg(feature = "db")]`), so removing the feature from the default may be enough.

- [ ] **Step 7: Build and test**

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test --lib --test settle_test
```

Expected: all pure-language tests pass.

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "split: remove runtime/actor/artifact/ffi from mirror"
```

---

### Task 5: Create the conversation crate

Fresh Rust crate at `/Users/alexwolf/dev/projects/conversation/`.

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/Cargo.toml`
- Create: `/Users/alexwolf/dev/projects/conversation/src/lib.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/main.rs`

- [ ] **Step 1: Initialize the crate**

```bash
cd /Users/alexwolf/dev/projects
mkdir -p conversation/src
```

- [ ] **Step 2: Write Cargo.toml**

```toml
[package]
name = "conversation"
version = "0.1.0"
edition = "2021"
description = "The runtime. Actors enter conversation."

[[bin]]
name = "conversation"
path = "src/main.rs"

[dependencies]
mirror = { path = "../mirror" }
smelter = { path = "../smelter" }
prism = { path = "../prism" }
spectral-db = { path = "../spectral-db" }
fragmentation = { path = "../fragmentation" }
fragmentation-git = { path = "../fragmentation-git" }
coincidence = { path = "../coincidence" }
ractor = "0.15"
tokio = { version = "1", features = ["full"] }
futures = "0.3"
eetf = "0.11"
git2 = "0.19"
tower-lsp = "0.20"
serde_json = "1"
ssh-key = { version = "0.6", features = ["std", "ed25519", "crypto"] }

[dev-dependencies]
tempfile = "3"
tokio = { version = "1", features = ["full", "test-util"] }
```

- [ ] **Step 3: Write src/lib.rs**

```rust
//! conversation — the runtime. Actors enter conversation.
//!
//! Mirror produces Beam<Mirror>. Conversation spawns it as a Domain.

pub mod runtime;
pub mod artifact;
pub mod actor;
pub mod ffi;
pub mod lsp;
pub mod boot;

pub use runtime::{RactorRuntime, DomainMessage, DomainActor};
pub use artifact::{ArtifactStore, MemoryStore, GitStore};
```

- [ ] **Step 4: Write src/main.rs skeleton**

```rust
//! conversation — the runtime CLI.

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let has_settle = args.iter().any(|a| a == "--settle");
    let has_train = args.iter().any(|a| a == "--train");
    let positional: Vec<&String> = args.iter().skip(1)
        .filter(|a| !a.starts_with("--")).collect();

    match positional.first().map(|s| s.as_str()) {
        Some("abyss") => {
            eprintln!("conversation abyss: not yet wired");
            process::exit(1);
        }
        Some("lsp") => {
            eprintln!("conversation lsp: not yet wired");
            process::exit(1);
        }
        Some("actor") => {
            eprintln!("conversation actor: not yet wired");
            process::exit(1);
        }
        Some("shell") => {
            eprintln!("conversation shell: not yet wired");
            process::exit(1);
        }
        _ => {
            eprintln!("conversation — actors enter conversation");
            eprintln!();
            eprintln!("usage: conversation abyss settle <file>  settle + spawn");
            eprintln!("       conversation lsp                  LSP server");
            eprintln!("       conversation actor <cmd>          actor lifecycle");
            eprintln!("       conversation shell [path]         REPL");
            process::exit(1);
        }
    }
}
```

- [ ] **Step 5: Verify it builds**

```bash
cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo build
```

Expected: compiles (empty modules are fine for now).

- [ ] **Step 6: Commit**

```bash
git init
git add -A
git commit -m "init: conversation crate — the runtime"
```

---

### Task 6: Move extracted modules into conversation

Port the runtime, artifact, actor, and ffi modules from mirror's extracted/ into conversation.

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/src/runtime.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/artifact.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/actor/` (entire directory)
- Create: `/Users/alexwolf/dev/projects/conversation/src/ffi.rs`

- [ ] **Step 1: Copy extracted files**

```bash
cp /Users/alexwolf/dev/projects/mirror/extracted/ractor_runtime.rs \
   /Users/alexwolf/dev/projects/conversation/src/runtime.rs
cp /Users/alexwolf/dev/projects/mirror/extracted/artifact.rs \
   /Users/alexwolf/dev/projects/conversation/src/artifact.rs
cp -r /Users/alexwolf/dev/projects/mirror/extracted/actor \
   /Users/alexwolf/dev/projects/conversation/src/actor
cp /Users/alexwolf/dev/projects/mirror/extracted/ffi.rs \
   /Users/alexwolf/dev/projects/conversation/src/ffi.rs
```

- [ ] **Step 2: Update imports**

In every copied file, change:
- `use crate::model::Domain` → `use mirror::model::Mirror` (or `use mirror::Mirror`)
- `use crate::model::DomainName` → `use mirror::model::DomainName`
- `use crate::model::ActionName` → `use mirror::model::ActionName`
- `use crate::check::Verified` → `use mirror::check::Verified`
- `use crate::Oid` → `use mirror::Oid`
- `use crate::` → `use mirror::` for all mirror types
- Keep `use crate::` for conversation-local types (runtime, artifact)

In `runtime.rs` specifically:
- `type Artifact = Domain` → `type Artifact = Mirror`
- `DomainActorState { domain: Domain }` → `DomainActorState { mirror: Mirror }`
- `state.domain.actions` → `state.mirror.actions`
- `state.domain.name` → `state.mirror.name`

- [ ] **Step 3: Build**

```bash
cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo build
```

Fix any remaining import errors. The key pattern: anything from the language (parse, resolve, compile, model, kernel) comes from `mirror::`. Anything from the runtime (ractor, actor dispatch) is local.

- [ ] **Step 4: Run tests**

```bash
nix develop -c cargo test
```

Expected: runtime tests pass (compile + spawn cycle).

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: port runtime/artifact/actor/ffi from mirror"
```

---

### Task 7: Merge conversation-lsp

Port the LSP server into conversation.

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/src/lsp/mod.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/lsp/server.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/lsp/analysis.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/lsp/position.rs`

- [ ] **Step 1: Create lsp module directory**

```bash
mkdir -p /Users/alexwolf/dev/projects/conversation/src/lsp
```

- [ ] **Step 2: Copy files**

```bash
cp /Users/alexwolf/dev/projects/conversation-lsp/src/server.rs \
   /Users/alexwolf/dev/projects/conversation/src/lsp/server.rs
cp /Users/alexwolf/dev/projects/conversation-lsp/src/analysis.rs \
   /Users/alexwolf/dev/projects/conversation/src/lsp/analysis.rs
cp /Users/alexwolf/dev/projects/conversation-lsp/src/position.rs \
   /Users/alexwolf/dev/projects/conversation/src/lsp/position.rs
```

- [ ] **Step 3: Write mod.rs**

Create `/Users/alexwolf/dev/projects/conversation/src/lsp/mod.rs`:

```rust
//! LSP server for conversation grammars.

pub mod server;
pub mod analysis;
pub mod position;
```

- [ ] **Step 4: Update imports**

In all lsp/*.rs files, change:
- `use conversation::` → `use mirror::` (for language types: Parse, Vector, Namespace, Domain→Mirror)
- Add `use mirror::` prefix for all mirror types

- [ ] **Step 5: Wire the `lsp` subcommand in main.rs**

Pull the LSP startup logic from conversation-lsp's main.rs into conversation's `Some("lsp")` branch.

- [ ] **Step 6: Build and test**

```bash
cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo build
```

Expected: compiles. LSP can be tested manually with an editor.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: merge conversation-lsp into conversation/src/lsp/"
```

---

### Task 8: Merge conversation-bin

Port first_boot and launch orchestration.

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/src/first_boot.rs`
- Create: `/Users/alexwolf/dev/projects/conversation/src/launch.rs`
- Modify: `/Users/alexwolf/dev/projects/conversation/src/main.rs`

- [ ] **Step 1: Copy files**

```bash
cp /Users/alexwolf/dev/projects/conversation-bin/src/first_boot.rs \
   /Users/alexwolf/dev/projects/conversation/src/first_boot.rs
cp /Users/alexwolf/dev/projects/conversation-bin/src/launch.rs \
   /Users/alexwolf/dev/projects/conversation/src/launch.rs
```

- [ ] **Step 2: Update imports**

Change `use conversation::` → `use mirror::` and `use crate::` as appropriate.

- [ ] **Step 3: Wire into main.rs**

Add subcommands from conversation-bin's main.rs (compile, render, actor, join, lens, first_boot, launch) into conversation's main.rs.

- [ ] **Step 4: Build and test**

```bash
cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo build
```

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: merge conversation-bin into conversation"
```

---

### Task 9: Merge conversation-beam

Port the BEAM/Gleam runtime.

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/beam/` (entire directory)

- [ ] **Step 1: Copy the BEAM directory**

```bash
cp -r /Users/alexwolf/dev/projects/conversation-beam/src \
   /Users/alexwolf/dev/projects/conversation/beam/src
cp /Users/alexwolf/dev/projects/conversation-beam/gleam.toml \
   /Users/alexwolf/dev/projects/conversation/beam/gleam.toml
```

Also copy test/, bin/, and any other necessary files.

- [ ] **Step 2: Move conversation_nif**

```bash
cp -r /Users/alexwolf/dev/projects/mirror/beam/native/conversation_nif \
   /Users/alexwolf/dev/projects/conversation/beam/native/conversation_nif
```

- [ ] **Step 3: Update conversation_nif's Cargo.toml**

Change the mirror dependency path:

```toml
[dependencies]
mirror = { path = "../../mirror", features = ["spectral"] }
```

(was `{ path = "../../../conversation" }` or similar)

- [ ] **Step 4: Update conversation_nif imports**

In `beam/native/conversation_nif/src/lib.rs`, all imports should already use `mirror::` prefix. Verify:
- `use mirror::ffi::` → these functions moved to conversation. The NIF either calls mirror directly for parse/compile, or needs to import from the parent conversation crate.

Decision: The NIF calls `mirror::parse` and `mirror::compile` directly (language operations). For runtime operations, it calls conversation. Update paths accordingly.

- [ ] **Step 5: Add workspace member**

In `/Users/alexwolf/dev/projects/conversation/Cargo.toml`:

```toml
[workspace]
members = [".", "beam/native/conversation_nif"]
```

- [ ] **Step 6: Build**

```bash
cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo build
```

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: merge conversation-beam + conversation_nif into conversation"
```

---

### Task 10: Clean up mirror's extracted/ and old beam/

Remove temporary files and old BEAM workspace member from mirror.

**Files:**
- Remove: `/Users/alexwolf/dev/projects/mirror/extracted/`
- Remove: `/Users/alexwolf/dev/projects/mirror/beam/native/` (NIF moved to conversation)

- [ ] **Step 1: Remove extracted directory**

```bash
rm -rf /Users/alexwolf/dev/projects/mirror/extracted
```

- [ ] **Step 2: Clean up beam directory**

If mirror's `beam/` directory only contained the NIF workspace member, remove it:

```bash
rm -rf /Users/alexwolf/dev/projects/mirror/beam/native
```

Keep `beam/src/` if it contains Gleam source that mirror uses (like prism_beam.gleam). Otherwise remove.

- [ ] **Step 3: Final test — both crates**

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test --lib --test settle_test

cd /Users/alexwolf/dev/projects/conversation
nix develop -c cargo test
```

Expected: both pass independently.

- [ ] **Step 4: Commit mirror**

```bash
cd /Users/alexwolf/dev/projects/mirror
git add -A
git commit -m "clean: remove extracted/ and old beam/native"
```

- [ ] **Step 5: Commit conversation**

```bash
cd /Users/alexwolf/dev/projects/conversation
git add -A
git commit -m "clean: final structure"
```

---

### Task 11: Update downstream references

Update all projects that referenced the old conversation crate path.

**Files:**
- Modify: Any Cargo.toml that has `conversation = { path = "../conversation" }`
- Modify: spectral-db, glue-pub, conversation-admin, etc.

- [ ] **Step 1: Find all path references**

```bash
grep -rn 'path.*conversation' /Users/alexwolf/dev/projects/*/Cargo.toml 2>/dev/null
```

- [ ] **Step 2: Update each reference**

For each Cargo.toml found:
- If it depended on the language (parse, resolve, compile): change to `mirror = { path = "../mirror" }`
- If it depended on the runtime (actors, FFI): change to `conversation = { path = "../conversation" }`
- If it depended on both: add both dependencies

- [ ] **Step 3: Update Rust import paths**

In each dependent crate's source files:
- `use conversation::` → `use mirror::` for language types
- Add `use conversation::` for runtime types if needed

- [ ] **Step 4: Build each dependent**

```bash
cd /Users/alexwolf/dev/projects/spectral-db && nix develop -c cargo build
cd /Users/alexwolf/dev/projects/conversation-lsp  # should be dead — verify
```

- [ ] **Step 5: Commit each affected project**

---

### Task 12: Archive dead projects

Mark conversation-lsp, conversation-bin, and conversation-beam as archived.

- [ ] **Step 1: Add ARCHIVED.md to each**

Create `ARCHIVED.md` in each dead project:

```markdown
# Archived

Merged into `/Users/alexwolf/dev/projects/conversation/` on 2026-04-04.
See docs/superpowers/specs/2026-04-04-mirror-conversation-split-design.md.
```

- [ ] **Step 2: Commit**

```bash
for d in conversation-lsp conversation-bin conversation-beam; do
  cd /Users/alexwolf/dev/projects/$d
  echo "# Archived\nMerged into conversation/ on 2026-04-04." > ARCHIVED.md
  git add ARCHIVED.md && git commit -m "archived: merged into conversation"
done
```
