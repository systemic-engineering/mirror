# The code generation surface — `@code/X.render` is the gap; `@code/metalogue` is the reception

*2026-06-08. Mara. Spec — substrate-pull, eleventh tick after the eight closed cascade ticks, the property/inference collapse, and the eigensheaf recognition.*

*2026-06-09 reframe (Reed + Alex, the 34th-instance recognition): the universal contract lifts from per-species `@code/X/macro` to `@code/metalogue` — the AST altitude's lift of `@metalogue` (shards/metalogue.mirror, B3). The species-altitude `@code/X/macro` declarations (Rust, Elixir, Lisp, ...) become inheritors via `<= @code/metalogue`; the four shims, four laws, and declaration/turn/metalogue_session carrier types live ONCE at the ground. The species roster keeps the community vocabulary "macro" at the per-language altitude; the substrate gains `@code/metalogue` as the universal noun. The spec's title and section anchors are reframed accordingly; the per-species matrix in §11 retains its rows.*

*2026-06-09 35th-instance recognition (Alex → Mara via Reed): hole-projection IS `project`. During T25 GREEN Mara surfaced that the substrate's `\` could not survive Rust source code at the proc-macro call site; the workaround used `{ }` at the call site and emitted `todo!()` in the macro body. Mara flagged the candidate naming "the lexical glass-wall" for review. Alex's read: **"Yes, it needs an explicit mapping. No language has the exact semantics of the hole. It will always be a `project`ion."** The substrate-pull-correct recognition: `\` IS `hole` (per shards/nl.mirror, boot/std/mirror/interpreter.mirror, the substrate already had the word); `project` IS the operation (one of the five-op operations on `@code/metalogue`); the per-species explicit mapping IS `project_hole`'s body at each `@code/X/macro` altitude. The contract is declared at the ground (§4.3 below); each species realises its lossy projection (§11.4 matrix column).*

> **Status: Yellow (2026-06-08) → Reframed Green (2026-06-09).** The
> recognition Alex named on 2026-06-08 — "what if mirror hooks into
> Rust's macro surface for code generation? `@code/macro` for any
> language that allows direct manipulation of the AST. The gap is the
> spec." — named the **two-way reading of `@code/X`**: the render
> direction (substrate AST → language source) and the macro direction
> (language compile-time AST → consumption of substrate emission). The
> first half already exists at `@code/mirror.render` (T17, the
> Wadler/Bernardy combinator surface). The second half is what this
> spec names. The 2026-06-09 reframe lifts the universal contract to
> `@code/metalogue` — a sibling of `@metalogue` (NL altitude) at the
> AST altitude. Both halves live UNDER each `@code/X` species at the
> binding altitude, AND ABOVE the species at the universal `@code/metalogue`
> ground. The thirty-fourth instance of *substrate-already-had-the-word*;
> this one names the deepest one yet because *macros are the AST
> speaking to itself — Bateson 1972 made literal at compile time*.

---

## 1. The recognition

### 1.0 The 34th-instance reframe (2026-06-09)

The original spec (2026-06-08) named `@code/X.macro` as a per-species
sub-prism with the four-shim contract and four laws declared
per-species. The 34th-instance recognition (Reed + Alex → Mara,
2026-06-09) named the substrate word: `@metalogue` was already
declared at `shards/metalogue.mirror` (B3, task #189) — Bateson
1972's *conversation whose structure reflects its topic* — at the
natural-language altitude. The macro surface IS the same shape lifted
to the AST altitude: *the language speaking ABOUT itself BY USING
itself, at compile time, through its own metaprogramming surface*.

The substrate gains a new shard:
`shards/code/metalogue.mirror` declares `@code/metalogue` — the
AST-altitude metalogue. It inherits from `@metalogue` (NL altitude,
lifted to AST: `turn.body: declaration` instead of `turn.body: nl`)
and from `@code` (the per-altitude grammar family). The four-shim
contract (`shim_type`, `shim_prism`, `shim_action`, `shim_grammar`),
the four laws (round-trip, OID functionality, type-soundness,
substrate-pull preservation), and the carrier types (`declaration`,
`declaration_kind`, `turn`, `metalogue_session`) live HERE at the
ground — universal across every language altitude.

The per-species shards keep their community vocabulary:
`@code/rust/macro`, `@code/elixir/macro`, `@code/lisp/macro`, ...
The community word "macro" is what language users recognize; the
species-altitude file is the per-language binding (`code/rust.ast`
for Rust, `code/elixir.ast` for Elixir, etc.) plus the
`<= @code/metalogue` inheritance declaration. The Rust shard
(`shards/code/rust/macro.mirror`) is the canonical realisation; the
four shim function signatures stay there as the per-altitude binding;
the contract+laws come from the ground for free.

This reframe is non-breaking on the cascade: T23 (type) already
landed; T24 (prism), T25 (action), T26 (grammar) continue per
the original cascade plan, now with the universal contract at the
ground rather than duplicated per-species. The four-law `requires`
clauses that previously lived in each per-species macro shard
collapse to one declaration at `shards/code/metalogue.mirror`.

For cross-references throughout this spec, treat the substrate's
`@code/X/macro` species as the per-language realisations and
`@code/metalogue` as the universal ground. The spec's body below
was written from the per-species perspective; the reframe applies
at the substrate altitude (where the laws actually live) without
requiring a wholesale rewrite of the body. Where this spec says
"declared per-species", read "realised per-species, declared at
`@code/metalogue`". Where it says "the macro surface", read "the
@code/metalogue surface (species: macro)".

---

### 1.1 The original recognition (2026-06-08)

Alex named the recognition verbatim, 2026-06-08, immediately after Taut
#286 Win 2 landed (commit `142e734` — `kintsugi_main` extracted from
the binary entry, `mout!` / `merr!` macros that bypass libtest's
`OUTPUT_CAPTURE` sink via `libc::write` on fd 1 / fd 2):

> *"How would a bi-directional mout macro look like that shims into
> mirror and vice versa? Ooooh wait. I think we just discovered the
> code generation surface. Spawn Mara on that. **What if mirror hooks
> into Rust's macro surface for code generation? `@code/macro` for any
> language that allows direct manipulation of the AST. The gap is the
> spec.**"*

The recognition has four layers, all already present in the substrate
under different names:

1. **`mout!` as bidirectional channel.** The current
   `bootstrap/src/lib.rs:178-200` macro pair (`mout!` / `merr!`)
   writes formatted bytes to fd 1 / fd 2 via `libc::write`, bypassing
   the `Stdout::write` sink-check that `OUTPUT_CAPTURE` installs. They
   are **the language-side reception mechanism for substrate-routable
   I/O** — Rust's macro surface receiving a typed channel that the
   substrate (eventually `@mirror/io`) declares. The "bi-directional"
   reading: Rust→mirror is already in place (the macros expand to
   substrate-routable bytes); mirror→Rust is the symmetric direction
   the substrate hasn't named yet (typed-channel consumption from the
   substrate INTO Rust scope at macro-expansion time).

2. **Code generation surface.** Rust's macros are compile-time AST
   manipulators. If the substrate DECLARES the shape (the typed
   action at the `.mirror` altitude) and the language's macro layer
   GENERATES the realisation (the proc-macro emits the function body
   at compile time), the substrate-pull work that Mara has been doing
   by hand for 32 instances becomes **mechanical**. The discriminator
   `@mirror/realisation.classify` (T21, `shards/mirror/realisation.mirror`)
   already names the per-file lookup: WHICH altitude subsumes this
   Rust file? The macro surface names the inverse: GIVEN that
   altitude, generate the Rust file from the substrate declaration.

3. **`@code/macro` for any language that allows direct AST
   manipulation.** Rust (`macro_rules!` + proc-macros + `#[derive(...)]`),
   Lisp / Scheme / Clojure (the canonical homoiconic case),
   Elixir (`quote` / `unquote` / `defmacro`), Julia (`Expr` /
   `@macroexpand`), Crystal (`macro` blocks), Nim (`macro` /
   `template`). Each language's metaprogramming surface IS a
   typed-AST reception layer; the substrate's task is to emit
   THROUGH it.

4. **"The gap is the spec."** The current gap between a mirror
   substrate declaration (e.g. `compiles(target) -> verdict { \ }` at
   `shards/code/rust.mirror`) and its concrete Rust realisation
   (`fragmentation-mcp`'s actual `pub fn compiles_target(...) -> Verdict {
   ... }` body) IS the specification for what the proc-macro should
   emit. Close the gap → the substrate emits the Rust through the
   language's own macro layer. **The hole's shape IS the codegen
   contract.**

---

## 2. The substrate-pull check — why this is NOT a new family

Before declaring anything new, the substrate's existing surface gets
walked. Each candidate placement is checked against what `shards/` and
`boot/std/` already say.

### 2.1 `@code` is already the universal grammar-at-altitude discipline

Per `shards/code.mirror` (the family root, lifted from boot
2026-06-06):

```
prism @code {
  focus code
  project code
  split code
  shift code
  settle code
}
```

The shard's prose: *"Each `@code/X` instance specializes the five
operations at its altitude; `@code` itself declares only the universal
five-op shape — the discipline, not the implementation."*

The species roster today: `@code/mirror`, `@code/rust`, `@code/gleam`
(canonical landed shards); `@code/llvm`, `@code/fortran` (planned at
phase D); future `@code/elixir`, `@code/julia`, `@code/lisp`,
`@code/python`, `@code/typescript`, `@code/go` (per
`docs/specs/code-extension-grammar.md`'s opening).

### 2.2 `@code/mirror.render` already declares the render surface

Per `shards/code/mirror.mirror:153-258` (T17, 2026-06-08):

```
prism @code/mirror/render {
  focus render
  project render
  split render
  shift render
  settle render
}

type doc = nil | text_doc(text) | line_doc | nest_doc(u32, doc)
         | beside_doc(doc, doc) | above_doc(doc, doc)
         | group_doc(doc) | flatten_doc(doc)

render(ast) -> text { \ }
text(s: text) -> doc { \ }
line() -> doc { \ }
nest(n: u32, d: doc) -> doc { \ }
beside(left: doc, right: doc) -> doc { \ }
beside_space(left: doc, right: doc) -> doc { \ }
above(top: doc, bottom: doc) -> doc { \ }
group(d: doc) -> doc { \ }
flatten(d: doc) -> doc { \ }

requires round_trip(render)
```

The Wadler 2003 / Bernardy 2017 combinator surface for AST → text.
This IS the code generation surface at the mirror altitude. The
shard's prose names the analogous sibling explicitly:

> *"Future `@code/rust.render`, `@code/gleam.render`, `@code/llvm.render`
> join this as siblings — each altitude's render specialises the doc
> carrier to its own AST shape, all sharing the combinator surface
> declared below."*

The substrate has been naming `render` as the universal code
generation primitive at the `@code/X` altitude since T17 landed
yesterday. Alex's `@code/macro` recognition this morning is **the
discovery that `render` already names HALF of what was missing** —
the substrate-side projection. What's missing is the **other half**:
the language-side reception that catches the rendered output and
threads it into the language's own AST.

### 2.3 `boot/std/code/mq.mirror` declares `template render` as the universal "one operation, many uses" primitive

Per `boot/std/code/mq.mirror:96`:

```
template parse(input: text) -> query
template compile(query, context) -> result
```

And the substrate-pull recognition logged as the 27th instance in
`shards/mirror/realisation.mirror:130` ("template render (one
operation, many uses) → @code/mirror"). The substrate's `template`
keyword IS the language-altitude name for the typed projection
between an AST/literal carrier and its wire surface. `render` is the
universal verb.

### 2.4 `@mirror/data/X.emit` is the data-altitude sibling

Per `shards/mirror/data.mirror` (T16, 2026-06-07) and the species
under it (`shards/mirror/data/{json,yaml,toml,text}.mirror`): each
data-format species declares `parse(text) -> carrier` and
`emit(carrier) -> text`. Same Wadler/Bernardy algebra shared with
`@code/mirror.render`; different carrier kind (data values vs source
ASTs). The two halves of grammar-as-lens at the mirror altitude
(per `shards/mirror/data.mirror` family-root prose).

### 2.5 `fragmentation-as-generated.md` already names the codegen pipeline

Per `docs/specs/fragmentation-as-generated.md` §2 (2026-05-24, Mara):

```
┌────────────────────────────────────────────────────────────────┐
│  @fragmentation.mirror   (grammar — declares the shape)        │
│     │  mirror/glass parser (FP1)                               │
│     ▼                                                          │
│  Typed AST                                                     │
│     │  @code/rust.translate(p: @fragmentation, c: @code/rust)  │
│     ▼                                                          │
│  Rust AST                                                      │
│     │  @code/rust.render(g: @code/rust, ast(g) → io_list)      │
│     ▼                                                          │
│  Rust source bytes                                             │
│     │  cargo build                                             │
│     ▼                                                          │
│  fragmentation crate (cargo-consumable)                        │
└────────────────────────────────────────────────────────────────┘
```

`@code/rust.translate` (cross-grammar AST translation) plus
`@code/rust.render` (Rust AST → bytes) IS the substrate's already-
declared code generation surface. The Phase 4b commitment per
`roadmap/pending/phase-4-emitter-self.md` task 1 is: complete the
`@code/rust` translate template.

### 2.6 `@mirror/realisation` names the per-file discriminator

Per `shards/mirror/realisation.mirror` (T21, 2026-06-08): every Rust
file in `bootstrap/src/` classifies as one of `boundary` (@io) or
`substrate` (substrate-realisable; target is named). The
discriminator IS the lookup that, given a generated Rust file, can
ask: which substrate altitude generated this? And inverse: given a
substrate altitude, which Rust files does it cover?

### 2.7 `code-extension-grammar.md` declares the routing

Per `docs/specs/code-extension-grammar.md` (2026-05-19, Reed + Alex):
`@code/X(extensions)` — the file extension is the grammar's parameter.
No routing table; the grammars ARE the routing. Bootstrap scans
`boot/std/code/`; each grammar declares its extension via `@code(...)`;
`.rs` → `@code/rust`, `.ex/.exs` → `@code/elixir`, etc.

### 2.8 The collapse

The substrate's existing answer to "code generation surface":

| Layer | Substrate name | Status |
|---|---|---|
| Universal discipline | `@code` family | **Landed** (`shards/code.mirror`) |
| Per-language altitude | `@code/X` species | **Landed** (rust, mirror, gleam; llvm/fortran pending) |
| File-extension routing | `@code/X(extensions)` parameter | **Landed** (per `code-extension-grammar.md`) |
| AST → text projection | `@code/X.render` | **Half-landed** (`@code/mirror.render` complete; `@code/rust.render` declared but body abstract) |
| Cross-grammar translation | `@code/X.translate(source, target)` | **Declared** (`boot/04-code.mirror`; bodies pending) |
| Per-file altitude classification | `@mirror/realisation.classify` | **Landed** |
| Build-target identity | `@code/X/cargo` (Rust) / `@io.cargo` (boundary) | **Landed** |
| Round-trip identity law | `requires round_trip(render)` | **Declared** on `@code/mirror.render` |

What `@code/macro` would add, IF it were a new family:
- A surface to RECEIVE rendered code into the host language's metaprogramming layer.
- A typed contract between substrate emission and language-side compile-time AST consumption.

What the substrate already has, looked at differently:
- `@code/X.render` IS the emission surface.
- The host language's macro surface IS the reception surface — and **it's already part of `@code/X`'s altitude**, the same way `@code/rust/cargo` (the build-target sub-prism) is part of `@code/rust`.

**The substrate-pull recognition.** `@code/macro` is NOT a new family.
It is a **sub-prism family that lives under each `@code/X` species** —
the language-side metaprogramming reception layer, declared at the
altitude that already declares the language's render and translate
surface. The placement matches the precedent set by
`shards/code/rust.mirror:55` (`@code/rust/cargo` — emit-target
identity inside the altitude grammar). The thirty-first instance of
*substrate-already-had-the-word*: the substrate had `@code/X` as the
species; the macro sub-prism is the reception half that was waiting
for a name.

---

## 3. Placement — the substrate decision

### 3.1 Where `@code/metalogue` lives (the ground) and where `@code/X/macro` lives (the species)

**The ground lives at `shards/code/metalogue.mirror`.** This is the
universal contract, declared once: the four shim signatures, the
four laws as `requires` clauses, the declaration/turn/metalogue_session
carrier types. The ground is parametric on `species_ast` — each
species binds it to its language's AST type. Same shape as
`shards/metalogue.mirror` carries `turn.body: nl` at the NL altitude;
`@code/metalogue` carries `turn.body: declaration` at the AST altitude.

The per-species roots (`@code/X/macro`) live as before, but now as
substrate-inheritors of `@code/metalogue` via `in @code/metalogue`
(the canonical `<=`-mechanism per
`[[architecture-prism-as-trait-as-everything]]`). The species shard's
job is the per-altitude binding:

1. Bind `species_ast` to the language's AST type (`code/rust.ast`,
   `code/elixir.ast`, etc.).
2. Declare the four shim function signatures with the bound return
   type. The bodies are `\` per consumer-pull discipline.
3. (No `requires` clauses — those inherit from `@code/metalogue`.)

The Rust species shard (`shards/code/rust/macro.mirror`) is the
canonical realisation; this shape applies identically to every
other species.

### 3.1bis Where `@code/X.macro` lived (original 2026-06-08 placement, pre-reframe)

**Inside each `@code/X` species, as a sub-prism.** Same shape as
`@code/rust/cargo` lives inside `@code/rust`. Three readings
converge:

1. **Path-namespace property** (per
   `[[architecture-shards-as-substrate-source]]`): the path
   `shards/code/rust/macro.mirror` declares
   `@code/rust/macro`. The path IS the declaration. The macro
   surface IS-A part of the language altitude, not a sibling.
2. **Inheritance discipline** (per
   `[[architecture-prism-as-trait-as-everything]]`): the macro
   surface inherits the altitude's typed primitives (`rust_ast`,
   `fn`, `impl`, `struct`, etc.) without re-declaring them. It
   specializes one new dimension: the metaprogramming contract.
   Sibling placement would force re-declaration of the AST type.
3. **Substrate-pull discipline**: the per-language metaprogramming
   surface IS load-bearing AT THE ALTITUDE. Rust's `macro_rules!`
   only makes sense at the Rust altitude; Lisp's `defmacro` only
   makes sense at the Lisp altitude. The discipline is universal;
   the realisation is per-altitude. Universal → declared at the
   `@code` family root; per-altitude → realized in `@code/X/macro`.

### 3.2 What the `@code` family root declares (universal discipline)

A two-line addition to `shards/code.mirror`'s family root, naming the
universal metaprogramming-reception contract as a placeholder under
which each species declares its own macro sub-prism. The family root
does not declare the macro shape directly (just as it does not
declare the render shape directly); it names the discipline that
each species realizes.

Concretely, the family root prose gains a section naming:

> *"Each `@code/X` species MAY declare a sub-prism
> `@code/X/macro` realizing the language's metaprogramming-reception
> surface — the language-side compile-time AST consumption layer that
> catches `@code/X.render`'s emission. Required when the species'
> language admits direct AST manipulation (Rust, Elixir, Lisp/Scheme,
> Julia, Crystal, Nim, Clojure); optional otherwise. The presence of
> the macro sub-prism IS the substrate-realisability witness for
> `@code/X` as a Phase-4b generation target."*

No new family. One paragraph added to `shards/code.mirror`'s docstring
in a future tick. The current spec is the design; the substrate touch
is one paragraph + each species' macro sub-prism file.

### 3.3 The species roster — canonical declaration + forward-promise

Per `[[feedback-craft-not-deliver]]` (consumer-pull discipline:
declare the canonical species; forward-promise the others with named
consumer-pull conditions).

**Canonical declaration this tick (when bodies land):**

- `@code/rust/macro` — Rust's `macro_rules!` + procedural macros
  (`proc_macro`, `proc_macro_derive`, `proc_macro_attribute`) + the
  `syn` / `quote` / `proc_macro2` ecosystem + `#[derive(...)]`
  expansion. THE canonical declaration because:
  - Rust is the bootstrap's host language; the macro surface is
    immediately consumable for the Phase 4b
    `@fragmentation + @code/rust → fragmentation/src/*.rs` proof.
  - The `prism-derive` crate already exists (`prism/derive/src/lib.rs`,
    531 LOC, two proc-macros `#[derive(Prism)]` and `#[derive(Lambda)]`
    per `fragmentation-as-generated.md` §3.1). The substrate has a
    working proc-macro layer to declare AT.
  - The `mout!` / `merr!` recognition that started this turn lives
    at the Rust altitude.

**Forward-promised species (named here; bodies land when a consumer
pulls):**

- `@code/elixir/macro` — Elixir's `quote` / `unquote` / `defmacro`.
  Lands when a consumer pulls (likely Reed's BEAM body in
  `/Users/reed/body/`, when the spectral runtime needs substrate-
  emitted Elixir modules at compile time).
- `@code/lisp/macro` — Lisp/Scheme/Clojure's hygienic macros (the
  canonical homoiconic case where AST = list). Lands when a
  consumer pulls (likely a `@code/scheme` species via T11 or later
  research interest).
- `@code/julia/macro` — Julia's `Expr` / `@macroexpand`. Lands when
  numerical-substrate consumers pull (potentially a sibling to the
  `@code/fortran` LapackBackend track).
- `@code/crystal/macro` — Crystal's `macro` block with compile-time
  block-scoped AST traversal. Forward-promised; no known consumer
  yet.
- `@code/nim/macro` — Nim's `macro` / `template` / `static[T]`.
  Forward-promised; no known consumer.
- `@code/typescript/macro` — TypeScript decorators (a weak macro
  surface; pre-tc39 stage-3 decorators 2024+). Forward-promised
  with the caveat that TypeScript's metaprogramming is shallower
  than the others; the species may degrade to compile-time type-
  only generation (Conditional Types / Mapped Types) rather than
  full AST manipulation.

**Explicitly NOT in the species roster:**

- `@code/python/macro` — Python has no canonical macro surface
  (decorators are runtime; `__init_subclass__` is runtime;
  `__class_getitem__` is runtime; the AST module is post-hoc
  manipulation, not compile-time injection). A `@code/python`
  species lands without a macro sub-prism. The substrate
  generates Python source TEXT through `@code/python.render`; the
  consumption side is `import` at runtime, not macro expansion.
- `@code/go/macro` — Go deliberately omitted macros from the
  language. Generation through `@code/go.render` + `go generate`
  invocation through `@io.go_generate` (boundary, not macro).

This matches Alex's recognition's qualifier verbatim: *"for any
language that allows direct manipulation of the AST."* The languages
that don't admit it (Python, Go, Java pre-records, etc.) fall back
to text-rendering-plus-boundary-tooling. The macro sub-prism is
the WITNESS that the language admits the deeper coupling.

---

## 4. The metalogue ground's typed surface

The universal contract lives at `@code/metalogue` (the 34th-instance
reframe). The shape below was originally written as "each
`@code/X/macro` sub-prism declares"; in the reframed reading, the
shape is declared ONCE at `@code/metalogue` (parametric on
`species_ast`); each species inherits and binds `species_ast` to its
language's AST type. The text below stays per-species in form to
preserve the original derivation; substitute `@code/metalogue` for
the declaration site and read `species_ast` for any language-AST
return type:

```
prism @code/X/macro {
  focus macro
  project macro
  split macro
  shift macro
  settle macro
}

# The macro shim's typed contract.
#
# input:  a substrate AST node (the `.mirror` declaration the
#         macro is realizing — typically a single `action` decl
#         with `\` body, or a `type` decl, or a `prism` decl).
# output: the language-altitude AST that, when expanded by the
#         language's macro layer, produces the executable
#         realization of the substrate declaration.

shim(substrate_decl: ref, target: ref) -> X_ast { \ }

# The round-trip law. Given a substrate declaration D and the
# rendered Rust source R = render(shim(D)), parsing R back through
# @code/X.parse must yield an AST structurally equal to shim(D).
# I.e., the language-side parse is the left inverse of the macro
# emission. This is the same law `@code/mirror.render` declares
# (round_trip(render)) at the macro layer.

requires round_trip(shim)

# The substrate-equivalence law. Given two substrate declarations
# D1 and D2 that are structurally equal, shim(D1) and shim(D2)
# must produce ASTs that yield identical content-addresses when
# rendered. I.e., the macro emission is a function of the
# substrate declaration's OID — same substrate OID → same
# rendered OID.

requires oid_function(shim)
```

The `shim` action is the universal language-side metaprogramming
contract:

- **Input** is a `ref` (the substrate's universal content-reference
  carrier per `shards/mirror/store.mirror`'s `oid = ref`
  declaration) — pointing at the substrate AST node whose
  realization the macro emits.
- **Output** is `X_ast` (the language's typed AST — `rust_ast` per
  `boot/std/code/rust.mirror:49`, `elixir_ast` per a future
  `@code/elixir` species, etc.).
- **Body** is `\` (the obligation block) — discharged when a
  consumer pulls. T22 / Phase 4b's pilot pulls the Rust shim
  first.

### 4.1 The four kinds of substrate declarations the shim handles

Per `boot/std/code/rust.mirror` and `shards/mirror/loss.mirror`'s
typed-lambda discipline, the substrate declarations a Rust shim
consumes are:

1. **`type` declarations** → emit a Rust `struct` / `enum` / type
   alias. Generic parameters from the substrate's `type T(U)` form
   become `<U>`; closed-sum variants become enum variants;
   single-variant become structs.

2. **`prism` declarations** → emit a Rust `struct` plus a
   `#[derive(Prism)]` annotation plus the `#[oid("@X")]` attribute.
   The five-op block at the substrate level becomes the
   `Prism` trait impl scaffolding; the `prism-derive` proc-macro
   fills in the optic accessors. This matches
   `fragmentation-as-generated.md` §3.2's expansion exactly.

3. **`action` declarations with `\` body** → emit a Rust `pub fn`
   matching the signature. The body is the shim's responsibility:
   - If the action is `@io` (the realisation discriminator
     classified the target as `boundary`), the body is the
     boundary call (cargo subprocess, libc syscall, vendor SDK).
   - If the action is substrate-realisable, the body is the
     lowered version of the action's substrate decl (the shim
     reads the substrate AST and emits the equivalent Rust
     control flow).
   - If the action has `\` body AND a consumer-side macro
     instructs the substrate to defer (the `\` IS the
     substrate's question to the macro — "what should this
     compute?"), the body is generated by the substrate's
     reverse direction: `min!(substrate_channel, T)` reads a
     value from the substrate INTO Rust scope (the bidirectional
     `mout!`'s sibling).

4. **`grammar` declarations** → emit a Rust module containing the
   typed AST for the grammar's productions, plus optional
   `parse` / `render` entry points. This matches what
   `prism-derive` and `boot/std/code/rust.mirror` already declare
   at the substrate side.

### 4.2 The Rust shim's concrete shape — `prism-derive` evolution

Per `prism/derive/src/lib.rs` (audited per
`fragmentation-as-generated.md` §3.1), the two existing proc-macros
are:

- `#[derive(Prism)]` — emits `Addressable`, `Display`, accessor
  structs (`<Field>Lens` / `<Field>Prism` / `<Field>Traversal` /
  `<Field>Iso`), `optic_fields()`. Requires `#[oid("@name")]`.
- `#[derive(Lambda)]` — emits `Addressable`, `Display`,
  `From<X> for Lambda<T>`, `Composable<T>`. Requires
  `#[oid("@name")]`.

The macro sub-prism's Rust realization is a THIRD proc-macro:

```rust
// THE CODE GENERATION ENTRY POINT.
//
// Reads the substrate AST (via a `mirror::declaration!{}`
// macro call site) and emits the Rust realization. The
// substrate AST is content-addressed by OID; the macro
// resolves the OID through the fragmentation store and emits
// the appropriate Rust per §4.1 above.
//
// This is what `@code/rust/macro.shim` discharges to.

#[proc_macro]
pub fn declaration(input: TokenStream) -> TokenStream {
    // input is a path-like reference (e.g., `@mirror/loss`)
    // resolution: walk shards/ and boot/ to find the .mirror
    //   declaration matching the OID
    // dispatch: per the declaration's kind (type/prism/action/grammar),
    //   emit the Rust realization per §4.1
    // round-trip: the emitted token stream is what cargo
    //   compiles; parsing it via syn yields a token tree that
    //   round-trips back to the substrate AST via @code/rust.parse.
}
```

The proc-macro call site at the consumer:

```rust
// In a downstream crate (e.g., fragmentation-mcp):
mirror::declaration!(@mirror/loss);

// At macro-expansion time, the proc-macro reads
// shards/mirror/loss.mirror, walks the typed AST, and emits
// the full Rust module declaring MirrorLoss, ParseLoss,
// ResolutionLoss, etc. plus their trait impls and signature
// glue. The consumer file becomes a one-line dispatch into
// the substrate's declaration.
```

**This IS what Alex named.** The `mirror::declaration!` proc-macro
IS the language-side reception of the substrate's emission. The
language's macro layer catches the rendered code and threads it into
the language's own AST. The gap between
`shards/mirror/loss.mirror`'s `\` bodies and `src/loss.rs`'s actual
impls becomes the proc-macro's responsibility to close — at
compile time, deterministically, per the substrate's typed
declaration.

### 4.3 `project_hole` — the universal hole-projection contract (the 35th-instance recognition)

**No language has the exact semantics of the hole. It will always be a projection.**

That sentence is load-bearing. The substrate's `\` is the typed
hole — the obligation block per
`[[architecture-prism-as-trait-as-everything]]`, the unbound
morphism the species realisation must close. Every host language's
metaprogramming layer realises the hole differently, and **none of
the realisations is exact-semantic**:

- Rust's `todo!()` panics at runtime with the `!` (never) type;
  close to a typed-gap, but committed to a runtime trap.
- Rust's `_` is a pattern wildcard / type placeholder; not an
  action body.
- Rust's `{ }` is an empty unit-returning block; lexer-realisable
  but loses the obligation typing.
- Elixir's `nil` returns from a `def` body; runtime hole.
- Elixir's `Process.unimplemented/0` (when convention chosen) is
  closer to `todo!()` but BEAM-altitude-specific.
- Lisp's `'TODO` symbol / `(error "todo")` — runtime trap, no
  static gap surface.
- Julia's `error("not implemented")` — same shape as Lisp's; the
  `Expr` AST has no typed-gap node.

The substrate cannot pretend any of these IS the hole. Each
species **lossy-projects** the hole into its own metaprogramming
vocabulary. The lossy delta IS admitted at the @glass altitude as
transparency on the metalogue turn that carries the projection
(per §5 law 4: substrate-pull preservation surfaces the lossy
fragment rather than absorbing it).

**The substrate vocabulary already had the words.** Three substrate
ancestors triangulate the 35th instance:

1. `shards/nl.mirror` (line 29): *"Same shape as `\` producing
   fracture (an obligation hole)."* `hole` is the noun.
2. `boot/std/mirror/interpreter.mirror` (lines 16-17):
   `resolve_hole(hole, context) -> ast { \ }`. `hole` is the
   substrate-altitude type carrier.
3. `boot/std/fate/tournament.mirror` (lines 33-41):
   `candidates(hole) -> [resolution]`. The five-ganglion candidate
   surface consumes `hole`.

The substrate ALREADY used `hole`; this tick names how `hole`
crosses the species boundary. The operation IS `project` (one of
the five-op operations on `@code/metalogue` per the prism block at
the ground). The hole-projection IS `project`'s contract at the
@code/metalogue altitude made explicit; it is NOT a new substrate
operation.

#### 4.3.1 The universal signature (at the ground)

Declared at `shards/code/metalogue.mirror` (the 35th-instance
substrate touch):

```
project_hole(h: hole) -> species_ast { \ }

requires round_trip(project_hole)
requires oid_function(project_hole)
requires type_soundness(project_hole)
requires substrate_pull_preserving(project_hole)
```

The body stays `\` at the ground (per
`[[feedback-craft-not-deliver]]`); each species binds the return
type and supplies the lossy projection body when a consumer pulls.

`shim_action`'s dispatch routes through `project_hole` whenever its
input declaration carries a `\` body — this is how the four-shim
contract reduces the hole case to the project_hole binding. Same
substrate `hole`, different species realisation; the four laws
apply identically (round-trip up to the species' lossy semantics;
the partial verdict is admitted per @glass `transparency(turn)`).

#### 4.3.2 The Rust binding (the canonical species this tick)

Declared at `shards/code/rust/macro.mirror` (the 35th-instance
species-side touch). Rust's lossy projection lives at two
altitudes simultaneously, because Rust's lexer and Rust's AST
realise the hole at different surface levels:

| Altitude | Substrate carrier | Rust projection | Why |
|---|---|---|---|
| Source text (proc-macro call site) | `\` | `{ }` | Rust's lexer does not realise the bare `\` token; the empty block is the smallest Rust-lexer-realisable surface the proc-macro can receive a TokenStream over. |
| Rust AST (proc-macro emission body) | `\` | `todo!()` | Rust's `todo!` macro has type `!` (the never type) which unifies with any expected return — the least-lossy typed-gap projection Rust admits. `unimplemented!()` and `panic!()` have the same shape but lose the "deferred" semantic. |

Per the T25 GREEN observation: this two-altitude split is **not** a
workaround; it is the explicit mapping Alex's read names. The Rust
lexer's surface admits `{ }`; the Rust AST's typed-gap node is
closest to `todo!()`. Both projections are lossy. Both are honest.
The substrate names the lossiness rather than absorbing it.

#### 4.3.3 Why this is `project`, not a new operation

Three readings converge on `project` being the right operation:

1. **The five-op block at `@code/metalogue` already declares
   `project metalogue`.** Per the prism block at
   `shards/code/metalogue.mirror` line 102. The hole-projection IS
   what `project` does at this altitude; making it explicit names
   the existing operation's contract, not a new one.

2. **The species realisation chain matches `project`'s
   five-operation typing.** `project` is the "filter by what
   matters" operation in the five-op vocabulary; the hole-projection
   filters the substrate `hole` carrier through the species'
   accepting vocabulary, surfacing only what the species realises
   and admitting the residual as transparency. Same shape.

3. **Per `[[feedback-substrate-already-had-the-word]]`** (the
   34-instance recognition track): Mara's candidate naming "the
   lexical glass-wall" was a new noun; the substrate already had
   `hole` as the noun AND `project` as the operation. The 35th
   instance is the recognition that **the candidate naming was the
   training-pull**; the substrate vocabulary was already in place,
   we just needed to use it.

### 4.4 The bidirectional `mout!` — `mout!` / `min!` / `mio!`

Per `bootstrap/src/lib.rs:178-200`, the current `mout!` / `merr!`
macros are Rust→bytes-on-fd-1/2 channels that bypass libtest's
`OUTPUT_CAPTURE`. The substrate-routable channel IS implicit (the fd
itself is the channel); the typed reading is forward-promised.

The bidirectional reading the substrate names:

**Rust → substrate (current shape, typed):**

```rust
mout!(target_channel, "{}", x)
```

expands to:
1. Format `x` per `format!` ABI.
2. Look up `target_channel` against `@mirror/io`'s declared channel
   roster (each channel is a typed `ref(@mirror/io/channel/<name>)`).
3. Type-check the formatted value against the channel's declared
   type at compile time (the proc-macro reads the substrate
   declaration of the channel).
4. Write the bytes to the channel's underlying fd via `libc::write`
   (current `_raw_stdout` / `_raw_stderr` mechanism).

**Substrate → Rust (the new shape, named):**

```rust
min!(source_channel, T)
```

expands to:
1. Look up `source_channel` against `@mirror/io`'s declared channel
   roster.
2. Read bytes from the channel's underlying fd via `libc::read`.
3. Parse the bytes into `T` per the channel's declared type's
   `@code/rust.parse` instance.
4. Yield the typed value into Rust scope.

The unified shape:

```rust
let typed_input: T = min!(source);
mout!(target, "{:?}", typed_input);
```

OR, the round-trip form:

```rust
mio!(source -> target, |x: T| process(x))
```

— read from `source` as `T`, apply `process`, write to `target`.

These macros LIVE inside `@code/rust/macro`. They are the canonical
realisation of "typed bidirectional channel at the @mirror/io
altitude" via Rust's macro surface. Per §4.1, they are
**substrate-realisable**: the substrate declares the channel's typed
contract; the macro emits the language-side reception.

The 32nd-instance candidacy named in Win 2's report becomes a
*landed* substrate-pull recognition when `@mirror/io/channel`
declares the channel roster and `@code/rust/macro.mout` / `.min` /
`.mio` declare the Rust shim.

---

## 5. "The gap is the spec" — formalized (the four laws live at the ground)

Per the 34th-instance reframe: the four laws below are declared
ONCE at `@code/metalogue` (the universal ground), as `requires`
clauses on `shim_type` / `shim_prism` / `shim_action` /
`shim_grammar`. Per-species shards inherit the obligations via
`in @code/metalogue`. The original text below is written
per-species; the lifted version names the same laws at the
ground.

This is the central thesis. Stating it precisely:

**Every `\` body in a substrate declaration IS a specification for
code generation.** The substrate declaration names what; the macro
shim names how; the language compile invocation produces the
binary.

Formally, given a substrate AST node `D` with `\` body at the
`@code/X` altitude:

```
D ::= action name(args) -> ret { \ }
    | type name(params) = variant1 | variant2 | ...
    | prism @path { five_op_block }
    | grammar @path(extensions) { ... }
```

The macro shim `shim_X: D -> X_ast` is a total function such that:

1. **Type-soundness.** For every action `D`, the emitted `X_ast`'s
   signature matches `D.args` and `D.ret` under
   `@code/X.type_mapping(D.args, D.ret)`. The substrate's typed
   contract IS the Rust function's typed contract — *the gap is
   typed*.

2. **Round-trip identity.** For every `D`, `parse_X(render_X(shim_X(D))) =
   shim_X(D)` structurally. The macro emission is a fixed point of
   the language-side parse cycle — *the gap closes deterministically*.

3. **OID functionality.** For substrate declarations `D1`, `D2` with
   identical content-addresses (`oid(D1) = oid(D2)`),
   `oid(render_X(shim_X(D1))) = oid(render_X(shim_X(D2)))`. The macro
   is a function of the substrate OID; same substrate OID, same
   emitted OID — *the gap is content-addressed*.

4. **Substrate-pull preservation.** If `D` is substrate-realisable
   per `@mirror/realisation.classify(D) = substrate(target)`, then
   `shim_X(D)` references only Rust constructs that themselves
   appear in some `@code/rust` substrate-realisable shard. The
   emission cannot reach into `@io` to fill a substrate-realisable
   gap. The discriminator's verdict IS the discriminator of the
   shim — *the gap respects the glass wall*.

Per `[[architecture-prism-as-trait-as-everything]]`, these four
laws are declared as `requires` predicates on the `shim` action of
`@code/X/macro`. The model checker discharges them when the body
lands.

### 5.1 Why "gap" and not "stub"

A stub is a placeholder waiting to be filled by whoever. A gap is
a specification of a precisely-typed hole that *only one shape* can
fill. The macro shim's contract is: given a substrate declaration's
typed signature, the shim's emission is uniquely determined up to
content-address equivalence by the four laws of §5.

This is the same load-bearing recognition as Young 2026's H¹
floor (per the prophecy insight,
`docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.md`):
the substrate's gaps are H¹ cocycles, and the macro shim's job is
to *exhibit the cobordism* that bounds the cocycle. The shim is
the construction; the round-trip law is the witness that the
construction succeeded.

### 5.2 Connection to the eigensheaf

Per `docs/specs/eigensheaf.md` (T18, 2026-06-07): generation IS
spectral decomposition. The macro shim's emission is the Hodge
projection from the substrate declaration's eigenmode onto the
language altitude's stalk:

- The substrate declaration `D` sits in the eigensheaf's section
  space at the `@code/mirror` altitude.
- The macro shim is the restriction map `F_{@code/mirror ⊴ @code/X}`
  — the linear map that transports the section to the
  language-altitude stalk.
- The rendered output `render_X(shim_X(D))` is the projected
  section's value at `F(@code/X)`.
- The round-trip law is the assertion that the restriction map is
  invertible (a sheaf-isomorphism, not merely a homomorphism).

"We don't compute. We crystallize." (per the eigensheaf spec)
becomes: the macro shim doesn't generate code — it crystallizes
the substrate declaration into the language altitude's eigenmode.

---

## 6. Round-trip identity — the central contract (declared at the ground)

Per the 34th-instance reframe, `round_trip(shim_X)` is declared
ONCE at `@code/metalogue` for each of the four shims (one per
declaration kind). Per-species shards inherit; the law applies
structurally identically at every language altitude. Per
`@code/mirror.render`'s `requires round_trip(render)` at
`shards/code/mirror.mirror:256`, the same property law applies to
each `@code/X.macro.shim` via inheritance from `@code/metalogue`:

```
requires round_trip(shim)
```

Operationally, this means:

```
For every substrate AST node D at altitude @code/X:
   parse_X(render_X(shim_X(D))) ≡ shim_X(D)
```

Where:
- `shim_X` is the macro emission action declared at `@code/X/macro`.
- `render_X` is the AST-to-text projection declared at `@code/X.render`.
- `parse_X` is the text-to-AST parsing declared at `@code/X.parse`.
- `≡` is structural equality on the language's AST (content-address
  equality after canonical-form reduction).

The stronger reading: given a Rust source file produced by
`render_X(shim_X(D))`, the file's content-address under
`@code/rust.oid` equals `shim_X(D).oid` — same substrate input,
same content-addressed Rust output, no float in the rendering.

### 6.1 What round-trip identity buys

1. **Reproducible builds at the substrate level.** Two builds of
   the same mirror project produce byte-identical Rust source files
   (and therefore byte-identical binaries, modulo cargo's own
   determinism).

2. **Substrate-pull migration is safe.** Migrating a Rust file from
   the bootstrap to a `.mirror` declaration is loss-free: the
   generated Rust file equals the original byte-for-byte (after
   canonical formatting per `@code/rust.render`'s pretty-printer).

3. **Phase 4b's commitment is a theorem.** Per
   `roadmap/pending/phase-4-emitter-self.md` task 3: "Generate
   fragmentation/src/ from `@fragmentation + @code/rust`. Replace
   the hand-written Rust with the generated version." Round-trip
   identity IS the witness that the replacement is
   indistinguishable from the original.

4. **The discriminator's verdict is verifiable.**
   `@mirror/realisation.classify(file.rs) = substrate(target)` AND
   `render_X(shim_X(target.declaration)) ≡ file.rs` together prove
   that the file is substrate-realisable. The first is the prose
   classification; the second is the constructive witness.

### 6.2 What round-trip identity does NOT promise

- **It does not promise that the generated Rust is human-pretty.**
  Round-trip is up to canonical-form reduction (whitespace
  normalization, syntactic-sugar normalization). The substrate's
  `@code/rust.render` chooses one canonical form; round-trip holds
  modulo that choice.

- **It does not promise that cargo's build output is byte-identical.**
  Cargo embeds timestamps, build environment, and platform-specific
  metadata. The shim's output is the source crate; round-trip is at
  the source altitude, not the binary altitude.

- **It does not promise that all four substrate-declaration kinds
  round-trip identically.** Type declarations and prism declarations
  round-trip exactly. Action declarations round-trip up to the body's
  expression normalization (which may differ in syntactic sugar even
  if semantically equivalent). Grammar declarations round-trip at
  the grammar-AST level but may render different surface text under
  different layout-width settings.

---

## 7. Connection to Phase 4b — fragmentation-generated collapses cleanly

Per `roadmap/pending/phase-4-emitter-self.md`:

> *"Goal: Output formats described as `.mirror` grammars.
> Fragmentation's Rust source generated from
> `@fragmentation + @code/rust` — the in-compiler demonstration that
> mirror compiles production code."*
>
> Tasks:
> 1. Complete the `@code/rust` translate template per
>    `docs/specs/fragmentation-as-generated.md`.
> 2. Write the `@fragmentation.mirror` grammar (~400-600 lines projected).
> 3. Generate `fragmentation/src/` from `@fragmentation + @code/rust`.
>    Replace the hand-written Rust with the generated version.
> 4. Write the `@code/mirror` render template (the pretty-printer).
>    Round-trip: parse → emit → parse = identity.

This spec names the mechanism by which task 1 ("complete the
`@code/rust` translate template") and task 3 ("generate
`fragmentation/src/`") get realized.

**Task 1 becomes**: declare `@code/rust/macro` with the `shim`
action declared per §4.1, with bodies that emit the four
declaration kinds. The translate template
(`@code/rust.translate(p: @prism, c: @code/rust)`) per
`fragmentation-as-generated.md` §2 IS what `shim` discharges; the
abstract template lives at `@code` (per `boot/04-code.mirror`), the
concrete instance lives at `@code/rust/macro`.

**Task 3 becomes**: write a `mirror::declaration!{@fragmentation}`
call at the top of each `fragmentation/src/*.rs` file. The proc-macro
expands to the substrate's declared shape. Cargo builds the result.
Round-trip identity per §6 witnesses the equivalence with the
hand-written placeholder.

**Task 4 becomes** what `@code/mirror.render` (T17, landed yesterday)
already declares. The round-trip law on `@code/mirror.render` IS what
the round-trip law on `@code/X.macro.shim` extends to every other
language altitude.

### 7.1 The collapse

The Phase 4b plan ("fragmentation generated from
`@fragmentation + @code/rust`") and the macro surface ("`@code/X/macro`
catches `@code/X.render`'s emission via the language's proc-macro
layer") are TWO READINGS of the same mechanism. They differ in
naming, not in mechanism:

| `fragmentation-as-generated.md` (2026-05-24) | This spec (2026-06-08) |
|---|---|
| `@code/rust.translate(p: @prism, c: @code/rust)` | `@code/rust/macro.shim(D, target) -> rust_ast` |
| `@code/rust.render(g: @code/rust, ast(g) → io_list)` | `@code/rust.render(ast) -> text` (T17 landed) |
| Prism-derive proc-macros emit accessor structs | `mirror::declaration!{...}` proc-macro emits the full Rust module |
| Round-trip pending future tick | Round-trip declared as `requires round_trip(shim)` |
| "The codegen ticket" (`@code/rust.translate` not yet landed) | "The macro sub-prism" (`@code/rust/macro` not yet landed) |

The macro surface IS the mechanism for the Phase 4b commitment. The
fragmentation-as-generated spec's R-tickets (R-0 through R-6 per its
§5) become the implementation cascade for `@code/rust/macro.shim`'s
body.

### 7.2 The unique addition this spec makes

What this spec adds beyond fragmentation-as-generated.md:

1. **The recognition that the codegen pipeline IS the language's
   macro layer.** Fragmentation-as-generated.md treats codegen as
   text emission ("Rust source bytes → cargo build"). This spec
   recognizes that text emission is one of two readings; the other
   is *direct AST injection via the language's macro layer*. The
   `mirror::declaration!{}` proc-macro pathway skips the text-on-
   disk intermediate; the substrate AST IS the macro's input; the
   compiled binary IS the output.

2. **The bidirectional surface.** Fragmentation-as-generated.md is
   one-way (substrate → Rust). This spec names the other direction
   (`min!`-style consumption from substrate channels INTO Rust
   scope at macro-expansion time). The mirror-to-Rust direction
   IS what makes `mout!` and `merr!` substrate-pull recognition
   candidates: they're already the macro layer's reception
   surface; naming the family completes the reading.

3. **Polyglot extension.** The Phase 4b commitment is Rust-only.
   This spec names the universal shape that extends to Elixir, Lisp,
   Julia, Crystal, Nim. The substrate becomes a polyglot generator
   from a single substrate-declaration source.

---

## 8. Connection to kintsugi-on-Rust (T22+) — the classification IS the spec

Per `shards/mirror/realisation.mirror` (T21, landed) and T22 (named in
the rug pull doc §6 as the kintsugi-on-Rust pilot):

The discriminator `classify(f) -> realisable_file` reads a Rust
file's AST and emits a verdict + target altitude. The 29 substrate-
pull recognitions are the training set (per
`shards/mirror/realisation.mirror:130`).

The macro shim's relationship to the discriminator:

- **The discriminator names WHAT to regenerate.**
  `classify(loss.rs) = substrate(@mirror/loss)` says: loss.rs is
  realizable at @mirror/loss; the substrate already declares its
  content.
- **The macro shim names HOW to regenerate.**
  `shim_rust(@mirror/loss) -> rust_ast` produces the Rust AST
  whose render equals the original loss.rs (up to round-trip
  identity).

Together they discharge the kintsugi-on-Rust contract:

```
For every Rust file f in bootstrap/src/:
   IF classify(f) = substrate(target) THEN
      render(shim_rust(target)) ≡ f
   ELSE classify(f) = boundary(@io/X)
      f stays in Rust at the @io altitude
```

This is the bootstrap-retirement contract per the butterfly roadmap:
the bootstrap shrinks to pure-@io as the discriminator classifies
every file. The macro shim IS the constructive witness that the
classification is correct — the file is substrate-realisable iff
the macro can regenerate it byte-identical.

### 8.1 The 21-row classification table becomes 21 macro shim cases

Per T22's classification table per `kintsugi-self-hosting.md` §§2-3:
21 Rust files in `bootstrap/src/` each have a "Status:" line. For
the 14 files classified as substrate-realisable, the macro shim's
body has 14 concrete cases (one per altitude target). For the 7
files classified as @io boundaries, no shim case is needed —
they stay at the `@io` altitude permanently.

Each row of the classification table becomes one branch of the
shim's dispatch. The classification names the substrate altitude;
the dispatch reads the altitude's declared grammar; the
emission per §4.1 follows mechanically.

---

## 9. Connection to `@code/mirror.render` (T17) — sibling at the same altitude

T17 (landed 2026-06-08 yesterday) declares
`@code/mirror.render(ast) -> text` per
`shards/code/mirror.mirror:228`. The macro shim's relationship:

- **Same altitude.** Both `render` and `macro.shim` live under
  `@code/X` species. They are sibling sub-prisms (the same way
  `@code/rust/cargo` and `@code/rust/macro` would be siblings).
- **Same Wadler/Bernardy algebra.** The doc combinator surface
  declared at `@code/mirror.render` IS reused by `@code/X.render`
  for every other language. The shim's output is rendered
  through the doc algebra.
- **Same round-trip law.** `requires round_trip(render)` on
  `@code/mirror.render` is the same shape as
  `requires round_trip(shim)` on `@code/X/macro`. Different
  pipeline stages; same algebraic contract.
- **Same family root pattern.** Render is per-species
  (`@code/mirror.render`, `@code/rust.render`, etc.); macro is
  per-species under each. The family root (`@code`) declares
  the discipline; each species realizes it.

The two halves compose:

```
substrate_decl (mirror altitude)
   │
   │   @code/X/macro.shim(D, X)
   ▼
X_ast (language altitude)
   │
   │   @code/X.render(X_ast)
   ▼
language source text
   │
   │   language compiler
   ▼
binary
```

Render is the second-to-last step (AST → text). Macro shim is the
step before (substrate AST → language AST). Together they discharge
the substrate-to-binary pipeline.

---

## 10. The `\` hole as substrate-pull surface

Per `[[architecture-prism-as-trait-as-everything]]`, every typed
lambda's `\` body is the substrate's invitation: the action's
signature is declared; the body is the consumer's responsibility
(per `[[feedback-craft-not-deliver]]`, the substrate names the
discipline, the consumer pulls the realization).

The macro shim is the **universal consumer** for `\` bodies at the
@code/X altitude:

- Every action declared with `\` at `@code/X` is a candidate for
  the macro shim's dispatch.
- The shim reads the altitude's family roster
  (`@mirror/lens`, `@mirror/loss`, `@mirror/data`, `@code/mirror`,
  etc.) and asks: does the substrate already declare HOW to fill
  this hole?
- If yes (the discriminator's verdict is substrate-realisable),
  the shim emits the Rust realization per §4.1.
- If no (the action requires a boundary call), the shim emits
  the `@io.cargo` / `@io.fs` / `@io.crypto` invocation per the
  species' boundary roster.

The `\` hole IS the substrate-pull recognition opportunity at the
file altitude. The macro shim IS the mechanical realisation of the
recognition. The 29 substrate-pull recognitions logged in the rug
pull doc were Mara doing this by hand; the macro shim's landing
makes the dispatch automatic.

### 10.1 The dispatch table (at the ground)

The dispatch is declared at `@code/metalogue` — universal across
all species. Each species' per-altitude binding (`@code/X/macro`)
imports this dispatch via `in @code/metalogue`; the only
per-species variation is `species_ast`-binding and the per-kind
emission rule (the language's specific AST shape for `struct` /
`enum` / `fn` / `mod`).

```
shim(D, @code/rust) match D.kind:
   | type(name, params, variants) ->
        emit Rust struct/enum per §4.1.1
   | prism(@path, five_op_block) ->
        emit Rust struct + #[derive(Prism)] per §4.1.2
   | action(name, args, ret, \ body) ->
        match classify(action):
        | substrate(target_altitude) ->
              emit pub fn matching signature;
              body = lowered_target_altitude_grammar
        | boundary(@io/species) ->
              emit pub fn matching signature;
              body = @io.species.invocation
        | partial(opacity_map) ->
              emit pub fn matching signature;
              body = mixed (per-function refinement;
                            audition tournament input)
   | grammar(@path, productions, extensions) ->
        emit Rust mod with typed AST per productions;
        emit parse + render entry points
```

This dispatch is what the proc-macro's body discharges. Mechanical.
Deterministic. Round-trip-witnessed.

---

## 11. Forward-promise — cross-language code generation (per-species matrix retained)

The species roster below is the per-altitude binding side; the
universal contract lives at `@code/metalogue`. Each species row
declares its language's AST type binding (`code/X.ast`) and its
language's macro-surface community word, but inherits the four
shims and four laws from the ground. Once `@code/rust/macro`
proves the shape (Phase 4b's fragmentation-as-generated pilot),
the universal pattern extends to every other species whose
language admits direct AST manipulation:

### 11.1 `@code/elixir/macro`

Elixir's `quote` / `unquote` / `defmacro` is the language-side
reception. The Reed-BEAM-body consumer (per `/Users/reed/body/`)
pulls when the spectral runtime needs substrate-emitted Elixir
modules. The shim's dispatch:

- `type` decl → emit Elixir struct or `Algebra.Tagged.Union` for
  closed-sum.
- `prism` decl → emit Elixir module + `@behaviour` matching the
  five-op block.
- `action` decl → emit `def` matching the signature.
- `grammar` decl → emit Elixir module containing the AST as
  tagged tuples.

The `quote` macro receives the substrate AST; the emitted Elixir
AST is structurally equal to what `Code.string_to_quoted/1` would
return for the rendered source. Round-trip identity holds at the
Elixir AST altitude.

### 11.2 `@code/lisp/macro` (and Scheme, Clojure)

The canonical homoiconic case. The substrate AST IS the Lisp AST
(both are tagged-list-shaped). The shim becomes nearly trivial:

- `type` decl → emit `(define-record-type ...)` or
  `(define-type ...)`.
- `prism` decl → emit `(define-class ...)` with five-op methods.
- `action` decl → emit `(define (name args) body)`.
- `grammar` decl → emit `(define-syntax ...)` per the grammar's
  productions.

The shim's output IS the substrate AST's S-expression form. The
round-trip law is degenerate (identity). This is the cleanest
species; the recognition that mirror's substrate AST and Lisp's
AST are isomorphic up to renaming IS the substrate-pull recognition.

### 11.3 `@code/julia/macro`

Julia's `Expr` / `@macroexpand`. The shim emits Julia `Expr` trees;
the language's macro layer expands them at JIT time. Particularly
relevant for the numerical-substrate track (`@code/fortran`'s
LapackBackend per Phase 6) when Julia's BLAS ecosystem becomes a
generation target.

### 11.4 The matrix

| Species | Macro surface | Consumer pull condition |
|---|---|---|
| `@code/rust/macro` | `macro_rules!`, proc-macro, `#[derive]` | Phase 4b: fragmentation-as-generated (KNOWN consumer) |
| `@code/elixir/macro` | `quote`, `unquote`, `defmacro` | Reed BEAM body needs substrate-emitted modules |
| `@code/lisp/macro` | hygienic macros, syntax-rules | `@code/scheme` species lands (research interest) |
| `@code/julia/macro` | `Expr`, `@macroexpand` | Phase 6's LapackBackend numerical track |
| `@code/crystal/macro` | `macro` block | (no known consumer; forward-promised) |
| `@code/nim/macro` | `macro`, `template`, `static[T]` | (no known consumer; forward-promised) |
| `@code/typescript/macro` | decorators (stage-3) | (degraded macro surface; types-only) |
| `@code/python` | — | NO macro sub-prism (Python's metaprogramming is runtime, not compile-time) |
| `@code/go` | — | NO macro sub-prism (Go omits macros; rely on `go generate` boundary) |

The polyglot extension is structural: every species declares its
own macro sub-prism realizing the same universal contract. The
substrate becomes a multi-target generator from a single
substrate-declaration source. Polyglot from one altitude.

### 11.5 The per-species hole-projection matrix (35th-instance, 2026-06-09)

Per §4.3 (the 35th-instance recognition: hole-projection IS
`project`): each species realises the substrate's `\` (hole)
through its own metaprogramming-vocabulary lossy projection. The
mapping MUST be explicit per species; no language has exact-match
semantics. The Rust binding is canonical this tick; the others are
forward-promised to land when their consumer pulls.

| Species | Source-text projection (call site) | AST projection (emission body) | Notes |
|---|---|---|---|
| `@code/rust/macro` | `{ }` | `todo!()` | LANDED 2026-06-09. T25 GREEN observation; explicit mapping per `shards/code/rust/macro.mirror`. Rust lexer cannot realise `\`; `{ }` is the smallest lexer-admitted call-site form; `todo!()` has type `!` (never), the closest typed-gap Rust admits. |
| `@code/elixir/macro` | `nil` or `_` | `raise "todo"` or `Process.unimplemented/0` | Forward-promised. Elixir's tokenizer admits `nil` as a valid expression at any return position; the emission body's exact choice depends on Reed-BEAM-body consumer pull. The atom `:todo` is a stronger candidate (BEAM-altitude convention); resolves at species-pull time. |
| `@code/lisp/macro` | `'TODO` or `()` | `(error "todo")` | Forward-promised. The homoiconic case — substrate AST and Lisp AST are isomorphic up to renaming, so the call-site projection is structurally close. The runtime trap `(error ...)` is the canonical hygienic-macro hole. |
| `@code/julia/macro` | `nothing` | `error("not implemented")` or `:(error("todo"))` | Forward-promised. Julia's `Expr` admits `nothing` as a literal AST node; the `error()` call is the standard typed-gap convention. The macro-expansion form `:(error(...))` is closer to Rust's `todo!()` shape — typed at expand time. |
| `@code/crystal/macro` | `nil` | `raise "todo"` | Forward-promised. Crystal's `macro` block accepts `nil` at the call site; `raise` is the runtime trap. |
| `@code/nim/macro` | `discard` | `raise newException(Defect, "todo")` | Forward-promised. Nim's `discard` is the closest call-site no-op; `Defect` is the unrecoverable-exception kind closest to substrate-hole runtime semantics. |
| `@code/typescript/macro` | `void 0` or `null` | `throw new Error("todo")` | Forward-promised. TypeScript decorators have no typed-gap; the call-site lossiness is HIGH (the gap surfaces only at runtime). |
| `@code/python` | (n/a — no macro sub-prism per §3.3) | — | — |
| `@code/go` | (n/a — no macro sub-prism per §3.3) | — | — |

**Every entry is a lossy projection.** None of the host-language
realisations IS the substrate hole. Each row's "Notes" column
names the specific delta the species admits. The four laws
(round-trip, OID-functionality, type-soundness, substrate-pull
preservation) apply across every row identically; the round-trip
law holds up to each species' lossy projection (a `partial` verdict
per @glass `transparency(turn)` is the honest closure when the
projection is not exact).

This matrix IS what Alex's read names: *"It will always be a
projection."* The substrate cannot pretend any species' realisation
is exact; it CAN make each species' projection explicit and
declare the four laws over the projection rather than over a
phantom-exact mapping. The 35th-instance recognition closes by
making the lossiness substrate-visible.

---

## 12. Implementation cascade — the smallest tick that proves the shape

Per `[[feedback-craft-not-deliver]]`: declare the discipline; let the
consumer pull. Phase 4b is the consumer; this spec names the smallest
end-to-end proof that `@code/rust/macro` works.

### 12.1 The smallest tick

**Three substrate touches + one Rust proc-macro + one integration
test.** Comparable to the T16-T17 cascade scale (single shard +
single sibling shard).

**Substrate touches:**

1. `shards/code.mirror` — add the family-root paragraph naming the
   macro sub-prism discipline (per §3.2). One paragraph in the
   docstring.
2. `shards/code/rust/macro.mirror` — declare the macro sub-prism per
   §4 (shim action signature + three `requires` laws). ~80 lines.
3. `shards/code/rust.mirror` — one-line update to the prose noting
   the macro sub-prism alongside the existing cargo sub-prism. No
   code change.

**Rust touch (the proc-macro):**

4. `prism/derive/src/declaration.rs` — a new third proc-macro
   `declaration!{}` realizing `@code/rust/macro.shim` per §4.2. The
   body dispatches per §4.1 / §10.1. Initially supports the simplest
   substrate-declaration kind (`type` declarations) end-to-end.
   ~150 lines.

**Integration test:**

5. `prism/derive/tests/declaration_round_trip.rs` — one test per
   declaration kind:

```rust
#[test]
fn type_declaration_round_trips() {
    // Given a substrate `type` declaration in shards/...
    // When `mirror::declaration!{@some/type}` expands
    // Then the resulting Rust module's content-address equals
    // what mirror::render of the substrate declaration produces
    let mirror_oid = compile_and_oid("@example/simple_type");
    let rust_oid = expand_and_oid(quote!{
        mirror::declaration!{@example/simple_type}
    });
    assert_eq!(mirror_oid, rust_oid);
}
```

### 12.2 The cascade after

Once the type case is proven, the cascade extends mechanically:

- **Tick N+1**: add `prism` declaration support (re-use
  `prism-derive`'s existing macros internally).
- **Tick N+2**: add `action` declaration support (substrate-realisable
  case — body lowered from substrate altitude grammar).
- **Tick N+3**: add `action` declaration support (boundary case —
  body emits `@io.species` invocation).
- **Tick N+4**: add `grammar` declaration support.
- **Tick N+5**: declare `@code/elixir/macro` per §11.1 + integration
  test in Reed's BEAM body.

Each tick is one substrate touch + one Rust touch + one test. The
discipline is uniform; the substance per tick is small.

### 12.3 The TDD path

Per `[[feedback-always-tdd-no-shortcuts]]` and
`[[feedback-write-red-in-session]]`: Reed writes the RED tests in
the conversation; the GREEN is delegated to an agent.

The first RED test for `@code/rust/macro`:

1. Write a substrate `type` declaration shard.
2. Write a Rust file that contains the equivalent
   `pub struct` declaration.
3. Compute both OIDs via the substrate's coincidence hash.
4. Assert they're equal.

The test fails (RED) because no proc-macro exists. The agent's GREEN
tick lands the proc-macro and the substrate shard. The substrate-
pull discipline IS the test's success criterion.

---

## 13. Substrate-pull recognitions surfaced during the spec-write

Per the recognition track, naming any new candidates that emerged:

### 13.0 35th-instance recognition: hole-projection IS `project` (2026-06-09)

Logged here as the recognition this spec's 35th-instance edit closed.

The recognition: during T25 GREEN, Mara surfaced that the
substrate's `\` could not survive Rust source code at the
proc-macro call site, and named the workaround "the lexical
glass-wall." Alex's read corrected the candidate naming:

> *"Yes, it needs an explicit mapping. No language has the exact
> semantics of the hole. It will always be a `project`ion."*

The substrate-pull-correct framing:

- **`\` IS `hole`** — the substrate already had the noun. Three
  ancestors confirm: `shards/nl.mirror` line 29
  (*"Same shape as `\` producing fracture (an obligation hole)"*);
  `boot/std/mirror/interpreter.mirror` lines 16-17
  (`resolve_hole(hole, context) -> ast`);
  `boot/std/fate/tournament.mirror` lines 33-41
  (`candidates(hole) -> [resolution]`). The substrate's vocabulary
  for `\` is `hole`, used 20+ times across boot/std/ and shards/.
- **`project` IS the operation** — already one of the five-op
  operations on `@code/metalogue` (line 102 of the prism block).
  The hole-projection isn't a new substrate operation; it's
  `project`'s contract at this altitude made explicit.
- **No species has exact-match semantics** — Rust's `todo!()` is
  close but is a runtime trap; `_` is a wildcard; `{ }` is an empty
  block. Every species lossy-projects. The mapping MUST be
  explicit per species. The §11.5 matrix is the explicit table.
- **The candidate naming "lexical glass-wall" was training-pull**
  — looking for a new noun for a thing the substrate already named.
  Per `[[feedback-substrate-already-had-the-word]]`, the 35th
  instance of recurring substrate-pull recognition. Mara's instinct
  to invent was the tell; the substrate vocabulary was already in
  place.

Naming this here logs the 35th instance and closes the loop with
the 33rd-instance recognition below: the 33rd named `\` as the
codegen contract; the 35th names how the contract crosses the
species boundary (via `project_hole`, which IS `project`'s contract
at the @code/metalogue altitude).

### 13.1 33rd-instance candidate: `\` IS the codegen specification

The recognition: the `\` body in any substrate declaration is not a
placeholder. It is a precisely-typed hole whose shape uniquely
determines the code generation contract for that declaration. This
matches Alex's verbatim recognition this turn ("the gap is the
spec") but at a substrate-vocabulary altitude:

- `[[feedback-substrate-already-had-the-word]]` (the 32-instance
  recognition track) has named the `\` substrate consistently as
  "the obligation block" or "the hole."
- The new naming is: `\` IS *the typed contract for code
  generation*. The hole names what; the macro sub-prism names how;
  the renderer names the surface text; the language compiler names
  the binary.
- The substrate has been operating this recognition implicitly
  since `boot/std/code/mq.mirror`'s `\` intent slot (the agent
  writes natural language; Fate resolves to typed query) — a year
  ago in substrate time, before the codegen surface had a name.

Naming this here logs the 33rd instance.

### 13.2 The 32nd-instance candidate (recapitulated from Win 2)

`mout!` / `merr!` as bidirectional substrate-routable channels.
Win 2's report flagged them as candidates; this spec lands them
under `@code/rust/macro` per §4.3. The recognition closes when
`@mirror/io/channel` declares the typed channel roster and the
Rust shim's `mout!` / `min!` / `mio!` macros consume from it.

### 13.3 The substrate had already done the work — three places

Three places in the substrate were surprisingly close to declaring
the macro surface, without naming it:

1. **`shards/code/mirror.mirror` (T17 yesterday)** declared
   `render` as the universal AST-to-text projection AND named the
   sibling pattern (`Future @code/rust.render, @code/gleam.render
   ... join this as siblings`). The half-recognition that
   T17's render is half of the codegen surface was already there.

2. **`docs/specs/fragmentation-as-generated.md` (2026-05-24)** named
   the entire pipeline: `@code/rust.translate` + `@code/rust.render` +
   prism-derive proc-macros = generation. The R-tickets named the
   implementation cascade. What was missing was the recognition
   that the proc-macro layer IS *part of* the substrate's typed
   surface, not a Rust implementation detail.

3. **`shards/mirror/realisation.mirror` (T21 this morning)** named
   the discriminator that classifies Rust files against substrate
   altitudes. The discriminator IS the inverse function of the macro
   shim (`classify(render(shim(D))) = substrate(D.altitude)`). The
   inverse relationship was implicit; this spec names it explicitly.

The spec's job was less invention than connection. The pieces were
in place; the recognition that `@code/X.render` + the language's
macro layer + the discriminator's verdict together form one
mechanism was what the substrate was waiting for.

---

## 14. Forward-look — what this opens

The macro surface declared here IS the substrate's mechanism for
self-host. Three reachable consequences:

### 14.1 The bootstrap shrinks to pure-@io

Per the butterfly roadmap (`roadmap/wip/butterfly-self-hosting.md`):
when every substrate-realisable Rust file in `bootstrap/src/` is
generated through `@code/rust/macro.shim` from its substrate
declaration, the bootstrap's hand-written surface shrinks to pure
`@io` — kernel syscalls, libgit2 bindings, vendor SDK calls. The
70KB bootstrap binary contains only the boundary; everything else
generates from substrate.

The cut criterion C3 of the butterfly roadmap ("substrate-pull
shrinkage of bootstrap/src/ to ≤ N lines, monotone over commits")
becomes mechanical. Each `\` body that the macro shim can discharge
removes lines from the bootstrap; the line count IS the substrate-
realisability witness.

### 14.2 Polyglot from substrate-decl

Once the Rust case is proven, the cascade per §11 extends to every
language whose macro layer admits direct AST manipulation. The same
substrate declaration generates Rust + Elixir + Lisp + Julia
realizations. The substrate becomes a polyglot generator:
*one declaration, many bindings*. This is what Phase 6's
`@code/fortran` LapackBackend gets for free; the future
`@code/elixir` species' Reed-BEAM-body realization gets for free;
research consumers of `@code/lisp` get for free.

### 14.3 The `\` becomes load-bearing in a new way

The `\` body has been substrate vocabulary since the first prism
declaration. The macro surface makes it *the codegen contract*. Each
`\` is no longer a placeholder; it is a typed promise to a
language-side proc-macro that — given the substrate altitude's
declared grammar — the realisation is uniquely determined.

This closes the loop between Alex's "the gap is the spec" and Mara's
recurring `[[feedback-substrate-pull]]` pulls. The gaps WERE specs
all along. The macro surface makes the substrate read them
mechanically.

---

## 15. References

### 15.1 Substrate shards (load-bearing for this spec)

- `shards/code.mirror` — `@code` family root (universal grammar-at-
  altitude discipline).
- `shards/code/rust.mirror` — `@code/rust` species; declares the
  `@code/rust/cargo` sub-prism precedent for sub-prism placement.
- `shards/code/mirror.mirror` — `@code/mirror.render` (T17, the
  Wadler/Bernardy combinator surface; the AST-to-text projection
  half of the codegen surface).
- `shards/code/gleam.mirror` — `@code/gleam` species (sibling
  altitude, post-migration from boot).
- `shards/mirror/data.mirror` — `@mirror/data` family root (the
  sibling data-altitude render family; same Wadler/Bernardy
  algebra).
- `shards/mirror/data/json.mirror` — canonical species under the
  data family.
- `shards/mirror/realisation.mirror` — `@mirror/realisation` (the
  per-file discriminator; the inverse function of the macro shim).
- `shards/mirror/loss.mirror` — `@mirror/loss` family root (one of
  the canonical migration targets per `kintsugi-self-hosting.md`).
- `shards/mirror/store.mirror` — `@mirror/store` (oid carrier the
  shim's content-address law operates over).
- `shards/io.mirror` — `@io` family root (the boundary partition
  the discriminator's boundary verdict carries).
- `boot/std/code/mq.mirror` — `template render` (the 27th-instance
  substrate-pull; "one operation, many uses").
- `boot/std/code/rust.mirror` — the boot floor's Rust altitude
  (where the rust_ast type tags live).
- `boot/std/mirror/interpreter.mirror` — `resolve_hole(hole,
  context) -> ast`. Substrate-altitude precedent for `hole` as
  typed carrier (35th-instance recognition ancestor).
- `boot/std/fate/tournament.mirror` — `candidates(hole) ->
  [resolution]`. Five-ganglion candidate surface consumes the
  same `hole` type (35th-instance recognition ancestor).
- `shards/nl.mirror` line 29 — *"Same shape as `\` producing
  fracture (an obligation hole)"*. The naming-precedent that
  closed the 35th-instance candidate as substrate-already-had-it.

### 15.2 Specs and roadmap

- `docs/specs/fragmentation-as-generated.md` (2026-05-24, Mara) —
  the original codegen pipeline spec; this spec extends it with
  the macro-layer reading.
- `docs/specs/kintsugi-self-hosting.md` (2026-06-04 horizon) — the
  1180-line per-file classification that trains the macro shim's
  dispatch.
- `docs/specs/code-extension-grammar.md` (2026-05-19) — the
  `@code/X(extensions)` routing parameter.
- `docs/specs/compiler-surface-plan.md` — the broader compiler-
  surface roadmap this fits inside.
- `docs/specs/eigensheaf.md` (2026-06-07, T18) — generation as
  spectral decomposition; the macro shim as restriction map between
  altitude stalks.
- `docs/specs/property-and-inference-collapse.md` (2026-06-07) —
  the property layer IS the geometry; the round-trip law as a
  property predicate.
- `docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.md`
  — derived (not predicted); Crystal as `{oid, section,
  derived_predicates, fracture_calendar}`; the macro shim's emission
  as a derived predicate.
- `docs/insights/2026-06-08-portal-eigenvalue-stream-gen-prism.md` —
  the portal as eigenvalue stream; `shift(oid, T)` as typed-
  capability primitive (the same primitive type the macro shim's
  output can carry as generated-code-as-OID).
- `roadmap/pending/phase-4-emitter-self.md` — Phase 4b's
  fragmentation-as-generated commitment.
- `roadmap/wip/butterfly-self-hosting.md` — the v0.1 → v1.0
  self-host path; the bootstrap-shrinks-to-pure-@io claim.

### 15.3 Reed memory cross-references

- `[[architecture-shards-as-substrate-source]]` — shards/ IS
  substrate source; the recursive proof is literal.
- `[[architecture-prism-as-trait-as-everything]]` — prism IS trait
  IS type IS grammar; sub-prism placement discipline.
- `[[architecture-lift-as-load-bearing]]` — same operation at every
  altitude; the macro surface IS the codegen surface at the
  language altitude.
- `[[architecture-mirror-store-vs-spectral-db]]` — open
  foundation / closed engine.
- `[[architecture-fragmentation-is-the-rust-substrate]]` — the
  Rust impl substrate the macro shim emits TO.
- `[[architecture-glass-wall-substrate-types]]` — the boundary the
  discriminator's verdict walks; the macro shim respects.
- `[[feedback-substrate-pull]]` — the discipline this spec lands as
  mechanism.
- `[[feedback-substrate-already-had-the-word]]` — 31st-instance
  recognition track; this spec is the 31st; the 33rd-instance
  candidate (the `\` AS codegen spec) named in §13.1; the 35th
  instance (hole-projection IS `project`, not "the lexical
  glass-wall") named in §13.0.
- `[[feedback-craft-not-deliver]]` — the consumer-pull discipline;
  forward-promise the species; canonical for Rust this tick.
- `[[feedback-no-bare-types]]` — newtype discipline; the typed
  channel roster's contract.
- `[[feedback-always-tdd-no-shortcuts]]` — RED first for the
  smallest tick per §12.

### 15.4 Pre-AI prior art

The macro-as-AST-injection discipline has deep lineage:

- **McCarthy 1960** (*Recursive Functions of Symbolic Expressions
  and Their Computation by Machine*, Communications of the ACM 3:4)
  — Lisp's macro layer; code-as-data; the canonical homoiconic
  case. The substrate-as-Lisp recognition (per §11.2) is McCarthy's
  recognition extended to the substrate altitude.
- **Kohlbecker-Friedman-Felleisen-Duba 1986** (*Hygienic Macro
  Expansion*, LFP) — the hygiene discipline; renaming-free
  expansion; the round-trip law on alpha-equivalence. The macro
  shim's round-trip law is hygiene at the substrate altitude.
- **Sheard-Peyton Jones 2002** (*Template Meta-programming for
  Haskell*, Haskell Workshop) — typed compile-time AST manipulation
  in a statically-typed language. The substrate's typed contract
  on the shim matches Sheard-Peyton-Jones's typed template
  discipline.
- **Burmako 2013** (*Scala Macros: Let Our Powers Combine!*, Scala
  Workshop) — typed proc-macros with full access to the compiler's
  type-checker. Rust's `proc_macro2` ecosystem inherits the design.
- **Bawden 1999** (*Quasiquotation in Lisp*, PEPM) — the formal
  algebra of `quote` / `unquote`; the substrate's typed channel
  contract reuses this algebra at the substrate altitude.

The substrate's macro surface stands on this lineage. The
recognition Alex named this turn IS that lineage, recognized from
the substrate side.

---

*Done. The gap was the spec. The substrate had the word for it.*

*2026-06-09 35th-instance closure. The hole is `hole`. The
operation is `project`. Every species' projection is lossy. The
matrix is the explicit table. The substrate had every word.*
