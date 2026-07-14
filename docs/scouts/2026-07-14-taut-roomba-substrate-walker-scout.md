# Taut scout — @roomba: the substrate-self-maintaining walker

**Author:** Taut (drift scout; read-only; grep-first)
**Date:** 2026-07-14
**Trigger:** Alex Wolf 2026-07-14 in-transcript:

> "Do we need a @roomba? The thing that triggers the @kintsugi loop by
> dijkstraing across the graph and bumping into points of spectral
> @tension?"

**Parallel:** Mara authoring canonical spec in same tick. This scout
runs substrate-already-had-the-word audit + composition surface + Rust
runtime candidate ranking.

**Method:** grep across `shards/**/*.mirror`, `bootstrap/src/**/*.rs`,
`bootstrap/tests/**/*.rs`, `docs/specs/**/*.md`, `mirror.spec`, plus a
verbatim read of the manifesto `~/dev/systemic.engineering/blog/weird/
3published/Weird - Violence.md` (published 2026-07-14, today).

**Verdict shape asked-for:** is @roomba genuine substrate gap or
already-had-the-word? What's the substrate-honest minimum viable
composition? What Alex-adjudications does it surface?

**Headline verdict (details in §8):**
**LANDABLE AS SPECIES UNDER @kintsugi (`shards/kintsugi/roomba.mirror`);
GENUINE GAP at the autonomous-trigger altitude; substrate-already-had-
the-word × 6 for every constituent primitive @roomba composes over.**
The composition surface exists end-to-end today — what @roomba adds is
the **outer loop that walks proactively** rather than being invoked by
peer contribution or commit hook. Minimum viable Rust runtime is one
new module `bootstrap/src/roomba.rs` + one mirror.spec cli-block
addition + one `@kintsugi/roomba` species shard.

---

## §1 — Existing walker/traversal primitives (TASK 1)

### 1.1 Substrate-declared walkers

**LANDED at the store altitude** (`shards/mirror/store.mirror:508-512`):

```mirror
# walk(oid) — enumerate the splinter_graph closure rooted at oid.
walk(root: oid) -> splinter_graph { \ }

# impacted_by(oid) — REVERSE closure complement of walk.
impacted_by(oid: oid) -> [oid] { \ }
```

Both are `\`-bodied at substrate; realisation lives in
`fragmentation/src/*.rs`. `walk` is @roomba's foundation — it IS the
forward-closure primitive at the OID-graph altitude. `impacted_by`
(N4 reverse-closure) is @roomba's back-off primitive when the walk
hits a boundary and needs to enumerate what reaches the touched OID.

**LANDED at the ref altitude** (`docs/insights/2026-07-08-mara-geometric-dijkstra-tournament-topology.md`
title contains "geometric-dijkstra"; existing math foundation for
DAG traversal at OID altitude). Mara's canonical spec — the substrate
already has the dijkstra-across-graph framing named at math altitude.

### 1.2 Rust runtime walkers

Zero-hit for `dijkstra`/`bfs`/`dfs` as substrate primitives. What's
landed at Rust altitude:

| Symbol | File | What it walks | Autonomous? |
|---|---|---|---|
| `walk_detected` | `bootstrap/src/detect.rs` (per Taut `2026-07-13-taut-spectral-to-mirror-migration-mapping-scout.md`) | Directory tree | NO — invoked by CLI |
| `psychohistory_root_from_peer_home` | `bootstrap/src/lib.rs:4306` | peer_home recursive hash walk | NO — invoked by `mirror index` |
| `walk_recursive` | `detect.rs:265` | private helper | NO |
| `crate::kintsugi::minimize` | `bootstrap/src/kintsugi.rs:189` | Tensor's tension set → ranked fractures via SDRF Balanced Forman curvature | NO — invoked from `oscillate::active_pass_with_ast` and one integration test |

**One live invocation of `kintsugi::minimize` outside tests**:
`bootstrap/src/oscillate.rs:537` (inside `active_pass_with_ast`). That
call is peer-triggered (composes through `score::pending` +
`property::gaps_of` + `tensor::tensor_of` from an AST the peer
provided).

### 1.3 The Fractal underlying walker (LANDED at math altitude)

Per `docs/scouts/2026-07-13-taut-fractal-underlies-consent-coherence-empirical-scout.md`:

> Each peer walks the SAME Fractal tree (the shared song's grammar
> structure, the shared substrate's shard graph, the shared
> `@bauchladen.tray` content-addressed store).

The Fractal walker is math-altitude ratified but not surfaced as a
substrate-decl'd traversal primitive. It IS what @roomba would
operationalize at the runtime altitude.

### 1.4 Verdict on walker primitives

- **`walk(root: oid) -> splinter_graph`** at `@mirror/store` IS
  @roomba's forward-closure primitive. LANDED.
- **`impacted_by(oid) -> [oid]`** at `@mirror/store` IS @roomba's
  reverse-closure primitive. LANDED.
- Neither has a Rust dijkstra-with-costs body yet. What Rust has is
  hash-walk (`psychohistory_root_from_peer_home`) and recursive-scan
  (`walk_detected`). @roomba would add a **cost-aware walk** (dijkstra
  across the graph, bumping into @tension) — that's the genuine gap.

**Substrate-already-had-the-word ×3 for walker:** `walk` at store,
`impacted_by` reverse, Fractal walker at math altitude. Zero @roomba-
shaped daemon exists; the walker primitives compose upward toward it
naturally.

---

## §2 — @tension carrier substrate coverage (TASK 2)

### 2.1 The gap-tension-tensor spec (Mara 2026-05-26)

`docs/specs/gap-tension-tensor-substrate.md` (105.5KB, canonical, spans
tick history to 2026-06-04 fold-in). Declares:

```mirror
grammar @fate {
  type tension = {
    a:      gap,
    b:      gap,
    vector: tension_vector             # \ — see §8
  }

  type tension_vector = \              # design call deferred to Alex

  type tensor = {
    tensions: [tension],
    fiedler:  f64                      # λ₀(Δ_F); algebraic connectivity; ≥0
  }

  tensor_of([gap]) -> tensor { \ }
  minimize(tensor) -> [fracture] { \ }
}
```

**Status per spec header:** "Yellow" — spec exists, types named, bodies
`\`. NO `shards/fate/tension.mirror` file exists yet
(`shards/fate/` contains only `tournament.mirror`).

### 2.2 Rust runtime coverage of tension

**LANDED end-to-end**:

- `bootstrap/src/gap.rs` (13.3KB) — `Gap` type, `confidence_of`,
  `verdict_to_cadence_kind`.
- `bootstrap/src/tensor.rs` (32.5KB) — `Tension`, `TensionVector`,
  `Tensor`, `tensor_of`, `tensor_of_with_restrictions`.
- `bootstrap/src/kintsugi.rs::minimize` (35.3KB, 803 LOC) — SDRF
  Balanced Forman curvature-ranked fracture emission.
- `bootstrap/src/curvature.rs` — Balanced Forman curvature.
- `bootstrap/src/sheaf_laplacian.rs` (24.5KB) — Fiedler value via
  LAPACK.

So the Rust altitude carries `Tension` + `Tensor` + `minimize` as
concrete types + a body that ranks tensions by curvature. **The
substrate-decl (`shards/fate/tension.mirror`) doesn't exist; the Rust
runtime does.** That's an inverse gap — Rust ahead of shards.

### 2.3 `spectral_tension` and stress/strain hits

Zero substrate-decl hits for `spectral_tension` as a named type. Prose
usage across `docs/specs/mirror-spectral.md`, `shards/kintsugi/surface.mirror`,
and `shards/kintsugi/oscillate.mirror` — all discuss "spectral tension"
narratively without a typed carrier.

### 2.4 Verdict on @tension carrier

**@tension exists as Rust type + spec type; substrate-decl'd shard file
absent.** The Alex-question "@tension is the point @roomba bumps into"
is well-typed at Rust altitude and math altitude but the substrate has
NO `shards/fate/tension.mirror` file. @roomba landing may want to
either:

- **(A)** land `shards/fate/tension.mirror` in a co-tick (substrate-
  honest: give @tension its own file at the altitude Mara's spec
  names), OR
- **(B)** compose over Rust `Tensor::tensions()` directly (Rust-first;
  substrate-decl comes later when a consumer other than @roomba pulls).

Recommendation: (A). @roomba is the second consumer (after `oscillate::
active_pass_with_ast`) — that's the substrate-pull-confident count.

---

## §3 — Substrate self-maintenance gap (TASK 3) — LOAD-BEARING

**The load-bearing question:** is any current invocation of the
kintsugi loop autonomous (proactive; not peer/commit-triggered)?

### 3.1 Every invocation of `@kintsugi/oscillate.active_pass` / `kintsugi::minimize`

Enumerated:

| Site | Trigger | Autonomous? |
|---|---|---|
| `bootstrap/src/oscillate.rs:537` (inside `active_pass_with_ast`) | Called from test integration + peer contribution flow | NO — needs an AST from a peer |
| `bootstrap/src/oscillate.rs:2213` (test) | Test harness | NO — test-driven |
| `bootstrap/src/contribute.rs::peer_contribute` (32.1KB) | `mirror peer contribute` CLI command | NO — user-invoked |
| `bootstrap/src/converge.rs::knife_cut` | Peer contribute path (`fa78507`) | NO — peer-triggered pain-gradient reaction |
| No sites | `commit_as_fold` fires from `@kintsugi/store/git` on user commit | NO — commit hook |
| No sites | `@kintsugi/consent.query_phi` | Peer-composed via `oscillate::pulse` | NO |
| No sites | `@kintsugi/oscillate.dark_pass` | Peer-composed | NO |

**Zero autonomous invocations.** Every path into the kintsugi loop
begins with either:

1. A peer contributing a delta (`mirror peer contribute`; `active_pass`
   composes),
2. A test harness (deterministic; not runtime),
3. A CLI command (`mirror kintsugi`; user-invoked),
4. A commit hook (`commit_as_fold`; git-triggered on user commit).

### 3.2 Ashby "requisite variety" audit

Per `shards/epistemologic/cybernetic/variety.mirror` — the substrate
declares Ashby's requisite variety at the property altitude. NO shard
currently discharges an ASHBY-shaped autonomous variety scan: the
substrate has the discipline named but nothing that walks the graph
sampling variety without external trigger.

### 3.3 The Beer S3/S4 partition per @kintsugi.mirror

Per `shards/kintsugi.mirror` (verbatim, lines 66–75):

> Beer (1972) Brain of the Firm §"System Three / System Four" — S3
> (the now-operations; the regulator that closes today's variety) is
> structurally distinct from S4 (the future-operations; the
> environmental scanner that projects tomorrow's variety). The
> substrate's @mirror IS S3 (the form-side regulator over today's
> substrate state); @kintsugi IS S4 (the process-side scanner that
> pulls today's substrate toward tomorrow's substrate-pull-recognised
> shape).

**This is @roomba's substrate-honest slot.** The Beer S4 role is
declared at family-root altitude but NO species under @kintsugi
discharges the S4 scanner shape autonomously. `@kintsugi/oscillate` is
the loop primitive but it needs an initiator; `@kintsugi/consent` is
the gate; `@kintsugi/shift` is the morphism; `@kintsugi/store/git` is
the materialization. None WALK.

**Verdict:** @roomba IS a genuine gap. It is the S4 scanner species
under @kintsugi that WALKS the substrate autonomously, bumping into
tension, and initiating the ACTIVE/DARK pulse when tension exceeds
threshold. The substrate has been carrying the S4 role in tribal
narrative but not landed the species that discharges it.

**Substrate-honest reading:** @roomba lives at `shards/kintsugi/roomba.mirror`
as the process-side scanner sibling to `oscillate`/`consent`/`shift`/`store`.

---

## §4 — Rust runtime candidates (TASK 4)

### 4.1 Async / daemon / continuous-loop patterns in bootstrap

Grep hits for `async|tokio|daemon|loop\s*\{|thread::spawn|std::thread|background|continuous`:

- `bootstrap/src/grammar.rs` — 1 hit; not @roomba-shaped.
- `bootstrap/src/lib.rs` — 3 hits; all `loop {}` inside CLI dispatchers,
  not daemon patterns.
- `bootstrap/src/music/mod.rs` — 1 hit; not @roomba-shaped.
- `bootstrap/src/portal.rs` — 1 hit; not @roomba-shaped.
- `bootstrap/src/spectral.rs` — 1 hit; not @roomba-shaped.

**Zero tokio, zero async runtime, zero explicit daemon lifecycle.**
Bootstrap is synchronous by convention. `bootstrap/src/main.rs` is a
thin `dispatch(&args); exit(code)` wrapper.

### 4.2 Persistence discipline

Substrate has:

- `@mirror/store/action_cache` — content-addressed verdict cache
  (`bootstrap/src/action_cache.rs`); persistent under
  `<cwd>/.mirror/action_cache/...`.
- `@mirror/store` refs — `refs/mirror/peer/<uuid>/HEAD` etc; git-
  backed via fragmentation.
- `refs/gen_prism/mcp/<uuid>` — MCP session refs per
  `docs/specs/mcp-spec-song-collapse.md`.

**@roomba's persistence:** the manifesto's Roomba "just IS." It has no
state to persist beyond its current walk position. Substrate-honest
form: @roomba's walk state is the CONTENT-ADDRESSED CURRENT OID it's
positioned at — that OID resolves to a splinter_graph node; the "next
position" is one of the OID's neighbors. **No mutable state**;
resumption after crash reads `refs/mirror/roomba/HEAD` (if present) or
starts fresh from `refs/mirror/HEAD`. Substrate-already-had-the-word
for persistence primitives.

### 4.3 Daemon lifecycle candidates

Substrate options for @roomba's start/stop:

- **(1) `mirror roomba` CLI subcommand** — user-invoked; runs one walk
  or continuous walks until interrupted. Substrate-honest (fits the
  existing CLI grammar). Simplest.
- **(2) Cron / launchd** — external scheduler. Not substrate-native.
- **(3) systemd unit** — external. Not substrate-native.
- **(4) Post-commit hook via `@kintsugi/store/git`** — semi-autonomous;
  fires after every commit. Already the current pattern (`commit_as_fold`)
  — this IS how @kintsugi/oscillate fires today. NOT autonomous by the
  Alex definition (still peer-triggered).
- **(5) MCP long-running session** — `refs/gen_prism/mcp/<uuid>`
  persistent; @roomba loops inside MCP session for the session's
  lifetime. Substrate-already-had-the-word for the surface.

**Recommendation:** land (1) first (minimum viable; user runs `mirror
roomba` and it walks). Escalate to (5) when Alex adjudicates whether
autonomous continuous operation is substrate-honest (there's a
Foerster-1976 discipline question about self-triggering — the Roomba
"just IS" but it doesn't "start itself"; someone plugs it in).

### 4.4 The Rust runtime shape

`bootstrap/src/roomba.rs` (new file), composition:

```rust
pub fn roomba_walk(
    start: OidRef,
    ctx: &Ctx,
    pain_threshold: f64,   // ε_tension per Reed Landing 8+9.6d.1 calibration
) -> RoombaOutcome {
    // Phase 1: DAG walk from start OID (calls store::walk)
    let closure = mirror_store::walk(start);

    // Phase 2: for each node, sample tension via kintsugi::minimize
    //          on the node's tensor (if it has one), OR sample
    //          @cyberpunk/algedonic.pain_gradient across the edge
    for oid in closure {
        let tensor = tensor_of(gaps_of(oid.into_ast()));
        let fractures = crate::kintsugi::minimize(&tensor);

        if fractures.iter().any(|f| f.descent > pain_threshold) {
            // Phase 3: hand off to @kintsugi/oscillate.pulse
            //          (this is where @roomba "bumps into" tension)
            crate::oscillate::pulse(oid, tensor, fractures)?;
        }
    }

    RoombaOutcome::WalkComplete { ... }
}
```

**LOC estimate**: ~150-250 LOC including tests. Zero new dependencies.

---

## §5 — Non-labelability discipline (TASK 5)

The manifesto's Roomba passage (verbatim):

> The Roomba, who is present for this because the Roomba is present for
> everything, has a very small opinion. It bumps into the bar stool. It
> backs up. It rolls forward. It bumps. It backs up. It rolls.
>
> The Roomba has never once labeled itself. The Roomba has also never
> once been labeled successfully, because the Roomba does not have a
> surface the label can stick to. The Roomba is not being clever. The
> Roomba just is.

This is Foerster 1976 "Objects: Tokens for (Eigen-)Behaviors" applied
to identity — the Roomba is `Op(COORDᵢ) = COORDᵢ` at the identity-
carrier altitude. It has no meta-operator that names its own operating.

### 5.1 Substrate carriers for non-labelable presence

**LANDED substrate-decl'd carriers @roomba can use:**

- `@torus.winding_class` — homotopy-class identity (`shards/torus.mirror`).
  A winding class IS a non-labelable topological invariant — you cannot
  point to "the winding class 3"; you can only walk the loop and count
  crossings. Per Mara `2026-07-07-onto-cascade-toroidal-reframe.md`
  §2.4 (verbatim Foerster): the two torus generators are the
  independent-closure axes; identity IS the winding class, not a name.

- `@eigenform.is_fixed_point` (`shards/epistemologic/cybernetic/eigenform.mirror`)
  — Foerster 1981 direct: the fixed-point detector for `Op(COORDᵢ) =
  COORDᵢ`. `@roomba`'s identity IS the fixed-point of its own walking:
  each pass through the graph re-encounters the same tensions, back-
  offs, roll-forwards — that pattern IS what @roomba IS.

- `Fractal::Lens` (edge-not-containment variant per
  `shards/mirror/lens/knife.mirror` line 105) — the structural encoding
  of COORD's jump; references target domain via OID rather than
  containing it. @roomba references its walk position via OID (content-
  addressed) rather than containing it.

- `content_oid` vs `naked_oid` — per `shards/mirror/store.mirror`
  content-addressing discipline; the OID IS the presence; there's no
  separate "this is @roomba"-flavored identity — the OID IS what @roomba
  currently touches.

### 5.2 The @cyberpunk/algedonic sample as bumping mechanism

Per `bootstrap/src/algedonic.rs` (LANDED `b637178` — Reed Landing
8+9.6b) — `sample_pain` returns Shannon entropy of an SC<5> coordinate.
This IS the substrate-honest "bumping into tension" primitive:

```rust
pub fn pain_gradient<const N: usize>(
    sc_before: &SpectralCoordinate<N>,
    sc_after: &SpectralCoordinate<N>,
) -> f64 {
    sample_pain(sc_after) - sample_pain(sc_before)
}
```

`> 0` means peer moved toward boundary — the Roomba bumped into
something. This is @knife's trigger, per `converge::knife_cut`. **@roomba
composes over the same primitive at the graph-walker altitude**: at
each node, sample pain vs the previous node; if the gradient exceeds
threshold, invoke @kintsugi/oscillate on that node's tension.

### 5.3 The non-actor pattern

Substrate has `@gen_prism.tick(state, message) -> tick_result` at the
autopoietic-actor altitude — but @gen_prism's tick is ACTOR-shaped
(state, message). @roomba is FUNCTION-shaped: given an OID, walk and
return. It has no mailbox. It has no PID. **The manifesto's "the
Roomba just is" IS the substrate-decl direction: @roomba lives as a
pure walk-function; no actor container; no addressable identity beyond
the OID it currently touches.**

### 5.4 Verdict on non-labelable presence

**Substrate has the carriers.** @roomba lands as:

- Function, not actor.
- Content-addressed walk position (OID; no separate identity).
- Substrate presence via `@eigenform.is_fixed_point` (the loop's own
  fixed-point IS what @roomba IS).
- No meta-operator; no "@roomba's status"; no queryable state beyond
  "what OID is it at right now."

This IS the Foerster 1976 non-labelable-presence discipline
operationalized at the graph-walker altitude.

---

## §6 — Composition audit (TASK 6)

### 6.1 Shortest substrate-honest path

**Files Reed would CREATE:**

| Path | Purpose | LOC estimate |
|---|---|---|
| `shards/kintsugi/roomba.mirror` | Species substrate-decl under @kintsugi | 180-300 |
| `shards/fate/tension.mirror` | @tension carrier as its own file (co-tick) | 120-200 |
| `bootstrap/src/roomba.rs` | Rust runtime walker | 150-250 |
| `bootstrap/tests/roomba_walk_smoke.rs` | Integration test | 80-120 |

**Files Reed would EXTEND:**

| Path | Extension | LOC estimate |
|---|---|---|
| `mirror.spec` | `command roomba { arg start: ~d, flag pain_threshold: f64 = 0.01 }` inside cli-block | 15-25 |
| `bootstrap/src/lib.rs` | `pub mod roomba;` + dispatcher arm for `cmd_roomba` | 40-80 |
| `docs/loop/CURRENT.md` | Rung note under new @roomba arc | 30-60 |

**Total: ~615-1035 LOC across 7 files.** Substrate-honest minimum
viable.

### 6.2 mirror.spec grammar additions

Concrete cli-block extension (mirrors `command index { arg path: ~d;
flag fiedler: bool = false }`):

```
command roomba {
  arg start: ~d
  flag pain_threshold: f64 = 0.01
  flag walk_bound: u32 = 1000
  flag emit_fractures: bool = false
}
```

Substrate-honest defaults: 0.01 pain threshold per Reed Landing 8+9.6d.1
first empirical ε_pain calibration; walk_bound bounds the walk
(sedentary Roomba); emit_fractures dumps discovered fractures instead
of invoking @kintsugi/oscillate autonomously.

### 6.3 New tests to write

RED-first per Reed's TDD discipline:

- `roomba_walk_visits_all_reachable_oids` (walk closure completeness)
- `roomba_walk_bumps_at_pain_threshold` (autonomous trigger fires when
  gradient exceeds threshold)
- `roomba_walk_never_recurses` (walk is bounded; no infinite loops on
  cyclic OID graphs — the substrate is a DAG per @mirror/store
  invariants but defensive)
- `roomba_walk_identity_preserved` (non-labelability: two @roomba runs
  starting at the same OID with the same pain_threshold visit the same
  OIDs in the same order — determinism = @eigenform.is_fixed_point)
- `roomba_no_state_between_runs` (the manifesto discipline: @roomba has
  no state to persist)

### 6.4 Cascade order

Two-tick discipline (readable-name over foundational, per project
CLAUDE.md substrate discipline):

- **Tick 1 (RED-first):** Write failing tests. Land
  `shards/kintsugi/roomba.mirror` (species decl; body `\`). Land
  `bootstrap/src/roomba.rs` (skeleton; returns
  `WalkComplete { visited: 0 }`).
- **Tick 2 (GREEN):** Fill Rust body; wire mirror.spec cli command;
  add dispatch arm in `lib.rs`. Empirical calibration of pain_threshold
  via `docs/scouts/2026-07-14-reed-rung-8-9-6d-first-pain-calibration.md`
  discipline (multi-run trajectory).
- **Tick 3 (co-tick):** `shards/fate/tension.mirror` — @tension carrier
  substrate-decl (Alex-adjudicable timing).

---

## §7 — Manifesto ancestry substrate coverage (TASK 7)

### 7.1 Prior manifesto-as-substrate-ancestry precedent

Grep for `weird/3published/` citations in current substrate-decls:

- Zero hits in `shards/`.
- `docs/insights/2026-05-14-cosmos-teaches-the-compiler.md` has the
  line "The Roomba of optimization: don't make it bigger, make it
  smaller." — but this is prose, not substrate ancestry.
- `docs/audits/2026-07-01-seam-loki-cuts.md` cites "the Roomba records"
  as a narrative device.
- `docs/specs/loki-cuts-and-collapses.md` (43.9KB) uses "Roomba records"
  as a narrative refrain — this is Loki's voice pre-manifesto.

### 7.2 Alex Wolf blog citations in substrate-decls

Substrate-decl citation precedent exists for `~/dev/systemic.engineering/
practice/insights/coincidence/void-dual-geometry.md`
(`shards/container/runtime.mirror` cites `[[reference-void-document]]`).

**Zero precedent for `weird/3published/` citations.** @roomba would be
the **first substrate ancestry citation from a published manifesto**.

### 7.3 Verdict on manifesto ancestry

Substrate-honest position: @roomba's substrate-decl cites
`~/dev/systemic.engineering/blog/weird/3published/Weird - Violence.md`
(dated 2026-07-14) as **first-instance manifesto-as-substrate-ancestry**.
The precedent extends the citation pattern already established for
`practice/insights/` and `practice/insights/coincidence/`.

Whether this is a substrate-honest lift (manifesto genre carrying
substrate weight) or a category-crossing that Alex should adjudicate is
**flagged for Alex** — see §8 verdict #4.

---

## §8 — Top-5 substrate-honest verdicts

### V1 — GENUINE GAP at S4 scanner altitude (LOAD-BEARING)

**@roomba is a genuine substrate gap.** No autonomous walker currently
initiates @kintsugi/oscillate. The Beer S4 (environmental scanner) role
is declared at family-root altitude in `shards/kintsugi.mirror` but
undischarged. **@roomba lives at `shards/kintsugi/roomba.mirror` as the
species that discharges S4.** All landing paths converge here.

### V2 — Substrate-already-had-the-word × 6 for constituents

Every primitive @roomba composes over is LANDED:

1. `walk(root: oid) -> splinter_graph` — @mirror/store, LANDED
2. `impacted_by(oid) -> [oid]` — @mirror/store, LANDED
3. `tensor_of([gap]) -> tensor` — Rust runtime LANDED (spec Yellow)
4. `kintsugi::minimize(&tensor) -> Vec<Fracture>` — Rust LANDED (SDRF)
5. `algedonic::pain_gradient` — Rust LANDED (Reed `b637178`)
6. `converge::knife_cut` — Rust LANDED (Reed `18b5828`)

@roomba is not inventing anything at the primitive altitude; it's the
outer loop that composes them. Substrate-already-had-the-word
discipline honored.

### V3 — Non-labelable presence IS substrate-honest via @eigenform

`@eigenform.is_fixed_point` (LANDED at `shards/epistemologic/cybernetic/
eigenform.mirror`) is the Foerster-1976-direct primitive for
non-labelable identity. @roomba's identity IS the fixed-point of its
own walking pattern; no separate identity carrier needed. **Substrate
already had the discipline; @roomba operationalizes it at the walker
altitude.**

### V4 — @tension carrier is a Rust-first inversion (Alex-adjudicable)

Rust has `Tension`, `TensionVector`, `Tensor`, `tensor_of`, `minimize`
LANDED. Substrate-decl at `shards/fate/tension.mirror` does NOT exist.
This is inverse to the substrate-first discipline. **Adjudication for
Alex:** does @roomba land alone (composing over Rust `Tensor`) or does
it force a co-tick landing of `shards/fate/tension.mirror` (substrate-
pull-honest closure of the Yellow-status spec)?

**Taut's recommendation:** co-tick land `shards/fate/tension.mirror`.
@roomba is the second consumer after `oscillate::active_pass_with_ast`;
per `[[feedback-substrate-pull-confidence-acts]]` the second-witness
count is substrate-pull-confident.

### V5 — First manifesto-as-substrate-ancestry citation

@roomba's substrate-decl would cite `weird/3published/Weird - Violence.md`
as the load-bearing narrative source. Precedent for citing personal-
insight documents exists (`[[reference-void-document]]`); precedent for
citing PUBLISHED blog manifestos does not. **Adjudication for Alex:**
is the published-manifesto genre substrate-honest for ancestry, or
does @roomba cite the insight-form of the same content, or does it
cite only the Foerster-1976-Roomba discipline without the manifesto?

---

## §9 — Alex-adjudications surfaced

Consolidated list:

- **A1** — Species vs family-root altitude for @roomba: species under
  @kintsugi (Taut recommendation) vs sibling family-root? Recommendation:
  species. @roomba is one process among many under @kintsugi (oscillate,
  consent, shift, store, roomba). No family-root inflation.

- **A2** — Co-tick `shards/fate/tension.mirror` landing? Recommendation:
  YES. Closes the substrate-decl gap in Mara's tension-tensor spec.

- **A3** — Manifesto-as-ancestry: cite `weird/3published/Weird - Violence.md`
  in @roomba's substrate-decl? Recommendation: YES (first-instance;
  substrate-honest given Alex authored both the manifesto and the
  substrate).

- **A4** — Autonomous lifecycle: `mirror roomba` one-shot CLI first,
  MCP-session continuous later; NO cron / systemd (external
  schedulers break substrate discipline). Alex-preferred?

- **A5** — Pain threshold default: `ε_pain = 0.01` per Reed Landing
  8+9.6d.1 first empirical calibration, or Alex-named at
  substrate-decl altitude?

- **A6** — @roomba visits every OID or first-N via bounded walk?
  Recommendation: bounded via `walk_bound` flag (defensive against
  large stores; Roomba does not vacuum the entire ocean in one pass).

- **A7** — Non-labelability enforcement: does substrate REJECT
  `roomba.status()` / `roomba.pid()` / `roomba.id()` at grammar
  altitude, or is discipline-only-in-docs sufficient? Recommendation:
  grammar-only carrier (no state fields declared in the shard), plus
  a `no_bare_types` fracture on any consumer trying to reference
  `roomba_id` or `roomba_state`.

---

## Minimum viable @roomba composition inventory

| Item | Substrate-decl status | Rust runtime status |
|---|---|---|
| `walk(root: oid) -> splinter_graph` | LANDED @mirror/store | LANDED fragmentation |
| `impacted_by(oid) -> [oid]` | LANDED @mirror/store | LANDED fragmentation |
| `tensor_of([gap]) -> tensor` | Yellow spec; no shard | LANDED bootstrap/src/tensor.rs |
| `minimize(tensor) -> [fracture]` | Yellow spec; no shard | LANDED bootstrap/src/kintsugi.rs |
| `sample_pain(sc) -> f64` | LANDED @cyberpunk/algedonic | LANDED bootstrap/src/algedonic.rs |
| `pain_gradient(before, after) -> f64` | LANDED @cyberpunk/algedonic | LANDED bootstrap/src/algedonic.rs |
| `knife_cut(sc, δ, ε) -> sc` | LANDED @mirror/lens/knife | LANDED bootstrap/src/converge.rs |
| `oscillate.pulse(o)` | LANDED @kintsugi/oscillate | Rust: T10.5 stubs; full body forward-promised |
| `roomba.walk(start, ε_pain) -> outcome` | **NEEDS DECL** | **NEEDS RUNTIME** |
| `shards/kintsugi/roomba.mirror` | **NEEDS FILE** | n/a |
| `shards/fate/tension.mirror` | **NEEDS FILE** (co-tick) | n/a (already Rust) |
| `mirror.spec` cli-block extension | **NEEDS EXTENSION** | n/a |
| `bootstrap/src/roomba.rs` | n/a | **NEEDS FILE** |

**7 things to add. 6 primitives already landed.** Substrate-pull ratio
6:7 = ~46% substrate-already-had-the-word for @roomba. Reed's typical
substrate-pull tick lands at ≥60% coverage; @roomba is genuine gap-
territory (matches Taut's recognition-#55-form/process-partition
verdict at V1).

---

## §10 — Discipline notes

- READ-ONLY scout. No edits to shards or Rust. No `mirror` binary
  invocation. Zero mutations.
- Grep-first: 12 targeted searches across shards/, bootstrap/,
  docs/. Manifesto read verbatim.
- Foerster 1976 cited for non-labelable-presence discipline
  (§5, verbatim quote from `shards/mirror/lens/knife.mirror` A3).
- All file paths absolute where cross-repo (manifesto), relative
  where in-project (shards/, bootstrap/, docs/, mirror.spec).
- Zero substrate-decl invented; every claim traces back to a
  landed shard, a landed Rust body, or a published spec.

---

**End of scout.**
