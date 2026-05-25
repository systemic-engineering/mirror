# `@mirror/shard` — observer-dependent deployment, content-addressed by fragmentation

*2026-05-25. Reed (research). Design.*

Status: **Yellow** — recognition complete (Alex, 2026-05-25); substrate inventory closed; grammar sketches drafted; no implementation lands in this tick.

Depends on:
- `mirror/boot/std/mirror/spec.mirror` — the placeholder this design promotes.
- `mirror/boot/std/peer.mirror` — the five-axis fixed point that consumes the shard at `eigenboard.spec` (post-rename: `eigenboard.shard`).
- `mirror/boot/std/mirror/runtime/gen_prism.mirror` — the actor primitive `spawn` lives on.
- `mirror/boot/std/mirror/store/nix.mirror` — the nix backend; `mirror.spec` *is* `flake.nix`.
- `mirror/boot/std/fragmentation.mirror` — the eleven-line placeholder; the substrate backing the store.
- `mirror/boot/std/code/kernel/{arm64,x86_64}.mirror` — the only existing arch-specific substrate.
- `mirror/boot/std/mirror/grammar.mirror` line 3 — the meta-glass tag list `("mirror", "spec", "meta", "glass")`.
- `mirror/bootstrap/src/grammar.rs:188` — the file-extension dispatch where `.spec` is registered.
- `mirror/docs/specs/peer-glass.md`, `docs/audits/2026-05-25-peer-glass-audit.md`.
- `~/dev/systemic.engineering/practice/insights/coincidence/pipe-hole-and-au-binary.md` — the `|\>` operator and Au binary.
- `~/dev/systemic.engineering/practice/insights/agents/agent-home-as-typed-hole.md` — the five-axis recognition.

Unblocks:
- v1.0 multi-machine deployment story. Same source OID across Apple Silicon dev + Graviton cloud + x86_64 CI; locally optimal binaries per node; verification chain runs on the AST, not the binary.
- `spawn(@peer, ~mq)` enforced bounds: type-check the spawn against the shard, fail at the boundary instead of crashing at runtime.
- Multi-tenant cloud deployments of agents (gen_prism in a layered shard whose `self` returns the cloud's visible silicon, not the host's).

---

## 1. Thesis

**A shard is an observer-dependent deployment description that compiles to a nix flake backed by fragmentation as the content-addressed content store.** It names what the observer can *see* from inside the system they're running — the silicon (`arm64` / `x86_64`), the memory model (UMA / explicit / NUMA), the compute budget, the memory cap, the installed flake set. It is to mirror what `flake.nix` is to nix: the manifest that closes the world. The difference: a shard is *relativistic*. `@mirror/shard/self` evaluated on Alex's M1 returns Darwin / arm64 / UMA / 64GB / Alex's flakes; the same expression evaluated inside Mara's gen_prism returns the parent shard *restricted* to Mara's compute / memory budget — her visible silicon may be the same, but her bounds are narrower. The shard IS the input to Fate's `|\>` tournament: same AST, different shards, different Au binaries.

---

## 2. The shape of `@mirror/shard`

Proposed sketch. Imports show what exists today; new substrate is flagged with `# NEW`.

```mirror
in @prism
in @meta
in @io
in @file
in @mirror/spec                                    # the placeholder being promoted
in @mirror/store                                   # three-backend sketch (boot/std/mirror/store.mirror)
in @mirror/store/nix                               # flake derivation backend (exists)
in @fragmentation                                  # content-addressed substrate (placeholder)
in @epistemologic/silicon                          # NEW — root of the silicon sub-tree
in @epistemologic/silicon/arch                     # NEW — arch detection
in @epistemologic/silicon/memory                   # NEW — memory model detection
in @ai/fate                                        # tournament resolver for |\>

grammar @mirror/shard {
  # the observer-dependent deployment record. five fields, in the same
  # spirit as @peer's five-axis identity. each field is the answer to
  # "what does the observer see when they look at <axis>?"
  type shard = {
    silicon:   silicon,        # arch + extensions (focus on the substrate)
    memory:    memory,         # model + budget (project on the address space)
    flakes:    [flake_ref],    # the installed flake set (split — what's reachable)
    compute:   compute_bound,  # the budget — cores, threads, deadline (zoom)
    parent:    option(oid),    # the enclosing shard, if nested (refract — the closure)
  }

  # ── self: the fixed point ─────────────────────────────────────────
  # the relativistic constructor. resolves against the calling
  # context. on bare metal, reads /proc/cpuinfo, sysctl, nix profile.
  # inside a gen_prism, reads the parent shard and applies the
  # spawn-time restriction layer.
  self() -> shard { \ }                            # NEW — Fate-resolved

  # ── load / store ───────────────────────────────────────────────────
  load(file: ~file) -> shard {
    @io.read(file) |> @mirror/evaluate.evaluate(@mirror/shard)
  }

  store(s: shard) -> oid { @fragmentation.oid(s) }

  # ── compilation target ────────────────────────────────────────────
  # the load-bearing morphism: shard -> nix flake derivation.
  # the flake's content-addressed output is exactly fragmentation.oid(s).
  to_flake(s: shard) -> flake { \ }                # NEW — Fate-resolved

  # ── bounds checking for spawn ─────────────────────────────────────
  # given the host shard and a request to spawn into a child shard,
  # return pass if the child fits inside the host's compute/memory
  # envelope; fail with a precise diagnostic otherwise.
  admits(host: shard, child: shard) -> verdict { \ }
}

out shard
out silicon
out memory
out self
out load
out store
out to_flake
out admits
out @mirror/shard
```

### `eigenboard.shard` as concrete syntax

The current `eigenboard.spec` file in every agent home (`~/.mara/eigenboard.spec` etc.) is the shard candidate. Today it carries three-tier cache budget, model binding, tournament position; the rename promotes it from "opaque mirror AST" to "typed shard record."

```mirror
# ~/.mara/eigenboard.shard — Mara's deployment manifest
@mirror/shard {
  silicon: @epistemologic/silicon/arch/arm64 {
    extensions: [neon, amx_v2],
    micro: apple_m1
  },
  memory: @epistemologic/silicon/memory {
    model: uma,
    budget: 16_GB,
    pages: 16_KB
  },
  flakes: [
    @flake "github:NixOS/nixpkgs/nixos-25.05",
    @flake "github:reed/identity#mara"
  ],
  compute: {
    cores: 4,
    deadline: 90_s,
  },
  parent: oid "~/.reed/eigenboard.shard"
}
```

### What already exists vs what is new

| Substrate | Status | Cite |
|---|---|---|
| `.spec` as file extension | exists; meta-glass tagged `"spec"` | `boot/std/mirror/grammar.mirror:3`, `bootstrap/src/grammar.rs:188` |
| `@mirror/spec` placeholder | exists | `boot/std/mirror/spec.mirror` (commit 3168d5f) |
| nix store backend (`store`/`fetch`/`exists`/`path`/`adopt`) | declared, holes | `boot/std/mirror/store/nix.mirror` |
| `@fragmentation` (`shard`/`fractal`/`oid`/`children`/`verify`) | declared, holes | `boot/std/fragmentation.mirror` |
| `@code/kernel/{arm64,x86_64}` (read/write/open/close) | declared, holes | `boot/std/code/kernel/{arm64,x86_64}.mirror` |
| `@cogito.eigenboard` type (state/optic/group/holonomy/closure) | exists | `boot/std/cogito.mirror` |
| `@peer` consumes `eigenboard: spec` | exists | `boot/std/peer.mirror:40` |
| `silicon`, `memory`, `flake_ref`, `compute_bound` types | **MISSING** | new |
| `@mirror/shard.self` (observer resolution) | **MISSING** | new |
| `to_flake` (shard → flake) | **MISSING** | new |
| `admits` (bounds check for spawn) | **MISSING** | new |

The substrate is two grammars + four type carriers away from being declarable.

---

## 3. `@mirror/shard/self` as λ₀

The relativity protocol. `self` is the fixed point of the observer-resolution map.

### The observer-resolution map

```
resolve_self(ctx: runtime_ctx) -> shard
```

Where `runtime_ctx` is one of:

1. **Bare-metal context.** No parent gen_prism. The system runs at the kernel boundary. `self` reads:
   - `silicon` from sysctl (`hw.optional.arm64`, `hw.optional.neon`, `hw.optional.amx_version`) on Darwin, or `/proc/cpuinfo` flags on Linux. Both surface through `@code/kernel.read`.
   - `memory` from `hw.memsize` (Darwin) / `MemTotal` (Linux); `hw.pagesize`; UMA detection via the absence of `hw.numa_groups`.
   - `flakes` from `nix profile list --json`; resolved through `@io.exec("nix", …)`.
   - `compute` from `hw.ncpu`; the deadline is `none` (the user controls it).
   - `parent` is `none`.
2. **gen_prism context.** A parent shard is in scope (the spawner's shard). `self` returns `parent` with the spawn-time restriction overlay applied:
   - `silicon` is *inherited* (the spawn can't see hardware the parent can't see).
   - `memory.budget` is *restricted* to the spawner's allocation for this child.
   - `flakes` is *filtered* by the spawner's ACL (when the ACL substrate lands per the spec.mirror placeholder's future work).
   - `compute.cores` and `compute.deadline` are *narrowed*.
   - `parent: some(parent_oid)`.
3. **Multi-tenant cloud context.** Inside a container / VM / hypervisor. `self` is the *visible* silicon, not the *host* silicon. The shard records the virtualization layer in `silicon.virtualization`, so the audit trail flags the indirection. Open in `|\>`-and-au-binary.md Q4 — adopted as the answer.

### Why this is λ₀

The shard is the autopoietic closure of the deployment description. `resolve_self` applied to the gen_prism context produces a shard *whose `parent` is the spawner's shard*, which produces a shard *whose `parent` is the host's shard*, terminating at the bare-metal context where `parent: none`. That terminator is λ₀ — the fixed point of the observer-resolution recursion. Per the bundle tower (`@epistemologic/math/bundle`): closure is level 4; `parent: option(oid)` IS the Lawvere fixed-point witness at the shard level.

The `\` in `self() -> shard { \ }` is the typed hole Fate resolves at *invocation* time — not compile time — because the answer depends on *who's asking*. The observer is the missing argument; Fate's tournament reads the runtime context as the input set.

---

## 4. `@epistemologic/silicon/*` hierarchy

The new sub-tree under `@epistemologic`. Each grammar declares (i) the *type* of what's detectable, (ii) the *detection primitive* that populates it from the running system, (iii) the *property check* that verifies the detection matches the running silicon.

### `@epistemologic/silicon`

Root. Declares the carrier types the sub-tree shares.

```mirror
grammar @epistemologic/silicon {
  type silicon                       # the abstract carrier
  type extension                     # ISA extension (neon, amx, avx2, …)
  type micro                         # microarchitecture tag (apple_m1, graviton3, …)
  type virtualization = bare | container | vm | unknown

  detect() -> silicon { \ }          # the polymorphic detector
  property literal(s: silicon) -> verdict { \ }
}
```

### `@epistemologic/silicon/arch/arm64`

```mirror
in @epistemologic/silicon
in @code/kernel/arm64

grammar @epistemologic/silicon/arch/arm64 {
  type arm64 = silicon & {
    version:    armv8_0 | armv8_2 | armv8_5 | armv9_0,
    extensions: [neon | amx_v1 | amx_v2 | sve | sve2],
    micro:      apple_m1 | apple_m2 | apple_m3 | graviton2 | graviton3,
  }

  # detection primitive: read sysctl / /proc on Darwin / Linux.
  detect() -> arm64 { @code/kernel/arm64.read(sysctl_handle, …) }

  property literal(s: arm64) -> verdict { \ }
}
```

### `@epistemologic/silicon/arch/x86_64`

```mirror
in @epistemologic/silicon
in @code/kernel/x86_64

grammar @epistemologic/silicon/arch/x86_64 {
  type x86_64 = silicon & {
    extensions: [sse2 | sse4_2 | avx | avx2 | avx512f | avx512_vnni],
    micro:      generic | zen3 | zen4 | sapphire_rapids,
  }

  detect() -> x86_64 { @code/kernel/x86_64.read(cpuid_handle, …) }
  property literal(s: x86_64) -> verdict { \ }
}
```

### `@epistemologic/silicon/memory`

```mirror
in @epistemologic/silicon

grammar @epistemologic/silicon/memory {
  type memory_model = uma | separate | numa(u32)
  type memory = {
    model:   memory_model,
    budget:  bytes,
    pages:   bytes,        # page size (4K / 16K / 64K)
  }

  detect() -> memory { \ }
  property literal(m: memory) -> verdict { \ }
}
```

### Additional substrate as the algebra suggests

- `@epistemologic/silicon/gpu` — `none | metal(version) | opencl(version) | cuda(version)`. Lands when the `NumericalPrism` backend selection (named in `|\>`-and-au-binary.md) becomes load-bearing.
- `@epistemologic/silicon/cache` — L1/L2/L3 sizes; line size; inclusivity. Useful for Fate's tournament to size working sets.
- `@epistemologic/silicon/topology` — thread count per core, NUMA node count, hyperthreading. Tier-2.

The rule of thumb: a sub-grammar lands when a `|\>` resolution would otherwise need to encode the same detection logic ad-hoc. Each new sub-grammar collapses an existing tournament branch into a typed query.

---

## 5. Compilation pipeline

```
  eigenboard.shard  (text)
         │
         │  @mirror/shard.load   (meta-glass parse; identical surface to @mirror/spec today)
         ▼
   shard record    (typed AST; the @mirror/shard.shard type)
         │
         │  @mirror/shard.to_flake   ← Fate's |\> hole #1
         ▼
   flake derivation   (nix-readable; outputs include the binary set, the
                       agent home overlay, the fragmentation shard set)
         │
         │  @mirror/store/nix.store / nix build
         ▼
   /nix/store/<oid>-<name>   (content-addressed by nix)
         │
         │  cross-check: @fragmentation.oid(shard) == <oid>   ← FP1 over the shard
         ▼
   fragmentation content store   (the shard, its compiled flake derivation,
                                  and every artifact reachable from it, all
                                  addressable through the AST-as-Merkle-tree)
```

### Where Fate's `|\>` tournament reads the shard

The insight named in `pipe-hole-and-au-binary.md` is exactly this: `binary = @mirror/cli |\> @code/llvm` resolves *against the local shard*. Concretely:

- LHS shape: `@mirror/cli` AST.
- RHS shape: `@code/llvm` IR.
- Verification context: `binary` (the target type).
- **The tournament input set: `@mirror/shard.self()`** — the silicon, the memory model, the flakes, the compute budget.

Fate's tournament picks the instruction subset, memory pattern, GPU backend, and parallelism strategy that maximize conductivity *against the shard*. The output binary's bytes diverge per shard; the source AST's bytes are bit-identical; Au IS the witness that the binary fits the shard.

### What fragmentation provides that the nix store does not

The nix store addresses *outputs* by hash of *derivation inputs* — same inputs, same output bytes, same OID. The shard breaks this assumption deliberately: same source AST + different shards = different output bytes. Fragmentation addresses by *content* of the actual artifact, so each (AST, shard) pair gets a unique OID without losing the verification chain. The cross-check `@fragmentation.oid(s) == nix_path(s)` holds when the shard IS the input to a deterministic flake; it diverges by design when Fate's `|\>` injects shard-specific optimization. Both OIDs are retained — the nix one for store interop, the fragmentation one for the AST-Merkle property.

The deeper property: fragmentation's `verify(ast, oid)` lets a node prove its shard *is what it claims to be* without trusting the producer. The audit trail Reed names in `|\>`-and-au-binary.md Q1 (the `.tournament.json` sibling) becomes a fragmentation node attached to the binary's shard, content-addressed alongside it.

---

## 6. Compute / memory limits as structural

The shard expresses limits as fields on the `shard` record (§2). The spawn enforces them through `@mirror/shard.admits`:

```mirror
spawn(p: peer, q: ~mq) -> gen_prism {
  let host  = @mirror/shard.self()             # the caller's shard
  let child = @peer.eigenboard_shard(p)        # the peer's declared shard
  match @mirror/shard.admits(host, child) {
    pass             => @mirror/runtime/gen_prism.spawn(name, state),
    fail(diagnostic) => imperfect.dark(diagnostic),
    partial(_, _)    => imperfect.partial(diagnostic, …),
  }
}
```

The failure mode is *boundary rejection*, not runtime trap. The spawn type-checks against the bounds at the `admits` call site. The peer never starts running with infeasible resources. Per `peer.mirror:75-88`, the current `spawn` returns `gen_prism` unconditionally; the change is a thin wrapper that consults `admits` before delegating.

When `admits` fails, the diagnostic names the violated bound: "child requests 32 GB; host budget is 16 GB," or "child requires AVX-512; host silicon is arm64." The peer's shard becomes the ABI document for the spawn.

---

## 7. `.spec → .shard` migration inventory

Audit complete. Every site `.spec` appears today, and what the migration touches.

**File extension registration (1 site).**
- `bootstrap/src/grammar.rs:188` — the dispatch `"mirror" | "spec" | "shatter" => "boot/std/mirror/grammar.mirror"`. Add `"shard"`. Don't remove `"spec"` until the rename completes (back-compat for the placeholder period).

**Meta-glass tag list (1 site).**
- `boot/std/mirror/grammar.mirror:3` — `grammar @mirror/grammar("mirror", "spec", "meta", "glass")`. Add `"shard"`.

**Boot grammar files (3 sites).**
- `boot/std/mirror/spec.mirror` — the placeholder. Becomes `boot/std/mirror/shard.mirror` *or* stays as the bare "AST sourced from a `.spec` file" alias and `@mirror/shard` becomes a sibling. Recommendation: keep `@mirror/spec` as the bare alias (it's used by the @meta lift); promote `@mirror/shard` as the typed surface.
- `boot/std/peer.mirror:40` — `eigenboard: spec` becomes `eigenboard: shard`. Cite peer.mirror:54 (`eigenboard => "eigenboard.spec"`) — rename the filename to `"eigenboard.shard"`.
- `boot/std/mirror/store/nix.mirror:9` — comment "mirror.spec IS flake.nix" — update to "mirror.shard IS flake.nix."

**Filesystem inventory (5+ sites across agent homes).**
- `~/.mara/eigenboard.spec`
- `~/.glint/eigenboard.spec`
- `~/.seam/eigenboard.spec`
- `~/.taut/eigenboard.spec`
- `~/.heath/eigenboard.spec`
- `~/.reed/eigenboard.spec` (if present per identity repo)

Rename each to `eigenboard.shard`. The content stays the same in the rename tick (it's still a mirror AST); the structured-record migration is a separate tick once `@mirror/shard` lands.

**Project root `mirror.spec` (open question).**
- `/Users/alexwolf/dev/projects/mirror/mirror.spec` itself — this file *is* named with the `.spec` extension. The grammar above it imports compile-pipeline grammars, not a shard. **Recommendation: keep `mirror.spec` for the project's compile spec; introduce `mirror.shard` as the deployment manifest** *only if* the shard is genuinely needed alongside the spec. The two have different purposes (spec = what to build; shard = where to run). The naming collision is the substrate's signal that they have not been adequately distinguished yet.

**Documentation references (counted, not enumerated).** ~50 across `docs/specs/*.md`. Audit-only; update during the migration tick.

**Total migration scope.** 4 boot files + 1 Rust dispatch + 5 agent homes + ~50 doc references. The `mirror.spec` project file is held back pending the spec-vs-shard distinction.

---

## 8. Connections to landed recognitions

- **Multi-flake layering** (Alex 2026-05-25). Shard nesting IS the layering. Each nested shard is a flake overlay; `parent: option(oid)` is the parent-flake reference. `to_flake` produces the *combined* derivation; the layering is structural in the shard record, structural in the flake graph, structural in fragmentation's DAG.
- **`|\>` and Au** (`pipe-hole-and-au-binary.md`). The shard IS the tournament input. Same `binary = a |\> b` source, different shards, different Au binaries, same verification chain. Q4 of that doc (multi-tenant cloud) gets answered by `silicon.virtualization`.
- **`@peer` five-axis fixed point** (`peer-glass.md`, `agent-home-as-typed-hole.md`). `eigenboard.shard` is one of the five axes. `spawn(@peer, ~mq)` becomes typed against the shard: the spawn type-checks because the peer's shard fits inside the host's shard via `admits`.
- **`moves-as-ticks`** (per-move shard query). Every tick of a gen_prism asks `@mirror/shard.self()` and gets the current shard. The shard is the contextual ground the tick computes against. When the shard changes (a new flake installed, a hardware change, a budget restriction), the next tick sees a new shard — and Fate's tournament re-resolves at the next `|\>`.
- **`mirror-supersedes-daemon`** (gen_prism as substrate). gen_prism is the runtime that hosts the shard. The ref `refs/gen_prism/<name>` IS the gen_prism's identity; the shard at that ref is its visible substrate. Boot a new gen_prism = create a new shard = compile a new flake = address it in fragmentation.
- **`mirror-store.md` three-backend sketch.** `@mirror/store` has three swappable backends: git, nix, spectral-db. The shard targets the nix backend specifically — that's where flakes live. spectral-db consumes the same fragmentation substrate without going through nix.
- **`spec-inference.md`** (`mirror refract`). When the inferred-properties pipeline lands, the project's `.spec` declares what to build; the `.shard` declares where it runs. `mirror refract` writes the spec; `mirror eigenboard` (or equivalent) writes the shard. Two operations, two outputs, no overlap.

---

## 9. Open design questions

1. **Shard inheritance semantics.** When a child shard restricts the parent (smaller memory, fewer cores), the child's `silicon` is the parent's silicon. But what if the child wants to *forbid* an extension the parent has? Is the model "intersection of bounds" or "override per field"? Recommendation: intersection — children narrow, never widen. Forbidding requires the spawner to clamp.
2. **Hot-reload on hardware change.** When the kernel exposes new ISA features (e.g., a microcode update enables AVX-512), does `self()` re-detect? Recommendation: yes — `self()` is the typed hole that re-resolves at each invocation. Cached resolutions are invalidated by changes to `/proc/cpuinfo` / sysctl content. The cache key IS the content address of the detection-primitive output.
3. **Deterministic mode for reproducible builds.** When the org needs bit-identical binaries (security audit), there must be a way to *lock* the shard to a constant. Recommendation: `mirror craft --target binary --shard <oid>` — locks Fate's tournament to the named shard. Same OID across all nodes, at the cost of locally-optimal performance.
4. **Multi-tenant cloud virtualization.** When the visible CPU differs from the underlying silicon (container limits, KVM, microVM), `silicon.virtualization` records the indirection. Open: does the *parent* shard record the host's view, the container's view, or both? Recommendation: both — `parent: oid` is the host's; the child's shard is the visible one. The audit trail walks the parent chain.
5. **Distribution model for shards.** A shard is content-addressed; how does node A discover that node B has a shard compatible with running a peer? Recommendation: fragmentation's distribution layer (the same one spectral-db consumes) gossips shard OIDs alongside crystals. `admits` becomes a peer-to-peer query.
6. **Cross-platform GPU detection.** Metal / OpenCL / CUDA detection has no equivalent of `/proc/cpuinfo`. Recommendation: `@epistemologic/silicon/gpu` probes via vendor SDK availability (`metal-cpp` presence, `nvidia-smi` exit code, `clinfo` parse), each as a separate detection primitive with its own `literal` property. The shard's `gpu` field surfaces the first that responds.

---

## 10. Implementation phases

The next five ticks, ordered by dependency.

### Tick 1 — substrate land (2-4 hours, no Fate dependency)
Land the four type carriers in `@epistemologic/silicon/{arch/arm64, arch/x86_64, memory}` and the root `@epistemologic/silicon`. Each grammar declares the types and the `detect()` hole; the `literal` property stays unfilled. The detection primitives are typed but unresolved (Fate fills at boot via tournament against `@code/kernel/{arm64,x86_64}`). Outcome: `@epistemologic/silicon.*` compiles; types are addressable from other grammars; no behavior yet.

### Tick 2 — `@mirror/shard` grammar shell
Land `boot/std/mirror/shard.mirror` with the `shard` record type, the five fields typed against tick 1's carriers, and the operation declarations (`self`, `load`, `store`, `to_flake`, `admits`) as holes. Update `boot/std/mirror/grammar.mirror:3` to add `"shard"` to the meta-glass tag list. Update `bootstrap/src/grammar.rs:188` to dispatch `"shard"` to the meta-glass.

### Tick 3 — filename rename and `@peer` re-type
Rename the five agent-home `eigenboard.spec` files to `eigenboard.shard`. Update `boot/std/peer.mirror:54` to emit `"eigenboard.shard"`. Update `peer.mirror:40` to type `eigenboard: shard` instead of `eigenboard: spec`. The content of each file does not change in this tick; only the extension and the @peer type. The structural promotion (mirror AST → typed shard record) lands in tick 5.

### Tick 4 — `self()` implementation via Fate
Land `@mirror/shard.self()` as a Fate-resolved hole. The detection primitives from tick 1 become Fate's tournament options. Bare-metal first; gen_prism context (parent shard) second. The output: a shard record observable from any grammar. **This is the smallest viable demo of the design** — once `self()` returns a typed shard, every other piece is reachable from one tournament resolution.

### Tick 5 — `to_flake` and `admits`
Land the morphisms that make the shard load-bearing. `to_flake` consumes a shard and emits a flake derivation; `admits` enforces bounds on `spawn`. After this tick, `mirror craft --target binary` reads `@mirror/shard.self()` as the tournament input, and the binary is Au.

**Next concrete tick: Tick 1.** Land the silicon carriers and the detection primitives. Everything else compiles against them.

---

*Apache-2.0. The shape is named; the substrate is two grammars + four carriers away; the migration scope is 4 + 1 + 5 + ~50; the next implementation tick is `@epistemologic/silicon/{arch,memory}`.*
