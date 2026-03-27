# 08 — OID-Native Model (Experimental)

## Status: Idea

What if a model operated entirely on content-addressed OIDs instead of
natural language tokens? Language understanding becomes a session-layer
concern — a translation boundary between human and model — not the
model's substrate.

---

## Insight

Every current LLM operates on language tokens. The vocabulary is natural
language — noisy, ambiguous, polysemous. Hallucination is structurally
possible because any token sequence can be produced without constraint.

An OID-native model inverts this. The vocabulary is content-addressed
references. Every token either resolves to something in the store or it
doesn't. The model learns relationships between structural elements —
types, actions, properties, compositions — not between words.

---

## Architecture

### The substrate

The conversation runtime is the model's substrate. The grammar is the
constraint. The model checker is the verifier. The OIDs are the vocabulary.

```
Human  ──language──▶  Session  ──OIDs──▶  Model  ──OIDs──▶  Session  ──language──▶  Human
                      (translate)         (structure)        (translate)
```

The session layer handles language — at the boundary, not at the core.
Like CSS to HTML. The meaning lives in the OIDs. Language is how you
render it for humans.

### Training data

Not internet text. The content-addressed graph. Every relationship is
verified. Every composition is typed. The model learns the structure of
knowledge, not the structure of language about knowledge.

### Hallucination

Structurally impossible. An OID resolves or it doesn't. The model can
only speak in references to things that actually exist in the store.
No RLHF needed to prevent hallucination — the vocabulary can't
hallucinate by construction.

### Verification

Every output sequence is checkable:

- Do these OIDs exist?
- Do they compose according to the grammar?
- Do the declared properties hold?

The model checker runs on the model's output the same way it runs on
any compiled grammar. Verification is free.

### Prediction as projection

The model's "next token prediction" isn't a language guess — it's a
structural projection. The next OID is a content-addressed reference to
something the model expects to follow. That prediction is verifiable
against the actual graph.

Connects directly to [07 — Projection](07-projection.md): the model
emits projections that the model checker can verify before execution.

---

## Properties

| Property | How it holds |
|----------|-------------|
| No hallucination | OIDs resolve or don't — vocabulary can't reference non-existent artifacts |
| Decidable verification | Sub-Turing grammar, model checker runs on output |
| Alignment by construction | Constraints are structural, not trained via RLHF |
| Language-independent | Core operates on structure; any natural language is a session-layer translation |
| Auditable | Every output is a sequence of verifiable references into the content-addressed store |

---

## Open questions

- What is the training objective for an OID-sequence model? Next-OID
  prediction over graph traversals? Property satisfaction? Composition
  completion?
- What is the minimum graph size for meaningful training?
- How does the session-layer translation work? An existing LLM as the
  boundary translator? A learned mapping?
- Can the model discover novel compositions — OID sequences that are
  valid but don't yet exist in the store? That's the generative question.
- What is the relationship between graph density and model capability?
  Does the model get smarter as the store grows?

---

## Connection to existing work

- **[06 — Model Checker](06-model-checker.md)**: verification infrastructure
  runs on model output unchanged
- **[07 — Projection](07-projection.md)**: model predictions are projections
  verifiable against the graph
- **[03 — Shipping](03-shipping.md)**: fragmentation provides the
  content-addressed store the model operates over
- **[04 — Fortran Bridge](04-fortran-bridge.md)**: eigenvalue evaluation
  backs property verification of model output
- **coincidence crate**: the measurement substrate — multiple observers
  arriving at the same OID is the coincidence principle applied to inference

---

## Design principle

Sub-Turing grammar. Decidable verification. Content-addressed vocabulary.
Language as presentation concern. The model speaks structure. The session
speaks language. The boundary between them is the NIF wall between proof
and reality.
