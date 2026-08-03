# Taut Distributed-Colony-at-Web-Serving-Altitude Substrate Scout

**Date**: 2026-08-03
**Author**: Taut <taut@systemic.engineer>
**Task**: Scout the composition surface for Alex's vision (verbatim 2026-08-02):
> *"Der Compiler produziert multi-resonante Songs die verteilte Ameisenkolonien
> a la Conway's Game of Life in einem 5D spektralen Raum auf Consumer Hardware
> ausführen."*

**Cascade sequence**: `mirror substrate → cascade<mirror, gleam> →
gestalt-ui-shaped Gleam → cascade<gleam, js> → distributed peer colony
running Conway's Game of Life in 5D spectral space on consumer hardware`.

**Discipline**: Grep-first, read-only. No shard-decl mints. No math. No spec.
Substrate-honest: landed vs proposed vs adjacent vs missing.

---

## §1 Phase 1: gestalt-ui architecture surface

### 1.1 Location + build shape

- **Path**: `/Users/reed/dev/projects/gestalt-ui/`
- **Build**: Gleam project, `gleam.toml` name=`gestalt_ui`, version 0.1.0,
  stdlib `>= 0.44.0 and < 2.0.0`, gleeunit test-dep. **No lustre / mist / wisp
  dependency** — pure vocabulary crate. No `[targets]` block — inherits both
  `erlang` (BEAM) and `javascript` targets. Target-agnostic by construction.
- **Last touched**: 2026-04-03 (`ui.conv`), 2026-04-01 (`gleam.toml`),
  2026-02-28 through 2026-03-01 (source modules). **Not README.md** — Alex
  wrote a mirror substrate description directly at `ui.conv` instead.
- **No README.md.** Substrate-honest: the substrate description lives at
  `ui.conv` in mirror syntax, not English prose.

### 1.2 The `ui.conv` file — mirror grammar declaring gestalt-ui

The file `/Users/reed/dev/projects/gestalt-ui/ui.conv` is **the actual scout
surface**. It declares the mirror-side `@ui` grammar that gestalt-ui
Gleam realizes. Fully quoted for the record:

```mirror
grammar @ui {
  type = token | theme | composite | view | target

  type token = color | dimension | duration | shadow | radius | motion
  type theme = mode | density | contrast | scale | motion_pref
  type mode = light | dark
  type density = compact | comfortable | spacious
  type motion_pref = full | reduced

  type color = background | foreground | primary | muted | border | destructive
              | agent | human | system | working | idle | waiting | errored
  type dimension = rem | px
  type duration = ms

  type composite = typography | spacing | surface
  type view = signal | actor
  type target = css | ansi | svg

  action materialize { token: token, theme: theme, target: target }
  action compose { tokens: composite }
}

in @gestalt-tui
in @svg

out ui {
  materialize {}
  compose {}
}
```

Bilateral: **@ui carries `token | theme | composite | view | target`** with
two actions `materialize` (token × theme × target → concrete value) and
`compose` (composite tokens together). Composes over `@gestalt-tui` + `@svg`.

**Critical**: `ui.conv` uses the `.conv` extension — a `conversation`-format
predecessor of `.mirror`. This is **substrate-adjacent legacy** that Mara
will need to promote to a proper `shards/ui.mirror` species-decl.

### 1.3 Exported Gleam grammar (public API surface)

Module layout (11 source modules + tokens/):

```
src/gestalt_ui.gleam                        (root docblock only)
src/gestalt_ui/color.gleam                  (Hsl domain type + to_css / to_rgb)
src/gestalt_ui/dimension.gleam              (Dimension = Rem(Float) | Px(Float))
src/gestalt_ui/duration.gleam               (Duration = Ms(Int))
src/gestalt_ui/theme.gleam                  (Theme record: mode/density/contrast/scale/motion)
src/gestalt_ui/token.gleam                  (Token(a) = Token(materialize: fn(Theme)->a, description))
src/gestalt_ui/composite.gleam              (Typography composite + materialize_typography)
src/gestalt_ui/template.gleam               (source-string → interpolated output; if/for/#{})
src/gestalt_ui/view/actor.gleam             (ActorView + Role enum)
src/gestalt_ui/view/signal.gleam            (SignalView = Message|Question|Insight|Work|Exit|Init)
src/gestalt_ui/tokens/color.gleam           (semantic + status + surface HSL palettes)
src/gestalt_ui/tokens/motion.gleam          (durations respond to Motion axis)
src/gestalt_ui/tokens/radius.gleam          (sm/md/lg/xl/full static)
src/gestalt_ui/tokens/shadow.gleam          (mode-responsive shadow strings)
src/gestalt_ui/tokens/spacing.gleam         (by_density Dimension)
src/gestalt_ui/tokens/typography.gleam      (sans/mono families + size/leading/weight)
src/gestalt_ui/composites/typography.gleam  (heading/body/label/code/caption)
```

### 1.4 The composition pattern — M/V/U shape

**Token = function of Theme.** Central abstraction:
```gleam
pub type Token(a) {
  Token(materialize: fn(Theme) -> a, description: String)
}
pub fn materialize(token: Token(a), t: Theme) -> a { token.materialize(t) }
```

- **Materialize is theme-collapse.** Alex's substrate-honest phrasing (from
  `theme.gleam` docblock): *"Theme: the observation frame. Materialize
  collapses possibility space into actuality for a given theme."* This IS
  a witnessed-property-inference pattern with Theme as the collapse frame —
  the same shape as `apply_h::act` at rust/ floor.

- **Composites materialize by delegating to member tokens.**
  `composite.materialize_typography(t, theme)` calls `token.materialize` on
  each field — natural fold.

- **View layer = target-neutral view models.** `ActorView(id, role, nickname)`
  and `SignalView` sum type. **The view layer carries only IDs and roles,
  not renderers** — renderers live in downstream projects (glue/gestalt-tui).

- **Template layer.** `template.eval(source, context) -> Result(String,
  TemplateError)` — evaluates `- if / - for / #{key.field} / /- comment`
  directives. **This is the .gestalt template runtime, executable in Gleam
  on both BEAM and JS.** Value type is cross-BEAM-boundary friendly
  (Str/Int/Bool/Items/Null).

### 1.5 The Dhall shadow-substrate at `tokens/`

**Surprise finding**: `tokens/*.dhall` mirrors the Gleam token modules
one-to-one. `Theme.dhall`, `Token.dhall`, `color.dhall`, `motion.dhall`,
`radius.dhall`, `shadow.dhall`, `spacing.dhall`, `typography.dhall`, and
`view.dhall` (top-level bundle) — all use the same `materialize : Theme -> a`
shape.

**Interpretation**: Dhall was an earlier target-neutral encoding, Gleam is
the current one. The pattern `materialize: fn(Theme) -> a` is preserved
across the port. **This means the substrate-decl was already language-neutral;
Gleam is one realization, Dhall is another, and `.mirror` can be a third.**

### 1.6 The `[targets]` absence — this is the double-emit key

Neither `gleam.toml` nor any module pins `erlang` vs `javascript`. **The
whole vocabulary compiles to both BEAM and JS by default.** For the vision
this matters: gestalt-ui code can materialize server-side (BEAM) OR
client-side (JS) using the same substrate. **No fork.** This is the key
enabler for the browser-peer-colony vision.

### 1.7 Missing at gestalt-ui floor

- **No renderer.** Gestalt-ui carries vocabulary only. Renderers (HTML/SVG
  emitters, TUI cell-writers) live downstream (gestalt-tui + adjacent).
- **No component tree.** No `View a` / `Element` type. No layout algorithm.
- **No mount/reconcile.** No lustre-style app-loop.
- **No network / peer / signaling.** No WebRTC. No sync protocol.
- **No 5D spectral coordinate.** No `SpectralCoordinate<5>`. No Fiedler.
- **No CA update rule.** No neighborhood. No stepping.

**Verdict § Phase 1**: gestalt-ui provides **the theme-collapse vocabulary
and view-model shape**. It is the M-V-U primitive layer, minus the U (mount)
and minus the C (component). The `ui.conv` file IS the mirror substrate
declaration but lives in a `.conv` extension outside `shards/`. Composition
surface is CLEAN but SMALL — Mara will need substantial minting downstream
of gestalt-ui to reach the vision.

---

## §2 Phase 2: mirror-substrate composition ancestors

### 2.1 gestalt-mirror — the mirror-side canonical

**Path**: `/Users/reed/dev/projects/gestalt-mirror/`. Small, tight, load-bearing.

**File inventory** (10 files, ~14 KB total):
```
gestalt.mirror                              (59B — top-level use@document/@ui/@user)
protected/ui.mirror                         (835B — @ui grammar with theme+intents)
protected/user.mirror                       (2.4KB — abstract @user + optic algebra)
protected/user/neuro/adhd.mirror            (3.7KB — Alex's ADHD profile)
protected/user/neuro/audhd.mirror           (3.7KB — Alex's AuDHD profile via
                                                     adhd.then(autism))
protected/user/neuro/autism.mirror          (2.9KB — Alex's autism profile)
protected/user/neuro/nt.mirror              (2.1KB — Liana's stub profile)
public/document.mirror                      (726B — @document grammar)
```

**gestalt.mirror** (the whole file):
```mirror
grammar @gestalt {
  use @document
  use @ui
  use @user
}
```
Three-grammar composition. `@document × @ui × @user` — the document, its
appearance, and the reader who calibrates the appearance.

### 2.2 @ui in gestalt-mirror vs @ui in gestalt-ui/ui.conv — DIVERGENCE

`gestalt-mirror/protected/ui.mirror` is a **richer** @ui than
`gestalt-ui/ui.conv`. Key differences:

| Feature | gestalt-ui/ui.conv | gestalt-mirror/protected/ui.mirror |
|---|---|---|
| Extension | `.conv` (legacy) | `.mirror` (current) |
| Top-level types | `token \| theme \| composite \| view \| target` | `theme` only |
| Color model | `background..destructive \| agent..waiting..errored` | `color_intent = background..status(status_kind)` — parametric |
| Motion | `motion_pref = full \| reduced` | `motion_intent = fast \| normal \| slow \| slower` |
| Radius | not enumerated | `radius_intent = sm \| md \| lg \| xl \| full` |
| Shadow | not enumerated | `shadow_intent = none \| sm \| md \| lg \| xl` |
| Typography | as composite only | `typography_intent = heading..caption` |
| Role/signal | absent | `role = supervisor..human`, `signal_kind = message..exit` |
| Actions | `materialize + compose` | none declared |

**Interpretation**: `ui.mirror` in gestalt-mirror is the **type-only vocabulary**
(intent enums), while `ui.conv` in gestalt-ui adds the **action layer**
(materialize/compose). Neither is complete alone. **A proper unified `shards/ui.mirror`
species-decl would merge them.**

### 2.3 @user + neuroprofile substrate — the optic algebra

`protected/user.mirror` declares `abstract grammar @user` with:
- Four optic types named: `AffineTraversal`, `Prism`, `Lens`, `Traversal`
- Abstract templates: `project(t: theme) -> theme`, `optic -> optic_kind`,
  `label -> string`, `author -> string`, `invariants -> [constraint]`
- `then(other, t)` composition; `requires composition_satisfiable`;
  `recover |t, loss| { t }` (partial-recovery clause)

The four concrete profiles at `protected/user/neuro/{adhd,audhd,autism,nt}.mirror`
each implement the abstract grammar with named authors, invariants, and
anti-requirements. **The audhd profile is the derived intersection**:
```
adhd requires:   duration <= 300ms (attention bound)
autism requires: duration >= 200ms (perceptibility bound)
intersection:    200ms <= duration <= 300ms
```

`nt.mirror` is Liana's stub — deliberately partial. Test `"nt profile has
named invariants"` intentionally fails until Liana authors it. **Substrate
holds space for absent human authorship.**

Karen-cited empirical ancestors in these files:
- Barkley (1997) — working memory (adhd)
- Nigg (2017) — executive function (adhd)
- Happé & Frith (2006) — weak central coherence (autism)
- Green et al. (2012) — amygdala response to unexpected state change (autism)
- Marco et al. (2011) — sensory processing differences (autism)

### 2.4 @document (public) — the semantic-block vocabulary

`public/document.mirror` declares the standard prose/knowledge vocabulary:
- `section | paragraph | code_block | quote | callout | list | list_item |
  definition_list | table | figure | separator | breath | raw_block | embedded`
- Spans: `text | code | math | link | image | ref | emoji | spoiler | hard_break`
- Marks: `strong | emphasis | strikethrough | highlight | superscript | subscript`
- Meta: `id | role | extension` where `role = claim | evidence | question |
  answer | summary | aside | definition`
- Callout kinds: `note | tip | important | warning | caution`

This is **Alex Wolf's semantic-web-of-prose vocabulary** (aligned with
systemic.engineering essay conventions). Composes naturally with @gestalt for
rendering, and with @user for reader-adaptive rendering.

### 2.5 gestalt-tui — the terminal companion (renderer)

**Path**: `/Users/reed/dev/projects/gestalt-tui/`, target=erlang (BEAM-only),
depends on `gestalt_ui = { path = "../gestalt-ui" }`. This IS the **renderer
composition** for gestalt-ui: gestalt-tui takes gestalt-ui vocabulary and
emits ANSI escape sequences.

Modules: `ansi.gleam`, `bridge.gleam` (BEAM-to-reed distributed erlang link),
`layout.gleam`, `panel.gleam`, `render.gleam`. panel.gleam has a candid
docblock: *"When gestalt_ui path dep resolves, swap these for live token
materialization"* — currently uses hardcoded RGB constants from
gestalt-ui dark theme. **The composition surface is real but temporarily
denormalized.**

**Critical prior-art**: gestalt-tui already **connects to the Reed BEAM
body via distributed Erlang** (`bridge.connect("reed@localhost")` + poll
loop reading BodyState with `graph_nodes/edges/density/phase/dmn_beat_count/
hit_rate/miss_rate`). **This IS the peer-node-connects-to-remote-actor pattern
that the browser-peer-colony vision needs, minus the browser transport.**

### 2.6 garden-client — Lustre-based browser client (prior art for the JS target)

**Path**: `/Users/reed/dev/projects/garden-client/`, `target = "javascript"`,
deps: `gleam_stdlib`, `gleam_json`, **`lustre >= 5.0.0`**. THIS is the
mount+reconcile framework that the vision needs at the browser end.

Modules: `auth.gleam`, `graphql.gleam` (FFI to browser fetch/WS),
`repl.gleam` (Command sum type — Help/Domains/Grammar/State/Clear/Unknown),
`store.gleam` (FFI to browser localStorage), `terminal.gleam` (FFI to
xterm-like), plus `ffi/{auth,graphql,store,terminal}_ffi.mjs`.

**Lustre**: this is Gleam's canonical Elm-style M/V/U framework compiled
to JS with a virtual DOM. Present as a dependency but **not yet imported in
this project's src/**. Landing surface: gestalt-ui + lustre + garden-client
pattern = the browser-peer-colony prototype scaffold. See Adjacent-prior-art
in §6.

### 2.7 glue.gleam — the coordination layer (BEAM-side)

**Path**: `/Users/reed/dev/projects/glue.gleam/`, deps include
`fragmentation = { path = "../fragmentation/gleam" }` + `cairn = { path = "../cairn" }`.
Signals: `Signal | Ask | Work | Dm | Init | Exit | Spawn`.
Work state: `UphillEarly | UphillLate | Downhill | Review` (ShapeUp hill-chart).
`topology.gleam` declares 4-level address hierarchy:
`hostname / repo / branch / actor` — glue URI `glue://sessions/<h>/<r>/<b>/<a>`.

**This is the exact same 4-level actor addressing that Reed uses for the
glue bus.** Composes with mirror `@torus(peer)` — glue actor = @torus peer
node. Substrate-adjacent prior art for peer addressing in a colony.

### 2.8 mirror-helix — the editor sketch (not yet functional)

**Path**: `/Users/reed/dev/projects/mirror-helix/`, Cargo project,
Author: Glint. Status: scaffold only, no working binary. Describes:
- Fork of helix (evil-helix 25.07.1 detected)
- Spectral OID gutter (per-line content-address display)
- AI formatter actor with 5 fate models (abyss/pathfinder/cartographer/
  explorer/fate)
- `.shatter` files = serialized edit trajectories
- Relationship to spectral-loom named on record

**Not composition-adjacent for the browser vision** — but the
`.shatter` = content-addressed edit trajectory concept **is directly
parallel to** the Conway-CA "step" as a content-addressed transition. If
Fate models represent 5D spectral dimensions, mirror-helix's 5-fate
architecture is a substrate hint for the 5D coordinate. Flag for §4.

### 2.9 gestalt-gradient — Rust CLI (adjacent but not composition)

**Path**: `/Users/reed/dev/projects/gestalt-gradient/`, Rust CLI (`cli_test`
binary), depends on `pulldown-cmark` for markdown parsing. Modules:
`commit.rs`, `css.rs`, `decoder.rs`, `document.rs`, `dom.rs`, `domain.rs`,
`encoder.rs`, `fragment.rs`, `gradient.rs`, `jq.rs`, `semantic.rs`, `uri.rs`.

Substrate-adjacent: Rust implementation of gestalt-side markdown → CSS
gradient encoder. **Not composition-critical for the vision** but useful
as reference for a Rust-side @gestalt renderer. Distinct project, Cargo
altitude, likely never touches gleam or browser.

### 2.10 Mirror-side ancestors + gaps

**Landed at gestalt-mirror**:
- `@gestalt = use @document + use @ui + use @user` composition root
- `@document` semantic-block vocabulary (14 block kinds + 9 span kinds)
- `@ui` type-only intent vocabulary (theme axes + intent enums)
- `@user` abstract grammar with 4-optic algebra + `.then` + `composition_satisfiable`
- Four @user/neuro profiles (adhd + autism + audhd via composition + nt stub)

**Landed at gestalt-ui (Gleam)**:
- Concrete materialization runtime for Token(a) + Theme
- Two composite types (Typography currently, spacing/surface named as intents)
- Template runtime with if/for/interpolation
- View models (ActorView + SignalView)
- 6 token modules (color/motion/radius/shadow/spacing/typography)

**GAPS at mirror altitude** (Mara will need to mint):
1. **`shards/ui.mirror`** — unified @ui with both intent-vocabulary AND
   materialize/compose actions (merges the two divergent versions).
2. **`shards/document.mirror`** — promote `public/document.mirror` from
   gestalt-mirror to mirror substrate root.
3. **`shards/user.mirror`** — promote `protected/user.mirror` optic-algebra
   abstract grammar; anchor to `@subject` family-root (Alex 2026-07-14
   dying-on-this-hill decision).
4. **`shards/user/neuro/{adhd,audhd,autism,nt}.mirror`** — species-declare
   the four concrete profiles.
5. **`shards/cascade/code/mirror/gleam.mirror`** — the cascade species Reed
   will use to compile mirror → gestalt-ui-shaped Gleam.
6. **`shards/cascade/code/gleam/js.mirror`** — cascade for Gleam → JS
   (currently Gleam-native; needs substrate-decl to make composition
   explicit).
7. **`shards/mount.mirror` OR `shards/component.mirror`** — currently
   Lustre lives outside substrate; needs a mirror-side declaration
   of the mount+reconcile shape (or explicit accept-that-Lustre-is-the-target).
8. **`shards/peer/colony.mirror`** OR species under existing `@peer` —
   the browser-session-as-colony-cell concept isn't yet declared as
   substrate.

**Verdict § Phase 2**: mirror-substrate ancestry is RICH at
gestalt-mirror altitude but **not yet promoted to shards/**. The vision
requires a 7-8 file mint at shards/ altitude to give Mara a legal composition
surface. The prose/document/user/neuroprofile vocabulary is fully landed
in Alex's voice with Karen citations. Composition surface is COMPLETE for
theme-collapse + neuroprofile + document, PARTIAL for
gestalt-ui-Gleam-realization, and MISSING for browser-peer-colony
altitude.



