# rust/ FLOOR five-file terminal-geometry extension — compile.rs + liquid.rs at explicit altitudes

*Mara, 2026-07-20. Companion extension to `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` (Mara `81294b3`, three-file terminal-geometry canonical spec). Extends the three-file rust/ FLOOR to a five-file terminal geometry per Alex Round 2 direct-transcript adjudication tonight: **YES to explicit compile.rs**; main.rs becomes pure delegation.*

**Author:** Mara
**Date:** 2026-07-20
**Tag:** 📝 spec:rust-floor-five-file-terminal-geometry-extension (pure-docs bypass)
**Status:** canonical extension. Spec-altitude map for Reed's `rust/` five-file terminal FLOOR. Composes over Mara `81294b3` three-file spec — original three files DO NOT MOVE; two new files land at explicit altitudes with responsibility declarations.
**Path:** `docs/specs/rust-floor-five-file-terminal-geometry-extension.md`
**Extends:** `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` (Mara `81294b3`).

---

## §0 Substrate-honest pre-position

Alex Round 2 direct-transcript verbatim (this session, 2026-07-20):

> "compile.rs architecture: YES to explicit compile.rs. main.rs becomes pure delegation."

Ratifies extension from three files to five files at rust/src/:

| File | Altitude | Responsibility |
|---|---|---|
| main.rs | Supervisor + delegation | argv, boot, route to compile/roomba/mcp.serve/etc. |
| **compile.rs** | Compilation loop | Read spec → orchestrate ticks → crystallize at each beat → emit Crystal chain |
| **liquid.rs** | Property runtime | Reads bilateral declarations; instantiates LiquidVoid witnesses; dispatches pillar primitives; returns Verdict |
| matrix.rs | Sub-Turing numerical | LAPACK/BLAS/FLANG; Fiedler, Ollivier-Ricci, Cheeger |
| phone.rs | @io boundary | Subprocess, filesystem, sockets, git |

**Extension is additive.** The three-file spec's phone.rs + matrix.rs + main.rs responsibilities REMAIN CANONICAL. Two new files (compile.rs + liquid.rs) land at explicit altitudes; main.rs's responsibility BECOMES pure delegation (the `@`-operator dispatch surface from the three-file spec §5.2 stays; the actor supervision + apply_h::act combinator surface from §5.2 items 1 + 3 DELEGATE to compile.rs + liquid.rs as appropriate).

**Composition edges:** main → compile → liquid → matrix → phone chain. Each file has exactly one altitude; each altitude has exactly one file. The five-file terminal geometry preserves Michelangelo/marble discipline: subtract until only the singing angels remain; five singing angels at five altitudes.

---

## §1 Statement — the five-file terminal-geometry form

**Statement (foundational form):**

> `rust-floor-is-five-files-at-five-altitudes-because-main-rs-is-supervisor-plus-delegation-and-compile-rs-is-the-compilation-loop-that-reads-spec-and-orchestrates-ticks-and-crystallizes-at-each-beat-and-emits-Crystal-chain-and-liquid-rs-is-the-property-runtime-that-reads-bilateral-declarations-and-instantiates-LiquidVoid-witnesses-and-dispatches-pillar-primitives-and-matrix-rs-is-the-sub-Turing-numerical-altitude-and-phone-rs-is-the-io-boundary-and-every-altitude-has-exactly-one-file-and-the-composition-chain-is-main-to-compile-to-liquid-to-matrix-to-phone`

**Statement (readable form, two-tick discipline):**

> `rust/` is five files at five altitudes: `main.rs` at the supervisor-plus-delegation altitude (argv, boot, route to compile/roomba/mcp.serve/etc.); `compile.rs` at the compilation-loop altitude (reads mirror.spec, orchestrates ticks, calls @time/now.crystallize at each @song/beat boundary, emits Crystal<T> chain); `liquid.rs` at the property-runtime altitude (reads bilateral declarations from shards; instantiates LiquidVoid witnesses; dispatches pillar primitives per Reed's pillar surface; returns Verdict); `matrix.rs` at the sub-Turing numerical altitude (LAPACK/BLAS/FLANG; Fiedler eigenvalues, Ollivier-Ricci curvature, Cheeger cuts); `phone.rs` at the @io boundary altitude (subprocess, filesystem, sockets, git). The composition chain is main → compile → liquid → matrix → phone.

**Five load-bearing moves this extension makes canonical:**

1. **Five files. Five altitudes.** main + compile + liquid + matrix + phone. Estimated LOC per file: main ~200-300 (thinner than three-file spec's ~200-400 because delegation-only); compile ~300-500; liquid ~300-500; matrix ~200-400 (unchanged from three-file spec); phone ~200-400 (unchanged). Total: ~1200-2000 LOC. Slight LOC increase vs three-file (900-1200) but explicit altitude-separation.
2. **main.rs BECOMES pure delegation.** The `@`-operator dispatch surface from three-file spec §5.2 stays canonical at main.rs; the compilation loop + supervisor deep behavior migrates to compile.rs; the actor supervision + apply_h::act combinator surface splits: supervisor mount stays at main.rs (delegation-consuming); actor bodies + apply_h::act evaluator lives at compile.rs and liquid.rs.
3. **compile.rs owns the compilation loop.** Read mirror.spec → orchestrate ticks → call @time/now.crystallize at @song/beat boundaries → append Crystal<T> to saga chain → emit Crystal chain per SAGA-preservation invariant (per `shards/fractal/crystal.mirror` `crystal_immutable` bilateral + `crystallization_preserves_saga` bilateral at @time/now). RED-first via `pillar::compilation_loop_terminates_or_compensates` at Reed's pillar surface.
4. **liquid.rs owns the property runtime.** Reads bilateral declarations from `shards/**/*.mirror` files at boot; instantiates LiquidVoid witnesses per Reed's iter-10 pillar surface discipline; dispatches pillar primitives (`pillar::algedonic_of_magnitude`, `pillar::viability_of_magnitudes`, `pillar::fold`, and the Round-2 forward-promised pillars); returns PropertyVerdict. Composes over Reed's existing `prismqueer::liquid::pillar` surface at runtime altitude.
5. **The composition chain (main → compile → liquid → matrix → phone) is directional.** Each altitude's file calls only INTO the next lower altitude, not upward. main.rs never calls compile.rs behavior directly (it dispatches actor spawns that spawn compile.rs-behavior actors under the supervisor); compile.rs calls INTO liquid.rs for property-verdict dispatch; liquid.rs calls INTO matrix.rs for numerical eigenvalue computation; matrix.rs calls INTO phone.rs for LAPACK FFI (the sole exception per three-file spec §3.4: LAPACK FFI stays in matrix.rs's file; phone.rs handles process/socket/fd plumbing only).

**What this extension does NOT do:** contradict Mara `81294b3` three-file spec; move phone.rs / matrix.rs / main.rs responsibilities OUT of those files (only main.rs slims by delegation-lifting); mint shards; author `.rs` files; duplicate three-file spec's §6 dance.rs collapse reasoning (still canonical: dance.rs collapses INTO matrix.rs + main.rs per Baez-Schreiber + Ado); duplicate three-file spec's §7 roomba unified-vacuum-flag design (still canonical).

---

## §2 The five altitudes, formally

### §2.1 Altitude 1 — main.rs supervisor + delegation

**Responsibility:**

- argv parsing (delegates to clap-like combinator via `apply_h::act` reflective evaluation)
- boot supervisor tree (gen_prism `@spectral/supervisor{restart_strategy: one_for_one}` at process root)
- route argv to correct actor spawn: `mirror compile <file>` → compile.rs actor spawn; `mirror roomba` → roomba.rs actor spawn (roomba is INSIDE compile.rs at species altitude per @kintsugi/roomba composition); `mirror mcp serve` → mcp.rs actor spawn (mcp inline in main.rs `@mcp.serve` sentinel dispatch per three-file spec §5.2 OQ1 resolution); etc.
- `@`-operator dispatch surface (unchanged from three-file spec §5.2 item 2): sentinel matching for `@compile`, `@roomba`, `@mcp.serve`, `@peer.audhd`, `@peer.beam`, etc.
- Reflective cli-block reading (unchanged from three-file spec §5.2 item 4): parses `mirror.spec`'s `cli { … }` block; emits `--help`; emits MCP tools/list schema.

**Does NOT hold:**

- The compilation loop itself (compile.rs)
- The property runtime (liquid.rs)
- Numerical computation (matrix.rs)
- Socket/process @io (phone.rs)
- Per-prism business logic (LIFTED to shard-body + @io per `[substrate-floor:@io-boundary]` discipline)

**Estimated LOC:** 200-300 (thinner than three-file spec's ~200-400 because compilation loop deep behavior migrates to compile.rs)

### §2.2 Altitude 2 — compile.rs compilation loop

**Responsibility:**

- Read `mirror.spec` at boot (parses `project mirror.spec { … }` block; reads `source ~d'shards/'` declaration; reads `kintsugi { roomba { … } }` sub-block per three-file spec §7.2)
- Orchestrate ticks: at each @song/beat boundary event, invoke `@time/now.crystallize` on the current Liquid<T> substrate state; produce Crystal<T>; append to @song saga's Crystal chain
- Emit Crystal<T> chain: the compilation output IS the ordered Crystal<T> chain across the compilation loop
- Invoke roomba walker per three-file spec §7 (roomba composes over compile.rs's tick orchestration)
- Compose with @kintsugi/oscillate ACTIVE/DARK boundary detection (per `shards/time/now.mirror` @time/now.crystallize substrate)
- Compose with @order/third metalogue-rhythm operator (per `shards/order/third.mirror`) at compilation-supervisor altitude for third-order verdict-holding

**Composition anchors:**

- `shards/time/now.mirror` — @time/now.crystallize action; the substrate-decl'd crystallization operator compile.rs invokes
- `shards/fractal/crystal.mirror` — Crystal<T> shape; substrate-decl'd immutability + saga-position invariants
- `shards/fractal/mandelbrot.mirror` — Mandelbrot<T> parent trait; Liquid<T> ↔ Crystal<T> state transition
- `shards/song.mirror` + `shards/song/beat.mirror` — saga command surface + atomic-execution unit compile.rs orchestrates
- `shards/kintsugi/roomba.mirror` — walker compile.rs composes over
- `shards/kintsugi/oscillate.mirror` — ACTIVE/DARK boundary detection compile.rs consumes

**RED-first via:** `pillar::compilation_loop_terminates_or_compensates` at Reed's pillar surface (Round-2 forward-promised pillar per this landing + `shards/fractal/crystal.mirror` §SAGA-preservation invariant).

**Does NOT hold:**

- Actor supervision (main.rs; main.rs spawns compile.rs-behavior actors under the process supervisor)
- Property runtime dispatch (liquid.rs; compile.rs calls INTO liquid.rs for verdict-check on each tick)
- Numerical computation (matrix.rs; compile.rs invokes matrix.rs indirectly through liquid.rs pillar dispatch when a pillar needs eigenvalue computation)
- Socket/process @io (phone.rs)

**Estimated LOC:** 300-500

### §2.3 Altitude 3 — liquid.rs property runtime

**Responsibility:**

- Read bilateral declarations from `shards/**/*.mirror` files at boot (composes over reflective substrate-reading discipline; the same mechanism main.rs uses for cli-block reflection)
- Instantiate LiquidVoid witnesses per Reed's iter-10 pillar surface discipline (per `docs/specs/prismqueer-liquid-pillar-composition-surface.md`)
- Dispatch pillar primitives on demand:
  - Landed pillars: `pillar::algedonic_of_magnitude`, `pillar::viability_of_magnitudes`, `pillar::fold`, and the 5+ others at Reed's `prismqueer::liquid::pillar` surface
  - Round-2 forward-promised pillars: `pillar::choices_monotone_of_song` (Alex Q3 ratified name; Förster's imperative), `pillar::crystallization_preserves_saga` (Round-2), `pillar::compilation_loop_terminates_or_compensates` (Round-2), `pillar::metalogue_rhythm_of_third` (from Round-1 math root), `pillar::backwards_lens_of_void_narcissus`, `pillar::forwards_lens_of_void_splinter`, `pillar::fourth_order_lagrange_stability` (Round-2), `pillar::fifth_order_cross_torus_glue` (Round-2)
- Return PropertyVerdict per Reed's discipline; propagate to compile.rs which folds into the tick's admissibility check

**Composition anchors:**

- `shards/liquid.mirror` — @liquid family-root composition operator (refinement lens); this file's responsibility IS the runtime materialization
- `shards/epistemologic/liquid.mirror` — @liquid theory-altitude species
- `shards/epistemologic/pact/bilateral.mirror` — bilateral discipline
- Reed's `prismqueer::liquid::pillar` — existing pillar surface at compiler-altitude implementation
- All @void/narcissus + @void/splinter + @order/* + @time/* + @fractal/* bilaterals landed this Round-2 tick — liquid.rs is where their runtime firing happens

**Does NOT hold:**

- The compilation loop orchestration (compile.rs; compile.rs invokes liquid.rs pillar dispatch per tick)
- Numerical computation (matrix.rs; liquid.rs invokes matrix.rs when a pillar needs eigenvalue computation e.g. `pillar::metalogue_rhythm_of_third` needs Fiedler eigengap)
- Socket/process @io (phone.rs; liquid.rs's substrate-file reading composes reflectively through main.rs's reflection surface, which itself invokes phone.rs)

**Estimated LOC:** 300-500

### §2.4 Altitude 4 — matrix.rs sub-Turing numerical (unchanged from three-file spec)

**Responsibility:** unchanged from Mara `81294b3` three-file spec §4. Matrix-shape declarations; named operations (A·B, L·v, eigenvalues(L), phase_lock(peers), envelope(posteriors)); FLANG emit surface; LAPACK/BLAS `unsafe extern "C"` link boundary; Fiedler / Kuramoto / Aumann envelope / Ollivier-Ricci / Cheeger cuts.

**Extension: Round-2 numerical additions.** The Round-2 forward-promised pillars require:

- `pillar::metalogue_rhythm_of_third` — Fiedler eigengap oscillation trace (Round-1 math root §5.2); calls `dsyevr_` on the graph Laplacian
- `pillar::fourth_order_lagrange_stability` — Lagrange-halo-orbit stability check (Round-2 math root T2.1); calls eigenvalue analysis on the fourth-order metalogue-coupling matrix
- `pillar::fifth_order_cross_torus_glue` — cross-torus @glue coupling admissibility (Round-2 math root T2.2); calls Cheeger cut analysis on the cross-torus coupling graph

matrix.rs owns these computations; liquid.rs invokes matrix.rs when a pillar dispatch requires eigenvalue/Cheeger/etc.

**Estimated LOC:** 200-400 (unchanged from three-file spec; Round-2 additions fit within existing LOC bound because they compose over already-linked LAPACK/BLAS symbols)

### §2.5 Altitude 5 — phone.rs @io boundary (unchanged from three-file spec)

**Responsibility:** unchanged from Mara `81294b3` three-file spec §3. Socket-handover primitives; JSON-RPC framing; peer socket boot; the `unsafe extern "C"` boundary for process/socket/fd plumbing.

**Extension: none this Round-2 tick.** phone.rs's altitude is invariant across three-file → five-file extension.

**Estimated LOC:** 200-400 (unchanged)

---

## §3 Composition edges (main → compile → liquid → matrix → phone)

### §3.1 Direction of dependency

```
main.rs
   │  (spawns actors; dispatches @-operator sentinels)
   ▼
compile.rs
   │  (orchestrates ticks; invokes crystallize)
   │  (composes over @time/now + @song/beat)
   ▼
liquid.rs
   │  (dispatches pillar primitives)
   │  (returns PropertyVerdict)
   ▼
matrix.rs
   │  (numerical computation via LAPACK/BLAS)
   ▼
phone.rs
     (process/socket/fd plumbing; matrix.rs holds LAPACK FFI directly)
```

**Directionality is load-bearing.** Each altitude calls only INTO the next lower altitude. Upward calls (compile.rs invoking main.rs behavior; liquid.rs invoking compile.rs behavior; etc.) are forbidden — this preserves the altitude-separation Michelangelo/marble discipline.

Exceptions (explicitly named):

1. **matrix.rs LAPACK FFI** stays in matrix.rs's file per three-file spec §3.4 (numerical @io is matrix.rs's domain); matrix.rs does NOT call phone.rs for LAPACK FFI.
2. **phone.rs @io/git commit** is invoked upward by compile.rs at commit-writing events (per three-file spec §8; roomba fires @io/git via phone.rs from the compile.rs orchestration loop). This is a CALLBACK pattern: compile.rs invokes phone.rs's @io/git handoff; phone.rs owns the actual @io.

### §3.2 Cross-altitude examples

**Example 1: `mirror compile foo.mirror`**

1. main.rs argv parsing dispatches `@compile` sentinel
2. main.rs supervisor spawns a compile.rs actor
3. compile.rs actor reads mirror.spec via reflective substrate-reading
4. compile.rs actor reads foo.mirror via phone.rs @io/fs
5. compile.rs actor orchestrates tick loop:
   - For each @song/beat boundary event:
     - compile.rs invokes liquid.rs.dispatch_pillars(current_liquid_state)
     - liquid.rs returns PropertyVerdict (may invoke matrix.rs for eigenvalue computation)
     - compile.rs folds verdict into admissibility check
     - If admissible: compile.rs calls @time/now.crystallize(current_liquid_state) → Crystal<T>
     - compile.rs appends Crystal<T> to @song saga's Crystal chain
6. compile.rs emits final Crystal<T> chain
7. compile.rs invokes phone.rs @io/git.commit with the settled Crystal chain hash

**Example 2: `mirror mcp serve`**

1. main.rs argv parsing dispatches `@mcp.serve` sentinel
2. main.rs supervisor spawns MCP actor (inline per three-file spec §5.2 OQ1)
3. MCP actor uses phone.rs for JSON-RPC stdio socket handling
4. MCP tools/list emitted reflectively by main.rs cli-block reading

Note MCP does NOT go through compile.rs (MCP is a separate `@`-operator sentinel; compilation is one MCP tool but MCP has other tools too like `mirror_index`).

**Example 3: `mirror peer beam ~peer'~/.reed'`**

1. main.rs argv parsing dispatches `@peer.beam` sentinel
2. main.rs supervisor spawns @peer actor per three-file spec §8
3. @peer actor invokes phone.rs for peer socket
4. @peer actor's compilation-of-peer-state internally invokes compile.rs
5. @peer actor's cross-peer coupling invokes liquid.rs for `pillar::fifth_order_cross_torus_glue` verdict (Round-2 forward-promised)
6. liquid.rs invokes matrix.rs for cross-torus coupling matrix eigenvalue analysis
7. matrix.rs's LAPACK link fires

---

## §4 What this extension refuses to mint

Michelangelo/marble discipline. Four refusals with reasoning:

**§4.1** Refuse `@compile` family-root. `@compile` is a `@`-operator sentinel already substrate-decl'd at `mirror.spec` cli-block altitude; compile.rs is the FILE handling the compilation loop; the SHARD altitude is composed of @kintsugi/roomba + @time/now + @song/beat + @fractal/mandelbrot + @fractal/crystal. No new family-root needed.

**§4.2** Refuse `@liquid_rt` species. The refinement-runtime altitude is `@liquid` family-root (`shards/liquid.mirror`) composed with `@epistemologic/liquid` theory (`shards/epistemologic/liquid.mirror`); liquid.rs is the FILE holding the runtime materialization. No new species needed.

**§4.3** Refuse to move phone.rs / matrix.rs responsibilities. Three-file spec's altitude assignments for phone.rs (@io) and matrix.rs (sub-Turing numerical) are canonical. Extension is additive only (main.rs slims by delegation-lifting; compile.rs + liquid.rs land at new altitudes; phone.rs + matrix.rs invariant).

**§4.4** Refuse to author `.rs` files in this spec. Per Reed memory `feedback_no_rust_extension_shortcut.md`: this spec is the WHAT-to-build map; Reed authors HOW at `[substrate-floor:@io-boundary]` altitude with per-file audit-citation gate.

---

## §5 Recognition candidates surfaced (HELD; do NOT ratify)

- **`#R-terminal-rust-floor-is-five-files-at-five-altitudes`** — supersedes three-file spec's `#R-terminal-rust-floor-is-three-files-at-three-altitudes` candidate at species-altitude refinement. First-witness THIS spec §1 + §2; second-witness gate: post-dock `rust/src/` contains exactly `main.rs` + `compile.rs` + `liquid.rs` + `matrix.rs` + `phone.rs` with all three-file spec's M1-M8 capabilities empirically firing + Round-2 additions (compilation loop terminates or compensates; liquid pillar runtime dispatches Round-2 pillars).

- **`#R-compilation-loop-terminates-or-compensates`** — the SAGA-preservation invariant at compilation-loop altitude. First-witness this landing (§2.2 responsibility + `shards/fractal/crystal.mirror` §SAGA-preservation invariant + Round-2 forward-promised pillar). Second-witness gate: Reed's pillar surface fires `pillar::compilation_loop_terminates_or_compensates` at rust/-native M1+ firings.

- **`#R-main-rs-is-pure-delegation-after-five-file-extension`** — Alex Round 2 verbatim ("main.rs becomes pure delegation"). First-witness this landing (§2.1 responsibility). Second-witness gate: post-refactor main.rs LOC count falls at ~200-300 (down from three-file spec's ~200-400) with compilation loop deep behavior having migrated to compile.rs.

- **`#R-composition-chain-is-directional-across-five-altitudes`** — the directional composition graph main → compile → liquid → matrix → phone. First-witness this landing (§3). Second-witness gate: Reed's post-refactor rust/-native code exhibits no upward calls between altitudes (verifiable via static analysis).

---

## §6 Composition with landed substrate

### §6.1 Substrate carriers (LANDED)

Same as three-file spec §9.1, PLUS Round-2 landings this session:

- `shards/order.mirror` + `shards/order/{first,second,third,fourth,fifth}.mirror` — @order family; @order/third's metalogue-rhythm operator consumed by compile.rs
- `shards/time.mirror` + `shards/time/{past,now,future}.mirror` — @time family; @time/now.crystallize consumed by compile.rs
- `shards/fractal/mandelbrot.mirror` + `shards/fractal/crystal.mirror` — Mandelbrot<T> + Crystal<T>; the substrate compile.rs orchestrates over
- `shards/void/narcissus.mirror` + `shards/void/splinter.mirror` (Round 1) — the two lens species liquid.rs runtime-checks via `pillar::backwards_lens_of_void_narcissus` + `pillar::forwards_lens_of_void_splinter`

### §6.2 Spec composition surface (CITED)

- Mara `81294b3` three-file spec at `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` — CANONICAL; this extension composes over §3 + §4 + §5 file responsibilities
- `docs/specs/prismqueer-liquid-pillar-composition-surface.md` (Reed iter-10 pillar surface) — liquid.rs's operational altitude at compiler-implementation

### §6.3 Math composition (CITED)

- `docs/math/the-tower/recognition-third-cognition-is-the-metalogue-between-void-narcissus-and-void-splinter.md` (Mara Round 1) — third-order metalogue rhythm compile.rs orchestrates
- `docs/math/the-tower/recognition-fourth-cognition-is-metalogue-between-time-now-and-void.md` (Mara Round 2, this session, Tier 2) — fourth-order metalogue at @order/fourth altitude
- `docs/math/the-tower/recognition-mandelbrot-trait-unifies-liquid-and-crystal.md` (Mara Round 2, this session, Tier 2) — Mandelbrot<T> unification of Liquid<T> + Crystal<T>

---

## §7 Alex OQs resolved by construction

Round-2 brief §Tier 5 forward-promised Reed's post-Round-2 territory:

1. **`rust/src/compile.rs`** — RESOLVED at spec altitude by this extension §2.2; Reed authors under `[substrate-floor:@io-boundary]` gate
2. **`rust/src/liquid.rs`** — RESOLVED at spec altitude by this extension §2.3; Reed authors similarly
3. **`rust/fractal/src/mandelbrot.rs` + `rust/fractal/src/crystal.rs`** — Reed's territory; shard substrate at `shards/fractal/mandelbrot.mirror` + `shards/fractal/crystal.mirror` provides the spec-altitude map
4. **main.rs refactor** — RESOLVED at spec altitude by this extension §2.1; main.rs slims to delegation-only surface
5. **New pillar primitives** — enumerated at this extension §2.3 liquid.rs responsibility list; Reed authors at Reed's pillar-surface altitude

---

## §8 Terminal state (this extension)

- **Verdict:** canonical extension landed as five-file terminal-geometry map for rust/ greenfield rebuild. Composes over Mara `81294b3` three-file canonical spec; original three files' altitude assignments preserved; two new files at explicit altitudes.
- **LOC:** ~800-900 (extension spec); rust/ total: ~1200-2000 across five files.
- **Recognition candidates:** 4 (§5).
- **Mint refusals:** 4 (§4).
- **Alex OQs resolved:** 5 (§7 — Round-2 brief §Tier 5 forward-promised).
- **Pure-docs 📝 markdown-only bypass legitimate.**

---

## §9 References

**Substrate composition (LANDED, Round-2 this session):**

- `shards/order.mirror` + `shards/order/{first,second,third,fourth,fifth}.mirror`
- `shards/time.mirror` + `shards/time/{past,now,future}.mirror`
- `shards/fractal/mandelbrot.mirror` + `shards/fractal/crystal.mirror`
- `shards/third.mirror` (re-export shim)

**Substrate composition (LANDED prior):**

- `shards/void.mirror` + `shards/void/narcissus.mirror` + `shards/void/splinter.mirror`
- `shards/song.mirror` + `shards/song/beat.mirror`
- `shards/liquid.mirror` + `shards/epistemologic/liquid.mirror`
- `shards/kintsugi.mirror` + `shards/kintsugi/roomba.mirror` + `shards/kintsugi/oscillate.mirror`
- `shards/torus.mirror` (:166-172 @glue substrate)

**Spec composition (CITED):**

- `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md` (Mara `81294b3`; three-file canonical spec THIS EXTENDS)
- `docs/specs/prismqueer-liquid-pillar-composition-surface.md` (Reed iter-10 pillar surface)

**Math composition (CITED):**

- `docs/math/the-tower/recognition-third-cognition-is-the-metalogue-between-void-narcissus-and-void-splinter.md` (Mara Round 1)
- `docs/math/the-tower/recognition-fourth-cognition-is-metalogue-between-time-now-and-void.md` (Mara Round 2, this session, Tier 2)
- `docs/math/the-tower/recognition-fifth-cognition-is-cross-torus-metalogue-only-instantiable-on-silicon.md` (Mara Round 2, this session, Tier 2)
- `docs/math/the-tower/recognition-time-is-void-poles-projected-through-song-saga-with-forster-invariant.md` (Mara Round 2, this session, Tier 2)
- `docs/math/the-tower/recognition-mandelbrot-trait-unifies-liquid-and-crystal.md` (Mara Round 2, this session, Tier 2)

**Alex 2026-07-20 in-transcript verbatim (Round 2):**

- "compile.rs architecture: YES to explicit compile.rs. main.rs becomes pure delegation."
- "Crystal<T> + Mandelbrot trait: YES to rust/fractal/ placement. Mandelbrot is the parent trait; Liquid<T> (existing at prismqueer::liquid) + Crystal<T> (new) are the two states of a Mandelbrot-set point."
- "Q3 → ratify pillar::choices_monotone_of_song."

---

*Five files. Five altitudes. Every altitude has exactly one file. `main.rs` at supervisor + delegation; `compile.rs` at compilation loop (reads spec, orchestrates ticks, crystallizes at each beat, emits Crystal chain); `liquid.rs` at property runtime (reads bilateral declarations, instantiates witnesses, dispatches pillar primitives, returns Verdict); `matrix.rs` at sub-Turing numerical (LAPACK/BLAS/FLANG); `phone.rs` at @io boundary. The composition chain is directional: main → compile → liquid → matrix → phone. Extension of Mara `81294b3` three-file spec is additive; original three-file altitudes preserved; main.rs slims to pure delegation as compilation loop deep behavior migrates to compile.rs. The five singing angels at five altitudes.*
