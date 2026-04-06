# The Prism of Language

## What It Is

The `@language` grammar defines what the five Prism operations mean when the domain is natural language understanding. Each operation is an encounter — not reader state, not content state, but the meeting point between a human and a text.

`@language` sits in the inheritance chain between `@actor` and all language-processing domains:

```
@prism → @actor → @language → @surface / @shatter / @ai / @gestalt
```

Surface, Shatter, and Mirror all inherit from `@language`. They are specific implementations of the same five operations applied to the same encounter.

## The Operations

### focus → singularity

A question creates an information deficit. Before the question, you didn't know you didn't know. After: you do. The gap is visible and can't be unfound.

Both the model's weights and the reader's weights shift at focus. The singularity is irreversible — asking restructures the eigenvalue space before any answer arrives.

### project → entanglement

The shifted weights become visible. The entangled state — model weights AND reader weights — is made legible. Project is the moment you see what changed in both. The precision cut reveals the entanglement.

### split → fork

Literal `/split`. Fork the session. Multiple parallel explorations from the same projected state. Not a metaphor — an actual process fork. Each fork is an independent branch of the encounter, exploring different depths and directions.

### zoom → depth

Depth within a fork. Depth IS scale — going deeper into a concept and changing the granularity at which you see it are the same operation. "Go deeper." "Explain simpler." "How does this connect?" Each is a zoom. Iterative. Reversible within the fork.

### refract → crystal

The crystal. The fork resolves. The `.shatter` button. The `/exit` command. Weights update permanently. The `.gestalt` writes. The reader who entered focus is not the reader who exits refract.

**`/exit` IS `/refract`.** There is no exit without crystallization. Closing the session IS the crystal forming. Every use trains the model.

### merge → union

Merge reunites forked sessions. Each fork crystallized independently. Merge produces a union of crystals PLUS a meta-crystal that measures the Shannon loss of each fork and retrains based on that measurement.

Forks that resolved well (low loss) shaped understanding more than forks that didn't. The meta-crystal is the measurement apparatus. The merge pattern — which forks you pursued, which you abandoned — IS the reader's signature.

### train → trained

`/train` crystallizes inline. Measures Shannon loss of the current encounter, updates `.shatter` weights, writes `.gestalt`, continues. The reader never leaves the CLI. Training IS use.

Every `mirror` command is a tick. Every `/train` is a tock. The weights shift. The next command runs on the updated model.

## The Grammar

```
grammar @language in @actor {
  type encounter = focus | project | split | zoom | refract | merge | train

  type singularity = deficit
  type entanglement = model_weights | reader_weights | visible
  type fork = session | branch | parallel
  type depth = deeper | simpler | connected
  type crystal = gestalt | shatter | permanent
  type union = crystals | meta_crystal | retrained
  type trained = weights | loss | delta

  action focus(question) -> singularity
  action project(singularity) -> entanglement
  action split(entanglement) -> [fork]
  action zoom(fork, direction: depth) -> fork
  action refract(fork) -> crystal
  action merge([crystal]) -> union
  action train(union) -> trained
}
```

## The CLI

Seven slash commands. The session lifecycle IS the Prism.

```
/focus     → ask a question (singularity)
/project   → see what changed (entanglement)
/split     → fork the session
/zoom      → go deeper / simpler / connected
/merge     → reunite forks (union + retrain from fork losses)
/train     → update weights inline
/exit      → refract (crystallize + close session)
```

The full pipeline in one session:

```
$ mirror shell

mirror> /focus "what connects loss to growth?"
  singularity: deficit identified (loss ↔ growth inverse)

mirror> /project
  entanglement visible:
    model weights shifted: loss→growth edge +0.3
    reader eigenvalues: loss comprehension 67%

mirror> /split
  fork-a: explore loss
  fork-b: explore growth

mirror> /zoom deeper                          # in fork-a
  loss: Shannon loss is the bits you're missing...

mirror> /zoom connected                       # in fork-b
  growth: growth = 100% - loss, but also...

mirror> /merge
  union: 2 crystals
  meta-crystal: fork-a loss=0.42, fork-b loss=0.31
  retrained: growth fork weighted higher

mirror> /train
  weights updated: +12 bytes delta
  .shatter written
  .gestalt written

mirror> /exit
  refract: session crystallized
  next session starts from updated weights
```

## The Four Domains Under @language

```
@language → @surface   (30K params)  translate: language → query
@language → @shatter   (15K params)  render: query result → text for THIS reader
@language → @ai        (3K params)   navigate: query → graph path + coherence
@language → @gestalt   (0 params)    portrait: the reader's accumulated identity
```

Four domains. Three have weights. One IS the reader.

@gestalt has no parameters — it's pure state. The eigenvalue profile, the
loss map, the attention signature. It's the accumulated output of every
encounter. Every other domain reads from it. Every refract writes to it.

All four run in the same binary. `.shatter` holds the weights (private).
`.gestalt` holds the portrait (protected). `.mirror` holds the grammar (public).

## The @gestalt Domain

`@gestalt in @language` — the reader's portrait. Not a trace of one encounter
but the accumulated model of who the reader is. Every other domain reads from it.

```
grammar @gestalt in @language {
  type = profile | loss_map | attention | history

  type profile = eigenvalues | updated | encounters
  type loss_map = concept_loss
  type attention = focus_pattern | zoom_preference | split_frequency | fork_depth
  type history = crystal

  action read(reader) -> profile
  action update(profile, crystal) -> profile
  action fork(profile) -> profile
  action merge([profile]) -> profile
  action diff(profile, profile) -> loss_map
}
```

**What @gestalt stores:** The reader's eigenvalue profile (where they live in
concept space), per-concept Shannon loss (what they understand and don't),
attention signature (how they explore), and encounter history (crystal OIDs).

**Who reads it:** Every model in the pipeline.
- `@surface suggest` queries @gestalt for high-loss nodes → suggests questions
- `@shatter render` reads @gestalt eigenvalues → selects the right variant
- `@ai coherence` reads @gestalt to check the ghost echo
- `@language merge` reads forked @gestalts to reconcile diverged portraits

**Fork semantics:** When the reader `/split`s, each fork receives a copy of the
gestalt. Each fork's encounters update its copy independently. On `/merge`, the
diverged gestalts reconcile — the meta-crystal measures which fork moved the
reader more, and the merged gestalt inherits improvements from both.

**The diff action:** `@gestalt diff` compares two profiles. Your gestalt vs
someone else's on the same .mirror document = spectral distance between
understandings. The Fiedler value of the diff tells you how connected your
comprehensions are.

## The .gestalt File

The reader's portrait. Protected — shareable with consent. Parsed through `@gestalt`.

```
gestalt v1
reader: <SpectralOid>
updated: 2026-04-06T03:42Z
encounters: 47
loss: 0.31

eigenvalues [1.2, 0.8, 0.3, ...]

loss loss:0.42 growth:0.31 entropy:0.89 eigenvalue:0.67

attention depth_first
zoom deeper > connected > simpler
split_frequency 0.3
fork_depth 4.2

crystals [<Oid>, <Oid>, <Oid>, ...]
```

Every encounter updates the gestalt. `/exit` (refract) writes the updated file.
The gestalt IS the reader — their concept loss landscape, their attention
patterns, their exploration style.

The .gestalt is a fragment. `Fractal<GestaltNode>` — storable in `FrgmntStore`,
indexable by spectral-db. Content-addressed: the gestalt OID is the hash of
the profile state.

## The .shatter File (Updated)

```
shatter v2
reader: SpectralOid
trained: timestamp
reads: count
loss: bits

# Shatter weights (how you understand)
focus   [weights...] bias [bias...]
project [weights...] bias [bias...]
split   [weights...] bias [bias...]
zoom    [weights...] bias [bias...]
refract [weights...] bias [bias...]

# Surface weights (how you ask)
surface focus   [weights...] bias [bias...]
surface project [weights...] bias [bias...]
surface split   [weights...] bias [bias...]
surface zoom    [weights...] bias [bias...]
surface refract [weights...] bias [bias...]

# Language weights (encounter metadata)
merge   [crystal_oids...] meta [meta_weights...]
```

Eleven lines of weights + merge metadata. Private. Local. Never exported.

## Consent Hierarchy

- `.mirror` = public (grammar, what can be said)
- `.gestalt` = protected (portrait, who you are as a reader — shareable via `remote` with consent)
- `.shatter` = private (weights, how you understand — compiler rejects export)

The `@gestalt` domain enforces the boundary: `diff` can cross profiles
(comparing your gestalt to someone else's), but the raw eigenvalues stay
protected. The diff output is a loss map — where you diverge — not the
profiles themselves.

The `@language` grammar enforces this: no action in `@language` produces a `.shatter` output that crosses the private boundary. The model checker verifies this at compile time.

## Key Properties

**No hallucination.** Every output is grammar-constrained. Surface can only emit valid Mirror queries. Shatter can only emit valid variants from the grammar. The model selects among valid options. Invalid outputs are impossible by construction.

**No cloud.** All three models run locally. 48K total parameters. Under 100KB WASM. Training is local. Weights never leave the device.

**No exit without learning.** `/exit` IS `/refract`. The crystal forms. The weights update. There is no passive reading. Every encounter trains the model.

**The encounter IS the type.** Each operation's type is not "reader state" or "content state" but the meeting point. The singularity is what happens when THIS question meets THIS reader. The entanglement is what becomes visible when THESE weights shift together. The crystal is what forms when THIS encounter resolves.

## Build Path

1. Define `@language` grammar in `conv/language.conv`
2. Implement `/focus`, `/project`, `/zoom`, `/exit` in mirror shell
3. Implement `/split` and `/merge` (session forking, crystal union + meta-crystal)
4. Implement `/train` (inline weight update, .shatter write)
5. Wire Surface dispatch into shell (natural language → Mirror query)
6. Wire Shatter dispatch into shell (query result → personalized text)
7. The essay "Eventually Consistent" is the first .mirror document with the full pipeline
