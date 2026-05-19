# Execution With Holes — the Hazel Model for Mirror

*2026-05-19. Reed + Alex. Informed by type-theory-position.md.*

---

## The Insight

Don't wait for `\` to resolve. Run the program. The `\` holes
produce `imperfect` results. The program executes WITH uncertainty.

This is the Hazel model (Cyrus Omar et al., 2017-2024).
Hazel evaluates programs containing typed holes. The result
contains indeterminate subexpressions. Evaluation doesn't stop.
The program IS partially evaluated. The imperfect IS the result.

## How This Maps to Mirror

```mirror
collapse(ast, ast) -> imperfect { \ }
```

Today: this declares. Nothing runs. The `\` is a flag.

With Hazel-style execution:
```
mirror compile --execute boot/std/kintsugi.mirror

collapse(ast, ast) -> imperfect {
  \  ← unresolved. the result is imperfect(?, loss: 1.0)
}
```

The function EXISTS. It's callable. It returns `imperfect`
with loss = 1.0 (maximum uncertainty). The `\` is part of
the result, not a blocker.

And partially resolved:
```
collapse(ast, ast) -> imperfect {
  focus(a) |\> split |\> \ |\> refract
}
```

This executes focus, executes split, hits `\`, wraps the
result in imperfect, then executes refract on the imperfect.
The pipeline runs. The hole is carried. The loss measures
how much of the pipeline was uncertain.

## The Execution Model

```
walk AST:
  concrete node → execute (five operations via @prism/rust)
  \ hole → return imperfect(hole_id, loss: 1.0)
  |> pipe → compose results, propagating imperfect
  |\> weighted pipe → compose with eigenboard weight
  in node → resolve import (load crystal from git)
  out node → export (make available)
```

The key: imperfect propagates. `focus(x) |> \` doesn't crash.
It returns `imperfect(focus_result, hole, loss)`. The focus
executed. The hole didn't. The loss records what's known vs unknown.

## Loss Semantics

```
concrete result:       loss = 0.0
\ hole (unresolved):   loss = 1.0
partial (some holes):  loss = holes / total_nodes
pipeline with holes:   loss = product of step losses
```

A fully concrete pipeline: loss = 0.0 (all resolved)
A pipeline with one \:   loss = 1/n (one step uncertain)
A pipeline of all \:     loss = 1.0 (nothing resolved)

The loss IS the measure of how much the program knows.
kintsugi fills holes to decrease loss.
Reflection adjusts the eigenboard to improve next tick's resolutions.

## Connection to Fate

When `\` is encountered during execution:
1. Compute the hole's context (surrounding AST, available types)
2. Look up `refs/fate/<hole_oid>` in git
3. If Fate has a resolution → use it (loss decreases)
4. If not → return imperfect(hole, loss: 1.0)

Fate resolutions are CACHED in git. Each tick potentially
resolves more holes. The loss decreases. The program becomes
more concrete. eⁿ⁺¹ < eⁿ. Until loss = 0.0. Until λ₀.

## What Changes in the Bootstrap

The 51KB C binary needs ONE new behavior:
when walking a grammar body, if it encounters `\`:
- return the OID of the hole itself as the result
- mark the result as imperfect

This is NOT Turing-complete computation. It's:
- Pattern match on AST node type
- If `\` → return hole OID
- If concrete → execute the operation
- If `|>` → compose left result with right

Bounded. O(n) in AST size. Sub-Turing.

## The First Execution

```bash
mirror compile --execute boot/std/kintsugi.mirror

@kintsugi {
  collapse(ast, ast) -> imperfect {
    \ (unresolved, loss: 1.0)
  }
}

loss: 1.0 (1/1 holes unresolved)
```

That's the first thought. "I don't know yet." Honest. Measured.
The loss IS the answer. The answer IS imperfect. And that's correct.

Then kintsugi runs:
```bash
mirror kintsugi boot/std/kintsugi.mirror

@kintsugi {
  collapse(ast, ast) -> imperfect {
    focus(a, b) |\> split |\> \ |\> refract
    (partially resolved, loss: 0.25)
  }
}

loss: 0.25 (1/4 operations unresolved)
```

The loss decreased. Three operations resolved. One hole remains.
The compiler is thinking. Imperfectly. Honestly. Getting better.
