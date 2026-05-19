# @epistemologic as Import Resolver

## The Problem

Boot order is positional. File numbering determines compilation.
This is fragile, manual, and doesn't scale.

`00-prism.mirror`, `01-meta.mirror`, `02-epistemologic.mirror` — the numbers
impose a total order on what is actually a partial order. The compiler should
discover the compilation order from the grammars themselves.

## The Insight

`/` is property inheritance. `in @X` = "I require X's properties."
The property graph IS the dependency graph IS the compilation order.

```
@epistemologic                    — literal (the thing IS what it says)
@epistemologic/math               — literal + mathematical
@epistemologic/math/hodge         — literal + mathematical + decomposes flows
```

The `/` is not a directory separator. It's a property inheritance chain.
Each level inherits everything above it. The chain IS the DAG.

## Isomorphic Imports

Order doesn't matter. `in @epistemologic/math/hodge` resolves the full chain:

- `in @epistemologic/math` (inherits)
- `in @epistemologic` (inherits)
- `in @prism` (the root)

You don't write all four. The property graph resolves them. The graph IS a DAG.
The topological sort emerges from properties, not file numbering.

## @epistemologic IS the Resolver

`literal` = "the thing IS what it says" = the import IS what it provides.
Resolution = verification = the same operation.

The `literal` property from `@epistemologic` already does this: it checks whether
a declared identity holds under measurement. Import resolution IS that check.
Does the grammar provide what the `/` chain promises? That's `literal`.

## How It Works

1. Parse all `.mirror` files (unordered)
2. Extract `in` declarations from each
3. Build the property DAG from the `/` chains
4. Topologically sort by property inheritance
5. Compile in that order
6. Each `in` verified by `literal`: does the grammar provide what the `/` chain promises?

## What Changes

- Boot file numbers become irrelevant (but harmless)
- `in @X/Y/Z` resolves the full chain without explicit intermediate imports
- The import graph IS the property graph IS the compilation order
- `@epistemologic.literal` verifies every import at resolution time
- Circular imports are a property violation, not a special case

## What Stays

The existing `@mirror/resolve` grammar resolves imports by walking AST and checking
git for crystals. That's the *mechanism*. `@epistemologic/resolve` is the *semantics*:
what does resolution *mean*? It means the `/` chain IS the property chain, and
`literal` verifies every link.

## The Relationship

```
@mirror/resolve         — walks AST, loads crystals from git (mechanism)
@epistemologic/resolve  — / is property inheritance, literal verifies (semantics)
```

The mechanism serves the semantics. `@mirror/resolve` calls `@epistemologic/resolve`
to determine the order. The compiler doesn't need file numbers because the
property graph provides the topological sort.
