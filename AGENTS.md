# Agents

Instructions for AI agents working on the mirror compiler.

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
  code/                @code/* altitude grammars (rust; llvm + fortran pending)
  io/                  @io/* contracts (cargo; flang pending)

boot/               LEGACY (shrinks per release; monotonic line-count contract)
  00..07-*.mirror      18 kernel grammars (option/result migrating to shards/std/)
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

direnv keeps the shell warm. `mirror join` (entry to λsh), `mirror run`,
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
canonical expression. **λsh and `mirror join` are the same transport under
two names:** λsh is the running mode, `mirror join` is the entry verb.

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
  commit as 🔴 with `[substrate-pull:realize]` naming what's deferred. Do NOT
  synthesize a 🟢 that isn't in staging.

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

**Shard.** The SpectralUuid-addressed settlement of composed splinters
into a stored fragment. The MIDDLE layer of the three-layer recognition;
what `settle` produces; what `@mirror/store` keeps; what
`peer.eigenboard` types as. `{ id: SpectralUuid, splinters: [splinter],
transparency }`. Realised in Rust as
`fragmentation/src/shard_ref.rs::ShardRef`.

**SpectralUuid.** The graph-navigatable spectral identifier; the TOP
layer. 128 bits, golden-ratio split: 48 ACTIVE (quantized
SpectralCoordinate<5>; navigable) + 80 DARK (BLAKE3-truncated content
hash; identity). Per `prism/core/src/spectral_uuid.rs` and
`[[architecture-shard-as-crdt]]`. Monoid homomorphism w.r.t. shard
merge.

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

Boundary-Rust commits MUST carry `[substrate-pull:realize]`. Reference what
is realized (the FFI symbol, the build step, the `@io` wrapper) in the
message. Pair with `🔧`, NOT `🟢` (real foot-gun). The bracket marker is
the FROZEN-bypass token; it's not a phase marker. Standalone boundary work
isn't a red/green pair, so `🟢` is rejected. (`🟢 [substrate-pull:realize]`
is only correct as the green half of a real red-first FFI test pair — rare.)

Test: *could a `.mirror` grammar express this?* Yes → capability → frozen.
No, because it crosses to the world → boundary → allowed, marked.

### The hook

The FROZEN `.rs` guard lives in git-tracked `.githooks/commit-msg` (mode
`100755`), run as **prelude** by the global household commit-msg hook. The
prelude scans staged `.rs` (additions AND modifications); rejects if no
`[bugfix:restore]` or `[substrate-pull:realize]` marker. A marked message
bypasses; the global hook then continues with phase/sequence/test policy.

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
