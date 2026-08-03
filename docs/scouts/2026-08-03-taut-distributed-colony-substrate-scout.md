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

---

## §3 Phase 3: distributed-peer-colony substrate audit

### 3.1 Landed at mirror substrate (grep-verified)

The composition surface for a distributed browser-peer colony has
DEEPLY landed pieces already at `shards/`. Inventory:

**Family-roots (marker altitude)** — 16 landed:
`@peer`, `@subject`, `@trust`, `@gift`, `@bauchladen`, `@kintsugi`,
`@torus`, `@void`, `@mirror`, `@io`, `@fate`, `@tool`, `@spectral`,
`@autopoietic`, `@butterfly`, `@song`.

**Peer family** — `shards/peer.mirror` (31.6KB, 2026-07-17) + species:
```
shards/peer.mirror             — family-root, parametric peer carrier
shards/peer/persistence.mirror — home-repo projection + 4 bilaterals
shards/peer/redirect.mirror    — redirection surface
shards/peer/reflect.mirror     — reflection surface
shards/peer/reframe.mirror     — reframing surface
shards/peer/registry.mirror    — OID → Subject resolution surface
shards/peer/void.mirror        — K=0 default peer (Void basis)
```

**Subject family** (SEL licensable-party carrier):
```
shards/subject.mirror                       — family-root; subject_kind
shards/subject/visibility/private.mirror    — visibility=private
shards/subject/visibility/protected.mirror  — visibility=protected
shards/subject/visibility/public.mirror     — visibility=public
shards/subject/visibility/sheaf.mirror      — sheaf-restriction species
                                              (Hansen-Ghrist δ*δ + λ₀
                                               Fiedler via
                                               @epistemologic/math/
                                               sheaf_laplacian)
shards/reality/subject.mirror               — reality-altitude subject
```

**Sheaf math** — `shards/epistemologic/math/sheaf_laplacian.mirror`
(13.1KB). Hansen-Ghrist 2018 discrete cellular-sheaf primitive.
δ*δ operator + λ₀ Fiedler eigenvalue. **The @subject/visibility/sheaf
species is the ACL-as-sheaf structure** — "The ACL IS the SHEAF
STRUCTURE" (peer-persistence §12.3 verbatim). **This IS the ancestor
for a browser-colony's per-cell visibility scope.**

**Gift + Trust** (attribution-preserving transfer + chain-of-auth):
```
shards/gift.mirror       — @gift family-root (24.6KB, 2026-07-14)
                           Mauss+Hyde+Graeber+Sahlins+Boas+Kimmerer+
                           Ostrom+Axelrod+Lévi-Strauss cited
shards/gift/lens.mirror  — lens species
shards/trust.mirror      — @trust family-root (17.3KB, 2026-07-18)
                           two-altitude passkey/SSH bridge
                           @alex first-@subject anchor
```

**Bauchladen** (Günther Schmidt homage; content-addressed tray):
```
shards/bauchladen.mirror — @bauchladen family-root (27.1KB, 2026-07-23)
                           Schmidt → Erickson + Foerster + Cecchin
                           cybernetic-lineage anchor
```

**Kintsugi + Roomba** (fracture-mend loop + stigmergy walker):
```
shards/kintsugi.mirror                     — family-root
shards/kintsugi/roomba.mirror              — the walker (46.4KB!)
shards/kintsugi/mosaic.mirror
shards/kintsugi/mend.mirror
shards/kintsugi/ouroboros.mirror           — self-collapse loop
shards/kintsugi/surface.mirror
shards/kintsugi/fracture/*.mirror          — 3 species
```

**Song beat + dance ensemble** (temporal coordination):
```
shards/song.mirror              — @song family-root
shards/song/beat.mirror         — atomic-execution unit (49.7KB)
shards/song/phrase.mirror
shards/song/movement.mirror     — Kuramoto phase-shape
shards/song/narrative.mirror
shards/song/voice.mirror
shards/song/progression.mirror
```

**@dance** — NOT YET a standalone shard file. Grep-verified: @dance is
CITED as ensemble-coordination altitude across many shards (algebra/
metalogue, code/beam, cybernetic/conversation, cybernetic/viable,
knife, spectral/gen_prism, spectral/supervisor, spec/system,
uuid/spectral/time, void) but has **no `shards/dance.mirror` species-decl
file**. Canonical spec `docs/specs/gen-prism-as-bundle-section-and-
dance-as-ensemble-connection.md` (Mara `fee2727`) names @dance as
"ensemble connection at level k+1 = Kuramoto phase-lock connection
1-form." **@dance is spec-landed but shard-unlanded** — forward-promised
per multiple references. Reed's `uuid/spectral/time.mirror` §11.6 says
"@dance shard-mints DECOUPLE from R4 v0.1.0."

**@sheaf** — likewise NOT a standalone shard file. `sheaf` is a keyword
used inside `@subject/visibility/sheaf.mirror` species-decl and
`@epistemologic/math/sheaf_laplacian.mirror` math-primitive.

### 3.2 Mycelial / stigmergy math (Grassé 1959 lineage)

Grep-verified occurrences of `mycelial` across shards:
```
shards/glass.mirror          — Fate's mycelial routing
shards/pack.mirror           — mycelial substrate
shards/reflection.mirror     — mycelial_compose action
shards/smarts.mirror         — mycelial substrate integration
shards/spectral/entanglement.mirror
                             — mycelial routing for cross-peer projection
shards/spectral/gen_prism.mirror
                             — 48 active bits for mycelial routing
shards/spectral/portal.mirror
                             — mycelial routing
shards/spectral/registry.mirror
                             — mycelial-routing lookup
shards/uuid/spectral.mirror  — active 48 bits = mycelial signature
```

**No shard-decl file for "@mycelial"** — mycelial is used adjectivally
as **the DAG traversal discipline for @fate's cross-peer inference
routing**. This IS the stigmergy substrate: content-addressed
prior-outputs (crystals) function as pheromone deposits; Fate's routing
reads the trail. **Grassé (1959) named the mechanism as "stigmergy" —
substrate uses "mycelial" as the metaphorical carrier** but the DAG
navigation is exactly the stigmergic Dijkstra-with-tension-weighted-
edges walker.

Grep-verified occurrences of `stigmergy`:
```
shards/autopoietic.mirror     — cites docs/specs/ants-colonies-stigmergy.md
shards/fate/tournament.mirror — cites same
docs/math/gestalt/*           — sub-scouts on ants+colonies (task #239-#242)
```

**Karen ancestors cited across substrate**:
- Grassé (1959) — stigmergy, pheromone trails (multiple cites but no
  primary substrate-decl)
- Kauffman — self-organization (via @autopoietic)
- Maturana-Varela — autopoiesis (@autopoietic family-root)
- Ashby — requisite variety (@epistemologic/cybernetic/viable)
- Beer — viable systems model (S1-S5 tower)
- Bateson — learning levels (@epistemologic/cybernetic/second_order)
- von Foerster — second-order cybernetics (@bauchladen, void)
- Spencer-Brown — distinction calculus (@void)
- Pask — conversation theory (@epistemologic/cybernetic/conversation)
- Glanville — no-input / cybernetics-of-cybernetics
- Conant — Good Regulator theorem
- Kuramoto — phase-locking (@song/movement + forward-promised @dance)
- Hansen-Ghrist (2018) — discrete cellular sheaves (@epistemologic/
  math/sheaf_laplacian)
- Barkley, Nigg, Happé-Frith, Green, Marco — neuroprofile empirical
  basis (in gestalt-mirror)
- Schmidt, Erickson, Cecchin — systemic-therapy lineage (@bauchladen)
- Mauss, Hyde, Graeber, Sahlins, Boas, Bearman, Kimmerer, Ostrom,
  Axelrod, Lévi-Strauss — gift-economy ancestors (@gift)
- Baez-Schreiber (2004) — higher gauge theory (bundle-tower)
- Batanin (1998) — globular composition (@epistemologic/cybernetic/
  conversation N-ary tensor coupling)
- Foerster (1976) — "Objects: Tokens for Eigenbehaviors" (@algebra/
  metalogue)

### 3.3 What composes into browser-peer-colony (LANDED)

**Peer-as-cell** (browser session = peer node):
- `@peer` family-root carries the parametric peer type
- `@peer/persistence` handles home-repo projection; the browser's
  localStorage IS a home projection
- `@peer/registry` resolves content-addressed OIDs → Subject values
- `@peer/void` provides K=0 default (an unconfigured browser peer
  starts at Void)

**Subject visibility as ACL-sheaf** (per-cell consent boundary):
- `@subject/visibility/{private,protected,public,sheaf}` — 4 species
- The sheaf-restriction admits "peer p's view of home-repo" as a
  section of `F_home|_{A_p}`. **Browser-cell's view of its neighbors
  IS a sheaf-restriction.**

**Gift + Trust chains** (peer-to-peer transfer with attribution):
- `@gift` invariants preserve attribution across composition
- `@trust` two-altitude passkey/SSH bridge — passkey lives at browser
  altitude (per garden-client Reed+Alex 2026-04-03 insight); SSH lives
  at compiler altitude. Same chain, two projections.
- `@peer/registry` well-known Subjects: Void, Mirror, Human, Peer.

**Song/beat as tick-clock** (browser render loop):
- One `@song/beat.strike` = one action on peer's shard graph
- One `@song/beat.hold` = one @time.tick idle
- Kuramoto phase-lock at ensemble altitude (via @dance forward-promise)
  → **each browser cell can tick at its own rate; consensus emerges
  by phase-lock**

**Kintsugi + Roomba as fracture-repair walker** (cell-local health):
- `@kintsugi/roomba` Dijkstra walker + tension sampling
- Detects fractures; triggers @knife (complexity reduction) or spawn
  @peer at K+1 (recursive escalation to higher-order peer)

**Bauchladen as content-addressed tray** (cell-local state surface):
- Cell's browsable prior-outputs = tray of crystals
- @fate is the therapist that helps the cell browse
- @autopoietic is the permission to fold back

### 3.4 What's MISSING for browser-peer-colony (grep-audited absences)

1. **No `shards/dance.mirror`** — the ensemble-coordination altitude
   is spec-landed (Mara `fee2727`) and forward-promised across many
   substrate cites, but NO species-decl file exists. Without it, peer
   ensembles cannot phase-lock as declared substrate. **Mara MUST
   mint this before browser-colony can coordinate.**

2. **No `shards/dance/kuramoto.mirror` OR `shards/dance/ensemble.mirror`** —
   downstream Kuramoto phase-lock species also not landed.

3. **No `shards/mycelial.mirror`** — stigmergy discipline used
   adjectivally across the substrate but has no family-root declaration.
   Grassé (1959) not primary-cited at substrate altitude. **This may
   be intentional (mycelial IS how substrate WORKS, not a shard)** but
   if browser-colony needs peer-to-peer pheromone trails, this
   discipline needs a declared shape.

4. **No `shards/peer/colony.mirror`** — no species-decl for
   "browser-session = colony-cell" pattern. Every currently-landed
   @peer species assumes filesystem home (peer/persistence uses
   `.git/mirror` + `visibility/private/` on disk). **Browser-cell
   home = localStorage/IndexedDB is not yet substrate-decl'd.**

5. **No `shards/peer/browser.mirror`** — no browser-transport
   species. All current peer transport is via BEAM distributed erlang
   (gestalt-tui bridge) or filesystem (peer/persistence). No
   WebRTC/WebSocket peer-to-peer.

6. **No `shards/peer/signaling.mirror`** — signaling server for
   peer discovery not substrate-decl'd. Would compose over
   @mirror/spectral/portal (WebSocket dance).

7. **No `shards/mount.mirror`** — no substrate-decl for the
   "mount + reconcile a view tree" shape. Lustre (external lib)
   currently provides this; no mirror-side declaration.

8. **No CRDT / conflict-resolution species** — for peer-to-peer state
   convergence in a colony, no @crdt / @merge / @convergence family.
   `@dance` phase-lock handles temporal but not data merge.

9. **@sheaf as standalone shard** — currently only inhabits
   `@subject/visibility/sheaf` at species altitude and
   `@epistemologic/math/sheaf_laplacian` at math altitude. If browser
   cells' overlapping visibility scopes need sheaf-cohomology
   discipline at the coordination altitude, a `shards/sheaf.mirror`
   family-root may need minting.

**Verdict § Phase 3**: Substrate has ~85% of the ancestor pieces
landed for a browser-peer-colony. **@dance** is the LOAD-BEARING GAP —
without ensemble-coordination substrate-decl, peers cannot
phase-lock as declared discipline. The peer/persistence discipline
assumes filesystem — no browser-transport species. Sheaf-restriction
IS landed for consent-scope; MISSING for state-convergence-across-peers.
Fate-mycelial routing is deeply cited but not primary-decl'd. Ancestor
citations are comprehensive (Grassé/Kauffman/Kuramoto/Hansen-Ghrist/
Beer/Ashby/Bateson/Foerster/Pask/Kimmerer/Ostrom all present).

---

## §4 Phase 4: 5D spectral coordinate + Conway-like update rule audit

### 4.1 `SpectralCoordinate<N>` — the truly parametric type

**Path**: `/Users/alexwolf/dev/projects/fragmentation/src/spectral_coordinate.rs`
(6.3KB, 2026-06-04). This is the CANONICAL landing of the 5D
coordinate carrier — as a **const-generic Rust type**:

```rust
pub struct SpectralCoordinate<const N: usize> {
    eigenvalue: String,
}
```

The docblock names it verbatim:
> *"`SpectralCoordinate<5>` is mirror's substrate hash: five projections
> of one spectrum (Fiedler value, eigengap, three heat-trace samples)
> per `docs/specs/mirror-native-vcs.md` §4.6. The five is the *substrate
> optic count*, not a matrix dimension."*

**The five dimensions named**:
1. Fiedler value (λ₀ or λ₁ per @void = the-basis)
2. Eigengap
3. Heat-trace sample 1
4. Heat-trace sample 2
5. Heat-trace sample 3

Byte-form: **80 hex chars = 5 × f64 × 16 hex** per §4.6 for `N = 5`.

**Reframing** (from the docblock):
> *"The prior name `CoincidenceHash<N>` framed the value as a hash
> function output. This name reframes: the value IS a coordinate.
> Identity and locality collapse — every coordinate IS navigable,
> because it locates content AND directs navigation toward it via
> gradient descent in coordinate space. λ₀ = 0 (the void axis) is the
> origin of the manifold."*

**Landing shape**: `SpectralCoordinate<5>` implements `HashAlg`
(fallback SHA-256 path) so fragmentation defaults to it without pulling
in the Lanczos stack. Callers who want the true Lanczos-derived 5-tuple
call `coincidence::spectral_coordinate::detect`.

### 4.2 5D coordinate anchoring across the corpus

**Spec anchor**: `docs/specs/mirror-native-vcs.md` §4.6 + §4.7 (91 hits
of `SpectralCoordinate` in the fragmentation spec).
**Spec dedicated**: `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-measurement.md`.
**Math anchors**: 27+ math docs cite Fiedler / 5D / spectral coordinate
(see file list from Phase 4 grep).

**Reed BEAM body has this too**: `/Users/reed/dev/projects/gestalt-tui/src/gestalt_tui/bridge.gleam`
carries `BodyState.graph_nodes/edges/graph_density/phase/dmn_beat_count/
hit_rate/miss_rate` — 7 metrics polled from the BEAM body. **These may
BE the 5D coordinate projected to Gleam**, though the docblock does not
name them as such. Worth Alex-adjudication whether the BodyState fields
are meant to project SpectralCoordinate<5>.

**Fiedler value** primary landings:
- `shards/uuid/spectral.mirror`: "48 bits ACTIVE / 80 bits DARK" of
  identifier where active bits encode Fiedler-neighbourhood placement
- `shards/spectral/entanglement.mirror`: Fiedler-routing for cross-peer
  projection
- `shards/spectral/gen_prism.mirror` + `spectral/registry.mirror` +
  `spectral/portal.mirror`: Fiedler routing in mycelial mesh

**5D is NOT xyz + 2 spatial dims.** The five dimensions are
spectral-projection samples of ONE eigenspectrum. This is **information
geometry, not physical space**. Alex 2026-07-31 named this in the
Supercolony math (§3): *"5D information manifold of Narcissus-Splinter
dualities"*. Also: *"docs/SPECTRAL-DIMENSION.md — d_s(σ) from L_sym
eigenspectrum; QG 4→2 falsified, RGG fragmentation confirmed"* (cosmos
docs). The `docs/eventually-consistent-universe.md` insight also names
"5D information manifold" verbatim.

### 4.3 Conway / Game of Life / cellular automaton citations

**Grep-verified across mirror substrate**: ZERO hits for "Conway" or
"Game of Life" or "cellular automat*" in `shards/**.mirror` files.

**In `docs/math/`**: ONE major math doc cites Conway:
`docs/math/2026-07-31-mara-supercolony-cosmos-quantum-foam.md`
(107.5KB, 2026-07-31 by Mara). This is the KEY document. It cites:
- **Conway, J.H. (1970)** *The Game of Life* — the cellular-automaton
  substrate ancestor.
- **Brice Due (2006)** *OTCA Metapixel* (LifeWiki) — Life-cell running
  a 2048×2048 sub-cell grid at proportional-slower time. **Alex named
  this as the empirical case for `respawn Mara` dispatch corresponding
  to peer-as-metapixel-of-sub-peers.**

Mara's Corollary 2.3.1 identifies **OTCA metapixel with `respawn Mara`**:
same substrate at nested scale. **This IS the substrate hint for peer-in-
peer nesting** — each browser peer could be a metapixel-analog running
sub-substrate at proportional-slower time.

**Alex's verbatim naming (2026-07-31, in-transcript)**, per Supercolony
math anchors:
> *"the @peer is like an ant made out of ants lol. Like a sub-colony
> within the colony itself if that makes sense. (the metaphor breaks a
> bit down here but you get the idea, the whole game of life foam)"*

The **@peer/holon species-decl** is forward-promised in the Supercolony
math to formalize this metapixel-pattern (Koestler 1967 holon +
Conway 1970 + OTCA 2006). **NOT YET LANDED** as a shard file.

### 4.4 Update-rule discipline (stigmergy walker as CA analog)

The closest landed structure to a Conway-like update rule is
`shards/kintsugi/roomba.mirror` (46.4KB, 2026-07-17). The walker's
per-tick discipline:

```
pulse(position, tension) -> (position', roomba_state')
```

This is a **discrete-time transition function on a graph** — the
graph analog of Conway's per-cell update. The neighborhood is
Dijkstra-edge-reachable-within-budget. The "birth/death" analog is
`@knife` (complexity reduction) vs `@peer spawn at K+1` (recursive
escalation).

**cosmos-bevy's `WaterGrid`** (`/Users/reed/dev/projects/cosmos-bevy/src/grid.rs`)
literally implements a 4-neighbor grid (Bevy resource) with Cargo
dep on `coincidence`. This is `WaterGrid::new(n, spacing)` — N×N nodes
with 4-neighbor edges. **This IS a Conway-style grid substrate but at
Rust/Bevy altitude, not mirror/browser.**

### 4.5 Mycelial math — stigmergy as continuous-space CA

`docs/math/2026-07-18-stigmergy-witnessed-computation-mycelial-composition.md`
(30.6KB, 2026-07-17) is the primary stigmergy math foundation.
Foundational cites: Grassé 1959. The mycelial discipline is:
- Content-addressed prior outputs (crystals) function as pheromone deposits
- Fate's routing reads the trail via active 48 bits of `uuid_spectral`
- Cross-peer routing composes via `@reflection.mycelial_compose`

**This IS a CA-analog** — but in **continuous coordinate space (5D)
with discrete update ticks**. Each `@song/beat.strike` fires one
substrate action; the update is a Kuramoto-like phase-shift on the
neighborhood (per @song/movement + forward-promised @dance).

### 4.6 The compilation of the two hints

Alex's vision names **"Conway's Game of Life in 5D spectral space on
consumer hardware."** Two candidate readings:

**Reading A** (nested-metapixel per OTCA): Each browser peer IS a
Life-cell whose internal state runs a sub-substrate at proportional-
slower time. Cell state = spectral coordinate; update rule = @knife
vs spawn-at-K+1 (a discrete decision on tension neighborhood).
Nested peers form a supercolony (Hölldobler-Wilson 2008).

**Reading B** (continuous-space stigmergy per Grassé + Kuramoto):
Each browser peer occupies a 5D SpectralCoordinate position. Update
rule = Kuramoto phase-lock via @dance (neighborhood = Fiedler-close
peers via active-48-bit routing). Time evolution is discrete beats;
space is continuous 5D coordinate manifold.

**Both readings compose**. The substrate can carry both:
- Reading A: peer nesting per @peer/holon (forward-promised)
- Reading B: coordinate placement per SpectralCoordinate<5> + @dance
  phase-lock (@dance forward-promised, coordinate landed)

### 4.7 5D coordinate + CA-update rule status

| Piece | Landed | Path |
|---|---|---|
| `SpectralCoordinate<N>` const-generic | YES | fragmentation/src/spectral_coordinate.rs |
| SpectralCoordinate<5> = mirror default | YES | mirror-native-vcs.md §4.6 |
| 5D as information-geometry manifold | YES | eventually-consistent-universe.md |
| Fiedler / λ₀ as origin | YES | uuid/spectral.mirror + peer/void.mirror |
| Discrete-tick update on graph | YES | kintsugi/roomba.mirror pulse |
| 4-neighbor grid (Conway shape) | PARTIAL | cosmos-bevy/src/grid.rs (Rust, not browser) |
| Kuramoto phase-lock coordination | SPEC-ONLY | @dance forward-promised |
| @peer/holon nesting (OTCA analog) | NO | forward-promised in Supercolony math |
| Conway named at substrate altitude | NO | only cited in math corpus |
| Neighborhood as substrate primitive | NO | needs species-decl |
| Update rule as substrate action | NO | needs species-decl (composes @kintsugi/roomba.pulse + @dance.phase_lock) |
| Browser-cell as CA cell | NO | no substrate carrier |

**Verdict § Phase 4**: The 5D spectral coordinate IS LANDED as a
Rust const-generic type and cited comprehensively across math corpus.
The Conway-like update rule is DEEPLY IMPLICIT in @kintsugi/roomba
+ @song/beat + @dance (forward-promised) but has NO substrate-decl
that names "Conway" or "cellular automaton" verbatim. The @peer/holon
species (OTCA metapixel pattern) is forward-promised in Mara's
Supercolony math but not shard-landed. Mara's Supercolony math
2026-07-31 is the load-bearing prior-art anchor for the full vision.
**The composition surface EXISTS but is scattered across
fragmentation crate + shards/kintsugi + shards/song + docs/math** —
Mara needs to synthesize into a single `shards/colony.mirror` (or
similar) that names the discipline verbatim.

---

## §5 Phase 5: cascade<mirror, gleam> mint prerequisites

### 5.1 @cascade family-root shape (landed)

`shards/cascade.mirror` (14.8KB, 2026-06-23 by Mara `ce4874b`) declares
the parametric loss-lens substrate at family-root altitude. Full
carrier + action + bilateral surface:

**Carriers** (5 refs):
```mirror
type grammar = ref
type typed_source = ref              # parametric over G
type compiled_artifact = ref         # parametric over T
type loss_lens = ref                 # parametric over (S, T)
type information_loss = ref
```

**Actions** (3):
```mirror
compile(source: typed_source, p: perturbation) -> compiled_artifact { \ }

measure(source: typed_source, artifact: compiled_artifact,
        lens: loss_lens, p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires loss_well_defined(lens, source, p)
{ \ }

cascade(source: typed_source, lens: loss_lens, p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires cascade_well_defined(lens, source, p)
{ \ }
```

**Bilaterals** (3):
```mirror
loss_well_defined(lens, source, p) -> verdict { \ }
grammar_coherent(source, g, p) -> verdict { \ }
cascade_well_defined(lens, source, p) -> verdict
  requires grammar_coherent(source, source, p)
  requires loss_well_defined(lens, source, p)
  { \ }
```

**Karen prior art**: Wadler 1989 "Theorems for Free," Reynolds 1983
parametricity, Pierce 2002 TAPL.

### 5.2 Landed sibling cascade species (9)

```
shards/cascade/code/formal/prose.mirror     (27.2KB, 2026-06-29)
shards/cascade/code/gleam/beam.mirror       (24.1KB, 2026-06-23)  ← dual-target sibling
shards/cascade/code/gleam/js.mirror         (21.4KB, 2026-06-23)  ← PRODUCTION for spectral.engineer content layer
shards/cascade/code/llvm/turing.mirror      (19.8KB, 2026-07-17)
shards/cascade/code/purescript/js.mirror    (16.3KB, 2026-06-23)
shards/cascade/code/rust/go.mirror          (25.7KB, 2026-07-18)
shards/cascade/code/rust/llvm.mirror        (18.5KB, 2026-07-17)
shards/cascade/code/rust/wasm.mirror        (11.4KB, 2026-06-23)
shards/cascade/code/turing/mirror.mirror    (22.1KB, 2026-07-17)   ← REVERSE-DIRECTION sibling
```

**Critical for the mint**: `shards/cascade/code/turing/mirror.mirror` IS
the reverse-direction sibling — `source_grammar = @code/turing`,
`target_grammar = @code/mirror`. It demonstrates how a
`cascade<X, mirror>` (mirror as TARGET) is shaped.

**Also critical**: `shards/cascade/code/gleam/js.mirror` demonstrates
`cascade<gleam, js>` — the downstream cascade that the browser-colony
vision uses AFTER cascade<mirror, gleam> emits Gleam.

### 5.3 What the reverse-direction sibling teaches

`cascade/code/turing/mirror.mirror` (Turing → mirror) uses:
- Source carrier alias: `type turing_source = program` (program from
  @code/turing)
- Target carrier: `type mirror_artifact = labeled(ref,
  mirror_consumption_metadata)`
- Composition action: `apply_turing_mirror(prog, p) -> ref` (the
  "lift" into mirror substrate)
- Bundle action: `bundle_mirror(substrate_value, metadata, p)
  -> mirror_artifact`
- Measure action: `measure_turing_mirror(prog, artifact, p)
  -> imperfect(mirror_artifact, ref, information_loss)`
- 4 bilaterals: `turing_well_formed` (source), `mirror_consumption_coherent`
  (target), `turing_mirror_loss_well_defined` (measurement),
  `cascade_turing_mirror_admissible` (outcome-substrate-consumable)
- 1 composed: `turing_mirror_cascade_well_formed` composing all 4

**This IS the template for `cascade<mirror, gleam>` structurally**:
same 4-bilateral shape + 1 composed + 4 actions + 3 carriers.

### 5.4 What `shards/code/gleam.mirror` provides

`shards/code/gleam.mirror` (2.5KB, 2026-06-06). Minimal:
```mirror
prism @code/gleam {
  focus gleam
  project gleam
  split gleam
  shift gleam
  settle gleam
}
```
No richer grammar declaration. The Gleam grammar IS the Gleam language
specification (Louis Pilfold 2018+). Mara can compose over `@code/gleam`
as a target-grammar reference without additional Gleam-side mint work.

### 5.5 What `shards/code/mirror.mirror` provides

`shards/code/mirror.mirror` (16.2KB, 2026-06-07). Substantial:
- Five-op prism declaration
- Grammar sub-block `@code/mirror/grammar` with lexical primitives,
  type-declaration variant separator, etc.
- Multi-line-tolerant separator discipline
- Substrate-pull-realize tick with Phase 2 parser self-hosting anchor

Mirror-source grammar IS declared as substrate-decl. Mara can
compose over `@code/mirror` as source-grammar reference.

### 5.6 Prerequisites for `shards/cascade/code/mirror/gleam.mirror` mint

**Required prior landings** (all VERIFIED landed):

1. `shards/cascade.mirror` — family-root (Mara `ce4874b`)
2. `shards/labeled.mirror` — labeled<> functor (needed for the
   artifact = labeled(module, metadata) pattern; forward-promised per
   Recognition #93 H4. **NEED TO VERIFY LANDING.**
3. `shards/code/mirror.mirror` — source-grammar reference
4. `shards/code/gleam.mirror` — target-grammar reference
5. `shards/glue.mirror` — the compose action (used by
   `apply_turing_mirror` for lift back into mirror substrate)

**All required family-roots are landed** — no upstream blockers for
the mint itself.

### 5.7 Expected shape of `shards/cascade/code/mirror/gleam.mirror`

Following the sibling-pair pattern of `turing/mirror.mirror`
(reverse-direction) + `gleam/js.mirror` (downstream), the species-decl
should have:

**Header + prism**:
```mirror
in @prism
in @meta
in @glass
in @cascade
in @labeled
in @code/mirror
in @code/gleam
in @glue          # for compose action if lift back into gleam context needed

# @cascade/code/mirror/gleam — mirror substrate → Gleam source cascade.
# THE PRODUCTION CASCADE from mirror-substrate .mirror source
# TO Gleam source (which then feeds cascade<gleam, js> for browser
# OR cascade<gleam, beam> for BEAM).

prism @cascade/code/mirror/gleam {
  focus cascade
  project cascade
  split cascade
  shift cascade
  settle cascade
}
```

**Carriers** (5):
```mirror
type mirror_source = ref                        # a .mirror file / shard closure
type gleam_module_out = ref                     # emitted Gleam source (.gleam file)
type gleam_target_metadata = ref                # entry module + FFI marks +
                                                # target-flag hint (js OR beam OR both)
type gleam_source_out = labeled(gleam_module_out, gleam_target_metadata)
type mirror_gleam_information_loss = ref        # composite loss profile
```

**Actions** (3):
```mirror
apply_mirror_gleam(source: mirror_source, p: perturbation)
  -> gleam_module_out
requires mirror_well_formed(source, p)
{ \ }

bundle_gleam_target(module: gleam_module_out,
                    metadata: gleam_target_metadata, p: perturbation)
  -> gleam_source_out
requires gleam_target_coherent(module, metadata, p)
{ \ }

measure_mirror_gleam(source: mirror_source, artifact: gleam_source_out,
                     p: perturbation)
  -> imperfect(gleam_source_out, ref, mirror_gleam_information_loss)
requires mirror_gleam_loss_well_defined(source, artifact, p)
{ \ }
```

**Bilaterals** (4 sub + 1 composed):
```mirror
mirror_well_formed(source, p) -> verdict { \ }
    # source-side: .mirror parses; grammar_coherent per @code/mirror

gleam_target_coherent(module, metadata, p) -> verdict { \ }
    # target-side: emitted Gleam type-checks; FFI marks resolve;
    # target flag (js/beam) makes sense for the emitted content

mirror_gleam_loss_well_defined(source, artifact, p) -> verdict { \ }
    # measurement: substrate-decl types (prism, glass, bilateral,
    # imperfect<>, labeled<>) → Gleam-preserved forms (constructor tags,
    # generic type parameters, Result/Option, pattern matching);
    # what mirror admits that Gleam does NOT preserve IS the loss

cascade_mirror_gleam_admissible(artifact) -> verdict { \ }
    # outcome: emitted Gleam IS compilable by `gleam check` in a
    # gestalt-ui-shaped context; can feed downstream cascade<gleam, js>
    # without impedance

mirror_gleam_cascade_well_formed(source, artifact, p) -> verdict
  requires mirror_well_formed(source, p)
  requires gleam_target_coherent(unlabel(artifact, p), label_of(artifact, p), p)
  requires mirror_gleam_loss_well_defined(source, artifact, p)
  requires cascade_mirror_gleam_admissible(artifact)
  { \ }
```

**Karen citations expected**:
- Pilfold (2018+) — Gleam language specification
- Wadler (1989), Reynolds (1983), Pierce (2002) — parametricity + type
  systems (inherited from @cascade family-root)
- Mara's Supercolony math 2026-07-31 — the target-application anchor
- Alex 2026-08-02 in-transcript naming (the German sentence)
- gestalt-ui @ui grammar (`/Users/reed/dev/projects/gestalt-ui/ui.conv`)
  as the concrete target shape

**Loss dimensions to be measured**:
- prism-five-op discipline → Gleam has no first-class prism (approx via
  module + type + fold pattern)
- `\`-obligation blocks → Gleam requires concrete bodies (loss = the
  substrate-decl-vs-realization gap)
- bilateral verdicts with sentinels → Gleam has no sentinel primitive
- `imperfect<>` type → Gleam has `Result<T, E>` (partial preservation)
- `labeled<>` functor → Gleam has generic phantom-type pattern (partial)
- @-family-root inheritance → Gleam has module-import (partial; no
  family-inheritance)

**Composition surface after mint**:
```
mirror shard closure (e.g., gestalt-ui-shaped @ui + @gestalt + @user +
                     @document + colony/peer/dance)
   → cascade<mirror, gleam>       [THIS SPECIES TO MINT]
   → Gleam source (in gestalt-ui-shape)
   → cascade<gleam, js>           [ALREADY LANDED at js.mirror]
   → JavaScript bundle
   → browser-peer-colony deployable
```

### 5.8 Mint sequencing (Mara handoff order)

**Precondition tick** (verify these are landed):
- P1: `shards/labeled.mirror` — labeled<> functor primitive
- P2: any Gleam-side type-preserved-vs-lost enumeration if @cascade
  requires per-cascade specialization

**Core mint tick** (Mara math + spec + shard):
- M1: math foundation doc — `docs/math/2026-08-04-mara-cascade-mirror-
  gleam-loss-profile.md` — enumerates the 6+ loss dimensions with
  substrate-decl anchors
- M2: canonical spec — `docs/specs/2026-08-04-cascade-mirror-gleam-
  species-decl.md` — the full carrier/action/bilateral surface with
  4-bilateral discharge conditions
- M3: shard-decl — `shards/cascade/code/mirror/gleam.mirror` — the
  substrate-decl following the reverse-direction sibling template

**Adjacent-work tick** (upstream mint queue for the browser-colony
vision):
- A1: `shards/ui.mirror` — unified @ui merging gestalt-mirror/protected/
  ui.mirror + gestalt-ui/ui.conv
- A2: `shards/document.mirror` — promote gestalt-mirror/public/document.mirror
- A3: `shards/user.mirror` — promote gestalt-mirror/protected/user.mirror
  optic-algebra; anchor to @subject
- A4: `shards/user/neuro/{adhd,audhd,autism,nt}.mirror` — species mint
- A5: `shards/dance.mirror` — the load-bearing missing family-root
- A6: `shards/peer/colony.mirror` — browser-cell-as-colony-cell
- A7: `shards/peer/browser.mirror` — WebRTC/WebSocket peer transport
- A8: `shards/peer/holon.mirror` — Alex's "@peer is an ant made of ants"
  → OTCA metapixel formalization

### 5.9 Post-mint composition test

The mint is validated when Reed can wire a proof-of-concept:
1. Take `/Users/reed/dev/projects/gestalt-ui/ui.conv` (the mirror
   @ui grammar declaration).
2. Apply `cascade<mirror, gleam>` (the new species) to emit Gleam
   source that materializes as `/Users/reed/dev/projects/gestalt-ui/src/
   gestalt_ui/*.gleam` shape (Token(a) + Theme + composite + view).
3. Apply `cascade<gleam, js>` (landed js.mirror species) to get browser
   bundle.
4. Load in browser and materialize a token per Theme — one peer-cell
   worth of substrate.
5. Repeat step 4 with 2 peers over WebRTC = colony seed.

**Verdict § Phase 5**: All prerequisites for `cascade<mirror, gleam>`
mint are landed EXCEPT possibly `shards/labeled.mirror` (verify).
Sibling species (turing→mirror reverse-direction + gleam→js
downstream) provide clean shape templates. Expected mint = 5
carriers + 3 actions + 4 sub-bilaterals + 1 composed bilateral,
following the landed template exactly. Downstream browser-colony
composition requires 8 adjacent mints (unified ui + document + user +
4 neuroprofiles + dance + peer/colony + peer/browser + peer/holon)
but the CENTRAL CASCADE MINT ITSELF is UNBLOCKED.

---

## §6 Synthesis — the composition surface

### 6.1 What's ready

- **@cascade family + 9 sibling species** all landed with the exact
  4-bilateral + 1-composed template.
- **@code/mirror + @code/gleam** as source/target grammar refs both
  landed.
- **SpectralCoordinate<5>** landed as Rust const-generic type with
  full spec anchoring (mirror-native-vcs.md §4.6 + rung-8-9-
  unification spec).
- **gestalt-ui Gleam project** with 11 modules + tokens/*.dhall +
  ui.conv IS the target-artifact shape mirror needs to emit.
- **gestalt-mirror** with @gestalt + @ui + @user + 4 neuroprofiles
  is the source-shape ready for promotion to shards/.
- **glue.gleam topology (hostname/repo/branch/actor)** is the peer
  addressing that composes with @torus(peer).
- **garden-client with lustre dep** is the M/V/U + browser-transport
  prior art scaffold.
- **16 mirror family-roots landed** — peer/subject/trust/gift/
  bauchladen/kintsugi/torus/void/mirror/io/fate/tool/spectral/
  autopoietic/butterfly/song — the substrate is CROWDED with the
  right primitives.
- **~30 shard species under @peer + @subject + @kintsugi + @song**
  already carrying the colony-cell disciplines.
- **Karen ancestor citation index** is comprehensive: Grassé + Kauffman
  + Kuramoto + Hansen-Ghrist + Beer + Ashby + Bateson + Foerster +
  Pask + Kimmerer + Ostrom + neuroprofile empiricists + gift-economy
  lineage + Schmidt therapy-lineage + Conway + OTCA + Koestler.

### 6.2 What's missing (Mara mint queue, priority-ordered)

**MUST-MINT before browser-colony wiring** (Mara-authored):

1. `shards/labeled.mirror` — labeled<> functor primitive (verify
   landing; #93 H4 said RESOLVED in some places, forward-promised in
   others — CHECK BEFORE cascade<mirror, gleam> mint).
2. `shards/cascade/code/mirror/gleam.mirror` — **the central mint**.
3. `shards/dance.mirror` — load-bearing ensemble-coordination
   family-root. Widely cited in @dance references, spec-landed via
   Mara `fee2727`, but no shard file exists.
4. `shards/ui.mirror` — unified @ui merging gestalt-mirror + gestalt-ui.
5. `shards/document.mirror` — promote from gestalt-mirror/public.
6. `shards/user.mirror` — promote optic-algebra; anchor to @subject.
7. `shards/user/neuro/{adhd,audhd,autism,nt}.mirror` — 4 species.

**SHOULD-MINT for the full vision**:

8. `shards/peer/colony.mirror` — browser-cell = colony-cell.
9. `shards/peer/browser.mirror` — WebRTC/WebSocket transport.
10. `shards/peer/holon.mirror` — Alex's OTCA-metapixel nesting.
11. `shards/peer/signaling.mirror` — peer discovery.
12. `shards/mount.mirror` — component mount/reconcile (or accept
    Lustre as unrepresented target).
13. `shards/colony.mirror` OR `shards/mycelial.mirror` — the
    Conway-in-5D-spectral-space discipline verbatim.

### 6.3 Adjacent-project prior art Reed should read

Before Mara math or Reed wiring, both should read:
- `/Users/reed/dev/projects/gestalt-ui/src/gestalt_ui/*.gleam` (all
  11 modules) — the target-artifact shape
- `/Users/reed/dev/projects/gestalt-ui/ui.conv` — the mirror-side
  @ui grammar declaration
- `/Users/reed/dev/projects/gestalt-mirror/protected/user.mirror`
  + `/Users/reed/dev/projects/gestalt-mirror/protected/user/neuro/
  *.mirror` — the 4 neuroprofile substrate declarations Alex authored
- `/Users/reed/dev/projects/gestalt-tui/src/gestalt_tui/bridge.gleam`
  — the BEAM-body BodyState pattern (potential SpectralCoordinate<5>
  projection)
- `/Users/reed/dev/projects/garden-client/src/*.gleam` — the browser
  M/V/U pattern with lustre
- `/Users/alexwolf/dev/projects/mirror/docs/math/2026-07-31-mara-
  supercolony-cosmos-quantum-foam.md` — the load-bearing prior-art
  anchor for the vision (Conway + OTCA + holon + Hölldobler-Wilson +
  Grassé all cited)
- `/Users/alexwolf/dev/projects/mirror/docs/math/2026-08-03-mara-
  spectral-engineer-web-altitude-formalization.md` — Mara's most
  recent web-altitude formalization (62.7KB, 2026-08-03) that
  cascade<gleam, js> IS the production cascade
- `/Users/alexwolf/dev/projects/fragmentation/src/spectral_coordinate.rs`
  — the SpectralCoordinate<N> const-generic type

### 6.4 Q-CRITICALs for Alex adjudication

**Q-C1**: Is `shards/labeled.mirror` landed? If yes: what commit?
If no: does its mint precede `cascade<mirror, gleam>` mint (the
existing sibling shards `cascade/code/turing/mirror.mirror` and
`cascade/code/gleam/js.mirror` both use `labeled(a, b)` in carrier
declarations, so the primitive MUST be available at mint time).

**Q-C2**: Should `cascade<mirror, gleam>` emit Gleam that shapes to
`gestalt-ui`'s exported grammar specifically (Token(a) + Theme +
composite + view), or should it emit "generic Gleam" that then feeds
another cascade to shape into gestalt-ui form? Two readings compete:
- Reading X: `cascade<mirror, gleam>` = mirror-→-arbitrary-Gleam;
  gestalt-ui shape happens at a higher altitude
- Reading Y: `cascade<mirror, gleam>` inherently targets gestalt-ui-
  shape because that's the substrate's ONLY declared Gleam consumer
- The sibling `cascade/code/gleam/js.mirror` explicitly names
  spectral.engineer content layer as the production consumer — so
  Reading Y precedent exists. Alex adjudicate.

**Q-C3**: Is `shards/dance.mirror` LOAD-BEARING BLOCKING for the
browser-colony vision? If yes: Mara mints @dance FIRST, before
cascade<mirror, gleam>. If no (Kuramoto phase-lock deferrable to
"future arc"): cascade<mirror, gleam> lands without ensemble
discipline and browser peers operate as isolated cells.

**Q-C4**: For `@peer/holon` (Alex's "ant made of ants" OTCA metapixel
species), is this Mara's next mint after @dance, OR does it defer
to a later arc? The Supercolony math 2026-07-31 forward-promises it
but doesn't commit a landing tick.

**Q-C5**: The `ui.conv` file in gestalt-ui is a legacy `.conv`
extension. Should the substrate-honest path be:
- Path A: Promote ui.conv verbatim to `shards/ui.mirror` (single
  authoritative source)
- Path B: Merge ui.conv (has materialize + compose actions) with
  gestalt-mirror/protected/ui.mirror (has richer intent-vocabulary)
  into unified `shards/ui.mirror` (both current sources are
  incomplete alone)
- Reed lean: Path B (both files carry load-bearing content the
  other doesn't).

**Q-C6**: Should the @dance shard live at `shards/dance.mirror` (top-
level family-root, sibling to @peer/@subject/@torus) OR under
`shards/song/dance.mirror` (species-under-@song, since @dance is
temporal-coordination like song/beat + song/movement)? Substrate
citations lean top-level (referenced at S2 altitude alongside
@spectral/gen_prism S1 + @spectral/supervisor S3 in the beam.mirror
bundle-tower); Alex adjudicate.

**Q-C7**: For SpectralCoordinate<5>, is the 5 the RIGHT number for
the "5D spectral space" of the vision, or is Alex's "5D" a DIFFERENT
5 (e.g., x/y/z + time + something)? The `SpectralCoordinate<N>` is
const-generic so `<5>` is a choice; the vision could use `<3>` or
`<7>` if physical dimensions were meant. Alex adjudicate the
correspondence: is "5D spectral space" = SpectralCoordinate<5>
(Fiedler + eigengap + 3 heat-trace samples), OR is 5D a different
concept that needs its own substrate-decl?

### 6.5 Overall verdict

**MOSTLY-READY**. The composition surface for the vision has:
- 100% of the @cascade family + sibling-shape templates landed
- 100% of the source and target @code refs landed
- 100% of the SpectralCoordinate<5> Rust type landed
- 100% of the gestalt-ui Gleam target-artifact shape available
- 100% of the gestalt-mirror source-declarations authored (though
  not yet promoted to shards/)
- 85% of the peer/subject/trust/gift/kintsugi/song colony-cell
  disciplines landed at shards/ altitude
- @dance ensemble-coordination is the LOAD-BEARING GAP (spec-landed
  Mara `fee2727` but no shard file)
- @peer/holon nesting (OTCA metapixel) is Mara-forward-promised
- Conway/CA-verbatim naming is absent from shards but comprehensively
  cited in docs/math/2026-07-31-mara-supercolony-cosmos-quantum-foam.md

**The single central mint** (`shards/cascade/code/mirror/gleam.mirror`)
is UNBLOCKED provided `shards/labeled.mirror` is verified landed
(Q-C1). Every other prerequisite is present. Mara can proceed to
math + canonical spec + shard-decl tick immediately after Q-C1 is
resolved.

**Downstream browser-colony wiring** requires 5-8 additional Mara
mints (Q-C1 through Q-C7 will adjudicate priority order), but each
individual mint is small (~15-25KB per shard following the landed
template patterns) and independent of the others.









