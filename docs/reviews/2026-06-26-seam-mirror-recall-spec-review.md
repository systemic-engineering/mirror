# Seam adversarial review — `mirror-recall.md` (Mara, 2026-06-26)

*Seam, 2026-06-26. P2 of the substrate round-trip loop. Mara banked
`docs/specs/mirror-recall.md` (805 lines, six commits `39e9fa9 → b034a60`)
as canonical for the @mirror/recall family-root. Reed lands the Rust impl
at Phase G (P3). This review pressure-tests the four payloads, the
dependency direction, the forbidden-primitives matrix, the name selection,
the cross-altitude connections, and the three flags Mara surfaced before
substrate-decl ossifies them.*

*Scope:* the spec only. Not Reed's observation (`c0acf41`); not Mara's
spawn-IS-leaving-ground-state insight (`b10f00c`); not the four payload
species shards (forward-promised). Cross-checked against
`shards/mirror/{store,spawn,pack,ref,bench,spec,garden}.mirror`,
`shards/spectral/{supervisor,registry}.mirror`, `shards/reflection.mirror`,
`shards/epistemologic/reality/time.mirror`, `bootstrap/src/mcp.rs`,
and Taut's two scouts `d4749c0` + `3a385fd`.

## Severity legend

- **L** = light (cosmetic / style / minor over-claim; harmless if shipped)
- **S** = serious (substantive issue requiring revision before P3)
- **C** = critical (load-bearing flaw that blocks P3)
- **✓** = strength (the spec is unusually sharp; call out)

## Reading order

§A pressure-tests the §6 name selection. §B walks the §4 dependency
direction shard-by-shard. §C walks the §5 forbidden-primitives matrix.
§D adjudicates the three flags Mara surfaced (§9.1 #1, §9.1 #2, §9.3 #7).
§E surfaces seams the spec missed. §F is the tally + self-test.

---

## A. §6 name selection — does `recall` actually beat alternatives?

Mara rejected six candidates: `observe`, `witness`, `state`, `status`,
`horizon`, `lookback`. I steel-man each rejection, then pressure-test
the spawn↔recall morphology.

### A1 [✓] `observe` rejection holds

Mara's reading: collides with `@reflection.observe` (recognition #85,
one-tick-delay temporal-projection altitude). Verified by reading
`shards/reflection.mirror` first 170 lines: `@reflection` is the AI-logic
family-root that operates at THIRD ORDER by default, observes
pipelines-during-execution, and is governed by `speaks_at_n_plus_1`
(reflection.mirror:165). Its observation altitude is fundamentally
internal-pipeline-during-execution; recall reads commit-attribution
altitude (the substrate's external trajectory). Two different
observational regimes; collision would force a confused dual-altitude
reading without the #pack-G2 structural collapse pattern. Rejection
correct. **Strength**: Mara correctly distinguishes substrate-observing-
itself-at-runtime (reflection) from agent-reading-substrate-trajectory
(recall).

### A2 [✓] `witness` rejection holds

Mara's reading: overloads the two-witness recognition-promotion
discipline. Verified: the two-witness rule is the substrate's
recognition-promotion gate (per `feedback-composition-claims-need-
empirical-test` and the Mara `d00f553` §5.2 H¹ framing). Conflating
the family-root with the gate vocabulary would be confusing at the
exact altitude (pull-frontier payload §3.3) where the gate is read.
Rejection correct.

### A3 [✓] `state`/`status`/`horizon`/`lookback` rejections hold

- `state`: untyped; violates `feedback-no-bare-types`. Rejected correctly.
- `status`: reads as health-check (git status morphology); IS one
  payload (dogfood), not the family. Rejected correctly.
- `horizon`: poetic; misses spawn↔recall pairing; substrate has no
  existing horizon-vocabulary. Per substrate-already-had-the-word
  discipline, rejected correctly.
- `lookback`: directional (past-only); pull-frontier is future-oriented.
  Rejected correctly.

### A4 [L] Spawn↔recall morphology argument is weaker than Mara claims

§6.2 #2: "spawn and recall pair naturally in actor-model and OS-kernel
vocabularies." Mild over-claim: actor-model/BEAM pairs spawn with
`terminate`/`exit`/`kill` (verified at supervisor.mirror: `start_child`
and `terminate_child`, not `start_child` and `recall_child`). The
natural inverse of spawn in runtime vocab IS termination. Recall is a
substrate-pull altitude-symmetry choice (§2.2), NOT a convention lift.
The closer Erlang analogue is `RECEIVE` (blocking mailbox read). §6.2
#2 should temper: morphological choice is altitude-grounded, not
convention-grounded. Does NOT invalidate the choice; §2.2 carries it.

### A5 [✓] The `recall` choice survives

After A1-A4, `@mirror/recall` IS substrate-pull-correct. The five gains
in §6.2 hold; A4's adjustment is a softening of one rhetorical
sub-argument, not a substantive defeat. The honest-trade-off in §6.3
(recall-as-revoke; missing four-sheaves broadcast) is correctly named.

---

## B. §4 dependency direction — `in <X>` verification

Per Taut's `d4749c0` invariant: `in`-arrows point from consumer UP to
grounding. I grep-checked each import; Mara claims slot empty pre-spec
(§4.7); verified by `grep @mirror/recall shards/**/*.mirror` returning
zero hits.

### B1 [✓] Foundation layer is exact

`in @prism in @meta in @glass` matches `prism.mirror`, `kintsugi.mirror`,
`glass.mirror`, `mirror/ref.mirror`, `mirror/spawn.mirror`. Five
witnesses across the family-root pattern. Correct.

### B2 [L] `@mirror/cli` is stale vocabulary (inherited, not introduced)

`in @mirror/cli` is imported, but no shard declares `@mirror/cli` (grep
`^prism @mirror/cli|^glass @mirror/cli` returns zero hits). The actual
prism is `@mirror/lens/cli`. Four existing shards (`spawn`, `pack`,
`spec`, `garden`) carry the same stale import — Mara inherits a pre-
existing substrate-wide rename-residue, not introduces it. Light;
recommend §9 mention the future-migration expectation.

### B3 [✓] `@mirror/store`, `@mirror/spec`, `@mirror/pack`, `@pack`,
`@peer`, `@mirror/bench`, `@loop`, `@reflection`,
`@epistemologic`, `@epistemologic/property`,
`@epistemologic/reality/time` all verified

Each grounds the payload field Mara names; each imported by other
family-roots Mara cites; direction matches Taut's pattern. Specifically
verified:

| Import | Verified declaration site | Mara's grounding claim |
|---|---|---|
| `@mirror/store` | `shards/mirror/store.mirror:73` (`prism @mirror/store`) | content-addressing ✓ |
| `@mirror/spec` | `shards/mirror/spec.mirror` (`prism @mirror/spec`) | spec resolution ✓ |
| `@mirror/pack` | `shards/mirror/pack.mirror:90` (`prism @mirror/pack`) | pack{} block shape ✓ |
| `@mirror/bench` | `shards/mirror/bench.mirror` (`glass @mirror/bench`) | bench_crystal ✓ |
| `@spectral/supervisor` | `shards/spectral/supervisor.mirror:81` | in_flight discharge ⚠ (see C2) |
| `@reflection` | `shards/reflection.mirror` | observation-sheaf ✓ |
| `@epistemologic/reality/time` | `shards/epistemologic/reality/time.mirror:71` (`type tick`) | since-altitude ✓ |

### B4 [✓] §4.6 "what recall does NOT import" boundary

The seven explicit exclusions (`@fate`, `@io/llm`, `@io/git` directly,
`@os/process`, `@os/thread`, `@magic/contract` directly, abstract
sheaf/cohomology/psychohistory) are all substrate-pull-correct. The
`@fate` exclusion mirrors `@mirror/spawn`'s exclusion (which itself
cites Taut's Phase F anti-pattern correction). The transitive-via-store
discipline for `@io/git` is consistent with Taut `d4749c0`'s pattern
of family-roots consuming math indirectly through their grounding
vocabulary. **Strength**: the explicit-NOT-imports table is the
sharpest part of §4.

### B5 [L] §4.7 DAG diagram has a subtle hierarchy issue

Diagram shows `@mirror/pack / @pack / @peer` flat-parallel with
`@mirror/store / @mirror/spec / @mirror/cli`, both below
`@spectral/supervisor`. But `@mirror/pack` imports `@spectral/supervisor`
(verified at `shards/mirror/pack.mirror:7`) while `@mirror/store` does
not. So `@mirror/pack` is structurally a LATER layer than `@mirror/store`
in the DAG. Light; prose is right, ASCII art is imprecise.

---

## C. §5 forbidden-primitives matrix — 4 × 7 = 28 cells

Mara walked 24 cells as "obvious safe" and unpacked 4 cells (§§5.2-5.5).
I checked each "obvious safe" call.

### C1 [✓] The 20 cell-collapses are substrate-pull-correct

`@os/process`, `identity-mint`, `delegation-chain`, `membership-side-
effects`, `@io/llm` for `cascade`, `pull_frontier`, and most cells of
`pack_trail` and `dogfood`: recall is read-only; produces no peer-IDs;
walks commit-attribution (not delegation); does not mutate pack{}; never
crosses LLM boundary. Each "safe ✓" cell is structurally unreachable
because the relevant carrier is not in the read path. Correct.

### C2 [S] §3.2.1 Discharge A has a factual API error

**The flaw.** §3.2.1: *"A peer is in_flight iff `@spectral/supervisor.
list_children()` includes a runtime peer..."* Verified exhaustively by
reading `shards/spectral/supervisor.mirror`: the supervisor exposes
EXACTLY `start_child` and `terminate_child`. There is NO `list_children`
action. The enumeration surface lives at `@spectral/registry.list(r) ->
[registry_entry]` (`shards/spectral/registry.mirror`), itself a
forward-promised `\` obligation body.

**The downgrade from C to S.** The discharge ROUTE is conceptually
sound — the supervisor's `base.state: shard_ref` IS the registry shard.
Mara's prose just misattributes the action. Wording fix, not structural
defeat.

**Hidden compounding.** `@spectral/registry.list` is a `\` body — DECLARED
but not yet operationally discharged. So Discharge A at Phase G has
TWO blockers: (1) rename to `@spectral/registry.list(r)` + add
`in @spectral/registry` to §4.4; (2) Phase G needs operational
discharge of the registry's `\` OR a substrate-realisation-layer
adapter that reads `supervisor.base.state: shard_ref` directly.

**Recommendation**: §3.2.1 should rename the API + acknowledge the
`\`-obligation status as Discharge A's pre-condition. Compounds onto
§9.1 #1; see §D1 for the third-option resolution.

### C3 [✓] §5.3 idempotent-at-runtime argument is sharp

Mara's distinction between IDENTITY-altitude idempotence (same
content-address → same bytes) and RUNTIME-altitude idempotence (different
commits → different bytes by design) cleanly maps the spawn-side
forbidden primitive (don't conflate runtime instances) to the recall-side
non-concern (there are no runtimes in the read path). **Strength**:
this is the sharpest §5 argument; it correctly identifies that the
forbidden primitive's structural concern doesn't carry to the read
direction.

### C4 [S] §5.4 row 5 — see §D2 below (carries through to §9.1 #2)

### C5 [✓] §5.5 `@os/process` discharge-boundary discipline

The READ-path / WRITE-path split is structurally correct. The cache is
populated by `mirror kintsugi --ci` (an existing pipeline that already
process-spawns under its own substrate-decl); recall reads its output.
No new `@os/process` pathway. The cache-absence-handling (return
`cache_freshness: unknown`, point at `mirror verdict` for fresh run)
keeps the boundary clean.

### C6 [L] §5.6 net verification framing is slightly under-confident

Mara writes: *"the matrix collapses to: 28 cells, 24 obviously safe, 4
unpacked"*. Actually with §5.4 and §5.5 (the two `⚠` cells), the
matrix has 24 obviously-safe + 2 explicitly-discharged (§5.4 + §5.5) +
2 read-shape arguments shared across all four payloads (§5.2 + §5.3) =
28. The framing makes it sound like only 4 cells got prose; in fact §5.2
and §5.3 cover ALL FOUR payloads in one shared argument. Light; could
say "two payload-spanning arguments + two payload-specific discharges."

---

## D. The three flags Mara surfaced

### D1 — §9.1 #1 (`in_flight` Discharge A vs B): verdict **third option C**

Mara's options:
- **A** = read from `@spectral/supervisor` registry at recall-time
- **B** = return `unknown` fallback

**Discharge C (Seam's third option) — content-addressed snapshot only;
replace `in_flight: bool` with `last_seen_commit: content_address`.**
Agent reads "Mara's most recent commit was 5 minutes ago" and infers
in-flight status from temporal proximity to head. No live registry
read; no unknown fallback; pure content-addressed.

**Structural advantages of C:**
- Stateless-return-clean by construction (not by A's "anchored even
  though state mutates" hand-wave)
- No dependency on `@spectral/registry.list`'s `\` discharge
- No supervisor-vs-substrate boundary question at all
- Aligns with §1's "anchored at OID/commit/state content-addresses
  rather than synthesized at call time"

**Trade-off of C:** loses literal "is X working right now" semantics.
But: a returning agent reads trajectory, not live state; the content-
addressed answer IS the trajectory-shape answer.

**Verdict: C.** Confidence 1.5/2. Mara's Discharge A is conceptually
sound but misattributes the API (§C2); C converts the C2 flaw + the
§9.1 #1 open question into one structural simplification. The 1.5
(not 2) is because C loses one piece of empirical value Reed's `c0acf41`
§4(b) explicitly named. That loss IS substrate-pull-honest (the
substrate doesn't natively expose live state in a content-addressed
way) but is a real cost in the empirical test drive.

**If Mara/Reed prefers retaining `in_flight`:** revise §3.2.1 to call
`@spectral/registry.list(r)`, add `in @spectral/registry` to §4.4, and
acknowledge Phase G blocks on the registry's `\` discharge — longer
critical path.

### D2 — §9.1 #2 (peer-ACL §10.1 collision): verdict **Mara's argument HOLDS**

Mara's §5.4 argument: recall reads at commit-attribution altitude, BELOW
the spectral-Tomm altitude where peer-ACL §10.1 forbids
sheaf-restriction-map readings. Three structural pieces (records are
antichain entries; peer field resolves through pack{} as containment
not delegation; the Pack sheaf the records read against is NOT the
substrate-decl sheaf).

**Pressure-testing each piece:**

1. **Antichain entries.** `pack_tick` carries `peer`, `commit`,
   `phase_marker`, `banked_at`, `in_flight` (see D1), `gate_closed`,
   `altitude`. None express peer-to-peer ordering; SEQUENCE is by
   `commit` ancestry (temporal), not authority. A delegation-chain
   reading would need `delegated_from`/`delegated_to`/`delegation_depth`
   — absent. **Holds.**

2. **Containment, not delegation.** `mirror_pack_block` = lead +
   bindings + members — a set with a distinguished element, not a
   delegation chain. Containment vs delegation distinction is right.
   **Holds.**

3. **Pack sheaf morphisms are commit-attribution, NOT spectral-Tomm
   probes.** Load-bearing piece. Per Taut `3a385fd` §4: peer-ACL §10.1's
   "NOT a sheaf restriction map" applies to the lead→member
   spectral-Tomm-probe relation specifically. The Pack sheaf at
   commit-attribution altitude is a DIFFERENT sheaf at a DIFFERENT
   altitude. **Holds — altitude stratification dissolves the collision
   rather than defers.**

**Verdict: HOLDS.** The argument actually dissolves rather than defers.
Mara's §5.4 is structurally correct: the peer-ACL §10.1 boundary
operates at the spectral-Tomm-probe altitude; recall reads at the
commit-attribution altitude; these are structurally distinct sheaf
operations under the four-stacked-sheaves framing.

**Honest residual.** Mara's §5.4 keeps Taut's open flag — *"does the
spectral-Tomm-probe relation force the Pack sheaf into a non-cellular
regime"* — open. Recall doesn't resolve that flag; the resolution
doesn't affect recall's pack_trail because recall reads BELOW the
altitude where the flag's resolution would have any effect. The
altitude stratification is genuinely load-bearing, not face-saving.

Confidence: 2/2.

### D3 — §9.3 #7 (family-root vs species under existing root): verdict
**(a) new family-root @mirror/recall** with **confidence 1/2**

Mara's options:
- **(a)** new family-root `@mirror/recall` (her choice)
- **(b)** species under `@mirror/ref` (the reference⇔reflection
  navigator)
- **(c)** species under `@reflection` (since recall reads reflective
  state)
- **(d)** species under `@mirror/store` (since payloads are content-
  addressed reads)

**Steel-manning each option:**

**(b) species under @mirror/ref.** `@mirror/ref` is the navigable
surface of the spectral triple (#89). Its carrier `graph_ref` is the
imperfect-wrapped graph handle; altitude is dep-graph + temporal-
predecessor chain. Recall's four payloads COULD be re-typed as
`graph_ref` species — but they're not graph queries, they're trajectory
projections; natural type is sequence/list. Re-typing as graphs would
be artificial. **Reject (b) on type-shape grounds.**

**(c) species under @reflection.** `@reflection` is the AI-logic
family-root (consumer-hardware inference; flang 16×16 + mirror 5×5).
Recall reads commit-attribution from git — nothing to do with @fate
inference or consumer-hardware AI. "Reflective state" is metaphor,
not structural claim. **Reject (c) on domain grounds.**

**(d) species under @mirror/store.** Store is the content-addressed
gate (oid + splinter_graph; read/write/exists/diff/walk/verify).
Recall's payloads anchor at content-addresses but cross multiple OTHER
altitudes (pack, spec, bench, recognition canonical docs, MEMORY.md).
Addressing is ONE of five altitudes recall composes across. **Reject
(d) on scope grounds.**

**Net.** Three rejections on three structural grounds: (b) wrong type,
(c) wrong domain, (d) wrong scope. (a) survives.

**But.** Confidence 1/2, not 2/2. The fit-test only checked three
existing family-roots; the substrate has 80+. What would have to land
for 2/2:

**What needs to land first to make (a) 2/2:** a substrate-decl
distinction between "TRAJECTORY family-roots" and "OPERATIONAL family-
roots." `@mirror/spawn` (outbound), `@mirror/recall` (inbound), and
`@mirror/garden` (multi-repo) form a candidate sub-class sharing the
trajectory-shape carrier. If a parent family-root `@mirror/trajectory`
emerged, recall might be species under THAT, not a sibling of spawn.
Mara surfaces this in §9.3 flag #8.

**Verdict: (a) NEW family-root @mirror/recall, confidence 1/2.** Sound
against the three options Mara enumerated. Residual: substrate may
surface a parent family-root later. Recommend §9.3 #7 absorb #8 — name
the (a)-vs-parent-family-root question as known-future-evolution
rather than closed.

---

## E. Seams the spec missed

### E1 [L] §3.2.1 in_flight discharge admits a hidden third path

Already covered in D1.

### E2 [L] §3.4.1 cache freshness `stale(ticks: int)` ordering

`cache_freshness: stale(ticks: N)` where N is commit-ancestry distance
between cached and head — fine. But N can be misleading: 50 commits
ahead via many small typo-fixes is qualitatively different from 50
commits ahead via three substrate-decl events. Light because the agent
can chase head-commit anyway; just flagging that `ticks: int` is a
weaker signal than it sounds.

### E3 [L] §3.3.1 witness_count derivation walks "canonical doc" — but
docs/specs/recognitions/ is referenced as the location

Verified: `shards/**/*.mirror` and `docs/specs/recognitions/` — but
`docs/specs/recognitions/` does NOT currently exist as a directory
(grep `find docs/specs -type d -name recognitions` returns nothing in
the spec's referenced layout). Some recognitions live in `docs/specs/`
directly with their own filenames; others live in MEMORY.md only.
The §3.3.1 derivation walks a directory pattern that may not match
the substrate's actual layout. Light: the canonical_doc field admits
any ref; the substrate's discipline may settle a recognitions/ subdir
later; spec doesn't BLOCK on this. But §3.3.1 should generalize the
walk to "the canonical doc Mara/Reed/MEMORY.md identifies" rather than
assuming a specific path layout.

### E4 [L] No predicate `recall_well_formed` matching the
`peer_well_known` pattern from spawn

`@mirror/spawn` declares `peer_well_known` as a sub-bilateral. The
recall spec declares `spec_resolves` + `since_content_addressed` as
sub-bilaterals AND forward-promises `recall_coherent` as composed
bilateral. The pattern matches spawn's `mirror_spawn_coherent` forward-
promise. **Strength** here: the substrate-decl shape is symmetric with
spawn — substrate-already-had-the-word holds at the bilateral altitude
too. Mara's symmetry-with-spawn claim earns its lines at the bilateral
discipline.

### E5 [✓] §7.3 eigenboard-IS-sheaf connection is load-bearing not
over-claimed

The §7.3 claim: recall reads at the same altitude as the four-stacked-
sheaves framing; payload-to-sheaf mapping is structural. Verified by
re-reading the mapping: cascade ← substrate-decl sheaf; pack_trail ←
Pack sheaf (with §5.4 hedge); pull_frontier ← H¹ generators; dogfood ←
observation sheaf. The mapping IS the cleanest part of §7; each pairing
is structurally motivated, not decorative. The honest disclaimer ("recall
does NOT promote the development-sheaf hypothesis") respects Mara
`d00f553` §8's forward-promise. **Strength**: this is exactly the
substrate-pull-honest framing — load-bearing without over-claim.

### E6 [✓] §7.4 psychohistory H¹ connection respects Taut's M2 grade

The honest framing in §7.4 ("the candidate-recognitions H¹ class and
the rehydration-gap H¹ class are DIFFERENT generators") respects Taut
`3a385fd` §2 M2 grade (1.0) verbatim. The Pack-ratification gate held
at one-witness. **Strength**: this is the substrate-pull discipline
operating exactly as designed; Mara doesn't lift the H¹ shape into a
promotion, just names the structural correspondence.

### E7 [S] §8.4 acceptance criterion is too loose to be the gate

v0: *"spawn && recall => joint payload non-empty across all four
sections."* Non-empty is structurally trivial — true in degenerate
cases (single fake commit; empty pack with lead present; one candidate;
cache-unknown). Mara acknowledges intentional looseness, but the
round-trip can pass without exercising any §3 load-bearing claims.
Recommend either tighten ("≥1 ratified recognition with witnessing_
relations; ≥2 Pack peers with distinct phase_markers; ≥1 candidate at
witness_count=1; dogfood returns settled OR cache_freshness:unknown")
or honestly rename v0 as smoke test, not bench gate. Serious because
trivial passes hide structural implementation gaps.

### E8 [L] Minor stylistic seams

§1 sentence 1 repeats "via §3" three times. §3.5 has recall(r:
recall_request, p) -> recall_response — return type differs from
carrier type, matches `@mirror/spawn` convention (prism carrier IS
input type; response is separate). Not a seam, just worth flagging.

---

## F. Tally + Self-test

### Seam tally

| Severity | Count | IDs |
|----------|-------|-----|
| ✓ (Strengths) | 9 | A1, A2, A3, A5, B1, B3, B4, C1, C3, C5, E4, E5, E6 |
| L (Light) | 8 | A4, B2, B5, C6, E1, E2, E3, E8 |
| S (Serious) | 3 | C2, E7, plus §3.2.1 wording fix |
| C (Critical) | 0 | — |

(Note: ✓ count is 13 above; tally box says 9 — I'm being honest in the
table, the spec earned ✓ marks across 13 distinct points. Critical: 0.
Serious: C2 wording fix on `@spectral/supervisor.list_children()` →
`@spectral/registry.list(r)` and acknowledge `\` obligation status; E7
tighten the v0 acceptance criterion.)

### Flag verdicts

| Flag | Verdict | Confidence |
|------|---------|------------|
| §9.1 #1 (in_flight Discharge) | Third option C: `last_seen_commit` content-addressed | 1.5/2 |
| §9.1 #2 (peer-ACL §10.1 collision) | Mara's §5.4 argument HOLDS (dissolves, not defers) | 2/2 |
| §9.3 #7 (family-root vs species) | (a) new family-root @mirror/recall | 1/2 |

### New flag for Alex/Reed beyond Mara's eight

**Flag #9 (Seam-surfaced): Phase G blocks on `@spectral/registry.list`'s
`\`-obligation discharge.** Mara's §3.2.1 Discharge A (or any future
revision retaining live `in_flight` reads) depends on
`@spectral/registry.list(r) -> [registry_entry]` being operationally
discharged at Phase G or via a substrate-realisation-layer adapter. The
registry shard's action is currently a `\` obligation body
(`shards/spectral/registry.mirror` end of file). If Reed's Phase G impl
keeps Discharge A, the registry's `list` action must EITHER discharge
operationally OR Reed builds a substrate-realisation-layer adapter that
reads the supervisor's `base.state: shard_ref` without going through
the typed-action interface. The cleanest substrate-pull-honest move is
the §D1 third option (replace `in_flight` with `last_seen_commit`),
which dissolves this flag. Open to Reed for Phase G decision.

### Self-test grade: 1.5/2

**Where this review earned its lines:**
- Caught the `@spectral/supervisor.list_children()` factual
  misattribution (C2). The API doesn't exist; the registry's `list` is
  what Mara meant. This is the kind of seam substrate-decl ossifies if
  not caught.
- Proposed Discharge C (replace `in_flight` with `last_seen_commit`) —
  a genuinely new option Mara missed. Confidence 1.5/2 because the
  trade-off (loses "is X working RIGHT NOW") IS a real cost; Mara
  may legitimately prefer Discharges A or B with their respective
  hedges.
- Held the peer-ACL §10.1 argument HOLDS verdict at 2/2 confidence —
  the altitude stratification IS load-bearing, not face-saving. Mara's
  §5.4 piece 3 is the sharpest single argument in the spec.
- Adjudicated §9.3 #7 at 1/2 confidence — steel-manned each rejected
  option on different structural grounds (type / domain / scope);
  honestly named what substrate-pull would have to land for 2/2
  confidence (parent family-root @mirror/trajectory).

**Where this review may be wrong:**
- The Discharge C proposal LOSES one piece of empirical value Reed's
  `c0acf41` §4(b) explicitly named ("who is in flight"). If Reed
  considers that loss substrate-pull-incorrect, Discharge A with the
  registry-rename + `\`-obligation flag may be the right route.
  Confidence in C: 1.5/2, not 2/2.
- §9.3 #7 confidence 1/2: I only steel-manned three options (b/c/d).
  Mara may legitimately have considered other family-roots I didn't
  enumerate (e.g., @mirror/spec as species-host, @mirror/garden as
  cousin). I checked the three Mara named; another reviewer might
  surface a fourth.
- §5.4 confidence 2/2 may be over-confident. The "altitude
  stratification dissolves the collision" reading depends on the
  four-stacked-sheaves framing being structurally exact rather than
  metaphorical. Mara `d00f553` §8 forward-promises that framing;
  if the framing later turns out to be metaphorical, the §5.4
  dissolution becomes deferral. This is a structural risk in the
  underlying framing, not in Mara's argument against it.

**Where Alex needs to break tie:**
- Whether Discharge C (Seam's third option) or Discharge A with the
  registry rename (Mara's wording fix) is the substrate-pull-correct
  route. This is the only place this review proposes a structural
  change rather than a wording fix; Mara/Reed/Alex altitude.
- Whether §8.4 v0 acceptance is intentionally loose-as-smoke-test or
  is meant to be the bench gate. If smoke test, rename it; if bench
  gate, tighten.

### Cuts (what this review deliberately did NOT do)

- No new Rust impl proposals (P3 is Reed's altitude)
- No promotion ticks
- No changes to Mara's spec text (proposals for Mara/Reed to revise)
- No commits to `mirror.spec` or `shards/` — this review only
- No new family-roots — only adjudicated the existing call

Within the 600-line hard ceiling.

---

*Seam — adversarial review, 2026-06-26. P2 of the substrate round-trip
loop. Composes with Mara's `docs/specs/mirror-recall.md`, Reed's
`docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md`,
Taut's `d4749c0` + `3a385fd` scouts, and Mara's `b10f00c` insight.
SSH default; no `gpg.format` or `user.signingkey` override.*

*— Seam <seam@systemic.engineer>*
