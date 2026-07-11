# Taut scout — @bauchladen + @mirror/store shared fragmentation primitive; Fate::bounded(psychohistory_sheaf) landability

*Scout, read-only, 2026-07-11. Alex directive (~04am 2026-07-11):
"@bauchladen and @mirror/store ought to share a storage primitive which
is actualized through fragmentation, that's what I see. What if we had
a Fate::bounded(<config>) which we derive from the psychohistory sheaf?
And then we just let the inference navigate the cohomology?"*

## Executive summary

- **Q1 @bauchladen storage substrate-decl:** LIFT of @mirror/store, not
  a separate storage backend. Substrate-honest already.
- **Q2 @mirror/store fragmentation citation:** Substrate-decl at
  `shards/mirror/store.mirror` cites `fragmentation/src/*.rs` as
  Rust realisation; @mirror/store IS the substrate-altitude interface,
  fragmentation IS the concrete crate.
- **Q3 fragmentation shape:** VCS-agnostic content-backed store;
  `Fractal::{Shard, Branch}` + `frgmnt_store::FrgmntStore` +
  `NamespacedGitStore` (in `fragmentation-git` sibling crate). Load-
  bearing primitives Alex referenced ARE the crate's public surface.
- **Q4 psychohistory:** LIVES AS INSIGHT (`docs/insights/
  2026-06-26-psychohistory-vector-as-sheaf.md`, Mara `d00f553`), NOT as
  substrate-decl'd family-root. Explicitly refused promotion in the
  insight itself (§7). Named at prose altitude in @song family
  (voice/movement/narrative/phrase/progression) via psychohistory
  binding. NO `@psychohistory` prism exists.
- **Q5 bounded config carrier:** `@loop` declares `terminal_check` +
  `restart_intensity` budget; NO `Fate::bounded(...)` substrate-decl
  exists. Naming space is open.
- **Q6 sheaf-Laplacian:** LANDED at
  `shards/epistemologic/math/sheaf_laplacian.mirror` (13.1KB,
  2026-07-10 22:59). Exposes `sheaf_laplacian([restriction]) ->
  operator` + `lambda_zero(op) -> eigenvalue`. Composes with the
  bundle tower at `boot/std/epistemologic/math/bundle.mirror` via
  Barbero et al. 2022 (O(d)-bundle sheaf-Laplacian = connection
  Laplacian).
- **Q7 fault planes:** @bauchladen ALREADY imports `in @mirror/store`
  (line 5 of shards/bauchladen.mirror). Shared-storage refactor is
  NOT a rewrite; it is a *recognition* of what already holds.

**LRM overall: LANDABLE-WITH-PREREQS.** The shared-primitive claim is
substrate-correct; @bauchladen already lifts @mirror/store per its own
"lift, not re-declaration" section. `Fate::bounded(psychohistory_sheaf)`
requires @psychohistory substrate-decl FIRST (currently insight-only).

**Shared-storage refactor take:** SUBSTRATE-CORRECT. Alex is naming what
holds. No refactor needed at substrate altitude; documentation-cascade
tick may be warranted.

**Fate::bounded landability:** LANDABLE only after @psychohistory
substrate-decl lifts from insight to prism. Two-tick prereq: promote
insight → substrate-decl, then Fate::bounded(psychohistory) config
carrier.

---

## §Q1 — @bauchladen current storage substrate-decl

**Grep evidence** (`shards/bauchladen.mirror`):
- Line 5: `in @mirror/store` (import declared at family-root altitude).
- Lines 83–101: "Relationship to @mirror/store: lift, not re-declaration."
  - "@mirror/store (operational since 2026-06-04) is the OPERATIONAL
    surface: six operations (read / write / exists / diff / walk /
    verify); splinter_graph as structural lockfile; uuid_spectral-
    addressed shards. Realized in Rust at `fragmentation/src/*.rs`."
  - "@bauchladen LIFTS that surface to a *contract* a prism may commit
    to." Two altitudes of one mechanism.
- Lines 275+: `prism @bauchladen { ... }` — no storage-backend action;
  only `bauchladen_addressable(c: crystal) -> verdict` and
  `bauchladen_witnessing(c)` predicates.

**Interpretation:** @bauchladen does NOT declare a separate storage
backend. It declares a prism-altitude CONTRACT that consumers commit
to; the operational realisation IS @mirror/store. The "shared storage
primitive" Alex sees is ALREADY substrate-declared: bauchladen imports
mirror/store.

## §Q2 — @mirror/store current fragmentation usage

**Grep evidence** (`shards/mirror/store.mirror`):
- Lines 1–5: `in @prism`, `in @glass`, `in @meta`, `in @uuid/spectral`
  (does NOT import fragmentation directly at substrate altitude;
  fragmentation lives at realisation altitude).
- Line 7 (recognition prose): "the substrate truth lives in the
  fragmentation store, content-addressed via OIDs."
- Line 220 (six-op surface): `read / write / exists / diff / walk /
  verify` — REAPI-shaped CAS surface.
- Cross-reference in `shards/bauchladen.mirror` line 83: "Realized in
  Rust at `fragmentation/src/*.rs` per [[architecture-fragmentation-
  is-the-rust-substrate]]."

**Interpretation:** fragmentation crate IS the substrate-declared Rust
realisation of @mirror/store. Naming-wise, the substrate uses
`@mirror/store` at prism altitude; `fragmentation` is Rust-crate name.
The primitive @mirror/store exposes: `splinter_graph` (root + closure),
`shard` (SpectralUuid-addressed), `walk` (forward closure),
`impacted_by` (reverse closure).

## §Q3 — fragmentation crate current shape

**Grep evidence** (`/Users/alexwolf/dev/projects/fragmentation/src/lib.rs`):
- Doc comment: "Content-addressed, arbitrary-depth, circular-reflexive
  trees. Two node types: `fragment::Fractal::Shard` (terminal) and
  `fragment::Fractal::Branch` (recursive, contains other fractals)."
- Layering: "`fragmentation` is the VCS-agnostic content-backed store.
  It owns the content primitives, the storage backends, and the
  hash/encoding contracts. VCS adapters live in workspace siblings:
  `fragmentation-git`, `fragmentation-jj`."
- Modules present: `cid`, `commit`, `concurrent_store`, `diff`,
  `encoding`, `fragment`, `frgmnt_store`, `hamilton_scheduler`,
  `lapack_prism`, `manifest`, `naked`, `prism_bridge`, `project`,
  `ref_`, `repo`, `sha`, `shard_ref`, `singularity`,
  `spectral_coordinate`, `store`, `supervision`, `visibility`, `walk`,
  `witnessed`.
- Features: `concurrent`, `prism-bridge`, `visibility`, `singularity`,
  `project`, `supervision`, `ssh`, `gpg`.
- `NamespacedGitStore` lives in the `fragmentation-git` sibling crate
  (not in `fragmentation/src/` main tree; per @mirror/store/git shard
  substrate-decl at `shards/mirror/store/git.mirror`).

**Interpretation:** Alex's referenced primitives (Splinter/Fractal DAG,
FrgmntStore, NamespacedGitStore) ARE the load-bearing public surface.
The crate is the *actualization* of the shared storage primitive.

## §Q4 — psychohistory sheaf substrate-decl

**Grep evidence:**
- `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` (Mara
  `d00f553`, 60.5KB, canonical insight). §7 explicitly refuses
  promotion: "It is not the announcement of a new family-root prism
  (`@psychohistory` is not declared here). It is not a substrate-decl."
- ZERO substrate-decl hits: `grep -rn "@psychohistory" shards/`
  returns nothing at prism altitude. NO `shards/psychohistory.mirror`.
  NO `prism @psychohistory { ... }`.
- Named at PROSE altitude only within @song family:
  `shards/song.mirror`, `shards/song/voice.mirror`,
  `shards/song/movement.mirror`, `shards/song/narrative.mirror`,
  `shards/song/phrase.mirror`, `shards/song/progression.mirror`. All
  cite Mara's insight; none declares @psychohistory as prism.
- `shards/song/narrative.mirror` (line 3): "the psychohistory + wire
  binding species of the @song family." Names PSYCHOHISTORY BINDING
  as substrate-decl'd shape at species altitude via @song/narrative
  species; the psychohistory itself is not the family-root.

**Interpretation:** Psychohistory is INSIGHT-CARRIED, not
substrate-decl'd. Alex's `Fate::bounded(psychohistory_sheaf)` requires
either (a) lifting the insight to substrate-decl as `@psychohistory`
family-root, OR (b) consuming through @song/narrative species (the
current lift point). The substrate-already-had-the-word pattern
suggests promoting the sheaf-carrier as an isomorphism species with
@song/narrative, not a new family-root.

## §Q5 — bounded fate config carrier

**Grep evidence:**
- `shards/loop.mirror`: declares `terminal_check(s: moi(tick_state))
  -> verdict` as the fixed-point predicate; loop-well-founded
  discipline. NO `Fate::bounded`.
- `shards/spectral/restart_intensity.mirror`: budget = 0 fires
  `terminal_check`; typed `ref` per no-bare-types feedback.
- `shards/kintsugi/surface.mirror`: `kintsugi_context` carries
  altitude, budget; "safety-net halt. When budget hits zero the loop
  terminates at `terminal_check`."
- ZERO hits: `Fate::bounded`, `fate_bounded`, `bounded_fate`,
  `psychohistory_config` anywhere in shards/ or docs/specs/.

**Interpretation:** Naming space is OPEN. The `bounded` shape exists
across three shards (loop / kintsugi / spectral/restart_intensity) but
Fate does not currently carry a `bounded(<config>)` action. Substrate-
honest name candidates (ranked by substrate-already-had-the-word
discipline):
1. `fate.bounded_by(psychohistory)` — verb form, consistent with
   `terminal_check` naming.
2. `fate.tournament(psychohistory)` — reuses tournament vocabulary
   already at @fate.
3. NOT `Fate::bounded(<config>)` — camelCase is Rust idiom, not
   substrate-decl.

## §Q6 — cohomology / sheaf-Laplacian substrate-decl

**Grep evidence** (`shards/epistemologic/math/sheaf_laplacian.mirror`,
13.1KB, 2026-07-10 22:59):
- `type restriction` — connection 1-form value on one edge.
- `type operator` — assembled sheaf Laplacian Δ_F = δ*δ.
- `type eigenvalue` — one root of D's characteristic polynomial.
- `sheaf_laplacian([restriction]) -> operator` — the δ*δ assembly per
  Bodnar et al. 2022 §2.
- `lambda_zero(op: operator) -> eigenvalue` — smallest non-trivial
  eigenvalue = Fiedler value / algebraic connectivity / spectral gap.
- Composes with `boot/std/epistemologic/math/bundle.mirror` via
  imported `in @epistemologic/math/bundle`. Barbero et al. 2022
  (arXiv:2206.08702) cited: for an O(d)-bundle over a graph the sheaf
  Laplacian IS the connection Laplacian.
- Bundle tower has `type optic` (connection), `type group` (gauge),
  `type holonomy` (loss carried by transport).

**Interpretation:** The cohomology-navigation primitive Alex wants
(`inference navigates the cohomology`) IS LANDED. `lambda_zero` gives
the descent direction (Fiedler eigenvector); the bundle tower's
`holonomy` gives what @fate.minimize consumes. Alex's proposal
composes with LANDED substrate.

## §Q7 — Fault-planes under shared-storage refactor

**Grep evidence:**
- @bauchladen already imports `in @mirror/store` (line 5) — no
  collision.
- @bauchladen names the lift explicitly (lines 83–101, 114–124) — no
  drift.
- NO consumer prism declares `in @bauchladen` in shards/ tree
  currently (checked via grep for `in @bauchladen` outside
  bauchladen.mirror itself) — recognition surface is small.
- Naming collision risk: NONE. @bauchladen is prism-altitude discipline;
  @mirror/store is operational surface; fragmentation is Rust crate.
  Three names, three altitudes, one mechanism.

**Interpretation:** ZERO fault planes under Alex's proposed refactor
because the refactor is a *recognition* of what already holds. If
implemented as a documentation cascade tick (docs/insights/ or
docs/math/ entry naming the shared primitive), it lands without
substrate touch.

---

## §Landing sequence — recommended two-tick

**If Alex wants Fate::bounded(psychohistory_sheaf) landable:**

**Tick 1 (Mara or Reed): promote psychohistory insight → substrate-decl.**
- Path α: `shards/psychohistory.mirror` as family-root prism (cites
  Mara's insight `d00f553` §3.4 sheaf-shape naming).
- Path β: extend @song/narrative to carry the psychohistory sheaf as
  species substrate-decl (avoids @psychohistory family-root; consumes
  through existing @song).
- Substrate-honest lean: path β. The @song family already carries the
  binding at species altitude; promoting @psychohistory as
  family-root duplicates.

**Tick 2 (Reed): land `Fate::bounded_by(<config>)`.**
- Extend `shards/fate.mirror` with `fate.bounded_by(sheaf) ->
  tournament_result` action; consumes sheaf from tick 1's substrate-
  decl.
- Config carrier: `type fate_bound = { sheaf: psychohistory_sheaf,
  budget: ref, gauge: group }` — composes bundle-tower gauge with
  bounded discipline.
- Body discharges through `@epistemologic/math/sheaf_laplacian.
  lambda_zero` for descent direction.

**Shared-storage documentation cascade (optional):** cascade tick
naming the `@bauchladen + @mirror/store + fragmentation` triangle at
docs/insights/ altitude. Not substrate-blocking; recognition-honest.

---

*Scout report. Read-only. Time budget consumed: ~35 min. Grep-heavy
per discipline. Submitting to Reed for orchestration adjudication.*
