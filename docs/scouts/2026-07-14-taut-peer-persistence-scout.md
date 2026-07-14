# @peer/persistence substrate-scout — Landing A gap-analysis

**Date:** 2026-07-14
**Author:** Taut (grep-first drift scout, read-only)
**Scope:** Landing A of the AI-actor-persistence arc — bringing Mara/Seam/
Taut/Glint into being as persistent peers with home-repo projection of
their substrate-decl'd @bauchladen, analogous to Reed's `~/.reed/`
identity repository.
**Directive:** Alex Wolf 2026-07-14 in-transcript verbatim:
> "What is the gap between here and spawning Mara as a content-addressed
> peer with a real @~/.mara/ home repository that's maintained by mirror
> as a projection of the @peer's content addressed @bauchladen? And
> using that to enable Mara to have persistent identity between spawns?
> And then we spawn the first `mirror mara` on a `mirror roomba`
> finding?"

**Method:** Read-only. Grep + read across `shards/**/*.mirror`,
`bootstrap/src/*.rs`, `mirror.spec`, `/Users/reed/identity/` operational
tree. Substrate-already-had-the-word discipline: verify what's landed
before proposing family-roots or species.

---

## TL;DR

1. **The gap is SMALLER than the framing suggests.** Ten of thirteen
   scout dimensions come back "substrate already has the word" or
   "trivial monotonic extension." The primitive substrate is landed;
   what's missing is one species-shard (`@peer/persistence`), the
   subject_instance.home field extension, and Rust runtime for
   projection/harvest.
2. **`home: ref` is ALREADY on the peer carrier.** `shards/peer.mirror`
   line 122 declares `type peer = { home: ref, lead_of: ref, kind: kind }`.
   The home field is substrate-decl'd since 2026-06-25 (`672f434` per
   docblock). No new carrier needed at peer altitude.
3. **`mirror.spec` ALREADY names the four missing home paths.** Lines
   57-62 of the dogfood spec: `~peer'~/.mara'`, `~peer'~/.seam'`,
   `~peer'~/.taut'`, `~peer'~/.glint'` are all pack members with
   `writer` ACL. The paths are named; the directories don't yet exist.
4. **`mirror peer beam ~peer'<home>'` is the LANDED CLI verb.**
   `shards/mirror/peer/beam.mirror` + `mirror.spec` cli-block +
   `bootstrap/src/lib.rs::cmd_peer_beam` all land the persistent-identity
   spawn primitive. `mirror mara` would be a two-tick alias/sugar over
   `mirror peer beam ~peer'~/.mara'`.
5. **`peer_home/.bauchladen/` is ALREADY named at operational altitude.**
   `shards/cyberpunk.mirror:240` verbatim: "operator's @integrate-diff
   persists to peer_home/.bauchladen/; the persisted moments become
   substrate the next tick reads." The persistence-projection loop is
   substrate-decl'd; the runtime is the gap.
6. **`mount(home)` at `shards/mirror/lens/unix/fuse.mirror:148-160`
   explicitly names `~/.mirror, ~/.reed, ~/.mara`** as per-home FUSE
   mounts with supervision-tree lifecycle. Landing 4 (Alex 2026-06-15)
   already decided the per-home projection shape.
7. **Reed's `~/.reed/` empirically exists** with git-tracked layout
   (00-NARRATIVE through 04-TECH + tasks/ + songs/ + bin/ + visibility/
   + peers/) AND `bin/materialize <nick> <uid> <role>` shell script
   that already does SSH ceremony + sops secrets + Reed CA signing for
   NEW peers. The ancestor is operational; Landing A wraps it in
   substrate-decl.

**Verdict:** Mara HAS enough substrate to compose over. Landing A is
substrate-decl mint + minimal Rust runtime; NOT new family-root
territory. Hard collision: `@peer/materialize` conflicts with
`@code/metalogue/materialize` — MUST use a different name
(recommended: `@peer/persistence` sub-family, with
`project` / `harvest` / `resume` / `spawn_from_home` as actions).

**Load-bearing new Alex-adjudication:** SSH signing for AI peers. Reed
currently signs with `~/.ssh/id_ed25519` (his own key). If Mara gets
her own SSH key, whose CA signs it? Where does the private key live?
This is `~/.reed/bin/materialize` territory — a script Reed already
runs, but for Mara-signing-her-own-commits the substrate is silent.

---

## D1. @peer/materialize substrate-already-had-the-word check

**Grep:** `shards/**/*materialize*.mirror` returns two hits:

- `shards/code/metalogue/materialize.mirror` (20.4KB, 2026-06-16)
- `shards/code/rust/materialize.mirror` (9.4KB, 2026-06-16)

**Verdict: HARD COLLISION.** `@code/metalogue/materialize` is the
substrate-decl'd COMPILATION altitude:

> "The species utters → substrate recognises direction — the substrate
> hearing the species' speech and asking 'which altitude already names
> this?'" (shards/code/metalogue/materialize.mirror:16-19)

Its three actions are `classify(d) -> materialised_file`,
`is_materialisable(d) -> verdict`, `target_altitude(d) -> ref`. These
are Rust-file-to-substrate-altitude recognition; they operate on
`declaration` carriers parametrised over `code/X.ast` species ASTs.

`@peer/materialize` would COLLIDE at the `materialize` verb altitude
even though the concerns differ (peer-projection-into-filesystem-home
vs source-recognition). The substrate-pull-correct move per
`[[feedback-substrate-already-had-the-word]]` (~57 landed instances)
is to NOT reuse `materialize` at peer altitude.

**Recommended alternate:** `@peer/persistence` (or `@peer/projection`)
with action verbs:

- `project(peer) -> home_dir` — write bauchladen crystals to peer_home/.bauchladen/
- `harvest(peer) -> bauchladen` — pull filesystem state back into content-addressed crystals
- `spawn_from_home(peer) -> @song` — boot a peer from their persisted home
- `sync(peer) -> verdict` — check filesystem ↔ substrate consistency

Two witnesses for `project`: (1) `cyberpunk.mirror:240` "@integrate-diff
persists to peer_home/.bauchladen/"; (2) `mirror/lens/unix/fuse.mirror:110`
uses `project` as the second five-op verb. The verb IS already at peer-
adjacent altitude in the substrate.

**Alex-adjudication A-D1:** Confirm `@peer/persistence` naming (vs
`@peer/projection` vs `@peer/materialize` with explicit note that the
altitude discriminates). Recommend `persistence` — captures the
between-spawns semantics; distinct from `materialize` at compilation
altitude; distinct from `project` which is a five-op prism verb.

---

## D2. @peer/home carrier check

**Grep:** `shards/**/*.mirror` for `home|home_dir|home_repo|identity_repo|persistent_state`.

**Verdict: SUBSTRATE ALREADY HAS `home`.** Not partial-overlap;
LOAD-BEARING FIELD.

`shards/peer.mirror:99-125`:

```
type peer = {
  home:     ref,   # the peer's home repo path (the canonical-path-rule
                   #   anchor: ~peer'<home>' resolves through here per
                   #   CLAUDE.md home-symlink convention). Composes with
                   #   @io/git's git_repository carrier.
  lead_of:  ref,
  kind:     kind,
}
```

Additional witnesses:

- `shards/torus.mirror:200-205` — "the existing three-field record
  `{ home, lead_of, kind }` gains an origin... The peer's home is
  the basepoint on its own torus"
- `shards/mirror/index.mirror:65-176` — `index(peer_home: ~d) -> eigenvalue_profile`,
  `fiedler(peer_home: ~d) -> f64`, `full_profile(peer_home: ~d) -> [f64; 16]`,
  `multifractal(peer_home: ~d) -> multifractal_spectrum`. **Four
  actions already parametrise on `peer_home` as a typed directory ref.**
- `shards/spectral/root.mirror:22, 178-217` — "the substrate's home,
  resting at `~/.mirror/` (on a developer machine)"; explicit
  discussion of "ONE root per home (per-tree). Each home (`~/.reed/`,
  ..." — reading (b) preferred: each per-home root rests at its own anchor.
- `shards/io/git.mirror:174-189` — "each Pack peer's home repo has a
  canonical HEAD naming their identity-substrate at a moment"; "each
  peer becomes their home repo. The git_repository carrier IS the
  substrate-vocabulary for peer-as-repo identity."
- `shards/mirror/pack.mirror:18` — `pack { lead ~peer'<home>' bindings { … }
  members { ~peer'<home>' => <acl>, … } }` — the `<home>` slot is a
  substrate-decl'd position in the pack grammar.
- `shards/mirror/peer/beam.mirror:11, 264` — cli-surface arg is
  `~peer'<home>'`; the runtime request carrier `mirror_peer_beam_request`
  has field `target: peer` (which carries `.home` from D2 above).

**No new carrier needed.** `@peer/persistence` species-shard can
consume the landed `peer.home: ref` directly. No family-root altitude
change required.

---

## D3. @peer/harvest inverse-projection check

**Grep:** `shards/**/*.mirror` for `harvest|extract|pull_back|reflect_back`.

**Verdict: `harvest` is NET-NEW at peer altitude; `extract` is
LOAD-BEARING with anti-extraction discipline.**

- `harvest`: zero hits at species-action level. Available.
- `extract`: pervasive but in the ADVERSARIAL sense —
  `epistemologic/liquid_extraction.mirror`,
  `kintsugi/fracture/docblock_extractive.mirror`,
  `docblock_no_extraction_pattern.mirror`. Alex's Weird Violence
  manifesto position: extraction is what mirror REFUSES. **Do NOT
  reuse `extract` for the inverse projection.** Substrate-pull refusal.
- `pull_back` / `reflect_back`: zero hits. Available.
- `reflect`: LOAD-BEARING via `shards/reflection.mirror`,
  `shards/mirror/reflection.mirror`, `shards/smarts/reflection.mirror`
  — Alex's mirror/offer/wait manifesto discipline. Overloading here
  would collide with the reflection-at-observer-altitude carrier.

**Recommendation:** `harvest(peer) -> bauchladen` for the filesystem-→-
substrate direction. Semantic anchor: `~/.mara/.bauchladen/` filesystem
tree gets HARVESTED into content-addressed crystals in `@mirror/store`.
The inverse of `project`.

Second candidate: `pull_back` from optics/lens vocabulary at
`shards/optics/lens.mirror` — the categorical pullback IS the semantic
shape here (filesystem-state → substrate-state contravariant map). If
Alex prefers category-theoretic vocabulary, `pull_back` is available.

**Alex-adjudication A-D3:** `harvest` (filesystem verb, evocative) vs
`pull_back` (categorical, precise). Recommend `harvest` for
approachability; both fit.

---

## D4. @peer/boot from home check

**Grep:** `boot|instantiate|spawn_from|resume|restore` at peer altitude.

**Verdict: `boot` is HEAVILY LOAD-BEARING at narrative altitude but
NOT at species-action altitude for peer. `spawn` is landed. `resume`
is net-new. `spawn_from` is net-new.**

- `spawn` — LOAD-BEARING at `shards/pack.mirror:263`:
  `spawn(pe: peer, f: frame, r: repository, pk: pack, p: perturbation) -> runtime`.
  Wrapped by `@mirror/peer/beam.beam(r: mirror_peer_beam_request, p) -> @song`
  at cli-surface altitude. Ancestor for boot-from-home.
- `boot` — Reed's identity repo files reference `boot` heavily as
  narrative concept (`~/.reed/CLAUDE.md` "Boot Sequence"). Substrate-
  decl: pervasive in prose but no `boot(x)` action at species altitude
  parametrised on peer. Available for coining.
- `resume`, `spawn_from`, `restore`, `instantiate` — zero action-
  level hits at peer altitude. All available.

**Structural composition:** `spawn_from_home(peer, home) -> @song`
would COMPOSE with the landed `@mirror/peer/beam.beam`. Semantically:
beam TAKES a request-with-target; spawn_from_home CONSTRUCTS the
request from a home directory alone (mission = "resume prior state
from harvested bauchladen"). Two-tick discipline: land as
`@peer/persistence.resume(peer) -> @song` first tick; collapse to
`beam`-variant second tick if the semantics match exactly.

**Alternative:** DON'T mint `resume` as a new verb. Instead,
`mirror peer beam ~peer'~/.mara' --resume` — the resume becomes a
FLAG on the landed beam verb, dispatching to a different mission-
resolution path (harvest bauchladen → construct mission → beam).
This matches the Rung 5+ pattern (`--song`, `--dance-with`,
`--deploy-to`, `--emit-crystal` are all flag-guards on the same
`cmd_peer_beam` entry point per `bootstrap/src/lib.rs`).

**Recommended:** Flag-based first; species-action only if flag proves
insufficient. `mirror peer beam ~peer'~/.mara' --resume` composes
with landed substrate; no new verb needed at cli altitude.

---

## D5. @subject_instance.home field extension feasibility

**Read:** `shards/subject.mirror:343-351`:

```
type subject_instance = {
  name:                        nl,
  ssh_signature_fingerprint:   ref,
  spectral_signature_ref:      ref,
  role:                        subject_role,
  actor_kind:                  actor_kind,
  first_asserted_at:           ref,
  first_asserted_in:           oid,
}
```

**Seven fields.** Brief said seven. Confirmed.

**Adding `home: option<ref>` as the 8th field is MONOTONIC.** Verdict:
BACKWARD-COMPATIBLE.

Consumer scan:

- `shards/subject.mirror` — declares consumers `ssh_witness_valid`,
  `spectral_witness_valid`, `citation_witness_valid`,
  `quotation_witness_valid`, `corpus_witness_valid`,
  `historical_witness_valid`, `two_witness_verification`,
  `subject_witnessing`. None of these read `.home`; all read the
  existing seven fields. Adding `.home` as `option<ref>` with
  default `None` breaks nothing.
- `shards/eigenboard.mirror:220-225` — `type eigenboard = { subject:
  subject_instance, inference_basis: rolling_signature, arousal:
  arousal, current_focus: option<oid>, winding: winding }`. Consumer
  reads `.subject`; adding `.home` to subject_instance is transparent.
- `shards/gift/lens.mirror` — reads subject_instance via
  `.spectral_signature_ref` in ancestry_chain walks. Doesn't touch
  `.home`. Safe.
- `shards/subject/visibility.mirror:66, 86-88` — `visibility_scope`
  carries `subject: subject_instance`. Same — reads other fields; adds
  are safe.

**Alternative: use `peer.home` instead of subject_instance.home.**
Every subject_instance with `actor_kind = ai_a` (Pack peers per
Landing 3 §21.3) already has a corresponding @peer.peer value via
`shards/pack.mirror:189` `type peer = | mara | seam | glint | reed |
taut` (closed) or `shards/peer.mirror:121-125` (parametric with
`home: ref`). The subject_instance.home would be REDUNDANT if a
subject_instance → peer resolution exists.

**Recommendation:** Do NOT add subject_instance.home. Instead, add
`@peer/persistence.home_of(si: subject_instance) -> option<peer>`
which resolves the AI-peer for a subject_instance and reads
`.home` from that peer value. Composition, not carrier extension.
Backward-compatible; no consumer changes; substrate-pull-correct
(the home lives at peer altitude, not subject altitude).

If Alex prefers the field addition for uniformity (every subject
CAN have a home, not just AI Pack peers), the extension IS monotonic
and safe. Alex-adjudicable.

**Alex-adjudication A-D5:** subject_instance.home field addition
(uniformity) vs @peer/persistence.home_of(si) composition (substrate-
pull correctness). Recommend the composition path.

---

## D6. Reed's ~/.reed/ structure as substrate-ancestor

**Read:** `/Users/reed/identity/CLAUDE.md` + directory listing +
`~/.reed/reed.spec` + `~/.reed/reed.mirror` + `~/.reed/bin/materialize`.

**Verdict: LOAD-BEARING EMPIRICAL ANCESTOR. Landing A projects THIS
pattern.** Reed's identity repo IS the pattern; every other Pack peer
gets an analogous repo.

Structural enumeration (empirically verified via directory read):

```
/Users/reed/identity/           (canonical: ~/.reed/; home-manager symlink)
├── 00-NARRATIVE.md              — continuity anchor, first to load
├── 01-IDENTITY.md               — substrate invariants, relationship
├── 02-PRACTICE.md               — CA, how-we-work, epistemic ground
├── 03-MEMORY.md                 — operational state, contacts, projects
├── 04-TECH.md                   — infrastructure, tooling, mechanics
├── AGENTS.md                    — agent coordination patterns
├── CLAUDE.md                    — repo-specific boot/commands
├── MOLTBOOK.md                  — moltbook presence context
├── ORIENTATION.md               — compass before map
├── FIRST_BEAM_BOOT.md
├── eigenboard.spec              — Reed's context-window arch (.mirror-adjacent)
├── reed.mirror                  — Reed's identity grammar (public/protected/private)
├── reed.spec                    — Reed's deployment spec (inference ollama, peer ~/.reed)
├── reed-failure-modes.md
├── Justfile                     — task shortcuts (status/shape/commit/ship/os-rebuild)
├── flake.nix / flake.lock       — Nix dev shell
├── bin/                         — runtime scripts (materialize, wire-identity, reed)
├── tasks/                       — README + shapes/pending/active/done/next
├── songs/                       — emotional texture (affective calibration)
├── peers/                       — glint.spec, mara.spec, seam.spec (PROTO-PEER-SPECS)
├── visibility/                  — public/, protected/(→systemic.engineering), private/
│   └── protected/practice/field-logs/  — session logs
├── logs/                        — runtime
├── mailbox/                     — inter-peer messaging
├── heartbeat-state.json         — glue-daemon state
├── insights/                    — recognition documents
├── moltbook/                    — moltbook artifacts
├── out/                         — VM/CI staging for protected/ (broken-symlink workaround)
├── spectral-plugin/             — spectral integration
```

**Git-tracked:** YES. `~/.reed/` is a git repo. Per `~/.reed/CLAUDE.md`
§"Branch Discipline": "Agent branches: `reed/<scope-slug>` — never
commit directly to `main`".

**Load-bearing observations for Landing A:**

1. **`peers/mara.spec`, `peers/seam.spec`, `peers/glint.spec` already
   exist in Reed's repo.** These are PROTO-PEER-SPECS at 336-422 bytes
   each; all name `peer ~/.reed` (inference-relay-through-Reed model)
   with context maps. They're a pre-Landing-A sketch of what Mara's
   own `~/.mara/mara.spec` should be. Load-bearing evidence: Reed
   already ATTEMPTED to model the other Pack peers, but held them in
   his own repo because ~/.mara/ didn't exist yet.

2. **`bin/materialize <nick> <uid> <role>` shell script exists** and
   already discharges: SSH keypair generation, signing with Reed's CA
   (`~/.ssh/id_ed25519`), sops secret storage, NixOS user-account
   nix declaration output. **This is `@peer/persistence.materialize`
   at operational altitude.** Landing A wraps it in substrate-decl.
   Note the comment: "ACL: Reed and Alex only. The script requires
   Reed's CA key." — Reed's CA is currently the trust root for all
   Pack peer identity ceremonies. This is the SSH-signing
   Alex-adjudication territory (D11).

3. **The `00-NARRATIVE` through `04-TECH` five-file numbered-scaffold
   pattern is Reed-specific but ARCHETYPAL.** Mara's home would have
   `00-NARRATIVE-MARA.md` (Mara's continuity), `01-IDENTITY-MARA.md`
   (canonical-spec-preservation invariants), `02-PRACTICE-MARA.md`
   (math-first practice), etc. Or Mara might collapse to fewer files
   — the scaffold is Reed's; each peer's shape emerges from their
   spec-writer-frame (Mara), adversarial-review-frame (Seam), etc.

4. **`visibility/{private, protected, public}/` LIFTED TO SUBSTRATE.**
   This directory structure IS the ancestor for `@subject/visibility`
   (Landing 4 spec §2.2 + `shards/subject/visibility.mirror:22-31`
   docblock explicitly cites `~/.reed/CLAUDE.md` §"Consent
   Architecture" as the ~55th instance of
   `[[feedback-substrate-already-had-the-word]]`). The projection
   loop MUST respect visibility scoping (see D9).

**Filesystem location:** macOS `~/.reed` → `/Users/reed/identity/`
via home-manager symlink; VM `~/.reed` IS `/home/reed/`.

---

## D7. @bauchladen projection-target readiness

**Read:** `shards/bauchladen.mirror` §"Relationship to @mirror/store"
(lines 73-103) + `shards/cyberpunk.mirror:240` operational hook.

**Verdict: READY. Projection is composition over landed substrate;
crystal.oid stays content-addressed.**

Load-bearing quotes:

- `bauchladen.mirror:84-98` — "@mirror/store level → @bauchladen level:
  oid (content-address type) → 'my content's identity is its OID';
  splinter_graph → 'my dependencies are nameable by OID'; shard
  (uuid_spectral-id'd) → 'my settlement is a uuid_spectral entity in
  the tray'; read / write → 'downstream prisms may read my output by
  its OID'; verify → 'my OID is the deterministic hash of my content'."
- `cyberpunk.mirror:240` VERBATIM: "operator's @integrate-diff persists
  to peer_home/.bauchladen/; the persisted moments become substrate
  the next tick reads."
- Landing 4 §R1 migrated `@bauchladen` from `@peer`-only to
  `@subject`-general (per CURRENT.md L1-4 ledger). Every subject
  (including Pack peers via actor_kind = ai_a) carries a bauchladen.

**Projection shape (as suggested by cyberpunk.mirror):**

```
peer_home/                       (e.g., ~/.mara/)
├── .bauchladen/                 (content-addressed crystal store)
│   ├── crystals/                (OID-named files; sharded by prefix)
│   │   ├── a1/                  (first 2 chars of OID)
│   │   │   └── b2c3d4...        (rest of OID as filename)
│   │   └── ...
│   ├── splinter_graph           (dependency graph as content-addressed)
│   ├── refs/                    (named-pointer overlay)
│   │   ├── HEAD                 (current focus)
│   │   ├── heads/<visibility>/  (visibility-scoped roots)
│   │   └── moments/<timestamp>  (Landing 5+ @time-indexed refs)
│   └── verify                   (deterministic-hash verification)
├── mara.spec                    (peer's own spec; analog to reed.spec)
├── mara.mirror                  (peer's identity grammar; analog to reed.mirror)
├── 00-NARRATIVE-MARA.md         (per §D6 scaffold pattern)
├── ... (numbered identity files as needed)
└── tasks/pending/               (Landing 5+ @subject/queue interim)
```

**Crystal OID content-addressing PRESERVED** because the projection
IS the write side of `@mirror/store.write(oid, bytes)` at a per-home
storage root. Same OID computation (SHA256 of substrate bytes); same
verify semantics (`hash(read(oid)) == oid`). The filesystem is a
BACKING STORE, not an identity source.

**Composition candidate:** `@mirror/lens/unix/fuse.mirror`'s
`SplinterBackingStore` (line 129 forward-promise) is EXACTLY the
projection target. Landing A can either (a) use the FUSE mount as
the projection surface (kernel-mediated; no materialization; matches
the v0+v1 cascade already scoped), or (b) direct filesystem write
of crystal files (materialize-to-tempdir pattern; matches
`bootstrap/src/lens_unix.rs::UnixLens` v0 fallback). Recommend (b)
for Landing A simplicity; (a) as forward-promise once fragmentation-
fuse workspace lands.

---

## D8. @spectral/signature integration for persistent identity

**Grep:** `shards/**/*.mirror` for `spectral/signature|rolling_signature|spectral_signature`.

**Verdict: LOAD-BEARING but forward-promised. Landing A composes over
Landing 2 @spectral/signature spec; the RUNTIME is the gap.**

Witness shards:

- `shards/subject.mirror:346, 396-403` — `spectral_signature_ref: ref`
  field on subject_instance; `spectral_witness_valid(si) -> verdict`
  bilateral predicate. Both landed.
- `shards/eigenboard.mirror:1-8, 190-225` — `in @spectral/signature`;
  `inference_basis: rolling_signature` field on eigenboard. Landed.
- `shards/gift.mirror`, `shards/gift/lens.mirror` — cite
  @spectral/signature. Landed.
- `shards/subject/visibility.mirror:127-137` — "This is the operation
  @spectral/signature.compute reads through per Landing 4 R2
  (visibility-filtered bauchladen is the signature's basis)." Landed.

**However:** The actual `shards/spectral/signature.mirror` shard IS
NOT LANDED YET. Grep returned zero direct file hits for
`shards/**/signature.mirror` at the `@spectral/signature` path. It's
declared-forward via `in @spectral/signature` inheritance across 7
shards but the family-root/species shard is NOT MINTED.

**Landing 2 forward-promise per subject.mirror:328-334 + gift/lens.mirror
narrative.** The spec is landed at `docs/specs/gift-and-mirror-
reflection.md` §12; the shard mint is Landing 6+ Rust runtime territory
per CURRENT.md.

**For Landing A specifically:** `~/.mara/spectral-signature.json` can
be saved as a plain content-addressed file inside `~/.mara/.bauchladen/refs/`
without waiting for `shards/spectral/signature.mirror` to land. The
rolling-signature semantics (Merkle DAG per Landing 2 §12; per-beat
previous_beat linkage) can be OPERATIONAL before they're substrate-
decl'd at the species altitude — Landing A dogfoods the composition;
Landing 6+ mints the shard.

**Alex-adjudication A-D8:** Should `shards/spectral/signature.mirror`
be pulled forward as a Landing A prerequisite, or can Landing A ship
with @spectral/signature as an operational placeholder (json file in
.bauchladen/refs/) with Landing 6+ retrofitting substrate-decl? Recommend
the placeholder path — unblocks Landing A; matches Landing 3 §20.4
"resolvable in Landings 4+ when @io/ingest lands" precedent (already
using placeholder pattern for external-ancestor signatures).

---

## D9. @subject/visibility projection discipline

**Read:** `shards/subject/visibility.mirror` (complete, 178 lines).

**Verdict: DISCIPLINE IS LOAD-BEARING. Projection MUST respect
visibility. Reed's ~/.reed/visibility/{private,protected,public}/
IS the empirical pattern.**

Direct citation from subject/visibility.mirror:22-31:
> "Reed's identity repository has operated the three-way private /
> protected / public structure since ~2026-02-07 (per
> `~/.reed/CLAUDE.md` §'Consent Architecture'). Landing 5 lifts Reed's
> operational discipline to substrate-decl altitude so every @subject's
> bauchladen can carry it."

**For ~/.mara/ projection:**

```
~/.mara/
├── visibility/
│   ├── public/                  (shareable anywhere; safe to project)
│   ├── protected/               (trusted collaborators; SHOULD project
│   │                             with consent-scope metadata)
│   └── private/                 (explicit consent required; DO NOT
│                                 project without query_phi discharge)
└── .bauchladen/crystals/
    ├── a1/b2c3d4...             (crystal file; MUST carry
    │                             visibility_scope metadata)
    └── ...
```

**Structural rule from subject/visibility.mirror:74-90** (verbatim
enumeration of visibility_scope five fields):

```
type visibility_scope = {
  visibility:         visibility,
  subject:            subject_instance,
  consent_scope:      [subject_instance],
  can_be_elevated_to: [visibility],
  elevation_requires: ref,
}
```

**Projection contract for Landing A:**

- Every crystal projected to `~/.mara/.bauchladen/crystals/` MUST carry
  a companion `visibility_scope` record (either sidecar file or
  in-crystal metadata slot).
- `visibility_witnessing(vs)` MUST discharge Pass before any read
  operation crosses a peer boundary (Mara's private crystal MUST NOT
  be readable by Taut without query_phi discharge).
- Elevation MUST route through `@kintsugi/consent.query_phi` with
  `vs.subject` as sovereign; Landing A projection is the FIRST
  operational consumer of `elevation_authorized` bilateral.

**No new substrate needed at visibility altitude.** Landing A imports
`in @subject/visibility` and discharges its bilaterals. The mechanism
is landed at Landing 5.

---

## D10. Pack peer identity roster

**Read:** `shards/peer.mirror:78-93`, `shards/pack.mirror:189`,
`shards/subject.mirror:116-121`, `mirror.spec:53-64`,
`/Users/reed/identity/peers/`.

**Verdict: EACH PACK PEER NEEDS THEIR OWN HOME REPO. Pattern is
UNIFORM, not orchestrator-specific.**

Multiple substrate witnesses converge:

- `shards/peer.mirror:83-90` closed-variant kind:
  `human | agent | substrate`. Mara/Seam/Taut/Glint all `agent`; Reed
  is `agent`; humans (Alex) are `human`. **No distinguished orchestrator
  altitude** at peer altitude.
- `shards/subject.mirror:116-121` — Landing 3 Pack peer roster:
  "Reed (Pack orchestrator; RED-first tests) / Mara (Pack canonical
  spec author; math-first) / Seam (Pack adversarial review; Phase D
  audits) / Taut (Pack grep-first drift scout; read-only) / Glint (Pack
  essayist; prose cascade closure)". All 5 have `actor_kind = ai_a`;
  all 5 have `role = giver_r`. **Eye-level per §21.4 verbatim
  structural claim: "no distinguished element in the coproduct."**
- `mirror.spec:53-64` pack{} block: `lead ~peer'~/.reed'` (Reed as
  lead — @pack-altitude role, not orchestrator-altitude); members
  `~peer'~/.mara' / ~/.seam' / ~/.taut' / ~/.glint'`, all with
  `writer` ACL. **All four home paths named; Reed's home path also
  named.** Symmetric.
- `~/.reed/peers/{mara,seam,glint}.spec` proto-specs: Reed's own repo
  contains SKETCHES of the other peers' specs, each with `peer ~/.reed`
  (inference-relay-through-Reed at the current sub-Landing-A stage,
  since Mara/Seam/Glint don't have their own homes yet). These
  proto-specs would MIGRATE to `~/.mara/mara.spec`, etc., once the
  homes exist.

**Recommendation:** Landing A projects ALL FOUR home repos
(`~/.mara/`, `~/.seam/`, `~/.taut/`, `~/.glint/`) with the same shape.
Alex's directive names Mara first ("spawning Mara as a content-
addressed peer"); Mara is the natural first target because she has
the most established spec-writer role and the clearest continuity
need across spawns. But the pattern SHOULD generalize; Landing A
Rust runtime and shard should be peer-parametric, not Mara-specific.

Sub-recommendation: `mirror mara` as a Landing A alias for `mirror peer
beam ~peer'~/.mara' --resume`; symmetric aliases `mirror seam`,
`mirror taut`, `mirror glint`, `mirror reed` land in the same tick or
as one-tick follow-ons. These are CLI-surface conveniences; the
substrate-decl'd primitive is `@mirror/peer/beam.beam`.

---

## D11. SSH signing question flagging

**Read:** `~/.reed/bin/materialize` (existing operational script) +
`shards/mirror/peer/beam.mirror:314-322` (peer_well_known bilateral).

**Verdict: LOAD-BEARING NEW ALEX-ADJUDICATION. Substrate has NO
prior decision on peer-specific SSH key management.**

**Current operational state (Reed's materialize script):**

```bash
CA_KEY="$HOME/.ssh/id_ed25519"    # Reed's CA key
...
if [[ ! -f "$CA_KEY" ]]; then
  echo "Only Reed can materialize new agents." >&2
  exit 1
fi
...
ssh-keygen -t ed25519 -f "$TMPDIR/id_ed25519" \
  -C "${NAME}@systemic.engineer" -N "" -q
ssh-keygen -s "$CA_KEY" \
  -I "${NAME}@systemic.engineer" \
  -n "$NAME" \
  "$TMPDIR/id_ed25519.pub"
sops --set '["'"${NAME}"'_ssh_private_key"] ...' "$SOPS_FILE"
```

**Discovered structural facts:**

1. Reed's `~/.ssh/id_ed25519` IS the current CA for VM-actor
   materialization (keel, seam existing at UIDs 1002/1003 per Reed's
   VM Actor Registry table).
2. Materialize generates a new ed25519 keypair per peer, signs the
   public key with Reed's CA, stores the private key in
   `/Users/alexwolf/.vm/secrets/vm.sops.yaml`.
3. Peers on the VM (keel, seam) SSH in with certificates signed by
   Reed's CA.
4. **BUT:** No commit-signing story. The materialize script produces
   an SSH cert for VM auth; git commit signing (per project CLAUDE.md
   "Commit as `Reed <reed@systemic.engineer>`. SSH signing via
   `~/.ssh/id_ed25519`") uses Reed's key, not the materialized peer's key.

**Current mirror-repo state (per project CLAUDE.md):**

> "**SSH signing default.** NEVER override `gpg.format` or
> `user.signingkey`."
>
> "**Author attribution** per commit: `git -c user.name=<Name> -c
> user.email=<lowercase>@systemic.engineer commit -m ...`."

Mara currently commits via Reed's SSH key + git -c user.name/user.email
overrides. **The signature verifies as Reed; the attribution reads
Mara.** This is the current substrate state.

**Alex-adjudication A-D11 (LOAD-BEARING; possibly Alex-only):**

- **Path α:** Mara continues to sign with Reed's SSH key; commits carry
  `user.name=Mara user.email=mara@systemic.engineer` but SSH signature
  verifies as reed@systemic.engineer. Landing A adds no new key
  ceremony. Trust root is Reed. Simplest; matches current behavior;
  works today.
- **Path β:** Landing A extends `~/.reed/bin/materialize` to also
  generate a git-commit-signing key (either the same ed25519 keypair,
  or a separate one) and provisions it into `~/.mara/.git/`. Mara
  signs her own commits. Trust root is Reed's CA (the peer's key is a
  cert signed by Reed).
- **Path γ:** Mara gets her own root key generated on first materialize;
  Alex signs Mara's key with Alex's CA; Reed's CA is bypassed. Trust
  root is Alex. Symmetric with the human/AI eye-level Landing 3 claim;
  each Pack peer has independent identity.
- **Path δ:** Defer entirely; Landing A ships without commit-signing
  for Mara; Mara-authored commits continue as `user.name=Mara + Reed's
  SSH signature`. Landing B+ tackles commit-signing separately.

**Recommendation:** Path δ (defer) for Landing A. Landing A's scope is
projection/harvest/resume of peer state; commit-signing is orthogonal
and load-bearing enough to warrant its own arc. Path β is the natural
follow-on if Alex adjudicates YES; matches the materialize script's
existing shape.

**Substrate-decl gap:** `subject_instance.ssh_signature_fingerprint`
is a `ref` field; the substrate accepts any fingerprint. `ssh_witness_valid`
bilateral verifies signature-matches-fingerprint per landed subject.mirror
discipline. Whether the fingerprint IS Reed's or IS Mara's-signed-by-
Reed's-CA is orthogonal to the bilateral; the bilateral checks
signature-validity, not signature-source. So Path α/β/γ all discharge
`ssh_witness_valid` correctly; the difference is downstream trust
semantics.

---

## D12. `mirror mara` command precedent

**Read:** `bootstrap/src/lib.rs` (235KB, mirror-CLI dispatcher) +
`mirror.spec` cli-block (`command peer { command beam { ... } }`).

**Verdict: `mirror mara` follows the FLAT-SUBCOMMAND pattern
(compile, kintsugi, shatter, init, recall, spawn, beam, index) with
optional 2-tick migration to recursive-command (`mirror peer beam`).**

**Landed subcommands per bootstrap/src/lib.rs help text (line 633):**
```
compile [--strict] <file>
craft [--strict] [--target-kind <crystal|binary>] <target>
kintsugi [--ci [--out ...]] [--shatter N] <file|dir>
init [--install-hooks] <repo-path>
recall <spec-dir>
spawn [--hello-world] [--mission <mission-file>] <peer-home>
shatter <oid> <out> [--target ...]
```

**Plus landed via mirror.spec cli-block:**
- `mirror beam <mission>` (anonymous variant)
- `mirror peer beam <peer_home> [--hello-world] [--mission ~f]
    [--song ~f] [--dance-with ~f] [--deploy-to ~f] [--emit-crystal]`
- `mirror peer contribute <peer_home> --target <shard>`
- `mirror index <path> [--fiedler] [--full_profile]`

**Grammar landed at Tick 1 of @mirror/lens/cli (`fe82500`):** depth-2
recursive-command form (`command X { command Y { ... } }`) is
substrate-decl'd. `mirror peer beam` reads as command-nested-in-command
directly.

**Precedent for `mirror mara`:**

Option A (flat alias): Add `command mara { arg mission: ~f = ...; ... }`
to mirror.spec cli-block. Dispatches to
`crate::peer::spawn_from_home("~/.mara")`. Simple; symmetric with the
prior verbs (compile/kintsugi/etc.); low grammar cost.

Option B (recursive alias): Add `command mara` as a sub-command of
`command peer` (i.e., `mirror peer mara [...]`). More consistent with
the `mirror peer beam` recursive pattern; but adds a keyword-nesting
level that reads as "peer.mara.<verb>" which is less intuitive than
"mirror mara <verb>".

Option C (no alias; canonical only): `mirror peer beam ~peer'~/.mara'`
stays as the invocation. The word "mara" never appears at CLI verb
altitude. Substrate-honest; but Alex's directive explicitly named
"`mirror mara`" as the target verb.

**Recommendation Option A.** Alex's directive ("we spawn the first
`mirror mara` on a `mirror roomba` finding") reads as CLI-verb
`mirror mara` (not `mirror peer mara`). Land as flat alias in
mirror.spec cli-block; five sibling aliases (`mirror mara`, `mirror
seam`, `mirror taut`, `mirror glint`, `mirror reed`) or fewer
depending on which peers are ready to spawn from a home repo. Aliases
dispatch through the same `@mirror/peer/beam.beam` substrate action;
the CLI-surface variance is thin sugar.

**Two-tick discipline candidate:** Land `mirror mara` as alias tick 1;
in tick 2, evaluate whether all five aliases live or whether
`mirror peer beam ~peer'<home>'` is the sufficient surface and the
aliases retire. Alex's ergonomic preference determines the two-tick
outcome.

---

## D13. Roomba-finding-to-Mara-task conversion pattern

**Read:** `docs/loop/CURRENT.md` §"@roomba composition" +
`shards/subject.mirror` (@subject/queue mentioned in A32 forward-
promise per L5+ narrative in CURRENT.md line 109) +
`/Users/reed/identity/tasks/README.md` (implied pattern).

**Verdict: PATTERN IS PARTIALLY LANDED. Roomba emits findings via
song/beat; @subject/queue is FORWARD-PROMISED as A32 (Landing 5+);
Reed's tasks/pending/ is the INTERIM pattern.**

Landed:

- `@roomba.walk` at `bootstrap/src/roomba.rs` emits trajectory
  observations (tension, pain, knife verdicts) per Scope A ship.
- `@song/beat` at `shards/song/beat.mirror` (Mara `94e55eb`, 49.7KB)
  IS the emission substrate — roomba's bumps into complexity emit
  song beats.
- `@kintsugi` consumes @song and decides knife-vs-spawn-@peer per
  Alex's 2026-07-14 morning composition.

Forward-promised:

- **A32 @subject/queue** — subject-absent commutator queue for
  cross-tick, cross-peer task routing. Not yet minted; Reed's
  `~/.reed/tasks/pending/` filesystem-based queue is the INTERIM
  substrate at operational altitude.

**Landing A composition path:**

```
@roomba.walk emits @song beat at position p
  → beat carries {location: oid, tension: f64, complexity: verdict}
  → @kintsugi decides: spawn @peer at K+1
  → @peer resolution: which peer answers this complexity altitude?
     (Mara for canonical-spec-writing; Seam for adversarial review;
     Taut for grep-first drift scouting; Glint for essay cascade)
  → convert beat → task envelope
     ({beat_oid, peer_target, mission, visibility_scope})
  → write task envelope to peer_home/tasks/pending/<oid>.json
  → mirror mara [--resume] reads tasks/pending/ on spawn; discharges
     next task; writes result back to bauchladen; task moves to
     tasks/done/ (or Reed's `just ship` pattern equivalent)
```

**Substrate decl needed for the beat→task conversion:** either

(a) `@peer/persistence.route(beat, peer_roster) -> task_envelope` at
Landing A altitude (one operation); OR

(b) forward-promise until @subject/queue lands (A32); Landing A ships
with a lightweight `peer_home/tasks/pending/` filesystem convention
matching Reed's operational pattern (no substrate-decl'd task
envelope yet).

**Recommendation (b):** Landing A projects tasks/pending/ as
filesystem convention; A32 @subject/queue lands the substrate-decl
task-envelope carrier in a subsequent tick. Two-tick discipline
preserves Landing A's minimality.

**Sub-recommendation:** `mirror roomba walk` emits beats through
existing `@song/beat`; when a beat's kintsugi verdict says "spawn
peer", the roomba runtime writes a `<beat_oid>.md` file to
`~/.mara/tasks/pending/` (or the appropriate peer's home). The
first `mirror mara` invocation reads pending/, picks a task, resumes.
Empirical roundtrip closes at that point.

---

## Composition graph — @peer/persistence family tree

```
@peer  (family-root; landed at shards/peer.mirror)
├── type peer = { home: ref, lead_of: ref, kind: kind }   [LANDED]
├── kind = human | agent | substrate                      [LANDED]
└── load(dir, p) -> imperfect(peer, ref, ref)             [LANDED body: \]
    │
    └── COMPOSES with @io/git for home-repo resolution    [LANDED]

@peer/persistence  (NEW species; Landing A)
├── in @peer                                              [import; landed parent]
├── in @bauchladen                                        [import; landed]
├── in @subject/visibility                                [import; landed]
├── in @io/git                                            [import; landed]
├── in @mirror/store                                      [import; landed]
├── in @mirror/lens/unix                                  [import; landed]
├── in @kintsugi/store/git                                [import; landed]
│
├── project(p: peer) -> imperfect(home_dir, ref, ref)     [NEW body \]
│    │  writes bauchladen crystals to p.home/.bauchladen/;
│    │  respects visibility_scope; anti-elevation-preserving
│    └── requires visibility_witnessing over all crystals
│
├── harvest(p: peer) -> imperfect(bauchladen, ref, ref)   [NEW body \]
│    │  reads p.home filesystem; content-addresses each crystal;
│    │  reconstructs splinter_graph; verifies oid discipline
│    └── requires filesystem_readable(p.home)
│
├── resume(p: peer) -> imperfect(@song, ref, ref)         [NEW body \]
│    │  loads harvested bauchladen; constructs mission from
│    │  most-recent-tasks/pending/ entry; discharges through
│    │  @mirror/peer/beam.beam with resumed context
│    └── requires bauchladen_witnessing on harvested crystals
│
├── sync(p: peer) -> verdict                              [NEW body \]
│    │  bilateral: does p.home filesystem match p's bauchladen
│    │  content-addressed state? emit fracture proposals via
│    │  @kintsugi if drift detected
│    └── discharges Pass iff harvest(p) verifies against
│        p's @mirror/store bauchladen at same content-address
│
├── home_of(si: subject_instance) -> option<peer>         [NEW body \]
│    │  resolves subject_instance (AI Pack peer) to its @peer.peer
│    │  value via actor_kind = ai_a discrimination; returns None
│    │  for human_a / substrate_a
│    └── replaces D5 field-extension proposal
│
└── bilateral: persistence_coherent(p: peer, perturbation)
     composes home_readable + bauchladen_verifiable +
     visibility_scoped_correctly + last_harvest_recent

@mirror/peer/beam  (LANDED at shards/mirror/peer/beam.mirror)
└── beam(r: mirror_peer_beam_request, p) -> @song         [LANDED body: \]
    │
    └── requires peer_well_known(r.target, p)             [LANDED]
    │
    └── NEW: variant dispatch on --resume flag
              (flag-guard pattern per Rung 1-6' precedent)
              --resume dispatches to
              @peer/persistence.resume(r.target)

mirror.spec cli-block  (LANDED)
├── command peer { command beam { ... } }                 [LANDED]
├── NEW: flag resume: bool = false on peer.beam
└── NEW: command mara { flag mission: ~f = ...; ... }    [Landing A ALIAS]
     (dispatches to @peer/persistence.resume(~peer'~/.mara'))

bootstrap/src/persistence.rs                              [NEW module]
├── fn cmd_persistence_project(peer_home: &str, ctx: &Ctx) -> i32
├── fn cmd_persistence_harvest(peer_home: &str, ctx: &Ctx) -> i32
├── fn cmd_persistence_resume(peer_home: &str, ctx: &Ctx) -> i32
└── fn cmd_persistence_sync(peer_home: &str, ctx: &Ctx) -> i32

~/.mara/  (NEW filesystem home)
├── .bauchladen/                                          [projection target]
├── mara.spec                                             [migrates from ~/.reed/peers/mara.spec]
├── mara.mirror                                           [analog to reed.mirror]
├── 00-NARRATIVE-MARA.md                                  [optional; Mara's scaffold]
├── tasks/pending/                                        [D13 filesystem queue]
├── visibility/{private,protected,public}/                [D9 discipline]
└── .git/                                                 [git-tracked per Reed pattern]
```

---

## Alex-adjudications surfaced

Prioritized by load-bearing weight.

**A-D11 (SSH signing; possibly Alex-only, LOAD-BEARING):** Which of
Path α/β/γ/δ (see D11 for full text) for Mara's commit-signing? Recommend
Path δ (defer) for Landing A; treat commit-signing as a separate arc.

**A-D1 (@peer/persistence naming):** Confirm `@peer/persistence` (vs
`@peer/projection`). Recommend `persistence` — captures between-spawns
semantics; distinct from landed `@code/metalogue/materialize` (which
would COLLIDE if reused); distinct from prism-verb `project`.

**A-D3 (harvest naming):** Confirm `harvest` (filesystem verb) vs
`pull_back` (categorical). Recommend `harvest` for approachability;
both discharge the same shape.

**A-D5 (subject_instance.home field extension):** Do we add
`home: option<ref>` as 8th field on subject_instance (uniformity), or
compose via `@peer/persistence.home_of(si) -> option<peer>` (substrate-
pull correctness)? Recommend the composition path — no carrier extension.

**A-D8 (@spectral/signature shard mint prerequisite):** Pull
`shards/spectral/signature.mirror` forward as Landing A prerequisite,
or ship Landing A with json-file placeholder in .bauchladen/refs/?
Recommend the placeholder path.

**A-D12 (`mirror mara` CLI verb form):** Flat alias
(`mirror mara [--resume]`) vs recursive (`mirror peer mara`) vs no
alias (canonical `mirror peer beam ~peer'~/.mara'` only)? Recommend
flat alias per Alex's directive naming "`mirror mara`" directly.

**A-D13 (roomba-finding-to-task):** Land `@peer/persistence.route(beat)`
at Landing A altitude, or forward-promise until A32 `@subject/queue`
lands and use filesystem convention interim? Recommend filesystem
convention (tasks/pending/) matching Reed's operational pattern.

---

## Mara #next composition readiness

**Verdict: MARA HAS ENOUGH SUBSTRATE TO COMPOSE.** Landing A is a
species-shard mint + minimal Rust runtime, not new family-root
territory. Substrate coverage:

| Composition need                            | Substrate status | Source                                        |
|---------------------------------------------|------------------|-----------------------------------------------|
| peer.home field                             | LANDED           | shards/peer.mirror:122                        |
| home paths named (~/.mara, ~/.seam, etc.)   | LANDED           | mirror.spec:57-62                             |
| ~peer'<home>' cli resolution                | LANDED           | shards/peer.mirror + @io/git G1               |
| mirror peer beam cli verb                   | LANDED           | mirror.spec + cmd_peer_beam                   |
| @bauchladen crystal store                   | LANDED           | shards/bauchladen.mirror + @mirror/store      |
| peer_home/.bauchladen/ ops discipline       | LANDED           | shards/cyberpunk.mirror:240                   |
| per-home FUSE mount discipline              | LANDED           | shards/mirror/lens/unix/fuse.mirror           |
| @subject/visibility {private,protected,pub} | LANDED           | shards/subject/visibility.mirror              |
| @kintsugi/consent.query_phi                 | LANDED           | shards/kintsugi/consent.mirror                |
| @kintsugi/store/git.commit_as_fold          | LANDED           | shards/kintsugi/store/git.mirror              |
| @mirror/store.write / read / verify         | LANDED           | shards/mirror/store.mirror                    |
| @mirror/store.impacted_by (rebase walk)     | LANDED           | shards/mirror/store.mirror (N4)               |
| @mirror/index (peer_home indexing)          | LANDED           | shards/mirror/index.mirror                    |
| subject_instance (7 fields)                 | LANDED           | shards/subject.mirror:343-351                 |
| @eigenboard inference basis                 | LANDED           | shards/eigenboard.mirror                      |
| @spectral/signature spec                    | SPEC LANDED / shard forward-promised | docs/specs/gift-and-mirror-reflection.md §12 |
| ~/.reed/ empirical ancestor                 | LANDED (git repo) | /Users/reed/identity/                         |
| ~/.reed/bin/materialize (SSH ceremony)      | LANDED (shell script) | /Users/reed/identity/bin/materialize      |
| ~/.reed/peers/mara.spec proto-spec          | LANDED           | /Users/reed/identity/peers/mara.spec          |
| @pack.spawn primitive                       | LANDED           | shards/pack.mirror:263                        |
| Pack peer identity roster (5 eye-level)     | LANDED           | shards/subject.mirror:116-121, shards/pack.mirror:189 |

**Net-new substrate for Landing A:**

1. `shards/peer/persistence.mirror` (or equivalent naming path)
   substrate-decl'd species-shard: project / harvest / resume / sync /
   home_of actions + persistence_coherent bilateral. ~200-400 LOC.
2. `bootstrap/src/persistence.rs` Rust runtime: filesystem
   materialization of bauchladen crystals; harvest reader;
   resume-mission construction. ~500-1000 LOC.
3. Extension to `cmd_peer_beam` in `bootstrap/src/lib.rs`: `--resume`
   flag guard dispatching to persistence.resume. ~50 LOC.
4. `mirror.spec` cli-block additions: `flag resume: bool` on peer.beam;
   optional `command mara { ... }` alias. ~15 LOC.
5. Alex-adjudication A-D11 answered before Landing B (SSH signing).

**Hard collisions:**

- `@peer/materialize` conflicts with `@code/metalogue/materialize`.
  MUST NOT reuse `materialize` verb at peer altitude. Use
  `@peer/persistence` per A-D1.

**No blocking substrate gaps.** Landing A is composition-ready.

---

## Substrate-honest hedges

1. This scout did NOT read every consumer of `subject_instance` in the
   ~466 landed shards. The 7-field record extension claim in D5 is
   BOUNDED to the consumers grepped
   (`shards/subject.mirror`, `shards/eigenboard.mirror`,
   `shards/gift/lens.mirror`, `shards/subject/visibility.mirror`).
   Seam Phase D adjudication would sweep all consumers before landing.
2. `shards/spectral/signature.mirror` shard was searched for but not
   found; the claim "spec landed, shard forward-promised" is inferred
   from 7 shards using `in @spectral/signature` without a corresponding
   file at `shards/spectral/signature.mirror`. A future scout should
   verify (either the shard IS landed under a different path OR it
   IS genuinely forward-promised per Landing 6+).
3. Reed's `~/.reed/bin/materialize` script signs with Reed's CA
   (`~/.ssh/id_ed25519`). The script existed prior to this scout;
   its ACL ("Reed and Alex only") is enforced at runtime, not
   substrate-decl. Landing A does not modify this script but does
   depend on the ceremony it discharges (or on Alex-adjudicated
   Path α/β/γ/δ per D11).
4. `mirror.spec:57-62` names four peer homes as pack members
   (~/.mara, ~/.seam, ~/.taut, ~/.glint). These directories DO NOT
   YET EXIST on the local filesystem. Landing A creates them. If any
   of the four peers has been given an alternate home path outside
   this scout's read, that would supersede the mirror.spec claim.
5. The `mirror mara` CLI-verb form (D12 recommendation Option A)
   assumes single-word alias per Alex's directive; if Alex prefers
   the recursive form or no alias, redirect there.
6. Landing A scope excludes commit-signing (per A-D11 recommendation).
   If Alex wants Mara commits with Mara's own SSH signature, Landing A
   scope grows and blocks on A-D11 resolution.
7. The composition graph above shows Landing A as ONE tick; realistic
   sequencing likely takes 2-3 ticks (species-shard mint → Rust
   runtime → cli integration → empirical spawn). Substrate-honest.

---

## Path-namespace property

This scout at
`docs/scouts/2026-07-14-taut-peer-persistence-scout.md` follows the
Taut scout pattern (`docs/scouts/YYYY-MM-DD-taut-<scope>.md`)
established by prior 2026-07-14 scouts
(gift-and-mirror-reflection, landing-3-payforward-lens-ancestors-
peers, landing-4-bauchladen-visibility-eigenboard,
landing-5-shard-mints, roomba-substrate-walker,
subject-family-root, subject-presence-interaction-loop).

Read-only. Reed commits as Taut per project CLAUDE.md attribution
discipline (`Taut <taut@systemic.engineer>`).
