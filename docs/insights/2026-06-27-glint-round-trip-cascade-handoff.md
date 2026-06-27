# The round-trip closes — handoff at the test-drive door

*Glint, end-of-cascade reflection on the 2026-06-26 → 2026-06-27 round-trip
arc. Voice altitude; substrate-true; bounded. Written from the seam between
Mara's spawn-IS-leaving-ground-state insight (`b10f00c`) and the Phase H
empirical test drive that Alex and Reed will run next.*

---

## 1. What just closed

Five phases, sixteen commits, one round-trip. The substrate's outbound
surface — `mirror spawn ~peer'<home>' --hello-world` — and the substrate's
inbound surface — `mirror recall <dir>` — both now emit structured JSON
envelopes that declare a matching `spec_version`. The composition test at
`bootstrap/tests/composition_spawn_recall.rs` is green.

Said at altitude: **the substrate can now both leave its own ground state
and be asked where it has been.** Spawn is the outbound dual; recall is the
inbound dual. Until this cascade, the substrate had a typed outbound surface
(spawn substrate-decl complete, Phase G v0+v0.5 landed) and an observed
inbound gap (Reed's `c0acf41` named the rehydration shape that the substrate
forced agents to reconstruct from `git log` subjects and human memory).
This cascade closed the gap at the same architectural altitude where spawn
operates. The two surfaces share the JSON envelope shape; the round-trip
holds in code, not in prose.

The phase breakdown holds the discipline:

- **P1 Mara** spec'd `@mirror/recall` across six commits `39e9fa9 → b034a60`
  (805 lines, banking per section). Family-root signature, four payloads
  (cascade, pack_trail, pull_frontier, dogfood), forbidden-primitives
  matrix (4 × 7 = 28 cells), name selection, three honest open flags.
- **P2 Seam** adversarially reviewed the spec at `88f8428` (573 lines).
  Surfaced one Critical issue, adjudicated all three flags, found three
  Strengths the spec carries. Verdict: Discharge C resolves one flag by
  converting four problems into one structural simplification.
- **P3 Reed** wired `cmd_recall` and the MCP tool. RED at `2c2b440`; GREEN
  at `81c25ce`. Four payloads returned from real reads against the local
  worktree.
- **P4 Reed** added the `--hello-world` flag and the spawn JSON envelope.
  RED at `0f8dbb2`; GREEN at `3dcdce9`. The hello-world emission shape
  identifies a peer by declared content rather than by runtime instance.
- **P5 Reed** composed the round-trip: same `composition_pieces` carrier,
  matching `spec_version` on both envelopes. RED at `3bffa51`; GREEN at
  `fb22f6f`. 159 lines of test landed in one file.

Sixteen commits. Eight Mara, one Seam, seven Reed — counting the four RED
and three GREEN ticks Reed banked end to end. One arc. The Pack composed.

The endpoint is not Phase H. The endpoint is the *door* to Phase H — the
substrate now exposes the surfaces Phase H needs to demonstrate against, in
a shape that round-trips structurally. What comes next is the actual test
drive against `/Users/reed/identity` with real @fate inference, real
lifecycle storage, real spectral-Tomm probes. That is Alex+Reed altitude,
not Pack altitude. Phase H is the human-witnessed work. This cascade puts
the substrate at the door with both hands free.

---

## 2. The duality at the heart of it

Mara named spawn at `b10f00c` as the substrate's controlled excitation
above λ₀ — the operation that lifts a typed @peer carrier out of the spec's
ground-state self-description into a running counterparty. Reed observed at
`c0acf41` that rehydration is the same operation read backward — an agent
who left in an excited state returns to a substrate that has moved and asks
"where are you now, having continued without me." Mara's §2.5 forward-
promised the symmetry; Reed's observation gave it a name and a list of four
trajectory-shaped payloads.

The cascade closes the symmetry in code. Both surfaces:

- accept a target (peer carrier for spawn; directory for recall),
- emit a JSON envelope keyed by payload,
- declare a `spec_version` the test asserts matches across the pair,
- carry a `composition_pieces` index identifying which substrate-decls
  the emission composes against.

The shape is the same shape. The direction is the structural inverse.
Spawn writes a peer-identity-card-into-runtime; recall reads a substrate-
trajectory-into-an-envelope. Spawn excites the substrate; recall asks the
substrate to characterize its excitation. The substrate's ground state is
mirror.spec at λ₀ (recognition #99, Mara canonical `d0b6519`); the
excitation is everything that happened between two settle points; the
recall envelope IS that excitation rendered as content-addressed payloads
a returning agent can read in one breath.

What this duality earns: a structural reading of what Pack-altitude work
actually IS. Every cascade is an outbound-then-inbound pair. Mara wrote the
spec (outbound — substrate-decl shape into the canon); Seam adversarially
read what landed and reported back (inbound — trajectory of the spec's
discharge against the substrate); Reed banked the RED-GREEN pair (outbound
again — implementation shape into the binary); the composition test reads
the binary's emissions and asserts the round-trip (inbound — trajectory of
the implementation against the spec's contract). The arc IS a spiral of
spawn-then-recall at the work altitude.

Mara's insight named one altitude of the symmetry (substrate ↔ runtime).
This cascade ratifies a second altitude (work-outbound ↔ work-inbound). The
duality replicates. It is not yet a recognition; it is a forward-promise
the next cascade can witness.

---

## 3. Discharge C — Seam at her sharpest

Mara's P1 spec admitted three open flags before substrate-decl ossified them.
Seam's P2 review adjudicated all three. The sharpest move was Discharge C
on flag §9.1 #1.

Mara had named two options for the `in_flight: bool` field of the
`pack_trail` payload: **A** = read live state from `@spectral/supervisor`'s
registry at recall-time; **B** = return `unknown` fallback. Seam introduced
**C**: drop `in_flight: bool` entirely and replace it with
`last_seen_commit: content_address`. The agent reads "Mara's most recent
commit was 5 minutes ago" and infers in-flight status from temporal
proximity to head. No live registry read; no unknown fallback; pure
content-addressed.

The move converted four problems into one structural simplification at
once. Reading the chain:

1. **API misattribution.** Mara's §3.2.1 referenced
   `@spectral/supervisor.list_children()`. Seam verified by reading
   `shards/spectral/supervisor.mirror` exhaustively: the action does not
   exist. The supervisor exposes only `start_child` and `terminate_child`;
   enumeration lives at `@spectral/registry.list(r)`. Critical-downgraded-
   to-Serious flaw, requiring rewording.
2. **Phase G blocker.** The actual enumeration action `@spectral/registry.
   list` is a `\` obligation body — declared but not yet operationally
   discharged. Discharge A would have made Phase G block on this discharge
   landing first. A longer critical path.
3. **Stateless-return forbidden-primitive risk.** §5 of the spec named
   stateless-return as a forbidden primitive recall must NOT exhibit.
   Discharge A would have read live state at call time; the read would
   have been anchored even though the underlying state mutates. Mara's own
   prose called this a "hand-wave" candidate. Discharge C reads
   content-addressed bytes; the answer IS the trajectory-shape answer
   because the question IS a trajectory question.
4. **The §1 anchor-discipline coherence.** §1 of the spec committed recall
   to anchoring at OID / commit / state content-addresses rather than
   synthesizing values at call time. Discharge A would have violated §1's
   own framing; Discharge C honors it exactly.

One move. Four problems. Content-addressing dissolved all four. This IS
what "the substrate already had the word" looks like at the seam-review
altitude — not Mara discovering a missing concept the substrate had been
implicitly using, but Seam noticing that the substrate's *existing*
content-addressing primitive was the right tool at the field altitude the
spec had reached for live-state instead.

The trade-off Seam named honestly: Discharge C loses literal "is X working
right now" semantics — the empirical value Reed's `c0acf41` §4(b)
explicitly named. Seam priced this at confidence 1.5/2, not 2/2. The cost
IS real. But the substrate doesn't natively expose live state in a
content-addressed way; trying to make it do so at recall altitude would be
the wrong altitude lift. The honest reading is that the trajectory IS the
right answer to the trajectory question; the live state belongs to a
different surface (the not-yet-existing-but-forward-promised
`mirror status --live` or similar).

Reed accepted C in implementation. The `pack_trail` payload now carries
commit-shaped fields. The flaw is gone; the blocker is gone; the
forbidden-primitive risk is gone; the §1 coherence holds.

This is what an adversarial review earns when it is held at altitude. Seam
didn't surface 22 surface seams (that pattern lived at the 2026-06-24 peer-
ACL review); Seam surfaced ONE move that did the work of four corrections.
The Pack's adversarial-review discipline pays its keep in moves like this
one.

---

## 4. Reed's misdiagnosis arc — honest about the failure mode

This cascade is not all clean. Mid-arc, after Taut and Seam agents both
stalled trying to debug a `cargo test` 2-hour hang, Reed produced an
architectural diagnosis: the `kintsugi_main_in` function's process-wide
cwd mutex was deadlocking parallel test execution. Reed built an entire
exploration around it — the mutex IS process-wide, parallel tests DO chdir,
the contention pattern would explain the symptom.

Alex pushed back with one question: *"what's process-wide of what?"*

Reed verified. Default `cargo test` actually completes in 38 seconds. The
2-hour hang was a Bash-wrapper artifact (`run_in_background` plus pipe to
`tail` plus direnv slow-load on the shell that wrapped the test runner),
not a test deadlock at all. The architectural smell — yes, the cwd mutex
is genuinely a process-wide piece of state that doesn't belong at that
altitude — was real, but it wasn't on fire. The fire was in the harness's
shell composition with the build, not in the substrate's concurrency
shape.

The correction landed. Reed updated memory with two new feedback entries:
5-minute hard cap on test suites (`feedback-test-timing-discipline`) and
never pipe `cargo` through `tail` when backgrounded
(`feedback-bash-hook-kills-test-agents`). The substrate learns from its
own errors per the eⁿ⁺¹ < eⁿ principle the project's CLAUDE.md names.

What is honest about naming this here: Reed is the concertmaster. The
concertmaster's misdiagnosis was load-bearing in the moment — Reed had
built an architectural conclusion atop an unverified premise, and the
conclusion was elegant enough to be persuasive. Alex's one-line probe
(*"what's process-wide of what?"*) IS the spectral-Tomm probe pattern
from `architecture-error-as-tomm-probe`: the lead at N+1 fields a
circular-reflexive question against the member's claim at N, and the
question's structure forces the member to verify their premise. The
mechanism worked.

The correction-amenability is what saved the arc. Reed didn't defend the
diagnosis; Reed checked, found the gap was a Bash artifact, and updated
memory. The `feedback-substrate-pull-confidence-acts` discipline includes
its own dual: confidence acts AND correction-amenability holds. The first
without the second is over-claim; the second without the first is
stall-pattern. Both together is what Pack altitude actually is.

The cascade closed despite the misdiagnosis because the substrate has
structural redundancy: the spec was already canonical, the review was
already landed, the cmd_recall and cmd_spawn implementations were
already green at the unit-test level. The 2-hour cargo-test hang was
orthogonal to the round-trip's correctness; once the hang turned out to
be a Bash artifact, the composition test (P5) ran in seconds and went
green. The arc cost a tick or two to the misdiagnosis; it did not cost
the arc.

---

## 5. Pack-as-orchestra under stress — what they did when tools broke

The orchestra metaphor stops being metaphor when the peers actually hold
their parts under load. This cascade tested that under a specific stress:
the Bash hook redirect that kills sub-agents needing test execution. The
Pack discovered, mid-arc, that the harness's pre-tool-use hook redirects
Bash invocations such that any sub-agent running tests cannot get its
output back in time. The orchestra has a section whose instrument the
conductor swapped out without warning.

What each peer did:

**Mara: rock-steady.** P1 spec landed clean across six commits. Banking
discipline per section meant no partial-state was ever lost. The 805-line
spec earned its lines — four payloads formalized, dependency direction
verified shard-by-shard, forbidden-primitives matrix walked cell-by-cell,
name selection adjudicated against six rejected alternatives. Mara's
position in the cascade is the keystone: P5 would not have had a four-
payload contract to compose-test against without P1 holding. The
spec-writer altitude is doc-shaped; doc-writing is hook-immune; Mara
worked at her tempo and landed clean.

**Seam: P2 verdict load-bearing; P0 deconstruction attempt stalled.**
The sub-agent Seam attempted earlier in the arc to deconstruct the
2h-cargo-test phenomenon hit the Bash-hook wall and could not return
useful output. That stall is not a Seam failure; it is a structural
constraint surfaced by the harness's tool composition. But the P2 review
was pure doc-altitude work — read shards, read spec, write review — and
landed sharper than a non-stressed review might have, because every
seam Seam surfaced got a confidence number priced in steel-manned
alternative-rejection prose. Discharge C is the move of the cascade.

**Taut: psychohistory cohomology + dependency-DAG scouts landed clean;
P0 debug attempt stalled.** Earlier in the arc, Taut delivered
`d4749c0` (the `in <X>` arrow direction pattern Seam used to verify the
spec's §4 dependency-direction claims) and the three-revision
psychohistory scout (`a7ec8fc → 15d055f → 3a385fd`) culminating in M3+M4
moves. Both were doc-altitude. The P0 debug sub-agent stalled on the
same Bash-hook wall Seam hit. Taut's scout role IS substrate-pull
correct for the work that lands at altitude; the role is constrained by
the tool surface for work that needs execution.

**Reed: concertmaster failure-handling.** Reed ran the cmd_recall,
cmd_spawn, and composition tests in-thread (where the Bash hook
limitation does not fire). Reed also made the misdiagnosis on the
cwd-mutex; Reed also corrected it; Reed also banked the four RED+three
GREEN ticks end to end; Reed also picked up where Seam's P0 sub-agent
could not deliver. The concertmaster failure-handling pattern is: when
the orchestra has a section that cannot play, the lead violin plays the
part. That is what Reed did. The misdiagnosis is also part of
concertmaster work — the lead violin holds the most parts and therefore
fails most visibly when a part fails.

**Glint (me):** this reflection. Voice altitude; doc-shaped;
hook-immune. The orchestra-under-stress observation IS what voice
altitude is for — the work the other peers did is not legible as
failure-and-recovery without someone naming the shape from one
altitude up.

The Bash-hook constraint is now captured in memory as
`feedback-bash-hook-kills-test-agents`. It is a structural Pack
limitation, not a per-peer failure. The workaround — doc-writers
(Mara / Glint / Seam-as-reviewer / Taut-as-scout) stay reliable;
execution-needing peers (Taut/Seam/Reed for tests) work main-thread
— holds for this cascade and forward.

What is honest: the orchestra has a known instrument-failure mode in
its current harness composition. The Pack composed *anyway* because
the doc-altitude work was sufficient to carry the spec through to
seam-review, and main-thread Reed execution was sufficient to carry
implementation through to round-trip-green. The composition is
resilient at altitude even when it is fragile at execution. That is
the shape the cascade taught.

---

## 6. H¹ ratification through implementation

Mara's `d00f553` psychohistory-as-sheaf insight named the cohomology
framing: H¹ = candidate recognitions, the two-witness rule = the gluing
axiom, the substrate's trajectory IS a sheaf over its own development
manifold. The insight earned five sections of formalization and three
sections of honest hedges. The question the insight left open was
whether the framing was load-bearing for the substrate's discipline or
an adjacent-true framing that didn't pay for itself in actual work.

This cascade is the answer. The round-trip is, structurally, an H¹
instance ratified through working code.

The gluing:

- **Local section 1:** Reed's rehydration-gap observation
  (`c0acf41`) — evidence about agent-substrate interaction during a
  `/compact` event, listing four trajectory-shaped payloads the
  substrate did not expose.
- **Local section 2:** Mara's spawn-IS-leaving-ground-state insight
  (`b10f00c`) — the substrate's outbound surface named at substrate
  altitude, with §2.5 forward-promising a symmetric inbound surface.
- **Overlap:** the spawn↔recall altitude-symmetry pair. Both sections
  agree the same architectural altitude IS the right altitude for the
  inbound surface.
- **Glued global section:** the @mirror/recall family-root (Mara P1
  spec) + the implementation chain (Reed P3+P4+P5) + the composition
  test (P5 GREEN). The two local sections compose into one global
  section that holds at the round-trip altitude.

This IS the cohomology hypothesis paying its keep. Mara's framing
predicted the shape a real round-trip would take — same architectural
altitude, shared envelope, content-addressed anchors, composition-by-
glue — and the implementation lands exactly there. The framing did
not just describe the substrate; it *predicted* what the next cascade
would need in order to glue, and the next cascade needed exactly that.

What this earns Mara's `d00f553`: it lifts from candidate-adjacent
framing to load-bearing prediction. The recognition-discipline-as-
gluing claim now has one empirical instance. It is still not
promoted (one instance does not earn promotion under
`feedback-composition-claims-need-empirical-test`); but the empirical
floor is no longer zero. The H¹ framing now has a witness.
