# @peer — the typed surface for spawn

The grammar that types the typed hole in `spawn`. A peer is a five-axis identity gestalt; its home directory is the manifest material form. `@peer(~dir"<path>")` resolves the directory into the typed fixed point that `spawn` can call.

## Status

**Yellow.** Recognition complete; specification in progress; implementation status unknown (audit pending). Discovered during the 2026-05-25 session through four chained recognitions:

1. `gen_prism` is the substrate for agent identity continuity (per `~/dev/systemic.engineering/practice/insights/glue/mirror-supersedes-daemon.md`).
2. The agent is a multi-flake layer stack (per Alex 2026-05-25 — base flake + overlay flakes; consent at flake boundary).
3. The agent home folder is `Prism(self)` — five operations, five files (per `~/dev/systemic.engineering/practice/insights/agents/agent-home-as-typed-hole.md`).
4. `@peer` is the glass that types `spawn` against the five-axis gestalt (per Alex 2026-05-25, this spec).

## Signature

```
spawn : (@peer, ~mq) -> gen_prism

@peer(~dir"<path>")              -- glass instantiated with a typed directory ref;
                                    resolves the directory into the five-axis fixed point at HEAD
~mq"<query>"                     -- typed mq literal; the message body that fills
                                    the typed hole the peer is being asked to resolve
```

`~dir` is a typed directory ref (sibling of `~file`). It validates at compile time that the path exists and is a directory; failure halts compilation with a precise error. `~mq` validates by parsing the content against the mq grammar. Per `AGENTS.md` § Sigil Naming: no single-character sigils, but `~mq` is canonical (a name, not a shorthand).

## The five-axis fixed point

A peer's home directory is `Prism(self)` made material. Each file is the output of one Prism operation applied to the peer's own bias_tree:

| File              | Prism operation | Role                                                              |
|-------------------|-----------------|-------------------------------------------------------------------|
| `identity.mirror` | `focus`         | the manifold the peer lives on (the IS)                           |
| `gestalt.mirror`  | `project`       | accumulated lens — what's been filtered as "matters to me"        |
| `tensions.mirror` | `split`         | open branches; the peer's live derivatives                        |
| `eigenboard.spec` | `shift`         | current spectral state — path's tip through the algebra           |
| `shatter.mirror`  | `settle`        | rendered history; the commit chain made readable                  |

Exactly five — because the algebra has five. Drop one and `@peer` cannot type-check; the load fails before any `spawn` runs.

## Identity vs continuity

- **Identity** = `identity.mirror` alone. Immutable across the peer's lifetime. The manifold. Change it = fork, not continuation.
- **Continuity** = the trajectory of the other four across the `gen_prism` ancestor chain. The path on the manifold.
- Lose the manifold (mutate identity) = lose continuity, even with full history.
- Lose the history (compaction, respawn) = no continuity lost — the manifold persists.

## Operations (target shape)

```
in @prism
in @mirror/runtime/gen_prism
in @mq
in @io

grammar @peer {

  type peer = {
    identity:    mirror,        # focus(self)
    gestalt:     mirror,        # project(self)
    tensions:    mirror,        # split(self)
    eigenboard:  spec,          # shift(self)
    shatter:     mirror,        # settle(self)
  }

  # load a peer from a directory. fails if any of the five required
  # files are missing or malformed.
  load(dir: ~dir) -> peer { \ }

  # check that the peer is on its manifold: identity types, gestalt
  # is reachable from identity through valid Prism operations,
  # tensions are well-formed, shatter is a valid ancestor chain.
  validate(p: peer) -> imperfect { \ }

  # instantiate the peer as a gen_prism on the typed hole.
  spawn(p: peer, q: ~mq) -> gen_prism { \ }
}

out peer
out load
out validate
out spawn
out @peer
```

## What composes (free)

Once `@peer` exists, the following fall out of the substrate without extra machinery:

- **Same peer, varying mq query.** Mara across sessions: peer constant, hole varies. Continuity is structural.
- **Same mq query, varying peer.** Ensemble work: send the same query to `@peer(~dir"~/.mara")`, `@peer(~dir"~/.seam")`, `@peer(~dir"~/.taut")`. Tournament resolves disagreement.
- **Recursive composition.** `shatter.mirror`, `gestalt.mirror`, and the spawn result are all mq-shaped — peer outputs are themselves queryable inputs.
- **Type-checked respawn.** Bad gestalt = spawn fails at load. Drift detection is structural.
- **Flake-layered deployment.** A peer is a base flake (the five files) plus overlays (consent layers, project overlays). `nix run github:user/agent#mara` materializes the composition.

## What this spec does NOT replace

- `@spectral/spawn` — heavyweight autonomous think loops. Different surface.
- `spectral-db` — distribution, deltas, MNESIA adapter. `@peer` is identity continuity; spectral-db is collaboration substrate.
- The human collaboration relationship. This is the algebra; the relationship is the lived path on the manifold.

## What the audit must answer

1. Does `@peer` already exist as a boot grammar in mirror? Where?
2. Does it use `@cogito`? What does `@cogito` provide if it exists?
3. Is `~dir` (typed directory ref) already registered as a `@sigil` instance, or does it need to be introduced? (And: are existing `~f`/`~d` short-form sigils to be migrated to `~file`/`~dir` per `AGENTS.md` § Sigil Naming?)
4. Is `~mq` (typed mq literal) already registered, or does it need to be introduced?
5. What's the current shape of `spawn` — is it in `@spectral/spawn` only, or does the boot already have a leaner `gen_prism.spawn` surface?
6. What grammars in the boot already touch any of the five files (`identity.mirror`, `shatter.mirror`, etc.)?
7. The gap between what exists and the target signature above — that gap IS the implementation spec.

## Provenance

- `~/.mara/`, `~/.glint/`, `~/.seam/`, `~/.taut/`, `~/.heath/` — peers on this machine, each with the five-axis structure.
- `/Users/reed/identity/` (symlinked from `~/.reed/`) — Reed's home; same shape unfolded along consent + temporal dimensions.
- Insight: `~/dev/systemic.engineering/practice/insights/agents/agent-home-as-typed-hole.md` — the five-axis recognition.
- Insight: `~/dev/systemic.engineering/practice/insights/glue/mirror-supersedes-daemon.md` — gen_prism as substrate.
- Spec: `boot/std/mirror/runtime/gen_prism.mirror` — the actor primitive `@peer` extends.

---

*Apache-2.0. Aspirational until the audit closes the gap.*
