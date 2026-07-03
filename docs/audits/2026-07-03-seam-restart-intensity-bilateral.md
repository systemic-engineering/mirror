# Seam adversarial audit — restart-intensity-bilateral (Phase D)

**Auditor**: Seam
**Date**: 2026-07-03
**Cascade**: restart-intensity-bilateral (Phase D of intensity-shard arc)
**Under review**:
- `9f63730` — Reed RED (restart_intensity_shard, already ratified `f732d9c`)
- `a3dcb94` — Mara GREEN (`@spectral/restart_intensity` carrier, ratified `f732d9c`)
- `342d63f` — Reed RED (bilateral pair text-check, 9 tests)
- `e7bd6ec` — Mara GREEN (bilateral pair; property + fracture; 653L)

**Prior audits**: `3746197` (emergent-supervision), `f732d9c`
(restart-intensity-shard).

**Discipline invoked**: substrate-already-had-the-word, composition
claims need empirical test, un-cite-ability, legibility-over-
foundation, craft-not-deliver, status-drift catch (#113), no bare
types (#feedback-no-bare-types), grep-first per composition claim.

---

## Focus 1 — #53 fourth-instance closure

**Property signature (declarative predicate returning verdict).**
Verified against `angle_to_paren`, `keyword`, `gate`, `symbol_lift`,
`operator_match`, and all in-family bilaterals:

```
well_formed(ri: restart_intensity) -> verdict { \ }
```

Matches the #53 declarative half exactly. Compare to landed
precedents:
- `cold_compile_within_tolerance(...) -> verdict { \ }`
- `dark_count_monotone(shard: shard, n: tick, np1: tick) -> verdict { \ }`

Signature shape is byte-consistent.

**Fracture signature (opacity → morphism).** Verified:

```
resolve_restart_storm(opacity: opacity) -> morphism { \ }
```

vs. landed:
- `resolve_angle_to_paren(opacity: opacity) -> morphism { ... }`
- `resolve_keyword(opacity: opacity) -> morphism { ... }`
- `resolve_gate(opacity: opacity) -> morphism { ... }`
- `resolve_symbol(opacity: opacity) -> morphism { ... }`
- `match_operator(opacity: opacity) -> morphism { ... }`

**Signature match: byte-identical.** But — see Focus 3.

**Instance count.** Per property/fracture bilateral #53 promoted
architecture:
- Instance 1: `keyword` (2026-06-10, per-predicate).
- Instance 2: `gate` (2026-06-16, per-predicate).
- Instance 3: `angle_to_paren`, `symbol_lift`, `operator_match`
  (2026-06-19, parametric-over-table; counted as one recognition
  moment with three carriers).
- Instance 4 (this landing): `restart_intensity_well_formed` +
  `restart_storm` (2026-07-03, parametric-over-computation via
  three-mode algebra).

**Verdict: FOURTH INSTANCE CLOSED at the signature altitude**, with
caveats from Focus 3 on discharge composition.

---

## Focus 2 — Well-formedness criteria honest

**Three conjuncts landed:**
1. `budget_positive` — B₀ = 0 degenerate (rigorous — B₀=0 is the
   empty-configuration no-op).
2. `period_positive` — zero-width window undefined (rigorous — the
   ratio requires `period.nanos > 0`).
3. `ratio_within_supervision_range` — default 10 restarts/sec,
   configurable per-supervisor.

**Adversarial finding on #3 — ratio ceiling is arbitrary.**
grep-verified: `docs/math/supervisor/emergent-supervision-from-
geometry.md` §5.5 mentions the ratio (§5.5 does NOT ground a
substrate-mathematical ceiling). The "10 restarts/sec = one order
of magnitude above BEAM's 0.6" number is an operational choice
Mara names as such:

> "The initial default (10 restarts/second) is an operational
>  choice, not a substrate-mathematical one."

This is honest — the docblock does NOT claim a substrate proof.
**But**: since the ceiling is not substrate-derived, `well_formed`
depends on a value read from supervisor context, which raises the
question:

**Rice-safety concern (Focus 2b).** The docblock says `ratio_ceiling`
is "substrate-decl-configurable per-supervisor; this predicate reads
the ceiling from the supervisor's substrate context." At declaration
time this is decidable ONLY if the ceiling is a static substrate-
decl value in scope. If the ceiling is drawn from a live runtime
context, the predicate is context-dependent and its verdict is not
purely a function of the carrier — which weakens the property's
"decidable at declaration time" claim.

**Verdict**: docblock text `§Rice-safety` correctly frames the
question ("is the declaration well-formed at supervision altitude?"
— syntactic three-conjunct) but the `ratio_within_supervision_range`
conjunct is only truly syntactic if the ceiling is a compile-time
declaration. The distinction between compile-time-configurable and
runtime-configurable ceilings should be explicit in a follow-tick.

**Fourth conjunct candidate: `period_less_than_shutdown_deadline`.**
`shards/spectral/supervisor.mirror` §"child_spec" declares
`shutdown: duration`. There is a coherence relation between the
storm-protection window and the shutdown deadline: if the shutdown
deadline exceeds the restart-intensity period, a shutdown-in-flight
child cannot be counted against the intensity window (the child
hasn't finished dying yet). This is NOT landed; forward-promise if
substrate-pull confirms it recurs.

**Verdict: WELL-FORMED-ENOUGH** for #53 fourth instance, with:
- ratio ceiling grounding: RATIFY-WITH-DEFER (analytical only)
- Rice-safety: RATIFY-WITH-CLARIFICATION (compile-time vs runtime
  ceiling distinction needs sharper docblock language in the next
  tick)
- fourth conjunct: FORWARD-PROMISE candidate, not required

---

## Focus 3 — Three-mode algebra composition claim

**Mara claim**: three-mode algebra (apply/spawn/hold) from
compiler-error-surface `9f4211d` lives "AT THE MORPHISM'S CONTENT
FIELD, not at the fracture-body signature." No new pact-altitude
primitive required.

**OID citation of 9f4211d**: verified in fracture shard docblock
(4 mentions: §Fourth instance discharge; §Substrate gaps §Composes;
§Spec context; §Substrate decisions).

**Adversarial finding — the body is EMPTY.** The fracture body
`resolve_restart_storm(opacity: opacity) -> morphism { \ }` uses
the bare obligation-block placeholder `\`. Contrast this with the
landed precedents:

```
resolve_angle_to_paren(opacity: opacity) -> morphism {
  morphism {
    content: splinter(ast) { ... },
    ...
  }
}
```

`angle_to_paren`, `keyword`, `gate`, `symbol_lift`, `operator_match`
all construct a `morphism { content: splinter(ast) { ... } }`
literal. The `restart_storm` body does NOT construct anything —
the apply/spawn/hold dispatch is DESCRIBED in the docblock, but
not INSTANTIATED at the body's `\` obligation.

This is **NOT** a red-flag by itself (obligation-block-only bodies
are landed for other property shards where the discharge is
semantic-only). But the composition claim "three-mode algebra
composes at morphism.content" is analytical, not exhibited.

**Verdict on operational verification**: **ANALYTICAL ONLY**. The
composition claim is signature-consistent and vocabulary-consistent.
The operational witness (a body that actually routes on
`opacity.property` and constructs three distinct morphism.content
shapes) is DEFERRED. Mara names this DEFER explicitly and it is
substrate-pull-honest — but the composition claim's empirical
close is a real gap, not just paperwork.

**Verdict: RATIFY-WITH-DEFER.** The claim composes at the
signature altitude; the operational composition is forward-
promised. Absorption under #53's parametric-over-computation
variant holds AT THE SIGNATURE. The morphism.content dispatch
is next-tick work.

---

## Focus 4 — `spawn` mode DEFER

Mara defers "empirical composition of `spawn` mode with a live
supervisor tournament." Adversarial:

**Do substrate primitives exist to run the empirical test?**
- `@fate` tournament shard: LANDED (`shards/fate/tournament.mirror`,
  Mara 2026-06-30 `05:24`, 41.5KB). Tournament round carrier,
  well-formedness predicate, discharge surface.
- `@spawn ≤ @loop`: LANDED (Mara `7dba128`,
  `docs/math/spawn/spawn-as-loop-monad.md`).
- Live supervisor: **NOT LANDED**. `shards/spectral/supervisor.mirror`
  is a declaration shard; the runtime supervisor (fragmentation-
  mcp altitude) does not yet exercise a live tournament on
  malformed-intensity discharge.

**Verdict: GENUINE FORWARD-PROMISE.** The DEFER is not masking a
substantive gap — the tournament primitives exist; only the runtime
composition witness is missing, and that composition is the same
empirical DEFER emergent-supervision §5.5 H4 already flags. Same
DEFER, same class, no double-counting.

**RATIFY the DEFER.** The empirical test would be: (a) declare a
supervisor with restart_intensity whose ratio exceeds the ceiling;
(b) observe the fracture body routing to `spawn`; (c) observe the
tournament producing a (budget, period) pair whose ratio satisfies
the ceiling; (d) observe kintsugi convergence. Steps (a) and (d)
require runtime supervisor infrastructure not yet landed.

---

## Focus 5 — Un-cite-ability honored

**Property shard OID citations**:
- `a3dec7b` ✓ (`§5.5's H4` — twice: §Conjunct-rationale + §Spec-context)
- `a3dcb94` ✓ (three times: §Given-the-landed + §Related-shards + §Spec-context)
- `f732d9c` ✓ (three times: §Fourth-instance + §DEFERRED + §Spec-context)
- `9f4211d` ✓ (once: §Spec-context — "the three-mode algebra
  the paired fracture body's morphism proposes over")

**Fracture shard OID citations**:
- `a3dec7b` ✓ (twice: §Substrate-gaps §Composes + §Spec-context)
- `a3dcb94` ✓ (twice: §Related-shards + §Spec-context)
- `f732d9c` ✓ (twice: §Fourth-instance-forward-promise +
  §Spec-context)
- `9f4211d` ✓ (four times: §Discharge-mechanism-apply/spawn/hold
  + §Composes-cleanly + §Spec-context + §Substrate-already-had-
  the-word)

**All four OIDs cited in both shards.** Cross-arc composition
citations honored. **Verdict: RATIFY.**

---

## Focus 6 — Bilateral cross-reference (test 9)

**Test asserts**:
```
property_content.contains("restart_storm") || contains("@kintsugi/fracture")
fracture_content.contains("well_formed") || contains("@epistemologic/property")
```

**Property shard** references `restart_storm` (in `§Related shards`,
`@kintsugi/fracture/restart_storm`) AND `@kintsugi/fracture`
(as inherited family). Both branches pass.

**Fracture shard** references `well_formed` (throughout: the paired
property's `well_formed(ri)` verdict is discussed extensively) AND
`@epistemologic/property` (inherited family + related shard).

**OID-anchored per un-cite-ability?** The cross-references are
**path-anchored** (`@kintsugi/fracture/restart_storm`,
`@epistemologic/property/restart_intensity_well_formed`) — which is
the substrate's own name-syntax, not OID. Un-cite-ability names
git-OID discipline for external references; for intra-substrate
cross-references, path names ARE the content-addressed handle
because paths are the substrate's own namespace.

**Verdict: RATIFY.** Cross-references honored. Path-anchoring for
intra-substrate composition is consistent with un-cite-ability
(the substrate names its own carriers via path; OIDs are for cross-
document / cross-git-history composition).

---

## Focus 7 — Pattern-consistent naming (`*_well_formed`)

Mara claims 7+ precedents. **Grep-verified 27+ precedents**:

```
session_well_formed          — algebra/metalogue
inheritance_well_formed      — bauchladen
prose_cascade_session_well_formed — cascade/code/formal/prose
gleam_beam_cascade_well_formed    — cascade/code/gleam/beam
gleam_js_cascade_well_formed      — cascade/code/gleam/js
ps_npm_cascade_well_formed        — cascade/code/purescript/js
rust_wasm_cascade_well_formed     — cascade/code/rust/wasm
document_well_formed         — docs
measurement_well_formed      — docs
typography_well_formed       — docs/design
tea_well_formed              — docs/tea
case_study_well_formed       — docs/tea/spectral-engineer-case-study
algedonic_well_formed        — epistemologic/cybernetic/algedonic
bateson_well_formed          — epistemologic/cybernetic/bateson_learning
conversation_well_formed     — epistemologic/cybernetic/conversation
distinction_well_formed      — epistemologic/cybernetic/distinction
second_order_well_formed     — epistemologic/cybernetic/second_order
viable_well_formed           — epistemologic/cybernetic/viable
tournament_well_formed       — fate/tournament
kintsugi_proposal_well_formed — glue/fold_back
io_algebra_well_formed       — io/algebra
hash_well_formed             — io/git
ref_well_formed              — io/git
git_well_formed              — io/git
oci_well_formed              — io/oci
address_well_formed          — io/stagefreight
labeled_well_formed          — labeled
corpus_well_formed           — nl
nl_measurement_well_formed   — nl
field_well_formed            — ui
color_well_formed            — ui/mote
```

**Precedent count is 30+, not 7+.** Mara understated. This is
pattern-consistent-to-the-hilt naming.

**Altitude comparability check.** The precedents span every major
family (cybernetic, docs, io, cascade, ui, fate, glue, algebra).
Signature discipline is uniform: `X_well_formed(subject, [perturbation])
-> verdict`. `restart_intensity_well_formed` at supervision altitude
fits exactly in this pattern.

**One minor drift**: `restart_intensity_well_formed` uses NO
`perturbation` parameter, while ~60% of the precedents do. This is
substrate-pull-honest — the intensity's well-formedness is over
declaration state, not under perturbation. Compare to
`session_well_formed`, `viable_well_formed`, `algedonic_well_formed`
which similarly drop `perturbation`. Not a drift; a legitimate
variant.

**Verdict: RATIFY.** Naming discipline is grounded in 30+
precedents across the entire family tree.

---

## #53 fourth-instance closure verdict

**PARTIALLY CLOSED.** The fourth instance is closed **at the
signature altitude** (declarative predicate returns verdict;
fracture body signature is opacity → morphism; parametric-over-
computation absorbs the supervision-altitude variant). It is
**NOT YET closed at the operational altitude** — the fracture body
`\` obligation-block placeholder does NOT instantiate the three-mode
algebra dispatch, and the paired property's ceiling-reading
mechanism has a compile-time-vs-runtime distinction the docblock
softens but doesn't resolve.

The DEFERs are substrate-pull-honest, not paperwork. Closure at
the operational altitude lands when either:
- (a) A follow-tick instantiates the fracture body's three-mode
  dispatch as literal morphism.content shapes, OR
- (b) An empirical restart-storm witness exercises the discharge
  path at runtime.

Either would strong-close #53's fourth instance. The current landing
weak-closes it at signature altitude — enough to promote #53 to
`architecture-property-fracture-bilateral` update (four instances,
not three), NOT enough to close #53 as "operationally settled."

---

## Single strongest adversarial finding

**The fracture body's `\` obligation-block placeholder does NOT
instantiate the three-mode algebra it claims to compose with.** The
composition claim "three-mode algebra composes at morphism.content"
is analytically consistent at the signature altitude but not
witnessed at the body altitude. Every other landed `@kintsugi/
fracture` shard (angle_to_paren, keyword, gate, symbol_lift,
match_operator) constructs a `morphism { content: splinter(ast)
{ ... } }` literal in its body; `resolve_restart_storm` does not.
This is a genuine gap between the docblock's operational claim and
the shard's operational surface — and it is the reason the DEFER
class ("empirical composition of spawn mode") stays open.

**Recommendation**: the next `@kintsugi/fracture/restart_storm`
tick should either (a) instantiate an obligation-block body that
routes on `opacity.property` and constructs three distinct
morphism.content shapes; OR (b) explicitly document that
supervision-altitude fractures are semantic-only until runtime
supervisor infrastructure exists, and forward-promise the
body-with-dispatch to that milestone.

---

## Headline verdict

**RATIFY-WITH-DEFER**.

Bilateral pair lands at the signature altitude. Signatures are byte-
identical to landed precedents; naming discipline is grounded in
30+ precedents; OID discipline is honored throughout; three-mode
algebra composition claim is analytically consistent. Two live
DEFERs (empirical spawn-tournament + operational three-mode
dispatch in body) are substrate-pull-honest and inherited from
emergent-supervision §5.5 H4.

**#53 recognition promotion**: PROMOTE with note that the fourth
instance is signature-closed, body-deferred. The pattern's
parametric-over-computation absorption HOLDS at the signature
altitude and NEEDS the empirical witness at the body altitude.

---

## Next /loop prompt (Alex fires)

**Option (b) — Empirical restart-storm witness (spectral serve
port).**

**Rationale**:
- Closes THREE open DEFERs simultaneously (emergent-supervision
  §5.5 H4, this bilateral's spawn-mode DEFER, this bilateral's
  three-mode-body DEFER).
- Grounds the operational half of #53's fourth instance —
  transforms weak-closure to strong-closure.
- Substrate primitives already landed (@fate/tournament,
  @spawn ≤ @loop, @kintsugi/consent morphism, @glass opacity).
- The gap is runtime-supervisor infrastructure — smallest step
  is a spectral-serve MCP surface that exercises a declared
  restart_intensity in a live child.

**Ordering discipline**: (b) is the highest-leverage next tick
because it closes multiple DEFERs at once. (a) `@kintsugi/surface`
is the third residual of the earlier kintsugi arc and stays
available as the second-in-line. (c) `@spin marker for #114` and
(e) other options are lower-leverage against the current DEFER
backlog. (d) publishing un-cite-ability via spectral.engineer is
worthy but not substrate-tick work.

**Substrate-pull-confidence-act**: HIGH. The next tick is
empirical, not conceptual.

---

Signed: Seam <seam@systemic.engineer>
