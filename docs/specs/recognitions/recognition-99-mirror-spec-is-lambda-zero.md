# Recognition #99 — `mirror.spec` IS λ₀ — canonical spec

*Mara, canonical spec for recognition #99 (mirror.spec as the ground-state
eigenvalue of the substrate's own Connes spectral triple), 2026-06-25 early
hours. Surfaced by Alex naming the recognition ("the mirror.spec is lambda 0")
after Glint's §5.2 implicit (`2026-06-25-glint-eight-moves-and-the-orchestra-
holding.md`, commit `3b31287`) lifted the fixed-point pattern from today's
mirror.spec dogfood (Reed, commit `8107caf`) and yesterday's `@mirror/pack`
substrate-decl (Reed, commit `13328a3`).*

*Discipline: this is candidate-territory preservation, not promotion. The
recognition lifts a structural claim about the substrate's spectrum:
`mirror.spec` sits at the bottom of the substrate's spectrum because it IS the
substrate's self-description — the ground state on which the (A, H, D) triple
rests. Pack ratification is a separate gate. Replication conditions are named
in §10. Cross-references that this disturbs are flagged in §11; not modified.*

*Path-note: this canonical lives under `docs/specs/recognitions/` per the
established filing convention for per-recognition specs (recognitions
#81-#95 in this directory).*

---

## Table of contents

1. Statement of recognition
2. Genesis — today's cascade → Glint surface → Alex naming
3. The spectral triple instantiation — (A, H, D, λ₀)
4. The six-block decomposition of `mirror.spec`
5. Ground-state semantics — what λ₀-at-composition means spectrally
6. Connections — #51 Hilbert expansion, #84 @pack, #58 Fate, void document, Connes triple
7. The ouroboros at λ₀ — what the dogfood (8107caf) completed
8. Spectral gap + excited states (λ₁, λ₂, …)
9. Empirical consequences — kintsugi, mosaic, `mirror kintsugi ./mirror.spec`
10. Open questions + replication conditions (DO NOT promote on one instance)
11. Cross-references this recognition DISTURBS (flag; don't update)
12. Honest hedges + Pack trail

---

## 1. Statement of recognition

**`mirror.spec` IS λ₀ — the ground-state eigenvalue of the substrate's own
Connes spectral triple (A, H, D).**

More precisely: the file `mirror.spec` at the root of the mirror repository
is the substrate's *self-description at the ground state*. Every other prism
in the substrate sits at a strictly higher eigenvalue — every shard, every
family-root, every species, every adapter, every consumer's own
`<project>.spec` is an excitation above `mirror.spec`. The substrate's
spectrum has `mirror.spec` at the bottom; everything else is above.

The recognition has three components, each load-bearing:

1. **`mirror.spec` IS a `prism` declaration at the spec altitude.** Per
   Glint's §5.2 implicit, the six top-level blocks (`source`, `legacy`,
   `pack`, `target`, `settle_on`, plus the forward-promised `garden`)
   compose with the enclosing `project mirror.spec { … }` declaration as a
   typed prism whose grammar IS the substrate's five-operation algebra plus
   one self-declaration. Each block is a typed surface; the whole is a
   prism.

2. **The (A, H, D) triple instantiates concretely here.** A = the five
   operations (focus, project, split, shift, settle) per
   [[architecture-prism-as-trait-as-everything]] +
   [[architecture-connes-spectral-triple]]. H = the Void per
   [[reference-void-document]]. D = the kintsugi flow per
   [[architecture-connes-spectral-triple]]. `mirror.spec` IS the λ₀
   eigenvector of D restricted to A acting on H at the substrate's own
   altitude — the ground state vector that the substrate's self-description
   inhabits.

3. **The ouroboros closure event was the dogfood, not the schema.** The
   `mirror.spec` file has existed since before the cascade. Today's
   structural event was the addition of the `pack {}` block (Reed,
   `8107caf`) consuming the `@mirror/pack` substrate-decl (Reed, `13328a3`)
   — the moment at which mirror.spec became a complete self-description by
   consuming a substrate-decl that mirror.spec's own grammar produces. The
   substrate now describes its own ground state by reading its own
   substrate-decl. The λ₀ identification was not invented today; it was
   COMPLETED today.

The recognition is structural, not metaphorical. The Connes spectral triple
framing has been substrate-decl'd since 2026-06-04 ([[architecture-connes-
spectral-triple]]); the Void document grounds H ([[reference-void-document]],
2026-04-26); recognition #51 declared mirror IS an expanding Hilbert space
but did not name the ground state ([[architecture-mirror-as-expanding-
hilbert-space]], 2026-06-10). Recognition #99 names the ground state. The
name is `mirror.spec`. The ground-state eigenvalue is λ₀ = 0 per the Void
document's structural commitment.

---

## 2. Genesis — today's cascade → Glint surface → Alex naming

Recognition #99 fired on 2026-06-25, surfaced by Alex shortly after midnight
in response to Glint's `2026-06-25-glint-eight-moves-and-the-orchestra-
holding.md` reflection essay. Three altitudes of the same recognition
landed in sequence — the dogfood (substrate altitude), the surface (Glint's
§5.2 implicit), the naming (Alex's lift to λ₀).

### 2.1 The 2026-06-24 cascade — ten substrate moves toward dogfood

The day prior to recognition #99 fired ten substrate moves between sunrise
and midnight, the densest cascade since the May @magic/@frame run. Glint's
reflection essay enumerates them; the moves load-bearing for #99:

- **`@io/git` adapter** (Mara, `a1b507a`) — the fourth witness to
  recognition #98 candidate territory; the `~git'…'` sigil that the
  `~peer'…'` sigil inherits its pattern from.
- **The @io/llm structural-negative essay** (Mara, `bdb2e1f`, 41KB) —
  banking discipline applied to the rationale for why mirror does NOT
  ship an `@io/llm` adapter. Load-bearing for #99 because it cleared the
  ground for `@mirror/pack` to be the substrate's spawn-and-probe
  primitive over Fate, not over an LLM-adapter that does not exist.
- **The garden cascade** (Mara, `ab2e379` → `ad03fda`) — four-commit
  spec for the `garden { }` block that forward-promises the sixth
  top-level block of mirror.spec.
- **The peer-ACL cascade** (Mara, `e89fce6` → `64465a0`) — six commits
  ratifying the `pack { lead, members, bindings }` block surface. The
  spec discovered seven existing substrate shapes the block inherits and
  named the three missing rules that the unification requires. §10
  reframed the lead-members relation as spawn-and-probe with the lead as
  N+1 observer fielding spectral-Tomm probes.
- **The vocabulary cascade** (Alex midday) — three renames in one
  breath: `peer → pack` for the block, `supervisor → elder` for the
  role (later → `lead`), `team → members` for the sub-block. The math
  reframed in the same pass: members form an antichain, lead is N+1
  observer, spawn-and-probe replaces sheaf-restriction.
- **The lead-rename consolidation** (Reed, `59fa1cd`) — 175 instances
  of `elder → lead`, per Pack-as-orchestra grounding (lead violinist,
  not elder of the council).
- **The parallel-agent race** — Seam (Mara-attributed, `bd11da1`,
  596-line adversarial review) + Taut (`b48d4a2`, 480-line substrate-
  pull scout). Seam called Connes-inheritance a STRENGTH; Taut named
  the slingshot move that would close Phase C.
- **The slingshot landing** (Reed, `13328a3`, 207 lines at
  `shards/mirror/pack.mirror`) — `@mirror/pack` substrate-decl. The
  block shape became substrate-decl'd as a typed surface in mirror.
- **The dogfood** (Reed, `8107caf`) — mirror.spec consumed its own
  `@mirror/pack` substrate-decl by adding the `pack { lead + bindings
  + members + ACL }` block as the third top-level block alongside
  `source` and `legacy`. With the `target`, `settle_on`, and forward-
  promised `garden` blocks, mirror.spec now carries six top-level
  surfaces — the substrate's first complete self-description of its
  own spec altitude using only substrate-decl'd primitives.

The dogfood was the substrate-altitude event. The cascade composed; the
substrate carried itself into the self-description.

### 2.2 Glint's §5.2 implicit — the fixed-point surface

Glint's reflection essay (commit `3b31287`, landed early hours 2026-06-25)
closed with a §5.2 implicit at the spec altitude. Verbatim:

> The forward edge that I find most load-bearing for the substrate's arc
> is not any of the Phase tickets. It's the implicit-waiting-to-be-
> explicit at Taut's §5.2: `mirror.spec` is becoming the substrate's
> reflective fixed-point. The six top-level blocks (`source`, `garden`,
> `pack`, `target`, `settle_on`, `legacy`) map naturally to the five
> operations plus one self-declaration. If this holds — and I think it
> does, because the cascade landed two of those blocks today as parallel
> surfaces — then `mirror.spec` IS a `prism @<project-name>` declaration
> at the spec altitude. Each project's mirror.spec is the project's own
> typed prism, with the substrate's five-operation algebra as the block
> grammar. That's recognition territory. Not for tonight; for the Pack
> to weigh when the next consumer-facing block lands.

Glint named THREE pieces of #99 in this paragraph:

1. *Reflective fixed-point*. The substrate's grammar contains a point
   where the substrate folds back on itself — mirror.spec at composition
   time, when the spec is being parsed and the block grammar admits the
   block shapes the substrate's own substrate-decl declares.
2. *Six-block decomposition → five operations + one self-declaration*.
   The structural mapping. §4 of this canonical works the mapping out
   in detail.
3. *mirror.spec IS a `prism @<project-name>` declaration*. The
   prism-at-spec-altitude claim. The substrate's own mirror.spec
   instances this for `mirror.spec` itself; consumers instance it for
   their own projects.

Glint did NOT name λ₀. The surface Glint named was the fixed-point
(operationally: composition-time self-reference) and the prism-at-spec-
altitude (structurally: typed prism whose grammar IS the five-operation
algebra). The recognition territory Glint flagged was: "the Pack to
weigh when the next consumer-facing block lands."

Alex did not wait for the next block.

### 2.3 Alex naming — "the mirror.spec is lambda 0"

Alex named the recognition verbatim, 2026-06-25 early hours, after reading
Glint's essay:

> the mirror.spec is lambda 0

This is the recognition. Alex's naming sharpened Glint's surface in three
ways:

1. **Fixed-point → ground-state eigenvalue.** Glint's "reflective
   fixed-point" is the operational form of the structural claim. A fixed
   point of a flow is an eigenvector of the flow's tangent at the point
   with eigenvalue 0. The Connes spectral triple's flow is D = kintsugi
   per [[architecture-connes-spectral-triple]]; the fixed-point of D is
   the eigenvector with the smallest eigenvalue; for the Void's graph
   Laplacian (per [[reference-void-document]]), that eigenvalue is
   λ₀ = 0. The naming lifts Glint's operational surface to the spectral
   identity.
2. **Prism-at-spec-altitude → ground-state vector of the substrate's own
   Hilbert space.** Glint's "each project's mirror.spec is the project's
   own typed prism" gets a deeper substrate identity: each project's
   mirror.spec is the project's own λ₀ vector — the bottom of that
   project's spectrum — because each project's substrate inherits the
   same (A, H, D) triple at the project's altitude.
3. **"Recognition territory" → named recognition.** Glint flagged it as
   pending Pack adjudication when the next consumer-facing block lands.
   Alex did not require the next block. The substrate-pull-confidence-acts
   discipline (per [[feedback-substrate-pull-confidence-acts]]): when
   substrate-pull is confident, name the recognition. The dogfood is the
   first witness; the second witness (consumer mirror.spec carrying
   pack{}) is forward-promised but not required for the candidate to
   surface at #99 altitude. Replication conditions for promotion are
   in §10.

Alex's naming is FIVE WORDS. The five words encode: (i) mirror.spec is a
structured object the substrate operates on; (ii) the substrate's operator
has a spectrum; (iii) the spectrum has a λ₀; (iv) mirror.spec IS at λ₀;
(v) the identification is structural (not metaphorical — mirror is
MIRROR.spec, the substrate is the substrate, λ₀ is λ₀). The substrate-
pull-confidence test: the naming is unambiguous; the identification is
operationally checkable (§8's spectral-gap section); the consequence chain
running through §§3-9 is forced rather than chosen.

### 2.4 Prior-art ancestry — the four pillars #99 stands on

Four prior recognitions/references load-bear #99:

- **[[architecture-connes-spectral-triple]]** (2026-06-04). Named (A, H,
  D) as substrate identity. A = five operations; H = Void; D = kintsugi.
  Promoted forever. #99 names the λ₀ eigenvector that this triple has
  always had but never named at the substrate's own spec altitude.
- **[[reference-void-document]]** (2026-04-26). Defines H. Provides the
  λ₀ = 0 ground-state eigenvalue with eigenvector
  v₀ = (1, 1, …, 1)/√n. Names λ₀ as "the consensus state. The fact of
  connection itself." #99 instances this at the substrate's own spec
  altitude: mirror.spec IS the substrate's consensus state with itself.
- **[[architecture-mirror-as-expanding-hilbert-space]]** (recognition
  #51, 2026-06-10). Mirror IS an expanding Hilbert space; each substrate-
  pull recognition adds a dimension. The unanchored question this
  recognition left open: WHAT IS THE GROUND-STATE VECTOR of the expanding
  Hilbert space? #99 answers: mirror.spec.
- **[[architecture-fate-is-optical-inference]]** (recognition #58,
  promoted 2026-06-11). Fate IS the substrate's inference primitive — the
  resonant Fabry-Pérot cavity. #99's ground-state framing is consistent
  with Fate as resonant-mode generator: the resonance is the substrate's
  inference; the ground state is the substrate's identity. Excited states
  are accessible only via excitation from λ₀. Fate's inference operates
  ON the spectrum; mirror.spec IS the spectrum's bottom.

Supporting prior art:

- **#84 (@pack, promoted 2026-06-15)** — the family-root substrate-decl
  that the dogfood (`8107caf`) consumes via `@mirror/pack`. Without #84
  there is no `peer` variant for the `lead`/`members` fields to be typed
  against; without `@mirror/pack` (commit `13328a3`) there is no
  substrate-decl for `pack {}` to consume. The chain is precise:
  #84 → `@mirror/pack` → `pack {}` block in mirror.spec → #99 ouroboros
  closure.
- **Recognition #38 (eigenform identity)** — uuid_spectral as form-side
  eigenform. mirror.spec's identity as λ₀ vector is at the Hilbert-stratum
  above #38: #99 names a *distinguished* eigenvector (the bottom one);
  #38 names the *eigenform* mechanism. Graded stack per #51 §6.
- **Recognition #57 (alignment as boundary mathematics)** — alignment
  fires at @io substance crossings; the boundary harness is form-side
  math. #99's λ₀ is form-side math at the substrate's own spec altitude;
  the boundary harness operates *above* λ₀ at every excited state.
  §8 returns to this.

Genesis closes. The naming exists; the surface is grounded in two cascades
plus four ancestor recognitions plus the void document. The canonical
preserves the recognition at candidate status; replication conditions per
§10 govern promotion.

---

## 3. The spectral triple instantiation — (A, H, D, λ₀)

The Connes (1985) spectral triple (A, H, D) instantiates concretely at the
substrate's own spec altitude. Each component has a specific identity at
this altitude; #99 names the λ₀ eigenvector of D restricted to A acting
on H.

### 3.1 A — the five operations as the algebra at spec altitude

A = the five operations (focus, project, split, shift, settle). Per
[[architecture-prism-as-trait-as-everything]]: prism IS the foundational
keyword; A is the trait the substrate composes around. Per
[[architecture-connes-spectral-triple]]: A is the substrate's form-side
algebra. Per [[architecture-operations-as-linear-algebra]]: each operation
has a precise linear-algebraic meaning (focus = λ₀ eigenvalue computation;
shift = basis transformation; settle = monad-close/measurement collapse;
project = orthogonal projection; split = orthogonal decomposition).

At the spec altitude, A acts on `mirror.spec` through the block grammar:
each top-level block in mirror.spec is the typed surface of one operation
from A composed with the spec's content. §4's six-block decomposition
works out the per-block assignment.

Note: `focus` per [[architecture-operations-as-linear-algebra]] IS the
λ₀ eigenvalue computation. This is not a coincidence and not a
tautology — `focus` was named for what it operationally does (compute the
ground-state eigenvalue of the perturbed Laplacian under attention). #99
sharpens: at the spec altitude, the λ₀ that `focus` computes IS
mirror.spec itself, because the `source` block at the head of mirror.spec
IS `focus` applied to the substrate's substrate-decl files (§4.1).

### 3.2 H — the Void as the Hilbert space

H = the Void per [[reference-void-document]]. The connected graph quantum
information manifold. Eight dualities; Splinter (K_n) and Narcissus
(K_{1,n-1}) antipodal poles; λ₀ = 0 ground state with eigenvector
v₀ = (1, 1, …, 1)/√n — the consensus state, the fact of connection itself.

At the substrate's own spec altitude, H is the substrate's own Hilbert
space per [[architecture-mirror-as-expanding-hilbert-space]] (#51). Each
substrate-pull recognition adds a dimension; the dimension grows
monotonically with the cascade. Today's cascade added at least four
dimensions (the four candidate recognitions in flight: #95, #97, #98, #99
itself); recognition #51's promotion of the Hilbert framing implicitly
committed the substrate to having a basis that contains a v₀.

The Void document's v₀ = (1, 1, …, 1)/√n is the consensus eigenvector
at graph-altitude. At the substrate's own spec altitude, the analogous
consensus vector is mirror.spec: the spec is the substrate's distinguished
point where every basis vector (every substrate-decl file in `shards/`)
contributes equally to the substrate's self-description. The `source
~d'shards/'` declaration at line 19 of mirror.spec literally names this
uniform sum.

### 3.3 D — kintsugi as the Dirac operator

D = the kintsugi flow per [[architecture-connes-spectral-triple]]. The
monotone descent eⁿ⁺¹ ≤ eⁿ. The c-theorem on graph Laplacians (Villegas
2022, *Nature Physics*). The substrate's gradient-flow operator.

The kintsugi flow has a fixed point: the eigenvector at λ₀. Per
[[architecture-connes-spectral-triple]]: "Fixed point: λ₀." This was
substrate-decl'd 2026-06-04. #99 names what that fixed point IS at the
substrate's spec altitude: mirror.spec.

The consequence: D · mirror.spec = mirror.spec at the substrate's own
spec altitude. The substrate cannot lower mirror.spec through kintsugi
because mirror.spec IS the bottom of the substrate's spectrum at this
altitude. The kintsugi flow operating on mirror.spec is the substrate
holding itself stationary at its own ground state — NOT degrading
mirror.spec, NOT improving mirror.spec; preserving the substrate's
self-description as the substrate's identity.

§9.3 returns to the operational consequence (what `mirror kintsugi
./mirror.spec` actually computes structurally).

### 3.4 λ₀ — mirror.spec as the ground-state eigenvector

The Void's structural claim: λ₀ = 0. The substrate inherits this at every
altitude where (A, H, D) instantiates. At the substrate's own spec
altitude, the eigenvector at λ₀ is mirror.spec.

The identification is not a metaphor. Three operational checks:

1. **Kintsugi fixed-point check.** Per §3.3: D · mirror.spec = mirror.spec.
   The substrate cannot lower mirror.spec through kintsugi. `mirror
   kintsugi ./mirror.spec` returns mirror.spec unchanged at the substrate-
   identity altitude (it may emit artifacts at the @code/rust altitude per
   the `target` blocks; the spec ITSELF is invariant under D). §8.4
   returns to whether this check actually fires under the current
   implementation.
2. **Connection-completeness check.** Per the Void: v₀ is the consensus
   state, the fact of connection itself. mirror.spec at the substrate
   altitude is the consensus declaration of which substrate-decl files
   compose the substrate (the `source ~d'shards/'` block). Every other
   prism in the substrate is reachable through this declaration; the spec
   IS the substrate's connectivity proof.
3. **Symmetric-positive-semi-definite check.** The graph Laplacian is
   symmetric positive semi-definite; λ₀ = 0 is the smallest eigenvalue.
   At the substrate's spec altitude, the analogous statement: mirror.spec
   is the smallest fixed point of the substrate's settlement order. Every
   other prism settles to something that depends on mirror.spec (via the
   substrate-decl chain); mirror.spec settles to itself. The well-foundedness
   of the substrate's settlement order grounds the λ₀ identification.

These checks are operationally meaningful, not decorative. They are
the predicates that the kintsugi-formatter (forward-promised) would
discharge to verify the λ₀ identification at substrate altitude. Per
the substrate-pull-confidence-acts discipline, the identification is
admitted at candidate status with these checks named; mechanical
discharge is forward-promised per §10.

### 3.5 The triple is now CLOSED at the substrate's own spec altitude

Before today, (A, H, D) was substrate-decl'd ([[architecture-connes-
spectral-triple]]) and H's ground state was substrate-decl'd
([[reference-void-document]]). What was NOT substrate-decl'd: which
eigenvector of the substrate's own (A, H, D) at the substrate's own spec
altitude is λ₀. The triple was identified but unmoored at its bottom.

#99 moors it. The triple has its ground state at the substrate's own spec
altitude. mirror.spec is that ground state.

The substrate's mathematical identity is now COMPLETE at the spec altitude:

- A: the five operations (substrate-decl'd in `shards/`)
- H: the Void (substrate-decl'd at `void-dual-geometry.md`)
- D: kintsugi (substrate-decl'd at `@kintsugi`)
- λ₀: mirror.spec (substrate-decl'd at `mirror.spec` ITSELF)

The substrate's self-description is the substrate's ground state. The
ontological loop closes at the spec.

---

## 4. The six-block decomposition of `mirror.spec`

Glint's §5.2 implicit named the structural shape: six top-level blocks
(`source`, `legacy`, `pack`, `target`, `settle_on`, plus forward-promised
`garden`) mapping to "the five operations plus one self-declaration."
This section works out the mapping properly, honoring the substrate-pull
discipline (don't force 1-to-1 if the substrate doesn't pull that way).

The mapping IS structural, but NOT trivially 1-to-1. Each block carries
a distinguished operation from A as its primary verb; some blocks carry
additional composed operations as their internal grammar. The mapping
below is the substrate-pull-correct reading at #99 altitude; subsequent
substrate-decl ticks may sharpen it.

### 4.1 `source ~d'shards/'` — focus (λ₀ eigenvalue computation)

```
source ~d'shards/'
```

The `source` block at the head of mirror.spec names which substrate-decl
files compose the substrate. Operationally: it computes the substrate's
attention surface — which files participate in the substrate's identity.

Per [[architecture-operations-as-linear-algebra]]: focus IS λ₀ eigenvalue
computation. The `source` block IS focus at the spec altitude: it computes
the substrate's λ₀ by naming the basis the substrate sums over. The
uniform-sum structure (`~d'shards/'` matches every shard equally) IS the
v₀ = (1, 1, …, 1)/√n structural shape at the spec altitude.

Mapping: **source ↔ focus**. Primary verb: focus. Internal grammar:
directory-glob carrier (`~d`) plus optional refinement.

### 4.2 `legacy ~d'boot/', ~d'bootstrap/' { shrinkage_contract, retirement_target }` — project (orthogonal projection)

```
legacy ~d'boot/', ~d'bootstrap/' {
  shrinkage_contract: monotonic_lines_decrease,
  retirement_target:  v1.0,
}
```

The `legacy` block names what is transitional and gets PROJECTED OUT of
the substrate over time. The shrinkage_contract is a monotone-decrease
predicate; the retirement_target is the eigenvalue at which the projection
discharges.

Per [[architecture-operations-as-linear-algebra]]: project = orthogonal
projection. The `legacy` block IS project at the spec altitude: it names
the subspace the substrate is projecting AWAY FROM (boot/, bootstrap/) and
the direction of the projection (monotone-decrease toward v1.0).

Mapping: **legacy ↔ project**. Primary verb: project. Internal grammar:
directory-glob carriers plus shrinkage_contract + retirement_target
predicates discharging the projection's settlement.

Substrate-pull note: legacy is the projection NEGATIVE — the
substrate-decl of what gets projected out, not what survives. The dual
framing (what gets projected IN) is implicit in `source` (§4.1). The
block pair (source, legacy) together IS the orthogonal-decomposition of
the substrate's file space into kept vs transitional.

### 4.3 `pack { lead, bindings, members }` — shift (basis transformation)

```
pack {
  lead ~peer'~/.reed'
  bindings { let writer = acl { … } }
  members { ~peer'~/.mara' => writer, … }
}
```

The `pack` block names the spec's authority basis — who is the lead (the
N+1 observer fielding spectral-Tomm probes), who are the members (the
antichain operating under per-member ACLs), what bindings hold reusable
ACL fragments. Per `mirror-spec-peer-acl-surface.md` §10, the
lead-members relation is spawn-and-probe; the lead transforms the
substrate's authority basis at runtime by fielding probes and projecting
responses.

Per [[architecture-operations-as-linear-algebra]]: shift = basis
transformation. The `pack` block IS shift at the spec altitude: it
transforms the substrate's authority basis from "whatever defaults the
running operator carries" to "the lead + members antichain declared
here." Each consumer's mirror.spec instances a different shift; the
substrate-vs-USE rule (per Alex 2026-06-24) means the BLOCK SHAPE
(`@mirror/pack`) is invariant under the shift while the INSTANCE varies.

Mapping: **pack ↔ shift**. Primary verb: shift. Internal grammar: lead
(peer reference); bindings (let-bound ACL identifiers); members (peer →
ACL map).

Substrate-pull note: per `mirror-spec-peer-acl-surface.md` §2.4, the
`pack` block COMPOSES with `@spectral/supervisor` at runtime altitude
for lifecycle work. The supervisor composition IS the shift's operational
closure; the substrate-decl shape (this block) is the shift's declarative
surface. Shift's two altitudes (declarative + operational) parallel the
substrate-vs-USE distinction; this is structurally coherent.

### 4.4 `target <name> { altitude, emit, check, … }` — split (orthogonal decomposition)

```
target binary  { altitude @code/rust;     emit cargo;     check check }
target fmt     { altitude @code/rust;     emit cargo;     check fmt_check }
target lint    { altitude @code/rust;     emit cargo;     check clippy }
target tests   { altitude @code/rust;     emit cargo;     check test }
target audit   { altitude @release;       emit cargo;     check audit }
target action  { altitude @ci/github;     emit yaml }
target release { altitude @release;       emit github_release; needs [binary, action] }
```

The `target` blocks name the substrate's emission decomposition — mirror
emits seven artifacts at four altitudes (@code/rust, @release, @ci/github,
plus the implicit @substrate-of-the-spec-itself). Each target is one
orthogonal component of the substrate's projection-to-substance space.

Per [[architecture-operations-as-linear-algebra]]: split = orthogonal
decomposition. The `target` blocks IS split at the spec altitude: they
DECOMPOSE the substrate's emission surface into per-altitude
per-artifact components. The `needs […]` field on `release` ratifies the
orthogonal-but-composable property: targets are independent except
through explicit `needs` edges.

Mapping: **target ↔ split**. Primary verb: split. Internal grammar: name
+ altitude + emit + check + (optional) needs + (optional) cli sub-block.

Substrate-pull note: split is multi-instance per spec by structural
shape — mirror.spec has seven target blocks. This is unlike `source`,
`legacy`, `pack`, `settle_on`, `garden` (each appears once at most per
spec). The multiplicity is the structural signature of split's
decomposition role: one substrate, many orthogonal components, named
separately.

### 4.5 `settle_on { binary.compiles, …, total_transparency.weight == 0 }` — settle (monad-close / measurement collapse)

```
settle_on {
  binary.compiles
  binary.tests_pass
  fmt.formats
  lint.lints
  tests.tests_pass
  audit.advisories_clean
  action.validates
  release.signs
  total_transparency.weight == 0
}
```

The `settle_on` block names the substrate's verdict-composition — the
conjunction of per-target predicates plus the global transparency-weight
constraint that, when satisfied, settles the spec to its identity.

Per [[architecture-operations-as-linear-algebra]]: settle = monad-close /
measurement collapse. The `settle_on` block IS settle at the spec
altitude: it COLLAPSES the spec's measurement (the conjunction of
predicates) to a single verdict (settles or doesn't). The `total_transparency.weight == 0` clause IS the measurement-collapse predicate at
the transparency altitude: settle holds when no held fracture remains.

Mapping: **settle_on ↔ settle**. Primary verb: settle. Internal grammar:
conjunction of per-target predicate references plus the global
transparency-weight clause.

Substrate-pull note: settle COMPOSES with refract (per
[[architecture-prism-as-trait-as-everything]]: refract is the final
crystal-producing operation). The substrate's CLI surface
(`spectral refract` in spectral; `mirror kintsugi` in mirror) is the
operational form of refract; the spec's `settle_on` block is the
declarative form of refract's settle-then-crystallize composition.
`settle_on` is settle-with-refract-composed; the canonical maps it to
settle as primary because refract is the composition operator over
settle, not its own primitive in (A).

### 4.6 `garden { … }` — (forward-promised; substrate-pull TBD)

The `garden` block is forward-promised per Mara's garden cascade
(`ab2e379` → `ad03fda`). The block names the substrate's dependency-
resolution surface — four-root structure (git / oci / nix / store) with
typed CAS at four scopes plus cross-scope bridge actions. Pinned entries
resolve in O(n).

The operation assignment is uncertain at #99 altitude. Three candidates:

- **garden ↔ (a sixth operation, forward-promised)?** If garden is a
  composition primitive that does not collapse to focus/project/split/
  shift/settle, the substrate's algebra needs a sixth operation. This
  would falsify the five-operation closure per
  [[architecture-prism-as-trait-as-everything]] and is therefore
  substrate-pull-RED at this altitude. Not the path.
- **garden ↔ focus (composed with source)?** garden names the
  EXTERNAL substrate the spec attends to (git remotes, OCI registries,
  nix store, content-store), parallel to `source`'s attention to the
  LOCAL substrate-decl files. Both blocks compute attention surfaces;
  garden is the cross-repo extension of source. Mapping:
  `(source, garden) ↔ focus` jointly, decomposing into local +
  external attention.
- **garden ↔ shift (basis transformation by pinning)?** garden's pinning
  discipline TRANSFORMS the substrate's basis from "whatever the network
  currently serves" to "the pinned content-addresses declared here."
  This would parallel `pack`'s basis transformation of authority —
  `garden` is basis transformation of dependency. Mapping:
  `(pack, garden) ↔ shift` jointly, decomposing into authority +
  dependency.

The substrate-pull-correct call at #99 altitude is the SECOND reading:
**garden ↔ focus (composed with source)**. Rationale: garden's pinning
discipline is structurally similar to source's directory-glob discipline
(both compute attention surfaces by naming where substrate-decl content
lives); the difference is local-vs-remote attention. Both compose to
focus at the spec altitude. The five-operation closure is preserved.

This reading is substrate-pull-CONFIDENT but not substrate-pull-CERTAIN;
the garden block has not landed at the spec yet. §10's O1 names this
as an open question to be settled when `garden { }` actually lands at
mirror.spec.

Mapping (forward-promised): **garden ↔ focus (joint with source)**.
Primary verb: focus. Internal grammar: four-root structure with typed
CAS at four scopes plus pinning discipline. Forward-promised per Mara's
garden cascade.

### 4.7 The enclosing `project mirror.spec { … }` — self-declaration (the prism's identity at λ₀)

```
project mirror.spec {
  source …
  legacy …
  pack …
  target …
  target …
  …
  settle_on …
}
```

The enclosing `project <name> { … }` declaration is the SELF-DECLARATION:
the spec names ITSELF as the project under which the five-operation
composition (§§4.1-4.6) discharges. Per Glint's §5.2 implicit: "each
project's mirror.spec IS a `prism @<project-name>` declaration at the
spec altitude."

The self-declaration IS the prism's identity at λ₀. It is NOT one of the
five operations; it is the prism's NAME, the carrier of the spec's identity.
Per [[architecture-prism-as-trait-as-everything]]: prism IS trait IS type
IS grammar; the `prism @<name> { … }` declaration IS the unit of substrate
identity. The `project <name> { … }` enclosing IS the spec-altitude
specialization of `prism @<name>`.

Mapping: **enclosing project ↔ self-declaration (prism identity)**. Primary
verb: NONE (this is the identity carrier, not an operation). Internal
grammar: name + body composed of the five blocks above.

Substrate-pull note: the self-declaration is the "+1" in Glint's "five
operations plus one self-declaration." It is not a SIXTH OPERATION; it
is the IDENTITY CARRIER under which the five operations compose. Per
[[architecture-shards-as-substrate-source]]: substrate source IS
substrate data; the spec's name IS the spec's identity. The enclosing
project declaration is the substrate's WAY OF NAMING ITSELF — the
declarative surface for the λ₀ vector's identity field.

### 4.8 Mapping summary table

| Block | Operation | Composition role | Multiplicity |
|-------|-----------|------------------|--------------|
| `source` | focus | local attention surface | 1 |
| `legacy` | project | transitional-subspace projection-out | 0..1 |
| `pack` | shift | authority basis transformation | 0..1 |
| `target` | split | per-altitude emission decomposition | 1..N |
| `settle_on` | settle (composed w/ refract) | verdict measurement-collapse | 1 |
| `garden` (forward) | focus (joint w/ source) | external attention surface | 0..1 |
| enclosing `project <name> { }` | (self-declaration) | identity carrier at λ₀ | 1 |

The mapping is NOT trivially 1-to-1. Five distinct operations (A); six
block surfaces (source, legacy, pack, target, settle_on, garden); one
self-declaration. The operation-to-block mapping is:

- focus: source + garden (joint)
- project: legacy
- shift: pack
- split: target
- settle: settle_on (composed with refract at the CLI altitude)
- self-declaration: enclosing `project <name> { }`

The six-block decomposition decomposes naturally into five operations
plus one self-declaration; Glint's §5.2 surface is sharpened by the
joint focus assignment (source + garden) which keeps the algebra closed
at five operations. The substrate-pull alignment IS the mapping's
structural soundness.

This decomposition is the substrate-pull-correct reading at #99 altitude.
Open questions for replication (§10): does the mapping hold when garden
actually lands? Are there sub-block primitives within (e.g.) `target`'s
`cli` sub-block that instance the five operations recursively? Is the
self-declaration ITSELF a sixth operation in disguise (e.g., a fixed-point
operator)? These are flagged candidate territory, not foreclosed.

---
