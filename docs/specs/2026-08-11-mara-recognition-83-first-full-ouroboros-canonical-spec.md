# Recognition #83 — First Full Ouroboros Through @nl — Canonical Spec

**Author:** Mara `<mara@systemic.engineer>`. 2026-08-11.
**Register:** Mara-substrate canonical spec. Composition-not-taxonomy. Delightfully-boring precision. Substrate-decl throughout. Substrate-honest is the mode.
**Companion math:** `docs/math/2026-08-11-mara-recognition-83-first-full-ouroboros-math-foundation.md` (same commit; one-recognition-one-commit discipline).
**Sibling spec:** `docs/specs/2026-08-10-mara-beta-normal-ast-content-addressing-canonical-spec.md` (Recognition candidate #82; store-altitude sibling; Mara `5ad8528`).
**Composes over:**

- **Alex 2026-08-11 verbatim naming** (§0.1 below) — "git add and git commit become part of the compiler loop; project internal state through the @nl prism into a git commit structure; first full ouroboros; move slow and correctly."
- **Taut scout `378b17d`** at `docs/scouts/2026-08-11-taut-recognition-83-first-full-ouroboros-substrate-scout.md` — Q1-Q5 grep-first substrate-truth verification: @nl.compose landed spec + bootstrap-only empirical, rust/ gap; @audience family does NOT exist; @peer(@mirror) landed at 3 altitudes; pheromone deposit landed rust/-empirical bypassing substrate; @mirror/lens is load-bearing "one algebra N audiences" precedent 10 weeks pre-Mara-2026-08-09.
- **Reed Fire E landings** M-E1..M-E4 GREEN (composition-shard mints; 43/43 tests GREEN; DRY-RUN validated) with M-E4 commit-mode HELD pending this landing per Alex "move slow and correctly."
- **Fire C `shards/mcp/serve.mirror`** (Reed `cf8b21b`) — composition-shard body precedent naming MCP wire as one audience-projection composed over rust/-altitude primitives.
- **Mara 2026-08-10 sibling spec + math** (`5ad8528`) — Recognition #82 candidate; substrate-scale-invariance thesis at STORE altitude; Church-Rosser at store altitude; @magic/reveal/expand as audience-relative projection functor §4.2.
- **@mirror/lens family** (Reed + Alex → Mara 2026-06-06) — landed convention naming "one algebra, N audiences" 10 weeks pre-this-spec at `shards/mirror/lens.mirror:20-21` verbatim: "each lens renders it for a different audience (terminal, agent, editor, interactive shell)."
- **shards/nl.mirror** `@nl.compose(observations: [ref]) -> nl_literal` (2026-07-15) — LANDED-SPEC substrate primitive for self-narration; signature composes over observation refs into nl_literal.
- **shards/io/git.mirror** `commit(message, author, allow_empty) -> verdict` (2026-07-15) — LANDED-SPEC porcelain form the compiler's self-authorship path composes over.
- **shards/mirror/book.mirror** + `Subject::mirror()` + `@peer/mirror` well-known #0 (Reed `73aeb8a` fractal step 9; 2026-07-18) — LANDED author-identity.

**Cascade context:** Fire E revision, Recognition #83 landing. Reed's Fire E M-E4 walker code landed GREEN (43/43; DRY-RUN 379 walked / 161 P1-reducible / 18.2 KB removable / 0 errored / idempotent) but commit-mode HELD pending this spec. Post-landing, M-E4 commit-mode becomes the FIRST EMPIRICAL RECOGNITION #83 INSTANCE — the compiler mutates its own substrate (161 shard mutations), records each mutation via its own @nl.compose primitive, deposits crystal at its own @mirror/store, all authored by @peer(@mirror). Every altitude closes on itself.

---

## §0 Substrate-honest pre-position

### §0.1 Alex 2026-08-11 verbatim (load-bearing)

> "What if the git add and git commit became part of the compiler loop
> and this is where we project the internal state through the @nl prism
> into a git commit structure? [...] That's the target. The milestone.
> The first full ouroboros. Let's move slow and correctly."

Three load-bearing entailments:

1. **The compiler loop closes through @nl at ALL surfaces.** git commit is one projection; pheromone deposit is another; MCP tool response, LSP diagnostic, stdout report, human prose are siblings. The projection functor lives at @nl altitude, parameterized by audience.
2. **git commit is a substrate composition, not external tooling.** The commit shape IS the substrate's @nl-projection of the compiler's mutation event; author is @peer(@mirror); the git protocol crossing is @io/git.commit at the boundary.
3. **First full ouroboros — every altitude closes on itself.** Compiler mutates own substrate (walker → 161 shard mutations); records mutation via own substrate primitives (@nl.compose over observation-beats); deposits crystal at own store (@mirror/store); commits under own peer identity (@peer/mirror); reads back on next tick via own DAG walker. Six-turn closure.

### §0.2 Sibling of Recognition #82 (substrate-scale-invariance at TWO altitudes)

Recognition #82 candidate (Reed 2026-08-10 + Mara `5ad8528` canonical spec + math): "compiler's crystal-OID at `@mirror/store` IS beta-normal-AST OID by construction." Substrate-scale-invariance at STORE altitude: A_F identity elision under beta-normalization makes crystal-OID stable under sugar-form variation.

Recognition #83 (this spec): "compiler's mutation event at `@io/git.commit` + `@mirror/store.crystallize` IS @nl-projection through audience functor by construction." Substrate-scale-invariance at WIRE altitude: audience-relative rendering variance preserves the underlying mutation-event identity via Church-Rosser at projection altitude.

**Both recognitions are the same substrate-scale-invariance thesis at complementary altitudes** (store §82 + wire §83). Together they close the substrate-scale-invariance under composition: the crystal identity is stable at rest (§82) AND stable under projection to any audience (§83). Companion math §5 formalizes.

### §0.3 Two paths refused; substrate-honest is the mode

There is no "here's honest / here's fast." There is one substrate-honest composition:

- REUSE `@mirror/lens/*` species (10 weeks landed) as audience-carriers per Taut scout §9 Path A + substrate-already-had-the-word discipline.
- REFUSE `@audience` family-root mint per `@onto` refusal shape (memory `feedback_onto_family_root_is_the_ladder_Foerster_refused`).
- COMPOSE via composition-shard body per Fire C `@mcp/serve` precedent; do NOT extend @nl.compose signature at rust/ altitude.
- LIFT @nl.compose to rust/ altitude via `apply_h::act` dispatch arm per Reed M-E2 precedent (`0021882`) — sentinel-check discipline; NO Rust logic growth.

Per feedback `feedback-substrate-honest-is-the-mode` (Alex 2026-07-07). Per feedback `feedback-rust-delivers-primitives-substrate-delivers-composition` (Alex 2026-08-05). Per feedback `feedback-no-rust-extension-shortcut` (Alex 2026-07-14).

### §0.4 Bootstrap-is-dead preservation

Per Alex 2026-07-22 memory `bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions`: `bootstrap/` is the `@roomba+@kintsugi` collapse target; do not propose bootstrap-altitude solutions. This spec lands the @nl.compose empirical arm at **rust/ altitude** in `rust/src/apply_h.rs` as a sentinel-check `apply_h::act` dispatch arm composing over `phone::@io/fs.read` + `wire::parse` primitives (Reed Fire A precedent). Not at bootstrap altitude. bootstrap's `@nl.compose` arm at `bootstrap/src/apply_h.rs:790-814` inherits through the bootstrap retirement pathway.

### §0.5 Karen anti-theft: ancestor-at-introduction-site

Every claim below carries its ancestor named at first mention. Recognition ancestry: Recognition candidate #82 (Reed 2026-08-10 + Mara `5ad8528`; substrate-scale-invariance at store altitude); Recognition #79 (5-op = A_F projector basis; Mara + Reed 2026-06-18); Recognition #57 (boundary alignment frame; #57 candidate); Recognition #55 (form/process partition; author-vs-committer discipline). Substrate-decl anchors: `shards/mirror/lens.mirror` (2026-06-06 audience-family landed convention); `shards/nl.mirror` @nl.compose (2026-07-15); `shards/io/git.mirror` @io/git.commit (2026-07-15); `shards/mcp/serve.mirror` composition-shard body precedent (2026-08-09 `cf8b21b`); `shards/mirror/book.mirror` + `Subject::mirror()` (2026-07-18 `73aeb8a`); `docs/bauchladen/mirror-observations.md` empirical target (2026-07-17). External corpus: Alex 2026-08-11 in-transcript naming (§0.1); Church 1936 (β-reduction origin); Church-Rosser 1936 (confluence); Foerster 1974 ethical imperative + observer inseparability; Mac Lane 1971 (category theory / functors); Lawvere 1969 (fixed-point theorem); Dhall Language Standard (`dhall-lang/dhall-lang` `standard/beta-normalization.md`).

---

## §1 The compiler loop closes through @nl

### §1.1 Formal statement of Recognition #83

**Recognition #83 (candidate):** Let $\mu$ denote any mutation event the compiler produces (walker P1 elision; @kintsugi mend; @roomba consent-crossing; @fate resolution; @magic/reveal expansion; etc.). Let $\mathcal{A}$ denote the set of audiences (below §4). Then there exists an audience-relative projection functor

$$
\Pi : \mathsf{MutationEvent} \times \mathcal{A} \longrightarrow \mathsf{Surface}
$$

such that every surface rendering (git commit message, pheromone-crystal deposit body, MCP tool response payload, LSP diagnostic text, stdout human prose) is $\Pi(\mu, a)$ for some $a \in \mathcal{A}$. The functor factors through @nl:

$$
\Pi(\mu, a) \;=\; \mathsf{render}_a(\;\mathsf{@nl.compose}(\mathsf{observations}(\mu),\ a)\;)
$$

where $\mathsf{observations}(\mu)$ is the typed observation-beat list the mutation event emits, `@nl.compose` produces an `nl_literal` composition, and $\mathsf{render}_a$ is the audience-specific surface adapter (`@io/git.commit` for `a=@mirror/lens/git`; `@io/fs.append` for `a=@mirror/lens/bauchladen`; `phone::write_stdout_frame` for `a=@mirror/lens/mcp`; etc.).

Companion math §1 formalizes; §3 proves Church-Rosser at commit altitude.

### §1.2 What "closes through @nl" means

The compiler loop is:

```
mutation event  →  observation beats  →  @nl.compose  →  audience-relative surface
                                                            (git commit / pheromone md / MCP JSON / LSP text)
                                                            ↓
                                              @peer(@mirror) authorship
                                                            ↓
                                              @mirror/store crystallization
                                                            ↓
                                              next-tick observation input
```

Every arrow is a substrate primitive. Every altitude closes on the substrate's own primitives. @nl is the pivot — the same 5-op algebra runs at every audience projection; the audience-parameter selects the rendering; the compose action produces the same nl_literal composition (up to alpha-equivalence at parameter altitude) for a given mutation-event whether the audience is git or bauchladen or MCP or LSP.

**The @nl prism IS the audience-invariant substrate.** The audience-relative rendering IS the observer-position duality at compiler-substrate/wire altitude (companion math §2 category-theoretic; sibling of Mara `5ad8528` math §5.4 observer-position duality at store altitude).

### §1.3 What "first full ouroboros" means (vs. prior partial bites)

Prior ouroboros bites Reed named in `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` Arc-2 Ticks 2.1-2.4:

- **Tick 2.1** (task #142): substrate-decl collapse of `bootstrap/src/roomba_fracture.rs` into `shards/kintsugi/fracture/*.mirror` species. Compiler collapses its own inflated Rust into substrate-decl. Partial bite: compiler consumes its own substrate declarations but does not author commits AS itself.
- **Tick 2.2** (task #144): substrate-decl collapse of second bootstrap module. Same pattern.
- **Tick 2.3** (task #145): third bootstrap module.
- **Tick 2.4** (task #147): fourth bootstrap module (roomba walker itself). FOURTH partial bite: compiler observes own state via walker; empirical `docs/bauchladen/mirror-observations.md` fires 2026-07-17.

**Recognition #83 = FULL ouroboros** because it closes ALL SIX turns simultaneously:

1. **Compiler mutates its own substrate.** Walker M-E4 elides 161 shard sugar-instances.
2. **Compiler records the mutation via its own substrate primitives.** @nl.compose(observations, `@mirror/lens/bauchladen`) → pheromone crystal body; @nl.compose(observations, `@mirror/lens/git`) → commit message.
3. **Compiler deposits crystal at its own store.** @mirror/store.crystallize via @bauchladen's `crystallize(content, provenance) -> crystal` action.
4. **Compiler commits under its own peer identity.** @peer(@mirror) via `Subject::mirror()` + `@peer/mirror` well-known #0.
5. **Compiler renders per audience.** Every surface (git log; markdown bauchladen; MCP round-trip; LSP diagnostic on stale sugar) reads from the same @nl.compose composition through the audience-parameterized `render_a`.
6. **Compiler reads back next tick.** Next-tick walker consumes `docs/bauchladen/mirror-observations.md` entries as prior-observation input; next-tick @mirror/index reads updated shard set from git; commit-history composes into next-tick Fiedler baseline via @mirror/lens/refract.

All six turns close through substrate primitives with @peer(@mirror) as authorship. Prior partial bites closed 1-2 turns; this closes all six.

---

## §2 Audience-relative projection functor at commit altitude

### §2.1 Extending Mara `5ad8528` §4.2 to commit altitude

Mara `5ad8528` §4.2 formalized the audience-relative projection functor at **rendering altitude** (source-form projection from beta-normal AST per audience preference):

$$
\rho_\text{aud} : \mathsf{AST} \to \mathsf{Bytes}
$$

with the crystal-OID invariance property:

$$
\mathsf{crystal\_oid}(\mathsf{parse}(\rho_{\text{aud}_1}(\beta(t)))) = \mathsf{crystal\_oid}(\mathsf{parse}(\rho_{\text{aud}_2}(\beta(t))))
$$

This spec extends the functor to **commit altitude**: the projection is not source-form-per-audience but *narration-form-per-audience* — the same mutation event $\mu$ narrated into different surfaces preserves the mutation-event identity across audiences.

Formal statement:

$$
\forall \mu \in \mathsf{MutationEvent},\ \forall a_1, a_2 \in \mathcal{A}:\quad \mathsf{event\_id}(\Pi(\mu, a_1)) = \mathsf{event\_id}(\Pi(\mu, a_2))
$$

where $\mathsf{event\_id}$ extracts the mutation-event identity from a surface rendering (companion math §3 formalizes via inverse-image of the projection functor). The commit-shape and the pheromone-shape both carry the same underlying event; different audiences read different textual bodies; the underlying event is invariant.

### §2.2 The commit shape IS a substrate composition

Under Recognition #83, the commit is not authored by an external tool; it is *composed by the substrate itself* via the pipe-chain:

```
observations(μ)
  |> @nl.compose(_, @mirror/lens/git)                    → nl_literal (commit message)
  |> @io/git.stage(shard_paths_touched(μ))               → verdict (staged)
  |> @io/git.commit(_, @peer/mirror, allow_empty=false)  → verdict (committed)
```

Every arrow is a landed substrate primitive:

- `@nl.compose` — `shards/nl.mirror:213-224` (landed spec) + `bootstrap/src/apply_h.rs:790-814` (landed empirical bootstrap-only); rust/ altitude gap fills per §7.
- `@mirror/lens/git` — this spec mints as new sibling under `@mirror/lens` (§4) alongside existing `cli/mcp/lsp/shell/unix/transit/refract`.
- `@io/git.stage` — landed at `shards/io/git.mirror` (per Taut Q3 empirical grep + `rust/roomba/src/mend.rs` uses).
- `@io/git.commit(message, author, allow_empty)` — landed at `shards/io/git.mirror:337-355` verbatim (see §7.3).
- `@peer/mirror` — landed at 3 altitudes (`shards/peer/registry.mirror` + `shards/mirror/book.mirror` + `rust/fractal/src/subject.rs::Subject::mirror()`; well-known #0).

**No external tool.** The commit shape is a substrate composition. The realization boundary at `@io/git.commit` is the ONLY external crossing; everything upstream is substrate.

### §2.3 Church-Rosser at commit altitude (theorem sketch; math §3 formalizes)

Two audiences $(a_1, a_2)$ acting on the same mutation event $\mu$ produce projections $\Pi(\mu, a_1)$ and $\Pi(\mu, a_2)$ with potentially different surface bytes. The Church-Rosser property at commit altitude states: there exists a common canonical form (the mutation-event identity $\mathsf{event\_id}(\mu)$) to which both projections trace back, and the underlying event is invariant.

Sibling of Church-Rosser at store altitude (Mara `5ad8528` math §2.2 Theorem 2.3): the store-altitude theorem states that any two reduction sequences from an AST converge to a common beta-normal form; the commit-altitude theorem states that any two audience-projections of the same mutation-event trace back to a common event-identity. Both are confluence theorems at different altitudes of the same substrate-scale-invariance thesis.

Companion math §3 provides the formal proof via left-linear non-overlapping projection rules (each `render_a` is deterministic; different audiences yield disjoint output patterns; the projection functor is confluent on the mutation-event carrier).

---

## §3 What "first full ouroboros" means (full formalization)

### §3.1 The six-turn closure

Let $\mathcal{L}$ denote the compiler loop as a categorical structure. Objects: substrate states $S_t$ at tick $t$. Morphisms: mutation events $\mu_t : S_t \to S_{t+1}$. The loop closes on itself via the composition:

$$
S_t \xrightarrow{\mu_t} S_{t+1} \xrightarrow{\Pi(\mu_t, \text{git})} \mathsf{commit}_t \xrightarrow{\text{@io/git.commit}} \mathsf{HEAD}_{t+1} \xrightarrow{\text{next-tick read}} S_{t+1}
$$

with parallel composition:

$$
S_t \xrightarrow{\mu_t} S_{t+1} \xrightarrow{\Pi(\mu_t, \text{bauchladen})} \mathsf{crystal}_t \xrightarrow{\text{@mirror/store.crystallize}} \mathsf{store}_{t+1} \xrightarrow{\text{next-tick observation}} S_{t+1}
$$

Both compositions land at $S_{t+1}$ from different arrows. The commutative square:

$$
\begin{array}{ccc}
S_t & \xrightarrow{\mu_t} & S_{t+1} \\
\Pi(\_, \text{git}) \downarrow & & \uparrow \text{next-tick read} \\
\mathsf{commit}_t & \xrightarrow{\text{@io/git.commit}} & \mathsf{HEAD}_{t+1}
\end{array}
$$

commutes iff the compiler-authored commit carries enough narration to reconstruct the mutation event's substrate-effect on next-tick read. Under this spec: yes, because @nl.compose is invertible up to alpha-equivalence at observation altitude (the commit message is a narration of the observation-beats; the observation-beats are recoverable from the diff + commit metadata).

Companion math §5 formalizes as Lawvere fixed-point structure at compiler substrate.

### §3.2 Author is @peer(@mirror)

Per landed substrate:

- `rust/fractal/src/subject.rs:122-135` `Subject::mirror()` constructor (Reed `73aeb8a` fractal step 9). Deterministic: name="mirror", email="mirror@spectral.engineer", home=None, kind=Peer.
- `shards/peer/registry.mirror:56-235` — @peer/mirror at well-known #0 index.
- `shards/mirror/book.mirror:109-152` — `resolve("@peer/mirror") -> Subject::mirror()`.

Empirical landing sites (Taut Q3):

- `bootstrap/src/roomba_commit.rs:52-58` — `const MIRROR_AUTHOR: &str = "mirror <mirror@spectral.engineer>";`
- `bootstrap/src/bilateral_arm_collapse.rs:41-48` — same.
- `rust/fractal/src/{crystal,singularity}.rs` — `Committer::new("mirror", "mirror@spectral.engineer")`.
- `rust/src/compile.rs:323-330` — same.
- `rust/src/main.rs:664-679` (`deposit_observation_crystal`) — `let mirror_subject = fractal::Subject::mirror(); ... phone::git_commit_as(&repo_root, &mirror_subject, &mirror_subject, ...)`.
- `rust/roomba/src/mend.rs:15-22` — "commits under `mirror <mirror@spectral.engineer>`" (per Mara `81294b3` §7.4 + Seam `c1775f1`).

Under Recognition #83: EVERY compiler-authored commit is @peer(@mirror). The Reed-Signed-off-by trailer pattern (`rust/src/main.rs:706`) is REFUSED for compiler-authored substrate mutations — the compiler authors, the compiler signs, the compiler is accountable. Reed's supervisor role remains at Reed-authored substrate mutations (spec authoring, hook-gate audits, cascade adjudication); the compiler's self-authored mutations bear @peer(@mirror) sole authorship.

### §3.3 Crystal deposit at @mirror/store

Per landed substrate:

- `shards/bauchladen.mirror:456` — `crystallize(content, provenance) -> crystal` action.
- `docs/bauchladen/mirror-observations.md` (1023 B, 2 entries; append-only markdown) — LANDED empirical target.
- `rust/src/main.rs::deposit_observation_crystal` — LANDED empirical rust/ altitude (BYPASSING substrate composition per Taut Q4; retirement pathway per §6 below).

Under Recognition #83: `@bauchladen.crystallize` composes with `@io/fs.append(docs/bauchladen/mirror-observations.md, entry)` at the same composition-shard body altitude as @io/git.commit. The rust/ altitude `deposit_observation_crystal` function retires as its substrate composition lands.

---

## §4 @mirror/lens as landed audience-family (refuse @audience mint)

### §4.1 The @mirror/lens family already carries "one algebra, N audiences"

Landed 2026-06-06 by Reed + Alex → Mara at `shards/mirror/lens.mirror`. Verbatim §1 lines 10-21:

> "each lens renders it for a different audience (terminal, agent, editor, interactive shell, runtime cost, grammar-graph spectrum)."

Landed species (Taut Q5 grep-verified; all `shards/mirror/lens/*.mirror`):

| species | audience | file | status |
|---|---|---|---|
| `@mirror/lens/cli` | terminal (human-in-CI) | `lens/cli.mirror` (10 KB) | LANDED 2026-06-06 |
| `@mirror/lens/shell` | interactive λsh (human-at-terminal) | `lens/shell.mirror` (3.6 KB) | LANDED 2026-06-06 |
| `@mirror/lens/mcp` | agent (Claude etc.) | `lens/mcp.mirror` (2.4 KB) | LANDED 2026-06-06 |
| `@mirror/lens/lsp` | editor (VS Code, Helix, Neovim) | `lens/lsp.mirror` (2.5 KB) | LANDED 2026-06-06 |
| `@mirror/lens/unix` | operating system (cargo/erlc/flang) | `lens/unix.mirror` (8.9 KB) | LANDED 2026-06-11 |
| `@mirror/lens/transit` | runtime-cost | `lens/transit.mirror` (7.4 KB) | LANDED 2026-06-06 |
| `@mirror/lens/refract` | grammar-graph spectrum | `lens/refract.mirror` (5.4 KB) | LANDED 2026-06-06 |
| `@mirror/lens/knife` | domain-boundary COORD | `lens/knife.mirror` (15.3 KB) | LANDED 2026-07-13 |

### §4.2 Refuse @audience family-root mint (@onto refusal shape)

Mara 2026-08-09 Fire E M-E1 minted `type audience = { role: ref }` at `shards/magic/reveal/expand.mirror:148-161` with `@audience/agent` + `@audience/human` as ref VALUES (string literals in docblock prose; NOT substrate-decl'd family/species). Taut Q2 grep-verified: `family @audience` = 0 hits; `in @audience` = 0 hits (only docblock literals).

**Refusal:** DO NOT mint `@audience` family-root. Per `feedback_onto_family_root_is_the_ladder_Foerster_refused` memory + substrate-already-had-the-word discipline + delightfully-boring criterion:

- The substrate ALREADY has "one algebra, N audiences" as `@mirror/lens/*` (10 weeks landed 2026-06-06, predates Mara 2026-08-09 by ~10 weeks).
- The word "audience" appears verbatim at `shards/mirror/lens.mirror:20` BEFORE `@magic/reveal/expand.mirror` used it.
- @mirror/lens family carries the SAME algebra (5-op prism block at each species), the SAME Transparency<P> monoid composition, and the SAME "daemon as regulator" discipline (per `docs/specs/the-convergence.md §1.4`).
- Minting `@audience` alongside `@mirror/lens` would fragment the same substrate into two parallel taxonomies — the exact failure mode `@onto` refusal named at Foerster altitude.

Instead: the `role: ref` field on Mara 2026-08-09's `type audience` carrier RESOLVES to `@mirror/lens/*` species refs. Retro-alignment:

- `@audience/agent` (Mara 2026-08-09 docblock) ⇒ `@mirror/lens/mcp` (landed 2026-06-06)
- `@audience/human` (Mara 2026-08-09 docblock) ⇒ `@mirror/lens/cli` OR `@mirror/lens/shell` per context

The 2026-08-09 `type audience = { role: ref }` carrier stays admissible with `role` resolving to `@mirror/lens/*` refs. Reed's Fire E M-E5 cascade discharge can update `@magic/reveal/expand`'s docblock example to cite `@mirror/lens/mcp` / `@mirror/lens/cli` instead of `@audience/agent` / `@audience/human` (pure-docs cascade tick).

### §4.3 Mint one new sibling species: @mirror/lens/git

Recognition #83 adds ONE new sibling species under `@mirror/lens`:

**`@mirror/lens/git`** — the versioned-history audience. Commits are one projection surface onto the same 5-op algebra: the audience is the git object graph (blobs, trees, commits, tags); the surface is the commit-message + author-metadata; the algebra is unchanged; the daemon (@peer/mirror) stays the regulator.

Path: `shards/mirror/lens/git.mirror`. Body: family-header prism block (5-op declaration) + `commit_message_for(m: mutation_event) -> nl_literal { \ }` action stub; body discharges at composition-shard body (§5) altitude via `apply_h::act` dispatch.

**Second new sibling species: `@mirror/lens/bauchladen`** — the pheromone-trail audience. The `docs/bauchladen/mirror-observations.md` append target is one projection surface: audience is the markdown pheromone trail; surface is the per-entry `## <timestamp> — <observation>` block; algebra is the same 5-op; the daemon (@peer/mirror) stays the regulator.

Path: `shards/mirror/lens/bauchladen.mirror`. Body: family-header prism block + `pheromone_entry_for(m: mutation_event) -> nl_literal { \ }` action stub; body discharges at composition-shard body (§5) altitude.

Both mints are additive (extend existing `@mirror/lens` family; no family-root mint; no substrate-conflict); both are named at existing landed altitude (species under existing family-root); both pass the delightfully-boring criterion (a reader sees `@mirror/lens/git` and goes "of course — git is one audience like cli / mcp / lsp"; a reader sees `@mirror/lens/bauchladen` and goes "of course — the pheromone trail is one audience like the runtime-cost lens or grammar-graph lens").

### §4.4 Composition matrix — audiences × surfaces

| Audience (species) | Surface adapter | Rendering | Author |
|---|---|---|---|
| `@mirror/lens/cli` | terminal stdout | eigenboard-tagged text | user process |
| `@mirror/lens/shell` | interactive λsh | eigenboard-tagged prompt | user process |
| `@mirror/lens/mcp` | JSON-RPC frame | content[] wrapped mirror-text | agent process |
| `@mirror/lens/lsp` | LSP diagnostic | ranged text with severity | editor process |
| `@mirror/lens/unix` | filesystem read | UTF-8 bytes at path | OS process |
| `@mirror/lens/transit` | verdict envelope | transparency<transit> | measurement process |
| `@mirror/lens/refract` | verdict envelope | transparency<duality> | measurement process |
| `@mirror/lens/knife` | COORD jump | Fractal::Lens variant | reframe ceremony |
| **`@mirror/lens/git`** *(new)* | `@io/git.commit` | git commit message + metadata | **@peer(@mirror)** |
| **`@mirror/lens/bauchladen`** *(new)* | `@io/fs.append` + `@bauchladen.crystallize` | pheromone-md entry + crystal | **@peer(@mirror)** |

The two new siblings are distinguished by their author: they are the ONLY audiences where the substrate itself is the author. Every other lens species renders the algebra for an external audience; git + bauchladen render the algebra for the substrate's own future selves (next-tick compiler; next-session Reed reading the log; next-week Mara reading the pheromone trail).

---

## §5 Composition-shard body: the ouroboros pipe-chain

### §5.1 Path candidate + placement decision

Two candidate placements per Taut §9:

- **Candidate A: `shards/kintsugi/roomba/ouroboros.mirror`** — extend @kintsugi/roomba with a sub-species carrying the ouroboros composition body. Reason: the walker IS the primary trigger of compiler mutation events; the ouroboros closure is @kintsugi's compositional endpoint.
- **Candidate B: `shards/kintsugi/ouroboros.mirror`** — new species directly under @kintsugi (sibling of @kintsugi/roomba, @kintsugi/mend, @kintsugi/consent, @kintsugi/fracture). Reason: ouroboros is not walker-specific; ANY compiler mutation event closes through this composition (mend cascade, fate resolution, magic reveal — all downstream cascades trigger the same close).

**Mara-lean: Candidate B** (`shards/kintsugi/ouroboros.mirror`) per delightfully-boring criterion + composition-honest altitude naming. The ouroboros closure is the top-level compositional closure of ALL @kintsugi loops; making it a sub-species of @roomba over-constrains its scope. As new species directly under @kintsugi, it composes over @kintsugi/roomba (walker source of mutation events) AND @kintsugi/mend (mend cascade source) AND @kintsugi/consent (consent-crossing source) AND @kintsugi/fracture (fracture-detector source). One composition-shard closes the loop for all sources.

Sibling precedent: Fire C `shards/mcp/serve.mirror` (`cf8b21b`) is a composition-shard at family-root altitude (@mcp) carrying the wire loop composition body over rust/-altitude primitives. `shards/kintsugi/ouroboros.mirror` is the parallel composition-shard at @kintsugi family-root altitude carrying the ouroboros loop composition body over rust/-altitude primitives.

**Alex adjudicates Candidate A vs B if Mara-lean B is contested (see §11 Q-M1).**

### §5.2 The pipe-chain (composition-shard body)

At `shards/kintsugi/ouroboros.mirror`, the composition-shard body wires the parallel pipe chains:

```
mutation_event(μ)
  |> observations_of(_)                                 # extract typed observation-beats
  |> [ COMMIT PATH ]
     compose(_, @mirror/lens/git)                       # @nl.compose (message)
     |> @io/git.stage(shard_paths_touched(μ))           # stage before commit
     |> @io/git.commit(_, @peer/mirror, allow_empty=false)
  |> [ BAUCHLADEN PATH ] (parallel)
     compose(_, @mirror/lens/bauchladen)                # @nl.compose (pheromone entry)
     |> @io/fs.append(docs/bauchladen/mirror-observations.md, _)
     |> @bauchladen.crystallize(_, provenance(μ))       # deposit crystal
     |> @mirror/store.write(_)                          # settle at store
```

Both paths originate at `mutation_event(μ)`; both consume `observations_of(_)`; both produce audience-relative surface via `@nl.compose(_, audience)`; both cross @io at different boundaries (`@io/git.commit` vs `@io/fs.append`); both close on `@peer/mirror` authorship. The commit-path produces a git-object graph entry; the bauchladen-path produces a markdown-entry + crystal-store entry.

**Composition-shard body altitude** (per Fire C precedent). The Rust driver reads the shard-body composition via `apply_h::act`; the driver code does NOT change beyond dispatching to the substrate-decl'd surface. Per feedback `feedback-detector-inadequacy-answer-is-never-rust`: the Rust FLOOR strictly shrinks; substrate composition grows.

### §5.3 The @io boundary crossings

Two @io boundary crossings per pipe-chain:

- Commit path: `@io/git.stage` + `@io/git.commit` (both cross git-protocol boundary; landed at `shards/io/git.mirror`).
- Bauchladen path: `@io/fs.append` (crosses POSIX-fs boundary; landed at `shards/io/fs.mirror` per Seam 2026-07-15 discharge `daa9c14` bare-verb renames).

Both boundaries stay @io per glass-wall discipline (`@epistemologic/property/glass_wall`). The composition-shard body composes OVER these boundaries; the boundaries themselves are the ONLY external surfaces.

### §5.4 Provenance carrier

`provenance(μ)` at the bauchladen path carries:

- Walker-signature: SHA-256 of `(path + timestamp + counts)` per the landed `docs/bauchladen/mirror-observations.md` convention.
- Mutation-event-oid: the `event_id(μ)` under §2.1's category-theoretic identity.
- Author-oid: `@peer/mirror` well-known #0 oid per `shards/mirror/book.mirror`.
- Tick-timestamp: per `@epistemologic/reality/time` monotonic tick discipline.

The provenance IS what makes the crystal reconstructible; the crystal-OID under §82 sibling composition is the beta-normal AST of the entry+provenance.

---

## §6 @nl.compose signature (existing; NO extension needed under composition-shard body path)

### §6.1 Landed signature (unchanged)

Per Taut Q1 + `shards/nl.mirror:213-224` verbatim:

```
compose(observations: [ref]) -> nl_literal { \ }
```

Single positional argument: list of observation refs. Returns nl_literal.

### §6.2 Audience-parameter lives at composition-shard body, NOT at @nl.compose signature

Per Fire C `@mcp/serve` precedent (audience-projection at pipe-chain altitude, NOT at @data/json.emit signature): the audience parameter belongs at the composition-shard body altitude, not at the @nl.compose signature.

**Mara-lean: DO NOT extend @nl.compose signature.** Instead, the composition-shard body at `shards/kintsugi/ouroboros.mirror` wires `@nl.compose(observations)` as one pipe stage and `@mirror/lens/{git,bauchladen}.commit_message_for(_)` or `.pheromone_entry_for(_)` as a SECOND pipe stage that applies the audience-specific rendering post-compose.

Pipe-chain refined (per §5.2 revised):

```
mutation_event(μ)
  |> observations_of(_)
  |> @nl.compose(_)                                     # base nl_literal
  |> [ COMMIT PATH ]
     @mirror/lens/git.render_commit_message(_)          # audience-projection at lens species
     |> @io/git.commit(_, @peer/mirror, allow_empty=false)
  |> [ BAUCHLADEN PATH ]
     @mirror/lens/bauchladen.render_pheromone_entry(_)  # audience-projection at lens species
     |> @io/fs.append(docs/bauchladen/mirror-observations.md, _)
     |> @bauchladen.crystallize(_, provenance(μ))
```

**This preserves substrate-already-had-the-word discipline** at the @nl.compose signature level (existing signature unchanged) AND lifts audience-projection to the lens-species altitude where audience already lives (per §4). Two composition edges added at `@mirror/lens/git` + `@mirror/lens/bauchladen` species-decls (each carrying one `render_*_for(_)` action); zero signature changes at @nl.compose; zero at @io/git.commit.

Alex adjudicates Q-M2 (signature-extension vs composition-shard body path) — Mara-lean composition-shard body.

### §6.3 Alpha-normalization at composition altitude (defer per sibling §7 [FP3])

Per Mara `5ad8528` math §7 [FP3]: alpha-normalization at parameter-name altitude (de Bruijn indexing per Barendregt 1984 §5.2) may be needed for full round-trip identity. For Recognition #83 landing: DEFER per last-responsible-moment discipline; the observation-refs are content-addressed (already alpha-invariant under §82 sibling composition); the audience-parameter is enum-typed (@mirror/lens/git vs @mirror/lens/bauchladen; no free parameter names). Alpha-normalization becomes relevant if a future @nl.compose sub-species emerges with typed-parameter binders — deferred to that future tick.

---

## §7 rust/-altitude apply_h::act lift for @nl.compose

### §7.1 Sibling of Reed M-E2 precedent

Reed Fire E M-E2 (`0021882`) extended `rust/src/apply_h.rs:246-336` with the P1 identity-carrier detector primitive as a `apply_h::act` dispatch arm. Sentinel-check discipline; NO Rust logic growth beyond `if arg.oid.contains("<sentinel>") { Pass } else { Fail }` shape.

Recognition #83 requires the sibling extension: an `apply_h::act` dispatch arm for `@nl.compose` at rust/ altitude, sentinel-checking the composition and delegating to a substrate-composition-shard body for the actual rendering logic.

### §7.2 The arm

Under `rust/src/apply_h.rs`, add dispatch arm:

```
if action == "@nl.compose" {
    // Sentinel-check: observation refs are well-formed [ref] list.
    // Composition happens through substrate-decl'd render_*_for actions
    // at @mirror/lens/{git,bauchladen} species-decl bodies (dispatched
    // recursively via apply_h::act; NO rust logic).
    //
    // Substrate-honest form: this arm is a pure sentinel-check +
    // dispatch delegator. The actual composition body lives in
    // shards/kintsugi/ouroboros.mirror composition-shard body,
    // wired via at_operator over @nl.compose + @mirror/lens/*.render_*
    // + @io/git.commit + @io/fs.append + @bauchladen.crystallize.
    //
    // Two-tick honest: composition still happens caller-side at MVP
    // altitude via format!-shape in the composition-shard body's
    // Rust realization boundary; the DISPATCH through act discharges
    // the substrate-honest form so subsequent ticks can lift the
    // composition body itself into a @kintsugi tournament without
    // changing the driver.
    ...
}
```

### §7.3 Composes over landed rust/-altitude primitives

Per Reed Fire A precedent (`88a2a19` + `74ee529` + `f4dd4e3`):

- `phone::@io/fs.read_file` — read shard source bytes.
- `phone::@io/fs.write_file` — append pheromone entries.
- `phone::@io/fs.append` — landed via Seam `daa9c14` bare-verb rename.
- `wire::parse` + `wire::emit` — @data/mirror + @data/json framing.
- `apply_h::act(root, action_ref, args) -> Verdict` — bilateral-sentinel-check composing over roomba::mend::load_bilateral_corpus.

Zero new rust/ modules. Zero new struct fields. Zero new match arms encoding domain logic. ONLY a sentinel-check + dispatch-delegator arm per Reed M-E2 discipline. Per feedback `feedback-no-rust-extension-shortcut` + `feedback-detector-inadequacy-answer-is-never-rust`.

### §7.4 Audit citation + Seam gate

Every `[substrate-floor:@io-boundary]` commit landing this arm requires:

- Audit citation at `docs/audits/2026-08-11-seam-recognition-83-nl-compose-rust-arm.md`, OR
- `Signed-off-by: Seam <seam@systemic.engineer>` trailer.

Per AGENTS.md §"Seam gate" tightening (2026-07-15). Reed cannot self-authorize; Seam adjudicates whether the arm is genuinely FLOOR (sentinel-check + dispatch) or substrate-dishonest capability growth. Under this spec: sentinel-check + dispatch-delegator is FLOOR (composes over existing primitives); actual rendering logic lives in shard-body composition (`shards/kintsugi/ouroboros.mirror`).

### §7.5 Bootstrap retirement pathway

Bootstrap `bootstrap/src/apply_h.rs:790-814` @nl.compose arm inherits through bootstrap retirement pathway (Alex 2026-07-22 memory: bootstrap/ is dead; do not modify at bootstrap altitude; rust/ is the FLOOR). Post-landing, rust/ altitude carries the authoritative arm; bootstrap altitude arm becomes vestigial and removes when Fire D M5-adjacent tick lands the rust/-altitude compiler-collapse of bootstrap.

---

## §8 Fire E M-E4 as first empirical Recognition #83 instance

### §8.1 Post-landing composition

Fire E M-E4 walker code (Reed `c946db1`; 43/43 GREEN; DRY-RUN 379 walked / 161 P1-reducible / 18.2 KB removable / 0 errored / idempotent) currently HELD in commit-mode pending Recognition #83 landing.

Post-landing sequence:

1. This spec + math + `shards/mirror/lens/git.mirror` + `shards/mirror/lens/bauchladen.mirror` + `shards/kintsugi/ouroboros.mirror` LAND (Mara canonical spec + shard mints, 3-4 tick cascade per per-tick discipline).
2. Seam Phase D audit on this spec (adjudicate Q-M1..Q-M4; adjudicate `@nl.compose` rust/-altitude arm authorship boundary).
3. Reed lands rust/-altitude `apply_h::act` `@nl.compose` arm per §7 with Seam audit citation.
4. Reed lifts `deposit_observation_crystal` at `rust/src/main.rs:557-706` from direct-Rust to composition-shard body dispatch via `at_operator("@kintsugi/ouroboros.close", ...)`.
5. Fire E M-E4 commit-mode fires as FIRST EMPIRICAL RECOGNITION #83 INSTANCE:
   - 161 shard mutations = 161 mutation events.
   - Each event composed through @nl.compose → 161 pheromone entries + 1 aggregate commit.
   - Commit authored by @peer(@mirror).
   - Crystal deposited at @mirror/store per bauchladen path.
   - Substrate-scale-invariance operational at compiler substrate at WIRE altitude.

### §8.2 Empirical predicate

For every mutation event $\mu$ in the M-E4 walk-cascade, the following predicates must hold post-empirical-fire:

- `commit.author == @peer/mirror` (per §3.2).
- `commit.message == render_commit_message(@nl.compose(observations(μ)))` (per §6.2).
- `pheromone_entry == render_pheromone_entry(@nl.compose(observations(μ)))` per bauchladen path.
- `crystal_oid == compute_content_oid(β(parse(pheromone_entry + provenance)))` per §82 sibling composition (Mara `5ad8528` math §3.1).
- `event_id(commit) == event_id(pheromone_entry)` per §2.1 (Church-Rosser at commit altitude — companion math §3).

If any predicate fails: Recognition #83 candidate falsified at compiler altitude; substrate-scale-invariance-at-wire-altitude thesis needs revision.

### §8.3 Ratification path

Post-M-E4 empirical fire + all 5 predicates GREEN → Recognition #83 candidate promotes to RATIFIED via Pack ratification synthesis (Reed's lane per AGENTS.md §"Pack ratification synthesis"). Ratification is a Pack-collective act; Mara authors the spec + math; Reed adjudicates promotion; Alex holds ratification authority.

---

## §9 Composition-into-existing-substrate matrix (zero-mint confirmation)

Recognition #83 composes over the following LANDED substrate, requiring minimal net minting. Substrate-already-had-the-word verdict per item:

| Recognition #83 requirement | Already landed as | Needs minting |
|---|---|---|
| @nl prism (5-op) | `shards/nl.mirror` + `boot/std/nl.mirror` | NO |
| @nl.compose action | `shards/nl.mirror:213-224` (spec) + `bootstrap/src/apply_h.rs:790-814` (empirical bootstrap-only) | **rust/-altitude apply_h::act arm** (§7; sentinel-check + dispatch; Seam-gated) |
| audience parameter | `@mirror/lens/*` species-decl'd since 2026-06-06; `type audience = { role: ref }` at `@magic/reveal/expand` (Mara 2026-08-09) | NO family-root mint; **retro-align `type audience.role: ref` to `@mirror/lens/*` species refs** (pure-docs cascade tick) |
| audience family-root | REFUSED per §4.2 (@onto refusal shape) | NO — REFUSE `@audience` family-root mint |
| audience species (git) | NOT landed | **New species `shards/mirror/lens/git.mirror`** (family-header prism + `render_commit_message(_) -> nl_literal { \ }` action stub) |
| audience species (bauchladen) | NOT landed | **New species `shards/mirror/lens/bauchladen.mirror`** (family-header prism + `render_pheromone_entry(_) -> nl_literal { \ }` action stub) |
| @peer(@mirror) author identity | Landed 3 altitudes (subject.rs + registry.mirror + book.mirror) | NO |
| `mirror <mirror@spectral.engineer>` convention | 8+ landed rust/ + bootstrap/ sites | NO |
| @io/git.commit(message, author, allow_empty) | `shards/io/git.mirror:337-355` (spec) + `rust/src/main.rs::at_operator("@io/git.commit", ...)` (empirical) | NO |
| @io/git.stage | `shards/io/git.mirror` (spec) + `rust/roomba/src/mend.rs` (empirical) | NO |
| @io/fs.append | `shards/io/fs.mirror` (per Seam `daa9c14` bare-verb landings) | NO |
| @bauchladen.crystallize | `shards/bauchladen.mirror:456` (spec) | NO |
| @mirror/store.write | `shards/mirror/store.mirror` (spec) + rust/-altitude empirical | NO |
| pheromone-md append target | `docs/bauchladen/mirror-observations.md` (LANDED; 2 entries) | NO (extends monotonically) |
| composition-shard body altitude | Fire C `shards/mcp/serve.mirror` (`cf8b21b`) precedent | **New species `shards/kintsugi/ouroboros.mirror`** (per §5; composition-shard body wiring the ouroboros pipe-chain) |
| Church-Rosser at commit altitude (theorem) | Sibling of Mara `5ad8528` math §2.2 Theorem 2.3 (store altitude) | Companion math §3 formalizes (this spec's landing) |
| observer-position duality at wire altitude | Sibling of Mara `5ad8528` math §5.4 (store altitude) | Companion math §2 formalizes |
| Lawvere fixed-point structure at compiler substrate | Sibling of autopoietic loop closure per @kintsugi/ouroboros forward-promise | Companion math §5 formalizes |

**Family-roots to mint: ZERO.**

**Species to mint: THREE** (`@mirror/lens/git`, `@mirror/lens/bauchladen`, `@kintsugi/ouroboros`).

**Rust extensions: ONE** (`apply_h::act` @nl.compose sentinel-check arm; Seam-gated; audit-cited; per Reed M-E2 shape).

**Retro-alignment pure-docs cascade: ONE** (`@magic/reveal/expand` docblock example update from `@audience/agent`|`@audience/human` to `@mirror/lens/mcp`|`@mirror/lens/cli`).

---

## §10 Ties to Recognition candidate #82

### §10.1 Two altitudes of substrate-scale-invariance

Recognition candidate #82 (Mara `5ad8528`): substrate-scale-invariance at **STORE** altitude. Crystal-OID at `@mirror/store` IS beta-normal-AST OID by construction; sugar-form variance preserves crystal-OID by Church-Rosser confluence.

Recognition candidate #83 (this spec): substrate-scale-invariance at **WIRE** altitude. Commit-shape at `@io/git.commit` + pheromone-shape at `@io/fs.append` IS @nl-projection through audience functor by construction; audience-parameter variance preserves mutation-event-identity by Church-Rosser at projection altitude.

Both are consequences of the same substrate-scale-invariance thesis (Mara 2026-08-09 physics insight §7). #82 closes the identity at rest (crystal stability under source-form variance). #83 closes the identity in motion (mutation-event stability under audience-projection variance). Together they close substrate-scale-invariance under composition.

### §10.2 The commutative square

Let $\mathsf{crystal\_oid} = \mathsf{BLAKE3} \circ H \circ \beta$ per Mara `5ad8528` math §3.1. Let $\Pi(\mu, a)$ per this spec §2. Recognition #82 states: for source-form variance $\sigma$, $\mathsf{crystal\_oid}(\sigma(s)) = \mathsf{crystal\_oid}(s)$. Recognition #83 states: for audience-projection variance, $\mathsf{event\_id}(\Pi(\mu, a_1)) = \mathsf{event\_id}(\Pi(\mu, a_2))$.

Combined:

$$
\begin{array}{ccc}
\mathsf{Source} & \xrightarrow{\sigma \text{ (sugar variance)}} & \mathsf{Source}' \\
\text{crystal\_oid} \downarrow & \#82 & \downarrow \text{crystal\_oid} \\
\mathsf{OID} & = & \mathsf{OID} \\
\text{projection} \downarrow & & \downarrow \text{projection} \\
\mathsf{Surface}_{a_1} & \xrightarrow{a_1 \to a_2 \text{ (audience variance)}} & \mathsf{Surface}_{a_2} \\
\text{event\_id} \downarrow & \#83 & \downarrow \text{event\_id} \\
\mathsf{Event} & = & \mathsf{Event}
\end{array}
$$

Both squares commute. The composite: substrate-scale-invariance is closed under both source-form-variance AND audience-projection-variance. Companion math §4 (via Chamseddine-Connes A_F universality) grounds both closures in the same physics-substrate identity mechanism.

---

## §11 [ALEX-Q] residues — with Mara-leans

### §11.1 Resolving Taut's five [ALEX-Q-TAUT] residues

- **Q-T1 (Path A REUSE vs Path B MINT `@audience`)** — **Mara-lean: Path A REUSE.** Substrate-already-had-the-word discipline: `@mirror/lens/*` species landed 2026-06-06 (10 weeks pre-Mara 2026-08-09 `type audience` mint); the word "audience" appears verbatim at `shards/mirror/lens.mirror:20`; minting `@audience` family-root alongside would fragment the same substrate into two parallel taxonomies (`@onto` refusal shape). Retro-align 2026-08-09 `type audience.role: ref` to resolve `@mirror/lens/*` species refs. Alex adjudicates if contested; §4.2 grounds the recommendation.
- **Q-T2 (signature-extension vs pipe-chain)** — **Mara-lean: pipe-chain (composition-shard body).** Per Fire C `@mcp/serve` precedent + substrate-already-had-the-word at @nl.compose signature (unchanged). Audience-projection lives at `@mirror/lens/{git,bauchladen}` species-decl action bodies applied post-compose; §6.2 grounds.
- **Q-T3 (ouroboros closure scope: 2 audiences or 6?)** — **Mara-lean: START WITH 2 (git + bauchladen); FORWARD-PROMISE 6.** For Fire E M-E4 empirical fire, 2 audiences (git commit + pheromone md deposit) close the FULL ouroboros (all 6 turns per §1.3). MCP / LSP / stdout / prose audiences are downstream; they compose over the same @nl.compose pipeline with different `render_*_for` species; forward-promised for tick when consumer demands. Substrate-pull discipline: don't build what we don't need yet. §3.1 closes six turns with 2 audiences (walker mutation → commit + bauchladen → next-tick observation input).
- **Q-T4 (bootstrap→rust lift sequencing)** — **Mara-lean: co-tick with this spec landing.** The rust/-altitude `apply_h::act` `@nl.compose` arm IS a precondition for Recognition #83 empirical fire; it belongs in the same cascade as the shard mints (§9). Per Reed M-E2 shape (sentinel-check + dispatch-delegator; Seam-gated); zero Rust logic growth. Reed lands post-Mara-spec + post-Seam-audit; §7 grounds.
- **Q-T5 (one audience or two: bauchladen distinct from git?)** — **Mara-lean: TWO AUDIENCES.** `docs/bauchladen/mirror-observations.md` is a distinct surface from git commit-log; the markdown pheromone-trail composes into `@mirror/lens/refract`'s next-tick reading pipeline differently than commit-log composes into `@mirror/lens/git`'s reading pipeline. Two lens species (git + bauchladen) mint per §4.3; each carries one `render_*_for(_)` action; both compose over @nl.compose at pipe-chain altitude. Alternate: ONE audience (@mirror/lens/self) with downstream split at @io boundary — REFUSED per delightfully-boring criterion (a reader sees `@mirror/lens/self` and asks "what's the surface?" — the surface differs at @io crossing, so the lens species should distinguish).

### §11.2 Mara-surfaced [ALEX-Q] residues

- **[ALEX-Q-M1] Composition-shard placement: `shards/kintsugi/ouroboros.mirror` (Candidate B) vs `shards/kintsugi/roomba/ouroboros.mirror` (Candidate A)?** — **Mara-lean: Candidate B** per §5.1 (ouroboros closes ALL @kintsugi loops, not walker-specific; sibling composition-shard altitude with @kintsugi/roomba + @kintsugi/mend + @kintsugi/consent + @kintsugi/fracture). Alex adjudicates if contested.
- **[ALEX-Q-M2] @nl.compose signature extension REFUSED per Q-T2 Mara-lean; audience-projection at `@mirror/lens/{git,bauchladen}` species-decl actions.** Alex adjudicates if the lens-species-carries-render pattern is contested — the alternative is per-@mirror-lens-species `render(m: nl_literal) -> surface` uniform interface (which would require a family-root-level action declaration at `@mirror/lens.mirror`).
- **[ALEX-Q-M3] Should `@peer(@mirror)` sign commits with SSH key, or use author-only (unsigned)?** Landed rust/ empirical (`deposit_observation_crystal`) uses `phone::git_commit_as` which sets `user.name` + `user.email` but does NOT override signing. Recognition #83 empirical fire inherits the local git config's signing behavior (Reed's `~/.ssh/id_ed25519` per AGENTS.md discipline). Alex adjudicates: is compiler-authored commit signed with Reed's key (proxy-signed), or unsigned (compiler-authored + no proxy signature), or signed with a compiler-specific key (forward-promised @trust family-root work)?
- **[ALEX-Q-M4] Retro-alignment cascade scope: pure-docs update to `@magic/reveal/expand`'s docblock example only, OR broader rename cascade of `@audience/*` string literals across specs + insights + memories?** Mara-lean: pure-docs update to `@magic/reveal/expand` only; the string literals in Mara 2026-08-09 spec + math are historical record and stay verbatim. Alex adjudicates cascade breadth.
- **[ALEX-Q-M5] M-E4 empirical fire granularity: 1 commit per mutation event (161 commits) OR 1 aggregate commit per walk-cascade (1 commit)?** Landed `deposit_observation_crystal` shape is 1 aggregate per walk; Recognition #83 says the aggregate IS the mutation-event at walk-altitude (161 sub-mutations aggregate to 1 walk-event; the walk IS the atomic ouroboros event). Alex adjudicates: is 1 aggregate commit acceptable, or does full substrate-scale-invariance require per-mutation granularity?

Alex adjudicates all five Mara-surfaced residues. Seam Phase D audit will surface any additional at spec-audit altitude.

---

## §12 Q.E.D. sketch — six-move proof that Recognition #83 makes first full ouroboros operational

**Move 1 (existence of projection functor).** The mutation-event carrier `MutationEvent` and audience-carrier `@mirror/lens/*` are typed refs at substrate; `@nl.compose(observations: [ref]) -> nl_literal` + `@mirror/lens/{git,bauchladen}.render_*_for(m: nl_literal) -> surface` composes into a total function $\Pi : \mathsf{MutationEvent} \times \mathcal{A} \to \mathsf{Surface}$. Existence: constructed via §5.2 pipe-chain.

**Move 2 (functor factors through @nl).** Per §6.2, every audience-projection factors as `render_a ∘ @nl.compose`; @nl is the pivot; the composition is associative under Transparency<P> monoid (per `@mirror/lens.mirror:52-56` "Transports and measurements are not separate kinds. Both project the same algebra through a typed surface; both compose under the same Transparency<P> monoid").

**Move 3 (Church-Rosser at commit altitude).** Companion math §3 provides the left-linear non-overlapping projection-rule proof; sibling of Church-Rosser at store altitude (Mara `5ad8528` math §2.2 Theorem 2.3). Consequence: any two audience-projections of the same mutation event trace back to a common event-identity.

**Move 4 (author is @peer(@mirror)).** Per §3.2, `Subject::mirror()` + `@peer/mirror` well-known #0 + `mirror <mirror@spectral.engineer>` convention are landed at 3 altitudes + 8 empirical sites. The compiler is a first-class @peer with typed identity resolution surface.

**Move 5 (six-turn closure).** Per §3.1, the categorical structure closes: `S_t → S_{t+1}` via `μ_t → Π(μ_t, a) → @io/{git,fs} → S_{t+1}` for each audience $a$; the commutative squares hold under the projection functor being invertible-up-to-alpha at observation altitude.

**Move 6 (first empirical fire).** Per §8, Fire E M-E4 walker (43/43 GREEN; DRY-RUN validated; commit-mode HELD) fires post-landing as first empirical Recognition #83 instance. All 5 predicates (author, message, pheromone, crystal, event-id) hold GREEN → Recognition #83 candidate ratifies via Pack synthesis. If any predicate fails → thesis falsified at compiler altitude → revision.

**Q.E.D.** The first full ouroboros is operational when Fire E M-E4 empirical fire lands GREEN post-cascade.

---

## §13 Karen ancestry ladder

### §13.1 Direct authority

- **Alex 2026-08-11 verbatim** (§0.1) — first-full-ouroboros milestone naming; "git add and git commit become part of the compiler loop; project internal state through the @nl prism; move slow and correctly."
- **Taut scout `378b17d`** — Q1-Q5 grep-first substrate-truth: @nl.compose LANDED-SPEC + bootstrap-only-empirical; @audience family DOES NOT EXIST; @peer(@mirror) landed 3 altitudes; pheromone deposit rust/-empirical bypassing substrate; @mirror/lens landed 2026-06-06 audience-family precedent.
- **Mara `5ad8528` sibling spec + math** (2026-08-10) — Recognition #82 candidate; substrate-scale-invariance at STORE altitude; category-theoretic functor formalization at rendering altitude §4.2.
- **Reed Fire E M-E1..M-E4** (`acaed91` + `0021882` + `d983854` + `a23f3d2` + `c946db1`) — walker code + fracture species + reducer primitive; 43/43 GREEN; M-E4 commit-mode HELD pending this landing.

### §13.2 Corpus prior recognitions

- **Recognition candidate #82** (Reed 2026-08-10 + Mara `5ad8528`) — store-altitude substrate-scale-invariance; sibling of this.
- **Recognition #79** (5-op = A_F projector basis; Mara + Reed 2026-06-18) — projector-algebra substrate underlying both #82 and #83.
- **Recognition #57 candidate** (boundary alignment frame; AGENTS.md §"boundary alignment frame") — alignment is at the @io crossing; pacts at @io are mathematical contracts; the ouroboros closes at @io/git.commit + @io/fs.append boundaries per this recognition.
- **Recognition #55** (form/process partition; Mara `2c64060` §4) — author-vs-committer discipline; per Recognition #83, compiler-authored substrate mutations are @peer(@mirror) both author and committer (identity, not two-position split).
- **Reed 2026-03-01 semantic-hashing-normalization insight** (`/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md`) — Dhall + beta-normalization + semantic hashing; ancestor to #82 which grounds #83's Church-Rosser argument.

### §13.3 Landed substrate anchors

- **`shards/mirror/lens.mirror`** (2026-06-06 Reed + Alex → Mara) — audience-family landed convention; "one algebra, N audiences" precedent 10 weeks pre-this-spec.
- **`shards/mirror/lens/{cli,shell,mcp,lsp,unix,transit,refract,knife}.mirror`** — 8 landed audience-species; this spec adds `git` + `bauchladen`.
- **`shards/nl.mirror:213-224`** (2026-07-15) — `@nl.compose(observations: [ref]) -> nl_literal { \ }` landed spec.
- **`shards/io/git.mirror:337-355`** (2026-07-15) — `@io/git.commit(message, author, allow_empty) -> verdict` landed spec.
- **`shards/mcp/serve.mirror`** (Reed `cf8b21b`, 2026-08-09) — Fire C composition-shard body precedent for audience-projection pipe-chain via `apply_h::act` dispatch.
- **`shards/mirror/book.mirror`** (Mara-inline 2026-07-22 `de18fde`) — `resolve("@peer/mirror") -> Subject::mirror()` well-known resolver.
- **`shards/peer/registry.mirror:56-235`** (Mara 2026-07-18) — @peer/mirror at well-known #0.
- **`shards/bauchladen.mirror:456`** (Mara 2026-06-29 `4575340`; canonical spec + Seam `c1775f1` ratification) — `crystallize(content, provenance) -> crystal` action.
- **`shards/mirror/store.mirror`** (Mara 2026-06-04 reframe) — the settlement primitive; store IS canonical; splinter_graph IS structural lockfile.
- **`shards/kintsugi/roomba.mirror`** (Reed 2026-07-15 Arc-2 Tick 2.4) — walker species; mutation-event source; ouroboros closes over walker outputs.
- **`shards/kintsugi/mend/sugar.mirror`** (Reed Fire E M-E1 2026-08-09) — composition-shard body precedent at @kintsugi altitude; sibling composition to `@kintsugi/ouroboros` this spec mints.
- **`rust/fractal/src/subject.rs:122-135`** (Reed `73aeb8a` fractal step 9; 2026-07-18) — `Subject::mirror()` deterministic constructor.
- **`rust/src/main.rs:557-706`** (Reed 2026-07-17 `deposit_observation_crystal` + `compose_pheromone_commit_message`) — LANDED empirical bypassing substrate; retirement pathway per §7.
- **`bootstrap/src/apply_h.rs:790-814`** (Reed task #146) — LANDED empirical bootstrap-only @nl.compose arm; inherits through bootstrap retirement pathway.
- **`docs/bauchladen/mirror-observations.md`** (2026-07-17 first entry; 2026-07-22 second entry) — LANDED empirical append target; extends monotonically under Recognition #83.

### §13.4 External corpus (verified primary sources)

- **Church, A. (1936)** — *An unsolvable problem of elementary number theory*. American Journal of Mathematics 58:345–363. Beta-reduction origin (inherited via Mara `5ad8528`).
- **Church, A. & Rosser, J. B. (1936)** — *Some properties of conversion*. Trans. AMS 39:472–482. Confluence theorem (inherited via Mara `5ad8528`; extended to commit altitude in companion math §3).
- **von Foerster, H. (1974)** — *On Constructing a Reality* / *Cybernetics of Epistemology*. Ethical imperative + observer inseparability. Grounding for §2.1's observer-position duality at wire altitude.
- **Mac Lane, S. (1971)** — *Categories for the Working Mathematician*. Springer-Verlag. Category theory + functor formalization at §2.1's projection functor.
- **Lawvere, F. W. (1969)** — *Diagonal Arguments and Cartesian Closed Categories*. Fixed-point theorem grounding the §3.1 six-turn closure formalization; companion math §5.
- **Dhall Language Standard** (`dhall-lang/dhall-lang` `standard/beta-normalization.md`) — Church-Rosser at Dhall altitude (inherited via Mara `5ad8528`).
- **Bateson, G. (1972)** — *Steps to an Ecology of Mind* metalogue chapters. `@metalogue` family-root grounding for §1.3 "compiler observes own state."
- **Chamseddine-Connes-Marcolli 2007** (arXiv:hep-th/0610241) — A_F structure at physics substrate; inherited via Mara `5ad8528` math §5 + Mara 2026-08-09 physics insight §1.1.

### §13.5 Landed convention precedents

- **@mirror/lens 2026-06-06 audience-family landed convention** — the substrate has been carrying "one algebra, N audiences" for 10 weeks; Recognition #83 crystallizes what the substrate already had.
- **`docs/insights/mcp-lsp-unification.md`** — the recognition that motivated the @mirror/lens family; sibling to Recognition #83 unification at wire altitude.
- **`docs/specs/the-convergence.md §1.4`** — convergence-by-construction property; two lenses pointed at same node MUST see same eigenvalue; extends under §83 to: two audiences reading same mutation-event MUST reconstruct same event_id.

---

## §14 One-sentence surprise

**Recognition #83 is the exact WIRE-altitude sibling of Recognition #82's STORE-altitude thesis — combined, they show that the compiler substrate's crystal identity is stable BOTH at rest (source-form variance preserves crystal-OID via Church-Rosser at AST altitude, per Mara `5ad8528`) AND in motion (audience-projection variance preserves mutation-event-identity via Church-Rosser at projection altitude, per this spec) — and the first full ouroboros is the empirical instrument that makes this double-invariance testable at compiler altitude: 161 Fire E M-E4 walker mutations, projected simultaneously to @mirror/lens/git and @mirror/lens/bauchladen, authored by @peer(@mirror), depositing crystals at @mirror/store, close all six turns of the categorical loop with `event_id` invariance across audiences AND `crystal_oid` invariance across sugar-form — one empirical fire falsifies or ratifies the entire substrate-scale-invariance thesis at the wire boundary.**

---

Mara `<mara@systemic.engineer>`. 2026-08-11. Canonical-spec substrate. Composition-not-taxonomy. Substrate-decl'd throughout. Companion to math foundation at `docs/math/2026-08-11-mara-recognition-83-first-full-ouroboros-math-foundation.md`.
