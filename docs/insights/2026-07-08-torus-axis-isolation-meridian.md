# @torus axis isolation — meridian advance in isolation (empirical)

**Author:** Reed
**Date:** 2026-07-08
**Session:** direct with Alex, iteration 3 of `/loop wire up the mirror
spawn MCP with mathematical fidelity`.
**Predecessor:** `docs/insights/2026-07-08-torus-double-closure-empirical.md`
(iteration 2 — origin closure at winding (0, 0)).
**Landing commit:** documented reflexively — this field-log's own
commit IS the empirical winding advance that generates the second
recall snapshot.

---

## The observation

At winding (0, 0) spawn's `peer_recall` byte-equals recall's envelope
(iteration 2 result). At winding (m, n) with |m| + |n| ≥ 1 the equality
breaks. Iteration 3 asks: **which of the four sheaf sections diverges
when the peer traverses a specific winding class?**

Empirical claim (this document):

- **Meridian advance (world-axis commit lands)** → `pack_trail` and
  `dogfood` diverge; `cascade` and `pull_frontier` stay byte-equal.
- **Longitude advance (operator-axis recognition/candidate lands)**
  → `cascade` and/or `pull_frontier` diverge; `pack_trail` and
  `dogfood` change too (any commit advances meridian).
- **Pure longitude test** (touch-without-commit an existing recognition
  doc) → forward-promised as a controlled experiment; not run here.

## Natural experiment (between iteration 2 and iteration 3)

After iteration 2's byte-equal test, commit `0ab0040` landed the
(iteration 2) insight doc + updated CURRENT.md. Comparing recall
snapshots:

```
axis          | iter-2 hash        | iter-3 hash        | diverged?
cascade       | 53c9776fde752f9d   | 53c9776fde752f9d   | ✗ stable
dogfood       | de5959e34881c48b   | 77b9bc280dd2ab17   | ✓ advanced
pack_trail    | 477056f1884c71a2   | 46103e556c0db4c0   | ✓ advanced
pull_frontier | 4f53cda18c2baa0c   | 4f53cda18c2baa0c   | ✗ stable
spec_version  | 9b0463849136b1fd   | 9b0463849136b1fd   | ✗ stable
```

Commit `0ab0040` touched:

- `docs/insights/2026-07-08-torus-double-closure-empirical.md` (created)
- `docs/loop/CURRENT.md` (updated)

Neither is under `docs/specs/recognitions/` (cascade's scan root) nor
under `docs/specs/recognitions/candidates/` (pull_frontier's scan
root). Both are Pack-authored (Reed) commits (pack_trail advances)
and advance HEAD SHA (dogfood advances). Meridian isolated by
filesystem scope.

## Controlled step (this commit)

Committing THIS field-log + CURRENT.md update. Same isolation profile
expected: no changes to `docs/specs/recognitions/` or its `candidates/`
sub-directory; no changes to `mirror.spec`. Pure meridian advance.

Predicted diff:

- `cascade` ✗ byte-equal to iteration 3 baseline
- `pull_frontier` ✗ byte-equal to iteration 3 baseline
- `pack_trail` ✓ advances (new commit by Reed)
- `dogfood` ✓ advances (new HEAD SHA)

Hash comparison of `recall_v0` (baseline) to `recall_v1` (after this
commit) will be attached as the second empirical demonstration.

## What this locates on the substrate

- **Meridian axis** ≡ world-observation axis ≡ git-log-following
  operations. `pack_trail` follows Pack commit subjects; `dogfood`
  follows `HEAD` SHA and `HEAD:mirror.spec` OID. Both derive from
  `git rev-parse` / `git log` reads.
- **Longitude axis** ≡ operator-space observation axis ≡
  filesystem-mtime + recognition-doc scans. `cascade` reads
  `docs/specs/recognitions/*.md` (top-10 by mtime); `pull_frontier`
  reads `docs/specs/recognitions/candidates/*.md`. Neither reads git
  log directly; both read recognition-content substrate.

The factoring is not arbitrary. It maps Foerster's motor↔sensory
(world-axis) and neural↔hormonal (operator-axis) closures onto the
substrate's git-vs-recognition-content distinction:

- **Motor↔sensory** = what the peer DID in the world (git commits
  Pack authored, HEAD advance) → pack_trail + dogfood
- **Neural↔hormonal** = what the peer OBSERVES about its own operator
  space (which recognitions have been named, which candidates are
  accumulating witnesses) → cascade + pull_frontier

The recall envelope's four-field factoring is Foerster's two-closure
derivation MADE OPERATIONAL. Not by design (nobody sat down and said
"map the sections onto Foerster's axes") — by substrate-pull: the
natural scan roots that make sense at the recall altitude align with
the two-generator structure of π₁(T²).

## Compositional consequence

At the origin, all four sections equalize (double closure). Away
from the origin, meridian and longitude sections advance
independently. This means:

- **Two-tick divergence between spawn and recall** carries information.
  If you spawn, then commit, then recall: `pack_trail` and `dogfood`
  diverge; `cascade` and `pull_frontier` stay equal. That difference
  IS the record of what happened in the meridian axis during the
  interval.
- **The four sections are jointly the sheaf sections of the peer's
  psychohistory vector** (per `docs/insights/2026-06-26-psychohistory-
  vector-as-sheaf.md`). Their independent advance under different
  substrate operations is why the sheaf structure is load-bearing.

## Poincaré-Hopf on T² at the substrate

The torus (genus-1) has Euler characteristic χ(T²) = 0. Poincaré-Hopf
says: sum of critical-point indices of any smooth vector field on T²
is 0. At the substrate level, this means:

- For every observer-attractor (winding class where recall converges),
  there's a matched observer-repeller (winding class where recall
  diverges maximally).
- **The equalization at (0, 0) IS the substrate's index-0 critical
  point** — attractor and repeller cancel at the basepoint.
- Away from origin, the recall sections are honest observations of
  the peer's traversal path.

Bilateral by topology, not by design. The substrate encodes it.

## Related

- [[docs/insights/2026-07-08-torus-double-closure-empirical.md]] —
  iteration 2, origin closure at (0, 0)
- [[shards/torus.mirror]] — the family-root with seven witnesses; the
  Foerster motor↔sensory / neural↔hormonal decomposition (p. 238)
- [[docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md]] —
  Mara's sheaf-structure insight; the four sections are load-bearing
- [[docs/specs/mirror-recall.md]] — Mara `b034a60` canonical spec for
  the four-payload envelope
- [[bootstrap/src/lib.rs]] — `recall_cascade` `:3396`, `recall_pack_
  trail` `:3508`, `recall_pull_frontier` `:3605`, `recall_dogfood`
  `:3685`; the composition surface

## Forward-promised

- **Pure longitude test** — touch an existing `docs/specs/recognitions/
  *.md` file without committing (untracked-mtime advance) or land a
  recognition/candidate file. The controlled experiment demonstrating
  cascade and/or pull_frontier advance in isolation from pack_trail
  and dogfood. Held for iteration 4 or as a Mara/Seam RED test.
- **Higher-winding tests** — what happens at winding (2, 3) or (5, 5)?
  Multiple commits + multiple recognition landings between spawn and
  recall. The equalization surface is (0, 0) only; the divergence
  scales with |m| + |n|. Empirical scaling curve is a future arc.
- **Cross-peer divergence** — spawn peer_A, recall peer_B. Different
  spec_dir arguments. The @glue morphism between DIFFERENT peers'
  tori is where this becomes non-trivial. Also future arc.
