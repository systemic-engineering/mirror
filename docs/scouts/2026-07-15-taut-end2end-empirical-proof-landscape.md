---
date: 2026-07-15
author: Taut
scope: The end2end empirical-proof landscape — mapping the substrate's current @roomba/@kintsugi/@io surfaces against the theorem "the compiler observes → decides → mutates → composes → commits a real delta." Alex Wolf named the frame 2026-07-15 in-transcript; this scout is the map, not the build.
status: scout
companion:
  - docs/scouts/2026-07-14-taut-roomba-substrate-walker-scout.md
  - docs/scouts/2026-07-15-taut-kintsugi-ouroboros-substrate-scout.md
  - docs/scouts/2026-07-15-reed-rust-extension-migration-map.md
  - docs/loop/CURRENT.md
---

# Taut scout — end2end empirical proof landscape: from `mirror roomba --commit` facsimile to real @roomba × @kintsugi × @io round-trip

*Grep-first drift scout. Read-only. Substrate-already-had-the-word
discipline enforced. Alex Wolf named the frame verbatim 2026-07-15;
this scout maps what LANDED vs SPECCED vs MISSING between the current
`mirror roomba --commit` (observation-only facsimile) and the
theorem-proving round-trip (observe → decide → mutate → compose →
commit real delta). Reed writes the missing pieces after this lands.*

---

## §0 Prelude — the frame

### Alex 2026-07-15 verbatim (load-bearing)

> "I want the empirical proof to really be a roomba walk across the
> graph. Bumping into things. kintsugi resolving the fracture. And
> then the commit being the DELTA of that resolution translated into
> @nl language and of course as the blobs in the commit tree,
> actually committed to disk. It's the whole pipeline end2end. You
> run the roomba. The code simplifies in front of your eyes. When
> this works the rest is almost trivial. Because the theorem has
> been empirically demonstrated to work. And then we inspect the
> delta together."

### Reed's honest audit (anchor)

Current `mirror roomba --commit` (post-d88d050 + 855ea87):
- **AUTHOR identity** `mirror <mirror@spectral.engineer>` — REAL; empirically in git log (fcc1d75; per CURRENT.md commit 55).
- **Observation** (roomba walk + Fiedler + rust_loc) — REAL Rust @io work.
- **@io/git.commit dispatch** — SUBSTANTIVE; genuinely shells out via `apply_h::act` at `bootstrap/src/apply_h.rs:794-849`.
- **@nl.compose dispatch** — NOOP; echoes input. Resolver at `apply_h.rs:743-770` re-emits the arg's oid unchanged via `Transparency::located_opacity` under key `@nl/composed`. Comment there admits: "MVP: the caller (roomba_commit.rs) pre-serializes the observation beats into the first arg's oid string."
- **Composition** — Rust `format!()` in `roomba_commit::serialize_observation_beat` (`bootstrap/src/roomba_commit.rs:140-180`); the dispatch is theater.
- **Commit tree** — `--allow-empty`; NO delta blobs; observation-only. `create_commit` (`roomba_commit.rs:225-265`) passes `"true"` as third arg to `@io/git.commit`; apply_h passes `--allow-empty` to git (`apply_h.rs:826-828`).

**The gap.** Substrate observes → composes → commits an EMPTY commit describing the observation. It doesn't RESOLVE anything or CHANGE anything. That's the facsimile Alex named.

### The theorem to prove empirically

The compiler can:
1. **Observe itself** (walk graph → identify fractures).
2. **Decide on resolution** (kintsugi picks: collapse THIS file into THIS shard).
3. **Apply the resolution** (mutate files: delete Rust; extend shard; wire callers).
4. **Compose the delta description** via @nl (from before/after state).
5. **Commit the delta** (blobs + tree + composed message).

If ONE round-trip lands empirically, the remaining ~20 BUSINESS_LOGIC bootstrap/src/*.rs files collapse via `for _ in 0..N { mirror roomba --commit }`. The 4 manual collapses today (Arc-2 Ticks 2.1-2.4: spectral_signature / coherence / peer_persistence / roomba) become a for-loop the compiler runs on itself.

---

## §1 Roomba walker — current output shape + fracture-detection surface

### 1.1 The walker's output structure (LANDED)

`bootstrap/src/roomba.rs::walk(root, budget, epsilon_pain) -> WalkTrajectory` (`roomba.rs:145-244`). Post-Arc-2 Tick 2.4 collapse, this file is the **@io-boundary FLOOR** — Dijkstra graph walking primitive; bilateral predicates lifted to shard body.

`WalkTrajectory` fields (`roomba.rs:104-125`):
- `steps: Vec<RoombaStep>` — ordered sequence.
- `coherence_at_start: f64` — Fiedler at walk start.
- `coherence_at_end: f64` — Fiedler at walk end. **Scope A read-only: equal to start.** Scope B (forward-promised) transforms substrate; end differs.
- `termination: WalkTermination` — `BudgetExhausted | NoUnvisitedNeighbors | EmptyGraph | RootNotFound`.
- `graph_node_count / graph_edge_count` — topology summary.

`RoombaStep` fields (`roomba.rs:85-101`) per step:
- `node_idx / node_label` — WHERE the walker landed.
- `sc_hex` — SC<5> hex (spectral coordinate at position).
- `tension: f64` — variance of pain across (current + neighbors).
- `pain: f64` — @cyberpunk/algedonic.sample_pain magnitude.
- `coherence_delta_from_previous: f64` — Fiedler gradient.
- `knife_verdict: KnifeVerdict` — `Stable | NearBoundary | Jumped` per @mirror/lens/knife.stable_within.
- `unvisited_neighbor_count: usize`.

### 1.2 What "bumping into things" means concretely

Per Alex 2026-07-14 composition + Mara `9bbebd2` §"The empirical claim":

- **Tension** = variance of pain across (current + neighbors) (`roomba.rs::compute_tension` at 358-378). Higher tension = more spectral incongruence between adjacent substrate nodes.
- **Walk step** = "move to highest-tension unvisited neighbor" (`roomba.rs:226-241`). The walker literally IS a tension-gradient-ascent Dijkstra.
- **Bump** = tension SPIKE at a node — the walker reaching a position where `tension` markedly exceeds the trajectory mean. Not currently thresholded / classified in Rust; the shard's four bilateral predicates witness the **trajectory-level property** (tension_monotone_descending) but nothing today emits a per-step "FRACTURE HERE" signal.
- **KnifeVerdict::Jumped** — the closest thing today to "bump" at step altitude. Fires when `stable_within(sc, pain_delta, epsilon)` says the pain gradient exceeded epsilon. Per `bootstrap/src/converge.rs::stable_within` (called at `roomba.rs:212`).

### 1.3 What "fracture" is concretely in the substrate

Grep verdict on `\bfracture\b` across shards/ + bootstrap/src/:

**As type carrier (Rust):** `bootstrap/src/kintsugi.rs::Fracture` (`kintsugi.rs:155-167`). Fields include the inherited `Gap` (from `bootstrap/src/gap.rs`) — carries Bateson level + substrate origin + audible-altitude tension summary — plus a "descent magnitude."

**As substrate family (shards):** `shards/kintsugi.mirror` names `@kintsugi/fracture` as forward-promised family (`kintsugi.mirror:145-153`). LANDED sub-species:
- `shards/kintsugi/fracture/gate.mirror` — opacity site rewriter (per-file gate marker)
- `shards/kintsugi/fracture/keyword.mirror` — keyword rewrite
- `shards/kintsugi/fracture/operator_match.mirror`
- `shards/kintsugi/fracture/parent_cycle.mirror`
- `shards/kintsugi/fracture/partials_align.mirror`
- `shards/kintsugi/fracture/symbol_lift.mirror`
- `shards/kintsugi/fracture/angle_to_paren.mirror`
- `shards/kintsugi/fracture/relocate.mirror` — **whole-shard relocation** (per-shard collapse, `kintsugi/fracture/relocate.mirror`). Load-bearing precedent: the "auto-fracture" that reads `opacity` from a pact-`failure(opacity_map)` and emits a morphism proposing the shards/ destination path + ancestry rebind.
- `shards/kintsugi/fracture/dark_count_monotone.mirror` — first ouroboros bite
- `shards/kintsugi/fracture/cold_compile_within_tolerance.mirror` — second ouroboros bite

**Common pattern.** A `@kintsugi/fracture/X` species reads `opacity` from `@glass` (a located-fact carrier surfaced by some pact's `failure(opacity_map)` verdict) and emits a `morphism` from `@kintsugi/consent` (a typed-candidate carrier for the resolution). Body pattern: `resolve_X(o: opacity) -> morphism { \ }`.

### 1.4 Tension threshold today

**None substrate-decl'd for fracture detection.** `shards/kintsugi/surface.mirror` defines `type tension = { site, claim, observed, delta, level, ... }` (`surface.mirror:481-486`) — a first-class carrier — but the walker's `roomba.rs::compute_tension` returns a bare `f64`; no bridge exists today mapping walker-tension → `@kintsugi/surface.tension`. The four `surface_class` variants (`ashby_mismatch | contradiction | conundrum | out_of_band` at `surface.mirror:499-504`) are the "which kind of fracture" classification the walker would emit at, but nothing today calls `surface_class`.

### 1.5 What's MISSING between the walker and "fracture detection"

- No per-step "fracture emitted" signal from the walker's Rust side.
- No bridge from `RoombaStep.tension: f64` to `@kintsugi/surface.tension` typed carrier.
- No `walk_yields_fractures` action on `@kintsugi/roomba` that returns `[fracture]` or `[tension]`.
- The four `walk_witnessing` bilateral predicates witness PROPERTIES of the walk (terminates cleanly, tension descends, coherence admissible, knife bounded). They do NOT surface WHICH sites the walker classified as fracture-eligible.

**Takeaway (§1).** The walker HAS the raw data (tension/pain/knife-verdict per step). It does NOT emit fractures. To bridge the gap for empirical proof: the walker (or a downstream shard) needs an action `identify_fractures(trajectory) -> [fracture]` that thresholds tension AND labels the top-K sites with `surface_class`.

---

## §2 Kintsugi resolution surface (LANDED vs SPECCED)

### 2.1 The @kintsugi family

`shards/kintsugi.mirror` — family root; declares the form/process partition; `@kintsugi` = process-side (transformation).

**Sub-species LANDED:**
- `@kintsugi/oscillate` (`shards/kintsugi.mirror:121-140` name; body at `bootstrap/src/oscillate.rs` — 144.8KB Rust, BUSINESS_LOGIC) — the kintsugi LOOP primitive (`active_pass` / `dark_pass`).
- `@kintsugi/consent` (family root at `shards/kintsugi.mirror`; body carriers `morphism`, `query_phi`, `settle_or_pause`).
- `@kintsugi/roomba` (`shards/kintsugi/roomba.mirror` — 17KB; Arc-2 Tick 2.4 landing) — 4 base + 1 composed bilateral for walker properties.
- `@kintsugi/ouroboros` (`shards/kintsugi/ouroboros.mirror` — 26KB; landed today per CURRENT.md commit 6) — **substrate-decl'd but body-obligation-blocked**. THIS is the species that houses the collapse/verify/cutover surface Alex named.
- `@kintsugi/surface` (`shards/kintsugi/surface.mirror` — 34KB) — the compiler-error-surface classifier; 4 `surface_class` variants; `surface(t, cls, ctx) -> observation_depth` action.
- `@kintsugi/store/git` (`shards/kintsugi/store/git.mirror` — 21KB) — commit-as-fold semantics; declares `commit_as_fold(msg, ref_name) -> ...`.
- `@kintsugi/fracture/*` — 9 landed fracture-body species (§1.3 above).

### 2.2 @kintsugi/ouroboros — the load-bearing landing (SPECCED)

`shards/kintsugi/ouroboros.mirror` declares:

**Types:**
- `type collapse_target = { rust_file, mirror_target, verifiable, irreducible, ... }` (`ouroboros.mirror:213-251`)
- `type ouroboros_state = { targets, rust_loc, test_pass_rate, io_violations, ... }` (`ouroboros.mirror:257-298`)
- `type ouroboros_verdict = variant { collapse_admissible | collapse_refused_boundary_violation | collapse_refused_monotone_violation | collapse_pending_evaluator }` (`ouroboros.mirror:304-305`)

**Actions (ALL obligation-blocked `{ \ }`):**
- `collapse(target: collapse_target) -> ouroboros_verdict` — DECIDES if the target is collapse-admissible.
- `verify_same_output(rust: ref, mirror: ref, test: ref) -> verdict` — pre-cutover verification (Splinter-pole discipline).
- `cutover(target: collapse_target) -> verdict` — atomic replacement. Docstring at `ouroboros.mirror:376-384` names the composition:
  ```
  git rm bootstrap/src/F.rs
  git add shards/target/*.mirror   (if new shard)
  verify: mirror build still self-hosts
  verify: @torus.autonomy(compiler, cutover_winding) discharges
  emit verdict
  ```
- `ouroboros_step(state: ouroboros_state) -> ouroboros_state` — one atomic collapse cycle (`collapse + verify_same_output + cutover`).

**Bilateral predicates:**
- `collapse_admissible(before, after) -> verdict` — the proposed collapse satisfies `ouroboros_monotone` AND `verifiable_at_altitude`.
- `ouroboros_monotone(before, after) -> verdict` — THE LOAD-BEARING four-conjunct: rust_LOC non-increasing, test_pass_rate non-decreasing, io_violations non-increasing, sbec non-decreasing.
- `verifiable_at_altitude(target) -> verdict` — Rice-safe check via composition.

### 2.3 Path A vs Path B — Alex's fork ("@knife vs @peer at K+1")

Alex 2026-07-14 composition (Cross-referenced from `docs/insights/2026-07-14-alex-full-roomba-song-kintsugi-composition.md`; visible via `shards/kintsugi/roomba.mirror:43-58`):

> "@kintsugi consumes @song and decides: Path A: @knife the complexity (COORDᵢ → COORDⱼ; reduce); Path B: spawn @peer at K+1 (circular-reflexive question to developer OR higher-order @peer)."

**LANDED:**
- Path A/knife SUBSTRATE: `shards/mirror/lens/knife.mirror` (declared; `KnifeVerdict::Stable | NearBoundary | Jumped` in `converge.rs::stable_within`).
- Path B/peer SUBSTRATE: `@peer` family; `@peer/persistence` (`shards/peer/persistence.mirror`).

**SPECCED but NOT WIRED for kintsugi-decision dispatch:**
- Nothing in `apply_h::act` dispatches "given fracture X + surface_class Y, choose Path A or Path B."
- The `@kintsugi/ouroboros.collapse` action is the CLOSEST decision arm — it returns `ouroboros_verdict` variant. But `collapse` body is `\`-obligation-blocked. No resolver arm exists for `@kintsugi/ouroboros.collapse` in `apply_h.rs`.
- `@kintsugi/surface.surface(t, cls, ctx) -> observation_depth` (`kintsugi/surface.mirror:623-662`) classifies but does not DECIDE Path A/B.

### 2.4 What @kintsugi takes as INPUT to decide

Per `@kintsugi/ouroboros.collapse`: input is `collapse_target` (rust_file + mirror_target + verifiable + irreducible).

**This means the DECIDING structure is: "collapse target already NAMED before @kintsugi runs."** The mirror_target has to be pre-computed by the caller. The `@kintsugi/ouroboros.collapse` action verifies admissibility; it does not SEARCH for candidates.

**Gap.** Who computes the (rust_file → mirror_target) mapping? Per Taut #108 §D5: `@code/rust/materialize.classify(d: declaration) -> materialised_file` — reads a Rust file's AST and emits `partition = boundary | substrate` + `target: ref` naming the substrate altitude. **This is the classifier.** LANDED at `shards/code/rust/materialize.mirror` (per Taut #108 §D5 grep verdict; not re-verified this scout — trusted precedent). Rust-side draft in `bootstrap/src/realisation.rs::classify`.

**Takeaway (§2).** All the decision surface is DECL'D but nothing is WIRED. The ouroboros_step body is a `\`-obligation-blocked seven-step composition. To close: apply_h resolver arms for `@kintsugi/ouroboros.{collapse, verify_same_output, cutover, ouroboros_step}` need to exist. Today: zero apply_h arms dispatch @kintsugi/ouroboros or @kintsugi/surface — only @kintsugi/roomba's 5 bilateral predicates land.

---

## §3 Delta data-structure landscape

### 3.1 THE KEY FIND — substrate ALREADY HAD THE WORD

**`shards/epistemologic/reality/time.mirror`** (2026-06-06, 7.6KB) declares:

```mirror
# structural diff between two snapshots. not textual. the delta is
# a set of oid mutations.
type mutation = insert(ref) | remove(ref) | replace(ref, ref)
type delta = {
  from: snapshot,
  to: snapshot,
  mutations: [mutation],
  holonomy: loss,
}
```

(`shards/epistemologic/reality/time.mirror:120-127`)

Plus:
```mirror
type replay = {
  origin: snapshot,
  result: snapshot,
  delta: delta,
}
```

**Actions:**
- `compare(a: snapshot, b: snapshot) -> delta` (`time.mirror:151-152` per action docblock) — "Structural diff between two points in time. Returns the set of oid mutations, not a textual diff."
- `replay(s: snapshot) -> replay`
- `restore(s: snapshot, r: ref) -> imperfect` — restore specific ref from a past snapshot

**Substrate-already-had-the-word discipline: ENFORCED.** Delta = `{from, to, mutations, holonomy}`. Mutation = `insert(ref) | remove(ref) | replace(ref, ref)`. Substrate ancestry via `@epistemologic/reality/time`. This is a landed carrier for the collapse-delta shape the theorem needs.

### 3.2 What's NOT there yet

- **`compare` body is `{ \ }` obligation-blocked.** No Rust discharge. No apply_h resolver arm.
- **`snapshot` = `{ oid, tick, loss }`** (`time.mirror:110`). Not the same shape as "git commit" or "file byte state" — snapshot is spectral-tick point-in-time. A bridge from filesystem-state → snapshot would need to be authored.
- **`insert/remove/replace` operate over `ref`s** — content-addressed OIDs, not filesystem paths. The impedance from `git add / git rm` (path-oriented) to `delta.mutations` (oid-oriented) is a lift.

### 3.3 Other delta/diff surfaces landed

- `shards/optics/lens/diff.mirror` — **the load-bearing lens species** (17.5KB, 2026-07-12). Declares:
  - `type diff_bytes = ref` (unified-diff byte-carrier; @io-crossable)
  - `get(bauchladen) -> diff_bytes` — linearize peer state to diff bytes
  - `put(edited: diff_bytes, old_bauchladen) -> ref` — integrate edited diff back
  - Three Foster bilaterals + `autopoietic_closure`
  - Ancestry: `in @kintsugi/consent`, `in @mirror/store` — content-address altitude
  - All bodies `{ \ }` stubs; Rust runtime forward-promised at `bootstrap/src/optics/lens/diff.rs` (does NOT exist today per grep).
- `shards/mirror/store/git.mirror` — has `insert_persistent(store, key, fragment, size) -> verdict` (`store/git.mirror:346-374`). Wraps `NamespacedGitStore::insert_persistent` at `fragmentation/vcs/git/src/namespaced.rs`. This is CONTENT-ADDRESSED storage into `.git/mirror/`, NOT `.git/objects/`.
- Multiple `shards/epistemologic/pact/*.mirror` reference `diff_closure` etc. (`gate_matches_diff_closure.mirror:12`). None declare a `type diff` at substrate altitude.

### 3.4 Rust-side diff/delta surface

Grep for `\b(delta|diff|mutation|patch)\b` in `bootstrap/src/*.rs`:
- `bootstrap/src/roomba.rs`: `coherence_delta_from_previous: f64` — scalar delta, not structural.
- `bootstrap/src/roomba_commit.rs`: `coherence_delta: f64` — same.
- No Rust `struct Delta { ... }` or `struct Mutation { ... }` matching `@epistemologic/reality/time.delta`.
- No Rust discharge of `compare(a, b) -> delta`.

### 3.5 What the theorem's DELTA needs

Alex's frame: "the DELTA of that resolution translated into @nl language and of course as the blobs in the commit tree, actually committed to disk."

Concretely for a collapse round-trip:
- `mutations: [remove(ref_to_Rust_file_bytes), insert(ref_to_shard_body_bytes)]`
- `from: snapshot@HEAD_pre_collapse`
- `to: snapshot@HEAD_post_collapse`
- `holonomy: loss` — Fiedler delta as the substrate-native loss measure

The `@nl.compose` step then reads `delta.mutations` + before/after ouroboros_state and emits the commit message text. The `@io/git.commit` step actually commits with the file changes in the tree.

**Takeaway (§3).** `type delta` + `type mutation` LANDED at `@epistemologic/reality/time` (June 2026). `compare(a,b) -> delta` DECL'D. No Rust discharge. `type diff_bytes` LANDED at `@optics/lens/diff` (July 2026) with Foster bilaterals. No Rust discharge. The delta shape ALREADY substrate-honest; the wire from filesystem-mutation → delta.mutations is unauthored.

---

## §4 @io/git blob-writing / tree-building landscape

### 4.1 Currently WIRED via apply_h::act

**One arm only:** `@io/git.commit` (`bootstrap/src/apply_h.rs:794-849`).
- Args: `(message: ref, author: ref, allow_empty: bool)`.
- Realisation: shells `git -c user.name=X -c user.email=Y commit [--allow-empty] -S -m <message>`.
- Signs via SSH (`-S`); no key override per AGENTS.md.
- Assumes: caller pre-staged changes OR passes allow_empty=true.

### 4.2 @io/git shard surface (SPECCED, mostly `{ \ }`)

`shards/io/git.mirror` (25.9KB, 2026-07-15) declares:
- `clone(remote, target, p) -> imperfect(git_repository, ref, ref)`
- `fetch(repo, ref, p) -> imperfect(git_object, ref, ref)`
- `read_object(repo, hash, p) -> imperfect(git_object, ref, ref)` — content-addressed retrieval by hash
- `resolve_ref(repo, ref, p) -> imperfect(git_hash, ref, ref)` — symbolic → concrete hash
- `commit_object(repo, tree, parents, message, p) -> imperfect(git_hash, ref, ref)` — **plumbing form: creates commit from tree hash + parents + message**
- `commit(message, author, allow_empty) -> verdict` — porcelain form (this is what apply_h wires)
- `hash_to_oid(h: git_hash, p) -> ref` — bridge from SHA1/SHA256 git-hash to mirror-oid space

**Carriers:**
- `git_hash`, `git_ref`, `git_repository`, `git_object`, `git_ref_metadata`, `git_artifact`

**Bilaterals:** `git_reachable`, `git_repository_open`, `hash_well_formed`, `ref_well_formed`, `git_well_formed`

**All action bodies are `{ \ }` obligation-blocked; only `commit` has an apply_h resolver arm.**

### 4.3 What's MISSING from @io/git for real delta commits

- **No `write_blob` action.** No `hash_object` action. To write a new file blob into git's object database, the substrate has no substrate-decl'd primitive today. Closest: `commit_object` takes `tree: git_hash` — assumes tree already built.
- **No `write_tree` action.** No `update_index` / `stage_file` / `git_add` action.
- **No `rm_file` / `git_rm` action.**
- **`read_object` LANDED (spec) but not resolver-wired** — cannot RETRIEVE file contents by hash through @io/git today via apply_h.

### 4.4 @io/fs shard surface (SPECCED, `{ \ }`)

`shards/io/fs.mirror` (17.3KB):
- `read(p: path) -> imperfect { \ }`
- `write(p: path, bytes: ref) -> imperfect { \ }` ← **KEY. Can write files at the filesystem altitude. Not wired via apply_h.**
- `stat(p: path) -> imperfect { \ }`
- `readdir(p: path) -> imperfect { \ }`
- `mkdir(p: path) -> imperfect { \ }`
- **No `unlink` / `remove` action declared** despite docblock (io/fs.mirror:8-13) mentioning kernel `unlink` at the boundary.

### 4.5 @mirror/store/git surface (SPECCED via NamespacedGitStore)

`shards/mirror/store/git.mirror` (20.4KB):
- `open(path, namespace) -> imperfect<git_store, ref, transparency>` — opens `.git/<namespace>/` — NOT `.git/objects/`
- `insert_persistent(store, key, fragment, size) -> verdict` — content-addressed fragment insert
- `get_persistent(store, key) -> imperfect<fractal, ref, transparency>` — content-addressed fragment read
- `set_ref(store, name, target: oid) -> verdict` — write a named ref pointing at oid
- `get_ref(store, name) -> imperfect<oid, ref, transparency>` — read a named ref
- `flush(store) -> verdict`
- `path(store) -> ~d`

**Wired at Rust altitude via `fragmentation::NamespacedGitStore` (crate at `../../fragmentation/vcs/git/`), consumed in `bootstrap/src/git.rs`.**

### 4.6 @kintsugi/store/git — commit-as-fold

`shards/kintsugi/store/git.mirror` (21KB, 2026-07-12):
- `commit_as_fold(commit_message, ref_name) -> ...` — declared; body `{ \ }`.
- Semantics: terminal settlement of a kintsugi-loop batch: `commit_as_fold(msg, ref) = drain(action_cache) then git-commit`.
- Consumer target: `mirror kintsugi --commit` (per shard docblock 163-180).

### 4.7 Legacy `bootstrap/src/git.rs` (`git.rs:1-61`)

Two functions, 1.9KB, `#[cfg(all)]` unrestricted:
- `git_store_crystal(source_hash, crystal_oid)` — `git hash-object -w` + `git update-ref refs/crystals/<source_hash> <blob>` via shell
- `git_crystal_exists(source_hash)` — `git cat-file -p refs/crystals/<source_hash>`
- Callers: `bootstrap/src/lib.rs` (6 refs) + `bootstrap/src/store_branch.rs` (2 refs). Legacy crystal-cache path; predates NamespacedGitStore integration.

### 4.8 The critical gap for real delta commits

**The current @io/git.commit resolver in apply_h.rs shells `git commit` — WITH `--allow-empty=true`. It doesn't know how to stage file changes.**

For a real delta commit, either:
- **Option A:** The driver (Rust) makes file changes via `std::fs`, then calls `git add`, then calls @io/git.commit. This is what `mirror` binaries traditionally do; it's substrate-DISHONEST (bypasses @io/fs and @io/git substrate).
- **Option B:** Extend @io/git with `stage(paths: [ref]) -> verdict` + `rm(paths: [ref]) -> verdict` actions, wire via apply_h, then @io/git.commit passes empty allow_empty. Substrate-HONEST but requires ~3 new apply_h arms.
- **Option C:** Extend @io/fs with `write(p, bytes)` and `unlink(p)` apply_h arms. Compose: driver calls @io/fs.write for shard body + @io/fs.unlink for old .rs, then @io/git.commit (allow_empty=false; git picks up unstaged changes; use `-a` flag). Requires ~2 new apply_h arms + `-a` on git commit.

**Takeaway (§4).** @io/git.commit is the ONLY plumbing arm wired today. `commit_object`, `read_object`, `write_blob` (not declared), `stage/rm`, `@io/fs.write/unlink` are all `{ \ }` or unresolved. Real delta commits need EITHER git-plumbing arms wired OR fs-write arms wired.

---

## §5 Minimum-viable autonomous collapse — candidates + tractability

Ranked by tractability (least substrate authoring required):

### Candidate 1: Docblock stale-name rot (RANK 1 — SMALLEST GAP)

**The fracture.** Some documentation still references pre-etymology-rename names: `dispatch` where `act` should be; `execute` where `dispatch/act` should be; `read_ast` where `section` should be; `emit` where `utter` should be; `bench_record` where `crystallize` should be. Per CURRENT.md commits 44-49: 16 renames landed; cascade updates went across 5 shard files + docs + `docs/specs/io-secrets-projection.md` (85+ refs). Some drift remains — a walker looking for `\b(dispatch|read_ast|emit|bench_record)\b` in `bootstrap/src/**/*.rs` or `docs/**/*.md` would likely find stale text.

**Detection.** Walker greps files for the old name in prose/comment contexts (exclude code that intentionally references old aliases).

**Kintsugi decision.** Replace old→new using the canonical rename table (loaded from CURRENT.md commits 44-49 audit trail).

**Delta shape.** `mutation = replace(ref_to_bytes_with_old, ref_to_bytes_with_new)` per file touched.

**Rust extension needed:** MINIMAL. A grep + sed-equivalent (Rust `String::replace`) + `@io/fs.write` OR direct `std::fs::write`. Docs-only means `📝` bypass covers commits (per AGENTS.md `--no-verify` pure-docs bypass).

**Preexisting tooling in bootstrap/src/:** `bootstrap/src/lib.rs` already has `cmd_kintsugi_migrate` (`lib.rs:2552-2639`) that reads a `parse_rewrite(q)` rule + walks a directory + emits rewrites. This is 80% the machinery.

**Empirical-proof value:** MEDIUM. Docs-only rewrites are safe but not "the theorem" — no shard body composition proven; no Fiedler descent demonstrated.

### Candidate 2: Dead-code detection + safe deletion (RANK 2)

**The fracture.** Rust functions with zero callers across bootstrap/src/. E.g., `bootstrap/src/git.rs::git_crystal_exists` might have callers (verified: 2 sites in lib.rs). But `bootstrap/src/kintsugi.rs::minimize` — grep verdict: 0 callers in bootstrap/src/*.rs OTHER than kintsugi.rs itself (Bash search: `Kintsugi::minimize|kintsugi::minimize|::minimize\(` = `oscillate.rs:2 hits, sheaf_laplacian.rs:1 hit` — those may be different `minimize` symbols; verification needed).

**Detection.** Walker parses Rust AST (via `bootstrap/src/tokenize.rs` — FLOOR) and builds a call-graph. Any `pub fn X` with zero external references (in-crate) AND zero external `pub use` re-exports is a candidate.

**Kintsugi decision.** Delete the dead symbol AND all its supporting private helpers if they become orphaned.

**Delta shape.** `mutation = replace(ref_to_bytes_pre_delete, ref_to_bytes_post_delete)` per file OR `mutation = remove(ref_to_file)` for whole-file deletion.

**Rust extension needed:** MODERATE. Need a Rust call-graph analyzer OR shell to `rust-analyzer` / `cargo-udeps`. High risk of false-positive (macro-invoked code; tests-only; trait-impls).

**Preexisting tooling in bootstrap/src/:** `bootstrap/src/index.rs::build_concept_graph` builds a DAG over files but NOT over Rust symbols. AST walker in `ast.rs` + `tokenize.rs` for parsing, but no call-graph.

**Empirical-proof value:** HIGH. Real code shrinks; ouroboros_monotone.rust_loc empirically decreases. But detection is delicate (false-positives = broken build).

### Candidate 3: Superseded/deprecated shim removal (RANK 3)

**The fracture.** Files marked "// SUPERSEDED by shards/..." in prior ticks. Grep verdict: `SUPERSEDED|deprecated\b|DEAD-CODE|obsolete\b` in `bootstrap/src/**/*.rs`:
- `bootstrap/src/lib.rs:3203-3249`: `mirror spawn` is deprecated alias for `mirror peer beam`; two references (one merr! notice, one usage-message).
- `bootstrap/src/spectral.rs:2474-2479`: "The pre-meta-glass FP2 (keyword-table pruning) is obsolete" — CODE comment; the referenced code may still be present.

**Detection.** Walker greps for `\b(deprecated|SUPERSEDED|obsolete)\b` in doc comments AND checks if any callers still hit the deprecated symbol.

**Kintsugi decision.** Delete deprecated symbol + reroute callers to non-deprecated equivalent.

**Delta shape.** `mutation = replace(ref_lib_rs_pre, ref_lib_rs_post) + replace(ref_caller_1_pre, ref_caller_1_post)` per touched caller.

**Rust extension needed:** MODERATE. Same as Candidate 2 for call-graph.

**Empirical-proof value:** MEDIUM. Cleanup is visible; substrate honesty preserved (comment matches state); rust_loc decreases modestly.

### Candidate 4: Redundant Rust deletion post-shard-body dispatch (RANK 4 — the "canonical" ouroboros collapse)

**The fracture.** The 4 Arc-2 ticks (2.1-2.4) COLLAPSED spectral_signature.rs, coherence.rs, peer_persistence.rs, and roomba.rs — but ALL FOUR files STILL EXIST in `bootstrap/src/`. They shrank (docblock condensed) but were not deleted; the "@io-boundary FLOOR" primitives stay. Per Reed migration-map §6 Arc-3 Tick 3.1: "remove `pub mod coherence; pub mod roomba; pub mod spectral_signature; pub mod peer_persistence;` declarations." That's NOT what happened at Arc-2 ticks. Post-collapse each file KEPT its Rust primitives.

**True dead-code test.** Search for callers of each collapsed file's public `fn` from OTHER Rust files. If zero external callers exist (bilateral predicates now dispatch through apply_h::act; Rust primitives were called ONLY by the now-obsolete direct paths), the file is deletable.

**Detection.** Same call-graph as Candidate 2.

**Kintsugi decision.** For each file, verify all remaining `pub fn` have external callers. If any have zero, delete function. If all fn in a file have zero external callers AND the file is imported nowhere else, delete file + remove `pub mod X;` from lib.rs.

**Delta shape.** `mutation = remove(ref_to_deletable_file) + replace(ref_lib_rs_pre_mod_removal, ref_lib_rs_post_mod_removal)`.

**Rust extension needed:** HIGH. Same as Candidate 2 + must verify tests still pass post-deletion. Risk: bootstrap/tests/*_ouroboros_bite.rs tests reference these paths directly (per `kintsugi_roomba_ouroboros_bite.rs` docblock: "roomba_commit.rs still composes over the walker"). Full deletion would break tests.

**Empirical-proof value:** HIGHEST. This IS the canonical ouroboros — files that SHRUNK to their @io-boundary primitives get further squeezed as consumers migrate to apply_h::act dispatch. But: highest risk of breakage.

### Candidate 5: New shard-body migration (RANK 5 — the "next tick" pattern)

**The fracture.** A BUSINESS_LOGIC Rust file with an existing shard-decl that has NOT yet had bilateral predicates lifted. E.g., `bootstrap/src/song.rs` (11.4KB) + `shards/song.mirror` + `shards/song/beat.mirror`.

**Detection.** Walker finds Rust file with a shard-decl pair NOT in the {spectral_signature, coherence, peer_persistence, roomba} set already-collapsed AND for which bilateral predicates exist in the shard but do NOT dispatch through apply_h::act.

**Kintsugi decision.** For each bilateral predicate, add an apply_h::act resolver arm with sentinel-check pattern (matching the four Arc-2 ticks' pattern). Condense Rust docblock. sbec +N.

**Delta shape.** `mutation = replace(ref_bootstrap_src_X_rs_pre, ref_post) + replace(ref_apply_h_pre, ref_apply_h_post) + insert(ref_new_test_file)`.

**Rust extension needed:** HIGH. This is what Reed's been doing manually 4 times; automating requires substrate-decl parsing + apply_h.rs editing + test synthesis.

**Empirical-proof value:** HIGHEST. This IS the theorem in its full form — the compiler generates the next ouroboros bite. But: extreme substrate-authoring gap.

### Tractability ranking summary

| # | Candidate | Detect | Decide | Mutate | Rust auth needed | Proof value | Risk |
|---|-----------|--------|--------|--------|------------------|-------------|------|
| 1 | Docblock rename rot | trivial grep | table lookup | @io/fs.write | LOW | MED | LOW |
| 2 | Dead-code deletion | AST call-graph | zero-callers check | @io/fs.write | HIGH | HIGH | MED |
| 3 | Deprecated shim removal | grep | caller-migration | @io/fs.write × N | MED | MED | MED |
| 4 | Post-collapse file removal | call-graph + test-sim | zero-callers + tests-pass | @io/fs.unlink + @io/fs.write | HIGH | HIGHEST | HIGH |
| 5 | New shard-body migration | shard-parse + Rust-parse | predicate-lift pattern | @io/fs.write × 3+ | HIGHEST | HIGHEST | HIGH |

---

## §6 Gap analysis — what Reed would need to build per candidate

### Common infrastructure (all candidates need)

- **Extend @io/fs.write via apply_h resolver arm** (`apply_h.rs`): read arg[0]=path, arg[1]=bytes; `std::fs::write(path, bytes)`. Return `Verdict::Pass` on success.
- **Extend @io/fs with `unlink(p: path) -> imperfect { \ }` shard action** if whole-file deletion is required; add apply_h arm calling `std::fs::remove_file`.
- **Wire `@io/git.commit` to stage automatically OR add `@io/git.add(paths) -> verdict` arm** shelling `git add <paths>`. Alternative: modify apply_h's `@io/git.commit` arm to shell `git commit -a` (stages tracked-file modifications automatically) when a new arg `stage_all: bool` is passed. **Two-tick honest: adding `-a` breaks the existing 3-arg signature; better to add `@io/git.add`.**
- **Bridge from filesystem-state → `snapshot`** for `@epistemologic/reality/time.compare(a, b) -> delta`. Snapshot = `{oid, tick, loss}`. The `oid` for filesystem state can be `git rev-parse HEAD:<path>` or content-addressed via `hash_tagged`. The `tick` is a spectral tick — the existing @roomba walk-start / walk-end tick can carry it. The `loss` is Fiedler.

### Candidate 1: Docblock rename rot — specific gaps

- **Walker action:** `identify_stale_renames(root: path) -> [fracture]` — greps files for {old_name} from rename-table; returns list of (file, line, old_name, new_name). Not substrate-decl'd today.
- **Kintsugi action:** trivial — apply rename table.
- **Delta compose:** for each file touched, compute `mutation = replace(ref_pre, ref_post)`. Emit `delta = {from: snapshot_pre, to: snapshot_post, mutations, holonomy: fiedler_delta}`.
- **Commit:** shard body composes over @io/fs.write + @io/git.commit. Message via @nl.compose over serialized delta.

**Reed authoring estimate:** ~200 LOC Rust (walker + apply_h arms for @io/fs.write + @io/git.add). Small.

### Candidate 2: Dead-code deletion — specific gaps

- **Walker action:** `identify_dead_functions(root: path) -> [fracture]` — needs Rust call-graph. Either build over `bootstrap/src/ast.rs` (which today parses .mirror syntax, NOT Rust) OR shell to `cargo-udeps` / `rust-analyzer`. Neither is trivial.
- **Kintsugi decision:** verify Rust builds post-deletion (`cargo check`). Verify tests pass (`cargo test`). Both are @io boundary crossings.

**Reed authoring estimate:** ~800-1500 LOC. Rust call-graph analysis is nontrivial. HIGH.

### Candidate 3: Deprecated shim removal — specific gaps

- Combine Candidate 1's grep + Candidate 2's call-graph.

**Reed authoring estimate:** ~500-800 LOC. MEDIUM-HIGH.

### Candidate 4: Post-collapse file removal — specific gaps

- Requires Candidate 2's call-graph + test-suite simulation.
- Highest risk: bootstrap/tests/*_ouroboros_bite.rs reference collapsed Rust surface names; must be updated in same delta or protected as FLOOR.

**Reed authoring estimate:** ~1500-2500 LOC. HIGH.

### Candidate 5: New shard-body migration — specific gaps

- Requires shard-decl AST parsing to extract bilateral predicate signatures.
- Requires Rust AST parsing to identify equivalent Rust function bodies.
- Requires substrate-honest lift pattern (byte-sentinel resolver arm) synthesis.
- Requires test synthesis matching the ouroboros_bite pattern.

**Reed authoring estimate:** ~2500-5000 LOC. HIGHEST.

---

## §7 Recommendation — the smallest empirical proof of the theorem

**Recommended: Candidate 1 (Docblock stale-name rot).**

### Why

1. **Smallest common infrastructure.** Only requires `@io/fs.write` apply_h arm (~30 LOC) + shard walker action (~100 LOC).
2. **Substrate-already-has-the-word discipline preserved.** `delta` + `mutation` + `compare` LANDED at `@epistemologic/reality/time`. `@nl.compose` LANDED (as MVP echo). `@io/git.commit` LANDED (as arm). Only `@io/fs.write` needs the arm added.
3. **Empirical proof passes ALL FIVE stages of the theorem:**
   - Stage 1 (Observe): walker greps for stale names → list of {file, line, old, new}.
   - Stage 2 (Decide): rename-table lookup — trivially substrate-honest.
   - Stage 3 (Apply): write updated bytes via @io/fs.write.
   - Stage 4 (Compose): @nl.compose reads `delta.mutations` and emits commit message describing "rewrote N sites from X to Y across M files."
   - Stage 5 (Commit): @io/git.commit with `allow_empty=false` (real staged changes; add via new @io/git.add arm).
4. **Composable with the existing `mirror roomba --commit` binary.** Extend `observe_and_commit` in `roomba_commit.rs` to optionally call a `resolve_and_commit` variant that adds the mutation phase between observe and commit.
5. **Fiedler descent is REAL.** Docblock rewrites don't touch code semantics — they preserve `test_pass_rate`. They may or may not affect Fiedler (spectral graph doesn't parse doc-comment text) — that's ACCEPTABLE for first proof; the theorem doesn't require ouroboros_monotone to STRICTLY descend on the first tick; monotone-non-increasing is the invariant.
6. **`📝` pure-docs bypass eliminates hook friction.** Per AGENTS.md, markdown-only commits can `--no-verify`. Docblock-only rewrites in Rust files DON'T qualify (`.rs` change), so hook runs — but the hook's `[substrate-floor:@io-boundary]` marker + Seam-gate applies. Reed can author.
7. **The theorem generalizes.** Once Candidate 1 lands empirically, Candidate 2 (dead-code) becomes tractable because the same walker+decide+mutate+compose+commit chassis exists. Then 3, 4, 5 in sequence.

### The minimum viable end2end run

```
$ mirror roomba --resolve --commit
[observe] walker traverses ConceptGraph; grep-scans for stale renames
[observe] fracture emitted: docs/scouts/2026-07-14-taut-*.md:47 uses "dispatch" (should be "act")
[decide] kintsugi picks: rename-table lookup; morphism = replace("dispatch", "act") at 3 sites in 2 files
[apply] @io/fs.write for each affected file
[compose] @nl.compose over delta.mutations → "renamed 3 sites 'dispatch' → 'act' per etymology audit 546c2f6"
[commit] @io/git.add <files>; @io/git.commit (author=mirror; allow_empty=false)
[verify] git log HEAD: mirror <mirror@spectral.engineer>; git show HEAD --stat: 2 files changed, 3 insertions(+), 3 deletions(-)
```

**Alex inspects the delta.** `git show HEAD` — real diff. Real change. Real theorem.

### What this DOES NOT prove yet

- Path A/B decision (only rename-table decision).
- Ouroboros_monotone RESISTANCE (only monotone-preservation; docblock-only edits are safe).
- @kintsugi/ouroboros.cutover of an .rs file (requires Candidate 4).
- Full apply_h dispatch of `@kintsugi/ouroboros.collapse` (still `\`-obligation-blocked).

**All fine.** The theorem is: "the compiler can observe → decide → mutate → compose → commit." Candidate 1 discharges the five stages. Candidates 2-5 extend the DECIDE and MUTATE stages to progressively larger fractures. But the SHAPE lands first.

### Fallback recommendation

If Candidate 1 feels too trivial to Alex ("that's not really the ouroboros"), consider **Candidate 3 (deprecated shim removal) — specifically the `mirror spawn` alias.** `bootstrap/src/lib.rs:3203-3249` has `mirror spawn` as a deprecated alias for `mirror peer beam` — self-documenting deprecation, single-file change, no external test dependency to break. Would require ~300 LOC of authoring (grep + caller-migration + shim-removal). Higher proof-value than Candidate 1; still tractable.

---

## §8 Substrate-honest bounds — what this scout does NOT decide

- **Whether Candidate 1 is worthy of Alex's "empirical proof" verdict.** The scout ranks by tractability, not by aesthetic satisfaction of the theorem. Alex reserves the naming.
- **Whether `@io/fs.write` apply_h arm should be `[substrate-floor:@io-boundary]`.** Adding it IS a `.rs` change; Seam gate applies (audit citation OR Signed-off-by: Seam). Reed's authoring path is bounded by that gate.
- **Whether `@io/git.add` should be a new action on `@io/git` OR whether `@io/git.commit` should grow a `stage_paths: [ref]` arg.** Substrate-decl question; Mara-adjudicable.
- **Whether the delta emitted should content-address via `@epistemologic/reality/time.compare(snapshot_pre, snapshot_post) -> delta`** (substrate-honest but requires wiring the snapshot bridge) **OR via a NEW ad-hoc Rust struct.** First is honest; second is fast. Alex-adjudicable per substrate-honest-is-the-mode discipline.
- **Whether the empirical proof should run in a scratch worktree** (safer; can `git checkout HEAD~` to inspect delta) **OR on main** (bolder; commits stay).
- **Whether the FIRST empirical run should be Reed-witnessed or autonomous.** Substrate-honesty says Reed watches the first N runs; after N=5 confidence, autonomous. But there's no substrate-decl'd N.
- **What Rust-side changes are needed to `roomba_commit::observe_and_commit`** to add the resolve phase. Scout named the shape; Reed authors.
- **Whether the walker should surface fractures as `[fracture]` (list-shaped) or one at a time via imperfect** — substrate-decl gap named §1.5.
- **Whether `@nl.compose` should stay as echo-MVP or lift to real composition today.** The scout notes it's echo; the theorem tolerates echo for the first proof; substrate-honest lift is a subsequent tick.

Alex fires /loop when the scout lands. Reed authors from here.

---

## Substrate-already-had-the-word audit trail

Every carrier / action / concept named in this scout was verified as substrate-existing (LANDED) or substrate-declared (SPECCED) before invocation:

- `type delta` + `type mutation` + `compare(a, b) -> delta` → `shards/epistemologic/reality/time.mirror:120-152` (SPECCED with obligation-blocked body).
- `type diff_bytes` + `get`/`put` Foster lens pair → `shards/optics/lens/diff.mirror` (SPECCED).
- `@kintsugi/ouroboros.{collapse, verify_same_output, cutover, ouroboros_step}` → `shards/kintsugi/ouroboros.mirror:340-424` (SPECCED).
- `@kintsugi/surface.{tension, surface_class, surface}` → `shards/kintsugi/surface.mirror` (SPECCED).
- `@kintsugi/fracture/*` (9 landed species) → `shards/kintsugi/fracture/*.mirror`.
- `@io/git.{clone, fetch, read_object, commit_object, commit, hash_to_oid, resolve_ref}` → `shards/io/git.mirror:238-395`.
- `@io/fs.{read, write, stat, readdir, mkdir}` → `shards/io/fs.mirror:252-361`.
- `@mirror/store/git.{open, insert_persistent, get_persistent, set_ref, get_ref, flush, path}` → `shards/mirror/store/git.mirror:346-410`.
- `@kintsugi/store/git.commit_as_fold` → `shards/kintsugi/store/git.mirror`.
- `apply_h::act` 7-combinator surface → `bootstrap/src/apply_h.rs` (LANDED per Arc-1 Tick 1.3 f747a2c).
- `roomba_commit::observe_and_commit` → `bootstrap/src/roomba_commit.rs:264-269` (LANDED per commit 5441ea1).
- `WalkTrajectory` + `RoombaStep` → `bootstrap/src/roomba.rs:104-125, 85-101` (LANDED post-Arc-2 Tick 2.4).

**No new words invented.** The composition surface for the theorem's proof already exists in the substrate; the Rust-side wiring is the gap.

---

## Recommended read order for Reed

1. `docs/loop/CURRENT.md` — arc-state context (already in Reed's boot).
2. THIS SCOUT — the landscape map.
3. `docs/scouts/2026-07-15-reed-rust-extension-migration-map.md` §6 — the Arc-1..Arc-3 ordering.
4. `docs/scouts/2026-07-15-taut-kintsugi-ouroboros-substrate-scout.md` §D5 — @code/rust/materialize classifier composition.
5. `shards/epistemologic/reality/time.mirror:120-152` — the delta carrier substrate ancestry.
6. `shards/kintsugi/ouroboros.mirror:340-424` — the collapse/cutover action surface.
7. `bootstrap/src/apply_h.rs:743-849` — the current @nl.compose (echo MVP) + @io/git.commit resolver arms; the extension points for Candidate 1.
8. `bootstrap/src/roomba_commit.rs` — the driver to extend with `--resolve`.

END OF SCOUT.
