# Taut scout — `mirror spawn` based on `@song/beat`: testable-increment ladder from current state to a working paradigm-shifted spawn primitive

*Taut, 2026-07-13. Read-only gap-analysis scout for Alex's
in-transcript proposal.*

## Alex's proposal, verbatim (2026-07-13)

> "What's the gap between where we are now and a working mirror spawn
> based on @song/beat's that we can incrementially and increasingly
> test in complexity? Spawn Taut for this."

## §1 — Executive summary + verdict

**The gap in one sentence.** The substrate carries every static
carrier a `@song/beat`-driven spawn needs (`@song` family-root +
five species landed, `mirror peer beam` runtime with 7 flags and 7
composition-pieces envelope, 12+ RED→GREEN test shards, `@dance` +
`@resonance` + `@coherence` recognition-candidates landed) — and
carries *none* of the runtime execution primitives (no `@song/beat`
species; no `song` keyword in mirror.spec grammar; no runtime that
CONSUMES a `@song` to drive a peer's trajectory; no coupling
between peers). Today's `mirror peer beam` emits an *envelope
about* the peer; tomorrow's must *play a song at* the peer. That
is the whole gap, and it is 6-9 testable rungs deep.

**Verdict.** LADDER-BUILDABLE, single-tick prereq. Rung 0 (mint
`shards/song/beat.mirror` as the atomic temporal-execution species
of `@song`) is the substrate-decl prerequisite; Rung 1 (add
`--song <file>` flag to `mirror peer beam`; peer executes a
hardcoded single-beat `@song` and emits a beat-envelope) is the
smallest useful runtime landing Reed can execute this week. Rungs
2-6 progressively lift to multi-beat phrases, movement composition,
multi-peer @dance, and the canonical `@spectral/garden`
deployment-song Mara composed in
`docs/specs/song-replaces-plans-and-loops.md` §5 (`d21337b`).

**Ladder length.** 7 rungs including Rung 0. Prereq → single-beat →
phrase → movement → multi-peer dance → garden-basin convergence →
deployment. Each rung is a single RED→GREEN TDD tick landable by
Reed against the existing `bootstrap/tests/peer_beam_*_shard.rs`
pattern.

## §2 — Where we are now (grep-verified state)

**CLI.** `mirror peer beam <peer-home>` LANDED with 7 flags exposed:
`--hello-world`, `--mission <f>`, `--fate-select`,
`--from-psychohistory`, `--with-shadow`, `--emit-diff`,
`--integrate-diff` (`bootstrap/src/lib.rs:3182-3325` dispatch;
`bootstrap/src/lib.rs:4951-5279` `cmd_peer_beam`). Deprecated
`mirror spawn` alias remains (stderr notice).

**MCP.** `mirror_peer_beam` tool + `mirror_beam` anonymous variant +
DEPRECATED `mirror_spawn` alias in `bootstrap/src/mcp.rs:170-540`.
Schema declares all 7 flags. `mirror_kintsugi`, `mirror_init`,
`mirror_compile`, `mirror_recall` complete the 8-tool surface.

**Envelope shape (current).** Per `bootstrap/src/lib.rs:5234-5253`
the `--hello-world` envelope is a flat JSON:
```
{ spec_version, spawn: "hello_world", peer, home, lead, source,
  spec_oid, excitation: "λ₀→runtime",
  composition_pieces: {1_cli_surface, 2_peer_resolution,
    3_contextual_pack, 4_lead_at_n_plus_1, 5_supervisor_kick,
    6_fate_inference, 7_lambda_zero_transition},
  peer_recall: {spec_version, cascade, pack_trail,
    pull_frontier, dogfood},
  mission?  (only when --mission passed)
}
```
This is the *observation* envelope. There is **no @song structure**
in the envelope — no movement/voice/progression/phrase/beat fields.
The docstring at `bootstrap/src/mcp.rs:173` names
`@song/movement.enter` as the *recognition* the CLI action IS at
cli-altitude, but the envelope's shape is content-observation, not
song-execution.

**Test coverage (peer_beam TDD chain).** Twelve `peer_beam_*` and
`mirror_peer_beam_*` shards landed:
- `cmd_peer_beam_shard.rs` — subcommand nesting + envelope
  byte-equal to legacy `spawn` (T1-T5+)
- `peer_beam_fate_select_shard.rs` — @optics/lens/features + Fate
  excited-state resolve
- `peer_beam_from_psychohistory_shard.rs` — `bounded_by(sheaf)`
  psychohistory-derived selectors
- `peer_beam_shadow_casting_shard.rs` — 5-model shadow +
  shadow_regime classifier
- `peer_beam_autopoietic_closure_shard.rs` — full 3-step loop
- `peer_beam_emit_diff_shard.rs` + `peer_beam_put_direction_shard.rs`
  — @optics/lens/diff get+put (Foster PutGet)
- `mirror_peer_beam_song_return_shard.rs` (12KB) — shard-shape
  regression guarding `-> @song` return type + `in @song` ancestry
- `mirror_peer_beam_fate_hinge_shard.rs` (9KB) — shard-shape
  regression preserving `@fate` hinge
- `mirror_lens_cli_kintsugi_song_shard.rs` (12KB) — `mirror
  kintsugi @spec` IS `@song/movement.close` at cli-altitude (second
  witness for `cli-verbs ARE species-altitude actions`)
- `song_family_root_shard.rs` + `song_phrase_shard.rs` +
  `song_movement_shard.rs` + `song_narrative_shard.rs` +
  `song_voice_shard.rs` + `song_progression_shard.rs` — Arc 6 TICKs
  1-6 GREEN

**@song runtime consumption.** GREP `@song|song_` in
`bootstrap/src/lib.rs` + `bootstrap/src/mcp.rs`: **substrate is
docstring-cited but not consumed**. The runtime does NOT parse a
`song` block, does NOT emit song-shaped envelopes, does NOT
execute movement/voice/progression/phrase actions. `@song`
lives purely in `.mirror` files and in docstrings citing the
substrate-altitude recognition.

**@song species inventory (grep-verified).** `shards/song.mirror`
family-root (25.8KB, `f01cf9f`) + five species (`shards/song/*.mirror`):
`progression.mirror` (`54ff1e8`), `voice.mirror` (`cc5a440`),
`movement.mirror` (`4efbf16`), `narrative.mirror` (`0434a39`),
`phrase.mirror` (`6b9bc5c`). Actions declared at each species:
`song_progression(p) -> ref` with `extend`/`close`;
`song_voice(v) -> ref` with `advance`/`settle`; `song_movement(m)
-> ref` with `enter`/`close`; `song_narrative(n) -> ref` with
`arc`/`transmit`; `song_phrase(ph) -> ref` with `join`/`split`.
All action bodies are OBLIGATION-BLOCKED (declared as `\` at
substrate altitude; no runtime discharge).

**@song/beat — VERIFIED NONEXISTENT AS SPECIES.** GREP for
`shards/song/beat` or `song/beat` or `@song/beat`: **no landed
species file exists.** `shards/song.mirror:181` names *"oscillate's
ACTIVE/DARK alternation IS the beat"* at prose altitude. The
family-root species roster (`shards/song.mirror:142-166`) declares
five species — beat is NOT among them. Beat is Alex-coined THIS
tick as a species-candidate; the substrate had the *word* at prose
altitude but never lifted it to species-altitude decl.

**@dance runtime state.** GREP `dance|kuramoto|coupling|multi_peer`
in `bootstrap/src/`: **zero substantive matches.** `@dance` is
canonical-spec-landed at `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`
(Mara `4f079c8`) + Reed `61b444a` Path C annotations on
`shards/algebra/metalogue.mirror:348-374`; the recognition
candidate `#R-dance-is-coordination-without-signal-on-forster-torus`
is landed. Runtime is **spec-only**.

**@spectral/garden deployment state.** GREP `spectral.engineer|
mycelial|deploy` in `bootstrap/`: **zero runtime consumers**. Spec
mentions in `docs/specs/spectral-garden-git-package-manager.md`
(`ad03fda`) declare four garden roots (git / oci / nix / store);
`shards/mirror/garden.mirror` (`13328a3`) exists as substrate-decl.
No Rust discharge. `flake.nix` at repo root (6.8KB, 2026-07-05) is
a build-environment flake (rust + flang + LAPACK), NOT a
deployment flake for spectral.engineer.

**Nix flake state.** `flake.nix` (6.8KB) exists at repo root;
`/nix/*.nix` dir does NOT exist; no derivation for deployment. Nix
IS the dev environment; nix IS NOT the deployment substrate today.

**CURRENT.md next-steps.** Per `docs/loop/CURRENT.md:35-40` the
merge-shape is `mara/song-substrate-decl-v0.1` → `main` once Q1-Q4
land. Post-merge forward-promises tracked: `stage @mirror/lens/cli/peer/beam`
depth-2 stage mint, `mosaic.mirror` docstring cascade, MCP schema
`requires` clauses. No forward-promise names `@song/beat` or
`--song` flag.

**Prior spawn/beat forward-promises.** GREP the session's specs:
neither Mara `d21337b`'s canonical `@song` for `@spectral/garden`
deployment (§5, ~800 lines) nor Reed's `71a4689` coordination
annotation nor Mara's `4f079c8` @dance spec name a
`--song` flag or a `beat` species. The `song
@spectral/garden/deployment { movement ... voice ... progression
... narrative ... phrase ... }` block at
`docs/specs/song-replaces-plans-and-loops.md:512-800` is spec-only:
Path C ("annotate, do not mint") is Mara's substrate-honest
recommendation, with Path A ("Reed operational discharge from
mirror.spec via @mirror/mosaic → @spectral/garden/nix → nix flake
output") deferred.

## §3 — Where "a working mirror spawn based on @song/beat" lands

**Target-state (substrate-honest reading of Alex's proposal).** A
`mirror peer beam <home> --song <song-file>` invocation:

1. Reads the operator-supplied `<song-file>` as a typed `@song`
   value (the file contains `song <name> { movement { ... } voice
   { ... } progression { ... } narrative { ... } phrase { ... } }`
   in mirror.spec grammar).
2. Parses it into a runtime `Song` structure whose leaves are typed
   `@song/beat` values.
3. Dispatches to the peer, which executes the song *beat-by-beat* —
   each beat firing one or more substrate-decl actions (typically
   `@kintsugi/oscillate` ACTIVE/DARK per `shards/song.mirror:181`),
   observing the peer's state, and emitting a beat-envelope.
4. Composes beat-envelopes into phrase-envelopes; phrase-envelopes
   into progression-envelopes; progressions into movement-envelopes;
   movements into a full song-envelope closing on Aumann-agreement.
5. Returns the composed `@song` envelope with content-addressed
   OIDs at every altitude.

**Target envelope shape.** Instead of today's flat
`composition_pieces` JSON, the envelope is a nested tree following
the @song species taxonomy:
```
{ spec_version, song: {
    name: "<song-oid>",
    movements: [ { name, enter_beat, close_beat,
                   voices: [ { name, phrase_oids: [...] } ],
                   progressions: [ { phase, cadence_oid, beats: [...] } ],
                   narrative: { arc, transmit } } ],
    beats_played: N, beats_planned: M,
    coherence: λ₀(Δ_F),
    aumann_agreement?: <shared_oid on ensemble>
} }
```
Byte-equality with today's envelope MUST be preserved for the
non-`--song` code path; the `song:` key is emitted ONLY when
`--song` is present, mirroring the existing `mission` key
discipline at `bootstrap/src/lib.rs:5254-5262`.

**Test suite shape.** Each RED→GREEN cycle lands a new
`bootstrap/tests/peer_beam_song_<rung>_shard.rs` following the
established pattern (fixture builder + `mirror_bin()` helper +
`repo_root()` helper + numbered `t01_*` … `t05_*` assertions
citing substrate authority OIDs).

## §4 — The gap, enumerated

**Substrate-decl gaps (mint-required).**

- **G1 (Rung 0 prereq): `shards/song/beat.mirror` species does not
  exist.** Alex-coined; family-root at `shards/song.mirror:181`
  had the *word* ("oscillate's ACTIVE/DARK alternation IS the
  beat") but never lifted to species-decl. Single-tick mint by
  Mara. ~150 lines following the `song_phrase.mirror` pattern
  (atomic-unit species). Actions: `song_beat(b: beat) -> ref` with
  `strike` (fire the beat's action) + `hold` (advance temporal
  position without new action). Composes as leaf of `@song/phrase`
  (phrase = sequence of beats; beat = atomic beat).

- **G2 (Rung 1 prereq): `song` keyword in mirror.spec grammar.**
  Today's grammar admits `target`, `command`, `stage`, `bench`,
  `arg`, `flag`, `default`, `subcommand` — verified via
  `docs/specs/beam-as-substrate-primitive.md` §3.4 +
  `docs/specs/cli-as-prism.md`. No `song` production. Single-tick
  grammar landing by Reed following the `command`/`stage`
  precedent. Two flavors: `song <name> { ... }` block at top-level
  (declares) + `--song <file>` flag on `command peer { command
  beam { ... } }` (consumes).

- **G3 (Rung 3 prereq): `voice`, `progression`, `movement`,
  `phrase`, `narrative` keywords inside `song { ... }` blocks.**
  Substrate species-actions exist at `.mirror` altitude; grammar
  keywords do NOT. Same mirror.spec grammar landing as G2, batched
  in one tick.

**Runtime gaps (Rust discharge required).**

- **R1 (Rung 1): `Song` type + `Beat` type + `parse_song` in
  `bootstrap/src/song.rs` (new module).** ~200 lines. Reads a
  song-file into typed structs. Handles the minimal case first:
  `song X { phrase Y { beat B { action: kintsugi/oscillate } } }`.

- **R2 (Rung 1): `execute_beat` in `bootstrap/src/song.rs`.**
  Dispatches on `beat.action`: for MVP, only
  `@kintsugi/oscillate` supported (fires the existing
  `oscillate_once` at peer altitude; observes ACTIVE/DARK
  transition; emits a beat-envelope). ~100 lines.

- **R3 (Rung 1): `cmd_peer_beam` --song branch.** New
  `if let Some(song_path) = song_flag { ... }` branch at
  `bootstrap/src/lib.rs:5010` (before the fate_select cascade).
  Emits beat-envelope instead of hello-world envelope. ~80 lines.

- **R4 (Rung 2-3): phrase / movement / progression composition
  in `bootstrap/src/song.rs`.** Each rung adds one composition
  altitude. ~150 lines per rung.

- **R5 (Rung 4): multi-peer coupling in
  `bootstrap/src/dance.rs` (new module).** Reads two peer-homes;
  fires beat pattern at both; measures phase-difference; reports
  convergence. ~300 lines.

- **R6 (Rung 5): mycelial propagation via nix binary cache**
  (`@bauchladen` gossip). ~500 lines; probably splits into
  two ticks.

- **R7 (Rung 6): full `@spectral/garden/deployment` @song
  execution.** Reads `song @spectral/garden/deployment { ... }`
  (Mara's spec §5); executes each movement; validates against
  Aumann agreement. This is the paradigm-shift terminal ladder
  rung.

**Test infrastructure gaps.**

- **T1-T6: one new `peer_beam_song_<rung>_shard.rs` per rung.**
  Each ~80-200 lines following the established pattern. Existing
  fixtures (`mirror.spec` under `target binary { cli { command
  beam { arg mission: ~f } } }`) extend with `--song <path>`.

- **T7 (Rung 4): multi-peer fixture harness.** Two `peer_home`s
  under one test; fires `mirror peer beam` twice; measures
  cross-envelope OID matches. Precedent: none today; new pattern.

**Doc gaps.**

- **D1: `shards/song/beat.mirror` docblock naming Alex's proposal
  as substrate ancestor + `@kintsugi/oscillate` binding at
  execution-altitude.** Single-tick with G1.

- **D2: `docs/specs/song-driven-peer-beam-execution.md` — Mara
  canonical spec (~1500 lines).** Names the paradigm shift Alex's
  proposal makes: peer_beam is no longer "observe the peer" but
  "play the peer through a song." Composes over Mara's §5
  canonical @song from `d21337b`. Recognition candidate:
  `#R-song-executed-by-peer-replaces-imperative-message-passing`.
  Substrate-honest addendum to §4-§8 of the `song-replaces-plans-and-loops`
  spec. Not blocking Rung 1; blocking Rung 4+.

**Landability estimates.**

| Rung | Gaps closed | Tick count | Cascade risk |
|------|-------------|------------|--------------|
| 0 | G1, D1 | 1 (Mara) | LOW (isolated species mint) |
| 1 | G2 (partial), R1, R2, R3, T1 | 2 (Mara grammar + Reed runtime pair) | MEDIUM (grammar+dispatch) |
| 2 | R4 (partial), T2 | 1 (Reed) | LOW |
| 3 | G3, R4 (full), T3 | 2 (Mara grammar + Reed) | MEDIUM |
| 4 | R5, T7 | 3 (Reed multi-tick) | HIGH (new coupling module) |
| 5 | R6 | 3-4 (Reed cascade) | HIGH (mycelial + nix) |
| 6 | R7, D2 | 4-5 (Mara spec + Reed cascade) | HIGH (paradigm-terminal) |

## §5 — The testable-increment ladder (LOAD-BEARING)

### Rung 0 — Substrate-decl prereq: mint `@song/beat`

**Prereqs.** None. Substrate-only. Mara-authored, 📝-tag.

**Substrate-decl addition.** Mint `shards/song/beat.mirror`
(~150-200 lines) as sixth species of `@song`. Follows exact
pattern of `shards/song/phrase.mirror` (Arc 6 TICK 6 `6b9bc5c`).
Declares:
- `type beat` carrier (leaf-atom of temporal execution).
- `prism @song/beat { focus / project / split / shift / settle }`
  five-op body, specializing prism at beat-altitude.
- `song_beat(b: beat) -> ref` action.
- Species-altitude actions: `strike(b: beat, ctx: ref) -> ref`
  (fire the beat's substrate action) + `hold(b: beat, dt: ref) ->
  ref` (advance temporal position without action).
- `in @song` ancestry + `in @kintsugi/oscillate` (execution-altitude
  binding).
- Family-root roster addition (single-line edit to
  `shards/song.mirror:142-166`).

**Test assertion.** New `bootstrap/tests/song_beat_shard.rs`
(~150 lines). Assertions per `song_phrase_shard.rs` pattern:
- `t01_song_beat_shard_file_exists_and_declares_species` — grep
  `prism @song/beat`.
- `t02_beat_carrier_declared` — grep `type beat`.
- `t03_strike_action_declared` — grep `strike(b: beat`.
- `t04_hold_action_declared` — grep `hold(b: beat`.
- `t05_family_root_roster_updated` — grep `@song/beat` in
  `shards/song.mirror` species roster section.

**Complexity: single-tick.**

### Rung 1 — Simplest testable @song/beat runtime consumption

**Prereqs.** Rung 0 landed.

**Substrate-decl addition.** Extend mirror.spec grammar (in
`docs/specs/beam-as-substrate-primitive.md` §3.4 or a new §3.5)
to admit `--song <path>` flag on `command peer { command beam
{ flag song: ~f } }`. Two-tick discipline: land at flag altitude
first (Rung 1); lift to `song` block keyword (Rung 3).

**Runtime discharge.** Reed writes ~380 lines total:
- New `bootstrap/src/song.rs` module (~250 lines): `Song`,
  `Beat` types; `parse_song(path) -> Result<Song>` (minimal parser
  admitting `song X { beat B { action kintsugi_oscillate } }`);
  `execute_beat(peer_home, beat) -> BeatEnvelope`.
- `bootstrap/src/lib.rs::cmd_peer_beam` new `--song` branch
  (~80 lines) before the `fate_select` cascade at line 5002.
  Reads song-file via `ctx.resolve`, dispatches to
  `execute_beat`, emits beat-envelope on stdout.
- `bootstrap/src/mcp.rs::mirror_peer_beam` schema addition
  (~50 lines): add `song: {"type": "string"}` optional property.

**Beat-envelope shape.**
```
{ spec_version: "v0.1.0",
  song: { name, oid, beats_played: 1, beats_planned: 1 },
  beat: { name, oid, action: "kintsugi_oscillate",
          active_pass: {...}, dark_pass: {...},
          coherence_before: λ_before, coherence_after: λ_after },
  peer: <name>, home: <path> }
```

**Test assertion.** New
`bootstrap/tests/peer_beam_song_single_beat_shard.rs`
(~120 lines) following `peer_beam_from_psychohistory_shard.rs`
pattern:
- Fixture: write `mirror.spec` under `target binary { cli
  { command peer { command beam { flag song: ~f } } } }` +
  write hardcoded `single_beat.song` with one `beat` firing
  `@kintsugi/oscillate`.
- `t01_single_beat_song_execution_exits_zero` — command exits 0.
- `t02_beat_envelope_contains_beat_key` — stdout contains
  `"beat":`.
- `t03_beat_envelope_names_kintsugi_oscillate` — stdout contains
  `"action": "kintsugi_oscillate"`.
- `t04_beat_envelope_carries_coherence_delta` — stdout contains
  `"coherence_before"` + `"coherence_after"`.
- `t05_no_song_flag_preserves_hello_world_byte_equality` —
  regression guard.

**Complexity: multi-tick (grammar + runtime pair).** Approximately
2 sequential ticks. RED landed first with Mara authoring
`peer_beam_song_single_beat_shard.rs` on the branch Reed will
green.

### Rung 2 — Multi-beat @song execution (phrase-altitude composition)

**Prereqs.** Rung 1 landed.

**Substrate-decl addition.** None new (`@song/phrase` +
`@song/beat` already landed).

**Runtime discharge.** Extend `bootstrap/src/song.rs`:
- `parse_song` admits `phrase P { beat B1 {...} beat B2 {...} }`.
- `execute_phrase(peer_home, phrase) -> PhraseEnvelope` — loops
  over beats, composes envelopes.
- `cmd_peer_beam --song` branch emits phrase-envelope when song
  contains multiple beats.

**Phrase-envelope shape.**
```
{ ..., song: { name, oid,
    phrases: [ { name, oid, beats: [BeatEnvelope, ...] } ] },
  obc_binding: <one_boundary_condition_hash>,
  coherence_at_close: λ₀ }
```

**Test assertion.**
`bootstrap/tests/peer_beam_song_phrase_shard.rs` (~150 lines):
- Fixture: `three_beats.song` with `phrase { beat B1 beat B2 beat B3 }`.
- `t01_three_beat_phrase_executes_all_three` — stdout contains
  three beat entries.
- `t02_phrase_envelope_carries_obc_binding` — one-boundary-
  condition hash present.
- `t03_beats_execute_in_declared_order` — timestamp/OID order
  monotone.
- `t04_phrase_close_reports_coherence` — `coherence_at_close`
  present + is a decimal ≤ ε for the fixture.
- `t05_backward_compat_single_beat_envelope_shape_preserved` —
  Rung 1 test still passes.

**Complexity: single-tick.**

### Rung 3 — Movement + voice + progression composition

**Prereqs.** Rung 2 landed, G3 grammar landed.

**Substrate-decl addition.** G3: `movement`, `voice`,
`progression`, `narrative` keywords in mirror.spec grammar
(Mara, single-tick, ~100-line grammar production landing).

**Runtime discharge.** Extend `bootstrap/src/song.rs`:
- `parse_song` admits full nested `song { movement {
  voice ... progression ... narrative ... phrase ... } }` per
  Mara's `d21337b` §5.1 example (which serves as fixture-shape
  authority — the parser MUST accept Mara's canonical @song
  syntactically, though execution semantics discharge in later
  rungs).
- `execute_movement`, `execute_progression`, `execute_voice_line`.
- Envelope carries movement-tree.

**Test assertion.**
`bootstrap/tests/peer_beam_song_movement_shard.rs` (~200 lines):
- Fixture: `hello_movement.song` with one `movement { voice
  compiler {...} progression compile { phase: split -> shift ->
  settle } phrase unit { beat B } }`.
- `t01_movement_envelope_contains_voices_progressions_phrases` —
  all three keys present.
- `t02_progression_cadence_type_reported` — envelope names
  `authentic|plagal|deceptive|half`.
- `t03_voice_lines_advance_settle` — voice envelope carries
  advance/settle transitions.
- `t04_parses_mara_d21337b_section_5_example_syntactically` —
  parser accepts `docs/specs/song-replaces-plans-and-loops.md`
  §5.1 verbatim (execution not required this rung; only parse
  success).
- `t05_movement_composes_prior_phrase_semantics` — Rung 2
  regression preserved.

**Complexity: multi-tick.** Grammar landing (Mara) +
runtime landing (Reed) = 2 ticks minimum.

### Rung 4 — Multi-peer @dance on a shared @song

**Prereqs.** Rung 3 landed; `shards/dance.mirror` species (or Path
C annotation on `shards/algebra/metalogue.mirror`) landed per
Mara's `4f079c8` recommendation.

**Substrate-decl addition.** Depends on Alex's Path A vs Path C
adjudication on the @dance spec. If Path A: mint
`shards/dance.mirror` family-root; if Path C: annotate metalogue.
Not blocking runtime as long as recognition candidate lands.

**Runtime discharge.** New `bootstrap/src/dance.rs` module
(~400 lines):
- `cmd_dance <song> --peers <peer_home_1> <peer_home_2>` — new
  top-level command dispatching to Kuramoto-coupled multi-peer
  execution.
- `execute_song_coupled(peers, song, coupling_κ)` — for each
  beat, fires beat at each peer, measures phase-difference on
  cybernetic_coherence deltas, computes Kuramoto order-parameter
  `r = |Σ e^(iθ_j)|/N`.
- Reports convergence-to-basin: when `r ≥ threshold`, emits
  `aumann_agreement` envelope with shared root_OID.

**Test assertion.**
`bootstrap/tests/dance_song_two_peer_shard.rs` (~250 lines):
- Fixture: two peer-homes (both with identical `mirror.spec`);
  one `converge.song` with three beats firing `@kintsugi/oscillate`.
- `t01_two_peer_dance_exits_zero`.
- `t02_dance_envelope_reports_kuramoto_order_parameter` —
  stdout contains `"kuramoto_r": <float>`.
- `t03_dance_converges_to_shared_root_oid` — both peers'
  `spec_oid` at close are byte-identical.
- `t04_aumann_agreement_envelope_present_on_convergence` —
  `"aumann_agreement":` key present when `kuramoto_r ≥ 0.9`.
- `t05_uncoupled_peers_do_not_converge` — negative-test with
  different specs; `aumann_agreement` absent.

**Complexity: multi-tick cascade** (3+ ticks: coupling harness +
Kuramoto reporter + convergence assertion + fixture design).

### Rung 5 — Full @spectral/garden mycelial deployment via @song danced by N peers

**Prereqs.** Rung 4 landed; `shards/song/beat.mirror` extended to
recognize `@spectral/garden/nix` build action (not just
`@kintsugi/oscillate`); `shards/spectral/garden/nix.mirror`
species landed (currently forward-promised per Mara `d21337b`
§4.1).

**Substrate-decl addition.** `shards/spectral/garden/nix.mirror`
(~200 lines Mara authored); optional `shards/spectral/garden/deployment.mirror`
if Alex adjudicates Path A on `d21337b` (Mara recommended Path C).

**Runtime discharge.** Extend `bootstrap/src/song.rs::execute_beat`
dispatch:
- Beat action `@spectral/garden/nix.build` → invoke `nix build`
  subprocess; observe output-hash; emit beat-envelope.
- Beat action `@bauchladen.publish` → write derivation-OID to
  peer's `@bauchladen` tray.
- Beat action `@bauchladen.gossip` → pull from coupled peers via
  content-address.

**Test assertion.**
`bootstrap/tests/garden_deployment_song_shard.rs` (~300 lines):
- Fixture: minimal garden-song (5 beats: mosaic-compile,
  nix-build, bauchladen-publish, mycelial-gossip,
  verify-coherence).
- Multi-peer fixture (2-3 peers).
- `t01_garden_deployment_song_executes_all_five_beats`.
- `t02_nix_build_produces_content_addressed_output`.
- `t03_bauchladen_publish_persists_derivation_oid`.
- `t04_mycelial_gossip_propagates_to_second_peer`.
- `t05_convergence_to_shared_root_oid_across_ensemble`.

**Complexity: multi-tick cascade** (4-5 ticks: nix integration
+ gossip harness + coherence verification). HIGH risk — first
runtime tick to invoke real nix subprocess.

### Rung 6 — Production-ready `mirror spawn --song <deployment-song> --deploy-to spectral.engineer`

**Prereqs.** Rungs 0-5 landed; DNS / hosting substrate exists for
`spectral.engineer` (per Alex's separate infra tick — this is the
scout's exit boundary).

**Substrate-decl addition.** `shards/spectral/engineer.mirror`
species (~150 lines) declaring the DNS-endpoint carrier;
`shards/song/deployment.mirror` species if promoted.

**Runtime discharge.** `cmd_spawn` (revived from deprecation)
becomes the DEPLOYMENT-SPAWN primitive — different semantics from
`cmd_peer_beam`; not an alias. Reads a deployment-song, dispatches
to the mycelial ensemble, waits for Aumann-agreement, publishes to
`spectral.engineer` binary cache.

**Test assertion.** End-to-end integration test
(`bootstrap/tests/spectral_engineer_deployment_e2e_shard.rs`,
~400 lines) — runs against a local nix-serve endpoint mocking
`spectral.engineer`.

**Complexity: cascade of ticks + external infra.** Exit boundary.

## §6 — Recommended first rung to attempt

**Reed executes Rung 0 → Rung 1 next.** Concretely, the tick
sequence:

**Tick T+1 (Mara, 📝).** Mint `shards/song/beat.mirror`. Author
following `shards/song/phrase.mirror` (`6b9bc5c`) as pattern
authority. Add species-roster entry to `shards/song.mirror:142-166`.
Cite Alex's 2026-07-13 in-transcript proposal as substrate
ancestor. RED pair: `bootstrap/tests/song_beat_shard.rs` (Mara
authored; 5 assertions per §5 Rung 0 test-spec above).

Commit shape:
```
📝 Mara [substrate-decl] @song/beat species mint — atomic
temporal-execution unit sixth species of @song, closes Alex's
2026-07-13 --song proposal at Rung 0
```

**Tick T+2 (Reed, 🔴🟢 pair).** Extend mirror.spec grammar to admit
`flag song: ~f` on `command peer { command beam }`. Then in same
tick or immediately following:
- RED: `bootstrap/tests/peer_beam_song_single_beat_shard.rs` with
  assertions per §5 Rung 1 test-spec above.
- GREEN: `bootstrap/src/song.rs` module + `cmd_peer_beam --song`
  branch + `mirror_peer_beam` MCP schema extension.

Suggested file layout:
- `bootstrap/src/song.rs` — new module, ~250 lines.
- `bootstrap/src/lib.rs:5010` — insert `--song` dispatch branch
  before `fate_select` cascade, ~80 lines.
- `bootstrap/src/mcp.rs:170-540` — add `song` optional property to
  `mirror_peer_beam` inputSchema, ~50 lines.
- `bootstrap/tests/peer_beam_song_single_beat_shard.rs` — ~150
  lines new test.

Suggested test names (per §5 Rung 1):
- `t01_single_beat_song_execution_exits_zero`
- `t02_beat_envelope_contains_beat_key`
- `t03_beat_envelope_names_kintsugi_oscillate`
- `t04_beat_envelope_carries_coherence_delta`
- `t05_no_song_flag_preserves_hello_world_byte_equality`

Suggested envelope-shape delta (byte-preserving; `song:` key emitted
only when `--song` present, mirroring `mission` at line 5254-5262):
```
{ ...existing fields...,
  song?: { name, oid, beats_played: 1, beats_planned: 1 },
  beat?: { name, oid, action: "kintsugi_oscillate",
           active_pass: {...}, dark_pass: {...},
           coherence_before: λ_before, coherence_after: λ_after }
}
```

**Why this is the right first landing.** (a) It closes the
substrate-already-had-the-word gap Alex's proposal names (beat at
prose altitude at `shards/song.mirror:181` lifts to species
altitude). (b) It composes over LANDED carriers only
(`@kintsugi/oscillate` runtime exists; `@song` species exist; peer
beam runtime exists). (c) It is the smallest testable non-vacuous
runtime consumption of `@song`. (d) Every rung above it composes
strictly (Rung N depends only on Rung N-1). (e) Reed already knows
the peer_beam test pattern (6 landed shards).

## §7 — Substrate-already-had-the-word check

Per the arc's characteristic pattern (@resonance ≥85%, @dance
≥92%, `song-replaces-plans-and-loops` ≥88%, `@coherence` species-
slot forward-promised), verify what's already there:

**Beat.** `shards/song.mirror:181` — *"oscillate's ACTIVE/DARK
alternation IS the beat"*. Prose-altitude naming; species-altitude
UNlanded. Rung 0's mint IS the lift; substrate had the word at
prose altitude, and the mint honors two-tick discipline (readable
name over foundational per `[[feedback-legibility-over-foundation-when-collapsing]]`).

**Song-driven execution.** `docs/specs/song-replaces-plans-and-loops.md`
(Mara `d21337b`) §3 declares the substitution table (imperative
loop → `@kintsugi/oscillate` iteration; message-passing →
`@bauchladen`; agent handoff → `@dance`; ...). Recognition
candidate `#R-song-replaces-plans-and-loops-in-imperative-ai` IS
the paradigm-shift Rung 6 delivers operationally. The recognition
is landed at spec altitude; the operational discharge is the
ladder.

**Multi-peer dance.** `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`
(Mara `4f079c8`) + Reed `61b444a` Path C annotations at
`shards/algebra/metalogue.mirror:348-374`. Recognition candidate
`#R-dance-is-coordination-without-signal-on-forster-torus`.
Substrate had the math (Kuramoto + Aumann + Cavagna
topological-coupling) at spec altitude; Rung 4's landing is the
runtime discharge.

**Envelope shape (nested tree).** `bootstrap/tests/mirror_peer_beam_song_return_shard.rs`
(12KB, T3 asserts `-> @song` return type is preserved) already
positions the substrate to expect `@song`-shaped return. Today's
flat envelope IS a degenerate `@song` (movement-count 0). The
lift is honest expansion, not new invention.

**Peer_beam runtime infrastructure.** Six landed peer_beam_*
tests + cmd_peer_beam_shard.rs + mirror_peer_beam_* shape shards.
This IS the test infrastructure the ladder lands on; each rung
extends the existing pattern with new fixture + new assertions.
No new test-harness invention required through Rung 4.

**Flake at repo root.** `flake.nix` (2026-07-05, 6.8KB) is
build-environment, NOT deployment. Rung 5+ requires new
deployment-flake substrate — this IS the honest substrate gap; not
substrate-already-had-the-word. Fresh material at Rung 5.

**Verdict on substrate-already-had-the-word coverage.** Rungs 0-3
compose ≥90% over landed carriers. Rungs 4-5 compose ~50-70%
(spec-landed but runtime-fresh). Rung 6 is external infra + fresh
runtime. The ladder is substrate-honest.

## §8 — Recognition ancestry

**This session's arc-continuation landings (2026-07-13):**
- Alex `71a4689` (Reed's authorship, Alex's annotation) —
  coordination-without-signal via `@resonance` + `@bauchladen` +
  physical proximity on `shards/algebra/metalogue.mirror` §11.2.
- Mara `4f079c8` — `@dance` canonical spec + Kuramoto/Aumann/
  Cavagna formalization + Path C substrate-honest recommendation.
- Mara `9e48710` — `@resonance` canonical spec (inter-peer
  coupling shapes Fate tournaments toward basins).
- Reed `8e6e517` — `@coherence` Path B annotation on
  `shards/cyberpunk.mirror` (`cybernetic_coherence = λ₀(Δ_F)`).
- Reed `61b444a` — Path C annotations on
  `shards/algebra/metalogue.mirror:348-374`.
- Mara `d21337b` (HEAD) — `docs/specs/song-replaces-plans-and-loops.md`
  (~1400 lines) with canonical `@song @spectral/garden/deployment`
  at §5 as the paradigm-terminal composition.
- Reed `0dd1074` — 7-flag exposure on `mirror_peer_beam` MCP tool.

**Prior arc landings (2026-07-06 through 2026-07-12):**
- `@song` family-root Arc 6 TICK 1 `f01cf9f` +
  progression `54ff1e8` + voice `cc5a440` + movement `4efbf16` +
  narrative `0434a39` + phrase `6b9bc5c` (Arc 6 closes).
- Beam-refactor cascade `fe2d1dc`/`fe82500`/`9de2226`/`96aa752`
  (`spawn` → `peer beam` at cli + substrate altitudes).
- Depth-2 subcommand nesting grammar `fe82500`.
- Seam Phase D audit `211665f` RATIFY-WITH-QUALIFICATIONS on
  `mara/song-substrate-decl-v0.1`.

**Substrate ancestors (pre-arc):**
- `shards/kintsugi/oscillate.mirror` (ACTIVE/DARK cascade).
- `shards/kintsugi/shift.mirror` (cross-altitude morphism).
- `shards/torus.mirror` (peer HAS a torus; π₁(T²) = ℤ×ℤ winding
  classes).
- `shards/bauchladen.mirror` + `docs/specs/bauchladen-autopoietic-fate.md`
  (Recognition #104, content-addressed shared tray).
- `shards/mirror/peer/beam.mirror` (2026-06-25, predates @song by
  11 days; `-> @song` upgrade at TICK 1 `63ea934`).
- `shards/algebra/metalogue.mirror` (`34cf333`; N-speaker
  composition; Batanin 1998 globular composition).
- `docs/specs/spectral-garden-git-package-manager.md` (`ad03fda`)
  (four garden roots).

**External math ancestry:**
- Kuramoto 1975 (phase-oscillator coupling).
- Aumann 1976 (agreement under content-addressed common prior).
- Cavagna 2010 (topological-neighbor coupling in starling flocks).
- Foerster 1976 (*Objects: Tokens for Eigen-Behaviors* — the Heist's
  substrate exposition).
- Batanin 1998 (globular composition; N-fold tensor factoring).
- Nix RFC 49 (flake semantics; content-addressed derivations).
- Sheldrake 2020 (*Entangled Life* — mycelial framing).

---

**Word count:** ~2350. Every claim cited with file:line + OID
where landed, or explicit forward-promise / spec-only naming
where not. Recommended first landing: **Rung 0 (Mara mints
`shards/song/beat.mirror`) → Rung 1 (Reed lands `--song` flag +
`bootstrap/src/song.rs` module + `peer_beam_song_single_beat_shard.rs`).**
