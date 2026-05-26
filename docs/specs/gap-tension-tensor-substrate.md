# `gap` / `tension` / `tensor` — substrate primitives for the proposed loop closure

*2026-05-26. Mara. Proposal — not implementation.*

**Status: Yellow.** The shape emerged in conversation between Alex and Reed on
2026-05-26 (Alex: *“I think `gap` lives in @epistemologic/property and is used
by @fate to build tensors.”*). The types named here are not declared in the
substrate yet. The compiler does not emit gap-typed output today. `@fate.minimize`
is named but has no body. **Every section below tags its altitude: declared,
proposed compiler output, proposed runtime, eventually.** Nothing in this
document runs today except where explicitly noted.

Depends on:
- `boot/std/epistemologic/property.mirror` — the `verdict` type; the
  `check` shape. `gap` extends this altitude by adding state and
  verifier-presence.
- `boot/std/epistemologic/property/halts.mirror`,
  `content_addressed.mirror`, `causality.mirror`, et al. — the concrete
  properties whose claims would surface as gaps when their verifier is
  `\`-bodied or absent.
- `boot/std/fate/tournament.mirror` — `@fate/tournament` already names
  rules and ganglia. `tension` and `tensor` would live as siblings under
  `@fate`, composing tournament selection with gap-gradient backtracking.
- `docs/specs/epistemologic-grammar.md` — the literal property; the
  IS-relationship; the verification-is-measurement frame. Gaps measure
  the distance between claim and verifier.
- `docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md` —
  fracture as the rewrite that closes gaps; confidence threshold for
  autonomous vs scene-dispatched application. `@fate.minimize` would
  emit a fracture sequence.
- `docs/insights/2026-05-26-fixed-and-the-spectral-feedback-fracture.md`
  — the loop closure narrative; what this spec gives a type to.

Unblocks (deferred per LRM until consumers surface):
- `@fate.minimize` body (today: declaration only).
- `mirror compile <file>` gap-typed output mode.
- `@spectral` integration: gap counts contribute to fiedler and
  holonomy measurements on the corpus.
- `@scene` curator UI: present a non-converging tensor as a scene the
  curator enters; consent governs which tension to relax first.

---

## 1. The recognition

A `claim` (the `requires` / `ensures` / `property` declarations scattered
across boot grammars) names what the substrate intends to be true. A
`verifier` is the body that decides the claim. Today many verifiers are
`\` — declared but not executing. The substrate has no first-class way
to talk about *the distance between claim and verification*; it has
`verdict` (pass/fail/partial) but not the meta-shape “this claim has a
verifier-shaped hole.”

`gap` names that meta-shape. `tension` names two gaps in opposition.
`tensor` is a structured collection of tensions — the substrate's input
to a backtracking optimizer. `@fate.minimize(tensor)` would walk the
tensor's gradient and emit a fracture sequence that closes gaps.

The loop, *proposed*:

```
compile  →  tensor  →  @fate.minimize(tensor)  →  fracture sequence  →  apply  →  new tensor  →  ...
```

Today: only `compile` runs. The other arrows are declarations or named
intentions. This spec gives them types so subsequent work can give them
bodies in sequence.

---

## 2. Placement

**`gap` lives in `@epistemologic/property`.** Claims and verdicts are
epistemologic-altitude; the gap between them is the same altitude. A
gap is a richer `verdict` — it carries the claim's text, the verifier's
presence, and a discrete state tier.

**`tension` and `tensor` live in `@fate`.** They compose gaps into
input for backtracking inference. `@fate` already houses the tournament
rules and the five-ganglia structure (`boot/std/fate/tournament.mirror`);
tensor minimization is a sibling shape — not selection across candidates,
but gradient walk across gaps.

This split mirrors the existing altitude separation: `@epistemologic`
measures what is; `@fate` decides what to do about it.

---

## 3. Types (declared shape — bodies are `\`)

### 3.1 `gap` in `@epistemologic/property`

```mirror
in @prism
in @epistemologic
in @epistemologic/property
in @nl

grammar @epistemologic/property {
  # ... existing verdict, check, reflect ...

  # a claim is the textual content of a requires / ensures / property
  # declaration. it names what the substrate intends to be true.
  # the claim's identity is its AST node; the text is the rendered form.
  type claim = {
    node:   ast,                    # the AST node carrying the claim
    text:   string,                 # the rendered claim ("halts(τ)", etc.)
    site:   span                    # source location
  }

  # a verifier is the body that decides the claim. it may be absent
  # (the substrate-level `\` marker) or present (a runnable lambda).
  type verifier = absent | present(check)

  # probability sits in [0, 1]. when a probability typing grammar lands
  # (per @epistemologic/probability, deferred), this alias becomes a
  # refined type. today it's a structural number.
  type probability = f64

  # the state tier. four discrete positions on the verdict manifold,
  # plus the absent corner for claims that COULD exist but DON'T.
  #
  # `verified` and `heuristic(p)` are NOT interchangeable: a heuristic
  # is MARKED as such, not asserted as proof. the substrate makes the
  # distinction structural so downstream consumers (fracture
  # generation, conductivity reporting, fiedler measurement) can weight
  # them differently.
  type gap_state =
      verified                          # verifier ran; returned pass
    | heuristic(probability)            # no verifier; pattern-match confidence
    | declared                          # claim only; no evidence; no estimate
    | absent                            # could be claimed; isn't (surfaced as candidate)

  # the gap itself: claim + verifier-presence + state.
  type gap = {
    claim:    claim,
    verifier: verifier,
    state:    gap_state
  }

  # surface every gap visible in an AST. used by `mirror compile` to
  # emit gap-typed output. proposed; \-bodied today.
  gaps_of(ast) -> [gap] { \ }
}

out claim
out verifier
out probability
out gap_state
out gap
out gaps_of
```

*Altitude: declared shape; bodies are `\`. The substrate would carry these
types; no consumer surfaces them yet.*

### 3.2 `tension` and `tensor` in `@fate`

```mirror
in @prism
in @epistemologic/property         # for gap, gap_state

grammar @fate {
  # a tension is two gaps in structural opposition. the vector names
  # the direction the tension pulls when minimized: which gap closes,
  # at what cost to the other.
  #
  # the vector field is INTENTIONALLY left as a hole at this altitude.
  # whether vector is (delta_a: probability, delta_b: probability) or
  # a richer tangent-space element is a design call Alex has not made.
  # see §8 (design calls flagged).
  type tension = {
    a:      gap,
    b:      gap,
    vector: tension_vector             # \ — see §8
  }

  type tension_vector = \              # design call deferred to Alex

  # a tensor is a structured collection of tensions plus a spectral
  # signature. the fiedler value is the algebraic connectivity of the
  # tension graph — low fiedler means the tensor is loosely coupled
  # (gaps can close independently); high fiedler means the tensor is
  # tightly coupled (closing one gap perturbs many others).
  type tensor = {
    tensions: [tension],
    fiedler:  f64                      # spectral signature; ≥0
  }

  # build a tensor from the gaps surfaced by `mirror compile`. proposed;
  # \-bodied today.
  tensor_of([gap]) -> tensor { \ }

  # the minimize action: walks the tensor's gradient and emits a
  # fracture sequence that closes gaps. proposed; \-bodied today.
  # the fracture type lives in @kintsugi/fracture per
  # docs/specs/kintsugi-fracture-confidence-and-scene-dispatch.md.
  minimize(tensor) -> [fracture] { \ }
}

out tension
out tension_vector
out tensor
out tensor_of
out minimize
```

*Altitude: declared shape; bodies are `\`. `@fate.minimize` is named in
§5 below as proposed runtime; the type signature is what this spec gives
it. The runtime is not built.*

---

## 4. The state tier in detail

*Altitude: declared (the type exists in the substrate once §3 lands).*

Four positions. Each has a structural meaning the downstream consumers
are proposed to respect:

### 4.1 `verified`

The verifier ran. It returned `pass`. The claim and the body agree.
Downstream: contributes positively to conductivity; does not surface
as a fracture candidate.

### 4.2 `heuristic(p: probability)`

No verifier ran. A pattern-match (an `@ai/explorer` ganglion, a
`@ai/introject` similarity check, an `@ai/cartographer` neighbourhood
scan) gave a probability that the claim holds. The substrate carries
the probability AS a probability — not as a verdict. Downstream
consumers MUST treat `heuristic(p)` as conditional evidence, not proof.

The MARKING is the point. Heuristics that masquerade as verifications
are the central failure mode this spec exists to prevent. The type
system refuses to upcast `heuristic(p)` to `verified` without an
actual verifier run.

### 4.3 `declared`

The claim is in the AST. No verifier. No heuristic. The substrate
makes NO estimate. Downstream: fracture candidate (write the verifier);
contributes to gap count but not to confidence.

This is the modal state for boot grammars today. Most `requires`,
`ensures`, and `property` declarations in `boot/std/**/*.mirror` carry
`\` bodies. Each one is a `declared` gap.

### 4.4 `absent`

The claim COULD be made (the property exists in scope; the type
admits it) but ISN'T (the declaration is missing). Downstream:
surfaced as a candidate; not a fracture target until the absence
becomes a tension with another claim.

Proposed mechanism for surfacing `absent`: when a property is `in`-imported
but never invoked on a type that admits it, the compiler can mark the
missing invocation as `absent`. *Eventually*; the import resolver would
need to track admission relations, which it doesn't today.

---

## 5. Compiler integration

*Altitude: **proposed compiler output**. Today `mirror compile <file>`
does not emit gap-typed output. The example below is what it WOULD emit
once `@epistemologic/property.gaps_of` carries a body and the compiler
wires the call into its reporting path.*

```
@spectral/portal.mirror
  declarations: 12
  claims (requires/ensures): 4
    requires content_addressed(portal)   declared; verifier absent
    requires halts(portal.open)          declared; verifier absent
    ensures  monotonic(portal.tick)      heuristic(0.72); no verifier
    ensures  causality(portal.events)    verified
  holes: 5 (line refs: 12, 28, 41, 67, 89)
  gap: 4 claims, 1 verification, 3 declared, 0 absent, 5 dependent holes
```

*Altitude: **proposed**. `mirror compile @spectral/portal.mirror` today
emits the OID, declaration count, and (per the cascade) the gestalt
fiedler. It does NOT emit the per-claim verdict breakdown. The example
is the spec of what gap-typed output would look like.*

The last line is the **gap line** — the per-file shape that composes
upward into the corpus tensor. A tool building the tensor reads gap
lines, not declaration counts.

---

## 6. `@fate.minimize` (proposed runtime)

*Altitude: **proposed runtime**. The action is declared in §3.2 with a
`\` body. Today nothing minimizes tensors; this section names what the
body would do.*

Input: a `tensor` built from corpus gaps.

Output: a `[fracture]` — a sequence of fracture rewrites that, applied
in order, would lower the tensor's energy. Each fracture lives in
`@kintsugi/fracture` per the existing fracture-and-scene-dispatch spec.

Proposed body sketch (declarative; not pseudocode for the runtime):

1. Rank tensions by `vector` magnitude (highest-pull first).
2. For each tension, propose the fracture rule whose application
   closes the higher-confidence gap and reduces the lower-confidence
   gap's opposition. Confidence comes from `kintsugi/fracture` per
   the fracture-confidence spec.
3. Concatenate proposed fractures into a sequence; emit.
4. The caller (proposed: `@kintsugi/scheduler` or a `mirror minimize`
   subcommand) applies the sequence. Autonomous-apply at
   `confidence = 1.0`; scene-dispatched otherwise.

**Non-converging tensors.** When `fiedler` is high and the gradient walk
fails to reduce energy below a threshold within a bounded step count,
`minimize` does NOT return an empty sequence. The substrate MUST surface
non-convergence as a first-class signal — see §8 (design call).

---

## 7. Migration order

What lands, in sequence, to close the loop. Each step depends on the
previous. None of these have landed.

1. **Types declared.** `gap`, `gap_state`, `claim`, `verifier`,
   `tension`, `tension_vector`, `tensor`. Per §3. `\` bodies. Just the
   substrate shape; nothing executes.

2. **Compiler emits gap-typed output.** `mirror compile` invokes
   `@epistemologic/property.gaps_of` per file and prints the gap line
   per §5. This requires `gaps_of` to carry a body (walks the AST,
   classifies each `requires` / `ensures` / `property` declaration into
   a `gap_state`). The classification logic is non-trivial — it has to
   distinguish `\`-bodied (declared) from `=`-bodied (verified) from
   heuristic-evidenced (no body, but a confidence signal from elsewhere
   in the corpus).

3. **`tension_of` and `tensor_of` carry bodies.** Composition rules
   for when two gaps are in opposition (proposed: when their claims
   reference overlapping sites and their verifier states differ). The
   fiedler computation reuses the existing spectral measurement
   substrate per `docs/specs/eigenboard-representation.md`.

4. **`@fate.minimize` carries a body.** Per §6. Requires the fracture
   catalog to be populated beyond `@kintsugi/fracture/generic-brackets`
   so a sequence has anything to draw from.

5. **Fracture sequence execution.** The caller wires `minimize` output
   into `@kintsugi/fracture` application. Autonomous at
   `confidence = 1.0` per the existing fracture-confidence spec;
   scene-dispatched below.

6. **Loop closure.** The composed pipeline runs continuously. New gaps
   surfaced by post-fracture compilation feed the next tensor.

Step 1 is small. Step 2 is the largest single piece of work (the
classifier). Steps 3–6 each depend on the previous.

---

## 8. Design calls flagged

These are decisions Alex has not made. The spec lays out the shape;
the substantive choice belongs upstream.

### 8.1 `tension_vector` structure

What IS the vector field on `tension`?

- **Option A:** `(delta_a: probability, delta_b: probability)` —
  scalar deltas, one per gap. Simple; loses information about whether
  the gaps share a common verifier-shape.
- **Option B:** A tangent-space element on the gap manifold — richer;
  composes naturally with fiedler; requires defining the manifold.
- **Option C:** A symbolic expression in `@nl` — the vector IS the
  claim of how-they-oppose; readable; harder to compute over.

*Default-not-chosen.* The spec carries `tension_vector = \` and waits
for a consumer to make the call. No fracture sequence depends on this
yet.

### 8.2 Heuristic confidence ↔ verdict tier interaction

When a `heuristic(0.95)` gap and a `declared` gap conflict, which wins?
The spec currently treats `heuristic` and `declared` as PARALLEL
positions on the manifold — neither dominates. But in practice,
fracture ranking has to choose one to address first.

- **Option A:** Verifier-presence dominates probability. A `declared`
  gap (verifier-shaped hole) is more urgent than a `heuristic(0.95)`
  gap (no verifier; high pattern-match).
- **Option B:** Probability dominates verifier-presence. A high-
  confidence heuristic outranks a low-evidence declared claim.
- **Option C:** Both contribute to a composite urgency score; weights
  configurable per `@scene`.

*Default-not-chosen.* `@fate.minimize`'s ranking depends on this.

### 8.3 Non-converging tensors

When `minimize` fails to lower energy, what does it return?

- **Option A:** Empty `[fracture]` + diagnostic to the caller's log.
- **Option B:** A `non_converging(tensor, last_attempt: [fracture])`
  variant on the return type, surfacing the failure structurally.
- **Option C:** Throw a scene to the curator: “this tensor doesn't
  resolve; you decide what to relax.”

*Default-not-chosen.* Option C composes nicely with the existing
fracture-confidence spec but expands the type signature.

---

## 9. Honesty markers — what executes today vs what is proposed

A per-section legend so readers can scan the operational state at a
glance:

| Section | Altitude | Executes today? |
|---------|----------|-----------------|
| §1 The recognition | Narrative | No — names a proposed loop |
| §2 Placement | Declared (proposed) | No — grammars don't exist |
| §3.1 `gap` types | Declared shape | No — bodies are `\` |
| §3.2 `tension` / `tensor` types | Declared shape | No — bodies are `\` |
| §4 State tier | Declared | No — type doesn't exist yet |
| §5 Compiler integration | Proposed compiler output | No — `mirror compile` doesn't emit gap lines |
| §6 `@fate.minimize` | Proposed runtime | No — declaration only |
| §7 Migration order | Roadmap | N/A — sequencing document |
| §8 Design calls | Open questions | N/A — awaits Alex's decision |

The one substrate primitive that EXISTS today and underlies this whole
spec: `verdict` in `boot/std/epistemologic/property.mirror`. Everything
else is the proposed extension on top of it.

---

## 10. Provenance

- Alex 2026-05-26 (conversation with Reed): *“I think `gap` lives in
  @epistemologic/property and is used by @fate to build tensors.”*
- Reed and Alex worked through the type shape and the proposed loop
  closure (`compile → tensor → minimize → fracture → apply → tensor`).
- Mara crystallised the spec on `mara/shard-chain` per the tick-2
  scope: declared shapes, honesty markers, design calls flagged,
  migration order named.
- The substrate has carried `verdict` since the bootstrap; this spec
  proposes the extension that closes the feedback loop named in
  `docs/insights/2026-05-26-fixed-and-the-spectral-feedback-fracture.md`
  (post-edit: “distinguish declared from executing”).
