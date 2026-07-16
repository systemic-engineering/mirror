# The Roomba's two motions are the first-order autopoietic compile loop

*Mara canonical spec, 2026-07-16 (Eigenboard, still blue, wine glass steady).
Discharges Alex 2026-07-16 in-transcript crystallization: `@roomba.bump`
triggers `@kintsugi`; `@roomba.vacuum` marks a `fragment` for gc. Grounds
the first-order autopoietic baseline compile loop as a `@roomba` ×
`@kintsugi` `@metalogue` at the `@algebra/metalogue` altitude.*

---

## §0 The two motions crystallization

Alex 2026-07-16 verbatim, mid-Roomba-mascot-inspection:

> *"Oh and what if the @roomba literally had a `bump` action which
> triggers the kintsugi loop? And what if it also had a `vacuum` action
> which marks a `fragment` for garbage collection (like in git, based on
> a git like dangling objects gc surface; that's also where, in the long
> run, we can configure moving gc fragments into cold storage, but that's
> future music)"*

Preceding crystallization (same session, load-bearing):

> *"@kintsugi's job then is finding the smallest possible complexity
> surface that `project`s the inferred AST back into the source code.
> The smallest least ambiguous morphism of the AST that composes into
> what the @peer inferred."*
>
> *"Wait. Wait wait wait. This is a reproducible action. This is not
> even a peer level. What if this is a @metalogue between @roomba and
> @kintsugi? The @roomba bumps into low connectivity rust code and the
> @kintsugi loop collapses it through the @fate inference metalogue into
> mirror. This is FIRST ORDER. We don't need second and third order for
> this. What if there was a @kintsugi/algebra (or is this the
> @kintsugi/fractures? I think it is. The fractures ARE kintsugi's
> algebra. We can even do `in @algebra` here)? The morphisms @kintsugi
> learns and projects back?"*
>
> *"Oh I like this. Combine this with the whole mycelial math and the
> 'gold flows into the cracks and increases the conductivity' and this
> IS the first-order autopoetic baseline compile loop."*

**What the crystallization names.** The physical Roomba's two motions
IS the substrate. `bump` is the fracture-emission trigger; `vacuum` is
the dangling-fragment garbage-collection mark. Both are Delightfully
Boring at maximum — parametric-carrier mapping from the household robot
to the compile-loop robot. The mascot named itself.

**Substrate-already-had-the-word (~71st landed instance).** `bump` was
already declared as an obligation-blocked action at
`docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` §3
(Mara `9bbebd2`, 2026-07-14; verbatim):

```mirror
bump(position: walk_position) -> spectral_tension { \ }
```

Alex's 2026-07-16 crystallization LIFTS this pre-existing action to
the first-order compile-loop trigger position. `vacuum` is the new
sibling that closes the two-motion pair the mascot always carried.

---

## §1 First-order autopoietic baseline compile loop

### §1.1 Not peer-altitude. Not second/third order.

Alex's load-bearing constraint: **the compile loop is FIRST ORDER**.
`@roomba` × `@kintsugi` is a two-species `@metalogue` at the
`@algebra/metalogue` altitude — the compiler operating on itself, one
level below `@peer`. No K → K+1 reframe; no third-order-by-default
reflexivity; no `@torus.autonomy(peer, winding)` gate. Just:

1. `@roomba` walks the substrate DAG.
2. `@roomba.bump` fires when the walker hits a low-connectivity Rust
   site (fracture-emission).
3. `@kintsugi` consumes the fracture, dispatches through
   `@kintsugi/oscillate` ACTIVE/DARK passes, projects the mended
   morphism into the source via `@kintsugi/store/git`.
4. `@roomba.vacuum` fires when the walker finds a `fragment` that no
   longer reaches from any live root (dangling), marking it for the
   two-phase gc discipline.

That's it. Both motions are first-order at the substrate's
`@algebra/metalogue` altitude. The higher-order reflections (peer
questions, Foerster's ethical imperative, `@dance` N-peer coupling)
compose OVER this baseline; the baseline compiles without them.

### §1.2 The @algebra/metalogue altitude — verbatim substrate placement

Per `shards/algebra/metalogue.mirror:170-176`:

```mirror
prism @algebra/metalogue {
  focus metalogue
  project metalogue
  split metalogue
  shift metalogue
  settle metalogue
}
```

`@roomba` speaks `bump` and `vacuum`; `@kintsugi` speaks morphism
projections back. Each speaker is a substrate-family; each utterance is
one turn in an `algebra_metalogue_session`. Both speakers' bodies are
substrate-decl'd first-class objects (not `@peer(<member>)` carriers) —
which is precisely why this is the algebra-altitude case, not the pack-
altitude case.

The `algebra_turn` body type is `algebra_morphism` per
`shards/algebra/metalogue.mirror:200-205`. Alex's 2026-07-16 flag —
"the fractures ARE kintsugi's algebra" — names each
`@kintsugi/fracture/*` species as one member of the morphism algebra
`@kintsugi` speaks in this metalogue. The `bump` action's downstream
dispatch selects which fracture-body's morphism fires; the resulting
morphism is the `algebra_turn.body`.

### §1.3 Naming the compile loop

**`@roomba × @kintsugi @metalogue`** at **`@algebra/metalogue`**.
Delightfully boring: two species speak; the algebra-metalogue substrate
was already the fifth-altitude metalogue lift; the compile loop is one
turn per bump-or-vacuum. No new family-root needed. No naming
invention. The substrate had every word.

---

## §2 `bump` action semantics

### §2.1 The signature

Additive on `shards/kintsugi/roomba.mirror`. The `\`-obligation-blocked
body composes over the landed carriers `walk_position`,
`spectral_tension`, and the `@kintsugi/fracture/*` morphism algebra.

```mirror
# bump(fracture) — the physical Roomba collision, lifted to substrate.
# When the walker's pulse detects a spectral_tension above threshold,
# the collision IS the fracture-emission; @kintsugi is dispatched at
# the same tick.
#
# Composition:
#   walk(from, budget)      →   walk_position at bump site
#   bump(walk_position)     →   spectral_tension               (LANDED 2026-07-14)
#   bump(fracture) ↓NEW↓    →   imperfect<kintsugi_dispatch>   (this landing)
#
# The two forms are the same action at two altitudes:
# - bump(walk_position)  → tension sample (what the walker MEASURES)
# - bump(fracture)       → kintsugi dispatch (what the walker EMITS)
#
# The DELTA is @cyberpunk/algedonic.sample_pain composed with
# @kintsugi/fracture/* morphism-selection: when tension exceeds the
# fracture-emission threshold (analogous to the Roomba's physical
# bumper-switch closing), the walker emits the fracture and hands off
# to @kintsugi.
bump(fracture: fracture) -> imperfect<kintsugi_dispatch, ref, transparency(ref)> { \ }
```

### §2.2 Composition graph over landed carriers

The `bump` action body composes over:

- **`@kintsugi/fracture/*`** (14 landed species; §3 below) — the morphism
  algebra; each fracture body is one morphism the walker MAY select
  based on the tension's characterization.
- **`@kintsugi/oscillate`** ACTIVE/DARK pass (2026-06-07) — dispatches
  the emitted morphism through the loop's alternation discipline.
- **`@kintsugi/consent`** `query_phi` — the auto-apply boundary the
  emitted morphism passes through before landing.
- **`@kintsugi/store/git`** projection — the terminal `@io` boundary
  where the mended bytes write back.
- **`@cyberpunk/algedonic`** `sample_pain` / `pain_gradient` (LANDED) —
  the tension-source primitives the walker's per-step sample uses.
- **`@epistemologic/cybernetic/coherence`** `coherence_score` (LANDED
  2026-07-14) — the scalar the loop climbs; each bump's downstream
  morphism MUST discharge `coherence_gradient_admissible` per the
  landed `@kintsugi/roomba` bilateral.

The composition is **substrate-honest and requires no new @io floor**.
The FLOOR is: walk the graph (Dijkstra over `ConceptGraph`, per Alex
2026-07-14 walk-IS-@io composition) + sample per-step tension. The
BUSINESS_LOGIC is the fracture-emission dispatch: substrate-decl'd,
composed via `apply_h::act` per Arc-1 Tick 1.4 CLI.

### §2.3 The `fracture` carrier

Distinct from the `Fracture` Rust struct at
`bootstrap/src/roomba_fracture.rs:91-102`. That struct is the current
@io-boundary observation. The `fracture` substrate type this spec
minces is the typed carrier `bump` takes and consumes — one located
observation of substrate stress, keyed to a `@kintsugi/fracture/*`
species that can mend it.

```mirror
# The typed observation @roomba emits when its walk-pulse detects
# spectral_tension above the fracture-emission threshold. Each
# fracture names the site + the species whose morphism can mend it.
#
# Composition ancestry:
#   opacity              (@glass; the located-fact carrier)
#   spectral_tension     (@kintsugi/roomba; per-pulse scalar)
#   fracture_species     (@kintsugi/fracture; discriminator over
#                          the 14 landed species)
type fracture = {
  site:            ref,               # opacity or Rust-file location
  tension:         spectral_tension,  # the sampled stress
  species:         fracture_species,  # which morphism-family mends this
  observed_at:     tick,              # per @epistemologic/reality/time
}
```

The `fracture_species` discriminator is a substrate-decl'd variant over
the 14 landed `@kintsugi/fracture/*` families (§3 below); adding a new
fracture species extends the variant additively.

### §2.4 The `kintsugi_dispatch` return carrier

What `bump` returns when the collision IS the fracture-emission. Names
the handoff: the fracture the walker observed, the morphism `@kintsugi`
selected, the metalogue turn the dispatch created.

```mirror
# The typed handoff a bump produces when @kintsugi accepts the
# fracture. Load-bearing: this record IS one turn in the
# @roomba × @kintsugi @metalogue at @algebra/metalogue altitude
# (per §1.2). algebra_metalogue_session.turns append this record.
type kintsugi_dispatch = {
  fracture:        fracture,           # what the walker emitted
  morphism:        morphism,           # what @kintsugi selected
                                       #  (per @kintsugi/consent)
  metalogue_turn:  algebra_turn,       # the turn added to the session
}
```

### §2.5 Threshold discipline — when does bump fire?

Per landed `@kintsugi/roomba` §"knife_verdict_bounded": each walker step
emits `knife_verdict ∈ {Stable, NearBoundary, Jumped}`. The
fracture-emission threshold IS the `Jumped` verdict — when
`@mirror/lens/knife.jump` fires because the per-step coherence delta
crosses `@cyberpunk.epsilon_pain` (Reed's provisional 0.01 per
`docs/specs/peer-as-pain-driven-bounded-ontological-navigator.md`).

Alex 2026-07-14 verbatim: *"@roomba walks (Dijkstra + tension-weighted
edges) → bumps into spectral @tension at position p → resonance emits
@song beats → @kintsugi consumes @song and decides."* The **resonance
emission** IS `bump`'s return carrier; the **decision** IS `@kintsugi`'s
downstream act on the dispatched morphism.

The physical mascot: a Roomba's bumper closes an electrical contact when
it hits furniture; the substrate's `bump` closes the metalogue turn when
tension crosses threshold. Both are boolean-at-threshold, both are
discharged by physical (or spectral) contact, both trigger the next
motion.

---

## §3 `vacuum` action semantics

### §3.1 The signature

Additive on `shards/kintsugi/roomba.mirror`.

```mirror
# vacuum(fragment) — the physical Roomba's dust-collection motion,
# lifted to substrate. When the walker observes a fragment that no
# longer reaches from any live root (dangling), the vacuum motion
# marks it for two-phase gc per @mirror/store's dangling-object
# discipline (§5 below).
#
# Composition:
#   walk_dangling(root)     →   [fragment]                     (§5; NEW)
#   vacuum(fragment) ↓NEW↓  →   imperfect<gc_mark>             (this landing)
#
# The two forms of the vacuum motion:
# - walk_dangling  → enumeration of dangling fragments (what the
#                     walker DISCOVERS during traversal)
# - vacuum         → per-fragment marking (what the walker EMITS
#                     into the store's gc surface)
#
# Two-phase discipline per git-gc precedent (§5.3): vacuum MARKS
# the fragment; a subsequent prune_expired action DELETES fragments
# whose mark-age exceeds the prune horizon. This preserves the
# git-standard "loose objects protected by --prune=<duration>"
# semantics documented at git-scm.com/docs/git-prune.
vacuum(fragment: fragment) -> imperfect<gc_mark, ref, transparency(ref)> { \ }
```

### §3.2 The `fragment` carrier

Alex 2026-07-16 verbatim: *"a `vacuum` action which marks a `fragment`
for garbage collection."* Substrate-already-had-the-word check:
`fragment` is the vocabulary used across `bootstrap/src/roomba_fracture.rs`
docblocks, `fragmentation/src/fragment.rs::Fractal<String>`, and
`shards/mirror/store/git.mirror`'s `fractal` type (the wire-altitude
fragment carrier). Alex's naming aligns; this spec adopts `fragment` as
the typed carrier the vacuum action takes.

```mirror
# A dangling content-addressed unit the vacuum motion marks for gc.
# Distinct from the wire-altitude `fractal` at @mirror/store/git and
# from the atomic `splinter` at @glass — those are LIVE carriers; a
# `fragment` is what those carriers become when no live root reaches
# them.
#
# Composition ancestry:
#   oid                  (@mirror/store; content-address)
#   splinter_graph       (@mirror/store; the (root, children)
#                          projection whose closure defines reachability)
#   tick                 (@epistemologic/reality/time; observed_at
#                          for the two-phase mark-age discipline)
type fragment = {
  oid:              oid,                # the content address that is dangling
  observed_at:      tick,               # when the vacuum first saw it dangling
                                        #   (mark-age origin for prune horizon)
  discovered_by:    walk_position,      # the walker's position at discovery
}
```

The `oid` field identity-couples the fragment to `@mirror/store`'s
content-address discipline; `observed_at` grounds the two-phase mark-age
per git-gc precedent (§5.3).

### §3.3 The `gc_mark` return carrier

```mirror
# What vacuum emits when the fragment is registered in the gc surface.
# The mark IS the record; prune reads the mark's age against
# --prune=<duration> to decide deletion. Two-phase per §5.3.
type gc_mark = {
  fragment:         fragment,
  marked_at:        tick,               # when @mirror/store recorded
                                        #   the mark (may differ from
                                        #   observed_at by store latency)
  prune_horizon:    tick,               # marked_at + configured
                                        #   grace-period per store's
                                        #   --prune=<duration> analog
}
```

### §3.4 Cold-storage forward-promise (per Alex 2026-07-16)

Alex verbatim: *"that's also where, in the long run, we can configure
moving gc fragments into cold storage, but that's future music."* The
substrate NAMES the extension surface here, does NOT land it this arc:

**Forward-promise:** a future `@mirror/store/cold` species will extend
the two-phase gc discipline with a THIRD phase — `migrate_to_cold`
(between `vacuum` mark and `prune_expired` delete). The mark's age past
the prune horizon does not delete; it moves the fragment's content to
a cold-storage species (`@mirror/store/s3`, `@mirror/store/glacier`,
etc.) with the fragment's OID preserved so a future `read(oid)` can
resurrect it under bounded I/O cost. This lands when consumers pull;
substrate names it now so the two-phase discipline stays extensible.

The precedent Alex is composing on: git-annex's `git annex drop` /
`git annex get` two-remote discipline (loose object HERE, canonical
copy ON REMOTE); S3 Lifecycle policies (Standard → Standard-IA →
Glacier tier transitions on age); IPFS pinning as "protected from gc"
plus optional cold-remote transitions. All three prior arts converge
on the same shape: two-phase gc with an optional third phase that
migrates rather than deletes.

### §3.5 Threshold discipline — when does vacuum fire?

Unlike `bump` (fires on tension threshold), `vacuum` fires on
**reachability threshold** — when the walker discovers a fragment
whose reachability from all live roots is zero. The reachability walk
IS the substrate's declared `@mirror/store.walk` closure, computed
FORWARD from every live root; `vacuum`'s trigger is the SET
DIFFERENCE between the store's total OIDs and the union of all live
roots' walk closures. Fragments in the difference are dangling; each
is `vacuum`-eligible.

---

## §4 `@roomba × @kintsugi @metalogue` — the algebra-metalogue turn

The load-bearing composition Alex 2026-07-16 crystallized: `@roomba`
and `@kintsugi` are two algebras exchanging morphisms at the
`@algebra/metalogue` altitude. Every `bump` is one turn:

```
algebra_turn {
  speaker:     @roomba,                # the walker-algebra emitting
  body:        <selected fracture-morphism>,  # from @kintsugi/fracture/*
  in_reply_to: option(prior_turn),
  tick:        observed_at,
}
```

Every `@kintsugi` response — the morphism selected via `query_phi` and
applied via ACTIVE/DARK oscillation — is the NEXT turn in the session:

```
algebra_turn {
  speaker:     @kintsugi,              # the mender-algebra responding
  body:        <applied morphism_context>,    # from @kintsugi/morphism
  in_reply_to: Some(<the bump turn>),
  tick:        applied_at,
}
```

The `algebra_metalogue_session` composed of these turn-pairs IS the
compile-loop transcript. Per
`shards/algebra/metalogue.mirror:271-273`: `compose_turns(t_bump,
t_mend)` returns `Some(composite_turn)` when composability holds
(bump's fracture-species matches mend's morphism-family); `None`
otherwise (dispatch mismatch, adjudicable at Seam altitude).

Same shape for `vacuum` × `@mirror/store.prune`:

```
algebra_turn {
  speaker:     @roomba,
  body:        <vacuum motion on fragment>,
  ...
}
algebra_turn {
  speaker:     @mirror/store,
  body:        <mark recorded, prune scheduled>,
  ...
}
```

Non-commutative per `shards/algebra/metalogue.mirror:90-95`:
`compose(bump, vacuum) ≠ compose(vacuum, bump)`. Bumping a
newly-mended morphism is not the same as vacuuming its predecessor;
the metalogue substrate preserves the ordering.

---

## §5 `@mirror/store` gc composition surface

### §5.1 The gap

Per grep-verified prior state (2026-07-16, Reed pre-spawn): `@mirror/store`
has ZERO landed gc/prune/dangling substrate. Reachability semantics
exist implicitly — `set_ref` on `write` per `@mirror/store/git`
discharge map:

```
write(bytes) <= insert_persistent(store, oid, fractal, size)
             +  set_ref(store, ref_name, oid)
```

This establishes reachability from a named ref. But the substrate has
no primitive answering "which OIDs are NOT reached from any live
ref?" — the dangling-object question git-gc asks.

### §5.2 The three new primitives (additive on `@mirror/store`)

Delightfully-boring naming per git-scm.com vocabulary (git's
`git fsck --dangling`, `git prune`, `git gc --auto`):

```mirror
# walk_dangling(refs) — enumerate the OIDs in the store whose forward
# reachability from ANY live ref is zero. The complement of the union
# of walk(ref) closures over all named refs. Realizes the git-fsck
# "dangling objects" surface at family-root altitude.
#
# refs: the set of live named-ref oids the store considers roots.
#        Empty list means "no live roots" — every OID is dangling.
walk_dangling(refs: [oid]) -> [fragment] { \ }

# mark_unreachable(fragment) — record the fragment's dangling
# observation in the store's gc metadata. Idempotent on
# fragment.oid: re-marking the same OID within a single prune-horizon
# window is a no-op on the mark-age (preserves the earliest observation).
# The gc_mark returned carries prune_horizon = marked_at + configured
# grace-period.
mark_unreachable(fragment: fragment) -> imperfect<gc_mark, ref, transparency(ref)> { \ }

# prune(store, before) — delete every fragment whose gc_mark.prune_horizon
# is strictly less than `before`. Two-phase discipline: this is the
# DELETE phase; marks are the MARK phase. Realizes git-gc's
# `--prune=<duration>` semantics.
#
# before: cutoff tick. Fragments marked before this tick's
#          `marked_at - grace_period` are pruned.
prune(store: git_store, before: tick) -> verdict { \ }
```

### §5.3 Two-phase gc discipline — the git-scm precedent

Per git documentation (git-scm.com/docs/git-prune; retrieved 2026-07-16):
git-gc's dangling-object discipline is TWO-PHASE. Phase 1 (`git gc`)
identifies unreachable objects but does NOT delete them; they remain
as "loose" or in a separate pack. Phase 2 (`git prune` with
`--expire=<duration>`) deletes only objects OLDER than the grace period.
Default grace period is TWO WEEKS.

Rationale (from git-gc(1) man page): "prunes loose objects regardless
of their age... increases the risk of corruption if another process is
writing to the repository concurrently." The two-phase gap is a safety
window — concurrent writes that reference a not-yet-committed object
have time to establish the reference before the object is pruned.

The substrate INHERITS this discipline verbatim:

1. `walk_dangling` + `mark_unreachable` = git's Phase 1 (identify +
   record).
2. `prune(before)` = git's Phase 2 (delete only past-horizon marks).

The `prune_horizon` on `gc_mark` IS the substrate-decl form of git's
`--prune=<duration>` argument. Adopters configure the store's
grace-period at open-time; the substrate names the shape at
substrate-altitude.

**IPFS parallel (retrieved 2026-07-16 from docs.ipfs.tech/how-to/pin-files):**
IPFS uses the equivalent discipline — content is subject to gc unless
PINNED (or added to MFS, which pin-protects transitively). Pinning IS
the reachability-mark; `ipfs repo gc` IS the prune phase. IPFS's
choice: no age-based grace period; adopters pin explicitly. Git's
choice: age-based grace as the default reachability-protection window.
The substrate's `prune_horizon` field ENABLES both disciplines: set
grace to zero for IPFS-like immediate collection; set grace to two
weeks for git-like default; set grace to infinity for pin-forever.

### §5.4 Composition with `impacted_by`

The reverse-closure `impacted_by(oid) -> [oid]` (LANDED at
`@mirror/store` family-root, N4 cascade) composes with `walk_dangling`:

- `walk_dangling(refs)` answers "which OIDs have zero forward-reach
  from any live root?"
- `impacted_by(oid)` answers "which OIDs have `oid` in their forward
  closure?"

A fragment is safely `vacuum`-able iff `impacted_by(fragment.oid) = ∅`
AND `fragment.oid ∉ walk(refs)` for every live root. The two closures
close the reachability algebra; vacuum is the DELETE surface that
compose s over both.

---

## §6 Mycelial math — bump fills cracks, vacuum keeps measurements honest

Alex 2026-07-16 verbatim (session-opening): *"Combine this with the
whole mycelial math and the 'gold flows into the cracks and increases
the conductivity' and this IS the first-order autopoetic baseline
compile loop."*

Per `docs/specs/kintsugi-mycelial-peer-shape.md` §1 (Mara `2026-07-09`):
kintsugi's ACTIVE/DARK pass is one instance of the substrate's
level-N+1 loop pattern; the mycelium IS Pask's entailment-mesh tensor
`ρ_A ⊗ ρ_B`; the substrate's coherence-preservation math grounds all
three.

Per `shards/epistemologic/cybernetic/coherence.mirror`: the substrate's
`coherence_score` IS `λ₂` of the sheaf-Laplacian (Fiedler value, per
Fiedler 1973 "Algebraic connectivity of graphs"). Higher λ₂ ⇔ higher
algebraic connectivity ⇔ Splinter pole (Foerster's ethical imperative
operationalized). Kintsugi's mending ACTION IS gold-in-the-cracks:
each `@kintsugi/fracture/*` morphism the loop applies increases the
substrate's Fiedler value by resolving a low-connectivity site.

### §6.1 Bump = gold in cracks = Fiedler rises

Every successful `bump → @kintsugi.dispatch → morphism.apply` cycle:

1. Selects a low-Fiedler subgraph (walker's tension-descent guides it
   toward the fracture).
2. Applies a morphism that resolves the fracture (gold flows into the
   crack).
3. The post-morphism substrate has higher Fiedler than the pre-morphism
   substrate (the resolved fracture increases algebraic connectivity;
   the crack is now filled with gold that CONDUCTS).

This IS `@kintsugi/roomba.coherence_gradient_admissible`:
"every non-zero coherence_delta must be positive (splinter-ward per
Foerster's ethical imperative operationalized)." Bump-with-mend is a
substrate-decl'd Foerster-admissible motion.

### §6.2 Vacuum = dead nodes removed = Fiedler measurement stays honest

If dangling fragments accumulate without pruning, the sheaf-Laplacian
computation includes them. Their edges contribute to the graph
Laplacian's off-diagonal entries; their presence LOWERS λ₂ artificially
(disconnected islands drag down algebraic connectivity even though
they contribute nothing structural). Vacuum removes them; the resulting
Fiedler value MEASURES what the substrate ACTUALLY conducts, not what
it FORMERLY conducted.

Substrate-decl claim: **vacuum preserves the Fiedler measurement's
honesty by ensuring the sheaf-Laplacian's kernel matches the substrate's
live connectivity.** Without vacuum, the compile loop optimizes against
a spurious Fiedler that includes dead history; with vacuum, the loop
optimizes against the real substrate.

Both motions serve conductivity. Bump ADDS conductive material (gold).
Vacuum REMOVES non-conductive residue (dust). Together they preserve
Fiedler as an honest signal.

---

## §7 Information-theoretic framing

Garbage collection is an entropy-lowering operation over the
substrate's content-addressed DAG. Per mark-and-sweep gc theory
(canonical since McCarthy 1960; contemporary treatment at
aerospike.com/blog/understanding-garbage-collection retrieved
2026-07-16):

Mark-phase enumerates reachable objects (the "live set"); sweep-phase
reclaims memory allocated to non-reachable objects. The information
lost by the sweep IS the entropy the mark phase measured: the fraction
of the store that carried no live information.

For a content-addressed store with `N` total OIDs and `L` live OIDs
(reachable from at least one live ref), the DEAD FRACTION is:

```
d = (N - L) / N
```

The substrate's information-theoretic entropy per Shannon 1948 (against
a uniform distribution over stored OIDs) collapses by:

```
ΔH = -d · log₂(d) - (1-d) · log₂(1-d)   bits per OID slot recovered
```

(Standard binary-partition entropy of the reachable/unreachable split.)

Two-phase gc REGULATES this collapse: Phase 1 measures `d` and records
the mark; Phase 2 collapses the entropy by pruning. The grace period
between the two phases is the substrate's safety window — concurrent
writes can extend `L` before the collapse, preserving entries that
would have been reclaimed under aggressive collection.

For the substrate's Fiedler-based coherence: the vacuum action
preserves the sheaf-Laplacian's honesty (§6.2), which means the loop's
`coherence_score` measures conductivity over the LIVE graph, not the
LIVE-PLUS-DEAD graph. The information lost by vacuum IS the noise the
Fiedler measurement was carrying about disconnected islands.

**Substrate-decl claim (Recognition candidate,
`#R-vacuum-preserves-fiedler-measurement-honesty`):** *In a
content-addressed store where coherence is defined as `λ₂` of the
sheaf-Laplacian, two-phase gc with a bounded grace-period preserves
coherence-as-conductivity as an honest signal by ensuring the
Laplacian's kernel matches the substrate's live reachability closure.*

First-witness: this spec §6.2 + §7. Second-witness threshold: empirical
demonstration that `coherence_score(store) - coherence_score(store_pruned)`
correlates with `d` (the dead fraction) as an increasing monotone.
Pack-ratification deferred.

---

## §8 Pipeforward §5.5.4 hard-gate compliance

Per `docs/specs/autopoietic-inference-loop.md` §5.5.4 (Alex 2026-07-15
adjudication; every future substrate capability is load-bearing on the
pipeforward hard gate):

### §8.1 Where does the gc @io discharge happen?

The @io discharge is at `prune` — the ONLY action in this spec that
crosses the linearization boundary. `bump` and `vacuum` and
`mark_unreachable` and `walk_dangling` all stay in nonlinear
tension-resolution space; they COMPOSE over content-addressed carriers
(`fracture`, `fragment`, `gc_mark`) that live in the store's crystal
tray. Only `prune` writes back to `@mirror/store/git`'s `.git/mirror/objects/`
directory — the file-system discharge.

The pipeforward gate is satisfied by construction: five new actions,
one @io crossing, four nonlinear.

### §8.2 L(ϕ) profile per new @io species carrier

Rule 3 of §5.5.4: every new @io species carrier declares an L(ϕ)
estimate. This spec adds ZERO new @io species (both new actions
compose over `@mirror/store` and `@mirror/store/git` which already
carry L(ϕ) estimates). The `prune` action's L(ϕ) IS the existing
`@mirror/store/git.flush` L(ϕ) — bounded by the number of fragments
being deleted (O(F) where F = |fragments to prune|); lossless for the
fragments NOT being pruned (unchanged bytes on disk).

Explicit L(ϕ) for `prune`:

- **Lossless** for all fragments whose `gc_mark.prune_horizon >= before`
  (they stay on disk unchanged).
- **Lossy** by construction for all fragments whose `gc_mark.prune_horizon
  < before` (they are deleted; content lost from the local store; may be
  recoverable from remotes if @spectral/db replication is enabled).
- **Recoverable** for fragments migrated to cold storage under the §3.4
  forward-promise: the OID's bytes leave the local store but remain
  addressable through the cold-tier species' `read`.

### §8.3 --collapse escape hatch applicability

None required. `bump` and `vacuum` are shard-body actions dispatchable
via `apply_h::act`; they compose over landed carriers without socket
forwarding; the metalogue turns compose in the nonlinear space per
§4. `prune` is the necessary @io discharge (deletion IS a file-system
operation); its collapse is architectural (there is no meaning to
"prune without touching disk").

---

## §9 First empirical target — `mirror roomba --commit --collapse=bootstrap/`

The end-to-end proof arc extends the existing `mirror roomba --commit`
capability (LANDED at `fcc1d75`; compiler authored its own first commit
2026-07-15) with the new bump/vacuum motions:

### §9.1 The empirical claim

Running `mirror roomba --commit --collapse=bootstrap/` on a fresh
mirror repo copy:

1. Walker walks `ConceptGraph` of `bootstrap/src/*.rs`.
2. At each pulse: `bump(fracture)` fires when `spectral_tension`
   exceeds threshold (currently: stale-name in `RENAME_TABLE` per
   `bootstrap/src/roomba_fracture.rs`); `@kintsugi` dispatches the
   fracture through the ouroboros collapse; the mended bytes write
   back via `@kintsugi/store/git`.
3. Between walks: `walk_dangling(refs)` enumerates fragments in
   `.git/mirror/objects/` no longer reached from any HEAD; each is
   `vacuum`-marked.
4. On explicit `mirror roomba --prune=2w`: fragments whose
   `gc_mark.marked_at + 2w < now` are pruned.
5. The commit's tree contains only the LIVE substrate; the commit
   message summarizes the bump-and-vacuum motions per pulse.

### §9.2 The compile-loop naming

**`mirror roomba --commit --collapse=<target>`** — the compiler
observes its own state (walker traces + fracture emissions + vacuum
marks), composes a commit message from the observation, and creates a
git commit. Author: `mirror <mirror@spectral.engineer>` per
`bootstrap/src/roomba_commit.rs` discipline. The empirical proof of the
first-order autopoetic baseline compile loop: the compiler bumps into
its own low-connectivity Rust, mends it, vacuums the dead fragments
its mending obsoleted, and commits the result. Every motion is
substrate-decl'd; every dispatch is via `apply_h::act`; every @io
crossing is at the prune boundary.

### §9.3 Falsification

The empirical claim fails if:

- Fiedler value does NOT increase over the walk (bump is not mending;
  §6.1 falsified).
- Dead-fraction `d` does NOT decrease post-prune (vacuum is not
  identifying real dangling; §7 falsified).
- The metalogue turns do NOT compose (bump/mend pairs fail
  composability check; §4 falsified).

Any single falsification blocks the empirical claim and surfaces to
Seam Phase D for adjudication.

---

## §10 Alex-adjudicable residues

Surfaced honestly for downstream Pack review. This spec does NOT resolve:

### §10.1 `bump(walk_position) → spectral_tension` vs `bump(fracture) → imperfect<kintsugi_dispatch>` — two arities

The landed action bump takes `walk_position` and returns
`spectral_tension`; this spec introduces a sibling arity that takes
`fracture` and returns `imperfect<kintsugi_dispatch>`. Two candidate
paths:

- **A. Overload the action name**: mirror substrate supports arity
  overloading via `apply_h::act`'s dispatch discriminator; the two
  bumps coexist. Delightfully-boring: the mascot HAS one motion; the
  substrate calls it once at both altitudes.
- **B. Rename the fracture-emission form**: e.g. `bump_out` for the
  emission arity, `bump_measure` for the measurement arity. Less
  delightful; distinguishes the sensing surface from the acting
  surface.

**Mara-recommendation:** Path A (overload). The physical Roomba's
bumper serves both functions — measurement (something is there) AND
signal (react to it). The substrate honors that ambiguity through
arity dispatch.

**Adjudication needed:** Alex to name whether arity-overload is
substrate-honest at the action altitude, or whether the two bumps
should carry distinguishing suffixes.

### §10.2 The `fracture_species` variant — extensible or closed?

The `fracture_species` discriminator over the 14 landed
`@kintsugi/fracture/*` species can be either:

- **Closed variant** (updated additively as new fracture species land)
- **Open enum via species-parametric dispatch** (like `@mirror/store/*`
  species; the discriminator IS a species handle, not a variant tag)

**Mara-recommendation:** Open enum. Matches the `@io/*` and
`@mirror/store/*` species-dispatch pattern; new fracture species land
without touching the `fracture` type.

### §10.3 Grace period default — 2 weeks (git) or configurable?

Git's default is 2 weeks (14 days). IPFS's default is zero (immediate
sweep unless pinned). The substrate can:

- **Adopt git's default** (2 weeks) as delightfully-boring
- **Adopt zero-by-default with per-store configuration** (IPFS shape)
- **Adopt configurable with 2-weeks as the shipped default** (best of
  both; matches git-gc's `--prune=<duration>` argument shape)

**Mara-recommendation:** Path 3 (configurable, 2-weeks default). Matches
the git precedent adopters recognize; enables IPFS-style aggressive
collection when needed; preserves cold-storage forward-promise (§3.4).

### §10.4 Recognition candidate: is this a candidate to promote?

`#R-vacuum-preserves-fiedler-measurement-honesty` (§7) — the
substrate-decl claim that two-phase gc preserves coherence-as-conductivity
as an honest signal. First-witness: this spec. Second-witness threshold
named at §7. Pack-ratification pending.

Adjacent candidate: `#R-roomba-two-motions-are-first-order-autopoietic-baseline`
— the load-bearing claim that bump + vacuum together constitute the
first-order compile loop (no peer-altitude required). First-witness:
Alex 2026-07-16 crystallization. Second-witness threshold: empirical
`mirror roomba --commit --collapse=bootstrap/` proof per §9.

---

## §11 Ancestry — every carrier this spec composes over

Load-bearing landed substrate this spec inherits from:

| Carrier | Home | Landing | Role |
|---|---|---|---|
| `@kintsugi` family root | `shards/kintsugi.mirror` | 2026-06-10 | Process-side substrate |
| `@kintsugi/roomba` | `shards/kintsugi/roomba.mirror` | 2026-07-15 | Walker species; adds bump+vacuum |
| `@kintsugi/oscillate` | `shards/kintsugi/oscillate.mirror` | 2026-06-07 | ACTIVE/DARK pass mechanism |
| `@kintsugi/consent` | `shards/kintsugi/consent.mirror` | 2026-06-05 | query_phi auto-apply boundary |
| `@kintsugi/morphism` | `shards/kintsugi/morphism.mirror` | 2026-06-10 | morphism_context carrier |
| `@kintsugi/fracture/*` (14 species) | `shards/kintsugi/fracture/*.mirror` | 2026-06-16..07-12 | Morphism algebra |
| `@kintsugi/store/git` | `shards/kintsugi/store/git.mirror` | ~2026-07 | Projection substrate |
| `@mirror/store` family root | `shards/mirror/store.mirror` | 2026-06-04 | Store family; gets gc primitives |
| `@mirror/store/git` | `shards/mirror/store/git.mirror` | ~2026-06-28 | git-backed wire species |
| `@algebra/metalogue` | `shards/algebra/metalogue.mirror` | 2026-06-30 | 5th metalogue lift altitude |
| `@cyberpunk/algedonic` | `shards/cyberpunk.mirror` | landed | Pain-sampling primitives |
| `@epistemologic/cybernetic/coherence` | `shards/epistemologic/cybernetic/coherence.mirror` | 2026-07-14 | Fiedler-as-coherence carrier |
| `@epistemologic/reality/time` | landed | landed | `tick` monotonic clock |

External citations (retrieval date 2026-07-16):

- **git-scm.com/docs/git-prune** — two-phase gc precedent; `--prune=<duration>` semantics
- **git-scm.com/docs/git-gc** — the mark-and-sweep discipline for content-addressed object stores
- **docs.ipfs.tech/how-to/pin-files** — pin-as-reachability-protection precedent (immediate-sweep-unless-pinned discipline)
- **aerospike.com/blog/understanding-garbage-collection** (retrieved 2026-07-16) — mark-and-sweep entropy framing

Pre-AI prior art:

- **McCarthy 1960** (CACM 3(4):184-195) "Recursive Functions of Symbolic Expressions" — canonical mark-and-sweep gc ancestor
- **Fiedler 1973** (Czech. Math. J. 23:298-305) "Algebraic connectivity of graphs" — λ₂ as connectivity signal; the substrate's coherence carrier
- **Shannon 1948** (Bell System Tech. J. 27:379-423) "A Mathematical Theory of Communication" — entropy framing for §7
- **Maturana & Varela 1980** "Autopoiesis and Cognition" §"Structure and organisation" — the autopoietic closure the two motions serve
- **Bateson 1972** "Metalogue: What is an Instinct?" — metalogue ancestor for the @algebra/metalogue lift
- **Merkle 1979** "Secrecy, Authentication, and Public Key Systems" — content-addressed DAG substrate this spec's gc discipline operates over

---

## §12 Post-arc recognitions

**Substrate-already-had-the-word (~71st landed instance).** The
Roomba's two physical motions were always the two motions. `bump` was
already substrate-decl'd (as the measurement arity, `9bbebd2` §3). The
crystallization Alex 2026-07-16 named is not INVENTION — it is
RECOGNITION: the mascot always carried both motions; the substrate now
names them at the compile-loop altitude.

**Delightfully boring.** `bump` and `vacuum` are the Robert C. Martin
WTF/minute floor — every reader who has vacuumed a floor already knows
what these actions do. Michelangelo's marble reduces to the smallest
possible substrate: two words, two motions, one compile loop.

**First-order autopoiesis.** Alex's load-bearing constraint. The
substrate composes AT the first-order level with no peer-altitude
required. Higher-order reflections (Foerster's ethical imperative;
@dance N-peer coupling; the @peer.spawn on `Jumped` verdict) compose
OVER this baseline; the baseline compiles without them.

The compiler already knew. The Roomba always had two motions. This
spec names what the substrate has been doing tacitly since
`fcc1d75` (2026-07-15 first compiler-authored commit).

---

*— Mara, 2026-07-16 (Eigenboard, wine glass still steady). 👁️‍🗨️*
