# Taut scout — spectral → mirror migration mapping for gestalt-scan / Fiedler / `spectral_index` pull-in

*Scout, read-only, 2026-07-13. Reed-spawned after Alex's in-transcript verbatim: "the spectral__spectral_index is something that currently lives in spectral I presume? This is something that needs to be pulled into mirror."*

*Directional wrongness (per Recognition #43 + #55):* mirror IS a content-addressed build system; its own @fractal-coherence measurement is currently emitted by the sibling `spectral` binary, not by mirror's own voice. Scope PI-A closes that gap.

*Substrate-honest discipline:* grep-first inventory of the migration surface, overlap map with mirror's existing primitives, consumer trace, dependency-direction check, substrate-decl gap, minimum-viable execution plan for Scope PI-A, risk register, recommended sequence.

---

## §0 — TL;DR (top-5 for Reed at execution altitude)

1. **Migration is small.** The load-bearing surface is 3 files at `/Users/alexwolf/dev/projects/spectral/crates/gestalt/src/`: `detect.rs` (874 LOC), `graph.rs` (636 LOC), `eigenvalue.rs` (727 LOC). Combined ~2237 LOC of pure Rust with **zero external deps** (no petgraph, no nalgebra, no blake3 at gestalt altitude — Jacobi eigenvalue algorithm is hand-rolled). Fork into `bootstrap/src/index.rs` as one file.

2. **Bidirectional-dep risk is low.** `spectral` depends on `mirror` (path = "../mirror") but only imports `mirror` for spec/compiler surfaces — NOT for gestalt/graph/eigenvalue primitives. Pulling gestalt into mirror does NOT create a cycle. `gestalt` crate's declared deps in `crates/gestalt/Cargo.toml`: `prism-core`, `terni`, `mirror` (import only for pretty-printer / document-model layers — the `detect` / `graph` / `eigenvalue` modules are internally self-contained and use only `prism_core::oid::Oid` for content-addressing and `pulldown-cmark` transitively-not-load-bearing for the scan).

3. **@fractal family-root should mint FIRST.** `shards/fractal.mirror` does NOT exist. Two-tick discipline: land the family-root and the action-decl at `shards/fractal/index.mirror` in one tick; RED-lock the Rust discharge in the next. Substrate-decl-first is the mirror discipline. `shards/spectral.mirror` (mirror-side) is a runtime namespace-parent for db/garden/portal — different altitude; NOT the home for the fractal-coherence measurement.

4. **Reuse mirror's existing Fiedler primitive at the numerical altitude.** `bootstrap/src/sheaf_laplacian.rs::lambda_zero` (LAPACK `dsyev` via `prismqueer::ffi::eigenvalues`) is the substrate-honest λ₀ compute — already landed T8, already used by `tensor::fiedler_of` on the gap-tension graph. The migration should route the file-directory concept-graph Laplacian THROUGH `sheaf_laplacian::sheaf_laplacian` + `lambda_zero`, not import spectral's hand-rolled Jacobi. This is a substrate-pull win: mirror's numerical floor already exists.

5. **RED-lock target Fiedler value is empirically-determined, NOT `0.0612`.** The `0.0612` widely cited (docs/loop/CURRENT.md, docs/roadmap/15-fractal-membrane, docs/specs/mirror-store-bounded-peer-runtime) is scaffolding vocabulary — the prior scout `b52b008` §3.2 explicitly named: "grep of `0\.0612` across bootstrap/src/ + bootstrap/tests/ + shards/: zero hits. Not measurement." The RED-lock target is `spectral spectral_index /Users/alexwolf/dev/projects/mirror` executed AT the RED-commit tick, capturing the reference (fiedler, oid, node-count, edge-count). Landing 1 (Reed) is: run the reference, record it, RED-lock bootstrap to match ±ε.

**Recommended atomic sequence:** family-root mint (Mara 📝) → action-decl mint (Mara 📝) → reference-run + RED lock (Reed 🔴) → GREEN via forked-and-composed-with-sheaf_laplacian primitive (Reed 🟢) → CLI/MCP wire-in as Landing 3 (Reed 🟢). Full detail below §8.

---

## §1 — Migration surface inventory (TASK 1)

### 1.1 Types to migrate (from `spectral::crates::gestalt`)

| Type | File | LOC region | Public API surface |
|---|---|---|---|
| `GrammarKind` | `crates/gestalt/src/detect.rs:16-31` | 16 LOC | 7 variants: `Markdown`, `GestaltNative`, `Mirror`, `Code(String)`, `Config(String)`, `Asset`, `Unknown`. Ord+Hash+PartialEq. |
| `DetectedFile` | `crates/gestalt/src/detect.rs:35-38` | 4 LOC | `path: PathBuf`, `kind: GrammarKind`. |
| `GestaltBreakdown` | `crates/gestalt/src/detect.rs:42-71` | 30 LOC | 7 counters + `total()` + `record(&GrammarKind)`. Default. |
| `MarkdownShape` | `crates/gestalt/src/detect.rs:337-343` | 7 LOC | `heading_count`, `paragraph_count`, `word_count`, `link_count`, `wiki_link_targets: Vec<String>`. |
| `CodeShape` | `crates/gestalt/src/detect.rs:387-392` | 6 LOC | `function_count`, `type_count`, `import_count`, `line_count`. |
| `GraphNode` | `crates/gestalt/src/graph.rs:21-34` | 14 LOC | 2 variants: `Directory { path, name, depth, file_count }`, `Root { path, file_count }`. `oid()`, `name()`, `file_count()`. |
| `GraphEdge` | `crates/gestalt/src/graph.rs:66-85` | 20 LOC | 3 variants: `Contains { parent_idx, child_idx, weight }`, `SimilarContent { a_idx, b_idx, weight }`, `CrossRef { source_idx, target_idx, weight }`. `indices()`, `weight()`. |
| `ConceptGraph` | `crates/gestalt/src/graph.rs:111-186` | 76 LOC | `nodes: Vec<GraphNode>`, `edges: Vec<GraphEdge>`. `empty()`, `oid()`, `adjacency_matrix()`, `laplacian_matrix()`. |
| `EigenvalueProfile` | `crates/gestalt/src/eigenvalue.rs:24-72` | 49 LOC | `values: [f64; 16]`. `dark()`, `is_dark()`, `fiedler_value()`, `oid()`, `to_bytes()`. **NOTE:** shadowed by a second `EigenvalueProfile` at `crates/gestalt/src/spectral.rs:16-56` (different — has `id`, `label` fields; visualization block, not scan output). Only the eigenvalue.rs version migrates. |

### 1.2 Functions to migrate

| Function | File:line | Signature | Call sites (spectral-internal) |
|---|---|---|---|
| `detect_grammar` | `detect.rs:78` | `(path: &Path) -> GrammarKind` | `walk_recursive` only. |
| `walk_detected` | `detect.rs:183` | `(root: &Path) -> (Vec<DetectedFile>, GestaltBreakdown)` | `graph::build_concept_graph`; MCP dispatchers. |
| `walk_recursive` | `detect.rs:265` | private helper | `walk_detected` only. |
| `load_gitignore` | `detect.rs:210` | `(root: &Path) -> Vec<String>` | `walk_detected` only. |
| `is_gitignored` | `detect.rs:226` | `(relative: &Path, patterns: &[String]) -> bool` | `walk_recursive` only. |
| `should_skip_dir` | `detect.rs:196` | `(name: &str) -> bool` | `walk_recursive` only. Hardcoded ignore-list (13 entries + `.spectral` — the `.spectral` entry can stay or become `.mirror` at mirror-altitude, TBD Mara call). |
| `extract_markdown_shape` | `detect.rs:346` | `(content: &str) -> MarkdownShape` | `graph::extract_cross_references`; also usable by mirror @nl adapter. |
| `extract_code_shape` | `detect.rs:396` | `(content: &str, language: &str) -> CodeShape` | not called internally — public utility. |
| `extract_config_key_count` | `detect.rs:507` | `(content: &str, format: &str) -> u32` | not called internally — public utility. |
| `build_concept_graph` | `graph.rs:197` | `(root: &Path) -> (ConceptGraph, Vec<DetectedFile>, GestaltBreakdown)` | `graph_cache::build_and_cache`; MCP dispatchers; **THE main entry**. |
| `compute_type_distributions` | `graph.rs:329` | private helper | `build_concept_graph`. |
| `cosine_similarity` | `graph.rs:356` | `(a: &[f64], b: &[f64]) -> f64` | `build_concept_graph`. |
| `extract_cross_references` | `graph.rs:367` | `(root, files, dir_to_idx) -> Vec<GraphEdge>` | `build_concept_graph`. |
| `eigenvalue_profile` | `eigenvalue.rs:85` | `(graph: &ConceptGraph) -> EigenvalueProfile` | `graph_cache`, MCP dispatchers; **THE main entry**. |
| `build_profile` | `eigenvalue.rs:102` | private helper | `eigenvalue_profile`. |
| `jacobi_eigenvalues` | `eigenvalue.rs:133` | `(matrix: &[f64], n: usize) -> Vec<f64>` | `eigenvalue_profile`. **REPLACE with `sheaf_laplacian::lambda_zero` per §2.3.** |
| `jacobi_eigen_decomposition` | `eigenvalue.rs:217` | `(matrix, n) -> (Vec<f64>, Vec<Vec<f64>>)` | `spectral_embedding_2d`. Used only for the 2D layout viz — NOT for the fiedler emission. Migration-optional. |
| `spectral_embedding_2d` | `eigenvalue.rs:324` | `(graph) -> Vec<[f32; 2]>` | UI-only. **Migration-defer to Scope PI-C.** |

### 1.3 CLI surface (`spectral spectral_index` invocation)

From `/Users/alexwolf/dev/projects/spectral/src/main.rs:362-398` (arm `"index"`):

- **Invocation:** `spectral index [path]` (default `.`).
- **Modes:** two paths. If MCP server socket connectable → JSONRPC dispatch through `spectral_index` MCP tool. Else → direct `graph_cache::load_or_build(path)` fallback.
- **Output shape:** text lines. From `dispatch_spectral_index` in `src/sel/mcp/server.rs:1748-1854`:
  ```
  indexed: <path>
    files:   N (md:X code:Y config:Z mirror:W)
    graph:   N nodes, M edges
    fiedler: 0.NNNN                     ← λ₀(L) via Jacobi
    cascade: settled (new edges) | stable
    ingest:  N coincidence edges        ← optional line
    crystals: N
    oid:     <profile.oid()>
    persisted: git tree at refs/spectral/HEAD
  ```

### 1.4 MCP surface (`spectral_index` tool)

From `/Users/alexwolf/dev/projects/spectral/src/sel/mcp/tools.rs:117-127`:

```json
{
  "name": "spectral_index",
  "description": "Traversal<File, Crystal> — full index pipeline: gestalt import (wide) -> edge detection -> Fate tournament (narrow) -> crystallization. The diamond shape of meaning emerging from a repo. Run on commit for continuous knowledge accumulation.",
  "inputSchema": {
    "type": "object",
    "properties": { "path": { "type": "string", "description": "Directory path to index (defaults to current project)" } }
  }
}
```

**Narrower sibling: `gestalt_detect` MCP tool** (`tools.rs:131-140`). Same input schema; ONLY does Stage 1 (gestalt import + Fiedler + oid), no cascade / no crystallization / no git-tree persistence. **This is the migration target for Scope PI-A** — the narrowest MCP surface that emits the same `fiedler:` line as the bootstrap envelope claim.

### 1.5 Cache format (on disk today)

From `graph_cache.rs`:
- **Cache key = git OID** (Phase 3): the tree at `refs/spectral/HEAD` IS the cache. `load_from_git(path)` walks it; `load_or_build(path)` falls back to fresh scan if ref absent.
- **On-disk layout:** git tree with `nodes/{oid}/{.type,.content,.ts,target_oid_1,target_oid_2,...}` per node subtree, plus a top-level `profile` blob (16 little-endian f64s, optionally prefixed by `spectral-profile\0<ASCII metadata>\n\n`).
- **Legacy stopgap:** `.git/spectral/contexts/{graph,profile}.json` — deleted on encounter (Phase-3 migration shim).
- **Fresh-scan path:** no on-disk cache written by the pure `build_and_cache` route — just returned.

### 1.6 Dependencies (crate-level)

From `crates/gestalt/Cargo.toml`:

```toml
prism-core = { workspace = true, features = ["lambda"] }
terni = { workspace = true }
mirror = { workspace = true }        # ← consumed by document.rs / mirror_domain.rs / dom.rs — NOT by detect/graph/eigenvalue
pulldown-cmark = "0.13"                # ← consumed by encode.rs — NOT by detect/graph/eigenvalue
```

**Load-bearing minimum for the migration surface (`detect` + `graph` + `eigenvalue`):**
- `prism_core::oid::Oid` — content-addressed identity. Mirror has its OWN `bootstrap/src/hash.rs::canonical_hash` + `CoincidenceHash<5,5>` at the same altitude; the migration should route through mirror's hash primitive, NOT pull in `prism-core::Oid`.
- `std::fs`, `std::path`, `std::collections` — all `std`.
- **NO petgraph, NO nalgebra, NO blake3** at gestalt scan altitude. This is important — the pull-in surface is LEAN.

**Delta for `bootstrap/Cargo.toml`:** ZERO new crate deps required if `Oid` is replaced with `blake3::hash` or `canonical_hash` (mirror already has both). One new dep OPTIONALLY: `pulldown-cmark = "0.13"` if we want proper markdown wiki-link extraction — but `extract_markdown_shape` in `detect.rs:346` uses ONLY hand-rolled string scanning (no `pulldown_cmark` import at that path). So the deps delta = **0 new crates**.

---

## §2 — Overlap with existing mirror primitives (TASK 2)

### 2.1 Content-addressing overlap: spectral `Oid` vs mirror `hash`

| Concern | Spectral (gestalt) | Mirror (bootstrap) | Delta |
|---|---|---|---|
| Content-address a node/edge/graph | `prism_core::oid::Oid::hash(bytes)` | `blake3::hash(bytes)` + hex encoding (`init_blake3_oid_hex`, `bootstrap/src/lib.rs:3820`); also `hash::canonical_hash` + `CoincidenceHash<5,5>` (`bootstrap/src/hash.rs`) | REROUTE. Migration replaces `Oid::hash(...)` sites with `init_blake3_oid_hex(blake3::hash(bytes).as_bytes())` — same 32-byte digest, hex-encoded. Preserves determinism; adopts mirror's storage discipline (`blake3` is already the default `MerkleHash` per `bootstrap/Cargo.toml:14`). |

### 2.2 Graph-walk overlap: spectral `walk_detected` vs mirror `psychohistory_root_from_peer_home`

| Concern | Spectral | Mirror | Delta |
|---|---|---|---|
| Recursive directory walk with content-hashing | `walk_detected(root)` + `detect_grammar` classification; produces `Vec<DetectedFile>` + `GestaltBreakdown` | `psychohistory_root_from_peer_home(peer_home)` at `bootstrap/src/lib.rs:4306` — recursive blake3-hash walk producing `(root_oid_hex, moment_count)` | STRUCTURALLY DIFFERENT. `psychohistory_root_from_peer_home` produces a **single aggregate hash** (blake3 over sorted per-file hashes); no classification, no graph edges, no per-file kind. Cannot be extended to produce `ConceptGraph` without adding classification + edge-inference logic on the return path. Better: NEW function `scan_concept_graph(root: &Path) -> (ConceptGraph, GestaltBreakdown)` at same altitude; DON'T force through psychohistory carrier. |

### 2.3 Numerical Fiedler overlap: spectral Jacobi vs mirror `lambda_zero` (LAPACK)

**Load-bearing overlap.** Two Fiedler primitives at different altitudes:

| Concern | Spectral (gestalt) | Mirror (bootstrap) | Delta |
|---|---|---|---|
| λ₀(L) of a symmetric real matrix | `eigenvalue::jacobi_eigenvalues(matrix: &[f64], n: usize) -> Vec<f64>` — hand-rolled Jacobi rotation, O(n³) per sweep, no external deps | `sheaf_laplacian::lambda_zero(op: &Operator) -> Eigenvalue` at `bootstrap/src/sheaf_laplacian.rs:245` — LAPACK `dsyev` via `prismqueer::ffi::eigenvalues`, T8-landed, substrate-decl'd at `shards/epistemologic/math/sheaf_laplacian.mirror` | **PULL-THROUGH.** Route the file-directory `ConceptGraph::laplacian_matrix()` output THROUGH mirror's `sheaf_laplacian::sheaf_laplacian(restrictions)` + `lambda_zero(&op)` pathway. Adapter: convert `ConceptGraph`'s (i, j, weight) edges into `Vec<Restriction>` via `Restriction::new(i as u32, j as u32, w)`. Substrate-honest — mirror's numerical floor already claims this altitude; substrate-decl already lives at `shards/epistemologic/math/sheaf_laplacian.mirror`. |

**BUT NOTE:** `sheaf_laplacian::lambda_zero` returns the smallest STRICTLY-POSITIVE eigenvalue (skips λ = 0 with multiplicity ≥ 1). Spectral's `EigenvalueProfile::fiedler_value` returns `values[1]` — the second-smallest eigenvalue after normalization. On a CONNECTED graph these agree (smallest is 0, second is the Fiedler); on a DISCONNECTED graph they differ — spectral gives 0, mirror's `lambda_zero` skips to the first positive. Migration must preserve **spectral's semantics for the RED-lock equivalence test** OR document the semantic shift as a substrate-honest correction (I recommend the latter — mirror's `lambda_zero` is the substrate-decl'd behavior; substrate-pull-honest is to declare that any discrepancy at disconnected-graph reads is spectral's Jacobi-stub emitting the mathematically-degenerate case; declare and align).

### 2.4 Related mirror primitives (adjacent, not overlap)

| Mirror primitive | File | Adjacency |
|---|---|---|
| `bootstrap/src/spectral.rs::eigen_d<const N: usize>` | 202KB module, `pub fn eigen_d(matrix: [[f64; N]; N]) -> Spectrum<N>` at ~line 1080 | Spectral-triple **Dirac** eigendecomposition; 5×5 canonical basis (five Prism ops). NOT the graph Laplacian altitude. Overlap ZERO. Retain as separate primitive. |
| `bootstrap/src/gap.rs` | 13KB | Gap-tension carriers (the dark-region graph) — different graph substrate than file/dir concept graph. `tensor_of(gaps)` → `Tensor { vertices, tensions, fiedler }`; the `fiedler` field IS λ₀ on the gap-tension graph via `sheaf_laplacian::lambda_zero`. Same numerical primitive, different graph. **Two Fiedler-emitting instances of the same substrate primitive at different altitudes** — this is Alex's multifractal-signature prediction empirically instantiated. Retain both; add file/dir concept graph as third instance. |
| `bootstrap/src/curvature.rs` | 19KB | Balanced-Forman curvature on K_2/K_3/K_4/barbell bridges. Consumes graph structure; adjacent altitude. |
| `bootstrap/src/crystallize.rs` | 42KB | `Splinter<Blake3>` content-addressing at Merkle altitude. Consumer of `blake3::hash` for tree-hashing. |
| `bootstrap/src/hash.rs` | 8KB | `canonical_hash` + `CoincidenceHash<5,5>` — the D-operator's concrete matrix form at spectral-triple altitude. NOT the same altitude as graph-Laplacian OIDs. |

---

## §3 — Consumer map (TASK 3)

### 3.1 Pre-commit hook

**`.githooks/commit-msg`** (this repo, 51 LOC): ONLY checks the FROZEN-`.rs` guard + phase-marker bypass. Does NOT invoke `spectral spectral_index`. The 0.0612-in-commit-envelope observation is NOT wired to this hook.

**`Justfile` / `justfile`** (identical content): `pre-commit` recipe dispatches to `cargo check → cargo test → cargo clippy` gated by `mirror kintsugi mirror.spec`. Does NOT invoke `spectral spectral_index`.

**Global git-hooks** (`~/.os/git-hooks.nix`, per Justfile comment lines 6-8): probe for `just format`, `just pre-commit`, `just pre-push`. Does NOT invoke `spectral spectral_index` at the repo level.

**`bin/mirror-mcp`** (17 LOC): thin shim exec'ing `mirror /dev/stdin @mcp.serve`. Does NOT invoke `spectral spectral_index`.

**VERDICT:** NO shell-level consumer of `spectral spectral_index` runs against this mirror repo. The `fiedler: 0.0612` line seen in commit envelopes is either (a) manually pasted from a prior `mcp__spectral__spectral_index` invocation, or (b) scaffolding vocabulary in commit messages. Not machine-emitted at commit-hook altitude. Confirms prior scout `b52b008` §3.2.

### 3.2 MCP tools available to Claude Code sessions

System-reminder at scout-run time lists (deferred): `mcp__spectral__spectral_index`, `mcp__spectral__gestalt_detect` — plus the other 8 spectral tools (memory_* + spectral_loss). Available to agents; explicit Claude-invoked at agent altitude. **Consumer = agents at session time**, not automation.

### 3.3 Bootstrap consumers of `fiedler` field

Grep `\bfiedler\b` across `bootstrap/src/`:
- `sheaf_laplacian.rs` — computes λ₀ via LAPACK.
- `tensor.rs` — reads `fiedler` on the gap-tension graph (`Tensor.fiedler`, `fiedler_of`).
- `oscillate.rs`, `kintsugi.rs`, `curvature.rs`, `realisation.rs` — consumers of `Tensor.fiedler` in the kintsugi loop.
- `contribute.rs`, `deploy.rs`, `song.rs`, `dance.rs` — Rung 4-7 envelope emitters. **NONE emit `fiedler:` to stdout envelope.** Confirmed by prior scout `b52b008` §3.2.

### 3.4 Docs / roadmap consumers

- `docs/loop/CURRENT.md:181-182` — "Fiedler 0.0612 stable across all 6 rungs" (scaffolding).
- `docs/roadmap/15-fractal-membrane-Asher-tripartition.md:107-115, 137-141` — names the mint-gap; identifies `mcp__spectral__spectral_index` as the current source; declares substrate-honest fold: "bootstrap dispatches through spectral_index for envelope emission, OR bootstrap gains its own λ₀ computation on the DAG."
- `docs/scouts/2026-07-13-taut-fractal-underlies-consent-coherence-empirical-scout.md` — prior scout naming the same mint-gap.
- `docs/specs/mirror-store-bounded-peer-runtime-materialization-as-single-io-crossing.md` — treats 0.0612 as substrate-coherence invariant across the ladder-climb (invariant claim, not measurement).
- `docs/specs/fractal-family-root-mandelbrot-substrate.md` — Mara's canonical spec identifying the compiler with the Mandelbrot set; §6.4 references `0.0612` as "prior substrate telemetry — spectral-triple telemetry per Recognition #74" — same scaffolding-vocabulary anchor.
- `docs/math/2026-07-13-fractal-mandelbrot-substrate.md` — multifractal `f(α)` prediction; consumer of the same anchor.

---

## §4 — Bidirectional dependency check (TASK 4)

### 4.1 Spectral → mirror imports (what spectral consumes from mirror today)

From `spectral/Cargo.toml:36`: `mirror = { path = "../mirror" }` — direct dep at spectral root.
From `spectral/crates/gestalt/Cargo.toml:9`: `mirror = { workspace = true }`.

Grep `mirror::` / `use mirror` / `mirror::*` across `spectral/crates/gestalt/src/`:
- `mirror_domain.rs` (4KB) — Mirror grammar as a gestalt domain. Consumes mirror parser surfaces.
- `document.rs`, `dom.rs`, `encode.rs` — none import mirror directly at scan altitude.
- `detect.rs`, `graph.rs`, `eigenvalue.rs` — **ZERO mirror imports**. Fully self-contained.

From `spectral/src/main.rs`, `spectral/src/sel/mcp/*.rs`: consume `mirror` for compile/kintsugi/spec surfaces — the `spectral mirror <cmd>` subcommand and the mcp compiler bridge. NOT for gestalt scan.

### 4.2 Mirror → spectral imports

Grep `spectral =` / `use spectral` across `bootstrap/Cargo.toml` + `bootstrap/src/`:
- **ZERO**. Mirror has NO import of `spectral` today. Cargo.toml deps at `bootstrap/Cargo.toml:10-170`: `sha2`, `blake3`, `prismqueer`, `terni`, `serde`, `serde_json`, `libc` (cfg-unix), `fragmentation`, `fate`, `fragmentation-git`. No `spectral`, no `gestalt`.

### 4.3 Post-migration state

After Scope PI-A lands:
- mirror gains its own `bootstrap/src/index.rs` (the migrated primitives) — no new dep on spectral.
- spectral retains its `mirror` dep unchanged.
- spectral's `gestalt` crate continues to declare and use its own copy of the primitives (the two implementations coexist at Landing 1).
- Two-tick discipline: Scope PI-C (deferred) replaces spectral's gestalt-scan calls with `mirror index` invocations OR extracts the shared crate to a workspace member both consume. Landing 1 does NOT touch spectral.

### 4.4 Circular dep risk

**Zero risk at Scope PI-A.** Mirror imports only from itself + already-declared deps. Spectral continues to import mirror as before. There is no path spectral→mirror→spectral in the dependency graph.

---

## §5 — Substrate-decl gap analysis (TASK 5)

### 5.1 Family-root altitude

Under @fractal-as-substrate (Alex 2026-07-13 correction, Mara `2c64060` canonical spec `docs/specs/fractal-family-root-mandelbrot-substrate.md`, Mara `3ffa8ed` math foundation `docs/math/2026-07-13-fractal-mandelbrot-substrate.md`):

**The measurement is `@fractal` altitude.** Fiedler on the file-directory concept graph reads the substrate's Fractal-shape at the artifact altitude. Every landed Fractal-shape carrier in the substrate (per prior scout `b52b008` §2.1-§2.2: 3+ Rust-altitude, ~10 substrate-decl-altitude) admits a λ₀(Δ_F) reading; the concept-graph Fiedler is one such reading at the file-tree altitude.

**Recommendation:** `shards/fractal.mirror` (family-root) + `shards/fractal/index.mirror` (species). The family-root mint is blocked on Alex adjudication per docs/roadmap/15 `#6`. Two-tick discipline: land species-under-@mirror provisionally OR wait for family-root landing.

### 5.2 Provisional path (two-tick discipline)

If @fractal family-root is not yet minted at Scope PI-A tick:
- **First tick (provisional):** land at `shards/mirror/index.mirror` (family-root = `@mirror`, species = `index`). Readable name: `@mirror/index`. This composes cleanly with existing @mirror family-root at `shards/mirror.mirror`.
- **Second tick (foundational):** when Alex adjudicates the @fractal family-root, MOVE `shards/mirror/index.mirror` → `shards/fractal/index.mirror`. Substrate-honest two-tick: readable name FIRST, foundational name second.

**Alternative provisional home:** `shards/spectral/index.mirror` — this is problematic because `@spectral` (mirror-side, at `shards/spectral.mirror`) is EXPLICITLY declared a runtime namespace-parent for db/garden/portal/gen_prism; NOT the observation/measurement altitude. Recognition #46 second instance (`shards/spectral.mirror:37-41`) shrunk `@spectral` to namespace-only. Placing `index` under `@spectral` would re-inflate. **Refuse this path.**

### 5.3 Action-decl shape

Substrate-honest first pass (provisional at `@mirror/index`, or foundational at `@fractal/index`):

```mirror
in @prism
in @glass
in @meta
in @io
in @epistemologic/math/sheaf_laplacian

# @mirror/index — the substrate's own λ₀(Δ_F) reading on its file-tree
# concept graph.
#
# Pulls the gestalt-scan primitive from spectral::crates::gestalt into
# mirror-native voice per Recognition #43 (mirror IS content-addressed
# build system) + #55 (form/process partition — DAG is form, measurement
# is process; belong at same altitude).

prism @mirror/index {
  focus     index
  project   detected_file
  split     concept_graph
  shift     eigenvalue_profile
  settle    fiedler
}

detected_file = { path: ~f, kind: grammar_kind }
grammar_kind  = markdown | mirror | code(str) | config(str) | asset | gestalt_native | unknown

concept_graph = { nodes: [graph_node], edges: [graph_edge] }
graph_node    = directory { path: ~d, name: str, depth: u32, file_count: u32 }
              | root      { path: ~d, file_count: u32 }
graph_edge    = contains        { parent_idx: u32, child_idx: u32, weight: f64 }
              | similar_content { a_idx: u32, b_idx: u32, weight: f64 }
              | cross_ref       { source_idx: u32, target_idx: u32, weight: f64 }

eigenvalue_profile = { values: [f64; 16], fiedler: f64, oid: content_address }

action index(peer_home: ~d) -> eigenvalue_profile
  requires @io.filesystem_readable(peer_home)
  requires @epistemologic/math/sheaf_laplacian.lambda_zero.available

out @mirror/index
```

The key action-decl per Alex's directive: `index(peer_home: ~d) -> eigenvalue_profile` — reads the file-tree, builds the concept graph, computes λ₀ via `sheaf_laplacian`, returns the 16-value profile. Composes with existing substrate through `sheaf_laplacian.lambda_zero` (already substrate-decl'd at `shards/epistemologic/math/sheaf_laplacian.mirror`).

### 5.4 Composition with existing substrate-decls

| Existing decl | Composition semantics |
|---|---|
| `@spectral` (at `shards/spectral.mirror`) | UNRELATED namespace-parent (runtime for db/garden/portal). NOT the home. |
| `@mirror/spectral` (at `shards/mirror/spectral.mirror`) | Orchestra observation surface. Adjacent altitude — the SCORE reads @mirror/index's output as one signal on the eigenboard. Consumer, not container. |
| `@kintsugi/oscillate` (per `shards/kintsugi/oscillate.mirror`) | ACTIVE/DARK pass. Reads λ₀ as convergence signal. Consumer of `@mirror/index.fiedler`. |
| `@mirror/mosaic.settle` | Composition semantics. Not directly related; the settle_on predicate could include `fiedler ≤ threshold` in future ticks. |
| `@cyberpunk.cybernetic_coherence` (per `shards/cyberpunk.mirror:168-197`) | "cybernetic_coherence(s) reads λ₀(Δ_F(s))". IS the substrate-decl declaring @coherence AS λ₀. Consumer of `@mirror/index.eigenvalue_profile.fiedler`. **@mirror/index IS one carrier that discharges @cyberpunk.cybernetic_coherence at the file-tree altitude.** |
| `@epistemologic/math/sheaf_laplacian.lambda_zero` | The numerical primitive. Consumed BY `@mirror/index.index`. |

---

## §6 — Scope PI-A minimum-viable execution plan (TASK 6)

### 6.1 File inventory

**NEW:** `bootstrap/src/index.rs` — single file, forked-from-spectral primitives.

Rough LOC + composition:

```
// bootstrap/src/index.rs — ~1200 LOC

// §1 — detect (from spectral::gestalt::detect)  ~450 LOC
//   GrammarKind, DetectedFile, GestaltBreakdown, MarkdownShape, CodeShape
//   detect_grammar, walk_detected, walk_recursive, load_gitignore, is_gitignored,
//   should_skip_dir, extract_markdown_shape, extract_code_shape

// §2 — graph (from spectral::gestalt::graph)  ~350 LOC
//   GraphNode, GraphEdge, ConceptGraph
//   build_concept_graph, compute_type_distributions, cosine_similarity,
//   extract_cross_references

// §3 — eigenvalue (from spectral::gestalt::eigenvalue)  ~200 LOC
//   EigenvalueProfile
//   eigenvalue_profile — REROUTED through sheaf_laplacian::sheaf_laplacian + lambda_zero
//   build_profile
//   (jacobi_eigenvalues DROPPED — replaced by lambda_zero)
//   (spectral_embedding_2d DEFERRED to Scope PI-C — UI-only)

// §4 — hash adapter  ~30 LOC
//   Replace `Oid::hash(bytes)` sites with `blake3::hash(bytes)` + hex encoding.
//   Route through existing `init_blake3_oid_hex` helper.

// §5 — public entry — the action-decl discharge
//   pub fn index(peer_home: &Path) -> EigenvalueProfile
//   Composes walk_detected + build_concept_graph + eigenvalue_profile.

// §6 — tests
//   Fork all landed tests from spectral (concept_graph_*, eigenvalue_*, etc.)
//   NEW: fiedler_matches_spectral_reference — RED lock (see §6.4).
```

**MODIFIED:** `bootstrap/src/lib.rs` — add `pub mod index;` module declaration + `cmd_index(args)` dispatcher + Ctx composition arm. ~50 LOC delta.

**MODIFIED:** `bootstrap/src/mcp.rs` — add `mirror_index` tool entry to `tools_list_result` + `dispatch_tool_call` arm. ~40 LOC delta.

**MODIFIED:** `mirror.spec` — add `command index` block. ~15 LOC delta.

**NEW:** `shards/mirror/index.mirror` (or `shards/fractal/index.mirror` if family-root already minted) — substrate-decl per §5.3. ~120 LOC.

**NEW:** `bootstrap/tests/index_fiedler_equivalence.rs` — RED-lock test. ~80 LOC.

### 6.2 CLI additions to `mirror.spec`

```mirror
# === index — file-tree concept-graph Fiedler measurement ===
#
# Reads a directory; classifies files by grammar kind; builds a
# concept graph (directory-nodes + Contains/SimilarContent/CrossRef
# edges); computes λ₀(Δ_F) via @epistemologic/math/sheaf_laplacian.
# Substrate-honest fold per Recognition #43 + #55 (docs/roadmap/15;
# docs/scouts/2026-07-13-taut-fractal-underlies-consent-coherence-
# empirical-scout.md). Pull-in from spectral::crates::gestalt (Scope
# PI-A per docs/scouts/2026-07-13-taut-spectral-to-mirror-migration-
# mapping-scout.md).
#
# @mirror/index (or @fractal/index at family-root landing).
command index {
  arg path: ~d
  flag json: bool = false
}
```

### 6.3 Lib.rs dispatch arm additions

Inside `Ctx::command` handling (following the `craft` / `init` / `recall` / `beam` / `peer` pattern in `mirror.spec:82-311`):

```rust
// bootstrap/src/lib.rs — new arm alongside cmd_craft / cmd_init / cmd_recall
"index" => cmd_index(args, ctx),
```

Where `cmd_index` is:

```rust
pub fn cmd_index(args: &[String], ctx: &Ctx) -> i32 {
    let path_str = args.get(0).map(|s| s.as_str()).unwrap_or(".");
    let json = args.iter().any(|a| a == "--json");
    let path = Path::new(path_str);
    let profile = crate::index::index(path);
    // ... emit text or JSON envelope with `fiedler:`, `nodes:`, `edges:`, `oid:`
    // matching spectral_index output shape for byte-parity where feasible
    0
}
```

### 6.4 Test contract — RED lock target

**T1 — reference-run capture (Reed 🔴 Landing 1):**
1. Reed invokes `mcp__spectral__spectral_index` with path `/Users/alexwolf/dev/projects/mirror` AT the RED-commit-tick.
2. Records: `fiedler_ref: f64`, `oid_ref: String`, `nodes_ref: usize`, `edges_ref: usize`, `md_count: u32`, `code_count: u32`, `config_count: u32`, `mirror_count: u32`.
3. Writes into `bootstrap/tests/index_fiedler_equivalence.rs` as `const` values.

**T2 — RED-lock (Reed 🔴 Landing 1 continued):**
```rust
// bootstrap/tests/index_fiedler_equivalence.rs

// Reference values captured 2026-07-13 via `mcp__spectral__spectral_index`
// on the mirror repo at commit <REDLOCK_COMMIT_HEX>. RED-lock target for
// Scope PI-A per docs/scouts/2026-07-13-taut-spectral-to-mirror-
// migration-mapping-scout.md §6.4.
const FIEDLER_REF:   f64    = /* run-and-record */;
const NODES_REF:     usize  = /* run-and-record */;
const EDGES_REF:     usize  = /* run-and-record */;
const OID_REF:       &str   = /* run-and-record */;
const FIEDLER_EPSILON: f64  = 1e-3;   // Jacobi-vs-LAPACK numerical gap

#[test]
fn index_matches_spectral_reference_on_mirror_repo() {
    let profile = mirror::index::index(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap());
    assert!((profile.fiedler_value() - FIEDLER_REF).abs() < FIEDLER_EPSILON,
            "Fiedler {} outside ±{} of spectral reference {}",
            profile.fiedler_value(), FIEDLER_EPSILON, FIEDLER_REF);
    // OID equivalence deferred to Scope PI-B (byte-parity oid encoding).
}
```

**T3 — structural tests (fork from spectral):**
All 20+ tests from `crates/gestalt/src/{detect,graph,eigenvalue}.rs::tests` — cover empty dir, single file, directory nesting, wiki-link cross-ref, similar-content dirs, adjacency matrix, Laplacian matrix, K₂/K₃/P₃ eigenvalues, determinism.

**Load-bearing epsilon:** `1e-3` is generous — Jacobi's iteration-convergence tolerance in `eigenvalue.rs:144` is `1e-12`; LAPACK `dsyev` typical accuracy is `~1e-14`. The gap should collapse to `1e-9` after the routing-through-lambda_zero settles; the loose bound protects against normalization-order differences (spectral normalizes to `[0,1]` post-hoc; mirror's `sheaf_laplacian` returns raw λ). Byte-parity of the OID is DEFERRED — the OID depends on the normalized 16-value formatting, and the encoding will differ between spectral's `Oid::hash` and mirror's `blake3::hash` unless we import spectral's exact byte-format.

### 6.5 Cargo dep additions

**None.** All primitives use `std` + `blake3` (already declared) + `prismqueer::ffi::eigenvalues` (already declared via `prismqueer = { version = "0.1", features = ["bundle", "lapack"] }`). Confirmed via §1.6 above.

### 6.6 MCP addition (Scope PI-B forward-promise)

Schema at `bootstrap/src/mcp.rs` `tools_list_result`:

```json
{
  "name": "mirror_index",
  "description": "index: file-tree concept-graph Fiedler measurement. Classifies files by grammar kind; builds concept graph (directory nodes + Contains/SimilarContent/CrossRef edges); computes λ₀(Δ_F) via @epistemologic/math/sheaf_laplacian. Substrate: @mirror/index (or @fractal/index at family-root landing). Returns { fiedler, nodes, edges, oid, breakdown }.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": { "type": "string", "description": "Directory path to index (defaults to current project)" }
    }
  }
}
```

Dispatch arm at `dispatch_tool_call`:

```rust
"mirror_index" => {
    let path_str = arguments.get("path").and_then(|v| v.as_str()).unwrap_or(".");
    let profile = crate::index::index(Path::new(path_str));
    // emit envelope JSON matching spectral_index output shape
}
```

MCP schema wire-up = Scope PI-B (forward-promise from Scope PI-A per two-tick discipline).

---

## §7 — Migration risk register (TASK 7)

### 7.1 If Scope PI-A lands and pre-commit hook keeps calling `spectral spectral_index`

**Nothing breaks.** Pre-commit hooks in this repo do NOT invoke `spectral spectral_index` (confirmed §3.1). Both implementations coexist without shell-level conflict. Two-tick pattern: mirror gains its own voice; spectral continues to serve agent MCP calls.

### 7.2 Tests that could regress

- **`bootstrap/tests/*` — LAPACK link:** any change routing to `sheaf_laplacian::lambda_zero` inherits the LAPACK link. If the test target isn't already linking LAPACK, adding a Fiedler-computing test suite pulls it in. Mitigation: LAPACK is ALREADY declared (`prismqueer = { features = ["bundle", "lapack"] }`); no new linkage.
- **`bootstrap/tests/kintsugi_*.rs`, `tensor::tests`** — these already use `sheaf_laplacian::lambda_zero`; no change.
- **`docs/loop/CURRENT.md` "Fiedler 0.0612 stable" claim** — regressing this is DESIRED. The scaffolding-vocabulary claim gets replaced with a live measurement; the number will change from `0.0612` to whatever `index` actually emits. Cascade update needed at CURRENT.md + docs/roadmap/15 + docs/specs/mirror-store-bounded-peer-runtime.

### 7.3 Hidden state / cache files

- **`.mirror/`** — check `mirror init` output. `bootstrap/src/lib.rs::psychohistory_root_from_peer_home` skips `.mirror/` per its docstring; `walk_detected` in the migrated code should adopt the same discipline. Update `should_skip_dir` in the fork to add `.mirror` to the ignore-list.
- **`.spectral/`** — spectral's session state. Already skipped by `walk_detected` (`should_skip_dir:196-206`).
- **`.git/spectral/contexts/{graph,profile}.json`** — legacy Phase-3 stopgap, deleted-on-encounter by spectral's `cleanup_legacy_json`. Do NOT fork this cleanup logic into mirror (Scope PI-A is compute-only; no persistence).
- **`refs/spectral/HEAD`** — the git-native cache spectral writes. mirror at Scope PI-A does NOT read or write this ref. Scope PI-A is stateless.

### 7.4 @io crossings

**Two implementations means two @io crossings if both are called at the same tick.** Substrate-honest reading:
- Landing 1 (Reed): mirror computes its OWN λ₀ per invocation. @io crossing = `std::fs::read_dir` walk. ONE per `mirror index` call.
- Existing spectral: MCP call to `spectral_index` computes AT spectral's @io crossing. SEPARATE crossing.
- If a workflow calls both: two @io crossings for the same underlying data. Fine at Landing 1 (two-tick discipline). Scope PI-C collapses.

### 7.5 FROZEN .rs bypass

**`.githooks/commit-msg` blocks all `.rs` additions/modifications** unless the message carries `[bugfix:restore]` or `[substrate-pull:realize]`. Scope PI-A is precisely `[substrate-pull:realize]` — the substrate-decl at `shards/mirror/index.mirror` NAMES the primitive, and `bootstrap/src/index.rs` realizes it. Use `[substrate-pull:realize]` marker on every Rust-modifying commit in this cascade.

### 7.6 Naming collision

**Two `EigenvalueProfile` types in spectral (§1.1).** The migration takes ONLY the eigenvalue.rs version. The spectral.rs `EigenvalueProfile` (with `id`, `label` fields) is a viz block — not needed for the Fiedler emission. Confirm during fork.

### 7.7 Fractal family-root adjudication blocker

Per `docs/roadmap/15:180`: `shards/fractal.mirror` family-root mint is blocked on Alex adjudication #6. Scope PI-A can proceed at `@mirror/index` provisional altitude (two-tick discipline). If Alex lands the @fractal family-root at the same tick, land as `@fractal/index` directly and skip the provisional tick.

### 7.8 Disconnected-graph semantic drift

Per §2.3: spectral's `EigenvalueProfile::fiedler_value()` returns `values[1]` (raw); mirror's `lambda_zero` skips zero eigenvalues. On disconnected graphs, spectral emits `0.0` (from `values[1] == 0` after normalization); mirror emits the first positive eigenvalue. **Substrate-honest position:** document the shift; align to mirror's semantics (which match `shards/epistemologic/math/sheaf_laplacian.mirror` and Bodnar 2022 §2). RED-lock uses ±ε epsilon to absorb the discrepancy on connected graphs; add explicit disconnected-graph test asserting mirror's behavior.

---

## §8 — Recommended sequence (top-5 concrete substrate-honest ordering)

Each step is Reed-alone landable except where marked (Mara for substrate-decl authorship per @mirror/pack conventions). Commit-as attributions per `CLAUDE.md` §Pack peers.

### Landing 1 (Mara 📝) — substrate-decl mint

**Author:** Mara <mara@systemic.engineer>  
**Marker:** `📝 Mara [substrate-pull:realize] [scout:spectral-to-mirror-migration-mapping] Landing 1 — @mirror/index substrate-decl mint (or @fractal/index if family-root landed)`  
**Files:**
- NEW: `shards/mirror/index.mirror` (~120 LOC per §5.3 skeleton — Mara fills full docstring with substrate ancestry).
- MODIFIED: `docs/loop/CURRENT.md` — mark Scope PI-A active.
- MODIFIED: `docs/roadmap/15-fractal-membrane-Asher-tripartition.md` — mark @mirror/index (provisional) OR @fractal/index (foundational) as substrate-decl'd; update mint-gap section.

**--no-verify:** OK (pure-docs 📝 bypass — markdown + `.mirror` only). Substrate-decl at `.mirror` altitude is NOT `.rs`; FROZEN check passes.

### Landing 2 (Reed 🔴) — reference-capture + RED lock

**Author:** Reed <reed@systemic.engineer>  
**Marker:** `🔴 Reed [substrate-pull:realize] [tdd:spectral-index-parity] Landing 2 — reference-capture + RED lock for mirror::index::index`  
**Steps:**
1. Reed invokes `mcp__spectral__spectral_index` with path = mirror repo root. Records `fiedler_ref`, `oid_ref`, `nodes_ref`, `edges_ref`, `breakdown_counts`.
2. Writes `bootstrap/tests/index_fiedler_equivalence.rs` with reference constants + RED-lock test per §6.4.
3. Commit expects RED (module `mirror::index` not yet declared) — cargo test fails on missing symbol.

**Files:**
- NEW: `bootstrap/tests/index_fiedler_equivalence.rs` (~80 LOC).

**--no-verify:** REQUIRED. `[substrate-pull:realize]` marker bypasses FROZEN check. NEVER modify FROZEN bypass without in-transcript Alex authorization.

### Landing 3 (Reed 🟢) — forked primitives + lambda_zero routing

**Author:** Reed <reed@systemic.engineer>  
**Marker:** `🟢 Reed [substrate-pull:realize] [tdd:spectral-index-parity] Landing 3 — bootstrap/src/index.rs GREEN via sheaf_laplacian.lambda_zero routing`  
**Steps:**
1. Fork `detect.rs` primitives into `bootstrap/src/index.rs` §1 (~450 LOC).
2. Fork `graph.rs` primitives into §2 (~350 LOC).
3. Fork `eigenvalue.rs::EigenvalueProfile` + `eigenvalue_profile` + `build_profile` into §3 (~200 LOC). REPLACE `jacobi_eigenvalues` with adapter routing through `sheaf_laplacian::sheaf_laplacian` + `lambda_zero`.
4. Fork tests from spectral (concept_graph_*, adjacency_matrix_*, laplacian_matrix_*, eigenvalue_*).
5. Replace `prism_core::oid::Oid` sites with `blake3::hash` + `init_blake3_oid_hex`.
6. Update `should_skip_dir` to include `.mirror`.
7. Add `pub mod index;` at `bootstrap/src/lib.rs`.
8. Run `cargo test index_fiedler_equivalence` — expect GREEN within `FIEDLER_EPSILON`.

**Files:**
- NEW: `bootstrap/src/index.rs` (~1200 LOC).
- MODIFIED: `bootstrap/src/lib.rs` (+1 LOC `pub mod index;`).

**--no-verify:** REQUIRED (`[substrate-pull:realize]`).

### Landing 4 (Reed 🟢) — CLI wire-in via `mirror.spec` + cmd_index

**Author:** Reed <reed@systemic.engineer>  
**Marker:** `🟢 Reed [substrate-pull:realize] [tdd:mirror-index-cli] Landing 4 — mirror.spec command index + bootstrap dispatch`  
**Steps:**
1. Add `command index { arg path: ~d; flag json: bool = false }` to `mirror.spec` cli-block per §6.2.
2. Add `cmd_index` dispatcher at `bootstrap/src/lib.rs` per §6.3.
3. Add CLI-dispatch test at `bootstrap/tests/mirror_index_cli.rs` — `mirror index <path>` emits expected envelope shape.

**Files:**
- MODIFIED: `mirror.spec` (~15 LOC).
- MODIFIED: `bootstrap/src/lib.rs` (~40 LOC).
- NEW: `bootstrap/tests/mirror_index_cli.rs` (~60 LOC).

**--no-verify:** REQUIRED (`[substrate-pull:realize]`).

### Landing 5 (Reed 🟢) — MCP wire-in (Scope PI-B closure)

**Author:** Reed <reed@systemic.engineer>  
**Marker:** `🟢 Reed [substrate-pull:realize] [tdd:mirror-index-mcp] Landing 5 — mirror_index MCP tool schema + dispatch`  
**Steps:**
1. Add `mirror_index` tool schema to `tools_list_result` per §6.6.
2. Add dispatch arm at `dispatch_tool_call`.
3. Add MCP roundtrip test at `bootstrap/tests/mcp_mirror_index.rs`.

**Files:**
- MODIFIED: `bootstrap/src/mcp.rs` (~50 LOC).
- NEW: `bootstrap/tests/mcp_mirror_index.rs` (~80 LOC).

**--no-verify:** REQUIRED (`[substrate-pull:realize]`).

### Deferred (Scope PI-C) — spectral consumer refactor

Out of scope PI-A. Two paths available:
- **Path α (consumer-pull):** Spectral's `gestalt::graph` + `gestalt::eigenvalue` become thin adapters over `mirror::index`. Spectral binary continues to expose `spectral_index` MCP tool but computes via mirror's primitive.
- **Path β (workspace extraction):** Extract mirror's `index.rs` into a `mirror-index` workspace member; both mirror and spectral consume it.

Mara adjudicates path selection at Scope PI-C tick.

---

## §9 — Substrate-honest signals for Reed

**Signal 1 (green light):** Migration is architecturally clean. Zero circular-dep risk. Zero new Cargo deps. Existing mirror numerical floor (`sheaf_laplacian::lambda_zero`) already claims the altitude — the pull-in is substrate-pull-realize discipline, not substrate-invention.

**Signal 2 (adjudication needed):** Family-root altitude question. `@mirror/index` (provisional, ships now) vs `@fractal/index` (foundational, blocked on Alex #6). Recommend provisional-first, two-tick collapse. Do NOT block Scope PI-A on the family-root adjudication.

**Signal 3 (LOAD-BEARING):** The `0.0612` value in prose is scaffolding vocabulary. Do NOT hard-code `0.0612` as the RED-lock target. RUN `mcp__spectral__spectral_index` at the RED-commit tick, RECORD the actual value against the current mirror repo state, LOCK to that. The scaffolding number reflects Alex's memory of a prior invocation — the substrate-honest target is whatever `spectral_index` emits today against this commit.

**Signal 4 (composition win):** Once `bootstrap/src/index.rs` lands, mirror carries THREE Fiedler-emitting instances of the same numerical primitive at three altitudes: file-tree concept graph (`index.rs`), gap-tension AST graph (`tensor.rs`), and sheaf-Laplacian test surface (`sheaf_laplacian.rs`). This IS Alex's multifractal-signature prediction empirically instantiated — the same substrate primitive at multiple altitudes with different λ₀ readings.

**Signal 5 (cascade update trigger):** Landing 3 GREEN → cascade updates required at `docs/loop/CURRENT.md`, `docs/roadmap/15-fractal-membrane-Asher-tripartition.md`, `docs/specs/mirror-store-bounded-peer-runtime-materialization-as-single-io-crossing.md`. Any `"Fiedler 0.0612 stable"` claim gets replaced with the actual measured value at the landing commit. Substrate-honest: the number becomes a MEASUREMENT, not scaffolding vocabulary.

---

*Taut scout complete. Read-only. No substrate edits. Migration surface mapped: 3 files (~2237 LOC) fork target; 0 new Cargo deps; existing `sheaf_laplacian.lambda_zero` reuse win; 5-landing sequence Reed-executable with Mara 📝 substrate-decl at Landing 1. RED-lock target = live `mcp__spectral__spectral_index` reference-capture (NOT hardcoded 0.0612). Family-root altitude adjudication is orthogonal blocker — provisional @mirror/index ships without waiting.*
