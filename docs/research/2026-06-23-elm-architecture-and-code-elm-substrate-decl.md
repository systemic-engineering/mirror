# The Elm Architecture (TEA) + @code/elm — notes for future substrate-decl

*2026-06-23. Mara. Survey altitude (not canonical). Research notes informing
forward-promised substrate-decl decisions. Kagi-verified per-claim where
marked; substrate-pull characterizations Mara-inferred unless cited.*

---

## §1. Recognition

The substrate keeps re-encountering one functional-reactive UI shape:

```
Model        (typed state)
View         (pure projection Model → Html)
Msg          (typed event sum)
Update       (pure (Msg, Model) → (Model, Cmd Msg))
Runtime      (the eventloop that ties them together)
```

This is The Elm Architecture (TEA), crystallized by Evan Czaplicki c. 2014 in
the Elm language. It has since seeded a generational lineage of UI frameworks
across every typed-FP-adjacent ecosystem: Gleam Lustre on BEAM+JS, Purescript
Halogen in JS, Iced and Yew in Rust, F# Bolero on .NET Blazor, Scala Tyrian
on Scala.js, Swift TCA (The Composable Architecture), and — at one remove —
React + Redux, the mainstream-JS reinterpretation that carried the pattern to
the largest consumer base.

Notes now because Loop Phase D (@docs family-root, combining @ui's instrument
primitives with @nl's typographic-content primitives) needs a composition
pattern for the spectral.engineer content layer. Gleam Lustre — the
TEA-shaped framework on the Gleam cascade that systemic.engineering already
runs on — is the substrate-pull-obvious candidate. Before committing, the
substrate wants to see the broader TEA landscape AND ask whether the load-
bearing primitive is `@code/elm` (the grammar) or `@docs/tea` / `@ui/tea`
(the composition pattern). This document maps both.

What this research informs: forward-promised `@code/elm` shard; the @docs
page-rendering pattern decision; and a candidate recognition #96/#97
territory around TEA-as-substrate-composition-pattern.

---

## §2. The Elm Architecture (TEA) substrate-decl shape

TEA is, structurally, a **typed eventloop with strict purity** between event
emission and state mutation. The substrate primitives it carries:

**Core triple — Model + View + Update.**
- `Model`: a typed state carrier. In Elm, this is a record type. The
  substrate's analogue: a `typed_state` carrier under a per-app grammar.
- `View`: a **pure** function `Model → Html Msg`. Returns a typed DOM
  fragment whose event handlers carry `Msg` values. Substrate analogue:
  a pure morphism `state → presentation` whose presentation carrier
  carries typed event references.
- `Update`: a **pure** function `(Msg, Model) → (Model, Cmd Msg)`. Takes
  the current state plus an incoming message, returns the new state plus
  optional effects to dispatch. Substrate analogue: a pure morphism
  `(event, state) → (state, effect_stream)`.

**Msg — the action-stream substrate.**
A sum type. Every interaction the View can produce is a constructor.
Exhaustive pattern match in Update means the compiler proves every
event has a handler. Substrate analogue: a typed_event sum under
`@epistemologic/property/exhaustive`.

**Runtime — the eventloop substrate.**
The Elm runtime is the only impure thing. It (1) renders the View, (2)
collects DOM events, (3) dispatches Msg values into Update, (4) executes
returned Cmds, (5) feeds Cmd results back as new Msgs, (6) re-renders via
virtual-DOM diff. Substrate analogue: the substrate-altitude eventloop;
the IO boundary at which purity ends.

**Commands (Cmd) + Subscriptions (Sub) — the effect surface.**
- `Cmd Msg`: a description of an effect to perform (HTTP request, time
  query, port-out), tagged with the Msg constructor to receive its
  result. Substrate analogue: a typed effect descriptor at `@io`.
- `Sub Msg`: a description of an event stream to listen to (websocket,
  animation frame, port-in). Substrate analogue: a typed subscription
  to an external event stream at `@io`.

The architectural invariant: **purity at the form altitude; effects only
at the @io boundary**. Update returns Cmds; the Runtime executes them;
results re-enter as Msgs. This is the substrate's form/substance partition
(#50) and alignment-as-boundary-mathematics (#57) operationalized at the
UI scale.

**How TEA maps to mirror substrate primitives:**

| TEA piece          | Mirror substrate analogue                                |
|--------------------|----------------------------------------------------------|
| Model              | `typed_state` carrier (per-app grammar)                  |
| Msg                | typed_event sum + `@epistemologic/property/exhaustive`   |
| Update             | pure action `(msg, model) -> (model, cmd)`               |
| View               | pure action `model -> presentation`                      |
| Cmd Msg            | typed effect descriptor at `@io`                         |
| Sub Msg            | typed subscription at `@io`                              |
| Runtime            | substrate-altitude eventloop                             |
| Html Msg           | presentation carrier parameterized by event type         |
| Virtual-DOM diff   | `kintsugi` between successive presentations              |

**Composed bilateral candidate:**

```
update_well_formed(u: update_fn, p: perturbation) -> verdict
view_well_formed(v: view_fn, p: perturbation) -> verdict
msg_well_typed(m: msg_type, p: perturbation) -> verdict
tea_app_coherent(model, view, update, msg) -> verdict
  requires update_well_formed(update, p)
  requires view_well_formed(view, p)
  requires msg_well_typed(msg, p)
```

Same composed-bilateral pattern as @cascade.cascade_well_defined,
@ui.ui_instrument_coherent, and the form/substance bilaterals across
the substrate (recognition #59 — kintsugi loop altitude-portable).

---

## §3. TEA variants across ecosystems

Per-ecosystem subsection. The breadth shows that TEA is not an Elm-specific
artifact — it is a substrate-discoverable shape that multiple independent
ecosystems landed on. That is the substrate-pull signal worth noting.

### §3.1 Elm itself (the canonical)

- **Author:** Evan Czaplicki.
- **Current version:** **0.19.1 (October 2019)**. *Kagi-verified.*
- **Release cadence:** Slow. Per multiple community sources surfaced in
  search (elm-discourse "Where is Elm going" thread, January 2024 update),
  Evan's compiler commits since 2019 have been sparse — mostly npm-
  installation polish. No 0.20 / 1.0 announcement has landed in 2024 or
  2025. *Kagi-verified.*
- **Substrate-pull characterization:** the canonical TEA. Strict purity.
  Single-package ecosystem governance (Evan owns the namespace). Compiler
  errors that became the gold standard for "friendly error messages" in
  the typed-FP community. No type classes — uses concrete types and ad-
  hoc record extensibility. Targets JavaScript only.
- **Community state:** Splintering. Elm-craft community site active; Elm
  Radio podcast active through late 2023; alternative compiler forks
  exist (gren-lang is the most prominent fork, taking TEA forward
  without Evan). Production users remain (NoRedInk historically, plus
  smaller shops); new adoption has plateaued.
- **Risk for @code/elm:** the language is **effectively maintenance-mode**.
  Substrate-decl'ing a grammar for a language whose upstream cadence is
  measured in years is a hedge: the substrate-decl outlives the grammar's
  evolution rate.

### §3.2 Gleam Lustre (the load-bearing one for spectral.engineer)

- **Author:** Hayleigh Thompson; published under `lustre-labs`.
- **Current version:** **v5.6.0 (June 2025 announcement on hexdocs)**.
  *Kagi-verified.*
- **Compatibility:** "Lustre up to date with the wider Gleam ecosystem,
  making Lustre compatible with the v1 releases of both gleam_erlang and
  gleam_otp" per the 5.6.0 announcement. *Kagi-verified.*
- **Substrate-pull characterization:** TEA on the Gleam cascade. Compiles
  to **both BEAM and JavaScript** via Gleam's dual targets — the same
  Lustre app runs server-side on the BEAM with OTP supervision AND
  client-side as a JS bundle. Universal Components pattern (presented at
  CodeBEAM Vancouver) allows components to ship server-rendered + client-
  hydrated from one source. *Kagi-verified.*
- **Relevance:** systemic.engineering already runs on the Gleam cascade
  (per CLAUDE/MEMORY). Lustre is the substrate-pull-obvious framework for
  the @docs content layer IF the conclusion is "use a TEA-shaped framework."
- **Risk:** young ecosystem (Gleam itself reached v1 in 2024); Lustre 5.x
  versioning suggests rapid iteration, which is good for a young
  framework but means breaking changes between 5.x → 6.x are possible.
  Less battle-tested than React/Redux at scale.

### §3.3 Purescript Halogen

- **Maintainer:** purescript-halogen org (originally Slamdata).
- **Substrate-pull characterization:** TEA-cousin built on **Free monads**.
  More mathematically dense than Elm; uses row polymorphism for component
  composition; supports HKT (Purescript has type classes, unlike Elm).
  Components form a tree; messages bubble; state per-component, not
  global.
- **Current state:** Mara-inferred from 2025-05 community signal: a
  Purescript Discourse post titled "PSA: stop recommending Halogen (we
  have React)" surfaced in search, suggesting the community is shifting
  toward Purescript-React bindings. *Kagi-surfaced; verdict the community
  itself is debating.*
- **Risk for @code/halogen:** if the Purescript community itself is
  moving off Halogen, substrate-deciding to follow it is anti-substrate-
  pull. Better to substrate-decl `@code/purescript` (which already exists
  as a cascade species, `shards/cascade/code/purescript/js.mirror`) and
  let consumers pick their own UI framework.

### §3.4 Rust Iced

- **Author:** Héctor Ramón (iced-rs org).
- **Current version:** **0.14.0 (December 7, 2025)**. *Kagi-verified.*
- **Status:** pre-1.0. Iced 0.14 announced as "the final experimental
  release before 1.0," shipping reactive rendering, time travel
  debugging, and animation APIs. *Kagi-verified.*
- **Substrate-pull characterization:** TEA for native cross-platform GUI.
  Splits the app into **State + Messages + Update + View**, exactly the
  Elm four-tuple. From the iced-rs README: "Inspired by The Elm
  Architecture, Iced expects you to split user interfaces into four
  different concepts." *Kagi-verified.*
- **Targets:** Windows, macOS, Linux, and the Web (via WASM, per the
  iced docs.rs page). *Kagi-verified.*
- **Relevance to spectral:** @ui's GPU eigenboard rendering is its own
  substrate (instrument-altitude per recognition #96 territory), but if
  spectral ever needs a TEA-shaped native control surface around the
  eigenboard, Iced is the substrate-pull-obvious Rust choice. Already
  composes with `@cascade/code/rust/wasm` for browser delivery.

### §3.5 Rust Yew

- **Maintainer:** yewstack org.
- **Current state:** active maintenance through mid-2026 per GitHub
  activity surfaced in search. *Kagi-verified.*
- **Substrate-pull characterization:** TEA-shaped Rust → WASM framework
  for client-side web apps. Component-based with `html!` macro;
  message-update-view cycle; uses `Properties` for component inputs.
  More React-influenced than pure-Elm; supports hooks-style functional
  components alongside class-style components. *Kagi-verified composition;
  Mara-inferred ergonomic comparison.*
- **Targets:** WASM client-side rendering. *Kagi-verified.*
- **Relevance:** would compose cleanly with `@cascade/code/rust/wasm`
  (shards/cascade/code/rust/wasm.mirror, landed 2026-06-23). If
  spectral.engineer wanted a Rust-only frontend stack, Yew + the
  existing rust-wasm cascade is the substrate-pull-obvious path.

### §3.6 F# Bolero / Elmish

- **Maintainer:** fsbolero org (IntelliFactory ecosystem).
- **Substrate-pull characterization:** brings F# to WebAssembly via
  **Blazor**, with HTML functions in Elmish (the F# TEA library) syntax.
  *Kagi-verified.* Elmish itself is the canonical .NET TEA library,
  predating Bolero and used across the F# UI ecosystem (Fable for JS,
  Avalonia for desktop, Bolero for Blazor-WASM).
- **Risk:** Blazor's WASM performance is a known throughput concern
  (fib(40) benchmark surfaced as historically slow per a 2020 r/fsharp
  thread; performance has improved since but remains below native-JS
  frameworks). *Kagi-surfaced.*
- **Relevance:** if `@code/fsharp` ever lands as a substrate-decl
  (currently not on the roadmap), the Elmish family is the substrate-
  pull-obvious UI side.

### §3.7 Scala Tyrian

- **Author:** Dave Smith (indigoengine org).
- **Current version:** **0.13.0**. *Kagi-verified.* "Elm-inspired Scala
  UI library" per the official site.
- **Use in the wild:** Google Summer of Code 2025 surfaced building a
  web UI for Workflows4s using Scala.js + Tyrian. *Kagi-verified.*
- **Relevance:** if `@code/scala` + `@cascade/code/scala/js` ever land
  (Scala.js is mature — 1.19.0 in April 2025 per Kagi-surfaced
  release announcement), Tyrian is the TEA-shaped UI option. Not on
  the spectral roadmap; noted for completeness.

### §3.8 Swift Composable Architecture (TCA)

- **Authors:** Brandon Williams + Stephen Celis (pointfreeco).
- **Current state:** **active in 2025**, documented at 1.12.0 in the
  pointfreeco GitHub Pages reference. *Kagi-verified.*
- **Substrate-pull characterization:** TEA in Swift for SwiftUI apps.
  Adds Swift-specific ergonomics on top of the Redux-influenced TEA
  base: **type-safe composition** (combining child reducers into parent
  reducers via lenses), **controlled side effects** (Effect type
  parameterized by Action), and **testing tools** for time-traveling
  reducer tests. *Kagi-verified.* Designed across Point-Free's video
  series — the most pedagogically documented TEA derivative in any
  ecosystem.
- **Relevance:** if spectral ever ships a native Apple-side companion
  (Glint as a native macOS app, say), TCA is the substrate-pull-obvious
  Swift choice. `@code/swift` does not exist yet.

### §3.9 React + Redux (the mainstream descendant)

- **Maintainer:** Meta (React) + Redux team (independent).
- **Current state:** Redux Fundamentals tutorial updated through
  February 2025; React Redux beginner guides published February 2025.
  *Kagi-verified.* Redux Toolkit (RTK) is the modern canonical form.
- **Substrate-pull characterization:** TEA's mainstream descendant.
  Direct mapping: `Msg = Action`, `Update = Reducer`, `Model = State`,
  `View = React component tree`. Loses TEA's purity guarantee at the
  language level (JavaScript can't enforce reducer purity); recovers
  much of it via Redux conventions + RTK's createSlice.
- **Relevance:** the largest TEA-influenced ecosystem by consumer
  count. If spectral.engineer ever wanted to ship React-based content,
  the TEA pattern would translate; substrate-pull would push toward
  Redux Toolkit + TypeScript for the purity-recovery margin. The
  forward-promised `@code/typescript` (not yet substrate-decl'd) would
  cascade to JS just like Purescript and Gleam do.

### §3.10 Kotlin Compose Multiplatform (adjacent, NOT pure TEA)

- **Maintainer:** JetBrains.
- **Current state:** **stable on Android, iOS production-ready as of
  KotlinConf 2025**. *Kagi-verified.* Targets Android, iOS, desktop, web.
- **Substrate-pull characterization:** declarative reactive, NOT pure
  TEA. Uses **state hoisting** and `MutableState` rather than
  Msg/Update; closer to SwiftUI than to Elm. Listed here as adjacent
  ancestor — TEA's "View as pure function of state" principle is
  carried, but Msg/Update is replaced by direct state mutation via
  hoisted callbacks.
- **Relevance:** counterexample-adjacent. Shows that "declarative
  reactive UI" is a broader category than TEA; TEA's specific
  contribution is the **typed message stream + pure update**, not just
  the View-as-pure-function-of-state.

### Variant count: **10 covered.** Of those, 9 are TEA-derived; Compose is
the adjacent-ancestor counterexample noted for partition clarity.

---

## §4. @code/elm substrate-decl shape proposal

**The question:** should `@code/elm` exist as a glass instance under @code
(per the @code/rust, @code/beam, @code/wasm, @code/gleam pattern)?

**The mechanical shape — what @code/elm would look like.**

If landed, it would follow the @code/gleam template (shards/code/gleam.mirror,
77 lines, the substrate-pull-tightened instance of the @code family):

```
prism @code/elm {
  focus elm
  project elm
  split elm
  shift elm
  settle elm
}
```

**Carriers:**

- `elm_source`: a `.elm` source file or fragment
- `elm_module`: a namespaced compilation unit (Elm's module system)
- `elm_package`: a published Elm package (elm.json declaration)
- `elm_app`: an Elm application root (Browser.element / Browser.document)

**Cascade species.** Elm targets **JavaScript only** — there is no
elm-native or elm-wasm target. The cascade species would be exactly one:

```
shards/cascade/code/elm/js.mirror   (Elm → JS; via `elm make`)
```

This is the simplest cascade in the @cascade family — single source,
single target, single toolchain, well-defined exit format.

**Honest hedge: should @code/elm exist?**

Three positions to weigh:

1. **Yes, decl it.** Elm's TEA is the canonical reference for the
   substrate's typed-eventloop pattern. Having `@code/elm` makes the
   substrate's ancestry explicit, and the cascade is the cleanest
   single-target case in the @cascade family (good for pedagogical
   weight in spec docs). Low maintenance cost — Elm doesn't change.

2. **No, don't decl it; substrate-decl the PATTERN instead.** What's
   load-bearing for spectral is the TEA composition pattern, not the
   Elm grammar specifically. If @docs uses Gleam Lustre, then `@code/
   gleam` is the substrate-decl that pulls weight, and Lustre's TEA
   shape composes through @cascade/code/gleam/js. Elm-the-grammar is
   redundant unless spectral specifically intends to compile or
   consume Elm code, which it doesn't.

3. **Defer.** No spectral or systemic.engineering code is in Elm.
   The Gleam cascade (which IS used) gives TEA via Lustre. `@code/
   elm` would be substrate-decl'd for completeness/ancestry, not for
   active consumption. Defer until a concrete consumer appears.

**Mara-recommendation: position (2) + (3) combined.** Don't substrate-
decl `@code/elm` until a consumer appears; DO substrate-decl the TEA
composition pattern (next section). The pattern is what's load-bearing;
the grammar is just one instance.

This is the same substrate-pull verdict the @code family already carries
for languages spectral doesn't actively use: `@code/elixir`, `@code/
ruby`, `@code/swift`, etc. do not have shards. The substrate only
declares grammars at the altitude it actively uses. Elm is currently
in that "noted ancestor, no active consumer" bucket.

---

## §5. TEA as substrate composition pattern

**The load-bearing question.** Could TEA be substrate-decl'd as a
composition pattern at @ui or @docs altitude (NOT at @code/elm altitude)?

**Proposed location.** `shards/docs/tea.mirror` (preferred) OR
`shards/ui/tea.mirror`. Reasoning per the @ui shard's recognition #96
candidate territory: @ui declares INSTRUMENT primitives (GPU rendering,
mote/arc/field), @docs (forward-promised, Phase D) composes @ui + @nl
for presentation. TEA is a **presentation-altitude composition pattern**,
not an instrument-altitude primitive. It lives at @docs, not @ui.

**The pattern, as substrate-decl.**

```
in @prism
in @glass
in @meta
in @nl
in @epistemologic/cybernetic/coherence
in @epistemologic/cybernetic/distinction

prism @docs/tea {
  focus tea_app
  project tea_app
  split tea_app
  shift tea_app
  settle tea_app
}

# Carriers
type tea_model
type tea_msg <= sum
type tea_cmd <= effect_descriptor
type tea_sub <= subscription_descriptor
type tea_view(m: tea_model) -> presentation
type tea_update(msg: tea_msg, m: tea_model) -> (tea_model, tea_cmd)

# Bilateral discipline
update_pure(u: tea_update, p: perturbation) -> verdict { \ }
view_pure(v: tea_view, p: perturbation) -> verdict { \ }
msg_exhaustive(m: tea_msg, p: perturbation) -> verdict { \ }

tea_app_coherent(model, view, update, msg) -> verdict
  requires update_pure(update, p)
  requires view_pure(view, p)
  requires msg_exhaustive(msg, p)
  { \ }

out @docs/tea
```

**Why this is substrate-pull-correct.**

1. **Typed eventloop with strict purity** is exactly the shape the
   substrate keeps re-discovering. Update is pure; View is pure; only
   the Runtime touches @io. That's the form/substance partition (#50)
   at the UI scale — and alignment-as-boundary-mathematics (#57) made
   operational in the presentation layer.

2. **Composes with @ui (Mote/Arc/Field state as Model) + @docs (typed
   page content) + a Msg type per page.** A spectral.engineer page
   would be:
   - Model = the page's typed state (current section, scroll position,
     interactive widget state, eigenboard render-target if applicable)
   - View = pure projection from Model to the @nl-typographic + @ui-
     instrument presentation
   - Msg = the typed set of interactions that page accepts
   - Update = the pure reducer for that page
   - Cmd / Sub = the @io effects the page needs (data fetches,
     animation subscriptions)

3. **Connection to recognition #95 @cascade.** TEA's Msg type is a
   typed event. If `Msg → Update` is read as a typed functor, the
   loss-lens framing (recognition #95) applies: each Update step has
   a measurable loss between predicted and actual next-Model. A
   spectral-instrumented TEA app could measure "how predictable is
   the user's next action given current Model?" as a loss surface.
   That's a forward connection worth flagging but NOT building yet.

4. **Composes with Gleam Lustre as the active runtime.** Lustre IS
   the TEA-shaped framework on the cascade spectral.engineering
   already runs (Gleam → BEAM + JS). Substrate-decl'ing `@docs/tea`
   at the pattern altitude makes Lustre a **canonical implementor**
   of the @docs/tea spec — same way @ui declares the primitives and
   `/Users/alexwolf/dev/projects/spectral/crates/ui` implements them.

**The recognition #96 (or #97) candidate.**

This is substrate-pull-confident enough to flag as a recognition
candidate. The pattern: **TEA-as-substrate-composition-pattern is the
presentation-altitude analogue of the form/substance partition (#50)
and the form/process partition (#55)**. It carries:

- Pure update + pure view at the form altitude
- Cmd/Sub at the substance/IO altitude
- Runtime as the boundary
- Msg as the typed event stream crossing the boundary

That's the substrate's own discipline pattern (recognitions #50, #55,
#57, #59) operationalized at the UI scale. The promotion gate is a
second independent witness — Halogen's Free-monad MVU might be the
second witness, since it carries the same purity discipline through a
different mechanism. Flag for Pack review.

---

## §6. Recommendations for spectral.engineer deployment

**Three concrete decisions Alex faces for Phase D / spectral.engineer:**

**1. UI framework for the @docs content layer.**

Mara-recommendation: **Gleam Lustre (TEA-shaped, on Gleam cascade)**.

Why: Lustre 5.6.0 (June 2025) is current; Gleam is already spectral's
chosen cascade per systemic.engineering precedent; TEA gives the
pure-update + pure-view discipline that composes with the substrate's
existing form/substance partition; Universal Components allow server-
rendered + client-hydrated from one source (good for spectral.engineer
content + interactive demos); BEAM supervision tree for server-side
robustness.

Risks: young ecosystem (Gleam v1 in 2024); rapid Lustre versioning
suggests possible breaking changes. Acceptable for a content site
with low traffic; revisit if scale changes.

**2. Substrate-decl TEA as `@docs/tea`.**

Mara-recommendation: **yes, candidate for landing in Phase D**.

Why: the pattern is load-bearing for spectral.engineer specifically and
substrate-pull-correct generally (sec §5). Substrate-decl'ing it lets
Lustre be a *canonical implementor* rather than the *de facto pattern*.
Lets future @docs implementations (if spectral ever ships React + TS
content too, say) share the spec.

Promotion gate before landing: Pack review on whether this is recognition
#96 / #97 candidate. If yes, the substrate-decl lands with full recognition-
canonical doc treatment per the #95 / #93 pattern. If no, lands as a
plain composition-shard without recognition altitude.

**3. Substrate-decl `@code/elm`.**

Mara-recommendation: **defer**.

Why: no active consumer; Gleam Lustre gives TEA without Elm; Elm itself
is effectively maintenance-mode (no 0.20 since October 2019). The
substrate-pull rule is "decl grammars actively used"; Elm fails the test.

Reconsider if: a spectral consumer adopts Elm, OR a pedagogical reason
appears (e.g., docs/specs/ wants to cite Elm with first-class substrate
treatment, parallel to how @code/rust is cited).

---

## §7. Forward-promised work

If the recommendations in §6 land:

1. **`shards/docs/tea.mirror`** — the TEA-as-substrate-composition-pattern
   shard. Phase D, post-Pack-review. Promotes to a recognition-canonical
   doc if recognition #96 / #97 lands.

2. **`shards/cascade/code/gleam/js.mirror`** — already exists
   (24.1KB, 2026-06-23, the Stage-1 cascade for Lustre's JS target).
   No new shard needed.

3. **`shards/cascade/code/gleam/beam.mirror`** — already exists
   (21.4KB, 2026-06-23, the Stage-1 cascade for Lustre's BEAM target).
   No new shard needed.

4. **`shards/code/elm.mirror`** — **deferred**. Re-evaluate when a
   consumer appears.

5. **`shards/cascade/code/elm/js.mirror`** — deferred with @code/elm.
   The simplest cascade in the family (single source, single target,
   well-defined toolchain via `elm make`); if @code/elm ever lands,
   this cascade is the trivial companion.

6. **Pack review trigger.** Open question for Seam / Reed / Mara / Glint
   / Taut: is TEA-as-substrate-composition-pattern a recognition #96 /
   #97 candidate? If yes, this notes document upgrades to a
   recognition-canonical doc under `docs/specs/recognitions/`.

---

## §8. Honest hedges

**Elm 0.19.1 status.** Evan Czaplicki's compiler activity since 2019 is
sparse per multiple community sources (Kagi-surfaced: 2024-01 elm-
discourse thread, lack of any 0.20 announcement through 2025). Real
community fragmentation: gren-lang fork carries TEA forward without
Evan; Lamdera adds full-stack features. Substrate-decl'ing `@code/elm`
inherits this uncertainty — the upstream cadence is glacial. The
defer-recommendation in §6 is the substrate-pull-honest call.

**TEA's runtime overhead and SSR/hot-reload patterns.** Strict purity is
not free. Each Update produces a full new Model; virtual-DOM diff is
necessary because the View is regenerated. For static-content sites
(spectral.engineer's primary use case), this overhead is negligible. For
heavy interactive demos (the eigenboard rendering), the cost matters —
and the eigenboard already lives at @ui's instrument-altitude, NOT at
@docs's presentation-altitude. So the TEA discipline applies to the
content shell, not the eigenboard interior. The partition is clean.

SSR: Lustre's Universal Components solve server-rendered + client-
hydrated from one source per the CodeBEAM Vancouver talk Kagi-surfaced.
This is a substantial advantage over Elm (which is client-only).

Hot-reload: Lustre's `lustre_dev_tools` package now uses Tailwind v4 per
the 5.6.0 announcement. Hot-reload is supported. Acceptable.

**Gleam Lustre maturity vs ecosystem size.** Lustre is at 5.6.0 (June
2025); Gleam itself reached v1 in 2024. Both are young relative to
React/Redux. The bet is that systemic.engineering's existing Gleam
commitment makes this acceptable risk — spectral.engineer's content
site is not the place to take a divergent bet, and Gleam's BEAM
foundation gives the OTP robustness floor that compensates for
ecosystem youth.

**TEA vs @ui's GPU rendering — partition clarity.** TEA is the substrate-
pull-correct shape for **presentation-altitude** UI (the @docs content
shell, navigation, prose-with-typed-state interactivity). The
**instrument-altitude** UI (the GPU eigenboard at `/Users/alexwolf/dev/
projects/spectral/crates/ui`) needs a different substrate — one that
talks to the GPU pipeline directly, where the Model is pixel/vertex
buffers and the Update is shader dispatch. This is exactly the
recognition #96 candidate territory the @ui shard names: instrument-
altitude vs presentation-altitude is a real partition.

So TEA does NOT compete with @ui's GPU rendering substrate — it composes
above it. The eigenboard is a single typed widget inside a TEA-shaped
@docs page; the page's Model carries a `widget_state(eigenboard)` field;
the View renders the prose around it and delegates the eigenboard to
@ui's instrument primitives. Two altitudes, one app.

**Survey vs canonical.** This document is survey altitude. The Pack-review
gate is real: if recognition #96 / #97 candidate lands, this content
gets re-rendered as a recognition-canonical doc. Until then, it
informs but does not commit.

---

*Mara, 2026-06-23. Eighth fire today.*
