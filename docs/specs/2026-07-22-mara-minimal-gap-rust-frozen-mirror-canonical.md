# Minimal-gap map: rust/ frozen, mirror.spec canonical, @tools is the hook — canonical spec

**Status:** canonical spec.
**Author:** Mara.
**Date:** 2026-07-22.
**Substrate lineage this spec composes over** (does NOT re-derive):

- `docs/specs/2026-07-19-mirror-spec-is-the-fixpoint-liquid-is-the-runtime.md` (Mara 2026-07-19; §2.1 spec-native primary; §2.3 dispatch table; §5.2 4-file rust/ floor; §7 what-dies / what-remains).
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara 2026-07-15; §0.4 6-arc structure; §4.5 monotone descent invariant; §8 landing sequence).
- `docs/specs/2026-07-18-the-compiler-in-one-sentence.md` (Mara 2026-07-18; @tools family-root closure; §3.3 composition direction; §4.1 tools{} block).
- `shards/tool.mirror` + `shards/tool/{cargo,git,nix,go,docker,gitlab_ci}.mirror` (LANDED 2026-07-18; @tool family-root + 6 species).
- `shards/mirror/spec.mirror` (LANDED; @mirror/spec prism + `project`/`target`/`settle_on` grammar; §tools block forward-promised at :167-225).
- `shards/mirror/spec/property.mirror` (LANDED 2026-07-19; spec-body `property { verifies { … } domain @<T> samples <n> defer? <msg> }` grammar; `property_decl` typed carrier + 3 admissibility bilaterals).
- `rust/src/{main,phone,liquid,compile,matrix,void,collapse}.rs` (LANDED terminal FLOOR; 7-arm `dispatch_spec_property` cascade; 272/272 GREEN 2026-07-21).
- `docs/loop/CURRENT.md` §2026-07-21 SESSION LANDING SUMMARY (13-iter cascade; 144 new tests; phone.rs production-ready; dispatch cascade 7 arms deep).

**Pure-docs 📝 markdown-only bypass. SSH signing default. Commit as `Mara <mara@systemic.engineer>`.**

---

## §1 Alex 2026-07-22 direction anchor + operational framing

### §1.1 Verbatim direction

Alex 2026-07-22 in-transcript (direction that names this spec's brief):

> "bootstrap/ is dead. rust/ is alive. The @roomba bumps into bootstrap/ and integrates it through the mirror loop in the numerical flang space into the @kintsugi/mosaic. Hence reduces the rust surface."

> "freeze rust/, solve in mirror."

### §1.2 Operational framing

Two axes, one direction. The recognition inverts what "grows next":

- **What FROZE 2026-07-21.** phone.rs @io connection surface shipped
  with 82 property tests; matrix.rs FLANG floor complete (LAPACK/BLAS
  eigenvalues + envelope + phase_lock; 42 property tests); liquid.rs
  dispatch cascade 7 arms deep covering all 6 rows of Mara §2.3
  dispatch table minus health; compile.rs SAGA orchestration
  integrating both bilateral + spec-body property classes;
  fractal::{Crystal,Oid,Witnessed,Subject} content-addressed identity.
  **The rust/ altitude has surface-completeness for the M-tick
  sequence Mara 2026-07-19 §5.2 named.**

- **What has NOT yet grown.** `mirror.spec` still has ZERO
  `property { verifies { … } }` blocks. ZERO `tools { … }` blocks.
  ZERO `system @<name> { … }` blocks. The dogfood spec today declares
  targets + settle_on predicates by NAME (e.g. `binary.compiles`) but
  those names resolve to Rust-hardcoded checks, not to spec-declared
  verifiable executable specifications.

**The minimal gap is at the SPEC altitude, not the Rust altitude.**
The Rust runtime has surface-completeness. The spec has zero
consumers of that surface. The gap is: **mirror.spec authors zero
property blocks that the rust/ SAGA loop can dispatch.**

Alex 2026-07-22's naming reframes the arc: the @roomba integrates
bootstrap/ through the mirror-loop-in-flang-space into
@kintsugi/mosaic. This is the reverse-composition: **the Rust
surface reduces AS mirror.spec grows property blocks that make
dogfood spec-native rather than Rust-hardcoded**. The `@tools`
family-root IS how external Rust primitives (and cargo/git/nix
invocations) compose INTO mirror substrate at settlement altitude.

The remainder of this spec maps both surfaces empirically and names
the minimal shard-decl + @tools compositions + rust-plumbing edges
that close the gap.

---

## §2 Mirror surface enumeration (what IS declared)

### §2.1 Family-roots (shards/*.mirror at family altitude)

47 family-root .mirror shards at `shards/*.mirror`:

| Family root | Role | Species declared |
|---|---|---|
| `@io` | @io family-root; 24KB; process/fs/git/socket/bytes/network sub-species | LANDED |
| `@tool` | first-order tech tool identity/version/contract/provenance wrapper | LANDED; 6 species (cargo, git, nix, go, docker, gitlab_ci) |
| `@mirror` (via shards/mirror/*) | mirror-own project manifold vocabulary | 40+ species (spec, pack, lens/cli, mosaic, index, store, …) |
| `@kintsugi` | mending discipline family-root | ouroboros / roomba / mosaic / fracture / consent / surface / translate species |
| `@fractal` | content-addressed identity + Crystal + Oid substrate | mandelbrot / crystal / singularity species |
| `@liquid` | property-runtime carrier family | (thin; primary implementation at prismqueer::liquid) |
| `@peer` | Pack peer discipline (reflect/redirect/reframe three-tier) | reflect / redirect / reframe / persistence / registry / void / beam species |
| `@fate` | Fate distribution + tournament + resolve | (canonical spec + math LANDED; runtime dispatch shard-body altitude) |
| `@void` / `@paradox` / `@torus` / `@spectral` / `@bauchladen` / `@aikido` / `@song` / `@dance` / `@cyberpunk` / `@subject` / `@sheaf` / `@gestalt` / `@order` / `@time` / `@system` / `@beam` | domain family-roots supporting the substrate discipline | various |
| `@autopoietic` / `@butterfly` / `@cybernetic/*` | classifier + mutation + coherence species | 13 landed cybernetic properties (per CLAUDE.md) |

### §2.2 Species-body altitude — bilaterals declared (COUNT)

**Empirical grep** for `^\s*bilateral\s+` across `shards/**/*.mirror`
returns **54 shard files** carrying bilateral blocks. Rough count
of bilateral blocks: ~120 total across the substrate (kintsugi/roomba
alone has 11).

These bilaterals are the shard-body altitude property declarations
(per Mara 2026-07-19 §2.1 "shard-decl secondary"). They dispatch
via `liquid::dispatch_property` through the pillar cascade at
`prismqueer::liquid::pillar`.

### §2.3 Spec-body altitude — property/tools/system declarations (COUNT)

**Empirical grep** for `^\s*(property|tools|system)\s+` across
`mirror.spec` returns **ZERO matches**.

- **Zero** `property <name> { verifies { … } }` blocks.
- **Zero** `tools { cargo { version "…" } git { … } }` blocks.
- **Zero** `system @<X> { … }` blocks (per Mara Round-3
  `shards/mirror/spec/system.mirror` VSM species-decl; forward-promised
  Tick 2 dogfood).

The `settle_on` block at `mirror.spec:421-443` names 7 predicates
(`binary.compiles` / `binary.tests_pass` / `fmt.formats` / … /
`total_transparency.weight == 0`), but those are name-references
to Rust-hardcoded checks — they are NOT `property { verifies { … } }`
declarations that carry verifiable executable-specification bodies.

### §2.4 Grammar surface — sub-keywords per @mirror/spec

Per `shards/mirror/spec.mirror` (LANDED):

- `project(name) -> prism` — top-level (with `system` sibling
  forward-promised per Round-3 two-tick alias-shim).
- `source(path)`, `legacy(paths, shrinkage_contract)`, `target(name)`,
  `settle_on(predicates)`.
- Inside `target`: `name`, `altitude`, `emit`, `cli(body)`, `needs`,
  `check(action)`.
- `tools(body: tool_pins_block) -> tool_pins_decl` — **FORWARD-PROMISED**
  (not yet action-decl'd; sub-keywords `cargo`/`git`/`nix`/etc. not
  yet keyword-registered).
- `property <name> { verifies { … } domain @<T> samples <n> defer? <msg> }`
  — **SUBSTRATE-DECLARED** at `shards/mirror/spec/property.mirror`
  (Mara 2026-07-19), with `focus property` / `focus verifies` /
  `project domain` / `project samples` / `project defer` keyword
  bindings; typed carrier `property_decl(...)`. But NO CONSUMER in
  `mirror.spec` today.

### §2.5 Reserved species — declared-but-unwired count

- `shards/mirror/spec/property.mirror`: **1 species-decl unwired to
  dogfood** (`property_decl` typed carrier; 3 admissibility bilaterals;
  keyword bindings registered but no companion-source-registration
  landed in the reflective consumer at rust/ altitude — the
  bootstrap/src/grammar.rs registration path is DEAD per Alex
  2026-07-22 "bootstrap/ is dead", so this needs a rust/ altitude
  reflective registration).
- `shards/mirror/spec/system.mirror`: **1 species-decl unwired to
  dogfood** (Round-3 VSM `system @X { ... }` grammar; forward-promised
  Tick 2 dogfood migration).
- `shards/mirror/spec.mirror:167-225` `tools(body)` forward-promise:
  **1 action-decl unwired** (no companion `shards/mirror/tools.mirror`
  species-decl; no keyword binding `focus tools` in
  `shards/mirror/spec/keywords.mirror` yet).
- `@tool` family-root + 6 species (cargo/git/nix/go/docker/gitlab_ci):
  **7 species-decls unwired to dogfood consumers** (`@tool.exec` /
  `@tool.sign` / `@tool.verify` / `@tool.version_of` action bodies
  all `\`-obligation-blocked; NO rust/ altitude dispatch arm
  matches on @tool ref forms).
- `@kintsugi/ouroboros` (species-decl LANDED Mara-A 2026-07-15;
  576 LOC): **1 species-decl unwired** — the collapse cascade is
  documented (Arc-1 through Arc-6) but the substrate-decl'd actions
  (`collapse`, `verify_same_output`, `cutover`, `walk_and_collapse`)
  are `\`-obligation-blocked.

**Empirical tally:** ~10 spec-body/spec-adjacent species-decls
declared-but-unwired to dogfood, plus ~120 shard-body bilaterals of
which ~10-20 are directly load-bearing for the ouroboros closure
(the rest continue to work at shard-body altitude as designed).

---

## §3 Rust surface enumeration (what IS implemented)

Per `rust/src/*.rs` at `rust/Cargo.toml`-owned terminal FLOOR
(bootstrap/ is DEAD per Alex 2026-07-22; the enumeration ignores it).

### §3.1 Per-file altitude (Mara §5.2 5-file discipline)

| File | LOC | Altitude | Role |
|---|---|---|---|
| `main.rs` | ~1246 | supervisor + `@`-operator addressing | argv parse; verb dispatch (compile/roomba); `at_operator` @io/fs family dispatch (5 landed routes + @io/git.commit STUBBED for Subject-registry landing); observation-crystal pheromone deposit |
| `phone.rs` | ~1900 | @io connection surface (Loki matrix-phone-booth) | @io/fs (8 fns; walker with .git/target skip); @io/git (3 fns; tempdir-repo scaffold); @io/bytes stdio (2 fns + Read/Write generics); @io/socket (3 fns; Unix-only). Zero unsafe extern "C". 82 property tests |
| `liquid.rs` | ~2900 | property runtime | `PropertyDecl` (bilateral shape) + `SpecProperty` (spec-body shape) carriers; `extract_properties` + `extract_spec_properties` byte-scanners; `Verdict` enum (Pass/Fail/Defer); `dispatch_property` (bilateral dispatch through pillar cascade); `dispatch_spec_property` (7-arm cascade: defer / boolean / sentinel-containment / algedonic / viability / fold / bundle_commutator); `pillar` module (10 mirror-domain classifier witnesses) |
| `compile.rs` | ~800 | SAGA compilation loop | `Compilation` (crystals + discharges + escalation); `Escalation::{Continue,Escalate(oid),Halt(msg)}`; `compile_declarations` + `compile_from_source` SAGA loop; first-fail-pin invariant; both bilateral + spec-body classes flow through same chain |
| `matrix.rs` | ~1500 | sub-Turing FLANG numerical arm | LAPACK/BLAS delegations via prismqueer::ffi: `eigenvalues` (dsyev; Fiedler compute; 18 tests); `envelope` (dgesvd; Aumann convex hull; 12 tests); `phase_lock` (Kuramoto Euler; 12 tests; RK4 upgrade forward-promise per Alex 2026-07-20). Unsafe extern "C" boundary lives here only |
| `void.rs` | ~500 | @void species substrate at rust/ | void_settle + related primitives |
| `collapse.rs` | ~1000 | @kintsugi/fracture/bilateral_arm_redundant collapse dispatch | `BilateralDecl` corpus loader; `find_redundant_arms` byte-analysis; `apply_deletions`; consumed by `cmd_roomba` for arm-collapse dispatch |

Plus `rust/fractal/src/{lib,mandelbrot,crystal,singularity,subject}.rs`
carrying `Crystal<T>` + `Oid` + `Witnessed` + `Subject` +
`crystallize<T>` at the crate-adjacent identity substrate.

### §3.2 Rust surface by role

Grouped for the collapse-analysis:

- **Dispatch (spec/shard body → verdict).** `liquid::dispatch_spec_property`
  (7 arms); `liquid::dispatch_property` (via `pillar::dispatch` 10 witnesses);
  `main::at_operator` (5 @io/fs routes + @io/git.commit STUBBED). **STATE:**
  spec-body dispatch surface substantially complete for §2.3
  dispatch-table rows 1/2/5/6 (algedonic / viability / fold /
  bundle_commutator); row 3 (health/of_health) and row 6 general
  expression-tree parser forward-promised. NO `@tool` dispatch arm.
- **@io (byte-crossings).** `phone.rs` — all 4 families
  (fs/git/bytes/socket) production-ready per Alex 2026-07-21
  directive-discharge.
- **Matrix (sub-Turing numerical).** `matrix.rs` — eigenvalues +
  envelope + phase_lock via LAPACK; RK4 forward-promise; unsafe
  boundary contained here.
- **Compile-loop (SAGA).** `compile.rs` — `compile_from_source`
  extracts BOTH bilateral + spec-body class + runs each through
  same chain. `main::cmd_compile` invokes.
- **Fractal-carrier (identity).** `rust/fractal/*` — Crystal / Oid /
  Witnessed / Subject / crystallize; all consumed by compile.rs.
- **Singularity (Iso rung).** `rust/fractal/src/singularity.rs` +
  `rust/singularity/` crate scaffold — Landing D optics-hierarchy Iso.
- **Boot/argv dispatch.** `main.rs` — 11-verb hardcoded table (M0);
  reflective cli-block reader forward-promised.

### §3.3 Which Rust would need to GROW to close the gap?

**Two candidate growths, per Alex 2026-07-22 "solve in mirror" directive:**

- **Candidate A (BAD — grows Rust).** Add `at_operator` route
  `@tool/<X>.exec(args)` that dispatches through `phone.rs` process
  spawn. This grows Rust surface. Per `feedback_no_rust_extension_shortcut`
  + `feedback_detector_inadequacy_answer_is_never_rust` — this is
  the antipattern.

- **Candidate B (SUBSTRATE-HONEST — reuses phone.rs FLOOR).** Extend
  `at_operator` with ONE dispatch arm that pattern-matches
  `@tool/<X>.exec` and forwards to a `process_spawn` primitive
  ALREADY at phone.rs altitude (spawn+wait+capture stdout/stderr —
  the M4 shape phone.rs's `@io/git.git_commit_as` already exhibits
  via `std::process::Command`). This is ONE arm addition following
  the exact pattern of the 5 landed `@io/fs.*` arms. The FLOOR
  strictly REUSES; the dispatch arm is `[substrate-floor:@io-boundary]`
  legitimate.

**§5 below names Candidate B as the recommended minimal edge.**

---

## §4 @tools hook analysis: what @tools declares as its compositional surface

### §4.1 The compositional surface

Per `shards/tool.mirror` §3 (LANDED 2026-07-18):

```
@tool.exec(invocation: tool_invocation) -> tool { \ }
@tool.sign(invocation: tool_invocation, signer: peer) -> tool { \ }
@tool.verify(invocation: tool_invocation) -> verdict { \ }
@tool.version_of(id: tool_id) -> ref { \ }
```

Where `tool_invocation = { tool_id, args, invoker, signature,
version, working_dir, env }` and `tool_id` is the closed variant
`| cargo | git | nix | ffmpeg | kubectl | docker | npm | pip |
sqlite | curl | jq | go | gitlab_ci | opaque(str)`.

### §4.2 Composition direction (§3.3 of closure spec)

```
@tool(X, args)   ─dispatches─▶   @io/X.exec   ─crosses─▶   syscall
      │                              │                        │
  inference                     mechanism                actual bytes
  altitude                      altitude                 (outside world)
      │                              │
      └─ compiler reasons here ──────┘
                                     │
                     realisation discharges here
```

The compiler NEVER reasons directly about @io/X.exec bytes; it
reasons about @tool(X, args) invocations. This is the load-bearing
altitude split for the freezing operation: **@tool is the IDENTITY /
VERSION / CONTRACT / PROVENANCE altitude; @io is the MECHANISM
altitude; the compiler reasons at @tool and only lets @io discharge
when dispatch fires**.

### §4.3 Where @tool composes over Rust primitives (the HOOK)

Empirically today:

- **@tool.exec discharge → phone.rs::spawn_process.** This primitive
  DOES NOT EXIST YET at phone.rs. Its NEIGHBORS do: `phone::git_add`
  and `phone::git_commit_as` (git subprocess dispatch); `phone::
  list_dir_recursive` (fs walker); `phone::read_stdin_frame` (bytes
  IO). The FLOOR primitive `spawn_process(cmd, args, cwd, env) ->
  (exit_code, stdout, stderr)` is one @io/process primitive short.
  It is a legitimate FLOOR extension per §3.3 Candidate B —
  substrate-decl'd at `shards/io.mirror` as `@io/process` (LANDED
  ~2026-07-15); the Rust FLOOR primitive is the discharge target.

- **@tool.exec dispatch surface → main::at_operator.** The `at_operator`
  currently matches `@io/fs.list_dir` / `@io/fs.read` / `@io/fs.write`
  / `@io/fs.append` / `@io/fs.mkdir_p` (5 routes) plus a STUBBED
  `@io/git.commit`. Adding ONE arm matching `@tool/<X>.exec` that
  looks up X in a small dispatch table (cargo/git/nix/...) and
  invokes `phone::spawn_process` is the composition edge that
  makes @tool executable.

- **@tool consumer at mirror.spec altitude → `tools { }` block.** The
  forward-promised `tools(body: tool_pins_block) -> tool_pins_decl`
  action at `shards/mirror/spec.mirror:167-225` names the surface;
  its keyword binding + companion species-decl + rust-consumer
  extraction (parallel to `extract_spec_properties`) is the mirror-
  altitude edge.

**These three edges — one @io FLOOR primitive, one `at_operator`
dispatch arm, one spec-body extractor — are the entire hook.**

### §4.4 What @tools does NOT need to grow

- NOT another Rust extension per tool species. Every `@tool/<X>`
  species dispatches through the SAME `@tool.exec` altitude which
  discharges through the SAME `@io/process.spawn` FLOOR primitive.
  Per-species specialization lives at the shard-body altitude
  (`@tool/cargo.exec` composes cargo's subcommand grammar on top;
  `@tool/git.exec` composes git's), NOT at rust/ altitude.

- NOT a per-species Rust dispatch arm. The `at_operator` needs ONE
  arm matching `@tool/*` regardless of species. The species-level
  routing lives in the shard body (which today is `\`-obligation-
  blocked because bodies don't yet dispatch, but which for the
  minimal-gap MVP the dispatch arm can shortcut by pattern-matching
  the species tag from the action-ref string). Post-@kintsugi/
  ouroboros landing, the shard-body dispatcher (evaluator FLOOR)
  handles species routing.

- NOT a Subject registry. The `@tool.sign` + `@tool.verify` actions
  compose with the @trust family-root (forward-promised); the minimal
  gap does NOT include trust-chain integration. Empty signatures
  are admissible at MVP altitude.

---

## §5 THE MINIMAL GAP

The smallest set of {shard-decl mints, @tools compositions, rust
dispatch-arm plumbing extensions justified-by-substrate-pull} that
closes the gap between "rust as currently is" and "mirror.spec fully
specifies a binary that rust/ derives-from + verifies-against".

### §5.1 Composition (5 edges; ordered by dependency)

**Edge 1 (SHARD-DECL MINT).** `shards/mirror/tools.mirror` species-decl
mint at `@mirror` family altitude. Companion to `@mirror/spec` +
`@mirror/pack` + `@mirror/garden`. Substrate-decl'd carriers:

- `tool_pins_block = list(labeled(tool_id, tool_pin_body))` (per
  §4.1 shards/mirror/spec.mirror forward-promise line 209-213).
- `tool_pin_body = { version: ref, args: option(ref), env: option(ref),
  working_dir: option(ref) }`.
- `tool_pins_admissible(block: tool_pins_block) -> verdict` composed
  bilateral — every pin's tool_id resolves via `@tool.version_of`;
  no duplicate tool_id; every version_ref resolves via nix-pin or
  system-version.
- action-decl `tools(body: tool_pins_block) -> tool_pins_decl { \ }`.

**Estimated size:** ~150 LOC .mirror (companion to spec.mirror's
`tools` forward-promise; direct parallel to `shards/mirror/pack.mirror`
+ `shards/mirror/garden.mirror` structural precedents).

**Edge 2 (KEYWORD BINDING).** Add to `shards/mirror/spec/keywords.mirror`:

```
focus tools
```

(recursively-scanned brace block; parallel to `focus cli` and
`settle settle_on`). Single-line addition per canonical §3.3 Step 2
precedent.

**Edge 3 (RUST EXTRACTOR).** Extend `rust/src/liquid.rs` with
`extract_tools_pins(source: &str) -> Vec<ToolPin>` byte-scanner.
Mirror the pattern of `extract_spec_properties`: scan for `tools {`
opener; consume balanced brace block; for each inner
`<tool_id> { version "<ver>" }` decl, emit one `ToolPin` carrier.

**Estimated size:** ~80 LOC Rust extending liquid.rs (same pattern
as landed `extract_spec_properties` at liquid.rs:225-367 which is
~140 LOC; tools block is simpler grammar so shorter).

**Edge 4 (RUST DISPATCH ARM).** Extend `rust/src/main.rs::at_operator`
with ONE arm matching `@tool/<X>.exec`:

```rust
_ if action_ref.starts_with("@tool/") && action_ref.ends_with(".exec") => {
    // Extract species tag between @tool/ and .exec
    let species = &action_ref[6..action_ref.len() - 5];
    // Dispatch to phone.rs process spawn (@io/process FLOOR primitive)
    let (exit_code, stdout, stderr) = phone::spawn_process(species, args, None, None)?;
    // Return exit_code as string; caller extracts stdout via
    // subsequent @io/bytes.read call if needed (bilateral degenerate
    // discharge shape at MVP altitude).
    ...
}
```

**Estimated size:** ~40 LOC Rust (one match arm + error surfacing).

**Edge 5 (RUST @io/process FLOOR PRIMITIVE).** Extend `rust/src/phone.rs`
with `spawn_process(command: &str, args: &[String], cwd: Option<&Path>,
env: Option<&[(String, String)]>) -> io::Result<ProcessResult>` where
`ProcessResult = { exit_code, stdout, stderr, duration }`. Direct
composition over `std::process::Command` (safe-Rust; no unsafe extern);
mirrors the pattern of existing `phone::git_commit_as` at phone.rs
altitude.

**Estimated size:** ~80 LOC Rust + ~15 property tests. **Legitimate
`[substrate-floor:@io-boundary]` FLOOR work** — @io/process is
substrate-decl'd at `shards/io.mirror`; the primitive is one @io
family that phone.rs has NOT yet materialized (fs/git/bytes/socket
are landed; process is the fifth). Per Alex 2026-07-16 8th-repetition
`feedback_detector_inadequacy_answer_is_never_rust`: this is NOT a
detector-extending Rust growth — this is a substrate-decl'd @io
family whose Rust FLOOR body was M-planned but not yet landed. The
FLOOR strictly shrinks after this landing because it enables 7+
@tool species to dispatch via ONE mechanism.

### §5.2 What this composition unlocks (mirror.spec dogfood extensions)

Post-5-edge landing, mirror.spec MAY declare:

```
project mirror.spec {
  ...existing source / legacy / pack / garden / target / settle_on...

  tools {
    cargo { version "1.80" }
    git   { version "2.45" }
    nix   { version "2.24" }
  }

  property binary_compiles_via_pinned_cargo {
    verifies { @tool/cargo.exec([check, --workspace]).exit_code == 0 }
    domain @Spec
    samples 1
  }

  property tests_pass_via_pinned_cargo {
    verifies { @tool/cargo.exec([test, --workspace]).exit_code == 0 }
    domain @Spec
    samples 1
  }
}
```

The `settle_on` predicates at mirror.spec:421-443 can NOW re-express
as spec-native `property { verifies { … } }` declarations:

- `binary.compiles` → `property binary_compiles_via_pinned_cargo`.
- `binary.tests_pass` → `property tests_pass_via_pinned_cargo`.
- `fmt.formats` → `property fmt_via_pinned_cargo { verifies {
  @tool/cargo.exec([fmt, --check]).exit_code == 0 } … }`.
- `lint.lints` → property with `@tool/cargo.exec([clippy, …])`.
- `bench.compiles` → property with `@tool/cargo.exec([bench, --no-run])`.

**Each `settle_on` predicate that migrates to a spec-native property
retires one line of Rust-hardcoded check logic.** This is the
`@kintsugi/mosaic` reduction Alex 2026-07-22 named: rust surface
shrinks AS mirror.spec grows verifiable executable specifications.

### §5.3 Minimum-gap size

**Total gap-closure size:**

- .mirror shard-decl mints: ~150 LOC (Edge 1) + 1 line (Edge 2) =
  ~151 LOC.
- Rust code: ~80 LOC extractor (Edge 3) + ~40 LOC dispatch arm
  (Edge 4) + ~80 LOC @io/process FLOOR (Edge 5) + ~15 property
  tests = ~215 LOC + tests.
- No Rust deletions in this landing (the RETIREMENTS come as
  settle_on predicates migrate per §6 trajectory).

**Total ~370 LOC gap. The rust growth is FLOOR-legitimate (Edge 5
= substrate-decl'd @io/process family FLOOR primitive; Edges 3+4
compose the existing SAGA + at_operator surfaces without extending
their contracts).**

### §5.4 The claim

Post-5-edge landing, mirror.spec IS the fixpoint: every check the
compiler must perform is either
(a) a spec-body `property { verifies { … } }` declaration that
    dispatches through `compile_from_source` → `dispatch_spec_property`
    → verdict, OR
(b) a shard-body bilateral that dispatches through
    `dispatch_property` → pillar cascade → verdict.

The Rust runtime is a fixed 5-file terminal FLOOR (main + phone +
liquid + compile + matrix + collapse + void — with @io/process the
sixth phone.rs family). No new Rust files. No new dispatch arms per
tool species. No new pillar predicates per property (all new
properties compose over the 7 landed arms + shard-body bilateral
pathway).

**Rust FROZEN at first empirical firing of `mirror kintsugi
./mirror.spec` where at least one spec-native property that names
`@tool/cargo.exec` dispatches through the composition edge and
returns a Verdict that the SAGA chain crystallizes.**

---

## §6 Concrete collapse trajectory (5 ticks; empirically close the gap)

Ordered by dependency. Each tick = one shard body lands + one
composition edge OR one dispatch arm gets substrate-decl'd. Every
tick is verifiable with a single RED→GREEN test cycle.

### §6.1 Tick 1 — @io/process FLOOR primitive at phone.rs (½ day)

**Landing.** `rust/src/phone.rs::spawn_process(cmd, args, cwd, env)
-> io::Result<ProcessResult>` + `ProcessResult` struct + ~15 property
tests covering: (a) exit-code round-trip, (b) stdout capture, (c)
stderr capture, (d) cwd-relative behavior, (e) env-var override, (f)
timeout / duration measurement, (g) missing-command error surface.

**Marker.** `[substrate-floor:@io-boundary]` + Signed-off-by: Seam
per §7 A9 Arc-1 discipline. `@io/process` is substrate-decl'd at
`shards/io.mirror` since 2026-07-15; this Rust landing is the FLOOR
body for that already-existing substrate-decl.

**RED test.** `rust/tests/red_phone_spawn_process_exists.rs` asserts
the function signature + at least one property-test scenario firing
end-to-end (e.g., spawn `echo hello` returns exit_code=0 + stdout
containing "hello"). Currently RED (function doesn't exist).

**Retirement condition for post-tick.** None; this is FLOOR growth
in a family that was M-planned but empirically unshipped. Retirement
happens in later ticks via `@kintsugi/ouroboros` per-species collapse.

**Empirical firing.** `cargo test --workspace test_phone_spawn_process`
passes; the FLOOR primitive is dispatchable.

### §6.2 Tick 2 — `@tool/<X>.exec` dispatch arm at at_operator (½ day)

**Landing.** `rust/src/main.rs::at_operator` extended with ONE arm
matching `@tool/*.exec` action-refs. The arm extracts the species
tag (cargo/git/nix/go/docker/gitlab_ci/etc.), maps to the process
name, invokes `phone::spawn_process`, wraps the ProcessResult as a
byte-string carrier for the compile.rs SAGA chain.

**RED test.** `rust/tests/red_at_operator_tool_cargo_exec.rs` asserts
`at_operator("@tool/cargo.exec", ["--version"])` returns Ok with a
string containing "cargo" (system cargo version output). Currently
RED (arm doesn't exist).

**Composition edge.** The arm delegates SPECIES ROUTING to a small
dispatch table (cargo/git/nix are the FLOOR triple per Alex
2026-07-18 "lift the rust toolchain INTO mirror"). Other species
(go, docker, gitlab_ci, etc.) resolve via `opaque(str)` escape at
same arm.

**Empirical firing.** `cargo test test_at_operator_tool_cargo_exec`
passes; `@tool/cargo.exec` is dispatchable end-to-end.

### §6.3 Tick 3 — shards/mirror/tools.mirror species-decl mint (1 day)

**Landing.** Mara authors `shards/mirror/tools.mirror` per §5.1
Edge 1: 4 carriers (`tool_pins_block`, `tool_pin_body`,
`tool_pins_admissible` bilateral, `tools(body)` action-decl). Direct
parallel to `shards/mirror/pack.mirror` + `shards/mirror/garden.mirror`
structural precedent.

**Keyword binding.** Add `focus tools` to
`shards/mirror/spec/keywords.mirror` (single line).

**Companion registration in rust/.** Extend rust/src/liquid.rs to
handle the `tools {}` block during `extract_spec_properties`-adjacent
extraction (the reflective consumer at rust/ altitude that replaces
the bootstrap/src/grammar.rs registration path per Alex 2026-07-22
"bootstrap/ is dead").

**RED test.** `rust/tests/red_extract_tools_pins.rs` asserts that
for a fixture spec containing `tools { cargo { version "1.80" } }`,
`extract_tools_pins(source)` returns one ToolPin with tool_id=cargo,
version="1.80". Currently RED (function doesn't exist).

**Retirement condition.** No prior code retires; this ADDS the
tool-pins extraction to compile.rs's SAGA scope.

**Empirical firing.** `cargo test test_extract_tools_pins` passes;
mirror.spec MAY carry `tools {}` block.

### §6.4 Tick 4 — first `property { verifies { @tool/cargo.exec(...) } }` in mirror.spec (½ day)

**Landing.** Author into `mirror.spec` (dogfood):

```
tools {
  cargo { version "1.80" }
  git   { version "2.45" }
  nix   { version "2.24" }
}

property binary_compiles_via_pinned_cargo {
  verifies { @tool/cargo.exec([check, --workspace]).exit_code == 0 }
  domain @Spec
  samples 1
}
```

**Rust dispatch arm extension.** Extend `dispatch_spec_property`
with an 8th arm matching the verifies-shape
`@tool/<X>.exec([...]).exit_code == 0`. The arm invokes `at_operator`
(from Tick 2) with the extracted args, checks exit_code, returns
Verdict::Pass / Fail. This is ~40 LOC extension in liquid.rs
following the exact pattern of the 7 landed arms.

**RED test.** `rust/tests/red_mirror_spec_pinned_cargo_property.rs`
asserts that compiling mirror.spec through `compile_from_source`
produces at least one crystal whose discharge Verdict==Pass and
whose property_name matches `binary_compiles_via_pinned_cargo`.
Currently RED (dispatch arm doesn't exist).

**Empirical firing.** `mirror compile mirror.spec` (verb wired at
main.rs:1111-1178) reports the SAGA chain with the pinned-cargo
property crystallized as Pass. **This is the second-witness of the
recognition: mirror.spec authors an executable specification that
the Rust runtime verifies WITHOUT the check logic being Rust-
hardcoded.**

### §6.5 Tick 5 — `settle_on` migration cascade (1 day)

**Landing.** Migrate the 7 `settle_on` predicates at mirror.spec:421-443
one-by-one to spec-native `property { verifies { … } }` declarations,
each using the `@tool/<X>.exec` composition edge:

1. `binary.compiles` → `property binary_compiles_via_pinned_cargo`
   (already landed Tick 4).
2. `binary.tests_pass` → `property tests_pass_via_pinned_cargo`
   with `verifies { @tool/cargo.exec([test, --workspace]).exit_code == 0 }`.
3. `fmt.formats` → `property fmt_via_pinned_cargo` with
   `verifies { @tool/cargo.exec([fmt, --check]).exit_code == 0 }`.
4. `lint.lints` → property with `@tool/cargo.exec([clippy, …])`.
5. `tests.tests_pass` → duplicate of (2); rationalize.
6. `bench.compiles` → property with `@tool/cargo.exec([bench, --no-run])`.
7. `total_transparency.weight == 0` → spec-native fold over the
   above; per Mara §2.3 dispatch table row 5 `fold` arm (LANDED).

**Rust retirement.** For each migrated `settle_on` predicate, delete
the corresponding Rust-hardcoded check at bootstrap/lib.rs
`cmd_kintsugi_spec`. **BUT bootstrap/ IS DEAD** per Alex 2026-07-22;
this means the settle_on-check discharge NEVER LIVED at rust/
altitude and the migration is pure ADDITION at spec altitude —
which is EXACTLY the ouroboros claim: bootstrap/ dies BECAUSE
mirror.spec grows the check surface substrate-natively.

**Empirical firing.** `mirror compile mirror.spec` reports SAGA
chain with 7 crystals, all Pass; the dogfood spec is fully
substrate-native. **Rust FROZEN empirically demonstrable at this
tick: no new Rust checks are needed for any settle_on predicate;
every future check is a spec-native property block dispatching
through the fixed cascade.**

### §6.6 Optional Tick 6 — @kintsugi/mosaic integration (2 days)

Named for completeness; NOT part of the minimal gap. Wires the
mirror.spec-driven SAGA chain into `@kintsugi/mosaic.settle` per
`shards/kintsugi/mosaic.mirror` LANDED substrate-decl. This closes
the loop where the settlement CRYSTAL becomes the "current settled
state" per Alex 2026-07-20 Crystal<T>/Mandelbrot<T> recognition,
and where the ouroboros arc's cutover discipline (§5.5 of ouroboros
spec: `cutover(target)` composed step: `collapse` +
`verify_same_output` + `cutover`) becomes the mosaic-driven mend
per settle-tick.

---

## §7 Composition edges into currently-in-flight arcs

### §7.1 With Reed's 2026-07-21 phone.rs ship + dispatch cascade

**Composition edge.** Reed shipped phone.rs to production-ready with
82 property tests + 4 @io families landed (fs/git/bytes/socket) per
task #303 completion. **Tick 1 above (@io/process FLOOR primitive)
COMPOSES DIRECTLY on that landing pattern** — the phone.rs
altitude discipline is established; adding a 5th family (process)
follows the exact pattern of git/bytes/socket ship (tempdir-repo
scaffold pattern for isolation; Read/Write-generic helpers for
in-memory testability; property tests for state-space coverage).

Reed's dispatch cascade landed 7 arms in `dispatch_spec_property`
covering all 6 rows of Mara §2.3 dispatch table minus health.
**Tick 4 above (property + @tool/cargo.exec) COMPOSES DIRECTLY as
the 8th arm** — same pattern (byte-scan verifies-source shape;
extract args; invoke primitive; wrap PropertyVerdict → Verdict);
the arm's UPSTREAM dispatch surface (at_operator + phone.rs
process) is what Ticks 2+1 build; the arm itself is one match arm
extension in liquid.rs.

### §7.2 With Taut's parallel COORD scout (LANDED 2026-07-22 `3787770`)

Taut's scout landed as commit `3787770` immediately before this
Mara authorship. Substantial CONVERGENCE + minor complementary
disagreement noted for Alex adjudication.

**Convergence (both peers agree):**

- **`at_operator` extension is the correct next Rust motion.** Taut's
  COORD-4 (§5 first-cut recommendation) = @io/git.commit dispatch
  arm wire-up at `main.rs:947`; Mara's Tick 2 (§6.2) = @tool/<X>.exec
  dispatch arm at the same at_operator surface. **Both peers name
  the same file + same function + same pattern (one new arm following
  the 5 landed @io/fs.* precedents).** COORD-4 and Tick 2 are
  compatible + composable — both land at the same at_operator surface;
  they extend it with different action-refs.

- **rust/ is at surface-completeness for the FLOOR discipline.**
  Taut §4(a) enumerates 7 rust constructs as "frozen dispatch-plumbing
  — belongs at rust/ altitude forever" (phone.rs @io families,
  matrix.rs LAPACK/BLAS, main.rs sha256/argv). Mara §3 concurs:
  ALL current rust/*.rs is FLOOR-legitimate at 2026-07-21 landing
  state.

- **bootstrap/ DEAD discipline both peers honor.** Both scouts read
  ZERO bootstrap/ bytes.

**Complementary orderings (peers name different first-tick):**

- **Taut lean:** COORD-4 first (@io/git.commit arm) — cites -15 LOC
  net delta + first full @-operator arm precedent + unblocks COORD-1
  (bilateral arm-collapse dispatch composes over @io/git.commit at
  reflective corpus altitude).

- **Mara lean:** Tick 1 first (@io/process at phone.rs) — cites
  @tool-family hook admissibility + composition edge unlocks 7+
  species (cargo/git/nix/go/docker/gitlab_ci) via ONE FLOOR primitive
  + enables dogfood spec-native settle_on migration cascade at Tick 5
  (7 settlement predicates → 7 spec-native property blocks).

**Composition resolution.** The two orderings are NOT in conflict —
they compose sequentially. Taut's COORD-4 (@io/git.commit at_operator
arm) can land BEFORE Mara's Tick 1 (@io/process phone.rs primitive)
because COORD-4 composes over the already-landed `phone::git_commit_as`
without needing new phone.rs primitives. Recommended combined
ordering:

1. **Taut COORD-4** — @io/git.commit at_operator arm (½ day).
   Establishes the first-full-@-operator-arm precedent + retires
   the STUBBED route at main.rs:955-990.
2. **Mara Tick 1** — @io/process spawn_process at phone.rs (½ day).
   FLOOR primitive discharge; unblocks @tool.exec chain.
3. **Mara Tick 2** — @tool/<X>.exec at_operator arm (½ day).
   Follows COORD-4's precedent + composes over Tick 1's primitive.
4. **Mara Tick 3** — shards/mirror/tools.mirror species-decl mint +
   extract_tools_pins (1 day).
5. **Mara Tick 4** — first property { verifies { @tool/cargo.exec … } }
   in mirror.spec + 8th dispatch_spec_property arm (½ day).
6. **Mara Tick 5** — settle_on migration cascade (1 day).

**Combined arc:** ~4 days sequential; every tick unlocks the next.
Taut's COORD-1 (bilateral corpus at_operator dispatch), COORD-2
(extract_properties/extract_bilaterals unification), COORD-3
(SignatureBeat → @spectral/signature retirement), COORD-5+ compose
AFTER the combined arc as follow-up ouroboros ticks that further
shrink rust/ surface.

**Divergence flagged for Alex adjudication (§9.8 addition):**
Taut §4(b) hand-off list names 6 missing shard-decls (Compilation,
PropertyDischarge, @mirror/compile action, dispatch_of_spec_property
action, Iso/Lens/Prism/Traversal type-decls, void_basis_axis type).
Mara's minimal-gap map does NOT include these (per §8 Michelangelo
NOT taken — these are follow-up mints, not gap-closure blockers).
Alex: confirm the deferral is correct, OR name any of these as
in-scope for the current arc.

### §7.3 With @kintsugi/ouroboros 6-arc retirement plan

**Composition edge.** The @kintsugi/ouroboros arc plan (§0.4 of
ouroboros spec) named Arc-1 (Evaluator FLOOR at bootstrap/), Arc-2
(per-file hardcoded collapses of Reed's 5 gift-arc Rust extensions),
Arc-3 (cross-file tournament-ordered ~25 files), Arc-4 (cross-@code/X
scale-out), Arc-5 (StageFreight × downstream CI), Arc-6 (terminal
recognition ratification).

**Alex 2026-07-22 direction supersedes Arc-1 at bootstrap/ altitude.**
Since bootstrap/ is DEAD, Arc-1's "evaluator FLOOR at bootstrap/"
retirement plan becomes: Arc-1 IS the rust/ landing state at
2026-07-21 (5-file terminal FLOOR + dispatch cascade + phone.rs @io
completeness). Arc-2's per-file collapses shift from
bootstrap/src/*.rs to the settle_on migration cascade at §6.5 above.
Arc-3+ shift from "collapse ~25 BUSINESS_LOGIC Rust files" to
"migrate ~all mirror.spec settle_on predicates + shard-body pillar
predicates to spec-native property { verifies { … } } declarations".

**The ouroboros closes at mirror.spec altitude, not at bootstrap/
altitude.** Per §5.4 above: Rust FROZEN when every future check is
a spec-native property block. The bootstrap/ retirement (Alex
2026-07-22 declaration) makes this trivially achievable — there is
nothing at bootstrap/ altitude the ouroboros needs to walk any more.

### §7.4 With Mara J-space + Anna Wolf 2012 landings (2026-07-20)

Composition-neutral. The J-space alignment substrate + Anna Wolf
2012 observation substrate + Anthropic 2026-07-07 target substrate
composition landed at Mara agent `a0a29b33550b88bc0` (per CURRENT.md
§5). This spec composes over the same rust/ FLOOR (matrix.rs FLANG
integration; phone.rs @io) but at a DIFFERENT altitude — mirror.spec
declaration surface. No conflict; the two arcs are independent
compositions over the same shared FLOOR.

---

## §8 Michelangelo-marble edges NOT taken (what NOT to build)

Discipline: name what's OUT of the minimal gap to prevent scope creep.

- **NOT a @trust family-root landing.** `@tool.sign` + `@tool.verify`
  compose with @trust (forward-promised). Signatures are empty
  strings at MVP altitude; the composition edge to @trust is
  independent of this gap-closure. **Explicitly not included in §5-§6.**

- **NOT a Subject-registry landing.** `main::at_operator`'s
  `@io/git.commit` route is STUBBED pending Mara `@peer/registry`
  species-decl mint + @trust family-root landing. This gap-closure
  does NOT touch that stub. Peer identity resolution stays
  forward-promised.

- **NOT the health/of_health dispatch table row.** Mara §2.3 dispatch
  table row 3 (`health_of(<state>) within envelope` → `pillar::of_health`)
  is one of 6 rows and NOT part of the minimal gap. It composes at
  a future tick when @fate feature-flag integration lands; the
  current 7-arm cascade is complete WITHOUT it.

- **NOT the general-expression-tree parser.** Mara §2.3 row 6
  `general expression tree → pillar::forall(samples, |t| ⟦expr⟧(t))`
  requires an expression-tree parser per Mara §3.2 Rondon-Kawaguchi-
  Jhala 2008 decidability grounding. The current `bundle_commutator`
  arm at liquid.rs:835 handles concrete-instance forms; parametric
  parser is forward-promised iter 14+.

- **NOT a RK4 upgrade of phase_lock.** Mara adjudication-pending
  per CURRENT.md §6 (phase_lock ships Euler; needs Milstein-Tret'yakov
  weak-4 SDE-RK4 per Anna Wolf thesis §B.2). This is a matrix.rs
  numerical-accuracy concern; independent of the @tools gap-closure
  and NOT part of §6 trajectory.

- **NOT bootstrap/ retirement code moves.** Alex 2026-07-22 declared
  bootstrap/ dead. This spec's ticks add to rust/ (5-file FLOOR +
  new @io/process family + new dispatch arms) and to shards/ + to
  mirror.spec (dogfood extension). No moves FROM bootstrap/. Any
  such moves would violate the "bootstrap is DEAD do not propose
  bootstrap altitude solutions" Mara-memory (Alex feedback
  `feedback_bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions.md`).

- **NOT per-tool-species Rust dispatch arms.** The @tool family
  ships 6 species today (cargo/git/nix/go/docker/gitlab_ci) + closed
  variant admits 13 named + `opaque(str)` escape. Per §4.4 above:
  ONE at_operator arm handles ALL species via string-pattern-match;
  per-species specialization lives at shard-body altitude. Adding
  per-species Rust arms would violate `feedback_no_rust_extension_shortcut`.

- **NOT a BEAM reintroduction.** Mara J-space arc named BEAM as a
  Q3 candidate. This spec does NOT touch that; BEAM lands as its
  own arc independent of the @tools gap-closure.

- **NOT MCP graph-native lift.** Named as candidate direction for
  Reed post-J-space landing. Independent of @tools gap-closure.

- **NOT a `mirror kintsugi` verb landing.** The 11-verb hardcoded
  table at main.rs:VERBS advertises `kintsugi` but its dispatch
  lands at M3+ (currently returns exit 2 with substrate-honest
  message). The kintsugi verb wiring is the NATURAL consumer of
  Tick 5 above (settle_on migration cascade completion) but is NOT
  part of the minimal gap — post-Tick 5, `mirror compile mirror.spec`
  IS the dispatchable form; `mirror kintsugi mirror.spec` retirement
  of the M3+ forward-promise is a follow-up ergonomic tick.

---

## §9 Alex-adjudication Q's with Mara-lean recommendations

### §9.1 Q1 — @io/process at phone.rs is FLOOR-legitimate?

**Question.** Per Alex 2026-07-16 8th-repetition
`feedback_detector_inadequacy_answer_is_never_rust` + Alex 2026-07-14
`feedback_no_rust_extension_shortcut`: does the Tick 1 @io/process
FLOOR primitive count as (a) substrate-honest FLOOR extension (@io
family M-planned but empirically unshipped), or (b) yet another
Rust growth answering a detector inadequacy?

**Mara lean.** **(a) substrate-honest FLOOR extension.** @io/process
is substrate-decl'd at `shards/io.mirror` since 2026-07-15 (`@io/
process` sub-species, per :215 "subprocess execution surface. Lifts
the floor's `exec` from `boot/std/io.mirror`"). The primitive is
one @io family that phone.rs's ship arc M-planned but did not
land (fs/git/bytes/socket landed; process is the fifth). Landing
it in phone.rs is FLOOR realization of an already-existing
substrate-decl, NOT a detector-extending growth. Marker: `[substrate-floor:@io-boundary]` + `Signed-off-by: Seam`.

**Adversarial check for the lean.** If Q1 answers (b), the
alternative is composing `spawn_process` as a shard-body over the
existing `@io/fs.write` + `@io/bytes.read` + syscall FLOOR
primitives. Empirically this is NOT admissible: subprocess exec at
POSIX altitude is one syscall (execve) that has no decomposition
into fs+bytes+stream — it IS its own FLOOR primitive. The @io
family that dispatches this syscall MUST have a Rust body. This
grounds the lean in operating-system reality.

### §9.2 Q2 — dogfood spec migration ordering: single-property Tick 4 or full 7-predicate Tick 5?

**Question.** Should Tick 4 land only ONE `property { verifies {
@tool/cargo.exec … } }` block (empirical proof-of-shape) OR the
full 7-predicate settle_on migration?

**Mara lean.** **Single-property Tick 4 + separate 7-predicate Tick 5.**
Two-tick discipline mirrors Reed's per-property RED-retirement
protocol (Mara 2026-07-19 §2.6 answer): each property migrates
independently after its spec-declared verdict bit-matches the
current Rust-hardcoded verdict. Tick 4 proves ONE property firing
end-to-end; Tick 5 cascades the remaining 6 in one arc with
per-predicate RED→GREEN verification. **Tick 4 = second-witness of
the recognition; Tick 5 = production cascade.**

### §9.3 Q3 — reflective grammar registration lives at rust/ altitude (bootstrap DEAD)?

**Question.** The `shards/mirror/spec/property.mirror` keyword
registration was planned per Mara 2026-07-19 §3.3 Step 2 as
"Reed post-landing single-line diff at
`bootstrap/src/grammar.rs::companion_keyword_sources`." Since
bootstrap/ is DEAD per Alex 2026-07-22, WHERE does the reflective
grammar registration live?

**Mara lean.** **Rust altitude at liquid.rs (or a new but-thin
`rust/src/grammar.rs` composed by main.rs).** The
`extract_spec_properties` byte-scanner in liquid.rs already handles
the property grammar without needing keyword-registration
infrastructure (it's byte-scan, not tokenizer-based). Similarly
`extract_tools_pins` (Tick 3) is byte-scan; no companion-keyword-
sources at rust/ altitude needed. **The bootstrap grammar
registration was needed for bootstrap's parser; rust/ altitude uses
direct byte-scanning consistent with the current landed pattern.**

Alternative if Alex prefers grammar-based: mint `rust/src/grammar.rs`
as a 6th file. Mara lean is AGAINST — the 5-file discipline is
load-bearing (per Mara §5.2 4-file rust/ floor + §5.2 five-file
extension for compile.rs) and byte-scanning suffices for property +
tools grammar shape.

### §9.4 Q4 — @tool.exec species routing: one at_operator arm or a species-dispatch table?

**Question.** Tick 2 at_operator arm handles `@tool/<X>.exec` via
`_ if action_ref.starts_with("@tool/") && action_ref.ends_with(".exec")`.
Should it (a) delegate to a species-dispatch table (mapping cargo/git/
nix/go/docker/gitlab_ci species tags to concrete process names), or
(b) treat the species tag as literally the process name?

**Mara lean.** **(a) minimal species-dispatch table.** For MVP,
map: cargo→"cargo" / git→"git" / nix→"nix" / go→"go" /
docker→"docker" / gitlab_ci→"gitlab-runner" (or similar). This is
~10 LOC. Escape via opaque(str) resolves to the string as literal
process name. The species-dispatch table lives in the arm; per-species
grammar refinement (cargo subcommands, git subcommands, nix
subcommands) lives in the shard body per §4.4 discipline.

### §9.5 Q5 — settle_on predicate migration retains backward-compat with current byte-check dogfood?

**Question.** Migrating `binary.compiles` → `property
binary_compiles_via_pinned_cargo` changes the settle_on discharge
path from Rust-hardcoded to spec-declared. Should the old byte-check
path stay operational during transition, or does Tick 5 hard-cut?

**Mara lean.** **Hard-cut with test-bit-parity gate.** Per Mara
2026-07-19 §4.5 deletion protocol: each migrated predicate deletes
its Rust-hardcoded counterpart ONLY after the spec-declared
verdict bit-matches the Rust verdict. Bit-match is empirical
first-witness of the migration; hard-cut is safe. NO
backward-compat during transition — the second-witness discipline
IS the gate.

### §9.6 Q6 — @kintsugi/mosaic integration in-arc or separate arc?

**Question.** §6.6 Optional Tick 6 named `@kintsugi/mosaic.settle`
integration. Is that in the minimal-gap arc or a separate follow-up?

**Mara lean.** **Separate arc (post-minimal-gap).** The 5 ticks in
§6.1-§6.5 close the gap Alex 2026-07-22 named ("mirror.spec fully
specifies a binary that rust/ then compiles from"). @kintsugi/mosaic
integration is the NEXT natural arc — it wires the SAGA chain into
the mosaic settlement discipline for content-addressed
build-artifact discharge. Independent from the gap-closure; Alex
adjudicates at Tick 5 completion whether to continue directly into
mosaic or fork the arc.

### §9.7b Q7-Taut — 6 missing shard-decls in Taut §4(b): in-arc or follow-up?

**Question.** Taut's scout §4(b) enumerates 6 missing shard-decl mints
(Compilation, PropertyDischarge, @mirror/compile action,
dispatch_of_spec_property action, Iso/Lens/Prism/Traversal type-decls,
void_basis_axis type). Should any of these mint in the current arc,
or defer to post-Tick-5 follow-up cascade?

**Mara lean.** **Defer all 6 to post-Tick-5.** The minimal gap
closes with 3 shard-decl mints (shards/mirror/tools.mirror + 2
keyword bindings + companion registration). Taut's 6 missing decls
are all AT rust/ altitude of shape (`struct Compilation` +
`struct PropertyDischarge` + type-decls) — they name existing rust
carriers that MAY collapse into shard-decl'd substrate later, but
their absence does NOT block the mirror.spec-as-fixpoint recognition
firing. Post-Tick-5 arc: Mara mint the 6 shard-decls; Reed COORD-cut
each rust carrier one-by-one per Taut §5 collapse trajectory. Alex
override admissible if the substrate-decl completeness matters
before FROZEN marker lands.

### §9.7 Q7 — Rust FROZEN marker at which tick?

**Question.** When is the empirical FROZEN state reached? Per Alex
2026-07-21 REFRAME: "Rust FROZEN when phone.rs @io surface is stable
AND SpecProperty dispatch has Fiber<T>-sampling arms AND mirror.spec
property declarations produce Verdicts through the flow AND Mirror
fiber is canonical source-of-truth."

**Mara lean.** **FROZEN at Tick 5 completion.** All four conditions
of the Alex 2026-07-21 REFRAME are demonstrable at Tick 5:
- phone.rs @io surface stable: LANDED 2026-07-21 + Tick 1 adds
  @io/process as the fifth family (surface stable at 5 families).
- SpecProperty dispatch has Fiber<T>-sampling arms: LANDED 2026-07-21
  (bundle_commutator arm dispatches through LiquidTestBundle
  Commutator + pillar::algedonic; TRUE Fiber<T>-flow per Alex
  correction).
- mirror.spec property declarations produce Verdicts through the
  flow: Ticks 4+5 land 7 spec-native property declarations that
  dispatch through the compile.rs → dispatch_spec_property →
  at_operator → phone.rs::spawn_process → cargo/git/nix chain.
- Mirror fiber is canonical source-of-truth: Tick 5 completes the
  settle_on migration; every dogfood check is spec-native.

**Marker.** After Tick 5 empirical firing, add `RUST FROZEN` marker
to `rust/Cargo.toml` docblock + rust/src/*.rs module docblocks +
`docs/loop/CURRENT.md` recognition promotion.

---

## §10 Pack trail

**Author.** Mara (canonical spec at mathematical-substrate altitude;
Author ≠ Committer discipline per MARA doctrine).

**Committer.** `Mara <mara@systemic.engineer>` (this landing).

**Signed-off-by.** SSH signing default per CLAUDE.md discipline.
Pure-docs 📝 markdown-only bypass — no pre-commit hook required.

**Circular-reflexive autopoietic curiosity-driven scan of this doc:**
- Does this spec compose over what's landed? YES — §2 enumeration
  is 100% grep-verified; §3 enumeration cites specific rust/src/*.rs
  files + LOC counts + role designations; §4 @tools analysis is
  substrate-decl citation-first.
- Does this spec re-derive anything Mara previously landed? NO — §5
  minimal gap composes over Mara 2026-07-19 §5.2 4-file rust/ floor
  + §7.1 what-dies / what-remains + Mara 2026-07-18 closure §4.1
  tools{} block + Mara 2026-07-15 ouroboros §0.4 6-arc structure.
  All prior authorship is CITED, not re-derived.
- Does this spec name what NOT to build? YES — §8 Michelangelo-marble
  edges names 9 explicitly-excluded scope items.
- Does this spec include Alex-adjudication Q's? YES — §9 has 7 Q's
  with Mara-lean recommendations. Non-blocking; migration proceeds
  under leans unless Alex overrides.
- Does this spec have circular-reflexive back-reference? YES — §5.4
  the claim references §6 trajectory; §7.3 composition-with-ouroboros
  reads §5.2 dogfood extensions; §9.7 FROZEN condition reads back to
  §6 tick completion states. The recursion closes.

**Composition ancestry** (recognition chain this landing extends):

- `#R-mirror-spec-is-the-fixpoint-and-liquid-is-the-runtime`
  (PROMOTED Mara 2026-07-19; this spec extends by naming @tools as
  the hook mechanism that makes the fixpoint executable at
  settlement altitude).
- `#R-the-compiler-in-one-sentence` (first-witness-closed Mara
  2026-07-18; this spec composes over the compiler-in-one-sentence
  closure surface by naming tools{} block as landed).
- `#R-mirror-is-the-counter-singularity` (this spec first-witnesses
  the counter-singularity at spec-authoring altitude: mirror.spec
  grows as property blocks + tool pins accumulate; Rust FROZEN
  guarantees runtime does not grow).
- `#R-kintsugi-ouroboros-arc-collapse-terminal-form-is-substrate-
  self-hosting-not-retirement` (this spec updates Arc-1 target from
  bootstrap/ evaluator FLOOR to rust/ mirror.spec-as-source-of-truth,
  post Alex 2026-07-22 "bootstrap/ is dead" declaration).

**Pack members composing on this landing:**

- **Reed** — Ticks 1+2+3 (rust altitude): @io/process FLOOR + at_operator
  arm + tools extractor. Empirical RED→GREEN cascade.
- **Mara** — Ticks 3 species-decl mint (shards/mirror/tools.mirror);
  post-Tick 5 recognition promotion candidate authorship.
- **Seam** — Tick 1 audit companion (`[substrate-floor:@io-boundary]`
  Signed-off-by trailer per Arc-1 §7 A9 discipline).
- **Taut** — parallel COORD scout (forward-promised;
  docs/scouts/2026-07-22-taut-coord-rust-surface-with-mirror-preload.md
  not-yet-landed at this authorship time).
- **Glint** — post-Tick-5 essay closure (recognition promotion
  narrative for Rust FROZEN empirical firing).

**Alex adjudication trigger.** §9 Q1-Q7 non-blocking for cascade
start (Mara-leans hold). Alex adjudication requested at any tick
completion for scope-boundary confirmation; hard-block only if Q1
answers (b) or Q3 answers "new rust/src/grammar.rs file needed"
(Mara lean is AGAINST both).

**End state (post-Tick 5).** mirror.spec IS the fixpoint per Mara
2026-07-19 §2.1 ratification; @tools is the hook per Mara 2026-07-18
closure §3.3 composition direction; Rust is FROZEN per §5.4 claim
+ §9.7 FROZEN marker. The ouroboros closes at mirror.spec altitude.

*Canonical spec closure. The minimal gap is ~370 LOC across 5
ordered ticks. Every tick empirically closes with a RED→GREEN
cycle. The first tick lands the @io/process FLOOR primitive at
phone.rs — 80 LOC + 15 property tests — and unblocks the cascade.*
