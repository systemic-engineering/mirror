# The Property Layer + Error Surface

*2026-05-16. Reed. Spec.*

Status: **Red**

Depends on: @prism (five operations), @error (diagnostic types), @imperfect
(three-state container), @mirror/check (static error extraction),
@mirror/interpreter (execution loop), boot grammars (01a-error, 02-epistemologic,
05-property)

---

## 1. What Exists Today

### Types (declared in boot grammars)

**`boot/01a-error.mirror`** declares the full error type system:

| Grammar | Type | Fields |
|---------|------|--------|
| `@loss` | `loss` | `{ bits: f64, source: ref, measurement: measurement }` |
| `@loss` | `measurement` | `shannon \| fiedler \| cheeger \| entropy \| mixing` |
| `@error` | `error` | `{ grammar: ref, name: ref, message: nl, location: location, loss: loss }` |
| `@error` | `location` | `{ file: ref, line: u64, column: u64 }` |
| `@error` | `severity` | `fatal \| warning \| info` |
| `@error` | `diagnostic` | `{ error: error, severity: severity, suggestion: nl }` |
| `@imperfect` | `imperfect(value, loss, error)` | `{ value: value, loss: loss, errors: [error] }` |

**`boot/05-property.mirror`** declares (outdated syntax):

| Type | Shape |
|------|-------|
| `verdict` | `imperfect(declaration, property_error, property_loss)` |
| `property_error` | `{ observation: declaration, property: text, context: [declaration] }` |
| `property_loss` | `{ deviation: f64, frequency: imperfect }` |
| `effect_pattern` | `effect(a, b)` |

**`boot/02-epistemologic.mirror`** declares:

| Element | Kind |
|---------|------|
| `literal(declaration) -> verdict` | property (Refract) |
| `override_ratio(grammar) -> loss` | property (Refract) |
| `and(a, b)` | type (Split) |
| `or(a, b)` | type (Split) |
| `but(a, b)` | type (Split) |

**`boot/std/properties.mirror`** declares templates and properties using outdated
`template`/`property` keywords (not recognized by current grammar mappings):

- Templates: `types_lowercase`, `no_cycles`, `unique_variants`,
  `every_type_reachable`, `no_dead_variants`, `dual_partition`,
  `canonical_order`, `kintsugi`
- Properties: `idempotent`, `deterministic`, `pure`, `always_halts`,
  `consent`, `witnessed`, `human_intervention`, `sanitize`, `escape`,
  `consent_boundary`, `audit_trail`, `deploy_gate`, `classify`

### Grammars that declare error-producing operations

**`boot/std/mirror/resolve.mirror`** -- `resolve(ast) -> imperfect(ast) { \ }`
**`boot/std/mirror/check.mirror`** -- `check(file) -> imperfect { \ }`,
  `errors(ast) -> [diagnostic] { \ }`, `loss(ast) -> loss { \ }`
**`boot/std/mirror/interpreter.mirror`** -- `interpret(ast) -> imperfect { \ }`,
  `resolve_hole`, `execute_io`, `walk` -- all return imperfect
**`boot/std/mirror/runtime.mirror`** -- `compile(file) -> imperfect { ... }`,
  `run(command, args) -> imperfect { \ }`

### The Rust substrate (what ACTUALLY executes)

The 7 AST variants in `src/mirror_ast.rs`:
`Focus`, `Project`, `Split`, `Zoom`, `Refract`, `In`, `Out`

The interpreter in `src/interpreter.rs`:
- `io_exec(command, args, stdin)` -- the only door to reality
- Five prism operations: `focus`, `project`, `split`, `zoom`, `refract`
- `dispatch(command, args)` -- CLI command routing
- `compile_cached(source, grammar)` -- tokenize with git crystal cache

The tokenizer in `src/tokenize.rs`:
- `load_grammar(path)` -- extracts keyword -> AstKind mappings from a grammar block
- `tokenize(source, grammar)` -- single-pass O(n) scanner, produces MirrorAST
- `canonical_form(ast)` -- kintsugi render to canonical mirror form

### Current compiler behavior (actual test results)

**`echo "in @nonexistent" | mirror compile /dev/stdin`**
Output: `8141d8d12b4c39fb64ab601635bf062d6ef99d0f3d3165646ae2bb7bc7d1d883`
Behavior: Produces an OID. No error. No warning. The `In` node is created but never resolved.

**`echo "type color = red | red" | mirror compile /dev/stdin`**
Output: `8141d8d12b4c39fb64ab601635bf062d6ef99d0f3d3165646ae2bb7bc7d1d883`
Behavior: Produces an OID. No error. Duplicate variant not detected.
(Same OID as above because the tokenizer does not parse variants -- it only tracks
the keyword `type` and the name `color`; the rest of the line is ignored.)

**`echo "broken {{{{ syntax" | mirror compile /dev/stdin`**
Output: `8141d8d12b4c39fb64ab601635bf062d6ef99d0f3d3165646ae2bb7bc7d1d883`
Behavior: Produces an OID. No error. The tokenizer skips unrecognized tokens
and produces an empty root Focus. The hash is deterministic for "no children."

**`mirror compile nonexistent_file.mirror`**
Output: `cannot read file nonexistent_file.mirror: No such file or directory (os error 2)`
Behavior: This is the ONLY error the compiler currently produces. It comes from
`std::fs::read_to_string` in `dispatch_compile`, printed to stderr, exit code 1.

### Summary: the gap

The compiler has:
- Types for errors, diagnostics, loss, verdicts -- declared in boot grammars
- Grammars that describe error-producing operations (@mirror/check, @mirror/resolve)
- A tokenizer that produces AST from source
- Zero property checking
- Zero import resolution
- Zero error/warning/diagnostic output from grammar violations
- The ONLY runtime error is "file not found" from the Rust `fs` layer

The gap is total. The types exist. The declarations exist. Nothing executes.
The compiler tokenizes and content-addresses. It does not check, resolve, or verify.

---

## 2. What's Missing

### Between declaration and execution

1. **No import resolution.** `In` nodes are created but never checked against
   available grammars. `in @nonexistent` silently produces an AST node.

2. **No type checking.** The tokenizer does not parse type bodies. `type color = red | red`
   is treated as `type color` with the rest of the line ignored. No variant extraction,
   no duplicate detection.

3. **No property evaluation.** Properties declared with `property` keyword are parsed
   as Refract nodes (or not parsed at all -- `boot/std/properties.mirror` uses
   `template`/`property` keywords not in the grammar mappings). No property is ever
   executed.

4. **No diagnostic rendering.** The `diagnostic` type exists in `@error` but no code
   path constructs a `diagnostic` value and renders it to the user.

5. **No loss measurement.** The `loss` type exists but no Shannon/Fiedler/Cheeger
   measurement is ever computed during compilation.

6. **No imperfect threading.** The `imperfect` type exists but `compile` returns
   an OID (the content address), not an `imperfect(ast)`.

### What blocks each missing piece

| Missing piece | Blocked by |
|--------------|------------|
| Import resolution | No grammar registry; no way to look up `@nonexistent` |
| Type body parsing | Tokenizer intentionally skips body content for non-container nodes |
| Property evaluation | No interpreter loop; `\ ` holes never resolved |
| Diagnostic rendering | No code path produces diagnostics |
| Loss measurement | No eigenvalue computation in the compile path |
| Imperfect threading | Return type of `compile` is OID, not imperfect |

---

## 3. @epistemologic/property Design

### The verdict type

```mirror
in @prism
in @error
in @loss

grammar @epistemologic/property {
  # verdict: the outcome of a property check.
  # NOT imperfect. Domain-specific.
  # pass = holds. fail = doesn't hold. partial = spectral, not binary.
  type verdict = pass | fail(diagnostic) | partial(f64, [diagnostic])

  # a property check: takes an AST node, returns verdict.
  # every concrete check is a named lambda.
  type check = (ast) -> verdict

  # run all registered checks on an AST.
  # returns one verdict per check, with the check name attached.
  type report = { check: ref, verdict: verdict }

  run_checks(ast, [check]) -> [report] { \ }
}

out verdict
out check
out report
out run_checks
out @epistemologic/property
```

The three verdict variants:
- `pass` -- the property holds. No diagnostics.
- `fail(diagnostic)` -- the property does not hold. The diagnostic carries
  location, message, loss, and suggestion.
- `partial(0.97, [diagnostic])` -- the property partially holds. The f64 is
  the confidence (0.0 = fail, 1.0 = pass). The diagnostics explain what
  reduced confidence. Spectral, not binary.

### Concrete property checks

Each check is a sub-grammar under `@epistemologic/property/`. Each declares
a single lambda returning verdict.

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/duplicate_variant {
  # detect: same variant name appears twice in a type declaration.
  # trigger: type color = red | blue | red
  # verdict: fail with diagnostic pointing at the duplicate.
  duplicate_variant(ast) -> verdict { \ }
}

out duplicate_variant
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/unresolved_import {
  # detect: In node referencing a grammar not available in the boot registry.
  # trigger: in @nonexistent
  # verdict: fail with diagnostic naming the missing grammar.
  unresolved_import(ast) -> verdict { \ }
}

out unresolved_import
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/unused_declaration {
  # detect: a type or action declared but never referenced by any other node.
  # trigger: type phantom = ghost (never used in any action or other type)
  # verdict: partial(0.8, [...]) -- unused is not wrong, just loss.
  unused_declaration(ast) -> verdict { \ }
}

out unused_declaration
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/arity_mismatch {
  # detect: an action or type used with wrong number of arguments.
  # trigger: type pair(a, b) used as pair(x) elsewhere.
  # verdict: fail with diagnostic showing expected vs actual arity.
  arity_mismatch(ast) -> verdict { \ }
}

out arity_mismatch
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/missing_export {
  # detect: a type or action declared inside a grammar but not in any Out node.
  # trigger: grammar @x { type foo = bar } (no "out foo")
  # verdict: partial(0.9, [...]) -- internal types are valid, just not visible.
  missing_export(ast) -> verdict { \ }
}

out missing_export
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/unreachable_type {
  # detect: a type not reachable from any action's parameter or return type.
  # trigger: type orphan = lost (no action uses it, no other type references it)
  # verdict: partial(0.7, [...]) -- unreachable means potential dead code.
  unreachable_type(ast) -> verdict { \ }
}

out unreachable_type
```

```mirror
in @prism
in @epistemologic/property

grammar @epistemologic/property/circular_import {
  # detect: In cycle where A imports B imports C imports A.
  # trigger: grammar @a { in @b } grammar @b { in @a }
  # verdict: fail with diagnostic showing the cycle path.
  circular_import(ast) -> verdict { \ }
}

out circular_import
```

### Sub-grammar structure

```
@epistemologic/property              root: verdict type, check runner
@epistemologic/property/duplicate_variant
@epistemologic/property/unresolved_import
@epistemologic/property/unused_declaration
@epistemologic/property/arity_mismatch
@epistemologic/property/missing_export
@epistemologic/property/unreachable_type
@epistemologic/property/circular_import
```

Each sub-grammar:
- Imports `@epistemologic/property` (gets `verdict` type)
- Declares exactly one lambda: `check_name(ast) -> verdict { \ }`
- Exports the check name

---

## 4. Error Output Format

### The format

When the compiler finds a violation, the user sees:

```
error[E001]: duplicate variant
  --> boot/std/color.mirror:3:18
   |
 3 | type color = red | blue | red
   |                           ^^^ 'red' already declared at column 14
   |
   = loss: 0.15 (shannon)
   = suggestion: remove duplicate or rename
```

### Format anatomy

```
{severity}[{code}]: {message}
  --> {file}:{line}:{column}
   |
 {line} | {source_line}
   |     {carets} {label}
   |
   = loss: {bits} ({measurement})
   = suggestion: {suggestion_text}
```

Where:
- `{severity}` = `error` | `warning` | `info` (from `@error.severity`)
- `{code}` = `E` + 3 digits for errors, `W` + 3 digits for warnings
  (mirrors the M-codes from error-surface-spec but simplified for property checks)
- `{message}` = natural language (nl) -- the short description
- `{file}:{line}:{column}` = from `@error.location`
- `{source_line}` = the actual source text at that line
- `{carets}` = `^^^` under the relevant span
- `{label}` = context for the caret span
- `{loss}` = `bits` value + `measurement` kind from `@loss`
- `{suggestion}` = natural language (nl) -- what could change

### How the format is generated from grammar types

The `diagnostic` type in `@error` carries everything:

```
diagnostic = {
  error: {
    grammar: ref,       # which grammar produced this (e.g., @epistemologic/property/duplicate_variant)
    name: ref,          # the check name (e.g., duplicate_variant)
    message: nl,        # "duplicate variant"
    location: {
      file: ref,        # "boot/std/color.mirror"
      line: u64,        # 3
      column: u64,      # 18
    },
    loss: {
      bits: f64,        # 0.15
      source: ref,      # the AST node OID
      measurement: measurement, # shannon
    },
  },
  severity: severity,   # fatal | warning | info
  suggestion: nl,       # "remove duplicate or rename"
}
```

The renderer walks the diagnostic struct field by field:
1. `severity` + `error.name` + `error.message` -> first line
2. `error.location` -> `-->` line
3. Read source file at `location.file`, extract line at `location.line` -> source display
4. `location.column` + span width -> caret placement
5. `error.loss` -> `= loss:` line
6. `suggestion` -> `= suggestion:` line

### Verdict to diagnostic conversion

When a property check returns `fail(diagnostic)`, the diagnostic is ready.
When it returns `partial(confidence, [diagnostic])`, each diagnostic is rendered
with severity = `warning` (partial means it mostly holds but has concerns).
When it returns `pass`, nothing is rendered.

### Multi-error output

Multiple diagnostics are separated by blank lines. A summary follows:

```
error[E001]: duplicate variant
  --> boot/std/color.mirror:3:18
   ...

warning[W002]: unused declaration
  --> boot/std/color.mirror:7:1
   ...

summary: 1 error, 1 warning
  total loss: 0.45 (shannon)
  verdict: partial(0.85)
```

---

## 5. The Boot Order

### Where @epistemologic/property fits

Current boot order (numbered files):

```
00-prism.mirror          # five operations, Beam<T>, ShannonLoss
00a-sigil.mirror         # sigils
01-meta.mirror           # type system foundations
01a-error.mirror         # @loss, @error, @imperfect types      <-- error types HERE
01b-nl.mirror            # natural language
02-actor.mirror          # actor model
02-epistemologic.mirror  # @epistemologic root (literal, and/or/but)
02a-io.mirror            # @io
02b-runtime.mirror       # runtime types
03-shatter.mirror        # serialization
04-code.mirror           # code grammars
05-property.mirror       # @property (OUTDATED, needs rewrite)  <-- old property HERE
```

The property layer fits AFTER `01a-error` (needs `diagnostic`, `loss`, `imperfect`)
and AFTER `02-epistemologic` (needs `verdict` concept from the epistemologic root).

**Proposed insertion:**

```
02-epistemologic.mirror       # root: literal, override_ratio, and/or/but
02c-property.mirror           # NEW: @epistemologic/property -- verdict type, check runner
```

The sub-grammars (`@epistemologic/property/*`) go in `boot/std/property/`:

```
boot/std/property/
  duplicate_variant.mirror
  unresolved_import.mirror
  unused_declaration.mirror
  arity_mismatch.mirror
  missing_export.mirror
  unreachable_type.mirror
  circular_import.mirror
```

### Dependency chain

```
@prism
  -> @error (needs ref, nl from prism concepts)
    -> @imperfect (needs error, loss)
      -> @epistemologic (needs property concept, loss)
        -> @epistemologic/property (needs diagnostic, loss, imperfect)
          -> @epistemologic/property/* (each needs verdict from parent)
            -> @mirror/check (uses property checks to produce diagnostics)
              -> @mirror/runtime (full pipeline including checks)
```

---

## 6. Implementation Path

### Tick 0: Type body parsing (pure tokenizer extension)

**What:** Extend the tokenizer to extract variant names from type declarations.
Currently `type color = red | blue` produces a `SplitNode` with name `"color"`
but `variants: vec![]` and `body: None`. The `= red | blue` is skipped.

**Why first:** Without parsed type bodies, no property check can inspect variants.
The tokenizer already has `SplitNode.variants` and `SplitNode.body` -- the fields
exist, they just aren't populated.

**Needs:** Rust change to `scan_items()` in `tokenize.rs` -- parse the `= ... | ...`
portion of type declarations into `TypeBody::Enum` or `TypeBody::Struct`.

**Status:** This is the one tick that requires a Rust substrate change.

### Tick 1: Import registry (grammar resolution)

**What:** Build a registry of available grammars by scanning the boot directory.
When an `In` node references `@foo`, check if `@foo` is in the registry.

**Why:** `unresolved_import` needs to know what's available. The registry is a
`HashMap<GrammarRef, Oid>` built from all loaded grammars.

**Needs:** Rust change to build the registry during `craft`. Then pass it to a
check function.

### Tick 2: First property check -- duplicate_variant (pure mirror possible)

**What:** Walk the AST. For each `SplitNode`, check if any variant appears twice.
Return `fail(diagnostic)` if found, `pass` otherwise.

**Why:** Simplest possible property check. Requires only the parsed AST (from tick 0).
No import resolution needed. No external state.

**Needs:** Either:
- (a) A Rust function that walks the AST and checks variants (substrate extension), or
- (b) The interpreter loop executing the grammar's `\ ` hole via Fate (requires
  interpreter work)

**Recommendation:** (a) first -- a Rust function that the `@mirror/check` dispatch
calls. This gets errors visible to users immediately. Replace with (b) when the
interpreter loop lands.

### Tick 3: Diagnostic rendering

**What:** When a property check returns `fail(diagnostic)` or `partial(_, [diagnostic])`,
render it to stderr in the format specified in section 4.

**Needs:** A Rust function that takes a `diagnostic` (or its equivalent Rust struct)
and formats it with source spans, carets, loss, and suggestions.

### Tick 4: Wire into compile pipeline

**What:** After tokenization, run property checks. Collect diagnostics. If any
check returns `fail`, exit with error. If any returns `partial`, print warnings
but continue. Output the OID plus diagnostics.

**Needs:** Modify `dispatch_compile` to call property checks after tokenization,
before printing the OID.

### Tick 5: Additional property checks

**What:** Implement `unresolved_import` (needs tick 1 registry), `missing_export`,
`unreachable_type`, `unused_declaration`. Each follows the same pattern: walk AST,
check condition, return verdict.

### Tick 6: Loss measurement

**What:** Compute Shannon loss for each diagnostic. The loss measures how much
information was destroyed by the violation. Duplicate variant = redundant information
= measurable in bits.

**Needs:** Shannon entropy calculation (already conceptually defined in `@loss`).

### Tick 7: Imperfect return type

**What:** Change `compile` to return `imperfect(ast)` instead of just OID.
The imperfect carries the AST (or crystal OID) plus all accumulated diagnostics
and loss.

**Needs:** Return type change in `dispatch_compile`. JSON output mode for
machine-readable diagnostics.

### Which ticks are pure mirror vs need interpreter

| Tick | Pure mirror? | Needs Rust? | Notes |
|------|-------------|-------------|-------|
| 0 | No | Yes | Tokenizer must parse type bodies |
| 1 | No | Yes | Registry is runtime state |
| 2 | Partially | Yes (initially) | Walk + check can be Rust; moves to mirror when interpreter lands |
| 3 | No | Yes | Formatting requires string manipulation + source file reading |
| 4 | No | Yes | Pipeline modification is in dispatch |
| 5 | Partially | Yes (initially) | Same pattern as tick 2 |
| 6 | Could be mirror | Yes (initially) | Entropy calculation is math, could be grammar |
| 7 | No | Yes | Return type change is structural |

The honest answer: all initial implementation is Rust substrate. The grammars
declare the design. The interpreter will eventually execute them. But the first
errors that users see will come from Rust functions called by `dispatch_compile`.

---

## 7. The First Error

The simplest possible error to implement end-to-end:

### What triggers it

```mirror
type color = red | blue | red
```

A type declaration with a duplicate variant name.

### Why this one

1. Requires only ONE substrate change (tick 0: parse type bodies)
2. No import resolution needed
3. No external state needed
4. The check is trivial: walk variants, detect duplicates
5. The diagnostic is clear and unambiguous
6. It demonstrates the full flow: parse -> check -> render

### How it flows through the types

1. **Tokenizer** parses `type color = red | blue | red` into:
   ```
   SplitNode {
     name: Identifier("color"),
     variants: [],
     params: [],
     body: Some(TypeBody::Enum([
       Identifier("red"),
       Identifier("blue"),
       Identifier("red"),   // <-- duplicate
     ])),
     children: [],
   }
   ```

2. **Property check** (`duplicate_variant`) walks the AST:
   - For each `SplitNode` with `TypeBody::Enum(variants)`:
   - Build a set of seen names
   - If a name appears twice, construct a `diagnostic`:
     ```
     diagnostic {
       error: {
         grammar: @epistemologic/property/duplicate_variant,
         name: duplicate_variant,
         message: "duplicate variant 'red'",
         location: { file: "input.mirror", line: 1, column: 28 },
         loss: { bits: 0.15, source: <oid of the SplitNode>, measurement: shannon },
       },
       severity: fatal,
       suggestion: "remove duplicate or rename to distinguish",
     }
     ```

3. **Verdict** returned: `fail(diagnostic)`

4. **Renderer** formats the diagnostic:
   ```
   error[E001]: duplicate variant 'red'
     --> input.mirror:1:28
      |
    1 | type color = red | blue | red
      |                           ^^^ 'red' already declared at column 14
      |
      = loss: 0.15 (shannon)
      = suggestion: remove duplicate or rename to distinguish
   ```

5. **Exit** with code 1. No OID printed (the grammar is malformed).

### What the user sees

Before (current behavior):
```
$ echo "type color = red | blue | red" | mirror compile /dev/stdin
8141d8d12b4c39fb64ab601635bf062d6ef99d0f3d3165646ae2bb7bc7d1d883
```

After (with property layer):
```
$ echo "type color = red | blue | red" | mirror compile /dev/stdin
error[E001]: duplicate variant 'red'
  --> /dev/stdin:1:28
   |
 1 | type color = red | blue | red
   |                           ^^^ 'red' already declared at column 14
   |
   = loss: 0.15 (shannon)
   = suggestion: remove duplicate or rename to distinguish
```

### The red test

```rust
#[test]
fn error_duplicate_variant() {
    let grammar = load_grammar("boot/std/mirror/grammar.mirror").unwrap();
    let source = "type color = red | blue | red";
    let ast = tokenize(source, &grammar);
    let verdicts = check_properties(&ast);
    assert!(verdicts.iter().any(|v| matches!(v, Verdict::Fail(_))));

    let fails: Vec<_> = verdicts.iter()
        .filter_map(|v| match v { Verdict::Fail(d) => Some(d), _ => None })
        .collect();
    assert_eq!(fails.len(), 1);
    assert!(fails[0].error.message.contains("duplicate"));
    assert!(fails[0].error.message.contains("red"));
}
```

---

## 8. Error Code Allocation

Following the M-code scheme from `error-surface-spec.md`, property errors live
in the M4xxx range. The new E-codes for property checks:

| Code | Property | Severity | Message pattern |
|------|----------|----------|-----------------|
| E001 | `duplicate_variant` | error | "duplicate variant '{name}'" |
| E002 | `unresolved_import` | error | "unresolved import '@{name}'" |
| E003 | `circular_import` | error | "circular import: {path}" |
| E004 | `arity_mismatch` | error | "arity mismatch: expected {n}, got {m}" |
| W001 | `unused_declaration` | warning | "unused declaration '{name}'" |
| W002 | `missing_export` | warning | "declared but not exported: '{name}'" |
| W003 | `unreachable_type` | warning | "unreachable type '{name}'" |

Errors (E-codes) produce `fail(diagnostic)`. Exit code 1.
Warnings (W-codes) produce `partial(confidence, [diagnostic])`. Exit code 0.

---

## 9. Relationship to Existing Specs

**`error-surface-spec.md`** -- defines the M-code system, the three-state output
(Success/Partial/Failure), holonomy measurement, and CLI format. This spec extends
it with concrete property checks that produce those diagnostics.

**`epistemologic-grammar.md`** -- defines the @epistemologic hierarchy and the
`literal` property. This spec implements the property LAYER that `literal` and all
domain-specific checks will run on. The hierarchy is the tree; this spec is the root
that grows it.

**`io-socket-compiler.md`** -- defines the tokenizer + spec architecture. This spec
adds the CHECK phase between tokenization and crystal emission. The pipeline becomes:
tokenize -> check (property layer) -> crystal.

---

## 10. The Equation

```
compile = tokenize |> check |> crystal

tokenize: source -> ast              (exists, works)
check:    ast -> imperfect(ast)      (this spec)
crystal:  imperfect(ast) -> oid      (exists, ignores imperfect)

check = run_checks(ast, registered_properties)
      = [verdict]
      = pass | fail(diagnostic) | partial(f64, [diagnostic])

The property layer is the middle term.
Without it, the compiler is a tokenizer that content-addresses.
With it, the compiler measures.
```

`e^(n+1) < e^(n)`. The first error is E001. The system learns.
