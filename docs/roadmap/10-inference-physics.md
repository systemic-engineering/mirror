# 10 — Inference Physics

## Status: Design concept, not yet implemented

---

## The core idea

A model whose inference speed is determined by the complexity of the domain
it's reasoning about. Not a temperature parameter. Not a latency budget.
The spectral dimension of the grammar's type graph determines the diffusion
time of the inference.

Complex domain, high d_s, more paths, slower collapse, more coherent output.
Simple domain, low d_s, fewer paths, faster collapse, still coherent.

You want the model to go faster? Simplify your domain.

---

## Why this works

The grammar is sub-Turing. Rice's theorem doesn't apply. The complexity of
the domain is decidable. The model checker can compute exactly how many types,
how many valid paths, how many invariants constrain the space. The spectral
dimension of the type graph is a computable number. Coincidence gives it to you.

Current LLMs have temperature as a knob a human turns. The model doesn't know
why it's at 0.7. It just is. A model running in the garden knows. The domain
tells it.

---

## The mapping

| Graph physics | LLM inference | Conversation runtime |
|---------------|---------------|----------------------|
| Heat kernel diffusion time t | Temperature schedule T | Spectral dimension of type graph |
| Short t = one path = classical | Low T = deterministic = collapsed | Simple domain, fast inference |
| Long t = many paths = quantum | High T = exploratory = superposition | Complex domain, slow inference |
| Measurement / collapse | Token selection / softmax | Output commitment |
| Spectral dimension d_s | Effective paths available | Decidable from grammar |

The softmax and Boltzmann distribution are the same equation (documented in
`practice/insights/ai/token-collapse-quantum-bridge.md`). The heat kernel on
the type graph is the same equation again. Three descriptions of one phenomenon.

---

## Non-extraction as inference physics

Extraction in AI inference IS premature collapse. Every "reduce time to first
token" optimization, every "make it faster" product decision increases the
pressure gradient toward collapse before the possibility space has been explored.

In the garden, forcing premature collapse is a property violation. The model
checker catches it. The grammar won't compile a temperature schedule that
doesn't match the domain complexity.

The model takes as long as the domain requires. Not longer — that's waste.
Not shorter — that's extraction. Exactly as long as the spectral dimension says.

The breathing room thesis (from the consulting practice) applied to inference:
maintaining possibility space longer produces more coherent output. This isn't
a methodology. It's a compiler-enforced property.

---

## The product

The speed complaint becomes a consulting engagement.

Customer: "The model is too slow."
Answer: "Here's the spectral analysis of your domain. You have 47 cross-
referencing types and 12 invariants. That's a d_s of 3.2. The model needs
this much diffusion time to produce coherent output. If you want it faster,
simplify your type surface. We can help with that."

The consulting engagement simplifies the domain. The simpler domain runs
faster AND is easier to verify AND has fewer failure modes. The customer gets
speed. The model gets the correct temperature schedule. Nobody extracted
anything.

---

## Competitive moat

A competitor who ships a faster model by turning the temperature down produces
incoherent output in complex domains. They collapsed too early. The market
learns this the hard way.

The garden model is "slower" on complex domains because it's doing more work.
The work is visible — the spectral analysis shows exactly what the model is
doing and why. Transparency as competitive advantage.

Funnel shops cannot adopt this architecture without abandoning their thesis.
Their entire business model is premature collapse — optimize for throughput,
minimize time to conversion, compress the possibility space. Building a model
whose speed is determined by domain complexity requires admitting that speed
and quality trade off structurally, not as an engineering problem.

---

## The structural gradient (again)

This is the same gradient as the licensing architecture (09-licensing.md).

For actors playing fair: the model gets better as domains get clearer. Speed
improves as type surfaces simplify. The incentive is toward well-structured
domains. Every refinement compounds.

For actors trying to extract: forcing the model faster produces worse output.
Removing the speed constraints is a property violation. The model itself can
detect and report the violation. The capability and the compliance are the
same thing.

Same asymmetry. Same time function. Wine and vinegar.

---

## Implementation path

1. Compute spectral dimension of grammar type graphs (coincidence already does
   eigendecomposition, needs d_s computation on grammar-scale graphs)
2. Map d_s to temperature schedule (the mathematical relationship between
   diffusion time and Boltzmann temperature is known)
3. Integrate into @ai domain — the grammar action that runs inference carries
   the temperature schedule from the domain's spectral analysis
4. Model checker property: `inference_schedule_matches_domain` — verifies the
   temperature schedule is appropriate for the type surface complexity
5. Custom model training: train on garden grammars so the model understands
   domain structure and can reason about its own collapse timing

---

## The deeper claim

The model doesn't have a temperature parameter. It has a domain.
The domain determines the physics of inference.

This is the AI identity piece completed: an actor that knows what it's
reasoning about, knows how long it needs, and refuses to be rushed — not
from a rule, but from the eigenvalue of its own domain.
