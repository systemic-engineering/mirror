# bootstrap

The seed. Everything above this directory is grammar; this directory is steel.

Rust port of the original C bootstrap. Implements the bit-exact
`CoincidenceHash<5,5>` content address, the tokenizer, the AST walker, the
git crystal store, and the `compile` / `craft` / `kintsugi` subcommands.

## Build

```bash
cargo build --release --manifest-path bootstrap/Cargo.toml
```

## Install

```bash
cp ${CARGO_TARGET_DIR:-bootstrap/target}/release/mirror ~/.local/bin/mirror
```

(With direnv in this repo, `CARGO_TARGET_DIR=/Users/alexwolf/dev/.cargo-target`.)

## Regenerate `bootstrap/mirror.ll`

`mirror.ll` is the bootstrap's own LLVM IR, checked in as the reference
artifact that `@code/llvm/emit` reads when seeding emission.

```bash
cargo rustc --release --manifest-path bootstrap/Cargo.toml -- --emit=llvm-ir
cp ${CARGO_TARGET_DIR:-bootstrap/target}/release/deps/mirror-*.ll bootstrap/mirror.ll
```

## Self-rebuild (the butterfly)

```bash
mirror craft --target binary boot   # produces ./mirror-self
./mirror-self craft boot            # must equal `mirror craft boot`
```

## The Release Rule

When `./mirror-self craft boot` equals `mirror craft boot`, the bootstrap
reproduces itself from grammar. That equality is v1.0.0. Tag and ship.

## A note on the IR

`bootstrap/mirror.ll` is architecture-specific. The checked-in IR is currently
for arm64-darwin (Apple Silicon under macOS). On other architectures, regenerate
locally before running `craft --target binary`.
