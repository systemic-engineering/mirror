# Taut scout — @io family type-as-constructor precedent

**Author:** Taut (grep-first drift scout; read-only)
**Date:** 2026-07-15
**Directive:** Alex `/loop` — collapse until Seam-un-adjudicable
**Prompted by:** Seam composition-alignment audit
`docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
§12 A1 (SEAM-DEFERRED); Seam-verbatim recommendation §12 A1 last
paragraph ("log Position (b) as a pending-scout for Taut to grep
the composition graph across ALL landed @io species and count how
many action-arrows are narrative-only vs substrate-mechanical").

Reed commits as Taut after review. No shard authorship. No Rust.

---

## §0 Scout scope + method

### 0.1 Question inherited from Seam

At `shards/io/secrets.mirror:156`, the composition-chain narrative
reads:

```
peer_key := @io/secrets.key_material_ref(peer)                # this shard
```

`key_material_ref` is a TYPE (declared at secrets.mirror:258 as a
two-field record `{ peer_ref, key_carrier }`). No landed action of
that name exists in @io/secrets; the exported actions are
`project`, `materialize`, `retrieve`, `round_trip`. The prose uses
the type-name-with-parens form as if it were a per-realisation
constructor factory.

Seam refused to pick between two positions at Seam altitude:

- **(a) Narrative-shorthand admissible** under
  `[[feedback-craft-not-deliver]]` — per-realisation construction
  is the consumer's job; substrate names the carrier + the actions
  that produce/consume it via `imperfect<carrier, ...>`; no
  constructor action needed.
- **(b) Petri-net completeness demands substrate-decl'd
  constructor** — every arrow in the composition graph must be
  substrate-mechanical for Rice-safe analysis; `@io/secrets` should
  carry `key_material_ref_of(peer: ref) -> imperfect<key_material_
  ref, ...>`.

Seam recommendation: two-tick discipline; ship (a) now; Taut grep
the composition graph across all landed @io species and count.
The precedent-of-precedent answers the (a) vs (b) question.

### 0.2 Ground-truth artifacts

Ten shards in the @io family, spanning 2026-06-05 (@io/cargo, first
landed species) → 2026-07-15 (@io/secrets + @io/fs + @io/crypto +
@io/secrets/sops, landed today):

| # | Shard | Landed | Size |
|---|-------|--------|------|
| 0 | `shards/io.mirror` (family root) | 2026-06-08 (T21) | 24.2 KB |
| 1 | `shards/io/cargo.mirror` | 2026-06-05 (first species) | 8.0 KB |
| 2 | `shards/io/stagefreight.mirror` | 2026-06-22 | 19.6 KB |
| 3 | `shards/io/oci.mirror` | 2026-06-24 | 25.6 KB |
| 4 | `shards/io/git.mirror` | 2026-06-24 | 24.1 KB |
| 5 | `shards/io/algebra.mirror` | 2026-06-30 | 40.8 KB |
| 6 | `shards/io/crypto.mirror` | 2026-07-15 (lift-tick companion) | 21.7 KB |
| 7 | `shards/io/fs.mirror` | 2026-07-15 (lift-tick companion) | 15.8 KB |
| 8 | `shards/io/secrets.mirror` | 2026-07-15 (A1 site) | 22.5 KB |
| 9 | `shards/io/secrets/sops.mirror` | 2026-07-15 (sub-species) | 18.4 KB |

### 0.3 Grep patterns

Three orthogonal grep passes:

- **P1 — narrative-shorthand-constructor:** in prose docblocks
  (`# ...` lines), match `@io/<species>.<type_name>(` where
  `<type_name>` is a declared type carrier (not an exported action).
- **P2 — substrate-decl'd carrier-constructor action:** in action
  signatures, match landed actions with typed return of the shape
  `<subject>(<input>) -> <carrier>` OR the naming shapes
  `<carrier>_of(...)`, `<subject>_to_<carrier>(...)`,
  `<carrier>_from_<input>(...)`, `<carrier>_for(...)`, or a
  parametric constructor like `ssh_key_material(...) -> key_material`.
- **P3 — accessor / projector actions:** landed actions of shape
  `<field>_of(<carrier>) -> <field_type>` where the action extracts
  a substrate-visible field from an input carrier (e.g. `hash_of`,
  `label_of`, `digest_of`, `payload_of`); these are landed accessors
  in `shards/labeled.mirror` (the `labeled<>` functor's
  value-side/label-side projectors) referenced in @io/git and
  @io/oci prose.

Criteria for classification:

- A **narrative-shorthand-constructor** MUST appear in prose
  (comment lines starting with `#`), reference a declared type as
  if calling it as a function, AND have NO substrate-decl'd action
  of the same name in the same shard.
- A **substrate-decl'd carrier-constructor** MUST be a landed
  action (non-comment line) whose return type is a carrier declared
  in the same shard OR its family root.
- Accessors on carriers (P3) are OUT-OF-SCOPE for the (a) vs (b)
  question — they extract from an existing carrier, not construct
  a new one.

---

## §1 Per-shard findings

### 1.1 `shards/io.mirror` (family root)

- **P1 (narrative-shorthand):** 0 instances.
- **P2 (substrate-decl'd constructor):** N/A — the family root
  declares the five-op `prism @io {...}` block; species carry the
  concrete constructors.
- **Observation:** The family root establishes discipline; the
  species-per-file pattern (one shard per species) means every
  concrete constructor lives in a species shard, not the root.

### 1.2 `shards/io/cargo.mirror` (2026-06-05; first species)

- **P1 (narrative-shorthand):** 0 instances of `@io/cargo.<type>(`
  in prose. One prose reference to `env_of(resolved)` at line 93
  (Admitted keys docblock) and line 34 (docblock note), but
  `env_of` is a MOSAIC action, not a cargo type-constructor
  shorthand.
- **P2 (substrate-decl'd constructor):** 1 landed action —
  `cargo_exit_to_transparency(code: exit_code, stderr: ~f) ->
  transparency` (line 129). Constructs `transparency` carrier from
  exit code + stderr. Named per `<subject>_to_<carrier>` pattern.
- **Additional landed action shape:** `env_allowlist(key: text) ->
  verdict` (line 107) — a predicate, not a constructor.
- **Type carriers declared:** `manifest` (type alias `~f`),
  `profile`, `exit_code`, `env`. Each is used as a PARAMETER type
  in action signatures; the CONSUMER constructs values via record
  literal / mosaic emission (line 34 docblock: "`env_of(resolved)`
  (mosaic.mirror line 126) constructs it").

Substrate-honest pattern in cargo: **types are declared, cargo
does not construct them; MOSAIC constructs them via `env_of`.**
Constructor location is another shard; cargo consumes.

### 1.3 `shards/io/stagefreight.mirror` (2026-06-22)

- **P1 (narrative-shorthand):** 0 instances of
  `@io/stagefreight.<type>(` in prose.
- **P2 (substrate-decl'd constructor):** 2 landed actions —
  - `address(crystal_oid: ref) -> spectral_coordinate` (line 306).
    Explicitly named in docblock (line 302-303): **"`address()` is
    the construction; `address_well_formed()` is the verification."**
  - `freight(crystal_oid, coord, projection, c, promise, p) ->
    freight_manifest` (line 333). Constructs the load-bearing
    `freight_manifest` carrier from six inputs.
- **Companion predicate:** `address_well_formed(coord, crystal_oid)`
  (line 356) — the C2 Seam-tick-68 closure that lifted the address
  derivation rules from spec-prose to substrate-decl.
- **Substrate-decl'd constructor rate:** 2/2 load-bearing carriers
  (`spectral_coordinate`, `freight_manifest`) have landed
  constructors.

Substrate-honest pattern in stagefreight: **every load-bearing
carrier has a landed constructor action.** The docblock explicitly
splits construction from verification.

### 1.4 `shards/io/oci.mirror` (2026-06-24)

- **P1 (narrative-shorthand):** 0 instances of `@io/oci.<type>(`
  in prose. Prose uses `digest_of(artifact)` and `payload_of(artifact)`
  (line 393), but these are `labeled<>` FUNCTOR PROJECTORS from
  `shards/labeled.mirror` (per line 414 verbatim: **"uses labeled<>
  projection per Mara A3 surprise (labeled functor's value-side
  accessor)"**) — accessors, not type-as-constructor shorthand.
- **P2 (substrate-decl'd constructor):** 3 landed actions —
  - `oid_to_digest(o: ref, p: perturbation) -> oci_digest`
    (line 308). Named per `<subject>_to_<carrier>` pattern.
  - `spectral_coordinate_to_repo(sc: ref, p: perturbation) ->
    oci_repository` (line 323). Same naming pattern.
  - `manifest_for(artifact: ref, p: perturbation) -> oci_manifest`
    (line 340). Named per `<carrier>_for` pattern.
- **Substrate-decl'd constructor rate:** 3/6 declared carriers
  (`oci_digest`, `oci_repository`, `oci_manifest`) have landed
  constructors. Two carriers (`oci_reference`, `oci_registry`) have
  no constructor because they arise from consumer-side URL parsing;
  `oci_artifact` uses the `labeled<>` functor.

Substrate-honest pattern in oci: **three landed cross-altitude
bridge constructors** (oid→digest, coord→repo, artifact→manifest).
Each converts a substrate input into a landed carrier. Zero
narrative-shorthand.

### 1.5 `shards/io/git.mirror` (2026-06-24)

- **P1 (narrative-shorthand):** 0 instances of `@io/git.<type>(`
  in prose. Prose uses `hash_of(artifact)` and `label_of(artifact)`
  (line 413, 416) — the same `labeled<>` FUNCTOR PROJECTORS as oci
  (verbatim: **"uses labeled<> projection ... value-side accessor"**).
  Accessor, not constructor shorthand.
- **P2 (substrate-decl'd constructor):** 1 landed action —
  `hash_to_oid(h: git_hash, p: perturbation) -> ref` (line 356).
  Named per `<subject>_to_<carrier>` pattern; this is the
  recognition-#98-candidate bridge constructor that lifts SHA1/SHA256
  git hashes into mirror's oid space.
- **Substrate-decl'd predicates (non-constructor bilaterals):**
  `git_reachable`, `git_repository_open`, `hash_well_formed`,
  `ref_well_formed` — verify carriers, do not construct them.

Substrate-honest pattern in git: **one landed cross-altitude bridge
constructor** (git_hash → mirror oid). Zero narrative-shorthand.

### 1.6 `shards/io/algebra.mirror` (2026-06-30)

- **P1 (narrative-shorthand):** 0 instances of `@io/algebra.<type>(`
  in prose. Prose extensively discusses `@fate.roll(restriction,
  hole)` and `@algebra.morphism_between` — these reference actions
  in OTHER shards, not type-as-constructor shorthand within algebra.
- **P2 (substrate-decl'd constructor):** 3 landed actions —
  - `expose(a: algebra_carrier) -> imperfect<io_algebra_exposure, ...>`
    (line 515). Constructs `io_algebra_exposure` from
    `algebra_carrier`. Named per verb-form (not `_to_`, but
    identical shape: input carrier → output carrier).
  - `consume(endpoint: algebra_endpoint) -> imperfect<algebra_carrier, ...>`
    (line 557). Inverse constructor: `algebra_endpoint` → `algebra_carrier`.
  - `address(e: io_algebra_exposure) -> oid` (line 581) — extracts
    the content-addressed oid from an exposure (an ACCESSOR-shaped
    constructor; substrate-pull-trivial per docblock line 572-575
    but declared explicitly for downstream typed entry point).
- **Additional action:** `translate(payload, source, target) ->
  imperfect<algebra_payload, ...>` — transforms payloads across
  algebras (not a constructor of a new carrier type; identity
  transformation with morphism composition).

Substrate-honest pattern in algebra: **expose + consume as
paired constructors** for the boundary crossing; `address` as
substrate-pull-trivial accessor lifted to explicit typed entry
point per docblock ("declaring the action explicitly gives
downstream prisms a typed entry point that doesn't require
unpacking the carrier"). Zero narrative-shorthand.

### 1.7 `shards/io/crypto.mirror` (2026-07-15; lift-tick companion)

- **P1 (narrative-shorthand):** 0 instances of `@io/crypto.<type>(`
  in prose.
- **P2 (substrate-decl'd constructor):** 1 landed action —
  `ssh_key_material(ssh_key_ref: ref) -> imperfect<key_material, ...>`
  (line 448). Constructs the `key_material` carrier from an
  ssh-key-ref (per docblock line 430-436: "Load ssh-key vendor
  material (openssh-format public/private keys) as key_material
  carrier. Per io.mirror 161-162 verbatim ... + Alex 2026-07-14
  SSH-signing design intent").
- **Additional constructor-shaped actions:** `sha1(bytes) -> ref`,
  `sha256(bytes) -> ref`, `ed25519_sign(key, msg) -> signature_bytes`,
  `aead_seal(ctx, plaintext) -> imperfect<ciphertext_bytes, ...>`,
  `age_encrypt(recipients, plaintext) -> imperfect<ciphertext_bytes, ...>`.
  Each PRODUCES a typed carrier from typed inputs; all landed as
  substrate-decl actions.

Substrate-honest pattern in crypto: **every carrier-producing
operation is a landed substrate-decl action.** The KEY LOAD-BEARING
FINDING for A1: `key_material` (the parametrically-parallel carrier
to `key_material_ref`) HAS its substrate-decl'd constructor —
`ssh_key_material(ssh_key_ref) -> imperfect<key_material, ...>`.
Direct precedent for A1 Position (b).

### 1.8 `shards/io/fs.mirror` (2026-07-15; lift-tick companion)

- **P1 (narrative-shorthand):** 0 instances of `@io/fs.<type>(`
  in prose.
- **P2 (substrate-decl'd constructor):** 0 landed constructors for
  the three carriers (`path`, `file_metadata`, `dir_entry`).
- **Constructor pattern:** Types arise on the OPERATIONAL SIDE —
  `fs_stat(p) -> imperfect<file_metadata, ...>` (line 293) returns
  the carrier; `fs_list(p) -> imperfect<[dir_entry], ...>` returns
  the carrier list. `path` is CONSUMER-CONSTRUCTED via record
  literal at the realisation boundary (consumer provides
  `{ segments, is_absolute }`).
- **Substrate-decl'd predicates:** `path_admissible`, `path_exists`,
  `writable`, `fs_well_formed` — verify carriers, do not construct.

Substrate-honest pattern in fs: **carriers arise from operations
(`fs_stat`, `fs_list`) OR from consumer record-literal construction
at the realisation boundary.** The `path` type has NO landed
`path_from_string(...)` or `path_of(...)` constructor; consumers
build the record directly. This is a SECOND admissible pattern
alongside crypto's landed-constructor pattern.

### 1.9 `shards/io/secrets.mirror` (2026-07-15; the A1 site)

- **P1 (narrative-shorthand):** **1 instance — the A1 site itself:**
  ```
  peer_key := @io/secrets.key_material_ref(peer)                # this shard
  ```
  at line 156. Prose composition-chain in the "Composition graph"
  docblock. No other instances of `@io/secrets.<type>(` in prose.
- **P2 (substrate-decl'd constructor):** 0 landed constructors for
  the three carriers (`key_material_ref`, `ciphertext_ref`,
  `secret_projection`).
- **Constructor pattern:**
  - `secret_projection` arises OPERATIONALLY from
    `project(section, k) -> imperfect<secret_projection, ...>`
    (line 366) — HAS a landed constructor action.
  - `ciphertext_ref` — no landed constructor; documented as arising
    from `retrieve` inverse plus consumer assembly.
  - `key_material_ref` — **no landed constructor**; the A1 site
    reads as if `@io/secrets.key_material_ref(peer)` were the
    constructor.

The A1 asymmetry: **`secret_projection` has `project()`;
`key_material_ref` has no equivalent.** The composition-chain
narrative treats them SYMMETRICALLY (both as
`@io/secrets.<carrier>(...)` calls), but the substrate-decl'd
surface is ASYMMETRIC.

### 1.10 `shards/io/secrets/sops.mirror` (2026-07-15; sub-species)

- **P1 (narrative-shorthand):** 0 instances of `@io/secrets/sops.<type>(`
  in prose.
- **P2 (substrate-decl'd constructor):** 2 landed actions —
  - `sops_encrypt(section, key_group) -> imperfect<sops_file_ref, ...>`
    (line 297). Constructs `sops_file_ref` from section + key_group.
  - `sops_key_group_from_sheaf_restriction(sr: sheaf_restriction) ->
    imperfect<sops_key_group, ...>` (line 349). **LOAD-BEARING
    PRECEDENT:** named per `<carrier>_from_<input>` pattern —
    the exact naming shape Seam's Position (b) proposed
    (`key_material_ref_of(peer)` uses the parallel `<carrier>_of`
    shape). Docblock (line 321-329) verbatim: **"THE composition
    bridge action. Given a @subject/visibility/sheaf.sheaf_restriction,
    project its peer_ref → ACL binding into a sops_key_group ..."**
- **Additional actions:** `sops_decrypt`, `sops_round_trip` — verb
  form; verification-side or full-round-trip discipline actions.

Substrate-honest pattern in sops: **the composition bridge from
@subject/visibility/sheaf into @io/secrets/sops IS substrate-decl'd
as a landed action** (`sops_key_group_from_sheaf_restriction`),
NOT as narrative-shorthand. Direct precedent for A1 Position (b) —
sops.mirror already landed the EXACT pattern proposed for secrets.mirror
one altitude up.

---

## §2 Ratio + pattern-of-pattern

### 2.1 Aggregate counts

Across nine species shards (family root excluded per §1.1):

| Shard | P1 narrative-shorthand | P2 substrate-decl'd constructor |
|-------|-----------------------:|--------------------------------:|
| cargo | 0 | 1 (`cargo_exit_to_transparency`) |
| stagefreight | 0 | 2 (`address`, `freight`) |
| oci | 0 | 3 (`oid_to_digest`, `spectral_coordinate_to_repo`, `manifest_for`) |
| git | 0 | 1 (`hash_to_oid`) |
| algebra | 0 | 3 (`expose`, `consume`, `address`) |
| crypto | 0 | 1+ (`ssh_key_material` + 5 typed-return crypto ops) |
| fs | 0 | 0 (consumer-side record literal + operational arise) |
| secrets | **1 (A1 site)** | 0 for `key_material_ref`; 1 for `secret_projection` (`project`) |
| secrets/sops | 0 | 2 (`sops_encrypt`, `sops_key_group_from_sheaf_restriction`) |
| **TOTAL** | **1** | **13+** |

### 2.2 Pattern-of-pattern

**Narrative-shorthand-constructor is NOT the dominant pattern
across the @io family. It is EXACTLY ONE INSTANCE, at the A1
site.**

Every other landed @io species (eight of nine) uses one of two
substrate-honest patterns:

- **Pattern L (LANDED-CONSTRUCTOR):** every load-bearing carrier
  has a substrate-decl'd action producing it — cargo, stagefreight,
  oci, git, algebra, crypto, secrets/sops (**seven of eight
  species**). Naming shapes attested:
  - `<subject>_to_<carrier>`: `cargo_exit_to_transparency`,
    `oid_to_digest`, `spectral_coordinate_to_repo`, `hash_to_oid`
  - `<carrier>_for`: `manifest_for`
  - `<carrier>_from_<input>`: `sops_key_group_from_sheaf_restriction`
  - Plain verb form producing carrier: `address`, `freight`,
    `expose`, `consume`, `sops_encrypt`, `project`, `ssh_key_material`
- **Pattern C (CONSUMER-LITERAL):** small typed records where the
  consumer builds via record literal at the realisation boundary —
  fs.mirror's `path` (`{ segments, is_absolute }`). **One of eight
  species.**

The 1/14 ratio (narrative-shorthand instances / total
carrier-production sites in @io family) empirically places
narrative-shorthand OUTSIDE the substrate's established @io pattern.
Seam's own recommendation phrasing ("if the count is high, say
>20%") sets the threshold. The measured rate is ~7% of
carrier-production sites, but ALL of that 7% concentrates at ONE
SITE in ONE SHARD — every other species is at 0%. The distribution
matters more than the mean: eight of nine species have zero
narrative-shorthand.

### 2.3 The empirical default

**Substrate-honest carrier production in @io = landed action with
typed return, OR consumer record literal at realisation boundary.**

Narrative-shorthand-constructor is not "the substrate's established
pattern" — it is a NOVELTY at the A1 site, without precedent in
the eight prior landed @io species.

---

## §3 A1 site-specific evidence

### 3.1 The composition-chain narrative structure

`secrets.mirror:149-158` verbatim:

```
# The Arc-2.3 peer_persistence.rs collapse chain (per shape-
# proposal §2.4 verbatim, corrected for interior-chain qualifier):
#
#   peer_visibility_materialize(peer, home, crystal, target_path):
#     ACL_peer := pack.members[peer.name]                          # existing
#     sr := @subject/visibility/sheaf.acl_project(F_home, ACL_peer) # d1ce901
#     section := @subject/visibility/sheaf.section_at(sr, crystal)  # d1ce901
#     peer_key := @io/secrets.key_material_ref(peer)                # this shard
#     sp := @io/secrets.project(section, peer_key)                  # this shard
#     @io/secrets.materialize(sp, target_path)                      # this shard → disk
```

Five arrows in the composition chain. Four resolve to landed
actions in named shards:

- `@subject/visibility/sheaf.acl_project` → landed (d1ce901)
- `@subject/visibility/sheaf.section_at` → landed (d1ce901)
- `@io/secrets.project` → landed (this shard, line 366)
- `@io/secrets.materialize` → landed (this shard, line 385)

One arrow does NOT resolve to a landed action:

- `@io/secrets.key_material_ref(peer)` — reads as call; no landed
  action exists; `key_material_ref` is the type declared at line 258.

The composition-chain notation is byte-symmetric across all five
arrows. Four of five carry substrate-decl'd backing; one does not.

### 3.2 The sibling asymmetry

Within `secrets.mirror` itself, the carriers split:

- `secret_projection` — HAS the landed constructor `project(section,
  k) -> imperfect<secret_projection, ...>`. Composition-chain arrow
  4 `sp := @io/secrets.project(section, peer_key)` resolves cleanly.
- `ciphertext_ref` — no landed constructor; used as a PARAMETER of
  `retrieve(cr, k)` (line 405). Composition-chain does not
  construct it; consumer would assemble.
- `key_material_ref` — no landed constructor; **the A1 site treats
  it as if the type itself is callable.**

`secret_projection` and `key_material_ref` are STRUCTURALLY
PARALLEL (both are two/four-field records with byte-equality
identity contracts, both are inputs to downstream @io/secrets
actions). One has a landed constructor; one does not. The narrative
treats them symmetrically; the substrate-decl surface does not.

### 3.3 Sibling shard precedent: crypto.mirror

The parametrically-parallel carrier in the sibling shard is
`key_material` (crypto.mirror:207, six-field record for a crypto
key). Its constructor: `ssh_key_material(ssh_key_ref: ref) ->
imperfect<key_material, ...>` (crypto.mirror:448).

Docblock explicit (crypto.mirror:430-436):

> Load ssh-key vendor material (openssh-format public/private
> keys) as key_material carrier. Per io.mirror 161-162 verbatim
> ("ssh-key" named as vendor surface) + Alex 2026-07-14 SSH-
> signing design intent (peer keys are ssh-key vendor material
> stored .git/mirror-side).

crypto.mirror ALREADY LANDED THE PATTERN. `key_material` (a
carrier) is constructed via a substrate-decl'd action
(`ssh_key_material`) that takes an opaque ref and returns
`imperfect<key_material, ...>`. This is EXACTLY the shape Seam's
Position (b) proposes for `key_material_ref_of(peer)` one altitude
up in @io/secrets.

### 3.4 Sibling shard precedent: secrets/sops.mirror

The sub-species landed alongside secrets.mirror as Landing 5 of
the same @secrets shape-proposal sequence. It carries
`sops_key_group_from_sheaf_restriction(sr: sheaf_restriction) ->
imperfect<sops_key_group, ...>` (sops.mirror:349) as a LANDED
COMPOSITION BRIDGE ACTION.

Docblock explicit (sops.mirror:321-333):

> THE composition bridge action. Given a @subject/visibility/
> sheaf.sheaf_restriction, project its peer_ref → ACL binding
> into a sops_key_group: enumerate the peers admitted by the ACL,
> resolve each peer's key material to its age/pgp/kms recipient
> form, assemble the sops_key_group.
>
> THIS IS THE ARROW that connects @subject/visibility/sheaf (ACL
> structure) to @io/secrets/sops (SOPS recipient list).

**sops.mirror already lands the EXACT naming shape**
(`<carrier>_from_<input>`) Seam's Position (b) proposes for secrets.
The sub-species discharges its composition-bridge arrow as a
substrate-decl'd action; the parent species does not. Landed on
the same day, in the same landing sequence.

### 3.5 Symmetry check across the Alex-verbatim origin

The Mara shape-proposal at `docs/scouts/2026-07-15-mara-secrets-
shape-proposal.md:245` has the same narrative form:

```
peer_key := @io/secrets.key_material_ref(peer, .git/mirror)   # NEW
```

(Note the two-arg form in the proposal; the landed shard collapsed
to one-arg because `.git/mirror` is implicit per @io opacity.) The
proposal marked it `# NEW` — signaling it is not yet substrate-
mechanical. The landing carried the narrative through without
promoting the arrow to a landed action, while sibling arrows
(`project`, `materialize`) were landed as actions.

The narrative-shorthand at the A1 site is a RESIDUE of the
proposal's exploratory notation, not a considered substrate-honest
pattern. The proposal itself does not defend
narrative-shorthand-constructor as the correct pattern; it uses the
notation to sketch the collapse chain before decisions about which
arrows land as substrate-decl actions.

---

## §4 Recommendation to Seam

### 4.1 Which position does empirical precedent favor

**Position (b) — petri-net completeness demands substrate-decl'd
constructor.**

Grounds:

- **G1 — Pattern-of-pattern (§2).** 1/14 sites use
  narrative-shorthand; 13/14 use landed constructor OR consumer
  record literal. Eight of nine landed @io species have zero
  narrative-shorthand-constructor instances. The A1 site is a
  singleton; the substrate does not have a "narrative-shorthand is
  fine" pattern across @io.
- **G2 — Sibling-shard direct precedent (§3.3).** crypto.mirror
  landed `ssh_key_material(ssh_key_ref) -> imperfect<key_material,
  ...>` on the SAME DAY as the A1 site, for the parametrically-
  parallel carrier. If crypto's key_material warrants a landed
  constructor, secrets' key_material_ref warrants one at parallel
  altitude by symmetry.
- **G3 — Sub-species direct precedent (§3.4).**
  secrets/sops.mirror landed `sops_key_group_from_sheaf_restriction`
  as a substrate-decl'd composition-bridge action, using the EXACT
  naming shape (`<carrier>_from_<input>`) that Position (b)
  proposes for secrets. If the sub-species carries the composition
  bridge as landed action, the parent species SHOULD carry its
  composition bridges as landed actions — otherwise the sub-species
  is more substrate-honest than the parent.
- **G4 — Intra-shard asymmetry (§3.2).** Within secrets.mirror,
  three parallel carriers get asymmetric treatment:
  `secret_projection` has `project()`; `key_material_ref` has
  narrative-shorthand; `ciphertext_ref` has neither. If the arc-
  2.3 collapse chain requires arrow-4 as substrate-mechanical, the
  fix is a landed constructor per crypto's precedent; if it does
  not, then `project`, `materialize`, `retrieve`, `round_trip`
  don't need to be landed actions either — but they ARE, so the
  asymmetry has no principled justification.

### 4.2 The two-tick discipline is honored

Seam's recommendation (audit §12 A1 last paragraph) was:

> two-tick discipline — land Position (a) for this ship (narrative
> shorthand admitted); log Position (b) as a pending-scout for Taut
> to grep the composition graph across ALL landed @io species

Two-tick discipline REQUIRES that if the second tick (b) supersedes
the first tick (a), the second tick lands cleanly with substrate-
pull. The empirical precedent (§2, §3) supports the second tick
because Position (b) is the substrate's established @io pattern;
Position (a) is a singleton exception.

Precedent-of-precedent verdict: **land Position (b) as a follow-up
Landing 6 to the @secrets shape-proposal sequence.** Proposed
action-signature (one candidate; Seam / Alex may refine the name
per @io/secrets naming discipline):

```
# === key_material_ref_of action ===
#
# THE substrate-decl'd composition-bridge action for peer → key
# material ref. Given a peer_ref, construct the key_material_ref
# carrier by resolving the peer's ssh key material through @io/
# git plumbing + @io/crypto.ssh_key_material.
#
# Per crypto.mirror precedent (ssh_key_material) + sops.mirror
# precedent (sops_key_group_from_sheaf_restriction): every
# composition-bridge arrow at @io altitude discharges as a
# landed action, not narrative-shorthand.
key_material_ref_of(peer: ref) -> imperfect { \ }
```

Body remains `\`-obligation-blocked per
`[[feedback-craft-not-deliver]]`; the substrate-decl surface names
the arrow; realisation discharges. This preserves BOTH
`[[feedback-craft-not-deliver]]` (body not landed) AND
`[[feedback-no-rust-extension-shortcut]]` (no Rust; composes over
`@io/crypto.ssh_key_material` at realisation).

The A1 site prose at `secrets.mirror:156` can then be updated:

```
peer_key := @io/secrets.key_material_ref_of(peer)             # this shard
```

Or preserved as-is if Mara/Seam judges the type-name-with-parens
notation admissible AS Alex-verbatim design intent, PROVIDED the
underlying action lands. The substrate-mechanical vs prose
distinction is orthogonal to the narrative variable naming.

### 4.3 Re-adjudicability by Seam spawn

The evidence in §2 + §3 is grep-first, empirically counted, and
substrate-honest per `[[feedback-substrate-already-had-the-word]]`
(the pattern was already landed in eight prior species). A Seam
spawn with this scout as ground truth CAN re-adjudicate A1:

- Position (a) admissibility rests on
  `[[feedback-craft-not-deliver]]`. This scout does NOT contest
  that principle; per-realisation body-construction IS admissible.
  What this scout shows is that per-realisation *arrow discharge*
  in composition chains has an established landed-action shape in
  the @io family (13+ instances). `[[feedback-craft-not-deliver]]`
  applies to BODIES; it does not require SURFACES to remain narrative.
- Position (b) petri-net completeness demand aligns with the
  substrate's own operational pattern across @io. The petri-net
  hole at A1 is not an abstract concern — it is a shape that eight
  prior species already resolved by landing the arrow as an action.

Seam can now re-adjudicate A1 with the precedent as ground truth.
The trade-off between `[[feedback-craft-not-deliver]]` (surface
minimalism) and `[[feedback-no-rust-extension-shortcut]]`
(analyzable substrate mechanics) is settled empirically by the
@io family's own landed choices: **surface is not minimized at the
cost of arrow-mechanics; every composition-bridge arrow lands as
an action, then the body remains obligation-blocked.**

---

## §5 Substrate-honest bounds

### 5.1 What this scout DOES decide

- What the empirical @io family pattern is for carrier construction
  (§2): landed constructor OR consumer record literal; not
  narrative-shorthand.
- Where sibling precedents live for the A1 case specifically (§3):
  crypto.mirror's `ssh_key_material` and sops.mirror's
  `sops_key_group_from_sheaf_restriction`.
- That Position (b) is the substrate-pull-correct default at @io
  species altitude per the family's own landed evidence (§4).

### 5.2 What this scout does NOT decide

- **Whether Alex adjudicates otherwise.** The Seam audit tagged A1
  as `Alex-adjudication residue`; this scout is precedent evidence
  Seam requested to inform a subsequent Seam spawn's
  re-adjudication. If Seam re-adjudication (informed by this
  scout) still cannot resolve, A1 remains Alex-only.
- **The exact name of the proposed action.** `key_material_ref_of`,
  `key_material_ref_from_peer`, `peer_key_material_ref` — Seam or
  Mara names it per substrate-decl naming discipline. This scout
  proposes ONE candidate per Position (b)'s original phrasing.
- **Whether the A1 site's narrative shorthand ALSO needs to change.**
  Once the action lands, the prose can either update to
  `key_material_ref_of(peer)` OR remain as-is (Alex-verbatim
  design-intent preservation). This is a Seam/Alex judgment about
  Alex-verbatim preservation vs symbol coherence.
- **Whether `ciphertext_ref` should also gain a landed constructor.**
  This scout observed the intra-shard asymmetry (§3.2) but scopes
  A1 to `key_material_ref` only per Seam's A1 phrasing.
- **Whether narrative-shorthand-constructor is FORBIDDEN in @io.**
  This scout only measures precedent, not admissibility floor.
  1/14 is not zero; there could be principled cases (e.g., the
  Alex-verbatim design-intent site is one). The measurement is a
  DEFAULT bias, not a HARD rule.
- **Whether the Rice-safe petri-net analyser matters at Arc-2.3
  altitude.** Position (b)'s underlying motivation
  (Rice-safe whole-tick petri-net analysis) has its own admissibility
  argument in `[[feedback-no-rust-extension-shortcut]]` and Mara-B
  §4.5.5; that argument is not adjudicated here, only cross-
  referenced.

### 5.3 If Seam re-adjudication (informed by this scout) still
cannot resolve

Then A1 IS Alex-only per Alex's `/loop` collapse directive. The
directive read: **"collapse until Seam-un-adjudicable"**. If the
precedent evidence in this scout DOES enable Seam re-adjudication,
collapse continues; if it does NOT, A1 sits at Alex-authority
altitude.

The scout's own judgment (Taut, grep-first, read-only): the
evidence IS sufficient for Seam re-adjudication because it
translates the abstract (a) vs (b) trade-off into an empirical
question about the @io family's established pattern, and the
family's pattern is measurable, counted, and cited by line number
across nine landed species.

---

## §6 Loop back

- Composite: (this scout single-file)
- Reed to review Taut's precedent as substrate-honest; commit as
  Taut; trigger Seam re-adjudication spawn with this scout as
  ground truth.
- Seam re-adjudication scope: re-open A1 in
  `docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
  §12; produce Landing 6 recommendation OR re-defer to Alex.

— Taut, 2026-07-15
