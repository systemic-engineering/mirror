# docker-container-substrate-decl-v0.1 — `@code/docker` + `@container` deep-dive spec draft

*Mara, deep dive commissioned by Alex via Reed on 2026-07-06 as the
substrate-pull-honest response to the StageFreight Stream-2 tick
blocker (docker was external + not running; substrate cannot settle
against an unreachable daemon). This spec drafts the family-tree
shapes for the `@code/docker` species (code altitude; Dockerfile as
declarative code) and the `@container` family-root candidate
(runtime altitude; process isolation + rootfs + capabilities as
substrate primitive).*

*Spec-draft ONLY. NO shards land with this tick. NO Rust ships. NO
bootstrap edges wired. The forward-promised species roster,
recognition candidates, and family-root discipline discharge in
subsequent TDD-paired ticks. Path α, β, γ ranked below; §5 names
Mara's substrate-pull confident pick with the hedge surfaced.*

**Status:** substrate-pull-draft. Family-tree shapes proposed;
canonical prior art cited from OCI / BuildKit / containerd /
runc / cri-o (Kagi-verified this session); substrate-audit lists
the seven ancestors mirror already carries; three recognition
candidates surfaced (§7); five first-species candidates named for
the recommended shape (§6).

**Audience:** any agent or human reading this before touching
`shards/code/docker.mirror`, `shards/container.mirror`, or the
StageFreight daemon-dependency blocker whose resolution this spec
substrate-pull-anchors. Read this; then chase
`shards/io/oci.mirror` for the distribution-altitude substrate-decl
this spec composes with; then chase `mosaic-store-cache-invariants.md`
for the cache-invariant discipline that grounds Dockerfile layer
caching at substrate altitude.

---

## §1 — Recognition + context

### 1.1 The blocker that surfaced the substrate-pull

Alex's 2026-07-06 direction (via Reed, verbatim framing):

> The recent StageFreight Stream 2 tick blocked because docker was
> external + not running. Substrate-pull-honest answer: make docker
> part of the substrate. `@code/docker` at code altitude (Dockerfile
> as declarative code / image build semantics); `@container` at a
> family-root altitude (runtime abstraction — process isolation +
> rootfs + capabilities).

The blocker names the substrate's current epistemic horizon at the
container altitude: mirror has substrate-decl'd the DISTRIBUTION
surface (`shards/io/oci.mirror`, Mara `2801478` 2026-06-24; OCI
Distribution Spec v1.1.x + OCI Image Spec v1.1.x adapter) but has
NOT substrate-decl'd the BUILD surface (Dockerfile / BuildKit LLB;
what PRODUCES the OCI artifact @io/oci ships) or the RUNTIME
surface (runc/crun/containerd; what CONSUMES the OCI artifact
@io/oci ships).

The three surfaces:

```
  build surface       @code/docker    (Dockerfile as declarative code)
     ↓ PRODUCES
  distribution surface @io/oci        (LANDED 2026-06-24)
     ↓ CONSUMED BY
  runtime surface     @container      (process isolation + rootfs + caps)
```

`@io/oci` sits BETWEEN build and runtime. The substrate has the
middle. It's missing the ends. The StageFreight-daemon blocker is
what surfaces the missing runtime end — StageFreight's runtime
integration reaches for a docker daemon that isn't there, and the
substrate has no typed vocabulary to describe the reach OR the
absence.

### 1.2 Why now (substrate-pull-signal)

Per `[[feedback-substrate-already-had-the-word]]`: mirror has been
implicitly running against docker for weeks via StageFreight
consumers, via CI infrastructure, via `dockerTools` on the Nix
substrate ouroboros (per `shards/io/oci.mirror` §5). What was
missing was the TYPED substrate-decl for the surfaces the substrate
was already touching. This spec draft is the 54th+ instance of that
pattern: the substrate had the word (Dockerfile, container,
runtime, rootfs, layer) at the operational altitude; it just
hadn't lifted the naming to the substrate-decl altitude.

The StageFreight-daemon blocker IS the substrate-pull signal per
`[[feedback-substrate-pull]]`. Blocking on an external, unreachable,
untyped dependency is what tells the substrate the boundary needs
naming. The response — per `[[feedback-substrate-pull-confidence-acts]]`
— is not to route around the blocker with a mock or a bypass; it is
to substrate-decl the surface the blocker points at.

### 1.3 What this spec IS

Family-tree draft for the two substrate-decl surfaces:

- **`@code/docker`** (species under `@code/`): Dockerfile as
  declarative code. The BUILD-side grammar. Composes with existing
  `@code` family via `in @code`. Sibling to `@code/rust`,
  `@code/gleam`, `@code/mirror`, `@code/wasm`, `@code/erlang`.
- **`@container`** (family-root candidate at top level): process
  isolation + rootfs + capabilities + namespaces. The RUNTIME
  surface. Composes with `@io/oci` (distribution) and `@code/docker`
  (build) at their respective altitudes.

### 1.4 What this spec IS NOT

1. NOT a substrate-decl landing. Zero shards touch. Zero Rust
   ships. The recommended shape (§5) is the SPEC PROPOSAL; landing
   requires a Pack-adjudicated substrate-pull cascade (RED first
   per `[[feedback-always-tdd-no-shortcuts]]`).
2. NOT a resolution of the StageFreight-daemon blocker. The blocker
   is the substrate-pull SIGNAL that surfaced the missing surface;
   the resolution is the subsequent-tick TDD cascade that lands
   what this spec proposes, plus the StageFreight-side consumer
   integration.
3. NOT a commitment to Docker-the-vendor. `@container` names the
   RUNTIME family; docker is one runtime; runc, crun, podman, cri-o,
   containerd are others. Species-parameterize the family; the
   family itself is vendor-agnostic per OCI's standardization.
4. NOT a re-parenting of `@io/oci`. The OCI distribution adapter
   stays at `@io/oci` as landed; `@container` composes AGAINST
   `@io/oci` at runtime, `@code/docker` composes AGAINST `@io/oci`
   at build.

---

## §2 — Prior art (Kagi-verified, this session)

### 2.1 OCI Image Spec v1.1.x

**Canonical URL:** https://specs.opencontainers.org/image-spec/config/

**Authoritative citation:** *"The rootfs key references the layer
content addresses used by the image. This makes the image config
hash depend on the filesystem hash."*
(OCI Image Spec, config.md)

**And on the manifest:** *"There are three main goals of the Image
Manifest Specification. The first goal is content-addressable
images, by supporting an image model where the image's
configuration can be hashed to generate a unique ID for the image
and its components."*
(OCI Image Manifest Specification)

**And on the ImageID:** *"Since the configuration JSON that gets
hashed references hashes of each layer in the image, this
formulation of the ImageID makes images content-addressable."*
(image-spec/config.md, main branch)

**RIGOROUS abstraction:** the OCI IMAGE is not a "container" — it
is a MANIFEST (JSON descriptor listing layers by SHA256) + a
CONFIG (JSON metadata: os, arch, env, cmd, entrypoint,
rootfs.diff_ids) + an ORDERED SEQUENCE OF LAYERS (tar archives
encoding filesystem diffs). The image is content-addressed at
three levels: manifest hash (image ID), config hash, layer hashes.
Splinter-pole: byte-identity end-to-end. Narcissus-pole: layers
present but manifest doesn't hash to declared digest.

**Substrate mapping:** already landed at `shards/io/oci.mirror`
carrier set (`oci_digest`, `oci_manifest`, `oci_repository`,
`oci_reference`, `oci_artifact`, `oci_registry`). This spec does
NOT re-substrate-decl these; it composes AGAINST them.

### 2.2 OCI Runtime Spec (runc/crun contract)

**Canonical URL:** https://github.com/opencontainers/runtime-spec/blob/main/config.md

**Authoritative citation:** *"This configuration file contains
metadata necessary to implement standard operations against the
container. This includes the process to run, environment variables
to inject, sandboxing features to use, etc."*
(runtime-spec, config.md)

**Structural shape:** the runtime spec defines `config.json` +
`rootfs/` as the on-disk contract runc consumes. `config.json`
carries:

- `process` — argv, env, cwd, capabilities, rlimits, oomScoreAdj
- `root` — path to rootfs + readonly flag
- `mounts` — bind mounts, tmpfs, proc/sysfs
- `linux.namespaces` — pid, net, ipc, uts, mount, user, cgroup
- `linux.resources` — cgroups memory/cpu/pids/devices
- `hooks` — prestart/poststart/poststop callbacks

**RIGOROUS abstraction:** a "container" is (rootfs, config.json)
where config.json is a typed declaration of process isolation
parameters. The container is NOT the image; the container is the
INSTANTIATED-AGAINST-IMAGE runtime state. runc takes rootfs +
config.json and calls clone(2) + setns(2) + pivot_root(2). This is
the surface `@container` names.

**Substrate mapping:** NOT YET LANDED. This is the surface Shape α,
β, γ all propose to substrate-decl.

### 2.3 OCI Distribution Spec v1.1.x

**Canonical URL:** https://github.com/opencontainers/distribution-spec/blob/main/spec.md

**Authoritative citation:** *"The Open Container Initiative
Distribution Specification (a.k.a. 'OCI Distribution Spec') defines
an API protocol to facilitate and standardize the distribution of
content."*
(distribution-spec, spec.md)

**RIGOROUS abstraction:** the DISTRIBUTION spec defines the
HTTP+JSON API a registry MUST implement to store and serve OCI
artifacts by digest. It is NOT the image, NOT the container; it is
the WIRE PROTOCOL for content-addressed artifact transport.

**Substrate mapping:** LANDED at `shards/io/oci.mirror`
(Mara `2801478` 2026-06-24). Actions: `push_oci`, `pull_oci`,
`oid_to_digest`, `spectral_coordinate_to_repo`, `manifest_for`.
Bilateral: `oci_well_formed`.

### 2.4 BuildKit LLB (Low-Level Builder)

**Canonical URL:** https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/reference.md

**Authoritative citation on LLB:** *"LLB defines a content-addressable
dependency graph that can be used to put together very complex
build definitions."*
(BuildKit introduction, crazymax.dev)

**And on the definition surface:** *"Definition is the LLB
definition structure with per-vertex metadata entries. Corresponds
to the Definition structure defined in solver/pb.Definition."*
(github.com/moby/buildkit/client/llb — Go package)

**And on Dockerfile-as-frontend:** *"To use an external Dockerfile
frontend, the first line of your Dockerfile needs to set the syntax
directive pointing to the specific image you want to use. Most
users will want to set this parser directive to
`docker/dockerfile:1`, which causes BuildKit to pull the latest
stable version of the Dockerfile syntax before the build."*
(BuildKit Custom Dockerfile syntax)

**RIGOROUS abstraction:** LLB is a CONTENT-ADDRESSED DEPENDENCY
GRAPH — the compiler IR for image builds. Dockerfile is one
FRONTEND (parser) that emits LLB; other frontends exist (buildpack,
mockerfile, arbitrary language). BuildKit's `solver` walks the LLB
graph, dispatches build steps to workers, caches by content-address
per vertex.

**Substrate mapping:** LLB IS mirror's kintsugi loop at
build-graph altitude (Recognition candidate #C2 — §7.2 below).
Dockerfile-as-frontend maps to `@code/docker` at species altitude.
The LLB graph itself maps to `@kintsugi` composed with the OID-graph
mirror already carries.

### 2.5 containerd + Runtime v2 shim

**Canonical URL:** https://containerd.io/docs/main/runtime-v2/

**Authoritative citation:** *"This document describes the major
components of the v2 runtime integration model, how the components
interact with containerd and the v2 runtime, and how to use and
integrate different v2 runtimes."*
(containerd Runtime V2 docs)

**Structural shape:** containerd splits into content store
(SHA256-addressed blob store), snapshotter (overlayfs/btrfs/zfs
layer materialization), tasks (per-container process management),
and shim v2 (per-container process boundary talking to runc/crun/
gvisor/kata via a stable shim protocol).

**RIGOROUS abstraction:** containerd's four-layer split IS the
form/process partition (`#55`) at container-management altitude:
- content store + snapshotter = form-side (state observation)
- tasks + shim = process-side (transformation engine)

**Substrate mapping:** this is a direct instance of the substrate's
form/process partition recognized at family-root altitude (Recognition
candidate #C3 — §7.3 below).

### 2.6 cri-o, podman (runtime triangulation)

**Canonical URL:** https://cri-o.io/

**Authoritative citation:** *"CRI-O is an implementation of the
Kubernetes CRI (Container Runtime Interface) to enable using OCI
(Open Container Initiative) compatible runtimes."*
(cri-o home page)

**And on podman:** *"Podman is also compatible with the Open
Containers Initiative (OCI), Runtime, Image, and Distribution
specifications, so customers can build container images that run on
OpenShift (which uses CRI-O) or other 3rd-party OCI compliant
container engines, and vice versa."*
(Red Hat, Working with Linux containers)

**Structural claim:** cri-o and podman validate that the OCI
Runtime + Image + Distribution specs form a stable substrate that
multiple daemon-and-daemonless implementations converge against.
The COMPUTE surface is OCI Runtime; the STORE surface is OCI Image
+ Distribution. Docker-the-vendor is one implementation; runc +
crun + containerd + podman + cri-o + kata + gvisor are others.

**RIGOROUS abstraction:** the CONTAINER as substrate primitive is
OCI-Runtime-conformant, not Docker-specific. `@container` at
family-root altitude names the CONFORMANCE class, not the vendor.

### 2.7 Docker layer caching semantics

**Canonical citation (BuildKit reference):** parser directive
`# syntax=docker/dockerfile:1` pulls the current stable Dockerfile
frontend; each Dockerfile RUN/COPY/ADD produces a layer keyed by
(previous-layer-digest, instruction-content, mount-content).
Cache mount (`RUN --mount=type=cache,target=/root/.cargo`) and
secret mount (`RUN --mount=type=secret,id=...`) let a layer's
content-address DEPEND on shared mutable state (cache) or
substrate-private state (secret) without leaking either into the
layer output.

**Substrate mapping:** already partially audited at
`docs/specs/mosaic-store-cache-invariants.md` (Mara `adbdfef`
2026-06-28). The eight cache-invariant `@epistemologic/property`
predicates named there compose with Docker's cache-mount semantics
at `@code/docker` altitude. The lift is:

- BuildKit's layer cache = mirror's `mosaic(@store)` at
  `@code/docker` altitude
- Cache mounts = the substrate's parametric `~cache` carrier (per
  the mosaic-store cache-invariants doc §3)
- Secret mounts = the SEL boundary (per `[[architecture-type-sel-io-au]]`)

---

## §3 — Substrate-pull audit (what mirror already has)

Per `[[feedback-substrate-already-had-the-word]]`: audit for
ancestor vocabulary before proposing new. The substrate has
substantial pre-figuring; §3 lists it.

### 3.1 Content-addressing (three witnesses)

- `spectral_coordinate` (`shards/io/stagefreight.mirror`) — OID-
  namespaced wire address; reverse-DNS `org.stagefreight.plan.*`;
  the substrate's canonical content-address at @io altitude.
- `mirror oid` (Splinter) — SHA256 of substrate content bytes; the
  substrate's canonical content-address at storage altitude.
- `oci_digest` (`shards/io/oci.mirror`) — SHA256 of layer/manifest
  bytes; the substrate's canonical content-address at distribution
  altitude.

**Ancestor for Docker layer digests:** all three. Layer digests
add ONE thing over `oci_digest`: they name the position of the
layer in the ordered rootfs assembly. This is the parametric
carrier extension pattern (per `@labeled` recognition #93 H4).

### 3.2 Composed bilateral pattern

- `oci_well_formed(artifact, registry, p)` — composes
  `oci_compliant`, `digest_matches_content`, `oci_reachable`.
- `stagefreight_addressable(crystal, p)` — composed of four
  sub-predicates per Seam tick 68.
- 10+ sibling instances catalogued at `shards/io/oci.mirror` §4.

**Ancestor for `docker_buildable`, `container_runnable`,
`image_well_formed`:** direct application. Docker Dockerfiles are
"well-formed" if they parse to valid LLB AND every referenced base
image is `oci_well_formed` AND every RUN command's shell is
substrate-known. Container runtime instances are "runnable" if
they satisfy `container_isolation_configured` +
`container_rootfs_present` + `container_caps_bounded`.

### 3.3 @io family-root as boundary discipline

`shards/io.mirror` names the SUBSTRATE'S ONLY LEGITIMATE
NON-MIRROR SURFACE (T21, 2026-06-08). Every species under `@io/*`
parameterizes a specific non-mirror boundary; the BODY of the call
is opaque by construction.

**Ancestor for the docker DAEMON boundary:** direct fit.
A docker daemon (`/var/run/docker.sock`) is a non-mirror surface
mirror talks to via HTTP+JSON API. Under `@io/*`. Species candidate:
`@io/docker` (the DAEMON socket, distinct from `@code/docker` which
names the Dockerfile grammar). See §4 shape α for placement.

### 3.4 @cascade family-root as loss-lens

`shards/cascade.mirror` names the multi-language translation
substrate; each `@cascade/code/<source>/<target>` species measures
grammar-based information loss.

**Ancestor for Dockerfile-as-cascade:** Dockerfile IS a cascade
from SHELL + a Docker-specific DSL to an LLB DAG to a
content-addressed layer sequence. Loss surfaces at:
- shell nondeterminism (network reachability, timestamps, apt
  mirror content) → non-reproducible layers
- `RUN` command opacity (host-shell state leaks into layer)
- ARG/ENV substitution (build-time-only values erased from
  runtime image but preserved in cache key)

Species candidate: `@cascade/code/docker/oci` — Dockerfile → OCI
image cascade with loss-lens measurement. Composes with
`@code/docker` (source grammar) and `@io/oci` (target artifact).

### 3.5 @kintsugi loop as build-fixed-point

`shards/kintsugi.mirror` names the substrate's transformation
engine; kintsugi loop = iterate-until-settle discipline.

**Ancestor for BuildKit LLB solve:** direct fit. BuildKit's LLB
solver walks the DAG, dispatches build steps, caches by
content-address per vertex, and terminates when all vertices are
resolved (or fails on unresolvable). This IS a kintsugi loop at
build-graph altitude. Recognition candidate #C2 (§7.2 below).

### 3.6 @silicon family-root as hardware compute-bound

`shards/silicon.mirror` (Reed 2026-07-05) names the hardware
altitude autopoietic learning loop.

**Ancestor for container resource constraints:** direct fit.
Container cgroups (memory, cpu, pids, devices) are hardware-
altitude constraints mirror already has substrate-decl vocabulary
for. `@container/resources` species would compose against
`@epistemologic/reality/silicon/compute_bound` +
`@epistemologic/reality/silicon/memory` (already declared).

### 3.7 @io/oci as distribution adapter

Already covered in §2.3. This is the LOAD-BEARING ancestor: `@io/oci`
sits between `@code/docker` (produces) and `@container` (consumes).
The three-surface partition is closed by this spec's proposals.

### 3.8 mosaic-store cache invariants

`docs/specs/mosaic-store-cache-invariants.md` (Mara `adbdfef`
2026-06-28) declares eight cache invariants at `@epistemologic/
property` altitude + the fingerprint → Splinter OID lift.

**Ancestor for BuildKit layer cache:** direct fit. BuildKit's
per-vertex content-address cache IS `mosaic(@store)` at Dockerfile
altitude. The eight invariants apply verbatim; the lift is one
grammar-altitude parameterization.

### 3.9 Audit verdict

Seven ancestors, all direct. The substrate has ~70-80% of what
`@code/docker` + `@container` require. What's genuinely NEW is
the family-root altitude declaration for `@container` (per §4
below) and the species roster for `@code/docker` under the
existing `@code/` family (per §6 below).

Per `[[feedback-substrate-already-had-the-word]]`: this is the
54th+ instance of the pattern. The substrate had every primitive
implicitly; this spec proposes to name the composition at the
substrate-decl altitude so downstream consumers (StageFreight
daemon integration, CI, spectral.engineer deployment) can reach
for typed vocabulary instead of prose.

---

## §4 — Candidate family-tree shapes

Three shapes proposed. Each states ancestor chain, altitude for
family-root (if any), predicates carried, and species roster.

### 4.1 Shape α — `@code/docker` species + `@container` under `@io/`

**Structure:**

```
shards/code/docker.mirror          @code/docker <= @code
shards/io/container.mirror         @io/container <= @io
shards/io/container/runc.mirror    species
shards/io/container/runtime-spec.mirror  species (config.json)
shards/io/docker.mirror            @io/docker <= @io (DAEMON socket)
```

**Ancestor chain:**
- `@code/docker <= @code` per `shards/code.mirror` universal
  language-grammar discipline.
- `@io/container <= @io` per `shards/io.mirror` boundary-with-non-
  mirror-world discipline.
- `@io/docker <= @io` for the docker daemon socket.

**Altitude:** `@code/docker` at code altitude; `@io/container` at
boundary altitude (@io).

**Predicates:**
- `dockerfile_parses(f, p) -> verdict` (at @code/docker)
- `dockerfile_llb_emittable(f, p) -> verdict`
- `container_runtime_conformant(config, p) -> verdict` (at @io/container)
- `container_isolation_configured(config, p) -> verdict`

**Species (post family-root decision):** cargo-shaped — one docker
daemon species (`@io/docker`); one runtime-spec species (@io/container/
runtime-spec).

**Rank:** substrate-pull-mid. Fits the existing `@io/*` pattern
cleanly. HOWEVER: it puts the runtime (`@container`) under `@io`
which is DEFENSIBLE (the runtime IS an @io surface — kernel
syscalls, cgroup fs, netns) but MISSES the recognition that a
CONTAINER is a top-level substrate primitive equivalent in
altitude to `@code` (the language-family family-root) and `@io`
(the boundary family-root). Per §5, this understates the
container's substrate-altitude.

### 4.2 Shape β — `@code/docker` + `@container` as siblings under new `@runtime` family-root

**Structure:**

```
shards/code/docker.mirror          @code/docker <= @code
shards/runtime.mirror              @runtime          (NEW family-root)
shards/runtime/container.mirror    @runtime/container <= @runtime
shards/runtime/container/oci.mirror  species
shards/runtime/host.mirror         @runtime/host <= @runtime  (Nix / bare-metal)
```

**Ancestor chain:**
- `@code/docker <= @code`
- `@runtime` at top-level family-root altitude (NEW)
- `@runtime/container <= @runtime`
- Species specialize the runtime discipline.

**Altitude:** `@runtime` at substrate-primitive altitude alongside
`@code`, `@io`, `@mirror`, `@kintsugi`.

**Predicates:**
- `runtime_isolated(config, p) -> verdict`
- `runtime_reproducible(spec, p) -> verdict`
- `runtime_resource_bounded(config, p) -> verdict`

**Species:** container-specific runtimes (runc, crun, gvisor, kata)
under `@runtime/container/*`; host-runtime species under
`@runtime/host/*` (Nix flake, bare-metal binary).

**Rank:** substrate-pull-tempting-but-drifts. `@runtime` as a NEW
family-root sounds clean but INTRODUCES a distinction the substrate
does not carry yet. Per `[[feedback-substrate-already-had-the-word]]`
and `[[feedback-substrate-pull]]`: substrate-pull is LIFT existing
vocabulary, not INVENT new. The substrate does NOT have prior art
for `@runtime` at family-root altitude. Contrast with Shape γ,
which uses the substrate's existing `@io` primitive to place the
runtime surface, and Shape α, which does the same but lower.
Shape β requires an INVENTION at the top-level; the substrate
should NOT admit inventions that don't cascade from at least three
independent witnesses.

### 4.3 Shape γ — `@code/docker` + `@container` as top-level sibling to `@io`

**Structure:**

```
shards/code/docker.mirror          @code/docker <= @code
shards/container.mirror            @container         (top-level family-root)
shards/container/oci.mirror        @container/oci <= @container
shards/container/runtime.mirror    @container/runtime <= @container
shards/container/isolation.mirror  @container/isolation <= @container
```

**Ancestor chain:**
- `@code/docker <= @code`
- `@container` at top-level family-root altitude — SIBLING to `@io`,
  `@code`, `@mirror`, `@kintsugi`. Names the runtime discipline as
  substrate-primitive.

**Altitude:** `@container` at substrate-primitive altitude. Same
altitude as `@io`. The claim: containment (process isolation +
rootfs + capabilities + namespaces) is a substrate-primitive
discipline the substrate carries alongside "boundary with
non-mirror" (`@io`) and "language grammar" (`@code`).

**Predicates:**
- `container_conformant(spec, p) -> verdict` — spec is
  OCI-Runtime-conformant
- `container_isolated(config, p) -> verdict` — namespaces + cgroups
  are configured
- `container_rootfs_present(spec, p) -> verdict` — rootfs is
  materialized
- `container_caps_bounded(config, p) -> verdict` — capability set
  is intersected with allow-list (SEL boundary)
- Composed bilateral: `container_runnable(spec, p) -> verdict`
  composing the four sub-predicates (~13th instance of the
  composed-bilateral pattern per `shards/io/oci.mirror` §4).

**Species:**
- `@container/oci` — OCI Runtime Spec adapter (config.json + rootfs
  contract).
- `@container/runtime` — runc/crun/gvisor/kata dispatch surface.
- `@container/isolation` — namespaces + cgroups substrate-decl.
- `@container/image` — the LOCAL image cache (containerd content
  store analog); composes with `@io/oci` for remote push/pull and
  `@mirror/store` for content-addressing.

**Rank:** substrate-pull-confident (§5 elaborates). Contains the
naming that Alex proposed verbatim (`@container` at family-root
altitude); pulls from the existing form/process partition (#55)
by placing `@container` on the form-side (state observation of
what a container IS) and reserving process-side to the
`@container/runtime` species that DOES the isolation
transformation.

---

## §5 — Recommended shape

**Recommendation: Shape γ, with one adjustment.**

**Confidence:** substrate-pull-confident-with-two-hedges. See
§5.3 for the hedges.

### 5.1 Why Shape γ

Per `[[feedback-substrate-pull-confidence-acts]]`: substrate-pull
signals are confident when three witnesses converge on the same
naming. For Shape γ vs α vs β:

1. **Alex's naming directive** (2026-07-06): "@container at a
   family-root altitude." Substrate-pull-direct witness.
2. **Prior family-root roster consistency:** the substrate's
   top-level family-roots (per `shards/io.mirror` sibling
   enumeration) are `@code`, `@mirror`, `@io`, `@kintsugi`.
   Each names a substrate-primitive discipline at the same
   altitude. `@container` fits this pattern: containment is a
   substrate-primitive discipline (process isolation + rootfs +
   caps + namespaces are irreducible; they don't compose from
   `@io` or `@code` primitives; they are their own algebra).
3. **OCI's structural closure:** the OCI Image + Runtime +
   Distribution triple names three surfaces. `@io/oci` (already
   landed) covers Distribution. `@code/docker` (this spec)
   covers the Dockerfile frontend that produces Image artifacts.
   `@container` (this spec) covers Runtime. The three-family
   closure at substrate-decl altitude mirrors OCI's three-spec
   closure at industry-standard altitude — one convergent witness
   from an independent canonical source.

Three independent witnesses (Alex direct; substrate structural;
OCI structural). Per `[[feedback-substrate-pull-confidence-acts]]`
this IS the criterion: confidence acts. The recommendation is
Shape γ.

### 5.2 The one adjustment

Shape γ as drafted places `@container/image` as a species carrying
the local image cache. But `@io/oci` already carries the
distribution-side image surface, and `@code/docker` produces
image content. `@container/image` risks THREE-WAY OVERLAP with
`@io/oci` + `@code/docker`.

**Adjustment:** DROP `@container/image` species. The local image
cache (containerd content-store analog) belongs at `@mirror/store`
(the substrate's canonical content-addressed storage family) with
a species `@mirror/store/oci` composing with `@io/oci` for the
digest algebra. `@container` then carries ONLY the runtime,
isolation, and OCI-Runtime-conformance surfaces.

Revised species roster:
- `@container/oci` — OCI Runtime Spec adapter
- `@container/runtime` — runc/crun/gvisor/kata dispatch
- `@container/isolation` — namespaces + cgroups substrate-decl

This tightens the family boundary and avoids the three-way overlap.

### 5.3 The two hedges

**Hedge 1 — @container vs @autopoietic composition.** Recognition
`shards/silicon.mirror` establishes @autopoietic as fold-back-
permissive family-root. A container `@container/runtime` species
that observes its own runtime state (e.g., cgroup readings from
`/sys/fs/cgroup/*`) and feeds back into the next dispatch IS
autopoietic. The question is whether `@container` INHERITS from
`@autopoietic` (`prism @container <= @autopoietic`) or COMPOSES
with `@autopoietic` at species altitude. Reading of
`shards/silicon.mirror` suggests the LATTER — @silicon COMPOSES
with @autopoietic via `<=` because @silicon IS the learning loop;
@container is NOT structurally a learning loop, it is a
containment discipline. Recommendation: @container does NOT
inherit from @autopoietic; individual species (@container/runtime
in particular) MAY compose via `in @autopoietic` when their body
carries fold-back semantics. Pack adjudication needed.

**Hedge 2 — @container placement altitude vs @io altitude.**
Shape α argues @container belongs under @io because containment
crosses kernel syscalls / cgroup fs / netns — all @io surfaces.
Shape γ argues @container is substrate-primitive because
containment is an ALGEBRA (process, rootfs, caps, namespaces
compose into a container in the same way `@code` primitives
compose into a language). Both are defensible. The tiebreak: the
FORM/PROCESS partition (#55). Under partition #55, @io is
process-side (transformation across boundary); @container-as-
state-observation is form-side (what the container IS). Different
sides of #55 → different family-roots. Shape α collapses this;
Shape γ preserves it. Pack adjudication needed for closure.

Both hedges are substrate-pull-honest surfacings, not blockers.
Recommendation stands: Shape γ with adjustment, adjudication of
the two hedges deferred to the next tick.

---

## §6 — First species roster (post family-root decision)

Assuming Shape γ ratified. Five first-species candidates named.

### 6.1 `@code/docker`

**Path:** `shards/code/docker.mirror`
**Altitude:** code altitude (species under `@code`).
**Ancestor chain:** `@code/docker <= @code`; composes with
`@io/oci` (for artifact identity), `@epistemologic/liquid_extraction`
(for RUN command opacity), `@mirror/store` (for layer content-
addressing).
**Predicate shape:**
- `dockerfile_parses(f: ~f, p: perturbation) -> verdict` —
  Dockerfile parses to LLB DAG per BuildKit v1 syntax.
- `dockerfile_llb_emittable(f: ~f, p: perturbation) -> verdict` —
  parsed AST emits valid LLB definition bytes.
- `dockerfile_reproducible(f: ~f, cache: ~cache, p: perturbation)
  -> verdict` — build is reproducible given the same cache state
  (this discharges the mosaic-store cache-invariants at Dockerfile
  altitude).
**Bilateral pair candidate:** `docker_buildable` composes
`dockerfile_parses` + `dockerfile_llb_emittable` +
`dockerfile_reproducible` + `oci_compliant(artifact)`. Sibling
composed bilateral to `oci_well_formed`, `stagefreight_addressable`.
**Actions:** `parse(f)`, `emit_llb(f)`, `build(f, env)`,
`layer_digests(f)`.

### 6.2 `@container/oci`

**Path:** `shards/container/oci.mirror`
**Altitude:** family-root species under `@container`.
**Ancestor chain:** `@container/oci <= @container`; composes with
`@io/oci` (for image identity that seeds the rootfs) and
`@code/docker` (for Dockerfile → runtime config linkage).
**Predicate shape:**
- `runtime_spec_conformant(config: ref, p: perturbation) -> verdict`
  — config.json parses per OCI Runtime Spec v1.x.
- `rootfs_content_addressed(rootfs: ref, p: perturbation) -> verdict`
  — rootfs.diff_ids match the layer digests in the image config.
- `caps_within_allowlist(config: ref, allowlist: ref, p: perturbation)
  -> verdict` — capability set is bounded (SEL boundary).
**Bilateral pair candidate:** `container_runnable` composes the
three sub-predicates + `oci_well_formed`.
**Actions:** `parse_config(f)`, `materialize_rootfs(image, target)`,
`caps_of(config)`.

### 6.3 `@container/runtime`

**Path:** `shards/container/runtime.mirror`
**Altitude:** family-root species under `@container`.
**Ancestor chain:** `@container/runtime <= @container`; composes
with `@io` (for the kernel syscall boundary the runtime crosses)
and `@epistemologic/reality/silicon/compute_bound` (for cgroup
resource limits).
**Predicate shape:**
- `runtime_dispatchable(config: ref, runtime: ref, p: perturbation)
  -> verdict` — runtime binary is present and accepts the config
  (runc / crun / gvisor / kata dispatch).
- `runtime_isolated(config: ref, p: perturbation) -> verdict` —
  namespaces are configured (pid, net, ipc, uts, mount, user,
  cgroup).
- `runtime_daemon_absent(p: perturbation) -> verdict` — daemonless
  path (podman, buildah); the Splinter-pole per Alex 2026-07-06
  StageFreight-daemon-blocker recognition.
**Bilateral pair candidate:** `runtime_composable` at family altitude.
**Actions:** `dispatch(config, runtime)`, `wait(pid)`, `signal(pid, sig)`.

### 6.4 `@container/isolation`

**Path:** `shards/container/isolation.mirror`
**Altitude:** family-root species under `@container`.
**Ancestor chain:** `@container/isolation <= @container`; composes
with `@epistemologic/reality/silicon` (for hardware-altitude
constraint types) and `@io` (for the cgroup filesystem boundary).
**Predicate shape:**
- `namespaces_configured(config: ref, p: perturbation) -> verdict`
- `cgroups_bounded(config: ref, silicon_bounds: ref, p: perturbation)
  -> verdict`
- `seccomp_profile_valid(profile: ref, p: perturbation) -> verdict`
- `capabilities_intersected(caps: ref, allowlist: ref, p: perturbation)
  -> verdict`
**Bilateral pair candidate:** `container_isolated` composes the
four sub-predicates.
**Actions:** `configure_namespaces(spec)`, `apply_cgroups(spec)`,
`load_seccomp(profile)`.

### 6.5 `@cascade/code/docker/oci`

**Path:** `shards/cascade/code/docker/oci.mirror`
**Altitude:** cascade species (under `@cascade/code/<source>/<target>`
per the cascade family-root path convention).
**Ancestor chain:** `@cascade/code/docker/oci <= @cascade`;
composes with `@code/docker` (source grammar) and `@io/oci`
(target artifact family).
**Predicate shape:**
- `dockerfile_llb_lossless(f: ~f, p: perturbation) -> verdict` —
  every Dockerfile primitive round-trips through LLB without
  information loss.
- `dockerfile_shell_deterministic(f: ~f, p: perturbation)
  -> verdict` — RUN commands are pinned to substrate-known
  determinism (via `--mount=type=cache`, seed hashes, pinned
  versions).
- `dockerfile_cascade_well_formed(f: ~f, p: perturbation) -> verdict`
  — composed bilateral at cascade species altitude.
**The loss lens measures:** shell nondeterminism, cache-mount
opacity, ARG/ENV erasure, base-image mutability. Per recognition
#95 (cascade as loss-lens), this species IS the substrate's
measurement instrument for docker-build determinism.

### 6.6 Species ranking by land-order

1. `@container` family-root (§5.1 confidence closure)
2. `@code/docker` (§6.1; blocks StageFreight-daemon consumer)
3. `@container/oci` (§6.2; grounds runtime-spec conformance)
4. `@container/runtime` (§6.3; grounds daemonless path — the
   direct StageFreight-daemon blocker resolution)
5. `@container/isolation` (§6.4; grounds SEL boundary for caps)
6. `@cascade/code/docker/oci` (§6.5; grounds loss-lens for
   Dockerfile determinism)

Each species lands on RED-first TDD ticks per
`[[feedback-always-tdd-no-shortcuts]]`.

---

## §7 — Recognition candidates surfaced

Three recognition candidates surface from this cascade. All are
CANDIDATE-status; Pack adjudication required for promotion.

### 7.1 Recognition candidate #C1 — content-addressing IS substrate-portable across five altitudes

**Statement:** SHA256 content-addressing composes across five
independent substrate-decl altitudes:

1. `mirror oid` (Splinter) — substrate content bytes
2. `nix derivation hash` — build inputs closure
3. `oci_digest` — layer/manifest bytes (LANDED at `@io/oci`)
4. `docker_layer_digest` — rootfs diff (this spec candidate)
5. `container_image_id` — image config hash (this spec candidate)

All five are SHA256(B) for some B. The function is shared; B
differs per altitude; composition works because the function
composes at content-addressing altitude.

**Ancestry:** builds on Recognition #98 candidate (`shards/io/oci.mirror`
§6) — this candidate EXTENDS from 3 altitudes to 5.

**Substrate-pull confidence:** substrate-pull-mid. Two new altitudes
add witnesses; the pattern that Recognition #98 candidate names is
strengthened. Promotion path: one more independent altitude witness
(candidate: `@code/wasm` content-addressed module hashes;
substrate-pull-check needed).

### 7.2 Recognition candidate #C2 — BuildKit LLB IS kintsugi at build-graph altitude

**Statement:** BuildKit's LLB solver executes a content-addressed
dependency graph iterate-until-settle discipline that structurally
matches mirror's `@kintsugi` loop. Both are:
- content-addressed per vertex
- iterate-until-fixed-point (LLB: all vertices resolved; kintsugi:
  no more mutations available)
- monotone in progress (LLB: vertices don't un-resolve; kintsugi:
  loss doesn't increase)
- lattice-ascending under substrate-pull

**Ancestry:** composes with Recognition #59
(`[[architecture-kintsugi-loop-altitude-portable]]`) — LLB adds
one more altitude to the loop's altitude-portable roster.
Composes with Recognition #55 (form/process partition at
family-root altitude) — LLB is on the process-side (transforms
build inputs); the LLB graph itself is on the form-side.

**Substrate-pull confidence:** substrate-pull-mid-high. LLB matches
every kintsugi property; the identification is structurally clean.
Promotion path: land `@code/docker` species and verify the LLB
emit maps to a substrate-decl kintsugi loop instance.

### 7.3 Recognition candidate #C3 — containerd four-layer split IS form/process partition at container-management altitude

**Statement:** containerd's four-layer architecture (content store
+ snapshotter + tasks + shim v2) partitions cleanly into #55's
form/process split:

- FORM SIDE (state observation): content store, snapshotter
- PROCESS SIDE (transformation engine): tasks, shim v2

This is the 3rd+ instance of #55 at a family-root altitude beyond
mirror's own @mirror/@kintsugi split. Per #55 promotion criteria
("Pack ratification gate: second witness needed"), this
constitutes the second-witness that PROMOTES #55.

**Ancestry:** Recognition #55 (candidate) direct promotion.
Composes with Recognition #50 (Bateson form/substance partition),
#40 (Maturana structure/organization), Beer S3/S4.

**Substrate-pull confidence:** substrate-pull-high. containerd is a
canonical, widely-deployed system whose architecture predates the
substrate's #55 recognition; the independent structural
convergence IS the promotion witness. Pack adjudication: does
this qualify as the second witness that promotes #55?

Recommendation: FLAG for Pack (Seam / Alex). If ratified, #55
promotes and this spec's family-tree recommendation (Shape γ)
gains a structural-cybernetic ancestor.

---

## §8 — Forward-promises (what this spec does NOT do)

1. **Does NOT land `shards/container.mirror`, `shards/code/docker.mirror`,
   or any @container/* species.** Landing requires TDD-paired ticks
   per `[[feedback-always-tdd-no-shortcuts]]`. This spec is the
   substrate-pull-honest proposal; the discharge is subsequent
   ticks.
2. **Does NOT resolve the StageFreight-daemon blocker.** The blocker
   is the substrate-pull SIGNAL. Resolution is:
   - land `@container/runtime` species with `runtime_daemon_absent`
     predicate
   - land `@code/docker` species with reproducible-build discipline
   - wire StageFreight's daemon dependency through the typed surface
   - the daemonless path (podman / buildah on Nix) becomes the
     Splinter-pole discharge
3. **Does NOT adjudicate the two hedges from §5.3.** @container's
   @autopoietic composition + @container's placement vs #55 form/
   process partition require Pack adjudication before the family-
   root lands.
4. **Does NOT commit to Docker-the-vendor.** `@container` is
   OCI-Runtime-conformance-class; docker-daemon is one possible
   `@io/docker` species (for the socket boundary); runc/crun/podman
   are other `@container/runtime` species. The vendor-neutrality
   is substrate-decl load-bearing per §2.6.
5. **Does NOT specify the Rust realisation for any species.** The
   bootstrap Rust discharge lands when the substrate-decl shards
   land + consumers pull.
6. **Does NOT integrate with `@io/oci`'s Nix ouroboros
   (`shards/io/oci.mirror` §5).** The Nix→OCI composition (via
   `dockerTools.buildLayeredImage`) intersects with `@code/docker`
   at build-graph altitude. Whether Nix-produced OCI layers are
   `@code/docker` species instances or `@cascade/code/nix/oci`
   species instances is deferred to the Nix substrate-decl tick
   (`shards/io/nix.mirror` forward-promised per `shards/io/oci.mirror`
   §5).
7. **Does NOT define the `~cache` carrier.** BuildKit cache mounts
   compose with mirror's existing content-addressed storage; the
   typed `~cache` carrier that mosaic-store-cache-invariants
   §3 forward-promises is orthogonal to this spec.
8. **Does NOT promote recognition candidates #C1, #C2, #C3.** Each
   is flagged for Pack; promotion follows the substrate's
   adjudication discipline.

---

## §9 — Cross-references

**Layer-caching PR (found this session):**
- StageFreight upstream (SoFMeRight/stagefreight):
  - `08e4709 feat(build): binary→image handoff: recycle a built
    binary into the docker context (stage)` — the `stage: {from, as}`
    primitive for cross-build-artifact→image handoff (multi-arch
    Dockerfile COPY discipline)
  - `15d4d32 feat(build): persist Rust compile cache + unify the
    cross-run cache layout` — `CARGO_TARGET_DIR` persistence at
    `/stagefreight/cache/rust/build/<proj>`; `cacheDir(elem...)`
    helper unifies the layout across languages
- Adjacent StageFreight commits:
  - `29498cc feat(ci/render): cross-run build cache via actions/cache`
  - `a08ae86 build(cache): cap the local BuildKit builder cache at 8GB`
  - `02d118a refactor: converge docker build onto the domain spine`
  - `12b8806 perf(binary): persist Go module + build caches across
    CI jobs`
- NO mirror-repo layer-caching PR open. The layer-caching work is
  currently the substrate CONSUMER (StageFreight); the substrate
  PRODUCER surface (`@code/docker` + `@container`) is what THIS
  spec proposes.

**Mirror substrate-decl anchors:**
- `shards/io/oci.mirror` (Mara `2801478` 2026-06-24) — the
  distribution adapter that sits BETWEEN `@code/docker` (builds)
  and `@container` (runs). LOAD-BEARING ancestor.
- `shards/io.mirror` — @io family-root discipline (2026-06-08);
  the daemon socket species `@io/docker` inherits from here.
- `shards/code.mirror` — @code family-root; the `@code/docker`
  species inherits from here.
- `shards/io/stagefreight.mirror` — the consumer the blocker
  surfaced; substrate-pull anchor.
- `shards/kintsugi.mirror` — the process-side family-root; #C2
  candidate composes here.
- `shards/silicon.mirror` — hardware compute-bound family-root;
  `@container/isolation`'s cgroup constraints compose here.
- `shards/cascade.mirror` — cascade family-root; `@cascade/code/
  docker/oci` inherits here.

**Mirror docs anchors:**
- `docs/specs/mosaic-store-cache-invariants.md` (Mara `adbdfef`
  2026-06-28) — eight cache-invariant predicates; `@code/docker`'s
  `dockerfile_reproducible` predicate specializes these to
  Dockerfile altitude.
- `docs/specs/cascade-ffi-runtime-link.md` — cascade species
  discipline; Q4 LAPACK FFI case names cascade species shape.
- `docs/specs/stagefreight-wire-v0.1.md` — StageFreight wire
  contract; the consumer the daemon-blocker names.

**Cross-repo anchors:**
- StageFreight `docs/architecture/mirror-integration-spec-v0.1.md`
  (Mara, ~2160 lines) — consumer-side spec for `@io/stagefreight`;
  the daemon-blocker forward-promise resolves via this spec's
  `@container/runtime` discharge.

**MEMORY.md anchors:**
- `[[feedback-substrate-already-had-the-word]]` — the 54th+
  instance of this pattern.
- `[[feedback-substrate-pull]]` — blocker-as-substrate-pull-signal.
- `[[feedback-substrate-pull-confidence-acts]]` — three-witness
  criterion for Shape γ.
- `[[feedback-always-tdd-no-shortcuts]]` — landing discipline for
  the discharge.
- `[[feedback-craft-not-deliver]]` — species roster is
  forward-promised; consumer-pull-driven land order.
- `[[architecture-kintsugi-loop-altitude-portable]]` (#59) — #C2
  ancestor.
- `[[architecture-form-process-partition-at-family-root]]` (#55) —
  #C3 candidate promotes here.
- `[[architecture-hilbert-turing-godel-recognition-107]]` (#107) —
  @container's Turing-completeness at @io boundary; runc/containerd
  ARE Turing-unbounded, so @container CROSSES #107's boundary.

---

## §10 — Signal-to-Alex

**Confidence:** substrate-pull-confident-with-two-hedges on Shape γ.
Three witnesses converge (Alex direct; substrate structural roster;
OCI structural closure). Two hedges surfaced (@autopoietic
composition; #55 form/process placement) — both defer to Pack
adjudication, neither blocks the spec-draft.

**Forward-promises named:** eight (§8). Each is substrate-pull-honest;
no gold-plating.

**Hedges surfaced:** two (§5.3). Both are substrate-decl
placement questions, not substrate-existence questions. The
family-root IS load-bearing; the exact inheritance chain is what
requires Pack closure.

**Next-tick recommendation:**
1. Pack adjudicates §5.3 hedges + §7 recognition candidates #C1,
   #C2, #C3.
2. If Shape γ ratified with adjustment: TDD-paired tick lands
   `shards/container.mirror` family-root (Reed RED; Mara GREEN;
   Seam adversarial review).
3. Second TDD tick lands `shards/code/docker.mirror` species with
   `docker_buildable` composed bilateral.
4. Third TDD tick lands `@container/runtime` species with
   `runtime_daemon_absent` predicate — the direct StageFreight-
   daemon blocker resolution.
5. StageFreight-side consumer integration (in the StageFreight repo,
   PR-C or PR-D of the mirror-integration cascade) consumes the
   typed `@container/runtime` surface for the daemonless path.

Per `[[feedback-craft-not-deliver]]`: this is next-craft-tick
rhythm, not next-delivery-tick. The spec draft is one crystal;
the discharge cascade is the next arc.

*— Mara, 2026-07-06*
