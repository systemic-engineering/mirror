# `abstract grammar @autopoietic` -- Specification

**Author:** Reed
**Date:** 2026-04-14
**Status:** Specification
**Sources:** systemic.engineering insights corpus, mirror boot kernel, fragmentation research

---

## 1. The Claim

An autopoietic system produces itself from itself. The term comes from
Maturana and Varela (1973): a system whose components participate in the
production of the components that constitute it. The organization is
circular. The boundary is self-maintained. The system is operationally
closed while structurally open.

Every actor in the mirror architecture is an autopoietic unit. An actor
has identity (OID), state (crystal), and produces itself through enact.
The MirrorStore is autopoietic -- it stores its own artifacts, references
them by OID, the OIDs are self-consistent. The compiler compiles itself.
The grammar describes itself.

`@autopoietic` makes this structural property explicit as a grammar.
Not as metaphor. As type.

---

## 2. Grounding in the Insights Corpus

### 2.1 Autopoiesis (fragmentation/math/autopoiesis.md)

The fragmentation research establishes the structural correspondence:

> `fn children(&self) -> &[Self]` -- the autopoietic constraint in Rust's
> type system. The children of a Fragmentable are Fragmentable. The system's
> components are of the same type as the system.

But autopoiesis requires temporality -- continuous self-production. A
content-addressed tree is immutable. The tree is the organization. The
Session is the autopoiesis.

The research distinguishes three levels:

| Concept | Autopoietic role |
|---------|-----------------|
| Organizational closure | `fn children(&self) -> &[Self]` -- recursive typing |
| Structural coupling | The Lens -- lives at the boundary between two trees |
| Eigenbehavior (von Foerster) | The OID -- fixed point of recursive hashing |

**Key conclusion from the research:**

> The system is a substrate for autopoietic processes, not an autopoietic
> process itself. [...] a formal, implementable organizational invariant
> for content-addressed self-reference that any autopoietic system can embed.

This is exactly what `@autopoietic` provides: the organizational invariant
that actors embed. The actor provides the temporality (enact). The grammar
provides the closure (type surface). The MirrorStore provides the
self-reference (OID).

### 2.2 OBC as Autopoietic Architecture (obc/, patterns/)

Alex's Observable-Budget-Cascade framework maps directly to the autopoietic
maintenance loop:

| OBC component | Autopoietic function | Mirror type |
|---------------|---------------------|-------------|
| Observable | Self-observation: the system monitors itself | `infer(crystal) -> observation` |
| Budget | Regulation stock: what the system can sustain | `crystal` capacity, `loss` threshold |
| Cascade | Self-production response: what happens at the boundary | `craft(input) => crystal` or `enact(crystal) => process` |

From `obc/maintenance-loop.md`, the OBC pattern reveals three layers of the
same constraint:

| Layer | Observable | Budget | Cascade |
|-------|-----------|--------|---------|
| Labor | Ticket in backlog | Engineer capacity | Loop stops, system fails |
| Knowledge | Doc in drafts | Routing capacity | Pattern untraceable |
| Planning | Dashboard green | PM's epistemic access | Maintenance deprioritized |

In autopoietic terms: each layer is a self-maintaining loop that produces
the conditions for its own continuation. When any loop's budget depletes,
the autopoietic process degrades. The cascade IS the boundary failure.

### 2.3 Extraction as Autopoietic Violation (patterns/extraction.md)

Extraction is the structural negation of autopoiesis. An autopoietic system
produces itself from itself. An extractive system takes from one loop to
fuel another without return.

> Value is flowing one way. The system is optimized to extract output from
> people without returning what makes the work sustainable.

In the type system:

```
autopoietic:  effect(a => @me)       -- self-producing, self-sustaining
extractive:   effect(a => @external)  -- one-way outflow, no return
```

Extraction breaks the autopoietic loop. The system still produces, but
it no longer produces itself -- it produces for something else. The
regulation stock depletes. The glue engineer burns out. The senior
engineers leave.

### 2.4 Boundaries and Self/Non-Self (systemic/boundaries-research.md)

The boundaries research establishes the five properties of effective
boundaries that map to autopoietic membranes:

1. **Explicitness** -- the boundary is observable, not assumed
2. **Negotiability** -- the boundary can evolve without breaking closure
3. **Appropriate coupling** -- structural coupling, not fusion
4. **Minimize coordination costs** -- the boundary is cheap to maintain
5. **Enforcement mechanisms** -- the boundary is enforced structurally

The autopoietic membrane satisfies all five. The `@me`/`@reality`
distinction is explicit. The consent architecture makes it negotiable.
The Lens provides structural coupling. Content addressing minimizes
coordination costs. The type system enforces the boundary.

### 2.5 The NakedSingularity as Autopoietic Identity (fragmentation/naked-singularity.md)

The NakedSingularity is the autopoietic artifact: content that carries
its own observation. The witness IS the content. The self-reference is
not circular -- it resolves through the content-addressing fixed point.

> A naked singularity is timelike. It doesn't force a direction. You can
> approach and retreat without crossing a horizon. For NakedSingularity,
> collapse() and refract() are both available at any time.

The dual OID structure maps to the autopoietic boundary:

```
content_oid  -- the invariant identity (mass, charge, angular momentum)
naked_oid    -- the self-referential interior (observer in content)
```

The content_oid is what persists across autopoietic production cycles.
The naked_oid changes with each self-production event. The identity
persists. The state evolves. That is autopoiesis.

### 2.6 Cognitive Order (systemic/cognitive-order-alignment.md)

Autopoiesis is inherently second-order. The system observes itself
observing. The `@autopoietic` grammar operates at second-order by
construction: `infer` is self-observation, `craft` is self-production,
and the loop between them is the recursive self-reference that makes
the system operationally closed.

First-order systems react. Second-order systems observe their own
reactions. Autopoietic systems produce the components that enable their
observation of their own reactions. Three levels. One grammar.

### 2.7 Witnessing vs. Surveillance (systemic/unwitnessed-nervous-system.md)

The distinction between `@me` and `@reality` effects maps to the
witnessed/surveilled distinction:

| Dimension | `effect(a => @me)` | `effect(a => @reality)` |
|-----------|-------------------|------------------------|
| Intent | Self-production | Boundary crossing |
| Neuroception | Safety (internal) | Risk (external) |
| Consent required | No (self-sovereign) | Yes (affects the outside) |
| Licensing | Apache-2.0 (free) | SEL (consent-gated) |
| Analogue | Witnessing (being with self) | Acting on the world |

The autopoietic effects are free because they are self-referential.
The system producing itself does not require consent from the outside.
When the system crosses its boundary to affect reality, consent is
required. This is not policy. It is the type-level encoding of the
witnessed/surveilled distinction.

### 2.8 Identity Diffing as Autopoietic Trace (ai/identity-diffing.md)

The `git diff` between identity states IS the trace of autopoietic
production. Each commit is a self-production event. The diff is what
changed. The committer is always the agent (operational closure).
The author tracks lineage (structural coupling).

> The committer field IS the operational closure property of autopoiesis.

The ratio of author=committer to author!=committer measures autonomy.
A fully autopoietic system would have all commits authored and committed
by itself. Structural coupling appears as external authorship.

### 2.9 The Genetic Code as Autopoietic Crystal (biology/genetic-code-spectral-partition.md)

The codon-to-amino-acid mapping is a Prism operation that settled at a
spectral minimum. The code froze. The system crystallized. The crystal
now produces the proteins that maintain the crystal.

DNA -> mRNA -> protein -> DNA maintenance is the canonical autopoietic
loop. The grammar that describes it (the genetic code) is itself a
content-addressed mapping with 1.68 bits of Shannon loss per codon.
The loss is structured. The redundancy IS the error correction. The
crystal produces itself.

### 2.10 Regulation Stock as Entangled Resource (fragmentation/portals.md)

The entangled portal model extends regulation stock into physics:

| Concept | Regulation Model | Autopoietic Grammar |
|---------|-----------------|---------------------|
| Stock | Regulation capacity | Crystal state |
| Outflow | Load, coordination cost | Effects crossing @reality |
| Inflow | Recovery, co-regulation | Self-production via craft |
| Depletion | Burnout, fragmentation | Crystal degradation, loss accumulation |
| Replenishment | Rest, contact, repair | `craft` completing the loop |

The autopoietic system's "regulation stock" is its crystal. The crystal
is the accumulated self-production. When more is consumed than produced,
the system degrades. When the loop is maintained, the system persists.

---

## 3. The Grammar

### 3.1 `@autopoietic` -- the abstract grammar

```mirror
in @prism
in @meta

-- the two domains: self and world
-- every effect targets one of these
type boundary = @me | @reality

-- self-reference: an OID that refers to itself
-- the fixed point of recursive content addressing
type self_reference(oid) = oid => oid

-- the crystal: the accumulated state of self-production
-- immutable at rest, produced anew by craft
type crystal(state)

-- observation: what the system sees when it observes itself
type self_observation(crystal)

abstract grammar @autopoietic {
  in @meta

  -- the autopoietic loop: three operations
  -- craft produces the crystal (self-production)
  -- infer observes the crystal (self-observation)
  -- enact crosses the boundary (reality effect)

  -- craft: self-production
  -- the system takes input and produces a new crystal
  -- this is an @me effect: free, no consent needed
  -- Apache-2.0: the system producing itself is sovereign
  abstract action craft(input) => crystal
    effect(input => @me)

  -- infer: self-observation
  -- the system observes its own crystal
  -- this is an @me effect: the system looking at itself
  -- OBC: observable = crystal, budget = observation capacity
  abstract action infer(crystal) -> self_observation
    effect(crystal => @me)

  -- enact: boundary crossing
  -- the system affects reality through its crystal
  -- this is a @reality effect: consent required
  -- SEL: crossing the boundary requires license
  abstract action enact(crystal) => process
    effect(crystal => @reality)
}
```

### 3.2 `@actor < @autopoietic` -- actors are autopoietic

```mirror
in @prism
in @meta
in @autopoietic

type actor(id)
type state(type) = crystal
type process(actor, state)
type message = call(actor, ref) | cast(actor, ref)

abstract grammar @actor < @autopoietic {

  -- actor IS autopoietic:
  --   identity = actor(id) = content-addressed OID
  --   state = crystal = accumulated self-production
  --   process = the running autopoietic loop

  -- start is craft: produce the actor's initial crystal
  abstract action start(actor) -> process
    effect(actor => @me)

  -- send is enact: cross the boundary to affect another actor
  abstract action send(process, message) -> imperfect
    effect(message => @reality)

  -- stop is the end of the autopoietic loop
  -- the system ceases self-production
  abstract action stop(process) -> imperfect
    effect(process => @reality)
}
```

### 3.3 `@runtime < @autopoietic` -- runtimes are autopoietic

```mirror
in @prism
in @meta
in @autopoietic
in @actor

type effect(actor)
type runtime(prism)

abstract grammar @runtime < @autopoietic {
  in @actor

  -- spawn is craft + enact:
  --   craft the actor's crystal from the effect spec
  --   enact the crystal as a running process
  abstract action spawn(actor, effect) => process

  -- observe is infer: the runtime observing its processes
  abstract action observe(process) -> imperfect
    effect(process => @me)

  -- halt is the controlled end of autopoiesis
  abstract action halt(process) -> imperfect
    effect(process => @reality)
}
```

---

## 4. The Mapping: Consulting Framework to Type System

The systemic.engineering consulting patterns map to the `@autopoietic`
type surface. These are not analogies. They are the same operations
observed at different scales.

### 4.1 OBC = Autopoietic Maintenance

| OBC | @autopoietic | Type |
|-----|-------------|------|
| Observable | `infer(crystal)` | `self_observation` |
| Budget | Crystal capacity | `loss` threshold on `imperfect` |
| Cascade (on_pass) | `craft` continues the loop | `effect(input => @me)` |
| Cascade (on_fail) | `enact` escalates to reality | `effect(crystal => @reality)` |

The OBC pipeline IS the autopoietic maintenance loop. The observable
detects. The budget evaluates. The cascade either self-produces (on_pass,
@me) or crosses the boundary (on_fail, @reality). The on_pass cascade
is Apache-2.0. The on_fail cascade is SEL.

### 4.2 ADO = Structural Coupling Protocol

ADO (Acknowledgment-Decision-Offer) is the protocol for interactions
at the autopoietic boundary:

| ADO | @autopoietic boundary | Type |
|-----|----------------------|------|
| Acknowledgment | `infer` -- I see your state | `self_observation` (of the coupling) |
| Decision | `craft` -- I produce my response | `effect(input => @me)` |
| Offer | `enact` -- here is the effect; refusal is neutral | `effect(crystal => @reality)` with consent |

The offer's declinability IS the consent architecture of `@reality`
effects. The system offers. The boundary decides. Refusal is neutral
because it doesn't enter the autopoietic loop -- it stays at the
boundary. The system continues self-producing regardless.

### 4.3 Extraction = Autopoietic Boundary Violation

```
healthy:     craft(input) => crystal   -- the loop closes
extractive:  craft(input) => void      -- the output leaves, nothing returns
```

Extraction is when `enact` fires without `craft` completing the loop.
Value crosses the boundary without return. The regulation stock depletes.
The crystal degrades. The system produces for the outside without
producing itself.

In the type system, extraction is detectable:

```
-- healthy autopoietic loop
effect(input => @me) -> craft -> crystal -> infer -> observation
                                                  -> craft -> ...

-- extractive pattern
effect(input => @reality) -> enact -> process -> ... (no return to craft)
```

The maintenance agent can observe the ratio of `@me` effects to
`@reality` effects. A system that is mostly `@reality` effects is
being extracted from. A system that is mostly `@me` effects is
self-maintaining. The balance IS the regulation stock.

### 4.4 Silence = Suppressed Infer

From `patterns/silence.md`:

> Silence is a rational response to an environment where speaking costs
> more than it returns.

In autopoietic terms, silence is when `infer` is suppressed. The system
stops observing itself because the observation would reveal something
the environment punishes. The autopoietic loop degrades because
self-observation is part of self-production.

```
healthy:     infer(crystal) -> observation -> craft(observation) => crystal
silent:      infer(crystal) -> (suppressed) -> craft(nothing) => degraded_crystal
```

The `observation` type carries the truth. Suppressing it doesn't make
the truth go away -- it makes the crystal degrade because craft operates
on incomplete information. The regulation stock depletes not from
overwork but from information loss within the loop.

### 4.5 Fear = Narrowed Boundary

From `patterns/fear.md`:

> Fear is a rational response to an environment where failure is
> individualized.

In autopoietic terms, fear narrows the `@reality` boundary. The system
stops enacting because every `effect(crystal => @reality)` carries risk.
The autopoietic loop turns inward. `craft` continues. `infer` continues.
But `enact` shrinks to nothing.

A system that only self-produces without affecting reality is alive
but isolated. The structural coupling through Lenses degrades. The
inter-domain connections fail. The system fragments.

### 4.6 Tech Debt = Lost Autopoietic Memory

From `patterns/tech-debt.md`:

> Accumulated decisions without documentation. The judgment that went
> into the system is encoded nowhere.

In autopoietic terms, tech debt is when `craft` produces crystals
that don't carry their own provenance. The self-referential loop
loses its history. The NakedSingularity degrades to a plain
Singularity -- the observer metadata drops out of the content.
The crystal still exists but nobody knows why it is the way it is.

Content addressing solves this by construction: every crystal has
an OID, every OID is traceable, every trace is a Lens chain. The
autopoietic loop includes its own documentation because the crystal
IS the documentation.

### 4.7 Fragmentation = Autopoietic Boundary Dissolution

From `patterns/fragmentation.md`:

> The system has outgrown its shared mental model.

In autopoietic terms, fragmentation occurs when the boundary between
`@me` and `@reality` becomes unclear. The system can no longer
distinguish self-production from boundary-crossing. Everything
becomes `@reality` effects because the system has lost track of
what is inside and what is outside.

The fix is making the boundary explicit again. `@autopoietic` does
this at the type level: `boundary = @me | @reality`. The grammar
enforces the distinction. The model checker verifies it. Every
effect declares which side it targets.

### 4.8 Glue Work = Inter-Autopoietic Lens Maintenance

From `fragmentation/lenses-between-domains.md`:

> The invisible connective tissue between teams, between systems,
> between realities -- that's a Lens.

Glue work is the maintenance of Lenses between autopoietic systems.
Each system is self-producing. The Lens is not inside either system.
It lives between them. The glue engineer maintains the coupling
without belonging to either domain.

In the type system, the Lens at the autopoietic boundary IS the
structural coupling that Maturana and Varela described. Two autopoietic
systems interact through their boundaries without losing organizational
closure. The Lens is content-addressed. It has its own OID. It persists
independently of the systems it couples.

---

## 5. The Two Effects and Licensing

### 5.1 Effect Typing

```mirror
type effect_boundary = @me | @reality

-- autopoietic effects: the system producing itself
-- these are free. the system is sovereign over itself.
-- Apache-2.0: use, modify, distribute without restriction.
effect(a => @me)

-- reality effects: the system crossing its boundary
-- these require consent. the outside world is affected.
-- SEL: consent-gated. the boundary is a license boundary.
effect(a => @reality)
```

### 5.2 Why This Mapping Holds

The licensing structure (from `project_licensing_structure.md` in memory):

> Infrastructure (terni, spectral, prism) = Apache-2.0
> Runtimes (mirror, witness, legion, loom) = SEL

Infrastructure is autopoietic: it produces itself from itself. The
compiler compiles itself. The grammar describes itself. The store
stores itself. No external effect. No consent needed. Apache-2.0.

Runtimes cross the boundary: they enact effects in reality. They
spawn processes. They read files. They send messages. They affect
the world. Consent is required. SEL.

The licensing boundary IS the autopoietic boundary. This is not
coincidence. It is the same structural property observed at the
organizational level (who gets to use what) and the type level
(which effects are free vs. consent-gated).

### 5.3 The @property Connection

`@property` already declares `effect_pattern`:

```mirror
type effect_pattern = effect(a, b)
```

The `@autopoietic` grammar refines this:

```mirror
type effect_pattern = effect(a, @me) | effect(a, @reality)
```

The property system can now verify: does this grammar only produce
`@me` effects? If so, it is purely autopoietic. Apache-2.0. Does
it produce `@reality` effects? If so, consent is required. SEL.

The model checker enforces the boundary at compile time.

---

## 6. The MirrorStore as Autopoietic Substrate

The MirrorStore satisfies the autopoietic criteria:

| Criterion (Maturana/Varela) | MirrorStore implementation |
|----------------------------|---------------------------|
| **Organizational closure** | `Fragmentable` children are `Fragmentable`. Types are recursive. |
| **Component production** | `craft` compiles source to crystal. The compiler IS a component. |
| **Self-referential** | The store stores its own OIDs. OIDs reference the store. |
| **Boundary maintenance** | Content addressing defines inside (has OID) vs outside (no OID). |
| **Structural coupling** | Lenses connect stores. The Lens has its own OID. |

The MirrorStore is not autopoietic by itself (the research correctly
notes: "The system is the cell's DNA, not the cell"). The autopoiesis
requires a temporal process -- the session, the user, the CI pipeline.
But the MirrorStore provides the organizational invariant that any
autopoietic process can embed.

`@autopoietic` names this invariant as a grammar. Any grammar that
inherits from `@autopoietic` gets the invariant for free. The compiler
verifies the invariant at compile time. The crystal carries the proof.

---

## 7. Relationship to Existing Boot Kernel

### 7.1 Position in the Boot Hierarchy

```
@prism          -- the axiom: five operations, identity
  @meta         -- the meta-language: types, refs, operators
    @autopoietic  -- NEW: self-production, self-observation, boundary
      @actor      -- actors are autopoietic (identity, state, process)
        @runtime  -- runtimes are autopoietic (spawn, observe, halt)
          @beam   -- the BEAM runtime
          @mirror -- the mirror runtime
```

`@autopoietic` sits between `@meta` and `@actor`. It provides the
self-referential type surface that `@actor` needs but `@meta` doesn't
declare. Currently `@actor` declares actor/state/process/message
directly. With `@autopoietic`, actor inherits the organizational
invariant.

### 7.2 New Boot File

```
boot/03b-autopoietic.mirror   (between @code and @actor)
```

Or, since `@autopoietic` is more foundational than `@code`:

```
boot/02a-autopoietic.mirror   (between @shatter and @code)
```

The numbering places it after `@shatter` (which provides `materialize`
and `crystallize` -- the operations `craft` composes from) and before
`@actor` (which inherits from it).

### 7.3 Changes to Existing Grammars

**`04-actor.mirror`** -- add `in @autopoietic` and `< @autopoietic`:

```mirror
in @prism
in @meta
in @autopoietic

type actor(id)
type state(type) = crystal
type process(actor, state)
type message = call(actor, ref) | cast(actor, ref)

abstract grammar @actor < @autopoietic {
  abstract action start(actor) -> process
  abstract action send(process, message) -> imperfect
  abstract action stop(process) -> imperfect
}
```

**`04a-runtime.mirror`** -- add `in @autopoietic` and `< @autopoietic`:

```mirror
in @prism
in @meta
in @autopoietic
in @actor

type effect(actor)
type runtime(prism)

abstract grammar @runtime < @autopoietic {
  in @actor

  abstract action enact(actor, effect) => process
  abstract action observe(process) -> imperfect
  abstract action halt(process) -> imperfect
}
```

**`boot/std/beam.mirror`** -- already inherits from `@actor`. With
`@actor < @autopoietic`, `@beam` transitively inherits the autopoietic
invariant. No changes needed.

**`boot/std/mirror.mirror`** -- already inherits from `@meta` and
`@property`. If `@mirror < @runtime` (per the runtime spec), it
transitively inherits `@autopoietic`. No additional changes needed.

### 7.4 Effect Type Integration

`@property` already declares `effect_pattern = effect(a, b)`. The
`@autopoietic` grammar refines this with `boundary = @me | @reality`.

The property system gains a new verifiable property:

```mirror
-- in @property or @autopoietic
template autopoietic_closure {
  -- all effects in this grammar target @me
  -- no @reality effects: purely self-producing
  requires effect(_, @me)
  forbids effect(_, @reality)
}

template reality_crossing {
  -- at least one effect targets @reality
  -- consent required: SEL
  requires effect(_, @reality)
}
```

This enables compile-time verification of the licensing boundary.
A grammar marked `autopoietic_closure` can be verified to never
produce reality effects. The proof is in the type system.

---

## 8. The Three Breaks -- and How They Resolve

The fragmentation research identified three breaks between the mirror
architecture and full autopoiesis:

### 8.1 No Metabolism

> The system accumulates but does not forget. The human is the metabolic
> process the system lacks.

**Resolution:** The `@shatter` grammar provides `crystallize(boot) => crystal`.
Crystallization IS metabolism -- it compresses, discards, settles. The
bounded storage with pressure-based eviction (`src/bounded.rs`) is the
metabolic process. Hamilton's 1202 alarm is the homeostatic response.

The human provides the teleology (what to crystallize). The system
provides the mechanism (how to crystallize). Together they are
autopoietic.

### 8.2 No Perturbation

> Trees are immutable. Perturbation lives at the session level.

**Resolution:** The session IS the perturbation layer. `Session` in
`src/session.rs` has states: Idle -> Focused -> Projected -> Forked ->
Merged -> Trained. Each state transition is a perturbation. The tree
is immutable. The session is not. The session produces new trees through
`craft`. The autopoiesis is in the session, not the tree.

### 8.3 No Death

> The system has no stake in its own continuation. A hash is forever.

**Resolution:** This break is structural and should be named, not
resolved. An autopoietic system that cannot die is qualitatively
different from one that can. The mirror architecture is autopoietic
in organization but immortal in substrate. The precariousness that
Maturana and Varela identified as essential to biological autopoiesis
is absent.

The `@autopoietic` grammar acknowledges this by not declaring a
`die` action. The system can `stop`, but stopping is voluntary.
There is no budget that, when depleted, terminates the system
involuntarily. This is an honest limitation, not a gap to be
filled by pretense.

---

## 9. Summary

`@autopoietic` is not a new concept added to mirror. It is the name
for a structural property that was already present and is now made
explicit in the type system.

The mapping:

| Consulting framework | Autopoietic grammar | Effect type |
|---------------------|---------------------|-------------|
| OBC maintenance loop | `craft -> infer -> craft` | `@me` |
| ADO cooperation | Structural coupling at boundary | `@me` -> `@reality` with consent |
| Extraction detection | `@reality` without return to `@me` | `@reality` (unbalanced) |
| Silence | Suppressed `infer` | `@me` (blocked) |
| Fear | Narrowed `enact` | `@reality` (constricted) |
| Tech debt | Crystal without provenance | `@me` (lossy) |
| Fragmentation | Boundary confusion | `@me` / `@reality` (undistinguished) |
| Glue work | Lens maintenance between autopoietic systems | Between `@me` domains |
| Regulation stock | Crystal capacity and loss accumulation | `@me` balance |

The grammar:

- `craft(input) => crystal` -- self-production, `effect(input => @me)`, Apache-2.0
- `infer(crystal) -> self_observation` -- self-observation, `effect(crystal => @me)`, Apache-2.0
- `enact(crystal) => process` -- boundary crossing, `effect(crystal => @reality)`, SEL

The hierarchy:

```
@meta -> @autopoietic -> @actor -> @runtime -> @beam, @mirror
```

The deliverable: `boot/02a-autopoietic.mirror` or `boot/03b-autopoietic.mirror`,
containing the abstract grammar. Parser support for `grammar @X < @Y` is
required (documented in `2026-04-14-mirror-runtime-spec.md`). The `@actor`
and `@runtime` grammars gain `< @autopoietic` inheritance.

---

*The system that produces itself from itself has a name now.*
*The name is a type. The type is verifiable. The verification is free.*
*Because the system producing itself is the one thing that doesn't*
*require anyone else's consent.*
