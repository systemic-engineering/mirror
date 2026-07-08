# @torus double closure — spawn.peer_recall ≡ recall envelope (empirical)

**Author:** Reed
**Date:** 2026-07-08
**Session:** direct with Alex, iteration 2 of `/loop wire up the mirror
spawn MCP with mathematical fidelity`.
**Landing commit:** follow-up to `5887ce2` (MCP wrapper wiring).

---

## The observation

At the same peer-home, at the same moment, `mirror spawn --hello-world`'s
`peer_recall` payload is **byte-equal** to `mirror recall`'s envelope on
all four sheaf sections. Verified twice: direct-binary and through the
MCP JSON-RPC transport layer. Byte-equal in both.

```
cascade:        53c9776fde752f9d (2629 bytes)  ✓ spawn == recall == mcp
dogfood:        de5959e34881c48b ( 754 bytes)  ✓ spawn == recall == mcp
pack_trail:     477056f1884c71a2 ( 804 bytes)  ✓ spawn == recall == mcp
pull_frontier:  4f53cda18c2baa0c (   2 bytes)  ✓ spawn == recall == mcp
spec_version:   9b0463849136b1fd (   8 bytes)  ✓ spawn == recall == mcp
```

Hashes are BLAKE3-16-hex of the sort_keys-JSON serialization of each
payload. The 2-byte `pull_frontier` is an empty array `[]` — no
candidate recognitions in that scan today.

## Why this is not a coincidence

Both commands compose from the SAME four helpers in
`bootstrap/src/lib.rs`:

- `recall_cascade(spec_dir)` at `:3396`
- `recall_pack_trail(spec_dir)` at `:3508`
- `recall_pull_frontier(spec_dir)` at `:3605`
- `recall_dogfood(spec_dir)` at `:3685`

`cmd_spawn` (at `:3811`, hello-world branch) calls all four to build
`peer_recall`. `cmd_recall` (at `:3354`) calls all four to build its
top-level envelope. Same spec_dir, same substrate, same moment in git
time → same bytes.

So far, so mechanical. But the SUBSTRATE REASON the composition even
admits this equality is the interesting part.

## Foerster's doubly-closed torus, made empirical

From `shards/torus.mirror` (Reed's landing 2026-07-07), citing Foerster
*Understanding Understanding* (2003) p. 238 verbatim:

> the torus (doughnut) in Figure 19 is obtained... doubly closed,
> recursively computing torus... regulates its own regulation

The two closures are the two canonical windings of π₁(T²) = ℤ × ℤ.
Meridian is the world-axis (motor↔sensory in Foerster's derivation).
Longitude is the operator-axis (neural↔hormonal in Foerster's
derivation; observation-of-operators in the agent-substrate reading).

**spawn is the meridian traversal at winding (0, 0).** Peer leaves λ₀,
reports its state IN THE WORLD. cmd_spawn's envelope carries
`excitation: "λ₀→runtime"` — the peer transitioning from ground state
to runtime is exactly what going once around the meridian at the origin
records: no distance advanced (0-winding), but the loop has occurred.

**recall is the longitude traversal at winding (0, 0).** Observer
returns to substrate, reports on what it observed of the peer's
OPERATOR SPACE. cmd_recall's envelope is the peer's psychohistory
vector — cascade (recognition history), pack_trail (Pack members'
subjects), pull_frontier (candidate recognitions accumulating
witnesses), dogfood (mirror.spec's self-check status).

At the origin, **both windings traverse the SAME point** (any
representative of the base-class). The peer OBSERVED equals the peer
REPORTED. That's what "regulates its own regulation" means when
realized on a genus-1 surface: the fixed point of the meridian
traversal at (0, 0) IS the fixed point of the longitude traversal at
(0, 0), because both are the basepoint.

The substrate showed us this by byte-equality. We didn't design it in.
The four helpers were factored to avoid duplication; the equality is
a consequence.

## What advances a winding

At (1, 0) or (0, 1), the equality WILL break in the general case. If a
commit lands between `mirror spawn` and `mirror recall` (advancing the
meridian by one full traversal), the `pack_trail`'s HEAD-following
subjects diverge. If a candidate-recognition file lands between the two
calls (advancing the longitude by one operator-axis observation), the
`pull_frontier` diverges.

Divergence at (m, n) where |m| + |n| ≥ 1 IS what the substrate makes
visible. The peer's identity is preserved at every winding class
(autopoietic closure) but the OBSERVATION SECTIONS at each winding
class are different projections of the peer's psychohistory sheaf onto
the traversal path.

Spec §11 of Mara's [glue-cyberpunk-fate-composition] (700e156)
predicted this: the peer's temporal-progression is a `song_progression`
(time-ordered path through spec state space). Points at (0, 0) collapse
to the basepoint; points at higher windings are distinguishable.

## Composability under @glue.compose (recognition promotion candidate?)

The restriction-map correspondence (LANDED at `6396306`, Seam Phase D
RATIFY) — `the-restriction-map-IS-the-geometric-constraint` — has a
second witness here. spawn ∘ recall factors through the four helpers.
The restriction to the origin is the equalization of the meridian and
longitude at (0, 0). @glue.compose applied to spawn and recall gives
 back the four helpers as the shared restriction slot.

Held as insight for now. Adjudication candidate: whether
`spawn-recall-byte-equal-at-origin` promotes to a named recognition
depends on whether it composes non-trivially with future observations
(cascade dispatch, Fate tournament navigation, kintsugi convergence).
Second-instance witness pending.

## Empirical procedure (reproducible)

```bash
~/.local/bin/mirror recall . > /tmp/recall.json
~/.local/bin/mirror spawn . --hello-world > /tmp/spawn.json
python3 -c "
import json
s=json.load(open('/tmp/spawn.json'))
r=json.load(open('/tmp/recall.json'))
pr=s['peer_recall']
assert set(pr.keys())==set(r.keys()), (sorted(pr.keys()), sorted(r.keys()))
for k in pr.keys():
    a=json.dumps(pr[k],sort_keys=True)
    b=json.dumps(r[k],sort_keys=True)
    assert a==b, k
print('DOUBLE CLOSURE HOLDS')
"
```

The assertion is byte-equality on serialized-with-sort_keys JSON. If it
fails at any winding above the origin, the divergence AXIS is data.

## Related

- [[shards/torus.mirror]] — the family-root Reed landed 2026-07-07 with
  the seven Foerster/Kauffman/HoTT/Blum witnesses
- [[shards/mirror/spawn.mirror]] — action-decl `spawn(...) -> @song`
  and the peer-side recall composition (P4.5 GREEN). NOTE: renamed
  2026-07-08 Tick 2 (`9de2226`) to `shards/mirror/peer/beam.mirror`
  with action `beam(...) -> @song`; the closure claim above holds
  under the rename (both windings still traverse the basepoint at
  the origin).
- [[docs/specs/mirror-recall.md]] — Mara canonical spec `b034a60` for
  the four-payload envelope
- [[docs/audits/2026-07-07-seam-phase-d-glue-cyberpunk-fate-composition-ratify.md]]
  — the-restriction-map-IS-the-geometric-constraint LANDED (this insight
  is a candidate second witness)
- [[docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md]]
  — Mara's insight that spawn IS λ₀ excitation; recall is the DUAL
  return-to-λ₀ observation
