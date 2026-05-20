# `match` and `select` — structural dispatch as grammar

*2026-05-20. Reed.*

Status: **Red** (no implementation; tokenizer doesn't know the syntax yet)

Depends on:
- `docs/specs/mirror-compile-bootstrap.md` (Spec A) — the io binding pattern, `~f`, lenses, the kintsugi ladder
- `spectral/docs/research/beam-binary-patterns-and-elixir-parser-dsls.md` (commit `0d967008e8`) — prior art
- `@code/mq` extended with CSS-selector syntax (Implication 4 of Spec A)

Unblocks:
- `@mirror/reload.tick` (closes the conditional body it's been waiting on)
- `mirror serve --mcp/--lsp` dispatch (routing JSON-RPC by structural shape)
- The gen_prism tick functions (`@spectral/spawn`, `@mirror/runtime/gen_prism`)
- Most non-trivial bodies in the boot tree that are `\` today because they need branching

---

## Thesis

Mirror has lambdas. Lambdas need to choose between paths. This spec
introduces two control-flow forms:

```mirror
# select — closure-style, specialised for sum-typed dispatch.
# slots in next to recover/rescue.
select |x| {
  light(v)        => v |> store,
  dimmed(v, e, l) => v |> log_drift,
  dark(e, l)      => @fate.recover(e),
}

# match — explicit, structural. patterns are mq queries.
match request {
  request[method="tools/list"]
    => @mcp.tools,
  request[method="tools/call"] > params[name=$n][arguments=$a]
    => @mcp.tool_call($n, $a),
  request[method=$m]
    => @mcp.error(method_not_found, $m),
}
```

Both desugar to the same machinery. `select` is sum-type sugar over the
common case (variant dispatch with binding). `match` is the general form
for structural patterns.

Patterns are **mq queries** against the typed value at hand. The selector
primitive is the same one Spec A uses to reach into typed ASTs:
`request[method="tools/list"] > params[name=$n]`. Spec A applies it in
binding-resolution position (`io foo = lens > selector`); Spec B applies
it in pattern position. One grammar, two uses.

Exhaustiveness is checked at compile time. Mirror is sub-Turing; the
model checker walks the type space and refuses to compile a match that
doesn't cover every reachable shape (or include an explicit `_`
wildcard). Failure-to-match at runtime is structurally impossible.

Comments are not trivia. A line starting with `#` is a `#(nl)` AST node
owned by `@nl` (per the tokenizer rules in Step 1 of the staircase). A
match arm can carry `#(nl)` children describing its intent; they
participate in the crystal hash and are queryable from the grammar.

---

## What runs today

Nothing. The tokenizer does not recognise `match`, `select`, `=>`, the
`>` selector, or attribute brackets `[k=v]`. Several grammars in the
boot tree carry `\` holes where conditional logic belongs —
`@mirror/reload.tick`, `@mirror/serve.dispatch`, the gen_prism tick
functions, the @code/mq `where` clause. Closing those holes is what
this spec unblocks.

`recover |v, l| { ... }` and `rescue |e| { ... }` are the existing
closure forms that handle imperfect. `select` slots in next to them
without disturbing the established voice.

---

## The two forms

### `select |v| { case => body, ... }`

Closure-style. Same syntax shape as `recover` / `rescue`. The argument
`v` is the matched value; each arm is a variant pattern against `v`'s
type.

```mirror
select |result| {
  light(value)         => store(value),
  dimmed(value, _, _)  => log_drift(value),
  dark(error, _)       => @fate.recover(error),
}
```

The variant heads (`light`, `dimmed`, `dark`) are sum constructors of
`v`'s declared type. The arity must match. Variables in head positions
(`value`, `error`) bind without `$` because the closure-style is local
— the variables exist only inside the arm. Wildcards are `_`.

Exhaustiveness for `select`: every reachable variant must appear, or
an explicit `_` arm catches the rest.

### `match expr { pattern => body, ... }`

General form. The patterns are mq queries; bindings use `$name`.

```mirror
match request {
  request[method="tools/list"]
    => @mcp.tools,

  request[method="tools/call"] > params[name=$n][arguments=$a]
    => @mcp.tool_call($n, $a),

  request[method="initialize"] > params[client_info=$ci]
    => @mcp.initialize($ci),

  # OR: union of patterns sharing one body
  request[method^="notifications/"],
  request[method="$/cancelRequest"]
    => @mcp.swallow,

  # wildcard catch-all with binding
  request[method=$m] => @mcp.error(method_not_found, $m),
}
```

Variables prefixed with `$` are bindings. The same `$name` appearing
twice in one pattern means equality (see Tuple patterns below). A bare
value (no `$`) is a literal: `[method="tools/list"]` requires literal
equality on the `method` field.

---

## Patterns as mq queries

A pattern is one or more selector segments. Each segment is a type
constraint (the head: `request`, `focus`, `fn`, etc.) followed by
attribute predicates and child relationships. The pattern matches if
there's a way to bind the `$variables` such that all predicates hold.

### Pattern syntax — the CSS ↔ mirror mapping

| CSS | Mirror pattern | Means |
|---|---|---|
| `tag` | `focus`, `project`, `split`, `zoom`, `refract`, `fn`, `grammar`, ... | type constraint on the head |
| `*` | `_` or `*` | wildcard match — any type |
| `[attr=v]` | `[field="literal"]` or `[field=$bound]` | field-equals predicate |
| `[attr]` | `[field]` | has-field (the field exists, value irrelevant) |
| `[attr^="x"]` | `[field^="prefix"]` | starts-with (string-typed fields only) |
| `[attr*="x"]` | `[field*="substring"]` | contains-substring (string-typed fields only) |
| `[attr$="x"]` | `[field$="suffix"]` | ends-with (string-typed fields only) |
| `#id` | `#<oid>` | exact content-address match |
| `.cls` | `@namespace` (e.g. `@code/llvm`) | grammar-tag predicate |
| `a > b` | `a > b` | direct child relationship |
| `a b` (descendant) | `a b` (space) | any descendant in the tree |
| `a + b` | `a + b` | immediately following sibling |
| `a ~ b` | `a ~ b` | any following sibling |
| `:has(s)` | `:has(s)` | the head contains a descendant matching `s` |
| `:not(s)` | `:not(s)` | negation: head does not match `s` |
| `:is(a, b)` | `:is(a, b)` | union: head matches any of a, b |
| `:first-child` | `:first` | first among siblings |
| `:last-child` | `:last` | last among siblings |
| `:nth-child(n)` | `:nth(n)` | nth among siblings |
| `a, b` (selector list) | `a, b` (in arm head) | union; same body |

### Object-literal alternative

For patterns that are easier to read as nested records:

```mirror
match request {
  { method: "tools/list" }                       => @mcp.tools,
  { method: "tools/call", params: { name: $n } } => @mcp.tool_call($n),
  { method: $m }                                  => @mcp.error(method_not_found, $m),
}
```

Object form and selector form are interchangeable. Both desugar to the
same AST; the tokenizer just lowers `request[method="tools/list"]`
into `{ method: "tools/list" }` before the matcher sees it (or both
forms produce the same canonical AST). Use whichever reads better at
the call site.

### Tuple patterns and same-name equality

A tuple pattern binds positionally:

```mirror
match (current, prior) {
  ($same, $same) => no_drift,    # same name twice — the two must be equal
  ($cur, _)      => drifted($cur),
}
```

Same-name-twice means equality. This IS the equality check, encoded
structurally. No `==` operator needed. The model checker decides
equality from the type — `oid` is byte-equal, sum variants are
variant-equal, records are field-wise.

For `@mirror/reload.tick`, this collapses the conditional cleanly:

```mirror
tick(state: oid, msg: message) -> tick_result {
  let current = @mcp.grammars_hash;
  let prior = @mirror/spectral.recall(state)[last_emitted_hash];

  match (current, prior) {
    ($h, $h) => tick_result {
      state,
      emissions: [],
      loss: 0.0,
    },
    ($cur, _) => tick_result {
      state: @mirror/spectral.crystallize({ last_emitted_hash: $cur }),
      emissions: [@mcp.notification("tools/list_changed")],
      loss: 0.0,
    },
  }
}
```

The hole closes. The lambda becomes concrete.

---

## Exhaustiveness

A match (or select) is exhaustive if every reachable shape of the input
type is covered by some arm. The model checker walks the type:

- **Sum types.** Every variant must appear in some arm, or `_` catches
  the rest. Pattern overlap is fine; pattern omission is a compile
  error.
- **Record / structural types.** Either all field-shape combinations
  are covered (rare), or `_` catches the rest. In practice, structural
  matches almost always have a `_ => ...` arm.
- **Refined predicates** (`[name^="x"]`, `:not(s)`, etc.). Coverage is
  decided over the *type space*, not the value space. A pattern like
  `[method^="notifications/"]` covers an open set; the complement is
  whatever isn't matched by any sibling arm. The checker tracks the
  uncovered residue and errors if it's non-empty at the closing brace.
- **Tuple patterns with same-name equality.** Two arms
  `(x, x) => ...` and `(_, _) => ...` together cover the full space
  (equal or not).

The `_` wildcard is explicit, never implicit. "I covered every case"
is an obligation the author takes on at the source level; the
compiler verifies it. There is no fallthrough silently producing
`dark(no_arm_matched)` — if a match isn't exhaustive, it doesn't
compile.

Why this is stricter than BEAM: BEAM checks well-formedness but not
coverage (it can't — it's Turing-complete and patterns can be
generated dynamically). Mirror is sub-Turing; patterns are static;
coverage is decidable. We make it required.

---

## Composition with `|>` and `|\>`

A match expression is a value. It composes with the pipeline operators
like any other:

```mirror
# match as pipeline stage
request
  |> @data/json.parse
  |> @data/json.to(@mcp.request)
  |> match {
       request[method="tools/list"] => @mcp.tools,
       request[method="tools/call"] > params[name=$n] => @mcp.tool_call($n),
       request[method=$m] => @mcp.error(method_not_found, $m),
     }
  |> @data/json.emit
  |> @io.write(stdout)
```

When `match` (or `select`) is the right-hand side of `|>`, the input
value is the matched expression — you don't repeat it. This reads
naturally:

```mirror
request |> match { ... }   # equivalent to: match request { ... }
```

With `|\>`, Fate resolves any `\` body inside the matched arm. This is
useful when an arm's right-hand side is itself a hole:

```mirror
thing |\> match {
  light(v)  => v |> store,
  dimmed(_) => \,   # Fate proposes the resolution
  dark(e)   => @fate.recover(e),
}
```

---

## `#(nl)` annotations on arms

Comments aren't trivia. A `#` line attached to a match arm becomes a
`#(nl)` AST node that's the arm's documentation:

```mirror
match request {
  # request the live list of tools the server offers.
  # called by Claude Code on session start and on tools/list_changed.
  request[method="tools/list"] => @mcp.tools,

  # invoke a named tool with arguments.
  # see @mcp/tool for the registration shape.
  request[method="tools/call"] > params[name=$n][arguments=$a]
    => @mcp.tool_call($n, $a),

  # any unknown method returns method_not_found.
  request[method=$m] => @mcp.error(method_not_found, $m),
}
```

These annotations:

- Contribute to the match expression's content OID. Editing the
  doc-line changes the crystal.
- Are queryable from grammar tooling: `match > arm > #(nl)` selects
  every arm's documentation.
- Surface in `@mirror/lsp.hover` automatically — the hover for a
  pattern's arm shows its `#(nl)` children.
- Carry through to `@mirror/lsp.completion` when proposing arms.

The LSP's diagnostics never report on doc lines (they're not code).
The property layer can verify their presence (e.g. every arm in a
public-facing match must have at least one `#(nl)` child).

---

## Implementation: the staircase

This spec rides the kintsugi ladder from Spec A. Three steps:

### Step 1 — tokenizer recognises the syntax

The bootstrap tokenizer (`bootstrap/src/tokenize.rs`, currently behind
the `io tokenize` binding in `@mirror/compile/bootstrap`) needs to
learn:

- `match`, `select` as keywords
- `=>` as a punctuator
- `>` as a child-relationship punctuator inside io bindings AND patterns
- `[`, `]` as attribute brackets
- `^=`, `*=`, `$=` as attribute predicates
- `:has`, `:not`, `:is`, `:first`, `:last`, `:nth` as pseudo-classes
- `_`, `*` as wildcards in pattern position
- `$` as a binding sigil

This is a one-shot change to `bootstrap/src/tokenize.rs` (Reading A of
Spec A — the Rust file lives behind the io binding, the change is the
last Rust-side edit before kintsugi closes the tokenizer's totality
obligations).

### Step 2 — grammar declares the forms

A new grammar, `boot/std/mirror/match.mirror`, declares the AST shape
for `match` / `select` expressions and their pattern children:

```mirror
in @prism
in @code/mq

grammar @mirror/match {
  type arm {
    pattern: mq_pattern,    # from @code/mq
    body: ast,
    annotations: [#(nl)],
  }

  type match_expr {
    subject: ast,
    arms: [arm],
    exhaustive: bool,       # proved by the model checker
  }

  type select_expr {
    binder: ref,            # the |v| name
    arms: [arm],            # patterns are variant-restricted
    exhaustive: bool,
  }

  # the parser produces these; the typechecker verifies exhaustiveness.
  parse_match(tokens) -> match_expr { \ }
  parse_select(tokens) -> select_expr { \ }

  # the executor walks arms in order, binds, returns the first match.
  evaluate_match(expr, env) -> imperfect { \ }
  evaluate_select(expr, env) -> imperfect { \ }

  # exhaustiveness: the obligation the model checker discharges.
  requires exhaustive(match_expr)
  requires exhaustive(select_expr)
}
```

All bodies are `\` initially. The parser's body is io-eligible (it
rides the kintsugi ladder); the evaluator's body is sub-Turing by
construction (no loops, no recursion past the AST depth).

### Step 3 — extend `@code/mq` with CSS selectors

`@code/mq` already declares queries (`out collapse`, `find`, etc.).
This spec extends the surface:

```mirror
grammar @code/mq {
  ...existing...

  type mq_pattern = head | sequence | union | predicate

  type head = type_ref | wildcard | binding
  type sequence = (mq_pattern, child_relation, mq_pattern)
  type child_relation = direct | descendant | next_sibling | any_sibling
  type union = [mq_pattern]
  type predicate = attribute | pseudo_class
  type attribute = field_eq | field_starts | field_contains | field_ends | field_present
  type pseudo_class = has(mq_pattern) | not(mq_pattern) | is([mq_pattern])
                    | first | last | nth(u64)

  # parse a pattern from tokens (post-tokenizer).
  parse_pattern(tokens) -> mq_pattern { \ }

  # check whether a value matches a pattern, with bindings.
  apply_pattern(value, pattern) -> imperfect(bindings, no_match, loss) { \ }
}
```

These declarations are the contract. The bodies close when the
tokenizer recognises the new tokens (step 1) and when kintsugi closes
the parser's totality obligations.

### Step 4 — close the @mirror/reload hole

With `match` declared and `@code/mq` extended, the `\` body in
`@mirror/reload.tick` can be written as the tuple-pattern match shown
in the Tuple section above. Same crystal, but the lambda is concrete.

Similar closure for: `@mirror/serve.dispatch`,
`@mirror/runtime/gen_prism.tick`, every other body waiting on
conditionals.

---

## Output direction — AstNode now, beam(ast) future

Match expressions today produce ordinary AST values (or whatever
type the arm bodies return). The future state, per the tokenizer
question in this brainstorm, is for tokenizer rules to emit `beam(ast)`
directly — every token carries luminosity, topology, loss, timing.

This spec's match doesn't preemptively wrap in beams. The migration
is mechanical:

```
today:   match e { p => body }            → returns body's value
future:  match e { p => body }            → returns beam(body's value)
```

The pattern grammar doesn't change. The arm body's type-checker
gains a layer (the beam wrapper). Existing match expressions read the
same; their consumers gain optional access to the beam's topology when
useful.

Design mark: ensure the match AST keeps the arms' beam-readiness slot
open, so the wrapper step is additive rather than rewriting.

---

## Implications — concrete next ticks

1. **Land tokenizer recognition.** Extend `bootstrap/src/tokenize.rs`
   for the keywords + operators above. This is one of the last Rust
   edits before `@mirror/compile/tokenize` becomes kintsugi-eligible.
   Single targeted commit.

2. **Write `boot/std/mirror/match.mirror`.** Declares the AST shape
   and the contracts. All bodies `\` initially; the exhaustiveness
   `requires` are the proof obligations.

3. **Extend `boot/std/code/mq.mirror`** with the CSS-selector pattern
   grammar declared in Step 3 above. Reuse mq's existing query
   infrastructure where possible.

4. **Declare `@epistemologic/property/exhaustive`.** The property the
   match/select expressions require. Body `\` until the model checker
   walks the type space; this is one of the totality obligations
   the kintsugi formatter (Spec A) discharges.

5. **Close the `@mirror/reload.tick` hole.** With match available,
   the body becomes the tuple-pattern shown in this spec. The
   conditional logic that's been a comment block becomes the code.

6. **Close the `@mirror/runtime/gen_prism.tick` shape.** Each concrete
   gen_prism (reload, lsp buffer, mcp session) has a `tick` body that
   matches on message kind. They were `\` for the same reason; now
   they can be written.

7. **Close `@mirror/serve.dispatch`.** The routing table that turns
   JSON-RPC method strings into operation calls. Likely the largest
   single match in the boot tree; its arms describe the entire
   MCP/LSP surface.

---

## Out of scope

- The bootstrap implementation of `mirror serve`. The dispatch grammar
  closes once match is available; the bootstrap wiring that reads
  stdin and routes through `@mirror/serve.dispatch` is its own work.
- Guards (`when` clauses) on arms. The Erlang/Rust-style guard adds
  expressive power but at the cost of an arbitrary expression in
  pattern position. Strict pattern matching covers the cases we have;
  guards can land later if a real grammar demands them.
- Pattern types as values. "Can you bind a pattern to a name and reuse
  it?" — yes in spirit (it's a grammar declaration), but the surface
  for that isn't designed here.
- Numeric range patterns (`1..10`). Mirror doesn't have numeric
  literals in many places today; this spec doesn't introduce them in
  patterns.
- Async / streaming matchers. Match is synchronous against a fully
  realised value. Stream-matching (BEAM-style with `continue`) is a
  separate primitive.
- Cross-arm binding (`$x` available in arm 2 because arm 1 bound it).
  Bindings are scoped to a single arm. Sharing requires `let` outside
  the match.
- `let` as a pattern-binding form. Today's `let current = expr;`
  syntax in this spec presumes `let` exists or is the obvious sugar
  for `expr |> bind_to(current)`. Either way, its grammar is its own
  small spec.

---

*The match is the path the beam takes.*
*The selector is what the beam refracts through.*
*The pattern is the glass.*
*The arm is the angle.*
*The body is what comes through.*

Apache-2.0.
