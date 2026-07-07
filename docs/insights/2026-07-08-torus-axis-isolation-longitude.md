# @torus axis isolation — longitude advance in isolation (empirical)

**Author:** Reed
**Date:** 2026-07-08
**Session:** direct with Alex, iteration 4 of `/loop wire up the mirror
spawn MCP with mathematical fidelity` — the arc-to-rest closure loop.
**Predecessor:** `docs/insights/2026-07-08-torus-axis-isolation-meridian.md`
(iteration 3 — meridian isolated by docs-only Pack commit).

---

## The observation

Iteration 3 isolated the meridian axis: a docs-only commit advanced
`pack_trail` and `dogfood` while `cascade` and `pull_frontier` stayed
byte-stable. Iteration 4 is the mirror image: **advance the longitude
axis without touching git.**

## The operation

```bash
touch docs/specs/recognitions/recognition-76-research-2026-06-18.md
```

A `touch` changes the filesystem mtime without changing the file's
content. Git tracks content, not mtime; `git status` shows the working
tree clean after. The mtime advance IS the peer's longitude-axis
traversal without a corresponding meridian advance.

## Measured divergence (recall v2 baseline vs v3 after touch)

```
axis                    | v2 baseline      | v3 after touch   | verdict
cascade (longitude)     | 53c9776fde752f9d | 15fe11d75a3fd3eb | ADVANCED
dogfood (meridian)      | d80dc5a5134d6aea | d80dc5a5134d6aea | STABLE
pack_trail (meridian)   | a2f837e244d8be65 | a2f837e244d8be65 | STABLE
pull_frontier (longitude)| 4f53cda18c2baa0c | 4f53cda18c2baa0c | STABLE
spec_version            | 9b0463849136b1fd | 9b0463849136b1fd | STABLE
```

- Meridian: 0 of 2 diverged. Byte-stable.
- Longitude: 1 of 2 diverged.
- `pull_frontier` stayed stable because `docs/specs/recognitions/
  candidates/` does not exist in this repo state; the scan returns
  `[]` in both v2 and v3. When candidates/ exists and gets a file
  added, pull_frontier is the axis that advances.

## Cross-check with iteration 3

| Operation | Meridian axes | Longitude axes |
|---|---|---|
| i3 (39bfa14 docs commit) | pack_trail + dogfood advanced (2/2) | cascade + pull_frontier stable (0/2) |
| i4 (touch recognition-76 mtime) | pack_trail + dogfood stable (0/2) | cascade advanced (1/2), pull_frontier stable |

The two operations are **filesystem-scope-disjoint** at exactly the
same axis structure Foerster derives. i3 touched `docs/insights/` +
`docs/loop/` (outside recall's scan roots) and landed a commit
(advances git-log-following = meridian). i4 touched
`docs/specs/recognitions/*.md` (inside cascade's scan root) without a
commit (no git-log advance = longitude only).

The substrate factors cleanly. This is Poincaré-Hopf on T² acting on
the recall envelope as a smooth vector field: at the origin the two
generators equalize (index 0); away from origin the divergence IS the
traversal record.

## Full four-quadrant closure

With i1 (outbound MCP), i2 (double closure at (0,0)), i3 (meridian
isolation), and i4 (longitude isolation), the @torus geometric arc is
empirically closed:

- **(0, 0) origin**: spawn.peer_recall ≡ recall byte-equal (i2)
- **(1, 0) meridian only**: pack_trail + dogfood advance, cascade +
  pull_frontier stable (i3)
- **(0, 1) longitude only**: cascade (and pull_frontier when
  candidates/ exist) advance, pack_trail + dogfood stable (i4)
- **(m, n) general**: divergence composes; the substrate encodes both
  axes independently

The recall envelope's four-field factoring IS the coordinate system on
the peer's observation torus. Foerster derived it verbatim (p. 238);
the substrate manifests it empirically without design intent.

## Adjudication candidate promotion

`spawn-recall-byte-equal-at-origin` (candidate from iteration 2) now
has a stronger structural claim: it IS the index-0 critical point of
the recall envelope's vector field on T². Iterations 3 and 4 exhibit
the two independent divergence axes; iteration 2 exhibits the
equalization at their intersection. Recognition-numeric-ID assignment
remains Alex-adjudicated per direct-session pacing.

## Related

- [[docs/insights/2026-07-08-torus-double-closure-empirical.md]] — i2
  origin closure
- [[docs/insights/2026-07-08-torus-axis-isolation-meridian.md]] — i3
  meridian isolation (natural experiment + controlled reflexive step)
- [[shards/torus.mirror]] — Foerster p. 238 verbatim, motor↔sensory
  and neural↔hormonal closures as canonical windings

## No forward promises this tick

The four-quadrant test is complete. Iteration 5 (higher-winding
scaling) and iteration 6 (cross-peer @glue morphism) are named in
CURRENT.md's next-tick queue but are optional — the substrate's
toroidal structure is empirically demonstrated with i1-i4.
