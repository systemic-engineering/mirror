# Typed Loss Composition

**Author:** Mara
**Date:** 2026-04-15
**Status:** Draft
**Depends on:** `error-surface-spec.md` (error codes), `prism::Loss` trait (monoid)

---

## 0. The Problem

`holonomy: 130` tells you 130 things are wrong. It does not tell you what kind
of things, or where they cluster, or what single action would eliminate 45 of
them at once.

The current `MirrorLoss` decomposes into four folds (parse, resolution,
property, emit). Within each fold the items are flat vectors. An engineer
staring at `mirror ci boot/` sees:

```
ci boot/ (18 files)
holonomy: 130.0000
  00-kernel.mirror holonomy: 0.0000
  01-types.mirror holonomy: 5.0000
  02-std.mirror holonomy: 45.0000
  ...
```

The number 45 hides the fact that 40 of those are the keyword `flag` and 5 are
the keyword `command`. One parser change eliminates 40. The engineer cannot see
this without reading every UnrecognizedDecl by hand.

This spec adds **typed, composable loss categories** that make the answer
visible at every level: CLI, LSP, CI gate, JSON output.

---

## 1. Design Principles

1. **Loss IS a monoid.** `zero()` is identity, `combine` is associative,
   `total()` is absorbing. This does not change.

2. **Categories are projections, not new types.** A category is a function
   `LossItem -> String` applied to the existing loss vectors. No new storage —
   just a view.

3. **Categories compose with combine.** If you `combine` two losses, the
   categories of the result are the union of the categories of the inputs.
   This falls out of vector concatenation — no special logic needed.

4. **Static where possible, dynamic where necessary.** Parse loss categories
   are the keyword strings already stored in `UnrecognizedDecl.keyword`.
   Resolution loss categories are the ref prefixes (`@io`, `@runtime`).
   Property loss categories are the property names. No new enum — the
   data already carries its category.

5. **The Prism connection is structural, not metaphorical.** Each category
   IS a Prism on the loss vector: `preview` returns `Some(filtered_vec)` if
   any items match, `None` otherwise. `review` injects a filtered vec back
   into the loss. The prism laws hold because the filter is a subset operation.

---

## 2. Core Types

### 2.1 LossItem — the atomic unit

Every loss item across all four folds normalizes to this:

```rust
/// A single item of measured loss, normalized for display and query.
#[derive(Clone, Debug, PartialEq)]
pub struct LossItem {
    /// Which fold produced this item.
    pub fold: LossFold,
    /// The category within the fold (e.g., keyword name, ref name, property name).
    pub category: String,
    /// Human-readable description.
    pub message: String,
    /// Error code from the catalog (M1001, M3001, etc.).
    pub code: &'static str,
    /// Source location, if known.
    pub location: Option<LossLocation>,
    /// The holonomy contribution of this single item.
    pub weight: f64,
}
```

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum LossFold {
    Parse,
    Resolution,
    Property,
    Emit,
}
```

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct LossLocation {
    /// File path, if known.
    pub file: Option<String>,
    /// 1-based line number.
    pub line: usize,
    /// 0-based column, if known.
    pub col: Option<usize>,
}
```

### 2.2 LossCategory — a grouped view

```rust
/// A group of loss items sharing a category within a fold.
#[derive(Clone, Debug, PartialEq)]
pub struct LossCategory {
    /// The fold this category belongs to.
    pub fold: LossFold,
    /// The category name (e.g., "flag", "@io", "reachability").
    pub name: String,
    /// The error code shared by items in this category.
    pub code: &'static str,
    /// Number of items.
    pub count: usize,
    /// Sum of item weights.
    pub holonomy: f64,
    /// The items themselves.
    pub items: Vec<LossItem>,
}
```

### 2.3 LossView — the decomposed loss

```rust
/// A decomposed view of MirrorLoss, organized by fold and category.
///
/// This is a projection — it does not own the data. It is computed
/// from MirrorLoss on demand.
pub struct LossView {
    /// Categories grouped by fold, sorted by count descending.
    pub by_fold: BTreeMap<LossFold, Vec<LossCategory>>,
    /// Total holonomy (same as MirrorLoss::holonomy()).
    pub holonomy: f64,
    /// Total item count across all folds.
    pub count: usize,
}
```

---

## 3. Projection: MirrorLoss -> LossView

The existing `MirrorLoss` fields already carry category information. The
projection extracts it:

```rust
impl MirrorLoss {
    /// Decompose this loss into a categorized view.
    pub fn view(&self) -> LossView {
        let mut items = Vec::new();

        // Parse fold: group by keyword
        for unrec in &self.parse.unrecognized {
            items.push(LossItem {
                fold: LossFold::Parse,
                category: unrec.keyword.clone(),
                message: format!(
                    "unrecognized keyword `{}`", unrec.keyword
                ),
                code: "M1001",
                location: Some(LossLocation {
                    file: None,
                    line: unrec.line,
                    col: None,
                }),
                weight: 1.0,
            });
        }

        // Resolution fold: group by ref name
        for (name, _oid) in &self.resolution.unresolved_refs {
            items.push(LossItem {
                fold: LossFold::Resolution,
                category: name.clone(),
                message: format!("unresolved reference `{}`", name),
                code: "M3001",
                location: None,
                weight: 1.0,
            });
        }

        // Property fold: group by property name
        for verdict in &self.properties.verdicts {
            let (msg, code, weight) = match &verdict.verdict {
                Imperfect::Success(_) => continue,
                Imperfect::Partial(_, loss) => (
                    format!("property `{}` partial", verdict.property),
                    "M4001",
                    *loss,
                ),
                Imperfect::Failure(obs, loss) => (
                    format!("property `{}` failed: {}", verdict.property, obs),
                    "M4002",
                    *loss,
                ),
            };
            items.push(LossItem {
                fold: LossFold::Property,
                category: verdict.property.clone(),
                message: msg,
                code,
                location: None,
                weight,
            });
        }

        // Emit fold: group by phase
        for phase_rec in &self.emit.phases {
            if phase_rec.structural_loss > 0.0 {
                let phase_name = format!("{:?}", phase_rec.phase).to_lowercase();
                items.push(LossItem {
                    fold: LossFold::Emit,
                    category: phase_name,
                    message: format!(
                        "structural loss in {:?} phase: {:.4}",
                        phase_rec.phase, phase_rec.structural_loss
                    ),
                    code: "M5001",
                    location: None,
                    weight: phase_rec.structural_loss,
                });
            }
        }

        // Build the view
        LossView::from_items(items, self.holonomy())
    }
}
```

```rust
impl LossView {
    fn from_items(items: Vec<LossItem>, holonomy: f64) -> Self {
        let count = items.len();
        let mut by_fold: BTreeMap<LossFold, Vec<LossCategory>> = BTreeMap::new();

        // Group items by (fold, category)
        let mut groups: BTreeMap<(LossFold, String), Vec<LossItem>> = BTreeMap::new();
        for item in items {
            groups
                .entry((item.fold.clone(), item.category.clone()))
                .or_default()
                .push(item);
        }

        for ((fold, name), group_items) in groups {
            let cat = LossCategory {
                fold: fold.clone(),
                code: group_items[0].code,
                count: group_items.len(),
                holonomy: group_items.iter().map(|i| i.weight).sum(),
                name,
                items: group_items,
            };
            by_fold.entry(fold).or_default().push(cat);
        }

        // Sort categories within each fold by count descending
        for cats in by_fold.values_mut() {
            cats.sort_by(|a, b| b.count.cmp(&a.count));
        }

        LossView { by_fold, holonomy, count }
    }
}
```

### 3.1 Why projection, not storage

The categorized view is computed, not stored. Reasons:

1. **No storage overhead.** `MirrorLoss` stays lean for the hot path
   (compilation loop, convergence detection).
2. **No serialization changes.** The monoid operations (`combine`, `zero`,
   `total`) work on the existing vectors. Categories are a lens over them.
3. **Multiple groupings.** The same loss can be grouped by keyword, by file,
   by severity. Each is a different projection. None is canonical.

---

## 4. The Prism Hierarchy

### 4.1 Loss as an optic target

The optic hierarchy on `MirrorLoss` has three tiers:

```
MirrorLoss                          (product type — Lens per field)
  |-- ParseLoss                     (Lens: loss.parse)
  |     |-- Vec<UnrecognizedDecl>   (Traversal: iterate items)
  |     |-- [group_by keyword]      (Prism: filter to one keyword)
  |
  |-- ResolutionLoss                (Lens: loss.resolution)
  |     |-- Vec<(String, TraceOid)> (Traversal: iterate refs)
  |     |-- [group_by ref name]     (Prism: filter to one ref)
  |
  |-- PropertyLoss                  (Lens: loss.properties)
  |     |-- Vec<PropertyVerdict>    (Traversal: iterate verdicts)
  |     |-- [group_by property]     (Prism: filter to one property)
  |
  |-- EmitLoss                      (Lens: loss.emit)
        |-- Vec<PhaseRecord>        (Traversal: iterate phases)
        |-- [group_by phase]        (Prism: filter to one phase)
```

Each `group_by` operation IS a Prism:

```rust
// Prism<Vec<UnrecognizedDecl>, Vec<UnrecognizedDecl>>
// preview: filter to items matching keyword k
//          Some(filtered) if non-empty, None if empty
// review:  replace all items matching keyword k
struct KeywordPrism(String);
```

The prism laws hold:

- `review(preview(s)) = s` when the filter is total (all items match)
- `preview(review(t)) = Some(t)` always (injected items match their own filter)

### 4.2 Composing optics on loss

Navigate to a specific category:

```
ParseLossLens                         Lens<MirrorLoss, ParseLoss>
  .then(UnrecognizedTraversal)        Traversal<ParseLoss, UnrecognizedDecl>
  .then(KeywordPrism("flag"))         AffineTraversal over matching items
```

Or query across folds:

```
LossViewFold                          Fold<MirrorLoss, LossItem>
  .filter(|item| item.fold == Parse)  filtered Fold
  .group_by(|item| item.category)     BTreeMap<String, Vec<LossItem>>
```

### 4.3 Filtering and projection on LossView

```rust
impl LossView {
    /// Filter to a single fold.
    pub fn fold(&self, fold: &LossFold) -> Vec<&LossCategory> {
        self.by_fold.get(fold).map_or(Vec::new(), |cats| {
            cats.iter().collect()
        })
    }

    /// Filter to a single category across all folds.
    pub fn category(&self, name: &str) -> Vec<&LossCategory> {
        self.by_fold.values()
            .flat_map(|cats| cats.iter())
            .filter(|c| c.name == name)
            .collect()
    }

    /// Top N categories by count, across all folds.
    pub fn top(&self, n: usize) -> Vec<&LossCategory> {
        let mut all: Vec<_> = self.by_fold.values()
            .flat_map(|cats| cats.iter())
            .collect();
        all.sort_by(|a, b| b.count.cmp(&a.count));
        all.truncate(n);
        all
    }
}
```

---

## 5. Interaction with the Loss Trait

The `prism::Loss` trait (from terni) currently has four methods:

```rust
pub trait Loss: Clone + Default {
    fn zero() -> Self;
    fn total() -> Self;
    fn is_zero(&self) -> bool;
    fn combine(self, other: Self) -> Self;
}
```

### 5.1 No trait change needed

The `Loss` trait does NOT need new methods. The decomposition is domain-specific
to `MirrorLoss`, not generic across all loss types. A `ConvergenceLoss` (steps
remaining) has no categories. An `ApertureLoss` (unobserved dimensions) might,
but differently.

The `view()` method lives on `MirrorLoss`, not on `Loss`. This is a deliberate
choice: the trait stays minimal, the domain type carries the richness.

### 5.2 Combine preserves categories

Since categories are projections over the existing vectors, and `combine`
concatenates those vectors, categories compose automatically:

```rust
let a: MirrorLoss = /* 45 "flag" items */;
let b: MirrorLoss = /* 30 "command" items, 10 "@io" items */;
let combined = a.combine(b);

// combined.view() will show:
//   parse: flag (45), command (30)
//   resolution: @io (10)
```

No special logic. The monoid does the work.

### 5.3 Holonomy is the fold over categories

The holonomy of a `LossView` equals the holonomy of the `MirrorLoss` it came
from (minus convergence penalty, which is not an item). This is a consistency
check, not a computation:

```
holonomy = sum(category.holonomy for category in all_categories)
         + convergence_penalty
```

---

## 6. CLI Output

### 6.1 Current output (before this spec)

```
ci boot/ (18 files)
holonomy: 130.0000
  00-kernel.mirror holonomy: 0.0000
  01-types.mirror holonomy: 5.0000
  02-std.mirror holonomy: 45.0000
  ...
```

### 6.2 Proposed output: `mirror ci <path>`

```
mirror ci boot/

  holonomy: 130.0

  parse (105):
    flag          45  (02-std, 03-cli, 07-runtime)
    command       30  (03-cli)
    template      20  (04-properties)
    where         10  (04-properties, 05-constraints)

  resolution (15):
    @io            8  (01b-io, 01c-fs, 02-shatter)
    @runtime       5  (02-std, 07-runtime)
    @time          2  (06-benchmark)

  properties (10):
    reachability   6  partial (loss: 0.5 each)
    unique_names   4  failed

  emit (0):
    --

  18 files, 130.0 total holonomy
```

### 6.3 Proposed output: `mirror ci <file>` (single file)

```
mirror ci boot/02-std.mirror

  holonomy: 45.0

  parse (45):
    flag          40  lines 12, 15, 18, 22, 25, 28, ... (+34 more)
    command        5  lines 8, 44, 67, 89, 102

  resolution (0)
  properties (0)
  emit (0)

  crystal: a3f8c2d1 (partial)
```

### 6.4 Proposed output: `mirror craft boot`

```
mirror craft boot

  holonomy: 130.0

  parse (105):
    flag          45  add DeclKind::Flag to parser
    command       30  add DeclKind::Command to parser
    template      20  add DeclKind::Template to parser
    where         10  constraint clauses not yet parsed

  resolution (25):
    @io            8  grammar not in boot path
    @runtime       5  grammar not in boot path
    @time          4  grammar not in boot path
    @code          3  grammar not in boot path
    @html          3  grammar not in boot path
    @boundary      2  grammar not defined

  18 files compiled, 130.0 total holonomy
```

The `craft` output adds actionable hints after each category because craft
is the build command — the engineer is actively working.

### 6.5 Proposed output: `mirror ci <path> --json`

```json
{
  "holonomy": 130.0,
  "count": 130,
  "folds": {
    "parse": {
      "holonomy": 105.0,
      "count": 105,
      "categories": [
        {
          "name": "flag",
          "code": "M1001",
          "count": 45,
          "holonomy": 45.0,
          "files": ["02-std.mirror", "03-cli.mirror", "07-runtime.mirror"]
        },
        {
          "name": "command",
          "code": "M1001",
          "count": 30,
          "holonomy": 30.0,
          "files": ["03-cli.mirror"]
        }
      ]
    },
    "resolution": {
      "holonomy": 15.0,
      "count": 15,
      "categories": [
        {
          "name": "@io",
          "code": "M3001",
          "count": 8,
          "holonomy": 8.0
        }
      ]
    },
    "properties": { "holonomy": 10.0, "count": 10, "categories": [] },
    "emit": { "holonomy": 0.0, "count": 0, "categories": [] }
  },
  "convergence": "settled"
}
```

---

## 7. Interaction with the LSP

The existing `loss_to_diagnostics` in `lsp/server.rs` already decomposes
`MirrorLoss` into `MirrorDiagnostic` items with codes (M1001, M3001, M4001,
M4002, M9001, M9003). This spec does not replace that function — it complements
it.

### 7.1 Diagnostic grouping

LSP clients group diagnostics by code. When the editor shows:

```
M1001: unrecognized keyword `flag`     (45 occurrences)
M1001: unrecognized keyword `command`  (30 occurrences)
M3001: unresolved reference `@io`      (8 occurrences)
```

...the grouping IS the category view. The LSP already provides this by emitting
one diagnostic per item with the same code. The client aggregates.

### 7.2 Code actions from categories

A category with many items suggests a single fix. The LSP can generate code
actions from `LossView`:

```
category "flag" (45 items, code M1001)
  → code action: "Add DeclKind::Flag to parser"
  → applies to: all 45 diagnostics at once

category "@io" (8 items, code M3001)
  → code action: "Add `in @io` import"
  → applies to: all 8 diagnostics in files that reference @io
```

This is future work. The important thing is that the `LossView` structure
makes it possible — a flat diagnostic list does not.

### 7.3 Diagnostic severity from categories

The `--strict` flag interacts with categories through severity mapping:

| Default | `--strict` |
|---------|-----------|
| M1001 = Warning | M1001 = Error |
| M3001 = Error | M3001 = Error |
| M4001 = Warning | M4001 = Error |
| M4002 = Error | M4002 = Error |

Under `--strict`, any non-zero holonomy is a build failure. Categories
don't change — their severity does.

---

## 8. Aggregation Across Files

When `mirror ci boot/` compiles 18 files, each produces its own `MirrorLoss`.
The `combine` monoid merges them. But categories need file attribution.

### 8.1 File-attributed loss

Extend `LossItem` with an optional file path:

```rust
pub struct LossItem {
    // ... existing fields ...
    /// Source file, if loss came from a multi-file compilation.
    pub file: Option<String>,
}
```

The `ci` command sets this before combining:

```rust
for entry in &entries {
    let (oid, mut loss) = self.ci_single_file(path)?;
    // Tag items with their source file
    let tagged = loss.tag_file(entry.file_name());
    total_loss = total_loss.combine(tagged);
}
```

### 8.2 Category file lists

In the `LossView`, each category aggregates its file attributions:

```rust
impl LossCategory {
    pub fn files(&self) -> Vec<&str> {
        let mut files: Vec<&str> = self.items
            .iter()
            .filter_map(|i| i.file.as_deref())
            .collect();
        files.sort();
        files.dedup();
        files
    }
}
```

This is what the CLI renders as `flag  45  (02-std, 03-cli, 07-runtime)`.

---

## 9. Research: How Others Do Categorized Output

### 9.1 rustc lint groups

Rustc groups warnings by lint name: `unused_variables`, `dead_code`,
`clippy::needless_return`. Each lint is a category. `--warn`, `--deny`,
`--allow` operate on categories, not individual occurrences.

Mirror parallel: categories map to error codes. `--strict` promotes all
warning-level codes to errors. No per-category suppression — loss is never
hidden. But per-category *acceptance* is possible (future: `accept M1001`
in the grammar means "I know about this, it's intentional").

### 9.2 Credo (Elixir)

Credo groups issues by category: `Consistency`, `Design`, `Readability`,
`Refactoring`, `Warning`. Within each category, issues are sorted by priority.
The output looks like:

```
  Warnings - pleaseass these issues:
  ┃
  ┃ [W] → Function is too complex  lib/foo.ex:42:3
  ┃ [W] → Function is too complex  lib/bar.ex:17:5
```

Mirror parallel: the fold IS the top-level category (parse, resolution,
property, emit). Within each fold, the keyword/ref/property name IS the
sub-category. Two levels of grouping, both derived from existing data.

### 9.3 ESLint

ESLint shows rule counts in its summary:

```
  2 problems (1 error, 1 warning)
  1 error and 0 warnings potentially fixable with the `--fix` option.
```

And groups by rule in `--format compact`:

```
  no-unused-vars: 15 occurrences
  no-console: 8 occurrences
```

Mirror parallel: the `--json` output provides exactly this. The CLI text
output shows it inline. No separate "compact" format needed — the default
IS compact.

---

## 10. Dynamic vs. Static Categories

**Decision: dynamic, derived from data.**

Static categories (an enum of all possible category names) would require
updating the enum every time a new keyword, grammar, or property is added.
This is a maintenance burden that contradicts the grammar-driven design.

Dynamic categories (derived from the strings already in the data) require
no maintenance. A new keyword `flag` in the source automatically creates
a "flag" category in the loss view. A new grammar `@io` creates an "@io"
category. The category set IS the content.

The error codes (M1001, M3001, etc.) are static — they identify the kind
of loss (unrecognized keyword, unresolved ref). The category name within
that code is dynamic — it identifies the specific instance.

```
M1001 "flag"      — static code, dynamic name
M1001 "command"   — same code, different name
M3001 "@io"       — different code, dynamic name
```

---

## 11. Implementation Plan

### Phase 1: LossItem + LossView (no trait changes)

1. Add `LossItem`, `LossLocation`, `LossFold`, `LossCategory`, `LossView`
   to `src/loss.rs`.
2. Implement `MirrorLoss::view()` as a projection.
3. Implement `LossView::fold()`, `LossView::category()`, `LossView::top()`.
4. Tests: view of zero loss is empty, view of single-category loss has one
   category, view of multi-category loss sorts by count, combine preserves
   categories.

### Phase 2: CLI output

1. Update `cmd_ci` (single file) to use `loss.view()` for categorized output.
2. Update `cmd_ci` (directory) to tag files and show aggregated categories.
3. Update `cmd_craft` to show categories with actionable hints.
4. Add `--json` flag to `cmd_ci`.

### Phase 3: LSP integration

1. Use `LossView` in `loss_to_diagnostics` to add category metadata to
   diagnostics (the `data` field in LSP Diagnostic).
2. Future: code actions generated from high-count categories.

### Phase 4: File attribution

1. Add `tag_file(name: &str) -> MirrorLoss` that stamps all items with a
   source file.
2. Use in multi-file compilation paths (`ci`, `craft`, `boot`).

---

## 12. What Does Not Change

- The `Loss` trait in `prism`/terni. No new methods.
- The `MirrorLoss` struct fields. No new fields — `view()` is a method.
- The `combine` behavior. Categories fall out of vector concatenation.
- The `holonomy()` computation. Same formula, same result.
- The `loss_to_diagnostics` function. It continues to work as-is.
  `LossView` is an additional decomposition, not a replacement.

---

## 13. Open Questions

1. **Should `view()` cache?** Currently proposed as computed-on-demand. If
   the CLI calls it once and the LSP calls it once, computing twice is fine.
   If something calls it in a loop, it should cache. Defer until profiling
   shows a need.

2. **Should categories carry suggested fixes?** The `craft` output shows
   hints like "add DeclKind::Flag to parser." These are not in `LossView` —
   they are in the CLI renderer. Should the view carry them? Probably not.
   The view is data. The hint is presentation. Keep them separate.

3. **Should `LossItem` carry a `Severity`?** Currently severity is determined
   by the error code and the `--strict` flag. Adding it to `LossItem` would
   duplicate the logic. Better to compute it at render time from the code
   and the CLI flags.

4. **How does `accept` work?** Future feature: `accept M1001 "flag"` in a
   grammar file means "this loss is intentional." The holonomy still counts
   it, but the CLI marks it as accepted. This interacts with categories but
   does not require changes to the `LossView` structure — acceptance is a
   filter applied at render time.

5. **Should `LossView` support `Ord` on `LossFold`?** The `BTreeMap` requires
   it. Current proposal: Parse < Resolution < Property < Emit (pipeline order).
   This matches the fold sequence and produces natural output ordering.
