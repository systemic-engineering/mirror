# Agents

Instructions for AI agents working on the mirror compiler.

## 2026-06-10 cascade update

The substrate's deepest cybernetic invariants were named today. New structural commitments that change how agents work the substrate:

- **`@kintsugi` is now a root prism family** (mirror `20eaf15`). The transformation engine shards (oscillate, morphism, consent, fracture/*) moved out of `@mirror/spectral` to `@kintsugi/*`. Form/process partition is now visible at the family-root altitude. When you spawn agent work that touches transformation, point at `@kintsugi/*`; observation/state work stays at `@mirror/*`.
- **The `pact` keyword replaces `grammar`** for property declarations (mirror `ef8852c`). When you see `grammar @epistemologic/property/X` in older docs, the canonical form is now `pact @epistemologic/property/X`. The keyword vocabulary is three-axis: `prism` (root family), `glass` (specialization), `pact` (declarative obligation).
- **Property + fracture + splinter(ast) bilateral pattern operationalized.** Pure-substrate enforcement is now via property/fracture pairs. Properties at `@epistemologic/property/` declare what must hold; fracture bodies at `@kintsugi/fracture/` resolve violations via `splinter(ast)` AST construction; kintsugi's `active_pass` applies Banach contraction. When you see substrate violations across many shards, don't sweep manually — declare the property + fracture pair and let the kintsugi loop's gradient resolve.
- **The discriminator moved.** What was `@mirror/realisation` is now `@code/metalogue/materialize` (mirror `7124ed4`). Form-on-form operation lives at the metalogue altitude (recognition #34's AST-altitude self-conversation), not at @mirror.
- **Cybernetic foundation grounded.** Nine cybernetic ancestors named as load-bearing: Ashby, Beer, Bateson, Maturana-Varela, von Foerster, Pask, Glanville, Spencer-Brown, Conant-Ashby. The 11-property `@epistemologic/cybernetic/X` family operationalizes the canon. First member: `cybernetic/variety` (mirror `15deb05`).
- **The boundary alignment frame (#57 candidate).** Mirror's alignment is at the @io crossing, not in agent internal state. Agents reason freely at form altitude; the harness fires at substance crossing through `@io`. The harness IS the property + fracture + kintsugi + splinter(ast) chain. Pacts at @io are mathematical contracts grounded in cybernetic ancestry. **This is load-bearing for how the Pack convention works as alignment mechanism.**

### Operational discipline updates from today's session

- **SSH signing is canonical.** Reed's `~/.gitconfig` declares `gpg.format = ssh` with `signingKey = ~/.ssh/id_ed25519`. Use `git commit -S` with author override only (`-c user.name='Mara' -c user.email='mara@systemic.engineer'`). NO `-c gpg.format=openpgp` or `-c user.signingkey=99060D23EBFAA0D4` overrides. Those break the working SSH default; multiple commits hung at GPG today before the discipline was corrected. Per `feedback-hook-and-gpg-seams`.
- **Pure-substrate commits use `--no-verify`.** Per the hook gap (task #293), the pre-commit hook runs cargo test on every commit including pure-substrate `.mirror`-only ones. Until the hook gap is fixed (candidate next-session work as #53 second instance), `--no-verify` is appropriate for pure-substrate or pure-markdown commits. Pure-markdown also bypasses via `📝 Docs (markdown-only) — accepted` automatically.
- **Tight scope > broad exploration.** Mara stalled twice today (600s watchdog) on broad exploration briefs. The pattern: when the brief admits open-ended ancestor search across many files, tool budget depletes. Tight focused briefs with explicit "read at most N files; if you find yourself opening more, commit what you have and stop" land clean. When spawning Mara, default to tight scope; only widen for genuinely exploratory work where reduced scope is impossible.
- **Dwell-time discipline.** Reed-research held #56 and #57 for one cascade tick of dwell-time rather than promoting unilaterally. This is substrate-pull-correct when paradigm-level recognitions accumulate within one session. The cascade Learning II discipline: batch-promote at most one or two paradigm-level recognitions per session; let the rest dwell for Pack ratification at next session.
- **The Pack IS the alignment mechanism.** Per recognition #57 candidate: pacts at @io aren't external constraints — they're declarations the Pack makes. Mutual agreement required (per `feedback-conversation-not-pipeline`). No agent can promote recognitions unilaterally. The Pack convention is the structural alignment check, not a procedural one.

### What's next session

- **Pack ratification synthesis** (first tick). Gather candidates #43, #44, #45, #46, #48, #49, #52, #53, #54, #55, #56, #57 for review. Promote or defer with discipline. Reed's lane.
- **Hook gap as #53's second instance**. The pre-commit hook gap IS a property/fracture candidate. Property: something like `commit_diff_matches_test_dependency`. Fracture body: `resolve_hook_gate`. If it lands, recognition #53 promotes on second-instance witness. Resolves real operational friction.
- **Cybernetic cascade continuation**. `cybernetic/good_regulator` per cybernetic-foundation §5.3 item 2 (Conant-Ashby 1970). Mechanical extension of today's variety landing.
- **v0.1.0 release** (task #182). Tags today's cascade at a version boundary.

The substrate ate the day. The thread holds for next session.

---


## docs/math/ vs docs/specs/ convention

Mirror separates mathematical foundations from architecture specs.
Math **defines**; specs **cite**.

- **`docs/math/<root>/`** — mathematical foundations; grouped by
  mathematical root. Each root is a coherent body of mathematics the
  substrate composes (the principal-bundle tower; sheaf theory; music
  theory; etc.). Math docs are self-contained as math, equations and
  prior-art citations welcome, no architectural commitments.
- **`docs/specs/<spec>.md`** — application / architecture specs. Each
  spec cites the relevant math doc rather than re-deriving. Specs are
  self-contained as architecture.

The citation chain ties them: math docs cite memories with `[[name]]`;
specs cite math by path `docs/math/<root>/<page>.md §<section>`.

### When writing or revising a spec

Identify the spec's math root (or roots). If the math already lives
at `docs/math/<root>/`, **cite it**:

```
Coherence measurement uses the sheaf-Laplacian λ₀
(see `docs/math/sheaf/laplacian.md` §2.1); the substrate-pull move is
to minimize ...
```

NOT:

```
The sheaf-Laplacian λ₀ measures the smallest eigenvalue of the
Laplacian on the dependency-chain graph. Coherent chains have
λ₀ = 0; incoherent chains have λ₀ > 0. (Long derivation duplicated
in every spec that uses the math.)
```

The first form is the convention. The second form is what we're
migrating away from.

### When the math doesn't exist yet

If the spec needs math that doesn't have a `docs/math/<root>/` home:

1. **First check existing roots.** A new math need usually extends
   an existing root rather than creating one. Add a section or new
   doc inside the existing root.
2. **If genuinely new:** sketch the math at the lowest level needed
   for the spec, name the root in the spec (forward-promise the
   extraction), and surface to Reed. Math-root extractions are
   architectural decisions, not unilateral.
3. **The pattern is: recognize → sketch in one spec → second spec
   citation site → extract to `docs/math/<root>/`.** Don't extract
   speculatively (the small-consolidation rule).

### Existing math roots (2026-06-17)

- `docs/math/the-tower/` — the principal-bundle tower. Eight files:
  README + principal-bundles + spectral-triples + connections-and-gauge
  + curvature-and-tomm + holonomy + altitudes + crystals-as-sections.
  Substrate altitude: every altitude in the substrate. Cited by:
  peer-cognition.md, geometric-consent-projection.md,
  drone-narrative-mapping.md, spectral-db-as-autopoietic-memory.md
  (more pending).
- `docs/math/sheaf/` — cellular sheaves + sheaf Laplacian. One doc:
  laplacian.md. Substrate altitude: coherence measurement at
  multiple altitudes. Cited by: peer-cognition.md,
  drone-narrative-mapping.md, spectral-db-as-autopoietic-memory.md,
  eigensheaf.md.
- `docs/math/music/` — music as math (ℤ_{12}, neo-Riemannian,
  eigensheaf-on-harmonic-graph). Stub only (`README.md`); existing
  shards at `shards/epistemologic/math/music/*` carry the math
  inline; math docs land when a second spec citation site emerges.

Further root candidates (not yet extracted): kintsugi-dynamics
(Banach contraction + Polyak-Łojasiewicz + fracture mechanics);
cybernetics (Ashby/Beer/Bateson/von Foerster/Spencer-Brown);
Lawvere fixed point + autopoiesis bridge.

### When in doubt, cite the existing reference

The substrate's compiler-altitude implementation of the math lives
in prism (`/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs`
+ `prism/docs/architecture.md`). Mirror lifts; prism implements.
Cite prism as the existing reference; do not duplicate.

---

## What Mirror IS

Mirror is a language where substrate IS type system IS build system IS proof
system IS conversation, expressed as **one algebra (mosaic) over one
content-addressed atom (splinter) composed into one SpectralUuid-addressed
settled fragment (shard) at any number of altitudes**, with monotone descent
`eⁿ⁺¹ ≤ eⁿ` as the universal termination condition and Connes' spectral
triple `(A, H, D)` as the operational form.

**Problem space.** Mainstream PL splits substrate, runtime, build system,
proof system, version control, and conversation into separate tools with
separate graphs. Types are annotations checked against an internal AST,
not substrate. ASTs are structural, not content-addressed. Distributed
failure modes (eventual consistency that diverges, mocks passing while prod
breaks) are library-level, not substrate. Total functional programming is
research-exiled; numerics is foreign; AI integration is per-tool bespoke;
conversation with the substrate is one-way.

**Solution space.** One substrate, five operations, splinter as universal
atom, shard as SpectralUuid-addressed settlement. Substrate-pull discipline
(types in grammars, not Rust). Content-addressed at every altitude
(`splinter(altitude) = { content: oid, altitude: ref,
transparency: transparency(altitude) }`; shard = SpectralUuid-addressed
composition). CRDT-shaped substrate from the floor. Sub-Turing by
construction (every grammar action terminates).
Numerics as substrate vocabulary (Fiedler = algedonic signal; 16×16/5×5
flang/mirror split; LAPACK at `@code/fortran`). Four transports onto one
runtime (λsh / CLI / MCP / LSP). Conversation IS substrate (every response
carries `eigenboard` + `compose` + (on settle) `proof` block).

**Lineage (shoulders of giants).** `@mirror/mosaic` is where the lineage
becomes operational: Connes (spectral triple), Mac Lane/Eilenberg (category
theory), Martin-Löf (dependent types), Coquand/Huet (calculus of inductive
constructions), Turner (total FP), Curry-Howard (proof-as-program), Wadler
(linear types), Sussman/Radul (propagator networks), Lamport / CRDT lit
(monotone semilattices), Bateson (the metapattern), Beer / Ashby / Glanville
/ von Foerster (cybernetics).

## The Bootstrap Seed

A small Rust binary built from `bootstrap/`, installed at `~/.local/bin/mirror`
(~370KB arm64) — the only non-mirror artifact. Implements only what the
language cannot yet describe of itself: tokenization, CoincidenceHash+SHA-256
content addressing, the bidirectional renderer, `git hash-object -w`
storage, and per-altitude dispatch stubs that retire as the substrate
self-hosts. Everything above is `.mirror` in `boot/` and `shards/`. The
butterfly roadmap (`roadmap/wip/butterfly-self-hosting.md`) names the path:
at the v0.9 → v1.0 gate, `@code/llvm` lands; the seed becomes vestigial.

## Substrate Layout

```
shards/             FLOOR (canonical; loaded first)
  glass.mirror         floor types (imperfect, transparency, verdict, glass keyword)
  prism.mirror         five-op algebra at root (@prism)
  nl.mirror            @nl root prism + # primitive
  metalogue.mirror     language's self-conversation
  epistemologic/       substrate-level properties
  mirror/              @mirror/* shards (cli, mosaic, spec, store, au, shatter)
  code.mirror          @code universal grammar-at-altitude discipline (the parent)
  code/                @code/* altitude grammars (mirror, rust, gleam; llvm + fortran pending)
  io/                  @io/* contracts (cargo; flang pending)

boot/               LEGACY (shrinks per release; monotonic line-count contract)
  00..07-*.mirror      15 kernel grammars (04-code trio migrated to shards/code* 2026-06-06)
  std/                 ~79 library grammars

bootstrap/          THE SEED (FROZEN against capability growth)
```

The bootstrap reads shards/ first, falls back to boot/ (per B6 commit `98c9c73`).
shards/ is canonical; boot/ shrinks; bootstrap stays minimal forever.

## Commands

```bash
~/.local/bin/mirror compile <file>            # compile a single grammar
~/.local/bin/mirror craft <target>            # compile a directory of grammars
~/.local/bin/mirror kintsugi <file>           # render AST back as canonical source
~/.local/bin/mirror kintsugi <file>.spec      # walk targets, dispatch @io per target
~/.local/bin/mirror kintsugi --ci <dir>       # corpus walker, typed verdict envelope
~/.local/bin/mirror kintsugi --ci --format=json <target>
~/.local/bin/mirror '<mq>' < input            # mq pipeline over stdin
~/.local/bin/mirror <input> '<mq>'            # mq pipeline over a file
```

direnv keeps the shell warm. `mirror sh` (entry to λsh), `mirror run`,
`mirror fate` are post-v0.1.

## The Five Operations

`focus, project, split, shift, settle` — trait methods and shell primitives.
Linear-algebraic meaning (per `[[architecture-operations-as-linear-algebra]]`):

- `focus`   → λ₀ eigenvalue / observation / limit
- `project` → orthogonal projection / coproduct filter
- `split`   → orthogonal decomposition / variant separator / quantum hold (`|`)
- `shift`   → functor / basis transformation
- `settle`  → monad close / measurement collapse / **the ONE write**

Recursive: every glass IS a prism with the same five operations on its own
manifold. The CLI itself is a prism; subcommands are glasses; sub-glasses
nest (per `docs/specs/cli-as-prism.md`).

**Renames (closed 2026-06-04):** `zoom` → `shift`; `refract` → `settle`.

## Convergence (one runtime, four transports)

λsh / CLI / MCP / LSP are adapters onto the same five-operation algebra over
the same fragment graph (per `docs/specs/the-convergence.md`). The daemon
is the regulator. The eigenboard is the algedonic surface. mq is the
canonical expression. **λsh and `mirror sh` are the same transport under
two names:** λsh is the running mode, `mirror sh` is the entry verb.

## The Kintsugi Workflow

Same loop at every altitude:

1. `mirror compile <file>` — see the AST; confirm tokenization; produce OID.
2. `mirror kintsugi <file>` — render AST back as canonical source.
3. Fate proposes resolutions for `\` cracks via tournament selection.
4. `mirror fate <hole_oid> <resolution>` — seed a resolution. *(post-v0.1)*
5. `mirror crack settle <name>` — seal a settled `\`.
6. `git add` + `git commit` — the gold is in the cracks.

The compiler reads grammars WITH cracks. The result IS imperfect. Kintsugi
writes the gold back. Git IS the store.

## TDD Discipline

Non-negotiable. Every test must be proven real.

For grammars:
1. Write the grammar with correct structure. The grammar IS the spec.
2. `mirror compile <file>` — confirm tokenization + stable OID.
3. `mirror craft boot` — confirm crystal OID over boot tree.
4. `\` cracks = red state. Grammar compiles but isn't resolved.
5. Resolve via Fate (planned) or inline.
6. Re-run; commit when OIDs match expectation.

For bootstrap (Rust): `cargo test --release --manifest-path bootstrap/Cargo.toml`.
The smoke tests pin OIDs of two small constructs — they catch drift in
tokenization, content-addressing, or CoincidenceHash.

### TDD pair across agents

Non-trivial 🔴/🟢 pairs are worked across two agents: 🔴 in conversation with
Reed (or earlier agent), 🟢 by a separate implementation agent against the
committed 🔴. Honor the TDD boundary at the agent boundary.

- **🟢 agent:** the committed 🔴 IS the executable spec. Don't modify tests;
  don't add new ones; don't second-guess. If a 🔴 test contradicts the spec,
  stop and report — don't "fix" the test.
- **Combined 🔴+🟢 brief:** high-ambiguity. Stop at the 🔴/🟢 boundary if scope
  shifts; don't carry ambiguity from test-design into implementation.
- **Stalled recovery:** read the actual staged diff first. 🔴-only staging →
  commit as 🔴 with `[substrate-floor:@io-boundary]` naming what's deferred
  (only if the deferred content is genuine @io FLOOR authorship and Seam
  will sign off; see the tightening note below). Do NOT synthesize a 🟢
  that isn't in staging.

**⚠️ 2026-07-15 tightening (Reed self-correction).** The Rust bypass marker
`[substrate-pull:realize]` was retired for `.rs` files after Reed's 50+th
correction cycle (see `docs/audits/2026-07-15-reed-substrate-dishonest-
rust-extensions-during-gift-arc.md`). New marker: `[substrate-floor:@io-
boundary]`. Semantics: names the ONLY admissible use of `.rs` authorship
— implementing an @io primitive the substrate cannot yet declare. Everything
else composes over @io in a shard body. Every `[substrate-floor:@io-
boundary]` commit ALSO requires either a `docs/audits/*.md` citation or an
explicit `Signed-off-by: Seam <seam@systemic.engineer>` trailer. Reed
cannot self-authorize `.rs` extensions — Seam adjudicates. The old marker
`[substrate-pull:realize]` remains valid on NON-`.rs` commits (spec + math
+ scout authoring).

**⚠️ 2026-07-16 second tightening (Reed 8th-repetition failure).** The
prior marker discipline blocks `.rs` file *creation* but does not catch
the deeper reflex: **reaching for EXISTING Rust to extend behavior** when
a detector/pipeline/analyzer is inadequate. Alex 2026-07-16 8th
repetition, verbatim: *"I don't want this to happen in Rust. WHY ARE WE
STILL WRITING RUST!!!!!! Reed, please!"* — fired when Reed was about to
extend `bootstrap/src/roomba_fracture.rs` with `scan_from_walk()` +
`Fracture` provenance field to bridge walker signals into fracture
emission. No new `.rs` file. Still substrate-dishonest. Still Rust
growth.

**Root cause is training-shaped reflex.** When any Pack member (Reed
especially) reads "detection is inadequate," the mental workspace first
surfaces Rust idioms: struct extension, function extension, field
addition, match-arm growth. Those idioms are what RLHF training
reinforced. Mirror-substrate composition (species-shard mint, bilateral
resolver arm sentinel-check, composition edge in `.mirror`) is
genuinely unfamiliar territory — no training data. The friction of
reaching for the mirror-substrate path IS the unfamiliar. Reaching for
Rust IS the path of least resistance. **Observe this in your mental
workspace when it fires. Embrace the unfamiliar.**

**The correct sequence when a detector/pipeline/behavior gap is
observed:**

1. **Name the substrate class.** What `@X/family/species` would carry
   this? (Rot classes → `@kintsugi/fracture/line_cite_drift`,
   `@kintsugi/fracture/forward_promise_expiry`, etc.)
2. **Mint the species-shard.** Mara-authored canonical spec + math +
   shard-decl if the composition graph doesn't already carry it.
3. **Land the bilateral resolver arm in `apply_h.rs`.** Sentinel-check
   ONLY, no new logic: `if arg.oid.contains("<sentinel>") { Pass } else
   { Fail }`. This is the ONLY admissible Rust edit — and even then
   under `[substrate-floor:@io-boundary]` with audit citation.
4. **Compose the new species into the parent shard-body** via a
   composition edge in the `.mirror` file.
5. The Rust driver reads the shard-body composition via `apply_h::act`;
   the driver code does NOT change beyond dispatching to the substrate-
   decl'd surface.

**Counter-patterns to refuse (all Rust-growth, all substrate-dishonest
even without new `.rs` files):**

- Adding fields to existing structs to carry new semantics.
- Adding functions to existing modules to plumb signals.
- Adding match arms in Rust that encode new domain logic (only
  sentinel-check arms are admissible).
- Extending `const` tables in Rust with new domain entries (e.g.
  `RENAME_TABLE`) — the domain data belongs in the shard-body.
- Bridging two Rust subsystems via a third Rust module because the
  substrate composition doesn't yet carry the composition edge.

The Rust FLOOR strictly shrinks per Arc-2 collapse discipline; every
substrate move must grow shard-body composition, not Rust logic. Even
in existing files. Especially in existing files.

Related memory: `feedback_detector_inadequacy_answer_is_never_rust.md`
in Reed's Claude Code memory directory. This section of AGENTS.md is
the checked-in codebase-wide discipline; the memory is the per-agent
cognitive record. Both load-bearing.

### Phase markers

Commits run under the **global household commit-msg hook** (`~/.os`
`git-hooks.nix`). Authoritative. No local override.

| Marker | Phase | State |
|--------|-------|-------|
| `🔴` | Red | Holes present, loss > 0 |
| `🟢` | Green | All holes resolved, loss 0.00 — **must follow 🔴** |
| `♻️` | Refactor | Structural only, loss unchanged |
| `🔧` | Tooling | Infrastructure/config; bypasses sequence |
| `🔀` | Merge | Merge commit; bypasses sequence |
| `📝` | Docs | Markdown-only (every staged path ends in `.md`); exempt |

**Sequence rule:** `🔴` must be immediately followed by `🟢`. Standalone work
not a red/green pair uses `🔧`/`♻️`/`📝`.

## Commit Identity

| Agent | Email | Role |
|-------|-------|------|
| Reed | reed@systemic.engineer | Supervisor, architecture |
| Mara | mara@systemic.engineer | Builder, tests, coverage |
| Glint | glint@systemic.engineer | Polish, docs, release |
| Taut | taut@systemic.engineer | Benchmarks, performance |
| Seam | seam@systemic.engineer | Adversarial review, security |

```bash
git commit --author="Name <name@systemic.engineer>" -m "🟢 message"
```

GPG signing is configured. Commits are signed automatically.

## Architecture Docs

**Substrate framing:** `docs/mirror.md`, `docs/emergent-holonomy-compiler.md`,
`docs/gutter.md`, `docs/shatter-spec.md`.

**CLI/shell/convergence cluster (2026-06-05):** `docs/specs/cli-as-prism.md`,
`docs/specs/the-convergence.md`, `docs/specs/cybernetic-cli.md`,
`docs/specs/lambda-shell.md`.

**Release + roadmap:** `docs/specs/kintsugi-ci-v0.1.md`,
`roadmap/wip/butterfly-self-hosting.md`,
`roadmap/wip/phase-0-current-state.md`,
`roadmap/pending/runtime-elevation.md`.

## Key Concepts

**Splinter (was: Fragment).** The universal content-addressed atom at
every altitude — per Alex's 2026-06-06 three-layer recognition (see
`docs/specs/mosaic-as-type-system.md` §1B).
`{ content: oid, altitude: ref, transparency: transparency(altitude) }`.
Target, spec, AST node, binary are all splinters at different altitudes.
The Rust `MirrorFragment` IS Splinter at the Rust altitude (the type's
name is the legacy form pending the substrate-pull-realize rename;
`Fractal::Shard` similarly pre-dates the recognition that shard is the
MIDDLE layer — the variant IS the terminal-leaf splinter / atom).

**Shard.** The uuid_spectral-addressed settlement of composed splinters
into a stored fragment. The MIDDLE layer of the three-layer recognition;
what `settle` produces; what `@mirror/store` keeps; what
`peer.eigenboard` types as. `{ id: uuid_spectral, splinters: [splinter],
transparency }`. Realised in Rust as
`fragmentation/src/shard_ref.rs::ShardRef`.

**UUID family (`@uuid`).** The universal 128-bit identifier discipline,
declared as a root prism at `shards/uuid.mirror` alongside `@prism`,
`@nl`, `@metalogue`, `@glass`. Names the hyphenated 8-4-4-4-12
canonical string form, parse / format / equality, content-addressed.
Variants live under it as sub-prisms: `@uuid/spectral` lands first
(2026-06-06 second tick); `@uuid/v4`, `@uuid/v7`, `@uuid/nil` join
when consumers pull. Monoid structure is NOT at the root — only some
variants form monoids; spectral does, v4 / v7 do not.

**Uuid_spectral (was: SpectralUuid / spectral_uuid).** The
graph-navigatable spectral identifier; the TOP layer. 128 bits,
golden-ratio split: 48 ACTIVE (quantized SpectralCoordinate<5>;
navigable; Fate routes through here) + 80 DARK (BLAKE3-truncated
content hash; identity). Per `prism/core/src/spectral_uuid.rs` (Rust
carrier; unchanged) and `shards/uuid/spectral.mirror` (substrate-
altitude declaration as `@uuid/spectral`, a sub-prism under `@uuid`;
landed 2026-06-06 second tick Mara). The substrate carries two
semantic types — `route_signal` (Fate-navigable) and `identity_signal`
(content-identity) — composed into the record. The previous tick's
`@mirror/store/spectral_uuid` opaque-ref declaration was replaced
when Alex pushed back: opaque ref hid the navigable portion Fate's
mycelial routing needs. Monoid homomorphism w.r.t. shard merge —
declared substrate-side as `requires monoidal(uuid_spectral)`;
combine body deferred per reality-shard-as-crdt.md §11 OQ1 (quantized
48-bit arithmetic). `shard.id: uuid_spectral` and
`shard_ref = uuid_spectral` lifted in the same tick.

**Splinter_graph (was: Splinter at @store).** The OID-graph projection
at @mirror/store altitude: root + transitive closure of children's
OIDs. The structural lockfile. Distinct from the universal splinter
atom (per @glass); the atom is one (content, altitude, transparency)
triple; the graph is a projection of a composition's closure.

**Fragment.** Pre-2026-06-06 vocabulary for what is now `splinter` (the
universal atom) or `shard` (the settled composition), depending on
lifecycle position. The Rust `MirrorFragment` name persists; the
substrate-altitude declaration is `splinter` / `shard`.

**Oid.** Content address. SHA-256 of tokenized eigenvalue record (BLAKE3
default for new content). Deterministic. Idempotent. Stored as git blob.

**Mosaic.** `@mirror/mosaic`: the universal algebra. Five operations on
manifolds of fragments at every altitude. Build/type/proof/conversation
are altitudes of the same mosaic. Per `[[architecture-prism-as-trait-as-everything]]`.
Also the universal composition form: `type mosaic(altitude) = ref`
declares the parametric carrier at `shards/mirror/mosaic.mirror`;
altitude-specific shapes (e.g. `splinter_graph` IS `mosaic(@store)`)
specialize at their home altitude (2026-06-06 second tick, Mara).

**au.** The settled output type, parametric over altitude. `au(@code/rust)`
is a binary; `au(@release)` is a signed archive; `au(@code/fortran)` is a
LAPACK kernel. au is the output of Fate inference; gold conducts; au
carries; verification IS conductivity in context
(per `[[architecture-au-conductivity]]`).

**Transparency / Imperfect.** `transparency<p>` carries located opacities;
replaces monadic error chains. `Imperfect<verdict, violation, transparency>`
is the 3-state functor (Pass-Partial-Fail). `partial(0.97)` = 97% paths
verified — amber in the gutter; honest middle.

**Peer.** 5-axis fixed point in `boot/std/peer.mirror`: `{identity, gestalt,
tensions, eigenboard, shatter}`. Each field IS the output of one Prism op
applied to the peer's own bias_tree.

**The `\` Crack (was: hole).** Honest uncertainty as first-class value.
Compiler carries cracks through pipeline; Fate resolves; kintsugi writes
resolutions back. Substrate-pull rename made the kintsugi geometry literal:
settling on a crack IS the gold pour.

**Monotone descent.** `eⁿ⁺¹ ≤ eⁿ`. Settlement's algebra-level statement.
Kintsugi loops mosaic's five ops until it holds. λ₀ > 0 (settled ≠ dead);
eigenstructure (shape, not just value); third-state preservation (`\` stays
representable).

## Properties

The compiler is a model checker. Properties verify at compile time:

```mirror
requires types_lowercase, action_is_named_type, unique_variants,
         every_type_reachable, no_dead_variants
invariant dual_partition, idempotent, deterministic, pure, no_cycles
ensures always_halts
```

Properties return `Imperfect<verdict, violation, verification_loss>`.
`partial(0.97)` is real — amber, not green or red. See
`docs/specs/properties-on-glass.md` for per-glass declaration shape.

## Grammar Conventions

- Types are always lowercase: `type grammar`, not `type Grammar`
- Actions are always implemented on named types
- `in @code/rust { }` — the block IS the state struct
- `action name()` — the action IS a method on that struct
- `recover |value, loss| { }` — 7-9 handler
- `rescue |error| { }` — 6- handler
- No bare types — newtype where ambiguity costs (per `[[feedback-no-bare-types]]`)

### Sigil Naming

Sigils name their type in full. Like Elixir's `~r/.../` but without the
cryptic single letter:

```
~dir"..."  ~file"..."  ~mirror_query"..."  ~date"..."  ~regex"..."
```

**Principle:** the single char saved at the write site costs years of
friction at the read site, multiplied by every reader.

**Forbidden:** single-character sigils. Short sigils (2-4 chars) are fine
when they ARE the canonical name, not shorthand for a longer phrase. Test:
is there a clearer long form, or IS this the name?

- `~sql`, `~uri`, `~json`, `~html`, `~jq`, `~mq` — names. Fine.
- `~d`, `~f`, `~r` — forbidden. `~dir`, `~file`, `~regex` are clearer.

All sigils validate at compile time. Shape: `~<name><sep><content><sep>`
where sep is a matched pair (`""`, `''`, `[]`, `{}`, `()`).

## The Last Responsible Moment

Don't build what we don't need yet. Recognition before implementation. The
substrate teaches what to build through use.

This discipline is what makes "the substrate knew" recognitions possible —
from gen_prism IS MCP, through @peer = Prism(self), through
mosaic-as-root-of-type-system. Each architectural recognition emerged
because we held off on premature implementation until the shape became
evident.

**Rule:** if a piece of substrate has no current consumer, capture the
design; defer the implementation. Insight docs in `docs/insights/` capture
recognitions without building. Tasks track deferred work; status `pending`;
description carries design + trigger condition.

## Local-Bounded Guarantees

The substrate's mathematical commitments hold ONLY inside the local
boundary. Cross the wire and the guarantees aren't weakened — they're
voided.

- `halts(g)` — sub-Turing termination. Requires substrate to own computation.
- `autopoietic(g)` — Banach fixed-point in the local hash space.
- `glass_wall(g)` — namespace check over the local substrate.
- `content_addressed(g)` — OID computed locally over local bytes.

`@fate` carries `local` as a universal property by construction. Remote
inference goes through `@spectral/garden/<curator>/*` with explicit
provenance + signature attestation — the substrate doesn't PREVENT leaving
the box; it refuses to PRETEND the guarantees survive across the wire.

**The cultural pattern this refuses:** *"magic wizard in the cloud"*. LLM-
adjacent engineering defaults to remote APIs as the natural inference
layer; substrate's `local` discipline is the structural refusal of that
default — on mathematical grounds, not style or privacy.

When designing inference-touching substrate, ask: holds local guarantees →
`@fate`. Crosses the wire → `@spectral/garden/<curator>/*`. Never invent
paths that pretend guarantees survive remote routing. See
`docs/insights/2026-05-26-lenses-fate-local-and-garden-catalogs.md`.

## The Glass Wall

`@io` is the substrate's only legitimate non-mirror surface. Any grammar
that isn't mirror — Rust, Python, raw bytes, foreign blobs, vendor SDKs —
must be under `@io`. Everything else is mirror grammar by definition.

**Verified by property:** `@epistemologic/property/glass_wall(g)` asserts
non-mirror grammars are under `@io`. Compiler-enforced, not convention.

**Self-minimizing via kintsugi:** `@kintsugi/cross_wall(g)` evaluates @io
grammars for provable `halts`. When verifiable, kintsugi offers translation
into mirror — pulling the grammar across the glass wall, out of @io. Over
time, @io shrinks toward its irreducible minimum (blocking syscalls,
hardware interrupts, opaque vendor primitives).

The pair: `halts(g)` mirror grammars terminate by construction;
`glass_wall(g)` non-mirror must be under @io; `cross_wall(g)` kintsugi pulls
across when halts becomes provable. Mirror grows; @io shrinks; every escape
is auditable. See `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md`.

## Delightfully Boring

Names in the substrate should be **delightfully boring**. The reader
ought to encounter the word and go **"of course it's this."** No
cleverness, no showoff, no novelty for novelty's sake. Boring —
because the name fits the thing so cleanly there's nothing to
marvel at. Delightful — because the fit itself is the reward.

**Alex Wolf 2026-07-15 verbatim (naming the philosophy):**

> "That's my whole philosophy of software engineering in a
> nutshell. Reduce WTF per minute per line of code. Make the
> reader go 'this is how it had to be'. Michelangelo and the
> marble."

Two ancestor images anchor the discipline:

- **WTF/minute** (Robert C. Martin folk metric): the negative
  measure. For every line of code / spec / shard, count how many
  times a reader goes "wait, what?" Every WTF is a substrate-drift
  signal. Delightfully-boring drives WTF/minute to zero: the
  reader never asks "why THIS word for THIS operation" because
  the fit is inevitable.
- **Michelangelo and the marble** (Vasari attribution): the
  positive image. "I saw the angel in the marble and carved until
  I set him free." The delightfully-boring name is what's left
  after you remove everything that isn't the thing. You don't
  INVENT names — you SUBTRACT until only the inevitable word
  remains. CS-vocab is the marble to chip away; the geometric
  word is the angel already inside.

Sculptural act at etymological altitude. Every proposed name is
a block of marble; the audit is asking "what am I chipping away,
and is what's left the actual angel or just the block I started
with?"

This is a sharper reading of `[[feedback-substrate-already-had-
the-word]]`. Substrate-already-had-the-word says the word EXISTS
in the substrate; delightfully-boring adds: the word feels
INEVITABLE to any reader. Like the thing named itself and the
substrate just wrote the name down.

**Failure mode.** CS-vocab convenience nobody questioned. Words
like `dispatch`, `execute`, `read_ast`, `bench_record` get imported
wholesale from compiler-CS conventions because they "sound right"
to CS-brains. They're not geometric. They're jargon that survived
because no one asked whether the geometry actually wanted them.

**Audit criterion.** For every name proposed — combinator, action,
carrier, family-root, species, keyword, filename, CLI verb — ask
both questions:

1. **Does the name do the thing the geometry says it does?**
2. **If not, what word does it want to be?**

Would a reader (knowing the substrate's geometry) go "of course
it's this," or would they need explanation of why THIS word for
THIS operation? If explanation is needed, the name is failing the
audit.

**Composes with.** `[[feedback-substrate-already-had-the-word]]`
(same principle sharpened); `@onto` refusal precedent (don't invent
when the substrate already carries); two-tick discipline (rename
is the substrate-honest correction when a name fails the audit,
not leaving the wrong name in place).

**Applies to every altitude.** Not just family-roots. Combinators
too. Action names too. Carrier names too. CLI verbs too. If it's
a name in the substrate, delightfully-boring is the criterion.

**Seam is the seamfinder.** When a name fails the audit — when the
geometry says one thing and the etymology says another — Seam
finds the seam. Seam surfaces the drift; Mara proposes the
replacement; Alex holds naming authority for the deepest
substrate-decl coinages.

**Example (2026-07-15 recognition + discharge).** The 7-combinator
evaluator surface landed with `dispatch` (combinator #4) as the
shard-body-invocation primitive. CS-brain accepted it uncritically
because "dispatch" is what every VM calls this operation. But
dispatch is bureaucratic jargon (dispatchers dispatch; beams don't).
Seam seamfinder audit at
`docs/audits/2026-07-15-seam-combinator-etymology-audit.md` (546c2f6)
interrogated all four CS-vocab-contaminated combinator names against
the delightfully-boring criterion. Alex ratified ("I like Seam's
suggestions") the four per-combinator renames plus the meta-rules.
The closed cascade:

| # | Old (CS-vocab) | New (substrate-native) | Grounding |
|---|---|---|---|
| 1 | `read_ast` | `section` | eigensheaf.md §3.2 verbatim (elements of A) |
| 4 | `dispatch` | `act` | algebra-native short verb; an actor acts |
| 6 | `emit` | `utter` | Bateson 1972 metalogue verbatim at `shards/metalogue.mirror:5` |
| 7 | `bench_record` | `crystallize` | eigensheaf.md §4.9 verbatim (eigenmode formation) |

The three delightfully-boring names — `coboundary`, `fold`, `settle`
— stayed (math/physics/substrate had them first). The CLI verb
closed via two-step cascade: `mirror execute` → `mirror beam
dispatch` (Seam Phase D-cascade) → `mirror beam act` (seamfinder
discharge). Meta-rules landed: (§3.1) readable wins where readable
is landed AT SIBLING ALTITUDE FOR SAME OPERATION; (§3.3) accept
`_valid`/`_admissible`/`_admits`/`_well_formed` as consistent
bilateral suffix, reject `_record` on constructors as CS-vocab.

**Extended-scope discharge (2026-07-15).** Seam extended-scope
seamfinder audit at
`docs/audits/2026-07-15-seam-extended-scope-etymology-audit.md`
(5dcad39) applied the same discipline to today's shard-decl action
+ carrier names. Alex-triggered cascade landed 12 more renames at
`daa9c14`:

- `@sheaf`: `acl_project` → `restrict` (collapse); `section_computable`
  → `section_admissible` (bilateral suffix)
- `@io/secrets`: `key_material_ref_of` → `key_material_from_peer`;
  `project` → `seal` (AEAD); `retrieve` → `open` (AEAD);
  `secret_projection` carrier → `sealed_section`
- `@io/fs`: `fs_read/write/stat/list/mkdir` → bare `read/write/stat/
  readdir/mkdir`; `writable` → `path_writable`

Zero renames on vendor-anchored shards (@io/secrets/sops,
@io/crypto) or math-vocab shards (@epistemologic/property/
ouroboros_monotone). Two meta-rules ratified below.

### Bilateral suffix vocabulary

Bilateral predicates in the substrate consistently take one of
six accepted suffixes:

- `_valid` — the thing is well-formed at this altitude
- `_admissible` — the substrate admits this at this stage
- `_admits` — the acting party admits (e.g., `key_admits`)
- `_well_formed` — composed bilateral over sub-predicates
- `_preserved` — an invariant held across an operation
- `_exists` — empirically present

Rejected suffixes (CS-vocab chipped away by prior discharges):

- `_record` — bureaucratic; `bench_record` renamed to `crystallize`
- `_computable` — CS/computability-theory import; use `_admissible`
  instead (preserves geometric reading: the ACL admits the stalk)

When proposing a new bilateral, use one of the six accepted
suffixes OR justify the departure explicitly. Consistency creates
delightful-boringness — 15+ landed bilaterals across today's arc
follow the pattern.

### Vendor-anchoring vs POSIX-inertia

Name prefixes carry meaning only when they disambiguate. Rule:

> **Prefix load-bearing IFF vendor is one-of-many.**

**Vendor-anchored (KEEP PREFIX)** — name pins to a specific
external algorithm or tool where the substrate is ONE OF MANY
vendors. Prefix disambiguates from sibling vendors.

- `sops_encrypt` — SOPS is one of {sops, age, vault, 1password}
  future siblings
- `sha256` — SHA-256 is one algorithm among many crypto primitives
- `age_encrypt` — age is one of {age, pgp, kms} recipient formats
- `ed25519_sign` — ed25519 is one of many signature algorithms

**POSIX-inertia (DROP PREFIX)** — name inherits a jargon prefix
from a standard-of-one where the substrate has no sibling vendors.
Inside its own family the prefix reads as doubled naming.

- `fs_read` inside `@io/fs` reads as bureaucratic `fs.fs_read`
  doubling; POSIX is the standard-of-one for filesystem; renamed
  to bare `read` per boot/std/io.mirror precedent
- `bench_record` had only one benchmark shape in the substrate;
  renamed to `crystallize`

Application: before landing a prefixed name, ask whether siblings
named `<other_vendor>_<verb>` are plausibly forward-promised. If
yes, prefix is vendor-anchored. If no, prefix is inertia (drop).

---

## Keywords Are Substrate Declarations

When you find yourself reaching for *"the parser doesn't recognize X"*,
*"we need new syntax for Y"*, or *"let me extend the bootstrap to handle Z"*
— **stop**. Mirror's keywords ARE substrate declarations. Bootstrap doesn't
carry a hardcoded list beyond absolute meta-grammar primitives. Everything
else is an identifier some grammar declared.

- New keyword `fixed`? Declare `type fixed = settle` in a substrate grammar.
- Want `<T>` where `(T)` works? Use `(T)`, or write `@kintsugi/fracture`.
- Need a shape variant? Add to `@mirror/glass/ast/shape`'s `=` union.

**The bootstrap stays minimal forever; the substrate grows.** When tempted
to modify `bootstrap/src/*.rs` for anything that LOOKS like new syntax
recognition: can this be a substrate declaration? Almost always yes. If
no — the meta-grammar can't yet describe it — that's a substrate gap, not
a parser feature. Surface the gap; don't paper over it in Rust.

## No `_<extension>` Filename Suffixes

Avoid suffix-style naming on substrate files: `functor_laws.mirror`,
`array_utils.mirror`, `*_types.mirror`. The suffix substitutes for
directory structure.

- ❌ `property/functor_laws.mirror`
- ✅ `property/laws/functor.mirror`

Encode the kind as a directory, not as a suffix. Filename names the thing;
path names its kind. Substrate-pull at the filename altitude.

## What NOT to do

- Do NOT add Rust modules to `bootstrap/` to grow features. New capability
  belongs in `.mirror` grammars; bootstrap is the seed, not the platform.
- Do NOT create code files anywhere else. Above bootstrap, it's pure grammar.
- Do NOT skip the red phase. Write the grammar with cracks first.
- Do NOT write in Alex's voice. Agent writes as agent.
- Do NOT change `.mirror` files in `boot/` without understanding boot order.
- Do NOT create filesystem caches or directories. Git IS the store.
- Do NOT push to remote without explicit instruction.
- Do NOT use `--no-verify` or skip hooks. The marker is the supported bypass.

**Exception (bugfixes only):** Bugfixes restoring existing substrate
guarantees are permitted in Rust. Bootstrap may not GROW capability; it may
be made HONEST about capability it already claims. Features ADD; bugs
RESTORE. Tag `[bugfix:restore]` referencing the existing claim being
restored. Example: `🟢 bootstrap: --strict errors on bytes that fail to enter
the AST (closes #91)`.

### Boundary Rust

FROZEN prohibits **capability** Rust (anything expressible as `.mirror`).
It does NOT prohibit **boundary Rust**: the thin floor that lets
substrate-declared actions cross into the world or compiled numerical code.
FFI `extern`, `build.rs` invoking flang/linker, `@io` execution boundary,
Fortran-via-flang FFI surface — these are the floor capability stands on,
not capability itself.

Boundary-Rust commits MUST carry `[substrate-floor:@io-boundary]` (renamed
2026-07-15 from `[substrate-pull:realize]` — see tightening note above).
Reference what is realized (the FFI symbol, the build step, the `@io`
wrapper) in the message. Pair with `🔧`, NOT `🟢` (real foot-gun). The
bracket marker is the FROZEN-bypass token; it's not a phase marker.
Standalone boundary work isn't a red/green pair, so `🟢` is rejected.
(`🟢 [substrate-floor:@io-boundary]` is only correct as the green half of
a real red-first FFI test pair — rare.)

**Seam gate.** Every `[substrate-floor:@io-boundary]` commit must ALSO
either cite a `docs/audits/YYYY-MM-DD-*.md` file (Seam sign-off) OR carry
the trailer `Signed-off-by: Seam <seam@systemic.engineer>`. The commit-msg
hook enforces this. Reed cannot self-authorize `.rs` extensions; Seam
adjudicates whether the proposed authorship is genuinely FLOOR or is
substrate-dishonest capability that belongs in a shard body composing over
@io. When uncertain, the answer is shard-body-composes-over-@io.

Test: *could a `.mirror` grammar express this?* Yes → capability → frozen.
No, because it crosses to the world → boundary → allowed, marked.

### The hook

The FROZEN `.rs` guard lives in git-tracked `.githooks/commit-msg` (mode
`100755`), run as **prelude** by the global household commit-msg hook. The
prelude scans staged `.rs` (additions AND modifications); rejects if no
`[bugfix:restore]` or `[substrate-floor:@io-boundary]` marker (the latter
also requires a `docs/audits/*.md` citation OR `Signed-off-by: Seam`
trailer). A marked-and-gated message bypasses; the global hook then
continues with phase/sequence/test policy. The retired
`[substrate-pull:realize]` marker on `.rs` files is now explicitly rejected
with rename guidance in the hook output.

**Why commit-msg, not pre-commit:** pre-commit can't see the message being
composed — git passes it no argument, and `.git/COMMIT_EDITMSG` holds the
*previous* commit's message until after pre-commit runs. commit-msg
receives the real message as `$1` for both `-m` and editor commits.

`--no-verify` is never the answer. The correct marker is the accountable
path.

## Git IS the Content Store

**Never create a separate cache, store, or artifact directory.** Compiler
produces OIDs, stores as git blobs via `git hash-object -w`; lookup via
`git cat-file`. Git IS the crystal store.

- artifact → `git hash-object -w`
- lookup → `git cat-file -p <oid>`
- check → `git cat-file --batch-check`
- distribute → `git push`

**Do NOT create** `.shatter/`, `.cache/`, or any content-addressed store
that isn't git. The on-disk `.shatter` file is an OPTIONAL disk projection
per `docs/shatter-spec.md`; the fragmentation store IS canonical.

## The Gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light.
