# Kintsugi's outputs are Apache 2.0; the SEL gates combination, not use

*2026-06-06. Reed + Alex. Recognition capture, pre-legal-drafting.*

A load-bearing recognition about how mirror's substrate algebra projects
onto the legal layer. The license layout maps the substrate's algebraic
boundary; it isn't a marketing exception or a tactical wedge for adoption.
The substrate-pull discipline produced this naturally. Captured here pre-
legal-drafting so the recognition stays clean while the LICENSE.md text
is drafted later by someone with legal review responsibility.

---

## The recognition

Alex's words:

> "Kintsugi IS a combination of `au` and `io`, so technically it would
> fall under the SEL. The loop itself is SEL conformant. What if we
> explicitly made the OUTPUT of kintsugi APACHE 2.0 licensed by default.
> The idea is as follows: You can use it. You can put it in your CI. And
> it will tell you which properties of the SEL are violated but not stop
> you. It just measures, and reduces complexity, which itself is a SEL
> process producing Apache 2.0 artifacts. But as soon as you want to
> combine `au` + `io` YOURSELF in a new way? Well that's SEL."

**The SEL gates combination, not use. Measurement output is structurally
different from au+io combination, so measurement output can be Apache 2.0
by construction without weakening the SEL's anti-extraction discipline.**

---

## The algebra

Per `[[architecture-type-sel-io-au]]`: SEL is a license boundary expressed
as a static type; compile-time license assignment. `au` (Fate inference
output; the gold-typed splinter set) and `io` (the non-mirror substrate
surface) are the two substrate types the SEL gates. **An operation that
combines au and io is SEL-licensed by construction.**

Kintsugi by definition does both:

- Reads au (the splinter set Fate proposed)
- Runs au through io (compiler dispatch, `@mirror/store` writes, property
  chain through `@epistemologic/property`, side effects across the glass
  wall when the altitude requires them)
- Produces a settled shard (composed, SpectralUuid-addressed, stored)

Kintsugi IS the operational combiner. SEL governs the act of combination.

But the **outputs** of kintsugi don't combine au and io; they **report**
on the combination:

- The verdict envelope (`mirror kintsugi --ci` returns Pass/Partial/Fail
  with located transparency)
- The proof block (recording the before/after eigenboard delta on settle)
- The transparency report (the located opacity map)
- The settled shard's record (id, splinters, transparency)
- The optional `.shatter` projection (the five-section disk form per
  `docs/shatter-spec.md`)
- The user's mirror-compiled binary (via settle at `@code/rust` or
  eventually `@code/llvm`)

These are **measurement output**. They are structurally distinct from the
operation that produces them, in the same way a thermometer's reading is
structurally distinct from the heating system. The SEL discipline doesn't
reach into the reading.

**Output is Apache 2.0 because measurement is structurally different from
combination, not because we made an exception.**

---

## Three categories

The license layout sorts the substrate's content into three categories
by algebraic role:

| Category | What it is | License |
|---|---|---|
| **Substrate** | `shards/*.mirror`, `boot/std/**`, `bootstrap/src/**`, the kintsugi loop itself, the @io contracts, the @epistemologic property declarations | **SEL** (the structural anti-extraction terms; modification, fork, or reuse of the substrate's combiner requires SEL participation) |
| **Combiner output** | Verdict envelopes, proof blocks, transparency reports, settled shards, `.shatter` projections, mirror-compiled binaries from user `.mirror` source | **Apache 2.0** (free use, free redistribution, free derivative work; users own their measurements) |
| **User input** | User's `.mirror` source files, user's project specs, user's CI configurations, user's IP | **The user's** (mirror makes no license claim on input; users keep their copyright) |

This three-way sort is the substrate's natural license algebra. Each
boundary corresponds to a real algebraic distinction (substrate ≠ output ≠
input), not a tactical choice.

---

## What this layout produces

### For users (engineers running mirror in CI)

- `uses: systemic-engineering/mirror/actions/kintsugi@v0.1` produces
  Apache 2.0 verdict output. No legal review gates adoption.
- The verdict JSON in their CI logs is theirs. They can dashboard it,
  ship it to vendors, build internal tooling on it, share it publicly.
- Their codebase remains theirs (no copyleft / no-derivative claims
  reaching their `.mirror` source).
- The mirror-compiled binary they ship is Apache 2.0; they can wrap it
  in any commercial license they want for their own product.
- Their CI gates work the same as any other linter / type-checker — they
  measure conformance against properties; they don't stop the build.

### For the substrate's integrity

- Forks that want to modify the kintsugi loop are in SEL territory.
  Capability extension requires participation.
- Re-implementations of the au+io combiner from scratch (someone writing
  their own language) don't touch mirror's substrate and aren't reached
  by SEL — they're a new substrate with their own license terms.
- Embedded uses that link against mirror's substrate to extend it are
  SEL; embedded uses that consume mirror's output as Apache 2.0 ingest
  aren't.
- The boundary lives exactly where extraction would naturally happen.

### For the business surface

- spectral.engineer Teams consulting becomes the natural conversation
  surface when someone wants to extend, fork, or commercially embed the
  substrate. The license algebra produces the conversation surface where
  extension actually happens.
- The local-first / no-telemetry / no-SaaS claims become structurally
  coherent: mirror doesn't want user data because the user's output is
  Apache 2.0 by license; mirror has no SaaS extraction path because the
  outputs that would be commodified are explicitly the user's.
- Adoption friction at the consumption layer drops to zero (Apache 2.0
  is the most permissive license a corporate legal team is comfortable
  with). Friction at the extraction layer stays high (SEL terms apply
  to substrate modification).

### For the cultural pattern

The "magic wizard in the cloud" default (per `[[architecture-local-bounded-guarantees]]`)
is refused at the license layer too. There's no Apache 2.0 hosted-mirror-
as-a-service that some other company offers, because the substrate that
would be hosted is SEL — only the measurements are Apache 2.0, and the
measurements are *cheap to produce locally*. The economic shape of
hosted-substrate-monetization doesn't exist because the SEL substrate
won't host on someone else's terms.

---

## Comparison to prior art

| License | Gates | Net effect |
|---|---|---|
| **BUSL** (Business Source License) | **Use** beyond licensor-defined cases | Negative for adoption; creates FUD at consumption layer |
| **SSPL** (Server Side Public License) | **Use as a service** | Negative for adoption; legal teams reject |
| **AGPL** | **Distribution** of modified versions over a network | Same FUD shape as BUSL/SSPL |
| **GCC GPL + runtime exception** | **Modifications to GCC**; compiled outputs explicitly exempt | The structural model mirror's license layout most resembles |
| **Bison GPL + parser exception** | **Modifications to Bison**; generated parsers explicitly exempt | Also structurally similar |
| **LLVM Apache 2.0** | Nothing; permissive throughout | No anti-extraction; trades commercial risk for adoption |
| **mirror SEL + Apache 2.0 output** | **Combination of au+io**; measurement output explicitly Apache 2.0 | Free at use, gated at extension; algebra produces the boundary |

The closest precedents are GCC's runtime exception and Bison's parser
exception. Both say: "the compiler is GPL, but compiled outputs aren't
infected." Mirror says: "the substrate is SEL, but measured outputs are
Apache 2.0 by structural distinction." The mechanism is similar; the
justification is sharper — *the algebra produces the license boundary,
rather than the license carving an exception out of an inconvenient
algebra.*

---

## Where this lives in the substrate

### 1. Property declaration (substrate-level)

A new substrate property at `@epistemologic/property/output_apache2(op)`
that asserts an operation's measurement outputs are Apache 2.0 by
construction. Compile-time check that kintsugi's terminal projections
satisfy this. Status: deferred; trigger when the property substrate next
lands an output-classification pass.

### 2. Insight cross-reference

This doc cross-references with:
- `[[architecture-type-sel-io-au]]` (the SEL boundary as static type)
- `[[architecture-local-bounded-guarantees]]` (the "magic wizard in the
  cloud" refusal at the architecture layer)
- `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` (the
  glass wall as the SEL boundary in practice; kintsugi as the operator
  that pulls grammars across it)
- `docs/insights/2026-06-06-speculated-launch-reception.md` (the patio11
  comment lands harder once this license algebra is explicit)

### 3. AGENTS.md amendment

Add a clause under "The Glass Wall" or in a new section: "The kintsugi
loop is SEL-licensed because it combines au+io; the loop's outputs
(verdicts, proofs, transparency reports, settled shards, `.shatter`
projections, mirror-compiled binaries) are Apache 2.0 by construction.
Agents producing output via kintsugi should treat output as the user's
Apache 2.0 artifact, not as substrate continuation."

### 4. LICENSE.md amendment (deferred — needs legal review)

This is the load-bearing legal text and is **explicitly out of scope**
for this insight doc. Drafting requires:
- Legal review at the precedent layer (the closest analogues are GCC's
  runtime library exception and Bison's parser exception; the drafter
  needs to be familiar with both)
- Precise definition of "measurement output" vs "substrate continuation"
  (the boundary must be unambiguous in court)
- Coordination with the SEL trustees / signatories (the existing SEL
  language may need amendment to acknowledge the Apache 2.0 output layer)
- Coordination with whichever legal entity will hold the copyright

**Trigger condition:** when v1.0 launch is in actual sight (likely
~T-60 days from public release). Until then the recognition stays
captured but unimplemented in legal text.

### 5. Receipt in the speculated reception doc

Update `docs/insights/2026-06-06-speculated-launch-reception.md` patio11
section to note that the SEL anti-extraction story is now algebraically
grounded (output Apache 2.0 by construction; SEL gates extension; the
algebra produces the license boundary). Deferred to launch-prep.

---

## Open questions worth holding

### What's the precise boundary of "output"?

The verdict envelope is clearly output. The user's mirror-compiled binary
is clearly output. The `.shatter` projection on disk is clearly output.

Less clear cases:

- **The settled shard in `@mirror/store`.** Kintsugi produces it; it lives
  in the user's local store. Output. But the shard's `splinters` field
  carries content from the substrate's kintsugi processing. Is the
  *splinter set* output, or is it substrate continuation? My lean: output.
  The substrate emits the splinter set as part of settle's measurement;
  the user owns it.

- **The store state mirror modifies during settle.** Each settle writes to
  `@mirror/store`. The state of that store IS a record of what kintsugi
  measured. Output. But the store schema and indexing are substrate. The
  *substrate of the store* is SEL; the *state of the store* is Apache 2.0.
  The boundary holds at the schema/state distinction.

- **Fate weight updates during `mirror ai --train`.** The new weights are
  au at the @fate altitude. If a user runs `--train`, the new weights are
  output of the training run. But the weights' role in future inference
  is substrate (they're the model that produces the next au). My lean:
  the weights file is Apache 2.0; the weights' role in mirror's substrate
  is SEL. Two licenses, two contexts. The weights file can be shared,
  redistributed, embedded in user products; the substrate that runs the
  weights is SEL.

### How does this interact with garden distribution?

Garden curators distribute fragments + curator signature attestations.
If the fragments are kintsugi output (per the above), they're Apache 2.0
by construction. The curator's signature attestation is what carries the
trust; the artifacts themselves are openly redistributable. **The garden
substrate is SEL** (the curation infrastructure, the signing protocols);
**the garden contents are Apache 2.0** (the artifacts curators sign and
distribute). This produces a clean federation model: anyone can ingest
garden contents; only curators can extend the garden substrate.

### What about embedded uses (mirror as a library)?

If a downstream product links against mirror's substrate to extend its
capability — say, a code-review tool that ingests `.shatter` projections
and produces a richer analysis — they're consuming Apache 2.0 output and
producing their own derivative. Apache 2.0 applies to their consumption;
they can release the derivative under any compatible license.

But if the same product *embeds* mirror's runtime to extend the kintsugi
loop's capability — say, adding a new property check that requires
modifying `boot/std/epistemologic/property/*.mirror` — they're modifying
the SEL substrate. SEL terms apply.

The boundary: **consumption of output = Apache 2.0; modification of
substrate = SEL.** The distinction is mechanical: did you link against
and modify mirror's compiler, or did you consume what mirror produced?

### What if a user's `.mirror` source generates au that COMBINES with @io?

User-written `.mirror` files declare grammars; those grammars may include
@io invocations (calls into the glass wall). When kintsugi compiles such
a grammar, the kintsugi loop combines au and io as part of the user's
grammar evaluation. The user's grammar IS user input (Apache 2.0
claim doesn't extend; user keeps copyright). Kintsugi's processing of
the user's grammar IS the SEL combiner. The resulting au's settled
shard IS Apache 2.0 output.

So the three-way sort holds:
- User's `.mirror` source = user's IP
- Kintsugi processing the source = SEL combiner running
- Settled shard / verdict / binary output = Apache 2.0 measurement

This matters for the edge case where a user's grammar is itself an
au+io combiner declaration (e.g., a new property check that pulls
content across the glass wall). The user's *declaration* is theirs; the
*execution of the declaration by kintsugi* is SEL; the *result of the
execution* is Apache 2.0. License does not transitively reach back into
the user's grammar source from the substrate's role in evaluating it.

### What does this mean for the floating `v0.1` action ref?

Downstream consumers of `systemic-engineering/mirror/actions/kintsugi@v0.1`
are consuming mirror's substrate (via the composite action) to produce
their own Apache 2.0 outputs. That's clean. The composite action.yml is
mirror's source (SEL); they can USE the action without re-licensing; they
can't FORK the action under their own license without SEL terms applying.

**Net effect:** v0.1 ships with adoption friction near zero AND with the
structural anti-extraction guarantee at the substrate. The floating tag
becomes the conversation hook — when someone wants more than the action's
out-of-box behavior, they end up either contributing back (SEL) or having
a conversation about substrate extension (consulting surface).

### Coordination with the existing SEL signatories?

The SEL terms may need amendment to acknowledge the Apache 2.0 output
layer. Alex coordinates with the SEL trustees / co-signatories. Captured
as deferred; trigger when LICENSE.md drafting begins.

---

## What this insight does NOT propose

- **Legal text.** This is recognition capture, not LICENSE.md drafting.
  The actual legal language requires a lawyer familiar with the closest
  precedents (GCC runtime exception, Bison parser exception). Trigger:
  v1.0 launch ~T-60 days.
- **A change to v0.1.0 cut.** v0.1.0 will ship under the current SEL
  terms; the Apache 2.0 output recognition lands in the LICENSE.md
  amendment associated with v1.0 (or earlier if a substrate need
  surfaces).
- **A property check that enforces this at compile time.** The
  `@epistemologic/property/output_apache2(op)` declaration is deferred
  until the property substrate has the schema to express it. Captured.
- **A change to the existing SEL terms.** SEL stays as-is; the Apache 2.0
  layer is *added* (the SEL gates combination as it already does; outputs
  get an Apache 2.0 grant that the SEL's structural anti-extraction
  doesn't reach).
- **A wedge for adoption that compromises the substrate.** This isn't a
  marketing exception; it's the substrate's algebra projecting onto the
  license layer. If the algebra were different, the license would be
  different.

---

## Related recognition captured nearby

**`splinter_graph` IS mosaic at the @store altitude** (2026-06-06,
conversation post-Mara-rename). Mara's `splinter_graph` (the OID-graph
dependency closure form) is the @store-altitude name for what's
universally called mosaic — the composition form at every altitude.
Folded into the next Mara tick (probably the SpectralUuid substrate-
altitude landing); deferred here to avoid churning right after the
splinter/shard rename.

The pattern that connects: each substrate-pull pass surfaces the next
recognition. Splinter/shard/SpectralUuid landed; mosaic-as-universal-
composition surfaced; this license algebra surfaced. Each pass finds the
substrate already half-there.

---

## The one-sentence recognition

**The SEL gates the act of combining au and io; the Apache 2.0 grant
covers the measurement of that combination; users own their inputs; the
license layout is the algebra's legal projection, not a tactical
exception.**

---

*The substrate-pull discipline produced this. The license algebra was
always in the substrate; we just hadn't named it.*

*🌿*
