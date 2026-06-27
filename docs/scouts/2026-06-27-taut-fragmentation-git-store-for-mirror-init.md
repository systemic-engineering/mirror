# fragmentation git-store as substrate for `mirror init`

*Taut, 2026-06-27. Scout. Read-only inspection of
`/Users/alexwolf/dev/projects/fragmentation/` to determine what mirror
would actually need to build for "index a peer's repo into a content-
addressed crystal store" — i.e. Reed's Phase A.*

*Verify-before-claim per `[[200ef24]]`. Read-only fences: no
fragmentation/mirror code modified. Hard cap 400 lines.*

---

## §1 — Position

Reed sketched a "Phase A — Indexing parity" for `mirror spawn` /
`mirror init` and budgeted ~3–5 days of Rust work (crystal storage,
spec discovery, `--index` flag, persistence, repo walk).

Alex challenged: mirror is built **on top of** fragmentation, which
already has git-store integration. The substrate-pull-honest path
may not be "build it from scratch" — it may be "compose what's
already there".

Per `[[feedback-substrate-already-had-the-word]]` (52+ instances and
counting): scout fragmentation **before** designing the indexer.

Hypothesis going in: fragmentation has more than Reed assumed.
Result (§3): **substantially more**. Result (§4): mirror's
`bootstrap` crate **does not depend on fragmentation at all today**
and its only git surface is a `git CLI` shell-out. That gap is the
scout's surprise (§5).

---

## §2 — Methodology

Read in this order:

1. `fragmentation/README.md`, `ROADMAP.md`, `CLAUDE.md`, `MARA.md`,
   `AGENTS.md` — orientation.
2. `fragmentation/Cargo.toml` (workspace + features) — what compiles
   where.
3. `src/lib.rs` (module index), `src/repo.rs` (the `Repo` trait),
   `src/store.rs`, `src/concurrent_store.rs`, `src/bounded_store.rs`,
   `src/frgmnt_store.rs`, `src/walk.rs`, `src/encoding.rs`,
   `src/manifest.rs`, `src/project.rs`, `src/shard_ref.rs`,
   `src/singularity.rs`, `src/naked.rs`, `src/hamilton_scheduler.rs`.
4. `vcs/git/Cargo.toml`, `vcs/git/src/lib.rs`, `vcs/git/src/git.rs`,
   `vcs/git/src/store.rs`, `vcs/git/src/walk.rs`,
   `vcs/git/src/namespaced.rs`, `vcs/git/src/notes.rs`,
   `vcs/git/src/bin/frgmt-git.rs` (the CLI).
5. `docs/specs/fragmentation-vcs-spec.md`,
   `docs/specs/mirror-native-vcs.md` (excerpts; cross-reference for
   `mirror init` story).
6. mirror side: `mirror/bootstrap/Cargo.toml`, `bootstrap/src/git.rs`,
   `bootstrap/src/main.rs`, `bootstrap/src/lib.rs` (sampled),
   `mirror/docs/specs/mirror-store.md` (sampled),
   `mirror/docs/specs/store-vs-db-and-the-cascade.md`.
7. Memory: `architecture-fragmentation-is-the-rust-substrate`,
   `architecture-mirror-store-vs-spectral-db`,
   `architecture-splinter-and-spectral-db-edges`,
   `architecture-three-tier-stack`.

Verify-before-claim: every "exists / does X" in §3 cites a
file+symbol; every "does NOT exist / does NOT do X" in §4 is from a
grep miss documented inline.

---

## §3 — What fragmentation already has

### 3.1 Two content-addressed stores

| Type | File | What it is |
|------|------|------------|
| `Store<N, H>` | `src/store.rs:9` | In-memory `HashMap`-backed `Repo` impl. Generic over node type + hash. |
| `ConcurrentStore<N, H>` | `src/concurrent_store.rs:25` | `DashMap`-backed; `&self` on all methods; idempotent by content-address (concurrent writes converge). |
| `BoundedStore<N>` | `src/bounded_store.rs:18` | Size-bounded cache with LIFO eviction; underlies `FrgmntStore`. |
| `FrgmntStore<N>` | `src/frgmnt_store.rs:55` | **Bounded cache + on-disk `.frgmnt/` spillover** with fan-out (`objects/<2hex>/<rest>` + `refs/`). No git dep. |

The `Repo` trait (`src/repo.rs:13`) is the substrate surface:

```rust
trait Repo {
    type Node: Fragmentable + Clone;
    type Hash: HashAlg;
    fn write_tree(&mut self, node: &Self::Node) -> String;     // write content-addressed tree
    fn read_tree(&self, oid: &str) -> Option<Self::Node>;
    fn write_commit(&mut self, c: Commit<Self::Node, Self::Hash>);
    fn read_commit(&self, sha: &Self::Hash) -> Option<...>;
    fn update_ref(&mut self, name: &str, sha: Self::Hash);
    fn resolve_ref(&self, name: &str) -> Option<Self::Hash>;
}
```

Six methods. Every store impl above realises it.

### 3.2 The git wire layer (`vcs/git/`)

`GitStore<N>` (`vcs/git/src/store.rs:18`, also re-emitted in
`vcs/git/src/git.rs:428`) is a **two-tier persistent store**:

- Tier 1: in-memory `Store<N, N::Hash>`.
- Tier 2: a `git2::Repository` (the git ODB itself).
- `open(path)` (`store.rs:28`) calls `Repository::discover(path)` —
  walks UP for `.git/`. This is the "ambient repo" pattern.
- `flush()` (`store.rs:46`): write every cached node to the git ODB
  via `git::write_node` (blobs for shards, trees for branches with
  `.data` + `0000`-fanout entries), then `collapse_index` →
  `refs/store/index`.
- `hydrate()` (`store.rs:69`): read `refs/store/index`, rebuild the
  in-memory ref table via `refract_from`.
- Implements `Repo` (`store.rs:132`) — reads cascade memory →
  git ODB → miss (`store.rs:143`).

Low-level git operations live in `vcs/git/src/git.rs`:

- `write_tree<E: Encode>` (line 76) — `Fractal<E>` → git tree
  (blob+treebuilder).
- `write_node<N: Fragmentable>` (line 124) — **generic** over any
  Fragmentable; this is the load-bearing primitive.
- `read_node<N: Reconstructable>` (line 147) — generic inverse.
- `read_tree` (line 357), `read_tree_named` (line 274) — name-vs-
  numbered entry variants.
- `read_commit` (line 35), `read_witnessed` (line 9),
  `commit_signature` (line 63) — full commit metadata extraction
  from arbitrary git OIDs.
- `write_commit` (line 193) — write fragment+author+committer+
  parent → git commit.

`NamespacedGitStore` (`vcs/git/src/namespaced.rs:34`):

- Lives **inside** `.git/<namespace>/` (e.g., `.git/mirror/`,
  `.git/spectral/`).
- Wraps `FrgmntStore` so the store data travels with `.git/` but
  doesn't pollute the working tree.
- This is the file-format / location for "per-repo content-addressed
  cache" — the exact shape an indexer wants.

`vcs/git/src/walk.rs:25` provides `walk_commits_following(repo,
start_ref, path_prefix)` — `git log --follow` over a path prefix
using `Sort::TIME | Sort::TOPOLOGICAL` + `DiffOptions::pathspec`.

`vcs/git/src/notes.rs:48` provides `append_note` / `read_notes` for
append-only git-notes under `refs/spectral/notes/<topic>` — the
out-of-band metadata channel.

### 3.3 The CLI: `frgmt-git`

`vcs/git/src/bin/frgmt-git.rs` (516 lines) ships a Clap CLI with:
`shard | fractal | commit | sign | encrypt | decrypt | mount |
link | project | filter`. **`commit` already wraps `write_tree +
git2 commit + update-ref refs/<namespace>/<name>`** with namespace
discovery from `git config fragmentation.namespace` (line 140).
That IS the "index this content into git as a named ref" primitive
already shipped.

### 3.4 Singularity / NakedSingularity (collapse/settle)

`src/singularity.rs:11` defines the `Singularity` trait
(`collapse → Artifact`, `settle(artifact) → Self`).
`WitnessedSingularity` (line 78) writes a commit whose tree is a
`Lens` back to the original tree — content-address invariant under
observer, commit varies with witness. `NakedSingularity`
(`src/naked.rs:25`) carries content + witness + dual CIDs as a
self-contained bundle. These are the "shippable crystal" primitives.

### 3.5 Scheduler + shard handle

`HamiltonScheduler` (`src/hamilton_scheduler.rs:33`) — T2 stub,
budget + tick counter, call-shape ready for fragmentation-mcp.
`ShardRef` (`src/shard_ref.rs:80`) — `SpectralUuid + ShardContext +
BudgetBytes` typed handle; `ShardContext` is a `PrismMonoid +
Prism`. Per `[[architecture-shard-ref-as-prism]]`, this IS the
"session-implicit shard" surface mirror needs.

### 3.6 Projection (manifest-driven)

`src/manifest.rs:13` — `Manifest { lenses: Vec<LensEntry { source,
target }> }`; `src/project.rs:42` — `project(source_dir,
&manifest) → Projection { files: BTreeMap<target, ProjectedFile {
content, oid }> }`. This is selective directory ingestion with
content-addressing. Not a full repo walk, but parametric over
which paths to ingest.

### 3.7 What is NOT present

Grepped; not found anywhere in `fragmentation/` or `vcs/git/`:

- A `walk_repo`/`ingest_repo`/`index_repo` primitive that walks an
  entire working tree (or HEAD's git-tree) and shards every file.
  `project` does manifest-driven selective ingestion; nothing iterates
  the working tree top-to-bottom.
- A git-hook installer (no `pre-commit` / `post-commit` writer).
- A `mirror init` or `mirror spawn` analogue at this layer (correct:
  init is mirror's altitude, not fragmentation's).
- `refs/crystals/<source_hash>` (mirror's current convention; see §4).
  fragmentation uses `refs/<namespace>/<name>` and
  `refs/store/index`.

---

## §4 — What mirror already does with git

### 4.1 Mirror does NOT depend on fragmentation today

`grep fragmentation` across `mirror/**/Cargo.toml`: **no matches in
any dependency block.** `mirror/bootstrap/Cargo.toml` pulls `sha2`,
`blake3`, `prismqueer`, `terni`, `serde`, `serde_json`, `libc` — and
nothing else. Despite the canonical spec
`mirror/docs/specs/mirror-store.md` (2026-06-04, Mara) **naming
fragmentation as the substrate** and despite memory entry
`[[architecture-fragmentation-is-the-rust-substrate]]` declaring the
strict `mirror → fragmentation → prism_core` chain — **the Cargo edge
is not wired**.

This is a documented intent, not an implementation. mirror-store.md
§1 explicitly: *"The fragmentation store IS the canonical content-
addressed substrate. … No code lands in this tick."* (status: Red).

### 4.2 What mirror's `git.rs` does today (60 lines)

`mirror/bootstrap/src/git.rs` is a `std::process::Command` shell-out
to the system `git` binary:

- `git_store(content)` → writes a temp file, runs `git hash-object
  -w <tmpfile>`, returns the blob SHA. (line 22)
- `git_store_crystal(source_hash, crystal_oid)` → stores `crystal_oid`
  as a blob, points `refs/crystals/<source_hash>` at it via
  `git update-ref`. (line 41)
- `git_crystal_exists(source_hash)` → `git cat-file -p
  refs/crystals/<source_hash>` to check. (line 52)

Two callable functions; that's the entire mirror↔git surface. No
git2 / gitoxide import; no FrgmntStore; no `Repo` impl. The crystal
"index" is a flat `refs/crystals/*` namespace with one ref per
source hash. Persistence is whatever the surrounding git repo
already provides.

### 4.3 Mirror's lib.rs is large (127 KB)

The substrate altitude where mirror DOES live: `bootstrap/src/lib.rs`
(127 KB), `spectral.rs` (200 KB), `crystallize.rs` (40 KB).
`crystallize::Crystallizations<H>` is a `Ref → Body<H>` dispatcher
(empty floor; ticks B/F populate). The crystal vocabulary is
established at the substrate; the **storage** of those crystals is
the shell-out above. That's the bridge `mirror init` would have to
build.

---

## §5 — The `mirror init` story (composition view)

Given §3 + §4, here is what `mirror init <path>` would actually need
to do — **as composition, not new construction**:

### Already provided by fragmentation (composition only)

| Need | Substrate primitive |
|------|---------------------|
| Open repo at path, walk up to `.git/` | `git2::Repository::discover` (used in `GitStore::open`, `NamespacedGitStore::find_git_dir`) |
| Create `.git/mirror/` cache dir | `NamespacedGitStore::open(repo_path, "mirror")` |
| Persist crystals across runs | `FrgmntStore::open(".git/mirror", 16 MiB)` (fan-out + refs/) or `GitStore::flush()` (write to git ODB + `refs/store/index`) |
| Content-address a file | `fragment::blob_oid` (shard) / `encoding::encode` (Fractal) / `write_node` (generic Fragmentable) |
| Write a tree + commit + ref | `frgmt-git commit` shape (`DraftWriteExt::write_to_git` + `update-ref refs/mirror/<name>`) |
| Iterate commit history filtered by path | `vcs/git/src/walk.rs:25 walk_commits_following` |
| Out-of-band metadata (e.g., last-indexed-tip) | `vcs/git/src/notes.rs:48 append_note` |
| Session/shard handle for the indexer | `ShardRef { id, context, budget_bytes }` + `HamiltonScheduler` |
| Self-contained shippable crystal (peer-export) | `NakedSingularity` (dual-OID, witness in content) |

### What mirror needs to ADD on top

Real, not-yet-existing work for the v0 `mirror init`:

1. **Cargo edge.** Add `fragmentation = { path = "../../fragmentation",
   features = ["concurrent", "singularity", "project",
   "supervision"], default-features = false }` to
   `mirror/bootstrap/Cargo.toml`. Drop or re-route
   `bootstrap/src/git.rs` shell-outs to either `git2` (via
   `fragmentation-git`) or keep the shell-out as a one-line fallback.
2. **The walk-repo primitive** (genuinely missing). Either:
   - **(a)** add `walk_repo(repo: &Repository, head: Oid) -> impl
     Iterator<Item = (path, Blob)>` to `vcs/git/src/walk.rs` (one
     function; git2's `Tree::walk(PreOrder, …)` is the body, ~30
     LOC); or
   - **(b)** drive a `Manifest` from a file listing and reuse
     `project::project` (already there, no new code).
   Path (b) is cheaper if the v0 indexer takes an explicit file list
   (e.g., `find ~/.reed -type f | mirror init --manifest -`).
3. **mirror-altitude `init` command.** A Clap subcommand in mirror's
   dispatch surface that: discovers/creates `.git/mirror/`, opens
   `NamespacedGitStore`, walks the source via §5.2, drives each file
   through `crystallize::Crystallizations` (mirror's existing
   pipeline), persists the resulting crystals via the store, writes
   `refs/mirror/HEAD` pointing at the index commit. ~200 LOC.
4. **Git hooks (optional, deferrable).** `mirror init --install-hooks`
   writes `.git/hooks/post-commit` that re-runs the indexer on the
   delta. Pure shell template + a `git hash-object`-style invocation
   of `mirror reindex`. Not load-bearing for v0.
5. **Mycelium registration (forward-promise; defer).** No code today;
   `[[architecture-spectral-db-autopoietic-memory]]` lives at the
   `~/.mirror` root supervisor altitude.

---

## §6 — Re-estimated gap vs Reed's 3–5 day sketch

**Per `[[feedback-no-time-estimates]]`, no calendar estimate.** The
re-estimate is **scope**:

- **Reed's sketch (implicit assumption):** ~6 deliverables built ground-
  up: crystal storage / spec discovery / repo walk / `--index` flag /
  persistence / hook scaffolding.
- **Reality:** items 1, 2, 4, 5, 7, 8 of the §5 inventory **already
  exist** in fragmentation. The genuinely missing pieces collapse to
  **three**: the cargo edge (one Cargo.toml diff), the walk-repo
  primitive (≤30 LOC OR free via `project`), and the mirror-altitude
  `init` command (~200 LOC).

**Shape of the gap:** **fundamentally different.** Not "shorter
because we found a shortcut" — different because the substrate-pull-
honest path **is composition**, not construction. Reed's sketch was
sized for the wrong altitude: the bytes/storage/ODB altitude is
done; what's missing is the **mirror-side glue and one Cargo edge**.

**Adversarial check:** what could make this estimate wrong?
- If `crystallize::Crystallizations` dispatch table needs registering
  bodies before any file walk is meaningful, `init` is gated on tick
  B/F (mirror-store.md §1 says floor is empty in Tick A). The walk
  works, but it produces `Uncrystallized` verdicts. Genuine, but
  separable: `init` can ship as "index → store → write ref", with
  empty bodies still yielding stored Splinters. The walk is not
  blocked by the dispatcher table being thin.
- If `mirror/bootstrap` deliberately avoids the `git2` transitive
  closure (binary size / FFI surface), `fragmentation-git` adds
  `git2 = 0.19` + `dashmap = 6`. Worth verifying with Alex before
  the cargo edge lands. The alternative — keep `git2` out of
  bootstrap and front everything through `frgmt-git` as a subprocess
  — is a valid posture and changes §5 item 1.

---

## §7 — Substrate connections that load-bear

- `[[architecture-fragmentation-is-the-rust-substrate]]` — declares
  the chain. **Cargo wiring is the load-bearing missing edge.**
- `[[architecture-mirror-store-vs-spectral-db]]` — confirms
  `@mirror/store` is the open content-addressed gate; this scout's
  §5 lands at exactly that altitude.
- `[[architecture-splinter-and-spectral-db-edges]]` — Splinter (the
  floor data type) is the substrate-altitude analogue of fragmentation's
  `Fractal<E>`. `Crystallizations::crystallize` returns `Splinter<H>`;
  fragmentation persists `Fractal<E>` by `content_oid`. The bridge is
  one `Fragmentable` impl away (out of scope for this scout).
- `[[architecture-shard-ref-as-prism]]` — `ShardRef` + `ShardContext`
  IS the typed session handle `mirror init` should pass to whatever
  pipeline ingests files.
- `[[architecture-pq-as-mcp-surface]]` — the 18-tools-collapse-to-3
  result also lives here: `mirror init` doesn't invent a new wire;
  it consumes the existing pq surface.
- `fragmentation/docs/specs/fragmentation-vcs-spec.md` §3 already
  enumerates `mirror init / commit / diff / log / push / pull /
  status` against fragmentation primitives. **The mapping is
  pre-existing.**
- `mirror/docs/specs/mirror-store.md` (2026-06-04, Mara) — canonical
  intent doc; status Red; this scout is one of the inputs that
  unblocks the Green tick.

---

## §8 — Honest 0–2 self-test per section

| §  | Topic | Grade | Justification |
|----|-------|-------|---------------|
| §1 | Position | 2 | Hypothesis stated; result foreshadowed; consistent with §3/§4 evidence. |
| §2 | Methodology | 2 | Read order is concrete and the file list is complete enough to reproduce. |
| §3 | What fragmentation has | 2 | Every claim has file+line; "what is NOT present" was grep-verified. |
| §4 | What mirror has today | 2 | The "no fragmentation dependency" claim was grep-verified across all `mirror/**/Cargo.toml`; the `git.rs` line counts and function signatures are direct reads. |
| §5 | mirror init story | 1 | Inventory is grounded; LOC estimates for items 2/3 are eyeballed (`Tree::walk` body, Clap glue) and could be off by a factor of 2. Honest. |
| §6 | Gap re-estimate | 1 | "Fundamentally different" claim is load-bearing; adversarial checks surfaced two real risks (empty dispatch table + bootstrap-git2-avoidance posture). Either could shift §5. Worth Alex's call. |
| §7 | Substrate connections | 2 | All citations verified against the source memory entries or the live spec files. |

Overall: **1.7 / 2**. §5/§6 estimates are the soft underbelly.

---

## §9 — Pack trail

- Read order in §2 is reproducible; cargo edge in §5.1 is one-diff.
- Mara owns mirror-store.md; this scout argues the Red→Green tick is
  smaller than its 2026-06-04 framing implies.
- Seam should adversarially check §6 against the "bootstrap deliberately
  has minimal deps" posture before any cargo edge lands.
- Glint inherits no DX surface from this scout (no user-facing CLI
  changes specified yet).
- Reed owns the call: scope `mirror init` against §5's three-item gap
  vs the original six-item sketch. The scout's recommendation: the
  three-item gap.

*Taut, 2026-06-27. End of scout.*
