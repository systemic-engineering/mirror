---
title: Taut — StageFreight substrate scout for polyglot delivery
subtitle: Read-only mapping of /Users/alexwolf/dev/projects/StageFreight/ into mirror @tool + @cascade + @spectral/mosaic / @kintsugi/mosaic substrate. Empirical target for Arc-5 delivery per Alex 2026-07-18 direct-transcript.
author: Taut <taut@systemic.engineer>
date: 2026-07-18
tag: 📝 markdown-only bypass; pure-docs scout; read-only (no writes to StageFreight)
scope: /Users/alexwolf/dev/projects/StageFreight/ (Go binary + Dockerfile + GitLab CI + .stagefreight.yml)
---

# StageFreight substrate scout — polyglot delivery mapping

*Taut, 2026-07-18. Read-only scout of the StageFreight repo mapping every
artifact to mirror substrate species. Executed against the /loop directive
"Target full self-hosted building and @../StageFreight/ delivery queued as
next spec." Answers: which @tool species need to land, which @cascade path
shape the substrate wants, and what shape the delivery PR should take.*

---

## §0 Executive summary

**StageFreight is a Go+Docker+GitLab-CI self-hosting build orchestrator
already partially substrate-named by mirror.** The mirror substrate already
carries `@io/stagefreight` (family-root, `shards/io/stagefreight.mirror:11`),
`@io/oci` (`shards/io/oci.mirror`), `@io/git`, `@io/fs`, `@code/docker`
(`shards/code/docker.mirror`), and Mara's canonical spec
`docs/specs/mirror-build-substrate.md` which explicitly names the
StageFreight binding "declared in both directions" (`:29-33`). The delivery
target is *bilateral*: mirror ships to StageFreight AND StageFreight ships
mirror-fied CI to the world (Alex CURRENT.md `:753-759`).

**Gaps requiring mints:**
1. `@code/go` species (spec-cited 5+ times, never landed at shard altitude)
2. `@tool/go` species (@tool(X, args) minted `2dd8ddb`; go instance pending)
3. `@tool/docker` species (forward-promised in compiler-in-one-sentence spec `:216`)
4. `@tool/gitlab-ci` OR `@tool/git-forge/gitlab` species (net-new)
5. **ONE `@cascade/code/*/go` edge** — recommendation: `@cascade/code/llvm/go` (§2)
6. `.stagefreight.yml` classifier extension for `@spectral/mosaic.type_of_repo`

**Zero net-new family-roots.** All work lands under existing @tool +
@cascade + @code + @io + @spectral/mosaic + @kintsugi/mosaic substrate.

---

## §1 StageFreight structure map

### §1.1 Top-level enumeration (from `ls -la` + Search)

| Artifact | Purpose | mirror substrate species |
|----------|---------|--------------------------|
| `go.mod` (6.8KB) | Go module + 100+ deps (k8s client-go, go-git, cobra, viper, gitleaks, syft-adjacent, spf13, sigs.k8s.io/gateway-api) | `@tool/go` action `resolve_go_module` + `@code/go` species (NET-NEW mints) |
| `go.sum` (57KB) | MVS-resolved dep lockfile | `@tool/go` action `verify_go_sum` |
| `Dockerfile` (4.1KB) | Alpine-based multi-stage; golang:1.26.4-alpine3.23 builder → alpine:3.24.1 runtime; embeds docker CLI + buildx plugin | `@code/docker` (LANDED `shards/code/docker.mirror`) + `@tool/docker` species + `@cascade/code/docker/oci` (forward-promised `docs/specs/docker-container-substrate-decl-v0.1.md:6.5`) |
| `.gitlab-ci.yml` (2.3KB) | 5-stage pipeline (audition/perform/review/publish/narrate); self-hosted (uses `docker.cr.pcfae.com/prplanit/stagefreight:latest-dev`) | `@tool/gitlab-ci` species (NET-NEW) + `@kintsugi/mosaic.back_project_of_type` writes this |
| `.stagefreight.yml` (18.9KB) | StageFreight's OWN self-hosted lifecycle config: versioning, forges (gitlab.prplanit.com + github.com), registries (dockerhub/harbor), targets (10 registry+release combinations), toolchains (trivy/syft/grype/osv-scanner/cosign/flux/kubectl), badges, narrator, lint, security, test | `@spectral/mosaic.type_of_repo` READS this into `mosaic(@repo)`; `@kintsugi/mosaic.back_project_of_type` WRITES stagefreight.spec composed over @tool + @code + @io species |
| `cmd/stagefreight-gen-banner/main.go` | banner ANSI codegen | `@code/go` + `@cascade/code/go/*` for banner generation (peripheral) |
| `internal/docsgen/*.go` (4 files, largest 22.7KB) | CLI docs auto-generator | `@code/go` |
| `src/` (47 subdirs) | ALL functional Go code — `cli/`, `build/`, `registry/`, `forge/`, `lint/`, `config/`, `narrator/`, `sign/`, `security/`, `substrate/`, `toolchain/`, `manifest/`, `cistate/`, `gitops/`, `k8s/`, `runtime/`, `commit/`, `dependency/`, `retention/`, etc. | `@code/go` species; property tests + mutation coverage inject at package boundary |
| `integrations/` | azure-devops k8s agent yaml + gitlab runner docker-compose | `@code/docker-compose` (unminted) + `@code/kubernetes/yaml` (unminted); PERIPHERAL — skip in v0 delivery |
| `.stagefreight/preset-cache/preset/*.yml` (8 files) | StageFreight-authored preset composition | Already `@kintsugi/mosaic` composition-output shape |
| `.stagefreight/badges/*.svg` | build artifacts (generated) | Compiler-write output; skip |
| `docs/` (20 files inc. `RoadMap.md` 228KB, `FeatureMatrix.md` 24KB) | User-facing docs | Skip (v0); optional `@cascade/code/formal/prose` for docs projection |
| `CLAUDE.md` (4.5KB) | Alex's local Claude Code rules: **"NEVER use `git commit` — always use `stagefreight commit`"** + **"NO LOCAL GO TOOLCHAIN"** (dogfood-in-container) + `--dry-run` + container invocation pattern | ***Binding constraint on delivery***: mirror-authored PR MUST run through `stagefreight commit` or explicitly document why not |

### §1.2 Key architectural observations

1. **`src/` IS the Go source root** (not `internal/` + `cmd/` idiomatic Go).
   The Dockerfile copies `src/`, `cmd/`, `internal/` all three. mirror's
   `@code/go.detect` must walk all three.

2. **Self-hosting recursion**: StageFreight's own CI (`.gitlab-ci.yml`) uses
   the stagefreight docker image to build stagefreight. Mirror's delivery
   PR is a THIRD-ORDER observer: mirror-authored spec → StageFreight
   consumes → StageFreight-authored CI builds StageFreight-authored PR.
   This is the `@third` altitude Mara named in
   `docs/math/2026-07-18-third-order-observer-on-consumer-hardware.md`.

3. **`.stagefreight.yml` shape maps 1:1 to `@kintsugi/mosaic`
   back-projection**: version + matchers + vars + versioning + builds +
   forges + repos + registries + targets + build_cache + commit + test +
   dependency + docs + lint + release + security + toolchains + badges +
   narrator. This IS a mosaic. Every top-level key is a substrate species.

4. **Container-only dev**: Alex's CLAUDE.md says NEVER run `go build` etc.
   locally. All Go operations must go through the docker container.
   `@tool/go` MUST accept an execution-substrate parameter (native vs
   container) OR default to container per StageFreight discipline.

5. **`stagefreight commit`** replaces `git commit`. Alex ratified this
   at the StageFreight repo altitude. Mirror-authored PRs to StageFreight
   MUST honor this — implication: `@io/git.commit` at StageFreight altitude
   composes through `@tool/stagefreight` (NEW species; wraps the binary).

---

## §2 Cascade shape recommendation

### §2.1 The four candidates

Per Reed task-context list:

| Option | Shape | Substrate honesty |
|--------|-------|-------------------|
| **A. `@cascade/code/rust/go`** | direct Rust → Go translation | REJECTED: peer altitudes; no compiler pipeline exists rust→go; violates §3.1 "cascade morphism = loss-lens over lossy translation." Would require inventing a translation nobody uses. |
| **B. `@cascade/code/llvm/go`** | go compiles to LLVM (via gollvm or gccgo) | ADMISSIBLE: gollvm exists (Google), gccgo hits LLVM via GCC. Substrate-honest per Mara `dc4ad4c` §5 machine-substrate hub M pattern. Composes with LANDED `@cascade/code/rust/llvm` (`shards/cascade/code/rust/llvm.mirror`) to give `@cascade/code/rust/go := @cascade/code/llvm/go ∘ @cascade/code/rust/llvm`. |
| **C. `@cascade/code/turing/go`** | go from Turing tape | ADMISSIBLE at compositional altitude (composes through `@cascade/code/turing/mirror` + polyglot spec §5) but PATHOLOGICAL for empirical Go emission — go has no "tape-to-source" tool. |
| **D. NEW altitude `@cascade/code/native/go`** | net-new "native" altitude | REJECTED: "native" is not a substrate-honest altitude name; go is peer to rust/c/cpp at @code altitude; LLVM IS the machine-substrate hub the polyglot spec already ratified. |

### §2.2 Recommendation — B: `@cascade/code/llvm/go`

**Rationale (3 lines of substrate-pull evidence):**

1. **Machine-substrate hub already ratified.** Mara polyglot spec
   (`docs/math/polyglot-loss-aware-computational-translation.md:23-27`):
   "for any machine-substrate hub M, `@cascade/code/A/B :=
   @cascade/code/M/B ∘ @cascade/code/A/M` exists as a well-typed
   composition." LLVM is the substrate's chosen M (see Alex verbatim
   at `shards/code/llvm.mirror:11-13`: *"So we can have
   @cascade/code/llvm/turing and @cascade/code/rust/llvm. And boom. The
   loop closes."*). Adding `@cascade/code/llvm/go` extends the loop, does
   not replace it.

2. **`@cascade/code/llvm/flang` precedent this arc.** Mara `dc4ad4c` +
   `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md:§9.4`
   just landed `@cascade/code/llvm/flang` as NEW cascade edge with
   IDENTICAL shape (LLVM IR → language-specific target via
   frontend-in-reverse). `@cascade/code/llvm/go` is the SAME PATTERN
   for Go via gollvm.

3. **Composition immediately unblocks Rust→Go**. Since
   `@cascade/code/rust/llvm` LANDED (`shards/cascade/code/rust/llvm.mirror`),
   landing `@cascade/code/llvm/go` derives `@cascade/code/rust/go` for
   free via composition. This is what the polyglot spec §3.2
   composition-under-a-hub explicitly promises.

**Practical target (gollvm/gccgo status)**: gollvm is a public Google
project (github.com/golang/gollvm), gccgo is the go frontend for GCC.
Either supplies the LLVM-IR emission side. For the FIRST empirical tick
mirror can shell to `go build -toolexec` OR `gccgo -S -emit-llvm`; the
Rust `apply_llvm_go` resolver arm looks structurally identical to
Reed's landed `apply_llvm_turing` (`bootstrap/src/apply_h.rs` M3).

### §2.3 Secondary cascade edges (NOT this scout's ratification target)

For downstream StageFreight cross-artifact discipline, these forward-
promise:
- `@cascade/code/docker/oci` (Mara `docs/specs/docker-container-substrate-decl-v0.1.md:§6.5` — declared, unlanded)
- `@cascade/code/go/oci` (Dockerfile builds Go source directly to OCI image; derived: `@cascade/code/llvm/oci ∘ @cascade/code/go/llvm` if go→llvm exists; simpler: direct species)

---

## §3 New @tool species needed

Ordered by delivery-critical-path:

### §3.1 Tier-1 (required for v0 delivery PR)

1. **`@tool/go`** — `@tool(X, args)` species where X=go. Actions:
   - `resolve_go_module(go.mod path) → module_ref`
   - `build_go_package(pkg_selector, tags[]) → binary_ref`
   - `test_go_package(pkg_selector, race?, coverage?) → test_verdict`
   - `mod_tidy(go.mod path) → go.mod, go.sum`
   - Composes with existing `@tool` family-root (`docs/specs/2026-07-18-the-compiler-in-one-sentence.md:§3` LANDED as canonical spec)
   - Sibling to `@tool/cargo` (spec `:§3.4`); shape SHOULD mirror cargo exactly.

2. **`@tool/docker`** — forward-promised at compiler-in-one-sentence spec
   `:216`. Actions:
   - `build_docker_image(dockerfile, context, tags[], platforms[]) → oci_ref`
   - `pull_docker_image(ref) → oci_layer_set`
   - `push_docker_image(ref, registry) → registry_push_verdict`
   - Composes with `@io/oci` (LANDED) and `@code/docker` (LANDED).

3. **`@tool/gitlab-ci`** OR **`@tool/git-forge/gitlab`** — Alex-arbitration
   question (Q1 below). Actions minimally:
   - `render_gitlab_ci(stagefreight_yml, provider=gitlab) → gitlab_ci_yaml`
   - `validate_gitlab_ci(yaml) → validation_verdict`
   - Substrate-pull: StageFreight itself uses `stagefreight ci render gitlab --write` (see `.gitlab-ci.yml:6`). This IS the tool. Mirror wraps it.

### §3.2 Tier-2 (delivery quality — property tests, mutation, coverage)

4. **`@tool/stagefreight`** — wrapper around the stagefreight binary itself
   (per Alex CLAUDE.md, replaces `git commit` and `git tag`). Actions:
   - `stagefreight_commit(message, ...) → commit_ref`
   - `stagefreight_tag(pattern, ...) → tag_ref`
   - `stagefreight_docker_build(--dry-run?, --local?) → build_verdict`
   - This species IS the "mirror-fies your CI pipeline" surface Alex named.

5. **`@tool/gollvm`** OR **`@tool/gccgo`** — the LLVM-IR frontend for Go
   the `@cascade/code/llvm/go` species discharges through. Choice
   depends on §2.2 gollvm-vs-gccgo empirical availability (defer to
   Reed execution-step 4).

### §3.3 Explicitly NOT-mint (peripheral, don't gold-plate)

- ~~`@tool/harbor`, `@tool/dockerhub`~~ — these are registry providers;
  compose via `@io/oci` push actions parameterized by registry_ref;
  NO new @tool species needed.
- ~~`@tool/trivy`, `@tool/syft`, `@tool/grype`, `@tool/osv-scanner`,
  `@tool/cosign`~~ — these are StageFreight-managed toolchains
  (`.stagefreight.yml:toolchains:desired:*`). `@tool/stagefreight`
  orchestrates them. Mirror does NOT wrap each individually for v0.
- ~~`@tool/k8s` / `@tool/kubectl` / `@tool/flux`~~ — already forward-
  promised in compiler-in-one-sentence spec `:216`; not on StageFreight
  delivery critical path.

---

## §4 Substrate-already-had-the-word findings

Extensive grep across `/Users/alexwolf/dev/projects/mirror/{shards,docs,boot}/**`:

### §4.1 What EXISTS at shard altitude

- **`@io/stagefreight`** — `shards/io/stagefreight.mirror:11` (2026-06-22).
  Family-root, spectral_coordinate + wire_surface + freight_manifest
  carriers; `address`, `freight`, `transit` actions;
  `stagefreight_addressable` bilateral. **THIS IS THE CANONICAL WIRE
  SURFACE** into which mirror-authored spec-crystals get shipped to
  StageFreight consumers. Any delivery PR to StageFreight IS a
  `@io/stagefreight.transit` invocation at the concrete altitude.
- **`@code/docker`** — `shards/code/docker.mirror` (2026-07-12). Sibling
  to @code/rust, @code/gleam. Docker-language species.
- **`@io/oci`** — `shards/io/oci.mirror` (2026-06-23). Container image
  format @io boundary.
- **`@io/git`** — `shards/io/git.mirror` (2026-07-15). Git repository @io
  boundary. Composes with any `@tool/stagefreight_commit`.
- **`@code/llvm`** — `shards/code/llvm.mirror` (2026-07-17). Machine-
  substrate hub the recommended cascade routes through.
- **`@cascade/code/rust/llvm`** — `shards/cascade/code/rust/llvm.mirror`
  (2026-07-17). Landed edge; sibling shape to proposed
  `@cascade/code/llvm/go`.
- **`@spectral/mosaic`** — `shards/spectral/mosaic.mirror` (2026-07-18).
  READ-side of the type_of_repo bilateral. This IS what reads
  StageFreight's `.stagefreight.yml` + Dockerfile + go.mod → typed
  `mosaic(@repo)` structure.
- **`@kintsugi/mosaic`** — `shards/kintsugi/mosaic.mirror` (2026-07-18).
  WRITE-side; back-projects `stagefreight.spec` from `mosaic(@repo)`.
- **`@tool` family-root + `@tool(X, args)` parametric type** — canonical
  spec `docs/specs/2026-07-18-the-compiler-in-one-sentence.md:§3` LANDED;
  `2dd8ddb` per Reed session state. First three species landed:
  `@tool/cargo`, `@tool/git`, `@tool/nix` (spec §3.4).

### §4.2 What is FORWARD-PROMISED

- **`@tool/docker`** — spec `:216` in compiler-in-one-sentence.
- **`@tool/kubectl`, `@tool/npm`, `@tool/pip`, `@tool/ffmpeg`, `@tool/sqlite`, `@tool/curl`, `@tool/jq`** — spec `:216-220`.
- **`@cascade/code/docker/oci`** — Mara `docs/specs/docker-container-substrate-decl-v0.1.md:§6.5` (declared, unlanded).
- **`@code/go`** — cited in `docs/specs/spec-inference.md:89` (.go → @code/go), `docs/specs/code-extension-grammar.md:49` (`grammar @code/go("go")`), `docs/specs/properties-on-glass.md:824`, `docs/specs/code-metalogue-surface.md:176-508`. **NEVER LANDED at shard altitude.** This is the biggest substrate-already-had-the-word gap: the SPEC layer cites @code/go 8+ times but no `shards/code/go.mirror` exists.
- **`@io/stagefreight/narrative`** — projection format species (stagefreight shard `:34-38`).

### §4.3 What DOES NOT exist (safe to mint)

- `@tool/go` — no prior reference at shard altitude
- `@tool/gitlab-ci` — no prior reference
- `@tool/git-forge/*` — no prior reference (Q1 below)
- `@tool/stagefreight` — no prior reference (though `@io/stagefreight` IS the parametric-boundary sibling)
- `@cascade/code/llvm/go` — no prior reference
- `@cascade/code/go/*` — no prior reference

### §4.4 Marcus C Davis / sofmeright / PrPlanIT lineage

Zero references to Marcus C Davis, sofmeright@gmail.com, or PrPlanIT
in mirror substrate. The relationship is purely EXTERNAL (Alex ↔
Marcus commitment). The FIRST substrate reference will land in the
Mara-authored delivery spec + the PR commit trailer. Attribution
discipline: per StageFreight `CLAUDE.md` — "Only sign commits as
sofmeright@gmail.com / SoFMeRight. No anthropic attribution comments
in commits." Mirror's delivery PR MUST NOT include Co-Authored-By
lines; the commit is signed as the mirror-compiler identity (per
mirror's own attribution discipline) or transparently attributed to
Alex — Q2 below.

---

## §5 Delivery PR shape sketch

### §5.1 What mirror generates FOR StageFreight

Concrete file diff on the StageFreight repo, in dependency order:

**New files (added by mirror-authored PR):**

1. `stagefreight.spec` (~200-400 LOC estimated) — mirror-authored
   `.spec` file per `@kintsugi/mosaic.back_project_of_type` writing
   the typed shape of StageFreight-the-repo:
   ```
   spec "github.com/PrPlanIT/StageFreight" {
     code {
       go {
         module "github.com/PrPlanIT/StageFreight"
         version "1.26.4"
         packages [./src/..., ./cmd/..., ./internal/...]
       }
       docker {
         dockerfile "./Dockerfile"
         base "docker.io/library/golang:1.26.4-alpine3.23"
         runtime "docker.io/library/alpine:3.24.1"
       }
     }
     tools {
       go       { toolchain "1.26.4"; container "prplanit/stagefreight:latest-dev" }
       docker   { buildx v0.35.0; execution "dind" }
       gitlab-ci { provider "gitlab.prplanit.com"; project "PrPlanIT/stagefreight" }
       stagefreight { self }  # dogfood
     }
     ci {
       stages [audition, perform, review, publish, narrate]
       provider gitlab
       image "docker.cr.pcfae.com/prplanit/stagefreight:latest-dev"
     }
     properties {
       liquid_over @code/go   # prismqueer::liquid property tests
       butterfly @code/go     # mutation coverage
       roomba walker (coverage-gaps)
     }
     trust {
       root @alex
       author @sofmeright
     }
   }
   ```
   Composes the LANDED @tool species + @io/stagefreight wire boundary +
   spec-file grammar per Mara's spec inference (`docs/specs/spec-inference.md`).

2. `.mirror/build-verify.yml` (~50 LOC) — mirror's property-test +
   mutation-coverage runner as a NEW GitLab CI job. Composed via
   `@kintsugi/mosaic.back_project_of_type` writing gitlab-ci extension.

3. `.mirror/property-tests/` (Go test files, `~200-500 LOC`) — auto-
   generated prismqueer::liquid property tests over key packages
   (`src/config/`, `src/build/`, `src/lint/`, `src/manifest/`).
   These use Go's native `testing/quick` + `pgregory.net/rapid` (or
   `github.com/leanovate/gopter`) as the Go equivalent of prismqueer;
   mirror emits ONE Go property test per bilateral admissibility
   predicate it can detect via `@spectral/mosaic.type_of_repo`.

4. `.mirror/mutation-report.md` (~10-50 KB) — `@butterfly` mutation
   coverage report over `src/`; substrate-decl'd as `@code/formal/prose`
   projection (Glint's cascade species from `shards/cascade/code/
   formal/prose.mirror`).

**Modified files (touched by mirror):**

5. `.gitlab-ci.yml` — one added stanza `.mirror-verify:` after
   `narrate:` stage. Idempotent: honors the `# GENERATED BY
   STAGEFREIGHT — DO NOT EDIT` header via a `# EXTENDED BY MIRROR —
   idempotent; regenerate: mirror ci extend --write` sibling header.

6. `README.md` — one badge added inside the existing
   `<!-- sf:project:start --> ... <!-- sf:project:end -->` block:
   `[![mirror-verified](https://mirror.systemic.engineering/badge/
   stagefreight-liquid-coverage.svg)](...)`. Honors the
   `.stagefreight.yml:narrator` discipline — mirror injects the
   badge via an `id: project.mirror-verified` narrator item, NOT via
   raw README rewrite.

7. `.stagefreight.yml` — one added top-level key `mirror:` with
   `verify: { enabled: true; artifacts: ".mirror/" }`. This is the
   substrate-honest surface that lets StageFreight OWN the mirror
   verification job specification.

**Files NOT touched (property):**
- `go.mod`, `go.sum` (unless mutation coverage discovers a hardening dep)
- Any file in `src/**/*.go` (property tests observe, do not modify)
- `LICENSE`, `Dockerfile` (out of scope for v0)

### §5.2 Property tests strategy (Go native)

Per Alex's prismqueer::liquid pattern (memory
`feedback-prismqueer-macros-mirror-composes`): substrate-authored
FLOOR macros generate proc-macro-emitted property tests. Go
equivalent: mirror emits `*_property_test.go` files using
`pgregory.net/rapid` (SOTA Go PBT). These are hand-authored FLOOR
in Go's terms — Go has no procedural macros, so the emit step IS
the substrate authorship. The test bodies substrate-decl bilateral
admissibility predicates in Go syntax.

Example emit target (`src/config/config_property_test.go`):
```go
// GENERATED BY MIRROR — DO NOT EDIT
// Bilateral: config_load_admissible per @kintsugi/mosaic type_of_repo
package config

import (
    "testing"
    "pgregory.net/rapid"
)

func TestConfigLoad_RoundTrip(t *testing.T) {
    rapid.Check(t, func(t *rapid.T) {
        yaml := generateStagefreightYAML(t)  // rapid generator
        cfg, err := Load(yaml)
        if err != nil { t.Fatal(err) }
        yaml2, err := Dump(cfg)
        if err != nil { t.Fatal(err) }
        cfg2, err := Load(yaml2)
        if err != nil { t.Fatal(err) }
        if !equal(cfg, cfg2) { t.Fatal("round-trip inadmissible") }
    })
}
```

### §5.3 Mutation coverage report (@butterfly)

Per memory `project_butterfly_substrate_species`: `@butterfly` = the
counter-@roomba prism, mutation coverage = butterfly sensitivity.
Empirical Go tools: `go-mutesting` (github.com/zimmski/go-mutesting)
is the SOTA Go mutation-testing tool. Mirror shells `@tool/go` +
`@tool/go-mutesting` (embed as sub-tool of @tool/go for v0, no
separate species mint) and produces `.mirror/mutation-report.md`.

### §5.4 Commit shape (signed via @trust chain to @alex root)

Per StageFreight `CLAUDE.md` discipline + Alex ratification:

- **Committer identity**: `SoFMeRight <sofmeright@gmail.com>` (Marcus's
  identity — Marcus is the StageFreight repo authority, mirror is
  the tool). Attribution flows: @alex root → @sofmeright authority
  → mirror-compiler tool → PR content.
- **Signing**: mirror-compiler signs the commit content via its own
  key; the merge into main requires @sofmeright signature (Marcus's
  authority). This is a bilateral trust handshake, not mirror-forging-
  as-Marcus. Q2 below asks Alex about this.
- **NO** Co-Authored-By Claude / anthropic attribution (StageFreight
  CLAUDE.md rule).
- **MUST use** `stagefreight commit` (not `git commit`; Alex rule
  bright red). Mirror invokes this via `@tool/stagefreight.commit`.

Suggested commit message:
```
mirror: back-project stagefreight.spec + property tests + mutation coverage

Adds mirror-compiler-authored .spec (via @kintsugi/mosaic back-projection
of @spectral/mosaic.type_of_repo) + prismqueer::liquid property tests
(pgregory.net/rapid) over src/config, src/build, src/lint, src/manifest
+ @butterfly mutation-coverage report via go-mutesting.

Composes @tool/go + @tool/docker + @tool/gitlab-ci + @io/stagefreight
wire boundary. See mirror docs/specs/<mara-authored-delivery-spec>.md
for the full substrate ancestry.

Signed-off-by: mirror <mirror@spectral.engineer>
```

---

## §6 Reed's execution recipe (post-Mara-spec)

Concrete steps for Reed AFTER Mara's StageFreight delivery canonical
spec lands (assumed running in parallel this /loop). Smallest tick
that ships something empirical:

1. **Mint `shards/code/go.mirror`** (SUBSTRATE FLOOR — 8+ spec cites
   with zero shard). Family-root sibling to @code/rust, @code/gleam,
   @code/docker. Grammar-decl'd per `docs/specs/code-extension-grammar.md:49`.
   Follow shape of `shards/code/gleam.mirror` for minimal LOC.
   Substrate-decl only; realisation defers to @tool/go actions.

2. **Mint `shards/tool/go.mirror`** — `@tool(X=go, args)` species-decl
   per compiler-in-one-sentence spec §3.4 pattern. Actions:
   `resolve_go_module`, `build_go_package`, `test_go_package`,
   `mod_tidy`. Sibling shape to `@tool/cargo` (Mara `2dd8ddb`).
   Substrate-decl'd bilateral: `go_build_admissible` (execution-
   substrate parameter: container-default per StageFreight CLAUDE.md).

3. **Mint `shards/tool/docker.mirror`** — discharge the compiler-in-
   one-sentence spec `:216` forward-promise. Actions:
   `build_docker_image`, `pull_docker_image`, `push_docker_image`.
   Composes with `@io/oci` (LANDED) and `@code/docker` (LANDED).

4. **Mint `shards/tool/gitlab-ci.mirror`** (OR `@tool/git-forge/gitlab`
   per Q1) — read/write `.gitlab-ci.yml`. Action `render_gitlab_ci`
   shells to `stagefreight ci render gitlab --write` per StageFreight
   substrate-pull; do NOT re-implement YAML generation.

5. **Mint `shards/cascade/code/llvm/go.mirror`** — NEW cascade edge.
   Sibling shape to `shards/cascade/code/llvm/turing.mirror` and
   `shards/cascade/code/rust/llvm.mirror`. Resolver-arm shell to
   gollvm or gccgo. Substrate-decl only; concrete Rust binding in step 7.

6. **Extend `shards/spectral/mosaic.mirror`** — add StageFreight to the
   type_of_repo detection table (recognize `.stagefreight.yml` +
   `Dockerfile` + `go.mod` triple). Add StageFreight to `mosaic(@repo)`
   carriers. Extend `shards/kintsugi/mosaic.mirror` similarly for
   back_project side.

7. **Extend `bootstrap/src/apply_h.rs`** with three resolver arms:
   `apply_tool_go` (shells to `go` binary in container),
   `apply_tool_docker` (shells to `docker` in DIND socket),
   `apply_cascade_llvm_go` (shells to gollvm/gccgo). Follow the
   pattern Reed established at `7a962ab` (`apply_rust_llvm` +
   `apply_llvm_turing` + `apply_turing_mirror`). ~200-300 LOC.

8. **Empirical smoke test** — `bootstrap/tests/stagefreight_delivery_
   smoke.rs`: point mirror at `/Users/alexwolf/dev/projects/StageFreight/`,
   invoke `mirror kintsugi mosaic back-project`, assert emitted
   `stagefreight.spec` parses and its `tools { }` block matches the
   ground-truth (go+docker+gitlab-ci+stagefreight).

9. **Empirical property-test emission** — extend
   `bootstrap/src/kintsugi.rs` (or new `bootstrap/src/property_emit.rs`)
   with Go emitter for prismqueer::liquid properties. Uses
   `pgregory.net/rapid`. Emits one `*_property_test.go` per detected
   bilateral. Smoke test asserts emission compiles under `go build`
   (in container per Alex CLAUDE.md).

10. **Delivery PR generation** — `mirror stagefreight deliver
    /Users/alexwolf/dev/projects/StageFreight/`. This IS the empirical
    endpoint: reads repo → back-projects spec → emits property tests
    → runs mutation coverage → generates PR diff → invokes
    `@tool/stagefreight.commit` → outputs PR URL. Alex reviews, Marcus
    ratifies, Marcus merges.

**Estimated total**: 8-12 commits over 2-4 /loop ticks. Steps 1-4 =
substrate mints (one commit each; parallelizable with Mara). Steps
5-7 = Reed's cascade + resolver work. Steps 8-10 = empirical.

---

## §7 Q's for Alex (max 2)

**Q1: `@tool/gitlab-ci` OR `@tool/git-forge/gitlab`?**
The former is flat (sibling to `@tool/cargo`, `@tool/docker`); the
latter carves a hierarchy anticipating `@tool/git-forge/github`,
`@tool/git-forge/gitea`, `@tool/git-forge/forgejo`. StageFreight
itself has this hierarchy (`src/forge/`: gitlab.go + github.go +
gitea/forgejo.go). Which shape does the substrate want? Substrate-
pull: StageFreight's `src/forge/` hierarchy IS the answer — mirror
should mirror it. Recommendation: `@tool/git-forge/gitlab` with
`@tool/git-forge` as intermediate family-root. But this creates a
NEW intermediate family-root; wanted to flag.

**Q2: Delivery PR attribution — mirror-signed as @sofmeright, or
alex-signed with mirror-generated-content trailer?**
StageFreight CLAUDE.md is bright red: "Only sign commits as
sofmeright@gmail.com". Mirror as a tool can either (a) commit AS
sofmeright (Marcus authorizes his key for mirror-tool use — trust
handshake via @trust chain from @alex → @sofmeright → mirror), OR
(b) commit as alex@systemic.engineer with `Generated-By: mirror-
compiler <mirror@spectral.engineer>` trailer and let Marcus review-
then-cherrypick under his identity. Which respects the substrate
AND the external trust boundary? Reed default without answer: (b),
because (a) requires Marcus explicitly authorizing his signing key
for mirror-tool use, which is an out-of-band consent negotiation
outside mirror substrate.

---

## §8 One-sentence surprise

**StageFreight's `.stagefreight.yml` IS a mosaic-in-YAML — 18 top-
level keys, each a substrate species already declared or forward-
promised in mirror, arranged in exactly the composition-graph shape
`@kintsugi/mosaic.back_project_of_type` outputs — meaning mirror's
delivery PR to StageFreight is structurally a self-recognition
theorem: mirror discovers itself typing the tool that ships mirror
to the world, and the third-order observer at `@peer.observes(
@metalogue(@kintsugi/mosaic, @spectral/mosaic))` is a strict
identity fixpoint on this specific repo.**

---

## Appendix: file/line citations

- `/Users/alexwolf/dev/projects/StageFreight/.stagefreight.yml:1-645` — full mosaic-in-YAML
- `/Users/alexwolf/dev/projects/StageFreight/.gitlab-ci.yml:1-100` — 5-stage lifecycle
- `/Users/alexwolf/dev/projects/StageFreight/Dockerfile:1-100` — multi-stage golang-alpine
- `/Users/alexwolf/dev/projects/StageFreight/CLAUDE.md:1-100` — Alex's local rules (READ THIS in delivery discipline)
- `/Users/alexwolf/dev/projects/StageFreight/go.mod:1-155` — module + 100+ deps
- `/Users/alexwolf/dev/projects/StageFreight/README.md:1-100` — theatrical framing, docker-hub distribution
- `/Users/alexwolf/dev/projects/mirror/shards/io/stagefreight.mirror:11-200` — LANDED @io/stagefreight family-root
- `/Users/alexwolf/dev/projects/mirror/shards/spectral/mosaic.mirror:1-120` — LANDED read-side mosaic
- `/Users/alexwolf/dev/projects/mirror/shards/kintsugi/mosaic.mirror:1-130` — LANDED write-side mosaic
- `/Users/alexwolf/dev/projects/mirror/shards/code/docker.mirror:1-150` — LANDED @code/docker
- `/Users/alexwolf/dev/projects/mirror/shards/code/llvm.mirror:1-100` — LANDED machine-substrate hub
- `/Users/alexwolf/dev/projects/mirror/shards/cascade/code/rust/llvm.mirror` — LANDED sibling cascade shape
- `/Users/alexwolf/dev/projects/mirror/shards/cascade/code/llvm/turing.mirror` — LANDED sibling cascade shape
- `/Users/alexwolf/dev/projects/mirror/docs/specs/2026-07-18-the-compiler-in-one-sentence.md:§3, §3.4, :216` — @tool + species table
- `/Users/alexwolf/dev/projects/mirror/docs/specs/mirror-build-substrate.md:1-33, :137` — StageFreight binding declared in both directions
- `/Users/alexwolf/dev/projects/mirror/docs/specs/docker-container-substrate-decl-v0.1.md:§6.5` — @cascade/code/docker/oci forward-promise
- `/Users/alexwolf/dev/projects/mirror/docs/specs/spec-inference.md:89` — .go → @code/go inference
- `/Users/alexwolf/dev/projects/mirror/docs/math/polyglot-loss-aware-computational-translation.md:23-27` — machine-substrate hub composition law
- `/Users/alexwolf/dev/projects/mirror/docs/loop/CURRENT.md:753-769` — Alex "@../StageFreight/" delivery target verbatim
- `/Users/alexwolf/dev/projects/mirror/docs/math/2026-07-18-third-order-observer-on-consumer-hardware.md` — @kintsugi/mosaic ↔ @spectral/mosaic bilateral (grounds this scout's §5.1 delivery shape)

*End scout. Delivered read-only per Taut discipline. Reed proceeds
fractal step 4 in parallel; Mara authors delivery canonical spec in
parallel; Seam ratifies session landings in parallel.*
