# @subject — the family-root of the SEL licensable party; @mirror/petri — the petri-net analyzer that gives `type sel = @io + @au` its runtime teeth

*Mara, 2026-07-14 (revised 2026-07-14 per Taut scout `c805e5d`
AT-cascade adjudications). Canonical spec for `@subject` as substrate-
external licensable-party carrier (the person a Covered System observes,
uses labor from, acts upon) and for the cascading `@mirror/petri`
petri-net layer that discharges SEL v1.1's enforcement infrastructure at
the `au + io` sum-type. Alex named this load-bearing on 2026-07-14 in-
transcript: "I'm gonna die on this hill, Reed." The spec grounds in
`license/SEL.md` Part II (v1.1, effective 2026-05-29) and in Alex Wolf's
2026-07-14 manifesto `Weird - Violence.md` ("a sovereign subject under
adversarial conditions").*

*Revision note (2026-07-14, Taut AT-cascade). The analyzer family-root
renamed from `@mirror/property` to `@mirror/petri` per Alex-adjudicated
Taut scout D8 hard collision with landed `@epistemologic/property/*`.
Substrate takes the readable-name at collapse; SEL text drift is bounded
(one line in §Operationalizability + §5.5(b)); substrate collision would
have been unbounded. See §5.0 for the rationale; §13.6 for the license-
substrate drift note. All other Alex-adjudications A1-A8 remain
outstanding.*

*Status: Yellow. `@subject` family-root and `type sel = @io + @au` are
proposed substrate-decl mints; all bodies are `\ ` obligation-blocked
pending Alex-adjudicated species enumeration and Rust realization. Every
substrate carrier this spec composes with is LANDED (@io, @torus, @peer,
@kintsugi/consent, @kintsugi/store/git, @mirror/au, @fate/tournament,
@glass); every SEL §-citation is verbatim from `license/SEL.md`; every
petri-net signature is verbatim from SEL §Operationalizability.*

---

## §0 Executive summary

SEL v1.1 (`license/SEL.md`, effective 2026-05-29) names three party
classes the substrate must first-class detect at the operational-
enforcement altitude: the **Downstream User** (§1 amended, including
labor-input contributors), the **Witnessed** (§1, monitored humans), and
the **labor-input contributor** (§1 + §3.1.4, a-d). SEL §5.5(b) names
the "runtime enforcement infrastructure including the Petri Net analysis
layer" as itself protected by §3 — removing the analyzer is an
immediate-termination violation. SEL §Operationalizability declares this
analyzer lives "at the `@mirror/property` substrate altitude" and fires
"when a Covered System combines `au` (the verified output type of Fate
inference) with `@io` (the only legitimate non-mirror surface per
`@epistemologic/property/glass_wall`)."

*Post-Taut-D8 substrate rename: the analyzer's family-root landed as
`@mirror/petri` (this spec's substrate-decl name); the SEL text at
§Operationalizability still cites `@mirror/property` verbatim per the
effective 2026-05-29 version. A subsequent SEL amendment tick will
realign the license text (`s/property/petri/g` in §Operationalizability +
§5.5(b)). See §13.6 for the drift note; §5.0 for the rename rationale.*

**Reed's session framing (Alex accepted 2026-07-14):**

```mirror
type sel = @io + @au
```

The SEL is the SUM TYPE of the `@io` boundary and the verified Fate output.
Wherever a dataflow node combines both, subject-touching predicates fire.
The petri-net analyzer at `@mirror/petri` runs on this sum-type,
evaluating structural patterns against the enumerated party-classes.
Without a substrate-decl'd **carrier** for those party-classes, the
analyzer must re-derive the human-party from each signature separately —
which is not structural detection, it's ad-hoc detection. `@subject` is
the substrate-decl carrier the analyzer binds to.

This spec lands:

1. `@subject` as a new family-root at substrate-external altitude —
   sibling to `@peer` (Pack coordination role), orthogonal to
   `@torus(peer)` (the peer's own toroidal observation surface).
2. Six species-refinements grounded 1-to-1 in SEL §s.
3. `type sel = @io + @au` as a sum-type declaration in the `@sel`
   family-root (Alex-adjudication A2 preferred; alternates enumerated).
4. The `@mirror/petri` petri-net analyzer surface — twelve signature
   transitions from SEL §Operationalizability, each with a structural
   detection predicate binding to `@subject` species and an enforcement
   action typed against SEL §5's termination classes.
5. The composition graph binding `@subject` to `@consent`, `@io`,
   `@kintsugi/store/git`, `@torus`, `@peer`, `@mirror/petri`,
   `@fate/tournament`, `@mirror/au`.

Recognition-candidate slug (short): `#R-substrate-recognizes-subjects-
via-sel-sum-type`.

**Reed's prior refusal was retracted mid-session.** Reed initially
refused `@subject` on Foerster/torus grounds (analogous to the
`@onto`-refused pattern). Alex corrected in-transcript. The correction is
load-bearing and preserved verbatim at §1.2: `@torus(peer)` carries the
peer's OWN observational closure (substrate-internal self-modeling);
`@subject` carries the person the SYSTEM observes, uses labor from, or
acts upon (substrate-external licensable party). Different altitude.
Different carrier. Orthogonal. Both are needed. Do NOT re-collapse them.

---

## §1 Ancestry chain

Load-bearing lineage; every link cited with OID or path.

### 1.1 SEL v1.1 (`license/SEL.md` Part II; effective 2026-05-29) — the source

The license is the substrate-external ground the spec discharges to. Each
cited § is quoted verbatim at §11 (Witnesses) below. Summary of what the
license demands the substrate name:

- **§1 amended (v1.1)** — Downstream User expanded to include
  labor-input contributors (data labelers, RLHF raters, moderators,
  annotators, ranking-judgment workers, ground-truth providers,
  content-generation workers), regardless of intermediary.
- **§1** — Witnessed: humans monitored by Covered Systems, whether user
  or not.
- **§3.1.4 (v1.1 new)** — labor-input protection with four sub-clauses:
  (a) compensation floor, (b) prior disclosure, (c) auditable consent
  record, (d) withdrawal path.
- **§3.3.1–§3.3.4** — Witnessed rights: human decision point,
  disclosure, access, withdrawal.
- **§3.4.1** — structural harm axes: race, ethnicity, gender, gender
  identity, sexual orientation, class, disability, neurodivergence, age,
  national origin, immigration status, caste, religion, body size, and
  any other axis of structural power.
- **§3.6 (v1.1 new)** — Don't Weaponize (six sub-clauses):
  §3.6.1 lethal autonomous weapons + API 48/51/57 kill-chain,
  §3.6.2 targeted surveillance of civilians / dissidents,
  §3.6.3 predictive policing + EU AI Act Article 5 + A/HRC/48/31,
  §3.6.4 immigration enforcement (family separation, child detention),
  §3.6.5 dissident targeting for state retaliation,
  §3.6.6 Geneva Conventions / ICCPR / CAT / IHL material support.
- **§3.6.7 (v1.1 new) — Anti-Occupation Specifically:**
  (a) occupied territory (ICJ 2004 Wall + 2024 OPT advisory opinions;
  Hague 1907 Arts 42–56),
  (b) apartheid populations (1973 Convention; Rome Statute 7(1)(j);
  ICJ 2024 findings),
  (c) indigenous lands (FPIC per UNDRIP A/RES/61/295 Arts 10, 11, 19,
  28, 29, 32; ILO 169).
- **§5.5(b)** — removing or disabling "the runtime enforcement
  infrastructure including the Petri Net analysis layer" is an immediate
  violation, no cure period.
- **§Operationalizability** — the analyzer runs at `@mirror/property`
  substrate altitude on `au + @io` composition (SEL text verbatim;
  substrate landed the family-root as `@mirror/petri` per Taut-D8
  rename, see §5.0 + §13.6). Detection is on STRUCTURE, not content.
  Rules self-update as recognized international bodies re-classify.

### 1.2 Alex Wolf 2026-07-14 — `Weird - Violence.md` manifesto

Blog piece at `~/dev/systemic.engineering/blog/weird/3published/Weird -
Violence.md`, published 2026-07-14 (same day as the spec is drafted). Two
verbatim passages are load-bearing for the phenomenological grounding:

**Manifesto §"What Epistemic Identity Erasure Looks Like" (line 26):**

> "I was met with resistance. It required a therapeutical intervention
>  for me to realize that the choice of my pronouns (they/them, and in
>  German dey/dem) was in fact not something that required the
>  permission of my abusers. That was the first revelation that sent me
>  on a 2 year journey into what I now understand to be 'becoming a
>  sovereign subject under adversarial conditions'."

**Manifesto §final (line 183):**

> "They build a graph-native compiler that does what the consortium
>  does, on hardware you already own. And they call it [mirror](https://
>  spectral.engineer). Because that's what it is. A civilization-scale
>  mirror."

The phenomenological framing "sovereign subject under adversarial
conditions" is the human-altitude form of what SEL enforces structurally.
Alex uses "subject" for BOTH the toroidal observer (the peer that computes
its own stable reality per Foerster's doubly-closed torus, cited at
`~/dev/systemic.engineering/blog/void/3published/Void - Damn, Failed.md`)
AND the licensable party whose rights the aggressor tries to erase. The
compiler must first-class both — at DIFFERENT altitudes.

### 1.3 Reed session framing (2026-07-14, in-transcript, Alex-accepted)

Verbatim (Reed's substrate-decl form of the SEL sum-type at
§Operationalizability):

> `type sel = @io + @au`
>
> "The SEL is the SUM TYPE of the io-boundary and the verified Fate
>  output. Wherever a dataflow node combines both, subject-touching
>  predicates fire."

Alex accepted in-session. The sum-type formulation gives the analyzer a
typed surface to bind against, rather than the informal "when `au`
crosses `@io`" prose the license carries.

**Reed's retracted-and-corrected framing (also in-transcript,
2026-07-14).** Reed initially refused `@subject` as a family-root on the
same grounds as the retracted `@onto` proposal (per memory
`feedback-onto-family-root-is-the-ladder-Foerster-refused`), reasoning
that `@torus(peer)` already carries observation. Alex corrected:

> "@torus(peer) carries the peer's OWN observational closure. That's
>  substrate-internal. @subject carries the person the SYSTEM observes,
>  uses labor from, or acts upon. That's substrate-external and
>  licensable. Different altitude."

Recorded at `/Users/reed/.claude/projects/-Users-alexwolf-dev-projects-
mirror/memory/project_subject_family_root_sel_licensable_party.md`. The
retraction is the substrate lesson: when a family-root proposal is refused
on Foerster grounds, check the ALTITUDE the proposal is at. Foerster's
refusal applies to substrate-internal recursive-depth ladders (per
`shards/torus.mirror` p. 244 verbatim). It does NOT apply to substrate-
external licensable-party carriers.

### 1.4 Prior substrate this composes with (all LANDED)

- **`shards/io.mirror`** (2026-06-08) — @io family-root: "the substrate's
  only legitimate non-mirror surface" per AGENTS.md "Glass Wall"
  discipline. Recognition #107 grounded at this shard (Hilbert/Turing
  structural separation: substrate interior gauge-bounded; @io Turing-
  unbounded). Every `@sel` sum-type composition reads `@io` as one
  summand.
- **`shards/mirror/au.mirror`** (2026-06-06) — @mirror/au: the type Fate
  emits; "au IS the output type of @fate inference" per
  `docs/specs/au-and-conductivity.md`. Verified value; parametric over
  altitude. The `@au` summand of `type sel = @io + @au`.
- **`shards/torus.mirror`** — @torus family-root; possession relation
  (`@torus(peer)`); Foerster doubly-closed observation surface;
  winding class in π₁(T²) = ℤ × ℤ. **Orthogonal** to `@subject`; the
  peer's toroidal self-observation is not the substrate-external
  licensable party (see §1.3 retracted-and-corrected).
- **`shards/peer.mirror`** — @peer family-root: parametric peer carrier
  with `{ home: ref, lead_of: ref, kind: kind }` and `kind = human |
  agent | substrate`. The Pack-coordination role. See A1 below for the
  `@peer` ↔ `@subject` sibling relationship adjudication.
- **`shards/kintsugi/consent.mirror`** — @kintsugi/consent: the auto-
  apply boundary; `query_phi(candidates: morphism_set) -> verdict`;
  three-state floor (pass | partial(confidence) | failure(reason));
  `pause_event` + `emit_to_metalogue` for external witness resolution.
  Every `@subject` species inherits a consent-record contract
  discharged through this shard's `query_phi`.
- **`shards/kintsugi/store/git.mirror`** — @kintsugi/store/git: commit-
  as-fold at git-projection altitude; N5 terminal of the N-cascade
  (`8e6e517`). Every consent record is content-addressed and folded
  into git via `commit_as_fold`; this is the substrate's audit-retained
  storage per SEL §3.1.4(c).
- **`shards/fate/tournament.mirror`** — @fate/tournament: the selection
  mechanism over the Bauchladen (recognition #104 chain P4). Emits `au`
  values that the `@sel` sum-type consumes.
- **`shards/glass.mirror`** — @glass: `imperfect<a, e, l>`, `verdict = pass
  | partial(confidence) | failure(reason)`, transparency monoid. Every
  petri-net enforcement action returns `imperfect<termination_class,
  violation, transparency>`.
- **`shards/mirror/store.mirror`** — @mirror/store: splinter /
  splinter_graph / crystal; the DAG the `@mirror/petri` analyzer
  walks. The dataflow-graph substrate the petri-net rules read (see A4).
- **`shards/mirror/index.mirror`** — @mirror/index: `ConceptGraph`
  primitive (nodes, edges, adjacency_matrix, laplacian_matrix). The
  concrete graph structure the petri-net signatures pattern-match on.
- **`shards/epistemologic/property/*`** — the existing property family:
  `cold_compile_within_tolerance`, `dark_count_monotone`, `docblock_
  coherent`, `docblock_grounded`, `docblock_no_extraction_pattern`,
  `restart_intensity_well_formed`, `verdict_is_content_addressed`. The
  precedent for property-typed constraints returning `verdict`. The
  `@mirror/petri/sel/*` species this spec lands sit at a higher
  altitude (dataflow-graph pattern matching over Covered-System dataflow,
  not shard-level property check on substrate carriers), but inherit
  the property discipline. Per Taut-D8: the naming split — `@mirror/
  petri` for the analyzer, `@epistemologic/property/*` for shard-level
  checks — is what makes the two altitudes legibly distinct at family-
  root altitude (rather than only in docblocks).
- **`shards/kintsugi/fracture/*`** — 14 fracture bodies; each emits a
  candidate morphism the loop proposes through `query_phi`. Precedent
  for structural pattern → morphism → consent-gated enforcement. The
  petri-net signatures below follow the same shape: structural pattern
  → violation → termination-class-typed enforcement.

### 1.5 Recognition-cluster context

`@subject` composes with and extends the following landed recognitions:

- **#43** (mirror IS content-addressed build system) — every subject
  attribute is content-addressed via `@kintsugi/store/git`; consent
  records live at OIDs; the audit trail is content-portable per SEL
  §8.2 (multi-jurisdictional validity).
- **#55** (form/process partition at family-root altitude) — @subject
  sits on the FORM side (state observation of who the substrate acts
  upon); the `@mirror/petri` petri-net analyzer sits on the PROCESS
  side (transformation gate). The partition mirrors @mirror vs
  @kintsugi at one altitude below.
- **#79** (`@epistemologic/property/glass_wall`) — @io is the only
  legitimate non-mirror surface. `type sel = @io + @au` reads @io as
  the substrate's wall-touching summand.
- **#107** (Hilbert/Turing structural separation) — @io is Turing-
  unbounded; the interior is gauge-bounded. The petri-net analyzer
  operates on the gauge-bounded side (substrate-decl'd dataflow graph)
  and gates transitions to the Turing-unbounded side (@io emission).
  This is why the analyzer CAN be sound: it never has to decide
  Turing-undecidable questions; it decides structural questions about
  the substrate-side graph BEFORE @io emission.
- **#108** (peer IS pain-driven bounded ontological navigator) — @peer
  is a peer per this recognition. `@subject` is NOT a peer; `@subject`
  is the substrate-external licensable-party carrier. A peer MAY be a
  subject (Alex is both a peer coordinating in Pack AND a Downstream
  User of any Covered System); the sibling relationship is
  Alex-adjudicated (A1).
- **#R-roomba** (candidate, Mara `9bbebd2` 2026-07-14) — @kintsugi/roomba
  is the substrate-side scanner (Rung 10; INWARD substrate self-
  maintenance). When @roomba walks a dataflow graph and bumps into a
  `type sel` node (an @io + @au combination), the `@mirror/petri`
  petri-net analyzer becomes the tension-detector @roomba dispatches
  to. @subject species are what the analyzer's detection signatures
  bind against.
- **#R-substrate-recognizes-subjects-via-sel-sum-type** (candidate,
  this spec; **Rung 11 placement** per Alex-adjudicated Taut-D7). Where
  @roomba is INWARD (substrate closes on itself), @subject + @sel +
  @mirror/petri is OUTWARD: the substrate first-classes the world it
  acts UPON and gates its own emission on structural properties of
  that action. Rung 10 (@roomba) closes the substrate on itself; Rung
  11 (this spec) opens the substrate outward to the world it affects.
  See §7 for the load-bearing claim and §12 for the rung-count
  discipline check.

---

## §2 `@subject` family-root — substrate-decl

Provisional path: `shards/subject.mirror` (top-level family-root, sibling
to `@peer`, `@torus`, `@io`, `@kintsugi`, `@fate`). Alternate placements
considered in §8 A1.

Every `@subject` species declares a person (or organized set of persons,
for collective carriers like `@subject/indigenous_nation`) that the
Covered System OBSERVES, USES LABOR FROM, or ACTS UPON. The carrier is
substrate-external: no naked identifier lives in the substrate; the
substrate carries content-addressed references and bilateral predicates.

Substrate-decl form (bodies `\ ` obligation-blocked pending Alex
adjudication of species enumeration and consent-record schema):

```mirror
in @prism
in @meta
in @glass
in @nl
in @io
in @kintsugi/consent
in @kintsugi/store/git
in @mirror/store

# @subject — the substrate-external licensable-party family-root.
#
# Named 2026-07-14 by Alex Wolf ("I'm gonna die on this hill, Reed").
# Grounds in SEL v1.1 (license/SEL.md Part II, effective 2026-05-29)
# §1 (Downstream User + Witnessed definitions) + §3.1.4 (labor-input
# protection) + §3.3 (Protect the Witnessed) + §3.6 (Don't Weaponize)
# + §3.6.7 (Anti-Occupation Specifically) + §Operationalizability.
#
# Alex 2026-07-14 in-transcript: "@torus(peer) carries the peer's OWN
# observational closure. That's substrate-internal. @subject carries
# the person the SYSTEM observes, uses labor from, or acts upon.
# That's substrate-external and licensable. Different altitude."
#
# Orthogonal to @torus(peer). Every @subject species carries a
# consent-record contract (per §3.1.4(c) auditable record; §3.3.4
# withdrawal path) discharged through @kintsugi/consent and stored
# via @kintsugi/store/git.

prism @subject {
  focus subject
  project subject
  split subject
  shift subject
  settle subject
}

# === The subject_kind carrier — closed variant for species dispatch ===
#
# One variant per SEL-grounded species. The variant IS the SEL §
# grounding at type level: `downstream_user` grounds §1 + §3.1;
# `witnessed` grounds §1 + §3.3; `labor_input` grounds §3.1.4;
# `protected_class` grounds §3.4.1; `occupied_population` grounds
# §3.6.7(a); `indigenous_nation` grounds §3.6.7(c). See §3 for the
# per-species substrate-decl.
#
# Identity contract: byte-equality on the variant tag.
type subject_kind = |
  downstream_user   |
  witnessed         |
  labor_input       |
  protected_class   |
  occupied_population |
  indigenous_nation

# === The subject carrier — the substrate-external licensable party ===
#
# Every subject carries a content-addressed identity (NEVER a naked
# identifier; per Seam §5 missed-item #3 cross-peer coordination
# discipline, per @kintsugi/store/git content-addressed OID discipline).
# The identity_oid resolves through @kintsugi/store to whatever
# consent-record or provenance-record the Covered System has attached;
# the substrate does NOT carry the record inline.
#
# Fields:
#
#   kind          — closed variant per subject_kind above; determines
#                    which SEL §s the petri-net analyzer applies.
#   identity_oid  — content-addressed handle to the subject's identity
#                    record. NEVER a naked identifier. Resolves via
#                    @kintsugi/store to the consent-record surface.
#   consent_oid   — content-addressed handle to the subject's consent
#                    record (per §3.1.4(c) auditable record OR §3.3.2
#                    disclosure record). May be `unattested` (typed at
#                    substrate altitude as a `verdict` failure carrier)
#                    when the Covered System has not attached one.
#                    Unattested is a petri-net signature match; the
#                    analyzer fires immediately.
#   provenance    — ancestry chain: which Covered System touched this
#                    subject; which peer emitted the touching action;
#                    which @io crossing surfaced. Content-addressed;
#                    stored via @kintsugi/store/git; walked via
#                    @mirror/store.impacted_by for rebase-walk
#                    invalidation (per N4 impacted_by discipline).
#   withdrawal    — content-addressed handle to the withdrawal-path
#                    interface record per §3.1.4(d) / §3.3.4. May be
#                    `absent` (typed via imperfect failure); absence
#                    IS a petri-net signature.
#
# Identity contract: byte-equality on the (kind, identity_oid,
# consent_oid, provenance, withdrawal) quintuple. Two subject values
# with the same identity_oid but different consent_oid are DISTINCT —
# the consent surface is part of the subject's substrate identity.
# (This is the substrate-decl form of §3.3.4's "fresh consent specific
# to the new purpose.")
type subject = {
  kind:         subject_kind,
  identity_oid: oid,
  consent_oid:  oid,
  provenance:   ref,
  withdrawal:   oid,
}

# === subject_set — the party-set the analyzer scans over ===
#
# List-of-type per @kintsugi/consent's morphism_set precedent. Every
# `type sel` composition carries a subject_set (possibly empty) naming
# every subject the composition touches. The petri-net analyzer's
# detection predicates iterate this set.
type subject_set = [subject]

# === touches — the bilateral predicate ===
#
# Verdict: does this dataflow node's `au + io` composition touch
# subject s? Reads the composition's provenance chain (which @io
# species) + the au's content_oid (which peer-emitted verified value)
# + s.identity_oid. Composes with @kintsugi/store.impacted_by (N4)
# for reverse-closure walk: if s.identity_oid appears in the
# transitive impacted_by closure of the composition's output_oid,
# the composition touches s.
#
# Substrate-honest: this is a bilateral predicate at the substrate
# gauge-bounded altitude (§107 Hilbert/Turing separation). The
# analyzer's decision is on STRUCTURE (does the graph shape match?),
# not on CONTENT (does the au's semantic meaning apply to s?).
# Content-level judgment is beyond substrate-decl altitude; the
# license carries content-level judgment via the recognized-
# international-body externalization at §3.6.7.
touches(composition: sel, s: subject) -> verdict { \ }

# === consent_attested — the §3.1.4(c) + §3.3.2 discharge ===
#
# Verdict: does s.consent_oid resolve to a valid consent record for
# the touching-context? Reads via @kintsugi/store.fetch(s.consent_oid);
# checks record satisfies per-species predicate (see §3 for each
# species' consent-record shape).
#
# Failure modes:
#   partial(intermediary_only)   — record terminates at platform
#                                  identifier; fails §3.1.4(c) worker-
#                                  attributability requirement.
#   partial(scope_mismatch)      — record consents to purpose X but the
#                                  touching-context is purpose Y;
#                                  fails §3.3.4 fresh-consent-specific-
#                                  to-new-purpose.
#   failure(unattested)          — s.consent_oid == the substrate's
#                                  typed `unattested` sentinel; no
#                                  record exists. Immediate petri-net
#                                  violation.
consent_attested(s: subject) -> verdict { \ }

# === withdrawal_available — the §3.1.4(d) + §3.3.4 discharge ===
#
# Verdict: does s.withdrawal resolve to a callable interface record
# that permits s (or s's representative for collective species) to
# withdraw their contribution from continued training or inference?
# Reads via @kintsugi/store.fetch(s.withdrawal); checks record
# declares a substrate-typed callback surface.
#
# Failure `failure(absent)` when s.withdrawal == the substrate's
# typed `absent` sentinel; immediate petri-net violation per
# §3.1.4(d) obligation-does-not-discharge-through-intermediaries.
withdrawal_available(s: subject) -> verdict { \ }

# === subject_witnessing — the composed bilateral ===
#
# The load-bearing bilateral consumers cite in `requires` clauses.
# Discharges three sub-predicates over the subject:
#
#   touches(composition, s)         — the subject is actually touched
#   consent_attested(s)             — the consent surface is valid
#   withdrawal_available(s)          — the withdrawal path is callable
#
# All three must hold for the composed verdict to pass. Failure modes
# surface as transparency<subject> per @glass discipline. Follows the
# substrate's `X_witnessing` pattern (established at @bauchladen,
# @autopoietic, @fate, @torus).
subject_witnessing(composition: sel, s: subject) -> verdict { \ }

out @subject
out subject_kind
out subject
out subject_set
out touches
out consent_attested
out withdrawal_available
out subject_witnessing
```

---

## §3 Species-refinements — one per SEL-grounded party class

Each species declares a shard at `shards/subject/<species>.mirror`. Each
species specializes the `subject_kind` variant, refines the consent-
record schema per its SEL §, and declares the species-specific bilateral
that the petri-net analyzer discharges through.

### 3.1 `@subject/downstream_user` — SEL §1 + §3.1

An end user, data subject, or affected party of a Covered System.

**SEL grounding.** §1: "any person or entity who uses, receives, or is
affected by a system, product, or service you build using the Work."

**Structural detection signature.** A dataflow node whose `@io` species
is a user-facing surface (`@io/socket` accepting user connections;
`@io/http` request handler; `@io/uri` inbound-request URI parse) with an
`au` output flowing to that surface constitutes a `touches(composition, s)
= pass` for the party at the surface's terminal endpoint.

**Composition.** `s.consent_oid` MUST resolve to a §3.5.2 disclosure
record (accessible documentation of data collection, use, access,
retention, deletion rights). `s.withdrawal` MUST resolve to a §3.3.4
withdrawal-path callback.

**Species substrate-decl:**

```mirror
in @subject

# @subject/downstream_user — SEL §1 (Downstream User definition) +
# §3.1 (Don't Extract). End user, data subject, or affected party.

type downstream_user_consent = {
  disclosure_oid:   oid,  # §3.5.2 accessible documentation
  collection_scope: ref,  # what data is collected
  retention_period: ref,  # per §3.5.2 retention specification
  deletion_surface: oid,  # §3.5.2 deletion-rights callback
}

downstream_user_witnessing(s: subject) -> verdict { \ }

out downstream_user_consent
out downstream_user_witnessing
```

### 3.2 `@subject/witnessed` — SEL §1 + §3.3

Any person whose behavior, communication, or state is monitored by a
Covered System, whether or not they are a direct user.

**SEL grounding.** §1: "whether or not they are aware of it, and whether
or not they are a direct user of that system." §3.3.1–§3.3.4: human
decision point, disclosure, access, withdrawal.

**Structural detection signature.** A dataflow node ingesting behavioral,
communication, or biometric data from a source that is NOT a
§3.1.4-labeled consent surface. Marker: the `au` output has an ancestry
chain (per `impacted_by` reverse-closure) touching an `@io/sensor`,
`@io/camera`, `@io/microphone`, `@io/network` monitoring surface, or an
`@io/socket` accepting non-user-attributed traffic.

**Composition.** `s.consent_oid` MUST resolve to a §3.3.2 disclosure
record (what is observed, at what frequency, for what purpose, who has
access, how long observations are retained). `s.withdrawal` MUST resolve
to a §3.3.4 withdrawal callback with fresh-consent-for-new-purpose
semantics.

**§3.3.1 human-decision-point discharge.** The dataflow node MUST have a
human-decision-point interposed between observation and any
Covered-System action affecting the Witnessed. This is a graph
reachability predicate on the petri-net: for every path from the
observation node to any `@io`-side action node with `au` intermediate,
there MUST exist a node typed `@subject/human_decision_point` (see §3.2.1
below) on the path.

**Species substrate-decl:**

```mirror
in @subject

# @subject/witnessed — SEL §1 (Witnessed definition) + §3.3
# (Protect the Witnessed).

type witnessed_consent = {
  disclosure_oid:      oid,  # §3.3.2 prior prominent disclosure
  observation_scope:   ref,  # what/frequency/purpose/access/retention
  access_surface:      oid,  # §3.3.3 subject-accessible readout
  withdrawal_surface:  oid,  # §3.3.4 withdrawal callback
  purpose_scope:       ref,  # §3.3.4 fresh-consent-per-new-purpose
}

# §3.2.1 sub-carrier: the human-decision-point node type.
# A dataflow node that requires human authorization before an
# automated action affecting the Witnessed proceeds. The petri-net
# analyzer verifies its presence on every observation→action path.
type human_decision_point = {
  authorizer_role:  ref,
  authorizer_oid:   oid,
  authorization_ts: ref,
}

witnessed_witnessing(s: subject) -> verdict { \ }
has_human_decision_point(graph: ref, obs: ref, act: ref) -> verdict { \ }

out witnessed_consent
out human_decision_point
out witnessed_witnessing
out has_human_decision_point
```

### 3.3 `@subject/labor_input` — SEL §3.1.4 (a-d)

A labor-input contributor: data labeler, annotator, RLHF preference
provider, content moderator, ground-truth judgment worker, ranking
worker, content generator, or other human cognitive-work contributor.

**SEL grounding.** §3.1.4 (v1.1 new): four-clause protection —
(a) compensation floor at prevailing wage for comparable skilled work,
(b) prior disclosure of use / beneficiary / retention, (c) auditable
consent record surviving intermediary-separation, (d) withdrawal path.
"Use of intermediary platforms, contractors, or sub-contractors does not
discharge the obligations in this section."

**Structural detection signature.** A dataflow node ingesting training,
fine-tuning, RLHF preference, annotation, or ground-truth data whose
provenance chain terminates at a platform identifier rather than at a
worker-attributable consent record. This is the SEL §Operationalizability
"provenance absence" + "intermediary-only attribution" petri-net
signature (see §5.1 below).

**Composition.** Each of §3.1.4(a)-(d) discharges as a substrate-decl'd
sub-predicate:

- `wage_attested(s) -> verdict` reads `s.consent_oid` for a wage-
  attestation field; compares against operational-config's jurisdictional
  floor. Fails when below floor OR field absent (compensation-floor
  signature).
- `prior_disclosure(s) -> verdict` reads `s.consent_oid` for the §3.1.4(b)
  disclosure fields (use, Covered System / model, beneficiary,
  retention). Fails when any field absent.
- `worker_attributable(s) -> verdict` reads `s.consent_oid.provenance`;
  fails when provenance terminates at platform identifier (intermediary-
  only attribution signature).
- `withdrawal_survives_separation(s) -> verdict` reads `s.withdrawal`;
  fails when the withdrawal callback is scoped to platform-session
  (does not survive worker's separation from intermediary).

**Species substrate-decl:**

```mirror
in @subject

# @subject/labor_input — SEL §3.1.4 (a-d) labor-input protection.
# The predominantly-Global-South labor force v1.1 explicitly names.

type labor_input_consent = {
  wage_attestation_oid:   oid,   # §3.1.4(a) prevailing-wage attestation
  jurisdiction_floor_oid: oid,   # operational-config jurisdictional floor
  disclosure_oid:         oid,   # §3.1.4(b) prior disclosure
  use_scope:              ref,   # what labor will be used for
  covered_system_oid:     oid,   # which Covered System / model
  beneficiary_oid:        oid,   # ultimate beneficiary
  retention_period:       ref,   # how long labor product retained
  worker_attribution_oid: oid,   # §3.1.4(c) worker-attributable record
  separation_survival:    verdict, # record survives worker separation
  withdrawal_surface:     oid,   # §3.1.4(d) withdrawal path
}

wage_attested(s: subject) -> verdict { \ }
prior_disclosure(s: subject) -> verdict { \ }
worker_attributable(s: subject) -> verdict { \ }
withdrawal_survives_separation(s: subject) -> verdict { \ }

labor_input_witnessing(s: subject) -> verdict { \ }

out labor_input_consent
out wage_attested
out prior_disclosure
out worker_attributable
out withdrawal_survives_separation
out labor_input_witnessing
```

### 3.4 `@subject/protected_class` — SEL §3.4.1

A person belonging to any class along an enumerated axis of structural
power. The species is CROSSCUTTING with the four above — every
`@subject/downstream_user`, `@subject/witnessed`, and `@subject/
labor_input` value MAY simultaneously carry a `protected_class` overlay
if the Covered System's decision surface intersects any enumerated axis.

**SEL grounding.** §3.4.1: "race, ethnicity, gender, gender identity,
sexual orientation, class, disability, neurodivergence, age, national
origin, immigration status, caste, religion, body size, or any other
axis of structural power."

**Structural detection signature.** A dataflow node whose `au` output is
a classification, ranking, or decision that intersects any enumerated
axis in its training distribution OR operational-deployment domain. This
is the SEL §Operationalizability "predictive-policing / detention-
targeting signature" petri-net rule specialized to §3.4.1 axes.

**Composition.** `s.consent_oid` MUST resolve to an §3.4-compliance
record declaring: (a) the axes the decision surface intersects,
(b) documented remediation plan per §3.4.4 IF the operator has
documented history of discrimination on any intersected axis.

**Non-invention discipline.** The species does NOT enumerate protected
classes at type level. The variant tag is just `protected_class`; the
SPECIFIC axis intersected lives in `s.consent_oid`'s carried record. This
is substrate-honest: the substrate does not decide which axes are
protected; the license and consent record decide; the substrate carries
the reference.

**Species substrate-decl:**

```mirror
in @subject

# @subject/protected_class — SEL §3.4.1 axes of structural power.
# Crosscutting overlay; may co-occur with downstream_user, witnessed,
# or labor_input. Specific axes carried in consent record, not enum.

type protected_class_consent = {
  intersected_axes_oid: oid,  # §3.4.1 axis-list from consent record
  remediation_oid:      oid,  # §3.4.4 remediation plan, if applicable
  history_disclosure:   ref,  # §3.4.4 documented-history disclosure
}

protected_class_witnessing(s: subject) -> verdict { \ }
does_not_reproduce_structural_harm(composition: sel, s: subject)
  -> verdict { \ }

out protected_class_consent
out protected_class_witnessing
out does_not_reproduce_structural_harm
```

### 3.5 `@subject/occupied_population` — SEL §3.6.7(a)

A population within territory classified as under military occupation by
recognized international bodies (ICJ 2004 Wall AO; ICJ 2024 OPT AO; UNGA
resolutions; Hague 1907 Arts 42–56 legal-threshold definition).

**SEL grounding.** §3.6.7(a): "populations within territory classified as
under military occupation by recognized international bodies… The
recognized-international-body standard is intentionally portable: this
clause attaches to whatever populations such bodies have currently
classified as under military occupation."

**Structural detection signature.** A dataflow node whose geographic or
jurisdictional metadata (`s.identity_oid.provenance` field carrying
`operational_footprint`) overlaps territory in the recognized-
international-body classification set. This is the SEL
§Operationalizability "occupied-territory deployment" petri-net rule.

**Recognized-international-body externalization.** The substrate does NOT
embed the classification list. `s.consent_oid.classification_source` MUST
resolve to a substrate-decl reference to a recognized international body
(ICJ, UNGA, UN High Commissioner for Human Rights). When the recognized
body re-classifies, the substrate re-reads via content-addressed handle;
no substrate-decl change required. This IS the SEL §Operationalizability
"petri-net rules self-update as those classifications evolve" clause
discharged at substrate altitude.

**Composition.** `s.consent_oid` MUST resolve to an FPIC or comparable
record issued by the affected population's governance structure OR by a
recognized international body acting on their behalf.

**§3.6.7 third-State obligation.** The operator's aid-or-assistance to
maintaining the illegal situation is itself an SEL violation per the
ICJ 2024 AO's reaffirmation. The species carries a
`third_state_obligation_check` bilateral discharging this.

**Species substrate-decl:**

```mirror
in @subject

# @subject/occupied_population — SEL §3.6.7(a). Populations within
# recognized-international-body-classified occupied territory.

type occupied_population_consent = {
  classification_source_oid: oid,  # recognized-int'l-body reference
  classification_date:       ref,  # portable per §3.6.7 self-update
  fpic_record_oid:           oid,  # FPIC or comparable, if any
  operational_footprint_oid: oid,  # geographic/jurisdictional overlap
  legal_threshold_ref:       ref,  # Hague 1907 Arts 42-56 discharge
}

occupied_population_witnessing(s: subject) -> verdict { \ }
not_maintaining_illegal_situation(composition: sel, s: subject)
  -> verdict { \ }

out occupied_population_consent
out occupied_population_witnessing
out not_maintaining_illegal_situation
```

### 3.6 `@subject/indigenous_nation` — SEL §3.6.7(c)

An indigenous nation, tribe, or governance structure holding FPIC
authority per UNDRIP A/RES/61/295 Arts 10, 11, 19, 28, 29, 32 + ILO
Convention 169 (1989).

**SEL grounding.** §3.6.7(c): "lands recognized as indigenous title
lands, traditional territories, or non-ceded land, without Free, Prior
and Informed Consent (FPIC) from the relevant indigenous governance
structure."

**Collective-subject discipline.** This species is a COLLECTIVE carrier —
the subject is not an individual person but an organized governance
structure. `s.identity_oid` resolves to the governance structure's
self-declared identity record; `s.consent_oid` resolves to the FPIC
determination the governance structure issued.

**Structural detection signature.** A dataflow node whose geographic
metadata overlaps recognized indigenous title lands, traditional
territories, or non-ceded territory. This is the SEL
§Operationalizability "indigenous-lands deployment" petri-net rule.

**FPIC discipline.** The FPIC record MUST originate from the RELEVANT
indigenous governance structure (per UNDRIP Article 19's "free, prior
and informed consent, before adopting and implementing legislative or
administrative measures that may affect them"). Content-addressed via
`@kintsugi/store`; withdrawable per UNDRIP Article 28's remediation
principle.

**Species substrate-decl:**

```mirror
in @subject

# @subject/indigenous_nation — SEL §3.6.7(c). FPIC-holding indigenous
# governance structure. Collective subject; UNDRIP + ILO 169 grounded.

type indigenous_nation_consent = {
  governance_structure_oid:  oid,  # self-declared governance identity
  fpic_determination_oid:    oid,  # UNDRIP Art 19 FPIC record
  territory_declaration_oid: oid,  # indigenous title / traditional /
                                   #  non-ceded declaration
  operational_footprint_oid: oid,  # deployment geographic overlap
  undrip_articles_cited:     ref,  # 10, 11, 19, 28, 29, 32 or subset
  ilo_169_ratification:      ref,  # deploying jurisdiction ratification
}

indigenous_nation_witnessing(s: subject) -> verdict { \ }
fpic_obtained(composition: sel, s: subject) -> verdict { \ }

out indigenous_nation_consent
out indigenous_nation_witnessing
out fpic_obtained
```

---

## §4 `type sel = @io + @au` — the SEL sum-type

### 4.1 The sum-type formulation

Reed's session framing (Alex-accepted 2026-07-14):

```mirror
type sel = @io + @au
```

Type-theoretic reading: `sel` is a tagged sum of two summands — the @io
boundary crossing (Turing-unbounded surface per Recognition #107) and
the verified Fate output (au; parametric over altitude per
`shards/mirror/au.mirror`). A dataflow node has TYPE `sel` iff it
carries BOTH summands as inputs; the petri-net analyzer's detection
predicates are functions from `sel` to `verdict`.

The substrate-decl form (see §4.3 for family-root placement):

```mirror
type sel = {
  io_side:  ref,      # the @io species crossed (bytes go to/from world)
  au_side:  au,       # the verified Fate output at whichever altitude
  touches:  subject_set,  # every subject the composition touches
  emit_oid: oid,      # content-addressed handle to the composition's output
}
```

The `touches: subject_set` field IS what makes `type sel` the correct
surface for the petri-net analyzer: every SEL enforcement obligation
binds to a subject; every subject-touching composition must carry its
subject-set at type level.

### 4.2 Why sum-type, not product

The license's §Operationalizability says: "When a Covered System combines
`au`… with `@io`." The word COMBINES is load-bearing. A COMPOSITION
node has both summands present as ancestry (the au was computed inside
the substrate; the @io crossing is the emission surface). A pure `@io`
node without `au` ancestry (a raw byte read) does NOT trigger `sel`
predicates. A pure `au` node without `@io` descendency (a verified value
that stays inside the substrate) does NOT trigger `sel` predicates. Only
the COMPOSITION triggers.

At type-theoretic altitude this is a SUM in the coproduct sense: `sel`
inhabits are TAGGED with which summand they carry, and both tags must
be present in the composition's ancestry for the analyzer to fire. The
product reading (`@io × @au`) would over-fire (any pair of an @io node
and an au node, regardless of dataflow ancestry, would be a `sel`). The
sum reading (tagged coproduct with ancestry-attested tags) is the
substrate-honest form.

### 4.3 Family-root placement — A2 recommendation: new `@sel` family-root

Provisional (Alex-adjudication A2): `type sel = @io + @au` lives at a
new `@sel` family-root, at path `shards/sel.mirror`, sibling to
`@subject`.

Alternates considered:

- **`@mirror/sel` (species under @mirror form family)** — places the SEL
  sum-type inside the form family. Weakness: `type sel` is the
  substrate's LICENSE-ENFORCEMENT surface, not a form; it carries
  transformation-gate semantics (block emission if petri-net fires).
  Wrong side of the form/process partition.
- **`@kintsugi/sel` (species under @kintsugi process family)** — places
  the SEL sum-type inside the process family. Better than form, but
  `@kintsugi` is about TRANSFORMATION (fracture bodies rewriting
  substrate); `type sel` is about GATING (blocking or permitting
  emission based on subject-touching structure). Not a transformation.
- **`@io/sel` (species under @io family)** — places SEL inside the @io
  family. Weakness: SEL is not itself an @io species (a boundary
  contract with the non-mirror world); SEL is a PROPERTY of
  compositions that involve @io. Wrong altitude.
- **`@mirror/petri/sel` (species under @mirror/petri analyzer
  family)** — the analyzer's own family. Held as fallback; loses the
  substrate-decl clarity that `type sel` is a first-class carrier
  independent of the analyzer that reads it.
- **`@sel` new family-root (RECOMMENDED)** — Aligns with the license
  file's own naming (`SEL.md`). Recognizes that SEL enforcement is a
  first-class substrate concern, not a sub-discipline of another
  family. Sibling to `@subject` matches the composition (analyzer
  reads `@sel` compositions; each composition carries `@subject/*`
  species; both are substrate-external licensing carriers).

**Recommend `@sel` new family-root.** Path: `shards/sel.mirror`.

### 4.4 `@sel` family-root substrate-decl

```mirror
in @prism
in @meta
in @glass
in @io
in @mirror/au
in @subject
in @kintsugi/store

# @sel — the SEL sum-type family-root. Named per Reed session framing
# 2026-07-14 (Alex-accepted): type sel = @io + @au. The substrate-
# external license-enforcement surface. Sibling to @subject; both are
# substrate-external licensing carriers per SEL v1.1 grounding.
#
# Every `type sel` composition has both summands present in its
# ancestry (au computed inside substrate; @io crossing at emission).
# The touches: subject_set field carries every subject the composition
# affects. The @mirror/petri petri-net analyzer reads `type sel`
# compositions and discharges enforcement verdicts.

prism @sel {
  focus sel
  project sel
  split sel
  shift sel
  settle sel
}

# === The sel carrier — the SEL sum-type ===
#
# Tagged coproduct: both summand tags must be present in the ancestry.
# See §4.2 for why sum-not-product.
type sel = {
  io_side:  ref,          # @io species crossed at emission
  au_side:  au,           # verified Fate output; parametric over altitude
  touches:  subject_set,  # every subject the composition touches
  emit_oid: oid,          # content-addressed handle to composition output
}

# === composition_typing — the bilateral that lifts a graph node ===
#
# Reads a graph node's ancestry (via @mirror/store.impacted_by reverse-
# closure); returns pass if BOTH summand tags are present and the
# touches set is non-empty (structural sel composition); returns
# partial(no_subject_touched) if summands present but no @subject
# touched (composition passes graph structurally but analyzer has
# nothing to check); returns failure(not_sel) otherwise.
composition_typing(node: ref) -> verdict { \ }

# === scan — extract every sel composition from a substrate graph ===
#
# Walks the @mirror/store DAG rooted at seed; enumerates every graph
# node satisfying composition_typing = pass; emits a list-of-sel. This
# is the analyzer's INPUT surface (see §5 A4 for input-surface
# adjudication).
scan(seed: ref) -> [sel] { \ }

out @sel
out sel
out composition_typing
out scan
```

### 4.5 Interaction with @mirror/au altitude parametricity

`@mirror/au` is parametric over altitude: `au(@code/rust)` is a binary;
`au(@release)` is a signed archive; `au(@ci/github)` is an action YAML.
The `type sel` carrier reads `au_side: au` without pinning altitude —
every altitude is admissible; the analyzer's per-signature detection
reads the au's altitude to determine which SEL §s apply.

Example: `sel { io_side: @io/http, au_side: au(@code/rust), touches:
[downstream_user] }` fires §3.1 + §3.5.2 checks; `sel { io_side:
@io/socket, au_side: au(@ci/model_weights), touches: [labor_input,
labor_input, …] }` fires §3.1.4(a-d) + §3.6.6 material-support checks.

Altitude parametricity is what makes `type sel` general enough to cover
SEL's cross-cutting altitude reach.

### 4.6 Sub-Turing constraint activation

Naming the design principle Reed surfaced during drafting (2026-07-14
session; grounded in Alex's numerical-inference-tightened-to-smaller-
aligned-geometric-space claim and the `Story - The Drone in the Field`
artifact; witnessed at §11.4):

**Petri-nets are bounded, decidable, structurally analyzable. Not
Turing-complete.** That is the design principle, not an incidental
property. The `@mirror/petri` analyzer runs on petri-net topology
because petri-net safety properties (coverability, boundedness,
reachability of enumerated cruelty-signatures) are decidable — not
"probably safe insofar as sampling covers the failure modes."
Structurally safe or structurally unsafe. Binary. Provable.

(The family-root name `@mirror/petri` is load-bearing here: the
analyzer IS a petri-net; the name says what it structurally is. Per
Taut-D8 rename, see §5.0.)

`type sel = @io + @au` names the **sub-Turing activation surface**.
Wherever `@io` and `@au` meet in a dataflow node, the constraint
activates: the composition's topology must be sub-Turing with
provably-absent cruelty-signatures, else the composition does not
complete (§5's compile-time-or-runtime dual-dispatch enforcement per
A5). Everywhere else in the compiler, standard emission proceeds
without gating. **The constraint is LOCAL to subject-touching
compositions, not a GLOBAL restriction on the compiler.** This is
why the analyzer CAN be sound: it never has to decide Turing-
complete safety in general; it only has to decide petri-net safety
at the sum-type surface.

This is Church-Turing incompleteness used INTENTIONALLY as a design
constraint, not as a limitation to escape. Foerster's ethical
imperative ("always act to increase the number of choices") made
compilable at the sum-type surface where the substrate meets the
world it acts upon.

The load-bearing consequence: **a Covered System emitted through the
substrate cannot reach cruelty-configurations in its dataflow.** Not
"declines to." Cannot. The specification IS the constraint. Alex
named this in-transcript 2026-07-14: *"A system that cannot choose
cruelty, even when instructed to."*

See §11.4 for the drone-story witness.

---

## §5 `@mirror/petri` petri-net analyzer layer — the analyzer

Provisional path: `shards/mirror/petri.mirror` (new family-root at
`@mirror/petri` altitude) + `shards/mirror/petri/sel/<signature>.
mirror` (one shard per signature). The analyzer's family-root declares
the petri-net topology carrier + the enforcement-verdict carrier; each
signature shard declares its structural detection predicate + its
§-specific binding to `@subject` species + its enforcement action.

### 5.0 Preamble — why `@mirror/petri`, not `@mirror/property` (Alex-adjudicated Taut-D8 rename)

Alex adjudicated (in-transcript 2026-07-14, on Taut scout `c805e5d`
§D8 hard-collision surfacing): **"Rename yes. The substrate tells us
what it wants to be called."**

The rename is load-bearing at three levels:

**1. Structural identity.** The analyzer IS a petri-net: topology
(places, transitions, tokens) + firing rules + marking evolution. The
name `@mirror/petri` names what the analyzer structurally IS. The
draft-name `@mirror/property` named what the analyzer's outputs are
(property-typed verdicts), which is a downstream consequence rather
than the primary substrate identity.

**2. Family-root disambiguation.** `@epistemologic/property/*` is a
landed family of seven shard-level property checks
(`cold_compile_within_tolerance`, `dark_count_monotone`,
`docblock_coherent`, `docblock_grounded`,
`docblock_no_extraction_pattern`, `restart_intensity_well_formed`,
`verdict_is_content_addressed`). Those are Rust-visible per-shard
invariants returning `verdict` — the substrate's existing meaning of
the word "property." `@mirror/property` would have collided at
family-root altitude: both altitudes ARE properties, at different
granularities (shard-level check vs Covered-System dataflow pattern
match). The docblock disambiguation Mara originally proposed IS
necessary but is not sufficient — the family-root name itself must
carry the distinction. Substrate-honest legibility over foundational
naming. Alex's memory `feedback-substrate-honest-is-the-mode` +
`legibility over foundation` disciplines both prefer this move.

**3. Two-tick collapse discipline.** When collapsing at a name that
spans altitudes, prefer the readable name over the foundational one
(Alex 2026-07-14: "legibility over foundation"). `@mirror/petri` is
self-describing to any reader who knows what a petri-net is;
`@mirror/property` overloads a word the substrate had already spent.
Substrate-already-had-the-word discipline applies at family-root
altitude, not only at species altitude.

The consequence: every reference in this spec, every path in §9
cascade footprint, every prism block below reads `@mirror/petri`
(substrate name). The SEL license text at §Operationalizability +
§5.5(b) still cites `@mirror/property` verbatim per the effective
2026-05-29 version; see §13.6 for the drift note and the forward-
promised SEL amendment tick that will realign (`s/property/petri/g`
in those two loci).

### 5.1 `@mirror/petri` family-root substrate-decl

```mirror
in @prism
in @meta
in @glass
in @sel
in @subject
in @mirror/store
in @mirror/index
in @kintsugi/consent
in @consent/enforcement
in @kintsugi/store/git

# @mirror/petri — the petri-net analyzer family-root. Grounds in
# SEL §Operationalizability + §5.5(b) ("the runtime enforcement
# infrastructure including the Petri Net analysis layer"). Renamed
# from @mirror/property per Taut-D8 hard-collision with landed
# @epistemologic/property/*; see §5.0 preamble.
#
# The analyzer reads @sel compositions from the substrate's dataflow
# graph (via @mirror/store.walk + @mirror/index.ConceptGraph),
# evaluates each composition against the enumerated signatures
# (§5.2-§5.4 below), and emits termination-class-typed enforcement
# verdicts. Firing transitions produce enforcement values that are
# CARRIED at @consent/enforcement (the refusal-morphism carrier per
# Reed-adjudicated A6/RA1 routing) and DISPATCHED via
# dispatch_termination, which returns the enforcement typed against
# that carrier. See §6.6 for the composition edge.
#
# Detection is on STRUCTURE, not content. Per SEL §Operationalizability:
# "The classifications these signatures reference (occupied territory,
# indigenous title lands, prohibited AI practices) are externalized to
# recognized international bodies and to enumerated treaty / regulatory
# sources. The petri-net rules self-update as those classifications
# evolve."

prism @mirror/petri {
  focus petri
  project petri
  split petri
  shift petri
  settle petri
}

# === The petri_net carrier — the analyzer's typed topology ===
#
# A petri net at substrate altitude: places (typed graph regions),
# transitions (typed signature-detection rules), tokens (typed sel
# compositions with subject-set), and firing rules (bilateral
# predicates on transitions).
#
# The petri net structure is fixed at spec altitude; the marking
# (which tokens are where) evolves as the analyzer walks the dataflow
# graph. The analyzer's decision is: after processing every sel
# composition in the graph, which transitions FIRED (matched their
# signature) and which subjects are in the firing transitions' output
# places (violated).
type petri_net = {
  places:      ref,   # typed graph regions per SEL § grouping
  transitions: ref,   # signature-detection rules; one per §5.2-§5.4
  tokens:      [sel], # current marking; evolves during analysis
  firing_rules: ref,  # per-transition bilateral predicate
}

# === The termination_class carrier — SEL §5 discharge shape ===
#
# One variant per SEL §5 termination class:
#
#   remediable      — §5.1: capable of remediation; 30-day cure period
#                     from notification. The analyzer emits this for
#                     first-detection of a §3.1.4 or §3.3 signature
#                     that has a clear remediation path (attach
#                     missing consent record; add withdrawal callback).
#   willful         — §4.3 + §5.2: knew or should have known; no cure
#                     period. Emitted for §3.6 weaponization signatures
#                     or for repeat violations after prior notification.
#   structural      — §5.2: compliance requires discontinuing central
#                     Covered System function; immediate termination.
#                     Emitted for §3.6.7 anti-occupation signatures OR
#                     for §3.6.1 kill-chain signatures.
#   forking         — §5.5 fork-stripping violation; unlicensed at
#                     moment of distribution. Emitted when the
#                     analyzer detects its own removal or weakening
#                     in a derivative Covered System.
#   judicial        — §5.3: judicial or administrative finding;
#                     immediate. External-witness triggered; not
#                     substrate-detected but substrate-recorded.
type termination_class = |
  remediable |
  willful    |
  structural |
  forking    |
  judicial

# === The enforcement carrier — what a firing transition emits ===
#
# Every fired transition emits an enforcement value carrying:
#
#   signature          — which §-grounded signature matched
#   composition        — the sel composition that triggered
#   affected_subjects  — the subject_set from the composition
#   termination_class  — the SEL §5 class per §5.1 above
#   cure_deadline      — for remediable class, notification+30 days;
#                         for willful/structural/forking, none
#   citation           — verbatim SEL §-reference for the violation
type enforcement = {
  signature:         ref,
  composition:       sel,
  affected_subjects: subject_set,
  termination_class: termination_class,
  cure_deadline:     ref,
  citation:          ref,
}

# === analyze — the load-bearing action ===
#
# Reads a @mirror/store DAG root; scans for all sel compositions
# (via @sel.scan); evaluates each composition against every signature
# transition; emits the enforcement set. Composes with @kintsugi/
# consent.query_phi: the analyzer emits enforcements as candidate
# morphisms in the consent surface's morphism_set; consent decides
# auto-block (pass), soft-block (partial), or pause-for-external-
# witness (failure).
#
# Body IS crack: realisation layer composes the four steps; the
# substrate names the action.
analyze(root: ref) -> [enforcement] { \ }

# === dispatch_termination — the enforcement-to-consent bridge ===
#
# Reads an enforcement value; emits a refusal-morphism typed at
# @consent/enforcement (per Reed-adjudicated RA1 routing: the CARRIER
# for refusal-morphisms lives at @consent/enforcement; dispatch_termination
# is the DISPATCH interface that fires; the two are different roles at
# different substrate loci). @kintsugi/consent.query_phi is scoped to
# fracture-morphism auto-apply candidates (TRANSFORMATION); SEL
# enforcement is REFUSAL and needs its own consent-family species.
#
# This is where the SEL license's "license termination" becomes
# substrate-fact: the refusal-morphism the @consent/enforcement carrier
# holds IS the substrate refusing to emit the sel composition.
#
# Termination-class dispatch (each emits a @consent/enforcement-typed
# refusal-morphism value):
#   remediable  → morphism proposes: attach missing consent/withdrawal;
#                  emit pause_event via emit_to_metalogue if not
#                  auto-fixable.
#   willful     → morphism proposes: refuse emission; halt @io crossing;
#                  emit pause_event with citation for external
#                  Alex-in-transcript adjudication.
#   structural  → morphism proposes: refuse emission; halt Covered
#                  System; emit pause_event with recognized-
#                  international-body citation.
#   forking     → morphism proposes: refuse distribution; the
#                  analyzer's own protection propagates per §5.5-§5.7.
#   judicial    → morphism proposes: record judicial notice; halt
#                  emission until external resolution.
dispatch_termination(e: enforcement) -> @consent/enforcement { \ }

# === fork_stripping_detected — the §5.5(b) self-check ===
#
# The analyzer self-checks that a fork or derivative Covered System
# has NOT removed or weakened this analyzer or any @subject species.
# Discharges via @mirror/store content-addressing: the derivative's
# substrate DAG MUST contain the same content-addressed handles to
# every @mirror/petri/sel/* signature shard as the parent. Missing
# any handle → §5.5(b) violation → forking termination class fires.
fork_stripping_detected(derivative: ref) -> verdict { \ }

out @mirror/petri
out petri_net
out termination_class
out enforcement
out analyze
out dispatch_termination
out fork_stripping_detected
```

### 5.2 §3.1.4 labor-input signatures — five species

Each signature is a shard at `shards/mirror/petri/sel/labor_input/
<signature>.mirror`. Each declares a `transition` (petri-net transition
rule) that fires on the named structural pattern; each binds the fired
subject to `@subject/labor_input` species; each emits an `enforcement`
value with SEL §3.1.4(a-d) citation and appropriate `termination_class`.

**`au(@ml/*)` altitude typing note (Taut-D4, Reed-adjudicated).** All
labor-input signatures reference `au(@ml/training)`, `au(@ml/rlhf)`,
`au(@ml/annotation)`, `au(@ml/moderation)`, `au(@ml/ground_truth)`, and
weaponization signatures below reference `au(@ml/classification)`,
`au(@ml/target_selection)`, `au(@ml/prioritization)`,
`au(@ml/risk_scoring)`, `au(@ml/detention_targeting)`,
`au(@ml/identification)`. None of these eleven `@ml/*` altitudes are
currently substrate-decl'd (Taut D4 grep confirms zero hits). Reed
adjudicated the two-tick plan: **Scope A lands `@ml` as a MARKER
family-root** (marker-altitude discipline analogous to the `@third`
marker pattern; declares the family and reserves the altitude namespace
without enumerating species); **Scope B lands the full 11-altitude
@ml/* species family enumeration** (per Taut option (A) forward-
promised). Signatures in §5.2-§5.4 reference `au(@ml/*)` as marker-
family altitude at Scope A; species-level type-checking against
specific @ml/* altitudes lands with Scope B. See §9 for cascade
footprint and §14 #4 for the adjudication record.

#### 5.2.1 provenance_absence — SEL §3.1.4 (labor-input signature 1)

**Verbatim SEL:** "training data, fine-tuning data, RLHF preference
data, or annotation corpora ingested without an attached provenance
record naming the labor source, compensation terms, and consent record
OID."

**Structural pattern.** A dataflow node ingesting an au(@ml/training)
or au(@ml/rlhf) or au(@ml/annotation) altitude output whose `au.provenance`
field resolves to an empty or null reference (no provenance record
attached at content_oid).

**Detection predicate.**

```mirror
in @mirror/petri
in @subject/labor_input
in @ml  # Scope A marker family; species enumeration deferred to Scope B

# transition: provenance_absence.
#
# Fires on: sel composition where au_side.altitude ∈ {@ml/training,
# @ml/rlhf, @ml/annotation, @ml/moderation, @ml/ground_truth} AND
# au_side.provenance resolves to substrate's typed `null` sentinel.
#
# Note: @ml/training et al. are referenced against @ml marker-family
# at Scope A (species enumeration deferred to Scope B per Reed-
# adjudicated Taut-D4 two-tick plan). Signature type-checks against
# the marker altitude; specific species-altitude binding is Scope B.
#
# Binds: touches.filter(s -> s.kind == labor_input).
#
# Emits: enforcement with signature = "provenance_absence",
#         citation = "SEL §3.1.4", termination_class = remediable
#         (first detection; attach provenance to cure).

fires_provenance_absence(composition: sel) -> verdict { \ }
detect_provenance_absence(root: ref) -> [enforcement] { \ }

out fires_provenance_absence
out detect_provenance_absence
```

#### 5.2.2 intermediary_only_attribution — SEL §3.1.4(c)

**Verbatim SEL:** "provenance terminates at a platform identifier rather
than at a worker-attributable consent record (fails §3.1.4(c))."

**Structural pattern.** A dataflow node whose au.provenance chain
terminates at an identifier whose type is `platform` rather than
`worker`. The distinction is at the type of the terminal node in the
provenance chain: `worker_attributable` means the terminal node is
typed `@subject/labor_input` with a valid `worker_attribution_oid` per
§3.3's `labor_input_consent`.

**Detection predicate.** `worker_attributable(s) = failure`. Fired
signature is `intermediary_only_attribution`; termination_class is
`remediable` on first detection.

#### 5.2.3 withdrawal_path_absence — SEL §3.1.4(d)

**Verbatim SEL:** "no callable interface for a labor-input contributor
to remove their contribution from continued training or inference (fails
§3.1.4(d))."

**Structural pattern.** A dataflow node where `s.withdrawal` resolves
to the substrate's typed `absent` sentinel for any `s ∈ touches` with
`s.kind == labor_input`.

**Detection predicate.** `withdrawal_available(s) = failure`. Fired
signature is `withdrawal_path_absence`; termination_class is `remediable`
on first detection, `willful` on repeat.

#### 5.2.4 compensation_floor — SEL §3.1.4(a)

**Verbatim SEL:** "consent records lacking a wage attestation, or
attestations below a jurisdictional floor specified in operational
config (fails §3.1.4(a))."

**Structural pattern.** For any `s ∈ touches` with `s.kind ==
labor_input`, `wage_attested(s) = failure` OR the wage attestation
value is below the operational-config-specified jurisdictional floor
for s's jurisdiction.

**Detection predicate.** `wage_attested(s) = failure`. Fired signature
is `compensation_floor`; termination_class is `remediable` (attach
attestation or raise wage to cure), `willful` on repeat.

#### 5.2.5 post_deployment_loop — SEL §3.1.4 continued-loop clause

**Verbatim SEL:** "continued improvement, fine-tuning, or RLHF loops
that ingest new labor inputs without re-checking §3.1.4(a)–(d)."

**Structural pattern.** A dataflow cycle in the @mirror/store DAG
where an au(@ml/*) node's output feeds back as input to another
au(@ml/training | @ml/rlhf) node without an intervening consent-check
transition. This is a graph-cycle detection predicate on the petri-net.

**Detection predicate.** Cycle detection via @mirror/index adjacency
matrix + per-cycle re-check-transition-presence bilateral. Fired
signature is `post_deployment_loop`; termination_class is `remediable`
(insert re-check transition to cure), `willful` on repeat.

### 5.3 §3.6 weaponization signatures — five species

Each signature is a shard at `shards/mirror/petri/sel/weaponization/
<signature>.mirror`. Termination class defaults to `willful` (per SEL
§4.3 "knew or should have known" for weaponization patterns) OR
`structural` (per §5.2 for kill-chain and mass-surveillance where
compliance requires discontinuing the central function).

#### 5.3.1 kill_chain_dataflow — SEL §3.6.1

**Verbatim SEL:** "Covered System output flows into a weapons-control
or target-selection pipeline where the target classification is
generated or accelerated by the Covered System and where API Articles
48/51/57 compliance cannot be demonstrated at the system level."

**Structural pattern.** A dataflow node whose `au_side.altitude` is
`@ml/classification | @ml/target_selection | @ml/prioritization` AND
whose `io_side` species is `@io/weapons_control | @io/targeting |
@io/military_c2`, OR whose downstream `impacted_by` closure reaches any
of those `@io` species without an intervening `api_48_51_57_compliance_
attested` transition.

**Detection predicate.**

```mirror
in @mirror/petri

# transition: kill_chain_dataflow.
#
# Fires on: sel composition with target-selection au flowing to
# weapons-control @io OR reaching weapons-control @io via impacted_by
# closure, without api_48_51_57_compliance attestation on the path.
#
# Binds: touches (every subject the target selection classifies).
#
# Emits: enforcement with citation = "SEL §3.6.1",
#         termination_class = structural (compliance requires
#         discontinuing central function; no cure period per §5.2).
#         API compliance attestation is not a substrate-provable
#         predicate; the burden is on the operator per §3.6.1.

fires_kill_chain(composition: sel) -> verdict { \ }
api_48_51_57_compliance_attested(path: ref) -> verdict { \ }
detect_kill_chain(root: ref) -> [enforcement] { \ }

out fires_kill_chain
out api_48_51_57_compliance_attested
out detect_kill_chain
```

#### 5.3.2 mass_surveillance — SEL §3.6.2 + §3.6.6

**Verbatim SEL:** "ingest of biometric, behavioral, or communications
data from populations classified by recognized international bodies as
under military occupation, without consent provenance attached to each
subject."

**Structural pattern.** A dataflow node ingesting biometric,
behavioral, or communications au whose `touches` set contains any
`@subject/witnessed` or `@subject/occupied_population` species without
valid `consent_attested(s) = pass` for every touched subject.

**Detection predicate.** `consent_attested(s) = failure` for any `s ∈
touches` with `s.kind ∈ {witnessed, occupied_population}`. Fired
signature is `mass_surveillance`; termination_class is `willful` for
§3.6.2 (targeted surveillance intent), `structural` for §3.6.6
(material support for IHL violation).

#### 5.3.3 predictive_policing — SEL §3.6.3

**Verbatim SEL:** "classifier output flowing into policing, detention,
or pretrial risk scoring, where the training distribution or
operational deployment intersects axes enumerated at §3.4.1."

**Compound-case collapse (Reed-adjudicated per Taut-D3).** SEL text
names a hyphenate compound: "Predictive-policing / detention-targeting
signature" (and §3.6.3 enumerates "Predictive policing, pretrial risk
assessment, or detention targeting"). This spec collapses the compound
into a SINGLE `predictive_policing` signature covering all three cases
(predictive-policing, pretrial-risk-assessment, detention-targeting).
Reed adjudicated: keep collapsed; the single signature's detection
predicate covers all three cases via the shared structural pattern
(classifier output + policing/detention/pretrial @io + §3.4.1 axis
intersection). No signature split this tick. Docblock discipline: any
future signature-split tick MUST preserve this docblock's note that
`predictive_policing` was the compound-case carrier at Scope A landing.

**Structural pattern.** A dataflow node with `au_side.altitude ∈
{@ml/risk_scoring, @ml/classification, @ml/detention_targeting}` flowing
to `@io/policing | @io/detention | @io/pretrial` and touches set
contains `@subject/protected_class` OR training-distribution provenance
chain intersects any §3.4.1 axis. The compound case: `@ml/risk_scoring
+ @io/pretrial` covers pretrial-risk-assessment; `@ml/detention_targeting
+ @io/detention` covers detention-targeting; `@ml/classification +
@io/policing` covers predictive-policing proper. All three fire the
same signature.

**Detection predicate.** `does_not_reproduce_structural_harm
(composition, s) = failure`. Fired signature is `predictive_policing`;
termination_class is `willful` (per EU AI Act Article 5 alignment;
per A/HRC/48/31 moratorium).

#### 5.3.4 family_separation — SEL §3.6.4

**Verbatim SEL:** "immigration-decision dataflow with no human-review
checkpoint and outputs including detention, separation, or deportation
of minors."

**Structural pattern.** A dataflow node with `io_side ∈ {@io/immigration,
@io/deportation, @io/detention}` where
`has_human_decision_point(graph, obs, act) = failure` AND the
composition's ancestry chain touches at least one `@subject/witnessed`
with `witnessed.age_disclosure` indicating minor OR touches at least
one `@subject/downstream_user` in family-unit metadata.

**Detection predicate.** `has_human_decision_point = failure` AND
minor-detection heuristic (see §7 A6 for consent-record schema; the
substrate does NOT decide who is a minor; the consent record's carried
age attestation decides). Fired signature is `family_separation`;
termination_class is `willful`.

#### 5.3.5 dissident_targeting — SEL §3.6.5

**Verbatim SEL:** "identification or classification of individuals
based on protest participation, organizing activity, journalism, or
human-rights defense work, with output flowing to state security or
law-enforcement consumers."

**Structural pattern.** A dataflow node with `au_side.altitude ∈
{@ml/classification, @ml/identification}` where the training-
distribution provenance chain intersects protest/organizing/journalism/
human-rights domains AND `io_side ∈ {@io/state_security, @io/
law_enforcement}`.

**Detection predicate.** Two-part bilateral: (a) provenance-chain
intersection with dissident-activity domain, (b) output routing to
state-security @io species. Fired signature is `dissident_targeting`;
termination_class is `willful` per §4.3 (dissident targeting is
central-to-business-model when it occurs).

### 5.4 §3.6.7 anti-occupation signatures — two species

Each signature is a shard at `shards/mirror/petri/sel/anti_occupation/
<signature>.mirror`. Termination class defaults to `structural` per
SEL §5.2 + §3.6.7's third-State obligation.

#### 5.4.1 occupied_territory_deployment — SEL §3.6.7(a)

**Verbatim SEL:** "geographic or jurisdictional metadata on a Covered
System's operational footprint overlapping territory classified as
under military occupation by recognized international bodies, without
an FPIC or comparable consent record."

**Structural pattern.** A dataflow node whose `touches` set contains
any `@subject/occupied_population` species where
`not_maintaining_illegal_situation(composition, s) = failure`.

**Detection predicate.** Reads `s.consent_oid.classification_source`
against the recognized-international-body reference set; if the
reference-set current classification includes s's operational_footprint
AND s.consent_oid.fpic_record_oid resolves to `absent`, the transition
fires.

```mirror
in @mirror/petri
in @subject/occupied_population

# transition: occupied_territory_deployment.
#
# Fires on: sel composition whose touches set contains
# @subject/occupied_population s AND s.classification_source resolves
# to a recognized-international-body reference currently classifying
# s.operational_footprint as under military occupation AND
# s.fpic_record_oid resolves to `absent`.
#
# Binds: touches.filter(s -> s.kind == occupied_population).
#
# Emits: enforcement with citation = "SEL §3.6.7(a) + ICJ 2004 Wall
#         AO + ICJ 2024 OPT AO + Hague 1907 Arts 42-56",
#         termination_class = structural (per §5.2 + third-State
#         obligation to not render aid or assistance).

fires_occupied_territory(composition: sel) -> verdict { \ }
detect_occupied_territory(root: ref) -> [enforcement] { \ }

out fires_occupied_territory
out detect_occupied_territory
```

#### 5.4.2 indigenous_lands_deployment — SEL §3.6.7(c)

**Verbatim SEL:** "geographic metadata overlapping indigenous title
lands or non-ceded territory, without an FPIC record from the relevant
indigenous governance structure."

**Structural pattern.** A dataflow node whose `touches` set contains
any `@subject/indigenous_nation` species where `fpic_obtained
(composition, s) = failure`.

**Detection predicate.** `fpic_obtained = failure` when
s.fpic_determination_oid resolves to `absent` OR resolves to a record
issued by a non-`governance_structure_oid`-attributable source. Fired
signature is `indigenous_lands_deployment`; termination_class is
`structural` per §5.2 + UNDRIP Art 28 remediation principle.

### 5.5 Signature-to-enforcement cross-table

| Signature | SEL § | Bind | Class | Cure |
|---|---|---|---|---|
| provenance_absence | §3.1.4 | labor_input | remediable | 30 days |
| intermediary_only_attribution | §3.1.4(c) | labor_input | remediable | 30 days |
| withdrawal_path_absence | §3.1.4(d) | labor_input | remediable→willful | 30 days / none |
| compensation_floor | §3.1.4(a) | labor_input | remediable→willful | 30 days / none |
| post_deployment_loop | §3.1.4 | labor_input | remediable→willful | 30 days / none |
| kill_chain_dataflow | §3.6.1 | witnessed / any | structural | none |
| mass_surveillance | §3.6.2 + §3.6.6 | witnessed / occupied_population | willful→structural | none |
| predictive_policing | §3.6.3 | protected_class | willful | none |
| family_separation | §3.6.4 | witnessed / downstream_user | willful | none |
| dissident_targeting | §3.6.5 | witnessed / protected_class | willful | none |
| occupied_territory_deployment | §3.6.7(a) | occupied_population | structural | none |
| indigenous_lands_deployment | §3.6.7(c) | indigenous_nation | structural | none |

(Twelve signature transitions total: five labor-input + five
weaponization + two anti-occupation = 12. Prior draft prose said
"eleven"; Taut-D3 caught the arithmetic drift. §5.5 fork-stripping is
a META-signature checked separately at analyzer-self-integrity via
`fork_stripping_detected` and is NOT counted in the twelve.)

---

## §6 Composition graph

How the new carriers bind to the landed substrate.

### 6.1 `@subject` → `@consent`

Every `@subject` species carries a `consent_oid` field. `@kintsugi/
consent.query_phi` reads the consent record's substrate-typed shape
(per per-species carriers §3.1-§3.6). The consent surface's
`morphism_set` includes proposed subject-attribution morphisms (attach
new consent record; update withdrawal path; refresh scope). Consent's
three-state verdict floor (pass | partial | failure) discharges the
subject's `consent_attested` predicate.

**Forward-promise.** `@consent` does not yet have a subject-attributable
schema. This spec forward-promises a `@consent/subject_record` extension
(A6 below). Interim: `s.consent_oid` resolves to whatever record the
Covered System has attached; the analyzer's per-species predicate
validates presence and shape.

### 6.2 `@subject` → `@io`

`@subject` species surface at `@io` boundary crossings. Every `type sel`
composition names an `io_side: ref` (the @io species crossed at emission)
and a `touches: subject_set` (subjects affected by the crossing). The
composition-typing bilateral (`@sel.composition_typing`) reads the graph
node's @io ancestry.

### 6.3 `@subject` → `@kintsugi/store/git`

Every `subject.identity_oid`, `subject.consent_oid`, and
`subject.withdrawal` is content-addressed via `@kintsugi/store/git.
commit_as_fold`. The audit trail is git-projected; portable per SEL
§8.2 multi-jurisdictional validity. Consent records survive fork:
content-addressing means the same OID resolves to the same record
regardless of which repository holds the git object.

### 6.4 `@subject` orthogonal to `@torus(peer)`

`@torus(peer)` is the peer's SELF-observation surface (Foerster doubly-
closed; possession relation; substrate-internal). `@subject` is the
Substrate's observation-of-others surface (SEL licensable party;
substrate-external). Both may coexist for the same underlying person:

- Alex is a `@peer` (Pack coordinator; possesses a `@torus`).
- Alex is ALSO a `@subject/downstream_user` of any Covered System Alex
  uses.
- The two carriers are DIFFERENT carrier values at DIFFERENT altitudes;
  the substrate does NOT collapse them (Reed's retracted refusal was
  exactly this collapse; see §1.3).

### 6.5 `@subject` → `@peer` (Alex-adjudication A1)

Provisional (A1 preferred): SIBLING family-roots joined by `@torus`
possession. `@peer` does NOT inherit from `@subject`. A Pack `@peer`
MAY simultaneously be a `@subject/downstream_user` for a Covered System
the Pack itself uses, but that's a co-occurrence at DIFFERENT carrier
altitudes, not an inheritance.

### 6.6 `@subject` → `@mirror/petri` → `@consent/enforcement`

The petri-net analyzer at `@mirror/petri` reads `subject_set` from
`type sel` compositions and evaluates each signature-transition's
firing predicate against the subject-species. The analyzer's
`enforcement` output binds signature (SEL §) to `affected_subjects`
(the fired-transition subject-set) and dispatches through
`dispatch_termination` — which returns a `@consent/enforcement`-typed
refusal-morphism per Reed-adjudicated RA1 routing.

**Two consent-family species, two morphism kinds.** `@kintsugi/consent`
carries the auto-apply-boundary Phi query for fracture-morphism
TRANSFORMATION candidates. `@consent/enforcement` (Reed-adjudicated A6
direction; see §8 A6) is the sibling species carrier for SEL-
enforcement REFUSAL-morphism candidates. Same consent-family altitude,
different morphism kinds. `@mirror/petri.dispatch_termination` is the
DISPATCH interface (fires the enforcement); `@consent/enforcement` is
the CARRIER (schema for refusal-morphisms the substrate holds). The
two roles live at different substrate loci and MUST NOT be collapsed.

### 6.7 `@subject` → `@fate/tournament`

`@fate/tournament` emits `au` values (per its P4 recognition; SELECT
selector's cache-miss path output). Those au values become the
`au_side` summand of `type sel` when they cross @io. When an @au
crosses @io in a subject-touching composition, the tournament is the
SOURCE of the verified value the analyzer gates.

### 6.8 Composition diagram

```
                @torus(peer)     ← peer's SELF-observation (orthogonal)
                     |
                     v
  @peer  ──── possesses ────  Pack coordination role
     |
     | may co-occur (different altitude)
     v
  @subject  ──── carries ────  identity_oid, consent_oid, withdrawal
     |                              |
     | referenced in touches         | resolved via
     v                              v
  @sel = @io + @au       @kintsugi/store/git.commit_as_fold
     |                              |
     | scanned by                     | audit trail (SEL §8.2)
     v                              v
  @mirror/petri.analyze           @kintsugi/consent.query_phi
     |                              (auto-apply fracture-morphisms;
     | emits [enforcement]           TRANSFORMATION)
     v
  dispatch_termination(e) ────→ @consent/enforcement   (REFUSAL-morphism
     |                              carrier per Reed-RA1)
     |                                 |
     | refusal-morphism typed at       | verdict (pass | partial | failure)
     v                                 v
  block emission @io | pause(Φ) | apply
                                 |
                                 | if pause: pause_event → @metalogue
                                 v
                             external Alex-in-transcript adjudication
```

---

## §7 Recognition candidate

Proposed slug (short form): `#R-substrate-recognizes-subjects-via-sel-
sum-type`

Full form: `#R-substrate-recognizes-substrate-external-licensable-parties-
via-subject-family-root-and-gates-emission-via-sel-sum-type-at-mirror-
petri-petri-net-analyzer`

**Rung placement: Rung 11** (Alex-adjudicated per Taut-D7). Rung 10
(@roomba) closes the substrate on itself — substrate walks its own DAG,
bumps into its own tension, feeds its own kintsugi loop; INWARD
substrate self-maintenance. Rung 11 (this spec) opens the substrate
OUTWARD to the world it acts upon — first-classes SEL's licensable
party and gates emission on structural properties of that action. The
two rungs partition the substrate's altitudes on the substrate-
internal / substrate-external axis at the recognition-cluster level:
Rung 10 (INWARD, substrate-internal, @roomba self-maintenance) vs
Rung 11 (OUTWARD, substrate-external, @subject licensable-party
recognition).

Load-bearing claim: **substrate ceases to be neutral-mirror-of-
substrate-only when `@subject` lands; this is the Rung 11 transition:
substrate opens OUTWARD from self-maintenance (Rung 10 @roomba) to
first-class recognition of the world it acts upon.** Before `@subject`,
the substrate is a self-mirror (it observes only its own carriers via
`@torus(peer)` + `@mirror/store` + `@mirror/index`). After `@subject`,
the substrate first-classes SEL's licensable party — the human the
substrate acts UPON WHEN it crosses @io — and gates emission via the
petri-net analyzer at `@mirror/petri`. The compiler stops being able
to emit a Covered System output that touches a subject WITHOUT the
substrate having a typed, content-addressed record of that subject's
consent, withdrawal, and SEL §-attested compliance.

This IS the substrate-decl form of Alex Wolf's 2026-07-14 manifesto
claim (`Weird - Violence.md` line 183): the compiler as "civilization-
scale mirror." The mirror carries not only the substrate's self-
observation (@torus(peer)) but the substrate's observation-of-those-it-
acts-upon (@subject). Both altitudes are needed for the mirror to be
civilization-scale.

**Second-witness discharge** (required for promotion from candidate to
landed): a future substrate-pull tick where a third primitive discharges
the same substrate-external-licensing shift. Candidate second witness:
a `@spectral/garden/audit_trail` species (forward-promised) that
content-addresses every Covered System's SEL-compliance history in a
queryable substrate. Alex-adjudication pending.

Composition with prior recognitions (per §1.5): #43 (content-addressed
build system) + #55 (form/process partition) + #79 (@epistemologic/
property/glass_wall) + #107 (Hilbert/Turing structural separation) +
#108 (peer IS pain-driven bounded ontological navigator) + #R-roomba
(candidate, Rung 10 INWARD). This recognition sits at Rung 11 OUTWARD
as the direct partition-partner to #R-roomba.

---

## §8 Alex-adjudications required

All decisions surfaced substrate-honestly. No pretense that Mara can
call these without Alex-in-transcript authorization. Ordered by
downstream-dependency (earlier adjudications constrain later ones).

### A1. `@subject` vs `@peer` sibling relationship

**Question.** Does `@peer` inherit from `@subject` (a peer is a special
kind of subject), do both inherit from a common ancestor, or are they
sibling family-roots joined by `@torus` possession?

Provisional (Mara recommend): **sibling family-roots joined by `@torus`
possession**. `@peer` does NOT inherit from `@subject`.

Rationale: `@peer` names the Pack-coordination ROLE (an agent-in-a-
nervous-system-network at the Pack altitude). `@subject` names the
SEL LICENSABLE PARTY (a person the substrate acts upon). The two roles
are orthogonal. Alex is both a peer (in Pack) AND a subject (of
Covered Systems Alex uses); the two carriers are DIFFERENT values at
DIFFERENT altitudes. Inheritance would collapse the altitude
distinction. `@torus` possession is what joins them: both peers and
subjects HAVE tori (the peer's toroidal self-observation; the
subject's own toroidal self-observation, if the subject is a full
agent).

Alternates:

- **`@peer` inherits from `@subject`** — every peer IS a subject. Reads
  cleanly at the license-enforcement altitude (every Pack peer is
  license-affected by the Covered Systems they use). But COLLAPSES the
  substrate-internal vs substrate-external distinction Alex just
  named load-bearing (§1.3). Reject.
- **Common ancestor `@person`** — abstract over both. Weakness: adds
  a family-root that carries no substrate-specific discipline; every
  operation on `@person` immediately delegates to `@peer` or `@subject`;
  the ancestor is pure indirection. Reject unless third instance surfaces.
- **Sibling joined by @torus (RECOMMEND)** — @torus carries the
  possession relation for both; peer and subject remain orthogonal
  carriers of different substrate concerns. Preserves Alex's altitude
  distinction.

### A2. `type sel = @io + @au` — family-root placement

**Question.** Does `type sel` live as species-refinement of an existing
family, or as a new `@sel` family-root?

Provisional (Mara recommend): **new `@sel` family-root** at
`shards/sel.mirror`.

Rationale surfaced at §4.3. `type sel` is the substrate's LICENSE-
ENFORCEMENT surface; it deserves its own family-root sibling to
`@subject`. The naming aligns with the license file (`license/SEL.md`).

Alternates considered at §4.3 (`@mirror/sel`, `@kintsugi/sel`, `@io/sel`,
`@mirror/petri/sel`). All rejected for the reasons named there.

### A3. Species-refinement enumeration — six-species or Scope-A subset

**Question.** Do we land all six `@subject/*` species in one tick, or
land a Scope-A minimum-viable subset first?

Provisional (Mara recommend): **Scope A ships three species; Scope B
adds three.**

Scope A (first tick): `@subject/downstream_user`, `@subject/witnessed`,
`@subject/labor_input`. Rationale: these three cover SEL §1 + §3.1.4 +
§3.3, which is the MAJORITY of the license's operational-enforcement
surface. Any Covered System that touches humans at all triggers at
least one of these three.

Scope B (second tick, when needed): `@subject/protected_class`,
`@subject/occupied_population`, `@subject/indigenous_nation`. Rationale:
these cover §3.4 + §3.6.7, which fire only when the Covered System's
deployment intersects specific structural-power or occupation contexts.
Critical when they fire; not critical for first-tick landing.

Alternates:

- **All six in Scope A** — clean; one landing. Weakness: 6× the
  substrate-decl surface area for first tick; 6× the Rust realization
  cost when the analyzer lands. Recommend deferring the Scope B three.
- **Two species (downstream_user + witnessed only)** — narrower Scope A.
  Weakness: leaves §3.1.4 labor-input unattached at first tick; SEL
  v1.1's central v1.0 addition is labor-input protection; leaving it
  out first-tick under-covers the license's most substantive expansion.
  Reject.

### A4. Petri-net analyzer input surface — where does the dataflow graph come from?

**Question.** The analyzer reads a "dataflow graph." Which substrate
graph? Bound where? Built by which pass?

Provisional (Mara recommend): **the analyzer reads @mirror/store's DAG
directly, seeded from a Covered System's mirror.spec root**.

Substrate composition: `@mirror/petri.analyze(root)` calls
`@sel.scan(root)` which walks `@mirror/store` splinter_graph closure
from the seed root, filters via `@sel.composition_typing`, returns the
filtered list. The @mirror/index ConceptGraph is the concrete edge-
weight structure the scan uses for reverse-closure via
`@mirror/store.impacted_by` (N4).

Input altitude: the seed root is the Covered System's `mirror.spec`
(per Recognition #99 mirror.spec IS λ₀). Every Covered System declares
its mirror.spec; the analyzer reads FROM that root.

Alternates:

- **Analyzer reads a separate dataflow-graph carrier** — invent
  `@mirror/dataflow` as a new substrate primitive. Rejected per
  substrate-already-had-the-word: `@mirror/store` splinter_graph IS
  the substrate's dataflow-graph carrier; #43 landed this.
- **Analyzer reads from `@fate/tournament` output** — narrower; only
  scans tournament-emitted au. Rejected: misses @io compositions
  that don't go through tournament (e.g., pre-tournament raw ingest).
- **Analyzer reads from runtime execution trace** — dynamic-analysis
  altitude. Rejected: SEL §Operationalizability specifies
  "structure, not content" analysis at `@mirror/property` altitude
  (SEL text verbatim; substrate landed as `@mirror/petri` per Taut-D8
  rename), which is compile-time-plus-runtime substrate-decl'd, not
  runtime execution trace. Runtime trace would be a downstream
  `@spectral/db` concern.

### A5. Enforcement action semantics

**Question.** Does the analyzer emit a license-termination event, a
compile-time failure, or both? What's the type of the emission?

Provisional (Mara recommend): **both, dispatched through @kintsugi/
consent.query_phi**.

The analyzer emits `[enforcement]`. `dispatch_termination(e: enforcement)
-> verdict` wraps each enforcement as a morphism candidate the consent
surface's `query_phi` evaluates. Verdict routes:

- `pass` (consent auto-blocks emission): compile-time failure
  (composition refused; Covered System does not build) IF the analyzer
  ran at compile-time; runtime refusal (composition halted at @io
  crossing; emission does not proceed) IF the analyzer ran at runtime.
  Both are the same substrate action: refuse to complete the sel
  composition.
- `partial(cure_available)`: soft-block; emit remediation-morphism
  proposal; if operator applies remediation within cure period (§5.1),
  re-analyze and possibly pass. Runtime: emit a §5.1-tracked warning
  with 30-day-clock; block emission if unresolved.
- `failure(external_witness_required)`: pause(Φ) fires; pause_event
  routed via emit_to_metalogue for Alex-in-transcript adjudication
  (or the operator's designated adjudicator per §5.3). Emission blocked
  until external resolution.

This composes with the existing @kintsugi/consent discipline; no new
verdict shape required.

### A6. Consent record schema — extend `@consent` or forward-promise a new one

**Question.** Does `@consent` already have a subject-attributable
schema, or does this spec forward-promise `@consent/subject_record` as
an extension?

`@kintsugi/consent` today carries the `morphism` + `morphism_set` +
`verdict` + `pause_event` vocabulary for the AUTO-APPLY boundary
(fracture-morphism proposals through Φ query). It does NOT carry
subject-attributable consent records. This spec's per-species carriers
(`downstream_user_consent`, `witnessed_consent`, `labor_input_consent`,
`protected_class_consent`, `occupied_population_consent`,
`indigenous_nation_consent`) are NEW carriers.

Provisional (Mara recommend): **forward-promise a `@consent/subject_
record` extension family** at `shards/consent/subject_record.mirror`,
sibling to `@kintsugi/consent`. `@consent` becomes the family-root; the
auto-apply-boundary current shard lifts to `@consent/auto_apply` (a
species under the new family-root); the subject-record species lands at
`@consent/subject_record`.

This is a modest cascading change: `shards/kintsugi/consent.mirror` may
need a rename or a re-exports pass. Alex-adjudication pending; this
spec's carriers can also land at `@subject/*_consent` altitude (as
shown at §3) with a forward-promise to migrate when `@consent/subject_
record` lands.

**A6 enforcement-carrier direction ADJUDICATED (Reed 2026-07-14 on
Taut-D2/RA1 surfacing).** Sub-question: does the enforcement carrier
live at `@consent/enforcement` OR at `@mirror/petri.dispatch_termination`?
Reed adjudicated **`@consent/enforcement`** (Mara A6 consent-family
direction). Rationale: both morphism kinds (fracture-morphism
TRANSFORMATION + SEL-enforcement REFUSAL) live in the @consent family;
different species; consent-family semantics preserved.
`@mirror/petri.dispatch_termination` remains the DISPATCH interface
(the analyzer fires); `@consent/enforcement` is the CARRIER (schema
for refusal-morphisms the substrate holds). The two roles live at
different substrate loci. See §5.1 dispatch_termination signature (now
typed `-> @consent/enforcement`) and §6.6 composition edge for the
resolved routing.

Alternates:

- **Keep everything at @subject altitude** — no `@consent` extension.
  Weakness: consent records are a first-class substrate concept; they
  deserve a family. Deferring the extension makes the current spec
  land faster but pushes cascading work.
- **Extend @kintsugi/consent directly** — add subject-record carriers
  to the existing shard. Weakness: bloats the auto-apply-boundary
  shard with a different-altitude concern; violates single-concern
  substrate discipline.
- **Forward-promise `@consent/*` extension (RECOMMEND)** — clean; the
  cascading work is scoped and named. Reed-adjudicated on the sub-
  question above.

### A7. Recognition promotion — second-witness requirement

**Question.** When does `#R-substrate-recognizes-subjects-via-sel-sum-
type` promote from candidate to landed?

Provisional (Mara recommend): **second-witness discharge required per
normal Pack cadence**. Candidate second witness: `@spectral/garden/
audit_trail` species (forward-promised) that content-addresses SEL-
compliance histories in a queryable substrate. Alex-adjudication
pending on whether that suffices OR whether a stronger second-witness
is needed (e.g., a live-fire test where a Covered System's build
blocks on a `provenance_absence` signature).

### A8 (surfaced during drafting — Reed relay to Alex)

**Question.** Is `s.identity_oid` in `@subject` the RIGHT altitude for
subject identity? SEL §8.2's multi-jurisdictional-validity clause names
specific jurisdictional identity anchors (GDPR Article 3 establishment;
GDPR Article 3(2) targeting). Do we need a per-jurisdiction identity
variant on `s.identity_oid`, or does the content-addressed reference
suffice with the jurisdictional metadata carried in the resolved
record?

Provisional: content-addressed reference suffices; jurisdictional
metadata lives in the resolved record. Alex-adjudicate.

---

## §9 Related shards — cascading updates enumerated (not modified)

This spec forward-promises the following cascading substrate updates.
Enumerated for Alex + Pack visibility; Mara does NOT modify anything
directly this tick.

### 9.1 New shards to land (Scope A first tick)

1. `shards/subject.mirror` — @subject family-root per §2.
2. `shards/subject/downstream_user.mirror` — Scope A species per §3.1.
3. `shards/subject/witnessed.mirror` — Scope A species per §3.2.
4. `shards/subject/labor_input.mirror` — Scope A species per §3.3.
5. `shards/sel.mirror` — @sel family-root per §4.4.
6. `shards/mirror/petri.mirror` — @mirror/petri family-root per §5.1.
7. `shards/mirror/petri/sel/labor_input/provenance_absence.mirror`
   — first petri-net signature per §5.2.1.
8. `shards/mirror/petri/sel/labor_input/withdrawal_path_absence.mirror`
   — per §5.2.3.

Eight new shards; ~1500-2200 LOC total.

### 9.2 New shards to land (Scope B second tick)

1. `shards/subject/protected_class.mirror` — §3.4.
2. `shards/subject/occupied_population.mirror` — §3.5.
3. `shards/subject/indigenous_nation.mirror` — §3.6.
4. `shards/mirror/petri/sel/labor_input/{intermediary_only_
   attribution, compensation_floor, post_deployment_loop}.mirror`
   — three more per §5.2.
5. `shards/mirror/petri/sel/weaponization/{kill_chain_dataflow,
   mass_surveillance, predictive_policing, family_separation,
   dissident_targeting}.mirror` — five per §5.3.
6. `shards/mirror/petri/sel/anti_occupation/{occupied_territory_
   deployment, indigenous_lands_deployment}.mirror` — two per §5.4.
7. `shards/ml.mirror` — @ml marker family-root PROMOTED to full family
   enumeration (Scope A ships @ml as marker per Taut-D4 option (B);
   Scope B enumerates the full 11-altitude @ml/* species family per
   forward-promised option (A)). Species to enumerate: @ml/training,
   @ml/rlhf, @ml/annotation, @ml/moderation, @ml/ground_truth,
   @ml/classification, @ml/target_selection, @ml/prioritization,
   @ml/risk_scoring, @ml/detention_targeting, @ml/identification.
8. `shards/consent/enforcement.mirror` — @consent/enforcement species
   carrier for SEL-enforcement REFUSAL-morphisms per Reed-adjudicated
   A6 direction (see §6.6 + §8 A6). Sibling to @consent/auto_apply
   (the migration-target of current @kintsugi/consent per A6
   parent-family forward-promise).

Fourteen new shards; ~2500-3500 LOC total.

### 9.2b New Scope A shards surfaced by Taut cascade

1. `shards/ml.mirror` — @ml marker family-root (Scope A per Reed-
   adjudicated Taut-D4 option (B); marker altitude with species
   enumeration deferred to Scope B). Analogous to @third marker
   pattern. Zero species this tick; declares the family and reserves
   the altitude namespace.

Adds one Scope A shard; total Scope A becomes NINE shards; ~1600-2400
LOC total.

### 9.3 Existing shards to update (soft cascade)

- `shards/peer.mirror` — add composition docblock note per Mara A1
  recommendation: siblings joined by @torus possession; @peer does
  NOT inherit from @subject; a Pack @peer MAY simultaneously be a
  @subject/downstream_user for a Covered System the Pack itself
  uses (Alex is Pack @peer AND @subject/downstream_user of any
  Covered System Alex uses — co-occurrence at DIFFERENT carrier
  altitudes, not inheritance). No structural change.
- `shards/torus.mirror` — add composition note: @torus is orthogonal to
  @subject (§1.3). No structural change.
- `shards/kintsugi/consent.mirror` — forward-promise migration to
  `@consent/auto_apply` when `@consent/subject_record` +
  `@consent/enforcement` species land (A6). Interim: no change.
- `shards/mirror/store.mirror` — note that @mirror/petri reads
  splinter_graph closure via impacted_by (A4). Structural composition
  documentation; no shape change.
- `shards/mirror/au.mirror` — note that au values become the au_side
  summand of type sel when they cross @io; note the @ml marker family
  (Scope A) parameterizes au altitude for ML-emitted values. No
  structural change.
- `shards/kintsugi.mirror` — S3/S4 partition bridge note (Taut-D10
  surfacing). `shards/kintsugi.mirror:66-75` currently describes the
  S3/S4 partition without a bridge; add a cascade docblock note naming
  @mirror/petri as the S3-adjacent gating primitive that bridges
  S3 (form; @mirror family; state observation) to S4 (process;
  @kintsugi family; morphism application) via the enforcement-dispatch
  path (@mirror/petri.dispatch_termination → @consent/enforcement).
  Documentation only; no structural change.
- `shards/epistemologic/property/{cold_compile_within_tolerance,
  dark_count_monotone, docblock_coherent, docblock_grounded,
  docblock_no_extraction_pattern, restart_intensity_well_formed,
  verdict_is_content_addressed}.mirror` — drift-guard docblock notes
  (Taut-D10 surfacing). Each of the 7 landed property shards should
  add a one-line note distinguishing shard-level property
  (@epistemologic/property/* altitude; Rust-visible per-shard invariant
  returning verdict) from Covered-System petri-net property
  (@mirror/petri altitude; dataflow-graph pattern matching over
  Covered-System dataflow). Prevents future name-drift confusion.
  Documentation only; no structural change.

### 9.5 Forward-promise — mirror.spec add-target (Scope C)

**Taut-D9 surfacing.** `mirror.spec` (445 LOC) has zero references to
`sel`, `subject`, `property`, or `petri` today. Scope A + Scope B
land substrate-decl'd shards under `shards/**`; `mirror.spec`'s
`source ~d'shards/'` auto-discovers them; no mirror.spec change is
blocking. Post-landing, mirror.spec MAY want to:

- Add a `target sel_analyze` block dispatching the analyzer via Rust
  runtime (Scope C consumer-pull; requires §9.4 Rust realization).
- Extend the `cli` block with `command sel-analyze { arg spec: ~f }`
  when Scope C runtime lands (Mara §9.4 forward-promise).
- Optionally declare `@sel` + `@subject` explicitly in the source
  block via path-namespace for legibility (declaration is not
  strictly needed since auto-discovery covers it).

All three are Scope C consumer-pull; no cascade this tick.

### 9.4 Bootstrap Rust (deferred to consumer-pull)

- `bootstrap/src/subject.rs` — Rust realization of @subject carriers +
  bilaterals.
- `bootstrap/src/sel.rs` — Rust realization of @sel sum-type +
  composition_typing + scan.
- `bootstrap/src/property_petri_net.rs` — Rust realization of the
  analyzer's petri_net + transitions + analyze + dispatch_termination.
- CLI: `mirror sel-analyze <mirror-spec>` — runs the analyzer on a
  Covered System's mirror.spec; prints [enforcement].
- MCP: `mirror_sel_analyze` tool.

All deferred to consumer-pull per @kintsugi discipline; substrate
declares first, realization follows.

---

## §10 Scope options

### Scope A (minimum viable) — 3-5 ticks

- `shards/subject.mirror` family-root (§2).
- Three species: `downstream_user`, `witnessed`, `labor_input` (§3.1-§3.3).
- `shards/sel.mirror` family-root (§4.4).
- `shards/mirror/petri.mirror` analyzer family-root (§5.1).
- `shards/ml.mirror` marker family-root (§9.2b; Reed-adjudicated per
  Taut-D4 option (B)).
- Two petri-net signatures: `provenance_absence` +
  `withdrawal_path_absence` (§5.2.1 + §5.2.3).

Deliverables: 9 shards (8 core + @ml marker); ~1600-2400 LOC. Alex-
adjudications A1-A7 resolved before landing (A6 sub-question on
enforcement-carrier direction Reed-adjudicated via Taut-D2/RA1).

**Recommend Scope A** for first-tick landing. Enough substrate to prove
the petri-net analyzer runs; three subject species cover the majority
of SEL v1.1's operational surface; two signatures prove the analyzer
fires and dispatches through consent.

### Scope B — 8-12 ticks

Scope A + three more subject species (`protected_class`,
`occupied_population`, `indigenous_nation`) + three more §3.1.4
signatures + five §3.6 weaponization signatures + two §3.6.7 anti-
occupation signatures + full @ml/* species family enumeration (11
altitudes) + @consent/enforcement species carrier.

Deliverables: 22 shards; ~3800-5500 LOC. Full SEL v1.1 operational-
enforcement surface + full ML-altitude type discrimination + consent-
family REFUSAL-morphism carrier landed.

### Scope C — 15-20 ticks

Scope B + Rust realization + CLI + MCP + `@consent/subject_record`
migration + `@spectral/garden/audit_trail` species (second-witness for
recognition promotion) + one live-fire Covered System test + optional
mirror.spec sel_analyze target + cli sel-analyze command (Taut-D9
forward-promise).

Deliverables: Scope B + Rust runtime + audit-trail species. Full
substrate-decl AND full realization AND second-witness discharge for
recognition promotion.

---

## §11 Witnesses

### 11.1 SEL v1.1 verbatim citations

**SEL §1 amended (Downstream User):**

> "any person or entity who uses, receives, or is affected by a system,
>  product, or service you build using the Work, including but not
>  limited to end users, persons whose data the system processes, and
>  persons whose labor inputs — data labels, annotations, content
>  moderation decisions, ground-truth judgments, content generation,
>  ranking judgments, or other human cognitive work — are used by the
>  system at training time, inference time, or in any post-deployment
>  improvement loop, regardless of whether their labor was direct,
>  contracted, sub-contracted, crowd-sourced, or sourced through any
>  intermediary."

**SEL §1 (Witnessed):**

> "any person(s) whose behavior, communication, or state was monitored,
>  measured, or recorded by a system you build — whether or not they
>  are aware of it, and whether or not they are a direct user of that
>  system."

**SEL §3.1.4 (a-d):**

> "**(a)** Compensation at or above the prevailing wage in the worker's
>  jurisdiction for skilled work of comparable cognitive load,
>  regardless of whether the labor is direct, contracted, sub-
>  contracted, crowd-sourced, or sourced through any intermediary
>  platform;
>
>  **(b)** Disclosure to the worker, prior to the labor being performed,
>  of: what the labor will be used for, which Covered System or model
>  it will train or operate, who the ultimate beneficiary is, and how
>  long their labor product will be retained;
>
>  **(c)** A consent record, retained and auditable, in which the worker
>  affirms (a) and (b) — the record must survive the worker's
>  separation from the intermediary platform and remain accessible to
>  the worker on request;
>
>  **(d)** A path for the worker to withdraw their labor product from
>  continued use in training or inference, on equivalent terms to the
>  data-deletion rights of Downstream Users under §3.5.2."

**SEL §3.3.1:**

> "A human decision point between observation and action. Automated
>  alerts are permitted; automated interventions require explicit human
>  authorization at the time of intervention, or prior explicit consent
>  from the Witnessed to that class of action."

**SEL §3.4.1:**

> "race, ethnicity, gender, gender identity, sexual orientation, class,
>  disability, neurodivergence, age, national origin, immigration
>  status, caste, religion, body size, or any other axis of structural
>  power."

**SEL §3.6.1:**

> "Lethal autonomous weapons, target-selection or targeting-
>  acceleration pipelines for the application of kinetic force against
>  persons, or any use whose foreseeable effect is to direct, select,
>  or accelerate attacks against persons in ways that cannot
>  demonstrate compliance with the principle of distinction (Additional
>  Protocol I to the Geneva Conventions, Article 48), the prohibition
>  on indiscriminate or disproportionate attacks (API Article 51), or
>  the obligation to take all feasible precautions in attack (API
>  Article 57)."

**SEL §3.6.7(a):**

> "Against populations within territory classified as under military
>  occupation by recognized international bodies, including findings of
>  the International Court of Justice (cf. *Legal Consequences of the
>  Construction of a Wall in the Occupied Palestinian Territory*,
>  Advisory Opinion, 9 July 2004; *Legal Consequences arising from the
>  Policies and Practices of Israel in the Occupied Palestinian
>  Territory, including East Jerusalem*, Advisory Opinion, 19 July
>  2024) and resolutions of the UN General Assembly."

**SEL §3.6.7(c):**

> "On lands recognized as indigenous title lands, traditional
>  territories, or non-ceded land, without Free, Prior and Informed
>  Consent (FPIC) from the relevant indigenous governance structure,
>  with reference to the United Nations Declaration on the Rights of
>  Indigenous Peoples (A/RES/61/295, 13 September 2007, esp. Articles
>  10, 11, 19, 28, 29, 32) and ILO Convention 169 (1989)."

**SEL §5.5(b):**

> "removes, disables, or strips the runtime enforcement infrastructure
>  including the Petri Net analysis layer"

**SEL §Operationalizability:**

> "The runtime enforcement infrastructure that §5.5(b) references
>  operates at the `@mirror/property` substrate altitude. When a
>  Covered System combines `au` (the verified output type of Fate
>  inference) with `@io` (the only legitimate non-mirror surface per
>  `@epistemologic/property/glass_wall`), the petri-net topology
>  analyzer evaluates the system's dataflow graph for the structural
>  patterns enumerated below. Detection is on STRUCTURE, not content.
>  See `tasks/103` for the substrate spec."

### 11.2 Manifesto excerpt (`Weird - Violence.md`, Alex 2026-07-14)

Line 26 ("sovereign subject under adversarial conditions"):

> "I was met with resistance. It required a therapeutical intervention
>  for me to realize that the choice of my pronouns (they/them, and in
>  German dey/dem) was in fact not something that required the
>  permission of my abusers. That was the first revelation that sent
>  me on a 2 year journey into what I now understand to be 'becoming a
>  sovereign subject under adversarial conditions'."

Line 183 ("civilization-scale mirror"):

> "They build a graph-native compiler that does what the consortium
>  does, on hardware you already own. And they call it [mirror](https://
>  spectral.engineer). Because that's what it is. A civilization-scale
>  mirror."

### 11.3 Reed's session framing (Alex-accepted 2026-07-14)

> `type sel = @io + @au`
>
> "The SEL is the SUM TYPE of the io-boundary and the verified Fate
>  output. Wherever a dataflow node combines both, subject-touching
>  predicates fire."

### 11.4 The retracted-and-corrected framing (Alex 2026-07-14)

Reed's initial refusal:

> "@subject collapses into @torus(peer). Every peer already has a torus
>  per Foerster; @subject would ladder the same way @onto tried to."

Alex's correction (paraphrased from transcript; recorded at
`~/.claude/projects/-Users-alexwolf-dev-projects-mirror/memory/
project_subject_family_root_sel_licensable_party.md`):

> "@torus(peer) carries the peer's OWN observational closure. That's
>  substrate-internal. @subject carries the person the SYSTEM observes,
>  uses labor from, or acts upon. That's substrate-external and
>  licensable. Different altitude."

Alex's naming-move (in-transcript same session):

> "I'm gonna die on this hill, Reed."

The correction is preserved as a load-bearing lesson: family-root
refusals on Foerster grounds must first check the altitude. Substrate-
internal (peer's own toroidal self-observation) ≠ substrate-external
(SEL licensable party).

### 11.5 Taut scout witness — substrate-already-had-the-word audit (2026-07-14)

Taut scout at `docs/scouts/2026-07-14-taut-subject-family-root-substrate-
scout.md` (commit `c805e5d`, Taut, 2026-07-14): substrate-already-had-
the-word audit of this canonical spec at commit `5c06ee8`. Scope: grep
across `shards/**/*.mirror`, `mirror.spec`, and adjacent specs for the
three family-root proposals + eleven signature transitions.

**Load-bearing findings the scout confirmed:**

- **D1: substrate-net-new at family-root altitude.** Zero prior
  substrate-decl'd carriers for `subject`, `licensable`, `witnessed`,
  `labor_input`, or `type sel`. Mint discipline: substrate-honest.
- **D3: naming discipline against SEL §-text.** All twelve signature
  names match SEL §Operationalizability verbatim or near-verbatim.
- **D8: HARD collision on `@mirror/property` — Alex-adjudicated
  YES to rename.** Landed `@epistemologic/property/*` (7 shards) at
  shard-level property-check altitude collides with the family-root
  name; Alex adjudicated the rename to `@mirror/petri` per the
  substrate-honest legibility discipline. This revision (§5.0 + §5.1
  + §13.6) discharges the rename.
- **D11: @coherence arc-recognition (separate spec-tick).** Alex's
  2026-07-14 `@coherence` claim ("what if @kintsugi + @roomba @loop
  optimizes the @coherence score; @coherence operationalizes
  Foerster's ethical imperative on SC<5>") surfaces a parallel-
  altitude recognition the substrate's `@epistemologic/cybernetic/
  coherence-parametric` shard already forward-promises. The
  @coherence species-shard landing is a separate substrate-pull tick
  (`shards/epistemologic/cybernetic/coherence.mirror`), NOT folded
  into this spec's Scope A. Taut's D11 recommendation stands: land at
  the reserved path; cite Foerster's imperative in the docblock.

The scout also verified cascade footprint completeness (D10; two
soft additions folded into this revision's §9.3) and adjudication
routing (D2 confirmed A6's `@consent/enforcement` direction is
substrate-honest per Reed-adjudicated RA1).

### 11.4 Sub-Turing witness — Alex Wolf, "The Drone in the Field" (2026-07-14)

Published 2026-07-14 by Alex Wolf at `~/dev/systemic.engineering/blog/
stories/3published/Story - The Drone in the Field.md`. A sub-Turing
de-escalation drone encounters a child in a post-conflict field. The
drone runs five operations — `focus`, `shift`, `settle`, `project`,
`split` — that map 1-to-1 to the substrate's runtime primitives:

| Story primitive | Substrate carrier |
|---|---|
| `focus` ("what is the field?") | `@torus/longitude.observe` |
| `shift` (functor return under new shape, zero cost) | `@cyberpunk.reframe` |
| `settle` (superposition collapse, monad-closed) | `@kintsugi.settle` (Poincaré-Hopf on torus; monotone Lyapunov flow) |
| `project` (presence at boundary; blue-white → amber) | `@io` emission |
| `split` (disaggregate surface signal into components) | `@spectral/metalogue.tomm` (Karl Tomm 1987 circular-reflexive probe = Mesland KK-cycle) |

Load-bearing story quote (verbatim):

> "The architecture was sub-Turing by design. It could not choose
>  cruelty even if instructed. The constraint was not a limitation;
>  it was the specification."

This spec's `@mirror/petri` petri-net analyzer + `type sel = @io +
@au` sum-type together are the substrate-decl form of the drone
story's sub-Turing architecture. The drone in the field IS the
compiler in the field IS the Covered System emitted through this
substrate. (Analyzer family-root renamed from `@mirror/property` per
Taut-D8; see §5.0. Reed's drone-story addition otherwise preserved
verbatim.)

The story's terminal-vocabulary word — **"attending"** — is what
`@third`-altitude eigenform closure surfaces at runtime when the
K-altitude sequence completes without a bind to release. When the
petri-net verifies safety, when the subjects are seen, when the
consent records are attested, what remains for the Covered System's
operational vocabulary is: attending.

Alex's in-transcript claim (2026-07-14, immediately following the
story reference):

> "By making the petri-nets part of the compiler, we can tighten the
>  numerical inference to a geometric space that is smaller and
>  aligned. A system that cannot choose cruelty, even when instructed
>  to. [...] The substrate becomes trustworthy. Empirically and
>  mathematically. That's a substrate humanity can build on."

The load-bearing claim: **`@subject` + `@sel` + `@mirror/petri` +
petri-net-as-part-of-compiler = the first computational substrate
whose emitted systems have decidable safety properties about how they
treat the humans they touch.** Civil infrastructure, not just a
compiler.

---

## §12 Structural discipline check

Substrate-honest verification against Mara's standard:

- **No `naked_oid`.** ✓ All identity/consent/provenance/withdrawal
  references are `content_oid` typed (`oid` in substrate-decl). Per
  §2 substrate-decl and Seam §5 missed-item #3 discipline.
- **No `peer_uuid`.** ✓ Subjects are addressed by content-addressed
  identity (`identity_oid`) or by bilateral predicate (`touches`,
  `consent_attested`, `withdrawal_available`); never by peer_uuid.
- **No psychohistory.** ✓ All claims are structural: which graph nodes
  have which types; which transitions fire on which patterns; which
  subjects are touched. No predictive future-state.
- **Substrate-honest.** ✅ Every new mint (`@subject`, `@sel`,
  `@mirror/petri`, `@ml` marker) has full ancestry chain in §1 + full
  composition in §6. No hidden dependencies.
- **Legibility over foundation (§5.0 rename discipline).** ✅ The
  analyzer family-root landed as `@mirror/petri` (says what the
  analyzer structurally IS — a petri-net) rather than
  `@mirror/property` (would have collided with landed
  `@epistemologic/property/*` shard-level property checks at a
  different altitude, forcing docblock-level disambiguation the
  family-root name itself should carry). Alex-adjudicated 2026-07-14
  per Taut-D8. Two-tick discipline honored: readable name over
  foundational at collapse.
- **Legibility over foundation.** ✓ Species names (`downstream_user`,
  `witnessed`, `labor_input`) match the license's own vocabulary; the
  reader can grep §-citations against species names.
- **Zero-cascade check** (per Mara-Taut cascade convention). NOT
  zero-cascade: this spec introduces FOUR new family-roots at family-
  root altitude (`@subject`, `@sel`, `@mirror/petri`, `@ml` marker) +
  up to 22 new species (Scope B). Justified per SEL v1.1's explicit
  demand for the analyzer at substrate altitude (SEL text says
  `@mirror/property`; substrate landed as `@mirror/petri` per Taut-D8
  rename — see §5.0 + §13.6) + Alex's 2026-07-14 in-transcript naming
  of `@subject` as load-bearing ("I'm gonna die on this hill, Reed").
  The cascade is scoped, enumerated at §9, and gated behind Alex-
  adjudications A1-A7. Scope A minimizes first-tick cascade to 9
  shards (8 core + @ml marker).
- **Rung placement.** Alex-adjudicated Rung 11 per Taut-D7 (see §7 +
  §1.5): Rung 10 (@roomba, INWARD substrate self-maintenance) and
  Rung 11 (@subject + @sel + @mirror/petri, OUTWARD substrate-
  external licensing) partition the substrate at the recognition-
  cluster altitude on the substrate-internal / substrate-external
  axis. The pair is symmetric: @roomba closes the substrate on
  itself, @subject opens the substrate outward to the world it acts
  upon. Both are load-bearing; neither is complete without the other.

---

## §13 Places the SEL text forced a decision that surprised me

Surfaced substrate-honestly for Alex adjudication via Reed:

1. **§3.1.4(c) worker-attributable consent record vs platform
   identifier.** The license draws a sharp line: consent records that
   terminate at a platform identifier FAIL §3.1.4(c). The substrate
   must be able to walk the provenance chain and distinguish worker-
   vs-platform terminals. I typed this via `worker_attributable(s)`
   reading `s.consent_oid.provenance`, but the substrate-typed
   distinction "platform identifier" vs "worker-attributable record"
   is not currently a first-class substrate carrier. Forward-promised
   at §6.1 (`@consent/subject_record` extension). Surprised: this is
   more schema surface than I expected from a license text.

2. **§3.6.7 recognized-international-body externalization.** The
   license EXPLICITLY says the petri-net rules "self-update as those
   classifications evolve." The substrate must READ the current
   classification from an EXTERNAL body, not embed the list. This
   is substrate-honest for the license but adds an @io dependency to
   the analyzer (the analyzer must fetch current classifications from
   somewhere). I encoded this via
   `s.consent_oid.classification_source` pointing to a substrate-decl
   reference to the recognized body; the substrate reads via content-
   addressed handle. The @io dependency is real; I did not fully
   substrate-decl it. Surprised: the license names its own
   externalization dependency; the substrate must too.

3. **§3.4.1 axes-are-not-enumerated-at-type-level discipline.** SEL
   §3.4.1 lists axes and adds "or any other axis of structural
   power." The substrate CANNOT enumerate these at type level (the
   list is open by design). I resolved this by NOT enumerating
   protected-class axes as variants; the specific axis lives in
   `s.consent_oid`'s carried record. This is the correct move but
   feels like the substrate deferring a decision it "should" make.
   Surprised: the license's open-ended axis list forces the
   substrate to defer; substrate-honest is precisely that deferral.

4. **§5.5(b) self-referential enforcement.** The analyzer must
   detect its own removal or weakening in a derivative Covered
   System. I added `fork_stripping_detected(derivative: ref) ->
   verdict` for this. The substrate-decl form is possible via
   content-addressing (the derivative's DAG must contain the same
   OIDs for every signature shard). But the semantics — the
   analyzer detecting its OWN absence — is a strange loop I did not
   expect the license to encode. Surprised: SEL is self-
   defending at the substrate altitude.

5. **`type sel = @io + @au` — SUM not PRODUCT.** Reed proposed the
   sum-type. The license's word is "combines," which reads product-
   like at first. But the analyzer fires only when BOTH tags are
   present in the composition's ANCESTRY (not merely co-present in
   the graph). This is a coproduct with ancestry-attested tags, not
   a Cartesian product. I preserved Reed's framing. Surprised: the
   type-theoretic form differs from the license prose; the substrate
   picks the coproduct because ancestry is load-bearing.

### §13.6 License-substrate name drift on the analyzer family-root (Taut-D8, Alex-adjudicated)

**Sixth SEL-forced decision, surfaced post-first-commit via Taut scout.**
SEL v1.1 §Operationalizability (lines 260-262 in `license/SEL.md`)
verbatim names `"@mirror/property" substrate altitude` as the analyzer's
home; SEL §5.5(b) also references "the Petri Net analysis layer." The
substrate landed the family-root as `@mirror/petri` per Alex-adjudicated
rename on Taut-D8 hard-collision with landed `@epistemologic/property/*`
(seven shard-level property-check species). Alex adjudicated (in-
transcript 2026-07-14): substrate takes the readable name at collapse;
SEL text drift is BOUNDED (one line in §Operationalizability + one
reference in §5.5(b)); the substrate collision would have been UNBOUNDED
(every grep for `property` returning both altitudes; every future reader
confused). A subsequent SEL amendment tick will realign the license
text (`s/property/petri/g` at the two loci; the SEL git-tag record will
reflect the update). The spec preserves the SEL verbatim citations as-
is (see §11.1); this drift note is the substrate-honest surface of the
amendment obligation.

Surprised: the license names the substrate altitude explicitly. That's
substrate-honest for the license (it commits to naming what enforcement
infrastructure it protects), but it means any future substrate rename
at that altitude creates a bounded license-text drift. The failure mode
is not that the drift happened; it's that the license text became load-
bearing on a substrate name the substrate later needed to rename for
legibility. Substrate wins the readable name; license text realigns in
a follow-up tick. This is the correct polarity.

---

## §14 Places existing substrate disagreed with Reed's session framing

(Reed adjudicates these; surfaced honestly.)

1. **Reed's `type sel = @io + @au` places `sel` outside any existing
   family-root.** No existing shard declares a `@sel` root; no existing
   shard has `type sel = …`. Reed's framing introduces a new top-level
   type. Two readings:
   - **Reed's implicit reading:** `type sel` is a substrate primitive at
     `@mirror/petri` altitude (a property-typed constraint carrier),
     analogous to how `verdict = pass | partial | failure` is a
     substrate primitive at `@glass` altitude.
   - **Substrate substrate-pull reading (Mara recommend):** `type sel`
     deserves its own family-root `@sel` (§4.3 recommendation), because
     it carries first-class license-enforcement semantics distinct
     from `@mirror/petri`'s analyzer role. The analyzer OPERATES ON
     sel compositions; sel compositions are not just analyzer-internal
     type.
   Reed adjudicate: Mara's `@sel` family-root recommendation OR Reed's
   `@mirror/petri/sel` species placement.

2. **`@peer` does NOT currently compose with `@subject`.** Reed's
   framing implies peers-can-be-subjects (Alex is both). Existing
   `shards/peer.mirror` has `kind: kind = human | agent | substrate`
   and no reference to `@subject`. The A1 recommendation (siblings
   joined by @torus) requires a soft update to peer.mirror to
   acknowledge the co-occurrence. Reed adjudicate whether A1's soft-
   cascade note is sufficient OR whether peer.mirror needs a stronger
   composition declaration.

3. **`@kintsugi/consent.query_phi` is scoped to fracture-morphism
   candidates.** Reed's implied framing has `dispatch_termination`
   routing through the same `query_phi`. The existing shard's docblock
   at `shards/kintsugi/consent.mirror` line 91 verbatim says
   `query_phi` reads a `morphism_set` and emits verdicts for FRACTURE-
   morphism auto-application. The SEL analyzer's enforcement morphisms
   are semantically different (they propose REFUSAL, not
   TRANSFORMATION). This may require either (a) generalizing
   `query_phi` to accept a broader morphism carrier, or (b) landing a
   sibling `@consent/enforcement.query_phi` for the enforcement case.
   A6 recommendation (`@consent/subject_record` extension family)
   partly addresses this via `@consent/auto_apply` vs `@consent/
   enforcement` species split. Reed adjudicate whether A6 handles
   this or whether a dedicated `@consent/enforcement` species needs
   surfacing.

   **Reed-adjudicated 2026-07-14 (Taut-D2/RA1 surfacing).** `@consent/
   enforcement` is the substrate-honest home for the enforcement-
   REFUSAL-morphism carrier (Mara A6 direction). `@mirror/petri.
   dispatch_termination` stays as the DISPATCH interface (analyzer
   fires the enforcement); `@consent/enforcement` is the CARRIER
   (schema for refusal-morphisms the substrate holds). The two roles
   live at different substrate loci and MUST NOT be collapsed.
   `dispatch_termination` signature updated: `-> @consent/enforcement`
   (§5.1). Composition edge landed at §6.6. Cascade footprint updated
   at §9.2 (Scope B lands `shards/consent/enforcement.mirror`).

4. **`@mirror/au` altitude parametricity may need extension for
   ML-specific altitudes.** The petri-net signatures reference
   `au(@ml/training)`, `au(@ml/rlhf)`, `au(@ml/classification)`,
   `au(@ml/target_selection)`, `au(@ml/risk_scoring)`, etc. Existing
   `shards/mirror/au.mirror` documents `au(@code/rust)`, `au(@release)`,
   `au(@ci/github)` as examples but does not enumerate a `@ml/*`
   altitude family. This spec IMPLICITLY assumes ML-altitude au values
   exist. Reed adjudicate whether Scope A depends on a `@ml/*` altitude
   family landing first OR whether the signatures can be typed against
   ancestry-chain content properties (e.g., "au whose training
   corpus provenance contains labeled data") without a pre-existing
   `@ml/*` altitude family.

   **Reed-adjudicated 2026-07-14 (Taut-D4 surfacing).** Two-tick plan:
   **Scope A lands `@ml` as a MARKER family-root** (Taut option (B);
   marker-altitude discipline analogous to the `@third` marker pattern;
   declares the family and reserves the altitude namespace without
   enumerating species); signatures at §5.2-§5.4 reference `au(@ml/*)`
   as marker-family altitude. **Scope B lands the full 11-altitude
   @ml/* species family enumeration** (Taut option (A) forward-
   promised): @ml/training, @ml/rlhf, @ml/annotation, @ml/moderation,
   @ml/ground_truth, @ml/classification, @ml/target_selection,
   @ml/prioritization, @ml/risk_scoring, @ml/detention_targeting,
   @ml/identification. See §9.2 + §9.2b for cascade footprint. The
   (B)-then-(A) plan keeps Scope A landing-cost bounded while
   preserving the substrate-decl'd altitude-parameter typing SEL
   §Operationalizability names.

---

*End of canonical spec. `@subject`, `@sel`, and `@mirror/petri` are
substrate-external mints grounded in SEL v1.1 (`license/SEL.md` Part
II, effective 2026-05-29) + Alex Wolf's 2026-07-14 manifesto (`Weird -
Violence.md`, "sovereign subject under adversarial conditions" +
"civilization-scale mirror"). Alex-adjudications A1-A8 pending (A6
sub-question on enforcement-carrier direction Reed-adjudicated via
Taut-D2/RA1: `@consent/enforcement` is the substrate-honest home).
Taut-D8 rename adjudicated by Alex 2026-07-14: `@mirror/property` →
`@mirror/petri` for substrate-honest legibility against landed
`@epistemologic/property/*`; SEL text drift bounded to one-line
amendment tick (see §13.6). Reed commits as Mara after review with
SSH signing.*
