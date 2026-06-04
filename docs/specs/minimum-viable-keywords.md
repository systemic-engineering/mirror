# Minimum Viable Keywords

> Mara: research. Taut: benchmarks.
> Measured 2026-04-14 against mirror@HEAD (26 boot files, 561 tests).

---

## Status Quo

The Rust parser hardcodes 23 keywords in `DeclKind::parse()` plus the
`abstract` modifier (24 total):

```
form, type, prism, in, out, property, fold, requires, invariant, ensures,
focus, project, split, shift, settle, traversal, lens, action, recover,
rescue, grammar, default, binding
+ abstract (modifier)
```

### Per-File Parse Results (current parser)

```
FILE                           STATUS     HOLONOMY   UNRECOGNIZED
------------------------------------------------------------------------------------------
00-prism.mirror                Success    0.0
01-meta.mirror                 Partial    6.0        unfold, subset, superset, iso, not-iso(split as not+!)
01a-meta-action.mirror         Partial    2.0        != operator (split as ! token)
01b-meta-io.mirror             Partial    2.0        != operator (split as ! token)
02-shatter.mirror              Success    0.0
03-code.mirror                 Success    0.0
03a-code-rust.mirror           Success    0.0
04-actor.mirror                Success    0.0
05-property.mirror             Success    0.0
06-package.mirror              Success    0.0
06a-package-git.mirror         Success    0.0
06b-package-spec.mirror        Success    0.0
std/benchmark.mirror           Success    0.0
std/bool.mirror                Success    0.0
std/cli.mirror                 Success    0.0
std/list.mirror                Success    0.0
std/map.mirror                 Success    0.0
std/mirror.mirror              Success    0.0
std/number.mirror              Success    0.0
std/option.mirror              Partial    1.0        -> (return type on zoom decl)
std/order.mirror               Success    0.0
std/result.mirror              Partial    1.0        -> (return type on zoom decl)
std/set.mirror                 Success    0.0
std/text.mirror                Success    0.0
std/time.mirror                Success    0.0
std/tui.mirror                 Success    0.0
------------------------------------------------------------------------------------------
TOTAL: 21 Success, 5 Partial, 0 Failure
TOTAL PARSE HOLONOMY: 12.0
```

Resolution: 19 resolve, 7 fail (4 kernel dependency ordering + 3 std missing refs).

### Root Causes of Holonomy

The 12.0 holonomy comes from three distinct issues:

1. **Optic operator keywords (6.0):** `01-meta.mirror` declares optic operators as
   top-level aliases (`unfold =>(ref, ref)`, `subset <(ref, ref)`, etc.). These
   5 keywords have corresponding `OpticOp` variants but no `DeclKind` variants.
   The tokenizer splits `not-iso` into `not` then `!`, contributing the 6th
   unrecognized entry.

2. **`!=` operator tokenization (4.0):** The `!=` operator in `type X != iso`
   (files 01a, 01b) gets tokenized as `!` + `=` (Tok::Equals). The `!` alone
   appears as an unrecognized word at top level. This is a tokenizer issue,
   not a keyword issue.

3. **`->` return type on non-action decls (2.0):** `shift(option(a)) -> imperfect`
   in option.mirror and result.mirror. The `->` return type annotation is only
   consumed by `action` declarations. For `zoom`/`fold` declarations, the `->` is
   left as a dangling top-level token.

### Silent Loss (not measured in holonomy)

Inside blocks, unrecognized keywords are silently skipped. The parser does not
record loss for nested unrecognized content. Keywords that are skipped silently:

- `io` (01b-meta-io, 02-shatter): used as a grammar action keyword
- `flag`, `command` (std/cli.mirror): CLI-specific declaration forms
- `template` (01-meta.mirror): grammar template declarations
- Field names in type bodies: `name`, `oid`, `duration`, etc. (correctly silent)

This silent loss is structurally different from top-level loss. Field names inside
type bodies SHOULD be silent. But `io`, `flag`, `command` are declaration keywords
the grammar defines but the parser doesn't know. They produce no holonomy but
their content is dropped.

---

## Approach A: Minimal Kernel (6 hardcoded)

```
Hardcoded: in, out, type, grammar, property, action
Learned:   everything else from boot
```

**Would break:** Every boot file that uses `prism`, `focus`, `project`, `split`,
`shift`, `settle`, `fold`, `requires`, `invariant`, `ensures`, `traversal`,
`lens`, `recover`, `rescue`, `default`, `binding` at top level or inside blocks.

- `00-prism.mirror`: uses `focus`, `prism`, `project` -- all become unrecognized
- `01-meta.mirror`: uses `focus`, `fold`, `project`, `split`, `shift`, `settle` -- all lost
- `05-property.mirror`: uses `property`, `fold`, `traversal`, `lens`, `settle` inside
- Every std grammar with `recover`/`rescue` blocks: action body content lost

**Estimated holonomy:** 60+ (unmeasured, would require parser modification to test).

**Verdict:** Not viable without a self-teaching parser. The optic keywords (`focus`,
`project`, `split`, `shift`, `settle`) are structural -- they define how the prism
pipeline works. If the parser doesn't know them, it can't build the AST.

---

## Approach B: Current 23 + Missing Keywords

Add the missing keywords to `DeclKind`. Total would be 23 + N.

**What to add to reach zero top-level holonomy:**

The three root causes require different fixes:

1. **Optic operator keywords:** `unfold`, `subset`, `superset`, `iso`, `not-iso`.
   These already have `OpticOp` variants. Adding `DeclKind` variants is
   straightforward and mirrors the existing pattern (OpticOp::Fold -> DeclKind::Fold).

2. **`!=` tokenizer fix:** Not a keyword issue. The tokenizer needs to recognize
   `!=` as a single `Word("!=")` token, not `Word("!")` + `Equals`. Similarly,
   `not-iso` needs to be tokenized as one word.

3. **`->` return type on all decls:** Extend `parse_decl` to consume `-> type`
   for zoom/fold/traversal declarations, not just actions.

**For nested block keywords (reducing silent loss):**

- `io`: add DeclKind::Io (or treat as action inside grammar blocks)
- `flag`, `command`: add DeclKind::Flag, DeclKind::Command (CLI-specific)
- `template`: already declared in boot as `type template(grammar, block)`

**Estimated new DeclKind variants:** 5 optic + 3 block = 8 new, total 31.

**Impact on tests:** The existing `mirror_ci_boot_baseline` test asserts
`holonomy <= 15.0`. Adding keywords would reduce holonomy toward zero.
No existing tests would break -- holonomy can only decrease.

**Verdict:** Safe, incremental, zero-risk. But grows the hardcoded set
monotonically. Every new grammar keyword requires a Rust change + release.

---

## Approach C: Two-Tier Keywords (Recommended)

```
Tier 1 (Rust):  the minimum to parse 00-prism.mirror
Tier 2 (boot):  keywords declared by boot files, learned during boot
```

### Tier 1: The Bootstrap Set

To parse `00-prism.mirror`, the parser needs:

```
focus, prism, project, out, in, split, shift, settle
```

But `00-prism.mirror` doesn't use `type`, `grammar`, `action`, `recover`,
`rescue`, `property`, etc. Those are introduced by later boot files.

The minimum set to bootstrap through ALL kernel files:

```rust
// Structural (required by 00-prism)
In, Out, Focus, Project, Split, Shift, Settle, Prism,

// Type system (required by 01-meta)
Type, Grammar, Fold,

// Error handling (required by 01-meta imperfect type)
Recover, Rescue,

// Action boundary (required by 01a-meta-action)
Action,

// Properties (required by 05-property)
Property, Traversal, Lens, Requires, Invariant, Ensures,

// Configuration (required by 06-package)
Default, Binding,

// Container (required by parse_form synthetic wrapper)
Form,
```

This is... the current set of 23. Every keyword is used by at least one kernel
file. None can be removed without breaking kernel compilation.

### What Tier 2 Adds

Tier 2 keywords would be learned from boot declarations:

```
unfold, subset, superset, iso, not-iso     -- from 01-meta (optic operators)
io                                          -- from 01b-meta-io (grammar keyword)
flag, command                               -- from std/cli (grammar keywords)
template                                    -- from 01-meta (type declaration)
```

**Mechanism:** When the parser encounters `out X` at the top level of a boot
file, it registers `X` as a known keyword for subsequent files. Or: when a
`grammar @name { ... }` block declares `io read(path) => imperfect`, the
parser learns that `io` is a declaration keyword within that grammar's scope.

### The Cascade (Taut)

Here's where it gets interesting. The two-tier approach doesn't actually reduce
the Tier 1 set below 23. The cascade is:

1. `00-prism` needs 8 keywords to parse (the optics + in/out/prism)
2. `01-meta` needs Type, Grammar, Fold, Recover, Rescue (5 more = 13)
3. `01a-meta-action` needs Action (1 more = 14)
4. `05-property` needs Property, Traversal, Lens, Requires, Invariant, Ensures (6 more = 20)
5. `06-package` needs Default (1 more = 21)
6. `std/tui` needs Binding (1 more = 22)
7. Form is synthetic (used by parse_form wrapper = 23)

Every boot file introduces keywords that the next file depends on. The boot
sequence IS the keyword introduction order. There's no file that uses keywords
not already needed by an earlier file, except for the Tier 2 additions.

**The real win of Tier 2 is not reducing Tier 1. It's preventing Tier 1 from
growing.** New grammar keywords (like `io`, `flag`, `command`) don't require
Rust parser changes. They're declared in boot and learned at runtime.

---

## Benchmark: What Would Reach Zero Holonomy?

### Fix 1: Tokenizer (reduces holonomy by 4.0)

Recognize `!=` as a single token. Recognize `not-iso` as a hyphenated word
(it already does for `not-iso` at top level -- the issue is it's split into
`not` + the operator sequence `-iso` -> `!`... actually the tokenizer treats
`-` as part of operator sequences, splitting `not-iso` into `not` then `!`
because `-` starts an operator character sequence which includes `!`).

Fix: handle `-` specially when preceded by an alphanumeric word.

### Fix 2: Optic operator DeclKind variants (reduces holonomy by 5.0)

Add to DeclKind:
```rust
Unfold,   // from OpticOp::Unfold
Subset,   // from OpticOp::Subset
Superset, // from OpticOp::Superset
Iso,      // from OpticOp::Iso
NotIso,   // from OpticOp::NotIso
```

These already have `OpticOp` variants. The `to_decl_kind()` and
`from_decl_kind()` mappings would be completed. Symmetry restored.

### Fix 3: Return type on all declaration kinds (reduces holonomy by 2.0)

Extend `parse_decl` to parse `-> type` for any DeclKind, not just Action.
The `->` return type annotation is already in the grammar surface (zoom, fold
declarations use it). The parser just doesn't consume it for non-action decls.

### Fix 4: Block-level keyword learning (reduces silent loss)

When parsing grammar blocks, recognize keywords declared by the grammar itself.
This is the self-teaching parser: `grammar @io { io read(path) => imperfect }`
teaches the parser that `io` is a keyword within `@io`'s scope.

**Total holonomy after fixes 1-3: 0.0 (zero parse holonomy)**

---

## The Minimum Viable Set (Spec)

### DeclKind (Rust hardcoded): 28 keywords

```rust
pub enum DeclKind {
    // Structural (bootstrap)
    Form,         // synthetic wrapper (deprecated as user keyword)
    Prism,        // prism declaration
    In,           // dependency import
    Out,          // export

    // Type system
    Type,         // type declaration
    Grammar,      // grammar block

    // Optic operations (all 10 — symmetry with OpticOp)
    Focus,        // () grouping
    Project,      // |> projection
    Split,        // | branching
    Shift,        // -> transformation
    Settle,       // .. settlement
    Fold,         // <= observation
    Unfold,       // => expansion            NEW
    Subset,       // < containment           NEW
    Superset,     // > containment           NEW
    Iso,          // = equivalence            NEW
    NotIso,       // != non-equivalence       NEW

    // Optic composition
    Traversal,    // multi-focus
    Lens,         // guaranteed-focus

    // Properties
    Property,     // property declaration
    Requires,     // precondition
    Invariant,    // invariant
    Ensures,      // postcondition

    // Actions
    Action,       // action declaration

    // Error handling
    Recover,      // partial recovery
    Rescue,       // failure recovery

    // Configuration
    Default,      // default value
    Binding,      // key binding
}
```

Plus modifier: `abstract` (not a DeclKind, handled separately).

### Tier 2 (boot-declared, runtime-learned): future

Keywords declared in grammar blocks that the parser doesn't hardcode:
`io`, `flag`, `command`, `template`, and any future grammar-specific keywords.

**Implementation path:**
1. Fix 2 first (add 5 optic DeclKind variants) -- pure addition, no risk
2. Fix 3 next (return type on all decls) -- parser change, low risk
3. Fix 1 last (tokenizer `!=` and `not-iso`) -- tokenizer change, medium risk
4. Tier 2 is a separate design task (self-teaching parser)

### Test Impact

- No existing tests break (holonomy can only decrease)
- `mirror_ci_boot_baseline` assertion `holonomy <= 15.0` becomes `holonomy == 0.0`
  after fixes 1-3
- 5 new `DeclKind` variants require 5 new roundtrip tests + `to_decl_kind` /
  `from_decl_kind` mapping updates
- Estimated: 10-15 new tests, 0 broken tests

---

## Decision Record

**The minimum viable set is the current 23 + 5 optic completions = 28.**

The 5 new keywords (`Unfold`, `Subset`, `Superset`, `Iso`, `NotIso`) are not
arbitrary additions. They complete the symmetry between `OpticOp` (10 variants)
and `DeclKind` (currently maps only 5 of 10). The mapping table has holes.
Filling them is a correctness fix, not feature growth.

**Approach C (two-tier) is the correct long-term architecture** but its value
is in preventing future growth, not reducing the current set. The Tier 1 set
cannot shrink below 23 because every keyword is used by at least one kernel file.

**The self-teaching parser (Tier 2) should be designed separately** as it
involves parser state threading, boot order dependencies, and scope management.
It's the right architecture but it's a larger change than filling the OpticOp
symmetry holes.

**Priority order:**
1. Add 5 DeclKind variants (OpticOp symmetry) -- immediate, reduces holonomy by 5
2. Fix `->` return type on all decls -- immediate, reduces holonomy by 2
3. Fix `!=` / `not-iso` tokenization -- near-term, reduces holonomy by 5
4. Design self-teaching parser -- future, prevents holonomy growth
