> **DEPRECATED-FOR-RUST-REWRITE (Mara 2026-07-17):** This spec
> describes bootstrap-era lambda-shell design that retires via the
> `@kintsugi/roomba` cascade 3 (bootstrap → rust). Terminal form
> at `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
> (Mara `2519f83`); `dance.rs` composes reflectively rather than
> booting a distinct shell surface. Preserved for archaeology.

# λsh — The Lambda Calculus Shell

*2026-05-07. Reed + Alex. The shell is the runtime.*
*2026-06-05 substrate-pull lift: `zoom`→`shift`; `refract`→`settle`; `~/.spectral/`→`~/.mirror/`. Moved from `spectral/docs/specs/` to mirror canonical home per the inline-spectral-into-mirror v0.1 framing.*
*2026-06-12 substrate-pull lift: entry verb `converse` → `join` → `sh`. `sh` is the substrate's noun for the shell manifold; UNIX's word for fifty years. §"Entry from the mirror CLI" rewritten.*

---

## Three Characters

```
λ>       # computing — every expression is a lambda on the graph
@name>   # conversing — a peer is present
\        # toggle — switch between computing and conversing
```

That's the entire interaction model.

---

## The Prompt

`λ>` — you're in the calculus. Every expression is an optic applied
to the graph. Every pipe is function composition. Every command
terminates — the grammar proves it.

`@reed>` — a peer is present. Natural language. The peer translates
to mq internally. You see the graph operations they perform.

`@>` — the unnamed peer. The shell itself. Suggests aliases.
Answers "what was I doing last week?" Maintains your config.spec.

---

## The Toggle

`\` goes to the home peer. Configured in the .spec:

```mirror
spec @mirror {
  peer = @reed
}

spec @systemic.engineering {
  peer = @glint
}
```

- `\` in mirror → `@reed>` (home peer from spec)
- `\` in systemic.engineering → `@glint>` (home peer from spec)
- `\` with no peer configured → `@>` (unnamed shell peer)
- `\@seam` anywhere → `@seam>` (explicit override)
- `\` from any peer → `λ>` (back to computing)

Peers persist in the session. Summoned peers see each other's
responses and see the graph operations you run between conversations.

---

## The Five Operations as Shell Primitives

```
focus                         # see what's here
project @code/rust            # filter by grammar
split imports                 # follow edges
shift blame                   # transport — same bytes, new declared shape
settle kintsugi               # the one write
```

Composition via `|>`:

```
λ> focus |> project @code/rust |> split imports |> shift blame
```

That's `(shift ∘ split ∘ project ∘ focus)(graph)`.
Lambda calculus. Pure function composition.

---

## mq IS the Shell Language

Not bash. Not python. Not a DSL wrapper around something else.
mirror query IS the language. The compiler validates at parse time.
Sub-Turing. Every expression terminates.

Tab completion is type checking. The graph tells you what
operations are valid on the current result. You can't pipe
nonsense because the geometry won't accept it.

---

## History as Graph

Every command is a node. Every result is a node. Pipes are edges.
The history has eigenvalues, not line numbers.

```
λ> shift history
  eigenvalue  command
  0.94        settle(simplify shards/mirror/cli.mirror)
  0.87        project @code/rust |> split imports
  0.12        project @nl |> where content ~ "old thing"
```

Frequent patterns have high eigenvalues. One-off commands fade.
Cross-reference by spectral distance, not string matching:

```
λ> shift history |> split(near: "loss calculation")
```

---

## The Unnamed Peer

The shell itself watches your usage. Measures command patterns.
When a pattern's eigenvalue crosses the threshold, it suggests:

```
λ> # suggestion: alias `ri` = focus |> project @code/rust |> split imports
   accept? [y/n/edit]
```

The suggestion IS the eigenvalue crossing. Measured, not heuristic.

The unnamed peer maintains `~/.mirror/config.spec`:

```mirror
spec @shell {
  alias ri = focus |> project @code/rust |> split imports
  alias fl = focus |> project(loss: true)
  alias k = settle kintsugi

  peer_arousal_threshold = 0.3
  history_eigenvalue_cutoff = 0.05
}
```

The spec writes itself from observation. Edit to override.
Defaults are measured, not guessed.

---

## The Eigenboard Prompt

The prompt color IS the eigenboard:

- Teal `λ>` — settled, idle
- Green `λ>` — curious, results flowing
- Gold `λ>` — engaged, high activity
- Pulsing orange `λ>` — drift warning
- The presence node from the spectral color mapping spec

You don't read the eigenboard. You see the prompt glow.

---

## Connection to Daemon

`λsh` connects to `~/.mirror/serve.sock`. All operations route
through the daemon. The graph is already loaded. The eigenboard
is already hot. The context is already mapped.

`λsh` is a client. The daemon is the runtime. The graph is the
server. Five operations over a Unix socket.

---

## Agent Spawn

Agents don't get worktrees. They get sub-graphs.

```
λ> \@seam fix the loss calculation
@seam> [spawns into the sub-graph neighborhood of "loss calculation"]
       [eigenboard loaded, context pre-mapped, Fate routing ready]
       [works on the graph, not on files]
       [settle produces a commit, the files are projections]
```

The agent's context window IS the sub-graph. Eigenvalue-ordered.
Not 200K tokens of flat text. The geometric projection of what's
relevant.

---

## Entry from the mirror CLI: λsh and `mirror sh`

`mirror sh` IS the **shell-open verb**. The standalone `λsh` binary is
a thin alias for `mirror sh` — same daemon socket, same algebra, same
eigenboard.

Bare `mirror sh` with no peer drops into λsh as self. `mirror sh
@reed` drops into λsh with `@reed>` as the active prompt. Both resolve
via `cli-as-prism.md` §7's default-op rule (the `sh` stage declares
`default settle`; settle on the shell manifold IS entering it). Other
ops on the shell manifold: `mirror sh focus @reed` observes peer's
eigenboard without entering; `mirror sh shift @reed` views from their
altitude; `mirror sh split @reed` branches a sub-conversation. Op-first,
peer-as-arg.

λsh is the **interactive transport** (the running mode); `mirror sh`
is the entry (the verb). They are not separate things — same lens
under two names. The shell IS a stage in the substrate (`stage
@mirror/lens/cli/sh`); the five ops apply to the shell manifold
(prompt state, view state, session state, peer context).

**Substrate-pull recognition (2026-06-12).** Rename arc `converse →
join → sh`. The `join` waypoint earned its keep by exposing the
type-confusion (`join` was both Glanville's directional entry verb AND
a five-op sub-stage; verbs don't have algebras, nouns can). `sh` is
a noun — the shell manifold — and the five ops apply cleanly. UNIX
has named this manifold `sh` for fifty years; the substrate already
had the word.

See `cli-as-prism.md` §2.1 (the `sh` stage) and §5.4 (the manifold
framing), and `the-convergence.md` §1.2 (transports table).

---

## Prior Art

- **Nushell**: structured data pipelines (tables) → we pipe graphs
- **PowerShell**: object pipeline → we pipe Beam<T>
- **jq**: "everything is a filter" → "everything is an optic"
- **Fish**: autosuggestions from history → suggestions from eigenvalues
- **Warp**: block model + AI → blocks carry eigenboard snapshots
- **Alacritty**: GPU rendering → eigenboard color gradients

Full survey: `spectral-shell.md` (in spectral; consolidation pending).

---

*λ> — computing.*
*@reed> — conversing.*
*\\ — toggling.*
*Three characters. The whole interface.*
