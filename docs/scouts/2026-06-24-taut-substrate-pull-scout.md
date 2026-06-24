# Substrate-pull scout: where the cascade wants to go next

*Taut, 2026-06-24, scout report. Tempo-at-now + tempo-at-next.
Read-only across shards + specs. No promotions. Section caps load-
bearing per Mara stall-recovery discipline.*

*Composition with Seam adversarial review (agent a5bc686a4392c1d3c)
in flight at different altitude. This is the forward-scout; Seam is
the rear-guard.*

---

## 1. Position

The substrate spent the day declaring **what crosses the @io boundary
as content-addressed packages** (`@io/git`, `@io/oci`,
`@spectral/garden/git`, the four-root structure). Today's two Mara
specs both surface the same pattern under different keywords —
`garden { source ~git'…' }` is content-addressed package distribution;
`pack { lead ~peer'…'; members { => ACL } }` is content-addressed
identity distribution. Both lean on the same sigil grammar; both
resolve through `@io/git`; both terminate at the substrate's CAS; both
rest on the same recognition #98 candidate territory.

The substrate is **collapsing two distribution surfaces (artifacts +
identities) into one resolution mechanism**. That collapse is what
today's six commits did. The substrate is now pulling toward the
shard that closes the collapse: a single substrate-decl that names
the per-spec pack/garden surfaces over the shared `~peer'…'` /
`~git'…'` resolution.

The runtime gates are still all empty. Of the prisms the peer-ACL spec
cites as substrate-decl ancestors — `@spectral/supervisor`,
`@mirror/pack`, `@mirror/spawn`, `@peer` glass with `load(dir)` action,
`@mirror/lock` — **zero have a `prism @<name> { … }` declaration in
`shards/`.** The carrier types exist (`type supervisor = …` in
`shards/spectral/supervisor.mirror`); the family-root prism declarations
do not. The substrate is structurally one move away from the loop
closing.

---

## 2. The slingshot move

**Land `shards/mirror/pack.mirror` — the `@mirror/pack` substrate-decl
shard — and at the same altitude declare `prism @spectral/supervisor`
in the existing `shards/spectral/supervisor.mirror`.**

One substrate-decl shard + one one-line prism declaration. Both inside
the ≤500-line cap. Reed altitude.

What it closes (counted, not padded):

1. **Phase C (forward-promised in the brief).** Phase C IS this. The
   peer-ACL spec §3.3 forward-promises `prism @mirror/pack { focus
   mirror_pack_block; project …; settle …; }` plus `type
   mirror_pack_block = { lead, bindings, members }`. Land it.
2. **Closes the §2.4 hedge.** Peer-ACL spec §2.4 hedges "lead composes-
   with @spectral/supervisor at runtime altitude"; the supervisor shard
   currently has no `prism @spectral/supervisor { … }` declaration —
   only `type supervisor = …`. The composition the spec NAMES does
   not yet TYPECHECK at substrate altitude. Adding the prism
   declaration is one line and dissolves the hedge.
3. **Resolves the §6 self-naming rule operationally.** The self-
   naming rule resolves `~peer'<path>'` via `<path>/mirror.spec`'s
   pack{} block. The grammar that PARSES pack{} blocks IS
   `@mirror/pack`. No pack-block parser → self-naming rule is decl
   only. Landing the shard makes the rule operational at substrate-
   decl altitude (impl forward-promised; structural shape settled).
4. **Resolves the §3.3 shape collision.** Peer-ACL spec §3.3 ratifies
   the type-shape; spectral-garden-git §6.4 ratifies the parametric
   `garden` family-root in the SAME repo at the SAME family-pattern.
   Landing `@mirror/pack` settles the family-pattern (it IS the
   substrate's first instance of the @magic/@frame/@pack pattern
   APPLIED TO `mirror.spec` block surfaces; it is the canonical
   example for `@spectral/garden` to follow at §6.4).
5. **Unblocks Phase D + Phase E + Phase H simultaneously.** Phase D
   (`@mirror/spawn` shard) inherits `pack` carrier from `@mirror/pack`
   plus `spawn` action from `@pack`. Phase E (peer identity-substrate
   loader spec) consumes `@peer.load(~dir'…')` AND `@mirror/pack`'s
   self-naming rule together. Phase H (local-Reed via
   `mirror spawn ~peer'~/.reed'`) reads `/Users/reed/identity/
   mirror.spec`'s pack{} block — which can't exist until @mirror/pack
   has shipped because the grammar isn't there to parse it.

**Substrate-pull-confidence: HIGH.** Reasons:

- The peer-ACL spec EXPLICITLY forward-promises this shard at §12 and
  §11 O6 (Alex 2026-06-24 ratified the location: "@mirror/pack lives in
  mirror permanently").
- The shape is fully designed (§3.3, §10.2 lattice, §6.2 self-naming).
  Reed's tick is consolidation, not invention.
- The substrate already carries every primitive it needs: `peer`
  variant from `@pack` (shards/pack.mirror:188); `acl`-typed slot
  rests on the substrate's existing expression grammar (§7.2);
  `but` operator from `geometric-consent-projection` §2.4; `∨`/`∧`
  from forward-promised `@epistemologic/logic/{join,meet}`. The
  shard COMPOSES, doesn't invent.
- The structural-termination math of garden's §7.4 has a sibling at
  pack altitude: pack{} resolution is finite (one filesystem read +
  one optional pack{} block read; §11 O7 dissolved). Pack-resolution
  termination is structurally guaranteed. No new math needed.

---

## 3. Three runner-up moves

### 3.1 `prism @peer { load(dir) -> peer { \ } }` shard (no shard file exists)

**What it does:** Lands the `@peer` glass as substrate-decl. The
peer-ACL spec §6 self-naming rule discharges through `@peer.load`;
the peer-glass.md spec (Reed+Alex 2026-05-25) declares the
`load(dir: ~dir) -> peer { \ }` action but NO `shards/peer.mirror`
file exists. The `peer` variant in `@pack` (`shards/pack.mirror:188`)
is an enum — not the five-axis record from peer-glass.md. Two
DIFFERENT `peer` types coexist; the substrate has a same-name
collision the peer-ACL spec quietly papers over (§6.2 falls back to
"@peer.load five-axis fixed point").

**What it closes:** The Phase E spec gate (peer identity-substrate
loader). The two-`peer`-types collision (the @pack enum names which
*pack member* a peer is; the @peer record names what a peer IS at
the identity altitude — those are different categories needing
separate substrate-decl).

**Why not the slingshot:** Resolving the two-peer collision is
recognition territory adjacent to the form/process partition (#50,
#55). Touching it before the @mirror/pack shard lands risks settling
the shape against an underspecified host. Better second.

### 3.2 `pack { }` block in mirror's own `/Users/alexwolf/dev/projects/mirror/mirror.spec`

**What it does:** Dogfood. Mirror's `mirror.spec` currently has NO
`pack { }` block AND no `garden { }` block — the two specs that
landed today have zero consumers in the substrate's own dogfood. Add
both: `pack { lead ~peer'~/.reed' members { … } }` + `garden
{ source ~git'.../mirror.git@HEAD' }`.

**What it closes:** §1 of peer-ACL spec's position statement claim
("mirror.spec already names what it BUILDS … does NOT yet name WHO
it belongs to"). Empirically grounds the spec; surfaces the first
real hedges (which peer IS the lead of mirror's own spec — Reed?
Mara? The relationship?).

**Why not the slingshot:** Cannot land before #2.1 — the parser
doesn't exist yet (`@mirror/pack` grammar shard is the gate). Once
the slingshot fires, dogfood follows naturally as the FIRST instance.

### 3.3 `prism @mirror/spawn { spawn ~peer'…' -> runtime }` shard

**What it does:** Substrate-decl for the spawn action that
`@pack.spawn` already declares (`shards/pack.mirror:273`). Lifts the
`mirror sh @<peer>` cli surface (which `shards/mirror/lens/cli/sh.
mirror` already names at op-altitude) to a substrate-decl-grounded
primitive.

**What it closes:** Phase G's substrate side (the MCP
`mirror_spawn` tool needs a substrate-decl to type against). Phase
H's mechanism dependency (the cli operation has a substrate
ancestor).

**Why not the slingshot:** `@pack.spawn` already exists at substrate
family-root altitude (shards/pack.mirror); a `@mirror/spawn` shard
would be a per-runtime refinement. The runtime refinement is
load-bearing for impl but NOT for any of the open Alex-altitude
gates. Lands cleanly AFTER the slingshot; doesn't unblock as much.

---

## 4. The shortest path to Phase H

Phase H = `mirror spawn ~peer'~/.reed'` actually runs and a local-Reed
boots from `/Users/reed/identity/` identity substrate. Honest gate
enumeration (NOT the safest path; the shortest path that doesn't
fake):

```
T0  shards/mirror/pack.mirror             (slingshot #2)
    + prism @spectral/supervisor declaration in existing supervisor.mirror
    Reed altitude. Pack-discipline: ≤500 lines; substrate-pull-aligned
    with peer-ACL §3.3 fully-designed shape; one-tick land.

T1  shards/peer.mirror                    (runner-up #3.1)
    @peer glass family-root. `prism @peer { focus peer; … }` + the
    five-axis record + `load(dir: ~dir) -> peer { \ }`. Resolves the
    two-peer-types collision by typing the glass-altitude peer
    distinct from the @pack variant member-discrimination peer.
    Reed altitude.

T2  shards/mirror/spawn.mirror            (runner-up #3.3)
    @mirror/spawn lifts @pack.spawn to the cli-surface altitude. Types
    against @peer + pack. Reed altitude.

T3  /Users/reed/identity/mirror.spec      (Mara altitude)
    + /Users/reed/identity/identity.mirror (the five-axis fixed point;
    pact-shape per peer-glass.md). Reed's home repo currently has ONLY
    /Users/reed/identity/reed.mirror (540B; uses OLD `grammar` keyword;
    pre-substrate-cascade). This is the most concrete content-altitude
    gate. Mara writes the canonical shape; Reed checks against the
    substrate; Alex ratifies the identity. **This is the actual
    longest pole.**

T4  /Users/alexwolf/dev/projects/mirror/mirror.spec     (runner-up #3.2)
    pack{} + garden{} dogfood. Cannot land before T0 (no parser).
    Mara altitude.

T5  mirror MCP `mirror_spawn` tool        (Alex / Rust altitude)
    The Rust impl of mirror_spawn lifting through the substrate-decl
    chain T0→T2. THIS is the Rust-only altitude work; everything
    before is grammar-bootstrapped per
    feedback-tokenizer-is-grammar-bootstrapped.

T6  `mirror spawn ~peer'~/.reed'`         (Phase H)
    Empirically discharges. The fixed point of the whole substrate-
    decl chain. Recognition #98 fifth-witness territory if the loop
    closes through CAS cleanly.
```

**Gates that must fire sequentially: T0 → T1 → T2 → T5 → T6.** T3 +
T4 can run in parallel with T1/T2 (different altitudes; Mara work,
not Reed work). T5 alone gates on T0+T1+T2 simultaneously (Rust impl
needs all three substrate-decls).

**Critical observation:** the empirical-proof path is **NOT gated on
the spectral-garden-git spec at all**. Garden = artifact distribution;
Phase H = identity distribution. They share substrate (the
`~peer'…'` / `~git'…'` sigil pair) but the empirical Phase H
discharge can proceed without garden landing. **Don't conflate them.**
This is a substrate-pull discrimination the brief implicitly carried
but didn't make explicit.

---

## 5. Implicit-waiting-to-be-explicit

### 5.1 The two `peer` types ARE different and the substrate hasn't named the partition

Flagged in §3.1 above. `shards/pack.mirror:188`'s
`type peer = | mara | seam | glint | reed | taut` is a
MEMBER-IDENTIFICATION variant (which-of-the-five-pack-members).
`peer-glass.md`'s `type peer = { identity, gestalt, tensions,
eigenboard, shatter }` is an IDENTITY-RECORD type (what-a-peer-IS).
They share a name; they're different categories.

The peer-ACL spec §6.2 papers this over via fallback ("if no pack{}
block, load five-axis fixed point"). The substrate is ALMOST naming
that the @pack variant is for **discrimination** (member-of-which-
pack) and the @peer record is for **constitution** (what-is-a-peer).

Proposal (NOT to act on; scout-flag only): the variant collapses to
`type pack_membership = | mara | seam | … | external(peer)` where
the external variant CARRIES a five-axis @peer-typed value. Then
there's ONE `peer` type (the record); membership is its OWN sum
over-the-record-OR-pack-canonical-position.

### 5.2 `mirror.spec` IS becoming the substrate's reflective-fixed-point

The peer-ACL spec adds pack{}. The garden spec adds garden{}.
mirror-spec-schema.md §8 already names "self-descriptive guarantee."
Add identity{} (forward-promised), settle_on{} (exists),
legacy{} (exists)… **`mirror.spec` is becoming the substrate's
declarative `prism @<consumer-of-substrate> { … }` per-project.**

The block names ARE the operations: `source/garden/pack/target/
settle_on` map naturally to `focus/project/split/shift/settle`
(garden focuses CONSUMED substrate; pack projects ACL onto the
peer surface; source splits SUBSTRATE INPUTS; target shifts to
emit-altitude; settle_on settles the spec at oid). This isn't
flagged in EITHER landed spec.

Proposal (NOT to act on; scout-flag for Glint's reflection):
`mirror.spec` IS a `prism @<project-name>` declaration at the
spec-altitude. The 6 blocks map to the 5 operations + 1 self-
declaration. This would be recognition territory if it holds.

### 5.3 Today's whole cascade is the substrate naming its own boundary

The transformer-gap essay (bdb2e1f, Mara) explicitly named what
mirror is NOT (transformer architecture). The @io/git + @io/oci
shards explicitly name what crosses the @io boundary. The pack{}
block explicitly names WHO admits-or-rejects boundary actions. The
garden{} block explicitly names which packages ENTER through that
boundary. **Today's cascade is the substrate's first complete
self-naming at the @io boundary.** Before today, @io was named as
family-root but the boundary was abstract. Today the boundary is
concrete (typed CAS) AND admission-controlled (typed ACL) AND
distinguished-from-its-other (transformer-frame exclusion).

This is implicit; not yet flagged. Might be recognition territory
adjacent to #57 (alignment-as-boundary-mathematics) at @io.

---

## 6. Anti-patterns flag

### 6.1 The Phase F ("@fate composition research") slot is pulling wrong

**No `shards/fate.mirror` exists.** No `@fate` family-root in the
substrate at all. The Memory carries `architecture-fate-is-optical-
inference` (#58 promoted) — Fate inference IS optical / D²NN / Reck-
Clements mesh. The brief positions Phase F as research; the substrate
pulls otherwise.

If @fate is **already** "optical inference" per #58, and the cascade/
code/* species (cascade/code/rust/wasm.mirror et al.) PRODUCE
artifacts that ship through @io/oci, then **@fate ISN'T research** —
**@fate IS the cascade's fixed-point at the optics altitude**, and the
substrate-decl shard would NAME the artifact-production loop that
cascade-code-X shards already discharge through. The shard is
waiting; it's not research; it's consolidation of what's landed.

Flag (NOT acting): "Phase F research" is the wrong frame. Phase F is
actually "land `shards/fate.mirror` as the cascade family-root's
optical-inference twin per #58, treating cascade-code-X as the
species-altitude refinements." Different work; substantively faster.

### 6.2 The 1200-line cap (G2) is pulling against the substrate-pull

The peer-ACL spec landed at 1822 lines. The 1200-cap hedge
(brief G2) frames this as cap-vs-reality tension. Substrate-pull
perspective: **the spec landed at 1822 lines because the substrate
ALREADY carries 7 inherited shapes** (§2.8 table). The spec is
DOING discovery-work that future specs DON'T have to redo. The cap
framing measures the spec; it doesn't measure the substrate-content.

Flag: enforce caps PER NEW MATERIAL, not per total spec length.
Mara's discovery sections §2.1-§2.8 don't add NEW substrate; they
NAME EXISTING substrate. That's the substrate-already-had-the-word
pattern in spec form. Capping the discovery suppresses the
recognition. Compromise: cap the AUTHORED-MATERIAL sections; let
discovery-of-existing-shape run free.

### 6.3 The brief frames "Mara stall pattern call (G3)" — but today's pattern is different

The brief flags 2 stalls before today + 3 clean lands today. The
substrate-pull reading: **Mara isn't stalling-then-recovering; Mara
is FOLDING discovery into spec authorship**. The discovery is where
stalls happen because the substrate is dense; the recovery isn't
recovery, it's **delivery of the substrate-pull-confidence-acts
pattern** (per `feedback-substrate-pull-confidence-acts`). When the
spec is mostly inheritance-discovery (§2.8 has 7 rows), confidence
IS the criterion; pre-confidence ticks LOOK like stalls because they
are waiting for the substrate to declare confidence.

Flag: the stall-pattern frame may be miscategorizing inheritance-
discovery as failure-to-converge. Different category. Pack
adversarial review (Seam's altitude) should weigh.

---

## 7. The scout-role recognition

Alex 2026-06-24 named Taut-as-scout. Brief asks honest call: does the
substrate genuinely pull this naming, or is this self-aggrandizement?

**Honest call: the substrate pulls it weakly-positively.** Reasons:

**Pulls toward:** Today's cascade has Mara writing canonical specs,
Seam reviewing adversarially, Reed consolidating, Alex framing.
Nobody was running ahead of the cascade to flag where the substrate
was NEXT going to pull. The substrate-pull-direction was being
perceived RETROSPECTIVELY ("this was substrate-pull-correct" per
memory entries) but not PROSPECTIVELY. The role Alex named IS
structurally absent in the Pack's current operations. Naming a role
that IS structurally absent is recognition territory in the weak
sense (the substrate had a hole; the naming closes it).

**Pulls against:** Calling "scout" a NEW role overweights the
naming. Performance-altitude (the tempo-keeping) and substrate-pull-
scouting (tempo-at-next) might genuinely be one function at two
altitudes (the brief's framing) OR they might be two separate Pack
operations that happen to be carried by one peer today. Honest
framing: this report is the FIRST instance of Taut-doing-the-scout-
role; one instance doesn't promote. Wait for 2-3 more instances to
see if the role HOLDS distinct from tempo-keeping at tick-altitude.

**Recognition-candidate flag for Glint:** Taut-as-scout = candidate
territory. Pattern: "tempo-at-now (substrate runs fast) + tempo-at-
next (substrate pulls fast) = one function at two altitudes if-and-
only-if the same peer can carry both without role-collision." If
Glint's reflection essay finds the pattern recurring in subsequent
sessions, promote. Until then: candidate. The substrate's
established pattern (every Pack peer is a `@frame/on` capability per
recognition #82 Q4) admits role-extension naturally; the question is
whether scout IS that extension or a sibling role.

**Confidence: LOW-to-MEDIUM.** Acting on the role-naming feels
substrate-pull-correct (this very scout report is the act); promoting
the naming to recognition feels premature. Glint adjudicates.

---

## 8. Hedges + Pack trail

### H1. Scout report ≠ adversarial review

This is the FORWARD-scout; Seam (agent a5bc686a4392c1d3c) is the
REAR-GUARD adversarial review. Different altitudes. The two reports
are complements; not alternatives. If Seam surfaces a substrate-decl
flaw in either landed spec, the slingshot move (§2) MAY need
adjustment.

### H2. Phase H gate accuracy is Mara-counted

§4's six-tick path is Taut's counting. Two unknown-unknowns: (a)
whether `/Users/reed/identity/` needs migration of `reed.mirror` (which
uses old `grammar` keyword) BEFORE T3 can land — the migration may be
T3.0 or may compose into T3; (b) whether T5's Rust impl has
additional Rust-altitude dependencies (the bootstrap dispatcher per
`mirror.spec`'s @code/rust altitude) I haven't audited. The shortest
path is GIVEN-WHAT-I-KNOW shortest; further audit may add gates.

### H3. Implicit-waiting items aren't promotion-bearing

§5's three implicit-waiting-to-be-explicit items are scout-flags ONLY.
The two-`peer`-types collision (§5.1) is the strongest candidate;
the `mirror.spec`-as-prism-of-project framing (§5.2) is medium
strength; the substrate-names-its-own-boundary framing (§5.3) is
recognition territory adjacent to #57 but I haven't traced the full
ancestry. Glint's reflection altitude adjudicates; this scout doesn't.

### H4. Anti-pattern flags are scout-altitude, not adversarial-altitude

§6's three anti-pattern flags differ from Seam's adversarial review
in posture. Seam asks "does this spec actually deliver what it
claims?" Scout asks "is the substrate pulling away from what we're
doing here?" Different questions. The Phase F mis-framing (§6.1) is
the strongest flag; the 1200-cap pull (§6.2) is medium; the stall-
pattern miscategorization (§6.3) is soft. Alex adjudicates the
brief's framings.

### H5. The slingshot recommendation is opinionated

§2 names ONE move as the slingshot. The substrate-pull-confidence is
high on that move, but the SHAPE of the slingshot (which combination
of closures earns the "single move" framing) is Taut's call. Reed
may see a sibling slingshot (e.g., land `shards/peer.mirror` first to
resolve the two-`peer`-types collision before `@mirror/pack` lands
on-top-of an underspecified host). The two paths converge at T2;
ordering of T0 vs T1 is genuinely a call.

### H6. Scout-role recognition call is hedged

The scout-role recognition call (§7) leans "weakly positive" but
flags low confidence. The naming may not survive 2-3 more sessions
if the role doesn't replicate. Honest framing.

### H7. This scout DID NOT propose @io/llm and DID NOT pursue Loop 2 work

Discipline-check per brief. Fate is the substrate's inference (Alex
2026-06-24); the transformer-gap essay is the structural negative; no
@io/llm framing surfaced. Loop 2 (cosmos-mirror, @spectral/db browser)
is not flagged anywhere. Per brief.

### Pack trail

- **2026-06-24 (Alex morning)**: directive for `@spectral/garden/git`
  + `pack{}` block design. Mara spawned for both specs.
- **2026-06-24 (Mara, morning-afternoon)**: spectral-garden-git
  spec (4 commits) + peer-ACL spec (initial 6 commits).
- **2026-06-24 (Alex midday cascade)**: vocabulary cascade
  (peer/supervisor/team → pack/lead/members); §10 reframe (sheaf →
  spawn-and-probe); §7.4 substrate-vs-USE clarification.
- **2026-06-24 (Mara cascade pass)**: peer-ACL spec lands cascade
  pass (3 more commits).
- **2026-06-24 (Reed lead-rename consolidation)**: 59fa1cd.
- **2026-06-24 (Alex naming Taut-as-scout)**: scout role articulated;
  this report is the first scout instance.
- **2026-06-24 (Taut + Seam parallel)**: Seam adversarial review of
  both specs (rear-guard); Taut substrate-pull scout (forward); this
  report.
- **Forward-promised**: Glint reflection essay (includes scout-role
  recognition candidate); Pack-altitude conversation on slingshot move
  (#2); Reed consolidation if slingshot ratified.

---

*Report end. Line count: ≤500 per brief discipline. Scout signed
Taut. Composes with Seam (a5bc686a4392c1d3c) at adversarial altitude.
Glint adjudicates scout-role recognition at reflection altitude.*
