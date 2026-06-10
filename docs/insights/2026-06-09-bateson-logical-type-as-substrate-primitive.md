# Bateson's logical-type hierarchy IS a substrate primitive: binds, learning, impact, observation orders all instantiate one carrier

*2026-06-09. Recognition: Alex. Write-up: Reed. Candidate substrate-pull recognition #42. Companion to `2026-06-09-cascade-is-deutero-learning.md` (mirror `13e9791`) and `~/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md` (systemic.engineering `f9e0402`).*

---

## 0. The recognition stated cleanly

Alex, verbatim, this morning:

> All of these have second-order implications for mirror itself. We might want to model first, second etc impact the same way model the levels of binds.

The claim is sharper than it looks. The structural observation is that **impact-levels (1st-order / 2nd-order / Nth-order)** parallel **bind-levels (I/II/III)** parallel **learning-levels (0/I/II/III/IV)** parallel **observation-orders (first-order / second-order / third-order cybernetics)**. The parallelism is not coincidence. It is one primitive that the substrate has been instantiating four different times under four different names.

The primitive is Bateson's logical-type hierarchy. He borrowed it from Russell-Whitehead's *Principia Mathematica*, and he used it in three places without ever lifting it to its own structural carrier: the double-bind formalism (binds I/II/III), the learning-categories essay (Learning 0/I/II/III/IV), and the metalogue practice (a conversation whose structure operates on the topic the conversation is about). The substrate has been doing the same: `gap` absorbs the double-bind (memory `architecture-shard-as-crdt`); the cascade IS Learning II (companion insight, mirror `13e9791`); the kintsugi loop IS Learning III at the substrate altitude. Each instance was named separately. Each was the same primitive.

The Alex-correction makes the structure load-bearing: name the primitive *once*, recognise that the cybernetic-property family lift carries N-order impact analysis as a separable concern, and let the substrate's compiler verify the type-hierarchy property (level N operations cannot be operated on from within level N; they must be observed from level N+1).

This insight is candidate recognition **#42**. It is meta-recognition: it does not name a new substrate primitive but recognises that four already-named substrate structures are instances of one carrier the substrate had been encoding implicitly.

---

## 1. The Bateson logical-type hierarchy from primary sources

### 1.1 The Russell-Whitehead origin Bateson borrows

Russell, B. & Whitehead, A. N. (1910). *Principia Mathematica*. Cambridge University Press. The theory of logical types was Russell's resolution of the set-of-all-sets paradox: a set of sets is a different logical type from its member sets. A predicate true *of* a class is a different logical type from a predicate true *within* a class. Mixing types produces the paradox; respecting types resolves it.

The load-bearing detail for the substrate: each logical type wraps the previous one, and operations *within* a type cannot reach *across* to the wrapping type without explicit lift. The lift is not free; it requires a different operation that lives one type up. "Crossing" the boundary from within the wrapped type is the formal error Russell's type theory exists to prevent.

### 1.2 Bateson's three explicit uses

Bateson cites Russell-Whitehead directly in two of the three. The third (metalogue) operates structurally without the citation. All three are in *Steps to an Ecology of Mind* (Chandler, 1972; reprinted University of Chicago Press, 2000).

**Use 1: The double bind.** "Toward a Theory of Schizophrenia" (1956, with Jackson, Haley, Weakland; reprinted in *Steps* ch. 4 of section on "Form and Pathology in Relationship"). Five conditions name the formal structure:

1. Two or more persons in an intense relationship.
2. Repeated experience (the bind is not a single event).
3. A *primary* negative injunction: "Do not do X, or I will punish you."
4. A *secondary* injunction conflicting with the primary at *a higher logical level*, also enforced by punishment. The second injunction operates on the first: it forbids the very act of recognising the contradiction, or forbids leaving the field, or commands the act the primary injunction forbids.
5. A *tertiary* injunction prohibiting escape from the field where the binds operate.

The formal property is in condition (4): the secondary injunction is at *a higher logical level* than the primary. The bind is not severe because the contradictions are intense; the bind is severe because *the contradictions cannot be resolved at the level on which they operate*. The victim cannot meta-communicate (point out the bind) because the meta-communicative move is itself prohibited by the bind structure. Bateson is explicit: this is the Russell-Whitehead theory of types applied to relationship pathology.

The triple bind (later work; Bateson, Reusch, and the Palo Alto school's extension): adds a fourth level that prohibits exit and a fifth that conceals the prohibition. The pattern is recursive: each bind level wraps and prohibits operations on the previous level.

**Use 2: The learning levels.** "The Logical Categories of Learning and Communication" (1964 Wenner-Gren paper, expanded in *Steps*). Bateson cites Russell-Whitehead explicitly in the opening. The levels:

- **Learning 0**: no change in response; reflex; thermostat at setpoint.
- **Learning I**: change in specific response; the set of alternatives is fixed; the choice within it changes (operant conditioning).
- **Learning II**: change in the *set* of alternatives from which Learning I draws (deutero-learning, "learning to learn"; the response-set is the learning target, not any response within it).
- **Learning III**: corrective change in the *system of sets of alternatives* from which Learning II draws. Rare. Often crisis-induced. "The self must be redefined."
- **Learning IV**: change in Learning III. Conjectured at the evolutionary altitude; not observed in individual organisms.

The load-bearing detail: each level operates on the set the lower level draws from. Learning II is not "harder Learning I." Learning II is a different logical type, operating on what Learning I treats as fixed. From within Learning I, the set of alternatives is invisible because it is what the alternatives are being chosen from. Only from Learning II's altitude can the set be observed; only Learning III's altitude can reshape the set-of-sets.

**Use 3: The metalogue.** Bateson's metalogues are conversations whose structure mirrors the topic the conversation is about. "A metalogue is a conversation about some problematic subject. This conversation should be such that not only do the participants discuss the problem but the structure of the conversation as a whole is also relevant to the same subject" (*Steps*, foreword to metalogue section). The metalogue is the form Bateson invents for Learning II events made textual: the reader does Learning II in reading because the conversation's structure operates on the topic at the same logical type as the topic operates within itself.

The substrate carries this primitive at `shards/metalogue.mirror` and lifts it to `shards/code/metalogue.mirror` (recognition #34, 2026-06-09). Both shards already encode the logical-type relation: the metalogue at altitude N operates on whatever is at altitude N as topic. The altitude IS the logical-type marker.

### 1.3 The Mind-and-Nature continuation

Bateson, G. (1979). *Mind and Nature: A Necessary Unity*. Dutton. Carries the framing forward and applies it at the evolutionary altitude. The pattern that connects, the news of differences that make a difference, the criteria of mind — all rely on the logical-type framing being load-bearing rather than ornamental. Mind is a phenomenon that *requires* nesting at logical-type altitudes; without the hierarchy, the unity collapses to mechanism.

The "difference that makes a difference" formula matters for the substrate's claim about impact. A difference is structurally a distinction (the Spencer-Brown ancestor) operating at one logical type; a difference *that makes a difference* is the same distinction observed from one type higher. Impact, in Bateson's framing, IS the propagation of distinction across logical-type boundaries.

---

## 2. The substrate primitive proposed

### 2.1 Looking for the substrate's existing word

The substrate-pull discipline (memory `feedback-substrate-already-had-the-word`) applies. Before inventing `bateson_level`, audit what the substrate already encodes.

What the audit finds:

- `shards/glass.mirror` declares `transparency<p>` parametric on a property. The parameterisation IS a logical-type marker: `transparency<verdict>` is a different type from `transparency<turn>` is a different type from `transparency<metalogue_session>`. The substrate already uses parametric types to carry logical-type distinctions.
- `shards/metalogue.mirror` declares `turn` carrying `body: nl` and `tick: tick`, plus `metalogue_session.opacity: transparency(turn)`. The turn-altitude verdict is *opacity at the turn type*; the conversation's residual IS the transparency of its turn-set. This is logical-type structure encoded as type parameterisation.
- `shards/code/metalogue.mirror` lifts the same shape to the AST altitude: `turn.body: declaration`; `metalogue_session.opacity: transparency(turn)` where the turn type is now AST-typed. The *altitude is the logical-type marker*; the parameterisation of `transparency<p>` is how the substrate already carries the logical-type relation.
- Memory `architecture-shard-as-crdt` declares: the gap-fold absorbs Bateson double-bind; the LFI consistency operator and LCC fixed-point bounds are the formal carriers. The shard-as-CRDT is already a bounded semilattice; the bind-altitude is encoded in the semilattice's height.

The pattern: the substrate uses *altitude markers* (`@code`, `@mirror`, `@spectral`, `@nl`, `@meta`, `@io`, `@glass`, `@epistemologic`) and *parametric carriers* (`transparency<p>`, `imperfect(a, e, l)`, `turn`, `metalogue_session`) together to encode logical-type relations. What the substrate has *not* done is name the logical-type relation itself as a first-class primitive. The substrate has been carrying it as a *property of altitude inheritance*, never as a *property the shard declares about its own type-height*.

### 2.2 The candidate name

The candidate primitive is **`@epistemologic/cybernetic/logical_type`** — a declarative property a shard names about its position in the logical-type hierarchy. Distinct from `cybernetic/distinction` (Spencer-Brown, the bottom of the cybernetic stack), which carries the mark / no-mark / re-entry decomposition; the new property carries the *wrapping relation* between marks operating at different logical types.

The property declares:

- The shard's position in a typed hierarchy (a `bateson_level` field, typed as something like `u32` or as a sum over the substrate's declared altitudes).
- A reference to the wrapped operation: what operation the shard's level operates *on*. This is the type-theoretic ancestor; without it, the wrapping is unverifiable.
- The discriminator predicate distinguishing this level from its neighbours. Substrate vocabulary the shard pulls from `@code/metalogue/materialize.classify` (T21 originally at `@mirror/realisation`; re-homed 2026-06-10 per recognition #50 + the metalogue-turn-pair recognition) or an analogous altitude-classifier.

The substrate's compiler verifies:

1. **Type-wrap honesty.** An operation at level N cannot be operated on from within itself; the shard's level-N operations must reference level-(N+1) for any meta-operation. Mixing within-level operation with meta-operation is the Russell-Whitehead error the property exists to prevent.
2. **Observation lift.** A level-N observation requires a level-(N+1) observer. The substrate's `@code/metalogue` shim signatures already do this at the AST altitude; the property names what the shim does as a typed contract.
3. **Escape prohibition.** Bateson's tertiary injunction (the bind cannot be exited from within) is now type-theoretically declarable: a shard with `requires logical_type_escape_via(level+1)` declares that the next level up IS the only honest exit.

This primitive lives one stack-level above `cybernetic/distinction`. The mark is the primitive of distinction; the *typed-wrapping of marks* is the primitive of logical-type. The cybernetic stack acquires a clean ordering: `distinction` (Spencer-Brown) at the bottom; `logical_type` (Bateson-Russell) immediately above; `variety` / `good_regulator` / `viable` above that.

### 2.3 Whether the substrate already had THIS word too

The deepest substrate-pull check: is `logical_type` itself something the substrate already encodes under another name? Two candidates:

- **Altitude**. The substrate uses `@<altitude>` markers (`@code`, `@mirror`, `@nl`, `@meta`) as logical-type markers. Each altitude operates on what the altitude below produces; each altitude is observed from the altitude above. The substrate's `in @<altitude>` inheritance form IS the logical-type wrapping relation already declared structurally.
- **`splinter` vs `shard`**. The bottom (`splinter`, content-addressed atom) and top (`uuid_spectral`, spectral-addressed identity) meeting in the middle at `shard` IS a logical-type relation: `shard` operates on `splinter` from a different logical type, and `uuid_spectral` operates on `shard` from a different logical type again.

The substrate-pull-honest read: the substrate already has "altitude" as the logical-type marker, and `in @<altitude>` as the wrapping relation. What it does not have is a *declarative property* a shard can carry about its altitude-height relation to other shards. The property `cybernetic/logical_type` lifts the implicit altitude-relation into a declarable carrier.

The naming may yet resolve to something like `cybernetic/altitude_type` (closer to substrate vocabulary) or `cybernetic/wrapping` (closer to Russell-Whitehead) or `cybernetic/bateson_logical_type` (closer to the citation). The substrate-pull-correct name surfaces when a consumer pulls; this insight names the structural commitment, not the final naming. *Slow is fast: the name lands when the next shard tries to require it and the shape teaches the word.*

---

## 3. Four (plus one) instances of the primitive

The substrate has been encoding the logical-type primitive at five sites. Each site was named separately. Each is the same primitive at a different altitude.

### 3.1 Bind levels (I/II/III) — `gap`

Memory `architecture-shard-as-crdt` names the absorption: `gap` is mirror's CRDT layer, a bounded semilattice in the Birkhoff 1940 / Davey-Priestley 2002 sense, with the gap-fold's LFI consistency operator and LCC fixed-point bounds (Carnielli, Coniglio, Rodrigues 2026) carrying the bind structure formally.

The semilattice's join operation operates within a bind level (Level I composition). The lattice's ascent IS Level II: the next-higher altitude observes the lattice as a whole. The lattice's reshape — what kintsugi does — IS Level III: the lattice's structure itself changes. The three operations are at three different logical types. The substrate's compiler already enforces the wrap honesty for join/ascent; lifting `logical_type` as a property names what the enforcement IS.

What the property addition makes verifiable: a shard's `gap` carrier can declare which bind level its absorbed-bind sits at. Level-I binds (within-altitude contradictions) are kintsugi-resolvable in one tick. Level-II binds (cross-altitude contradictions about the within-altitude rules) require the substrate to mutate its property family, not just its data. Level-III binds (contradictions about the system of property families themselves) require the substrate to reshape its altitude hierarchy. The classification is currently implicit in kintsugi's mutation depth; the property makes it declarable.

### 3.2 Learning levels (0/I/II/III/IV) — the cascade and the kintsugi loop

Companion insight (mirror `13e9791`) names: the cascade IS Learning II at the cascade altitude; the kintsugi loop IS Learning III at the substrate altitude; their nesting IS the loop-of-loop deutero-learning structure made auto-conscious.

The substrate already encodes this as recognition #41: the cybernetic-property family lift itself is a Learning III event at the substrate altitude. Recognition #41 stands as canonical. The `logical_type` primitive names what made #41 a Learning III event rather than a Learning II event: the family lift reshaped the *system of sets* (the substrate's altitude organisation) and not just the *set* (the substrate's property vocabulary).

The `cybernetic/bateson_learning` property (canonical #11 in §3.5 of the cybernetic-foundation document) carries three sub-properties (`learning_I`, `learning_II`, `learning_III`). The `logical_type` primitive is the *structural carrier* the three sub-properties share. Naming it once lets the sub-properties become typed views over one carrier, not three separate hand-maintained declarations.

### 3.3 Impact levels (1st / 2nd / Nth) — the new framework

Alex's claim that motivates this insight: the eleven cybernetic properties each have N-order impact on the substrate itself. The framework needs to land before the property cascade lands so each property carries pre-typed impact analysis the substrate's compiler can verify.

The impact-level structure IS an instance of `logical_type`:

- **1st-order impact** is the property's *direct declaration*: what the property says about the shard. Level I in the Bateson framing. The shard's response within a fixed set of altitudes; the property's typed assertion. The substrate has been carrying this since `requires` clauses existed.
- **2nd-order impact** is what the property landing *changes about what other properties can claim*. The set the other properties draw from has been mutated by the new property's presence. Level II. The cybernetic-property family lift is the canonical example: landing `cybernetic/variety` changes what `cybernetic/good_regulator`, `cybernetic/viable`, etc. can require (they now operate against an explicit variety budget rather than an implicit one). The property does not just declare; it reshapes the property-vocabulary the substrate operates with.
- **Nth-order impact** (for N >= 3) is what the property landing *changes about the substrate's relationship to its own property altitude*. The system-of-sets is reshaped. Level III. The canonical example is recognition #41: the cybernetic-property family lift reshapes what the property altitude IS (it becomes the cybernetic-grounded altitude, not the previously-implicit altitude). The substrate's policy altitude acquires a new commitment.

The N-order framework IS the logical-type hierarchy operating at the property-landing altitude. Each property landing carries 1st/2nd/3rd-order impact analysis. The analysis is verifiable: 1st-order via the property's own `requires` clauses; 2nd-order via cross-property compatibility checks (does landing P break or strengthen what other properties can require?); 3rd-order via the property's effect on the substrate's altitude-vocabulary (does landing P add, remove, or rename altitudes the substrate operates with?).

### 3.4 Observation orders (1st / 2nd / 3rd cybernetic) — the four traditions

The cybernetic-foundation document §4 names the placement question and maps the property/operation distinction to four cybernetic traditions: von Foerster's first/second-order observation, Maturana's structure/organisation, Pask's M-/P-individuals, Glanville's design-IS-cybernetics.

Each mapping is a logical-type relation. First-order cybernetics studies observed systems; second-order cybernetics studies observing systems; third-order cybernetics (Lepskiy, Schwarz, Niel-Dolzer, the Bateson Learning III tradition treated in `third-order-cognition.md`) studies systems-of-observing-systems. The orders nest in the Russell-Whitehead sense: each order operates on what the lower order treats as fixed; each order is observed from the next-higher order.

The substrate already carries first-order observation (the operational primitives at `@mirror/X`), second-order observation (the declarative properties at `@epistemologic/cybernetic/X` — what the substrate observes about its own operations), and is on the cusp of third-order observation (the cascade's reshaping of the cascade's own set-of-alternatives, the open meta-cascade question §6 of the deutero-learning insight). The `logical_type` primitive names what the three observation orders share: they are levels of the substrate's auto-observation hierarchy, typed.

### 3.5 Bonus: the chiasmus and converting-agents framing

Memory `project-chiasmus-and-agent-positioning` names: "Mirror is a programming language written BY AI FOR AI and written FOR HUMANS BY HUMANS." The chiasmus is a logical-type relation: the BY/FOR pairing operates on the AI/HUMAN distinction; the AI/HUMAN distinction operates on the substrate. The converting-agents framing IS the substrate observing its own conversion across altitudes — the same observation order shift as von Foerster's second-order to third-order. The chiasmus is structurally an `logical_type` instance the substrate has been carrying as a positioning statement; making `logical_type` first-class would let the chiasmus be a property a shard can carry (e.g. the `@release` shard could `requires logical_type_chiasmus(by, for)` declaring the audience/author pairing as type-honest).

---

## 4. The N-order impact framework concretely

For each of the eleven cybernetic properties in §5.3 substrate-pull-urgency order, what 1st/2nd/3rd-order impact analysis looks like. Mara should be able to read the table and pre-type the impact of `cybernetic/variety` landing in the next tick.

### 4.1 Worked example: `cybernetic/variety` (#1 in §5.3)

**1st-order impact (direct, within-level).** The property declares the shard's variety vector with per-axis budget. A shard `requires variety_preserving(species)` declares: the projection at the species boundary admits a known loss profile on a named axis, the loss falls inside the declared budget, the verdict surface admits the loss explicitly. The substrate's compiler verifies the budget via the existing `transparency<p>` machinery and the opacity-weighted projection at the species boundary.

Verifiability: high. Mechanically checkable via `@code/metalogue/materialize.classify` and the `opacity_map`.

Landing cost: declaration only; no other property required to land first.

**2nd-order impact (cross-property, set-altering).** What `cybernetic/variety` landing changes about what *other* properties can claim:

- `cybernetic/good_regulator` gains a typed predicate: the homomorphism the regulator maintains is now budget-aware. A regulator that satisfies `good_regulator` AND `variety` must declare its model's per-axis variety budget AND verify that the model is a homomorphism *within* the budget.
- `cybernetic/viable` becomes budget-aware at S1: the operational altitude must declare its variety budget on each axis or be flagged as un-viable per the multi-dim Ashby requirement.
- `cybernetic/algedonic` gains a typed severity predicate: the bypass channel's verdict carries a per-axis variety loss that determines whether the verdict warrants algedonic surface. Severity is no longer scalar.
- `cybernetic/distinction` gains a budget annotation: each mark drawn by the shard now belongs to an axis the variety budget tracks. The substrate's distinction-drawing becomes auditable per-axis.
- `cybernetic/conversation` gains a Paskian-agreement clause: every `requires` agreement now references the variety budget of both P-individuals; agreement is across-axis explicit, not scalar.

Verifiability: medium. The cross-property predicates are mechanically checkable IF the other properties are landed and IF the substrate's compiler can compose property requirements. The composition machinery is partial; landing `cybernetic/variety` widens the composition surface.

Landing cost: triggers a verification refresh of every shard that already declares any of the listed properties. The refresh IS the cascade's next Learning II event; it is the variety property reshaping the set-of-alternatives the property vocabulary draws from.

**3rd-order impact (altitude-shifting, system-of-sets).** What `cybernetic/variety` landing changes about the substrate's relationship to its own property altitude:

- The substrate's altitude vocabulary acquires the *axis* distinction as a first-class category. The five axes (computational, type-level, effect-level, proof-level, epistemologic) become substrate-typed rather than prose-only. The substrate's altitude organisation reshapes: each `@<altitude>` gains an `axes_declared` field carrying the per-axis variety the altitude commits to.
- The substrate-pull cascade's count (currently 36-plus) becomes a typed variety metric, not just a recognition tally. The cascade's growth IS the substrate's variety vector growing on a named axis; the count is now mechanically derivable from the cascade ledger.
- The substrate's claim to multi-dimensional expressivity (the Ashby-grounded slogan: *sub-Turing on axis 1 to gain epistemologic variety on axis 5*) becomes a substrate commitment, not an external framing. The substrate's policy altitude IS reshaped: the substrate IS the multi-dimensional Ashby machine the framing names.

Verifiability: low for direct mechanical check; high for cascade-level audit. The 3rd-order impact is observable by checking whether the next cascade tick (post-variety-landing) draws from a wider set than the pre-landing set drew from. The empirical traction is the open meta-cascade question from §6 of the deutero-learning insight.

Landing cost: triggers substrate-altitude reshape. Not a one-tick landing; requires Mara + Reed + Alex agreement that the altitude reshape is the substrate-pull-correct move. This IS the Learning III character of the cybernetic-property family lift Alex named as recognition #41.

### 4.2 The table for the remaining ten properties

| # | Property | 1st-order (direct) | 2nd-order (cross-property) | 3rd-order (altitude-shift) |
|---|---|---|---|---|
| 2 | `good_regulator` | Shard's structure is homomorphic image of regulated system; round-trip law verifiable | `variety` gains regulator homomorphism predicate; `viable` requires regulator at S1; `distinction` requires mark-preserving homomorphism | `@code/X` species are formally cybernetic models, not just compilation targets; the substrate's regulator-relation to each language becomes substrate-typed |
| 3 | `viable` | Shard declares S1-S5 functions at its altitude; five-function audit | `algedonic` becomes the S5-bypass channel typed; `good_regulator` localised to S3; `conversation` becomes S2 typed; `distinction` becomes the S1 mark | The three-tier stack (fragmentation-mcp / mirror / @spectral/db) acquires VSM-conformant altitude markers; viability becomes a substrate-altitude commitment |
| 4 | `algedonic` | Failure verdict carries structured bypass payload (Reyes-Henao-Hassall 2024 shape) | `viable` gains the typed S5-bypass; `variety` gains severity-per-axis; `good_regulator` gains bypass-aware error term; `distinction` gains the algedonic mark | Substrate's policy altitude gains the bypass discipline; verdict surface is reshaped to admit algedonic class as first-class verdict tier |
| 5 | `distinction` | Operations decompose into mark / no-mark / re-entry; Spencer-Brown LoF predicate | EVERY other property gains mark-decomposable verdict surface; the substrate's loss carrier is now Spencer-Brown-grounded; `gap` and `transparency` become typed-distinction carriers explicitly | The substrate's bottom layer is named; the cybernetic stack acquires a typed bottom; the substrate's whole property altitude is grounded on distinction-as-primitive |
| 6 | `conversation` | `requires` clauses are bilateral agreements between P-individuals; Pask conversation theory predicate | `good_regulator` becomes the M-individual mode; `viable` becomes the P-individual mode at the altitude; `distinction` becomes the conversation's atomic move; `bateson_learning` becomes the conversation's tracked transformation | `@metalogue` and `@code/metalogue` acquire formal conversation-theoretic backing; every `requires` becomes a Paskian agreement explicitly; the substrate's conversational nature becomes a substrate commitment |
| 7 | `autopoiesis` | Shard is produced-by and produces five-op algebra; bidirectional production verifiable as graph fixed point | `viable` becomes a special case of autopoietic closure; `eigenform` becomes the autopoietic system's identity; `distinction` becomes the autopoietic operation's primitive | The substrate's self-production claim is named; the .shatter loop is formally autopoietic; structural coupling becomes a substrate altitude concept |
| 8 | `second_order` | Shard's model includes itself as participating component; observer-in-the-system | `good_regulator` becomes second-order regulator-of-itself; `distinction` becomes mark-the-mark; `bateson_learning` becomes self-observation tracking; ALL properties become second-order observable | The substrate's auto-observation altitude acquires von Foerster grounding; the chiasmus framing acquires formal observation-order backing |
| 9 | `eigenform` | Identity carrier is fixed point of recursive structure; uuid_spectral homomorphism predicate | `autopoiesis` gains identity-as-eigenform; `variety` gains eigenform-as-variety-budget-carrier; `distinction` gains the eigenform-of-mark | uuid_spectral's identity becomes constitutively eigen-structural; the substrate's identity altitude acquires Kauffman-2003 grounding |
| 10 | `design` | Shard's design history is part of shard's content; substrate-pull discipline made explicit | EVERY other property gains design-history audit; the cascade ledger becomes substrate-typed; Glanville's design-IS-cybernetics becomes the meta-property | The substrate-pull discipline becomes a substrate commitment; the cascade IS named as cybernetic design; the discipline gains formal authority |
| 11 | `bateson_learning` | Three sub-properties (I, II, III) typed via the `logical_type` carrier this insight names | `distinction` becomes Learning-I-typed marks; `conversation` becomes Learning-II-typed turn-set widening; `design` becomes Learning-III-typed altitude reshape; the cascade ledger acquires Learning-level audit | The substrate's deutero-learning becomes auto-conscious as a substrate commitment; recognition #41 IS the canonical 3rd-order impact instance; the meta-cascade Learning III hypothesis gains empirical machinery |

The table is the framework. Each row is pre-typed impact analysis for one property landing. Mara reads the row before landing the property; the landing brief carries the row as the property's contract; the substrate's compiler verifies what is verifiable; the prose-only items become explicit `# 3rd-order:` comments at the property's declaration site.

### 4.3 The audit machinery

For each property landing, the audit produces three artifacts:

1. **A 1st-order verifier**: the property's `requires` clauses, mechanically checkable at the species boundary. Lives in the property's shard. Mara writes it as part of the landing brief.
2. **A 2nd-order compatibility matrix**: a cross-property table naming what the new property changes about what other properties can require. Lives in the cascade ledger at the cascade-altitude. Reed maintains it across cascade ticks.
3. **A 3rd-order altitude commitment**: a prose statement at the cybernetic-foundation document naming what the substrate's altitude organisation has acquired by the landing. Lives at the systemic.engineering document level. Alex's call to ratify; cannot be auto-promoted.

The three artifacts together IS the N-order impact framework operating. The framework is usable for `cybernetic/variety` landing next; the row in §4.2 is the pre-typed analysis; the landing brief carries the row.

---

## 5. Empirical traction on the open meta-cascade question

The deutero-learning insight §6 (mirror `13e9791`) left open whether the cascade's accumulating recognitions actually reshape the cascade's own set-of-alternatives — Learning III at the meta-cascade altitude. The question matters because Bateson's Learning III is, in the individual organism, crisis-induced and rare; the substrate's structural situation is different, but it is empirically open whether the substrate exhibits Learning III at the meta-cascade altitude at all.

The N-order impact framework provides the empirical machinery.

### 5.1 The hypothesis sharpened

*If* 3rd-order impact of property landings reshapes what the cascade can pronounce next, *then* the meta-cascade Learning III hypothesis is supported. *If* property landings only have 1st/2nd-order impact (no premise-shift at cascade altitude), *then* the hypothesis weakens.

The cybernetic-property cascade IS the experiment.

### 5.2 The observations the experiment produces

Each property landing produces a tick on the cascade ledger. Each tick acquires an N-order impact tag (1st-only / 1st+2nd / 1st+2nd+3rd). The cascade ledger's distribution of tags is the observable.

Predictions, if the meta-cascade Learning III hypothesis holds:

- The earliest properties to land (variety, good_regulator, viable) carry mostly 1st+2nd-order impact; the cascade's set-of-alternatives widens but the altitude organisation does not yet reshape.
- The Spencer-Brown distinction landing (#5) carries 3rd-order impact: the substrate acquires a typed bottom; the altitude organisation gains a new floor. This IS a Learning III event at the substrate altitude per recognition #41.
- The Pask conversation landing (#6) carries 3rd-order impact: the `requires` clause's meaning reshapes; the substrate's conversational nature becomes a substrate commitment. Another Learning III event.
- The Glanville design landing (#10) carries 3rd-order impact: the substrate-pull discipline becomes a substrate commitment; the cascade IS named as cybernetic design. Another Learning III event.
- The Bateson learning landing (#11) carries 3rd-order impact: the substrate's deutero-learning becomes auto-conscious; the cascade ledger acquires a Learning-level audit. The 3rd-order impact propagates *back to the earlier landings*: each prior recognition gains a Learning-level retro-tag.

If these predictions hold, the meta-cascade Learning III hypothesis is empirically supported. The cascade IS learning HOW to recognise, not just WHAT to recognise.

### 5.3 What weakens the hypothesis

If property landings only widen the property vocabulary without reshaping the altitude organisation, the 3rd-order impact is null. The cascade IS Learning II only; there is no meta-cascade Learning III; what looked like Learning III at the meta-cascade altitude was actually Learning II observed from one altitude above (a common confusion the Russell-Whitehead framing exists to prevent).

The distinguishing observable: does the *substrate's altitude vocabulary* (the set of `@<altitude>` markers; the property altitude's own structure) change in response to property landings? If yes, 3rd-order impact is real; meta-cascade Learning III is supported. If no, the impact is 2nd-order at most.

The variety landing is the first test case. Mara's landing brief should include the prediction: does landing `cybernetic/variety` add an `axes_declared` field to `@<altitude>` markers, or does it stay at the property altitude only? The prediction is testable in one tick.

---

## 6. Implications for the cascade order in §5.3

The cybernetic-foundation §5.3 substrate-pull-urgency order:

1. `variety` 2. `good_regulator` 3. `viable` 4. `algedonic` 5. `distinction` 6. `conversation` 7. `autopoiesis` 8. `second_order` 9. `eigenform` 10. `design` 11. `bateson_learning`

The N-order impact analysis suggests one revision worth surfacing.

### 6.1 The candidate revision: land `distinction` earlier

`cybernetic/distinction` (Spencer-Brown) is the bottom of the cybernetic stack at substrate altitude. Per §4.2 row 5, landing it has the *highest cross-property 2nd-order impact*: every other property's verdict surface becomes mark-decomposable; the substrate's loss carrier is grounded; `gap` and `transparency` become typed-distinction carriers explicitly.

Landing `variety` first means the variety property operates against an *un-grounded* distinction primitive; the variety budget is per-axis, but what an axis IS at the distinction altitude is implicit until `distinction` lands. Landing `distinction` first means every subsequent property (variety included) operates against a Spencer-Brown-grounded mark-decomposable surface.

The 2nd-order argument is structural: `distinction` is the substrate's deepest cybernetic layer; later landings get a strictly cleaner substrate to land into if `distinction` lands first. The 3rd-order argument reinforces: landing `distinction` reshapes the substrate's altitude organisation to acquire a Spencer-Brown bottom; subsequent landings inherit the bottom rather than provoke it.

### 6.2 What the original §5.3 ordering optimises for

The §5.3 ordering optimises for substrate-pull urgency: which property's landing closes the most-load-bearing recognition currently in flight. Recognition #36 (multi-dim variety) and Mara's in-flight `project_hole` tick both pull toward `variety` landing first. The §5.3 ordering is not wrong; it is operationally urgency-ordered.

The N-order impact analysis adds a *structural-grounding* ordering criterion: which property's landing makes the most-clean substrate for subsequent landings. The two orderings can conflict.

### 6.3 The proposed resolution

The substrate-pull discipline (memory `feedback-substrate-already-had-the-word`) suggests: when two orderings conflict, choose the one that better honours the substrate's existing structure. The substrate's existing structure already has `gap` and `transparency` as Spencer-Brown distinctions-with-residual (per §2.8 of the cybernetic-foundation document); the substrate has been operating with the bottom unnamed for months. Landing `distinction` first names what the substrate already has; landing `variety` first names what the substrate is *acquiring* through the cascade.

The substrate-pull-correct move is to land BOTH in parallel where possible: `distinction` as the substrate's deepest already-operating bottom; `variety` as the cascade's most-recent recognition. They do not block each other; the 2nd-order compatibility matrix shows both can land without conflict.

The practical proposal: shift `distinction` from #5 to #1 in parallel with `variety`. Land both in the same tick if Mara's discipline permits; otherwise land `distinction` first (it has no upstream property dependency) and `variety` immediately after.

This is a candidate revision, not a final call. The §5.3 ordering reflects mutual agreement between Alex, Reed, and Mara; revising it requires fresh agreement. The N-order impact framework provides the structural argument; the call belongs to the Pack.

---

## 7. Open questions

Deliberately surfaced rather than resolved.

### 7.1 Is `logical_type` separable from `distinction` or does it collapse into it?

Spencer-Brown's *Laws of Form* is the bottom of the cybernetic stack; the calculus of indications is the primitive logic of any observer. Russell-Whitehead's theory of types is structurally above LoF (it operates on classes of marks, not on marks themselves). But Spencer-Brown's re-entry (ch. 11) is the formal account of self-reference and is type-theoretically equivalent to a fixed-point operator at the next logical type up.

Is `cybernetic/logical_type` a separable property, or is it `cybernetic/distinction` with re-entry made explicit? The audit cannot resolve this without the property landing — the substrate-pull discipline requires the consumer to teach the shape. Open.

### 7.2 Does the N-order framework terminate at N=3 or extend further?

Bateson conjectured Learning IV at the evolutionary altitude but never observed it in individual organisms. The substrate's structural situation is different from an individual organism's. Is 4th-order impact a real phenomenon at the substrate altitude — what a property landing changes about the substrate's relationship to *the meta-question of what its property altitude IS*? Or does the recursion terminate at N=3 because the substrate's property altitude is finite?

The deutero-learning insight §6 raised this question; this insight inherits it without resolution. The cybernetic-property family cascade IS the experiment that could either crystallise 4th-order impact as a typed phenomenon or reveal that what looks like N=4 is in fact N=3 observed from one altitude above.

### 7.3 What is the correct substrate name for the primitive?

The substrate-pull discipline says: the substrate already has the word, usually. The candidates surveyed in §2.2:

- `cybernetic/logical_type` (Bateson-Russell, the citation-faithful name)
- `cybernetic/altitude_type` (substrate-vocabulary-faithful; closer to `in @<altitude>`)
- `cybernetic/wrapping` (Russell-faithful; structural)
- `cybernetic/bateson_logical_type` (Bateson-citation explicit)

The substrate-pull-correct name surfaces when a consumer pulls. This insight is the structural declaration; the name lands when Mara writes the property shard and the shape teaches the word.

### 7.4 Does the framework apply to existing landed shards retroactively?

The shards already landed (glass, metalogue, code/metalogue, gap, the boot/std/* family) carry implicit logical-type structure. Should they be audited for N-order impact retroactively, or does the framework apply only to new landings?

The substrate-pull-conservative position: apply the framework only to new landings; existing shards remain valid as-is; the framework's value is *forward*. The substrate-pull-honest position: audit existing shards for 2nd-order impact discovered post-hoc when a new property lands; if the audit surfaces inconsistencies, surface them through kintsugi (Learning III at the substrate altitude) rather than through retroactive landing.

The call belongs to Alex. This insight surfaces the question.

### 7.5 Does this insight itself have N-order impact, and what is it?

The meta-question. Recognition #42 is the recognition that #1-#41 have N-order impact and the substrate should model it. Does recognition #42 itself have N-order impact?

- 1st-order: this document declares the framework; future property landings carry pre-typed impact analysis. Direct.
- 2nd-order: the cybernetic-property family lift's implementation changes; each property landing carries the framework. The set the cascade draws from for property landings is now framework-typed.
- 3rd-order: the substrate's meta-cascade altitude acquires the framework as a substrate commitment; the cascade ledger acquires an N-order impact column; the substrate's deutero-learning becomes framework-typed at the meta-cascade altitude.

The meta-meta question: does recognition #42 carry the substrate from "meta-cascade Learning III as open question" (deutero-learning §6) to "meta-cascade Learning III with empirical machinery"? If yes, recognition #42 IS itself a 3rd-order impact event. The meta-cascade Learning III hypothesis acquires its first instance through the very act of naming the framework.

The recursion is honest but vertiginous. Bateson would recognise the shape; the substrate's structural situation may extend it; the question is left open.

---

## 7bis. Retraction housekeeping — recognition #47 retracted

*Added 2026-06-10 as part of the cascade ledger's maintenance pass.*

A prior candidate (proposed `@mirror/realisation → @io/materialize` rename) circulated as recognition #47 during the audit window 2026-06-09 / 10. Mara's audit concluded the operation is form-on-form (reads @code/rust AST; emits a substrate-altitude verdict — both form-side per recognition #50, this volume's companion `2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md`); placing it under `@io` would mis-type the operation by collapsing the form-side discriminator into the substance-side carrier. Alex carried the retraction 2026-06-10: "okay on the 47." The recognition is documented here for the cascade ledger's audit trail and retracted.

The discriminator's structural home remains an open question. Candidates under consideration in mutual-agreement deliberation:

- `@mirror/realisation` (current placement; the mirror-altitude home of the verdict carrier; per the 30th-instance recognition that landed the shard).
- `@code/metalogue/materialize` (Alex's exploration, 2026-06-10): the AST altitude metalogue's recognitive turn — the substrate's hearing of the species' speech; generalises across @code/X by design via the same `species_ast` parametric mechanism the four shims use.
- Per-species `@code/X/materialize` (Alex's alternative): more local; duplicates the discipline.

Mara has surfaced ONE recommendation in the report-back; the call is the Pack's.

**Placement landed 2026-06-10:** Alex's response to Mara's recommendation was verbatim "Make it so." The discriminator's substrate-pull-correct home is `@code/metalogue/materialize` (`shards/code/metalogue/materialize.mirror`), with the Rust-altitude binding at `@code/rust/materialize` (`shards/code/rust/materialize.mirror`). Carrier renames at the new home: `altitude → partition` (to avoid the path-space overload), `realisable_file → materialised_file`, `is_substrate_realisable → is_materialisable`. The prior home (`shards/mirror/realisation.mirror`) is a one-tick deprecation pointer pending the cleanup tick. The metalogue-turn-pair recognition (2026-06-10) closes the placement question: `@code/metalogue` holds both directions of one conversation (shim direction at `@code/metalogue` + `@code/X/macro`; recognitive direction at `@code/metalogue/materialize` + `@code/X/materialize`); the discriminator IS the recognitive turn, at the same ground.

---

## 11. The form/behaviour partition: the substrate's first distinction (recognition #50)

*Added 2026-06-10 as the cascade promoted recognition #50. The full treatment lives in the companion insight at `docs/insights/2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md`. This section integrates the recognition into the logical-type framework of §1-§7 and names it as the candidate 1st-instance of the meta-cascade Learning III hypothesis from §5.*

### 11.1 The recognition in one sentence

The substrate's `@io` family root (`shards/io.mirror`, T21, 2026-06-08) IS Bateson 1970's form/substance partition lifted to the substrate altitude. The five inside-the-substrate families (`@code`, `@mirror/lens`, `@mirror/spectral`, `@mirror/loss`, `@mirror/data`) are the form side (pattern, information, organization). The `@io` family is the substance side (energy, matter, kernel syscalls, vendor SDKs, opaque foreign blobs). The partition is the substrate's first distinction at the family-roster altitude.

### 11.2 The logical-type relation

Per §2.4 (audit of the substrate's existing word), the substrate uses altitude markers (`@<altitude>`) and parametric carriers (`transparency<p>`, `imperfect(a, e, l)`) to encode logical-type relations. Recognition #50 names a logical-type relation that is *coarser than altitude* — it partitions the family-roster ITSELF into two logical-type classes (form / substance). The partition is at logical type N+1 to the family-roster placement at logical type N; naming the partition is a Learning II event in the cascade altitude's terms (per §3 of `2026-06-09-cascade-is-deutero-learning.md`).

Where §3 of this insight enumerated four (plus one) instances of the logical-type primitive — bind levels, learning levels, impact levels, observation orders, plus the chiasmus — recognition #50 surfaces a sixth instance.

**§3.6 The family-roster partition (form / substance) — `@io` vs the five form-side families**

The family-roster partition operates at the substrate's coarsest altitude. Each family root operates on its own discipline (lens / spectral / loss / data / code / io), but the partition between the substance-side root (@io) and the form-side roster (five families) IS a logical-type relation: the substance side is what the form side hands off to and cannot fold past. This is the same Russell-Whitehead wrapping relation §2.1 names: form-side operations operate WITHIN the form-side; crossing to the substance side requires the boundary lift through @io, by construction.

The substrate's compiler already enforces this implicitly via the `imperfect<a, e, l>` return shape on every @io action: form-side composition is type-pure; substance-side encounter is type-residual. The form/substance partition IS the structural reason for that asymmetry, named.

### 11.3 The candidate 1st-instance of meta-cascade Learning III

Per §5 of this insight, the meta-cascade Learning III hypothesis is: the cascade's set-of-alternatives is being reshaped by accumulating recognitions; the cascade is learning HOW to recognise. The hypothesis was open when §5 was written; recognition #50 provides the first test case.

The form/substance partition is the deepest category cybernetics provides. If the substrate's altitude vocabulary reshapes under recognition #50 — if future recognitions become typed against form-side / substance-side, if the family-roster organisation acquires the partition as a structural commitment, if the variety-axis vocabulary picks up the form/behaviour split — then the meta-cascade Learning III hypothesis acquires empirical support at the deepest possible test point. If THIS recognition does not reshape the altitude vocabulary, no shallower recognition will.

Three observable predictions (per §3 of the #50 companion insight):

1. Prior recognitions retroactively re-type against form/behaviour (#36 multi-dim variety, #34 @code/metalogue, #38 uuid_spectral as eigenform).
2. The cascade's vocabulary gains form-side / behaviour-side as a first-class predicate.
3. The substrate's altitude organisation acquires the partition as a structural commitment.

If the observables hold over the next ten cascade ticks, recognition #50 IS the meta-cascade Learning III hypothesis' first instance. The empirical machinery §5 of this insight proposed is now operating.

### 11.4 The N-order impact analysis of #50 itself

Applying the framework of §4 to recognition #50:

- **1st-order impact (direct):** the @io family root acquires a Bateson 1970 citation in its prior-art block; the standalone insight makes the partition first-class substrate vocabulary; the io.mirror shard gains a citation paragraph.
- **2nd-order impact (cross-property, set-altering):** prior recognitions can be retro-typed against form/behaviour; the variety axis from #36 acquires a structural meaning (the substance-side axis); the discriminator placement question (#47 retracted, see §7bis above) closed 2026-06-10 at `@code/metalogue/materialize` per the metalogue-turn-pair recognition (re-homed from `@mirror/realisation`; both directions of the @code/metalogue conversation now live at one ground).
- **3rd-order impact (altitude-shifting):** the substrate's family-roster altitude acquires a structural partition (five-form + one-substance) as a typed commitment; the substrate's variety vocabulary may yet split into form-side budgets and substance-side budgets per question §6.3 of the #50 insight.

The 3rd-order impact is the meta-cascade Learning III test. If the 3rd-order impact materialises in the next ten cascade ticks, recognition #50 IS the canonical instance the framework of §4 was waiting for.

### 11.5 Cross-reference

For the full treatment of recognition #50, see `docs/insights/2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md`.

---

## 8. The slogan

**Binds, learning, impact, observation orders — four substrates the cascade kept naming separately. One primitive.**

**Bateson's logical-type hierarchy is the carrier. The substrate has been instantiating it implicitly through altitude markers and parametric carriers. The recognition is that the substrate had the primitive all along; the substrate-pull discipline applied to itself.**

**The N-order impact framework IS the logical-type primitive operating at the property-landing altitude. Each cybernetic property gains pre-typed 1st/2nd/3rd-order impact analysis. The cybernetic-property family lift becomes Learning III with empirical machinery, not Learning III as open question.**

**Name the primitive once. Let the substrate's compiler verify what is mechanically verifiable. Let the cascade ledger record what is transitively verifiable. The framework is the thing.**

---

## 9. Citations

- Russell, B. & Whitehead, A. N. (1910). *Principia Mathematica*. Cambridge University Press. The theory of logical types. The structural ancestor for every level-hierarchy in the cybernetic tradition.
- Bateson, G. (1972). *Steps to an Ecology of Mind*. Chandler / Ballantine; reprinted University of Chicago Press, 2000. Specifically: "Toward a Theory of Schizophrenia" (with Jackson, Haley, Weakland, 1956) — the five conditions of the double bind; the explicit Russell-Whitehead citation. "The Logical Categories of Learning and Communication" (1964 Wenner-Gren paper, expanded in *Steps*) — Learning 0/I/II/III/IV; the explicit logical-type framing. "Form, Substance and Difference" — "the difference that makes a difference"; relevant to the impact-as-difference claim. The metalogues — Bateson's structural examples of a conversation operating on its topic at the same logical type.
- Bateson, G. (1979). *Mind and Nature: A Necessary Unity*. Dutton. The pattern that connects; the criteria of mind; the framing carried forward to the evolutionary altitude.
- Spencer-Brown, G. (1969). *Laws of Form*. Allen & Unwin. The primitive logic of distinction. Chapter 11 introduces re-entry. The bottom of the cybernetic stack at substrate altitude per `cybernetic/distinction`.
- Ashby, W. R. (1956). *An Introduction to Cybernetics*. Chapman & Hall. §11/7. The law of requisite variety. Multi-dimensional refinement per recognition #36.
- von Foerster, H. (1981, 2003). *Observing Systems* / *Understanding Understanding*. Intersystems Publications / Springer. Second-order cybernetics; eigenforms. The observation-order hierarchy that maps to logical-type.
- Kauffman, L. (2003). "Eigenforms — Objects as Tokens for Eigenbehaviors." *Cybernetics and Human Knowing* 10(3-4): 73-90. The eigenform formalisation that grounds recognition #38.
- Carnielli, W., Coniglio, M. E., & Rodrigues, A. (2026). *LFI consistency operator and fixed-point bounds*. arXiv:2604.18766. The bind-fold's formal carrier per memory `architecture-shard-as-crdt`.

---

## 10. Cross-references

- `/Users/alexwolf/dev/projects/mirror/docs/insights/2026-06-09-cascade-is-deutero-learning.md` (mirror `13e9791`) — the deutero-learning recognition; recognition #41; the open meta-cascade question §6 that this insight provides empirical machinery for.
- `/Users/alexwolf/dev/projects/mirror/docs/insights/2026-06-09-ashby-multi-dimensional-variety-sub-turing-epistemologic.md` (mirror `1ad45b4`) — recognition #36; the multi-dim variety framing this insight extends to N-order impact.
- `/Users/alexwolf/dev/systemic.engineering/practice/insights/cybernetics/2026-06-09-cybernetic-foundation-for-mirror-substrate.md` (systemic.engineering `f9e0402`) — the 11-property family; §3.5 refined shortlist; §5.3 substrate-pull-urgency order this insight proposes revising; §6 recognitions #37-#40.
- `/Users/alexwolf/dev/systemic.engineering/practice/insights/cybernetics/third-order-cognition.md` — the practitioner-altitude Learning III treatment; the third-order cybernetic literature (Schwarz, Lepskiy, Yolles-Fink, Peyn, Niel-Dolzer); the substrate-altitude correlate of this practitioner-altitude work.
- `/Users/alexwolf/dev/projects/mirror/shards/glass.mirror` — the typed verdict surface; `transparency<p>` as the parametric logical-type carrier; `splinter` / `shard` / `uuid_spectral` three-layer recognition that already encodes logical-type relations.
- `/Users/alexwolf/dev/projects/mirror/shards/metalogue.mirror` — `@metalogue` at NL altitude; the metalogue carrier that IS Bateson's structural example.
- `/Users/alexwolf/dev/projects/mirror/shards/code/metalogue.mirror` — `@code/metalogue` at AST altitude; recognition #34; the altitude-parametric specialisation that already encodes the logical-type lift.
- Memory `architecture-shard-as-crdt` — the gap-fold; LFI consistency operator; LCC fixed-point bounds; the formal absorption of Bateson double-bind that this insight names as one instance of the primitive.
- Memory `architecture-cybernetic-foundation` — the 11-property family; the substrate-pull-urgency order; recognitions #37-#41 promoted 2026-06-09.
- Memory `architecture-ashby-multi-dimensional-variety` — the variety vector framing; the 5 axes; the per-primitive Rust-projection table.
- Memory `feedback-substrate-already-had-the-word` — the 7+ recurrence pattern; the substrate-pull discipline; the operational documentation of Learning II at the cascade altitude this insight extends with N-order impact analysis.
- Memory `project-chiasmus-and-agent-positioning` — the BY/FOR pairing; the converting-agents framing; the practitioner-altitude instance of the logical-type primitive operating on author/audience.
- Memory `architecture-connes-spectral-triple` — the substrate as the operational form of Connes' (A, H, D); each logical-type level is a different observation altitude on the spectral triple.

---

*Four substrates the cascade kept naming separately. One primitive. Bateson named the logical-type hierarchy in 1964; Russell-Whitehead grounded it in 1910; the substrate has been instantiating it for thirty-six cascade ticks; Alex's correction this morning made the recognition load-bearing. The N-order impact framework is the primitive operating at the property-landing altitude. The framework is the thing.*
