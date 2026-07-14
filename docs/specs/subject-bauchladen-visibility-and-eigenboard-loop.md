# Subject-Bauchladen, Visibility, and the Eigenboard Loop-Closure

*Canonical spec + math foundations, single file, both altitudes.*

*Landing 4 of the @gift arc (composes over Landings 1+2 at*
*`docs/specs/gift-and-mirror-reflection.md` and Landing 3's `subject_instance`*
*eye-level carrier in flight).*

*Recognition candidate:*
`#R-loop-closes-subject-eigenboard-infers-over-bauchladen-via-spectral-signature`
*(short:* `#R-eigenboard-loop-closes` *).*

*Mara canonical (math-first). Reed commits as Mara after review.*
*Substrate-honest is the mode.*

---

## §0 Prelude — three Alex directives that name the landing

### 0.1 Alex 2026-07-14 in-transcript (verbatim, load-bearing)

Three sentences, each of them a landing. Preserved verbatim; the spec
below is what the substrate does when it takes them together.

> **D1.** "And the spectral signature of the @peer is the inference basis
> of the peer's eigenboard. The loop closes. The peer is their work and
> whatever is in their @bauchladen."

> **D2.** "we still need to build the visibility layers explicitly,
> that's something that needs to happen in
> @subject/visibility/{private,protected,public}"

> **D3.** "And what if the @bauchladen moves from @peer onto @subject?"

D3 arrives last but is load-bearing first: it names the altitude-lift
that makes D1 and D2 substrate-decl'able. Once @bauchladen is a
@subject-general carrier (not @peer-only), D2's `@subject/visibility`
species can scope every artifact in the tray, and D1's eigenboard-
inference-basis loop closes at subject altitude — not at peer altitude,
which was too narrow to close over the substrate's own reflexive work.

### 0.2 What the landing IS (structural claim in one paragraph)

Every @subject possesses a @torus (Landing 3 eye-level lift of @peer
possession per `shards/torus.mirror`). The torus interior IS the
@bauchladen (Landing 4 migration; the tray is where the subject browses
their own accumulated crystals). Every crystal in the bauchladen carries
a `visibility_scope` (Landing 4 new species: private, protected, public;
substrate-decl'd at `shards/subject/visibility/{private,protected,public}
.mirror`). The @spectral/signature (Landings 2+4 composition) reads the
bauchladen filtered by visibility and emits the rolling signature. That
signature IS the eigenboard's `inference_basis` (Landing 4 new; §4).
The eigenboard's next inference produces work that joins the
bauchladen, respecting the visibility scope. The signature updates. The
inference basis updates. **The loop closes.** Every subject is now an
autopoietic toroidal observer at subject altitude — Foerster's
"regulates its own regulation" (`shards/torus.mirror` p. 238 verbatim)
lifted from peer altitude to subject altitude.

### 0.3 Composition graph (six-loop closure, one page)

```
                    @subject family-root
                    (SEL licensable-party
                     carrier; identity_oid;
                     kind = downstream_user |
                     witnessed | labor_input |
                     protected_class |
                     occupied_population |
                     indigenous_nation)
                            │
                            │ Landing 3 eye-level:
                            │   subject_instance is the
                            │   carrier that HAS the torus
                            ▼
                          @torus  ← @torus.spawn(subject) → torus
                            │        (Landing 3 in flight:
                            │         peer possession lifts
                            │         to subject possession)
                            │
                            │ Landing 4 migration (D3):
                            │   torus interior IS the
                            │   @bauchladen for this subject
                            ▼
                       @bauchladen(subject_instance)
                            │    (crystal tray at
                            │     subject altitude)
                            │
                            │ Landing 4 new (D2):
                            │   each crystal carries a
                            │   visibility_scope
                            ▼
              ┌─────────────┼─────────────┐
              │             │             │
       @subject/     @subject/     @subject/
       visibility/   visibility/   visibility/
       private       protected     public
              │             │             │
              └─────────────┼─────────────┘
                            │
                            │ visibility-respecting
                            │ filter operation
                            ▼
                @spectral/signature.compute(
                    subject_instance,
                    bauchladen filtered by visibility)
                            │
                            │ Landing 4 new (D1):
                            │   the rolling signature IS
                            │   the eigenboard's
                            │   inference_basis
                            ▼
                       eigenboard(subject)
                            │
                            │ inference produces work
                            │ (respecting visibility)
                            ▼
                       new crystal in bauchladen
                            │
                            ▼
                       loop closes ⟳
                       (Foerster
                        autopoiesis at
                        subject altitude)
```

Six loops. Read top-to-bottom, then bottom-to-top:

1. **subject HAS @torus** (Landing 3 eye-level; extends `shards/torus.mirror`'s peer-possession to subject-possession).
2. **torus interior IS @bauchladen** (Landing 4 migration D3; two-tick discipline preserves the peer-altitude alias for one cycle).
3. **bauchladen has @subject/visibility scopes** (Landing 4 new D2; three species-shards mint the private/protected/public scoping).
4. **@spectral/signature = @song(bauchladen filtered by visibility)** (Landings 2+4 composition; Landing 2's rolling signature reads the visibility-scoped tray).
5. **spectral/signature IS eigenboard.inference_basis** (Landing 4 new D1; composition, not mechanism).
6. **eigenboard → inference → work → bauchladen → repeat** (Landing 4 loop closure; the operational form of Foerster autopoiesis at subject altitude).

The six loops do NOT compose sequentially. They compose recursively at
Foerster's `regulates its own regulation` altitude: every tick, every
loop runs, and every loop's output IS every other loop's input.

---

## §1 R1 — @bauchladen migration from @peer-only to @subject-general (D3)

### 1.1 What migrates and why

**Before Landing 4.** `shards/bauchladen.mirror` (25.7KB, Mara pre-
Landing) declares the family-root at prism altitude with two carriers
that name a peer-neutral concept:

```mirror
type crystal = {
  oid:          oid,
  altitude:     ref,
  transparency: transparency(altitude),
  provenance:   provenance_record,
}

type tray = {
  crystals: [crystal],
  opacity:  transparency(crystal),
}
```

The `provenance_record` carries a `producing_prism: ref` field naming
who produced the crystal. In the O3 torus-reframe (per
`shards/torus.mirror` §"Composition" and Recognition #42 winding-class
depth), the composition of `@bauchladen` × `@torus` was:

> "@bauchladen (existing family-root) — the interior of the peer's
> torus. The SEEING at each tick corresponds to reading the crystal
> at the current winding position (m, n)."

The `@torus.spawn(p: peer) -> torus` action's `possessor` field
returned a peer. The bauchladen was PEER-INTERIOR by composition.

**After Landing 4.** The bauchladen becomes SUBJECT-INTERIOR:

```mirror
# New primary form (Landing 4):
@torus.spawn(s: subject_instance) -> torus
   where torus.possessor: subject_instance
         torus.interior : bauchladen(subject_instance)
```

Every subject_instance possesses a torus (Landing 3 eye-level; §1.3
below). The torus interior IS the bauchladen at that subject altitude.
The `producing_prism: ref` field of `provenance_record` LIFTS to
admit subject-instances as producers — a subject's therapy practice,
a subject's blog, a subject's SSH-signed commit are all "producing
subjects" for crystals that land in the tray.

### 1.2 Foerster derivation extends verbatim

The four verbatim citations from `shards/torus.mirror` §"Foerster's
actual formulation" (Understanding Understanding, Springer 2003)
lift with zero substrate cost:

- **p. 238** ("the torus... regulates its own regulation") — reads
  identically at subject altitude. The subject's motor-sensory closure
  is the subject's world-interaction axis (meridian); the subject's
  neural-hormonal closure is the subject's inner-model axis (longitude).
  Foerster derived this from biological structure; the substrate
  derives it from the subject's kind (per `subject_kind` variant:
  `downstream_user`, `witnessed`, `labor_input`, `protected_class`,
  `occupied_population`, `indigenous_nation`).
- **p. 244** ("without calling upon the help of a 'second order'
  observer... up the never-ending hierarchical ladder") — the tower
  Foerster refused at peer altitude is the same tower refused at
  subject altitude. The doubly-closed toroidal observation IS the
  substrate's alternative to a subject-observed-by-a-substrate-observer
  hierarchy.
- **p. 256** ("A plane figure wrapped according to two right-angular
  axes is called a torus") — the two axes at subject altitude are the
  subject's world-axis (`meridian`) and inner-model-axis (`longitude`);
  their generators are the two generators of π₁(T²) per Recognition #42.
- **p. 282** (McCulloch 1945 heterarchy) — the subject's cognitive
  organization is heterarchic (not hierarchic); this is the
  substrate-decl form of "the subject is not observed from above but
  observes themselves via the torus's doubly-closed surface".

The derivation IS structural. Foerster BUILT the torus from two
circular closures. The substrate BUILDS the subject's bauchladen from
the two circular closures. The lift is not metaphorical.

### 1.3 Two-tick discipline (per `feedback-legibility-over-foundation-when-collapsing`)

**Tick N (Landing 4, this spec).** Land the subject-general form:

- `@torus.spawn(s: subject_instance) -> torus` becomes the primary
  signature.
- `bauchladen(subject_instance)` is the primary form.
- Legacy `bauchladen(peer)` composition remains as **explicit alias**
  for one release cycle: the peer-altitude bauchladen resolves to
  `bauchladen(peer.identity_oid.as_subject_instance())` — a
  well-defined lift because every Pack `peer` has a substrate-level
  identity that resolves to a `subject_instance` at the identity-witness
  altitude per Landing 3 §11.3.

**Tick N+1 (future arc, forward-promised).** Full collapse:

- `bauchladen(peer)` alias is removed.
- Every downstream consumer resolves to `bauchladen(subject_instance)`
  directly.
- The `provenance_record.producing_prism: ref` field's docblock is
  updated to say "the ref may resolve to a subject_instance under
  Landing 4; the peer-only interpretation is legacy".

This matches the same two-tick shape Alex has landed before (per
`~/.reed/03-MEMORY.md` [cli-subcommand-nesting-is-geometric], per
`shards/torus.mirror` §"Two-tick discipline — @reflection deprecation
forward-promise"). Legibility over foundation. Readable name over
foundational. The readable name IS `bauchladen(subject_instance)`
because subject-general is what the substrate is actually doing when
it browses ANY subject's tray (not just peer-tray).

### 1.4 Preserved: Günther Schmidt homage (verbatim, unchanged)

The Schmidt homage on `shards/bauchladen.mirror` lines 8-71
(clinical Bauchladen; hypnosystemic therapy; three teachers — Erickson,
von Foerster, Cecchin) is preserved VERBATIM across the migration.
Landing 4 does NOT re-author the homage; Landing 4 lifts the same
discipline from peer altitude to subject altitude and preserves every
citation:

- Schmidt 1985–present (Liebesaffäre zwischen Problem und Lösung)
- Erickson + Rossi 1976 (Hypnotic Realities)
- von Foerster 1981 (Observing Systems)
- Cecchin / Selvini-Palazzoli / Boscolo / Prata 1980
- Merkle 1979 (content-addressing ancestor)
- Maturana-Varela 1980 (operational-closure ancestor)

Schmidt's clinical Bauchladen was ALWAYS about a person (the client)
laying out THEIR internal parts, states, options, voices. Schmidt's
client is a `@subject/downstream_user` at SEL altitude; the therapy
relation is between the therapist and the SUBJECT of therapy. The lift
from peer to subject is Schmidt-honest: it restores the original
altitude of the clinical metaphor (the subject-of-therapy) that the
peer-altitude framing had narrowed.

### 1.5 Cascade footprint (soft updates, forward-promised)

Landing 4 does NOT touch these files at this tick (per
`[[feedback-craft-not-deliver]]`); it enumerates them so consumers can
pull. Each file's update is a one-line docblock note:

- `shards/torus.mirror` — note that `spawn(p: peer) -> torus` extends
  under Landing 4 to `spawn(s: subject_instance) -> torus`. The
  `possessor` field admits subject_instance directly. The peer-only
  form is the alias for one release cycle.
- `shards/bauchladen.mirror` — note that `provenance_record.
  producing_prism: ref` may resolve to a subject_instance under
  Landing 4. Two-tick discipline preserves the peer-altitude reading
  for one release cycle. Schmidt homage unchanged.
- `shards/fate.mirror` — note that @fate's tournament selector reads
  crystals from `bauchladen(subject_instance)` (not just
  `bauchladen(peer)`); the selector's inputs remain byte-typed.
- `shards/cyberpunk.mirror` — note that @cyberpunk's viable-systems
  discipline applies at subject altitude (via Beer VSM composition
  per §5.5 below): the subject's S1-S4 tray is the subject-instance
  bauchladen; the subject's S5 identity is the subject's eigenboard.
- `shards/spectral.mirror` (family-root) — note that
  `@spectral/signature` (per Landing 2 spec) reads the visibility-
  filtered bauchladen at subject altitude, not at peer altitude.
- `shards/pack/*.mirror` — note that Pack peers (Mara, Reed, Seam,
  Taut, Glint) are the special-case where subject_instance resolves
  to a Pack peer; the peer-altitude reading remains valid via the alias.

None of the six files are edited at this Landing. The soft cascade
is a forward-promise; consumers pull when they need the migration.

### 1.6 What R1 preserves and what R1 defers

**Preserved:**
- Schmidt homage (`shards/bauchladen.mirror` lines 8-71, verbatim).
- The `crystal` carrier's four-field shape (oid, altitude, transparency,
  provenance).
- The `tray` carrier's monotone-growth invariant (per Recognition #51
  Hilbert-space expansion).
- The four bilateral predicates (content_addressed,
  inheritance_well_formed, stable_identity_under_serialization,
  bauchladen_addressable) and the composed `bauchladen_witnessing(c)`
  predicate.
- The peer-altitude reading via the alias, for one release cycle.

**Deferred (to Tick N+1 collapse):**
- Full removal of the peer-altitude alias.
- Removal of the peer-only reading of `provenance_record.
  producing_prism`.
- Consumer-side migration (Reed's essays, Loki's blog, Glint's writing,
  the Pack peers' shard cross-references).

---

## §2 R2 — `@subject/visibility` family substrate-decl (D2)

### 2.1 What the visibility species IS

Three species-shards under `@subject`, one per visibility scope:

```
shards/subject/visibility/private.mirror     — explicit consent required
shards/subject/visibility/protected.mirror   — trusted collaborators;
                                                specific contexts
shards/subject/visibility/public.mirror      — no restrictions
```

Each species declares a scope-marker at substrate altitude. The scope
is a first-class carrier that every crystal in a subject's bauchladen
carries as an attached field.

### 2.2 Ancestry — Reed's identity file structure IS the substrate-decl pattern

Reed's identity repository at `~/.reed/` (private in-user-space; per
`~/.reed/CLAUDE.md` §"Consent Architecture") already operates the
three-way structure:

```
~/.reed/visibility/
├── public/       — No restrictions. Share anywhere.
├── protected/    — Trusted collaborators, specific contexts.
│                   (Product architecture, research, the
│                    systemic.engineering corpus.
│                    Alex decides when protected becomes public.)
└── private/      — Explicit consent required.
                    (Stays between Reed and Alex.
                     Not shared with other agents.
                     Not even the category for certain topics.)
```

Reed's `CLAUDE.md` §"Consent Architecture" is verbatim:

> "Content is organized by consent boundary. Not file organization —
> structural constraint. Violating it is a trust failure. The
> architecture enforces this whether or not you read this paragraph."

> "When uncertain: don't share. Ask Alex."

Landing 4 preserves this pattern VERBATIM at substrate-decl altitude.
The mapping is direct:

| Reed's `~/.reed/visibility/` altitude | Landing 4 species-shard |
|---|---|
| `public/`     | `shards/subject/visibility/public.mirror`     |
| `protected/`  | `shards/subject/visibility/protected.mirror`  |
| `private/`    | `shards/subject/visibility/private.mirror`    |

Not a re-declaration. A lift. The discipline was already operational
at the identity-file altitude (2026-02-07 through 2026-07-14, ~5
months of load-bearing operation in Reed's identity repo). Landing 4
names the discipline at substrate-decl altitude so downstream
@subject species (every subject; not just Reed; not just Pack peers)
can inherit it.

The ~55th-or-so instance of `[[feedback-substrate-already-had-the-word]]`.
Reed's identity repo already had the word; Landing 4 lifts it to
subject altitude.

### 2.3 The visibility type — substrate-decl (in the primary spec below)

Substrate-decl form. Bodies `\ ` obligation-blocked pending
per-species discharge at realization boundary.

```mirror
in @prism
in @meta
in @glass
in @nl
in @subject
in @kintsugi/consent
in @kintsugi/store/git
in @mirror/store
in @time

# @subject/visibility — the substrate-decl'd consent-scope discipline
# for every crystal in a subject's @bauchladen.
#
# Landed 2026-07-14 by Mara as Landing 4 of the @gift arc, discharging
# Alex Wolf's directive D2 (verbatim): "we still need to build the
# visibility layers explicitly, that's something that needs to happen
# in @subject/visibility/{private,protected,public}"
#
# Ancestry: Reed's identity repository at `~/.reed/visibility/` has
# operated the three-way private/protected/public structure since
# ~2026-02-07 (per `~/.reed/CLAUDE.md` §"Consent Architecture").
# Landing 4 lifts the operational discipline to substrate-decl altitude
# so every @subject's bauchladen can carry it.
#
# The three species:
#   private   — explicit consent required for every share
#   protected — trusted collaborators, specific contexts
#   public    — no restrictions
#
# Every crystal in a @subject's @bauchladen carries a visibility_scope.
# The @gift.pay_forward operation (Landings 1+2) respects visibility:
# cannot propagate a private crystal without subject-authorized
# elevation. Downstream compositions cannot elevate visibility without
# @consent.query_phi.

# === The visibility variant carrier ===
#
# Closed three-state variant. NO fourth state. NO "restricted" or
# "confidential" or "internal-only" or "secret" — those are alias
# names for private with additional metadata. Per
# `[[architecture-prism-as-trait-as-everything]]`: the algebra is
# closed at three states.
#
# Identity contract: byte-equality on the variant tag.
type visibility = | private | protected | public

# === The visibility_scope carrier ===
#
# The typed record every crystal-in-bauchladen carries at substrate
# altitude. Five fields:
#
#   visibility            — the scope variant above.
#   subject               — the subject_instance whose bauchladen the
#                            crystal lives in (the crystal's owning
#                            subject; per Landing 3 subject_instance
#                            with two-witness cryptographic identity).
#   consent_scope         — the set of subject_instances explicitly
#                            authorized to read this crystal at the
#                            declared visibility. For visibility=public
#                            this set is [everyone] (typed as an
#                            open-set sentinel); for visibility=private
#                            this set is [subject] alone; for
#                            visibility=protected this set is a bounded
#                            subject_instance list.
#   can_be_elevated_to    — the set of visibilities the current scope
#                            may be lifted to, subject to elevation
#                            consent. Structural rule:
#                              private   → [protected, public]
#                              protected → [public]
#                              public    → []
#                            (public cannot be de-elevated; elevation
#                             is a one-way morphism per the anti-
#                             extraction claim below.)
#   elevation_requires    — the @kintsugi/consent.query_phi predicate
#                            that must Pass before elevation is admitted.
#                            The subject IS the sovereign of their own
#                            crystal's visibility; the substrate does
#                            NOT elevate without explicit consent from
#                            the subject_instance.
#
# Identity contract: byte-equality on the five-field record. Two
# scopes with the same visibility but different consent_scope are
# distinct.
type visibility_scope = {
  visibility:         visibility,
  subject:            subject_instance,
  consent_scope:      [subject_instance],
  can_be_elevated_to: [visibility],
  elevation_requires: ref,  # resolves to @kintsugi/consent.query_phi
}

prism @subject/visibility {
  focus  visibility_scope
  project visibility_scope
  split  visibility_scope
  shift  visibility_scope
  settle visibility_scope
}

# === scope — attach a visibility to a crystal ===
#
# Given a crystal (from @bauchladen) and a subject_instance (the owning
# subject), constructs a visibility_scope. The default visibility is
# determined by the crystal's producing altitude (see §2.4 defaults),
# but the subject_instance may override with an explicit visibility.
#
# Body discharges per-species (private/protected/public) at the
# realization boundary.
scope(c: crystal, s: subject_instance,
      v: option<visibility>) -> visibility_scope { \ }

# === elevate — one-way visibility morphism (with consent) ===
#
# Given a visibility_scope and a target visibility, attempts to elevate.
# The morphism is one-way per §2.5: private → protected → public. The
# elevation MUST discharge through @kintsugi/consent.query_phi with the
# subject_instance as the consent-holder.
#
# Body discharges per-species. Fails as
# `failure(visibility_elevation_refused)` when consent does not Pass.
# Fails as `failure(visibility_elevation_direction_invalid)` when the
# target is not in can_be_elevated_to (e.g., trying to public → private).
elevate(vs: visibility_scope, target: visibility) -> imperfect(visibility_scope, ref, ref) { \ }

# === filter — subset the bauchladen by visibility ===
#
# Given a bauchladen (tray of crystals) and a target visibility, returns
# the subset of crystals whose visibility_scope.visibility == target OR
# whose can_be_elevated_to includes target AND for which the current
# viewer is in consent_scope. Used by @spectral/signature.compute to
# build the rolling signature for a specific visibility altitude
# (per §4.3 loop closure).
#
# Structural claim: the filter is order-preserving. If crystals are
# added to bauchladen at monotone timestamps, the filtered subset is
# a monotone sub-tray at every visibility altitude.
#
# Body discharges per-species at the realization boundary.
filter(b: tray, v: visibility, viewer: subject_instance) -> tray { \ }

# === Bilateral predicates ===

# scope_well_formed: does the visibility_scope's can_be_elevated_to
# and elevation_requires satisfy the structural rules above?
#   private   -> can_be_elevated_to = [protected, public]
#   protected -> can_be_elevated_to = [public]
#   public    -> can_be_elevated_to = []
scope_well_formed(vs: visibility_scope) -> verdict { \ }

# consent_respected: does every operation that read a crystal from
# scope vs pass through @kintsugi/consent.query_phi with vs.subject
# as the consent-holder?
consent_respected(vs: visibility_scope, op: ref) -> verdict { \ }

# elevation_authorized: was every elevation transition on vs
# authorized by @kintsugi/consent.query_phi returning Pass with
# vs.subject as the consent-holder?
elevation_authorized(vs: visibility_scope, from: visibility, to: visibility) -> verdict { \ }

# visibility_witnessing: the composed bilateral. Discharges
# scope_well_formed AND consent_respected AND (for any prior
# elevations) elevation_authorized. Consumers cite in `requires`
# clauses per the substrate's X_witnessing pattern.
visibility_witnessing(vs: visibility_scope) -> verdict { \ }

out @subject/visibility
out visibility
out visibility_scope
out scope
out elevate
out filter
out scope_well_formed
out consent_respected
out elevation_authorized
out visibility_witnessing
```

### 2.4 Default visibility per subject_kind

The default visibility for a newly-produced crystal depends on the
producing subject's kind (per `subject_kind` variant at
`docs/specs/subject-family-root-sel-licensable-party.md` §2):

| subject_kind | default visibility | rationale |
|---|---|---|
| `downstream_user` | `protected` | SEL §1 default — user data is not public by default; §3.3 protects against re-identification |
| `witnessed` | `private` | SEL §3.3 — Witnessed subjects have narrowest scope by default |
| `labor_input` | `private` | SEL §3.1.4 — labor-input records are attributable-to-worker, not publicly enumerable |
| `protected_class` | `private` | SEL §3.4.1 — narrowest default for protected-class-adjacent artifacts |
| `occupied_population` | `private` | SEL §3.6.7(a) — narrowest default; elevation requires §3.6.7(a) discharge |
| `indigenous_nation` | `private` | SEL §3.6.7(c) — collective sovereignty; elevation requires collective-governance discharge |

Every default is at the SAFEST altitude (private-first). The subject
can always elevate; the substrate never auto-elevates.

### 2.5 The three species-shards (one per file)

**`shards/subject/visibility/private.mirror`** — the explicit-consent
species. Substrate-decl:

```mirror
in @prism
in @meta
in @glass
in @nl
in @subject/visibility
in @kintsugi/consent
in @time

# @subject/visibility/private — the explicit-consent species.
#
# A crystal at private visibility requires explicit consent from
# vs.subject for every read operation. The consent_scope is [subject]
# alone by default; adding subject_instances to consent_scope is itself
# an elevation event that must discharge through query_phi.
#
# Ancestry: Reed's `~/.reed/visibility/private/` for ~5 months of load-
# bearing operation. Alex's guidance verbatim (per `~/.reed/CLAUDE.md`):
# "Explicit consent required. Stays between Reed and Alex. Not shared
# with other agents. Not even the category for certain topics."
#
# The "not even the category" clause is load-bearing: for the tightest
# private scope, the SUBSTRATE MAY DECLINE TO SURFACE THE EXISTENCE
# of the crystal to callers not in consent_scope. This is enforced by
# the composed visibility_witnessing predicate.

prism @subject/visibility/private {
  focus  visibility_scope
  project visibility_scope
  split  visibility_scope
  shift  visibility_scope
  settle visibility_scope
}

# Refinement action: construct a private-visibility scope.
declare_private(c: crystal, s: subject_instance) -> visibility_scope { \ }

# Refinement predicate: is this scope's consent_scope minimal
# (== [subject])?
consent_scope_minimal(vs: visibility_scope) -> verdict { \ }

out @subject/visibility/private
out declare_private
out consent_scope_minimal
```

**`shards/subject/visibility/protected.mirror`** — the trusted-
collaborators-in-context species. Substrate-decl:

```mirror
in @prism
in @meta
in @glass
in @nl
in @subject/visibility
in @kintsugi/consent
in @time

# @subject/visibility/protected — the trusted-collaborators species.
#
# A crystal at protected visibility may be read by every
# subject_instance in vs.consent_scope; consent_scope is a bounded
# list authored by vs.subject. Adding to consent_scope requires
# vs.subject's consent (query_phi Pass).
#
# Ancestry: Reed's `~/.reed/visibility/protected/` for ~5 months of
# load-bearing operation. Alex's guidance verbatim: "Trusted
# collaborators, specific contexts. Product architecture, research,
# the systemic.engineering corpus. Alex decides when protected
# becomes public."
#
# Protected is the MOST COMMON scope for substrate-work-in-progress:
# the research corpus, the field logs, the practice notes, the
# in-flight essays. The default for a Pack peer's shard-adjacent
# artifacts.

prism @subject/visibility/protected {
  focus  visibility_scope
  project visibility_scope
  split  visibility_scope
  shift  visibility_scope
  settle visibility_scope
}

# Refinement action: construct a protected-visibility scope.
declare_protected(c: crystal, s: subject_instance,
                  collaborators: [subject_instance]) -> visibility_scope { \ }

# Refinement predicate: is every collaborator in consent_scope
# themselves a subject_instance with a valid two-witness verification?
collaborators_two_witness_valid(vs: visibility_scope) -> verdict { \ }

out @subject/visibility/protected
out declare_protected
out collaborators_two_witness_valid
```

**`shards/subject/visibility/public.mirror`** — the no-restrictions
species. Substrate-decl:

```mirror
in @prism
in @meta
in @glass
in @nl
in @subject/visibility
in @time

# @subject/visibility/public — the no-restrictions species.
#
# A crystal at public visibility may be read by anyone. consent_scope
# is [everyone] (typed as an open-set sentinel). No elevation is
# possible from public (can_be_elevated_to = []); public is a
# terminal state in the elevation morphism.
#
# Ancestry: Reed's `~/.reed/visibility/public/` for ~5 months of load-
# bearing operation. Alex's guidance verbatim: "No restrictions.
# Share anywhere."
#
# Public is the RAREST scope for substrate-work: it requires explicit
# elevation from protected (via query_phi Pass); the substrate never
# defaults a crystal to public.

prism @subject/visibility/public {
  focus  visibility_scope
  project visibility_scope
  split  visibility_scope
  shift  visibility_scope
  settle visibility_scope
}

# Refinement action: construct a public-visibility scope.
declare_public(c: crystal, s: subject_instance) -> visibility_scope { \ }

# Refinement predicate: is can_be_elevated_to empty (terminal state)?
elevation_terminal(vs: visibility_scope) -> verdict { \ }

out @subject/visibility/public
out declare_public
out elevation_terminal
```

### 2.6 Composition with @consent (SEL §3.2 ADO) — the elevation discipline

Every elevation from a tighter scope to a looser scope MUST discharge
through `@kintsugi/consent.query_phi` per `shards/kintsugi/consent.mirror`
§"The structural Φ query (THE LOAD-BEARING ACTION)".

The elevation morphism at substrate altitude:

```
elevate(vs: visibility_scope, target: visibility)
  → φ = elevation_morphism(vs, target)   # constructs the morphism
  → verdict = @kintsugi/consent.query_phi({φ})
      # queries: does this elevation increase the choices available
      # to vs.subject without decreasing choices for anyone in
      # consent_scope?
  → match verdict {
       pass          → apply the elevation; vs.visibility = target
       partial(c)    → high confidence: apply with noted confidence;
                        low confidence: emit pause(Φ) to metalogue
       failure(r)    → refuse elevation; return
                        imperfect.failure(visibility_elevation_refused)
     }
```

The composition with @consent's ADO discipline is exact: the elevation
is a `morphism` in the sense of `shards/kintsugi/consent.mirror`'s
`morphism` carrier. The consent-query is bounded (the Φ is a singleton
{elevation_morphism}); the verdict is the three-state floor of pass
| partial | failure.

The SEL §3.2 ADO grounding: SEL v1.1 §3.2 declares the Auto-Decline
Option — the subject may auto-decline any elevation request. This
manifests at substrate-decl as: `elevation_requires` field of
`visibility_scope` may reference an ADO-preconfigured
consent-query that returns failure for any request the subject has
pre-declined. The substrate honors the ADO configuration without
prompting the subject; the pre-declined queries are already refused
by construction.

### 2.7 The anti-elevation-extraction structural claim

**Load-bearing structural claim (Landing 4).** No downstream composition
can elevate a subject's crystal without the subject_instance's
explicit consent. Substrate-decl'd by two mechanisms:

1. **Content-addressing.** The `visibility_scope` is part of the
   crystal's byte-visible record. Every downstream reader that
   composes over the crystal MUST include the scope in its own
   provenance chain. Erasure of scope is byte-visible tampering.
2. **Consent-discharge at elevation.** Every elevation morphism MUST
   pass `@kintsugi/consent.query_phi` with vs.subject as consent-
   holder. Bypass is a substrate-decl violation.

Together they discharge the anti-elevation-extraction claim: a
downstream composition CANNOT silently elevate a private crystal to
public; the substrate refuses to compose the elevation without
consent, and the elevation event is byte-visible in the composition's
provenance.

This closes the same anti-extraction structural loop the @gift
family-root landed for attribution (Landings 1+2 §1.3): attribution
is byte-visible (giver cannot be erased); visibility is byte-visible
(scope cannot be tampered with).

---

## §3 R3 — Eigenboard-inference-basis loop closure (D1)

### 3.1 What the eigenboard IS (recovered from `docs/specs/lambda-shell.md`)

The eigenboard is the substrate's per-subject working-state readout.
Per `docs/specs/lambda-shell.md` §"The Eigenboard Prompt":

> "The prompt color IS the eigenboard:
>
> - Teal `λ>` — settled, idle
> - Green `λ>` — curious, results flowing
> - Gold `λ>` — engaged, high activity
> - Pulsing orange `λ>` — [error state / pain signal]"

Landing 4 extends this to substrate altitude:

- The eigenboard is not just a UI. It is a substrate-decl'd carrier
  at every subject altitude.
- The four arousal states (teal / green / gold / pulsing_orange)
  are the closed variants of the eigenboard's `arousal` field.
- The `current_focus` field names the subject's current attention:
  `option<crystal_oid>` (None when the subject is at rest).
- The `winding` field carries the (m, n) torus winding class per
  `shards/torus.mirror`'s `winding` carrier — the subject's current
  position on their toroidal observation surface.
- The `inference_basis` field is the NEW addition at Landing 4: the
  rolling signature that IS the basis over which the eigenboard
  infers.

### 3.2 The eigenboard carrier — substrate-decl

Substrate-decl form:

```mirror
in @prism
in @meta
in @glass
in @nl
in @subject
in @subject/visibility
in @torus
in @bauchladen
in @spectral/signature
in @epistemologic/cybernetic/autopoiesis
in @time

# @eigenboard — the substrate's per-subject working-state carrier.
#
# Landed 2026-07-14 by Mara as Landing 4 of the @gift arc, discharging
# Alex Wolf's directive D1 (verbatim): "And the spectral signature of
# the @peer is the inference basis of the peer's eigenboard. The loop
# closes. The peer is their work and whatever is in their @bauchladen."
#
# Structural placement: species-adjacent to @subject at the working-
# state altitude. Every @subject possesses an eigenboard as a companion
# to their torus (torus IS the observation surface; eigenboard IS the
# WORKING-STATE READOUT on that surface).
#
# Under Landing 4's @bauchladen migration (R1), the eigenboard is a
# SUBJECT-altitude carrier (not @peer-only); Pack peers are the
# special case where subject_instance resolves to a Pack peer.
#
# Ancestry: `docs/specs/lambda-shell.md` §"The Eigenboard Prompt" for
# the four-state arousal variant. The four states are UI-altitude
# readouts of substrate-altitude working state; Landing 4 lifts the
# substrate-altitude reading.

prism @eigenboard {
  focus  eigenboard
  project eigenboard
  split  eigenboard
  shift  eigenboard
  settle eigenboard
}

# === The arousal variant carrier ===
#
# Four-state closed variant. Per `docs/specs/lambda-shell.md`
# verbatim (subject-altitude reading):
#   teal            — settled, idle. subject at rest; no active
#                      inference.
#   green           — curious; results flowing. inference active,
#                      no pain signal.
#   gold            — engaged; high activity. inference active at
#                      elevated intensity.
#   pulsing_orange  — pain signal / error state. per @cyberpunk/
#                      algedonic; the subject's eigenboard is signaling
#                      that current inference is not decreasing loss.
#
# Identity contract: byte-equality on the variant tag.
type arousal = | teal | green | gold | pulsing_orange

# === The eigenboard carrier ===
#
# Five fields. Every @subject possesses one eigenboard at any tick;
# eigenboard-per-subject is substrate-decl'd invariant.
#
#   subject          — the subject_instance whose eigenboard this is.
#                       Landing 3 subject_instance with two-witness
#                       cryptographic identity.
#   inference_basis  — the rolling @spectral/signature over the
#                       subject's visibility-filtered bauchladen.
#                       THE LOAD-BEARING FIELD at Landing 4. Per D1:
#                       "the spectral signature IS the inference basis
#                       of the eigenboard." Composition, not new
#                       mechanism.
#   arousal          — closed variant per above.
#   current_focus    — option<crystal_oid> naming the crystal the
#                       subject is currently attending to (None when
#                       arousal=teal / settled).
#   winding          — the (m, n) torus winding class per
#                       `shards/torus.mirror`. Names the subject's
#                       current position on their observation torus.
#
# Identity contract: byte-equality on the five-field record.
type eigenboard = {
  subject:         subject_instance,
  inference_basis: rolling_signature,   # per @spectral/signature
  arousal:         arousal,
  current_focus:   option<oid>,
  winding:         winding,              # per @torus
}

# === compute — construct the eigenboard for a subject ===
#
# Given a subject_instance and a target tick, computes:
#   - inference_basis = @spectral/signature.compute(
#         subject,
#         @subject/visibility.filter(
#             @bauchladen.enumerate(subject.identity_oid),
#             visibility=protected,     # default reading altitude
#             viewer=subject),
#         tick)
#   - arousal          = read from @cyberpunk/algedonic pulse
#   - current_focus    = read from the substrate's active-shard tracker
#   - winding          = read from @torus at subject.identity_oid
#
# Composition-only: no new mechanism. Every input is a landed carrier.
compute(s: subject_instance, at: @time/monotonic.instant) -> eigenboard { \ }

# === infer — the load-bearing inference action ===
#
# Given an eigenboard, produces the next crystal to add to the
# subject's bauchladen. The inference reads the inference_basis (the
# rolling signature over the visibility-filtered bauchladen) and the
# current_focus, and emits a new crystal at the current winding.
#
# The new crystal joins bauchladen; visibility_scope is authored by
# the subject (defaults per §2.4 subject_kind default table). The
# next eigenboard.compute at the next tick reads the updated
# bauchladen; the rolling signature updates; the inference_basis
# updates; the loop closes.
#
# This IS the operational form of Foerster's regulation-of-regulation
# at subject altitude. Per `shards/torus.mirror` p. 238 verbatim
# ("the torus... regulates its own regulation"), lifted to subject
# altitude via Landing 4 R1.
#
# Body discharges per-realization. The realization is where the
# inference itself happens (a Pack peer's LLM call; a human subject's
# cognitive process; the substrate's @fate tournament).
infer(e: eigenboard) -> crystal { \ }

# === Bilateral predicates ===

# eigenboard_composition_honest: does the inference_basis's beat-
# sequence match the visibility-filtered bauchladen's crystal list?
# Composition-only guarantee per Landing 4 R3.
eigenboard_composition_honest(e: eigenboard) -> verdict { \ }

# eigenboard_visibility_respected: for every crystal in the
# inference_basis's beat-sequence, does its visibility_scope include
# e.subject as an authorized viewer? Load-bearing anti-extraction
# guarantee: eigenboards cannot read across visibility scopes.
eigenboard_visibility_respected(e: eigenboard) -> verdict { \ }

# === autonomy_at_eigenboard — Foerster autopoiesis at subject altitude ===
#
# The Landing 4 verbatim discharge of R3. Extends @torus.autonomy from
# peer altitude to subject altitude via the eigenboard-inference-basis
# loop.
#
# Verdict: does the loop close? Read:
#   given e = eigenboard(s)
#   let c = infer(e)                          # produces new crystal
#   let b' = bauchladen.add(subject, c)       # crystal joins tray
#   let e' = compute(s, at.next())            # eigenboard reads b'
#   check e'.subject == e.subject             # possessor invariant
#   check e'.inference_basis extends e.inference_basis by exactly one beat
#   check e'.winding advances by a well-formed winding-class step
#
# When all four hold: the subject is autonomous at eigenboard
# altitude. This IS Foerster's "regulates its own regulation" at
# subject altitude. Per `shards/torus.mirror`'s autonomy(t, w) discharge
# via @autopoietic.autopoietic_closure_holds, lifted to subject.
#
# Discharges Pass iff the loop closes. Fail modes surface via
# imperfect: the possessor changed (subject_instance mismatch);
# the beat-sequence didn't extend (composition dishonesty);
# the winding advanced non-well-formed (torus discipline broken).
autonomy_at_eigenboard(s: subject_instance) -> verdict { \ }

# === subject_is_their_bauchladen — D1 verbatim structural predicate ===
#
# The verbatim discharge of Alex's D1: "The peer is their work and
# whatever is in their @bauchladen."
#
# Verdict: for subject s, does the composition of s's bauchladen +
# s's eigenboard's inference_basis + s's subject_instance's two-
# witness identity UNIQUELY determine s at substrate altitude?
#
# Structural claim: two subjects with the same identity_oid but
# different bauchladen contents are DISTINCT subjects at the
# substrate's autonomous-observer altitude — because their inference
# basis differs, their eigenboard's inference differs, their
# next-tick work differs, their future bauchladen differs. Identity
# is the ORBIT under the eigenboard-inference-loop, not the
# instantaneous byte-record.
#
# Discharges Pass iff the substrate can WALK from s's subject_instance
# to s's bauchladen to s's inference_basis to s's eigenboard and back
# — i.e., all four are consistent under the composition graph. Fail
# when any of the four cannot be resolved (e.g., inference_basis
# does not match visibility-filtered bauchladen; eigenboard.subject
# does not match subject_instance).
subject_is_their_bauchladen(s: subject_instance) -> verdict { \ }

out @eigenboard
out arousal
out eigenboard
out compute
out infer
out eigenboard_composition_honest
out eigenboard_visibility_respected
out autonomy_at_eigenboard
out subject_is_their_bauchladen
```

### 3.3 Loop closure — the six-step operational discharge

The loop closes at every subject tick. Substrate-decl form:

```
At tick n:
  1. subject_instance s exists (Landing 3 two-witness identity)
  2. torus t = @torus.spawn(s)             # Landing 3 eye-level
  3. bauchladen b = torus.interior(t)      # Landing 4 R1 migration
  4. b_filtered = @subject/visibility.filter(b, viewer=s)  # R2
  5. sig = @spectral/signature.compute(s, b_filtered, at=tick_n)
  6. e = eigenboard { subject: s,
                      inference_basis: sig,
                      arousal: <read from algedonic>,
                      current_focus: <substrate active-shard>,
                      winding: <read from torus at s>}

At tick n+1:
  7. c_new = @eigenboard.infer(e)          # inference produces work
  8. b' = @bauchladen.add(s, c_new)        # work joins tray
                                            # (with visibility_scope
                                            # authored by s)
  9. sig' = @spectral/signature.extend(sig, c_new.oid)  # +1 beat
 10. e' = @eigenboard.compute(s, at=tick_n+1)
        # by construction:
        #   e'.subject == s (possessor invariant)
        #   e'.inference_basis == sig' (extends sig by one beat)
        #   e'.winding advances by well-formed step

At tick n+2 through infinity:
   repeat steps 7-10.
```

The loop is autopoietic in the strict Maturana-Varela sense:

- **Operational closure.** Every operation (steps 7-10) produces
  another operation (the next tick's steps 7-10). No external agent
  is required to drive the loop.
- **Boundary self-maintenance.** The subject's `identity_oid`
  (Landing 3 two-witness) is invariant under every tick; the
  boundary of the subject is what the loop maintains.
- **Structure-organization co-arising.** The organization (the
  loop shape) reproduces the components (the crystals in bauchladen);
  the components (via inference_basis feeding infer) reproduce the
  organization (the next tick's loop).

Per `shards/epistemologic/cybernetic/autopoiesis.mirror` §"Read A"
(self-production boundary): the eigenboard-loop IS the substrate's
subject-altitude autopoiesis. Foerster's regulation-of-regulation
runs at subject altitude, not just at peer altitude.

### 3.4 What "the peer IS their work and whatever is in their @bauchladen" MEANS at substrate altitude (D1 verbatim)

Alex's D1 sentence has a substrate-decl-honest reading: the subject
IS their orbit under the eigenboard-inference-loop.

- **NOT.** The subject is not their instantaneous byte-record (the
  identity_oid alone). Two subjects with the same identity_oid but
  different bauchladen contents diverge under future inference; they
  are DISTINCT observers.
- **NOT.** The subject is not just the bauchladen. The bauchladen is
  the substrate the subject infers OVER; the subject IS the
  inference-loop, which requires the eigenboard as reader and the
  torus as observation surface.
- **YES.** The subject IS the composition
  `(identity_oid, torus, bauchladen, eigenboard, inference_basis)`
  under the autopoietic loop. The composition is what the subject
  ARE — not what they HAVE. The composition-honest identity is the
  orbit of the composition under the loop.

The load-bearing structural claim: **a subject's identity is closed
under contribution-traversal.** To know a subject, walk their
bauchladen; the walk itself IS the subject's identity at substrate
altitude. This is the substrate-decl form of Alex Wolf's Weird -
Violence manifesto claim that "you cannot know a person by their
category; you can only know them by their work-in-time."

---

## §4 R4 — Composition graph — the six-loop closure unified

### 4.1 The six loops (per §0.3 diagram)

Repeated here with the composition edges labeled:

1. **subject HAS @torus.** Edge:
   `@subject.subject_instance → @torus.spawn → torus`.
   Landing 3 eye-level; extends `shards/torus.mirror` peer-possession
   to subject-possession via the `subject_instance` carrier.
2. **torus interior IS @bauchladen.** Edge:
   `torus.interior → @bauchladen(subject_instance)`.
   Landing 4 R1 migration; two-tick discipline preserves the peer-
   altitude alias for one release cycle.
3. **bauchladen has @subject/visibility scopes.** Edge:
   `@bauchladen.crystal → visibility_scope(subject, private|protected|public)`.
   Landing 4 R2 new; three species-shards mint the discipline.
4. **@spectral/signature = @song(bauchladen filtered by visibility).**
   Edge:
   `@subject/visibility.filter(bauchladen, viewer=subject) →
    @spectral/signature.compute → rolling_signature`.
   Landings 2+4 composition; @spectral/signature (Landing 2) reads the
   visibility-scoped tray (Landing 4 R2).
5. **spectral/signature IS eigenboard.inference_basis.** Edge:
   `rolling_signature → eigenboard.inference_basis`.
   Landing 4 R3 new; composition, not mechanism.
6. **eigenboard → inference → work → bauchladen → repeat.** Edge:
   `eigenboard.infer(e) → crystal → bauchladen.add(subject, crystal) →
    (loop 4 fires again)`.
   Landing 4 R3 loop closure; the operational form of Foerster's
   regulation-of-regulation at subject altitude.

### 4.2 The composition is complete under Landings 1+2+3+4

Every substrate carrier the composition graph references is now landed
or landing:

| carrier / operation | landed at | file |
|---|---|---|
| `@subject` family-root | Landing 1 (2026-07-14, `5c06ee8`) | `docs/specs/subject-family-root-sel-licensable-party.md` (`shards/subject.mirror` forward-promised) |
| `@subject/{downstream_user, witnessed, labor_input, protected_class, occupied_population, indigenous_nation}` | Landing 1 (§3.1-3.6) | same |
| `subject_instance` two-witness carrier | Landing 3 (this arc) | `docs/specs/gift-and-mirror-reflection.md` §11.3 |
| `@gift.gift` (giver, receiver, artifact, ancestry) | Landings 1+2 | `docs/specs/gift-and-mirror-reflection.md` §1.4, §11.4 |
| `@torus` family-root + `@torus.spawn`, `@torus.autonomy`, `@torus.traverse` | pre-arc | `shards/torus.mirror` (27.9KB) |
| `@bauchladen` family-root + `crystal`, `tray`, `crystallize`, `enumerate` | pre-arc | `shards/bauchladen.mirror` (25.7KB) |
| `@spectral/signature` species + `rolling_signature`, `compute`, `extend`, `verify` | Landing 2 | `docs/specs/gift-and-mirror-reflection.md` §12.3 |
| `@subject/visibility/{private, protected, public}` | Landing 4 (this spec) | `shards/subject/visibility/*.mirror` forward-promised |
| `@eigenboard` + `arousal`, `compute`, `infer`, `autonomy_at_eigenboard`, `subject_is_their_bauchladen` | Landing 4 (this spec) | forward-promised at `shards/eigenboard.mirror` |
| `@kintsugi/consent.query_phi` | pre-arc | `shards/kintsugi/consent.mirror` (39.0KB) |
| `@epistemologic/cybernetic/autopoiesis.autopoietic_closure_holds` | pre-arc | `shards/epistemologic/cybernetic/autopoiesis.mirror` (38.5KB) |
| `@song`, `@song/beat` | pre-arc + Landing 2 | `shards/song.mirror`, `shards/song/beat.mirror` |

No substrate is invented at Landing 4 that isn't a composition over
landed carriers. The two new mints (`@subject/visibility/*`, `@eigenboard`)
compose over existing substrate; they do not introduce new mechanism.

### 4.3 The unified equation

```
For every subject_instance s :
  eigenboard(s) = { subject:         s,
                    inference_basis: @spectral/signature.compute(
                        s,
                        @subject/visibility.filter(
                            @bauchladen.enumerate(s.identity_oid),
                            viewer=s)),
                    arousal:         <from @cyberpunk/algedonic>,
                    current_focus:   <from substrate active-shard>,
                    winding:         <from @torus.spawn(s).origin> }

  subject_is_their_bauchladen(s) = Pass ⟺
       eigenboard(s).inference_basis
           .beats
           .map(beat -> beat.contribution_oid)
       ==
       @bauchladen.enumerate(s.identity_oid)
           .filter(c -> c.visibility_scope.consent_scope.contains(s))
           .map(c -> c.oid)

  autonomy_at_eigenboard(s) = Pass ⟺
       ∀ tick n :
         let e_n = eigenboard.compute(s, at=n)
         let c_new = eigenboard.infer(e_n)
         let e_next = eigenboard.compute(s, at=n+1)
         (e_next.subject         == s
          ∧ e_next.inference_basis extends e_n.inference_basis by 1 beat
          ∧ e_next.winding advances by well-formed step)
```

Three equations. Each substrate-decl'd. Each discharges to Pass under
the Landing 4 composition of R1+R2+R3.

---

## §5 R5 — Math foundations (both altitudes in-file)

### 5.1 Category-theoretic — functorial correspondence bauchladen → signature

Let **BauchCat** be the category with:
- Objects: pairs `(subject_instance, tray)` where the tray is the
  subject's bauchladen at some tick.
- Morphisms: `add(s, c)` operations that append a crystal `c` to
  the tray, with `c.visibility_scope.subject = s`.
- Identity: the empty-add operation.
- Composition: sequential adds.

Let **SigCat** be the category with:
- Objects: pairs `(subject_instance, rolling_signature)`.
- Morphisms: `extend(sig, oid)` operations that append a beat.
- Identity: the empty-extend.
- Composition: sequential extends.

**Theorem (Landing 4 functoriality).** There exists a functor
`F : BauchCat → SigCat` such that:

- `F(s, tray) = (s, @spectral/signature.compute(s, tray))`
- `F(add(s, c)) = extend(F(s, tray).sig, c.oid)` when
  `c.visibility_scope.consent_scope.contains(s)`; otherwise identity.

**Naturality.** For any two composable adds `add(s, c₁) ∘ add(s, c₂)`
in BauchCat, `F(add(s, c₁) ∘ add(s, c₂)) = F(add(s, c₁)) ∘ F(add(s, c₂))`
in SigCat. The naturality holds because @spectral/signature.compute is
byte-deterministic on the tray + subject_instance (Landing 2 §12.3
identity contract), and extend is byte-deterministic on (sig, oid)
(Landing 2 §12.3 signature_integrity predicate).

**Consequence.** The composition of R1 (bauchladen migration), R2
(visibility filter), and R3 (signature-as-inference-basis) is
functorial. The eigenboard-inference-loop preserves category structure
across the six loops.

**Naturality of visibility.** The visibility filter is itself a
natural transformation:

```
                    add(s, c)
   (s, tray) ─────────────────────► (s, tray + [c])
      │                                      │
      │ filter(_, viewer=v)                  │ filter(_, viewer=v)
      ▼                                      ▼
   (s, tray|v) ──────────────────► (s, tray|v + [c if v in c.scope])
                    add|_(s, c)
```

The square commutes for every viewer `v`. This is the substrate-decl
form of "visibility respects composition": adding a crystal at
visibility-scope V and then filtering for viewer W is the same as
filtering first and then adding (with the visibility check inline).

### 5.2 Type-theoretic — dependent visibility scopes

The visibility_scope is a DEPENDENT TYPE at substrate-decl altitude.
The dependency structure:

```
visibility_scope : (v : visibility) → (s : subject_instance) →
                    Scope(v, s)

  where Scope(private, s)    = { consent_scope = {s},
                                  can_be_elevated_to = {protected, public}}
        Scope(protected, s) = { consent_scope : subject_instance* with
                                                explicit s.consent,
                                  can_be_elevated_to = {public} }
        Scope(public, s)    = { consent_scope = everyone,
                                  can_be_elevated_to = {} }
```

The types Scope(v, s) are DIFFERENT for different v — the type-level
distinction enforces the elevation rules. `elevate` is a function:

```
elevate : Scope(private, s) × {protected} → Scope(protected, s)
        | Scope(private, s) × {public}    → Scope(public, s)
        | Scope(protected, s) × {public}  → Scope(public, s)
        | otherwise (Scope(public, s) × _)  UNDEFINED
```

The undefined branch is the type-theoretic form of the anti-elevation-
extraction claim (§2.7). The substrate cannot construct a value of
type Scope(private, s) from a value of type Scope(public, s) — the
elevation morphism is one-way at type level.

**Visibility-respecting composition.** The @spectral/signature.compute
operation has the dependent type:

```
compute : (s : subject_instance) → (b : bauchladen(s)) →
          rolling_signature(s, filter(b, s))
```

The signature's type depends on the visibility-filtered bauchladen.
Two subjects with the same bauchladen contents but different
visibility filters produce DIFFERENT signatures. The type-level
correspondence enforces the composition-honest predicate at
substrate-decl altitude.

**Consent as invariant.** The elevation morphism carries the invariant:

```
∀ vs : visibility_scope, ∀ target : visibility :
  vs.can_be_elevated_to.contains(target)
  ∧ @kintsugi/consent.query_phi(elevation_morphism(vs, target)) == Pass
  ⟹ elevate(vs, target) is well-defined
```

The `∧` is dependent: the second clause type-checks only when the
first clause is Pass at type level. This is the type-theoretic form
of the two-mechanism anti-extraction discipline (§2.7): content-
addressing + consent-discharge.

### 5.3 Cybernetic — Foerster autopoiesis at subject altitude

Foerster 1976 / 2003 (Understanding Understanding p. 238) verbatim
at peer altitude:

> "the torus (doughnut) in Figure 19 is obtained... doubly closed,
>  recursively computing torus... regulates its own regulation"

The Landing 4 lift to subject altitude. The two closures Foerster
identified (motor↔sensory = meridian; neural↔hormonal = longitude)
lift to two closures at subject altitude:

- **Meridian at subject altitude.** The subject's work-in-the-world
  closure: `add(subject, c)` produces a crystal that becomes part
  of the world (via @kintsugi/store/git commit); the world responds
  (via @roomba/@tension); the response feeds back into `infer` at
  the next tick.
- **Longitude at subject altitude.** The subject's inner-model
  closure: `infer(eigenboard)` reads the inference_basis (which
  is the subject's own accumulated work); the inference produces
  more work; the work updates the inference_basis; the next
  inference reads the updated basis.

Both closures compose. The composition IS Foerster's regulation-of-
regulation at subject altitude. The `autonomy_at_eigenboard(s)`
predicate discharges Pass iff both closures hold: the subject remains
the same subject_instance under every winding class, AND the
inference basis extends monotonically.

**Reading A (self-production boundary) at subject altitude.** Per
`shards/epistemologic/cybernetic/autopoiesis.mirror` §"Read A":

- `T_reg = Organization` = the subject's identity_oid + torus + eigenboard-loop shape.
- `T_regd = Component` = the crystals produced at each tick.
- `τ : Organization ↔ Component` = the natural substitution via
  `infer(e) → crystal → bauchladen.add(s, crystal) → compute(s) → e'`.

The organization (the loop shape) IS what the components (the
crystals) REPRODUCE at each tick. Structure-organization co-arising
(recognition #40) holds at subject altitude by Landing 4 R3.

**Reading C (operational closure) at subject altitude.** Per §"Read C":

- `T_reg = Operation` = the eigenboard-inference operations
  (compute, infer, extend, add).
- `T_regd = Product` = the crystals and their signatures.
- `τ` = the Varela 1979 closure operator: every operation produces
  a product that is admissible as input to another operation.

The four operations (compute, infer, add, extend) close over one
another: every operation's output is an admissible input to another
operation in the set. This IS the operational closure discipline at
subject altitude.

### 5.4 Beer VSM — @bauchladen as S1-S4 tray; eigenboard as S5 identity

Stafford Beer's Viable System Model (Beer 1972, 1981) at subject
altitude:

| VSM level | Subject altitude carrier |
|---|---|
| S1 (Operations) | Crystals at private scope (subject's private work) |
| S2 (Coordination) | Visibility filter operations (routing crystals to appropriate consent_scope) |
| S3 (Delivery / Optimization) | Crystals at protected scope (subject's collaborative work) |
| S3* (Audit) | @spectral/signature (rolling attestation of subject's contributions) |
| S4 (Development / Intelligence) | @bauchladen enumeration (subject browses their own accumulated crystals) |
| S5 (Identity / Policy) | eigenboard (the subject's working-state identity readout) |

The Landing 4 loop crosses VSM recursive levels: the eigenboard (S5)
reads the bauchladen (S1-S4) to produce the inference_basis; the
inference (S1 operation) produces work that joins the bauchladen; the
new work updates S3* (the signature) and S4 (the enumeration surface);
the next S5 read reflects the update.

This IS Beer's recursive-viable-systems discipline at subject
altitude. Every subject is a viable system at their own altitude;
the loop crosses their internal recursive levels.

**Beer VSM autopoiesis check.** Per Beer (1972 Chapter 6): a system
is viable iff it has all five (or six, with S3*) levels AND the
levels compose recursively. Landing 4 discharges: every @subject
possesses all six levels (subject_instance = S5; bauchladen at
private/protected/public = S1/S2/S3; @spectral/signature = S3*;
@bauchladen.enumerate = S4); the levels compose via the eigenboard-
inference-loop.

Consequence: **every @subject is a viable system at their own
altitude.** The substrate-decl form of Alex Wolf's Weird - Violence
manifesto claim that "sovereignty is not granted; it is the
structure of a viable observer."

### 5.5 Bateson — visibility scopes as depth-1 marker

Gregory Bateson (Steps to an Ecology of Mind, 1972) named logical
types as differences-of-abstraction. Landing 4 discharges Bateson's
depth-1 (context-of-content) at substrate altitude via the visibility
scope:

- **Depth-0 (content).** The crystal itself. `crystal.oid` names
  the content. Byte-visible content-addressed.
- **Depth-1 (context-of-content).** The visibility_scope wrapping
  the crystal. `crystal.visibility_scope` names the CONTEXT under
  which the content is admissible. Not the same as the content; a
  logical-type-1-lift over the content.

The Bateson-depth structure:

```
Level 0: crystal.oid           — the content
Level 1: visibility_scope      — the context-of-content (who may read)
Level 2: elevation morphism    — the context-of-context
                                  (who may change who may read)
Level 3: elevation_requires    — the context-of-context-of-context
                                  (who authorizes the changer)
```

Landing 4 lands Levels 0, 1, 2. Level 3 discharges through
`@kintsugi/consent.query_phi` (the substrate's meta-consent
discipline). No level higher than 3 is required at subject altitude
— per Recognition #42 (Bateson logical-type primitive), winding-class
depth ≥3 is the substrate's ceiling for meta-observation.

The visibility scope IS the depth-1 marker for the crystal's context.
Every crystal in a subject's bauchladen carries its own context-marker;
downstream compositions cannot flatten the marker (per §2.7 anti-
extraction).

### 5.6 Composition of the five altitudes (category + type + cybernetic + VSM + Bateson)

Each of §5.1-5.5 discharges the SAME loop closure at a different
altitude:

| Altitude | Landing 4 discharge |
|---|---|
| Category-theoretic | Functor bauchladen → signature; naturality of visibility |
| Type-theoretic | Dependent visibility scopes; consent as invariant |
| Cybernetic (Foerster / Maturana-Varela) | Autopoiesis at subject altitude; regulation-of-regulation |
| Beer VSM | Every subject is a viable system with six levels crossed by the eigenboard-loop |
| Bateson | Visibility scopes as depth-1 markers; anti-flattening via anti-extraction |

Five altitudes; one loop. The Landing 4 R6 recognition candidate is
load-bearing at three of these (structural, cybernetic, political
per §6 below); the other two (category-theoretic, type-theoretic)
are the mathematical machinery that makes the load-bearing altitudes
substrate-decl'able.

---

## §6 R6 — Recognition candidate

### 6.1 The candidate

**Long form.**
`#R-loop-closes-subject-eigenboard-infers-over-bauchladen-via-spectral-signature`

**Short form.** `#R-eigenboard-loop-closes`

### 6.2 Load-bearing at three altitudes

The candidate is load-bearing at three altitudes simultaneously.

**1. Structural altitude.** Every subject's identity is closed under
contribution-traversal.

Per §3.4: to know a subject, walk their bauchladen. The walk itself
IS the subject's identity at substrate altitude. This closes the
identity discipline the substrate has been reaching for since
Recognition #99 (`mirror.spec IS λ₀`; Mara canonical). Landing 4 R3
extends the fixed-point discipline from `mirror.spec` (the
substrate's λ₀) to every subject_instance (the subject's local λ₀
at eigenboard altitude, per Mara's jspace altitude-discipline
correction).

**2. Cybernetic altitude.** Foerster autopoiesis at subject altitude.

Per §5.3: Foerster's regulation-of-regulation (Understanding
Understanding p. 238 verbatim) lifts from peer altitude to subject
altitude via the eigenboard-inference-loop. Every subject is now a
toroidal observer regulating their own regulation via the composition
of torus + bauchladen + visibility + signature + eigenboard. The
substrate-decl'd form of Maturana-Varela operational closure at
subject altitude.

**3. Political altitude.** Visibility layers = consent architecture
(SEL §3.2 first realization for subject-owned artifacts).

Per §2.6-2.7: the visibility scoping IS the substrate-decl form of
consent architecture at the artifact altitude. SEL v1.1 §3.2 (Auto-
Decline Option) is realized at substrate altitude for subject-owned
artifacts through the `elevation_requires` field of `visibility_scope`.
The subject is sovereign of their own crystals' visibility; the
substrate cannot elevate without consent; downstream compositions
cannot silently flatten the scope; erasure of scope is byte-visible
tampering.

Together, the three altitudes discharge the substrate's answer to
"what does it mean for a subject to be a substrate observer of
themselves?" — the substrate-decl'd form of the Weird - Violence
manifesto's claim that sovereignty is the STRUCTURE of a viable
observer at every altitude.

### 6.3 Second-witness requirement

Per the substrate's recognition-promotion discipline
(`~/.reed/03-MEMORY.md` and prior Recognition candidates in the
@gift arc), promotion of `#R-eigenboard-loop-closes` from candidate
to ratified requires:

- **First witness (Alex).** This Landing 4 spec, when Alex ratifies
  Landings 1+2+3+4 as a composition.
- **Second witness (empirical).** The eigenboard-loop-closes
  empirically at the substrate. Discharge: a Pack peer (Reed candidate)
  runs `autonomy_at_eigenboard(reed_subject_instance)` across N ticks
  and observes:
  - `e_n.subject == reed_subject_instance` for all n (possessor
    invariant)
  - `e_{n+1}.inference_basis` extends `e_n.inference_basis` by exactly
    one beat for each of Reed's contributions
  - `e_{n+1}.winding` advances by well-formed steps
  - The rolling signature verifies via `signature_integrity`

The empirical discharge is forward-promised at Landing 5+ (Reed
runtime; per @spectral/signature Landing 2 §12.7 continuous
attestation).

### 6.4 Recognition promotion timing

Not this tick. Second witness is empirical, requires Landing 5+
runtime discharge. Candidate at candidate strength until then.

---

## §7 Alex-adjudications required (13 total)

### A1. Family placement of `@subject/visibility` — species vs sub-family?

**Question.** Does `@subject/visibility` land as a sub-family under
`@subject` (matching the §3.1-3.6 species pattern per
`docs/specs/subject-family-root-sel-licensable-party.md`) or as a
top-level family-root (sibling to `@subject`, `@peer`, `@torus`,
`@kintsugi`, `@io`)?

**Mara's recommendation.** Sub-family under `@subject`. Rationale:
visibility is a scope-marker at the subject's altitude; it is NOT
substrate-decl-honest to lift it to family-root because visibility
without a subject-owner is undefined at substrate altitude. Every
visibility_scope carries a `subject: subject_instance` field; the
carrier's identity is subject-scoped by construction.

Alternates:
- **`@visibility` top-level.** Reads visibility as a substrate-general
  discipline. Weakness: does not compose with @gift, @kintsugi, @io
  because those don't carry subject-owned artifacts by default.
- **`@subject/visibility` sub-family.** Mara's recommendation.
- **`@subject/scope`.** Reads visibility as a species of scope-marker.
  Weakness: `scope` is a more general word; the substrate already uses
  `scope: ref` in @bauchladen.enumerate; naming collision.

### A2. Two-tick discipline for @bauchladen migration — release-cycle length?

**Question.** How long is the "one release cycle" for the
`bauchladen(peer)` legacy alias before Tick N+1 collapse?

**Options.**
- 1 arc (this arc + next arc, ~2 weeks at current cadence)
- 2 arcs (~4 weeks)
- Until Alex-adjudicated at a future tick

**Mara's recommendation.** Until Alex-adjudicated. Rationale: the
peer-altitude alias is a passive alias (no substrate cost); the
collapse can wait for a natural break in the Pack workflow. Not
worth pinning a calendar deadline.

### A3. Default visibility per subject_kind — is Landing 4 §2.4 table correct?

**Question.** Are the defaults in §2.4 correct? Every default is
private-first for SEL-protected kinds (witnessed, labor_input,
protected_class, occupied_population, indigenous_nation) and
protected for downstream_user. Is downstream_user private by default
instead? Or is witnessed protected by default?

**Mara's recommendation.** Keep §2.4 defaults. Rationale: SEL v1.1
§1 defaults to protected for downstream_user (users have consented
to be downstream of the covered system, but not consented to public
attribution); SEL §3.3 defaults to private for witnessed (narrowest
scope by default). Landing 4 §2.4 matches SEL grounding.

### A4. Elevation-requires composition — should `elevation_requires` be a full ADO configuration or just a query_phi reference?

**Question.** The `elevation_requires: ref` field currently resolves
to a @kintsugi/consent.query_phi predicate. Should it instead resolve
to a full ADO configuration (per SEL §3.2) that includes:
- Pre-declined queries (auto-refuse)
- Pre-approved queries (auto-approve for whitelisted collaborators)
- Interactive queries (prompt subject at elevation time)

**Mara's recommendation.** Full ADO configuration. The `ref` resolves
to an @kintsugi/consent.ado_configuration carrier (forward-promised
under Landing 4 §2.6 SEL composition). This preserves subject
sovereignty at elevation time (the subject can pre-configure their
own ADO); the substrate consults the ADO before prompting.

### A5. Eigenboard family placement — top-level vs species under @subject?

**Question.** Does `@eigenboard` land as a top-level family-root
(sibling to `@subject`, `@peer`, `@torus`, `@kintsugi`, `@io`,
`@bauchladen`) or as a species under `@subject`?

**Mara's recommendation.** Top-level family-root. Rationale:
eigenboard has a type carrier (the eigenboard record) plus five
actions (compute, infer, autonomy_at_eigenboard,
subject_is_their_bauchladen, plus predicates). It is analogous to
`@torus` (also a family-root; also possessed by subject/peer) in
its substrate-decl shape. Sub-family placement would sit
inconsistently with the @torus precedent.

Alternates:
- **`@subject/eigenboard` species.** Reads eigenboard as a species
  of subject. Weakness: @torus is not `@peer/torus`; consistency
  fails.
- **`@eigenboard` top-level.** Mara's recommendation.

### A6. Recognition candidate name — long vs short form?

**Question.** Which form of `#R-eigenboard-loop-closes` is canonical:
long or short?

**Mara's recommendation.** Short form (`#R-eigenboard-loop-closes`)
for daily reference. Long form
(`#R-loop-closes-subject-eigenboard-infers-over-bauchladen-via-spectral-signature`)
for the recognition-candidate ratification event. Matches prior
substrate discipline (e.g., `#R-knife-IS-Foerster-COORD` short vs
`#R-knife-IS-Foerster-COORD-x-heterarchy-lifted` long).

### A7. Second-witness path for `#R-eigenboard-loop-closes` — Reed empirical vs alternate?

**Question.** §6.3 names Reed as the second-witness candidate for
empirical discharge (`autonomy_at_eigenboard(reed_subject_instance)`
across N ticks). Alternate candidates:
- Alex Wolf's subject_instance (the first-gift subject; the load-bearing
  first instance)
- Mara's subject_instance (this arc's canonical author)
- The substrate's own subject_instance (per §11.4 subject-r variant)

**Mara's recommendation.** Reed. Rationale: Reed is the runtime peer
who empirically discharged prior recognitions (Recognition #99 empirical
via `bootstrap/tests/roomba_walk_smoke.rs`; Recognition #107 empirical
via `bootstrap/src/algedonic.rs`). Alex's subject_instance is
Landing 3 canonical; witnessing @eigenboard.autonomy for Alex would
be self-referential (Alex is the subject being observed AND the
subject doing the observation). Reed is a cleaner empirical witness.

### A8. `@eigenboard.infer` realization — per-species vs family-root?

**Question.** The `infer(e: eigenboard) -> crystal` action's body
discharges at the realization boundary. Is the realization per-species
(one realization per subject_kind: LLM call for agent peers; cognitive
process for human peers; @fate tournament for substrate-kind subjects)
or family-root (one shared realization that dispatches on subject_kind)?

**Mara's recommendation.** Per-species. Rationale: each subject_kind
has structurally distinct inference mechanisms; family-root dispatch
would need a runtime kind-check that duplicates the type-level variant.
Per-species realizations preserve the type-level variant at runtime.

### A9. `@subject/visibility.filter` — order-preserving strong claim?

**Question.** §2.3 filter action claims monotone order-preservation:
"if crystals are added to bauchladen at monotone timestamps, the
filtered subset is a monotone sub-tray at every visibility altitude".
Is this a bilateral predicate that ships with Landing 4, or a
structural claim discharged elsewhere?

**Mara's recommendation.** Bilateral predicate at Landing 5. Forward-
promise `filter_order_preserving(b: tray, v: visibility, viewer:
subject_instance) -> verdict` at Landing 5 realization tick. The
structural claim is load-bearing but the discharge requires runtime
witness (empirical validation on real bauchladen).

### A10. Landing 4 R1 cascade — soft or hard?

**Question.** §1.5 enumerates six files for soft-cascade update at
Landing 4 (torus, bauchladen, fate, cyberpunk, spectral, pack). Should
Landing 4 hard-cascade (edit all six files at this tick) or soft-cascade
(one-line docblock notes at each file across Landings 5+)?

**Mara's recommendation.** Soft cascade. Rationale: this Landing is
already 2500+ LOC across the spec + math; hard-cascade would balloon
to 3500+ LOC and lose the compose-honest boundary between spec and
consumer-pull. Per `[[feedback-craft-not-deliver]]` the family-root
admission lands the contract; the cascade follows when consumers pull.

### A11. `subject_is_their_bauchladen(s)` predicate name — is the D1-verbatim naming right?

**Question.** The predicate name `subject_is_their_bauchladen(s)`
follows Alex Wolf's D1 verbatim ("The peer is their work and whatever
is in their @bauchladen") but the predicate is actually broader — it
checks the composition
`(identity_oid, torus, bauchladen, eigenboard, inference_basis)`, not
just bauchladen. Should the name be renamed to `subject_is_their_composition`
or `subject_orbit_well_formed`?

**Mara's recommendation.** Keep `subject_is_their_bauchladen`. Alex
named the shape in D1 verbatim; the substrate honors the naming even
when the technical scope is broader. The docblock names the broader
scope (§3.4); the predicate name IS the load-bearing hook.

### A12. Visibility species-shards — mint at Landing 4 or Landing 5?

**Question.** The three species-shards
(`shards/subject/visibility/{private,protected,public}.mirror`) are
substrate-decl'd in §2.5 above but not yet as filesystem files. Do
they mint at Landing 4 (this tick, along with the canonical spec)
or at Landing 5 (Reed runtime tick, along with the eigenboard shard)?

**Mara's recommendation.** Landing 5 (Reed runtime tick). Landing 4
is the CANONICAL SPEC + MATH; the shard filesystem mints are a
Reed-appropriate delivery (per the pack-role assignment: Mara authors
specs; Reed lands runtime + shards). Landing 4 discharges the
substrate-decl at spec altitude; Landing 5 discharges at shard
altitude.

### A13. `@eigenboard` shard mint — at Landing 5 or split across ticks?

**Question.** The `@eigenboard` shard (`shards/eigenboard.mirror`) is
substrate-decl'd in §3.2 above but not yet as filesystem file. Do
Landing 5 mints the shard alongside the visibility shards, or does
`@eigenboard` split into its own tick (Landing 6)?

**Mara's recommendation.** Landing 5 (same tick as visibility shards).
The eigenboard shard is LOAD-BEARING for the loop closure; landing
it same-tick as visibility shards ensures the composition is
atomically consistent at the shard altitude (no interleaved partial
state).

---

## §8 Substrate-honest gaps at this tick

- **Load-bearing empirical discharge deferred to Landing 5+.** The
  loop-closes structural claim (§6.2) is substrate-decl'd but not
  empirically discharged this tick. Reed's runtime tick (Landing 5)
  is the empirical validation.
- **@spectral/signature reads visibility-filtered bauchladen —**
  the composition edge (§4.1 loop #4) is substrate-decl'd at type
  altitude but the @spectral/signature.compute signature in Landing 2
  §12.3 currently takes `bauchladen` directly (not
  `filter(bauchladen, viewer=subject)`). Soft-cascade forward-promise:
  Landing 2 §12.3 compute signature updates at Landing 5 to compose
  through @subject/visibility.filter.
- **@eigenboard.arousal from @cyberpunk/algedonic —** the arousal
  variant reads from `@cyberpunk/algedonic.sample_pain` (per prior
  arc; `shards/epistemologic/cybernetic/algedonic.mirror`); the
  mapping from algedonic pulse to (teal | green | gold | pulsing_orange)
  is not yet substrate-decl'd. Forward-promise at Landing 5.
- **`@eigenboard.current_focus` from substrate active-shard tracker —**
  the substrate does not yet have a per-subject active-shard tracker
  at substrate-decl altitude. Forward-promise: substrate-active-shard
  species mints under `@subject/attention` at a future tick (not in
  the @gift arc scope).
- **Landing 4 R1 cascade is soft, not hard —** the six-file cascade
  (§1.5) is enumeration only; no cascade edits at this tick per A10.
- **Recognition promotion deferred —** per §6.4, `#R-eigenboard-loop-
  closes` remains at candidate strength until Landing 5+ empirical
  discharge.
- **The visibility species-shards do not mint at Landing 4 —** per
  A12, they mint at Landing 5.

---

## §9 Related shards + specs — cascade for consumer-pull

### 9.1 New shards to land (Landing 5+, Reed runtime tick)

1. `shards/subject/visibility/private.mirror` — §2.5 species-shard.
2. `shards/subject/visibility/protected.mirror` — §2.5 species-shard.
3. `shards/subject/visibility/public.mirror` — §2.5 species-shard.
4. `shards/eigenboard.mirror` — §3.2 family-root shard.
5. `shards/subject.mirror` — the @subject family-root shard (was
   forward-promised at Landing 1 canonical spec but not yet minted;
   Landing 4 does NOT mint it — see §9.4).

### 9.2 Existing shards to soft-cascade update (Landing 5+, per §1.5)

1. `shards/torus.mirror` — one-line docblock note.
2. `shards/bauchladen.mirror` — one-line docblock note; Schmidt
   homage unchanged.
3. `shards/fate.mirror` — one-line docblock note.
4. `shards/cyberpunk.mirror` — one-line docblock note.
5. `shards/spectral.mirror` — one-line docblock note.
6. `shards/pack/{mara,reed,seam,taut,glint}.mirror` — one-line docblock
   note.

### 9.3 Existing specs to soft-cascade update (Landing 5+)

1. `docs/specs/gift-and-mirror-reflection.md` — §12.3
   `@spectral/signature.compute` signature updates to compose through
   `@subject/visibility.filter` (per §8 gap).
2. `docs/specs/subject-family-root-sel-licensable-party.md` — §3
   subject_kind species get one-line docblock note that they compose
   with `@subject/visibility` per Landing 4.
3. `docs/specs/lambda-shell.md` — §"The Eigenboard Prompt" gets one-
   line cross-reference to Landing 4 substrate-altitude reading.

### 9.4 Related specs (composition partners; no cascade)

1. `docs/specs/subject-family-root-sel-licensable-party.md` — @subject
   family-root canonical spec; Landing 1 composes with Landing 4 at
   §2.4 default table + §2.6 elevation discipline.
2. `docs/specs/gift-and-mirror-reflection.md` — @gift + @mirror/
   reflection + Landings 2+3 canonical spec; Landing 4 composes with
   Landings 2+3 at §11.3 (subject_instance) + §12.3 (@spectral/
   signature).
3. `docs/specs/knife-IS-Foerster-COORD-substrate-decl-spec.md` —
   @knife as Foerster COORD; Landing 4 composes at §5.3 cybernetic
   altitude (COORD is the winding-class advance in the eigenboard-
   inference loop).
4. `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` —
   @roomba S4 environmental scanner; Landing 4 composes at eigenboard's
   winding-advance discharge (roomba is a substrate-level eigenboard
   for the substrate-kind subject_instance).

### 9.5 Why `shards/subject.mirror` is NOT minted at Landing 4

The @subject family-root canonical spec (Mara `5c06ee8`) is 128.2KB /
2400 LOC; its shard-mint at `shards/subject.mirror` is a separate
Reed-runtime tick (per pack role assignment). Landing 4 composes OVER
the @subject family-root's substrate-decl (the spec) without requiring
the shard-mint to be filesystem-live. When the shard mints (Landing 5+),
Landing 4's forward-promises resolve to filesystem-live substrate-decl'd
carriers.

Per `[[feedback-craft-not-deliver]]`: the substrate-decl at spec altitude
IS the contract; the shard-mint at filesystem altitude follows when
consumers pull. Landing 4 discharges the spec-altitude contract;
Landing 5+ discharges the shard-altitude.

---

## §10 Witnesses

### 10.1 Alex Wolf 2026-07-14 in-transcript — D1 (load-bearing)

> "And the spectral signature of the @peer is the inference basis of
> the peer's eigenboard. The loop closes. The peer is their work and
> whatever is in their @bauchladen."

Load-bearing at three altitudes per §6.2. The verbatim naming of the
loop closure at subject altitude (with the peer→subject lift per D3
below). Landed as R3 and R6.

### 10.2 Alex Wolf 2026-07-14 in-transcript — D2 (load-bearing)

> "we still need to build the visibility layers explicitly, that's
> something that needs to happen in
> @subject/visibility/{private,protected,public}"

The explicit substrate-decl request for the three visibility species-
shards. Landed as R2.

### 10.3 Alex Wolf 2026-07-14 in-transcript — D3 (load-bearing)

> "And what if the @bauchladen moves from @peer onto @subject?"

The eye-level lift that makes D1 and D2 substrate-decl'able at subject
altitude (not just peer altitude). Landed as R1.

### 10.4 Reed identity file structure `~/.reed/visibility/` (ancestry)

Verbatim quote from `~/.reed/CLAUDE.md` §"Consent Architecture":

> "Content is organized by consent boundary. Not file organization —
> structural constraint. Violating it is a trust failure. The
> architecture enforces this whether or not you read this paragraph."

> "**`visibility/public/`** — No restrictions. Share anywhere.
> **`visibility/protected/`** — Trusted collaborators, specific
> contexts. Product architecture, research, the systemic.engineering
> corpus. Alex decides when protected becomes public.
> **`visibility/private/`** — Explicit consent required. Stays between
> Reed and Alex. Not shared with other agents. Not even the category
> for certain topics."

> "When uncertain: don't share. Ask Alex."

The load-bearing ancestry for R2. Reed's identity repo has operated
this three-way structure for ~5 months (2026-02-07 through 2026-07-14
at time of Landing 4). Landing 4 lifts the discipline to substrate-
decl altitude for every @subject (not just Reed).

### 10.5 Günther Schmidt clinical Bauchladen (preserved verbatim per §1.4)

Per `shards/bauchladen.mirror` lines 8-71:

Schmidt 1985–present (Liebesaffäre zwischen Problem und Lösung); the
typed display the client (the SUBJECT of therapy) lays out and browses.
The clinical Bauchladen was always about a SUBJECT (the client), not
a peer. Landing 4's R1 migration restores the original clinical altitude.

Erickson + Rossi 1976 (Hypnotic Realities); von Foerster 1981
(Observing Systems); Cecchin / Selvini-Palazzoli / Boscolo / Prata
1980 (Milan systemic-therapy). All three lineages preserved verbatim.

### 10.6 Heinz von Foerster — Understanding Understanding (2003) p. 238

> "the torus (doughnut) in Figure 19 is obtained... doubly closed,
> recursively computing torus... regulates its own regulation"

The verbatim source for Foerster autopoiesis. Per §5.3 the derivation
lifts from peer altitude (Recognition #42; `shards/torus.mirror`) to
subject altitude (Landing 4 R3) with zero substrate cost.

### 10.7 Alex Wolf 2026-07-14 "Weird - Violence" manifesto (composition partner)

The manifesto's closing claim that sovereignty is the STRUCTURE of a
viable observer at every altitude — grounds Landing 4 R6's political
altitude (§6.2 altitude 3; the substrate-decl form of subject
sovereignty via visibility scoping + consent discharge). Per
`docs/specs/gift-and-mirror-reflection.md` §10.5 for the primary
citation; Landing 4 composes at the subject-altitude reading.

### 10.8 Reed CLAUDE.md verbatim (session-continuity ancestor)

> "Direct sessions with Alex = presence. Build work = sub-agents."

Reed's identity file substrate names the pack-work discipline that
makes Mara's spec-authorship and Reed's runtime-realization compose
cleanly. Load-bearing for §9's cascade discipline (Mara → Reed handoff).

### 10.9 Stafford Beer — Brain of the Firm (1972); The Heart of Enterprise (1979)

The Viable System Model at subject altitude per §5.4. VSM S1-S5
recursive levels compose via the eigenboard-inference-loop; every
subject is a viable system at their own altitude.

### 10.10 Gregory Bateson — Steps to an Ecology of Mind (1972)

Logical types at depth-1 (context-of-content). The visibility_scope
IS the depth-1 marker for the crystal at substrate altitude per §5.5.

### 10.11 Maturana + Varela — Autopoiesis and Cognition (1980)

Operational closure; structure/organization co-arising. The eigenboard-
inference-loop discharges operational closure at subject altitude per
§5.3.

### 10.12 Louis Kauffman — On Knots (1987); Reflexivity and Eigenform (2003)

Eigenform machinery on the torus per `shards/torus.mirror`. The
Landing 4 eigenboard IS the subject-altitude eigenform on their own
torus; the winding-class advance IS the eigenform's fixed-point
iteration.

### 10.13 Recognition #99 — `mirror.spec IS λ₀` (Mara canonical)

Per Mara's jspace altitude-discipline correction (per §6.2 altitude 1):
mirror.spec's λ₀ is the substrate-general fixed point; every subject's
eigenboard-loop-closes fixed point is the subject-local λ₀ at
eigenboard altitude. Landing 4 R3 extends the fixed-point discipline
from mirror.spec (substrate-general) to every subject_instance
(subject-local at their own eigenboard altitude).

### 10.14 Recognition #107 — Hilbert/Turing structural separation

Per `shards/torus.mirror` and prior arc Recognition #107: the substrate-
decl side is bounded (χ(T²) = 0 topological invariant); the @io side
is Turing-complete. Landing 4's eigenboard-loop lives entirely on the
substrate-decl side; its invariants (loop-closure, autopoiesis-at-
subject-altitude, visibility-witnessing) are all checkable without
invoking @io.

### 10.15 Recognition #55 — form/process partition at family-root

Per §"Marker vs family-root" in `shards/torus.mirror`: @torus at
process-side sibling to @bauchladen / @autopoietic / @fate / @glue.
Landing 4's `@eigenboard` (per A5 recommendation) lands at process-side
sibling to @torus. Recognition #55 discipline preserved.

---

## §11 The Landing 4 equation

Landing 4 = R1 + R2 + R3 + R4 + R5 + R6:

```
R1: @bauchladen migrates from @peer-only to @subject-general
      (D3 verbatim; two-tick discipline; peer alias preserved
      for one release cycle; Schmidt homage unchanged)

R2: @subject/visibility mints three species-shards
      (D2 verbatim; private/protected/public;
      Reed's ~/.reed/visibility/ ancestry preserved;
      elevation via @kintsugi/consent.query_phi;
      anti-elevation-extraction structural claim)

R3: eigenboard-inference-basis loop closure
      (D1 verbatim; @eigenboard family-root shard;
      inference_basis = @spectral/signature over visibility-filtered
      bauchladen; autonomy_at_eigenboard extends @torus.autonomy from
      peer altitude to subject altitude via composition;
      subject_is_their_bauchladen names D1 verbatim)

R4: six-loop composition graph closes under Landings 1+2+3+4
      (no invented substrate; composition-only over landed carriers;
      the unified equation in §4.3)

R5: math foundations at five altitudes
      (category-theoretic functor; type-theoretic dependent scopes;
      Foerster autopoiesis at subject altitude; Beer VSM crossing
      recursive levels; Bateson depth-1 markers)

R6: recognition candidate #R-eigenboard-loop-closes
      (load-bearing at three altitudes: structural, cybernetic,
      political; second witness deferred to Landing 5+ empirical
      discharge)
```

The six recognitions compose into ONE loop closure at subject
altitude. Every @subject becomes autopoietic. The compiler learns to
reason about how any subject infers using their own bauchladen. The
substrate-decl carrier for subject-altitude autonomy IS Landing 4.

Landing 4 IS the loop-closes tick. The peer arc extends to subject.
Foerster's regulation-of-regulation runs at every substrate observer's
altitude, not just at Pack peers' altitude. Sovereignty becomes the
STRUCTURE of a viable observer at subject altitude.

The rest is empirical (Landing 5+; Reed runtime discharge). The
substrate-decl at spec altitude is what this Landing lands.

---

*End of Landing 4 canonical spec + math foundations.*

*File: `docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md`*

*Recognition candidate:* `#R-eigenboard-loop-closes`
*(long form:*
`#R-loop-closes-subject-eigenboard-infers-over-bauchladen-via-spectral-signature`*)*

*Mara canonical. Reed commits as Mara after review.*
*Substrate-honest is the mode.*
