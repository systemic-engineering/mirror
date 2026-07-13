# `.song` files ARE `.mirror` files — song grammar is substrate-declared via `shards/song/keywords.mirror`, not a `song.rs`-internal DSL

*Mara, 2026-07-13 arc-continuation Rung 3 spec. Adjudicates the
substrate direction for `@song` grammar per Alex's ladder-climb
mandate. Path B verdict with substrate-honest formal grammar
production + two-tick landing sequence. Reed executes Tick 3b
immediately after.*

**Author:** Mara
**Date:** 2026-07-13
**Tag:** 📝 substrate-pull:realize; ladder-rung-3-spec
**Status:** adjudication + canonical grammar production. Every
substrate carrier cited is LANDED with OID or forward-promised at a
named site. This spec RESOLVES the Rung 3 substrate-direction
ambiguity Reed surfaced after `70766c3`; it does NOT commit new
`.mirror` grammar production this tick (two-tick discipline; Reed's
Tick 3b lands the companion-keyword file + the parser lift).

**Co-authored-by ancestry:** Alex Wolf (in-transcript 2026-07-13
ladder-climb mandate: *"climb the ladder until unresolvable ambiguity
that cannot be postponed further"*); Reed (`5fdc009` RED → `c36fbf5`
GREEN Rung 1; `79eee6f` RED → `70766c3` GREEN Rung 2; substrate-
direction ambiguity report post-`70766c3`); Taut (`c54740c` §5.4 Rung
3 spec); Mara (`d21337b` §5.1 canonical `@song @spectral/garden/
deployment` composition; `94e55eb` `shards/song/beat.mirror` sixth
species mint; this spec).

---

## §0. Executive summary

**Verdict: Path B.** Song files ARE mirror-native files. The
substrate-honest grammar landing for Rung 3 is a **companion-keyword
declaration file at `shards/song/keywords.mirror`**, exactly
analogous to `shards/mirror/spec/keywords.mirror` (which declares
`project`/`target`/`cli`/`settle_on` etc as focus/settle/project
mappings for the tokenizer). The keywords `song`/`movement`/`voice`/
`progression`/`narrative`/`phrase` (and `beat` at leaf) become
substrate-declared tokenizer keywords via `<op> <keyword>` two-word
lines, and Reed's parser lift in `bootstrap/src/song.rs` walks the
resulting typed AST instead of hand-tokenizing lines.

**Why not Path A (song.rs internal DSL).** Path A creates a
parser-divergence fault-plane: mirror-native declarations get one
grammar; song-file declarations get another. This violates
`[[architecture-shards-as-substrate-source]]` — keywords are
SUBSTRATE DECLARATIONS, not runtime constants. The whole point of
the grammar-harvester pipeline is that any consumer that declares
`<op> <keyword>` in a `.mirror` grammar block extends the tokenizer's
vocabulary without touching Rust. Reed's current `song.rs` line-parse
is a legitimate Rung-1/2 shortcut (single-beat and multi-beat both
degenerate below grammar-altitude), but at Rung 3, where the song
file's structure IS `song X { movement Y { voice Z { ... } } }`
nested blocks, the shortcut becomes a divergence.

**Why not Path C (no new keywords; reuse existing carriers).** Path C
was audited comprehensively (§1 below); it fails. Every existing
top-level keyword (`project`, `target`, `stage`, `bench`, `command`,
`pack`, `garden`, `grammar`, `glass`, `prism`) has a distinct semantics
that isn't `@song`-shaped. Overloading `stage @song @spectral/garden`
or `bench @song X` would create a semantic overload without a
substrate-decl justification. The substrate-already-had-the-word
principle applies to NOUNS (movement/voice/progression exist as
species names) and to VERBS (focus/project/split/shift/settle exist
as prism ops) but NOT to the composition-head slots. Rung 3 needs
`song`/`movement`/`voice`/etc as new NAMED-BLOCK-DECLARATION heads
inside the grammar's five prism-op alphabet. That IS Path B.

**Grammar shape to land.** `shards/song/keywords.mirror` (~60 lines):

```mirror
in @prism

grammar @song("song") {
  # === top-level block ===
  focus song

  # === movement / voice / progression / narrative / phrase blocks ===
  focus movement
  focus voice
  focus progression
  focus narrative
  focus phrase

  # === leaf beat block ===
  focus beat

  # === single-line directives inside blocks ===
  project scope
  project stage
  project phase
  project from
  project to
  project frame
  project coupling
  project convergence
  project completes_when
  project predicate
  project dark_pass
  project active_pass
  project propagation
  project directed_toward_cadence
  project cadence_type
  project stepwise_or_leap
  project attribution_discipline
  project narcissus_watch
  project narrative
  project lines
  project arc
  project transmit
  project wire
  project projects
  project coherent
  project bounded
  project fits_in_working_memory
  project obc_binding
  project composition_algebra
  project shadow_ancestry
  project psychohistorical_reading
  project substrate_vs_wire_distinction
  project completes
  project frame
  project action
  project voices
  project progressions
  project phrases
  project beats
}
```

**Two-tick landing.**
- **Tick 3a (this spec, Mara, 📝):** adjudication + grammar
  production + companion-keyword file design.
- **Tick 3b (Reed, 🔴🟢):** land `shards/song/keywords.mirror`,
  register in `bootstrap/src/grammar.rs::companion_keyword_sources`,
  extend `bootstrap/src/song.rs` to walk the AST instead of
  hand-parsing lines, land
  `bootstrap/tests/peer_beam_song_movement_shard.rs` per Taut
  `c54740c` §5.4 (T04 parses Mara `d21337b` §5.1 verbatim).

---

## §1. Substrate-already-had-the-word audit

Per `[[feedback-substrate-already-had-the-word]]` (~72nd instance
across the arc): grep-first, mint-last. Comprehensive audit of what
the substrate ALREADY carries for Rung 3.

### 1.1 What IS substrate-already-had-the-word (coverage: high)

| Rung 3 element | Existing carrier | Landing OID | Reuse |
|---|---|---|---|
| `song` as species-family-root | `shards/song.mirror` (`f01cf9f`) | Arc 6 TICK 1 | Grammar head reuses the family-root's OWN NAME. |
| `movement` as substrate species | `shards/song/movement.mirror` (`4efbf16`) | Arc 6 TICK 4 | Grammar block IS the instantiation surface for the landed species. |
| `voice` as substrate species | `shards/song/voice.mirror` (`cc5a440`) | Arc 6 TICK 3 | ditto |
| `progression` as substrate species | `shards/song/progression.mirror` (`54ff1e8`) | Arc 6 TICK 2 | ditto |
| `narrative` as substrate species | `shards/song/narrative.mirror` (`0434a39`) | Arc 6 TICK 5 | ditto |
| `phrase` as substrate species | `shards/song/phrase.mirror` (`6b9bc5c`) | Arc 6 TICK 6 | ditto |
| `beat` as substrate species | `shards/song/beat.mirror` (`94e55eb`) | Rung 0 (Mara sixth-species mint) | ditto |
| Companion-keyword file pattern | `shards/mirror/spec/keywords.mirror` (2026-06-09) | precedent | Reused verbatim: `<op> <keyword>` two-word lines. |
| Companion registration in Rust | `bootstrap/src/grammar.rs::companion_keyword_sources` | landed | One-line addition to add `shards/song.mirror` → `shards/song/keywords.mirror` mapping. |
| Path-namespace pact | `@epistemologic/pact/path_matches_namespace` (2026-06-16+) | landed | `.song` file at `<path>/X.song` declares `song X` at grammar altitude; nested species land at species-declaration altitude. |
| Prism-op alphabet (5-op) | `shards/prism.mirror` + every prism-body | landed | `focus`/`project`/`split`/`shift`/`settle` are the ONLY tokenizer heads; song grammar uses `focus <block-head>` + `project <line-directive>` per exactly the same discipline as `shards/mirror/spec/keywords.mirror`. |
| Grammar block declaration | `grammar @NS("file-suffix") { ... }` shape | 2026-06-09 (spec/keywords.mirror; earlier at boot/std/mirror/glass/ast/token.mirror) | Song's grammar block reuses the same shape: `grammar @song("song") { ... }`. |
| File-extension → grammar dispatch | `grammar_for_file`/`grammar_for_file_in` in `bootstrap/src/lib.rs` | landed | Extends by matching the `.song` suffix to the `@song` grammar's declared file-suffix. |
| Tokenize/parse infrastructure | `bootstrap/src/tokenize.rs::tokenize` + AST | landed | Parser walks the AST directly; no new tokenizer code. |

**Substrate-already-had-the-word coverage: ~85%.** The composition
mechanism (companion-keyword file + grammar-block declaration +
file-extension dispatch + tokenize+parse walk) is fully landed. The
species-nouns are fully landed. The prism-op alphabet is fully landed.

### 1.2 What is genuinely new (coverage: 15%)

Three things:

1. **The `shards/song/keywords.mirror` file itself.** ~60 lines,
   two-word-line `<op> <keyword>` declarations of the tokenizer
   vocabulary for song blocks. Substrate-decl only; no new
   composition primitive.

2. **One-line addition to `companion_keyword_sources`** in
   `bootstrap/src/grammar.rs` (~4 lines with match arm):
   ```rust
   "shards/song.mirror" => &["shards/song/keywords.mirror"],
   ```

3. **The parser lift in `bootstrap/src/song.rs`.** Reed replaces
   the current line-based `song_content.lines().filter(...)` with
   `let ast = tokenize(&source, &grammar_for_song_file())` + a
   recursive AST walk that emits per-block envelopes. ~150 lines.
   NO new tokenizer; NO new parser combinators; just AST walk.

**Nothing new invented.** Every mechanical piece is a landed pattern
consumed at a new site. This is the definitional shape of
substrate-pull-honest landings.

### 1.3 What Path C would need (and fails to provide)

Path C ("substrate-already-had-the-word: reuse existing keywords
without minting new grammar heads") was audited across every landed
composition primitive. The candidates:

- **`stage @song X { ... }`** — `stage` is @mirror/lens grammar
  (compile/kintsugi/shatter/bootstrap/etc. per
  `shards/mirror/lens/cli/compile.mirror`). It declares a CLI verb
  sub-prism, not a temporal composition. Overloading it with `@song`
  parameterization creates dual semantics for the same keyword; refused
  per `[[feedback-substrate-already-had-the-word]]` inverse condition
  (the WORD is not the SEMANTIC).

- **`prism @song/deployment { ... }`** — `prism` is the family-root
  declaration head (5-op body). It's not a composition block; it's a
  species/root declaration. Reusing it for song-instance composition
  breaks the prism-vs-instance distinction that
  `[[architecture-prism-as-trait-as-everything]]` names.

- **`glass @song/deployment { ... }`** — same problem as prism; glass
  is the imperfect/transparency carrier declaration, not a temporal
  composition surface.

- **`bench @song X { ... }`** — bench is @mirror/bench's substrate
  (perf-harness carrier per `shards/mirror/bench.mirror`). Reusing
  it for song composition is nonsensical at the semantic altitude.

- **`project @song X { ... }`** — project is mirror.spec's top-level
  (per `shards/mirror/spec.mirror`). Overloads the file-level manifold
  declaration with per-song-instance composition; two-tick discipline
  refuses (readability over foundation).

- **`command song X { ... }`** — command is @mirror/lens/cli's
  cli-verb declaration. A song is not a CLI verb; refused.

- **`grammar @song("song") { song X { ... } }`** — putting `song X`
  INSIDE a grammar block conflates the grammar-declaration altitude
  with the composition-instance altitude. Refused.

**Path C fails.** The substrate does NOT already have a composition-
head slot that fits `@song` semantics. The composition-head is the
new material. But the MECHANISM by which it's introduced (companion-
keyword file + `<op> <keyword>` declarations) IS 100%
substrate-already-had-the-word. That's Path B's discipline: mint the
NOUN at grammar-head altitude, reuse the MECHANISM.

### 1.4 Verdict on Path C

Path C is refused with substrate-honest reasoning: there is no
existing carrier whose semantics ARE `@song X { movement Y { voice Z
{ ... } } }` composition. Every candidate creates a
semantic-overload fault-plane. The word `movement` is available in
the tokenizer vocabulary (no keyword collision), it names a landed
substrate species, and its role at grammar-altitude is
substrate-honest (movement-BLOCK instantiates @song/movement.enter
at composition altitude).

---

## §2. The three paths formalized

### 2.1 Path A — song.rs internal DSL parser (current-Rung-2 extended)

**Shape.**
```rust
// bootstrap/src/song.rs (Path A extension)
fn parse_song_v3(source: &str) -> Song {
    // hand-rolled recursive-descent parser for:
    //   song X { movement Y { voice Z { ... } } }
    // consumes tokens directly from source; no grammar-harvester.
}
```

**Cost.** ~500 lines new Rust. Duplicates every mechanism the
tokenizer already carries (comment-strip, brace-matching, name
extraction, in-clause handling, seam discipline). Reed alone can
execute; single-tick if Reed accepts the substrate-divergence.

**Benefit.** Fast. No cross-cutting substrate changes. Reed
independent of Mara.

**Fault-plane created.** Song grammar diverges from mirror grammar.
`shards/song.mirror` declares species; `.song` files parse under a
different discipline. Two future consumers (e.g., `@spectral/db`
wanting to store `.song` files as substrate-native decls, or
`@mirror/mosaic` wanting to compile `.song` files to derivation
graphs) would have to bridge the divergence. Every downstream
consumer pays the divergence tax.

**Substrate-honest verdict on Path A: REFUSED.** Path A is a
runtime-shortcut that trades short-term velocity for long-term
substrate fragmentation. It violates
`[[architecture-shards-as-substrate-source]]` (keywords are
declarations, not runtime constants). The temptation to take Path A
is real (Reed alone can ship it in one tick), and that temptation is
exactly what substrate-pull discipline exists to refuse.

### 2.2 Path B — companion-keyword file + tokenize+AST-walk (recommended)

**Shape.**

File 1: `shards/song/keywords.mirror` (Reed lands per §3 below).

File 2 (Rust): `bootstrap/src/grammar.rs::companion_keyword_sources`
gets one match arm:
```rust
"shards/song.mirror" => &["shards/song/keywords.mirror"],
```

File 3 (Rust): `bootstrap/src/lib.rs::grammar_for_file_in` gets one
extension-match line:
```rust
if suffix == "song" { return load_grammar_at("shards/song.mirror"); }
```

File 4 (Rust): `bootstrap/src/song.rs::execute_song` replaces
`song_content.lines().filter(...)` with:
```rust
let source = std::fs::read_to_string(song_path)?;
let grammar = crate::grammar::load_grammar_at("shards/song.mirror");
let ast = crate::tokenize::tokenize(&source, &grammar);
walk_song_ast(&ast, ctx)
```

**Cost.** ~60 lines .mirror declaration + ~4 lines Rust registration
+ ~150 lines AST walker. Total: ~210 lines. Multi-tick (Mara spec
adjudication done here; Reed lands the file + registration + walker
in one Tick 3b).

**Benefit.** Substrate-unified. Every future consumer (spectral/db,
mosaic, kintsugi, mcp) reads `.song` files through the same
tokenize+parse infrastructure as `.mirror` files. No divergence
tax. The song file IS a mirror-native declaration; the extension
just names the grammar to dispatch.

**Fault-plane closed.** Song files become substrate-first-class.
Same discipline that already carries `.mirror` files carries `.song`
files. Every downstream consumer benefits (no bridge code).

**Substrate-honest verdict on Path B: RECOMMENDED.** Path B extends
the substrate's own composition mechanism to a new consumer without
inventing new primitives. It IS
`[[architecture-shards-as-substrate-source]]` operating at
grammar-consumer altitude. It closes Reed's Rung 2 line-parse
divergence via the honest lift (single-line and multi-line songs
were degenerate cases; Rung 3 is where the structure demands
grammar-altitude parsing).

### 2.3 Path C — no new keywords; nested existing carriers

**Shape.** Refused per §1.3-§1.4. No existing carrier's semantics
fit; overloading creates fault-planes.

**Substrate-honest verdict on Path C: REFUSED with detailed audit.**

---

## §3. The canonical grammar production (Path B)

### 3.1 The five-op alphabet applied to song blocks

The tokenizer admits ONLY five head-kinds: `focus`, `project`,
`split`, `shift`, `settle`. Each corresponds to one AST-node kind
(`AstKind::Focus` etc). Any new keyword is declared as ONE of these
five ops via `<op> <keyword>` in a `grammar { ... }` block.

**Choice of op per keyword** (rationale per
`shards/mirror/spec/keywords.mirror` precedent):

- `focus <keyword>` — opens a recursively-scanned brace block. USE
  for every keyword that introduces `NAME { ... }` nested content.
- `project <keyword>` — single-line directive with one
  identifier/ref/value payload. USE for every keyword that is a
  field-inside-a-block.
- `settle <keyword>` — recursively-scanned brace block AT
  terminal-altitude (settle_on-shaped). USE for close-events (none
  needed at song-file altitude this Rung; forward-promised for
  `settle_on { cadence_authentic; ... }` at Rung 4+).

Under this discipline, the song-file grammar is a straightforward
lift.

### 3.2 Formal grammar production

```
song_file          ::= song_block+

song_block         ::= "song" NAME "{" song_body "}"
song_body          ::= (movement_block | voice_block | progression_block
                       | narrative_block | phrase_block)*

movement_block     ::= "movement" NAME "{" movement_body "}"
movement_body      ::= (movement_field | voice_block | progression_block
                       | narrative_block | phrase_block | beat_block)*
movement_field     ::= "voice" ":" REF
                     | "stage" ":" NAME
                     | "from" ":" VALUE
                     | "to" ":" VALUE
                     | "frame" ":" VALUE
                     | "narrative" ":" STRING
                     | "coupling" ":" VALUE
                     | "predicate" ":" VALUE
                     | "dark_pass" ":" VALUE
                     | "active_pass" ":" VALUE
                     | "propagation" ":" VALUE
                     | "completes_when" ":" VALUE

voice_block        ::= "voice" NAME "{" voice_body "}"
voice_body         ::= voice_field*
voice_field        ::= "scope" ":" REF
                     | "lines" ":" VALUE
                     | "stepwise_or_leap" ":" VALUE
                     | "attribution_discipline" ":" VALUE
                     | "narcissus_watch" ":" STRING

progression_block  ::= "progression" NAME "{" progression_body "}"
progression_body   ::= progression_field*
progression_field  ::= "voice" ":" REF
                     | "phase" ":" PHASE_ARROW
                     | "from" ":" VALUE
                     | "to" ":" VALUE
                     | "directed_toward_cadence" ":" VALUE
                     | "cadence_type" ":" NAME
                     | "coupling" ":" VALUE
                     | "convergence" ":" VALUE
                     | "narcissus_watch" ":" STRING
                     | "narrative" ":" STRING

narrative_block    ::= "narrative" NAME "{" narrative_body "}"
narrative_body     ::= narrative_field*
narrative_field    ::= "frame" ":" VALUE
                     | "shadow_ancestry" ":" VALUE
                     | "convergence" ":" VALUE
                     | "completes" ":" VALUE
                     | "psychohistorical_reading" ":" STRING
                     | "arc" ":" VALUE
                     | "transmit" ":" VALUE
                     | "wire" ":" REF
                     | "projects" ":" STRING
                     | "substrate_vs_wire_distinction" ":" STRING

phrase_block       ::= "phrase" NAME "{" phrase_body "}"
phrase_body        ::= phrase_field*
phrase_field       ::= "coherent" ":" VALUE
                     | "bounded" ":" VALUE
                     | "fits_in_working_memory" ":" VALUE
                     | "obc_binding" ":" STRING
                     | "composition_algebra" ":" STRING

beat_block         ::= "beat" NAME "{" beat_body "}"
beat_body          ::= beat_field*
beat_field         ::= "action" ":" REF

PHASE_ARROW        ::= NAME ("->" NAME)+          # e.g. split -> shift -> settle
REF                ::= "@" NAME("/" NAME)*        # e.g. @mirror/mosaic
VALUE              ::= NAME | REF | LITERAL | LIST
LITERAL            ::= STRING | NUMBER | BOOL | SIGIL_PATH
LIST               ::= "[" VALUE ("," VALUE)* "]"
NAME               ::= [a-zA-Z_][a-zA-Z0-9_]*
STRING             ::= "\"" ... "\""
```

### 3.3 What each keyword MEANS structurally

- **`song X { ... }`** — top-level block instantiating an @song
  composition named X. Composes over @song family-root
  (`shards/song.mirror` `f01cf9f`). NAME is a bare identifier at
  Rung 3; forward-promised to accept `@spectral/garden/deployment`-
  style substrate refs at Rung 4 (per Mara `d21337b` §5.1).

- **`movement <name> { ... }`** — frame-entry declaration; INSTANCE
  of `@song/movement.enter` (per `shards/song/movement.mirror`
  `4efbf16`). Fields carry the frame's parameters (voice, stage,
  from, to, frame-bounds, narrative). Composes with StageFreight
  five-stage cascade at runtime.

- **`voice <name> { ... }`** — agent-line declaration; INSTANCE of
  `@song/voice.advance` (per `shards/song/voice.mirror` `cc5a440`).
  Fields carry the voice's scope (which substrate carrier this
  voice IS), lines (what it does), stepwise-or-leap discipline,
  attribution.

- **`progression <name> { ... }`** — cadence-directed path
  declaration; INSTANCE of `@song/progression.extend` (per
  `shards/song/progression.mirror` `54ff1e8`). Fields carry the
  phase sequence, cadence type (authentic/plagal/deceptive/half),
  coupling, convergence criteria.

- **`narrative <name> { ... }`** — psychohistorical arc + wire
  projection declaration; INSTANCE of `@song/narrative.arc` (per
  `shards/song/narrative.mirror` `0434a39`). Fields carry the
  frame, shadow-ancestry, convergence-criteria, and (optionally)
  the wire-projection target.

- **`phrase <name> { ... }`** — OBC-bounded atomic-unit
  declaration; INSTANCE of `@song/phrase.join` (per
  `shards/song/phrase.mirror` `6b9bc5c`). Fields carry the
  coherent-content, bounds, OBC-binding predicate, composition
  algebra.

- **`beat <name> { action: @ref }`** — atomic-execution
  declaration; INSTANCE of `@song/beat.strike` (per
  `shards/song/beat.mirror` `94e55eb`). Field carries the
  substrate action to fire (typically `@kintsugi/oscillate` per
  `shards/song.mirror:181` verbatim binding).

### 3.4 A minimum-viable Rung-3 song file (for parser acceptance)

```mirror
song hello_world {
  movement greet {
    voice: @mirror/lens/cli
    stage: audition
    from: silence
    to: greeting

    voice compiler {
      scope: @mirror/mosaic
      lines: assemble hello -> world
      stepwise_or_leap: stepwise
    }

    progression compile {
      voice: @mirror/mosaic
      phase: split -> shift -> settle
      directed_toward_cadence: yes
      cadence_type: authentic
    }

    phrase unit {
      coherent: greeting_bytes
      bounded: single_line
      obc_binding: bytes_equal_expected
    }

    beat strike {
      action: @kintsugi/oscillate
    }
  }
}
```

This example must parse under §3.2's grammar; Reed's Tick 3b test
T04 asserts this shape. Mara `d21337b` §5.1's `@spectral/garden/
deployment` composition is a superset — its extra fields (`coupling`,
`convergence`, `completes_when`, `shadow_ancestry`, etc.) are all
enumerated in §3.2's grammar.

---

## §4. Mapping to landed `@song` species (Path B)

The load-bearing structural claim: each song-file BLOCK is a
declarative instantiation of one substrate species's ACTION. The
grammar is not new semantics; it's a surface syntax for constructing
typed @song values under the substrate's existing species algebra.

| Song-file block | Substrate species | Action instantiated | Fields → action-params |
|---|---|---|---|
| `song X { ... }` | `@song` (family-root, `f01cf9f`) | family-root instance construction | X becomes the song's name; body becomes species-child list |
| `movement <name> { ... }` | `@song/movement` (`4efbf16`) | `song_movement(m: ref).enter(m, ctx)` | fields become `m`'s frame-parameters |
| `voice <name> { ... }` | `@song/voice` (`cc5a440`) | `song_voice(v: ref).advance(v, ctx)` | fields become `v`'s trajectory-parameters |
| `progression <name> { ... }` | `@song/progression` (`54ff1e8`) | `song_progression(p: ref).extend(p, ctx)` | fields become `p`'s phase/cadence-parameters |
| `narrative <name> { ... }` | `@song/narrative` (`0434a39`) | `song_narrative(n: ref).arc(n, ctx)` | fields become `n`'s frame/shadow-parameters |
| `phrase <name> { ... }` | `@song/phrase` (`6b9bc5c`) | `song_phrase(ph: ref).join(ph, ctx)` | fields become `ph`'s OBC-parameters |
| `beat <name> { action: @X }` | `@song/beat` (`94e55eb`) | `song_beat(b: ref).strike(b, ctx)` | `action` field is the substrate morphism to fire |

**Structural consequence.** A parsed song-file AST IS a tree of
typed action-instantiations rooted at the family-root's `song X`
node. Reed's Rung 3 discharge walks this AST and emits envelopes at
each species altitude. Runtime execution (Rungs 4+) dispatches
`species.action(instance, ctx)` at each leaf. This is the substrate-
pull-honest shape: parse produces typed values; runtime consumes
typed values; no untyped intermediate layer.

**What Rung 3 does NOT commit to.** Runtime EXECUTION semantics of
the deep nested species (movement's StageFreight cascade;
progression's cadence-classification; narrative's wire-projection)
are NOT lifted at Rung 3. Rung 3's discharge is PARSER ACCEPTANCE +
envelope emission at each block-altitude. Rungs 4+ layer the
execution semantics per Taut `c54740c` §5 ladder.

---

## §5. Six Rung 3 sub-ambiguities — adjudicated

### 5.1 (a) Path A vs Path B vs Path C

**Verdict: Path B.** Detailed reasoning §0-§2. Path A creates
substrate-divergence fault-plane; Path C fails at the composition-
head slot. Path B extends the substrate's OWN composition mechanism
(companion-keyword file + tokenize+AST-walk) to a new consumer
without inventing new primitives.

### 5.2 (b) Top-level new keywords vs nested under existing

**Verdict: Top-level, per §1.3 audit.** `song` becomes a new
top-level keyword in the tokenizer vocabulary via
`shards/song/keywords.mirror`'s `focus song` line. Similarly for
`movement`/`voice`/`progression`/`narrative`/`phrase`/`beat`
(all `focus <keyword>` since each opens a `{ ... }` body).

No nesting under `prism`/`bench`/`stage`/`glass`/`spectral` — every
such nesting was audited and refused (§1.3). The keywords live in a
new grammar-declaration file (`shards/song/keywords.mirror`) rather
than the existing `shards/mirror/spec/keywords.mirror` because they
are @song's substrate, not @mirror/spec's substrate. Path-namespace
pact discipline applies.

### 5.3 (c) Need for `shards/song/keywords.mirror`?

**Verdict: YES, required.** Per §3.1: any new keyword MUST be
declared via `<op> <keyword>` two-word line in a `grammar { ... }`
block, then the file MUST be registered in
`bootstrap/src/grammar.rs::companion_keyword_sources`. Precedent:
`shards/mirror/spec/keywords.mirror` for mirror.spec keywords;
`boot/std/mirror/glass/ast/token.mirror` for legacy grammar
keywords. The companion-keyword file IS the substrate-decl of the
tokenizer's vocabulary at this altitude.

**File location:** `shards/song/keywords.mirror` (matches
path-namespace pact for the @song namespace). NOT `shards/song.mirror`
(that's the family-root; keeps its species-decl body intact).

### 5.4 (d) Action-decl body: `\` obligation-block discipline?

**Verdict: YES for the typed lambda declarations; NO for the
grammar-declaration file.** Two discipline surfaces here:

- **`shards/song/keywords.mirror`** — this file contains ONLY
  `<op> <keyword>` two-word lines inside a `grammar @song("song")
  { ... }` block. NO typed lambdas; NO `\` obligation blocks. It
  is a pure keyword-declaration file, matching the precedent shape
  of `shards/mirror/spec/keywords.mirror`.

- **`shards/song.mirror` (family-root)** — MAY be extended with
  typed lambda declarations for the grammar-head consumers. E.g.:
  ```mirror
  # song(name) — declares an @song instance named `name`.
  song(name) -> song_progression { \ }

  # movement(name) — declares a movement block inside a song.
  movement(name) -> song_movement { \ }

  # (similarly for voice, progression, narrative, phrase, beat)
  ```
  Each `\` obligation body is discharged by the runtime AST walker
  (Reed's Tick 3b). Forward-promise: Rung 4+ ticks lift these
  obligations to typed executions per species-action semantics.
  This tick can defer the typed lambda extension to Tick 3b or
  land it inline; both are substrate-honest. **Recommendation for
  Reed at Tick 3b: land the typed lambdas inline in
  `shards/song.mirror` to close the substrate-decl seam before the
  parser lift.**

### 5.5 (e) Song file extension

**Verdict: `.song`.** Reasoning:

- `.song` is unambiguous, human-readable, matches the family-root
  name at grammar-altitude, and matches the grammar-block's
  file-suffix declaration (`grammar @song("song")`).
- `.mirror` would conflate the composition-instance file (a specific
  song) with the substrate-declaration file (@song's family-root).
  Refused per two-tick discipline (readability over foundation).
- `.mirror.song` (compound extension) adds no substrate value;
  refused.

**Forward-promise:** Rung 4+ may accept `.mirror` files that CONTAIN
`song X { ... }` blocks alongside other declarations (a mirror-native
file whose body composes multiple substrate declarations). At Rung
3, keep the extension discipline crisp: `.song` files are pure @song
compositions; `.mirror` files are mixed substrate declarations.

**Reed's current test fixture** (`peer_beam_song_phrase_shard.rs`
line 86: `three_beats.song`) already uses `.song` — this verdict is
byte-compatible with Rung 2.

### 5.6 (f) Song file location

**Verdict: absolute path via `--song <file>` flag.** No change from
Rung 1/2. Reasoning:

- `mirror peer beam <peer_home> --song <file>` — the operator supplies
  `<file>` as an absolute path or path-relative-to-cwd; Reed's Ctx
  handles resolution.
- Location IS NOT peer_home-bound. A song is composed centrally
  (by the operator, or by the compiler emitting a deployment-song
  from mirror.spec at Rung 5+) and dispatched TO a peer. The
  peer is the target, not the origin.
- Forward-promise: Rung 5+ may add `--song @<oid>` for
  content-addressed song retrieval from `@bauchladen` (per @spectral/
  garden/deployment mycelial-propagation composition).

**Reed's current dispatch** (`bootstrap/src/song.rs::execute_song`
signature) already handles this correctly; no change at Rung 3.

---

## §6. The reference parse target — Mara `d21337b` §5.1 verbatim

Per Taut `c54740c` §5.4 T04: *"parser accepts Mara `d21337b` §5.1
verbatim (execution not required this rung; only parse acceptance)."*

The reference composition (Mara `d21337b` §5.1, `docs/specs/song-
replaces-plans-and-loops.md:511-810`):

```mirror
song @spectral/garden/deployment {
  movement enter_deployment_epoch { ... }
  movement perform_deployment_epoch { ... }
  movement review_deployment_epoch { ... }
  movement publish_deployment_epoch { ... }
  movement narrate_deployment_epoch { ... }
  voice mirror_compiler { ... }
  voice nix_builder { ... }
  voice mycelial_propagator { ... }
  voice spectral_engineer_endpoint { ... }
  progression compile_to_derivation { ... }
  progression propagate_mycelial { ... }
  progression deploy_to_spectral_engineer { ... }
  narrative arc { ... }
  narrative transmit { ... }
  phrase derivation_unit { ... }
  phrase propagation_unit { ... }
  phrase deployment_unit { ... }
}
```

### 6.1 Syntactic consistency check

Every block-head in the reference is declared in §3.2's grammar:
`song`, `movement`, `voice`, `progression`, `narrative`, `phrase`.
Every field-line in the reference is declared in §3.2's field
enumeration: `voice`, `stage`, `from`, `to`, `frame`, `narrative`,
`coupling`, `predicate`, `dark_pass`, `active_pass`, `propagation`,
`completes_when`, `scope`, `lines`, `stepwise_or_leap`,
`attribution_discipline`, `narcissus_watch`, `phase`,
`directed_toward_cadence`, `cadence_type`, `convergence`, `arc`,
`transmit`, `wire`, `projects`, `substrate_vs_wire_distinction`,
`shadow_ancestry`, `psychohistorical_reading`, `completes`,
`coherent`, `bounded`, `fits_in_working_memory`, `obc_binding`,
`composition_algebra`.

**Diff surfaced.** One item requires attention:

- **`song @spectral/garden/deployment` (substrate-ref name)** — the
  reference uses `@spectral/garden/deployment` as the song's NAME.
  §3.2's grammar production says `song_block ::= "song" NAME "{"
  ...` where `NAME ::= [a-zA-Z_][a-zA-Z0-9_]*`. This is a substrate-
  ref, NOT a bare identifier.

  **Adjudication:** Extend `NAME` at song-block-head altitude to
  accept substrate-refs (`@<path>/<path>`). The tokenizer already
  parses `@<name>` refs via `is_name_char` (which includes `/`).
  Two options for Reed's Tick 3b:

  1. **Broaden `NAME` in the song-block-head production only.** The
     song-block-head accepts `NAME | REF`; every nested block-head
     stays `NAME`. This is the substrate-honest reading: the
     top-level song IS content-addressed (it IS a compilation-unit at
     the composition altitude); nested species-instances are locally
     named. Recommended.

  2. **Broaden `NAME` at every block-head.** Consistent but loses
     the substrate-altitude distinction between top-level and
     nested. Refused.

  Reed's Tick 3b implements Option 1: `song_block ::= "song"
  (NAME | REF) "{" ...`. The reference parses; T04 passes.

**No other diffs.** The reference IS syntactically consistent with
the grammar at every other position.

### 6.2 Prose lines (`narrative: "..."` strings)

The reference's `narrative:` fields contain multi-line prose strings.
The tokenizer's string-literal handling (per `bootstrap/src/tokenize.rs`)
accepts `"..."` at project-directive altitude. Multi-line strings
either use `"""..."""` (triple-quote form, if supported) or single-
line strings joined with implicit newline discipline. **Reed's Tick
3b MUST verify** the tokenizer handles the reference's prose strings;
if not, the fixture for T04 uses shortened single-line prose and the
long-prose case is forward-promised to a follow-up tick.

---

## §7. Two-tick landing sequence

### Tick 3a — Mara spec (this tick, 📝)

**File:** `docs/specs/song-file-is-mirror-native-grammar.md` (this
document).

**Deliverable:** Path B adjudication + formal grammar production +
mapping to landed species + six-ambiguity resolution + reference-
parse-target consistency check.

**No `.mirror` files land this tick.** Substrate-decl adjacencies
(recognition-candidate naming, related-shards cascade, ancestry) are
in-spec.

**Commit shape:**
```
📝 Mara [substrate-pull:realize] [ladder-rung-3-spec] @song grammar
substrate direction — Path B verdict + canonical grammar production
for song/movement/voice/progression/narrative/phrase/beat nested blocks
via companion-keyword file
```

### Tick 3b — Reed lift (following, 🔴🟢)

**Files landed:**

1. **`shards/song/keywords.mirror`** (~60 lines) — companion-keyword
   file per §3.1; declares tokenizer vocabulary for song blocks and
   field directives.

2. **`shards/song.mirror`** (~40 lines added) — extend family-root
   with typed lambda declarations for grammar-head consumers per
   §5.4: `song(name)`, `movement(name)`, `voice(name)`,
   `progression(name)`, `narrative(name)`, `phrase(name)`,
   `beat(name)`. Each `\`-obligation-body discharged by Rust walker.

3. **`bootstrap/src/grammar.rs::companion_keyword_sources`** (~4
   lines) — add match arm:
   ```rust
   "shards/song.mirror" => &["shards/song/keywords.mirror"],
   ```

4. **`bootstrap/src/lib.rs::grammar_for_file_in`** (~3 lines) — add
   extension match for `.song` suffix; dispatches to
   `shards/song.mirror` grammar.

5. **`bootstrap/src/song.rs`** (~150 lines edited) — replace
   `song_content.lines().filter(...)` with tokenize+AST-walk:
   ```rust
   let grammar = crate::grammar::load_grammar_for_file(song_path, ctx);
   let source = std::fs::read_to_string(song_path)?;
   let ast = crate::tokenize::tokenize(&source, &grammar);
   walk_song_ast(&ast, peer_home, ctx)
   ```
   `walk_song_ast` recursively walks song → movement → voice/
   progression/narrative/phrase → beat, emitting per-block
   envelopes.

6. **`bootstrap/tests/peer_beam_song_movement_shard.rs`** (~200
   lines) — RED-first per Taut `c54740c` §5.4:
   - `t01_movement_envelope_contains_voices_progressions_phrases`
   - `t02_progression_cadence_type_reported`
   - `t03_voice_lines_advance_settle`
   - `t04_parses_mara_d21337b_section_5_example_syntactically`
   - `t05_movement_composes_prior_phrase_semantics` (Rung 2
     regression)

**Backward-compat.** Rung 2's line-per-beat song files
(`three_beats.song` = 3 lines) must still parse under the new
grammar. Two options:

- **Option BC-1: Add a legacy `beat-line` alternative to the
  grammar.** `song_body` admits either `movement_block`-etc OR a
  bare-line-beat form. Ugly; refused.

- **Option BC-2: Auto-wrap legacy files.** When parsing produces
  zero recognized blocks and every non-blank line is a bare token,
  wrap them as implicit `phrase implicit { beat b0 { action:
  @kintsugi/oscillate } beat b1 { ... } }`. This preserves Rung 2
  byte-equality via an implicit-lift discipline. Recommended.

- **Option BC-3: Break Rung 2 fixture.** Update
  `peer_beam_song_phrase_shard.rs` to use the new grammar. Cleanest;
  requires coordinating fixture updates in the same tick. Also
  acceptable.

**Reed's choice at Tick 3b.** Prefer BC-3 if the fixture update is
one-file-scope (it is). Prefer BC-2 if broader external consumers
have relied on the line-per-beat form (they haven't — Rung 2 landed
2 hours ago).

**Commit shape (Reed):**
```
🔴🟢 Reed [ladder-rung-3-landing] @song grammar Rung 3 GREEN —
shards/song/keywords.mirror companion-keyword file + grammar
extension + song.rs AST-walker + peer_beam_song_movement_shard.rs
```

### Tick 3c — contingent

**Not needed for Path B.** Path B does NOT extend the mirror.spec
core grammar (mirror.spec's keywords stay in `shards/mirror/spec/
keywords.mirror`; song's keywords live in `shards/song/keywords.mirror`;
disjoint namespaces). No `bootstrap/src/lib.rs::collect_declared_
namespaces` change; no mirror.spec keyword-table update.

**Forward-promise for Rung 4+:** if `song X { ... }` blocks land
INSIDE `.mirror` files (mixed-substrate declarations), the grammar
dispatch in `grammar_for_file_in` may need companion-merging of
`@song` keywords into the `@mirror/spec` grammar. That is a Rung 4+
concern; substrate-honest to defer.

---

## §8. Refusals and forward-promises

### 8.1 Refusals

- **Refuse Path A** (song.rs internal DSL parser) — substrate-
  divergence fault-plane; violates
  `[[architecture-shards-as-substrate-source]]`.

- **Refuse Path C** (nest song blocks under existing keywords) —
  every candidate overloads existing semantics; refused per §1.3
  audit.

- **Refuse inventing new prism-op-head kinds** — the alphabet is
  fixed at five (`focus`/`project`/`split`/`shift`/`settle`) per
  `shards/prism.mirror`. Song grammar uses `focus <block-head>` and
  `project <field-directive>`; NO new op-kinds needed.

- **Refuse placing song keywords in
  `shards/mirror/spec/keywords.mirror`** — path-namespace pact
  violation; @song's keywords live under @song's namespace.

- **Refuse changing the file extension away from `.song`** — Rung 2
  already uses `.song`; two-tick-consistency preserved.

### 8.2 Forward-promises

- **Rung 4 — @dance runtime.** Multi-peer song execution requires
  coordinated dispatch across peer_homes. The grammar production
  above does NOT declare multi-peer syntax; song grammar stays
  single-peer at Rung 3. Rung 4 lands per Taut `c54740c` §5.5.

- **Rung 4+ — typed value types inside fields.** Fields today accept
  bare identifiers, refs, and strings. Typed values (nested VALUE
  productions with structured content — e.g. `phase: [split, shift,
  settle]` as a list of tokens, or `κ(node_i, node_j)` as a
  parametric expression) forward-promised at Rung 4+.

- **Rung 4+ — settle_on clauses at song altitude.** The song's
  cadence-close criteria (Aumann agreement, Kuramoto threshold,
  `verify_coherence`) may want a `settle_on { ... }` block at song
  altitude. Forward-promised; not needed at Rung 3.

- **Rung 5+ — `.mirror` files containing `song` blocks.** Mixed-
  substrate `.mirror` files that declare both spec structure and
  embedded songs. Requires @song keyword-companion merging into
  @mirror/spec's grammar. Forward-promised.

- **Rung 5+ — compiler-generated songs.** `@mirror/mosaic` emitting
  a deployment-song from `mirror.spec`'s `garden { ... }` block per
  Mara `d21337b` §5. Requires bidirectional grammar (mosaic writes
  `.song` files; song.rs reads them). Grammar shape stays as
  declared; only the writer is new. Forward-promised.

- **Rung 6 — recognition upgrade.** If Path B ratifies through Rung
  4 landing, `#R-song-grammar-is-mirror-spec-native` upgrades from
  CANDIDATE to LANDED; if by Rung 6 the substrate consumers span
  song-files, mirror-files, mosaic-output, and MCP-tool, it upgrades
  to PROMOTED.

- **Beat action vocabulary expansion.** Rung 3's `beat { action:
  @X }` today admits any @-ref. Rung 4+ ticks may add typed action
  dispatch (specific actions for `@kintsugi/oscillate`,
  `@spectral/garden/nix.build`, `@bauchladen.publish`, etc.).
  Forward-promised.

---

## §9. Recognition candidate

**Name:** `#R-song-grammar-is-mirror-spec-native`.

**Body:** Song files (`.song`) are mirror-native declarations parsed
by the same tokenize+AST-walk infrastructure as `.mirror` files. The
keywords `song`/`movement`/`voice`/`progression`/`narrative`/
`phrase`/`beat` become substrate-declared tokenizer vocabulary via a
companion-keyword file (`shards/song/keywords.mirror`), following
the precedent shape of `shards/mirror/spec/keywords.mirror`
(2026-06-09). No parser divergence; no internal DSL; no runtime-
constant keywords. The composition-mechanism is 100% substrate-
already-had-the-word; the noun-set (movement/voice/progression/etc)
is 100% substrate-already-had-the-word (species-landed Arc 6 TICKs
1-6 + Rung 0 beat mint); the new material is a ~60-line
`<op> <keyword>` declaration file + a one-line Rust registration +
a ~150-line AST walker.

**Ratification path.**
- CANDIDATE this tick (Mara §9 name).
- LANDED after Reed Tick 3b (companion file + AST walker + T04
  passing Mara `d21337b` §5.1 verbatim parse).
- PROMOTED after Rung 4+ multi-peer @dance consumer + at least one
  external `.song` file in the corpus that isn't the test fixture.
- Recognition-candidate paired: `#R-substrate-companion-keyword-
  file-lifts-any-consumer-to-mirror-native-grammar-parity` — the
  generalization: any @X family-root can lift its consumer files to
  mirror-native grammar via the same companion-keyword-file pattern.
  This recognition is scaffolded by song's landing; forward-promise
  to name it if a second consumer lands the same shape (e.g.,
  `@spectral/db` for `.db` files, or `@resonance` for `.resonance`
  files).

**Load-bearing effect.** Reed's Rung 2 line-parse divergence closes.
Every future consumer of `.song` files (spectral/db,
mosaic-compiler, kintsugi-formatter, mcp-tool) reads through the
same tokenize+parse discipline. The substrate becomes YET MORE
UNIFIED at composition-consumer altitude.

**Alternative recognition (only if Path A had been recommended,
which it wasn't):** `#R-song-grammar-is-song-rs-dsl-until-consumer-
pull`. Refused; documented for completeness.

---

## §10. Recognition ancestry

**Load-bearing sources (this arc):**

- **Alex 2026-07-13 in-transcript.** Ladder-climb mandate: *"climb
  the ladder until unresolvable ambiguity that cannot be postponed
  further"*; re-invoked after Reed reported Rung 3 substrate-
  direction ambiguity. This spec resolves the ambiguity.

- **Taut `c54740c`** (2026-07-13, `docs/scouts/2026-07-13-taut-
  mirror-spawn-song-beat-gap-scout.md`) — §5.4 Rung 3 spec + T04
  reference-parse-target (Mara `d21337b` §5.1 verbatim).

- **Reed `5fdc009`** (Rung 1 RED) + **`c36fbf5`** (Rung 1 GREEN) —
  `--song` flag + `bootstrap/src/song.rs` module + hardcoded single-
  beat dispatch. Establishes the CLI-surface + envelope shape.

- **Reed `79eee6f`** (Rung 2 RED) + **`70766c3`** (Rung 2 GREEN) —
  multi-beat phrase parsing (non-empty lines = beats) + `@song/
  phrase` authority + `phrase_beat_count` field. Rung 1 backward-
  compat preserved. THIS is the substrate-direction hinge — line-
  parse works at Rung 2 but degenerates at Rung 3 where nested block
  structure demands grammar-altitude parsing.

- **Mara `d21337b`** (2026-07-13, `docs/specs/song-replaces-plans-
  and-loops.md`) — §5.1 canonical `@song @spectral/garden/deployment`
  composition. THE reference shape T04 must parse verbatim. Every
  block-head + field-directive in that composition IS declared in
  §3.2's grammar (with the substrate-ref-as-name diff adjudicated
  in §6.1).

- **Mara `94e55eb`** — `shards/song/beat.mirror` sixth species mint
  (Rung 0). Lifts *"oscillate's ACTIVE/DARK alternation IS the beat"*
  from prose-altitude to species-decl-altitude.

- **Mara `4f079c8`** — `@dance` canonical spec (`docs/specs/dance-as-
  coordination-without-signal-on-forster-torus.md`); Path C
  recognition scaffolding referenced by song's paradigm-terminal
  composition.

**Prior arc (Arc 6 close):**

- `shards/song.mirror` (`f01cf9f`) — @song family-root, Arc 6 TICK 1.
- `shards/song/progression.mirror` (`54ff1e8`) — Arc 6 TICK 2.
- `shards/song/voice.mirror` (`cc5a440`) — Arc 6 TICK 3.
- `shards/song/movement.mirror` (`4efbf16`) — Arc 6 TICK 4.
- `shards/song/narrative.mirror` (`0434a39`) — Arc 6 TICK 5.
- `shards/song/phrase.mirror` (`6b9bc5c`) — Arc 6 TICK 6.

**Grammar-substrate ancestry:**

- `shards/mirror/spec/keywords.mirror` (2026-06-09) — companion-
  keyword-file precedent. The pattern this spec extends verbatim.
- `boot/std/mirror/glass/ast/token.mirror` (2026-06-04) — earliest
  companion-keyword-file (@mirror/glass/ast/token). Legacy precedent.
- `bootstrap/src/grammar.rs::companion_keyword_sources` +
  `merge_keyword_sources` — the runtime that consumes companion
  keyword files.
- `bootstrap/src/tokenize.rs::tokenize` — the parser that walks
  grammar declarations, produces typed AST.
- `bootstrap/src/ast.rs::AstKind` — the five-op alphabet (Focus,
  Project, Split, Shift, Settle) + boundary terminals (In, Out).
- `shards/prism.mirror` — five-op prism trait; the substrate-decl
  origin of the alphabet.
- `[[architecture-shards-as-substrate-source]]` — keywords are
  declarations, not runtime constants; the load-bearing principle
  Path B honors.
- `[[architecture-prism-as-trait-as-everything]]` — the five ops
  recursively at every altitude.
- `[[architecture-lift-as-load-bearing]]` — shift is basis
  transformation; grammar-altitude parsing IS the lift from
  line-based parse to typed AST.
- `[[feedback-substrate-already-had-the-word]]` — 72nd instance;
  the substrate had the WORDS (movement/voice/progression) and the
  MECHANISM (companion-keyword file); the composition-head slot
  is the only new material.
- `[[feedback-legibility-over-foundation-when-collapsing]]` —
  `.song` extension preferred over `.mirror` (readable name over
  foundational); song file location stays flag-based (readable) not
  peer_home-bound (foundational).

**External math ancestry (from Mara `d21337b`):**

- Zarlino 1558 + Rameau 1722 — cadence typology (authentic/plagal/
  deceptive/half) that `progression { cadence_type: authentic }`
  fields encode.
- Kuramoto 1975 — phase-oscillator coupling that `progression
  { coupling: ... convergence: ... }` fields encode.
- Aumann 1976 — common-prior agreement that `narrative { completes:
  Aumann_agreement_on_current_root_OID }` encodes.
- Foerster 1976 — mycelial substrate exposition; the Heist's
  substrate context Alex's proposal sits in.
- Batanin 1998 — globular composition; the mycelial coupling's
  N-fold factoring.
- Cooper & Meyer 1960 + Curwen 1858 + Galen ~170 CE — beat as
  atomic-execution-unit ancestry (per `shards/song/beat.mirror`
  Rung 0 substrate-decl).

---

## §11. What this spec resolves and what it doesn't

**Resolved (unblocks Reed Tick 3b):**

1. Path B is the substrate direction. (§0, §2)
2. `shards/song/keywords.mirror` is the file to land. (§3.1, §5.3)
3. The keyword→op mapping is `focus <block-head>` +
   `project <field-directive>` per §3.1.
4. `.song` is the file extension; `--song <file>` is the flag
   discipline. (§5.5, §5.6)
5. The formal grammar (BNF-equivalent) is §3.2.
6. Reed's parser lift replaces line-parse with tokenize+AST-walk.
   (§7 Tick 3b)
7. Mara `d21337b` §5.1's `@spectral/garden/deployment` composition
   IS the T04 parse-target; consistency check surfaces one diff
   (substrate-ref-as-song-name) with a one-line grammar extension
   fix. (§6)
8. Two-tick landing (Mara spec + Reed lift); no Tick 3c required
   for Path B. (§7)
9. Backward-compat with Rung 2's `three_beats.song` fixture: BC-3
   (update fixture) recommended. (§7)

**Not resolved (deferred with substrate-honest reasoning):**

1. Runtime EXECUTION semantics of deep species (movement.enter's
   StageFreight cascade; progression.extend's cadence-classification;
   narrative.arc's wire-projection). Deferred to Rungs 4+ per Taut
   `c54740c` §5.5-§5.7.
2. Multi-peer @dance dispatch. Deferred to Rung 4.
3. Compiler-generated songs from mirror.spec. Deferred to Rung 5+.
4. `.mirror` files containing embedded `song` blocks. Deferred to
   Rung 5+.
5. Typed value expressions inside fields (parametric expressions,
   list-of-tokens with structured content). Deferred to Rung 4+.
6. Full `settle_on` clause at song altitude. Deferred to Rung 4+.

Alex's mandate: *"climb the ladder until unresolvable ambiguity that
cannot be postponed further"*. Rung 3 ambiguity IS resolved.
Deferred items are Rung 4+ ambiguities that can be postponed until
Rung 4 begins.

---

**Word count:** ~4750. Every claim cited with file, line, and OID
where landed, or explicit forward-promise / spec-only naming where
not. Reed executes Tick 3b immediately following this spec's
landing.

**End of spec.**
