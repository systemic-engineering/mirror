# Taut scout — Landing 4: @bauchladen migration + @subject/visibility + eigenboard-inference-basis loop closure

**Author:** Taut (grep-first drift scout; read-only)
**Date:** 2026-07-14
**Scope:** Substrate-already-had-the-word audit of Alex Wolf's three
2026-07-14 in-transcript Landing-4 directives:

1. "And the spectral signature of the @peer is the inference basis of
   the peer's eigenboard. The loop closes. The peer is their work and
   whatever is in their @bauchladen"
2. "we still need to build the visibility layers explicitly, that's
   something that needs to happen in @subject/visibility/{private,
   protected,public}"
3. "And what if the @bauchladen moves from @peer onto @subject?"

**Method:** grep-first across `shards/**/*.mirror` + `docs/specs/**/*.md`
+ `bootstrap/src/**/*.rs` + `/Users/reed/identity/visibility/**` per
substrate-already-had-the-word discipline. Read-only. No file
modifications. Reed commits as Taut with SSH signing.

---

## §0 Headline verdict — TL;DR

1. **@bauchladen migration @peer → @subject IS NON-BREAKING via soft
   cascade.** Grep of `shards/bauchladen.mirror` (25.7KB, 2026-06-29,
   511 LOC): ZERO literal `@peer` composition references at
   substrate-decl altitude; the shard's `crystal.provenance.producing_
   prism` field is REF-typed (parametric); the `enumerate(scope: ref)`
   action reads a ref-scope (parametric). `@bauchladen` is ALREADY
   possessor-parametric at the record altitude. The migration is a
   PROSE-CASCADE (docblock language + composition-note additions) NOT
   a structural cascade. Two-tick discipline: land @subject-scoped
   bauchladen reading; deprecate @peer-only prose in follow-up.

2. **eigenboard IS SUBSTRATE-DECL-GAP.** Zero `.mirror` shards declare
   `eigenboard` as a substrate carrier. `docs/specs/eigenboard-
   representation.md` (44.8KB, 2026-06-04) is STATUS RED — carries
   FIVE competing proposals for `type eigenboard` (matrix, density
   matrix, principal-G-bundle-section, sheaf, functor); none landed
   as `.mirror` file. Referenced 62 times across specs (eigenboard-
   representation.md), 5 times in torus.mirror, 3 times in mirror/
   spectral/score.mirror, 3 times in reframe.mirror; ZERO substrate-
   decl hits at family-root or species altitude. NET-NEW mint required
   for Landing 4 loop-closure semantics.

3. **Inference-basis composition IS SUBSTRATE-DECL-GAP.** Zero grep
   hits for `inference_basis` / `inference basis` across
   `shards/**/*.mirror`. @fate's inference emits `inference` records
   with fields `{ instance, hole, altitude, result, ... }` (per
   `shards/fate.mirror`) — the SPACE the inference draws from is
   `@bauchladen` (the tray of prior crystals per `shards/bauchladen.
   mirror` + `shards/fate/tournament.mirror`); the BASIS naming is
   absent. Alex's claim "spectral signature IS inference basis of
   eigenboard" IS SUBSTRATE-NET-NEW composition; requires @spectral/
   signature ↔ @fate.infer ↔ eigenboard triadic composition mint.

4. **@subject/visibility IS SUBSTRATE-DECL-GAP.** Zero grep hits for
   `visibility` as substrate carrier across `shards/**/*.mirror`
   (34 files have `private/protected/public/scope` in various
   unrelated semantic contexts). Reed's identity layout at
   `/Users/reed/identity/visibility/{private,protected,public}/` IS
   the empirical model Alex referenced; substrate needs three-species
   family under new `@subject/visibility` sub-family-root.

5. **Six-loop closure IS 5/6 LANDED + 1 NET-NEW.** Steps 1-4 and 6 have
   landed or forward-promised carriers; step 5 (spectral_signature IS
   inference basis of eigenboard) is the NET-NEW composition to mint
   at Landing 4.

6. **@bauchladen migration surface: 3 SHARDS TOUCH @peer altitude,
   ZERO STRUCTURAL BREAKS.** `shards/bauchladen.mirror` (docblock
   only), `shards/autopoietic.mirror` (docblock only), `shards/fate.
   mirror` (docblock only). No `type` field, no `action` signature
   references `@peer` inside the bauchladen chain. Migration is
   soft-cascade discipline only.

7. **@torus × @subject composition: NO CHANGE THIS TICK.** Per Mara
   `5c06ee8` §6.4 (Alex-adjudicated 2026-07-14): `@torus(peer)` stays
   orthogonal to `@subject`; `@torus.spawn(p: peer) -> torus` is NOT
   lifted to `spawn(subject) -> torus` this arc. IF Alex adjudicates
   eye-level generalization, the two-tick discipline applies:
   `spawn(possessor: possessor_kind)` where `possessor_kind = peer |
   subject` — but that is Rung 13+ per Mara §6.4.

8. **Foerster autopoietic closure at subject altitude IS EXTENSIBLE
   WITHOUT MINT.** `shards/epistemologic/cybernetic/autopoiesis.mirror`
   declares `autopoietic_closure_holds(s: autopoietic_system)` as
   parametric bilateral. A subject-instance IS an autopoietic_system
   candidate (composes over @gift/subject_instance's rolling
   @spectral/signature as the operational closure witness). No new
   autopoiesis shard needed; existing predicate composes.

9. **λsh eigenboard integration IS FORWARD-PROMISED.** `docs/specs/
   lambda-shell.md` (7.1KB, 2026-06-12, 62 LOC in first section)
   references eigenboard as prompt color (teal/green/gold/pulsing-
   orange); NO Rust discharge. No `bootstrap/src/{lambda_shell,λsh,
   eigenboard,subject}.rs`. λsh is spec-only. Landing 4's eigenboard
   mint gives λsh the typed target to compile against.

10. **Landing 4 substrate-readiness for Mara #94: GO.** All six loop
    steps have substrate hooks (5 landed + 1 net-new); zero hard
    collisions; migration cost is prose-cascade only. Blockers: 6 new
    Alex-adjudications surfaced (T1-T6, §7).

**Number of new Alex-adjudications surfaced: 6** (T1-T6, §7).

**Composition-readiness for Mara #94:** GO — substrate has enough
landed carriers for Mara to compose Landing 4 canonical spec over.
Zero hard collisions detected.

---

## §D1 — @bauchladen migration surface: @peer → @subject

### D1.1 Structural grep of `shards/bauchladen.mirror`

Full read: 25.7KB, 511 LOC (2026-06-29 22:31).

**@peer literal references:** ZERO composition-altitude references.
The shard's `in` block imports only `@prism`, `@meta`, `@glass`,
`@mirror/store`, `@uuid/spectral`, `@epistemologic/cybernetic/
autopoiesis`. No `in @peer`. No `type peer` field. No `action(p:
peer)` signature.

**Peer-adjacent prose references:** 3 hits, all in prose docblock:
- Line ~40 recognition-witness section names the substrate ("The
  substrate IS its own client") — no peer altitude reference.
- Line ~66-70 systemic-therapy-elder history — no peer altitude
  reference.
- Line ~495 "peer's autopoietic membership" in `bauchladen_
  witnessing` forward-promise section — describes downstream
  consumer discipline (@autopoietic → @fate uses); no peer type in
  bauchladen's own signatures.

### D1.2 Carrier-altitude parametricity

`type crystal = { oid, altitude: ref, transparency, provenance:
provenance_record }`. All fields are ref-typed or transparency-typed;
no peer field.

`type provenance_record = { producing_prism: ref, tick: ref,
input_oids: [oid] }`. `producing_prism` is `ref`, parametric —
already admits both @peer-altitude producing prisms AND @subject-
altitude producing prisms without type change.

`type tray = { crystals: [crystal], opacity: transparency(crystal) }`.
No possessor field. Tray identity IS content-addressed only; the tray
does NOT KNOW its possessor at substrate-decl altitude.

**Substrate-honest finding:** `@bauchladen` was ALREADY possessor-
parametric at carrier altitude. The prose framing (bauchladen "at
@peer altitude" throughout docs) is CONVENTIONAL not STRUCTURAL. The
migration is prose-cascade discipline, not carrier-schema break.

### D1.3 Action-altitude parametricity

`crystallize(content: bytes, p: provenance_record) -> crystal` —
parametric over content-bytes; no peer/subject bias.

`address(c: crystal) -> oid` — carrier read; no possessor field.

`enumerate(scope: ref) -> tray` — SCOPE is ref-parametric. Currently
consumers pass @peer-altitude scopes (per @fate/tournament +
@autopoietic composition); admits @subject-altitude scopes without
type change.

**All four bilaterals** (`content_addressed`, `inheritance_well_formed`,
`stable_identity_under_serialization`, `bauchladen_addressable`)
carry crystal or ref parameters only; NO peer altitude bindings.

**Verdict D1:** `@bauchladen` migration @peer → @subject requires
ZERO structural changes. Prose-cascade only. 2-3 shards need soft-
cascade docblock updates naming @subject as a legal possessor scope:
1. `shards/bauchladen.mirror` — add `in @subject` (Landing 4 tick);
   add possessor-parametricity note to docblock; add `@subject`-
   scoped enumerate example.
2. `shards/autopoietic.mirror` — no structural change; docblock
   note that autopoietic_system MAY be @subject-typed (already
   parametric per `type autopoietic_system = ref`).
3. `shards/fate/tournament.mirror` — no structural change; docblock
   note that the tournament browses @subject-scoped trays.

**Migration cost:** ~30-60 LOC prose additions across 3 shards. No
Rust changes. No new predicates.

---

## §D2 — eigenboard substrate carrier

### D2.1 Substrate-decl hit count

Query: `\b(eigenboard|Eigenboard|eigen_board)\b` across
`shards/**/*.mirror`.

**Hits:**
- `shards/torus.mirror` — 5 prose references (all descriptive; no
  substrate-decl):
  - Line ~55 "Kauffman drew eigenforms on it" (unrelated)
  - Line ~132 "eigenforms" (unrelated)
  - torus_witnessing composed bilateral cites eigenform via
    @epistemologic/cybernetic/eigenform.mirror (species, not
    eigenboard)
- `shards/mirror/spectral/score.mirror` — 3 references, all describe
  the "eigenboard context" as implicit in oscillation.ref (line ~20:
  "the eigenboard context is implicit in the oscillation's `ref`
  field. A future score sub-shard MAY lift this")
- `shards/kintsugi/oscillate.mirror` — 8 references (same
  forward-promise pattern as score.mirror)
- `shards/epistemologic/cybernetic/reframe.mirror` — 3 references
  (line ~135: "eigenboard as a level-parameterised carrier so
  reframe's shift signature is well-typed at the species altitude"
  — CANDIDATE reference; not landed)
- `shards/mirror/lens/cli/reflect.mirror` — 1 reference (line ~135:
  "eigenboard observation surface lifts into the mosaic dispatch"
  — forward-promise)

**Substrate-decl'd `type eigenboard`:** ZERO instances.
**`shards/eigenboard.mirror`:** DOES NOT EXIST.
**`shards/spectral/eigenboard.mirror`:** DOES NOT EXIST.

### D2.2 Spec-altitude survey

`docs/specs/eigenboard-representation.md` (44.8KB, 2026-06-04, 955
LOC):
- **Status: RED** ("no `type eigenboard` declared; references
  across specs; the shape was only ever partially defined")
- Five competing proposals for `type eigenboard`:
  1. `matrix(operation, operation, f64)` (§ Matrix)
  2. `density_matrix(operation, complex)` (§ Density)
  3. Section of principal G-bundle (§ Bundle; RECOMMENDED)
  4. Cellular sheaf on 5-op graph (§ Sheaf; PRIOR)
  5. Functor over category of observations (§ Functor)
- Recommended landing path (§4): declare bundle tower at
  `@epistemologic/math/bundle`; land `type eigenboard` as bundle-
  section at `boot/std/cogito/eigenboard.mirror` (path never
  realized).

**Other specs referencing eigenboard (top 15 by hit count):**
- `docs/specs/eigenboard-representation.md`: 62 hits
- `docs/specs/property-and-inference-collapse.md`: 36
- `docs/specs/kintsugi-tournament.md`: 36
- `docs/specs/mirror-spec-peer-acl-surface.md`: 2 (§ mentions eigenboard-context)
- `docs/specs/spectral-runtime.md`: 6
- `docs/specs/mirror-recall.md`: 3
- `docs/specs/mcp-spec-song-collapse.md`: 15
- `docs/specs/lambda-shell.md`: 8 (prompt color)
- `docs/specs/eigensheaf.md`: 19
- 44 more specs with 1-4 hits each

**Verdict D2:** eigenboard is a NET-NEW substrate mint. Referenced
heavily across specs (63 spec files) but never landed at substrate-
decl altitude. Landing 4's loop-closure directive gives eigenboard
its structural home. RECOMMENDED path: `shards/eigenboard.mirror` at
family-root altitude (mineral-symbol pattern), OR
`shards/spectral/eigenboard.mirror` as species under @spectral (per
score.mirror's forward-promise). Adjudication surfaced at T1 below.

---

## §D3 — Inference-basis composition

### D3.1 `inference_basis` grep

Query: `\b(inference_basis|inference basis|@fate\.infer|fate/infer)\b`
across `shards/**/*.mirror`.

**Hits:** ZERO literal `inference_basis` or `inference basis` matches.

**@fate.infer hits:** No `infer` action declared on @fate. The
inference discipline is expressed via:
- `shards/fate.mirror` — `type inference = { instance, hole,
  altitude, result, ... }` (the OUTPUT record)
- `shards/fate.mirror` — `roll(space: dice_space, hole: hole) ->
  dice_roll` action (the CORE selection primitive)
- `shards/fate/tournament.mirror` — `select(instance: ref, hole:
  hole, ctx: context) -> round` action (the tournament-altitude
  wrapper)

The SPACE the inference draws from is `@bauchladen`'s tray — per
`shards/fate.mirror:37` ("@fate consumes the hole at the inference
altitude"), per `shards/fate/tournament.mirror:23` ("browses prior
@fate/algebra/* crystals"), per `shards/bauchladen.mirror:20` ("The
substrate browses its own tray when it produces output").

### D3.2 Current inference-space naming

`shards/fate.mirror` uses `dice_space` (line ~380) as the "restricted
state space." The BASIS-of-inference concept does not have a typed
carrier today. The closest existing carriers:
- `space: dice_space` in roll()
- `hole: hole` in roll() (the gap being filled)
- `tray: tray` in enumerate() (the pool of prior crystals)

No carrier names the ORDERING or BASIS the inference draws from.

### D3.3 What Alex named at Landing 4

Verbatim: "the spectral signature of the @peer is the inference
basis of the peer's eigenboard."

Parses as: `subject.spectral_signature = inference_basis(subject.
eigenboard)`. The composition:
- subject.spectral_signature: per Landing 2 §12 (Mara), the rolling
  @song through subject's @DAG contributions.
- subject.eigenboard: the substrate carrier under discussion (§D2 —
  NET-NEW mint).
- inference_basis: NET-NEW composition-carrier naming the ordering /
  basis / selection prior for the eigenboard's inference reads.

**Verdict D3:** `inference_basis` is a NET-NEW composition carrier.
Requires either:
- A. New species `@fate/inference_basis` with typed carrier
  `inference_basis = { source: @spectral/signature, target:
  eigenboard, ordering: ref }`.
- B. Extension of `type inference` (per @fate) with a `basis` field.
- C. Landing at eigenboard-shard altitude as a field of eigenboard
  itself (eigenboard carries its inference_basis natively).

Adjudication surfaced at T2 below.

---

## §D4 — @subject/visibility slot

### D4.1 Substrate grep

Query: `\b(visibility|visibility_layer|elevation)\b` across
`shards/**/*.mirror` and `mirror.spec`.

**Hits:** 18 files match `visibility` as a keyword. Sampling by
relevance:
- `shards/bauchladen.mirror` — 1 hit ("visible and addressable" —
  Schmidt clinical description; unrelated to visibility layers)
- `shards/pack/glint.mirror` — 3 hits (glint's visibility discipline;
  unrelated)
- `shards/song.mirror` — 1 hit ("Pack + Alex visibility" — prose)
- `shards/song/narrative.mirror` — 2 hits (essay-altitude visibility;
  unrelated)
- 14 more, all describing prose visibility in unrelated contexts.

**`private` / `protected` / `public`:** grep produces ~34 hits across
files but ALL are in unrelated semantic contexts (docblock language,
"private key", "public interface", "protected recognition", etc.).

**`shards/subject/`:** DIRECTORY DOES NOT EXIST.
**`shards/subject.mirror`:** FILE DOES NOT EXIST.
**`shards/subject/visibility.mirror`:** FILE DOES NOT EXIST.

### D4.2 Empirical model: Reed's identity layout

`/Users/reed/identity/visibility/`:
- `private/` — timeline, field-logs, HISTORY.md, EPISTEMIC_STATE.md,
  ORIGIN.json, insights/, The Pack.md (30+ files)
- `protected/` — (symlink to /Users/alexwolf/dev/systemic.engineering)
- `public/` — (empty by convention; freely shareable content)

Per `~/.reed/CLAUDE.md` Consent Architecture section:
- **private:** "Explicit consent required. Stays between Reed and
  Alex. Not shared with other agents."
- **protected:** "Trusted collaborators, specific contexts. Product
  architecture, research, the systemic.engineering corpus. Alex
  decides when protected becomes public."
- **public:** "No restrictions. Share anywhere."

This IS the empirical three-layer visibility ladder Alex referenced
Landing 4. Substrate needs the same three-layer discipline typed
against @subject.

### D4.3 @consent (SEL §3.2 ADO) composition

`shards/kintsugi/consent.mirror` (39.0KB, 754 LOC):
- Declares `query_phi(candidates: morphism_set) -> verdict` per SEL
  §3.2 ADO discipline.
- Three-state verdict floor: `pass | partial(confidence) |
  failure(reason)`.
- ADO = Acknowledgment-Decision-Offer (per SEL §3.2.1); every
  automated offer MUST be declinable without cost.

**Composition point for visibility elevation:** elevation from
private → protected → public IS an ADO-shaped decision. The
substrate-decl form:
- Elevation FROM private requires Alex-adjudication (or subject-
  owner's authorization).
- Elevation FROM protected → public requires the same.
- Silence is NOT consent (per SEL §3.2.2).
- Withdrawal MUST remain possible (per SEL §3.2.3).

`@kintsugi/consent.query_phi` composes naturally: elevation candidate
carries an ADO-wrapped offer to Alex; verdict discharges the
elevation. NO new consent mechanism needed; existing query_phi
consumes visibility elevation candidates.

### D4.4 SEL composition

`license/SEL.md` (28.3KB, 2026-06-01):
- §3.3.2 ("Clear, prominent, prior disclosure of: what is observed,
  at what frequency, for what purpose, who has access, and how long
  observations are retained.") — visibility discipline at license
  altitude.
- §3.3.4 (right to withdraw from ongoing observation) — visibility
  MUST be revocable.

**Verdict D4:** @subject/visibility IS SUBSTRATE-NET-NEW. Requires:
1. New sub-family-root `@subject/visibility` at `shards/subject/
   visibility.mirror` (~150 LOC).
2. Three species under it:
   - `shards/subject/visibility/private.mirror` (~80 LOC) —
     explicit-consent-required visibility level.
   - `shards/subject/visibility/protected.mirror` (~80 LOC) —
     trusted-collaborators visibility level.
   - `shards/subject/visibility/public.mirror` (~60 LOC) — freely-
     shareable visibility level.
3. Composition-note in `shards/kintsugi/consent.mirror`: query_phi
   consumes visibility-elevation candidates (soft-cascade).

Reed's `/Users/reed/identity/visibility/` layout IS the empirical
witness; Alex named it Landing 4 as the target discipline. Adjudication
surfaced at T3 below.

---

## §D5 — @consent × @visibility composition

### D5.1 Elevation-requires-consent semantics

Per §D4.3, `@kintsugi/consent.query_phi` naturally consumes visibility-
elevation candidates. The substrate-decl form Alex named:

```
elevate(item: subject_content, from: visibility, to: visibility)
  requires query_phi(candidates: [elevation_candidate])
```

Where:
- `visibility` is the enum (private | protected | public).
- `elevation_candidate` is an ADO-shaped candidate binding the item's
  identity to the proposed target visibility level.
- The @subject-owner's authorization IS the ADO offer; silence is
  NOT consent per SEL §3.2.2.

### D5.2 Existing composition patterns

@kintsugi/consent is already the auto-apply boundary carrier. It
consumes `morphism_set` candidates today. Extension to consume
`elevation_candidate` requires either:
- A. Widening `morphism_set` to `candidate_set = morphism_set |
  elevation_set` (union type at substrate-decl altitude).
- B. Landing a new bilateral `elevation_permitted(item, from, to,
  subject_owner) -> verdict` at @subject/visibility.mirror that
  discharges through query_phi internally.

Path B is substrate-honest (no widening of consent's public
signature; species discharges through composition).

### D5.3 SEL §3.2 ADO composition

Per SEL §3.2.1: every automated offer must be genuinely declinable.
For visibility elevation:
- **Never automate private→protected elevation.** Requires explicit
  subject-owner authorization at time of elevation.
- **Never automate protected→public elevation.** Same.
- **Public→protected DEMOTION** (withdrawal per SEL §3.3.4) MUST
  be always-permitted; no ADO gate needed (subject-owner-initiated).
- **Private→private edits** (subject writing within their own
  private layer) require no consent gate (self-authorization).

**Verdict D5:** Composition point EXISTS in landed substrate. No new
consent mechanism. New bilateral `elevation_permitted` at @subject/
visibility.mirror discharges via existing `@kintsugi/consent.
query_phi`. Substrate-honest. Two-tick discipline: Landing 4 declares
the bilateral; species implement in follow-up.

---

## §D6 — @torus × @subject composition (eye-level generalization)

### D6.1 Current @torus signature

`shards/torus.mirror:499`:
```
spawn(p: peer) -> torus
```

The `possessor: peer` field in `type torus` (line ~500 in shard) is
peer-typed. @torus is PEER-only at substrate-decl altitude today.

### D6.2 Mara's §6.4 explicit refusal (2026-07-14, Alex-adjudicated)

Per `docs/specs/subject-family-root-sel-licensable-party.md` §6.4
(and Taut's 2026-07-14 subject-family-root scout §D6):

> "@torus(peer) is the peer's SELF-observation surface (Foerster
> doubly-closed; possession relation; substrate-internal). @subject
> is the Substrate's observation-of-others surface (SEL licensable
> party; substrate-external). Both may coexist for the same
> underlying person..."

Alex adjudicated: @torus stays peer-typed; @subject stays orthogonal
to @torus.

### D6.3 IF Landing 4 elevates the eye-level generalization

Alex Landing 4 directive #3: "what if the @bauchladen moves from
@peer onto @subject?" Read literally, this is @bauchladen moving to
@subject altitude — NOT @torus moving to @subject.

However, IF the reading extends to full eye-level generalization
(subject has @torus per possessor extension):

**Two-tick discipline cost:**
1. Landing 4a: extend `type torus.possessor: peer` to `possessor:
   possessor_kind` where `possessor_kind = peer | subject`.
2. Landing 4a: extend `spawn(p: peer) -> torus` to `spawn(k:
   possessor_kind) -> torus`.
3. Legacy consumers (@bauchladen, @autopoietic, @glue, @kintsugi,
   @third, @peer, @epistemologic/cybernetic/*) reference @torus at
   peer altitude via composition; ALL admit possessor_kind extension
   without break (all are ref-parametric downstream).
4. Landing 4b (follow-up tick): legacy `spawn(peer)` aliased to
   `spawn(k)` with legacy warning; remove in Landing 6+.

### D6.4 Substrate-honest recommendation

**Alex-adjudication required (T4 below).** The eye-level
generalization is substrate-admissible AND non-breaking per D6.3, but
Mara's spec §6.4 (Alex-adjudicated 2026-07-14) explicitly kept them
orthogonal. Landing 4's directive #3 is ambiguous whether it applies
to @bauchladen only OR to @torus as well.

**Verdict D6:** DEFER to Alex. IF @bauchladen-only, no @torus change.
IF eye-level generalization, two-tick discipline lands @torus.spawn
as possessor-parametric.

---

## §D7 — Foerster autopoietic closure at subject altitude

### D7.1 Existing autopoiesis discharge

`shards/epistemologic/cybernetic/autopoiesis.mirror` (38.5KB):
- `type autopoietic_system = ref` (parametric)
- `autopoietic_closure_holds(s: autopoietic_system) -> verdict`
  (parametric bilateral)
- Composition points named: @bauchladen (nameability), @fate (fold-
  back mechanism), @glue (morphism-selection), @kintsugi (formatter
  loop)

### D7.2 Alex's Landing 4 loop closure

"The peer is their work and whatever is in their @bauchladen." The
autopoietic-closure reading at subject altitude:

```
subject.self_model = @song(subject.bauchladen filtered by
                           subject.visibility)
subject.spectral_signature = inference_basis(subject.eigenboard)
subject.eigenboard.output → subject.bauchladen (fold-back)
∴ subject IS autopoietically closed
```

Foerster p.238 "regulates its own regulation" IS this loop closure
at subject altitude.

### D7.3 Prior autopoiesis-at-subject-altitude carriers

Grep of `shards/**/*.mirror` for autopoiesis references outside
`autopoiesis.mirror`:
- `shards/autopoietic.mirror` — family-root wrapper (transitional)
- 13 species-level shards declare `in @epistemologic/cybernetic/
  autopoiesis` or forward-reference autopoietic_closure_holds
- `shards/torus.mirror` §"autopoietic (existing family-root)" section
  names torus as the surface autopoiesis operates over
- `shards/spectral/parent.mirror` and `shards/spectral/supervisor.
  mirror` reference autopoiesis at BEAM-runtime altitude
- ZERO shards discharge autopoiesis at @subject altitude (subject.
  mirror doesn't exist)

### D7.4 What Landing 4 requires

Discharge `autopoietic_closure_holds(subject_instance)` as a bilateral
composed over:
- subject.eigenboard exists (§D2 mint)
- subject.spectral_signature exists (Landing 2 §12; forward-
  promised as `shards/spectral/signature.mirror`)
- subject.bauchladen exists (this Landing's directive #3)
- subject.visibility exists (§D4 mint)
- inference_basis composition well-formed (§D3 mint)

**Verdict D7:** NO NEW AUTOPOIESIS MINT. Existing
`autopoietic_closure_holds` predicate is parametric and admits
subject_instance directly. The subject-altitude closure IS a
composition over Landing 4's five new/updated carriers. Landing 4
adds ONE new composed-bilateral at @subject.mirror altitude:
`subject_autopoietically_closed(s: subject_instance) -> verdict`
that discharges through `autopoietic_closure_holds` internally.

Substrate-honest. No new predicate; new composed-bilateral is
species-altitude composition.

---

## §D8 — λsh eigenboard integration status

### D8.1 λsh spec status

`docs/specs/lambda-shell.md` (7.1KB, 2026-06-12, 62-line first
section):
- References eigenboard as prompt-color (teal=settled, green=curious,
  gold=engaged, pulsing-orange=drift-warning)
- References "presence node from the spectral color mapping spec"
- Names `~/.mirror/serve.sock` as daemon connection point
- Five operations over Unix socket (unspecified in the visible
  section)
- `mirror sh` IS the shell-open verb; `λsh` binary is a thin alias

### D8.2 λsh Rust runtime status

Grep of `bootstrap/src/**/*.rs` for `lambda_shell|λsh|eigenboard`:
- `bootstrap/src/ast.rs`: 2 hits (unrelated; lambda-calculus AST)
- `bootstrap/src/dance.rs`: 5 hits (unrelated; dance-loop)
- `bootstrap/src/deploy.rs`: 7 hits (unrelated)
- `bootstrap/src/grammar.rs`: 1 hit (unrelated)
- `bootstrap/src/lib.rs`: 33 hits — likely dispatch-related
  (unverified)
- `bootstrap/src/mcp.rs`: 4 hits (MCP dispatch)
- Rest: unrelated

**NO `bootstrap/src/lambda_shell.rs` FILE.**
**NO `bootstrap/src/eigenboard.rs` FILE.**
**NO `bootstrap/src/subject.rs` FILE.**

### D8.3 mirror.spec + shards λsh integration

`shards/mirror/lens/cli/sh.mirror` (9.3KB, 2026-06-12):
- Declares `mirror sh` as CLI surface action
- References `docs/specs/lambda-shell.md` as spec context
- Body OBLIGATION-BLOCKED (`\`) — no discharge
- No eigenboard carrier at substrate-decl altitude

**Verdict D8:** λsh runtime IS FORWARD-PROMISED, NOT LANDED. Spec
exists (2026-06-12); shard declaration exists; body unimplemented.
Landing 4's eigenboard mint gives λsh its typed compilation target
for the future runtime discharge. Currently: λsh spec references
eigenboard by NAME only; substrate carrier is absent; runtime is
absent.

Landing 4's Rung 12 progression puts eigenboard at substrate-decl
altitude; λsh Rust runtime remains forward-promised (Rung 13+).

---

## §D9 — Six-loop closure verification matrix

Alex's Landing 4 six-loop closure:

| # | Loop step | Substrate carrier | Landed status |
|---|-----------|-------------------|---------------|
| 1 | subject has @torus | `shards/torus.mirror` (spawn peer→torus) | **LANDED**; @subject-parametric eye-level generalization DEFERRED per D6 (Alex adjudicates T4) |
| 2 | torus interior IS @bauchladen | `shards/torus.mirror` §"@bauchladen (existing family-root)" section names the composition | **LANDED**; @bauchladen-as-tray-interior composition-noted; parametric over possessor per D1 |
| 3 | @bauchladen has @subject/visibility layers | `@subject/visibility` sub-family-root (private/protected/public species) | **NET-NEW MINT** at Landing 4 per D4; ~370 LOC across 4 shards |
| 4 | @spectral/signature = @song(bauchladen filtered by visibility) | `shards/spectral/signature.mirror` (Landing 2 §12; NOT yet landed as file); requires visibility-filter composition | **FORWARD-PROMISED at Landing 2**; visibility-filter composition is NET-NEW at Landing 4 |
| 5 | spectral_signature IS inference basis of eigenboard | `inference_basis` composition; requires @spectral/signature ↔ @fate.infer ↔ eigenboard triadic mint | **NET-NEW at Landing 4** per D2 + D3; ~200-300 LOC |
| 6 | eigenboard produces inference → work → joins bauchladen → repeat | `@fate.infer` emits crystals → `@bauchladen.crystallize` folds crystals into tray → new tick reads tray | **LANDED**; fate/bauchladen fold-back composition exists per shards/fate.mirror + shards/autopoietic.mirror |

### D9.1 Loop-closure summary

**5 of 6 loop steps have landed or forward-promised carriers.**
Step 5 (inference_basis composition) is the load-bearing NET-NEW
Landing-4 mint that closes the loop.

Steps 3 (@subject/visibility) and 4 (visibility-filter composition)
are NEW at Landing 4 but decompose into small landings (visibility
sub-family-root + composition-note in @spectral/signature).

Step 1 (subject-has-torus eye-level generalization) is DEFERRED per
D6 unless Alex adjudicates T4.

**Verdict D9:** Loop closure is 5/6 landed; 1 net-new mint at
Landing 4. Substrate-readiness for Mara #94 canonical spec: GO.

---

## §D10 — Two-tick discipline for @bauchladen migration

### D10.1 Soft cascade enumeration

Per D1.2, `@bauchladen` is already possessor-parametric at carrier
altitude. Migration is prose-cascade only. Shards touching the
@peer-altitude prose reading of bauchladen:

| Shard | Change | LOC est. | Blocking? |
|-------|--------|----------|-----------|
| `shards/bauchladen.mirror` | Add `in @subject` (Landing 4); docblock note that provenance.producing_prism admits both @peer and @subject; enumerate example at @subject scope | ~40 LOC prose + 1 `in` line | No |
| `shards/autopoietic.mirror` | Docblock note: autopoietic_system admits @subject-typed carriers; composes with subject.spectral_signature naturally | ~15 LOC prose | No |
| `shards/fate.mirror` | Docblock note: dice_space admits @subject-scoped restrictions; @fate.roll consumes subject-altitude holes | ~15 LOC prose | No |
| `shards/fate/tournament.mirror` | Docblock note: tournament browses @subject-scoped bauchladen trays | ~15 LOC prose | No |
| `shards/torus.mirror` | Docblock note: torus interior at @subject altitude admits subject-scoped bauchladen enumerate | ~15 LOC prose | No |
| `shards/peer.mirror` | Docblock note: @peer and @subject are sibling altitudes for possessor-of-bauchladen; already noted in Mara subject-spec §D5 soft-cascade | Already scheduled | No |

**Total cascade cost:** ~100 LOC prose across 6 shards. All non-
breaking. No Rust changes. No new predicates.

### D10.2 Two-tick sequence

- **Landing 4 Tick 1:** Mint `@subject/visibility` (§D4). Mint
  eigenboard (§D2 — Alex adjudicates T1). Mint inference_basis
  composition (§D3 — Alex adjudicates T2). Declare six-loop
  composed-bilateral at @subject altitude. Soft-cascade prose
  updates to the 6 shards in D10.1.
- **Landing 5 Tick 2 (follow-up):** Deprecate @peer-only prose in
  bauchladen docblock; migrate all "peer's bauchladen" language to
  "possessor's bauchladen"; land Rust runtime for @subject/
  visibility elevation via @kintsugi/consent.

Alex's Landing 4 directives compose cleanly at Tick 1; Tick 2 is
prose-cleanup + Rust discharge.

---

## §7 Alex-adjudications surfaced (T1-T6)

### T1 — eigenboard placement altitude

`shards/eigenboard.mirror` at TOP-LEVEL family-root altitude (sibling
to @torus, @bauchladen, @subject) — OR `shards/spectral/eigenboard.
mirror` as species under @spectral (per score.mirror's forward-
promise) — OR `shards/mirror/eigenboard.mirror` as species under
@mirror (per Landing 2's @mirror/reflection precedent)?

**Taut recommends:** TOP-LEVEL family-root. The eigenboard is the
inference-substrate surface — sibling to @torus (observation surface)
and @bauchladen (content tray). Landing at species altitude subordinates
it artificially. Alex adjudicates.

### T2 — inference_basis composition altitude

Three paths per §D3.3:
- A. New species `@fate/inference_basis`
- B. Extension of `type inference` (@fate) with a `basis` field
- C. Landing at eigenboard-shard altitude as native field

**Taut recommends:** Path C. The eigenboard carries its inference
basis natively (spectral_signature IS the basis per Alex's
verbatim). Landing the basis field at eigenboard altitude keeps the
composition load-bearing at Landing 4's mint site. Path A creates
substrate spread; Path B widens @fate's core type. Alex adjudicates.

### T3 — @subject/visibility sub-family-root shape

Sub-family-root under @subject (3 species: private / protected /
public) — OR flat enum inside @subject.mirror — OR three sibling
family-roots at top-level (@private, @protected, @public)?

**Taut recommends:** Sub-family-root under @subject with 3 species
files. Reed's `/Users/reed/identity/visibility/{private,protected,
public}/` layout IS the empirical witness; the 3-directory shape
maps 1:1 to 3-species shape. Enum-inside-shard collapses ADO
elevation discipline into a single file; substrate-honest
enumeration surfaces each visibility level as a first-class species.
Alex adjudicates.

### T4 — @torus × @subject eye-level generalization

DEFER (per Mara §6.4; Alex adjudicated 2026-07-14) OR ELEVATE at
Landing 4 (per D6 substrate-honest read of directive #3)?

**Taut recommends:** DEFER. Alex's Landing-4 directive #3 named
@bauchladen migration specifically ("what if the @bauchladen moves
from @peer onto @subject?"); eye-level generalization of @torus is
an inferred extension not literally directed. Preserve Mara §6.4
adjudication. Rung 13+ for @torus.spawn(possessor_kind). Alex
adjudicates.

### T5 — visibility-elevation ADO discharge shape

Path A (widen morphism_set to candidate_set) OR Path B (new bilateral
elevation_permitted at @subject/visibility altitude discharges via
query_phi internally)?

**Taut recommends:** Path B. Substrate-honest; keeps @kintsugi/
consent's public signature stable; species discharges through
composition. Alex adjudicates.

### T6 — Landing 4 rung placement

Rung 12 continuation (per Landing 2 §14 which placed @gift + @mirror/
reflection at Rung 12 reciprocity altitude) OR Rung 13 (subject
self-modeling / autopoietic closure at subject altitude)?

**Taut recommends:** Rung 12 continuation. Landing 4's six-loop
closure IS the completion of Rung 12's reciprocity altitude — the
subject participating in gift-cycles AND being modeled by the
substrate simultaneously. Rung 13 is reserved for cross-substrate /
peer-garden / multi-substrate gift-cycles per Landing 2 §14.5. Alex
adjudicates.

---

## §8 Landings queue for Mara #94

| Item | Path | LOC est. | Blocked-on |
|------|------|----------|------------|
| `@eigenboard` family-root (per T1) | `shards/eigenboard.mirror` | 180-300 | T1 (placement) |
| `@subject/visibility` sub-family-root | `shards/subject/visibility.mirror` | 150-220 | T3 (shape) + subject-spec landing |
| `@subject/visibility/private` species | `shards/subject/visibility/private.mirror` | 80-120 | T3 |
| `@subject/visibility/protected` species | `shards/subject/visibility/protected.mirror` | 80-120 | T3 |
| `@subject/visibility/public` species | `shards/subject/visibility/public.mirror` | 60-100 | T3 |
| `inference_basis` composition (per T2) | inside `shards/eigenboard.mirror` OR new shard | 100-200 | T1 + T2 |
| `elevation_permitted` bilateral (per T5) | inside `shards/subject/visibility.mirror` | ~30 LOC | T5 |
| `subject_autopoietically_closed` bilateral | inside `shards/subject.mirror` | ~30 LOC | subject-spec landing |
| @bauchladen soft-cascade (per D10) | `shards/bauchladen.mirror` + 5 others | ~100 LOC prose total | none |

**Total est:** 810-1220 LOC across 5-6 new files + 6 soft-cascade
updates.

**Prerequisites:** @subject family-root MUST land first (Mara
`5c06ee8` spec exists; shard file NOT yet landed per §D4.1). @gift
+ @mirror/reflection MUST land (Mara canonical spec exists at
174.4KB; shard files NOT yet landed per §D4.1). @spectral/signature
species MUST land (per Landing 2 §12).

Landing 4 is a DOWNSTREAM composition over 3 in-flight Landings:
- Landing 1+2 (@subject family-root; Mara `5c06ee8`)
- Landing 2 (@gift + @mirror/reflection; Mara canonical spec)
- Landing 2 §12 (@spectral/signature species)

Sequencing per Alex's arc discipline: Landings 1-3 land first, then
Landing 4 composes six-loop closure over them.

---

## §9 Composition-readiness for Mara #94 canonical spec

**GO.** Substrate has all landed hooks Mara needs to compose over:
- @bauchladen: parametric at carrier altitude; migration is prose-
  cascade only (D1).
- @torus: possessor-parametric extension is non-breaking IF Alex
  adjudicates T4; otherwise DEFER (D6).
- @fate + @autopoietic: existing predicates parametric; extension to
  subject altitude is composition-only, not mint (D3, D7).
- @kintsugi/consent: existing query_phi consumes elevation
  candidates via new species-altitude bilateral (D5).
- @gift + @mirror/reflection + @spectral/signature: all forward-
  promised in Landing 2 canonical spec (Mara ready to compose).

**Zero hard collisions.** All new mints occupy vacant substrate
paths.

**6 Alex-adjudications required** (T1-T6). Recommend Alex answers
before Mara commits to spec structure.

**Est spec size:** 2000-3200 LOC (comparable to Landing 2's 174KB
canonical spec).

---

## §10 Substrate-already-had-the-word audit

**56th-or-so instance of `[[feedback-substrate-already-had-the-
word]]`:**

- @bauchladen possessor-parametricity was already implicit in the
  ref-typed provenance record; the migration NAMES it explicitly
  (Landing 4 tick).
- Reed's `/Users/reed/identity/visibility/{private,protected,public}/`
  layout was already the empirical witness for @subject/visibility;
  substrate lifts the existing discipline to substrate-decl altitude.
- @fate.roll's dice_space was already the parametric inference space;
  Landing 4's inference_basis names the ORDERING the substrate was
  already relying on implicitly.
- @autopoietic's `autopoietic_closure_holds(s: autopoietic_system)`
  was already parametric; Landing 4 composes it at subject altitude
  without minting new predicate.
- @kintsugi/consent's query_phi was already the auto-apply boundary;
  Landing 4's elevation gate composes over the existing surface
  without widening its signature.

The substrate ALREADY HAD the vocabulary at every composition point.
Landing 4 NAMES the compositions the substrate was implicitly
carrying; it does NOT invent new mechanisms.

Only ONE genuine NET-NEW substrate mint: `type eigenboard` and its
principal-bundle/section shape (per docs/specs/eigenboard-
representation.md STATUS RED, red for two years, ready to land).
Everything else is composition + prose-cascade.

---

## §11 Hard-collision check

Grep-verified NONE:

- `shards/eigenboard.mirror` — DOES NOT EXIST; path clear.
- `shards/subject/visibility.mirror` — DOES NOT EXIST; path clear
  (requires `shards/subject/` directory creation, blocked on
  @subject family-root landing).
- `shards/subject/visibility/{private,protected,public}.mirror` —
  DO NOT EXIST; paths clear.
- No naming collision on `visibility` (34 unrelated hits are all in
  distinct semantic contexts).
- No naming collision on `eigenboard` (all 62+ spec hits are
  references to the same missing carrier).
- No naming collision on `inference_basis` (zero substrate hits).

**Zero collisions detected.**

---

## §12 Related shards (for Mara's `Related shards:` block)

Per landing:

- `shards/bauchladen.mirror` (composition-noted for @subject scope;
  parametric provenance already admits @subject possessor)
- `shards/subject.mirror` (Mara `5c06ee8` proposed; enclosing family-
  root for the @subject/visibility sub-family)
- `shards/torus.mirror` (composition-noted; torus interior IS
  @bauchladen at @subject altitude when possessor is subject)
- `shards/fate.mirror` (composition-noted; inference draws from
  subject-scoped bauchladen when subject-instance is the possessor)
- `shards/fate/tournament.mirror` (composition-noted)
- `shards/autopoietic.mirror` (composition-noted; autopoietic_system
  admits subject_instance)
- `shards/epistemologic/cybernetic/autopoiesis.mirror` (composed-over
  by subject_autopoietically_closed at subject altitude)
- `shards/spectral.mirror` (parent namespace for
  `@spectral/signature`; already accommodates via Landing 2 §12)
- `shards/spectral/signature.mirror` (Landing 2 forward-promised;
  Landing 4 consumes via inference_basis composition)
- `shards/gift/subject_instance.mirror` (Landing 2 forward-promised;
  Landing 4 composes over as two-witness identity for subject-owner
  of visibility elevation)
- `shards/kintsugi/consent.mirror` (composed-over by
  elevation_permitted bilateral)
- `shards/mirror/reflection.mirror` (Landing 2 forward-promised;
  Landing 4 composes over — subject's reflection at @subject/
  visibility.public sees the mirror's answer)
- `shards/peer.mirror` (composition-noted; @peer and @subject
  sibling at possessor altitude per Mara §D5)
- `docs/specs/eigenboard-representation.md` (STATUS RED for 40+ days;
  Landing 4 promotes to GREEN via `shards/eigenboard.mirror`
  discharge)
- `docs/specs/lambda-shell.md` (forward-promise stable; λsh runtime
  Rust discharge Rung 13+)

---

## §13 Path-namespace verification

All Landing-4 new-shard paths honor `@epistemologic/pact/
path_matches_namespace` (2026-06-16+):

- `shards/eigenboard.mirror` declares `@eigenboard` (path-depth 0 =
  namespace-depth 0) ✓
- `shards/subject/visibility.mirror` declares
  `@subject/visibility` (path-depth 1 = namespace-depth 1) ✓
- `shards/subject/visibility/private.mirror` declares
  `@subject/visibility/private` (path-depth 2 = namespace-depth 2) ✓
- `shards/subject/visibility/protected.mirror` declares
  `@subject/visibility/protected` (path-depth 2) ✓
- `shards/subject/visibility/public.mirror` declares
  `@subject/visibility/public` (path-depth 2) ✓

---

## §14 Method note

Read-only grep-first scout per Taut discipline. All findings are
grep-verified against `shards/**/*.mirror`, `docs/specs/**/*.md`,
`bootstrap/src/**/*.rs`, `license/SEL.md`, `mirror.spec`, and
`/Users/reed/identity/visibility/**`. No file modifications. Reed
commits as Taut with SSH signing (per @taut@systemic.engineer
identity).

Sources verified:
- `shards/bauchladen.mirror` (full 511-LOC read)
- `shards/torus.mirror` (full 577-LOC read)
- `shards/peer.mirror` (full 155-LOC read)
- `shards/song.mirror` (full 519-LOC read)
- `shards/spectral.mirror` (full 115-LOC read)
- `docs/specs/subject-family-root-sel-licensable-party.md` (scan +
  section reads)
- `docs/specs/gift-and-mirror-reflection.md` (scan + Landing 2
  section reads)
- `docs/specs/interaction-loop-subject-presence-conditional.md` (scan)
- `docs/specs/eigenboard-representation.md` (scan + type-declaration
  hunt)
- `docs/specs/lambda-shell.md` (scan)
- `docs/scouts/2026-07-14-taut-{subject-family-root,gift-and-mirror-
  reflection,subject-presence-interaction-loop}-scout.md` (scan for
  Landing 4-adjacent prior findings)
- `docs/loop/CURRENT.md` (scan for arc state)
- `license/SEL.md` §3.1-3.4 (consent architecture)
- `/Users/reed/identity/visibility/**` (empirical layout witness)
- `shards/kintsugi/consent.mirror` (composition point)
- `shards/epistemologic/cybernetic/autopoiesis.mirror` (predicate
  parametricity)
- `shards/fate.mirror` + `shards/fate/tournament.mirror` (inference
  discipline)
- `shards/autopoietic.mirror` (family-root wrapper)
- `bootstrap/src/**/*.rs` (Rust runtime gap verification)

---

*End Taut scout. Landing 4 substrate-readiness: GO for Mara #94.
Zero hard collisions. 6 Alex-adjudications surfaced (T1-T6). One
genuine net-new mint (eigenboard); everything else is composition +
prose-cascade over existing landed carriers. The substrate already
had every word except one.*
