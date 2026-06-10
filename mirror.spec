in @mirror/cli
in @mirror/mosaic
in @mirror/spec
in @property
in @io

# mirror.spec — the dogfood instance.
#
# Mirror's own project manifold. The substrate compiles itself by
# declaring itself: this spec IS what mosaic settles into the mirror
# binary, the CI action, and the GitHub release.
#
# Per docs/specs/mirror-spec-schema.md §8 ("The Self-Descriptive
# Mirror Spec"). The binary that comes out of `mirror kintsugi
# ./mirror.spec` is the binary that reads this file. The loop closes
# at the substrate's edge.

project mirror.spec {
  source ~d'shards/'

  legacy ~d'boot/', ~d'bootstrap/' {
    shrinkage_contract: monotonic_lines_decrease,
    retirement_target:  v1.0,
  }

  target binary {
    name     "mirror"
    altitude @code/rust
    emit     cargo
    check    check

    cli {
      # Mirror is the substrate compiler. It reads grammars, settles
      # them into a graph, and emits artifacts at named altitudes.

      command compile {
        # Compile a grammar against its imports.
        arg path: ~d
        flag strict: bool = true
      }

      command kintsugi {
        # Settle a project. Run mosaic on the spec.
        arg spec: ~f = ~f'./mirror.spec'
        flag target: list(str) = []
        flag emit_shatter: bool = false
      }

      command shatter {
        # Project a settled shard to .shatter format.
        arg oid: content_address
        arg out: ~f
      }
    }
  }

  # === pre-commit chain (insight #43 substrate-pull, 2026-06-09) ===
  #
  # The five pre-commit check altitudes lifted from `just pre-commit`'s
  # shell-out chain into substrate `target` blocks. Each target is one
  # @io/cargo action; mosaic settles the chain by walking the targets
  # and dispatching the named cargo subcommand per
  # shards/io/cargo.mirror's contract.
  #
  # First tick (this commit): each target falls back to spawning cargo
  # at the @io boundary; the substrate is consumed for the DISPATCH
  # only. Subsequent ticks replace per-altitude execution with
  # substrate-native settlement (content-addressed-skip; eigensheaf
  # parallelism; transparency<p> aggregation) as recognitions #44+
  # land.
  #
  # Ordering note: substrate-pull says altitudes settle in dependency
  # order. cargo check → cargo test → cargo clippy (clippy includes a
  # check pass); cargo fmt --check is independent (formatter altitude);
  # cargo audit reads the lockfile (@release-adjacent). The bootstrap
  # dispatcher walks targets in declaration order today; the eigensheaf-
  # Laplacian parallelism analysis lands at recognition #44+.

  target fmt {
    name     "mirror"
    altitude @code/rust
    emit     cargo
    check    fmt_check
  }

  target lint {
    name     "mirror"
    altitude @code/rust
    emit     cargo
    check    clippy
  }

  target tests {
    name     "mirror"
    altitude @code/rust
    emit     cargo
    check    test
  }

  target audit {
    name     "mirror"
    altitude @release
    emit     cargo
    check    audit
  }

  target action {
    name     "build"
    altitude @ci/github
    emit     yaml
  }

  target release {
    name     "mirror"
    altitude @release
    emit     github_release
    needs    [binary, action]
  }

  settle_on {
    binary.compiles
    binary.tests_pass
    fmt.formats
    lint.lints
    tests.tests_pass
    audit.advisories_clean
    action.validates
    release.signs
    total_transparency.weight == 0
  }
}
