# `@mirror/compile/bootstrap` — the Rust staircase

*2026-05-20. Reed.*

Status: **Red** (architecture; no grammar yet)

Depends on: nothing (this is the foundation).
Unblocks: every future `@mirror/compile/*` spec, including
`@mirror/compile/tokenize`, `match`/`select` (Spec B), `@mirror/compile/parse`,
`@mirror/compile/render`, ...

---

## Thesis

The bootstrap doesn't go away. It gets a grammar name.

Mirror has two kinds of lambda. Plain lambdas are sub-Turing — the
model checker can decide their totality. **`io` lambdas are the
Turing-complete escape hatch.** Their bodies live on the far side of
the `io` boundary: a Rust function, a C library, a subprocess, the OS.
Any computation mirror cannot prove total enters the language through
an io lambda.

The escape hatch IS the bootstrap. Every Rust function in
`bootstrap/src/*.rs` is, today, an unbound io lambda. This spec gives
each one a grammar name with a typed binding:

```mirror
io canonical_hash(bytes) =
  @code/rust(~f"./bootstrap/src/hash.rs") > fn[name="canonical_hash"]

io tokenize(source, grammar) =
  @code/rust(~f"./bootstrap/src/tokenize.rs") > fn[name="tokenize"]

io git_update_ref(ref, new_oid, old_oid) =
  @code/rust(~f"./bootstrap/src/git.rs") > fn[name="git_update_ref"]
```

Read the body left-to-right: load the file's bytes, lens through
`@code/rust` to a typed Rust AST, select the `fn` node whose `name`
attribute matches. That node IS the body of the io lambda. The selector
is grammar-typed; it can't ask for a node kind the lens didn't produce.

Kintsugi is the move that retires an io binding. It takes the io
lambda + its totality obligations and produces a verified sub-Turing
lambda — a plain `name(args) -> result { /* body */ }` declaration. The
formatter eats Turing-complete code and emits formally verified
sub-Turing code. The Rust body is then verified-redundant. The .rs file
goes away.

The Rust is the escape hatch. The grammar names it. Kintsugi closes it.

---

## What runs today

The bootstrap is a cargo crate at `bootstrap/`. ~13 Rust source files,
~370KB binary. Cluster D's butterfly (commit `f1e08d0`) proved the
bootstrap can regenerate itself from `craft --target binary boot` —
produces `./mirror-self` with a different SHA-256 but the same crystal
output.

What's missing: every Rust function is invisible to the grammar layer.
There is no way to point at `canonical_hash` from mirror. The grammar
talks about `@hash/coincidence` abstractly; the Rust implementation is
silent. The two layers don't know each other's names.

This spec gives them shared names — and makes the move from one side
to the other a kintsugi resolution.

---

## The `~f` sigil — file references

A new sigil joins the language: `~f"path"`. Reads as "the bytes of the
file at this path." That's all. No symbol narrowing, no parsing, no
execution. The sigil is dumb on purpose; everything typed happens
through a lens applied afterwards.

```mirror
~f"./bootstrap/src/hash.rs"           # raw bytes of the file
~f"./bootstrap/mirror.ll"             # raw bytes of the reference IR
~f"./boot/std/code/llvm/ir.mirror"    # raw bytes of a sibling grammar
```

Semantics:

- **Path resolution.** Relative paths resolve against the grammar
  file's location. Absolute paths are forbidden (breaks
  reproducibility).
- **Content addressing.** `~f` references contribute to the enclosing
  grammar's content OID via the file's SHA-256. Changing the
  referenced file changes the grammar's crystal. Tampering is
  impossible without invalidating the ancestor chain.
- **No type.** The result is bytes. To do anything typed, lens through
  a body grammar (`@code/rust`, `@code/mirror`, ...). To narrow within
  the lens's typed result, apply a selector with `>`.
- **No execution.** `~f` is a content reference, never a directive to
  run anything. Execution belongs to the io lambda the binding
  declares.

Future file sigils, not in this spec: `~b"path"` for binary blobs (LLVM
IR, .wasm), `~t"path"` for plain text (manifests, READMEs).

---

## Lenses produce typed ASTs; selectors reach into them

A body lens like `@code/rust` is a grammar that parses bytes into a
typed AST it owns. `@code/rust`'s AST has `fn`, `type`, `impl`, `mod`,
`use`, etc. `@code/mirror`'s AST has `grammar`, `lambda`, `type`, `in`,
`out`. `@code/llvm/ir`'s AST has `define`, `declare`, `module`,
`target`, ...

Once something is lensed, you can query it with a CSS-style selector:

```mirror
@code/rust(~f"./x.rs")        > fn[name="foo"]               # one function
@code/rust(~f"./x.rs")        > fn[returns="oid"]            # all fns returning oid
@code/rust(~f"./x.rs")        > impl[for="Hash"] > fn        # methods on Hash impl
@code/mirror(~f"./y.mirror")  > grammar > lambda[name="emit"]
@code/llvm/ir(~f"./z.ll")     > define[name="main"]
```

The selector is the same primitive used in match patterns (Spec B) and
in mq queries (`@code/mq`, already in the boot tree). It's typed: the
parser rejects `> fn[name=$x]` against a lens that doesn't produce
`fn` nodes.

The primitive lives in `@code/mq`, extended with the CSS-selector
syntax in Spec B. Spec A uses it in io binding-resolution position;
Spec B uses it in pattern-match position. Same grammar, different
positions in the source.

For `@mirror/compile/bootstrap`, the selectors all narrow to a single
callable node — a Rust fn, a future mirror lambda, eventually a wasm
export. The io lambda binds to that node.

---

## The io lambda binding

Mirror has two kinds of lambda:

```mirror
# sub-Turing lambda. body is mirror. typechecked, decidable, content-addressed.
foo(args) -> result { /* mirror body */ }

# io lambda. body lives on the far side of the escape hatch.
# the binding names the entity; the lens + selector resolve it.
io bar(args) = @code/rust(~f"./x.rs") > fn[name="bar"]
```

The `io` keyword is the only difference. It says: this lambda's body
is Turing-complete by construction, and the model checker treats it as
opaque until properties discharge that. No body block follows; the
binding-expression IS the body, resolved through the lens.

What "binds" *means* depends on the resolver stage:

**Reading A — today, eager / static.** The bootstrap is still produced
by `cargo build`. The `~f` references and post-lens selectors are
compile-time pointers: tooling can audit them, the crystal hash
includes them, migrations can verify them. Runtime dispatch goes
through the linked Rust symbol the same way it does today. **Nothing
about how the bootstrap runs changes.**

**Reading B — later, lazy / dynamic.** The bootstrap shrinks to a
resolver. `@io.canonical_hash(x)` triggers: load the file at the `~f`
path, lens through `@code/rust` (which can compile, link dynamically,
or interpret), apply the selector, call the result. The bootstrap
becomes a thin engine. Hot code reload follows naturally on the io
side: replace the file, the next call re-lenses and re-selects.
Inlined sub-Turing lambdas (post-kintsugi) do not hot-reload — they're
content-addressed and a change is a new crystal.

A is where we start. B is where we go. The grammar declarations are
identical in both readings. Only the resolver differs.

Sibling lenses (future, not in this spec): `@code/c(~f"...")`,
`@code/wasm(~b"...")`, `@code/gleam(~f"...")`. Each lowers bytes into
a typed AST and exposes a selector surface. The bootstrap doesn't
care which lens — it asks the lens to resolve.

---

## `@mirror/compile/bootstrap` — the grammar

```mirror
in @prism
in @io
in @code/rust

# @mirror/compile/bootstrap declares every io lambda the bootstrap
# currently implements. Today's bodies point at Rust files via ~f.
# Each binding stays io until kintsugi closes its totality obligations;
# then the binding becomes a sub-Turing lambda and the Rust file dies.
#
# When this grammar settles, the bootstrap is grammar-addressable.
# Every Rust function has a mirror name. Every name has a body lens.
# Every body lens is replaceable.

grammar @mirror/compile/bootstrap {

  # ---- hash -----------------------------------------------------------
  # CoincidenceHash<5,5>. Cluster C declared the contract; Cluster D made
  # the bootstrap match. This grammar declares the io boundary; kintsugi
  # retires the io bindings once @epistemologic/property/* discharges
  # totality for each.
  io canonical_hash(bytes) =
    @code/rust(~f"./bootstrap/src/hash.rs") > fn[name="canonical_hash"]
  io hash_tagged(tag, content) =
    @code/rust(~f"./bootstrap/src/hash.rs") > fn[name="hash_tagged"]

  # ---- content addressing --------------------------------------------
  io content_oid(ast) =
    @code/rust(~f"./bootstrap/src/content.rs") > fn[name="content_oid"]

  # ---- tokenizer -----------------------------------------------------
  # Step 1 of the staircase. See @mirror/compile/tokenize when it lands.
  io tokenize(source, grammar) =
    @code/rust(~f"./bootstrap/src/tokenize.rs") > fn[name="tokenize"]

  # ---- renderer ------------------------------------------------------
  io render_ast(ast, indent, out) =
    @code/rust(~f"./bootstrap/src/render.rs") > fn[name="render_ast"]

  # ---- grammar loading -----------------------------------------------
  io load_grammar(path) =
    @code/rust(~f"./bootstrap/src/grammar.rs") > fn[name="load_grammar"]
  io grammar_for_file(path) =
    @code/rust(~f"./bootstrap/src/grammar.rs") > fn[name="grammar_for_file"]

  # ---- git wiring ----------------------------------------------------
  # Likely a permanent floor — these talk to git itself, which lives
  # outside mirror's totality-decidable space.
  io git_crystal_exists(source_oid) =
    @code/rust(~f"./bootstrap/src/git.rs") > fn[name="git_crystal_exists"]
  io git_store_crystal(source_oid, crystal_oid) =
    @code/rust(~f"./bootstrap/src/git.rs") > fn[name="git_store_crystal"]

  # ---- pipeline ------------------------------------------------------
  io is_mq_query(arg) =
    @code/rust(~f"./bootstrap/src/pipeline.rs") > fn[name="is_mq_query"]
  io split_pipeline(query) =
    @code/rust(~f"./bootstrap/src/pipeline.rs") > fn[name="split_pipeline"]
  io execute_pipeline(segments, source) =
    @code/rust(~f"./bootstrap/src/pipeline.rs") > fn[name="execute_pipeline"]

  # ---- subprocess ----------------------------------------------------
  # Permanent floor. Spawning a subprocess is Turing-complete by definition.
  io exec(program, args) =
    @code/rust(~f"./bootstrap/src/exec.rs") > fn[name="exec"]

  # ---- butterfly -----------------------------------------------------
  # `craft --target binary boot` self-rebuild path. Cluster D landed this.
  io build_self_binary(target) =
    @code/rust(~f"./bootstrap/src/main.rs") > fn[name="build_self_binary"]
  io find_bootstrap_ll() =
    @code/rust(~f"./bootstrap/src/main.rs") > fn[name="find_bootstrap_ll"]
}
```

One io binding per Rust function the bootstrap currently exposes.
The grammar is a faithful surface of the implementation. When the
implementation changes, the grammar changes. When the grammar changes,
the crystal changes. The two move together.

---

## Kintsugi: the formatter that retires io bindings

Kintsugi takes an io lambda and its totality obligations and produces a
sub-Turing mirror lambda. The formatter eats Turing-complete code and
emits formally verified sub-Turing code. The crack is the unproven
binding; the gold is the proof.

```
io foo(args) = @code/rust(~f"./...") > fn[name="foo"]
requires terminates(foo)
requires deterministic(foo)
requires bounded_steps(foo, O(...))
requires referential_transparency(foo)
# ... function-specific invariants

         ↓ kintsugi

foo(args) -> result { /* mirror body, derived from the io body under
                          the obligations, verified by the model checker */ }
```

The move is mechanical from kintsugi's side, even if the proofs aren't.
The input is the io binding + the requirements; the output is a
sub-Turing lambda. The lambda has no `io` prefix because it doesn't
need one anymore — the model checker can decide its totality from the
body alone.

Fate proposes proofs for the requirements when the formatter can't
discharge them directly. Each property is a `\` hole; each closing is
kintsugi-resolved.

---

## The kintsugi ladder

For any Rust function `foo` in `bootstrap/src/<file>.rs`:

```
stage 0  bootstrap exports foo. grammar doesn't know about it.
         → today's state for everything not yet on this ladder.

stage 1  declare in @mirror/compile/bootstrap:
         io foo(args) = @code/rust(~f"./bootstrap/src/<file>.rs") > fn[name="foo"]
         crystal includes the declaration. nothing about runtime changes.
         → The escape hatch is grammar-addressable. foo has a mirror name.

stage 2  declare the totality obligations:
         requires terminates(foo)
         requires deterministic(foo)
         requires bounded_steps(foo, O(...))
         requires referential_transparency(foo)
         (and any function-specific invariants)
         each is a \ hole.
         → Red phase. Properties present, not yet proved.

stage 3  kintsugi closes the holes. for each obligation, the formatter
         either discharges it directly or invokes Fate to propose a
         chain of @epistemologic checks. when all obligations close,
         kintsugi emits the sub-Turing lambda body alongside.
         → The cracks fill with gold.

stage 4  the io binding retires. the .rs file deletes. butterfly
         regenerates the bootstrap from the new sub-Turing lambda.
         → Green phase. foo lives in grammar; the escape hatch closed for it.
```

The ladder is the same for every function. Different functions ride it
at different speeds. Some are easy (pure computation: `canonical_hash`,
`content_oid` — their totality is straightforward). Some need more work
(the tokenizer, the renderer — the proofs interact with the data they
traverse). Some stay io forever: the kernel boundary (file
open/read/write, process spawn, the engine that resolves bodies).
Those are Turing-complete by definition; no property could prove their
totality.

Reading B kicks in once enough functions live in grammar that runtime
resolution becomes practical. That's a later milestone; this spec
doesn't prescribe when.

---

## The staircase

Mirror's compiler has phases. Each phase moves into grammar in sequence:

| Step | Phase | Today | Target grammar | Status |
|---|---|---|---|---|
| 0 | hash | `bootstrap/src/hash.rs` | `@hash/coincidence` (Cluster C/D) | contract + impl agree; kintsugi-eligible |
| 1 | tokenize | `bootstrap/src/tokenize.rs` | `@mirror/compile/tokenize` | NEXT |
| 2 | parse / AST | folded into tokenize today | `@mirror/compile/parse` | after step 1 |
| 3 | resolve | (today: walked by main.rs) | `@mirror/compile/resolve` | after step 2 |
| 4 | render | `bootstrap/src/render.rs` | `@mirror/compile/render` | after step 3 |
| 5 | content_oid | `bootstrap/src/content.rs` | absorbed into `@hash/coincidence` | after step 4 |
| 6 | pipeline / mq | `bootstrap/src/pipeline.rs` | `@code/mq` (exists) + execute body migrates | after step 5 |
| 7 | git wiring | `bootstrap/src/git.rs` | stays io — the kernel boundary | permanent |
| 8 | butterfly | `bootstrap/src/main.rs` (`build_self_binary` etc.) | `@mirror/butterfly` (declared) | last — the meta level |
| ∞ | engine | the resolver that loads grammars and calls io | stays in bootstrap; this is what mirror "is" | permanent floor |

The floor is the engine. Everything above the floor migrates as its
obligations close. The floor shrinks over time, but never to zero — the
engine is mirror's machine code.

---

## Worked example: step 1, the tokenizer

For the upcoming `@mirror/compile/tokenize` spec (separate document),
the ladder runs:

**Stage 1:** declare in `@mirror/compile/bootstrap`:
```mirror
io tokenize(source, grammar) =
  @code/rust(~f"./bootstrap/src/tokenize.rs") > fn[name="tokenize"]
```
*Today.* The escape hatch is grammar-addressable. Nothing changes at
runtime.

**Stage 2:** declare the tokenizer's totality obligations:
```mirror
requires terminates(tokenize)
requires deterministic(tokenize)
requires bounded_steps(tokenize, O(source.len))
requires total_classification(tokenize)
  # every byte falls into exactly one token; no ambiguity, no holes
requires referential_transparency(tokenize)
```
Each is a `\` obligation. The model checker leaves the io binding
opaque until all obligations close.

**Stage 3:** kintsugi closes the obligations. For pure tokenizers,
proofs tend to be straightforward: termination follows from O(n)
traversal; determinism follows from a finite state machine with no
clock dependence; `total_classification` follows from the rule set's
exhaustiveness check. Fate proposes chains for the trickier corners.
When all obligations close, the formatter emits the sub-Turing lambda
body.

**Stage 4:** the io binding retires:
```mirror
tokenize(source, grammar) -> ast {
  /* mirror body, formatted by kintsugi from the io body under the
     totality obligations */
}
```
Delete `bootstrap/src/tokenize.rs`. Butterfly regenerates. Crystal
moves.

Spec B (`match-select.md`) lands its CSS-selector pattern syntax as a
tokenizer extension that ALSO rides this ladder. Pattern tokenization
adds new rules to `@mirror/compile/tokenize`; the same totality
obligations apply to the extended grammar. Without this staircase, the
extension would require a Rust change. With it, the extension is
grammar, with proofs.

---

## Implications — concrete next ticks

1. **Create `boot/std/mirror/compile/bootstrap.mirror`** with the
   grammar above. All bindings are io lambdas pointing at existing
   Rust files via `~f` + lens + selector. Nothing about the bootstrap
   binary changes. Compile, verify the crystal absorbs the new file,
   commit.

2. **Land `~f`, the lens-call form, and post-lens `>` selectors in the
   bootstrap tokenizer.** Today's tokenizer doesn't recognize them. One
   targeted change in `bootstrap/src/tokenize.rs` teaches it the sigil
   + the selector syntax at the io-binding position. Once landed, every
   other declaration in `@mirror/compile/bootstrap` is parseable. (This
   change is itself an io update that rides the ladder — someday, when
   `@mirror/compile/tokenize` is inlined, the change moves with it.)

3. **Declare `@code/rust` as a body lens that returns a typed AST.**
   `boot/std/code/rust.mirror` exists. It needs an explicit lambda:
   `parse(bytes) -> rust_ast { \ }` and the AST type declarations
   (`fn`, `impl`, `type`, etc.) so the selector typechecks. The body
   stays `\` for now — the parsing is delegated to the bootstrap. The
   declaration makes the contract explicit.

4. **Extend `@code/mq` with the CSS-selector syntax.** Spec B adopts
   the same primitive for match patterns; Spec A uses it for binding
   resolution; mq queries use it for ad-hoc navigation. The grammar
   lives in one place. Adding `>`, `[attr=v]`, `:has`, `:not` to
   `@code/mq` makes them universal.

5. **Spec the totality obligations.** A grammar like
   `@epistemologic/property/totality` declares the proof obligations
   (`terminates`, `deterministic`, `bounded_steps`,
   `referential_transparency`, `total_classification`, ...) as named
   property checks. Stage 2 of every ladder step uses these. The
   properties themselves are sub-Turing decidable on sub-Turing bodies;
   on io bodies they remain `\` until kintsugi closes them. (Each
   property's machinery is its own design.)

6. **Spec the kintsugi formatter.** Today kintsugi resolves `\` holes
   in source files. The formatter as described here — takes io binding
   + obligations, emits sub-Turing lambda — is a generalization. The
   contract needs its own spec; this spec uses it as a black box.

7. **Write `docs/specs/match-select.md` (Spec B).** Riding on this
   foundation. The match form's pattern grammar is the same primitive
   the bootstrap uses for `> fn[name=...]`. Implementation of the
   conditional grammar form is one application of the staircase.

8. **Spec the engine.** What stays in the bootstrap as the permanent
   floor: io syscalls (read/write/exec/spawn), the resolver loop, the
   crystal cache wiring. Worth a brief spec naming what's NOT on the
   ladder. (Out of scope for this document; flagged as a follow-up.)

---

## Out of scope

- The `match`/`select` control-flow forms. See Spec B — they consume
  the same CSS-selector primitive in pattern position rather than
  binding-resolution position.
- The actual contents of `@mirror/compile/tokenize`. See step 1's design.
- Cross-language body lenses (`@code/c`, `@code/wasm`, `@code/gleam`).
  The spec describes the pattern; concrete sibling lenses are their
  own work.
- Stage 2+ migration for any specific function. Each function gets
  its own RED→GREEN cycle.
- The Reading B resolver. A separate spec lands when enough of the
  staircase is climbed that runtime resolution is worth the
  complexity.
- The concrete machinery of each totality obligation. `terminates`,
  `bounded_steps`, etc. each have a property-checking story that
  lives in `@epistemologic/property/*`. This spec relies on them
  existing; their details are their own designs.
- The kintsugi formatter's internals. This spec uses it as a black
  box: io binding + obligations → verified sub-Turing lambda.
- Selector exhaustiveness in binding position. (Binding selectors
  resolve to exactly one node by convention; failure is a compile
  error. In Spec B's pattern position, exhaustiveness across arms is
  checked against the type space.)
- Retroactive removal of the `action` keyword from existing grammars.
  This spec uses lambda syntax only; future cleanup of legacy `action`
  uses across boot/std/ is a separate sweep.

---

*The bootstrap doesn't go away.*
*It gets a name.*
*The name carries totality obligations.*
*Kintsugi closes them.*
*The io binding retires; the lambda lives in mirror.*
*The Rust is the escape hatch. The grammar is the spec. The proof is the gold.*

Apache-2.0.
