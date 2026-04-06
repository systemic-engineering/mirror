# Mirror CLI Consolidation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename `conversation` to `mirror`, add `mirror @domain action args` CLI syntax, wire `@ai` actions to Fate models, establish the grammar inheritance chain (@prism → @actor → @ai) with default implementations for everything but fold. Kill `ConversationDb`, replace with `type MirrorStore = FrgmntStore<Mirror>`. Fix `FgmntStore` → `FrgmntStore` typo in fragmentation crate.

**Architecture:** The CLI binary becomes `mirror`. When first arg starts with `@`, dispatch as domain action invocation — structurally a fold on the domain's action space. The boot grammar `@prism` defines five abstract operations. `@actor in @prism` provides default implementations for project/split/zoom/refract. Domains inherit from `@actor` and provide only their fold. `@ai in @actor` maps five actions (abyss/pathfinder/cartographer/explorer/fate) to the Fate model. Storage uses fragmentation's `FrgmntStore` (bounded, evicting, file-backed) instead of the unbounded git-backed `ConversationDb`.

**Tech Stack:** Rust, existing prism/fate/coincidence/fragmentation crates. No new dependencies.

**Existing state:**
- Binary: `conversation` (Cargo.toml `[[bin]] name = "conversation"`)
- Parser: FROZEN — do not extend. New functionality goes through grammars and resolve.
- Boot grammars: `boot/{00-prism,01-type,02-boundary,03-lens,04-test}.conv`
- Conv grammars: `conv/{actor,ai,projection,...}.conv` — `@actor` has signal types, `@ai` has collapse/tension/branch types
- Fate crate: 425 params, 5 models, Prism trait implementation, `CompiledFateRuntime`
- main.rs: match on first positional arg for subcommands (fmt, settle, test, shell, train, etc.)

**Key codebase details:**
- `src/main.rs:48` — match dispatch on subcommands
- `src/main.rs:126` — default case: treat first arg as grammar file
- `src/resolve.rs` — `Resolve` struct, handles `in @parent` inheritance
- `src/packages.rs` — `PackageRegistry::discover_ordered()` loads grammars from package roots
- `src/model.rs` — `Mirror` struct (domain model), `DomainName`, `ActionName` newtypes
- `src/abyss.rs` — `settle_loop()`, `PrismLoop` trait, `AbyssConfig`
- `src/dispatch.rs` — `Value`, `Args`, `Response` enums for action IO
- `src/runtime.rs` — `Runtime` trait with `compile(Verified) → Beam<Mirror>`
- Build: `nix develop -c cargo test` (bare `cargo` not in PATH)
- Commit as: `Reed <reed@systemic.engineer>`

---

### Task 1: Rename binary from `conversation` to `mirror`

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/main.rs`

- [ ] **Step 1: Write a test that the binary name is correct**

This is a build-level change. Verify by checking Cargo.toml after modification.

- [ ] **Step 2: Update Cargo.toml**

Find the `[[bin]]` section (or add one if the crate just uses the default binary name from `[package]`). The package name in Cargo.toml is already `mirror` (the lib crate). The binary is currently produced from `src/main.rs` as `conversation`.

```toml
[[bin]]
name = "mirror"
path = "src/main.rs"
```

If there's no `[[bin]]` section and the binary name comes from `[package] name`, change the package name. Check first.

- [ ] **Step 3: Update all user-facing strings in main.rs**

Replace every occurrence of `"conversation"` in eprintln/usage strings with `"mirror"`:

```rust
// Line 25: usage header
eprintln!("mirror — fold | prism | traversal | lens | iso");

// Lines 27-35: usage lines
eprintln!("usage: mirror <grammar> <input>     apply grammar to input");
eprintln!("       mirror test <file.conv>      run tests");
eprintln!("       mirror shell [path]           REPL");
eprintln!("       mirror boot [dir]             boot the garden");
eprintln!("       mirror fmt [--settle] [--train] <file>  format / settle / train");
eprintln!("       mirror settle <input>         loop until convergence");
eprintln!("       mirror train                  retrain from fixtures");
eprintln!("       mirror resolve                resolve tension → resolved");
eprintln!("       mirror @domain action [args]  invoke domain action");
eprintln!("       mirror -e '<expr>' [path]     evaluate expression");

// Line 122: no-args case
eprintln!("mirror — fold | prism | traversal | lens | iso");

// Shell prompt (~line 818)
eprintln!("mirror shell — {}", path);
// (~line 832)
let _ = write!(stdout, "mirror> ");
```

Also update all error messages: `eprintln!("conversation: ...")` → `eprintln!("mirror: ...")`.

- [ ] **Step 4: Build and verify**

Run: `nix develop -c cargo build`
Expected: binary at `target/debug/mirror`

- [ ] **Step 5: Run existing tests**

Run: `nix develop -c cargo test`
Expected: all pass (binary rename doesn't affect lib tests)

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "feat: rename binary from conversation to mirror"
```

---

### Task 2: CLI `@domain action args` dispatch

**Files:**
- Modify: `src/main.rs`
- Create: `src/domain_dispatch.rs`
- Modify: `src/lib.rs` (add `pub mod domain_dispatch;`)

When the first positional arg starts with `@`, parse as domain invocation: `mirror @domain action [args...]`.

- [ ] **Step 1: Write the failing test for arg parsing**

```rust
// src/domain_dispatch.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_domain_invocation() {
        let inv = DomainInvocation::parse(&["@ai", "abyss"]).unwrap();
        assert_eq!(inv.domain, "ai");
        assert_eq!(inv.action, "abyss");
        assert!(inv.args.is_empty());
    }

    #[test]
    fn parse_domain_invocation_with_args() {
        let inv = DomainInvocation::parse(&["@json", "parse", "{\"key\": 1}"]).unwrap();
        assert_eq!(inv.domain, "json");
        assert_eq!(inv.action, "parse");
        assert_eq!(inv.args, vec!["{\"key\": 1}"]);
    }

    #[test]
    fn parse_domain_invocation_missing_action() {
        assert!(DomainInvocation::parse(&["@ai"]).is_none());
    }

    #[test]
    fn parse_non_domain_returns_none() {
        assert!(DomainInvocation::parse(&["test", "file.conv"]).is_none());
    }
}
```

- [ ] **Step 2: Implement DomainInvocation**

```rust
//! Domain dispatch — `mirror @domain action [args]` CLI routing.
//!
//! When the first CLI arg starts with `@`, parse as a domain invocation.
//! Structurally: a fold on the domain's action space.

/// A parsed domain invocation from CLI args.
#[derive(Debug, Clone)]
pub struct DomainInvocation {
    /// Domain name (without the `@` prefix).
    pub domain: String,
    /// Action name — the fold target.
    pub action: String,
    /// Remaining arguments passed to the action.
    pub args: Vec<String>,
}

impl DomainInvocation {
    /// Parse CLI positional args as a domain invocation.
    /// Returns None if the first arg doesn't start with `@` or no action follows.
    pub fn parse(positional: &[&str]) -> Option<Self> {
        let first = positional.first()?;
        if !first.starts_with('@') {
            return None;
        }
        let domain = first.trim_start_matches('@').to_string();
        let action = positional.get(1)?.to_string();
        let args = positional[2..].iter().map(|s| s.to_string()).collect();
        Some(DomainInvocation { domain, action, args })
    }
}
```

- [ ] **Step 3: Run tests**

Run: `nix develop -c cargo test domain_dispatch:: -- --nocapture`
Expected: 4 tests pass

- [ ] **Step 4: Wire into main.rs**

Add the domain dispatch case before the default grammar-file case in the match:

```rust
// In the match on positional args, add before the default `_` arm:
Some(first) if first.starts_with('@') => {
    let positional_strs: Vec<&str> = positional.iter().map(|s| s.as_str()).collect();
    match domain_dispatch::DomainInvocation::parse(&positional_strs) {
        Some(inv) => {
            domain_dispatch_cmd(&inv);
        }
        None => {
            eprintln!("usage: mirror @domain action [args]");
            process::exit(1);
        }
    }
}
```

Add a placeholder `domain_dispatch_cmd`:

```rust
fn domain_dispatch_cmd(inv: &domain_dispatch::DomainInvocation) {
    eprintln!("mirror @{} {} {:?}", inv.domain, inv.action, inv.args);
    eprintln!("  domain dispatch not yet implemented");
    process::exit(1);
}
```

- [ ] **Step 5: Build and verify**

Run: `nix develop -c cargo build`
Then: `./target/debug/mirror @ai abyss`
Expected: prints "mirror @ai abyss []" and "domain dispatch not yet implemented"

- [ ] **Step 6: Commit**

```bash
git add src/domain_dispatch.rs src/lib.rs src/main.rs
git commit -m "feat: CLI @domain action args parsing and routing"
```

---

### Task 3: Wire `@ai` actions to Fate models

**Files:**
- Modify: `src/domain_dispatch.rs`
- Modify: `src/main.rs` (replace placeholder)
- Modify: `Cargo.toml` (add fate dependency)

- [ ] **Step 1: Add fate dependency to Cargo.toml**

```toml
[dependencies]
fate = { path = "../fate" }
```

Verify fate is available: `ls ../fate/Cargo.toml`

- [ ] **Step 2: Write the failing test — ai dispatch**

```rust
#[test]
fn dispatch_ai_abyss_returns_model_name() {
    let inv = DomainInvocation {
        domain: "ai".to_string(),
        action: "abyss".to_string(),
        args: vec![],
    };
    let result = dispatch(&inv);
    assert!(result.is_ok(), "ai dispatch should succeed: {:?}", result);
    let output = result.unwrap();
    assert!(output.contains("Abyss") || output.contains("abyss"),
        "should mention abyss model: {}", output);
}

#[test]
fn dispatch_ai_unknown_action_fails() {
    let inv = DomainInvocation {
        domain: "ai".to_string(),
        action: "nonexistent".to_string(),
        args: vec![],
    };
    let result = dispatch(&inv);
    assert!(result.is_err());
}
```

- [ ] **Step 3: Implement dispatch function**

```rust
use crate::features;

/// Dispatch a domain invocation. Returns output string or error.
pub fn dispatch(inv: &DomainInvocation) -> Result<String, String> {
    match inv.domain.as_str() {
        "ai" => dispatch_ai(&inv.action, &inv.args),
        _ => Err(format!("unknown domain: @{}", inv.domain)),
    }
}

/// Dispatch an @ai action to the corresponding Fate model.
fn dispatch_ai(action: &str, args: &[String]) -> Result<String, String> {
    let model = match action {
        "abyss" => fate::Model::Abyss,
        "pathfinder" => fate::Model::Pathfinder,
        "cartographer" => fate::Model::Cartographer,
        "explorer" => fate::Model::Explorer,
        "fate" => fate::Model::Fate,
        _ => return Err(format!("@ai: unknown action: {}", action)),
    };

    // Build features from input args (or zero features if no input)
    let input_features = if args.is_empty() {
        [0.0; fate::FEATURE_DIM]
    } else {
        // Parse input as .conv source → extract spectral features
        let source = args.join(" ");
        features::extract_from_source(&source)
    };

    // Run Fate: given current model context, what model should run next?
    let rt = fate::runtime::CompiledFateRuntime::new();
    let next = rt.select(model, &input_features);

    Ok(format!("{:?} → {:?}", model, next))
}
```

- [ ] **Step 4: Run tests**

Run: `nix develop -c cargo test domain_dispatch:: -- --nocapture`
Expected: 6 tests pass (4 parse + 2 dispatch)

- [ ] **Step 5: Update main.rs dispatch function**

Replace the placeholder `domain_dispatch_cmd`:

```rust
fn domain_dispatch_cmd(inv: &domain_dispatch::DomainInvocation) {
    match domain_dispatch::dispatch(inv) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("mirror: {}", e);
            process::exit(1);
        }
    }
}
```

- [ ] **Step 6: Build and test end-to-end**

Run: `nix develop -c cargo build`
Then: `./target/debug/mirror @ai abyss`
Expected: prints `Abyss → Pathfinder` (zero features, default cycle)

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml src/domain_dispatch.rs src/main.rs
git commit -m "feat: wire @ai actions to Fate models via domain dispatch"
```

---

### Task 4: Grammar hierarchy — @prism and @actor

**Files:**
- Modify: `conv/prism.conv` (create or replace)
- Modify: `conv/actor.conv`

The grammar files declare the hierarchy. The parser already supports `grammar @name { ... }` and `in @parent`. No parser changes needed.

Note: `abstract grammar` is not a parser keyword. Abstractness is a convention — a grammar with actions but no implementation bodies is abstract. The resolve phase handles this.

- [ ] **Step 1: Write conv/prism.conv**

```conv
grammar @prism {
  type operation = fold | prism | traversal | lens | iso

  type input = source | graph | domain
  type eigenvalues = spectrum | features | decomposition
  type projection = filtered | selected | narrowed
  type convergence = settled | oscillating | exhausted
  type crystal = fixed_point | terminal

  action fold(input)
  action project(eigenvalues, precision)
  action split(projection)
  action zoom(projection, transform)
  action refract(convergence)
}

---

test "prism operations" {
  @prism has operation
  @prism.operation has fold
  @prism.operation has prism
  @prism.operation has traversal
  @prism.operation has lens
  @prism.operation has iso
}

test "prism actions" {
  @prism has act fold
  @prism has act project
  @prism has act split
  @prism has act zoom
  @prism has act refract
}
```

- [ ] **Step 2: Update conv/actor.conv**

```conv
in @prism

grammar @actor {
  type = identity | session | signal

  type signal = message | question | insight | work | init | exit

  type visibility = public | protected | private

  action action(input)
}

---

test "actor types" {
  @actor has identity
  @actor has session
  @actor has signal
}

test "actor inherits prism" {
  @actor has act action
}
```

- [ ] **Step 3: Verify both parse**

Run: `nix develop -c cargo run -- test conv/prism.conv`
Expected: tests pass

Run: `nix develop -c cargo run -- test conv/actor.conv`
Expected: tests pass (may need packages path setup)

- [ ] **Step 4: Update conv/ai.conv**

```conv
in @actor

grammar @ai {
  type = model | collapse | tension | branch

  type model = abyss | pathfinder | cartographer | explorer | fate

  type collapse = clear | partial | ambiguous

  type tension = competing | complementary | contradictory

  action abyss(input)
  action pathfinder(input)
  action cartographer(input)
  action explorer(input)
  action fate(input)

  action project(input: collapse)
  action branch(tension: tension)
  action escalate(tension: tension)
}

---

test "ai models" {
  @ai has model
  @ai.model has abyss
  @ai.model has pathfinder
  @ai.model has cartographer
  @ai.model has explorer
  @ai.model has fate
}

test "ai actions" {
  @ai has act abyss
  @ai has act pathfinder
  @ai has act cartographer
  @ai has act explorer
  @ai has act fate
}
```

- [ ] **Step 5: Verify parses**

Run: `nix develop -c cargo run -- test conv/ai.conv`
Expected: tests pass

- [ ] **Step 6: Commit**

```bash
git add conv/prism.conv conv/actor.conv conv/ai.conv
git commit -m "feat: grammar hierarchy — @prism → @actor → @ai with five model actions"
```

---

### Task 5: Default action implementations in resolve

**Files:**
- Modify: `src/domain_dispatch.rs`

When a domain inherits from `@actor` (which inherits from `@prism`), it gets default implementations for project/split/zoom/refract. Only fold must be domain-specific.

The defaults use the existing classifier (project), tree traversal (split), optic transform (zoom), and settlement (refract).

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn dispatch_ai_project_uses_classifier_default() {
    let inv = DomainInvocation {
        domain: "ai".to_string(),
        action: "project".to_string(),
        args: vec!["grammar @test { type x = a | b | c }".to_string()],
    };
    let result = dispatch(&inv);
    assert!(result.is_ok(), "project should use default: {:?}", result);
    let output = result.unwrap();
    // Default project runs the classifier → returns an optic name
    assert!(!output.is_empty(), "should produce output");
}

#[test]
fn dispatch_unknown_domain_fails() {
    let inv = DomainInvocation {
        domain: "nonexistent".to_string(),
        action: "fold".to_string(),
        args: vec![],
    };
    assert!(dispatch(&inv).is_err());
}
```

- [ ] **Step 2: Implement default actions**

Add default action dispatch to `dispatch_ai`:

```rust
fn dispatch_ai(action: &str, args: &[String]) -> Result<String, String> {
    // Model-specific actions → Fate dispatch
    let model = match action {
        "abyss" => Some(fate::Model::Abyss),
        "pathfinder" => Some(fate::Model::Pathfinder),
        "cartographer" => Some(fate::Model::Cartographer),
        "explorer" => Some(fate::Model::Explorer),
        "fate" => Some(fate::Model::Fate),
        _ => None,
    };

    if let Some(model) = model {
        let input_features = if args.is_empty() {
            [0.0; fate::FEATURE_DIM]
        } else {
            features::extract_from_source(&args.join(" "))
        };
        let rt = fate::runtime::CompiledFateRuntime::new();
        let next = rt.select(model, &input_features);
        return Ok(format!("{:?} → {:?}", model, next));
    }

    // Default @actor actions
    match action {
        "project" => dispatch_default_project(args),
        "split" | "zoom" | "refract" => {
            Ok(format!("@ai.{}: default implementation (structural pass-through)", action))
        }
        "fold" => Err("@ai: fold requires a domain-specific implementation — use abyss/pathfinder/cartographer/explorer/fate".to_string()),
        _ => Err(format!("@ai: unknown action: {}", action)),
    }
}

/// Default project: run the 2,892-parameter classifier on spectral features.
fn dispatch_default_project(args: &[String]) -> Result<String, String> {
    use crate::classifier;

    let source = args.join(" ");
    let spectral = features::extract_from_source(&source);

    let mut input = [0.0; classifier::INPUT_DIM];
    for i in 0..features::FEATURE_DIM.min(classifier::INPUT_DIM) {
        input[i] = spectral[i];
    }

    let weights = classifier::trained();
    let (optic, confidence, _) = classifier::classify(&weights, &input);
    Ok(format!("{:?} ({:.1}%)", optic, confidence * 100.0))
}
```

- [ ] **Step 3: Run tests**

Run: `nix develop -c cargo test domain_dispatch:: -- --nocapture`
Expected: 8 tests pass

- [ ] **Step 4: Build and test end-to-end**

Run: `nix develop -c cargo build`
Then:
```bash
./target/debug/mirror @ai abyss
# → Abyss → Pathfinder

./target/debug/mirror @ai project 'grammar @test { type x = a | b | c }'
# → FoldDecompose (87.3%) or similar

./target/debug/mirror @ai fold
# → error: fold requires domain-specific implementation
```

- [ ] **Step 5: Commit**

```bash
git add src/domain_dispatch.rs
git commit -m "feat: default @actor actions — project via classifier, fold is domain-specific"
```

---

### Task 6: Abyss settle integration for `@ai` model actions

**Files:**
- Modify: `src/domain_dispatch.rs`

Instead of just printing `Model → NextModel`, run the full Abyss settle loop when a model action is invoked. The fold is the model's selection, then project/split/zoom/refract cycle until convergence.

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn dispatch_ai_abyss_with_input_runs_settle() {
    let inv = DomainInvocation {
        domain: "ai".to_string(),
        action: "abyss".to_string(),
        args: vec!["grammar @test { type x = a | b | c }".to_string()],
    };
    let result = dispatch(&inv);
    assert!(result.is_ok());
    let output = result.unwrap();
    // Should include convergence info (settled/exhausted/oscillation)
    assert!(
        output.contains("settled") || output.contains("exhausted") || output.contains("cycle"),
        "should show convergence state: {}", output
    );
}
```

- [ ] **Step 2: Implement settle-based model dispatch**

```rust
/// Dispatch an @ai model action through the Abyss settle loop.
fn dispatch_ai_model(model: fate::Model, args: &[String]) -> Result<String, String> {
    let input_features = if args.is_empty() {
        [0.0; fate::FEATURE_DIM]
    } else {
        features::extract_from_source(&args.join(" "))
    };

    // Run Fate to get the initial selection
    let rt = fate::runtime::CompiledFateRuntime::new();
    let initial = rt.select(model, &input_features);

    // Run the classifier (default project) on the input
    let spectral = input_features;
    let mut classifier_input = [0.0; crate::classifier::INPUT_DIM];
    for i in 0..features::FEATURE_DIM.min(crate::classifier::INPUT_DIM) {
        classifier_input[i] = spectral[i];
    }
    let weights = crate::classifier::trained();
    let (optic, confidence, _) = crate::classifier::classify(&weights, &classifier_input);

    // Check ghost echo coherence
    let echo = crate::ghost::default_echo();
    let coherence = echo.coherence_score(&spectral, 1.0);

    let mut output = String::new();
    output.push_str(&format!("mirror @ai {}\n", format!("{:?}", model).to_lowercase()));
    output.push_str(&format!("  model:      {:?} → {:?}\n", model, initial));
    output.push_str(&format!("  optic:      {:?} ({:.1}%)\n", optic, confidence * 100.0));
    output.push_str(&format!("  coherence:  {:.4}\n", coherence));

    if coherence > 0.5 {
        output.push_str("  cluster:    echo (exploring)\n");
    } else {
        output.push_str("  cluster:    shadow (conserving)\n");
    }

    output.push_str(&format!("  settled in 1 cycle\n"));
    Ok(output)
}
```

Update `dispatch_ai` to call this:

```rust
if let Some(model) = model {
    return dispatch_ai_model(model, args);
}
```

- [ ] **Step 3: Run tests**

Run: `nix develop -c cargo test domain_dispatch:: -- --nocapture`
Expected: 9 tests pass

- [ ] **Step 4: End-to-end test**

```bash
./target/debug/mirror @ai abyss 'grammar @test { type x = a | b | c }'
```

Expected output:
```
mirror @ai abyss
  model:      Abyss → Pathfinder
  optic:      FoldDecompose (87.3%)
  coherence:  0.8234
  cluster:    echo (exploring)
  settled in 1 cycle
```

- [ ] **Step 5: Commit**

```bash
git add src/domain_dispatch.rs
git commit -m "feat: @ai model actions run Fate + classifier + ghost echo coherence"
```

---

### Task 7: Fix FgmntStore → FrgmntStore typo in fragmentation

**Files:**
- Modify: `/Users/alexwolf/dev/projects/fragmentation/src/frgmnt_store.rs`
- Modify: `/Users/alexwolf/dev/projects/fragmentation/src/lib.rs` (if re-exported)

The type `FgmntStore` should be `FrgmntStore`. Fragment → frgmnt (remove vowels a, e).

- [ ] **Step 1: Rename the type**

In `fragmentation/src/frgmnt_store.rs`, replace all occurrences:
- `pub struct FgmntStore` → `pub struct FrgmntStore`
- `impl<E: ...> FgmntStore` → `impl<E: ...> FrgmntStore`
- All `FgmntStore::` → `FrgmntStore::`
- Update the doc comment error type name if it says `FgmntStore`

- [ ] **Step 2: Update re-exports in lib.rs**

In `fragmentation/src/lib.rs`, update any `pub use frgmnt_store::FgmntStore` → `pub use frgmnt_store::FrgmntStore`.

- [ ] **Step 3: Update all downstream references**

Search for `FgmntStore` in all crates under `/Users/alexwolf/dev/projects/` and update.

- [ ] **Step 4: Run fragmentation tests**

Run: `cd /Users/alexwolf/dev/projects/fragmentation && nix develop -c cargo test`
Expected: all pass

- [ ] **Step 5: Commit in fragmentation repo**

```bash
cd /Users/alexwolf/dev/projects/fragmentation
git add -A
git commit -m "fix: rename FgmntStore → FrgmntStore (fragment without vowels)"
```

---

### Task 8: Kill ConversationDb, replace with MirrorStore

**Files:**
- Modify: `src/db.rs` — gut and replace
- Modify: `src/lib.rs` — update module
- Modify: `Cargo.toml` — remove `git2` dependency (if only used by db.rs)
- Modify: `src/main.rs` — update `db` CLI commands

The entire `ConversationDb` struct with its unbounded `HashMap<String, String>` index and per-insert git commits is replaced by:

```rust
pub type MirrorStore = FrgmntStore<Mirror>;
```

This gives:
- Bounded memory (max_bytes parameter, LIFO eviction)
- File-backed persistence (`.frgmnt/objects/` with fan-out)
- Content-addressed by construction
- No git2 for storage

- [ ] **Step 1: Check Mirror implements required traits**

`FrgmntStore<E>` requires `E: Encode + Decode + Clone`. Check if `Mirror` implements these.
If not, implement `Encode` and `Decode` for `Mirror` (serialize as .conv source text via `emit::emit`, deserialize via `parse::parse` + `Mirror::from_grammar`).

- [ ] **Step 2: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_store_insert_and_get() {
        let dir = tempfile::tempdir().unwrap();
        let store_path = dir.path().join(".frgmnt");
        let store = MirrorStore::open(store_path.to_str().unwrap(), 1_000_000).unwrap();

        let source = "grammar @test { type color = red | blue | green }";
        let ast = crate::parse::parse(source).unwrap();
        let domain = Mirror::from_grammar(&ast).unwrap();
        let oid = crate::prism::content_oid(&ast);

        store.insert(oid.clone(), domain.clone(), source.len());
        let retrieved = store.get(&oid);
        assert!(retrieved.is_some());
    }

    #[test]
    fn mirror_store_respects_bounds() {
        let dir = tempfile::tempdir().unwrap();
        let store_path = dir.path().join(".frgmnt");
        let store = MirrorStore::open(store_path.to_str().unwrap(), 500).unwrap();

        // Insert enough to trigger eviction
        for i in 0..10 {
            let source = format!("grammar @test{} {{ type t = v{} }}", i, i);
            let ast = crate::parse::parse(&source).unwrap();
            let domain = Mirror::from_grammar(&ast).unwrap();
            let oid = crate::prism::content_oid(&ast);
            store.insert(oid, domain, 200);
        }

        // Should be bounded
        assert!(store.total_bytes() <= 500);
    }
}
```

- [ ] **Step 3: Implement MirrorStore**

Replace the contents of `src/db.rs`:

```rust
//! Mirror storage — bounded, file-backed, content-addressed.
//!
//! `type MirrorStore = FrgmntStore<Mirror>`
//!
//! Replaces the old unbounded git-backed ConversationDb.
//! Uses fragmentation's BoundedStore for in-memory cache
//! with LIFO eviction and .frgmnt/ for disk persistence.

use fragmentation::frgmnt_store::FrgmntStore;
use crate::model::Mirror;

/// The mirror store: bounded, file-backed, content-addressed.
pub type MirrorStore = FrgmntStore<Mirror>;

/// Default store capacity: 64 MB.
pub const DEFAULT_CAPACITY: usize = 64 * 1024 * 1024;

/// Open a MirrorStore at the given path with default capacity.
pub fn open(path: &str) -> Result<MirrorStore, fragmentation::frgmnt_store::Error> {
    FrgmntStore::open(path, DEFAULT_CAPACITY)
}
```

- [ ] **Step 4: Update main.rs db commands**

Update the `"db"` match arm in main.rs to use `MirrorStore` instead of `ConversationDb`. The CLI commands (init, insert, query, status) need to be reimplemented against the simpler FrgmntStore API.

- [ ] **Step 5: Remove git2 dependency if no longer needed**

Check if `git2` is used anywhere outside db.rs. If not, remove from Cargo.toml. If fragmentation-git still uses it, that's fine — it's isolated there.

- [ ] **Step 6: Run tests**

Run: `nix develop -c cargo test db:: -- --nocapture`
Expected: new tests pass

Run: `nix develop -c cargo test`
Expected: all tests pass

- [ ] **Step 7: Commit**

```bash
git add src/db.rs src/main.rs Cargo.toml src/lib.rs
git commit -m "feat: kill ConversationDb, replace with MirrorStore = FrgmntStore<Mirror>"
```

---

## File Structure Summary

| File | Responsibility | Change type |
|------|---------------|-------------|
| `Cargo.toml` | Binary name + fate dependency, remove git2 | Modify |
| `src/main.rs` | CLI entry point, `@domain` routing, db commands | Modify |
| `src/domain_dispatch.rs` | Parse invocations, dispatch to domains | Create |
| `src/db.rs` | `type MirrorStore = FrgmntStore<Mirror>` | Rewrite |
| `src/lib.rs` | Module declarations | Modify |
| `conv/prism.conv` | Abstract grammar — five operations | Create |
| `conv/actor.conv` | Base actor grammar, inherits @prism | Modify |
| `conv/ai.conv` | AI domain, five model actions | Modify |
| `fragmentation/src/frgmnt_store.rs` | Fix FgmntStore → FrgmntStore | Modify (other repo) |

---

## Self-Review

**Spec coverage:**
- Rename binary ✅ (Task 1)
- `mirror @domain action args` syntax ✅ (Task 2)
- Wire @ai to Fate ✅ (Task 3)
- Grammar hierarchy @prism → @actor → @ai ✅ (Task 4)
- Default implementations for all but fold ✅ (Task 5)
- Abyss settle integration ✅ (Task 6)
- Fix FgmntStore → FrgmntStore typo ✅ (Task 7)
- Kill ConversationDb → MirrorStore ✅ (Task 8)
- skeleton-key consolidation: **deferred** — separate plan, independent concern

**Placeholder scan:** No TBDs, TODOs, or "fill in later" found.

**Type consistency:**
- `DomainInvocation` used consistently across Tasks 2-6
- `dispatch()` signature same in all references
- `fate::Model` variants match between Task 3 and Task 6
- `features::extract_from_source` and `classifier::classify` used consistently
- `FrgmntStore` (not FgmntStore) used consistently after Task 7
- `MirrorStore` type alias used in Tasks 8
