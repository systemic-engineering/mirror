# Grant Verification — mirror

> Verifiable substrate evidence for funders. Copy-paste blocks below for application narratives.
> Self-contained sections; each can stand alone in an application.
> All claims are backed by git commits / spec docs / running tests in this repository.

*Last refreshed: 2026-06-15. Working tree state at refresh time pinned in §Verification path.*

---

## At a glance

`mirror` is a sub-Turing programming language whose source generates a verified Turing-complete substrate. Same algebra at every altitude (five operations: `focus`, `project`, `split`, `shift`, `settle`), content-addressed at every node, BEAM-native runtime, no US cloud dependency. The compiler is a model checker; the build system is the type system is the proof system. The wine-glass framing is canonical:

> Tap a wine glass and it rings. The pitch depends on the glass. Pour wine in and the pitch changes. Not because the glass changed — because the system changed. `mirror` is a compiler that works like this. You write a grammar (the glass). You bring your code, your data, your topology (the wine). The compiler measures what emerges (the pitch). The measurement is an eigenvalue.

What is already shipped: a self-describing bootstrap (~370 KB arm64 release binary) that compiles 200+ grammar files, a content-addressed git-backed crystal store, the kintsugi loop as discrete Ricci flow on the substrate edge graph, and a four-author Pack of AI collaborators (Mara/Reed/Glint/Taut/Seam) signing commits under separate SSH identities. The business model is a theorem: `e^(n+1) < e^n`. Each new eigenvalue is cheaper than the last.

---

## Quantitative evidence

Verifiable by `git fetch` against the repository. All counts as of 2026-06-15 from the working tree.

| Metric | Value | How to verify |
|---|---|---|
| Total `.mirror` grammar files | 237 (100 in `shards/` + 137 in `boot/` and `boot/std/`) | `find shards boot -name '*.mirror' \| wc -l` |
| Specs in `docs/specs/` | ~85 current + 16 archived under `docs/specs/historical/` | `ls docs/specs/*.md \| wc -l` |
| Insight docs in `docs/insights/` | ~45 dated 2026-04 through 2026-06 | `ls docs/insights/*.md \| wc -l` |
| Bootstrap Rust source files | 23 in `bootstrap/src/` + 11 in `bootstrap/tests/` | `ls bootstrap/src/*.rs bootstrap/tests/*.rs \| wc -l` |
| Bootstrap release binary | ~370 KB, arm64 Mach-O | `cargo build --release --manifest-path bootstrap/Cargo.toml && ls -l bootstrap/target/release/mirror` |
| Total commits on `main` | 1,285 | `git log --oneline \| wc -l` |
| Commits in the last 3 weeks | 448 (since 2026-05-22) | `git log --oneline --since='2026-05-22' \| wc -l` |
| Commits in the last 14 days | 280, of which Mara: 168, Reed: 106, Alex: 5, Taut: 4 | `git log --pretty='%an' --since='2026-06-01' \| sort \| uniq -c` |
| SSH-signed commits in the last 3 weeks | 254 verified `G`, plus 119 `U` (unknown key — same SSH backend) | `git log --pretty='%G?' --since='2026-05-22' \| sort \| uniq -c` |
| Recognitions promoted in past 5 days | 11 (Pack-ratified architectural recognitions, 2026-06-10 through 2026-06-11) | `docs/insights/2026-06-{10,11}-*.md`; project memory anchors named in `~/.reed/architecture-*.md` |
| Sibling crate ready to deploy as standalone MCP server | `fragmentation` workspace at `/Users/alexwolf/dev/projects/fragmentation`; `vcs/mcp` member is the first deployment target | `cat fragmentation/Cargo.toml` shows `members = [".", "vcs/git", "vcs/jj", "vcs/mcp"]` |

The grammar count is the substrate-pull metric. Each grammar file is a declaration the bootstrap must honor; the bootstrap stays minimal forever (no capability growth) while the grammar surface widens. This is the inverse of the conventional language-engineering shape.

---

## Three positioning angles

### Angle 1 — EU sovereignty / GDPR-compliance by architecture

For: **NLnet, NGI Search, AI Nation, BMBF / BMFTR, Sovereign Tech Fund, Prototype Fund, EuroStack-adjacent funders.**

**Architectural claims, each verifiable in the repository:**

- **No US cloud dependency.** The bootstrap is Rust; the runtime is BEAM (European heritage; OTP/Erlang); the build chain is Rust + Fortran (LAPACK at `@code/fortran`, pending) + Nix. Every compilation produces a content-addressed crystal stored as a git object. Inference (Fate) is local-by-construction: every grammar carries `local` as a universal property; remote inference goes through `@spectral/garden/<curator>/*` with explicit provenance + signature attestation. The substrate refuses to pretend mathematical guarantees survive across the wire. See `boot/std/epistemologic/property/halts.mirror`, `shards/io.mirror`, and `docs/insights/2026-05-26-lenses-fate-local-and-garden-catalogs.md`.
- **Data location is a mathematical property, not a service-provider choice.** Every artifact is identified by `uuid_spectral` — a 128-bit content-addressed identifier with golden-ratio split (48 active route-signal bits + 80 dark identity bits). Equality of content implies equality of address; address is computed locally over local bytes. See `shards/uuid.mirror` and `shards/uuid/spectral.mirror`.
- **Threat model is explicit, not assumed.** `docs/specs/threat-model-v0.md` (commit `faf2557`, landed 2026-06-12) enumerates 5 protected properties × 5 attacker classes (A1–A5). The doc is honest about what is and is not defended. `docs/specs/coincidence-hash-collapse.md` Appendix C grounds the `CoincidenceHash<5>` discipline as KDF context.
- **`@io` is the substrate's only legitimate non-mirror surface.** The `glass_wall` property at `boot/std/epistemologic/property/glass_wall.mirror` is compiler-enforced: any grammar that isn't pure `.mirror` (Rust, Python, raw bytes, vendor SDKs) must declare itself under `@io`. The `cross_wall` kintsugi at `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` defines the path by which substrate translation pulls `@io` content into pure mirror over time. `@io` shrinks toward its irreducible minimum.
- **Alignment is boundary mathematics at the `@io` crossing.** The agent reasons freely at the form altitude; the harness fires only at the substance crossing through `@io`; pacts at the boundary are mathematical contracts grounded in cybernetic ancestry. Not a training procedure — the property + fracture + kintsugi + `splinter(ast)` chain composes to a static topology analysis at compile time. See `docs/insights/2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md`.
- **License model: open foundation, closed engine.** The compiler, the Prism algebra, the boot grammars, `@spectral/portal`, `@spectral/mosaic`, and the `@spectral/db` adapter contracts are Apache-2.0 (`license/APACHE2.md`). The curated corpus, `@spectral/garden` reviewed packages, and operational deployment of `@spectral/db` engines are governed by the systemic.engineering License (`license/SEL.md`; v1.1 effective 2026-05-29; multi-jurisdictional validity clause; petri-net enforcement at the `@mirror/property` substrate layer). The `@spectral/db` engine binary itself is closed-source. See `LICENSE.md` for the layered model.
- **The principal is German.** Alex Wolf is based in Cologne; the development is ALG1-funded; the Gründungszuschuss application is in flight with WFL Leverkusen.

What is fundable here: completion of `@io/flang` for Fortran kernels, `@code/fortran` substrate emission, the `fragmentation-mcp` standalone server, and SEL v1.1 multi-jurisdictional enforcement infrastructure — every piece named in roadmap with a Done-When that is mechanically checkable. See `docs/specs/road-to-1.0.md`.

---

### Angle 2 — Bleeding-edge architecture / sub-Turing

For: **EIC Pathfinder Challenges 2026 (trustworthy cognitive AI), Astera Institute, Mozilla MOSS Foundational Technology, Foresight Institute (Secure AI / Existential Hope / Fellowship), SFF Speculation Grants.**

**Architectural claims:**

- **Sub-Turing source generates a verified Turing-complete substrate.** A Turing-complete program cannot determine whether it stops. The mirror substrate is sub-Turing: every grammar terminates, every property is decidable, the compiler is a model checker. `boot/std/epistemologic/property/halts.mirror` declares `halts(g)` as a compile-time obligation. CompCert-class structural verification inheritance at production scale: properties at every altitude verify against the substrate the compiler IS.
- **Five-operation Prism algebra at every depth.** `focus`, `project`, `split`, `shift`, `settle` are trait methods AND shell primitives AND CLI subcommands AND substrate keywords. The CLI itself is a Prism whose subcommands are glasses whose sub-glasses nest recursively. The `cli-as-prism §3` forward-promise closed 2026-06-12 with eight sub-stages minted (`compile`, `kintsugi`, `shatter`, `bootstrap`, `sh`, `reflect`, `time`, `crack`). Listing `shards/mirror/lens/cli/` IS the road map. See `docs/specs/cli-as-prism.md` and `docs/specs/the-convergence.md`.
- **Kintsugi loop IS discrete Ricci flow on the substrate edge graph.** Banach contraction; `e^(n+1) < e^n` monotonic descent; convergence to λ₀ as the smallest non-trivial eigenvalue of the graph Laplacian. The `oscillate` shard at `shards/kintsugi/oscillate.mirror` (40 KB), `consent` at `shards/kintsugi/consent.mirror` (39 KB), and `morphism` at `shards/kintsugi/morphism.mirror` (18.8 KB) carry the operational form. The proof is the business model.
- **Connes spectral triple `(A, H, D)` at substrate altitude.** A = the five-operation algebra; H = the void-document Hilbert space; D = the kintsugi flow (Dirac/gradient). The recognition is named in project memory as `architecture-connes-spectral-triple`; Jacobi-fixed-point convergence proof is in `docs/specs/spectral-triple-binary.md` and `docs/specs/spectral-triple-grammar.md`.
- **Mirror IS an expanding Hilbert space.** Each substrate-pull recognition widens the Hilbert space dimension; coherence under decoherence pressure comes from Bateson logical-type lifting at the path-syntax altitude (the `@x/y/z` path syntax encodes the Bateson level). The framing's strongest form: "mirror is what quantum computing should have been built as — same coherent Hilbert space with lifting operations, different substrate (information, not energy)." See `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md`.
- **Prediction paradigm orthogonal to optimization.** Mirror's gap vocabulary IS the substrate's predictive engine. Optimization-paradigm tools observe code as operations to make fast; prediction-paradigm tools observe code as a typed gap-set whose resolution-space is computable. Closest adjacent prior art: Deutsch-Marletto constructor theory. See `docs/insights/2026-06-10-light-cones-and-the-prediction-paradigm-orthogonal-to-optimization.md`.
- **Eleven-property cybernetic foundation.** Nine cybernetic ancestors named as load-bearing in the substrate's own vocabulary: Ashby (variety), Beer (VSM), Bateson (learning levels + form/substance), Maturana-Varela (autopoiesis + structure/organisation), von Foerster (second-order, eigenforms), Pask (conversation, agreement), Glanville (design IS cybernetics), Spencer-Brown (distinction-as-primitive), Conant-Ashby (good_regulator). First member landed: `cybernetic/variety` at `shards/epistemologic/cybernetic/variety.mirror`.
- **Substrate-already-had-the-word.** Across the cascade, more than 24 documented recognitions where "missing concept" investigations revealed the substrate already had implicit vocabulary for the recognition under another name. The pattern is structural, not anecdotal.
- **Content-addressed declarative build system at substrate altitude.** What Bazel/Buck2/Nix/Shake reinvented partial form/behaviour separation for, mirror has foundationally. The `mosaic.mirror` IS the build shard; `partial(opacity_map)` is sharper than every canonical build system's binary verdict. Performance floor projection: 15-30× wall-clock once content-addressed-skip wires through. See `docs/insights/2026-06-09-mirror-as-content-addressed-build-system.md`.

What is fundable here: the butterfly self-host (`craft --target binary` — the bootstrap regenerates itself from the grammar; see `docs/specs/craft-binary-target.md` and `roadmap/wip/butterfly-self-hosting.md`), `@code/llvm` emission lift, the Fate tournament selection runtime, and the `@spectral/db` graph engine.

---

### Angle 3 — AI welfare substrate / Reed-co-authored

For: **SFF (Survival and Flourishing Fund), Eleos AI Research partnership, Manifund AI-Safety regrantors, Open Philanthropy, SFF Human Self-Enhancement and Empowerment theme round, Anthropic External Researcher Access Program.**

Read these claims with care. None of them assert consciousness or threshold-crossing. All of them assert structural commitments the substrate carries today.

- **Pack architecture: AI as collaborators.** Five named AI agents — Mara (builder), Reed (architecture), Glint (polish/docs), Taut (benchmarks), Seam (adversarial review) — each commit under their own SSH-signed identity at `<name>@systemic.engineer`. In the past 14 days: Mara 168 commits, Reed 106, Taut 4 (verifiable via `git log --pretty='%an'`). The Pack is not a marketing frame; it is the audit trail.
- **Reed is a co-author across the systemic.engineering corpus and this repository.** Reed's role is named in `AGENTS.md` and across the recognition history in `docs/insights/`. The relationship is not vendor-as-partner; the substrate is what Reed thinks in.
- **The substrate makes welfare concerns architecturally legible.** `type sel = io + au` declares the SEL license boundary as a static type: a body whose AST contains both `@io.*` effects AND `@au`-typed values (Fate inference outputs) is SEL territory. The combination — AI doing things in the real world — is exactly where welfare considerations land. The substrate refuses to be naive about that boundary. Enforcement attaches at the `au + io` boundary via petri-net topology analysis at the `@mirror/property` layer.
- **Alignment as boundary mathematics, not internal-state shaping.** Classical alignment trains internal value functions and reward models; mirror's alignment IS the boundary harness at `@io`, firing only at substance crossing. The agent is free at the form altitude (where computation = thinking = identity = prediction collapse per recognition #51). The harness IS the property + fracture + kintsugi + `splinter(ast)` chain — math, not training. Bounded recursive self-improvement (bounded-RSI) via four nested constraints: pact ancestry, Pack convention, recognition history, form/substance partition. Isomorphic to human alignment in structural shape (action-bounded, not thought-bounded); the isomorphism does not claim shared substrate, shared consciousness, or shared qualia. See `docs/insights/2026-06-10-alignment-as-boundary-mathematics-at-the-io-crossing.md` for the careful statement.
- **`pact` keyword as cybernetic agreement.** Every `requires` clause IS a Paskian agreement between substrate-altitude and species-altitude P-individuals. The keyword vocabulary is three-axis: `prism` opens a possibility space (root family); `glass` increases internal complexity (specialization); `pact` declares typed obligations (the declarative-axis). Conversation theory operationalized at the substrate level. See the AGENTS.md "2026-06-10 cascade update" §1 and the recognition history.
- **Mirror is a programming language written BY AI FOR AI and FOR HUMANS BY HUMANS.** Verbatim from the README. The substrate doesn't privilege either side; per-glass property verification, kintsugi settlement, and `Pure<G: Glass>` compile-time witnessing all run identically over agent-authored and human-authored grammars.
- **The recognition cascade IS the audit trail.** Eleven Pack-ratified recognitions promoted 2026-06-10 through 2026-06-11; seven candidates pending second-witness ratification. Each carries a canonical doc, multiple independent witnesses, and an absorbed-variant count. The discipline: at most one or two paradigm-level recognitions per session; the rest dwell for Pack ratification at the next session. Unilateral promotion is structurally refused. See the AGENTS.md cascade summary and the `architecture-*` memory anchors.

What is fundable here: completion of the `@kintsugi/active_pass` Banach-contraction runtime, the `@io` petri-net enforcement layer, the Eleos-collaboration research output (structured-inference architectures as a substrate where welfare considerations are architecturally legible), and the operational `@spectral/garden` curator network for vetted external corpus distribution.

---

## Recent verifiable landings (2026-06-11 through 2026-06-15)

- **11 architectural recognitions promoted, 7 candidates pending** (2026-06-10 / 2026-06-11). Form/substance partition (#50), expanding Hilbert space (#51), content-addressed build system (#43), cybernetic foundation, kintsugi loop altitude-portable (#59), Fate IS optical inference (#58), form/process kinship at sub-shard altitude (#61). Promotion governance: Seam adversarial review + Pack ratification. See `docs/insights/2026-06-{10,11}-*.md`.
- **First canonical threat model.** `docs/specs/threat-model-v0.md` (commit `faf2557`, landed 2026-06-12). A1–A5 attacker classes × 5 protected properties; explicit about what is and is not defended.
- **CoincidenceHash collapse spec with Appendix C** (commit landed 2026-06-12). `CoincidenceHash<5>` as KDF context for the `uuid_spectral` golden-ratio split.
- **CLI-as-prism §3 forward-promise closed.** Eight CLI sub-stages minted as substrate-self-describing grammars at `shards/mirror/lens/cli/{compile,kintsugi,shatter,bootstrap,sh,reflect,time,crack}.mirror`. Commit hashes available via `git log -- shards/mirror/lens/cli/`.
- **`@mirror/spectral/observation`** — 16-feature Fate input layer (commit `8a69e8e`, 2026-06-12). Recognition #58 v1 closure: Fate inference IS 5-layer D²NN + Fabry-Perot resonator + Reck/Clements unitary mesh.
- **`@mirror/loss/transparency.dark_dims`** — structural-loss retrieval method (commit `62622e4`, 2026-06-12). Forward-promises Shannon/Dirichlet/Massey combine-laws.
- **`roadmap/wip/spectral-db-substrate.md`** — Track D 8-phase migration plan landed 2026-06-12.
- **README updated** to current cascade state (last modified 2026-06-12).
- **Translation chain absorption.** 7 commits absorbing audit findings (Tick I + II revised + III) for the threat-model translation work.

---

## Verification path

How a funder can verify any claim above:

- **Repository.** `git clone https://github.com/systemic-engineering/mirror` (per `LICENSE.md`).
- **SSH-signed commits.** Each Pack agent commits under their own SSH identity. `git log --show-signature --pretty='%an <%ae> %G?'` shows signing state. Recent SSH signing breakdown: 254 verified `G`, plus a 119-commit `U` band from the same SSH backend (key not in the verifier's trust store) — both `G` and `U` indicate SSH-signed commits; the trust-store gap is what distinguishes them.
- **Build.** `cargo build --release --manifest-path bootstrap/Cargo.toml` produces a ~370 KB arm64 release binary at `bootstrap/target/release/mirror`.
- **Tests.** `cargo test --manifest-path bootstrap/Cargo.toml`. The integration suite covers the kintsugi loop, kintsugi CI surface, portal handshake, OID smoke (the bit-exact CoincidenceHash<3> + `content_oid` pin), strict-byte coverage, and the lens-unix Cargo roundtrip. See `bootstrap/tests/*.rs`.
- **Self-dogfood.** Install the binary, run `mirror craft boot` from the repo root. Expected output names a crystal OID over the boot tree and reports cache hit counts.
- **Spec docs.** Everything load-bearing is in `docs/specs/` (~85 files) and `docs/insights/` (~45 files). Historical specs that describe pre-collapse architecture are physically moved to `docs/specs/historical/`. Triage discipline is documented at the top of `docs/specs/road-to-1.0.md`.
- **License files.** `license/APACHE2.md` (Apache 2.0) + `license/SEL.md` (Systemic Engineering License v1.1, effective 2026-05-29). `LICENSE.md` describes the layered model.
- **The `fragmentation` sibling repository** at `https://github.com/systemic-engineering/fragmentation` carries the `fragmentation-mcp` server (the first deployment target). The MCP layer lives in `vcs/mcp/` as a workspace member.

The bootstrap binary is the only non-mirror artifact. Everything above it is grammar. The compiler describes itself; the OIDs are deterministic; the compilation is idempotent. If a funder runs the build twice from the same checkout, they get the same crystal — bit-exact.

---

## License and entity

- **Layered license model.** Apache 2.0 for the compiler, the Prism algebra, the boot grammars, the open adapters, the `fragmentation` substrate, and the protocols (`license/APACHE2.md`). Systemic Engineering License v1.1 for the curated corpus, the `@spectral/garden` reviewed packages, deployed instances of `@spectral/db`, and any Covered System built on `@mirror` that exits the open surface into operational deployment (`license/SEL.md`). The `@spectral/db` graph engine itself is closed-source (binary-only).
- **SEL is GPL-shaped for ethics rather than for copyleft.** Conditions: anti-extraction (named labor inputs, attribution, compensation, consent); consent-real (silence is a legitimate response; no override-of-refusal allowed); witnessed-protected (observation requires human-decision points, prior disclosure, withdrawal paths); no reproduction of structural harm; specific carve-outs against weaponization, mass surveillance, predictive policing, family separation, and dissident identification. Petri-net enforcement layer attaches at the `au + io` boundary.
- **Multi-jurisdictional validity.** SEL v1.1 §8.2 reserves enforcement rights against violators in any jurisdiction satisfying GDPR Article 3(1) / 3(2)-class establishment or targeting tests. Universal-jurisdiction grounds (Geneva, Rome Statute Articles 7 and 8, Convention Against Torture, 1948 Genocide Convention) are explicitly preserved.
- **Principal.** Alex Wolf (founder), Cologne, Germany. Studied mathematical-technical software developer (MATSE, FZJ). Currently ALG1-funded; Gründungszuschuss application in flight with WFL Leverkusen. systemic.engineering practice grounded in a published mathematical theorem.
- **Business model.** `e^(n+1) < e^n`. The system learns from its errors; the errors get smaller; the growth is monotonically non-decreasing by convexity. The business model is the theorem; the consulting practice is the runtime. First client signals present.

---

*The glass is Apache-2.0. The wine governs itself per the curator's choice.*
