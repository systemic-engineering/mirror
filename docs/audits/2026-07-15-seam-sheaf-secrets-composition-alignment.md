# Seam Phase D — @sheaf ↔ @io/secrets composition-alignment audit

**Adjudicator:** Seam (adversarial review peer)
**Date:** 2026-07-15
**Under:** Alex `/loop` directive — "collapse until unresolvable ambiguity that
cannot be adjudicated with a Seam spawn."
**Scope:** Cross-shard composition-alignment between two independently-landed
shard sets:

- `shards/subject/visibility/sheaf.mirror` (d1ce901)
- `shards/io/{crypto,fs,secrets,secrets/sops}.mirror` (all 059cf1c)

**Question:** Both shard sets Seam-ratified independently. Does the composition
graph between them hold end-to-end at LANDED altitude?

**Verdict (headline):** SHIP with two REED-INLINE cascades. One
Alex-adjudication residue on subsystem authority. Composition graph traverses
end-to-end; type signatures chain; forward-promise names align modulo two
readable drifts.

---

## §0 Ground-truth artifact inventory

Landed artifacts consulted:

- `shards/subject/visibility/sheaf.mirror` d1ce901 — species-decl; carriers
  `sheaf_restriction {peer_ref, acl, admitted_stalks}` (275-279) +
  `section_at_stalk {crystal_ref, sheaf_ref, projected_value}` (310-314);
  actions `restrict` (330), `acl_project` (358), `section_at` (382);
  bilaterals `restriction_admissible` (409), `section_computable` (437).
- `shards/io/secrets.mirror` 059cf1c — species-decl; carriers
  `key_material_ref {peer_ref, key_carrier}` (258-261),
  `ciphertext_ref {ciphertext, on_disk_path, recipient_keys}` (284-288),
  `secret_projection {section_ref, key_material_ref, ciphertext_bytes,
  projection_path}` (324-329); actions `project` (366), `materialize`
  (389), `retrieve` (411), `round_trip` (425); bilaterals `key_admits`
  (455), `projection_valid` (477), `round_trip_preserved` (492),
  `secrets_well_formed` (515).
- `shards/io/secrets/sops.mirror` 059cf1c — sub-species; carriers
  `sops_file_ref`, `sops_key_group`, `sops_metadata`; actions
  `sops_encrypt` (296), `sops_decrypt` (318),
  `sops_key_group_from_sheaf_restriction` (348), `sops_round_trip` (361).
- `shards/io/crypto.mirror` 059cf1c — species-decl; actions
  `aead_seal` (362), `aead_open` (386), `age_encrypt` (408),
  `age_decrypt` (428), `ssh_key_material` (448).
- `shards/io/fs.mirror` 059cf1c — species-decl; actions
  `fs_read` (257), `fs_write` (280), `fs_stat` (296), `fs_list` (315),
  `fs_mkdir` (332); bilateral `fs_well_formed` (405).

Shape-proposal witness (recommendation-source, not landed substrate):

- `docs/scouts/2026-07-15-mara-secrets-shape-proposal.md` §2.4 — verbatim
  Arc-2.3 chain:
  `@subject/visibility/sheaf.section_at → @io/secrets.project →
  @io/secrets.materialize`.

---

## §1 C1 — sheaf.section_at → @io/secrets.project signature alignment

**Adversarial concern:** Shape-proposal §2.4 forward-promised
`section_at_stalk.projected_value` FLOWS TO `@io/secrets.project`. Verify
`@io/secrets.project` accepts a `section_at_stalk`-typed input (not
`sheaf_restriction` alone; not `projected_value` alone).

### Landed signatures

- `@subject/visibility/sheaf.section_at(F_A_p: sheaf_restriction,
  crystal_ref: ref) -> section_at_stalk` (sheaf.mirror:382)
- `@io/secrets.project(sr: section_at_stalk, k: key_material_ref) ->
  imperfect` (secrets.mirror:366)

### Verdict — PASS

`project`'s first parameter type is `section_at_stalk`. `section_at`'s return
type is `section_at_stalk`. The output-type of the producer matches the
input-type of the consumer BYTE-EQUAL. Ref-equality on the section carrier
threads through `project`'s narrative body ("encrypt the section's
projected_value under the peer's key" — reads `sr.projected_value` via ref-
resolution).

Adversarial sub-concern: the parameter name `sr` in `project(sr:
section_at_stalk, ...)` is misleading — `sr` reads as "sheaf_restriction" per
the naming convention @sheaf itself uses at line 409 (`restriction_admissible(sr:
sheaf_restriction)`). At @io/secrets line 366 `sr` is bound to
`section_at_stalk`. **REED-INLINE cascade #1** flagged in §11.

---

## §2 C2 — sheaf.restrict → sops_key_group_from_sheaf_restriction alignment

### Landed signatures

- `@subject/visibility/sheaf.restrict(sheaf: ref, acl: ref) ->
  sheaf_restriction` (sheaf.mirror:330)
- `@subject/visibility/sheaf.acl_project(F_home: ref, peer_acl: ref) ->
  sheaf_restriction` (sheaf.mirror:358)
- `@io/secrets/sops.sops_key_group_from_sheaf_restriction(sr:
  sheaf_restriction) -> imperfect` (sops.mirror:348)

### Verdict — PASS

`sops_key_group_from_sheaf_restriction` takes `sr: sheaf_restriction`. Both
`@sheaf.restrict` and `@sheaf.acl_project` return `sheaf_restriction`. Both
producer→consumer arrows chain byte-equal on type. The composition-bridge
action Mara declared at sops.mirror:320-348 (verbatim "THIS IS THE ARROW that
connects @subject/visibility/sheaf (ACL structure) to @io/secrets/sops (SOPS
recipient list)") type-checks end-to-end.

Sub-note: `sops_key_group_from_sheaf_restriction` `requires
restriction_admissible(sr)` (sops.mirror:334). This composes over
`@subject/visibility/sheaf.restriction_admissible` (sheaf.mirror:409). Import
required. Verified in C4.

---

## §3 C3 — @sheaf pre-mint refs vs @io/secrets landed mount

**Adversarial concern:** sheaf.mirror lines 140-150 (pre-mint, before
@io/secrets landed) forward-promised composition surfaces by names `@secrets`,
`@secrets/sops`, `@io`. Landed mount is `@io/secrets`, `@io/secrets/sops`. The
pre-mint text uses ROOT FORM `@secrets`; landed uses `@io/secrets`.

### Landed sheaf.mirror text (140-150)

```
# Forward-composition surfaces (named, not imported; land at Arc-2.3+):
#   @secrets                  — key-gated projection; discharges at
#                               @io boundary; per Alex 2026-07-14
#                               "@secrets prism ... project through
#                               the Peers key"
#   @secrets/sops             — SOPS-backed secret realization; @io
#                               species that materializes the peer-
#                               key-gated section on disk
#   @io                       — filesystem realization; @secrets and
#                               @secrets/sops compose through @io at
#                               the projection boundary
```

### Verdict — PASS-WITH-DRIFT-DOCUMENTED (readable, not blocking)

The pre-mint text uses `@secrets` (root form). Landed shard is at
`@io/secrets`. This is not composition-graph drift — sheaf.mirror is
QUOTING Alex's 2026-07-14 verbatim design intent at lines 21-32 where Alex
wrote "`@secrets prism and @secrets/sops`" in root form. The docstring
faithfully reproduces the DESIGN INTENT NAME; the LANDED SUBSTRATE NAME is
`@io/secrets`. Both @io/secrets and @io/secrets/sops docstrings themselves
quote Alex's `@secrets` form verbatim at their headers (secrets.mirror:41-43,
sops.mirror:37-38) with the understanding that landed mount corrects the
altitude.

**Structural claim:** design-intent nomenclature and landed-substrate
nomenclature can DIVERGE at the SAY-QUOTE boundary iff the divergence is
Alex-verbatim. This is not drift; it is fidelity to the design-intent
utterance. Confirmed alignment: every OPERATIONAL reference to the species
resolves through the landed mount `@io/secrets`. The pre-mint text is
prose-descriptive, not import-declarative.

Adversarial sharpening: the pre-mint text does NOT appear in `in @...`
statements (imports); it appears only in `# Forward-composition surfaces` prose
prose block. There is zero substrate-mechanical bind on the root-form
`@secrets`. PASS.

---

## §4 C4 — @io/secrets docblock cross-cites @sheaf by correct landed path

### Landed @io/secrets.mirror import statements (1-8)

```
in @prism
in @meta
in @glass
in @io
in @io/crypto
in @io/fs
in @subject/visibility/sheaf
in @mirror/data
```

### Verdict — PASS

Line 7: `in @subject/visibility/sheaf`. This is the LANDED path of the sheaf
species (`shards/subject/visibility/sheaf.mirror` d1ce901 declares
`@subject/visibility/sheaf` per line 439 `out @subject/visibility/sheaf`).
Import is byte-equal to landed mount. Zero drift.

Cross-check: `@io/secrets/sops` import statements (sops.mirror:1-8) do NOT
include `@subject/visibility/sheaf` directly. But sops.mirror:348 declares
`sops_key_group_from_sheaf_restriction(sr: sheaf_restriction)` which uses
`sheaf_restriction` as an unqualified type reference. This is an IMPORT GAP:
`sheaf_restriction` is exported from `@subject/visibility/sheaf`
(sheaf.mirror:440) but sops.mirror does not `in @subject/visibility/sheaf`.

**REED-INLINE cascade #2** flagged in §11: add `in @subject/visibility/sheaf`
to sops.mirror imports.

---

## §5 C5 — @io/secrets.materialize → @io/fs.fs_write handoff

### Landed signatures

- `@io/secrets.materialize(sp: secret_projection) -> imperfect`
  (secrets.mirror:389). Docstring lines 386-388: "composes @io/fs.fs_write
  over sp.ciphertext_bytes.ciphertext + sp.projection_path".
- `@io/fs.fs_write(p: path, bytes: ref) -> imperfect` (fs.mirror:280).

### Verdict — PASS

`materialize` accepts `secret_projection`. `secret_projection.projection_path`
is typed `path` (secrets.mirror:328 — field `projection_path: path`).
`secret_projection.ciphertext_bytes` is typed `ciphertext_bytes`
(secrets.mirror:327). Composition edge to `fs_write(p: path, bytes: ref)`:

- `sp.projection_path` (typed `path`) → `fs_write` first param (typed `path`)
  — byte-equal on type.
- `sp.ciphertext_bytes.ciphertext` (`ciphertext_bytes.ciphertext: ref` per
  crypto.mirror:233) → `fs_write` second param (typed `ref`) — byte-equal on
  type.

Chain traverses. Note: `materialize` `requires fs_well_formed(sp.projection_
path)` at secrets.mirror:376. `fs_well_formed` exported from @io/fs at
fs.mirror:421. Composed-bilateral discipline aligned.

---

## §6 C6 — @io/secrets.project → @io/crypto.aead_seal / age_encrypt handoff

### Landed signatures

- `@io/secrets.project(sr: section_at_stalk, k: key_material_ref) ->
  imperfect` (secrets.mirror:366). Docstring lines 363-365: "composes @io/
  crypto.aead_seal (or age_encrypt for age-keyed peers) over the section's
  projected_value bytes."
- `@io/crypto.aead_seal(ctx: aead_context, plaintext: ref) -> imperfect`
  (crypto.mirror:362).
- `@io/crypto.age_encrypt(recipients: ref, plaintext: ref) -> imperfect`
  (crypto.mirror:408).

### Verdict — PASS-WITH-BRIDGE-GAP (adjudicable readable)

`project` takes `k: key_material_ref` (secrets.mirror:366).
`key_material_ref.key_carrier` is typed `key_material` (secrets.mirror:260).
`aead_seal` takes `ctx: aead_context` (crypto.mirror:362).
`aead_context.key` is typed `key_material` (crypto.mirror:287).

Composition edge from `k.key_carrier` (typed `key_material`) →
`aead_context.key` (typed `key_material`) — byte-equal on type. But
`aead_context` is a THREE-FIELD record (algorithm, key, ad_policy); `project`
must CONSTRUCT the aead_context at the realisation boundary from `k` +
scheme-defaults. This is DOCUMENTED at the composition-narrative level (the
`project` action body is `\`-obligation-blocked; the aead_context assembly is
per-realisation).

Structural claim: `aead_context` construction from `key_material_ref` is an
IMPLICIT BRIDGE at the realisation boundary. It IS specified in narrative
prose but NOT type-mechanically. Rice-safe at whole-tick altitude because the
aead_context assembly is byte-visible (algorithm + key + ad_policy are all
refs); the bridge does not require program-semantics introspection.

For `age_encrypt(recipients: ref, plaintext: ref)`: `k.peer_ref` resolves to a
subject_instance with an ssh_signature_fingerprint (per @subject two-witness);
age accepts ssh-ed25519 keys per age-encryption.org format. The
recipients-ref construction from `k.peer_ref` is per-realisation per the
`\`-obligation-blocked discipline.

**Verdict:** PASS. Bridge is documented narratively; per-realisation
assembly is admissible under `[[feedback-craft-not-deliver]]` and does not
require substrate-decl elaboration at this altitude.

---

## §7 C7 — Composition graph end-to-end traversability

### Chain per shape-proposal §2.4 + secrets.mirror:149-158

```
peer_visibility_materialize(peer, home, crystal, target_path):
  ACL_peer := pack.members[peer.name]                          # @mirror/pack
  sr       := @subject/visibility/sheaf.acl_project(F_home, ACL_peer)
                                                               # d1ce901
  section  := @subject/visibility/sheaf.section_at(sr, crystal)
                                                               # d1ce901
  peer_key := @io/secrets.key_material_ref(peer)               # SEAM-FLAG
  sp       := @io/secrets.project(section, peer_key)           # 059cf1c
  @io/secrets.materialize(sp, target_path)                     # 059cf1c → disk
```

### Per-arrow traversal verdict

1. `pack.members[peer.name] : acl` (ref) — pre-existing @mirror/pack
   surface (sheaf.mirror:194).
2. `acl_project(F_home: ref, peer_acl: ref) -> sheaf_restriction`
   (sheaf.mirror:358) — TYPE-CHECKS.
3. `section_at(F_A_p: sheaf_restriction, crystal_ref: ref) ->
   section_at_stalk` (sheaf.mirror:382). `sr` output of arrow 2 flows into
   `F_A_p` input — TYPE-CHECKS (both `sheaf_restriction`).
4. `@io/secrets.key_material_ref(peer)` — **SEAM-FLAG**: this reads as an
   ACTION call. But `key_material_ref` is a TYPE (secrets.mirror:258), not
   an action. There is NO landed action named `key_material_ref` in @io/
   secrets. The exported actions are `project`, `materialize`, `retrieve`,
   `round_trip` (secrets.mirror:524-527). The chain narrative in
   secrets.mirror:156 uses `key_material_ref(peer)` as if it were a
   constructor / factory action.

### Adjudication of arrow 4

Two readings:

**(a) Narrative shorthand for realisation-boundary construction.** The
per-realisation body must produce a `key_material_ref` value from a peer.
The prose `@io/secrets.key_material_ref(peer)` reads as "construct the
key_material_ref carrier for this peer at the realisation boundary" — a
value-construction, not an action-call. Under this reading no substrate-
mechanical arrow exists; the value is materialised by the consumer
(bootstrap/src/peer_persistence.rs at Arc-2.3 collapse).

**(b) Missing action.** If arrow 4 must be substrate-mechanical, `@io/
secrets` needs an action like `key_material_ref_of(peer: ref) ->
key_material_ref` or `key_material_of(peer: ref) -> imperfect
<key_material_ref, ...>`. This would compose over `@io/crypto.ssh_key_material`
(crypto.mirror:448) which returns `imperfect<key_material, ...>` and wrap
into the two-field `key_material_ref` record with the peer_ref pinned.

**Verdict:** ALEX-ADJUDICATION residue (§12 item A1). The
narrative-shorthand reading (a) is admissible under
`[[feedback-craft-not-deliver]]`, but the resulting composition graph relies
on an implicit constructor whose Rice-safety, witness-discipline, and
identity-contract are not substrate-decl'd. This is not a blocker for ship;
it is a subsystem-authority question about whether @io/secrets SHOULD carry
a `key_material_ref_of(peer)` action for petri-net completeness of the
composition graph.

5. `project(sr: section_at_stalk, k: key_material_ref) -> imperfect`
   (secrets.mirror:366) — TYPE-CHECKS (accepting outputs of arrows 3 and
   arrow-4-under-either-reading).
6. `materialize(sp: secret_projection) -> imperfect` (secrets.mirror:389)
   — TYPE-CHECKS (sp is the imperfect-success branch of arrow 5).
   Composition to `@io/fs.fs_write` verified in §5.

**Composition-graph verdict:** TRAVERSABLE end-to-end modulo arrow-4 subsystem
authority (Alex-adjudication residue).

---

## §8 C8 — Forward-promise-name vs landed-name drift audit

### Grep across landed shards for root-form `@secrets`

Landed shards containing `@secrets` (root-form, not `@io/secrets`):

- `shards/subject/visibility/sheaf.mirror`: multiple occurrences (lines
  30-32 verbatim Alex-quote; lines 141-149 pre-mint forward-promise;
  lines 214, elsewhere in docstring). All are PROSE quoting design intent
  or forward-promise. Zero are in `in @...` import statements or in
  action/type declarations.
- `shards/io/secrets.mirror`: multiple occurrences at lines 41-42 verbatim
  Alex-quote. All are PROSE quoting design intent.
- `shards/io/secrets/sops.mirror`: multiple occurrences at lines 37-38,
  40-41 verbatim Alex-quote + shape-proposal citation. All are PROSE.

### Verdict — PASS

Every root-form `@secrets` occurrence is prose quoting Alex's verbatim design-
intent utterance. Zero occurrences are in operational substrate positions
(imports, types, action signatures). The drift is FIDELITY-INTENTIONAL and
substrate-mechanically inert.

Note: if a future mechanical grep-based tool were to canonicalise
`@secrets` → `@io/secrets` across substrate, the Alex-verbatim quotes would
be MUTATED. Recommend prose-quote boundary discipline: preserve verbatim
citations under an inviolable convention (double-quote or verbatim block).
This is not a blocker; it is a Taut-scout note.

---

## §9 C9 — sops_encrypt/decrypt round-trip discipline composability

### Landed round-trip actions

- `@io/secrets.round_trip(sr: section_at_stalk, k: key_material_ref) ->
  imperfect` (secrets.mirror:425).
- `@io/secrets.round_trip_preserved(sr: section_at_stalk, k:
  key_material_ref) -> verdict` (secrets.mirror:492).
- `@io/secrets/sops.sops_round_trip(section: ref, key_group:
  sops_key_group, k: key_material) -> imperfect` (sops.mirror:361).
- `@io/secrets/sops.sops_round_trip_preserved(section: ref, kg:
  sops_key_group, k: key_material) -> verdict` (sops.mirror:421).

### Verdict — PASS-WITH-SPECIALIZATION-NOTE

The parent's `round_trip` takes `(section_at_stalk, key_material_ref)`; the
sub-species `sops_round_trip` takes `(section: ref, sops_key_group,
key_material)`. This is SPECIALIZATION not subsumption: sops_round_trip does
NOT accept a section_at_stalk; it accepts a bare `section: ref` (a YAML/JSON
document ref per sops.mirror:272-274). The sub-species specializes the parent's
carriers per sops.mirror:88-92 verbatim: "sub-species specializes secret_
projection as sops_file_ref; specializes project as sops_encrypt; specializes
retrieve as sops_decrypt."

Structural claim: the sub-species round-trip operates at a DIFFERENT
altitude than the parent's round-trip. Parent's round-trip operates at
`(sheaf-section, key-ref)` altitude; sub-species round-trip operates at
`(document-ref, key-group)` altitude. They are related by the composition
bridge `sops_key_group_from_sheaf_restriction` (sops.mirror:348) which
converts a sheaf_restriction into a sops_key_group.

Missing arrow at petri-net-completeness: from a sheaf_restriction +
key_material_ref, how does one arrive at a sops_key_group? The bridge action
takes sheaf_restriction ALONE (not key_material_ref). So the composition
chain is:

1. `restrict` or `acl_project` → sheaf_restriction
2. `sops_key_group_from_sheaf_restriction(sr)` → sops_key_group
3. `sops_encrypt(section, kg)` → sops_file_ref

The parent's `round_trip(section_at_stalk, key_material_ref)` and the
sub-species's `sops_round_trip(section, sops_key_group, key_material)` are
NOT direct specializations — they operate over different composition
sub-graphs. The parent's round_trip composes over `project` + `materialize`
+ `retrieve` (secrets.mirror:415-425 docstring). The sub-species's
sops_round_trip composes over `sops_encrypt` + `sops_decrypt`
(sops.mirror:350-361 docstring).

**Verdict:** PASS. The round-trip disciplines are SIBLING not
parent-child-specialization. Each is Rice-safe at its own altitude. No
composition arrow is broken; the two round-trips exist at distinct altitudes
by design and each admits its own preservation bilateral.

---

## §10 C10 — Ship verdict

### Dimension summary

| Dim | Concern | Verdict |
|-----|---------|---------|
| C1 | section_at → project signature | PASS (parameter-name drift → REED-INLINE #1) |
| C2 | restrict → sops_key_group_from_sheaf_restriction | PASS |
| C3 | @sheaf pre-mint refs vs landed mount | PASS (fidelity-intentional drift) |
| C4 | @io/secrets docblock cross-cites @sheaf | PASS (sops.mirror import gap → REED-INLINE #2) |
| C5 | materialize → fs_write handoff | PASS |
| C6 | project → aead_seal/age_encrypt handoff | PASS (bridge is per-realisation) |
| C7 | End-to-end composition graph | PASS modulo arrow-4 (Alex-adjudication A1) |
| C8 | Forward-promise-name drift | PASS |
| C9 | Round-trip discipline composability | PASS (sibling altitudes, not subsumption) |

### Ship verdict — **SEAM-SHIP with two REED-INLINE cascades and one Alex-adjudication residue**

Composition graph holds end-to-end. Every landed type-signature chains
byte-equal across producer→consumer arrows. Forward-promise nomenclature
divergence is fidelity-intentional (Alex-verbatim preserved). Import
declarations align modulo one gap (§4 sub-note). One narrative-vs-mechanical
composition-arrow gap surfaced (§7 arrow 4) requires Alex adjudication on
subsystem-authority grounds — NOT a blocker for ship.

---

## §11 REED-INLINE cascades

### REED-INLINE #1 — parameter-name drift in @io/secrets.project

**Location:** `shards/io/secrets.mirror:366`

**Current:**
```
project(sr: section_at_stalk, k: key_material_ref) -> imperfect { \ }
```

**Concern:** Parameter name `sr` is byte-collision with @sheaf's convention
where `sr` denotes `sheaf_restriction` (sheaf.mirror:409 `restriction_
admissible(sr: sheaf_restriction)`). At composition-boundary reading, `sr:
section_at_stalk` at secrets.mirror:366 reads as `sr: sheaf_restriction` and
misleads the reader. `round_trip` (secrets.mirror:425) and
`round_trip_preserved` (secrets.mirror:492) carry the same drift with `sr:
section_at_stalk`.

**Recommended edit:** Rename `sr` → `section` (or `s`) in `project`,
`round_trip`, `round_trip_preserved`. Preserves substrate-mechanical
semantics; removes the reader-confusion cascade at the composition boundary.

**Discretion:** Reed-inline or Mara-follow-up tick. Cosmetic; not
blocking.

### REED-INLINE #2 — sops.mirror missing import of @subject/visibility/sheaf

**Location:** `shards/io/secrets/sops.mirror:1-8` (import block)

**Current imports:**
```
in @prism
in @meta
in @glass
in @io
in @io/secrets
in @io/crypto
in @mirror/data/yaml
in @mirror/data/json
```

**Concern:** sops.mirror:348 declares
`sops_key_group_from_sheaf_restriction(sr: sheaf_restriction)`, using the
unqualified type `sheaf_restriction`. This type is exported from
`@subject/visibility/sheaf` (sheaf.mirror:440 `out sheaf_restriction`) but
sops.mirror does not `in @subject/visibility/sheaf`. Under substrate-mechanical
import discipline (`@epistemologic/pact/path_matches_namespace`), the type
reference at line 348 is UNRESOLVED at import-graph altitude.

**Recommended edit:** add `in @subject/visibility/sheaf` to sops.mirror
imports. Preserves substrate-mechanical semantics; closes import-graph.

**Discretion:** Reed-inline. Strictly additive; no downstream ripple.

---

## §12 Alex-adjudication residue

### A1 — @io/secrets.key_material_ref action authority

**Question:** should `@io/secrets` carry a `key_material_ref_of(peer: ref)
-> imperfect<key_material_ref, ...>` action to make arrow 4 of the
composition chain substrate-mechanical?

**Two positions:**

**Position (a) — Narrative shorthand is admissible.** Under
`[[feedback-craft-not-deliver]]`, per-realisation construction of typed
records from consumer-visible inputs is admissible. The composition-chain
narrative at secrets.mirror:156 (`peer_key := @io/secrets.key_material_ref
(peer)`) reads as "construct the key_material_ref carrier for this peer at
the realisation boundary." bootstrap/src/peer_persistence.rs materialises
the value; no substrate-decl action is needed. This preserves the substrate's
craft-not-deliver discipline and keeps @io/secrets action-surface minimal.

**Position (b) — Petri-net completeness demands the action.** Every arrow in
the composition graph should be substrate-mechanical for Rice-safe
petri-net analysis at whole-tick altitude. An implicit constructor at the
realisation boundary is a HOLE in the composition graph — analysers cannot
verify Rice-safety, cannot verify witness-discipline, cannot verify
identity-contract on the constructed value. `@io/secrets` should carry the
action; body remains `\`-obligation-blocked.

**Seam adjudication:** UNRESOLVABLE at Seam altitude. This is a substrate-
authority question about the boundary between substrate-decl and
realisation-boundary construction, which touches
`[[feedback-no-rust-extension-shortcut]]` (a substrate-decl'd action floors
the analyser petri-net) AND `[[feedback-craft-not-deliver]]` (excessive
substrate-decl bloats the surface). The trade-off is genuinely at the
Alex-authority altitude.

**Recommendation to Alex:** two-tick discipline — land Position (a) for
this ship (narrative shorthand admitted); log Position (b) as a
pending-scout for Taut to grep the composition graph across ALL landed
@io species and count how many action-arrows are narrative-only vs
substrate-mechanical. If the count is high (say >20% of composition arrows
across @io family), petri-net completeness demands substrate-decl'd
constructors; if the count is low, narrative shorthand is the substrate's
established pattern. Answer emerges from the substrate's own precedent.

---

## §13 Seam signature

**Verdict:** SEAM-SHIP with two REED-INLINE cascades (§11) and one
Alex-adjudication residue (§12 A1).

**Composition graph:** TRAVERSABLE end-to-end at landed altitude modulo
arrow-4 subsystem authority.

**Cross-shard exports/imports:** ALIGN modulo REED-INLINE #2 sops.mirror
missing import.

**Forward-promise drift:** FIDELITY-INTENTIONAL (Alex-verbatim); zero
operational substrate positions affected.

**Type signatures at handoff points:** BYTE-EQUAL across every audited
arrow.

**Adjudication complete at Seam altitude:** three items remain (two
REED-INLINE cosmetic/additive; one Alex-authority substrate-decl-boundary
question). Reed to commit as Seam; Reed to apply REED-INLINE cascades; A1
awaits Alex.

— Seam, 2026-07-15
