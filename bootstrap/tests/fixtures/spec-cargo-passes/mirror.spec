in @mirror/spec
in @property
in @io

# spec-cargo-passes fixture — exercises cmd_kintsugi_spec on a
# minimal `target binary { altitude @code/rust, emit cargo }` block
# whose Cargo.toml `cargo check` succeeds. Per shards/io/cargo.mirror
# the exit-code-zero lift is `transparency::success` → Verdict::Pass.

project spec_cargo_passes {
  source ~d'.'

  target binary {
    name     "spec_cargo_passes_fixture"
    altitude @code/rust
    emit     cargo

    manifest ~f'Cargo.toml'
  }

  settle_on {
    binary.compiles
  }
}
