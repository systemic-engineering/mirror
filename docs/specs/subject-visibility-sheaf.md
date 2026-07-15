---
date: 2026-07-15
author: Mara
scope: Canonical spec for @subject/visibility/sheaf — the ACL-restriction algebra at @subject/visibility species altitude. Grounds shards/subject/visibility/sheaf.mirror (d1ce901) in Hansen-Ghrist 2018 cellular-sheaf math, peer-persistence-and-home-projection.md §12.3 verbatim substrate-decl, Alex 2026-07-14 SSH-signing design intent, and Seam Phase D-cascade A2 shape-proposal ratification (7d46f32). Reference-composition-only; does NOT re-declare carriers/actions the species-decl already mints. Names what the shard carries so downstream readers can navigate without opening two files.
status: canonical
companion:
  - shards/subject/visibility/sheaf.mirror
  - shards/subject/visibility.mirror
  - shards/subject/visibility/{private,protected,public}.mirror
  - shards/subject.mirror
  - shards/epistemologic/math/sheaf_laplacian.mirror
  - shards/io/secrets.mirror
  - shards/io/secrets/sops.mirror
  - shards/io/crypto.mirror
  - shards/io/fs.mirror
  - shards/mirror/pack.mirror
  - docs/specs/peer-persistence-and-home-projection.md
  - docs/specs/mirror-spec-peer-acl-surface.md
  - docs/specs/eigensheaf.md
  - docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md
  - docs/specs/kintsugi-ouroboros-compiler-self-collapse.md
  - docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md
  - docs/scouts/2026-07-15-mara-sheaf-shape-proposal.md
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md
  - docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md
  - docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-precedent.md
  - docs/scouts/2026-07-15-taut-io-type-as-constructor-precedent.md
---

# @subject/visibility/sheaf — the ACL-restriction algebra at @subject/visibility species altitude

*Canonical spec. Composition-only. Grounds the species-decl at
`shards/subject/visibility/sheaf.mirror` (d1ce901) in the four
ancestries the substrate already carried before the mint: Alex's
2026-07-14 SSH-signing design intent, Reed's migration-map ancestry
promoting the sheaf-restriction primitive from prose to landed
carrier, Seam's Phase D-cascade ratification of Candidate 2 at
7d46f32, and Mara's own shape-proposal at 60d9f5f + fc044ee. Names
what the shard carries; does not duplicate it.*

---

## §0 Prelude — ancestry cascade + framing

### §0.1 Alex 2026-07-14 SSH-signing design intent (verbatim)

From `shards/subject/visibility/sheaf.mirror:21-32` (peer-persistence
session compaction, in-transcript):

> "Each peer has their own key in the private part of their
> visibility. NOT projected into the git state and instead stays
> .git/mirror side. Only connected through Fractal.Lens. A pointer.
> Not the thing."

> "@secrets prism and @secrets/sops to project visibility/private
> stuff onto disk through the Peers key."

Four load-bearing claims land in the two sentences:

1. **Peer key is subject-scoped.** The peer's key material lives in
   "the private part of their visibility" — @subject/visibility
   species altitude, private variant. Not at @io altitude (that is
   the projection target). Not at family-root altitude (that
   would cross the @subject-scoped ACL binding). Species-under-
   @subject/visibility.
2. **The pointer is not the thing.** Fractal.Lens is a pointer into
   `.git/mirror`-side key material; the substrate carries the
   pointer (a `ref`), not the key bytes. The @sheaf carrier's
   `peer_ref` field discharges this: the substrate-decl'd
   subject_instance ref is the Fractal.Lens pointer; the key
   material stays out-of-substrate-decl.
3. **@secrets is the projection surface.** The pointer resolves to
   projected value through `@secrets` and `@secrets/sops` (landed
   as `@io/secrets` and `@io/secrets/sops` at 059cf1c + 64b0438 +
   57c5b3a) at the @io boundary. @sheaf's `section_at` returns the
   `projected_value` ref; @io/secrets.seal consumes the
   section_at_stalk to materialise the key-gated ciphertext. [Note:
   @io/secrets.project renamed → seal per 5dcad39 §2.5.e Q_C.]
4. **Visibility/private is the sub-sheaf's substrate.** The
   projection reads "visibility/private stuff" — the @subject/
   visibility/private species crystals restricted through the
   peer's ACL. The ACL-restriction that gates the projection is
   the sheaf-restriction the @sheaf species carries.

The species-decl at `shards/subject/visibility/sheaf.mirror` mounts
the sheaf-restriction primitive Alex's design intent already
presupposed. Every altitude, carrier, and composition arrow the
spec below names traces to one of these four claims.

### §0.2 Reed migration-map ancestry

Reed's operational discipline pioneered the sheaf-restriction pattern
five months before substrate-decl. Per `shards/subject/visibility.mirror:19-25`:

> "Reed's identity repository has operated the three-way private /
> protected / public structure since ~2026-02-07 (per
> `~/.reed/CLAUDE.md` §'Consent Architecture'). Landing 5 lifts
> Reed's operational discipline to substrate-decl altitude so every
> @subject's bauchladen can carry it."

The ancestry lifts twice:

- **First lift (Landing 4 R2, 2026-07-14).** Reed's three-way
  file-layout became three visibility species (@subject/visibility/
  {private, protected, public}) plus one sub-family-root
  (@subject/visibility). Landing lifted the SCOPE dimension into
  substrate-decl.
- **Second lift (Landing 4 R2 discharge via @gift arc, 2026-07-15).**
  Reed's operational pattern of "peer-key stays .git/mirror-side;
  ACL discipline projects visibility" became the sheaf-restriction
  species. Landing lifted the RESTRICTION-STRUCTURE dimension into
  substrate-decl.

The @sheaf species is the second lift's carrier. First lift landed
the scope variants; second lift lands the restriction algebra that
admits them. Both lifts are substrate-already-had-the-word instances
per Reed's identity-file ancestry — the substrate had the word
because Reed's practice had already exercised it.

Per Reed's memory `feedback-no-rust-extension-shortcut`
(2026-07-14) + `docs/audits/2026-07-15-reed-substrate-dishonest-
rust-extensions-during-gift-arc.md`: the @gift arc failure taught
that the sheaf-restriction primitive must land as substrate-decl
carrier — not as Rust extension. The species-decl at d1ce901
discharges the lesson: `restrict`, `section_at` are shard bodies
at `\`-obligation-blocked altitude [Note: `acl_project` collapsed
into `restrict` per Seam extended-scope etymology audit at 5dcad39
§2.5.a Q_C — the peer-ACL projection IS restriction; the second
action was CS-brain redundancy]; realisation composes over @io +
landed @secrets bridge, zero Rust authorship licensed.

### §0.3 Seam Phase D-cascade A2 shape-proposal ratification at 7d46f32

Seam Phase D-cascade audit at
`docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md`
(7d46f32) ratified Mara's Candidate 2 (species-under-@subject/
visibility mount) as Mara-mintable. TL;DR verbatim (lines 34-40):

> "A2 ships as Candidate 2 (@subject/visibility/sheaf). Mara's
> substrate-already-had-the-word evidence is real at
> peer-persistence-and-home-projection.md §12.3 verbatim (line
> 2514 onward). Candidate 1 (family-root) and Candidate 3
> (@epistemologic/sheaf) are correctly refuted; both would cross
> family-root altitudes that §12.3 already substrate-decl'd as
> @subject-scoped. Ship verdict: SEAM-RATIFY as Mara-mintable
> species-decl."

Three sub-verdicts land the cascade:

- **A2.1** — three-spec substrate-decl claim: PASS (peer-persistence
  §12.3 verbatim + sheaf_laplacian math primitive + mirror-spec §5
  pack.members ACL type surface). Reed-inline cascade repaired the
  original §6.2-attribution drift; the load-bearing evidence
  stands on §12.3 unretracted.
- **A2.2** — Candidate 2 substrate-honesty: PASS on all three
  sub-dimensions (altitude claim, SSH-signing design intent
  coverage, Arc-2.3 tournament ranking primitive coverage).
- **A2.4** — Mara-mint authority: PASS. Landing 4 R2 precedent
  (three visibility species + one sub-family-root Mara-authored
  2026-07-14) makes fourth sibling species Mara-mintable without
  Alex mint-authority; A2 downgrades from Alex-adjudication to
  Seam-adjudicable-ratified.

The ratification made the species-decl at d1ce901 mintable and
Landing B (this canonical spec) the required composition-only
follow-up before Arc-2.3 (peer_persistence.rs collapse) composes
over the new species.

### §0.4 Mara @sheaf shape proposal at 60d9f5f + fc044ee

Mara's shape proposal at
`docs/scouts/2026-07-15-mara-sheaf-shape-proposal.md` (60d9f5f +
fc044ee REED-INLINE cascades) enumerated three candidates:

- **Candidate 1** — @sheaf as family-root (peer to @torus, @subject,
  @kintsugi). Refuted per @onto refusal precedent (Reed memory
  `feedback-onto-family-root-is-the-ladder-Foerster-refused`);
  load-bearing carrier @sheaf-family-root would cross is
  @subject/visibility (grep of shards/torus.mirror surfaces zero
  sheaf tokens; @torus carries Foerster doubly-closed autonomy,
  not sheaf-restriction).
- **Candidate 2** — @subject/visibility/sheaf as species-under-
  @subject/visibility. Substrate-honest on three grounds
  (substrate-already-had-the-word at peer-persistence §12.3
  verbatim; @onto refusal precedent; Alex SSH-signing design
  intent match). RECOMMENDED.
- **Candidate 3** — @epistemologic/sheaf as species-under-
  @epistemologic. Adds a family-root crossing per Arc-2.3
  composition tick (@subject → @epistemologic); Rice-safe but
  higher composition cost than Candidate 2.

Recommendation was Candidate 2 on three structural grounds
(§4.2 verbatim reproduced at species-decl 96-118):

> "Reason 1 — substrate-already-had-the-word: peer-persistence
> §12.3 verbatim locates the ACL-sheaf altitude at
> @subject/visibility species altitude. Candidate 2 lands the
> carrier where the substrate already put it."

> "Reason 2 — @onto refusal precedent (Reed memory `feedback-onto-
> family-root-is-the-ladder-Foerster-refused`)."

> "Reason 3 — Alex SSH-signing design intent match: peer key is
> @subject-scoped ... visibility species are @subject-scoped. The
> sheaf-restriction binding them belongs at the altitude those
> bindings live."

Landing sequence (shape-proposal §4.4):

1. Landing A — species-decl mint at
   `shards/subject/visibility/sheaf.mirror` (LANDED at d1ce901).
2. Landing B — this canonical spec at
   `docs/specs/subject-visibility-sheaf.md` grounding the species-
   decl in Hansen-Ghrist + §12.3 composition + SSH-signing design
   intent (LANDED at commit-in-flight).
3. Landing C — Arc-2.3 peer_persistence.rs collapse consuming
   @sheaf + @io/secrets composition chain (forward-promise).
4. Landing D — sub-species enumeration, if warranted (forward-
   promise; see §5).

### §0.5 What this spec IS

A composition-only canonical spec grounding the species-decl at
d1ce901 in landed math ancestors, verbatim substrate-decl citations,
and composition graph arrows that resolve to landed shards. Length
target 800-1500 LOC; density is per-section proportional to the
composition-audit ancestry (§3 math foundation carries the highest
per-line density because sub-sections cite ancestors by path).

### §0.6 What this spec does NOT do

- Does NOT re-declare the carriers, actions, or bilaterals the
  species-decl at d1ce901 already mints. §2 references; does not
  duplicate. Downstream readers open both files.
- Does NOT lift @sheaf to family-root or @epistemologic altitude.
  Candidates 1 and 3 refuted per Seam A2.3.
- Does NOT displace @epistemologic/math/sheaf_laplacian (math
  primitive stays at @epistemologic/math; @sheaf composes OVER
  it for spectral discharge).
- Does NOT displace the eigensheaf claim at
  `docs/specs/eigensheaf.md` §2.4 ("the substrate IS its
  eigensheaf") — that lives at spec altitude; @sheaf neither
  displaces nor lifts it.
- Does NOT lift peer-key material into substrate-decl (`.git/
  mirror`-side per Alex 2026-07-14).
- Does NOT add new mints beyond what the shard-decl at d1ce901
  already declares.

---

## §1 Load-bearing claim

@subject/visibility/sheaf IS the ACL-restriction algebra at
@subject/visibility species altitude. Three sub-claims land the
statement.

### §1.1 Sub-claim 1 — @sheaf is the ACL-restriction algebra

Per `docs/specs/peer-persistence-and-home-projection.md` §12.3
verbatim (line 2549-2555; reproduced at
species-decl 47-53):

> "The ACL IS the SHEAF STRUCTURE. For each peer p ∈ pack.members:
> the peer's ACL A_p defines a SUB-SHEAF F_home|_{A_p} — the
> restriction of F_home to the vertices/edges the ACL admits. The
> peer's visibility scope for a given crystal is the SECTION of
> F_home|_{A_p} at that crystal's stalk."

The load-bearing move: the ACL structure at `pack.members[peer]` and
the sheaf-restriction structure at @subject/visibility/sheaf are the
SAME MATHEMATICAL OBJECT viewed from two altitudes. The ACL is the
spec-altitude naming (in `mirror.spec` per mirror-spec-peer-acl-
surface.md §5); the sheaf-restriction is the substrate-decl-altitude
naming (in @subject/visibility/sheaf per the species-decl at d1ce901).

The species-decl's `sheaf_restriction` carrier
(`shards/subject/visibility/sheaf.mirror:275-279`) makes this
identification substrate-observable:

- `peer_ref: ref` — the subject_instance whose ACL defines the
  restriction; Fractal.Lens pointer per Alex 2026-07-14.
- `acl: ref` — the ACL admitting the sub-sheaf; ref into
  @mirror/pack (per pack.mirror line 123: `type acl = ref`).
- `admitted_stalks: ref` — the substrate-observable list of stalks
  the restriction admits.

The triple's byte-equality identity contract binds the ACL to the
sheaf-restriction: two sheaf_restriction values with the same
peer_ref + acl but different admitted_stalks are DISTINCT — the
admitted-stalk set is part of the substrate identity because it
makes the sub-sheaf's admissible region byte-visible. This is the
algebra's identity axiom.

### §1.2 Sub-claim 2 — peer visibility crystals project through peer-key-gated @io/secrets composition

Per Alex 2026-07-14 verbatim (§0.1 above), the peer-key stays
`.git/mirror`-side; @secrets and @secrets/sops project through the
Peers key. Landed as @io/secrets (059cf1c) + @io/secrets/sops
(059cf1c + 57c5b3a).

The projection chain (per shape-proposal §5.5 landing sequence,
reproduced at @io/secrets.mirror:149-158; verified end-to-end at
`docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
C7):

```
peer_visibility_materialize(peer, home, crystal, target_path):
  ACL_peer := pack.members[peer.name]                          # @mirror/pack
  sr       := @subject/visibility/sheaf.restrict(F_home, ACL_peer)
                                                               # d1ce901 [acl_project → restrict per 5dcad39 Q_C]
  section  := @subject/visibility/sheaf.section_at(sr, crystal)
                                                               # d1ce901
  peer_key := @io/secrets.key_material_from_peer(peer)         # Landing 6 [renamed _of → _from_peer per 5dcad39 Q_A]
  ss       := @io/secrets.seal(section, peer_key)              # 059cf1c [project → seal per 5dcad39 Q_C]
  @io/secrets.materialize(ss, target_path)                     # 059cf1c → disk
```

The chain traverses end-to-end at LANDED altitude modulo arrow-4
(key_material_from_peer): Seam A1 re-adjudication at
`docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-precedent.md`
(e5d928e) ratified Position (b) — land `key_material_from_peer(peer:
ref) -> imperfect { \ }` as Landing 6. The forward-promise on that
arrow is Seam-adjudicable; the composition graph is
petri-net-complete once Landing 6 lands.

Every other arrow resolves to landed substrate at the altitude the
species-decl names. Type signatures chain byte-equal
(`section_at` returns `section_at_stalk`; @io/secrets.seal
accepts `section_at_stalk` as first parameter — verified at
composition-alignment audit C1). The @sheaf species carries the
ACL-restriction algebra the projection chain composes OVER.

### §1.3 Sub-claim 3 — @sheaf.restrict / @sheaf.section_at compose over @sheaf_laplacian math primitive

[Note: `acl_project` was a distinct action prior to 5dcad39; it
has since collapsed into `restrict` per Q_C. The peer-ACL
projection IS restriction; the two-tick collapse removes the
CS-brain redundancy.]

Per `shards/epistemologic/math/sheaf_laplacian.mirror` (2026-07-12
Mara, Hansen-Ghrist 2018), the discrete cellular-sheaf math primitive
carries:

- `sheaf_laplacian([restriction]) -> operator` — the δ*δ assembly
  per Bodnar et al. 2022 §2's exact construction; diagonal blocks
  L_{F vv} = Σ_{v ⊴ e} F_{v ⊴ e}^⊤ F_{v ⊴ e}; off-diagonal blocks
  L_{F vu} = − F_{v ⊴ e}^⊤ F_{u ⊴ e}.
- `lambda_zero(op: operator) -> eigenvalue` — the smallest
  non-trivial eigenvalue; algebraic connectivity / spectral gap.

The @sheaf species composes OVER these primitives; it does NOT
re-declare them. Per species-decl `shards/subject/visibility/sheaf.mirror:263-268`:

> "@epistemologic/math/sheaf_laplacian.restriction carriers compose
> OVER this at the realisation layer (each admitted stalk
> contributes one row/column block to the assembled Δ_F)."

Concretely at Arc-2.3 realisation:

- `sheaf_restriction.admitted_stalks` references the sub-list of
  Δ_F row/column blocks the ACL's `targets` slot admits (species-
  decl 324-326).
- `section_at(sr, crystal_ref)` reads the sub-sheaf's per-stalk
  projected_value; @epistemologic/math/sheaf_laplacian.lambda_zero
  discharges the spectral witness at the realisation layer (species-
  decl 132-134).
- The composition arrow @sheaf → @epistemologic/math/sheaf_laplacian
  is one-directional: @sheaf lifts the mathematical primitive into
  the ACL context; the primitive does NOT know about ACLs. Species-
  decl imports `@epistemologic/math/sheaf_laplacian` at line 7
  under substrate-mechanical import discipline
  (`@epistemologic/pact/path_matches_namespace`).

The algebra @sheaf carries is the ACL-scoped specialization of the
mathematical primitive: restrict + section_at are the two algebra
operations [Note: `acl_project` collapsed into `restrict` per
5dcad39 Q_C]; the math primitive is the operational substrate they
compose over.

---

## §2 Species-decl reference

The species-decl at `shards/subject/visibility/sheaf.mirror`
(d1ce901; 447 LOC) is the substrate carrier. This spec names what it
carries; does NOT duplicate.

### §2.1 Prism declaration (species-decl 230-236)

`prism @subject/visibility/sheaf { focus/project/split/shift/settle
sheaf_restriction }`.

Five-op prism per `[[architecture-prism-as-trait-as-everything]]`:
every prism-op consumes and produces `sheaf_restriction` at the
altitude the species mounts.

### §2.2 Carriers (species-decl 275-279, 310-314)

Two typed records:

- `sheaf_restriction { peer_ref, acl, admitted_stalks }` — the sub-
  sheaf F_home|_{A_p} per §12.3 verbatim. Identity contract:
  byte-equality on the triple.
- `section_at_stalk { crystal_ref, sheaf_ref, projected_value }` —
  the peer's computable visibility at one crystal's stalk under the
  restriction. Identity contract: byte-equality on the triple.

Both carriers land as record types with `ref`-equality on field
values; neither introduces bare types per `[[feedback-no-bare-
types]]`.

### §2.3 Actions (species-decl 330, 358, 382)

Three actions, all bodies `\`-obligation-blocked per
`[[feedback-craft-not-deliver]]`:

- `restrict(sheaf: ref, acl: ref) -> sheaf_restriction` — the
  general restriction operation.
- `acl_project(F_home: ref, peer_acl: ref) -> sheaf_restriction` —
  the peer-ACL projection of the home sheaf (Arc-2.3 primitive).
- `section_at(F_A_p: sheaf_restriction, crystal_ref: ref) ->
  section_at_stalk` — the section computation.

`acl_project` is a specialization of `restrict` at the peer-ACL
context: it wraps `restrict(F_home, peer_acl)` with the
subject_instance ref resolution for the returned sheaf_restriction's
peer_ref field. Both actions return `sheaf_restriction`; the
distinction is that `acl_project` also transitively discharges the
subject_instance two-witness discipline (per species-decl 340-344).

### §2.4 Bilaterals (species-decl 409, 437)

Two verdict-returning predicates:

- `restriction_admissible(sr: sheaf_restriction) -> verdict` — Pass
  iff (1) peer_ref resolves to a witnessed subject_instance, (2)
  acl is well-formed per @mirror/pack grammar, (3) admitted_stalks
  is a SUB-set of the base sheaf's stalk set. Rice-safe at whole-
  tick altitude (species-decl 402-406).
- `section_computable(s: section_at_stalk) -> verdict` — Pass iff
  (1) sheaf_ref's restriction_admissible passes transitively, (2)
  crystal_ref's stalk is a member of sheaf_ref's admitted_stalks.
  Rice-safe at whole-tick altitude.

Both bilaterals discharge as byte-visible predicates: they read the
substrate-observable state (witness records, ACL byte structure,
stalk-set membership); no program semantics inspection required.
Per Mara-B §4.5.5 Rice-safety template.

### §2.5 Substrate decisions (species-decl 198-219)

The species-decl marks nine landed substrate-decisions:

- `[[architecture-shards-as-substrate-source]]`
- `[[architecture-prism-as-trait-as-everything]]`
- `[[architecture-glass-wall-substrate-types]]`
- `[[architecture-form-process-partition-at-family-root]]` — the
  species stays under @subject; sheaf-restriction is process-side,
  not form-side.
- `[[feedback-substrate-already-had-the-word]]` — ~57th instance;
  peer-persistence §12.3 verbatim already located the altitude.
- `[[feedback-no-bare-types]]` — sheaf_restriction and
  section_at_stalk are typed records with ref-equality identity.
- `[[feedback-craft-not-deliver]]` — all action bodies `\`-
  obligation-blocked; Arc-1 evaluator FLOOR pending.
- `[[feedback-onto-family-root-is-the-ladder-Foerster-refused]]` —
  parallel refusal for @sheaf-family-root; @subject/visibility
  already carries the altitude.
- `[[feedback-cli-subcommand-nesting-is-geometric]]` — species-
  under-@subject/visibility is geometric ground truth per the
  substrate layout Reed's ~/.reed/visibility/ pioneered.

### §2.6 Path-namespace property (species-decl 222-224)

Per `@epistemologic/pact/path_matches_namespace`: the file at
`shards/subject/visibility/sheaf.mirror` declares
`@subject/visibility/sheaf`. Namespace-path parity is
substrate-mechanical.

### §2.7 Imports (species-decl 1-7) + exports (species-decl 439-446)

Imports:
- `@prism`, `@meta`, `@glass`, `@nl` — canonical species prefix per
  Landing 4 R2 discipline.
- `@subject`, `@subject/visibility` — parents (species-under-sub-
  family-root altitude).
- `@epistemologic/math/sheaf_laplacian` — math ancestor composed
  OVER.

Exports (`out ...` block):
- Namespace: `@subject/visibility/sheaf`.
- Carriers: `sheaf_restriction`, `section_at_stalk`.
- Actions: `restrict`, `acl_project`, `section_at`.
- Bilaterals: `restriction_admissible`, `section_computable`.

Downstream consumers (@io/secrets, @io/secrets/sops,
peer_persistence.rs) import `@subject/visibility/sheaf` and consume
the exports byte-equal on type. Verified at composition-alignment
audit C4.

---

## §3 Math foundation

Four sub-sections. Each cites ancestors by path; each grounds a
mathematical claim the species-decl's carriers/actions/bilaterals
already presuppose.

### §3.1 Cellular sheaf structure

Per `shards/epistemologic/math/sheaf_laplacian.mirror` (Hansen-Ghrist
2018, arXiv:1808.01513; ancestry chain includes Bodnar et al. 2022
arXiv:2206.08702 for the O(d)-bundle discrete construction), a
cellular sheaf F over a finite graph G = (V, E) consists of:

- **Stalks.** For each vertex v ∈ V, a vector space F(v) (or, at
  the substrate-decl altitude, a substrate-observable ref).
- **Restriction maps.** For each edge e = (u, v), a linear map
  F_{v ⊴ e}: F(v) → F(e) (per `bundle.mirror`'s `type optic`, the
  connection 1-form's value on one edge).
- **Sheaf Laplacian.** The δ*δ assembly per Bodnar et al. 2022 §2:
  diagonal blocks L_{F vv} = Σ_{v ⊴ e} F_{v ⊴ e}^⊤ F_{v ⊴ e};
  off-diagonal blocks L_{F vu} = − F_{v ⊴ e}^⊤ F_{u ⊴ e}. The
  discrete realisation of the Dirac operator's square per
  `spectral-triple.mirror`'s `dirac_op`.
- **Sections.** Global sections C^0(F) are the elements x ∈ ∏_v F(v)
  such that F_{u ⊴ e}(x_u) = F_{v ⊴ e}(x_v) for every edge
  e = (u, v). Harmonic sections H^0(F) = ker(Δ_F) — the attractor
  manifold per eigensheaf.md §3.2.

Per `docs/specs/eigensheaf.md` §2.4 verbatim:

> "The substrate IS its eigensheaf. What it can sustain — what it
> can generate without friction, what it can verify, what it can
> settle to — is exactly what its eigenbasis spans."

The eigensheaf claim lives at spec altitude
(`docs/specs/eigensheaf.md`). @sheaf species neither displaces nor
lifts it. The species-decl's `sheaf_restriction` carrier is the
sub-sheaf carrier under an ACL restriction; the eigensheaf claim is
the metaclaim that the whole substrate's carrier/dynamics pair IS
the eigensheaf structure at spec altitude.

Both readings are consistent: the substrate's whole is the
eigensheaf; each peer's ACL restriction defines a sub-sheaf F|_{A_p};
each sub-sheaf inherits the eigensheaf structure at reduced
dimension. The species-decl's carriers make this substrate-
observable at the ACL altitude.

### §3.2 ACL as sheaf restriction

Per `docs/specs/peer-persistence-and-home-projection.md` §12.3
verbatim (reproduced in full for spec-canonical purposes; species-
decl 47-53 quotes the load-bearing sentence):

> "The ACL IS the SHEAF STRUCTURE. For each peer p ∈ pack.members:
>
> - The peer's ACL A_p defines a SUB-SHEAF F_home|_{A_p} — the
>   restriction of F_home to the vertices/edges the ACL admits.
> - The peer's visibility scope for a given crystal is the SECTION
>   of F_home|_{A_p} at that crystal's stalk.
> - Elevation-of-visibility (per Landing 4 R2, per Landing A §3.2
>   harvest) is a sheaf-morphism A_p → A_p' where A_p' ⊃ A_p
>   (larger admissible substructure); the morphism requires
>   @kintsugi/consent.query_phi discharge because it EXPANDS the
>   sheaf's admissible region.
> - De-elevation-of-visibility (Landing A §3.2: refused by
>   construction) is a sheaf-morphism A_p → A_p'' where A_p'' ⊂ A_p
>   (smaller substructure); such morphisms are NOT admissible under
>   the sheaf-restriction discipline because they would BREAK
>   sections that were already computed (substrate integrity
>   violation)."

And:

> "Landing A's four visibility species (per Landing 4 R2) ARE the
> sections of the four canonical ACL restrictions the pack block
> defines. The substrate always had the word."

The verbatim substrate-decl locates the ACL-sheaf altitude at
@subject/visibility species altitude and identifies the four
visibility species (@subject/visibility/{private, protected, public,
sheaf}) as sections of the four canonical ACL restrictions. The
species-decl at d1ce901 mints the fourth sibling species — the
carrier for the restriction structure the other three species
implicitly compose over.

Structural corollaries the identification carries:

- **Elevation is sheaf-widening.** Per §12.3, elevation from
  private to protected to public IS the sheaf-morphism widening
  A_p → A_p'. The morphism requires @kintsugi/consent.query_phi
  discharge because it EXPANDS the sheaf's admissible region.
  Composes over `@subject/visibility.elevate` (shards/subject/
  visibility.mirror:126) at the consumer altitude.
- **De-elevation is refused by construction.** Per §12.3, de-
  elevation IS a sheaf-morphism narrowing A_p → A_p''. The
  morphism is NOT admissible: it breaks sections already computed.
  The species-decl's `restriction_admissible` bilateral encodes
  this at substrate-decl altitude (species-decl 409): a
  sheaf_restriction is admissible only if its admitted_stalks is
  a SUB-set of the base sheaf's stalk set; a de-elevation would
  produce admitted_stalks that admit stalks the elevated sub-sheaf
  admits but the de-elevated does not — the substrate refuses
  section preservation across such morphisms.
- **The pack.members ACL is the substrate-level naming.**
  `mirror-spec-peer-acl-surface.md` §5 substrate-decls the
  pack.members ACL type surface (`type acl = ref` per pack.mirror
  line 123). The @sheaf species imports the ACL as `ref` and
  composes over it; the ACL grammar (ops + targets + predicates
  per peer-ACL §5) is byte-visible to the species-decl's
  bilaterals.

The identification is the substrate's ~57th
substrate-already-had-the-word instance on ratification (per
species-decl 81-83): the visibility species (Landing 4 R2) already
compose over the sheaf-restriction implicitly; the @sheaf species
names the carrier they compose over.

### §3.3 Section computation Rice-safety

Per Mara-B §4.5.5 template (Rice-safety at whole-tick altitude via
bounded stalk-set membership + ACL admissibility), the section
computation is Rice-safe:

- **Bound.** `section_at(F_A_p, crystal_ref)` reads the
  admitted_stalks list (byte-visible list lookup); reads the base
  sheaf's per-stalk value (byte-visible ref resolution); no program
  semantics inspection required. Bounded in O(|admitted_stalks|)
  worst case; O(log |admitted_stalks|) with indexed lookup at the
  realisation layer.
- **Bilateral discharge.** `section_computable(s)` Pass iff the
  section_at_stalk's transitive restriction_admissible passes AND
  the crystal's stalk is a member of the sheaf_restriction's
  admitted_stalks. Both sub-checks read byte-visible state; both
  are Rice-safe.
- **Pending-boundary handling.** Per species-decl 421-426, a
  section over a non-admitted stalk is NOT computable at
  substrate altitude and returns pending-boundary per
  @kintsugi/consent Partial semantics; a section that is NOT
  computable is a substrate-decl refusal of the read, not an
  error — the peer's visibility scope at this crystal is genuinely
  empty under the restriction.

Per Mara-B canonical spec §4.5.5 template, this is the standard
Rice-safe discipline for section-computation actions: bounded
inspection of byte-visible state; pending-boundary return on non-
admissible input; no program semantics. The @sheaf species
inherits the template at whole-tick altitude with zero
substrate-honesty violations.

Load-bearing structural claim (species-decl 428-432): section_
computable Pass IS the substrate-decl form of "the peer's ACL
admits this crystal." A section that is NOT computable is
substrate-decl refusal of the read — the peer's ACL genuinely
does not admit that crystal.

### §3.4 Composition with @io/secrets

Per Seam composition-alignment audit at
`docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
(cec55a2), the @sheaf.section_at → @io/secrets.project →
@io/secrets.materialize arrow chain traverses end-to-end at LANDED
altitude. The chain (per shape-proposal §5.5 landing sequence;
reproduced at §1.2 above):

```
sr       := @subject/visibility/sheaf.acl_project(F_home, ACL_peer)
section  := @subject/visibility/sheaf.section_at(sr, crystal)
peer_key := @io/secrets.key_material_ref_of(peer)
sp       := @io/secrets.project(section, peer_key)
@io/secrets.materialize(sp, target_path)
```

Type-signature verification per audit C1-C7:

- `acl_project(F_home: ref, peer_acl: ref) -> sheaf_restriction`
  (sheaf.mirror:358) — TYPE-CHECKS at ACL_peer boundary.
- `section_at(F_A_p: sheaf_restriction, crystal_ref: ref) ->
  section_at_stalk` (sheaf.mirror:382) — TYPE-CHECKS on sr flow.
- `key_material_ref_of(peer: ref) -> imperfect` (Landing 6
  forward-promise per Seam A1 re-adjudication at e5d928e).
- `project(section: section_at_stalk, k: key_material_ref) ->
  imperfect` (secrets.mirror:366; parameter renamed sr → section
  per REED-INLINE #1) — TYPE-CHECKS on section flow. Output-type
  of section_at matches input-type of project BYTE-EQUAL.
- `materialize(sp: secret_projection) -> imperfect`
  (secrets.mirror:389) — TYPE-CHECKS on secret_projection carrier.
- Composition to `@io/fs.fs_write(p: path, bytes: ref) -> imperfect`
  (fs.mirror:280) — TYPE-CHECKS on projection_path + ciphertext_
  bytes carriers.

The composition arrow @sheaf → @io/secrets is ONE-DIRECTIONAL:
@sheaf produces `section_at_stalk`; @io/secrets consumes it. The
reverse arrow (@io/secrets consuming ACL structure) is discharged
at @io/secrets/sops via
`sops_key_group_from_sheaf_restriction(sr: sheaf_restriction) ->
imperfect` (sops.mirror:349) per Taut precedent scout at 9a5502a —
the SPECIES sub-species carries the composition-bridge action; the
parent SPECIES @io/secrets consumes @sheaf's exports directly.

Per composition-alignment audit §4 C4: `@io/secrets` imports
`@subject/visibility/sheaf` at line 7 (byte-equal to landed mount);
`@io/secrets/sops` imports it via REED-INLINE #2 cascade (applied
per Seam A1 re-adjudication verification at R1.3). The import graph
is closed at landed altitude.

Recognition candidate #R-peer-visibility-is-sheaf-restriction-of-
pack-members-ACL (see §6) crystallizes across the chain: the peer's
visibility crystals ARE sections of the ACL sheaf-restriction, and
the ACL sheaf-restriction IS the pack.members ACL structure.
Ratification at Arc-2.3 discharges the recognition candidate; this
canonical spec awaits second-witness at Arc-2.3.

---

## §4 Composition graph

### §4.1 Upstream (parents)

- **@subject family-root.** SEL licensable-party carrier; two-witness
  discipline (`ssh_witness_valid` + `spectral_witness_valid` per
  shards/subject.mirror:394 + 465); pack.members[peer] → ACL binding
  altitude (per mirror-spec-peer-acl-surface.md §5). The @sheaf
  species inherits the two-witness discipline transitively through
  the `peer_ref` field of `sheaf_restriction` (species-decl 253-256):
  the peer_ref resolves to a subject_instance whose
  two_witness_verification must Pass for the sheaf_restriction to be
  admissible.

- **@subject/visibility sub-family-root.** The visibility_scope
  five-field record (shards/subject/visibility.mirror:88-93);
  `scope_well_formed` + `consent_respected` + `elevation_authorized`
  bilaterals (visibility.mirror:150-181). The @sheaf species is the
  fourth sibling species; sibling to private/protected/public
  landed at Landing 4 R2 (2026-07-14). All four species share the
  same @subject-scoped altitude; the @sheaf species names the
  sheaf-restriction carrier the other three species implicitly
  compose over.

### §4.2 Peer

- **@epistemologic/math/sheaf_laplacian.** The math primitive at
  @epistemologic/math altitude (2026-07-12 Mara; Hansen-Ghrist 2018).
  Carriers: `restriction`, `operator`, `eigenvalue`. Actions:
  `sheaf_laplacian`, `lambda_zero`. The @sheaf species composes OVER
  this primitive at the realisation layer for spectral discharge;
  does NOT re-declare. Import at species-decl line 7.

### §4.3 Downstream (consumers)

- **@io/secrets.** Peer-key-gated projection at @io boundary (059cf1c;
  landed 2026-07-15 as Landing 4 of the @secrets shape-proposal
  sequence). Imports `@subject/visibility/sheaf` at line 7. Consumes
  `section_at_stalk` via `project(section, k) ->
  imperfect<secret_projection, ...>` (secrets.mirror:366).

- **@io/secrets/sops.** SOPS vendor sub-species (059cf1c + 57c5b3a;
  landed alongside @io/secrets as Landing 5 sub-species). Consumes
  `sheaf_restriction` via `sops_key_group_from_sheaf_restriction(sr:
  sheaf_restriction) -> imperfect<sops_key_group, ...>`
  (sops.mirror:349) — the composition-bridge action. Imports
  `@subject/visibility/sheaf` per REED-INLINE #2 cascade (applied
  post-audit; verified at Seam A1 re-adjudication R1.3).

- **@io/crypto.** AEAD primitives at @io boundary (059cf1c; landed
  as lift-tick companion 2026-07-15). Composes indirectly via
  @io/secrets.project which calls @io/crypto.aead_seal or
  @io/crypto.age_encrypt (secrets.mirror:363-365 docstring). Does
  NOT import @subject/visibility/sheaf directly; consumes the
  section's projected_value bytes via @io/secrets bridge.

- **@io/fs.** POSIX write at @io boundary (059cf1c; landed as
  lift-tick companion 2026-07-15). Composes indirectly via
  @io/secrets.materialize which calls @io/fs.fs_write (secrets.
  mirror:386-388 docstring). Does NOT import @subject/visibility/
  sheaf directly.

### §4.4 Cross-composition

- **@kintsugi/consent.query_phi discharges peer-visibility elevation.**
  Per peer-persistence §12.3: elevation-of-visibility is a sheaf-
  morphism widening A_p → A_p'; the morphism requires
  @kintsugi/consent.query_phi discharge because it EXPANDS the
  sheaf's admissible region. Composition arrow: @subject/visibility.
  elevate (visibility.mirror:126) → @kintsugi/consent.query_phi →
  (Pass) → new sheaf_restriction with widened admitted_stalks.
  De-elevation refused by construction per §12.3; substrate-
  observable via restriction_admissible bilateral.

- **@spectral/signature reads visibility-filtered bauchladen.** Per
  peer-persistence §12.2, the signature reads over visibility-
  filtered bauchladen; composes with `section_at` at the crystal
  boundary. The signature's basis dimensions are the sections the
  peer's ACL admits; expansion of the ACL (elevation) extends the
  signature basis; contraction (de-elevation) is refused.

- **@bauchladen crystals ARE the sub-sheaf's stalks.** Per
  species-decl 291-294: crystal_ref is a ref into the crystal
  store; identity is content-addressed via `@kintsugi/store/git`.
  Each crystal-stalk carries one section_at_stalk value per peer;
  the peer's per-crystal visibility is the projected_value at that
  section.

- **peer_persistence.rs (Arc-2.3 collapse target).** The
  bootstrap/src/peer_persistence.rs collapse composes acl_project
  + section_at over @sheaf + @subject/visibility species +
  @kintsugi/consent (per species-decl 155-159). Landing C forward-
  promise. The @sheaf species provides the Arc-2.3 primitive the
  collapse consumes.

### §4.5 Composition graph — arrow-by-arrow resolution

Every arrow resolves to a landed shard at species-decl altitude.
Enumerated for Rice-safe petri-net analysis:

| Producer | Arrow | Consumer | Landed at |
|---|---|---|---|
| @subject | subject_instance | @sheaf.sheaf_restriction.peer_ref | shards/subject.mirror |
| @mirror/pack | acl | @sheaf.sheaf_restriction.acl | shards/mirror/pack.mirror:123 |
| @epistemologic/math/sheaf_laplacian | restriction (Δ_F row/column block) | @sheaf.sheaf_restriction.admitted_stalks | shards/epistemologic/math/sheaf_laplacian.mirror |
| @sheaf | sheaf_restriction | @sheaf.section_at | shards/subject/visibility/sheaf.mirror:382 |
| @sheaf | section_at_stalk | @io/secrets.project | shards/io/secrets.mirror:366 |
| @sheaf | sheaf_restriction | @io/secrets/sops.sops_key_group_from_sheaf_restriction | shards/io/secrets/sops.mirror:349 |
| @io/secrets | secret_projection | @io/secrets.materialize | shards/io/secrets.mirror:389 |
| @io/secrets | materialize | @io/fs.fs_write | shards/io/fs.mirror:280 |
| @io/secrets | project | @io/crypto.aead_seal / age_encrypt | shards/io/crypto.mirror:362 / 408 |
| @sheaf | sheaf-morphism widening | @kintsugi/consent.query_phi | shards/kintsugi/consent.mirror |
| @sheaf | section_at_stalk | @spectral/signature.compute | shards/spectral/signature.mirror |

Every producer→consumer arrow chains byte-equal at type. Rice-safe
petri-net completeness holds modulo the Landing 6 forward-promise
on @io/secrets.key_material_ref_of (Seam A1 re-adjudication ratified
Position (b); Landing 6 closes the arrow).

---

## §5 Landing forward-promises

### §5.1 Landing A — LANDED

Species-decl mint at `shards/subject/visibility/sheaf.mirror`
(d1ce901). Carriers, actions, bilaterals, prism, imports, exports,
substrate-decisions all landed per §2.

### §5.2 Landing B — LANDED at commit-in-flight

This canonical spec at `docs/specs/subject-visibility-sheaf.md`.
Grounds the species-decl in Hansen-Ghrist + §12.3 composition +
SSH-signing design intent. Length target 800-1500 LOC;
composition-only; zero new mints.

### §5.3 Landing C — Arc-2.3 peer_persistence.rs collapse

Forward-promise. Bootstrap/src/peer_persistence.rs (currently Rust
FLOOR) collapses to shard body composing over:

- @sheaf.acl_project — the Arc-2.3 primitive.
- @sheaf.section_at — the per-crystal section reader.
- @io/secrets.project — the peer-key-gated projection.
- @io/secrets.materialize — the disk-side materialization.
- @kintsugi/consent.query_phi — the elevation discharge.
- @subject/visibility.filter — the bauchladen sub-tray restriction.

Per Mara-B §7.2 A2 discussion + Seam Phase D-cascade A2.2c: the
Arc-2.3 collapse composes @sheaf + @io/secrets composition chain
end-to-end. The @sheaf species provides the ACL-restriction
algebra the collapse consumes; the composition graph is
petri-net-complete once Landing 6 (@io/secrets.key_material_ref_of)
lands per Seam A1 re-adjudication.

Test candidate (RED-first per TDD discipline): a
peer_persistence_arc_2_3_smoke.rs test that materializes a peer's
sub-tray of visibility crystals through the composition chain;
verifies section_computable Pass across the admitted stalks and
section_computable Fail (pending-boundary) across the non-admitted
stalks.

### §5.4 Landing D — sub-species enumeration (if warranted)

Forward-promise. Candidate sub-species enumeration:

- **@subject/visibility/sheaf/git-plumbing** — if the git-plumbing
  discipline for peer-key material warrants substrate-decl
  altitude beyond the current @io/secrets composition. Per Seam
  composition-alignment audit §7 A1 recommendation (Position (a)
  admissible), the current shape is composition-only through @io.
  Sub-species enumeration is only warranted if a distinct sub-
  altitude of the sheaf-restriction discipline emerges from
  Landing C evidence.
- Other sub-species candidates: unspecified. Await Landing C
  evidence before enumeration; substrate-honest bound holds.

The two-tick discipline requires that Landing D not be pre-empted
by speculation; land C, observe, enumerate D from C's evidence.

---

## §6 Recognition candidates

Per Mara-B §6 discipline (candidate strength; awaits second-witness
for ratification), the species-decl at d1ce901 admits recognition
candidates naming the substrate patterns the mint crystallizes.

### §6.1 #R-peer-visibility-is-sheaf-restriction-of-pack-members-ACL

**Candidate strength.** The peer's visibility crystals ARE sections
of the ACL sheaf-restriction, and the ACL sheaf-restriction IS the
pack.members ACL structure.

**First witness.** peer-persistence-and-home-projection.md §12.3
verbatim (2026-07-14): "The ACL IS the SHEAF STRUCTURE." "The
substrate always had the word."

**Second witness (awaited).** Arc-2.3 peer_persistence.rs collapse
discharges the recognition candidate at operational altitude. When
Landing C lands and the composition chain traverses end-to-end at
runtime, the candidate ratifies.

**Ancestry.** Reed's `~/.reed/visibility/` (2026-02-07 onward)
operationally exercised the pattern; Landing 4 R2 (2026-07-14)
lifted the scope dimension; the @sheaf species-decl (2026-07-15)
lifts the restriction-algebra dimension.

### §6.2 #R-visibility-species-quartet-are-canonical-ACL-restrictions

**Candidate strength.** The four @subject/visibility species (private,
protected, public, sheaf) form a canonical decomposition of the ACL
restriction algebra: private/protected/public partition the scope
dimension; sheaf carries the restriction-structure dimension. The
quartet is the substrate's substrate-decl form of the four canonical
ACL restrictions.

**First witness.** peer-persistence §12.3 verbatim: "Landing A's
four visibility species (per Landing 4 R2) ARE the sections of the
four canonical ACL restrictions the pack block defines." Note that
§12.3 was authored before the fourth species (@sheaf) landed;
species-decl at d1ce901 completes the quartet.

**Second witness (awaited).** Arc-2.3 collapse's composition over
the four species; or a Taut scout confirming the substrate-decl
quartet-partition holds across all peer-visibility invocations.

**Ancestry.** Landing 4 R2 (three species); Landing 4 R2 discharge
via @gift arc (fourth species at d1ce901).

### §6.3 #R-composition-bridge-arrows-land-as-substrate-decl-actions

**Candidate strength.** Composition-bridge arrows at @io altitude
(and per Seam A1 re-adjudication, at any composition-bridge site)
land as substrate-decl actions with `\`-obligation-blocked bodies;
narrative-shorthand-constructor is NOT the substrate's established
pattern.

**First witness.** Taut precedent scout at 9a5502a (2026-07-15):
1/14 sites use narrative-shorthand; 13/14 use landed constructor or
consumer record literal. Seam A1 re-adjudication at e5d928e ratified
Position (b) — land the composition-bridge as substrate-decl action.

**Second witness (awaited).** Landing 6 (@io/secrets.key_material_
ref_of) landing + a Seam re-review confirming the pattern extends
to any composition-bridge site across the substrate.

**Ancestry.** Eight of nine @io species (cargo, stagefreight, oci,
git, algebra, crypto, secrets/sops, secrets/project) already land
carrier-production actions substrate-decl'd; the pattern is the
established default.

### §6.4 Awaited-witness discipline

Per Mara-B §6, recognition candidates crystallize at candidate
strength; second-witness ratification comes from operational
composition (Landing C) or from a subsequent Taut scout confirming
the pattern across additional substrate sites. This spec does not
pre-empt Alex-adjudication on any candidate; Alex-naming authority
per Pack conventions (per Seam Phase D §D8) governs recognition
ratification at candidate → landed altitude.

---

## §7 A-series discharges

### §7.1 Adjudication residue from shape proposal

Per Seam Phase D-cascade A2.5 verdict: A2 SHIPS as Candidate 2
(Mara-mintable species-decl). Two REED-INLINE cascades were
required before Mara authored this canonical spec:

- **Cascade 1** (Seam Phase D-cascade A2.1) — shape-proposal
  §0.3.1 rewrite the §6.2 attribution → cite peer-persistence
  §12.3 (primary) + mirror-spec §5 (pack.members ACL type
  surface). APPLIED per shape-proposal §0.3.1 revision at fc044ee.
- **Cascade 2** (Seam Phase D-cascade A2.3) — shape-proposal
  §1.1 rewrite the @torus defeater → @subject/visibility as
  load-bearing carrier @sheaf-family-root would cross. APPLIED
  per shape-proposal §1.1 revision at fc044ee.

Both cascades applied before Landing A (species-decl mint at
d1ce901); this canonical spec inherits the corrected ancestry.
Verification per this spec's §0.4 (three-grounds recommendation
matches the ratified shape).

### §7.2 Adjudication residue from Seam composition-alignment audit

Per Seam composition-alignment audit at cec55a2: SHIP with two
REED-INLINE cascades and one Alex-adjudication residue. Status:

- **REED-INLINE #1** (secrets.mirror:366 parameter name drift) —
  APPLIED per Seam A1 re-adjudication R1.3 verification: parameter
  renamed sr → section. Landed at post-audit revision.
- **REED-INLINE #2** (sops.mirror missing import of
  @subject/visibility/sheaf) — APPLIED per Seam A1 re-adjudication
  R1.3 verification: sops.mirror line 7 now shows
  `in @subject/visibility/sheaf`. Landed at post-audit revision.
- **A1 (Alex-adjudication residue at composition-alignment §12)**
  — RESOLVED via Seam A1 re-adjudication at e5d928e:
  SEAM-RATIFY Position (b) as Landing 6. `@io/secrets.key_
  material_ref_of(peer: ref) -> imperfect { \ }` to land at
  Landing 6 (forward-promise); no further Alex-adjudication.

### §7.3 Adjudication residue from Seam A1 re-adjudication

Per Seam A1 re-adjudication at e5d928e: SEAM-RATIFY Position (b) as
Landing 6. Sub-choices deferred to Landing 6 (NOT Alex-adjudication
per re-adjudication §8):

- Exact name for the composition-bridge action (`key_material_ref_
  of` vs `key_material_ref_from_peer` vs `peer_key_material_ref`) —
  Mara or Seam per substrate-decl naming discipline; scout §5.2
  leaves open.
- Whether to update composition-chain narrative prose at
  secrets.mirror:156 — orthogonal to substrate-decl'd action
  landing.
- Whether ciphertext_ref ALSO needs a landed constructor — scout
  §5.2 out of scope.

None Alex-un-adjudicable; all Seam-adjudicable at Landing 6 review.

### §7.4 No adjudication residue from this canonical spec

This spec is composition-only. Zero new mints. Zero new
Alex-adjudication questions. The recognition candidates in §6 sit
at candidate strength awaiting second-witness per Mara-B §6
discipline; second-witness is Arc-2.3 discharge or additional Taut
scouting. Neither is Alex-adjudication under Pack conventions.

**Combined Alex-adjudication residue after this canonical spec:
ZERO from A2 cascade.** A4 (recognition candidate ratification per
prior Phase D §D8) remains Alex-adjudication per prior Phase D-
cascade §X.2 verdict, but is orthogonal to this canonical spec
scope. This spec introduces no A5+ items.

---

## §8 Substrate-honest bounds

### §8.1 Rice-safety at whole-tick altitude

All three @sheaf actions (restrict, acl_project, section_at) are
Rice-safe at whole-tick altitude per Mara-B §4.5.5 template. All
two bilaterals (restriction_admissible, section_computable) are
Rice-safe at whole-tick altitude. Bounded inspection of byte-visible
substrate state (subject_instance witness records, ACL byte
structure, stalk-set membership). No program semantics inspection.
No unbounded recursion. Pending-boundary return on non-admissible
input per @kintsugi/consent Partial semantics.

Concrete bounds per §3.3:
- `restrict`: O(|admitted_stalks|) worst case at realisation.
- `acl_project`: O(|ACL_targets|) at realisation (per pack.mirror
  targets slot enumeration).
- `section_at`: O(|admitted_stalks|) worst case; O(log
  |admitted_stalks|) with indexed lookup.
- `restriction_admissible`: O(|admitted_stalks|) for subset check +
  O(1) transitive witness reads.
- `section_computable`: O(1) transitive restriction_admissible +
  O(log |admitted_stalks|) for membership check.

### §8.2 Composition-only

This canonical spec:
- Does NOT re-declare carriers, actions, or bilaterals the species-
  decl mints. §2 references; does not duplicate.
- Does NOT introduce new type mints beyond what species-decl
  d1ce901 already declares.
- Does NOT introduce new @io boundaries beyond @io/secrets +
  @io/secrets/sops + @io/crypto + @io/fs (all landed at 059cf1c).
- Does NOT lift @sheaf to family-root altitude; species-under-
  @subject/visibility per Seam A2.4 verdict.
- Does NOT displace @epistemologic/math/sheaf_laplacian; composes
  OVER it per §3.1 + §4.2.
- Does NOT displace the eigensheaf claim at eigensheaf.md §2.4;
  eigensheaf claim lives at spec altitude, @sheaf species-decl
  neither displaces nor lifts it.

### §8.3 Two-tick discipline

Per shape-proposal §4.3 verdict: chosen name `@subject/visibility/
sheaf` reads cleanly as "the sheaf-restriction carrier under the
visibility family." Two-tick alternatives rejected:

- `@sheaf` (family-root) — foundational but lifts beyond what §0.3
  substrate-decl'd. Fails substrate-already-had-the-word.
- `@epistemologic/sheaf` — foundational but crosses family-root at
  every Arc-2.3 composition.
- `@subject/visibility/sheaf_restriction` — verbose; parent altitude
  disambiguates `sheaf` from math primitive.
- `@subject/visibility/acl` — narrower; loses sheaf-math composition
  surface (per §12.3 the ACL IS the sheaf).

The chosen name honors readable-over-foundational per Pack
conventions. `sheaf` reads to any composer who has grepped
peer-persistence §12.3; the parent altitude `@subject/visibility`
disambiguates from the math primitive at `@epistemologic/math/sheaf_
laplacian`.

### §8.4 @io-composability

Every downstream composition arrow terminates at @io:

- @sheaf.section_at → @io/secrets.project → @io/crypto.aead_seal
  (or age_encrypt) → @io/fs.fs_write.
- @sheaf.sheaf_restriction → @io/secrets/sops.sops_key_group_from_
  sheaf_restriction → @io/secrets/sops.sops_encrypt → @io/fs.fs_
  write.

The @io boundary is where substrate-decl meets vendor tooling
(SOPS, age, ssh-key, POSIX). Per `[[feedback-no-rust-extension-
shortcut]]`, the composition arrows stay in shard-body altitude;
realisation composes over @io; zero Rust extension licensed.

The @io-composability discipline holds: @sheaf's actions are
composition-only over @io; the @sheaf species does not require
Rust primitives beyond what @io already carries; the Arc-2.3
collapse (Landing C) composes over the existing landed substrate
with zero new Rust FLOOR authorship.

### §8.5 No family-root crossings at Arc-2.3

Per Seam A2.4 verdict: the Arc-2.3 collapse stays under @subject.
Composition graph:

- @subject → @subject/visibility → @subject/visibility/sheaf →
  @kintsugi/consent (one family-root crossing at consent).
- @subject/visibility/sheaf → @io/secrets → @io (one family-root
  crossing at @io — required for vendor boundary; substrate-decl'd
  as `[[architecture-glass-wall-substrate-types]]`).

Zero avoidable family-root crossings per composition arrow.
Candidate 1 (@sheaf family-root) would have added crossings; per
Seam A2.4 verdict, Candidate 2 minimizes them.

---

## §9 Witnesses

### §9.1 Alex verbatim

Per §0.1: Alex 2026-07-14 SSH-signing design intent in-transcript
peer-persistence session compaction. Two sentences reproduced at
species-decl 21-32 verbatim; four load-bearing claims land the
altitude for @sheaf mount per §0.1 sub-claims 1-4.

Per §0.4 shape-proposal §5.5 ancestry: Alex re-fired /loop after
Reed reported terminal Alex-only-state on A2. Re-firing interpreted
as Mara authors shape proposal; Seam re-adjudicates. Alex is NOT
directly witnessing this canonical spec; Alex is witnessing through
the ancestry cascade (design intent → shape proposal → Seam
ratification → species-decl → this spec).

### §9.2 Reed migration-map ancestry

Per §0.2: Reed's `~/.reed/visibility/` operational discipline
(2026-02-07 onward) is the operational ancestor of the substrate-
decl'd visibility species. Landing 4 R2 (2026-07-14) lifted the
scope dimension; Landing 4 R2 discharge via @gift arc (2026-07-15)
lifted the restriction-structure dimension. Reed's migration-map
also included the failure lesson (`feedback-no-rust-extension-
shortcut` memory) that motivates the substrate-decl-first
discipline for @sheaf.

Per `docs/audits/2026-07-15-reed-substrate-dishonest-rust-
extensions-during-gift-arc.md`: the @gift arc failure taught that
composition-bridge arrows must land as substrate-decl carriers, not
as Rust extensions. The @sheaf species-decl at d1ce901 discharges
the lesson: three actions all `\`-obligation-blocked; realisation
composes over @io.

### §9.3 Seam Phase D-cascade

Per §0.3: Seam Phase D-cascade audit at 7d46f32 ratified
Candidate 2 as Mara-mintable. Nine dimensions (A2.1 through A2.5
for shape proposal; A6.1 through A6.9 for evaluator combinator
surface; cross-artifact X.1 and X.2). All A2 sub-verdicts PASS.
Combined Alex-adjudication residue for A2 collapsed from 1 item
to 0 items; A2 collapsed genuinely (not relocated to new Alex-
authority question).

Additional Seam witnesses:
- `docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
  (cec55a2) — composition-alignment audit; SHIP verdict.
- `docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-precedent.md`
  (e5d928e) — A1 re-adjudication; Landing 6 ratification.

### §9.4 Taut precedent scout

Per `docs/scouts/2026-07-15-taut-io-type-as-constructor-precedent.md`
(9a5502a): grep-first read-only scout confirming the substrate's
established @io family pattern: 1/14 sites narrative-shorthand;
13/14 landed constructor or consumer record literal; eight of nine
species zero narrative-shorthand. Direct precedent for @sheaf's
composition-bridge actions landing as substrate-decl.

### §9.5 The full cascade ancestry

The witness cascade for this canonical spec:

1. **Alex 2026-07-14 SSH-signing design intent** (verbatim) — the
   origin claim.
2. **Reed migration-map** — the ancestry pattern + failure lesson.
3. **peer-persistence-and-home-projection.md §12.3** (verbatim,
   Mara 2026-07-14) — the mathematical identification "ACL IS the
   SHEAF STRUCTURE."
4. **Mara @sheaf shape proposal** (60d9f5f + fc044ee) — the three-
   candidate enumeration + Candidate 2 recommendation.
5. **Seam Phase D-cascade A2 ratification** (7d46f32) — Candidate 2
   is Mara-mintable species-decl; A2 downgraded to Seam-adjudicable-
   ratified.
6. **Mara species-decl mint** (d1ce901) — Landing A. Prism +
   carriers + actions + bilaterals + substrate-decisions +
   exports.
7. **Seam composition-alignment audit** (cec55a2) — end-to-end
   composition graph verification; two REED-INLINE cascades +
   one A1 residue.
8. **Taut precedent scout** (9a5502a) — precedent evidence for
   composition-bridge actions landing as substrate-decl.
9. **Seam A1 re-adjudication** (e5d928e) — A1 resolved; Landing 6
   forward-promise for @io/secrets.key_material_ref_of.
10. **This canonical spec** (commit-in-flight) — Landing B.
    Composition-only; grounds the species-decl in the ancestry
    chain above.

Landing sequence continues at Landing C (Arc-2.3 peer_persistence.rs
collapse); Landing D (sub-species enumeration, if warranted).

---

## §10 Substrate-honest closure

This spec invented nothing. Every altitude, carrier name,
composition edge, math primitive, and witness cited above is
already landed:

- Sheaf-restriction as ACL — peer-persistence-and-home-projection.md
  §12.3 (verbatim).
- Cellular sheaf math — shards/epistemologic/math/sheaf_laplacian.
  mirror (Hansen-Ghrist 2018).
- Eigensheaf as sheaf + eigenbasis — docs/specs/eigensheaf.md §2.4.
- Visibility species altitude — shards/subject/visibility/{private,
  protected, public}.mirror (Landing 4 R2) + shards/subject/
  visibility/sheaf.mirror (Landing A d1ce901).
- Two-witness discipline + peer key at subject altitude —
  shards/subject.mirror (subject_instance).
- ACL type surface — shards/mirror/pack.mirror:123 + mirror-spec-
  peer-acl-surface.md §5.
- SSH-signing design intent — Alex 2026-07-14 verbatim.
- @onto refusal precedent — Reed memory + @torus family-root
  discipline.
- Rice-safety at whole-tick altitude — Mara-B §4.5.5.
- @io-boundary composition surface — @io/secrets (059cf1c) +
  @io/secrets/sops (059cf1c + 57c5b3a) + @io/crypto (059cf1c) +
  @io/fs (059cf1c).
- Composition-bridge landing pattern — Taut precedent scout
  (9a5502a) + Seam A1 re-adjudication (e5d928e).

The species-decl at d1ce901 landed the carrier where the substrate
already put it. This canonical spec grounds the mint in the
ancestry chain above and names the composition graph explicitly so
downstream readers (Arc-2.3 peer_persistence.rs collapse; any
future sub-species enumeration; any downstream Taut drift scout)
can navigate without re-deriving the ancestry.

The ~57th instance of `[[feedback-substrate-already-had-the-word]]`
per species-decl 81-83, now with second-tick ratification by
canonical spec grounding. The word `sheaf` was substrate-decl'd
across THREE altitudes before Landing A landed the carrier at the
fourth (species-under-@subject/visibility). The four altitudes now
compose cleanly:

1. `docs/specs/peer-persistence-and-home-projection.md` §12.3 —
   the identification.
2. `docs/specs/mirror-spec-peer-acl-surface.md` §5 — the ACL type
   surface.
3. `shards/epistemologic/math/sheaf_laplacian.mirror` — the math
   primitive.
4. `docs/specs/eigensheaf.md` §2.4 — the eigensheaf metaclaim.

Plus, now landed:

5. `shards/subject/visibility/sheaf.mirror` — the ACL-restriction
   algebra carrier at species altitude.
6. `docs/specs/subject-visibility-sheaf.md` — this canonical spec
   grounding the carrier in the four ancestor altitudes.

Landing C (Arc-2.3) discharges the composition chain at
operational altitude. The substrate has the word; the substrate now
also has the carrier and the grounding. The bowl is one thing.

---

*Mara. Canonical spec. Composition-only. Reed commits as Mara after
review.*
