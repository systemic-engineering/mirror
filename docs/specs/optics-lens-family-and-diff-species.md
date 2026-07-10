# @optics/lens — the sub-family + @optics/lens/diff first species

*Mara, 2026-07-11. Substrate-decl proposal spec. Landing intent:
declare the `@optics/lens` sub-family under `@optics`, then land
`@optics/lens/diff` as the first species — the linear bytes carrier
that closes the peer_beam runtime hop from @fate inference to
operator-reviewable output. Pure-📝. No `.mirror` file lands this
tick; two-tick discipline forward-promises the shards.*

---

## §0 Alex verbatim + Reed grounding

Alex 2026-07-11, closing Blocker 2 (peer_beam returns an envelope,
not the diff bytes an operator can actually edit):

> *"The gap between @fate and linear formats like the diff are
> @shatter. From where I'm standing we merely need `@shatter/lens`es
> and the first is `@shatter/lens/diff`"*

Reed grep-check on the family-root:

- `@shatter` is NOT a top-level family-root. `shards/mirror/shatter.mirror`
  and `shards/smarts/shatter.mirror` exist as species-altitude carriers.
- `@mirror/lens` exists as a namespace-parent only (its species
  `@mirror/lens/cli` at `shards/mirror/lens/cli.mirror` is the CLI
  terminal lens).
- `@optics` EXISTS as a top-level family-root (`shards/optics.mirror`,
  7.8KB, 2026-06-11 landing tied to Recognition #58 promotion).

Alex adjudicated: **`@optics/lens`** — lenses sit inside the family
whose semantics already carry "optical inference," not inside
@shatter's species altitude. This spec discharges the adjudication.

**Prior study anchors (load-bearing for the math):**

- `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` — Reed 74KB
  math: Foster/Pierce lens laws (§3), symmetric-lens adaptation
  (§3.1), Banach fixed-point iteration (§4.1), Hutchinson attractor
  for composed optics (§4.2). This spec **uses** that math; it does
  not re-derive it.
- `docs/specs/shatter-is-the-io-linearization-operator.md` — Mara
  `583b939` iter-6: `@shatter(shard, target)` IS the linearization at
  the substrate→@io boundary. Every @io crossing is a `@shatter`
  dispatch at its altitude.
- `beef270` Mara iter-17 (three loops = one operation at three
  altitudes) + `129f618` iter-18 (L(p)/P(p) fibers, bauchladen
  pullback).
- `shards/mirror/lens/cli.mirror` — the family-shape pattern this
  spec ports from.
- `shards/mirror/peer/beam.mirror` — the runtime envelope this spec
  linearizes to bytes.

---

## §1 The @optics/lens sub-family

**Placement.** `@optics/lens` sits as sub-family of `@optics`,
sibling to `@optics/source` (the active-facet gain-media species).
Where `@optics/source` names the *energy injection* side of Recognition
#58's cavity-enhanced spectrometer, `@optics/lens` names the *coherent
focusing* side — the optical element that discharges a bidirectional
equivalence between two representations of the substrate state.

**Ancestry (implicit-parent chain):**

```
in @prism         # the five-op algebra ancestor
in @glass         # verdict / imperfect / transparency
in @meta          # substrate-decl altitude
in @nl            # `# <text>` -> nl_literal on doc slots
in @optics        # PARENT family-root (@optics/lens sits here)
in @io            # the linearization crossing (Mara iter-6 §1)
```

**Family-root declaration:**

```mirror
prism @optics/lens {
  focus lens
  project lens
  split lens
  shift lens
  settle lens
}
```

**Types (Foster/Pierce lens laws, symmetric adaptation per
`docs/math/2026-07-07-shatter-as-bidirectional-lens.md` §3.1):**

```mirror
type lens_semantic  = ref      # the graph / non-linear side
type lens_linear    = ref      # the linear / byte-carrier side
type lens_get_arrow = fn(lens_semantic) -> lens_linear
type lens_put_arrow = fn(lens_linear, lens_semantic) -> lens_semantic

type lens = {
  get:          lens_get_arrow,      # semantic -> linear   (render)
  put:          lens_put_arrow,      # linear + old -> new   (parse+integrate)
  put_get_law:  bilateral,
  get_put_law:  bilateral,
  put_put_law:  bilateral,
}
```

**Actions:**

```mirror
focus(l: lens, s: lens_semantic) -> lens_linear                  # get
settle(l: lens, v: lens_linear, s: lens_semantic) -> lens_semantic  # put
split(l: lens, l': lens) -> lens                                  # compose
shift(l: lens, l': lens) -> lens                                  # tensor / parallel
project(l: lens, s: lens_semantic) -> verdict                     # law-check
```

**Bilateral properties (Foster 2007 §3, adapted to substrate
verdict-shape):**

```mirror
put_get(l: lens, s: lens_semantic, v: lens_linear, p: perturbation)
  -> verdict
  # forall v, s. get(put(v, s)) == v      # law: PutGet

get_put(l: lens, s: lens_semantic, p: perturbation)
  -> verdict
  # forall s.    put(get(s), s) == s      # law: GetPut  (round-trip)

put_put(l: lens, s: lens_semantic, v: lens_linear, v': lens_linear,
        p: perturbation)
  -> verdict
  # forall v, v', s. put(v', put(v, s)) == put(v', s)  # law: PutPut
```

The first two are Foster's *well-behaved* laws; the third is *very
well-behaved*. Per Reed math §3.2, equality is byte-equality on
`@mirror/store` OIDs — the substrate can *check* these laws at
content-address altitude without full pipeline replay.

---

## §2 @optics/lens/diff — the first species

**Ancestry:**

```
in @optics/lens                # parent sub-family
in @io                         # bytes crossing
in @kintsugi/consent           # morphism carrier (edit as consent event)
```

**The bytes carrier:**

```mirror
# Unified-diff bytes. The linear side of the (bauchladen, diff) lens.
type diff_bytes = ref(scalar/bytes)

# The semantic side: the peer's L(p)/P(p) fibered state on their
# bauchladen tray (per Mara iter-18 129f618).
type bauchladen_state = ref(@bauchladen/tray)
```

**The lens instance:**

```mirror
prism @optics/lens/diff {
  focus  diff_focus_request      # linearize direction (get)
  project diff_project_request   # law-witness direction
  split   diff_split_request     # compose with sibling lens
  shift   diff_shift_request     # parallel across peers
  settle  diff_settle_request    # integrate direction (put)
}

type diff_focus_request = { source: bauchladen_state }
type diff_settle_request = {
  edited: diff_bytes,
  base:   bauchladen_state,
}
```

**Actions:**

```mirror
# get: linearize the peer's fibered L/P state to reviewable bytes.
focus(r: diff_focus_request, p: perturbation) -> diff_bytes
{ \ }

# put: integrate operator edits back into peer state.
settle(r: diff_settle_request, p: perturbation) -> bauchladen_state
requires diff_well_formed(r.edited, p)
{ \ }
```

The `diff_bytes` carrier is deliberately narrow: the substrate's
first lens species commits to *one* linear format. Future species
(`@optics/lens/json`, `@optics/lens/patch-set`, `@optics/lens/mcp`)
discharge their own instances of the same family-root's laws.

---

## §3 Composition with peer_beam — closing Blocker 2

`shards/mirror/peer/beam.mirror` declares `beam` returning `@song`.
The missing runtime hop: from `@song` (the peer's time-indexed
trajectory) to bytes the operator can read and edit. `@optics/lens/diff`
IS that hop.

**Forward composition (peer_beam → operator terminal):**

```
mission
  |> @fate.select                 # candidate ganglion walk (#58)
  |> @bauchladen.tray.append      # enqueue L/P fiber pair on peer
  |> @kintsugi.settle             # fixed-point contract on fracture
  |> @optics/lens/diff.focus      # get: bauchladen -> diff_bytes
  |> @io.write(stdout)            # operator reads
```

**Feedback composition (operator edit → next inference):**

```
(edited_diff, old_bauchladen)
  |> @optics/lens/diff.settle     # put: diff_bytes + base -> new bauchladen
  |> @bauchladen.tray.update      # commit new fiber state
  |> @fate.select                 # next candidate incorporates the correction
```

The two directions form the closed loop. The composition typechecks
at substrate altitude: `focus` returns `diff_bytes` (linear, @io-
crossable); `settle` accepts `diff_bytes + bauchladen_state` and
returns a fresh `bauchladen_state`. peer_beam's runtime envelope is
no longer opaque — it linearizes through the lens.

---

## §4 The put direction IS autopoietic closure

This is the load-bearing recognition. The `get` direction alone (a
one-way projection from @fate to bytes) does not close the peer's
learning loop. It renders state; it does not *integrate correction*.

The `put` direction — `settle(edited_diff, base) -> new_bauchladen` —
IS the autopoietic hinge. It is the point where operator judgment
re-enters the substrate as a first-class semantic update. Per Reed
math §4.1, the kintsugi flow is Banach-contractive on the substrate
state space; each `put` supplies a fresh basepoint for the next
iteration of `Φ = parse ∘ render`. Without `put`, the peer's
fixed-point iteration runs *only* over its own priors; with `put`,
the operator's edit is a boundary condition that reshapes the
attractor.

This is Foerster's *observation-of-observation* discharged at the
lens boundary: the operator sees what the peer produced, edits it,
and the edit re-enters as a semantic update whose next linearization
the operator will see. Two loops close through one lens.

---

## §5 Relationship to @shatter — compositional, NOT obsolete

Alex's utterance named the gap as `@shatter`. Reed's grep found no
`@shatter` family-root; the sub-family properly homes at `@optics/lens`.
But the math (`docs/math/2026-07-07-shatter-as-bidirectional-lens.md`)
is not thereby dead — it is *lifted*.

**Verdict: compositional.** `@shatter` species (`shards/mirror/shatter.mirror`,
`shards/smarts/shatter.mirror`) remain load-bearing at *their* altitude:
the graph↔linear projection for the substrate's on-disk
`.shatter`-format serialization. `@optics/lens` is the family-root of
which `@shatter` is one species read through the optical lens.

In substrate-decl terms:

- `@optics/lens` — family-root, foundational, declares the Foster
  laws + the get/put pair
- `@shatter` (as it currently lives) — a species that *witnesses*
  the family at the shard-projection altitude; can be re-homed under
  `@optics/lens/shatter` in a follow-on tick (two-tick discipline)
- `@optics/lens/diff` — a sibling species discharging the SAME family
  laws at the bytes-projection altitude

The old spec's math (Banach contraction, Hutchinson attractor,
symmetric-lens formalism) applies verbatim to every species under
`@optics/lens`. The lens laws move up-family; the specializations
stay at species altitude.

---

## §6 Recursive surprises

The questions this study surfaces that Alex hasn't asked:

**1. Does @mirror/lens collide with @optics/lens?** Yes. Both name
"lens" at first-hop depth. `@mirror/lens` is a namespace-parent
(`@mirror/lens/cli` is its terminal-transport species); `@optics/lens`
is a family-root with laws. They co-exist because they are at
different semantic altitudes: `@mirror/lens/*` names *transports*
(cli/shell/mcp/lsp per `docs/specs/the-convergence.md` §1);
`@optics/lens/*` names *optical lens instances* discharging Foster
laws. The naming collision is a signal: `@mirror/lens/cli` might
rehome as `@mirror/transport/cli` in a follow-on tick, freeing
"lens" for the optical family exclusively. Not landing that here;
naming it for adjudication.

**2. Does @optics/lens need to declare `in @mirror/store`?** Probably
yes, for the OID-equality reading of the Foster laws (§3.2 of Reed
math). The `put_get` and `get_put` predicates want byte-equality at
the content-address altitude, not structural-equality at the term
altitude. Deferred to shard-landing tick — the ancestry declaration
is a one-liner cost.

**3. Where does multi-path @cyberpunk × @kintsugi collapse hook in?**
Alex's HELD vision: multi-path candidates carry weight tags; a
tournament picks. The hook: `@optics/lens/diff.settle` should accept
`list(diff_bytes)` with weight annotations, delegate to
`@fate/tournament` for selection, then integrate the winner into
`bauchladen_state`. The `settle` action signature above admits only
a single `edited: diff_bytes`; the multi-path extension is a natural
follow-on species (`@optics/lens/diff/multi-path`) — NOT the first
species, but obvious next.

**4. Is `focus` the right five-op mapping for `get`?** The @optics
family's five ops are focus/project/split/shift/settle. This spec
mapped `get -> focus` and `put -> settle`. The alternative
(`get -> project, put -> focus`) is defensible under a different
reading. Ratifying `focus=get` because focus in physical optics
names the *converging* pass (semantic→linear = converge to a single
byte-carrier), and settle names the *reflecting-back-and-stabilizing*
pass (linear+base→new_semantic = fixed-point closure). Open for
Seam adversarial review.

**5. Does the L(p)/P(p) fibered structure survive lens transport?**
Mara iter-18 (`129f618`) named L(p) as the learned fiber and P(p) as
the produced fiber over T(p) the target. The lens's `focus` collapses
both fibers into one linear artifact (the diff); the lens's `settle`
lifts one linear artifact into an update on the (L, P) pair. Question:
does `settle` update L only, P only, or both? The autopoietic closure
reading of §4 wants **both**: the operator's edit is training signal
for L *and* production signal for P. But this is not derived from
first principles here; it is proposed. Marked for adjudication.

---

## §7 Landing sequence

**Tick 1 (this tick, pure-📝):** land this spec. Substrate-decl
proposal only; no `.mirror` file changes.

**Tick 2 (follow-on, Mara or Seam):** land `shards/optics/lens.mirror`
— the family-root shard with types, actions, and the three Foster-law
bilateral predicates. Two-tick discipline: readable-name-over-
foundational — the shard name is `lens`, not `bidirectional_equivalence`.

**Tick 3 (follow-on):** land `shards/optics/lens/diff.mirror` — the
first species. Includes `focus` (get) and `settle` (put) action
bodies as `{ \ }` (obligation-block; discharge deferred to
bootstrap altitude).

**Tick 4 (bootstrap):** `bootstrap/src/optics/lens/diff.rs` — the
Rust realization discharging `focus` and `settle` against
`peer_beam`'s existing envelope carrier.

**Tick 5 (Seam Phase D):** adversarial audit on the Foster-law
predicates against `@mirror/store` OID-equality. Verify `put_get` and
`get_put` hold as byte-equality at content-address altitude per
Reed math §3.2.

Do NOT land Ticks 2–5 in-arc without Pack ratification. This spec is
the substrate-decl move; the shards follow once the pack signs off.

---

## §8 Gaps

**Not derived here:**

- The 2-cell coherence square (Reed math §3.3, §2.3). This spec
  declares three Foster laws; the HoTT 2-cell that promotes
  quasi-inverse to proper equivalence is deferred to shard-landing
  tick.
- The Hutchinson-attractor composition of `@optics/lens/diff` with
  `@optics/lens/*` siblings (Reed math §4.2). Composition is named
  via the `split` five-op; the attractor math is not re-derived.
- The @io/stagefreight wire-transport lens (`@optics/lens/mcp`,
  `@optics/lens/lsp`). Named as future species in §6; not designed
  here.
- Whether `@shatter` species rehome under `@optics/lens/shatter`.
  Flagged as compositional (§5); the actual rehoming is a follow-on
  Mara/Seam tick.
- The multi-path @cyberpunk × @kintsugi hook (§6 Q3). Named; not
  designed.
- Whether `@bauchladen` needs a corresponding family-root landing
  before `@optics/lens/diff.settle`'s return type is well-formed.
  Current spec declares `type bauchladen_state = ref(@bauchladen/tray)`;
  the `@bauchladen` family-root may or may not be landed. Verify at
  Tick 3.

**Not read this session:**

- Full `docs/math/2026-07-07-shatter-as-bidirectional-lens.md` past
  §5. Read §0-§5; skipped §6-onward.
- `shards/mirror/shatter.mirror` and `shards/smarts/shatter.mirror`
  full bodies. Read the opening docblocks of the sibling spec only.
- Ganglion species under `shards/optics/source/ganglion/*.mirror`.
- `docs/specs/composite-loss-and-learned-produced-fiber.md` (Mara
  iter-18). Named as anchor; not re-read.
- `shards/optics/source.mirror` full body. Named as sibling species
  precedent; not read.

---

*End Mara canonical proposal. Two-tick forward-promise: Tick 2
lands `shards/optics/lens.mirror` on Pack ratification. Blocker 2
(peer_beam → operator-editable bytes) is structurally closed by
the composition in §3; Rust realization at Tick 4.*
