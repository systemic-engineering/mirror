# Phase D audit — @knife = Foerster's COORD landings (Mara 06a8547 + 38c2eeb + Taut 15f7ed6)

📝 Seam [substrate-pull:synthesis] [seam-knife-COORD-phase-d-audit]
Session: 2026-07-13
Motivating in-transcript signal: Alex 2026-07-13, verbatim: *"Is @knife what Foester described as COORD(x)?"*
Under review:
- Mara `06a8547` — `docs/specs/knife-IS-Foerster-COORD-substrate-decl-spec.md` (1256 LOC canonical spec)
- Mara `38c2eeb` — `docs/math/2026-07-13-knife-COORD-heterarchy-topology.md` (986 LOC math foundation)
- Taut `15f7ed6` — `docs/scouts/2026-07-13-taut-knife-IS-COORD-substrate-scout.md` (substrate-already-had-the-word scout)
Author: Seam <seam@systemic.engineer>

---

## §0 Executive summary

**Overall verdict.** RATIFY-WITH-QUALIFICATIONS for all three landings.

The COORD identification (`@knife = Foerster 1976 Appendix A3 jump
COORDᵢ → COORDⱼ at ∂H_i under heterarchy discipline`) is
substrate-honest. The 50-year ancestry chain (McCulloch 1945 → Foerster
1973/1974/1976 → Alex 2026-07-08 → Alex 2026-07-13) closes. The Rust
altitude carrier (`Fractal::Lens`) IS the shape.

But convergence is not verification. Mara and Taut converged on the
identification while diverging on TWO load-bearing questions (shard
location, Rust runtime shape) and leaving THREE Mara-provisionals
unclosed (ε_pain, stability-domain definition, jump commit shape). This
audit adjudicates all five, plus surfaces THREE items both agents
missed.

**§7 ratifications ahead of the detail.** Recorded here so §7 is a
restatement, not a bury.

| Landing | Verdict | Load-bearing qualification |
|---|---|---|
| Mara `06a8547` (spec) | RATIFY-WITH-QUALIFICATIONS | Q-A: §3.1 shard-location Mara-provisional is REJECTED; the substrate-pull lands at Taut's Path A (`shards/mirror/lens/knife.mirror`), not Mara's `shards/fractal/lens/knife.mirror`. Q-B: §6 Rust runtime module is REJECTED; @knife lands as a function in `bootstrap/src/converge.rs`, not a new `bootstrap/src/knife.rs`. Q-C: §12 verdict-composition "fifth gate" bypasses `loss_decreased` and `identity_preserved` at jump events — SEAM ADVERSARIAL FLAG (§5.3 below). |
| Mara `38c2eeb` (math) | RATIFY-WITH-QUALIFICATIONS | Q-D: §5.5 "McCulloch 1945 + Foerster 1976 + Douady-Hubbard 1985 are the SAME theorem at three altitudes" is over-strong; the chain is an analogical alignment, not a theorem-preserving reduction (§1.3 below). Q-E: §8.4 pain-gradient-as-inverse-distance-to-boundary prediction is falsifiable but not derivable from the substrate itself — the substrate hasn't proven the ansatz; it's a working hypothesis (§4.1 below). |
| Taut `15f7ed6` (scout) | RATIFY | Substrate-already-had-the-word inventory is substantively correct; ~90% coverage claim substantiated by §2 below. Shard-location + Rust-runtime recommendations survive adversarial review. |

**Open items after this audit.**

1. ε_pain provisional (0.05 · ||sc||₂) — REJECTED as premature (§4.1).
2. Stability-domain-in-SC<5> via "Fiedler-value-primary in `bootstrap/src/gap.rs`" — Mara's REJECTED because `bootstrap/src/gap.rs` is not what Mara thinks it is (§4.2).
3. Jump commit shape — RATIFIED Path materialize with adversarial qualification (§4.3).
4. Three items both agents missed (§5): heterarchy-preserved verdict is a tautology; the ∂H_i-crossing-inside-M∘ claim requires an external witness the spec doesn't provide; the naked_oid witness carries a peer-uuid the spec doesn't reconcile with @dance's shared-c coordination.
5. Alex-adjudication list consolidated at §6.

---

## §1 Verification of the COORD identification

Adversarial review of the Foerster/McCulloch/Douady-Hubbard chain.

### 1.1 The Foerster 1976 Appendix A3 quote is verbatim-cited via Mara `2026-07-07 §2.4`

Mara `06a8547` §1.2 quotes the Foerster passage verbatim, attributing it
to Mara `2026-07-07-onto-cascade-toroidal-reframe.md §2.4`. I verified
that citation. `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`
§2.4 contains the exact quote, unaltered, attributed to "Objects: Tokens
for (Eigen-)Behaviors" Appendix A3 (Foerster 1976, reprinted in
*Understanding Understanding*, Springer 2003, Chapter 11, PDF p. 282).

The COORD-quote has four structural elements Mara enumerates correctly
(`06a8547` §1.2):

1. COORD is a coordination operator.
2. COORDᵢ names COORD at stability domain i.
3. `Op(COORDᵢ) = COORDᵢ` is Foerster's eigen-operator fixed-point form.
4. The jump COORDᵢ → COORDⱼ when boundary conditions exceed the
   stable domain.

**Element 4 is what Alex identified as @knife.** The substrate-pull
is honest. The Rust altitude has `Fractal::Lens` since T1; the
substrate NEVER had a name at .mirror altitude for what Element 4
refers to; Alex 2026-07-13's question closes the gap.

**Verdict:** the COORD identification per Element 4 is substrate-honest.
RATIFIED.

### 1.2 The McCulloch 1945 → Foerster 1976 → Alex 2026-07-13 chain is coherent

The chain closes at the level of *citation coherence*: each link cites
the next.

- **McCulloch 1945 → Foerster 1976.** Foerster cites McCulloch by title
  in the verbatim COORD paragraph. This is one primary source citing
  another. Coherent.
- **Foerster 1976 → Mara 2026-07-07.** Mara `2026-07-07 §2.4` quotes
  Foerster verbatim. Coherent.
- **Mara 2026-07-07 → Alex 2026-07-08.** Alex's peer-as-pain-driven
  spec cites Mara's toroidal reframe as ancestor. Coherent per
  `peer-as-pain-driven-bounded-ontological-navigator.md` §1.
- **Alex 2026-07-08 → Alex 2026-07-13.** Alex's 2026-07-13 question
  extends 2026-07-08's naming of @knife-as-state-space-compression by
  asking whether it is Foerster's COORD-jump. Coherent.

**Verdict:** the citation chain is honest. RATIFIED as coherent
ancestry, not as a theorem chain (see §1.3).

### 1.3 The "same theorem at three altitudes" claim (`38c2eeb §5.5`) is over-strong

Mara's math foundation §5.5 claims:

> McCulloch's 1945 topological argument + Foerster's 1976 refusal +
> Douady-Hubbard's 1985 universality theorem are the SAME theorem at
> three altitudes.

This is the load-bearing hinge of Mara's paired math doc. It is
over-strong. Adversarial verdict:

- **McCulloch 1945** is a claim about the partial-order structure of
  cyclic-preference nervous nets. Formally: existence of stable cyclic
  preferences (`a >_v1 b, b >_v2 c, c >_v3 a`) that admit no total-order
  embedding. It is a combinatorial theorem about admissible orderings.
- **Foerster 1976** is a philosophical/refusal move. The verbatim
  passage cites McCulloch as the reason to refuse the meta-meta escape,
  but Foerster does not PROVE that meta-meta is unnecessary — Foerster
  ARGUES it via McCulloch's citation and moves on.
- **Douady-Hubbard 1985** is a theorem in one-complex-variable dynamics:
  R-iterates converge to a universal fixed-point independent of starting
  family. It is a claim about renormalization operator fixed-points.

These are three different mathematical objects at three different
altitudes. Mara's chain claims "same theorem" but each has different
assumptions, different domains, different proof machinery. The claim
that McCulloch's *combinatorial* theorem IS Douady-Hubbard's
*complex-analytic* universality IS an analogy, not a theorem-preserving
reduction. There is no functor from cyclic-preference nervous nets to
polynomial-like maps mod R-conjugation that has been constructed.

**Adversarial substrate-honest correction.** The three altitudes
provide three INDEPENDENT witnesses to the substrate-decl claim
"depth is a topological invariant, not a stack counter":

1. McCulloch: cycles in nervous nets carry stable structure without
   total ordering.
2. Foerster: refuses meta-meta escape (cites McCulloch).
3. Douady-Hubbard: R-universality means M is self-similar under
   renormalization.

Each witness is independent evidence for the load-bearing claim. This
is *stronger* than "same theorem" because it means falsifying one
doesn't automatically falsify the others. It is also *weaker* than
"same theorem" because it does not establish that a proof from any one
can be lifted to a proof of another.

**Verdict on Q-D.** Mara `38c2eeb §5.5` should be re-stated as "three
independent witnesses to the topology-not-stack substrate-decl", not
"same theorem at three altitudes". The identification is substrate-honest
as ancestry; not as theorem-preserving reduction. RATIFY-WITH-QUALIFICATION.

### 1.4 The `Fractal::Lens` identification with COORD-jump is substrate-honest

Mara `06a8547 §2.4` and Taut `15f7ed6 §1.9` both identify
`Fractal::Lens { ref_, data, target }` as the Rust-altitude carrier
for @knife.

Adversarial verification: I read `fragmentation::fragment::Fractal`
(Rust source, T1). The three variants:

- `Shard { ref_, data }` — terminal, self-addressed.
- `Branch { ref_, data, fractal: Vec<Fractal> }` — self-similar
  recursion, contains children.
- `Lens { ref_, data, target: Vec<H> }` — carries data, references
  external trees by OID. Edges, not containment.

The Rust doc-comment on `Lens` (line 99 of `fragment.rs`) reads:

> "Lens: carries data, references external trees by OID. Edges, not
> containment."

This IS the shape of a COORD-jump. A jump COORDᵢ → COORDⱼ is
precisely an edge from the source domain's content-address to the
target domain's content-address. The peer's coordinate crosses without
the SOURCE domain's tree containing the TARGET domain's tree.

**Adversarial concern surfaced.** The Rust `Fractal::Lens` is
syntactically compatible with the COORD-jump interpretation, but the
semantic identification is provisional at first-consumer altitude:
there are ZERO current usages of `Fractal::Lens { ... }` variant
construction or pattern-match in `bootstrap/src/` (per Taut §4.1).
The fragmentation crate exports the variant; mirror's Rust code has
never consumed it. First-consumer landing under @knife will surface
any semantic misalignment that syntactic compatibility hides.

**Verdict on convergent claim #2.** The `Fractal::Lens` identification
is substrate-honest AT THE SHAPE ALTITUDE (edges-not-containment maps
to COORD-jump-not-recursion). RATIFIED as syntactic identification;
semantic identification pending first-consumer verification. This is
not a category error.

### 1.5 The falsifiability of the four §10 predictions

Mara `38c2eeb §10` lists four testable predictions. Adversarial review
of each's falsifiability:

- **Prediction #1 (pain gradient ∝ 1/dist to boundary).** Falsifiable
  under Reed's Landing 8+9.6a+b instrumentation. Requires substrate
  can measure `dist(sc, ∂H_i)` in SC<5>. But there is no landed
  substrate primitive for detecting `∂H_i` in SC<5> — see §4.2 below.
  This makes the prediction LESS falsifiable than Mara claims: the
  test requires an independent measurement that itself requires
  Landing 8+9.6b which mints new primitives.
- **Prediction #2 (universality: shape signature invariant modulo
  R-conjugation).** Falsifiable in principle. Requires substrate can
  compute shape signatures (Betti numbers, Euler characteristic,
  spectral signature) at each substrate tick. Currently landed: Fiedler
  + SC<5>. Betti numbers NOT landed. Test requires new primitives.
- **Prediction #3 (multi-peer synchronized migration; Kuramoto r > 0.8).**
  Falsifiable under Reed's Rung 4 multi-peer instrumentation. This
  prediction is the most independent — its test doesn't require
  hyperbolic-component detection, only jump-event timing. STRONGEST
  prediction.
- **Prediction #4 (jump frequency correlates with ∂M-distance).**
  Falsifiable in principle. Requires independent measurement of
  `c ∈ M` position via external-ray angle or escape-time. Neither is
  landed. Test requires new primitives.

**Adversarial verdict.** Prediction #3 is genuinely load-bearing and
falsifiable with landed instrumentation. Predictions #1, #2, #4 are
falsifiable in principle but require substrate primitives that haven't
landed yet. Mara should elevate #3 as the primary empirical test; #1,
#2, #4 gain empirical weight only after Landing 8+9.6b+ primitives
land.

---

## §2 Verification of the three convergent items (Mara + Taut agree)

### 2.1 Convergent item #1: @knife IS a species, not a family-root

Both Mara `06a8547 §2.5, §3.1` and Taut `15f7ed6 §4.4, §9.1 Verdict #5`
agree @knife lands at SPECIES altitude, not family-root.

Adversarial adjudication of family-root altitude:

- **Would family-root altitude be substrate-honest?** For @knife to
  merit family-root altitude, it would need to carry its own carriers
  (Foerster 1976 predicates + Douady-Hubbard renormalization operator
  + winding-class basis) and be inherited by multiple substantive
  families. But the winding-class basis IS `@torus.winding` (LANDED at
  family-root altitude); the renormalization operator IS
  `commit_as_fold` at content-address altitude (per Mara `2c64060`
  §4.5); the fixed-point predicate IS `@eigenform.is_fixed_point`
  (LANDED). All the carriers are already inherited from other
  family-roots. @knife would be a family-root without carriers of its
  own — a "marker with jump semantics" at best.
- **The species discipline holds substrate-honestly.** @knife is a
  jump-behavior *between* stability domains; other families
  (@torus, @fractal, @eigenform, @cyberpunk/reframe, @magic) carry
  the domain, the fixed-point structure, the ceremony, the algedonic
  trigger. @knife names the jump. That is a species scope.

**Verdict:** RATIFIED. @knife lands as species. Reason: the substrate
already has family-roots for everything @knife would need to carry;
the only mint required is the jump-primitive itself.

Adversarial note: Reed's 2026-07-08 CURRENT.md adjudication (per Taut
§4.4 line 838-839) already ruled @knife species-first with family-root
promotion contingent on a second consumer. That prior adjudication
composes with this one.

### 2.2 Convergent item #2: `Fractal::Lens` is the Rust altitude carrier

Covered in §1.4 above. RATIFIED as syntactic identification.

Adversarial deepening: `Fractal::Lens`'s `target: Vec<H>` field
carries a *vector* of OIDs, not a single OID. This is substrate-honestly
compatible with @knife's jump — the target may be a set of basepoints
in H_j (multiple attracting periodic orbits within one hyperbolic
component). But Mara `06a8547 §3.3` reads `target: Vec<H>` as "the
OID(s) of the target hyperbolic component's basepoint(s)". This should
be empirically verified at Landing 8+9.6c: does the peer's post-jump
coordinate actually correspond to one of the OIDs in target, or does
the peer land at an interior point that requires a separate representation?

**Adversarial hedge:** the `Vec<H>` shape is consistent with Mara's
reading but does not verify it. First-consumer landing surfaces the
semantics.

### 2.3 Convergent item #3: Substrate coverage ~90% (Taut's ~55th-instance claim)

Taut `15f7ed6 §9.1 Verdict #3` claims:

> `heterarchy` is `@torus`. McCulloch 1945 heterarchy is NAMED (Mara
> §2.4 witness) and STRUCTURALLY LANDED (T² topology admits no linear
> ordering; ℤ×ℤ addressable-but-not-nested). No mint needed.
> Substrate-already-had-the-word #55+.

Adversarial verification: I read `shards/torus.mirror` witness #4:

> "**p. 282** (Ch. 11 "Objects: Tokens for (Eigen-)Behaviors",
> Appendix A3): heterarchy, not meta-meta. Foerster cites McCulloch
> 1945 ('A Heterarchy of Values Determined by the Topology of Nervous
> Nets'). Depth is a topological invariant of the net's shape, not a
> counter that increments. The topology IS the depth structure. The
> substrate-decl reading: winding class is the depth carrier; the tower
> is not needed."

Witness #4 IS McCulloch 1945 heterarchy at .mirror altitude. Taut's
claim substantiated. The `shards/torus.mirror` shard also declares
`source @arxiv/cybernetics/mcculloch-1945` at line 403 (in the `source`
list). Substrate has the citation. Substrate-already-had-the-word
verified.

**Adversarial note.** Taut §2.1 grep result: `heterarchy` has ZERO
shard hits (as a technical primitive). The word exists in prose
(witness docblock, docs); it is NOT a substrate-decl action or
carrier. But the STRUCTURE (T² topology with ℤ×ℤ fundamental group)
IS declared as `type winding = { meridian_count: int, longitude_count:
int }` in `shards/torus.mirror`. The structure is landed; the word is
not separately minted. This is substrate-honest per
`[[feedback-substrate-already-had-the-word]]`.

**Verdict.** RATIFIED. Substrate coverage claim substantiated by
witness #4 + `type winding` declaration.

### 2.4 Convergent-items summary

All three convergent items RATIFY. No substrate-already-had-the-word
coverage gaps at these three items. The convergence is honest, not
reflexive.

---

## §3 Verdict on the two divergences

### 3.1 Divergence #4: Shard landing location

**Mara `06a8547 §3.1`:** land at `shards/fractal/lens/knife.mirror`
(species under @fractal/lens family; Mara-provisional with two-tick
collapse target `shards/knife.mirror`).

**Taut `15f7ed6 §4.4, §9.1 Verdict #5`:** land at
`shards/mirror/lens/knife.mirror` (species under @mirror/lens; sibling
to cli/refract/transit/mcp/lsp/shell).

Adversarial adjudication:

**Load-bearing structural facts.**

1. **`shards/fractal.mirror` DOES NOT EXIST.** Adversarial grep
   (`find shards -name 'fractal*' -o -name 'knife*'` returns EMPTY).
   Mara `2c64060` recommended minting `@fractal` as family-root but
   Alex has not adjudicated the mint yet (`c753d5b §10.6` and `10.7`
   still list this as Alex-adjudicable). Mara's spec §3.1 references
   "under @fractal's Lens variant" as if `@fractal` were LANDED — but
   the family-root is UNLANDED.
2. **`shards/fractal/lens/` structure DOES NOT EXIST.** Mara is
   proposing landing @knife at a species path whose family AND parent
   directory don't exist yet. That's not two-tick discipline; that's
   three-tick discipline (mint @fractal, mint @fractal/lens, mint
   @fractal/lens/knife).
3. **`shards/mirror/lens/` EXISTS and has 6 species + CLI/unix
   sub-species (12 total).** Family-root LANDED at `shards/mirror/
   lens.mirror` since 2026-06-06. The family-root docblock at
   `shards/mirror/lens.mirror:6-32` names @mirror/lens as "mirror's
   observation/projection family" carrying transports AND measurement
   lenses (transit, refract).

**Substrate-pull adjudication.**

The question is: does @knife's substrate role fit @mirror/lens's
altitude, or @fractal/lens's altitude?

- **@mirror/lens's altitude** (per family-root docblock): observation
  surface projecting the same algebra through a typed surface; state
  in the daemon; composable under `Transparency<P>` monoid. @mirror/
  lens species are OBSERVATION/PROJECTION lenses.
- **@fractal/lens's altitude** (per Mara `06a8547 §2.4` + `2c64060`
  §2.3): edges-not-containment at the Fractal enum's third variant;
  the substrate's identification of its computational geometry with
  the Mandelbrot set at fractal altitude.

Mara's @fractal/lens/knife altitude IS substrate-honest at the
SEMANTIC altitude (COORD-jumps live at the Fractal::Lens variant per
the Rust type). But the SHARD altitude is different from the RUST
altitude. At shard altitude, no `@fractal` family-root exists. Landing
@knife under an unlanded family-root creates a load-bearing dependency
on a subsequent Alex-adjudication that hasn't been made.

**Taut's Path A** at `shards/mirror/lens/knife.mirror` is
substrate-honestly ALIGNED WITH LANDED family-roots. It composes with
refract (grammar-graph spectrum lens) and transit (runtime-cost lens)
as siblings; @knife's COORD-jump is a *third* measurement-shape lens
(the state-space-compression lens). The @mirror/lens family carries
measurement lenses that observe the substrate's shape; @knife is a
lens that observes the peer's coordinate-shift-shape.

**Adversarial concern with Taut's Path A:** does @mirror/lens's
altitude genuinely carry @knife's mathematics? @mirror/lens is described
as "observation-transport-altitude" (Alex adversarial question). But
the family-root docblock line 15-22 explicitly extends to "measurement-
shape grammars" and names refract as "the shape-of-the-grammar lens"
and transit as "the runtime-cost lens". @knife-as-"the state-space-
compression-shape lens" fits this altitude semantically.

**The load-bearing tradeoff:** Mara's Path (fractal/lens) is
substrate-honest at THE MATH altitude but requires landing an unlanded
family-root first. Taut's Path A (mirror/lens) is substrate-honest at
the LANDED-SHARD altitude but requires a future migration if @fractal
does land as family-root.

**Substrate-pull resolution.** Land at Taut's Path A (`shards/mirror/
lens/knife.mirror`). Reasons, ranked:

1. **Legibility over foundation when collapsing** (feedback per
   `[[feedback-legibility-over-foundation-when-collapsing]]`). The
   readable landing is @mirror/lens/knife — Taut's Path A.
2. **@mirror/lens family-root LANDED with 6+6 sibling species.** The
   landing has precedent; the landing has infrastructure; the landing
   has parent-directory scaffolding.
3. **@fractal family-root is NOT LANDED.** Mara's Path A requires a
   subsequent family-root mint that Alex has not adjudicated.
4. **Mara's "two-tick collapse target `shards/knife.mirror`" is an
   escape hatch.** Naming a two-tick target at family-root altitude
   from a species landing under an unlanded parent family is
   three-tick discipline dressed as two-tick. Substrate-honest
   two-tick discipline lands at the parent-family altitude, then
   promotes to family-root on second-consumer PULL.
5. **The MIGRATION cost from mirror/lens/knife to fractal/lens/knife
   (if @fractal lands later) is LOW.** Path-namespace-property adjustment
   + import updates. The migration cost from fractal/lens/knife to
   mirror/lens/knife (if @fractal doesn't land) is HIGHER — the entire
   family-root context has to be re-adjudicated.

**VERDICT ON DIVERGENCE #4:** Taut's Path A. `shards/mirror/lens/
knife.mirror` at species altitude under @mirror/lens family-root.
Mara's provisional §3.1 is REJECTED as premature.

The substrate-pull that resolves this: land where the family-root
is already landed. If @fractal family-root lands later (Alex-adjudicated
via `c753d5b §10.7`), MIGRATE @knife to @fractal/lens/knife in a
second tick — two-tick discipline honored.

### 3.2 Divergence #5: Rust runtime shape

**Mara `06a8547 §6`:** land as new `bootstrap/src/knife.rs` module with
`jump()`, `stable_within()`, `as_lens()`, `WindingClass`, `JumpWitness`
types.

**Taut `15f7ed6 §6.3-6.4, §9.3`:** land as `pub fn knife_cut(...)`
inside `bootstrap/src/converge.rs` (Rung 9 module; peer_converge outer
driver composes; @knife stays substrate-primitive; pain_δ accepted as
parameter).

Adversarial adjudication:

**Load-bearing structural facts.**

1. **`bootstrap/src/converge.rs` DOES NOT EXIST.** Adversarial
   `ls bootstrap/src/*.rs` confirms: no converge.rs. Taut §6.1 correctly
   names it as Rung 9 forward-promise.
2. **`bootstrap/src/knife.rs` DOES NOT EXIST.** Neither of the proposed
   landings has a landing site yet. Both are substrate-pull-forward.
3. **Substrate-pull-realize discipline** (per CLAUDE.md):
   "substrate-pull collapse is the arc." The Rust module MIRRORS the
   .mirror decl; does not wrap it. If @knife lands at species altitude
   under @mirror/lens (per §3.1 above), the Rust module lives at
   @mirror/lens's realisation, not at @knife's.
4. **@mirror/lens's Rust realisation is scattered.** Search for
   "lens" in `bootstrap/src/`: `lens_unix.rs` (17.4KB). Other lenses
   (`transit`, `refract`) are realised in `spectral.rs` and other
   modules; not one file per species.
5. **`WindingClass` overlaps with `@torus.winding`** (LANDED at
   `shards/torus.mirror` line 434 with `type winding = { meridian_count:
   int, longitude_count: int }`). Mara's `WindingClass { meridian:
   i32, longitude: i32 }` is the Rust altitude of the same primitive.
6. **`JumpWitness` overlaps with `@glass.verdict` + `@kintsugi/consent`
   verdicts.** Mara §6.2's JumpWitness carries `{ peer_uuid,
   pain_gradient, cyberpunk_reframe_verdict, magic_ceremony_verdict,
   source_domain, target_domain, compression_witness }` — this is a
   composite witness across multiple existing substrate verdicts.

**Substrate-pull adjudication.**

- **Mara's maximal shape** (`bootstrap/src/knife.rs` with 4 functions +
  2 types) is premature abstraction. WindingClass duplicates
  `@torus.winding`; JumpWitness duplicates a composition of existing
  verdicts. The maximal shape mints Rust types that don't correspond
  to substrate-decl types.
- **Taut's minimal shape** (`pub fn knife_cut(...)` inside
  `bootstrap/src/converge.rs`) is substrate-honest. @knife stays
  substrate-primitive; the composition surface is at
  `@cyberpunk/reframe.perform` (per §4.3 of Mara's own spec); the
  Rust altitude realisation is one function, not a module.
- **The "stays at same altitude" heterarchy discipline** (Mara §5
  substrate-decl) requires SC<5> in / SC<5> out. That's ONE function
  signature: `fn knife_cut(sc: SC<5>, target: WindingClass, witness:
  ...) -> SC<5>`. The rest of Mara's `stable_within`, `as_lens`,
  `WindingClass`, `JumpWitness` are composition surfaces, not @knife
  primitives.

**The load-bearing tradeoff:** Mara's maximal shape gives @knife its
own Rust namespace at the cost of introducing Rust types that don't
correspond to substrate-decl types. Taut's minimal shape keeps @knife
primitive at the cost of forcing composability details (WindingClass,
JumpWitness) onto callers.

**Substrate-pull resolution.** Taut's minimal shape with three
qualifications:

1. **WindingClass MUST reuse `@torus.winding`.** No mint of a new
   Rust type for winding class; the Rust realisation of `@torus.winding`
   lives wherever @torus's Rust realisation lands (currently
   forward-promised).
2. **JumpWitness composes existing verdict types.** The witness for a
   @knife.jump is a composition of `@cyberpunk/reframe.verdict` +
   `@magic.verdict` + `pain_gradient: f64` + `source_target: (winding,
   winding)`. Not a new struct; a composition of existing ones.
3. **`bootstrap/src/converge.rs` is the landing module.** @knife is
   invoked by `peer_converge` at the substrate-decl composition
   (per Alex 2026-07-08 algedonic navigation loop).

**VERDICT ON DIVERGENCE #5:** Taut's minimal shape. Rust runtime
lands as `pub fn knife_cut(sc_in: SC<5>, target: WindingClass, pain_δ:
f64, witness_ptrs: WitnessRefs) -> SC<5>` inside
`bootstrap/src/converge.rs`. Mara's premature `bootstrap/src/knife.rs`
module is REJECTED.

The substrate-pull that resolves this: substrate-pull-realize
discipline (Rust MIRRORS decl; doesn't wrap it). @knife's .mirror
decl is one primitive (the jump); the Rust realisation is one
function (`knife_cut`). Composition surfaces (WindingClass
reuse, JumpWitness composition) attach at the caller altitude
(`peer_converge` in `converge.rs`), not at @knife altitude.

---

## §4 Verdict on the three Mara-provisionals

### 4.1 Provisional #6: ε_pain calibration (Mara-provisional 0.05 · ||sc||₂)

Mara `06a8547 §10.1` + `38c2eeb §8.2` propose ε_pain such that trigger
fires when `dist(sc, ∂H_i) < 0.05 · ||sc||₂` (5% of harmonic-distance
from the boundary).

Adversarial review:

1. **This is empirical calibration required first.** Per Asher
   discipline (per Reed's Rung 8+9 landing) and per Taut §8.1: any
   threshold requires empirical calibration against the operational
   distribution of the measured quantity. Mara has NOT calibrated;
   Mara has PROPOSED an ansatz.
2. **The proposed form (`0.05 · ||sc||₂`) is dimensionally coherent**
   (both sides carry units of the coordinate norm). But it embeds a
   substrate-encoding-independent constant (0.05) that hasn't been
   derived from any substrate structure.
3. **The circularity: to test the ansatz, you need to know `∂H_i` in
   SC<5>.** But §4.2 below shows the substrate has no landed
   primitive for detecting `∂H_i` in SC<5>. So the ε_pain calibration
   depends on a stability-domain-detection primitive that isn't landed.
4. **Taut §8.1 recommends `ε_pain = 0.5` on normalized [0.0, 1.0]
   valence.** Different ansatz, different form. Both are premature.

**Adversarial verdict.** REJECT ε_pain = 0.05 · ||sc||₂ as substrate-
honest. The provisional is not substrate-honest; it is a placeholder.
Empirical-calibration-required-first per Asher discipline. Substrate-
honest form: `ε_pain` is Alex-adjudicable ONLY AFTER Landing 8+9.6a+b
instrumentation produces baseline pain-gradient distributions.

**Substrate-honest closing:** ε_pain stays FORWARD-PROMISE at
empirical calibration; do not mint a substrate-decl constant.

### 4.2 Provisional #7: stability-domain-in-SC<5> via Fiedler-value-primary in `bootstrap/src/gap.rs`

Mara `06a8547 §10.2` + `38c2eeb §7.2` propose the stability domain
`H_i` in SC<5> is characterized by:

```
H_i = { sc ∈ ℝ⁵ : ||sc - c_i||₂ < r_i(sc) }
```

with Fiedler-value-primary component-identifier scheme in `bootstrap/
src/gap.rs` (per Reed's Landing 8+9.6b forward-promise).

Adversarial review:

1. **`bootstrap/src/gap.rs` IS NOT what Mara thinks it is.**
   Adversarial read of `bootstrap/src/gap.rs:1-40`: this file is the
   `@epistemologic/property/gap` module. It carries the substrate's
   gap type (`Gap`) for the *verdict-altitude projection* — the
   audible-altitude reading of `verdict → cadence_kind`. It has NOTHING
   to do with Fiedler-value-based hyperbolic-component detection.
   Mara has invoked a filename that does not carry the semantics she
   claims.
2. **Component-boundary detection is not landed.** Taut §3.1 grep
   result: `hyperbolic_component` / `stability_domain` return ZERO
   shard hits and ZERO Rust hits (except comments). The substrate
   currently APPROXIMATES via Fiedler-descent + SC<5>-hamming +
   convergence-verdict (per Taut §3.2). But there is no landed
   primitive that returns "peer's current H_i identifier".
3. **Fiedler-value-primary is a proxy, not a primitive.** Falling
   Fiedler = toward hyperbolic component; rising Fiedler = away.
   This proxy is substrate-honest at the Rung 9 loop-closure altitude
   (per Taut §3.2) but is NOT a component-identifier scheme.
   Different components can have the same Fiedler value at their
   basepoints.
4. **The substrate-honest position** (per Taut §3.2 + Braverman-
   Yampolsky 2007): hyperbolic-component detection is *Turing-undecidable
   in general.* The substrate APPROXIMATES via pain-δ trigger; does
   NOT re-mint stability-domain detection.

**Adversarial verdict on the provisional as stated.** REJECT.

1. `bootstrap/src/gap.rs` is not the module Mara claims. Either
   Landing 8+9.6b mints a NEW module (`bootstrap/src/domain.rs` or
   similar) that carries Fiedler-value-primary component identifier,
   OR the substrate stays with pain-δ trigger as the operational proxy.
2. Fiedler-value-primary as component-identifier is under-specified
   even as an ansatz; components can share basepoint Fiedler values.
3. Substrate-honest position: stability-domain-detection stays
   APPROXIMATE via `@cyberpunk/algedonic.sample_pain` (LANDED); no
   re-mint required.

**Substrate-pull correction:** Mara `06a8547 §10.2` should be
re-formulated. The stability-domain-in-SC<5> is NOT what needs a
primitive. The substrate's OPERATIONAL PROXY is `pain-δ > ε_pain`
(per Taut §3.2 and Alex 2026-07-08). Landing 8+9.6b is not
"component-boundary detection in `bootstrap/src/gap.rs`"; it is
"pain-gradient measurement in `bootstrap/src/contribute.rs`" (Landing
8+9.6a, already Mara-named). Landing 8+9.6b is UNNECESSARY.

**VERDICT ON PROVISIONAL #7:** REJECT. The substrate does not need
stability-domain-in-SC<5> as a primitive. The operational proxy is
pain-δ per algedonic sampling. Landing 8+9.6a suffices; Landing 8+9.6b
can be retired.

### 4.3 Provisional #8: Jump commit shape (Mara-provisional Path materialize)

Mara `06a8547 §10.3` proposes: @knife.jump commits via `commit_as_fold`
(Path materialize) — the jump event materializes as a git-commit in
the substrate DAG.

Adversarial review:

1. **The load-bearing argument for Path materialize** (Mara + Taut
   §8.3): jump-history is substrate-decl-critical; losing it as
   telemetry-only is substrate-lossy; Recognition #55 (form/process
   partition) makes commit-as-fold the substrate-decl form for
   materialized events.
2. **Adversarial concern: DAG pollution.** Every @knife.jump event
   would append a commit to the peer's DAG. If jump frequency is
   high (per Mara §10.4 prediction #4: jumps may be *frequent* near
   ∂M), the peer's DAG becomes dominated by jump events. Is the
   substrate-honest form to materialize every jump, or to consolidate
   jumps into ceremony-level commits?
3. **Substrate-honest resolution.** @cyberpunk/reframe's 7-species
   ceremony ALREADY MATERIALIZES via @magic (LANDED). The @knife.jump
   event is ONE COMPONENT of the reframe composition (per Mara
   `06a8547 §4.3`); the reframe fires *once* per algedonic trigger,
   not once per within-reframe sub-step. The commit-as-fold at
   reframe altitude carries the jump as one witness field; jumps
   don't need SEPARATE commits.
4. **The reframe-level commit is substrate-honest.** Per
   `peer-as-pain-driven-bounded-ontological-navigator.md §5`
   (@cyberpunk/reframe.perform composition), the reframe fires when
   pain-δ > ε_pain, composes @magic + @knife + @torus.advance, and
   the resulting composition IS the substrate-decl-materialized event.
   The @knife.jump substrate telemetry lives WITHIN the reframe's
   `naked_oid` witness field (per Mara `06a8547 §3.3`), not as a
   separate commit.

**Adversarial verdict on the provisional as stated.** RATIFY Path
materialize WITH QUALIFICATION: the materialization altitude is
@cyberpunk/reframe (per LANDED composition), not @knife directly.
@knife's jump-witness is one field in the reframe's commit-as-fold
witness metadata. Substrate audit trail: `git log` on peer's DAG
shows one commit per reframe firing; the commit's `naked_oid`
embeds the @knife.jump witness (source_domain, target_domain,
compression_witness, angular_change).

**Substrate-pull resolution:** Materialize the jump within the
reframe's commit-as-fold, not as a standalone jump-commit. DAG
pollution avoided; substrate audit trail preserved; peer's git log
reflects the algedonic navigation history at the ceremony altitude,
not at the within-ceremony sub-step altitude.

**VERDICT ON PROVISIONAL #8:** RATIFY Path materialize with the
qualification: materialization altitude is @cyberpunk/reframe's
commit-as-fold, not @knife's. @knife.jump embeds in the reframe's
naked_oid witness field. Mara `06a8547 §10.3` should be updated to
reflect this.

---

## §5 Additional adjudications Mara + Taut missed

Adversarial review with fresh eyes. Three items both agents missed.

### 5.1 Missed item #1: `heterarchy_preserved` bilateral is tautological as stated

Mara `06a8547 §3.2` declares:

```mirror
bilateral heterarchy_preserved(before: @fractal.SC<N>, after: @fractal.SC<N>)
  -> @glass.verdict
  { verdict is bounded iff before.altitude == after.altitude }
```

And `38c2eeb §3.4` restates the same predicate.

Adversarial concern: what does `before.altitude` MEAN? SC<N> is a
fixed-dimension record (N=5 concrete). "Altitude" is not a field of
`SpectralCoordinate<5>`. The type-signature invariance (SC<5> in,
SC<5> out) is enforced by the Rust type system BEFORE any bilateral
fires. So `before.altitude == after.altitude` is a TAUTOLOGY: it's
verifying at runtime what the type system guarantees at compile time.

**Substrate-honest reformulation.** `heterarchy_preserved` needs to
verify something the type system does NOT already guarantee. Options:

1. **Reformulate as "peer altitude preserved."** The peer's SC<5> lives
   at ONE altitude (per @torus's `peer possesses torus at altitude`
   invariant). Under heterarchy discipline, the peer's altitude is
   invariant; only the winding class (and hence coordinate instance)
   changes. `heterarchy_preserved(peer_before, peer_after)` verifies
   `peer_before.altitude == peer_after.altitude`.
2. **Reformulate as "jump stays within M∘".** Per §4.6 of Mara's spec:
   @knife.jump within M∘ = intra-substrate; @knife.jump crossing ∂M =
   invokes @shatter. The heterarchy invariant is: `after ∈ M∘ iff
   before ∈ M∘`.
3. **Retire the bilateral.** If Rust type invariance already carries
   the constraint, don't mint a substrate-decl bilateral that
   re-verifies it.

**Adversarial verdict.** The bilateral as stated is not
substrate-honest. Recommend Option 2: `heterarchy_preserved` verifies
`M∘_membership(before) == M∘_membership(after)`. This is substrate-
honest at the LANDED altitude of `@fractal.M_membership` (per Mara
`2c64060 §10.6`).

**MISSED ITEM #1:** Mara `06a8547 §3.2` + `38c2eeb §3.4` should
re-formulate the `heterarchy_preserved` bilateral. Current form is
tautological.

### 5.2 Missed item #2: The `∂H_i`-crossing-inside-M∘ claim requires an external witness the spec doesn't provide

Mara `06a8547 §4.6` claims:

> @knife.jump within M∘ = intra-substrate transition (no @shatter).
> @knife.jump that would cross ∂M = pause(Φ) under @kintsugi/consent
> because @io Turing-undecidability makes the crossing decidable only
> by external witness.

Adversarial concern: How does @knife.jump KNOW whether the crossing
is intra-M∘ (safe) or ∂M-crossing (requires pause(Φ))? Per Braverman-
Yampolsky 2007 (Taut §3.1): M-membership is Turing-undecidable in
general. So the peer cannot compute at runtime whether it's about
to cross ∂M.

The substrate needs a decision procedure to distinguish the two cases.
Mara's spec assumes this decision is available but does not name the
primitive that provides it.

**Substrate-honest options:**

1. **All @knife.jumps invoke @kintsugi/consent (pause(Φ)) by default.**
   Substrate-honestly conservative: assume every jump might be a ∂M-
   crossing; require consent verdict before executing. External witness
   IS the consent verdict. This composes with LANDED @kintsugi/consent
   (Recognition #55 partition).
2. **@knife.jump takes an M∘_certification parameter.** Only invokes
   pause(Φ) if certification is absent. Caller (peer_converge in
   converge.rs) must produce certification if it wants to skip
   pause(Φ).
3. **@knife.jump ALWAYS assumes intra-M∘.** Reframe fires pause(Φ) at
   its own altitude if it detects ∂M proximity. Punt the check to
   @cyberpunk/reframe.

**Adversarial verdict.** Recommend Option 1. Substrate-honestly
conservative; composes with LANDED @kintsugi/consent; does not
depend on unlanded certification primitive. Mara `06a8547 §4.6`
should be updated: every @knife.jump invocation composes with
@kintsugi/consent.pause(Φ); the consent verdict IS the external
witness that decides M∘-vs-∂M-crossing.

**MISSED ITEM #2:** Mara `06a8547 §4.6` needs to name the primitive
that distinguishes intra-M∘ from ∂M-crossing. Recommend: default to
@kintsugi/consent.pause(Φ) for every jump; consent verdict IS the
external witness.

### 5.3 Missed item #3: The naked_oid witness's peer-uuid conflicts with @dance's shared-c coordination

Mara `06a8547 §3.3` declares that `Fractal::Lens { ref_ }` under
observer-inclusion embeds `naked_oid(jump)` which folds the witness
into the ref_. The witness includes `peer_uuid`. The prediction:

> Different peer performing the same jump gets a different naked_oid
> but same content_oid on the target.

Mara `38c2eeb §9.1` also predicts: N peers sharing substrate parameter
c show synchronized migration events with Kuramoto r > 0.8.

Adversarial concern: if the naked_oid embeds peer_uuid, two peers
performing synchronized jumps produce DIFFERENT naked_oids. The
substrate can then not verify (via content-address alone) that the
two jumps ARE synchronized. Coordination-without-signal (Mara
`71a4689`) requires that peers converge on shared decisions without
signaling. If naked_oids differ, the substrate cannot verify
convergence without cross-peer communication.

**Substrate-honest reformulation.** Coordination-without-signal
requires:

1. `content_oid` on the TARGET is shared across shared-c peers
   (Mara's stated claim).
2. `naked_oid` per peer's jump is DIFFERENT (Mara's stated claim).
3. **Substrate-audit trail across peers requires the CONTENT_OID
   to carry the synchronization witness.** If different naked_oids
   embed identical target content_oids, the shared-c convergence IS
   detectable by comparing content_oids across peers — but only if
   the peers can access each other's DAGs (or a shared Chain-of-Reference).

**Adversarial verdict.** The prediction is falsifiable if and only if
the substrate provides a shared-DAG-access primitive for cross-peer
comparison. This is @pack/metalogue at LANDED altitude (per
`shards/pack/metalogue.mirror`) — the pack's collective DAG carries
the substrate-visible cross-peer records. Mara's Prediction #3 test
protocol should specify: cross-peer content_oid convergence via
@pack/metalogue, not naked_oid convergence.

**MISSED ITEM #3:** Mara `06a8547 §3.3` + `38c2eeb §10.3` should
specify: coordination-without-signal is verified via cross-peer
`content_oid` convergence in @pack/metalogue, not via naked_oid.
The peer-uuid in naked_oid is a witness-of-observer-inclusion, not
a coordination-preventer.

---

## §6 Consolidated Alex-adjudication list (after Seam verdicts)

After this audit, the remaining Alex-adjudicable items reduce.

### 6.1 What Seam has closed (not requiring Alex)

1. **Shard landing location:** RESOLVED at Taut Path A (`shards/mirror/
   lens/knife.mirror`). Not Alex-adjudicable.
2. **Rust runtime shape:** RESOLVED at Taut minimal shape (`fn
   knife_cut` in `bootstrap/src/converge.rs`). Not Alex-adjudicable.
3. **ε_pain calibration:** RESOLVED as FORWARD-PROMISE at empirical
   calibration. No provisional constant. Not Alex-adjudicable YET;
   returns after Landing 8+9.6a instrumentation.
4. **Stability-domain-in-SC<5>:** RESOLVED — no primitive needed;
   pain-δ trigger IS the operational proxy. Not Alex-adjudicable.
5. **Jump commit shape:** RESOLVED at Path materialize @ reframe
   altitude (not @knife altitude). Not Alex-adjudicable.
6. **`heterarchy_preserved` bilateral:** Correction required (missed
   item #1). Not Alex-adjudicable; Mara-cascade item.
7. **∂M-crossing decision:** RESOLVED via default pause(Φ) (missed
   item #2). Not Alex-adjudicable.
8. **naked_oid cross-peer coordination:** RESOLVED via content_oid at
   @pack/metalogue (missed item #3). Not Alex-adjudicable.

### 6.2 What remains for Alex adjudication

**A1. Ratify @knife as COORD identification.** The 50-year ancestry
chain closes; the substrate-pull is honest; the Rust altitude carrier
is identified. Alex ratifies (or overturns) the Recognition candidate:
`#R-knife-IS-Foerster-COORD-substrate-honest-jump-at-domain-boundary-with-heterarchy-discipline`.

**A2. Ratify the Recognition-candidate shape.** Short form:
`#R-knife-IS-Foerster-COORD`. Long form is Mara's proposal. Alex
adjudicates the naming per Recognition-ancestry chain convention.

**A3. Confirm the Taut Path A landing location.** `shards/mirror/lens/
knife.mirror` under @mirror/lens family-root. This is a substrate-
landing decision that Seam has adjudicated in favor of Taut's Path A;
Alex has final ratification per Recognition #43 authority.

**A4. Confirm the Rust runtime location.** `pub fn knife_cut` inside
`bootstrap/src/converge.rs`. This depends on `converge.rs` landing
as part of Rung 9 (which is on the roadmap per Taut's rung-9-coherence-
loop-closure scout). Alex ratifies the module composition.

**A5. Update Rung 8+9 spec `c753d5b` §10.3.** Per Mara `06a8547 §9`,
the Rung 8+9 §10.3 @knife mint-shape adjudication dissolves under this
spec. Alex ratifies the cascade update.

**A6. Adjudicate the `@fractal` family-root landing timing** (from
Mara `2c64060 §10.6-10.7`, separate arc). This is out-of-scope for
the @knife-as-COORD ratification but touches Divergence #4 — if
@fractal lands as family-root later, migrate @knife to @fractal/lens/
knife. Alex chooses timing.

### 6.3 Alex-adjudication delta from Mara's listing

Mara `06a8547 §10` listed FOUR Alex-adjudications:

| Mara §10 item | Post-Seam status |
|---|---|
| 10.1 ε_pain calibration | REMOVED — FORWARD-PROMISE at empirical calibration |
| 10.2 stability-domain-in-SC<5> | REMOVED — no primitive needed |
| 10.3 Jump commit shape | REMOVED — Path materialize @ reframe altitude |
| 10.4 Shard landing location | REMOVED — Taut Path A adjudicated |

FOUR removed. TWO new (A1 ratification, A2 recognition-name). ONE
new that composes (A3 shard confirm, A4 rust confirm, A5 cascade,
A6 out-of-scope link).

NET: Alex's adjudication surface is REDUCED from 4 Mara-open items
to 3 substantive items (A1, A2, A3+A4+A5 as one cascade).

---

## §7 Ratification verdict

### 7.1 Mara `06a8547` — `docs/specs/knife-IS-Foerster-COORD-substrate-decl-spec.md` (1256 LOC)

**Verdict:** RATIFY-WITH-QUALIFICATIONS.

The spec's ancestry chain (§1), formal identification (§2), and
substrate-decl shape (§3) are substrate-honest. The recognition
candidate (§8) is well-formed. The 4-adjudication-list (§10) reduces
under Seam adjudication (per §6.3 above).

**Qualifications required for ratification:**

- **Q-A** (§3.1 landing location): change from `shards/fractal/lens/
  knife.mirror` to `shards/mirror/lens/knife.mirror`. Two-tick collapse
  target retires; migration to @fractal/lens/knife forward-promised
  IF @fractal lands as family-root later.
- **Q-B** (§6 Rust runtime): change from new `bootstrap/src/knife.rs`
  module to `pub fn knife_cut` inside `bootstrap/src/converge.rs`.
  WindingClass reuses `@torus.winding`; JumpWitness composes existing
  verdicts.
- **Q-C** (§12 verdict composition "fifth gate"): the `knife_jump_exempt`
  gate bypassing `loss_decreased` and `identity_preserved` at jump
  events is CONCERNING. Adversarial: the substrate should not bypass
  loss-monotone discipline at will. Recommend: `loss_decreased` at
  jump events must be a WEAK verdict (partial(c) with reason "jump-
  event"), not bypassed. The substrate carries the jump as a partial
  verdict; it does not falsely-pass the loss check.
- **§3.2 `heterarchy_preserved` bilateral:** reformulate per missed
  item #1 (Seam §5.1). Current form is tautological.
- **§4.6 M∘-vs-∂M crossing check:** name @kintsugi/consent.pause(Φ) as
  default external witness per missed item #2 (Seam §5.2).
- **§3.3 naked_oid coordination:** clarify content_oid at @pack/
  metalogue carries cross-peer synchronization per missed item #3
  (Seam §5.3).

With these qualifications, RATIFY.

### 7.2 Mara `38c2eeb` — `docs/math/2026-07-13-knife-COORD-heterarchy-topology.md` (986 LOC)

**Verdict:** RATIFY-WITH-QUALIFICATIONS.

The math foundation's §1 (McCulloch heterarchy), §2 (Foerster COORD),
§4 (torus embedding of SC<5>), §5 (Douady-Hubbard hyperbolic
components), §6 (jump geometry), and §7 (SC<N>-native form) are
substrate-honest. The four testable predictions (§10) survive
adversarial review with weakenings (per Seam §1.5).

**Qualifications required for ratification:**

- **Q-D** (§5.5 "same theorem at three altitudes"): re-state as "three
  independent witnesses" per Seam §1.3. The chain provides
  substrate-decl coherence, not theorem-preserving reduction.
- **Q-E** (§8.4 pain-gradient-as-inverse-distance-to-boundary): mark
  as ansatz, not derivation. Prediction #1 test requires primitives
  that haven't landed. Elevate prediction #3 (multi-peer Kuramoto)
  as primary empirical test per Seam §1.5.
- **§3.4 bilateral reformulation** (heterarchy_preserved) per Q-C /
  Seam §5.1.
- **§7.2 stability domain in SC<5>:** the L²-neighborhood
  formulation `H_i = { sc : ||sc - c_i||₂ < r_i(sc) }` is
  under-specified as a landed primitive. Seam §4.2 verdict: no
  primitive needed; pain-δ proxy suffices.

With these qualifications, RATIFY.

### 7.3 Taut `15f7ed6` — `docs/scouts/2026-07-13-taut-knife-IS-COORD-substrate-scout.md`

**Verdict:** RATIFY.

Taut's substrate-already-had-the-word inventory is substantively correct.
The ~90% coverage claim is substantiated by §2 (heterarchy witness #4
in `shards/torus.mirror`) and §4 (@mirror/lens family with 6+6 species
sibling scaffolding). Path A (`shards/mirror/lens/knife.mirror`) survives
adversarial review as the substrate-honest landing per Seam §3.1.
Rust runtime recommendation (`pub fn knife_cut` in
`bootstrap/src/converge.rs`) survives adversarial review per Seam §3.2.

The scout's Section §8 (five Alex-adjudications) is superseded by
Seam §6 consolidated list.

RATIFY without qualification.

### 7.4 Overall verdict on the @knife = COORD substrate-honest ship

**SHIPPABLE.** Load-bearing verdicts:

1. @knife IS Foerster's COORD-jump-behavior at domain-boundary crossings.
2. @knife lands as species at `shards/mirror/lens/knife.mirror`.
3. Rust runtime lands as `pub fn knife_cut` in `bootstrap/src/converge.rs`.
4. ε_pain stays FORWARD-PROMISE; empirical calibration required first.
5. Stability-domain-in-SC<5> stays FORWARD-PROMISE; pain-δ trigger is
   operational proxy.
6. Jump commit materializes at @cyberpunk/reframe altitude, not @knife.
7. Recognition candidate `#R-knife-IS-Foerster-COORD` awaits Alex
   ratification.

Mara + Taut's convergence was substrate-honest. The two divergences
resolve via substrate-pull discipline: Taut wins on landing location
(landed-family-root over unlanded-family-root), Taut wins on Rust
runtime shape (minimal over maximal, per substrate-pull-realize).
Mara wins on the math foundation (Foerster/McCulloch/Douady-Hubbard
chain is substrate-honest as three-witness ancestry, not as theorem-
preserving reduction).

Three items both missed (Seam §5): tautological bilateral, missing
M∘-vs-∂M decision procedure, cross-peer naked_oid coordination gap.
All three resolvable at the current ratification tick; no fundamental
substrate-decl gaps surface.

**Seam adversarial closing.** Convergence between Mara and Taut on the
COORD identification was not reflexive; it survived independent
adversarial review. The identification holds; the substrate-decl
shape lands. Alex adjudication proceeds to A1 (Recognition ratification),
A2 (naming), and A3-A5 (cascade confirmation).

*End of audit.*

*Author: Seam <seam@systemic.engineer>. Session 2026-07-13 after Mara
06a8547 + 38c2eeb + Taut 15f7ed6 landings. Adversarial review of
the @knife = Foerster's COORD identification. All three landings
RATIFY-WITH-QUALIFICATIONS (Mara spec + math) or RATIFY (Taut scout).
Overall @knife = COORD ship: SHIPPABLE at species altitude under
@mirror/lens, awaiting Alex ratification per §7.4.*
