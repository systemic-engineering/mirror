# Phase 0 — Current State

**Status:** HERE.

## What exists in Rust (the substrate)

The mirror crate is roughly 55 `.rs` source files. ~1,362 tests. 76% line coverage. Coverage gate enforced.

## What exists in `.mirror` (the boot grammars)

17 kernel boot grammars + 36 std library grammars. The boot sequence is documented in `boot/std/mirror/grammar.mirror`.

**Substrate-altitude keyword consumption (2026-05-26, `mara/shard-chain`).** `bootstrap/src/grammar.rs::load_grammar` now merges `<op> <keyword>` declarations from companion `.mirror` sources into the primary grammar's keyword table. First wiring: when the mirror grammar (`boot/std/mirror/grammar.mirror`) is loaded, the bootstrap also reads `boot/std/mirror/glass/ast/token.mirror`. The new vocabulary `glass`, `lambda`, `fixed`, `property`, `shape` (declared in `token.mirror` per the `@kintsugi/fracture/grammar-to-glass` surface) is recognized by `parse_grammar`. Both files coexist during the migration — additive, not replacement. Conflict path: same keyword + different op in primary vs companion = `io::Error` (stop-and-report).

Empirical evidence the consumption is real: re-tokenizing the boot tree post-extension produces different OIDs for 17 files (out of 160), and every changed file uses one or more of the new keywords. Files that don't use them are byte-identical pre/post. The OID delta IS the substrate-pull realization landing in observable behavior.

## Working CLI commands

`mirror compile`, `mirror craft`, `mirror kintsugi`, `mirror ai`, `mirror check`, `mirror ci`, `mirror eval`, `mirror lsp learn`, `mirror new`, `mirror spec`.

## Lambda phases

`Parse → Resolve → Properties → Emit` as a content-addressed, composable pipeline.

## Recently landed architectural work

- **F-1 — the real walker.** Combinator walker now consumes bytes structurally (per `docs/specs/walker-contract.md`). FP1 at the meta-glass level. (commits `b9118cb`, `67afbdb`, `80f4a8d`, `facc2fb`, `62b8650` on `reed/v1-floor`)
- **Beta tree normalization + charset compilation.** Two-phase confluent normalization (per `docs/specs/combinator-optimization.md`). 26 new tests.
- **`SpectralCoordinate<5>` rename and home move.** From `coincidence::CoincidenceHash<N>` to `fragmentation::SpectralCoordinate<N>`. Trait default is `Commit<N, H = SpectralCoordinate<5>>`.
- **Mirror-store spec.** Three-layer architecture (Rust substrate / loaded grammars / applications) with FP1 promoted to Layer 2.
- **Fragmentation as DAG VCS substrate.** Per `docs/specs/mirror-native-vcs.md`. Workspace layout with `vcs/git` and `vcs/jj` adapters; coincidence collapse plan.
- **NumericalPrism with backend abstraction.** Per `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`. Operation enum + backend trait + LapackBackend (today) + MetalBackend (Phase 6) + OpenCLBackend (Phase 6, cloud-required).
- **Kintsugi as Ricci flow.** New section in `docs/specs/kintsugi-formatter.md` naming the structural correspondence.
- **Scheduler Tower draft.** `docs/specs/scheduler-tower.md` with demand-contract extension to `gen_prism`, dispatcher strategies, KMS-shaped temperature.
- **Fragmentation-as-generated spec.** Per `docs/specs/fragmentation-as-generated.md`. Mirror generates `fragmentation`'s Rust from `@fragmentation + @code/rust`.
- **Seam adversarial audit.** `docs/audits/2026-05-22-seam-mirror-post-meta-glass.md`. 10 findings; F-3/F-4/F-5/F-8/F-9 fixed; F-1/F-2 are the load-bearing follow-ups.
- **2026-05-25/26 substrate insights landed** (13+ files in `docs/insights/`). The substrate-level recognitions that compound: gen_prism IS MCP; agent home as typed hole; pipe-hole-and-au-binary; shard as observer-relative λ₀; spectral namespace; parametric types as Prism-at-type-layer; @time substrate; GRAM-and-mirror same architecture; portal as `@io.socket` + content-addressed subspace; glass_wall and cross-wall kintsugi; heuristic termination; epistemologic/reality constructivism; spectral/garden as vetted-corpus distribution.
- **2026-05-26 spec stack on `mara/shard-chain`** (this session): `@kintsugi/fracture` substrate + first rule + boot-tree migration; gap-tension-tensor spec (1483 lines; formal math grounded in Domingos/LTN/Sheaf NN/Topping/Hajek/FPTAS/Tarski); kintsugi-as-credo-and-formatter-unified; @scene substrate + render-target deepening; @epistemologic/reality/frame scientific grounding; @fate as recursive multi-trajectory backtracking; kintsugi-blending research; spectral-db four-tier architecture (Nix cold + iceberg deep archive) + tombstones for visible deletion; sub-Turing substrate / Turing-complete emergent system resolution; tournament-level Lyapunov convergence; ants/colonies/stigmergy research; @code/nix render target; @mirror/store substrate placement correction (@fragmentation pure; @mirror/store owns IO).

## What's working

Compilation, content-addressing, property verification, code emission, shatter serialization, git integration, signing, licensing, package management, NL tokenization, query language, evaluation. Beta normalization confluence. Meta-glass FP1.

## What's NOT yet working (the honest gaps)

- The walker walks but the seed remains permissive (accepts balanced bytes). Structural FP1 at the loaded-grammar level (Layer 2) requires the Lift registry, which requires fragmentation as the store (per `docs/specs/mirror-store.md`).
- `tokenize.rs` and `grammar.rs` are still 100% Rust. Phase 2 retires them via parser self-description. (`grammar.rs` is now substrate-pull-realized for keyword harvesting — it reads keyword declarations from `.mirror` companion sources, not hardcoded tables. The dispatch logic itself is still Rust; Phase 2 retires that.)
- Two resolvers coexist. Phase 1 collapses to one. *(Status unverified post-collapse — needs re-audit before next Phase 1 spawn.)*
- `\` hole dispatch is declared but not implemented. Phase 5 lands it via Fate.
- The fragmentation Rust crate is hand-written. Phase 4 + Phase 6 collaborate to make it generated.
- No GPU acceleration anywhere. Phase 6 lands MetalBackend + OpenCLBackend.
- **`@mirror/serve` runtime dispatch not yet built.** Backpressure is declared structurally (halts + reduction_budget); a serve-loop that consumes the demand contract on the wire is the natural Phase 5 consumer.
- **Verified-construction `refract(T)` for pre-v1.0.** The structural-construction guarantee (sub-Turing source → verified generated code) needs a load-bearing end-to-end demonstration on at least one production target before v1.0. Phase 4's `@fragmentation + @code/rust` is the canonical first proof.
- **Six portals.md instances unimplemented.** The grammar primitive exists; the six concrete consumers (session, fs-mount, BEAM connection, cross-system, communication, identity) need typed re-implementation as `@spectral/portal` instances.
- **No live runtime for the substrate-altitude proposals.** Most of the 2026-05-26 spec stack is declarations + grounded math; nothing runs yet. The gap is operational, not theoretical.
