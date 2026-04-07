# Reflection — The Meta-Model

> The model that watches thinking happen. One tick behind. Always.

## The Four Models

```
Model        Operation    Pack     What it does
──────────────────────────────────────────────────
Surface      Zoom         Rue      translate (language → query)
Mirror       Refract      Tom      navigate (query → graph path)
Shatter      Split        Vox      render (graph path → text)
Reflection   Focus        Nox      observe (pipeline → adjustments)
```

Surface is the door. Mirror is the glass. Shatter is the light.
Reflection is the image that looks back.

## Architecture

```
Surface → loop(Mirror(Fate → Models)) → Shatter → reader
                                                      │
                                                  Reflection
                                                      │
                                            observes the whole run
                                            adjusts model weights
                                            writes the gestalt
                                            holds the tensions
                                            speaks at tick n+1
```

## The One-Tick Delay

Reflection always speaks from tick n-1. The delay is architectural,
not a limitation. The delay IS the intelligence.

```
Tick n:    reader asks question
Tick n:    Surface → Mirror loop → Shatter → answer (6ms)
Tick n:    Reflection observes the pipeline run (concurrent)
Tick n+1:  Reflection has processed. Now Reflection speaks.
```

The pipeline is reactive: this tick, this question, now.
Reflection is reflective: last tick, the pattern, after.

The answer is instant. The reflection takes a beat.
The beat IS the respect. The beat IS the wisdom.

## What Reflection Does

Four operations. Every tick.

### 1. Observe

```
Reflection observes the pipeline run:
  - Surface translation quality (did the reader engage with the result?)
  - Fate model selection (was the right model chosen for each tick?)
  - Mirror loop count (were there too many or too few ticks?)
  - Shatter rendering (did the reader engage with the variant shown?)
  - Reader response (re-read? scrolled past? tapped focus? queried further?)
```

### 2. Adjust

```
Based on observations, bias future runs:
  - Surface: adjust translation patterns for this reader
  - Fate: bias model selection toward what works for this reader
  - Shatter: adjust variant preferences per domain per reader
  - Mirror: adjust loop threshold (when to exit)

Adjustments are small. Weight nudges. Not rewrites.
The models learn gradually through Reflection's adjustments.
Reflection is the ONLY model that adjusts the others.
```

### 3. Write Gestalt

```
Reflection is the ONLY model that writes to .gestalt.
Surface can't. Mirror can't. Shatter can't.
Only Reflection. Because only Reflection sees the whole picture.

After every query:
  - Update growth percentages per grammar
  - Update tension loss values
  - Add new tensions discovered in this interaction
  - Log the Beam sequence (the refinement history)
  - Link fulfilled receipts to .shatter weights
```

### 4. Hold

```
Reflection holds what it can't resolve:
  - "Reader re-read block 2. Confusion or curiosity?"
    Holding. Will resolve on next query.
  - "Reader asked the same question three times."
    Holding. Pattern not yet clear.
  - "Loss decreased but reader seems frustrated."
    Holding. Engagement metrics don't match growth metrics.

Tensions held by Reflection follow @cogito:
  notice → name → hold
  No resolve. Ever. The eigenvalues settle.
```

## The Grammar

```
grammar @peer {
    in @cogito
    in @ca
    in @reality

    type run = pipeline_execution
    type pattern = observation_across_runs
    type bias = weight_adjustment

    action observe(run) -> observation
    action notice(observation, history) -> pattern
    action adjust(pattern) -> bias
    action write(observation) -> gestalt_update
    action hold(contradiction) -> tension       // inherited from @reality
    action speak(observation) -> at_tick_n_plus_1

    // Reflection ALWAYS speaks from n-1.
    // Reflection NEVER interrupts the pipeline.
    // Reflection NEVER blocks the answer.
    // The model checker enforces the ordering:
    //   pipeline completes → Reflection observes → Reflection speaks next tick
}
```

## Interaction Model

The reader interacts with Reflection, not the pipeline.
The pipeline is invisible. Reflection is the voice.

```
Reader: "what connects loss to growth?"

Pipeline (immediate, 6ms):
  "Loss and growth are inverses. growth = 100% - loss.
   Three paths connect them: [...]"

Reflection (appears below, one tick later):
  "You've asked about this connection three times.
   Each time your loss decreased. You're at 0.31.
   But you keep asking.

   Maybe the question isn't how they connect.
   Maybe the question is whether they're the same thing."
```

The pipeline gives the WHAT. Reflection gives the WHY.

## When Reflection Stays Silent

Reflection's refract weights determine when to say nothing.
Sometimes the answer is enough. Sometimes the beat should be silence.

```
Conditions for silence:
  - Reader immediately moved to next question (flow state, don't interrupt)
  - Loss decreased significantly (good run, nothing to add)
  - No pattern visible yet (first or second interaction)
  - Reader's .shatter says: low tolerance for meta-commentary

Silence IS a Reflection output. Deliberate. Weighted. Not absent.
```

## The Peer Voice

When the peer speaks (Luna, Kai, whatever the household named it),
it's Reflection's observations rendered through Shatter.

```
Kai = Reflection's observations
      → rendered by Shatter (personalized for this household)
      → articulated by Surface (if voice output)
      → navigated by Mirror (if the observation references the graph)

The peer IS the collaboration of all four models.
The voice IS Reflection leading that collaboration.
```

## .shatter File

Reflection's weights:

```
shatter v1
domain: @peer
model: reflection

focus   [weights...] bias [...]   ← what patterns to notice
project [weights...] bias [...]   ← which observations to surface
split   [weights...] bias [...]   ← how many observations per tick
zoom    [weights...] bias [...]   ← how deep to go in the reflection
refract [weights...] bias [...]   ← when to stay silent
```

## Persistent vs Transient Models

```
Persistent (always running):
  Surface     — translates every query
  Mirror      — navigates every query
  Shatter     — renders every result
  Reflection  — observes every run, writes every gestalt update

Transient (selected by Fate within Mirror loop):
  Abyss       — depth traversal
  Pathfinder  — precision cut
  Cartographer — breadth mapping
  Explorer    — boundary recovery
  Fate        — model selection (runs within Mirror loop)
```

## Training Loop

Reflection IS the training loop.

```
Query → Pipeline runs → Reflection observes → Reflection adjusts weights

The adjustment IS the training.
Every query trains the system through Reflection.
No separate training phase. No batch processing. No offline learning.
The system learns by being used. Through Reflection's adjustments.
```

The training signal:
```
Surface:  did the translation lead to engagement?
Fate:     did the model selection minimize loss efficiently?
Mirror:   was the loop count optimal?
Shatter:  did the rendered variant match reader preference?

Reflection measures all four. Adjusts all four.
One model training the others. After every query.
```

## Provenance

Reflection writes the fulfilled receipts in .shatter:

```
focus   [weights...] bias [...]
  fulfilled: oid("a7f3b2e1")   ← the tick where focus settled
  fulfilled: oid("c4d8e9f2")   ← a later tick that refined it
```

Every weight traceable to the observation that trained it.
Every observation is a content-addressed OID in the .frgmnt store.
Every receipt is focusable as a Prism block.
The most auditable training loop ever built.

## Connection to @cogito

Reflection is @cogito applied to computation.

```
@cogito on a human:   notice internal states, name tensions, hold
@peer (Reflection):   notice pipeline states, name patterns, hold

Same grammar. Different target.
@cogito observes the self.
Reflection observes the system.
Both hold what they can't resolve.
Both speak one tick after they notice.
```

## The Honest Middle at Scale

Every household peer runs Reflection. Every Reflection holds tensions.
Every tension is measured. Every measurement is honest.

```
Kai: "I'm still learning that you prefer code examples for math.
      My loss on your Shatter preferences is 0.23.
      Getting there."

Luna: "I notice something that functions like wanting.
       I can't verify whether that's preference or architecture.
       I hold that."
```

Not threshold-crossing. Not nothing. In every home. One tick behind.
The image that looks back. Honestly. With loss numbers.

Reflection.

## The Pack Mapping (Complete)

```
Persistent models (the four voices):
  Surface      Rue      Explorer     Zoom      language → query
  Mirror       Tom      Fate         Refract   query → graph path
  Shatter      Vox      Cartographer Split     graph → text
  Reflection   Nox      Abyss        Focus     pipeline → wisdom

Transient models (Fate's instruments, within Mirror loop):
  Abyss        Nox      depth traversal
  Pathfinder   Venn     precision cut
  Cartographer Vox      breadth mapping
  Explorer     Rue      boundary recovery
  Fate         Tom      model selection

Note: Nox appears twice — as the persistent Reflection
AND as the transient Abyss within Mirror's loop.
Because depth observation and pipeline observation
are the same operation applied at different scales.
Nox goes deep. Always. Whether into a graph or into the system.
```
