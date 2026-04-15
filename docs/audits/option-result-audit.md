# Option/Result Audit: Where Imperfect Would Preserve Information

Audited: 2026-04-15
Scope: Every `.rs` file in `/Users/alexwolf/dev/projects/mirror/src/`

## The Question

Where does the codebase use `Option<T>` when it should use `Imperfect<T, E, L>` -- because the None case carries information that's being dropped? Where does it use `Result<T, E>` when it should use `Imperfect<T, E, L>` -- because the Ok case might be partial?

## Summary

The codebase already uses Imperfect extensively at the boundaries that matter most: the compilation pipeline (`parse_form`, `compile_source`, `dispatch`, `Store::insert`, `Store::get`, `Transport::transport`). The loss.rs module is well-designed with four-fold loss tracking. Most Option/Result usage is either semantically correct or in interior code where the information loss is local and recoverable.

The highest-priority findings are in the **boundary functions** that bridge between the Imperfect-aware pipeline and the outside world, where Result collapses partial success into total failure.

---

## SHOULD CHANGE -- actively losing information

| File | Line | Current type | Should be | Why | Priority |
|------|------|-------------|-----------|-----|----------|
| declaration.rs | 261 | `MirrorData.grammar_ref: Option<String>` | `Imperfect<String, GrammarRefAbsence, ParseLoss>` | None conflates "not an action" with "action but grammar ref missing" with "action but grammar ref unresolved". Three distinct states collapsed to one. | HIGH |
| declaration.rs | 263 | `MirrorData.body_text: Option<String>` | `Imperfect<String, BodyAbsence, ParseLoss>` | None conflates "abstract action" with "action whose body failed to parse" with "not an action". The `is_abstract` bool partially compensates but doesn't capture parse failure. | HIGH |
| declaration.rs | 267 | `MirrorData.return_type: Option<String>` | `Imperfect<String, ReturnTypeAbsence, ParseLoss>` | None conflates "no return type annotation" with "return type annotation present but unparseable". | MEDIUM |
| declaration.rs | 269 | `MirrorData.parent_ref: Option<String>` | `Imperfect<String, ParentAbsence, ResolutionLoss>` | None conflates "not a grammar" with "grammar but no parent" with "grammar with unresolved parent". | MEDIUM |
| mirror_runtime.rs | 114 | `Form.grammar_ref: Option<String>` | Same as MirrorData above | Same information loss, duplicated in the intermediate representation. | HIGH |
| mirror_runtime.rs | 117 | `Form.body_text: Option<String>` | Same as MirrorData above | Same. | HIGH |
| mirror_runtime.rs | 121 | `Form.return_type: Option<String>` | Same as MirrorData above | Same. | MEDIUM |
| mirror_runtime.rs | 128 | `Form.parent_ref: Option<String>` | Same as MirrorData above | Same. | MEDIUM |
| bundle.rs | 70 | `MirrorCompiler::compile() -> Result<CompiledShatter, MirrorRuntimeError>` | `Imperfect<CompiledShatter, MirrorRuntimeError, MirrorLoss>` | Compilation can succeed partially (warnings, unrecognized decls). The Ok case flattens partial results to total success. The internal `compile_source` returns Imperfect but this method collapses it via `.into()`. | HIGH |
| bundle.rs | 146-157 | `Transport::transport` Err arm | Should propagate `MirrorRuntimeError` as Failure, not Partial with empty string | When compilation fails completely, this returns `Imperfect::Partial(String::new(), loss)` because the error type is `Infallible`. The empty string IS information loss -- the error message from `MirrorRuntimeError` is dropped. | HIGH |
| cli.rs | 78 | `Cli::open() -> Result<Self, CliError>` | `Imperfect<Cli, CliError, MirrorLoss>` | If spec.mirror exists but has warnings, the open succeeds but warnings are dropped. The compile_file call returns CompiledShatter but any ParseLoss from that compilation is lost. | MEDIUM |
| cli.rs | 62 | `Cli.crystal_oid: Option<MirrorHash>` | `Imperfect<MirrorHash, CrystalAbsence, MirrorLoss>` | None conflates "no spec.mirror file" with "spec.mirror failed to compile". The open() method silently swallows compile errors by using if-exists-then-compile. | MEDIUM |
| git_store.rs | 39 | `MirrorGitStore::get_crystal() -> Option<Fractal<String>>` | `Imperfect<Fractal<String>, StoreError, StoreLoss>` | None conflates "OID not in store" with "store corrupted" with "deserialization failed". | MEDIUM |
| git_store.rs | 49 | `MirrorGitStore::head() -> Option<String>` | `Imperfect<String, RefError, StoreLoss>` | None conflates "HEAD ref not set" with "refs directory missing" with "ref file corrupted". | LOW |
| git_store.rs | 67 | `MirrorGitStore::get_branch() -> Option<String>` | `Imperfect<String, RefError, StoreLoss>` | Same as head(). | LOW |
| git_store.rs | 97 | `MirrorGitStore::get_file_ref() -> Option<String>` | `Imperfect<String, RefError, StoreLoss>` | Same as head(). | LOW |
| optic.rs | 26-30 | `ActionDef.grammar_ref: Option<String>`, `ActionDef.body: Option<String>` | Same analysis as MirrorData | Inherits the Option collapse from MirrorData. These are extracted from fragments that already lost the information. | MEDIUM |
| shatter_format.rs | 178 | `parse_shatter_frontmatter() -> Result<(ShatterMeta, &str), String>` | `Imperfect<(ShatterMeta, &str), ParseError, ParseLoss>` | Tolerates unknown sub-keys silently (lines 235, 241, 268) -- this IS measured loss that's being dropped. Unknown keys are information that existed in the source but didn't survive parsing. | MEDIUM |
| loss.rs | 275 | `MirrorLoss.crystal: Option<Oid>` | `Imperfect<Oid, CrystalAbsence, MirrorLoss>` | None conflates "compilation hasn't produced output yet" with "compilation failed" with "compilation not attempted". The Convergence enum partially compensates. | MEDIUM |
| session.rs | 66 | `Session::focus() -> Result<String, String>` | `Imperfect<String, SessionError, SessionLoss>` | The Ok string is always present but could carry loss (e.g., question was ambiguous, deficit partially identified). | LOW |
| session.rs | 76 | `Session::project() -> Result<String, String>` | `Imperfect<String, SessionError, SessionLoss>` | Same -- projection might be partial. | LOW |
| session.rs | 92 | `Session::split() -> Result<String, String>` | `Imperfect<String, SessionError, SessionLoss>` | Same. | LOW |
| session.rs | 119 | `Session::zoom() -> Result<String, String>` | `Imperfect<String, SessionError, SessionLoss>` | Same. | LOW |
| session.rs | 156 | `Session::merge() -> Result<String, String>` | `Imperfect<String, SessionError, MergeConflicts>` | Merge might produce conflicts that are resolved but lossy. The Ok flattens this. | LOW |
| gestalt.rs | 363 | `GestaltProfile::from_gestalt_text() -> Result<Self, String>` | `Imperfect<GestaltProfile, ParseError, ParseLoss>` | Unknown lines are silently skipped (line 489: "for forward compatibility"). This IS information loss -- the profile was parsed but some fields were unrecognized. Exactly the Partial case. | MEDIUM |
| dispatch.rs | 41 | `Response::Error(String)` | Should be `Response::Partial(Value, DispatchLoss)` variant | Response is binary (Ok/Error) when dispatch can partially succeed -- e.g., action ran but returned degraded results. | MEDIUM |

---

## GRAY AREA -- could go either way

| File | Line | Current type | Should be | Why | Priority |
|------|------|-------------|-----------|-----|----------|
| emit_rust.rs | 319 | `parse_field() -> Option<(String, String)>` | Possibly `Imperfect` | Returns None for fields without a colon, but "identity" (no colon) might be a field with an inferred type -- that's a different thing than "not a field". However, this is a pure helper used in one context. | LOW |
| emit_rust.rs | 329 | `extract_subset_ref() -> Option<String>` | Possibly `Imperfect` | Returns None when no subset relation exists, but "no subset" and "subset to an unresolvable type" are different states. Very narrow scope. | LOW |
| extension.rs | 7 | `classify_extension() -> Option<&'static str>` | Possibly `Imperfect` | None means "unknown extension" which is genuinely not-present. But the boundary between "unknown" and "not yet taught" is information. The `classify_file()` wrapper compensates by defaulting to `@code/unknown`. | LOW |
| shatter_format.rs | 75 | `Luminosity::parse() -> Option<Self>` | Possibly `Imperfect` | None for unknown luminosity string. In a frontmatter parser, "dimme" (typo for "dimmed") carries information about the intended state. | LOW |
| kernel.rs | 431 | `Addressable::node_content() -> Option<&str>` | Possibly `Imperfect` | None means "directory" vs Some means "file". This is semantically correct for filesystem but at the trait level, a node whose content failed to read is indistinguishable from a directory. | LOW |
| classifier.rs | 141 | `Weights::from_bytes() -> Option<Self>` | Possibly `Imperfect` | None on wrong byte count, but wrong byte count by 8 bytes (one parameter) vs wrong by 1000 bytes carry different diagnostic information. | LOW |
| filter.rs | 94 | `SignFilter::from_keys_dir() -> Option<Self>` | Possibly `Imperfect` | None conflates "no .pub files in dir" with "dir unreadable" with ".pub files present but unparseable". The from_env() chain partially compensates. | LOW |
| filter.rs | 111 | `SignFilter::from_pub_file() -> Option<Self>` | Possibly `Imperfect` | None conflates "file unreadable" with "file content unparseable". | LOW |
| filter.rs | 140 | `SignFilter::from_env() -> Option<Self>` | Possibly `Imperfect` | None means "no signing identity found anywhere in the env chain". The chain of fallbacks silently drops errors from each stage. | MEDIUM |
| spec.rs | 37 | `SpecConfig::resolve_command() -> Option<&SpecBlock>` | Possibly `Imperfect` | None means "not in spec" which is genuinely not-present. But fuzzy matching ("compil" vs "compile") could return Partial. | LOW |
| spec.rs | 100 | `SpecBlock::setting() -> Option<&str>` | Possibly `Imperfect` | Same -- key not present is genuinely absent. | LOW |
| lsp/server.rs | 39 | `MirrorDiagnostic.code: Option<String>` | Keep as Option | Diagnostic code is either present or not -- no partial state. LSP protocol defines this as optional. | LOW |
| domain/filesystem.rs | 57 | `Folder.content: Option<String>` | Possibly `Imperfect` | None means "directory". But line 93 shows `std::fs::read_to_string(p).ok()` which silently drops IO errors to None, conflating "directory" with "unreadable file". | MEDIUM |
| main.rs | 25 | `Cli::open("spec.mirror").unwrap_or_default()` | Should surface the loss | If open fails, the CLI silently falls back to defaults. The user never learns why their spec wasn't loaded. | MEDIUM |

---

## KEEP AS IS -- Option/Result is semantically correct

| File | Line | Current type | Why it's correct | Priority |
|------|------|-------------|-----------------|----------|
| kernel.rs | 165 | `Trace.parent: Option<TraceOid>` | Root traces genuinely have no parent. Not-present is not partial. | KEEP |
| kernel.rs | 189 | `Trace::parent() -> Option<&TraceOid>` | Delegation to the above. | KEEP |
| kernel.rs | 201 | `Trace::into_result() -> Result<T, E>` | Trace IS the bridge between Result and Imperfect. Trace wraps a Result with trace metadata. The Imperfect-ness lives at the pipeline level, not the individual trace. | KEEP |
| kernel.rs | 395 | `Repo::resolve_ref() -> Option<sha>` | Ref genuinely not present in the store. | KEEP |
| kernel.rs | 396 | `Repo::read_tree() -> Option<node>` | Tree genuinely not in object store. | KEEP |
| store.rs | 123 | `ForeignKey::foreign_hex() -> Option<&str>` | A shard may genuinely not have a foreign key. Not-present is not partial. | KEEP |
| store.rs | 151-154 | `Store::insert/get -> Imperfect` | Already uses Imperfect. Correct. | KEEP |
| loss.rs | 275 | `MirrorLoss.crystal: Option<Oid>` | Listed above as SHOULD CHANGE but borderline. The Convergence enum carries the "why" independently. Could argue this is correct as-is since crystal presence is orthogonal to loss. | KEEP |
| declaration.rs | 54 | `DeclKind::parse() -> Option<DeclKind>` | Unknown string is genuinely not a declaration kind. This is a vocabulary lookup. | KEEP |
| declaration.rs | 169 | `OpticOp::from_token() -> Option<OpticOp>` | Same -- unknown token is not an optic. | KEEP |
| declaration.rs | 204 | `OpticOp::to_decl_kind() -> Option<DeclKind>` | Some optics genuinely have no corresponding DeclKind. | KEEP |
| declaration.rs | 222 | `OpticOp::from_decl_kind() -> Option<OpticOp>` | Same. | KEEP |
| declaration.rs | 332 | `MirrorData::decode() -> Result<Self, String>` | Decoding binary bytes either works or doesn't. No partial state. | KEEP |
| mirror_runtime.rs | 301 | `parse_form() -> Imperfect<MirrorFragment, ...>` | Already uses Imperfect. Correct. | KEEP |
| git_prism.rs | 76 | `GitPrism::open() -> Result<Self, git2::Error>` | Either the repo exists or it doesn't. No partial state. | KEEP |
| git_prism.rs | 82 | `resolve_ref() -> Result<git2::Oid, git2::Error>` | Ref resolution is binary in git. | KEEP |
| git_prism.rs | 120 | `tree_at() -> Result<Vec<TreeEntry>, git2::Error>` | Git tree walk either succeeds or fails. No partial trees. | KEEP |
| git_prism.rs | 144 | `show() -> Result<String, git2::Error>` | Blob read is binary. | KEEP |
| git_prism.rs | 158 | `diff() -> Result<Vec<DiffEntry>, git2::Error>` | Diff computation is binary. | KEEP |
| git_prism.rs | 197 | `log() -> Result<Vec<LogEntry>, git2::Error>` | Log walk is binary. | KEEP |
| git_store.rs | 22 | `MirrorGitStore::open() -> Result<Self, NamespacedStoreError>` | Store either opens or doesn't. | KEEP |
| git_store.rs | 44 | `set_head() -> Result<(), Error>` | Ref write is binary. | KEEP |
| bounded.rs | 80 | `BoundedMemoryStore::get() -> Option<&V>` | Key genuinely not in store. HashMap semantics. | KEEP |
| bounded.rs | 140 | `BoundedMemoryStore::remove() -> Option<V>` | Key genuinely not in store. | KEEP |
| bounded.rs | 160 | `evict_oldest_unsettled() -> Option<String>` | No unsettled entries is a genuine state, not information loss. | KEEP |
| cli.rs | 349-370 | `Cli::command_help() -> Option<&'static str>` | Unknown command genuinely has no help text. | KEEP |
| spec.rs | 65-75 | `SpecConfig::discover()` | Returns Default when no spec found. This is correct -- absence of spec.mirror is not an error. | KEEP |
| ast_prism.rs | 50-72 | ASTPrism Prism trait | Uses Optic (the prism crate's beam type) correctly. Loss tracking is at the pipeline level. | KEEP |
| sign.rs | 42-67 | `sign_oid/verify_oid -> Result<_, SignError>` | Crypto operations are binary -- signature is valid or invalid. | KEEP |
| generate_crate.rs | all | No Option/Result of concern | Pure code generation. No loss boundaries. | KEEP |
| runtime.rs | all | `MetalRuntime` trait | No Option/Result in the interface. | KEEP |
| prism.rs | all | `Prism<V>` enum | No Option/Result -- pure structural type. | KEEP |
| mirror_bf.rs | all | No Option/Result of concern | Pure computation. | KEEP |
| abyss.rs | all | `settle_loop` is `todo!()` | Not yet implemented. Will need Imperfect when implemented. | KEEP |

---

## Patterns Worth Noting

### 1. The Form/MirrorData Option Cluster (HIGH)

The four `Option<String>` fields on `MirrorData` and `Form` (`grammar_ref`, `body_text`, `return_type`, `parent_ref`) are the single biggest source of information loss. They flow through the entire pipeline: parse -> Form -> MirrorData -> fragment -> store -> retrieve -> MirrorData -> Form -> emit. Each stage that touches None loses the "why."

A purpose-built enum per field would be more informative than Imperfect here:

```
enum GrammarRefState {
    Present(String),           // grammar ref was found and resolved
    Inherited,                 // no explicit ref; inherits from parent
    Missing { context: String }, // expected but not found
    NotApplicable,             // this declaration kind doesn't have grammar refs
}
```

### 2. The compile() Collapse (HIGH)

`MirrorCompiler::compile()` and `bundle.rs Transport::transport()` both collapse Imperfect results into Result. The `compile_source()` method returns Imperfect, but `compile()` calls `.into()` on it, which drops Partial to Ok. Anyone calling `compile()` instead of `compile_source()` loses all parse warnings.

### 3. The Git Store Option Chain (MEDIUM)

`MirrorGitStore` uses Option for all retrieval methods. This is consistent with the underlying `NamespacedGitStore` API but loses the distinction between "not stored" and "stored but corrupted/stale." The `store.rs` `Store` trait already models this correctly with Imperfect -- the git store should implement the same trait.

### 4. The Session Result Chain (LOW)

All Session methods return `Result<String, String>`. The session state machine is inherently about partial states (focused but not projected, projected but not split). The String error type drops structural information about what state transition failed and why.

### 5. GestaltProfile Silent Forward Compatibility (MEDIUM)

`from_gestalt_text()` silently skips unknown lines for "forward compatibility." This is a textbook Partial case: the profile was loaded, but some fields were not recognized. The information about what was skipped is useful for diagnostics and migration tooling.
