# Kintsugi: Unified Compiler Surface

**Author:** Reed + Alex
**Date:** 2026-04-14
**Status:** Design

---

## Summary

Mirror unifies what other ecosystems split across four tools (compiler, formatter,
linter, type checker) into one pass, one type, one holonomy. The formatter is named
**kintsugi** — the art of repairing with gold, where the seams are visible.

The CLI is a generic optic compositor. Commands are optic compositions. Flags are
optics. The grammar declares them. The Rust runtime applies them.

---

## The Type Surface

```mirror
type pure = iso
type real = mut(block)
pure != real

type observation = pure
type template(grammar, block) = pure
type effect = real
type io(effect) = real
type action(grammar, effect) = real

type error(observation)
type loss = pure - real
type growth = real - pure

type mut(block) != iso

type imperfect(observation, error(observation), loss) {
  recover |observation, loss| <= imperfect
  rescue |error(observation), loss| <= imperfect
}
```

Five tiers of purity:

| Type | Purity | License |
|------|--------|---------|
| `observation` | pure | Apache-2.0 |
| `template` | pure, iso | Apache-2.0 |
| `effect = mut(block)` | impure | declared |
| `io(effect)` | impure | declared |
| `action(grammar, effect)` | impure | SEL |

One axiom: `pure != real`. One keyword: `mut`. Everything derives.

---

## The Operator Table

Ten operators. The complete relational algebra:

| ASCII | Unicode | Name | Meaning |
|-------|---------|------|---------|
| `=` | `≡` | Iso | these are the same |
| `!=` | `≢` | NotIso | these are not the same |
| `<=` | `⇐` | Fold | left receives right (with loss, double = heavy) |
| `=>` | `⇒` | Unfold | right produces left (double = heavy) |
| `<` | `⊂` | Subset | proper subset |
| `>` | `⊃` | Superset | proper superset |
| `\|` | `\|` | Split | branching / variants (unchanged) |
| `()` | `()` | Focus | grouping / parameters (unchanged) |
| `->` | `→` | Zoom | flow / transformation (single = light) |
| `<-` | `←` | Zoom | reverse flow / pull (single = light) |
| `..` | `‥` | Refract | spread / settlement |
| `.` | `∘` | Compose | optic composition |

Kintsugi hoists ASCII operators to unicode canonical form. The spectral hash
doesn't change — same eigenvalues, same OID. Operators hoist. Delimiters
(`{ }`, `()`) don't. If it expresses a relation, it gets unicode. If it
contains structure, it stays ASCII.

Inline relation markers in type declarations:

```mirror
type user { name: text, email: text, age: nat }
type contact { <user, name: text, email: text }
type admin { >user, name: text, email: text, age: nat, role: text }
type alias { =user }
type action { !=observation }
type mortal { <=human }
```

Six markers. Six transformation semantics:

| Marker | Relation | Transform | Recovery |
|--------|----------|-----------|----------|
| `<` | subset | project: lossless | dead code (never fires) |
| `>` | superset | embed: needs data | default provider |
| `=` | iso | rename: lossless | dead code |
| `!=` | partition | blocked: type error | illegal |
| `<=` | fold | lossy: measured | loss observer |
| `>=` | unfold | expand: needs data | reverse provider |

---

## Kintsugi: The Formatter

Kintsugi is a template optic. `= iso`. Lossless. The spectral hash doesn't change.
The OID stays the same. The surface changes, the content doesn't.

### Canonical Order

```
1. in        imports first (what you depend on)
2. type      types next (what you define)
3. template  templates next (pure transforms)
4. grammar   grammars next (what you compose)
5. property  properties next (what you verify)
6. action    actions last (what you do)
```

Observation before action. The same order as the license split.

### Optic Composition

```
kintsugi = hoist . sort_deps . normalize . align
```

| Lens | What it does |
|------|-------------|
| `hoist` | Reorder declarations by kind (canonical order) |
| `sort_deps` | Within each kind, sort by dependency graph |
| `normalize` | Fix spacing around operators |
| `align` | Align split pipes vertically |

### The Kintsugi Property

```mirror
property kintsugi(grammar) <= verdict {
  template fmt
  iso equal
  refract verdict
}
```

Apply the formatter. Compare to original. If equal: Success (already golden).
If different: Partial (here's where the gold goes). Never Failure.

### KintsugiLoss

```rust
pub struct KintsugiLoss {
    pub hoisted: Vec<(String, usize, usize)>,  // (name, from_line, to_line)
    pub reordered: Vec<(String, String)>,        // (before_dep, after_dep)
    pub normalized: usize,                        // spacing fixes
}
```

The loss tells you what the gold filled. The seams are visible.

---

## Properties as Credo

Properties are folds that return `Imperfect<Declaration, PropertyError, PropertyLoss>`.
Not `Imperfect<(), String, f64>`. The observation is always present.

### PropertyVerdict

```rust
pub struct PropertyVerdict {
    pub property: String,
    pub verdict: Imperfect<Declaration, PropertyError, PropertyLoss>,
}

pub struct PropertyError {
    pub observation: Declaration,   // the AST node that failed
    pub property: String,           // which property
    pub context: Vec<Declaration>,  // surrounding nodes for span rendering
}

pub struct PropertyLoss {
    pub deviation: f64,             // how far from passing
    pub frequency: Option<f64>,     // for Collector properties: majority ratio
}
```

### Two Kinds of Properties

**Structural** — declared in boot, always run:

```mirror
property types_lowercase(grammar) <= verdict
property no_cycles(grammar) <= verdict
property kintsugi(grammar) <= verdict
```

**Discovered** — Collector pattern, aggregated across files:

```mirror
property consistent_naming(grammar) <= verdict {
  traversal names
  lens frequency
  refract verdict
}
```

Phase 1: fold across all files, aggregate frequency data.
Phase 2: fold per file, compare against aggregate.
No hardcoded rule. The codebase defines its own convention.

### spec.mirror Overrides

```mirror
properties {
  default all
  consistent_naming = snake_case    -- override discovery
  kintsugi = required               -- not advisory
  ignore no_dead_variants           -- opt out
}
```

Three operations: default set, explicit value, ignore.
Typo in property name = compile error (type-checked).

### Priority IS Holonomy

No A/B/C/D priority levels. The holonomy IS the priority. A property producing
holonomy 3.0 is more urgent than one producing 0.1. The measurement IS the ranking.

---

## The CLI as Optic Compositor

The CLI is a generic optic applier. Commands are compositions. Flags are optics.
The grammar declares them. The Rust runtime applies them.

### Grammar Declaration

```mirror
grammar @cli {
  in @meta

  flag strict = prism(imperfect => success | failure)
  flag format(json | human) = lens(imperfect => text)
  flag check = prism(imperfect => pass | fail)
  flag verbose = lens(loss => text)

  command compile = parse . resolve . emit
  command kintsugi = parse . resolve . canonical_order
  command ci = parse . resolve . properties . emit
  command focus = parse
  command explain(code) = catalog . render
}
```

### Composition

Each pipeline step is a fold returning `Imperfect`. They compose via `eh`:

```
parse:       source -> Imperfect<AST, ParseError, ParseLoss>
resolve:     AST -> Imperfect<Resolved, ResolveError, ResolutionLoss>
properties:  Resolved -> Imperfect<Verdicts, PropertyError, PropertyLoss>
emit:        Resolved -> Imperfect<Crystal, EmitError, EmitLoss>
kintsugi:    Resolved -> Imperfect<Canonical, (), KintsugiLoss>
```

### Flags as Optics

`--strict` is a Prism: projects `Imperfect` into `Success | Failure`.
`--format json` is a Lens: focuses on `Imperfect`, renders as text.
`--check` is a Prism: pass or fail exit code, no output.
`--verbose` is a Lens: focuses on loss detail.

New flags are declared in the grammar, not hardcoded in Rust.

### Exit Codes

| Result | Default | --strict |
|--------|---------|----------|
| Success | 0 | 0 |
| Partial | 0 | 1 |
| Failure | 1 | 1 |

### Output Format

```
Success:  compiled path -> crystal:oid    holonomy: 0.0
Partial:  compiled path -> crystal:oid    holonomy: 1.3  (loss breakdown)
Failure:  failed path                     error[M2001]: ...
```

`--format json` serializes `MirrorLoss` directly. The JSON IS the optic traversal.

---

## Architectural Decisions

### Form dissolves into Fractal

The `Form` struct is a parallel AST. `Fractal<MirrorData>` is the content-addressed
tree. There should be one representation. Parse directly into `Fractal`. The declaration
IS the fragment. The optic IS the content address.

`Fractal<D>` implements the optic hierarchy:
- `Lens<Fractal<D>, D>` — focus on data (always present)
- `Prism<Fractal<D>, Fractal<D>>` — project into child by OID
- `Traversal<Fractal<D>, D>` — walk all nodes
- `Fold<Fractal<D>, D>` — read-only observation

### Declaration fields use Imperfect, not Option

`Option<String>` is binary — present or absent. `Imperfect<String, (), Loss>` is
ternary — present, partially present, or absent with measured loss.

```
grammar_ref: Option<String>  ->  Imperfect<String, (), RefLoss>
body_text: Option<String>    ->  Imperfect<String, (), ParseLoss>
return_type: Option<String>  ->  Imperfect<String, (), ResolutionLoss>
```

### `form` keyword produces deprecation warning

The `form` keyword still compiles (backward compatibility) but produces `Partial`
with deprecation loss. The grammar says `grammar`, not `form`.

### Loss is never suppressed

No `#[allow(...)]`. No ignore annotations on individual lines. Loss is a measurement.
You can accept it via `spec.mirror` overrides. You cannot hide it.

### Holonomy in every output

Even Success shows `holonomy: 0.0`. The measurement always happens. Zero is meaningful.

---

## Red Tests (Current Specifications)

| Test | Status | What it specifies |
|------|--------|------------------|
| `form_keyword_produces_warning` | RED | `form` must produce Partial |
| `mirror_ci_boot_success` | IGNORED | All 18 boot files resolve, zero loss |
| `declaration_fields_not_option` | GREEN (baseline) | Documents Option gap |
| `compile_returns_fractal_not_form` | GREEN (baseline) | Documents Form/Fragment duplication |

---

## What This Replaces

| Elixir tool | Mirror equivalent | How |
|-------------|------------------|-----|
| `mix compile` | `mirror compile .` | parse . resolve . emit |
| `mix format` | `mirror kintsugi .` | parse . resolve . canonical_order |
| `mix credo` | `mirror ci .` | properties fold (structural + discovered) |
| `dialyzer` | `mirror ci .` | resolution fold (type checking) |
| `.formatter.exs` | `spec.mirror` | type-checked, content-addressed |
| `.credo.exs` | `spec.mirror` | properties block |
| `mix format --check` | `mirror kintsugi --check` | prism: pass or fail |
| `mix credo --strict` | `mirror ci --strict` | prism: Partial becomes Failure |

One command. One type. One holonomy. `mirror ci .`
