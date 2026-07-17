> **DEPRECATED-FOR-RUST-REWRITE (Mara 2026-07-17):** This spec
> describes an earlier framing of the bootstrap retirement path. The
> terminal-form map for the current arc lives at
> `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
> (Mara `2519f83`), where `@kintsugi/roomba` reads the
> `kintsugi { roomba { } }` cascade block in `mirror.spec` and
> materializes `rust/` as the terminal FLOOR. Preserved here for
> archaeology of the earlier framing.

# Bootstrap retirement plan — the Rust floor sealed

*2026-05-21. Reed. Updated 2026-06-04 (Reed + Alex).*

Status: **Red** (plan only; no Rust changes, no grammar changes; this
spec sequences the substrate-pull at the implementation level)

> **2026-06-04 reframe (Reed + Alex, canonical) — the shards/ floor and
> the legacy substrate.**
>
> The substrate-pull arc has a destination clearer than this plan's
> v1 framing: **`shards/` is source of truth.** All new substrate
> lands in `shards/` (per [[prism-floor-and-the-grammar-rename]] and
> [[shard-design]]):
>
> ```
> shards/
>   glass.mirror           # @glass — types of glass wall + glass keyword
>   metalogue.mirror       # @metalogue — language's self-conversation
>                          # The metalogue IS the glass wall
>   nl.mirror              # @nl — root prism; # is the @nl primitive
>   epistemologic/         # @epistemologic/* — properties + math + silicon
>     property/path_matches_namespace.mirror   # NEW (path-namespace property)
>   mirror/                # @mirror/* — the compiler's surface
>     prism.mirror cli.mirror shatter.mirror store.mirror
>     mosaic.mirror spec.mirror au.mirror
> mirror.spec              # mirror's dogfood; uses @mirror/cli to declare CLI
> ```
>
> **legacy `boot/`** + **legacy `bootstrap/`** are this plan's targets.
> They retain shrinkage contracts: each tick below shrinks the Rust
> floor a little more, and the legacy floors retreat as `shards/` takes
> over the corresponding phase. The "shrink → retire" verdicts in this
> plan are still correct; the **destination** is shards/, not a leaner
> bootstrap. The leaner bootstrap is a way-station.
>
> **Key types/concepts that frame the destination:**
>
> - **mosaic** is the build-system prism; composes shards; settles to au.
> - **mirror.spec** is the multi-dimensional manifold; kintsugi operates
>   on it (per [[kintsugi-ci-v0.1]] post-reframe).
> - **au** is the verified-construction output, parametric over altitude
>   (`au(@code/rust)`, `au(@release)`, etc.).
> - **`.shatter`** is an OPTIONAL disk projection of `au + splinter +
>   mosaic` (per [[../shatter-spec]]); the fragmentation store is
>   canonical (per [[mirror-store]]).
> - **No deps; shards are self-contained crystals.** Splinter IS the
>   structural lockfile. Composition by OID, not name resolution.
> - **MirrorLoss is dead** (per task #126); replaced by `transparency<p>`.
> - **`<= prism` is redundant** in `prism @X { … }`; drop it everywhere.
> - **`#` → @nl term**; **`\` → fracture** per
>   [[gap-tension-tensor-substrate]] §11.
> - **Five operations:** focus, project, split, **shift** (was zoom),
>   **settle** (was refract) — operational form of Connes' (A, H, D)
>   spectral triple per [[prism-core-as-spectral-triple]] /
>   [[spectral-triple-binary]].
>
> **Self-descriptive all the way down:** `mirror.spec` uses
> `@mirror/cli` to declare what mirror's CLI looks like. The CLI
> surface that builds mirror IS the CLI surface mirror exposes. The
> substrate compiles itself by declaring itself.
>
> The Ticks 1–6 below remain the legacy-floor shrinkage plan;
> shards/ is where the substrate that replaces them lands. Treat
> "retire" verdicts as "moves to shards/" where the moved code is
> substrate-declared rather than Rust-implemented.

Depends on:
- `docs/specs/prism-core-as-spectral-triple.md` — the v1 architecture
  (the 1500-line floor; the (A, H, D) evaluator).
- `docs/specs/spectral-triple-grammar.md` — the supertrait closure and
  the four CLOSED gap resolutions in `prism-core` (commit `5d98c6e`)
  and `terni` (commit `caae5216`).
- `docs/research/embedded-and-self-hosting.md` — size projections,
  embedded ecosystem context, the 200–300KB envelope.
- `docs/specs/mirror-compile-bootstrap.md` — the `io` binding staircase
  (`@code/rust(~f"…") > fn[name="…"]`); the kintsugi retirement model.
- `bootstrap/src/spectral.rs` (commit `1076642`, branch `reed/v1-floor`)
  — the evaluator's current shape: `compose_a`, `apply_h`, `eigen_d`,
  with property tests against prism-core's verified substrate.

Unblocks:
- A next-session executor can pick up one tick at a time. Each tick's
  acceptance criteria fit a single commit.
- The `no_std` stretch becomes a falsifiable next-step proposal rather
  than a vague aspiration.

---

## Why this matters

The spectral-triple recognition (`prism-core-as-spectral-triple.md`)
named the architecture. The supertrait closure (`spectral-triple-grammar.md`
§Phase 1 audit, four CLOSED resolutions) made `prism-core` a *verified*
substrate. `bootstrap/src/spectral.rs` is the evaluator over that
substrate — real, but unwired: every public symbol is exercised only by
its own tests until the downstream retirements dispatch through it.

This plan maps the gap. Every Rust module remaining in `bootstrap/src/`
gets one of four verdicts: **retire** (replaced by spectral evaluator +
grammar), **shrink** (becomes thin wrapper), **stay** (permanent floor),
**move** (relocate to a clean IO shim). The retirements compose into
the evaluator one tick at a time, each tick a single commit's worth of
work, with smoke-OID verification between ticks.

The substrate-pull pattern (each tick shrinks the Rust floor) is the
load-bearing voice. The plan embodies it: each tick is a small,
falsifiable claim that the substrate yields a little more. The
`no_std` stretch sits at the end: not a blocker for v1, but a deeper
goal that depends on what comes before.

---

## Today's inventory

Module-by-module table. Lines measured by `wc -l` on
`reed/v1-floor`@`1076642`. Dependencies listed are *call-site*
dependencies — modules whose retirement is blocked by another.

| Module | LOC | Role | Calls into | Called by |
|---|---|---|---|---|
| `main.rs` | 776 | CLI dispatch; `cmd_compile`, `cmd_craft`, `cmd_kintsugi`; `--strict` enforcement; the kintsugi-loop scaffold; build-self-binary butterfly | ast, content, git, grammar, hash, pipeline, render, tokenize | (entry point) |
| `tokenize.rs` | 768 | Parser. Source bytes → AST. Hand-written recursive descent over grammar mappings. Six entry points: `tokenize`, `scan_items` (the 580-line core), plus helpers (`scan_brace_block`, `scan_paren_block`, `capture_io_body_end`, …) | ast, grammar | main, pipeline |
| `spectral.rs` | 618 | The (A, H, D) evaluator. `Seed<S>`, `Verdict<S>`, `apply_h`, `apply_h_content`, `compose_a`, `eigen_d` + 5×5 power-iteration solver. `#[allow(dead_code)]` until downstream retirements dispatch through it. | ast, content, prism-core, terni | (tests only, today) |
| `render.rs` | 322 | AST → bytes. Inverse of tokenize. Three entry points: `render_ast` (default), `render_ast_mirror` (canonical form), `render_ast_with_grammar` (reverse-lookup keywords) | ast, grammar | main, pipeline |
| `hash.rs` | 273 | `CoincidenceHash<5,5>`. The concrete D in matrix form. `canonical_hash`, `hash_tagged`, `encode_into_basis`, `projection_from_seed`, `projection_apply`, canonical-projection `OnceLock` cache | sha2 | main, content |
| `grammar.rs` | 232 | `.mirror` loading + keyword↔kind tables. `parse_grammar`, `load_grammar`, `grammar_for_file`, `grammar_path_for_ref`, `is_skip_word` | ast, std::fs | main, pipeline, render, tokenize |
| `pipeline.rs` | 182 | mq pipeline parser + executor. `split_pipeline`, `execute_pipeline`, `is_mq_query`, kintsugi/butterfly dispatch | ast, content, exec, grammar, render, tokenize | main |
| `ast.rs` | 140 | The State type for H: `AstKind` (10 variants), `AstNode`, `DarkSpan`, `line_col_at` | (none) | content, hash, main, pipeline, render, spectral, tokenize |
| `content.rs` | 138 | `content_oid` — D's action on AST states. Recursive walk dispatching to `hash_tagged` per kind | ast, hash | main, pipeline, spectral |
| `git.rs` | 60 | @io kernel: content-addressed storage. `git_store_crystal`, `git_crystal_exists` via `Command::new("git")` | std::process, std::fs, std::env, std::time | main |
| `exec.rs` | 29 | @io kernel: subprocess spawning. `io_exec` (used by butterfly's clang shell-out) | std::process, std::io | pipeline |

**Total bootstrap Rust:** 3538 LOC (3 LOC under the 3541 the prior
audit projected). `spectral.rs` is real but inert. The five modules
marked for retirement (`tokenize`, `render`, `content`, `pipeline`,
`grammar`) sum to 1942 LOC. The `cmd_*` block in `main.rs` and the
kintsugi-loop scaffold sum to ~500 LOC. The IO kernel
(`git.rs` + `exec.rs`) is 89 LOC.

---

## Per-module classification

The four verdicts:

- **retire** — the module's responsibility moves into `spectral.rs` +
  grammar declarations. The .rs file goes away.
- **shrink** — the module's responsibility is partly absorbed; a thin
  wrapper remains.
- **stay** — the module is the permanent floor; no retirement target.
- **move** — the module is reclassified into a separate IO crate
  (the `no_std` enabler; see stretch analysis).

### `spectral.rs` — STAY (grows)

The evaluator. Already stands on the verified `prism-core` substrate
plus `terni::Metric`. Future retirements compose into it: the AST
walker, the renderer-inverse, the kintsugi-tick stages, the
pipeline-segment executor all become specific `Prism` impls and
`apply_h` call-sites. As retirements land, this file grows from 618
LOC to ~800–1000 LOC and `#[allow(dead_code)]` is removed when
`cmd_compile` first dispatches through it.

**Acceptance:** all 11 existing property tests in
`bootstrap/src/spectral.rs#tests` keep passing; new property tests
(`compose_a_associates`, `apply_h_content_matches_content_oid`,
`eigen_d_5x5_smoke`, etc.) extend with each retirement.

### `ast.rs` — STAY

The state type for H. Each `AstNode` is a state vector; `AstKind` is
the discrete fiber tag. `DarkSpan` and `line_col_at` are the
diagnostic substrate. No retirement target — H itself doesn't retire,
it just gets used through `apply_h` instead of via hand-written
recursive walks.

**Acceptance:** signature stable across all retirement ticks; no
new public methods needed (the methods already exposed cover what
`spectral.rs` will dispatch through).

### `hash.rs` — STAY

The concrete D in matrix form. The `CoincidenceHash<5,5>` geometry is
declared in `boot/std/hash/coincidence.mirror`, but the *evaluator* of
that geometry is this Rust file. Once the spectral-triple-grammar
property `bounded_commutator` closes, `hash.rs` could in principle
retire — but the cost is paying for `eigen_d` over a 5×5 matrix to
re-derive what the byte-exact SHA-256 implementation already produces.
Not worth it for v1. Stays as the permanent concrete D.

**Acceptance:** smoke OIDs `a8312da6…` and `3ba4c79d…` byte-stable
through all retirements; boot crystal `41470e69f2…` stable; the
canonical 5×5 projection cache invariants hold (no allocation in
`canonical_hash` on the cache-hit path).

### `content.rs` — RETIRE

`content_oid` is a recursive walk dispatching per `AstKind` to
`hash_tagged`. This is *exactly* the shape of `apply_h_content` in
`spectral.rs`: D's scalar action on an AST state vector.

The retirement: a small `Prism` impl per `AstKind` that focuses on
the kind, projects to the canonical byte representation (name +
optional body + children's OIDs joined by `:`), and refracts to the
tagged hash. The 10 match arms become 10 `Prism` impls (or one
parametric impl over the kind tag, which is cleaner). The recursive
walk becomes `eh`-composition: each child's `apply_h_content` Verdict
feeds the parent's projection.

**Replacement shape.** A combinator `ContentPrism { kind: AstKind,
tag: &'static str }` whose `focus` writes the name bytes, whose
`project` walks children via `apply_h_content` and accumulates them
into the buffer, whose `settle` calls `hash_tagged(tag, buf)`. The
match-on-kind in current `content_oid` collapses to a kind-indexed
dispatch (one `[ContentPrism; 10]` table indexed by `AstKind as u8`).

**Dependencies.** None (independent — first retirement).

**Acceptance:** for every `.mirror` and `.rs` and `.ll` file in
`boot/` and `bootstrap/src/`, the new `apply_h_content` Verdict's
`Success` payload byte-equals the old `content_oid` String. Smoke
OIDs `a8312da6…` / `3ba4c79d…` byte-stable. Boot crystal
`41470e69f2…` stable. Dark count 58 across 23 files unchanged.

**Effort.** Small (< 1 session). The dispatch is mechanical; the
hash bytes are already correct in `hash.rs`.

**Binary size delta.** Net zero ± 5KB. The Prism-impl machinery has
some generic overhead; the match arms collapse into a constant table.
~140 LOC retire, replaced by ~120 LOC of Prism impls.

### `render.rs` — RETIRE

The inverse of tokenize in A. Three entry points: `render_ast`
(default), `render_ast_mirror` (canonical), `render_ast_with_grammar`
(reverse-lookup keywords through the grammar). All three follow the
same shape: recursive walk dispatching per `AstKind` to bytes-out.

The retirement: a `RenderPrism { grammar: &Grammar }` whose
`focus` reads the AST kind, whose `project` writes the
indent+keyword+name, whose `settle` walks children via
`apply_h(RenderPrism, child)` and emits the closing bytes. Output
collected via the `Holonomy` accumulator (the renderer is a pure
function of the AST, so the Verdict is always `Success(bytes)`).

**Replacement shape.** One `Prism` impl whose `State` is
`(AstNode, Vec<u8>)` and whose `Verdict` is `Success(Vec<u8>)`. The
grammar reverse-lookup table moves into a grammar declaration:
`boot/std/mirror/grammar.mirror` already lists the keyword↔kind
mappings; `render` becomes data-driven.

**Dependencies.** Retire `content.rs` first (so the AST-walk Prism
pattern is established). `render` and `content` could be retired in
either order in principle, but retiring `content` first means the
walker idiom is reusable.

**Acceptance:** for every file `f` in `boot/`, `render_ast(tokenize(f))`
(round-trip) byte-equals the current `render_ast` output. The kintsugi
formatter run (`mirror kintsugi --shatter 1 <file>` on every boot
file) produces byte-identical output before and after. Smoke OIDs
stable.

**Effort.** Medium (1 session). Three current entry points collapse
into one parametric Prism; the grammar reverse-lookup table needs
careful handling for the Rust grammar (which has reverse-lookup
collisions on `fn` ↔ `Focus` ↔ `Project`).

**Binary size delta.** ~325 LOC retire, replaced by ~150 LOC of
parametric Prism. Net negative ~10KB.

### `tokenize.rs` — RETIRE (the big one)

768 LOC. The single biggest gap. Hand-written recursive descent over
grammar mappings, with bespoke handlers for `io`/`match`/`select`
(Spec A/B), brace/paren scanning, `Dark`-region capture, and
LLVM-IR-specific body capture for `target datalayout = "…"` keyword
forms.

The retirement is the **Parser-as-Prism move** from
`prism-core-as-spectral-triple.md` §"What retires from the bootstrap":
*tokenize becomes a tree of A-elements (parser rules as combinator-data),
evaluated by apply_h*. Each scanner combinator (`seq`, `choice`,
`repeat`, `capture`, `literal`, `charset`, `brace_block`, `paren_block`,
`io_binding`, `match_arm`, `select_variant`) becomes a Prism impl.
The parser ceases to be code; it becomes a tree of combinator-data
loaded from `boot/std/mirror/grammar.mirror` (and the per-language
grammars: `code/rust.mirror`, `code/llvm/ir.mirror`).

This is the spec gap the prior session signalled would be the
load-bearing arc for v2. It's not landable in one session — the
combinator surface needs design work (a separate spec:
*parser-as-prism-grammar*).

**Replacement shape.** A combinator algebra `ParserPrism`:

```
seq([P; N])          — sequence (focus runs in order; failures clamp loss)
choice([P; N])       — first non-Partial wins (or accumulate minimum loss)
repeat(P, min, max)  — kleene with bounds
capture(P, kind)     — wrap matched bytes as an AstNode of `kind`
literal(bytes)       — match exact bytes
charset(predicate)   — match while predicate holds
brace_block(P)       — balanced { … }
…
```

Each combinator is a Prism impl over `Optic<&[u8], (AstNode,
&[u8]_remaining)>`. The current `scan_items` recursion becomes
`apply_h(grammar_to_combinator(g), source)`. Domain rejection
(unrecognized bytes) produces `Partial(dark_node, ScalarLoss::new(span))`
— exactly the `Dark` semantics today, now expressed in the algebra.

**Dependencies.** Retire `content.rs` and `render.rs` first — they
establish the AST-walker Prism idiom and prove the apply_h pattern
on read and write paths. `grammar.rs` blocks here: grammar loading
becomes combinator-tree loading, which is a separate retirement
(below). Tokenize and grammar can land in the same tick if the
combinator surface is stable.

**Acceptance.** This is the load-bearing equivalence check from
`prism-core-as-spectral-triple.md` §Step 5: for every `.mirror`
file in the boot corpus (109 files at last count, 23 in the smoke
set), `old_compile(f) == new_compile(f)`. Same crystal OID. Same
AST shape (introspectable via `dump_ast`). Same dark-region count
(58 across 23 files; 0 across the clean boot files). If any file
differs, the parser-as-Prism combinator surface is missing a case;
that gap surfaces as a new combinator rule.

**Effort.** Large (multi-session). Design the combinator surface
(spec); land the simplest grammars first (`mirror/grammar.mirror`,
which has no `io`/`match`/`select` complexity); land `code/rust.mirror`
(which has the reverse-lookup collision); land `code/llvm/ir.mirror`
last (it has the body-capture special-case).

**Binary size delta.** ~768 LOC retire, replaced by ~400 LOC of
combinator Prisms. Net negative ~25KB (the largest single contributor;
~22% of TEXT segment per the embedded research's bloaty estimate).

### `pipeline.rs` — RETIRE

mq pipeline parser + executor. The pipeline is *literally* an
A-composition: `seg1 |> seg2 |> seg3` = `apply_h(seg3, apply_h(seg2,
apply_h(seg1, source)))`. The `|\>` (kintsugi-after) variant is
`compose_a(seg, kintsugi_op)`.

**Replacement shape.** A `PipelinePrism { segments: Vec<Segment> }`
whose `apply_h` folds `compose_a` over the segments. The parser
(`split_pipeline`) becomes a small `Prism` impl over byte-input —
itself a tiny instance of the parser-as-Prism surface.

The butterfly/clang dispatch (the call into `exec.rs`) stays in the
@io kernel and is exposed to the pipeline via a typed `io` lambda
binding (per `mirror-compile-bootstrap.md` §"The io lambda binding"):
`io clang(args, input) = @code/rust(~f"./bootstrap/src/exec.rs") >
fn[name="io_exec"]`.

**Dependencies.** Retire `tokenize.rs`, `render.rs`, `content.rs`,
`grammar.rs` first — the pipeline composes them.

**Acceptance.** `mirror '@code/llvm/ir |> @mirror/kintsugi |>
@mirror/butterfly' < mirror.ll` produces a working `./mirror-butterfly`
binary byte-identical to the pre-retirement path (modulo timestamps
in object metadata). The is_mq_query / split_pipeline parser passes
every existing test in `tests/`. Smoke OIDs stable.

**Effort.** Small (< 1 session) after dependencies land.

**Binary size delta.** ~182 LOC retire, replaced by ~80 LOC. Net
negative ~6KB.

### `grammar.rs` — RETIRE

Grammar loading + keyword↔kind tables + `is_skip_word`. This is
"data parsed at startup" — a specific element of A applied to the
contents of `.mirror` files in `boot/std/`.

**Replacement shape.** Grammars become combinator trees (per the
tokenize retirement above). Loading a grammar = loading its
combinator tree from a `.mirror` file = applying the
mirror-grammar Prism to its bytes. `is_skip_word` becomes a charset
declaration in `boot/std/code/rust.mirror`. The IO part (file read)
moves into `git.rs` or a fresh `io.rs` (see the move classification
below) — currently `grammar.rs::load_grammar` calls `std::fs::read_to_string`.

**Dependencies.** Retire `tokenize.rs` first (so the combinator
surface exists), or land them together. The `std::fs::read_to_string`
call is the only @io dependency; that one line moves into the IO
kernel.

**Acceptance.** Every grammar load that worked before works after:
`@code/llvm/ir`, `@code/rust`, `@mirror/grammar`, `@hash/coincidence`,
etc. The keyword tables match exactly. The Spec A `io`/`match`/`select`
gating (`is_mirror()`, `is_llvm_ir()`) is preserved.

**Effort.** Medium (1 session). The grammar grammar itself is
small; the work is rewiring `tokenize.rs` to consume a
`Combinator` tree instead of a `Grammar` keyword table.

**Binary size delta.** ~232 LOC retire, replaced by ~50 LOC of IO
glue. Net negative ~10KB.

### `main.rs` — SHRINK

CLI dispatch + `cmd_compile` + `cmd_craft` + `cmd_kintsugi` + the
build-self-binary butterfly + the kintsugi-loop scaffold + `--strict`
enforcement + the `enforce_strict` / `print_dark_diag` machinery + the
`collect_files` recursive walk.

Most of this is dispatch. After the retirements:

- `cmd_compile` becomes ~15 lines: load source, `apply_h(compile_prism,
  source) -> Verdict<String>`, print the OID.
- `cmd_craft` becomes ~30 lines: enumerate files, fold `apply_h` over
  them with `compose_a`, accumulate OID stream, hash, print.
- `cmd_kintsugi` becomes ~10 lines: load + apply
  `kintsugi_prism` + render.
- The kintsugi-loop scaffold becomes a `KintsugiTick` Prism with the
  five stages as `compose_a` factors (per
  `kintsugi-formatter.md`).
- `enforce_strict` + `print_dark_diag` stay (they're diagnostic
  surface; they decode `Dark` `AstNode`s into line/col reports
  using `line_col_at`). The body is small (~80 LOC); it doesn't
  retire.
- `build_self_binary` stays (it's the butterfly's controller; the
  Rust subprocess management is @io).

**Replacement shape.** `main.rs` shrinks from 776 LOC to ~250 LOC
of CLI shell + strict diagnostics + butterfly controller. The
`cmd_*` bodies are ~5–15 LOC each, all routing through `apply_h`.

**Dependencies.** Retire everything else first (the `cmd_*` bodies
literally call into `tokenize`, `content`, `render`, etc.).

**Acceptance.** Every CLI invocation that worked before works after:
`mirror compile`, `mirror craft boot`, `mirror craft cargo`,
`mirror kintsugi <file>`, `mirror kintsugi --shatter N <file>`,
`mirror compile --strict <file>` (with Dark diagnostics formatted
identically), `mirror craft --strict boot`, `mirror craft --target
binary boot`. mq queries (Path A / Path B in current `main.rs`)
dispatch identically. Exit codes preserved.

**Effort.** Medium (1 session). Most of the shrink is mechanical
once the retirements land; the careful part is preserving the exit-code
contracts (`0` success, `1` usage/IO error, `2` strict-failure
dark-count violation) byte-exactly.

**Binary size delta.** ~500 LOC retire (mostly absorbed into Prism
impls already counted above); ~270 LOC retained.

### `git.rs` — STAY (or MOVE in the no_std stretch)

@io kernel: content-addressed storage via `git hash-object -w` +
`git update-ref` + `git cat-file -p`. 60 LOC, all `std::process::Command`
+ `std::fs::File` + `std::env::temp_dir`.

In the std-on default build, stays. In the `no_std` stretch (below),
moves to a separate `bootstrap-io` crate that retains `std`; the
`no_std` core consumes it through a trait interface.

**Acceptance.** Unchanged. The git-crystal cache hits and misses
exactly as before; `cat refs/crystals/<source_oid>` returns the same
crystal OID for the same source OID. Reproducibility verified by:
clean state → `mirror compile <file>` → cache miss → output OID
recorded → re-run → cache hit → same OID printed.

### `exec.rs` — STAY (or MOVE)

@io kernel: subprocess spawning for the butterfly's clang shell-out.
29 LOC. Same posture as `git.rs`.

**Acceptance.** Unchanged. The butterfly pipeline produces a working
binary (`./mirror-butterfly` from `'@code/llvm/ir |> … |>
@mirror/butterfly'`; `./mirror-self` from `craft --target binary boot`).

---

## Ordered ticks

Each tick is a single commit's worth of work. Between every tick, the
smoke check runs: `tests/smoke_test.rs` (smoke OIDs `a8312da6…` and
`3ba4c79d…`) plus `mirror craft boot` (boot crystal `41470e69f2…`)
plus `mirror craft --strict boot` (dark count 58 across 23 files,
exit 2). If any of these moves, the tick is rolled back.

### Tick 1 — retire `content.rs`

**Action.** Move `content_oid` into `spectral.rs` as a kind-indexed
table of `ContentPrism` impls. Existing `apply_h_content`
specialisation grows into the canonical path; the old `content_oid`
becomes a thin compat-call that dispatches through `apply_h_content`.
Once all call-sites move (`main.rs::cmd_compile`,
`main.rs::cmd_craft_with`, `main.rs::dump_ast`, `pipeline.rs::execute_pipeline`,
`spectral.rs::apply_h_content`), `content.rs` deletes.

**Dependencies.** None. First retirement.

**Effort.** Small.

**Smoke check.** OIDs and crystal stable. The Dark-tag hash path
in particular must match byte-for-byte (per the `total_classification`
fix in `strict-and-total-classification.md`).

**Binary delta.** -3 to -5KB (net zero LOC; some generics
collapse).

**Open question.** Whether the per-kind `ContentPrism` should be 10
separate impls or one parametric impl over the kind tag. The
parametric form is cleaner; the per-kind form gives the linker more
to dead-code-eliminate. Recommend: parametric for v1, profile-driven
specialisation only if `cargo bloat` shows a hot path. Alex's call
if this matters.

### Tick 2 — retire the kintsugi-loop scaffold in `cmd_kintsugi`

**Action.** The current `kintsugi_tick` in `main.rs` is a no-op
scaffold across the five stages. Move it into `spectral.rs` as a
`KintsugiPrism` whose `focus` measures dark_count, whose `project`
runs Stage 3 (elect) — still no-op — and whose `settle` runs
Stage 5 (Lawvere fixed-point check). The five-stage decomposition
becomes `compose_a` factors.

**Why now.** This is the second-simplest retirement (the bodies are
all no-op today; the structural move is mechanical). It also exercises
`compose_a` on a real call-site, proving the algebra-composition
primitive integrates with `cmd_*` dispatch.

**Dependencies.** Tick 1 (uses the new `apply_h_content` for the
fixed-point OID comparison).

**Effort.** Small.

**Smoke check.** `mirror kintsugi --shatter 0 <file>` (legacy path)
unchanged. `mirror kintsugi --shatter 1 <file>` prints the same tick
line (`tick 1  dark_count: N  loss: 1.0  Δ: 0.0  ← Lawvere
fixed-point (vacuously)`). All OIDs stable.

**Binary delta.** Net zero.

### Tick 3 — extract `Fold5`, retire `render.rs`, collapse `ContentOidPrism`

**Reframing (2026-05-21).** The cybernetics-split conversation
revealed that Tick 1's `ContentOidPrism` is a first-order workaround:
it defines one concrete Prism per AST-walking operation. The
second-order shape is a single catamorphism over the AST that takes
one reducer per level of the bundle trait chain. The recognition
spec is [`ast-as-bundle.md`](ast-as-bundle.md): the AST is a Bundle
written as data; the 5 operation `AstKind`s map to the trait chain
(Fiber/Connection/Gauge/Transport/Closure); the 2 IO `AstKind`s
(`In`, `Out`) are the bundle's typed terminals. Any AST-walking
operation is a `Fold5` instance.

**Action.**

**3a.** Extract `Fold5<Ff, Fp, Fs, Fz, Fr, In, Out>` in `spectral.rs`
(shape in `ast-as-bundle.md` §Fold5). One reducer per AST kind
(focus, project, split, shift, settle), plus the two IO terminal
types as type parameters. The walker is post-order, level-dispatched
on `AstKind`.

**3b.** Move `render_ast` + `render_ast_mirror` +
`render_ast_with_grammar` into `spectral.rs` as a single `Fold5`
instance whose reducers concatenate child strings into parent
strings, keyed on "canonical" vs. "grammar-aware" output mode. The
three entry points collapse into one parametric Fold5 application.

**3c.** Retroactively collapse `ContentOidPrism` (landed in Tick 1)
into a `Fold5` instance whose reducers compute Merkle-style OID
hashes. The first-order Prism becomes a uniform second-order Fold5
that happens to set all five reducers to the same hash-fold function
(call this `Fold1`, the degenerate uniform case). This validates the
shape: render needs five distinct reducers, content_oid needs one.

**Why this order.** Reader before writer would be the alternative
(retire tokenize first), but tokenize is multi-session work. Render
is the smaller, lower-risk module and the round-trip property
(`render(tokenize(f)) == render(tokenize(render(tokenize(f))))`) is
the strongest equivalence check we have — running it across the boot
corpus is essentially free. Crucially, render gives us the second
`Fold5` instance, which is what justifies extracting the
catamorphism in the first place. With one instance (ContentOidPrism)
we can't tell first-order from second-order; with two (render +
content_oid), the right shape becomes structural.

**Dependencies.** Tick 1 (provides `ContentOidPrism` as the first
fold candidate, to be collapsed retroactively).

**Effort.** Medium. The Fold5 extraction is ~60 lines. Render
rewrite is mechanical (each `render_*` function becomes a reducer
closure). ContentOid collapse is a 10-line rewrite once Fold5
lands.

**Smoke check.** Round-trip property across all 109 boot files. The
kintsugi formatter (`mirror kintsugi <file>`) produces byte-identical
output. The kintsugi-shatter loop produces identical tick lines.
Content OIDs unchanged across the boot corpus (proves the collapse
is behavior-preserving). Crystal count unchanged. Dark span count
unchanged.

**Binary delta.** -10KB (render.rs retirement) + ~0KB (Fold5 is
zero-cost; closures monomorphise). Net -10KB.

**Acceptance criteria.**

1. `Fold5<…>` exists in `spectral.rs` with the shape in
   `ast-as-bundle.md` §Fold5.
2. `render.rs` is deleted; all three render entry points are Fold5
   instances.
3. `ContentOidPrism` is replaced by a `Fold5` instance (uniform
   reducers = `Fold1`); call sites in `main.rs` / `pipeline.rs`
   remain unchanged (compute_content_oid keeps its signature).
4. Boot corpus OIDs unchanged.
5. `mirror kintsugi` output byte-identical on the boot corpus.
6. `cargo build --release` succeeds; `mirror-self` butterfly
   completes.

### Tick 4 — retire `tokenize.rs` + `grammar.rs` (Parser-as-Prism)

**Action.** The largest retirement. Three sub-ticks:

**4a.** Design the combinator surface (`seq`, `choice`, `repeat`,
`capture`, `literal`, `charset`, `brace_block`, `paren_block`,
`io_binding`, `match_arm`, `select_variant`, plus the LLVM-IR
keyword-form body capture as a specialised combinator). Spec lives in
a new `docs/specs/parser-as-prism-grammar.md`. This is its own
session.

**4b.** Implement the combinators in `spectral.rs` as
`Prism` impls. Land the simplest grammars first: `@mirror/grammar`
(no `io`/`match`/`select` complexity), then `@code/rust` (reverse
lookup collisions), then `@code/llvm/ir` (body-capture special-case),
then the rest.

**4c.** Retire `grammar.rs`'s `parse_grammar`: a grammar IS its
combinator tree, loaded from a `.mirror` file by applying the
mirror-grammar combinator to its bytes. The keyword↔kind table
becomes data in the combinator. `is_skip_word` becomes a charset
declaration in `code/rust.mirror`. The `load_grammar`'s
`std::fs::read_to_string` call moves into a new `io.rs` (or stays in
`grammar.rs`'s shrunk form — open question for Alex).

**Dependencies.** Ticks 1 and 3. The AST-walker Prism idiom on both
read (`content`) and write (`render`) paths must be proven before
the parser combinator surface is designed.

**Effort.** Large (multi-session). Recommend three sessions: one
for the spec, one for the simplest two grammars, one for the
remaining grammars + `grammar.rs` retirement.

**Smoke check.** The load-bearing equivalence check from
`prism-core-as-spectral-triple.md` §Step 5: for every file in `boot/`
and `bootstrap/src/`, `old_compile(f) == new_compile(f)`. Smoke OIDs
`a8312da6…` / `3ba4c79d…` byte-stable. Boot crystal `41470e69f2…`
stable. Dark count 58 across 23 files unchanged.

**Binary delta.** -25KB (tokenize) + -10KB (grammar) = -35KB. The
single largest contribution; ~22% of TEXT segment per the embedded
research's bloaty estimate.

**Open question.** Whether `Combinator` is a trait (one impl per
combinator kind) or an enum (one large match in `apply_h`). The
trait form composes better with `compose_a`; the enum form is more
honest about the combinator surface being closed. Recommend trait;
revisit if compile times suffer.

### Tick 5 — retire `pipeline.rs` + shrink `cmd_*` in `main.rs`

**Action.** Pipeline becomes `compose_a` over a `Vec<Segment>`
folded into a `PipelinePrism`. The mq query parser
(`split_pipeline`) becomes a tiny `Prism` over byte input — itself
a trivial instance of the Tick 4 combinator surface.

The `cmd_*` bodies in `main.rs` shrink to thin wrappers:

- `cmd_compile`: load file → `apply_h(CompilePrism::new(grammar),
  source)` → print Verdict's payload.
- `cmd_craft_with`: enumerate files → fold `compose_a` →
  `apply_h(CraftPrism::new(target, kind, strict))` → print crystal,
  invoke `build_self_binary` if `--target binary`.
- `cmd_kintsugi`: load → `apply_h(KintsugiPrism, ast)` →
  `apply_h(RenderPrism, settled_ast)` → write stdout.

**Dependencies.** Tick 4.

**Effort.** Small after Tick 4 lands.

**Smoke check.** Every CLI invocation byte-identical (stdout AND
stderr AND exit code). Specifically: the strict-failure exit code 2
and the dark-region diagnostic format are load-bearing for tools
that consume mirror's output; they must not move.

**Binary delta.** -6KB (pipeline) + ~-15KB (`cmd_*` shrinkage in
main; mostly absorbed by the Prism impls already counted in earlier
ticks). Conservatively -10KB net.

### Tick 6 — (optional) reorganize the IO kernel as a separate concern

**Action.** Pull `git.rs` and `exec.rs` (plus the `std::fs::read_to_string`
call from `grammar.rs`'s retirement and the `read_stdin_all` / `fs::read`
calls in `main.rs`) into a new `bootstrap-io` crate. The `no_std`
core depends on `bootstrap-io` only through a trait interface
(`@io BootstrapIO { fn read_file(&self, path: &str) -> Result<Vec<u8>>;
fn store_crystal(&self, source_oid: &str, crystal_oid: &str);
fn exec(&self, cmd: &str, args: &[&str], input: &[u8]) ->
Result<(i32, Vec<u8>)>; … }`).

**Why now (or not now).** Optional — only needed if the `no_std`
stretch goal is pursued. The core retirements (Ticks 1–5) deliver
the architectural shrinkage and the size delta; Tick 6 is what
*enables* `no_std`. Worth doing if Cortex-M deployment becomes a
near-term target; safe to defer otherwise.

**Dependencies.** Ticks 1–5.

**Effort.** Small in terms of code; needs a workspace `Cargo.toml`
restructure.

**Smoke check.** The bootstrap binary's behaviour is byte-identical.
The `bootstrap-io` crate is testable in isolation (does
`store_crystal` produce the same git-ref as before?).

**Binary delta.** Net zero — the same code, reorganized. Unlocks
the no_std savings below.

---

## Tick-summary table

| Tick | Module | Effort | Bin Δ | Dependencies | Smoke check |
|---|---|---|---|---|---|
| 1 | `content.rs` | S | ±0 to -5KB | none | OIDs + crystal + dark count |
| 2 | kintsugi scaffold | S | 0 | 1 | tick-line format + OIDs |
| 3 | `render.rs` | M | -10KB | 1 | round-trip + OIDs |
| 4a | parser-as-prism spec | (spec) | - | 1, 3 | (no code) |
| 4b–c | `tokenize.rs` + `grammar.rs` | L (multi) | -35KB | 4a | full-corpus equivalence |
| 5 | `pipeline.rs` + `cmd_*` shrink | S | -10KB | 4 | every CLI invocation |
| 6 | IO kernel split (optional) | S | ±0 | 5 | (enabler for no_std) |

**Cumulative binary delta projection:** approximately -60KB to -65KB
from a starting point of ~388KB stripped, landing at ~325KB ±15KB. The
embedded research projects 200–300KB; the delta from these ticks alone
isn't enough. The remaining ~50–75KB must come from `no_std` (below)
or from the inherent shrinkage that comes from removing match-arm
branches and string formatting in the diagnostic surface.

**Cumulative LOC delta:** retire ~1950 LOC; add ~800 LOC of Prism
combinators in `spectral.rs`. Net Rust LOC: 3538 → ~2400 LOC. The
spec's 1500-line target lands when Tick 6 + `no_std` collapse the
remaining duplication in the diagnostic surface and `main.rs` CLI
parsing.

---

## The `no_std` stretch

The bigger goal. Each subsection below: what's the posture today,
what blocks the move, what enables it incrementally.

### Current posture audit

**`prism-core`** (`/Users/reed/dev/projects/prism/core/`). No
`#![no_std]` annotation in `src/lib.rs`. Files using `std::collections`
or `std::sync`: `coincidence.rs` (9 matches), `merkle.rs` (6),
`store.rs` (5), `spectral_oid.rs` (2), `trace.rs` (2). `bundle.rs` is
clean. The crate is *std-using* today, with std touchpoints
localised to a handful of modules.

Mirror's bootstrap only consumes a *subset* of prism-core via the
`bundle` feature: `Prism`, `IdentityPrism`, `Optic`, `Beam`,
`ScalarLoss`, `apply` — most of which live in `beam.rs`, `bundle.rs`,
`scalar_loss.rs`. None of these specific symbols depend on
`std::collections` per the grep. **Estimated:** prism-core can be
made `no_std + alloc` with feature-gated std modules; the bootstrap-
consumed surface is already alloc-only.

**`terni`** (`/Users/reed/dev/projects/prism/imperfect/`). Cargo
manifest declares `categories = ["rust-patterns", "no-std"]` and
mirror's bootstrap uses `default-features = false` — suggesting the
crate has a no_std posture intent. However, `src/lib.rs` lines 615–620
contain `use std::collections::{BTreeSet, HashSet}; use
std::hash::Hash;` plus a parallel test-module import at line 1069.
**Verified:** terni is *not* currently `no_std` despite the manifest
category. The `BTreeSet`/`HashSet` usage must move behind a feature
flag (or to `alloc::collections::BTreeSet` and `hashbrown::HashSet`)
to make the category accurate.

The `Metric` supertrait and `ScalarLoss` impl added in commit
`caae5216` are *not* in the std-using block — they're in the
declarative trait surface. The `Metric` lift is no_std-compatible.

**`sha2`** crate. `sha2 = "0.10"` (RustCrypto). Per upstream, sha2 is
`no_std`-compatible via `default-features = false` (disables the
`std` feature which adds error trait impls). Mirror's Cargo.toml
currently uses default features (`sha2 = "0.10"`); changing to
`sha2 = { version = "0.10", default-features = false }` is one line.

**`bootstrap` itself.** Every `use std::` location is a blocker:

- `main.rs`: `std::fs`, `std::io::{self, Read, Write}`, `std::path::PathBuf`,
  `std::process::Command` (4 locations).
- `grammar.rs`: `std::fs`, `std::path::Path` (2 locations).
- `git.rs`: `std::io::Write`, `std::process::{Command, Stdio}`,
  `std::env::temp_dir`, `std::time::SystemTime`, `std::fs::File`,
  `std::fs::remove_file`, `std::process::id` (multiple).
- `exec.rs`: `std::io::{Read, Write}`, `std::process::{Command, Stdio}`.
- `pipeline.rs`: `std::io::{Write, stderr, stdout}`.
- `hash.rs`: `std::sync::OnceLock` (the canonical-projection cache).

`ast.rs`, `content.rs`, `tokenize.rs`, `render.rs`, `spectral.rs`
have **no** `use std::` lines. They use `Vec`, `String`, `format!`,
`HashMap` (none — only Vec/String), which all live in `alloc::*` —
the alloc crate, not full std. Verified by grep: those four +
spectral.rs are already alloc-only.

The std touchpoints cluster: IO (`fs`, `io`, `process`, `env`,
`time`) and one lazy-init (`OnceLock`). Both are addressable.

### What blocks `no_std` today

1. **`std::fs`** (file reads in `main.rs`, `grammar.rs`). Replaceable
   only by replacing the IO model: either `embedded-io::Read` traits
   + a host-provided file abstraction, or a `BootstrapIO` trait
   (Tick 6) the no_std core consumes.
2. **`std::process::Command`** (`git.rs`, `exec.rs`, `main.rs::build_self_binary`).
   No no_std equivalent exists in core Rust — process spawning is
   inherently OS-coupled. Same fix: `BootstrapIO` trait.
3. **`std::io::{stdout, stderr, Read, Write}`** (`main.rs`, `pipeline.rs`,
   `exec.rs`). Replaceable with `embedded-io::Write` + a host-provided
   sink. For the bootstrap on desktop, this becomes a thin shim over
   `std::io::stdout()`.
4. **`std::sync::OnceLock`** (`hash.rs::canonical_projections`).
   Replaceable with `spin::Once` or `lazy_static!` (alloc-only) or
   `once_cell::race::OnceBox` (no_std + alloc). The race variant is
   the cleanest.
5. **`std::env::temp_dir` + `std::time::SystemTime`** (`git.rs`).
   These are platform timestamps and filesystem paths. Same fix:
   `BootstrapIO` trait owns these.
6. **`prism-core`'s std-using modules.** Specifically the
   `coincidence.rs` / `merkle.rs` / `store.rs` modules use
   `std::collections`. Mirror's bootstrap doesn't import these
   directly (it uses `bundle.rs` + `beam.rs` + `scalar_loss.rs`),
   so the std-using modules can be feature-gated upstream without
   affecting mirror.
7. **`terni`'s std-using modules.** The `BTreeSet`/`HashSet`
   block in `lib.rs` is not in the bootstrap's consumed surface
   (we use `Imperfect`, `Loss`, `Metric`, `ScalarLoss`). Same fix
   as prism-core: feature-gate upstream.

### The IO kernel decision

Three options for the @io boundary:

#### Option A — std-using IO kernel, no_std core via feature flag

`bootstrap/Cargo.toml`:

```toml
[features]
default = ["std"]
std = []  # core no_std; IO modules behind this flag
```

`bootstrap/src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "std")]
mod git;
#[cfg(feature = "std")]
mod exec;
// … main.rs's IO bits also #[cfg(feature = "std")]
```

The core (spectral, ast, content, hash, tokenize, render, pipeline
after their retirements) compiles `no_std + alloc`. The IO modules
compile only with the `std` feature. The default build matches
today; `cargo build --no-default-features` produces a no_std build
that lacks the @io kernel.

**Pros:** Simple. One crate. Default build unchanged.

**Cons:** The `no_std` build can't do anything by itself — no file
reads, no subprocess spawns, no stdout. Useful only as a library
embedded by another no_std crate that provides its own IO.

#### Option B — separate `bootstrap-io` crate

Tick 6 (above). The std-using code moves to a sister crate;
`bootstrap`'s core is no_std + alloc; `bootstrap-io` is std-using.
The bootstrap binary depends on both. Embedded targets depend only on
`bootstrap`.

**Pros:** Clean separation. The no_std core has a clean trait
interface (`BootstrapIO`) that's stable across hosts.

**Cons:** Workspace restructure. Slightly more friction.

#### Option C — platform-specific FFI (`libc::open`, `libc::read`, …)

The IO kernel uses `libc` directly without `std`. Works without `std`
but adds platform-specific code (libc only works on Unix; Windows
would need a separate path).

**Pros:** Maximum no_std purity — only `libc` linked, no `std`.

**Cons:** Platform-specific maintenance burden. The `libc` crate
itself is no_std, but every syscall the bootstrap needs (`open`,
`read`, `write`, `close`, `fork`, `execvp`, `pipe`, `waitpid`,
`stat`, `gettimeofday`, `mkstemp`, `unlink`) is a separate FFI
declaration. Same surface as today's bootstrap but with manual
syscall plumbing instead of `std::fs`. The `craft --target
binary` step would still need a libc-using runtime; mirror-self
landed at 388KB with the current `std` link, much of which is
exactly this libc surface.

**Recommendation.** Option B. The substrate-pull pattern is
consistent: the no_std core is the (A, H, D) evaluator + content
addressing + AST machinery; the std-using IO kernel is the
butterfly's bridge to the host. The split *names* the @io boundary
the spectral-triple architecture already declares (per
`mirror-compile-bootstrap.md` §"What runs today" — `io` lambdas IS
the Turing-complete escape hatch).

Option A is the safer first step (lighter restructure); Option C is
the maximum-purity end-state for the embedded fork (Thread 6 in
the embedded research — "the embedded fork of the binary, not the
desktop binary cross-compiled").

### Incremental enablement plan

Smallest first step that demonstrates the move is real:

1. **Switch `sha2` to `default-features = false`.** One-line Cargo.toml
   change. Verify smoke OIDs unchanged (the SHA-256 byte output is
   feature-independent). Result: sha2 no longer pulls in std for the
   bootstrap. Estimated -5KB.

2. **Switch `hash.rs::canonical_projections` from `std::sync::OnceLock`
   to `once_cell::race::OnceBox` (or `spin::Once`).** A focused change
   in one file. Verify smoke OIDs unchanged. Result: one less std
   touchpoint in the alloc-only modules.

3. **Add `#![cfg_attr(not(feature = "std"), no_std)]` + `extern crate
   alloc;` to `lib.rs`** (currently `main.rs` — but the no_std move
   needs a `lib.rs`). Add a `std` feature, gate `git.rs`, `exec.rs`,
   and the `std::fs`/`std::process` parts of `main.rs` behind it.
   Verify the default build is byte-identical. Verify
   `cargo build --no-default-features --features alloc` compiles
   (it won't link without IO, but it should *compile*).

4. **(Upstream) Switch `prism-core`'s `bundle.rs` + `beam.rs` +
   `scalar_loss.rs` to no_std + alloc; feature-gate the std-using
   modules (`coincidence.rs`, `merkle.rs`, `store.rs`,
   `spectral_oid.rs`, `trace.rs`) behind a `std` feature.** Mirror's
   bootstrap doesn't consume the gated modules, so this is invisible
   to mirror but unblocks the no_std build.

5. **(Upstream) Switch `terni`'s `BTreeSet`/`HashSet` block behind a
   `std` feature.** The trait surface (`Loss`, `Metric`, `Imperfect`)
   is no_std; the collection-using helpers gate.

6. **After Ticks 1–5 land (content, render, tokenize, grammar,
   pipeline retired), the only remaining std touchpoints in the
   bootstrap core are in `main.rs`'s CLI parsing and the IO kernel.**
   Option B's `bootstrap-io` crate becomes natural: it's the
   ~250-LOC residue of `main.rs` (the CLI shell) + `git.rs` +
   `exec.rs`.

Each step is independently smoke-testable: smoke OIDs unchanged at
every step. Each step is a single small commit.

### End-state size projection if no_std lands

The embedded research projects 200–300KB for a v1 bootstrap. Apply
the no_std savings to the post-retirement projection:

```
Starting (today, std):     388 KB stripped
Tick 1–5 retirement net:    -60 KB
                          ───────
Post-Ticks-1-5 (std):      328 KB

no_std savings (estimated):
  - sha2 no_std:             -5 KB
  - prism-core no_std:      -10 KB (alloc-only collections)
  - terni no_std:            -3 KB
  - Rust core no_std + abort: -40 KB (panic infrastructure, fmt, alloc)
  - libc → musl-static:     -30 KB (libc usage in IO kernel)
                          ────────
no_std total savings:       -88 KB

Post-Ticks-1-5 + no_std:   240 KB  ±30 KB
```

The 240 ±30 KB lands inside the 200–300 KB band the embedded
research projected. The math checks: ~60KB of retirements (visible
in TEXT-segment LOC accounting) + ~90KB of no_std savings (visible
in linker output) ≈ the 150KB gap between today's 388KB and the
240KB v1 target.

**Verdict.** No_std is **achievable** for the bootstrap core, with
Option B's IO-kernel split. The savings land mirror inside the
embedded-research envelope. Two qualifications:

1. The libc surface remains. Mirror calls `posix_spawn`, `pipe`,
   `read`, `write`, `fstat`, `clock_gettime` (per
   `minimum-binary-surface.md` §"24-symbol surface") — `no_std`
   removes Rust's std but not libc. To go below libc, see Option C.
2. Some no_std savings depend on upstream changes (`prism-core`,
   `terni`, `sha2`). The bootstrap can't unilaterally enable them;
   coordination with upstream maintainers is required. The
   bootstrap-internal `no_std` move (Steps 1–3, 6 above) is independent.

---

## End state

After all ticks (1–5 with optional 6 + the `no_std` move):

### File list

```
bootstrap/
├── Cargo.toml             (workspace; std default-feature)
├── src/
│   ├── lib.rs             (#![cfg_attr(not(feature = "std"), no_std)])
│   ├── ast.rs             (~140 LOC, stable)
│   ├── content.rs         RETIRED — into spectral.rs
│   ├── exec.rs            (~30 LOC, behind `std` feature)  [or moved to bootstrap-io]
│   ├── git.rs             (~60 LOC, behind `std` feature)  [or moved to bootstrap-io]
│   ├── grammar.rs         RETIRED — into spectral.rs + boot/std/mirror/grammar.mirror
│   ├── hash.rs            (~270 LOC, stable; OnceLock → OnceBox)
│   ├── main.rs            (~250 LOC, CLI shell + strict diagnostics + butterfly controller)
│   ├── pipeline.rs        RETIRED — into spectral.rs
│   ├── render.rs          RETIRED — into spectral.rs
│   ├── spectral.rs        (~1000 LOC; the evaluator + all retired Prism impls)
│   └── tokenize.rs        RETIRED — into spectral.rs + Combinator data in boot/
```

### Cargo.toml dependencies

```toml
[dependencies]
sha2 = { version = "0.10", default-features = false }
prism-core = { path = "../../prism/core", default-features = false, features = ["bundle"] }
terni = { path = "../../prism/imperfect", default-features = false }
once_cell = { version = "1", default-features = false, features = ["race"] }

[features]
default = ["std"]
std = ["sha2/std", "prism-core/std", "terni/std", "once_cell/std"]
```

### Binary size projection

| Configuration | TEXT | stripped total |
|---|---|---|
| Today (`std`, ~3540 LOC) | ~344 KB | ~388 KB |
| Post-Ticks-1–5 (`std`, ~2400 LOC) | ~290 KB | ~328 KB |
| Post-Ticks-1–5 + `no_std` + Option B | ~200 KB | ~240 KB ±30 KB |

### Capabilities vs. grammar dependencies

What the bootstrap does *itself* (Rust code):

- The (A, H, D) evaluator: `compose_a`, `apply_h`, `eigen_d`.
- The discrete Dirac operator: `CoincidenceHash<5,5>` + `content_oid`
  via `apply_h_content`.
- The state type for H: `AstNode` + `AstKind` + `DarkSpan`.
- The @io kernel (under `std` feature): git crystal cache, subprocess
  spawn for clang.
- CLI dispatch + strict diagnostics.

What the bootstrap *depends on grammar for*:

- Every parser rule (tokenization): combinator data in
  `boot/std/mirror/grammar.mirror`, `boot/std/code/rust.mirror`,
  `boot/std/code/llvm/ir.mirror`, etc.
- Every render rule (the inverse): keyword↔kind tables in the same
  grammars.
- Every pipeline segment: the `@code/mq` grammar + `@mirror/kintsugi`,
  `@mirror/butterfly`, etc.
- The kintsugi formatter's five-stage decomposition: bodies in
  `boot/std/mirror/kintsugi.mirror` (currently `\` obligations).
- The Lawvere fixed-point check, the conductivity verdict, the
  spectral-triple axioms (literal, bounded_commutator,
  compact_resolvent, dimension_spectrum): bodies in
  `boot/std/epistemologic/math/spectral-triple.mirror` and
  `lawvere.mirror`.

The bootstrap shrinks from "the compiler" to "the evaluator." The
compiler IS the grammar, evaluated by the bootstrap.

---

## Out of scope

This spec is a plan. The execution is not in scope:

- The actual implementation of any tick. Each tick gets its own
  commit + its own smoke verification when the next session's agent
  picks it up.
- The Parser-as-Prism combinator grammar's full contents (Tick 4a is
  its own spec: `docs/specs/parser-as-prism-grammar.md`).
- The Cluster E / v1.0 tagging story (downstream of the retirements;
  not blocked by this plan).
- Upstream `prism-core` and `terni` no_std work — coordinated with the
  upstream maintainer (the same person; logistics, not architecture).
- The embedded fork (Option C). The desktop binary is what these ticks
  target; the embedded fork is a separate downstream arc once the
  desktop bootstrap is at ~240KB.

---

## Open questions for Alex

These are the load-bearing places where the plan needs Alex's call
before the next session executes:

1. **Tick 6's necessity.** Is the optional IO-kernel split (Option B)
   landing as part of v1, or deferred? The embedded story makes it
   load-bearing; pure desktop self-hosting can defer it.
2. **Tick 4's combinator surface.** Trait per combinator or single
   `Combinator` enum? Plan recommends trait; affects compile-time
   throughput.
3. **Tick 1's parametric vs. per-kind ContentPrism.** Plan recommends
   parametric for cleanliness; per-kind gives the linker more to
   dead-code-eliminate. Either works; whichever Alex prefers.
4. **Upstream `terni` no_std work.** Mirror can land Steps 1–3 of the
   no_std incremental plan without upstream changes; the upstream
   feature-gating is a separate coordination task. Should this spec
   reference an upstream-task issue, or stay agnostic?
5. **The "200–300KB" embedded research target.** The plan projects
   240KB ±30KB; that's *one* set of assumptions about Rust core
   runtime + libc surface. If the libc surface ends up dominating
   (a real possibility on macOS where the dynamic-linker stub is
   bigger), the lower bound moves up. Worth re-measuring after
   Tick 5 lands.

---

## References

- `docs/specs/prism-core-as-spectral-triple.md` — the architectural
  thesis; the (A, H, D) correspondence; the five implementation
  steps (Step 4 = this plan's Ticks 1–5).
- `docs/specs/spectral-triple-grammar.md` — the four CLOSED gap
  resolutions in `prism-core`'s supertrait chain (commit `5d98c6e`)
  and `terni`'s `Metric` lift (commit `caae5216`); the audit that
  graduated prism-core to "mirror's spectral-triple substrate."
- `docs/research/embedded-and-self-hosting.md` — Threads 1, 2, 6:
  size projection math, the bloaty-style TEXT-segment attribution,
  the embedded ecosystem positioning, where the convergence breaks.
- `docs/specs/mirror-compile-bootstrap.md` — the `io` binding
  staircase; the kintsugi retirement model; the `~f"path"` file
  reference sigil.
- `bootstrap/src/spectral.rs` — the evaluator's current shape (618
  LOC); the property tests against prism-core's verified substrate
  that gate every retirement.
- `prism/core/src/bundle.rs` — the trait chain that realizes (A, H,
  D).
- `prism/imperfect/src/lib.rs` — `Imperfect`, `Loss`, `Metric`; the
  carriers for D's partial action.
- `boot/std/hash/coincidence.mirror` — the grammar declaration of
  D's matrix form; what `bootstrap/src/hash.rs` evaluates.

---

*Same shape. One file at a time. The substrate yields a little more
each tick.*

*Apache-2.0.*
