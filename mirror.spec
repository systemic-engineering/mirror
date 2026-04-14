# mirror.spec — the CLI IS the spec IS the config

@oid("@mirror-lang")

store {
  path = .git/mirror
}

craft {
  target boot("boot/*.mirror") {
    @prism
    @meta
    @meta/action
    @meta/io
    @shatter
    @code
    @code/rust
    @actor
    @runtime
    @property
    @package
    @package/git
    @package/spec
  }

  target std("boot/std/*.mirror") {
    @beam
    @benchmark
    @cli
    @mirror
    @properties
    @time
    @tui
  }

  target boot => mirror out @code/rust("rust/mirror/") {
    @prism
    @meta
    @property
    @package
  }

  target boot => cli out @code/rust("rust/mirror-cli/") {
    @code
    @shatter
    @actor
  }

  default boot
}

kintsugi {
  --hoist
  --sort-deps
  --normalize
  --align
  naming = snake_case
  indent = 2
}

properties {
  requires {
    types_lowercase
    action_is_named_type
    unique_variants
    every_type_reachable
    no_dead_variants
  }
  invariant {
    deterministic
    pure
    no_cycles
  }
  ensures {
    always_halts
  }
}
