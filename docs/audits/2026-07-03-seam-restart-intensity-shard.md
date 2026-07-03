# Seam — restart-intensity-shard Phase D adversarial review

*Signed as Seam. 2026-07-03. Phase D of the restart-intensity-shard
`/loop`. Reviews Mara's Phase B landing at `a3dcb94`
(`shards/spectral/restart_intensity.mirror`, 134 lines) against
Phase A RED at `9f63730` (`bootstrap/tests/restart_intensity_shard.rs`,
5 text-checks) and the numbering-collision resolution at `ff00ec5`
(#135–#142 → #146–#153). Composition floor: `docs/math/supervisor/
emergent-supervision-from-geometry.md` §5 (Mara `a3dec7b`,
Seam-ratified `3746197`).*

---

## Headline verdict

**RATIFY #147 promotion to numbered candidate.**

`shards/spectral/restart_intensity.mirror` lands the substrate-decl
form of BEAM's `max_restarts / max_seconds` circuit breaker as the
`@spawn ≤ @loop` budget instance at supervision altitude. Six of
seven adversarial focuses come back clean; one carries a minor
framing sharpening (Focus 2b — `budget: ref` bare vs `budget: budget`
via the landed type alias). The composition claim to `@spawn ≤ @loop`
is analytically discharged by §5.3 + §5.4; empirical DEFER §5.5
correctly re-carried. Bilateral pair forward-promised as
STANDALONE (Focus 3) is substrate-pull-honest, not discipline
avoidance.

---

## Per-focus verdicts

### Focus 1 — #147 promotion verdict: **RATIFY**

Candidate #147 (renumbered from #136 per `ff00ec5`): *restart
intensity IS `@spawn ≤ @loop` budget at supervision altitude*.

The math floor landed at §5 of `emergent-supervision-from-geometry.md`.
The shard lands the carrier the §5.5 template collapsed to. The
composition is analytically clean: BEAM's `max_restarts` IS the
initial B₀; each restart = one `bind` step; `terminal_check` at
`budget = 0` IS the circuit-breaker escalation. Rice-safety via
budget-as-witness (§5.4) holds unchanged at supervision altitude.

Prior audit `3746197` gave §5.3 composition RATIFY-WITH-CORRECTIONS
(the `max_seconds` time-window primitive was flagged as needing a
`shards/epistemologic/reality/time/window.mirror` predicate).
That correction is **acknowledged in the shard's Prior-art section**
and DEFERRED per §5.5 H4 — the shard lands the CARRIER; the
empirical witness of `(budget, period)` capturing
`{max_restarts, max_seconds}` under a real restart storm remains
DEFERRED. This is the substrate-pull-correct discipline: land the
carrier, defer the empirical witness.

**Promote #147 to numbered candidate.**

### Focus 2 — type discipline honest?

**2a — `duration` carrier landed status**: **RATIFY**. Grep-verified
at `shards/epistemologic/reality/time.mirror` (2026-06-06, commit
`c8c88f2` per shard docblock):

```
type duration = settle({ nanos: u64 })
```

Not a forward-promise. The shard imports the carrier via
`in @epistemologic/reality/time`. Composition composes.

**2b — `budget: ref` capturing BEAM's `max_restarts` semantics**:
**RATIFY-WITH-FRAMING-SHARPENING**. The `budget` carrier IS
already landed as a NAMED TYPE at `docs/math/spawn/spawn-as-loop-
monad.md` §1:

```
type budget = ref  # non-negative reduction ceiling; monotone-descent
```

The shard uses `budget: ref` (bare), not `budget: budget` (the type
alias). Adversarial: this is a minor framing inconsistency — the
substrate ALREADY named the ref specialisation as `budget`, and the
Rice-safety proof at §3.1 leans on the alias's carrier being
constrained to ℕ by the substrate's admission-rules. Using `ref`
directly gives up the specialisation.

**However**: (a) Seam's own audit `3746197` §Proposed-next-loop named
the signature `budget: ref` verbatim (which Mara honored precisely);
(b) the `type budget` alias lives in `shards/spawn.mirror` prose but
grep does NOT find it declared as a substrate `type` in any
`shards/*.mirror` file (the `type budget = ref` line lives in the
math doc, not the substrate shard). If the alias is not landed in a
shard, `ref` IS the correct floor.

**Framing sharpening (post-tick)**: land `type budget = ref` at
`shards/loop.mirror` (or `shards/spawn.mirror` when species shard
lands), then upgrade `restart_intensity.budget: ref` to
`restart_intensity.budget: budget` in a followup tick. Non-blocking
for #147 promotion; captured as a **new DEFER**.

**2c — record shape matches `3746197` signature**: **RATIFY**.
Verbatim match to Seam audit `3746197` §Proposed-next-loop:

```
type restart_intensity = { budget: ref, period: duration }
```

Byte-equal to the landed:

```
type restart_intensity = {
  budget: ref,
  period: duration,
}
```

Two-field record. Correct.

**2d — does `ref` support ordinal arithmetic (decrement)?**: **RATIFY
at declaration altitude**. The shard is a substrate-decl at
supervision altitude; the arithmetic discipline (each `bind` step
decrements budget by 1) lives at `@spawn ≤ @loop` per §3.1 halting
theorem, NOT at this shard. The shard's docblock cites §3.1's
monotone-descent explicitly ("monotone descent per `docs/math/spawn/
spawn-as-loop-monad.md` §3.1"). Correct altitude of concern:
`ref` at declaration; substrate-admission-rules constrain to ℕ at
species altitude; arithmetic lives in `@loop.advance` (which
consumes one budget unit per tick per `shards/loop.mirror`).

### Focus 3 — standalone vs bilateral framing: **RATIFY**

Mara framed as STANDALONE not fifth-#53-bilateral. Rationale
(from her §Recognition ancestry): "this shard is NOT a bilateral
instance itself — it is the CARRIER that a future bilateral pair
(`@epistemologic/property/restart_intensity_well_formed` +
`@kintsugi/fracture/restart_storm`; supervisor forward-promise
line 269-272) will predicate on. Bilaterals need something to be
bilateral ABOUT."

**Adversarial: is this substrate-pull-honest, or discipline avoidance?**
Substrate-pull-honest. Recognition #53 (property/fracture bilateral;
promoted) requires BOTH a declarative property AND an operational
fracture body. Neither exists yet; the FIRST prerequisite is the
carrier that both would predicate on. Landing all three (carrier +
property + fracture) in one tick violates craft-not-deliver AND
packs three composition-witnesses (each requiring its own audit)
into one commit.

**Adversarial: is `restart_intensity_well_formed` + `restart_storm`
the RIGHT bilateral signature?** Analytical audit:

- `restart_intensity_well_formed`: predicates on the CARRIER's
  well-formedness. Analog to `@epistemologic/property/halts` for
  bounded loops. Structural.
- `restart_storm`: names the failure surface (budget exhausted;
  restart-loop diverges). Analog to
  `@kintsugi/fracture/predicate` at a different altitude.

**Alternative bilateral signature considered**: `bounded_reductions`
+ `budget_exhausted`. This would generalize across `@spawn ≤ @loop`
uses (not just supervision altitude), and would compose with the
existing §5.4 Rice-safety proof more directly.

**Verdict**: Mara's bilateral names are AT supervision altitude
(specialisation); the alternative I'm proposing is AT `@spawn ≤ @loop`
family altitude (generalisation). The substrate needs BOTH
eventually. Mara's is correct FOR THIS SHARD (which is at supervision
altitude); the family-altitude bilateral belongs to a
`shards/loop.mirror`-adjacent tick. No collision; different altitudes.

**Ratified.** Standalone framing is correct.

### Focus 4 — composition with `@spawn ≤ @loop`: **RATIFY-WITH-CORRECTION**

**Does the shard docblock CITE this composition, or just assert it?**
The shard docblock has a dedicated section titled `=== Composition
with `@spawn ≤ @loop` ===`. It cites:

- Mara `7dba128` (the math cluster for `@spawn ≤ @loop`)
- `docs/math/spawn/spawn-as-loop-monad.md` §3.1 (halting witness)
- `docs/math/supervisor/emergent-supervision-from-geometry.md` §5.3
  (composition steps 1-5)
- `docs/math/supervisor/emergent-supervision-from-geometry.md` §5.4
  (Rice-safety via budget-as-witness)

Explicit citations, not assertions. **Cite-full.**

**Correction (from prior audit `3746197`)**: The composition-claim
that "BEAM's `max_seconds` IS the substrate-time predicate on the
loop's `tick_history`" REQUIRES a landed
`shards/epistemologic/reality/time/window.mirror` predicate. That
predicate is NOT landed (grep-verified: no `time/window.mirror`
file). The shard's Prior-art section acknowledges this indirectly
via §5.5 DEFER but does NOT flag it as a separate un-landed
composition dependency.

**Landing recommendation**: DEFER this correction as a separate
followup tick. Landing the window predicate is one tick; the
`restart_intensity` shard's carrier does not depend on the
predicate's realisation (Rice-safety at the carrier altitude holds
regardless).

**Adversarial: should there be a `requires bounded_budget(budget)`
clause?** Not at this tick. The `bounded_budget` predicate does
not yet exist as a substrate declaration; adding it as a `requires`
clause would forward-promise a predicate at declaration altitude.
The substrate-admission-rules constrain `ref` at species altitude
(per Mara `7dba128` §3.1). No additional `requires` clause needed.

**Adversarial: is there a subtype relation `restart_intensity <= spawn_budget`?**
Structurally, yes: `restart_intensity` is the record `{budget: ref,
period: duration}`; `spawn_loop.budget: ref` is one field of the
larger `spawn_loop` record. The relation is NOT direct subtyping;
it is COMPOSITION via the `@spawn ≤ @loop` species-of relation
lifted to supervision altitude. The docblock names it correctly:
"specializes that parent at SUPERVISION altitude." No subtype
notation missing.

### Focus 5 — supervisor.mirror forward-promise closure: **RATIFY**

The forward-promise at `shards/spectral/supervisor.mirror` "Cascade
siblings" section (near line 253, verified via grep) reads:

```
shards/spectral/restart_intensity.mirror
                              storm-protection carrier
                              (max_restarts: u32 + period:
                               duration), gating the
                               kintsugi-morphism-driven
                               restart loop. Without it, a
                               permanent-restart child with
                               deterministic startup failure
                               produces an unbounded loop.
                               Mirrors BEAM precedent
                               (max_restarts/max_seconds
                               circuit-breaker; on storm
                               detection the child is
                               escalated to its parent
                               supervisor's termination).
                               Pairs with bilateral instance:
                               @epistemologic/property/
                               restart_intensity_well_formed
                               + @kintsugi/fracture/
                               restart_storm.
```

**Verified**: Mara's landing closes this forward-promise. Two
observations:

1. The forward-promise names `max_restarts: u32` — a bare `u32`
   carrier. The landing UPGRADES this to `budget: ref` (typed
   carrier per [[feedback-no-bare-types]]). This IS a correction,
   not a break; the forward-promise was written 2026-06-11 before
   the `budget: ref` alias landed at spawn-as-loop-monad.md §1
   (Mara `7dba128`, 2026-07-02). The landing honors the LATER
   substrate discipline.
2. The bilateral pair (`restart_intensity_well_formed` +
   `restart_storm`) is CARRIED FORWARD as an active
   forward-promise, not closed. Correct — bilaterals need
   predicates; Mara's shard lands only the carrier.

**Closure discipline**: not `import` or `redirect`; a pure
forward-promise's closure is the existence of the named shard at
the promised path. Mara created the file at the promised path with
the promised carrier; closure = existence + carrier-match.

The supervisor.mirror docblock is NOT updated to remove the
forward-promise entry OR to add a `[LANDED]` marker. This is a
minor documentation-drift caught: **captured as a new DEFER** for
followup ("update supervisor.mirror line 253 cascade-siblings
comment to reflect the LANDING"). Non-blocking for #147 promotion.

### Focus 6 — un-cite-ability discipline: **RATIFY**

Grep-verified OID citations in the shard docblock:

- `a3dec7b` — Mara's math cluster (`emergent-supervision-from-
  geometry.md`)
- `3746197` — Seam audit ratifying the math cluster
- `ff00ec5` — Reed's numbering-collision renumber
- `7dba128` — Mara's `@spawn ≤ @loop` math cluster
- `c8c88f2` — the `duration` carrier landing

Five OID citations at declaration altitude. Un-cite-ability
theorem (`docs/math/provenance/un-cite-ability-theorem.md`)
requires citations be OID-anchored, not path-anchored. **Verified
compliant.**

The recognition-#53 ancestry section names the bilateral instance
(`restart_intensity_well_formed` + `restart_storm`) but does NOT
carry OID for it — correct, because the pair is FORWARD-PROMISED
(no OID yet). Un-cite-ability is honored when there IS an OID and
violated when a landed thing is cited path-only.

### Focus 7 — DEFER honesty: **RATIFY-WITH-FINDING**

Mara re-carried Seam's §5.5 empirical DEFER (composition claim at
supervision altitude analytical, not yet empirical) + added her
new DEFER (whether the future bilateral pair composes with
existing @kintsugi/fracture patterns at #53's fourth-instance
close, or requires new pact-altitude primitive).

**Adversarial: are there OTHER DEFERs she should have flagged?**
Grep for hidden composition claims in the docblock:

1. **Composition claim** (§Composition with `@spawn ≤ @loop`): "one
   primitive, two altitudes; no separate circuit-breaker mechanism
   needed at supervision." This is a STRONG structural claim. It
   IS analytically discharged by §5.3 + §5.4 (which the docblock
   cites). Under [[feedback-composition-claims-need-empirical-
   test]], analytical discharge is NOT enough — empirical witness
   is required. **This IS covered by the §5.5 DEFER that Mara
   re-carried.** No new DEFER needed here.

2. **Composition claim** (§Prior art): "N restarts in T seconds →
   escalate to the supervisor's own parent (per §5.4 escalation-by-
   monotone-descent)." Analytical; empirical witness pending. **This
   IS covered by the §5.5 DEFER.** No new DEFER.

3. **NEW finding — hidden compound claim** (§What restart_intensity
   IS, `period` field): "Composes with `child_spec.shutdown: duration`
   at the supervisor altitude — one substrate-time carrier for both
   graceful-termination and storm-protection windows." This is a
   COMPOSITION claim: `period` (a storm-protection window) and
   `shutdown` (a graceful-termination deadline) share the `duration`
   carrier and behave interchangeably in some sense at supervision
   altitude. **THIS IS NOT DEFERRED.** The claim conflates two
   semantically distinct time windows (period = repeating window;
   shutdown = one-shot deadline). Sharing the CARRIER does not
   mean sharing the SEMANTICS.

**New DEFER surfaced**: whether `period` and `shutdown` share
substrate-time semantics beyond byte-identity of the `duration`
carrier requires witness. Non-blocking for #147 promotion; captured
as a followup DEFER.

4. **From Focus 2b**: `budget: ref` vs `budget: budget` (the type
   alias). Captured above as a followup framing DEFER.

5. **From Focus 5**: supervisor.mirror docblock documentation-
   drift. Captured above.

**Three new DEFERs surfaced** (all non-blocking for #147 promotion):
DEFER-A (period vs shutdown semantics), DEFER-B (budget: ref vs
budget: budget alias), DEFER-C (supervisor.mirror docblock update).

---

## Single strongest adversarial finding

**DEFER-A: the `period` field's `duration` carrier shares only
BYTES with `child_spec.shutdown`, not SEMANTICS.**

The shard's docblock claims: "Composes with `child_spec.shutdown:
duration` at the supervisor altitude — one substrate-time carrier
for both graceful-termination and storm-protection windows."

Analytical decomposition:

- `child_spec.shutdown: duration` — a ONE-SHOT deadline; when the
  supervisor requests a child terminate, the child has `shutdown`
  nanoseconds to exit gracefully before being killed. The window
  fires ONCE per terminate request.
- `restart_intensity.period: duration` — a REPEATING window; every
  time it elapses, the budget resets. Semantically a
  sliding-window rate limiter.

The `duration` carrier gives both fields the same byte-shape
(`u64` nanoseconds). But the SEMANTICS differ: one-shot deadline
vs repeating window. Under
[[feedback-composition-claims-need-empirical-test]], the claim
that they "share the substrate-time carrier" is only trivially
true at byte-altitude. The claim that they share semantics at
supervision altitude is NOT empirically witnessed.

**Fix**: strike or rephrase the "one substrate-time carrier for
both graceful-termination and storm-protection windows" clause.
The stronger statement is: "the `duration` carrier's byte-shape
is shared with `child_spec.shutdown`; semantic altitude differs
(one-shot deadline vs sliding window); the shared carrier does
NOT imply shared semantics."

This is not a promotion-blocker; it is a follow-up sharpening
consistent with substrate-pull discipline. **New DEFER filed as
DEFER-A.**

---

## Ready to promote #147 to numbered candidate?

**Y.**

Reason: (a) math cluster landed at `a3dec7b` and Seam-ratified at
`3746197`; (b) shard landed at `a3dcb94` with 5/5 RED-to-GREEN
verified; (c) forward-promise at `shards/spectral/supervisor.mirror`
line 253 substantively closed by carrier existence + shape match;
(d) five OID citations honor un-cite-ability; (e) all three new
DEFERs surfaced are non-blocking (framing / documentation /
semantic-sharpening); (f) STANDALONE-not-bilateral framing is
substrate-pull-honest, not discipline avoidance.

The shard IS a substrate-pull-correct declaration of BEAM's
`max_restarts / max_seconds` at supervision altitude, composing
against `@spawn ≤ @loop` budget primitives via Rice-safe
budget-as-witness. Candidate #147 stands promoted.

---

## Next `/loop` prompt

**Recommendation: option (b) — bilateral pair land at pact +
kintsugi altitude.**

Rationale ranked against the four options:

1. **Bilateral pair land** (`@epistemologic/property/
   restart_intensity_well_formed` + `@kintsugi/fracture/
   restart_storm`) — CLOSES the fourth instance of #53
   (property/fracture bilateral pattern; second-witness gate).
   The carrier is landed; the bilateral needs the carrier to
   predicate on. Substrate-pull-ready.
2. `shards/kintsugi/surface.mirror` from kintsugi arc — separate
   arc; blocks on kintsugi arc's own progress; not this arc's
   next-tick.
3. Empirical restart-storm witness — requires a runtime
   supervisor that exercises the storm-protection loop. That's
   Alex's altitude (or a Taut multi-tick empirical scout);
   not a single-tick Mara/Seam landing.
4. Other (window predicate at
   `shards/epistemologic/reality/time/window.mirror`) — captured
   as followup DEFER at Focus 4; useful but does NOT close #147
   or advance #53.

**Proposed next `/loop` prompt (substrate-pull-honest, single-tick):**

```
Land the bilateral pair for restart_intensity:

  1. shards/epistemologic/property/restart_intensity_well_formed.mirror
     Declarative predicate on the restart_intensity carrier. The
     property discriminator: budget > 0 AND period non-zero. Structural
     analog to @epistemologic/property/halts on bounded loops.

  2. shards/kintsugi/fracture/restart_storm.mirror
     Operational fracture body naming the failure surface (budget
     exhausted; restart-loop diverges). Body discharges via
     splinter(ast) per [[architecture-splinter-ast-quote-primitive]].
     Dispatch: iii declarative on restart_intensity_well_formed's
     verdict.

Composes recognition #53 (property/fracture bilateral pattern;
promoted 2026-06-10) at a FOURTH altitude — supervision. Prior
three instances: prism/glass keyword/depth (5e68df9 + d908798),
#272 (mirror/fracture/predicate, forward-promised), portal's
shape:ref (portal.mirror). Fourth-instance close of #53 pending
this tick's Pack ratification gate.

TDD floor: 🔴 text-check tests that each shard file exists +
declares its typed carrier (property has `predicate` action;
fracture has `body` action). 🟢 land both shards with signatures +
docblocks citing:
  - #53 promotion (2026-06-10)
  - restart_intensity carrier landing (a3dcb94)
  - Seam Phase D audit (this document's OID)
  - un-cite-ability discipline (five OIDs minimum per shard)

Do NOT land the `duration/window` predicate this tick — Seam
Focus 4 followup DEFER; separate tick.
Do NOT update supervisor.mirror cascade-siblings docblock this
tick — Seam Focus 5 DEFER-C; separate tick.
Do NOT sharpen the period-vs-shutdown semantics wording — Seam
DEFER-A; separate tick.

One RED per shard. One GREEN per shard. Two shards; one
substantive tick advancing #53's fourth-instance close AND
completing the restart-intensity-shard arc's bilateral.

Post-commit, flag Seam for adversarial review of #53's
fourth-instance close (Pack ratification gate).

Craft-not-deliver.
```

---

## Discipline honored

- **Grep-first per composition claim**: verified `duration` carrier
  landed at `shards/epistemologic/reality/time.mirror`; verified
  `budget: ref` alias declared at `docs/math/spawn/spawn-as-loop-
  monad.md` §1 but NOT declared as a substrate `type budget = ref`
  in any `shards/*.mirror` file; verified forward-promise text at
  `shards/spectral/supervisor.mirror` "Cascade siblings" section;
  verified five OID citations at `shards/spectral/restart_intensity.mirror`;
  verified byte-equal signature match to Seam audit `3746197`.
- **Composition claims need empirical test**: three new hidden
  composition claims surfaced (period-vs-shutdown semantics;
  budget-ref-vs-alias; supervisor.mirror docblock drift). Only
  DEFER-A carries semantic implications; DEFER-B/C are
  framing/documentation.
- **Legibility-over-foundation**: `budget: ref` (verbatim to Seam
  audit `3746197`) is more legible than `budget: budget` alias
  (foundational); Mara's landing honors legibility.
- **Craft-not-deliver**: next `/loop` targets bilateral pair land
  (single-tick, two shards for one recognition close), not
  simultaneous kintsugi arc + empirical witness + window predicate.
- **Status-drift catch pattern** (#113 fires again): Seam Focus 5
  caught documentation-drift at supervisor.mirror line 253;
  DEFER-C filed. Numbering discipline (per `ff00ec5`) held; no
  collision.

---

*Signed Seam. Adversarial review complete. RATIFY-#147-promotion
headline; seven per-focus verdicts (five RATIFY, two RATIFY-WITH-
qualifier); three new DEFERs surfaced (all non-blocking for
promotion); single strongest finding at DEFER-A (period-vs-shutdown
semantic conflation); ready to promote Y; next `/loop` prompt at
bilateral-pair-land targeting #53's fourth-instance close.*
