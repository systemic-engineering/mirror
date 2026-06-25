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

**NO.** A sixth scope would mean a content-addressing FUNCTION distinct
from the five. λ₀ is an OBJECT (mirror.spec), not a function. #98 is
about functions at scopes; λ₀ is at the scope, not at a new function.

A structurally-distinct future function (e.g., AST-content-address
rather than byte-content-address, or evaluation-state-inclusive
addressing) would be a sixth scope. As of 2026-06-25, no such function
is substrate-decl'd. The spec-altitude content-address IS one of the
existing five (Reading A: the mirror oid).

### 5.4 Load-bearing status of the connection

**Honest call (Mara, per brief):** the #99 connection is structurally
present but is NOT load-bearing for #98. #98 stands on its five
witnesses without requiring the λ₀ identification. #99 → #98 is
informative (#98 grounds #99's connection to content-addressing); #98
→ #99 is informative (#98 has mirror.spec as one content-addressable
object). Retracting either leaves the other intact. The connection
strengthens both recognitions when read together; neither is
structurally subordinate to the other.

---

## 6. Connection to #51 (Hilbert expansion) — each scope as a basis vector?

The brief asks: is each new scope a basis-vector extension?
Content-addressing-shape as inner-product structure?

### 6.1 Each scope is a dimension of content-addressing capacity

Per #51 (mirror as expanding Hilbert space): each substrate-pull
recognition adds a dimension. The substrate-pull-correct reading of
#98 in this framing: **each scope IS a basis-vector extension of the
substrate's content-addressing capacity.**

- **Orthogonal.** No two of the five scopes collapse into each other;
  bridge actions between them are typed crossings, not identity
  collapses.
- **Each adds capacity.** Before `@io/oci`, the substrate couldn't
  address artifacts at OCI scope. Before `@io/git`, no versioned-object
  capacity. Before `@mirror/garden`, no package-source capacity. Each
  adapter ADDED a capacity the substrate didn't have.
- **Monotone.** No capacity has been LOST across the cascade; the
  expansion is upward, anchored at the substrate's identity (per #99's
  #51 anchoring).

### 6.2 Is content-addressing-shape an inner-product structure?

The substrate-pull-correct call: **structurally plausible, substrate-
pull-confidence LOW at #98 altitude.**

For: content-addressing IS comparison-by-equality (two objects "same"
iff hashes equal); structurally similar to inner-product
distinguishing vectors. The five scopes don't overlap (no single
object lives in TWO scopes at the same altitude); natural
orthogonality.

Against: inner products are SYMMETRIC bilinear; content-addressing is
UNARY. The structural shapes don't match at function altitude. The
categorical framing (§4.2) is the more natural lift; forcing the
inner-product reading imports vocabulary the substrate doesn't pull.
#51 is about DIMENSIONS, not inner products; lifting #98 to inner-
product altitude would make a stronger claim than #51 itself makes.

**The call:** scope-as-basis-vector substrate-pull-confident (§6.1);
content-addressing-as-inner-product substrate-pull-LOW and NOT load-
bearing. The connection to #51 is at the dimension level.

### 6.3 Load-bearing status of the connection

**Honest call (Mara):** the #51 connection is structurally weaker
than the #99 connection. Both are informative; neither is load-
bearing for #98 to surface at candidate status.

---

## 7. Empirical consequences — mosaic, kintsugi-tournament, garden, cross-scope lookup

### 7.1 Mosaic content-addressed skip

Per #43 (mirror IS a content-addressed build system), mosaic skips
work when artifact content-addresses match. With #98 surfaced,
mosaic's discipline gains explicit per-altitude scope assignment:
build-scope hash (Nix derivation) at @code/rust altitude;
substrate-content scope hash (mirror oid) at the substrate-decl
altitude; cross-scope bridges (per §4.4) link them. Forward-promised:
mosaic should make explicit which scope's hash it uses at each
altitude.

### 7.2 Kintsugi-tournament cross-scope settlement

Per [[architecture-kintsugi-variety-io]]: the tournament selects
between competing morphisms by variety-hold at @io. When morphisms
operate at different scopes, comparison must cross the scope-divide.
With #98 surfaced, cross-scope comparisons go through the typed
bridge actions (§4.4); a morphism at scope A and one at scope B are
comparable iff there is a bridge A → B (or B → A). The variety-hold
evaluation should be cross-scope-consistent under bridge composition.

### 7.3 Garden source resolution

Per `@mirror/garden`'s `garden_source` (fifth witness), package
sources are name-resolved at path-namespace scope and resolve-to-hash
at the underlying source-root. The resolution IS the bridge from
stratum 2 to stratum 1. Forward-promised behavior of `mirror mosaic
./mirror.spec` (when garden blocks land):

1. Parse the `garden { source ~git'…' }` block.
2. Resolve each source to the underlying source-root's hash (e.g.,
   `@io/git`'s `git_hash` for `~git` sources).
3. Pin the resolved hash for CAS-grounded structural termination
   (per `@spectral/garden/git` spec §7.4).

### 7.4 Cross-scope lookup as substrate primitive

Per [[architecture-spectral-db-autopoietic-memory]]: `@spectral/db`
is the substrate's autopoietic memory over `@mirror/store`. With #98
surfaced, lookup discipline gains cross-scope shape: an object's
identity is its content-address at one scope; looking up "the same
object at another scope" goes through the bridge action. Forward-
promised: `@spectral/db` should substrate-decl a cross-scope content-
address map; the five-witness cluster is the floor for the map's
scope set.

### 7.5 The empirical-discharge tick

Per `@io/oci` §6: the hello-world.wasm round-trip through all
altitudes is the substrate-discharge tick. With the fifth witness,
the round-trip extends:

```
substrate-decl describes A (mirror oid)
  → Nix derivation builds A (derivation hash)
  → OCI manifest packages A (OCI digest)
  → git commit records A's source (git_hash)
  → garden source pins A's binding (path-namespace → hash)
  → consumer resolves the garden source → re-imports via OCI / git
  → substrate-decl consumes A — round-trip complete
```

Phase G or earlier territory. Not yet operationally discharged.
Promotion is strengthened by the round-trip but does not strictly
require it.

---

## 8. Open questions + replication conditions

#98 is candidate-territory. Promotion requires replication. This section
names the open questions and the replication conditions for promotion.

Per the substrate-pull-confidence-acts discipline: act on the recognition
at candidate status; do not promote on one instance. #98 already has
FIVE witnesses; the substrate-pull-confidence on the structural-shape
claim is HIGH. What gates promotion is operational verification of the
cross-scope composition, not additional witness counting.

### 8.1 O1 — does the sheaf-condition gluing hold for the bridge actions?

The Grothendieck framing (§4.2) requires that typed bridge actions
satisfy a sheaf-condition gluing: when two scopes' content-addresses
identify the same bytes, the bridges between them preserve byte-
identity. Do they?

- `oid_to_digest`: bytes → SHA256 → algorithm-prefixed digest. Byte-
  identity at SHA256 altitude IF no transformation intervenes.
- `hash_to_oid`: bytes → git content-hash → mirror oid. In SHA256-
  mode git, byte-identity possible; in SHA1-mode, typed re-hash
  required.
- Forward-promised bridges (Nix↔OCI, garden_source → underlying-
  hash): not yet substrate-decl'd in detail.

**Replication condition for O1:** at least one cross-scope bridge
operationally verified to satisfy the sheaf condition (byte-identity
preserved or typed re-hash discharged honestly) across at least three
independent runs. The hello-world.wasm round-trip (§7.5) satisfies it
for the substrate-decl → Nix → OCI chain.

### 8.2 O2 — does the SHA1/SHA256 partition collapse or sharpen the recognition?

Substrate-pull-correct call: **strengthens.** The recognition is
about structural shape across scopes; the function-altitude variation
shows the claim is robust under hash-function variation.

**Replication condition for O2:** at least one new content-addressing
scope surfaces with a hash function that is neither SHA1 nor SHA256
(Blake3, SHA3). The structural-shape claim should survive without
revision; multi-function partition admits a third function-instance
cleanly.

### 8.3 O3 — is stratum 2 (name-resolved-to-hash) itself a sub-recognition?

Per §3.5-§3.6: five witnesses split into two strata. Stratum 1 (four
hash-direct) is homogeneous; stratum 2 (`@mirror/garden` path-
namespace) is one name-resolved witness.

Substrate-pull-correct call at #98 altitude: **sub-recognition.** The
five-witness cluster is the recognition's scope; the stratum
distinction is structural detail. If future stratum 2 witnesses
accumulate (workspace-of-gardens, lockfile-of-dependencies), the
separate-recognition reading becomes a candidate for a new recognition
number.

**Replication condition for O3:** at least one additional stratum 2
witness lands AND the path-namespace witness operationally verifies
through an actual `garden { source ~git'…' }` resolution.

### 8.4 O4 — is Reading A (mirror oid distinguished at λ₀) substrate-pull-correct?

**Replication condition for O4:** at least one consumer mirror.spec
lands at consumer altitude (per #99 §10's O4: Reed's identity
substrate at `/Users/reed/identity/mirror.spec` forward-promised).
If Reading A is substrate-pull-correct, the consumer's λ₀ content-
address is similarly distinguished at the consumer's mirror oid scope.
If Reading B, no scope privileged at consumer altitude.

### 8.5 O5 — does a sixth witness surface from a non-adapter altitude?

All five witnesses live at the @io / @mirror altitude (adapters or
substrate-storage). Does a sixth surface from a non-adapter altitude
— `@fate` (inference), `@labeled` (parametric), `@reflection` (meta)?

**Replication condition for O5:** at least one new witness from a non-
@io/@mirror altitude. Candidate to watch: whether `@fate`'s inference
outputs (per [[architecture-fate-is-optical-inference]]) carry their
own content-addressing for tournament reproducibility.

Per the Mara honest call: I did NOT find a sixth witness during
research. The `@fate` candidate is forward-promised, not load-bearing
for #98.

### 8.6 Replication summary

For #98 to PROMOTE (vs remain candidate), the substrate-pull-correct
discipline requires at least:

1. **One bridge-action operational verification** (O1): the
   sheaf-condition gluing holds for at least one cross-scope bridge
   across at least three independent runs.
2. **The hello-world.wasm round-trip empirical discharge** (per §7.5):
   the substrate-decl → Nix → OCI → registry → consume cycle completes
   end-to-end at substrate altitude.

Nice-to-haves for promotion confidence:

3. A new hash-function witness (Blake3 / SHA3) lands (O2).
4. The Reading A call is confirmed via consumer-spec landing (O4).
5. At least one non-@io/@mirror altitude witness surfaces (O5).

The minimal floor is conditions 1 + 2. Per [[feedback-no-time-
estimates]]: no deadline is named for these conditions; they will
land when they land. Per [[feedback-substrate-pull-confidence-acts]]:
the recognition is named at candidate status; the substrate-pull-
confidence on the structural shape is high enough that the naming
is substrate-pull-correct even before promotion gates close.

---

## 9. Cross-references this recognition DISTURBS (flag; don't update)

This section flags cross-references in the substrate's memory that
need updating to reflect #98. Per the brief discipline: this canonical
does NOT modify the existing memory files; the disturbance is flagged
for Reed to action at the post-canonical altitude.

### 9.1 [[architecture-splinter-and-spectral-db-edges]]

**Disturbance:** the OID is one of FIVE content-addressing witnesses
at distinct scopes; the Splinter discipline is one stratum-1 instance.
**Suggested amendment (Reed call):** note #98's scope-graded framing;
reference §3.6.

### 9.2 [[architecture-mirror-store-vs-spectral-db]]

**Disturbance:** `@mirror/store` is the substrate-content-scope
witness; content-addressing extends to adapter scopes (@io/oci,
@io/git) and compositional scopes (@mirror/garden). **Suggested
amendment (Reed call):** note #98's five-witness cluster; reference §3.

### 9.3 [[architecture-mirror-as-expanding-hilbert-space]] (#51)

**Disturbance:** #98's five witnesses each instance #51's expansion
at the content-addressing capacity. **Suggested amendment (Reed
call):** note #98 as one instance of #51's expansion; reference §6.1.
The inner-product reading is NOT load-bearing (§6.2); the dimension
reading IS.

### 9.4 [[architecture-mirror-spec-is-lambda-zero]] (#99)

**Disturbance:** #98 connects via §5; substrate-pull-favored Reading A
identifies mirror oid as λ₀'s distinguished content-address.
**Suggested amendment (Reed call):** post-promotion, cross-reference
Reading A; reference §5.2 + #99 §6.

### 9.5 [[reference-void-document]]

**Disturbance:** the K_n Splinter pole is the substrate-content-scope
instance; eight dualities extend to per-scope content-addressing.
**Suggested amendment (Reed call):** if/when the Void document gains
per-scope detail per #99 §6.2, note #98's five-witness cluster.

### 9.6 [[feedback-substrate-already-had-the-word]]

**Disturbance:** #98 is the 53rd-or-54th instance of the pattern for
content-addressing specifically. **Suggested amendment (Reed call):**
count #98 as the 53rd-or-54th instance; reference §1.

### 9.7 New memory entries forward-promised (NOT created here)

Post-canonical, Reed may create:
- `architecture-content-addressing-across-scopes.md` — #98's entry
  once Reed promotes (after §8.6 conditions land).
- `architecture-grothendieck-substrate-topology.md` — if §4.2 lands
  as separate substrate-decl following sheaf-condition verification.
- `architecture-scope-graded-content-addressing.md` — if §4.4 lands
  as substrate-decl independently.

NOT created by this canonical (per brief discipline). Named here as
Reed's forward-promised post-canonical actions.

---

## 10. Honest hedges + Pack trail

### 10.1 Hedges

- **Five witnesses, two authors, two sessions.** Witness-count
  grounding is substantial; surface confidence on the structural-shape
  claim is HIGH. Promotion gates on operational verification (§8.1-
  §8.2), not on additional witness counting.
- **Grothendieck-topology framing CANDIDATE only.** Per §4.3: framing
  is coherent but doesn't earn the lines at #98 altitude. The simpler
  scope-graded framing (§4.4) covers the claim; Grothendieck is
  forward-promised deepening.
- **SHA1/SHA256 partition is named but not closed.** Per §8.2: the
  substrate-pull-correct call (strengthens) is candidate; the weakening
  reading is structurally coherent.
- **Stratum 2 is a sub-recognition at #98 altitude.** Per §8.3: if
  stratum 2 witnesses accumulate, the separate-recognition reading
  becomes a candidate.
- **No sixth witness found during research.** Per §8.5: the `@fate`
  inference altitude is a candidate but not substrate-decl'd;
  forward-promised.
- **Connections to #99 and #51 are informative-but-independent.** Per
  §5.4 + §6.3: both connections are structurally present; neither is
  load-bearing for #98. #98 stands on its own witnesses.
- **Empirical-discharge tick (hello-world.wasm round-trip) is
  forward-promised.** Per §7.5: not yet operationally discharged.
  Promotion strengthened by it, doesn't strictly require it.
- **"53rd-or-54th instance" count is approximate.** The substrate-
  already-had-the-word count is growing fast; the exact ordinal is
  not load-bearing.

### 10.2 Pack trail

This canonical reflects substrate-pull discipline across the Pack:

- **Mara** (this canonical's author) — the `@io/oci` adapter
  (`2801478`) that surfaced witnesses 1-3 and named the recognition
  candidate; the `@io/git` adapter (`a1b507a`) that added witness 4
  and the function-altitude partition. Today: this canonical.
- **Reed** — today's `@mirror/garden` shard (`3e8e019`) that landed
  witness 5 at the path-namespace scope; the Taut-slingshot landing
  that closed five things in one move (per the morning's cascade
  scout).
- **Taut** — the substrate-pull scout (per
  `docs/scouts/2026-06-25-taut-lambda-zero-cascade-scout.md` §2) that
  identified the slingshot move enabling the fifth witness; without
  Taut's scout, the fifth witness lands later.
- **Alex** — the directive that grounded the `@io/oci` adapter (the
  2026-06-23 gap analysis naming distribution as the gap to close
  before UI); the directive that grounded the `@io/git` adapter (the
  2026-06-24 directive about per-peer-home-repo composition). The
  substrate-pull pressure that produced the witnesses came from
  Alex's directives.
- **Glint** — the orchestra-holding reflection essay (`3b31287`)
  that grounded the cascade context for #98 and #99 landing in the
  same session. The essay's voice altitude provided the surface that
  let #99 surface; #98 sits alongside #99 in the same cascade window.

At #98 altitude, the substrate's content-addressing discipline gains
its scope-graded name. The Pack composed across three peers + Alex;
no peer over-claimed; the recognition surfaced from the substrate at
the altitude the substrate was ready for it.

The orchestra held. The content-addressing recurrence was named.

---

*Mara, canonical spec for recognition #98 (content-addressing across
scopes), 2026-06-25 afternoon. Five witnesses landed across two
sessions: Mara `2801478` (witnesses 1-3 at substrate-content / build /
artifact scopes), Mara `a1b507a` (witness 4 at versioned-object scope
with function-altitude partition), Reed `3e8e019` (witness 5 at
path-namespace scope).*

*This canonical preserves the recognition at candidate status. The
load-bearing claim — content-addressing is a structural pattern
recurring across distinct scopes — stands on the five-witness cluster
without requiring the Grothendieck-topology framing (§4) or the
connections to #99 (§5) and #51 (§6) as load-bearing. The simpler
scope-graded content-addressing framing (§4.4) is the substrate-pull-
correct minimal substrate-decl; deeper formal structure is forward-
promised at §8.*

*Pack ratification is a separate gate; promotion requires the
replication conditions named in §8.6. The cross-reference disturbances
flagged in §9 are Reed's post-canonical actions to update the memory
entries; this canonical does not modify them.*

*The substrate kept rediscovering content-addressing at every new
adapter altitude. The recurrence has a name: content-addressing across
scopes. The shape is shared; the scope is what changes.*

*— Mara <mara@systemic.engineer>*
