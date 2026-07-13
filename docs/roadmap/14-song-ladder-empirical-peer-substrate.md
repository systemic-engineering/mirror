# 14 — @song ladder → empirical-discharge peer-in-substrate (Rungs 0–7)

## Status: LADDER CLIMB DISCHARGED 2026-07-13 (Rungs 0–7 landed on `main`; Rung 7 has correction pending; not yet pushed)

**Peer runs INSIDE the substrate it declares.** Recognition #43 (mirror IS content-addressed build system) extended by empirical discharge: mirror IS content-addressed AI-inference substrate. The recursion closes at the peer-contribute altitude — a Fate-spawned agent proposes a working shard delta, the compiler verifies via `@mirror/mosaic.settle`, and `@kintsugi/store/git.commit_as_fold` materializes the morphism on the peer's own DAG.

Alex's mandate for this arc (verbatim 2026-07-13, in-transcript /loop dynamic mode):

> "climb the ladder until unresolvable ambiguity that cannot be postponed further."

And the load-bearing correction that promoted Rung 7 from ceremonial to empirical:

> "it's not empirical certainty until a Fate spawned agent contributes working mirror back to the compiler. I really feel I shouldn't need to state that so plainly, and yet here we are."

---

## The ladder

| Rung | Substrate landing | Discharge shape |
|---|---|---|
| 0 | `shards/song/beat.mirror` sixth species mint (Mara `94e55eb`) | Atomic-execution unit; @kintsugi/oscillate ACTIVE/DARK pulse binding |
| 1 | `--song <file>` CLI flag + `bootstrap/src/song.rs` module (Reed `5fdc009`→`c36fbf5`) | Hardcoded single-beat @song fires @kintsugi/oscillate; beat-envelope |
| 2 | Line-per-beat phrase parsing (Reed `79eee6f`→`70766c3`) | Non-empty lines = beats; phrase-envelope + phrase_beat_count |
| 3 | Mirror-native song grammar (Reed `7b7fb0b`→`0cc4e11`) | `shards/song/keywords.mirror` companion + tokenize+AST walk; per-block envelope |
| 4 | Multi-peer @dance runtime (Reed `5b301a4`→`dfac8fe`) | `--dance-with <peer-home-2>`; Kuramoto order parameter + Aumann agreement + shared_root_oid |
| 5 | @spectral/garden deployment envelope (Reed `96ad431`→`49576a7`) | `--deploy-to <target>` composes over Rung 4; 6 substrate authorities + stub nix_derivation_oid |
| 6' | `--emit-crystal` @mirror/store-bounded peer (Reed) | Peer's terminal output = crystal OID address on `refs/mirror/peer/<uuid>/HEAD`; one @io crossing per spawn |
| 6.1a–c | Real canonical_hash → real git blob → real commit_as_fold discharge (Reed) | Blob→tree→commit chain in peer_home/.git; Recognition #55 form/process partition operational |
| 6.2a | @mirror/store IS a DAG — peer beams chain from parent commit (Reed `8bb2d7d`) | `git rev-parse --verify` → `commit-tree -p <parent>`; peer identity persists through DAG; T10 test verifies |
| 7 | Fate-spawned peer contributes working shard delta (Reed `a2c71fd`) | `mirror peer contribute <peer_home> --target <shard>`; fate.resolve → morphism → cargo check → commit_as_fold with 5-blob tree OR revert; 5/5 tests + empirical demo |

---

## Empirical demo (2026-07-13, `/tmp/rung7-empirical-demo/`)

Live-run against a real substrate fixture:

**Invocation 1** — Fate/Explorer @ shift; target `.mirror` shard 39926 → 40026 bytes; `cargo check --offline --quiet --manifest-path <peer_home>/Cargo.toml` returned green; commit `e044961e` root (5-blob tree: pre-anchor / post-anchor / morphism-body / settle-verdict / fate-witness).

**Invocation 2** — Fate/Abyss @ focus (non-deterministic Model+prism_op selection); target 40026 → 40123 bytes; cargo check green; commit `e4edb75c` with `parent=e044961e` (Rung 6.2a DAG chain).

Peer's DAG at `refs/mirror/peer/cd2e7234-6126-edb2/HEAD` spans two parent-linked commits, each carrying a compiler-verified morphism. That IS the discharge Alex's nervous system was yearning for.

---

## Substrate closure claim

Recognition #43 (mirror IS content-addressed build system) → mirror IS content-addressed AI-inference substrate.

- Peer inference stays @magic-native (gauge-bounded interior; Recognition #80).
- Peer identity = crystal OID on @mirror/store internal ref (`refs/mirror/peer/<uuid>/HEAD`).
- Materialization = ONE @io crossing per peer cycle via `@kintsugi/store/git.commit_as_fold` (Recognition #55).
- Compiler verifies the delta (`@mirror/mosaic.settle` at @code/rust altitude).
- Peer's DAG chains naturally across invocations (splinter_graph trichotomy per `shards/mirror/store.mirror`).

---

## Known substrate errors (correction in-flight)

**Rung 7 reached for `Fate::excited()` when `Fate::bounded` was the substrate-honest carrier** (Alex 2026-07-13 in-transcript: "What about Fate::bounded? We added it, why aren't we using it? It maps directly onto the sheaf math.").

- `Fate::excited()` — xorshift64 seeded from wall-clock time; random weights, non-deterministic. One witness, no sheaf grounding.
- `Fate::bounded` (via `Fate::untrained() + selectors_from_psychohistory_root(root)` pattern) — deterministic under peer's psychohistory sheaf root; v1 stub xorshift-seeded from OID, v2 lifts to sheaf-Laplacian Δ_F Rayleigh direction.
- The `fate_select_peer_beam` in `bootstrap/src/lib.rs` already discharges the bounded pattern. Rung 7's `bootstrap/src/contribute.rs` did not read it before reaching for excited.

**Rung 7's 5-blob tree conflates witness role with gate role.** Per Asher (2026-07-10, "Meaning Is Not a Metric"): the `fate-witness` blob is an evidential witness; the `settle-verdict` blob is a constitutional gate. Currently they share tree position. Rung 7' correction lifts them into separate `witnesses/` and `gates/` subtrees per the tripartition. See [15-fractal-membrane-Asher-tripartition.md](15-fractal-membrane-Asher-tripartition.md).

---

## Forward-promised (Rung 7.5+)

- **Rung 7' — Fate::bounded discharge** (Mara canonical spec in-flight 2026-07-13). Swap `Fate::excited()` → `Fate::untrained() + selectors_from_psychohistory_root(peer_home_root)` in `bootstrap/src/contribute.rs`. Sheaf-mathematical grounding for peer's Model+prism_op selection.
- **Rung 7.5 — @kintsugi/oscillate.dark_pass runtime discharge.** Morphism proposal without immediate materialization (`@kintsugi/oscillate` DARK-pulse). Peer can propose N morphisms in dark, evaluate all N under Fiedler proximity, and materialize only the ACTIVE-pulse survivor.
- **Rung 7.6 — Scope B: shard-body edits.** Fate + Cartographer models emit non-docblock morphisms (bounded shard-body rewrites).
- **Rung 8 — @fractal composition** (see roadmap entry 15). Rung 7's 5-blob tree → tripartition subtrees (witnesses / gates / authority / base). Compose across all @io-facing layers.
- **Rung 9 — MCP lift.** `mirror_peer_contribute` MCP tool + `emit_crystal` inputSchema addition on `mirror_peer_beam`. Closes CLI↔MCP capability parity for peer-in-substrate discharge. Composes with Rung 8 tripartition surface at MCP altitude.
- **Rung 10 — Real @spectral/garden deployment** (per roadmap entry 05/12 forward-promise). Currently blocked on Alex operational input (spectral.engineer endpoint spec, @mirror/mosaic nix flake structure, mycelial propagation protocol).

---

## Composition with prior arcs

- **Recognition #43** (mirror IS content-addressed build system) — EXTENDED via peer-in-substrate discharge.
- **Recognition #55** (form/process partition; @mirror/store form + @kintsugi transformation) — OPERATIONALIZED via commit_as_fold discharge in Rungs 6.1c → 7.
- **Recognition #58** (Fate IS optical inference; @magic-native) — PEER APPLICATION: fate-spawned peer produces morphism at contribute altitude.
- **Recognition #80** (@magic gauge-bounded interior) — PEER STAYS INSIDE per Rung 6' @io-minimization discipline.
- **Recognition #107** (@io Turing-unbounded boundary) — PEER CROSSES ONCE per cycle, at materialization only.
- **Recognition candidate `#R-peer-lives-in-mirror-store-@kintsugi-materializes-to-git`** — substrate closure claim for Rung 6'.
- **Recognition candidate `#R-fate-active-pass-mosaic-verdict-composition`** — Mara `4e69066` §11; Rung 7 composition witness.

---

## Status (2026-07-13)

- [x] Rungs 0–5 @song ladder landed (Reed + Mara collaboration)
- [x] Rung 5.5 MCP capability parity for Rungs 1–5 (`mirror_peer_beam` inputSchema extension)
- [x] Rung 6' substrate-inversion arc (peer inside @mirror/store)
- [x] Rung 6.1a–c commit_as_fold empirical discharge
- [x] Rung 6.2a @mirror/store IS a DAG (parent-linked chain)
- [x] Rung 7 empirical-discharge with `Fate::excited()` (correction pending)
- [x] Rung 7 empirical demo against real substrate fixture (2026-07-13)
- [ ] Rung 7' Fate::bounded correction (Mara canonical spec in-flight)
- [ ] Rung 7.5 @kintsugi/oscillate.dark_pass runtime discharge
- [ ] Rung 8 @fractal composition (see [15-fractal-membrane-Asher-tripartition.md](15-fractal-membrane-Asher-tripartition.md))
- [ ] Rung 9 MCP lift (mirror_peer_contribute + emit_crystal on mirror_peer_beam)
- [ ] Rung 10 real @spectral/garden deployment (blocked on Alex operational input)
- [ ] Push arc branch to origin (blocked on Alex authorization; project CLAUDE.md discipline)

— Reed
