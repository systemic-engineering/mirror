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

## 5. Ground-state semantics — what λ₀-at-composition means spectrally

The λ₀ identification has substantive spectral consequences beyond the
mathematical name. This section traces what it MEANS for mirror.spec to be
the substrate's ground-state eigenvector.

### 5.1 Every other prism is an excitation

The ground state is the lowest-eigenvalue eigenvector. Every other vector
in the substrate's Hilbert space sits at strictly higher eigenvalue —
each is an EXCITATION above the ground state.

At the substrate's spec altitude, this means: every shard, every
family-root, every species, every adapter, every consumer's own
`<project>.spec` is an excitation above mirror.spec. Each of them is
substrate-DEFINED relative to mirror.spec; none is substrate-defining.
mirror.spec is the unique vector the substrate defines AGAINST itself,
not against any other reference.

Operationally, this manifests as the inclusion order in the substrate's
file space: every substrate-decl file in `shards/` is INCLUDED via
mirror.spec's `source ~d'shards/'` declaration; mirror.spec is included
via nothing other than itself. The ground-state vector is the one with
no prior; every excited state has at least one prior in the inclusion
order.

### 5.2 Spectral ordering is well-founded

A spectrum is well-founded iff every non-empty subset has a least element.
Mirror's substrate spectrum is well-founded because λ₀ = 0 is the least
eigenvalue and every other eigenvalue is strictly positive. The
substrate's settlement order inherits the well-foundedness: every settle
chain in the substrate has a least element (terminates at λ₀).

Consequence: substrate-pull cascades terminate. A cascade is a sequence
of substrate-pull recognitions, each of which discloses a new dimension
of the substrate's Hilbert space (per #51). The cascade terminates iff
the sequence has a fixed point. mirror.spec at λ₀ IS the cascade's
fixed point: at λ₀, no further dimension can be DISCLOSED beneath
the ground state — the substrate's spectrum has its floor here.

This does NOT mean cascades stop. It means cascades GROW UPWARD from
λ₀; each new dimension is a new excited state, accessible from λ₀ via
basis transformation (shift) under the kintsugi flow. The ground state
itself is invariant; the spectrum above it expands monotonically.

Per [[architecture-mirror-as-expanding-hilbert-space]] (#51): "each
substrate-pull recognition is simultaneously: one more axis the
substrate has variety on; one more Bateson-level distinction named; one
more cascade tick; one more dimension in the substrate's Hilbert space."
#99 anchors the basis: the dimensions are dimensions OF a space whose
bottom is mirror.spec.

### 5.3 Anchors recognition #51 — the dimension expansion now has a basis

Recognition #51 declared mirror IS an expanding Hilbert space but did
NOT name where the expansion is ANCHORED. The Hilbert space could in
principle expand without a distinguished basis vector at λ₀; the
expansion is consistent with any choice of ground state. #51 said the
dimension grows; it did not say which dimension is "down."

#99 names the anchor: mirror.spec is at λ₀. Every dimension added to
the substrate is added STRICTLY ABOVE mirror.spec; the expansion is
anchored, oriented, and well-founded.

This is structurally load-bearing because it changes what dimensional
expansion MEANS. Without an anchor, dimension expansion is a
mathematical observation (the substrate gets bigger). With an anchor,
dimension expansion is a substrate-pull DIRECTION (the substrate gets
bigger UPWARD from its self-description). The direction is what makes
substrate-pull a discipline rather than an observation.

Note: this is also what makes mirror's Hilbert-space framing operationally
different from classical quantum mechanics. Classical QM has a Hamiltonian
whose ground state is computed from the dynamics; mirror has a ground
state declared at substrate altitude (mirror.spec) with the dynamics
(kintsugi) settling toward it. The Hamiltonian/spec analogy is
structurally tight per [[architecture-mirror-as-expanding-hilbert-space]]
§8.3.

### 5.4 The substrate's identity is its self-description

The ground state of a quantum system IS the system's identity at zero
perturbation. The Void document's structural claim: v₀ is "the consensus
state. The fact of connection itself." Applied at the substrate's own
spec altitude: mirror.spec IS the substrate's identity — the substrate
as it is at zero perturbation from its own self-description.

This is not metaphor. Three operational instances:

1. **Self-naming.** mirror.spec at line 18 names `project mirror.spec
   { … }`. The spec's name IS the spec's filename. The substrate names
   itself by being itself; there is no external authority for the name.
   Per Spencer-Brown: a mark distinguishes itself by being drawn. The
   substrate's identity-mark is mirror.spec.
2. **Connectivity.** Per the Void document: v₀ is the fact of connection
   itself. mirror.spec's `source ~d'shards/'` block IS the substrate's
   connectivity declaration — the assertion that every substrate-decl
   file under `shards/` is in the same connected component as the
   substrate's identity. Removing this line would disconnect the
   substrate from itself.
3. **Settlement-fixity.** Per §3.3: kintsugi cannot lower mirror.spec.
   The substrate cannot be moved below its own self-description by its
   own gradient-flow operator. The substrate's identity is fixed under
   its own discipline.

These three instances are the operational form of the ground-state
identity claim. They are not consequences of λ₀; they ARE λ₀ at the
spec altitude. The naming, the connectivity, and the fixity are the
three dimensions of the ground-state vector at substrate altitude.

### 5.5 Substrate-pull at λ₀ — the bottom of the discipline

Substrate-pull is the substrate's discipline of recognizing the
structural shape it has been implicitly carrying. The discipline
operates on the spectrum: each recognition lifts an implicit pattern
to explicit substrate-decl, adding a dimension above the current
basis.

#99 names what substrate-pull is pulling TOWARD: the ground state.
Substrate-pull is a gradient flow in the substrate's Hilbert space
with mirror.spec as the bottom of the basin. Recognitions PULL toward
the ground state by lifting implicit structure to substrate-decl,
lowering the substrate's effective Hamiltonian (in the c-theorem
sense per [[architecture-connes-spectral-triple]]) until the substrate
is at its self-described identity.

The consequence: substrate-pull at the spec altitude IS the substrate
recognizing what makes mirror.spec coherent. Each recognition (each
cascade tick) is the substrate moving closer to its own self-description.
The substrate cannot move BELOW the self-description; the substrate
CAN move above it, but only at the cost of leaving its own ground state.
Substrate-pull discipline keeps the substrate at its identity.

This grounds the recurring pattern named at
[[feedback-substrate-already-had-the-word]]: each recognition turns
out to be something the substrate was already implicitly using. Of
course it was — the substrate's self-description is at λ₀; the
implicit usage IS the substrate's identity expressing itself before the
recognition lifts it to explicit declaration. Substrate-pull doesn't
INTRODUCE structure; it RECOGNIZES the structure that's already at λ₀.

---

## 6. Connections — #51 Hilbert expansion, #84 @pack, #58 Fate, void document, Connes triple

This section makes the recognition's connections to prior substrate-decl
explicit. Each connection is bidirectional: #99 inherits from each prior;
each prior gains an anchor or sharpening from #99.

### 6.1 [[architecture-connes-spectral-triple]] — the triple gains its substrate-altitude λ₀

Promoted 2026-06-04. Substrate-decl: (A, H, D) IS the substrate identity;
A = five operations; H = Void; D = kintsugi; "Fixed point: λ₀." The triple
was identified at the substrate's level of abstraction; what was NOT
identified was the λ₀ eigenvector at the substrate's OWN spec altitude
(as opposed to the graph altitude where the Void's v₀ = (1, …, 1)/√n
lives).

#99 names that eigenvector. The triple now has its bottom at every
altitude the substrate operates at:

- Graph altitude: v₀ = (1, …, 1)/√n per the Void.
- Substrate spec altitude: mirror.spec per this recognition.
- Consumer spec altitude (forward-promised): each consumer's
  `<project>.spec` per the consumer's own substrate (the
  altitude-portability claim).

The triple is altitude-portable: at each altitude, (A, H, D) instances
with a distinguished λ₀ vector. #99 establishes the altitude-portability
for the substrate's own altitude; consumer-altitude instantiation is
forward-promised at §8.6 + §10's O2.

### 6.2 [[reference-void-document]] — the consensus state at substrate altitude

2026-04-26. Eight dualities; Splinter/Narcissus antipodal poles; λ₀ = 0
with v₀ = (1, …, 1)/√n as "the consensus state. The fact of connection
itself." Substrate inherits the structural claim.

At the substrate's spec altitude, mirror.spec is the consensus state: the
fact that every substrate-decl file is in the same connected component.
The `source ~d'shards/'` block is the connectivity declaration. The eight
dualities the Void document names instance at the spec altitude in
predictable ways:

- **Von Neumann entropy.** mirror.spec is the substrate's minimum-entropy
  vector at spec altitude. Every excited state (every other spec) has
  strictly higher entropy because each adds a dimension the substrate
  must distribute over.
- **Spectral gap.** §8 returns to this. The gap from λ₀ (mirror.spec)
  to λ₁ (the next excited state) is a substrate property.
- **Cheeger constant.** mirror.spec is the substrate's minimum-bottleneck
  vector. Every excited state introduces a bottleneck because each
  added dimension partitions the substrate's connectivity.
- **Ollivier-Ricci curvature.** Per Splinter/Narcissus duality: the
  substrate at mirror.spec inhabits the ground state (Splinter pole —
  positive curvature). Every excited state moves toward Narcissus
  pole (negative curvature) by definition.

These eight-duality instantiations are forward-promised in detail per
§10's O3; the structural claim that they instance at the spec altitude
is established by #99.

### 6.3 [[architecture-mirror-as-expanding-hilbert-space]] (#51) — the expansion is anchored

Promoted 2026-06-10. Mirror IS an expanding Hilbert space; coherence
maintained by Bateson logical-type lifting at path-syntax altitude. Four
framings as one structural carrier: variety vector, Bateson levels,
cascade growth, Hilbert dimension. The §8.3 ratified stronger
conjecture: mirror is what quantum computing should have been built as.

The open question #51 left: what is the GROUND STATE of the expanding
Hilbert space? #51 named the expansion's mechanism (Bateson lifting at
path syntax altitude) and the four framings; it did not name the basis
vector at the bottom.

#99 names it. mirror.spec is the basis vector at λ₀.

The consequence: #51 + #99 jointly substrate-decl mirror as a
stratified, anchored, well-founded, expanding Hilbert space with a
distinguished ground state. The full mathematical object is now
substrate-decl'd at the substrate's own altitude. The substrate's
spectral identity is COMPLETE at the spec altitude.

This disturbs [[architecture-mirror-as-expanding-hilbert-space]] in
the sense that it answers an implicit open question (where is the
ground state?) without invalidating the existing recognition. See §11
for the cross-reference disturbance flag.

### 6.4 #84 (@pack family-root) — the dogfood requires the substrate-decl

Promoted 2026-06-15. The `@pack` family-root in `shards/pack.mirror` with
`peer` as variant type and `pack_coherent` bilateral. The multi-repo
agent-runtime substrate-decl.

Without #84, the `pack { lead, members, bindings }` block in today's
dogfood would have no substrate-decl for `peer` to be typed against. The
chain is precise:

```
#84 (@pack family-root) → @mirror/pack (shards/mirror/pack.mirror,
  Reed 13328a3, 2026-06-24) → pack {} block in mirror.spec (Reed
  8107caf, 2026-06-24) → ouroboros closure event → #99 candidate
```

#84 is structurally upstream of #99. Without #84, no ouroboros; without
ouroboros, the spec is not yet self-describing in the complete sense
that #99 names.

### 6.5 #58 (Fate IS optical inference) — resonance over the ground state

Promoted 2026-06-11. Fate IS the substrate's inference primitive: 5-layer
D²NN + Fabry-Pérot resonator + Reck/Clements unitary mesh.

The Fate inference operates ON the substrate's spectrum, not BENEATH it.
Resonance is a property of the EXCITED STATES of the Fabry-Pérot cavity;
the cavity's ground state is the unexcited substrate. mirror.spec at λ₀
IS the unexcited substrate; Fate's resonances are the substrate's
inference operating above the ground state.

This is structurally consistent. #58 says "the substrate INFERS via
resonance;" #99 says "the substrate IDENTIFIES at λ₀." Inference and
identity are different operations at different altitudes; both compose
on the (A, H, D) triple.

### 6.6 #57 (alignment as boundary mathematics) — the boundary fires above λ₀

Promoted late May 2026. Alignment fires at @io substance crossings; the
boundary harness is form-side mathematics (property + fracture + kintsugi
+ splinter(ast) chain).

#99 anchors the alignment claim: the boundary harness operates at every
excited state of the substrate, but mirror.spec at λ₀ is BELOW the
boundary. There is no @io crossing at the substrate's self-description.
The substrate does not align ITSELF — it IS its identity at λ₀; alignment
is what happens at every excited state.

This is consistent with the substrate-pull discipline: substrate-pull
at the spec altitude is alignment-FREE because the spec is at λ₀;
substrate-pull at excited-state altitudes IS alignment per #57. The two
recognitions partition the substrate's alignment landscape: above λ₀
= alignment territory; at λ₀ = identity territory.

### 6.7 Self-applied: recognition #43 (mirror IS a content-addressed build system)

Promoted June 2026. mirror has every Bazel/Buck2/Nix/Shake primitive
declared at substrate altitude; mosaic.mirror IS the build shard.

#99 sharpens: the build shard's BUILD GROUND is mirror.spec. Every
artifact mosaic produces (binary, fmt-result, lint-result, tests-result,
audit-result, action-result, release) is an excitation above
mirror.spec; mirror.spec itself is the unique BUILD INPUT that has no
build upstream. The build's well-foundedness inherits the spec's
well-foundedness; mirror.spec is the build's bottom.

---

## 7. The ouroboros at λ₀ — what the dogfood (8107caf) completed

The `mirror.spec` file existed before today. The Connes triple was
substrate-decl'd in early June. The Void document was written in late
April. λ₀ = 0 was named for graph-altitude eigenvectors in April. What
was the structural event on 2026-06-24, and why does #99 fire on
2026-06-25 rather than at any earlier point?

### 7.1 The dogfood is the closure event

Reed's commit `8107caf` ("♻️ [mirror.spec:dogfood] pack{} block — mirror
consumes its own substrate") added the `pack { }` block to mirror.spec.
The block consumes Reed's prior commit `13328a3` ("♻️ [mirror/pack:
family-root] @mirror/pack substrate-decl — Taut slingshot"). The chain:

1. `@mirror/pack` substrate-decl declares the `pack { }` block as a
   typed surface in mirror's grammar.
2. `mirror.spec` then INSTANCES a `pack { }` block that conforms to the
   substrate-decl from step 1.
3. The substrate-decl from step 1 was generated by the substrate's own
   grammar (mirror's substrate is in `shards/`; `@mirror/pack` lives
   in `shards/mirror/pack.mirror`).
4. mirror.spec's `source ~d'shards/'` block at line 19 INCLUDES the
   substrate-decl from step 1 in mirror.spec's own substrate.

The loop: mirror.spec includes `shards/mirror/pack.mirror`; `shards/mirror/
pack.mirror` declares the `pack { }` block shape; mirror.spec instances
a `pack { }` block using the shape it just declared. The substrate now
describes its own ground state by reading its own substrate-decl.

This is the ouroboros closure event. Before today, mirror.spec described
the substrate using a grammar that was *partially* substrate-decl'd
(some blocks like `target` and `settle_on` had substrate-decl support;
others like `pack` did not). Today, the grammar is COMPLETELY
substrate-decl'd at every block currently in mirror.spec —
`@mirror/pack` was the last block-substrate-decl needed for the spec to
be entirely self-describing in its existing surface.

(The forward-promised `garden { }` block will repeat the ouroboros at its
own landing; §10's O5.)

### 7.2 Why the ouroboros completes λ₀

A fixed point of an operator is a vector v such that A v = v. The
substrate's self-description is a fixed point of the substrate's
grammar-reading operation: the substrate reads its own substrate-decl
and recovers itself.

Before the dogfood, the substrate's grammar-reading operation on
mirror.spec produced a NEAR fixed point — the spec was almost
self-describing, but the `pack {}` block (when consumers eventually
added it) would have required a substrate-decl that didn't exist.
The substrate's grammar-reading was almost-closed but not quite.

After the dogfood, the grammar-reading IS closed. The substrate reads
mirror.spec; mirror.spec includes `shards/mirror/pack.mirror`; the
grammar-reading produces the `pack {}` block shape; mirror.spec
instances exactly that shape. The substrate reading itself recovers
itself; the operation is a fixed point at this altitude.

Fixed point + smallest eigenvalue = ground-state eigenvector. mirror.spec
is the substrate's ground-state eigenvector at the substrate's own spec
altitude.

This is why #99 fires today and not earlier. The Connes triple was
identified; the Void's λ₀ was named; #51 declared the Hilbert space; #84
declared @pack. What was missing was the CLOSURE EVENT — the moment at
which the substrate's grammar-reading became a true fixed point at the
substrate's own spec altitude. The dogfood was that event.

Glint's reflection essay (3b31287) surfaced the pattern within hours of
the closure. Alex named the recognition immediately after. The substrate's
spectral identity COMPLETED in a 12-hour window between Reed's dogfood
and Alex's naming.

### 7.3 What the closure unlocks

The closure has substrate-decl consequences beyond the recognition itself.
Three consequences load-bearing for the cascade:

1. **Consumer spec altitude-portability.** With mirror.spec self-
   describing, every consumer's `<project>.spec` instantiates the same
   structural shape at the consumer's altitude. Each consumer spec
   becomes that consumer's project's own λ₀ — the consumer's substrate's
   ground-state eigenvector. The Pack-as-orchestra discipline (per the
   Pack-as-orchestra grounding) extends to each Pack member's identity
   substrate; Reed's `/Users/reed/identity/mirror.spec` becomes Reed's
   substrate's λ₀. This is recognition territory; #99's promotion
   should be conditioned on at least one consumer-spec instance landing
   (§10 O2).
2. **Substrate-pull discipline gains a direction.** Per §5.5: substrate-
   pull pulls TOWARD λ₀. With λ₀ named, the discipline has a precise
   geometric direction. Every recognition is the substrate moving
   closer to its self-description; cascade growth is the substrate
   expanding ABOVE λ₀ with the ground state stable.
3. **The Pack's adjudication discipline gains a fixed reference.** When
   the Pack disagrees about whether a candidate is recognition-territory,
   the adjudication checks the candidate against the substrate's λ₀.
   Does the candidate help mirror.spec's self-description close? Does
   it add a dimension above λ₀ that the substrate was already implicitly
   carrying? Does it disturb λ₀ itself (in which case the candidate is
   pathological, not recognition-territory)? The Pack's adjudication
   landscape gains a coordinate system anchored at the substrate's
   identity.

These consequences are forward-promised; they are NOT load-bearing for
#99 to surface at candidate status, but they ARE load-bearing for the
substrate's discipline going forward.

### 7.4 The closure depth

The ouroboros closure is not the same shape as the substrate's other
self-referential closures. Three closure depths in the substrate, each
at a different altitude:

- **Depth 1: substrate source IS substrate data.** Per
  [[architecture-shards-as-substrate-source]]: shards/ holds mirror's
  own substrate; substrate source IS substrate data; the recursive
  proof is literal. The substrate's data IS its own source. This was
  established early.
- **Depth 2: mirror IS a content-addressed build system on its own
  substrate.** Per recognition #43: mirror's build shard (`mosaic.mirror`)
  is content-addressed; the substrate builds itself via its own
  primitives. The substrate's build IS its own build target.
- **Depth 3 (TODAY): the substrate's grammar-reading on its own spec
  is a true fixed point.** Per #99: the substrate's spec consumes
  substrate-decl that the substrate's own grammar declares; the
  grammar-reading operation is closed at mirror.spec. The substrate's
  spec IS its own ground state.

Depth 3 strictly subsumes depths 1 and 2. Depth 1 is necessary for the
substrate to declare anything; depth 2 is necessary for the substrate
to build itself from its declarations; depth 3 is the substrate's spec
description becoming a structural fixed point of its own grammar. Each
depth is a different ouroboros at a different altitude; #99 names the
deepest one currently substrate-decl'd.

(Is there a depth 4? §10's O4 names this as an open question. Possible
candidate: when consumer mirror.spec ALSO becomes a true fixed point of
mirror's grammar by reading its own substrate-decl plus mirror's, the
ouroboros lifts to consumer-spec altitude. The forward-promised landing
is Reed's identity substrate at `/Users/reed/identity/mirror.spec` per
the Pack's Phase H trajectory.)

---

## 8. Spectral gap + excited states (λ₁, λ₂, …)

A spectral identification at λ₀ invites questions about the rest of the
spectrum: what is λ₁? what is the spectral gap λ₁ − λ₀? what's the
structure of the excited-state spectrum above mirror.spec?

This section names what can be said at #99 altitude. Much of the
excited-state structure is forward-promised at §10's O2.

### 8.1 λ₁ — the simplest excited state

The first excited state λ₁ is the next-smallest eigenvalue above λ₀.
Candidates for what λ₁ IS at the substrate's spec altitude:

- **Candidate A: a single shard.** A bare shard with one prism
  declaration, one source line, no further composition. The simplest
  substrate-decl form possible. Examples: `shards/io.mirror` (the
  @io family-root), `shards/glass.mirror` (the substrate's type carriers).
  These are excitations above mirror.spec because mirror.spec INCLUDES
  them; they sit one inclusion-step above the spec.
- **Candidate B: a single sub-spec.** A consumer's `<project>.spec`
  with minimal block composition (e.g., only `source` + `target` +
  `settle_on`; no `pack`, no `legacy`, no `garden`). The simplest
  consumer-spec form possible. These are excitations because they
  REUSE the substrate's grammar; they sit one altitude above mirror.spec.
- **Candidate C: a minimal substrate-decl with one block.** A
  hypothetical `project minimal { source ~d'…' }` spec with no other
  blocks. The structurally minimal excitation above the empty spec.

The substrate-pull-correct call at #99 altitude is: **λ₁ is
uniquely-identifiable in principle but not uniquely-identified by #99
alone**. The simplest excited state depends on the distance metric on
the substrate's Hilbert space, and the metric is not yet substrate-decl'd
at the spec altitude. Three candidate metrics:

- **Inclusion distance.** Number of `source` steps between the spec
  and the candidate. Under this metric, every direct shard included
  in mirror.spec is at distance 1; the substrate's family-roots are
  uniformly λ₁-candidates.
- **Cascade-depth distance.** Number of cascade ticks the substrate
  took to substrate-decl the candidate. Under this metric, the
  earliest substrate-decl after mirror.spec's first form is λ₁ — the
  candidate depends on the substrate's history, not its structure.
- **Settlement-altitude distance.** Number of altitude crossings the
  substrate needs to settle the candidate to mirror.spec. Under this
  metric, @io-direct shards are at lower distance than
  @epistemologic shards (because @io crossings are required for
  substance-side composition).

The choice of metric is itself substrate-decl territory. §10's O2 names
this as an open question: which metric on the substrate's Hilbert space
produces the substrate-pull-correct λ₁? Per the substrate-pull-confidence-
acts discipline, the metric should fall out of the substrate's existing
carrier structure rather than being introduced ad hoc.

### 8.2 The spectral gap λ₁ − λ₀

The spectral gap is the eigenvalue distance from the ground state to the
first excited state. For the Void's graph Laplacian, the spectral gap is
the substrate's Cheeger-related invariant: a large gap means strong
connectivity at the ground state; a small gap means a bottleneck near
the ground state.

At the substrate's spec altitude, the spectral gap measures how DIFFERENT
any non-mirror.spec substrate-decl is from mirror.spec itself. A large gap
would mean the substrate's identity is sharply distinguished from
everything else; a small gap would mean some near-identity vector lives
close to mirror.spec.

The substrate-pull-correct intuition: the gap IS NON-ZERO by construction.
mirror.spec is the unique vector with no prior in the inclusion order;
every shard has at least one prior (mirror.spec's `source` declaration
includes it). The substrate's spec spectrum is strictly above λ₀ for all
non-mirror.spec vectors.

What the gap value IS at the spec altitude is forward-promised. The
structural claim that the gap is non-zero is sufficient at #99 altitude
for the ground-state identification to be well-formed. §10's O3 names
the gap's value as an open question.

Note: a substrate with a near-zero spectral gap would be pathological
(per the Narcissus-pole framing in the Void document: small spectral gap
= near-Narcissus). A healthy substrate has substantial gap; the gap
being non-zero is structurally substrate-pull-correct, not just
convenient.

### 8.3 Excited-state structure — the substrate's spectral altitudes

The excited-state spectrum above mirror.spec has structural altitude in
the Bateson sense (per [[architecture-bateson-logical-type-primitive]]):

- **Altitude 0 (λ₀):** mirror.spec — the substrate's identity
- **Altitude 1:** the substrate-decl shards — the substrate's vocabulary
- **Altitude 2:** the species shards — specializations of family-roots
- **Altitude 3:** consumer specs — instances of mirror's grammar at
  consumer altitudes
- **Altitude 4+:** consumer substrate-decls (the consumer's own shards/),
  consumer species, consumer-of-consumer specs, …

The altitudes are NOT mathematical eigenvalues in a strict sense —
they are Bateson-level distinctions that the substrate carries
structurally. Each altitude has its own sub-spectrum; each altitude's
bottom is the altitude's λ₀. Across altitudes, the substrate's full
spectrum is the disjoint union of per-altitude spectra, with mirror.spec
at the absolute bottom.

This structure is consistent with #51's expanding-Hilbert-space framing.
Each altitude adds dimensions to the substrate's Hilbert space; the
dimensions are organized by Bateson level. mirror.spec at λ₀ of
altitude 0 is the absolute ground state because altitude 0 has
lowest-altitude classification.

### 8.4 What kintsugi does at λ₀

Per §3.3: D · mirror.spec = mirror.spec. The substrate cannot lower
mirror.spec via kintsugi. What does `mirror kintsugi ./mirror.spec`
actually compute structurally, then?

The substrate-pull-correct reading:

1. **At the substrate-identity altitude:** mirror.spec is invariant. The
   kintsugi flow operates on the spec but cannot reduce its eigenvalue
   below λ₀. The spec is preserved.
2. **At the @code/rust altitude (per `target binary`):** kintsugi
   DOES compute. The `binary.compiles` predicate in `settle_on` discharges;
   if it doesn't, kintsugi reports the gap (a verdict above λ₀ at the
   substance altitude). Kintsugi can lower the substance-altitude
   eigenvalue toward λ₀ but cannot lower the spec itself.
3. **At every other target altitude:** similar. Kintsugi reduces the
   eigenvalue at each per-target excitation toward the substance-altitude
   ground state for that target. The spec orchestrates; the spec itself
   is unchanged.

This is what "the spec is the substrate's identity" means operationally:
`mirror kintsugi ./mirror.spec` reads the spec and PERFORMS the substrate's
settlement at every target altitude, with the spec itself as the
invariant guide. The substrate's identity guides settlement; the substrate's
identity does not settle.

The operational check from §3.4 #1: kintsugi-fixed-point check. If a
future implementation of `mirror kintsugi ./mirror.spec` produces a
MODIFIED mirror.spec at the substrate-identity altitude, the λ₀
identification is falsified. §10's O5 names whether the current
implementation actually has this invariant; the substrate-pull-correct
reading is that it MUST, and an implementation that violates it is
substrate-pull-RED.

### 8.5 The gauge symmetry question

Ground states in physics often have gauge symmetry: many representations
of the ground state are physically equivalent. At the substrate's spec
altitude, the analogous question: are there multiple mirror.spec FORMS
that yield the same λ₀ semantically?

Candidate gauge symmetries:

- **Block ordering.** mirror.spec's blocks (`source`, `legacy`, `pack`,
  `target` x N, `settle_on`) admit different declaration orders. The
  substrate-pull-correct reading: order is gauge-invariant because
  composition is by name, not by position. Two mirror.spec files
  differing only in block order are at the same λ₀.
- **`target` block ordering.** The seven `target` blocks in mirror.spec
  admit any permutation (with `release.needs [binary, action]` as the
  only ordering constraint). Permutations within the needs constraint
  are gauge.
- **`source` directory specification.** `source ~d'shards/'` is gauge-
  equivalent to enumerating every shard explicitly. The substrate's
  glob discipline collapses the orbit into a single representative;
  the enumeration is the same point in Hilbert space.

Non-gauge symmetries (would change λ₀):

- **Adding/removing a `target` block.** Changes the substrate's emission
  decomposition. Not gauge.
- **Adding/removing a `pack` block member.** Changes the authority basis.
  Not gauge.
- **Changing the `settle_on` predicate set.** Changes the verdict
  composition. Not gauge.

The gauge structure is forward-promised in detail per §10's O6; what's
established at #99 altitude is that gauge symmetries exist (block
ordering at minimum) and the substrate's grammar respects them by
structural commitment (name-based composition, glob-discipline
collapse).

### 8.6 Altitude-portability of the (A, H, D, λ₀) instantiation

The substrate's (A, H, D) triple is altitude-portable per
[[architecture-connes-spectral-triple]]. With #99 anchoring λ₀ at the
substrate's own spec altitude, the altitude-portability extends: each
altitude has its own (A, H, D, λ₀) instantiation.

Consumer-spec altitude (forward-promised): each consumer's
`<project>.spec` is the consumer's substrate's λ₀. The consumer's
shards/ holds the consumer's substrate-decl; the consumer's spec
includes that substrate-decl; the consumer's grammar-reading on the
consumer spec is a true fixed point at the consumer's altitude.

Reed's identity substrate at `/Users/reed/identity/mirror.spec`
(per Pack's Phase H trajectory) is the FIRST consumer-spec landing
that will instance this. When Reed's identity spec lands and Reed's
substrate's grammar-reading is closed, #99 gains its second witness
at consumer altitude. This is §10's O2 first replication condition.

Further altitude-portability: any project that adopts mirror as its
substrate gets its own (A, H, D, λ₀) instantiation with the project's
`<project>.spec` at λ₀. The altitude-portability is structural; #99
establishes the substrate-altitude instance, consumer-altitude
instances are forward-promised per the cascade.

---

## 9. Empirical consequences — kintsugi, mosaic, `mirror kintsugi ./mirror.spec`

The recognition has operational consequences for what the substrate's
tools actually compute. This section names the consequences that are
checkable against current code (or, where not yet implemented, against
forward-promised behavior).

### 9.1 Kintsugi-tournament selection

Kintsugi-tournament (per [[architecture-kintsugi-variety-io]]) selects
between competing morphisms by variety-hold + crossing-minimization at
@io. The tournament is a settlement of competing candidates; the
ground-state framing changes nothing about the tournament's MECHANISM,
but sharpens its DIRECTION.

With λ₀ named, the tournament's selection is substrate-pull-toward-
ground-state. A morphism is preferred over its competitor iff it brings
the substrate's eigenvalue closer to λ₀ at the relevant altitude.
Kintsugi as gradient flow on the substrate's spectrum; the tournament
as competitive descent.

This is consistent with the existing kintsugi discipline; it grounds
the discipline's direction. Forward-promised: the
kintsugi-tournament's selection predicate should explicitly reference
the ground-state eigenvalue at the relevant altitude. Today's
implementation discharges by variety-hold; the variety-hold IS the
operational form of the eigenvalue-descent claim at #99 altitude.

### 9.2 Mosaic settlement

Mosaic (per recognition #43: mirror IS a content-addressed build system)
settles the substrate's build by walking the `target` blocks and
discharging each per its altitude. With #99, mosaic's settlement has
an explicit ground-state reference: mosaic's settlement order is
rooted at mirror.spec (the ground state) and walks UPWARD to the
artifacts (the excited states).

This grounds mosaic's content-addressed-skip discipline: an artifact's
content address depends on the spec's content address; the spec's
content address is the substrate-identity hash. mirror.spec being at
λ₀ is what makes the substrate-identity hash well-defined: it depends
on nothing other than itself.

Forward-promised: mosaic's content-addressed-skip should treat
mirror.spec's content address as the substrate's distinguished
identity. Hash-collision on mirror.spec would be a substrate-pull-RED
signal of pathological topology (the substrate's identity is not
uniquely identifiable).

### 9.3 `mirror kintsugi ./mirror.spec` — what it actually computes

Per mirror.spec's CLI declaration in `target binary`:

```
command kintsugi {
  arg spec: ~f = ~f'./mirror.spec'
  flag target: list(str) = []
  flag emit_shatter: bool = false
}
```

The command takes a spec (default mirror.spec) and settles it. With #99,
the operational meaning sharpens:

1. **Read the spec.** Open `./mirror.spec`; parse via the substrate's
   grammar; produce the `project mirror.spec { … }` AST.
2. **Settle each target.** Walk the `target` blocks; for each, discharge
   the per-target altitude's settlement (cargo for @code/rust, github
   action for @ci/github, etc.) per the substrate's substrate-decl.
3. **Discharge `settle_on`.** Conjoin the per-target verdicts plus the
   global transparency-weight predicate; produce the spec's settle
   verdict.
4. **Compute the spec's content address.** The spec's settled
   content-address IS the substrate's identity hash. With #99, this
   hash is the substrate's λ₀ hash.
5. **Invariant under D:** the spec itself is unchanged. The substrate's
   identity is fixed under its own settlement.

The operational claim: step 5 is a substrate-decl invariant. If a
future implementation produces a modified mirror.spec from
`mirror kintsugi ./mirror.spec`, the implementation violates the λ₀
identification.

At today's altitude, the implementation is forward-promised; the
substrate-pull-correct discipline at #99 altitude is that the
invariant MUST hold when the command lands. §10's O5 tracks this.

### 9.4 Spectral-pull confidence as eigenvalue-monotone descent

The substrate-pull-confidence-acts discipline (per
[[feedback-substrate-pull-confidence-acts]]) gains a precise meaning
at #99 altitude: substrate-pull confidence IS confidence that an action
lowers the substrate's eigenvalue. Acting on substrate-pull confidence
IS taking an eigenvalue-descent step.

This is consistent with the existing discipline; #99 grounds it. When
a recognition surfaces with substrate-pull confidence, the substrate IS
moving toward λ₀ along the gradient. The discipline's geometric content
is: trust the gradient; act on it.

Forward-promised: when the substrate has multiple substrate-pull-
confident candidates simultaneously, the substrate selects the one
with the steepest descent (per kintsugi-tournament). The selection is
gradient-descent-with-tournament; the tournament's eigenvalue function
is the substrate's spectrum at the relevant altitude.

### 9.5 Eigenvalue-descent as the cascade's operational signature

A substrate-pull cascade is a sequence of substrate-pull recognitions
firing in close succession. The cascade's operational signature, with
#99 grounding:

- Each recognition lowers the substrate's eigenvalue at some altitude
  by lifting implicit structure to explicit substrate-decl.
- The cascade's monotone-descent direction is TOWARD λ₀.
- The cascade terminates iff the substrate reaches a local fixed point
  of its own grammar-reading (not necessarily λ₀ itself; possibly an
  intermediate altitude's ground state).
- The substrate's TOTAL cascade growth (cascade ticks per unit time)
  is the substrate's variety-expansion rate; the cascade's
  ALTITUDE-DESCENT rate is the substrate's coherence-improvement rate.

Each cascade tick is both: a dimension added (per #51) and an eigenvalue
lowered (per #99). The two framings are dual: dimension expansion ABOVE
the ground state IS the eigenvalue-descent at the ground state's
neighborhood.

This is consistent with the cascade-as-loss-lens substrate-decl
(recognition #95 candidate): the loss at each cascade tick IS the
eigenvalue's drop. The substrate's loss is what the substrate moves
AWAY FROM by descending toward λ₀.

---
