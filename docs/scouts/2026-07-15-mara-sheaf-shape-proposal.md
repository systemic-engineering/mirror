---
date: 2026-07-15
author: Mara
scope: Shape proposal for @sheaf shard-decl mint. Discharges A2 from Seam Phase D adjudication (`docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md` §D12) after Alex's /loop re-fire redirected A2 from terminal Alex-only-state to Mara-authors-shape-then-Seam-re-adjudicates. Enumerates three substrate-honest candidates for @sheaf; recommends ONE; hands to Seam for adjudication (Alex-adjudication residue named if Seam refuses on family-root mint authority grounds).
status: scout — shape proposal (design-choice artifact, NOT canonical spec)
companion:
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md
  - docs/specs/kintsugi-ouroboros-compiler-self-collapse.md
  - docs/specs/eigensheaf.md
  - docs/specs/mirror-spec-peer-acl-surface.md
  - docs/specs/peer-persistence-and-home-projection.md
  - shards/subject.mirror
  - shards/subject/visibility/{private,protected,public}.mirror
  - shards/torus.mirror
  - shards/epistemologic/math/sheaf_laplacian.mirror
---

# Mara — @sheaf shape proposal (three candidates + one recommendation)

*Design-choice artifact for Seam adjudication. Three substrate-honest
candidates for @sheaf, one recommendation. Not a canonical spec;
canonical follows Seam adjudication + (if escalated) Alex
ratification.*

---

## §0 Framing

### §0.1 The question A2 asks

Mara-B §7.2 A2 recommended Option A: mint `@sheaf` as family-root as
Landing D forward-promise before Arc-2.3 (peer_persistence collapse).
Seam Phase D §D12 triaged A2 as Alex-adjudication because family-root
mint authority is Alex's — not because Option A is substrate-
dishonest. Alex re-fired /loop; interpretation: Mara authors the
shape proposal, Seam re-adjudicates whether family-root is warranted
(vs species under existing family).

### §0.2 What @sheaf must carry (arc-derived load)

1. **Arc-2.3 ACL primitive.** Per Mara-B §7.2 + kintsugi-ouroboros
   §3.2 Tick 2.3, collapse of `bootstrap/src/peer_persistence.rs`
   requires an `acl_project` action gating peer-visibility of another
   peer's home-repo.

2. **Alex 2026-07-14 SSH-signing design intent** (verbatim from
   peer-persistence session compaction):

   > "Each peer has their own key in the private part of their
   > visibility. NOT projected into the git state and instead stays
   > .git/mirror side. Only connected through Fractal.Lens. A
   > pointer. Not the thing."

   > "@secrets prism and @secrets/sops to project visibility/private
   > stuff onto disk through the Peers key."

   @sheaf must support peer-key-gated visibility projection through a
   Fractal.Lens pointer to `.git/mirror`-side key material; compose
   with `@secrets`/`@secrets/sops` at the projection boundary.

### §0.3 Substrate-already-had-the-word evidence (load-bearing)

Grep across the substrate for prior @sheaf-adjacent language reveals
three landed carriers:

1. **`docs/specs/mirror-spec-peer-acl-surface.md` §5** substrate-
   decl's the pack.members ACL type surface (§6.2 carries the
   self-naming rule, distinct from sheaf structure). The
   "ACL IS the sheaf structure" language is not at §6.2 — that
   was a Mara mis-attribution corrected by Seam Phase D-cascade
   audit at `docs/audits/2026-07-15-seam-kintsugi-ouroboros-
   phase-d-cascade-a2-a6.md` §A2.1. Mirror-spec §10 sheaf framing
   was reframed sheaf → spawn-and-probe per Alex 2026-06-24 and
   is retracted. The load-bearing verbatim substrate-decl of
   "ACL IS the sheaf structure" lives at bullet 2 below
   (peer-persistence §12.3, verbatim, load-bearing).

2. **`docs/specs/peer-persistence-and-home-projection.md` §12.3**
   verbatim:

   > "The ACL IS the SHEAF STRUCTURE. For each peer `p ∈ pack.
   > members`: The peer's ACL `A_p` defines a SUB-SHEAF `F_home|_{A_p}`
   > — the restriction of `F_home` to the vertices/edges the ACL
   > admits. The peer's visibility scope for a given crystal is the
   > SECTION of `F_home|_{A_p}` at that crystal's stalk."

   And explicitly:

   > "Landing A's four visibility species (per Landing 4 R2) ARE the
   > sections of the four canonical ACL restrictions the pack block
   > defines. The substrate always had the word."

3. **`shards/epistemologic/math/sheaf_laplacian.mirror`** — the
   discrete cellular-sheaf math primitive (Hansen-Ghrist 2018) is
   already substrate-decl'd as species under `@epistemologic/math`.

**Consequence.** The @sheaf altitude is SUBSTRATE-ALREADY-DECL'D
across specs; missing is a dischargeable shard at substrate-decl
altitude. The three candidates differ in WHERE that shard mounts.

### §0.4 Discipline the recommendation honors

- Substrate-already-had-the-word ENFORCED (@onto refusal precedent).
- Composition-only surface; zero new mints beyond @sheaf itself.
- Two-tick discipline: readable name over foundational.
- Rice-safe: actions read empirical byte-visible state.
- No Rust; spec/scout altitude only.

---

## §1 Candidate 1 — @sheaf as family-root

**Path.** `shards/sheaf.mirror`. Peer to `@torus`, `@subject`,
`@kintsugi`, `@bauchladen`, `@autopoietic`, `@fate`, `@glue`,
`@peer`, `@gift`.

**Prism.** `prism @sheaf { focus/project/split/shift/settle sheaf }`.

**Carriers.** `sheaf` (base graph + restriction map bundle),
`section` (harmonic representative at a stalk), `acl` (sub-sheaf
restriction).

**Actions.** `acl_project(F, A) -> sub_sheaf`, `section_at(F, v) ->
section`, `sheaf_witnessing(F) -> verdict`.

### 1.1 Substrate-already-had-the-word check

Load-bearing defeater carrier that @sheaf-family-root would cross:
**`@subject/visibility`** (species altitude under @subject family-
root; carries the ACL-sheaf-restriction altitude per peer-
persistence §12.3 verbatim). `shards/torus.mirror` carries zero
sheaf tokens on grep (per Seam Phase D-cascade §A2.3 confirmation);
@torus does NOT carry sheaf-adjacent structure. Other adjacent
carriers at non-defeating altitudes: `@subject` (SEL-licensable
party; parent of visibility), `@epistemologic/math/sheaf_laplacian`
(Hansen-Ghrist math primitive), `docs/specs/eigensheaf.md` §2.4
("the substrate IS its eigensheaf").

The word `sheaf` is used across THREE substrate altitudes; each has a
landed carrier. Adding `@sheaf` at family-root would need to name
what it adds beyond the three.

### 1.2 Load-bearing altitude claim

Proposed: "sheaf-restriction-as-ACL-primitive at substrate altitude,"
lifting the ACL sheaf structure from its current position (embedded
under @subject/visibility) to peer altitude of @subject.

**Against the lift.** The ACL sheaf ALREADY composes over @subject
(visibility species) and `pack.members` (peer-to-ACL binding). The
family-roots that use it use it THROUGH @subject/visibility, not as
a peer altitude. That is species-under-@subject composition, not
peer-to-@subject altitude.

### 1.3 Ancestor citations

`docs/specs/eigensheaf.md` §2.4; `docs/specs/mirror-spec-peer-acl-
surface.md` §6.2; `docs/specs/peer-persistence-and-home-projection.md`
§12.3; `shards/subject.mirror`; `shards/subject/visibility/
{private,protected,public}.mirror`; Alex 2026-07-14 SSH-signing
message; Hansen-Ghrist 2018 (arXiv:1808.01513); Foerster 2003 (@torus
precedent for refusing altitude that another family-root carries).

### 1.4 Alex SSH-signing design intent coverage

Possible but architecturally awkward. `@subject` already carries
the two-witness discipline (`ssh_witness_valid` + `spectral_witness_
valid`); Alex's peer-key discipline composes over `ssh_witness_valid`
as its first witness. Family-root @sheaf would CROSS @subject's
two-witness altitude to reach the peer key — re-introduces the
peer-key surface at family-root altitude rather than accepting
@subject's substrate-decl'd binding. Requires cross-family-root
composition at Arc-2.3.

### 1.5 Arc-2.3 tournament ranking primitive coverage

Yes. `@sheaf.acl_project(F, peer_ACL) -> sub_sheaf` carries the
ACL-gate primitive; Arc-2.3 shard body composes over @sheaf +
@subject/visibility + @kintsugi/consent.

### 1.6 Substrate-honest bound

Does NOT invent sheaf math (at @epistemologic/math/sheaf_laplacian),
ACL primitive (at pack.members), or visibility species (at @subject/
visibility). ADDS a peer-to-@subject family-root abstraction of the
ACL-sheaf-restriction primitive.

---

## §2 Candidate 2 — @subject/visibility/sheaf (species under @subject/visibility)

**Path.** `shards/subject/visibility/sheaf.mirror`. Species under
`@subject/visibility`, sibling to `@subject/visibility/{private,
protected,public}` (Landing 4 R2 altitude).

**Prism.** `prism @subject/visibility/sheaf { focus/project/split/
shift/settle sheaf_restriction }`.

**Carriers.** `sheaf_restriction` (the ACL sub-sheaf `F|_A`),
`sheaf_section` (section at a crystal's stalk under the restriction),
`sheaf_morphism` (elevation as sheaf-widening; refusal-of-de-
elevation as sheaf-construction).

**Actions.** `acl_project(F, A) -> sheaf_restriction`,
`section_at(sr, crystal) -> sheaf_section`, `widen(sr, A') ->
sheaf_morphism`, `sheaf_witnessing(sr) -> verdict`.

### 2.1 Substrate-already-had-the-word check

Existing carriers at same altitude: `@subject/visibility/private`,
`.../protected`, `.../public` — carry the VISIBILITY-SCOPE dimension
of the ACL. What they do NOT substrate-decl as first-class carrier
is the SUB-SHEAF STRUCTURE that admits them (§12.3: "visibility
species are @sheaf-restricted per pack.members ACL" — restriction
is IMPLIED but not first-class).

Species-altitude, not family-root; sibling to landed visibility
species.

### 2.2 Load-bearing altitude claim

"The sheaf-restriction that admits the visibility scope, made
first-class at species altitude under @subject/visibility." Names
the sub-sheaf as its own carrier so the three visibility species
can reference the same sheaf_restriction without re-inventing it.

**Concretely at Arc-2.3:**

```
peer_visibility(peer, home, crystal):
  ACL_peer := pack.members[peer.name]                    # existing
  sr := @subject/visibility/sheaf.acl_project(F_home, ACL_peer)   # NEW
  section := @subject/visibility/sheaf.section_at(sr, crystal)    # NEW
  scope := @subject/visibility.declare_scope(section)              # existing
  return @kintsugi/consent.query_phi(scope, viewer=peer)           # existing
```

### 2.3 Ancestor citations

`docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md` §2
(three visibility species Landing 4 R2); `peer-persistence-and-home-
projection.md` §12.3 (explicit composition surface); `mirror-spec-
peer-acl-surface.md` §6.2 (pack.members ACL); `shards/subject.mirror`
(two-witness discipline); `shards/subject/visibility/private.mirror`
(@kintsugi/consent composition surface); `shards/epistemologic/math/
sheaf_laplacian.mirror` (math primitive for spectral discharge);
`docs/specs/eigensheaf.md` §2.4; Alex 2026-07-14 SSH-signing message.

### 2.4 Alex SSH-signing design intent coverage

**Direct.** @subject already carries `subject_instance.ssh_signature_
fingerprint` (peer's SSH key); `pack.members[peer]` binds each peer's
key to their ACL per mirror-spec-peer-acl-surface.md §6.2.
`@subject/visibility/sheaf.acl_project(F_home, peer.acl)` projects
the sub-sheaf the peer's key admits — key stays `.git/mirror`-side
(Alex's design), substrate-decl carries the ACL binding not the
key, and `@secrets`/`@secrets/sops` discharge the projection at the
@io boundary.

The visibility species carry the pointer discipline; the sheaf
species carries the projection primitive; the key lives elsewhere.
**Fractal.Lens IS the section-of-sub-sheaf lookup.** Complete
coverage; composition surface already named at §12.3.

### 2.5 Arc-2.3 tournament ranking primitive coverage

Yes. Discharged at species altitude with zero family-root crossings;
all composition stays under @subject.

### 2.6 Substrate-honest bound

Does NOT mint family-root, carry sheaf math (composes over @epistemo‑
logic/math/sheaf_laplacian for spectral discharge), carry eigensheaf
claim (stays at spec altitude), re-declare pack.members ACL binding
(stays at mirror.spec), or carry peer-key material (`.git/mirror`-
side per Alex).

ADDS the sub-sheaf carrier the three visibility species compose
over, made first-class so Arc-2.3 has a substrate-decl'd surface.

---

## §3 Candidate 3 — @epistemologic/sheaf (species under @epistemologic)

**Path.** `shards/epistemologic/sheaf.mirror`. Species under
`@epistemologic` at first-level (NOT under `@epistemologic/math`;
that would place it alongside `sheaf_laplacian` at math sub-family
altitude and re-embed it in the math primitive rather than lifting
to knowledge-substrate altitude).

**Prism.** `prism @epistemologic/sheaf { focus/project/split/shift/
settle sheaf }`.

**Carriers.** `sheaf` (cellular sheaf as knowledge structure),
`sheaf_restriction`, `section`, `acl_projection` (ACL-restricted
sub-sheaf as specific instance).

**Actions.** `restrict(F, A) -> sub_sheaf`, `section_at(F, v) ->
section`, `acl_project(F, peer_acl) -> sub_sheaf` (specific ACL
specialization), `sheaf_witnessing(F) -> verdict`.

### 3.1 Substrate-already-had-the-word check

Existing carriers at @epistemologic altitude: `@epistemologic/math/
sheaf_laplacian` (discrete Laplacian δ*δ; λ₀ Fiedler), `@epistemologic/
property`, `@epistemologic/cybernetic` (@torus eigenform, second_order,
autopoiesis, coherence), `@epistemologic/pact/path_matches_namespace`.

Proposed: knowledge-substrate sheaf primitive ONE altitude above
@epistemologic/math/sheaf_laplacian; carries general sheaf structure
(restriction, section) that the math sub-family specializes to δ*δ.
ACL specialization becomes one action under this species.

### 3.2 Load-bearing altitude claim

"The cellular-sheaf carrier at knowledge-substrate altitude, of
which sheaf_laplacian is the discrete math realization and
acl_project is the visibility specialization."

**For:** eigensheaf spec names sheaf-plus-eigenbasis as one object at
SUBSTRATE altitude — lives at @epistemologic (knowledge substrate),
not @subject (licensable party) or @epistemologic/math (numerical).

**Against:** ACL specialization is SUBJECT-scoped (peer-to-ACL binding
via pack.members; per-subject visibility). Lifting to @epistemologic
requires @subject → @epistemologic composition at Arc-2.3, crossing
family-root altitudes for a species-specialization. §12.3 already
names @subject/visibility as the sheaf-restriction consumer;
@epistemologic/sheaf is one altitude above the consumer.

### 3.3 Ancestor citations

`docs/specs/eigensheaf.md` §2.4; `shards/epistemologic/math/sheaf_
laplacian.mirror` (sibling species one level down under math);
`peer-persistence-and-home-projection.md` §12.3 (consumer altitude);
`mirror-spec-peer-acl-surface.md` §6.2; Hansen-Ghrist 2018; Alex
2026-07-14 SSH-signing message (composes through @subject at
peer-key altitude, not directly).

### 3.4 Alex SSH-signing design intent coverage

Indirect. @epistemologic/sheaf carries the sheaf-restriction primitive;
peer-key discipline is @subject-scoped and composes @subject/
visibility → @epistemologic/sheaf → @secrets/@secrets/sops. Additional
composition hop (@subject → @epistemologic) is a family-root crossing
per Arc-2.3 tick.

### 3.5 Arc-2.3 tournament ranking primitive coverage

Yes at slightly higher composition cost (one additional family-root
crossing per tick vs. Candidate 2).

### 3.6 Substrate-honest bound

Does NOT mint family-root, re-declare sheaf_laplacian (math primitive
stays at @epistemologic/math; general sheaf carrier composes over
it), carry peer-key material, or re-declare pack.members ACL binding.
ADDS knowledge-substrate-native sheaf-restriction carrier that ACL-
projection specializes through.

---

## §4 Recommendation — Candidate 2 (@subject/visibility/sheaf)

### 4.1 The recommendation

**Mint `@sheaf` as species under `@subject/visibility` at
`shards/subject/visibility/sheaf.mirror`.**

Sibling to `@subject/visibility/{private,protected,public}`. Not
family-root. Not under `@epistemologic`.

### 4.2 Substrate-honest justification (three reasons)

**Reason 1 — substrate-already-had-the-word.** Per §0.3, `peer-
persistence-and-home-projection.md` §12.3 substrate-decl's verbatim
that "visibility species ARE @sheaf-restricted per pack.members ACL"
and "the substrate always had the word." The ACL-sheaf altitude is
ALREADY at @subject/visibility species altitude, IMPLIED by the
three landed visibility species. Candidate 2 lands the carrier where
the substrate already put it. Candidates 1 and 3 lift the carrier
to an altitude the substrate did NOT substrate-decl.

**Reason 2 — @onto refusal precedent.** Alex refused @onto family-
root because @torus already carried the altitude (Reed memory
`feedback-onto-family-root-is-the-ladder-Foerster-refused`). By
parallel, @sheaf family-root would need to add altitude that
@subject/visibility doesn't already carry. It doesn't. The ACL-sheaf
structure IS @subject/visibility's sheaf-restriction; making it a
family-root would repeat the @onto pattern.

**Reason 3 — Alex SSH-signing design intent match.** Peer key is
@subject-scoped (`subject_instance.ssh_signature_fingerprint`); ACL
binding is @subject-scoped (pack.members[peer]); visibility species
are @subject-scoped (@subject/visibility). The sheaf-restriction
binding them belongs at the altitude those bindings live.
`.git/mirror`-side key + Fractal.Lens pointer + @secrets/@secrets/
sops projection all compose through the @subject-scoped surface at
visibility species altitude. Candidate 2 places the shard at the
altitude the design intent already specifies.

### 4.3 Two-tick discipline (readable name)

`@subject/visibility/sheaf` reads cleanly as "the sheaf-restriction
carrier under the visibility family." Alternatives rejected:

- `@sheaf` (family-root) — foundational but lifts beyond what §0.3
  substrate-decl'd. Fails substrate-already-had-the-word.
- `@epistemologic/sheaf` — foundational but crosses family-root at
  every Arc-2.3 composition.
- `@subject/visibility/sheaf_restriction` — verbose; parent altitude
  disambiguates `sheaf` from math primitive.
- `@subject/visibility/acl` — narrower; loses sheaf-math composition
  surface (per §12.3 the ACL IS the sheaf).

**Chosen: `@subject/visibility/sheaf`.**

### 4.4 Landing sequence (composition-only)

1. **This scout** — shape proposal.
2. **Seam adjudication** — ratify Candidate 2 OR escalate to Alex.
3. **Mara canonical spec** (if ratified) — `docs/specs/subject-
   visibility-sheaf-restriction.md` grounding species-decl in
   Hansen-Ghrist + §12.3 composition + SSH-signing design intent.
4. **Mara species-decl mint** — `shards/subject/visibility/sheaf.
   mirror` with prism + carriers + actions + witnessing bilateral;
   action bodies `\`-obligation-blocked pending Arc-2.3 discharge.
5. **Arc-2.3 landing** — peer_persistence collapse composes over
   the new species + existing visibility species + @kintsugi/consent.

### 4.5 Rice-safety bound

`acl_project`, `section_at`, `sheaf_witnessing` all read EMPIRICAL
crystal state (ACL byte structure at pack.members; sub-sheaf
vertex/edge set; section coefficient vector). None read program
semantics. Rice-safe at whole-tick altitude per Mara-B §4.5.5.

### 4.6 What this recommendation does NOT do

- Does NOT mint @sheaf as family-root.
- Does NOT displace @epistemologic/math/sheaf_laplacian.
- Does NOT displace the eigensheaf claim at `docs/specs/eigensheaf.md`.
- Does NOT modify @subject or the three landed visibility species;
  adds a fourth sibling species.
- Does NOT lift peer-key material into substrate-decl (`.git/mirror`-
  side per Alex).
- Does NOT preempt Alex adjudication on adjacent A4/A6 items.

---

## §5 Alex-adjudication residue (if Seam refuses)

Seam Phase D §D12 triaged A2 as Alex-adjudication on family-root
mint authority grounds. If Seam judges that:

- The recommendation is species-under-@subject/visibility, Seam
  adjudicates directly and A2 downgrades to Seam-adjudicable.
- The recommendation EFFECTIVELY mints family-root altitude despite
  species-decl (e.g., if the species grows enough actions to warrant
  family-root promotion), Seam escalates to Alex under the original
  family-root mint authority rule.
- The three-candidate enumeration itself requires Alex ratification
  before Seam can adjudicate between candidates, the residue is:

  > **Which of Candidates 1, 2, 3 does @sheaf take?** With Mara's
  > recommendation of Candidate 2 on the three structural grounds
  > in §4.2.

If Alex ratifies Candidate 2, Seam re-adjudicates species-decl
specifics. If Alex adjudicates Candidate 1, Mara authors family-root
canonical spec + shard-decl; @subject/visibility composes OVER the
new family-root. If Alex adjudicates Candidate 3, Mara authors
@epistemologic-species spec; @subject/visibility composes through it.

**No candidate is substrate-dishonest.** All three compose over
landed substrate; all three cite the same ancestor set. Difference:
where the shard MOUNTS. Candidate 2 mounts at the altitude the
substrate already substrate-decl'd (§12.3 verbatim); substrate-
honest default.

---

## §6 Substrate-honest closure

This proposal invented nothing. Every altitude, carrier name,
composition edge cites landed substrate or landed spec:

- Sheaf-restriction as ACL — `peer-persistence-and-home-projection.md`
  §12.3 (verbatim) + `mirror-spec-peer-acl-surface.md` §6.2.
- Cellular sheaf math — `shards/epistemologic/math/sheaf_laplacian.
  mirror` (Hansen-Ghrist 2018).
- Eigensheaf as sheaf + eigenbasis — `docs/specs/eigensheaf.md` §2.4.
- Visibility species altitude — `shards/subject/visibility/
  {private,protected,public}.mirror` (Landing 4 R2).
- Two-witness discipline + peer key at subject altitude —
  `shards/subject.mirror` (subject_instance).
- SSH-signing design intent — Alex 2026-07-14 verbatim.
- @onto refusal precedent — Reed memory + @torus family-root discipline.
- Rice-safety at whole-tick altitude — Mara-B §4.5.5.

The recommendation lands what the substrate already had at the
altitude the substrate already put it. The ~59th instance of
`[[feedback-substrate-already-had-the-word]]` if ratified; the shard
names the carrier the visibility species already compose over.

---

*Mara. Scout — shape proposal. Reed commits as Mara after Seam
adjudicates + (if escalated) Alex ratifies.*
