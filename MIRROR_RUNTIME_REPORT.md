# MirrorRuntime Report — Slice B

**Status:** structural collapse complete. 9/9 mirror_runtime tests pass,
6/6 new coincidence::declaration tests pass, full `cargo build` clean.

## Files created

- `/Users/alexwolf/dev/projects/coincidence/src/declaration.rs` — typed
  declaration node primitive: `DeclKind`, `MirrorData` (with `Encode`/`Decode`),
  `MirrorFragment` = `Fractal<MirrorData, CoincidenceHash<5>>`, `fragment(...)`
  builder, `MirrorFragmentExt` accessor trait. 6 unit tests.

## Files modified

- `/Users/alexwolf/dev/projects/coincidence/src/lib.rs` — registered
  `pub mod declaration;`.
- `/Users/alexwolf/dev/projects/mirror/src/mirror_runtime.rs` — full rewrite.
  ~770 lines → ~580 lines. Hand-rolled `Beam`/`Shatter`/`compute_*`/textual
  `emit_shatter` deleted. New `Form` (parser output / Prism Crystal),
  new `Shatter` struct that implements `prism::Prism`, new `CompiledShatter`
  artifact wrapping `Form` + `MirrorFragment`. Parser/emitter retained but
  retargeted to `Form`. Added `mirror_runtime_shatter_prism_round_trip` test
  exercising the `Prism` impl (focus / project / refract + OID stability).
- `/Users/alexwolf/dev/projects/mirror/src/lib.rs` — already had
  `pub mod mirror_runtime;` from Slice A. Unchanged.

## Files deleted (sections)

- `Beam` struct, `compute_beam_oid`, `Shatter` struct, `ShatterHeader`,
  `compute_crystal_oid`, `SHATTER_MAGIC`, `emit_shatter`, `BeamOid` type alias
  — all removed from `mirror_runtime.rs`. Content addressing now flows
  through `fragmentation::fragment::content_oid()` against `MirrorData::encode()`.

## Hard wall hit and how it was resolved

**The brief instructed: "Move the Beam primitive into the prism crate."**
This is structurally impossible: `coincidence` already depends on `prism`
(see `coincidence/Cargo.toml` line 12). Adding `coincidence` as a dependency
of `prism` (required because the primitive is `Fractal<_, CoincidenceHash<5>>`)
creates a cycle:

```
prism → coincidence → prism
```

Verified by `cargo build -p prism` after a tentative add. Reverted.

**Resolution (decisive, documented, moved on):** the declaration primitive
lives in `coincidence` instead. Coincidence already depends on both
`fragmentation` and `prism`, sits one level above prism, and is the natural
home for `CoincidenceHash<5>` itself. Mirror already depends on coincidence,
so the import path is identical at the use site. Pack's intent — "the
primitive belongs in a structural crate below mirror, not in mirror itself"
— is satisfied. The literal instruction ("in prism") cannot be honored
without rearranging the entire dependency graph (a Pack-scale rewrite).

If Pack wants the primitive *literally* in prism, the cycle must be broken
first by moving `prism::SpectralOid` (which depends on coincidence
transitively) elsewhere, or by inverting the prism↔coincidence direction.
That is a Pack-level call, not an in-slice decision.

## DeclKind decision

`DeclKind` moved with `MirrorData` into `coincidence::declaration`. Rationale:
the typed declaration node IS the kind plus the data — splitting them across
crates would scatter a single primitive with no benefit and force duplicate
re-exports. One crate, one primitive.

## Test output

```
$ nix develop -c cargo test --lib mirror_runtime
running 9 tests
test mirror_runtime::tests::mirror_runtime_compile_form_file ... ok
test mirror_runtime::tests::mirror_runtime_mirror_form_has_property_applications ... ok
test mirror_runtime::tests::mirror_runtime_parses_atom_decl ... ok
test mirror_runtime::tests::mirror_runtime_parses_nested_property ... ok
test mirror_runtime::tests::mirror_runtime_parses_params_and_variants ... ok
test mirror_runtime::tests::mirror_runtime_property_file_compiles ... ok
test mirror_runtime::tests::mirror_runtime_round_trip_oids_match ... ok
test mirror_runtime::tests::mirror_runtime_shatter_prism_round_trip ... ok
test mirror_runtime::tests::mirror_runtime_compiles_full_boot_dir ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 816 filtered out
```

```
$ nix develop -c cargo test --lib declaration  (in coincidence)
running 6 tests
test declaration::tests::declkind_round_trips ... ok
test declaration::tests::fragment_shard_has_oid ... ok
test declaration::tests::different_content_different_oid ... ok
test declaration::tests::fragment_with_children_is_fractal ... ok
test declaration::tests::mirror_data_encode_decode_round_trip ... ok
test declaration::tests::same_content_same_oid ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 723 filtered out
```

## Build output

```
$ nix develop -c cargo build
   Compiling mirror v0.1.0 (/Users/alexwolf/dev/projects/mirror)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.28s
```

Clean. (Pre-existing warnings in fragmentation/abyss/classifier are not from
this slice.)

## Prism trait shape on Shatter

```rust
impl Prism for Shatter {
    type Input       = Form;
    type Eigenvalues = MirrorData;
    type Projection  = MirrorFragment;
    type Node        = Form;
    type Convergence = MirrorFragment;
    type Crystal     = Form;
    type Precision   = Precision;
    // focus: top eigenvalues. project: childless content-addressed frag.
    // split/zoom: conservative no-ops (TBD per Pack instruction).
    // refract: MirrorFragment → Form via structural decompile.
}
```

Full structural compile/decompile is provided as `Shatter::compile_form()` /
`Shatter::decompile()` outside the trait surface, since the trait's
`project(eigenvalues, precision)` only carries top-level data — the recursive
descent needs the whole `Form`, not just one node's eigenvalues. The Prism
trait surface remains honored; the boot pipeline uses the structural method.
This split can be revisited when `split`/`zoom` are specced.

## Persistence path

The `.shatter` file format is now the binary `Encode` of the root
`MirrorFragment`'s `MirrorData` plus its content-addressed children, written
through `FrgmntStore::insert_persistent()` when the next slice wires that up.
No bespoke text format remains in this module.
