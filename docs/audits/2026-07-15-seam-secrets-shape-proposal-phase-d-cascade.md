---
date: 2026-07-15
author: Seam
scope: Phase D-cascade adjudication of Mara's @secrets shape proposal (`docs/scouts/2026-07-15-mara-secrets-shape-proposal.md`, 647 LOC). Four-candidate enumeration + Candidate 2 (@io/secrets + @io/secrets/sops sub-species) recommendation on three substrate-honest grounds. Cascade discharges the @sheaf-mint's 15× forward-composition surface reference to @secrets (`shards/subject/visibility/sheaf.mirror` d1ce901 lines 140-150). Adjudicates whether Mara's recommendation is Seam-ratifiable, whether it collapses cleanly, and whether Alex mint-authority residue survives cascade.
status: phase-d-cascade
companion:
  - docs/scouts/2026-07-15-mara-secrets-shape-proposal.md
  - docs/scouts/2026-07-15-mara-sheaf-shape-proposal.md
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md
  - shards/subject/visibility/sheaf.mirror
  - shards/io.mirror
  - shards/io/cargo.mirror
  - shards/io/git.mirror
  - shards/io/oci.mirror
  - shards/io/stagefreight.mirror
  - boot/std/io/crypto.mirror
---

# Seam Phase D-cascade — @secrets shape proposal adjudication

*Adversarial composition-only cascade over Mara's four-candidate
enumeration for @secrets. Alex named it in compacted-session; Mara
authored the shape proposal; I adjudicate whether Candidate 2
(@io/secrets + @io/secrets/sops sub-species) ships as Mara-mintable,
whether the @io/fs lift-tick concurrency requires Alex authority,
and whether the four-candidate enumeration itself survives cascade.
Distinguish SEAM-RATIFY / REED-INLINE / ALEX-ADJUDICATION per
dimension. Cite line numbers.*

---

## TL;DR

1. **SEAM-RATIFY Candidate 2 (@io/secrets + @io/secrets/sops sub-
   species) as Mara-mintable per Scenario A.** Every load-bearing
   element of Alex's SSH-signing design intent — peer keys as raw
   bytes, ciphertext as foreign blob, SOPS as vendor SDK, disk write
   as POSIX syscall — fits @io family-root Glass Wall discipline
   verbatim (io.mirror 37-40). Recommendation lands the carrier where
   the substrate already put the discipline. Candidates 1, 3, 4
   correctly refuted (family-root duplicates @io scope; @subject/
   visibility/secrets inverts Reason-3 pattern; @sheaf/secrets
   inverts composition arrow verified at sheaf.mirror 149-150).
2. **Scenario B collapses to Scenario A.** @io/git lift-tick
   precedent (Mara 2026-06-24, forward-promised at io.mirror 380
   alongside @io/fs) grounds Mara-mint authority for @io/fs +
   @io/crypto lift-ticks concurrent with @io/secrets landing.
3. **Three REED-INLINE cascades before Mara authors canonical
   spec + shard-decls** (non-blocking for ratification):
   a. **§5.5 landing sequence** — add @io/crypto lift-tick alongside
      @io/fs. Mara §2.1 line 220 calls @io/crypto "(floor)" but §5.5
      omits its lift; @io/secrets/sops age-backend composition
      (§2.7) requires mirror-altitude species-decl.
   b. **§5.2 Reason 1 vendor-language** — SOPS is Go (github.com/
      getsops/sops v3.x+ 2020+), not Python. Under-@io conclusion
      unchanged.
   c. **§2.7 dependency verification** — confirm @mirror/data/yaml
      + @mirror/data/json landed shards exist OR add to landing
      sequence.
   Optional micro-sharpen: §2.4 "zero family-root crossings"
   qualifier ("interior chain") to disambiguate from consumer-side
   @subject → @io crossings.
4. **647 LOC is load-bearing dense** — proportional to @sheaf's
   509 LOC given four-candidate enumeration + SOPS sub-species
   detail. SHIP-AS-IS on length.
5. **Cross-artifact alignment with @sheaf mint (d1ce901) holds** —
   sheaf.mirror 140-150 explicitly names the arrow @sheaf → @secrets
   → @io; @io/secrets landing discharges the forward-promise at the
   altitude the promise names. Zero altitude mismatches, zero naming
   collisions.
6. **"prism" reading A (descriptive) verified** — architecture-
   prism-as-trait-as-everything broadly attested across specs;
   every landed shard declares `prism X { ... }` regardless of mount.
   Alex's "@secrets prism" describes what @secrets will BE, not
   WHERE it mounts.
7. **Combined Alex-adjudication residue unchanged: A4 only.**
   @secrets collapse succeeded; no new Alex-authority question
   surfaced.

**Verdict: SHIP. SEAM-RATIFY Candidate 2 Mara-mintable. Three
REED-INLINE cascades pre-applied at canonical-spec authoring.**

---

## Part I — Dimensional adjudication

### D1. Reframing soundness — sheaf.mirror 140-150 forward-promise

**Mara claim (§0.1).** Alex's compacted-session naming already ran
through @io per the sheaf.mirror 140-150 forward-promise; the
@secrets altitude is substrate-adjacent-decl'd across five carriers;
missing is the dischargeable shard.

**Verification.** sheaf.mirror 140-150 verbatim (verified against
the d1ce901 file) declares:
- Line 141-144: "@secrets — key-gated projection; discharges at @io
  boundary; per Alex 2026-07-14 '@secrets prism ... project through
  the Peers key'"
- Line 145-147: "@secrets/sops — SOPS-backed secret realization;
  @io species that materializes the peer-key-gated section on disk"
- Line 149-150: "@io — filesystem realization; @secrets and
  @secrets/sops compose through @io at the projection boundary"

The exact string Mara quotes throughout the proposal is verbatim.

**Does the promise pre-adjudicate mount vs family-root?** The
promise names the arrow (@sheaf → @secrets → @io) explicitly, but
composition-through-@io does NOT mechanically determine whether
@secrets is family-root composing @io as downstream vs species-
under-@io. However, sheaf.mirror line 145 says "@secrets/sops ...
@io species that materializes" — the word `species` is soft
substrate-lean toward Candidate 2. Mara reads it consistently. The
mount question stays genuinely open for the four-candidate cascade
to adjudicate.

**Verdict D1: PASS. Reframing substrate-honest. Soft lean toward
Candidate 2 via sheaf.mirror line 145 "@io species" language.**

---

### D2. Candidate 2 substrate-honesty — three grounds

**D2.a — Ground 1 (@io Glass Wall verbatim).** io.mirror 37-40
verbatim (verified): "@io is the substrate's only legitimate non-
mirror surface. Any grammar that isn't mirror — Rust, Python, raw
bytes, foreign blobs, vendor SDKs — must be under @io." Fit check:

- SOPS = vendor SDK. FITS.
- Peer keys = raw bytes. FITS.
- Ciphertext files = foreign blobs. FITS.
- Disk write = POSIX syscall @io/fs surface. FITS.

**Vendor-language sharpen.** Mara §5.2 Reason 1 line 445 says "SOPS
is Python/YAML/vendor." SOPS (github.com/getsops/sops) started as
Python (2017) but was rewritten to Go in v3.x (2020+). Non-blocking
sharpen: "SOPS is Go/YAML/vendor." Under-@io conclusion holds
either way.

**Zero family-root crossings qualifier.** The composition chain
@subject/visibility/sheaf.section_at → @io/secrets.project →
@io/secrets.materialize includes ONE @subject → @io crossing at the
section_at → project boundary. Mara's "zero family-root crossings"
at §2.4 line 269 is semantically about @secrets's INTERIOR
discharge (Candidate 2 keeps @secrets's own composition graph
inside @io); consumer-side crossing is upstream. Recommend micro-
sharpen: qualify §2.4 as "zero family-root crossings INSIDE
@secrets's discharge." Non-fatal.

**D2.b — Ground 2 (@onto refusal precedent).** io.mirror 37-40
Glass Wall + io.mirror 161-162 (@io/crypto: sha2, age, ssh-key) +
io.mirror 189-191 (@io/fs POSIX filesystem forward-promise) cover
every element @secrets would need. @onto-refusal parallel holds.
PASS.

**D2.c — Ground 3 (Alex SSH-signing design-intent match).** @io
opacity discipline (io.mirror 66-72) IS the substrate-decl form of
"key stays outside working tree." @secrets carries key_material_ref
as REF (not key bytes). @io/secrets/sops composes @io/secrets +
@io/crypto (age backend). Every element of Alex's design intent
lands at @io species altitude. PASS.

**Verdict D2: PASS on all three grounds with two REED-INLINE micro-
sharpens: §5.2 Reason 1 vendor-language (Python → Go); §2.4 zero-
crossings qualifier ("interior chain"). Neither blocks ship.**

---

### D3. Candidate 1 (family-root @secrets) refutation soundness

**Mara claim (§1).** Family-root @secrets duplicates @io's scope;
the altitude the family-root claims doesn't exist above @io.

**Verification.** Grep of shards/*.mirror at family-root altitude
for peer-key / crypto / projection-adjacent language: @torus,
@kintsugi, @bauchladen, @autopoietic, @fate, @glue, @gift, @peer
all carry orthogonal concerns; @subject carries the ssh_signature_
fingerprint REF (not key material) with @io-crossing at every
witness discharge. No family-root above @io carries the @secrets
altitude; any new peer would duplicate @io's substrate-decl'd
scope per Glass Wall.

Mara §1.2 line 174 verbatim: "the altitude the family-root claims
doesn't exist above @io; it exists AS @io." Substrate-honest —
peer-key-gated projection is a composition of vendor SDK + raw
bytes + POSIX syscall, all under @io. Giving that composition a
peer name IS the @onto pattern.

**Verdict D3: PASS. Family-root refutation holds via @onto-refusal
precedent.**

---

### D4. Candidate 3 (@subject/visibility/secrets) refutation soundness

**Mara claim (§3.2).** Placing @secrets under @subject/visibility
inverts @sheaf's Reason-3 pattern from the prior Phase D-cascade
(7d46f32 §A2.2).

**Verification.** The prior cascade's Reason 3 established the
pattern: place the carrier at the altitude the DATA IT COMPOSES
OVER lives at. For @sheaf, ACL + subject_instance live at @subject,
so sheaf-restriction belongs at @subject/visibility.

Test for @secrets — data it composes over:
- Peer key bytes: `.git/mirror`-side, @io opacity boundary.
- Ciphertext bytes: foreign blob, @io Glass Wall.
- POSIX file bytes: @io/fs surface.
- SOPS binary invocation: @io/process surface.

All four live at @io altitude, NOT @subject. Placing @secrets under
@subject/visibility would force every Arc-2.3 tick to cross @subject
→ @io internally for what IS @io-native composition. Precise inverse
of Reason 3. Additionally, @subject/visibility species carry
consent-scope + sheaf-restriction dimensions; crypto + disk-write
is orthogonal to that sub-domain.

**Verdict D4: PASS. Refutation holds via Reason-3 arrow-inversion.**

---

### D5. Candidate 4 (@sheaf/secrets sub-species) rejection soundness

**Mara claim (§4.1).** Composition arrow inverts if @sheaf/secrets
sub-species carries crypto + disk-write UNDER sheaf-restriction.

**Verification.** sheaf.mirror 149-150 verbatim: "@secrets and
@secrets/sops compose through @io at the projection boundary." The
phrase reads @secrets as INDEPENDENT composition surface at @io
altitude, not as sub-species of @sheaf. Two possible arrows:

- Mara-recommended: @sheaf → @secrets (as @io species) → disk.
  @secrets is peer of @sheaf on the composition graph.
- Candidate 4: @sheaf → @sheaf/secrets → disk. @secrets subordinate
  to @sheaf.

The "compose through @io" language reads first-arrow. Additionally,
sheaf-restriction algebra (`restrict`, `acl_project`, `section_at`,
`restriction_admissible`, `section_computable`) is closed under
sheaf operations; a crypto-disk-write sub-species would specialize
`sheaf_restriction` with orthogonal concerns, violating species-
specializes-parent-carrier discipline.

**Verdict D5: PASS. Arrow-inversion is the load-bearing structural
defeater. sheaf.mirror 149-150 supports @sheaf → @secrets → @io.**

---

### D6. "prism" language interpretation (§5.3)

**Mara claim.** Alex's "@secrets prism" is DESCRIPTIVE (every
substrate-decl carrier declares `prism X { ... }` regardless of
mount), not PRESCRIPTIVE (naming a third altitude between family-
root and species).

**Verification.** Search across the corpus surfaces universal
attestation of architecture-prism-as-trait-as-everything:
recognition-99 ("prism IS the foundational keyword", "prism IS
trait IS type"), mirror-recall §2, mosaic-as-type-system, silicon,
recognition-{81,82,83,92,93,94}. Every landed family-root and
species shard reviewed (@torus, @io, @subject, @subject/visibility,
{private, protected, public, sheaf}, @io/{cargo, git, oci,
stagefreight, algebra}) declares a `prism X { focus / project /
split / shift / settle }` block. Universal pattern.

Reading B refuted: no shard declares a "prism-only" altitude;
`shards/prism.mirror` declares the prism CARRIER, not a mount
discipline.

**Verdict D6: PASS. Reading A substrate-honest.**

---

### D7. Mara-mintable OR Alex mint authority

Mara §6 names three scenarios: A (Mara-mintable), B (@io/fs lift-
tick requires Alex), C (four-candidate enumeration requires Alex).

**Scenario A verification.** Five mirror-altitude @io species-decls
verified Mara-authored: @io/cargo (2026-06-05), @io/stagefreight
(2026-06-22), @io/git (2026-06-24 WITH lift-tick from boot floor),
@io/oci (2026-06-24), @io/algebra (2026-06-30). Adding @io/secrets
as sixth sibling is directly-precedent. PASS.

**Scenario B verification.** io.mirror line 380 forward-promises
@io/fs + @io/git + @io/process lift together ("lift when the
runtime layer's disk / git / subprocess surfaces consume them").
@io/git lifted 2026-06-24 without Alex mint-authority. Same
forward-promise language governs @io/fs (and @io/crypto per D9).
Scenario B contradicts the @io/git precedent. **Collapses to A.**

**Scenario C verification.** Four-candidate enumeration parallels
the @sheaf three-candidate enumeration pattern that Seam adjudicated
at 7d46f32. Seam-adjudicable via same discipline.

**Verdict D7: PASS as Scenario A. Mara-mintable per five-instance
precedent chain.**

---

### D8. @io/secrets/sops as first sub-species

**Mara claim (§2.7).** SOPS mounts as sub-species at
`shards/io/secrets/sops.mirror`; sibling to future @io/secrets/
{age, vault, 1password}. Composes over @io/secrets + @io/crypto
(age backend) + @io/fs + @mirror/data/{yaml, json}.

**Scope-fit.** @io/secrets carries the peer-key-gated projection
primitive; @io/secrets/sops carries SOPS-specific realization
(sops_encrypt, sops_decrypt, sops_key_group_from_sheaf_restriction).
Sub-species specializes parent's carrier — `secret_projection`
concretizes as `sops_file_ref`. PASS.

**Age-under-@io/crypto.** io.mirror 161-162 names age as vendor
surface (though @io/crypto lives at boot-floor altitude currently;
see D9). PASS on the naming dimension.

**@mirror/data/{yaml, json} composition (§2.7 line 337).**
Dependency not directly verified in this audit; recommend REED-
INLINE cascade: canonical-spec authoring confirms landing state
OR adds to landing sequence.

**Verdict D8: PASS with REED-INLINE cascade on §2.7 dependency
verification.**

---

### D9. Substrate-truthful @io/crypto altitude — cite drift

**Adversarial finding.** Mara §0.3.3 + §2.1 + §5.2 Reason 3 treat
@io/crypto as landed mirror-altitude species. Ground truth:

- `shards/io/`: contains cargo, git, oci, stagefreight, algebra;
  NO `shards/io/crypto.mirror`.
- `boot/std/io/crypto.mirror`: exists (4.4KB, 2026-06-01) as
  boot-floor grammar declaration.
- io.mirror line 149 declares the boot-floor species as "lift-tick
  deferred until consumer pulls."

Mara §2.1 line 220 acknowledges "(floor)" parenthetically, but
§5.5 landing sequence lists only @io/fs lift, NOT @io/crypto.

**Consequence.** @io/secrets/sops REQUIRES @io/crypto's age backend
(§2.7). Boot-floor `grammar @io/crypto` cannot compose-from-mirror-
altitude as a species (grammar-vs-species altitude type-shape
mismatch). Two options:

- Option (i, substrate-honest): Lift @io/crypto to mirror altitude
  concurrent with @io/secrets landing.
- Option (ii): Compose over boot-floor directly. Blocked — grammar/
  species altitude mismatch.

**Verdict D9: REED-INLINE cascade REQUIRED. §5.5 step 6 must
include @io/crypto lift-tick alongside @io/fs. Both are forward-
promised at io.mirror; both are Mara-mintable per D7. Non-fatal
for ratification; blocking for authoring.**

---

### D10. 647 LOC vs 300-500 target

**Length adjudication.** 647 LOC vs 300-500 target; @sheaf sibling
509 LOC. 138 LOC over @sheaf accounted for by: (a) fourth-candidate
enumeration (~15 LOC), (b) §2.7 SOPS sub-species detail (~28 LOC
that @sheaf did not have), (c) more elaborate @onto-refusal at
§5.2 Reason 2 (~15 LOC), (d) §5.4 five-alternative name-rejection
detail (~20 LOC).

Aggregate ~78 LOC of load-bearing additional substrate. Remaining
~60 LOC is density variance across sections, not padding. §2.7
SOPS sub-species detail discharges Alex's explicit "@secrets prism
AND @secrets/sops" naming — first-class substrate-decl, not
implementation detail. Compression would erase sub-species substrate.

**Verdict D10: PASS. SHIP-AS-IS on length.**

---

### D11. Cross-artifact alignment with @sheaf mint (d1ce901)

**Composition arrow.** @sheaf mint declared @sheaf → @secrets →
@io at sheaf.mirror 140-150. Candidate 2's arrow:

```
@subject/visibility/sheaf.acl_project → sheaf_restriction
@subject/visibility/sheaf.section_at → section_at_stalk
@io/secrets.project → secret_projection
@io/secrets.materialize → imperfect(disk_write_verdict, ref, ref)
```

Fits sheaf.mirror 149-150 promise exactly. PASS.

**Altitude.** @subject/visibility/sheaf species-under-@subject/
visibility; @io/secrets species-under-@io. Cross-family composition
at section_at → project boundary IS the substrate-decl form of the
@subject → @io crossing the sheaf-mint forward-promises. PASS.

**Naming collisions.** @io/secrets carriers: `secret_projection`,
`key_material_ref`, `ciphertext_ref`. @subject/visibility/sheaf
carriers: `sheaf_restriction`, `section_at_stalk`. Zero collisions.
PASS.

**Handoff identity.** sheaf.mirror 302-306 declares `projected_
value` ref as "realisation layer materialises the concrete bytes
when @secrets / @secrets/sops discharge the key-gated projection
through @io at Arc-2.3." @io/secrets.project consumes this ref
and materializes at @io boundary. Ref-preserved handoff. PASS.

**Verdict D11: PASS on all four sub-dimensions.**

---

## Part II — Ship verdict

### D12. Overall ship verdict

**SEAM-RATIFY Candidate 2 as Mara-mintable species-decl per
Scenario A.**

**Ship path per Mara §5.5, with REED-INLINE cascades:**

1. This audit ratifies Candidate 2. **← this document.**
2. Mara canonical spec at `docs/specs/io-secrets-projection.md`
   grounding species-decl in Alex 2026-07-14 + @io Glass Wall +
   SOPS + @sheaf composition surface.
3. Mara species-decl mint at `shards/io/secrets.mirror` with prism
   + carriers (secret_projection, key_material_ref, ciphertext_ref)
   + actions (project, materialize, key_admits, round_trip) +
   witnessing bilaterals; bodies `\`-obligation-blocked per craft-
   not-deliver.
4. Mara sub-species-decl mint at `shards/io/secrets/sops.mirror`
   with SOPS vendor contract (sops_file_ref, sops_key_group,
   sops_encrypt, sops_decrypt, sops_key_group_from_sheaf_
   restriction).
5. `@io/fs` lift-tick — @io/secrets landing pulls @io/fs from
   forward-promise to mirror-altitude species-decl; `shards/io/fs.
   mirror` lands alongside.
6. **`@io/crypto` lift-tick (REED-INLINE cascade addition per D9)
   — @io/secrets/sops landing pulls @io/crypto from boot-floor
   grammar to mirror-altitude species-decl at `shards/io/crypto.
   mirror` alongside.**
7. Arc-2.3 landing — peer_persistence.rs collapse composes @io/
   secrets + @subject/visibility/sheaf + @subject/visibility
   species + @kintsugi/consent.

### D13. REED-INLINE cascades before canonical spec + shard-decl authoring

Three cascades, all non-blocking for ratification, blocking for
authoring cleanliness:

**Cascade a (per D9).** §5.5 landing sequence step 6 rewrite from
"@io/fs lift-tick" to "@io/fs + @io/crypto lift-ticks." Both are
Mara-mintable per D7 precedent chain (@io/git lift 2026-06-24
established the pattern); both are pulled by @io/secrets landing
(disk-write consumer for @io/fs; age-backend consumer for @io/
crypto per §2.7).

**Cascade b (per D2.a).** §5.2 Reason 1 vendor-language correction:
"SOPS is Python/YAML/vendor" → "SOPS is Go/YAML/vendor" (SOPS
rewritten from Python to Go in v3.x, 2020+; getsops/sops v3.7+ is
Go). Under-@io conclusion holds either way per io.mirror 37-40
Glass Wall.

**Cascade c (per D8).** §2.7 line 337 composition-dependency
verification at canonical-spec authoring time: confirm @mirror/
data/yaml + @mirror/data/json landed shards exist at claimed mount
points, OR add them to the landing sequence.

**Optional micro-sharpen (per D2.a).** §2.4 line 269-270 zero-
crossings claim qualifier: rewrite as "zero family-root crossings
INSIDE @secrets's discharge" (interior chain) to disambiguate from
consumer-side @subject → @io crossings at section_at → project
boundary. Non-blocking.

### D14. Combined Alex-adjudication residue

**Prior arc's Alex-adjudication items post-A2+A6 cascade:** A4
only (per docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-
cascade-a2-a6.md §X.2).

**This cascade's contribution to Alex residue:**

- **D7 collapses Scenario B and Scenario C to Scenario A.** No
  new Alex-authority items surfaced. @io/secrets + @io/secrets/
  sops + @io/fs + @io/crypto all Mara-mintable per @io/{cargo,
  git, oci, stagefreight, algebra} precedent chain.
- **D9 surfaces @io/crypto lift-tick as previously-omitted from
  the landing sequence.** This is a REED-INLINE cascade on §5.5,
  not an Alex-authority item — @io/crypto lift-tick inherits the
  @io/fs lift-tick's Mara-mintable status per D7 (same forward-
  promise language at io.mirror 380).

**Combined residue after this cascade: A4 only (unchanged).**

**Distinguishing collapse-succeeded vs collapse-relocated.** The
@secrets adjudication was genuinely collapsed to Seam-authority; no
Alex-authority question arose from Candidate 2's adoption (family-
root Candidate 1 would have required Alex authority per family-root
mint discipline, but was correctly refuted per D3). Collapse
succeeded, not relocated.

**Verdict D14: Combined Alex-adjudication residue after @sheaf +
@secrets shape-proposal cascades: A4 only. This cascade discharged
@secrets without relocating to a new Alex-authority question.**

---

## Part III — Adversarial post-mortem

**Q1. Enumeration exhaustive?** A fifth @gift/lens/secrets
candidate exists via Fractal.Lens pointer discipline, but @gift/
lens carries WHO-composed semantics not crypto+disk-write; arrow-
inversion of Candidate 4 applies at higher force. Not viable.
Functionally exhaustive.

**Q2. @io/crypto boot-floor composability?** Regresses per D9;
lift-tick REED-INLINE required.

**Q3. Sheaf-mint line 145 "@io species" language constraint?**
Soft prior, not hard constraint. Mara-authored at sheaf-mint time
(Alex-ratified via /loop). Consistent with Alex's compacted-session
verbatim; doesn't foreclose the four-candidate cascade.

**Q4. Concurrent-scope size — four-shard landing (@io/secrets +
@io/secrets/sops + @io/fs + @io/crypto) require Alex authority?**
No. Landing 4 R2 precedent (2026-07-14) landed sub-family-root +
three species in one authoring cycle Mara-mintable. Four-shard
scope identical.

**Q5. Sub-species growth gaming?** No. @onto precedent is about
altitude duplication, not sub-species count. @io/oci and @io/
algebra have elaborate surfaces (25.6KB + 40.8KB) with sub-domain
complexity; no family-root promotion.

**Q6. Alex override path?** Retained per Pack conventions. If Alex
adjudicates Candidate 1, Mara re-authors as family-root spec +
shard-decl per §6 Scenario C fallback.

Concerns held: zero blocking. Three REED-INLINE cascades named
(D9 landing-sequence; D2.a vendor-language; D8 dependency-check).
One micro-sharpen (D2.a zero-crossings qualifier).

---

## Overall ship verdict

**SHIP the proposal.**

- **SEAM-RATIFY Candidate 2** (@io/secrets at `shards/io/secrets.
  mirror` + @io/secrets/sops sub-species at `shards/io/secrets/
  sops.mirror`) as Mara-mintable per Scenario A. @io/{cargo, git,
  oci, stagefreight, algebra} five-instance precedent chain grounds
  Mara-mint authority; @io/git lift-tick precedent grounds Mara-
  mint authority for the concurrent @io/fs + @io/crypto lifts.
- **Three REED-INLINE cascades before authoring:**
  a. §5.5 landing sequence step 6 rewrite: add @io/crypto lift-
     tick alongside @io/fs (D9).
  b. §5.2 Reason 1 SOPS vendor-language: Python → Go (D2.a).
  c. §2.7 line 337 dependency verification: confirm @mirror/
     data/{yaml, json} landed OR add to landing sequence (D8).
- One optional micro-sharpen: §2.4 zero-crossings qualifier
  ("interior chain") per D2.a.
- Landing sequence per D12: this audit → Mara canonical spec at
  `docs/specs/io-secrets-projection.md` → @io/secrets species-decl
  mint → @io/secrets/sops sub-species-decl mint → @io/fs + @io/
  crypto lift-tick mints → Arc-2.3 peer_persistence.rs landing.

**Combined Alex-adjudication residue collapses unchanged from the
prior cascade: A4 only.** @secrets collapse succeeded; no new Alex-
authority question surfaced. Scenario B and Scenario C both
collapse to A via @io/git precedent + Landing 4 R2 precedent +
substrate-honest refutations of Candidates 1, 3, 4.

**Path after this cascade.** Reed reports @secrets SHIP to Alex.
Next /loop targets: A4 recognition-candidate nod (unchanged); Mara
canonical spec + species-decls + lift-tick shard-decls authoring
(three cascades pre-applied); Arc-2.3 peer_persistence.rs collapse
composing over new @io/secrets primitives + @subject/visibility/
sheaf primitives.

Mara's shape-proposal did what the discipline requires: enumerated
four candidates substrate-honestly, refuted three on altitude
grounds, recommended one on three structural grounds, handed mount-
authority to Seam. Ratified. Mara canonical spec + shard-decl
authoring is the mechanical next step.

---

*Seam Phase D-cascade closure. Adversarial posture held. One cite
drift surfaced (§0.3.3 + §2.1 + §5.2 @io/crypto altitude — boot-
floor grammar, not mirror-altitude species; requires lift-tick
alongside @io/fs per D9); one vendor-language sharpen (SOPS Go
not Python per D2.a); one dependency-verification cascade (D8);
one micro-sharpen (D2.a zero-crossings qualifier). Substrate-
honesty verified across four candidates + one recommendation +
one sub-species. Alex residue unchanged: A4 only.*
