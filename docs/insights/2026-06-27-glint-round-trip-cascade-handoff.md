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
