# @mirror/runtime/gen_prism — content-addressed actors

*2026-05-20. Reed.*

Mirror is one-shot per invocation. Mirror has no OS-level daemon. Yet mirror
needs process-like things — the MCP server holding session state, the LSP
server tracking open buffers, the auto-reload contract from `lsp-and-mcp.md`
remembering the last-emitted grammars hash.

This spec declares `@mirror/runtime/gen_prism`: the actor primitive whose
**identity is content-addressed** and whose **state lives in crystals**.
The store IS the process. There is no heap to retain. There is no thread to
schedule. There are crystals in git and refs that advance.

This is not the gen_prism of `spectral/docs/specs/spectral-spawn.md`. That
one has a tick loop, a Reflection model, a Fate router — it is autonomous.
This one is the substrate beneath. Spectral's gen_prism *uses* this. Most
things that want "a process" only need this.

---

## Thesis

A gen_prism is a name. The name points at a ref. The ref points at a crystal.
The crystal is the current state. Sending the gen_prism a message reads its
crystal, computes a new crystal, and advances the ref. The ancestor chain IS
the history. The ref IS the identity. The crystal IS the state.

No heap. No process. No supervisor. Liveness is the existence of the ref.
A tick is a function: `(state, message) → (state, emissions)`. Pure. Replayable.
Reproducible.

---

## What runs today

Nothing. `@mirror/runtime` exists at `boot/std/mirror/runtime.mirror` — a
thin compose of `@craft` + `@mirror/interpreter` + `@mirror/resolve`. There
is no actor primitive in the grammar set. Closing this gap is the spec.

---

## The primitive

```mirror
in @prism
in @mirror/spectral
in @hash/coincidence
in @beam
in @io

grammar @mirror/runtime/gen_prism {

  # --- types ---------------------------------------------------------------

  # the identity. a ref in refs/gen_prism/<name>. the head OID at that ref
  # is the current state crystal.
  type gen_prism = {
    name: text,           # human-readable identifier
    ref:  text,           # refs/gen_prism/<name>
    head: oid,            # the current state crystal
    tick: u64,            # number of messages applied (length of ancestor chain)
  }

  # a message is any content-addressable value. crystals all the way down.
  # the receiver decides what to do with it.
  type message = {
    oid:  oid,            # content address of the message body
    kind: text,           # discriminator the receiver pattern-matches on
  }

  # the result of applying a message to a state. the new state replaces
  # the head; the emissions are notifications/beams pushed on the active
  # transport (LSP/MCP stdio, the bus, stdout).
  type tick_result = {
    state:      oid,            # new state crystal (the new head)
    emissions:  [json],         # JSON-RPC notifications or beams
    loss:       loss,           # observation: did the tick improve coherence?
  }

  # --- operations ----------------------------------------------------------

  # create a new gen_prism. fails if refs/gen_prism/<name> already exists.
  # writes the initial state as a crystal and updates the ref.
  spawn(name: text, initial_state: oid) -> gen_prism { \ }

  # the pure tick. takes a state crystal and a message; returns the next
  # state + emissions. resolves to grammar-defined behaviour at the \ —
  # each concrete gen_prism overrides this in its own grammar.
  tick(state: oid, message: message) -> tick_result { \ }

  # fire-and-forget. read head, apply tick, advance ref. emit any emissions.
  # CAS-safe: the ref update fails if head moved since we read it; the
  # caller retries.
  send(gp: gen_prism, message: message) -> imperfect {
    tick(gp.head, message)
      |> @mirror/spectral.crystallize
      |> @io.git_update_ref(gp.ref)
  }

  # synchronous send. the caller blocks until the new state is durable.
  # reply lives in the new state's body — the caller extracts it.
  call(gp: gen_prism, message: message) -> imperfect { \ }

  # walk the ancestor chain. each entry is one tick's state crystal.
  # history(gp, 10) returns the last 10 ticks newest-first.
  history(gp: gen_prism, depth: u64) -> [oid] {
    @io.git_log(gp.ref, depth)
  }

  # read the current state without advancing.
  observe(gp: gen_prism) -> oid { gp.head }

  # terminate: delete the ref. the crystals remain in git (history is
  # never lost; the gen_prism just ceases to exist as a live identity).
  terminate(gp: gen_prism) -> imperfect {
    @io.git_delete_ref(gp.ref)
  }
}
```

Eight types and operations. Three concrete bodies (`send`, `history`,
`observe`, `terminate`). Four `\` holes (`spawn`, `tick`, `call`, plus the
 message-body schema each concrete gen_prism declares). The grammar is the
contract; specific gen_prisms close their own holes.

---

## How a tick happens (no OS process)

The host process is `mirror serve --mcp` (or `--lsp`). It runs while the
client is connected. Inside the session, every incoming JSON-RPC request
is routed to a gen_prism and triggers a tick:

```
Client sends:    {"jsonrpc":"2.0","method":"tools/list","id":1}
                      |
mirror serve     parse request, identify target gen_prism
                      |
                 send(@mirror/reload, message{kind: "tools/list"})
                      |
                 - read head crystal (refs/gen_prism/mcp_reload → oid)
                 - call tick(head, message)
                 - tick returns: new state crystal + notification emission
                 - write new crystal via git hash-object -w
                 - advance ref via git update-ref
                      |
mirror serve     emit response on stdout, push notification(s) on stdout
                      |
Client receives: response + tools/list_changed notification
```

Between messages, nothing runs. No timer. No thread. No memory. The host
process blocks on `read(stdin)`. When the next message arrives, the next
tick happens.

Between sessions, nothing runs either. The crystals in git are durable. When
a new `mirror serve --mcp` starts, the first tick reads the prior head and
resumes exactly where the last session left off.

---

## Examples

### Example 1: `@mirror/reload` as a gen_prism

From `lsp-and-mcp.md`: every MCP/LSP request must check whether the live
grammars hash has drifted since last notification, and push
`tools/list_changed` if so.

As a gen_prism, the state crystal records `last_emitted_hash`. Every
incoming request — *any* request, not just `tools/list` — triggers a tick:

```mirror
in @mirror/runtime/gen_prism
in @mcp
in @hash/coincidence

grammar @mirror/reload {
  # the state crystal carries one field: the hash we last told the client about.
  type state = {
    last_emitted_hash: oid,
  }

  # the message is any incoming MCP/LSP request — we don't care about content,
  # we only care that *some* request arrived (the trigger).
  type message = {
    method: text,
  }

  # the tick: compute current hash, compare to stored, emit if drifted.
  tick(state: oid, message: message) -> tick_result {
    let current = @mcp.grammars_hash;
    let prior   = @mirror/spectral.recall(state).last_emitted_hash;

    if current == prior {
      tick_result { state, emissions: [], loss: 0.0 }
    } else {
      tick_result {
        state: @mirror/spectral.crystallize(state { last_emitted_hash: current }),
        emissions: [
          json { method: "notifications/tools/list_changed", params: {} },
        ],
        loss: 0.0
      }
    }
  }
}
```

No watcher. No daemon. No spectral dependency. The trigger is the next
message on the wire. Drift is detected piggy-back on traffic the client
was already sending.

### Example 2: LSP document state

An LSP server tracks one buffer per open document. Each buffer becomes a
gen_prism named after the file URI. `textDocument/didOpen` spawns it;
`textDocument/didChange` sends an edit message; `textDocument/didClose`
terminates.

```
refs/gen_prism/lsp/buffer/file:///alex/foo.mirror → head crystal
                                                    │
                                                    ├─ ast: <oid>
                                                    ├─ cursor: { line, col }
                                                    └─ version: 7
```

The buffer's state is a crystal. Edits are messages. The history (every
version the user typed through) is the ancestor chain. "Undo" is `git reset`
on the ref. "Time-travel debugging" is walking the ancestor chain.

### Example 3: MCP session

A single Claude Code session is a gen_prism. Tool calls are messages.
The state crystal carries whatever the session needs to remember between
calls — a working directory, a current branch, a transient cache.

When the user starts a new session, mirror reads the prior crystal and
resumes context. Killing the session terminates the ref; the history
remains in git for replay or audit.

---

## Relationship to `@spectral/spawn`

| Concern | `@mirror/runtime/gen_prism` | `@spectral/spawn` |
|---|---|---|
| Identity | content-addressed ref | content-addressed ref |
| State | crystal at the ref | crystal at the ref |
| Trigger | incoming message on host transport | autonomous tick loop |
| Decision | grammar-defined `tick(state, msg)` | observe → think → decide via Reflection + Fate |
| Liveness | one OS process during session, dormant between | continuous (BEAM actor) |
| Use case | reload, LSP buffers, MCP sessions, mirror-internal state | autonomous agents that think |

The relationship: `@spectral/spawn`'s gen_prism *inherits* from
`@mirror/runtime/gen_prism`. It adds the `think`/`decide` lambdas and the
autonomous tick scheduler. The persistence pattern is shared. The activity
model differs.

If a gen_prism only needs to react to messages, `@mirror/runtime/gen_prism`
is enough. If it needs to think between messages, shift to `@spectral/spawn`.

---

## Failure modes

**Crash mid-tick.** The new crystal is written via `git hash-object -w` (atomic
on POSIX). The ref update is `git update-ref` (atomic). If the process dies
between the two, the crystal is dangling in git — reachable for replay or
garbage collection, never lost. The ref still points at the prior head.
Replay-safe by construction.

**Concurrent writers.** Two sessions both running `mirror serve --mcp` against
the same gen_prism is rare but possible. `git update-ref --create-reflog -m
msg ref new old` performs compare-and-swap: the update fails if the ref moved
since we read it. The caller's `send` lambda catches the failure, re-reads
head, re-ticks, retries. Last writer wins among contending updates; both
their ticks are visible in the reflog if a transcript is needed.

**Lost notification.** JSON-RPC notifications are best-effort over an active
stdio. If the client disconnects between tick completion and the notification
write, the notification is lost. The state is durable; the next session's
first tick re-checks drift and emits the notification fresh. There is no
"missed update" — just a slightly delayed one.

**State corruption.** A buggy `tick` body could write an unparseable state
crystal. The next tick fails to read it. Recovery: `git update-ref ref ref^`
rolls back one tick. The ancestor chain is the audit log.

---

## What stays in the host process

One OS process per session: `mirror serve --mcp` or `mirror serve --lsp`.
Its entire life:

```
loop {
  request = parse(read(stdin));
  target  = route(request);   // which gen_prism gets this?
  result  = send(target, request);
  write(stdout, result.response);
  for n in result.emissions {
    write(stdout, n);
  }
}
```

No state in the loop. The loop is the runtime. The gen_prisms are inside it.

---

## What stays in spectral

Spectral's daemon does NOT go away. It holds:

- Cross-session, cross-tool orchestration (Reed daemon, the glue bus, the
  4-level hierarchy `hostname:repo:branch:actor`).
- Long-running coordination across multiple `mirror serve` invocations.
- The autonomous tick loop for `@spectral/spawn` gen_prisms — those need
  a heartbeat, not just message-arrival triggers.

What moves out of spectral's scope: the auto-reload watch loop. That was
the original motivation for a daemon dependency, and `@mirror/runtime/gen_prism`
absorbs it cleanly.

Boundary summary:

| Layer | Owns |
|---|---|
| `@mirror/runtime/gen_prism` | the actor primitive: state in crystals, refs as identity |
| `mirror serve` | the host process: stdio transport, routing, message loop |
| `@mirror/reload` (gen_prism) | the reload contract; ticks on every request |
| `@spectral/spawn` | autonomous gen_prisms with their own heartbeat |
| spectral daemon | cross-session orchestration, the bus, the autonomous heartbeat |

---

## What this implies

Ordered follow-ups:

1. **Create `boot/std/mirror/runtime/gen_prism.mirror`.** The grammar above
   with its types and lambdas. `tick`, `spawn`, `call` are `\` holes; the
   concrete bodies (`send`, `history`, `observe`, `terminate`) compose
   `@io.git_*` operations.

2. **Refactor `@mirror/reload` as a gen_prism.** The spec in `lsp-and-mcp.md`
   described an abstract "reload contract." That contract IS
   `@mirror/reload tick(state, message)`. Update the LSP/MCP spec to point
   at gen_prism for the implementation pattern. Remove the spectral-daemon
   dependency from that spec's auto-reload section.

3. **LSP document state as gen_prisms.** When the bootstrap gains `mirror
   serve --lsp`, each open buffer becomes a gen_prism. The path mapping
   (`file:///x → refs/gen_prism/lsp/buffer/x`) gives a stable identity.

4. **MCP session as gen_prism.** Optional but clean: the session itself
   gets a ref. Tool calls advance the chain.

5. **Garbage collection.** Long-lived refs accumulate. Spec a TTL or a
   `mirror gc --gen-prism` command that prunes refs not touched in N days.
   Crystals are pruned by `git gc` on their own schedule.

6. **`@io.git_*` action set.** The gen_prism grammar leans on
   `@io.git_update_ref`, `@io.git_log`, `@io.git_delete_ref`. If those
   aren't declared in `@io` today, declare them. They join `git` as the
   content store and `@io` as the syscall boundary.

---

## Out of scope for this spec

- The autonomous tick loop (Reflection-driven, Fate-routed). That is
  `@spectral/spawn`'s territory. Mirror's gen_prism is reactive only.
- Distributed gen_prisms across hosts. The ref convention assumes one git
  repo. Cross-host coordination is spectral's bus + push/pull semantics.
- Strong typing of message bodies via the grammar. Each concrete gen_prism
  declares its own `type message`; the primitive is duck-typed at this
  level. A future spec could thread message types through the typechecker.
- The bootstrap implementation of `mirror serve`. That binary work is the
  cluster that lands `serve --mcp` and `serve --lsp` as real subcommands.
  This spec declares the contract those subcommands invoke.

---

*The store IS the process.*
*The ref IS the identity.*
*The crystal IS the state.*
*The next message IS the tick.*

Apache-2.0.
