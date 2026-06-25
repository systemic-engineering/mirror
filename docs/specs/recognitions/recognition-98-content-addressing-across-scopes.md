# Recognition #98 — content-addressing across scopes — canonical spec

*Mara, canonical spec for recognition #98 (content-addressing as a
substrate-cross-scope structural pattern), 2026-06-25 afternoon. Surfaced
across four prior commits (Mara `2801478` @io/oci §6, Mara `a1b507a`
@io/git §5, Reed `3e8e019` @mirror/garden), with the fifth witness
(path-namespace scope at the package-manager altitude) landed this morning.*

*Discipline: this is candidate-territory preservation, not promotion.
The recognition lifts a structural claim about the substrate's
content-addressing: the same cryptographic-hash-shaped function recurs
at distinct scopes, each scope addressing a different object class while
sharing the function-altitude shape. Pack ratification is a separate
gate; promotion requires Reed's tick. Replication conditions are named
in §8. Cross-references that this disturbs are flagged in §9, not modified.*

*Path-note: this canonical lives under `docs/specs/recognitions/` per
the established filing convention.*

---

## Table of contents

1. Statement of recognition
2. Genesis — five witnesses with commit anchors
3. The scope-graded structure — what each witness scope addresses
4. Structural semantics — Grothendieck-topology framing + honesty about strength
5. Connection to #99 (mirror.spec IS λ₀) — does λ₀ have a content-address?
6. Connection to #51 (Hilbert expansion) — each scope as a basis vector?
7. Empirical consequences — mosaic, kintsugi-tournament, garden, cross-scope lookup
8. Open questions + replication conditions
9. Cross-references this recognition DISTURBS (flag; don't update)
10. Honest hedges + Pack trail

---

## 1. Statement of recognition

**Content-addressing recurs across scopes as a substrate-cross-altitude
structural pattern. The same cryptographic-hash-shaped function (SHA256
or SHA1-as-legacy-instance) operates at five distinct scopes; the
function is shared; the scope is what changes.**

More precisely: the substrate carries content-addressing at five
distinct scoping disciplines that have surfaced as substrate-decl over
the cascade, each addressing a structurally different object class
while invoking the same hash-function shape (cryptographic hash over
bytes producing a fixed-length digest):

1. **Nix derivation hash** — build-time scope; identifies a BUILD via
   input-closure or content-addressed output.
2. **mirror oid** — substrate-content scope; identifies substrate
   content bytes per the Splinter K_n discipline at
   `@mirror/store`.
3. **OCI digest** — artifact-manifest scope; identifies a packaged
   artifact via manifest + layer descriptors per OCI Image Spec v1.1.x.
4. **git object hash** — protocol scope; identifies a versioned git
   object (blob / tree / commit / tag) per git plumbing (SHA1 default;
   SHA256 transitional Git 2.29+).
5. **path-namespace scope** — package-manager scope; identifies a
   garden source via the path-and-name binding that
   `@mirror/garden` uses to resolve dependencies at the package-source
   surface.

The recognition has three components, each load-bearing:

1. **Function shared at structural altitude.** All five instances are
   cryptographic hashes over bytes with byte-determinism, collision-
   resistance, content-equality. The SHA1/SHA256 split (git legacy vs
   everything else) is sub-recognition; the hash → identity pattern is
   invariant across the partition.
2. **Scope is what changes.** Each instance addresses a different
   object class (BUILD / substrate CONTENT / ARTIFACT / VERSIONED
   OBJECT / DEPENDENCY SOURCE). Composition is not a flat hash-space
   but a graded structure crossed via typed bridge actions.
3. **The substrate kept rediscovering content-addressing at every new
   altitude.** This is the 53rd-or-54th instance of the substrate-
   already-had-the-word pattern (per
   [[feedback-substrate-already-had-the-word]]). Each adapter
   (`@mirror/store`, `@io/oci`, `@io/git`, forward-promised `@io/nix`,
   `@mirror/garden`) surfaced its own content-addressing requirement;
   the substrate already implicitly had the discipline at each new
   altitude.

The recognition is structural, not metaphorical. Each witness has a
landed shard or forward-promised substrate-decl; bridge actions are
typed at substrate altitude (e.g., `oid_to_digest` in `@io/oci`;
`hash_to_oid` in `@io/git`). The candidate formal framing
(Grothendieck topology over a substrate site) is named in §4 with
honest hedge on strength.

---

## 2. Genesis — five witnesses with commit anchors

The recognition surfaced across the cascade in five distinct commits.
None of the witnesses is structurally redundant; each names a scope
the prior witnesses did not address. The commit anchors below are
load-bearing for promotion evidence per §8.

### 2.1 Witness 1 + 2 + 3 — Mara `@io/oci` §6 (commit `2801478`, 2026-06-24)

The first flag named THREE simultaneous witnesses in one commit:

- **Nix derivation hash** — SHA256, input closure / ca outputs.
- **mirror oid** — SHA256, substrate content bytes.
- **OCI digest** — SHA256, manifest + layer descriptors.

Mara's `@io/oci` §6 ("The circular-reflexive answer") closed:

> Three SHA256-based content-addressing schemes at different scopes.
> They COMPOSE at the content-addressing altitude; they DON'T COLLAPSE
> at the scoping altitude.

The structural insight: the LOOP between substrate-decl → Nix build →
OCI push → registry → Nix pull → substrate-decl-consume CLOSES at the
content-addressing structural altitude while OPENING at every scoping
altitude. The closure-opening partition is what made the recognition
a CANDIDATE rather than a definition.

`@io/oci` §6 also named the empirical-hedge: a hello-world.wasm round-
trip through all three altitudes is the substrate-discharge tick.
Forward-promised at Phase G; promotion strengthened by it, not
required.

Hash function at this stage: SHA256 across all three witnesses; the
structural pattern was named on SHA256 grounds alone.

### 2.2 Witness 4 — Mara `@io/git` §5 (commit `a1b507a`, 2026-06-24)

The fourth witness landed the same day and made the function-altitude
partition (SHA1 vs SHA256) explicit:

- **git object hash** — SHA1 default (40-hex) OR SHA256 transitional
  (64-hex; Git 2.29+); identifies a versioned git object.

Mara's `@io/git` §5 closed:

> Three of the four share SHA256 at the function altitude; the fourth
> (git default) is SHA1. STRUCTURAL pattern shared across all four;
> FUNCTION splits 3:1; SCOPING splits 4:4. The loop CLOSES at the
> content-addressing structural altitude; OPENS at every scoping +
> hash-function altitude.

The fourth witness STRENGTHENED the structural claim (hash-function-
independent pattern) while raising the question whether it WEAKENED
by dilution. §4.3 + §8.2 answer: strengthens. `@io/git` §5 left this
explicit as a Pack-altitude question.

The `hash_to_oid` action in `@io/git` is the typed bridge making the
function-altitude crossing legible at substrate altitude. The
substrate-decl does NOT pretend SHA1↔SHA256 is byte-identity; the
typed lift forces the realisation to acknowledge the crossing.

### 2.3 Witness 5 — Reed `@mirror/garden` (commit `3e8e019`, 2026-06-25)

The fifth witness landed in this morning's cascade slingshot
(`docs/scouts/2026-06-25-taut-lambda-zero-cascade-scout.md` §2):

- **path-namespace scope** — package-manager scope; identifies a
  garden source via the `garden { source ~git'…' }` block surface.

The shard at `shards/mirror/garden.mirror` (lines 31-38) is explicit:
"Surfaces recognition #98 fifth witness (path-namespace scope)."

The path-namespace scope is structurally distinct from the prior four
because it is NOT a direct cryptographic-hash address. The `garden`
block identifies sources via path-and-name binding (`~git'…'`); the
underlying CAS is INHERITED from `@io/git`'s git_hash, but the SCOPING
DISCIPLINE at package-manager altitude is name-resolved-to-hash, not
hash-direct.

This witness adds a new structural dimension: the five scopes are not
all at the same structural altitude. Four are hash-direct; one
(@mirror/garden's path-namespace) is name-resolved-to-hash. The
pattern is graded; §3.6 returns to this.

`@mirror/garden`'s `garden_source = ref` (line 86) is typed as a
reference to a package source; the source is content-addressed via
composition with `@io/git`'s `git_hash` at resolution time. The graded
structure (name → hash) is itself recognition-territory adjacent; see
§8's O3.

### 2.4 The five witnesses are independent

- THREE independent commits, TWO authors (Mara: 1-4; Reed: 5), TWO
  sessions (2026-06-24: 1-4; 2026-06-25: 5).
- Five structurally-distinct scoping disciplines.
- Hash functions split 3:1:1 (SHA256: Nix/mirror oid/OCI;
  SHA1-or-SHA256: git; not-direct-hash: @mirror/garden).
- No retrofitting; each surfaced from its adapter's own substrate-
  pull pressure.

Five witnesses across two authors at independent altitudes is the
substrate-pull-correct floor for candidate-status surfacing. Promotion
requires Reed's tick plus §8's replication conditions.

---

## 3. The scope-graded structure — what each witness scope addresses

This section works out what each of the five scopes actually addresses,
making the scoping-discipline distinctions precise. The structural
claim from §1 is that the function is shared; this section names what
distinguishes each scope's domain.

### 3.1 Nix derivation hash — addressing BUILDS

Identifies a BUILD: closure of all build inputs (source, script,
dependencies, environment) producing an output, or — under ca-
derivations — the output's content directly. Hash function: SHA256
(current nixpkgs default). Determinism: input-addressed builds
deterministic on inputs; output-addressed builds deterministic on
outputs. Scope-shape: closure includes ALL transitively-reachable
inputs; the hash is a build-time fixed-point. Bridges: via
`pkgs.dockerTools.buildLayeredImage` (Nix → OCI tarball);
`pkgs.dockerTools.pullImage` (OCI → Nix store).

### 3.2 mirror oid — addressing SUBSTRATE CONTENT

A mirror oid (`@mirror/store` Splinter) identifies substrate content
bytes: OID-graph K_n where every typed substrate object has a content-
determined identity. Hash: SHA256. Determinism: byte-determined.
Scope-shape: flat content-addressing over substrate-typed bytes; no
closure, no version graph, no manifest layer. Bridges: `oid_to_digest`
(typed lift; byte-identity at SHA256 altitude); `hash_to_oid`
(function-altitude crossing in SHA1 mode).

### 3.3 OCI digest — addressing ARTIFACTS

An OCI digest (per OCI Image Spec v1.1.x §6 descriptor) identifies a
packaged artifact: a manifest JSON whose content references layer
descriptors, each content-addressed. Hash: SHA256. Determinism: byte-
determined on manifest JSON. Scope-shape: layered — digest addresses
manifest; manifest addresses descriptors; descriptors address layers.
A two-level CAS structure. Bridges: `pkgs.dockerTools` family (Nix
derivation → OCI tarball); skopeo/oras (tarball → registry).

### 3.4 git object hash — addressing VERSIONED OBJECTS

A git object hash identifies a git object (blob / tree / commit /
tag) in the versioned object graph. Hash: SHA1 default (40-hex) OR
SHA256 transitional (64-hex; Git 2.29+) — the function-altitude
partition is explicit per `@io/git`'s `hash_to_oid` action.
Determinism: per git's content-hashing rules (e.g.,
`commit <size>\0<header>\n<message>`). Scope-shape: versioned graph;
commits form a DAG; blob/tree content-addresses are leaves; trees
recursively content-addressed. Bridges: `hash_to_oid` (function-
altitude crossing; SHA1→SHA256 re-hash where required); forward-
promised `@spectral/garden/git` for per-peer-home-repo composition.

### 3.5 path-namespace scope — addressing PACKAGE SOURCES

The `@mirror/garden` `garden_source` carrier identifies a package
source at the dependency-resolution altitude. Typed `ref`; underlying
content-addressing is INHERITED from the source-root (currently git
for `~git'…'`; future oci/nix/store roots forward-promised per
`@spectral/garden/git` spec §6 four-root structure). For `~git'…'`
sources, the underlying hash is `@io/git`'s `git_hash`. Determinism:
the path-namespace binding maps to a unique resolved hash via the
source-root's resolution discipline; the binding ITSELF is name-shaped,
the hash is hash-shaped. Scope-shape: name-resolved-to-hash. Bridge:
forward-promised `garden_source → underlying-hash` resolution at
parse/settle altitude.

### 3.6 The graded structure

The five scopes split into TWO structural strata:

- **Stratum 1 — hash-direct scopes:** Nix derivation, mirror oid, OCI
  digest, git hash. Four scopes; each is a cryptographic-hash function
  directly applied to bytes; the address IS the hash.
- **Stratum 2 — name-resolved-to-hash scope:** `@mirror/garden`
  path-namespace. One scope; the address is a path-and-name binding
  that resolves to one of stratum 1's hashes at lookup time.

This graded structure is itself structurally meaningful. Stratum 1 is
the substrate's direct content-addressing discipline; stratum 2 is the
substrate's compositional content-addressing discipline (one altitude
of indirection above stratum 1). The pattern lifts: when a future
witness emerges at "package-of-packages" altitude (e.g., a workspace
of gardens), the stratum could continue.

Substrate-pull-confident reading: stratum 2's existence makes the
recognition more interesting, not less. The five-witness cluster is
not just a same-function-different-scope pattern; it is a graded
discipline where indirection composes over the direct addressing
altitude. §4 returns to this in the Grothendieck-topology framing.

---

## 4. Structural semantics — Grothendieck-topology framing + honesty about strength

The brief asks whether the five witnesses are "the same function
instantiated at different sites, or genuinely different functions that
happen to coincide." This section answers and names a candidate formal
framing with honest hedge on whether it earns the lines.

### 4.1 Same structural function, different sites

The substrate-pull-correct answer: **same STRUCTURAL function at
different SITES, with one site (git default) at a different
hash-function instance.**

All five sites instance cryptographic hashing over bytes producing
fixed-length digests with byte-determinism, collision-resistance, and
content-equality semantics. Each site addresses a different object
class (builds / substrate content / artifacts / versioned objects /
package sources); the object class IS the site. Stratum 1 sites use
SHA256 in four of five cases; git default is SHA1 (SHA256 transitional);
the function-altitude variation is real but sub-recognition.

Not coincidence: the substrate-pull pressure at each adapter altitude
PRODUCED the content-addressing requirement because the underlying
problem (identify objects by content rather than by external naming
authority) is shared across the five sites' object classes. The
recognition lifts the shared discipline; coincidence would be the
ABSENCE of the shared discipline, not its presence.

### 4.2 The Grothendieck-topology candidate framing

A Grothendieck topology on a category C is a system of covering
families satisfying sieve / pullback / transitivity axioms; sheaves
on it assign data to each object such that the data on coverings
glues consistently.

The candidate framing for #98:

- **Site:** the category of scopes (build, substrate-content,
  artifact, versioned-object, package-source); morphisms are the
  typed bridge actions (`oid_to_digest`, `hash_to_oid`, Nix↔OCI
  bridge via `dockerTools`, forward-promised `garden_source →
  underlying-hash`).
- **Coverings:** sets of bridge actions whose composition yields the
  full content-address chain across multiple scopes (e.g., mirror oid
  → OCI digest covers the joint scope substrate-content + artifact).
- **Sheaf of identities:** the per-scope content-addressing function;
  the sheaf condition would require that bridges preserve byte-
  identity when two scopes' addresses identify the same bytes.

The framing's appeal: a sheaf is exactly the data structure for "same
thing assigned to different sites, gluing consistently."

### 4.3 Honest hedge: does the framing earn the lines?

Per the brief's discipline and the Taut anti-pattern lesson from #99,
this canonical names the framing as CANDIDATE and hedges on strength.

**For:** precise formal name; composes with the already-substrate-
decl'd Connes-spectral-triple family (sheaves on sites + spectral
triples both live in non-commutative geometry); altitude-portable to
future witness scopes.

**Against:** the substrate does not currently substrate-decl
categorical / topos-theoretic primitives; lifting #98 to Grothendieck
would introduce vocabulary the substrate doesn't pull elsewhere. The
sheaf-condition gluing is not operationally verified for the bridge
actions (§8's O1). A simpler scope-graded framing covers the
structural claim equally well; substrate-pull favors simpler frames
when they suffice.

**The call:** the Grothendieck framing is CANDIDATE; the recognition
does NOT require it. The load-bearing claim is the structural-shape-
across-scopes pattern (§1); Grothendieck is one possible deeper
substrate-decl. Per the Mara honesty-limit discipline (per #99 §10
precedent): name the candidate, work it cleanly, don't promote the
framing on #98's strength alone.

If the substrate later substrate-decls sheaves at another altitude
(e.g., [[project-eigenboard-is-sheaf]]) AND the sheaf-condition is
operationally verified, the framing earns explicit substrate-decl.

### 4.4 The simpler alternative: scope-graded content-addressing

- **Carriers:** five typed scope-addresses (`derivation_hash`, `oid`,
  `oci_digest`, `git_hash`, `garden_source`).
- **Bridge actions:** typed lifts between scope-addresses
  (`oid_to_digest`, `hash_to_oid`, forward-promised Nix↔OCI, forward-
  promised `garden_source → underlying-hash`).
- **Composition law:** bridge actions compose; type-checked at
  substrate altitude; realisation discharges the function-altitude
  work (SHA256 vs SHA1; byte-equality vs re-hash).

This covers everything §4.2 covers except sheaf-condition naming.
Substrate-pull-confident: this is enough for #98 at candidate altitude.

### 4.5 What this recognition is NOT

- **NOT a claim that all hash functions are interchangeable.** SHA1's
  collision attacks (SHAttered, 2017) are real; security-critical
  scopes must specify their function. The recognition is about
  STRUCTURAL PATTERN across scopes, not function commutativity.
- **NOT a claim that the loop closes at content-addressing altitude.**
  Per `@io/oci` §6: the loop CLOSES at the structural altitude and
  OPENS at every scoping altitude. The closing-opening partition IS
  the structural fact.
- **NOT a definition of content-addressing.** The substrate is
  recognizing that content-addressing recurred at five distinct
  altitudes WITHOUT being introduced as a primitive. The recurrence
  IS the recognition.

---

## 5. Connection to #99 (mirror.spec IS λ₀) — does λ₀ have a content-address?

The brief asks: does λ₀ have a content-address? Is it one of the five
witness scopes, or a new (sixth) scope?

### 5.1 mirror.spec's content-address at each existing scope

mirror.spec is a file at the root of the mirror repository. All five
scopes admit a content-address for it:

- **mirror oid:** mirror.spec's bytes → SHA256 → mirror oid (Splinter
  discipline applies directly).
- **git object hash:** mirror.spec is a blob at HEAD; git_hash is
  well-defined.
- **Nix derivation hash:** mirror.spec is part of the build closure
  for `target binary`; the derivation hash includes its content.
- **OCI digest:** when shipped in an OCI artifact, mirror.spec
  contributes to a layer descriptor under the manifest's digest.
- **path-namespace:** a CONSUMER's garden block can reference
  mirror.spec's repository as a source (mirror's own garden can't —
  self-referential).

Five existing scopes, five existing content-addresses. None is a sixth.

### 5.2 Is there a "λ₀ content-address" that is distinguished?

Three candidate readings:

- **Reading A: mirror oid is distinguished.** Splinter is the
  substrate's NATIVE content-addressing at substrate altitude; the
  other four scopes are EXTERNAL adapters at @io. λ₀ inhabits
  substrate altitude; the substrate's native content-addressing of
  itself IS the mirror oid.
- **Reading B: all five are equal-citizens.** Per #99 §8.6 (altitude-
  portability), each scope's content-address identifies the same bytes
  from its scoping discipline; no scope is privileged.
- **Reading C: no λ₀ content-address.** Per the Void document, λ₀ is
  the consensus state — the BYTES, not a hash. Content-addressing is
  a verification discipline, not an identity-naming discipline.

The substrate-pull-correct call: **Reading A substrate-pull-favored;
B and C coherent at lower confidence.** Argument for A: λ₀ inhabits
substrate altitude; the substrate's native content-addressing-of-
itself IS the mirror oid at that altitude. This canonical commits to
Reading A; forward-promises resolution at §8's O4.

### 5.3 Is there a sixth scope at the λ₀ altitude?
