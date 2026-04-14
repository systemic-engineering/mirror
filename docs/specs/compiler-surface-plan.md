# Compiler Surface Plan — The Unified Fold

**Author:** Mara
**Date:** 2026-04-14
**Status:** Plan
**Branch:** `reed/fold-operator`
**Base:** 499 tests passing, 5 ignored

---

## The Thesis

Mirror unifies what Elixir splits across four tools:

```
mix compile  → compile fold   (Failure)
mix format   → template fold  (iso, lossless)
mix credo    → property fold  (Partial verdicts)
dialyzer     → resolution fold (type checking)
```

One command. One type (`Imperfect`). One holonomy number.

---

## Phase 1: Operator Table

**Goal:** Add `!=`, `>=`, `<`, `>` to `OpticOp`. Wire into parser. Tests.

Currently `OpticOp` has six variants: `Iso`, `Fold`, `Split`, `Focus`, `Zoom`, `Refract`. The operator table needs four more tokens recognized by `from_token`, but only two new enum variants — `NotIso` and `Unfold`. `<` and `>` are `Subset` and `Superset`.

### 1.1 Red: `OpticOp::NotIso` variant and `!=` token

**File:** `src/declaration.rs`

Add test:

```rust
#[test]
fn operator_not_iso_maps_to_bang_equals() {
    assert_eq!(OpticOp::from_token("!="), Some(OpticOp::NotIso));
}

#[test]
fn optic_op_not_iso_as_str() {
    assert_eq!(OpticOp::NotIso.as_str(), "!=");
}

#[test]
fn optic_op_not_iso_display() {
    assert_eq!(format!("{}", OpticOp::NotIso), "!=");
}
```

```
nix develop -c cargo test -- operator_not_iso  # FAIL: no variant NotIso
```

### 1.2 Green: Add `NotIso` variant

**File:** `src/declaration.rs`

Add `NotIso` to the `OpticOp` enum:

```rust
/// `!=` — superposition broken. The not-iso: these are NOT the same.
/// Irreversibility axiom. `collapse(block) != iso`.
NotIso,
```

Wire into `from_token`:

```rust
"!=" => Some(OpticOp::NotIso),
```

Wire into `as_str`:

```rust
OpticOp::NotIso => "!=",
```

Wire into `to_decl_kind` (returns `None` — `!=` is structural like `=`):

```rust
OpticOp::NotIso => None,
```

Update `optic_op_as_str_roundtrips_through_from_token` to include `OpticOp::NotIso`.

```
nix develop -c cargo test -- operator_not_iso  # PASS
```

### 1.3 Red: Tokenizer recognizes `!=`

**File:** `src/mirror_runtime.rs`

The tokenizer currently handles `=` as `Tok::Equals`. The `!` character falls into the default word-building branch but is not alphanumeric, so `!=` would be tokenized as two separate tokens. Need to handle `!` in the operator character set or add `Tok::NotEquals`.

Add test:

```rust
#[test]
fn tokenizer_handles_bang_equals() {
    let source = "type collapse(block) != iso\n";
    let result = parse_form(source);
    let form = result.ok().unwrap();
    assert!(form.optic_ops.contains(&OpticOp::NotIso));
}
```

```
nix develop -c cargo test -- tokenizer_handles_bang_equals  # FAIL
```

### 1.4 Green: Wire `!=` into tokenizer and parser

**File:** `src/mirror_runtime.rs`

Option A (simplest): Add `!` to the operator character set in the tokenizer so `!=` becomes `Tok::Word("!=")`. Then in `parse_decl`, after the fold check and before the `Equals` check, check for `Word("!=")`:

```rust
// NotIso operator: `!=` is tokenized as Word("!=") (or Word("!") + Equals)
let is_not_iso = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "!=" || w == "!");
```

If `Word("!")` followed by `Tok::Equals`, consume both and push `OpticOp::NotIso`. Then collect variants the same way as Iso.

```
nix develop -c cargo test -- tokenizer_handles_bang_equals  # PASS
```

### 1.5 Red: `OpticOp::Unfold` variant and `>=` token

**File:** `src/declaration.rs`

```rust
#[test]
fn operator_unfold_maps_to_gt_equals() {
    assert_eq!(OpticOp::from_token(">="), Some(OpticOp::Unfold));
}

#[test]
fn optic_op_unfold_as_str() {
    assert_eq!(OpticOp::Unfold.as_str(), ">=");
}
```

```
nix develop -c cargo test -- operator_unfold  # FAIL
```

### 1.6 Green: Add `Unfold` variant

**File:** `src/declaration.rs`

```rust
/// `>=` — unfold. Right receives left. The dual of fold.
Unfold,
```

Wire `from_token(">=")`, `as_str`, `to_decl_kind` (returns `None`). Update roundtrip test.

```
nix develop -c cargo test -- operator_unfold  # PASS
```

### 1.7 Red: `OpticOp::Subset` and `OpticOp::Superset`

**File:** `src/declaration.rs`

```rust
#[test]
fn operator_subset_maps_to_lt() {
    assert_eq!(OpticOp::from_token("<"), Some(OpticOp::Subset));
}

#[test]
fn operator_superset_maps_to_gt() {
    assert_eq!(OpticOp::from_token(">"), Some(OpticOp::Superset));
}
```

```
nix develop -c cargo test -- operator_subset  # FAIL: currently "<" is not matched
```

### 1.8 Green: Add `Subset` and `Superset`

**File:** `src/declaration.rs`

```rust
/// `<` — proper subset.
Subset,
/// `>` — proper superset.
Superset,
```

Wire `from_token`. Important: `<` alone is `Subset`, but `<=` is `Fold`. The match must check `<=` before `<`. Currently `from_token` receives a single token string, so `"<"` maps to `Subset` and `"<="` maps to `Fold` — no ambiguity.

Wire `as_str`, `to_decl_kind` (both return `None`).

```
nix develop -c cargo test -- operator_subset  # PASS
nix develop -c cargo test -- operator_superset  # PASS
```

### 1.9 Red: `>=` tokenizer + parser

**File:** `src/mirror_runtime.rs`

The tokenizer currently groups `>` into operator sequences. `>=` would be tokenized as `Word(">")` + `Tok::Equals` (since `=` is `Tok::Equals`). The parser needs to handle this the same way it handles `<=`:

```rust
#[test]
fn parse_unfold_operator() {
    let source = "action emit(resolved) >= crystal\n";
    let result = parse_form(source);
    let form = result.ok().unwrap();
    assert!(form.optic_ops.contains(&OpticOp::Unfold));
}
```

```
nix develop -c cargo test -- parse_unfold_operator  # FAIL
```

### 1.10 Green: Wire `>=` into parser

**File:** `src/mirror_runtime.rs`

In `parse_decl`, after the fold (`<=`) check, add an unfold (`>=`) check:

```rust
let is_unfold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == ">")
    && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
```

Same collection logic as fold.

```
nix develop -c cargo test -- parse_unfold_operator  # PASS
```

### 1.11 Commit

```
git add src/declaration.rs src/mirror_runtime.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: complete operator table — !=, >=, <, > wired into OpticOp and parser"
```

---

## Phase 2: Type Surface

**Goal:** `observation`, `template`, `action`, `collapse` as first-class types with `!=` constraint checking. The type declarations in `01-meta.mirror` already define these; this phase makes the compiler understand them semantically.

### 2.1 Red: Parse `type collapse(block) != iso`

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn parse_not_iso_type() {
    let source = "type collapse(block) != iso\n";
    let result = parse_form(source);
    let form = result.ok().unwrap();
    assert_eq!(form.kind, DeclKind::Type);
    assert_eq!(form.name, "collapse");
    assert!(form.optic_ops.contains(&OpticOp::NotIso));
    assert_eq!(form.variants, vec!["iso"]);
}
```

This test exercises the `!=` path added in Phase 1. It should already pass if Phase 1 was done correctly. If not, this is where the gap shows.

```
nix develop -c cargo test -- parse_not_iso_type  # verify PASS from Phase 1
```

### 2.2 Red: `DeclKind::Template`

**File:** `src/declaration.rs`

```rust
#[test]
fn decl_kind_template() {
    assert_eq!(DeclKind::parse("template"), Some(DeclKind::Template));
    assert_eq!(DeclKind::Template.as_str(), "template");
}
```

```
nix develop -c cargo test -- decl_kind_template  # FAIL: no variant Template
```

### 2.3 Green: Add `Template` to `DeclKind`

**File:** `src/declaration.rs`

Add `Template` variant to `DeclKind`. Wire `parse("template")` and `as_str()`. Update the `decl_kind_parse_roundtrip_all_variants` test to include it (bump count to 24).

```
nix develop -c cargo test -- decl_kind_template  # PASS
```

### 2.4 Red: `DeclKind::Observation`

**File:** `src/declaration.rs`

```rust
#[test]
fn decl_kind_observation() {
    assert_eq!(DeclKind::parse("observation"), Some(DeclKind::Observation));
}
```

```
nix develop -c cargo test -- decl_kind_observation  # FAIL
```

### 2.5 Green: Add `Observation` to `DeclKind`

Same pattern. Wire parse + as_str. Bump variant count to 25.

```
nix develop -c cargo test -- decl_kind_observation  # PASS
```

### 2.6 Red: Parse `template fmt(grammar, block) = iso { ... }`

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn parse_template_declaration() {
    let source = "template fmt(grammar, block) = iso {\n  lens hoist\n  lens sort_deps\n}\n";
    let result = parse_form(source);
    let form = result.ok().unwrap();
    assert_eq!(form.kind, DeclKind::Template);
    assert_eq!(form.name, "fmt");
    assert_eq!(form.params, vec!["grammar", "block"]);
    assert!(form.optic_ops.contains(&OpticOp::Iso));
    assert_eq!(form.children.len(), 2);
    assert_eq!(form.children[0].kind, DeclKind::Lens);
    assert_eq!(form.children[0].name, "hoist");
}
```

```
nix develop -c cargo test -- parse_template_declaration  # should PASS if DeclKind::Template is wired
```

### 2.7 Red: `IsoConstraint` — type-level invariant checking

**File:** `src/mirror_runtime.rs`

The `!=` operator on a type declaration establishes a constraint: "this type is NOT iso." The compiler should record this and check it during property evaluation.

```rust
#[test]
fn not_iso_constraint_recorded() {
    let source = "type collapse(block) != iso\ntype template(grammar, block) = iso\n";
    let result = parse_form(source);
    let form = result.ok().unwrap();
    // The file-level form has two children
    let collapse = &form.children[0];
    let template = &form.children[1];
    assert!(collapse.optic_ops.contains(&OpticOp::NotIso));
    assert!(template.optic_ops.contains(&OpticOp::Iso));
}
```

This should pass from Phase 1 work. The semantic checking (verifying that a template actually IS iso) comes in Phase 4 (properties).

### 2.8 Red: M2005 — `<=` in type declaration

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn error_fold_in_type_declaration() {
    let source = "type x <= y\n";
    let result = parse_form(source);
    assert!(result.is_err(), "<= should not be valid in type declarations");
}
```

```
nix develop -c cargo test -- error_fold_in_type_declaration  # FAIL: currently parses
```

### 2.9 Green: Reject `<=` in type declarations

**File:** `src/mirror_runtime.rs`

After parsing a fold operator, check if the declaration kind is `Type` or `Template`. If so, return `Failure` with M2005:

```rust
if is_fold && matches!(kind, DeclKind::Type | DeclKind::Template) {
    return Err(err("M2005: fold operator `<=` is not valid in type declarations"));
}
```

```
nix develop -c cargo test -- error_fold_in_type_declaration  # PASS
```

### 2.10 Commit

```
git add src/declaration.rs src/mirror_runtime.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: type surface — Template, Observation DeclKinds, != constraint, M2005 enforcement"
```

---

## Phase 3: Formatter

**Goal:** `mirror fmt` command. Canonical ordering. Idempotent. Template (iso). Wadler-Lindig algebra documents.

### 3.1 Red: `emit_form` — Form to canonical source text

**File:** `src/mirror_runtime.rs` (new function, tests in same file)

The formatter's core is a function that takes a `Form` and produces canonical `.mirror` source text. Round-trip property: `parse(emit(parse(source))) == parse(source)`.

```rust
#[test]
fn emit_form_type_simple() {
    let source = "type color = red | blue\n";
    let form = parse_form(source).ok().unwrap();
    let emitted = emit_form(&form);
    assert_eq!(emitted, "type color = red | blue\n");
}

#[test]
fn emit_form_roundtrip() {
    let source = "type color = red | blue\n";
    let form = parse_form(source).ok().unwrap();
    let emitted = emit_form(&form);
    let reparsed = parse_form(&emitted).ok().unwrap();
    assert_eq!(form, reparsed);
}
```

```
nix develop -c cargo test -- emit_form_type_simple  # FAIL: no function emit_form
```

### 3.2 Green: Implement `emit_form`

**File:** `src/mirror_runtime.rs`

```rust
pub fn emit_form(form: &Form) -> String {
    let mut out = String::new();
    emit_form_inner(form, &mut out, 0);
    out
}

fn emit_form_inner(form: &Form, out: &mut String, indent: usize) {
    // ... pattern match on form.kind, emit keyword + name + params + operator + variants
    // For blocks: emit children indented by 2
}
```

This is the Wadler-Lindig equivalent for mirror: declarative document construction. Start simple (string building), upgrade to algebra documents later if line-width awareness is needed.

```
nix develop -c cargo test -- emit_form_type_simple  # PASS
```

### 3.3 Red: Canonical ordering

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn fmt_canonical_order() {
    // Declarations out of order: types before imports
    let source = "type color = red | blue\nin @prism\n";
    let form = parse_form(source).ok().unwrap();
    let formatted = fmt_form(&form);
    // Canonical: in first, then type
    assert!(formatted.starts_with("in @prism\n"));
    assert!(formatted.contains("type color"));
}
```

Canonical order:
1. `in` — imports first
2. `type` / `observation` — types next
3. `template` — templates next
4. `grammar` — grammars next
5. `property` — properties next
6. `action` — actions last

Within a category: dependency sort (if type `A` references type `B`, `B` comes first). If no dependency, alphabetical.

```
nix develop -c cargo test -- fmt_canonical_order  # FAIL: no function fmt_form
```

### 3.4 Green: Implement `fmt_form`

**File:** `src/mirror_runtime.rs`

```rust
pub fn fmt_form(form: &Form) -> String {
    let sorted = canonical_sort(form);
    emit_form(&sorted)
}

fn canonical_sort(form: &Form) -> Form {
    // Sort children by DeclKind priority, then alphabetically within kind
    let mut sorted = form.clone();
    sorted.children.sort_by(|a, b| {
        let pa = decl_priority(&a.kind);
        let pb = decl_priority(&b.kind);
        pa.cmp(&pb).then(a.name.cmp(&b.name))
    });
    sorted
}

fn decl_priority(kind: &DeclKind) -> u8 {
    match kind {
        DeclKind::In => 0,
        DeclKind::Type | DeclKind::Observation => 1,
        DeclKind::Template => 2,
        DeclKind::Grammar => 3,
        DeclKind::Property => 4,
        DeclKind::Requires | DeclKind::Invariant | DeclKind::Ensures => 5,
        DeclKind::Action => 6,
        DeclKind::Out => 7,
        _ => 10,
    }
}
```

```
nix develop -c cargo test -- fmt_canonical_order  # PASS
```

### 3.5 Red: Idempotency

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn fmt_idempotent() {
    let source = "type b = x\ntype a = y\nin @prism\n";
    let once = fmt_form(&parse_form(source).ok().unwrap());
    let twice = fmt_form(&parse_form(&once).ok().unwrap());
    assert_eq!(once, twice, "formatter must be idempotent");
}
```

```
nix develop -c cargo test -- fmt_idempotent  # PASS (should pass by construction)
```

### 3.6 Red: Formatting preserves OID (iso property)

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn fmt_preserves_oid() {
    let source = "type b = x\nin @prism\ntype a = y\n";
    let form = parse_form(source).ok().unwrap();
    let formatted_form = parse_form(&fmt_form(&form)).ok().unwrap();
    let shatter = Shatter;
    let frag_before = shatter.compile_form(&form);
    let frag_after = shatter.compile_form(&formatted_form);
    // Content OIDs should be identical — formatting is iso
    // NOTE: This will fail if compile_form is order-dependent.
    // The spectral hash IS order-invariant, so this should pass.
    // If it fails, that's a real finding: compile_form needs to sort.
    assert_eq!(frag_before.oid(), frag_after.oid());
}
```

```
nix develop -c cargo test -- fmt_preserves_oid  # may FAIL — reveals if compile_form is order-dependent
```

### 3.7 Green: Make `compile_form` order-invariant

If the test fails: sort children in `Form::to_fragment()` before computing the content hash. The spectral hash must be order-invariant for formatting to be iso.

If the test passes: formatting is already iso. Document this.

### 3.8 Red: Comment preservation

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn fmt_preserves_comments() {
    let source = "-- this is a comment\ntype color = red | blue\n";
    let form = parse_form(source).ok().unwrap();
    let formatted = fmt_form(&form);
    assert!(formatted.contains("-- this is a comment"));
}
```

Comments are currently discarded by the tokenizer. This test will fail. The fix requires the tokenizer to capture comments as a parallel stream (like Elixir's formatter does). This is deferred — the test documents the gap.

```
nix develop -c cargo test -- fmt_preserves_comments  # FAIL (expected — deferred)
```

Mark as `#[ignore]` with a note: "comments not yet preserved by formatter — requires tokenizer change."

### 3.9 Commit

```
git add src/mirror_runtime.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: mirror fmt — emit_form, canonical ordering, idempotency, OID preservation"
```

---

## Phase 4: Property Checks

**Goal:** Wire property evaluation. `property p(grammar) <= verdict` executes the optic pipeline and returns `Imperfect`. Start with two concrete properties: `canonical_order` and `types_lowercase`.

### 4.1 Red: `PropertyRunner` trait

**File:** `src/mirror_runtime.rs` (or new file `src/property.rs`)

```rust
#[test]
fn property_types_lowercase_passes() {
    let source = "type color = red | blue\ntype size = small | large\n";
    let form = parse_form(source).ok().unwrap();
    let verdict = run_property("types_lowercase", &form);
    assert!(verdict.is_ok());
}

#[test]
fn property_types_lowercase_fails_on_uppercase() {
    let source = "type Color = red | blue\n";
    let form = parse_form(source).ok().unwrap();
    let verdict = run_property("types_lowercase", &form);
    assert!(verdict.is_partial() || verdict.is_err());
}
```

```
nix develop -c cargo test -- property_types_lowercase  # FAIL: no function run_property
```

### 4.2 Green: Implement `run_property`

**File:** `src/mirror_runtime.rs` (or `src/property.rs`, added to `lib.rs`)

```rust
use crate::loss::PropertyVerdict;
use prism::Imperfect;

pub fn run_property(name: &str, form: &Form) -> Imperfect<(), String, f64> {
    match name {
        "types_lowercase" => check_types_lowercase(form),
        "canonical_order" => check_canonical_order(form),
        _ => Imperfect::Failure(format!("unknown property: {}", name), 1.0),
    }
}

fn check_types_lowercase(form: &Form) -> Imperfect<(), String, f64> {
    let violations: Vec<&str> = collect_type_names(form)
        .into_iter()
        .filter(|n| n.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
        .collect();
    if violations.is_empty() {
        Imperfect::Success(())
    } else {
        Imperfect::Partial((), violations.len() as f64)
    }
}

fn collect_type_names(form: &Form) -> Vec<&str> {
    let mut names = Vec::new();
    if form.kind == DeclKind::Type && !form.name.is_empty() {
        names.push(form.name.as_str());
    }
    for child in &form.children {
        names.extend(collect_type_names(child));
    }
    names
}
```

```
nix develop -c cargo test -- property_types_lowercase  # PASS
```

### 4.3 Red: `check_canonical_order`

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn property_canonical_order_passes_when_sorted() {
    let source = "in @prism\ntype color = red | blue\n";
    let form = parse_form(source).ok().unwrap();
    let verdict = run_property("canonical_order", &form);
    assert!(verdict.is_ok());
}

#[test]
fn property_canonical_order_fails_when_unsorted() {
    let source = "type color = red | blue\nin @prism\n";
    let form = parse_form(source).ok().unwrap();
    let verdict = run_property("canonical_order", &form);
    assert!(!verdict.is_ok(), "out-of-order should not be Success");
}
```

```
nix develop -c cargo test -- property_canonical_order  # FAIL
```

### 4.4 Green: Implement `check_canonical_order`

```rust
fn check_canonical_order(form: &Form) -> Imperfect<(), String, f64> {
    let sorted = canonical_sort(form);
    if form.children.iter().map(|c| (&c.kind, &c.name)).collect::<Vec<_>>()
        == sorted.children.iter().map(|c| (&c.kind, &c.name)).collect::<Vec<_>>()
    {
        Imperfect::Success(())
    } else {
        Imperfect::Partial((), 1.0)
    }
}
```

```
nix develop -c cargo test -- property_canonical_order  # PASS
```

### 4.5 Red: Wire properties into compile pipeline via `MirrorLoss`

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn compile_runs_spec_properties() {
    // A spec.mirror that requires types_lowercase
    let spec_source = "grammar @test {\n  requires types_lowercase\n}\n";
    let code_source = "type Color = red\n";
    // When compiling code_source against spec_source's requirements,
    // PropertyLoss should contain a verdict for types_lowercase
    let form = parse_form(code_source).ok().unwrap();
    let verdicts = run_spec_properties(&["types_lowercase"], &form);
    assert!(!verdicts.is_empty());
    assert_eq!(verdicts[0].property, "types_lowercase");
}
```

```
nix develop -c cargo test -- compile_runs_spec_properties  # FAIL
```

### 4.6 Green: `run_spec_properties`

```rust
pub fn run_spec_properties(property_names: &[&str], form: &Form) -> Vec<PropertyVerdict> {
    property_names
        .iter()
        .map(|name| PropertyVerdict {
            property: name.to_string(),
            verdict: run_property(name, form),
        })
        .collect()
}
```

```
nix develop -c cargo test -- compile_runs_spec_properties  # PASS
```

### 4.7 Red: Property verdicts flow into `MirrorLoss.holonomy()`

```rust
#[test]
fn property_loss_flows_into_holonomy() {
    let source = "type Color = red\n";
    let form = parse_form(source).ok().unwrap();
    let verdicts = run_spec_properties(&["types_lowercase"], &form);
    let loss = MirrorLoss {
        properties: PropertyLoss { verdicts },
        ..MirrorLoss::zero()
    };
    assert!(loss.holonomy() > 0.0, "property failure should contribute to holonomy");
}
```

This should already pass — `PropertyLoss::holonomy()` sums verdict losses. If it doesn't, wire it.

```
nix develop -c cargo test -- property_loss_flows_into_holonomy  # PASS
```

### 4.8 Commit

```
git add src/mirror_runtime.rs src/loss.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: property checks — types_lowercase, canonical_order, wired into MirrorLoss"
```

---

## Phase 5: CLI Integration

**Goal:** `mirror ci` runs all folds, reports holonomy. `--strict` flag. `--format json`. Error codes M1001-M9001.

### 5.1 Red: `mirror compile` output format

**File:** `src/cli.rs`

```rust
#[test]
fn cli_compile_success_output() {
    let result = Imperfect::<Form, MirrorRuntimeError, MirrorLoss>::Success(
        Form::new(DeclKind::Type, "test", vec![], vec![], vec![])
    );
    let output = format_compile_result(&result, OutputFormat::Human);
    assert!(output.contains("holonomy: 0.0"));
}

#[test]
fn cli_compile_partial_shows_loss() {
    let loss = MirrorLoss {
        parse: ParseLoss {
            unrecognized: vec![UnrecognizedDecl {
                keyword: "widget".into(),
                line: 5,
                content: "foo".into(),
            }],
        },
        ..MirrorLoss::zero()
    };
    let result = Imperfect::Partial(
        Form::new(DeclKind::Type, "test", vec![], vec![], vec![]),
        loss,
    );
    let output = format_compile_result(&result, OutputFormat::Human);
    assert!(output.contains("partial"));
    assert!(output.contains("M1001"));
    assert!(output.contains("widget"));
}
```

```
nix develop -c cargo test -- cli_compile  # FAIL: no function format_compile_result
```

### 5.2 Green: `format_compile_result`

**File:** `src/cli.rs`

```rust
pub enum OutputFormat {
    Human,
    Json,
}

pub fn format_compile_result(
    result: &Imperfect<Form, MirrorRuntimeError, MirrorLoss>,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Human => format_human(result),
        OutputFormat::Json => format_json(result),
    }
}
```

Implement `format_human` following the error surface spec's output structure. `format_json` serializes `MirrorLoss` fields.

```
nix develop -c cargo test -- cli_compile  # PASS
```

### 5.3 Red: `--strict` flag

**File:** `src/cli.rs`

```rust
#[test]
fn cli_strict_converts_partial_to_failure_exit() {
    let loss = MirrorLoss {
        parse: ParseLoss {
            unrecognized: vec![UnrecognizedDecl {
                keyword: "widget".into(),
                line: 1,
                content: "x".into(),
            }],
        },
        ..MirrorLoss::zero()
    };
    let result = Imperfect::Partial(
        Form::new(DeclKind::Type, "t", vec![], vec![], vec![]),
        loss,
    );
    let exit = compute_exit_code(&result, true); // strict=true
    assert_eq!(exit, 1, "strict mode: Partial → exit 1");
}

#[test]
fn cli_non_strict_partial_is_success() {
    let loss = MirrorLoss::zero();
    let result = Imperfect::Partial(
        Form::new(DeclKind::Type, "t", vec![], vec![], vec![]),
        loss,
    );
    let exit = compute_exit_code(&result, false);
    assert_eq!(exit, 0, "non-strict: Partial → exit 0");
}
```

```
nix develop -c cargo test -- cli_strict  # FAIL
```

### 5.4 Green: `compute_exit_code`

```rust
pub fn compute_exit_code<T, E, L: Loss>(result: &Imperfect<T, E, L>, strict: bool) -> i32 {
    match result {
        Imperfect::Success(_) => 0,
        Imperfect::Partial(_, _) => if strict { 1 } else { 0 },
        Imperfect::Failure(_, _) => 1,
    }
}
```

```
nix develop -c cargo test -- cli_strict  # PASS
```

### 5.5 Red: `mirror ci` runs all folds

**File:** `src/cli.rs`

```rust
#[test]
fn cli_ci_runs_all_folds() {
    let source = "type Color = red\n";
    let spec_properties = vec!["types_lowercase", "canonical_order"];
    let result = run_ci(source, &spec_properties);
    // CI result should have holonomy > 0 (uppercase type name)
    match &result {
        Imperfect::Partial(_, loss) => {
            assert!(loss.holonomy() > 0.0);
            assert!(!loss.properties.verdicts.is_empty());
        }
        other => panic!("expected Partial, got {:?}", other),
    }
}
```

```
nix develop -c cargo test -- cli_ci_runs_all_folds  # FAIL
```

### 5.6 Green: `run_ci`

```rust
pub fn run_ci(
    source: &str,
    spec_properties: &[&str],
) -> Imperfect<Form, MirrorRuntimeError, MirrorLoss> {
    let parsed = parse_form(source);
    match parsed {
        Imperfect::Success(form) | Imperfect::Partial(form, _) => {
            let verdicts = run_spec_properties(spec_properties, &form);
            let prop_loss = PropertyLoss { verdicts };
            let mut loss = match parsed {
                Imperfect::Partial(_, l) => l,
                _ => MirrorLoss::zero(),
            };
            loss.properties = prop_loss;
            if loss.is_zero() {
                Imperfect::Success(form)
            } else {
                Imperfect::Partial(form, loss)
            }
        }
        Imperfect::Failure(e, l) => Imperfect::Failure(e, l),
    }
}
```

Note: The above has a borrow issue because `parsed` is moved. The actual implementation will need to destructure more carefully.

```
nix develop -c cargo test -- cli_ci_runs_all_folds  # PASS
```

### 5.7 Red: `mirror explain M2001`

**File:** `src/cli.rs`

```rust
#[test]
fn cli_explain_m2001() {
    let explanation = explain_error("M2001");
    assert!(explanation.is_some());
    assert!(explanation.unwrap().contains("requires a name"));
}

#[test]
fn cli_explain_unknown() {
    let explanation = explain_error("M9999");
    assert!(explanation.is_none());
}
```

```
nix develop -c cargo test -- cli_explain  # FAIL
```

### 5.8 Green: `explain_error`

```rust
pub fn explain_error(code: &str) -> Option<&'static str> {
    match code {
        "M1001" => Some(
            "M1001 — Unrecognized Declaration\n\n\
             The parser encountered a keyword it does not recognize.\n\
             Mirror recognizes: grammar, type, template, action, property, in, out.\n\n\
             The unrecognized keyword is measured as loss (holonomy += 1.0 per occurrence).\n\
             If recognized declarations also exist, the result is Partial, not Failure."
        ),
        "M2001" => Some(
            "M2001 — Bare Keyword\n\n\
             A declaration keyword (`type`, `grammar`, `action`) appeared without a name.\n\n\
             Usage:\n  type color = red | blue\n  grammar @domain { ... }\n  action boot(identity) <= imperfect"
        ),
        "M2002" => Some(
            "M2002 — Bare `in`\n\n\
             `in` requires a target grammar reference.\n\n\
             Usage:\n  in @prism\n  in @code/rust"
        ),
        "M2003" => Some(
            "M2003 — Duplicate Type Name\n\n\
             Two type declarations in the same scope share a name and parameter signature.\n\
             Each type name must be unique within its scope.\n\
             Parameterized specializations (e.g., `type abstract(grammar)` and `type abstract(action)`) are allowed."
        ),
        "M2004" => Some(
            "M2004 — Double Operator\n\n\
             Two consecutive operators appeared where one was expected.\n\
             Example: `type x = = y` — the second `=` is unexpected."
        ),
        "M2005" => Some(
            "M2005 — Fold Operator in Wrong Context\n\n\
             The fold operator `<=` appeared in a type or template declaration.\n\
             `<=` is valid in `property` and `action` declarations.\n\
             For type composition, use `=`. For type inheritance, use `in @parent`."
        ),
        "M4001" => Some(
            "M4001 — Property Verdict\n\n\
             A property check returned a non-zero holonomy.\n\
             The property evaluated but found deviations from the declared invariant.\n\
             This is a measurement, not an error."
        ),
        "M9001" => Some(
            "M9001 — Boot Failure\n\n\
             The mirror boot sequence did not settle to a crystal.\n\
             Check: all boot files resolve, zero loss, zero holonomy, idempotent."
        ),
        _ => None,
    }
}
```

```
nix develop -c cargo test -- cli_explain  # PASS
```

### 5.9 Commit

```
git add src/cli.rs src/mirror_runtime.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: CLI surface — compile/ci/explain, --strict, --format json, error codes M1001-M9001"
```

---

## Phase 6: spec.mirror

**Goal:** The project config file that replaces all separate configs. Default properties. Default formatting. Deploy targets.

### 6.1 Red: Parse `spec.mirror` as project config

**File:** `src/mirror_runtime.rs`

```rust
#[test]
fn parse_spec_mirror() {
    let source = r#"
in @spec
in @config

grammar @mirror/project {
  default(target) = native
  default(visibility) = protected

  requires types_lowercase
  requires canonical_order
  invariant deterministic
  ensures always_halts

  boot = [
    00-prism,
    01-meta,
    10-mirror,
  ]

  deploy @cli
}
"#;
    let result = parse_form(source);
    let form = result.ok().unwrap();
    // Find the grammar child
    let grammar = form.children.iter()
        .find(|c| c.kind == DeclKind::Grammar)
        .unwrap();
    assert_eq!(grammar.name, "@mirror/project");
    // Find requires children
    let requires: Vec<_> = grammar.children.iter()
        .filter(|c| c.kind == DeclKind::Requires)
        .collect();
    assert_eq!(requires.len(), 2);
    assert_eq!(requires[0].name, "types_lowercase");
}
```

This should already pass — the current parser handles `grammar` blocks with `requires`, `invariant`, `ensures`, `default`, and `boot`. Verify.

```
nix develop -c cargo test -- parse_spec_mirror  # should PASS
```

### 6.2 Red: Extract spec properties from parsed spec

**File:** `src/mirror_runtime.rs` (or new `src/spec.rs`)

```rust
#[test]
fn extract_spec_properties() {
    let spec = parse_form(SPEC_MIRROR_SOURCE).ok().unwrap();
    let props = extract_properties(&spec);
    assert!(props.requires.contains(&"types_lowercase".to_string()));
    assert!(props.invariants.contains(&"deterministic".to_string()));
    assert!(props.ensures.contains(&"always_halts".to_string()));
}
```

```
nix develop -c cargo test -- extract_spec_properties  # FAIL
```

### 6.3 Green: `SpecProperties` struct and `extract_properties`

```rust
pub struct SpecProperties {
    pub requires: Vec<String>,
    pub invariants: Vec<String>,
    pub ensures: Vec<String>,
    pub defaults: Vec<(String, String)>,
}

pub fn extract_properties(form: &Form) -> SpecProperties {
    let mut props = SpecProperties {
        requires: Vec::new(),
        invariants: Vec::new(),
        ensures: Vec::new(),
        defaults: Vec::new(),
    };
    collect_properties(form, &mut props);
    props
}

fn collect_properties(form: &Form, props: &mut SpecProperties) {
    match form.kind {
        DeclKind::Requires => props.requires.push(form.name.clone()),
        DeclKind::Invariant => props.invariants.push(form.name.clone()),
        DeclKind::Ensures => props.ensures.push(form.name.clone()),
        DeclKind::Default => {
            if !form.params.is_empty() && !form.variants.is_empty() {
                props.defaults.push((form.params[0].clone(), form.variants[0].clone()));
            }
        }
        _ => {}
    }
    for child in &form.children {
        collect_properties(child, props);
    }
}
```

```
nix develop -c cargo test -- extract_spec_properties  # PASS
```

### 6.4 Red: `mirror ci` reads `spec.mirror` and runs its properties

**File:** `src/cli.rs`

```rust
#[test]
fn ci_uses_spec_mirror() {
    let spec_source = "grammar @test {\n  requires types_lowercase\n}\n";
    let code_source = "type Color = red\n";
    let spec = parse_form(spec_source).ok().unwrap();
    let props = extract_properties(&spec);
    let prop_names: Vec<&str> = props.requires.iter().map(|s| s.as_str()).collect();
    let result = run_ci(code_source, &prop_names);
    assert!(matches!(result, Imperfect::Partial(_, _)));
}
```

```
nix develop -c cargo test -- ci_uses_spec_mirror  # PASS (should work from Phase 5)
```

### 6.5 Red: Default properties when no spec.mirror exists

```rust
#[test]
fn default_properties_without_spec() {
    let defaults = default_spec_properties();
    // At minimum: types_lowercase, canonical_order
    assert!(defaults.requires.contains(&"types_lowercase".to_string()));
    assert!(defaults.requires.contains(&"canonical_order".to_string()));
}
```

```
nix develop -c cargo test -- default_properties_without_spec  # FAIL
```

### 6.6 Green: `default_spec_properties`

```rust
pub fn default_spec_properties() -> SpecProperties {
    SpecProperties {
        requires: vec![
            "types_lowercase".to_string(),
            "canonical_order".to_string(),
        ],
        invariants: vec![],
        ensures: vec![],
        defaults: vec![],
    }
}
```

```
nix develop -c cargo test -- default_properties_without_spec  # PASS
```

### 6.7 Commit

```
git add src/mirror_runtime.rs src/cli.rs
git commit --author="Mara <mara@systemic.engineer>" -m "feat: spec.mirror — project config surface, property extraction, defaults"
```

---

## The Full CLI Surface (after all phases)

```
mirror compile .           -- compile fold. Success/Partial/Failure. holonomy reported.
mirror fmt .               -- template fold. iso. OID doesn't change. Canonical order.
mirror fmt . --check       -- check mode. Exit 1 if any file would change.
mirror ci .                -- all folds. holonomy reported. Properties from spec.mirror.
mirror ci . --strict       -- Partial becomes Failure. Exit 1 on any holonomy > 0.
mirror ci . --format json  -- machine-readable output.
mirror explain M2001       -- error code tutorial.
```

## Test Count Estimate

| Phase | New Tests | Running Total |
|-------|-----------|---------------|
| Current baseline | 499 | 499 |
| Phase 1: Operators | ~12 | ~511 |
| Phase 2: Type Surface | ~8 | ~519 |
| Phase 3: Formatter | ~8 | ~527 |
| Phase 4: Properties | ~8 | ~535 |
| Phase 5: CLI | ~10 | ~545 |
| Phase 6: spec.mirror | ~6 | ~551 |

## Dependencies Between Phases

```
Phase 1 ──→ Phase 2 ──→ Phase 4
                          ↓
Phase 3 ─────────────→ Phase 5 ←── Phase 6
```

- Phase 1 is required by Phase 2 (`!=` operator)
- Phase 2 is required by Phase 4 (type constraints inform properties)
- Phase 3 is independent (can be done in parallel with Phase 2)
- Phase 5 depends on Phase 3, 4 (needs fmt + properties for `ci`)
- Phase 6 depends on Phase 5 (needs CLI to read spec.mirror)

Each phase is independently shippable. Each phase has its own red tests. Each commit is green.
