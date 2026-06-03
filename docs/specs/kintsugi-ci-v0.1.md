# kintsugi-ci-v0.1 — kintsugi wired up in GitHub Actions, with mirror as the Actions provider

*2026-06-02. Reed + Alex + Mara. Status: load-bearing recognition; spec.*

**v0.1 of the spectral stack is the moment a public-repo author on GitHub can
add a `systemic-engineering/mirror/actions/kintsugi@v0.1` step to their
workflow, run the kintsugi loop against their corpus, and gate merges on the
verdict.** Recursive self-host: mirror's own `.github/workflows/` uses the
same published actions to gate its own kintsugi. That recursion is the proof
point.

The arc that ships v0.1: `fragmentation-mcp` (already deployed) →
`mirror-mcp` (the CLI's `kintsugi` subcommand re-exposed) →
`kintsugi-ci` (this spec). Three altitudes; one engine; one composition law.

---

## 0. The recognition in one sentence

**A GitHub Actions job is the wire altitude where the kintsugi loop becomes
an externalized merge gate.** The substrate that runs is the same
substrate the [[kintsugi-minimum-runnable]] dispatcher already binds.
The CI is not a new engine; it is the existing engine, *invoked from
outside the repo it gates*, with the verdict written back to the PR
that called it. The reproducibility / determinism bar from
[[../cicd/kintsugi-thesis]] holds across that wire by construction
(content-addressed substrate, pinned `H`-world, pinned `@fate` model OID
when the au column lands).

---

## 1. The deliverable — what shipping v0.1 means

### 1.1 The user-facing surface

A user with a public GitHub repo writes this in `.github/workflows/kintsugi.yml`:

```yaml
name: kintsugi

on:
  pull_request:
  push:
    branches: [main]

jobs:
  kintsugi:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: systemic-engineering/mirror/actions/kintsugi@v0.1
        with:
          target: src/             # path the loop walks
          threshold: 0.8           # accept verdicts at this confidence
          shatter: 4               # iteration depth
          fail-on: failure         # gate severity: failure | partial | none
```

The step:

1. Materializes the pinned mirror toolchain (a single binary plus the
   shipped boot grammars). Source: `systemic-engineering/mirror` at tag
   `v0.1.x`.
2. Walks `target`. For each `.mirror` file, runs the kintsugi loop per
   [[kintsugi-minimum-runnable]] (propose / measure / elect / verify /
   fixpoint) and per [[kintsugi-tournament]] for multi-candidate cases.
3. Composes the per-file `Transparency<PropertyVerdict>` verdicts (see
   [[../../../prism/imperfect/src/transparency]]) into a single workflow
   verdict: `pass | partial(min_confidence) | fail`.
4. Emits the verdict three ways:
   - **Stdout** as **stringified mirror AST** by default — a
     blank-line-separated sequence of `<key> <value>` records (the
     substrate-pull-correct shape per §1.4). JSON is the @io boundary
     and lives behind `--format=json`.
   - **GitHub step output** (`outputs.verdict`, `outputs.confidence`,
     `outputs.objective`) for downstream steps. The action's `run.sh`
     invokes `mirror kintsugi --ci --format=json` and pipes through
     `jq` to translate at the @io boundary.
   - **PR comment** (one per loop run, replacing the previous) with the
     `variety_loss` breakdown per glass / per op (see
     [[kintsugi-variety]] §6) — the visible-repair part of the kintsugi
     metaphor, made literal.
5. Exits 0 on `pass`, 0 on `partial` (unless `fail-on: partial`), nonzero
   on `fail`. The exit determines whether the workflow gate is green.

That is what v0.1 ships. Anything richer (custom strategies, multi-target,
reflected commits) is post-v0.1 and called out in §11.

### 1.2 The concrete artifacts

```
systemic-engineering/mirror (tagged v0.1.0)
├── actions/
│   └── kintsugi/
│       ├── action.yml          # composite action; see §5
│       └── README.md           # input reference + examples
├── .github/
│   └── workflows/
│       ├── kintsugi.yml        # recursive self-host (§6)
│       ├── ci.yml              # current quality gate (unchanged shape)
│       └── release.yml         # tags v0.1.x; cuts artifacts (§7)
├── bootstrap/                  # the mirror binary; per road-to-1.0 Tick 1
├── boot/                       # the shipped grammar baseline
└── ...
```

At the same tag, three things must be true:

- **The action resolves.** `uses: systemic-engineering/mirror/actions/kintsugi@v0.1`
  picks up `actions/kintsugi/action.yml` at that tag.
- **The action runs.** Against a fixture corpus (see §1.3), the action
  produces the documented verdict shape and exit code.
- **Mirror gates itself with it.** Mirror's `.github/workflows/kintsugi.yml`
  uses `./actions/kintsugi` (local path) on PRs, and that gate is
  required by branch protection on `main`.

### 1.3 The fixture corpus and the cut criterion

There are exactly **two** fixture corpora pinned by v0.1:

| Fixture | Source | Expected verdict | Purpose |
|---|---|---|---|
| `fixtures/kintsugi-pass/` | three `.mirror` files; all bodies concrete | `pass`, confidence 1.0 | Smoke. Action exits 0; gate green. |
| `fixtures/kintsugi-partial/` | three `.mirror` files; one body parked at `\` | `partial`, confidence ≥ 0.6, one located opacity | Demonstrates the located-opacity surface from [[../../../prism/imperfect/src/transparency]]. |

The v0.1 cut criterion: **on both fixtures, on the
`ubuntu-latest`-of-cut-date GitHub runner, the action produces the
verdict above with byte-identical `outputs.objective` across three
consecutive runs.** Determinism is verified across the wire by the
cut. Reproducibility across machines (the [[../cicd/kintsugi-thesis]]
Claim 1 + Claim 8 bar) is verified by also running the same fixtures
locally via `just kintsugi-ci-local fixtures/kintsugi-pass` and
comparing the objective.

The action is **not** required to handle every grammar in the wild at
v0.1. It is required to run the loop deterministically and emit the
verdict shape. The wider grammar coverage is a follow-on; the wire is
the deliverable.

### 1.4 The substrate-pull correction (T11.2.5)

T11.2 + T11.3 originally shipped JSON as the default `--ci` output
format. That was wrong. `@io` crossings are decoherence events; the
substrate stays in mirror as long as possible per
[[kintsugi-variety]] §3. The default wire format is the stringified
mirror AST — a blank-line-separated sequence of `<key> <value>`
records, lossless and human-readable. JSON appears only at the @io
boundary — i.e., in the action's `run.sh` when GitHub Actions needs to
set `$GITHUB_OUTPUT`.

#### Default (mirror-text) shape — single file

```text
verdict    success
target     "boot/std/nl.mirror"
objective  0.0
iterations 1
dark_count 0
```

#### Default (mirror-text) shape — corpus

Aggregate record first; one blank-line-separated record per file,
sorted by path. Per-file records key on `file` (not `target`):

```text
verdict          partial
target           "bootstrap/tests/fixtures/kintsugi-partial"
objective        3.0
iterations       1
dark_count       3
files_processed  1

file         "bootstrap/tests/fixtures/kintsugi-partial/dark.mirror"
verdict      partial
objective    3.0
iterations   1
dark_count   3
```

#### JSON shape (the @io boundary)

Invoked via `mirror kintsugi --ci --format=json <target>`. Same field
set; identical aggregation rules. The action's `run.sh` is the only
place this should appear.

#### Action's `run.sh` design (T11.4)

The action runs the substrate-native path, then crosses to JSON only
at the point of `$GITHUB_OUTPUT`:

```bash
#!/usr/bin/env bash
set -euo pipefail
# Substrate-native verdict (mirror-text); kept around for debug / artifact upload.
mirror kintsugi --ci --shatter "$SHATTER" "$TARGET" > /tmp/kintsugi-verdict.mirror
# @io crossing: same verdict, JSON-shaped, for jq + $GITHUB_OUTPUT.
mirror kintsugi --ci --format=json --shatter "$SHATTER" "$TARGET" > /tmp/kintsugi-verdict.json
jq -r '"verdict=" + .verdict, "objective=" + (.objective|tostring)' \
   /tmp/kintsugi-verdict.json >> "$GITHUB_OUTPUT"
```

The double invocation is cheap (the kintsugi loop is deterministic and
cached at the OID layer) and keeps the substrate artifact available
for download as a workflow artifact. The single-invocation path is
also valid (`--format=json` only) for runners that don't want the
substrate-native artifact.

### 1.5 The substrate-pull closure (T11.2.6)

T11.2.5 shipped the right wire shape (mirror-text key-value records)
but the substrate didn't yet know what a verdict IS — it just emitted
text the tokenizer happened to parse as identifiers. The form was
substrate-pull-aligned; the meaning wasn't yet.

T11.2.6 closes the loop by declaring `verdict`, `verdict_entry`, and
`corpus_verdict` as typed records in `boot/std/kintsugi.mirror`,
plus the `discrimination = success | partial | failure` enum that
names the three positions:

```mirror
type discrimination = success | partial | failure

type verdict = {
  verdict: discrimination,
  target: text,
  objective: f64,
  iterations: u64,
  dark_count: u64,
}

type verdict_entry = {
  file: text,
  verdict: discrimination,
  objective: f64,
  iterations: u64,
  dark_count: u64,
}

type corpus_verdict = {
  verdict: discrimination,
  target: text,
  objective: f64,
  iterations: u64,
  dark_count: u64,
  files_processed: u64,
  per_file: [verdict_entry],
}
```

The Rust emitter's wire shape is **unchanged** from T11.2.5 — the
canonical mirror-text record `<key> <value>` form is precisely the
canonical instance form of these typed records. What T11.2.6 adds is
the substrate-authoritative declaration of the type those records
belong to. The substrate now knows what a verdict IS.

The load-bearing invariant: **the field set declared in
`boot/std/kintsugi.mirror` matches the field set the Rust emitter
writes, byte-equal at the key level.** Drift on either side breaks
the round-trip tests in
`bootstrap/tests/kintsugi_ci_typed_verdict.rs` — that's the gate.
The action's `run.sh` design from §1.4 is unchanged: mirror-text in
the substrate, JSON only at the `@io` boundary.

---

## 2. Current state map

### 2.1 What already exists

| Piece | Location | State |
|---|---|---|
| The kintsugi loop's mathematical objective | [[kintsugi-variety]] §1 | Spec; `variety_hold` property declared; implementation deferred to property bodies. |
| The tournament merge mechanism | [[kintsugi-tournament]] §1 | Spec; detect → enumerate → score → eliminate → apply; first concrete consumer is the file-system collision case. |
| The minimum-runnable engine | [[kintsugi-minimum-runnable]] §1–§6 | Spec + ticks A–F. Tick A (dispatcher), Tick B (substrate add), Tick C (rename through engine) are the foundation this CI rides. |
| The `kintsugi` CLI subcommand | `bootstrap/src/main.rs::cmd_kintsugi` | Lives in mirror's bootstrap binary per [[mirror-binary-architecture]]. Today: reads, tokenizes, renders canonical. The §9 ticks of minimum-runnable wire it to the engine. |
| The pq wire altitude | [[../../../prism/docs/specs/pq]] §0, §2 | Spec; `focus / project / refract` over the `Prism` trait. The `imperfect` channel carries the variety verdict ([[kintsugi-variety]] §13). T11.1 landed this. |
| The fragmentation-mcp server | `/Users/alexwolf/dev/projects/fragmentation/docs/specs/fragmentation-mcp.md` (referenced by [[../../../prism/docs/specs/pq]]) | Deployed; the substrate that holds content-addressed state. Provides the `H`-world the kintsugi engine operates over. |
| The `Transparency` / `PropertyVerdict` verdict carrier | `/Users/alexwolf/dev/projects/prism/imperfect/src/transparency.rs` | Landed. The Fail-dominates / Partial-min-confidence / Pass-neutral merge is tested. |
| The existing CI workflow | `/Users/alexwolf/dev/projects/mirror/.github/workflows/ci.yml` | 668 B; calls `systemic-engineering/ci/actions/nix-setup@main` then `nix develop -c just check`. The shape v0.1 evolves. |
| The Justfile recipes | `/Users/alexwolf/dev/projects/mirror/Justfile` | `format`, `pre-commit`, `pre-push`, `build`, `install`. The local-parity surface CI mirrors. |
| The `systemic-engineering/ci` reusable workflows | `/Users/reed/dev/projects/ci/` | Elixir-focused. Provides `nix-setup`, `check.yml`, `notify-ntfy.yml`. The Nix-setup composite IS used by today's mirror CI. The Elixir actions DO NOT carry over. |

### 2.2 What needs to be built

| Piece | Where it lives | Tick |
|---|---|---|
| `mirror kintsugi --ci` flag | `bootstrap/src/main.rs::cmd_kintsugi` | T11.2 |
| Verdict-as-JSON serialiser | `bootstrap/src/main.rs` + `boot/std/kintsugi.mirror` | T11.2 |
| Per-walk recursive driver | `bootstrap/src/main.rs::cmd_kintsugi` (`--target <dir>`) | T11.3 |
| Substrate-pull correction: mirror-text default + `--format=json` | `bootstrap/src/main.rs::emit_*` | T11.2.5 |
| Typed `verdict`/`verdict_entry`/`corpus_verdict` records in substrate | `boot/std/kintsugi.mirror` | T11.2.6 |
| `actions/kintsugi/action.yml` | `mirror/actions/kintsugi/` | T11.4 |
| Fixture corpora | `mirror/fixtures/kintsugi-pass/`, `mirror/fixtures/kintsugi-partial/` | T11.4 |
| Recursive self-host workflow | `mirror/.github/workflows/kintsugi.yml` | T11.5 |
| Release workflow + tagging | `mirror/.github/workflows/release.yml` | T11.6 |
| `v0.1` floating tag pointer | git refs | T11.7 |

Nothing here invents a new substrate. Every item is glue between
existing altitudes.

### 2.3 What `ci/` carries over and what doesn't

`/Users/reed/dev/projects/ci/` is a Reed-curated Elixir CI library. It
is the right shape for *Elixir* projects with `mix check` as the
quality gate. Mirror is not Elixir. The carry-over surface is narrow:

**Carries:**
- The `nix-setup` composite action (`ci/actions/nix-setup/action.yml`).
  Mirror's existing CI already uses it; v0.1 keeps it.
- The Justfile recipe convention (`format / pre-commit / pre-push`).
  Mirror already follows it.
- The OBC mapping in `ci/README.md` (Observable / Budget / on_pass /
  on_fail). The kintsugi action IS the budget; the verdict IS the
  pass/fail.
- The `notify-ntfy.yml` reusable workflow for `on_fail` cascades.
  Optional; pulled in by mirror's `.github/workflows/kintsugi.yml`
  *if* a Reed-side notification is wanted.
- The local/CI parity discipline ([[/Users/reed/dev/projects/ci/WORKFLOW.md]]).
  Mirror's `just kintsugi-ci-local` recipe is the parity primitive
  (per §5.3).

**Does NOT carry:**
- `elixir-setup`, `elixir-test`, `elixir-quality`, `hex-publish`,
  `docs-check.yml`, `elixir-ci.yml`, `elixir-matrix.yml` — Elixir-
  specific.
- The hex.pm / HexDocs / GitHub Pages docs pipeline — out of scope.
- The Alpine / container matrix — mirror has one runtime (the binary);
  no matrix.

**Does NOT obviate this spec.** `ci/` is the right template for the
*Nix-setup outer shell* of mirror's CI. The kintsugi action itself is
net new; the verdict shape, the fixture corpora, the recursive self-
host, and the Actions package shape are not in `ci/`. The Elixir
composite-action layout (`actions/<name>/action.yml`) is the right
shape to inherit — but the content is new.

---

## 3. The architecture

### 3.1 The three altitudes

```
  ┌─────────────────────────────────────────────────────────────┐
  │  Wire altitude — kintsugi-ci                                │
  │    actions/kintsugi/action.yml                              │
  │    .github/workflows/kintsugi.yml                           │
  │    GitHub Actions runner, sandboxed, ephemeral              │
  └────────────────────────┬────────────────────────────────────┘
                           │ invokes
                           ▼
  ┌─────────────────────────────────────────────────────────────┐
  │  Binary altitude — the mirror CLI                           │
  │    mirror kintsugi --ci --target <dir> --shatter <N>        │
  │    (lives in bootstrap/ per mirror-binary-architecture.md)  │
  │    Calls the dispatcher; emits imperfect-shaped JSON        │
  └────────────────────────┬────────────────────────────────────┘
                           │ binds
                           ▼
  ┌─────────────────────────────────────────────────────────────┐
  │  Substrate altitude — the kintsugi engine                   │
  │    Crystallizations<H> dispatcher (kintsugi-minimum-runnable)│
  │    @kintsugi/fracture/*, @epistemologic/property/*          │
  │    @fate.infer (when the au column lands; v0.1 ships without)│
  │    Verdict composition via Transparency<PropertyVerdict>    │
  └─────────────────────────────────────────────────────────────┘
```

The wire altitude is *thin*. It is `actions/checkout`, install the
mirror binary, run `mirror kintsugi --ci ...`, format stdout, post the
verdict to GitHub. Every operation that touches substrate happens at
the binary altitude through the dispatcher.

### 3.2 Where each existing spec lands

- **[[mirror-binary-architecture]]** — names where the CLI's kintsugi
  lives (`bootstrap/`). The action does not bundle a runtime; it
  unpacks the bootstrap binary that mirror itself ships.
- **[[kintsugi-minimum-runnable]]** — names the dispatcher Tick A, the
  fracture substrate Tick B, the engine-run-on-corpus Tick C. The
  CI's `--target <dir>` invokes the engine over the corpus exactly
  the way Tick C invokes it over `boot/`. Same code path; different
  target.
- **[[kintsugi-tournament]]** — the multi-candidate case. v0.1 ships
  with the single-candidate engine; multi-candidate tournaments fire
  when a fracture's `enumerate` returns >1 strategy (file-system
  collisions, near-identical declarations). The CI's verdict shape
  is invariant; whether one or many candidates ran is opaque to the
  wire.
- **[[kintsugi-variety]]** — the `variety_hold` property is what the
  verdict reflects. The action's `outputs.objective` is the composed
  `kintsugi_objective` from §6 of that spec. When the @fate column
  lands, the @io crossing minimization becomes a runtime concern
  inside the engine; the wire shape doesn't change.
- **[[kintsugi-formatter]] / [[kintsugi-shatter]]** — the iteration
  rule and shatter-N depth. The action's `shatter: <N>` input maps
  directly to `mirror kintsugi --shatter N`.
- **[[kintsugi-self-hosting]]** — names the broader mirror-self goal.
  v0.1 does NOT require `craft --target binary` self-hosting; the
  action ships the bootstrap binary. The self-hosting is mirror's
  v1.0 cut, not v0.1.
- **[[road-to-1.0]]** — names the bigger v1.0. v0.1 is *strictly less*
  than v1.0. Specifically: v0.1 does not require the mirror binary to
  rebuild itself, does not require `craft --target binary`, does not
  require @fate to be wired. It requires only that the engine run
  deterministically across the wire on the two fixture corpora and
  that mirror gate itself with the same action. v1.0 adds the
  self-host of the binary; v0.1 ships the self-host of the *CI gate*.
- **[[../../../prism/docs/specs/pq]]** — the verdict shape that crosses
  the wire IS the `imperfect` channel from pq §2.4. The action's
  stdout is structurally identical to a pq response carrying an
  `imperfect` verdict over the corpus walk.

### 3.3 How the fragmentation-mcp / mirror-mcp / kintsugi-ci chain composes

```
fragmentation-mcp  (substrate altitude, persistent server)
        ↑
        │ holds content-addressed shards; Splinter<H> store
        │
mirror-mcp        (binary altitude, request/response)
        ↑
        │ mirror CLI re-exposed as MCP tools; `mirror_kintsugi`
        │ is one of them. Resolves substrate refs through frgmnt.
        │
kintsugi-ci       (wire altitude, ephemeral runner)
        ↑
        │ GitHub Actions step; thin wrapper over `mirror kintsugi`.
        │ Does not persist state; the verdict is the side effect.
```

v0.1's CI **does not require fragmentation-mcp to be reachable from
the runner**. The runner is sandboxed; it ships its own substrate
(the boot grammars, the dispatcher, the action body bindings) in the
bootstrap binary. fragmentation-mcp is the *durable* substrate; the
CI ephemeral substrate is a snapshot taken at install time. The two
stay consistent because they share content addressing: a verdict
computed on the runner is byte-identical to a verdict computed
locally against fragmentation-mcp on the same inputs.

When the @fate column lands (post-v0.1), the runner *will* need
either a model snapshot bundled in the action or a network reach to
a pinned model OID. Per [[../cicd/kintsugi-thesis]] Claim 2, the
local-by-construction invariant on `@fate` is the architectural
defense; the operational instance for the CI runner is a deferred
question (§11).

---

## 4. The chain to v0.1 — tick decomposition

T11.1 just landed (the pq wire altitude per
[[../../../prism/docs/specs/pq]]). The chain from there to v0.1
shipped is **seven ticks**:

### T11.2 — `mirror kintsugi --ci` flag and JSON verdict serialiser

- **Scope:** add the `--ci` flag to `cmd_kintsugi`. When set, after
  the loop runs, serialise the final `Imperfect<Splinter<H>,
  CrystallizeError, Transparency<Ref>>` carrier as JSON to stdout in
  the `imperfect`-channel shape from [[../../../prism/docs/specs/pq]]
  §2.4. No PR-comment formatting at this tick; pure stdout.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]` on the JSON serialiser
  shape; the schema is a substrate-altitude obligation, not a Rust
  invention.
- **Verification:** `mirror kintsugi --ci fixtures/kintsugi-pass/foo.mirror`
  emits valid JSON matching the schema in §1.1. `jq '.verdict'`
  returns `"pass"`.
- **Artifact:** `bootstrap/src/main.rs` updated; one new
  `bootstrap/tests/ci_verdict.rs` test file pinning the schema.

### T11.3 — `--target <dir>` walks the corpus

- **Scope:** add `--target <dir>` to `cmd_kintsugi` so it walks a
  directory of `.mirror` files (sorted by content-OID per
  [[../cicd/kintsugi-thesis]] Claim 8). For each file, run the loop;
  compose verdicts via `Transparency::combine`. Emit one composite
  verdict (not per-file).
- **Marker:** 🔴/🟢 `[substrate-pull:realize]`.
- **Verification:** `mirror kintsugi --ci --target fixtures/kintsugi-pass/`
  composes three per-file verdicts into one `pass`. Same flag against
  `fixtures/kintsugi-partial/` produces `partial` with one located
  opacity at the parked-body file.
- **Artifact:** `bootstrap/src/main.rs` walker + `fixtures/`
  directories populated.

### T11.2.5 — substrate-pull correction: mirror-text default + `--format=json`

- **Scope:** correct T11.2 + T11.3 which shipped JSON as the default
  `--ci` output. Make stringified mirror AST the default; gate JSON
  behind `--format=json`. The aggregation rules and field set are
  unchanged; only the *output format* changes. See §1.4 for the
  shapes and the action's `run.sh` design.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]`. The default-format
  choice is a substrate obligation; the JSON path is the @io
  boundary, isolated to `--format=json`.
- **Verification:** `mirror kintsugi --ci <file>` emits a mirror-text
  record (no leading `{`); `mirror kintsugi --ci --format=json <file>`
  emits the T11.2 JSON envelope verbatim. Both shapes deterministic
  across runs. `bootstrap/tests/kintsugi_ci.rs` and
  `bootstrap/tests/kintsugi_ci_target.rs` retrofit to assert the
  mirror-text default while keeping the JSON subset behind
  `--format=json`.
- **Artifact:** `bootstrap/src/main.rs` (`CiFormat` enum, two
  emitters, CLI flag); the two test files; this spec section.

### T11.4 — `actions/kintsugi/action.yml` composite action

- **Scope:** write `mirror/actions/kintsugi/action.yml` as a Composite
  action (see §5 for the argument). Inputs: `target`, `threshold`,
  `shatter`, `fail-on`. Steps: install the mirror binary (download
  the release artifact for the runner's architecture), run
  `mirror kintsugi --ci --target <target> --shatter <shatter>`, parse
  stdout, write step outputs, exit appropriately. PR-comment
  posting via `actions/github-script` is part of this tick.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]` on the action shape
  (a substrate-pull because the action is YAML, not Rust, and the
  shape is locked).
- **Verification:** the action.yml validates with `actionlint`. The
  action runs successfully on the two fixture corpora through
  `act` (the local GitHub Actions runner) with byte-identical
  `outputs.objective` across three runs.
- **Artifact:** `mirror/actions/kintsugi/action.yml`,
  `mirror/actions/kintsugi/README.md`, `mirror/fixtures/kintsugi-{pass,partial}/`.

### T11.5 — recursive self-host: `mirror/.github/workflows/kintsugi.yml`

- **Scope:** write the workflow that uses `./actions/kintsugi` (local
  path) against `boot/` and reports the verdict. Branch protection on
  `main` requires the workflow's `kintsugi` job to pass.
- **Marker:** 🟢. The wiring is mechanical; the test is the workflow
  passing on the substrate snapshot at the tick's commit.
- **Verification:** open a PR against `main` (a no-op PR is fine).
  The `kintsugi` check appears, runs, and passes. Repeat with a PR
  that introduces a parked body; verify the check correctly reports
  `partial` and that the gate's policy (`fail-on: partial`) blocks
  merge.
- **Artifact:** `mirror/.github/workflows/kintsugi.yml`.
  `mirror/.github/CODEOWNERS` updated if needed for branch protection.

### T11.6 — release workflow: build, sign, attach binary, tag

- **Scope:** write `mirror/.github/workflows/release.yml`. Triggered
  on `v0.1.*` tags. Steps: matrix build the bootstrap binary for
  `{x86_64, aarch64} × {linux, darwin}`, compute SHA-256 for each,
  attach as release artifacts, write the release notes from
  `CHANGELOG.md`. The release artifacts are what
  `actions/kintsugi/action.yml` downloads at runtime.
- **Marker:** 🔴/🟢 `[substrate-pull:realize]` on the artifact naming
  scheme (substrate-locked so the action knows where to fetch from).
- **Verification:** cut a `v0.1.0-rc.1` tag in a fork. The release
  workflow produces four binaries with documented SHA-256s. The
  action.yml's install step (against the rc tag) downloads and
  verifies them. The two-fixture verdict matches the pre-release
  baseline.
- **Artifact:** `mirror/.github/workflows/release.yml`, `CHANGELOG.md`
  updated, `actions/kintsugi/action.yml` install logic finalised.

### T11.7 — cut `v0.1.0`, point `v0.1` floating tag

- **Scope:** tag `v0.1.0` from the commit on which T11.5 + T11.6
  ship green. Create the `v0.1` floating tag pointing at `v0.1.0`.
  (Resolution policy in §7.) Update branch protection on `main` so
  the `kintsugi` job is required. Update README to advertise
  `@v0.1`.
- **Marker:** 🟢. This is a tag.
- **Verification:** an external repo can add `uses:
  systemic-engineering/mirror/actions/kintsugi@v0.1` to its workflow
  and the action resolves, downloads, runs, and reports a verdict on
  its first PR. **Mirror's own kintsugi job, on a PR cut after the
  tag, runs via the published action (not the local path) and
  passes.** That second clause is the recursive self-host's first
  observable proof.
- **Artifact:** `v0.1.0` and `v0.1` git tags; updated README; updated
  branch protection.

### T11.8 (post-release) — convert internal call site to published action

- **Scope:** the workflow from T11.5 used `./actions/kintsugi`. Switch
  to `systemic-engineering/mirror/actions/kintsugi@v0.1`. This is
  what closes the recursive self-host: mirror now uses its own
  *published* action to gate its own development. (See §6.)
- **Marker:** 🟢.
- **Verification:** the kintsugi check on the PR for this tick passes,
  using the v0.1-resolved action against the substrate at the PR's
  HEAD. The eⁿ⁺¹ ≤ eⁿ property holds: the new substrate's verdict
  has not regressed against `main`'s.
- **Artifact:** `mirror/.github/workflows/kintsugi.yml` updated.

**Total: 7 ticks from T11.1 to v0.1 shipped.** Including T11.8 (the
recursive self-host *via published action*), 8 ticks.

T11.2 ↔ T11.3 ↔ T11.4 are sequential. T11.5 depends on T11.4. T11.6
can land in parallel with T11.5 (release artifacts are independent
of the workflow that consumes them). T11.7 depends on T11.5 + T11.6.
T11.8 depends on T11.7.

---

## 5. The GitHub Actions package shape

### 5.1 Why Composite, not Docker, not JavaScript

Three action shapes are possible. The case for each:

| Shape | Pro | Con | Verdict |
|---|---|---|---|
| **Composite** | Pure YAML; transparent steps; users can fork & audit; runs on the host runner (full caching of toolchain installs); no container build to maintain | Each step is a shell process; some plumbing around outputs | **Pick.** Matches mirror's local/CI parity discipline ([[/Users/reed/dev/projects/ci/WORKFLOW.md]]): the action's steps are the same shell commands `just kintsugi-ci-local` runs. |
| **Docker** | Hermetic by container | Requires building & publishing an image; Linux-only on most runners; loses local/CI parity (the local recipe doesn't run in the container); harder to debug | Reject. Mirror's binary is a single static-ish artifact already; we don't need an image to ship it. |
| **JavaScript** | Fast startup; rich GH API access | Requires bundled Node toolchain; adds a TypeScript/JS substrate to the kintsugi pipeline that doesn't exist anywhere else in the stack | Reject. The action's logic is shell-shaped (download, run, parse). JS would be overkill and a foreign substrate. |

Composite wins on three grounds: (1) local/CI parity (the action's
shell steps are the same shell `just kintsugi-ci-local` runs), (2) no
new substrate (mirror has no JS / no container image; adding one for
the action would be capability growth at the wrong altitude), (3)
transparency (the action is one short YAML file users can read).

The one tradeoff: composite actions don't have step `id`-scoped output
plumbing as clean as JS actions. We handle this with one shell step
that parses stdout via `jq` and writes `$GITHUB_OUTPUT`.

### 5.2 `actions/kintsugi/action.yml`

```yaml
name: 'mirror kintsugi'
description: 'Run the kintsugi loop over a corpus; emit a typed verdict.'
author: 'Reed <reed@systemic.engineer>'

inputs:
  target:
    description: 'Path the loop walks (file or directory).'
    required: true
  threshold:
    description: 'Acceptance threshold for partial verdicts (0.0–1.0).'
    required: false
    default: '0.8'
  shatter:
    description: 'Shatter-N iteration depth (per kintsugi-shatter spec).'
    required: false
    default: '4'
  fail-on:
    description: 'Severity that fails the gate: failure | partial | none.'
    required: false
    default: 'failure'
  mirror-version:
    description: 'Override the mirror binary version. Default: this action tag.'
    required: false
    default: ''
  post-comment:
    description: 'Post a PR comment with the verdict breakdown.'
    required: false
    default: 'true'

outputs:
  verdict:
    description: 'pass | partial | fail'
    value: ${{ steps.kintsugi.outputs.verdict }}
  confidence:
    description: 'Numeric confidence in [0.0, 1.0]'
    value: ${{ steps.kintsugi.outputs.confidence }}
  objective:
    description: 'The kintsugi_objective scalar (see kintsugi-variety §6)'
    value: ${{ steps.kintsugi.outputs.objective }}
  opacities:
    description: 'JSON array of located opacities (the gold-filled cracks)'
    value: ${{ steps.kintsugi.outputs.opacities }}

runs:
  using: 'composite'
  steps:
    - name: Install mirror binary
      shell: bash
      run: |
        ${{ github.action_path }}/bin/install-mirror.sh \
          "${{ inputs.mirror-version || github.action_ref }}"

    - name: Run kintsugi
      id: kintsugi
      shell: bash
      run: |
        ${{ github.action_path }}/bin/run-kintsugi.sh \
          --target "${{ inputs.target }}" \
          --threshold "${{ inputs.threshold }}" \
          --shatter "${{ inputs.shatter }}" \
          --fail-on "${{ inputs.fail-on }}"

    - name: Post PR comment
      if: inputs.post-comment == 'true' && github.event_name == 'pull_request'
      uses: actions/github-script@v7
      with:
        script: |
          const verdict = '${{ steps.kintsugi.outputs.verdict }}';
          const objective = '${{ steps.kintsugi.outputs.objective }}';
          const opacities = JSON.parse('${{ steps.kintsugi.outputs.opacities }}');
          // body assembly + sticky-comment replacement
          // ... (delegated to a helper module shipped with the action)

branding:
  icon: 'check-circle'
  color: 'yellow'
```

The two `bin/*.sh` scripts are shipped inside the action directory.
`install-mirror.sh` resolves the release artifact for the runner's
OS/arch, downloads it from the corresponding GitHub Release
(matching `inputs.mirror-version`), verifies its SHA-256 against
the checksum file from the release, and places it on `PATH`.
`run-kintsugi.sh` invokes the binary, captures stdout, parses the
`imperfect`-channel JSON with `jq`, writes `$GITHUB_OUTPUT`, and
exits per `--fail-on`.

### 5.3 Local parity recipe

A new Justfile recipe in `mirror/Justfile`:

```just
# Run the kintsugi-ci action's logic locally against a target.
# Same shell commands the action runs; same verdict shape.
# Substrate-native artifact + JSON for the gate check.
kintsugi-ci-local target shatter='4' threshold='0.8' fail_on='failure':
    {{MIRROR_BIN_RELEASE}} kintsugi --ci \
        --shatter {{shatter}} \
        {{target}} \
        | tee /tmp/kintsugi-verdict.mirror
    {{MIRROR_BIN_RELEASE}} kintsugi --ci --format=json \
        --shatter {{shatter}} \
        {{target}} \
        > /tmp/kintsugi-verdict.json
    @jq -e --arg fail_on {{fail_on}} \
        'if .verdict == "failure" then false
         elif .verdict == "partial" and $fail_on == "partial" then false
         else true end' /tmp/kintsugi-verdict.json
```

Local/CI parity bar: `just kintsugi-ci-local fixtures/kintsugi-pass`
and the CI's `kintsugi` job produce byte-identical
`/tmp/kintsugi-verdict.json` (modulo paths normalised to repo-
relative). This is the operational version of
[[../cicd/kintsugi-thesis]] Claim 1 + Claim 8 at the wire altitude.

### 5.4 Reusable workflow (deferred)

A reusable workflow `mirror/.github/workflows/kintsugi-gate.yml` that
other repos could call with `uses:
systemic-engineering/mirror/.github/workflows/kintsugi-gate.yml@v0.1`
is *deferred to v0.2*. The composite action covers the v0.1 surface
and keeps the deliverable small. The reusable-workflow pattern
(matching `systemic-engineering/ci/.github/workflows/check.yml`) is a
clean follow-on once the action stabilises.

---

## 6. Recursive self-host

### 6.1 Pre-v0.1: mirror gates itself via the local action path

Before the v0.1 tag exists, mirror's workflow uses the in-repo action
directly:

```yaml
# mirror/.github/workflows/kintsugi.yml (T11.5)
name: kintsugi

on:
  pull_request:
  push:
    branches: [main]

jobs:
  kintsugi:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./actions/kintsugi         # local path; pre-v0.1
        with:
          target: boot/
          shatter: 4
          threshold: 0.8
          fail-on: failure
```

This is the bootstrap: the action exists but is not yet published; the
workflow exercises it against `boot/`; branch protection on `main`
requires the `kintsugi` job.

### 6.2 Post-v0.1: mirror gates itself via the published action

At T11.8, the workflow flips to the published reference:

```yaml
# mirror/.github/workflows/kintsugi.yml (T11.8, post-v0.1)
jobs:
  kintsugi:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: systemic-engineering/mirror/actions/kintsugi@v0.1   # published
        with:
          target: boot/
          shatter: 4
          threshold: 0.8
          fail-on: failure
```

**This is the recursion.** Mirror's gate on its own development
resolves the action by tag, downloads its own bootstrap binary,
runs the kintsugi loop, and reports back. A change to the action
requires a new tag, which requires the existing action to pass on
the PR that cuts it. The substrate-pull discipline becomes
structural: the action cannot regress because the gate it must
pass *is itself*.

### 6.3 The boundary case: changing the action.yml itself

When a PR changes `actions/kintsugi/action.yml`, the workflow
continues to use `@v0.1` (the previous version's action). The PR's
changes to the action are not exercised by the gate; the gate is
exercised by the *previous* version. This is the right default:
the action's own evolution does not pre-empt the gate, and the
feedback loop stays one-tick-delayed (a v0.1.1 tag re-runs the gate
with the new action, gating subsequent PRs).

For PRs that explicitly want to test the new action against itself,
*two* workflow jobs are added — `kintsugi-v0.1` (published) and
`kintsugi-head` (local path). Branch protection requires only
`kintsugi-v0.1`; the `kintsugi-head` job is informational. This is
the right shape for action-evolution PRs.

---

## 7. Versioning

### 7.1 What `@v0.1` resolves to

`@v0.1` is a **floating tag** pointing at the most recent
`v0.1.*` release tag. Resolution order:

1. `v0.1.0` is cut at T11.7. The `v0.1` tag is created pointing at
   the same commit.
2. `v0.1.1`, `v0.1.2`, ... are cut as substrate or action changes
   land. Each release advances the `v0.1` tag.
3. `v0.2.0` (when it comes) does **not** advance `v0.1`. Consumers
   on `@v0.1` stay on the v0.1 line.

This matches the GitHub Actions convention
(`actions/checkout@v4` floats over `v4.x.y`). The semver discipline:
v0.1 releases are bug fixes and backward-compatible additions to the
inputs/outputs; breaking changes to inputs/outputs cut a v0.2.

### 7.2 The cut criterion

At T11.7, the cut criterion is:

1. The two fixture corpora (§1.3) produce the expected verdicts
   with byte-identical `outputs.objective` across three consecutive
   runs on `ubuntu-latest`.
2. Mirror's own `.github/workflows/kintsugi.yml` (pre-T11.8, using
   `./actions/kintsugi`) is green on the commit being tagged.
3. The release workflow (T11.6) successfully produces all four
   target binaries with documented SHA-256s, and the action's
   `install-mirror.sh` script correctly resolves and verifies them.
4. `CHANGELOG.md` has a v0.1.0 section describing the deliverable
   surface, the fixture baseline OIDs, and the bootstrap binary
   SHA-256s.

The cut is a `git tag -s v0.1.0` followed by `git tag v0.1` from
the same commit, then `git push --tags`. The release workflow
fires; the floating tag becomes resolvable.

### 7.3 Connection to road-to-1.0

[[road-to-1.0]] defines v1.0 as: bootstrap in-repo (Tick 1), OID
stability declared in grammar (Tick 10), `craft --target binary`
self-host (Tick 11), grammar baseline frozen (Tick 12), repo hygiene
(Tick 15), tag.

**v0.1 ⊂ v1.0.** Specifically, v0.1 requires:

- Bootstrap in-repo. **Yes** — `actions/kintsugi/action.yml`
  installs the bootstrap binary from a release artifact; the binary
  must build from `bootstrap/`. (Road-to-1.0 Tick 1.)
- Working tree clean. **Yes** — required for any release tag cut.
  (Road-to-1.0 Tick 3.)
- `mirror kintsugi` works. **Yes, with the CI hooks** — T11.2 and
  T11.3 add `--ci` and `--target`. (Road-to-1.0 Tick 11 is broader.)

v0.1 does NOT require:

- `craft --target binary` (road-to-1.0 Tick 11) — the action ships
  the bootstrap binary directly; no self-host of the binary needed
  to ship the CI gate.
- The full v1.0 grammar baseline (road-to-1.0 Tick 12) — v0.1
  ships against whatever `boot/` is at the cut commit; the cut
  records the OID.
- `@hash/coincidence` as grammar (road-to-1.0 Tick 10) — the
  bootstrap's hash discipline is sufficient for v0.1's reproducibility
  bar.
- `mirror run` / `mirror fate` subcommands (road-to-1.0 Tick 9) —
  the engine runs through `kintsugi`, not `run` / `fate`.

v0.1 is therefore cuttable significantly before v1.0. The two
releases are independent on the calendar; v0.1 ships when the seven
ticks (T11.2–T11.7) close, irrespective of where Road-to-1.0 sits.

When v1.0 cuts, the action's `mirror-version` default updates to
`v1.0.0`, and the v0.1 line continues to be supported with bugfix
backports for some calendar window (TBD; not v0.1's concern).

---

## 8. What v0.1 deliberately does NOT include

This is the boundary. Anything below this line is post-v0.1.

- **@fate-bound resolutions in CI.** v0.1 ships single-candidate
  fractures only. The dispatcher binds `enumerate` to return one
  candidate; the loop runs the property checks and emits the
  verdict. Multi-candidate tournaments fire when they fire (per
  [[kintsugi-tournament]]) but do not exercise `@fate.infer` in
  v0.1. (The au column lands post-v0.1; see
  [[../cicd/kintsugi-thesis]] Claim 3–5 for the required pinning.)
- **Reflected commits.** The action does not push commits with the
  loop's proposed rewrites. It runs the loop, reports the verdict,
  exits. A `--write` mode for the action that opens a follow-up PR
  with the proposed rewrites is a v0.2 surface.
- **Custom strategy registration.** The closed strategy vocabulary
  from [[kintsugi-tournament]] is what runs. Action consumers cannot
  register new strategies through the wire. (Post-v0.1: a
  `strategies: <path>` input that loads a `.mirror` file declaring
  additional `@kintsugi/merge.strategy` instances.)
- **Reusable workflow.** §5.4 defers this to v0.2.
- **Non-GitHub CI providers.** GitLab / Buildkite / CircleCI
  equivalents are post-v0.1. The mirror CLI's `--ci` flag is
  provider-agnostic; the action shape is GitHub-specific. Other
  providers can shell out to `mirror kintsugi --ci` directly.
- **Cross-machine reproducibility of `outputs.objective`.** Per
  [[../cicd/kintsugi-thesis]] §6, the cross-machine bar requires the
  toolchain pinning of Claim 9. v0.1 verifies determinism on
  `ubuntu-latest` only; the macOS / Linux x86_64 binaries are
  shipped but not yet asserted byte-identical across them.
- **Variety verdict beyond pass/partial/fail.** The full
  `kintsugi_objective` scalar from [[kintsugi-variety]] §6 is
  surfaced as `outputs.objective`, but the policy on it (e.g.
  "reject if objective regresses against the previous merge") is
  not yet a built-in. A v0.2 input `compare-against: main` can
  add the regression check.

---

## 9. Risks

- **Action install determinism.** Downloading the binary at
  step-time introduces a network dependency. Mitigation:
  `install-mirror.sh` pins the binary by SHA-256 (from the release
  artifact). GitHub releases are immutable per tag; the
  reproducibility bar is held by the cryptographic verification.
  Determinism failure mode: GitHub releases go down. The action
  fails; the gate fails; merge is blocked. This is the correct
  behaviour.
- **Composite action output plumbing.** Step outputs in composite
  actions require shell discipline (`echo "key=value" >> $GITHUB_OUTPUT`).
  The two helper scripts (§5.2) encapsulate this; testing via `act`
  (T11.4) catches issues before publish.
- **The `outputs.opacities` array can be large.** GitHub step
  output values are capped (~1MB). For corpora with many parked
  bodies, the JSON array could approach the cap. Mitigation: the
  action truncates to the top N opacities (N=20 default; an
  `opacities-limit` input override) and indicates truncation in
  the PR comment.
- **PR-comment spam.** Re-running the loop on every PR push could
  produce many comments. Mitigation: the GitHub-script step uses
  the sticky-comment pattern (find the previous comment by marker;
  edit in place; don't duplicate).
- **Action-self regression.** §6.3 names the boundary case: an
  action change can't gate itself. The two-job pattern
  (`kintsugi-v0.1` + `kintsugi-head`) on action-evolution PRs is
  the recommended workflow; this is documented in
  `actions/kintsugi/README.md` but not enforced at the wire.
- **The bootstrap binary is per-machine reproducible only.**
  [[../cicd/kintsugi-thesis]] §3 names the v1.x ladder for
  cross-machine reproducibility. Until then, the four binaries
  shipped at release (`{x86_64,aarch64} × {linux,darwin}`) are
  independently built and may differ at byte level *between
  architectures*. The kintsugi verdict on the same corpus is
  asserted byte-identical *within an architecture*. This is the
  v0.1 honest bar.
- **v0.1 is small enough to ship in a week — and that's the risk.**
  The temptation will be to bundle in a few "obvious" extras. The
  boundary in §8 is load-bearing: anything below the line waits.

---

## 10. Open questions

1. **Should the action publish its own action.yml as v0.1, or live
   only inside the mirror monorepo?** Today's plan: it lives in
   `mirror/actions/kintsugi/`. The `uses:` path is
   `systemic-engineering/mirror/actions/kintsugi@v0.1`. GitHub
   resolves sub-paths inside repo tags natively. Alternative: a
   separate `systemic-engineering/kintsugi-action` repo that
   vendors the action's `action.yml` and helper scripts. **Default:
   stay in the monorepo.** Decision: needs Reed's call before T11.4.
2. **Should the action runtime install the mirror binary or vendor
   it as a release attachment in the action's own release?** Today's
   plan: install from the `systemic-engineering/mirror` release
   matching the action tag. This is the natural lock-step. Alternative:
   the action bundles the binary as an LFS attachment or downloads
   from a Cachix cache. **Default: install from release.** Decision:
   needs Reed's call before T11.4.
3. **What's the policy on the PR comment's content for `partial`
   verdicts?** The comment can be terse (one line, link to the
   `outputs.opacities`) or rich (a markdown table per opacity with
   the verdict shape). **Default: rich.** The opacity carrier is
   typed; the rendering is one helper function; the visual signal
   is the kintsugi metaphor at the wire altitude (you see the
   gold). Decision: needs Reed's review of the comment template
   during T11.4.
4. **Does the action verify the binary's SHA-256 against an
   in-action-tracked checksum, or pull the checksum from the
   release?** Today's plan: the action ships a `checksums.json`
   in `actions/kintsugi/` that's updated by `release.yml` at tag
   time. This is one fewer network call; it also means an action
   change requires a new tag. Alternative: pull from the release
   at runtime. **Default: in-action checksums.** Decision:
   substrate-pull on the trust model, needs alignment.
5. **At T11.5, when mirror's own workflow first runs with
   `./actions/kintsugi` against `boot/`, what happens if `boot/`
   currently has Dark regions?** Per the Justfile comment, the
   boot tree has in-progress Dark regions today. The gate would
   fail. **Resolution:** either run T11.5 only against a subset
   that's currently Dark-free (e.g. `boot/std/kintsugi/`), or land
   T11.5 with `fail-on: partial` first and tighten to `fail-on:
   failure` once boot/ is clean. Decision: needs Reed's call at
   T11.5 — depends on the boot-tree state at that tick.

---

## 11. What this spec changes

- v0.1 of the spectral stack is named: kintsugi wired up in GitHub
  Actions, with mirror as the Actions provider. Not the binary
  self-host (that's v1.0). Not the agent surface (that's v0.2+).
  The CI gate.
- The arc fragmentation-mcp → mirror-mcp → kintsugi-ci is named as
  the three altitudes of a single composition; the wire altitude is
  where v0.1 plants its flag.
- The Actions package shape is decided: composite action, helper
  shell scripts, in-repo `actions/kintsugi/`, floating `@v0.1`
  tag over `v0.1.*` releases.
- The recursive self-host is named with its two phases: pre-cut
  (local path), post-cut (published action via `@v0.1`). The
  boundary case for action-evolution PRs gets the two-job pattern.
- The seven ticks T11.2–T11.7 (plus T11.8 for the recursion's
  observable proof) are the chain. Each tick has a named artifact
  and a verification rule.
- The `ci/` carry-over is the `nix-setup` action and the OBC
  vocabulary. The Elixir composite actions don't apply; the
  composite-action template shape does.
- v0.1 is **strictly less** than v1.0. The cut criterion is
  decoupled from the road-to-1.0 ticks. v0.1 can ship before
  v1.0; both calendars are independent.

---

## 12. Prior art and references

The research grounding this spec:

- [[../cicd/README]] — the three-layer model for kintsugi-as-build-system.
- [[../cicd/kintsugi-thesis]] — the nine reproducibility claims; what
  v0.1 inherits, what waits.
- [[../cicd/prior-art]] — the build-system lineage; Bazel,
  GitHub Actions, Tekton, ArgoCD as the wire-altitude prior art.
- [[/Users/reed/dev/projects/ci/README]] — the OBC mapping; the
  `nix-setup` composite action; the reusable-workflow convention.
- [[/Users/reed/dev/projects/ci/WORKFLOW.md]] — the local/CI
  parity discipline. v0.1's `just kintsugi-ci-local` is the
  parity primitive.

In-spec dependencies:

- [[kintsugi-minimum-runnable]] — the dispatcher, the fracture
  substrate, the engine the action invokes.
- [[kintsugi-tournament]] — the multi-candidate case (v0.1 ships
  with single-candidate; tournament fires when it fires).
- [[kintsugi-variety]] — the @io minimization objective; the
  `outputs.objective` numeric.
- [[kintsugi-formatter]] — the iteration rule; `shatter: <N>`
  maps to the iteration depth.
- [[kintsugi-shatter]] — the recursive settle.
- [[kintsugi-self-hosting]] — the longer-arc binary self-host;
  v0.1 ships ahead of this.
- [[kintsugi-fracture-confidence-and-scene-dispatch]] — the
  confidence threshold; v0.1's `threshold:` input.
- [[kintsugi-wiring]] — the eight wires; v0.1 exercises the
  observation half of the loop (wires 1–6); the reflection half
  (wires 7–8) is what powers post-v0.1's `--write` mode.
- [[road-to-1.0]] — the broader cut; v0.1 sits inside it.
- [[mirror-binary-architecture]] — where the CLI's kintsugi
  subcommand lives.
- [[../../../prism/docs/specs/pq]] — the verdict shape on the wire.

In-corpus dependencies:

- Memory: `architecture-kintsugi-variety-io` — the @io minimization
  objective.
- Memory: `architecture-kintsugi-bias-lift` — substrate-pull as
  gradient. v0.1's recursion is substrate-pull at the wire altitude.
- Memory: `architecture-three-tier-stack` — fragmentation-mcp /
  mirror / @spectral/db. v0.1 lives on the mirror tier; rides on
  fragmentation; does not require @spectral/db.
- Memory: `architecture-pq-as-mcp-surface` — the pq wire altitude.
  v0.1's `outputs.opacities` shape is the pq `imperfect` channel.
- Memory: `feedback-loss-from-epistemologic-properties` — the
  loss is composed from declared properties, not invented.

---

*Pottery breaks. Gold conducts. The crack is the joint; the joint is
the value. At v0.1, the wire that carries the verdict is itself the
gold filling the crack between substrate and the world that depends
on it. The recursion is the proof: mirror's own crack is filled by
mirror's own gold, and the gold conducts because the substrate
holds.*

Apache-2.0.
