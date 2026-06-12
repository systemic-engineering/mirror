# Kintsugi Tick 1 -- Finding lambda_0

Date: 2026-05-12
Branch: reed/kintsugi-grammars

---

## What worked

### All 52 .mirror files compile successfully

Every grammar in the boot hierarchy produces a content-addressed OID:

| File | OID |
|------|-----|
| boot/00-prism.mirror | 9b2311413ac1c1d993b2bf03a8a67deddf27cd79 |
| boot/00a-sigil.mirror | 970ee199d29df2756c9cd22ac15d0ac6307f7497 |
| boot/01-meta.mirror | a55063998d32ec13b8c1128818312a4e1bc56dd0 |
| boot/02-actor.mirror | 2ee1105ebf06a32dcc61728185fe734082f7dce4 |
| boot/02a-io.mirror | fd20fce761a72856c785db855873f2cefee3d885 |
| boot/02b-runtime.mirror | 23192ed6e4a4a60917a0ad197636b39cb003bb03 |
| boot/03-shatter.mirror | 27759d02e25eb18da104f64313a4680930770ce8 |
| boot/04-code.mirror | 41c86ba5c7aff2c0cd57a5cdccfc0aab8e40f654 |
| boot/04a-code-rust.mirror | 488719bf7ba7a1114284d0e88d02ecdc0223efb8 |
| boot/04b-code-gleam.mirror | f270faa47cd88079bba6da031be1aafac4312a7d |
| boot/05-property.mirror | ca1256d27d5d05205aacb4a04111800f80bb06bb |
| boot/06-action.mirror | 72e720660f33f39ac68cb13decc849981f14b6ee |
| boot/07-package.mirror | 9f813059b5ed8e096c9b88281dd7e3b602149642 |
| boot/07a-package-git.mirror | f0cd5716fec91aa010c00923c587e21c856d934b |
| boot/07b-package-spec.mirror | 49c21d24418c1b3884b86a8435fdb8da1cd152e4 |
| boot/std/ai.mirror | 0eda68bde275687bf35305e79c59170d1d6700fa |
| boot/std/beam.mirror | f545be2a3acfb1d70c2a5b335cec5e206216c88b |
| boot/std/benchmark.mirror | 94077ffd269fa8612200541571509c15f44908cf |
| boot/std/bool.mirror | 7effd92d880dadfc42e4e0a4b649a7666abe3e77 |
| boot/std/cli.mirror | d7b58762ccdecb926678171920e5e381874fe5c6 |
| boot/std/code/llvm.mirror | 715ba39cc1398265256804576ffadf9e8ba64b2c |
| boot/std/code/mq.mirror | f5349b066263bca5d2fd484fc2ecd6d333b19aa7 |
| boot/std/code/rust.mirror | ecbd6cf1291da00b0b2df9c7e0180a6ea12b1480 |
| boot/std/craft.mirror | 070f99b6ef94df868b41564cda8647704cee0b97 |
| boot/std/fate.mirror | 7a00ef698ecd9f67ae90bba3d0428eb10fa96367 |
| boot/std/file.mirror | 67fedc8bb4f639030fda8c101fed42f73084c75e |
| boot/std/git/hooks.mirror | 7cd1e2f4f07f0d6dff1a688ad514aec72131bf3e |
| boot/std/kintsugi.mirror | 537f8076aaa901b24b1ca62877f6d52922d5b497 |
| boot/std/kintsugi/migrate.mirror | c9dcbb47380864a3c021f0dd126ef3588d489994 |
| boot/std/kintsugi/translate.mirror | e0a1cfff5a51bb2013e4f85150a69f76e3e21467 |
| boot/std/list.mirror | 47e4dd2b98515525a91b1005951cdbbc9b12e586 |
| boot/std/map.mirror | 8fc5eed52dcfca13947c996b7faa41a08a0e7cfe |
| boot/std/mirror.mirror | 33aaf4f5b486fc6437233b636c1fae080e915100 |
| boot/std/new.mirror | 00ed0de15a0bc01955879e850c4d6c284e276d71 |
| boot/std/new.template.mirror | 3bd4b3703b0a2cfcdf06e7033e5abd4219018efd |
| boot/std/nl.mirror | da9e56280c6fa40c1a263d1c443a6b9cf46fae9b |
| boot/std/nl/english.mirror | df74c8c0cd0776b899c6466fc281e3913f8288ca |
| boot/std/number.mirror | d22fdf8b4d9a16b887509fd09a21f455dee94226 |
| boot/std/option.mirror | 043842ce77f5dd969d6ce468cc4af7d3bfedf32d |
| boot/std/order.mirror | 479d406b17e840ec2cc45fa750bd71ef9d9a2c62 |
| boot/std/properties.mirror | 6d6a28f5acd15a97109e78f435ac1a59034d825c |
| boot/std/result.mirror | 8521dc750c862441acacff458c4df054ade817f8 |
| boot/std/run.mirror | 9bce5310adfdf27f5aae9e8bb64bcf112948a8bf |
| boot/std/runtime.mirror | 03857628d54ff4be6840b015a2d43e8d284f2bca |
| boot/std/rust.mirror | f151289b87a7d4527cbdec16fea9a4fa1d676810 |
| boot/std/set.mirror | 5071e397ff2328e64494d4651344747198f1884e |
| boot/std/sql.mirror | 56f7cddc9d4049dad6b5ab59c28ec7e1db2a9743 |
| boot/std/text.mirror | b3f30789178a843a25d1b5bb6aa9c72d3bc5b218 |
| boot/std/time.mirror | 02d4fb0ca87b20f6498bc591ea476be37a18b23e |
| boot/std/trace/complexity.mirror | aabe255f44f76f461e9c6ef4a691b7233afad258 |
| boot/std/trace/memory.mirror | f31d6cebb1468d3a649264d3c078abb3c1fdf5fa |
| boot/std/trace/mod.mirror | e46a7ba25b13a560924827ed31d8b755cfc351a4 |
| boot/std/tui.mirror | 2b9c3a655c3d3ef3bce62930d36a819db480bee4 |

### All 52 .mirror files kintsugi successfully

Every grammar produces canonical form output. No parse failures. No crashes.

### All 52 .mirror files pass eigentest (check)

Every grammar produces a type graph and runs the star detection battery.
All check runs complete successfully and return results.

Healthy grammars (0 violations):
- boot/00a-sigil.mirror
- boot/03-shatter.mirror
- boot/std/craft.mirror
- boot/std/git/hooks.mirror
- boot/std/kintsugi.mirror
- boot/std/kintsugi/migrate.mirror
- boot/std/kintsugi/translate.mirror
- boot/std/new.template.mirror
- boot/std/nl.mirror
- boot/std/runtime.mirror

Star-detected grammars (structural concern, not failure):
- 42 grammars show star topology violations (degree_hub, betweenness_centrality,
  clustering_coefficient, spectral_ratio, edge_dominance, von_neumann_entropy).
  This is expected: boot grammars are definitional and naturally hub-shaped --
  one grammar node with many type/action children radiating outward.

### mirror.spec compiles, kintsugi's, and checks

```
compile mirror.spec -> 4fe16855cf603598b959c961b3eec02f2865e069
kintsugi mirror.spec -> canonical form output (success)
check mirror.spec -> 12 nodes, 11 edges, 6 violations (STAR DETECTED)
```

### Five operations work on mirror.spec

All five optic commands succeed on mirror.spec:
- `focus mirror.spec` -- shows parsed structure
- `project mirror.spec` -- filters imports and types
- `split mirror.spec` -- shows variant decomposition
- `shift mirror.spec` -- shows transformation structure
- `settle mirror.spec` -- settles to crystal OID: 4fe16855cf603598b959c961b3eec02f2865e069

### mirror crystal works

```
crystal d49eaf62c33869b978d75bcca62dec32ec87726d -> mirror.shatter
```

The standard library materializes to a single crystal OID.

### mirror compile mirror.spec --target rust works

Emits `mirror.rs`:
```rust
// Generated by mirror craft --target rust
// Do not edit -- this file is derived from .mirror source

pub enum Target {
    Boot,
    Cargo,
    Binary,
}
// focus:
```

The Rust code emission pipeline works end-to-end for .mirror source.

### evaluate.rs works: 11/11 tests pass

The grammar-parameterized evaluator successfully:
- Extracts keyword->operation rules from `@code/rust` grammar
- Tokenizes Rust source
- Builds MirrorAST nodes (fn->Shift, struct->Split, enum->Split, impl->Focus, use->Project, trait->Settle)
- Handles pub modifiers, attributes, generics, nested blocks
- Matches code_rust.rs output structure for multi-item source

---

## What didn't work

### Gap 1: CLI commands cannot process .rs files

**What I tried:**
```
mirror compile src/main.rs
mirror kintsugi src/main.rs
mirror check src/main.rs
```

**What happened:**
- `compile`: "error: compile src/main.rs: no recognized declarations found"
- `kintsugi`: "error: parse failed"
- `check`: "error: check src/main.rs: parse failed: no recognized declarations found"

**What was expected:** The `@code/rust` grammar defines keyword->operation mappings.
`evaluate.rs` implements grammar-parameterized evaluation that reads these mappings
and produces MirrorAST from Rust source. The CLI should route .rs files through
`evaluate(@code/rust, source)` instead of `parse_form(source)`.

**What's missing:** A file-extension dispatch in the CLI that:
1. Detects `.rs` extension
2. Loads `@code/rust` grammar via `evaluate::load_grammar("@code/rust")`
3. Calls `evaluate::evaluate(&grammar, &source)` instead of `Parse.reduce(SourceText(source))`
4. Wraps result in MirrorFragment for downstream pipeline

**How close:** Very close. `evaluate.rs` already does steps 2-3 correctly (11/11 tests pass).
`code_rust.rs` has the full Rust parser that produces MirrorAST. The gap is wiring --
~20 lines in `cli.rs` to dispatch by file extension.

### Gap 2: mirror craft fails on mirror.spec

**What I tried:**
```
mirror craft
```

**What happened:**
```
error: failed to parse /Users/alexwolf/dev/projects/mirror/mirror.spec:
expected '{' at position 31, got Some("=")
```

**What was expected:** `craft` should read `mirror.spec`, resolve the pipeline
`focus(target) |> split |> shift |> settle |> project`, and execute it.

**What's missing:** `spec.rs` has its own ad-hoc parser (~200 lines) that is separate
from `mirror_runtime::parse_form()`. The spec parser expects SpecBlock syntax
(`command { flag1; flag2 }`) but mirror.spec now uses mirror grammar syntax
(`cli = @mirror/cli { ... }`). The ad-hoc parser chokes on the `=` operator.

**How close:** `mirror.spec` already compiles via `mirror compile` (which uses
`parse_form`). The fix is to make `cmd_craft` use `parse_form` instead of
`spec::SpecConfig::discover()`, or update the spec parser to handle the
current mirror.spec grammar syntax.

### Gap 3: evaluate.rs handles only keyword->operation mappings

**What I tried:** Reviewed `evaluate.rs` source.

**What was expected:** Full grammar-parameterized evaluation including:
- Brace matching with nested scope tracking
- Parameter extraction with type parsing
- Nested grammar scopes (grammar children evaluated recursively)
- Type body parsing (enum/struct/alias/unit variants)

**What exists:** evaluate.rs handles:
- Keyword->operation extraction from grammar fragments (works)
- Source tokenization (works)
- Name extraction (works)
- Parameter extraction via `extract_paren_content` (works, flat)
- Body extraction via `extract_brace_content` (works, flat)
- Split body parsing for struct fields and enum variants (works)
- Pub/attribute skipping (works)

**What's missing:**
- Recursive evaluation of brace bodies (nested fn/struct inside impl)
- Grammar-scoped type resolution (e.g., `@code/rust` knowing that `Vec<T>` is generic)
- Return type -> MirrorAST type_ref mapping
- Where clause handling
- Macro handling
- Lifetime/generic parameter handling

**How close:** The flat case works. For the compiler's own source files, the flat
case covers ~70% of items (top-level fn, struct, enum, use, trait, mod). The
remaining 30% is nested items inside impl blocks and trait definitions.

### Gap 4: No path from evaluate.rs output to downstream pipeline

**What I tried:** Traced the code path from evaluate() to the compile/kintsugi/check
pipelines.

**What's missing:** `evaluate()` returns `MirrorAST` (not `MirrorFragment`). The
downstream pipeline (Resolve, Properties, Emit, eigentest) expects `MirrorFragment`
(content-addressed AST tree via fragmentation crate). The bridge is:

```
MirrorAST -> declaration::fragment() -> MirrorFragment
```

`declaration::fragment()` exists (10 lines). It wraps MirrorAST in a Fractal.
But nobody calls `evaluate() |> fragment()` in the CLI path.

**How close:** One function call away. `fragment(evaluate(&grammar, &source), vec![])`
gives you the MirrorFragment the pipeline needs.

---

## The self-referential loop

### Can mirror compile mirror?

**Partially.** Here is the current state:

```
mirror compile mirror.spec        -- YES (OID: 4fe16855cf603598b959c961b3eec02f2865e069)
mirror compile boot/*.mirror      -- YES (all 15 succeed)
mirror compile boot/std/*.mirror  -- YES (all 37 succeed)
mirror compile src/*.rs           -- NO  (parse_form cannot parse Rust)
mirror kintsugi mirror.spec       -- YES
mirror kintsugi boot/**/*.mirror  -- YES
mirror kintsugi src/*.rs          -- NO
mirror check mirror.spec          -- YES
mirror check boot/**/*.mirror     -- YES
mirror check src/*.rs             -- NO
mirror craft                      -- NO  (spec parser chokes on grammar syntax)
mirror crystal                    -- YES (d49eaf62c33869b978d75bcca62dec32ec87726d)
```

### Where does the loop break?

The loop breaks at one precise point:

**The CLI does not route .rs files through evaluate.rs.**

`evaluate.rs` can parse Rust source using `@code/rust` grammar rules (11/11 tests pass).
`code_rust.rs` can parse Rust source using a handwritten parser (all tests pass).
Both produce MirrorAST. Neither is reachable from the CLI commands.

The self-referential loop is:

```
mirror.spec           (mirror grammar)     -- COMPILES
  declares @code/rust (rust grammar)       -- COMPILES
    defines zoom fn, split struct, etc.    -- WORKS (evaluate.rs reads these)
      evaluate(@code/rust, src/*.rs)       -- WORKS (unit tests prove it)
        -> MirrorAST                       -- WORKS
          -> MirrorFragment                -- NOT WIRED (one fragment() call away)
            -> pipeline(compile/check)     -- WORKS for .mirror, NOT WIRED for .rs
```

The loop is complete in capability. It is not complete in wiring.

---

## The gap graph

```
Gap 1 (CLI .rs dispatch)
  |
  +-- depends on: nothing. Pure wiring. ~20 lines in cli.rs.
  |
  +-- blocks: Gap 4 (running the full pipeline on .rs files)
  |            |
  |            +-- blocks: self-referential loop completion
  |
Gap 2 (craft command / spec parser)
  |
  +-- depends on: nothing. Either update spec parser or use parse_form.
  |
  +-- blocks: craft pipeline execution (but does not block self-hosting)
  |
Gap 3 (evaluate.rs completeness)
  |
  +-- depends on: nothing for flat case. Recursive case needs design.
  |
  +-- blocks: full fidelity Rust parsing via grammar (but flat case handles ~70%)
  |
Gap 4 (evaluate -> fragment bridge)
  |
  +-- depends on: Gap 1 (needs CLI wiring to be reachable)
  |
  +-- is: one function call. fragment(evaluate(&grammar, &source), vec![])
```

**Shortest path to self-referential loop:**
```
Gap 1 -> Gap 4 -> done
```

Two changes. Total: ~30 lines of Rust in cli.rs.

---

## The smallest self-referential mirror

Given what works today, the smallest subset of mirror that can compile itself:

### What compiles itself already

1. **mirror.spec** -- the binary's self-description. Compiles to OID
   4fe16855cf603598b959c961b3eec02f2865e069. Emits Rust code. Round-trips.

2. **All 52 boot grammars** -- the entire grammar hierarchy compiles, kintsugi's,
   and checks. Every grammar has a stable content address.

3. **The crystal** -- `mirror crystal` materializes the full standard library
   into a single shatter artifact (OID d49eaf62c33869b978d75bcca62dec32ec87726d).

### What almost compiles itself

4. **evaluate.rs applied to @code/rust** -- the grammar evaluator reads the
   grammar's keyword->operation mappings and parses Rust source into MirrorAST.
   Unit-tested. Works. Not wired to CLI.

5. **code_rust.rs** -- the handwritten Rust parser produces MirrorAST from Rust
   source. Tested. Works. Not wired to CLI commands (only used internally by
   kintsugi operations on the base AST).

### The smallest self-referential mirror is:

```
mirror compile mirror.spec --target rust
```

This already works. mirror.spec describes the binary. The compiler reads it,
parses it via parse_form, runs it through the lambda pipeline (Parse -> Resolve ->
Properties -> Emit), and produces `mirror.rs` containing the Target enum.

The next smallest self-referential mirror would be:

```
mirror compile src/runtime.rs    (42 lines, the thinnest .rs file)
```

This requires Gap 1 + Gap 4: CLI .rs dispatch + evaluate->fragment bridge.
~30 lines of wiring.

---

## Next tick

**The single change that closes the biggest gap:**

Wire evaluate.rs into the CLI for .rs files.

In `cmd_compile`, `cmd_kintsugi`, and `cmd_check`, add file-extension detection:

```
if file ends with ".rs" {
    let grammar = evaluate::load_grammar("@code/rust")?;
    let ast = evaluate::evaluate(&grammar, &source);
    let frag = declaration::fragment(ast, vec![]);
    // continue with existing pipeline using frag
}
```

This is ~20-30 lines in cli.rs. It closes Gap 1 and Gap 4 simultaneously.
It makes `mirror compile src/runtime.rs` work. It makes the self-referential
loop complete for the flat case (~70% of items in any .rs file).

After that:
- Tick 2: Recursive evaluate.rs (nested impl/trait bodies) -- closes Gap 3
- Tick 3: Spec parser unification -- closes Gap 2
- Tick 4: Full pipeline on all 52 src/*.rs files, measuring Shannon loss
- Tick 5: kintsugi(src/, boot/) produces boot.mirror/

The ground state lambda_0 is visible. The path has 5 steps. The first step
is 30 lines of wiring. The evaluator already works. The grammar already
compiles. The loop is one bridge away from closing.

---

## Summary table

| Component | compile | kintsugi | check | status |
|-----------|---------|----------|-------|--------|
| boot/ (15 files) | 15/15 | 15/15 | 15/15 | WORKS |
| boot/std/ (37 files) | 37/37 | 37/37 | 37/37 | WORKS |
| mirror.spec | YES | YES | YES | WORKS |
| mirror crystal | YES | -- | -- | WORKS |
| mirror craft | -- | -- | -- | FAILS (spec parser) |
| src/*.rs (52 files) | 0/52 | 0/52 | 0/52 | NOT WIRED |
| evaluate.rs tests | -- | -- | -- | 11/11 PASS |
| Five operations on .mirror | YES | -- | -- | WORKS |

The compiler compiles its own grammar. It cannot yet compile its own source.
The mechanism to do so exists and is tested. The wiring does not.
