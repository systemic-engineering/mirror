# Mirror LSP: .shatter-Powered Language Server

**Author:** Mara
**Date:** 2026-04-14
**Status:** Implementation plan. Five phases. Each independently shippable.

---

## Principle

The LSP does not compute. It reads. The compiler writes `.shatter` artifacts to
`.git/mirror/`. The LSP reads them. The `.shatter` IS the LSP response.

```
save .mirror file
  -> mirror craft (background)
  -> write .shatter to .git/mirror/ store
  -> LSP reads .shatter frontmatter
  -> update editor (gutter colors, diagnostics, completions)
```

---

## Current State

| Component | File | Status |
|-----------|------|--------|
| `MirrorGitStore` | `src/git_store.rs` | Stores `Fractal<String>` by OID in `.git/mirror/`. `store_crystal`, `get_crystal`, refs. Working. |
| `MirrorLoss` | `src/loss.rs` | Four-fold loss (parse, resolution, property, emit) + convergence + holonomy. Working. |
| `CompiledShatter` | `src/mirror_runtime.rs` | `{ form: Form, fragment: MirrorFragment }`. Has `crystal()` -> OID. Working. |
| `compile_source` | `src/mirror_runtime.rs` | Returns `Imperfect<CompiledShatter, MirrorRuntimeError, MirrorLoss>`. Working. |
| `emit_rust` | `src/emit_rust.rs` | `CompiledShatter` -> Rust source. Working. |
| `src/lsp/` | `src/lsp/mod.rs` | Grammar *generation* from tree-sitter. NOT an LSP server. `mirror lsp learn @code/python`. |
| CLI `lsp` command | `src/cli.rs:1051` | Dispatches only to `learn` subcommand. No `serve` / bare `mirror lsp`. |
| `DeclKind` | `src/declaration.rs` | 23 variants. Completion source. |
| `OpticOp` | `src/declaration.rs` | 10 variants. Completion source. |
| `Form.parent_ref` | `src/mirror_runtime.rs:128` | Already exists on `Form`. Parser already handles `grammar @name < @parent`. |

**What does not exist:**
- `.shatter` serialization format (frontmatter + body)
- Luminosity enum (`light` / `dimmed` / `dark`)
- Background compilation triggered by file save
- LSP server (JSON-RPC, stdio transport)
- `mirror lsp` (bare) starting a server

---

## Phase 1: .shatter Serialization

**Goal:** Serialize `CompiledShatter` + `MirrorLoss` into the `.shatter` frontmatter+body format. Deserialize back. Round-trip.

**New file:** `src/shatter_format.rs`

### Task 1.1: Luminosity enum (2 min)

Red: write a test in `src/shatter_format.rs`.

```rust
#[test]
fn luminosity_from_loss_zero() {
    let loss = MirrorLoss::zero();
    assert_eq!(Luminosity::from_loss(&loss), Luminosity::Light);
}

#[test]
fn luminosity_from_loss_partial() {
    let mut loss = MirrorLoss::zero();
    loss.parse.unrecognized.push(UnrecognizedDecl {
        keyword: "widget".into(), line: 1, content: "foo".into(),
    });
    assert_eq!(Luminosity::from_loss(&loss), Luminosity::Dimmed);
}

#[test]
fn luminosity_from_loss_failure() {
    let loss = MirrorLoss::total();
    assert_eq!(Luminosity::from_loss(&loss), Luminosity::Dark);
}
```

Green: implement.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Luminosity {
    Light,   // zero loss, crystal settled
    Dimmed,  // partial, loss measured
    Dark,    // failure, error carried
}

impl Luminosity {
    pub fn from_loss(loss: &MirrorLoss) -> Self {
        if loss.convergence == Convergence::BudgetExhausted {
            Luminosity::Dark
        } else if loss.is_zero() {
            Luminosity::Light
        } else {
            Luminosity::Dimmed
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Luminosity::Light => "light",
            Luminosity::Dimmed => "dimmed",
            Luminosity::Dark => "dark",
        }
    }
}
```

### Task 1.2: ShatterMeta struct (2 min)

```rust
pub struct ShatterMeta {
    pub oid: String,
    pub luminosity: Luminosity,
    pub holonomy: f64,
    pub loss: ShatterLossBreakdown,
    pub beam: ShatterBeamInfo,
}

pub struct ShatterLossBreakdown {
    pub parse: f64,
    pub resolution: f64,
    pub properties: f64,
    pub emit: f64,
}

pub struct ShatterBeamInfo {
    pub compiler: String,
    pub prism: String,
    pub target: String,
}
```

Red: test that `ShatterMeta::from_compiled` produces correct fields.

```rust
#[test]
fn shatter_meta_from_compiled() {
    let runtime = MirrorRuntime::new();
    let compiled: Result<CompiledShatter, _> =
        runtime.compile_source("type color = red | blue").into();
    let compiled = compiled.unwrap();
    let loss = MirrorLoss::zero();
    let meta = ShatterMeta::from_compiled(&compiled, &loss);
    assert_eq!(meta.luminosity, Luminosity::Light);
    assert_eq!(meta.holonomy, 0.0);
    assert_eq!(meta.beam.prism, "shatter");
}
```

Green: implement `ShatterMeta::from_compiled(compiled: &CompiledShatter, loss: &MirrorLoss) -> Self`.

### Task 1.3: Serialize to frontmatter (3 min)

Red:

```rust
#[test]
fn emit_shatter_frontmatter() {
    let meta = ShatterMeta {
        oid: "a3f8c2d1".into(),
        luminosity: Luminosity::Light,
        holonomy: 0.0,
        loss: ShatterLossBreakdown { parse: 0.0, resolution: 0.0, properties: 0.0, emit: 0.0 },
        beam: ShatterBeamInfo {
            compiler: "mirror-v0.1".into(),
            prism: "shatter".into(),
            target: "rust".into(),
        },
    };
    let body = "type color = red | blue\n";
    let output = emit_shatter_with_frontmatter(&meta, body);
    assert!(output.starts_with("---\n"));
    assert!(output.contains("oid: a3f8c2d1"));
    assert!(output.contains("luminosity: light"));
    assert!(output.contains("holonomy: 0"));
    assert!(output.ends_with("type color = red | blue\n"));
}
```

Green: implement `emit_shatter_with_frontmatter(meta: &ShatterMeta, body: &str) -> String`.

The function writes YAML-like frontmatter between `---` delimiters, then the body.
No dependency on a YAML crate. The format is simple enough to emit manually:

```rust
pub fn emit_shatter_with_frontmatter(meta: &ShatterMeta, body: &str) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&format!("oid: {}\n", meta.oid));
    out.push_str(&format!("luminosity: {}\n", meta.luminosity.as_str()));
    out.push_str(&format!("holonomy: {}\n", meta.holonomy));
    out.push_str("loss:\n");
    out.push_str(&format!("  parse: {}\n", meta.loss.parse));
    out.push_str(&format!("  resolution: {}\n", meta.loss.resolution));
    out.push_str(&format!("  properties: {}\n", meta.loss.properties));
    out.push_str(&format!("  emit: {}\n", meta.loss.emit));
    out.push_str("beam:\n");
    out.push_str(&format!("  compiler: {}\n", meta.beam.compiler));
    out.push_str(&format!("  prism: {}\n", meta.beam.prism));
    out.push_str(&format!("  target: {}\n", meta.beam.target));
    out.push_str("---\n\n");
    out.push_str(body);
    out
}
```

### Task 1.4: Parse frontmatter (3 min)

Red:

```rust
#[test]
fn parse_shatter_frontmatter_roundtrip() {
    let meta = ShatterMeta { /* ... same as above ... */ };
    let body = "type color = red | blue\n";
    let serialized = emit_shatter_with_frontmatter(&meta, body);
    let (parsed_meta, parsed_body) = parse_shatter_frontmatter(&serialized).unwrap();
    assert_eq!(parsed_meta.oid, "a3f8c2d1");
    assert_eq!(parsed_meta.luminosity, Luminosity::Light);
    assert_eq!(parsed_body, body);
}
```

Green: implement `parse_shatter_frontmatter(source: &str) -> Result<(ShatterMeta, &str), String>`.

Parse rules:
1. Source must start with `---\n`
2. Find second `---\n` — everything between is frontmatter
3. Parse key-value lines. Indented lines under `loss:` and `beam:` are sub-keys.
4. Everything after second `---\n` (plus optional blank line) is the body.

No serde. No YAML crate. Line-by-line parsing. The format is constrained enough.

### Task 1.5: Store/retrieve .shatter in MirrorGitStore (3 min)

Red:

```rust
#[test]
fn store_and_retrieve_shatter() {
    let dir = tempfile::tempdir().unwrap();
    git2::Repository::init(dir.path()).unwrap();
    let store = MirrorGitStore::open(dir.path()).unwrap();

    let meta = ShatterMeta { /* ... */ };
    let body = "type color = red | blue\n";
    let shatter_content = emit_shatter_with_frontmatter(&meta, body);

    let crystal = fragmentation::encoding::encode(&shatter_content);
    let oid = fragmentation::fragment::content_oid(&crystal);
    store.store_crystal(&oid, crystal, shatter_content.len());

    let got = store.get_crystal(&oid).unwrap();
    let decoded = fragmentation::encoding::decode::<String>(&got);
    let (parsed_meta, parsed_body) = parse_shatter_frontmatter(&decoded).unwrap();
    assert_eq!(parsed_meta.oid, meta.oid);
    assert_eq!(parsed_body, body);
}
```

Green: this test exercises existing `MirrorGitStore` APIs with `.shatter` content.
No new store methods needed. The `.shatter` is just a `String` stored as a `Fractal<String>`.

### Task 1.6: Wire into module tree (2 min)

Add `pub mod shatter_format;` to `src/lib.rs`.

Run: `nix develop -c cargo test -p mirror shatter_format`

Verify: all 5+ tests pass. Coverage check: `nix develop -c cargo llvm-cov --workspace --fail-under-lines 100`.

**Phase 1 deliverable:** `.shatter` format exists. Serialize, deserialize, round-trip, store, retrieve. No behavioral changes to the CLI.

---

## Phase 2: Background Compilation (craft -> store -> notify)

**Goal:** When a `.mirror` file is compiled, produce a `.shatter` and store it in `.git/mirror/`.

**Files changed:**
- `src/mirror_runtime.rs` — new method `compile_to_shatter`
- `src/cli.rs` — update `cmd_compile` to write `.shatter` to store

### Task 2.1: compile_to_shatter method (3 min)

Red:

```rust
#[test]
fn compile_to_shatter_produces_stored_artifact() {
    let dir = tempfile::tempdir().unwrap();
    git2::Repository::init(dir.path()).unwrap();
    let store = MirrorGitStore::open(dir.path()).unwrap();
    let runtime = MirrorRuntime::new();

    let source = "type color = red | blue";
    let result = runtime.compile_to_shatter(source, &store);
    assert!(result.is_ok());

    let (meta, _body) = result.unwrap();
    assert_eq!(meta.luminosity, Luminosity::Light);

    // Verify it's in the store
    let crystal = store.get_crystal(&meta.oid);
    assert!(crystal.is_some());
}
```

Green: implement on `MirrorRuntime`:

```rust
pub fn compile_to_shatter(
    &self,
    source: &str,
    store: &MirrorGitStore,
) -> Imperfect<(ShatterMeta, String), MirrorRuntimeError, MirrorLoss> {
    let result = self.compile_source(source);
    let loss = result.loss().clone();
    result.map(|compiled| {
        let body = source.to_string();
        let meta = ShatterMeta::from_compiled(&compiled, &loss);
        let shatter_content = emit_shatter_with_frontmatter(&meta, &body);
        let crystal = fragmentation::encoding::encode(&shatter_content);
        store.store_crystal(&meta.oid, crystal, shatter_content.len());
        (meta, body)
    })
}
```

### Task 2.2: File-path to OID index (3 min)

The LSP needs to look up "what `.shatter` corresponds to this `.mirror` file?"
Add a ref in the store: `files/<path-hash> -> <shatter-oid>`.

Red:

```rust
#[test]
fn store_file_ref_and_lookup() {
    let dir = tempfile::tempdir().unwrap();
    git2::Repository::init(dir.path()).unwrap();
    let store = MirrorGitStore::open(dir.path()).unwrap();

    store.set_file_ref("src/main.mirror", "abc123").unwrap();
    assert_eq!(store.get_file_ref("src/main.mirror").as_deref(), Some("abc123"));
}
```

Green: add to `MirrorGitStore`:

```rust
pub fn set_file_ref(&self, path: &str, oid: &str) -> Result<(), Error> {
    let key = format!("files/{}", hex_hash(path));
    self.inner.set_ref(&key, oid)
}

pub fn get_file_ref(&self, path: &str) -> Option<String> {
    let key = format!("files/{}", hex_hash(path));
    self.inner.get_ref(&key)
}

fn hex_hash(path: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hex::encode(&hasher.finalize()[..8])
}
```

### Task 2.3: Update cmd_compile to write .shatter (3 min)

Red:

```rust
#[test]
fn cmd_compile_writes_shatter_to_store() {
    let dir = tempfile::tempdir().unwrap();
    git2::Repository::init(dir.path()).unwrap();
    let mirror_file = dir.path().join("test.mirror");
    std::fs::write(&mirror_file, "type color = red | blue").unwrap();

    let cli = Cli::open_in(dir.path(), None).unwrap();
    let result = cli.dispatch("compile", &[mirror_file.to_str().unwrap().to_string()]);
    assert!(result.is_ok());

    let store = MirrorGitStore::open(dir.path()).unwrap();
    let file_ref = store.get_file_ref(mirror_file.to_str().unwrap());
    assert!(file_ref.is_some(), "compile should write file ref to store");
}
```

Green: in `cmd_compile`, after successful compilation:

1. Try to open `MirrorGitStore` at the repo root (discover via `git2::Repository::discover`)
2. If store opens: serialize `.shatter`, store crystal, set file ref
3. If not a git repo: skip (no store, no `.shatter` — CLI still works)

This is additive. The existing `cmd_compile` output does not change.

### Task 2.4: Notification channel stub (2 min)

For Phase 3, the LSP needs to know when a `.shatter` was updated. For now,
define the notification type without wiring it.

```rust
/// Notification that a .shatter was updated in the store.
pub struct ShatterNotification {
    pub file_path: String,
    pub oid: String,
    pub luminosity: Luminosity,
}
```

No tests needed — this is a data type. Phase 3 will consume it.

**Phase 2 deliverable:** `mirror compile foo.mirror` writes a `.shatter` to `.git/mirror/` and records a file-path -> OID ref. Existing CLI behavior unchanged.

---

## Phase 3: LSP Server

**Goal:** A minimal LSP server that reads `.shatter` artifacts and serves diagnostics.

**New file:** `src/lsp/server.rs`
**Dependency:** `tower-lsp` (add to `Cargo.toml`)

### Task 3.1: Add tower-lsp dependency (2 min)

In `Cargo.toml`, under `[dependencies]`:

```toml
tower-lsp = { version = "0.20", optional = true }
tokio = { version = "1", features = ["rt", "macros", "io-std"], optional = true }
```

Under `[features]`:

```toml
lsp = ["dep:tower-lsp", "dep:tokio"]
```

Run: `nix develop -c cargo check --features lsp`

### Task 3.2: LSP backend struct (3 min)

Red:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_backend_initializes() {
        let backend = MirrorLspBackend::new(None);
        assert!(backend.shatter_cache.is_empty());
    }
}
```

Green:

```rust
use std::collections::HashMap;
use std::sync::RwLock;

pub struct MirrorLspBackend {
    /// Map from file URI to cached ShatterMeta
    shatter_cache: RwLock<HashMap<String, ShatterMeta>>,
    /// Git store, if available
    store: Option<MirrorGitStore>,
    /// Runtime for compilation
    runtime: MirrorRuntime,
}

impl MirrorLspBackend {
    pub fn new(repo_path: Option<&Path>) -> Self {
        let store = repo_path.and_then(|p| MirrorGitStore::open(p).ok());
        MirrorLspBackend {
            shatter_cache: RwLock::new(HashMap::new()),
            store,
            runtime: MirrorRuntime::new(),
        }
    }
}
```

### Task 3.3: MirrorLoss -> LSP Diagnostic mapping (5 min)

This is the core mapping. No tower-lsp dependency needed for the pure function.

Red:

```rust
#[test]
fn loss_to_diagnostics_empty_for_zero_loss() {
    let loss = MirrorLoss::zero();
    let diags = loss_to_diagnostics(&loss);
    assert!(diags.is_empty());
}

#[test]
fn loss_to_diagnostics_warning_for_unrecognized() {
    let mut loss = MirrorLoss::zero();
    loss.parse.unrecognized.push(UnrecognizedDecl {
        keyword: "widget".into(), line: 5, content: "foo".into(),
    });
    let diags = loss_to_diagnostics(&loss);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
    assert!(diags[0].message.contains("widget"));
    assert_eq!(diags[0].line, 4); // 0-indexed for LSP
}

#[test]
fn loss_to_diagnostics_error_for_unresolved() {
    let mut loss = MirrorLoss::zero();
    loss.resolution.unresolved_refs.push(("@missing".into(), TraceOid::new("t")));
    let diags = loss_to_diagnostics(&loss);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
    assert!(diags[0].message.contains("@missing"));
}

#[test]
fn loss_to_diagnostics_for_property_failure() {
    let mut loss = MirrorLoss::zero();
    loss.properties.verdicts.push(PropertyVerdict {
        property: "unique_variants".into(),
        verdict: Imperfect::Failure("duplicate found".into(), 1.0),
    });
    let diags = loss_to_diagnostics(&loss);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
}

#[test]
fn loss_to_diagnostics_for_property_partial() {
    let mut loss = MirrorLoss::zero();
    loss.properties.verdicts.push(PropertyVerdict {
        property: "reachability".into(),
        verdict: Imperfect::Partial((), 0.5),
    });
    let diags = loss_to_diagnostics(&loss);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
}

#[test]
fn loss_to_diagnostics_budget_exhausted() {
    let loss = MirrorLoss::total();
    let diags = loss_to_diagnostics(&loss);
    assert!(diags.iter().any(|d| d.severity == DiagnosticSeverity::Error));
    assert!(diags.iter().any(|d| d.message.contains("budget")));
}
```

Green: implement the pure mapping function.

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct MirrorDiagnostic {
    pub line: usize,       // 0-indexed
    pub col: usize,        // 0-indexed
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>, // M1001, M2001, etc.
}

pub fn loss_to_diagnostics(loss: &MirrorLoss) -> Vec<MirrorDiagnostic> {
    let mut diags = Vec::new();

    // Parse loss: unrecognized declarations
    for unrec in &loss.parse.unrecognized {
        diags.push(MirrorDiagnostic {
            line: unrec.line.saturating_sub(1), // 1-indexed -> 0-indexed
            col: 0,
            end_col: unrec.keyword.len(),
            severity: DiagnosticSeverity::Warning,
            message: format!("unrecognized keyword '{}'", unrec.keyword),
            code: Some("M1001".into()),
        });
    }

    // Resolution loss: unresolved refs
    for (name, _trace) in &loss.resolution.unresolved_refs {
        diags.push(MirrorDiagnostic {
            line: 0,  // TODO: carry line info through resolution
            col: 0,
            end_col: name.len(),
            severity: DiagnosticSeverity::Error,
            message: format!("unresolved reference '{}'", name),
            code: Some("M3001".into()),
        });
    }

    // Property loss: verdicts
    for verdict in &loss.properties.verdicts {
        match &verdict.verdict {
            Imperfect::Success(_) => {} // no diagnostic
            Imperfect::Partial(_, loss_val) => {
                diags.push(MirrorDiagnostic {
                    line: 0,
                    col: 0,
                    end_col: 0,
                    severity: DiagnosticSeverity::Warning,
                    message: format!("property '{}' partial (loss: {})", verdict.property, loss_val),
                    code: Some("M4001".into()),
                });
            }
            Imperfect::Failure(obs, _) => {
                diags.push(MirrorDiagnostic {
                    line: 0,
                    col: 0,
                    end_col: 0,
                    severity: DiagnosticSeverity::Error,
                    message: format!("property '{}' failed: {}", verdict.property, obs),
                    code: Some("M4002".into()),
                });
            }
        }
    }

    // Convergence
    match &loss.convergence {
        Convergence::BudgetExhausted => {
            diags.push(MirrorDiagnostic {
                line: 0, col: 0, end_col: 0,
                severity: DiagnosticSeverity::Error,
                message: "compilation budget exhausted".into(),
                code: Some("M9002".into()),
            });
        }
        Convergence::Oscillating(n) => {
            diags.push(MirrorDiagnostic {
                line: 0, col: 0, end_col: 0,
                severity: DiagnosticSeverity::Warning,
                message: format!("oscillating between {} attractors", n),
                code: Some("M9003".into()),
            });
        }
        _ => {}
    }

    diags
}
```

This function has NO LSP protocol dependency. It maps mirror's domain types to
mirror's diagnostic types. The tower-lsp adapter (Task 3.5) converts these to
`lsp_types::Diagnostic`.

### Task 3.4: Completion items from DeclKind + OpticOp (3 min)

Red:

```rust
#[test]
fn completion_items_include_all_decl_kinds() {
    let items = mirror_completion_items();
    assert!(items.iter().any(|i| i.label == "grammar"));
    assert!(items.iter().any(|i| i.label == "type"));
    assert!(items.iter().any(|i| i.label == "action"));
    assert!(items.iter().any(|i| i.label == "property"));
    assert!(items.iter().any(|i| i.label == "in"));
}

#[test]
fn completion_items_include_optic_ops() {
    let items = mirror_completion_items();
    assert!(items.iter().any(|i| i.label == "<="));
    assert!(items.iter().any(|i| i.label == "="));
    assert!(items.iter().any(|i| i.label == "<"));
}
```

Green:

```rust
pub struct CompletionItem {
    pub label: String,
    pub detail: String,
    pub kind: CompletionKind,
}

pub enum CompletionKind {
    Keyword,
    Operator,
}

pub fn mirror_completion_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // DeclKind keywords
    for kind in DeclKind::all() {
        items.push(CompletionItem {
            label: kind.as_str().to_string(),
            detail: format!("{} declaration", kind.as_str()),
            kind: CompletionKind::Keyword,
        });
    }
    // OpticOp operators
    for op in OpticOp::all() {
        items.push(CompletionItem {
            label: op.as_str().to_string(),
            detail: format!("{} operator", op.as_str()),
            kind: CompletionKind::Operator,
        });
    }
    items
}
```

Note: `DeclKind::all()` and `OpticOp::all()` may need to be added if they don't
exist. They are simple `const fn` returning a slice of all variants.

### Task 3.5: tower-lsp LanguageServer impl (5 min)

This wires the pure functions from 3.3 and 3.4 into the LSP protocol.

**File:** `src/lsp/server.rs`

```rust
#[cfg(feature = "lsp")]
mod server_impl {
    use tower_lsp::jsonrpc::Result;
    use tower_lsp::lsp_types::*;
    use tower_lsp::{Client, LanguageServer};

    use super::*;

    #[tower_lsp::async_trait]
    impl LanguageServer for MirrorLspBackend {
        async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::FULL,
                    )),
                    completion_provider: Some(CompletionOptions::default()),
                    hover_provider: Some(HoverProviderCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            })
        }

        async fn initialized(&self, _: InitializedParams) { /* log */ }
        async fn shutdown(&self) -> Result<()> { Ok(()) }

        async fn did_open(&self, params: DidOpenTextDocumentParams) {
            self.on_change(params.text_document.uri, params.text_document.text).await;
        }

        async fn did_change(&self, params: DidChangeTextDocumentParams) {
            if let Some(change) = params.content_changes.into_iter().last() {
                self.on_change(params.text_document.uri, change.text).await;
            }
        }

        async fn did_save(&self, params: DidSaveTextDocumentParams) {
            // On save: recompile, write .shatter to store
            if let Some(text) = params.text {
                self.on_change(params.text_document.uri, text).await;
            }
        }

        async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
            let items = mirror_completion_items()
                .into_iter()
                .map(|item| lsp_types::CompletionItem {
                    label: item.label,
                    detail: Some(item.detail),
                    kind: Some(match item.kind {
                        CompletionKind::Keyword => CompletionItemKind::KEYWORD,
                        CompletionKind::Operator => CompletionItemKind::OPERATOR,
                    }),
                    ..Default::default()
                })
                .collect();
            Ok(Some(CompletionResponse::Array(items)))
        }

        async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
            // Read .shatter meta from cache -> show holonomy + luminosity
            let uri = params.text_document_position_params.text_document.uri.to_string();
            let cache = self.shatter_cache.read().unwrap();
            if let Some(meta) = cache.get(&uri) {
                Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "**luminosity:** {}\n**holonomy:** {:.4}\n**crystal:** {}",
                            meta.luminosity.as_str(), meta.holonomy, meta.oid
                        ),
                    }),
                    range: None,
                }))
            } else {
                Ok(None)
            }
        }
    }
}
```

The `on_change` method (on `MirrorLspBackend`):

```rust
impl MirrorLspBackend {
    async fn on_change(&self, uri: Url, text: String) {
        let result = self.runtime.compile_source(&text);
        let loss = result.loss().clone();

        // Update .shatter in store if available
        if let Some(ref store) = self.store {
            if let Ok(compiled) = Result::<CompiledShatter, _>::from(result.clone()) {
                let meta = ShatterMeta::from_compiled(&compiled, &loss);
                let shatter = emit_shatter_with_frontmatter(&meta, &text);
                let crystal = fragmentation::encoding::encode(&shatter);
                store.store_crystal(&meta.oid, crystal, shatter.len());
                store.set_file_ref(uri.as_str(), &meta.oid).ok();
                self.shatter_cache.write().unwrap().insert(uri.to_string(), meta);
            }
        }

        // Publish diagnostics
        let diags = loss_to_diagnostics(&loss);
        let lsp_diags: Vec<Diagnostic> = diags.into_iter().map(|d| {
            Diagnostic {
                range: Range {
                    start: Position { line: d.line as u32, character: d.col as u32 },
                    end: Position { line: d.line as u32, character: d.end_col as u32 },
                },
                severity: Some(match d.severity {
                    DiagnosticSeverity::Error => lsp_types::DiagnosticSeverity::ERROR,
                    DiagnosticSeverity::Warning => lsp_types::DiagnosticSeverity::WARNING,
                    DiagnosticSeverity::Info => lsp_types::DiagnosticSeverity::INFORMATION,
                }),
                code: d.code.map(|c| NumberOrString::String(c)),
                source: Some("mirror".into()),
                message: d.message,
                ..Default::default()
            }
        }).collect();

        self.client.publish_diagnostics(uri, lsp_diags, None).await;
    }
}
```

Tests for the tower-lsp impl are integration-level and belong in Phase 5.
The pure functions (loss_to_diagnostics, mirror_completion_items) are already
unit-tested in Tasks 3.3 and 3.4.

**Phase 3 deliverable:** A working LSP server behind `--features lsp`. Diagnostics from loss, keyword completions, holonomy hover.

---

## Phase 4: Wire into CLI

**Goal:** `mirror lsp` (bare) starts the LSP server over stdio.

**Files changed:** `src/cli.rs`, `src/main.rs`

### Task 4.1: Update cmd_lsp dispatch (3 min)

Red:

```rust
#[test]
fn cmd_lsp_no_args_returns_server_start_message() {
    // Without the lsp feature, bare `mirror lsp` should print a message
    let cli = Cli::open("nonexistent.mirror").unwrap();
    let result = cli.dispatch("lsp", &[]);
    // With lsp feature: starts server (won't return in test)
    // Without lsp feature: returns usage message mentioning the feature
    assert!(result.is_ok());
}
```

Green: update `cmd_lsp` in `src/cli.rs`:

```rust
fn cmd_lsp(&self, args: &[String]) -> Result<String, CliError> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        return Ok(Self::lsp_help_text().to_string());
    }

    // No subcommand = start the server
    if args.is_empty() {
        return self.cmd_lsp_serve(args);
    }

    match args[0].as_str() {
        "learn" => self.cmd_lsp_learn(&args[1..]),
        "serve" => self.cmd_lsp_serve(&args[1..]),
        other => Err(CliError::Usage(format!(
            "mirror lsp: unknown subcommand '{}'", other
        ))),
    }
}

#[cfg(feature = "lsp")]
fn cmd_lsp_serve(&self, args: &[String]) -> Result<String, CliError> {
    use crate::lsp::server::run_lsp_server;
    let tcp = args.iter().any(|a| a == "--tcp");
    // This blocks until the server exits
    run_lsp_server(tcp)?;
    Ok("LSP server exited".into())
}

#[cfg(not(feature = "lsp"))]
fn cmd_lsp_serve(&self, _args: &[String]) -> Result<String, CliError> {
    Err(CliError::Usage(
        "mirror lsp: LSP server requires --features lsp\n\
         build with: cargo build --features lsp".into()
    ))
}
```

### Task 4.2: LSP server entry point (3 min)

**File:** `src/lsp/server.rs`

```rust
#[cfg(feature = "lsp")]
pub fn run_lsp_server(tcp: bool) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        if tcp {
            // TCP mode for debugging (port 9257 — "MLSP" on phone keypad)
            todo!("TCP transport")
        } else {
            // Stdio mode — standard LSP transport
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();

            let (service, socket) = tower_lsp::LspService::new(|client| {
                let repo_path = std::env::current_dir().ok();
                MirrorLspBackend::new_with_client(
                    client,
                    repo_path.as_deref(),
                )
            });

            tower_lsp::Server::new(stdin, stdout, socket)
                .serve(service)
                .await;
        }
        Ok(())
    })
}
```

### Task 4.3: Update help text (2 min)

Update `help_text()` in `src/cli.rs`:

```
tools:
  lsp                start the language server (stdio)
  lsp learn          generate @code grammar from tree-sitter + LSP
  lsp --tcp          start on TCP port 9257 (debugging)
```

### Task 4.4: Update lsp/mod.rs (1 min)

```rust
pub mod generate;
pub mod language;
pub mod node_types;

#[cfg(feature = "lsp")]
pub mod server;
```

Run: `nix develop -c cargo build --features lsp`

**Phase 4 deliverable:** `mirror lsp` starts an LSP server over stdio. `mirror lsp --tcp` for debugging. `mirror lsp learn` still works as before.

---

## Phase 5: Editor Integration

**Goal:** Config files for VS Code, Neovim, and Spacemacs. No Rust code.

### Task 5.1: VS Code extension scaffold (5 min)

**New directory:** `editors/vscode/`

**File:** `editors/vscode/package.json`

```json
{
  "name": "mirror-lsp",
  "displayName": "Mirror Language",
  "description": "Language support for .mirror files",
  "version": "0.1.0",
  "engines": { "vscode": "^1.75.0" },
  "categories": ["Programming Languages"],
  "activationEvents": ["onLanguage:mirror"],
  "main": "./extension.js",
  "contributes": {
    "languages": [{
      "id": "mirror",
      "aliases": ["Mirror"],
      "extensions": [".mirror"],
      "configuration": "./language-configuration.json"
    }],
    "configuration": {
      "type": "object",
      "title": "Mirror",
      "properties": {
        "mirror.serverPath": {
          "type": "string",
          "default": "mirror",
          "description": "Path to the mirror binary"
        }
      }
    }
  }
}
```

**File:** `editors/vscode/extension.js`

```javascript
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
  const serverOptions = {
    command: 'mirror',
    args: ['lsp'],
  };
  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'mirror' }],
  };
  client = new LanguageClient('mirror-lsp', 'Mirror', serverOptions, clientOptions);
  client.start();
}

function deactivate() {
  if (client) return client.stop();
}

module.exports = { activate, deactivate };
```

**File:** `editors/vscode/language-configuration.json`

```json
{
  "comments": { "lineComment": "--" },
  "brackets": [["{", "}"], ["(", ")"], ["[", "]"]],
  "autoClosingPairs": [
    { "open": "{", "close": "}" },
    { "open": "(", "close": ")" },
    { "open": "[", "close": "]" },
    { "open": "\"", "close": "\"" }
  ]
}
```

### Task 5.2: Neovim lspconfig entry (2 min)

**New file:** `editors/neovim/mirror.lua`

```lua
-- Add to your nvim config (e.g., after/plugin/lspconfig.lua)
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.mirror then
  configs.mirror = {
    default_config = {
      cmd = { 'mirror', 'lsp' },
      filetypes = { 'mirror' },
      root_dir = lspconfig.util.root_pattern('.git', 'spec.mirror'),
      settings = {},
    },
  }
end

lspconfig.mirror.setup({})

-- Associate .mirror files
vim.filetype.add({
  extension = {
    mirror = 'mirror',
  },
})
```

### Task 5.3: Spacemacs layer (3 min)

**New file:** `editors/spacemacs/mirror/packages.el`

```elisp
(defconst mirror-packages '(lsp-mode))

(defun mirror/init-lsp-mode ()
  (use-package lsp-mode
    :commands lsp
    :hook (mirror-mode . lsp)
    :config
    (lsp-register-client
     (make-lsp-client
      :new-connection (lsp-stdio-connection '("mirror" "lsp"))
      :major-modes '(mirror-mode)
      :server-id 'mirror-lsp))))
```

**New file:** `editors/spacemacs/mirror/config.el`

```elisp
(define-derived-mode mirror-mode prog-mode "Mirror"
  "Major mode for editing .mirror files."
  (setq-local comment-start "-- ")
  (setq-local comment-end ""))

(add-to-list 'auto-mode-alist '("\\.mirror\\'" . mirror-mode))
```

**Phase 5 deliverable:** Drop-in editor configs. `mirror lsp` connects. Gutter shows green/amber/red based on luminosity. Diagnostics from `.shatter` frontmatter appear inline.

---

## Gutter Color Mapping

The three luminosity states map to three gutter colors across all editors:

| Luminosity | Color | Meaning | LSP State |
|-----------|-------|---------|-----------|
| `light` | Green | Zero loss, crystal settled | No diagnostics |
| `dimmed` | Amber | Partial, loss measured | Warning diagnostics |
| `dark` | Red | Failure, error carried | Error diagnostics |

The gutter color is derived from the diagnostics published by the LSP. Editors
already color the gutter based on diagnostic severity:
- No diagnostics = clean gutter (green in themed editors)
- Warnings = yellow/amber gutter markers
- Errors = red gutter markers

No custom rendering needed. The LSP protocol's existing diagnostic severity
mapping produces the correct gutter colors by construction.

---

## Dependency Order

```
Phase 1: .shatter serialization       (0 new deps, pure Rust)
Phase 2: background compilation        (0 new deps, uses existing store)
Phase 3: LSP server                    (tower-lsp, tokio — behind feature flag)
Phase 4: CLI wiring                    (cfg(feature = "lsp") gates)
Phase 5: editor configs               (no Rust, config files only)
```

Each phase merges independently. Phase 1 can ship today. Phase 5 can ship
without Phase 3 (it just won't connect until the server exists).

---

## What This Does NOT Include

- **File watching.** The LSP recompiles on `didOpen`/`didChange`/`didSave`. No
  filesystem watcher. The editor sends the events.
- **Cross-file resolution.** `in @X` resolution requires the boot registry.
  Phase 3 compiles single files. Cross-file resolution is a future tick
  (requires `MirrorRegistry` integration in the LSP backend).
- **Semantic tokens.** Syntax highlighting via LSP semantic tokens. Future tick.
  Tree-sitter grammars handle highlighting today.
- **Code actions.** "Fix this" suggestions. Mirror's philosophy is observation,
  not instruction. Code actions are structurally inappropriate.
- **Rename.** Symbol rename across files. Requires cross-file resolution.
- **Go-to-definition.** Requires cross-file resolution + file-path tracking.

---

*The compiler writes. The LSP reads. The `.shatter` is the protocol.*
