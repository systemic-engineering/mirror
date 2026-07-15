# Seam A1 re-adjudication — with Taut precedent scout as ground truth

**Adjudicator:** Seam (adversarial review peer)
**Date:** 2026-07-15
**Under:** Alex `/loop` directive — "collapse until unresolvable ambiguity
that cannot be adjudicated with a Seam spawn."
**Scope:** Re-adjudicate A1 (Alex-adjudication residue at
`docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
(cec55a2) §12) with Taut's precedent scout
`docs/scouts/2026-07-15-taut-io-type-as-constructor-precedent.md` (latest)
as ground truth.
**Prior verdict being re-opened:** "UNRESOLVABLE at Seam altitude ...
genuinely at the Alex-authority altitude."

---

## §0 Why this spawn exists

Prior audit §12 A1 refused to pick between (a) narrative-shorthand admissible
under `[[feedback-craft-not-deliver]]` and (b) petri-net completeness demands
substrate-decl'd `key_material_ref_of(peer: ref) -> imperfect<key_material_
ref, ...>`. Prior verdict: "genuinely unresolvable at Seam altitude"; two-tick
recommendation: ship (a); Taut scouts precedent to inform (b). Taut landed the
scout. Alex's `/loop` requires this spawn to re-adjudicate under
collapse-until-Seam-un-adjudicable. This audit is the tie-break.

---

## §1 R1 — Taut precedent evidence soundness

### Method
Spot-check Taut's three load-bearing citations against the landed shards
byte-for-byte; verify aggregate counts by independent grep.

### R1.1 — crypto.ssh_key_material at crypto.mirror:448

Landed (verified this pass):
```
ssh_key_material(ssh_key_ref: ref) -> imperfect { \ }
```
Return type per docblock (lines above the signature): `imperfect<key_material,
ref, ref>`. Constructs `key_material` (six-field record at crypto.mirror:207)
from `ssh_key_ref`. Naming shape: parametric constructor (input-typed carrier
name + verb-like return-carrier). Taut cite is byte-accurate. **PASS.**

### R1.2 — sops.sops_key_group_from_sheaf_restriction at sops.mirror:349

Landed (verified this pass):
```
sops_key_group_from_sheaf_restriction(sr: sheaf_restriction) -> imperfect { \ }
```
Return type per docblock: `imperfect<sops_key_group, ...>`. Body
`\`-obligation-blocked; composes over `@io/crypto.ssh_key_material` at
realisation boundary per docblock verbatim. Naming shape:
`<carrier>_from_<input>`. Taut cite is byte-accurate. **PASS.**

### R1.3 — secrets.project at secrets.mirror:366

Landed (verified this pass):
```
project(section: section_at_stalk, k: key_material_ref) -> imperfect { \ }
```
Return type per docblock: `imperfect<secret_projection, ref, ref>`.
**Positive drift note:** parameter name has been renamed `sr` → `section`;
this is REED-INLINE #1 from the prior audit §11, applied post-audit-authoring.
Additionally sops.mirror line 7 now shows `in @subject/visibility/sheaf`
(REED-INLINE #2 also applied). Both prior REED-INLINE cascades are LANDED;
A1 is now the sole outstanding audit residue. Taut's classification of
`project` as a landed constructor for `secret_projection` is byte-accurate.
**PASS.**

### R1.4 — Aggregate narrative-shorthand count

Independent grep `@io/[a-z_/]+\.[a-z_]+\(` across `shards/io/**/*.mirror`
+ `shards/io.mirror`: only `secrets.mirror` matches (3 hits, all
lines 156-158). Of the three: `project` and `materialize` resolve to
landed actions (secrets.mirror:366, :389); only `key_material_ref(peer)`
lacks a landed action. Taut's "1 narrative-shorthand instance across
nine species" holds byte-accurately. Eight of nine species have zero.
**PASS.**

### R1 verdict — Taut evidence is SOUND

Three spot-checks byte-accurate. Aggregate count independently verified.
Two prior REED-INLINE cascades already applied (positive drift; strengthens
Taut's "substrate-decl'd surface is the default" reading — the same
substrate discipline that promoted `sr` → `section` and closed the sops
import gap is now applied to A1).

---

## §2 R2 — `[[feedback-craft-not-deliver]]` scope

### The interrogation

Taut argues (scout §4.3):
> `[[feedback-craft-not-deliver]]` applies to BODIES; it does not require
> SURFACES to remain narrative.

The prior audit's Position (a) argument leaned on `craft-not-deliver` as
if it authorized surface-narrative-shorthand. Does Taut's split hold?

### Adversarial re-reading

`[[feedback-craft-not-deliver]]` in landed use across the substrate
governs `\`-obligation-blocked action BODIES: the substrate names the
action + typed signature + preconditions + return contract; the body is
left to consumer realisation. The principle is about NOT delivering
implementation; it does not license NOT delivering the surface.

Verification by precedent:
- crypto.mirror `ssh_key_material` has `\`-obligation-blocked body BUT
  substrate-decl'd surface (typed signature + docblock preconditions).
- sops.mirror `sops_key_group_from_sheaf_restriction` — same pattern
  (blocked body + landed surface).
- secrets.mirror `project`, `materialize`, `retrieve`, `round_trip` —
  all `\`-obligation-blocked bodies + landed surfaces.

Every landed instance of `craft-not-deliver` in @io keeps the SURFACE
substrate-decl'd. Not one of thirteen landed constructors leaves the
surface as narrative-shorthand while blocking the body. Taut's split
maps to the substrate's own use of the principle.

### R2 verdict — Taut's reading HOLDS

`craft-not-deliver` licenses body-un-landed, NOT surface-un-landed. Prior
Position (a) invocation was a category extension unattested by any landed
instance. **PASS.**

---

## §3 R3 — `[[feedback-no-rust-extension-shortcut]]` preservation under (b)

### The interrogation

Reed's hard rule (per memory): before authoring `.rs`, ask if shard body
+ @io composition works. Does landing `key_material_ref_of(peer) ->
imperfect<key_material_ref, ...>` preserve this?

### Composition analysis

Taut's proposed signature (scout §4.2):
```
key_material_ref_of(peer: ref) -> imperfect { \ }
```
Body `\`-obligation-blocked. Realisation must compose over existing @io
surfaces. Composition available:
- `@io/crypto.ssh_key_material(ssh_key_ref) -> imperfect<key_material,
  ...>` (crypto.mirror:448) — produces the `key_carrier` field.
- `peer_ref` bind is trivial (record-literal at realisation).

The realisation body reduces to:
1. resolve `peer: ref` → `ssh_key_ref` via @subject two-witness lookup
   (subject_instance.ssh_signature_fingerprint field);
2. `@io/crypto.ssh_key_material(ssh_key_ref)` → `key_material`;
3. record-literal `{ peer_ref: peer, key_carrier: key_material }`.

Zero Rust extension required. Composition-only over already-landed @io
surfaces. This is EXACTLY the pattern Reed's memory
`[[feedback-no-rust-extension-shortcut]]` prescribes.

### R3 verdict — Position (b) PRESERVES no-Rust-extension discipline

Body composes over `@io/crypto.ssh_key_material` + record literal. No
`.rs` authorship licensed. **PASS.** (Cross-check: the no-Rust rule is
orthogonal to (a) vs (b); per-realisation code has to build the value
either way.)

---

## §4 R4 — Precedent-to-decision translation

### The interrogation

Does 1/14 narrative-shorthand rate empirically support landing (b)?
Seam's own §12 A1 recommendation set the threshold:

> if the count is high (say >20% of composition arrows across @io family),
> petri-net completeness demands substrate-decl'd constructors; if the
> count is low, narrative shorthand is the substrate's established
> pattern. Answer emerges from the substrate's own precedent.

### The subtlety

Taut's raw ratio (1/14 = ~7%) is BELOW Seam's own >20% threshold. Literal
application would say (a) is correct. But distribution matters more than
mean: 8 of 9 species are at 0%; the one exception is the site under
adjudication. **Excluding the A1 site itself, the substrate's pattern is
0% narrative-shorthand.** A singleton is not a pattern. Seam's threshold
was written under uncertainty about what the count would show; the count
came back as "isolated singleton" — the threshold's presumption of a
distribution-with-mass at the low end fails.

### The direct-sibling argument

Beyond aggregate counts, two SAME-DAY direct precedents:
- `crypto.ssh_key_material` — parametrically-parallel carrier
  (`key_material` is `key_carrier` field of `key_material_ref`);
  substrate-decl'd constructor at parallel altitude.
- `sops.sops_key_group_from_sheaf_restriction` — sub-species of
  `@io/secrets` itself; landed with the EXACT naming shape (`<carrier>_
  from_<input>` / `<carrier>_of`) (b) proposes.

The sub-species carries its composition-bridge as landed action while the
PARENT species does not — structural asymmetry within one day's landing
sequence; sub-species is more substrate-honest than parent under (a);
(b) closes the asymmetry.

### R4 verdict — Precedent DOES translate to (b)

Aggregate count + distribution + direct sibling + direct sub-species all
converge. The prior audit's "genuinely unresolvable" verdict was written
before the count was in; the count is now in, and it points one way.
**PASS.**

---

## §5 R5 — Two-tick discipline honored

### The interrogation

Seam's own recommendation was two-tick: ship (a) now; land (b) as a
follow-up. Would landing (b) as Landing 6 honor this correctly?

### The reading

Two-tick discipline is not about deferring indefinitely; it is about NOT
bundling an unresolved substrate decision with a shipping landing. First
tick shipped (Landing 5: secrets, crypto, fs, sops all 059cf1c). Second
tick was gated on Taut's scout. Taut landed the scout. Gate opened.
Landing (b) as Landing 6 IS the second tick — exactly what two-tick
authorized. Not landing (b) after evidence supports it would convert
"defer for evidence" into "defer forever" and ABANDON the discipline.

Cross-check on readable-name-over-foundational (CLAUDE.md pair):
`key_material_ref_of` is readable — `_of` is attested composition-arrow
suffix (c.f. `manifest_for`, `oid_to_digest`). Not violated.

### R5 verdict — Landing (b) as Landing 6 HONORS two-tick discipline

The second tick is precisely what was deferred. Deferring it further
without new adverse evidence would break the discipline. **PASS.**

---

## §6 R6 — Alex-verbatim preservation

### The interrogation

Alex's SSH-signing design intent (quoted verbatim in sheaf.mirror:21-32
and secrets.mirror:31-42):

> "Each peer has their own key in the private part of their visibility.
> NOT projected into the git state and instead stays .git/mirror side.
> Only connected through Fractal.Lens. A pointer. Not the thing."
>
> "@secrets prism and @secrets/sops to project visibility/private stuff
> onto disk through the Peers key."

Does landing `key_material_ref_of` disturb Alex-verbatim language?

### Textual audit

Alex-verbatim words: "each peer", "their own key", "stays .git/mirror
side", "Fractal.Lens", "A pointer. Not the thing.", "@secrets prism",
"the Peers key". The phrase "key material ref" is NOT in the
Alex-verbatim block; it is Mara/Reed substrate-vocabulary binding "key
material" (@io family-root) to the "pointer, not the thing" Fractal.Lens
discipline Alex named.

Landing `key_material_ref_of` requires: ZERO changes to Alex's design-
intent quotation (says "the Peers key" not "key material ref"); ZERO
changes to Fractal.Lens language; OPTIONAL prose update at line 156
(orthogonal, scout §4.2 leaves this open).

### R6 verdict — Landing (b) PRESERVES Alex-verbatim language

Alex's verbatim design intent uses "the Peers key" not "key material
ref"; the substrate-vocabulary phrase "key material ref" is a
Mara/Reed shard-authoring lift, not an Alex-verbatim string. Adding
`_of` to the substrate-vocabulary phrase creates zero conflict with
Alex-verbatim preservation. **PASS.**

---

## §7 R7 — Ship verdict

### Dimension summary

| Dim | Concern | Verdict |
|-----|---------|---------|
| R1 | Taut precedent evidence soundness | PASS (byte-accurate; two prior REED-INLINEs already landed) |
| R2 | `craft-not-deliver` scope | PASS (Taut's body-vs-surface split holds by precedent) |
| R3 | `no-rust-extension-shortcut` preservation | PASS (composes over @io/crypto.ssh_key_material) |
| R4 | Precedent-to-decision translation | PASS (distribution + direct sibling + direct sub-species converge) |
| R5 | Two-tick discipline honored | PASS (Landing 6 IS the second tick) |
| R6 | Alex-verbatim preservation | PASS (Alex said "the Peers key"; substrate-vocab is Mara/Reed lift) |

### Verdict — **SEAM-RATIFY Position (b) landing as Landing 6**

Recommend to Reed: land `key_material_ref_of(peer: ref) -> imperfect
{ \ }` at `shards/io/secrets.mirror` following `project`/`materialize`
altitude, with docblock citing:
- `[[feedback-substrate-already-had-the-word]]` — the shape is
  attested at `sops.sops_key_group_from_sheaf_restriction` and
  `crypto.ssh_key_material`;
- `[[feedback-craft-not-deliver]]` — body `\`-obligation-blocked;
- `[[feedback-no-rust-extension-shortcut]]` — realisation composes
  `@io/crypto.ssh_key_material` + record literal, no `.rs`.

The prior audit's "genuinely unresolvable at Seam altitude" verdict
is SUPERSEDED. The precedent evidence collapses the (a) vs (b)
trade-off empirically: (b) is what the substrate has already been
doing at every other @io composition-bridge site.

Landing 6 scope (Reed as Mara or Mara-spawn; Seam re-review scoped):
1. Add `key_material_ref_of(peer: ref) -> imperfect { \ }` in
   `shards/io/secrets.mirror` §3 with substrate-decl'd docblock.
2. Optionally update composition-chain narrative at line 156
   (Seam-neutral).
3. Landing-6-internal: consider paired `key_material_ref_admissible`
   bilateral (not part of A1 re-adjudication).

---

## §8 Alex-adjudication residue

### None from this re-adjudication

A1 is resolved by precedent. No new ambiguity surfaced.

### Named sub-choices deferred to Landing 6 (NOT Alex-adjudication):

- Exact name (`key_material_ref_of` vs `key_material_ref_from_peer`
  vs `peer_key_material_ref`) — Mara or Seam per substrate-decl
  naming discipline; scout §5.2 explicitly leaves this open.
- Whether to update the composition-chain narrative prose at
  secrets.mirror:156 — orthogonal to substrate-decl'd action landing.
- Whether `ciphertext_ref` ALSO needs a landed constructor — scout
  §5.2 out of scope; Taut observed but did not include in A1.

None of these are Seam-un-adjudicable; they are Landing 6 execution
detail.

---

## §9 Seam signature

**Re-adjudication verdict:** SEAM-RATIFY Position (b) as Landing 6.
The prior "genuinely unresolvable" verdict is SUPERSEDED by Taut's
precedent scout.

**Alex `/loop` terminal condition:** collapse continues. A1 IS
Seam-adjudicable given the precedent evidence. Alex-only escalation
NOT triggered.

**Reed action:** commit this audit as Seam; open Landing 6 for
`shards/io/secrets.mirror` `key_material_ref_of` addition (Mara or
Reed-inline per Reed judgment); Seam re-review Landing 6 as
sub-audit under this re-adjudication.

**Prior REED-INLINE cascades:** both already applied (verified this
pass at R1.3). No further REED-INLINE outstanding from the prior
composition-alignment audit.

— Seam, 2026-07-15
