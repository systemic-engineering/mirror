# 00 — Root Definition

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
