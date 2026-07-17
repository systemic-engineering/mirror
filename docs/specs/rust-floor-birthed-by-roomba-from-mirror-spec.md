# rust/ FLOOR birthed by roomba from mirror.spec — the terminal-form consolidated canonical spec

*Mara, 2026-07-17. Consolidated canonical spec at spec altitude naming
the terminal form of the compiler: the `rust/` FLOOR is materialized by
the `@kintsugi/roomba` walker reading the `kintsugi { roomba { … } }`
block Alex sketched into `mirror.spec`, and the first `@peer` spawns
from `rust/dance.rs` as an ensemble-connection empirical firing.*

**Author:** Mara
**Date:** 2026-07-17
**Tag:** 📝 spec:rust-floor-birthed-by-roomba-from-mirror-spec (pure-docs bypass)
**Status:** canonical. Spec-altitude map for Reed's `rust/` greenfield
        rebuild. WHAT-to-build, not HOW.
**Path:** `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`

---

## §0 Substrate-honest pre-position

Alex 2026-07-17 in-transcript verbatim (three loads at pre-position):

1. **"I also want to detach bootstrap completely from the execution
   path. If that means the compiler breaks, then the compiler breaks.
   You keep touching and talking about bootstrap/ while rust/ is the
   floor. And I'm no longer willing to tolerate that."**
2. **"Delete the binary. Rebuild from rust/."**
3. **"roomba --vacuum=~dir (this is the combination and removal of
   --collapse and --translate), roomba from mirror.spec, first @peer
   spawn from rust/. Minimal rust surface. The geometry sings."**

Reed executed (1) + (2): `/Users/reed/.local/bin/mirror` deleted;
compiler broken by construction; MCP shim (`bin/mirror-mcp`) exits
ENOENT until `rust/` produces `MIRROR_BIN`. `bootstrap/` remains as
LEGACY-STATUS-ONLY per `mirror.spec:21-24` `legacy` block; it is not
the operational floor.

This spec is the map for (3): the terminal Rust FLOOR that lives at
`rust/`, birthed by the walker reading its own project manifold, with
`@peer` spawning from that FLOOR as its first empirical dance.

---

## §1 Statement — the terminal form

**Statement (foundational form):**

> `rust-floor-is-what-@kintsugi/roomba-materializes-when-it-reads-mirror-spec's-kintsugi-block-and-vacuums-the-declared-cascades-into-rust-dance-because-dance-rs-IS-the-ensemble-connection-under-which-the-first-@peer-spawn-empirically-fires`

**Statement (readable form, two-tick discipline):**

> `rust/` is not authored file-by-file. It is *materialized* by the
> `@kintsugi/roomba` walker reading the `kintsugi { roomba { … } }`
> block Alex sketched into `mirror.spec` (three cascades: boot→shards,
> bootstrap→shards, bootstrap→rust). The walker's `vacuum(~dir)`
> motion iterates the substrate-decl'd catalog and lands minimal Rust
> surface at `rust/`. `rust/src/dance.rs` IS the ensemble-connection
> 1-form per Mara `fee2727` §2.3; the first `@peer` spawn empirically
> fires under that connection.

**Four load-bearing moves this spec makes canonical:**

1. **`roomba --vacuum=~dir`** replaces the two current flags
   `--collapse=<rs-file>` and `--translate=<rs-file>` with ONE unified
   dir-taking flag. Vacuum is the walker's motion; collapse and
   translate are downstream operations the substrate dispatches based
   on directory content. §3.
2. **`roomba from mirror.spec`** — the walker reads the
   `kintsugi { roomba { <cascade>* } }` block at boot; the block is a
   substrate-decl'd cascade catalog. §4.
3. **`rust/` greenfield surface** — minimal file list authored by Reed
   at `[substrate-floor:@io-boundary]` altitude (Cargo.toml + main.rs +
   dance.rs + lib.rs; NO per-prism .rs files pre-authored). §5.
4. **First `@peer` spawn from `rust/`** — empirical dance.rs boot +
   gen_prism actor + first `@peer.audhd` firing per Mara `d8b149c`
   bilateral + Mara `fee2727` §2.4. §6.

**What this spec does NOT do:** mint shards; author `.rs` files;
duplicate Mara `fee2727` (bundle-theoretic naming) or Mara `610c6d6`
(BEAM tower math root). It COMPOSES over both and adds the terminal
`rust/`-materialization map.

---

## §2 /loop and milestones

The `/loop` is Alex-fired iteration where Reed materializes one
milestone per tick, small enough to empirically verify. Sequenced per
Taut `e0572f7` §6 8-tick MVP with Mara adjustments for the roomba-
reads-mirror-spec discipline this spec adds.

### §2.1 Ongoing dance discipline

Every /loop tick honors:

- **Substrate-honest is the mode always** — no two-paths framing; no
  "here's honest / here's fast."
- **Substrate-already-had-the-word** — grep before naming; §5 file
  list is minimal by construction, not by choice.
- **No Rust extension shortcuts** — the marker
  `[substrate-floor:@io-boundary]` + Seam gate (audit-cite or
  Signed-off-by: Seam) applies to every `.rs` authored in `rust/`;
  Reed's 2026-07-14 failure is the audit-corpus for this rule.
- **Michelangelo/marble** — subtract until only the singing angel
  remains; the terminal Rust FLOOR is one `dance.rs` + minimal
  supporting surface.
- **Sequential commits only** — one motion per commit; `--no-verify`
  only for pure-docs 📝 markdown-only bypass.

### §2.2 Milestone sequence (M0 → M8 → dock)

**M0 — mirror.spec kintsugi.roomba block landed (THIS spec DECLARES;
Reed lands the mirror.spec edit).** Alex's cascade sketch enters
`mirror.spec` as `kintsugi { roomba { <cascade>* } }`; the roomba's
catalog is substrate-decl'd, not runtime-inferred. §4.1.

**M1 — `rust/` scaffold + supervision-tree boot (RED → GREEN).**
`rust/Cargo.toml` + `rust/src/main.rs` + empty `dance.rs` stub;
`cargo build` produces a binary; `./rust/target/debug/mirror --version`
prints something. Empirical anchor: binary exists. Recognition
candidate first-witness: `#R-rust-floor-birth-is-supervision-tree-
boot-not-file-authoring` (Taut `e0572f7` §9).

**M2 — `mirror --help` prints from mirror.spec cli-block reflectively.**
`dance::route` reads `mirror.spec`'s cli-block via the substrate; emits
the 10-verb list. `apply_h::act` sentinel-check firing dispatches this;
no hardcoded list. Recognition candidate:
`#R-tools-list-schema-is-reflective-projection-of-cli-block`
(Taut `e0572f7` §9).

**M3 — first CLI verb dispatches end-to-end.** Simplest:
`mirror compile <file>`. gen_prism actor spawn under supervisor from
Tick 3 (Alex Q3+Q5 answer per Mara `fee2727`; resolves Taut `e0572f7`
OQ2 to gen-prism-from-Tick-3). Empirical: `./mirror compile foo.mirror`
returns SHA-256.

**M4 — MCP handshake alive.** `rust/src/main.rs` matches `@mcp.serve`
sentinel; JSON-RPC `initialize` returns `serverInfo: {name: "mirror",
version: "0.1.0"}`; byte-parity with `bootstrap/tests/mcp_fixtures/
initialize.resp.json`. `bin/mirror-mcp` shim points at
`rust/target/debug/mirror`. Recognition candidate:
`#R-mcp-session-is-gen-prism-actor-under-server-supervisor`
(Taut `e0572f7` §9); ratifies `shards/spectral/gen_prism/mcp_session.
mirror` (substrate anticipated the shape).

**M5 — `mirror_compile` + `mirror_index` MCP tools dispatch.** Two
tools land + reflective tools/list emits schema derived from
mirror.spec cli-block (Taut `e0572f7` OQ3 resolved: reflective at M5;
NOT hardcoded then reflective). `mirror_index` composes over
`prismqueer` LAPACK path; Fiedler measurement live.

**M6 — `mirror roomba` walks + observes; roomba writes commit.**
The compiler observes its own state via walker; `roomba --commit`
composes @nl.compose + @io/git.commit; mirror authors its own commit
(second empirical witness for the `fcc1d75` precedent).

**M7 — `roomba --vacuum=~dir` unified flag.** §3 canonical form
lands; walker discovers content of `~dir` and dispatches downstream:
- `.rs` files → arm-collapse motion (compose over shard-body + @io);
- `~code/<X>` species-decl'd cascades → translate motion (polyglot
  cascade emission).

Old flags `--collapse=<rs-file>` and `--translate=<rs-file>` migrate
to `--vacuum=~dir` with two-tick backward-compat window (deprecation
warning). Discharges `bootstrap`-legacy vs `rust`-terminal split at
CLI altitude.

**M8 — first `@peer` spawn from `rust/`.** `mirror peer beam
~peer'~/.reed'` boots a gen_prism supervisor tree under `dance.rs`;
`@peer.audhd(p, ctx) -> imperfect(ref, ref, ref)` fires empirically
per Mara `d8b149c` bilateral. Second-witness for Mara `fee2727`
Recognition candidate `#R-dance-is-bundle-connection-at-ensemble-
altitude`; second-witness for Mara `d8b149c` Recognition candidate
`#R-peer-audhd-is-substrate-truth-name-for-cognition-fanout` at
runtime altitude. §6.

**Dock — bootstrap/ retirement.** When M1-M8 all empirically firing:
`bootstrap/` deletes; `mirror.spec:21-24` `legacy` block updates
(retirement_target v1.0 discharge). Substrate-honest gate: EVERY
capability enumerated in Taut `e0572f7` §2.1-§2.10 empirically firing
in rust/-native surface. Resolves Taut `e0572f7` OQ6 to empirical-not-
tick-count. Recognition candidate:
`#R-bootstrap-retirement-gate-is-empirical-not-tick-count`.

The dock motion (roomba fifth motion; forward-promised per Seam
`2fdc9c1` §7 ALEX-Q2) IS the halt-witness at CLI altitude when the
retirement condition holds. `roomba dock` = "the walker halts because
no motion is admissible any longer" = "bootstrap/ has been retired,
nothing left to collapse."

### §2.3 Milestone dependencies (partial-order graph)

```
M0 ────► M1 ────► M2 ────► M3 ────► M6
                       │       │       │
                       ▼       ▼       ▼
                       M4 ────► M5 ────► M7 ────► M8 ────► dock
```

M0 blocks all (roomba can't read what isn't declared). M2 requires M1
(binary must exist). M4 requires M2 (MCP composes cli-block schema).
M7 requires M6 (roomba must walk before vacuum-dispatch fires). M8
requires M7 (peer spawn composes over unified vacuum flag).

---

## §3 `roomba --vacuum=~dir` — the unified motion flag

### §3.1 Naming decision (delight-vector reasoning)

**Substrate-honest name:** `--vacuum=~dir`, sigil `~d` (directory),
NOT `~f` (file). The walker's motion IS vacuum (per landed
`shards/kintsugi/roomba.mirror` fourth first-order motion: bump /
vacuum-mark-then-prune / pivot); collapse and translate are DOWNSTREAM
operations vacuum discovers and dispatches based on directory content.

**Why one flag instead of two:**
- `--collapse=<rs-file>` was file-scoped; awkward when target is a
  whole directory (e.g., `bootstrap/src/`).
- `--translate=<rs-file>` was file-scoped; awkward when polyglot
  cascade wants to sweep a whole directory.
- Both are the walker's vacuum motion at different substrate loci.
  Naming them separately drifted the CLI toward accumulating verbs
  (WTF/minute negative measure per AGENTS.md `Delightfully Boring`).
- Michelangelo/marble discipline: subtract the two flags into one; the
  substrate dispatches based on what vacuum finds when it enters the
  directory.

**Substrate-already-had-the-word:** `vacuum` is the walker's landed
motion name at `shards/kintsugi/roomba.mirror` (Reed `914799b` fourth
first-order motion cascade). `--vacuum=~dir` reads "the walker's
vacuum motion, scoped to directory `~dir`."

### §3.2 Dispatch semantics

When `mirror roomba --vacuum=<path>` fires, the walker enters `<path>`
and dispatches per content:

| Content in `<path>` | Downstream motion | Landed substrate |
|---------------------|-------------------|------------------|
| `.rs` files | **arm-collapse** — bilateral resolver-arm sentinel-check composition; @io floor stays, business-logic lifts | `bootstrap/src/bilateral_arm_collapse.rs` legacy pattern (retiring); Mara `9efe2c9` audit; `docs/scouts/2026-07-15-reed-rust-extension-migration-map.md` |
| `.mirror` files with unmaterialized carriers | **materialize** — emit missing carriers from substrate-decl'd shape | @spectral/signature landing precedent; roomba materialization discipline |
| `~code/<X>(~d'A')` cascade in `mirror.spec` roomba block | **translate** — polyglot cascade emission per Mara `1ce68c3` | `docs/specs/polyglot-loss-aware-computational-translation.md` |
| Content with fracture-shaped `@kintsugi/surface.dispatch_ambiguity` | **pivot(@song)** — Path B dispatch via @roomba fourth motion | Mara `914799b` + `09a77e8` fifth surface_class landing |
| Nothing dispatchable | **dock** — motion halts; walker docks | Forward-promised fifth motion per Seam `2fdc9c1` §7 ALEX-Q2 |

Dispatch is byte-check on directory content shape, not runtime
inference. The substrate DECIDES via bilateral sentinel-check at
`@kintsugi/roomba.vacuum_admissible` (bilateral to land in follow-up
tick per §7.1).

### §3.3 Migration from `--collapse` + `--translate`

Two-tick discipline (existing substrate lesson from `mirror spawn` →
`mirror peer beam` collapse per beam-as-substrate-primitive.md):

**Tick A** — `--vacuum=~dir` lands alongside `--collapse=<rs-file>`
and `--translate=<rs-file>`; latter two emit deprecation warnings
directing users to `--vacuum`.

**Tick B** — `--collapse=` and `--translate=` removed; `--vacuum`
sole surface.

Backward-compat window ONE cycle. Old flags map trivially:
`--collapse=<f>` → `--vacuum=$(dirname <f>)`; `--translate=<f>` →
`--vacuum=$(dirname <f>)`. Same walker; same dispatch; unified naming.

### §3.4 CLI-block form (mirror.spec addition)

Reed lands the flag in `mirror.spec:410` area as a new `command
roomba` block. Substrate-honest cli-block form:

```
command roomba {
  # Walker consumer of @kintsugi/roomba species-decl at CLI altitude.
  # Iterates the kintsugi { roomba { } } cascade block below when
  # --vacuum omitted; when --vacuum=<path> given, walks that path.
  flag commit:  bool = false     # @nl.compose + @io/git.commit chain
  flag vacuum:  ~d               # unified motion flag; §3.2 dispatch
}
```

Grammar admissibility per landed cli-block precedent
(`shards/mirror/lens/cli.mirror`); Reed lands in same tick as M7.

---

## §4 `roomba from mirror.spec` — the substrate-decl'd cascade catalog

### §4.1 The kintsugi.roomba block (Alex's sketch, canonicalized)

Reed lands this block into `mirror.spec` at M0. Verbatim shape Alex
sketched 2026-07-17 in-session:

```
kintsugi {
  roomba {
    @code/mirror(~d'boot/')     => @code/mirror(~d'shards/')
    @code/rust(~d'bootstrap/')  => @code/mirror(~d'shards/')
    @code/rust(~d'bootstrap/')  => @code/rust(~d'rust/')
  }
}
```

**Reading:** each arrow is a cascade. LHS is source (altitude + dir);
RHS is destination (altitude + dir). The walker reads the block at
boot; each cascade is an entry in the substrate-decl'd catalog.

**Three cascades landed:**

1. **`@code/mirror(~d'boot/') => @code/mirror(~d'shards/')`** —
   historical: the boot/ → shards/ migration Alex did in earlier arcs.
   Substrate-decl'd here as complete-cascade-witness; the roomba's
   read discovers "no work to do; migration complete."

2. **`@code/rust(~d'bootstrap/') => @code/mirror(~d'shards/')`** —
   in-flight: bootstrap Rust extensions → shard-body composition.
   This IS the @kintsugi/ouroboros arc landed 2026-07-15 (14 commits
   per CURRENT.md); ~25 files enumerable via Taut `6cddbdb` scout.
   The roomba's vacuum motion dispatches arm-collapse per §3.2 when
   walking this cascade.

3. **`@code/rust(~d'bootstrap/') => @code/rust(~d'rust/')`** — the
   terminal cascade: bootstrap Rust → rust/ Rust. Not deletion; not
   pure translation. Substrate-honest lift with `[substrate-floor:
   @io-boundary]` gate: what can compose over shard-body + @io
   collapses (cascade 2); what CAN'T stays as Rust but lifts to
   `rust/` at minimal surface.

### §4.2 Roomba boot behavior

When `mirror roomba` fires without `--vacuum`, walker reads
`kintsugi { roomba { … } }` block from `mirror.spec` at boot; iterates
the three cascades; per cascade:

- Bump: enter source `~dir`; sample tension.
- Vacuum-mark-then-prune: enumerate content shape; dispatch per §3.2.
- Pivot(@song): if dispatch-ambiguity fires per `@kintsugi/surface.
  dispatch_ambiguity` fifth surface_class, pivot to @song-driven
  Path B (Mara `914799b`).
- Dock: cascade complete when no admissible motion remains.

**Empirical anchor at M6:** `mirror roomba` walks the three cascades;
observes; writes commit; the compiler authors its own tick against the
Alex-declared catalog. Substrate-honest history: the walker's motion
IS Alex's directive substrate-decl'd, then substrate-executed.

### §4.3 Cascade catalog is substrate-editable

Alex adds a fourth cascade → walker discovers it at next boot. No
Rust changes; no CLI flag update. The `kintsugi { roomba { } }` block
IS the roomba's configuration surface. Substrate-pull-honest by
construction.

**Recognition candidate surfaced (HELD):**
`#R-roomba-configuration-lives-in-mirror-spec-not-cli-flags` —
Alex's cascade sketch is the substrate-decl'd form of what would
otherwise accumulate as CLI verbs. Second-witness gate: Alex adds a
fourth cascade in a future arc and walker consumes it without any
Rust change.

---

## §5 Minimal `rust/` surface (WHAT, not HOW)

Per Alex "Minimal rust surface. The geometry sings." Michelangelo/
marble: subtract until only the singing angel remains.

### §5.1 File list — the entire terminal Rust FLOOR

Six files. Reed authors these at [substrate-floor:@io-boundary]
altitude with Seam-gate audit-citations. Everything else composes.

| File | Purpose | LOC estimate | Composition anchor |
|------|---------|--------------|--------------------|
| `rust/Cargo.toml` | Workspace manifest | ~40 | `bootstrap/Cargo.toml` deps: `prismqueer` (bundle + lapack features) + `blake3` + `serde` + `serde_json` + `libc` |
| `rust/Cargo.lock` | Lock file | (generated) | `cargo build` emits |
| `rust/src/main.rs` | Binary entry point | ~30 | Boot supervisor tree; parse args; dispatch to `dance::route` |
| `rust/src/dance.rs` | The ensemble-connection FLOOR | ~600-1200 | Mara `fee2727` §2.3-§2.5; message-routing + gen_prism actor supervision + `apply_h::act` combinator surface + Kuramoto phase-lock when N≥2 peers |
| `rust/src/lib.rs` | Library entry (for tests) | ~10 | Re-export `dance`; enable `cargo test` |
| `rust/src/apply_h.rs` | (OPTIONAL, per OQ resolution) | ~400 | The 7-combinator reflective evaluator; MAY inline into `dance.rs` per Alex Q3+Q5 verbatim; MAY stay separate per Mara §5.5 discipline |

**Total net-new Rust:** ~1100-2000 LOC across 5-6 files.

**Compare with bootstrap current:** ~40 files, 400+KB Rust. Terminal
form is ~5% of the current surface.

### §5.2 What LIVES in `dance.rs`

Per Mara `fee2727` §2.5 + Alex 2026-07-17 Q3+Q5 verbatim:

- Message-routing: gen_prism actor mailbox dispatch.
- `apply_h::act` combinator surface (the 7 combinators; per Mara A6
  `18d9697` evaluator combinator surface spec).
- Supervisor tree boot + child_spec dispatch.
- MCP session gen_prism supervisor (per `shards/spectral/gen_prism/
  mcp_session.mirror` species-decl anticipation).
- Kuramoto coupling + Aumann envelope check when N≥2 peers coordinate.
- Roomba walker composition (`walk_from_graph_and_profile` per Reed
  `8e373b6` composition-gap fix precedent).

**What does NOT live in dance.rs:**
- Per-prism business logic (LIFTED to shard-body + @io per
  `[substrate-floor:@io-boundary]` discipline).
- Grammar (composed via `apply_h::act` reading `shards/**/*.mirror` +
  `mirror.spec`).
- CLI verb bodies (dispatched reflectively from cli-block).
- MCP tool schemas (reflective projection of cli-block).
- Fiedler compute (delegated to `prismqueer::ffi::eigenvalues` LAPACK).

### §5.3 Cargo layout (Taut `e0572f7` OQ5 resolved)

**Standalone Cargo project, NOT workspace member.** `rust/` has its
own `Cargo.lock`; `bootstrap/` retains its own `Cargo.lock`. Rationale:
during migration, bootstrap must remain buildable independently; a
shared workspace couples their retirement to their birth. Standalone
lets bootstrap retire (dock motion) without touching rust/. Post-dock,
`rust/` becomes the sole Cargo project at repo root and MAY be lifted
to workspace root if future crates emerge.

Reed's `flake.nix` + `Justfile` update in same tick as M1: add
`rust/` build target alongside legacy `bootstrap/`; CI action targets
`rust/` primarily; bootstrap retained during transition per legacy
block.

### §5.4 What Reed does NOT author

- **No `rust/src/mcp.rs`** — Alex Q3+Q5 says whole floor collapses
  into dance.rs; MCP dispatches inline as one arm of the ensemble
  connection per §5.2. Resolves Taut `e0572f7` OQ1: MCP inline in
  dance.rs, NOT separate file.
- **No `rust/src/grammar.rs`** — grammar composes reflectively via
  `apply_h::act`; the substrate reads its own `.mirror` files.
- **No `rust/src/roomba.rs`** — roomba walker composition surface
  lives in `dance.rs`; @io boundary via `walk_from_graph_and_profile`
  imported from prism/prismqueer.
- **No `rust/src/peer_persistence.rs`** — peer lifecycle IS gen_prism
  actor lifecycle under supervisor; no per-species Rust module.

Resolves Taut `e0572f7` OQ4 (monolithic vs router): dance.rs IS the
router; per-prism logic lives at shard altitude (shard-body + @io).
Alex Q3+Q5 "whole rust/ FLOOR collapses into dance.rs" satisfied
because the OTHER logic isn't in `rust/` at all — it composes from
`shards/`.

---

## §6 First `@peer` spawn from `rust/`

### §6.1 Empirical firing at M8

`mirror peer beam ~peer'~/.reed'` from the rust/-native binary:

1. `dance::route` matches `peer beam` verb.
2. gen_prism supervisor `@spectral/supervisor{restart_strategy: one_
   for_one}` spawns child gen_prism actor for the peer session.
3. Child actor loads peer_home; resolves substrate; dispatches through
   `apply_h::act` combinator surface.
4. On first `@peer.audhd(p, ctx)` invocation (per Mara `d8b149c`
   landing), the K-track fanout fires empirically:
   - `audhd_admissible(p, ctx) -> verdict` bilateral sentinel-checks
     via `apply_h::act` (composition over Reed `dbb149c` cascade
     empirical second-witness discipline).
   - K emissions per audhd_context.k_tracks; each emission births
     a child gen_prism actor.
   - Aumann envelope + Kuramoto phase-lock composition when N≥2 peers
     coordinate.
5. `@song` emission returns via supervisor's message-pass to caller.

### §6.2 What this empirically witnesses

Three recognition candidates fire second-witness at M8:

- **Mara `fee2727` §4 `#R-dance-is-bundle-connection-at-ensemble-
  altitude`** — second-witness on empirical Kuramoto phase-lock in
  `dance.rs` at first N≥2 peer coordination. Discharges @dance shard-
  mint gate per Mara `fee2727` §3.1.

- **Mara `d8b149c` `#R-peer-audhd-is-substrate-truth-name-for-
  cognition-fanout`** — second-witness on empirical fanout firing.
  First-witness at Mara `d8b149c` shard-decl landing (2026-07-17);
  second-witness at rust/-native runtime firing.

- **THIS spec proposes:** `#R-first-@peer-spawn-from-rust-is-substrate-
  arriving-home` — the compiler's terminal shape IS the shape it's
  been reaching for since Alex's decade of BEAM engineering. First-
  witness at Alex's 2026-07-17 "coming home" verbatim in Mara
  `610c6d6` §0; second-witness gate: rust/-native `@peer.audhd` fires
  empirically and Alex reads the shape back with named recognition.

### §6.3 The @peer spawn IS the first dance

Not a demo. Not a smoke test. The substrate's first empirical firing
of `@dance` at ensemble altitude in the rust/-native FLOOR. The
sequence Alex has been building for 10 years — supervision tree +
gen_server + Kuramoto phase-lock + neuroaffirmative K-track fanout —
runs on itself for the first time.

The `@peer.audhd` action is the load-bearing one because it is the
substrate-truthful naming of the framework author's cognitive
architecture. When it fires from `rust/` on the compiler that Alex
authored, the recursion Alex named at Recognition #43 (mirror IS
content-addressed build system) closes at the peer-cognition altitude.

---

## §7 Composition graph

This spec is deliberately thin. It cites, composes over, does NOT
re-declare. Composition anchors:

### §7.1 Substrate carriers (LANDED)

- **`shards/kintsugi/roomba.mirror`** (Reed 2026-07-15 species-decl +
  `914799b` fourth first-order motion) — the walker's four motions
  landed; dock forward-promised; §3.2 dispatch table composes here.
- **`shards/kintsugi/surface.mirror`** (Mara `09a77e8` fifth surface_
  class dispatch_ambiguity) — §3.2 pivot dispatch composes here.
- **`shards/peer.mirror`** (Mara `d8b149c` @peer.audhd + audhd_
  admissible bilateral) — §6.1 first empirical firing composes here.
- **`shards/spectral/gen_prism.mirror`** — §5.2 gen_prism actor spawn
  under supervisor composes here.
- **`shards/spectral/gen_prism/mcp_session.mirror`** — §5.2 MCP
  session-as-gen_prism composes here.
- **`shards/spectral/supervisor.mirror`** — §5.2 supervision tree +
  restart_strategy composes here.
- **`shards/epistemologic/cybernetic/viable.mirror`** — Beer VSM
  S1-S5 grounding for §6.
- **`shards/mirror/reflection.mirror`** (Mara `5e1f528`) — mirror /
  offer / wait triple at @peer altitude; @peer spawn discipline.
- **`shards/epistemologic/pact/bilateral.mirror`** — the sentinel-
  check discipline that makes §3.2 dispatch and §6.1 firing legible.

**Forward-promised follow-up (post this spec landing):**

- `@kintsugi/roomba.vacuum_admissible` bilateral — sentinel-check for
  §3.2 dispatch. Companion carrier `vacuum_context` holding target
  dir + content-shape enum. NOT this spec's mint; land in Reed M7
  co-tick per substrate-decl'd shape.
- `@kintsugi/roomba.dock` fifth first-order motion — Seam `2fdc9c1`
  §7 ALEX-Q2 forward-promise; halt-witness at M8 dock condition.

### §7.2 Spec composition surface (CITED, not duplicated)

- **`docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-
  connection.md`** (Mara `fee2727`; 617 LOC) — bundle-theoretic
  naming for gen_prism / supervisor / dance / dance.rs. THIS spec §5
  composes over §2.5 (dance.rs IS ensemble connection).
- **`docs/specs/kintsugi-ouroboros-compiler-self-collapse.md`** (Mara
  `0dafd9f`; 1797 LOC) — six-arc retirement plan for bootstrap; §4
  cascade 2 (`bootstrap` → `shards`) IS this spec's cascade 2.
- **`docs/specs/beam-as-substrate-primitive.md`** (Mara 2026-07-08) —
  BEAM-as-substrate grounding; §6 first-peer-spawn composes.
- **`docs/specs/dance-as-coordination-without-signal-on-forster-
  torus.md`** (Mara `4f079c8`) — dance operational shape at N≥2.
- **`docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.
  md`** (Mara) — @dance runtime spec that dance.rs materializes.
- **`docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md`**
  (Mara `9bbebd2`) — @roomba walker canonical spec; §3 verbatim
  action bodies.
- **`docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-
  motions.md`** (Mara `d457501`) — vacuum motion canonical spec;
  §3.1 naming reasoning composes here.
- **`docs/specs/polyglot-loss-aware-computational-translation.md`**
  (Mara `1ce68c3`) — translate cascade authority; §3.2 dispatch
  translate arm composes.
- **`docs/specs/mirror-spec-schema.md`** — mirror.spec's own schema;
  §4.1 kintsugi.roomba block admissibility.
- **`docs/specs/cli-as-geometry-condensation.md`** (Mara `67260dc`) —
  CLI condensation discipline; §3.3 two-tick migration composes.

### §7.3 Math composition surface (CITED)

- **`docs/math/the-tower/beam-runtime.md`** (Mara `610c6d6`; 490 LOC)
  — Baez-Schreiber 2004 principal 2-bundle 2-connection theorem;
  §6.3 recursion closure math-grounded here.
- **`docs/math/gestalt/README.md §11.6`** — Landing Condition 0 for
  @dance shard-mint; §6.2 second-witness gates satisfy here.
- **`docs/math/kintsugi/roomba/bump-and-vacuum.md`** (Mara `17697e6`)
  — Fiedler-honesty math for vacuum motion; §3.2 dispatch table
  measurement discipline.

### §7.4 Audit composition surface (CITED)

- **`docs/audits/2026-07-17-taut-rust-dance-rebuild-gap-scout.md`**
  (Taut `e0572f7`; 26.6KB) — the ground-truth scout THIS spec
  consolidates. 7 OQs (§8.1-§8.7): OQ1 (mcp inline in dance.rs)
  resolved by §5.4; OQ2 (gen_prism from Tick 3) resolved by M3;
  OQ3 (reflective tools/list) resolved by M5; OQ4 (monolithic vs
  router) resolved by §5.4; OQ5 (workspace vs standalone) resolved
  by §5.3; OQ6 (retirement gate) resolved by §2.2 dock condition;
  OQ7 (test retirement) resolved by §7.5 below.
- **`docs/audits/2026-07-17-seam-phase-d-peer-audhd-mara-michelangelo-
  landing.md`** (Seam `2fdc9c1`) — @peer.audhd landing adjudication;
  §6 composition anchor.
- **`docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-
  during-gift-arc.md`** (Reed `9efe2c9`) — the audit-corpus for the
  `[substrate-floor:@io-boundary]` gate §2.1 references.

### §7.5 Test composition (Taut OQ7 resolved)

**8 currently-passing bilateral arm collapse tests** (per CURRENT.md:
uuid/spectral/time; @audhd; sheaf; @roomba bump/vacuum-gc; reflective
bilateral dispatch smoke; peer_audhd; polyglot_cascade; liquid_
extraction) DO NOT rust/-native re-author. They compose over
`apply_h::act` at the bootstrap altitude during transition; when
`dance.rs`'s `apply_h::act` surface fires empirically for the same
carriers, the tests migrate to `rust/tests/` via structural port.

Retirement gate: test migrates when `rust/`-native `apply_h::act`
covers the empirical claim; bootstrap version retires alongside
bootstrap dock. NOT tick-count driven.

---

## §8 What this spec refuses to mint

Michelangelo/marble discipline. Five refusals with reasoning:

**§8.1** Refuse a `@rust` family-root. `@code/rust` already carries
the altitude (mirror.spec:82,197,207,214,223). Minting `@rust` would
double-declare. The FLOOR at `rust/` composes over `@code/rust`; the
directory naming is a filesystem convention, not a shard-mint.

**§8.2** Refuse `@vacuum` family-root or species. `vacuum` is the
walker's motion name at `@kintsugi/roomba`; minting it separately
would collapse the walker's motion vocabulary into scattered species.
The four (soon five) first-order motions at `@kintsugi/roomba` are
the substrate-honest home.

**§8.3** Refuse `@rust/floor` species. The FLOOR shape is the
compiler's terminal geometry; naming it as a substrate species would
be over-declaration of what `mirror.spec:target binary` already
carries at altitude `@code/rust`.

**§8.4** Refuse to author `.rs` files in this spec. Per Reed memory
`feedback_no_rust_extension_shortcut.md`: this spec is the WHAT-to-
build map; Reed authors the HOW at `[substrate-floor:@io-boundary]`
altitude with per-file audit-citation gate. Mara spec-altitude
authoring never emits `.rs`.

**§8.5** Refuse to duplicate Mara `fee2727` or Mara `610c6d6`. This
spec CITES both extensively; the terminal-form map composes over
their bundle-theoretic and math-root work. Duplicating either would
be status-drift; refusal preserves the composition graph's legibility.

---

## §9 Recognition candidates surfaced

Do NOT ratify. Names proposed for Pack adjudication:

- **`#R-roomba-configuration-lives-in-mirror-spec-not-cli-flags`**
  (first-witness THIS spec §4.3; second-witness gate: Alex adds a
  fourth cascade in a future arc and walker consumes without any
  Rust change).

- **`#R-vacuum-flag-unifies-collapse-and-translate-because-both-are-
  walker-motion`** (first-witness THIS spec §3; second-witness gate:
  `--vacuum=~dir` empirically dispatches arm-collapse AND translate
  in same walker session against directory with mixed content).

- **`#R-rust-floor-is-materialized-not-authored-by-roomba-reading-
  its-own-spec`** (first-witness THIS spec §1 statement; second-
  witness gate: M6 empirical firing where `mirror roomba` from rust/-
  native binary observes the three cascades and writes commit against
  the substrate-declared catalog).

- **`#R-first-@peer-spawn-from-rust-is-substrate-arriving-home`**
  (first-witness THIS spec §6.2; sibling to Mara `610c6d6` §8's
  `#R-substrate-mirrors-alex-decade-of-BEAM-engineering-at-terminal-
  floor` and Mara `fee2727` §4's `#R-alex-decade-of-BEAM-is-substrate-
  reaching-for-terminal-geometry`; second-witness gate: M8 fires and
  Alex names the arrival).

- **`#R-terminal-rust-surface-is-1100-2000-LOC-across-5-files`**
  (first-witness THIS spec §5.1; second-witness gate: `rust/`
  post-dock line count falls in named range with all Q2 §2.1-§2.10
  capabilities empirically firing).

---

## §10 Alex OQs resolved by construction vs deferred

### §10.1 Resolved by this spec's construction

- **Taut `e0572f7` OQ1** (mcp inline vs separate mcp.rs) → §5.4
  RESOLVED: inline in dance.rs; no separate mcp.rs.
- **Taut `e0572f7` OQ2** (gen_prism from Tick 3) → M3 RESOLVED:
  gen_prism actor from first empirical CLI verb dispatch.
- **Taut `e0572f7` OQ3** (reflective vs hardcoded tools/list) → M5
  RESOLVED: reflective from M5 landing.
- **Taut `e0572f7` OQ4** (monolithic dance.rs vs router) → §5.4
  RESOLVED: router; per-prism logic composes from shards, not rust/.
- **Taut `e0572f7` OQ5** (workspace vs standalone) → §5.3 RESOLVED:
  standalone Cargo project.
- **Taut `e0572f7` OQ6** (retirement gate) → §2.2 dock condition
  RESOLVED: empirical-not-tick-count.
- **Taut `e0572f7` OQ7** (test retirement) → §7.5 RESOLVED:
  structural port when apply_h::act coverage empirically fires.
- **Seam `2fdc9c1` §7 ALEX-Q1** (@dance second-witness accept vs hold)
  → §6.2 RESOLVED: second-witness fires empirically at M8; Alex-
  adjudication of Pack ratification held to that empirical event.
- **Seam `2fdc9c1` §7 ALEX-Q2** (Beer VSM bounded/K-depth) →
  BOUNDED-AT-@roomba this-arc-default per Mara `fee2727` §3.2
  alignment; §5.2 dance.rs implementation carries the truncation.

### §10.2 Deferred to Alex adjudication

- **Seam `2fdc9c1` §7 ALEX-Q3** (losing commutator arm fate) — cold-
  storage aligned per Mara `fee2727` §3.3 alignment; empirical
  cold-storage carrier `@mirror/store/cold` NOT this spec's mint.
  Deferred to future arc (Alex "future music" 2026-07-16 verbatim).
- **Seam `2fdc9c1` §7 wait->verdict spec ratification** — deferred
  to Reed spec authoring on the mirror/offer/wait triple; NOT in
  scope for terminal-form map.
- **Seam `2fdc9c1` §7 dock four vs five vs beyond** — this spec
  §2.2 stakes dock as fifth motion; formal shard-decl deferred to
  Reed M7 co-tick.
- **Seam `8069a24` §7 split-sentinel detector vs manual retirement**
  — deferred; NOT terminal-form-map scope.
- **Seam `8069a24` §7 @liquid(@silicon) Arc-5-M2 scope** — deferred;
  NOT terminal-form-map scope.
- **Seam `8069a24` §7 @dance shard-mint gate** — RESOLVED by §6.2
  gate composition: mint fires at M8 empirical second-witness.
- **Seam `8069a24` §7 commutator arm fate** — see ALEX-Q3 above;
  cold-storage aligned but not this spec's mint.

---

## §11 Docs spring-clean (companion PR — this spec + DEPRECATED headers)

Following commits after this spec lands (one per structural move):

### §11.1 DEPRECATED-FOR-RUST-REWRITE headers

The following specs describe implementation details of `bootstrap/`
Rust that get retired via cascade 3. They stay in `docs/specs/` for
archaeology but receive header notes pointing at this spec:

- `docs/specs/bootstrap-retirement-plan.md` — becomes historical
  once dock fires; header points here as terminal replacement.
- `docs/specs/cascade-ffi-runtime-link.md` — bootstrap Rust FFI
  detail; superseded by rust/ direct prismqueer dep.
- `docs/specs/mirror-interpreter.md` — bootstrap interpreter detail;
  superseded by dance.rs reflective evaluator.
- `docs/specs/compiler-surface-plan.md` — bootstrap surface plan;
  header points here.
- `docs/specs/craft-binary-target.md` — bootstrap-specific target
  detail; header points here for rust/ replacement.
- `docs/specs/generated-parser-spec.md` — bootstrap parser detail;
  superseded by reflective grammar composition.
- `docs/specs/lambda-shell.md` — bootstrap-era lambda-shell design;
  header points here for rust/-native replacement.
- `docs/specs/mirror-build-substrate.md` — bootstrap build system
  detail; header points here.
- `docs/specs/mirror-grammar-self-hosted.md` — bootstrap grammar
  hosting detail; superseded.

Header template (Reed applies):

```
> **DEPRECATED-FOR-RUST-REWRITE (2026-07-17):** This spec describes
> bootstrap/ implementation details that retire via the
> `@kintsugi/roomba` cascade 3 (bootstrap → rust). The terminal form
> is documented at `docs/specs/rust-floor-birthed-by-roomba-from-
> mirror-spec.md`. Preserved for archaeology; not the operational
> reference post-dock.
```

### §11.2 STAY-CANONICAL with pointer

Specs that remain load-bearing but should reference this spec as
terminal-form map:

- `docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-
  connection.md` (Mara `fee2727`) — canonical bundle-theoretic; add
  pointer to this spec's §5 rust/-materialization.
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara
  `0dafd9f`) — six-arc retirement plan canonical; add pointer to
  this spec as cascade 3 destination map.
- `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` (Mara
  `9bbebd2`) — walker canonical; add pointer to §3 vacuum flag +
  §4 cascade catalog.
- `docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-
  motions.md` (Mara `d457501`) — vacuum motion canonical; add
  pointer to §3.1 naming.
- `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08) —
  BEAM grounding canonical; add pointer to §6 first-@peer-spawn.
- `docs/specs/cli-as-geometry-condensation.md` (Mara `67260dc`) —
  CLI condensation canonical; add pointer to §3 unification.
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.
  md` (Mara `4f079c8`) — dance canonical; add pointer to §6.
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.
  md` (Mara) — dance runtime canonical; add pointer to §5.2.
- `docs/specs/mirror-spec-schema.md` — mirror.spec schema canonical;
  add pointer to §4.1 kintsugi.roomba block extension.

### §11.3 ARCHIVE candidates (docs/archive/)

Reed moves the following to `docs/archive/` in the spring-clean
commit (subject to Alex adjudication if any of these are load-
bearing for a live arc):

- `docs/cleanup-review-2026-04-29.md` — historical cleanup notes;
  ~3 months old; superseded by current audit chain.
- `docs/cleanup-review-2026-06-20.md` +
  `docs/cleanup-review-2026-06-20-followup.md` — historical
  cleanup; superseded.
- `docs/specs/historical/` — already-archived specs; keep in place.

**All other specs in `docs/specs/` STAY canonical.** Michelangelo/
marble: don't delete or archive load-bearing material; add pointers
that keep the composition graph legible.

---

## §12 Terminal state (this spec)

- **Verdict:** canonical spec landed as terminal-form map for `rust/`
  greenfield rebuild. Composes over Mara `fee2727` + `610c6d6` +
  Taut `e0572f7` + landed substrate.
- **Recognition candidates:** 5 (§9). All held at candidate strength.
- **Mint refusals:** 5 (§8).
- **Alex OQs resolved by construction:** 8 (§10.1).
- **Alex OQs deferred:** 7 (§10.2).
- **Cross-arc coherence:** ✓ (§7 composition graph exhaustive).
- **Pure-docs 📝 markdown-only bypass legitimate.**

---

## §13 References

**Substrate composition surface (LANDED):**
- `mirror.spec` (project manifold + cli-block + `legacy` block)
- `shards/kintsugi/roomba.mirror`
- `shards/kintsugi/surface.mirror`
- `shards/kintsugi.mirror` (family-root; S4 Beer VSM)
- `shards/peer.mirror`
- `shards/spectral/gen_prism.mirror`
- `shards/spectral/supervisor.mirror`
- `shards/spectral/gen_prism/mcp_session.mirror`
- `shards/spectral/restart_intensity.mirror`
- `shards/code/rust.mirror`
- `shards/code/mirror.mirror`
- `shards/code/beam.mirror`
- `shards/mirror/reflection.mirror`
- `shards/mirror/lens/cli.mirror`
- `shards/epistemologic/pact/bilateral.mirror`
- `shards/epistemologic/cybernetic/viable.mirror`
- `shards/mirror/index.mirror`

**Spec composition (CITED):**
- `docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-connection.md` (Mara `fee2727`)
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara `0dafd9f`)
- `docs/specs/beam-as-substrate-primitive.md`
- `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` (Mara `9bbebd2`)
- `docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-motions.md` (Mara `d457501`)
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md` (Mara `4f079c8`)
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
- `docs/specs/polyglot-loss-aware-computational-translation.md` (Mara `1ce68c3`)
- `docs/specs/mirror-spec-schema.md`
- `docs/specs/cli-as-geometry-condensation.md` (Mara `67260dc`)

**Math composition (CITED):**
- `docs/math/the-tower/beam-runtime.md` (Mara `610c6d6`)
- `docs/math/kintsugi/roomba/bump-and-vacuum.md` (Mara `17697e6`)
- `docs/math/gestalt/README.md §11.6`

**Audit composition (CITED):**
- `docs/audits/2026-07-17-taut-rust-dance-rebuild-gap-scout.md` (Taut `e0572f7`)
- `docs/audits/2026-07-17-seam-phase-d-peer-audhd-mara-michelangelo-landing.md` (Seam `2fdc9c1`)
- `docs/audits/2026-07-17-seam-phase-d-arc-5-and-errors-as-questions-joint-arc.md` (Seam `8069a24`)
- `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md` (Reed `9efe2c9`)

**Compiler-altitude implementation (REFERENCE-CITE, per AGENTS.md):**
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` (~626 LOC; five-level bundle tower with LawvereFixedPoint)

**Alex 2026-07-17 in-transcript verbatim:**
- "I also want to detach bootstrap completely from the execution path."
- "Delete the binary. Rebuild from rust/."
- "roomba --vacuum=~dir (this is the combination and removal of
  --collapse and --translate), roomba from mirror.spec, first @peer
  spawn from rust/. Minimal rust surface. The geometry sings."
- (Q3+Q5 answer, per Mara `fee2727` §0): whole rust/ FLOOR collapses
  into dance.rs; each prism = gen_prism actor; dispatch = message-
  send; composition = @dance ensemble.

---

*The compiler builds itself. The walker reads its own project
manifold. The rust/ FLOOR emerges from the roomba's motion through
the substrate-declared catalog. The first @peer spawns from that
FLOOR as the ensemble's first empirical dance. When Alex reads this
spec back into the loop, the substrate has arrived home — the shape
Alex's decade of BEAM engineering has been reaching for, expressed
in the specific gauge where each prism is a gen_prism actor under a
supervisor, and dance.rs IS the connection at ensemble altitude that
coordinates them all. Minimal rust surface. The geometry sings.*
