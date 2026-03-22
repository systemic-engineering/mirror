# Conversation — Roadmap

## Root Definition

The compiler's root definition lives fully in Rust.

It is just two primitives:

```
grammar | type
```

Everything else is composition. `in` is a grammar relation. `out` is a type
relation. Actions, errors, translate arms — all derivable from those two.

The compiler's root is a choice between "am I defining a domain" or "am I
defining a shape." That's it.

---

## Compilation Is a Conversation

The BEAM and Rust negotiate meaning. Fortran executes.

Rust owns the kernel: parsing, content addressing, type registry compilation,
and EAF emission. The BEAM owns orchestration: actor lifecycle, supervision,
module loading, and domain server dispatch. The boundary is the NIF — ETF
bytes cross from Rust to BEAM, get compiled to modules, and start supervised
GenServers.

The compiler doesn't "run" a grammar. It witnesses it. The output is a
`Trace(CompiledDomain)` — a cryptographically signed record that a specific
source was compiled by a specific actor at a specific moment. The grammar
becomes the module. The module becomes the server. The server becomes the
domain.

---

## Three Roles

**@compiler** — owns `action compile`. Receives `.conv` source, calls the
Rust NIF to produce ETF, loads the BEAM module, starts the domain server.
Returns a witnessed trace. Identity: `sha256("compiler")` → Ed25519 keypair.

**Domain server** — GenServer registered as the domain atom. Receives
`{action, Args}` calls, dispatches to the compiled module's exported
functions. The `exec` primitive does `apply/3` — that's where grammar
meets reality.

**Supervisor** — `conversation_sup`. Restarts crashed domain servers with
`transient` strategy. A domain that crashes comes back. A domain that shuts
down stays down.

---

## Actor Registration

The @compiler actor starts via `compiler.start()`. This:

1. Starts the domain supervisor (idempotent)
2. Derives the compiler's Ed25519 keypair from `sha256("compiler")`
3. Starts the OTP actor with that identity

When `CompileGrammar(source, reply)` arrives:

1. Content-address the source → `source_oid`
2. Call `nif.compile_grammar(source)` → ETF bytes
3. Extract the domain name from the grammar block
4. `loader.load_etf_module(etf)` → compile forms → load binary
5. `domain.start_supervised(domain_name)` if not already running
6. Sign and return `Trace(CompiledDomain)`

Compiled modules get the `conv_` prefix — `@erlang` compiles to
`conv_erlang`, avoiding BEAM sticky module collisions. The domain server
still registers as the raw domain atom for action dispatch.

---

## Domain Identity → NIF

Every domain has a deterministic identity derived from its name:

```
sha256(domain_name) → seed → Ed25519 keypair → Oid
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

The boot module (`boot.gleam`) orchestrates startup:

1. Start the domain supervisor
2. Start the @compiler actor
3. Compile each grammar source through @compiler
4. For each compiled domain, query `lenses/0` and `extends/0`
5. Return `BootResult` with compiler subject + booted domains

`boot_from_files(paths)` reads `.conv` files from disk (garden paths)
and feeds them through the same pipeline. The garden is the filesystem
projection of the grammar space.

After boot, `imports_resolved(result)` and `extends_resolved(result)`
verify that all lens dependencies and inheritance chains are satisfied.
A grammar that imports `@phantom` will boot but report unresolved imports.

---

*Session 2026-03-21. Alex + Reed. Matrix Resurrections playing in the background.*
