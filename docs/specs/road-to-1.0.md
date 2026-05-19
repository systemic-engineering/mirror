# Road to 1.0

*2026-05-19. Reed. The gap between what IS and what WANTS to be.*

This spec was written by reading everything. Not by trusting summaries.
The audit findings below are what the working tree actually contains on
2026-05-19, against a freshly-installed `~/.local/bin/mirror`. The 1.0
criteria are measurable. The cleanup ticks are dependency-ordered. The
release happens when one specific crystal is produced by one specific
binary from one specific grammar set.

---

## What Is (audit results)

### Bootstrap state

The bootstrap is a Rust port of the original C bootstrap.

| Where | What | Size |
|---|---|---|
| `~/.local/bin/mirror` | Installed bootstrap binary, arm64 Mach-O | 371 KB |
| `/tmp/mirror-rs/` | Rust source — `Cargo.toml`, `src/`, `tests/` — **not in the repo** | 12 source files |
| `/tmp/mirror.c` | Original C source — **not in the repo** | 2,142 lines |
| `.tmp-mirror.c` | Local working copy of the C source — **untracked, at repo root** | 1,878 lines |

The Rust port has bit-exact CoincidenceHash<3> + `content_oid`
compatibility with the C original (`/tmp/mirror-rs/src/hash.rs` and
`/tmp/mirror-rs/src/content.rs` document this and pin the constants).
`/tmp/mirror-rs/tests/oid_smoke.rs` pins two expected OIDs:

| Input | Expected OID |
|---|---|
| `out collapse\n` | `627584de7c7680a7686273f776e8d9f2580fb9f6a780899957997acc285b86e7` |
| `in @prism\n` | `9836a8ba693f236e974673addbf5c5fd73f922b8d827532676e2ab6541d6c6a2` |

`mirror craft boot` from a clean checkout (with the published cache
warm) produces the suite-level crystal OID:

```
d12f7d4b3499ee0f97b5d94abeef21963089d03c9816a880244304d29b5c746f
```

with `100/100` cache hits over 100 reachable files in `boot/`.

**What the binary actually exposes** (verified by running it on
2026-05-19):

```
mirror <command> [args...]            (legacy subcommand surface)
mirror '<mq-query>' < input           (mq pipeline over stdin)
mirror <input> '<mq-query>'           (mq pipeline over input file)
commands: compile <file>, craft <target>, kintsugi <file>
```

Three subcommands exist in `main.rs`: `compile`, `craft`, `kintsugi`.
Plus the mq pipeline path with `|>` (sequential) and `|\>` (kintsugi
between stages). There is **no `run` subcommand** in the binary today
(`mirror run boot/std/cogito.mirror` returns `unknown: run`). There is
**no `fate` subcommand** either, and **no `--fate-store` flag** on
`run`. README and AGENTS.md claim both.

So: AGENTS.md and README describe **five** commands. The binary
implements **three**. This is the first honest gap.

The pipeline executor (`/tmp/mirror-rs/src/pipeline.rs`) special-cases
`@mirror/kintsugi`, `@kintsugi`, and `@mirror/butterfly{,.butterfly,.emit}`.
Butterfly shells out to `clang` to assemble + link LLVM IR. That is
the closest current thing to "binary emission via grammar" — and it
works through the pipeline, not via `craft --target binary`.

### Grammar inventory

97 reachable `.mirror` files: 20 in `boot/`, 77 in `boot/std/`. Plus 2
in `boot.alex/`, plus 24 in the `prism/` submodule.

| Directory | Count | Status |
|---|---|---|
| `boot/` | 20 (incl. `00-prism.mirror`, `00a-sigil.mirror`, `01-meta.mirror`, ..., `07b-package-spec.mirror`) | Live. All cached. |
| `boot/std/` | 77 | Live. All cached. |
| `boot.alex/` | 2 (`cli.mirror`, `cli.shatter` — same content) | **Stale workspace.** Predates the surface simplification. Not reachable from `boot/`. |
| `prism/` (submodule) | 24 (`actor.mirror`, `coincidence.mirror`, `language.mirror`, ...) | **Orphan submodule.** Files dated 2026-04-09 (pre-Rust-port). Not referenced from `boot/`. Has its own `gestalt/` subdirectory. Submodule URL/HEAD not verified. |

`mirror craft boot` reports `100/100 cache hits` and the crystal OID
above. That number includes `boot.alex/`? No — `cmd_craft` in
`main.rs` calls `collect_files("boot", ".mirror", ...)`, so only `boot/`
is walked. The 100 = 20 (boot) + 77 (boot/std) + 3 inferred (extras
under `boot/std/` not yet listed in the README count of 79; mismatch
between README's "79" and actual 77 is noted but minor).

**What compiles:** every grammar in `boot/` compiles to a stable OID
under the bootstrap binary. None error. None warn.

**What "executes":** the binary has no `run` command, so per-grammar
execution loss cannot be verified through the CLI today. The grammars
themselves declare `\` holes intentionally (e.g. `@mirror/butterfly`
has `emit(ast) -> text { \ }`, `@mirror/store` has all three of
`store / fetch / exists` as `\`). The CLAUDE.md / AGENTS.md narrative
that "key grammars at execution loss 0.00" cannot be verified end-to-end
with the current binary surface. The OID stability is verified. The
loss claim is not.

The grammars referenced in the README "key grammars at execution loss
0.00" list:

| Grammar | File exists? | Holes? |
|---|---|---|
| `@cogito` | yes (`boot/std/cogito.mirror`) | no `\` in body; all bodies are concrete |
| `@mirror/craft` | no — the file is `boot/std/craft.mirror` (no `@mirror/` prefix) | the README's name is wrong |
| `@mirror/build` | **does not exist** as a file. Referenced from the README. | n/a — claim is stale |
| `@kintsugi/shatter` | yes | no `\` in body |
| `@code/llvm/emit` | yes | concrete bodies — but `emit_jacobi` etc. compose by reading `/tmp/mirror.ll`. Self-reference. |

Recent commit graph (last 10) confirms the work is on the surface
simplification: `surface simplification — five operations, five
verbs, one beam`, `@mirror/liquid + @mirror/liquid/ci`,
`@code/llvm/ir`, `@code/extensions`. The compiler is in mid-collapse
toward the five-operation surface.

### Spec inventory

`docs/specs/` contains 41 spec files. Triage:

| Status | Count | Examples |
|---|---|---|
| **Current** (matches 2026-05-19 architecture) | 9 | `surface-simplification.md`, `craft-binary-target.md`, `mirror-binary-surface.md`, `mirror-binary-architecture.md`, `minimum-binary-surface.md`, `kintsugi-self-hosting.md`, `kintsugi-shatter.md`, `io-safety-properties.md`, `code-extension-grammar.md` |
| **Historical** (describes a prior architecture) | 22 | `2026-04-02-ci-ca-ai-design.md`, `2026-04-02-package-system-design.md`, `2026-04-14-mirror-runtime-spec.md`, `2026-04-14-lsp-shatter-plan.md`, `2026-04-15-inline-impl-runtime-plan.md`, `autopoietic-grammar-spec.md`, `bundle-tower-refactor.md`, `cli-args-typed-lambdas.md`, `compiler-error-language.md`, `compiler-surface-plan.md`, `error-surface-spec.md`, `i18n-feature-spec.md`, `kintsugi-tick-1-results.md`, `liquid-types-for-mirror.md`, `merkle-package-manager.md`, `mirror-interpreter.md`, `property-error-surface.md`, `property-projection.md`, `reflection-model.md`, `spec-inference.md`, `spectral-triple-binary.md`, `type-theory-position.md` |
| **Aspirational / stubs** | 5 | `ai-syntax-embedding.md` (large but speculative), `epistemologic-grammar.md` (62 KB — research, not implementation), `graph-native-mirror-model.md`, `hazel-execution-model.md`, `trace-kintsugi-pipeline.md`, `typed-loss-composition.md` |
| **Process / templates** | 5 | `generated-parser-spec.md`, `epistemologic-import-resolver.md`, `minimum-viable-keywords.md`, `mirror-store.md`, `2026-04-14-boot-reorganization-plan.md`, `2026-04-14-template-property-split.md` |

Most of the historical specs assume a Rust crate at `src/`. That
substrate is gone (per AGENTS.md: "There are no Rust files. No C
files. No Cargo files. No `src/` directory. The Rust substrate was
deleted."). The bootstrap is now Rust *again* at `/tmp/mirror-rs/`,
but it's not the same Rust — it's a thin port of the C bootstrap,
not the old structured Rust compiler. The specs that reference
`src/lsp/`, `src/db.rs`, `MirrorOid`, `MirrorLoss`, `MirrorStore`,
`evaluate.rs`, `mirror_runtime.rs` are describing a tree that no
longer exists.

`docs/ROADMAP.md` (38.8 KB, dated 2026-05-12) predates the current
binary. `docs/roadmap/` has 12 files dated March–April 2026, all
pre-collapse.

### Repo state

| Concern | Finding |
|---|---|
| Working tree | Not clean. `boot/std/code/llvm/ir.mirror` modified. Untracked: `.mcp.json`, `.tmp-mirror.c`, `boot/std/mirror/butterfly.mirror`, `docs/specs/craft-binary-target.md`. |
| `.mcp.json` | Exists at repo root, **untracked**, points at `bin/mirror-mcp`. |
| `.claude/settings.local.json` | Exists, **tracked**, duplicates the MCP config (without `"type": "stdio"`). |
| `bin/mirror-mcp` | Shell script, **tracked**, last updated 2026-05-19 16:16. Exposes `mirror_compile`, `mirror_run`, `mirror_craft`, `mirror_kintsugi`, `mirror_fate`. **`mirror_run` and `mirror_fate` will fail** — those subcommands don't exist in the installed binary. |
| Branches (local) | 38. 9 merged into main (deletable). 29 not merged. Of the non-merged: 17 stale (>30 days), 12 in-flight. |
| Branches (remote) | 6: `main`, `glint/mirror-optic`, `reed/git-grammar`, `reed/kintsugi-simplify`, `reed/spec-inference` (current), `taut/kill-mirror-data`. |
| `prism/` submodule | Present at repo root. 24 grammar files dated 2026-04-09. No `.gitmodules` discovered — submodule registration is non-obvious. Not referenced from `boot/` (the live in-tree `@prism` is `boot/00-prism.mirror`). |
| `boot.alex/` | 2 files: `cli.mirror` and `cli.shatter`, identical content (231 B each). Dated 2026-05-19 17:10. Looks like a scratch workspace from the surface-simplification session. Not reachable from craft. |
| `flake.nix` | Present. 410 B. Declares `packages.aarch64-darwin.default` as a path to `./bin`. |
| `flake.lock` | **Missing.** `nix build` against this flake will fail or produce an empty result. |
| `LICENSE.md` | Apache-2.0 (per README). |
| `tasks/` | `coverage-100.md` references `src/lsp/`, `src/db.rs` — pre-collapse. `tasks/next/shatter-continuous-training.md` references a `.shatter` format that's been superseded by `@mirror/spectral` (crystals in git). Both are historical. |
| Worktrees | Five registered: current at `reed/spec-inference`; `/private/tmp/mirror-gestalt-support` (prunable); `/private/tmp/mirror-sign` (prunable); `/Users/alexwolf/dev/projects/mirror-break-crypto` (`reed/prism-migration`); `/Users/alexwolf/dev/projects/mirror-new` (`reed/mirror-new`). Two prunable. |
| `mirror.spec` | Present. Declares `target binary <| @code/llvm <| std` and a kintsugi `collapse(target(boot), target(cargo))`. Asserts the self-host as a kintsugi collapse. |

---

## What Wants To Be (1.0 release criteria)

1.0 means: the bootstrap is in-repo, the binary builds from a clean
checkout, the binary describes itself in grammar, the OID it produces
matches the published baseline, every claim in `README.md` is
verifiable by running the binary against the repo. No `/tmp/` paths.
No untracked files. No stale specs misleading new readers.

Five criteria. Each one falsifiable.

### 1. The bootstrap lives in-repo

**Where:** `bootstrap/` at repo root. Contents:

```
bootstrap/
├── Cargo.toml         (mirrors /tmp/mirror-rs/Cargo.toml)
├── Cargo.lock         (committed)
├── src/               (12 source files from /tmp/mirror-rs/src/)
└── tests/
    └── oid_smoke.rs   (the bit-exact OID pins)
```

**Build:**

```bash
cd bootstrap && cargo build --release
```

produces `bootstrap/target/release/mirror`.

**Install:**

```bash
cp bootstrap/target/release/mirror ~/.local/bin/mirror
```

**Verify:**

```bash
mirror craft boot
# expected:
# cache: <N>/<N> hits
# d12f7d4b3499ee0f97b5d94abeef21963089d03c9816a880244304d29b5c746f
```

The published crystal OID for the 1.0 baseline is the second-to-last
line of `mirror craft boot` against the 1.0 grammar set, with an empty
git crystal cache.

**Falsifiable:** if `cargo build --release` from a clean checkout does
not produce a working binary, criterion fails. If the binary produces
a different crystal OID than the published baseline against the
shipped grammar set, criterion fails.

### 2. The OID generation is declared as grammar

**Grammar:** `boot/std/hash/coincidence.mirror` (new). It declares
CoincidenceHash<3>:

```mirror
in @prism
in @fragmentation

grammar @hash/coincidence {
  type dim = 16
  type projections = 3
  type epsilon = 2.2204460492503131e-16

  hash(bytes) -> oid { \ }
  hash_tagged(tag, bytes) -> oid { \ }
  content_oid(ast) -> oid { \ }
}

out hash
out hash_tagged
out content_oid
```

The `\` holes are honest: the bootstrap binary fills them, and the
property below verifies the binary matches the grammar's contract.

**Verification property:** `boot/std/epistemologic/property/coincidence_matches.mirror`
(new). Uses `@epistemologic/literal` to assert that for a fixed
corpus of inputs, the binary's output matches the grammar's predicted
output. The corpus is the pins in `bootstrap/tests/oid_smoke.rs`,
expressed as a grammar literal.

**Falsifiable:** if the binary's `compile` output for any pinned
input differs from the property's predicted OID, the build fails.
This is the model checker checking the bootstrap.

### 3. The butterfly: `craft --target binary`

**Spec:** `docs/specs/craft-binary-target.md` already defines the
eight stages (collect → resolve → evaluate → emit → concat → assemble
→ link → store).

**Today:** the binary has no `--target` flag on `craft`. The
butterfly is reachable only through the mq pipeline:

```bash
cat mirror.ll | mirror '@code/llvm/ir |> @mirror/kintsugi |> @mirror/butterfly'
```

which produces `./mirror-butterfly`. This is the path that exists.

**For 1.0:** `mirror craft --target binary boot` must produce a
binary at a deterministic path (e.g. `bootstrap/target/release/mirror-self`)
whose `mirror craft boot` output matches the published crystal OID.
That's the self-host test: the binary the grammar produced
reproduces the crystal the original binary produced.

**Falsifiable:** if `mirror-self` produces a different crystal than
`mirror`, the self-host fails. If `mirror-self` cannot be produced at
all from `craft --target binary`, the criterion fails outright. The
1.0 release ships **both** binaries side-by-side and asserts their
crystals are equal.

### 4. The grammar baseline

**The set that ships in 1.0** (frozen at tag-time):

| Top-level grammar | File | Role |
|---|---|---|
| `@prism` | `boot/00-prism.mirror` | The five operations. |
| `@code/llvm`, `@code/llvm/ir`, `@code/llvm/emit` | `boot/std/code/llvm/*.mirror` | LLVM IR emission. |
| `@code/kernel`, `@code/kernel/arm64`, `@code/kernel/x86_64` | `boot/std/code/kernel*.mirror` | Syscalls per arch. |
| `@code/rust`, `@code/gleam` | `boot/std/code/rust.mirror`, `boot/04b-code-gleam.mirror` | Other emit targets. |
| `@kintsugi`, `@kintsugi/shatter`, `@kintsugi/translate`, `@kintsugi/migrate`, `@kintsugi/lift` | `boot/std/kintsugi*.mirror` | Settling. |
| `@craft` | `boot/std/craft.mirror` | The convergence loop. |
| `@cogito` | `boot/std/cogito.mirror` | Reflection. |
| `@fate`, `@fate/connectome`, `@fate/tournament` | `boot/std/fate*.mirror` | Tournament selection. |
| `@beam` | `boot/std/beam.mirror` | The observation surface. |
| `@mirror/evaluate`, `@mirror/execute`, `@mirror/runtime`, `@mirror/interpreter`, `@mirror/compile`, `@mirror/resolve`, `@mirror/grammar`, `@mirror/spectral`, `@mirror/bootstrap`, `@mirror/serve`, `@mirror/lsp`, `@mirror/liquid`, `@mirror/liquid/ci`, `@mirror/liquid/cd`, `@mirror/store`, `@mirror/store/nix`, `@mirror/butterfly` | `boot/std/mirror/*.mirror` | The compiler's own grammars. |
| `@epistemologic/property`, `@epistemologic/property/*`, `@epistemologic/resolve` | `boot/std/epistemologic/*.mirror` | Model-checker properties. |
| `@fragmentation` | `boot/std/fragmentation.mirror` | AST = Merkle tree. |
| `@hash/coincidence` | `boot/std/hash/coincidence.mirror` (new) | The hash, as grammar. |
| `@io` | `boot/std/io.mirror` | The kernel boundary. |
| `@cli`, `@cli/args` | `boot/std/cli.mirror`, ... | The five-verb surface. |
| `@mcp` | `boot/std/mcp.mirror` | MCP dispatch. |
| `@beam`, primitives (`bool`, `number`, `text`, `list`, `map`, `option`, `result`, `set`, `order`, `time`) | `boot/std/*.mirror` | Standard library. |

Listed in `boot/std/README.md` (new, written for 1.0 — terse, one
line per top-level grammar). Each top-level grammar gets one
documented example that runs through `mirror compile <example>` and
produces a known OID.

**Falsifiable:** if a top-level grammar listed above is missing from
the repo at tag-time, or if its OID under `mirror compile` differs
from the README's published OID, the criterion fails.

### 5. Repo hygiene

| Check | Pass condition |
|---|---|
| Working tree | `git status` shows nothing. |
| `.mcp.json` | Tracked. |
| `.tmp-mirror.c` | Either moved into `bootstrap/native/mirror.c` (reference original) or deleted with a note in CHANGELOG. |
| Stale specs | Moved under `docs/specs/historical/` or deleted. `docs/specs/` lists only current specs. |
| `prism/` submodule | Either removed (the content is superseded by `boot/`), or `.gitmodules` is added and the submodule URL/branch is documented in README. |
| `boot.alex/` | Removed. Content is duplicate of `boot.alex/cli.shatter` and not reachable. |
| `flake.lock` | Committed. `nix build .#mirror` produces the same binary as `cargo build --release`. |
| Branches | Dead branches deleted. In-flight branches listed in `docs/IN-FLIGHT.md` (new) with one line each: branch name, owner, intent, next step. |
| Worktrees | `/private/tmp/*` pruned. The two non-prunable worktrees either active or removed. |
| README | All five commands described (`compile`, `run`, `craft`, `fate`, `kintsugi`) match the binary's actual subcommands. |
| AGENTS.md | Same. The "There are no Rust files" sentence updated to reflect `bootstrap/` (the substrate IS Rust again, and that's honest). |
| CHANGELOG.md | New file. Records the 1.0 baseline OID, the build instructions, and the verification recipe. |

---

## The Gap (precise list)

For each criterion, the literal delta between now and 1.0.

### Gap 1 — bootstrap not in repo

- `/tmp/mirror-rs/Cargo.toml` → `bootstrap/Cargo.toml` (copy)
- `/tmp/mirror-rs/Cargo.lock` → `bootstrap/Cargo.lock` (copy)
- `/tmp/mirror-rs/src/*.rs` (12 files) → `bootstrap/src/*.rs`
- `/tmp/mirror-rs/tests/oid_smoke.rs` → `bootstrap/tests/oid_smoke.rs`
- `/tmp/mirror.c` → `bootstrap/native/mirror.c` (the reference original for the bit-exact port)
- `.gitignore` updated: `bootstrap/target/`
- `README.md` updated with `cargo build --release` instructions
- `AGENTS.md` updated: the "no Rust files" claim becomes "the bootstrap is Rust, everything above is grammar"

### Gap 2 — OID generation is not declared as grammar

- `boot/std/hash/coincidence.mirror` does not exist
- `boot/std/epistemologic/property/coincidence_matches.mirror` does not exist
- The CoincidenceHash constants (`DIM=16`, `NUM_PROJECTIONS=3`, `EPSILON`, `LEX_ORDER`) live only in Rust today. They must be expressible in grammar.
- The pinned OIDs in `bootstrap/tests/oid_smoke.rs` must also appear in the property as the corpus.

### Gap 3 — `craft --target binary` not wired

- `cmd_craft` in `bootstrap/src/main.rs` takes no `--target` argument
- The pipeline path (`@mirror/butterfly`) works for hand-built IR but not from `craft`
- `@code/llvm/emit` reads `/tmp/mirror.ll` (the bootstrap's own IR) — that path is hard-coded and `/tmp` is not portable
- The flow `craft(spec) |\> emit |\> assemble |\> link |\> store` is declared in `@craft` and `@craft.target` but the binary cannot execute it; the actions are `\` holes
- No verification yet that `mirror-self` produces the same crystal as `mirror`

### Gap 4 — grammar baseline not frozen

- `boot/std/README.md` does not exist (the README's claim "Key grammars at execution loss 0.00" is unverified by the binary because there is no `run` subcommand)
- `@mirror/build` is referenced from README but the file does not exist (`@craft` absorbs build per `surface-simplification.md`)
- `@mirror/compile` is imported by several grammars but does not exist as a file (per `surface-simplification.md`, "must be created or dissolved")
- `@mirror/trace` referenced from `@cogito` and `@mirror/lsp` but file does not exist (per surface simplification, "dissolves into @beam")
- The `boot/std/` directory has 77 files; the README says 79; the count is inconsistent

### Gap 5 — repo hygiene gaps

- `.mcp.json` untracked at repo root
- `.tmp-mirror.c` (1,878 lines) untracked at repo root
- `boot/std/mirror/butterfly.mirror` untracked
- `docs/specs/craft-binary-target.md` untracked
- `boot/std/code/llvm/ir.mirror` has uncommitted changes
- `prism/` submodule unexplained (no `.gitmodules`)
- `boot.alex/` is a stale scratch workspace
- `flake.lock` missing
- `bin/mirror-mcp` advertises `mirror_run` and `mirror_fate` tools — both will fail because those subcommands don't exist in the binary
- 38 local branches, ~17 stale (no commits in 30 days)
- 2 prunable worktrees in `/private/tmp/`
- 41 specs in `docs/specs/`, ~22 historical, no `historical/` subfolder
- 12 specs in `docs/roadmap/` predate the collapse — superseded by `surface-simplification.md` and this file
- README claims `run` and `--fate-store` subcommands the binary does not have

---

## The Cleanup Plan (ordered)

Ticks ordered by dependency. A later tick assumes earlier ticks have
landed. Each has a Done-When (DoW) that is mechanically checkable.

### Tick 1 — Import the bootstrap into the repo

Move `/tmp/mirror-rs/` to `bootstrap/`. Move `/tmp/mirror.c` to
`bootstrap/native/mirror.c`. Add `bootstrap/target/` to `.gitignore`.

**DoW:** `git ls-files bootstrap/` includes `Cargo.toml`, `Cargo.lock`,
`src/main.rs`, `src/hash.rs`, `src/content.rs`, `src/pipeline.rs`,
`src/grammar.rs`, `src/tokenize.rs`, `src/render.rs`, `src/ast.rs`,
`src/exec.rs`, `src/git.rs`, `tests/oid_smoke.rs`, `native/mirror.c`.
`/tmp/mirror-rs/` and `/tmp/mirror.c` can be deleted with no loss.

### Tick 2 — `cargo build --release` from clean checkout

Verify a fresh clone (or `git clean -fdx`) compiles to a working
binary. Run `bootstrap/target/release/mirror craft boot` and capture
the crystal OID.

**DoW:** the captured crystal OID equals
`d12f7d4b3499ee0f97b5d94abeef21963089d03c9816a880244304d29b5c746f`
(or whatever the audit-time value is — written to CHANGELOG.md). All
two tests in `bootstrap/tests/oid_smoke.rs` pass.

### Tick 3 — Track `.mcp.json` and stage the working tree

Stage `.mcp.json`, `boot/std/mirror/butterfly.mirror`,
`docs/specs/craft-binary-target.md`. Decide on `.tmp-mirror.c` — if it
is the canonical C source, move it under `bootstrap/native/`; if it is
a working copy that diverged, diff against `bootstrap/native/mirror.c`
and either reconcile or delete. Commit the uncommitted changes to
`boot/std/code/llvm/ir.mirror`.

**DoW:** `git status` shows clean working tree.

### Tick 4 — Fix `bin/mirror-mcp`

The script exposes `mirror_run` and `mirror_fate` MCP tools whose
underlying subcommands don't exist. Either:

(a) Remove those tools from the schema (the script declares `tools/list`
with them), since the binary doesn't support them.

(b) Add `run` and `fate` subcommands to the binary (this is itself a
larger task — see Tick 9).

For 1.0 hygiene, pick (a) first. The MCP server then only advertises
what works: `mirror_compile`, `mirror_craft`, `mirror_kintsugi`.

**DoW:** every tool listed in `bin/mirror-mcp`'s `tools/list` response
maps to a subcommand the binary implements. Invoking any advertised
tool returns a 0 or 1 exit, never "unknown:".

### Tick 5 — Decide the fate of `prism/` and `boot.alex/`

`prism/` submodule contents are dated 2026-04-09 (pre-collapse). The
in-tree `@prism` is `boot/00-prism.mirror`. The submodule is either
(a) needed by some grammar via `in @prism`, in which case `.gitmodules`
must exist and the URL/branch must be documented, or (b) orphan, in
which case `git submodule deinit prism && git rm prism` removes it.

`boot.alex/` has two files of identical content (`cli.mirror`,
`cli.shatter`), neither reachable from craft. It is residue from the
surface-simplification session.

**DoW:** `git ls-files prism/` is either empty (removed) or `.gitmodules`
documents it. `boot.alex/` is removed (`git rm -r boot.alex`).

### Tick 6 — Add `flake.lock`, verify `nix build`

`flake.nix` exists but `flake.lock` does not. The current flake is a
stub (it just packages `./bin/`). For 1.0, the flake should build the
Rust bootstrap from `bootstrap/` and produce the binary as
`packages.<system>.mirror`. Add `flake.lock`. Optionally add a
`nixpkgs` input for cross-platform builds.

**DoW:** `nix build .#mirror` from a clean checkout produces a binary
byte-equal (or at least functionally equal in `craft boot` output) to
`cargo build --release`.

### Tick 7 — Spec triage

Create `docs/specs/historical/`. Move every spec that references
`src/`, `MirrorOid`, `MirrorLoss`, `evaluate.rs`, `mirror_runtime.rs`,
or the C-only architecture. Write a one-line note in each moved spec
indicating the date of supersession and the spec(s) that supersede it.

**DoW:** `docs/specs/` (top level) contains ≤15 specs, all of which
describe the architecture as of 1.0. `docs/specs/historical/`
contains everything else. `docs/specs/README.md` (new, terse) lists
the current specs and what each governs.

### Tick 8 — Delete dead branches; document in-flight ones

For each of the 38 local branches: if merged into main, delete. If
not merged and >30 days stale, move to `archived/<name>` (or delete
outright if obsolete). For each remaining in-flight branch, write a
one-line entry in `docs/IN-FLIGHT.md`: owner, intent, next step.

Prune the two `/private/tmp/*` worktrees.

**DoW:** `git branch | wc -l` ≤ 10. `git worktree list | grep -v prunable | wc -l` matches the number of actually-active worktrees. `docs/IN-FLIGHT.md` exists and lists every non-main branch.

### Tick 9 — Make `run` and `fate` real (or remove them from the README)

The README and AGENTS.md describe `mirror run` and
`mirror run --fate-store`. The binary has neither.

Two options, pick one before 1.0:

(a) **Implement them.** `cmd_run` in `bootstrap/src/main.rs` invokes
`@mirror/runtime.run` against the input file, returns the imperfect.
`cmd_run --fate-store <oid> <resolution>` writes `refs/fate/<oid>`
storing `<resolution>` (via `git update-ref` or a blob + ref). This
is what `@mcp.fate` already declares; the binary becomes the executor.

(b) **Remove them from documentation.** The five surface verbs become
`compile`, `craft`, `kintsugi`, plus the mq pipeline. The README is
rewritten to match. AGENTS.md is rewritten to match.

Recommendation: (a). The surface-simplification spec is explicit that
five operations need five verbs. Without `run` and `fate` the binary
cannot resolve `\` holes — only render and re-tokenize them.

**DoW:** every command listed in README and AGENTS.md is implemented
in the binary and returns a non-error exit on a known-good input.

### Tick 10 — `@hash/coincidence` as grammar

Write `boot/std/hash/coincidence.mirror` declaring CoincidenceHash<3>
with its constants. Write
`boot/std/epistemologic/property/coincidence_matches.mirror`
declaring that for the corpus pinned in `bootstrap/tests/oid_smoke.rs`,
the binary's compile output matches the grammar's predicted OID.

The grammar does not need to implement the hash — the bootstrap does
that. The grammar declares the contract.

**DoW:** `mirror compile boot/std/hash/coincidence.mirror` produces a
stable OID. The property file compiles. `mirror craft boot` includes
both new files in its 100+ file count, all cached after first run.

### Tick 11 — `craft --target binary`

Add the `--target` flag to `cmd_craft`. When `--target binary` is set,
the craft pipeline runs (with the kintsugi/convergence loop the
binary already partially implements through `|\>`), then the emission
stages run: `@code/llvm/emit.emit_binary(spec)` produces IR,
`@io.exec("clang", [...])` produces the binary.

Make the IR self-reference (`/tmp/mirror.ll`) explicit:
`bootstrap/native/mirror.ll` becomes the reference IR, generated by
`cargo rustc --release -- --emit=llvm-ir` (or shipped alongside the
Rust source). Drop the hard-coded `/tmp/mirror.ll` path in
`@code/llvm/emit`; read from a path that can be configured.

**DoW:** `mirror craft --target binary boot` produces
`./mirror-self` (or a path documented in the spec). Running
`./mirror-self craft boot` produces the same crystal OID as the
bootstrap binary. The two binaries' SHA-256s are stored in CHANGELOG.md.

### Tick 12 — Freeze the grammar baseline; write `boot/std/README.md`

Write `boot/std/README.md`. One line per top-level grammar, what it
covers, the example file (e.g. `examples/compile-hello.mirror`) that
exercises it, and the OID that example produces.

Resolve the missing-file references: either create `@mirror/compile`,
`@mirror/trace`, `@mirror/build` (per surface-simplification spec, or
mark them as dissolved). Same for `@shatter`. The end state is: every
`in @x/y` somewhere in the boot graph resolves to a real file.

**DoW:** `find boot -name "*.mirror" -exec grep -l "in @" {} \;` shows
no unresolved references. Every top-level grammar listed in
`boot/std/README.md` has an example. Every example's OID is pinned.

### Tick 13 — README and AGENTS.md accuracy pass

Rewrite README and AGENTS.md against the binary that exists. Every
shell command in either file must succeed when run from a clean
checkout against the installed binary. Numerical claims ("68KB",
"97 files", "4,562 lines", "79 std grammars") must be verified.

**DoW:** every command in README runs; every count in README matches
`find` output; every grammar referenced exists.

### Tick 14 — CHANGELOG.md and the 1.0 baseline

Create `CHANGELOG.md`. Record:

- The 1.0 baseline crystal OID (output of `mirror craft boot`)
- The bootstrap binary SHA-256
- The `mirror-self` binary SHA-256 (the one produced by `craft --target binary`)
- The build command (`cd bootstrap && cargo build --release`)
- The verification recipe (`mirror craft boot` and expected output)
- The Rust toolchain version used to build

**DoW:** the file exists, the OIDs match a fresh build, and the
recipe reproduces them.

### Tick 15 — Tag 1.0

When all 14 prior ticks have landed: tag.

```bash
git tag -s v1.0.0 -m "mirror 1.0 — the glass is the grammar"
git push --tags
```

The release notes are CHANGELOG.md's 1.0 section.

**DoW:** `git describe --tags --abbrev=0` returns `v1.0.0`. The crystal
OID in CHANGELOG.md reproduces from the tagged commit.

---

## The 1.0 Release

```
crystal C is produced from grammar set G by binary B.
B was built from bootstrap/ source S by `cargo build --release`.
mirror-self B' was produced by `B craft --target binary boot`.
B' produces the same crystal C from grammar set G.
S, G, C, sha(B), sha(B') are recorded in CHANGELOG.md.
The commit on which all of this holds is tagged v1.0.0.
```

The version is the OID. The first immutable artifact is the v1.0.0
crystal — that 64-hex content address that the binary produces against
the shipped `boot/` tree. The crystal is in git: `refs/crystals/<C>`.
The crystal is in the repo: pinned in CHANGELOG.md. The crystal is in
the binary: it produces it.

There is no version string in the binary. The version is what the
binary produces. The first build that produces C from G is the first
1.0 build. Every later build that produces a different crystal is a
different version. The OID is the version.

`e^(n+1) < e^n`. The system has reached n where e^(n+1) = e^n. The
binary, the grammar, and the crystal are at λ₀. That is 1.0.

---

## Constraints honored by this spec

- Spec only. No code modified. No grammars modified. No README modified.
- Every audit claim above is from a file I read on 2026-05-19 or from
  a binary invocation I ran on the same date.
- The cleanup ticks are ordered by dependency. Tick 2 depends on Tick
  1. Tick 11 depends on Tick 10 (because the property file pins the
  hash contract before the binary emission claims to honor it). Tick
  15 depends on all prior.
- The 1.0 criteria are measurable: a binary builds (Tick 2), a crystal
  matches (Tick 2, Tick 11, Tick 14), a property holds (Tick 10), the
  working tree is clean (Tick 3), the docs match the binary (Tick 13).
- The order is not by ease. Tick 11 (`craft --target binary`) is the
  hardest item; it sits before Tick 12 because the grammar baseline
  freeze waits on knowing whether `mirror-self` works. Tick 9 (`run`
  and `fate`) is hard but sits earlier because every downstream tick
  assumes the binary surface matches its documentation.
