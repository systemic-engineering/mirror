# Prior art — reproducibility, determinism, and where each generation leaks

*2026-06-01. The deep survey, reframed. Every system scored on the two
properties that matter: reproducibility (byte-identical outputs from
byte-identical inputs) and determinism (no hidden state). Each section
names what the system solved, what it taught, and where it leaks today.*

Read [[README]] first for the bar. This file is the homework that
grounds the engineering claim.

---

## Why a survey around reproducibility-and-determinism

The build-system field is fifty years old. The surface looks
repetitive (YAML files, dependency DAGs, container sandboxes) but the
conceptual progress underneath is real — and almost every step of
that progress was earned by a different attempt to close a different
reproducibility leak. Make's mtime leaks. Bazel's PATH leaks. Cargo's
`build.rs` leaks. GitHub Actions's container image leaks. Each fix
was its own lesson; the field is in some sense a forty-year argument
about what reproducibility means and how much determinism the
underlying compiler/runtime/cloud allows.

The kintsugi/au proposal makes a stronger claim than most: that
reproducibility + determinism survive the introduction of an AI
inference column in the build graph. The honest survey is the one
that names where every prior system leaks first, so the claim has
something to stand on.

Two discipline notes:

1. **Citations.** Where systems are widely known, I name author + year
   + venue and assume the reader can find primary sources. WebSearch
   / WebFetch were unavailable during this research round; citations
   are from training knowledge with cutoff Jan 2026. The gaps to
   revisit are listed at the end (§6).
2. **Scoring.** "Reproducible" and "deterministic" admit grades. I
   use:
   - **R** ∈ {**no**, **per-machine**, **across-machine**, **across-time**} —
     does the same input give the same output, and how widely?
   - **D** ∈ {**no**, **with-care**, **by-construction**} — is
     non-determinism prevented by the system, or only by the
     operator's discipline?
   The grades are not precise; they're shorthand for the discussion.

---

## 1. The build-system lineage

From Stuart Feldman's Make through Buck2 and Nix, the field worked out
four ideas in order: (a) builds are DAGs, (b) caches must be
content-addressed, (c) hermeticity is binary, and (d) builds are
functions. Each idea took a decade or more to land cleanly; each
landed because the previous generation's reproducibility leaks
forced it.

### 1.1 Make — the original DAG, the original leak

**Stuart Feldman, Bell Labs, 1976.** *"Make — A Program for
Maintaining Computer Programs"*, SP&E. The single most consequential
build tool ever shipped.

**R: no.** **D: no.**

**What it solved.** A way to express "this output depends on these
inputs; rebuild when an input is newer" without rebuilding everything
every time. The Makefile is a DAG of *targets* with *recipes*; mtime
is the staleness signal.

**Where it leaks.**

- **mtime is clock-dependent.** A file touched in the future (clock
  skew, NFS, file extracted from an archive with preserved
  timestamps) is "newer than" its outputs forever. Builds silently
  succeed with stale outputs.
- **Implicit dependencies fail silently.** If the recipe reads a file
  the Makefile didn't declare, the build is wrong-but-fast: it
  compiles, but caches will lie.
- **Recipes are arbitrary shell.** A recipe can read `$RANDOM`, the
  system clock, the network. The build's "correctness" is the
  operator's discipline.

**What it taught the field.** Builds are DAGs of outputs from inputs.
The next forty years are arguably one long response to mtime's silent
failure: every "hermetic" or "reproducible" feature in every
successor system attempts to detect or prevent what mtime cannot
catch.

**Inheritance for kintsugi.** The DAG framing carries straight
through. Mirror's substrate is itself a DAG (refs point at refs).
Make's cure — content-addressing the staleness signal as the
recipe's hash — is what every successor inherits and what kintsugi
inherits via `Splinter<H>`'s OID.

### 1.2 Plan 9 `mk` — discipline through parsimony

**Tom Duff, AT&T Bell Labs, ~1986–1989.** USENIX paper *"Experience
with mk"* (the exact year I'd want to confirm).

**R: no.** **D: no.**

**What it solved.** Make's accreted complexity (recursive sub-makes,
implicit pattern rules with subtle precedence). `mk` killed implicit
patterns, made recipes pure shell, and introduced virtual targets
as first-class.

**Where it leaks.** Same as Make. The improvements are surface
parsimony, not foundation. Recipes are still shell; caches are still
mtime. The contribution is aesthetic discipline.

**What it taught the field.** A build tool is a language; the
language should be parsimonious. Discipline matters more than
cleverness. The Nix expression language, Bazel's Starlark, and SCons's
Python-subset all inherit this aesthetic.

**Inheritance for kintsugi.** The substrate-pull discipline of
AGENTS.md echoes Duff's parsimony: the floor stays small, capability
lives in the substrate.

### 1.3 Vac / Venti — content-addressable storage

**Sean Quinlan and Sean Dorward, Bell Labs, 2002.** USENIX FAST 2002:
*"Venti: A New Approach to Archival Storage"*. Russ Cox's `vac` tool
sits on top of Venti.

**R: across-time** (within the storage layer). **D: by-construction.**

**What it solved.** A storage layer where every block is keyed by
the SHA-1 of its content. Writes are idempotent (same bytes → same
key); identical blocks are deduplicated automatically; a snapshot's
identity is a single hash that recursively names every block beneath
it.

**Where it leaks.** Storage only, not orchestration. Venti said
*where artifacts should live*; it didn't say *how to compute them*. A
non-deterministic compiler driving Venti still produces different
artifacts at different addresses. The reproducibility floor is the
hash function itself (SHA-1 was sufficient in 2002; modern Venti-like
systems use BLAKE3 or SHA-256). Once the bytes exist, the address
is determined.

**What it taught the field.** *Content-addressable storage is the
foundation every modern build cache rests on.* Bazel's action cache,
Nix's store, git's object database, Docker's layer hashes, IPFS,
S3 versioning, Turbopack's remote cache — all of them are Venti's
children. The single sentence "a block's name is its hash"
reorganised distributed systems for twenty years.

**Inheritance for kintsugi.** Direct. `Splinter<H>` is the Venti
story applied to substrate values. The Merkle structure (each level
hashed from its children's OIDs, not the recursive content) is the
standard shape from Venti through git through every modern Merkle
tree, ported into mirror with the `H` parameter so the store and the
engine can inhabit different hash-worlds in the same binary
([[../specs/store-vs-db-and-the-cascade|store-vs-db-and-the-cascade]]
§2). The kintsugi inheritance is: store-OID is what makes everything
upstream of it have a chance at reproducibility.

### 1.4 Autotools, CMake, SCons, Tup — the long middle of partial fixes

This section is short on purpose. Autotools (Cygnus / GNU,
~1991–1994) is the canonical *meta-build* (a build that generates a
build). CMake (Hoffman / Kitware, 2000) takes itself seriously as a
language. SCons (Steven Knight, 2000) first made cross-language
hermeticity through scanner-based dependency discovery. Tup (Mike
Shal, 2009) uses FUSE / ptrace to *prove* hermeticity by intercepting
syscalls.

**R: per-machine, mostly.** **D: with-care.**

**Where they leak.**

- Autotools embeds host paths into the generated Makefile; the
  Makefile that builds correctly on machine A may not on machine B.
- CMake's "out-of-source build" reduced the leak but didn't close it;
  generators (Ninja, Make, Visual Studio) produce different artifacts.
- SCons's scanner-based hermeticity catches most missing dependencies
  but can be fooled by indirection (preprocessor macros that resolve
  to filenames the scanner missed).
- Tup's FUSE / ptrace catches the rest in principle, but the
  interception layer itself is the new substrate-of-trust, and bugs
  in ptrace handling (or platform-specific syscall variants) leak.

**What this generation taught the field.** Hermeticity earned by
*scanning* or by *syscall interception* is fragile. You really want
hermeticity earned by *construction* — the build environment is
literally not permitted to read anything not declared. That insight
is what Bazel and Nix run with.

**Inheritance for kintsugi.** The negative lesson, mostly. Don't
write a build that generates a build that runs a build. The kintsugi
dispatcher is one indirection — substrate ref to Rust body — and that
is it.

### 1.5 Bazel — hermeticity by sandbox construction

**Google, internal as Blaze ~2007, open-sourced as Bazel in 2015.**
The single most influential modern build system. Starlark for build
descriptions; sandboxed actions; content-addressed action cache;
remote execution as a first-class feature (Remote Execution API,
standardised across Bazel / Buck / Pants ~2018).

**R: across-machine** (with discipline). **D: with-care** (the
sandbox enforces; the operator can disable).

**What it solved.** Hermeticity discipline at scale. Sandboxed
actions cannot read undeclared inputs, cannot reach the network
(unless `--define` lets them), cannot consult `/etc`, cannot embed
timestamps unless the operator's recipe deliberately does. The
content-addressed action cache makes a cache hit *trustworthy*: same
recipe + same inputs + same environment → same action key → reuse the
cached output safely.

**Where it leaks.**

- **PATH leaks.** Hermeticity is enforced for action *inputs* but
  not for the *toolchain* unless the toolchain is explicitly hermetic
  (the user must opt into a hermetic Python / C++ / Java toolchain;
  the default ones inherit from the host).
- **Timestamp leaks.** A compiler that embeds `__DATE__` /
  `__TIME__` into the binary breaks reproducibility silently. The
  fix is `SOURCE_DATE_EPOCH` ([reproducible-builds.org](
  https://reproducible-builds.org/specs/source-date-epoch/) standard,
  ~2015), which Bazel can pass but doesn't enforce by default.
- **Non-deterministic compilers.** Some C++ compilers produce
  different bytes across runs (PGO profile names, address-sanitizer
  thunks, parallel codegen ordering). Bazel's cache key doesn't catch
  this — the action runs deterministically; the compiler doesn't.
  See the reproducible-builds.org project for the catalog.
- **`--config=` flags.** Configuration that lives outside the
  declared inputs (build flags chosen at the command line) is a
  reproducibility leak by design: same Bazel invocation, different
  configs → different artifacts. The fix is to bake configs into the
  `BUILD` file or `.bazelrc`; the discipline is the user's.
- **Remote execution non-determinism.** When the action runs on a
  remote executor, the executor's environment (libc version, kernel
  version, CPU model) becomes part of the implicit input set. Bazel's
  Remote Execution API encodes the platform but cannot enforce its
  byte-identity.

**What it taught the field.** *Hermeticity earned by sandbox
construction is the cheapest viable form.* You don't have to prove
the recipe is hermetic; you make the recipe physically incapable of
reading anything not declared. The remaining leaks (toolchain,
compiler, platform) are the residue the next generation must close.

**Inheritance for kintsugi.** Content-addressed action cache →
`Splinter<H>`. Sandboxed hermeticity → made *mathematical* in mirror
by `@fate`'s refusal to do remote inference: the impossibility lives
in the substrate, not in a runtime sandbox. The substrate-pull
discipline (AGENTS.md §"Boundary Rust is not frozen capability") is
what keeps the toolchain leak from happening in mirror — the floor is
boundary Rust marked `[substrate-pull:realize]`, audited in git, not
ambient on `PATH`.

### 1.6 Buck and Buck2 — the Rust rewrite

**Facebook / Meta. Buck originally 2012 (Java); Buck2 (Rust)
open-sourced 2023.** Same conceptual lineage as Bazel — hermetic
actions, content cache, remote execution — with engineering polish
and a focus on incrementality.

**R: across-machine** (same caveats as Bazel). **D: with-care.**

**What Buck2 specifically pushed forward.** A serious commitment to
*perfectly reproducible builds*: Meta's internal Buck2 has been the
testbed for "byte-identical artifacts across all builders" as a goal
rather than an aspiration. The team has published talks on the
specific work required (compiler determinism patches, `SOURCE_DATE_EPOCH`
discipline, eliminating timestamp embedding from action outputs). The
BXL extension language admits some user programs at query time,
which is a determinism risk the team has documented.

**Where it leaks.** Same surface as Bazel; the team has been more
public about closing each leak case by case. Toolchain hermeticity
remains the long pole.

**Inheritance for kintsugi.** The Rust-as-floor discipline. Mirror's
bootstrap is also Rust by necessity (the boundary that the substrate
cannot yet describe of itself); the FROZEN-`.rs` discipline echoes
Buck2's discipline of keeping the floor lean. The performance-budget
instinct transfers too: inference cost matters if a build step is
going to be inferred, and the @fate substrate already bounds that
with the sub-Turing invariant.

### 1.7 Nix — hash-of-recipe → hash-of-output

**Eelco Dolstra, Utrecht University, 2006.** PhD thesis: *"The
Purely Functional Software Deployment Model"*. Then Nix the package
manager; then NixOS; then flakes (~2019); then the broader ecosystem
(home-manager, nix-darwin, devenv).

**R: across-machine, across-time** (the gold standard). **D:
by-construction** (within the language; builders can still leak).

**What it solved.** A build's identity is the *hash of its recipe*:
the derivation. Two builds with byte-identical recipes have
byte-identical outputs (or one of them is broken). The store is
`/nix/store/<hash>-<name>`; the hash names the dependency graph that
produced the build, recursively. Upgrades and rollbacks are atomic
because nothing mutates: each version lives at its own hash; the
symlinks swap.

**Where it (still) leaks.**

- **Non-deterministic builders.** Nix derivations are sandboxed and
  PATH-controlled, but the *builder* (the actual compiler / linker)
  can still produce different bytes across runs. C++ with parallel
  codegen, Java with `.class` file ordering, JavaScript bundlers with
  hash-randomized module IDs — all leak. The Nix community catalogs
  these in
  [r13y.com](https://r13y.com)
  (NixOS reproducibility tracker), with the leak rate at low
  single-digit percent for the package set.
- **`__noChroot` builds.** A derivation can opt out of the sandbox
  (for builds that genuinely need network or special devices). When
  used, the reproducibility guarantee is voided for that derivation
  and everything downstream of it.
- **Fixed-output derivations.** Derivations whose output hash is
  pre-declared (e.g. tarballs fetched from upstream) trust the
  upstream URL to be content-stable. Most are; some aren't (GitHub's
  auto-generated tarballs are not byte-stable across changes to git's
  tar implementation, a known issue).
- **Floating-point determinism.** Builders that use floating-point
  arithmetic across architectures can produce different bit patterns
  (x86 80-bit extended precision vs SSE2 64-bit), unrelated to Nix
  itself but surfacing through it.

The Nix reproducibility tracker is the honest acknowledgment: even the
gold standard has measurable leaks. The leak rate is small enough
that Nix is the practical answer today; the residue is the field's
open problem.

**What it taught the field.** *Hash-of-recipe → hash-of-output is the
build's true identity.* The single most important insight in the
forty-year lineage above. mtime is wrong; declared inputs are
incomplete; the recipe itself, content-hashed, is the only honest
staleness signal. Every other modern build system either inherits
this idea (Bazel's action key is structurally a derivation hash) or
works around its absence.

**Inheritance for kintsugi.** The Nix inheritance is the deepest of
the build-system lineage:

- The substrate's content-addressing (`Splinter<H>` — OID-as-identity)
  is *exactly* the Nix derivation hash, abstracted over the hash
  backend.
- The reproducibility story (same bytes in, same bytes out) is
  inherited verbatim.
- The crystallization step — substrate ref to Rust body — is a *bound
  derivation*: the OID of a `Crystallization<H>` is the hash of
  (path, body-impl-hash), the same shape as a Nix derivation.
- The non-deterministic-builder leak that Nix tolerates as a residue
  is what kintsugi has to address explicitly for its au column. The
  fate substrate is structurally a "builder" that produces an au
  value; the discipline that makes fate's builder deterministic
  (pinned model OID, fixed seed, temperature zero, deterministic
  sampling policy) is the work in [[kintsugi-thesis]] §3.

The honest comparison: Nix achieves across-time reproducibility for
~95% of packages and a slowly-shrinking leak set; kintsugi must
achieve the same bar for its hand-written column *and* the au column.
The hand-written column inherits Nix's discipline directly. The au
column inherits *neither* a tradition nor a reference implementation;
that's where the engineering substance is.

### 1.8 Cargo — explicit non-reproducibility

**Rust, ~2014, Yehuda Katz & Carl Lerche.** Per-package build with
workspace + features. Lock-file for dependency resolution.

**R: per-machine**, possibly. **D: no.**

**What it solved.** A clean per-package build with strong dependency
resolution. `Cargo.lock` pins the dependency graph; `cargo build`
within one machine is repeatable.

**Where it leaks, structurally.**

- **`build.rs` runs arbitrary code.** A `build.rs` script can call
  the network, read the system clock, embed compile-time strings,
  link to system libraries by name. This is by design (it's the
  ecosystem's escape hatch for OS-specific integration) and it is
  the load-bearing reproducibility leak in the Rust toolchain. There
  is no way to forbid `build.rs` while keeping the existing
  ecosystem.
- **rustc's incremental compilation cache is not deterministic
  across machines.** Incremental compilation depends on internal
  fingerprints that can vary; the *final* binary is reproducible
  with `-C codegen-units=1` and disabling incremental, but the
  intermediate cache is not portable.
- **Procedural macros run at compile time.** A proc-macro can do
  anything `build.rs` can do; same leak.

**What it taught the field.** *Reproducibility cannot be opt-in
through user discipline alone.* If the build system allows arbitrary
code, the build is not reproducible unless every actor consistently
opts into reproducibility — and they don't. Cargo's lockfile fixes
dependency resolution but not the build. The reproducible-builds
community has documented this extensively; the upshot for
distributors (Debian, NixOS, Alpine) is that Rust crates require
case-by-case audit and often patching.

**Inheritance for kintsugi.** Cautionary. The substrate-pull
discipline is the structural answer: a `Body<H>` cannot type-check
as `pure` today (Rust's type system has no `pure` annotation), but
the substrate-side declaration `requires deterministic(foo)` is a
property check that can refuse a body that isn't deterministic. The
property check today is `\` (parked) for Rust bodies; closing it is
named in [[kintsugi-thesis]] §3.8.

### 1.9 Mix, Turbo, Nx — per-ecosystem builds

**Mix (Elixir, ~2012, José Valim).** Task-based; the pipeline shape
is clean enough that downstream task-based tools (Rake, Leiningen)
all resemble it.

**Turbo (Vercel, ~2022) and Nx (Nrwl, ~2017).** Content-based
caching for JavaScript monorepos.

**R: per-machine to across-machine.** **D: with-care.**

**Where they leak.** Each one carved its own content-cache discipline
on top of an ecosystem (Node.js, Elixir) that didn't have hermetic
defaults. JavaScript in particular: webpack/esbuild/rollup outputs
were historically non-deterministic (hash-randomized module IDs,
parallel emit ordering); the JavaScript reproducibility story is
worse than the C/Rust story by an order of magnitude. Turbo and Nx
mitigate but don't close.

**Inheritance for kintsugi.** Mix's task-shape (`mirror compile /
craft / kintsugi`) is honestly an optical-vocabulary Mix. The Cargo
lesson — monolithic per-package is too coarse — is one mirror avoids
by making the *grammar*, not the directory, the unit of
recompilation.

### 1.10 Earthly, Dagger — composable function builds

**Earthly (Vlad Ionescu, 2020).** Dockerfile-as-build-system; each
target runs in a container.

**Dagger (Solomon Hykes et al., ~2022).** The CI pipeline IS a
function in a typed programming language (Go, Python, TypeScript). It
runs builds in BuildKit containers under the hood; the conceptual
move is that *pipelines are values* you compose, not YAML files you
write.

**R: per-container** (the container layer is reproducible if
Buildkit's layer hashes are stable; toolchain inside is whatever the
container had).
**D: with-care** (containers enforce some hermeticity; the language
above is general-purpose).

**Where they leak.** Container hermeticity gets you "this commit
runs this image" but not byte-identity of the final artifact unless
every layer in the image is reproducible — which is the
docker-reproducibility problem (a different research area; see
[reproducible-builds.org/docs/source-date-epoch](https://reproducible-builds.org/specs/source-date-epoch/)
for the canonical workaround).

**What this generation taught the field.** *Build composition can be
function composition.* This is the closest the prior art gets to the
kintsugi posture. Where Dagger sits structurally below kintsugi:
Dagger composes hand-written functions in a host language; kintsugi
composes substrate-declared actions whose bodies may be hand-written
*or au-inferred*.

**Inheritance for kintsugi.** Strong. If kintsugi had a closest
living relative in 2026, it would be Dagger. The differences are: (a)
the substrate is mirror, not a host language, so the function bodies
are grammar declarations not Go / Python code; (b) the typed inference
slot; (c) the conductivity-verdict admission discipline; (d) the
substrate-pull invariant that keeps capability out of the floor. The
reproducibility story Dagger has is the BuildKit story; kintsugi
needs its own, and that's the work.

---

## 2. The CI/CD lineage

CI/CD is younger than build systems by about thirty years. Most of
its discoveries are operational rather than conceptual; the
conceptual progress that matters is the marketplace pattern, the
verb / noun split, and the failure modes of pipelines-as-YAML.

CI/CD reproducibility is, in the honest case, *per-pipeline-run*: a
given commit kicks off a given pipeline that produces some artifacts.
*Across runs* of the same pipeline, reproducibility is rare — fresh
VMs, network-pulled tools, package indices that drift.

### 2.1 CruiseControl / Jenkins / Hudson — plugin chaos

**CruiseControl (ThoughtWorks, 2001).** Hudson (Kohsuke Kawaguchi,
Sun, 2005) → Jenkins (community fork, 2011).

**R: no.** **D: no.**

**Where it leaks.** Jenkins's plugin ecosystem grew to tens of
thousands of plugins, many abandoned, many security-relevant, many
incompatible. The plugin contract was a verbal agreement; verbal
agreements rot. Build reproducibility was never the point; the point
was running tests on commits.

**What it taught the field, the hard way.** *Extensibility without
typed contracts becomes a debt graph.* The Crystallizations table is
the positive answer: a single typed seam (`Ref → Body<H>`), small
closed surface, marker discipline (`[substrate-pull:realize]`) in
`git log`.

### 2.2 Travis / CircleCI — YAML pipelines as the ceiling

**Travis CI (2011), CircleCI (2011).**

**R: no, by design.** **D: no.**

**Where they leak.** YAML pipelines run in fresh VMs that pull tools
from upstream registries (Docker Hub, npm, PyPI). Network access is
unrestricted; package indices drift; the same `.travis.yml` run at
different times produces different artifacts. The Travis / CircleCI
generation never promised reproducibility — they promised
fast-feedback automation.

**What they taught the field.** Declarative pipelines are easier to
reason about than imperative ones, but YAML hits its ceiling fast.

**Inheritance for kintsugi.** Mirror's substrate is not YAML, by
design. The grammar declarations are a real language with a
sub-Turing invariant.

### 2.3 GitHub Actions — marketplace as body registry

**GitHub / Microsoft, 2018 GA.**

**R: per-run, rarely across-run.** **D: with-care.**

**What it solved.** A reusable-step pattern with discoverability
built in. The matrix-strategy idiom. Composite actions for
cross-language reuse.

**Where it leaks.**

- **`uses: actions/checkout@v4` is a moving pointer.** The `v4` tag
  can be re-tagged silently to point at a new SHA; only pinning
  `uses: actions/checkout@<full-sha>` makes the action reproducible.
  Most workflows in the wild use the moving tag.
- **Runner images change weekly.** The `ubuntu-latest` runner gets
  package updates between runs. A workflow that worked on Monday can
  break on Tuesday for reasons unrelated to the workflow's own code.
- **Network access is unrestricted.** Steps can install anything
  from anywhere.

**What it taught the field.** *A marketplace IS a body registry.*
The `uses: actions/checkout@v4` line is, structurally,
`Crystallizations`'s `lookup(@actions/checkout) → the v4 body`. The
marketplace's content hashes (the action's git SHA) make the binding
reproducible *when used with SHA pinning*; the convention of using
moving tags is the leak.

**Inheritance for kintsugi.** The marketplace pattern, with type
discipline added and SHA-pinning as the default. Crystallizations is
the body registry; the `@`-prefixed Ref is the marketplace
identifier; the `Body<H>` shape is the typed contract Actions never
grew. The au column is the inference slot Actions doesn't have.

### 2.4 Tekton — verbs and nouns

**Continuous Delivery Foundation / Google, 2019.**

**R: per-pipeline-run.** **D: with-care.**

**What it taught the field.** Separate verbs (Task / Pipeline) from
nouns (TaskRun / PipelineRun). The right shape: the definition is the
description, the execution is the event, and the event is queryable
as a first-class object.

**Inheritance for kintsugi.** A `Crystallization<H>` is a *Task* (the
binding); a `crystallize` call is a *TaskRun* (the event). A
`Fracture` declaration is a *Pipeline*; `kintsugi_tick` evaluating a
fracture is a *PipelineRun*. The fit is good enough that kintsugi
could in principle deploy as a Tekton controller variant — but the
reproducibility story is stronger (content-addressed all the way down,
not just at the artifact registry).

### 2.5 Drone, Concourse, Argo Workflows — container-as-step

**Drone (Brad Rydzewski, 2014); Concourse (Pivotal, 2014); Argo
Workflows (Intuit → CNCF, 2017).**

**R: per-container.** **D: with-care** (the container is the
hermeticity boundary).

**Where they leak.** Container hermeticity is real but not absolute:
the kernel below the container, the cgroup limits, the network
namespace, the host volume mounts — every one of these is a
non-reproducibility surface. The reproducibility-of-builds people
have shown that *image* reproducibility is achievable with care
(`SOURCE_DATE_EPOCH`, deterministic image-build tooling); the
*runtime* reproducibility within the container is what the operator
discipline buys.

**Inheritance for kintsugi.** Operational only. Where containers
gave the CI generation a *practical* hermeticity floor, mirror gets
*mathematical* hermeticity from the @fate substrate invariant. The
two stories aren't incompatible: a kintsugi-built artifact can run
inside a container in a Kubernetes pod, and the kintsugi verdict
travels in the artifact's store-OID for free.

### 2.6 Garden, Skaffold — multi-service dev loops

**Garden.io (~2018), Skaffold (Google, 2018).**

**R: per-machine** (dev-loop reproducibility is not the goal). **D:
no** (file watchers, ports, port-forward state).

**What they taught the field.** Multi-service builds are graphs of
stacks, not graphs of files. The unit of recompilation is the
service; the unit of redeployment is the stack.

**Inheritance for kintsugi.** The graph-of-stacks framing maps to the
multi-substrate idea: a kintsugi build artifact can span grammars,
prisms, glasses, fragmentations.

---

## 3. The GitOps + declarative IaC lineage

This lineage is the closest spiritual ancestor of kintsugi's
reconciliation loop. The reproducibility story here is different from
the build-system story: GitOps is about *converging the observed
world to the declared world*, not about *making the build artifact
byte-identical*. The drift-detection idiom is the load-bearing
inheritance.

### 3.1 Terraform — plan, apply, drift

**HashiCorp, 2014.**

**R: convergence-based, not byte-based.** **D: no** (cloud APIs are
inherently non-deterministic).

**What it solved.** Cloud resources as declarative HCL; a state file
that records the resources Terraform thinks exist; a `plan` step that
compares declared-vs-observed; an `apply` step that executes the
diff. Drift detection became a first-class concept.

**Where it leaks.**

- **Provider non-determinism.** Cloud APIs return timestamps,
  auto-generated IDs, machine-allocated IPs — none of these are
  deterministic functions of the request. Terraform's state file
  captures them as observed values; replaying the same config
  produces different state.
- **The state file is mutable shared state.** Lock contention and
  state corruption are first-class failure modes the industry has
  built whole products (Terraform Cloud, Atlantis) to mitigate.

**What it taught the field.** *Declare state, detect drift,
converge.* The Terraform state file is, structurally, a `Splinter`
analogue: a serialised snapshot of the observed system,
content-addressable in principle, used as the input to the next
reconciliation. The drift-detection idiom is exactly what kintsugi
needs: `Crystallizations` knows what's *registered*, the store
knows what's *been crystallized lately*, and the delta is the work to
do.

**Inheritance for kintsugi.** The drift-detection idiom is the
mechanical model for kintsugi's reconciliation:

```
  declared    := source of truth (Crystallizations registration intent)
  observed    := the current store (Splinter OIDs known)
  drift       := declared - observed
  apply       := for each missing ref: crystallize
  convergence := the loop terminates when drift is empty
```

The state file's shared-mutable failure mode is something kintsugi
avoids by construction: the store is content-addressed and
append-only; "the state" is always recoverable from the store's
OIDs, never maintained as a separate mutable artifact. Where
Terraform must trust the cloud API's reported state (which is not
deterministic), kintsugi re-hashes the store and compares — drift
detection is byte-based, not API-based.

### 3.2 Pulumi — IaC in real languages

**Pulumi Corp, 2017.** Terraform-shaped but with TypeScript / Python
/ Go / C#.

**R: same as Terraform.** **D: with-care** (host language is general
purpose; can do anything).

**Inheritance for kintsugi.** Cautionary. Mirror's discipline comes
from the substrate (sub-Turing, content-addressed, glass-walled).
Kintsugi does not need to be in Rust or Python; it needs to be in
mirror.

### 3.3 Crossplane — cloud as CRD

**Upbound, 2018.**

**Inheritance.** The kintsugi engine is structurally a controller: it
watches the declared substrate, observes the store, and drives the
store toward the substrate's declared state. The difference is
altitude: Crossplane drives cloud resources; kintsugi drives build
artifacts.

### 3.4 ArgoCD, Flux — reconciliation at scale

**ArgoCD (Intuit → CNCF, 2018), Flux (Weaveworks → CNCF, 2016).**

**R: per-deployment.** **D: with-care.**

**What they taught the field.** Declared state in git, observed
state in cluster, controller drives convergence. The eventual-
consistency discipline they brought to deployment automation is the
conceptual ancestor of `kintsugi_tick` running as a long-lived
process: each tick is one reconciliation step; the loop converges
when the OID-tree of the observed state matches the OID-tree of the
declared state.

**Inheritance for kintsugi.** Direct, named. `kintsugi_tick` is a
reconciliation loop with eventual consistency over the substrate. The
store's append-only OID-addressed history is the observed-state log.
The substrate declarations are the declared-state source of truth.

### 3.5 Kubernetes operators

**CoreOS, 2016, with the operator-sdk landing ~2018.**

**Inheritance.** Kintsugi-as-controller: the
`@kintsugi/fracture/*` declarations are CRDs; the `Crystallizations`
table is the operator's bound logic; `kintsugi_tick` is the
reconciliation step.

### 3.6 Nix flakes — the closest existing system

**Nix flakes (~2019 RFC, ~2021 widely adopted).** Pinned, hermetic,
content-addressed Nix derivations as the unit of build / deploy.

**R: across-machine, across-time** (the strongest reproducibility
story in production today). **D: by-construction** (within the Nix
language; builders can still leak per §1.7).

**What it taught the field.** Hermeticity + content-addressing +
locked dependencies can be packaged as a unit (the flake) and
shared. The `flake.lock` file is, structurally, a Splinter analogue
at the project root.

**Inheritance for kintsugi.** The flake is the closest spiritual
ancestor of the kintsugi build artifact. The differences are: (a)
mirror's substrate is multi-grammar where a flake is mono-language
(Nix); (b) the engine layer (`@spectral/db`) exposes the dependency
graph as a queryable surface where flakes leave it implicit; (c) the
au column has no flakes analogue. But the discipline — pinned
content-addressed hermetic — is the same.

The honest claim: **kintsugi's hand-written column inherits Nix's
reproducibility story directly**. The substrate-declared
crystallization-of-a-Rust-body is shaped exactly like a Nix
derivation; the determinism leaks Nix has (non-deterministic
builders) are the determinism leaks kintsugi has, at the same
altitude, with the same workarounds (`SOURCE_DATE_EPOCH`,
`-C codegen-units=1`, deterministic linker order).

What Nix does not yet have is the au column. That is the new work,
and it is the only new work.

---

## 4. The AI-in-build territory — where every current system fails

Today, no production build system in the Anthropic/OpenAI-era LLM
landscape achieves Nix-grade reproducibility for AI-produced outputs.
This section is honest about that, and about whether kintsugi can do
better. The answer is "yes, if four specific things are pinned" —
those things are named in [[kintsugi-thesis]] §3.

### 4.1 GitHub Copilot, Cursor, Windsurf, Zed AI — editor surfaces

**R: no.** **D: no.**

**Where they sit.** *Above the build line.* The AI proposes; the
build verifies. The build itself has no idea AI was involved; the
AI's output is code that humans then run through ordinary CI.

**Reproducibility properties.** The model version is GitHub's /
Anthropic's / OpenAI's choice; it changes silently. Temperature is
default-non-zero. The "same prompt, same output" property is not
provided; not even promised. A team that wanted to reproduce a
Copilot suggestion six months later cannot — the model that produced
it has been deprecated.

**Where kintsugi sits relative to this.** Strictly below. Inside the
build graph. The au-typed body is *in* the build, not above it; its
inputs are content-addressed; the model that produced it is
content-addressed; the cache key includes the model OID.

### 4.2 Sweep, aider, OpenHands, Devin — agentic coding

**R: no.** **D: no.**

**Where they sit.** *Around the build.* The agent's loop *includes*
the build as a tool call; the agent is not part of the build. Same
non-reproducibility as 4.1, with the additional non-determinism of
tool-call ordering (the agent's choices depend on the model's
sampled output at each step).

**Honest tension.** An agent that writes a PR which then runs through
kintsugi is a different setup from kintsugi-with-au-bodies. The
first is an agent on top of a normal build (non-reproducible
proposal, reproducible verification); the second is a build with
inference as a column (the inference itself is reproducible because
its inputs are pinned). Both can coexist; they are not the same
proposition.

### 4.3 MetaGPT, AutoGen, CrewAI, AutoGPT — orchestration frameworks

**R: no.** **D: no.**

Same as above. Orchestration is not part of the build graph.

### 4.4 LLM determinism, as a question of operator discipline

Are LLMs ever deterministic? In principle, yes:

- Temperature = 0 (greedy decoding).
- Fixed seed for any sampling step.
- Pinned model weights (the weight tensor is bytes; the bytes have a
  hash; pin the hash).
- Deterministic kernel implementations (some matrix multiplication
  libraries — Triton, cuBLAS — produce different bit patterns across
  hardware or across runs even at temperature=0; this is a known
  issue for which the workarounds are documented in PyTorch's
  reproducibility docs).
- No system-level randomness (no Python `hash(str)` randomization
  affecting tokenization order; no `random.seed` left unset).

When all of these hold, a forward pass through an LLM is a
deterministic function of (model weights, prompt, sampling
parameters). In practice this combination is rarely deployed:
production LLM endpoints non-determinise on purpose (better-looking
samples at temperature ≈ 0.7), and the model behind the endpoint
changes silently.

The kintsugi proposition is that the substrate enforces this
combination by construction, not by operator discipline:

- `@fate` refuses remote inference (mathematically, via the
  `local` universal property in `AGENTS.md`). No silent model
  drift.
- The model OID is part of the cache key for every au-typed value.
  Cache miss = explicit rebuild.
- Temperature and seed are substrate-declared parameters; the
  fracture's OID includes them.
- Deterministic kernels are a `requires deterministic(@fate.infer)`
  property that the model checker can refuse on platforms that
  cannot satisfy it (e.g. GPU substrates that admit non-deterministic
  reduction order). On such platforms, fate runs on CPU paths only,
  or the property fails and the build refuses.

This is engineering, not novelty. The work is to wire each of these
in and prove the chain. [[kintsugi-thesis]] §3 is that work.

### 4.5 Honest research counter-examples

A few research directions deserve naming, because they flirt with the
same shape from a different angle:

- **Auto-tuning compilers — Halide schedules (Ragan-Kelley et al.,
  MIT, 2013; auto-scheduler 2019).** A compiler's *schedule* is
  auto-generated by a search procedure with a learned cost model.
  The schedule IS a build artifact whose body is inferred. Halide's
  schedules ARE deterministic given the same search seed and cost
  model — this is the closest existing system to "reproducible
  inferred build output", and the discipline of pinning the cost
  model's training hash is exactly what kintsugi needs for `@fate`.
- **Learned superoptimizers — STOKE (Schkufza et al., 2013), Bansal
  & Aiken (2006), neural variants.** A superoptimizer searches for a
  shorter or faster program with the same observable behaviour. The
  admission criterion is observational equivalence on a test suite;
  kintsugi's admission criterion is a composite of
  `@epistemologic/properties`. Different admission shape; same
  proposition shape.
- **Program synthesis under contracts — Sketch (Solar-Lezama, MIT,
  2008), Synquid (Polikarpova et al., 2016), types-driven holes in
  Idris / Agda / Liquid Haskell.** Sketch's `??` holes are mirror's
  `{ \ }` parked obligations; Sketch's contract is mirror's
  `requires` properties; Sketch's synthesis is fate's inference.
  Sketch is reproducible *given the SAT solver and search ordering
  are deterministic*; both Z3 and CVC5 have deterministic modes.
  The kintsugi discipline is the same flavour: pin the seed, pin the
  search, get the same output.
- **DeepCoder, AlphaCode, code-generation models writing entire
  programs.** Above the build line; non-reproducible.

The honest claim: **research has shown reproducible inferred output
is possible** (Halide auto-scheduler, Sketch, Synquid). **Production
has not delivered it for LLMs.** Kintsugi's work is the production
delivery — not a research breakthrough, but the engineering of pinning
each of (model OID, prompt OID, temperature, seed, sampling policy,
kernel implementation) into the substrate's cache key.

### 4.6 The gap, named precisely

The gap is not "AI is missing from build systems." AI is everywhere
near builds (above them, around them, inside their schedules). The
gap is *the build-graph-resident AI body whose output is
reproducible*:

- A build graph that has a column whose values are au.
- Au's content address includes: prompt OID, model OID, temperature,
  seed, sampling policy OID.
- Same (prompt, model, temperature, seed, sampling) → same au value,
  byte-identical.
- Verification of an au-bound joint is a verdict over
  `@epistemologic/properties` at the joint's altitude, not a
  confidence score.
- The verdict composes deterministically via
  `PropertyVerdict::merge_with` (Pass / Partial / Fail dominance is a
  pure function).
- The hermeticity invariant holds because @fate refuses remote
  inference — the inference is local by substrate law, not by
  network policy.

No production build system in 2026 has this combination together. The
closest research — Sketch / Synquid for synthesis discipline,
Halide for inferred-output reproducibility, Nix for hash-of-recipe
reproducibility — has the pieces but not the join. Kintsugi proposes
to be the join, and the work is to deliver each piece without
compromise.

---

## 5. The wisdom, distilled — what kintsugi inherits, and what it owes

| Generation | Reproducibility lesson | Determinism lesson | Kintsugi inheritance |
|---|---|---|---|
| Make (1976) | mtime is a lie. | Recipes are shell; shell is not deterministic. | DAG framing; never mtime-based staleness. |
| Plan 9 mk (~1989) | Parsimony makes leaks fewer. | Same as Make. | Small floor; substrate-pull. |
| Vac / Venti (2002) | A block's name is its hash. | Hash is a pure function. | `Splinter<H>`; Merkle-style OID. |
| Bazel (2007 / 2015) | Hermeticity by sandbox; cache key by recipe hash. | Sandbox enforces; toolchain doesn't. | Mathematical hermeticity via @fate's local invariant; Crystallizations as typed action cache. |
| Nix (2006) | Hash-of-recipe → hash-of-output. Non-deterministic builders are the residue. | Pure Nix language; impure builders the leak. | Direct: `Splinter<H>` is the derivation hash, abstracted. Inherits the non-deterministic-builder leak; resolves it for the au column by pinning model + seed + sampling. |
| Cargo | `build.rs` is the leak. | Reproducibility cannot be opt-in. | Substrate-pull discipline as the structural answer; `requires deterministic` as the property check. |
| Mix / Turbo / Nx | Per-ecosystem cache discipline retrofit. | Per-ecosystem with-care. | Task-shape; grammar (not file) as unit of recompilation. |
| GitHub Actions (2018) | Marketplace with content-addressed bodies; moving tags are the leak. | Per-action; runner image drift. | Crystallizations as typed marketplace; SHA-pinning by default. |
| Terraform (2014) | Declare-detect-converge; state file is the analogue of Splinter. | Cloud APIs are inherently non-deterministic. | Drift-detection model for `kintsugi_tick`; byte-based not API-based. |
| Tekton (2019) | Verb / noun split; first-class observability of events. | Same as containers below. | `Crystallization<H>` ⇆ Task; `crystallize` call ⇆ TaskRun. |
| ArgoCD / Flux | Reconciliation loops; eventual consistency. | Per-deployment. | `kintsugi_tick` as the reconciliation step; OID-tree equality as the convergence check. |
| K8s operators | The world is CRDs + controllers. | Per-controller. | Kintsugi as controller; `@kintsugi/fracture/*` as CRDs. |
| Nix flakes (~2019) | Pinned + hermetic + content-addressed bundle. | By-construction within Nix. | Closest existing system; au column is the missing piece. |
| Dagger (2022) | Build composition = function composition. | With-care; host-language general. | `Body<H>: Fn(…) -> Imperfect<…>`. |
| Sketch / Synquid | Synthesis under typed contracts. | Deterministic given pinned solver + seed. | Au is the body; `requires` properties are the contract; @fate is the synthesis. |
| Halide auto-scheduler | Reproducible inferred build output (given pinned cost model). | Deterministic given fixed search seed. | Existence proof that pinned-seed inference can be reproducible. |
| Production LLMs | Non-reproducible by default; model drift is silent. | Non-deterministic by default. | The gap kintsugi must close: pin model OID + temperature + seed + sampling in the cache key. |

None of these are claimed as kintsugi's invention. The kintsugi
contribution is the *recomposition* of these lessons such that the
reproducibility chain survives the addition of an AI-inference column.
Every individual property has prior art; the integration is the work.

---

## 6. What I couldn't verify in this round — gaps to revisit

WebSearch and WebFetch were unavailable during this session; the
following citations are from training knowledge (cutoff Jan 2026) and
deserve a fresh-read confirmation before publication:

1. **Tom Duff *mk* paper exact citation.** I cited "mid-1980s
   USENIX"; the year and venue want confirmation.
2. **Buck2's "perfectly reproducible builds" team talks.** Meta has
   given several public talks; the most recent (~2024–2025) is the
   one to cite for the current state of the discipline.
3. **Eelco Dolstra's PhD thesis** ([Dolstra 2006](https://edolstra.github.io/pubs/phd-thesis.pdf)).
   The reproducibility claim's primary source; I cited from memory.
4. **NixOS reproducibility tracker, r13y.com.** The leak-rate number
   I cite (~5%) is approximate; the live data wants checking.
5. **`SOURCE_DATE_EPOCH` standard.** The reproducible-builds.org
   project page; cited from memory.
6. **The PyTorch reproducibility docs** ([pytorch.org/docs/stable/notes/randomness.html](https://pytorch.org/docs/stable/notes/randomness.html)).
   The canonical reference for "what determinism costs on GPU."
7. **Halide auto-scheduler papers** (Adams et al., SIGGRAPH 2019).
   The reproducibility claim about pinned cost models needs a
   careful read of the actual paper.
8. **Sketch / Synquid follow-ups.** The 2008 thesis and Synquid (2016)
   are canonical; the LLM-synthesis-with-contracts work (2023–2026)
   wants a survey.

These gaps do not change the synthesis. They would tighten specific
claims; the structural argument — reproducibility + determinism are
the bar, and kintsugi can hit them if four specific pieces are
pinned — stands.

---

*Forty years of build systems. Twenty years of CI/CD. Ten years of
GitOps. Three years of agentic AI. Every generation closed one leak
and left another. Kintsugi inherits the closures and owes the residue.*
