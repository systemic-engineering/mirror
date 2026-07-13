# Taut scout — Fate::bounded call-site audit + @fractal tripartition + @io-crossing inventory + Rung 7 5-blob jurisdiction

*Scout, read-only, 2026-07-13.*

*Trigger:* Alex in-transcript verbatim 2026-07-13:
- "What about Fate::bounded? We added it, why aren't we using it? It maps directly onto the sheaf math."
- "How can we formalize this into a @fractal surface which we then use to compose all @io facing layers in mirror?"
- "The whole point of mirror is to minimize @io crossings and stay in @magic non-linear Eigenvalue land as long as possible."

*Ancestry:* Kimberley Asher, *Meaning Is Not a Metric* (2026-07-10, 15pp) —
`/Users/reed/dev/systemic.engineering/blog/_src/kimberley-asher_meaning-is-not-a-metric.pdf` — the Orchard tripartition of *evidential witnesses* / *constitutional gates* / *governance authority* + the load-bearing sentence "Evidence may support. Gates may permit. Authority may act. None automatically converts into another."

*Rung-7 GREEN under adjudication:* Reed `a2c71fd` `bootstrap/src/contribute.rs`.

## Executive summary

Three substrate-honest verdicts (top 3):

1. **Fate::excited is the wrong call at every non-anonymous site.** Three Rust call sites reach for `Fate::excited()` (wall-clock-seeded xorshift, non-deterministic); only one of them — `fate_select_peer_beam` — is anonymous / origin-of-torus and thus substrate-honestly excited. `contribute.rs::peer_contribute` (Rung 7) and every future `--fate-select` composition against a named peer should be `Fate::untrained() + selectors_from_psychohistory_root(...)` — the pattern `fate_bounded_by_psychohistory_peer_beam` already discharges as v1 stub of `Fate::bounded(cfg)` per Mara `ce9745f`. **Rung 7 substrate-honest correction:** swap the `Fate::excited()` at `contribute.rs:63` for the bounded pattern reading the peer's own psychohistory sheaf. The sheaf math Alex named IS this substitution.

2. **@fractal is not a new family-root. @kintsugi/consent IS the tripartition surface at auto-apply altitude.** `shards/kintsugi/consent.mirror` already carries all three Asher jurisdictions: **evidential witnesses** = `loss_decreasing` + `identity_preserving` (per-morphism), + `is_pareto` (per-set) + `is_settled` (per-trajectory); **constitutional gate** = `admissibility_singleton` + the pre-filter that removes candidates failing the two per-morphism gates; **governance authority** = `should_auto_apply` / `should_escalate` / `emit_to_metalogue`. `query_phi(candidates) -> verdict` IS the tripartition composed into one substrate-fact. Promote to family-root altitude by *citing consent from every @io-facing surface* (Rung 7's `settle_rust_workspace` currently bypasses it); do NOT mint `@fractal`. The self-similar / multi-axial reading Alex named is `@kintsugi/consent` recursively applied at every altitude — same five ops, same verdict carrier, same `promote reluctantly / demote readily` asymmetry.

3. **Rung 7's 5-blob tree has zero jurisdictional separation.** Per Asher: witnesses / gates / authority are structurally distinct subtrees. The 5-blob tree at `contribute.rs::materialize_morphism` (pre-anchor / post-anchor / morphism-body / settle-verdict / fate-witness) mixes all three at one altitude with no path-namespace discipline, no gate check, and treats the settle-verdict as an evidential witness when it is a constitutional gate. Refactor: `witnesses/{pre-anchor, post-anchor, morphism-body}` (evidence of the transition), `gates/{settle-verdict, glass-wall-crossing-count}` (permission to admit; per Asher gates cast **zero votes**), `authority/{fate-witness, consent-verdict}` (governance record). Missing entirely: a **consent-verdict** blob — Rung 7 discharges directly to `settled` / `imperfect` without going through `query_phi`, so the auto-apply boundary is currently *implicit in Rust code* instead of *substrate-fact*. That's the training-pull; the substrate-honest reading uses `@kintsugi/consent`.

Everything below is the empirical audit that grounds these three verdicts.

---

## §Task 1 — Fate::bounded call-site audit

### 1.1 The mint status

There is no `Fate::bounded(cfg)` **implementation** in the fate crate today (`/Users/alexwolf/dev/projects/fate/src/lib.rs`). What exists:

| Symbol | Location | State |
|---|---|---|
| `Fate::excited()` | `fate/src/lib.rs:~310+` | LANDED — xorshift64 seeded from `SystemTime::now().nanos ^ addr(excited)` |
| `Fate::untrained()` | `fate/src/lib.rs:~260+` | LANDED — all-zero weights → uniform |
| `Fate::bounded(cfg)` | *(not in fate crate)* | SPEC-LANDED at `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` (Mara `96ff532`); SUBSTRATE-LANDED as `@fate/tournament.bounded_by(sheaf, perturbation) -> tournament_result` at `shards/fate/tournament.mirror:942` (Mara `ce9745f`); Rust discharge forward-promised at `fate/src/bounded.rs` |
| `fate_engine.selectors = selectors_from_psychohistory_root(root)` | `bootstrap/src/lib.rs:4555, 4683-4684` | LANDED as **v1 stub** — this IS `Fate::bounded(BoundedConfig { weights })` composed inline. v1 xorshift64 seeded from psychohistory_root; v2 = sheaf-Laplacian Δ_F Rayleigh direction (Mara iter-30 §3). |

**Verdict:** Alex's question "we added it, why aren't we using it?" reads honestly as: the *substrate-decl* landed (`bounded_by` at `@fate/tournament`); the *v1 Rust discharge* landed (`selectors_from_psychohistory_root` at `bootstrap/src/lib.rs`); but the *call sites* have not been updated to reach through the bounded pattern. The bounded pattern IS the untrained + selectors-mutation composition; it just doesn't have a single-symbol Rust name yet.

### 1.2 Every `Fate::excited()` call site

3 call sites in the tree (excluding test / spec docstrings):

| Call site | File:line | Substrate-honest? | Bounded-alternative psychohistory root |
|---|---|---|---|
| **CS-1** `fate_select_peer_beam` | `bootstrap/src/lib.rs:4781` | **YES.** This site is anonymous / origin-of-torus — `Features::default()` explicitly names "no observation yet"; the peer has no history to bound against. `Fate::excited()` is substrate-correct here (Seam `d9b7c35` Adj 1 verdict landed). | N/A — this IS the anonymous case; bounded pattern would over-constrain. |
| **CS-2** `contribute.rs::peer_contribute` (Rung 7) | `bootstrap/src/contribute.rs:63` | **NO.** `peer_home` IS a named-peer context; `psychohistory_root_from_peer_home(peer_home)` was already implemented; using `Fate::excited()` here throws away the substrate ground truth Alex named ("it maps directly onto the sheaf math"). | `psychohistory_root_from_peer_home(peer_home)` — Rung 6.1c witness path. |
| **CS-3** MCP `bootstrap/src/mcp.rs:mirror_peer_beam` inputSchema docstring (2× at property descriptions) | `bootstrap/src/mcp.rs:~174, ~183, ~275` | Documentation-only; refers to `--fate-select` altitude. Should read "Fate::excited() OR Fate::bounded(...) per --from-psychohistory flag composition" to name both regimes. | Docstring drift; low-priority. |

### 1.3 Every `Fate::untrained() + selectors = X` bounded-pattern call site

2 call sites, both discharging the bounded pattern as v1 stubs:

| Call site | File:line | Composition |
|---|---|---|
| **BS-1** `fate_bounded_shadow_peer_beam` (`--from-psychohistory --with-shadow`) | `bootstrap/src/lib.rs:4555` | `Fate::untrained()` + `selectors_from_psychohistory_root(psychohistory_root_oid)` + `cast_shadows_over_models(...)` + `shadow_regime(...)`. **Substrate-honest.** |
| **BS-2** `fate_bounded_by_psychohistory_peer_beam` (`--from-psychohistory`) | `bootstrap/src/lib.rs:4683-4684` | `Fate::untrained()` + `selectors_from_psychohistory_root(psychohistory_root_oid)` + `resolve(&features, 5)`. **Substrate-honest.** Comment names the intent verbatim: "This is `Fate::bounded(config)` where config.weights is derived from the peer's psychohistory sheaf via deterministic stub (v1); v2 will replace with sheaf-Laplacian Δ_F Rayleigh direction." |

**Drift count:** 1 site (CS-2 `contribute.rs`) reaches for `excited` when `bounded` is available. CS-1 is substrate-honest excited (anonymous case). CS-3 is documentation.

### 1.4 The single landed bounded-pattern site: completeness read

`fate_bounded_by_psychohistory_peer_beam` (BS-2) is the one landed empirical discharge of the bounded pattern. Reading it against Mara iter-30 §3:

- ✅ `Fate::untrained()` as base (private connection field zeroed) — matches BoundedConfig §5.
- ✅ `selectors` mutated to psychohistory-derived weights — matches BoundedConfig.weights (Level 0 Fiber sections).
- ✅ `resolve(&features, 5)` — the existing Rayleigh descent through selectors[4] recursive tower per fate crate.
- ❌ **Missing:** BoundedConfig.holonomy_ceiling (Level 3 Transport bound per `bounded_by_respects_holonomy`); depth_cap (Level 4 Closure bound per `bounded_by_below_lawvere_depth`).
- ❌ **v1 xorshift stub blocks:** the seed is `first_8_bytes(root_hex) as u64`; downstream weights are `xorshift64(seed) * uniform_scale`. This IS a deterministic hash-of-content → weights map; it is NOT sheaf-Laplacian Δ_F Rayleigh direction extraction. The direction chosen is content-addressed but ARBITRARY w.r.t. the graph structure.

**Shortest path to v2 (per Mara iter-30 §3):**

1. **Build the sheaf.** `psychohistory_root_from_peer_home` already walks the peer_home and computes moment OIDs; the sheaf F over @bauchladen.tray is the OID-graph rooted at psychohistory_root_oid. This is `walk(root: oid) -> splinter_graph` at @mirror/store altitude (already declared).
2. **Compute Δ_F.** Sheaf-Laplacian per `shards/epistemologic/math/sheaf_laplacian.mirror` (LANDED). The operator is over the splinter_graph edges.
3. **Rayleigh descent.** First eigenvector v₁ of Δ_F = direction of maximum coherence gain. `weights = v₁_projected_onto_5_by_16` (5 models × 16 features).
4. **Ceiling / depth.** BoundedConfig.holonomy_ceiling = sum-of-signed-magnitudes cap; BoundedConfig.depth_cap = max resolve() depth. Both are substrate-decl'd at `shards/fate/tournament.mirror:942-995`.

Prerequisite for v2: `sheaf_laplacian` needs a Rust-altitude runtime OR the peer_home walk needs a numerical Δ_F kernel. Neither is in bootstrap today (per `feedback-craft-not-deliver` and the FROZEN-bootstrap discipline).

### 1.5 Recommendation (grep-first, non-synthetic)

- **Rung 7 correction (single-line):** at `contribute.rs:63`, replace `let fate_engine = fate::Fate::excited();` with the bounded pattern; introduce `let (psychohistory_root_oid, _) = psychohistory_root_from_peer_home(peer_home);` and `let mut fate_engine = fate::Fate::untrained(); fate_engine.selectors = selectors_from_psychohistory_root(&psychohistory_root_oid);`. The rest of `peer_contribute` unchanged.
- **The refactor Alex is naming:** `Fate::bounded(cfg: BoundedConfig)` at `fate/src/lib.rs` as a single-symbol constructor consuming the psychohistory-derived weights. Once landed, all four bounded call sites collapse to `let fate_engine = fate::Fate::bounded(BoundedConfig::from_psychohistory_root(&root));`. This is Mara iter-30 §5 landing Tick 3 — forward-promised at `shards/fate/tournament.mirror:942`.

---

## §Task 2 — @fractal / tripartition substrate-already-had-the-word scout

Alex's question: "How can we formalize this into a @fractal surface which we then use to compose all @io facing layers in mirror?"

The substrate carries three Asher-tripartition candidates. I audit each against Asher's load-bearing sentence:

> "Evidence may support. Gates may permit. Authority may act. None automatically converts into another." — Asher p. 10

### 2.1 @kintsugi/consent — THE canonical tripartition surface

`shards/kintsugi/consent.mirror` (39KB, 41 hits on tripartition keywords).

| Asher role | @kintsugi/consent surface | Function |
|---|---|---|
| **Evidential witness** | `loss_decreasing(m: morphism) -> verdict` | Per-morphism; reads dissonance-holonomy; supports patternhood. |
| **Evidential witness** | `identity_preserving(m: morphism) -> verdict` | Per-morphism; reads uuid_spectral DARK bits; supports patternhood. |
| **Evidential witness** | `is_pareto(candidates: pareto_set) -> verdict` (from `shards/epistemologic/math/music/dissonance.mirror`, imported) | Per-set; supports patternhood via pareto-front discrimination. |
| **Evidential witness** | `is_settled(c: cadence_kind) -> verdict` (from `shards/epistemologic/math/music/cadence.mirror`, imported) | Per-trajectory; supports patternhood via authentic/plagal/half/deceptive read. |
| **Constitutional gate** | `admissibility_singleton(candidates) -> verdict` | "Permits or prohibits consideration"; zero votes toward patternhood; filters before the ranking runs. |
| **Governance authority** | `should_auto_apply(c: verdict) -> verdict` | Determines which subsystem may act — the formatter auto-applies. |
| **Governance authority** | `should_escalate(c: verdict) -> verdict` | Determines the escalation dispatch — the metalogue bridge fires. |
| **Governance authority** | `emit_to_metalogue(p: pause_event) -> turn` | The ONE write of an authority action — surfaces the pause event as a metalogue turn. |
| **Compositional query** | `query_phi(candidates: morphism_set) -> verdict` | THE structural Φ query. Composes gate + gate → rank → verdict at consent altitude. |

**Non-redundance:** `loss_decreasing` reads dissonance; `identity_preserving` reads uuid_spectral dark bits; `is_pareto` reads the pareto-front; `is_settled` reads cadence axis. Four **non-redundant** evidential witnesses per Asher's "five gauges connected to the same pipe" test.

**Jurisdictional-separation:** the substrate-decl comment at `consent.mirror:504` names it verbatim:

> "Symmetry note: admissibility_singleton IS the consent-altitude specialisation of dissonance.mirror's is_pareto. The two are the same verdict observed at two altitudes — dissonance's pause becomes consent's failure; dissonance's singleton becomes consent's pass."

Gates are the **consent-altitude reading** of witnesses. Authority (`should_auto_apply` / `should_escalate`) is the **third altitude** above both. Asher's three-layer taxonomy IS the three-altitude composition already in consent.

**Promote-reluctantly-demote-readily:** the pause_event carrier + `emit_to_metalogue(p)` + `partial(confidence)` middle state ARE the conservative membrane Asher names. Auto-apply requires singleton + full-confidence pass. Everything else escalates or waits. This is the discipline verbatim.

**Verdict:** `@kintsugi/consent` IS the tripartition. It does NOT need to be lifted to family-root altitude, but it DOES need to be **consumed by every @io-facing surface**. Rung 7 currently bypasses it (see Task 4).

### 2.2 @glass.verdict + @dissonance.is_pareto + @cadence.is_settled — the substrate FLOOR

- `@glass.verdict = pass | partial(confidence) | failure(reason)` — the three-state floor Asher's Orchard preserves as HOLD / REVIEW / DISMISS (Asher p. 15).
- `is_pareto(candidates) -> verdict` — evidential-witness discriminator (per-set).
- `is_settled(c: cadence_kind) -> verdict` — evidential-witness discriminator (per-trajectory).

These three ARE the substrate-decl'd primitives @kintsugi/consent composes over. **They are witnesses, not gates.** Substrate-honest reading.

### 2.3 @mirror/store trichotomy + @mirror/store/git.commit_as_fold

`shards/mirror/store.mirror` names the three-layer trichotomy verbatim (§Trichotomy at OID altitude):

- `splinter` (leaf) — git's blob analog; substrate's evidence-atom.
- `splinter_graph` (composite) — git's tree analog; substrate's evidence-composition.
- `crystal` (settled root) — git's commit analog; substrate's authority-record.

**Asher-mapping:** splinter = witness-atom; splinter_graph = witness-composition (multi-axial evidence composed non-redundantly by OID); crystal = **authority-of-record**. `commit_as_fold` at `shards/kintsugi/store/git.mirror` IS the governance action: witnesses + gates fold into an authority-of-record. **This IS the tripartition at @store altitude.**

**Non-redundance:** OID-graph edges are content-addressed; two edges from the same content are the SAME edge. Asher's "five gauges connected to the same pipe" impossibility is content-addressing at substrate altitude.

**Verdict:** @mirror/store trichotomy IS the tripartition at @store altitude, complementary to @kintsugi/consent's at auto-apply altitude. Same shape, different altitude. This IS the self-similar / multi-axial / recursive-membrane structure Alex named as `@fractal`.

### 2.4 @mirror/mosaic.settle — governance authority at the algebra floor

`shards/mirror/mosaic.mirror` names `settle` as the fifth of the five-op algebra (the ONE write; measurement collapse). Per docs/specs `settle` = **governance authority**: the ONE action that changes system state. Recognitions #55 (form/process partition) + #43 (content-addressed build system) already name this.

### 2.5 @cyberpunk (Recognition #80 / #107) — gauge-bounded vs Turing-unbounded

Recognition #80 (`docs/math/the-tower/recognition-80-magic-as-form-process-substrate-decl.md`, LANDED 2026-07-06) + Recognition #107 (`@io` family-root, LANDED 2026-07-08) name the tripartition at the **substrate boundary**:

- **@magic** — gauge-bounded (mathematical guarantees hold; @fate optical inference; Rayleigh descent).
- **@io** — Turing-unbounded (the wire; boundary Rust; @io family-root).
- **@cyberpunk** — the reframe operator that mediates the crossing (`@cyberpunk/reframe.reframe(peer, level_K, pain_δ)`).

**Asher-mapping:** @magic surface = witness-domain (evidence lives here); @io surface = gate-domain (permission to cross the wire); @cyberpunk = authority-domain (level-shift ceremony; the pain-authorized decision to promote or demote).

### 2.6 @song/narrative.psychohistory_sheaf (Mara `2c26537`)

`shards/song/narrative.mirror` declares `psychohistory_sheaf` as the Rayleigh-descent-bounded sheaf. Per Mara `f2c712e` iter-34 + `ce301cc` iter-35 §Task-1 `cast_shadow` is Level 3 Transport (shadow-cast diagnostic). This composes cleanly into the tripartition: `psychohistory_sheaf` IS the witness-composition; `cast_shadow` IS the gate-application (each shadow evaluates a candidate direction); `bounded_by(sheaf, perturbation) -> tournament_result` IS the authority-action.

### 2.7 @fate/tournament.bounded_by (Mara `ce9745f`)

`shards/fate/tournament.mirror:942`:

```
bounded_by(sheaf: psychohistory_sheaf, p: perturbation) -> tournament_result { \ }
requires bounded_by_respects_holonomy(sheaf, result) — Level 3 Transport
requires bounded_by_below_lawvere_depth(sheaf, result) — Level 4 Closure
```

**Asher-mapping:** `sheaf` = evidential-witness composition; `perturbation` = the local neighborhood (gate scope); `bounded_by_respects_holonomy` + `bounded_by_below_lawvere_depth` = constitutional gates (permission bounds); `tournament_result` = authority-of-record (the winner).

### 2.8 @optics/lens/features (Mara `f3af5b4`)

Features are the get-direction observation surface. Per `docs/specs/flags-as-lens-applications-on-mirror-peer-beam.md` — the flag composition IS a lens composition. **@optics/lens/features IS the evidential-witness observation surface at CLI altitude.**

### 2.9 "fractal" / "self-similar" / "multi-axial" / "recursive-membrane" references

Grepped 45 files (42 shards + 3 doc-clusters). Findings:

- **"fractal"**: zero hits at family-root altitude. Grammar-parametric altitude only (`fragmentation` = the pattern; `Fractal::Shard` is a Rust type name predating the shard/splinter recognition). **Not load-bearing today.** Not in any species declaration. No `@fractal` family-root exists.
- **"self-similar"**: hits at `docs/math/the-tower/*` describing Bateson learning III + recursion tower. Load-bearing at math altitude; not a substrate-decl.
- **"multi-axial"**: zero hits in shards. Two hits in docs (`docs/specs/recognitions/recognition-92-neutrosophic-three-axis-substrate.md`) referencing the 3-axis truth/indeterminacy/falsity carrier (Neutrosophic logic per Smarandache). Adjacent to Asher's multi-axial escalation but not the same construct.
- **"recursive-membrane"**: zero hits.
- **"jurisdictional-separation" / "promote reluctantly"**: zero hits in substrate. Load-bearing terminology from Asher paper; not yet substrate-lifted.

**Interpretation:** the "fractal" reading Alex is naming IS the recursion of the tripartition across altitudes (already carried by @kintsugi/consent + @mirror/store trichotomy + @cyberpunk + @fate/tournament.bounded_by + @song/narrative.psychohistory_sheaf + @optics/lens/features + @mirror/mosaic.settle). Every altitude has evidence-witnesses, constitutional-gates, and governance-authority. That IS "fractal" in the self-similar sense.

### 2.10 Verdict — does @fractal need to be a family-root?

**NO. Do not mint `@fractal`.**

Reasoning:
1. **Substrate already had the word × 7.** The tripartition is landed at seven altitudes (consent, glass/dissonance/cadence, mirror/store, mirror/mosaic, cyberpunk, fate/tournament.bounded_by, song/narrative.psychohistory_sheaf, optics/lens/features).
2. **Family-root inflation.** Minting `@fractal` would create an ancestor for every prism, since every prism recursively carries the five ops. Per Recognition #40 (autopoiesis) + Recognition #55 (form/process partition), the substrate's recursion is already carried by the family-root inheritance graph.
3. **The right lift is CITATIONAL, not new-mint.** The @io-facing surfaces (Task 3 below) should **cite** `@kintsugi/consent.query_phi` at the auto-apply boundary. That's the "formalize into a surface which composes all @io facing layers" Alex named — but the surface EXISTS.
4. **Two-tick discipline:** `@kintsugi/consent` is already the readable name; `@fractal` would be the foundational name. Per substrate-already-had-the-word, the readable name wins.

**What TO do instead of minting:**

- **Lift @kintsugi/consent citations at every @io crossing.** Task 3 enumerates the crossings. Each one currently bypasses `query_phi`; each should route through it as its auto-apply boundary.
- **Document the tripartition-across-altitudes recursion** in a docs/math extension of `docs/math/the-tower/`. The `promote reluctantly / demote readily` asymmetry is the substrate's self-similar discipline at every altitude. Not a family-root; a math-altitude cascade doc.

---

## §Task 3 — @io-facing crossings enumeration

### 3.1 Substrate-decl @io references (shards)

Grep of `@io/(git|fs|cargo|shell|process|file|network|http|nix)` across `shards/**/*.mirror`:

| Substrate reference | Home shard | Function |
|---|---|---|
| `@io` (family-root) | `shards/io.mirror` | Root prism, 22.8KB |
| `@io/git` | `shards/io/git.mirror` | Git-shell boundary contract, 24.1KB |
| `@io/cargo` | `shards/io/cargo.mirror` | Cargo boundary contract, 8.0KB |
| `@io/oci` | `shards/io/oci.mirror` | OCI/container boundary, 25.6KB |
| `@io/algebra` | `shards/io/algebra.mirror` | Algebra of boundary crossings, 40.8KB |
| `@io/fs` | (multiple references) | Not-yet-declared as own shard; consumed in `shards/mirror/store.mirror` |
| `@io/nix` | (references in `shards/spectral/*`) | Not-yet-declared as own shard |

**Landed contracts:** git, cargo, oci, algebra. **Not-yet-declared but referenced:** fs, nix, shell, process, network.

### 3.2 Rust altitude @io crossings — subprocess spawns (`Command::new(...)`)

Grepped all `bootstrap/src/*.rs`. Total: **13 `Command::new(...)` sites** across 7 files.

| Site | File:line | Program | Altitude | Substrate-honest home |
|---|---|---|---|---|
| C-1 | `bootstrap/src/lib.rs:365-374` (Ctx::command) | (variable) | wrapper — carries `current_dir(ctx.cwd())` | Substrate wrapper; not itself a crossing |
| C-2 | `bootstrap/src/lib.rs:~3779` `init_enumerate_git_ls_files` | `git ls-files` | @io/git | LANDED |
| C-3 | `bootstrap/src/lib.rs:~3979` `git_last_commit_for` | `git -C ... log ...` | @io/git | LANDED |
| C-4 | `bootstrap/src/lib.rs:~4007` `recall_pack_trail` | `git -C ... log ...` | @io/git | LANDED |
| C-5 | `bootstrap/src/lib.rs:~4169` `git_first_commit_for` | `git -C ... log ...` | @io/git | LANDED |
| C-6 | `bootstrap/src/lib.rs:~4210` (recall spec_oid) | `git -C ... hash-object ...` | @io/git | LANDED |
| C-7 | `bootstrap/src/lib.rs:~4224` (recall most_recent_landed_at) | `git -C ... log ...` | @io/git | LANDED |
| C-8 | `bootstrap/src/git.rs:~8` `exec_capture` | (variable) | wrapper | Substrate wrapper |
| C-9 | `bootstrap/src/git.rs:~50` (update-ref crystal store) | `git update-ref` | @io/git | LANDED |
| C-10..C-15 | `bootstrap/src/store_branch.rs` (6 sites) | `git init` / `git hash-object` / `git mktree` / `git rev-parse` / `git commit-tree` / `git update-ref` | @io/git | LANDED (Rung 6.1c) |
| C-16..C-22 | `bootstrap/src/contribute.rs` (7 sites) | `cargo check` (1) + `git init` / `git hash-object` (3×) / `git mktree` / `git rev-parse` / `git commit-tree` / `git update-ref` (6) | @io/cargo (1) + @io/git (6) | LANDED (Rung 7) |

**Total:** 20 explicit substrate crossings (13 unique Command::new call chains). All to `git` and `cargo`. Both have landed substrate contracts.

### 3.3 Rust altitude @io crossings — filesystem (`std::fs::*`)

Grepped `fs::read | fs::write | fs::create_dir | fs::remove | fs::read_to_string | fs::read_dir` across `bootstrap/src/*.rs`. Total: **~55 sites** across 10 files.

Distribution:

| File | Count | Predominant purpose |
|---|---|---|
| `bootstrap/src/lib.rs` | 24 | Recall / peer-beam / spec-read |
| `bootstrap/src/spectral.rs` | 3 | Spectral I/O |
| `bootstrap/src/lens_unix.rs` | 6 | Unix lens crossings |
| `bootstrap/src/action_cache.rs` | 9 | Cache read/write |
| `bootstrap/src/contribute.rs` | 3 | Rung 7 target shard read/write |
| `bootstrap/src/dance.rs` | 1 | Song file read |
| `bootstrap/src/deploy.rs` | 2 | Song + spec read |
| `bootstrap/src/song.rs` | 1 | Song source read |
| `bootstrap/src/git.rs` | 1 | Git object write helper |
| `bootstrap/src/mcp.rs` | 1 | MCP file read |

**Altitude:** all `@io/fs` — the not-yet-declared @io species. **This is a substrate gap.**

### 3.4 Gate/witness/authority discipline audit per @io crossing

For each @io crossing category, does it factor through the tripartition?

| Category | Crossings | Gate? | Witness? | Authority? | Substrate-honest? |
|---|---|---|---|---|---|
| **@io/git write** (blob / tree / commit / update-ref) | 8 | ❌ no auto-apply gate (`query_phi` not consulted) | ❌ no per-crossing witness accumulator | ✅ `commit_as_fold` at Rung 6.1c is authority-of-record | Partial: authority-side landed, gate + witness bypassed |
| **@io/git read** (rev-parse / log / hash-object / ls-files) | 8 | N/A (read-only crossing) | ✅ result IS the observation | N/A | Substrate-honest as read |
| **@io/cargo** (cargo check) | 1 | ❌ no auto-apply gate | ✅ stdout/stderr IS the witness (Rung 7 blob) | ❌ NO governance surface — Rust match-arm decides settle vs revert directly | Bare @io — no consent surface |
| **@io/fs read** (spec / mission / song / target / peer_home) | ~30 | ❌ no gate | ✅ bytes are the observation | ❌ authority buried in Rust logic | Bare @io |
| **@io/fs write** (target_shard write / target_shard revert / action_cache write / .bauchladen moment write) | ~10 | ❌ no gate (Rung 7 writes then compiles then reverts on failure — reverse of promote-reluctantly discipline) | Partial (Rung 7 attempts a compile-witness after the write) | ❌ authority buried in Rust logic | **Anti-substrate: write-first-revert-on-failure IS demote-readily-promote-freely, the exact opposite of Asher's discipline** |

### 3.5 Gap report — the substrate-honest promote-reluctantly write pattern

Rung 7's pattern at `contribute.rs::peer_contribute` (lines 87-166):

```
1. write post_bytes to target_shard              ← promotion happens FIRST
2. cargo check                                    ← gate applied AFTER
3. if imperfect: revert target_shard to pre_bytes ← demote-readily via undo
```

This is `promote freely, demote-readily`. Asher's discipline is `promote reluctantly, demote readily`. The substrate-honest form:

```
1. compute post_bytes in memory
2. simulate compile in a scratch workspace (@io/fs gate)
3. query_phi(candidate_morphisms) — auto-apply boundary
4. IF verdict = pass: commit_as_fold to peer's DAG (the ONE write)
   IF verdict = partial(confidence): pause_event + emit_to_metalogue
   IF verdict = failure: refuse; log
5. target_shard write happens ONLY on the pass path, after query_phi
```

The current implementation lets the peer's morphism reshape the observer (the peer_home) before the constitutional gate fires. Per Asher p. 11 verbatim: *"A false admission may reshape the observer that must later correct it. That asymmetry is why the membrane is conservative."*

**This IS Alex's `@fractal surface which composes all @io facing layers`:** every @io write should go through `query_phi` first. That's not a new family-root; that's a **discipline of citing @kintsugi/consent at every @io crossing**.

---

## §Task 4 — Rung 7 5-blob tree jurisdiction audit

The 5-blob tree at `bootstrap/src/contribute.rs::materialize_morphism:333-340`:

```
100644 blob {pre_blob}     pre-anchor
100644 blob {post_blob}    post-anchor
100644 blob {morphism_blob} morphism-body
100644 blob {verdict_blob}  settle-verdict
100644 blob {fate_blob}     fate-witness
```

### 4.1 Asher-tripartition categorization

Per Asher pp. 8-9:

| Blob | Content | Asher role | Correct subtree |
|---|---|---|---|
| **pre-anchor** | target_shard bytes BEFORE morphism | **evidential witness** (temporal persistence: the pre-state was observed) | `witnesses/` |
| **post-anchor** | target_shard bytes AFTER morphism | **evidential witness** (temporal transition: the post-state after the morphism) | `witnesses/` |
| **morphism-body** | the docstring-append line | **evidential witness** (representational mismatch: the delta being proposed) | `witnesses/` |
| **settle-verdict** | cargo check stdout/stderr | **CONSTITUTIONAL GATE** (alignment admissibility: does the peer compile?) NOT a witness | `gates/` |
| **fate-witness** | fate_model + prism_op + peer_uuid | **governance authority** (provenance-of-decision: which subsystem selected this morphism) NOT a witness | `authority/` |

### 4.2 Jurisdictional-separation violations

Per Asher's load-bearing sentence "*None automatically converts into another*":

1. **`settle-verdict` is named "witness" in the code comment** (`contribute.rs:22` "settle-verdict + fate-witness blobs are substrate-honest provenance"). But **cargo check IS a gate, not a witness**. The verdict permits or prohibits admission; it does not add evidential support for the morphism's meaningfulness. Per Asher p. 8: *"The mechanism had allowed a constitutional gate to cast an evidential vote. That produced the key distinction: Provenance is necessary for trust, but it is not evidence of emergence."* Rung 7 has this exact bug: the compile-verdict is being treated as evidence of the morphism's substrate-fit, when it is only permission to admit.

2. **`fate-witness` is named "witness" but IS authority.** The fate_model + prism_op is *which subsystem selected this morphism*. Per Asher p. 9: *"Governance authority — This determines which subsystem may act."* The fate decision IS the authority-of-record; it's not evidence of the morphism's meaningfulness. Same jurisdictional-blur as settle-verdict.

3. **`pre-anchor` and `post-anchor` are both witnesses BUT redundant.** Per Asher p. 11: *"Five gauges connected to the same pipe do not constitute five independent confirmations."* Pre + post + morphism-body is the same information three ways: `morphism-body = diff(pre-anchor, post-anchor)`. This is redundant witness accumulation.

4. **Missing evidential witnesses.** Asher's five witness axes (p. 8-9):
   - temporal persistence — ❌ NOT witnessed (would need @autopoietic.tick observation across multiple ticks)
   - geometric coherence — ❌ NOT witnessed (would need sheaf-Laplacian λ₀(Δ_F) measurement of the peer's state graph)
   - contextual recurrence — ❌ NOT witnessed (would need cross-peer or cross-song observation)
   - perturbational stability — ❌ NOT witnessed (would need cast_shadow at Level 3 Transport per Mara `ce301cc`)
   - representational mismatch — partial (morphism-body IS the delta, which is one witness of representational-fit-attempt)

   **Only ONE of the five Asher witness axes is currently landed.** Rung 7's "5-blob tree" is misnamed w.r.t. Asher's five-witness canon.

5. **Missing consent-verdict blob.** `query_phi(morphism_set) -> verdict` was never called. The auto-apply boundary is implicit in the Rust `match settle_verdict` at line 101-166. Per §2.1 audit, `@kintsugi/consent` IS the substrate-decl'd auto-apply gate; Rung 7 bypasses it entirely.

### 4.3 Substrate-honest refactor — tripartition subtree structure

Per Asher pp. 8-9 + `@kintsugi/consent` tripartition audit:

```
witnesses/                    ← evidential; per §2 pp. 8-9
  temporal_persistence         (∈ splinter_graph edges to prior ticks)
  geometric_coherence          (λ₀(Δ_F) on peer state graph)
  contextual_recurrence        (cross-song / cross-peer witness)
  perturbational_stability     (cast_shadow(sheaf, direction, p) per Mara ce301cc)
  representational_mismatch    (morphism-body; the delta as evidence-of-fit-attempt)
gates/                         ← constitutional; per §2 pp. 8-9
  settle_verdict               (cargo check — alignment admissibility gate)
  glass_wall_crossing_count    (@io shrinkage witness per @kintsugi/cross_wall)
  provenance_integrity         (peer_uuid content-addressed authorship)
authority/                     ← governance; per §2 pp. 8-9
  fate_witness                 (fate_model + prism_op — WHICH subsystem selected)
  consent_verdict              (query_phi(morphism_set) — should_auto_apply / should_escalate output)
base/                          ← Asher "Base Fabric" — raw unresolved state per p. 14
  pre_anchor                   (target bytes at t-1)
  post_anchor                  (target bytes at t)
```

**Wire-level shape:** the 5-blob flat tree → a 4-subtree tree (`witnesses/`, `gates/`, `authority/`, `base/`). Git-native (subtrees are trees within trees, first-class in commit-tree). Auditor can query `witnesses/` alone for pattern-hood evidence; `gates/` alone for permission-record; `authority/` alone for governance-record.

**Missing blob role identified:** `authority/consent_verdict` — the query_phi output. This is the substrate-decl gate Rung 7 needs to route through. Without it, the auto-apply boundary is Rust-code-implicit; with it, the boundary is substrate-fact.

### 4.4 Extension — the Rung 7 rebuild that composes @kintsugi/consent

Per §3.5 + §4.3: the substrate-honest Rung 7 discharges through `@kintsugi/consent.query_phi` before writing the morphism. Sequence:

1. Compute `morphism_set` = [candidate morphism from Fate::bounded(psychohistory_root)].
   *(Bounded not excited — per §1.2 correction.)*
2. Build the 4-subtree candidate tree in memory (witnesses / gates / authority / base — none written yet).
3. Query `query_phi(morphism_set) -> verdict`.
4. Branch:
   - `pass` → `commit_as_fold` writes the tree; target_shard bytes written; peer's DAG advances.
   - `partial(confidence)` → `emit_to_metalogue(pause_event)` — surfaces to the metalogue session; target unchanged; peer's DAG unchanged.
   - `failure(reason)` → refuse. Log. target unchanged.

This IS the "compose all @io facing layers" Alex named. Every @io write becomes: **candidate → witness/gate/authority tree → query_phi → verdict → conditional write**. Not a new family-root; a discipline of citation.

---

## §5 — Ancestry (Asher paper) — verbatim citations

Per the load-bearing sentence guidance, three verbatim citations from Asher pp. 8-11 that this scout treats as substrate-decl:

> "Evidence may support. Gates may permit. Authority may act. None automatically converts into another." — p. 10

> "The mechanism had allowed a constitutional gate to cast an evidential vote. That produced the key distinction: Provenance is necessary for trust, but it is not evidence of emergence." — p. 8

> "A missed early pattern may be recovered later through recurrence. A false admission may reshape the observer that must later correct it. That asymmetry is why the membrane is conservative." — p. 11

The tripartition rides Asher's `promote reluctantly / demote readily` asymmetry. That asymmetry IS the substrate's discipline at every altitude Task 2 audited. Every existing carrier holds it; nothing needs to be minted; the drift is in the @io-facing Rust code (Task 3 + Task 4), not the substrate.

---

## §6 — Recognition candidates surfaced (not promoted; Alex-adjudication)

Two candidates surface from this scout. Both HELD for Alex-numeric-ID assignment per queue discipline.

- **`asher-tripartition-IS-the-substrate-discipline-at-every-altitude`** — @kintsugi/consent + @mirror/store trichotomy + @cyberpunk (Recognition #80 / #107) + @fate/tournament.bounded_by + @song/narrative.psychohistory_sheaf + @mirror/mosaic.settle + @optics/lens/features form a seven-altitude recursion of the same evidence/gate/authority triple. Not a new family-root; a recognition of the substrate's self-similar structure.
- **`Fate::bounded-IS-the-substrate-honest-default-at-every-named-peer-site`** — anonymous peers use `Fate::excited()` (no history to bound against); named peers should use `Fate::bounded(psychohistory_root)`. Rung 7 is the first empirical witness of the drift.

---

## §7 — Immediate corrections (grep-first; substrate-honest)

Reed-lane (one commit each; not blocking; Alex-approve-then-merge):

1. **Rung 7 substrate-honest correction.** Swap `Fate::excited()` at `bootstrap/src/contribute.rs:63` for the bounded pattern via `psychohistory_root_from_peer_home(peer_home)` + `selectors_from_psychohistory_root(root)`. One-file diff; ~8 lines.
2. **5-blob → 4-subtree tree refactor.** Regroup `materialize_morphism` output into `witnesses/`, `gates/`, `authority/`, `base/` subtrees per §4.3. Wire-level: same 5 blobs, 4 subtrees. Auditor query surface improves; substrate-honest naming.
3. **Route Rung 7 through `@kintsugi/consent.query_phi`.** Add the substrate-decl'd auto-apply boundary check per §4.4 sequence. Requires `query_phi` Rust-altitude stub OR forward-promise if runtime lands post-#386.
4. **Mint the substrate-decl `Fate::bounded(cfg)` in fate crate.** Single-symbol constructor over `BoundedConfig { weights, holonomy_ceiling, depth_cap, ... }` per Mara iter-30 §5. Collapses 3 call sites to `let fate_engine = fate::Fate::bounded(BoundedConfig::from_psychohistory_root(&root));`.
5. **@io/fs substrate-decl mint.** Currently ~55 `std::fs::*` crossings in bootstrap have no substrate-decl home. `shards/io/fs.mirror` sister to `shards/io/{git,cargo,oci,algebra}.mirror`. Low-priority substrate hygiene.

Docs-lane:
6. **Cascade doc `docs/math/the-tower/asher-tripartition-recursion.md`.** Document the seven-altitude self-similar tripartition per §2 audit. Ancestry: Asher 2026-07-10 + Recognition #55 (form/process) + Recognition #80 (@magic) + Recognition #107 (@io family-root). Not a substrate-decl; a math-altitude cascade doc.

---

## §8 — Non-recommendations (substrate-already-had-the-word refusals)

Do NOT:

- Mint `@fractal` family-root. Substrate already carries the self-similar tripartition at seven altitudes per §2.
- Add "witness" / "gate" / "authority" as separate family-roots. @kintsugi/consent IS the composed surface at auto-apply altitude; @mirror/store trichotomy IS it at store altitude.
- Extend `@epistemologic/cybernetic/*` with a 15th species for the tripartition. The three-axis structure IS in glass.verdict already; the 13 cybernetic species already cite it via `imperfect<verdict, violation, transparency>`.
- Mint `Fate::promoted_reluctantly` or similar Rust methods. The `promote reluctantly / demote readily` discipline is per-call-site; the constructor stays `bounded(cfg)`; the discipline is the call-site's routing through `query_phi`.

---

## §9 — Traceability

- `bootstrap/src/lib.rs:4375-4453` — `selectors_from_psychohistory_root` — v1 stub, LANDED.
- `bootstrap/src/lib.rs:4681-4759` — `fate_bounded_by_psychohistory_peer_beam` — the ONE landed bounded call site.
- `bootstrap/src/lib.rs:4544-4655` — `fate_bounded_shadow_peer_beam` — landed bounded + cast_shadow composition.
- `bootstrap/src/lib.rs:4763-5069` — `fate_select_peer_beam` — anonymous case (excited substrate-honest).
- `bootstrap/src/contribute.rs:63` — Rung 7 excited call site (substrate-honestly wrong; Task 1 correction).
- `bootstrap/src/contribute.rs:246-442` — `materialize_morphism` 5-blob tree (Task 4 refactor target).
- `shards/kintsugi/consent.mirror` — tripartition substrate surface (Task 2 §2.1).
- `shards/mirror/store.mirror` — trichotomy at store altitude (Task 2 §2.3).
- `shards/fate/tournament.mirror:942-995` — `bounded_by` substrate-decl (Task 1 §1.1).
- `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` — Mara iter-30 canonical (Task 1 §1.4).
- `/Users/reed/dev/systemic.engineering/blog/_src/kimberley-asher_meaning-is-not-a-metric.pdf` pp. 8-11 — Asher tripartition load-bearing citations.

---

*Taut scout complete. Read-only. No substrate edits. No family-root mints proposed. All three verdicts point at citation-and-routing corrections, not new-mint.*
