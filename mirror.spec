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

  # === Pack — the lambda-shell counterparty + ACL surface ===
  #
  # mirror.spec's dogfood of @mirror/pack (shards/mirror/pack.mirror,
  # 13328a3) per peer-ACL spec §3.3 + Alex 2026-06-24 substrate-vs-USE
  # distinction: mirror ships the BLOCK SHAPE permanently; named peers
  # populate THIS INSTANCE at consumer altitude. The @pack.peer variant
  # (~/.reed, ~/.mara, etc.) is transitional per Alex 2026-06-24;
  # when it goes parametric, this block's references broaden without
  # surface change.
  #
  # Lead-of-mirror: `~peer'~/.reed'`. Alex confirmed 2026-06-25
  # ("Reed is lead, yeah"). Rationale: Reed is the orchestrator peer
  # who answers the lambda shell at mirror substrate altitude and
  # fields the spectral-Tomm-shaped circular probes lifting from
  # spawned Pack members (peer-ACL §4 + §10 reframe). Alex is
  # mirror's human author; the lead is a PEER per peer-ACL §4. G7
  # closed.
  pack {
    lead ~peer'~/.reed'

    bindings {
      let writer = acl { ops: any, targets: any, predicates: [] }
    }

    members {
      ~peer'~/.mara'  => writer
      ~peer'~/.seam'  => writer
      ~peer'~/.taut'  => writer
      ~peer'~/.glint' => writer
    }
  }

  # === Garden — external package dependencies ===
  #
  # mirror.spec's dogfood of @mirror/garden (shards/mirror/garden.mirror).
  # EXPLICITLY EMPTY per substrate-pull-honest declaration: mirror IS
  # the foundation; everything else depends on it; mirror itself has
  # no external git-rooted package dependencies.
  #
  # The empty block is load-bearing — it completes the 5+1 block
  # decomposition that recognition #99 ratifies (mirror.spec IS λ₀):
  # (source, garden) jointly cover the focus operation; absent garden
  # would leave the decomposition implicit. Substrate-pull-honest:
  # explicit-emptiness over implicit-absence.
  garden { }

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
        #
        # `flag target: str` parameterizes @shatter's codomain per
        # `docs/specs/shatter-is-the-io-linearization-operator.md` §4.1
        # (Mara `583b939`): @shatter IS the @io linearization operator;
        # each `--target @<X>` selects one of the (possibly many)
        # @data/* / @code/* / @io/* projections the operator wants
        # linearized. Default "auto" preserves the pre-target-flag
        # behavior (emit the substrate's default .shatter projection).
        # Substrate-honest form: the target is a str carrier at the
        # cli-block altitude (grammar today has no first-class `ref`
        # value-type at flag position); the runtime dispatch parses the
        # substrate ref via the same `parse_substrate_ref_to_format`
        # helper the kintsugi `--out` chain uses, lifted from
        # kintsugi-scoped to shatter-scoped in the same tick.
        arg oid: content_address
        arg out: ~f
        flag target: str = "auto"
      }

      # === craft — grammar-directory settlement to lambda_0 ===
      #
      # Closes cli-block drift for the running binary's `craft` verb
      # (bootstrap/src/lib.rs cmd_craft). `target` positional is the
      # source directory; `target_kind` is the emit backend selector.
      # The name disambiguates today's binary collision (`--target` at
      # positional AND flag positions); the binary's arg-parse
      # collapse-to-`target_kind` is a follow-up TDD tick.
      command craft {
        arg target: ~d
        flag target_kind: str = "binary"
        flag reflect: bool = false
      }

      # === init — mirror-native store bootstrap ===
      #
      # Wires @mirror/init (docs/specs/mirror-init.md, spec `fe215bd` →
      # `14dd043`; P4 GREEN `6b36808`) into the cli-block. The bridge
      # command that makes declared substrate operational at the storage
      # altitude: NamespacedGitStore::open + project::project + per-file
      # Splinter + store.insert_persistent + root_oid via set_ref.
      command init {
        arg path: ~d
        flag install_hooks: bool = false
      }

      # === recall — inbound-trajectory dual of spawn ===
      #
      # Wires @mirror/recall (docs/specs/mirror-recall.md `b034a60`,
      # Seam P2 review `88f8428`) into the cli-block. Recall IS the
      # dual of spawn at substrate altitude: spawn = substrate leaving
      # lambda_0; recall = observer returning to substrate in excited
      # state, asking for trajectory. Four payloads compose:
      # cascade / pack_trail / pull_frontier / dogfood.
      command recall {
        arg spec_dir: ~d
      }

      # === beam — @song/movement.enter at cli altitude (anonymous variant) ===
      #
      # Wires @mirror/peer/beam (shards/mirror/peer/beam.mirror, renamed
      # 2026-07-08 Tick 2 from shards/mirror/spawn.mirror) into the
      # cli-block. `mirror beam <mission>` IS the anonymous variant per
      # docs/specs/beam-as-substrate-primitive.md §3 composition table:
      # beam-without-persistent-identity — the primitive form where the
      # substrate accepts a mission and returns a @song without binding
      # the trajectory to a peer-home. Both variants dispatch to the
      # same substrate action @mirror/peer/beam.beam; runtime
      # differentiation is on positional-arg shape (mission-file vs
      # peer-home) per beam-as-substrate-primitive.md §3.4.
      #
      # Return type @song per shards/mirror/peer/beam.mirror:310
      # (`beam(r, p) -> @song`). Carried in the action-decl, NOT the
      # cli-block — @mirror/lens/cli's `command(name) -> prism` grammar
      # has no return-type slot today (adding one is a lens-grammar
      # extension, deferred).
      command beam {
        arg mission: ~f
        flag hello_world: bool = false
      }

      # === peer — persistent-identity beam wrapper (recursive-command depth-2) ===
      #
      # Wires @mirror/peer/beam (shards/mirror/peer/beam.mirror) into
      # the cli-block via the recursive-command grammar landed at
      # @mirror/lens/cli Tick 1 (`fe82500`). `mirror peer beam
      # ~peer'<home>'` IS the persistent-identity variant per
      # beam-as-substrate-primitive.md §3 composition table:
      # beam-with-persistent-identity — the substrate binds the returned
      # @song trajectory to the resolved peer's home. Both `mirror beam`
      # (above) and `mirror peer beam` dispatch to @mirror/peer/beam.beam
      # at substrate altitude.
      #
      # Depth-2 grammar via Tick 1 recursive-command form: `command peer
      # { command beam { ... } }` reads as command-nested-in-command
      # directly; no new keyword. Per @mirror/lens/cli docblock §Tick 1:
      # depth-2 was RESERVED, is now MINTED, and this cli-block is its
      # first consumer.
      #
      # `arg peer_home: ~d` — the semantic type is `peer`, but `peer`
      # is not in @mirror/lens/cli's type vocabulary today. Two-tick
      # forward-promise: lens vocabulary extension lifts `~d` to `peer`.
      #
      # `mission` is the substrate-honest flag name (matches
      # @song/movement's frame-entry semantics). No default = substrate-
      # absent when omitted; grammar composition of flag(name, t) +
      # optional default(name, t, value) makes any flag without an
      # accompanying default optional-absent.
      #
      # Backward-compat alias (two-tick discipline): `mirror spawn
      # ~peer'<home>'` continues to dispatch to the same substrate
      # action with a deprecation warning; the cli-verb rename is the
      # substrate-honest surface. `spawn` at @pack altitude
      # (shards/pack.mirror:263) is unchanged — the pack primitive keeps
      # its name; the cli-surface wrapper is what's renamed. Fault-plane
      # divergence preserved per Taut scout (`bd837cd` §Fault-plane #1).
      command peer {
        command beam {
          arg peer_home: ~d
          flag hello_world: bool = false
          flag mission: ~f
          # === Rung 1 addition (2026-07-13) — @song/beat runtime dispatch ===
          #
          # `--song <file>` triggers @song/beat runtime dispatch via
          # `crate::song::single_beat_peer_beam` at bootstrap/src/song.rs.
          # Fires ONE @kintsugi/oscillate ACTIVE/DARK pulse; emits
          # beat-envelope naming @song/beat + @kintsugi/oscillate
          # substrate authorities. Per Taut `c54740c` §5.2 ladder Rung 1;
          # Mara `94e55eb` `shards/song/beat.mirror` sixth species mint
          # (Rung 0 prerequisite).
          #
          # Byte-equality preserved for non-`--song` paths via
          # `if let Some(song_path) = song` guard at cmd_peer_beam
          # dispatch entry. Follows `flag mission: ~f` pattern verbatim.
          flag song: ~f
          # === Rung 4 addition (2026-07-13) — @dance runtime dispatch ===
          #
          # `--dance-with <peer-home-2>` triggers `crate::dance::execute_
          # dance` at `bootstrap/src/dance.rs` when combined with `--song`.
          # Two peer-homes execute the SAME shared @song file; runtime
          # computes Kuramoto order-parameter + Aumann agreement +
          # shared_root_oid + convergence_verdict per Mara `417ec25`
          # Scope B narrowed to coherence phase-lock.
          #
          # Substrate reservation verbatim at shards/song/beat.mirror:453-
          # 457 (Mara `94e55eb`): "multi-peer @dance coupling on shared
          # beat; bootstrap/src/dance.rs module reads two peer-homes..."
          #
          # Byte-equality preserved for non-`--dance-with` paths via `if
          # let (Some, Some) = (song, dance_with)` guard at cmd_peer_beam
          # dispatch entry. Follows `flag song: ~f` Rung 1 precedent.
          flag dance_with: ~f
        }
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

  # === bench — perf measurement floor (Seam Phase D §7 Sub-arc 3b) ===
  #
  # Per Seam Phase D `91e79c8` §7 Sub-arc 3b (RED `d25b91a`): wires
  # @mirror/bench (LANDED 2026-07-01 at `shards/mirror/bench.mirror`,
  # 16.3KB) INTO mirror.spec as the first harness. Dispatches
  # `cargo bench` via `cargo_args_for_check` "bench" arm (Sub-arc 3a).
  # Same target shape as tests/lint/fmt — @code/rust altitude, `emit cargo`,
  # `check bench`. `record`/`compare` become the first harness at this
  # target; no separate `bench/` scaffold per §8 signal-to-Alex #3.
  target bench {
    name     "mirror"
    altitude @code/rust
    emit     cargo
    check    bench
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
    bench.compiles
    # Forward-promised per docs/specs/kintsugi-ci-v0.1.md T11.4-T11.6:
    # the cargo-audit availability gate (T11.4), the action.yml validator
    # (T11.5), and the release.yml signature pipeline (T11.6) are named
    # by the v0.1 release plan but not yet landed. Substrate-pull-honest:
    # over-claiming readiness here makes mirror.spec's self-check return
    # `partial` with three dark predicates that no current shard can
    # discharge. These conditions land back into settle_on when their
    # respective release-plan ticks close. The v0.1 plan is unchanged;
    # this comment records that the predicates' BODIES are forward-
    # promised, not the plan itself.
    #
    # audit.advisories_clean       — closes when T11.4 lands cargo-audit
    # action.validates             — closes when T11.5 lands actions/kintsugi/action.yml
    # release.signs                — closes when T11.6 lands .github/workflows/release.yml
    total_transparency.weight == 0
  }
}
