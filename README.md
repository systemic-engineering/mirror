# mirror

> **Mirror is a programming language written BY AI FOR AI and written FOR HUMANS BY HUMANS.**

![](./void_256.png)

The glass is the grammar. The wine is what you bring.  
The pitch is the eigenvalue. Neither alone. Both together.  
(This project will feel weirdly coherent and entirely backwards. That's by design. It's also convergent. 🍷)

_The author takes no responsibility for any cognitive dissonance this project produces in the reader._  
_Cheers, Alex 🌈_

> _Here for grant verification? See [docs/GRANTS.md](./docs/GRANTS.md) for copy-paste-ready substrate evidence._

---

## Who writes mirror

Both audiences. Each writes for itself.

**By AI for AI** — agents author `@kintsugi/dispatch.mirror`, `@fate/tournament.mirror`,
the inference-shaped grammars. Other agents (Reflection, Fate, the supervisor)
read them. The audience is structurally itself.

**For humans by humans** — humans author `@product/pricing.mirror`,
`@policy/onboarding.mirror`, the domain grammars. Other humans read them.
The audience is again structurally itself.

The substrate doesn't privilege either side. Per-glass property verification,
kintsugi settlement, `Pure<G: Glass>` compile-time witnessing — all run identically
over agent-authored and human-authored grammars. The name is the philosophy: a
substrate that reflects whoever writes in it.

---

## The Wine Glass

Tap a wine glass and it rings. The pitch depends on the glass — its shape,
its thickness, its material. Pour wine in and the pitch changes. Not because
the glass changed. Because the system changed. The glass and the wine together
produce a frequency that neither produces alone.

`mirror` is a compiler that works like this. You write a grammar (the glass).
You bring your code, your data, your topology (the wine). The compiler measures
what emerges (the pitch). The measurement is an eigenvalue — a mathematical
fingerprint of how the structure connects.

You don't need to know what an eigenvalue is. You just need to know that when
you tap the glass, the pitch tells you something true about what's inside.

---

## What It Does

Three commands today. Five operations. Everything settles.

```
mirror compile <file>              tap the glass. get the pitch.
mirror craft <target>              compile a directory of grammars.
mirror kintsugi <file>             render the AST back as canonical source.
mirror sh [@peer]                  enter the shell (λsh; with peer if given).
mirror '<mq-query>' < input        mq pipeline over stdin.
mirror <input> '<mq-query>'        mq pipeline over a file.
```

Every compiled artifact is content-addressed. Same source, same pitch, forever.
Git is the content store. Always has been.

The CLI surface is self-describing: each sub-stage is a minted grammar under
`shards/mirror/lens/cli/` (today: compile, kintsugi, shatter, bootstrap, sh,
reflect, time, crack). The directory listing IS the road to 1.0 — see
`docs/specs/road-to-1.0.md` for sequencing.

---

## The Five Operations

Everything in mirror is a prism. Five operations, five ways to interact
with the glass:

**focus** — narrow on the thing. Point the instrument. Get a reading.

**project** — carve a view. The graph is too much, so you take a slice.

**split** — hold multiple positions simultaneously. See from here and there.

**shift** — cross between registers. From code to abstraction. From the thing
to the pattern of the thing.

**settle** — the geometry settling. You made something and the
measurement shows you what you actually made. Not what you intended. What you
made.

---

## The Honest Hole

```mirror
abstract default = \
```

`\` means: "I don't know the pitch yet. The glass will tell me."

This is not a placeholder. It is honest uncertainty as a first-class value.
The compiler carries `\` through the pipeline. It doesn't guess. It doesn't
default to something convenient. It waits for the structure to disclose
the answer.

A grammar that contains `\` compiles. It just compiles with a hole where
certainty hasn't arrived yet. The hole is the specification.

Kintsugi resolves holes: `mirror run` shows them, Fate proposes resolutions,
`mirror run --fate-store` seeds a resolution, `mirror kintsugi` writes it
back into source. The gold in the cracks.

---

## Sub-Turing

A Turing-complete program cannot determine whether it will ever stop. You
can't prove what it does. You can only run it and watch. Seventy years of
patches on a foundation with a hole in it — type systems, linters, CI/CD,
formal verification bolted onto the side.

`mirror` is sub-Turing. The glass can prove what pitch it produces. Every
grammar terminates. Every property is decidable. The compiler is a model
checker. It doesn't just compile your code — it verifies it.

```mirror
invariant pure
invariant deterministic
invariant no_cycles
ensures always_halts
```

The glass holds because it can prove it holds.

---

## Architecture

Pure grammar above the bootstrap.

```
bootstrap/             Rust source for the bootstrap seed (cargo)
~/.local/bin/mirror    the installed seed (~370KB arm64, built from bootstrap/)
boot/                  18 boot files define the language
boot/std/              79 library grammars extend it
prism/                 24 grammars (the prism ontology)
mirror.spec            the binary describes itself
```

97 grammar files. The bootstrap is the only non-mirror artifact, and it
implements exactly three things:

1. **Tokenizer** — state machine over `.mirror` source, body-capturing for keyword forms
2. **SHA-256 + CoincidenceHash<3>** — content addressing for AST nodes
3. **Git wiring** — `git hash-object -w` writes crystals; `refs/crystals/<source>` indexes them

Everything above the glass is grammar. Cluster D of the road to 1.0 makes
the bootstrap regenerate itself from `craft --target binary boot`, at which
point the Rust source becomes a seed that can be discarded.

---

## The Grammar

18 boot files define the language. 79 std grammars extend it. The compiler
learns by reading them in order — optics, then meta, then actors, then IO,
then code generation, then verification, then packages.

`boot/` is the glass. `boot/std/` is the shelf of glasses above it.
New glass, not new machinery.

Key grammars:
- `@cogito` — the Reflection loop (observe, strategy, perturb, reflect)
- `@craft` — the convergence loop: compile + reflect + tournament → λ₀
- `@kintsugi/shatter` — fracture IS the five operations
- `@code/llvm/emit` — LLVM IR emission from grammar
- `@beam` — the observation surface (absorbs trace + benchmark + measurement)

---

## The Kintsugi Workflow

```
mirror kintsugi <file>     render the AST back as canonical source.
git add + git commit       the gold is in the cracks.
```

The compiler reads grammars with `\` holes. The hole is the specification.
Fate proposes resolutions through tournament selection (elite 1, beam 8,
halving 3). Kintsugi writes the gold back into the source file. Commit.

`mirror run` (execute the grammar, observe the loss) and `mirror fate`
(seed a resolution at `refs/fate/<oid>`) are next — see road-to-1.0.md.

---

## Performance

Bootstrap seed: ~370KB (arm64, release).

```
mirror craft boot: 97 files, 95 cached, 2 recompiled.
Key grammars: all at execution loss 0.00.
```

---

## Build

```bash
cargo build --release --manifest-path bootstrap/Cargo.toml
cp $(cargo metadata --no-deps --manifest-path bootstrap/Cargo.toml \
     --format-version=1 | jq -r .target_directory)/release/mirror \
   ~/.local/bin/mirror
```

Once installed, users do not rebuild the seed. The compiler extends itself
through grammar:

```bash
mirror craft boot     # compile all 97 grammar files
mirror compile <f>    # compile one grammar, return its OID
```

The grammar describes the compiler. The compiler executes the grammar.
The OIDs are deterministic. The compilation is idempotent.

## The Three Tiers

Mirror is the middle tier of a layered substrate:

```
spectral   —  the AI runtime in mirror, behind the SEL gate
               (closed; the agents themselves think here)
     ↑
  mirror   —  the graph-based agent memory layer
               (open; per-glass properties, kintsugi settlement)
     ↑
fragmentation  —  content-addressed storage + HamiltonScheduler
                   (open; first deployment target; useful with any agent)
     ↑
prism_core  —  zero deps, the five-operation kernel
```

Dependency direction is strict: `mirror → fragmentation → prism_core (no deps)`.
The `fragmentation-mcp` ships as a standalone open-source MCP server — native
git integration with the HamiltonScheduler managing agent working memory.
It's useful even if you never touch mirror. Mirror builds on it.

---

## Structural recognitions (as of 2026-06-13)

The substrate's deepest cybernetic invariants are now named. The recognition cascade promoted 11 today and surfaced 7 more as candidates awaiting Pack ratification. The substrate has been operationalizing these structurally for months without naming the ancestors; today the cybernetic tradition becomes load-bearing in the substrate's own vocabulary.

- **Form/substance partition.** `@mirror` is form (what the substrate IS at any moment); `@io` is substance (energy/matter; what the world does in response to a boundary call). Bateson 1970 "Form, Substance and Difference" (19th Korzybski Memorial Lecture, *SEM* Part V) is the cybernetic ancestor. The 8:1 form-side : behaviour-side root-prism ratio is structural per Bateson.
- **Form/process partition at family-root altitude.** `@mirror` is state-observation (form); `@kintsugi` is the transformation engine (process). Mosaic stays at `@mirror`; kintsugi operates ON the mosaic. The substrate's top-level vocabulary now makes the past/future light cone partition visible at the family-root altitude. Maturana structure/organisation + Beer S3/S4 + Bateson form/substance + Hilbert/Bateson lifting all converge here.
- **Mirror IS the operational form of an expanding Hilbert space.** Each substrate-pull recognition widens the Hilbert space dimension; the substrate's coherence under decoherence pressure comes from Bateson logical-type lifting at the path-syntax altitude. The path syntax `@x/y/z` encodes the Bateson level. Mirror is what quantum computing should have been built as — same coherent Hilbert space with lifting operations, different substrate (information, not energy).
- **Mirror IS a content-addressed declarative build system.** Bazel/Buck2/Nix/Shake all reinvented partial form/behaviour separation from scratch; mirror has it foundationally. The mosaic.mirror IS the build shard; `partial(opacity_map)` is genuinely sharper than every canon's binary verdict. The smallest first tick (today's `f2040a0`) lifted `just pre-commit` to `mirror.spec` settlement. Performance floor projection: 15-30× wall-clock once content-addressed-skip wires through.
- **Cybernetic foundation.** Nine cybernetic ancestors named as load-bearing: Ashby (variety), Beer (VSM), Bateson (learning levels + form/substance + metalogue), Maturana-Varela (autopoiesis + structure/organisation), von Foerster (second-order, eigenforms), Pask (conversation, agreement), Glanville (design-IS-cybernetics), Spencer-Brown (distinction-as-primitive), Conant-Ashby (good_regulator). The 11-property `@epistemologic/cybernetic/X` family operationalizes the canon. First member landed today: `cybernetic/variety`.
- **Auto-formatter floor (property/fracture/splinter(ast)/kintsugi bilateral chain).** Properties at `@epistemologic/property/` declare what must hold; fracture bodies at `@kintsugi/fracture/` resolve violations via `splinter(ast)` AST-fragment construction; the kintsugi loop's `active_pass` applies Banach contraction toward fixed point. The substrate's auto-formatting discipline is now math, not manual sweeps. First instance landed today: keyword/depth (`prism @X/Y` → `glass @X/Y` resolution).
- **The `pact` keyword.** Replaces the legacy `grammar` keyword (from when the language was called `conversation`). Operationalizes recognition that every `requires` clause IS a Paskian agreement between substrate-altitude and species-altitude P-individuals. The keyword vocabulary is now three-axis: `prism` opens a possibility space (root family); `glass` increases internal complexity (specialization); `pact` declares typed obligations (the declarative-axis).
- **The prediction paradigm orthogonal to optimization** (candidate). Mirror's gap vocabulary IS the substrate's predictive engine. Optimization-paradigm tools observe code as operations to make fast; prediction-paradigm tools observe code as a typed gap-set whose resolution-space is computable. The substrate's c grows monotonically with the cascade.
- **Alignment as boundary mathematics at the @io crossing** (candidate). Classical alignment shapes internal state; mirror's alignment is at the boundary. The agent's reasoning is unconstrained at form altitude (where computation = thinking = identity = prediction collapse); the constraint fires only at the substance crossing through `@io`; at the boundary, pacts are load-bearing as mathematical contracts grounded in cybernetic ancestry. Bounded RSI via four nested constraints: pact ancestry + Pack convention + recognition history + form/substance partition.

The canonical sites for each: insight docs at `docs/insights/2026-06-{09,10}-*.md`; the cybernetic foundation at `~/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md`. The substrate landings: `15deb05` (cybernetic/variety), `f2040a0` (pre-commit lift to mirror.spec), `7124ed4` → `1e95c0f` (discriminator at @code/metalogue/materialize), `20eaf15` (@kintsugi family migration), `5e68df9` + `9f84ef0` + `a3789c2` + `ef8852c` + `186b8d1` (the property + fracture + splinter(ast) + pact + alignment chain).

### Today's landings (2026-06-11 → 2026-06-13)

- **Recognition #58 promoted (Fate IS optical inference).** Fate inference IS 5-layer D²NN + active Fabry-Perot resonator + Reck/Clements unitary mesh; three independent witnesses; Seam adversarial-promoted 2026-06-11. V1 closure today: `@mirror/spectral/observation` (16-feature Fate input layer) + `@mirror/loss/transparency.dark_dims` (structural-loss retrieval method).
- **Recognition #59 promoted (kintsugi loop altitude-portable).** The bilateral property/fracture pattern extends through every substrate altitude (keyword + operator + symbol) plus self-applied meta-foundation; absorbs four variant shapes (per-predicate, parametric-over-table, parametric-over-computation) under one signature; promoted 2026-06-11 via the optical-keywords cascade.
- **Recognition #61 promoted (form/process kinship at sub-shard altitude).** The form/process partition (#55, @mirror vs @kintsugi at family-root altitude) recurs at sub-shard altitude within a single cascade; two convergent instances (entanglement's `restriction_map` vocabulary kinship + portal's `shape:ref` typed-shape kinship); Seam-promoted 2026-06-11 via adversarial review.
- **CLI-as-prism §3 forward-promise closed.** Eight sub-stages of `@mirror/lens/cli` minted today: `compile`, `kintsugi`, `shatter`, `bootstrap`, `sh`, `reflect`, `time`, `crack`. The CLI surface is now substrate-self-describing; `ls shards/mirror/lens/cli/` is the road map.
- **First canonical mirror threat model.** `docs/specs/threat-model-v0.md` (A1–A5 attacker classes + 5 protected properties); `coincidence-hash-collapse.md` Appendix C (CoincidenceHash⟨5⟩ as KDF context); `roadmap/wip/spectral-db-substrate.md` (Track D's 8-phase migration plan); plus `2026-06-12-trace-altitudes-are-the-wire-dimensions.md` (recognition candidate) and recognition #43 §12 addendum (Jacobi-fixed-point convergence proof).

---

## License

Layered:

- **The compiler + protocols + open adapters + fragmentation** — Apache 2.0 ([`license/APACHE2.md`](./license/APACHE2.md))
- **Curated corpus + garden packages + operational deployment** — systemic.engineering License ([`license/SEL.md`](./license/SEL.md); v1.0 effective; v1.1 draft amendments included)
- **`@spectral/db` engine** — closed-source (binary-only). The AI runtime behind the SEL gate.
- **`@spectral/garden` packages** — per-curator (substrate verifies signatures regardless)

See [`LICENSE.md`](./LICENSE.md) for the layered model.

**`type sel = io + au`** — the SEL license boundary is statically verifiable.
A body whose AST contains both `@io.*` effects AND `@au`-typed values (Fate
inference outputs) is SEL territory. The combination is where AI does things
in the real world; the substrate refuses to be naive about that boundary.
Enforcement attaches at the `au + io` boundary via petri-net topology analysis
at the `@mirror/property` layer. See SEL Part II.

The glass is Apache-2.0. The wine governs itself per the curator's choice.

`e^(n+1) < e^n`
