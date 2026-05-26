# `@mirror/grammar` — self-hosted tokenization + spectral measurement

*2026-05-26. Mara. Proposal — not implementation.*

Status: **Yellow** (the recognition crystallized in conversation with Alex
during the #91 strict-mode fix; substrate primitives partially in place;
depends on `@hash/coincidence`, `@spectral/portal`, `@kintsugi/fracture`,
and the meta-grammar lift; no grammar changes shipped yet)

Depends on:
- `boot/std/mirror/grammar.mirror` (commit `—`) — the existing meta-glass.
  Carries the keyword table the bootstrap reads on every parse, plus the
  `refract <name> = <combinator>` declarations that the bootstrap currently
  IGNORES (lines containing `(` `>` `{` `}` are filtered out per
  `grammar.rs::parse_grammar`).
- `bootstrap/src/tokenize.rs` — the Rust tokenizer the meta-glass is
  meant to replace. Currently 893 lines of pattern-matching on bytes:
  action-decl, io-binding, match-expr, select-expr, comment, string,
  attribute, LLVM-IR-sigil, decorator, parametric-return, Dark-fallback.
- `boot/std/hash/coincidence.mirror` — `CoincidenceHash<5,5>`; the
  five-dimensions × five-projections shape; λ₀ = 0 as the Dark fallback.
  Spectral measurement composes naturally over this.
- `boot/std/spectral/portal.mirror`, `boot/std/spectral/portal/codec.mirror`,
  `boot/std/mirror/runtime/gen_prism.mirror` — the runtime surface the
  long-running tokenizer could live in as a gen_prism.
- `boot/std/kintsugi/fracture.mirror`,
  `boot/std/kintsugi/fracture/generic-brackets.mirror` — fracture as
  closure-operator on the AST lattice; tokenization gaps surface as
  candidate fracture rules.
- `boot/std/epistemologic/property/total_classification.mirror` —
  the contract `--strict` enforces: every source byte enters an AST
  node or errors. Now actually enforced (commit `ddd874a` for #91).
- `docs/insights/2026-05-26-kintsugi-as-credo-and-formatter-unified.md`
  — `--strict` should have caught syntax drift; detection IS
  transformation; the meta-bug is that the tokenizer's silent paths
  bypassed the substrate's measurement layer.
- `docs/insights/2026-05-26-epistemologic-reality-frame-scientific-
  grounding.md` — the altitude reframing: bootstrap-pull mistakes
  vs substrate-pull recognitions. This spec is squarely a
  substrate-pull recognition (the tokenizer wants to live in mirror).
- `docs/specs/bootstrap-retirement-plan.md` — Cluster D's
  `craft --target binary boot` regenerates the bootstrap from
  grammar. Tokenization is one of the last substrate pieces still
  expressed only in Rust; lifting it is on Cluster D's critical path.

Unblocks (deferred per LRM until consumers surface):
- `@mirror/grammar/measure` — spectral measurement on tokenization
  output; per-file holonomy; corpus-wide curvature; fracture ranking.
- `@kintsugi/fracture/list-brackets` — fracture rule for `-> [T]`
  return types (surfaced by the #91 strict audit; 10 boot files).
- `@kintsugi/fracture/requires-without-body` — fracture rule for
  bodyless `requires foo(arg)` declarations (8 boot files).
- `@kintsugi/fracture/decl-without-body` — fracture rule for
  bodyless `op name(args) -> type` declarations (15 boot files).
- Cluster D's `craft --target binary boot` self-rebuild.

---

## 1. Naming + placement

Three placements were considered:

1. `@mirror/ast/token` — tokenization as its own substrate, parallel
   to the AST.
2. `@mirror/grammar/token` — tokenization as a sub-grammar of the
   meta-glass.
3. `@mirror/grammar` (unified) — tokenization IS the meta-glass; the
   meta-glass already exists at `boot/std/mirror/grammar.mirror`.

**Recommendation: option 3 — unified `@mirror/grammar`.** The meta-glass
at `boot/std/mirror/grammar.mirror` already declares the lexical
primitives (`refract whitespace`, `refract identifier`, `refract name`),
the top-level forms (`refract grammar_form`, `refract prism_form`,
`refract abstract_form`, `refract io_form`), and the choice combinator
over all forms (`refract form = choice(...)`). The Rust tokenizer is the
thing that doesn't yet read these declarations; the meta-glass is
already the spec. The spec landing isn't a NEW substrate — it's the
activation of the existing one.

The alternative names would partition cleanly later if a real
separation emerges (e.g., a lexer/parser split). At the bootstrap's
current altitude, the partition is overhead. Tokenization, parsing,
and grammar-declaration are one mechanism: the meta-glass.

The Rust tokenizer becomes `@mirror/grammar/interpreter` (or stays
internal as `bootstrap/src/tokenize.rs` until Cluster D retires the
Rust source). The grammar-rule definitions ALL move to `.mirror`
files under `@mirror/grammar/*`.

---

## 2. The meta-grammar substrate

Mirror has grammars (action-decl, type-decl, io-binding, match-expr,
select-expr, parametric-return, ...). Today they're hardcoded as Rust
patterns in `scan_items`. To self-host, the grammar-for-grammars must
be expressible IN mirror.

The existing `@mirror/grammar` declares lexical primitives and form
shapes using combinators that don't yet have substrate-side semantics:

```mirror
refract whitespace        = repeat(charset(whitespace), 0, none)
refract identifier        = repeat(charset(word_char), 1, none)
refract reference         = seq(literal("@"), name)
refract comment           = seq(literal("#"), @nl(until_newline))
refract grammar_form      = seq(literal("grammar"), whitespace, reference,
                                paren_block, whitespace, brace_block)
refract form              = choice(comment, grammar_form, prism_form,
                                   abstract_form, io_form, match_form,
                                   select_form, in_form, out_form,
                                   refract_form, op_decl, dark_fallback)
```

The meta-grammar substrate is the set of combinators these declarations
use:

```mirror
# @mirror/grammar/combinator — the substrate-side combinator algebra.

grammar @mirror/grammar/combinator {
  # Primitive matchers (the leaves):

  action literal(s: text) -> matcher        { \ }   # match exact bytes
  action charset(c: char_class) -> matcher  { \ }   # match any of class
  action until(stop: matcher) -> matcher    { \ }   # match up to stop

  # Combinators (the constructors):

  action seq(ms: [matcher]) -> matcher           { \ }  # ordered sequence
  action choice(ms: [matcher]) -> matcher        { \ }  # first-match wins
  action repeat(m: matcher, lo: u64, hi: opt_u64) -> matcher  { \ }  # Kleene-bounded
  action optional(m: matcher) -> matcher         { \ }  # zero-or-one
  action lookahead(m: matcher) -> matcher        { \ }  # match without consuming

  # Block-shaped (the structural ones):

  action paren_block(body: matcher) -> matcher   { \ }  # `(` body `)`
  action brace_block(body: matcher) -> matcher   { \ }  # `{` body `}`
  action bracket_block(body: matcher) -> matcher { \ }  # `[` body `]`

  # The fallback (the λ₀ = 0 axis from @hash/coincidence):

  action dark_fallback() -> matcher              { \ }  # absorb any byte run
  # When no other matcher fires, dark_fallback coalesces the run of
  # unrecognized bytes into a Dark span. The fix for #91 (commit
  # ddd874a) IS this combinator's behavior; the spec lifts it from
  # hard-coded into substrate-declared.

  # Character classes:

  type char_class = whitespace | word_char | name_char | digit
                  | alpha | ascii_punct | any
}
```

The matcher type is opaque at the bootstrap altitude: a Fate-resolvable
hole the substrate doesn't yet know how to compile. The grammar
declares the shape; Stage 3 below makes the bootstrap evaluate it.

---

## 3. Tokenization rules expressed as mirror

The rules currently hardcoded in `scan_items` lift to combinator
declarations. The action-decl rule is the canonical example:

```mirror
# @mirror/grammar/action_decl — the declaration form.
#
#   <decorator>? <name>(<args>)? -> <type>(<params>)? { <body> }
#
# Decorator is optional (`property`, `fn`, `template`, `select`, ...).
# Argument parens are optional (nullary forms: `serve -> imperfect { ... }`).
# Return type may be parametric: `T(U)` (canonical) or `T<U>` (deprecated
# foreign-language drift; @kintsugi/fracture/generic-brackets canonicalizes).
# Body is `{ ... }`, possibly multi-line, possibly an obligation marker `\`.
#
# The #91 strict audit surfaced 15 files with bodyless declarations
# (`-> type` with no `{ ... }`). @kintsugi/fracture/decl-without-body
# fractures these to canonical form (the spec is open whether
# canonicalization adds an empty `{ }` or admits the bodyless form).

in @mirror/grammar/combinator

grammar @mirror/grammar/action_decl {

  refract decorator = choice(literal("property"), literal("fn"),
                             literal("template"), literal("select"))

  refract type_ref  = seq(name, optional(paren_block(repeat(type_ref, 0, none))))

  refract args      = repeat(seq(name, optional(literal(":"), whitespace, type_ref)),
                             0, none)

  refract body      = brace_block(repeat(any, 0, none))  # opaque-to-substrate

  refract form      = seq(optional(decorator), whitespace,
                          name,
                          optional(paren_block(args)),
                          whitespace, literal("->"), whitespace,
                          type_ref,
                          whitespace,
                          body)
}

out form
out @mirror/grammar/action_decl
```

Similar lifts for the other forms:

```mirror
# @mirror/grammar/io_binding — the Spec A io-lambda binding form.
#
#   io <name>(<args>) = <lens-call> > <selector>

# @mirror/grammar/match_expr — the Spec B match form.
#
#   match <subject> { <arm> => <body>, ... }

# @mirror/grammar/select_expr — the Spec B select form.
#
#   select |<binder>| { <variant>(args) => <body>, ... }

# @mirror/grammar/comment — line and block comments.
#
#   //  -- /* */  (LLVM-IR also: ;)

# @mirror/grammar/string — string literal with escape sequences.
#
#   "..."  with \" \\ \n escapes; interpolation `{var}` is currently
#   not specified (boot/std/hash/coincidence.mirror surfaces this).
```

The full corpus of forms maps 1:1 from `scan_items` branches to
grammar-side `refract <name> = <combinator>` declarations. The lift
is mechanical IF the substrate can evaluate the combinators (Stage
3 below).

### The byte-coverage contract as a property

The `--strict` guarantee — every source byte enters an AST node or
errors — lifts to a property over the form choice:

```mirror
in @epistemologic/property/total_classification

grammar @mirror/grammar/total_classification {
  # The form choice closes the byte axis: dark_fallback is the
  # λ₀ = 0 element guaranteeing total coverage.
  requires last_alternative(form, dark_fallback)
  requires total(form)   # every byte enters some alternative
}
```

The substrate enforces totality by inspecting the `form` choice's last
alternative; if it isn't `dark_fallback`, `--strict` rejects the
grammar itself (not just the input). The contract is a meta-property
of the meta-glass.

---

## 4. The bootstrap-reads-grammar path

The bootstrap currently hardcodes tokenization in Rust. Self-hosting
requires reading the grammar at startup and dispatching to it.

### The interpreter shape

The Rust side becomes a deterministic combinator interpreter:

```rust
// bootstrap/src/grammar_interp.rs (post-lift)

enum Matcher {
    Literal(Vec<u8>),
    Charset(CharClass),
    Until(Box<Matcher>),
    Seq(Vec<Matcher>),
    Choice(Vec<Matcher>),
    Repeat { inner: Box<Matcher>, lo: usize, hi: Option<usize> },
    Optional(Box<Matcher>),
    Lookahead(Box<Matcher>),
    ParenBlock(Box<Matcher>),
    BraceBlock(Box<Matcher>),
    BracketBlock(Box<Matcher>),
    DarkFallback,
}

fn match_at(m: &Matcher, bytes: &[u8], pos: usize) -> Option<(usize, AstNode)> { ... }
```

The interpreter is small (target: < 500 lines). It carries no rule
logic — only the algebra. Adding a new form means adding a `.mirror`
file under `@mirror/grammar/*`, not editing Rust.

### Determinism (no @fate stochasticity)

The tokenizer must be deterministic: same source bytes, same AST.
No @fate dispatch at the tokenization altitude — @fate is for
inference-level holes (loss > 0). Tokenization holes are structural
failures and surface as Dark spans, not as Fate-resolvable obligations.

This pairs cleanly with `@epistemologic/property/halts`: the
combinator interpreter halts by construction (bounded repeat;
first-match-wins choice; no left-recursion in the meta-glass).

### Grammar lookup at startup

On first invocation, the bootstrap loads the grammar for the file
being compiled (currently `grammar_for_file` in Rust). With
self-hosting, the bootstrap loads `@mirror/grammar` itself, then
uses it to tokenize all subsequent input INCLUDING the next grammar
file.

The meta-grammar lookup table is a small fixed file the Rust
interpreter reads literally (the bootstrap's lookup table is the
only piece that can't be self-described — it's the seed). Every
other grammar is loaded via the interpreter.

---

## 5. Round-trip closure (the butterfly's wing for tokenization)

The ultimate test: bootstrap compiles its own grammar to produce a
new bootstrap that uses the same grammar. Self-hosting verification.

### The round-trip path

```
  source.mirror
    |
    | tokenize via @mirror/grammar (interpreter reads grammar)
    v
  AST
    |
    | shatter via @mirror/spectral.crystallize
    v
  crystal (oid)
    |
    | reflect via kintsugi (canonical-source emission)
    v
  source'.mirror
```

`source == source'` for canonical input. For non-canonical input,
`canonical(source) == source'` (kintsugi's idempotence law from
@kintsugi/fracture).

### Composition with `craft --target binary boot`

Cluster D's plan (per `docs/specs/bootstrap-retirement-plan.md` and
`docs/specs/craft-binary-target.md`): `craft --target binary boot`
compiles the boot tree into a binary that IS the next bootstrap.
The new bootstrap reads its own grammar; the grammar reproduces the
binary; the cycle closes. This is the butterfly's wing — the
self-hosting fixed point.

At the tokenization altitude, the test is one OID equality:

```bash
oid_of(@mirror/grammar via current bootstrap)
  ==
oid_of(@mirror/grammar via the bootstrap that just regenerated itself)
```

If the OIDs match, the tokenizer is self-hosted by construction.

---

## 6. Spectral measurement integration

*The big payoff Alex sees.* Currently `--strict` is binary: errors or
passes. The fix for #91 (commit `ddd874a`) made the binary axis
honest, but the substrate of `@hash/coincidence` and `@spectral`
admits continuous measurement that's strictly more informative.

### Holonomy on `source → AST → shatter → source`

The round-trip from §5 is a closed loop. The substrate carries
holonomy as a first-class invariant. For canonical input, holonomy
is 0 by construction. For non-canonical input, holonomy localizes
drift to specific bytes.

```mirror
in @mirror/grammar
in @hash/coincidence

grammar @mirror/grammar/measure {

  # Per-file holonomy: 0.0 means perfect tokenization; > 0.0
  # quantifies drift. The drift is byte-localized via the Dark
  # span source ranges (`DarkSpan { start, end }` in the AST).
  action holonomy(file: text) -> f64                  { \ }

  # The drift map: which byte ranges contribute non-zero curvature.
  action drift_ranges(file: text) -> [(u64, u64, f64)] { \ }
}
```

The binary `--strict` axis collapses to `holonomy(file) == 0.0`. The
continuous axis lets the gutter render drift as amber-graded color
(per `docs/specs/gutter-lenses.md`).

### Conductivity through the tokenization graph

The tokenization graph: nodes = grammar rules; edges = `seq`/`choice`
composition; weights = match frequency on the corpus. Conductivity
is a spectral invariant (per @hash/coincidence's `ricci` projection)
measuring how well rules compose. Conflicts (e.g., two `choice`
alternatives both matching the same byte sequence) show as low
conductivity.

```mirror
  # Per-rule conductivity through the choice graph. Low conductivity
  # localizes substrate-pull at the rule altitude (a rule that almost
  # nothing reaches; or a rule that catches everything).
  action conductivity(rule: ref) -> f64                { \ }
```

### Fiedler value of the AST

The AST is a graph (nodes = nodes; edges = parent/child). Its Fiedler
value (the second-smallest eigenvalue of the Laplacian) distinguishes
"well-formed" from "almost-well-formed" structurally — a continuous
good-shape metric.

```mirror
  action fiedler(ast: ref) -> f64                      { \ }
```

### Fracture ranking by corpus curvature

`@kintsugi/fracture/<rule>` instances candidate-rank by total corpus
curvature they'd heal. Ranking becomes spectral, not ad hoc:

```mirror
  # For each candidate fracture, sum the holonomy across the boot
  # corpus that would settle to 0.0 after applying the fracture's
  # closure operator. Higher numbers — prioritize the fracture.
  action rank_fractures(corpus: [text]) -> [(fracture, f64)]  { \ }
```

Applied to the #91 strict audit's 67 surfaced files: the three
predicted fracture rules (list-brackets, requires-without-body,
decl-without-body) would each get a ranked curvature number; the
highest-curvature rule lands first.

### Composition over `@hash/coincidence`

The five-dimensions × five-projections shape of
`@hash/coincidence` already carries the substrate's spectral axes:

- `entropy` projection — information content of the AST
- `spectral` projection — Laplacian eigenvalues (the Fiedler
  value lives here)
- `cheeger` projection — graph cut quality
- `ricci` projection — curvature concentration (the holonomy
  and conductivity measures live here)
- `mixing` projection — random-walk mixing time

The tokenization measurement substrate IS a projection-by-projection
lift of `@hash/coincidence` over the AST graph. No new substrate —
just application.

---

## 7. Migration path

Staged transition from current Rust tokenizer to self-hosted.

### Stage 1 — Define `@mirror/grammar/combinator` substrate

Declarations only; no evaluator yet. The `.mirror` files land but
the bootstrap doesn't read them — it continues to use
`bootstrap/src/tokenize.rs`. The lift is documentation at this stage.

**Deliverables**:
- `boot/std/mirror/grammar/combinator.mirror` (the matcher type + actions)
- `boot/std/mirror/grammar/action_decl.mirror` (one form as proof-of-concept)
- Round-trip tests: AST of `action_decl.mirror` carries the same OID
  as the Rust tokenizer produces for the equivalent source.

**Validation gate**: `@kintsugi/fracture` round-trip on these files
holds (closure-operator idempotence).

### Stage 2 — Express ALL tokenization rules as `@mirror/grammar/*`

The full corpus of `scan_items` branches lifts to grammar-side
declarations. Hand-verified against Rust behavior (round-trip tests
on the entire boot tree).

**Deliverables**:
- `boot/std/mirror/grammar/{io_binding,match_expr,select_expr,comment,
  string,attribute,llvm_sigil,decorator,parametric_return,dark_fallback}
  .mirror`
- The `total_classification` property declared as a meta-property
  of the form choice.
- 100% behavioral parity with `bootstrap/src/tokenize.rs` on the
  boot corpus (147 files; 67 of them carry post-#91 darks that should
  remain unchanged byte-for-byte after the lift).

**Validation gate**: corpus-wide OID equality — every file in `boot/`
produces the same OID through both tokenizers.

### Stage 3 — Bootstrap reads its own grammar at startup

The Rust tokenizer becomes the COMBINATOR INTERPRETER, not the rule
registry. Rule logic moves out of Rust into `.mirror`. The Rust
code is small (< 500 lines target) and never grows again.

**Deliverables**:
- `bootstrap/src/grammar_interp.rs` — the combinator interpreter
  (`[bugfix:restore]` doesn't apply here — this IS a feature lift,
  but the lift is the bootstrap-retirement plan's Cluster D deliverable;
  authorization comes from the cluster, not the FROZEN carveout).
- `bootstrap/src/tokenize.rs` deleted; replaced by
  `bootstrap/src/grammar_load.rs` (load the meta-grammar) +
  `grammar_interp.rs` (evaluate it).
- `cargo test -p bootstrap` continues to pass; OID smoke tests
  unchanged.

**Validation gate**: bootstrap compiles its own grammar to the same
OID before/after Stage 3 (the butterfly's wing closes for the
first time at this stage).

### Stage 4 — Add spectral measurement substrate

`@mirror/grammar/measure` lands as a queryable surface. The gutter
(`docs/specs/gutter-lenses.md`) reads the measurement output and
renders amber-graded drift. `--strict` becomes `holonomy(file) == 0.0`
in terms of the measurement substrate.

**Deliverables**:
- `boot/std/mirror/grammar/measure.mirror` (the measurement actions)
- Gutter integration (the existing red/amber/green stays binary; a
  new gradient mode reads measure.holonomy)
- Fracture ranking lands; `mirror kintsugi --propose` lists
  candidate fractures ordered by curvature.

**Validation gate**: the #91 audit's 67-file drift gets ranked; the
three predicted top fracture rules surface at the top of the
ranking by curvature; the smaller categories (4–8 in the commit
message) are ranked below.

### Stage 5 — Rule logic ALL grammar-side; Rust is plumbing

The Rust source ships only:
- the combinator interpreter (~500 lines),
- the SHA-256 + CoincidenceHash kernel,
- `git hash-object -w` storage,
- the CLI dispatch.

Every grammar rule, every tokenization shape, every measurement
actor lives in `.mirror`. Adding a new form is one `.mirror` file.

### Stage 6 — Cluster D's butterfly

`craft --target binary boot` regenerates the bootstrap from grammar.
The meta-grammar IS the spec. The bootstrap that emerges reads the
same grammar; the cycle closes; the Rust source becomes vestigial
per Cluster D's plan.

---

## 8. What the 67 surfaced drifts inform

The `--strict` audit (commit `ddd874a`) surfaced 67/147 boot files
with hidden tokenization drift. Categorized:

| Category | Pattern | Files | Fracture rule |
|---|---|---|---|
| 1 | `op name(args) -> type` without `{}` | ~15 | `@kintsugi/fracture/decl-without-body` |
| 2 | `-> [T]` / `-> [T \| U]` list-type return | ~10 | `@kintsugi/fracture/list-brackets` |
| 3 | `requires foo(arg)` without `{}` | ~8 | `@kintsugi/fracture/requires-without-body` |
| 4 | `name = value` plain assignments | ~4 | `@kintsugi/fracture/decl-assignment` |
| 5 | `\| variant` pipe-alternation lines | ~5 | `@kintsugi/fracture/sum-type-alternation` |
| 6 | `-> T<U>` foreign-language parametric | ~2 | `@kintsugi/fracture/generic-brackets` (exists) |
| 7 | `refract name = combinator(...)` | 2 | `@kintsugi/fracture/refract-combinator` |
| 8 | misc (foreign body, string interp, ...) | ~5 | (case-by-case) |

Categories 1–3 are the brief's predicted three fracture rules — they
dominate. Each lifts to a grammar-side `refract <name> = ...`
declaration in `@mirror/grammar/*` AND a fracture rule in
`@kintsugi/fracture/<name>` that canonicalizes the source.

Categories 4–7 are smaller surfaces but structurally the same shape:
a declaration form the meta-glass doesn't yet admit. Each is one
`refract <name> = ...` line per form + one fracture rule per
canonicalization.

Category 8 contains substrate gaps that surface design calls (see
§9). Inline foreign code (`std::fs::read_to_string(...)`) belongs
under `@io` per glass_wall; string interpolation `{var}` needs a
specified semantics; hyphenated paths after `out @` need either a
fracture rule (replace `-` with `_`) or a tokenizer admission.

---

## 9. Open design calls

Questions the implementation will need to resolve. Surface them; don't
try to answer all.

### 9.1 — `matcher` as a type: opaque or structured?

The `matcher` type in `@mirror/grammar/combinator` is currently
opaque (a `\` Fate-hole). Three options:

- **Opaque**: the substrate's interpreter dispatches by name on the
  declaration RHS; no structural matcher value.
- **Algebraic**: `matcher` is a sum type (`literal | charset | seq |
  choice | ...`) declared in mirror; the interpreter pattern-matches.
- **Closure**: `matcher = bytes → imperfect(ast)`; the interpreter
  is just function application.

Leaning algebraic (#2) for OID-stability and inspectability. Closures
shipped as Fate-holes (#3) are the obvious lift later.

### 9.2 — Bodyless declarations: admit or canonicalize?

15 boot files use the bodyless `op name(args) -> type` form.
@kintsugi/fracture/decl-without-body could either ADMIT the bodyless
form (extend the grammar) or CANONICALIZE to `{ \ }` (the obligation
marker, signaling "body deferred to Fate"). Each is a substrate-pull
direction.

Leaning canonicalize — the obligation marker is the existing
Fate-handshake; admitting a bodyless form bifurcates the AST shape.

### 9.3 — List-type brackets: `[T]` admitted, or canonicalize to `list(T)`?

10 boot files use `-> [T]`. The canonical form per the existing
parametric-return rule is `list(T)`; the fracture is one rewrite.

Leaning canonicalize, BUT: the bracket syntax is widely-recognized
in functional languages and Alex may want it admitted as sugar.
Design call defers to Alex.

### 9.4 — String interpolation `{var}`: tokenizer-level or post-parse?

`boot/std/hash/coincidence.mirror` line 62: `"coincidence:projection:
{i}:{projections}"`. The interpolation IS part of mirror semantics
or a foreign-grammar lift?

Leaning post-parse: the string carries verbatim bytes; a downstream
lens (`@nl/string`, `@text/interpolate`) extracts the `{var}` slots
from the bytes. The tokenizer stays oblivious.

### 9.5 — Hyphenated paths in `out @a/b-c`

The `out` rule's name reader uses `is_name_char` which excludes `-`.
Boot file `boot/std/epistemologic/math/spectral-triple.mirror` ships
the pattern. Either extend `is_name_char` (substrate-pull: "names
can contain hyphens") or canonicalize the path (substrate-pull:
"underscores not hyphens").

Leaning the substrate-pull direction Alex picks. Both are coherent;
the choice is taste.

### 9.6 — Inline foreign code: glass_wall enforcement

`boot/std/file.mirror` ships `std::fs::read_to_string(...)` inline.
Per `@epistemologic/property/glass_wall`, non-mirror grammars must
be under `@io`. The foreign-code line should be inside an `@io {
... }` block or behind a `~rust"..."` sigil.

Leaning fracture (move to `@io.read_text(path)` etc.) — the
glass_wall property is structural; the violation surfaced by #91
is a kintsugi opportunity.

### 9.7 — `@fate` at the tokenizer altitude

The tokenizer must halt; `@fate` is permitted at the AST altitude
but not at tokenization. §4 declares this; the open question is
whether ANY combinator can return a Fate-hole, or whether
Fate-resolution waits until a later pipeline stage.

Leaning: combinators may carry Fate-holes for BODIES (between
balanced delimiters) but not for STRUCTURE (the choice + seq +
repeat algebra is deterministic).

### 9.8 — Performance: interpreter overhead vs hardcoded Rust

The Rust tokenizer is ~893 lines of dense pattern-matching. The
combinator interpreter would be ~500 lines of dispatch. The
performance ratio is unmeasured. Cluster D's plan acknowledges
the lift may cost 2-5× tokenization time — acceptable for the
self-hosting payoff, but worth measuring.

Leaning: ship the interpreter; measure; optimize if the cost is
over-budget for `mirror compile` SLA.

### 9.9 — `total_classification` as meta-property

The `last_alternative(form, dark_fallback)` invariant is a property
over a grammar declaration, not over source. The substrate doesn't
currently support meta-properties (properties OF the grammar, not
OF the source the grammar tokenizes). The lift requires extending
the property substrate — a small reach, deferred.

---

## 10. Composition with existing substrate

### `@hash/coincidence`

Naturally pairs. The OID of a `.mirror` file already lives in
`CoincidenceHash<5,5>` space. The spectral measurement substrate of
§6 IS application of the same five projections (entropy, spectral,
cheeger, ricci, mixing) over the AST graph. No new substrate — just
application.

### `@mirror/runtime/gen_prism`

The tokenizer in a long-running deployment IS a gen_prism: it
ticks on each `mirror compile` invocation; its head is the grammar
at startup; its ancestor chain is the corpus of files tokenized.
Long-running deployments (the spectral.engineer cloud per
`docs/specs/road-to-1.0.md`) keep the gen_prism warm; cold-start
deployments respawn per-invocation.

### `@kintsugi/fracture`

Fracture rules operate on the AST; the tokenizer produces the AST.
The loop closes: a fracture detects drift in tokenization output;
the canonicalized source re-tokenizes; the new AST settles. The
#91 audit data IS the prior art for the first three fracture rules
the substrate ships.

Fracture confidence (per
`docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md`)
grades the autonomous-apply axis: high-confidence fractures
(list-brackets, generic-brackets) auto-apply; lower-confidence
fractures (string-interp semantics, hyphenated paths) enter a
`@scene` with the curator.

### `@spectral` (db, eigenvalues, conductivity)

The measurement substrate of §6 IS a thin lens over the existing
spectral substrate. Per-file holonomy is one Laplacian eigenvalue
query; conductivity is one Ricci-projection query; Fiedler is the
second-smallest eigenvalue of the AST's Laplacian. The substrate
ALREADY computes these for crystals (per
`docs/specs/eigenboard-representation.md`); the lift exposes them at
the tokenizer altitude.

### `@epistemologic/property/total_classification`

The existing property declared the byte-coverage contract; the #91
fix made the bootstrap honor it. The meta-property `last_alternative
(form, dark_fallback)` lifts this from a source-level property to
a grammar-level property — the substrate enforces that EVERY
grammar covers its byte axis.

### `@epistemologic/property/halts`

The combinator interpreter must halt by construction. Per the
sub-Turing discipline (`requires halts(g)` in
`boot/std/mirror/runtime/gen_prism.mirror`), the interpreter's
language (the combinator algebra) is bounded: no left-recursion in
the meta-glass; bounded repeat; first-match-wins choice. The halt
property composes through.

### `@spectral/portal`

Distributed tokenization: a grammar file at one node, source at
another. The portal carries both bytes (source) and content-addressed
fragmentations (grammar OID). One node can tokenize against a
grammar it hasn't seen, by reading the OID through the portal. Per
`boot/std/spectral/portal.mirror`'s typed-transport-over-content-
addressed-subspace architecture, this is one composition away.

---

## Forward look

The spec lands as a yellow-status proposal. Implementation lives in
Cluster D's bootstrap-retirement plan; the lift unblocks the
butterfly's wing.

The #91 fix made `--strict` honest; this spec makes it CONTINUOUS.
Binary error/pass → holonomy gradient. Ad-hoc fracture lists →
spectrally-ranked candidate rules. Hardcoded Rust patterns →
grammar-side declarations. Three altitude shifts; one substrate.

The substrate IS the measurement.

---

*End of spec.*
