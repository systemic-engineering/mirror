---
date: 2026-07-15
author: Mara
scope: Shape proposal for @secrets shard-decl mint. Discharges the @sheaf forward-promise pointing at `@secrets` + `@secrets/sops` (15 references in `shards/subject/visibility/sheaf.mirror` at d1ce901; from Alex 2026-07-14 SSH-signing message "@secrets prism and @secrets/sops to project visibility/private stuff onto disk through the Peers key"). Enumerates four substrate-honest candidates for @secrets; recommends ONE; hands to Seam for adjudication. Alex-adjudication residue named if Seam refuses.
status: scout — shape proposal (design-choice artifact, NOT canonical spec)
companion:
  - shards/subject/visibility/sheaf.mirror
  - docs/scouts/2026-07-15-mara-sheaf-shape-proposal.md
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md
  - shards/io.mirror
  - shards/subject.mirror
  - shards/subject/visibility.mirror
  - shards/subject/visibility/private.mirror
  - shards/torus.mirror
  - shards/peer.mirror
  - shards/gift/lens.mirror
---

# Mara — @secrets shape proposal (four candidates + one recommendation)

*Design-choice artifact for Seam adjudication. Four substrate-
honest candidates (three primary + one sub-species) for @secrets;
one recommendation. Companion to `docs/scouts/2026-07-15-mara-
sheaf-shape-proposal.md` — @sheaf carries the ACL-restriction
algebra; @secrets projects peer-key-gated visibility crystals
onto disk through sops-style secrets management.*

---

## §0 Framing

### §0.1 The question this scout asks

Alex Wolf named `@secrets` and `@secrets/sops` in a compacted-
session message alongside the @sheaf design intent (verbatim,
re-cited from `shards/subject/visibility/sheaf.mirror` 30-32):

> "@secrets prism and @secrets/sops to project visibility/private
> stuff onto disk through the Peers key."

@sheaf landed at Mara `d1ce901` (species-under-@subject/
visibility per Seam Phase D-cascade Candidate 2). `@secrets` is
referenced 15 times in that shard as forward-composition surface.
**@secrets does NOT exist as a landed shard.** The forward-
promise is now substrate-honest to discharge.

Two structural facts constrain this scout:

1. **Alex named `@secrets/sops` as a SPECIES.** Decisive grammar:
   @secrets is a PARENT altitude admitting species. Two parent
   shapes: family-root OR sub-family-root under a landed family-
   root.

2. **Alex called it "@secrets prism."** Per
   `[[architecture-prism-as-trait-as-everything]]`, EVERY
   substrate-decl carrier IS a prism (family-roots + species +
   sub-species all declare a `prism X { ... }` block). "Prism"
   here is DESCRIPTIVE (naming the substrate-decl form), not a
   THIRD structural category. See §5.3.

### §0.2 What @secrets must carry

Directly from Alex 2026-07-14 SSH-signing intent (verbatim from
sheaf.mirror 21-32):

> "Each peer has their own key in the private part of their
> visibility. NOT projected into the git state and instead stays
> .git/mirror side. Only connected through Fractal.Lens. A
> pointer. Not the thing."
>
> "@secrets prism and @secrets/sops to project visibility/private
> stuff onto disk through the Peers key."

Five load-bearing carriers:

1. **Peer-key-material at `.git/mirror` boundary.** Keys live
   OUTSIDE the git working tree. Key bytes are opaque per
   `shards/io.mirror` 67-73 ("BODY of the call is opaque ...
   what makes @io the LEGITIMATE non-mirror surface").

2. **Projection primitive: sheaf_restriction × peer_key →
   materialized_bytes.** Composes @sheaf.section_at
   (`section_at_stalk.projected_value`, a ref) through key-
   gated crypto to disk-side bytes.

3. **Fractal.Lens pointer discipline.** The peer_ref in a
   sheaf_restriction is a POINTER; @secrets is where the
   pointer DEREFERENCES to bytes via key-gated crypto.

4. **@io boundary composition.** @secrets discharges at
   `@io/crypto` (hash + sig + AEAD; `shards/io.mirror` 161-162)
   + `@io/fs` (POSIX filesystem; forward-promised at 189-191).

5. **@secrets/sops as species.** SOPS (Mozilla 2017+; PGP/age/
   KMS-encrypted YAML/JSON, in-place edits, team-member access
   lists) is the FIRST concrete projection method Alex named.
   Future sibling species: age, vault, 1password.

### §0.3 Substrate-already-had-the-word evidence

Grep for @secrets-adjacent language:

1. **`shards/subject/visibility/sheaf.mirror` (d1ce901)** — 15
   references naming this species at 140-150: "@secrets ...
   compose through @io at the projection boundary."

2. **`shards/io.mirror` 37-40 (AGENTS.md Glass Wall verbatim):**
   "@io is the substrate's only legitimate non-mirror surface.
   Any grammar that isn't mirror — Rust, Python, raw bytes,
   foreign blobs, vendor SDKs — must be under @io." SOPS IS
   vendor tooling; peer keys ARE raw bytes; ciphertext files
   ARE foreign blobs.

3. **`shards/io.mirror` 161-162 (@io/crypto):** "hash + sig +
   AEAD over bytes. Vendor surface (sha2, age, ssh-key)." age
   IS one of SOPS's primary key backends.

4. **`shards/io.mirror` 189-191 (@io/fs; forward-promised):**
   "POSIX filesystem surface. Lands when `@mirror/store`'s git-
   backed implementation lifts its disk surface." @secrets IS
   the consumer this promise names.

5. **`shards/subject/visibility/private.mirror` (Reed ancestry):**
   Reed's ~/.reed/visibility/private/ has operated the explicit-
   consent discipline since ~2026-02-07; Alex's "@secrets ... to
   project visibility/private stuff onto disk" IS the projection
   Landing 4 substrate-lifted at @subject/visibility/private.

6. **`shards/gift/lens.mirror`** — sibling design intent phase
   with @secrets (Alex 2026-07-14 same message week); @gift/lens
   carries WHO composed, @secrets carries HOW projection
   materializes. Orthogonal.

**Consequence.** The @secrets altitude is substrate-adjacent-
decl'd across five carriers; missing is a dischargeable shard.
The four candidates differ in WHERE the shard mounts.

### §0.4 Discipline honored

Substrate-already-had-the-word ENFORCED (@onto refusal); no-
Rust-extension-shortcut ENFORCED (body work composes over @io);
composition-only surface; two-tick discipline (readable over
foundational); Rice-safe (reads byte-visible state).

---

## §1 Candidate 1 — @secrets as family-root

**Path.** `shards/secrets.mirror`. Peer to @torus, @subject,
@kintsugi, @bauchladen, @autopoietic, @fate, @glue, @peer,
@gift, @io. Species: `@secrets/sops` + future
`@secrets/age`, `@secrets/vault`.

**Carriers.** `secret_projection`, `key_material_ref`,
`sops_file_ref`. **Actions.** `project`, `materialize`,
`key_admits`.

### 1.1 Substrate-already-had-the-word check

Defeater carriers @secrets-family-root would cross:

- **@io family-root discipline** (`shards/io.mirror` 37-40
  verbatim): "vendor SDKs must be under @io." SOPS IS vendor.
  Family-root @secrets DUPLICATES what @io family-root already
  covers.
- **@io/crypto** (162): age IS a SOPS backend; already at @io
  species altitude.

### 1.2 Load-bearing altitude claim

"Peer-key-gated projection as substrate-primary altitude."
Against: the projection ALREADY composes @io/crypto + @io/fs —
both species-under-@io. Family-root @secrets claims altitude
AT-OR-ABOVE @io for what IS an @io crossing (per §0.2 carrier
4). The altitude the family-root claims doesn't exist above @io;
it exists AS @io. @onto refusal pattern.

### 1.3 Ancestors + bound

sheaf.mirror (d1ce901); io.mirror 37-40, 161-162, 189-191; Alex
2026-07-14; SOPS/age/Vault; Reed memory @onto-refusal.
**Coverage:** full but @io-crossing — imports @io/crypto + @io/
fs and re-exports as if @secrets-native. **@sheaf composition:**
crosses family-root twice (sheaf → secrets → io internally).
**Bound:** ADDS peer-to-@io family-root altitude duplicating
@io's scope.

---

## §2 Candidate 2 — @io/secrets (species under @io)

**Path.** `shards/io/secrets.mirror`. Species under @io, sibling
to @io/cargo, @io/git, @io/oci, @io/stagefreight, @io/algebra,
@io/crypto (floor), @io/fs (forward-promised). Sub-species:
`@io/secrets/sops` at `shards/io/secrets/sops.mirror`; future
`@io/secrets/age`, `@io/secrets/vault`, `@io/secrets/1password`.

**Prism.** `prism @io/secrets { focus/project/split/shift/settle
secret_projection }`.

**Carriers.**
- `secret_projection` (peer-key-gated projection to disk)
- `key_material_ref` (pointer into .git/mirror-side storage)
- `ciphertext_ref` (on-disk encrypted blob; opaque bytes)

**Actions.**
- `project(sr: sheaf_restriction, k: key_material_ref) ->
  secret_projection`
- `materialize(sp: secret_projection, path: ref) ->
  imperfect(disk_write_verdict, ref, ref)`
- `key_admits(k: key_material_ref, sr: sheaf_restriction) ->
  verdict`
- `round_trip(sp: secret_projection) -> verdict` (encrypt-then-
  decrypt returns same projected_value)

### 2.1 Substrate-already-had-the-word check

Sibling landed @io species: @io/cargo (2026-06-05, first mirror-
altitude @io species; the precedent), @io/git, @io/oci, @io/
stagefreight, @io/algebra, @io/crypto (floor), @io/fs (forward-
promised; pulled by this landing).

**@io/crypto carries AEAD primitives.** **@io/fs will carry
POSIX write.** @io/secrets carries the KEY-GATED COMPOSITION of
both with @sheaf sections — a composition not first-class at any
landed species altitude.

The @io family-root discipline (37-40 verbatim) SPECIFIES this
location. Every load-bearing element of the @secrets discipline
is what the @io family-root declares its scope covers.

### 2.2 Load-bearing altitude claim

"Peer-key-gated projection-onto-disk at @io species altitude,
composing @io/crypto (key algebra) + @io/fs (disk write) with
@sheaf sections."

**Arc-2.3 peer_persistence.rs collapse:**

```
peer_visibility_materialize(peer, home, crystal, target_path):
  ACL_peer := pack.members[peer.name]                          # existing
  sr := @subject/visibility/sheaf.acl_project(F_home, ACL_peer) # d1ce901
  section := @subject/visibility/sheaf.section_at(sr, crystal)  # d1ce901
  peer_key := @io/secrets.key_material_ref(peer, .git/mirror)   # NEW
  sp := @io/secrets.project(section, peer_key)                  # NEW
  @io/secrets.materialize(sp, target_path)                      # NEW → disk
```

Fractal.Lens pointer discipline preserved: `peer_key` is a REF
(per @gift/lens shift pattern); dereferences at @io boundary;
substrate never carries key bytes in-tree. `.git/mirror`
boundary honored via `key_material_ref` as a ref whose
resolution crosses the @io boundary per @io's "irreducibly
opaque surface" discipline.

### 2.3 Ancestor citations

sheaf.mirror (d1ce901; 140-150 forward-promise); io.mirror 37-
40 (Glass Wall verbatim); 161-162 (@io/crypto, age); 189-191
(@io/fs forward-promised); io/cargo.mirror (species-decl
precedent); io/git.mirror + io/oci.mirror (sibling pattern);
Alex 2026-07-14 (sheaf.mirror 21-32); SOPS/age/Vault; Reed
memories @onto-refusal + no-Rust-extension-shortcut.

### 2.4 Alex SSH-signing design-intent coverage

**Direct.** All five §0.2 carriers land at @io species altitude
with zero family-root crossings:

1. **Peer-key .git/mirror boundary:** `key_material_ref` field
   resolves through @io/git plumbing (landed sibling); never
   enters working tree. @io's opacity discipline (67-73) IS the
   substrate-decl form of "key stays .git/mirror-side."
2. **Projection primitive:** `project(sr, k)` composes @sheaf's
   section_at_stalk.projected_value with @io/crypto's AEAD (age-
   shaped) into a secret_projection.
3. **Fractal.Lens pointer discipline:** `key_material_ref` IS
   the pointer; `secret_projection` after `materialize` IS the
   thing (on disk). @gift/lens's shift composes ORTHOGONALLY —
   observes WHO composed; @io/secrets projects HOW section
   materializes.
4. **@io boundary composition:** Native @io species; Bateson
   form/substance partition (io.mirror 259-278) puts @secrets
   at substance-side boundary. `imperfect<a, e, l>` return per
   @io discipline (74-80).
5. **@secrets/sops as species:** Direct sub-species at
   `shards/io/secrets/sops.mirror` (§2.7).

### 2.5 @sheaf composition coverage

Full. Zero family-root crossings:

```
@subject/visibility/sheaf.acl_project(F_home, peer.acl)
  → sheaf_restriction
@subject/visibility/sheaf.section_at(sr, crystal)
  → section_at_stalk
@io/secrets.project(section_at_stalk, key_material_ref)
  → secret_projection
@io/secrets.materialize(secret_projection, target_path)
  → imperfect(disk_write_verdict, ref, ref)
```

`projected_value` ref (sheaf.mirror 302) is the substrate-decl
handoff surface. Handoff preserves ref-equality identity;
@io/secrets discharges actual bytes at @io boundary.

### 2.6 Substrate-honest bound

Does NOT: mint family-root; carry crypto/fs primitives
(@io/crypto + @io/fs carry); carry peer-key material
(.git/mirror-side; shard carries REF); carry sops parsing (sub-
species); displace @sheaf or @gift/lens. ADDS the peer-key-gated
projection carrier at the altitude @io family-root explicitly
substrate-decls.

### 2.7 @io/secrets/sops sub-species (Candidate 2's SOPS landing)

**Path.** `shards/io/secrets/sops.mirror`. Sub-species under
`@io/secrets` at the vendor-tool altitude. Sibling to future
`@io/secrets/age`, `@io/secrets/vault`, `@io/secrets/1password`.

**Carriers.** `sops_file_ref` (on-disk encrypted YAML/JSON;
opaque bytes per @io); `sops_key_group` (SOPS-native public-key
recipient list; refs into @io/crypto for concrete age/pgp/kms
recipients).

**Actions.** `sops_encrypt(section, key_group) -> sops_file_
ref`; `sops_decrypt(sops_file_ref, key) -> section` (round-trips
per @io/secrets.round_trip discipline); `sops_key_group_from_
sheaf_restriction(sr) -> sops_key_group` (composition bridge:
@sheaf ACL projects into SOPS key-group ACL).

**Bound.** Does NOT re-declare YAML/JSON (composes over
@mirror/data/yaml + @mirror/data/json); does NOT re-declare
age/pgp/kms crypto (composes over @io/crypto); does NOT carry
the sops binary itself (opaque per @io; carries CONTRACT).

Ancestor: SOPS project (Mozilla 2017; getsops/sops). Species
specialization pattern @io/cargo established: declare vendor-
tool contract; runtime opaque-executes.

---

## §3 Candidate 3 — @subject/visibility/secrets (species under @subject/visibility)

**Path.** `shards/subject/visibility/secrets.mirror`. Species
sibling to `@subject/visibility/{private, protected, public,
sheaf}`.

**Carriers.** `secret_binding` (subject-scoped binding between
peer's ACL and key material); `peer_key_ref` (pointer into
subject_instance.ssh_signature_fingerprint per subject.mirror
two-witness discipline).

**Actions.** `bind`, `project_through` (dispatches to
@io/secrets or @io/secrets/sops for actual disk write).

### 3.1 Substrate-already-had-the-word check

Sibling landed species carry the visibility-SCOPE dimension +
sheaf-restriction. NONE substrate-decl the KEY-BINDING between
subject_instance's ssh_signature_fingerprint and the projection
mechanism.

**BUT:** the subject-key binding IS ALREADY at `shards/subject.
mirror` (`subject_instance.ssh_signature_fingerprint`). And
@sheaf.sheaf_restriction.peer_ref ALREADY resolves through
@subject to that fingerprint. The binding EXISTS at @subject
family-root altitude, not at @subject/visibility species altitude.

Placing @secrets as species under @subject/visibility asserts
key-gated projection is a VISIBILITY-SPECIES concern. It is not.
Visibility species carry consent scope, ACL structure, sheaf-
restriction; they DO NOT carry crypto or disk-write. The
projection is @io-native, not visibility-native.

### 3.2 Load-bearing altitude claim + bound

"Subject-scoped key-binding to sheaf-restriction, first-class at
species altitude." **Against:** projection is @io-crossing;
placing under @subject/visibility puts @io concerns at subject
altitude and forces every Arc-2.3 composition to cross
@subject → @io INTERNALLY — the inverse of @sheaf Candidate 2's
Reason 3 (subject-scoped concerns at subject altitude, @io-
scoped concerns at @io altitude).

**Bound:** ADDS subject-scoped key-binding that duplicates the
subject_instance-to-key binding already at @subject family-root.

### 3.3 Ancestor citations

Same as Candidate 2, plus: subject.mirror (two-witness
discipline; ssh_signature_fingerprint); subject/visibility/
private.mirror (sibling); subject/visibility/sheaf.mirror
(sibling; ACL-restriction consumer).

---

## §4 Candidate 4 (optional) — @sheaf/secrets (sub-species under @sheaf itself)

**Path.** `shards/subject/visibility/sheaf/secrets.mirror`.

### 4.1 Substrate-already-had-the-word check + bound

**Immediate defeater.** @sheaf carries sheaf-restriction algebra;
@sheaf/secrets would carry crypto + disk-write UNDER sheaf-
restriction. Composition arrow inverted: crypto/disk-write don't
restrict TO sheaves; they compose OVER sheaf sections. Sheaf.
mirror 140-150 makes this explicit: "@secrets and @secrets/sops
compose through @io at the projection boundary" — arrow flows
@sheaf → @secrets → @io, not @sheaf → @sheaf/secrets → @io.

REJECTED at §0.4 discipline check. Included for enumeration
completeness; not a live candidate.

---

## §5 Recommendation — Candidate 2 (@io/secrets, with @io/secrets/sops as first sub-species)

### §5.1 The recommendation

**Mint `@io/secrets` as species under `@io` at `shards/io/
secrets.mirror`.**

Sibling to @io/cargo, @io/git, @io/oci, @io/stagefreight,
@io/algebra, @io/crypto (floor), @io/fs (forward-promised;
lifted by this landing).

**Mint `@io/secrets/sops` as sub-species under `@io/secrets` at
`shards/io/secrets/sops.mirror`** — the SOPS-specific vendor
projection Alex named. First sibling of future `@io/secrets/
age`, `@io/secrets/vault`, `@io/secrets/1password`.

Not family-root. Not under `@subject/visibility`. Not under
`@sheaf`.

### §5.2 Substrate-honest justification (three reasons)

**Reason 1 — substrate-already-had-the-word.** io.mirror 37-40
verbatim (AGENTS.md Glass Wall): "@io is the substrate's only
legitimate non-mirror surface. Any grammar that isn't mirror —
Rust, Python, raw bytes, foreign blobs, vendor SDKs — must be
under @io." SOPS is Python/YAML/vendor; peer keys are raw
bytes; ciphertext files are foreign blobs; disk-write is POSIX
syscall. Every load-bearing element of @secrets discipline is
what @io family-root explicitly covers. Candidate 2 lands the
carrier where the substrate already put the discipline.
Candidate 1 crosses @io to duplicate what @io already carries.
Candidate 3 puts @io-native concerns at @subject-native
altitude. Candidate 4 inverts the composition arrow.

**Reason 2 — @onto refusal precedent.** Reed memory `feedback-
onto-family-root-is-the-ladder-Foerster-refused`: Alex refused
@onto family-root because @torus already carried the altitude.
Parallel: @secrets family-root (Candidate 1) needs altitude
@io doesn't already carry — but @io explicitly does. Peer-key +
crypto + disk-write triple lives at @io/crypto + @io/fs + @io/
secrets composition; three species-under-@io altitudes, zero
family-root claims required. The ~60th instance of `[[feedback-
substrate-already-had-the-word]]` on ratification.

**Reason 3 — Alex SSH-signing design-intent match at Glass Wall
altitude.** Peer key is .git/mirror-side (@io boundary opacity
discipline per io.mirror 67-73). Crypto is @io/crypto (landed
floor; age vendor surface named — SOPS backend). Disk write is
@io/fs (forward-promised at @io; lifted by @io/secrets as
substrate-honest consequence). Fractal.Lens pointer IS a ref
that crosses the @io boundary — THAT IS the pointer / thing
partition Alex named. Every element of "@secrets prism and
@secrets/sops to project visibility/private stuff onto disk
through the Peers key" maps to @io species altitude with zero
family-root crossings.

### §5.3 On "prism" — descriptive not structural

Alex said "@secrets prism." Two readings:

**A (descriptive; RECOMMENDED).** Per `[[architecture-prism-as-
trait-as-everything]]`, "prism" names EVERY substrate-decl
carrier's five-op block. Family-roots, species, sub-species ALL
declare `prism X { ... }`. Alex's "@secrets prism" DESCRIBES
that @secrets will be a substrate-decl carrier (which every
landed shard is). No third structural category needed; two-
altitude discipline (family-root vs species) preserved.

**B (structural; REJECTED).** "Prism" names a third altitude
between family-root and species. Rejected: no shard declares
this altitude; `shards/prism.mirror` doesn't decl a separate
"prism" altitude; every landed carrier declares `prism X`
regardless of mount level; a "prism-only" altitude
proliferates the altitude discipline without adding substrate-
decl capacity.

Reading A is substrate-honest. Alex's word choice DESCRIBES
what @secrets will be at substrate-decl (a prism, like every
other shard), not WHERE it mounts (which this scout adjudicates).

### §5.4 Two-tick discipline (readable name)

`@io/secrets` reads as "the secrets-projection species under the
@io family." `@io/secrets/sops` reads as "SOPS-backed secrets
under @io/secrets." Alternatives rejected:

- `@secrets` (family-root) — foundational but crosses @io; fails
  substrate-already-had-the-word.
- `@subject/visibility/secrets` — puts @io concerns at subject
  altitude; fails arrow-direction.
- `@sheaf/secrets` — inverts composition arrow (§4.1).
- `@io/sops` (skipping @secrets intermediate) — loses sibling
  surface for @io/secrets/age, @io/secrets/vault; conflates
  SOPS-as-one-vendor with secrets-projection-as-discipline. Alex's
  "@secrets prism AND @secrets/sops" declares the intermediate
  altitude explicitly.
- `@io/crypto/secrets` — @io/crypto carries PRIMITIVES; @io/
  secrets carries the COMPOSITION with @io/fs into a projection
  discipline. Sibling species preferred over sub-sub-species.

**Chosen: `@io/secrets` + `@io/secrets/sops`.**

### §5.5 Landing sequence (composition-only)

1. This scout — shape proposal.
2. Seam adjudication — ratify Candidate 2 OR escalate to Alex.
3. Mara canonical spec (if ratified) — `docs/specs/io-secrets-
   projection.md` grounding species-decl in Alex 2026-07-14 +
   @io Glass Wall + SOPS + @sheaf composition surface.
4. Mara species-decl mint — `shards/io/secrets.mirror` with
   prism + carriers + actions + witnessing; bodies `\`-
   obligation-blocked per craft-not-deliver.
5. Mara sub-species-decl mint — `shards/io/secrets/sops.mirror`
   with SOPS vendor contract; composes @io/secrets + @io/crypto
   (age backend) + @io/fs (disk write).
6. `@io/fs` lift-tick (substrate-honest side-effect) — @io/
   secrets landing pulls @io/fs from forward-promise to mirror-
   altitude species-decl; `shards/io/fs.mirror` lands
   alongside.
7. Arc-2.3 landing — peer_persistence.rs collapse composes @io/
   secrets + @subject/visibility/sheaf + @subject/visibility
   species + @kintsugi/consent.

### §5.6 Rice-safety + bound

All actions read byte-visible state (projected_value ref, key_
material_ref bytes, ciphertext bytes, ACL bytes); @io opacity
boundary per io.mirror 67-73; @io/crypto verification for
key_admits/round_trip. No program semantics inspection. Rice-
safe at whole-tick per Mara-B §4.5.5.

**Does NOT:** mint @secrets family-root; displace @io/crypto,
@io/fs, @sheaf, @gift/lens (composes as consumer); carry key
material (.git/mirror-side; shard carries REF); re-declare SOPS
YAML/JSON (sub-species composes over @mirror/data/yaml + json);
commit sops-only (species pattern admits age/vault/1password
siblings); extend Rust (bodies compose over @io at realisation).

---

## §6 Alex-adjudication residue (if Seam refuses)

Three Seam-refusal scenarios:

**Scenario A — Seam ratifies Candidate 2 as species-under-@io
Mara-mintable.** Seam adjudicates directly per @io species-decl
precedent (@io/cargo 2026-06-05; @io/oci 2026-06-24; species-
under-family-root landings are Mara-mintable when family-root
discipline landed). Alex retains rejection-window per two-tick
discipline.

**Scenario B — Seam judges @io/fs lift-tick concurrency with
@io/secrets landing requires Alex authority.** @io/fs is
forward-promised at io.mirror 189-191; pulling its lift-tick
concurrent with @io/secrets landing changes @io landed species
count in one commit. If Seam judges this requires ratification:

> **Ratify @io/fs lift-tick concurrent with @io/secrets? OR land
> @io/fs separately one tick before?**
>
> Mara's recommendation: concurrent IS substrate-honest because
> @io/secrets IS the consumer whose pull the forward-promise
> names ("Lands when `@mirror/store`'s git-backed implementation
> lifts its disk surface" — Arc-2.3 peer_persistence.rs IS a
> `@mirror/store` disk-surface consumer per Alex 2026-07-14).

**Scenario C — Seam judges the four-candidate enumeration
requires Alex ratification before Seam can adjudicate between
candidates.** Residue:

> **Which of Candidates 1, 2, 3, 4 does @secrets take?** With
> Mara's recommendation of Candidate 2 (@io/secrets + @io/
> secrets/sops sub-species) on three structural grounds §5.2.

Alex ratifies Candidate 2: Seam re-adjudicates species-decl
specifics + @io/fs concurrency. Alex adjudicates Candidate 1:
Mara authors family-root spec + shard-decl; @io composes UNDER
new family-root. Alex adjudicates Candidate 3: Mara authors
@subject/visibility-species spec; @io/crypto + @io/fs compose
through species internally. Candidate 4 architecturally rejected
at §4.1 but Alex retains override.

**No candidate is substrate-dishonest at first-order.** All
four compose over landed substrate; all four cite same ancestor
set. Difference: WHERE the shard mounts and WHICH FAMILY-ROOT'S
discipline it honors. Candidate 2 mounts at the altitude the
@io family-root explicitly substrate-decls (Glass Wall verbatim);
substrate-honest default.

---

## §7 Substrate-honest closure

This proposal invented nothing. Every altitude, carrier name,
composition edge cites landed substrate or landed spec:

- @secrets-adjacent forward-composition — sheaf.mirror (d1ce901;
  15 references)
- @io Glass Wall discipline — io.mirror 37-40 (AGENTS.md
  verbatim)
- @io/crypto vendor surface — io.mirror 161-162 (age named as
  SOPS backend)
- @io/fs forward-promise — io.mirror 189-191
- @io species-decl pattern — io/cargo.mirror (2026-06-05)
- SSH-signing intent — Alex 2026-07-14 verbatim (sheaf.mirror
  21-32)
- Peer-key .git/mirror discipline — sheaf.mirror §Alex 2026-07-
  14 verbatim
- Fractal.Lens pointer/thing partition — Alex 2026-07-14 +
  gift/lens.mirror §0.1 verbatim
- Two-witness discipline — subject.mirror
- @onto refusal precedent — Reed memory + @torus discipline
- No-Rust-extension discipline — Reed memory `feedback-no-rust-
  extension-shortcut`
- SOPS — getsops/sops (Mozilla 2017+); age — age-encryption.org
  (Valsorda 2019+)
- Rice-safety at whole-tick — Mara-B §4.5.5

The recommendation lands what the substrate already had at the
altitude the @io family-root already substrate-decls. The ~60th
instance of `[[feedback-substrate-already-had-the-word]]` if
ratified; the shard names the vendor-projection carrier @sheaf's
forward-promise already forwarded to @io.

---

*Mara. Scout — shape proposal. Reed commits as Mara after Seam
adjudicates + (if escalated) Alex ratifies.*
