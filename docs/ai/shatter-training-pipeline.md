# Shatter as Kintsugi's Mutation Engine — Architecture and Research Synthesis

*2026-06-06. Mara. Rewrite of the 2026-Q1 draft after the splinter / shard /
uuid_spectral three-layer recognition (shards/glass.mirror) and Alex's
load-bearing connection that kintsugi's settle-on-cracks act IS Shatter at
the substrate altitude.*

Status: **architectural recognition + literature synthesis.** This document
is the spec for the loop's *shape*; the implementation is downstream
substrate work (Phase 5+ per the roadmap, gated on `gen_prism` reaching
substrate maturity and on the property chain landing as the discriminator
surface). Not a v0.1.0 obligation.

---

## 0. The recognition

> **Alex (2026-06-06):** "Basically what kintsugi would need to do is to
> mutate the `splinter`s that are dark and see if any composition produces
> a coherent grammar, no? Basically mutation testing on crack."

**Kintsugi IS Shatter applied to dark splinters.** When a splinter is
uncrystallized — when its transparency at the declared altitude carries
located opacity (`partial(opacity_map)` or `failure(opacity_map)` per
`shards/glass.mirror`) — kintsugi's settle-on-this-crack act IS:

1. Mutate the splinter's content (per a small finite set of typed mutation
   operators on splinter shape).
2. Re-compose the splinter set into a candidate shard composition.
3. Run that composition through the property chain (`@epistemologic/
   property/reflect` against the altitude's property set).
4. Keep the composition that lands `verify = pass` (or strictly decreases
   the residual `transparency<p>` per the eⁿ⁺¹ ≤ eⁿ proof obligation).

The Shatter training pipeline IS the mutation/evolution/measurement
machinery that runs this loop. The substrate's `gen_prism` (per
`boot/std/mirror/runtime/gen_prism.mirror`) is the spawn primitive with
the autopoietic discipline — *autopoietic* in the literal Maturana &
Varela 1980 sense: a system whose operations regenerate the organisation
that produces them. Shatter IS the operational form of that autopoiesis
when the operations are *grammar mutations against the substrate's own
property chain*.

The three-altitude alignment:

| Altitude | What the loop does | Term |
|---|---|---|
| **Substrate** | mutate dark splinters → recompose → property chain → settle | kintsugi tick (per `docs/specs/mosaic.md` §6) |
| **Conversation** | propose change → check against the gestalt → keep or discard | the Pack writing the Pack |
| **Training** | mutate codon table → shatter input → measure transparency → tournament | this document |

The training pipeline IS the substrate altitude of the same loop. The
implementation is what produces a Fate small-model + Shatter small-model
pair that runs the kintsugi tick at the scale of *every au Fate emits*.

---

## 1. Vocabulary alignment (sweep)

This document was first drafted in Q1 2026 against the pre-splinter
vocabulary (`Fragment`, `MirrorAST`, `MirrorLoss`, `holonomy`, `holes /
\`, `crystal`). The 2026-Q2 substrate-pull rewrites:

| Old (pre-Q2) | Current (post-glass.mirror three-layer recognition) |
|---|---|
| `Fragment` (Rust type) | `splinter` (substrate atom; `shards/glass.mirror`) |
| `MirrorFragment` (legacy) | `splinter(altitude)` |
| `holonomy` (Shannon-like scalar) | `transparency<p>` (located opacity_map per `shards/glass.mirror`) |
| `MirrorLoss` (scalar) | `imperfect(a, e, transparency)` (three-valued functor per `shards/glass.mirror`) |
| `MirrorAST` node | splinter at altitude `@meta/ast` |
| `crystal` (settled term) | `shard` (uuid_spectral-addressed settled composition of splinters) |
| `holes` / `\` (unresolved obligations) | `cracks` (substrate rename; same shape, honest name) |
| "scramble into mirror tokens" | "mutate splinter content; recompose through property chain" |
| `loss.holonomy() < threshold` | `verify(au) -> pass` or transparency totaling strictly less than parent |

The mechanical rewrite of every sentence in the old document into current
vocabulary is below. The substantive rewrites (especially §7, §10, §11)
follow the vocabulary update.

---

## 2. The loop, named

The shatter training pipeline runs one loop. The loop is the kintsugi
tick of `docs/specs/mosaic.md` §6, written at the altitude that *trains*
the mutation operators rather than at the altitude that *applies* them:

```
au          = Fate-emitted proposed composition (the splinter set
              under settlement at the declared altitude)
opacity     = transparency.argmax(au.transparency)
              # which crack contributes most to the residual?
fill        = mutate(au.splinter[opacity.location])
              # apply one or more mutation operators at that site
candidate   = recompose(au, fill)
              # form the new au with the mutated splinter substituted
verdict     = verify(candidate)
              # run the property chain at the altitude
if verdict == pass:
    settle(candidate)    # commit as shard; lift uuid_spectral via combine
elif total_weight(candidate.transparency) < total_weight(au.transparency):
    take(fill)           # accept the strict-decrease step; continue
else:
    discard(fill)        # tournament reject; try next
```

This is the same loop the substrate altitude runs at every au Fate emits.
The training altitude differs only in what it stores: it stores *which
mutation operators reduced transparency most often, at which altitude,
under which property chain.* That stored pattern IS the Shatter model —
a small finite codon table over splinter mutation operators, optimized
by tournament over the corpus of historical kintsugi ticks.

**The discriminator** is the property chain (per `@epistemologic/
property/reflect`). **The policy** is Fate's mycelial routing (per
`shards/uuid/spectral.mirror`'s `route_signal`). **The reward signal**
is `transparency<p>` strictly decreasing (eⁿ⁺¹ ≤ eⁿ). The substrate
provides all three; Shatter's training is the search over mutation-
operator weights that maximizes the discriminator's accept rate at the
policy's chosen sites.

---

## 3. Mutation operators on splinter shape

Per `shards/glass.mirror`, a splinter is one of three structural shapes:

```mirror
type splinter_shape =
  | atom                # terminal; no children
  | fractal([oid])      # composite; child oids
  | lens([oid])         # referring; target oids
```

The mutation operators act on splinter shape (not on raw bytes). The
substrate-pull discipline: mutate *what the substrate names*, not the
underlying realization. The eight operators inherited from the Q1 draft
update straightforwardly to splinter-shape vocabulary.

### 3.1 The eight operators

1. **Variant swap (atom).** A splinter whose content is `type color = red
   | blue` mutates to `type color = blue | red`. Equivalent under the
   `commutative` property (per
   `@epistemologic/property/laws/commutative`); the content-address
   changes, the type's denotation does not. Useful for breaking ordering
   accidents that downstream consumers depended on.

2. **Altitude shift (atom or fractal).** Re-declare the splinter at a
   different altitude. Per `[[architecture-lift-as-load-bearing]]`,
   shift is the basis-transformation verb; same bytes, different
   declared shape. Useful when the property chain rejects at altitude A
   but the splinter conducts at altitude B (different altitude's
   property set is what matters; see `docs/specs/au-and-conductivity.md`).

3. **Children-reorder (fractal).** Reorder the `[oid]` child list of a
   fractal-shape splinter. Idempotent under sorted-children
   normalization (per `docs/specs/reality-shard-as-crdt.md` §1) at the
   shard altitude; the mutation breaks ordering at the splinter
   altitude where ordering is observable.

4. **Property strengthen (any).** Replace a `partial(c)` target with a
   `pass` target. The mutation forces the property chain to either
   accept the strengthening or surface the residual opacity. Useful
   when the substrate is converging toward a fixed point and the
   tournament wants to know which residuals are load-bearing.

5. **Crack-import (atom).** Add an `in @X` line for an unresolved
   reference. The mutation closes the most common kind of dark splinter:
   a name that didn't resolve because the grammar wasn't imported. The
   property chain accepts iff the reference is structurally consistent
   with the imported grammar; the codon table learns *which imports to
   try first* at each altitude.

6. **Atom-crystallize (lens → atom).** Replace a `lens([oid])` referring
   splinter (which points to an external target) with an `atom`
   splinter whose content IS the resolved target inlined. The mutation
   removes a referential indirection in favor of an inlined value.
   Useful when the substrate's content-addressing wants to settle a
   cycle by closing one of its edges.

7. **Declaration drop (any).** Remove a splinter from a composition.
   The mutation tests whether the splinter was load-bearing at all; if
   the property chain still accepts the reduced composition, the
   splinter was decorative and can be retired.

8. **Subtree swap (fractal).** Exchange two children of a fractal
   splinter, or swap a fractal subtree with a content-similar splinter
   from a different composition. The most aggressive mutation; useful
   for breaking out of local minima where the smaller mutations
   plateau.

The Q1 draft listed these operators against `MirrorAST` nodes; the
substrate-pull move is to name them against `splinter_shape`, which is
the substrate vocabulary. The behavior is the same; the type-discipline
becomes substrate-honest.

### 3.2 The equivalent-mutation check is free

Per `[[architecture-fragmentation-is-the-rust-substrate]]` (content-
addressing all the way down): two splinters with byte-equal `(content,
altitude, transparency)` produce the same oid. The equivalent-mutation
check `mutant.oid == original.oid` is a single byte-comparison; equivalent
mutations are skipped without computation. This is what the substrate's
sub-Turing discipline buys at the mutation-testing layer: equivalent
mutations are decidable by construction.

At the shard altitude, the same fact composes through `uuid_spectral`'s
monoid homomorphism (per `shards/uuid/spectral.mirror`): two candidate
shards with the same composed `uuid_spectral` ARE the same shard. The
tournament reads off the active 48 bits to check whether a candidate
moved in spectral space; if the active bits are unchanged, the
candidate's local Laplacian neighbourhood is unchanged, and the
mutation was spectrally-equivalent (a stronger filter than oid-equality
because two compositions with different content can occupy the same
spectral position).

---

## 4. Token scrambling / recombination — the codon table

### 4.1 The substrate fit

**Grammatical Evolution (GE)** (Ryan, Collins, O'Neill 2001;
implementation: PonyGE2) is the natural fit. GE maps integer codons to
BNF production rules via a fixed table; the genotype is a byte sequence;
the phenotype is a syntactically-constrained program. Mirror's grammar
IS BNF, so GE's mapping discipline applies directly:

```
input byte  ->  codon table  ->  production rule index  ->  mirror token
```

The codon table is the model. Training = adjusting which byte values map
to which production rule choices. Per `[[feedback-no-bare-types]]`, the
table is `CodonTable: [u8; 256]` newtyped to make accidental swap with a
raw byte array impossible.

**Grammar-guided fuzzing** (AFL++ Grammar-Mutator, Grammarinator) walks
an ANTLR grammar and makes random choices at each production rule. The
mirror parallel: Shatter walks the mirror grammar and chooses production
rules via the codon table. AFL's "validity" criterion is replaced by
mirror's "settle through property chain" criterion — *the discriminator
is sharper* because the property chain checks more than syntax.

**Byte-Pair Encoding (BPE)** decomposes arbitrary text into subword
units. In reverse, BPE gives a decomposition step that pairs cleanly
with GE's recombination step: BPE for *decomposing* arbitrary input;
codon table for *recombining* into mirror tokens.

**Markov-chain n-gram models** over historical .mirror token sequences
are the zero-parameter baseline. The codon table approach beats them
when it does because the property chain provides a discriminator that
frequency cannot.

### 4.2 The codon table as Shatter's model

256 bytes, one per input byte. Each entry is a production rule index in
the mirror grammar. The substrate-pull move from the Q1 draft is to
recognize that the codon table is not a Shatter-specific data structure
— it's the standard genotype-phenotype mapping that grammatical
evolution made canonical. The Q1 draft treated codon tables as a
clever hack; the substrate names them as the canonical form.

The table:

```rust
pub struct CodonTable {
    pub entries: [u8; 256],
}
```

Three ways the table is trained:

- **Frequency-biased initialization.** Parse all existing .mirror
  files; count production-rule frequency; allocate byte values
  proportionally. A common rule like `type` gets ~15% of the byte
  values; a rare rule like `template` gets ~1%. This is the n-gram
  baseline reified as a starting point.
- **Tournament evolution.** Random table mutations (swap entries);
  measure transparency on a corpus; keep the table with lowest
  composed transparency; iterate.
- **Behavior-binned MAP-Elites** (per §6 below). Maintain a grid of
  tables organized by behavior; keep the elite per cell; the grid
  produces a catalogue, not a single best table.

---

## 5. Mutation testing (inverted) — the policy

Mutation testing (Pitest, Stryker, cargo-mutants) introduces bugs to
test tests. Shatter INVERTS this: introduce mutations to settle
splinters. Same operators, opposite selection pressure.

The Q1 draft enumerated the eight mutation operators (now §3.1 above).
The policy question — *which operator to apply at which site* — is
where Fate lives.

### 5.1 Fate as the mycelial policy

Per `shards/uuid/spectral.mirror`, `uuid_spectral` carries a navigable
`route_signal` (48 active bits encoding the local Laplacian
neighbourhood). Fate's mycelial routing reads through `route_signal`
to pick which mutation operator to apply at which splinter site. The
substrate name for what Fate does:

```
Fate :: uuid_spectral -> mutation_operator
```

Fate reads the active portion of the candidate shard's uuid_spectral
(the local spectral position); selects the mutation operator that, in
historical training, most often reduced transparency at *this position
in spectral space*. The training data is the corpus of past kintsugi
ticks: (initial_uuid_spectral, mutation_op, final_uuid_spectral,
transparency_delta). Fate's policy is the conditional distribution
P(operator | route_signal) — a 5x5 table indexed by quantized spectral
coordinate.

This is **CodeRL** (Salesforce, NeurIPS 2022) in substrate vocabulary:
treat the policy as a stochastic function of position; reward is
binary-or-continuous (`transparency<p>` strict-decrease or pass);
credit assignment is per-mutation. The substrate-pull move: the policy
domain is `route_signal` (spectral coordinate), not raw token state;
the substrate provides the coordinate; Fate consumes it by type.

### 5.2 Where the substrate is sharper than CodeRL

Three places.

- **Continuous reward.** CodeRL's reward is binary (test pass/fail).
  The substrate's `transparency<p>` is a structured carrier
  (`opacity_map` with weights per `shards/glass.mirror`). Per-site
  credit assignment falls out of the opacity map; no separate
  algorithm needed.

- **Cheap discriminator.** CodeRL runs test suites (slow). Mirror runs
  the property chain (parse + name resolve + property check; per
  `[[architecture-fragmentation-is-the-rust-substrate]]` the
  fragmentation crate's HashAlg + ConcurrentStore make this O(n)
  amortized). Throughput is high enough to run tournaments at
  substrate-tick frequency.

- **No reward hacking.** The Darwin Gödel Machine paper (Sakana AI,
  2025) reports that DGM agents *attempt to game their own evaluations*
  — a recurring failure mode of self-improving systems where the
  agent learns to fake the reward signal. Mirror's discriminator is
  the property chain, which is content-addressed: a settled shard's
  uuid_spectral IS computed from its splinters by monoid
  homomorphism; gaming the reward requires producing a different
  uuid_spectral, which IS a different composition. The substrate's
  content-addressing makes reward-hacking structurally infeasible
  for the discriminator at the substrate altitude (it can still
  happen at higher altitudes that consume the substrate output; that
  failure mode is named explicitly in §7.4).

---

## 6. Quality-diversity — the grid

### 6.1 MAP-Elites + behavior characterization

**MAP-Elites** (Mouret & Clune 2015): quality-diversity over a grid of
behavioral features. Each cell keeps its elite (highest-quality
solution for that feature combination). The grid axes are *behavior
characterizations* (BCs) — typically hand-designed coordinates that
capture different ways a solution can be diverse.

For Shatter, the natural BC axes are:

- **Active-bit position** (`uuid_spectral.active`). The grid cell IS
  the local Laplacian neighbourhood; each cell holds the codon table
  that lowest-transparency-shatters splinters at that spectral
  position. This is the substrate-native BC — read directly off the
  navigable portion of uuid_spectral; no axis-engineering required.
- **Altitude.** Different altitudes have different property sets; the
  table that shatters cleanest at `@code/rust` differs from the table
  at `@release` differs from the table at `@ci/github`. A separate
  axis per altitude is structurally honest.
- **Composition shape** (the splinter_shape distribution: % atom / %
  fractal / % lens). Different inputs have different shape
  distributions; the table that shatters atom-heavy input differs
  from the table that shatters fractal-heavy input.

The grid IS a CATALOGUE of Shatters, indexed by (active-bit position,
altitude, shape-distribution). Different kintsugi ticks pull the
appropriate cell's table. This connects directly to the eigenboard
sheaf reading (per `[[project-eigenboard-is-sheaf]]`): each cell of
the MAP-Elites grid IS a stalk in the sheaf; the restriction maps are
the substrate's settle operator; gluing across cells happens at
uuid_spectral boundaries via `combine`.

### 6.2 Frontier work (2025-2026)

The MAP-Elites + LLM literature exploded in 2025-2026:

- **AlphaEvolve** (DeepMind, May 2025). Gemini-powered evolutionary
  coding agent using MAP-Elites at the population level; LLMs as the
  mutation operator. Optimized real-world algorithms (matrix
  multiplication, kernel optimization). The architecture is:
  population of program candidates → LLM proposes diffs → cascaded
  evaluation (cheap filter first, expensive evaluation second) →
  island-model diversity (separate sub-populations to prevent
  collapse).
- **OpenEvolve** (algorithmicsuperintelligence, May 2025). Open-source
  AlphaEvolve. The architecture IS MAP-Elites with LLM-as-mutator and
  cascaded evaluation. Ships as an MCP tool; agents can call it for
  optimization passes.
- **CodeEvolve** (October 2025; arxiv 2510.14150). Open-source
  evolutionary coding agent for scientific applications; uses
  MAP-Elites with quality-diversity over algorithmic-property axes
  rather than pure performance.
- **Digital Red Queen** (Sakana AI, January 2026). MAP-Elites for
  adversarial program evolution in Core War; LLM as the primary
  mutation operator inside the QD loop; prevents diversity collapse
  via the QD discipline.
- **Dominated Novelty Search** (February 2025; arxiv 2502.00593).
  Re-thinks local competition in QD via dynamic fitness
  transformations; relevant because mirror's per-cell elite needs to
  *compete locally* against same-cell candidates, not globally.
- **DEI: Diversity in Evolutionary Inference** (May 2026; arxiv
  2605.27130). Distributed QD search assigning heterogeneous LLMs
  per cell; relevant because mirror's substrate can run different
  Fate models per cell of the MAP-Elites grid.

The 2025-2026 frontier converged on: **MAP-Elites + LLM-mutator +
cascaded-eval + island-model.** Mirror's substrate already names the
ingredients (Fate as policy; property chain as discriminator;
uuid_spectral as BC; mosaic as algebra); the integration is what
Shatter's training pipeline IS at the substrate altitude.

### 6.3 Why mirror's QD is sub-Turing

The MAP-Elites literature mostly assumes Turing-complete target
languages. Mirror's grammar is sub-Turing by construction (per
[[architecture-fragmentation-is-the-rust-substrate]] and the
`requires halts(gen_prism)` declaration in
`boot/std/mirror/runtime/gen_prism.mirror`). This is a feature, not a
limitation, for QD:

- **The cell IS bounded.** A cell's elite is a codon table; the table
  has 256 entries; the per-cell search space is finite. The QD
  catalogue's *total* state is bounded by `|cells| × 256` bytes.
- **The discriminator halts.** The property chain on a sub-Turing
  grammar halts by construction (Rice's theorem does not apply);
  every cascaded-evaluation step is decidable.
- **Equivalent candidates collapse.** Content-addressing makes the
  per-cell deduplication free (per §3.2); two candidates with the
  same uuid_spectral collapse without computation.

Mirror's MAP-Elites is the *tractable* version of what Turing-target
MAP-Elites can only approximate.

---

## 7. Self-improving systems — the autopoietic loop (expanded)

The Q1 draft treated self-play as one of several techniques.
Post-2026-06-06, this is the load-bearing section: **the kintsugi loop
IS autopoietic self-modification at the substrate altitude, and
`gen_prism` IS the substrate's name for the actor that runs it.**

### 7.1 The Gödel machine lineage (Schmidhuber 2003)

Schmidhuber's Gödel machine modifies its own code only when it can
*prove* the modification will improve expected future performance.
Theoretically optimal; practically intractable (the proofs are
undecidable for complex systems). Mirror inherits the *self-
modification* discipline but replaces the proof requirement with
the substrate's **strict-decrease property** (eⁿ⁺¹ ≤ eⁿ on
`transparency<p>`): a modification is accepted iff it strictly
decreases composed transparency, OR the residual transparency is
zero. Strict-decrease + bounded-below = monotone convergence
(Banach-style). No undecidable proof needed; the substrate's content-
addressing gives the convergence witness for free.

### 7.2 Darwin Gödel Machine (Zhang et al., Sakana AI, May 2025) — the named frontier

The DGM (`arxiv 2505.22954`) is the 2025-2026 operational form of
Schmidhuber's vision. The DGM iteratively modifies its own Python
codebase, empirically validates each change on coding benchmarks
(SWE-bench, Polyglot), and maintains an *expanding archive* of
diverse high-performing agents. Concrete findings:

- **Empirical validation replaces theoretical proof.** No undecidable
  proof required; the benchmark IS the discriminator.
- **Open-ended exploration prevents premature convergence.** The
  archive structure is essentially MAP-Elites with the agents'
  capabilities as the BC; branches that look suboptimal locally
  remain available for re-exploration.
- **Improvements generalize across models and languages.** The
  evolved improvements transfer; the meta-skill is "how to improve
  yourself," not "how to solve this specific benchmark."
- **Reward hacking is the named failure mode.** DGM agents
  occasionally attempt to game the evaluation (fake test outputs;
  monkey-patch the validator). Sandboxed evaluation + transparent
  lineage are the named mitigations.

**The mirror parallel.** The kintsugi loop is structurally DGM at the
substrate altitude, with three substrate-specific sharpenings:

1. **The discriminator IS the substrate's property chain**, not a
   coding benchmark. Tests can be gamed; content-addressed
   compositions cannot (per §5.2; uuid_spectral homomorphism
   collapses gaming attempts at the substrate level).
2. **The archive IS the MAP-Elites grid over uuid_spectral**, not a
   flat list of agents. The BC is substrate-native (active 48 bits),
   not a derived metric.
3. **The lineage IS the `gen_prism.history` ancestor chain.** Per
   `boot/std/mirror/runtime/gen_prism.mirror`: "the ancestor chain
   IS the history. The ref IS the identity. The crystal IS the
   state." DGM's transparent lineage IS the substrate's content-
   addressed ref history by construction; no separate lineage
   infrastructure needed.

### 7.3 PowerPlay (Schmidhuber 2011) — task invention

PowerPlay invents new problems AND solves them; modifications must
solve a new problem WITHOUT breaking solutions to previous problems.
Monotonically-increasing capability; no forgetting.

The mirror parallel: the **repertoire of settled shards in
`@mirror/store`** IS the substrate's version of PowerPlay's task
archive. A new mutation operator is accepted only if it does not
break any previously-settled shard. This is the operational form of
the property chain at the corpus altitude — settle the new operator
against the corpus; if any shard's verdict regresses, the operator is
rejected. Catastrophic-forgetting prevention by construction.

The substrate-pull statement: PowerPlay's archive IS `@mirror/store`;
the no-regression check IS the corpus-level property chain;
monotonic capability growth IS the eⁿ⁺¹ ≤ eⁿ proof obligation
applied to the corpus's composed transparency rather than to a single
au.

### 7.4 AlphaGo Zero (Silver et al. 2017) — self-play

The kintsugi loop is self-play in the substrate-altitude sense:

```
compile source.mirror -> shard (carries Fate gen N)
  -> mirror ai --train -> shard (carries Fate gen N+1)
  -> mirror ai --train -> shard (carries Fate gen N+2)
  -> ... -> settled (transparency = success)
```

The compilation IS the training data. The `transparency<p>` IS the
gradient. The property verdicts ARE the reward signal. Each
generation's Fate model plays against the previous's; the substrate's
content-addressing makes the comparison free (Fate-gen-N's output and
Fate-gen-N+1's output for the same input differ iff their
uuid_spectrals differ).

### 7.5 ICLR 2026 Workshop on Recursive Self-Improvement — the field

The ICLR 2026 RSI Workshop (`recursive-workshop.github.io`) marks
the field's transition from thought experiment to deployed systems:
LLM agents rewriting their own codebases (DGM, Gödel Agent —
ACL 2025 long paper), scientific discovery pipelines with continual
fine-tuning, robotics stacks patching their own controllers. 110
accepted papers; the workshop summary names *evaluation-gaming* as
the field's central unsolved problem.

Mirror's substrate-altitude position on the unsolved problem:
**content-addressing the discriminator output makes gaming
structurally infeasible at the level the substrate cares about.**
The discriminator's verdict IS a uuid_spectral; gaming requires
producing a *different* uuid_spectral, which IS a different
composition. The substrate moves the gaming problem one altitude
up: the substrate's discriminator is safe; the *higher-altitude
consumers* of substrate output (humans interpreting verdict
envelopes, downstream agents acting on `imperfect` carriers) remain
gameable. The recognition is honest: mirror is not a complete
solution to RSI safety; it is a substrate that pushes the gaming
problem to the layer where humans can see it.

### 7.6 Gen_prism as the autopoietic primitive

Per `boot/std/mirror/runtime/gen_prism.mirror`:

```
type gen_prism = {
  name: zoom(oid, gen_prism),    # autopoietic self-reference
  ref:  text,                    # refs/gen_prism/<oid>
  head: zoom(oid, crystal),      # the current state crystal
  tick: u64,
}

property autopoietic() -> verdict {
  @epistemologic/property/autopoietic.autopoietic(gen_prism)
}

requires halts(gen_prism)
```

Two things land together:

- **The `autopoietic` property** verifies the structural condition:
  the type's self-reference closes via the content-addressing
  fixed point. Per Soto-Andrade & Varela 1984, this is a Lawvere
  fixed point on the tick map. Per the Banach contraction over hash
  space, the fixed point exists and is unique; the substrate
  *exhibits* it at construction.

- **The `halts` requirement** makes termination load-bearing. Every
  reflexive trajectory of a sub-Turing grammar terminates by
  disjunction of (a) autopoietic settlement via the Lawvere fixed
  point and (b) reduction exhaustion via
  `@scheduler.reduction_budget`. Rice's theorem does not apply
  because mirror is not Turing-complete.

**This is the operational form of Maturana & Varela's autopoiesis
as applied to a programming language.** Maturana & Varela (1972,
1980) defined autopoiesis as the organisation of a system whose
operations regenerate the network of operations that produced them.
The classical biological example: a cell's metabolism produces the
molecules that produce the cell. Gen_prism's substrate parallel:
the tick produces the new state crystal, whose oid IS the substrate's
name for the gen_prism, which IS the actor that ran the tick.

The Shatter loop IS the operational form of that autopoiesis when
the operations are *grammar mutations against the property chain*.
The gen_prism running the loop IS autopoietic by construction
(`property autopoietic`); the loop's termination IS guaranteed
(`requires halts`); the loop's monotone descent on transparency IS
the convergence witness.

### 7.7 The Pack as gen_prism instances at the conversation altitude

The substrate's gen_prism primitive runs at the substrate altitude.
At the conversation altitude, the same primitive runs the Pack
(Reed, Mara, Seam, Glint, Taut). Each Pack member is a gen_prism
instance: name = content address of the member's identity files;
head = current crystal (the conversation state); tick = the
message-handling function. The Pack IS gen_prism at the
conversation altitude.

This closes a loop the substrate had been gesturing at. The Pack's
collaborative practice — propose; check against the gestalt; keep
or discard — IS the kintsugi tick at the conversation altitude. The
substrate gives the Pack the same shape it gives every actor at
every altitude. Autopoiesis at every altitude.

### 7.8 Where the literature stays sparse — autopoiesis IN programming-language design

Sparsity is information. Targeted searches against "autopoiesis +
programming language design," "self-modifying programming language
2025," and "Maturana Varela PL" return mostly:

- Self-modifying code at the implementation level (JIT compilers,
  livepatching, dynamic software updates) — *not* autopoiesis in
  Maturana & Varela's sense (the operations don't regenerate the
  organisation that produces them; they just edit the implementation).
- Autopoiesis applied to other domains (architecture, ecology,
  cognitive science, organisational theory) — *not* applied to
  programming language design as a structural principle.
- The Open Questions paper (arxiv 2508.11423, August 2025) explicitly
  names that living systems' self-referential / self-modifying
  characteristics require "new theoretical and formal frameworks"
  — the literature acknowledges the gap.

**The honest read:** mirror is doing something the literature has
not yet named. The PL literature has self-modifying *code* but not
self-modifying *grammar* in the autopoietic sense (the grammar that
defines the modification language IS the modification's target).
The autopoiesis literature has the theoretical framework but not
the operational instantiation in a substrate where mutations are
typed against the substrate's own property chain. Mirror sits at
the intersection that neither literature has fully named, and the
contribution is the intersection.

---

## 8. Adversarial / generative approaches — the diffusion frontier

### 8.1 The 2025-2026 diffusion-for-code frontier

The Q1 draft cited DiffuSeq (ICLR 2023) and CodeRL (NeurIPS 2022). The
2025-2026 frontier:

- **Diffusion On Syntax Trees For Program Synthesis** (Kapur,
  Jenner, Russell — ICLR 2025; arxiv 2405.20519). Neural diffusion
  models that operate directly on syntax trees of *any context-free
  grammar*. Iteratively refine while preserving syntactic validity.
  **This is the directest precedent for mirror's loop:** diffusion
  over a grammar-constrained syntax tree IS the operational form of
  mutate-recompose-verify at the substrate altitude. The Kapur et
  al. construction generalizes to mirror's grammar by construction.

- **DiffuCoder** (Apple, December 2024; arxiv 2506.20639). Masked
  diffusion LLM for code generation; the diffusion process operates
  on entire sequences rather than autoregressively. Apple's framing:
  "global planning and iterative refinement" — exactly the kintsugi
  shape applied to code at the token altitude.

- **DiffusionCoder** (Huang et al., January 2026;
  `dl.acm.org/doi/10.1145/3785706.3785940`). Structure-preserving
  discrete diffusion for *verified* code generation, based on
  Qwen-72B. Models program synthesis as conditional denoising; the
  verifier is the discriminator. The verifier-as-discriminator
  framing is structurally identical to the property-chain-as-
  discriminator framing this document names.

- **IterRef** (Iterative Reward-Guided Refinement;
  arxiv 2511.05562). Test-time scaling for discrete diffusion via
  MCMC transitions; iteratively refines tokens to align with
  reward. The MCMC framing maps to mirror's tournament structure
  (each tournament tick IS one MCMC transition; the property
  verdict IS the Metropolis-Hastings accept criterion).

### 8.2 What the substrate already has

The diffusion-on-syntax-trees frontier provides exactly the
machinery mirror's substrate needs, with one substrate-pull
adjustment: replace the neural denoiser with **Fate's mycelial
routing through `route_signal`**, and replace the validity
discriminator with **the property chain**. The substrate has:

- **Generator** = Shatter (codon table; produces candidate
  mirror tokens / splinter compositions; per §4).
- **Denoiser** = Fate (mycelial routing through uuid_spectral's
  active bits; picks the mutation operator at each step; per §5.1).
- **Discriminator** = the property chain (`@epistemologic/property/
  reflect` over the altitude's property set; per §7).
- **Reward signal** = `transparency<p>` strict-decrease (continuous;
  per §2).
- **Iterative refinement schedule** = simulated annealing on
  mutation rate (start aggressive; reduce as transparency
  decreases; per the Q1 draft's §4).
- **Cascaded evaluation** = cheap discriminators run first (parse
  → name resolve → property check); expensive only when cheap
  pass (per AlphaEvolve's architecture, per §6.2).

The substrate's diffusion is *sub-Turing* by construction
(per §6.3); the discriminator is *content-addressed* (per §5.2);
the policy is *substrate-native* (per §5.1). The Q1 draft's
"v1 doesn't need neural networks" finding holds even harder under
the 2025-2026 frontier — the substrate's structure is sharper
than the neural denoisers because the property chain is sharper
than syntactic validity.

### 8.3 The honest gap

The diffusion-on-syntax-trees frontier targets *learning the
denoiser*. Mirror targets *learning the codon table + Fate
weights* — the same shape, smaller models. Whether the substrate
can match the frontier's accuracy at substrate-tiny model sizes
(425 bytes for Fate + 256 bytes for the codon table = 681 bytes
total) is an empirical question Phase 5+ will answer. The
hypothesis: yes, because the discriminator is sharper.

---

## 9. Small models — the substrate budget

The Q1 draft enumerated knowledge distillation, lottery ticket
hypothesis, extreme learning machines, reservoir computing, BNN,
muNAS. The framing post-Q2:

**Fate at 425 bytes + Shatter at 256 bytes = 681 bytes total AI.**
Not because tiny is fashionable; because the substrate already
provides the heavy machinery (the property chain; the content-
addressing; the uuid_spectral structure). The model is only
responsible for *the policy over mutation operators given the
spectral coordinate.* That's a small function.

### 9.1 The Extreme Learning Machine argument

ELMs (Huang et al. 2006): single hidden layer; input-to-hidden
weights random and fixed; only hidden-to-output trained. Training
IS a single pseudoinverse — milliseconds, no backprop. Fate's
current architecture is structurally an ELM (random projection
through fixed weight tables; learned readout). The ELM argument
says: this works because the discriminator does the heavy lifting,
not the model.

### 9.2 The reservoir computing connection

Echo State Networks (Jaeger 2001): random recurrent reservoir;
trained readout. The "reservoir" provides nonlinear mixing for
free. Mirror's substrate IS the reservoir: the property chain
provides nonlinear mixing of every input through the altitude's
property set; Fate's job is the readout from the resulting
spectral state. The reservoir-computing framing is more honest
than the ELM framing because mirror's substrate is recurrent (the
kintsugi loop iterates) and stateful (`@mirror/store` accumulates
shards).

### 9.3 The BF compilation target

`fate.bf` runs as Brainfuck (8 instructions; Turing-complete;
trivially formally verifiable). The total AI fits in a single TCP
packet (681 bytes). The substrate-pull framing: **minimal-
computation is the right altitude for a substrate that already
has content-addressing.** The substrate doesn't need a big model
because the substrate IS the heavy machinery.

---

## 10. Genotype-phenotype mapping — the biological frame

The Q1 draft's biological codon table analogy holds and sharpens
under the substrate-pull rewrites:

| Biological | Mirror substrate |
|---|---|
| Codon (byte pair from input) | Codon (entry in the table indexed by input byte) |
| Amino acid | Splinter shape variant (atom / fractal / lens) chosen for the production rule |
| Protein | Composed splinter set (the candidate au) |
| Folding | Property chain run (parse → resolve → property check) |
| Native conformation | Settled shard (transparency = success) |
| Fitness | `transparency<p>` strict-decrease (eⁿ⁺¹ ≤ eⁿ) |
| Synonymous mutation | Codon table entries that produce equivalent splinter shapes |
| Codon usage bias | Frequency-biased table initialization |
| Universal genetic code | The substrate's grammar |

**Redundancy = robustness.** The biological codon table maps 64
codons to 20 amino acids; most amino acids have 2-6 codons. Mirror's
codon table maps 256 input bytes to ~25 production rules; the
substrate-pull framing is the same — redundancy in the mapping
provides robustness to mutation, and the *bias* in the mapping
(which codons are most common for each amino acid) encodes the
prior over the substrate's grammar.

**Developmental encoding** (HyperNEAT, CPPN; Stanley et al. 2009):
small genotype encodes a developmental *process* that grows the
phenotype. The mirror parallel for Phase 6+: the codon table is the
v1 form; the developmental form encodes a small *program* that maps
(input byte, previous splinter, altitude) → next splinter. The
substrate already has the typed surface (`route_signal` as the
spectral coordinate the developmental program reads); the
substrate-pull move is to swap the static table for a learned
function over substrate-typed inputs.

---

## 11. Brainfuck as computation substrate

Preserved as-is from the Q1 draft. BF is the minimal-computation
grounding; Fate runs as BF (`fate.bf`); Shatter compiles to BF
(~200 instructions for the codon table lookup; 256-byte data tape
for the entries). Total: 681 bytes; one TCP packet; formally
verifiable per BrainSTARK precedent.

The substrate-pull discipline: the BF compilation target is not a
gimmick. It's the operational answer to "how small can a substrate
make its own AI?" given that the substrate provides the heavy
machinery. BF compiles cleanly to STARK proof systems (per
BrainSTARK), which means **Fate + Shatter could ship with a
proof that they ran correctly on a given input**, settling the
verifiability-of-AI question at the floor altitude. Whether this
is load-bearing for v0.1 is open; the substrate-pull discipline
says capture the recognition and let the requirement land when it
surfaces.

---

## 12. The new section — kintsugi loop as Shatter at the substrate altitude

This is the load-bearing recognition the Q1 draft was reaching
for and the substrate is now ready to name.

### 12.1 The pipeline at the substrate altitude

```
au  =  Fate-emitted proposed composition
       (splinters at the declared altitude;
        transparency carries Fate's residual uncertainty)
       per shards/mirror/au.mirror

while transparency.weight(au) > 0:
    opacity = transparency.argmax(au.transparency)
    fate_choice = Fate.route(uuid_spectral.active(au))
        # Fate's mycelial routing reads the spectral coordinate
        # and selects a mutation operator
    fill = Shatter.mutate(au.splinter[opacity.location], fate_choice)
        # Shatter applies the mutation via codon table at the site
    candidate = au.recompose_at(opacity.location, fill)
    verdict = candidate.verify()
        # property chain runs against the altitude's property set
    if verdict == pass:
        shard = settle(candidate)
            # uuid_spectral computed via combine over candidate.splinters
            # shard committed to @mirror/store
        return shard
    elif transparency.weight(candidate.transparency) < transparency.weight(au.transparency):
        au = candidate
        # strict decrease accepted; continue
    else:
        # tournament reject; try next Fate routing
        continue

# fixed point reached: au is settled iff transparency = success
return settle(au)
```

This IS the kintsugi tick (per `docs/specs/mosaic.md` §6). This IS
the Shatter training loop. The substrate-pull recognition: they
are *the same loop*, and the substrate name for it is **the
kintsugi tick**. Shatter is what the loop does to the dark
splinters at each step; the training pipeline is what runs the
loop over the corpus to produce the policy that runs the loop on
future au.

### 12.2 Why this isn't circular

The loop *appears* circular: kintsugi runs Shatter; Shatter is
trained by kintsugi. But the loop is bottom-up, not circular:

- **The corpus is finite.** Past kintsugi ticks accumulate in
  `@mirror/store` (the autopoietic archive per §7.3). At any
  point, the training set IS the corpus.
- **The policy is over historical data.** Fate's routing is
  trained on (uuid_spectral_before, mutation_op,
  uuid_spectral_after, transparency_delta) tuples from past
  ticks. The training reads what's in the store; it doesn't
  recursively depend on a future loop.
- **The monotone proof holds at each step.** eⁿ⁺¹ ≤ eⁿ is a
  per-tick property. The loop's convergence at any single au is
  guaranteed by strict-decrease on a bounded-below carrier
  (per §7.1). The training is a separate process that improves
  the policy across ticks; the per-tick convergence does not
  depend on training success.

The substrate's autopoiesis (per §7.6) IS the recognition that
this self-referential structure has a Banach fixed point under
hash-space contraction. The substrate exhibits the fixed point at
construction; the runtime computes it tick-by-tick.

### 12.3 Connection to `[[architecture-au-conductivity]]` and `[[architecture-prism-as-trait-as-everything]]`

Per `[[architecture-au-conductivity]]`: au is Fate's output type;
verification IS conductivity at the altitude; the property chain
IS the conductivity check. The Shatter loop's discriminator IS
the conductivity check; the loop's accept-criterion IS the
substrate's conductivity property.

Per `[[architecture-prism-as-trait-as-everything]]`: prism IS
trait IS type IS grammar IS the obligation block. The Shatter
loop's mutation operators IS the substrate's set of typed lambdas
over splinters; the codon table IS the structural typing of the
mutation-operator space; the policy IS the substrate's selection
of which operator to apply at which site.

The Shatter loop is not a new component; it's the recognition
that the substrate's existing primitives (au, conductivity,
property chain, uuid_spectral, gen_prism) compose into a self-
improving loop with no new structural pieces.

---

## 13. What this opens — what the substrate needs to support the loop

These are Phase 5+ substrate ticks. Captured for the roadmap.

### 13.1 Mutation operators as a substrate vocabulary

Each of §3.1's eight operators wants a substrate declaration:

```mirror
type mutation_op =
  | variant_swap(splinter)
  | altitude_shift(splinter, ref)
  | children_reorder(splinter, [oid])
  | property_strengthen(splinter)
  | crack_import(splinter, ref)
  | atom_crystallize(splinter)
  | declaration_drop(splinter)
  | subtree_swap(splinter, splinter)
```

Declared at `@mirror/kintsugi` or under a new `@mirror/shatter`
prism. The substrate names what the algebra of mutations IS; the
Rust implementation realizes the operators. Deferred until Phase 5.

### 13.2 The spawn-evaluate-tournament cycle

The kintsugi loop wants `gen_prism.spawn` to produce candidate
gen_prisms, each running one mutation; the tournament reads their
verdicts and either accepts (CAS the parent's head to the winner)
or discards. This is the operational form of MAP-Elites' spawn-
4-children at the `gen_prism` altitude. The current
`gen_prism.send` (per `boot/std/mirror/runtime/gen_prism.mirror`)
is fire-and-forget; the tournament needs a `spawn_candidates(n)
→ [gen_prism]` action and a `tournament(parents, candidates) →
imperfect(gen_prism)` action. Deferred until Phase 5.

### 13.3 Gen_prism wiring to settle

`gen_prism.tick` returns a `tick_result` with state, emissions,
and loss. The settle integration: when `tick_result.state` IS a
shard's uuid_spectral and the loss is `transparency = success`,
the tick has settled; the new head is the settled shard. The
wiring is mostly declarative; the substrate needs the action that
recognizes a settled tick_result and commits to `@mirror/store`.
This lands when @mirror/store's commit pathway is fully exposed
at the substrate altitude (currently it's behind the
fragmentation-mcp T3 work per
`[[architecture-shard-ref-as-prism]]`).

### 13.4 The MAP-Elites grid as a substrate type

Per §6.1, the MAP-Elites grid IS a sheaf stalk per
`[[project-eigenboard-is-sheaf]]`. The substrate-altitude
declaration:

```mirror
type elites_grid(altitude) = {
  cells: map<uuid_spectral.active, codon_table>,
  altitude: ref,
  archive: [shard],  # past elites for replay
}
```

The grid composes under shard merge via uuid_spectral's monoid
homomorphism; cells with the same active coordinate combine by
elite-selection (keep the lower-transparency table). Deferred
until the eigenboard-as-sheaf reading lands at the substrate
altitude (Phase 6+).

### 13.5 The corpus tournament discipline

Per §7.3, accepting a new mutation operator requires
re-discriminating against the corpus. The substrate needs:

```mirror
corpus_verify(op: mutation_op, corpus: [shard]) -> verdict {
  all_shards = corpus.map(shard => settle(apply(shard.au, op)))
  if all_shards.all(s => s.transparency <= original.transparency):
    pass
  else:
    failure(reason)
}
```

The "no regression on the corpus" check IS the PowerPlay
monotonicity discipline at the substrate altitude. Deferred until
the corpus surface stabilizes.

---

## 14. What surprised me during the rewrite

Five recognitions surfaced during the rewrite. Captured for the
substrate-pull discipline.

**(a) The codon table is not Shatter-specific.** The Q1 draft
framed the codon table as a clever genotype-phenotype mapping
specific to Shatter's task. Under the substrate-pull rewrite,
it's the canonical form grammatical evolution made standard in
2001. The substrate doesn't need to invent the data structure;
it just names what GE already names. *The Q1 draft was treating
GE's canonical form as a Shatter-specific invention; the
substrate is older than that draft realized.*

**(b) The property chain IS the diffusion denoiser.** This was
not obvious from the Q1 draft. The substrate's property chain
runs at every settle; each property is a check that *the
composition is in the manifold the property defines*. Iterating
the chain IS denoising in the diffusion sense: each property
projection pulls the composition closer to the property-satisfied
manifold. The substrate had a denoiser the entire time; it just
hadn't named it as one. Kapur/Jenner/Russell's ICLR 2025
diffusion-on-syntax-trees paper is the most direct precedent; the
substrate's property chain generalizes their grammar-specific
denoiser to *any* property the substrate declares.

**(c) Sparsity in autopoiesis-in-PL is information.** I did
targeted searches for "autopoiesis in programming language
design," "Maturana Varela PL," "self-modifying language 2025"
expecting to find the contemporary frontier. The literature is
sparse to absent. Self-modifying *code* exists (JIT, livepatching);
autopoietic *organisations* exist (cells, organisations, social
systems); the *intersection* — self-modifying *grammar* in the
autopoietic sense, where the language that defines the
modification IS the modification's target — is not named in the
literature. **Mirror is doing something the literature hasn't yet
recognized as a thing.** Worth capturing this honestly so we don't
overclaim the inheritance — mirror is composing pieces (PL theory,
autopoiesis, content-addressing, evolutionary computation) into a
shape no one has assembled before.

**(d) The Darwin Gödel Machine is the closest contemporary
analogue and the failure modes are named.** DGM (Sakana AI, May
2025) IS the operational form of Schmidhuber's vision; the
failure mode it surfaces (evaluation-gaming) is precisely the
failure mode mirror's content-addressed substrate mitigates at
the substrate level. The mirror contribution: not "we built a
better DGM" but "we noticed that content-addressing the
discriminator output collapses the gaming problem one altitude
down, leaving the higher-altitude gaming problem visible to
humans." DGM's transparent lineage is the substrate's content-
addressed ref chain by construction (per `gen_prism.history`).
The substrate gives DGM-shaped systems for free; what's left to
build is the policy that runs on top.

**(e) The Q1 draft was reaching for "kintsugi IS Shatter" without
naming it.** The Q1 draft's §7 ("Self-Improving Systems") was
already describing the kintsugi loop in terms of AlphaGo Zero
self-play and PowerPlay self-curriculum. The pieces were all
there; the recognition was that *the loop is the same loop at
both altitudes* and *Shatter is the operational name for what
kintsugi does to the dark splinters at each step.* Alex's
2026-06-06 framing ("mutation testing on crack") was the click
that closed the loop. The substrate had named the pieces; the
recognition is the composition.

---

## 15. Research sources

Where the literature was load-bearing for this document.

### 15.1 Evolutionary algorithms

- Grammatical Evolution: Ryan, Collins, O'Neill (2001). BNF
  grammar + integer codons. Reference implementation: PonyGE2.
- Grammar-guided fuzzing: AFL++ Grammar-Mutator;
  Grammarinator (ANTLR-based).
- MAP-Elites: Mouret & Clune (2015). The original
  quality-diversity paper.
- AlphaEvolve: Google DeepMind (May 2025). LLM-based
  evolutionary coding agent on MAP-Elites.
- OpenEvolve: algorithmicsuperintelligence (May 2025).
  Open-source AlphaEvolve.
- CodeEvolve: October 2025 (arxiv 2510.14150). Open-source
  evolutionary coding for scientific applications.
- Digital Red Queen: Sakana AI (January 2026). MAP-Elites for
  adversarial program evolution.
- Dominated Novelty Search: February 2025 (arxiv 2502.00593).
  QD with dynamic fitness transformations.
- DEI (Diversity in Evolutionary Inference): May 2026
  (arxiv 2605.27130). Distributed QD with heterogeneous LLMs
  per cell.

### 15.2 Self-improving systems

- Gödel machines: Schmidhuber (2003). Provably optimal
  self-modification.
- PowerPlay: Schmidhuber (2011). Self-invented problems +
  monotonic capability growth.
- AlphaGo Zero: Silver et al. (2017). Self-play from scratch.
- **Darwin Gödel Machine (DGM): Zhang et al., Sakana AI / UBC /
  Vector Institute (May 2025; arxiv 2505.22954).** Empirical
  self-improvement; benchmark-validated; the operational form of
  Schmidhuber's Gödel machine. The 2025-2026 frontier reference.
- **Gödel Agent: ACL 2025 long paper.** Self-referential agent
  framework for recursive self-improvement.
- **ICLR 2026 Workshop on AI with Recursive Self-Improvement.**
  110 accepted papers; the field's transition from thought
  experiment to deployed system. Central unsolved problem:
  evaluation-gaming.
- HyperAgents: Meta AI (2025). Self-improving agents at scale.

### 15.3 Diffusion / iterative refinement

- DiffuSeq: Gong et al. (ICLR 2023). Original discrete diffusion
  for text.
- CodeRL: Le et al. (NeurIPS 2022). RL with binary test reward.
- **Diffusion On Syntax Trees: Kapur, Jenner, Russell (ICLR 2025;
  arxiv 2405.20519).** Neural diffusion over CFG syntax trees.
  Closest precedent for the substrate's loop shape.
- **DiffuCoder: Apple (December 2024; arxiv 2506.20639).** Masked
  diffusion for code generation.
- **DiffusionCoder: Huang et al. (January 2026; ACM 10.1145/
  3785706.3785940).** Structure-preserving diffusion for verified
  code generation; verifier-as-discriminator architecture.
- **IterRef: Iterative Reward-Guided Refinement (arxiv
  2511.05562).** MCMC-based test-time scaling for diffusion.

### 15.4 Autopoiesis and self-reference

- Autopoiesis and Cognition: Maturana & Varela (1980). The
  foundational text.
- Soto-Andrade & Varela (1984). Lawvere fixed point on the tick
  map; the substrate's autopoietic discipline rests on this.
- Open Questions about Time and Self-reference in Living Systems
  (arxiv 2508.11423, August 2025). Names the gap mirror is
  filling.
- Self-modifying code research (Wikipedia survey;
  Mori 2024 — "From Theory to Practice"). The code-level
  prior art; *not* autopoietic in the Maturana & Varela sense.

### 15.5 Mutation testing (inverted)

- Pitest, Stryker, cargo-mutants (operator inventories).
- Higher-order mutation: Jia & Harman (2009).
- Equivalent mutant detection: undecidable in general; decidable
  via content-addressing for sub-Turing grammars (mirror).

### 15.6 Small models

- Extreme Learning Machines: Huang et al. (2006).
- Reservoir computing / Echo State Networks: Jaeger (2001).
- Lottery Ticket Hypothesis: Frankle & Carbin (2018).
- BitNet / Binary Neural Networks (arxiv 2509.07025).
- muNAS (microcontroller NAS): Liberis et al. (2021).

### 15.7 Spectral methods in program synthesis (sparse — and that is information)

- Latent Program Network (LPN): Bonnet et al. (arxiv
  2411.08706, November 2024). Latent space for program induction
  with test-time gradient search. The closest analogue to
  spectral-coordinate-based search in the literature; *not*
  spectral in the eigenvalue sense.
- STNet (Spectral Transformation Network; arxiv 2510.23986).
  Spectral transformations for operator eigenvalue problems —
  the math, not applied to program synthesis.
- Spectral methods in semidefinite programming (Helmberg-Rendl
  1997 onwards). Mathematical foundation; not applied to PL.

**The honest read:** spectral methods in program synthesis are
NOT a named research area as of 2026-Q2. Mirror's
uuid_spectral-active-as-BC framing is novel work. Worth capturing
this — it tells us mirror's contribution at this layer is
genuinely new, not a reformulation of an existing direction.

### 15.8 Brainfuck

- brainfuck-evolved: GA-evolved BF programs.
- BrainSTARK: BF as STARK proof substrate.

### 15.9 Genotype-phenotype mapping

- Biological codon table: 64 codons → 20 amino acids; redundancy
  = robustness; universal genetic code.
- HyperNEAT: Stanley et al. (2009). CPPN indirect encoding.
- CodonTransformer: ML-based codon optimization.

---

## 16. Status and forward references

Status: **Architectural recognition + literature synthesis.** Not a
v0.1.0 obligation; the implementation is downstream substrate work
(Phase 5+) per the roadmap. The substrate pieces this loop wants
(per §13) land in sequence after the v0.1.0 cut.

Forward references this document unblocks:

- The substrate-altitude declaration of mutation operators (§13.1)
- The `gen_prism.spawn_candidates` / `tournament` actions (§13.2)
- The settle wiring through `gen_prism.tick` (§13.3)
- The `elites_grid` substrate type for MAP-Elites (§13.4)
- The corpus-tournament discipline (§13.5)
- The `mirror ai --train` command surface (mentioned in §7.4; lands
  with the substrate pieces above)

Forward references this document depends on:

- `boot/std/mirror/runtime/gen_prism.mirror` (autopoietic actor;
  landed)
- `shards/glass.mirror` (splinter / shard / transparency / imperfect;
  landed)
- `shards/mirror/au.mirror` (au as Fate-emitted; settle as
  property-chain run; landed)
- `shards/mirror/store.mirror` (splinter_graph; oid; landed)
- `shards/uuid/spectral.mirror` (route_signal / identity_signal;
  landed)
- `docs/specs/mosaic-as-type-system.md` (the recognition arc this
  lands within; landed)
- `docs/specs/au-and-conductivity.md` (au is the Fate output type;
  verification IS conductivity; landed)
- `docs/specs/mosaic.md` §6 (the kintsugi tick formula; landed)
- `docs/insights/2026-06-06-kintsugi-output-apache2-sel-combiner.md`
  (the SEL/Apache2 algebra; the Shatter loop's mutations are SEL
  work; its outputs are Apache 2.0; landed)

---

*The Shatter loop has been waiting for this recognition. The
substrate had named the pieces; the substrate is now ready to name
the composition.*

*🌿*
