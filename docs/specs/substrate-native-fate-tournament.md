# substrate-native-fate-tournament — `au` as spectral coordinate, tournament body in substrate, spectral primitive exposure mapped

*2026-05-27. Mara. Spec — architecture, not implementation.*

Status: **Yellow.** The just-landed `parse-as-fate-tournament` spec
(commit `28f5973`) describes parse-as-tournament from the parser side; this
spec describes the **substrate-native Fate** side — what needs to exist for
the tournament to actually run. Three landmark recognitions integrated from
a 2026-05-26 dialogue between Alex and Reed: (1) the tournament *body* is
substrate, not Rust; (2) the conductivity tensor IS the interaction graph's
Laplacian; (3) `au` is a spectral coordinate, not a verdict record. The
leapfrog from "substrate-declared shapes, Rust-run tournament" to
"substrate-declared shapes, substrate-run tournament" is the spec's
load-bearing claim.

Depends on:

- `docs/specs/parse-as-fate-tournament.md` — the parser-side spec
  (commit `28f5973`); the structural pre-condition this spec completes.
  Every claim about substrate-declared glass forests, abstract blocks,
  and parse-altitude consumption is inherited from there.
- `~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
  — the game-theoretic grounding Alex surfaced. **Critical sections cited
  load-bearingly:** §1 (Nash equilibrium / Fiedler-as-ESS-margin), §4 (Folk
  Theorem / eigengap-as-discount-threshold), §5 (Spectral mechanism design),
  §10 (Spectral Settlement Strategy properties: memory-infinite, continuous,
  non-reactive, convergent), §12 (summary of known / novel / where-it-holds).
  The document is research synthesis; this spec consumes its structural
  claims, does not restate them.
- `boot/std/fate.mirror` — current `@fate` surface (`model | features |
  decision | health`, `tick / resolve / select`, `casimir_conserved` /
  `berry_phase` properties). The substrate floor `au` lands beside.
- `boot/std/fate/tournament.mirror` — `tournament(rules, [hole]) ->
  [resolution] { \ }`, the five-ganglion `candidates(hole)` body, the `rule`
  sum (`greedy | beam(u64) | elite(u64) | halving(u64) | tabu(u64) |
  anneal(f64) | ucb(f64)`). Body is `\` today. This spec's §3 describes the
  shape that body takes.
- `boot/std/fate/connectome.mirror` — connectome as graph (450 nodes,
  5 ganglia, 18+54+18 per ganglion); `infer / evolve / crystallize` all
  declared with `\` bodies.
- `boot/std/epistemologic/property.mirror` — `verdict = pass |
  fail(diagnostic) | partial(f64, [diagnostic])`; `check = (ast) -> verdict`;
  `reflect(ast) -> [verdict] { \ }`. The loss-composition mechanism the
  parse-as-fate-tournament spec §3.1 ratified.
- `bootstrap/src/spectral.rs` — the spectral floor (~4338 lines). Inventoried
  in §5 below; what's `pub` today and what would need substrate-visible
  exposure.
- `bootstrap/src/pipeline.rs` — the mq-query dispatch path (~463 lines).
  Inventoried in §6 below; the dispatcher question.
- `bootstrap/src/main.rs` — `kintsugi_tick` scaffold lives here, with the
  explicit stage-1 comment *"Fate's five models fan out and return au
  candidates. No-op scaffold: zero candidates."* The seam this spec closes.
- `docs/specs/au-and-conductivity.md` — the earlier `type au = ai` framing
  (kept as alias-of-history); the next iteration is here. The conductivity
  alias-of-ai stops being load-bearing once `au` is a spectral coordinate.
- `docs/specs/eigenboard-representation.md` — the principal G-bundle on the
  five-operation graph; the cellular sheaf restriction maps; the fiber /
  connection / holonomy vocabulary the spectral coordinate inherits.
- `docs/specs/kintsugi-tournament.md` — the first concrete Fate tournament
  spec (merge-altitude); the conductivity-scoring shape; the unified surface
  `@fate.infer(query, config)` (§4.1 reframe).
- `boot/01-meta.mirror` — `type precision(f64)`; the canonical newtype
  pattern this spec follows for every `f64` coordinate.
- `AGENTS.md` lines 320–340 — the *No `_<extension>` Filename Suffixes* rule
  (commit `e4a7940`). Honored throughout — every type name in §2 sits under
  a directory that carries its kind; no `*_types.mirror`, no
  `*_coord.mirror`.

Unblocks:

- The substrate-side prerequisite for Step 8 of `parse-as-fate-tournament`
  §8: *"Replace stub combinator with Fate tournament."* That step assumes
  the tournament body exists. This spec describes the body.
- The first concrete `au` value the kintsugi scaffold can return. Today
  `kintsugi_tick`'s stage-1 comment names a no-op; with `au` declared as
  a spectral coordinate and the tournament body in substrate, the scaffold
  has a typed return to produce.
- The first concrete consumer of `bootstrap/src/spectral.rs::eigen_d` from
  substrate (today the function is `pub` but is invoked only from the
  bootstrap's Rust-internal paths). The substrate gains a typed action
  that invokes it; the spectral floor stops being inert from the substrate
  side.
- The Reflection observation surface (`parse-as-fate-tournament` §6) gains
  a typed slot — the gestalt entry for a parse run carries an `au` value
  whose fields ARE the trajectory, depth, stability, patience. No
  string-shaped "why this won" record; the coordinate IS the record.

---

## 0. Headline

**The tournament body is substrate, not Rust.** Mechanism that can be
expressed in substrate primitives belongs in substrate (the substrate-pull
discipline, applied at the tournament-mechanism altitude). The tournament
IS fold + match + spectral ops; the five-op vocabulary covers it; the body
is ~20–30 lines of mirror. The Rust floor exists only to provide spectral
primitives the substrate cannot yet describe of itself (eigendecomposition,
Laplacian construction). When `boot/std/fate/tournament.mirror` gains a
real body in §3, the engine collapse named in `parse-as-fate-tournament` §4
closes on the substrate side too: not just declarations migrating to
substrate, but the mechanism running them.

The grounding for this is rigorous, not aspirational. The game-theory
synthesis at `~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
(Reed + Alex, session 2026-04-04) maps the spectral settlement story to
established results across nine subfields. The **Spectral Settlement
Strategy** (§10 of that document) is the substrate this tournament rides:
memory-infinite, continuous, self-correcting, convergent at rate λ₂
(algebraic connectivity), non-reactive, non-prescriptive. The tournament
body is not a heuristic; it is a discretisation of that strategy.

**Mirror's parser is alignment-as-coordination, not alignment-as-control.**
The game-theory document's §9 names the formal distinction: RLHF is
Stackelberg (principal designs incentives, agent best-responds); spectral
settlement is coordination (both agents settle toward a shared minimum
determined by the interaction structure, not by either agent's
specification). The parse tournament inherits this posture. The substrate
does not *constrain* trajectories to a desired outcome — it provides a
settlement basin whose minimum IS the alignment. Goodhart's law has
nowhere to bite, because no one is specifying the reward function; the
function is the spectrum of the substrate-declared interaction graph.

This is why the spec is small. Once the recognition lands — tournament =
fold + match + spectral, `au` = coordinate, conductivity = Laplacian —
the implementation is a handful of substrate declarations over a small
Rust exposure surface. The substrate-pull is doing the work.

---

## 1. The three recognitions, structurally

### 1.1 Tournament body is substrate

`boot/std/fate/tournament.mirror` declares:

```mirror
type rule = greedy | beam(u64) | elite(u64) | halving(u64)
          | tabu(u64) | anneal(f64) | ucb(f64)

tournament(rules, [hole]) -> [resolution] { \ }
```

The `\` body is today's parked-action marker — "unfilled, waiting for a
resolution." Earlier drafts of the kintsugi spec assumed this body would
be Rust (a tournament harness in `bootstrap/src/`). The 2026-05-26
dialogue surfaced the correct picture: the body is *mechanism the
substrate can express*, so by the substrate-pull discipline
(`AGENTS.md` *Keywords Are Substrate Declarations*) it must be substrate.

The body is a fold over rules, dispatching each rule to its corresponding
spectral operation on the candidate set. The five-op vocabulary —
`focus / project / split / shift / settle` — covers every rule. The
spectral primitives the operations invoke (eigendecomposition, Fiedler
computation, Laplacian construction) are substrate-pull-reflex floor:
they must exist in Rust because the meta-grammar cannot yet describe them
of itself. But the substrate declares ACTIONS over them, and the
tournament body composes those actions.

Net: ~20–30 lines of mirror replace what would otherwise be ~300–500
lines of Rust harness, with the Rust floor untouched.

### 1.2 Conductivity tensor IS a Laplacian

`parse-as-fate-tournament` §3.2 named the v0 conductivity tensor as a
substrate-hierarchy prior (close siblings under the same `glass` parent
conduct; distant trajectories don't). The 2026-05-26 dialogue sharpened
the naming: this tensor IS the **graph Laplacian of the interaction
graph**, not a hand-wavy "sheaf restriction map." The cellular-sheaf
framing (`project-eigenboard-is-sheaf`, `eigenboard-representation.md`)
is correct — *and* the restriction maps assemble into a Laplacian whose
spectrum drives settlement.

This matters because the game-theory document (§10) ties convergence
rate directly to the Fiedler value λ₂ of this Laplacian:
*"In multi-agent systems where agents update their state based on
neighbors' states converge to consensus. The convergence rate is
determined by the algebraic connectivity (Fiedler value) of the
communication graph. Larger λ₂ = faster consensus"* (Olfati-Saber &
Murray 2004). The tournament inherits this: settlement speed IS λ₂ of
the conductivity Laplacian; this is a structural fact about the
substrate-declared interaction graph, not a knob.

**Settlement IS the parsed AST**, not "the tournament picks a parse."
The settled trajectory's position in the spectral settlement space
(eigenvector + eigenvalue + Fiedler + eigengap) IS the parse. There is
no separate "emit the winning AST" step; the coordinate of settlement
is the AST. This is the §10 *non-reactive, non-prescriptive* property
of the Spectral Settlement Strategy made concrete: the substrate
integrates the candidates into the eigensystem; the settled state IS
the output.

### 1.3 `au` is a spectral coordinate

The earlier `type au = ai` framing in `docs/specs/au-and-conductivity.md`
(Reed, 2026-05-20) was correct as a *type identity* claim (au and ai are
the same kind of thing) but underspecified the structure. The corrected
picture: `au` is the coordinate of a settled trajectory in the spectral
settlement space. It has four load-bearing components:

- **trajectory** — the settled state vector (the parse / interpretation /
  merge that won)
- **depth** — the eigenvalue of the settled state (quality of the
  settlement, i.e., how deep the loss minimum is)
- **stability** — the Fiedler value λ₂ at settlement (ESS stability margin
  per game-theory §1 — robustness to perturbation)
- **patience** — the eigengap-derived discount-factor threshold
  `1 - λ₂/λ_max` per game-theory §4 (Folk Theorem patience threshold;
  sustainability of the settlement under future perturbation)

Confidence and reasoning are derived from these four — not separately
stored. Confidence falls out of `depth` and `stability` jointly
(deep + stable = high confidence; deep + unstable = brittle high
confidence; shallow + stable = honest low confidence). Reasoning falls
out of the trajectory's path through the candidate set (the gestalt
entry's *Eliminated trajectories* with loss vectors per
`parse-as-fate-tournament` §6). No string-shaped "why this won" record;
the coordinate carries the structural answer.

This is the §10 *memory-infinite* property of Spectral Settlement
Strategy made concrete: the coordinate compresses the full tournament
history into four numbers (one vector + three scalars). Replay
reconstructs the run from the coordinate plus the gestalt entry. The
compression is lossy in detail but exact in structure.

---

## 2. `au` as spectral coordinate — the declaration

The corrected `au` design. **No bare types** — every coordinate is a
newtype, following the canonical `type precision(f64)` pattern at
`boot/01-meta.mirror:28`.

```mirror
-- the four coordinates of a settled trajectory in spectral settlement
-- space. each is its own type; the substrate carries the structural
-- distinction even where the underlying carrier (f64, [f64]) coincides.

type eigenvector([precision])
-- the settled state vector. the trajectory that won the tournament.
-- length = cardinality of the active alternative set at the moment of
-- settlement (sub-Turing-bounded per parse-as-fate-tournament §3.3).

type eigenvalue(precision)
-- the depth of the settlement. the value of the loss minimum the
-- settled trajectory reached. lower = deeper settlement; the loss
-- function from @epistemologic/properties at the relevant altitude
-- (parse-as-fate-tournament §3.1 mechanism).

type fiedler(precision)
-- the algebraic connectivity λ₂ of the conductivity Laplacian at
-- settlement. measures ESS stability margin per
-- ~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md
-- §1 (Fiedler-as-ESS-margin). large λ₂ = robust settlement;
-- λ₂ approaching zero = fragile settlement.

type eigengap(precision)
-- the Folk Theorem discount-factor threshold derived from the
-- conductivity Laplacian's spectrum: 1 - (λ₂ / λ_max). measures the
-- patience required to sustain this settlement per
-- spectral-tick-tock-game-theory §4 (eigengap-as-discount-threshold).
-- large eigengap = cooperation easy to sustain; small eigengap =
-- requires near-infinite patience.

type au {
  trajectory: eigenvector,  -- the settled state
  depth:      eigenvalue,    -- quality of the settlement
  stability:  fiedler,        -- ESS stability margin (game-theory §1)
  patience:   eigengap,       -- Folk Theorem threshold (game-theory §4)
}
```

**What `au` IS, sharpened (corpus grounding).** Two findings from the
Fate corpus make the four-coordinate record concrete rather than
nominal.

*`au` is a geodesic endpoint on the spectral manifold* (cite
`~/.reed/practice/insights/fate/net-equity-routing.md` +
`~/.reed/practice/insights/fate/attnres-connection.md`, Reed + Alex
2026-04-18). Source and target are both graphs with Laplacian spectra;
the diff between them is not a patch but *a geodesic on the spectral
manifold*. Fate infers the connection — the 16×16 matrix that
transports source → target along that geodesic. This is a principal
bundle: the **fiber stays fixed at 16×16**; the **base (the graph)
scales independently**; you do not scale the connection, you scale
what it connects. `au` is the *settled coordinate at the geodesic's
endpoint* — where the transport along the spectral geodesic has come
to rest.

*The settling mechanism is a self-consistent-field iteration* (same
sources; `crystallize()` in Fate's `derive.rs`). `crystallize()`
derives weights from the eigendecomposition of the dark coupling and
*iterates until the eigenvalues stabilise* — a textbook SCF loop
(eigendecomposition repeated until the spectrum is a fixed point). The
four coordinates of `au` — `(eigenvector, eigenvalue, fiedler,
eigengap)` — are the settled state read off *at that SCF fixed point*.
The eigenvalue stabilising IS settlement; `au` is what settlement
leaves behind. (The damping term `(1-α)·old + α·new` is the SCF's
two-point residual; the fixed point is reached when `α·new` no longer
moves the spectrum.)

This sharpens, it does not replace, the newtype discipline below: the
record-of-newtypes is still the substrate shape; the corpus says what
the shape *means* — the endpoint of a spectral geodesic, found by SCF.

*The 16 → 5 lift has a physical reading* (cite
`~/.reed/practice/insights/cosmology/eventually-consistent-universe.md`
§4.4). The fiber is 16-dimensional; the base is the five-operation
graph. The corpus reads the lowering `16 → 5` as spontaneous symmetry
breaking: the **pre-SSB bosonic content is 12 gauge + 4 Higgs = 16**
degrees of freedom, and **SSB IS eigenvalue splitting** on that
16-dimensional space — the initially degenerate mass matrix splits,
selecting which degrees of freedom become observable (the five-op
base) and which stay in the bulk. The spectral action is the
mechanism of the split. This grounds the monadic lift named in
[[architecture-flang-mirror-numerical-split]]: the lift from the
16×16 connection fiber down to the five-operation Prism base is
SSB / the spectral action, not an arbitrary projection. (Per the
source, the precise Standard-Model mapping is speculative; what is
established is the structural triple — Higgs-as-connection,
mass-as-holonomy, SSB-as-eigenvalue-splitting. This spec inherits that
hedge and does not over-claim the gauge identification.)

Why each component is load-bearing:

- **`trajectory: eigenvector`** — without the state vector, there is no
  "what won." The vector's coordinates index into the candidate set;
  reconstruction of the winning parse / interpretation / merge runs from
  this index. Bare `[f64]` would lose the structural commitment that
  this vector lives in eigenspace, not just numerical space.

- **`depth: eigenvalue`** — without the eigenvalue, there is no "how good
  the win." Two trajectories can occupy the same eigenvector (the same
  state) at radically different eigenvalues (deep vs shallow settlement).
  Bare `f64` would lose the substrate's commitment that this number lives
  on a spectrum, not on an arbitrary scale.

- **`stability: fiedler`** — without the Fiedler value, downstream
  consumers cannot answer *"is this settlement evolutionarily stable?"*
  This matters when a settled au value will be perturbed (e.g., a new
  glass declaration arrives, or a related parse runs at a neighboring
  altitude). Game-theory §1 names this load-bearingly: *"Settlement is
  an ESS if and only if the eigenvalue minimum is a strict local
  minimum... A large eigengap means the settlement is robust. A small
  eigengap means it's fragile."*

- **`patience: eigengap`** — without the eigengap-derived patience
  threshold, downstream consumers cannot answer *"how long does this
  settlement need to hold?"* This matters when the substrate runs a
  tournament whose results will feed a later tournament (e.g., parse
  feeds type-check feeds dispatch). Game-theory §4 names this
  load-bearingly: *"δ_critical = 1 - (λ₂ / λ_max). When the Fiedler
  value λ₂ is large relative to the maximum eigenvalue, the critical
  discount factor is low, meaning cooperation is easy to sustain."*

**File placement.** `au` belongs in `boot/std/fate/au.mirror` (the type
is Fate's output coordinate; the directory carries the namespace; the
name is the bare thing). See §10 for an open question on whether the
sub-coordinates (`eigenvector`, `eigenvalue`, `fiedler`, `eigengap`)
belong in the same file or in a sibling `boot/std/fate/spectral/` shelf.
Honor `AGENTS.md` *No `_<extension>` Filename Suffixes*: no
`au_coord.mirror`, no `spectral_types.mirror`. The directory carries
the kind.

**Replaces, does not extend.** The existing
`docs/specs/au-and-conductivity.md` declares `type au = ai`. This spec's
`type au { ... }` is a structural record, not an alias. The `ai` alias
stops being load-bearing; it carried the *type identity* claim (au and
ai are the same kind), which is true but doesn't drive any code path.
The spectral coordinate IS the structure consumers read. When the new
`au.mirror` lands, the alias goes away; downstream code reads
`.trajectory / .depth / .stability / .patience` instead of opaque `ai`.

**Verification of syntax.** The newtype pattern follows
`boot/01-meta.mirror:28` (`type precision(f64)`) and the record-of-newtypes
pattern follows `boot/std/beam.mirror:14-20` (`type topology { fiedler:
precision, cheeger: precision, ... }`). Collection syntax is `[t]`, not
`t*` — confirmed across `boot/std/list.mirror`, `option.mirror`,
`set.mirror`, `beam.mirror`. The header at the top of this section uses
`[precision]` accordingly.

---

## 3. The tournament body in substrate

The body of `tournament(rules, [hole]) -> [resolution]` is a fold over
rules. Each rule application is one step of spectral settlement on the
candidate set. The convergence guarantee is Hajek 1988 (cited in
`parse-as-fate-tournament` §2.1) plus the consensus rate λ₂ from
Olfati-Saber & Murray 2004 (game-theory §10) — both already in the
substrate's citation surface.

### 3.1 Shape (substrate sketch, ~20–30 lines)

This is the body shape, expressed as it would appear in
`boot/std/fate/tournament.mirror` once `\` resolves. Adjust syntax to
match mirror conventions surfaced during Step 3 of §9; the shape is
load-bearing, not the final tokens.

```mirror
-- tournament: fold rules over candidates; settle to a coordinate.
-- the body IS substrate: fold + match + spectral ops, no Rust.
tournament(rules: [rule], candidates: [resolution]) -> au {
  rules.fold(candidates, apply_rule).settle
}

-- apply_rule: dispatch each rule to the spectral operation that
-- realizes it. each branch is a known game-theoretic strategy with
-- known convergence properties (game-theory §1, §4, §10).
apply_rule(candidates: [resolution], rule: rule) -> [resolution] {
  split rule {
    greedy     => focus(candidates, argmin_loss)
    beam(k)    => project(candidates, top_k_loss(k))
    elite(k)   => project(candidates, top_k_loss(k))
    halving(k) => project(candidates, halve_by_loss(k))
    tabu(k)    => project(candidates, exclude_recent(k))
    anneal(t)  => zoom(candidates, boltzmann(t))
    ucb(c)     => zoom(candidates, ucb_select(c))
  }
}

-- settle: settle candidates to their spectral coordinate.
-- this is where the eigendecomposition + Laplacian + Fiedler
-- computation happens; the result IS the au coordinate.
settle(candidates: [resolution]) -> au {
  settle(candidates, eigendecompose, fiedler_of, eigengap_of)
}
```

Each rule maps to a documented game-theoretic strategy with documented
convergence properties. The substrate's commitment to those properties
rides on the spectral floor's correctness; the floor is Rust, but the
claim is *named in substrate*.

| Rule | Game-theoretic name | Convergence property | Cite |
|------|---------------------|----------------------|------|
| `greedy` | best-response (no inertia) | converges on potential games | Monderer & Shapley 1996 |
| `beam(k)` | bounded exploration | k-best regret bound | Standard beam search lit |
| `elite(k)` | (μ+λ)-style preservation | preserves dominant trajectories | Evolutionary algorithms canon |
| `halving(k)` | successive halving | log(n)/log(k) rounds to top-1 | Karnin et al. 2013 |
| `tabu(k)` | tabu search | cycle-avoidance | Glover 1986 |
| `anneal(t)` | simulated annealing | converges if cooling ∝ 1/log(t) | Hajek 1988 |
| `ucb(c)` | upper confidence bound | no-regret per Hart & Mas-Colell | Auer et al. 2002 |

The `fold` IS consensus dynamics; each rule application is one step of
spectral settlement. This is the §10 *non-reactive* property of Spectral
Settlement Strategy concretised: the tournament doesn't "respond to"
candidates by selecting one; it integrates the candidate set into the
eigensystem; the settled state IS the response.

**The loss IS the spectral action (corpus grounding).** This spec's
loss — "the composite of `@epistemologic/properties` at the relevant
altitude" (§10.5, inherited from `parse-as-fate-tournament` §3.1) — is
realised concretely as a **spectral-action difference** (cite
`~/.reed/practice/insights/spectral-db/dirac-operator-on-graphs.md`
§5, Chamseddine–Connes 1996):

```
loss(transformation) = Tr(f(D_before/Λ)) − Tr(f(D_after/Λ))
```

where `D = d + d*` is the graph Dirac operator and `f` is a cutoff
function at scale `Λ`. The spectral action `Tr(f(D/Λ))` IS the
information content of the graph at scale `Λ`; a transformation that
adds structure raises it, one that removes structure lowers it. **This
replaces `ShannonLoss` principledly** — not a hand-computed entropy
axis bolted on beside the property verdicts, but the same quantity the
spectral triple already generates (`Tr(ρ ln ρ)` for `ρ = D²|₀/Tr(D²|₀)`
IS the BGS / von Neumann entropy). The `depth: eigenvalue` coordinate
of `au` is where this difference bottoms out; the loss table above
ranks trajectories by it. So `argmin_loss` / `top_k_loss` /
`halve_by_loss` are ranking by spectral-action difference, computed
from the same `D` whose `dsyev` eigensystem the kernel already
produces (see `numerical-substrate-via-fortran` §1.5).

### 3.2 The five-op posture per rule

Not decoration. The five-op vocabulary maps to the substrate's
commitments per `CLAUDE.md`'s *Everything is a Prism*. See §7 for the
completeness claim. Briefly:

- **`greedy`** uses `focus` — pick the dominant trajectory; the lowest-loss
  eigenmode dominates.
- **`beam(k) / elite(k) / halving(k) / tabu(k)`** use `project` — filter
  the candidate set down to its top-k or its tabu-allowed subset; this
  is projection onto a low-loss subspace.
- **`anneal(t) / ucb(c)`** use `zoom` — change the scale of evaluation
  (temperature; exploration coefficient); this is the consensus operator,
  reweighting which eigenmodes are emphasized.
- **`settle`** uses `settle` — settle to the eigenvalue minimum; the
  fixed point IS the coordinate.
- **`split`** appears in `apply_rule`'s rule-dispatch and in candidate
  partitioning by Fiedler vector at higher tournament altitudes (§7).

The symmetry isn't aesthetic — it's a structural commitment. The
five-op symmetry that makes Prism Prism is what makes the tournament
body complete.

---

## 4. Required substrate primitives

For §3's body to be writable in substrate, the substrate needs the
following collection / numerical / spectral primitives. Each is listed
with its current status and what it needs to land. "Trivial" means a
one-line declaration; "small" means a focused `.mirror` file; "medium"
means a multi-action grammar with new spectral-floor bindings.

| Primitive | Status today | What it needs | Effort |
|-----------|--------------|---------------|--------|
| `[t].fold(initial, fn)` | needs verification — `boot/std/list.mirror` declares `list(a) = empty \| cons(a, list(a))` but no `fold` action surfaced in audit | Add `fold(list, b, fn) -> b` to `boot/std/list.mirror`; one substrate action, body `\` (delegates to the substrate's evaluator) | trivial |
| `[t].argmin(scoring_fn)` | likely missing | Add to `boot/std/list.mirror`; one action; body `\` | trivial |
| `[t].top_k(k, scoring_fn)` | likely missing | Add to `boot/std/list.mirror`; one action; body `\` | trivial |
| `[t].halve(k, scoring_fn)` | likely missing | Add to `boot/std/list.mirror`; one action; body `\` | trivial |
| `[t].exclude_recent(k)` | tabu-specific; likely missing | Add to `boot/std/list.mirror` or a tournament-local helper; one action | trivial |
| `boltzmann(t)` sampling | likely missing | New `boot/std/fate/sampling.mirror`; one action invoking RNG floor | small |
| `ucb_select(c)` sampling | likely missing | Same file as `boltzmann`; one action | small |
| `eigendecompose([f64])` | spectral.rs has `eigen_d<N>` (line 982) but it's const-generic over array size; needs a substrate-callable wrapper that handles dynamic sizes | New `boot/std/fate/spectral/eigen.mirror`; declaration over a new `pub` fn in `spectral.rs` (a `Vec<Vec<f64>>` wrapper around `eigen_d`) | medium |
| `laplacian_of(conductivity)` | not present in spectral.rs (audit confirms) | New `pub fn laplacian_of(graph: &Graph) -> Vec<Vec<f64>>` in `spectral.rs`; substrate declaration in `boot/std/fate/spectral/laplacian.mirror` | medium |
| `fiedler_of([[f64]])` | not present in spectral.rs (audit confirms) | New `pub fn fiedler_of(matrix: &[Vec<f64>]) -> f64` in `spectral.rs`; substrate declaration in `boot/std/fate/spectral/fiedler.mirror` | medium |
| `eigengap_of([[f64]])` | not present (audit confirms) | New `pub fn eigengap_of(matrix: &[Vec<f64>]) -> f64`; substrate declaration in `boot/std/fate/spectral/eigengap.mirror` | medium |
| `settle_to_minimum(initial, gradient_fn)` | derived from `eigendecompose + fiedler + eigengap`; needs no new primitive once those exist | One substrate action composing the three above | trivial |

The collection primitives (`fold / argmin / top_k / halve / exclude_recent`)
are canonical functional-collection operations and should land in
`boot/std/list.mirror` (a single small commit). The sampling primitives
(`boltzmann`, `ucb_select`) are randomness-bearing and need a substrate-
level RNG action; they land in their own file.

The spectral primitives (`eigendecompose / laplacian_of / fiedler_of /
eigengap_of`) are the substrate-pull-reflex floor — they cannot be
described in the meta-grammar without first having `f64`-arithmetic
actions, which the substrate doesn't have today and won't on the v1
horizon. They must be Rust. **What this spec adds on the Rust side**
is minimal: thin `pub` wrappers around existing internal machinery,
or (where the internal machinery is `eigen_d<N>`'s const-generic form)
a dynamic-size wrapper that boxes the array into a `Vec`. The actual
spectral mathematics is already there.

**Verification of "already there."** §5 inventories the spectral.rs
surface in detail. Headline finding: `eigen_d` is `pub` and works for
static-size matrices. `laplacian_of / fiedler_of / eigengap_of` are
not present today — the leapfrog needs them added. This is the only
Rust addition the spec calls for; everything else is substrate.

---

## 5. Spectral floor exposure (the substrate-pull-reflex boundary)

`bootstrap/src/spectral.rs` is **4338 lines** (`wc -l` verified). The
file contains the parser combinator runner (Combinator enum + walker),
the AST fold infrastructure (`Fold5 / Fold5At / fold1`), the
spectral primitives (`Spectrum<N> / eigen_d<N>`), and the content-OID
machinery (`ContentOidPrism / compute_content_oid`). For substrate-
native Fate, the spectral subset is what matters.

### 5.1 What's `pub` today

From the spectral subset only (combinator + AST + OID surfaces are
in scope for the parser side, not for this spec):

```rust
// line ~957
pub struct Spectrum<const N: usize> {
    pub eigenvalues: [f64; N],      // descending magnitude
    pub eigenvectors: [[f64; N]; N], // column i = eigenvec i
}

// line ~979
pub fn eigen_d<const N: usize>(matrix: [[f64; N]; N]) -> Spectrum<N>;
//   power iteration with deflation
//   MAX_ITERS = 200, TOL = 1e-12
//   well-separated eigenvalues by coincidence-hash construction
```

The `pub fn` surface for spectral primitives is **exactly one function**:
`eigen_d`. Everything else needed (Laplacian, Fiedler, eigengap) is not
on the public surface today. (Combinator/AST/OID `pub` surfaces are
listed in §5.3 for completeness but are out of scope for this spec.)

### 5.2 What substrate needs invoked

For the tournament body in §3:

| Function needed | Substrate declaration | Rust addition |
|-----------------|----------------------|---------------|
| `eigendecompose(matrix: [[f64]]) -> ([f64], [[f64]])` | `boot/std/fate/spectral/eigen.mirror` declares `action eigendecompose(matrix) -> (eigenvalues, eigenvectors) { \ }` | Add `pub fn eigen_d_dyn(matrix: &[Vec<f64>]) -> (Vec<f64>, Vec<Vec<f64>>)` wrapping `eigen_d<N>` with dynamic sizing (boxes into const-generic, runs, unboxes) |
| `laplacian_of(conductivity: [[f64]]) -> [[f64]]` | `boot/std/fate/spectral/laplacian.mirror` declares `action laplacian_of(graph) -> matrix { \ }` | Add `pub fn laplacian_of(graph: &[Vec<f64>]) -> Vec<Vec<f64>>` computing `D - A` (degree matrix minus adjacency) |
| `fiedler_of(matrix: [[f64]]) -> f64` | `boot/std/fate/spectral/fiedler.mirror` declares `action fiedler_of(matrix) -> precision { \ }` | Add `pub fn fiedler_of(matrix: &[Vec<f64>]) -> f64` returning the second-smallest eigenvalue |
| `eigengap_of(matrix: [[f64]]) -> f64` | `boot/std/fate/spectral/eigengap.mirror` declares `action eigengap_of(matrix) -> precision { \ }` | Add `pub fn eigengap_of(matrix: &[Vec<f64>]) -> f64` returning `1.0 - (lambda_2 / lambda_max)` |

**Total Rust additions:** four `pub` functions, each ~20–40 lines of Rust
(thin wrappers over `eigen_d`-style math). One new spectral subfolder
(`bootstrap/src/spectral/` or inline in `spectral.rs`); the existing
4338-line spectral.rs absorbs them if §5.4 holds.

**Total substrate additions:** four `.mirror` files under
`boot/std/fate/spectral/`, each ~10–20 lines (action declaration + `\`
body + property annotations). Total ~50–80 lines of substrate; ~80–160
lines of Rust.

Net: the substrate-pull-reflex floor for §3's body to run is small.
This is the substrate-pull discipline at the spectral altitude.

### 5.3 What's pub today (full inventory, for completeness)

The spectral.rs `pub` surface across the full file (line numbers
approximate; verified via search). The combinator / AST subsections are
listed for completeness — they are the parser-side surface, not part of
this spec's spectral-floor scope.

```rust
// types
pub type Seed<S>;            // line ~93
pub type Verdict<S>;         // line ~101

// content-OID
pub struct ContentOidPrism;  // line ~175
pub fn compute_content_oid(node: &AstNode) -> String;  // line ~136

// fold infrastructure
pub struct Fold5<...>;       // line ~331
pub struct Fold5At<...>;     // line ~460
pub fn fold1<F, Out>(...);   // line ~917
pub fn compose_a<...>(...);  // line ~944
pub fn render_ast(...);      // line ~555
pub fn seed<S>(state: S);    // line ~107

// spectral (the in-scope subset)
pub struct Spectrum<const N: usize>;  // line ~957
pub fn eigen_d<const N: usize>(...);  // line ~979

// combinator (parser-side; out of scope here)
pub enum Combinator;         // line ~1097
pub enum CharsetKind;        // line ~1170
pub fn literal_kind(...);    // line ~1188
pub fn combinator_tree_oid(...); // line ~1280
pub fn op_keyword_choice();  // line ~1926
pub fn prism_seed();         // line ~1977
pub fn normalize_phase1(...); // line ~2073
pub fn normalize_phase2(...); // line ~2233
pub fn normalize(...);       // line ~2310
```

**Headline:** the spectral primitive surface for tournament work is
*sparse*. `eigen_d` is the only spectral function `pub` today.
`fiedler_of`, `laplacian_of`, `eigengap_of` would all be additions.
The leapfrog is therefore mildly blocked on Rust exposure (~80–160
lines, all thin wrappers), not on substrate declaration.

This matters for §9's step ordering: Tick 2 (substrate declares actions
over spectral primitives) requires Tick 1.5 (Rust exposes the
primitives) to land first. If Alex chooses to keep the leapfrog
minimal, the substrate can declare *only* `eigendecompose` (which
`eigen_d` already supports) and derive Fiedler / Laplacian / eigengap
in substrate by composition. This is medium-effort substrate work and
avoids the Rust additions entirely. See §10 for the open decision.

### 5.4 Substrate-pull-reflex floor — what stays Rust

Eigendecomposition, Laplacian construction, Fiedler computation,
and eigengap derivation are NOT substrate-declarable on the v1
horizon. The meta-grammar doesn't have `f64`-arithmetic actions,
matrix-multiplication primitives, or convergence-bounded iteration
(power-iteration with deflation). These must be Rust until the
substrate gains those capabilities (likely v2+, possibly never if
the spectral floor proves stable enough to leave alone).

The substrate-pull discipline is *not violated* by these living in
Rust. The discipline says: *mechanism the substrate can express
belongs in substrate*. These are mechanism the substrate cannot
express; they are floor, not feature.

The substrate CAN declare ACTIONS that invoke them. That is the
correct shape. The action declaration carries the substrate's
commitment that this operation exists at this signature with this
verdict; the body is `\` (parked, delegated to the Rust floor). When
the substrate evaluator encounters the `\` body, it dispatches to
the Rust implementation. This is the same shape `boot/std/git.mirror`
uses for git operations: substrate declares `compute_oid`,
`store_blob`, `cat_file`; bodies are `\`; Rust runs them.

---

## 6. The dispatcher question

How does `@fate.tournament(rules, candidates)` get invoked from a parse
situation today? `bootstrap/src/pipeline.rs` is **463 lines** (`wc -l`
verified). Audit:

### 6.1 What pipeline.rs does today

- `parse_rewrite(query)` — recognises `<sym> => <repl>` queries.
- `apply_rewrites(rules, source)` — applies rewrites to a byte source.
- `split_pipeline(query)` — splits a mq-query on `|>` into segments.
- `execute_pipeline(segs, source)` — dispatches each segment.
- `is_mq_query(arg)` — recognises whether a CLI arg is an mq query.

The dispatch loop in `execute_pipeline` matches on segment kind (rewrite
vs grammar-ref vs literal) and routes accordingly. Grammar refs invoke
grammar actions; the dispatch path is hardcoded per grammar today
(`@code/llvm/ir`, `@mirror/kintsugi`, etc., have explicit branches).

### 6.2 What dispatch path `@fate.tournament` would use

Not present today. The audit of `bootstrap/src/main.rs` confirms: no
`@fate` dispatch path exists. The `kintsugi_tick` scaffold names what
the path WOULD be — stage 1's comment says *"Fate's five models fan out
and return au candidates. No-op scaffold: zero candidates."* The seam
is named; the code through the seam is not.

### 6.3 What the minimum Rust addition is

For the substrate-declared tournament body to be invoked end-to-end,
the pipeline dispatcher needs to recognise grammar-action invocations
in general — not just hardcoded references. This is the **general
action-dispatch path**, which Cluster D of `road-to-1.0.md` already
names as load-bearing for `mirror run` / `mirror fate`. The path is:

1. Parse the mq-query segment as a grammar-ref + action-call.
2. Resolve the grammar from `boot/`.
3. Look up the action by name.
4. Walk the action's body (substrate-declared expression tree).
5. For each `\` body encountered, dispatch to the Rust floor.
6. Return the action's result.

Steps 1–3 mostly exist (grammar harvester, action enumeration). Step 4
is the substrate evaluator — partially implemented; needs hardening to
handle `fold`, `match`, and spectral-action invocation. Step 5 is the
floor-dispatch table — needs extension for the four spectral additions
in §5.2. Step 6 returns a typed `au` value back through the pipeline.

**Estimated dispatcher work:** moderate. The pieces exist; the
integration tightens. ~100–200 lines of Rust in `pipeline.rs` and
`main.rs` to close the dispatcher seam. Plus the four `pub` spectral
functions from §5.2.

**Stop-and-report check:** the substrate-declared action-dispatch path
does NOT currently route to user-declared grammar actions. The audit
confirms hardcoded dispatch only. The leapfrog is therefore blocked on
dispatcher work — Tick 0.5 in §9 — before substrate declarations can
run end-to-end. This is the correct shape: the substrate-pull-reflex
floor includes both spectral primitives AND the dispatcher that routes
to them. Both must land before §9's Tick 3 can fire.

### 6.4 What this is NOT

The dispatcher work is **not new capability** — it's substrate-pull-realize.
The bootstrap already claims to dispatch grammar actions (via grammar
harvester, action enumeration, and the kintsugi scaffold's named seam).
Making the dispatcher *honest* about that claim is bugfix-class work,
not feature-class. It can land under `[bugfix:restore]` per `AGENTS.md`'s
exception clause.

---

## 7. The five-op completeness claim

The five-op vocabulary (`focus / project / split / shift / settle`) is
sufficient for the tournament body. This is not decoration; it's the
complete set of spectral operations needed to run a tournament. The
claim connects to `CLAUDE.md`'s *Everything is a Prism* commitment.

The mapping:

- **`focus`** — picks the dominant eigenmode. The tournament's `greedy`
  rule IS `focus`: select the trajectory whose loss is smallest;
  structurally, project onto the lowest-eigenvalue eigenvector. The
  five-op `focus` is precisely this operation at the spectral altitude.

- **`project`** — projects out high-eigenvalue modes (the noise) and
  retains low-eigenvalue modes (the signal). The tournament's `beam(k)`,
  `elite(k)`, `halving(k)`, and `tabu(k)` rules are all `project`:
  filter the candidate set down to its top-k or constraint-allowed
  subset; structurally, project onto the subspace of low-loss
  eigenmodes (or, for tabu, project out cycles in the trajectory
  history). The five-op `project` covers all four.

- **`split`** — partitions by Fiedler vector. The tournament uses
  `split` at two altitudes: (1) `apply_rule`'s rule-dispatch (split the
  rule sum into branches; deterministic, not spectral), and (2) at
  hierarchical tournament altitudes, partitioning the candidate set by
  the Fiedler vector of the conductivity Laplacian — the natural
  coalition boundary per game-theory §6 (*"Coalition formation as
  spectral clustering. Agents with similar eigenvalue profiles form
  natural clusters in the spectral embedding. These clusters ARE
  coalitions. The Fiedler vector partitions agents into groups with
  strong internal cohesion and weak external coupling."*).

- **`zoom`** — the consensus operator (eigendecomposition / scale
  change). The tournament's `anneal(t)` and `ucb(c)` rules ARE `zoom`:
  reweight which eigenmodes are emphasized (annealing decreases
  temperature, sharpening focus on the dominant mode; UCB increases
  exploration weight on under-tried trajectories). The five-op `zoom`
  is precisely the scale-change operation at the spectral altitude.

- **`settle`** — settles to the eigenvalue minimum (the fixed point).
  The tournament's `settle` step IS `settle`: drive the candidate set
  to its loss minimum; the settled state IS the `au` coordinate. The
  five-op `settle` is precisely this operation at the spectral
  altitude. The `eⁿ⁺¹ < eⁿ` proof from `CLAUDE.md` rides on `settle`'s
  monotone-non-increasing loss guarantee.

No tournament mechanism requires an operation outside the five. Adding
a sixth op would be the substrate-pull-warning the §6 dispatcher seam
rules against. The five-op symmetry is the substrate's commitment that
the tournament can be expressed structurally; the substrate cannot be
asked to add operations to make new mechanisms work — new mechanisms
must decompose into the five.

**Connection to `CLAUDE.md`'s claim.** *"Everything is a Prism. Every
command runs one or more of: focus, project, split, shift, settle.
These are not metaphors. They are the trait methods."* The tournament
is a command; it runs all five. The completeness is exhibited, not
asserted.

---

## 8. The five ganglia as candidate proposers

`boot/std/fate/tournament.mirror`'s `candidates(hole)` action calls five
ganglia:

```mirror
candidates(hole) -> [resolution] {
  @ai/abyss.depth(hole)
  @ai/introject.pattern(hole)
  @ai/cartographer.map(hole)
  @ai/explorer.explore(hole)
  @ai/fate.select(hole)
}
```

All five ganglion bodies are `\` today. Each ganglion is a 90-neuron
subgraph of the 450-node connectome (`boot/std/fate/connectome.mirror`).
The SHAPE of what each ganglion contributes at parse altitude — not the
implementations — is in scope for this spec. (Implementations are a
separate substrate-design surface, larger than this spec; see §11.)

### 8.1 Sketch at parse altitude

The five ganglia map to the five Prism operations per
`eigenboard-representation.md` (*"Fate↔operation mapping (corrected)"*).
At parse altitude, each ganglion contributes a parse-trajectory proposal
from its operation's posture:

| Ganglion | Operation | Parse-altitude proposal sketch |
|----------|-----------|-------------------------------|
| `@ai/abyss.depth(hole)` | focus | depth-first parse trajectory; commit to the dominant interpretation early; useful when grammar structure is unambiguous |
| `@ai/introject.pattern(hole)` | project | pattern-matching parse from corpus; project onto known parse shapes; useful when the fragment matches a high-frequency pattern |
| `@ai/cartographer.map(hole)` | split | topological alternative; partition the candidate space by structural features; useful when the parse forest branches widely |
| `@ai/explorer.explore(hole)` | shift | breadth-first alternative; sample candidates across the parse forest; useful when no single trajectory dominates |
| `@ai/fate.select(hole)` | settle | meta-selected alternative; settle candidates through the eigenboard's gauge slice; useful when other ganglia produce conflicting proposals |

The shapes are *postures*, not algorithms. The actual proposals at each
ganglion are computed by the ganglion's substrate (its 90-neuron
subgraph); the shape of the proposal is constrained by the
operation's posture.

**Verification.** This mapping matches `eigenboard-representation.md`'s
*Fate↔operation mapping (corrected)* section (lines ~174–227); the
ground truth is there. The parse-altitude application is this spec's
contribution; it specialises the cross-altitude mapping to parse work.

### 8.2 What each ganglion needs to know about the parse altitude

Each ganglion's `(hole) -> [resolution]` body, at parse altitude,
needs:

1. **The fragment being parsed.** A byte slice (or a partial token
   stream).
2. **The active glass set.** The substrate-declared shapes available
   as alternatives at this parse state.
3. **The conductivity context.** The substrate hierarchy distance
   between candidate trajectories (the v0 prior from
   `parse-as-fate-tournament` §3.2).
4. **The tournament's current state.** Which trajectories have been
   eliminated; which remain alive; the current loss-vector profile.

These inputs are the parse-altitude *features vector* that each
ganglion's inference reads. They match the cross-altitude `features`
type declared in `boot/std/fate.mirror:13`. The substrate-pull is
minimal: the parse altitude reuses the cross-altitude feature shape;
the ganglia don't need parse-specific input types.

### 8.3 What is NOT in scope

The IMPLEMENTATIONS of the five ganglia at parse altitude — what each
ganglion's 90-neuron subgraph actually computes — are out of scope.
That is a separate substrate-design surface, larger than this spec
and requiring engagement with `@fate/connectome`'s structural
commitments. This spec names the *SHAPE* of what each ganglion
contributes; the substrate that fills those shapes is later work.

For the leapfrog in §9, the ganglia can land with bodies that return
a single candidate constructed from a heuristic prior (e.g., abyss
returns the leftmost trajectory; explorer returns the rightmost;
cartographer returns the structurally-largest; introject returns the
most-frequent; fate returns the substrate-hierarchy-closest). These
are scaffolding bodies — sufficient to exercise the tournament
end-to-end, insufficient to satisfy the substrate's eventual
commitments. The scaffolds get replaced as the ganglion substrate
design solidifies.

---

## 9. Minimum-viable leapfrog sequence

Refined from prior sessions, now that the substrate-pull discipline is
corrected and the spectral-floor exposure surface is mapped. Each tick
is a one-commit unit; each is independently verifiable.

| Tick | Scope | Substrate or Rust | Dependency |
|------|-------|-------------------|------------|
| 0.5 | Honest dispatcher — substrate-pull-realize the grammar-action dispatch path in `pipeline.rs` so that user-declared grammar actions invoke through the standard path, not via hardcoded branches | Mostly Rust (~100–200 lines, `[bugfix:restore]` per AGENTS.md exception); minimal substrate | Required by all subsequent ticks; closes §6 gap |
| 1 | Declare `type au` + sub-coordinates as newtypes in `boot/std/fate/au.mirror` | Pure substrate (~30–50 lines of mirror) | Independent; can land in parallel with 0.5 |
| 1.5 | Add four `pub` spectral functions to `bootstrap/src/spectral.rs`: `eigen_d_dyn`, `laplacian_of`, `fiedler_of`, `eigengap_of` (each ~20–40 lines Rust, all thin wrappers over existing math) | Pure Rust (`[bugfix:restore]` if framed as *"the spec said spectral primitives are callable from substrate; restoring that capability"*) | Required by Tick 2; can land in parallel with Tick 1 |
| 2 | Declare spectral actions over the new `pub` surface in `boot/std/fate/spectral/{eigen,laplacian,fiedler,eigengap}.mirror` | Pure substrate (~40–80 lines across four files) | Requires Tick 1.5 |
| 2.5 | Declare missing collection primitives (`fold / argmin / top_k / halve / exclude_recent`) in `boot/std/list.mirror`; declare sampling primitives (`boltzmann / ucb_select`) in `boot/std/fate/sampling.mirror` | Pure substrate (~30–50 lines) | Required by Tick 3 |
| 3 | Write tournament body in substrate (~20–30 lines) in `boot/std/fate/tournament.mirror`; replace `\` with the §3 fold | Pure substrate | Requires Ticks 1, 2, 2.5 |
| 4 | First end-to-end run: action-decl ambiguity tournament against a substrate-declared `glass lambda = abstract(...) { ... }` (per `parse-as-fate-tournament` §8 Step 3) | Pure substrate (assuming Tick 0.5 closed the dispatcher) | Requires Ticks 0.5, 1, 2, 2.5, 3 |

**Effort estimate:**

- Tick 0.5: medium (~1–2 sessions; dispatcher work, integration testing).
- Tick 1: small (~30 minutes; declaration only).
- Tick 1.5: small (~1–2 hours; thin wrappers).
- Tick 2: small (~1 hour; substrate declarations).
- Tick 2.5: small (~1 hour; substrate declarations).
- Tick 3: small (~1–2 hours; body shape sketched, fits in ~30 lines).
- Tick 4: medium (~1 session; end-to-end exercise, debugging).

**End state:** substrate-native Fate runs first parse tournament against
a substrate-declared `glass`. The Rust addition is ~150–250 lines across
Ticks 0.5 and 1.5; the substrate addition is ~150–250 lines across
Ticks 1, 2, 2.5, 3.

Net: ~300–500 lines of work (half substrate, half Rust) to unlock the
engine collapse named in `parse-as-fate-tournament` §4. The Rust
portion is bugfix-class (substrate-pull-realize, not feature-add); the
substrate portion is fresh declaration. The substrate-pull discipline
holds throughout.

**Substrate-pull-realize markers.** Commits in this sequence that
restore substrate-pull-already-claimed capability use `[substrate-pull:realize]`
per `AGENTS.md`. Tick 0.5 specifically — the dispatcher work —
restores capability the substrate has been declaring (via `\` bodies in
tournament / candidates / each ganglion) for which the Rust evaluator
has been silent. That is the canonical shape of substrate-pull-realize.

---

## 10. Open decisions to surface

Things that need Alex's eyes before any tick fires. The spec resolves
what's clearly resolved and surfaces the rest.

### 10.1 Exact syntax for newtype declarations in mirror — likely RESOLVED

The canonical pattern is `type precision(f64)` at `boot/01-meta.mirror:28`.
This matches `type oid(ref)` at `boot/std/git.mirror:5`, `type loss(precision)`
at `boot/std/beam.mirror:25`, `type nl(text)` at `boot/std/nl.mirror:3`,
`type token(text)` at `boot/std/nl/english.mirror:18`, and many others.
The spec's §2 declarations follow this pattern exactly.

What needs confirmation: is `type eigenvector([precision])` (newtype over
list-of-newtype) supported? Audit shows `type loss(precision)` works
(newtype over newtype) and `[ref]` collection syntax works in fields
(`path: [prism]` etc.). No example of `type name([newtype])` surfaced —
but the pattern composes from supported pieces. Likely supported;
confirm at Tick 1.

### 10.2 Where `au` lives — open

Two candidates:

- `boot/std/au.mirror` — top-level; `au` as a substrate-wide coordinate
  type, not Fate-specific. Reflects the fact that au values flow through
  many altitudes (parse, type-check, dispatch, merge), not just Fate.
- `boot/std/fate/au.mirror` — under Fate; `au` as Fate's output
  coordinate. Reflects the fact that au is constructed BY Fate
  tournaments, and the spectral coordinate structure is what makes it
  Fate-typed.

Reed's lean: `boot/std/fate/au.mirror`. The construction site is Fate;
the consumers are many. Filename naming follows the construction site,
not the consumption surface — same as `boot/std/git.mirror` (constructed
in git; consumed everywhere).

Open.

### 10.3 Sub-coordinate placement — open

`eigenvector`, `eigenvalue`, `fiedler`, `eigengap` are sub-coordinates of
`au`. Two placements:

- Inline in `boot/std/fate/au.mirror` — single file; all four declared
  together; consumers `in @fate/au` once and have all five types.
- Separate `boot/std/fate/spectral/{eigenvector,eigenvalue,fiedler,eigengap}.mirror`
  — each in its own file under a `spectral/` shelf; consumers `in
  @fate/spectral/eigenvector` etc. selectively.

Reed's lean: inline. The four sub-coordinates are tightly coupled (they
move together; consumers always want all four when constructing or
destructuring an `au`). Sibling files in `spectral/` would scatter the
coordinate structure across four files for no consumer benefit.

AGENTS.md *No `_<extension>` Filename Suffixes* (lines 320–340)
reinforces inline placement: the directory structure carries the kind,
and splitting four tightly-coupled coordinates into four files would
be substituting filename multiplicity for type coupling — the inverse
of the path-IS-substrate-structure principle.

Open.

### 10.4 Whether each ganglion gets parse-altitude-specific actions or generic ones reused — open

From §8: each ganglion's `(hole) -> [resolution]` body could be:

- **Parse-specific** — `@ai/abyss.depth_at_parse(hole) -> [resolution]`,
  with a separate action per altitude. Strong typing; each altitude
  declares its own contract; bodies specialise.
- **Generic** — `@ai/abyss.depth(hole) -> [resolution]`, with one
  action per ganglion that handles whatever altitude the `hole` carries
  (the `hole` type itself encodes altitude). One body; routes by hole
  type.

Reed's lean: generic. The substrate-pull discipline says: capability
lives in substrate; the substrate's hole type already carries altitude
(it's a typed pointer into the eigenboard's section). Forcing per-altitude
action families would be the OOP-trained reflex `AGENTS.md` *Keywords
Are Substrate Declarations* warns against.

Open.

### 10.5 How the tournament invokes the loss function — RESOLVED via parse-as-fate-tournament §3.1

The loss IS the composite of `@epistemologic/properties` at the relevant
altitude (`parse-as-fate-tournament` §3.1 mechanism, RESOLVED there).
Mechanism: `reflect(ast) -> [verdict]` returns the verdict vector;
`pass / partial / fail` map to per-axis contributions; trajectory
domination is component-wise on the vector.

This spec inherits the resolution. The tournament body's `argmin_loss`,
`top_k_loss`, `halve_by_loss`, and `boltzmann(t)` all invoke the
property-composite loss via a substrate action — likely
`@fate.loss(candidate) -> [precision]` declared in `boot/std/fate.mirror`
(or a new `boot/std/fate/loss.mirror`), with body composing
`@epistemologic/property/reflect(ast)` into a verdict vector.

The concrete numerical realisation of that composite is the
spectral-action difference `Tr(f(D_before/Λ)) − Tr(f(D_after/Λ))` (§3.1,
cite `dirac-operator-on-graphs.md` §5). The property verdicts give the
per-axis *shape* of the loss vector; the spectral action gives the
*scale-aware magnitude* on the structural axis. They compose; the
spectral action is not a competing axis but the principled successor
to the `ShannonLoss` the design otherwise leaves un-grounded.

What needs confirmation: that the property reflect mechanism returns
verdicts at parse altitude in the shape the tournament body expects.
The parse-altitude properties don't exist yet
(`parse-as-fate-tournament` §3.1 names this as the only remaining
substrate work the loss function needs). The tournament body's
dependence on this is therefore: it works as soon as parse-altitude
properties land.

### 10.6 Whether Tick 1.5 is needed at all — open

As noted in §5.3, the substrate can declare *only* `eigendecompose`
(which `eigen_d` already supports via a dynamic-size wrapper) and derive
Fiedler / Laplacian / eigengap in substrate by composition. Pros: no
Rust additions beyond Tick 0.5; substrate-pull discipline at its purest.
Cons: substrate-side composition of these primitives is more substrate
work (medium, not small) and may surface meta-grammar gaps that drive
the leapfrog into a yak-shave on `f64`-arithmetic actions.

Reed's lean: keep Tick 1.5. The Rust additions are thin wrappers (~80–160
lines total), the substrate-side composition can land later as a
*purification* tick once the leapfrog has end-to-end verification, and
the risk of yak-shaving the substrate's `f64` machinery while trying to
land the tournament is structural — better to land it through Rust first
and purify later.

Open.

---

## 11. What this spec doesn't cover

The spec's boundary matters. Out of scope:

- **Ganglion implementations.** §8 sketches what each ganglion proposes
  at parse altitude. The actual 90-neuron subgraph design — what
  `@ai/abyss` computes structurally; how `@ai/explorer`'s breadth-first
  search routes through its substrate; how `@ai/cartographer` builds
  its topological alternative — is a separate substrate-design surface,
  larger than this spec. Scaffold bodies (heuristic-prior single
  candidate) are sufficient for the §9 leapfrog; replacement bodies
  land as the ganglion substrate design solidifies.

- **`@fate.minimize` — the gap-tensor solver.** Different altitude.
  See `docs/specs/gap-tension-tensor-substrate.md`. The minimize
  operation runs at the meta-altitude of *picking which tournament to
  run*; the tournament body itself is what this spec covers. Both
  consume the same spectral floor; the dispatcher routes between
  them.

- **Mycelial inference.** The naming is in
  `docs/specs/eigenboard-representation.md` *Mycelial substrate (new
  section)* (lines 529–622); not yet substrate. When it lands, the
  tournament's conductivity tensor gains a mycelial-prior input;
  spec-level work is downstream of this one.

- **Curvature-detected residual conductivity.** v2 of the
  conductivity tensor per `parse-as-fate-tournament` §3.2; deferred.
  Pure-learned is forbidden by the Hajek stationarity precondition
  (cited there); v0 is what lands at Step 8 of that spec; v2 is
  later work.

- **Scheduler Tower coordination.** Phase 5 #108 names the tower's
  coordination rules — which tournament runs when. The parser is one
  consumer; this spec does not declare how the tower routes between
  the parser, the kintsugi merge resolver, and other consumers.
  `docs/specs/scheduler-tower.md` (Reed, 2026-05-25) is the load-bearing
  reference.

- **Reflection's prior-adjustment algorithm.** The gestalt entry
  shape (`parse-as-fate-tournament` §6) names what Reflection
  observes. How it updates priors from those observations is a Phase 5
  design surface.

- **Two-player + N-player generalisations.** Game-theory §6 names the
  N-player connection (Hodge-Shapley / cooperative game theory).
  This spec describes the single-tournament case; the N-tournament
  case (multi-altitude coordination per the Scheduler Tower) is
  downstream.

- **Zero-sum tournament dynamics.** Game-theory §7 names that
  settlement does NOT exist in pure zero-sum games (saddle, not
  minimum). Parse-altitude tournaments are non-zero-sum by
  construction (both the substrate and the consumer want the same
  outcome — a parseable AST), so this spec doesn't engage with the
  zero-sum case. Downstream consumers that face zero-sum dynamics
  (e.g., adversarial security review) would need a separate
  spectral-saddle-finding spec.

---

## 12. Provenance

*Session-long dialogue between Alex and Reed, 2026-05-26 → 2026-05-27.*
The three structural recognitions — tournament body is substrate, the
conductivity tensor IS a Laplacian, `au` is a spectral coordinate —
landed together. The parser-side complement (`parse-as-fate-tournament`,
commit `28f5973`) describes the architecture from the parser side; this
spec describes it from the Fate side.

No implementation in this commit. The next tick is §9 Tick 0.5 (the
dispatcher work) or — if Alex green-lights the leapfrog as substrate-first
— §9 Tick 1 (the `au` declaration).

The spec captures the architecture. The implementation answers it.

The spec is Mara's, written on `mara/shard-chain` against HEAD `e4a7940`
(the `@epistemologic/property/laws/` subfolder + AGENTS rule against
`_<extension>` naming). Markdown only; no `.rs` files modified; no
`.mirror` files modified. Pre-commit hook passes cleanly (filters `.rs`
additions/modifications; markdown is untouched).
