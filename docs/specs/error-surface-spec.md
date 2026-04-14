# Mirror Compiler Error Surface Spec

**Author:** Mara
**Date:** 2026-04-14
**Status:** Draft

---

## 1. Research Summary: How Compilers Present Errors

### Rust (rustc)

**Structure:** Error code (E0xxx) + short summary + annotated source span + "help:" suggestions. Multi-span diagnostics show cause and effect across files.

```
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let x: i32 = "hello";
  |            ---   ^^^^^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this
```

**Structured data:** Every diagnostic is a `Diagnostic` struct with level, code, message, spans (each with file/line/col/label), children (sub-diagnostics), and suggestions. JSON output via `--error-format=json`. This is a typed error system — errors carry structured data, not just strings.

**Partial success:** Warnings coexist with errors. The compiler continues past non-fatal errors to report multiple issues. `#[allow(...)]` suppresses. Lints are a separate tier from hard errors.

**`--explain E0308`:** Prints a full tutorial for that error code. The error catalog is part of the compiler, not the docs.

**What mirror can learn:** The span annotation model is excellent. But rustc's errors assume a single correct answer exists. Mirror's errors don't — they measure what was lost, not what was wrong. The `--explain` pattern maps well to `mirror explain M0001`.

### Elm

**Structure:** No error codes. Full English paragraphs. Conversational tone. The error message IS a tutorial.

```
-- TYPE MISMATCH -------- src/Main.elm

The 2nd argument to `add` is not what I expect:

6|   add 1 "hello"
             ^^^^^^^
This argument is a string of type:

    String

But `add` needs the 2nd argument to be:

    number

Hint: Try using String.toInt to convert it?
```

**Structured data:** Errors are structured internally (`Report` type) but presented as prose. No machine-readable format.

**Partial success:** Elm does not produce partial output. It either compiles or it doesn't. No warnings in production — the language design eliminates the need.

**What mirror can learn:** The conversational tone is powerful but only works because Elm's type system is simple enough for single-answer errors. Mirror's errors are measurements, not corrections. The prose style doesn't fit — but the *readability standard* does. Every error should be legible to someone who doesn't know compiler internals.

### Gleam

**Structure:** Named error (not coded), source span, context. Influenced by Elm's friendliness but more structured.

```
error: Unknown variable
  ┌─ src/main.gleam:2:3
  │
2 │   wibble
  │   ^^^^^^ Did you mean `wobble`?
```

**Structured data:** Errors are typed enums in the compiler. Machine-readable output planned. Each error variant carries the data needed to render it.

**Partial success:** Like Elm, Gleam does not emit partial results. Compilation either succeeds or fails.

**What mirror can learn:** Gleam's typed error enums are the right model for internal representation. But Gleam's binary success/failure is exactly what mirror rejects. Mirror has three states. Gleam's architecture minus its binary assumption.

### ReasonML / OCaml

**Structure:** Type errors show expected vs. actual, sometimes with unification traces. The "this expression has type X but was expected of type Y" pattern.

```
Error: This expression has type string
       but an expression was expected of type int
```

**Structured data:** Errors carry location, expected type, actual type. The unification trace can be extracted. Recent OCaml versions improved formatting significantly.

**Partial success:** OCaml produces `.cmi` files even when later compilation fails, enabling partial compilation of dependency chains.

**What mirror can learn:** The partial compilation of dependency chains is relevant. Mirror's boot process already does this — files that resolve get crystals, files that don't get recorded as failed. OCaml's approach to partial `.cmi` emission maps to mirror's partial boot.

### Zig

**Structure:** Compile errors are values. `@compileError("message")` is a built-in that makes error reporting part of the language, not a separate system.

```
./example.zig:4:5: error: expected type 'u32', found 'i32'
./example.zig:2:36: note: called from here
```

**Structured data:** Errors carry source location and notes (secondary locations). The `@compileError` mechanism means user-defined compile errors have the same structure as built-in ones.

**Partial success:** Zig's lazy evaluation means unused code paths don't produce errors. This is a form of partial success: the compiler only measures what you observe.

**What mirror can learn:** `@compileError` as a language-level primitive is a pattern mirror should study. In mirror, the equivalent is that grammars can define properties that produce `Imperfect` — the grammar author defines what "error" means for their domain. Zig proved that compile errors can be first-class values. Mirror's `Imperfect` proves they can be first-class *measurements*.

### Lean 4

**Structure:** Proof state errors show the goal, the hypotheses, and what failed. Tactic failures show the proof state at the point of failure.

```
tactic 'exact' failed, type mismatch
  1 + 1
has type
  Nat
but is expected to have type
  1 + 1 = 2
```

**Structured data:** Fully structured. The proof state is a data structure (goals, hypotheses, metavariables). Errors carry the full context needed to understand what went wrong.

**Partial success:** Lean has "sorry" — an axiom that discharges any goal, marking it as incomplete. The proof compiles with sorrys, but the compiler tracks them. This is Lean's version of `Partial`: the proof exists, but with measured loss (the sorry count).

**What mirror can learn:** `sorry` IS `Partial`. Lean discovered the same three-state structure independently: a proof is either complete (Success), complete-with-sorrys (Partial), or stuck (Failure). The sorry count is a holonomy metric. Mirror should cite this precedent.

---

## 2. Mirror's Error Model: How Imperfect Differs

Every compiler above treats errors as obstacles. Something went wrong, here's what, fix it. Even Lean's sorry model treats incompleteness as a gap to fill.

Mirror treats errors as measurements.

### The Three States

```
Imperfect<T, E, L: Loss>

Success(T)       — zero loss. The transformation preserved everything.
Partial(T, L)    — a value exists, but something was lost getting here.
Failure(E, L)    — no value survived, but the cost is measured.
```

This is not `Result<T, E>` with extra steps. The middle state — `Partial` — is the innovation. Most real compilations are partial. The grammar parsed, but an unrecognized keyword was dropped. The types resolved, but an import couldn't be found. The crystal formed, but some properties didn't hold.

Collapsing this to `Ok`/`Err` destroys information. Every compiler above does this. Mirror doesn't.

### Loss as Measurement

`MirrorLoss` is not an error message. It is a structured measurement across four folds:

| Fold | Type | What it measures |
|------|------|-----------------|
| Parse | `ParseLoss` | Unrecognized declarations — information in the source that didn't survive parsing |
| Resolution | `ResolutionLoss` | Unresolved references — symbols the resolver couldn't find, plus resolution ratio |
| Property | `PropertyLoss` | Property verdicts — each verdict is itself `Imperfect<(), String, f64>` |
| Emit | `EmitLoss` | Phase records with structural loss, staleness, dark dimensions |

Plus convergence status and crystal identity.

### Holonomy

The total holonomy is a single `f64` summarizing how far the result is from perfect. Zero holonomy = settled crystal. Infinite holonomy = budget exhausted.

```
holonomy = parse.holonomy()
         + resolution.holonomy()
         + properties.holonomy()
         + emit.holonomy()
         + convergence_penalty
```

No other compiler has this. Rustc knows "how many errors." Mirror knows "how much was lost." The difference is dimensional: error count is cardinal, holonomy is geometric.

### What Mirror Can Adopt

From the research:

1. **Error codes** (from Rust): `M0001` through `M9999`. Machine-readable, explainable.
2. **Source spans** (from Rust/Gleam): Annotated source with carets and labels.
3. **Explain command** (from Rust): `mirror explain M0001` prints a tutorial.
4. **Three-state acknowledgment** (from Lean): Cite `sorry` as prior art for `Partial`.
5. **Conversational readability** (from Elm): Every error legible to non-experts.
6. **Typed error enums** (from Gleam): Internal representation is structured data, not strings.

What mirror must NOT adopt:

1. **Binary success/failure** (from Elm, Gleam): Mirror has three states. Don't collapse them.
2. **"Fix this" framing** (from all): Mirror errors are observations, not instructions.
3. **Warning suppression** (from Rust): Loss cannot be suppressed. It can be accepted.

---

## 3. The Optic Chain: inspect, focus, project

### The Insight

Alex's formulation:

```
inspect(&self, Optic) -> focus(&self, Optic) -> project(&self, Optic)
```

Errors are not special. They are values. Values can be navigated with optics. The framework crate already defines the optics hierarchy:

```
Iso → Lens → AffineTraversal → Traversal
       ↓           , ↗
      Prism ------'

Traversal → Fold (read-only)
Traversal → Setter (write-only)
```

The question: where does `Imperfect<T, E, L>` sit in this hierarchy?

### Imperfect as Prism

`Imperfect<T, E, L>` is a sum type with three variants. Each variant is a prism target:

```rust
// Prism<Imperfect<T, E, L>, T> — focuses on the success value
// preview: Success(t) → Some(t), Partial(t, _) → Some(t), Failure → None
// review: t → Success(t)
struct SuccessPrism;

// Prism<Imperfect<T, E, L>, (T, L)> — focuses on the partial case
// preview: Partial(t, l) → Some((t, l)), _ → None
// review: (t, l) → Partial(t, l)
struct PartialPrism;

// Prism<Imperfect<T, E, L>, (E, L)> — focuses on the failure case
// preview: Failure(e, l) → Some((e, l)), _ → None
// review: (e, l) → Failure(e, l)
struct FailurePrism;
```

Wait — there's a subtlety. `SuccessPrism` as defined above would match both `Success` and `Partial` (since both carry `T`). This is intentional: `is_ok()` returns true for both. But this breaks the prism law `review(preview(s)) = s` for `Partial` — reviewing gives `Success`, not `Partial`.

Two options:

**Option A: Strict prism.** `SuccessPrism` matches ONLY `Success(t)`. Three prisms, one per variant, each satisfying the laws.

**Option B: Lossy prism.** `ValuePrism` matches both `Success` and `Partial`, extracting `T`. `review` produces `Success(t)` — the loss is... lost. This is itself a lossy operation, which is poetically appropriate but breaks the law.

**Decision: Option A.** Prism laws are not negotiable. Three prisms, three variants, three lawful operations. If you want "give me the value regardless," compose: `SuccessPrism.preview(x).or_else(|| PartialPrism.preview(x).map(|(t, _)| t))`. That's a Fold, not a Prism.

### Loss as Lens

`MirrorLoss` is a product type with named fields. Each field is a lens:

```rust
// Lens<MirrorLoss, ParseLoss>
struct ParseLossLens;
// get: loss.parse.clone()
// set: MirrorLoss { parse: new_parse, ..loss }

// Lens<MirrorLoss, ResolutionLoss>
struct ResolutionLossLens;

// Lens<MirrorLoss, PropertyLoss>
struct PropertyLossLens;

// Lens<MirrorLoss, EmitLoss>
struct EmitLossLens;
```

These compose naturally:

```rust
// Lens<MirrorLoss, ParseLoss> ∘ Lens<ParseLoss, Vec<UnrecognizedDecl>>
// = Lens<MirrorLoss, Vec<UnrecognizedDecl>>
let unrecognized = ParseLossLens.compose(UnrecognizedLens);
```

### The Chain: inspect, focus, project

The three operations correspond to decreasing generality:

| Operation | Optic tier | What it does | Returns |
|-----------|-----------|--------------|---------|
| `inspect` | Fold | Read-only observation of all focused values | `Vec<A>` |
| `focus` | Lens | Zoom into a specific field, read and write | `A` (always present) |
| `project` | Prism | Extract a value if the variant matches | `Option<A>` |

Applied to an `Imperfect` compilation result:

```
result.inspect(HolonomyFold)        → [3.5]     // the total holonomy
result.inspect(UnrecognizedFold)     → ["widget"] // all unrecognized keywords

result.focus(LossLens)               → MirrorLoss { ... }
result.focus(LossLens.then(ParseLossLens)) → ParseLoss { ... }

result.project(SuccessPrism)         → Some(crystal)  // if Success
result.project(FailurePrism)         → Some((err, loss))  // if Failure
```

### Connection to the Fold Operator `<=`

In mirror source:

```
property check(grammar) <= verdict
```

The `<=` is the fold operator. Every fold returns `Imperfect`. The optic chain navigates the result:

```
source <= ast                    // parse fold → Imperfect<AST, ParseError, MirrorLoss>
ast <= resolved(ast)             // resolution fold → Imperfect<Resolved, ResError, MirrorLoss>
resolved <= verdict per property // property fold → Imperfect<(), String, f64> per property
resolved <= crystal              // emit fold → Imperfect<Crystal, EmitError, MirrorLoss>
```

Each intermediate `Imperfect` can be navigated with optics before passing to the next fold. This means error handling IS optic composition:

```
compile(source)
  .eh(|ast| resolve(ast))        // eh = terni-functor bind, accumulates loss
  .eh(|resolved| check(resolved))
  .eh(|checked| emit(checked))
```

At any point, you can inspect, focus, or project:

```
let result = compile(source);

// Inspect: what did we lose?
let holonomy = result.inspect(HolonomyFold);

// Focus: zoom into the parse loss
let parse_loss = result.focus(LossLens.then(ParseLossLens));

// Project: did we get a crystal?
let crystal = result.project(SuccessPrism);
```

### Imperfect as Traversal

A traversal visits all values. For `Imperfect`, this means visiting the value in both `Success` and `Partial`:

```rust
// Traversal<Imperfect<T, E, L>, T>
// traverse: Success(t) → [t], Partial(t, _) → [t], Failure → []
// rebuild: replace the T in Success or Partial
struct ValueTraversal;
```

This is the `Fold` that Option A's strict prisms compose into. It's a valid traversal — 0 or 1 elements.

More interesting: `Traversal<Vec<Imperfect<T, E, L>>, T>` — traversing a batch of compilation results, extracting all values that survived:

```rust
// Given: Vec<Imperfect<Crystal, Error, MirrorLoss>>
// traverse → all crystals from Success and Partial results
// The Failures are not visited — their loss is in the Loss, not the value.
```

This is exactly what boot does. It compiles N files, gets N `Imperfect` results, and the boot result is the traversal of all values that survived.

---

## 4. CLI Error Format

### Design Principles

1. **Three-state output.** `mirror compile` prints different headers for Success, Partial, and Failure.
2. **Holonomy always visible.** Even Success shows `holonomy: 0.0` — the measurement happened.
3. **Error codes.** `M0001` through `M9999`. Every error has a code. `mirror explain M0001`.
4. **Source spans.** Annotated source with `^^^` carets and labels.
5. **Loss breakdown.** Which fold produced the loss, what was lost.
6. **No "fix this" language.** Mirror says what it observed, not what you should do.

### Output Structure

#### Success

```
  compiled example.mirror → crystal:a3f8c2d1

  holonomy: 0.0
  crystal: a3f8c2d1e4b5...
```

Minimal. The measurement happened. Nothing was lost.

#### Partial

```
  compiled example.mirror → crystal:a3f8c2d1 (partial)

  holonomy: 1.0
  crystal: a3f8c2d1e4b5...

  loss:
    parse: 1 unrecognized declaration
      M1001 line 5: unrecognized keyword `widget`
       5 │ widget foo
         │ ^^^^^^

    resolution: 0
    properties: 0
    emit: 0
```

The crystal formed. Something was lost. The loss is itemized by fold.

#### Failure

```
  failed example.mirror

  holonomy: inf
  crystal: none

  error[M2001]: `type` requires a name
   --> example.mirror:1:1
    │
  1 │ type
    │ ^^^^ expected name after `type`
    │
    = loss: parse fold terminated. No AST produced.
    = holonomy: inf (budget exhausted — no crystal possible)
```

No crystal. The error is the primary output. Loss context follows.

### Color and Formatting

- Error codes: **bold red** for Failure errors, **bold yellow** for Partial loss.
- Source spans: line numbers in **blue**, carets in **red** or **yellow**.
- Holonomy: **green** for 0.0, **yellow** for finite > 0, **red** for infinite.
- Crystal OID: **dim** (it's metadata, not the point).

### Machine-Readable Output

`mirror compile --format json` produces:

```json
{
  "state": "partial",
  "holonomy": 1.0,
  "crystal": "a3f8c2d1e4b5...",
  "loss": {
    "parse": {
      "unrecognized": [
        { "keyword": "widget", "line": 5, "content": "foo" }
      ],
      "holonomy": 1.0
    },
    "resolution": { "unresolved_refs": [], "ratio": 1.0, "holonomy": 0.0 },
    "properties": { "verdicts": [], "holonomy": 0.0 },
    "emit": { "phases": [], "staleness": 0, "dark_dims": [], "holonomy": 0.0 }
  },
  "convergence": "settled"
}
```

This is the `MirrorLoss` struct serialized. The JSON IS the optic traversal — every field accessible by path.

---

## 5. Error Catalog

Error codes are assigned by fold and severity:

| Range | Fold | Severity |
|-------|------|----------|
| M1xxx | Parse | Loss (Partial) or Error (Failure) |
| M2xxx | Parse | Structural error (always Failure) |
| M3xxx | Resolution | Unresolved references |
| M4xxx | Property | Property check failures |
| M5xxx | Emit | Crystal formation failures |
| M9xxx | Boot | System-level errors |

### M1001 — Unrecognized Declaration

**Test:** `error_only_unrecognized`
**Source:** `widget foo\nroute /bar\n`
**State:** Failure (nothing recognized survived)

```
error[M1001]: unrecognized declaration `widget`
 --> <source>:1:1
  │
1 │ widget foo
  │ ^^^^^^ not a recognized keyword
  │
  = note: mirror recognizes: grammar, type, action, property, in

error[M1001]: unrecognized declaration `route`
 --> <source>:2:1
  │
2 │ route /bar
  │ ^^^^^ not a recognized keyword
```

**MirrorLoss fields:**
- `parse.unrecognized`: `[UnrecognizedDecl { keyword: "widget", line: 1, content: "foo" }, UnrecognizedDecl { keyword: "route", line: 2, content: "/bar" }]`
- All other folds: zero
**Holonomy:** 2.0 (one per unrecognized declaration)

When unrecognized declarations appear alongside recognized ones, the result is `Partial`, not `Failure`. The M1001 entries appear in the loss section, not the error section.

### M2001 — Bare Keyword (No Name)

**Tests:** `error_type_no_name`, `error_grammar_no_name`, `error_action_no_name`

#### `error_type_no_name`
**Source:** `type\n`

```
error[M2001]: `type` requires a name
 --> <source>:1:1
  │
1 │ type
  │ ^^^^ expected name after `type`
  │
  = note: usage: `type color = red | blue`
```

**MirrorLoss fields:**
- Parse fold terminates. No AST nodes produced.
- `convergence`: `BudgetExhausted`
**Holonomy:** infinity

#### `error_grammar_no_name`
**Source:** `grammar\n`

```
error[M2001]: `grammar` requires a name
 --> <source>:1:1
  │
1 │ grammar
  │ ^^^^^^^ expected name after `grammar` (e.g., `grammar @domain { ... }`)
```

**MirrorLoss fields:** Same pattern as `type` — bare keyword, parse failure.
**Holonomy:** infinity

#### `error_action_no_name`
**Source:** `action\n`

```
error[M2001]: `action` requires a name
 --> <source>:1:1
  │
1 │ action
  │ ^^^^^^ expected name after `action`
  │
  = note: usage: `action boot(identity) <= imperfect`
```

**MirrorLoss fields:** Same pattern.
**Holonomy:** infinity

### M2002 — Bare `in` (No Target)

**Test:** `error_in_no_target`
**Source:** `in\n`

```
error[M2002]: `in` requires a target grammar
 --> <source>:1:1
  │
1 │ in
  │ ^^ expected grammar reference after `in` (e.g., `in @actor`)
```

**MirrorLoss fields:** Parse failure. No target to resolve.
**Holonomy:** infinity

### M2003 — Duplicate Type Names

**Test:** `error_duplicate_type_names`
**Source:** `type color = red | blue\ntype color = green | yellow\n`

```
error[M2003]: duplicate type name `color`
 --> <source>:2:6
  │
1 │ type color = red | blue
  │      ----- first definition here
2 │ type color = green | yellow
  │      ^^^^^ redefined here
  │
  = note: each type name must be unique within its scope
```

**MirrorLoss fields:**
- This is a structural collision. Two AST nodes claim the same name.
- `parse.holonomy()`: 0.0 (parsing succeeded — the collision is semantic, not syntactic)
- The collision is detected during resolution or a parse-level name check.
**Holonomy:** 1.0 (one collision)

### M2004 — Double Operator

**Test:** `error_double_operator`
**Source:** `type x = = y\n`

```
error[M2004]: unexpected operator `=`
 --> <source>:1:10
  │
1 │ type x = = y
  │          ^ unexpected second `=` — did you mean `type x = y`?
```

**MirrorLoss fields:**
- The parser either produces a malformed AST (Partial) or fails (Failure).
- If Partial: the extra `=` is recorded as structural noise.
**Holonomy:** >= 1.0

### M2005 — Fold Operator in Wrong Context

**Test:** `error_fold_in_type_declaration` (Note: the `fold_in_property_declaration` test is a *success* case — `<=` is valid in property declarations.)
**Source:** `type x <= y\n`

```
error[M2005]: fold operator `<=` is not valid in type declarations
 --> <source>:1:8
  │
1 │ type x <= y
  │        ^^ `<=` is a fold operator — valid in `property` and `action` declarations
  │
  = note: for type composition, use `=` (e.g., `type x = y`)
  = note: for type inheritance, use `in` (e.g., `in @parent`)
```

**MirrorLoss fields:**
- If the parser records `OpticOp::Fold` on a type node: Partial with semantic loss.
- If the parser rejects it: Failure.
**Holonomy:** >= 1.0

### M4001 — Fold in Property Declaration (Success Case)

**Test:** `fold_in_property_declaration`
**Source:** `property check(grammar) <= verdict { traversal types\n  refract verdict\n}\n`

This is NOT an error. This is the correct usage of `<=`. Included here to define the success case:

```
  compiled <source> → crystal:...

  holonomy: 0.0
```

**MirrorLoss fields:** All zero.
**Holonomy:** 0.0

### M4002 — Fold Not Silent (AI Grammar)

**Test:** `ai_grammar_fold_not_silent`
**Source:** The `AI_GRAMMAR` constant — `action boot(identity) <= imperfect` inside a grammar block.

This test asserts that `<=` is either parsed correctly (producing `OpticOp::Fold` on the action) or honestly reported as loss. It must NOT be silently swallowed.

**If Success:**
- `action boot` has `OpticOp::Fold` in its `optic_ops`.
- Holonomy: 0.0.
- No error message needed.

**If Partial:**
- `action boot` has `OpticOp::Fold` (the fold landed).
- Some other loss exists (e.g., from unrecognized content in the grammar block).
- The loss is from something other than dropping `<=`.

**If Failure:**
```
error[M2005]: fold operator `<=` not yet supported in action declarations
 --> <source>:8:24
  │
8 │   action boot(identity) <= imperfect
  │                          ^^ fold operator in action
  │
  = loss: the `<=` was observed but could not be compiled
```

**MirrorLoss fields:**
- `parse.holonomy()` or `properties.holonomy()` > 0.0
**Holonomy:** > 0.0

### M9001 — Boot Success (CI Gate)

**Test:** `mirror_ci_boot_success`

This is the CI gate. When this test passes, the compiler ships.

**Conditions for pass:**
1. Zero resolution failures: every boot file resolves.
2. Zero loss: `total_loss.is_zero() == true`.
3. Zero holonomy: `total_loss.holonomy() == 0.0`.
4. Crystal identity law: `compile(compile(boot)) == compile(boot)`.

**CLI output when this succeeds:**

```
  mirror boot: Success(Mirror)

  files: 18/18 resolved
  holonomy: 0.0
  crystal: a3f8c2d1e4b5...
  idempotent: yes
```

**CLI output when this fails (current state):**

```
  mirror boot: Partial(Mirror)

  files: 10/18 resolved, 5 failed
  holonomy: 0.0 (parse only — resolution failures not yet in holonomy)

  failed:
    05-property: unresolved `in @form` — @form not defined in boot
    10-mirror: unresolved `in @form`, `in @type`, `in @boundary`, `in @lens`
    11-spec: unresolved references
    16-tui: unresolved references
    20-cli: unresolved references

  note: resolution failures are measured but not yet reflected in holonomy.
  This is the gap. When resolution loss flows into MirrorLoss.holonomy(),
  the number will be non-zero and the truth will be visible.
```

**MirrorLoss fields (current baseline):**
- `parse`: zero (all files parse)
- `resolution`: the 5 failed files have unresolved refs, but these are tracked in `boot.failed`, not yet in `MirrorLoss.resolution`
- `total_loss.holonomy()`: 0.0 (this is the gap — resolution failures should contribute)
**Holonomy:** 0.0 (should be > 0.0 once resolution loss is wired)

---

## 6. Design Decisions

### Error Codes: Yes

Mirror adopts error codes. Format: `M` + 4 digits. The `M` stands for mirror.

Rationale: error codes enable `mirror explain M2001`, machine-readable output, and documentation linking. Elm's no-codes approach works for a language with few error types. Mirror has four folds, each with its own loss types. Codes are load-bearing.

### "Observed" Not "Expected"

Mirror errors say what was observed, not what was expected:

- **Rust:** "expected `i32`, found `&str`" — prescriptive.
- **Mirror:** "observed `widget` at line 5 — not a recognized keyword" — descriptive.

This is not style. This is structural. Mirror's errors are measurements. A measurement says what it saw. The instrument doesn't tell you what you should have written.

### Loss Is Never Suppressed

Rust has `#[allow(unused)]`. Mirror has no equivalent. Loss is a measurement. You don't suppress a measurement. You can accept it (Partial with acknowledged loss), but you can't hide it.

If you want zero holonomy, write code that produces zero holonomy. There is no annotation that makes loss invisible.

### Partial Is Not a Warning

In Rust, warnings are things that compiled but look suspicious. In mirror, Partial is a thing that compiled and lost something measurable. These are different:

- A warning is a guess. The compiler thinks something might be wrong.
- Partial is a fact. The compiler measured what was lost.

Mirror does not have warnings. It has Success, Partial, and Failure. The three states are the complete description.

### Holonomy in Every Output

Even `Success` shows `holonomy: 0.0`. The measurement always happens. Zero is a meaningful result — it means the crystal settled perfectly. Omitting it for success would imply that measurement only happens on failure. It doesn't.

---

## 7. Implementation Notes (Not Code — Architecture)

### The Error Enum

Internally, each error code maps to a variant of a `MirrorError` enum. Each variant carries the structured data needed to render it:

- `M1001(UnrecognizedDecl)` — the keyword, line, content
- `M2001 { keyword: String, line: usize }` — bare keyword
- `M2002 { line: usize }` — bare `in`
- `M2003 { name: String, first_line: usize, second_line: usize }` — duplicate
- `M2004 { operator: String, line: usize, col: usize }` — double operator
- `M2005 { operator: String, context: String, line: usize, col: usize }` — wrong context

This enum is the Gleam pattern: typed errors with structured data. The rendering is separate from the data.

### The Optic Implementations

When the framework crate is available (it's currently in `_archive`), `Imperfect` should implement or provide:

1. `SuccessPrism`, `PartialPrism`, `FailurePrism` — three prisms, one per variant.
2. `ValueTraversal` — traversal over the value in Success and Partial (0 or 1 elements).
3. `LossLens` — lens into the loss (zero for Success, carried for Partial/Failure).
4. `HolonomyFold` — fold that computes holonomy from any `Imperfect`.

These live in the `prism` crate (terni), not in mirror. They are generic over `T`, `E`, `L`.

### The CLI Renderer

The CLI renderer takes `Imperfect<Compiled, String, MirrorLoss>` and produces the formatted output. It is a `Fold` — it reads the result without modifying it:

```
render: Imperfect<Compiled, String, MirrorLoss> → String
```

This is `inspect` in the optic chain. The renderer inspects the result and produces human-readable output. It never modifies the result. It never consumes it. It observes.

---

## 8. Open Questions

1. **Should resolution failures flow into `MirrorLoss.holonomy()`?** Currently they don't — the boot baseline test shows `holonomy: 0.0` despite 5 failed files. This feels like a gap. The holonomy should reflect all measured loss.

2. **Should `mirror explain` work offline?** Rust's `--explain` requires the compiler binary. Mirror could ship error explanations as `.mirror` files in the boot grammar — dogfooding the format.

3. **Should error codes be stable across versions?** Yes. Once assigned, an error code never changes meaning. It can be deprecated (with a note in `mirror explain`), but never reassigned.

4. **Should the optic implementations live in terni or in a separate crate?** Terni is the natural home (it owns `Imperfect`), but it would need a dependency on the framework crate. If framework is archived, this blocks. Alternative: a bridge crate `terni-optics` that provides the implementations.

5. **Should `Partial` results have a different exit code from `Success`?** Proposal: exit 0 for Success, exit 0 for Partial (it compiled), exit 1 for Failure. But: `mirror compile --strict` exits 1 for Partial too. The `--strict` flag means "I want Success or nothing."
