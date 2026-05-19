# Template / Property Split

**Author:** Reed + Alex
**Date:** 2026-04-14
**Status:** Spec

---

## The Split

Two kinds of observation. One compiler.

```
template    observes structure     iso          kintsugi
property    observes effects       <= verdict   alignment
```

Templates check shape. Properties check effects. There is no third thing.

---

## Templates: Structure Observation

Templates are iso. They observe the surface form of declarations.
They don't touch effects. They don't return verdicts. They return
the same structure, canonicalized.

```mirror
template kintsugi(grammar) = iso {
  lens hoist
  lens sort_deps
  lens normalize
  lens align
}
```

A template answers: "is this structure canonical?" The answer is
binary — either the structure matches the canonical form or it doesn't.
No loss. No holonomy. Iso.

### What's a template (not a property)

Everything that checks structure without observing effects:

```
types_lowercase         structure: are type names lowercase?
canonical_order         structure: are declarations in the right order?
no_cycles               structure: is the dependency graph acyclic?
no_dead_variants        structure: are all variants reachable?
unique_variants         structure: are variant names unique?
every_type_reachable    structure: can all types be reached?
dual_partition          structure: is the type graph bipartite?
```

These are all kintsugi lenses. They observe shape. They're iso.

### Template in the CLI

```
mirror craft . --kintsugi       run all templates, canonicalize
mirror craft . --kintsugi --check   check without writing
```

---

## Properties: Effect Observation

Properties observe effects. They pattern match on the optic shape
of an effect's type signature. They fold into verdict. They return
`Imperfect` because effects are lossy. The loss is measured.

```mirror
property human_intervention(effect(ai => human)) <= verdict {
  requires witnessed
  ensures consented
}
```

The argument to a property IS an optic. The property fires on every
effect whose type signature matches the pattern. Structural pattern
matching on the effect graph.

### The argument is a pattern

```mirror
property name(effect(A => B)) <= verdict         -- any effect from A to B
property name(effect(A => B)) where A != B        -- only partition crossings
property name(effect(any => production))           -- any deployment
property name(effect(ai => real))                  -- any AI touching reality
property name(effect(a => a))                      -- endomorphisms (idempotence)
```

The compiler traverses all effects in the grammar. For each effect,
it matches the type signature against the property's argument pattern.
If it matches, the property fires. The result is `Imperfect<Declaration, PropertyError, PropertyLoss>`.

### What's a property (not a template)

Everything that observes effects:

```
human_intervention(effect(ai => human))       consent at AI/human boundary
consent_required(effect(a => b)) where a != b  consent at any partition
always_halts(effect(any => any))               bounded execution
idempotent(effect(a => a))                     same result on re-apply
deterministic(effect(a => a))                  same input → same output
pure(effect(any => any))                       no mut in effect graph
```

These all pattern match on effects. They fold to verdict. They're not iso —
they measure loss.

### Property in the CLI

```
mirror craft . --ci             run all properties, report verdicts
mirror craft . --ci --strict    Partial → Failure
```

---

## The Composition

```
mirror craft .                   templates + properties + emit
mirror craft . --kintsugi        templates only
mirror craft . --ci              properties only
mirror craft . --check           no output, just check (exit code)
mirror craft . --strict          Partial → Failure
```

Templates and properties compose in the same pipeline:

```
source → parse → resolve → templates → properties → emit
                            (structure)  (effects)    (crystal)
```

Templates run first — they canonicalize the structure before properties
observe it. Properties run on canonical structure. This means:

1. A property never fires on non-canonical code
2. Kintsugi is a prerequisite for property checking
3. The canonical form IS the form that properties observe

### Pipeline as optic composition

```mirror
command craft = parse . resolve . templates . properties . emit

-- flags select which optics to include:
-- --kintsugi:  parse . resolve . templates
-- --ci:        parse . resolve . templates . properties
-- --check:     parse . resolve . templates . properties (no emit)
-- (no flags):  parse . resolve . templates . properties . emit
```

---

## The Type Surface

### Template type

```mirror
type template(grammar, block) = iso

-- a template is:
--   parameterized by grammar and block
--   iso (reversible, lossless)
--   pure (no effects)
```

### Property type

```mirror
type property(effect_pattern) = effect_pattern <= verdict

-- a property is:
--   parameterized by an effect pattern (an optic)
--   folds to verdict via <=
--   the pattern is structurally matched against all effects
```

### Verdict type

```mirror
type verdict = imperfect(declaration, property_error, property_loss)

-- verdict IS imperfect:
--   Success(declaration)           property holds
--   Partial(declaration, loss)     property holds with measured deviation
--   Failure(error, loss)           property violated
```

---

## Effect Pattern Matching

The property argument is a structural pattern on an effect's optic.
The compiler matches it.

### Match rules

```
effect(A => B)              matches any effect from type A to type B
effect(A => B) where A != B matches only partition crossings
effect(any => any)          matches all effects
effect(A => A)              matches endomorphisms (self-referential)
effect(_ => production)     matches any effect targeting production
```

### Resolution

The compiler:

1. Collects all `action` and `io` declarations in the grammar
2. Extracts each effect's type signature (input => output)
3. For each property, matches the property's pattern against each effect
4. For each match, evaluates the property body
5. Returns `Imperfect<Declaration, PropertyError, PropertyLoss>` per match
6. Accumulates into `PropertyLoss` in `MirrorLoss`

### Example: alignment across a codebase

```mirror
property alignment(effect(ai => human)) <= verdict {
  requires witnessed
  ensures consented
}
```

The compiler finds:
```
action suggest(query) -> recommendation     ai => human? check types...
  query: @ai.query                           ai ✓
  recommendation: @human.review              human ✓
  MATCH → fire property → requires witnessed → check...

action compile(source) -> crystal            ai => ai?
  source: @mirror.source                     not human
  crystal: @mirror.crystal                   not human
  NO MATCH → skip

action deploy(shard) -> production           any => production?
  not ai => human
  NO MATCH for alignment, but matches deployment properties
```

---

## Boot Integration

### 05-property.mirror becomes two sections

```mirror
grammar @property {
  in @meta

  -- templates (structure observation, iso)
  template types_lowercase(grammar) = iso
  template no_cycles(grammar) = iso
  template unique_variants(grammar) = iso
  template every_type_reachable(grammar) = iso
  template no_dead_variants(grammar) = iso
  template dual_partition(grammar) = iso
  template canonical_order(grammar) = iso

  -- properties (effect observation, <= verdict)
  property idempotent(effect(a => a)) <= verdict
  property deterministic(effect(a => a)) <= verdict
  property pure(effect(any => any)) <= verdict
  property always_halts(effect(any => any)) <= verdict
  property action_is_named_type(effect(any => any)) <= verdict
}
```

Templates and properties in the same grammar. Same `@property` grammar.
Different declaration keywords. Different semantics.

### 10-mirror.mirror uses both

```mirror
grammar @mirror {
  in @meta
  in @property

  -- structural requirements (templates)
  requires unique_variants
  requires every_type_reachable
  requires no_dead_variants
  requires types_lowercase
  requires canonical_order

  -- effect requirements (properties)
  invariant idempotent
  invariant deterministic
  invariant pure
  invariant no_cycles
  ensures always_halts
}
```

`requires` for templates (must hold at compile time).
`invariant` for properties (must hold throughout execution).
`ensures` for properties (must hold after execution).

The lifecycle keywords (`requires`, `invariant`, `ensures`) apply to both
templates and properties. The distinction is in the declaration, not the usage.

---

## Red Tests

The following tests should be written RED to drive implementation:

```
test template_types_lowercase_is_iso
  - types_lowercase declared as template, not property
  - returns iso result, not verdict

test property_matches_effect_pattern
  - property with effect(ai => human) pattern
  - fires on matching effects, skips non-matching

test property_does_not_fire_on_template
  - templates don't produce verdicts
  - properties don't produce canonical forms

test kintsugi_runs_before_properties
  - in the pipeline, templates run first
  - properties observe canonical structure

test alignment_property_matches_ai_human_effects
  - the alignment property fires on effects crossing the ai/human boundary
  - does NOT fire on ai/ai or human/human effects
```

---

## Summary

Two observations. One compiler.

```
template    shape     iso          kintsugi
property    effect    <= verdict   alignment
```

Templates are the formatter. Properties are the checker.
The formatter runs first. The checker observes canonical structure.
The pipeline composes them. The CLI selects them.
The type system distinguishes them.

One `Imperfect`. All the way down.
