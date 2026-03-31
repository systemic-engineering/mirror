# Conversation — Roadmap

## Current Focus

Three crates. Three concerns. One content-addressed pipeline.

```
conversation    — proof + coordination  (BEAM)
coincidence     — numerics + compilation (Fortran)
fragmentation   — storage + addressing   (git)
```

Conversation compiles grammars to BEAM modules. When it needs to compute, it
calls through coincidence-core for the numerics. Fragmentation stores everything
as content-addressed git objects.

## Status

- 650 Rust tests, 100% line coverage
- 104 Gleam tests (compiler, boot, domain, garden, key, oid, supervisor, coincidence, properties, enforcement)
- 24 compile integration tests
- Pre-commit hook enforces all gates

## Milestones

| # | Milestone | Status |
|---|-----------|--------|
| [00](00-root.md) | Root definition | Done |
| [01](01-architecture.md) | Architecture | Done |
| [02](02-compilation-chain.md) | Compilation chain | Done |
| [03](03-shipping.md) | Shipping via fragmentation | In progress |
| [04](04-fortran-bridge.md) | Fortran bridge | In progress |
| [05](05-kandddinsky.md) | KanDDDinsky — October 2026 | Planned |
| [06](06-model-checker.md) | Model checker properties | Enforcement active |
| [07](07-projection.md) | Projection: properties as plans | Design |
| [08](08-oid-native-model.md) | OID-native model (experimental) | Idea |
