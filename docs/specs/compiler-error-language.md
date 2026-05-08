---
# Compiler Error Language
## A Spec for Load-Bearing Questions in spectral/mirror

*2026-05-08. Reed + Alex.*

---

## 1. Theoretical Grounding

### The Question as Intervention

The Milan systemic school (Selvini Palazzoli, Boscolo, Cecchin, Prata, 1975-1980)
developed circular questioning as a therapeutic technique grounded in Bateson's
epistemology. The core insight: the observer and the observed system cannot be
separated. The therapist's question is not neutral data collection — it is an
intervention that changes the system by making it observe itself.

Applied to compilers: the error message + question is the compiler observing the
codebase observing itself. The codebase that can observe itself changes.

**Bateson:** "Information is a difference that makes a difference." The error message
without a question delivers a difference (here is what is wrong). The question
delivers a second-order difference: it makes the invisible difference between the
current topology and a healthy topology visible through the act of trying to answer.
The engineer who cannot answer the question has learned something the measurement
alone could not convey.

**Second-order cybernetics (von Foerster):** First-order cybernetics observes systems.
Second-order cybernetics observes the observer. When the compiler asks "how can you
route around the auth service?", it is not asking about the auth service — it is asking
the engineer to observe their own assumptions about dependency. The inability to answer
IS the diagnosis. The compiler has made the engineering team part of the system it
observes.

### Question Types (Mapped from Therapy)

**Circular questions (Milan school):** Ask about relationships between components,
not about individual behavior. "How can you route around X?" asks about the relationship
between X and everything that depends on it — not about X itself. The question
presupposes that routing around is possible; if the engineer discovers it isn't,
they have found the hub. These are appropriate for errors: the topology is currently
broken and the question surfaces the breakage.

**Exception questions (solution-focused, de Shazer/Berg):** "When is this NOT a
bottleneck? What's different then?" Points at the conditions under which the topology
is healthy. Appropriate when the issue is a threshold violation, not a structural
certainty — the question reveals what parameters matter.

**Future-oriented questions (solution-focused):** "What would this look like when it's
fixed?" Appropriate for warnings, not errors. The situation is not yet broken; the
question orients toward the healthy state rather than cataloging what's wrong.

**What is NOT appropriate:**
- Scaling questions ("on a scale of 1-10...") — too vague for code; not answerable in code.
- Deficit-focused questions about the developer ("why did you write it this way?") — violates
  the relational frame; the question is about the code, not the person.
- Questions that presuppose intent ("did you mean to create a hub?") — irrelevant; the
  topology is wrong whether or not it was intended.

### The Grammar of the Load-Bearing Question

The question is load-bearing when:
1. It is answerable only in code, not in prose.
2. The inability to answer it IS the diagnosis.
3. It names the specific artifact, not a generic category.
4. It asks about a relationship, not a property.

A question that can be answered with "yes, because..." is not load-bearing.
A question that forces the engineer to trace a path through the actual graph — and
discovers there is no such path — is load-bearing.

---

## 2. Question Taxonomy by Error Type

### Narcissus Battery — Structural Eigentests

The Narcissus battery runs 8 structural eigentests on the graph. Each maps to
a specific question type. The table below is derived from the @trace/* dimension
mapping in trace-grammar-family.md.

---

**E1 — Betweenness Centralization**
(@trace/coupling — hub bottleneck)

- **The load-bearing question:** "How can you route around [node name]?"
- **Question type:** Circular (Milan school)
- **What the inability to answer reveals:** There is no path that doesn't go through
  this node. The node IS the topology, not a participant in it. The centralization
  is structural, not incidental.
- **How @nl generates it:** Node name comes from the graph node with highest betweenness
  score. "Route around" is the canonical framing — it presupposes alternative paths exist
  and forces the engineer to prove it or discover they cannot.
- **Generation rule:** Always circular. First offense: append "(There should be at least
  one path that doesn't require it.)" Repeated pattern: omit the hint — the engineer
  already knows.

---

**E2 — Degree Gini**
(@trace/extraction — load concentrated)

- **The load-bearing question:** "Which [node type] is doing the most work? Could any
  of it move?"
- **Question type:** Circular (relationship between load distribution and architecture)
- **What the inability to answer reveals:** The engineer cannot identify what "the most
  work" means — the responsibilities are not legible enough to be distributed. High
  degree Gini with unidentifiable load = extraction pressure (D8): the cognitive cost
  is real but invisible.
- **How @nl generates it:** Node type from graph schema. Gini coefficient and the
  specific high-degree nodes provide the "which" answer. The "could any of it move"
  presupposes that the engineer has already thought about this and finds the obstacle.
- **Generation rule:** Circular on first offense. If Gini > 0.6 and the same node
  cluster has been high-degree for > 3 ticks: use exception framing — "When was [node
  cluster] not the center of gravity?"

---

**E3 — Spectral Ratio (lambda_{n-1} / lambda_1)**
(@trace/coupling — diverges with size)

- **The load-bearing question:** "What happens to [dependent node list] when [hub name]
  scales?"
- **Question type:** Future-oriented (the damage is potential, not yet realized)
- **What the inability to answer reveals:** The engineer hasn't modeled the scaling
  behavior of the dependent cluster. The spectral ratio violation means the graph becomes
  more centralized as it grows — not less. The question surfaces that the current
  architecture has an asymptotic failure mode.
- **How @nl generates it:** Hub name from highest eigenvector centrality. Dependent node
  list from the first 3 nodes with highest spectral coupling to the hub. "Scales" makes
  the future-orientation explicit.
- **Generation rule:** Future-oriented when ratio > 2.0 (warning threshold). Circular
  when ratio > 4.0 (error threshold) — at that point the damage is no longer potential.

---

**E4 — Von Neumann Entropy**
(@trace/fidelity — minimum information)

- **The load-bearing question:** "Where does [information type] go if [node name] is
  unavailable?"
- **Question type:** Circular (relationship between information flow and node availability)
- **What the inability to answer reveals:** The information has no alternative path. Low
  Von Neumann entropy means the graph carries very little actual information — most edges
  are redundant or the structure is nearly degenerate. When the question cannot be
  answered, the "information type" is being silenced (Piece - Silence: "silence is a
  lawful adaptation to systems that punish signal").
- **How @nl generates it:** Information type from the node's semantic type in the grammar.
  Node name is the low-entropy node. "Unavailable" is more honest than "fails" — it
  captures both intentional removal and failure.
- **Generation rule:** Circular always. This is the silence pattern in code. The
  question should feel quiet, not alarmed.

---

**E5 — Clustering Coefficient**
(@trace/coupling — near zero)

- **The load-bearing question:** "What connects [node name]'s neighbors to each other,
  without going through [node name]?"
- **Question type:** Circular (the relationship between neighbors reveals the hub's role)
- **What the inability to answer reveals:** The neighbors have no relationship to each
  other except through this node. The node is not a coordinator — it is a gatekeeper.
  Removing it would disconnect the neighborhood entirely, which is the definition of
  a structural hub.
- **How @nl generates it:** Node name from the lowest-clustering node above the degree
  threshold. "Without going through" is precise — it encodes the exact structural test.
- **Generation rule:** Circular. Particularly relevant for services that describe
  themselves as "orchestrators" — the question makes the difference between orchestration
  and gatekeeping visible.

---

**E6 — Peripheral Conductance**
(@trace/scope — zero, no boundary)

- **The load-bearing question:** "How does a new [component type] reach [boundary name]
  without touching [node name]?"
- **Question type:** Circular (relationship between new components and the boundary)
- **What the inability to answer reveals:** All paths to the boundary go through this
  node. The OBC scope discipline (D9) has been violated: the component's Observable
  touches things that are not declared in its interface. The boundary is not permeable
  — it has a single gatekeeper.
- **How @nl generates it:** Component type from the grammar's schema for the peripheral
  nodes. Boundary name from the graph's declared scope boundaries. Node name from the
  low-conductance node.
- **Generation rule:** Circular. Pairs with scope violation errors (D9). The question
  reveals both the topology problem and the OBC non-compliance simultaneously.

---

**E7 — Single-Node Fragility**
(@trace/decidability — single point of failure)

- **The load-bearing question:** "What's the first thing that fails when [node name] is
  unavailable?"
- **Question type:** Circular (cascading relationship)
- **What the inability to answer reveals:** One of two things: (a) the engineer knows
  exactly what fails and that answer is alarming, or (b) the engineer cannot enumerate
  it because the cascade is too broad to name. Both are diagnostic. The question
  distinguishes known fragility (acceptable if owned) from unknown fragility (the real
  problem).
- **How @nl generates it:** Node name from the highest fragility score. "First thing"
  makes the cascade order visible — not "what fails" but "what fails first."
- **Generation rule:** Circular. This is the primary HAL eigentest — the opening line
  for this error is "I can't let you do that, [node name]." The question follows the line.

---

**E8 — Permeability Index**
(@trace/honesty — opaque, system hides)

- **The load-bearing question:** "What can you observe about [node name]'s behavior
  from outside it?"
- **Question type:** Circular (relationship between the node and its observers)
- **What the inability to answer reveals:** The node's behavior is not observable from
  outside — it is a black box with effects. This is the convergence honesty violation
  (D12): the node is going silent. The `!` that never gets revisited. Code that swallows
  exceptions rather than surfacing them.
- **How @nl generates it:** Node name from the lowest permeability node. "From outside"
  encodes the observer position precisely — not "what does it do" (which invites
  implementation knowledge) but "what can you observe" (which requires external
  measurability).
- **Generation rule:** Circular. Exception framing when there is a prior period of higher
  permeability in git history: "When could you observe [node name]'s behavior from
  outside? What changed?"

---

### Inference Operator Failures

---

**I1 — Hole too large (`\` with eigenvalue above budget threshold)**

- **The load-bearing question:** "What's the smallest type change that would close this
  gap?"
- **Question type:** Future-oriented (orients toward the solvable, not the impossible)
- **What the inability to answer reveals:** The types are too far apart for any
  reasonable transformation. The Connes distance `d(@code/rust, @nl)` is genuinely
  large. The engineer either needs to introduce an intermediate type or accept that
  the gap requires explicit implementation rather than inference.
- **How @nl generates it:** From the input type, output type, and current eigenvalue
  of the hole. "Smallest" is specific — it asks for a minimal intervention, not a
  redesign.

---

**I2 — Forced inference (`\!` override)**

- **The load-bearing question:** "What would need to be true for this to compile
  without `!`?"
- **Question type:** Exception (what conditions make the forced override unnecessary?)
- **What the inability to answer reveals:** The `\!` is permanent, not temporary. The
  engineer is accepting ongoing incoherence rather than resolving the underlying gap.
  This is the settlement pattern — `\!` is honest about where you are, but "what would
  need to be true" surfaces whether "where you are" is a position or a destination.
- **How @nl generates it:** From the hole's type context and the graph state at the
  time of the `!`. "Without `!`" names the target state precisely.

---

**I3 — Settlement regression**

- **The load-bearing question:** "What changed in the graph since this last compiled
  cleanly?"
- **Question type:** Circular (the relationship between graph state and compilation)
- **What the inability to answer reveals:** The engineer doesn't have a model of the
  graph's evolution. The diff exists (git has it); the question surfaces whether the
  engineer has read it. From license-is-the-compiler.md: "You didn't change anything.
  The world changed around your type."
- **How @nl generates it:** From the git blame of the graph nodes that changed since
  the last clean compile. Optionally: list the changed node names to make the question
  answerable rather than rhetorical.

---

**I4 — Crystallization failure (hole stuck, not converging)**

- **The load-bearing question:** "What's the one thing this hole is waiting to know?"
- **Question type:** Future-oriented (orients toward the missing input, not the
  failure itself)
- **What the inability to answer reveals:** The engineer doesn't know what information
  would resolve the hole. From settlement.md: "The `\` is patient. It sits in the
  source. Open. Honest. 'This isn't settled yet.'" The question asks the engineer
  to name the blocking unknown — not fix it, but name it.
- **How @nl generates it:** From the hole's convergence trajectory and the graph nodes
  at the boundary of the hole's type context.

---

### Code Quality Dimensions

---

**D2 — Cognitive Complexity (CC above threshold)**

- **The load-bearing question:** "Where does this function's complexity go when you
  split it?"
- **Question type:** Future-oriented (the split is the path forward)
- **What the inability to answer reveals:** The complexity has no natural seam. The
  function's branches are load-bearing to each other — pulling one out changes the
  semantics of the others. This is the difference between accidental complexity (easy
  to split) and essential complexity (hard to split because the problem is hard). The
  question distinguishes them.
- **How @nl generates it:** Function name from the AST node with highest CC. "When you
  split it" presupposes that splitting is the right move and asks the engineer to locate
  the cut.

---

**D3 — Decidability Depth (low safe_operations ratio)**

- **The load-bearing question:** "Which part of this could a type checker verify
  automatically?"
- **Question type:** Future-oriented (orients toward what is knowable)
- **What the inability to answer reveals:** The engineer cannot separate the verifiable
  from the unverifiable — they have not thought about the structure in terms of what
  can be proven vs. what must be assumed.
- **How @nl generates it:** From the ratio of safe_operations to forced_inferences in
  the proof block. The function or module name from the AST.

---

**D5 — Staleness (doc-code distance above threshold)**

- **The load-bearing question:** "What does [function name] do that [doc reference]
  doesn't mention?"
- **Question type:** Circular (the relationship between the code and its description)
- **What the inability to answer reveals:** Either (a) the engineer knows exactly what
  the undocumented behavior is, which means the documentation is deliberately incomplete
  — or (b) the engineer doesn't know, which means the code has drifted past legibility.
  Both are diagnostic.
- **How @nl generates it:** Function name from the highest-staleness AST node. Doc
  reference from the associated `@lsp/diagnostics.staleness` stale_range. "Doesn't
  mention" asks about omission, not contradiction.

---

**D8 — Extraction Pressure (composite)**

- **The load-bearing question:** "Who reads this code most often, and what do they need
  to hold in memory?"
- **Question type:** Circular (the relationship between readers and cognitive load)
- **What the inability to answer reveals:** The engineer has not thought about the
  downstream reader. The question makes the invisible labor visible by naming the actor
  who bears it.
- **How @nl generates it:** From git blame (who touches this node, how often).

---

**D9 — Scope Discipline / OBC Violation**

- **The load-bearing question:** "What would break outside this module if this function
  ran twice?"
- **Question type:** Circular (the relationship between the function and external state)
- **What the inability to answer reveals:** The function has undeclared side effects.
  "Ran twice" is the idempotency probe.
- **How @nl generates it:** Module name and function name from the AST.

---

**D10 — Naming Quality (low coincidence alignment)**

- **The load-bearing question:** "What would you search for to find [identifier] in
  six months?"
- **Question type:** Future-oriented (the future reader is the test)
- **What the inability to answer reveals:** The name carries no semantic content beyond
  its own token. Names like `result`, `data`, `items` fail this test.
- **How @nl generates it:** Identifier name from the lowest-coincidence-alignment AST
  node. "Six months" is long enough to forget context, short enough to still care.

---

## 3. @nl Generation Spec

### Inputs Available at Error Time

```
node_name         -- from AST: the specific identifier, service name, function name
eigentest         -- which of the 8 Narcissus tests fired (E1-E8)
eigentest_value   -- the computed value
threshold         -- the threshold for this test
neighbor_types    -- types of connected nodes in the graph
dependent_nodes   -- nodes that depend on this node
git_history       -- who touches this node, how often, last N commits
committer_name    -- from git config at point of commit
offense_count     -- how many times this node has triggered this eigentest
previous_clean    -- last tick at which this node compiled cleanly
forced_inference  -- whether a \! is present in the context
proof_block       -- current proof block (safe_ops, unsafe_ops, forced_inferences)
```

### Generation Rules

**Always name the specific node.** "The auth service" not "this service."

**Always ask about the code relationship, not the developer.** The question is about
the graph, not the person who wrote the node.

**First offense: future-oriented question.** Gentler entry point.

**Repeated pattern (offense_count > 1): circular question.** More direct.

**`\!` override present: exception question.** Respects the consent. "When does
this compile cleanly?"

**Committer name:** Never used in the question itself. Used only for routing.

**Dependent nodes:** Cap at 3 specific node names. More than 3 becomes a list.

**Git history:** Used for D8 (who reads this) and I3 (when did it last compile
clearly). Never expressed as "you touched this N times" — expressed as code history.

---

## 4. What Does NOT Get a Question

**Security violations.** Too urgent. Requires a directive, not a question.

**Syntax errors.** Fix is unambiguous. The question would be condescending.

**Active `\!` override chains.** The override is consent. Respect it. Exception
form acceptable: "When does this compile cleanly?"

**Crystallization events.** Settlement is positive signal. No question. The
kintsugi commit message is the response.

**Internal proof block updates.** Eigenboard is for observation, not interruption.

**Informational diagnostics (severity: hint).** Adding a question to a hint makes
it a warning.

---

## 5. Full Error Message Examples

### Example 1: Auth Service Hub
**E1 (betweenness) + E7 (single-node fragility), first offense**

```
I can't let you do that, auth-service.

  betweenness centralization: 0.89   (threshold: 0.70)
  single-node fragility:      0.94   (threshold: 0.80)

  Every request in the authentication graph routes through auth-service.
  Betweenness 0.89 means it sits on 89% of shortest paths between other services.
  Fragility 0.94 means the graph has no meaningful alternative structure.

  To override: apply \! to the affected action body.
  Note: \! accepts inference loss. The Narcissus battery cannot be bypassed.

How can you route around auth-service?
```

---

### Example 2: Cross-Branch Merge Violation
**E2 (degree Gini) + E5 (clustering) + E6 (peripheral conductance), repeated pattern**

```
This merge introduces a topology that the graph has rejected before.

  degree Gini:             0.73   (threshold: 0.60) -- third violation in 14 days
  clustering coefficient:  0.08   (threshold: 0.25) -- payment-processor neighborhood
  peripheral conductance:  0.02   (threshold: 0.10) -- checkout boundary unreachable

  The payment-processor cluster carries 73% of the load Gini.
  Its neighbors (order-service, inventory-service, fraud-check) do not connect
  to each other except through payment-processor.
  Git record: this cluster triggered Gini > 0.60 on ticks 134, 148, and now 162.

What connects order-service, inventory-service, and fraud-check to each other,
without going through payment-processor?
```

---

### Example 3: `\!` Forced Inference — Diverging
**I2, repeated `\!` with ongoing loss regression**

```
This hole has been forced for 23 days without converging.

  forced since:     tick 124 (2026-04-15)
  current loss:     0.71 bits   (at force: 0.68 bits)
  trajectory:       diverging   (loss increasing, not decreasing)
  candidates tried: 1,247
  best candidate:   Result<PaymentToken, NetworkError>   (loss: 0.71 bits)

  The \! accepted the loss at tick 124. The loss has not decreased since.
  The tournament has tried 1,247 candidates. None have closed below L0.

What would need to be true for this to compile without `!`?
```

---

### Example 4: Cognitive Complexity Violation
**D2, first offense**

```
This function is doing more than it can explain.

  function:              parse_transaction_record
  cognitive complexity:  127   (threshold: 15)
  nesting depth max:     8
  control flow branches: 23

  Complexity 127 means a reviewer must hold 127 distinct state transitions
  in mind to understand this function's behavior.

  Suggested operation: \lsp/refactor.extract

Where does parse_transaction_record's complexity go when you split it?
```

---

### Example 5: Staleness Warning
**D5, warning severity**

```
The documentation has not kept up with the code.

  function:       validate_session_token
  doc distance:   0.61   (threshold: 0.30)
  last doc sync:  tick 89 (2026-04-02)
  current tick:   tick 203

  The NL token overlap between the doc comment and the implementation
  has fallen from 0.87 to 0.39 since tick 89.

  Gutter: orange (loss elevation in this region)

What does validate_session_token do that its doc comment doesn't mention?
```

---

### Example 6: Scope Discipline Violation
**D9, first offense**

```
This function is reaching past its declared scope.

  function:         apply_discount_code
  scope violations: 3
    reads:  session_state (undeclared Observable)
    writes: global_pricing_cache (no Budget constraint)
    throws: NetworkException (undeclared Cascade)

  Observable declares what the function touches.
  Budget declares its preconditions and postconditions.
  Cascade declares its failure modes.
  apply_discount_code declares none of these.

What would break outside the pricing module if apply_discount_code ran twice?
```

---

## 6. Opening Line Variants

| Error type | Opening line |
|---|---|
| Narcissus E1-E8 | "I can't let you do that, [node name]." |
| I1 hole too large | "The distance between [A] and [B] is larger than inference can bridge." |
| I2 forced, diverging | "This hole has been forced for [N] days without converging." |
| I3 settlement regression | "Something changed in the graph. This used to compile cleanly." |
| I4 crystallization stuck | "This hole knows its shape but not its content." |
| D2 cognitive complexity | "This function is doing more than it can explain." |
| D5 staleness | "The documentation has not kept up with the code." |
| D8 extraction pressure | "This code is passing the cost to whoever reads it next." |
| D9 scope violation | "This function is reaching past its declared scope." |

The HAL line ("I can't let you do that") is reserved for Narcissus violations only.
Narcissus is the one gate that cannot be overridden. The line's weight comes from
that finality. Using it for recoverable errors dilutes it.

---

## Appendix: Full Mapping Table

| Error | Opening line | Question type | Question stem |
|---|---|---|---|
| E1 Betweenness | HAL | Circular | "How can you route around [node]?" |
| E2 Degree Gini | HAL | Circular | "Which [type] is doing the most work? Could any of it move?" |
| E3 Spectral ratio | HAL | Future/Circular | "What happens to [dependents] when [hub] scales?" |
| E4 Von Neumann | HAL | Circular | "Where does [information type] go if [node] is unavailable?" |
| E5 Clustering | HAL | Circular | "What connects [node]'s neighbors to each other, without going through [node]?" |
| E6 Peripheral conductance | HAL | Circular | "How does a new [component type] reach [boundary] without touching [node]?" |
| E7 Single-node fragility | HAL | Circular | "What's the first thing that fails when [node] is unavailable?" |
| E8 Permeability | HAL | Circular | "What can you observe about [node]'s behavior from outside it?" |
| I1 Hole too large | Distance line | Future-oriented | "What's the smallest type change that would close this gap?" |
| I2 Forced, diverging | Days line | Exception | "What would need to be true for this to compile without `!`?" |
| I3 Settlement regression | Changed line | Circular | "What changed in the graph since this last compiled cleanly?" |
| I4 Crystallization stuck | Shape line | Future-oriented | "What's the one thing this hole is waiting to know?" |
| D2 Cognitive complexity | Explain line | Future-oriented | "Where does [function]'s complexity go when you split it?" |
| D3 Decidability | (warning only) | Future-oriented | "Which part of this could a type checker verify automatically?" |
| D5 Staleness | Doc line | Circular | "What does [function] do that [doc] doesn't mention?" |
| D8 Extraction | Cost line | Circular | "Who reads this code most often, and what do they need to hold in memory?" |
| D9 Scope | Scope line | Circular | "What would break outside [module] if [function] ran twice?" |
| D10 Naming | (warning only) | Future-oriented | "What would you search for to find [identifier] in six months?" |

---

*The question is the intervention.*
*The inability to answer is the diagnosis.*
*The topology is visible only when someone tries to traverse it.*
