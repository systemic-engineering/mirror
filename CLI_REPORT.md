# mirror CLI — minimum viable surface

## Files modified

- `/Users/alexwolf/dev/projects/mirror/src/main.rs` — replaced (938 → 213 lines)

## Files deleted

None. The old subcommands were excised by total replacement of `main.rs`. The
`db`, `domain_dispatch`, `resolve`, etc. modules in `lib.rs` are untouched —
they remain available to the library, just no longer reachable through the CLI.

## New CLI surface

- `mirror compile <file>` — writes `<file>.shatter`, prints crystal OID. Exit 0.
- `mirror '<query>' <file> [--compile <target>]` — parses query + target as
  Forms, prints honest TBD message. Exit 2. With `--compile`, also writes the
  `.shatter` artifact.
- Anything else / no args → usage to stderr, exit 1.

## .shatter on-disk format

Tiny length-prefixed envelope around `MirrorData::encode()`:

```
node := u32_le(data_len) data_bytes u32_le(child_count) child*
```

`data_bytes` is exactly the `Encode` impl from
`coincidence::declaration::MirrorData`. No bespoke hashing path. The whole
fragment tree round-trips structurally. `Fractal<_, _>` itself does not
implement `Encode` in the fragmentation crate, so the framing is in
`encode_fragment()` in `main.rs`.

## Verification

### `cargo build`

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.18s
```

### `cargo run -- compile boot/00-form.mirror`

```
boot/00-form.mirror.shatter
crystal: 9dba181d869ca88c125ad4d6e463a28263f0a63b
exit=0
-rw-r--r--@ 1 reed  staff  128 Apr  7 13:43 boot/00-form.mirror.shatter
```

128-byte file written to disk. Crystal OID printed.

### `cargo run -- 'form @x { fold input }' boot/00-form.mirror`

```
mirror: query parsed as form `@x`, target parsed as form `@form` —
applying form-as-operation semantics is TBD; the runtime does not yet
implement this.
exit=2
```

Both inputs parsed, TBD message emitted, exit 2.

### `cargo test --lib`

```
test result: FAILED. 810 passed; 2 failed; 1 ignored; 0 measured;
0 filtered out; finished in 139.31s
```

The 2 failures are the pre-existing known ones:

- `abyss::tests::boot_sequence_settles_combined`
- `emit::tests::round_trip_boot`

Failure count unchanged from baseline.

## Hard walls

None hit.

## Notes

- The query mode is intentionally a stub. Form-as-operation semantics are not
  specced; same posture as `split` / `zoom` on the `Shatter` `Prism` trait.
- The `db` and `@domain` subcommands from the old CLI were not preserved.
  They remain available as library modules; surfacing them is deferred to a
  future CLI dispatch.
- New `main.rs` is 213 lines, down from 938.
