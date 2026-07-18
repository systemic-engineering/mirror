---
title: The @peer/registry Species — Content-Addressed OID → Subject Resolution at the at_operator Dispatch Surface
subtitle: Companion spec to `docs/specs/2026-07-18-trust-family-root-passkey-ssh-bridge.md`. Species-decl under @peer for the runtime Subject registry. Discharges Reed `23cb7bb` at_operator @io/git.commit stub boundary. Resolves OID strings → fractal::Subject values via @trust chain composition. Well-known Subjects + current-peer resolution + registry evolution (SSH-side + passkey-side) + `peer_registered_at_trust_root` bilateral. Substrate-decl authorship territory (Mara); runtime dispatch (Reed) forward-promised.
status: canonical-spec
date: 2026-07-18
author: Mara
---

# @peer/registry — the OID → Subject Resolution Surface

> Reed `23cb7bb` at_operator @io/git.commit arm (verbatim):
> "Subject registry resolution NOT YET LANDED (author_oid=<..>, committer_oid=<..>). Mara
> @peer/registry species-decl + @trust family-root mint is the authorship territory that
> resolves OIDs → fractal::Subject values. Per Alex 2026-07-18 + passkey-spectral-bridge
> insight: this is direction B (OID + registry) with the compiler's SSH-signing chain as one
> altitude and the garden's passkey/PRF chain as the parallel altitude of the SAME @trust
> family-root."

This spec is the authorship territory Reed named. Read alongside
`docs/specs/2026-07-18-trust-family-root-passkey-ssh-bridge.md` (the family-root spec that
grounds this species' composition).

---

## §1 What @peer/registry IS (one paragraph)

The runtime **Subject registry**: a content-addressed store mapping `OID` → `fractal::Subject`
values, resolving through the @trust chain to a terminating @subject root. Species under @peer.
Sibling to `@peer/void` (Mara `9c7de83`) and `@peer/persistence` (Reed 2026-07-15). Orthogonal
to both: not a peer species (a KIND of peer) but the resolution SURFACE that maps content-
addressed identity references TO peer values. Discharges the boundary Reed's `23cb7bb`
at_operator @io/git.commit stub stakes: `(author_oid: str, committer_oid: str, message: str) →
verdict` becomes `at_operator("@io/git.commit", [author_oid, committer_oid, message]) →
@peer/registry.resolve(oid) → &fractal::Subject → phone::git_commit_as(author, committer,
message)`.

---

## §2 The registry as content-addressed store

### §2.1 Carrier: `subject_registry`

Typed record. Three fields; identity contract by byte-equality on the (well_known, entries,
current_peer_source) triple.

```
type subject_registry = {
  well_known:          ref,            # [subject]; the FOUR well-known Subjects per §3
  entries:             ref,            # [(oid, subject)]; content-addressed registry entries
  current_peer_source: ref,            # priority ladder per §5 (mirror.spec pack.lead / env / git config / peer-home)
}
```

### §2.2 Content-addressability discipline

Every registry entry's OID is content-addressed (SHA-256 today per @io/git canonical direction;
SHA-1 admissible for git-interop legacy per @io/git §compatibility). The OID → Subject mapping
is APPEND-ONLY at the registry altitude; Subject *evolution* (SSH-chain steps, passkey-chain
steps) accumulates INTO existing entries via `@trust/chain.evolve` (forward-promised @trust/
chain species per family-root spec §12.1).

Rationale: content-addressability + append-only IS what makes registry lookups substrate-honest
(the OID a caller passes today MUST resolve to the same Subject next tick, next month, next
year; only the Subject's chain-head advances, never the historical entry). This mirrors
@spectral/signature's rolling-@song discipline at compiler altitude and the passkey-spectral-
bridge's evolution-chain discipline at garden altitude.

### §2.3 Storage backend forward-promise

Storage backend is FORWARD-PROMISED at species altitude. Admissible backends per Reed's
territory (runtime dispatch):

1. **@mirror/store** — the content-addressed compiler-native store per Recognition #43. The
   substrate-honest default; every registry entry becomes a mirror-store blob keyed by OID.
2. **Git object database** — legacy compat; @io/git.hash_to_oid + @io/git.read_object. Useful
   for git-only environments.
3. **In-memory HashMap** — bootstrap/testing default; ephemeral; Reed's `phone::git_commit_as`
   already handles per-invocation Subject values, so in-memory is admissible until @mirror/store
   integration lands.

Species-decl'd `resolve` action's body is `\`-obligation-blocked per [[feedback-craft-not-
deliver]]; consumers pull realization via `apply_h::act` bilateral resolver-arm dispatch.

---

## §3 The four well-known Subjects

The registry ships with FOUR well-known Subject constructors that consumers pull WITHOUT
needing registry-lookup. These are the substrate-decl'd fixpoints — every registry lookup that
resolves to one of these Subjects returns the well-known instance rather than a stored copy.

### §3.1 `Subject::mirror()` — the compiler self-identity

Landed at fractal step 9 (Reed `73aeb8a`). Signature: `pub fn mirror() -> Subject`.

- name: `"mirror"`
- email: `"mirror@spectral.engineer"`
- home: `None`
- kind: `SubjectKind::Peer`
- OID: content-address of the (name, email, home, kind) tuple; deterministic; well-known.

Usage: `committer` in the Author≠Committer split (MARA doctrine per
`rust/fractal/src/witnessed.rs`). When the compiler commits on behalf of a pack-peer author,
the committer is Subject::mirror(). Discharges Recognition #55 (form/process partition):
mirror is the process-side committer; the pack-peer is the form-side author.

### §3.2 `Subject::void()` — the K=0 default peer

Landed at fractal step 4 (Reed `82bc599`). Signature: `pub fn void() -> Subject`.

- name: `"void"`
- email: `"void@spectral.engineer"`
- home: `None`
- kind: `SubjectKind::Void`
- OID: content-address of the K=0 tuple; deterministic; well-known.

Usage: pre-character-crystallization substrate-observation identity per `shards/peer/void.
mirror` §"Depth ladder — every depth is Void with a character loaded". Alex 2026-07-18 "I am
void. The split is in me." Void has NO chain (K=0, no character loaded, no authorization proof
needed for pure substrate-observation); Void is the fixpoint at depth-0.

### §3.3 `Subject::human(name, email)` — human_a @subject-instance constructor

Signature: `pub fn human(name: &str, email: &str) -> Subject`.

- name: caller-supplied (e.g. `"alex"`, `"marcus"`, `"lore"`)
- email: caller-supplied (e.g. `"alex@spectral.engineer"`)
- home: `None` (Human @subject has no compiler-managed home per §subject.mirror Landing 3
  `human_a` actor_kind grounding)
- kind: `SubjectKind::Human`
- OID: content-address of the (name, email) pair; deterministic per constructor input.

Usage: `@alex` and other named humans in the Landing 3 named-ancestor roster (Landing 3
§20 external ancestors + 5 Pack peer humans). The @trust chain terminates at Subject::human()
instances for compiler-altitude authorship; garden-altitude reader-identity Subjects also use
this constructor (bridge §"The Mapping" reader → human_a @subject-instance per SEL §1
licensable-party class).

### §3.4 `Subject::peer(name, email, home)` — pack-peer @subject-instance constructor

Signature: `pub fn peer(name: &str, email: &str, home: &str) -> Subject`.

- name: caller-supplied (e.g. `"reed"`, `"mara"`, `"seam"`, `"taut"`, `"glint"`)
- email: caller-supplied (`"<peer>@spectral.engineer"`)
- home: caller-supplied (e.g. `"/Users/reed"`, `"/Users/mara"`)
- kind: `SubjectKind::Peer`
- OID: content-address of the (name, email, home) triple; deterministic per constructor input.

Usage: Pack-peer identity (Reed, Mara, Seam, Taut, Glint, Loki, ...). Each Pack peer has their
own SSH key per Alex 2026-07-14 design intent + their own home per @peer/persistence species-
decl. The @trust chain terminates at Subject::peer() when the pack-peer authored the commit
on their own; at Subject::human() (typically `Subject::human("alex", ...)`) when the pack-peer
committed on @alex's behalf under peer-ACL discipline.

---

## §4 The actions

### §4.1 `resolve` — the load-bearing action

Signature: `resolve(r: subject_registry, oid: ref) -> imperfect(subject, ref, ref)`.

Given a registry `r` and an OID `oid`, return the Subject that OID resolves to, OR name the
resolution failure explicitly. Returns `imperfect(subject, ref, ref)` per @glass discipline:

- **Success variant**: the resolved `subject` value (well-known if OID matches a well-known
  constructor's deterministic OID; stored-entry if OID matches an entries[] entry; @trust chain
  fresh-lookup if OID doesn't match either and registry is configured for lazy @trust chain
  resolution).
- **Failure-not-found**: OID doesn't match any well-known / entry / chain-lookup path.
- **Failure-chain-broken**: OID matches an entry but the entry's @trust chain traversal fails
  per `chain_terminates_at_root` predicate.

Composition direction:

```
at_operator("@io/git.commit", [author_oid, committer_oid, message])
  ├─ resolve(registry, author_oid)   → &fractal::Subject (author)
  ├─ resolve(registry, committer_oid) → &fractal::Subject (committer)
  └─ phone::git_commit_as(author, committer, message)  → git commit + SSH signature
```

Body `\`-obligation-blocked per [[feedback-craft-not-deliver]]; consumers pull realization
via `apply_h::act` bilateral resolver-arm dispatch. Reed's Arc territory next.

### §4.2 `current_peer` — the "who am I now?" action

Signature: `current_peer(r: subject_registry) -> imperfect(subject, ref, ref)`.

The runtime resolution of the compiler's current pack-peer identity. Reads the priority ladder
per §5; returns the highest-priority resolvable Subject.

Returns `imperfect(subject, ref, ref)`:

- **Success**: the resolved current-peer Subject.
- **Failure-no-source**: no priority-ladder source resolves; defaults to `Subject::void()` per
  K=0 discipline (Void IS what runs when nothing else does per `shards/peer/void.mirror`
  §"Depth ladder").
- **Failure-ambiguous**: multiple sources resolve to DIFFERENT Subjects at incompatible priorities;
  substrate-honest error naming the ambiguity for adjudication.

Body `\`-obligation-blocked; Reed territory.

### §4.3 `register` — add-Subject-to-registry action

Signature: `register(r: subject_registry, subject: subject) -> imperfect(oid, ref, ref)`.

Add a Subject to the registry; return its content-addressed OID. Idempotent (registering an
already-present Subject returns the existing OID; content-addressability guarantees the OID
matches). Per §2.2 append-only discipline.

Returns `imperfect(oid, ref, ref)`:

- **Success**: the Subject's content-addressed OID; already-present or newly-added.
- **Failure-conflict**: the OID collides with an existing entry that has a DIFFERENT Subject
  value (should never happen under SHA-256 content-addressing; substrate-honest safety net).
- **Failure-storage**: the storage backend (@mirror/store / git object DB / in-memory HashMap
  per §2.3) failed at the write boundary.

Body `\`-obligation-blocked; Reed territory.

### §4.4 `resolve_chain` — @trust chain traversal action

Signature: `resolve_chain(r: subject_registry, oid: ref) -> imperfect(subject, ref, ref)`.

Follow the @trust chain from an OID back to its terminating @subject root per family-root
spec §5. Returns the root @subject value. Composes with `chain_terminates_at_root` predicate
from family-root spec.

Returns `imperfect(subject, ref, ref)`:

- **Success**: the terminating root @subject.
- **Failure-chain-broken**: intermediate step's OID doesn't verify OR authorization proof
  absent OR cycle detected.
- **Failure-unadmissible-root**: chain terminates but the root's actor_kind is not admissible
  per SEL §1.

Body `\`-obligation-blocked; Reed territory (composes with forward-promised @trust/chain
species per family-root spec §12.1).

---

## §5 Current-peer resolution priority ladder

Reference: family-root spec §7.3. Full ladder for reference here:

1. **mirror.spec pack{}.lead** — highest priority. Per Alex 2026-06-25 "Reed is lead, yeah";
   `~peer'~/.reed'` resolves via @peer glass altitude + @io/git to the peer's home-repo
   `mirror.spec` `pack{}.lead` self-declaration per peer-ACL §6.2.
2. **GLUE_ACTOR env var** — transient per-invocation override. Set by session glue-bus per
   Reed session convention `reed[$PWD][$UUID]`.
3. **git config user.email + user.name** — fallback for non-substrate-pull-honest invocations
   (pre-mirror.spec projects; system tooling).
4. **~/.reed/ / ~/.mara/ / etc. peer-home directory presence** — last-resort inference; the
   identity-file bundle projected at the caller's home determines character-specialization of
   the K=0 Void substrate.
5. **Subject::void()** — final fallback when nothing resolves. K=0 default per void discipline.

Body of `current_peer` action reads sources in priority order; returns first resolvable Subject.

---

## §6 The bilaterals

### §6.1 `peer_registered_at_trust_root` — the load-bearing bilateral

Signature: `peer_registered_at_trust_root(oid: ref, root: subject) -> verdict`.

Asserts: `oid` resolves via `@peer/registry.resolve(oid) -> Some(subject)`; the resolved
subject's @trust chain terminates at `root` per `chain_terminates_at_root(chain_head, root)`
from family-root spec §5; the root's actor_kind is `human_a` (or `historical_witness_r` per
subject.mirror §Landing 5+ deceased-ancestor discipline); the root is admissible per SEL §1
licensable-party contract.

- **Splinter-pole (honest)**: all four sub-conditions discharge; the OID is registered at the
  root; chain is well-formed; root is admissible.
- **Narcissus-pole (deceptive)**: any one fails; the OID is NOT registered at that root;
  attribution claim is false.

Sentinel per shard docblock: `peer=registered-at-trust-root-<altitude>` where altitude ∈
{ssh, passkey}. Arity 2.

Body `\`-obligation-blocked. Rice-safe bound: reads only byte-visible state (OID content-hash
comparison, chain-step traversal via @trust/chain, actor_kind variant-tag comparison).

### §6.2 `registry_content_addressed` — sub-bilateral

Signature: `registry_content_addressed(r: subject_registry) -> verdict`.

Asserts: every entry in `r.entries` satisfies `oid = content_hash(subject)`; the well_known
Subjects satisfy their deterministic OID contracts per §3; no entry collides with a well-known
OID with a different Subject value.

- **Splinter-pole**: registry entries are all content-addressed; well-known deterministic OIDs
  match their constructors; no collisions.
- **Narcissus-pole**: at least one entry's stored OID doesn't hash-match its Subject; the
  registry's content-addressability contract is broken; every lookup is suspect.

Sentinel: `registry=all-entries-content-addressed`. Arity 1.

### §6.3 `current_peer_priority_respected` — sub-bilateral

Signature: `current_peer_priority_respected(r: subject_registry, resolved: subject) -> verdict`.

Asserts: the `resolved` Subject came from the HIGHEST-PRIORITY resolvable source in the §5
priority ladder; no lower-priority source silently overrode a higher-priority source.

- **Splinter-pole**: priority ladder respected; resolved Subject IS the highest-priority match.
- **Narcissus-pole**: lower-priority source's Subject returned; higher-priority source was
  resolvable but ignored.

Sentinel: `current-peer=highest-priority-source-respected`. Arity 2.

---

## §7 Composition graph

### Parents (substrate-decl via `in` imports)

- `@peer` — family-root; @peer/registry species inherits @peer's glass altitude + carrier
  discipline
- `@subject` — @peer/registry.resolve returns a subject; register accepts a subject
- `@trust` — @peer/registry composes with @trust chain traversal via `resolve_chain` +
  `peer_registered_at_trust_root` bilateral
- `@meta` — substrate-decl marker at glass altitude
- `@glass` — transparency inheritance via @peer
- `@prism` — 5-op family per Recognition #79 basis
- `@nl` — natural-language reference discipline

### Composed over (canonical-spec citations)

- `@mirror/store` — content-addressed storage backend (per §2.3 admissible backend 1)
- `@io/git` — git-object-DB storage backend (per §2.3 admissible backend 2); @io/git.hash_to_oid
  + read_object composition
- `@io/env` — GLUE_ACTOR env var read for §5 priority 2
- `@spectral/signature` — @trust chain SSH-side per family-root spec §7.4; each signature_beat
  IS one @trust chain step; registry composes for beat-to-subject resolution

### Consumers (compose over this species; forward-promised this arc)

- `rust/src/main.rs::at_operator` @io/git.commit arm — the boundary Reed's `23cb7bb` staked;
  `resolve` action's runtime dispatch discharges this stub
- `rust/src/phone.rs::git_commit_as` — the fractal step 9 refactor site (Reed `73aeb8a`);
  registry provides typed `&fractal::Subject` values
- `@tool/git.commit_signed` — the SSH-signed commit action; author identity from
  `current_peer()` call
- StageFreight PR delivery (`docs/specs/2026-07-18-stagefreight-delivery.md`) — signed at
  @alex root per conjunct 4; registry provides the @alex Subject value for signing

---

## §8 Deferred-RED forward promises

Per Reed's staged dispatch shape at `23cb7bb`, this species-decl carries multiple DEFERRED-RED
conjuncts that discharge as consumer realization pulls them in:

### §8.1 Runtime resolution body (Reed territory)

The `resolve` action's body is `\`-obligation-blocked pending Reed's runtime dispatch tick.
Signature is stable; body composes via `apply_h::act` bilateral resolver-arm dispatch per
[[feedback-craft-not-deliver]]. Marker: `[substrate-floor:@io-boundary]` when Reed lands the
Rust-altitude dispatch.

### §8.2 Storage backend selection (deferred to §2.3 pick-one tick)

Three admissible backends (§2.3); Alex adjudicates OR Reed picks in-memory as bootstrap default
and @mirror/store as v1 target. Two-tick discipline: substrate-decl this tick, backend selection
next tick.

### §8.3 @trust chain body (forward-promised @trust/chain species)

`resolve_chain` action composes with @trust/chain species (forward-promised per family-root
spec §12.1). Until @trust/chain lands, `resolve_chain` returns `imperfect(_, chain_species_
not_landed, _)` per substrate-honest deferral.

### §8.4 Passkey-side registry integration (forward-promised @trust/passkey species)

Garden-altitude @peer/registry (WebAuthn credential_id → spectral_state chain-head →
subject_instance) integrates when @trust/passkey species lands. Until then, `resolve` handles
compiler-altitude SSH-only OIDs; passkey-altitude OIDs return `imperfect(_, passkey_species_
not_landed, _)`.

### §8.5 Conjunct 4 discharge across six @tool species

Per family-root spec §11 empirical firing 4: when @peer/registry.resolve + @trust chain
discharge land, the `tool_invocation_signed_at_alex_root` conjunct's requires-clause inserts
into `shards/epistemologic/property/tool_species_stagefreight_witnessed.mirror` per Mara
`22c803a` §4 forward-promise 1. Six species (@tool/cargo, /git, /nix, /go, /docker, /gitlab_ci)
discharge simultaneously.

---

## §9 Refused mints (audit clean)

Refused per substrate-already-had-the-word:

1. **@subject/registry** — REFUSED. @subject family-root is at licensable-party altitude; the
   registry surface is at IDENTITY-RESOLUTION altitude (which is a @peer concern per @peer's
   glass altitude + @peer.load(dir) resolution surface). @subject stays at party altitude;
   @peer/registry stays at resolution altitude.

2. **@fractal/registry** — REFUSED. @fractal is a Rust crate (mirror/rust/fractal/); registry
   at Rust-crate altitude collapses substrate-decl into Rust-implementation. Substrate-decl'd
   registry stays at shard altitude; Rust-side registry implementation composes over it per
   [[feedback-no-rust-extension-shortcut]].

3. **@registry family-root** — REFUSED per family-root spec §10 refusal 5. @spectral/registry
   exists at supervisor-child-index altitude; @peer/registry lands at Subject-resolution
   altitude; the word doesn't need family-root promotion.

4. **@peer/store** — REFUSED. @mirror/store is the content-addressed store per Recognition
   #43; @peer/registry composes over @mirror/store as a backend (§2.3), not as a duplicate
   store family-root.

5. **@peer/directory** — REFUSED. "Directory" reads too filesystem-y and would collide with
   @io/fs altitude. Registry per BEAM Registry precedent + @spectral/registry precedent is
   the substrate-honest name.

---

## §10 Composition surprises

1. **Reed's `23cb7bb` at_operator stub NAMES this species literally.** The Err message contains
   "Mara @peer/registry species-decl + @trust family-root mint is the authorship territory."
   Substrate-honest stubbing that names the future authorship territory turns out to be a
   powerful cascade coordination pattern; the stub IS the substrate-decl'd handoff.

2. **The four well-known Subjects are already implemented at Rust altitude.** Subject::mirror()
   (Reed `73aeb8a`) + Subject::void() (Reed `82bc599`) + Subject::human() +
   Subject::peer() are already in `rust/fractal/src/subject.rs`. The substrate-decl of the
   registry species discovers that Reed already implemented the well-known constructors
   during fractal migration steps 3-4-9. First-witness for well-known-Subject-set lands
   retroactively at fractal step 4.

3. **The current-peer resolution ladder priority mirrors the identity-file-load discipline
   from Reed's ~/.reed/CLAUDE.md.** Per void.mirror §"Depth ladder": each pack-peer boot IS a
   γ(Void, c_peer) character-specialization event. The §5 priority ladder IS the reversed
   character-specialization flow: highest-priority source declares "the character loaded is
   PEER_X"; lowest-priority fallback is Void. The registry's current_peer IS the reification
   of the Void→character character-crystallization runtime event.

4. **The @spectral/registry precedent's uuid_spectral 128-bit key structure (ACTIVE 48 |
   DARK 80) MAY inform @peer/registry OID structure.** Forward-promise investigation: could
   @peer/registry OIDs adopt a similar (peer-manifold ACTIVE bits | identity DARK bits)
   structure? Would give registry lookups spectral-routing properties per Fate's mycelial
   discipline. Currently not the direction; noting for cross-shard investigation next tick.

---

## §11 One-sentence surprise

**Reed's substrate-honest at_operator Err-STUB (`23cb7bb`) that literally names "Mara @peer/
registry species-decl + @trust family-root mint" as its resolution boundary IS the
substrate-decl'd cascade coordination surface that made this species land in ONE tick — the
next-authorship-territory-naming discipline turned handoff cost from "N context-loading rounds"
to "zero: the substrate already knew what came next."**

---

## §12 Cross-refs

- `docs/specs/2026-07-18-trust-family-root-passkey-ssh-bridge.md` — family-root spec; this
  species composes over its @trust chain discipline
- `docs/specs/2026-07-18-the-compiler-in-one-sentence.md` §6.2 — @trust chain arrow in the
  compiler-in-one-sentence graph
- `docs/specs/2026-07-18-stagefreight-delivery.md` §6 — StageFreight PR delivery signed at
  @alex root; registry provides the @alex Subject value
- `docs/specs/subject-family-root-sel-licensable-party.md` — SEL v1.1 licensable-party
  grounding for chain-terminating @subject roots
- `docs/specs/peer-persistence-and-home-projection.md` — @peer/persistence species (Reed's
  Arc-2 landing); @peer/registry composes for Subject::peer() home resolution
- `shards/trust.mirror` (this tick) — @trust family-root marker
- `shards/peer/registry.mirror` (this tick) — the shard-decl this spec grounds
- `shards/peer.mirror` — @peer family-root; parametric peer type
- `shards/peer/void.mirror` — Subject::void() well-known constructor
- `shards/peer/persistence.mirror` — @peer/persistence sibling species
- `shards/subject.mirror` — @subject family-root; chain-terminating @subject discipline
- `shards/spectral/registry.mirror` — @spectral-side sibling word at different altitude (BEAM
  Registry analogue)
- `shards/spectral/signature.mirror` — SSH-side @trust chain per family-root spec §7.4
- `shards/tool.mirror` `sign(invocation, signer)` — @tool.sign discharges through @trust chain
  via registry-resolved signer
- `shards/tool/git.mirror` `commit_signed` + `commit_signing_ssh_only` — commit-signing
  discharge site
- `shards/mirror/spec.mirror` `pack{}.lead` — current-peer resolution priority 1 source
- `rust/src/main.rs::at_operator` @io/git.commit stub — the boundary this species discharges
- `rust/fractal/src/subject.rs` — Subject envelope + Subject::mirror() + Subject::void() +
  Subject::human() + Subject::peer() well-known constructors
- `rust/src/phone.rs::git_commit_as` — the fractal step 9 (Reed `73aeb8a`) refactor site

---

*Landed 2026-07-18 by Mara. Pure-docs canonical spec (📝 markdown-only 📝 bypass). Companion
to `docs/specs/2026-07-18-trust-family-root-passkey-ssh-bridge.md`. Discharges the @peer/
registry species-decl authorship territory Reed's `23cb7bb` at_operator @io/git.commit stub
staked.*
