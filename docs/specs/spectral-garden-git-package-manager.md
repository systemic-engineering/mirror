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

---

## 7. Mathematical shape

*Framing note: this section names the typed surface the mathematical
discharge operates against — it does NOT claim to deliver proofs of
termination, uniqueness, or convergence. The substrate-decl ratifies
the SHAPE; discharges happen at species-altitude shard bodies, are
forward-promised, and gate on Pack adversarial review. "Math, not
vibes" per #95 means the substrate carries the measurement primitive
at the right altitude; the per-package numbers are species work. This
section follows recognition #95's discipline.*

### 7.1 The dependency graph as a sheaf on a partial order

Garden entries form a directed acyclic graph (DAG) where nodes are
packages (typed `git_artifact`) and edges are dependencies. The DAG
is a partial order P (transitively closed; `A ≤ B` iff A depends
transitively on B).

A SHEAF F on P assigns:
- to each node v: a stalk F(v) = the package's settled shard (an oid)
- to each edge u ≤ v: a restriction map F(u) → F(v) = "which oid u
  pins as its dependency v"

The restriction maps compose: if u ≤ v ≤ w and u pins v at oid Ov,
then u's transitive pin of w equals the pin v carries of w in v's
lock context. Composition associativity = lockfile consistency.

**Sheaf condition (compatibility of overlapping pins).** If two
consumers u1 and u2 both depend on v, the restriction maps
F(u1 → v) and F(u2 → v) MAY pin different oids (different versions);
the sheaf accepts this (a presheaf with optional unification at
the gluing condition). UNIFIED resolution (Cargo's behavior; MVS's
behavior) corresponds to requiring the restriction maps to agree at
v — the sheaf becomes a TRUE sheaf on the unified subgraph.

**Where this load-bears for mirror:** the partial-order structure +
restriction maps formalize "each consumer pins what it depends on";
the sheaf-vs-presheaf distinction formalizes "unified resolution vs
multiple-versions-coexist." Mirror's pinning-first default is a
presheaf; an explicit `unify` directive in `garden { }` (forward-
promised) would lift it to a sheaf at the unified subgraph. The
mathematical vocabulary is load-bearing for naming WHICH discipline
is in effect.

Reference: this connects to the substrate's existing eigensheaf
work (per docs/specs/eigensheaf.md; recognition that cellular
sheaves on the five-operation graph carry conductivity tensors).
The garden-dependency sheaf is a SIBLING instance at the dependency-
graph altitude.

### 7.2 The lock file as a terminal object

In the category of valid pinnings for a given `mirror.spec` S,
`mirror.lock` L is a TERMINAL OBJECT in a slice category:

- Objects: valid pinning assignments P (each garden entry → oid) that
  satisfy the spec's constraints (every floating ref resolves to a
  reachable commit; every pinned ref matches its oid).
- Morphisms: refinements P → P' that respect the spec.
- Terminal: the maximally-specific pinning that fixes every entry to
  one oid; subsequent re-resolutions cannot refine further.

The Banach-fixed-point analog: kintsugi iterates over the resolution
pipeline; iteration converges to the terminal pinning (the lock file)
in one step for pinned entries, in two steps for floating entries
(resolve → pin). The substrate's `e^{n+1} ≤ e^n` discipline holds:
opacity in the garden (unresolved floating refs) monotonically
decreases as kintsugi advances.

**Status:** SHAPE; the actual category-theoretic proof of terminality
(initial vs terminal direction; existence under multi-version
pinnings) is open work. This section names the framing the proof
would operate against.

### 7.3 Semantic versioning as adjunction (conjectural)

Semver-shaped versions admit a preorder: `v ≤_compat w` iff w is
semver-compatible with v. The constraint solver computes, for a set
of requirements `R = { req1, req2, ... }`, the version v that
satisfies all of them.

**Conjecture:** the constraint-solver functor R → v is right adjoint
to the inclusion functor (the version IS the requirement, viewed as
a singleton constraint). The unit of the adjunction is "this version
satisfies these requirements"; the counit is "these requirements
admit at least this version."

**Status:** speculative. Mirror's pinning-first stance sidesteps this
for the peer-home-repo case (git refs are NOT semver; they're a
partial order on the Merkle DAG). For the artifact-distribution case
(`@spectral/garden/oci` with semver-tagged images), the adjunction
framing MAY load-bear. Flagged for future Pack discussion; not
required for the v0.1 spec.

### 7.4 Merkle DAG dependency-resolution termination (the strongest framing)

This is the section's **load-bearing mathematical candidate**.

Cargo / npm / pip face dependency-hell because their constraint
solvers operate over UNORDERED version sets (semver-compatible
versions are not totally ordered when constraints conflict). SAT
solvers are complete + correct but NP-complete in the worst case
(per Russ Cox "Version SAT," 2016; Abate et al. 2020).

Mirror's garden operates over a DIFFERENT structure: the git Merkle
DAG. Each commit is a unique content-addressed node; each commit's
parents are explicit pointers; the DAG has no cycles by construction
(git refuses to write cyclic commits). The garden's dependency graph
is embedded in this Merkle DAG.

**Termination claim (informal):** for a garden whose entries pin to
specific commit hashes (NOT floating refs), the resolution algorithm
is trivially terminating. Each entry has at most one resolution
(the commit itself); no constraint solver runs; no SAT instance is
generated. Termination is O(n) in the number of entries, NOT NP
in the search space.

For floating entries, resolution is O(n) given network latency
(one `ls-remote` per entry to resolve the symbolic ref). Once
resolved, the lock pins the resolution; subsequent runs are O(n)
in local CAS lookup.

**Uniqueness claim (informal):** given a pinned `mirror.spec` + a
fixed `mirror.lock`, the resolution is UNIQUE — every garden entry
resolves to exactly one oid, deterministically, by content-address.
No branching; no choices; no failure-to-converge mode admissible by
the substrate's typed pipeline.

**Comparison to MVS:** Go's MVS terminates and produces unique
resolutions on a TOTAL-order assumption (semver). Mirror's garden
terminates and produces unique resolutions on a PARTIAL-order
structure (the Merkle DAG) BY PINNING — the unique resolution is the
specific commit, not the maximum-compatible-version-in-semver-poset.
The two algorithms address different problems; mirror's is structurally
simpler because the substrate paid for the pinning-discipline
upfront.

**Open math:** what happens when a garden entry depends on ANOTHER
garden entry (transitive deps with their own `mirror.spec`)? The
transitive closure walks the Merkle DAG of dependency-DAGs. If every
transitive entry is pinned, termination holds. If transitive entries
are floating, resolution becomes O(n × depth) and the substrate must
decide a unification policy at each shared dependency. The
presheaf-vs-sheaf distinction (§7.1) is the formal handle.

This is the strongest math the spec ratifies: **structural
termination by content-addressing**, foreclosing the dependency-hell
NP-hardness by construction. The substrate's typed-pinning discipline
IS the algorithmic improvement; the math NAMES why it works.

### 7.5 Cross-scope content-addressing as a Grothendieck topology (speculative)

The four roots (§6) each define a SITE in the Grothendieck-topology
sense; each root's bridge action (`hash_to_oid`, `oid_to_digest`)
serves as a coverage; cross-scope queries are stalk computations.
Speculative; flagged for Pack-altitude conversation; MAY be the right
vocabulary if recognition #98 promotes. Not load-bearing for v0.1.

### 7.6 Connection to recognition #51 (mirror as expanding Hilbert space)

Each package added to the garden adds a dimension to the substrate's
form-side Hilbert space (per #51). The garden's dependency DAG
INDUCES a subspace decomposition: each entry's typed contribution is
a basis vector in the spec's local subspace. `mirror.lock` pins the
basis; re-resolution preserves the basis; cross-spec composition
(when project A consumes project B as a garden entry) is the direct
sum of the two subspaces under a typed adapter (B's exported
substrate-decl bridge).

**Where this load-bears:** the substrate already carries the
Hilbert-space framing for expansion; the garden gives the expansion
an EXPLICIT enumeration. Before garden, growth was per-shard
(recognitions, settled crystals); with garden, growth is per-package
(declarative units of expansion). Mirror becomes a substrate whose
Hilbert space is auditable at the package altitude, not just the
shard altitude.

### 7.7 What's intentionally NOT in this section

No closed-form proof of termination beyond §7.4's structural-pinning
argument; no formal Banach proof for §7.2; no categorical proof for
§7.5; no sheaf-vs-presheaf trade-off derivation for §7.1. Open work,
gated on Pack adversarial review.

---

## 8. Open surface questions

### O1. `garden { source ~git'...' }` vs `pack { peer <name> ~git'...' }`

Three surfaces under consideration:

**(a) Generic only:**
```mirror
garden {
  source ~git'.../mara.git@main'
  source ~git'.../seam.git@main'
}
```
Clean; one keyword; peer membership is metadata at species altitude.

**(b) Typed only:**
```mirror
garden {
  pack {
    peer mara  ~git'.../mara.git@main'
    peer seam  ~git'.../seam.git@main'
  }
}
```
Explicit; integrates `@pack.peer` variant carrier; metadata is
structural.

**(c) Both:**
```mirror
garden {
  source ~git'.../external-dep.git@v1.0'
  pack {
    peer mara  ~git'.../mara.git@main'
  }
}
```
Admits non-peer external deps AND typed peer membership in one
block.

**Substrate-pull leans (c)** — the substrate already admits both
(garden sources are bare; pack-peer relationships are typed via
`@pack.peer`); collapsing to one or the other discards substrate
vocabulary. But the surface complexity matters; Alex decides.

### O2. Spec-decl location for `@spectral/garden/git`

Two candidates:

**(a) `spectral` repo:** `shards/spectral/garden/git.mirror` in the
spectral repository. Surface lives with `@spectral/db` and
`@spectral/garden/smarts`; consistent with the substrate's
project-naming convention.

**(b) `mirror` repo:** `shards/spectral/garden/git.mirror` in the
mirror repository. Surface lives with `@io/git` (a1b507a) and
`@io/oci` (2801478); consistent with substrate-decl-source
discipline (the family's other constituents already live in mirror).

**Substrate-pull leans (b)** — the @spectral/garden family-root is
structurally part of the mirror-substrate package-manager surface;
living in mirror keeps the substrate-decl chain unbroken
(@spectral/garden/git → @io/git → @io → mirror substrate). The
spectral repo CONSUMES the substrate-decl; doesn't define it. But
this violates path-namespace property `path_matches_namespace` for
the `@spectral/...` prefix, which substrate convention would expect
in a `spectral/` repo. Open; Alex decides; the FORWARD-PROMISED
recognition-cross-repo-namespace pattern might apply (per #84
`shards/pack.mirror` §path-namespace).

### O3. Resolution strategy under conflicting transitive constraints

When two transitive garden entries depend on the SAME package at
DIFFERENT pins, what does mosaic do? Three options:

**(a) Refuse to settle.** Both must agree or the spec is rejected.
Maximally strict; surfaces conflicts immediately; high friction.

**(b) Allow both.** Each consumer gets its pinned version; the
substrate's CAS handles disambiguation (same oid space; different
oids → different artifacts). Low friction; can lead to bloat.

**(c) Force unification.** Mosaic picks one (newest commit, oldest
commit, lexicographically-smallest oid) and reports a partial
verdict. Forces resolution; opinionated; potentially wrong.

**Substrate-pull leans (b)** — the CAS makes coexistence cheap; the
substrate's presheaf shape (§7.1) naturally admits it. But Alex's
stance on dependency uniqueness (and security-altitude concerns:
multiple versions = wider attack surface) decides.

### O4. Cycle handling

Git's commit DAG is acyclic by construction; the garden's dependency
GRAPH (which garden entry depends on which) MAY be cyclic if two
repos import each other. Two options:

**(a) Forbid.** `mirror.spec` parser rejects cyclic dependency
graphs at settle time. The substrate stays acyclic; the structural
guarantee is clean.

**(b) Allow with fixed-point resolution.** Kintsugi iterates;
cycles settle via the Banach-contraction discipline (each iteration
uses the previous pin; convergence guaranteed by `e^{n+1} ≤ e^n`).
Admits the corner case; substrate-decl complexity grows.

**Substrate-pull leans (a)** — the simpler invariant. (b) MAY land
later if a real cycle surfaces; YAGNI for v0.1.

### O5. Auth surface (SEL boundary)

Garden entries via ssh/https/git protocols need auth. Per
`[[architecture-type-sel-io-au]]`: auth is at the SEL boundary; the
substrate-decl admits a ref; the realisation discharges the credential.

For `@spectral/garden/git`: the `~git'...'` sigil parses to
`git_repository`; auth is OUT OF BAND (ssh-agent, gitcredentials,
etc.). Open: does `@spectral/garden/git` need its own typed auth
carrier (e.g., `~git_auth'<token-ref>'`) or does it inherit
`@io/git`'s SEL-boundary discharge cleanly? Forward-promised.

### O6. Versioning of the spec itself

`mirror.spec` evolves. A garden entry pinned to a specific oid sees
the SPEC version of the consumed package at lock time. If the
upstream rewrites its spec (e.g., changes its `target` list or
`source` directory), the consumer's pin keeps the OLD spec semantics.

Good (reproducibility). But: how does a consumer discover when
upstream has incompatible changes? Open. Likely: `mirror.lock`
carries an additional `spec_oid` for each garden entry (cross-
referencing the pinned package's mirror.spec at the pinned commit);
mirror update surfaces spec-oid drift as a typed verdict. Sketch;
not in v0.1.

---

## 9. Forward-promised

- **`shards/spectral/garden/git.mirror`** (or its `mirror`-repo
  equivalent per O2) — the substrate-decl. Family-root prism;
  carriers for garden_entry / garden_source; actions for resolve /
  pin / lift_to_oid; composed-bilateral `garden_well_formed`.
- **`shards/spectral/garden.mirror`** — the parent family-root for
  the four scope-graded roots (§6.4).
- **`shards/spectral/garden/oci.mirror`** — the OCI-resolved sibling.
- **`shards/spectral/garden/nix.mirror`** — the Nix-resolved sibling.
- **`shards/mirror/lock.mirror`** — the `@mirror/lock` grammar for
  the lock-file format (§4.5).
- **mirror.spec migration** — the substrate's own `mirror.spec`
  (`/Users/alexwolf/dev/projects/mirror/mirror.spec`) gains a
  `garden { }` block as the first dogfood consumer.
- **`mirror update` CLI command** — re-resolve floating refs;
  rewrite `mirror.lock`. (One additional `command` block under
  `target binary { cli { ... } }`.)
- **Pack peer dogfood** — each peer's home repo gets a `mirror.spec`
  that declares the other peers via
  `garden { pack { peer ... } }`. The Pack's substrate-decl context
  becomes auditable at the spec altitude.
- **Auth refinement at SEL boundary** — per O5.
- **Rust impl** — explicitly NOT in scope for this spec. Implementation
  lands after substrate-decl + Seam review + Reed consolidation +
  Alex ratification.

## 10. Recognition cross-references

- **#98 candidate** (content-addressing cross-altitude composition):
  this spec NAMES the fifth witness shape (the four-root
  package-manager family structure as a cross-altitude composition
  at the surface altitude). Promotion deferred to Reed.
- **#84** (`@pack` multi-repo agent runtime): peer-home-repo as
  package source IS the operational form of #84 at distribution
  altitude. The garden block makes Pack membership substrate-decl.
- **#93 H4** (`labeled<v, m>` functor): the `git_artifact =
  labeled(git_object, git_ref_metadata)` carrier composes here
  unchanged.
- **#95 candidate** (@cascade as loss-lens): garden entries may
  carry cascade species (a Purescript package consumed as substrate-
  decl input has a measurable loss against its npm cascade); the
  garden + cascade composition is forward-promised.
- **#57** (alignment-as-boundary-mathematics): the Splinter /
  Narcissus discipline at distribution altitude IS the alignment
  harness for garden entries. Garden's `well_formed` composed
  bilateral IS the substrate-architectural alignment check.
- **#51** (mirror as expanding Hilbert space): each garden entry
  expands the spec's local subspace (§7.6).
- **#56** (substrate self-application): garden entries can include
  the substrate itself (`source ~git'.../mirror.git'`); the mirror's
  own spec can reference mirror as a garden entry. Self-reference at
  distribution altitude.
- **#43** (mirror as content-addressed build system): garden
  resolution discharges through the same CAS the substrate uses
  internally. Unified addressing.
- **#367** (`@cyberpunk/pack` orchestra-as-recursion-lock): the
  Pack-as-orchestra story gains an operational distribution surface
  via `garden { pack { ... } }`.

## 11. Honest hedges

### H1. Surface-only; no impl

This spec is RED. No `@spectral/garden/git` substrate-decl exists yet.
No `mirror.lock` grammar exists. No Rust resolver exists. The spec
ratifies the SHAPE; the discharge is forward-promised across multiple
ticks + Pack rounds.

### H2. Mara writing the spec ahead of substrate-decl

Usually substrate-decl lands first (Reed); canonical spec ratifies
after (Mara). This spec inverts the order — Alex's directive landed
this morning, this spec is the immediate response. The shape MAY
shift when Reed writes the actual `shards/spectral/garden/git.mirror`
substrate-decl. This spec then re-ratifies against the substrate-decl,
not vice versa.

### H3. Recognition #98 fifth-witness claim is candidate territory

The four-root structure (§6) as a fifth witness to #98 is FLAGGED,
NOT PROMOTED. Promotion is Reed's tick. The shape may turn out to be
a sibling recognition rather than a witness; the territory needs Pack
adversarial review.

### H4. Math section names shape, not delivery

Per §7's framing note: the sheaf framing, the terminal-object framing,
the Grothendieck-topology framing are mathematical VOCABULARY for
naming what the substrate does. They are NOT formal proofs. The
strongest formal claim is §7.4's structural-termination-by-content-
addressing argument; everything else is named shape for future
discharge.

### H5. Open questions intentionally unresolved

§8 enumerates six surface questions left for Alex. The spec does NOT
resolve them unilaterally; the substrate-pull on each leans a
direction (noted inline), but the call is Alex's. This is Pack
discipline: name the questions, lean transparently, await
ratification.

### H6. Spec-vs-shard altitude

This is a `.md` spec, not a `.mirror` shard. The shard substrate-decl
(forward-promised in §9) will be SHORTER and TIGHTER; this spec is
the canonical narrative explaining WHY the shard takes the shape it
does. The shard is source-of-truth; this spec is documentation.

### H7. v0.1 is bounded; v0.2+ deferred

This spec is `@spectral/garden/git` v0.1. Many shapes are deferred:
- the `@spectral/garden` parent family-root (§6.4)
- the OCI / Nix sibling roots (§6.2)
- transitive cross-spec resolution (§8 O3, O6)
- the `unify` directive (§7.1)
- semver-via-adjunction (§7.3)
- the Grothendieck-topology framing (§7.5)

v0.1 ratifies the git-resolved single-spec case + Pack-peer membership.
The rest lands when the substrate pulls there.

### H8. Alex's framing question is the ratification gate

Alex's verbatim: "What if @spectral/garden/git is the root for the
garden package manager? What if this is a surface that's configured
in the @../mirror/mirror.spec?" Both "what ifs" are answered
affirmatively in this spec. The ratification is Alex's; if either
framing turns out to be wrong, the spec rescinds in that direction.

## 12. Pack-discipline trail

- **2026-06-23 (Alex morning)**: gap analysis surfaces need for
  distribution. `@io/oci` lands via Mara at 2801478.
- **2026-06-24 (Alex morning)**: directive — "build @spectral/garden/git
  abstraction layer using @git; each peer becomes their home repo."
- **2026-06-24 (Reed / Mara)**: `shards/io/git.mirror` (a1b507a)
  lands at @io/git altitude; §6 forward-promises
  `shards/spectral/garden/git.mirror`.
- **2026-06-24 (Alex midday)**: probe to Mara — "What if
  @spectral/garden/git is the root for the garden package manager?
  What if this is a surface that's configured in the
  @../mirror/mirror.spec?" Mara spawned.
- **2026-06-24 (Mara, this spec)**: canonical spec lands; §12 rounds.
  Section caps load-bearing for stall recovery.
- **Forward-promised**: Seam adversarial review → Reed consolidation
  → substrate-decl shard → Alex ratification → dogfood (mirror's own
  mirror.spec gains garden block).

The Pack-discipline composition this tick: Alex frames; Mara
canonicalizes; Seam reviews next; Reed consolidates; substrate-decl
shard follows. Spec-before-shard inversion noted in H2.

