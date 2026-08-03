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

