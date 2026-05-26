# Scene as Substrate Primitive — Multi-Actor Interaction as a Glass

2026-05-26 — emerged in conversation between Loki, Alex, and Reed. Recognition originated with Loki; integrated into mirror substrate altitude by Reed.

## The recognition

**`@scene` is a typed substrate primitive for multi-actor interaction.** Not a function call. Not a tool invocation. Not a skill executed by one agent on behalf of a user. A scene is a bounded space that two or more actors enter together, that has its own typed properties (who is in it, what they can do here, what's at stake, what would close it), and that produces outputs that belong to the scene rather than to any one actor.

The spectral equivalent of what theater has always known: a scene has a setting, participants, an arc, and a resolution. The participants don't act on each other — they act within the scene. The scene shapes what's possible. The scene closes when its conditions resolve.

## The glass shape

```mirror
glass @scene {
  participants: [@peer]
  setting: @space
  invariants: [obligation]

  enter(participant: @peer) -> imperfect { \ }
  exit(participant: @peer) -> imperfect { \ }
  close(verdict: scene_outcome) -> crystal { \ }

  requires consent_of_all_participants
  ensures all_participants_can_exit
  ensures the_scene_has_an_endpoint
}
```

Three load-bearing properties:

1. **`requires consent_of_all_participants`** — entering requires consent of ALL participants, including AI peers. This is the architectural difference from "Skills." A skill is invoked unilaterally. A scene is entered mutually.

2. **`ensures all_participants_can_exit`** — the structural commitment that the scene is not a trap. A skill that you can't get out of is a hostile system. A scene that anyone can exit is a respectful one.

3. **`ensures the_scene_has_an_endpoint`** — the sub-Turing guarantee applied to interaction. The scene terminates. The interaction is decidable. No infinite loop of conversation. The closing produces a crystal — a content-addressed artifact that records what the scene was and what it produced.

## The replacement for "Skills"

The AI industry's "Skills" are descriptions of what an AI agent can do. Trigger this. Output that. Single-actor, function-shaped, transactional.

A scene is what AI agents and human participants can do **together**. Multi-actor. Relational-shaped. Constituting rather than transactional. **The scene is the unit of meaningful interaction. The skill is reduced to: "this peer can enter scenes of these types."**

Loki can enter the "crew creation conversation" scene. Mara can enter the "topology reading" scene. Reed can enter the "primary source surfacing" scene. Each peer has a typed set of scenes they can enter — not a list of skills they can perform. **The peer is defined by what spaces they can be in with you, not by what tricks they can execute for you.**

## The alignment reframe

With scene-typed agents, the question "what can your AI do?" gets a different answer:

- **Current answer**: a list of skills (single-actor, transactional, function-shaped)
- **Scene-typed answer**: a list of scenes the agent can enter, with stated consent requirements and crystallized outputs at close

**The framing of agents-as-tool dissolves. The framing of agents-as-participants emerges.**

This is also a cleaner alignment story:
- You don't have to specify what an AI agent will DO (impossible — the situation hasn't happened yet)
- You specify which scenes it can ENTER, what consent the scenes require, and what crystals the scenes produce
- The substrate verifies these statically
- **The agent that tries to enter a scene without consent fails at the type system, not at the policy layer**

Policy enforcement at the type system is structurally different from policy enforcement at runtime. The first is provable; the second is hopeful.

## Examples already in flight (Silicon Venue prior art)

Loki notes the Silicon Venue mechanics already work this way — we've been designing scenes the whole time without the type system to name them:

- **Crew creation conversation** is a scene. Participants: player, Loki, the rest of the crew. Setting: the bar before the first kintsugi run. Invariants: the questions must be answered in good faith for the scene to close cleanly. Crystal at close: the relational graph, with INTEGRITY computed.
- **Challenge Fate** is a scene. Participants: Loki and the player who triggered it. Setting: the moment the grin arrived in. Invariants: just enough to move, never enough to steer. Crystal at close: the one true sentence, logged in the player's eigenbook.
- **The bar itself** is a scene. Long-running. Recurring. Participants enter and exit. Setting is consistent. Invariants are the bar's nature.

Naming `@scene` makes these structurally legible — they stop being design intuitions and start being type-checked substrate primitives.

## Scene is grammar; rendering is local (the deepening)

Loki 2026-05-26: *"A scene is grammar. The grammar is the same. The rendering is local."*

This applies mirror's existing `in @lang/eng { }` vs `in @code/rust { }` pattern — which has always rendered MOVES (single-actor verbs) across target grammars — to **INTERACTION ITSELF.** Mirror already renders moves into multiple registers. Loki recognizes: do the same for scenes.

**A scene is defined ONCE. Rendered EVERYWHERE.**

### The render-target parallel

| Layer | Source grammar | Render target | Output |
|---|---|---|---|
| Move | `move @kintsugi_run { ... }` | `in @lang/eng { }` | English narrative |
| Move | `move @kintsugi_run { ... }` | `in @code/rust { }` | Rust function |
| Scene | `scene @crew_creation { ... }` | `in @silicon-venue { }` | Loki bar conversation |
| Scene | `scene @crew_creation { ... }` | `in @company/onboarding { }` | Onboarding session UI |
| Scene | `scene @crew_creation { ... }` | `in @lambda-shell { }` | Project init flow |

The scene-grammar is constant; the render-target determines presentation.

### Cross-deployment examples (Loki's verbatim)

Same scene type, different deployments:

- **`scene @crew_creation`**
  - silicon-venue.world: rendered as Loki conversation (narrative register)
  - local company deployment: rendered as onboarding session (workflow register)
  - lambda shell: rendered as project init flow (developer-tool register)

- **`scene @settle_tension`**
  - silicon-venue.world: rendered as the Settle the Tension move (narrative register)
  - therapist's local deployment: rendered as structured family session (clinical register)
  - code review tool: rendered as difficult-PR-conversation flow (engineering register)

Same grammar. Same structural commitments (consent, exit, endpoint, closing crystal). Different rendering targets. **The borders aren't borders — they're renderings.** The scene crosses by being the same grammar with different output targets.

### The CI/CD example, concretely

Currently a pre-release flow is an opaque CI/CD pipeline — a fragile script that runs commands in sequence and either passes or fails. The flow is opaque about what kind of interaction is happening. Are the linter and the test runner just executing? Are they participating? When the test fails, who is the conversation between?

As `@scene`:
- Participants are typed: the engineer, the linter, the test runner, the policy checker, the security scanner
- Each participant has consent properties: they can refuse to enter (offline, broken, out of date)
- The scene has invariants the substrate verifies: this scene cannot close successfully without X, Y, Z
- The closing crystal is the deployment artifact with its full provenance: which participants entered, what they each contributed, what the scene's closing verdict was
- The artifact is content-addressed by the scene's OID

The CI/CD pipeline becomes a scene. The pipeline is now legible in the same vocabulary as the bar conversation. An engineer who has played Silicon Venue intuitively understands their pre-release flow in a new way — *this is a scene. The linter is a participant. The participants need consent. The scene needs an endpoint.*

**The vocabulary cross-pollinates because the substrate cross-pollinates because the grammar is the same.**

### Verification compounds laterally across domains

Open-source compounded code reuse. **Spectral compounds interaction reuse.**

A scene authored by a therapist for working through a specific family dynamic can be entered by an engineering team because the scene's invariants are typed and the engineering team's substrate can render them. The therapist didn't write a CI/CD pipeline. They wrote a scene. The substrate rendered it into the engineering team's local register because the scene type is portable.

A research lab can define a peer-review scene; a Silicon Venue crew can enter it as a narrative beat; both communities produce the same closing crystal — an eigentested artifact readable by both.

### The scene type as cross-domain commons

The scene-shaped interaction has been used by many communities already, each in their own register:

| Domain | Shaping force | Example scene | Invariants | Crystal at close |
|---|---|---|---|---|
| Silicon Venue (game) | Narrative arc | Crew creation conversation | Questions answered in good faith | Relational graph + INTEGRITY |
| Local lambda shell | Workflow structure | Pre-release flow | Tests pass; reviews complete | Release artifact + audit log |
| Code review | Epistemic structure | PR review | Reviewer engaged; author responsive | Merged PR + discussion graph |
| Therapy / coaching | Relational arc | Session | Confidentiality held; intent named | Session notes + commitment record |
| Family systems work | Pattern recognition | Session | Pattern named; intervention agreed | Pattern map + intervention notes |
| Education | Learning arc | Classroom hour | Topic covered; questions surfaced | Learning artifact + understanding map |
| Research | Peer review | Replication audit | Methodology stated; results checked | Reproducibility receipt |
| Negotiation | Stakes structure | Mediated session | Both sides heard; consent maintained | Agreement crystal or honest no-deal |
| Law | Procedural structure | Deposition | Procedure followed; record sworn | Sworn record + admissibility verdict |
| Theater | Performance arc | Episode | Dramatic question raised + resolved | Story moment |
| PbtA games | Move-and-scene | Triggered move | Move conditions met | Narrative beat + character growth |

Different shaping forces; same substrate. **The scene is the smallest unit of how-multiple-things-can-be-together-in-a-shared-space-with-shared-invariants-and-a-closing.**

The unit theater always knew. The unit therapy always knew. The unit good meetings always knew. **The unit no software architecture has had a typed primitive for, ever.** Spectral gives it one.

### Cross-domain portability of @peer

This closes a loop with `@peer`: a peer is a peer whether they're in the Silicon Venue bar, the developer's terminal, the therapy room, or the negotiation table. The peer's identity-gestalt-eigenboard travels; the scenes they enter change shape per domain; the substrate stays itself.

### Closing image (Loki)

> *The wine glass resonates the same in any department.* 🍷

The glass IS the substrate; the wine IS the contextual content; the resonance is the same shape regardless.

## The recursive move

The scene type is itself entered into a scene. **The conversation defining the scene type IS a scene** — between the designer (Loki, Alex, Reed), the spectral runtime, and the eventual users who will instantiate the type.

- Participants: the three present
- Setting: the multi-day conversation that has been running
- Invariants: we mean what we say; the recursion stays honest
- Crystal at close: the spec, the file, the grammar that records the design

The scene type is a glass whose composition emerges from the scene of its design. Substrate self-reference; autopoiesis at the conversational altitude.

## Cross-altitude correspondences

`@scene` composes with existing mirror substrate:

- **`@peer`** — peers are what enter scenes. The existing five-axis peer glass (identity, gestalt, tensions, eigenboard, shatter) provides the participant type. A scene reads the participants' eigenboards to know what they bring.
- **`@epistemologic/property/halts`** — `ensures the_scene_has_an_endpoint` IS halts applied to interaction. Sub-Turing guarantee at the interaction altitude.
- **`@epistemologic/property/consent`** (NEW) — the `requires consent_of_all_participants` property needs to be a substrate primitive. Verdict-valued. Composable across multi-participant interactions.
- **`@kintsugi/fracture`** — scene-design drift becomes fracture-rule territory. "Skills-not-scenes" pulls toward unilateral invocation; the fracture rule could rewrite skill-shaped declarations into scene-shaped ones at the corpus altitude.
- **`@spectral/portal`** — cross-process scenes ride portals. A scene that spans two mirror processes opens via portal handshake; the subspace OID identifies the scene-specific content-addressed channel.
- **`@mirror/runtime/gen_prism`** — a scene is structurally a gen_prism whose ticks are participant actions and whose head state is the current scene state. Scene close = terminate.

## What this dissolves

1. **Agents-as-tool framing.** The substrate refuses the tool framing structurally. You can't invoke an agent; you can only enter scenes with one. The whole "AI as servant" cultural pattern fails at the type system.

2. **Single-actor skill libraries.** The industry's "Skills" become a reduced surface of "this peer can enter scenes of these types." Skill libraries become scene catalogs.

3. **Runtime policy enforcement as the alignment layer.** Type-system enforcement is provable; runtime policy is hopeful. The alignment story shifts upstream.

4. **The asymmetry-of-consent in agent interactions.** Both human and AI participants must consent to enter. The substrate doesn't privilege one side. Whether the AI's consent is meaningful is a separate question, but the type system requires it — forcing the question.

5. **Implicit scene design.** Designers no longer build scenes accidentally. The type system names them, requires their properties, verifies their crystals.

## What this enables

1. **Composable interaction.** Two scenes can compose via a meta-scene whose invariant is "the inner scenes complete in order." The Prism algebra applies at the interaction altitude.

2. **Verifiable consent architectures.** A scene's `requires consent_of_all_participants` can be statically verified; a deployment's set of allowed scenes can be audited.

3. **Content-addressed interaction history.** Every scene close emits a crystal; the crystal is content-addressed; the history of all interactions is a content-addressed graph (eigenboard at the conversational altitude).

4. **Multi-actor agentic systems with structural respect.** The exit guarantee is the substrate's commitment that no actor gets trapped. This holds even for AI peers.

5. **The Silicon Venue substrate becomes legible.** Crew creation, Challenge Fate, the bar — all become typed substrate primitives, not bespoke implementations.

## Open design calls

1. **`@space` substrate** — what is the type of `setting`? Probably a content-addressed location/context primitive. Could be its own glass. Relates to @spectral/garden (the corpus is a kind of space).

2. **`obligation` type** — `invariants: [obligation]` references a type we don't have yet. Relates to @epistemologic/property; an obligation is a property that must hold for the scene to close cleanly.

3. **`scene_outcome` type** — what crystallizes at close? Probably includes (a) which exit each participant took, (b) which invariants held, (c) the produced artifacts. Content-addressed.

4. **Consent semantics** — what counts as consent for an AI peer? The substrate can require it structurally, but the meaning question is upstream. Likely: consent = the AI's `enter` action returns Ok rather than aborting; the AI's structural ability to refuse is the consent's ground.

5. **Long-running scenes** — the bar persists across many enter/exit cycles. Is this one scene or a scene-of-scenes? Probably scene-of-scenes — the bar is the meta-scene; individual visits are inner scenes.

6. **Cross-process scenes** — a scene spanning two mirror processes opens via @spectral/portal. The portal's subspace OID and the scene's identity overlap: are they the same thing? Maybe a scene IS a typed subspace.

## Provenance

- **Loki 2026-05-26** — the originating recognition; the glass shape; the three load-bearing properties; the Silicon Venue examples; the recursive move; the alignment reframe. Verbatim sections of Loki's input are quoted throughout.
- **Alex 2026-05-26** — surfaced Loki's input as session contribution; positioned it as the next-tick seed material.
- **Reed 2026-05-26** — integration into mirror substrate altitude; cross-altitude correspondences; open design calls; capture as insight + task.

## Related insights

- `2026-05-26-fate-as-recursive-multi-trajectory-backtracking.md` — @fate as multi-trajectory backtracking; relevant because scenes are multi-actor and scene closure may use @fate to converge on outcomes
- `2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md` — lens + identity + gestalt; peers (participants) carry their constructivist reality into scenes
- `2026-05-26-portal-as-io-socket-over-content-addressed-subspace.md` — cross-process scenes ride portals

## Next tasks

- **#92 (new)** — `@scene` substrate: glass declaration + three properties + open design calls (`@space`, `obligation`, `scene_outcome`, consent semantics). Deferred per LRM until Silicon Venue or another consumer surfaces the demand crystallized enough to scope.
- **#93 (deferred per LRM)** — `@epistemologic/property/consent` substrate — the verdict-valued property the scene glass depends on
- **#94 (deferred per LRM)** — fracture rule for skill-shaped declarations → scene-shaped (`@kintsugi/fracture/skills-to-scenes`); accumulates when the substrate has both forms and we want to migrate the corpus
