# `@mirror/mosaic` — The Build System as a Glass Within @mirror

*2026-06-04. Reed + Alex. Spec.*

Status: **Red**

Depends on: `@prism` (the five operations), `@mirror` (the compiler),
`@property`/`@io` (the delegation contract).

Forward references: `au-and-conductivity.md` (au as the output type),
`mirror-store.md` (fragmentation as canonical), `kintsugi-ci-v0.1.md`
(kintsugi runs mosaic; mosaic settles; the loop is shared),
`mirror-spec-schema.md` (mirror.spec is mosaic's input).

---

## 1. Recognition

Mosaic is the substrate's build system.

Not a sibling to `@mirror`. Not its own top-level prism. **A glass within
`@mirror`** — declared via the `glass` keyword inside the compiler's own
prism. The substrate's way of saying: the build system is part of the
compiler's surface, behind the same wall.

A build is a settlement. The build manifold has the same five
operations as everything else — focus, project, split, shift, settle —
because every settlement in the substrate is a Prism. Mosaic IS those
five operations applied to `mirror.spec`. The verb is the same. The
manifold is what changes.

The analogy:
- `@mirror` settles a grammar's MirrorAST into a graph.
- `@mirror/mosaic` settles a project's `mirror.spec` into an artifact at
  an altitude (`au(@code/rust)`, `au(@release)`, …).

Same verb (settle). Same lattice (transparency over check properties).
Same proof obligation (eⁿ⁺¹ < eⁿ).

---

## 2. The Glass Declaration

Inside `shards/mirror.mirror` (or wherever `@mirror` declares its glass
wall):

```mirror
in @prism
in @meta
in @property
in @io

prism @mirror {
  # ... the compiler proper ...

  glass @mirror/mosaic {
    # The build manifold. Five operations on mirror.spec.
    focus   spec                    -> manifold
    project targets                 -> resolved
    split   shards                  -> [shard]
    shift   altitudes               -> emitter
    settle  emitter                 -> au
  }
}
```

`glass` declares mosaic as part of the compiler's transparent surface.
From outside, `@mirror/mosaic` is callable; from inside, mosaic has the
same access to `@mirror`'s grammar and AST machinery that the rest of
the compiler has. The glass wall is the published surface; the
declaration says: mosaic is on the inside, looking out the same window.

The five lines under `glass @mirror/mosaic` are the operation
signatures. Each maps onto the build manifold.

---

## 3. Per-Operation Semantics on the Build Manifold

### `focus  spec -> manifold`

Read `mirror.spec`. Evaluate it as a `@mirror/project` grammar. Produce
the build manifold: the typed graph of source/legacy/target/settle_on
nodes connected by altitude and dependency. The manifold is what every
subsequent operation sees.

Focus is read-only. Failure here is a syntax failure on the spec; the
transparency is `failure(opacity_map)` and mosaic halts before any I/O.

### `project  targets -> resolved`

Given the manifold, filter to the requested targets (`mirror kintsugi
./mirror.spec --target binary,action`). Resolve `source` and `legacy`
directives into their concrete file sets. Resolve dependencies between
targets. Produce `resolved` — the manifold restricted to what is
actually being built this invocation.

Projection is structural. No I/O beyond reading the file system to
resolve globs.

### `split  shards -> [shard]`

The manifold's source shards are independent crystals. Split parses
each `.mirror` shard into a MirrorAST, content-addresses it, and emits
the list. Each shard carries its own `transparency<p>` from its own
compile. No cross-shard dependencies at this stage — split is the
parallelizable phase.

This is the variety-preservation move: shards stay separable; their
fibers are not yet mixed. Mosaic preserves the fibered structure of
the project until shift demands a coherent emitter.

### `shift  altitudes -> emitter`

For each target, shift the resolved shards from their declared altitude
to the target's altitude. A target at `@code/rust` pulls shards down to
the Rust altitude; a target at `@release` pulls them down to the
packaging altitude. **Shift is where `@io` crossings happen** — if the
shift hits an altitude that requires an external tool (cargo, llvm,
opencl, kubectl, …), that tool is called through `@io`.

The emitter is whatever the altitude requires:
- `@code/rust` → a Cargo workspace projection + a `cargo` @io call;
- `@ci/github` → a `.github/workflows/*.yml` projection + no external
  call (YAML is the artifact);
- `@release` → a signed bundle projection + a `gh release` @io call.

An emitter is not yet `au`; it is the staged thing ready to settle.

### `settle  emitter -> au`

Run the emitter. Capture the verdict. If the emitter terminates clean,
produce `au(altitude)`. If it produces opacities (compile errors, test
failures, missing signatures), produce `imperfect(au(altitude), e,
transparency<p>)`.

Settlement is the only operation that completes the loop: it produces
the gold (au) and the located opacity map (transparency) in one step.
The transparency map is what kintsugi reads to pick the next focus.

---

## 4. The Cargo `@io` Contract

When shift hits `@code/rust`, cargo gets called. The contract:

```mirror
in @io
in @code/rust

# Inside the @mirror/mosaic glass, for the @code/rust altitude:
shift @code/rust(resolved) -> rust_emitter {
  let workspace = project_workspace(resolved)
  let env = env_of(resolved)
  rust_emitter {
    workspace: workspace,
    invocation: @io.cargo.build {
      manifest: workspace.cargo_toml,
      env:      env,
      profile:  resolved.profile,
    }
  }
}

settle rust_emitter -> au(@code/rust) {
  let verdict = run(emitter.invocation)
  imperfect_of(verdict, emitter.workspace.artifact_path)
}
```

Cargo is one `@io` delegation. Its verdict is captured as an opacity
map if it fails. **`Cargo.lock` is an `@io` artifact, not substrate
truth.** The substrate truth is the Splinter graph (per `splinter +
spectral-db edges`); `Cargo.lock` is the projection cargo demands.

The same contract holds for every backend tool:
- `@code/llvm` shifts to an `@io.llc` invocation;
- `@code/opencl` shifts to an `@io.opencl_compile` invocation;
- `@code/metal` shifts to an `@io.xcrun_metal` invocation;
- `@ci/github` does NOT shift to an @io tool — YAML is the emitter, the
  artifact is the YAML file itself;
- `@release` shifts to an `@io.gh_release_create` invocation.

Each tool is named in its altitude grammar. Mosaic does not know about
cargo — it knows about `@code/rust`, which knows about cargo. The
substrate-pull goes through the altitude, not through mosaic.

---

## 5. Settlement Criteria

Mosaic settles when:

```mirror
settle_on {
  all_targets.au_produced
  total_transparency.weight = 0   # zero opacities
}
```

The spec declares the `settle_on` block; mosaic checks it. If all
targets produce `au` and the composed transparency over the whole build
is `success` (no opacities anywhere), mosaic emits the final `au` and
the build is settled.

If any target's transparency is `partial`, mosaic emits `au` for what
settled and `transparency<p>` for what did not. The kintsugi loop reads
the latter to pick the next iteration. **A partial settlement is
still a settlement** — mosaic does not halt on partial; it reports.

Failure (`transparency = failure(opacity_map)`) is the only halt: an
opacity that cannot be located on the AST means the substrate cannot
name what is wrong, and the loop has nothing to drive.

---

## 6. Connection to Kintsugi

Kintsugi runs mosaic. Mosaic settles (or partially settles). Kintsugi
reads the transparency. Picks the highest-weight opacity. Fills the
hole (via Fate tournament, the `\` resolver, or human override). Runs
mosaic again. The new transparency has lower weight (or kintsugi
rejects the fill). The proof eⁿ⁺¹ < eⁿ IS the total transparency
weight decreasing tick over tick.

```
kintsugi tick:
  emitter      = mosaic.settle(spec)
  opacity      = transparency.argmax(emitter.transparency)
  fill         = fate.tournament(opacity)
  new_spec     = apply(spec, fill)
  new_emitter  = mosaic.settle(new_spec)
  if total_weight(new_emitter.transparency) < total_weight(emitter.transparency):
    take(fill)
  else:
    reject(fill)
```

Mosaic's loop IS kintsugi's loop, viewed at the build altitude. Same
five operations. Different manifold. Same proof.

See `kintsugi-ci-v0.1.md` for the CI surface that calls this loop in a
runner; see `mirror-kintsugi.md` for kintsugi as a grammar pass; this
spec covers kintsugi as a **build pass**, which is the same loop
applied to `mirror.spec` instead of a single grammar.

---

## 7. The `.shatter` Projection

Mosaic's output is `au` at an altitude. Mosaic's working data is the
build manifold. **The optional projection to disk is `.shatter`**.

The fragmentation store (`@mirror/store`, per the open/closed split) is
canonical: the build manifold lives there, content-addressed. The
`.shatter` file on disk is a projection of that content into the AST
projection format — useful for inspection, sharing, debugging. Not
authoritative.

For each target, mosaic can write `target.shatter` next to the source.
The `.shatter` carries: the projected AST, the `transparency<p>` (per
the transparency spec), the proof block, the eigenvalue trajectory, and
the altitude. Reading `target.shatter` reconstructs what mosaic
settled, without needing the store.

---

## 8. Phase-Bounded Delegation (v0.1 → v1.0)

The cargo @io call shrinks over time. The shrinkage is the substrate-
pull: as mirror takes over compilation phases, the cargo @io call
shrinks to just the link step. The contract is staged:

| Phase  | Cargo handles                              | Mirror handles                       |
|--------|---------------------------------------------|--------------------------------------|
| v0.1   | parsing, type-check, codegen, link          | (mosaic only orchestrates)           |
| v0.3   | type-check, codegen, link                   | parsing                              |
| v0.5   | codegen, link                               | parsing, type-check                  |
| v0.7   | link                                        | parsing, type-check, codegen         |
| v1.0   | (nothing)                                   | the lot — mosaic is the build       |

Each phase reduction is a separate kintsugi pass on the
mirror.spec/mosaic boundary. The proof of progress is the cargo @io
footprint in `Cargo.lock` (which shrinks). Cargo is not removed; it is
retreated to its final irreducible role and then displaced.

This is the substrate-pull pattern in operation: not rewriting cargo,
but growing mosaic until cargo has no surface left to defend.

---

## 9. Mirror's Own Dogfood

`mirror.spec` at the root of the mirror project is the proof case. It
uses `@mirror/mosaic` to build mirror itself. The binary that runs the
build IS the binary the build produces. The build is its own fixed
point.

The sequence:
1. Bootstrap mirror builds the current mirror binary via cargo (v0.1).
2. Mirror reads its own `mirror.spec`.
3. Mosaic settles `mirror.spec` and emits `au(@code/rust)` — the next
   mirror binary.
4. The next binary is byte-equivalent to (or strictly better than) the
   bootstrap binary.
5. Repeat.

This is the self-hosting contract `spectral-db-mirror.md` describes for
the database. Same shape, at the build altitude: the system describes
itself, and mosaic settles the description into a running binary.

---

## 10. Open Questions

1. **Parallelism.** Split says shards are independent; how is the
   actual parallel execution scheduled? Probably HamiltonScheduler
   (per the architecture note), with priorities derived from the
   manifold's eigenvalues. Needs reconciliation with @mirror/runtime.

2. **Incremental settlement.** When `mirror.spec` changes one shard,
   mosaic should not re-shift the whole project. The Splinter graph
   already content-addresses everything; mosaic should be able to
   diff and re-settle only the affected fibers. The exact contract
   needs a spec of its own ("mosaic-incremental.md"?).

3. **Cross-target settlement.** A `release` target depends on a
   `binary` target depends on shards. The `settle_on` block is
   currently a flat list; it likely needs to express ordering and
   cross-target gates. Defer to mirror-spec-schema for now.

4. **Failure recovery.** If `cargo build` fails mid-settle, the
   transparency map captures it — but what happens to the partial
   workspace on disk? Mosaic should treat the workspace as
   ephemeral (in `target/mosaic/`, throwaway) and only persist `au`
   on success. Needs a `target` directory contract.

5. **The other backends.** llvm, opencl, metal, beam are named in this
   spec; each will need its own altitude grammar declaring its @io
   contract. v0.1 ships `@code/rust` (cargo) and `@ci/github` (YAML).
   Others are forward work.

6. **`.shatter` write policy.** Is `.shatter` always written, or only
   on `mirror kintsugi --emit shatter`? Recommendation: only on
   explicit emit. The store is canonical; disk projection is for
   humans.

---

*Mosaic is the substrate's build system.*
*Build = settlement. The five operations apply at the build altitude.*
*A glass inside @mirror, not a sibling of @mirror.*
*Cargo is one @io call that shrinks each release.*
*Kintsugi's loop and mosaic's loop are the same loop at the same altitude.*
*The mirror binary that runs mosaic IS the binary mosaic produces.*
