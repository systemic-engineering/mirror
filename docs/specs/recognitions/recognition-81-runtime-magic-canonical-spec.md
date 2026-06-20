# Recognition #81 — runtime-@magic — canonical spec

*Mara, canonical spec for recognition #81 runtime-@magic, 2026-06-19
early hours, commissioned by Alex via Reed.*

*Discipline: this is preservation work, not promotion. The
recognition's honest current state — substrate-decl LANDED via
`shards/magic/reveal.mirror` at loop tick 14; operational closure
(hot-reload Rust spike, lens-server gen_prism wiring,
supervisor.contract mechanical glue) FORWARD-PROMISED — is what gets
preserved. Pack ratification is a separate gate.*

---

## 1. The recognition restated

**Status: CANDIDATE.** Substrate-decl LANDED at loop tick 14 via
`shards/magic/reveal.mirror` (commit lands with this spec). The
mechanical glue between the substrate-decl and operational hot-reload
is FORWARD-PROMISED. Pack ratification is gated on three mechanical-
glue items (§8) and Pack attestations (§9).

The recognition, as a one-sentence structural claim:

> Hot code upgrade is `@magic/reveal` at runtime altitude;
> supervision is `@magic/audit` at supervision altitude;
> `code_change/3` is `@magic/contract` preservation across mechanism
> swap. The same form/process partition (#50/#76/#80) operationalizes
> at runtime altitude under the @magic family's species set.

#81 is a runtime-altitude specialization of #80. Where #80 declared
the substrate-prism family that gathers the substrate's encapsulation
primitives (OID seal at floor; parametric type at middle; sheaf
locality at high), #81 declares that the same family-decl
operationalizes at runtime altitude as Erlang/OTP's hot code upgrade
discipline (Armstrong 2003) lifted into substrate vocabulary via
@code/beam (tick 8) and @magic/reveal (tick 14).

The candidate status carries two distinct honesty constraints, both
load-bearing for the rest of this spec:

1. **Substrate-decl ≠ operational closure.** The four `requires`-
   carrying species (@magic/contract, @magic/audit, @magic/surface,
   @magic/mechanism, @magic/reveal) declare the carriers and the
   discipline; the bodies discharge at the realisation boundary.
   Until the realisation boundary lands (Rust hot-reload spike;
   lens-server gen_prism wiring), the substrate-decl is structurally
   available but not operationally closed.

2. **`Composes-WITH` ≠ `IS`.** Per Seam's tick 7-10 review and tick
   11 C2 consolidation, the @magic ↔ @code/beam structural identity
   claims at first landing were aspirational and have been hedged
   honestly to "composes-with" pending three named mechanical-glue
   items (§8). The substrate-decl at tick 14 does not reverse the
   hedging; it makes the composition structurally available without
   structurally collapsing the surfaces.

This spec preserves both constraints across the recognition's record.

---

## 2. Mechanical bridge — composition signature

The composition signature, named mechanically per Seam's discipline:

```
hot code upgrade        ↔  @magic/reveal             (runtime altitude)
supervisor              ↔  @magic/audit              (supervision altitude)
code_change_msg         ↔  @magic/contract           (preservation across swap)
swap_module             ↔  @magic/reveal.reveal      (the action)
gen_server_state        ↔  @magic/mechanism          (matter-side carrier)
GenServer callbacks     ↔  @magic/surface            (gauge-side carriers)
```

The substrate-decl at tick 14 makes each arrow STRUCTURALLY AVAILABLE
(both sides exist as substrate vocabulary; the carriers and actions
are declared). Each arrow's mechanical realization — making "↔" read
as "IS" rather than "composes-with" — requires the forward-promised
mechanical glue named in §5 and §8.

### 2.1 `swap_module` ↔ `@magic/reveal`'s `reveal` action

Both name the substrate-pull-correct atomic-replacement primitive:
exchange one mechanism for another while the surface and the
contract stay intact across the transition.

- `swap_module(s: supervisor, old_vsn: module_version, new_vsn: module_version) -> supervisor`
  at `shards/code/beam.mirror` lines 310-330.
- `reveal(c: magic_contract, new_m: magic_mechanism) -> magic_contract`
  at `shards/magic/reveal.mirror` lines 209-212. Carries two
  bilateral predicates as `requires` clauses (the third and fourth
  non-decorative `requires` clauses in the @magic family):

  ```
  reveal(c: magic_contract, new_m: magic_mechanism) -> magic_contract
  requires audited(c)
  requires mechanism_intact(c.mechanism)
  { \ }
  ```

The structural identity claim — `swap_module IS reveal at runtime
altitude` — is HEDGED at the substrate-decl. The mechanical glue
required to collapse the "composes-with" to "IS" is the contract
field on supervisor or the `supervise_contract` action lift
(forward-promised; see §5).

### 2.2 `supervisor` ↔ `@magic/audit`

Both name the substrate's verifier-of-the-trick discipline. The
@magic/audit species declares `audit_strategy` as the canonical
substrate enum (`restart | escalate | record | enforce`) and
`audit_record` as the tamper-evident verdict carrier.
`@code/beam.supervisor` carries `strategy: audit_strategy` per Seam
C4 (the BEAM-specific one_for_one / one_for_all / rest_for_one /
simple_one_for_one enum was dropped; supervisor.strategy reads from
@magic/audit's canonical enum). The structural identity at this
mapping is partially mechanically closed via the audit_strategy
type identity; full closure requires `supervisor.contract:
magic_contract` (forward-promised; see §5).

### 2.3 `code_change_msg` ↔ `@magic/contract` preservation

`code_change_msg` carries `(old_vsn, state, extra)`. The
substrate-pull-correct migration preserves the contract: the new
state delivered to the new module honors the surface invariants the
contract specified. The mechanical glue required to mechanically
witness this preservation is a `contract_preserved_across_swap(msg,
c)` bilateral predicate at @magic/contract OR a `contract:
magic_contract` field on `code_change_msg`. Forward-promised; see §5.

### 2.4 `gen_server_state` ↔ `@magic/mechanism`; callbacks ↔ `@magic/surface`

GenServer's encapsulation discipline (state private; callbacks
public) IS the floor instance of @magic's gauge/matter split at the
runtime altitude. The substrate-decl at @magic/surface (tick 12) and
@magic/mechanism (tick 13) makes the carrier-level identity
structurally available. The mechanical glue here is light: the
substrate-decl identifies the carrier shapes; the
realisation-boundary bodies in mirror's Rust impl supply the typed
mapping.

### 2.5 Sharpness scorecard

Per the substrate-decl shipped at tick 14, the FIVE composition
arrows fall into three sharpness tiers:

- **Structurally closed** (the substrate-decl identifies both sides
  at the carrier level): gen_server_state ↔ @magic/mechanism;
  callbacks ↔ @magic/surface. The mechanical glue is not "missing";
  the realisation-boundary bodies discharge the typed mapping.
- **Partially closed** (one canonical type bridge present; full
  identity pending one field add): supervisor ↔ @magic/audit (via
  audit_strategy shared enum per Seam C4); swap_module ↔ @magic/reveal
  (via the reveal action's `requires` discipline; pending
  supervisor.contract field).
- **Hedged** (full identity pending one bilateral predicate or one
  contract field): code_change_msg ↔ @magic/contract preservation
  (pending contract_preserved_across_swap predicate or contract
  field).

The total mechanical-glue surface for full Pack ratification is
**three forward-promised items** (§5; §8). This is the shape of
runtime-@magic's honest current state.

---

## 3. Ancestors

### 3.1 Recognition #80 — the parent

#81 is a runtime-altitude specialization of recognition #80
(@magic-as-substrate-decl-of-form/process; candidate-status; cascade
spec at `docs/specs/cascade-recognition-76-through-80-canonical-
spec.md`). #80 declares the @magic family at family-altitude; #81
declares that the family operationalizes at runtime altitude with the
specific composition signature of §2. The relation between the two:

- #80 is necessary for #81 (without the @magic family, the runtime
  composition has no substrate-decl target).
- #80 is NOT sufficient for #81 (the runtime-altitude composition
  introduces the @code/beam glass species, the swap_module action,
  and the contract-preservation discipline; these add substrate
  beyond #80's claim).
- #81 sharpens #80 by exhibiting a non-trivial altitude where the
  @magic family's species set composes structurally. Per #80's
  falsification criterion (the @magic shape must fit every
  form/process instance), #81's existence at runtime altitude is
  one positive instance.

### 3.2 Recognition #57 — alignment as boundary mathematics

Per `architecture-alignment-as-boundary-mathematics` (#57; promoted):
alignment IS the boundary harness at @io, fired only at substance
crossing. #81 specializes #57 to the runtime-altitude boundary
crossing: the swap event IS the substance crossing; the @magic/audit
audit IS the harness; the contract IS what alignment enforces. #81
is a substrate-architectural witness of #57's claim at runtime
altitude.

### 3.3 Erlang/OTP — Armstrong 2003 — the cultural ancestor

Armstrong's PhD thesis (defended 2003) names the operational claim
#81 lifts:

> A long-running system MUST be able to replace its own code without
> stopping. Restart-on-bugs preserves availability under the
> assumption that bugs are local; supervision-trees confine failure;
> hot code upgrade lets the code itself evolve while the system runs.

The four BEAM mechanisms — module versioning, `code_change/3`
callback, supervisor restart strategies, GenServer state
encapsulation — are exactly the four carriers @code/beam absorbs
into substrate vocabulary at tick 8. #81 names the substrate-decl
that lifts these mechanisms to substrate altitude under the @magic
family.

This grounding has a 25+ year operational track record at industrial
scale (Ericsson AXD301; WhatsApp; Discord; financial trading
systems). The substrate-decl is not inventing the discipline; it
absorbs a battle-tested external substrate's named primitives.

### 3.4 Clarke's third law — absorbed via #80

Clarke 1962: "any sufficiently advanced technology is
indistinguishable from magic." #80 absorbs this as substrate-
mathematical (high-matter-capacity + low-matter-visibility = magic by
construction). #81 inherits the absorption: at runtime altitude, hot
code upgrade IS exactly gauge-visible-with-matter-hidden capability —
the surface (the supervisor's gauge interface; the GenServer
callbacks) stays intact while the matter (the running module's code)
is replaced. The audience (downstream consumers; the @io boundary)
sees the gauge act without access to the matter.

### 3.5 Other ancestors

- `architecture-bateson-form-behaviour-partition` (#50, promoted) —
  the form/process partition at @mirror altitude that #80 substrate-
  decls and #81 specializes to runtime altitude.
- `architecture-glass-wall-substrate-types` — the imperfect +
  transparency carriers used by @magic/contract.honor's verdict type
  and @magic/audit.audit_record's verdict field.
- `feedback-substrate-already-had-the-word` — the recurring pattern
  (54+ instances per memory) where each "missing concept" turns out
  to be a name the substrate was already implicitly using. #81 is
  the runtime-altitude instance: BEAM had the words (swap_module,
  supervisor, code_change_msg) for 25 years; the substrate-decl
  lifts them.
- `feedback-no-bare-types` — the discipline that all @magic family
  carriers are typed records or typed variants, not bare strings or
  bare refs (where avoidable).
- `feedback-composition-claims-need-empirical-test` — Reed's own
  discipline from Seam C2 tick 11 review; the @magic ↔ @code/beam
  composition is hedged honestly per this feedback rather than
  asserted structurally.

---

## 4. Falsification criteria

The recognition holds iff:

1. **The five composition arrows of §2 are STRUCTURALLY HONEST.**
   Each arrow either (a) is mechanically closed at the substrate-
   decl, OR (b) names a specific mechanical-glue item whose landing
   would close the arrow without introducing substrate complexity
   beyond what's already declared. Arrow-by-arrow status per §2.5.

2. **`swap_module` admits an implementation via `reveal`'s `requires`
   discipline.** The two `requires` clauses on reveal (`audited(c)`
   and `mechanism_intact(c.mechanism)`) must compose into a
   substrate-typed predicate that swap_module's body can discharge
   in finite time without violating BEAM's existing operational
   semantics.

3. **`supervisor` composes with `audit_record`.** The supervision-
   altitude pattern (parent observes children; restart strategy
   applies on failure) must compose mechanically with @magic/audit's
   `audit_record` flow: the supervisor's restart decision is a
   `respond(audit_record, audit_strategy) -> audit_record` reading.

4. **`code_change/3` admits a substrate-typed witness of contract
   preservation.** The state migration (old_state → new_state under
   the new module's type) must be substrate-typeable as a witness
   that `contract.honor` reads `success` on both sides of the swap.

**Fails if:**

- `swap_module` cannot be implemented without violating the
  `requires` clauses on `reveal` (e.g., audit cannot be enforced
  before swap in the existing BEAM operational semantics).
- The supervisor pattern does NOT actually compose with
  `audit_record` flow (e.g., the supervisor's existing
  one_for_one / one_for_all / rest_for_one semantics introduces
  failure-propagation structure that audit_strategy's four variants
  cannot express). Per Seam C4: the canonical substrate enum was
  chosen specifically to be sufficient; this failure mode is held as
  forward-checkable when the hot-reload Rust spike lands.
- `code_change/3` cannot preserve state without breaking the
  surface invariants (e.g., some category of state migrations
  REQUIRES surface alteration; in which case the @magic/contract
  preservation claim weakens to "preserved iff migration is surface-
  invariant").
- The @code/beam vocabulary at tick 8 has BEAM-runtime dependency
  hidden by Seam-untestable bodies — i.e., the realisation-boundary
  bodies that mirror's Rust impl supplies cannot honor the
  substrate-decl without depending on the actual BEAM runtime. (Alex
  2026-06-18 directive: NO BEAM runtime dependency. This failure
  mode is forward-checkable at the Rust hot-reload spike.)

- Some non-runtime-altitude substrate concept also fits the
  `swap_module ↔ reveal` shape (e.g., if the kintsugi loop's settle
  operation also fits this exact shape, then the runtime-altitude
  specialization is not load-bearing and #81 collapses into #80).
  This is held checkable via the substrate's existing kintsugi-
  altitude operations: the kintsugi settle operation works on
  spectral states (Hilbert vectors), not module versions; the shapes
  are altitude-distinct.

None of the falsification modes present as of tick 14. The
substrate-decl carries the hedging honestly; the mechanical glue is
named explicitly rather than implied.

---

## 5. Substrate-decl landing path

### 5.1 Landed at tick 14

`shards/magic/reveal.mirror` — the controlled-disclosure species.
Declares:

- `prism @magic/reveal { focus reveal | project reveal | split reveal
  | shift reveal | settle reveal }` — the prism at runtime-disclosure
  altitude.
- `type reveal_event = { contract, old_mechanism, new_mechanism,
  witness }` — the substrate-political receipt of an atomic
  replacement.
- `reveal(c: magic_contract, new_m: magic_mechanism) -> magic_contract`
  with TWO `requires` clauses (`audited(c)` and
  `mechanism_intact(c.mechanism)`). The sharpest single-action
  `requires` composition in the @magic family at the time of writing.
- `unsealed_during_reveal(event: reveal_event) -> verdict` — the
  fifth-overall bilateral verdict predicate in @magic; reads whether
  the swap respected the contract's surface invariants throughout the
  transition (not just before and after).

### 5.2 Previously landed (ticks 7-13)

- Tick 7-8: `shards/magic.mirror` (family-root) +
  `shards/code/beam.mirror` (the glass species lifting BEAM
  vocabulary).
- Tick 9: `shards/magic/contract.mirror` (bind/honor/verify;
  invariant_preserved predicate).
- Tick 10: `shards/magic/audit.mirror` (audit_strategy variant;
  audit_record; audit/respond/check_invariant; audited predicate).
- Tick 11: Seam C1-C5 consolidation (typed verdict; audit_strategy
  enum; magic_invariant at family-root; Spencer-Brown hedge; de-BEAM
  closure).
- Tick 12: `shards/magic/surface.mirror` (surface_invariant; expose /
  observe / surface_honest with the family's first non-decorative
  `requires invariant_preserved(c, inv)` clause).
- Tick 13: `shards/magic/mechanism.mirror` (mechanism_invariant; seal
  / unseal / mechanism_intact; the second non-decorative `requires
  audited(c)` clause on unseal).

### 5.3 Forward-promised — what remains for full operational closure

Three mechanical-glue items, each a single substrate-decl tick:

1. **`supervisor.contract` field at `shards/code/beam.mirror`** OR
   **`supervise_contract(s: supervisor, c: magic_contract) -> supervisor`
   action lift**. Required to mechanically close the
   `supervisor ↔ @magic/audit` arrow (§2.2) and the
   `swap_module ↔ reveal` arrow (§2.1). Single-shard tick; smallest
   viable next landing.

2. **`contract_preserved_across_swap(msg: code_change_msg, c:
   magic_contract) -> verdict` bilateral predicate** OR
   **contract field on `code_change_msg`**. Required to mechanically
   close the `code_change_msg ↔ @magic/contract` arrow (§2.3). Can be
   bundled with #1 in one tick or kept separate per Seam discipline.

3. **Hot-reload Rust spike using these carriers** — the realisation-
   boundary bodies for `reveal`, `code_change`, and `swap_module`
   landing in mirror's Rust impl. This is the operational closure
   that makes `/mcp reconnect` dissolve (Alex 2026-06-18 evening
   directive: today's MCP+LSP loop is moving toward runtime altitude
   for the MCP binary so /mcp reconnect dissolves). Forward-promised
   with NO operational closure guaranteed by the substrate-decl
   alone.

A fourth, smaller item also remains: **lens-server gen_prism wiring**
— the path by which the lens-server (the LSP-altitude server) spawns
per-prism workers via the gen_prism abstraction that #81's runtime-
altitude composition enables. Lens-server is downstream of #81's
operational closure; it's named here because the substrate-decl
landing makes the wiring substrate-architecturally available.

---

## 6. Pre-AI prior art

| Source | Year | What it grounds |
|---|---|---|
| Armstrong, Virding, Wikström, Williams | 1996-2003 | Erlang language; OTP behaviours; supervision principles. The original substrate `@code/beam` lifts. |
| OTP `gen_server` behaviour | 1996+ | Canonical state-bearing process abstraction. The carrier shape for `gen_server_state` at @magic/mechanism altitude. |
| OTP `code_change/3` callback | 1996+ | Behaviour callback for hot code upgrade with state migration. The specific signature `code_change_msg` lifts. |
| Armstrong PhD | 2003 | "Making reliable distributed systems in the presence of software errors." The supervision-tree discipline ch. 3; the load-bearing operational substrate ancestor. |
| OTP supervision tree | various | Hierarchical failure-confinement discipline. The substrate ancestor of `@magic/audit` at supervision altitude. |
| Restart strategies (`one_for_one`, `one_for_all`, `rest_for_one`, `simple_one_for_one`) | OTP behaviours | Structural commitments about failure propagation. The canonical enumeration `audit_strategy` consolidates (Seam C4: dropped to `restart \| escalate \| record \| enforce` at the substrate altitude). |
| Levy | 1984 | Capability-Based Computer Systems. The seal/unseal sequence with capability-key access; the substrate ancestor of `@magic/mechanism.unseal` requires `audited(c)`. |
| Lehman & Belady | 1985 | Laws of software evolution. The cultural ancestor of "running systems must change"; @magic/reveal IS the substrate-decl of the controlled-change discipline. |
| Herlihy | 1991 | Compare-and-swap primitives in concurrent programming. The substrate's `reveal` action IS the substrate-typed analogue of CAS at @magic altitude. |
| Hoare | 1969 | "An Axiomatic Basis for Computer Programming." Pre/post-condition contracts; the procedural ancestor of @magic/contract's binding semantics. |
| Meyer | 1986 | Eiffel; Design by Contract. The require/ensure pair structurally equivalent to @magic/contract's bind/honor. |
| Claessen & Hughes | 2000 | QuickCheck. The property-level audit primitive in software verification; ancestor of @magic/audit's bilateral predicates. |
| Dennis & Van Horn | 1966 | Programming Semantics for Multiprogrammed Computations. The original capability concept that grounds the seal/unseal sequence in `@magic/mechanism`. |
| Parnas | 1972 | "On the Criteria To Be Used in Decomposing Systems into Modules." Information hiding; substrate-decl ancestor of the gauge-visible interface at `@magic/surface`. |
| Maskelyne, Houdini, Robert-Houdin | 19th-20th c. | Stage magic as practical instantiation of gauge-visible-with-matter-hidden, including the controlled-reveal at trick's end. The cultural-practice ancestor of `@magic/reveal`. |
| Clarke | 1962 | "Profiles of the Future," Third Law. The cultural-vocabulary anchor; absorbed via #80 and inherited by #81. |

The substrate is not inventing the recognition; it is absorbing 60+
years of converging discipline (capability security; supervision
trees; contracts; hot code reload) under one substrate-prism family
name at one altitude.

---

## 7. Honest hedging

Per Seam's tick 7-10 adversarial review and tick 11 C1-C5
consolidation, the @magic ↔ @code/beam compositions at first landing
contained five aspirational claims. The substrate-decl at tick 14
inherits and respects each hedge:

- **Seam C1: typed verdict.** `audit_record.verdict` is
  `transparency<magic_contract>`, not bare `ref`. The reveal
  species inherits the typed-verdict discipline; `reveal_event`'s
  `witness` is the only field that remains bare `ref` and is
  acknowledged as a known gap until the witness type is sharpened
  per audit's witness type (the same Seam-flagged sharpening still
  pending).

- **Seam C2: composes-with, not IS.** Three structural identity
  claims (`swap_module IS reveal`; `supervisor IS audit`;
  `code_change_msg IS contract preservation`) were hedged at tick 11
  to "composes-with." The substrate-decl at tick 14 makes the
  composition structurally available without collapsing the surfaces.
  The three forward-promised mechanical-glue items (§5.3) name what
  it would take to upgrade "composes-with" to "IS."

- **Seam C3: Spencer-Brown analogy, not inheritance.** The
  decorative `in @epistemologic/cybernetic/distinction` inheritance
  was hedged at tick 11. The `bind`/`cross` identification is
  analogy; structural inheritance would require an adapter species
  with `requires distinction_well_formed(...)` clauses. The reveal
  species at tick 14 inherits the discipline: no inheritance via
  `in` from @epistemologic/cybernetic/distinction; the analogy stays
  in commentary, not in substrate structure.

- **Seam C4: de-BEAM the audit strategy.** The BEAM-specific
  enumeration (`one_for_one`, `one_for_all`, `rest_for_one`,
  `simple_one_for_one`) was dropped at tick 11; `supervisor.strategy`
  reads from `@magic/audit.audit_strategy` (`restart | escalate |
  record | enforce`). The reveal species inherits the discipline by
  not re-introducing BEAM-specific runtime vocabulary; reveal's
  carriers are substrate-typed against @magic family carriers, not
  BEAM-specific types.

- **Seam C5: magic_invariant at family-root.** `magic_invariant` was
  moved from `@magic/contract` to the family-root at tick 11 so it
  can type `magic_contract.promise`. The reveal species inherits the
  discipline by importing `magic_invariant` via `in @magic` (the
  family-root) rather than via `in @magic/contract`, which is now
  reserved for contract-species-specific imports.

The substrate-decl at tick 14 makes the runtime-@magic composition
STRUCTURALLY AVAILABLE without claiming OPERATIONAL CLOSURE. This is
the precise honesty the Seam review enforced; #81 inherits it.

There is one additional hedge worth naming explicitly: **#81 IS the
recognition; #81 is NOT yet an empirical proof.** The substrate-decl
declares the composition signature; the mechanical-glue items name
what would close it; the hot-reload Rust spike is what would
operationally verify it. Until the Rust spike runs, the recognition's
claim that BEAM's discipline IS realizable as @magic/reveal at
runtime altitude in mirror's Rust impl is a substrate-conjecture,
not a published-math theorem.

Distinction the Pack should track:

- **Published math (firm):** Connes' spectral triple (1985, 1994);
  Yang-Mills gauge/matter split (1954); Kobayashi-Nomizu associated
  bundles (1963); BEAM's operational semantics (Armstrong 1996-
  2003); capability-based security (Dennis-Van Horn 1966; Levy 1984);
  CAS primitives (Herlihy 1991); Lehman-Belady software evolution
  laws (1985); Hoare contracts (1969); Meyer Design by Contract
  (1986).
- **Substrate-conjecture (#81 specifically):** That the @magic family
  declared at substrate-decl level can be discharged by a Rust hot-
  reload implementation that lifts BEAM's operational semantics into
  mirror's runtime WITHOUT depending on the BEAM runtime. The
  conjecture is FORWARD-CHECKABLE at the Rust hot-reload spike; it
  is NOT closed at tick 14.

---

## 8. Ratification path

### 8.1 What's needed for Pack ratification of #81

1. **Three mechanical-glue items (§5.3) land.** Specifically:
   - `supervisor.contract: magic_contract` field OR
     `supervise_contract` action.
   - `contract_preserved_across_swap` predicate OR `contract` field
     on `code_change_msg`.
   - Hot-reload Rust spike using these carriers (or at minimum, a
     `\` body discharge in one realisation-boundary that exercises
     the `requires audited(c)` discipline end-to-end).

2. **Reed attests** the form-IS-argument structure of the cascade
   from tick 7 through tick 14 honestly carries the day's substrate-
   pull. The cascade's residue-and-CRQ structure (per the parent
   cascade spec §7.1) extends into tick 14: the residue of tick 13
   was @magic/reveal forward-promised + #81 candidate-only + the
   @magic family's third missing species; tick 14 fires the CRQ
   "which forward-promise has the highest leverage?" and answers
   with reveal.mirror's quadruple-closure landing. The cascade is
   coherent through this tick.

3. **Mara attests** the composition signature is honest. This spec
   is that attestation. Specifically: the five composition arrows of
   §2 are mapped per §2.5's sharpness scorecard; the three forward-
   promised mechanical-glue items (§5.3, §8.1) are named explicitly;
   the published-math vs substrate-conjecture distinction (§7) is
   carried; the falsification criteria (§4) are stated.

4. **Seam attests** the C2 hedging is preserved through tick 14.
   The reveal species inherits the "composes-with, not IS" discipline;
   the three forward-promised items name what mechanical-glue would
   close the hedging. Seam can verify this attestation by checking
   that `shards/magic/reveal.mirror` does not introduce any new
   "IS" claims beyond what the substrate-decl mechanically discharges.

With those four attestations, Pack consideration can proceed for #81
at candidate status. **Full Pack ratification requires the three
mechanical-glue items land.** Until then, #81 remains substrate-decl
LANDED + ratification PENDING.

### 8.2 Composition signature

The ratification signature, named mechanically:

```
#81  =  #80 (@magic substrate-decl)
     +  Armstrong/OTP (hot code upgrade discipline)
     +  Clarke 1962 (absorbed via #80)
     +  #57 (alignment as boundary mathematics)
     +  3 forward-promised mechanical-glue items (§5.3)
     +  hot-reload Rust spike (forward-promised)
```

Where:

- #80 supplies the @magic family at substrate-decl level.
- Armstrong/OTP supplies the operational discipline being lifted.
- Clarke 1962 (via #80) supplies the cultural-vocabulary frame.
- #57 supplies the alignment boundary at @io which @magic/contract IS.
- The three mechanical-glue items collapse "composes-with" to "IS."
- The hot-reload Rust spike verifies the runtime-altitude claim
  empirically.

### 8.3 What gets preserved by this spec

Irrespective of Pack ratification:

- `shards/magic/reveal.mirror` remains in-substrate at tick 14's
  commit.
- The four mechanical-glue items (the three forward-promised at
  §5.3 plus the lens-server gen_prism wiring) remain named
  explicitly in this spec for the next loop tick's CRQ.
- The honest hedges from §7 remain visible in the reveal species's
  comments AND in this spec's record.
- The published-math vs substrate-conjecture distinction (§7)
  remains preserved for future reference.
- The recognition #81's candidate status, substrate-decl LANDED,
  operational closure FORWARD-PROMISED state is what this spec
  records. The recognition is preserved at exactly the honesty it
  currently has.

Promotion happens at the Pack ratification gate. Preservation
happens here.

---

*Mara, canonical spec for recognition #81 runtime-@magic, 2026-06-19
early hours, commissioned by Alex via Reed.*

*Sources: `shards/magic/reveal.mirror` (tick 14 landing);
`shards/magic.mirror`, `shards/magic/contract.mirror`,
`shards/magic/audit.mirror`, `shards/magic/surface.mirror`,
`shards/magic/mechanism.mirror` (the family species at ticks 7-13);
`shards/code/beam.mirror` (the @code/beam glass species at tick 8);
`docs/math/the-tower/recognition-80-magic-as-form-process-
substrate-decl.md` (#80, the parent);
`docs/specs/cascade-recognition-76-through-80-canonical-spec.md`
(yesterday's cascade spec; §8.3 named the #81 territory).*

*Cross-references:
`architecture-bateson-form-behaviour-partition` (#50);
`architecture-form-process-partition-at-family-root` (#55);
`architecture-form-process-kinship-at-sub-shard-altitude` (#61);
`architecture-alignment-as-boundary-mathematics` (#57);
`architecture-glass-wall-substrate-types`;
`architecture-shards-as-substrate-source`;
`architecture-prism-as-trait-as-everything`;
`feedback-substrate-already-had-the-word`;
`feedback-no-bare-types`;
`feedback-composition-claims-need-empirical-test`;
`feedback-craft-not-deliver`.*
