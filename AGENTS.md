# Agents

Instructions for AI agents working on the mirror compiler.

## The Compiler

`mirror` — an emergent holonomy compiler. `.mirror` files -> content-addressed
artifacts -> verified domains. The compiler IS the LSP. The CLI IS the REPL.
The gutter IS terni.

The compilation return type is `Oid` — a content-addressed SHA-256 hash stored as a git blob.

## The Bootstrap Seed

The bootstrap seed is a small Rust binary built from `bootstrap/`. The
installed binary lives at `~/.local/bin/mirror` (~370KB arm64). It is the
only non-mirror artifact in the system.

```
bootstrap/Cargo.toml
bootstrap/src/        # tokenizer, hash, content, render, pipeline, git wiring
bootstrap/tests/      # OID smoke tests
```

The bootstrap implements only what the language cannot yet describe of
itself: tokenization, the CoincidenceHash<3>+SHA-256 content address, the
bidirectional renderer, and `git hash-object -w` storage. Everything above
that — grammars, properties, the compilation loop — is `.mirror` in `boot/`.
Cluster D of the road to 1.0 lets `craft --target binary boot` regenerate
the seed; at that point the Rust source becomes vestigial.

All extensions happen through `.mirror` grammars in `boot/` or `boot/std/`.
The compiler evaluates them. The `\` hole handles what isn't concrete yet.
Fate resolves it.

## Commands

```bash
cd /Users/alexwolf/dev/projects/mirror
~/.local/bin/mirror compile <file>     # compile a single grammar
~/.local/bin/mirror craft <target>     # compile a directory of grammars
~/.local/bin/mirror kintsugi <file>    # render the AST back as canonical source
~/.local/bin/mirror '<mq>' < input     # mq pipeline over stdin
~/.local/bin/mirror <input> '<mq>'     # mq pipeline over a file
```

direnv keeps the shell warm. Use `~/.local/bin/mirror` directly.

`mirror run` and `mirror fate` are on the road to 1.0 — the bootstrap does
not yet implement them. See `docs/specs/road-to-1.0.md`.

## The Kintsugi Workflow

This is how grammars evolve. Steps 1–3 are planned for the 1.0 cycle; today,
step 4 is the working surface:

1. `mirror run <file>` — execute the grammar. See the `\` holes. Measure the loss. *(future)*
2. Fate proposes resolutions through tournament selection. *(future)*
3. `mirror fate <hole_oid> <resolution>` — seed a resolution. *(future)*
4. `mirror kintsugi <file>` — render the AST back as canonical source.
5. `git add` + `git commit` — the gold is in the cracks.

The compiler reads grammars WITH holes. The result IS imperfect. Kintsugi
writes the gold back. The commit captures the resolution. Git IS the store.

## TDD Discipline

Non-negotiable. Every test must be proven real.

For grammars, TDD means:
1. Write the grammar with the correct structure. The grammar is the specification.
2. `mirror compile <file>` — confirm the grammar tokenizes and produces a stable OID.
3. `mirror craft boot` — confirm the crystal OID over the boot tree is unchanged or matches expectation.
4. If there are `\` holes, that's the red state. The grammar compiles but isn't resolved.
5. Resolve holes through Fate (planned) or manual resolution writing the body inline.
6. Re-run `mirror compile`/`craft`. Commit when the OIDs match expectation.

For the bootstrap (Rust): `cargo test --release --manifest-path bootstrap/Cargo.toml`.
The smoke tests pin the OID of two small constructs — they catch drift in
tokenization, content-addressing, or CoincidenceHash.

### TDD pair across agents

Non-trivial 🔴/🟢 pairs are typically worked across two agents: the 🔴 in
conversation with Reed (or by an earlier agent), the 🟢 by a separate
implementation agent against the committed 🔴. Honor the TDD boundary at the
agent boundary too — test-design and implementation are two distinct cognitive
jobs, and the boundary between them is where stalls happen.

- **If you're the 🟢 agent**, the committed 🔴 IS the executable spec. Don't
  modify the tests, don't add new ones, don't second-guess them. Make them
  pass. If a 🔴 test asserts something you believe contradicts the spec, stop
  and report rather than "fixing" the test.
- **If you receive a combined 🔴+🟢 brief**, scope is high-ambiguity. Stop and
  report at the 🔴/🟢 boundary if scope shifts under you, rather than carrying
  ambiguity from test-design into implementation.
- **Recovery from a stalled run**: read the actual staged diff first; don't
  assume from the brief what's there. If the staged diff is 🔴-only, commit
  as 🔴 with `[substrate-pull:realize]` and a body naming what's deliberately
  deferred. Do NOT synthesize a 🟢 that isn't in the staging. Refuse the
  pretty pattern when the honest pattern is uglier and truer.

This pattern was named on tick #126 (recovery agent's "Option C" — honest
🔴-only commit when the brief had expected to find 🟢-ready staging).

### Phase markers

Every commit message must start with a phase marker. mirror commits run under
the **global household commit-msg hook** (`~/.os` `git-hooks.nix`), which is now
authoritative — there is no local `core.hooksPath` override. The global hook
enforces exactly one marker, the 🔴→🟢 sequence rule, and (if a `Justfile` with
a `pre-commit` recipe exists) a staged-only test run. mirror has **no Justfile**,
so that test step short-circuits with `(no 'just pre-commit' recipe found —
skipping test validation)`. The FROZEN `.rs` guard (below) is the real per-repo
gate.

| Marker | Phase | State |
|--------|-------|-------|
| `🔴` | Red | Holes present, loss > 0 |
| `🟢` | Green | All holes resolved, loss 0.00 — **must follow 🔴** |
| `♻️` | Refactor | Structural only, loss unchanged |
| `🔧` | Tooling | Infrastructure/config; bypasses the sequence rule |
| `🔀` | Merge | Merge commit; bypasses the sequence rule |
| `📝` | Docs | Markdown-only — valid ONLY when every staged path ends in `.md`; exempt from the sequence rule |

**Sequence rule:** `🔴` must be immediately followed by `🟢`, and `🟢` requires a
preceding `🔴`. Standalone work that isn't a red/green pair must NOT use `🟢` —
it will be rejected ("declared 🟢 but previous commit was not 🔴"). Use `🔧`,
`♻️`, or `📝` instead. Doc-only commits (every staged path `.md`) use `📝`.

## Commit Identity

Each agent commits as themselves:

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

## Architecture

Read these docs before working:

- `docs/mirror.md` — what mirror IS
- `docs/emergent-holonomy-compiler.md` — the full architecture
- `docs/gutter.md` — holonomy rendered as green/amber/red
- `docs/shatter-spec.md` — the .shatter crystal format

## Key Concepts

### Oid

Content address. SHA-256 of tokenized eigenvalue record.
64 hex chars. Deterministic. Idempotent.

### The Five Operations

focus, project, split, zoom, refract.
Every command runs one or more of these. They are the trait methods.
They map to the Prism optic.

### The `\` Hole

`abstract default = \` — honest uncertainty as a first-class value.
The compiler carries holes through the pipeline. Fate resolves them.
Kintsugi writes the resolutions back into source.

### Imperfect

The compilation return type wraps `<verdict, violation, verification_loss>`.
Partial verdicts are real — `partial(0.97)` means 97% of paths verified.

## Boot Sequence

```
boot/00-prism.mirror        the five optics
boot/00a-sigil.mirror       navigation sigils (. .. ... ~ @ ^ HEAD)
boot/01-meta.mirror         meta operations
boot/01a-error.mirror       error handling (recover/rescue)
boot/01b-nl.mirror          natural language interface
boot/02-actor.mirror        actor model
boot/02-epistemologic.mirror epistemology
boot/02a-io.mirror          IO boundary
boot/02b-runtime.mirror     runtime primitives
boot/03-shatter.mirror      crystal format
boot/04-code.mirror         code generation
boot/04a-code-rust.mirror   Rust target
boot/04b-code-gleam.mirror  Gleam target
boot/05-property.mirror     verification properties
boot/06-action.mirror       action optic (GAT)
boot/07-package.mirror      package management
boot/07a-package-git.mirror git packages
boot/07b-package-spec.mirror package specs
boot/std/                   79 library grammars
```

18 boot files + 79 std grammars = 97 total.
The boot files establish the language. Each file builds on the previous.
The compiler learns by reading them in order.

## Properties

The compiler is a model checker. Properties are verified at compile time.

```mirror
requires types_lowercase
requires action_is_named_type
requires unique_variants
requires every_type_reachable
requires no_dead_variants
invariant dual_partition
invariant idempotent
invariant deterministic
invariant pure
invariant no_cycles
ensures always_halts
```

Properties return `Imperfect<verdict, violation, verification_loss>`.
Partial verdicts are real — `partial(0.97)` means 97% of paths verified.

## Grammar Conventions

- Types are always lowercase: `type grammar`, not `type Grammar`
- Actions are always implemented on named types
- `in @code/rust { }` — the block IS the state struct
- `action name()` — the action IS a method on that struct
- `recover |value, loss| { }` — 7-9 handler
- `rescue |error| { }` — 6- handler

### Sigil Naming

Sigils name their type in full. Like Elixir's `~r/.../` but without the cryptic single letter.

```
~dir"/Users/alexwolf/.mara"                  # not ~d
~file"~/notes.md"                            # not ~f
~mirror_query"focus |> project @code/rust"   # not ~mq
~date"2026-05-25"                            # not ~D
~regex"^[a-z]+$"                             # not ~r
```

**The principle:** future engineers will thank us. Some experienced engineers will hate it; the experienced ones who've debugged unfamiliar codebases will recognize it. Early-career engineers won't have to learn cryptic abbreviations to read code. The single character saved at the write site costs years of friction at the read site, multiplied by every reader.

**Single-character sigils are forbidden.** Short sigils (2-4 chars) are fine when they ARE the canonical name, not shorthand-for-a-longer-phrase. The test: is there a clearer long form, or IS this the name?

- `~sql`, `~uri`, `~json`, `~html`, `~css` — names. Fine.
- `~jq` — a name (the jq query language). Fine.
- `~mq` — a name (the mirror query language). Fine. Engineers will understand `~mq` from `~jq`.
- `~mirror` — a full mirror grammar literal. Different thing from `~mq`. Sibling, not synonym.
- `~d` for directory — forbidden. `~dir` has a clearer long form.
- `~f` for file — forbidden. `~file` has a clearer long form.
- `~r` for regex — forbidden. `~regex` has a clearer long form.

**All sigils validate at compile time.** The sigil name picks the validator; the validator returns the typed value or fails the compile with a precise error. Per Elixir's `~r`/`~D`/`~U` pattern, generalized.

**Shape:** `~<sigil_name><separator><content><separator>`. Separators are matched pairs (`""`, `''`, `[]`, `{}`, `()`). Choose the separator that minimizes escaping for the content.

## The Last Responsible Moment

Don't build what we don't need yet. Recognition before implementation. The substrate teaches what to build through use.

This discipline is what makes "the substrate knew" recognitions possible. Each architectural recognition (gen_prism IS MCP; @peer = Prism(self); shard = observer-relative λ₀; portal = `@io.socket` + content-addressed subspace; glass_wall as inverted halts; spectral triple as heuristic composition; lens as constructivism made structural; garden as vetted-corpus distribution) emerged because we held off on premature implementation until the shape became evident.

**The rule:** if a piece of substrate has no current consumer, capture the design; defer the implementation. The capture stays useful; the implementation might miss-shape itself against future demand we haven't yet learned.

**Apply via:**

- Insight docs in `docs/insights/` capture recognitions without building.
- Tasks in the task list track the deferred work; status `pending`; description carries the design + the trigger condition.
- When a real consumer surfaces (a downstream task needing the deferred piece), pull the design from capture and implement against the now-clearer shape.

**Why this works:** mirror's substrate compounds. Each recognition makes the next one cheaper. Deferred captures aren't lost work; they're seeded design. When demand surfaces, the implementation is faster because the design already exists, AND it's better-shaped because the demand sharpened the requirements.

## The Local-Bounded Guarantees

The substrate's mathematical commitments hold ONLY inside the local boundary. Cross the wire and the guarantees aren't weakened — they're voided.

- `halts(g)` — sub-Turing termination. Requires the substrate to own the computation.
- `autopoietic(g)` — Banach fixed-point in the local hash space.
- `glass_wall(g)` — namespace check over the local substrate.
- `content_addressed(g)` — OID computed locally over local bytes.
- is-copium's sub-Turing alignment escape — undecidable cross-wire.

`@fate` carries `local` as a universal property by construction. Any inference routed through `@fate` satisfies these guarantees. Remote inference goes through `@spectral/garden/*` packages — the substrate doesn't PREVENT the user from leaving the box; it refuses to PRETEND the guarantees still hold across the wire. Garden carries explicit curator signatures + audit trails where the structural proofs end.

**The cultural pattern this refuses:** the *"magic wizard in the cloud"* default. LLM-adjacent engineering defaults to remote APIs as the natural inference layer; substantial pre-training plus convenience makes this near-invisible. The substrate's `local` discipline is the structural refusal of that default — not on style grounds; not on privacy grounds; on mathematical grounds.

**Practical application:** when designing any new substrate piece that touches inference, ask:

- Does it hold the local guarantees? → lives under `@fate`.
- Does it cross the wire? → must live under `@spectral/garden/<curator>/*` with explicit provenance + signature attestation.
- Never invent paths that pretend the guarantees survive remote routing. If you find yourself constructing a workaround that preserves remote-routing-as-@fate, stop — you're doing what the substrate exists to refuse.

See `docs/insights/2026-05-26-lenses-fate-local-and-garden-catalogs.md` § "Why `local` is universal."

## The Glass Wall

`@io` is the substrate's only legitimate non-mirror surface. Any grammar that isn't mirror — Rust, Python, Go, raw bytes, foreign binary blobs, vendor SDKs — must be under the `@io` namespace. Everything else is mirror grammar by definition.

**Verified by property:** `@epistemologic/property/glass_wall(g)` asserts that non-mirror grammars are under `@io`. Applied at compile time across the boot tree. The discipline isn't held by convention; the compiler enforces it.

**Self-minimizing via kintsugi:** `@kintsugi/cross_wall(g)` evaluates @io grammars for provable halts. When an @io grammar's behavior is verifiable, kintsugi can offer translation into mirror — pulling the grammar across the glass wall, out of @io. Over time, @io shrinks toward its irreducible minimum (blocking syscalls, hardware interrupts, opaque vendor primitives).

**The pair with halts:**

- `halts(g)` — mirror grammars terminate by construction (sub-Turing).
- `glass_wall(g)` — non-mirror grammars must be under @io.
- `cross_wall(g)` — kintsugi pulls @io grammars across when halts becomes provable.

Together: mirror grows; @io shrinks; nothing escapes the boundary; every escape is auditable.

See `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md`.

## Keywords Are Substrate Declarations

When you find yourself reaching for *"the parser doesn't recognize X"*,
*"we need new syntax for Y"*, or *"let me extend the bootstrap to handle Z"* —
**stop**. The substrate-pull reflex is wrong-shaped. The shape that's right
almost always is: **declare it in the substrate.**

Mirror's keywords ARE substrate declarations. The bootstrap doesn't carry a
hardcoded list of keywords beyond the absolute meta-grammar primitives.
Everything else is an identifier that some grammar in the substrate has
declared. To add a keyword, declare it. To rename a keyword, alias it.
The *"parser learns new syntax"* framing is OOP-trained reflex; mirror's
parser doesn't learn — the substrate accumulates.

Examples of the right move:

- New keyword `fixed`? Declare `type fixed = refract` (or whatever Prism op
  it composes from) in a substrate grammar. Done.
- Want `<T>` to work where `(T)` works? Use `(T)`; don't extend the parser.
  Or write a `@kintsugi/fracture` rule for migration.
- Need a new shape variant? Add it to `@mirror/glass/ast/shape`'s `=` union;
  the substrate carries it forward.

**The bootstrap stays minimal forever; the substrate grows.**

When tempted to modify `bootstrap/src/*.rs` for anything that LOOKS like
new syntax recognition, ask first: can this be a substrate declaration?
The answer is almost always yes. If the answer turns out to be no — the
requirement is a primitive the meta-grammar can't yet describe of itself —
that's a substrate gap, not a parser feature. Surface the gap; don't paper
over it in Rust.

This is the same reflex as the FROZEN/bugfix-only policy below, applied
at the syntax altitude: the bootstrap describes only what the language
cannot yet describe of itself. Everything you can write as a substrate
declaration, you must.

## No `_<extension>` Filename Suffixes

Avoid suffix-style naming on substrate files: `functor_laws.mirror`,
`array_utils.mirror`, `string_helpers.mirror`, `*_types.mirror`. The
suffix is substituting for directory structure. If `functor` the property
needs to be disambiguated from the laws of functors, the directory does
the disambiguating:

- ❌ `property/functor_laws.mirror`
- ✅ `property/laws/functor.mirror`

Same principle for utility-shaped, helpers-shaped, types-shaped files —
encode the kind as a directory, not as a suffix. The filename names the
thing; the path names its kind. Sibling files in `laws/` keep their bare
names (`monoidal.mirror`, `monotonicity.mirror`) — the directory already
carries the law-shape; the name doesn't need to restate it.

This is substrate-pull at the filename altitude: the structure carries
the meaning; the name doesn't restate the structure. Same reflex as
*"keywords are substrate declarations"* — paths ARE substrate structure.

## What NOT to do

- Do NOT add new Rust modules to `bootstrap/` to grow features. New capability
  belongs in `.mirror` grammars; the bootstrap is the seed, not the platform.
- Do NOT create code files anywhere else in the repo. Above the bootstrap,
  it's pure grammar.
- Do NOT skip the red phase. Write the grammar with holes first.
- Do NOT write in Alex's voice. Agent writes as agent.
- Do NOT change .mirror files in `boot/` without understanding the boot order.
- Do NOT create filesystem caches or directories. Git IS the store.

**Exception (bugfixes only):** Bugfixes that restore existing substrate
guarantees are permitted in Rust. The bootstrap may not GROW capability,
but it may be made HONEST about capability it already claims. The
distinction: features ADD; bugs RESTORE. Reference the existing claim
being restored in the commit message, and tag it `[bugfix:restore]` so
the convention is greppable.

Example: `--strict` always *claimed* that every source byte enters the
AST or errors; the implementation lied; restoring it is a bugfix, not
a feature. See commit `🟢 bootstrap: --strict errors on bytes that fail
to enter the AST (closes #91)` for the canonical shape.

### Boundary Rust is not frozen capability

The FROZEN policy prohibits adding **capability** to Rust — anything
expressible as a `.mirror` grammar. It does NOT prohibit **boundary Rust**:
the thin floor that lets substrate-declared actions cross into the world or
into compiled numerical code. Boundary Rust is the substrate-pull reflex
*realized* deliberately at the only altitude where it belongs — the floor —
rather than papered over inside capability logic.

The distinction is altitude, not language:

- **Capability Rust (FROZEN):** parser logic, dispatch logic, keyword
  recognition, evaluation rules — anything a grammar can describe of itself.
  Declare it in the substrate. Never in Rust.
- **Boundary / floor Rust (allowed, with marker):** FFI `extern` declarations,
  build integration (`build.rs` invoking flang / the linker), the `@io`
  execution boundary, and the FFI surface of the spectral-floor numerical
  primitives (Fortran via flang). These are the floor the capability stands
  on, not the capability. A grammar cannot declare a C ABI symbol or shell out
  to a compiler; that is exactly the primitive the meta-grammar can't yet
  describe of itself — a substrate floor, surfaced honestly.

Boundary-Rust commits MUST carry the `[substrate-pull:realize]` marker. The
marker is a **greppable, accountable assertion**: it says *this change is
floor/boundary work, not capability creep*. Reference what is being realized
(the FFI symbol, the build step, the `@io` wrapper) in the message. Abusing
the marker to slip capability logic into Rust is a trust violation — and a
visible one, permanently legible in `git log`. The marker does not hide the
change; it signs it.

**Pair `[substrate-pull:realize]` with `🔧`, NOT `🟢` (real foot-gun).** The
bracket marker is the FROZEN-bypass token; it is not a phase marker. The
commit still needs exactly one phase marker for the global hook. Standalone
boundary work — adding an FFI `extern` block, a `build.rs` link step, an `@io`
wrapper — is not a red/green pair, so `🟢` is wrong: the global hook rejects a
`🟢` that doesn't follow a `🔴` ("declared 🟢 but previous commit was not 🔴").
Use `🔧 [substrate-pull:realize]` — tooling bypasses the sequence rule and is
the right altitude for floor/build/FFI work. (`🟢 [substrate-pull:realize]` is
only correct when the boundary change is the green half of an actual
red-first FFI test pair — rare; the default is `🔧`.)

When unsure which side of the line a change sits on, ask: *could a `.mirror`
grammar express this?* If yes, it's capability — frozen. If no, and the
reason is that the change crosses to the world (a symbol, a process, a link
step), it's boundary — allowed, marked.

### The hook honors the marker

mirror runs under the **global household commit-msg hook** (`~/.os`
`git-hooks.nix`); there is no local `core.hooksPath` override and nothing to
install by hand. The FROZEN `.rs` guard lives in the **git-tracked**
`.githooks/commit-msg` (mode `100755`), which the global hook runs as a
**prelude** — first, before its own phase/sequence policy. The policy travels
with the repo and is reviewable in `git log`; do NOT copy anything into
`.git/hooks/`.

The prelude flow:

1. Global hook resolves the repo root and, if `.githooks/commit-msg` is a
   tracked executable, runs it with the message file as `$1`.
2. The prelude scans staged `.rs` (`git diff --cached --diff-filter=AM`,
   covering additions AND modifications). If any `.rs` is staged and the
   message carries neither `[bugfix:restore]` nor `[substrate-pull:realize]`,
   it rejects (exit 1) — the commit is blocked. A marked message bypasses the
   FROZEN check ("marker present — FROZEN check bypassed").
3. If the prelude passes, the global hook continues with the phase-marker,
   sequence, and (Justfile-gated) test policy. mirror has no Justfile, so the
   test step is skipped; the FROZEN prelude is the real per-repo gate.

**Why commit-msg, not pre-commit.** The FROZEN check must read the message to
honor the bypass markers. A `pre-commit` hook cannot see the message being
composed — git passes it no argument, and with `git commit -m` the message is
not written to `.git/COMMIT_EDITMSG` until *after* pre-commit runs; at
pre-commit time that file still holds the *previous* commit's message, so a
pre-commit reading it would bypass on the prior commit's marker — a false
bypass. A `commit-msg` hook receives the real message as `$1` for both `-m`
and editor commits, so the bypass is reliable.

`docs/hooks/pre-commit.sample` is **superseded** by the tracked
`.githooks/commit-msg` + the global prelude; it is retained only as
documentation of the arrangement, not as install guidance.

`--no-verify` is never the answer. A `.rs` change that genuinely belongs (a
bugfix restoring an existing guarantee, or boundary/floor work) carries the
correct marker in the message; that is the supported, accountable path.

## Git IS the Content Store

**Never create a separate cache, store, or artifact directory.**

The compiler produces SHA-256 OIDs and stores them as git blobs via
`git hash-object -w`. Lookup via `git cat-file`. Git IS the crystal store.

- Compiled artifact -> git blob (`git hash-object -w`)
- Lookup crystal -> `git cat-file -p <oid>`
- Check if cached -> `git cat-file --batch-check`
- Distribute -> `git push`

**Do NOT create:**
- `.shatter/` directories
- `.cache/` directories
- Any filesystem cache alongside git
- Any content-addressed store that isn't git

**The test:** If your code creates a directory to store compiled
artifacts, you're wrong. Write a git blob instead.

## The Gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light.
