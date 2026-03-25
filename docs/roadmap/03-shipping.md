# 03 — Shipping via Fragmentation

The compiler writes ETF blobs to git via fragmentation. The `frgmt` binary
(from the fragmentation crate) packages the result into a shippable artifact.

```
.conv source
  → @compiler (parse → resolve → compile → ETF)
  → fragmentation::git::write_tree + write_commit
  → refs/fragmentation/conversation/<module>
  → frgmt collapse <ref>
  → Nix derivation 1: escript compiles ETF → .beam
  → Nix derivation 2: OTP release packaging
  → /nix/store/... (shippable binary)
```

---

## The Bridge Is Git

fragmentation writes native git objects. Nix reads git repos via
`builtins.fetchGit`. No FUSE in the build path — the Nix sandbox does not
expose `/dev/fuse`, and this is a hard constraint. FUSE is the development
surface (inspect, navigate, diff). Git is the build surface.

---

## Two-Derivation Build

**Derivation 1:** An escript runs `binary_to_term → compile:forms → write .beam`.
Same three-step process as `loader_ffi:load_etf_module/1`, but ahead of time.

**Derivation 2:** Takes `.beam` files, adds `.app`, boot script, `sys.config`,
`vm.args`, optionally bundles ERTS. Standard OTP release via `relx` or
`mix release`.

---

## What the Compiler Writes

Each `CompileGrammar` call produces ETF containing EAF. The compiler writes
this to git via fragmentation, creating a commit on a ref like
`refs/fragmentation/conversation/<domain>`. The `frgmt collapse` command reads
the tree at that ref and invokes the flake.

---

## Runtime vs. AOT

Two loading paths coexist:
- **Runtime** (development): `loader_ffi.erl` does `binary_to_term →
  compile:forms → code:load_binary` live on the BEAM.
- **AOT** (`collapse`): escript does the same three steps during the Nix build,
  writes `.beam` files to disk, packages as release.

The compiler doesn't change. The consumer changes.

---

## Open Questions

- **ETF layout:** flat `etf/` directory per domain, or nested? Flat is simplest.
- **Ref convention:** mutable `latest` for dev, immutable `<source-hash>` for
  production builds, or both.
- **ETF version pinning:** compiler and Nix build must use the same Erlang
  version. Pin in the flake.
- **Gleam interop:** Gleam runtime modules (`beam/`) need to be in the same
  release as compiler-produced modules. Built separately, combined in
  derivation 2.
