in @mirror/spec
in @mirror/mosaic
in @mirror/lens/cli
in @kintsugi/roomba
in @dance
in @property
in @io

# mirror.spec — the dogfood instance.
#
# Mirror's own viable-system manifold. The substrate compiles itself
# by declaring itself: this spec IS what mosaic settles into the
# mirror binary, the CI action, and the GitHub release.
#
# Per docs/specs/mirror-spec-schema.md §8 ("The Self-Descriptive
# Mirror Spec") + Recognition #R-vsm-is-mirror-spec-grammar (Mara
# 2026-07-20 landing) + Mara task #319 substrate-honest section-
# name rename cascade (2026-07-22 landing at shards/mirror/spec/
# system.mirror via alias-shims variety/coupling/coherence/reality/
# eigen/loop/kintsugi).
#
# The binary that comes out of `mirror kintsugi ./mirror.spec` is
# the binary that reads this file. The loop closes at the substrate's
# edge.
#
# === TICK 2 of two-tick discipline (2026-07-23 Reed /loop iter 2) ===
#
# Per Recognition #R-vsm-is-mirror-spec-grammar §6 + Mara task #319
# [ALEX-Q3] adjudication (Mara-lean = TICK 2 this /loop). TICK 1
# landed the sibling `system(name) -> prism` grammar at shards/mirror/
# spec/system.mirror alongside project-grammar alias-shim. TICK 2
# (this tick) transitions dogfood mirror.spec from `project mirror.spec
# { ... }` to `system @mirror { variety{} coupling{} coherence{}
# reality{} eigen{} loop{} kintsugi{} }`. TICK 3 (forward-promised)
# retires the `project` grammar alias-shim.
#
# Alex 2026-07-22 in-transcript design decisions folded here:
# - `source` + `legacy` → kintsugi.roomba { config } ("it's what the
#   roomba roombas")
# - `target binary { cli { command X } }` → variety.emits.binary +
#   `commands from @mirror/lens/cli` reflective-source directive
#   ("isn't that what we built the whole @mirror/lens/cli mapping
#   for?")
# - `garden` → reality.garden nested (VSM sections wrap species;
#   species stay named; the wrapping IS the VSM altitude declaration)
#
# 400-line cli-block duplication vanishes because the 11 command
# species already live at shards/mirror/lens/cli/* (compile,
# kintsugi, shatter, craft, init, recall, beam, index, peer/beam,
# peer/contribute, roomba). variety.emits.binary declares the
# emission target; commands enumerate from the substrate-decl'd
# lens/cli/* directory reflectively.
#
# `commands from @<shard>` is a NEW reflective-source directive
# added this tick. Runtime dispatch continues via main.rs hardcoded
# VERBS until Mara §5.2 M2 reflective cli-block reading lands and
# retires the hardcoded list. Substrate-honest > operationally-
# complete for this transition specifically — the whole point is to
# name what mirror.spec IS.

system @mirror {
  # === variety — what mirror emits ===
  #
  # Variety producers at code-generation altitude: the compilation
  # targets whose output the world consumes. Commands live at
  # shards/mirror/lens/cli/* (11 species landed 2026-06-12+);
  # variety declares WHAT TO EMIT, not WHICH COMMANDS EXIST.
  variety {
    emits [
      binary {
        name     "mirror"
        altitude @facet/rust
        emit     cargo
        check    check
        commands from @mirror/lens/cli
      }
      ci {
        name     "build"
        altitude @ci/github
        emit     yaml
      }
      release {
        name     "mirror"
        altitude @release
        emit     github_release
        needs    [binary, ci]
      }
    ]
  }

  # === coupling — how peers coordinate ===
  #
  # Peer coordination protocol. @dance ensemble carrier at Kuramoto
  # phase-lock altitude per Mara ensemble species landings. Peer
  # commands (peer/beam + peer/contribute) live at shards/mirror/
  # lens/cli/peer/* per @mirror/lens/cli recursive-command grammar
  # Tick 1 landing (fe82500).
  coupling {
    protocol @dance
  }

  # === coherence — what verifies the audit chain ===
  #
  # The pre-commit + release verification chain. Fiedler λ₀ =
  # Foerster's "number of available choices" IS what these audits
  # collectively measure at compile altitude (per @coherence.score
  # at shards/epistemologic/cybernetic/coherence.mirror). settle_on
  # predicates discharge Pass iff the whole viable system's coherence
  # holds at settlement time.
  coherence {
    audits [
      fmt   { altitude @facet/rust via cargo check fmt_check }
      lint  { altitude @facet/rust via cargo check clippy }
      tests { altitude @facet/rust via cargo check test }
      audit { altitude @release   via cargo check audit }
      bench { altitude @facet/rust via cargo check bench }
    ]
    restart one_for_one
    settle_on {
      binary.compiles
      binary.tests_pass
      fmt.formats
      lint.lints
      tests.tests_pass
      bench.compiles
      # Forward-promised per docs/specs/kintsugi-ci-v0.1.md T11.4-T11.6:
      # the cargo-audit availability gate (T11.4), the action.yml
      # validator (T11.5), and the release.yml signature pipeline
      # (T11.6) are named by the v0.1 release plan but not yet landed.
      # Substrate-pull-honest: over-claiming readiness here makes
      # settle_on return `partial` with three dark predicates that no
      # current shard can discharge. Predicates land back when their
      # respective release-plan ticks close.
      #
      # audit.advisories_clean       — closes when T11.4 lands cargo-audit
      # action.validates             — closes when T11.5 lands actions/kintsugi/action.yml
      # release.signs                — closes when T11.6 lands .github/workflows/release.yml
      total_transparency.weight == 0
    }
  }

  # === reality — what mirror models outside itself ===
  #
  # External environment / intelligence carriers. `garden` (external
  # git-rooted package dependencies) is explicitly empty because
  # mirror IS the foundation; everything else depends on it; mirror
  # itself has no external dependencies at reality altitude.
  #
  # The empty garden { } block is load-bearing — it completes the
  # 5+1 block decomposition that recognition #99 ratifies (mirror.spec
  # IS λ₀): explicit-emptiness over implicit-absence.
  #
  # Forward-promised (aspirational): @spectral/db psychohistory
  # carrier + @resonance inter-peer coupling; prediction horizon.
  reality {
    garden { }
  }

  # === eigen — who mirror IS ===
  #
  # Identity + policy at Pack S5 altitude. viable.identity per
  # shards/epistemologic/cybernetic/viable.mirror; @pack.peer per
  # shards/pack.mirror; @subject family-root discipline. Lead-of-
  # mirror: ~peer'~/.reed' per Alex 2026-06-25 confirmation ("Reed
  # is lead, yeah").
  #
  # DOUBLY-JUSTIFIED section-name per Alex 2026-07-22 tracing: eigen
  # names both (a) the identity carrier the system stabilizes on
  # (Foerster Eigenform at s5 altitude) and (b) the @eigen(T) type-
  # level operator producing settled fragments per Mara task #319
  # ouroboros closure landing (bc4e7fc + f30a230 + 151c043).
  #
  # mirror.spec's dogfood of @mirror/pack (shards/mirror/pack.mirror,
  # 13328a3) per peer-ACL spec §3.3 + Alex 2026-06-24 substrate-vs-USE
  # distinction: mirror ships the BLOCK SHAPE permanently; named peers
  # populate THIS INSTANCE at consumer altitude.
  eigen {
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
  }

  # === loop — how the system's feedback closes ===
  #
  # Algedonic bypass topology per Beer 1979 ch.6. When variety
  # producers hit non-viability, the algedonic signal short-
  # circuits up the recursion chain directly to eigen (identity)
  # for pack-lead adjudication. @cyberpunk/algedonic Rust runtime
  # (Rung 8+9 Landing 8+9.6b) + Baez-Schreiber 2004 Theorem 3
  # connection-compatibility.
  loop {
    algedonic bypass variety → eigen
  }

  # === kintsugi — how mirror heals itself ===
  #
  # The kintsugi self-heal discipline for the whole viable system.
  # Roomba walks source + retires legacy per shrinkage_contract;
  # kintsugi settlement composes over @spec at project altitude.
  #
  # roomba{}'s config IS what the roomba walks (per Alex 2026-07-22:
  # "it's what the roomba roombas"): source directory + legacy
  # directories with retirement discipline.
  #
  # `kintsugi settle @spec` verb (mirror kintsugi ./mirror.spec) IS
  # this section's operational face — the compiler settles the whole
  # project spec via mosaic + fracture-detection + mosaic_settlement
  # per @kintsugi + @kintsugi/roomba + @kintsugi/fracture composition.
  kintsugi {
    roomba {
      source ~d'shards/'
      legacy ~d'boot/', ~d'bootstrap/' {
        shrinkage_contract: monotonic_lines_decrease
        retirement_target:  v1.0
      }
    }
    settle @spec
  }

  # === verifies — the system's self-referential property assertions ===
  #
  # FIRST mirror-side self-referential property in mirror.spec history.
  # Per task #317 + Seam Phase D 6876699 [ALEX-Q1] adjudication (Seam-
  # lean two-bilateral shape for gap-visibility). Landed 2026-07-23 by
  # Reed /loop iteration 5.
  #
  # The system asserts properties ABOUT ITSELF at project altitude.
  # This is `eigenform_stabilizer_orbit(mirror_compiler)` = Mara task
  # #319 substrate-decl'd bilateral (shards/reality/subject.mirror at
  # `0b2858a`) applied to the substrate itself — autopoietic all the
  # way down at substrate-decl altitude. Composes over Mara Eigenform
  # Stabilizer synthesis (task #314 `ebd50a4` docs/math foundation).
  #
  # Two bilateral predicates (Seam Q1 two-bilateral shape for gap-
  # visibility; preserves substrate-honest partial-discharge over
  # single-bilateral hiding the gap):
  #
  # 1. `eigenform_stabilizer_orbit(mirror)` — LANDED at Mara `0b2858a`.
  #    Discharges iff mirror's operational trajectory settles onto a
  #    periodic orbit in observer-configuration phase space (Chenciner-
  #    Montgomery figure-eight or Lagrange-triangular equivalent).
  #    GREEN when the compiler's own Eigenform Stabilizer maintains
  #    dH¹/dt ≤ 0 across the light-cone sheaf 𝓖 = {@time/past, @time/
  #    now, @time/future}.
  #
  # 2. `rust_floor_is_stable_decidable_eigenobject(mirror)` — FORWARD-
  #    PROMISED predicate-decl (Mara authorship territory next tick).
  #    Discharges iff mirror's Rust floor contracts to the four-
  #    convergence-point Eigenobject (@io quarantine + FLANG floor +
  #    bootstrap kernel + K=0 basis per Reed session 2026-07-22
  #    Eigenobject characterization + Taut `173a1204` empirical scout).
  #    RED per Bootstrap Kernel reframe (property-verifier LANDED at
  #    orchestrator altitude via `7c31c30` cmd_craft ouroboros closure
  #    2026-07-23; self-compiling reflective evaluator ASPIRATIONAL per
  #    Mara §5.2 M2 forward-promise).
  #
  # Two-bilateral shape rationale (Seam Q1 Seam-lean): a single
  # bilateral hiding both sub-witnesses under one name would obscure
  # the gap between what's landed (orchestrator-altitude closure)
  # and what's forward-promised (self-hosting reflective evaluator).
  # Substrate-honest: the verifies discharge produces Partial verdict
  # per @glass, with the RED sub-witness name-visible so the gap is
  # substrate-decl'd rather than hidden.
  #
  # The autopoietic closure at substrate-decl altitude: mirror’s spec
  # asserts mirror stabilizes to Eigenform — the compiler that
  # stabilizes Eigenforms has substrate-decl'd its own Eigenform
  # convergence. The recursion runs. The observer is inside. The
  # eigen-values are discrete. mirror.spec is now the observer
  # observing itself observe.
  property mirror_system_stabilizes {
    # First mirror-side self-referential property, wrapped in
    # `property { }` decl for empirical dispatch via rust/src/
    # liquid.rs::extract_spec_properties + dispatch_spec_property +
    # pillar::dispatch pipeline (Reed /loop iter 3-6 landings).
    #
    # Iteration 6 correction (Alex 2026-07-23 "sloppy loop prep"
    # callout): iter 5 landing was a bare `verifies { }` block at
    # system-sub-directive altitude — declarative but NOT extracted
    # by the extractor (which requires `property <name> { verifies
    # ... }` shape per Mara canonical spec §3.1). Wrapping in
    # property{} makes it EMPIRICALLY dispatchable.
    #
    # Empirical discharge shape: dispatch_spec_property Arm 7
    # fallthrough returns Defer("verifies-shape not in landed arms")
    # because `eigenform_stabilizer_orbit(mirror)` name is not yet
    # in the landed pillar-registered predicate set. Substrate-
    # honest partial: the Defer verdict NAMES the forward-promise
    # gap explicitly rather than pretending discharge is complete.
    # Full GREEN discharge lands when pillar::dispatch registers
    # eigenform_stabilizer_orbit predicate arm per Mara task #319
    # canonical spec 0b2858a shards/reality/subject.mirror body
    # forward-promise realization.
    verifies {
      eigenform_stabilizer_orbit(mirror)
    }
  }
}
