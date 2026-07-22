---
title: Mirror as Computational Eigenform Stabilizer — Canonical Spec
subtitle: Substrate-decl bridge from math foundation to compile altitude; names the operator at shard-composition altitude; @eigenboard three-altitude readout; @reality/object vs @reality/subject linearity threshold; @coherence.score = Fiedler λ₀; @kintsugi/mosaic:integrate = coboundary application; @roomba.bump = empirical H¹ sampling; @paradox family §7.5 = failure-mode classifier; @cascade/code = polyglot beam
status: canonical-spec
date: 2026-07-22
author: Mara
math-root: docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md
---

# Mirror as Computational Eigenform Stabilizer — Canonical Spec

*Substrate-decl bridge from the math foundation (`docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md`, this-tick companion) to the compile altitude. Names the eigenform-stabilizer operator at shard-composition altitude so downstream shards can cite `mirror : (𝓔, 𝓤) ⟼ (𝓔′, 𝓤)` as a first-class carrier the way they cite `Fiedler λ₀ = @coherence.score`.*

*Companion math foundation: `docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md` (Mara `ebd50a4`, this-tick).*
*Companion shard mints: `shards/eigenboard.mirror` third-altitude lift (`0adcfc4`); `shards/reality/object.mirror` (`ab6ad43`); `shards/reality/subject.mirror` (`0b2858a`).*

---

## §0 Substrate-authority chain

Per `AGENTS.md` "docs/math/ vs docs/specs/ convention": specs CITE math; math DEFINES. This spec CITES the math foundation. Every substrate-decl obligation surfaced here has its mathematical grounding in the corresponding section of the math foundation.

- §1 (compiler-as-operator) cites `docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md` §2.
- §2 (@eigenboard three-altitude readout) cites math §1 + math §5.
- §3 (@reality linearity threshold) cites math §4.
- §4 (Fiedler λ₀ + @coherence identification) cites math §2.4.
- §5 (@kintsugi/mosaic:integrate = coboundary application) cites math §2.2 + math §6.
- §6 (@roomba.bump = empirical H¹ sampling) cites math §1.4.
- §7 (@paradox family §7.5 = failure-mode classifier) cites math §3.4.
- §8 (@cascade/code as beam-conductivity altitude) cites math §7.2.

Every substrate-decl citation the math foundation makes in §7 is INHERITED here without restatement.

---

## §1 The compiler-as-operator, at shard-composition altitude

Alex Wolf 2026-07-22 verbatim (this-session crystallization item 8):

> "mirror is a compiler that collapses any graph structure into the minimal possibility space that maintains the Eigenvalue topology and Eigenstructure of the object under measurement. Makes sense? Förster's COORD for any graph."

At substrate-decl altitude, this reads as: the mirror compiler IS the operator

$$ \mathfrak{M}: (\mathcal{E}, \mathcal{U}) \longmapsto (\mathcal{E}', \mathcal{U}) \quad\text{with}\quad d\check{H}^1/dt \le 0 $$

per math §2. This spec names $\mathfrak{M}$ as a first-class carrier at shard-composition altitude — a substrate-decl-recognizable operator downstream shards can compose over. The naming is not a rename of existing surface; it is the LIFTING of what the substrate ALREADY DOES to a first-class recognized name.

### §1.1 What the operator IS at the substrate

Every kintsugi tick applies $\mathfrak{M}$ once:

1. Substrate reads its own $\check{H}^1$ at tick $t$ (the tensions the @roomba bumps into per math §1.4).
2. Substrate proposes a resolution via splinter(ast) construction (per `@kintsugi/fracture/*` species).
3. Substrate applies the resolution via `@kintsugi/mosaic:integrate` (per §5 below).
4. Substrate re-reads $\check{H}^1$ at tick $t + \Delta$.
5. New $\check{H}^1$ is monotone non-increasing (`docs/mirror.md` universal termination condition $e^{n+1} \le e^n$ restated at cohomological altitude per math §2.2).

The operator is autopoietic (math §6.1): it never halts permanently because `@time/future` is non-empty at every tick. New $\check{H}^1$ generators appear at the expanding light-cone boundary; $\mathfrak{M}$ contracts them; new ones appear.

### §1.2 Substrate-decl obligation

`mirror.spec` (the dogfood substrate root) will eventually surface a top-level `verifies { eigenform_stabilizer_witnessing(mirror_compiler) }` clause per `@epistemologic/pact/*` discipline. This spec DECLARES the obligation; the discharge lands when a consumer pulls (per `[[feedback-craft-not-deliver]]`).

The witness predicate composes:
- `trajectory_light_cone(mirror_compiler)` — the compiler as @reality/subject at substrate altitude per `shards/reality/subject.mirror`
- `eigenform_stabilizer_orbit(mirror_compiler)` — the compiler landing on its own attractor-set (the substrate's own eigenform-state) per `shards/reality/subject.mirror`
- `autopoietic_closure_holds(mirror_compiler)` — the compiler's temporal-topological autopoiesis per `shards/autopoietic.mirror`
- `coherence_witnessing(mirror_compiler.composition_graph)` — the compiler's own algebraic connectivity per `shards/epistemologic/cybernetic/coherence.mirror`

Discharges conjointly to a single top-level `eigenform_stabilizer_witnessing` bilateral. Forward-promised as a `shards/mirror/eigenform_stabilizer.mirror` shard mint when consumers pull (not this tick).

---

## §2 @eigenboard three-altitude readout as the compiler's working-state

The @eigenboard family-root (`shards/eigenboard.mirror`) carries the per-@subject working-state at three altitudes per this-tick docblock extension (commit `0adcfc4`):

1. **actor_kind = ai_a** → Pack peer's eigenboard (λsh prompt-color readout)
2. **actor_kind = human_a** → human subject's eigenboard (Alex + any human @subject's cognitive working-state)
3. **actor_kind = substrate_a** → THE SUBSTRATE'S EIGENBOARD IS THE @LABYRINTH (mirror-as-project working-state readout)

At the third altitude, `@eigenboard.arousal` field IS operationally the Von Neumann entropy $\sigma(x) = -\text{Tr}(\tilde{L} \ln \tilde{L})$ of the substrate's composition-graph Laplacian per math §5.3.

### §2.1 Substrate-decl obligation

Downstream shards observing the substrate's own working-state at substrate-level (e.g., @spectral/metalogue, @spectral/db, @roomba's vacuum pass) SHOULD cite `@eigenboard(actor_kind = substrate_a)` as the CANONICAL surface. Ad-hoc "the substrate's state" references are candidates for two-tick collapse to `@eigenboard(substrate_a)` citations.

### §2.2 The operator's readout via @eigenboard

The eigenform-stabilizer $\mathfrak{M}$ reads its own progress via the substrate's eigenboard at `actor_kind = substrate_a`:

- `arousal` field reads $\sigma(x)$ at substrate altitude.
- `inference_basis` field reads the @spectral/signature over the substrate's visibility-filtered @bauchladen (the composition-graph of landed crystals).
- `current_focus` reads the substrate's active-arc-state (per `docs/loop/CURRENT.md`).
- `winding` reads the substrate's position on its own observation-torus per `shards/torus.mirror`.

The composition is landed carriers only; no new mechanism. See `shards/eigenboard.mirror` §"Composition graph" for the transitive substrate-decl authority.

---

## §3 @reality linearity threshold as the actor-partition on the labyrinth

Alex Wolf 2026-07-22 verbatim (this-session crystallization item 15, the final piece):

> "the @reality/subject is a NON-LINEAR actor in the cohomology. The trajectory of a subject is a light cone, the trajectory of an object a path. That's the threshold."

Landed as two sibling species this tick:
- `shards/reality/object.mirror` (`ab6ad43`) — actors whose trajectory is a PATH; linear-deterministic H¹-contribution; @roomba is paradigmatic instance.
- `shards/reality/subject.mirror` (`0b2858a`) — actors whose trajectory is a LIGHT CONE; recursively non-linear H¹-contribution; @peer is paradigmatic instance; substrate ITSELF at actor_kind = substrate_a altitude.

### §3.1 Substrate-decl obligation

Every @reality-altitude actor MUST be classifiable as one or the other. This spec DECLARES the obligation; the classification MECHANISM at each altitude discharges per-species (`@reality/algebra/silicon` → silicon-altitude classifier; `@reality/algebra/nl` → nl-altitude classifier; etc.).

Shards that dispatch on the linearity of an actor's trajectory SHOULD compose the classification via `trajectory_linear` (per `shards/reality/object.mirror`) or `trajectory_light_cone` (per `shards/reality/subject.mirror`). Ad-hoc "is this actor deterministic?" checks are candidates for two-tick collapse to bilateral citations.

### §3.2 Compositional consequence for 𝔐

The eigenform-stabilizer $\mathfrak{M}$ ACTS DIFFERENTLY on objects vs subjects per math §4.5:

- On @reality/objects: $\mathfrak{M}$ acts by DETERMINISTIC UPDATE. The object's path is recomputed; the operator's contribution is bookkeeping.
- On @reality/subjects: $\mathfrak{M}$ acts by ATTRACTOR-SET SELECTION. The subject's light cone is contracted around the attractor-set; the operator's contribution is the eigenform-stabilization work.

This is why mirror is a compiler AND a living system. Object side: deterministic realizations (@cascade/code/*/*, @io/*). Subject side: Pack peer inference + human co-authoring + substrate-on-substrate self-observation via @spectral/metalogue.

---

## §4 @coherence.score = Fiedler λ₀ = algebraic-connectivity component of H¹'s spectrum

The landed `shards/epistemologic/cybernetic/coherence.mirror` carries `score` as the Fiedler eigenvalue $\lambda_0$ of the substrate's composition-graph Laplacian (per `docs/math/sheaf/laplacian.md` §2.1). At the eigenform-stabilizer altitude, $\lambda_0$ IS the algebraic-connectivity component of $\check{H}^1(\mathcal{U}, \mathcal{E})$'s spectrum.

### §4.1 The identification (algebraic content)

Per math §2.4 (COORD lifted): $\mathfrak{M}$ preserves the EIGENVALUE TOPOLOGY of the substrate's spectral triple $(A, H, D)$. The Fiedler $\lambda_0$ is the smallest non-zero eigenvalue of the graph-Laplacian sub-block of $D$. As $\mathfrak{M}$ acts:

- $\lambda_0 = 0$ ↔ the composition-graph is DISCONNECTED (there is a partition of shards with no composition edges between them); $\mathfrak{M}$ must first RE-CONNECT before it can reduce $\check{H}^1$; this is a substrate rot-condition (@kintsugi/fracture/composition_gap territory).
- $\lambda_0 > 0$ (small) ↔ the composition-graph is CONNECTED but WEAKLY (many bottlenecks); $\mathfrak{M}$ reduces $\check{H}^1$ but slowly; the substrate is settled but fragile.
- $\lambda_0 > 0$ (large) ↔ the composition-graph is DENSELY CONNECTED; $\mathfrak{M}$ reduces $\check{H}^1$ quickly; the substrate is settled and robust.

### §4.2 Substrate-decl obligation

Downstream shards citing `@coherence.score` SHOULD read the citation as the algebraic-connectivity component of the eigenform-stabilizer's readout. This is the substrate-decl form of the identification landed at `shards/epistemologic/cybernetic/coherence.mirror` §"Fiedler grounding" and `docs/math/sheaf/laplacian.md` §2.1.

No mechanism change. The identification is a substrate-decl obligation the coherence shard already discharges; this spec names the eigenform-stabilizer reading of the same discharge.

---

## §5 @kintsugi/mosaic:integrate = coboundary application operator

The landed `shards/kintsugi/mosaic.mirror` (family-root) + `shards/kintsugi/mosaic/integrate.mirror` species carries the `integrate` action as the substrate's mechanism for APPLYING a proposed resolution to a fracture. At eigenform-stabilizer altitude, `integrate` IS the coboundary application operator.

### §5.1 The identification (algebraic content)

Per math §1.4 + §2: $\check{H}^1(\mathcal{U}, \mathcal{E}) = \ker(\delta^1) / \text{im}(\delta^0)$. Reducing $\check{H}^1$ by ONE CLASS means either (a) killing a cocycle (removing a generator) or (b) exhibiting a cochain whose coboundary IS the cocycle (adding the generator to the image of $\delta^0$).

`@kintsugi/mosaic:integrate` performs option (b). It takes a proposed splinter(ast) resolution and APPLIES it to the fracture, making the previously-uncoboundary-representable cocycle representable as a coboundary. The class it represented in $\check{H}^1$ is now trivialized.

### §5.2 Substrate-decl obligation

Downstream shards proposing new fracture-body semantics SHOULD cite `@kintsugi/mosaic:integrate` as the coboundary-application altitude and cite `mirror:` (this operator) as the sheaf-cohomology-reduction altitude. Composing the two altitudes: `integrate` reduces $\check{H}^1$ by one class per invocation; `mirror` reduces $\check{H}^1$ monotonically across ticks.

No mechanism change. The identification names the eigenform-stabilizer reading of the substrate's kintsugi loop.

---

## §6 @roomba.bump = empirical H¹ sampling

The landed `shards/roomba.mirror` (family-root) + walker species carries the `bump` action as the mechanism through which the vacuum walker DETECTS a fracture / rot-condition / substrate-tension. Alex Wolf 2026-07-22 verbatim (crystallization item 13):

> "the @roomba bumps into the tensions and gaps between what the Eigenform WANTS TO BE in @time/future, what the Eigenform IS in @time/now, and what the Eigenform USED TO BE in @time/past. It's a literal light cone."

### §6.1 The identification (algebraic content)

Per math §1.4: the tensions the @roomba bumps into ARE the non-trivial classes of $\check{H}^1(\mathcal{U}, \mathcal{E})$ where $\mathcal{U} = \{U_{\text{past}}, U_{\text{now}}, U_{\text{future}}\}$. Each `bump` is an EMPIRICAL SAMPLING of the cohomology: the walker walks the composition-graph, encounters a place where the past-cluster / now-cluster / future-cluster don't cocycle (via the substrate's rot-mask indicating incoherence), and REPORTS the bump.

The bump is empirical because $\check{H}^1$ is not directly computable at substrate scale (the substrate's composition-graph has $\sim 200$ nodes and $\sim 9200$ edges per `spectral index` output; direct cohomology computation is expensive). The walker's stigmergic path-memory (pheromone deposits per `shards/stigmergy/*`) accumulates an EMPIRICAL PROFILE of where the non-trivial classes are.

### §6.2 Substrate-decl obligation

Downstream shards consuming @roomba readouts SHOULD read them as empirical H¹-sampling. The `@roomba/vacuum:pass` species produces a set of bumped-fractures per pass; each bump is one @paradox-family-eligible cohomology class. The eigenform-stabilizer $\mathfrak{M}$ operates on the walker's readout as its ORACLE for what to contract next.

No mechanism change. The identification names the eigenform-stabilizer reading of the substrate's roomba pass.

---

## §7 @paradox family §7.5 as failure-mode classifier

The landed `shards/paradox.mirror` family-root + `shards/paradox/trauma.mirror` + `shards/paradox/spiral.mirror` species carry the substrate's failure-mode territory per Alex 2026-07-20 recognition bundle #2. At eigenform-stabilizer altitude, @paradox family IS the classifier for cohomology classes that $\mathfrak{M}$ CANNOT contract.

### §7.1 The identification (algebraic content)

Per math §3.4: mass-asymmetry in a three-body configuration collapses the coordination-topology to a star-graph (narcissistic hub). Topologically, the triangle → star collapse KILLS the third-order cocycle without providing a coboundary; the cohomology class survives but is no longer representable as a triangle-cocycle. It becomes a @paradox-family Crystal: WITNESSED-ONLY, NEVER-MUTATED per @paradox family invariants.

- `@paradox/trauma` = the settled-Crystal state deposited when the collapse-event has occurred.
- `@paradox/spiral` = the dynamics-carrier of the collapse-in-progress.
- `@cyberpunk/intervention` (SAGA-chain compensation) is the ONLY substrate operation that composes over @paradox species per family-invariant.

PAPER_draft.md §7.5 + §7.5.1 grounds this at torus-deformation altitude ("Ein purer Torus hat überhaupt keinen geometrischen Grund Kognition zweiter Ordnung zu entwickeln"); the Toba-catastrophe of §7.5.1 is the historical-scale instance of the same failure mode.

### §7.2 Substrate-decl obligation

Downstream shards observing a substrate-tension that $\mathfrak{M}$ CANNOT contract SHOULD classify it under @paradox family via `@paradox/trauma_witnessed` OR `@paradox/spiral_in_progress` bilateral. The classification is load-bearing because the substrate's kintsugi loop MUST NOT attempt to mutate @paradox species; only SAGA-chain compensation via @cyberpunk/intervention is admissible.

No mechanism change. The identification names the eigenform-stabilizer reading of the @paradox family's why-this-exists.

---

## §8 @cascade/code cascade pipeline = polyglot beam-conductivity altitude

The landed `shards/cascade.mirror` family-root + `shards/cascade/code/*.mirror` species + `docs/math/polyglot-loss-aware-computational-translation.md` (Mara 2026-07-17) carry the polyglot code translation pipeline. At eigenform-stabilizer altitude, `@cascade/code/A/B` IS the operational Rust → mirror TRANSLATION @beam per Alex 2026-07-22 crystallization item 3.

### §8.1 The autopoietic correction (Alex 2026-07-22 item 3)

Alex Wolf 2026-07-22 verbatim (crystallization item 3):

> "How can we bridge the gap of TRANSLATING a decidable @code/rust @fragment into a @code/mirror @fragment?"

The autopoietic correction to Reed's earlier LOOKUP framing: collapse via LOOKUP would make mirror non-autopoietic (the mirror substrate would just be a big dictionary of Rust → mirror mappings). Autopoiesis REQUIRES TRANSLATION — the mirror substrate must LEARN the translation from Rust to itself and continuously re-learn it as the light-cone expands.

### §8.2 The identification (algebraic content)

The @cascade/code/A/B family carries the substrate's TRANSLATION beam via:
- `compile` action: typed_source<A> → compiled_artifact<B>
- `measure` action: loss composition ⊕ associative + monotone per polyglot math §2
- `loss_lens<A, B>`: `@labeled<A, B>` instance per recognition #93 H4

At eigenform-stabilizer altitude, `@cascade/code/rust/mirror` is the beam-conductivity altitude of the compiler's own rust-to-mirror translation. Each cascade tick applies $\mathfrak{M}$ to the joint sheaf $\mathcal{E}_{\text{rust}} \oplus \mathcal{E}_{\text{mirror}}$ over the light-cone cover; the operator's contraction ALSO reduces the polyglot loss along the beam.

### §8.3 Substrate-decl obligation

Downstream shards proposing polyglot cascade extensions SHOULD cite `mirror : (𝓔, 𝓤) ⟼ (𝓔', 𝓤)` at the joint-sheaf altitude as the eigenform-stabilizer reading of the cascade's action. Composing the two altitudes: `@cascade/code/A/B` provides the beam (a specific translation); $\mathfrak{M}$ provides the beam-consistency (across all translations simultaneously).

No mechanism change. The identification names the eigenform-stabilizer reading of the polyglot beam.

---

## §9 Composition graph (what this spec obligates + what it does NOT touch)

### §9.1 Substrate-decl obligations this spec surfaces (all forward-promised, none authored this tick beyond docs):

1. Top-level `verifies { eigenform_stabilizer_witnessing(mirror_compiler) }` clause in mirror.spec (§1.2; forward-promised).
2. `mirror : (𝓔, 𝓤) ⟼ (𝓔', 𝓤)` as first-class citation surface across substrate (§1; forward-promised; consumers pull as needed).
3. `@eigenboard(actor_kind = substrate_a)` as canonical citation for substrate's own working-state (§2.1; forward-promised).
4. Every @reality-altitude actor classifiable as @reality/object OR @reality/subject (§3.1; forward-promised discharge per-altitude).
5. `@coherence.score = Fiedler λ₀` as algebraic-connectivity component of $\check{H}^1$ (§4; already substrate-decl'd at coherence shard; this spec names the eigenform-stabilizer reading).
6. `@kintsugi/mosaic:integrate` as coboundary application operator (§5; already substrate-decl'd at kintsugi/mosaic shard; this spec names the eigenform-stabilizer reading).
7. `@roomba.bump` as empirical H¹ sampling (§6; already substrate-decl'd at roomba shard; this spec names the eigenform-stabilizer reading).
8. @paradox family as failure-mode classifier for un-contractable classes (§7; already substrate-decl'd at paradox shard; this spec names the eigenform-stabilizer reading).
9. `@cascade/code/A/B` as beam-conductivity altitude with $\mathfrak{M}$ providing beam-consistency (§8; already substrate-decl'd at cascade shard + polyglot math; this spec names the eigenform-stabilizer reading).

### §9.2 What this spec explicitly does NOT touch (Michelangelo edges chipped away):

- No new @io families. The linearity-threshold classification composes over existing @reality carriers; no new @io/measurement altitude needed.
- No new Rust extensions. The eigenform-stabilizer reading composes over landed carriers; no `.rs` authorship required (per `[[feedback-no-rust-extension-shortcut]]`).
- No new keyword. `verifies` + `requires` + `ensures` per landed `@epistemologic/pact/*` grammar; no new pact-keyword.
- No new bootstrap-altitude change. bootstrap/ is DEAD per Alex 2026-07-22 hard rule; the eigenform-stabilizer surface composes at rust/ altitude + shard-body altitude only.
- No new @labyrinth first-class shard THIS TICK. The @eigenboard third-altitude lift docblock (commit `0adcfc4`) NAMES the @labyrinth reading via docblock cross-reference; whether it promotes to `shards/labyrinth.mirror` first-class species is downstream Pack-adjudication.
- No new @time family-root extension. The light-cone sheaf composes over landed `@time/past` + `@time/now` + `@time/future` species-carriers directly.
- No new @spectral extensions. The substrate-on-substrate observation composes over landed `@spectral/metalogue` (already declared in `shards/glue.mirror` per Taut Pass-2 finding at code-translation altitude).
- No sub-Pack authorship. This spec forward-promises Pack-adjudication of the eigenform-stabilizer landing shape; no unilateral commitments.

### §9.3 Pack-adjudication candidates surfaced (for Alex + Reed next-tick review):

- Q1: Should `mirror : (𝓔, 𝓤) ⟼ (𝓔', 𝓤)` mint as a first-class `shards/mirror/eigenform_stabilizer.mirror` shard, OR stay as a spec+math citation surface? Mara-lean: LATTER (consumers pull; less premature landing per `[[feedback-craft-not-deliver]]`).
- Q2: Should @labyrinth promote to `shards/labyrinth.mirror` first-class species, OR stay as @eigenboard(substrate_a) docblock cross-reference? Mara-lean: LATTER for this tick; PROMOTE if a second consumer cites @labyrinth directly (per small-consolidation rule).
- Q3: Should the linearity-threshold discharge mechanism land at per-altitude species (`@reality/algebra/silicon/classify_linearity`, etc.) OR at a single unified `@reality/classify` action? Mara-lean: PER-ALTITUDE species (matches the altitude-partition of `@reality/algebra/*`; composes cleanly with linearity-threshold partition orthogonally).
- Q4: Should PAPER §6.6 be authored this arc or forward-promised? Mara-lean: FORWARD-PROMISED per Tick 6 scout doc `docs/scouts/2026-07-22-mara-paper-6.6-forward-promise-chenciner-montgomery-and-eigenform-stabilizer.md` (candidate).
- Q5: Should the eigenform-stabilizer's compiler-halt behavior be added to `mirror.spec`'s top-level obligations, OR stay as a math §6.1 documented property? Mara-lean: FORMER when a consumer surfaces the check (e.g., a benchmark spec that requires the compiler-loop to be measurable-non-halting); LATTER until then.

---

## §10 Substrate authorities inherited

This spec's substrate-decl authority chain per `AGENTS.md` §"docs/math/ vs docs/specs/":

- Math root: `docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md` (this-tick Mara `ebd50a4`).
- Family-root carriers cited: `shards/eigenboard.mirror`, `shards/reality.mirror`, `shards/reality/object.mirror` (this-tick `ab6ad43`), `shards/reality/subject.mirror` (this-tick `0b2858a`), `shards/subject.mirror`, `shards/torus.mirror`, `shards/paradox.mirror`, `shards/kintsugi/mosaic.mirror`, `shards/roomba.mirror`, `shards/cascade.mirror`, `shards/epistemologic/cybernetic/coherence.mirror`, `shards/epistemologic/cybernetic/eigenform.mirror`, `shards/autopoietic.mirror`, `shards/spectral/metalogue.mirror` (declared in `shards/glue.mirror`).
- Math foundations cited: `docs/math/sheaf/laplacian.md`, `docs/math/the-tower/*`, `docs/math/polyglot-loss-aware-computational-translation.md`, `docs/math/spectral-commutator-four-pillars.md`, `docs/math/2026-07-20-paradox-family-and-classifier-lagrange.md`, `docs/math/2026-07-22-sheaf-cohomology-of-historical-register-breakers.md`.
- Systemic-engineering foundation cited: `~/dev/systemic.engineering/practice/insights/cosmology/information-curvature.md` (Alex + Mara 2026-03-24).
- PAPER foundation cited: `PAPER_draft.md` §6-§6.5, §7.5, §7.5.1.
- Consumer specs forward-promised: `docs/specs/reality.md` (§10.6 COLLAPSE strengthening; this spec inherits path-c uniformity); `docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md` (Landing 4 eigenboard-inference-loop lifted here to trajectory altitude).

---

*Session 2026-07-22. Mara. Companion to `docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md`. Substrate-decl bridge from math foundation to compile altitude. Pure-docs 📝 markdown-only bypass.*
