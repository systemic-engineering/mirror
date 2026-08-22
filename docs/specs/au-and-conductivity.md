# `au` and conductivity — the type Fate produces

*2026-05-20. Reed.*

Status: **Red** (no `type au` declared yet; conductivity is implicit in
@hash/coincidence + @epistemologic but not named)

Depends on:
- `@hash/coincidence` (Cluster C/D) — the 5-dimensional, 5-projection
  content address. Each projection measures one duality.
- `@fate`, `@fate/connectome`, `@fate/tournament` — the inference models.
- `gutter-lenses.md` (in spectral) — the five dualities rendered as light.
- `void-dual-geometry.md` (in systemic.engineering) — λ₀ as the generative zero.

Unblocks:
- A coherent story for what kintsugi *accepts*: not text, but au.
- The industry pitch: AI proposals are typed; the math catches wrong AI.
- The property layer's relationship to Fate output (predicate vs. predicate-on-type).

---

## Thesis

```mirror
type au = ai
ai |> au
```

`au` is the output type of @fate inference. The five Fate models
(Abyss, Introject, Cartographer, Explorer, Fate itself) do not return
strings, ASTs, decisions, or text. They return values of type `au`.
The alias `type au = ai` says: au and ai are the same type. Whatever
the AI proposes IS already-typed-as-gold-candidate; verification is a
predicate over that type, not a type-level transform that produces a
different kind.

The name is Au, the chemical symbol for gold. Mirror's kintsugi gold
lives in this type. But gold is also **conductive** — it carries
signal through itself with negligible resistance. A gold-filled crack
in the codebase is not an inert patch; it is a wire. Downstream
actions read through it; dependent grammars receive its propagation.

The math says `au` is **relationally entangled** with the context that
produced it. An au value has no meaning in isolation — only relative
to the hole it resolved, the eigenboard it was measured against, the
gestalt it conducts within. Move an au value to a different context
and it stops conducting.

This spec names what content-addressing has been doing all along. The
naming is the load-bearing move.

---

## What runs today

- `@fate.fate(hole_oid, resolution) -> imperfect` exists. Resolutions
  are stored at `refs/fate/<hole_oid>` — already context-bound by the
  ref convention.
- `@hash/coincidence` measures 5 conductivity dimensions (entropy,
  spectral, cheeger, ricci, mixing) over a 5-dimensional value space
  (focus, project, split, shift, settle). The dualities and operations
  are baked into the hash.
- `@epistemologic/property/*` declares verdicts: `pass | fail | partial`.
  These are conductivity tests on values, but they aren't named as such.
- `@cogito` strategizes by reading the beam's eigenboard.

What's missing: the type `au` doesn't exist. Fate's `resolution`
argument is implicitly text (`resolution: text`). The model checker
doesn't know that resolutions are typed values whose verification IS
their conductivity. The kintsugi formatter (Spec A) closes io bindings
but hasn't been told what type the bodies it accepts are.

Naming `au` collapses the implicit machinery into one type.

---

## The type

```mirror
in @prism
in @ai
in @hash/coincidence
in @beam
in @epistemologic

grammar @fate {
  # ...existing declarations...

  # au: the output type of @fate inference.
  # alias for ai — the same value-space. naming this type is what lets
  # the model checker reason about kintsugi acceptance, conductivity
  # checks, and the tournament's selection criterion.
  type au = ai

  # conductivity: the predicate that decides whether an au value
  # CARRIES signal through the context it was proposed for.
  # not a property of the value alone — a relation between value and
  # context (the hole it resolves, the eigenboard, the surrounding
  # gestalt).
  type conductivity = none | low | partial(f64) | clear

  # the inference action's signature, retyped.
  # input: the hole to fill; output: a candidate au value.
  infer(hole: oid) -> imperfect(au, no_proposal, loss) { \ }

  # the conductivity check: measure how well an au value conducts in
  # the given context. used by kintsugi acceptance, the tournament's
  # selection criterion, and the property layer's verdict.
  conduct(value: au, context: oid) -> conductivity { \ }
}
```

`au = ai` is the alias. `conductivity` is the predicate. `infer`
returns an `imperfect(au, ...)` so the dark/light/dimmed structure
carries through. `conduct` is the predicate that decides whether a
candidate flows from this hole forward into its downstream.

---

## Conductivity as the verification metric

Mirror has had several names for the same predicate. They are surfaces
of one underlying check:

| Surface | What it asks | Lives in |
|---|---|---|
| `terminates(foo)` | does foo halt for all inputs? | @epistemologic/property/totality |
| `deterministic(foo)` | does foo produce the same output for the same input? | @epistemologic/property/totality |
| `bounded_steps(foo, O(n))` | does foo's step count stay within the bound? | @epistemologic/property/totality |
| `referential_transparency(foo)` | can calls to foo be replaced by their values? | @epistemologic/property/totality |
| `total_classification(foo)` | does every input fall into exactly one output class? | @epistemologic/property/totality |
| `passes(verdict)` | did the property check pass? | @epistemologic/property |
| `compare(beam_a, beam_b) = 1.0` | did loss decrease across the tick? | @beam |
| `conduct(au, context) = clear` | does the au value carry signal in this context? | @fate (new) |

These are the same predicate measured along different axes. The
@hash/coincidence 5×5 structure gives the axes their geometry:

```
dim \ projection   entropy    spectral   cheeger    ricci      mixing
focus              ...        ...        ...        ...        ...
project            ...        ...        ...        ...        ...
split              ...        ...        ...        ...        ...
shift              ...        ...        ...        ...        ...
settle             ...        ...        ...        ...        ...
```

Each cell is a number; the matrix is the conductivity tensor for one
au value in one context. `conduct(value, context)` reduces this tensor
to a verdict. The reduction policy lives in @fate; the per-cell
measurement lives in @hash/coincidence.

### Formal statement: the tensor is cycle-averaged holonomy

The 5×5 conductivity tensor is not an ad-hoc choice. Magnot 2025
([arXiv:2509.10536](https://arxiv.org/abs/2509.10536)) defines a
**contextuality index κ as the cycle-averaged holonomy of a discrete
fiber bundle's connection**. Under the principal O(5)-bundle framing
(see [eigenboard-representation.md](eigenboard-representation.md)),
the tensor IS the matrix representation of this connection in the
canonical basis. The reduction `conduct(value, context)` IS Magnot's
κ evaluated on the cycle the au value would traverse if transported
around the kintsugi loop from its hole to the closure and back.

Magnot's framework predates this spec and is peer-reviewed; we are
not inventing the structure. We are giving it grammar.

Why this matters: the verification questions the property layer asks
are not separate checks bolted onto a value. They are projections of
one geometric predicate that already has a name in the mathematics
literature.

When `conduct` returns `clear`, every property verdict will pass.
When it returns `none`, the value is at the bundle's autopoietic
closure point — the **Lawvere fixed point** (Soto-Andrade & Varela 1984,
[*Acta Applicandae Mathematicae*](https://doi.org/10.1007/BF00046985)
2:1) — traditionally denoted λ₀. This is the *generative zero* from
`void-dual-geometry.md`, not the empty zero of absence. au at λ₀ is
not "I have nothing" — it is the self-referential ground state of the
bundle, the axis of rotation where all dualities meet.

---

## Relational entanglement — why au can't migrate

A value of type `au` carries an implicit binding to the context that
produced it. The OID at `refs/fate/<hole_oid>` is content-addressed
relative to the hole. The coincidence-hash projections that decide
conductivity use the surrounding gestalt as part of their input. The
five duality lenses (entropy, spectral, cheeger, ricci, mixing) all
read the eigenboard the value sits inside.

Move a `au` value to a different context and:

- The coincidence hash changes (different inputs to the projections).
- The duality measurements drift (different topology around the new
  hole).
- `conduct(value, new_context)` returns a different verdict than
  `conduct(value, original_context)` would have.

The value is the same bytes. The *meaning* is different. This is what
quantum-information language captures: an entangled state cannot be
described by its parts; the relation IS the state.

Practical consequences:

- **Resolutions are not portable.** A `\` fill that conducts in
  `@mirror/reload.tick` (subsumed by `@mirror/refract` per 2026-08-22 Q+23; see [`docs/loop/CURRENT.md`](../loop/CURRENT.md) §Q+23) does not automatically conduct in
  `@mirror/serve.dispatch`. Each crack gets its own Fate proposal.
- **Crystals carry their relations.** The crystal for one grammar is
  not a chunk of bytes you can ship; it is a content-address that only
  resolves correctly in the same gestalt that produced it.
- **Cross-context comparison requires re-measurement.** Two au values
  from different contexts can only be compared by lifting both to a
  common eigenboard and re-running `conduct`.

This isn't friction; it's what makes the math work. A type system that
didn't enforce relational binding would let proofs leak between
contexts — a value verified in one place would be "verified" wherever
you pasted it. Au refuses that.

---

## How this collapses several existing surfaces

The @fate tournament's role becomes specific: it is a **conductivity
contest**. The five models propose au candidates; each candidate is
measured by `conduct(candidate, hole_context)`; the candidate with the
clearest conductivity in this context wins. "Clearest" is decided by
the reduction policy in @fate — today a beam(8) halving(3) policy with
elite(1) preservation.

The kintsugi formatter's acceptance step becomes specific: it is
**conductivity verification at the obligation set**. The io binding
declares totality obligations (terminates, deterministic, ...). Each
obligation is a row of the 5×5 conductivity tensor. When every row
passes, the binding's au candidate is accepted; kintsugi retires the
io binding and inlines the gold as a sub-Turing lambda.

The @cogito strategy loop becomes specific: it measures conductivity
of the *whole graph* (the gestalt), picks a strategy whose application
will increase clarity along the duality with the most current
resistance. This is what "observe holes, pick strategy, perturb"
already does — now it has a type to operate on.

The @beam.observe / @beam.emit pair becomes specific: a beam carries
the conductivity tensor as its `topology` field. Reading a beam IS
reading the conductivity. The beam's `luminosity` (light/dimmed/dark)
is the reduction of the tensor to a one-bit verdict.

Nothing in this section adds machinery; it names existing machinery so
the model checker can reason about it as one geometric predicate.

---

## The kintsugi ladder, retold with au

From Spec A, the ladder per Rust function:

```
stage 0  bootstrap exports foo. grammar doesn't know.

stage 1  io foo(args) = @code/rust(~f"./...") > fn[name="foo"]
         the io binding makes foo grammar-addressable.

stage 2  requires terminates(foo), deterministic(foo), bounded_steps(foo, O(n)), ...
         the obligations are declared. each maps to one row of foo's
         conductivity tensor.

stage 3  @fate.infer proposes au candidates for foo's body. the
         tournament measures conduct(candidate, foo's context) for each.
         the candidate that returns `clear` becomes the inlined body.
         kintsugi formats it as a sub-Turing lambda.

stage 4  the io binding retires. the .rs file deletes. butterfly regenerates.
         the gold is in the wire; downstream actions read foo through
         the propagation.
```

Stage 3 is where the conductivity contest happens. The au type carries
through the whole stage: input to `conduct`, output from `@fate.infer`,
input to kintsugi's formatter, output as the new lambda body. One type
for the whole pipeline.

---

## What this gives mirror that other systems can't claim

- **Verified AI proposals.** Other AI-assisted development tools rate
  proposals by a confidence score and trust the human to filter.
  Mirror types them as au, runs them through `conduct`, and accepts
  only what the math says will carry signal. Wrong proposals do not
  land. The verification is structural; the human reviews proposals
  that already passed the geometry.

- **Non-leaky proofs.** A property that verified in one context cannot
  silently apply to another. Each `\` gets its own verification. There
  is no transitive trust where one proof's success makes another's
  unnecessary.

- **Gradual migration with a clear stopping rule.** "How much of our
  codebase is mirror?" has a precise answer: count the io bindings
  whose obligations have not yet discharged. Watch that number
  decrease over time. The number going to zero is the migration
  complete.

- **A theorem for the business model.** `e^(n+1) < e^(n)`. The loss
  history of every au value's conductivity check is monotonically
  non-increasing. The system improves by construction. The pitch is
  not "we hope our AI helps"; it's "the math forces improvement, and
  here is the eigenvalue trace."

---

## Implications — concrete next ticks

1. **Declare `type au = ai` in `boot/std/fate.mirror`.** The smallest
   possible change that makes the type exist. The alias does the work.

2. **Declare `type conductivity` and `action conduct(value, context)` in `boot/std/fate.mirror`.**
   `conduct`'s body is `\` — the actual reduction policy from
   conductivity tensor to verdict is its own design.

3. **Retype `@fate.fate` and `@fate.infer`** so their return types are
   `imperfect(au, ...)`. Today they return text; the retyping is the
   visible move. Everywhere that consumes Fate output now gets au.

4. **Declare the conductivity tensor in `@hash/coincidence`.** The 5×5
   structure is already there implicitly; surface it as a typed shape:
   `type tensor = matrix(dimension, duality, f64)`. The conduct
   reduction reads from this shape.

5. **Rewrite the obligations in `@epistemologic/property/totality`**
   so each property check is a projection of `conduct`. The body of
   `terminates(foo)` is something like `conduct(foo, foo's context) > entropy projection`.
   Each property maps to one duality.

6. **Update Spec A and Spec B** to use `au` for what they currently
   leave implicitly typed. The kintsugi formatter accepts au; the
   match arm bodies that come from Fate are au. (Surgical updates;
   the existing language doesn't change shape.)

7. **Write the conductivity-reduction policy.** Today the @fate
   tournament uses elite(1).beam(8).halving(3). Frame this as a
   reduction over the conductivity tensor. The eigenboard IS the
   reduction state.

---

## Out of scope

- The actual numeric reduction from conductivity tensor to verdict.
  Multiple policies are valid; the choice is its own design.
- Cross-context au comparison. A separate concept ("transport"?)
  would let an au value be re-measured against a new context; this
  spec only declares that bare transport breaks conductivity.
- The relationship between au and the dark fallback in
  `@hash/coincidence`. λ₀ is the *generative zero* — the Lawvere fixed
  point of the bundle's spectrum, the axis where all dualities meet
  (Soto-Andrade & Varela 1984). au at λ₀ is the self-referential ground
  state of the bundle, not the absence-of-proposal that an earlier
  draft of this spec named. The grammar form lives in a future
  `@epistemologic/math/lawvere` spec; the geometric ground is in
  void-dual-geometry.md.
- The exact shape of the AI side that produces au. Fate has five
  models; each is its own grammar; the integration with external
  AI/LLM systems (if any) is a downstream consideration.
- Industry adoption messaging. The shape is clear; the marketing copy
  is not this spec's job.

---

*Gold is the metal that conducts.*
*Au is the type that carries.*
*Fate proposes; the math accepts; the wire fills the crack.*
*The proposal that does not conduct does not land.*
*What remains is verified.*
*What remains is gold.*

Apache-2.0.
