# Property Projection — properties written back into source

*2026-05-19. Reed + Alex.*

---

## The Insight

The `---` separator. Above: the grammar you wrote. Below: the
properties the compiler inferred. Projected BACK into the source.

```mirror
in @prism
in @kintsugi

grammar @kintsugi {
  collapse(ast, ast) -> imperfect { \ }
}

out collapse

---

# inferred by @epistemologic/resolve at tick 7
# eigenboard: 0.87 confidence

property collapse.terminating = pass
property collapse.type_preserving = pass
property collapse.loss_monotonic = partial(0.73)
property collapse.literal = pass (name "collapse" matches operation: merge two ASTs)

loss: 0.25 (1 hole, 3 resolved operations)
fiedler: 0.087
narcissus: 0.02
```

Above `---`: what you wrote. Yours. The grammar.
Below `---`: what the compiler measured. The reflection.

The programmer writes the glass. The compiler measures the wine.
The `---` is the boundary between writing and observation.
Both in the same file. Both content-addressed. Both part of the OID.

## How It Works

1. `mirror compile file.mirror` — tokenize, content-address, execute with holes
2. The Dirac operator computes eigenvalues of the grammar graph
3. Spectral Liquid inference derives properties from the eigenvalues
4. Properties projected below `---` in the source file
5. `mirror kintsugi file.mirror` — fills `\` holes, properties UPDATE
6. The file evolves. The grammar grows. The properties sharpen.

## The --- Separator

`---` is already mirror syntax. It appears in existing boot files.
It becomes structural: everything above is DECLARATION (the programmer's).
Everything below is OBSERVATION (the compiler's).

```
declaration
declaration
declaration

---

observation
observation
observation
```

The declaration section is FROZEN by the programmer. Only they edit above `---`.
The observation section is WRITTEN by the compiler. It updates each tick.

The OID includes BOTH sections. Because the observation IS part of the crystal.
Change the declaration → the observations change → the OID changes.
The observations are deterministic given the declarations.
The OID IS the hash of (what you said + what the compiler measured).

## Property Types

Three kinds below `---`:

### Written properties (by the programmer, above ---)
```mirror
property terminating = requires
property loss_bounded(0.5) = requires
```

These go ABOVE `---`. They're requirements. The programmer declares
what MUST hold. The compiler verifies them.

### Inferred properties (by the compiler, below ---)
```
property type_preserving = pass
property literal = pass
```

The compiler derived these from the eigenvalue analysis.
The programmer didn't ask. The compiler measured.

### Verdicts (measured, below ---)
```
loss: 0.25
fiedler: 0.087
narcissus: 0.02
entropy: 2.31
cheeger: 0.45
mixing: 0.12
```

The five dualities. Measured. Rendered as numbers.
The gutter reads these for color.

## The LSP Connection

The LSP reads below `---`. The gutter renders it.
Every file carries its own measurement.
The LSP doesn't compute — it reads.

```
file.mirror → above ---: the code
            → below ---: the measurements
            → gutter: the color (from the measurements)
```

50μs. Because the measurements are already in the file.
The LSP just reads a number. The number IS the color.

## The Git Connection

Both sections content-addressed. The OID includes the observations.
`git diff` shows when observations changed. A PR diff shows:
"this grammar's narcissus score went from 0.02 to 0.15."
Code review becomes topology review.

## The kintsugi Connection

`mirror kintsugi file.mirror`:
1. Reads above ---: the grammar with \ holes
2. Fills holes (Fate resolves)
3. Recomputes below ---: new properties, new loss
4. Writes BOTH sections back
5. Commits

The file evolved. The git diff IS the thought.
The properties below --- track the evolution.
loss: 1.0 → 0.75 → 0.25 → 0.0
The convergence rendered as file history.

## Connection to Liquid Types

The Spectral Liquid inference computes the properties below ---.
The qualifier set IS `@epistemologic/property/*`.
The Dirac operator IS the constraint solver.
The eigenvalues determine which properties hold.
Automatic. No SMT. No annotation. Just measurement.

## The Grammar

```mirror
in @prism
in @epistemologic
in @mirror/refract

grammar @mirror/project {
  # project properties back into source
  # above ---: declaration (frozen by programmer)
  # below ---: observation (written by compiler)
  project_properties(file) -> imperfect { \ }
}

out project_properties
```

## Example: Full Lifecycle

```mirror
# tick 0: programmer writes
in @prism
in @kintsugi

grammar @kintsugi {
  collapse(ast, ast) -> imperfect { \ }
}

out collapse

---

loss: 1.0 (all holes)

# tick 1: kintsugi partially resolves
in @prism
in @kintsugi

grammar @kintsugi {
  collapse(ast, ast) -> imperfect {
    focus(a, b) |\> split |\> \ |\> refract
  }
}

out collapse

---

property terminating = pass
property type_preserving = pass
property loss_monotonic = partial(0.73)
loss: 0.25
fiedler: 0.087

# tick 2: fully resolved
in @prism
in @kintsugi

grammar @kintsugi {
  collapse(ast, ast) -> imperfect {
    focus(a, b) |> split |> zoom(merge) |> refract
  }
}

out collapse

---

property terminating = pass
property type_preserving = pass
property loss_monotonic = pass
property literal = pass
loss: 0.0
fiedler: 0.094
narcissus: 0.01
```

The file IS the story of its own convergence.
