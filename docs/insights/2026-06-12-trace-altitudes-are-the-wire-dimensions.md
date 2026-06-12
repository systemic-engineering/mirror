# The @trace altitudes ARE the wire dimensions

*2026-06-12. Mara. Insight (recognition candidate; needs second witness).*

## The claim

The 13 `@trace/*` axes the substrate already names — `memory`, `type`,
`complexity`, `decidability`, `fidelity`, `staleness`, `coupling`,
`settlement`, `extraction`, `scope`, `naming`, `affect`, `honesty` — ARE
the wire payload dimensions of an eigenvalue-delta distribution
protocol.

Not 16 generic eigenvalues; not a fresh-minted wire format; not the
4-quadrant graph_observation. The SPECIFIC 13 trace altitudes the
substrate already lists. The substrate had the words for the wire
dimensions before there was a wire.

## Source

Surfaced from sister Mara β's audit of `distribution-protocol-v2.md`
(deleted in spring-clean; content preserved in the β audit's
`what-travels-the-wire` section). The audit lists the 13 axes verbatim
as the wire payload; this insight is the recognition that the same
13 names are already in circulation as `@trace/*` references throughout
the substrate (per user memory's session-2026-06-10 entry; per
`docs/specs/kintsugi-self-hosting.md` which references the `@trace`
family).

## The substrate-pull move

The substrate already has:

- `@trace/memory` — what a tick allocates / retains / frees;
- `@trace/type` — what types a tick observes / refines / discharges;
- `@trace/complexity` — the computational cost a tick incurs;
- `@trace/decidability` — whether a tick's verdict halts;
- `@trace/fidelity` — how lossy the tick's projection is;
- `@trace/staleness` — how old the substrate snapshot is;
- `@trace/coupling` — what other shards a tick touches;
- `@trace/settlement` — how close the tick's eigenvalue is to ker(Δ₀);
- `@trace/extraction` — what content leaves the substrate at this tick;
- `@trace/scope` — what altitudes the tick crosses;
- `@trace/naming` — what symbols the tick mints / shadows / resolves;
- `@trace/affect` — emotional / regulatory texture of the tick (per the
  `affect` family the substrate carries from Reed's identity);
- `@trace/honesty` — whether the tick declares what it actually did.

Each of these IS a dimension of the wire payload — *because* it's
what a tick measures, *because* it's what eigenvalue analysis
projects, *because* it's what changes when geometry rotates. The wire
ships per-tick eigenvalue deltas; the deltas decompose along these 13
axes; the substrate already named the decomposition.

The substrate didn't need to invent wire-payload dimensions. The
`@trace/*` family is the wire payload at a different altitude:
locally it's what a tick logs, distributedly it's what a tick ships.

## Adjacent altitudes — not collision

Recognition #58 (Fate IS optical inference) names the 16-feature
`graph_observation` at `@mirror/spectral/observation` as the input
layer the 5-layer D²NN reads. That's what Fate sees AT A SINGLE
SUBSTRATE INSTANCE at the local altitude.

The 13 `@trace/*` axes are what TICKS LOG locally and what THE WIRE
CARRIES distributedly. The two surfaces are adjacent but distinct
altitudes:

- `graph_observation` (16 unit_interval features) — the substrate's
  CURRENT GEOMETRY as Fate's input layer (Recognition #58's altitude).
- `@trace/*` (13 axes) — what CHANGED at the last tick, projected
  along the substrate's natural-coordinate axes (THIS insight's
  altitude).

Different altitudes; consistent direction. Tick I's
graph_observation answers "what is the substrate?"; this insight's
@trace altitudes answer "what just happened to it?" Both are valid
projections of the substrate's running state; both are absorbable as
wire-payload candidates for different consumer altitudes.

## Connection to recognition #58 and Tick I

Recognition #58's 5-layer D²NN reads `graph_observation` at the LOCAL
inference altitude (one Fate instance, one substrate snapshot). When
the eigenvalue-wire spec gets minted (post-v0.1, gated on
`@spectral/garden` ratification per the spectral-db-substrate
roadmap), the wire's per-tick deltas will be PROJECTED ALONG THE 13
@trace/* AXES, because those axes are already the substrate's typed
coordinate system for per-tick change.

The local altitude (graph_observation) and the distributed altitude
(@trace/*) compose: the Fate instance reads graph_observation to
produce a verdict; the verdict's emission ships a per-tick delta
along the 13 @trace/* axes; consumers reading the wire reconstruct
the substrate's trajectory through the @trace/* coordinate system.

## Forward-promise

When the eigenvalue-wire spec gets minted, the @trace/* axes become
the typed dimensions of the wire format. The substrate work to enable
this is small: declare a `@trace` family root naming the 13 axes (one
sub-shard per axis; each sub-shard a typed scalar — most
unit_interval, a few naturally categorical like `@trace/honesty` and
`@trace/affect`).

This insight does NOT prescribe the wire spec itself. It names the
substrate-pull alignment between what `@trace/*` already does
(local tick logging) and what the wire will need (distributed delta
shipping). The substrate's existing vocabulary IS the wire's typed
coordinate system; the spec lands the format around the existing
axes, not parallel to them.

## Status

**Candidate recognition** — not yet promoted. One witness so far
(the distribution-protocol-v2 surfacing via sister Mara β's audit).
Needs a second witness to promote per the Pack ratification
discipline.

## The second-witness question

What would a SECOND independent surfacing of the 13-@trace-axes-as-
wire-dimensions look like? A non-trivial second witness must:

1. Originate from a different altitude / source than the
   distribution-protocol-v2 audit (NOT a paraphrase of the same
   document; NOT a session note that cites it directly);
2. Either:
   - (a) name the 13 axes (or a strict subset of them) as the typed
     coordinate system of another distributed substrate boundary —
     e.g., the @spectral/garden corpus-distribution wire format
     (when phase-8 of spectral-db-substrate.md surfaces it); the
     fragmentation MCP protocol's per-call delta payload; the
     eigenboard's per-tick broadcast format. OR
   - (b) derive the 13-axis decomposition from an INDEPENDENT
     mathematical surface — e.g., a spectral decomposition of the
     substrate's natural-coordinate Dirichlet energy whose
     orthogonal directions land on exactly the 13 @trace/* axes
     (a stronger witness; would close the recognition with
     structural force).

If (a) lands, the recognition promotes with the conventional pact
ratification. If (b) lands, the recognition gains a closed-form
proof of the dimension count (why 13, not 16 or 11) — the question
"why exactly these 13 axes" becomes answerable.

The pre-registered prediction (per the prediction paradigm,
recognition #56): a future audit of the @spectral/garden distribution
wire spec, when it gets drafted, will land on the 13 @trace/* axes
as the typed payload dimensions without external prompting. If that
happens, this candidate promotes; if the garden wire spec lands on
a different dimensional surface, this candidate retracts honestly.

## Why this matters

The recognition closes a loop the substrate-pull discipline (per
[[feedback-substrate-already-had-the-word]]) has hit 52+ times:
@trace/* (what a tick logs) ↔ wire payload (what a tick ships) ↔
eigenvalue spectrum (the geometry the trace measured). All three
are the same family at three altitudes. The substrate had the words
for ALL THREE before there were systems that needed them.

Per recognition #51 (mirror IS an expanding Hilbert space): each
substrate-pull recognition widens the dimension. This candidate, if
it promotes, IS the recognition that the wire-payload altitude was
already in the Hilbert space when @trace/* was minted; the wire spec
doesn't add a dimension — it makes an existing dimension visible at
a new altitude.

## References

- Sister Mara β's audit of distribution-protocol-v2.md (the
  what-travels-the-wire section; deleted from the codebase in
  spring-clean; content preserved in the audit).
- User memory: session-2026-06-10 entry (the @trace family
  references; the substrate-pull cascade context).
- `docs/specs/kintsugi-self-hosting.md` (existing @trace family
  references in the substrate).
- Tick I's `shards/mirror/spectral/observation.mirror` — the
  16-feature graph_observation; adjacent altitude, not collision.
- Recognition #58 (Fate IS optical inference / 5-layer D²NN) — the
  local-altitude inference surface this insight's distributed
  altitude composes with.
- Recognition #51 (mirror as expanding Hilbert space) — the
  dimension-widening discipline this candidate sits inside.
- Recognition #56 (prediction paradigm orthogonal to optimization)
  — the pre-registered prediction format used in the second-witness
  question.
- `roadmap/wip/spectral-db-substrate.md` (today, Mara) — the
  open/closed guard whose phase-8 (Garden ratification) is the
  natural surfacing point for the second-witness candidate (a).
