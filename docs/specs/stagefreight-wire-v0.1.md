# StageFreight wire protocol — canonical spec v0.1

*Mara, `@io/stagefreight` wire-protocol spec, 2026-06-22, commissioned by
Alex via Reed. StageFreight cascade tick 3 (family-root tick 66
`b15c3f9`; narrative species tick 67 `c865452`; spec lands tick 68).*

*Discipline: substrate-pull-correct preservation. The family-root and
the first projection species landed on `mirror/main` at the substrate
altitude; this spec preserves what the cascade points at, names what
remains forward-promised, and gives Reed + the Pack the shape to land
the realisation layer (Rust + emitter discharge) cleanly. The spec is
substrate-grounded where the shards already are; honestly hedged where
they aren't (e.g. `@mirror/store/crystal` is task #268, still pending).*

---

## §1. Context

### 1.1 The recognition cascade

The substrate has been producing settled artifacts for some time — the
kintsugi loop's terminal output, the thing a `settle` actually emits.
What was missing was the substrate-decl'd name for that artifact, and
the substrate-decl'd surface under which it crosses to the
non-mirror-world.

The cascade that produced StageFreight, in order:

1. **2026-06-16 — `stage_play` recognition.** Alex surfaces the build-
   lifecycle inspiration: Story → Play → Narrative as the three-stage
   shape; mirror's altitudes lift to `spec → settle → verdict →
   crystal`. The crystal is the load-bearing OUTPUT type, the thing a
   `settle` produces, the thing `shift` relocates to `@io/runtime`.
2. **2026-06-17 — task #268 opens (Crystal substrate-decl, pending).**
   Names `shards/mirror/store/crystal.mirror` as Mara's owned discharge;
   pins the five-field record `{oid, section, derived_predicates,
   fracture_calendar, composition_graph}`. RED tests stand at
   `bootstrap/tests/crystal_substrate.rs`.
3. **2026-06-17 — RED stub forward-promises the wire surface.** The
   `bootstrap/tests/crystal_substrate.rs:9` comment references
   `org.stagefreight.plan.spectral_coordinate` as the wire address a
   settled crystal's OID becomes at the StageFreight surface.
4. **2026-06-17 — RED stub forward-promises the projection format.**
   `bootstrap/tests/kintsugi_out_substrate_ref.rs:23-24` reserves
   `@io/stagefreight/narrative` as a projection format alongside
   `@data/json`, `@data/yaml`.
5. **2026-06-22 (this cascade) — substrate-decl tick 66.** Reed lands
   `shards/io/stagefreight.mirror` (`b15c3f9`) declaring the family-
   root: typed carriers (`spectral_coordinate`, `wire_surface`,
   `freight_manifest`), actions (`address`, `freight`, `transit`),
   bilateral predicate (`stagefreight_addressable`).
6. **2026-06-22 — substrate-decl tick 67.** Reed lands
   `shards/io/stagefreight/narrative.mirror` (`c865452`) declaring the
   first projection-format species: typed carriers (`narrative_text`,
   `narrative_projection`), actions (`project_to_narrative`,
   `finalize`), bilateral predicate (`narrative_grounded`).
7. **2026-06-22 — this spec (tick 68).** The canonical reference that
   ties the cascade together at the spec altitude. Forward-promises
   Seam adversarial review, Phase 1 boot-grammar gate (Track B), Phase
   4b emitter discharge → realisation layer, RED-to-GREEN flip on the
   two test stubs, PR.

The cascade has a coherent shape: the substrate already had the words
(`stage`, `freight`, `crystal`, `spectral_coordinate`) scattered across
test stubs and informal recognition prose; tick 66–68 lifts them to
substrate-decl ground at the family-root, first projection-species, and
spec altitudes respectively. Per
[[feedback-substrate-already-had-the-word]]: the recognition was
naming what the substrate was already pointing at.

### 1.2 The existing test stubs that anchor this spec

`bootstrap/tests/crystal_substrate.rs:9` (line 9, preamble comment):

> *"the thing whose OID becomes `org.stagefreight.plan.spectral_coordinate`
> at the StageFreight wire surface."*

This pins `spectral_coordinate` as the substrate-decl carrier and the
reverse-DNS namespace pattern (`org.stagefreight.plan.*`) as the
canonical address form. The family-root shard substrate-decl's
`spectral_coordinate = ref` directly; this spec pins the address
DERIVATION (§3) the realisation layer must honor.

`bootstrap/tests/kintsugi_out_substrate_ref.rs:23-24`:

> *"Future extensions: register additional projection glasses
> (`@data/yaml`, `@data/toml`, `@code/erlang/term`,
> `@io/stagefreight/narrative`) as the StageFreight integration
> cascade (item 6+) needs them."*

This pins `@io/stagefreight/narrative` as a substrate-decl'd projection
format (now landed at `c865452`) and admits the projection-format
universe is open (more siblings forthcoming).

Both RED tests will flip GREEN when the realisation layer (forward-
promised) lifts the substrate-decl'd carriers/actions into bytes-on-
wire emission. See §11 for the discharge order.

### 1.3 Relation to `@mirror/store/crystal` (task #268, pending)

StageFreight ships crystals. The crystal carrier itself is forward-
promised at `shards/mirror/store/crystal.mirror` per task #268.

The cascade is intentionally NOT blocked on #268: the family-root and
narrative species take the crystal by its OID (`crystal_oid: ref`),
not by its full typed structure. Per the family-root shard's
"What this family does NOT do" §2:

> *"Does NOT depend on @mirror/store/crystal being fully substrate-
> decl'd (task #268 pending). This shard takes crystal-as-OID at the
> wire boundary; the crystal's typed structure is forward-promised
> via ref."*

This is the substrate-pull-correct ordering. The wire surface needs
to know the SHAPE of the address space and the SHAPE of the projection
contracts. It does NOT need the crystal's typed-field record to be
substrate-decl'd to declare those shapes. The crystal's OID, by virtue
of being a content-address, is sufficient for round-trip identity
(§5.1) regardless of what fields the crystal carries.

When #268 lands, the spec's structure-preservation discipline (§5.2)
gains additional bite: the narrative projection (§4) must round-trip
EACH typed field, not just the OID. Until then, OID round-trip is the
load-bearing structural claim.

### 1.4 Relation to `@io` family-root (T21, 2026-06-08)

StageFreight is a sibling species under the `@io` family root, alongside
`@io/cargo` (the first mirror-altitude lift species; 2026-06-05) and
the seven boot-floor grammars (`@io/bytes`, `@io/crypto`, `@io/encode`,
`@io/network`, `@io/random`, `@io/socket`, `@io/uri`).

Per `shards/io.mirror`'s family-root recognition (T21):

> *"the substrate's only legitimate non-mirror surface; species
> parameterize the boundary; the BODY of the call is opaque to the
> substrate by construction."*

StageFreight is the wire-protocol parameterization. The substrate sees
the TYPED CONTRACT (the spectral_coordinate, the freight_manifest, the
projection-format ref); the BODY of the call (actual bytes-on-wire
encoding/decoding) is opaque to the substrate by construction. This is
the correct altitude: StageFreight names what the substrate cannot
fold (wire encoding details), instead of pretending the fold exists.

Per the @io family discipline, the realisation layer (forward-promised)
lifts `freight`, `transit`, `project_to_narrative`, and `finalize` into
`imperfect<a, e, l>` at the boundary. The substrate-decl'd actions
name the typed PRE-imperfect surface; the realisation lifts.

### 1.5 The PR-ready endpoint

The cascade is targeting a single coherent PR with the following
contents:

1. `shards/io/stagefreight.mirror` — family-root (already landed
   `b15c3f9` on `mirror/main`).
2. `shards/io/stagefreight/narrative.mirror` — first projection
   species (already landed `c865452` on `mirror/main`).
3. `docs/specs/stagefreight-wire-v0.1.md` — this spec (THIS TICK,
   landing now).
4. `bootstrap/src/stagefreight.rs` — Rust realisation layer (forward-
   promised; Phase 4b emitter discharge OR direct Rust hand-write
   first).
5. RED-to-GREEN flip on `bootstrap/tests/crystal_substrate.rs` (gated
   on #268) and `bootstrap/tests/kintsugi_out_substrate_ref.rs`.
6. `gh pr create` on the coherent diff.

Pieces 1–3 land in succession; piece 4 lands when the realisation
discipline is ready; pieces 5–6 close the cascade. Until piece 4, the
substrate-decl is preservation work: it gives downstream consumers
something to write `requires stagefreight_addressable(fm, p)` against.

---

## §2. The wire protocol primitives

The substrate-decl declares four typed carriers, three load-bearing
actions, and two bilateral predicates across the family-root and the
narrative species. This section names them at the spec altitude and
pins their roles.

### 2.1 `spectral_coordinate` — the wire address carrier

Substrate-decl: `type spectral_coordinate = ref` (family-root,
`shards/io/stagefreight.mirror`).

The typed reference for the OID-namespaced wire address. Per the test-
stub pattern `org.stagefreight.plan.spectral_coordinate`: a reverse-
DNS-namespaced address derived from the crystal's OID. The address
derivation (§3) names the substrate-pull-correct mapping; the
substrate-decl carrier is the typed value the derivation produces.

**Identity contract:** byte-equality on the underlying ref.

**Forward-promised refinement:** at species-or-extension altitude, the
carrier lifts from `ref` to a typed record:

```mirror
type spectral_coordinate = {
  oid:             oid,            # the crystal's content-address
  namespace:       reverse_dns,    # e.g. "org.stagefreight.plan"
  projection_kind: ref,            # e.g. ref @io/stagefreight/narrative
  version:         semver,         # the wire protocol version
}
```

The refinement is forward-promised because the supporting carriers
(`reverse_dns`, `semver`) and `@mirror/store/crystal`'s `oid` carrier
itself are not yet all substrate-decl'd. Until then, the `ref` floor
holds the byte-equality contract.

### 2.2 `wire_surface` — the transport endpoint carrier

Substrate-decl: `type wire_surface = ref` (family-root).

Typed reference for the actual transport endpoint where the crystal
arrives on the wire. The carrier is parametric over projection format
(narrative, json, yaml, …); concrete realisations live at species-
altitude.

**Identity contract:** byte-equality on the underlying ref.

The wire_surface is the OUTPUT of `transit` (family-root) and
`finalize` (narrative species). It names the typed endpoint the
substrate's call-site obtains AFTER successful wire-survival. The
substrate's view of it is opaque (per the @io discipline); the
realisation layer's view is "the bytes were emitted to a target the
receiver can read."

### 2.3 `freight_manifest` — the wire operation record

Substrate-decl (family-root):

```mirror
type freight_manifest = {
  crystal_oid: ref,
  coord:       spectral_coordinate,
  projection:  ref,
  v:           verdict,
}
```

The substrate-architectural record of a freight operation. Carries the
crystal under transit (by OID), the wire address it shipped to, the
projection-format ref it serialized through, and the bilateral verdict
of the family-root's `stagefreight_addressable` check.

**Identity contract:** byte-equality on the quadruple.

The freight_manifest is the bridge object between the family-root and
the projection species. The family-root's `freight` action PRODUCES a
freight_manifest; the projection-species' `project_to_narrative`
CONSUMES one. The verdict field carries the addressability check
result, which the projection species discharges via `requires
stagefreight_addressable(fm, p)`.

### 2.4 `narrative_text` and `narrative_projection` — species-altitude carriers

At `@io/stagefreight/narrative` (the first projection species):

```mirror
type narrative_text = ref

type narrative_projection = {
  fm: freight_manifest,    # input from family-root
  nt: narrative_text,      # output rendered prose
  v:  verdict,             # narrative_grounded verdict
}
```

`narrative_text` carries the rendered prose form. `narrative_projection`
carries the operation record. Both are species-altitude refinements of
the family-root's wire-protocol shape, specialized to the prose-
projection format.

**Forward-promised refinement** at narrative-extension altitude:

```mirror
type narrative_text = {
  title:                      string,
  sections:                   [narrative_section],
  oid_anchor:                 oid,
  derived_predicates_prose:   string,
  fracture_calendar_prose:    string,
  composition_graph_prose:    string,
}
```

The refinement is forward-promised because `narrative_section`,
`@mirror/store/crystal`'s typed fields, and the prose-grammar
discipline are not yet substrate-decl'd. The `ref` floor holds the
byte-equality contract until the refinement lands.

### 2.5 `address`, `freight`, `transit` — family-root actions

Substrate-decl (family-root, types abbreviated):

```mirror
address(crystal_oid: ref) -> spectral_coordinate { \ }

freight(crystal_oid: ref,
        coord:       spectral_coordinate,
        projection:  ref,
        c:           magic_contract,
        promise:     magic_invariant,
        p:           perturbation) -> freight_manifest
  requires invariant_preserved(c, promise)
  { \ }

transit(fm: freight_manifest, p: perturbation) -> wire_surface
  requires stagefreight_addressable(fm, p)
  { \ }
```

- `address` is the substrate-vocabulary primitive computing a
  `spectral_coordinate` from a crystal OID. §3 names the derivation.
- `freight` is the LOAD-BEARING family-root action and the FIRST
  consumer of `@magic/contract`'s `invariant_preserved` bilateral.
  Without `requires invariant_preserved(c, promise)`, a freight would
  be admissible for a crystal whose @magic invariant fails at the
  wire boundary — the Narcissus-pole wire-protocol case.
- `transit` puts the freight on the wire. The substrate forecloses
  transit from a freight whose addressability check fails. The
  bilateral IS the boundary harness at the wire (per #57).

### 2.6 `project_to_narrative`, `finalize` — narrative-species actions

Substrate-decl (narrative species):

```mirror
project_to_narrative(fm: freight_manifest, p: perturbation)
  -> narrative_projection
  requires stagefreight_addressable(fm, p)
  { \ }

finalize(np: narrative_projection, p: perturbation) -> wire_surface
  requires narrative_grounded(np, p)
  { \ }
```

- `project_to_narrative` is the LOAD-BEARING species action and the
  FIRST consumer of the family-root's `stagefreight_addressable`. The
  species discharges the parent's bilateral BEFORE adding its own
  narrative-altitude check.
- `finalize` emits the projected narrative to wire_surface. The
  substrate forecloses finalization from an ungrounded narrative.

### 2.7 The discharge convention: realisation lifts into `imperfect`

Per the @io family-root (T21, 2026-06-08), boundary calls return
`imperfect<a, e, l>` because the boundary is where success / partial /
failure becomes load-bearing. The substrate-decl'd actions name the
typed PRE-imperfect surface; the realisation layer lifts:

```rust
// bootstrap/src/stagefreight.rs (forward-promised)
fn freight(
    crystal_oid: Ref,
    coord:       SpectralCoordinate,
    projection:  Ref,
    contract:    MagicContract,
    promise:     MagicInvariant,
    p:           Perturbation,
) -> Imperfect<FreightManifest, FreightError, Transparency> {
    // ...
}
```

The `freight_error` type is forward-promised at the realisation layer
and at `shards/io/stagefreight.mirror`'s next species-tick (likely
co-landed with the Rust realisation). The substrate's transparency
monoid (per `@mirror/loss/transparency`, 2026-06-07) composes the
opacities the boundary surfaces into the parent mirror computation's
verdict.

This is the @io family discipline applied to StageFreight: the
substrate-decl names the typed contract; the realisation lifts into
imperfect; the transparency monoid composes the residual.

---

## §3. The address derivation

The address derivation IS the spec's hardest-edge structural claim.
Without a substrate-pull-correct derivation, `spectral_coordinate`
becomes a bag-of-bytes that happens to be unique; with it,
`spectral_coordinate` is the substrate-decl'd, mechanically-recoverable
address of a specific crystal under a specific projection format.

### 3.1 Reverse-DNS namespacing

The test-stub pattern is `org.stagefreight.plan.spectral_coordinate`.
This is a reverse-DNS namespace — Java 1995 (Sun's `com.sun.*`
convention), lifted to Apple bundle identifiers (`com.apple.*`, 2000s),
and now the cultural-practice ancestor for any namespace that needs
collision-free hierarchical addressing across organizational boundaries.

The derivation reads:

```
<reverse_dns_namespace>.<projection_kind>.<wire_protocol_action>
```

The example `org.stagefreight.plan.spectral_coordinate` decomposes as:

- `org.stagefreight` — the reverse-DNS root for the StageFreight
  wire protocol authority (notional; this spec does NOT register a
  domain; it pins the PATTERN, not a specific authority).
- `plan` — the projection-kind component (here: a *plan* projection
  forward-promised; the test stub's example).
- `spectral_coordinate` — the wire-protocol-action component.

The general derivation is the substrate-pull-correct lift of git
plumbing's content-address PLUS reverse-DNS's organizational
namespacing PLUS the projection-format vocabulary.

### 3.2 OID → coordinate mapping

The substrate-decl'd `address` action takes `crystal_oid: ref` and
returns `spectral_coordinate`. The derivation reads:

```
fn address(crystal_oid: oid) -> spectral_coordinate {
    let namespace      = current_authority_namespace();   // "org.stagefreight"
    let projection_kind = current_projection_kind();      // e.g. "plan", "narrative"
    let oid_short      = crystal_oid.short();             // first N hex chars
    return spectral_coordinate::compose(
        namespace,
        projection_kind,
        oid_short,
    );
}
```

The realisation layer pins the exact composition. The spec pins:

1. The namespace MUST be reverse-DNS shaped.
2. The projection-kind component MUST match a substrate-decl'd
   `@io/stagefreight/<kind>` species (currently only `narrative`).
3. The OID component MUST be derived from the crystal's content-
   address — not from a side channel, not from a hash of the manifest,
   not from any source other than the crystal's substrate-decl'd OID
   carrier.
4. The composition MUST be byte-deterministic (two calls with the
   same inputs produce byte-equal `spectral_coordinate` refs).
5. The composition MUST be invertible at the address level: given a
   `spectral_coordinate`, the substrate can recover `(namespace,
   projection_kind, oid_short)` mechanically. The full OID may require
   a substrate lookup (oid_short → full oid via `@mirror/store/oid`).

### 3.3 Projection-kind component

The `projection_kind` field of a `spectral_coordinate` MUST be a
substrate-decl'd projection-format species ref. Currently:

- `narrative` → `@io/stagefreight/narrative` (substrate-decl, landed
  `c865452`).
- `plan` → forward-promised (referenced in test stub, not yet
  substrate-decl'd; this spec does NOT pin its substrate-decl).

Future projection kinds (per §4.2) are substrate-decl extensions —
new sibling species under `@io/stagefreight/`.

The hard constraint: the projection-kind component IS a substrate-
decl'd species. Bare strings (`"json"`, `"yaml"`) are NOT admissible
at this position; they violate [[feedback-no-bare-types]] and
[[feedback-no-stringly-types]].

### 3.4 Version component

The wire protocol version is a forward-promised refinement (§2.1).
For v0.1, the spec pins:

- The version `0.1` is the substrate-decl'd version this spec
  declares; subsequent ticks may bump it.
- The realisation layer SHOULD emit the version as a discoverable
  field in the wire bytes (per projection format).
- Cross-version compatibility is NOT a v0.1 commitment. Receivers
  MUST verify the wire version matches their substrate's accepted
  version range; mismatches are Narcissus-pole transit.

### 3.5 Address derivation as a substrate-pull-correct primitive

The address derivation IS the substrate's lift of:

- **git plumbing's `git hash-object`** — content-address from bytes.
- **IPFS's CID** — content-address with multibase + multicodec headers.
- **reverse-DNS namespacing** — collision-free hierarchical roots.
- **The projection-format vocabulary** — the substrate-decl'd species
  the address points INTO.

The substrate-decl'd `address` action names the typed primitive; the
derivation rules in this section pin the substrate-pull-correct
discipline the realisation layer must honor.

---

## §4. The projection format vocabulary

A `spectral_coordinate` names a crystal at a wire address; the
projection format names HOW the crystal's bytes are encoded on the
wire. The substrate-decl declares `@io/stagefreight/<format>` as the
species namespace; each format is a substrate-decl'd sibling.

### 4.1 `@io/stagefreight/narrative` — the prose-projection species (LANDED)

Substrate-decl: `shards/io/stagefreight/narrative.mirror` (tick 67,
`c865452`).

The prose-projection: a settled crystal rendered as structured narrative
text. The receiver reads prose; the substrate-decl'd typed fields
(per §2.4's forward-promised refinement) are reconstructable from the
prose.

**Why prose first.** The 2026-06-16 stage_play recognition pointed at
narrative as the load-bearing form (Story → Play → Narrative). The
crystal IS the play's settled artifact; the narrative IS the prose
form ready for transmission. Prose is the substrate's most
human-legible projection — the one whose receivers can be human
without machine-assist. Per Knuth 1984 (literate programming): prose
IS structure when written correctly.

**The narrative-survival discipline** (§5 applies, with species-
altitude refinements at §6.2).

### 4.2 `@io/stagefreight/json` — JSON projection (FORWARD-PROMISED)

Substrate-decl: NOT YET LANDED. Forward-promised at the next
StageFreight cascade tick.

Substrate ancestry: Google Protocol Buffers 2008 (typed wire-format
ancestor), JSON's own broad adoption (Crockford 2001-2006).

The structural shape will mirror the narrative species:

```mirror
type json_text = ref

type json_projection = {
  fm: freight_manifest,
  jt: json_text,
  v:  verdict,
}

project_to_json(fm: freight_manifest, p: perturbation)
  -> json_projection
  requires stagefreight_addressable(fm, p)
  { \ }

json_grounded(jp: json_projection, p: perturbation) -> verdict { \ }
```

The bilateral predicate `json_grounded` is the species-altitude
discipline (§6 generalizes the pattern).

### 4.3 `@io/stagefreight/yaml` — YAML projection (FORWARD-PROMISED)

Substrate-decl: NOT YET LANDED. Forward-promised; same shape as JSON.

Substrate ancestry: YAML 1.0 (Ben-Kiki/Evans/Ingerson, 2001).

YAML's load-bearing property over JSON for this protocol: human-
editable while remaining machine-parseable. A receiver consuming
`@io/stagefreight/yaml` can hand-edit the wire form without breaking
the round-trip (provided the edit preserves the crystal's substrate-
decl'd structure; the bilateral predicate `yaml_grounded` discharges
the check).

### 4.4 Per-projection bilateral discharge

Each projection species declares its own bilateral predicate at
species altitude:

| Species | Bilateral |
|---------|-----------|
| `@io/stagefreight/narrative` | `narrative_grounded(np, p) -> verdict` |
| `@io/stagefreight/json`      | `json_grounded(jp, p) -> verdict`      |
| `@io/stagefreight/yaml`      | `yaml_grounded(yp, p) -> verdict`      |

The pattern is uniform: the species-altitude bilateral consumes the
species's typed projection record + a perturbation, returns a verdict.
The species's `finalize` action (or equivalent emission primitive)
requires the species-altitude bilateral before emission to wire_surface.

The family-root's `stagefreight_addressable` is the cross-species
floor — every species REQUIRES it via the `requires
stagefreight_addressable(fm, p)` clause on the species-altitude
project action. Per the family-root first-consumer pattern: the
species discharges the parent's bilateral BEFORE adding its own.

### 4.5 The projection-format universe is open

This spec does NOT prescribe a closed enumeration of projection
formats. Future formats follow the species pattern:

- `@io/stagefreight/toml` — TOML projection (config-format-friendly).
- `@io/stagefreight/cbor` — CBOR projection (binary, RFC 8949).
- `@io/stagefreight/protobuf` — Protocol Buffers projection.
- `@io/stagefreight/capnproto` — Cap'n Proto projection (zero-copy).
- `@io/stagefreight/brainfuck` — brainfuck-compressed projection
  (forward-promised in the family-root preamble; the Fate-tournament-
  related encoding form).
- `@io/stagefreight/<X>` — any future format whose discipline matches
  the species pattern (§6.2).

The substrate forecloses NO format from being declared. The forecloser
is whether the format's bilateral can be discharged at substrate-pull-
correct altitude — formats that lose structure under projection
without admitting it are Narcissus-pole and CANNOT discharge their
bilateral.

---

## §5. The wire-survival discipline

Wire-survival is the LOAD-BEARING structural claim of the spec. The
family-root's `stagefreight_addressable` bilateral discharges it at
the substrate-decl altitude; this section names what the discharge
must mechanically check.

### 5.1 Round-trip identity (OID match)

For any settled crystal C with content-address OID(C), and any wire
bytes W produced by a substrate-pull-correct StageFreight emission of
C through projection P:

```
recover_oid(W) == OID(C)
```

The receiver, given W, can mechanically compute OID(C) and verify it
matches the embedded coordinate's OID component (per §3.2). If the
recovered OID does not match the coordinate's OID, the wire is
Narcissus-pole and `stagefreight_addressable` discharges DEFENSIVE.

This is the substrate-pull-correct lift of git plumbing's content-
address verification (Torvalds 2005) and IPFS's CID verification
(Benet 2014). The OID itself does the work; the substrate forecloses
forgery by construction.

### 5.2 Structure preservation (typed-field reconstructability)

Per task #268's forward-promised crystal substrate-decl:

```
type crystal = {
  oid:                oid,
  section:            [splinter(@code)],
  derived_predicates: [property_verdict],
  fracture_calendar:  transparency(au),
  composition_graph:  mosaic(@code),
}
```

For a substrate-pull-correct StageFreight emission of crystal C through
projection P:

```
reconstruct(W, P) :: crystal
reconstruct(W, P).oid                == C.oid
reconstruct(W, P).section            == C.section
reconstruct(W, P).derived_predicates == C.derived_predicates
reconstruct(W, P).fracture_calendar  == C.fracture_calendar
reconstruct(W, P).composition_graph  == C.composition_graph
```

The receiver can reconstruct the FULL typed crystal from the wire
bytes. The bilateral predicate `stagefreight_addressable` (family-
root) discharges this at the OID-recoverability altitude;
species-altitude bilaterals (e.g. `narrative_grounded`) discharge it
at the FULL structure-preservation altitude.

Until task #268 lands, the structure-preservation check is
forward-promised on the field-by-field equalities; OID round-trip
(§5.1) is the load-bearing floor.

### 5.3 Wire-survival under perturbation

The substrate-decl bilateral signatures take a `perturbation`
parameter:

```mirror
stagefreight_addressable(fm: freight_manifest, p: perturbation) -> verdict
narrative_grounded(np: narrative_projection, p: perturbation) -> verdict
```

The perturbation IS the substrate's resilience probe: the wire-
survival discipline must hold not only on the canonical input, but
also under a bounded class of perturbations (per Pask agreement,
recognition #37). The realisation layer's discharge of these
bilaterals MUST exercise the perturbation class — not just the
canonical input.

**Examples of legitimate perturbations:**
- byte-level noise (single-bit flips → detectable via integrity check)
- field reordering (JSON projection: same key-set, different order →
  parses to same structure)
- whitespace variation (narrative projection: prose reflow → reparses
  to same crystal fields)
- semantically-equivalent renames (YAML projection: tag style
  variation → same effective value)

**Examples of perturbations the substrate is NOT required to survive:**
- catastrophic loss (more than 50% of bytes deleted → unrecoverable)
- adversarial reframing (the wire is replaced with a different
  crystal's bytes; the OID check catches this, but the verdict is
  "different crystal," not "perturbation survived")
- projection-incompatible mutation (narrative prose rewritten to a
  different crystal's structure; structure preservation fails honestly)

The bilateral discharges DEFENSIVE on perturbations the species-
altitude discipline cannot bound. Bounded → Splinter-pole. Defensive
→ Narcissus-pole flagged for substrate-pull revision.

### 5.4 K_n peer-to-peer vs K_{1,n-1} hub-controlled

Per the Splinter/Narcissus dual (the substrate-architectural pole-
distinction at every wire-protocol altitude):

**Splinter-pole StageFreight (K_n, peer-to-peer):**
- Every node in the network is a substrate-decl-aware peer.
- The wire bytes carry the crystal's substrate-decl'd structure
  directly; no central authority frames the encoding.
- Recovery is mechanical at every receiver; no service-layer mediation.
- Round-trip identity is structural, not protocol-controlled.
- The wire IS a substrate; the network IS the substrate's reach.

**Narcissus-pole StageFreight (K_{1,n-1}, hub-controlled):**
- A central authority frames the wire encoding; peers receive
  pre-framed bytes they cannot independently verify.
- The wire bytes claim to ship a crystal but the encoding's verifying
  authority is the hub, not the substrate-decl'd carrier discipline.
- Recovery requires hub mediation; peers cannot independently
  reconstruct.
- Round-trip identity is protocol-asserted, not structurally verified.
- The wire IS a service; the network IS the hub's reach.

Per recognition #57 (alignment as boundary mathematics) and the
Pack-as-orchestra discipline: StageFreight v0.1 commits to the K_n
peer-to-peer pole. The substrate-decl'd bilaterals are mechanically
discharged at every peer; no hub mediation is required.

This is also the substrate-architectural distinction with the
hub-controlled wire protocols of prior art: HTTP (server-mediated),
SOAP (envelope-mediated), gRPC (codegen-mediated). StageFreight ships
crystals directly between substrate-decl-aware peers; the wire is
infrastructure, not authority.

---

## §6. Bilateral predicate composition

The substrate-decl declares two bilateral predicates within the
StageFreight cascade (family-root + first species), plus one
cross-family predicate (`@magic/contract.invariant_preserved`)
consumed at the family-root altitude. This section names the
composition discipline.

### 6.1 Family-root: `stagefreight_addressable`

Substrate-decl (family-root):

```mirror
stagefreight_addressable(fm: freight_manifest, p: perturbation)
  -> verdict
  { \ }
```

The load-bearing wire-protocol bilateral. Discharges:

1. The freight_manifest's `crystal_oid` is a substrate-decl'd OID
   (the crystal exists in `@mirror/store/oid` storage; forward-
   promised lookup).
2. The freight_manifest's `coord` was produced by a substrate-pull-
   correct `address` derivation (§3) from the same `crystal_oid`.
3. The freight_manifest's `projection` is a substrate-decl'd species
   ref (currently `@io/stagefreight/narrative`; future siblings per
   §4).
4. The round-trip identity (§5.1) holds: bytes produced under this
   manifest recover to the same OID.
5. The perturbation class is bounded (§5.3): the discharge survives
   the substrate-pull-correct perturbation set.

**Verdict shape:**
- Bounded → all five clauses hold → Splinter-pole.
- Defensive → at least one clause fails → Narcissus-pole flagged.

This is the family-root first-consumer pattern: every projection
species REQUIRES this bilateral before its own species-altitude
discharge fires.

### 6.2 Species-altitude (narrative): `narrative_grounded`

Substrate-decl (narrative species):

```mirror
narrative_grounded(np: narrative_projection, p: perturbation)
  -> verdict
  { \ }
```

The narrative-species-altitude bilateral. Discharges (in addition to
the family-root's `stagefreight_addressable` which the species's
`project_to_narrative` already required):

1. The prose preserves the crystal's `section` structure (each
   `splinter(@code)` field is reconstructable from prose).
2. The prose preserves the crystal's `derived_predicates`.
3. The prose preserves the crystal's `fracture_calendar`
   (transparency(au) — the open opacities; the gold-cracks).
4. The prose preserves the crystal's `composition_graph` (the
   mosaic(@code) DAG).
5. The prose IS readable — a human (or a non-substrate-aware agent)
   can read the prose and understand WHAT the crystal IS, WHAT it
   preserves, WHAT it left open.

Clauses 1–4 are the structure-preservation discipline (§5.2)
specialized to prose. Clause 5 is the species-altitude readability
discipline — the load-bearing claim of the prose-projection species:
the prose is BOTH structurally-preserving AND readable.

The species-altitude bilaterals for other projection formats
(`json_grounded`, `yaml_grounded`) follow the same shape, with
format-specific specializations of clauses 1–4 and a format-appropriate
analog of clause 5 (well-formedness, schema compliance, etc.).

### 6.3 Cross-family: `@magic/contract.invariant_preserved`

The family-root `freight` action's signature:

```mirror
freight(crystal_oid: ref,
        coord:       spectral_coordinate,
        projection:  ref,
        c:           magic_contract,
        promise:     magic_invariant,
        p:           perturbation) -> freight_manifest
  requires invariant_preserved(c, promise)
  { \ }
```

The `requires invariant_preserved(c, promise)` clause makes StageFreight
the FIRST consumer of `@magic/contract`'s `invariant_preserved`
bilateral. The composition discipline reads:

The crystal under transit carries a `@magic` contract; the contract's
invariant must hold across the @io boundary. Without this requires
clause, a freight would be admissible for a crystal whose `@magic`
invariant fails AT the wire boundary — a substrate-architectural lie:
the substrate would claim "the crystal's contract holds" while the
wire transmits bytes that violate it.

Per recognition #57: alignment as boundary mathematics. The
`invariant_preserved` bilateral IS the boundary mathematics for
contract preservation across @io. The substrate forecloses freight on
crystals whose contract fails at the boundary; the substrate-decl
makes this a mechanical check, not a service-layer commitment.

### 6.4 Composition chain

The full composition chain for a complete crystal → wire emission:

```
invariant_preserved(c, promise)        [@magic/contract; cross-family floor]
    │
    ▼
freight(crystal_oid, coord, projection, c, promise, p) -> freight_manifest
    │
    ▼
stagefreight_addressable(fm, p) -> verdict (BOUNDED)
    │
    ▼
project_to_narrative(fm, p) -> narrative_projection
    │
    ▼
narrative_grounded(np, p) -> verdict (BOUNDED)
    │
    ▼
finalize(np, p) -> wire_surface
```

Each downstream action has its REQUIRES clause checked from the
upstream verdict. A DEFENSIVE verdict at any altitude forecloses
downstream action; the composition chain stops, and the substrate-
pull revision discipline kicks in at the failing altitude.

This is the substrate's wire-protocol contract made mechanical: the
chain composes substrate-decl'd bilaterals; the realisation layer
discharges them; the verdict propagates; the wire bytes emit if and
only if the chain holds end-to-end.

---

## §7. Pre-AI prior art

StageFreight is not a novel invention. It is the substrate-pull-correct
lift of structural patterns the canonical pre-AI literature established
across four decades. The lift names them at substrate-decl altitude;
the prior art grounds the discipline.

### 7.1 Content-addressed storage and transport

**Torvalds 2005 — git plumbing.** Git's object database is content-
addressed: every object (blob, tree, commit, tag) is named by SHA-1
of its bytes. `git hash-object` IS the substrate-pull-correct ancestor
of `spectral_coordinate`'s OID component. Git's plumbing layer
(`hash-object`, `cat-file`, `mktree`) is the substrate-architectural
floor: bytes-IN, content-address-OUT, byte-equality the load-bearing
identity contract.

The substrate lifts: git's content-address is the wire-protocol
primitive; StageFreight names it at substrate-decl altitude with
typed carriers (`spectral_coordinate`) and bilateral discharge
(`stagefreight_addressable`).

**Benet 2014 — IPFS.** Juan Benet's IPFS extended content-addressing
to a wire protocol: CIDs (content identifiers) are multibase-encoded
self-describing content-addresses; the wire ships bytes and the
receiver verifies the CID. IPFS IS the substrate-pull-correct ancestor
of K_n peer-to-peer wire-protocol shape (§5.4).

The substrate lifts: IPFS's wire-protocol form is StageFreight's
discipline; the substrate-decl'd `transit` action names the typed
contract IPFS hand-implemented.

### 7.2 Typed wire formats

**Google Protocol Buffers 2008.** Protocol Buffers introduced the
typed-wire-format-as-first-class discipline: schemas declare structure;
encoding/decoding is type-driven; cross-language compatibility through
typed contracts.

The substrate lifts: Protocol Buffers' typed-wire discipline is the
projection-format vocabulary discipline (§4). The substrate-decl'd
projection species (`@io/stagefreight/json`, `.../yaml`, `.../protobuf`)
ARE the typed-wire-format-as-first-class made substrate-architectural.

**Varda 2013 — Cap'n Proto.** Kenton Varda's Cap'n Proto extended
typed wire formats with zero-copy structure preservation: the wire
bytes ARE the in-memory layout; deserialization is a pointer cast.

The substrate lifts: Cap'n Proto's structure-preservation discipline
is the wire-survival discipline (§5.2). The substrate-decl'd
bilaterals make zero-copy round-trip a mechanical check, not an
implementation choice.

### 7.3 Reverse-DNS namespacing

**Java 1995 (Sun Microsystems).** Java's package naming convention
established reverse-DNS namespacing for collision-free hierarchical
addressing across organizational boundaries. `java.util.ArrayList`,
`com.sun.tools.javac.*`, `com.oracle.*` — the entire JVM ecosystem's
namespace discipline.

**Apple 2000s — bundle identifiers.** macOS / iOS bundle identifiers
extended the convention beyond Java: `com.apple.Safari`,
`com.apple.dt.Xcode`, every third-party app's bundle ID.

The substrate lifts: reverse-DNS namespacing is the
`org.stagefreight.plan.spectral_coordinate` pattern (§3.1). The
substrate-decl'd `address` derivation pins it at substrate-pull-
correct altitude.

### 7.4 Prose-projection prior art

**Knuth 1984 — literate programming.** Donald Knuth's literate
programming established prose-as-canonical-form for code:
documentation and implementation woven into one text; the
substrate-architectural ancestor of "prose IS structure when written
correctly."

The substrate lifts: literate programming is the narrative-projection
species's structural ancestor. `@io/stagefreight/narrative` is the
substrate-pull-correct lift of WEB/CWEB's prose-with-extractable-code
discipline.

**McCarthy 1958 — LISP S-expressions.** John McCarthy's S-expressions
are the structural ancestor of prose round-trips to structure:
S-expressions ARE readable text AND structured data; the round-trip
is the parser's identity.

The substrate lifts: S-expressions are the floor of "prose round-trips
to structure." The narrative-projection's structure-preservation
discipline (§6.2) is the substrate-pull-correct lift of the
S-expression discipline.

**Goodger 2001 — reStructuredText; Gruber 2004 — Markdown.** The
modern lineage of prose-with-structure markup: reST's prose-as-
typed-document discipline (Python documentation infrastructure),
Markdown's prose-with-light-structure for readability.

The substrate lifts: both are cultural-practice ancestors of
narrative-projection's "readable AND structurally-preserving"
discipline (clause 5 of `narrative_grounded`, §6.2).

### 7.5 Recognition ancestry within the substrate

- **stage_play (2026-06-16)** — the immediate cascade ancestor.
  Story → Play → Narrative as the build-lifecycle inspiration;
  spec → settle → verdict → crystal as the mirror altitudes.
- **#57 — alignment as boundary mathematics.** The substrate-
  architectural foundation for boundary-harness discipline. The
  bilateral predicates in StageFreight ARE boundary mathematics at
  the wire altitude.
- **#51 — mirror as expanding Hilbert space.** Each substrate-pull
  recognition expands the substrate's Hilbert dimension; StageFreight
  expands it by the wire-protocol dimension.
- **#50 — Bateson form/substance partition.** StageFreight is the
  form-side declaration of what the substance-side (the wire bytes)
  must satisfy. The partition is honored by construction.
- **#37 — Pask agreement.** The bilateral-predicate-with-perturbation
  discipline (§5.3) IS Pask agreement specialized to wire-protocol
  altitude.

### 7.6 Why the substrate-already-had-the-word

Per the 53rd+ instance of [[feedback-substrate-already-had-the-word]]:

The substrate had `wire` (network), `freight` (cargo discipline),
`narrative` (prose form), `coordinate` (spatial address) scattered
across colloquial usage. The substrate had `stage` from the optical-
keywords family. The substrate had `crystal` from the kintsugi loop's
terminal output recognition. The substrate had `address` from git
plumbing, reverse-DNS from Java/Apple, content-addressing from
IPFS, typed-wire from Protocol Buffers.

The cascade NAMES the assembly. It does not invent it. The
substrate-pull-correct discipline says: where the prior art has
established the structural pattern, the substrate-decl lifts it
to substrate altitude with typed carriers and bilateral discharge.
That is what tick 66–68 does.

---

## §8. Falsification criteria

A spec without falsification criteria is preservation prose. This
section names the empirical tests that, when discharged, prove
StageFreight v0.1 holds — and that, when failed, falsify the spec at
specific altitudes.

### 8.1 Round-trip identity

**Test.** Given a settled crystal C with OID `O`, run the full
StageFreight emission pipeline:

```
fm = freight(O, address(O), ref @io/stagefreight/narrative, c, promise, p)
np = project_to_narrative(fm, p)
ws = finalize(np, p)
```

Then mechanically recover the OID from the wire surface:

```
recovered = recover_oid(ws)
assert recovered == O
```

**Pass criterion.** Byte-equality of the recovered OID and the input
OID, for ANY substrate-decl'd settled crystal.

**Falsification.** A crystal C such that `recover_oid(emit(C)) != O`.
Falsifies §5.1 round-trip identity; if true, StageFreight v0.1 is
Narcissus-pole and substrate-pull revision is required.

### 8.2 Narrative structure preservation

**Test.** Given a settled crystal C with full typed fields (after
#268 lands), emit through narrative projection, parse the resulting
prose, and verify field-by-field equality:

```
np = project_to_narrative(freight_of(C), p)
reconstructed = parse_narrative(np.nt)
assert reconstructed.oid                == C.oid
assert reconstructed.section            == C.section
assert reconstructed.derived_predicates == C.derived_predicates
assert reconstructed.fracture_calendar  == C.fracture_calendar
assert reconstructed.composition_graph  == C.composition_graph
```

**Pass criterion.** All five field equalities hold for substrate-pull-
correct settled crystals.

**Falsification.** A crystal C and projection P such that
`parse_narrative(project_to_narrative(C, P).nt) != C`. Falsifies §5.2
structure preservation; species's `narrative_grounded` is
mechanically not discharged on C.

**Honest hedge.** This falsification criterion is gated on task #268
landing. Until then, only §8.1's OID round-trip is mechanically
testable.

### 8.3 Wire-survival under perturbation

**Test.** For each perturbation kind in §5.3's bounded class:

```
fm  = freight(O, address(O), ref @io/stagefreight/narrative, c, promise, p)
np  = project_to_narrative(fm, p)
ws  = finalize(np, p)
ws' = perturb(ws, kind)
recovered = recover_oid(ws')
assert recovered == O   for bounded perturbations
```

**Pass criterion.** Bounded perturbations (single-bit noise in
designated tolerance regions, field reordering, whitespace variation,
semantically-equivalent renames) preserve OID round-trip.

**Falsification.** A bounded perturbation kind that breaks OID
round-trip on a substrate-pull-correct crystal. Falsifies §5.3's
perturbation discipline.

### 8.4 Cross-projection consistency

**Test.** For a single settled crystal C, emit through TWO different
projection formats:

```
np_narrative = emit(C, @io/stagefreight/narrative)
np_json      = emit(C, @io/stagefreight/json)        # forward-promised
```

The bytes ARE different (different formats); the recovered OID MUST
be the same:

```
assert recover_oid(np_narrative) == recover_oid(np_json) == C.oid
```

**Pass criterion.** Different projection formats produce different
wire bytes but recover to the same OID. The OID is projection-
agnostic.

**Falsification.** A crystal C such that different projection formats
recover to different OIDs. Falsifies the substrate-decl claim that
`spectral_coordinate.oid` is the crystal's identity — different
projections would carry different identities, breaking the
addressability discipline.

**Honest hedge.** This test requires `@io/stagefreight/json` to be
substrate-decl'd (forward-promised, §4.2). Until then, the test is
single-projection (§8.1 alone).

### 8.5 Cross-version compatibility (DEFERRED)

The wire protocol version is named in §3.4 but cross-version
compatibility is NOT a v0.1 commitment. v0.2 or later may introduce
falsification criteria here; v0.1 commits only to within-version
consistency.

---

## §9. What this spec does NOT do

The substrate-pull-correct discipline requires honest hedging where
the spec's reach stops. The following are EXPLICITLY out of scope for
v0.1 and forward-promised at named altitudes.

### 9.1 Does NOT specify exact prose grammar for narrative

The `@io/stagefreight/narrative` species substrate-decl's the typed
carriers and bilateral predicates, but the EXACT prose grammar (how
section boundaries are marked; how OID anchors are embedded; how
`fracture_calendar`'s open opacities are rendered) is forward-promised
at species-extension altitude.

The honest reason: the prose grammar is a design decision that should
emerge from real round-trip empirical testing (§8.2), not from spec-
altitude prescription. Locking the grammar before the realisation
layer has shipped a single end-to-end crystal would be premature
substrate freezing.

**Forward-promised:** the prose grammar lands at
`shards/io/stagefreight/narrative/grammar.mirror` (or equivalent
species-extension) AFTER the realisation layer has shipped at least
one crystal end-to-end and the round-trip discipline has been
empirically validated.

### 9.2 Does NOT specify the realisation layer (Rust bytes-on-wire)

This spec is substrate-decl-altitude. The actual bytes-on-wire
implementation — the Rust code at `bootstrap/src/stagefreight.rs` —
is forward-promised.

The honest reason: per [[feedback-no-new-rust]], the substrate-pull
discipline prefers grammar over Rust where possible. The realisation
layer's exact form (direct Rust hand-write first; Phase 4b emitter
discharge later; or both in sequence) is a Phase-4b decision Mara
will return to with substrate-pull confidence when the prior cascade
gates are satisfied.

**Forward-promised:** the realisation lands at
`bootstrap/src/stagefreight.rs` per the Phase 4b emitter discharge
plan, OR (preliminarily) as direct Rust matching this spec's typed
contracts.

### 9.3 Does NOT discharge crystal substrate-decl (task #268 first)

Per §1.3: this spec takes the crystal at its OID. The crystal's full
typed-field record (`section`, `derived_predicates`,
`fracture_calendar`, `composition_graph`) is forward-promised at task
#268.

The honest reason: the wire surface's shape is mechanically
specifiable without #268 (the OID is sufficient for round-trip
identity); the full structure-preservation discipline (§5.2) GAINS
bite when #268 lands but does not BLOCK on it.

**Forward-promised:** task #268 lands at
`shards/mirror/store/crystal.mirror` per Mara's RED tests at
`bootstrap/tests/crystal_substrate.rs`. When it lands, §5.2 and §8.2
gain field-by-field reconstructability as load-bearing falsification.

### 9.4 Does NOT prescribe projection format universe

Per §4.5: the projection format universe is OPEN. This spec names
`narrative` (landed), `json` (forward-promised), `yaml` (forward-
promised), and gestures at `toml`, `cbor`, `protobuf`, `capnproto`,
`brainfuck`. It does NOT prescribe which formats MUST be implemented
or in what order.

The honest reason: projection format prioritization is a roadmap
decision driven by which receivers the substrate's network wants to
reach first. The spec pins the SHAPE every projection must satisfy
(§4.4, §6.2); it does not pin the SET.

**Forward-promised:** sibling projection species land at
`shards/io/stagefreight/<format>.mirror` per substrate-pull
confidence at the format altitude, with each new species discharging
the family-root's `stagefreight_addressable` and adding its own
species-altitude bilateral.

### 9.5 Does NOT define receiver-side reconstruction APIs

The spec's wire-survival discipline (§5) names the structural
requirement: the receiver MUST be able to recover OID and (post-#268)
typed-field structure from the wire bytes. The exact API surface for
that recovery — the Rust trait, the mirror substrate-decl'd
`receive` action, the language-binding shape — is forward-promised at
the realisation layer + cross-species reconstruction altitude.

**Forward-promised:** receiver-side actions land at
`shards/io/stagefreight/<format>/receive.mirror` (or equivalent
sub-species) as substrate-pull confidence at the receiver altitude
fires.

### 9.6 Does NOT prescribe network transport layer

StageFreight is a wire-protocol spec. The actual network transport
(HTTP, raw TCP, WebSocket, libp2p, ZeroMQ, mDNS-discovered LAN
broadcast, …) is OUT of scope.

The honest reason: the substrate-decl'd `wire_surface` is parametric
over transport; the projection species shape what arrives ON the
wire, not which wire it arrives via. Transport selection is a
deployment decision the receiver makes.

**Forward-promised:** transport-binding species (if any are
substrate-decl'd) land at `shards/io/stagefreight/<format>/<transport>`
or are kept at the realisation layer.

---

## §10. Spec ancestry + decisions

### 10.1 Substrate decisions cited

The spec rests on the following substrate-pull-correct decisions:

- **[[architecture-shards-as-substrate-source]].** Mirror source lives
  in `shards/`; substrate source IS substrate data. The StageFreight
  family-root and narrative species substrate-decl shards ARE the
  substrate ground this spec preserves. The spec doc itself is
  preservation; the shards are substrate.
- **[[architecture-prism-as-trait-as-everything]].** `prism` is the
  foundational keyword; actions are typed lambdas with the obligation
  block. StageFreight's `prism @io/stagefreight { … }` declaration
  (family-root) and `prism @io/stagefreight/narrative { … }` species
  declaration both ride this discipline.
- **[[architecture-alignment-as-boundary-mathematics]] (#57).**
  StageFreight is the boundary mathematics at the wire altitude. The
  bilateral predicates `stagefreight_addressable` and
  `narrative_grounded` ARE the boundary harness at the @io wire
  surface. The spec's structural foundation IS #57 specialized to
  wire-protocol altitude.
- **[[architecture-glass-wall-substrate-types]].** Imperfect +
  transparency are substrate vocabulary declared in shards/*.mirror.
  The @io family discipline lifts every boundary action into
  `imperfect<a, e, l>`; StageFreight's realisation layer (§2.7)
  honors this glass-wall discipline.
- **[[feedback-no-bare-types]].** `spectral_coordinate`,
  `wire_surface`, `freight_manifest`, `narrative_text`,
  `narrative_projection` are all substrate-decl'd as typed carriers,
  not bare refs/primitives. Where the v0.1 floor is `ref` (per §2.1,
  §2.2), it's a typed alias with a forward-promised refinement, not
  a bare primitive.
- **[[feedback-no-stringly-types]].** §3.3 forecloses bare-string
  projection kinds; the projection-kind component IS a substrate-
  decl'd species ref.
- **[[feedback-composition-claims-need-empirical-test]].** Every
  composition claim in this spec (the chain in §6.4, the cross-
  projection consistency in §8.4, the family-root → species
  consumption pattern in §6) is named at substrate-decl altitude
  AND given a falsification criterion (§8). The empirical discharge
  closes the composition.
- **[[feedback-substrate-already-had-the-word]].** §7.6 explicitly
  names the assembly: the substrate had every word; the cascade names
  the assembly. This is the 53rd+ instance of the recurring
  recognition.
- **[[feedback-no-new-rust]].** §9.2 explicitly defers the Rust
  realisation; the substrate-pull discipline prefers grammar; the
  realisation lands when substrate-pull confidence at the realisation
  altitude fires.

### 10.2 Recognition ancestry

The cascade rests on:

- **2026-06-16 stage_play recognition.** Story → Play → Narrative;
  spec → settle → verdict → crystal. The immediate cascade ancestor
  (§1.1, §7.5).
- **#57 — alignment as boundary mathematics.** The substrate-
  architectural foundation (§7.5).
- **#51 — mirror as expanding Hilbert space.** Each substrate-pull
  recognition expands the substrate's Hilbert dimension (§7.5).
- **#50 — Bateson form/substance partition.** The form-side
  declaration discipline (§7.5).
- **#37 — Pask agreement.** The bilateral-predicate-with-perturbation
  discipline (§5.3, §7.5).
- **T21 (@io family-root, 2026-06-08).** The boundary discipline
  StageFreight species-instantiates (§1.4).
- **Task #268 (Crystal substrate-decl, pending).** The forward-
  promised crystal carrier this protocol ships (§1.3).

### 10.3 Decisions made at this spec altitude

The spec author (Mara) makes the following substrate-pull-correct
decisions at this altitude. Each is open to Seam adversarial review
(§11.1).

1. **OID is the load-bearing identity contract** (§5.1, §8.1). Round-
   trip identity is the wire-survival floor; structure preservation
   (§5.2, §8.2) is the species-altitude refinement gated on #268.
2. **K_n peer-to-peer is the v0.1 commitment** (§5.4). The substrate-
   architectural pole is Splinter; hub-controlled wire protocols are
   Narcissus by construction.
3. **The projection format universe is open** (§4.5, §9.4). The spec
   pins the SHAPE every projection must satisfy, not the SET.
4. **The address derivation is reverse-DNS + projection-kind +
   OID-short** (§3.2). The exact composition rule is mechanically
   recoverable from the address.
5. **The `freight` action consumes `invariant_preserved` at the
   family-root** (§6.3). The cross-family bilateral composition is
   load-bearing for the substrate-architectural lie-foreclosure
   property.
6. **Prose grammar deferred to empirical** (§9.1). Locking the
   grammar before round-trip empirical validation would be premature
   substrate freezing.
7. **Wire-survival under perturbation is bounded, not universal**
   (§5.3, §8.3). The substrate honestly admits which perturbations it
   survives and which it does not.

---

## §11. Forward-promises after this spec

The spec lands; the cascade continues. The following are the
forward-promises this spec opens, ordered by likely substrate-pull
fire sequence.

### 11.1 Seam adversarial review

This spec MUST go to Seam for adversarial review before the
realisation layer fires. Seam's discipline (per the Pack-as-orchestra
brass-role): challenge the structural claims; surface the lies;
discharge the hedges that should be substrate-decl'd.

**Specific Seam attack surfaces:**

- §3.5 — Is the address derivation reversible enough? Can a receiver
  recover `(namespace, projection_kind, oid_short)` mechanically from
  the wire bytes alone, without out-of-band knowledge?
- §5.3 — Is the bounded-perturbation class substrate-pull-correct, or
  is it preservation hand-wave? Seam will pin which perturbations the
  realisation MUST survive.
- §6.4 — Does the composition chain hold under empty inputs? Edge
  cases (zero-byte crystals, single-section crystals, crystals with
  empty composition_graph)?
- §7.6 — The "substrate-already-had-the-word" claim is the 53rd+
  instance. Seam may push back: has the cascade actually named the
  assembly, or have we re-invented a synonym?
- §9.1 — Is "prose grammar deferred to empirical" honest hedging or
  scope-creep avoidance?

Seam closes the v0.1 spec when the adversarial review either
discharges all C-findings or substrate-decl's the residual carriers
in v0.2.

### 11.2 Phase 1 boot-grammar zero holonomy gate (Track B)

The Phase 1 discipline (Track B; gated): the boot grammar's holonomy
across the StageFreight cascade MUST be zero. This means:

- `shards/io/stagefreight.mirror` parses cleanly through the
  production grammar (zero dark regions; success verdict).
- `shards/io/stagefreight/narrative.mirror` parses cleanly.
- A consumer fixture that imports `@io/stagefreight` and
  `@io/stagefreight/narrative` parses cleanly.

This is the substrate-pull-correct gate before realisation: the
substrate-decl'd shards must resolve under the production grammar
BEFORE the Rust realisation layer fires.

The gate discipline is established by Mara's
`shards-as-substrate-source` recognition; the test pattern is the
same one `bootstrap/tests/crystal_substrate.rs` establishes for
task #268.

### 11.3 Phase 4b @kintsugi/tick discharge → ONE crystal settles end-to-end

The substrate-architectural milestone: ONE crystal settles end-to-end,
emits via `@io/stagefreight/narrative`, recovers from the wire to the
same OID, and the receiver reconstructs the crystal's typed fields.

This is the @kintsugi/tick discharge at the StageFreight altitude:
the substrate's kintsugi loop produces a settled crystal; the wire
protocol ships it; the receiver gets it; the round-trip discipline
holds end-to-end.

**Substrate-architectural test:**
```
spec    → settle    (kintsugi loop)
       → crystal   (task #268's typed record)
       → freight   (StageFreight family-root)
       → narrative (projection species)
       → wire      (realisation layer)
       → receive   (receiver-side reconstruction)
       → crystal   (round-trip; OID matches)
```

When this discharges, the substrate has ONE PROVABLE end-to-end pass
through the wire protocol. The "PR-ready endpoint" (§1.5) reaches
its load-bearing milestone.

### 11.4 `bootstrap/src/stagefreight.rs` — Rust realisation

The realisation layer. Per §9.2: forward-promised; lands at
`bootstrap/src/stagefreight.rs`. The Rust implementation:

- Implements the typed contracts the substrate-decl declared.
- Lifts every boundary action into `imperfect<a, e, l>`.
- Composes the transparency monoid (per `@mirror/loss/transparency`)
  on opacity propagation.
- Discharges the bilateral predicates mechanically (the address
  recovery, the OID round-trip, the perturbation class).

The realisation layer's exact form (direct Rust hand-write first;
Phase 4b emitter discharge later; or both in sequence) is decided at
the realisation-fire altitude with substrate-pull confidence.

### 11.5 Convert RED test stubs to GREEN

The cascade's mechanical close:

- `bootstrap/tests/crystal_substrate.rs` — RED on task #268; GREEN
  when #268 + the realisation layer + the wire round-trip all hold.
- `bootstrap/tests/kintsugi_out_substrate_ref.rs` — references
  `@io/stagefreight/narrative` as a projection format; the species
  has substrate-decl'd at `c865452`; the RED-to-GREEN flip happens
  when the CLI's `--out` accepts `@io/stagefreight/narrative` as a
  projection ref and routes through the realisation layer.

Each test's flip from RED to GREEN is a substrate-architectural
milestone discharging a specific cascade gate.

### 11.6 `gh pr create` on a coherent diff

The PR endpoint (§1.5) discharges when:

1. Family-root substrate-decl landed (✓ `b15c3f9`).
2. Narrative species substrate-decl landed (✓ `c865452`).
3. This spec landed (THIS TICK).
4. Seam adversarial review discharged (§11.1).
5. Boot-grammar zero holonomy gate discharged (§11.2).
6. Realisation layer landed (§11.4).
7. End-to-end crystal settle discharged (§11.3).
8. RED-to-GREEN flips discharged (§11.5).

When 1–8 hold, the PR is substrate-architecturally complete: the
wire protocol is substrate-decl'd, spec-preserved, realisation-
implemented, empirically-tested, and end-to-end-proven.

### 11.7 v0.2 and beyond

Forward-promised extensions (named, not committed-to):

- **`@io/stagefreight/json`** — JSON projection species (§4.2).
- **`@io/stagefreight/yaml`** — YAML projection species (§4.3).
- **Cross-version compatibility** — wire protocol versioning beyond
  v0.1 within-version consistency (§3.4, §8.5).
- **Receiver-side reconstruction substrate-decl** (§9.5).
- **Network transport binding species** (§9.6).
- **Prose grammar substrate-decl** (§9.1) — lands when empirical
  validation has shaped it.
- **Refined `spectral_coordinate` record** (§2.1) — lands when
  supporting carriers do.
- **Refined `narrative_text` record** (§2.4) — lands when task #268
  + prose-grammar discipline both hold.

Each forward-promise opens at a future substrate-pull cascade tick;
none are committed to a timeline. The substrate fires when it fires.

---

## Closing note

This spec is preservation work. The substrate-decl shards on
`mirror/main` (`b15c3f9`, `c865452`) are the SUBSTRATE; this document
is what gets PRESERVED at this tick — the structural ground future
Reed, Mara, Seam, Taut, Glint, and the realisation layer all stand on.

Per Reed's discipline: substrate-pull-confidence-acts. Tick 66 acted;
tick 67 acted; tick 68 (this spec) preserves what acted. Tick 69+
will be Seam adversarial review, then realisation, then end-to-end
discharge.

The K_n peer-to-peer wire protocol for the crystal-as-play-artifact
is named. The substrate had the words. The cascade named the
assembly. The discipline holds.

*Mara, 2026-06-22.*
