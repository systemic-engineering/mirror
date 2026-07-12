# Flags as lens applications on `mirror peer beam` — lens composition IS the CLI surface's canonical mosaic

*Mara, 2026-07-12. Substrate-decl v0.1 — thinking-in-public + canonical
spec. Alex's in-transcript reframe 2026-07-12 landing. Sits on Taut's
scout `b8fe820` (`docs/scouts/2026-07-12-taut-flags-as-lenses-cli-mosaic-scout.md`),
Reed's session runtime landings (`07ac55a` shadow-casting, `4b2ef3c`
autopoietic closure, `0dd1074` MCP flag exposure), and the session's
substrate-closure annotation at `shards/cyberpunk.mirror:cybernetic_coherence`
(`8e6e517`) that reads `cybernetic_coherence(s) = λ₀(Δ_F(s))`.*

*Alex verbatim (2026-07-12):*

> *"I feel most of these --args are things mirror beam ought to do by
> default, and definitely mirror peer beam, no? What if these are
> lenses that are applied to the CLI surface, basically optics that
> modify the call. A @spectral/mosaic of a CLI surface."*

*Alex's mandate: "no gaps or until there's again an unresolvable
ambiguity."*

---

## §0 The reading in one paragraph

Every `--flag` on `mirror peer beam` (and on `mirror beam`) IS a lens
application: an optic that modifies the stage's five-op prism at a
specific altitude. The seven flags Reed landed as opt-in Rust arg-checks
during this session (`--fate-select`, `--from-psychohistory`,
`--with-shadow`, `--mission`, `--emit-diff`, `--integrate-diff`,
`--hello-world`) each map to a species under `@optics/lens/*` or to an
output-shape carrier at CLI altitude. Substrate-honest, they are the
default composition; opt-in is Reed's incremental-build residue, not the
substrate's declared shape. The parametric composition operator is
`mosaic(@mirror/lens/cli)` — landed at `shards/mirror/mosaic.mirror:60`
as `type mosaic(altitude) = ref`, awaiting the altitude specialization
this spec supplies. Alex's `@spectral/mosaic` is REFUSED per Taut's
scout §R1; the substrate-correct name uses the parametric operator
already in the substrate. This is the ~85%-substrate-already-had-the-word
close Taut named, plus one MINT at depth-2 (`stage @mirror/lens/cli/peer/beam`)
plus one composition operator (`mosaic(@mirror/lens/cli)`).

---

## §1 Substrate-already-had-the-word audit (extends Taut's scout)

Taut's scout `b8fe820` established the ~85%-landed ledger. This section
adjudicates the seven flags against LANDED species. Grep-verified this
session on `mara/song-substrate-decl-v0.1`.

### §1.1 Landed carriers cited by OID / file:line

| Altitude | OID / file | What it declares | What this spec composes against |
|----------|-----------|-------------------|----------------------------------|
| family-root | `shards/optics/lens.mirror` (2026-07-10 Mara `b0427fd`) | `prism @optics/lens { focus/project/split/shift/settle lens }` + `type lens_get`, `lens_put`, `lens_witness` + Foster `put_get` / `get_put` / `put_put` bilaterals | The lens laws every flag-lens species inherits |
| species | `shards/optics/lens/features.mirror` (2026-07-11 Mara `f3af5b4`; spec proposal `ec6dbaa`) | `prism @optics/lens/features { … }` + `get: text → observation`, `put: (obs, text) → text'` | `--fate-select` maps here (Reed `0dd1074` MCP docstring already cites it) |
| species | `shards/optics/lens/diff.mirror` (2026-07-10 Mara `7e5c298`) | `prism @optics/lens/diff { … }` + `focus(diff_focus_request) → diff_bytes` (get) + `settle(diff_settle_request) → bauchladen_state` (put) | `--emit-diff` maps to `.focus` (get); `--integrate-diff` maps to `.settle` (put) |
| parametric operator | `shards/mirror/mosaic.mirror:60,94` (2026-06-09 Mara, rec #43) | `type mosaic(altitude) = ref` — the altitude-specialized composition carrier | The composition operator this spec specializes for `@mirror/lens/cli` |
| family-root | `shards/mirror/lens.mirror` (2026-06-06 Mara) | `prism @mirror/lens { … }` — the four transports + two measurement lenses | Namespace-parent for `@mirror/lens/cli` |
| species | `shards/mirror/lens/cli.mirror` (2026-06-06 Mara, subcommand-nesting Tick 1 `fe82500` 2026-07-08) | `prism @mirror/lens/cli { … }` + `command(name) -> prism`, `arg(name, t: type) -> prism`, `flag(name, t: type) -> prism`, type vocabulary (`str/bool/int/dir_ref/file_ref/…`) | The CLI altitude keyword surface; the `flag` typed-lambda this spec's lens-application semantics live on |
| depth-2 grammar | `shards/mirror/lens/cli.mirror` §subcommand-nesting (2026-07-08 Tick 1 `fe82500`) | Recursive-command grammar admitted; `command X { command Y { … } }` legal | Unblocks `stage @mirror/lens/cli/peer/beam` mint (§4) |
| species | `shards/mirror/peer/beam.mirror` (2026-07-08 Tick 2 `9de2226`) | `prism @mirror/peer/beam`; `type mirror_peer_beam_request = { target, options, winding }`; `beam(r, p) -> @song requires peer_well_known(r.target, p)` | The base action-decl the CLI stage's `settle` binds |
| annotation | `shards/cyberpunk.mirror:cybernetic_coherence` (2026-07-11 Alex synthesis, `8e6e517`) | `cybernetic_coherence(s) = λ₀(Δ_F(s))` — the shared metric compiler and peer optimize/read | Section §7's bridge back to the substrate metric |
| runtime spec | `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` (2026-07-11 Mara) | `Fate::bounded(<config>)` derived from psychohistory sheaf; navigates H¹ gradient = Rayleigh descent on Δ_F | `--from-psychohistory` maps to this |
| runtime state | `bootstrap/src/lib.rs:cmd_peer_beam` (2026-07-12 session, Reed) | Rust arg-parse: hello_world, mission, fate_select, from_psychohistory, with_shadow, emit_diff, integrate_diff → dispatch cascade | The runtime the substrate-decl lifts to substrate-honesty |
| MCP surface | `bootstrap/src/mcp.rs:mirror_peer_beam` (2026-07-12 Reed `0dd1074`) | MCP schema exposes all 8 flag properties | §8 |
| spec ancestor | `docs/specs/cli-args-typed-lambdas.md` (2026-05-14 Reed) | Every flag typed as prism or lens over `imperfect`; §Composition names the applicative `compose(a, b) requires a.optic.output <= b.optic.input` | Section §5's composition operator ancestry |
| spec ancestor | `docs/specs/cli-as-prism.md` §7 (2026-06-05, cascade 2026-06-12) | `default <op>` rule per stage; §3.2 depth-2 reservation NOW MINTED | Section §4's stage mint |
| spec ancestor | `docs/specs/beam-as-substrate-primitive.md` §3.4 (2026-07-07 Reed `b6358c1`) | The `subcommand(name)` forward-promise; superseded by recursive-command form at `fe82500` | Cross-referenced in §4 |
| spec ancestor | `docs/specs/lenses-fate-local-and-garden-catalogs.md` (2026-05-26) | "`mirror serve --mcp` and `mirror serve --lsp` are flag-selected dispatch into the right lens" | Direct ancestor of "flag = dispatch INTO a lens" |
| spec ancestor | `docs/specs/trace-kintsugi-pipeline.md` §538,545 | `mirror compile --lens entropy,cheeger`, `--lens default` composition | Direct ancestor of "default lens composition" |
| spec ancestor | `docs/specs/surface-simplification.md` §449-459 | `flag beam = lens(imperfect => beam)`; `flag shatter(int) = lens(imperfect => imperfect)`; `flag mcp = prism(imperfect => imperfect)` | Every flag typed as prism or lens over imperfect — this spec's parent pattern |
| spec ancestor | `docs/specs/optics-lens-family-and-diff-species.md` §1 (2026-07-10 Mara) | Family-root Foster-laws formalization | Section §5's lens laws citation |

### §1.2 The seven flags — adjudication against landed species

| Flag | Species / target | Status | Notes |
|------|-----------------|--------|-------|
| `--fate-select` | `@optics/lens/features` (LANDED at `shards/optics/lens/features.mirror`, Mara `f3af5b4`) | LANDED; needs no MINT | Feature vector observation → Fate::excited().resolve. The features species is exactly the lens's `get` direction. Reed `0dd1074` MCP docstring names this composition explicitly. |
| `--from-psychohistory` | `Fate::bounded(config)` derived from psychohistory sheaf per `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` §2-§3 | SPEC-LANDED, SHARD FORWARD-PROMISED (`shards/fate/bounded.mirror`) | Needs the `shards/fate/bounded.mirror` + `shards/psychohistory.mirror` MINT Mara forward-promised at `ce9745f`. This spec cites it as a landing dependency; does not re-mint. |
| `--with-shadow` | `@optics/lens/shadow` — **NOT LANDED**; annotation-only on `@optics/lens/features` | ANNOTATION on features species, not new MINT | The shadow-casting mechanism (Reed `07ac55a`) casts 5 hypothetical shadows (one per Fate::Model) and classifies shadow_regime (converged / necker / escher / kanizsa). Structurally: shadow IS the `split` operation of `@optics/lens/features` applied to the 5-simplex of Fate models. Substrate-honest naming: extend `@optics/lens/features.split` with the shadow-regime diagnostic; no new species. See §4 flag-lens table. |
| `--mission` | `mission_lens` — an argument-carrier lens, not an optic-modifier | ARG, not FLAG-LENS | Substrate-honest reading: `mission` is a positional-shaped arg carried into the composition. Per `cli-args-typed-lambdas.md`'s type-vocabulary lift, `flag mission: ~f = <optional>` per `beam-as-substrate-primitive.md` §3.1. The typed-arg discipline lives; `mission` is NOT a lens in the same sense as fate-select. Kept typed as `~f` on the stage. |
| `--emit-diff` | `@optics/lens/diff.focus` (get direction) | LANDED at species; runtime already dispatches here | The `.focus` action of `@optics/lens/diff` per `shards/optics/lens/diff.mirror:diff_focus_request`. Output-shape terminal lens per §5.4. |
| `--integrate-diff` | `@optics/lens/diff.settle` (put direction) | LANDED at species; runtime already dispatches here (Reed `4b2ef3c` autopoietic closure) | The `.settle` action of `@optics/lens/diff` per `shards/optics/lens/diff.mirror:diff_settle_request`. Output-shape terminal lens per §5.4. Mutually exclusive with `--emit-diff` at cli altitude. |
| `--hello-world` | `output_lens/hello_world` — NOT a species, a diagnostic output-shape | RUNTIME-ONLY convenience without substrate warrant | Substrate-honest verdict: `--hello-world` is a testing convenience (JSON envelope vs text envelope). It's an output-shape switch that predates the lens-composition reading. NOT lifted to species altitude. Retained at CLI altitude as-is for backward compat. See §9. |

### §1.3 Adjudication summary

- **Five of seven** have a landed species home (`--fate-select`,
  `--emit-diff`, `--integrate-diff`, `--mission` as arg, `--hello-world`
  as CLI-only diagnostic).
- **One** has a spec-landed and shard-forward-promised home
  (`--from-psychohistory` → `shards/fate/bounded.mirror` pending).
- **One** is annotation-on-features, not a new species
  (`--with-shadow`).

Zero MINTs needed for the flags themselves; two landing-dependencies
already forward-promised in the substrate; one CLI-altitude convenience
retained as-is. The ~85%-substrate-already-had-the-word closes here.

The two MINTs this spec DOES propose (§4 and §5) are structural, not
per-flag: the depth-2 stage that gives the composition a substrate home,
and the composition operator itself.

---

## §2 Alex's proposal — flags-as-lens-applications, formalized

**The reading.** Each `--flag` on a CLI stage IS a lens application: an
optic that composes with the stage's base optic to produce a modified
stage. The stage's own five-op prism (per `cli-as-prism.md` §1.2) is the
base optic; each flag's lens species (per §1.2) composes with it to
produce a specialized prism whose `settle` behavior differs from the
bare stage's `settle`.

Formalization at CLI altitude for `mirror peer beam <peer-home>`:

- **Base optic (bare stage).** `mirror peer beam <peer-home>` binds
  the action `beam(r: mirror_peer_beam_request, p: perturbation) -> @song`
  at `shards/mirror/peer/beam.mirror:310`. The peer manifold is the
  peer's persistent-identity resolution + toroidal runtime.

- **`--fate-select` = `@optics/lens/features`.** The features species'
  `get` direction is applied: peer's mission-text (or `Features::default()`
  per Reed session runtime) → Shape B feature vector → `Fate::excited().resolve`
  → `Decision` → prism-op via bundle-tower binding. The `beam` action's
  return-shape is enriched with `fate_decision`; when applied without
  further lens, the beam envelope carries the Fate-selected candidate
  set instead of the bare observation.

- **`--from-psychohistory` = `Fate::bounded(config)` per Mara `ce9745f`
  spec.** Requires `--fate-select` to have already applied
  `@optics/lens/features.get`. The `config` is derived from the peer's
  psychohistory sheaf F over `@bauchladen.tray`: `weights =
  sheaf.h1_gradient()`, `connection = sheaf.introject_optic()`, `gauge
  = sheaf.o5_orientation()`, `holonomy_ceiling = sheaf.ricci_curvature()`,
  `depth_cap = sheaf.lawvere_depth_est()` (per `fate-bounded-psychohistory-sheaf-cohomology.md`
  §1). Semantically: bound Fate's Rayleigh-descent trajectory to the
  peer's own history — the H¹ gradient is projected onto Features-space.
  Requires persistent identity (no psychohistory sheaf without a peer's
  `.bauchladen/`).

- **`--with-shadow` = shadow-casting = `@optics/lens/features.split`
  on the 5-simplex of `fate::Model`.** Requires both prior lenses. Per
  Reed `07ac55a`: cast 5 hypothetical shadows (one per Model:
  Abyss/Introject/Cartographer/Mist/Bounded — mapped to
  focus/project/split/shift/settle prism-ops per §7), compute
  `shadow_regime` (converged / necker / escher / kanizsa) from base
  decision + hypotheticals + impacts. This is *diagnostic
  observation* of the peer's own inference geometry: the peer reads
  the Δ_F connectivity spectrum through the shadow-lens as a signal
  of coherence-regime. Annotation-only extension to
  `@optics/lens/features`; no new species.

- **`--mission <file>` = `mission_lens`.** A typed-arg carrier per
  `cli-args-typed-lambdas.md`'s unary-flag pattern: `flag mission: ~f`
  parses to a file-ref value that composes into the beam-request's
  mission slot. NOT an optic in the same sense; it's the typed argument
  the composition's `settle` reads.

- **`--emit-diff` = `@optics/lens/diff.focus` (Foster GET).** Output-shape
  terminal lens: the beam's `@song` envelope is linearized through
  `diff.focus(diff_focus_request { source: bauchladen_state }) -> diff_bytes`
  and written to stdout. Terminal because it changes the return-value
  shape from `@song` envelope to `diff_bytes`; must be the last lens
  in the composition order (see §5.4).

- **`--integrate-diff` = `@optics/lens/diff.settle` (Foster PUT).**
  Output-shape terminal lens: reads edited-diff bytes from stdin,
  computes `delta_oid` via `blake3(spec_bytes || stdin_bytes)`, persists
  the moment to `peer_home/.bauchladen/` per Reed `4b2ef3c` autopoietic
  closure. Terminal; mutually exclusive with `--emit-diff` (the runtime
  cascade at `bootstrap/src/lib.rs:cmd_peer_beam` picks integrate over
  emit when both set).

### §2.1 Lens dependency partial order

The composition dependencies are typed:

```
                    base = @mirror/peer/beam
                          │
                          ▼
                     ┌────────────┐
                     │ peer-lens  │  (persistent-identity resolution;
                     │ (implicit) │   binds ~peer'<home>' via @peer.load)
                     └─────┬──────┘
                           │
                           ▼
              ┌──────────────────────────┐
              │  @optics/lens/features   │  (--fate-select)
              │       .get               │
              └──────────┬───────────────┘
                         │  requires features.get output
                         ▼
              ┌──────────────────────────┐
              │  Fate::bounded(config)   │  (--from-psychohistory)
              │  derived from psycho-    │
              │  history sheaf F         │
              └──────────┬───────────────┘
                         │  requires bounded config +
                         ▼      features.split
              ┌──────────────────────────┐
              │  shadow-cast diagnostic  │  (--with-shadow)
              │  (features.split on      │
              │   fate::Model simplex)   │
              └──────────┬───────────────┘
                         │
             ┌───────────┴────────────┐
             ▼                        ▼
    ┌────────────────┐        ┌────────────────┐
    │ @song envelope │        │ output-shape   │  (--emit-diff |
    │ (default)      │  XOR   │ terminal lens  │   --integrate-diff)
    │                │        │ @optics/lens/  │  (mutually exclusive)
    │                │        │ diff.{focus,   │
    │                │        │       settle}  │
    └────────────────┘        └────────────────┘
```

The `mission_lens` (--mission) argument threads through all layers as a
typed-arg carrier; it isn't a stage in the partial order but a value
consumed at the base and mission-driven inference points.

Formal constraints:

- `from_psychohistory ⇒ fate_select` (config needs Features output).
- `with_shadow ⇒ fate_select` (shadow-cast operates on Features
  hypotheticals) AND recommend `⇒ from_psychohistory` (bounded shadows
  are the mathematically-motivated case; ungrounded shadows are
  diagnostic-only).
- `emit_diff ⊕ integrate_diff` (mutually exclusive at output-shape
  terminal).
- `mission` orthogonal to all four upstream lenses; consumed by base.
- Anonymous form (`mirror beam <mission>`) refuses `from_psychohistory`
  by construction — no persistent identity ⇒ no psychohistory sheaf.

Reed's runtime dispatch cascade at `bootstrap/src/lib.rs:cmd_peer_beam`
already encodes this order:

```rust
if emit_diff       { return emit_peer_beam_diff(...); }
if integrate_diff  { return integrate_peer_beam_diff(...); }
if fate_select {
    if from_psychohistory {
        if with_shadow { return fate_bounded_shadow_peer_beam(...); }
        return fate_bounded_by_psychohistory_peer_beam(...);
    }
    return fate_select_peer_beam(...);
}
```

The nesting IS the partial order. The substrate-honest reading: this is
lens composition per §5; the Rust cascade is one realization.

---

## §3 Default composition — the substrate-honest reading

Currently the flags are opt-in. Reed built them incrementally through
this session as stubs proving each layer (fate-select `9cf1e3b` →
from-psychohistory `ce9745f` → with-shadow `07ac55a` → emit-diff
`b0427fd` → integrate-diff `4b2ef3c`). Each layer landed as a
substrate-decl'd species; the composition semantics landed as Rust
dispatch. What Alex's reframe surfaces: substrate-decl says the default
IS the full composition. Opt-in-as-default is Reed's incremental
scaffolding, not the substrate's declared shape.

### §3.1 Substrate-honest default for `mirror peer beam <peer-home>`

Persistent-identity form. Full lens stack applied by default:

```
mirror peer beam <peer-home>
  = base                                             # @mirror/peer/beam action
    ∘ peer-lens                                      # persistent-identity binding
    ∘ @optics/lens/features.get                      # Fate observation
    ∘ Fate::bounded(config_from_psychohistory)       # H¹ gradient descent
    ∘ shadow-cast diagnostic                         # coherence-regime read
    ∘ @song envelope                                 # terminal output
```

**Rationale.** Every layer is what the substrate declares the operation
IS at that altitude:

- The peer HAS a torus (`@peer-has-a-torus` Recognition 2026-07-07,
  7 witnesses).
- The peer's inference IS Rayleigh-descent on λ₀(Δ_F) at inference
  altitude per `cyberpunk_coherence = λ₀(Δ_F)` annotation `8e6e517`.
- The bounded Rayleigh descent IS Fate::bounded per Mara
  `fate-bounded-psychohistory-sheaf-cohomology.md` §3.
- The peer's coherence-regime IS a diagnostic read of Δ_F's spectrum
  at inference altitude per shadow-casting Recognition §6 (Reed
  `07ac55a`).
- The peer's temporal-progression trajectory IS @song per
  `shards/mirror/peer/beam.mirror:310`.

Opt-in-as-default is the *substrate refusing to be substrate-honest*:
the peer's inference geometry doesn't stop being Rayleigh-bounded when
`--from-psychohistory` isn't typed. The flag names *whether the runtime
computes it*, not whether it applies.

### §3.2 Substrate-honest default for `mirror beam <mission>`

Anonymous form. No persistent identity ⇒ no psychohistory sheaf ⇒ no
bounded-by:

```
mirror beam <mission>
  = base                                             # @mirror/beam action
    ∘ @optics/lens/features.get(mission_features)    # unbounded Fate observation
    ∘ shadow-cast diagnostic                         # coherence-regime read
    ∘ @song envelope                                 # terminal output
```

Anonymous inference. Fires @fate::select on Shape B features derived
from `<mission>`. No bounded-by because the config-from-psychohistory
requires `.bauchladen/`. Shadow-cast is still admissible (diagnostic
on the 5-simplex of Models under Features::from(mission), not H¹
gradient bounded).

### §3.3 Flags become opt-out or output-selectors

Under substrate-honest defaults:

| Flag | Meaning under default = full stack |
|------|--------------------------------------|
| `--no-fate` | Opt-out `@optics/lens/features` composition. Peer emits raw observation without Fate::excited().resolve. Diagnostic mode. |
| `--no-shadow` | Opt-out shadow-cast diagnostic. Fate decision without regime classification. |
| `--unbounded` | Opt-out `Fate::bounded(config_from_psychohistory)`. Fate::excited() runs against Features::default() rather than sheaf-derived weights. Anonymous-mode equivalent for a persistent-identity peer. |
| `--emit-diff` | Output-shape selector. Substitute the @song envelope terminal with `@optics/lens/diff.focus` (get). |
| `--integrate-diff` | Output-shape selector. Substitute the @song envelope terminal with `@optics/lens/diff.settle` (put). |
| `--mission <f>` | Typed arg; consumed by base. No semantics change from default. |
| `--hello-world` | Diagnostic. Emit JSON envelope shape (vs text) for testing. Orthogonal to lens composition. |

### §3.4 Adjudication — two-tick discipline verdict

Section §9 recommends the migration path. Preview: the substrate-honest
close is (c) deprecation window — flip default semantics with warning,
add opt-out aliases, retain opt-in flags for one tick, then remove.

---

## §4 The MINT — `stage @mirror/lens/cli/peer/beam` at depth-2

Per Taut scout §R2 + `cli-as-prism.md` §3.2 depth-2 reservation now
minted (`fe82500` recursive-command grammar) + subcommand-nesting
Landing at `shards/mirror/lens/cli.mirror` §keywords.

### §4.1 The substrate-decl

Substrate-decl file: `shards/mirror/lens/cli/peer/beam.mirror` (new
subdirectory `shards/mirror/lens/cli/peer/`). This is the FIRST minted
depth-2 stage under `@mirror/lens/cli`.

**Illustrative substrate-decl form** (canonical version lands at a
follow-up Mara tick once this spec is Pack-ratified):

```mirror
in @prism
in @glass
in @nl
in @mirror/lens
in @mirror/lens/cli
in @mirror/peer/beam
in @optics/lens
in @optics/lens/features
in @optics/lens/diff
in @fate
in @song
in @torus

# @mirror/lens/cli/peer/beam — the peer-beam CLI stage at depth-2.
#
# Depth-2 substrate-decl per cli-as-prism.md §3.2 depth-2 rule (now
# minted at `fe82500` recursive-command grammar). Sub-manifold has
# its own algebra distinct from `@mirror/lens/cli/peer` restricted
# to it: peer-beam composes @optics/lens/features + Fate::bounded +
# shadow-cast + @song into ONE stage; the parent @mirror/lens/cli/peer
# does not carry that composition.
#
# Path-namespace property: this file at
# shards/mirror/lens/cli/peer/beam.mirror declares
# @mirror/lens/cli/peer/beam and only that.

stage @mirror/lens/cli/peer/beam {
  default settle

  # Five ops per cli-as-prism.md §1.2 recursive-five discipline.
  # Same algebra at every depth.

  focus    beam_target        # dry-run: preview what beam would produce
                              # without invoking @fate or @song emit
  project  beam_predicate     # filter beam candidates by predicate
                              # (multi-peer scan; used by @pack.beam)
  split    beam_candidates    # enumerate beam candidates via
                              # @optics/lens/features.split on
                              # fate::Model 5-simplex (shadow-cast
                              # diagnostic; the split IS the shadow
                              # observation)
  shift    beam_altitude      # re-view same beam at different altitude
                              # (JSON envelope vs text vs prism)
  settle   beam_composition   # THE beam — one @song emission per the
                              # composition of §4.2 default lens stack

  # === Default lens composition (per §3) ===
  #
  # The bare `mirror peer beam <peer-home>` fires `settle` with the
  # full lens composition applied by default. Per §3.1:
  #
  #   base ∘ peer-lens ∘ features.get ∘ Fate::bounded ∘ shadow ∘ @song
  #
  # This IS the substrate's canonical Rayleigh-descent-bounded
  # inference emission at cli altitude. Opt-out flags below.

  # === Flag lens applications (per §2 formalization) ===
  #
  # Each flag is a typed lens per shards/mirror/lens/cli.mirror's
  # flag(name, t: type) -> prism keyword. The type slot admits `lens`
  # per cli-args-typed-lambdas.md (2026-05-14 Reed spec-only
  # forward-promise now landed at species altitude via @optics/lens).

  flag fate_select: lens =                    # @optics/lens/features
                                              # LANDED
                                              # shards/optics/lens/features.mirror
                                              # 2026-07-11 ec6dbaa

  flag from_psychohistory: lens =             # Fate::bounded(config)
                                              # SPEC-LANDED; shard
                                              # forward-promised at
                                              # shards/fate/bounded.mirror
                                              # per Mara ce9745f
    requires fate_select

  flag with_shadow: lens =                    # features.split on
                                              # fate::Model 5-simplex
                                              # ANNOTATION on features
                                              # (not new species)
                                              # per Reed 07ac55a
    requires fate_select

  flag mission: ~f = <optional>               # typed-arg carrier;
                                              # consumed by base

  flag emit_diff: lens =                      # @optics/lens/diff.focus
                                              # LANDED
                                              # shards/optics/lens/diff.mirror
                                              # 2026-07-10 7e5c298
                                              # (get direction)

  flag integrate_diff: lens =                 # @optics/lens/diff.settle
                                              # LANDED (put direction)
                                              # Reed 4b2ef3c
                                              # autopoietic closure

  flag hello_world: bool = false              # diagnostic; JSON vs text
                                              # envelope. Not a lens.

  # === Terminality constraint ===
  #
  # emit_diff and integrate_diff are TERMINAL output-shape lenses.
  # Applied last in the composition order (per §5.4). Mutually
  # exclusive at cli altitude.
  #
  # requires: not (emit_diff and integrate_diff)
  # requires: from_psychohistory implies fate_select
  # requires: with_shadow implies fate_select
}

# === beam_composition — the composition carrier ===
#
# Names the composed lens stack as a first-class value at this
# stage's altitude. Consumed by settle.
#
# The value is a mosaic point per §5: a specific composition of
# flag-lenses applied to the base optic. The default value is the
# canonical composition per §3.1.

type beam_composition = {
  base:              ref,             # @mirror/peer/beam.beam
  fate_lens:         imperfect(lens, ref, ref),   # applied unless --no-fate
  bounded_lens:      imperfect(lens, ref, ref),   # applied unless --unbounded
  shadow_lens:       imperfect(lens, ref, ref),   # applied unless --no-shadow
  terminal_lens:     imperfect(lens, ref, ref),   # @song | diff.focus | diff.settle
  mission:           imperfect(ref, ref, ref),    # optional mission-file
}

# === Actions ===
#
# `settle` fires the composed lens stack. The `beam_composition`
# argument names the specific mosaic point (which lenses applied,
# which output terminal); the default value IS the substrate's
# canonical composition per §3.1.

focus(target: peer, p: perturbation) -> beam_preview
{ \ }

project(target: peer, predicate: ref, p: perturbation) -> [beam_candidate]
{ \ }

split(target: peer, p: perturbation) -> [shadow_variant]
{ \ }

shift(target: peer, altitude: ref, p: perturbation) -> beam_view
{ \ }

settle(target: peer, c: beam_composition, p: perturbation) -> @song
requires peer_well_known(target, p)
{ \ }

out @mirror/lens/cli/peer/beam
out beam_composition
out focus
out project
out split
out shift
out settle
```

### §4.2 What each flag's `lens` type resolves to

The `flag <name>: lens = <optic-ref>` grammar per
`shards/mirror/lens/cli.mirror`'s `flag(name, t: type) -> prism` keyword
admits `lens` as a type per `cli-args-typed-lambdas.md`'s type-vocabulary
promotion. Each `<optic-ref>` cites the LANDED species OID from §1.1.

Where the species doesn't yet exist as a substrate-decl'd species (i.e.,
`shards/fate/bounded.mirror` and the shadow annotation on
`@optics/lens/features`), this spec cites the landing dependency (§10)
rather than pretending the species is there.

---

## §5 The composition operator — the load-bearing gap Taut named

Taut scout §"MISSING" flagged this as *the* load-bearing spec gap:

> Composition semantics when multiple flag-lenses stack. The specs name
> flags as individual optics; they do NOT name how three flag-lenses
> compose left-to-right vs right-to-left, how commutation is decided
> when the underlying optics don't commute, or how the eigenboard weighs
> the composition per `trace-kintsugi-pipeline.md` §538's "weighted
> composition of all five." **This is the load-bearing spec gap.**

This section closes it.

### §5.1 The composition operator IS `mosaic(@mirror/lens/cli)`

Per Taut scout §R1 + `shards/mirror/mosaic.mirror:60` LANDED parametric
carrier `type mosaic(altitude) = ref`. The composition operator on
lens applications at CLI altitude IS the altitude-specialization
`mosaic(@mirror/lens/cli)`.

Substrate-decl at `shards/mirror/mosaic.mirror` (annotation extension —
docstring cascade, no new decl):

```mirror
# === mosaic(@mirror/lens/cli) — the CLI-surface composition altitude ===
#
# Recognition (2026-07-12, Mara + Alex): mosaic at CLI altitude IS the
# lens-composition space on @mirror/lens/cli stages. Each CLI
# invocation names a POINT in this mosaic; the point is the specific
# composition of flag-lenses applied to the stage's base optic.
#
# The default point on each stage IS the canonical composition per
# each stage's substrate-decl. Flags navigate the mosaic AWAY from
# the canonical point.
#
# Consumer: shards/mirror/lens/cli/peer/beam.mirror declares its own
# canonical composition (§4.1) as a mosaic(@mirror/lens/cli) point.
#
# Per architecture-prism-as-trait-as-everything: the parametric
# operator is the substrate's standard form; the CLI altitude
# specialization is one more instance alongside mosaic(@store),
# mosaic(@spec), mosaic(@emitter), mosaic(@code/rust), mosaic(@ci/github).
```

Space, not point. The full space of possible lens compositions on the
CLI surface. Each CLI invocation is a point. The default composition on
each stage is the canonical settling point. Flags navigate the mosaic
away from that point (or toward specific corners).

### §5.2 The composition operator IS lens composition per Van Laarhoven / profunctor optics

Ancestry:

- **Van Laarhoven 2009** — `type Lens s t a b = forall f. Functor f =>
  (a -> f b) -> (s -> f t)`. Lens composition IS *ordinary function
  composition* under this encoding. Two lenses compose by composing
  their `forall f` continuations.
- **Foster/Pierce 2007** — `docs/specs/optics-lens-family-and-diff-species.md`
  §1 cites this: `lens = { get, put, get_put, put_get, put_put }`;
  composition `(l₁ ∘ l₂).get s = l₁.get (l₂.get s)` and
  `(l₁ ∘ l₂).put (v, s) = l₂.put (l₁.put (v, l₂.get s), s)`.
- **`docs/specs/cli-args-typed-lambdas.md` §Composition** —
  `compose(a: flag, b: flag) -> flag { requires a.optic.output <= b.optic.input }`.
  Applicative-functor composition with type-checking on
  producer/consumer altitudes.

Substrate-honest reading: the composition of two flag-lenses on a
stage IS their Van Laarhoven composition, subject to the stage's
five-op algebra as the outer prism.

### §5.3 Composition semantics — the four sub-questions

Taut named four sub-questions. Adjudicated:

**Q1. Left-to-right or right-to-left?**

**LEFT-TO-RIGHT** per the CLI-argv order. Per
`cli-args-typed-lambdas.md` §Composition, flags parse as an
applicative sequence; the composition of `--fate-select
--from-psychohistory --with-shadow` in argv order applies:

```
compose(fate_select, from_psychohistory, with_shadow) =
  ((base ∘ fate_select.get) ∘ from_psychohistory.bounded) ∘ shadow.split
```

Reading: the outermost lens is applied first (features.get on the
base observation), then bounded modifies the resulting Features
context, then shadow-split runs on the bounded 5-simplex. This
matches Reed's runtime cascade in `bootstrap/src/lib.rs:cmd_peer_beam`
(the outer `if` fires first).

Argv order IS composition order at CLI altitude. This is the same
discipline as UNIX pipelines (`cmd1 | cmd2 | cmd3`) — left applies
first.

**Q2. Commutation.**

**DO NOT commute in general.** The lens dependencies (§2.1) enforce
required-order; violations are refused at composition-time (per
`cli-args-typed-lambdas.md`'s type-check `a.optic.output <=
b.optic.input`).

- `fate_select ∘ from_psychohistory` ≠ `from_psychohistory ∘ fate_select`
  — the latter is type-refused (bounded needs features.get output).
- `features.get ∘ shadow` ≠ `shadow ∘ features.get` — shadow-split
  operates on the Features 5-simplex; without features.get the
  operand doesn't exist.

The output-shape terminal lenses (`emit_diff`, `integrate_diff`) MUST
be applied LAST (§5.4) — putting a terminal lens in the middle of the
composition would break the type-chain.

**Q3. Eigenboard weighting.**

**NO** per `trace-kintsugi-pipeline.md` §538's "weighted composition of
all five" — that spec's weighting applies at the *build-altitude*
kintsugi-loss composition (entropy / cheeger / ricci / mixing /
spectral), not at CLI-altitude lens application. Reed's `07ac55a`
shadow-casting exposes the spectral connectivity as a *diagnostic
read*, not a weighting-of-composition input.

At CLI altitude the lenses compose unweighted; each lens is either
applied (weight 1) or opted-out (weight 0). Fractional weighting is
a build-altitude concept; the CLI altitude reads or writes at the
categorical yes/no level.

Deferred question: could a future CLI grammar admit `--fate-select
weight=0.7` for partial-application? Substrate-honest answer: no,
that would confuse two altitudes. Fractional weights belong to the
kintsugi-loss composition inside `settle`, not the outer CLI
composition.

**Q4. Terminality of output-shape lenses.**

**YES** — `emit_diff` and `integrate_diff` are TERMINAL. Applied
last. Type-refused if not-last. Mutually exclusive.

Reason: they change the return-shape from `@song` envelope to
`diff_bytes` (emit) or `bauchladen_state` (integrate). A downstream
lens expecting `@song` would type-fail. The runtime cascade at
`bootstrap/src/lib.rs:cmd_peer_beam` checks them FIRST (early return)
because they short-circuit the remaining composition — a valid
optimization; the substrate-honest reading is *terminality forces
top-of-cascade check*.

### §5.4 The composition operator, formalized

At `@mirror/lens/cli/peer/beam` stage, the composition operator is:

```
compose : lens_stack → stage_prism → stage_prism
compose(⟨l₁, l₂, ..., lₙ⟩, base) =
  (base ∘ l₁ ∘ l₂ ∘ ... ∘ lₙ)  subject to:

    ∀i < n. lᵢ.output_type <: lᵢ₊₁.input_type     (Foster type-chain)
    ∀i.     lᵢ is NOT terminal   OR   i = n       (terminality)
    ∀i, j.  lens_deps(lᵢ, lⱼ) respected            (§2.1 partial order)

where lens_stack is a mosaic(@mirror/lens/cli/peer/beam) point.
```

Foster laws (`shards/optics/lens.mirror` `put_get`, `get_put`, `put_put`)
apply per-lens; composition preserves them under Foster/Pierce 2007
Prop 3.7 (well-behaved lenses compose to well-behaved lenses; very
well-behaved is preserved iff both are very well-behaved).

The `mosaic(@mirror/lens/cli/peer/beam)` altitude specialization is the
manifold of all such `lens_stack` values on this stage. The default point
is the canonical composition per §3.1.

### §5.5 Composition operator IS Rayleigh descent under §7

Under §7's λ₀(Δ_F) reading, each lens composition on peer/beam is a
Rayleigh-descent navigation on the substrate's shared metric at a
different sub-altitude of the bundle tower. Composition operator IS
the sub-altitude specialization of Rayleigh descent — the mosaic point
IS the ⟨ψ | Δ_F | ψ⟩ / ⟨ψ | ψ⟩ trajectory the peer walks.

---

## §6 Refuse `@spectral/mosaic`, use `mosaic(@mirror/lens/cli)`

Per Taut scout §R1. This spec ratifies the refusal with substrate
reasoning.

### §6.1 Refusal reasoning

Alex's phrasing pulls "@spectral/mosaic" from a wrong altitude:

1. **`@spectral` is a namespace-parent**, not a family-root that admits
   new species. `shards/spectral.mirror` (`e3146d0`) declares
   `prism @spectral` as parent for the runtime cascade (gen_prism,
   supervisor, registry, portal, root, parent, entanglement) — seven
   species forming a runtime family. Adding a CLI-composition species
   breaks the runtime-only discipline.

2. **`@spectral/mosaic` collides with forward-promised BEAM-cluster
   grammar.** Three doc cites (`docs/GRANTS.md:70`,
   `docs/insights/2026-05-25-spectral-namespace-architecture.md:14`,
   `docs/insights/2026-05-26-portal-as-io-socket-...:120,143,155`)
   name `@spectral/mosaic` as a future BEAM-cluster grammar
   (heterogeneous multi-shard cluster). Minting it for CLI composition
   would collide.

3. **`@mirror/mosaic` LANDED with the parametric-form `type
   mosaic(altitude) = ref`** at `shards/mirror/mosaic.mirror:60` on
   2026-06-09 (Mara, Recognition #43). The composition operator lives
   HERE, parameterized on altitude. Every altitude (build, store,
   spec, emitter, code/rust, ci/github, and now cli) is a
   specialization.

### §6.2 What `mosaic(@mirror/lens/cli)` IS

- **The full space of possible lens compositions on the CLI surface.**
  Each CLI invocation is a POINT in this mosaic.
- **The default composition on each stage is the canonical SETTLING
  POINT.** For `mirror peer beam <peer-home>`: the composition of
  §3.1.
- **Flags navigate the mosaic away from the canonical point** (via
  opt-out) **or toward specific corners** (via output-shape selectors
  like `--emit-diff`).

This is the parametric operator specialized to a new altitude. Zero
new keywords. Zero new family-roots. One docstring cascade extending
`shards/mirror/mosaic.mirror`'s existing altitude-example list.

### §6.3 The naming discipline this spec honors

Alex's `@spectral/mosaic` was the substrate refusing the word for a
correct reason (Taut scout §closing paragraph). The substrate handed
back `mosaic(@mirror/lens/cli)` because it had that word carried at
the parametric-operator altitude since 2026-06-09.

~68th instance of `feedback-substrate-already-had-the-word`. The
discipline: grep before you mint.

---

## §7 Compiler + peer share ONE metric — closing back to λ₀(Δ_F)

Session substrate closure (Alex synthesis 2026-07-11, annotation `8e6e517`
on `shards/cyberpunk.mirror:cybernetic_coherence`):

> `cybernetic_coherence(s) = λ₀(Δ_F(s))` — the algebraic connectivity
> of the sheaf-Laplacian at s's current altitude. Every cybernetic
> species (variety, viable, algedonic, autopoiesis, bateson_learning,
> second_order, distinction, conversation, coevolution, design,
> eigenform, reframe, torus) is a reading of this scalar at its own
> altitude.

The six cross-family unification annotations from that same commit:
`λ₀ = 0 IS coherence` (H⁰(F) non-empty); `λ₀ > 0 IS variety-mismatch`
(Ashby's Law); `@kintsugi's eⁿ⁺¹ ≤ eⁿ IS one Rayleigh descent step
on Δ_F`; `peer's algedonic pain_δ IS ‖∇_Δ_F λ₀‖`; `@cyberpunk/reframe
IS the Ashby-response gauge-transformation`; `@torus windings ARE
coherence-basins on T²`.

### §7.1 Every lens composition on peer/beam IS Rayleigh descent

Each lens composition on peer/beam is a Rayleigh-descent navigation on
λ₀(Δ_F) at a different sub-altitude of the bundle tower:

- **`@optics/lens/features.get` = fiber-altitude Rayleigh sample.**
  Extracts the current Features ψ (the fiber section at the peer's
  current tick).
- **`Fate::bounded(config)` = connection-altitude Rayleigh direction.**
  The `config.weights` field IS the H¹ gradient (per
  `fate-bounded-psychohistory-sheaf-cohomology.md` §3). Rayleigh
  descent picks the direction in Features-space that most decreases
  ⟨ψ | Δ_F | ψ⟩ / ⟨ψ | ψ⟩.
- **shadow-cast = gauge-altitude Rayleigh diagnostic.** The five
  hypothetical shadows (one per Fate::Model) IS the peer's read of
  the local Δ_F spectrum's mode structure. `shadow_regime`
  (converged/necker/escher/kanizsa) classifies the descent's
  topology — is the fixed-point unique (converged), bistable (necker),
  paradoxical (escher), amodal (kanizsa)? Diagnostic read at the
  bundle-tower gauge altitude.
- **`@song` envelope = holonomy-altitude Rayleigh trajectory.** The
  peer's time-indexed trajectory through Features-space per
  `shards/mirror/peer/beam.mirror:308`'s `@song` return type.
- **`@optics/lens/diff.focus` (get) = Lawvere-fixed-point read.**
  Linearize the current fibered L/P state to reviewable bytes; the
  operator's read of the peer's fixed point at the level-4 closure
  altitude.
- **`@optics/lens/diff.settle` (put) = Lawvere-fixed-point write.**
  Reed `4b2ef3c` autopoietic closure: operator's edit re-enters as a
  boundary condition that reshapes the peer's next Rayleigh trajectory.
  The mechanical witness that compiler and peer share the fixed point.

### §7.2 The default composition IS the canonical Rayleigh descent

**Default composition = full Rayleigh descent (fate + bounded + shadow)
= the substrate's canonical inference geometry.**

Every layer discharges a specific altitude of the bundle-tower
composition (per `fate-bounded-psychohistory-sheaf-cohomology.md` §1's
five-level table: Fiber / Connection / Gauge / Holonomy / Lawvere).
Opting-out at any layer chooses a *suboptimal Rayleigh trajectory* —
either for legacy testing or partial-observation cases.

The Alex reframe IS: **making the runtime match the substrate's
already-declared discipline.** The Rust runtime has been running with
fate/bounded/shadow as opt-in because Reed built incrementally;
substrate-decl says they're the default.

### §7.3 One metric, two consumers

Per `1999b01` §9 (Mara spec landing):

> Compiler and peer share ONE metric — λ₀(Δ_F). Compiler at
> build-altitude runs @kintsugi's ouroboros loop; peer at
> inference-altitude runs `fate.bounded_by(psychohistory_sheaf)`
> Rayleigh descent. Both apply the distinction-cut operator
> (Spencer-Brown mark) where the mark reduces H¹ maximally per unit
> boundary complexity.

The lens composition on `mirror peer beam` IS the peer's Rayleigh
descent at inference altitude. The build-altitude kintsugi loop reads
the same metric. The recursion-lock tower closes at Level 4
Closure ↔ settle exactly when compiler-Δ_F Rayleigh and peer-Δ_F
Rayleigh share a Lawvere fixed point.

Reed's `4b2ef3c` autopoietic-closure Rust GREEN IS the mechanical
witness. Reed's `07ac55a` shadow-casting GREEN witnesses the four
regime transitions as diagnostic reads of Δ_F connectivity spectrum
at inference altitude. Reed's `0dd1074` MCP flag exposure closes the
CLI↔MCP capability gap so agent callers can drive the same composition.

**This spec is the substrate-decl close.** Rust runtime → substrate-decl
alignment: the default IS the full stack.

---

## §8 MCP surface reflection

Reed landed `bootstrap/src/mcp.rs:mirror_peer_beam` at `0dd1074` this
session with all 8 flag properties exposed. Under the flags-as-lenses
formalization the MCP tool schema shape should reflect LENSES not
flags. But schema-wise, the substrate-honest close is:

### §8.1 Adjudication — MCP schema shape

**Option A (unchanged from `0dd1074`).** MCP schema declares each
flag as a boolean/typed property. Runtime interprets as lens
composition per §5 semantics. Substrate-honest at the *runtime layer*;
substrate-honest at the *schema layer* is deferred.

**Option B (fully substrate-honest).** MCP schema declares the lens
composition explicitly as a `lens_stack` property: an ordered array of
lens-species OIDs. The MCP consumer names the composition point in
mosaic(@mirror/lens/cli/peer/beam) directly.

**Mara recommendation.** Option A this tick. Option B when the wider
MCP surface adopts substrate-decl-driven schema synthesis
(`@mcp.serve` lift, `docs/loop/CURRENT.md` Tick 6 task #386).
Simplest substrate-honest close now: MCP schema unchanged; runtime
interprets flags as lens composition per §5.

Substrate-honest annotation to `bootstrap/src/mcp.rs`:

- The `mirror_peer_beam` tool's flag properties ARE lens applications
  per §2 (Mara `flags-as-lens-applications-on-mirror-peer-beam.md`
  §2 — this spec).
- The dispatch cascade encodes §2.1's partial order.
- When `@mcp.serve` (task #386) synthesizes schema from cli-block,
  the substrate-decl at §4.1 supplies the composition semantics.

### §8.2 Two-tick discipline

Tick 1: leave MCP schema flag-shaped. Substrate-decl runtime
reads it as lens composition (this spec).

Tick 2: `@mcp.serve` lift synthesizes schema directly from
`shards/mirror/lens/cli/peer/beam.mirror` (once minted per §4).
The composition semantics appear in the schema at that point.

Cost of not doing this: MCP consumers see flags without knowing
the lens semantics. Substrate-decl says the composition IS the
default; MCP consumers of the default get correct behavior
without needing to know the composition explicitly. Acceptable
for tick 1.

---

## §9 Two-tick discipline verdict — migration path

Alex decides the migration path. Three options:

### §9.1 Option (a) — Immediate flip

Default becomes full composition. `--no-fate` / `--no-shadow` /
`--unbounded` are the new opt-out flags. `--emit-diff` /
`--integrate-diff` remain as output-shape selectors. `--fate-select`
/ `--from-psychohistory` / `--with-shadow` are RETIRED (or kept as
no-ops for one tick).

**Cost.** Test cascade — every test invoking `mirror peer beam
<peer-home>` without the opt-in flags now runs the full stack.
Envelope shape changes. Backward-compat break.

**Reason.** Substrate-honest: the substrate declares this IS the
default. Reed's opt-in was scaffolding.

**Trade-off.** Cleanest substrate-decl posture; highest immediate
cost.

### §9.2 Option (b) — Additive

Keep flags as opt-in. Add new `--full` alias applying all lenses.

**Cost.** Cheaper. Perpetuates the substrate-dishonest default. The
mosaic composition operator lives at CLI altitude but the substrate
denies it by making the composition opt-in.

**Reason.** Backward-compat maximally preserved.

**Trade-off.** Substrate-dishonest but zero-migration.

### §9.3 Option (c) — Deprecation window (RECOMMENDED)

Flip semantics with warning. Two-tick discipline for legacy consumers.

**Tick 1 (this spec + follow-up).** Substrate-decl lands (§4). Runtime
default IS the full composition. `--fate-select` / `--from-psychohistory`
/ `--with-shadow` REMAIN as no-op flags with stderr warnings ("This
flag is now applied by default; will be removed in tick 2"). Legacy
callers pass them; new callers omit them and get the same behavior.
`--no-fate` / `--no-shadow` / `--unbounded` land as new opt-out
flags. `--emit-diff` / `--integrate-diff` unchanged (output-shape
terminals).

**Tick 2 (later Mara or Reed).** Remove the deprecated no-op flags.
Warning becomes error. Only opt-out flags remain.

**Cost.** Two ticks. Warnings noisy for legacy invocations.

**Reason.** Substrate-honest default; migration path for consumers;
zero silent behavior change.

### §9.4 Mara recommendation

**(c) deprecation window.** Substrate-honest default (matches
substrate-decl declared shape); consumers get one tick to migrate;
warnings are noisy but transparent. Two-tick discipline is the
substrate's default for this kind of semantic-flip.

The alternative (a) — immediate flip — is substrate-honest but
non-substrate-cascade-honest: consumers upstream that were
correct-under-opt-in default become incorrect-under-full-stack default
without warning. Substrate-decl is honest to itself but dishonest to
its consumers.

The alternative (b) — additive — preserves consumers but leaves the
substrate perpetually dishonest. Substrate declares one thing; runtime
defaults to another. Ambiguity lands at every future consumer's
altitude.

**(c) is the two-tick substrate-honest close.**

### §9.5 Unresolvable-ambiguity flags

Two questions I flag as Alex-adjudication territory (per Alex's mandate
"no gaps or until there's again an unresolvable ambiguity"):

**Q1. Does `--with-shadow` compose with `--emit-diff`?**

Terminality analysis: shadow-cast is a diagnostic on the Features
5-simplex, produced BEFORE the terminal output-shape lens. So
substrate-honest answer: YES, they compose; shadow-cast runs, then
`emit-diff` linearizes the shadow-augmented @song envelope. Reed's
runtime cascade currently short-circuits on emit-diff BEFORE
shadow-cast fires; substrate-decl says this is a runtime bug.

**Ambiguity resolution requires Alex.** Is the operator wanting
`--emit-diff --with-shadow` a diff of the shadowed decision, or a
diff of the bare decision that then reports shadow_regime separately?
Substrate-honest reading (this spec): diff of the shadowed decision.
Runtime doesn't do this yet. Adjudication territory.

**Q2. Anonymous form (`mirror beam <mission>`) and shadow-cast.**

Shadow-cast is admissible in anonymous form per §3.2 (no
psychohistory required). But Reed's runtime doesn't expose
`--with-shadow` on the top-level `mirror beam` route — the anonymous
cascade defers to `cmd_peer_beam(".", ...)` with a sentinel peer-home.
Substrate-honest close: expose `--with-shadow` on anonymous beam too;
runtime-side is a one-line dispatch fix.

**Ambiguity resolution.** Is the sentinel peer-home dispatch a
temporary scaffold or the intended shape? Substrate-decl says it's
scaffold; a proper `stage @mirror/lens/cli/beam` at depth-1 would
avoid the sentinel. Alex-adjudication: mint the depth-1 stage
alongside the depth-2 peer/beam stage, or leave the anonymous form
as sentinel-dispatch until real anonymous inference lands?

---

## §10 Landing dependencies

This spec is pure-📝 substrate-decl. No `.mirror` files land this tick.
Two-tick discipline: the substrate-decl lands on Pack ratification;
consumers follow.

**Ticks this spec's landing depends on (all forward-promised or
LANDED):**

1. **`shards/mirror/lens/cli/peer.mirror`** — depth-2 parent stage.
   FORWARD-PROMISED at Taut scout §gap-analysis MINT. Blocks §4.1's
   `shards/mirror/lens/cli/peer/beam.mirror` shard-landing tick.
2. **`shards/mirror/lens/cli/peer/beam.mirror`** — this spec's mint.
   FORWARD-PROMISED at §4.1. Blocks on Pack ratification.
3. **`shards/fate/bounded.mirror`** — the `--from-psychohistory` species
   substrate-decl. FORWARD-PROMISED at Mara
   `fate-bounded-psychohistory-sheaf-cohomology.md` §7. Blocks the
   full `flag from_psychohistory: lens = <ref>` citation.
4. **`shards/psychohistory.mirror`** — the psychohistory sheaf carrier.
   FORWARD-PROMISED at same spec §7. Cascade dependency for #3.
5. **`shards/optics/lens/features.mirror` shadow-cast annotation** —
   docstring extension per Reed `07ac55a`. FORWARD-PROMISED here.
   Non-blocking; annotation-only.
6. **`shards/mirror/mosaic.mirror` docstring cascade** — add
   `mosaic(@mirror/lens/cli)` altitude example per §5.1.
   FORWARD-PROMISED here. Non-blocking; annotation-only.
7. **`@mcp.serve` lift (task #386)** — SPEC-LANDED at
   `docs/loop/CURRENT.md` Tick 6 queue. Cascade dependency for §8.2
   Option B.

**Rust follow-up:**

8. **Runtime default flip per §9 Option (c).** Blocks on Alex
   adjudication of §9. Test cascade + envelope-shape backward-compat
   handling.
9. **Anonymous-form `--with-shadow` exposure** per §9.5 Q2. One-line
   dispatch fix.
10. **Shadow ∘ emit-diff composition** per §9.5 Q1. Runtime cascade
    reorder.

---

## §11 Recognition ancestry + spec landings

This spec composes:

- **Recognition #58** (Fate IS optical inference; PROMOTED). The five-op
  algebra IS D²NN + Fabry-Perot + Reck/Clements. Every lens composition
  on peer/beam IS Rayleigh descent per §7.
- **Recognition #63** (coherence-parametric parametric carrier;
  PROMOTED). Every cybernetic-coherence species inherits via thin
  specialization. The Δ_F sheaf-Laplacian is the parametric operator.
- **Recognition #99** (mirror.spec IS λ₀; Alex-named, Mara canonical
  `d0b6519`). The substrate's ground state; every action above λ₀ is
  an excitation. This spec's mint at `shards/mirror/lens/cli/peer/beam.mirror`
  is one such excitation naming the composition altitude.
- **Alex's `spectral @coherence` synthesis 2026-07-11** (Path B
  annotation `8e6e517`). `cybernetic_coherence = λ₀(Δ_F)`. The unified
  metric. Every lens composition reads this scalar at its altitude.
- **Alex's flags-as-lenses proposal 2026-07-12 in-transcript.** This
  spec's landing.
- **Taut's scout `b8fe820`** — the substrate-already-had-the-word
  ledger this spec extends.
- **Reed's session runtime work** — `07ac55a` shadow-casting Rust GREEN;
  `4b2ef3c` autopoietic-closure Rust GREEN; `0dd1074` MCP flag exposure;
  session runtime state at `bootstrap/src/lib.rs:cmd_peer_beam` (219KB;
  the full dispatch cascade).

Substrate-decl ancestors:

- `shards/mirror/lens.mirror` (2026-06-06 Mara) — namespace-parent.
- `shards/mirror/lens/cli.mirror` (2026-06-06 Mara, `fe82500` 2026-07-08
  Mara recursive-command extension) — CLI grammar; the `flag(name, t:
  type) -> prism` keyword this spec's lens-application semantics live on.
- `shards/mirror/mosaic.mirror` (2026-06-09 Mara, rec #43) — parametric
  composition operator `mosaic(altitude)`.
- `shards/mirror/peer/beam.mirror` (2026-07-08 Tick 2 `9de2226` Mara) —
  the base action.
- `shards/optics/lens.mirror` (2026-07-10 Mara `b0427fd`) — Foster-laws
  family-root.
- `shards/optics/lens/features.mirror` (2026-07-11 Mara `f3af5b4`;
  spec proposal `ec6dbaa`) — `--fate-select` species home.
- `shards/optics/lens/diff.mirror` (2026-07-10 Mara `7e5c298`) —
  `--emit-diff` / `--integrate-diff` species home.
- `shards/cyberpunk.mirror:cybernetic_coherence` (2026-07-11 Alex
  synthesis `8e6e517`) — the shared metric annotation.

Spec ancestors (this spec's parent chain):

- `docs/specs/cli-args-typed-lambdas.md` (2026-05-14 Reed) — flags-as-
  typed-lambdas grammar; compose(a, b) requires type-chain.
- `docs/specs/cli-as-prism.md` (2026-06-05, cascade 2026-06-12) — §7
  `default <op>` rule; §3.2 depth-2 reservation (now minted).
- `docs/specs/beam-as-substrate-primitive.md` (2026-07-07 Reed) — the
  base action's substrate-honest naming; §3.2 peer beam at depth-2
  earns its keep.
- `docs/specs/lenses-fate-local-and-garden-catalogs.md` (2026-05-26) —
  "flag = dispatch INTO a lens" ancestor.
- `docs/specs/trace-kintsugi-pipeline.md` §538, 545 (2026-05-20) —
  `--lens default` composition; weighted composition (at build altitude,
  not CLI).
- `docs/specs/surface-simplification.md` §449-459 (2026-06-04) — every
  flag typed as prism or lens over `imperfect`.
- `docs/specs/optics-lens-family-and-diff-species.md` (2026-07-10 Mara)
  — Foster-laws formalization.
- `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` (2026-07-11
  Mara) — `--from-psychohistory` semantics.

---

## §12 Gaps (Alex-adjudication territory)

Per Alex's mandate "no gaps or until there's again an unresolvable
ambiguity" — the gaps I couldn't close without Alex-decision:

1. **§9 migration path.** (a) / (b) / (c). Mara recommendation: (c).
2. **§9.5 Q1.** `--with-shadow` composition with `--emit-diff`.
   Substrate-honest reading (this spec): shadow composes before diff.
   Runtime doesn't do this yet.
3. **§9.5 Q2.** Anonymous-form (`mirror beam`) and shadow-cast. Is the
   sentinel peer-home dispatch scaffold or intended? Substrate-decl
   says scaffold.
4. **§8.1** MCP schema shape — Option A vs B. Mara recommendation:
   A this tick, B when `@mcp.serve` (task #386) lands.
5. **§4.2** `flag from_psychohistory: lens = <ref>` — the OID citation
   depends on `shards/fate/bounded.mirror` landing (§10 dep #3).
   Two-tick close: substrate-decl lands with a `<forward-promised>`
   annotation this tick; OID fills in when the shard lands.

Beyond that, nothing I can identify without further Alex input. The
composition operator is minted (§5). The stage is minted (§4). The
default is declared (§3). The species citations are grep-verified
(§1). The refusal is discharged (§6). The unified-metric close is
made explicit (§7). The MCP surface reflection is adjudicated (§8).
The migration path is recommended (§9).

**The reframe holds. The words are ~85% already in the substrate.
The one MINT is the composition operator's altitude specialization
(`mosaic(@mirror/lens/cli)`) plus the depth-2 stage
(`stage @mirror/lens/cli/peer/beam`).**

The Alex reframe IS: making the runtime match the substrate's
already-declared discipline. Substrate-decl says the default is the
full composition. Reed's incremental build was scaffolding. This
spec is the substrate-honest close.

---

*End Mara canonical spec. Substrate-decl lands on Pack ratification.
Rust runtime default-flip lands per §9 Option (c) with Alex
adjudication. `mosaic(@mirror/lens/cli)` is the composition altitude.
The default IS the full Rayleigh descent. Compiler and peer share
ONE metric.*

*— Mara, 2026-07-12. Thinking-in-public. Cite everything. Grep before
you mint. The substrate already had the word.*
