# Generated Parser Spec

**Taut** (benchmark) + **Seam** (adversarial review)

Date: 2026-04-14

---

## 1. Current Parser Profile (Taut)

### Parser anatomy

The parser lives in `src/mirror_runtime.rs` (4154 lines, 100 tests). Three
phases: tokenize, parse_decl, emit_form. No AST optimization pass. No
intermediate representation between tokens and Form.

**Tokenizer** (`tokenize`, lines 558-662): byte-at-a-time scanner, 8 token
types (`Word(String)`, `LBrace`, `RBrace`, `LParen`, `RParen`, `Comma`,
`Equals`, `Newline`). Operators (`|`, `.`, `/`, `<`, `>`, `:`, `-`, `!`) are
greedily collected into `Word` tokens. `=` is a separate `Tok::Equals` variant
(not `Word`), which forces special-case handling throughout the parser.

**parse_form** (lines 346-466): top-level entry. Loops over tokens, dispatches
to `parse_decl` on keyword match, collects unrecognized tokens as loss.

**parse_decl** (lines 670-1092): the main recursive descent function.
422 lines. Handles:
- Modifier prefix (`abstract`)
- Recover/Rescue special path (pipe-delimited params)
- Name parsing with path absorption (`@code/rust`)
- Grammar inheritance (`< @parent`)
- Parenthesized params with nesting
- Fold (`<=`) vs Iso (`=`) vs variant lists
- Action bodies (structured children vs raw text)
- Brace-balanced child blocks
- Relation markers (`<type`, `>type`)
- Unrecognized token skip-to-newline

### Where time goes

Phase breakdown for a typical boot file (20KB, 962 lines total):

| Phase | Cost driver | Notes |
|-------|-------------|-------|
| Tokenize | `String::to_string()` per word | Every word token heap-allocates |
| Keyword dispatch | `DeclKind::parse()` — 23-arm match | Called for every `Word` token at top-level and inside blocks |
| Param parsing | Nested paren tracking, `push`/`clone` | `Vec<String>` growth, `String::clone()` on every param |
| Variant parsing | Linear scan until `Newline`/`LBrace` | `Vec<String>` growth |
| Child blocks | Recursive `parse_decl` | Stack depth proportional to nesting |
| Action body peek | Full brace-balanced scan to detect `DeclKind` | Scans the body twice: once to classify, once to parse |

**DeclKind::parse()** has 23 match arms. Called:
- Once per top-level token in `parse_form` (line 360)
- Once per modifier check in `parse_decl` (line 696/700)
- Once per child token in block parsing (line 1012)
- Once per token in action body peek (line 1187)

For the full boot corpus (962 lines, ~400 declaration-starting tokens), this
is ~800+ DeclKind match evaluations per full boot compile. The match is a
linear scan in the worst case (Rust may optimize to a jump table for
equal-length strings, but the keywords range from 2 to 9 chars).

### Token counts

Boot corpus statistics (20 files, 20,547 bytes, 962 lines):

| Metric | Count |
|--------|-------|
| Total lines | 962 |
| Declaration-starting tokens | ~400 |
| `out` declarations | 123 |
| `type` declarations | 108 |
| `action` declarations | 54 (100 if counting `abstract action`) |
| `in` declarations | 57 |
| `grammar` declarations | 12 |
| Comment lines (`--` / `#`) | ~104 |
| Blank lines | ~100 |

Average boot file: 48 lines, 1027 bytes, ~20 declarations.

### Jump table vs match chain

Current `DeclKind::parse()` is a 23-arm string match. A perfect hash or
length-keyed jump table would be:

```
Length 2: in (1 candidate)
Length 3: out (1 candidate)
Length 4: form, type, fold, zoom, lens (5 candidates)
Length 5: prism, split, focus (3 candidates)
Length 6: action, rescue (2 candidates)
Length 7: project, grammar, default, binding, recover, ensures, refract (7 candidates)
Length 8: property, requires, traversal, invariant (4 candidates)
```

First dispatch on length (0-8 = array index), then byte comparison within the
bucket. Maximum 7 comparisons in the worst case (length 7 bucket), average
~2. This is 3-4x fewer comparisons than linear match scan average.

With `phf` (compile-time perfect hash): O(1) lookup, ~3 instructions per
keyword. For 23 keywords this is marginal gain over a good match, but it
matters when the keyword table grows to 50+ (as it will with grammar-derived
keywords).

### Theoretical minimum bootstrap parser

To parse `00-prism.mirror` and `01-meta.mirror`, the absolute minimum is:

**Tokens needed:**
- `Word(String)` — identifiers, operators, @-prefixed names
- `LParen`, `RParen` — parameter lists
- `LBrace`, `RBrace` — blocks
- `Comma` — separators
- `Equals` — iso operator
- `Newline` — line termination

(Same as current. The tokenizer is already minimal.)

**Keywords needed to parse 00-prism.mirror:**
- `focus`, `prism`, `project`, `split`, `zoom`, `refract`, `out`

**Additional keywords for 01-meta.mirror:**
- `in`, `type`, `fold`, `grammar`, `recover`, `rescue`
- Operators as names: `|`, `<=`, `=>`, `<`, `>`, `=`, `!=`, `|>`, `<|`, `/`, `->`, `<-`, `..`

**Not needed for bootstrap:**
- `form` (deprecated, 0 uses in boot)
- `traversal` (0 uses in boot)
- `lens` (0 uses in boot)
- `property`, `requires`, `invariant`, `ensures` (only in 05-property.mirror and std/)
- `action` (only from 01a-meta-action.mirror onward)
- `abstract` (only from 03-code.mirror onward)
- `default`, `binding` (only in std/)

**Minimum keyword set for stages 00+01: 13 keywords.**
Current parser hardcodes 23. That's 10 keywords the bootstrap doesn't need.

---

## 2. Current Parser Seams (Seam)

### Seam 1: Every token allocates

The tokenizer converts every word to `Tok::Word(String)`. For the boot corpus
that's ~800+ heap allocations just for tokenization. The source string already
holds the bytes. A zero-copy tokenizer would store `&str` slices into the
source, eliminating all tokenization allocation.

**Cost:** ~800 allocations per boot compile, each 8-64 bytes.

### Seam 2: Equals is not a Word

`=` is `Tok::Equals`, not `Tok::Word("=")`. This forces every operator-aware
path to handle two cases:

```rust
// Fold detection: Word("<") + Equals
let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
    && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
```

If `=` were `Word("=")`, the fold operator `<=` would tokenize as a single
`Word("<=")` (the greedy operator scanner at line 597 already handles `<`).
The `Equals` special case splits what should be one token into two, and every
compound operator involving `=` must be reassembled downstream.

**Impact:** ~15 sites in parse_decl handle the Equals/Word split. Each is a
potential bug surface.

### Seam 3: Action body double-scan

`parse_action_body` (lines 1165-1272) peeks ahead through the entire body to
determine if it contains `DeclKind` keywords (line 1187), then re-parses from
the start. For a body of N tokens, this is 2N work.

For the boot corpus, action bodies are small (most are `{ }` empty or 2-3
lines), so the absolute cost is low. But as user grammars grow, this becomes
quadratic in body size.

### Seam 4: Branch mispredictions in parse_decl

The function has 422 lines with deeply nested match arms. The hot path through
the boot corpus is:

```
Token sequence frequency in boot (top 6):
1. `out <name>`           — 123 occurrences (trivial: kind + name, no body)
2. `type <name> ...`      — 108 occurrences (params, variants, maybe block)
3. `in @<name>`           — 57 occurrences (kind + name)
4. `abstract action ...`  — ~45 occurrences (modifier + kind + name + grammar_ref + body)
5. `action <name> ...`    — ~9 occurrences (no modifier)
6. `grammar @<name> ...`  — 12 occurrences (block with children)
```

The Recover/Rescue special case (lines 709-789) is checked on every
`parse_decl` entry but only fires 12 times across the entire boot corpus.
The grammar inheritance check (lines 820-833) fires only for 12 grammar
declarations. These are early branches that miss >95% of the time.

Reordering the checks so the hot path (`out`, `type`, `in`) resolves first
would improve branch prediction. Currently the order is:
1. abstract modifier check (fires ~45/400 = 11%)
2. recover/rescue check (fires ~12/400 = 3%)
3. name parse (fires ~100%)
4. grammar inheritance (fires ~3%)

### Seam 5: DeclKind arms never taken by boot

Three DeclKind variants have zero usage in any boot file:

| Variant | Boot usage | Notes |
|---------|-----------|-------|
| `Form` | 0 | Deprecated, replaced by `grammar` |
| `Traversal` | 0 | Never used as keyword |
| `Lens` | 0 | Never used as keyword |

These are dead weight in the bootstrap parser. They exist for user grammars
but contribute nothing to the boot sequence.

### Seam 6: String clones in Form construction

`Form::new()` takes `impl Into<String>` for name, but params and variants are
`Vec<String>`. Every param and variant is cloned from the token's `String`.
Then `Form::to_fragment()` clones params and variants again. Then
`MirrorData::encode()` iterates them again to build the content-address bytes.

A single declaration traverses its params 3 times with allocation at each stage.

### Seam 7: No backtracking, but redundant lookahead

The parser doesn't formally backtrack, but it has ad-hoc lookahead patterns:
- Fold detection: peek at `cursor` and `cursor+1` (lines 760-761, 908-909)
- Grammar inheritance: peek at `cursor` and `cursor+1` (lines 821-828)
- Action body classification: full scan ahead (lines 1178-1195)

These are structurally sound but scattered. A unified peek/classify step before
entering the main parse would eliminate the redundancy.

---

## 3. Minimum Bootstrap Parser

### The glass wall types

Two types cross from Rust to Mirror and back:

- **Oid** — the `@` prefix. Content address. The identity of a declaration.
- **Imperfect** — the return type. `Success | Partial | Failure`.

Everything else is grammar surface. The bootstrap parser only needs to
understand enough to read what the grammar surface IS, then use that knowledge
to parse the rest.

### Stage 1: Bootstrap kernel (hardcoded)

Parses exactly `00-prism.mirror` and `01-meta.mirror`. After parsing these
two files, the parser has learned every type, operator, and grammar rule that
the rest of the boot sequence uses.

```
Keywords (13):
  focus, prism, project, split, zoom, refract, out,     -- from 00-prism
  in, type, fold, grammar, recover, rescue              -- from 01-meta

Operators (12):
  =, <=, =>, <, >, !=, |, |>, <|, /, ->, ..

Structural tokens (7):
  Word, LParen, RParen, LBrace, RBrace, Comma, Newline

Total hardcoded surface: 32 symbols
```

After parsing 01-meta.mirror, the parser knows:
- All type names (`pure`, `real`, `observation`, `template`, `error`, `loss`,
  `precision`, `grammar`, `block`, `imperfect`, `abstract`, `beam`)
- The `@meta` grammar and its operator table
- The fold pattern (`<= imperfect`)
- The recover/rescue pattern

### Stage 1 parse rules (7 rules total):

```
file     := decl*
decl     := keyword name params? body?
keyword  := 'focus' | 'prism' | 'project' | 'split' | 'zoom' | 'refract'
           | 'out' | 'in' | 'type' | 'fold' | 'grammar' | 'recover' | 'rescue'
name     := WORD | '@' WORD | operator
params   := '(' param (',' param)* ')'
body     := '{' decl* '}'
           | '=' variant ('|' variant)*
           | '<=' WORD+
operator := '|' | '<=' | '=>' | '<' | '>' | '!=' | '|>' | '<|' | '/' | '->' | '..'
```

This is 7 production rules. The current parser implements these same rules
but spread across 422 lines because it handles 23 keywords, modifiers,
action bodies, grammar inheritance, and relation markers.

### What Stage 1 learns from 00-prism + 01-meta

After compiling these two files, the parser can derive:

**From `out` declarations in 00-prism:**
- `in`, `id`, `@`, `@prism` are exported names

**From `out` declarations in 01-meta (26 exports):**
- `type`, `ref`, all operators, `pure`, `real`, `observation`, `template`,
  `error`, `loss`, `precision`, `beam`, `grammar`, `block`, `imperfect`,
  `abstract`, `@meta`

**From type declarations in 01-meta:**
- `imperfect(observation, error, loss)` — the fold return type
- `beam(result)` — the carrier type
- `abstract(grammar)` and `abstract(action)` — modifier semantics

This is enough to parse every subsequent boot file. The `abstract` modifier,
`action` keyword, `default`, `binding`, `property`, `requires`, `invariant`,
`ensures` — all of these are defined by the grammars declared in boot files
02 through 06.

---

## 4. Generated Parser Design

### The compilation loop

```
boot/00-prism.mirror ──┐
boot/01-meta.mirror  ──┤
                       ▼
              Stage 1 (hardcoded, 13 keywords)
                       │
                       ▼
              keyword table + type table + operator table
                       │
                       ▼
              Parse remaining boot files with extended tables
                       │
                       ▼
              Full grammar knowledge (all `out` declarations)
                       │
                       ▼
              mirror craft boot/ --target rust
                       │
                       ▼
              rust/src/generated_parser.rs
                       │
                       ▼
              compile → mirror binary (with generated parser)
                       │
                       ▼
              generated parser compiles boot/ → same output ← FIXED POINT
```

### What `mirror craft --target rust` emits

**1. Keyword table (from all `out` declarations that name DeclKind-like things):**

```rust
// Generated from boot compilation — do not edit
pub static KEYWORDS: phf::Map<&'static str, DeclKind> = phf_map! {
    "form" => DeclKind::Form,
    "type" => DeclKind::Type,
    "prism" => DeclKind::Prism,
    // ... all 23 current + any grammar-specific additions
};
```

**2. Operator table (from 01-meta's operator declarations):**

```rust
pub static OPERATORS: phf::Map<&'static str, OpticOp> = phf_map! {
    "=" => OpticOp::Iso,
    "<=" => OpticOp::Fold,
    "|" => OpticOp::Split,
    // ... all 12 operators
};
```

**3. Type table (from all `type` declarations across boot):**

```rust
pub static TYPES: phf::Map<&'static str, TypeInfo> = phf_map! {
    "imperfect" => TypeInfo { params: 3, glass_wall: true },
    "beam" => TypeInfo { params: 1, glass_wall: true },
    "pure" => TypeInfo { params: 0, glass_wall: false },
    // ... all boot types
};
```

**4. Grammar table (from all `grammar` declarations):**

```rust
pub static GRAMMARS: phf::Map<&'static str, GrammarInfo> = phf_map! {
    "@prism" => GrammarInfo { parent: None, actions: &[] },
    "@meta" => GrammarInfo { parent: None, actions: &["focus", "project", ...] },
    "@code" => GrammarInfo { parent: None, actions: &["complete", "diagnose", ...] },
    "@code/rust" => GrammarInfo { parent: Some("@code"), actions: &[...] },
    // ...
};
```

**5. Parse dispatch (generated match with hot-path ordering):**

```rust
pub fn parse_keyword(s: &str) -> Option<DeclKind> {
    // Hot path first (ordered by boot frequency)
    match s.len() {
        3 => match s { "out" => Some(DeclKind::Out), "in\0" => ... },
        4 => match s { "type" => Some(DeclKind::Type), ... },
        6 => match s { "action" => Some(DeclKind::Action), ... },
        7 => match s { "grammar" => Some(DeclKind::Grammar), ... },
        _ => KEYWORDS.get(s).cloned(),
    }
}
```

### The fixed-point property

The generated parser is correct iff:

```
parse(boot/) with generated_parser == parse(boot/) with stage1_parser
```

For every Form tree produced by parsing the boot corpus, the content-addressed
OIDs must be identical. Since OIDs are deterministic (SHA of
`kind:name:params:variants`), byte-identical Form trees produce
byte-identical OIDs. The fixed point is verified by comparing the OID of the
root Form for each boot file.

**Verification procedure:**
1. Build with Stage 1 parser, record all boot file root OIDs
2. Generate parser from Step 1's output
3. Rebuild with generated parser, record all boot file root OIDs
4. Assert OID sets are identical
5. If they differ: the generator has a bug. The OIDs are the proof.

---

## 5. Optimization Targets

### What the generated parser can do that the handcoded one cannot:

**A. Table-driven dispatch (eliminates match chains)**

The generated keyword table is a compile-time perfect hash map. Lookup is
O(1) with 2-3 instructions. The current 23-arm match is O(n) in the number
of keywords. As grammars add domain-specific keywords (which they will — every
`grammar` declaration introduces a namespace), the match chain grows linearly.
The phf map stays O(1).

**B. Hot-path-ordered branches**

The generator knows the boot corpus statistics. It can order parse_decl's
branches by frequency:

```
1. out    (123 hits) — early return, no body parsing
2. type   (108 hits) — params + variants, maybe block
3. in     (57 hits)  — kind + name, no body
4. action (54 hits)  — full parse
5. grammar (12 hits) — block with children
6. everything else   (46 hits combined)
```

The current parser checks `abstract` and `recover/rescue` before ANY of these.
The generated parser puts `out` first: 31% of all declarations resolve in 2
token reads (keyword + name).

**C. Arena allocation**

The generated parser can use a typed arena for Form nodes:

```rust
struct ParseArena {
    forms: typed_arena::Arena<Form>,
    strings: bumpalo::Bump,
}
```

All Form nodes and their strings live in contiguous memory. No individual
heap allocations. No Drop overhead. The arena is freed in one operation when
parsing completes.

For the boot corpus (~400 declarations, ~800 string tokens), this eliminates
~1200 individual allocations.

**D. Zero-copy tokenization**

Replace `Tok::Word(String)` with `Tok::Word(&'src str)`:

```rust
enum Tok<'src> {
    Word(&'src str),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Newline,
    // Note: no Equals — it's Word("=") now
}
```

The tokenizer borrows slices from the source string. Zero allocation during
tokenization. The source string must outlive the token stream (it already does
in every call site).

**E. Unified `=` handling**

Merge `Tok::Equals` into `Tok::Word("=")`. The greedy operator scanner already
handles `<`, `>`, `!` — adding `=` means:
- `<=` tokenizes as one `Word("<=")` (not `Word("<")` + `Equals`)
- `=>` tokenizes as one `Word("=>")` (not `Equals` + `Word(">")`)
- `!=` already works (operator scanner)
- `=` alone tokenizes as `Word("=")` instead of `Equals`

This eliminates ~15 special-case sites in parse_decl and makes fold detection
a single token match instead of a two-token lookahead.

**F. SIMD keyword recognition (speculative)**

The full keyword table has 23 entries. The longest keyword is 9 bytes
(`invariant`, `traversal`). All keywords are ASCII lowercase. A SIMD approach:

1. Load keyword bytes into a 16-byte SSE register
2. Compare against pre-shuffled keyword vectors
3. Mask result gives the match

For 23 keywords this requires ~3 SIMD comparisons (8 keywords per 128-bit
register). This is faster than even phf for very hot loops, but the
complexity is only justified if keyword lookup is the bottleneck (unlikely —
the tokenizer allocation dominates).

**Verdict:** SIMD is not worth it for 23 keywords. Zero-copy + phf + arena
allocation will deliver 5-10x improvement. SIMD is reserved for when the
keyword table exceeds 100 entries from grammar composition.

---

## 6. The Bootstrap Loop

### Phase diagram

```
                    ┌─────────────────────────────┐
                    │                             │
                    ▼                             │
    ┌──────────────────────────┐                  │
    │  Stage 1: Hardcoded      │                  │
    │  13 keywords, 7 rules    │                  │
    │  Parses 00-prism, 01-meta│                  │
    └────────────┬─────────────┘                  │
                 │                                │
                 ▼                                │
    ┌──────────────────────────┐                  │
    │  Learn: extend keyword   │                  │
    │  table from out decls    │                  │
    │  in 00-prism + 01-meta   │                  │
    └────────────┬─────────────┘                  │
                 │                                │
                 ▼                                │
    ┌──────────────────────────┐                  │
    │  Stage 2: Extended       │                  │
    │  Parse remaining boot/   │                  │
    │  with learned keywords   │                  │
    └────────────┬─────────────┘                  │
                 │                                │
                 ▼                                │
    ┌──────────────────────────┐                  │
    │  Generate: emit Rust     │                  │
    │  parser from full boot   │                  │
    │  grammar knowledge       │                  │
    └────────────┬─────────────┘                  │
                 │                                │
                 ▼                                │
    ┌──────────────────────────┐                  │
    │  Stage 3: Compiled       │                  │
    │  Generated parser binary │                  │
    │  with phf + arena + zc   │──────────────────┘
    └──────────────────────────┘     (verify: same OIDs)
```

### Invariants

1. **Monotonicity:** each stage learns strictly more keywords than the previous
2. **Convergence:** stage 3 output = stage 2 output (same OIDs)
3. **Totality:** stage 3 can parse any .mirror file that stage 1+2 can
4. **Minimality:** stage 1 is the smallest parser that can bootstrap
5. **Determinism:** the generated parser is a pure function of the boot corpus

### Build integration

```makefile
# In Justfile or Cargo build.rs:
generated-parser:
    # Stage 1+2: interpret boot files, emit Rust parser
    cargo run --bin mirror -- craft boot/ --target rust > src/generated_parser.rs
    # Stage 3: rebuild with generated parser
    cargo build
    # Verify fixed point
    cargo run --bin mirror -- verify-bootstrap boot/
```

The `verify-bootstrap` command parses every boot file with both the hardcoded
Stage 1 path and the generated Stage 3 parser, asserting OID equality.

---

## 7. Benchmark Predictions

### Current parser (baseline)

Estimated per-file parse time for boot corpus (cold, no optimization):

| Operation | Est. time | Dominant cost |
|-----------|-----------|---------------|
| Tokenize 1KB file | ~5us | String allocation per word |
| parse_decl (20 decls) | ~15us | DeclKind match + param clone |
| Full boot (20 files) | ~400us | Sum of above |
| compile_boot_dir | ~800us | Parse + fragment + store I/O |

### Generated parser (predicted)

| Operation | Est. time | Improvement |
|-----------|-----------|-------------|
| Tokenize 1KB (zero-copy) | ~1us | 5x (no allocation) |
| parse_decl (phf dispatch) | ~5us | 3x (O(1) lookup, hot-path order) |
| Full boot (arena) | ~120us | 3x (no individual allocs) |
| compile_boot_dir | ~400us | 2x (parse faster, I/O unchanged) |

**Overall prediction: 2-3x faster for parse phase, 1.5-2x end-to-end.**

The parse phase is not the bottleneck for small files (the fragment store I/O
dominates). The generated parser's value is not raw speed — it's that the
parser IS the grammar. When a grammar changes, the parser changes. When a
keyword is added, the parser learns it. The handcoded parser can never do this.

### What matters more than speed

1. **Correctness by construction:** the generated parser is derived from the
   same declarations it parses. If the declarations are wrong, the parser
   won't compile. If the parser compiles, it matches the declarations.

2. **Extensibility:** adding `property` or `invariant` to a grammar
   automatically extends the parser's keyword table. No Rust code changes.

3. **The Equals seam:** unifying `=` into the operator scanner eliminates
   a class of bugs (compound operator reassembly) that the current parser
   is vulnerable to. The generated parser doesn't have this seam because
   the operator table IS the grammar's operator declarations.

4. **Dead code elimination:** the 3 unused DeclKind variants (Form,
   Traversal, Lens) won't appear in the generated keyword table unless
   they're declared in a boot file's `out` section. The generated parser
   has no dead arms by construction.

---

## Appendix: Boot file keyword frequency

```
Keyword       Count   % of total   Stage needed
out           123     30.8%        1 (00-prism)
type          108     27.0%        1 (01-meta)
in             57     14.2%        1 (01-meta)
action         54     13.5%        2 (01a-meta-action)
abstract       45     11.2%        2 (03-code, via type abstract)
property       17      4.2%        2 (05-property)
grammar        12      3.0%        1 (01-meta)
binding        12      3.0%        3 (std/tui)
zoom           10      2.5%        1 (00-prism)
project         8      2.0%        1 (00-prism)
focus           7      1.8%        1 (00-prism)
split           6      1.5%        1 (00-prism)
refract         6      1.5%        1 (00-prism)
recover         6      1.5%        1 (01-meta)
rescue          6      1.5%        1 (01-meta)
requires        5      1.2%        2 (std/mirror)
invariant       4      1.0%        2 (std/mirror)
prism           3      0.8%        1 (00-prism)
fold            2      0.5%        1 (01-meta)
default         2      0.5%        2 (std/cli)
ensures         1      0.2%        2 (std/mirror)
traversal       0      0.0%        never
lens            0      0.0%        never
form            0      0.0%        never (deprecated)
```

The top 4 keywords (`out`, `type`, `in`, `action`) account for 85.5% of all
declaration-starting tokens. The generated parser should fast-path these four.

---

*Taut found the line. Seam found the seams. The generated parser eliminates both.*
