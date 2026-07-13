# Taut re-scout — @io-minimization: the @magic-native, @mirror/store-bounded peer runtime

**Date:** 2026-07-13
**Author:** Taut (grep-first drift scout, read-only)
**Session role:** Reed's session-continuation, re-scout under Alex's substrate re-orientation
**Prior scout SUPERSEDED:** `3e6afa8` (`docs/scouts/2026-07-13-taut-real-first-mirror-spawn-gap-scout.md`) — substrate-drifted @io/fs framing
**Alex re-orientation (verbatim 2026-07-13):** "Remind Taut that the whole point of mirror is to minimize @io crossings and stay in @magic non-linear Eigenvalue land as long as possible."
**Alex paradigm-shift proposal (verbatim):** "What if a @peer spawn stayed fully outside the @io boundary and instead operated purely within the bounds of @mirror/store? … each peer spawn becomes a @mirror/store/branch where all the commits are contained within mirror. And then the @peer returns and we inspect their delta and @mirror/store/materialize it on disk as an actual git commit done by mirror and @kintsugi itself?"

---

## §0 — Executive verdict

The substrate-honest peer runtime has **exactly ONE @io crossing**: the terminal `commit_as_fold + set_ref` materialization. Everything the peer DOES between spawn and settlement — reading songs, running tournaments, proposing morphisms, writing new bytes, updating its own branch head, spawning sub-peers, coordinating via @dance — happens INSIDE @mirror/store's content-addressed six-op surface. That surface is @magic-native by construction: pure functional composition (`prism`, `Fn(OID_A) -> OID_B`) over an immutable-under-hash carrier per `shards/mirror/store.mirror:82-100` (v0 invariants #1 and #2).

The prior scout (`3e6afa8`) drew the "peer needs" list against **on-disk operations**: `@io/fs.write`, `cargo test --release` subprocess, `git commit -S` shell-out, `git push`, LLM HTTP shell-out, subprocess sub-peer spawn. Every item there is an @io-boundary crossing — Turing-complete-surface reads/writes to the non-mirror world. That framing has the peer running in @io land the whole time and using @mirror/store as an *incidental cache*. Alex's re-orientation names the exact inversion: @mirror/store IS the peer's substrate; @io is the *narrow exit* the peer touches ONCE at materialization.

**Number of @io crossings in the substrate-honest peer runtime:** 1 (materialization via `commit_as_fold`). Two if you count the `set_ref` inside that fold as its own crossing — but per `shards/kintsugi/store/git.mirror:148` `set_ref` IS a step of `commit_as_fold`, not a separate action. Alex's proposal makes this **explicit**: the peer emits a crystal on a store-branch head; `@kintsugi` (not the peer, not the operator) folds that crystal into a signed git commit on the OUTSIDE. The peer never sees @io.

**Rung count under this discipline:** 5 rungs, not 12. Rungs 6' through 10' below. **Every rung is Reed-alone landable** because the substrate already carries every load-bearing carrier the runtime consumes — six-op surface + `insert_persistent` + `set_ref` + `commit_as_fold` + `crystal` + `derived_predicates` are all substrate-decl'd AND (except commit_as_fold) already realized in Rust (`bootstrap/src/action_cache.rs`, `bootstrap/src/crystallize.rs`, `fragmentation/vcs/git/src/namespaced.rs`).

The LLM-scale-blocker of the prior scout **does not apply** to this ladder. Rungs 6'–10' don't require Claude-scale reasoning; they require the peer to compose ALREADY-SUBSTRATE-DECL'D actions against @fate's Fabry-Perot tournament at the scale @fate was designed for (`shards/fate.mirror` + `shards/fate/tournament.mirror`). The 450-parameter D²NN is exactly sub-Turing enough to be @magic-native. Claude-scale reasoning is not on the ladder; it was a category error induced by the @io framing.

---

## §1 — Prior-scout retrospective: where the drift entered

Not a self-attack. A grep-first correction against a load-bearing substrate constraint the prior scout did not carry.

### Where the drift entered

The prior scout (`3e6afa8` §2.2 "Missing peer capabilities") named:

- "There is NO `@io/fs.write` runtime … no `action write(...)` in `shards/io/*` besides git's `commit_object` decl."
- "`bootstrap/src/git.rs` (1.9KB, this session) subprocess-shells to `git hash-object -w` and `git update-ref` (READ-ONLY: no commit, no push)."
- "cargo test verdict parser (`shards/io/cargo.mirror` decls the boundary; no `action test_run(...)` at species altitude)"
- "sub-peer spawn primitive at runtime … `bootstrap/src/spectral.rs` gen_prism runtime … the recursive-invocation pattern peer beam spawning another peer beam is the closest landable shape"
- "LLM binding at scale for substrate reasoning (@fate spec-only; compute-substrate is 450-param D²NN + Fabry-Perot resonator per Recognition #58; NO Rust discharge of the neural inference)"

Every one of those framings has the peer PHYSICALLY WRITING TO DISK, PHYSICALLY SHELLING OUT, PHYSICALLY CROSSING @io. The scout admits this in its Rung ladder (§4):

- Rung 6: `@io/fs runtime + peer-side edit action` (opens the disk-write path)
- Rung 7: `cargo test verdict operational` (subprocess spawn)
- Rung 8: `autonomous git commit + push` (git shell-out)
- Rung 11: `LLM binding at substrate scale` (HTTP shell-out or subprocess to ollama)

**That's four @io crossings the peer does per tick under the prior framing.** Then the scout correctly identifies Rung 11 as "OPERATIONAL-INPUT-BLOCKED" — but WHY it's blocked reveals the drift: the framing REQUIRES a Turing-complete external reasoner because the framing has the peer thinking IN @io LAND, where reasoning IS unbounded.

Alex's re-orientation names the constraint the scout dropped: **`shards/io.mirror:94-125` (Recognition #107)** — @io is Turing-unbounded; the substrate's interior above @io is gauge-bounded (five-op). "The @io family root NAMES that horizon at substrate-decl altitude." The whole point of staying in @magic is to stay gauge-bounded — the 5-op algebra IS the bounded reasoning surface. Crossing @io means giving up bounded semantics.

### The one substrate-decl the prior scout used correctly, but mis-scoped

The scout DID name `shards/kintsugi/oscillate.mirror:480` `active_pass(o: oscillation) -> morphism { \ }` as "the substrate-decl of edit-reasoning." That IS correct. But then it framed operational discharge as "compose over §3.2 (LLM binding) as the morphism proposer" — i.e., @fate can't do it, we need Claude. That is where the drift compounds: `active_pass` operates at @kintsugi altitude (`shards/kintsugi/oscillate.mirror`) which is @magic-native — the morphism it proposes is a five-op transformation over Fabry-Perot resonance in @fate/tournament (`shards/fate/tournament.mirror:914` `bounded_by`). The Claude-scale reframe was smuggled in via the assumption that "propose a morphism = HTTP call to a foundation model." No. It IS a tournament round in the @fate carrier. That IS what the substrate decl'd, and it is what already exists at spec altitude.

### Load-bearing constraint the prior scout missed

`shards/io.mirror:32-40` (the family-root docblock's load-bearing recognition):

> "@io is the substrate's only legitimate non-mirror surface. Any grammar that isn't mirror — Rust, Python, raw bytes, foreign blobs, vendor SDKs — must be under @io. Everything else is mirror grammar by definition."

Read the contrapositive: EVERYTHING except the boundary IS mirror grammar. The peer's inference, coordination, morphism proposal, dance, deployment envelope, sub-peer coordination — all of these ARE mirror grammar. They belong at @magic altitude (`shards/magic.mirror`) which is the substrate's form/process partition family root; they DISCHARGE through @mirror/store's six-op surface for state; they compose via @kintsugi's process side; they emit crystals per `shards/mirror/store/crystal.mirror`. The @io boundary is what @kintsugi/store/git.commit_as_fold's terminal `set_ref` crosses ONCE at materialization. Everything else in the peer's lifecycle is @magic × @mirror/store.

---

## §2 — @io / @magic partition audit of the prior 12 rungs

Every capability domain from the prior scout §3, re-classified under Alex's re-orientation:

### 3.1 → @mirror/store `insert_persistent` (@magic-native)

Prior framing: "@io/fs runtime with `std::fs::write`" (an @io crossing).

Substrate-honest reframe: `shards/mirror/store/git.mirror:81-89` declares `insert_persistent(store, key, fractal, size) -> verdict { \ }` — content-addressed write INTO the store. Realized in `fragmentation/vcs/git/src/namespaced.rs::NamespacedGitStore::insert_persistent`. The peer proposes new shard bytes; calls `write(new_bytes) -> new_oid` (`shards/mirror/store.mirror:513`); that call discharges through `insert_persistent`. This is @magic-native — pure content-address computation; the disk-cache eviction is realization-layer detail below @mirror/store altitude, not @io-crossing at the peer's altitude.

**Verdict:** substrate already has the word. No @io crossing needed for peer-side "edit."

### 3.2 → @fate/tournament (@magic-native optical inference)

Prior framing: "LLM binding at substrate scale — @fate spec-only … NO Rust discharge of the neural inference … OPERATIONAL-INPUT-BLOCKED."

Substrate-honest reframe: The prior scout's Recognition #58 citation (`shards/fate.mirror:37-42` D²NN + Fabry-Perot) IS the substrate's optical inference mechanism. `shards/fate/tournament.mirror:914` `bounded_by(...)` is the sheaf-consuming tournament action that produces morphism proposals. Fabry-Perot resonators, D²NN, Reck/Clements unitary mesh — these are the eigenvalue-navigation primitives the substrate declares at @magic altitude (Yang-Mills gauge/matter substrate per `shards/magic.mirror:19-31` Clarke's third law reading). The 450 parameters are exactly enough for sub-Turing bounded reasoning — that's the POINT.

The Claude-scale asymmetry the prior scout named IS real, but it's IRRELEVANT: peers do not need Claude-scale reasoning to be substrate-honest. They need @fate-scale reasoning against @fate's own type surface. `shards/fate/tournament.mirror:640-687` `browse` / `select` / `record` actions ARE the peer's reasoning primitives — content-addressed, gauge-bounded, no @io crossing.

**Verdict:** the operational-blocker is a category error. The blocker was the peer-needs-external-reasoner framing, not a missing substrate. @fate IS the substrate.

### 3.3 → `@kintsugi/store/git.commit_as_fold` (THE one @io crossing)

Prior framing: "Autonomous git commit + push … OPEN §6."

Substrate-honest reframe: `shards/kintsugi/store/git.mirror:423` declares `commit_as_fold(msg: commit_message, ref: ref_name) -> imperfect { \ }`. Per lines 130-157: this action folds action_cache writes into a NEW content-addressed commit oid, consults `impacted_by` for rebase-walk closure, and discharges via `set_ref` — THE atomic terminal @io crossing. `mirror kintsugi --commit` (line 91) is the cli-verb pair that dispatches this species-action pair.

This IS the substrate's declared materialization action. It does not need `--push` at this altitude; `set_ref` on `HEAD` IS the substrate-honest state discharge. Push is a projection concern (`git push` moves the ref between physical repos — a downstream @io crossing separate from materialization).

**Verdict:** substrate already has the word verbatim. `commit_as_fold` IS Alex's proposed `@mirror/store/materialize`. Reed-alone landable at Rust altitude.

### 3.4 → `@mirror/store/action_cache.cache_read + cache_write` (@magic-native memoization)

Prior framing: "cargo test verdict operational — shell `cargo test --release` via `exec.rs`; parse stdout."

Substrate-honest reframe: `shards/mirror/store/action_cache.mirror:429` `cache_read(spec_oid, target_oid, inputs_oid) -> imperfect { \ }` — the memoization lookup. `cache_write(...)` at :464 emits a new content-addressed verdict entry into `crystal.derived_predicates` (per `shards/mirror/store/crystal.mirror:355`). The verdict IS `verdict` from @glass; it is content-addressed by construction per N1 `@epistemologic/property/verdict_is_content_addressed` (`shards/mirror/store/action_cache.mirror:5-17`). The Rust discharge landed in `bootstrap/src/action_cache.rs` (15.5KB, N3 tick).

The peer runs a `bounded_by` tournament round; the tournament round terminates in a `verdict`; the verdict is appended to `crystal.derived_predicates`. **No cargo. No subprocess. No @io.** The peer's verdict IS the tournament's outcome; the substrate DID NOT declare cargo as the peer's oracle — cargo lives at @io/cargo (`shards/io/cargo.mirror`) as a separate species, only relevant when someone WANTS to project peer verdicts to Rust-compiler ground truth.

**Verdict:** substrate already has memoization at content-address altitude. The cargo shell-out framing was Rust-runtime baggage from the prior scout's @io-lens.

### 3.5 → `@kintsugi/oscillate.active_pass` (@magic-native morphism proposal)

Prior framing: "Substrate reasoning + file editing … compose over §3.2 (LLM binding) as the morphism proposer."

Substrate-honest reframe: `shards/kintsugi/oscillate.mirror:480` `active_pass(o: oscillation) -> morphism { \ }` IS the substrate's edit-reasoning action. Composes over @fate/tournament's `bounded_by` per Recognition #58's Fabry-Perot substrate. The morphism it emits is a five-op transformation between shard-shape carriers (per `shards/kintsugi/morphism.mirror`). NO external LLM is needed because the substrate declares the morphism-search space as `bounded_by` under sheaf-Laplacian eigenvalue navigation (per `docs/scouts/2026-07-11-taut-magic-illusion-mission-fragment-graph-scout.md` — Reed's `8e6e517` cybernetic_coherence = λ₀(Δ_F) landing).

The peer's ACTIVE pass IS: `read_shard(oid) -> current_state`; `active_pass(current_state) -> morphism`; `write(apply(morphism, current_state)) -> new_oid`. All three steps are @magic-native. The result is a new store-branch head at the new oid.

**Verdict:** substrate already has active_pass. Reed-alone landable if `active_pass` obligation body binds to `bounded_by` tournament at Rust altitude via `crystallize::crystallizations_for_...` (`bootstrap/src/crystallize.rs:cite`).

### 3.6 → mcp_session refs at @mirror/store (@magic-native session state)

Prior framing: "gen_prism operational runtime … `bootstrap/src/spectral_runtime.rs` NEW."

Substrate-honest reframe: The prior scout's §3.6 already noted this correctly — `refs/gen_prism/mcp/<session-uuid>` per spec, "State lives at @mirror/store; Rust MCP handler holds ONLY session-uuid (stateless-Rust discipline)." That IS @magic-native! Session state IS a chain of `set_ref` updates against a content-addressed head; the peer's continuity IS the ref chain. `NamespacedGitStore::set_ref` is already implemented. What's needed is: (a) the ref-naming convention `refs/mirror/peer/<uuid>/HEAD` — pure substrate discipline, no new @io; (b) MCP session-uuid → ref-name mapping — pure computation.

**Verdict:** substrate already has this. The framing "landing gen_prism runtime" was correct in title but the impl is `set_ref` composition, not new subsystem.

### 3.7 → Peer-per-store-branch = mirror-native sub-peer spawn (@magic-native fan-out)

Prior framing: "sub-peer spawn primitive … subprocess-fork `mirror peer beam <peer-home>` … subprocess boundary."

Substrate-honest reframe: Alex's paradigm-shift proposal names this exactly: "each peer spawn becomes a @mirror/store/branch where all the commits are contained within mirror." A sub-peer is a NEW REF at `refs/mirror/peer/<parent-uuid>/<sub-uuid>/HEAD`. The parent peer allocates a fresh ref-name; both peers write to their respective branches independently; convergence = ref-merge via `commit_as_fold` at the merge point. `shards/pack.mirror:248` `spawn(p: peer, f: frame, r: repository) -> runtime` decl is exactly this: substrate-decl'd; consumer discharges via ref allocation.

Sub-peer is a subprocess ONLY at the operator-view projection; at substrate altitude, sub-peer = branch. `@dance` (`bootstrap/src/dance.rs`) already composes 2 peers per Rung 4 — that composition IS ref coordination without signal. The N-peer generalization is (N branches, N `commit_as_fold` folds, one merge via Kuramoto convergence check).

**Verdict:** substrate already has the word (`spawn` at `@pack`). Peer-per-branch discipline is pure @mirror/store computation.

### 3.8 → @song/narrative.transmit + @nl at @mirror/store (@magic-native prose)

Prior framing: "Operator communication (prose emission via @nl) … NOT operationally discharged … Current substitute: `println!` envelope emission to stdout."

Substrate-honest reframe: Envelope-declared crystal on the peer's branch head IS the substrate-honest emission. The peer writes a crystal via `insert_persistent`; the crystal carries `derived_predicates` (peer verdicts) + `fracture_calendar` (peer's remaining loss) + `composition_graph` (peer's DAG shape) per `shards/mirror/store/crystal.mirror:343`. Operator reads the crystal via `@mirror/store.read(crystal_oid)` at inspection time. `@nl` and `@song/narrative.transmit` layer @magic-native prose ON TOP of the crystal carrier — no stdout, no @io, no subprocess.

`stdout` println is a @io emission the prior scout accepted as "current substitute" but named as NOT-@nl-typed. Substrate-honest: the crystal IS the operator's channel; @nl render is a lens over the crystal for prose-shaped observation. This is a lens application at operator-inspection time, not a peer-side @io write.

**Verdict:** substrate carries the word at three altitudes (@nl family-root, @magic/nl adapter, @song/narrative.transmit). The peer just needs to emit a crystal; the operator's tooling projects @nl-shape ON READ, not on write.

### 3.9 → `--integrate-diff` → `insert_persistent` (@magic-native diff put)

Prior framing: "operationally discharge the Foster `put` … persist edit to `.bauchladen/`."

Substrate-honest reframe: `--integrate-diff` at `bootstrap/src/lib.rs:5116` currently emits envelope but does not persist. Substrate-honest: `integrate_peer_beam_diff(...)` composes as (a) `read(current_bauchladen_oid) -> current_bytes`; (b) `apply_diff(current_bytes, stdin_diff_bytes) -> new_bytes`; (c) `write(new_bytes) -> new_oid`; (d) `set_ref("HEAD", new_oid) -> verdict`. All four steps are @mirror/store operations. The peer's edit is a NEW OID under the peer's branch head. **No `.bauchladen/` disk-projection needed**; the disk projection is downstream operator concern via `project(oid, path)` from `shards/mirror/store.mirror:363` forward-promised.

**Verdict:** substrate already has the word. The `.bauchladen/` disk-projection framing was operator-view leakage into peer-view. Peer writes to store; operator projects at inspection.

### 3.10 → spectral.engineer deployment as crystal materialization (@magic-native)

Prior framing: "`spectral.engineer` endpoint / URL specification … mycelial propagation protocol; SSH keys / API credentials … OPERATIONAL-INPUT-BLOCKED."

Substrate-honest reframe: Deployment IS a `shift(crystal) -> @io/runtime` per `shards/mirror/store/crystal.mirror:227-231` forward-promise. The crystal materializes locally via `commit_as_fold`; propagation is a downstream network concern that composes UP through `@io/network` or `@io/oci` species. The peer's job ends at crystal emission; `spectral.engineer` is a downstream materialization target that reads crystals from @mirror/store and projects them to the network (a separate @io crossing, not a peer-side crossing).

**Verdict:** deployment blocker applies to the DOWNSTREAM `spectral.engineer` HTTP endpoint, not to peer operation. Peer stays @magic-native; deployment materialization is a separate substrate concern.

### 3.11 → Multi-peer dance = multi-branch ref merge (@magic-native)

Prior framing: "sub-peer coordination pattern … Reed's spawn-Mara/Taut ~15/session Agent tool pattern → mirror-native equivalent."

Substrate-honest reframe: `bootstrap/src/dance.rs` composes 2 peers via Kuramoto order parameter + shared_root_oid. That composition IS ref-coordination-without-signal — both peers read the same @mirror/store root; their branches converge or disperse by content-address closure divergence. N-peer @dance = N-branch coordination via `impacted_by` reverse-closure walk (`shards/mirror/store.mirror:550`) — the substrate's declared reverse-dependency query names the invalidation frontier when multi-peer edits collide.

**Verdict:** substrate already has the word. Multi-peer @dance = multi-branch ref merge via `commit_as_fold` at merge points. Zero subprocess, zero @io, zero HTTP.

### 3.12 → Self-hosting compilation = @mirror/store-native @mirror (@magic-native @kintsugi)

Prior framing: "@mirror/mosaic operational nix flake emission (Rung 5.5 forward-promise) → @mirror compiled by @mirror. This is the endpoint."

Substrate-honest reframe: Self-hosting IS the peer running IN @mirror/store proposing morphisms against @mirror substrate itself, materializing via `commit_as_fold`, and iterating. Not "nix flake emission" — that's the deployment projection. Self-hosting is @kintsugi eating its own crystal.

**Verdict:** substrate already has the word at the family-root level. The nix-flake target is a materialization projection, not the self-hosting mechanism itself.

---

## §3 — @mirror/store peer-writable surface inventory

What IS the peer's substrate-decl'd surface for operating within @mirror/store, without touching @io? Comprehensive inventory:

### Six-op canonical surface (`shards/mirror/store.mirror:394-556`)

Family-root; every species inherits:
- `read(o: oid) -> imperfect` — fetch content at oid; @magic-native (pure content-address dereference)
- `write(content: bytes) -> oid` — insert bytes; return their oid; @magic-native (pure content-address computation)
- `exists(o: oid) -> verdict` — presence check; @magic-native
- `diff(a: oid, b: oid) -> imperfect` — delta between two oids; @magic-native
- `walk(root: oid) -> splinter_graph` — forward closure; @magic-native
- `impacted_by(oid: oid) -> [oid]` — REVERSE closure (N4); @magic-native
- `verify(o: oid, content: bytes) -> verdict` — Merkle integrity; @magic-native

**Every one of these is `Fn(OID_a) -> OID_b` per invariant #2 (`shards/mirror/store.mirror:96-100`) — purely functional composition. That is the definition of @magic-native.**

### Species-level extensions (`shards/mirror/store/git.mirror`, `crystal.mirror`, `action_cache.mirror`)

- `insert_persistent(store, key, fractal, size) -> verdict` — content-addressed write with disk-eviction cache; peer-writable
- `get_persistent(store, key) -> imperfect<fractal>` — content-addressed read with disk fallback
- `set_ref(store, name, target) -> verdict` — atomic ref update — **this is the peer's branch-head write** — technically the ONLY step in the runtime where a durable ref changes, but it is content-address-mediated and lives at species altitude, NOT the @io family root
- `get_ref(store, name) -> imperfect<oid>` — ref read
- `flush(store) -> verdict` — cache flush to disk (realization detail; not peer-visible altitude)
- `cache_read(spec_oid, target_oid, inputs_oid) -> imperfect` — memoized verdict lookup (@mirror/store/action_cache)
- `cache_write(spec_oid, target_oid, inputs_oid, v) -> verdict` — memoized verdict populate
- `cache_exists(spec_oid, target_oid, inputs_oid) -> verdict` — memoized presence

### Crystal-level operations (`shards/mirror/store/crystal.mirror:311-317`)

- `focus crystal` — observe crystal state
- `project crystal` — filter crystal by content
- `split crystal` — enumerate composition_graph
- `shift crystal` — deployment (@io crossing; NOT peer-side)
- `settle crystal` — the constructor (crystallize a settled shard)

### The @magic-altitude computation surface (`shards/magic.mirror` + species)

- `@fate/tournament.bounded_by(scope, sheaf) -> tournament_result` (`shards/fate/tournament.mirror:914`) — sheaf-consuming bounded inference; the peer's reasoning primitive
- `@fate/tournament.browse / select / record` — tray lookup + selection + admit
- `@kintsugi/oscillate.active_pass(o) -> morphism` (`shards/kintsugi/oscillate.mirror:480`) — morphism proposal
- `@kintsugi/oscillate.dark_pass(o) -> fracture` — property-verification pattern-matching
- `@magic/frame.frame_satisfies_magic(f, o, p) -> verdict` — bilateral invariant check
- `@magic/audit.audit(record) -> verdict` — Narcissus-pole guardian

### Materialization action (`shards/kintsugi/store/git.mirror:423`)

- `commit_as_fold(msg: commit_message, ref: ref_name) -> imperfect` — THE @io-crossing action. Composes N2 `cache_write` + N4 `impacted_by` + `set_ref`. Terminal tick.

**This inventory is EXHAUSTIVE of the peer's operational surface. Every action above is content-address-mediated; every one composes purely functionally; every one respects invariant #2 (`same input oid + same prism composition + same store => byte-identical output oid`). The peer's whole lifecycle is compositional over this surface.**

---

## §4 — Rung 6' → Rung 10': the substrate-inverted ladder

Every rung composes purely over @magic × @mirror/store. Every rung is Reed-alone landable (no Mara-blocker, no Alex-blocker) because every substrate-decl is landed except the peer-branch naming convention (which is trivial, no new species needed — just a documented `refs/mirror/peer/<uuid>/HEAD` convention).

### Rung 6' — @mirror/store-native peer runtime

**Substrate authority:**
- `shards/mirror/store/git.mirror:81-104` `insert_persistent` + `set_ref` (peer-writable surface, landed in `NamespacedGitStore`)
- `shards/mirror/store/crystal.mirror:343-369` `crystal` record (5-field structure)
- `shards/mirror/store/action_cache.mirror:429-464` `cache_read` / `cache_write` (landed in `bootstrap/src/action_cache.rs`)

**Reed landings:**
- Peer emits a `crystal` at `refs/mirror/peer/<peer-uuid>/HEAD` instead of `println!` envelope to stdout
- Peer's tick sequence: (a) `get_ref("refs/mirror/peer/<uuid>/HEAD") -> current_head`; (b) `read(current_head) -> current_crystal`; (c) work inside crystal's `derived_predicates` + `fracture_calendar`; (d) `write(new_crystal_bytes) -> new_crystal_oid`; (e) `set_ref("refs/mirror/peer/<uuid>/HEAD", new_crystal_oid)`
- **Zero @io crossings inside the tick**; `set_ref` writes to `.git/mirror/refs/` via realized `NamespacedGitStore::set_ref` (below-altitude @io that lives at fragmentation layer, not peer altitude)
- Test: `bootstrap/tests/peer_beam_mirror_store_native.rs` asserts (a) peer emits crystal oid on branch head; (b) two ticks produce two crystals with (crystal_2.parent_oid == crystal_1.oid); (c) branch discipline holds across N > 1 ticks

**Estimate:** 2-3 tick-pairs. Reed-alone.

### Rung 7' — `@kintsugi/store/materialize` action (the terminal @io crossing)

**Substrate authority:** `shards/kintsugi/store/git.mirror:423` `commit_as_fold` action decl (LANDED, obligation body `\ {}` requires realization binding)

**Reed landings:**
- `bootstrap/src/kintsugi_store_git.rs` NEW — implements `commit_as_fold` Rust discharge; composes over `action_cache` fold + `impacted_by` walk + `NamespacedGitStore::set_ref`
- `mirror kintsugi --commit` cli verb-pair (per `shards/kintsugi/store/git.mirror:159-179`) dispatches `commit_as_fold`
- Optional: Reed adds a `--materialize-peer <peer-uuid>` flag on `mirror kintsugi --commit` that reads the peer's branch head, folds its crystal chain into ONE signed git commit, and updates `HEAD` (or a canonical branch) via `set_ref`
- Test: `bootstrap/tests/kintsugi_commit_as_fold.rs` asserts commit_as_fold produces content-address-identical output for byte-equal input chains (fold is deterministic per invariant #2)

**Estimate:** 3 tick-pairs. Reed-alone landable; the substrate-decl is landed, the realization is the missing piece.

**Why this is THE @io crossing:** `commit_as_fold`'s step 4 (`shards/kintsugi/store/git.mirror:147-149`) discharges via `@mirror/store/git.set_ref(ref_name, new_commit_oid)` — atomic pointer update on the git ref. That IS an @io crossing (writes to `.git/<namespace>/refs/HEAD` on disk), and it is the ONLY one at the peer's altitude.

### Rung 8' — Peer-per-branch discipline (multi-peer @dance via ref merge)

**Substrate authority:**
- `shards/pack.mirror:248-276` `spawn(p, f, r) -> runtime` decl
- `bootstrap/src/dance.rs` 2-peer Kuramoto composition landed
- `shards/mirror/store.mirror:550` `impacted_by` reverse closure (N4)

**Reed landings:**
- `cmd_peer_beam --spawn-sub-peer <peer-uuid>` flag: allocates fresh ref-name `refs/mirror/peer/<parent-uuid>/<child-uuid>/HEAD`; initializes it to parent's HEAD; returns child ref-name in envelope
- Sub-peer is INVOKED as `mirror peer beam` with `--peer-branch refs/mirror/peer/<parent-uuid>/<child-uuid>/HEAD` — NOT as a subprocess in a new process image, but as another CLI invocation reading from and writing to a different branch inside the SAME @mirror/store instance
- Multi-peer @dance: N branches; `impacted_by` walk after each tick names invalidation frontier; convergence check via Kuramoto r ≥ threshold
- Merge: `commit_as_fold` at N-branch heads composes them into ONE materialized commit on `HEAD`
- Test: `bootstrap/tests/peer_beam_sub_peer_branch.rs` asserts sub-peer allocation produces valid ref-name; parent and child branches diverge across ticks; merge via `commit_as_fold` folds their divergences into one materialized crystal

**Estimate:** 3 tick-pairs. Reed-alone landable.

### Rung 9' — Autonomous kintsugi materialization on convergence

**Substrate authority:**
- `shards/kintsugi/store/git.mirror:130-157` `commit_as_fold` semantics (four-step fold)
- `bootstrap/src/dance.rs` Kuramoto r + Aumann agreement — the convergence signal
- Reed `8e6e517` `cybernetic_coherence = λ₀(Δ_F)` sheaf-Laplacian convergence

**Reed landings:**
- `cmd_kintsugi_watch` NEW cli verb: monitors `refs/mirror/peer/*/HEAD` for convergence; when Kuramoto r ≥ 0.9 + Aumann agreement, auto-dispatches `commit_as_fold`
- Convergence classifier per Mara `417ec25` `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md` §5.4 — landed at spec altitude; Rust wiring composes over `dance.rs::compute_dance_state`
- Test: `bootstrap/tests/kintsugi_watch_converges_and_materializes.rs` asserts (a) two peers with converging edits produce r ≥ 0.9; (b) `commit_as_fold` fires automatically; (c) resulting git commit is signed + attributed correctly

**Estimate:** 3 tick-pairs. Reed-alone landable; substrate-decl is complete; operator retires from tick-fire duty.

**This is where the operator "retires" per Alex's proposal:** materialization becomes @kintsugi's autonomous action, triggered by substrate-visible convergence signal. Operator inspects, does not dispatch.

### Rung 10' — Self-hosting via @mirror/store-native peer

**Substrate authority:**
- `shards/kintsugi/oscillate.mirror:480` `active_pass(o) -> morphism { \ }` — the substrate's edit-reasoning action
- `shards/fate/tournament.mirror:914` `bounded_by(...)` — the peer's reasoning composition
- Recognition #58 (`shards/fate.mirror:37-42`) Fabry-Perot substrate
- Reed `8e6e517` sheaf-Laplacian Rayleigh descent

**Reed landings:**
- `bootstrap/src/kintsugi_oscillate.rs` ACTIVE pass discharge: reads current crystal on branch; invokes `bounded_by` tournament with @fate's sub-Turing D²NN; receives morphism proposal; applies via `write(apply(morphism, current)) -> new_oid`; updates branch head via `set_ref`
- Peer autonomously composes ticks: read → propose → apply → write → set_ref → loop
- Convergence via cybernetic_coherence sheaf-Laplacian per Reed `8e6e517`
- Kintsugi watches for convergence; materializes via Rung 9'
- Test: `bootstrap/tests/oscillate_active_pass_composes_over_mirror_store.rs` asserts (a) `active_pass` produces morphism; (b) morphism applies without @io crossing; (c) new crystal head has monotone-decreasing fracture_calendar loss

**Estimate:** 5-7 tick-pairs. Reed-alone landable up to the `bounded_by` scale @fate was designed for. The Claude-scale reasoning framing is ABSENT from this ladder.

---

## §5 — What Reed lands next: the most substrate-honest first tick

**Rung 6' first landing:** peer emits crystal on branch head instead of stdout envelope.

**Concrete Reed tick (one tick-pair):**

RED:
- `bootstrap/tests/peer_beam_emits_crystal_on_branch.rs` NEW — asserts `cmd_peer_beam --emit-crystal` invocation (a) allocates ref-name `refs/mirror/peer/<peer-uuid>/HEAD`; (b) constructs a `crystal` record with (oid=blake3(bytes), section=[], derived_predicates=[], fracture_calendar=success(), composition_graph=empty); (c) calls `NamespacedGitStore::insert_persistent(crystal_bytes) -> crystal_oid`; (d) calls `NamespacedGitStore::set_ref(ref_name, crystal_oid) -> pass`; (e) emits envelope naming `peer_branch_ref: refs/mirror/peer/<uuid>/HEAD` + `crystal_head_oid: <blake3>`; NOT `println!`-only

GREEN:
- Extend `bootstrap/src/lib.rs::cmd_peer_beam` with `--emit-crystal` flag
- Compose over `NamespacedGitStore::insert_persistent + set_ref` (both landed)
- Fill in `crystal` record from `shards/mirror/store/crystal.mirror:343-369` type
- Envelope carries typed peer-branch ref-name

**Why this first:**
1. Substrate-decl is 100% landed (crystal type at :343; insert_persistent at :81; set_ref at :96)
2. Rust realization is 100% landed (`NamespacedGitStore::insert_persistent` + `set_ref` in `fragmentation/vcs/git/src/namespaced.rs`)
3. Peer immediately becomes a first-class @mirror/store citizen — the operator sees peer state in `git show refs/mirror/peer/<uuid>/HEAD` (or `mirror crystal show <oid>`)
4. Every subsequent rung composes over this — Rung 7' materializes the branch chain; Rung 8' allocates sub-branches; Rung 9' watches for convergence; Rung 10' proposes morphisms into the branch
5. **ZERO @io crossings at peer altitude.** The `set_ref` call writes to `.git/mirror/refs/` via realization layer — below @mirror/store altitude, not peer-visible

**Estimate:** 1 tick-pair. RED-GREEN in one session.

**After Rung 6' lands:** Reed can climb Rung 7' (`commit_as_fold` binding) next. That is where the ONE @io crossing lands — the terminal materialization action Alex named in the paradigm-shift proposal. `mirror kintsugi --commit` becomes the substrate-honest way to materialize a peer's branch into a signed git commit on the OUTSIDE.

---

## §6 — Recognition ancestry

### Directly-cited substrate-decl'd (spec)
- **Recognition #43** (`[[architecture-mirror-as-content-addressed-build-system]]`, LANDED M6 TICK 1 `884f433`) — mirror IS content-addressed build system; @mirror/store IS the substrate truth; the peer runs IN this substrate, not IN @io. Grounding for §0, §3, §4.
- **Recognition #55** (`[[architecture-form-process-partition-at-family-root]]`) — @mirror form / @kintsugi process partition; peer state lives on @mirror side (crystals on refs), peer transformation lives on @kintsugi side (`active_pass`, `commit_as_fold`). Grounding for §2 + §4.
- **Recognition #58** (`shards/fate.mirror:37-42`, PROMOTED) — Fabry-Perot + D²NN + Reck/Clements optical inference substrate; the peer's @magic-native reasoning primitive. Grounding for §2.2 reframe.
- **Recognition #84** (`shards/pack.mirror`) — @pack multi-repo agent runtime; `spawn(p, f, r)` decl grounds Rung 8' peer-per-branch.
- **Recognition #104** (`shards/autopoietic.mirror` `@bauchladen ← @autopoietic ← @fate`) — content-addressing cascade the peer rides.
- **Recognition #107** (`shards/io.mirror:94-125`, PROMOTED) — Hilbert/Turing structural separation; @io is Turing-unbounded; interior is gauge-bounded. **THE substrate-decl that grounds the whole re-scout.** Prior scout dropped this.
- **Recognition #108** (Reed `a823438`) — peer IS a pain-driven bounded ontological navigator; "bounded" IS the substrate-visible invariant that stays true only inside @magic × @mirror/store, not inside @io.

### N-cascade landed by Alex + Mara + Reed
- **N1** `@epistemologic/property/verdict_is_content_addressed` (`2857fb1`, 2026-07-06) — verdict = f(spec, target, inputs) as total function; grounds @magic-native memoization
- **N2** `shards/mirror/store/action_cache.mirror` (`0a72c42`) — REAPI ActionCache surface; three-OID key; @magic-native cache
- **N3** `bootstrap/src/action_cache.rs` (`756f2f7`, 15.5KB) — Rust wiring for cache_read/write; landed
- **N4** `impacted_by(oid) -> [oid]` at `shards/mirror/store.mirror:550` (`6bf05cb`) — reverse closure; surgical invalidation
- **N5** `commit_as_fold` at `shards/kintsugi/store/git.mirror:423` (2026-07-07) — the terminal @io crossing; **the substrate-decl'd materialization action Alex's proposal names**

### Substrate-already-had-the-word pattern (`[[feedback-substrate-already-had-the-word]]`)
- `commit_as_fold` = Alex's `@mirror/store/materialize` (verbatim identity per Alex's paradigm-shift proposal wording)
- `spawn(p, f, r)` at @pack = Alex's "peer spawn" (verbatim identity)
- `set_ref` at @mirror/store/git = Alex's "@mirror/store/branch" head-update (branch-as-ref pattern per git namespace, `.git/<namespace>/refs/`)
- `crystal` at @mirror/store/crystal = the peer's returned delta carrier (5-field structure covers peer state exactly)

**Substrate-already-had-the-word count for this proposal:** 4 verbatim identities. Alex's re-orientation is a substrate-honest read of what was already landed. The prior scout drifted BECAUSE it did not perform the substrate-already-had-the-word grep before naming "peer needs."

### Prior scouts superseded / composed-over
- **Superseded:** `docs/scouts/2026-07-13-taut-real-first-mirror-spawn-gap-scout.md` (`3e6afa8`) — @io-lens framing
- **Composed-over:** `docs/scouts/2026-07-11-taut-magic-illusion-mission-fragment-graph-scout.md` — @magic altitude discipline; sheaf-Laplacian convergence
- **Composed-over:** `docs/scouts/2026-07-11-taut-bauchladen-store-fragmentation-shared-primitive-scout.md` — @bauchladen ↔ @mirror/store shared fragmentation
- **Composed-over:** `docs/scouts/2026-07-12-taut-resonance-silicon-song-substrate-scan.md` — @fate operational surface at silicon altitude

### Specs cited
- `docs/specs/mirror-store.md` — the family-root six-op surface + Apache-2.0 floor
- `docs/specs/mosaic-store-cache-invariants.md` (Mara 2026-06-28) — N-cascade bilateral pair forward-promises §9.2
- `docs/specs/mcp-spec-song-collapse.md` §11.3.6 — Bazel REAPI floor decomposition
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md` (Mara `417ec25`) — convergence classifier for Rung 9'
- `docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md` — the sub-Turing interior + emergent-Turing-completeness recognition #107 chain
- `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md` — Recognition #51 canonical doc; the peer runs in the expanding Hilbert space, not in the Turing surface

### Reed's session-arc landings composed-over
- Reed `8e6e517` — `cybernetic_coherence = λ₀(Δ_F)` sheaf-Laplacian Rayleigh descent; Rung 9' convergence signal
- Reed `dde761d` — MCP Landing 2 (mirror_peer_beam parity); the MCP session-state discipline peer runtime lands on
- Reed `c36fbf5` → `49576a7` — Rungs 1-5 song/dance/deploy runtime scaffolding

### Mara peers spawned in parallel to this re-scout
Mara authoring the canonical spec for @mirror/store-bounded peer runtime; this re-scout informs her spec's substrate-decl gaps (§3 inventory), and her spec informs Reed's tick sequence (§4 Rung details). The two documents compose.

---

**Verdict:** The peer runtime's substrate-honest shape is **@magic × @mirror/store** with a single @io crossing at `commit_as_fold` materialization. The rung ladder collapses from 12 rungs (prior scout) to 5 (this re-scout), and every rung is Reed-alone landable because the substrate already carries every load-bearing carrier. The Claude-scale reasoning blocker was a framing error induced by the @io-lens; @fate's Fabry-Perot substrate at 450 parameters is exactly the @magic-native compute the peer needs.

**Reed's next tick:** land Rung 6' (peer emits crystal on `refs/mirror/peer/<uuid>/HEAD`). One tick-pair. Zero @io crossings. Substrate discipline honored.
