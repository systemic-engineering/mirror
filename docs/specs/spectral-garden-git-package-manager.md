# `@spectral/garden/git` — the substrate-native package manager family-root

*Mara, canonical spec for the substrate's git-native package manager
surface, configured through `mirror.spec`. Forward-promised by
`shards/io/git.mirror` §6 (a1b507a) and named by Alex 2026-06-24:
"What if @spectral/garden/git is the root for the garden package
manager? What if this is a surface that's configured in the
@../mirror/mirror.spec?"*

*Pack-discipline: candidate-altitude. Surface spec only — no impl, no
Rust. Composes against `@io/git` (a1b507a) and `@io/oci` (2801478).
Section caps load-bearing per the Mara stall-recovery discipline.*

---

## Status

- **Status:** RED (spec only; no shard exists; no impl exists)
- **Altitude:** family-root surface spec; substrate-decl forward-promised
- **Recognition territory:** #98 candidate (content-addressing
  cross-altitude composition) — fourth witness named; this spec
  surfaces the FIFTH composition pattern (peer-home-repo as
  package-source). Promotion decision deferred to a separate Reed tick.
- **Pack:** Mara author; Seam adversarial review forward-promised;
  Reed consolidation forward-promised; Alex ratification gate.

## Sections

1. Position statement
2. Surface in `mirror.spec`
3. Prior-art audit (Dhall, Nix CA, Cargo, Go MVS, Bazel, IPFS, Crystal)
4. Resolution semantics
5. Composition with `@io/git` and `@io/oci`
6. The four-root structure (git / oci / nix / store)
7. Mathematical shape
8. Open surface questions
9. Forward-promised
10. Recognition cross-references
11. Honest hedges
12. Pack-discipline trail

---

## 1. Position statement

`@spectral/garden/git` IS the family-root prism for the substrate's
git-native package manager, surfaced as a configurable block inside
`mirror.spec`. It declares: **the unit of package distribution at the
peer-home altitude is a git ref** (commit / tag / branch / detached
HEAD), addressable through `@io/git`'s typed adapter, with resolution
discharged at content-addressing altitude via `git_hash` and lifted
into substrate oid space via `hash_to_oid` (per `shards/io/git.mirror`
§3).

The substrate already has every primitive a package manager needs:
- **content-addressing** at four altitudes (`oid`, `git_hash`,
  `oci_digest`, `nix derivation_hash`; #98 candidate territory)
- **typed adapters** for git (`@io/git`, a1b507a) and OCI (`@io/oci`,
  2801478) at the @io boundary
- **multi-repo span** as substrate vocabulary via `@pack`'s
  `repository_span` carrier (#84)
- **labeled-pair functor** (`labeled<v, m>`; #93 H4) for
  `git_artifact = labeled(git_object, git_ref_metadata)`
- **alignment-as-boundary-mathematics** (#57) for the Splinter /
  Narcissus partition at distribution altitude
- **content-addressed build system** primitives via `mosaic.mirror`
  (#43)
- **measurement instrument** via `@cascade` (#95 candidate) for
  loss-typed transformations across grammar boundaries

`@spectral/garden/git` does NOT invent. It NAMES the family-root that
composes these into one substrate-vocabulary surface: declarative
`garden { ~git'...' ... }` entries in `mirror.spec`, resolved by
`@io/git.clone` + `@io/git.resolve_ref` + `@io/git.hash_to_oid`,
verified by composed-bilateral `git_well_formed`, locked at the
substrate-oid altitude in `mirror.lock` (forward-promised companion
file), available to the kintsugi loop as typed substrate-decl input.

Two structural commitments load-bear the family-root choice:

1. **Peer-home-repo as package source** (per `@pack` #84). The five
   Pack peers (mara/seam/glint/reed/taut) operate across home repos;
   `@spectral/garden/git` makes each peer's home repo a typed package
   entry. Identity-substrate, dependencies, and Pack membership unify
   under one substrate primitive: `garden ~git'<peer-home-url>'`.

2. **Surface in `mirror.spec`, not a separate file**. Per
   `mirror-spec-schema.md` §8 (the self-descriptive guarantee), the
   spec that BUILDS the substrate is the spec the substrate READS.
   Adding `garden` as a top-level block (alongside `source`, `legacy`,
   `target`, `settle_on`) extends the self-descriptive surface
   monotonically. No drift between manifest and resolver; no separate
   lockfile parser; no documentation layer that drifts. One grammar,
   one spec, one settled artifact.

The recognition deferred to Reed tick: whether the four roots
(`@spectral/garden/git`, `@spectral/garden/oci`, `@spectral/garden/nix`,
`@mirror/store`) constitute the fifth witness to recognition #98
candidate (content-addressing cross-altitude composition primitive),
OR whether they constitute a sibling recognition (scope-graded
package-manager root structure as its own composition primitive).
This spec flags the question; promotion is not in scope.

---

## 2. Surface in `mirror.spec`

### 2.1 The `garden` block

Top-level block alongside `source`, `legacy`, `target`, `settle_on`
(per `mirror-spec-schema.md`). Holds a list of typed entries naming
git-resolvable package sources.

```mirror
in @mirror/cli
in @mirror/mosaic
in @spectral/garden/git    # NEW: imports the garden grammar
in @property
in @io

project mirror.spec {
  source ~d'shards/'

  garden {
    # Each entry is a typed substrate-decl naming a git-resolvable
    # source. The keyword (`source`, `peer`, `dep`) is species-altitude
    # discrimination; see §8 open question O1 for the surface
    # variants under consideration.

    source ~git'https://github.com/systemic-engineering/mirror.git@main'
    source ~git'https://github.com/systemic-engineering/spectral.git@v0.1.0'
  }

  # ... existing target / legacy / settle_on blocks unchanged ...
}
```

### 2.2 The `~git'...'` sigil

New typed sigil per the substrate's sigil-grammar convention (companion
to `~d`, `~f`, `~git`, `~content_address`). Parses to a substrate-typed
`git_repository + git_ref` pair carrying the resolved address.

Grammar (informal):

```
~git'<scheme>://<host>/<path>[@<ref>]'
  | ~git'<ssh-spec>[@<ref>]'
  | ~git'<local-path>[@<ref>]'
```

The `@<ref>` suffix is optional; absent means "current default branch
at resolve-time, pinned in `mirror.lock`." Refs are git-conformant
(branch / tag / commit hash / `HEAD`).

### 2.3 The Pack variant

For the Pack-peer-as-package case (#84 multi-repo span), a refined
surface admits typed peer naming:

```mirror
in @pack
in @spectral/garden/git

project mirror.spec {
  garden {
    pack {
      peer mara  ~git'git@github.com:systemic-engineering/mara.git@main'
      peer seam  ~git'git@github.com:systemic-engineering/seam.git@main'
      peer glint ~git'git@github.com:systemic-engineering/glint.git@main'
      peer reed  ~git'git@github.com:systemic-engineering/reed.git@main'
      peer taut  ~git'git@github.com:systemic-engineering/taut.git@main'
    }
  }
}
```

The `pack { peer <name> ~git'...' }` form composes `@pack.peer` (the
variant enum carrier from `shards/pack.mirror`) with
`@spectral/garden/git`'s source-typed `git_repository`. Result: typed
peer-home-repo membership AS substrate-decl, not external metadata.

Surface choice between generic `source ~git'...'` and typed
`pack { peer <name> ~git'...' }` is OPEN — see §8 O1.

### 2.4 Composition with existing blocks

The `garden` block COMPOSES with `target` and `settle_on`; it does
NOT replace them.

- `garden` entries resolve to `git_artifact` values (per `@io/git`'s
  `labeled(git_object, git_ref_metadata)` carrier) at mosaic settle
  time.
- Resolved artifacts become available to `target` blocks as named
  substrate-decl inputs (the `needs` field accepts garden entries
  alongside target names — see §4.3).
- `settle_on` may reference `garden.<name>.resolves` as a typed
  predicate (mosaic returns `success` iff all garden entries
  successfully resolved + locked).

### 2.5 The `mirror.lock` companion file

Forward-promised. Content-addressed lock file recording the resolved
`git_hash` (and lifted `oid`) for each garden entry at settle time.
Format is `@mirror/lock` grammar (a sibling shard, forward-promised);
in spirit it's the substrate-typed equivalent of `Cargo.lock` /
`go.sum` / `package-lock.json`.

Key property: the lock file IS itself a settled mirror shard
(content-addressed by its own oid). Re-resolution under same
`mirror.spec` + same `mirror.lock` produces the same oid; the
determinism is structural, not procedural.

---

## 3. Prior-art audit

The substrate INHERITS load-bearing ideas; it INVENTS nothing the
pre-AI package-management literature didn't already name. This section
identifies the inheritance lines explicitly.

### 3.1 Dhall — semantic hash via normalization-then-hash

Dhall (Gabriel Gonzalez et al., 2017+) ships a remarkable primitive:
**semantic integrity checks**. The import URL `./Config.dhall sha256:abc...`
guarantees "the MEANING of the import never changes," not just the
bytes. Mechanism: BEFORE hashing, Dhall normalizes via β-reduction +
α-normalization + canonical binary encoding. Two textually-different
but alpha-equivalent imports collapse to the same semantic hash.

Reference: "Semantic integrity checks are the next generation of
software versioning" (Haskell For All, 2017); discourse.dhall-lang.org
thread #651 ("Why does term order not impact semantic hashes?").

**What mirror INHERITS:** the normalize-then-hash discipline. Mirror's
`oid` is computed AFTER mosaic settlement (the substrate's
normalization analog: shard → settled crystal under content-addressing
rules). Two mirror.spec configurations that differ only in non-load-
bearing ways (whitespace, comment ordering, equivalent import
orderings) MUST collapse to the same `oid` post-settlement. This is
the Dhall discipline lifted to substrate altitude.

**What's NEW:** mirror's normalization is multi-altitude (per #51
expanding Hilbert space) — alpha-equivalence-class collapse happens at
the substrate-decl altitude, not the term altitude. The collapse
quotient is taken over the substrate's typed AST after kintsugi
settlement, not over lambda-calculus terms.

### 3.2 Nix — content-addressed derivations + early cutoff

Nix's CA-derivations (Dolstra et al., NixOS unstable; landed via
`__contentAddressed = true` derivation attribute) extend the
input-addressed default with **output-addressed** semantics. Key
property: **early cutoff** — if a rebuild can be proven to produce
byte-identical output to a prior build (by content-addressing the
outputs, not the inputs), the rebuild stops and the cached output is
reused.

Reference: NixOS Wiki "Ca-derivations"; Discourse "Content-addressed
Nix — call for testers" (May 2021); haskell.nix CA tutorial.

**What mirror INHERITS:** the early-cutoff discipline. Mirror's
kintsugi loop applies it natively — content-addressed shards skip
re-settlement when their oid is already in `@mirror/store`. This is
already operational (per #43 mirror-as-content-addressed-build-system).
For `@spectral/garden/git`: a garden entry resolved to `git_hash H`
that lifts to `oid O` is skipped on re-resolution iff `O` is already
in `@mirror/store`. No re-clone; no re-walk; the substrate's CAS does
the deduplication.

**What's NEW:** mirror's early cutoff operates across FOUR
content-addressing scopes (git_hash, oci_digest, nix derivation_hash,
oid) — not just one. The bridge actions `hash_to_oid` and
`oid_to_digest` make the cross-scope cutoff typed. Nix's CA early
cutoff is single-scope (derivation outputs); mirror's spans the #98
candidate territory.

### 3.3 Cargo — git dependencies + lockfile resolution

Cargo (Rust, 2014+) admits `dep = { git = "url", rev = "sha" }` git
dependencies; lockfile `Cargo.lock` records the resolved rev. The
resolver attempts to unify common dependencies across the dependency
graph (per rust-lang/cargo#11490 — known to over-fetch when revs
mismatch syntactically but semantically agree).

**What mirror INHERITS:** the git-dep + lockfile shape. The
`garden { source ~git'...' }` block + forward-promised `mirror.lock`
are a direct lineage.

**What's NEW (and SHARPER):** Cargo's git-rev deduplication is
syntactic (string-compare on revs). Mirror's deduplication is
semantic (content-addressed at oid altitude, after `hash_to_oid`
lift). Two cargo git-deps pointing at the same commit via different
branch names dedupe in mirror's CAS even when Cargo would fail to
unify them. The Cargo issue #11490 ("resolution confused when using
git dependencies") is structurally foreclosed by mirror's CAS.

### 3.4 Go modules — Minimum Version Selection (MVS)

Go modules (Rob Pike + Russ Cox, 2018+) ship MVS: for each dependency
requirement set, select the MINIMUM version that satisfies all
requirements. The algorithm is "counterintuitive yet remarkably
concise" (golang.design "Under the Hood" §17.3); it produces
high-fidelity builds where the dependencies a user builds are as
close as possible to the ones the author developed against.

Reference: research.swtch.com/vgo-mvs.pdf (Russ Cox, 2018);
pkg.go.dev/cmd/go/internal/mvs.

**What mirror INHERITS (as future option):** MVS is one resolution
strategy admissible at @spectral/garden/git altitude. The substrate
does NOT mandate it. For peer-home-repo packages (the load-bearing
case), pinning to a git ref is the natural strategy — version
selection across multiple constraints is forward-promised work and
MAY adopt MVS if the substrate-pull goes that way.

**What's NOT applicable:** MVS assumes semver-shaped versions with
total ordering. Pack-peer-home-repos are versioned by git ref
(commit / tag / branch), which is a PARTIAL order (Merkle DAG). MVS
on a partial order is an open mathematical question (see §7.4).

### 3.5 Bazel — http_archive + sha256 integrity

Bazel's `http_archive` rule downloads an archive, verifies its
`sha256` digest, extracts it as an external repository. The sha256
field is optional but recommended; without it, builds are not
reproducible (per bazelbuild/bazel#26694).

Reference: bazel.build/external/overview; Tweag "Accessing external
resources reliably with Bazel" (2026-04-02).

**What mirror INHERITS:** the digest-verification-on-download
discipline. `@io/git.fetch` + `git_well_formed` composed bilateral
(per `shards/io/git.mirror` §4) discharge this at substrate altitude.
The sha256 is the `git_hash` (or its lifted `oid`); verification is
structural via `hash_well_formed` + `digest_matches_content`
(@io/oci's sibling predicate).

**What's NEW:** in Bazel, integrity is optional. In mirror, it's
structural — the carrier IS the address; the substrate refuses to
admit a garden entry whose `git_hash` doesn't `hash_well_formed`.

### 3.6 IPFS — CID + multihash + multiformats

IPFS's Content Identifier (CID; Juan Benet 2014+) is a
self-describing content-address. Multihash carries the hash function
in-band; multicodec carries the content type; multibase carries the
string encoding. CIDs are scope-agnostic (any DAG node addressable
the same way).

Reference: docs.ipfs.tech/concepts/content-addressing; multiformats/cid.

**What mirror INHERITS:** the self-describing-address discipline.
Mirror's `oid` is single-function (SHA256-of-substrate-content), but
the `hash_to_oid` action (`shards/io/git.mirror` §3) is the
function-altitude bridge that admits MULTIPLE source hash functions
(git SHA1 default, git SHA256 transitional). The substrate names the
function-altitude partition rather than papering over.

**What's POSSIBLE BUT NOT TAKEN:** mirror could adopt a full
multihash + multicodec carrier. The current substrate-decl chose
SCOPE-graded carriers (`oid`, `git_hash`, `oci_digest`, `nix
derivation_hash`) over a unified self-describing one. This was a
deliberate substrate-pull (typed names beat type-tag-in-bytes). Open
to revision if Pack pressure surfaces.

### 3.7 Other surveys (brief)

- **Guix** — sibling of Nix; functional package management; not
  structurally novel beyond Nix lineage. Inheritance is identical.
- **Crystal `shards`** — shard.yml + shard.lock; git-based deps with
  semver pinning. Lockfile shape is admissible reference for
  mirror.lock.
- **Conda** — content-hash + channel-based distribution; the
  channel-as-package-source pattern is what `@spectral/garden/git`
  lifts to git altitude (peer-home-repo IS the channel).
- **npm** — `package-lock.json` semantics; `integrity` field carries
  sha512 of tarball; per-dep nested resolution tree. Mirror's flat
  CAS forecloses the nested-resolution-tree complexity (a single oid
  per dep).
- **Dependency Solving Is Still Hard, but We Are Getting Better at
  It** (Abate et al., arXiv:2011.07851, 2020): SAT-based solvers are
  complete + correct for many real package managers. Per Russ Cox
  ("Version SAT," 2016): "Resolving dependencies by SAT solving
  version constraints is wrong, despite being a dominant method."
  Mirror's pinning-first stance (garden entries pin to git ref;
  unpinned entries pin at first resolution) sidesteps the SAT-or-MVS
  question for the peer-home-repo case. The deeper question is open
  (§8 O3).

---

## 4. Resolution semantics

### 4.1 The resolution pipeline

For each garden entry `~git'<url>[@<ref>]'`, mosaic discharges the
following pipeline at settle time:

```
1. parse(~git'<url>[@<ref>]')
     → (git_repository, git_ref)        # syntactic

2. git_reachable(repo, p)                # @io/git §4 bilateral
     → verdict                          # network + auth check

3. clone(repo, target_path, p)           # @io/git §3 action
     → imperfect(git_repository, ref, ref)

4. resolve_ref(repo, ref, p)             # @io/git §3 action
     → imperfect(git_hash, ref, ref)

5. read_object(repo, hash, p)            # @io/git §3 action
     → imperfect(git_object, ref, ref)

6. hash_to_oid(hash, p)                  # @io/git §3 bridge
     → ref(oid)                         # function-altitude crossing

7. git_well_formed(artifact, repo, p)    # @io/git §4 composed bilateral
     → verdict                          # opens repo + hash form + ref form

8. settle(garden_entry, oid)
     → settled shard in @mirror/store    # CAS commit
```

The pipeline IS substrate-vocabulary. Every step is a typed action or
bilateral in `@io/git`. No new substrate primitives are introduced;
`@spectral/garden/git`'s contribution is the COMPOSITION + the
`garden` block surface syntax.

### 4.2 The two resolution modes

**Pinned mode (preferred).** Garden entry includes an explicit `@<ref>`
that is a commit hash. Resolution is deterministic and offline-cacheable
after first clone. `mirror.lock` records the lifted `oid`; subsequent
resolutions verify the oid is in `@mirror/store` and skip the clone
(early-cutoff per §3.2).

```mirror
source ~git'https://example.com/foo.git@abc123def...'    # SHA1 pinned
source ~git'https://example.com/foo.git@sha256:def456...' # SHA256 pinned
```

**Floating mode.** Garden entry uses a symbolic ref (`main`, `v0.1.0`,
branch name). Resolution is non-deterministic at first run; mosaic
resolves to the current commit and pins the resolution in
`mirror.lock`. Subsequent runs use the pinned oid; explicit `mirror
update` re-resolves and rewrites the lock file.

```mirror
source ~git'https://example.com/foo.git@main'    # floating; pins to first-seen commit
source ~git'https://example.com/foo.git'         # absent ref; equiv to @HEAD
```

The semantic difference is locked in the SUBSTRATE: pinned entries
produce a `git_hash` syntactically equal to the @<ref> suffix;
floating entries produce a `git_hash` retrieved at resolve time. Both
go through `hash_to_oid` and settle into the same `mirror.lock`
format. Consumers cannot tell which mode was used after lock; the
distinction matters only at lock-generation altitude.

### 4.3 The `needs` extension on `target`

A `target` block may reference garden entries as inputs via the
`needs` field, alongside other targets:

```mirror
garden {
  source ~git'https://github.com/systemic-engineering/spectral.git@v0.1.0'
}

target binary {
  name     "mirror"
  altitude @code/rust
  emit     cargo
  needs    [spectral]    # references garden entry by inferred name
}
```

The garden entry's name (here: `spectral`, inferred from the repo
path's last segment) becomes a typed reference; mosaic walks the
DAG with garden entries as leaf nodes and targets as internal nodes.
Explicit naming via `source name ~git'...'` is forward-promised (§8
O2).

### 4.4 Determinism contract

LOAD-BEARING property of the resolution pipeline:

```
for any (mirror.spec S, mirror.lock L):
  settle(S, L) produces shard with oid O
  and re-settle(S, L) produces shard with oid O                  (idempotence)
  and settle(S', L) where S' ≡ S under mosaic normalization
    also produces oid O                                          (extensionality)
  and settle(S, L') where L' has different pins
    may produce a different oid O'                               (lock matters)
```

This is the Dhall semantic-hash discipline (§3.1) lifted to the
substrate altitude. The normalization quotient is taken over the
mirror AST AFTER mosaic settlement, not the surface text. Comment
ordering, whitespace, equivalent import orderings do not change the
oid; lock-file pin changes DO.

### 4.5 The lock-file shape

`mirror.lock` is itself a `.mirror` shard parsed by `@mirror/lock`
grammar (forward-promised; sibling of `@mirror/project`). Sketch:

```mirror
in @mirror/lock
in @spectral/garden/git

lock mirror.lock {
  generated_at  "2026-06-24T10:30:00Z"
  spec_oid      ~oid'1234...abcd'           # oid of mirror.spec at lock time

  garden_entry {
    name        "spectral"
    source      ~git'https://github.com/systemic-engineering/spectral.git'
    ref         "v0.1.0"
    git_hash    "deadbeef..."               # 40-hex SHA1 or 64-hex SHA256
    oid         ~oid'5678...ef01'           # lifted via hash_to_oid
    well_formed verdict                     # cached verdict at lock time
  }

  # ... one entry per garden source ...
}
```

The lock file is content-addressed by its own oid; the
`spec_oid` field cross-references the spec it was generated against;
re-generating the lock against the same spec + same upstream state
produces the same lock-file oid. The substrate's CAS forecloses
spurious lockfile churn (which has plagued every text-based
lockfile format from `package-lock.json` to `Cargo.lock`).

---

## 5. Composition with `@io/git` and `@io/oci`

### 5.1 `@io/git` is the realisation; `@spectral/garden/git` is the surface

Clean partition:

| Layer | Altitude | Role |
|---|---|---|
| `@spectral/garden/git` | `mirror.spec` surface | declarative package entries |
| `@io/git` | @io adapter altitude | typed actions + bilaterals over git protocol |
| libgit2 / gitoxide / shell-out | realisation boundary | byte-level execution |

`@spectral/garden/git`'s `garden { source ~git'...' }` parses to a
family of `@io/git` action invocations (per §4.1 pipeline). The
surface adds the declarative + Pack-typed layer; the @io adapter
does the work; the realisation discharges at the boundary. Three
altitudes, one substrate.

This is the SAME pattern as `target binary { emit cargo }` →
`@io/cargo` → cargo binary at the @io edge. `@spectral/garden/git` is
the peer of `target` at the spec altitude.

### 5.2 `@io/oci` as registry-side composition

`@io/oci` (2801478) ships compiled artifacts to OCI registries.
`@spectral/garden/git` resolves git-based packages. The two compose
at the distribution altitude:

- Source distribution: `garden { source ~git'...' }` resolves source
  via `@io/git`; mosaic settles the source under `@code/<lang>`
  altitude; the settled crystal is content-addressed by oid.
- Artifact distribution: the settled crystal can be lifted to an
  oci_artifact via `oid_to_digest` (`@io/oci` §3) and pushed to a
  registry via `push_oci`.
- Consumer pull: a downstream `garden { source ~oci'...' }`
  (`@spectral/garden/oci`, forward-promised §6.2) resolves the
  pre-built artifact from registry without re-cloning + re-settling.

The composition is scope-graded:
- `@spectral/garden/git`: source-altitude distribution (dev / CI / peer
  homes)
- `@spectral/garden/oci`: artifact-altitude distribution (prod /
  registry-distributed)

The SAME `mirror.spec` can declare BOTH:

```mirror
garden {
  source ~git'https://github.com/systemic-engineering/spectral.git@v0.1.0'
  source ~oci'ghcr.io/systemic-engineering/spectral@sha256:def...'
}
```

Mosaic resolves both, content-addresses both, verifies both via the
`@io/git` + `@io/oci` well_formed bilaterals, lifts both to oid via
the respective bridge actions. The substrate-vocabulary names the
DISTRIBUTION SCOPE; the package identity at oid altitude is unified.

### 5.3 The `oid_to_digest` / `hash_to_oid` bridge cycle

LOAD-BEARING composition. Per `@io/git` §3 + `@io/oci` §3:

```
git_hash --hash_to_oid()--> oid --oid_to_digest()--> oci_digest
```

The cycle CLOSES at content-addressing altitude (all three are
SHA256-of-something modulo the SHA1/SHA256 function-altitude split at
the git scope). The cycle OPENS at scoping altitude (git scope is
versioned object graph; oid scope is substrate bytes; OCI scope is
manifest + layer descriptors). This is the recognition #98 candidate
structural pattern.

For `@spectral/garden/git`: a garden entry's `git_hash` round-trips
through oid space cleanly when the package is consumed AS substrate
bytes (mirror shards). When the package is consumed AS an OCI
artifact (a compiled binary, a wasm module), the round-trip goes
`git_hash` → `oid` (the source shard) → build → new artifact oid →
`oci_digest` (different scope; the bridge is `oid_to_digest`).

The substrate names every crossing; consumers MUST NOT assume
blanket identity across the cycle (per `shards/io/git.mirror` §8
Hedge 3).

---

## 6. The four-root structure (git / oci / nix / store)

### 6.1 Scope grading of the package-manager family

The substrate's package-manager family has FOUR roots, one per
scoping discipline:

| Root | Hash function | Scope | Use case |
|---|---|---|---|
| `@mirror/store` | SHA256 (oid) | substrate content bytes | internal substrate dedup; intra-shard reference |
| `@spectral/garden/git` | SHA1 / SHA256 (git_hash) | versioned object graph | peer-home / dev / CI source-level distribution |
| `@spectral/garden/oci` | SHA256 (oci_digest) | manifest + layer descriptors | registry-distributed prod artifact distribution |
| `@spectral/garden/nix` | SHA256 (derivation_hash) | build input/output closure | hermetic build distribution |

Each root is admissible in `garden { ... }`:

```mirror
garden {
  source ~git'https://github.com/sys/foo.git@v0.1.0'       # @spectral/garden/git
  source ~oci'ghcr.io/sys/foo@sha256:abc...'                # @spectral/garden/oci
  source ~nix'github:sys/foo/v0.1.0'                        # @spectral/garden/nix
  source ~oid'1234...abcd'                                  # @mirror/store (intra-substrate)
}
```

The surface is unified (one `garden` block; one settlement pipeline);
the scope is graded (each entry resolves through its root's
`@io/<species>` adapter); the content-addressing is composed (all
four roots lift to oid via their respective bridge actions).

### 6.2 Forward-promised siblings

- `@spectral/garden/oci` — the OCI-registry-resolved sibling.
  Surface: `source ~oci'registry/repo@digest'`. Resolution pipeline:
  `oci_reachable` → `pull_oci` → `digest_matches_content` →
  `oid_to_digest` inverse (CAS lookup; SAME oid space). Mirror's
  shipping story for prod artifacts.
- `@spectral/garden/nix` — the Nix-resolved sibling. Surface:
  `source ~nix'<flake-ref>'`. Resolution pipeline: nix evaluate +
  derivation_hash compute + (forward-promised) `derivation_to_oid`
  bridge. The substrate's hermetic-build story for dev environments.
- Each garden-root substrate-decl lives at
  `shards/spectral/garden/<species>.mirror` (path-namespace property
  per `@epistemologic/pact/path_matches_namespace`).

### 6.3 The ouroboros at the four-root altitude

Per `shards/io/git.mirror` §5: substrate-decl describes A → Nix builds A
→ OCI packages A → git records A's provenance → registry stores A →
consumer pulls A → substrate consumes A → describes B depending on A
→ loop at altitude+1.

The `@spectral/garden/*` family-root is where this loop becomes
SURFACE substrate-decl. Consumers DECLARE which scope they're
pulling from; mosaic discharges through the corresponding root;
oid-altitude unification preserves identity across the loop.

This is what would make the four-root structure the FIFTH witness
to recognition #98 candidate — the package-manager family-root
GRAPH itself exhibits the cross-altitude composition primitive at a
NEW altitude (surface-level scoping vs internal addressing).
Promotion decision is Reed's; this spec flags the territory.

### 6.4 What `@spectral/garden` (without species) names

Forward-promised. The parent family-root for the four scope-graded
roots above. Likely shape:

```mirror
in @spectral/garden
in @io

prism @spectral/garden {
  focus garden
  project garden
  split garden
  shift garden
  settle garden
}

type garden_source = ref       # parametric over species
type garden_lock   = ref       # parametric over species

resolve(s: garden_source, p: perturbation) -> imperfect(ref, ref, ref) { \ }
```

Species shards (`shards/spectral/garden/git.mirror`,
`shards/spectral/garden/oci.mirror`, etc.) refine. The parent
provides the contract; the species discharge through their
`@io/<species>` adapter. This follows the @magic/@frame/@smarts
family-root pattern (per `shards/pack.mirror` §"Substrate-decl shape").


