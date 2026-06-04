# Parser-as-Prism — grammars are combinator trees, kintsugi is the fixed point

*2026-05-21. Reed.*

Status: **Red** (the recognition; the combinator surface is named; the
inverse fold's shape lands here and the implementation follows in
sub-ticks 4b and 4c)

Depends on:
- `docs/specs/ast-as-bundle.md` — the AST IS a Bundle written as data; the
  five operation `AstKind`s are the trait-chain levels, `In`/`Out` are
  typed terminals. The forward fold (AST → anything) is a `Fold5`.
- `docs/specs/prism-core-as-spectral-triple.md` — the (A, H, D) substrate.
  This spec writes the parser as a specific element of A.
- `docs/specs/spectral-triple-grammar.md` — the supertrait closure
  (`Optic`, `GroupStructure`, `LawvereFixedPoint`, `Metric`) that
  combinators inherit by impl-ing `Prism`.
- `docs/specs/bootstrap-retirement-plan.md` §Tick 4 (lines ~557–600).
  This spec is sub-tick 4a; sub-ticks 4b and 4c land the implementation.
- `docs/specs/strict-and-total-classification.md` — the contract every
  combinator must preserve (Dark count byte-stable, strict-failure
  exit code 2, per-file diagnostic format).
- `docs/specs/kintsugi-formatter.md` — the contraction map the parser's
  inverse closes into. Tick 2's scaffold folds into this tick (see
  §"Kintsugi fixed-point property" below).
- `bootstrap/src/tokenize.rs` on `reed/v1-floor` — the 768-line
  hand-written recursive descent this spec retires.
- `bootstrap/src/grammar.rs` on `reed/v1-floor` — the 232-line
  keyword-table loader retired in sub-tick 4c.
- `bootstrap/src/spectral.rs` on `reed/v1-floor` — the home of `Fold5`,
  `Fold5At`, `apply_h`, `ContentOidPrism`. The combinator surface lands
  here next to its fold dual.

Unblocks:
- Tick 4b lands the combinator implementations grammar by grammar.
- Tick 4c retires `grammar.rs` entirely.
- Tick 5 (`pipeline.rs` retirement) reuses the same surface for mq
  query parsing.
- The mirror compiler becomes self-hosting at the parser level: the
  Rust seed encodes just-enough-meta-glass to parse
  `mirror/glass.mirror`, the full grammar of `.mirror` files
  (including its own form). `apply_h(meta_glass, glass.mirror.bytes)
  == meta_glass` is the load-bearing fixed point. Every other
  grammar — `mirror/00-prism.mirror`, `mirror/code/rust.mirror`,
  `mirror/code/llvm/ir.mirror`, the entire boot tree — loads through
  `apply_h(meta_glass, …)`.
- The boot/ tree retires. `mirror/` is the only source folder. The
  rename `grammar → glass` and the directory migration are one
  kintsugi pass: `mirror kintsugi boot/ --transform='grammar =>
  glass' --out=mirror/`. The transform is structural — only tokens
  the meta-glass parses as the keyword `grammar` (and the file
  basename derived from it) get rewritten.
- Host grammars defer to other grammars via the cross-grammar shift
  `@<grammar>(<body>)`. Comments in every host shift to `@nl` (the
  natural-language grammar); fenced code blocks with language tags
  dispatch back to the tagged host. The bare-`@nl` form (this tick)
  recognises inline backticks, fenced blocks, and `mirror>` doctest
  prompts. A full `@nl/markdown <@data/markdown` arrives in a
  follow-up tick with `docs/specs/grammar-inheritance.md`.

---

## Recognition

The AST-as-bundle recognition named one half of the picture: any
operation that *consumes* an AST is a `Fold5` instance — a bundle
morphism into a target type. The forward direction.

This spec names the other half. Any operation that *produces* an AST
from bytes is a `Prism` whose `Beam<In>` carries source bytes and whose
`Beam<Refracted>` carries an `AstNode`. The parser IS that Prism. A
grammar IS the data form of that Prism — a tree of named combinators
that, applied to its bytes by the evaluator, yields an AST.

The two directions are dual. `Fold5` is the catamorphism (AST → Out).
Parser-as-Prism is the anamorphism (In → AST). Together they close the
kintsugi loop: every `.mirror` file in `boot/` is at the fixed point
`tokenize ∘ render = id_AST` and `render ∘ tokenize = id_bytes (modulo
canonical formatting)`. The loop iterates to that fixed point because
both directions are now elements of A and compose via `compose_a`.

There is no second algebra to introduce. The parser doesn't get its own
machinery. It's an element of the same A that `Fold5` instances inhabit
— the supertrait closure (`Connection::Optic: Prism`,
`Transport::Holonomy: Metric`, …) covers both directions.

What changes from the hand-written tokenizer is *not* the bytes it
produces. The corpus is byte-stable across Tick 4. What changes is the
surface: the parser becomes data the grammar declares; the dispatch
becomes one composition law instead of 768 lines of recursive descent
with bespoke handlers for `io`/`match`/`select`/llvm-ir body capture.

---

## The `Combinator` surface

### Trait or enum?

**Decision: enum.** `Combinator` is a closed enum, one variant per
named combinator. A single `impl Prism for Combinator` dispatches via
match. The surface is enforced syntactically — adding a combinator
means editing the enum AND adding a match arm. Four load-bearing
reasons:

1. **The closed surface matches the philosophy.** The void pentagon
   is closed; the five operations are closed; the combinator
   vocabulary is closed. An enum is honest about this. A trait is
   nominally open and needs a sealed-trait pattern to recover the
   honesty.
2. **The seed is data, not a tree of trait objects.** Per §"Bootstrap
   loop" below, the seed is the Combinator-tree encoding of
   `boot/00-prism.mirror`. Combinators-as-data wants an enum: the
   seed is an enum literal, the tree is serializable, and
   `combinator_tree_oid` is a straight Merkle hash over variants. A
   trait-object tree would need vtable-pointer hashing or a separate
   Repr type.
3. **Binary size.** Per the Tick 3 measurement (`Fold5` added +9.5KB
   from monomorphisation before claw-back), trait-per-combinator
   risks a similar bloat at every `apply_h` site. The enum form is
   one match — one dispatch point, no generic explosion. We are
   optimising for ≤240KB; the +5–10KB matters.
4. **`compose_a` integration is one wrap.** The trait-composes-
   natively advantage is real but small: a single `impl Prism for
   Combinator` makes the enum compose through `compose_a` the same
   way. The cost is one match in `apply_h`, not per-variant
   ceremony.

Fallback: if the match arm in `apply_h` exceeds ~200 lines as the
vocabulary grows, split each variant's body into a free function the
arm calls. The enum stays closed; the dispatch stays one match.

The enum shape:

```rust
pub enum Combinator {
    // Primitives
    Seq(Vec<Combinator>),
    Choice(Vec<Combinator>),
    Repeat { body: Box<Combinator>, min: usize, max: Option<usize> },
    Capture { body: Box<Combinator>, kind: AstKind },
    Literal(Vec<u8>),
    Charset(CharsetKind),                  // closed enum, not a fn ptr
    BraceBlock(Box<Combinator>),
    ParenBlock(Box<Combinator>),
    // Named compositions
    IoBinding,
    MatchArm,
    SelectVariant,
    KeywordFormBody { keyword: Vec<u8>, kind: AstKind },
    // Fallback (named so strict classification has one home)
    DarkFallback,
}

impl Prism for Combinator { /* one match in apply_h */ }
```

`CharsetKind` is a small named enum (`WordChar`, `NameChar`,
`IrIdentChar`, `IoNameChar`, …) rather than a function pointer, so
the combinator tree stays serializable. Adding a charset means
adding a variant — same closure discipline as the outer `Combinator`.

The `Beam<In>` for `Combinator` carries `(source: &[u8], offset:
usize)`; the `Beam<Refracted>` carries `Imperfect<AstFragment,
Infallible, ScalarLoss>` where `ScalarLoss` is the Dark-region
accumulator (per `Transport::Holonomy: Metric`). The walker is
`apply_h(combinator, (source, 0))`; the verdict's `Success` payload
is an `AstNode` (after the root `Seq` lifts its fragments).

### The named vocabulary

Each combinator is one named impl. The vocabulary is closed: extending
it requires extending this spec and adding a hand-coded impl in
`spectral.rs`. The vocabulary covers everything the current 768 lines
of `tokenize.rs` does.

**Primitives.**

- **`seq([C; N])`** — parse N combinators in order. Verdict is
  `Success(children)` if all succeed; `Partial(children_so_far,
  accumulated_loss)` otherwise. `Failure` is structurally absent
  (combinators emit Dark fragments instead of failing).
- **`choice([C; N])`** — first non-Partial wins; ties broken by
  smallest `ScalarLoss` (`terni::Metric::distance_to`); zero-progress
  fall-through hands off to `dark_fallback`. The Rust grammar's
  `is_skip_word` table becomes one `choice` arm of empty-settle
  `literal`s.
- **`repeat(C, min..=max)`** — Kleene with bounds. Termination
  follows from `Transport::Holonomy: Metric`: per-iteration loss is
  non-negative, total loss is bounded by the source's length.
- **`capture(C, kind: AstKind)`** — wrap the span consumed by `C` as
  an `AstNode` of `kind`, reading the `name` from a sub-combinator
  fragment. The combinator→AST seam.
- **`literal(&[u8])`** — exact byte match. Atomic. `if word ==
  "focus"` collapses to `literal(b"focus")`.
- **`charset(predicate: fn(u8) -> bool)`** — byte-class. The four
  predicates in `tokenize.rs` (`is_word_char`, `is_name_char`,
  `is_ir_ident_char`, `is_io_name_char`) become four `charset`
  instances.
- **`brace_block(C)`** — balanced `{…}` (the `scan_brace_block`
  body), then `C` over the inner bytes.
- **`paren_block(C)`** — balanced `(…)` (the `scan_paren_block`
  body).

**Named compositions** (each one is a `seq`/`choice` tree the seed
hand-codes; collected into named combinators because the grammar
files refer to them by name).

#### `io_binding` — the `io <name>(<args>) = <lens-call> > <selector>` form

Currently the `IoBinding` AstKind, handled via `Fold5::on_other`. This
combinator composes:

```
seq([
  literal(b"io"),
  charset(is_io_name_char),                    # name
  optional(paren_block(args_combinator)),      # args list
  literal(b"="),
  capture_until(newline_or_eol_continuation),  # the rhs body, verbatim
])
.shift(AstKind::IoBinding)
```

The `capture_until` here is the existing `capture_io_body_end`
function expressed as a combinator: scan forward respecting line-
continuation markers (`=`, `,`, `(`, `[`, `>` at end-of-line). The
combinator's body output is the verbatim bytes between the `=` and
the terminating newline — the same string the current tokenizer
emits.

#### `match_arm` — the `match <subject> { <arm> => <body>, … }` form

Currently the `MatchExpr` AstKind, handled via `Fold5::on_other`.
Composes:

```
seq([
  literal(b"match"),
  capture_subject_until(brace_or_newline),     # subject expression
  brace_block(repeat(arm_combinator, 0..)),    # the arm list
]).shift(AstKind::MatchExpr)
```

The arm_combinator is `seq([pattern, literal(b"=>"), body, optional(literal(b","))])`,
where `pattern` and `body` are themselves combinators describing the
match grammar's arm shape. For Tick 4 the body remains verbatim (the
current tokenizer keeps it as opaque bytes); a future spec — once
`@mirror/match.parse_match` closes — replaces the verbatim body with
a structured pattern fragment.

#### `select_variant` — the `select |<binder>| { <variant> => <body>, … }` form

Currently the `SelectExpr` AstKind, handled via `Fold5::on_other`.
Same shape as `match_arm` with `select` keyword and the `|x|` binder
in the header.

#### `keyword_form_body` — the LLVM-IR body capture special case

The `@code/llvm/ir.mirror` grammar has a unique shape: a keyword
(`define`, `declare`, `target`, `attributes`, …) is followed by
verbatim bytes that may include a brace block (`define i32 @foo() {
… }`). The current tokenizer handles this in a dedicated `llvm &&
word_at_line_start` branch.

This is a single specialised combinator, not a general composition,
because the body capture must look ahead for an opening `{` on the
same line (continuation) versus an end-of-line (statement). It
composes more atomic combinators internally:

```
keyword_form_body(keyword: &'static [u8], kind: AstKind) =
  seq([
    literal(keyword),
    skip_hspace,
    optional(sigil_prefix),                    # @, %, !, #
    charset(is_ir_ident_char),                 # the name
    capture_verbatim_body_until_eol_or_brace_block,
  ]).shift(kind)
```

The composition is named once as `keyword_form_body` so the llvm-ir
grammar's combinator tree can refer to it by name, the same way the
mirror grammar refers to `io_binding`. It's not a primitive; it's a
named composition. Naming it explicitly keeps llvm-ir's grammar file
small (one reference per keyword) and keeps the dispatch readable.

#### `dark_fallback` — the unrecognised-bytes combinator

See §"Dark spans" below. This is the fallback combinator at every
`choice` site: when every arm declines to advance, `dark_fallback`
captures the unrecognised span as a Dark fragment. It's named
explicitly so the strict-classification contract is one combinator's
responsibility, not a property scattered across every `choice` impl.

---

## The `on_other` resolution

Tick 3's `Fold5::on_other` reducer folds six AST kinds into one arm:
`In`, `Out`, `Dark`, `IoBinding`, `MatchExpr`, `SelectExpr`. That
reducer IS the debt this spec resolves.

| AstKind      | Resolution                                                                                |
|--------------|-------------------------------------------------------------------------------------------|
| `In`         | Bundle's typed `In` terminal at root `seq([in_decls, body, out_decls])`. Tagged children still emit for round-trip rendering, but the *combinator* sees them as the bundle's domain, not an `on_other` case. |
| `Out`        | Same as `In`, codomain side.                                                              |
| `IoBinding`  | Produced canonical-shaped by the `io_binding` combinator (above) via `capture`. No catch-all dispatch.    |
| `MatchExpr`  | Produced by `match_arm`.                                                                  |
| `SelectExpr` | Produced by `select_variant`.                                                             |
| `Dark`       | Survives as a marker, produced exclusively by `dark_fallback`. The strict-classification sentinel. |

After Tick 4, `Fold5::on_other` shrinks to one arm (`AstKind::Dark`)
and is renamed `on_dark`. The 8-kind AST stays — Dark survives — but
the runtime catch-all dissolves. Each Spec A/B kind is now *produced*
canonically by its combinator and *consumed* canonically by its kind
arm in `Fold5`.

**Deviation from the original Tick 4 plan.** The plan kept
`IoBinding`/`MatchExpr`/`SelectExpr` perpetually on `on_other`. This
spec collapses that catch-all in 4c. Supersedes:
`bootstrap-retirement-plan.md` §Tick 4 sub-tick decomposition (adds
`on_other` → `on_dark` rename to 4c's scope).

---

## Bootstrap loop

The seed encodes `mirror/glass.mirror` — the **meta-glass**, the full
grammar of `.mirror` files. Not `00-prism.mirror`. The earlier
framing in this document (seed = encoding of `00-prism.mirror`) is
superseded: `00-prism.mirror` is a *consumer* of the meta-glass; it
declares the Prism algebra but does not declare how to parse `.mirror`
files. The meta-glass declares that, including its own form.

The Rust seed is the just-enough-meta-glass to parse `glass.mirror`:
the structural primitives (`Seq`, `Choice`, `Repeat`, `Capture`,
`Literal`, `Charset`, `BraceBlock`, `ParenBlock`, `Until`, `Shift`)
wired into a Combinator literal sized to the bootstrap need. The
operation-specific knowledge (keyword names, kind tags, body forms,
comment syntax) moves out of Rust and into `glass.mirror`. The seed
shrinks from 47 LOC (4b.2's encoding of `00-prism.mirror`) to ~30 LOC
(the encoding of just `glass.mirror`'s parser scaffolding).

A pre-meta-glass version (the 4b.2 seed encoding `00-prism.mirror`):

```rust
fn prism_seed() -> Combinator {
    // The Combinator-tree encoding of boot/00-prism.mirror.
    // Five operation literals + the `prism @(<name>) { … }` body form
    // + the abstract `io tick(type) -> tock(type)` declaration
    // + the `project in(prism)` / `project out(prism)` bindings.
    use Combinator::*;
    Seq(vec![
        Choice(vec![
            literal_kind(b"focus",   AstKind::Focus),
            literal_kind(b"project", AstKind::Project),
            literal_kind(b"split",  AstKind::Split),
            literal_kind(b"shift",  AstKind::Shift),
            literal_kind(b"settle", AstKind::Settle),
        ]),
        // … prism-body form, abstract io form, in/out decls …
    ])
}
```

(Sketch only; the exact tree is the 4b.1 deliverable, derived
mechanically from the 306-byte file.)

The mirror compiler is self-hosting at the parser level when three
fixed points hold:

### FP1 — the meta-glass parses itself

```rust
let seed = prism_seed();                       // ~30 LOC Rust encoding
let glass_bytes = read_file("mirror/glass.mirror");
let meta_glass =
    apply_h(seed, (glass_bytes, 0))
        .into_focus().unwrap();
assert_eq!(combinator_tree_oid(&seed),
           combinator_tree_oid(&meta_glass));
```

**This is the load-bearing equation.** The Rust seed parses the file
that declares it, and the two trees hash byte-identical. Mirror's
grammar describes mirror's grammar; the description is the
implementation. After FP1 holds, the Rust seed is structurally
redundant — we could in principle compile from the `.mirror` file
and discard the Rust copy. We keep the Rust copy as the trusted base
(it's what runs before any `.mirror` file has been parsed), but the
equation guarantees the two stay in lockstep.

### FP2 — the algebra lifts cleanly

```rust
let prism_tree =
    apply_h(meta_glass.clone(), (read_file("mirror/00-prism.mirror"), 0))
        .into_focus().unwrap();
```

`00-prism.mirror` declares the Prism algebra (the five operations, the
identity prism, the abstract io declaration). After the meta-glass
lands, `00-prism.mirror` is a *consumer* of it — the algebra-declaration
file, parsed by the meta-glass like any other `.mirror` file. The
FP2 assertion is well-formedness: the parse succeeds, produces no
Dark spans, and the resulting tree is a well-typed `Combinator`
encoding of the five operations. The OID of `prism_tree` is not
expected to equal the meta-glass's OID — they are different grammars.

### FP3 — every other grammar lifts the same way

```rust
let rust_combinator =
    apply_h(meta_glass.clone(),
            (read_file("mirror/code/rust.mirror"), 0))
        .into_focus().unwrap();
let llvm_combinator =
    apply_h(meta_glass.clone(),
            (read_file("mirror/code/llvm/ir.mirror"), 0))
        .into_focus().unwrap();
// … one per grammar file in mirror/
```

Source files then parse through `apply_h(g_combinator, …)`. Two
`apply_h` calls per source file: first lifts the grammar declaration
to a Combinator tree, second lifts source bytes to an AST.

`combinator_tree_oid` is `compute_content_oid` extended to walk the
`Combinator` enum — a straight Merkle hash over variants. The seed
and `meta_glass` must hash byte-identical (FP1). All other lifts
(FP2, FP3) are well-formedness assertions, not OID equalities.

This is the heart of Tick 4. The Rust seed is ~30 lines because
`glass.mirror` is ~2 KB and structural; everything else is data. The
232 lines of `grammar.rs` retire because their work is
`apply_h(meta_glass, …)`. The 768 lines of `tokenize.rs` retire
because their work is `apply_h(g_combinator, …)`.

---

## Keyword↔kind tables as data

The mirror grammar's body, today, names five mappings:

```mirror
grammar @mirror/grammar("mirror", "spec", "shatter") {
  focus grammar
  split type
  project in
  project out
  zoom abstract
}
```

Each `<op> <keyword>` line becomes one branch of the grammar's
top-level `choice`. The combinator tree for `@mirror/grammar` has the
shape:

```
choice([
  capture(seq([literal(b"grammar"), name, optional(brace_block(body))]),
          AstKind::Focus),
  capture(seq([literal(b"type"), name]),
          AstKind::Split),
  capture(seq([literal(b"in"), name]),
          AstKind::Project),
  capture(seq([literal(b"out"), name]),
          AstKind::Project),
  capture(seq([literal(b"abstract"), name, brace_block(body)]),
          AstKind::Zoom),
  io_binding,            # the named composition for the io form
  match_arm,             # the named composition for match
  select_variant,        # the named composition for select
  dark_fallback,         # the strict-classification sentinel
])
```

The seed builds this tree by reading the `<op> <keyword>` lines and
emitting one `capture(seq([literal(keyword), …]), op_kind)` branch
per line. The `Focus`-kind branches are special: they may carry a
brace-block body and recurse. The seed knows this because `Focus` and
`Settle` are the two operation kinds that traditionally have nested
children (per `ast-as-bundle.md`'s trait-chain mapping: Fiber and
Closure both carry inner structure).

`Focus`-kind attachment to the `focus` literal happens at the `capture`
level: the second argument to `capture` is the `AstKind::Focus`
constant. The seed reads `focus grammar` from the grammar file and
emits exactly that capture branch.

---

## Reverse-lookup collisions

`code/rust.mirror` has multiple keywords per kind (`split` ↔
`struct`/`enum`, `focus` ↔ `impl`/`mod`). Forward parsing is
unambiguous — each keyword's `literal` branch knows its kind. The
collision is reverse-direction: when `Fold5At<render>` emits a Split
node, which keyword does it write?

Resolution: **carry the keyword on the captured node**. Every
`capture(seq([literal(kw), …]), kind)` records `node.keyword = kw`.
The renderer reads `node.keyword` directly; no reverse lookup needed.

Synthesised nodes (only produced inside the kintsugi loop, materialising
candidate au values without going through tokenize) need a default:
first-keyword-for-kind walked from the rendering grammar's combinator
tree. Same shape as today's `keyword_for_kind`, but a tree-walk over
combinator branches instead of a `Vec<Mapping>` lookup.

The renderer stays grammar-aware via `node.grammar_tag`. The collision
is only a default-resolution question; tokenized nodes carry their own
keyword.

---

## Dark spans

`dark_fallback` sits at the bottom of every top-level `choice`. When
every other arm declines to advance, it scans forward through the
unknown construct — a word, optional whitespace, optional balanced
brace block — and emits `AstNode { kind: Dark, body: Some(verbatim),
dark_span: { start, end } }`.

Three byte-for-byte equivalences with today's tokenizer:

1. **Span covers keyword + braces** (Seam T1.1 fix): `fn { foo }`,
   `gn { foo }`, `fn { fox }` produce three distinct OIDs because
   the leading keyword and enclosing `{ }` are inside the hashed
   region.
2. **`\` bodies are not Dark.** The `body_is_obligation` check
   becomes a sibling combinator branch *above* `dark_fallback` in
   the choice list — obligation bodies match the kintsugi-marker
   shape before unknowns fall through.
3. **Bare unknown chars advance one byte without producing a Dark
   node.** Only `word + maybe-brace-block` triggers Dark emission.

**Strict-failure contract.** Per
`strict-and-total-classification.md`: any Dark in the final AST →
`--strict` exits 2 with the per-region diagnostic format. Preserved
because `dark_fallback` produces exactly one `AstKind::Dark` per
unknown construct with identical span boundaries. The boot corpus's
Dark count (58 across 23 files) is byte-stable across Tick 4; if it
moves, the tick is rolled back.

---

## Kintsugi fixed-point property (folded from Tick 2)

The original Tick 2 in the retirement plan scaffolded the kintsugi
formatter loop as `IdentityPrism` composition. With the parser-as-Prism
surface in place, that scaffold *is* the round-trip property of the
combinator algebra. Tick 2 dissolves into Tick 4.

**The kintsugi fixed-point property.** Let `R` be the `Fold5At<render>`
catamorphism and `T` be the parser-as-Prism (the `Combinator` tree for
a given grammar). For every `.mirror` file in `boot/` that parses
without producing Dark nodes:

```
R(T(R(T(bytes)))) == R(T(bytes))    # bytes-level fixed point
T(R(ast)) == ast                    # AST-level fixed point
```

The loop, written as an algebra element:

```rust
let kintsugi: Box<dyn Prism<…>> =
    IdentityPrism::compose(R, T)
        .iterate_to_fixpoint();
```

Where `iterate_to_fixpoint` applies `R ∘ T` repeatedly until the
output OID stops changing. By the Banach contraction-map argument
from `kintsugi-formatter.md` (the Magnot 2025 cycle-averaged-holonomy
inequality), for finite obligation sets over a sub-Turing grammar,
this iteration terminates in finite time. For the boot corpus today,
every file's first iteration is already a fixed point: the round-trip
property holds at depth 1.

**Acceptance for the kintsugi side of Tick 4.** Every file in `boot/`
satisfies the fixed-point property at depth 1 after Tick 4 lands —
i.e., one round-trip is byte-identical to the canonical form. The 109
boot files at depth 1 settle to themselves. Files that don't settle
at depth 1 must reach a fixed point by depth N for some finite N, and
N is recorded per file (today, every file has N = 1; if Tick 4
introduces any depth-> 1 file, the scaffold is wrong).

The Tick 2 scaffold's specific deliverable (the no-op
five-stage `kintsugi_tick` line) becomes one line in the property
test: `assert_eq!(round_trip(file), file)` for every file in the
corpus, run once per `mirror kintsugi --shatter N` invocation.

---

## The migration command

Tick 4b.3 lands the meta-glass AND moves the file tree at the same
moment. One command does the migration:

```
mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/
```

Three things happen in one settle:

1. **Symbol rewrite.** The mq-query `'grammar => glass'` is a
   structural rewrite. Anywhere the meta-glass parses the symbol
   `grammar` (as a keyword, identifier, or path component inside
   `@<ref>`), the symbol becomes `glass`. English prose in `@nl`
   comments containing the word "grammar" stays unchanged because
   it parses through `@nl`, not through `@mirror/glass`. This is
   what makes the rewrite safe — the meta-glass tells kintsugi
   which tokens are structural vs prose.
2. **File renames.** Files whose basename matches the rewritten
   symbol are renamed in lockstep: `grammar.mirror` becomes
   `glass.mirror`. The basename is the symbol the file declares.
3. **Path canonicalisation.** `--out=mirror/` settles paths to
   their canonical location under `mirror/`. Bootstrap-historical
   prefixes (`std/mirror/`) drop because they have no semantic
   content. Real namespace prefixes (`code/rust/`, `code/llvm/ir/`)
   preserve. The resulting tree is the canonical mirror standard
   library.

The mapping:

| From | To |
|---|---|
| `boot/00-prism.mirror` | `mirror/00-prism.mirror` |
| `boot/std/mirror/grammar.mirror` | `mirror/glass.mirror` |
| `boot/std/mirror/compile.mirror` | `mirror/compile.mirror` |
| `boot/std/mirror/runtime/gen_prism.mirror` | `mirror/runtime/gen_prism.mirror` |
| `boot/std/code/rust.mirror` | `mirror/code/rust.mirror` |
| `boot/std/code/llvm/ir.mirror` | `mirror/code/llvm/ir.mirror` |

Cross-file references update inside files: `in @mirror/grammar`
becomes `in @mirror/glass`. The rewrite sees the path component
structurally, not as text.

**The chicken-and-egg.** The Rust seed loads the meta-glass at a
path. Before the migration runs, that path is
`boot/std/mirror/grammar.mirror`; after, `mirror/glass.mirror`. The
same commit that lands the migration updates the path constant in
`spectral.rs` (or moves it to a build-arg, which is the cleaner
long-term shape). One commit, one settle.

The transform mode itself is a Tick-5-adjacent feature folded
forward to Tick 4b.3 because the meta-glass is the moment it
becomes safe: without the meta-glass, the rewrite can't tell which
`grammar` tokens are keywords vs identifiers vs path components.
With the meta-glass, the rewrite is structural by construction.

---

## Cross-grammar lifts

The `@<grammar>(<body>)` form lifts a body of bytes into another
grammar's parser. This is the bridge that lets host grammars
(`@mirror/glass`, `@code/rust`, `@code/llvm/ir`) defer to other
grammars for embedded content. Three concrete uses:

- **Comments shift to `@nl`.** `# operator = @nl(until_newline)` in
  `mirror/glass.mirror`. Every `#`-prefixed line shifts the
  rest-of-line into the natural-language grammar. Multi-line
  comments are just chains of single-line shifts.
- **Inline backticks shift back to the host.** `` `code` `` inside
  `@nl` shifts into whichever host grammar invoked it (`@mirror/glass`
  when called from a `.mirror` file's comment, `@code/rust` when
  called from a `.rs` file's comment).
- **Fenced code blocks dispatch by language tag.** ` ```rust ` lifts
  to `@code/rust`; ` ```mirror ` shifts to `@mirror/glass`; no tag
  shifts to `@<host>` (the calling grammar).
- **Doctest prompts** (indented `mirror> `) shift the prompt body to
  `@mirror/glass`; following non-prompt lines are expected output.

The Combinator variant:

```rust
Combinator::Shift {
    grammar: Reference,        // the @<path> reference
    body: Box<Combinator>,     // body-extractor (Until, Capture, etc.)
}
```

The walker resolves `grammar` to its loaded Combinator tree at parse
time, extracts the body bytes via `body`, and applies the target
grammar to those bytes. Recursion bottoms out because every shift has
a bounded `until` terminator.

---

## `@nl` — the bare cross-grammar shift target

`@nl` is the natural-language grammar. The **bare-`@nl`** form that
lands in 4b.3 recognises three structural elements inside
otherwise-opaque prose:

1. **Inline backticks** shift back to the host grammar:
   `code_inline = literal("\`") > @<host>(until "\`") > literal("\`")`.
2. **Fenced code blocks** with a language tag dispatch by tag:
   ` ```rust ` shifts to `@code/rust`, ` ```mirror ` to `@mirror/glass`,
   no tag shifts to `@<host>`.
3. **Doctest prompts** (indented `mirror> `) shift the prompt body to
   `@<host>`; following non-prompt lines are expected output.

`mirror/nl.mirror` is ~500 bytes, declares these three forms, and uses
the same Combinator primitives the meta-glass already needs (`Choice`,
`Repeat`, `Literal`, `Charset`, `Shift`, `Until`).

**Deferred to a follow-up tick (4b.3.5 or later).**

- The full `@nl/markdown` grammar with CommonMark structure
  (headings, lists, paragraphs, links). That tick introduces the
  `@data/*` namespace (`@data/markdown`, `@data/json`, etc.) and the
  `<` subtyping operator. See `docs/specs/grammar-inheritance.md`
  (to be written).
- Doctest *execution*. 4b.3 lands the doctest grammar (the `mirror>`
  prompt parses through the host). A separate tick lands the harness
  that runs them and compares against expected output.

---

## Sub-tick decomposition

The retirement plan names sub-ticks 4a, 4b, 4c. This spec is 4a.

### 4b — combinator implementations, sub-tick by sub-tick

Land incrementally. Smoke check between stages: smoke OIDs
`a8312da6…` and `3ba4c79d…` byte-stable; boot crystal `41470e69f2…`
stable; Dark count 58/23; full-corpus round-trip at depth 1.

- **4b.1 — `Combinator` enum + `prism_seed()` (partial FP1).**
  [LANDED commit `8b2c490` on `reed/v1-floor`.] Hand-coded
  `prism_seed()` as the encoding of `00-prism.mirror`'s
  five-operation keyword choice (~8 LOC). FP1 holds over the
  keyword subset; FP2 holds against the keyword Choice of
  `grammar.mirror`. Variants `Seq`/`Choice`/`LiteralKind` real; the
  rest `unimplemented!()`.
- **4b.2 — heterogeneous `apply_h` + full seed.** [LANDED commits
  prism `7b78778`, mirror `4366c25`.] Root fix to prism-core:
  `apply_h` takes independent `SIn`/`Out` type parameters. Bypass
  shims (`apply_h_content`, 4b.1's `parse_with` helper) deleted.
  `prism_seed()` grew to 47 LOC encoding the full `00-prism.mirror`.
  FP1 holds over the full file. Variants `Seq`/`Choice`/`Capture`/
  `BraceBlock`/`Literal`/`LiteralKind`/`IoBinding` real.
- **4b.3 — the meta-glass + bare `@nl` + migration to `mirror/`.**
  [THIS TICK.] Five parts, one commit-graph:
  1. Grow `boot/std/mirror/grammar.mirror` into the full meta-glass
     form. The file declares every syntactic form mirror knows about
     (block structures, operation declarations, references, the `#`
     comment shift to `@nl`, the `<ast>`/backtick shifts, the doctest
     prompt) using `settle` declarations.
  2. Implement the Combinator variants the meta-glass references:
     `Repeat`, `Charset` (with the `CharsetKind` enum filled in),
     `ParenBlock`, `Until` (or extend `Capture` with a stop
     condition), and `Shift` (the cross-grammar shift
     `@<grammar>(<body>)`).
  3. Shrink `prism_seed()` to ~30 LOC encoding just-enough-meta-glass
     to parse `glass.mirror`. The seed is purely structural; no
     operation-specific knowledge in Rust.
  4. Write `mirror/nl.mirror` (~500 bytes) declaring the bare-`@nl`
     grammar (inline backticks, fenced code blocks with language
     tags, indented `mirror>` doctest prompts).
  5. Extend mq-query with the `=>` rewrite operator; extend kintsugi
     with `--transform <mq-query>` and `--out <path>` flags. Run the
     migration:
     `mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/`.
     Update the `spectral.rs` path constant in the same commit.
  6. **FP1 promoted.** Assert
     `apply_h(meta_glass_seed, glass.mirror.bytes) == meta_glass`
     (OID-equal). The old FP1 (seed parses `00-prism.mirror` to
     itself) is replaced by this stronger statement at the
     meta-glass level. The new FP2 asserts well-formedness of
     `00-prism.mirror`'s parse.
  7. Smoke check: every `.mirror` file in `mirror/` parses with
     `apply_h(meta_glass, file.bytes)`; resulting ASTs match
     today's `tokenize.rs` output. Butterfly still produces a
     working `./mirror-self`.
- **4b.3.5 — `@data/markdown` + `@nl/markdown` + grammar inheritance.**
  Introduces the `@data/*` namespace, the `<` subtyping operator,
  and `mirror/data/markdown.mirror` + `mirror/nl/markdown.mirror`.
  New spec: `docs/specs/grammar-inheritance.md`. Out of scope for
  4b.3; explicit follow-up.
- **4b.4 — `@code/rust`.** Seven keywords with reverse-lookup
  collisions. `node.keyword` carries the original; no reverse table
  needed. Smoke: tokenize `bootstrap/src/*.rs`, compare ASTs and
  rendered bytes.
- **4b.5 — `@code/llvm/ir`.** The `keyword_form_body` special case
  exercised. Smoke: tokenize `bootstrap/mirror.ll`; butterfly
  pipeline produces a working `./mirror-butterfly`.
- **4b.6 — remaining grammars.** `@epistemologic/property/*`,
  `@epistemologic/math/*`, `@mirror/kintsugi`, `@hash/coincidence`.
  All share the meta-glass shape; smoke-clean once 4b.3 lands.

"Smoke clean" per grammar: same files → same OIDs, same Dark count,
same AST shape. Any divergence surfaces a missing combinator and
becomes a vocabulary extension to this spec.

### 4c — `grammar.rs` and `on_other` retirement

After 4b lands all grammars on the combinator surface:

1. **Retire `parse_grammar`** — its `Vec<Mapping>` output is now the
   combinator tree from `apply_h(seed, …)`.
2. **Retire `load_grammar` and `grammar_for_file`** — collapse to
   `read_file` + `apply_h(seed, bytes)` plus a one-line extension
   switch.
3. **Retire `is_skip_word`** — becomes a `choice` of empty-settle
   literals in `code/rust.mirror`'s combinator tree.
4. **Retire `keyword_for_kind`** — default-lookup becomes a tree-walk
   over `capture` branches; O(branches) is fine for renderer usage.
5. **Rename `Fold5::on_other` → `Fold5::on_dark`** — one-arm match on
   `AstKind::Dark`; `In`/`Out` dispatch through dedicated reducers
   (or fold into the `seq` walker — TBD).
6. **Smoke check.** Full corpus: OIDs stable, crystal stable, Dark
   count stable, `mirror craft --target binary boot` produces a
   working `./mirror-self`.

---

## Acceptance criteria

Tick 4 lands when, against `reed/v1-floor`:

1. **Smoke OIDs byte-stable.** Both `a8312da6…` and `3ba4c79d…`.
2. **Boot crystal byte-stable.** `41470e69f2…`.
3. **Dark count byte-stable.** 58 across 23 files; 0 across clean
   boot files. `mirror craft --strict boot` exits 2 with the
   identical per-file diagnostic format.
4. **Round-trip property at depth 1** on all 109 boot files:
   `R(T(bytes)) == bytes` byte-equal (modulo canonical
   whitespace, which is already canonical in the corpus).
5. **`mirror craft --target binary boot`** produces a working
   `./mirror-self` whose own `mirror compile <self.ll>` reproduces
   its crystal OID. The butterfly closes.
6. **`tokenize.rs` deleted, `grammar.rs` deleted.** Both files gone
   from `bootstrap/src/`; the modules they implemented now live as
   combinator data inside `bootstrap/src/spectral.rs`.
7. **`Fold5::on_other` renamed to `on_dark`** and reduces to a
   one-arm match on `AstKind::Dark`. The bundle terminals
   (`In`/`Out`) and the Spec A/B extensions
   (`IoBinding`/`MatchExpr`/`SelectExpr`) no longer route through the
   catch-all.
8. **Meta-glass self-hosting fixed point** (FP1):
   `apply_h(prism_seed(), mirror/glass.mirror.bytes)` produces a
   `Combinator` tree (`meta_glass`) whose OID equals the Rust seed's
   OID. The meta-grammar parses the file that declares it. The
   mirror compiler is self-hosting at the parser level.
9. **Algebra lifts cleanly** (FP2):
   `apply_h(meta_glass, mirror/00-prism.mirror.bytes)` produces a
   well-typed Combinator tree with no Dark spans. The algebra
   declaration parses cleanly through the meta-glass.
10. **Every other grammar lifts the same way** (FP3): for each
    `mirror/**/*.mirror`, `apply_h(meta_glass, file.bytes)`
    produces a Combinator tree with no Dark spans (or only the Dark
    spans the file genuinely declares).
11. **Migration complete.** `boot/` is gone from the source tree;
    every file is under `mirror/`. The symbol `grammar` no longer
    appears as a keyword or path component anywhere in the corpus
    (only as English in `@nl` comments).
12. **Bare `@nl` grammar exists** at `mirror/nl.mirror`. The three
    structural forms (inline backticks, fenced code blocks with
    language tags, indented `mirror>` doctest prompts) parse
    correctly inside any host grammar's comments.

---

## Deviations from the original Tick 4 plan

The retirement plan §Tick 4 named the combinator vocabulary as: `seq`,
`choice`, `repeat`, `capture`, `literal`, `charset`, `brace_block`,
`paren_block`, `io_binding`, `match_arm`, `select_variant`, plus a
"specialised combinator for the LLVM-IR keyword-form body capture".
This spec keeps the vocabulary, names the LLVM-IR specialised
combinator as `keyword_form_body`, and adds two named primitives the
plan did not enumerate:

- **`dark_fallback`** — the explicit fallback combinator. The plan
  said "domain rejection (unrecognized bytes) produces
  `Partial(dark_node, ScalarLoss::new(span))` — exactly the Dark
  semantics today". This spec names that rejection a *combinator*
  (`dark_fallback`) so the strict-classification contract is one
  combinator's responsibility, not a per-choice property. Supersedes:
  `bootstrap-retirement-plan.md` §"`tokenize.rs` — RETIRE" paragraph
  on Dark semantics.
- **`Combinator` as closed enum** — the plan's open question on
  trait-vs-enum is resolved as enum. The seed-as-`00-prism.mirror`
  recognition forces the decision: combinators are data the seed
  produces from bytes, and data wants an enum (serializable; one
  Merkle hash over variants; no vtable pointers). Closed-surface
  honesty, binary-size pressure (≤240KB target), and a single
  `impl Prism for Combinator` dispatch site also point this way.
  See §"Trait or enum?" above for the full tradeoff table.
  Supersedes: `bootstrap-retirement-plan.md` §Tick 4 "Open question"
  paragraph on trait-vs-enum.
- **Seed shape — the Combinator-tree encoding of `boot/00-prism.mirror`.**
  The plan said "~80 lines of hand-coded Rust". This spec recognises
  that `00-prism.mirror` (306 bytes) IS the algebra A written as a
  `.mirror` file; the seed is the Rust encoding of that file's
  combinator tree (~25 lines, monomorphic). The fixed-point chain
  gains a third equation (FP1 above): the seed parses the file that
  declares it. Supersedes: `bootstrap-retirement-plan.md` §Tick 4
  paragraph on the seed sizing.

This spec also folds Tick 2 (kintsugi-loop scaffold) into Tick 4. The
plan kept Tick 2 as a standalone tick; Alex's directive (in this
session's brief) was to fold it. The fold-in lands as the §"Kintsugi
fixed-point property" above; Tick 2 dissolves into the round-trip
property the combinator surface establishes for free. Supersedes:
`bootstrap-retirement-plan.md` §"Tick 2 — retire the kintsugi-loop
scaffold in `cmd_kintsugi`".

---

## Decisions confirmed (2026-05-21)

- **`Combinator` is a closed enum.** Resolved per the trade-off
  table in §"Trait or enum?". Combinators are data the seed
  produces from bytes.
- **`Dark` survives as a marker `AstKind`.** Strict classification's
  byte-stable Dark count is load-bearing; naming the failure mode
  beats hiding it. `Fold5::on_other` collapses to `on_dark` (one arm,
  one kind) per 4c.
- **The seed lives in `spectral.rs`.** ~30 lines of monomorphic Rust
  after 4b.3 (the encoding of just-enough-meta-glass to parse
  `glass.mirror`). No separate `seed.rs`. Revisit only if the
  `no_std` stretch's `bootstrap-io` split needs seed isolation.
- **Seed encodes the meta-glass, not `00-prism.mirror`.** [Reshape
  2026-05-21 PM.] The earlier framing (seed = encoding of
  `00-prism.mirror`) was a 4b.2 intermediate. The meta-glass IS the
  full grammar of `.mirror` files; FP1 is now
  `apply_h(seed, glass.mirror.bytes) == meta_glass`. `00-prism.mirror`
  is a consumer of the meta-glass (algebra declaration), not a
  seed-target.
- **Cross-grammar shift `@<grammar>(<body>)` is a Combinator primitive.**
  Required for `# = @nl(until_newline)` and every other host-defers-
  to-grammar form. Added in 4b.3 as `Combinator::Shift`.
- **The boot/ → mirror/ migration and the `grammar → glass` rename
  are one kintsugi pass.** `mirror kintsugi boot/
  --transform='grammar => glass' --out=mirror/`. Lands in 4b.3 with
  the meta-glass.
- **`mq-query` gets the `=>` rewrite operator in 4b.3.** Originally
  scheduled for Tick 5 alongside the full `mq.mirror` grammar; the
  meta-glass forces the operator forward because the migration
  needs it. The full `mq.mirror` grammar declaration stays in
  Tick 5.
- **`<` is genuine subtyping, not just import.** Deferred to 4b.3.5
  with `docs/specs/grammar-inheritance.md`. Out of scope for 4b.3.

## Open questions

1. **`@trace/*` grammar interaction.** The `@trace` grammars
   (referenced from `code/rust.mirror`'s `in @trace`) may not load
   under the combinator surface in their current form — the
   `@trace/*` namespace mostly carries comments and type
   declarations, not body forms. If any `@trace/*` grammar uses a
   body form not covered by the named combinators (`io_binding`,
   `match_arm`, `select_variant`, `keyword_form_body`), the
   vocabulary needs an extension. Recommended: enumerate the bodies
   in `@trace/*` during 4b.4 and either add a named combinator or
   declare the grammar tokenization-only (no executable body forms).
   Alex's call if any `@trace/*` body needs a new combinator.

2. **The exact `prism_seed()` tree.** The Combinator-tree encoding
   of `boot/00-prism.mirror` is mechanical but not yet written.
   4b.1's first deliverable is to produce it and verify FP1 hashes
   match. If the encoding requires more than ~30 lines of Rust, the
   `00-prism.mirror` file needs simplification first. The 306-byte
   file is currently dense enough that ~25 lines should suffice;
   confirmation belongs to 4b.1.

---

*A grammar is a Prism written as data.*
*A parser is a Prism applied to bytes.*
*The seed is the trusted base; the fixed point is the proof.*
*`tokenize ∘ render = id` on parsing-correct input.*
*`render ∘ tokenize = id` on canonically-formatted input.*
*The kintsugi loop iterates to the intersection.*
*The combinator surface was always there; today we name it.*

Apache-2.0.
