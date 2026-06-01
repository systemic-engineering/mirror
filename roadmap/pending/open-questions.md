# Open Questions

The live ones that need design decisions before they unblock.

## Q1: Should the Rust AST be regenerated from `@mirror/ast`?

The Rust unification landed in the 2026-05-08/09 compiler collapse — one `AstKind` enum + `AstNode` struct in `bootstrap/src/ast.rs`. But `01-meta.mirror` declares a parameterized AST (`ast(g)`, `expression(g)`, `declaration(g)`, `pattern(g)`, `type_ref(g)`) that's a different shape. The real open question: does the Rust `AstNode` converge to the parameterized `ast(g)` shape **by being regenerated from the `.mirror` grammar via `@code/rust`**, or does it stay hand-written and the `.mirror` grammar adapts to it?

Mirror's overall direction (Phase 4's fragmentation-as-generated demonstration) points at the first answer. **Recommend: this becomes Phase 4 R-0-bis** — a smaller-than-fragmentation proof of `@code/rust`, ordered before R-2's full `@fragmentation.mirror` grammar.

## Q2: Self-teaching parser bootstrap

Minimum-viable-keywords is 23. The Tier 1 set can't shrink below that because each keyword is used by at least one kernel file. The question is exactly how the parser learns Tier 2 keywords from boot-declared grammars without re-entering an infinite bootstrap loop.

## Q3: What stays in Rust permanently?

The `@io` boundary is clear in shape. The question is exactly which `.rs` files stay (LAPACK FFI, Metal/OpenCL dispatch, syscall wrappers, hash primitives at the git adapter boundary) and which migrate to `.mirror` via grammar rewrites.

## Q4: Compilation target of self-hosted mirror

Today `mirror compile` produces `CompiledShatter` (in-memory) + shatter file on disk. Phase 7 might add: native binaries via `mirror craft --target binary`; WASM via `--target wasm`; OpenCL kernels via `--target opencl`; spectral-db replicas via `--target spectral-db`. Which targets ship at v1.0.

## Q5: Cycle handling in the DAG VCS substrate

Fragmentation v1 is DAG-native (multi-parent acyclic). Cycle handling is deferred to spectral-db (per the mirror-native-vcs spec). The question is what spectral-db's exact cycle-handling semantics look like — fixed-point iteration on Merkle hashes; cycle-breaking via canonical-order; cycle-as-explicit-edge-type. To be specced in spectral-db's own corpus.

## Q6: Hash representation for cross-platform stability

`SpectralCoordinate<5>` is 5 × IEEE-754 `f64` = 40 bytes, 48-bit rounded for cross-platform byte stability. The rounding scheme is chosen pragmatically; needs validation against actual byte-output drift between LAPACK builds (OpenBLAS vs. Accelerate vs. Apple's vecLib). Acceptance criterion: same input → same bytes across all v1.0 deployment targets.

## Q7: Scheduler Tower's temperature `β` — user-tunable or auto-adapted?

The Scheduler Tower spec lands `β = 1.0` as default and doesn't expose tuning at v1.0. Post-v1.0, the gestalt can record per-workload optimal `β` and the runtime auto-adapts. Whether that auto-adaptation ships at v1.0 or at v1.1+ is open.

## Q8: Broadway batching as a placeholder spec

Mara's research recommends waiting on Broadway-style batching until a real bulk-write workload surfaces (likely spectral-db's distributed paths). The question is whether to draft a placeholder spec now (so future contributors don't re-derive Broadway's design) or wait. Recommended: wait.

## Q9: License model for `@spectral/db`

The closed graph engine ships under a commercial license. Per-deployment? Per-org? Per-shard? Per-node? Needs to land before v1.0. Considerations: the `|\>` operator produces per-shard binaries from the same source AST, which means "per-binary" and "per-deployment" can diverge; the license needs to name which it counts.

## Q10: Public contract for the `@spectral/db` adapter boundary

The closed binary speaks to the open adapters (`mnesia`, `sql/postgres`, `sql/lite`) over a defined protocol. That protocol is the public contract: versioning and stability matter here specifically. Third-party adapters (`dynamo`, `redis`, `sqlserver`) need this contract published before they can be written. Open: where the contract lives (a `.mirror` grammar file? a versioned spec doc? both?) and what the v1.0 commitment is.

## Q11: License model for `@spectral/garden`

Per-package: each curator chooses (Apache-2.0 / commercial / mixed). The substrate verifies signatures regardless of license; ed25519 + content-addressing closes tampering by construction. Open: which gardens ship at v1.0 launch, what minimum reviewer-credential discipline counts, and whether spectral.engineer hosts a canonical default-garden or stays neutral substrate.

## Q12: Trigger conditions for LRM-deferred substrate

Three pieces are captured-but-deferred per the Last Responsible Moment: `@kintsugi/cross_wall` (#80), `@epistemologic/reality` (Track G), `@spectral/garden` (Track H). Each has a natural demand signal that should trigger implementation. Document the triggers so the next session knows when to pull from capture:

- **`cross_wall`** triggers when an `@io` grammar's halts becomes provable and a user-or-substrate asks to pull it. Likely first consumer: the fragmentation Rust crate as Phase 4 R-tick lands.
- **`@epistemologic/reality`** triggers when per-peer lens authoring surfaces (Phase 7 onboarding; the garden's reviewer-lens chain; per-peer eigenboard customization).
- **`@spectral/garden`** triggers when Phase 7 onboarding needs a concrete content source — the DGSF/ICF/practitioner corpus has nowhere else to live structurally.

## Q13: The next-altitude recognition

The 2026-05-25/26 session crossed multiple "the substrate knew" moments — gen_prism IS MCP; @peer = Prism(self); shard = observer-relative λ₀; portal = `@io.socket` + content-addressed subspace; glass_wall as inverted halts; spectral triple as heuristic composition; lens as constructivism made structural; garden as vetted-corpus distribution. The pattern itself suggests there's another altitude waiting to be recognized. Candidates: the relational topology of multi-peer composition (the *cluster as organism*, not as N independent peers); the substrate's gestalt-of-gestalts at the garden+peer composition layer; the meta-curator (what verifies the verifiers when multiple gardens disagree); or something not yet named.

## Q14: Sub-Turing substrate / Turing-complete emergent system altitude split

Resolved 2026-05-26 per `docs/insights/2026-05-26-mirror-sub-turing-substrate-with-emergent-turing-completeness.md`. Mirror itself is sub-Turing (compiler, type system, halting decidable). The system that emerges from mirror (autonomous @fate agents modifying mirror code + humans-in-the-loop via @scene verifying behavioral changes) is Turing-complete. Two altitudes; no contradiction. The Lachmann-Sella 1995 result (ant colonies are Turing-complete) lives at the system altitude; mirror's substrate floor stays sub-Turing.

## Q15: Tournament-level Lyapunov convergence

Resolved 2026-05-26 per gap-tension-tensor spec §10. The convergence proof's unit of monotone-decrease is the tournament round, not the individual fracture. Knapsack-style relaxation: greedy single-fracture-Lyapunov is too restrictive; tournament-selected composition with backtracking gives global improvement even when individual fractures locally worsen the norm. Cited: Hajek 1988 (simulated annealing per-step ascent under macro descent); FPTAS knapsack approximation algorithms; A* admissibility (Hart-Nilsson-Raphael 1968); B&B as PTAS (Hendrich-Pferschy-Klotz 2025).

## Q16: `@mirror/store` layered framing

Resolved 2026-05-26 (after a framing iteration). `@fragmentation` stays pure substrate; `@mirror/store` owns IO. The four adapters live at `@mirror/store/{nix,git,bare,sqlite}`. Default is git (lowest entry bar). Iceberg adapters sit below: `@mirror/store/iceberg/{tape,glacier,filecoin,storj,...}`.
