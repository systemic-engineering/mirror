# 01 — Architecture

## Three Roles

**@compiler** — owns `action compile`. Receives `.conv` source, calls the
Rust NIF to produce ETF, loads the BEAM module. Returns a witnessed trace.
Identity: `sha512("compiler")` → first 32 bytes → Ed25519 keypair.

In supervised mode (`start_named`), the compiler is pure — it compiles and
loads modules but does not start domain servers. The garden handles lifecycle.

In standalone mode (`start`), the compiler starts its own domain supervisor
and starts domain servers after compilation. This is the backwards-compatible
imperative path.

**Domain server** — GenServer registered as the domain atom. Receives
`{action, Args}` calls, dispatches to the compiled module's exported
functions. The `exec` primitive does `apply/3` — that's where grammar
meets reality.

**Supervision** — two layers:

- `conversation/supervisor.gleam`: static supervisor (RestForOne) managing
  @compiler + garden. If @compiler crashes, garden + all domains restart.
- `conversation/garden.gleam`: factory supervisor managing domain servers
  dynamically. If one domain crashes, only that domain restarts.

The old `conversation_sup.erl` (Erlang `simple_one_for_one`) is deprecated.
New code should use the Gleam supervision modules.

---

## Actor Registration

The @compiler actor starts via `compiler.start()` (standalone) or
`compiler.start_named(name)` (supervised).

Standalone path — `compiler.start()`:

1. Starts the domain supervisor (idempotent)
2. Derives the compiler's Ed25519 keypair from `sha512("compiler")` (first 32 bytes → seed)
3. Starts the OTP actor with that identity
4. On `CompileGrammar`: compiles, loads, starts domain server, returns trace

Supervised path — `compiler.start_named(name)`:

1. Derives the compiler's Ed25519 keypair from `sha512("compiler")` (first 32 bytes → seed)
2. Starts the OTP actor with that identity, registered under `name`
3. On `CompileGrammar`: compiles, loads, returns trace (no domain lifecycle)

Domain servers are started separately via `garden.start_domain(name, domain)`.

Compiled modules get the `conv_` prefix — `@erlang` compiles to
`conv_erlang`, avoiding BEAM sticky module collisions. The domain server
still registers as the raw domain atom for action dispatch.

---

## Domain Identity → NIF

Every domain has a deterministic identity derived from its name:

```
sha512(domain_name) → first 32 bytes → Ed25519 seed → keypair → Oid
```

This is the cairn pattern. The identity is the name. The name is the
address. No registry lookup. No UUID generation. If you know the domain
name, you can compute its public key.

The NIF boundary uses Rustler — no unsafe C layer. Two functions cross:

- `parse_conv(source)` → `{ok, oid} | {error, reason}`
- `compile_grammar(source)` → `{ok, etf_binary} | {error, reason}`

`compile_grammar` runs on a dirty CPU scheduler (`DirtyCpu`) because
compilation involves parsing, type resolution, and EAF emission — work
that shouldn't block the BEAM's normal schedulers.

---

## Bootstrap File

`bootstrap.conv` declares `@compiler` — the grammar that compiles grammars:

```
grammar @compiler {
  type = grammar | type

  action compile {
    source: grammar
  }
}
```

Two root types. One action. Everything else extends from this.

---

## Boot Sequence

Two paths. Same compilation loop, different lifecycle management.

### Imperative (standalone)

`boot.boot_from_files(paths)`:

1. Start the domain supervisor
2. Start the @compiler actor
3. Compile each grammar source through @compiler
4. For each compiled domain, query `lenses/0` and `extends/0`
5. Domain servers started inline during compilation
6. Return `BootResult` with compiler subject + booted domains

### Supervised (embedded)

`boot.supervised_boot_from_files(compiler_subject, garden_name, paths)`:

1. @compiler and garden are already running (started by a parent supervisor)
2. Compile each grammar source through the named @compiler
3. Start each domain server through `garden.start_domain(name, domain)`
4. Return `List(BootedDomain)` with domain names, lenses, and extends

The supervised path is used by Reed and any actor that embeds the conversation
supervision tree inside its own tree. The imperative path is for CLI and
standalone use.

After boot, `imports_resolved(result)` and `extends_resolved(result)`
verify that all lens dependencies and inheritance chains are satisfied.
A grammar that imports `@phantom` will boot but report unresolved imports.
