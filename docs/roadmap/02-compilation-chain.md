# 02 — Compilation Chain

## Action Visibility

Grammar actions support `public`, `protected`, and `private` modifiers.
Bare `action` defaults to `protected` (backwards compatible).

- **`public`** — direct return `{ok, Args}`, no gen_server round-trip
- **`protected`** — `gen_server:call` through domain server (current default)
- **`private`** — same dispatch as protected, but not exported from BEAM module

Visibility stored in TypeRegistry, emitted as `visibility/0` function on
compiled modules. Private actions filtered from export list.

---

## Traced Compilation Chain

Each compilation phase produces a linked trace with parent OIDs:

```
source → parse(Trace) → resolve(Trace) → compile(Trace) → CompiledDomain(Trace)
```

`compile_grammar_traced` NIF returns per-phase content OIDs. The compiler
actor chains them: parse_trace → resolve_trace → compile_trace → swap_trace.
Every running module has a verifiable chain back to source.

---

## Cairn Identity Hierarchy

`key.derive_child(root_pub, name)` — `sha512(root_pub || name) → Ed25519 seed`.
Anyone with the root public key can derive any actor's public key and verify
traces. Flat derivation (`sha512(name)`) preserved for backwards compatibility.

Compiler supports both modes: `start()` (flat) and `start_with_root(key)` (hierarchical).
