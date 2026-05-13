# CLI Arguments as Typed Lambdas

**Author:** Reed
**Date:** 2026-05-14
**Status:** Research / Design

---

## Problem

Mirror's CLI is grammar-dispatched. Commands are grammar references. But flags
have no type surface. The existing `boot/std/cli.mirror` declares:

```mirror
flag strict = prism(imperfect => success | failure)
flag format(json | human) = lens(imperfect => text)
flag check = prism(imperfect => pass | fail)
flag verbose = lens(loss => text)
```

This is already optics-as-flags. But `flag` is a keyword the parser doesn't
know (it's in Tier 2 -- runtime-learned, not hardcoded). And the arity rules
are implicit. When someone writes:

```
mirror kintsugi --lift src/mcp.rs --target std --git/commit "message"
```

Three different things are happening:
- `--lift` is nullary (a toggle, a prism: focus or don't)
- `--target std` is unary (a lens: transform by this value)
- `--git/commit "message"` is unary with a grammar path

Each flag is a lambda. The arity is the type. The composition is application.
This document maps the design space and proposes the mirror type surface.

---

## Prior Art

### 1. Haskell: optparse-applicative

**Core insight: Parser is an Applicative Functor.**

The type of a parser is `Parser a` where `a` is the result type. Flags compose
via `<*>` (applicative apply) and `<|>` (alternative). The parser for a whole
CLI is built by combining individual flag parsers:

```haskell
data Options = Options
  { optTarget :: String
  , optStrict :: Bool
  , optFormat :: Format
  }

optionsParser :: Parser Options
optionsParser = Options
  <$> strOption (long "target" <> short 't' <> metavar "TARGET")
  <*> switch (long "strict" <> short 's')
  <*> option auto (long "format" <> value Human)
```

**What's the type of a flag?**
- `switch` : `Mod FlagFields Bool -> Parser Bool` (nullary -- presence/absence)
- `strOption` : `Mod OptionFields String -> Parser String` (unary -- takes value)
- `option auto` : `Mod OptionFields a -> Parser a` (unary -- parsed value)
- `argument` : `ReadM a -> Mod ArgumentFields a -> Parser a` (positional)

**How is arity represented?** By the parser combinator used. `switch` is nullary.
`option`/`strOption` is unary. `some`/`many` wraps any parser into variadic.

**How do flags compose?** Via applicative `<*>`. Each parser produces one field.
The product type collects them. Order-independent -- the applicative structure
means all flags parse independently, then combine.

**Type error for invalid composition:** Compile-time. If you try `Options <$> switch ...`
where Options expects a `String` first, GHC gives a type mismatch.

**Help generation:** Automatic from `Mod` metadata (long name, short name, help text,
metavar). The parser IS the help source.

**Mirror relevance:** The applicative functor pattern maps directly to optic composition.
A `Parser a` is structurally a `Lens' [String] a` -- it focuses on a specific part
of the argument list and extracts a typed value.

### 2. OCaml: Cmdliner

**Core insight: Terms are lifted values combined via application.**

```ocaml
let count =
  let doc = "Repeat the greeting $(docv) times." in
  Arg.(value & opt int 1 & info ["c"; "count"] ~doc)

let name =
  let doc = "Name of the person to greet." in
  Arg.(required & pos 0 (some string) None & info [] ~doc)

let greet_t = Term.(const greet $ count $ name)
```

`Term.const` lifts a function. `Term.($)` applies terms (like `<*>`).
`Arg.value` wraps an argument spec into a term. The term carries both the
parsing logic AND the man page metadata.

**What's the type of a flag?** `'a Term.t` -- a term that will produce an `'a`.
- `Arg.flag` : `'a -> 'a Arg.t` (nullary, returns default or the flag value)
- `Arg.opt` : `'a Arg.conv -> 'a -> 'a Arg.t` (unary with converter and default)
- `Arg.pos` : positional by index

**How is arity represented?** By the Arg combinator. `flag` = nullary. `opt` = unary.
`pos_all` = variadic.

**Help generation:** Man pages generated automatically from `Arg.info` and `Cmd.info`.
The term structure IS the documentation.

**Mirror relevance:** Cmdliner's "term = computation + documentation" duality is
exactly what we want. A mirror `flag` declaration should carry both the optic
(the computation) and the documentation (for `--help`).

### 3. Elm: elm-cli-options-parser

**Core insight: The compiler guarantees data integrity or gives a clear error.**

Elm's approach enforces explicit declarations -- no ambiguous mixes of optional
and required positional arguments. Standard features like `--help` are baked in.
The library ensures help messages remain in sync with code. Typo suggestions
are automatic.

**Mirror relevance:** Mirror already has this property. Grammars declare what exists.
If a flag isn't declared, it doesn't parse. The grammar IS the validation. Elm
confirms the pattern: declare the surface, derive everything else.

### 4. Nushell: Structured Data Pipeline

**Core insight: The pipeline carries typed values, not text.**

Nushell's `PipelineData` is an enum: `Empty | Value | ListStream | ByteStream`.
Commands are categorized as producers, filters, or formats. Each defines how it
handles typed input.

**How do flags compose?** They don't compose as a pipeline -- they configure a
single command. But the pipeline between commands IS typed composition. Each
stage declares its input/output types. Mismatches are caught at parse time.

**Mirror relevance:** Mirror flags compose as a pipeline within a command. The
command itself is one stage. The pipeline is `flag . flag . flag`. This is
different from Nushell's command-to-command pipeline but the type discipline
is the same: each stage declares its input and output types.

### 5. PowerShell: Parameter Binding

**Core insight: Two binding modes -- ByValue (type match) and ByPropertyName (structural match).**

PowerShell tries ByValue first: if the pipeline object's type matches a parameter
type, it binds. If that fails, it tries ByPropertyName: if the object has a
property whose name matches a parameter name, it binds that property.

This is optic dispatch. ByValue is a prism (type match). ByPropertyName is a
lens (structural focus by name).

**Mirror relevance:** Mirror flags are already optics. A `prism` flag matches by
type (does the result have this variant?). A `lens` flag focuses by structure
(transform this field). PowerShell discovered this pattern empirically. Mirror
can declare it in the type system.

### 6. jq: Concatenative Composition

**Core insight: Every filter is a function from JSON to JSON. Composition is `.` (pipe).**

jq's evaluation model is branching and nondeterministic -- a filter can produce
zero, one, or many outputs. The pipeline is not just linear; it's a DAG.
Composition is concatenative: `f | g` means "apply f, then apply g to each result."

**What's the type of a filter?** `JSON -> [JSON]` (nondeterministic, like a List monad).

**Mirror relevance:** jq's model maps to the optic hierarchy:
- A filter that always succeeds = lens
- A filter that might fail = prism
- A filter that produces multiple results = traversal
- A filter that is invertible = iso

The jq `.` composition operator is the same as mirror's `.` optic composition.
`focus . project . split` in mirror is structurally `f | g | h` in jq.

### 7. Lambda Calculus for Agent Composition (arXiv:2604.11767)

Recent work formalizes agent composition as typed lambda calculus. Each agent
is a typed function. Composition is application. The type system catches invalid
compositions (e.g., passing an image agent's output to a text-only agent).

**Mirror relevance:** Each mirror flag IS a typed lambda:
- Nullary: `() -> transform` (a thunk -- no argument, just apply the optic)
- Unary: `a -> transform` (a function -- takes one argument, applies the optic)
- Variadic: `[a] -> transform` (a fold -- takes many arguments)

The flag pipeline `--lift --target std --format json` is:
```
lift() . target("std") . format("json")
```
Which is lambda application: `(format "json") . (target "std") . (lift)`.

---

## The Mirror Approach

### Flags as Typed Lambdas

Each flag is a lambda whose type determines its arity. The optic type determines
what the flag does to the pipeline state.

```mirror
in @prism
in @meta

# A flag is a named lambda that transforms the pipeline state.
# The arity comes from the parameter list.
# The optic type comes from the body.

grammar @cli/args {

  # --- Flag arity types ---

  # nullary: no argument. Presence = apply the optic. Absence = identity.
  # --strict, --verbose, --lift
  # Type: () -> optic
  type nullary(lens) {
    parse: text -> bool,
    optic: lens
  }

  # unary: takes one argument. The argument type is declared.
  # --target std, --format json, --indent 2
  # Type: a -> optic
  type unary(lens, a) {
    parse: text -> a,
    optic: a -> lens
  }

  # optional: takes zero or one argument. Has a default.
  # --color [always|never|auto]
  # Type: option(a) -> optic
  type optional(lens, a) {
    parse: text -> a,
    default: a,
    optic: a -> lens
  }

  # variadic: takes one or more arguments.
  # --include *.mirror --include *.rs
  # Type: [a] -> optic
  type variadic(lens, a) {
    parse: text -> a,
    optic: [a] -> lens
  }

  # --- The flag declaration ---

  # A flag is a name + arity + optic + documentation.
  type flag(arity) {
    name: ref,            # the --name (maps to @grammar/name)
    grammar: ref,         # the grammar reference (@grammar/name)
    arity: arity,
    doc: text             # one-line description for --help
  }

  # --- Pipeline type ---

  # A pipeline is an ordered sequence of applied flags.
  # Each flag's output type must match the next flag's input type.
  # The pipeline's type is: input -> output
  #   where input = first flag's input
  #   and   output = last flag's output
  type pipeline {
    flags: [flag],
    input: type,
    output: type
  }

  # --- Composition ---

  # Flags compose left to right.
  # --strict --format json = strict() . format("json")
  #
  # Type checking: strict outputs (success | failure).
  # format accepts imperfect. success < imperfect. Valid.
  #
  # But: format outputs text. strict accepts imperfect. text != imperfect.
  # So --format json --strict is a type error:
  #   "flag 'strict' expects imperfect, but 'format' outputs text"

  compose(a: flag, b: flag) -> flag {
    requires a.optic.output <= b.optic.input
  }

  # --- Parsing ---

  # parse turns [text] into a pipeline.
  # 1. Tokenize: split on whitespace, group --flag [value]
  # 2. Resolve: --name -> @grammar/name lookup
  # 3. Type: check arity matches argument count
  # 4. Compose: check pipeline type compatibility

  action parse(args: [text]) -> pipeline

  # --- Help generation ---

  # --help is derived from the grammar's flag declarations.
  # Each flag's doc field becomes a line in --help.
  # The grammar hierarchy provides grouping.

  action help(grammar) -> text {
    flags = grammar.declarations |> project(flag)
    flags |> format_help
  }
}
```

### The Grammar Surface

Here is how flags are declared in a mirror grammar. This is what
`boot/std/cli.mirror` evolves into:

```mirror
in @prism
in @meta
in @cli/args

grammar @cli {

  # --- Flags as typed lambdas ---

  # nullary: --strict is a prism. It either passes or fails.
  # No argument. Presence applies the optic.
  flag strict = prism(imperfect => success | failure)
    # "only emit on full success"

  # unary: --format takes an argument from an enum.
  # The enum IS the argument type. Invalid values are type errors.
  flag format(json | human) = lens(imperfect => text)
    # "output format"

  # unary: --target takes a grammar reference.
  flag target(ref) = lens(grammar => grammar)
    # "target grammar for code emission"

  # unary: --indent takes a number.
  flag indent(int) = lens(text => text)
    # "indentation width"

  # nullary: --verbose is a lens that renders loss as text.
  flag verbose = lens(loss => text)
    # "show loss details"

  # nullary: --lift is a prism.
  flag lift = prism(imperfect => imperfect)
    # "hoist nested declarations to top level"

  # optional: --color defaults to auto.
  flag color(always | never | auto) = lens(text => text)
    # "color output"
    default auto

  # --- Commands as optic compositions ---

  command compile = parse . resolve . emit
  command kintsugi = parse . resolve . canonical_order
  command ci = parse . resolve . properties . emit

  # --- Actions as typed lambdas ---

  action focus(path: text) -> imperfect
  action project(path: text) -> imperfect
  action split(path: text) -> imperfect
  action zoom(path: text) -> imperfect
  action refract(path: text) -> imperfect
}
```

### The Key Insight: flag IS lambda

A `flag` declaration is syntactic sugar for a typed lambda in the grammar.
The desugaring is:

```
flag strict = prism(imperfect => success | failure)
```

desugars to:

```
strict() -> prism(imperfect => success | failure)
```

A nullary lambda. And:

```
flag format(json | human) = lens(imperfect => text)
```

desugars to:

```
format(value: json | human) -> lens(imperfect => text)
```

A unary lambda. The `flag` keyword is sugar that:
1. Registers the name as a CLI flag (`--name`)
2. Maps `--name` to `@grammar/name` (grammar reference)
3. Infers arity from the parameter list
4. Generates `--help` from the doc comment

### The -- to @ Mapping

Every `--flag` is a grammar reference:

```
--strict       ->  @cli/strict
--format json  ->  @cli/format(json)
--target std   ->  @cli/target(std)
--git/commit   ->  @git/commit
```

The `/` in `--git/commit` is the grammar path separator. `--git/commit "msg"`
is `@git/commit("msg")`. The flag IS the grammar action.

For flags within the current grammar, the prefix is implicit:
`--strict` = `@cli/strict`. For cross-grammar flags, the path is explicit:
`--git/commit` = `@git/commit`.

---

## Type Examples

### Valid composition

```
mirror kintsugi --lift --target std --format json src/mcp.rs

Pipeline:
  lift()           : prism(imperfect => imperfect)
  target("std")    : lens(grammar => grammar)
  format("json")   : lens(imperfect => text)

Composition: lift . target . format
  lift   output: imperfect
  target input:  grammar     -- grammar < imperfect? No.

Wait. This reveals a design constraint: for flags to compose as a
pipeline, they need compatible types. But most CLI flags don't
pipeline -- they configure independently.
```

This is the key distinction from prior art. CLI flags are NOT a pipeline.
They are an **applicative product** -- each flag configures one dimension
of the command's behavior, independently.

### The Corrected Model: Applicative, Not Pipeline

```mirror
# Flags compose as an applicative product, not a pipeline.
# Each flag independently transforms the command configuration.
# The command receives the product of all flag values.

type config(command) {
  flags: [applied_flag],
  positional: [text]
}

# Each flag application is independent:
#   --strict        -> config.strict = true
#   --format json   -> config.format = json
#   --target std    -> config.target = std

# The command receives the whole config and applies its optic composition:
#   kintsugi(config) = parse . resolve . canonical_order
#   where config.strict gates the output prism
#   and   config.format selects the output lens
```

This matches optparse-applicative exactly: flags are independent parsers
combined via `<*>` into a product type. The product type IS the command
configuration. The command's optic pipeline consumes the configuration.

### Type error: unknown flag

```
mirror kintsugi --bogus src/

Error: unknown flag '--bogus'
  grammar @cli has no declaration 'bogus'

  did you mean: --verbose (edit distance 2)

  available flags for 'kintsugi':
    --strict    only emit on full success
    --format    output format (json | human)
    --lift      hoist nested declarations
    --verbose   show loss details
```

The error is a grammar lookup failure. The suggestion is edit-distance.
The available flags come from the grammar's flag declarations.

### Type error: wrong arity

```
mirror kintsugi --strict json src/

Error: flag '--strict' takes no arguments, but got 'json'
  strict : () -> prism(imperfect => success | failure)

  did you mean: --format json?
```

The arity mismatch comes from the lambda type. `strict` has zero
parameters. Passing an argument is a type error.

### Type error: wrong argument type

```
mirror kintsugi --format xml src/

Error: invalid argument 'xml' for flag '--format'
  format : (json | human) -> lens(imperfect => text)

  expected one of: json, human
  got: xml
```

The enum type constrains the argument. `xml` is not in `json | human`.
This is a type error at parse time, not a runtime error.

### Type error: missing required positional

```
mirror kintsugi --strict

Error: missing required argument 'path'
  kintsugi(path: text) -> imperfect

  usage: mirror kintsugi [flags] <path>
```

---

## Help Generation

`--help` is derived from grammar declarations. No hardcoded strings.

```
$ mirror kintsugi --help

mirror kintsugi - the repair pass

Usage: mirror kintsugi [flags] <path>

Flags:
  --strict           only emit on full success
  --format <format>  output format (json | human) [default: human]
  --lift             hoist nested declarations to top level
  --indent <n>       indentation width [default: 2]
  --verbose          show loss details
  --color <mode>     color output (always | never | auto) [default: auto]

Optic: parse . resolve . canonical_order

Grammar: @cli/kintsugi
```

The structure:
1. **Name + doc:** from `command kintsugi` declaration and its doc comment
2. **Usage line:** generated from action signature + flag declarations
3. **Flags:** from `flag` declarations. Name from declaration. Metavar from
   parameter type. Default from `default` clause. Doc from comment.
4. **Optic:** from `command kintsugi = parse . resolve . canonical_order`
5. **Grammar:** the grammar path, always shown

### Cross-grammar help

```
$ mirror --help

mirror - the measurement substrate

Commands:
  focus     observe the spectral state
  project   filter by what matters
  split     explore what's connected
  zoom      transform at scale
  refract   settle. done. crystal.

  kintsugi  the repair pass
  compile   compile grammars
  ci        continuous integration pass

  lsp       language server
  repl      interactive shell

Flags (global):
  --strict           only emit on full success
  --format <format>  output format (json | human)
  --verbose          show loss details

Run 'mirror <command> --help' for command-specific flags.
```

Global flags come from the top-level grammar. Command-specific flags
come from the command's grammar or its imported grammars.

---

## Design Decisions

### 1. Applicative product, not pipeline

Flags configure independently. They are combined as a product (record/struct),
not composed as a pipeline. This matches how every real CLI works and aligns
with optparse-applicative and cmdliner.

The optic pipeline is the command's implementation (e.g., `parse . resolve . emit`).
Flags parameterize that pipeline, they don't extend it.

### 2. flag IS lambda sugar

No new concept needed. A flag is a lambda with extra metadata (doc, default).
The parser already knows lambdas. `flag` is a declaration kind that adds:
- CLI registration (makes the name available as `--name`)
- Grammar path mapping (`--name` -> `@grammar/name`)
- Help metadata

### 3. Arity from parameter list

- `flag strict = ...` -- zero params -- nullary (boolean toggle)
- `flag format(json | human) = ...` -- one param, enum type -- unary
- `flag indent(int) = ...` -- one param, primitive type -- unary
- `flag include([text]) = ...` -- one param, list type -- variadic

No separate arity annotation. The parameter list IS the arity.
This is how lambda calculus works. The number of bound variables
is the function's arity.

### 4. -- to @ is mechanical

The mapping `--name` to `@grammar/name` is purely mechanical:
- Strip `--`
- Replace `/` with grammar path separator
- Look up in the current grammar scope
- Fall back to imported grammars

No special syntax. No external library. The grammar IS the parser.

### 5. Doc comments as help text

The comment immediately following a flag declaration is its `--help` text.
Same pattern as Rust's `///` doc comments, but using mirror's `#` syntax.

```mirror
flag strict = prism(imperfect => success | failure)
  # only emit on full success
```

The `# only emit on full success` becomes the flag's help text.

### 6. No external parsing library

The grammar IS the parser. The type system IS the validator. The
doc comments ARE the help text. No clap, no structopt, no argparse.
The self-hosting compiler compiles its own CLI from its own grammar.

---

## Comparison Table

| Aspect | optparse-applicative | cmdliner | Elm CLI | mirror |
|--------|---------------------|----------|---------|--------|
| **Flag type** | `Parser a` | `'a Term.t` | `CliOption a` | `flag = lambda` |
| **Arity** | combinator choice | Arg combinator | explicit | parameter list |
| **Composition** | applicative `<*>` | `Term.($)` | `|>` | applicative product |
| **Help source** | `Mod` metadata | `Arg.info` | declaration | doc comment |
| **Type errors** | compile-time | compile-time | compile-time | parse-time |
| **Pipeline** | N/A | N/A | N/A | command = optic chain |
| **Grammar ref** | N/A | N/A | N/A | `--name` = `@grammar/name` |

Mirror's unique contribution: flags are grammar references. The mapping
between CLI surface and grammar namespace is structural, not conventional.
Every flag that exists in `--help` exists as a declaration in a `.mirror` file.
Adding a flag means adding a grammar declaration. Removing a flag means
removing a declaration. The grammar and the CLI are the same thing.

---

## What Changes in boot/std/cli.mirror

The existing declarations are already almost right. What changes:

1. `flag` stays as a declaration keyword (Tier 2, runtime-learned)
2. `command` stays as a declaration keyword (Tier 2, runtime-learned)
3. Doc comments after flags become `--help` text
4. The `=` in `flag name = optic(...)` means "this flag's optic is..."
5. Parameters in `flag name(type)` determine arity
6. `default` clause in flag body sets default value for optional flags

No new keywords. No new syntax. The existing declarations just need
the type system to know what `flag` and `command` mean. That's the
self-teaching parser (Tier 2 from minimum-viable-keywords.md).

---

## Bootstrap Path

1. **Now:** `flag` and `command` are silently skipped (Tier 2 keywords)
2. **Tier 2 parser:** learn `flag` and `command` from grammar declarations
3. **Type checking:** validate flag arity against parameter list
4. **CLI generation:** generate `--help` from flag declarations
5. **Dispatch:** generate command dispatch from command declarations
6. **Self-hosting:** `mirror craft cli` produces the CLI from `@cli` grammar

Steps 2-4 can happen without any Rust changes to the parser. The grammar
declares what `flag` means. The compiler learns it. The CLI generates from it.

This is the compiled-cli-spec taken to its conclusion: the CLI is not
generated from `mirror.spec` (a separate config format). The CLI is
generated from `@cli` (a standard grammar). Same types. Same compiler.
Same optics. One surface.
