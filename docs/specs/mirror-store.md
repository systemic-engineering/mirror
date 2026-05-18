# @mirror/store — content-addressed storage as grammar

*2026-05-18. Reed + Alex.*

---

## The Contract

Three operations. Same interface. Different backends.

```mirror
in @prism

grammar @mirror/store {
  store(crystal) -> oid { \ }
  fetch(oid) -> imperfect { \ }
  exists(oid) -> bool { \ }
}

out store
out fetch
out exists
```

The `\` is the backend. Swap one `in` line, the entire storage changes.
The grammars above don't notice. The contract holds.

---

## The Backends

### @mirror/store/git — today

```mirror
in @mirror/store
in @io

grammar @mirror/store/git {
  store(crystal) -> oid { @io.exec("git", ["hash-object", "-w"]) }
  fetch(oid) -> imperfect { @io.exec("git", ["cat-file", "-p"]) }
  exists(oid) -> bool { @io.exec("git", ["cat-file", "-e"]) }
}
```

Git IS the store. Content-addressed. Deduplicated. Distributed via push/pull.
Every crystal IS a git object. Every OID IS a git SHA.
The lockfile IS the content address. No `flake.lock`. No `Cargo.lock`.
Same grammar, same OID, always.

### @mirror/store/nix — when mirror is a nix provider

```mirror
in @mirror/store
in @io

grammar @mirror/store/nix {
  store(crystal) -> oid { @io.exec("nix", ["store", "add"]) }
  fetch(oid) -> imperfect { @io.read("/nix/store/" + oid) }
  exists(oid) -> bool { @io.stat("/nix/store/" + oid) }
}
```

The nix store IS a directory of content-addressed artifacts.
Mirror produces content-addressed artifacts.
The OID maps to the store path.
mirror.spec IS flake.nix.
`in` IS dependency declaration.
`mirror craft` IS `nix build`.

### @mirror/store/spectral-db — the endgame

```mirror
in @mirror/store

grammar @mirror/store/spectral-db {
  # the graph IS the store.
  # no filesystem. no exec. the eigenvalues ARE the addresses.
  store(crystal) -> oid { refract(crystal) }
  fetch(oid) -> imperfect { focus(oid) }
  exists(oid) -> bool { split(oid) -> bool }
}
```

The five operations ARE the storage operations:
- store IS refract (settle, crystallize, compute OID)
- fetch IS focus (look closer, retrieve by address)
- exists IS split (enumerate, check membership)

The store and the compiler merge. Same object. Same graph. Same eigenvalues.
No filesystem. No git. No nix. The graph navigating itself.

---

## The Dependency Model

`in` IS the only dependency keyword.

```mirror
in @prism
in @code/kernel/arm64
in @other/lib
in @another/lib

spec @myapp {
  target binary <| @code/kernel/arm64 <| std
}
```

- `in` = import = dependency = door
- The compiler resolves `in` by finding the grammar, loading it, checking it exists
- That IS dependency resolution
- The `in` graph IS the lockfile (OIDs are deterministic)
- `mirror craft @myapp` walks the `in` graph, resolves bottom-up, content-addresses each step

No `flake.lock`. No `Cargo.lock`. No `package-lock.json`.
The lock IS the content address. Same grammar, same OID, always.

---

## mirror IS nix

```
nix build         = mirror craft
nix flake         = mirror.spec
nix store         = @mirror/store (git, nix, or spectral-db)
nix hash          = OID (CoincidenceHash)
nix derivation    = grammar (in → out, deterministic fiber)
nix develop       = mirror (the compiler IS the dev environment)
flake.lock        = the in graph (OIDs are deterministic)
```

Sub-Turing guarantees reproducibility.
Same input → same output. Always. By construction. Not by convention.

The flake.nix becomes:

```nix
{
  outputs = { self }: {
    packages.default = ./mirror;  # the 8KB binary
  };
}
```

No cargo. No cc. No buildInputs. Just the binary.
`nix build` copies 8KB. The compiler IS the build system.

---

## The Path

```
today:    @mirror/store/git     (git objects, refs/crystals/)
next:     @mirror/store/nix     (mirror as nix provider)
endgame:  @mirror/store/spectral-db (the graph IS the store)
```

Swap one `in` line at each step. The grammars above don't change.
The contract holds. The doors remain. The fiber changes. The wine changes.
The glass stays the same.
