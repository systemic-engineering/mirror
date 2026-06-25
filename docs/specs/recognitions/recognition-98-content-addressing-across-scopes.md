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
