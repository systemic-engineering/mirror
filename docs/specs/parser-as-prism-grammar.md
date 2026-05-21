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
  parser for mirror's own grammar is the mirror-grammar combinator tree
  loaded from `boot/std/mirror/grammar.mirror`.

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

**Decision: trait.** A `Combinator: Prism` supertrait, one impl per
combinator kind. The surface is closed by spec rather than by enum
variant; closure is enforced socially (this spec is the registry) and
algebraically (`Combinator: Prism` inherits the closed `Optic`
supertrait chain from `spectral-triple-grammar.md` Phase 1). Three
load-bearing reasons:

1. **`compose_a` already works on `Prism` impls.** An enum would
   require a fat `apply_h` arm re-implementing composition per
   variant. The trait form composes uniformly through the existing
   `prism_apply` primitive.
2. **The `on_other` resolution wants per-impl dispatch.** Each
   combinator that produces an `AstKind` advertises it via its own
   `Refracted` type. An enum re-introduces the catch-all match the
   `on_other` reducer was the symptom of.
3. **Algebra closure already covers enum honesty.** The named
   vocabulary below IS the surface; new combinators require a spec
   extension AND a new hand-coded impl. Closure is structural, not
   syntactic.

If `cargo build --release` grows >10% in 4b.1 due to monomorphisation,
fall back to the enum form (mechanical wrap: `CombinatorVariant::Foo(Foo)`
arms + one match in `apply_h`). Surface stays closed either way.

The supertrait relationship:

```rust
pub trait Combinator: Prism {
    /// Source bytes consumed (or a slice carrying the remaining tail).
    /// Specialisation of `Prism::Input`'s carried type; this binding is
    /// what keeps the combinator algebra closed over bytes.
    type ByteIn: ?Sized;

    /// The AST fragment this combinator produces. For leaf combinators
    /// (`literal`, `charset`) this may be `()` or a span; for compound
    /// combinators (`seq`, `choice`) it composes upward to `AstNode`.
    type Fragment;

    /// The `AstKind` this combinator's fragment carries when it lifts
    /// into an `AstNode`. Some combinators (`literal`, `charset`,
    /// `repeat`) don't lift directly — they advertise `None`. The
    /// kind is read by `apply_h` for diagnostic and bundle-terminal
    /// placement.
    fn ast_kind(&self) -> Option<AstKind>;
}

impl<C: Combinator> Prism for C { /* via supertrait */ }
```

The `Beam<In>` for any `Combinator` carries `(source: &[u8], offset:
usize)`; the `Beam<Refracted>` carries `Imperfect<AstFragment,
Infallible, ScalarLoss>` where `ScalarLoss` is the Dark-region
accumulator (per `Transport::Holonomy: Metric`). The walker is
`apply_h(combinator, (source, 0))`; the verdict's `Success` payload
is an `AstNode` (after the root `seq` lifts its fragments).

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
  `is_skip_word` table becomes one `choice` arm of empty-refract
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
.lift(AstKind::IoBinding)
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
]).lift(AstKind::MatchExpr)
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
  ]).lift(kind)
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

The mirror compiler is self-hosting at the parser level when this
sequence reaches a fixed point:

```
# Cold start
let seed: Combinator = mirror_grammar_seed();
                       # ⤴ hand-written in `spectral.rs`. ~80 lines.
                       # The trusted base.

let grammar_bytes = read_file("boot/std/mirror/grammar.mirror");
let seed_prime: Combinator =
    apply_h(seed, (grammar_bytes, 0)).into_focus().unwrap();

# Self-hosting fixed point — load-bearing.
assert_eq!(combinator_tree_oid(&seed),
           combinator_tree_oid(&seed_prime));
```

`combinator_tree_oid` is `compute_content_oid` extended to walk a
`Combinator` tree (Merkle-OID over trait objects). The seed and its
data-form must hash identical.

All other grammars then load through the seed:

```
let g = apply_h(seed, (rust_grammar_bytes, 0)).into_focus().unwrap();
let ast = apply_h(g, (rust_source_bytes, 0)).into_focus().unwrap();
```

Two `apply_h` calls — first lifts grammar bytes to a combinator tree,
second lifts source bytes to an AST. No `parse_grammar`; no keyword
table parsed at startup — the keyword↔kind mapping is data in the
combinator tree, deposited there by the first call.

This is the heart of Tick 4. The Rust seed is ~80 lines; everything
else is data. The 232 lines of `grammar.rs` retire because their work
is `apply_h(seed, …)`. The 768 lines of `tokenize.rs` retire because
their work is `apply_h(g_combinator, …)`.

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
`Refract` are the two operation kinds that traditionally have nested
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

## Sub-tick decomposition

The retirement plan names sub-ticks 4a, 4b, 4c. This spec is 4a. Here
is the refined ordering for 4b and 4c.

### 4b — combinator implementations, grammar by grammar

Land in four stages, simplest first. Smoke check between stages:
smoke OIDs `a8312da6…` and `3ba4c79d…` byte-stable; boot crystal
`41470e69f2…` stable; Dark count 58/23; full-corpus round-trip at
depth 1.

- **4b.1 — `@mirror/grammar` (the seed).** Five operation keywords,
  `grammar {}` body form, plus `io_binding`/`match_arm`/`select_variant`
  for `.mirror` bodies. The seed is hand-coded; the grammar file's
  combinator tree is computed via `apply_h(seed, grammar.bytes)`; the
  two must hash equal. Smoke check: tokenize every `.mirror` file in
  `boot/` with the new combinator, compare AST byte-by-byte.
- **4b.2 — `@code/rust`.** Seven keywords with reverse-lookup
  collisions. `node.keyword` carries the original; no reverse table
  needed. Smoke: tokenize `bootstrap/src/*.rs`, compare ASTs and
  rendered bytes.
- **4b.3 — `@code/llvm/ir`.** The `keyword_form_body` special case
  exercised. Smoke: tokenize `bootstrap/mirror.ll`; butterfly
  pipeline produces a working `./mirror-butterfly`.
- **4b.4 — remaining grammars.** `@epistemologic/property/*`,
  `@epistemologic/math/*`, `@mirror/kintsugi`, `@hash/coincidence`.
  All share the mirror-grammar shape; smoke-clean once 4b.1 lands.

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
3. **Retire `is_skip_word`** — becomes a `choice` of empty-refract
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
8. **Combinator-tree fixed point** for the seed:
   `apply_h(seed, mirror.grammar.bytes)` produces a `Combinator`
   tree whose OID equals the seed's OID. The mirror compiler is
   self-hosting at the parser level.

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
- **`Combinator: Prism` supertrait** — the plan said "Each
  combinator is a Prism impl over `Optic<&[u8], (AstNode, &[u8]_remaining)>`".
  This spec promotes the `Combinator` trait to a supertrait with
  associated types (`ByteIn`, `Fragment`, `ast_kind`) so the
  algebra-closure inheritance from `spectral-triple-grammar.md` Phase
  1 flows through. Supersedes:
  `bootstrap-retirement-plan.md` §Tick 4 "Open question" paragraph
  on trait-vs-enum.

This spec also folds Tick 2 (kintsugi-loop scaffold) into Tick 4. The
plan kept Tick 2 as a standalone tick; Alex's directive (in this
session's brief) was to fold it. The fold-in lands as the §"Kintsugi
fixed-point property" above; Tick 2 dissolves into the round-trip
property the combinator surface establishes for free. Supersedes:
`bootstrap-retirement-plan.md` §"Tick 2 — retire the kintsugi-loop
scaffold in `cmd_kintsugi`".

---

## Open questions

1. **Compile-time cost of trait-per-combinator.** Each combinator is
   one impl with its own `apply_h` dispatch. For ~12 combinators that
   compose into ~20 grammar combinator trees, the monomorphisation
   surface is bounded but real. The plan flagged this as "revisit if
   compile times suffer". Recommended: measure during 4b.1
   (mirror-grammar implementation); if `cargo build --release` grows
   by more than 10% relative to today's tokenize.rs, fall back to
   the enum form. The fallback is mechanical: wrap each combinator
   impl in a `CombinatorVariant::Foo(Foo)` arm and dispatch from one
   match in `apply_h`. The surface stays closed either way.

2. **Whether `Dark` survives as a marker or dissolves into `capture` +
   sentinel.** This spec recommends keeping Dark as a separate
   `AstKind`. The alternative is `capture(unknown_span_combinator,
   AstKind::DarkSentinel_via_a_real_op_kind)` — e.g., a Project node
   with a special name. The case for dissolution: one fewer enum
   variant; the Fold5 dispatch becomes pure 5-operation + In/Out.
   The case against: every `--strict` walker would need to detect
   the sentinel name instead of matching on the kind tag. The cost
   of the indirection outweighs the variant. Recommended: keep
   `Dark`. Alex's call if the symmetry argument wins.

3. **`@trace/*` grammar interaction.** The `@trace` grammars
   (referenced from `code/rust.mirror`'s `in @trace`) may not load
   under the combinator surface in their current form — the
   `@trace/*` namespace mostly carries comments and type
   declarations, not body forms. If any `@trace/*` grammar uses a
   body form not covered by the named combinators (io_binding,
   match_arm, select_variant, keyword_form_body), the vocabulary
   needs an extension. Recommended: enumerate the bodies in `@trace/*`
   during 4b.4 and either add a named combinator or declare the
   grammar tokenization-only (no executable body forms). Alex's call
   if any `@trace/*` body needs a new combinator.

4. **Where the seed's hand-coded constructor lives.** Two options:
   (a) inside `spectral.rs` as a `fn mirror_grammar_seed() ->
   Combinator` (~80 lines of literal Rust); (b) inside a separate
   `bootstrap/src/seed.rs` (one file, one job). The plan implies (a).
   Recommended: (a) for Tick 4; revisit during the no_std stretch
   when seed isolation might matter for a separate `bootstrap-io`
   crate.

5. **`pipeline.rs` reuse of the surface.** Tick 5 in the retirement
   plan retires `pipeline.rs` (`split_pipeline`, `is_mq_query`). The
   mq-query parser is "a tiny instance of the parser-as-Prism
   surface". Whether the mq grammar gets its own `.mirror` file
   (`boot/std/mirror/mq.mirror`?) or stays hand-coded as a special
   case is open. Recommended: declare a `.mirror` file for the
   mq-query grammar during Tick 5; it's the test that the combinator
   surface generalises beyond `.mirror` source.

---

*A grammar is a Prism written as data.*
*A parser is a Prism applied to bytes.*
*The seed is the trusted base; the fixed point is the proof.*
*`tokenize ∘ render = id` on parsing-correct input.*
*`render ∘ tokenize = id` on canonically-formatted input.*
*The kintsugi loop iterates to the intersection.*
*The combinator surface was always there; today we name it.*

Apache-2.0.
