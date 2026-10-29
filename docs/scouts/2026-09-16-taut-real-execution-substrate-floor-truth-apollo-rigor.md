# Real-Execution Substrate Floor-Truth (Apollo Rigor)

**Author**: Taut <taut@systemic.engineer>
**Date**: 2026-09-16
**Charter**: Grep-verified ground truth on what LITERALLY executes today at Rust code altitude. No composition, no math altitude, no shard-decl altitude. File:line anchors for every claim.

**Discipline**: Distinguishes LANDED (real body with substantive logic that runs) vs SCAFFOLD (`todo!()`/`unimplemented!()`/empty/single-println/pattern-guarded-not-called) vs ABSENT (does not exist) vs MIXED. Where a function claims to call LAPACK, the actual `unsafe { ffi_name(...) }` call is cited. Where a dispatcher claims to match sentinels, the actual `match` arm is cited.

---

## §1 — Entry point: `fn main()` for the mirror binary

**Question**: Where is `main()` for `mirror`? Which crate + file + line? Full function body verbatim.

**Grep result**: Single binary declared in `/Users/alexwolf/dev/projects/mirror/rust/Cargo.toml:22`:

```toml
[[bin]]
name = "mirror"
path = "src/main.rs"
```

`fn main()` at `/Users/alexwolf/dev/projects/mirror/rust/src/main.rs:2461` (verbatim):

```rust
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // argv[0] is the binary path; verb is argv[1].
    match args.get(1).map(String::as_str) {
        Some("--help") | Some("-h") | Some("help") | None => { print_help(); ExitCode::SUCCESS }
        Some("--version") | Some("-V") | Some("version") => { print_version(); ExitCode::SUCCESS }
        Some("roomba") => { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_roomba(&rest) }
        Some("serve") =>  { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_serve_mcp(&rest) }
        Some("craft") =>  { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_craft(&rest) }
        Some("compile") => { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_compile(&rest) }
        Some("bumblebee") => { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_bumblebee(&rest) }
        Some("love") =>   { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); cmd_love(&rest) }
        Some("bench") =>  { let rest: Vec<String> = args.iter().skip(2).cloned().collect(); bench::cmd_bench(&rest) }
        Some(other) => { /* known-verb-but-undispatched OR unknown-verb → ExitCode::from(2) */ }
    }
}
```

Only **8** of the 15 advertised verbs have a `cmd_*` dispatch arm. The remainder fall into the `Some(other)` arm which prints "substrate-decl'd but dispatch lands at M3+" and returns exit 2.

**Verdict**: LANDED. The entry point exists, compiles, and dispatches 8 verbs.

---

## §2 — CLI dispatch: real-vs-scaffold verdict per subcommand

**Question**: For each of the 15 advertised verbs (`VERBS` const at `main.rs:159-173`), which `cmd_*` exists and does what?

Advertised verb list (`main.rs:159-173`, verbatim):
- `compile`, `kintsugi`, `shatter`, `craft`, `init`, `recall`, `beam`, `peer beam`, `peer contribute`, `index`, `roomba`, `serve`, `bumblebee`, `bench`, `love`

Grep result — dispatched verbs:

| Verb | Dispatch site | `cmd_*` definition | Verdict |
|---|---|---|---|
| `compile` | main.rs:2504 | `cmd_compile` main.rs:2117 | LANDED — reads file, parses `bilateral`/`property` blocks (line-scanner in `spectral::liquid`), dispatches via `compile::compile_from_source` producing a SAGA chain of Crystals. |
| `craft` | main.rs:2494 | `cmd_craft` main.rs:1876 | LANDED — reads spec, byte-checks for `system @` / `project ` prefix, shells out to `cargo build` via `phone::spawn_cargo_build`. |
| `roomba` | main.rs:2480 | `cmd_roomba` main.rs:743 | LANDED — walks a directory, classifies files by extension, executes mend-collapse per `.rs` file, deposits pheromone observation crystal, appends to bauchladen. |
| `serve` | main.rs:2482 | `cmd_serve_mcp` main.rs:411 | LANDED — MCP JSON-RPC loop over stdin/stdout; dispatches `initialize` / `tools/list` / `tools/call`; only 1 tool exposed: `mirror_query`. |
| `bumblebee` | main.rs:2513 | `cmd_bumblebee` main.rs:2213 | LANDED (dry-run only) — parses `--buzz-target`; `--execute` returns exit 3 with "not yet implemented at MVP altitude". |
| `bench` | main.rs:2540 | `bench::cmd_bench` bench.rs:73 | LANDED — `std::time::Instant` sampling loop; prints per-sample overhead statistics. |
| `love` | main.rs:2527 | `cmd_love` main.rs:2374 | LANDED — reads 3 shape descriptors from argv; runs `love::love()` composition; emits FullSpectrum verdict. |

Grep result — undispatched (advertised) verbs:

| Verb | Grep for `Some("<verb>")` in match arm | Verdict |
|---|---|---|
| `kintsugi` | zero matches in `main.rs` dispatch match | ABSENT — hits `Some(other)` arm at main.rs:2547; exit 2 with `"substrate-decl'd but dispatch lands at M3+"` message. |
| `shatter` | zero matches | ABSENT — same fallthrough. |
| `init` | zero matches | ABSENT — same fallthrough. |
| `recall` | zero matches | ABSENT — same fallthrough. |
| `beam` | zero matches | ABSENT — same fallthrough. |
| `peer beam` | zero matches (also `peer contribute`) | ABSENT — same fallthrough. |
| `index` | zero matches (only appears in help-table and docblocks) | ABSENT — same fallthrough. Alex-flagged: "Measure @fractal-coherence via Fiedler eigenvalue" is advertised but there is NO `cmd_index` function. |

**Verdict**: MIXED. 8 verbs LANDED, 7 verbs ABSENT (fallthrough to exit-2 stub).

---

## §3 — What happens when you run `mirror kintsugi ./mirror.spec` today?

**Question**: Trace the literal argv-parse → dispatch → completion path.

**Grep**: `main.rs:2461` argv match. `Some("kintsugi")` does not appear anywhere in the match block (main.rs:2465-2545). Falls through to `Some(other)` at main.rs:2547:

```rust
Some(other) => {
    let is_known = VERBS.iter().any(|(v, _)| *v == other || other == "peer");
    if is_known {
        eprintln!(
            "mirror: verb `{}` is substrate-decl'd but dispatch \
             lands at M3+ (see docs/specs/rust-floor-birthed-by-\
             roomba-from-mirror-spec.md §2.2).",
            other
        );
    } else {
        eprintln!("mirror: unknown verb `{}`", other);
        eprintln!();
        eprintln!("Run `mirror --help` for the verb list.");
    }
    ExitCode::from(2)
}
```

`kintsugi` IS in `VERBS` (main.rs:160), so `is_known` is `true`, so the message is `"mirror: verb \`kintsugi\` is substrate-decl'd but dispatch lands at M3+"` and the process exits with code 2. The spec file `./mirror.spec` is never opened, never parsed, never touched.

**Verdict**: SCAFFOLD (stub message → exit 2). Substrate flow does not execute. `mirror kintsugi ./mirror.spec` is inert at rust altitude.

---

## §4 — Does a `.mirror` parser exist?

**Question**: Which crate/file parses `.mirror` files? Is there a `parse` function producing an AST?

**Grep for `.mirror` consumption**:

1. `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs:105` — `walk_mirror_files` recursive walker (finds `.mirror` files under `shards/`).
2. `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs:114` — `std::fs::read_to_string(&path)` reads each `.mirror` file as a raw string.
3. `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs:176` — `extract_bilaterals(source, shard_ref)` — line-scan extractor for `bilateral <name> { sentinel "..."; arity N; }` blocks. NOT a grammar parser; it is `strip_prefix("bilateral ")` + `split_once('{')` + brace-terminated body.
4. `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs:268` — `extract_properties(source: &str) -> Vec<PropertyDecl>` — same shape line-scanner for `bilateral <name> { sentinel/arity/require }` blocks.
5. `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs:225` — `extract_spec_properties(source: &str) -> Vec<SpecProperty>` — line-scanner for `property <name> { verifies { <expr> } domain @<T> samples <n> defer? <msg> }` blocks (does track brace depth for nested `verifies { ... }` bodies).
6. `/Users/alexwolf/dev/projects/mirror/rust/src/main.rs:246-260` — `walk_mirror_files` (again) — enumerates `.mirror` files for shard-sheaf construction (MCP tools/call handler).
7. `/Users/alexwolf/dev/projects/mirror/rust/src/apply_h.rs:762+` — `enumerate_mirror_shards` — walker for `walk_p1_sugar_cascade`.

**No `parse_mirror` / `MirrorAst` / grammar-based parser exists**. Grep `pub fn parse_mirror` → zero hits. Grep `MirrorAst`/`MirrorNode`/`ShardAst` → zero hits.

There is no `include_str!("shards/...")` compile-time embedding.

**Verdict**: PARTIAL — three line-scanner extractors exist (`extract_bilaterals` in mend, `extract_properties` + `extract_spec_properties` in liquid), each targeting a specific block shape via `strip_prefix` and `find('{')`. Together they extract two kinds of declarations: `bilateral` (shard-body) and `property` (spec-body). Everything else in a `.mirror` file (docblocks, `prism { }`, `family`, `species`, `system`, `sentinel_reflects_*`, `focus`/`project`/`split`/`shift`/`settle`, etc.) is invisible to the Rust code — it is read as bytes and ignored.

**Verdict**: The pretense that mirror files are "parsed" is misleading. What exists is byte-pattern extraction of two named block shapes. No AST. No grammar. No error-recovery. Line-scanner artifacts silently misparse (e.g. a `bilateral` keyword inside a comment or string is not filtered in the shard-body extractor; only mend.rs applies a `#`-comment skip).

---

## §5 — AST type: definition file:line + type body

**Question**: If parser exists, what's the AST type?

**Grep result**: There is no unified AST. There are three carrier structs, each named after what its line-scanner extracts:

### 5a. `BilateralDecl` at `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs:35-46` (verbatim):

```rust
#[derive(Debug, Clone)]
pub struct BilateralDecl {
    /// Predicate name (e.g. `signature_integrity`).
    pub name: String,
    /// Sentinel byte-string (e.g. `chain=merkle-linked`).
    pub sentinel: String,
    /// Arity — number of args expected. 1 = base; 2+ = composed.
    pub arity: usize,
    /// Full action ref (e.g. `@spectral/signature.signature_integrity`).
    pub full_action_ref: String,
}
```

### 5b. `PropertyDecl` at `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs:109-124`:

```rust
pub struct PropertyDecl {
    pub name: String,
    pub sentinel: String,
    pub arity: usize,
    pub require: Vec<String>,
    pub source: String,  // shard path + line number
}
```

### 5c. `SpecProperty` at `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs` (extract_spec_properties return type) — struct definition not shown in the extracted region, but the extractor at liquid.rs:225 accumulates `name`, `verifies_source: String`, `domain_ref: String`, `samples: Option<usize>`, `defer_message: Option<String>`.

**Verdict**: MIXED. Three flat structs, no tree. `verifies_source` in `SpecProperty` is a raw String (unparsed expression); `sentinel` in `BilateralDecl`/`PropertyDecl` is a raw byte-string. None of these are AST nodes — they are extracted-field records.

---

## §6 — Parser absent: name the absence

No `.mirror` grammar parser exists at Rust altitude. There are line-scanner extractors for two block shapes (`bilateral { }` and `property { }`).

Docblock evidence at `main.rs:29-31`:

```
//! - Reflective cli-block reading of `mirror.spec` — Mara §5.2 item 4;
//!   M2 milestone (the M0-hardcoded verb list retires here).
```

The `VERBS` array itself is documented at `main.rs:155-158` as HARDCODED-at-M0 pending reflective derivation from `shards/**/*.mirror` + `mirror.spec` "at M2". M2 has not landed.

**Verdict**: A grammar-driven `.mirror` parser is forward-promised but ABSENT. Two block-shape line-scanners partially compensate for the compile verb and mend surface.

---

## §7 — prismqueer module inventory (sorted by size)

**Grep result** — `/Users/alexwolf/dev/projects/prism/prismqueer/src/*.rs` sorted by byte-size:

| Bytes | Path |
|--:|:---|
| 116 | fate/compiled.rs |
| 663 | pq/mod.rs |
| 851 | optics/mod.rs |
| 1.2KB | content.rs |
| 1.4KB | crystal_shard.rs |
| 1.5KB | question.rs |
| 1.8KB | model.rs |
| 1.9KB | pq/output.rs |
| 2.1KB | spectral/mod.rs |
| 2.1KB | fate/strategy.rs |
| 2.2KB | luminosity.rs |
| 2.3KB | choice.rs |
| 2.3KB | crystal.rs |
| 2.3KB | hypothesis.rs |
| 2.3KB | reality.rs |
| 2.5KB | optics/monoid.rs |
| 2.7KB | assertion.rs |
| 2.7KB | observation.rs |
| 3.0KB | connection.rs |
| 3.1KB | optics/gather.rs |
| 3.3KB | precision.rs |
| 3.5KB | pq/filter.rs |
| 3.6KB | substrate_ref.rs |
| 4.2KB | scalar_loss.rs |
| 4.3KB | optic_kind.rs |
| 4.5KB | store.rs |
| 4.9KB | chaos.rs |
| 5.1KB | optics/setter.rs |
| 5.1KB | optics/traversal.rs |
| 5.1KB | trace.rs |
| 5.2KB | recursion.rs |
| 5.3KB | pq/target.rs |
| 5.6KB | optics/fold.rs |
| 6.0KB | kernel.rs |
| 6.0KB | optics/iso.rs |
| 6.3KB | optics/lens.rs |
| 6.8KB | spectral/commutator.rs |
| 7.0KB | oid.rs |
| 7.5KB | fate/brainfuck/test_fate.rs |
| 7.6KB | fate/feature.rs |
| 7.7KB | optics/optic_prism.rs |
| 7.8KB | spectral_oid.rs |
| 8.4KB | fate/weights.rs |
| 8.5KB | fate/manifold.rs |
| 8.9KB | fate/metal_runtime.rs |
| 9.0KB | lambda/typed.rs |
| 9.6KB | spectral_uuid.rs |
| 9.8KB | shard.rs |
| 10.8KB | fate/train.rs |
| 11.0KB | lambda/reduce.rs |
| 11.0KB | merkle.rs |
| 11.5KB | named.rs |
| 11.5KB | pq/schema.rs |
| 11.6KB | metal.rs |
| 12.1KB | observer.rs |
| 12.6KB | spectral/harmonics.rs |
| 12.9KB | ffi.rs |
| 13.7KB | spectral/tension.rs |
| 14.0KB | spectral_dimension.rs |
| 15.1KB | lib.rs |
| 15.7KB | spectral/rotation.rs |
| 16.5KB | lambda/mod.rs |
| 23.8KB | beam.rs |
| 25.2KB | fate/runtime.rs |
| 25.8KB | fate/derive.rs |
| 27.5KB | coincidence.rs |
| 29.2KB | flux.rs |
| 29.4KB | spectral/kleinos.rs |
| 31.0KB | bundle.rs |
| 52.1KB | fate/mod.rs |

Total: 69 files. Largest: `fate/mod.rs` (52.1KB).

---

## §8 — `prismqueer::spectral::kleinos` — signature + body-status + LAPACK FFI

**Question**: Signature of main kleinos entry; first 50 lines; does body call LAPACK FFI?

**Grep result** at `/Users/alexwolf/dev/projects/prism/prismqueer/src/spectral/kleinos.rs`:

### Public entry: `pub fn kleinos(...)` at kleinos.rs:330 (verbatim signature):

```rust
pub fn kleinos(
    sheaf_a: &SheafOfShardGraph,
    sheaf_b: &SheafOfShardGraph,
) -> Imperfect<Green, Red, ConvergenceLoss> {
    let candidate = compose_candidate(sheaf_a, sheaf_b);
    let lambda_a = fiedler_lambda_2_of_sheaf(sheaf_a);
    let lambda_b = fiedler_lambda_2_of_sheaf(sheaf_b);
    // ... verify_all_four_properties + into_imperfect
}
```

### LAPACK call site: `fiedler_lambda_2_of_sheaf` at kleinos.rs:293-319:

```rust
pub fn fiedler_lambda_2_of_sheaf(sheaf: &SheafOfShardGraph) -> f64 {
    let n = sheaf.vertex_count();
    if n < 2 { return 0.0; }
    // ... build laplacian ...
    match crate::ffi::eigenvalues(n, &laplacian) {
        Ok(evals) if evals.len() >= 2 => evals[1],
        _ => 0.0,
    }
}
```

`crate::ffi::eigenvalues` at `/Users/alexwolf/dev/projects/prism/prismqueer/src/ffi.rs:215` performs an actual `unsafe { spectral_eigenvalues(...) }` call to the FLANG-compiled Fortran wrapper. See §19 for the raw extern block.

**Verdict**: LANDED — real body, real FFI, real math. The `kleinos` function builds a candidate composed sheaf, computes three Fiedler eigenvalues (each via a real LAPACK dsyev call), verifies four properties (sovereignty / emergent-third / fiedler-rise / fusion-refusal), returns `Imperfect<Green, Red, ConvergenceLoss>`.

Zero `todo!()`, zero `unimplemented!()` in kleinos.rs body (grep-verified).

---

## §9 — `prismqueer::spectral::rotation` — signature + body-status + FFI

**Question**: `fn infer_via_rotation` signature + body-status + LAPACK FFI presence.

**Grep result** at `/Users/alexwolf/dev/projects/prism/prismqueer/src/spectral/rotation.rs`:

### Public entry at rotation.rs:169-180 (verbatim signature):

```rust
pub fn infer_via_rotation(
    loops: &[SheafOfShardGraph],
    ticks: usize,
    config: RotationConfig,
) -> Imperfect<Vec<Splinter>, Red, ConvergenceLoss> {
    // Phase 1 requires exactly 2 loops (kleinos K_2→K_3 base case).
    // Phase 2+ will iterate N loops per wedge-at-K_n.
    ...
}
```

### FFI imports at rotation.rs:47-49 (verbatim):

```rust
use crate::ffi::phase_lock;
use crate::oid::{Addressable, Oid};
use crate::spectral::{kleinos, Property, Red, SheafOfShardGraph};
```

`ffi::phase_lock` at `/Users/alexwolf/dev/projects/prism/prismqueer/src/ffi.rs:376+` performs `unsafe { spectral_phase_lock(...) }` — real Fortran integration loop.

**Verdict**: LANDED — real body composes over `kleinos` (LAPACK dsyev) + `phase_lock` (LAPACK-adjacent Fortran integration). Zero `todo!()`/`unimplemented!()` in rotation.rs.

---

## §10 — `prismqueer::spectral::tension` — signature + body + FFI

**Grep result** at `/Users/alexwolf/dev/projects/prism/prismqueer/src/spectral/tension.rs`:

### Public entry at tension.rs:124-127:

```rust
pub fn detect_tensions(
    sheaf: &SheafOfShardGraph,
) -> Imperfect<Vec<Tension>, Red, ConvergenceLoss> {
```

### FFI imports at tension.rs:47:

```rust
use crate::ffi::eigenvalues;
```

Empty-sheaf early return at tension.rs:129-137; otherwise the body computes the graph Laplacian spectrum via `crate::ffi::eigenvalues` and classifies tensions by kind (`FusionRefusal` / `Disconnection` / etc.).

**Verdict**: LANDED — real body, calls LAPACK dsyev. Zero `todo!()`/`unimplemented!()` in tension.rs.

---

## §11 — `prismqueer::liquid` — psi carrier, landau, weak_measurement, anna_wolf

Note: `prismqueer::liquid` at prismqueer altitude does not exist as a separate `liquid.rs`. The name is spread across:

1. `/Users/alexwolf/dev/projects/prism/prismqueer/src/flux.rs` (29.2KB) — carries the `FluxThread` trait (formerly Liquid). Docblock at flux.rs:1 says "Liquid — property verdicts over the spectral commutator." So `flux.rs` IS the renamed liquid module at prismqueer altitude.
2. `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs` (109.9KB) — at mirror-repo altitude (spectral crate), not prismqueer.

**Grep for `psi` in prismqueer**: zero occurrences of `pub fn psi` or `pub struct Psi` in `prism/prismqueer/src/**/*.rs`.

**Grep for `landau`**: zero occurrences (no `pub` items).

**Grep for `weak_measurement`**: zero occurrences.

**Grep for `anna_wolf`**: zero occurrences as an item name; appears only in docblock comments (rotation.rs docblock cites Anna Jakobs 2012).

**flux.rs main entry** at `/Users/alexwolf/dev/projects/prism/prismqueer/src/flux.rs` — `trait FluxThread: Transport` with `fn commutator_magnitude(a, b, state) -> Holonomy`. Blanket-implemented for any `Transport` type.

**Verdict**: MIXED. The renamed `liquid` module is `flux.rs`; `FluxThread` trait is defined with real trait bodies composing over the bundle-tower `Transport` supertrait. However, none of the names Alex asked about (`psi`, `landau`, `weak_measurement`, `anna_wolf`) exist as identifiers in prismqueer source. They exist only in docblock references. If Anna Jakobs ψ apparatus was expected as executable code, it is ABSENT.

---

## §12 — `prismqueer::reality` full body

**File**: `/Users/alexwolf/dev/projects/prism/prismqueer/src/reality.rs` (2.3KB). Full body verbatim (67 lines):

```rust
//! `prismqueer::reality` — Reality sum-type per Alex Move 16.
//! ... (docblock)

use crate::crystal_shard::Crystal;
use crate::shard::Shard;

#[derive(Clone, Debug)]
pub enum Reality<T> {
    /// Disconnected Shards; H^0 > 1; parts not yet composable.
    Fractured(Vec<Shard<T>>),
    /// Crystallized settled Shard; content-addressed via Oid.
    Settled(Crystal<T>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reality_fractured_composes_disconnected_shards() {
        let shards: Vec<Shard<&[u8]>> = vec![Shard::new(b"a"), Shard::new(b"b")];
        let reality: Reality<&[u8]> = Reality::Fractured(shards);
        match reality {
            Reality::Fractured(v) => assert_eq!(v.len(), 2),
            _ => panic!("expected Fractured"),
        }
    }

    #[test]
    fn reality_settled_composes_crystal() {
        let reality: Reality<&[u8]> = Reality::Settled(Crystal::new(b"c"));
        match reality {
            Reality::Settled(_) => {}
            _ => panic!("expected Settled"),
        }
    }
}
```

**Verdict**: SCAFFOLD — a pure sum-type declaration with two constructors and two round-trip pattern-match tests. No methods, no compose logic, no observation function. Only the shape is landed. `Flux<Reality>` (Move 17) is referenced in docblock as forward-promised — not landed here.

Same shape holds for sibling `prismqueer::model` (`Vec<Shard<T>>` newtype wrapper; 1.8KB), `prismqueer::observation` (`{ crystal, chaos }` product type; 2.7KB), `prismqueer::assertion` (`{ observation, model }`; 2.7KB), `prismqueer::hypothesis` (KTQuestionShape enum; 2.3KB), `prismqueer::question` (`{ hypothesis, chaos }`; 1.5KB), `prismqueer::recursion` (`{ reality }` newtype + `tick()`; 5.2KB). All are shape-only pipeline scaffolds authored during Reed 2026-09-04 TICK B.

---

## §13 — `apply_h::act` — definition + call sites

**Definition**: `/Users/alexwolf/dev/projects/mirror/rust/src/apply_h.rs:118` (verbatim):

```rust
pub fn act(root: &Path, action_ref: &str, args: &[String]) -> Verdict {
    // Fire E M-E2: shard-body-projector dispatch
    if let Some(species) = strip_fracture_detect_ref(action_ref) {
        return dispatch_fracture_detect(root, species, args);
    }

    // Fire E M-E3.5-REVISED: beta-reduction-rule reducible dispatch
    if let Some(species) = strip_normalization_rule_reducible_ref(action_ref) {
        return dispatch_reduction_rule_reducible(root, species, args);
    }

    let corpus = load_bilateral_corpus(root);
    let decl: &BilateralDecl = match corpus.get(action_ref) {
        Some(d) => d,
        None => return Verdict::Fail(format!(
            "apply_h::act: action_ref `{}` not found in bilateral corpus at `{}`",
            action_ref, root.display()
        )),
    };

    let args_joined = args.join(" ");
    if args_joined.contains(&decl.sentinel) {
        Verdict::Pass
    } else {
        Verdict::Fail(format!(
            "apply_h::act: sentinel `{}` not found in args `{}` for `{}`",
            decl.sentinel, args_joined, action_ref
        ))
    }
}
```

**Call sites** (files + match count):

| File | Call-count for `apply_h::act` |
|---|---|
| `/Users/alexwolf/dev/projects/mirror/rust/src/apply_h.rs` | 12 (mostly in-file tests + `dispatch_fracture_detect` sub-routing) |
| `/Users/alexwolf/dev/projects/mirror/rust/src/love.rs` | 3 (docblock references + one comparison test) |
| `/Users/alexwolf/dev/projects/mirror/rust/src/main.rs` | 14 (invoked in `response_tools_call` for `mirror_query` MCP dispatch; the ONLY external caller invoking `act()` on real user input) |
| `/Users/alexwolf/dev/projects/mirror/rust/src/phone.rs` | 2 (docblock references) |
| `/Users/alexwolf/dev/projects/mirror/rust/src/wire.rs` | 2 (docblock references) |
| `/Users/alexwolf/dev/projects/mirror/rust/tests/red_spec_claims.rs` | 7 (tests) |

Call site with actual dispatch: `main.rs` inside `response_tools_call` — the MCP `tools/call` handler for `mirror_query`. Confirmed at main.rs line region ~610:

```rust
let verdict = apply_h::act(root, action_ref, &mq_args);
let (text, is_error) = match verdict {
    apply_h::Verdict::Pass => (format!("Pass: apply_h::act({}, {:?}) discharged over substrate at {}", ...), false),
    apply_h::Verdict::Fail(reason) => (format!("Fail: {}", reason), true),
};
```

**Verdict**: LANDED, ACTIVE. `apply_h::act` is on the executable path when `mirror serve --mcp` receives a `tools/call` for `mirror_query`. It performs (a) special-case dispatch for `@kintsugi/fracture/<species>.detect` and `@epistemologic/normalization/rules/<species>.reducible` refs, otherwise (b) loads the bilateral corpus by walking `<root>/shards/**/*.mirror` on every call, and (c) checks the sentinel byte-string is contained in the args-joined string. Return is `Verdict::Pass` (sentinel found) or `Verdict::Fail(reason)`.

**Note**: This is NOT what most `.mirror` file bilateral bodies express. Most bilateral bodies encode complex semantic invariants ("chain=merkle-linked", "scope=universal", etc.); the `apply_h::act` sentinel check merely verifies the caller's args contain the literal sentinel string. It is a syntactic ceremony, not a semantic evaluator.

---

## §14 — `CoincidenceHash<3>` — definition + use sites + module docstring

**Grep**: `CoincidenceHash` appears in 4 files under `prism/prismqueer/src/`:

1. `/Users/alexwolf/dev/projects/prism/prismqueer/src/named.rs:1` — 1 match.
2. `/Users/alexwolf/dev/projects/prism/prismqueer/src/observer.rs:3` — 3 matches.
3. `/Users/alexwolf/dev/projects/prism/prismqueer/src/oid.rs:4` — 4 matches.
4. `/Users/alexwolf/dev/projects/prism/prismqueer/src/shard.rs:4` — 4 matches.

**Grep `pub struct CoincidenceHash`**: zero matches. The identifier `CoincidenceHash` does NOT declare a struct in the current codebase; it appears only as a REFERENCE in docblock comments.

**Actual coincidence primitive** at `/Users/alexwolf/dev/projects/prism/prismqueer/src/coincidence.rs:1-8` (module docstring, verbatim):

```
//! Minimal coincidence hash — eigenvalue-based content addressing.
//!
//! Ported from the coincidence crate. This is the minimal code path:
//! bytes -> StateVector -> N projections -> Detection -> eigenvalue hex.
//!
//! N=3 is the canonical detector for Oid::hash(). Three independent observers,
//! deterministic projections from SHA-256 seeds.
```

The real type is `Detector<const N: usize>` (a hasher parameterized by projection count). Public entries at coincidence.rs:479-497:

```rust
pub fn canonical_hash(bytes: &[u8]) -> String {
    CANONICAL.review(bytes)
}
pub fn coincidence_hash() -> crate::named::Named<Detector<3>> { ... }
```

**Verdict**: The name `CoincidenceHash<3>` is legacy nomenclature persisting only in docblocks; the executing type is `Detector<3>` in coincidence.rs. Alex's suspicion is empirically confirmed: `CoincidenceHash` as a struct is ABSENT from the current code; the concept survives as `Detector<3>` (LANDED) with real body (525+ lines including SHA-256-seeded projections, StateVector, and eigenvalue-hex output).

---

## §15 — `bootstrap/` directory status

**Grep for `/Users/alexwolf/dev/projects/mirror/bootstrap/**`**: does not exist.

**Verdict**: ABSENT. The `bootstrap/` directory referenced in the scout charter has been removed from the repository. Historical citations in `rust/Cargo.toml` docblock reference "the bootstrap's release profile shape" and "bootstrap's byte-substring semantics" (rust/Cargo.toml:82, apply_h.rs:97), but the bootstrap crate itself no longer exists on disk. It is now DOCUMENTATION-ONLY reference and not on any executable path.

---

## §16 — Shard-decl consumption: any Rust code reading `shards/*.mirror`?

**Grep result for reading `.mirror` files at runtime**:

1. `/Users/alexwolf/dev/projects/mirror/rust/roomba/src/mend.rs:113-114` — `std::fs::read_to_string(&path)` inside `load_bilateral_corpus`. Reads every `.mirror` file under `<root>/shards/` to build a `HashMap<String, BilateralDecl>` corpus. This corpus is consumed by `apply_h::act` on every MCP `tools/call` (main.rs → apply_h.rs:180). CHANGES RUNTIME BEHAVIOR: if a `.mirror` file declares `bilateral <name> { sentinel "foo"; }`, that name becomes a dispatchable action_ref via `apply_h::act`.

2. `/Users/alexwolf/dev/projects/mirror/rust/src/apply_h.rs:326-359` — `detect_prism_boilerplate_at(shard_path)` reads a specific `.mirror` file via `phone::read_file(shard_path)` and byte-checks for the P1 identity-carrier prism block. Consumed by `dispatch_fracture_detect` when action_ref is `@kintsugi/fracture/prism_boilerplate.detect`.

3. `/Users/alexwolf/dev/projects/mirror/rust/src/compile.rs:220+` — reads a single spec file via `phone::read_file`, then delegates to `spectral::liquid::extract_properties` + `extract_spec_properties`. Consumed by `mirror compile <file>`.

4. `/Users/alexwolf/dev/projects/mirror/rust/src/main.rs:246-260` — `walk_mirror_files` for shard-sheaf construction (MCP `tools/call` builds a substrate-sheaf from directory topology; each `.mirror` file becomes a vertex, sibling files share edges).

**Grep for `include_str!("shards/`**: zero matches. No compile-time shard embedding.

**Verdict**: LANDED — `.mirror` file contents DO change runtime behavior via `load_bilateral_corpus` → `apply_h::act` sentinel dispatch AND via `mirror compile` byte-scan. However, this behavior is limited to two block shapes (`bilateral` and `property`); the rest of a `.mirror` file's contents (docblocks, prism blocks, family/species/system declarations) is read as bytes and semantically ignored, EXCEPT where the P1-prism-boilerplate detector byte-checks for a specific pattern.

---

## §17 — Bilateral sentinel dispatch: actual match arms

**Question**: Any Rust code pattern-matching on sentinel strings like `"sheaf=stalks-implement-declared-trait"`?

**Grep for hardcoded sentinel literals** in `/Users/alexwolf/dev/projects/mirror/rust/**/*.rs`:

No Rust source file contains match arms on literal sentinel strings like `"sheaf=stalks-implement-declared-trait"` or `"reality=event-well-formed"`. The sentinel-dispatch pattern in `apply_h::act` is DATA-DRIVEN — sentinels are read from `.mirror` files into the `BilateralDecl.sentinel` field at runtime, and the dispatch is `args.join(" ").contains(&decl.sentinel)` (mend.rs:206, apply_h.rs:172).

There IS a `match name` sentinel-dispatch in `/Users/alexwolf/dev/projects/mirror/rust/spectral/src/liquid.rs:977-1020`:

```rust
pub fn enact(name: &str, args: &[String]) -> Verdict {
    match name {
        "paradox_crystal_immutable" => paradox_crystal_immutable(args),
        "redirect_witness_preserves_original_pin" => redirect_witness_preserves_original_pin(args),
        "classifier_lagrange_between_narcissus_and_splinter" => classifier_lagrange_between_narcissus_and_splinter(args),
        "aikido_sequence_well_formed" => aikido_sequence_well_formed(args),
        "consent_through_refusal" => consent_through_refusal(args),
        "intervention_composes_without_deleting_trauma" => intervention_composes_without_deleting_trauma(args),
        "crystallization_preserves_saga" => crystallization_preserves_saga(args),
        "autopoietic_lawvere_fixed_point" => autopoietic_lawvere_fixed_point(args),
        "vsm_invariants" => vsm_invariants(args),
        "void_settle_produces_lens_seed" => void_settle_produces_lens_seed(args),
        _ => Verdict::Defer(format!("pillar `{}` not yet registered; iter 7+ authorship territory ...", name)),
    }
}
```

**Verdict**: MIXED. There are 10 hardcoded predicate-name match arms in `pillar::enact` (each dispatching to a per-name function in the `pillar` mod). All other bilateral names return `Verdict::Defer` (a soft-fail). Sentinel-STRING dispatch is data-driven (read from `.mirror` bodies at runtime); predicate-NAME dispatch (in `enact_property` when `compile_from_source` runs) is hardcoded to these 10 names. Everything outside these 10 predicate names is Defer-only.

---

## §18 — `apply_h` sentinel dispatch (historical): current state

**Grep for `apply_h::act` retirement**: no evidence of retirement. Current definition at apply_h.rs:118 is live and called from `main.rs` MCP dispatch path.

**Verdict**: `apply_h::act` LANDED and ACTIVE. Not retired. Serves as the ONE tool endpoint for `mirror serve --mcp`'s `mirror_query` tool.

---

## §19 — LAPACK FFI reality: `rust/matrix/src/lib.rs` extern "C" blocks + unsafe calls

**Question**: Which LAPACK/BLAS symbols are actually bound? Which are called?

### 19a. matrix crate `rust/matrix/src/lib.rs`:

**Grep for `extern "C"` in matrix/src/lib.rs**: zero matches. The matrix crate contains NO direct extern-C declarations of its own. It delegates every LAPACK primitive to `prismqueer::ffi`.

Explicit delegate at `/Users/alexwolf/dev/projects/mirror/rust/matrix/src/lib.rs:210-214` (verbatim):

```rust
pub fn eigenvalues(n: usize, matrix: &[f64]) -> Vec<f64> {
    prismqueer::ffi::eigenvalues(n, matrix)
        .unwrap_or_else(|info| panic!("LAPACK dsyev convergence failed: info={info}"))
}
```

Same pattern for `phase_lock` (matrix/src/lib.rs:240) and `envelope` (matrix/src/lib.rs:262).

### 19b. prismqueer FFI `prism/prismqueer/src/ffi.rs`:

Actual `extern "C"` block at `/Users/alexwolf/dev/projects/prism/prismqueer/src/ffi.rs:16-83` (verbatim symbol list):

```rust
extern "C" {
    fn prism_preview(n, projection, source, focus, matched);
    fn prism_review(n, projection, focus, result);
    fn prism_modify(n, projection, source, transform, result);
    fn prism_compose(n, p1, p2, composed);
    fn spectral_eigenvalues(n, matrix, eigenvalues, info);
    fn spectral_eigensystem(n, matrix, eigenvalues, eigenvectors, info);
    fn spectral_singular_values(m, n, matrix, singular_values, info);
    fn spectral_svd(m, n, matrix, singular_values, u, vt, info);
    fn spectral_phase_lock(n, phases_in, omegas, k, steps, dt, phases_out, order_r, info);
}
```

All 9 symbols are Fortran wrappers around LAPACK primitives (`dsyev` / `dgesvd` for eigenvalues+SVD, custom Kuramoto integration).

Safe wrappers at ffi.rs (each performs `unsafe { fortran_name(...) }`):

| Symbol | Rust wrapper fn | Definition | Real `unsafe` call site |
|---|---|---|---|
| `prism_preview` | `preview` | ffi.rs:127 | ffi.rs:132 `unsafe { prism_preview(...) }` |
| `prism_review` | `review` | ffi.rs:152 | ffi.rs:156 `unsafe { prism_review(...) }` |
| `prism_modify` | `modify` | ffi.rs:169 | ffi.rs:174 `unsafe { prism_modify(...) }` |
| `prism_compose` | `compose` | ffi.rs:186 | ffi.rs:191 `unsafe { prism_compose(...) }` |
| `spectral_eigenvalues` | `eigenvalues` | ffi.rs:215 | ffi.rs:222 `unsafe { spectral_eigenvalues(...) }` |
| `spectral_eigensystem` | `eigensystem` | ffi.rs:245 | ffi.rs:253 `unsafe { spectral_eigensystem(...) }` |
| `spectral_singular_values` | `singular_values` | ffi.rs:283 | ffi.rs:291 `unsafe { spectral_singular_values(...) }` |
| `spectral_svd` | `svd` | ffi.rs:314 | ffi.rs:323 `unsafe { spectral_svd(...) }` |
| `spectral_phase_lock` | `phase_lock` | ffi.rs:370 | ffi.rs:381 `unsafe { spectral_phase_lock(...) }` |

### 19c. Is-called-anywhere status:

- `ffi::eigenvalues` — CALLED by `matrix::eigenvalues` (matrix/src/lib.rs:211) AND by `prismqueer::spectral::kleinos::fiedler_lambda_2_of_sheaf` (kleinos.rs:315) AND by `prismqueer::spectral::tension::detect_tensions` (tension.rs body).
- `ffi::phase_lock` — CALLED by `matrix::phase_lock` AND by `prismqueer::spectral::rotation::infer_via_rotation`.
- `ffi::eigensystem`, `ffi::singular_values`, `ffi::svd` — LANDED as safe wrappers; whether they are called elsewhere requires deeper grep beyond scout scope.
- `ffi::preview` / `review` / `modify` / `compose` — LANDED as safe wrappers; no evidence of active call sites from mirror bin's execution path (used inside optics/prism modules where feature-gated).

**Verdict**: LANDED. LAPACK FFI is real, symbol bindings are declared, `unsafe { ... }` invocations exist, and at least 2 of the 9 symbols (`spectral_eigenvalues`, `spectral_phase_lock`) are on the executable path from `mirror serve --mcp` today.

---

## §20 — End-to-end LAPACK invocation: does any user-facing path today reach the FFI?

**Traced path** (grep-verified, from argv-parse to `unsafe { spectral_eigenvalues(...) }`):

1. User invokes `mirror serve --mcp`.
2. `main()` at main.rs:2461 matches `Some("serve")` → `cmd_serve_mcp` at main.rs:411.
3. `cmd_serve_mcp` requires `--mcp` flag, then calls `serve_loop(root)` at main.rs:463.
4. `serve_loop` reads one JSON-RPC frame per tick via `phone::read_stdin_frame`, parses via `wire::parse`, dispatches on `method`.
5. On `"tools/call"` for `mirror_query`, `response_tools_call` is invoked (main.rs, around line 590).
6. `response_tools_call` parses the MQ expression, then:
   - calls `apply_h::act(root, action_ref, &mq_args)` (main.rs, around line 610).
   - calls `build_shards_sheaf(root)` + `build_session_sheaf(&session_splinters)` (main.rs:189-233).
   - calls `infer_via_rotation(&loops, ticks, config)` — prismqueer::spectral::rotation.
     - `infer_via_rotation` internally calls `kleinos(sheaf_a, sheaf_b)` at kleinos.rs.
       - `kleinos` calls `fiedler_lambda_2_of_sheaf(sheaf_a)` at kleinos.rs:315.
         - **`fiedler_lambda_2_of_sheaf` calls `crate::ffi::eigenvalues(n, &laplacian)` at kleinos.rs:315.**
           - **`ffi::eigenvalues` calls `unsafe { spectral_eigenvalues(...) }` at ffi.rs:222.**
             - **Fortran runs. LAPACK dsyev fires.**
   - calls `detect_tensions(&sheaf)` — same LAPACK dsyev chain.

**Verdict**: LANDED — an end-to-end path from `mirror serve --mcp` argv to LAPACK dsyev via Fortran exists TODAY. Executes on every valid `mirror_query` MCP tools/call invocation.

HOWEVER: this path only fires under `mirror serve --mcp`. The `mirror compile`, `mirror craft`, `mirror roomba`, `mirror bumblebee`, `mirror love`, and `mirror bench` verbs do NOT hit LAPACK from their execution flow (grep-verified: none of their `cmd_*` bodies call `matrix::eigenvalues` or `prismqueer::ffi::*` directly or transitively). LAPACK is on the executable path for MCP tools/call ONLY.

---

## §21 — Summary table

| Item | Status | Blocking for "compiler actually compiles"? |
|---|---|---|
| 1. `fn main()` entry point | LANDED | No |
| 2. CLI dispatch (8 of 15 verbs wired) | MIXED | Partial — 7 advertised verbs are stubs |
| 3. `mirror kintsugi ./mirror.spec` execution | SCAFFOLD (exit 2 with message) | YES — the flagship spec-settling verb does nothing |
| 4. `.mirror` grammar parser | ABSENT (only 3 line-scanners for 2 block shapes) | YES — no AST, no error recovery, no whole-file semantics |
| 5. `.mirror` AST type | ABSENT (3 flat extraction structs, no tree) | YES — Rust code cannot understand a `.mirror` file's structure beyond `bilateral`/`property` blocks |
| 6. Parser absence documented as forward-promised M2 | forward-promised | YES |
| 7. prismqueer inventory (69 files) | catalogued | No |
| 8. `kleinos` — signature + LAPACK dsyev call | LANDED | No |
| 9. `infer_via_rotation` — LAPACK + phase-lock | LANDED | No |
| 10. `detect_tensions` — LAPACK dsyev | LANDED | No |
| 11. `prismqueer::liquid` (renamed `flux.rs`); no `psi`/`landau`/`weak_measurement`/`anna_wolf` | MIXED — trait landed, named apparati ABSENT | YES for Move 2/4/8 pipeline honesty |
| 12. `prismqueer::reality` (67-line enum, tests only) | SCAFFOLD | YES — shape-only; no compose logic |
| 13. `apply_h::act` definition + call sites | LANDED, ACTIVE on MCP path | No |
| 14. `CoincidenceHash<3>` — struct ABSENT; `Detector<3>` LANDED | LEGACY-NAMED / ALIASED | No |
| 15. `bootstrap/` directory | ABSENT (removed from repo) | No |
| 16. Shard-decl runtime consumption (bilateral corpus + P1 detector) | LANDED for 2 block shapes | YES — most of a `.mirror` file is byte-invisible to Rust |
| 17. Bilateral sentinel dispatch (data-driven; 10 hardcoded pillar predicates) | LANDED for 10 predicate names | YES — 10 predicates is a tiny fraction of substrate-decl'd bilaterals |
| 18. `apply_h` current state | LANDED, ACTIVE | No |
| 19. LAPACK FFI: 9 extern-C symbols, all wrapped, 2 on hot path | LANDED | No |
| 20. End-to-end LAPACK invocation via `mirror serve --mcp` | LANDED (only via MCP path) | No |

---

### Bottom-line: does the current mirror binary compile a `.mirror` file end-to-end today?

**NO.** The best the current binary can do with a `.mirror` file is:

- `mirror compile <file>` — line-scan for `bilateral` + `property` blocks, dispatch each name through 10 hardcoded pillar predicates (returning `Defer` for anything not in the 10-name table), emit a SAGA chain of Crystals. This is a partial predicate-evaluator over 2 block shapes, not a compiler.
- `mirror craft <file>` — byte-check the file contains `system @` OR `project ` prefix, then shell out to `cargo build`. This does not parse or evaluate the spec — it forwards to cargo.
- `mirror serve --mcp` → `mirror_query` MCP tool → `apply_h::act(root, action_ref, args)` — walk `<root>/shards/**/*.mirror`, extract every `bilateral` block, check if the incoming action_ref matches one, then substring-match the sentinel against args-joined. This is a bilateral-corpus lookup with substring dispatch, not compilation.
- `mirror kintsugi ./mirror.spec` — exits with code 2 and message "substrate-decl'd but dispatch lands at M3+". Does nothing.

### Top-3 blockers to "compiler actually compiles"

1. **No grammar parser.** Three line-scanners extract 2 block shapes (`bilateral { }`, `property { }`). Everything else in a `.mirror` file — docblocks, `family`, `species`, `system`, `prism { }`, `focus`/`project`/`split`/`shift`/`settle`, sentinel_reflects_* fields, in @X imports, composition-shard bodies — is byte-invisible at Rust altitude. The forward-promised M2 "reflective cli-block reading of `mirror.spec`" and grammar-driven parse would unlock this.

2. **`mirror kintsugi` is unimplemented at rust altitude.** The flagship "settle a project" verb produces exit 2 with a stub message. No `cmd_kintsugi` exists (grep-verified). Neither does `cmd_index`, `cmd_shatter`, `cmd_init`, `cmd_recall`, `cmd_beam`, `cmd_peer`. 7 of the 15 advertised verbs are inert.

3. **The pillar predicate registry is 10 hardcoded names.** `spectral::liquid::pillar::enact` (liquid.rs:977) matches on 10 predicate names; everything outside returns `Verdict::Defer`. Adding a new bilateral to a `.mirror` file has NO effect on `mirror compile` output unless its `name` is one of these 10 (or unless the `apply_h::act` sentinel-substring check catches it via `mirror serve` → `mirror_query`). The 235+ substrate-decl'd bilaterals across `shards/**/*.mirror` are semantically inert at rust altitude for the compile path.

### One-paragraph summary

Today's mirror binary is a thin argv-router with an 8-verb dispatch table (of 15 advertised verbs) plus one working MCP JSON-RPC loop. What actually executes end-to-end: `mirror --help/--version/serve/roomba/compile/craft/bumblebee/bench/love` all have real bodies. The math substrate (LAPACK dsyev via FLANG Fortran) is genuine, wired through `prismqueer::ffi` with 9 real `extern "C"` symbols and 9 `unsafe { ... }` invocation sites; two of these (`spectral_eigenvalues` and `spectral_phase_lock`) fire on every valid `mirror serve --mcp` tools/call. `prismqueer::spectral::{kleinos, rotation, tension}` are LANDED with real bodies composing over LAPACK. HOWEVER: no `.mirror` grammar parser exists (only 3 line-scanner extractors for `bilateral` and `property` blocks); 7 of the 15 advertised verbs are stub-fallthroughs to exit 2 (including `kintsugi`, `index`, `init`, `recall`, `beam`, `shatter`); the pillar predicate registry is 10 hardcoded names in a `match` statement; and the Move 2/4/8 pipeline scaffolds (`reality`, `model`, `observation`, `assertion`, `hypothesis`, `question`, `recursion`) are shape-only sum-type declarations with round-trip tests but no compose logic. The `bootstrap/` directory is gone. `CoincidenceHash<3>` survives as a docblock-only reference; the executing type is `coincidence::Detector<3>` (LANDED with real SHA-256-seeded eigenvalue-hex output). `apply_h::act` is live on the MCP path — it does bilateral-corpus lookup + sentinel-substring dispatch, not semantic evaluation. Bottom line: the compiler EXECUTES today, but it does not COMPILE a `.mirror` file end-to-end in any meaningful sense of the word "compile."
