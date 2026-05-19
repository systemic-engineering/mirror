# mirror <input> <mq> — the entire binary surface

*2026-05-19. Reed + Alex.*

---

## The Binary

```
mirror <input> <query>
```

Input is a path or stdin. Query is mq. Everything else is grammar.

```
mirror = @io.read(input) |> @code/mq.parse(query) |> @mirror/execute |> @io.write(output)
```

Four operations. Read. Parse. Execute. Write. The rest is grammar.

## Subcommands ARE Queries

```bash
mirror boot/ "focus |> split |> zoom |> refract |> project"  # craft
mirror file.mirror "kintsugi"                                  # kintsugi
mirror file.mirror "run"                                       # run
mirror boot/ "kintsugi --shatter 0"                           # shatter
mirror /tmp/mirror.ll "@code/llvm/ir |> kintsugi"             # LLVM optimizer
mirror . "refract |> liquid"                                   # spectral.engineer
mirror stdin "@data/json.parse |> dispatch |> @data/json.emit" # MCP/LSP
```

No subcommands. The query IS the command.

## MCP and LSP ARE the Same Pattern

```
stdin |> @data/json.parse |> dispatch |> @data/json.emit |> stdout
```

The dispatch is the only difference:
- MCP: dispatch to mirror tools
- LSP: dispatch to language server operations

Both are: `mirror stdin <mq>`

## @mirror/lsp

```mirror
in @prism
in @mcp
in @code/mq

grammar @mirror/lsp {
  dispatch(request) -> response {
    @code/mq.parse(request.method) |> @mirror/execute |> response
  }
}
```

One grammar. The LSP. Same JSON-RPC transport as MCP.
Same stdin/stdout. Different dispatch. One grammar apart.

## λsh IS the CLI

```
λ> boot/ |> kintsugi --shatter 0
λ> /tmp/mirror.ll |> @code/llvm/ir |> kintsugi |> @code/kernel/arm64
λ> . |> refract |> liquid
λ> stdin |> @mcp.dispatch
```

The lambda shell IS the CLI IS the binary IS `mirror <input> <mq>`.
