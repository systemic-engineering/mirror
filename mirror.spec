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
    action.validates
    release.signs
    total_transparency.weight == 0
  }
}
