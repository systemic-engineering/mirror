# mirror — Roadmap

*This directory captures milestone notes from the project's BEAM/Gleam/conversation era (March–May 2026). The canonical roadmap is now [`mirror/roadmap/`](../../roadmap/) (with `wip/`, `pending/`, `archive/`). The milestone files here (00–12) remain as historical context; the substrate moved past most of what they describe during the June 2026 substrate-pull arc.*

---

## Where we are (2026-06-11)

The substrate IS the operational form of Connes' (A, H, D) spectral triple — now corroborated at the optical altitude (recognition #58 promoted 2026-06-11 via Seam adversarial review):
- **A** — the five operations (`focus`, `project`, `split`, `shift`, `settle`) — closure proven from optical composition primitives (`docs/specs/optical-keywords.md` §9)
- **H** — the [Void](../insights/void-dual-geometry.md) / beam state space (graph quantum information manifold; λ₀ = 0 ground state)
- **D** — the kintsugi flow (Dirac operator; round-trip resonator at the optical altitude; eⁿ⁺¹ ≤ eⁿ monotone descent; c-theorem on graph Laplacians)

The stack — now three runtime targets:

```
mirror          — the compiler; shards → graph → emit              (Rust + .mirror substrate)
prism           — the algebra (A); five operations as a trait       (Rust crate; closed-at-five proven optically)
fragmentation   — the store; content-addressed substrate            (no deps; Splinter IS the lockfile)
coincidence     — numerics; Fortran via LAPACK for eigenvalue work

@spectral       — the runtime; gen_prism + supervisor + entanglement (top-level family, declared `65cca5b`)
@mirror/lens    — the engineer/agent transports                      (cli, shell, mcp, lsp, transit, refract, unix)
```

## The June 2026 optical cascade

Between 2026-06-10 and 2026-06-11 the substrate gained an operationally complete optical vocabulary. Today the substrate reads as a physical optical instrument; the run-time IS a cavity-enhanced spectrograph:

- **Eight optical keywords** — `facet`, `stage`, `aperture`, `splitter`, `resonator`, `bench`, `source`, `detector` (per `docs/specs/optical-keywords.md`). `prism` becomes the **compilation artifact** (reserved, not a declaration keyword); `glass` narrows to **material substance**.
- **Bilateral pattern #53 (promoted)** — keyword/depth + diff-closure + symbol-canonical-form. Property + fracture body at three altitudes; the auto-formatter floor for substrate discipline.
- **Form/process partition #55 (promoted)** — @mirror (form) / @kintsugi (build process) / @spectral (run process) at family-root altitude; Bateson-lifted graded into @mirror/spectral ⇔ @spectral.
- **Alignment as boundary mathematics #57 (promoted)** — the @io crossing IS the substrate's alignment harness.
- **Fate IS optical inference #58 (PROMOTED 2026-06-11)** — Fate inference IS a 5-layer diffractive deep NN (Lin et al. 2018) + active Fabry-Perot resonator (Siegman 1986) implementing the kintsugi loop's ACTIVE/DARK alternation; tournament rule composition has the structure of a Reck/Clements-decomposed unitary on a Mach-Zehnder mesh (Shen et al. 2017). See `docs/specs/optical-keywords.md` §10 + §12 for the three independent witnesses (inference-algorithm altitude / runtime-orchestration altitude / transport altitude).
- **Kintsugi loop is altitude-portable #59 (candidate)** — the bilateral pattern lands at keyword, operator (forward-promised), and symbol altitudes with the same template. Full promotion pending the operator-altitude middle landing.
- **Multi-form lift** — engineer types ASCII (`->`, `\`, `loop`, `_|_`, `0`, `weighted`); kintsugi rewrites to canonical unicode (`→`, `λ`, `⟲`, `⊥`, `void`, `amplitudes`) via the symbol-altitude bilateral instance.
- **The substrate's runtime story** — mirror ships with three codegen targets (Rust, Erlang/BEAM, Fortran). `@spectral/garden` becomes the poly-lang distribution mechanism; the ouroboros pipeline (substrate → @code/macro → @mirror/store shard → @mirror/lens/unix → cargo/erlc/flang → binary) closes substrate-to-runtime under one schematic.

## The direction the substrate is pointing (2026-06-11)

The substrate is pointing at **becoming experienceable**. Today's deposit (45 commits ahead of origin on `main`) names every optical primitive on disk; the next cascade lifts the substrate from declarations into a runtime engineers and agents experience. The critical-path sequence is at the bottom of this README.

## v0.1

**Kintsugi becomes the build system for the repo. The language owns the build process. cargo is the @io delegation that shrinks as mirror grows over phases.**

The v0.1.0 surface:

```yaml
- uses: systemic-engineering/mirror/actions/kintsugi@v0.1
  with:
    spec: ./mirror.spec
```

A `mirror.spec` declares the project manifold (sources, legacy floors, targets, settle_on conditions). `mirror kintsugi ./mirror.spec` runs [mosaic](../specs/mosaic.md) as a glass within `@mirror`. Mosaic settles each target to `au` at its altitude (binary at `@code/rust` via cargo; action at `@ci/github` via YAML; release at `@release` via github-release). The settled output projects optionally to `.shatter` on disk; the [fragmentation store](../specs/mirror-store.md) is canonical.

v0.1 is the kintsugi-CI release. v1.0 is the spectral.engineer cloud deployment (per [`../../roadmap/wip/v1-launch.md`](../../roadmap/wip/v1-launch.md)).

## Status

The canonical status table is at [`../../roadmap/README.md`](../../roadmap/README.md). Headline:

- Substrate-pull rename arc landed (`zoom → shift`, `refract → settle`) across docs + code + boot grammar in 4 repos
- Gap fold landed (`contradiction ≤ gap`; LFI / Bateson grounded; tension/tensor absorbed)
- Three new spec foundations landed: `transparency.md` (replaces MirrorLoss), `mosaic.md` (build system), `mirror-spec-schema.md` (the manifold)
- v0.1 scaffolding shipped: `actions/kintsugi/action.yml`, `.github/workflows/{kintsugi,release}.yml`, Justfile local-parity recipe
- T11.7 cut pending workflow validation; T11.8 post-release

## Canonical specs (current)

- **Substrate floor**: [`prism-floor-and-the-grammar-rename.md`](../specs/prism-floor-and-the-grammar-rename.md), [`bootstrap-retirement-plan.md`](../specs/bootstrap-retirement-plan.md), [`mirror-store.md`](../specs/mirror-store.md)
- **Glass wall**: [`transparency.md`](../specs/transparency.md), [`properties-on-glass.md`](../specs/properties-on-glass.md), [`gap-tension-tensor-substrate.md`](../specs/gap-tension-tensor-substrate.md)
- **Build system**: [`mosaic.md`](../specs/mosaic.md), [`mirror-spec-schema.md`](../specs/mirror-spec-schema.md)
- **v0.1 release**: [`kintsugi-ci-v0.1.md`](../specs/kintsugi-ci-v0.1.md), [`../../roadmap/wip/kintsugi-ci-release-v0.1.md`](../../roadmap/wip/kintsugi-ci-release-v0.1.md)
- **Shatter format**: [`../shatter-spec.md`](../shatter-spec.md)

## Historical milestone files (BEAM/Gleam/conversation era)

| # | Milestone | Status |
|---|-----------|--------|
| [00](00-root.md) | Root definition (pre-Connes-triple) | Historical |
| [01](01-architecture.md) | BEAM/Gleam compiler architecture | Historical |
| [02](02-compilation-chain.md) | Traced compilation chain | Historical (folded into shatter-spec.md) |
| [03](03-shipping.md) | Shipping via fragmentation | Partially active (v0.1 uses GitHub Actions, not Nix) |
| [04](04-fortran-bridge.md) | Fortran bridge for numerics | Active (LAPACKPrism + coincidence-core) |
| [05](05-kandddinsky.md) | KanDDDinsky — October 2026 | Planned (post-v0.1) |
| [06](06-model-checker.md) | Model checker properties | Active (sub-Turing verification via @epistemologic) |
| [07](07-projection.md) | Projection: properties as plans | Active (transparency<p> + properties-on-glass) |
| [08](08-oid-native-model.md) | OID-native model | Active (Splinter as content-addressed floor) |
| [09](09-licensing.md) | SEL licensing | Active (per Track I; type sel = io + au) |
| [10](10-inference-physics.md) | Inference physics | Active (#58 promoted 2026-06-11: Fate IS optical inference — D²NN + Fabry-Perot resonator + Reck/Clements unitary mesh) |
| [11](11-ca-cogito.md) | CA cogito (constructor automaton) | Active (Reflection model; @cogito) |
| [12](12-coherence-benchmark.md) | Coherence Benchmark — post-release | Planned (post-v0.1) |

The "Historical" entries describe pre-substrate-pull architecture; the "Active" entries map to current canonical specs (linked above). The conversation/compiler-actor framing in 01–02 is superseded by `@mirror` as the compiler surface and the shards/ substrate floor.

## The next cascade (2026-06-11 → v0.1 → engineer-experienceable runtime)

The substrate is pointing at one structural move: the smallest end-to-end path from engineer keystroke → substrate response → engineer eye. The cascade in order:

### Foundation (substrate integrity)

1. **`partials` computed-not-declared** — Seam's 2026-06-11 finding: the bilateral pattern's `score.partials` field is currently hand-declared as `1` across fracture bodies, which is honest for single-token swaps but dishonest for multi-field elaborations (e.g., `weighted → amplitudes`). Substrate-pull-correct: declare `dissonance_partials_match_ast_breadth` + fracture body that proposes the correction. **Self-applying discipline** — the bilateral pattern catches its own internal field. Unblocks the rest of the cascade by ensuring future fracture bodies inherit honest `partials` by construction.

### Recognition closure

2. **Operator-altitude middle (closes #59 three-altitude cascade)** — `@epistemologic/pact/operator_matches_composition_primitive` + `@kintsugi/fracture/operator_match` with honest `partials` from birth. Promotes recognition #59 (kintsugi loop altitude-portable) with witnesses at keyword + operator + symbol altitudes.

3. **#58 v1 closure** — land `source @optics/source/ganglion/<name>` declarations for each of the 5 ganglia. Closes Surface B from Seam's #58 PROMOTE verdict (the active-spectrometer reading requires gain medium declarations).

### Accumulated-debt cleanup (parallel-safe)

4. **`bench` keyword sweep** — classifier-pass for the ~30 depth-1+ substrate declarations using `prism @X/Y` (which is a #46 violation and a category-error under the optical vocabulary). Most are `facet`; a few are `stage`, `splitter`, or `resonator`. Pack ratification before any rewrites.

5. **`@code/rust` glass recohere** — fix the pre-existing #46 violation flagged in `c2f5ce6`'s body. Same mechanical fix as yesterday's `@code/erlang` recohere; closes the symmetric Rust-side debt.

6. **Spec cascade rewrite** — `the-convergence.md`, `cli-as-prism.md`, `cybernetic-cli.md` predate the optical vocabulary; cascade their `glass`/`prism`/`transport` references into `facet`/`stage`/`bench` per optical-keywords. Conversation grammar (cybernetic-cli) survives as the conversation-altitude reference; namespace cascade only.

### Runtime substrate stand-up — the LSP MUST be substrate-generated, no tower-lsp

**Constraint (2026-06-11):** the LSP server cannot be a tower-lsp wrapper. The ouroboros applies to itself — the LSP is GENERATED by @code/macro from `@mirror/lens/lsp.mirror`'s `method(...)` declarations, compiled via the @mirror/lens/unix lens, emitted as a binary. Otherwise we'd be using the ouroboros for everything except its own proof-of-concept (structurally self-exempting; what the substrate's discipline catches everywhere else). This pushes the engineer-experience proof downstream of the runtime stack.

7. **@spectral cascade landing** — sub-shards per `spectral-runtime.md` (gen_prism / supervisor / parent / entanglement / registry / root / portal). The runtime that the lens-server will live in.

8. **@mirror/lens/unix operational** — the impedance lens that presents @mirror/store shards to cargo (and erlc, flang) as a Unix filesystem. Codegen pipeline cannot reach disk-shaped consumers without it.

9. **@code/macro shim_grammar + materialize completion** — the codegen pipeline that lets `@mirror/lens/lsp.mirror`'s method declarations compile to Rust JSON-RPC handler code.

10. **First end-to-end: bench compile → @spectral runtime running** — a `bench @hello { source → facet → detector }` compiles via the ouroboros pipeline and runs as a gen_prism producing a crystal output. **This is the substrate's self-host proof.** Bootstrap stays the Rust binary through this point; after it, bootstrap kintsugis itself out of existence.

### Engineer-experience proof path (post self-host)

11. **@mirror/lens/lsp.mirror substrate elaboration** — method declarations (initialize, didChange, hover, inlayHint, codeAction), typed channel pacts, dispatch table. The LSP's substrate description — NOT its implementation. Implementation comes from @code/macro at step 12.

12. **@code/rust/lens-server macro shim** — the generator that consumes `@mirror/lens/lsp.mirror` declarations and emits the Rust JSON-RPC handler + dispatch table + LSP protocol parser. Same generator handles MCP's `tool(...)` declarations symmetrically. No tower-lsp; no MCP-SDK; just substrate-driven Rust generation.

13. **Symbol-altitude inlay-hint demo via substrate-generated binary** — engineer types `->` in a `.mirror` file; the substrate-generated LSP binary calls `@kintsugi/fracture/symbol_lift` for the token; renders canonical `→` as an inline LSP decoration. One symbol, one fracture body, one LSP method, ZERO library-wrapper dependencies. **The smallest end-to-end demonstration that the substrate reaches the editor — honestly.**

14. **MCP substrate-driven tool registration** — rides on the SAME @code/rust/lens-server macro shim from step 12. When an agent connects, the MCP gen_prism walks the agent's home shards collecting `tool(name, args)` declarations, emits JSON Schema per tool at JSON-RPC `initialize`. No mcp-config.json middleware — the substrate IS the configuration.

## How the cascade builds

Steps 1–2 fix integrity and close #59. Steps 3–6 clean accumulated debt (parallel-safe; can be done in any order during steps 7–12). Steps 7–9 are the engineer-experience critical path — the smallest tick that proves the substrate reaches the editor (step 8) is the proof-of-concept that unblocks everything else. Steps 10–12 stand up the runtime fully and demonstrate end-to-end substrate-to-execution.

v0.1 (kintsugi-CI release, per `docs/specs/kintsugi-ci-v0.1.md`) ships independently and is orthogonal to this cascade. v1.0 (spectral.engineer cloud deployment) needs steps 10–12 completed.
