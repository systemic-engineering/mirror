# `|\>` and Au: the pipe with a hole produces locally-optimal binaries

*2026-05-25. Reed + Alex.*

Status: **Yellow** — recognition complete; prior art established; implementation lives in the @code/llvm + Fate substrate that's already on the ROADMAP.

---

## Thesis

The `|\>` operator in mirror grammars is **pipe-with-a-hole**: the `\` typed-hole sigil embedded inside the composition operator itself. When the compiler sees `binary = @mirror/cli |\> @code/llvm`, it doesn't fix the algorithm that maps CLI grammar to LLVM IR — it asks Fate to resolve that algorithm at compile time, on the local hardware. The resulting binary is **Au**: verified conductivity in context. Same AST OID across deployments; different binary OIDs, each locally optimal. The verification chain runs on the AST; the binary stays free to adapt.

---

## The operator

`\` in mirror is the typed hole — honest uncertainty as a first-class value. `abstract default = \` declares a slot for Fate to fill via tournament selection.

`|>` is forward composition: `a |> b` means "a's output feeds b's input."

`|\>` is the composition of the two: the operator that says **"compose `a` and `b`, but the algorithm that connects them is itself a typed hole."**

What Fate resolves when it sees `a |\> b`:

- **LHS shape** — whatever `a` produces (a typed grammar surface).
- **RHS shape** — whatever `b` consumes (a typed grammar surface).
- **Target type** — the LHS binding (`binary`, `crystal`, `oid`, whatever) provides the verification context that constrains what the hole's resolution must produce.
- **The transformation chain itself** — the sequence of grammars, the order of phases, the instruction-set selection, the memory pattern.

The spec stays implementation-agnostic. The compilation infrastructure self-organizes the path. The shape declares the endpoints; Fate's tournament finds the network between them.

---

## Hardware adaptation, by construction

For `binary = @mirror/cli |\> @code/llvm`, Fate's tournament at compile time probes the local hardware and selects:

| Hardware | Selection |
|---|---|
| Apple Silicon M1/M2/M3 | ARMv8.5-A + NEON + AMX matrix coprocessor; UMA memory model; Metal where GPU is involved |
| AWS Graviton | ARMv8.2-A + NEON; no AMX; explicit memory; OpenCL where GPU is involved |
| x86_64 Linux with AVX-512 | AVX-512 instruction subset; explicit memory; OpenCL or CUDA-via-OpenCL where GPU is involved |
| x86_64 Linux without AVX-512 | AVX2 fallback; same memory pattern |
| Generic CPU-only | LapackBackend; conservative ISA |

The AST that produced these is **bit-identical** across all of them. The content-addressed source OID is the same. Each binary's bytes differ; each is locally optimal. The verification chain runs on the AST; it stays intact even when the binary diverges.

This is **the Physarum pattern extended to hardware**. Different cities have different terrain; the slime mold builds an optimal network for each city; the cities' rail networks serve the same logical purpose but the physical tracks differ per terrain. `|\>` does the same for instruction selection — same logical composition (AST), different physical tracks (instruction set) per terrain (CPU).

---

## What this dissolves

Several pieces of conventional compilation friction collapse:

1. **Cross-compile configuration.** No `-march=native` vs `-march=x86_64-v3` vs target triples. Fate detects what's available and picks the optimal subset.

2. **Build matrices for heterogeneous deployment.** A cluster of mixed Apple Silicon + Graviton + x86_64 Linux doesn't need a CI matrix of N targets × M configurations. Each node compiles locally-optimally from the same source AST. One source repo. N optimal binaries.

3. **Manual feature flags for backend selection.** The NumericalPrism backend decision tree from `coincidence/heterogeneous-numerical-prism.md` (MetalBackend on Apple Silicon, OpenCLBackend on cloud GPU, LapackBackend fallback) becomes one operator: `gpu_compute = @prism |\> @code/gpu`. Fate picks the backend that fits.

4. **The Apple-UMA-vs-cloud-OpenCL tension** named in `mirror/ROADMAP.md` §2 dissolves. The ROADMAP currently names it as "Mac dev-bonus, cloud non-optional" — a tension between paths. With `|\>`, both paths exist as Fate-resolutions of the same hole; neither requires a code-level switch.

5. **Fat binaries.** No need to ship multiple-architecture bundles. Each machine compiles its own from the verified AST.

---

## Au as the binary

From the project memory: *Au is the output type of Fate inference; gold conducts; verification = conductivity in context.*

The binary produced by `|\>` IS Au, structurally:

- **Locally optimal** — the conductivity. The binary uses the instruction set, memory model, and parallelism strategy that maximize throughput on the local CPU.
- **Content-addressed source** — the verification. The AST OID anchors the binary to a verified shape, even as the bytes adapt.
- **Fate-resolved transformation** — the gold in the cracks. The transformation chain is the kintsugi step: a hole filled with the optimal algorithm for this terrain.

The binary's bytes diverge; the AST's bytes don't; the verification chain is unbroken because it runs on the AST, not the binary.

**This is a stronger guarantee than "same bytes everywhere."** That conventional approach produces least-common-denominator binaries — compromise compilations that work on the worst supported hardware. Au-via-`|\>` produces best-case binaries for each terrain, anchored to a single verified source.

---

## The verification chain for spectral.engineer

The ROADMAP's Phase 7 names spectral.engineer as the v1.0 deployment target. With `|\>` as the substrate operator:

- **Every deployment proves it ran the same AST.** Source OID equality is the verification predicate.
- **Each deployment ran the binary that was locally optimal.** Binary OID is allowed to differ; Fate's tournament logs which resolution it picked.
- **The pair composes to:** *locally-optimal-and-source-verified*. Production hardware runs the binary that exploits its full ISA; the audit trail proves the source was the same as dev / staging / other production nodes.

This is stronger than reproducible builds in the conventional sense (same bytes everywhere) because it doesn't sacrifice performance for verifiability — the two run on different layers of the content-addressed graph.

---

## What the spec author writes vs what's discovered

| Author writes | Compiler discovers |
|---|---|
| Input grammar (LHS) | Algorithm chain |
| Output grammar (RHS) | Phase ordering |
| Target type (verification context) | Instruction set subset |
| | Memory model (UMA vs explicit) |
| | GPU backend selection |
| | Optimization decisions |
| | Parallelism strategy |

This is the right division of labor. The author knows *what shape they want*. The compiler knows *how to find the best path on this hardware*. The `|\>` operator names the boundary.

---

## Prior art

This pattern shows up in pieces across the literature; the synthesis is novel:

- **Fat binaries (Mach-O, AppImage)** — ship multiple architectures in one file. The opposite move: same bytes, multiple targets, vs `|\>`'s different bytes per target with same source.
- **LTO (link-time optimization)** — partial hardware adaptation at link, not at compose. `|\>` operates earlier in the pipeline.
- **JIT compilers (HotSpot, V8)** — hardware-adaptive but with no source verification. The runtime adapts; the AST isn't content-addressed.
- **Nix derivations** — content-addressed source AND output, but not hardware-adaptive in this sense; same derivation hash means same output bytes.
- **CompCert** — verified compilation, but for a fixed target architecture.
- **Physarum polycephalum network optimization** (Tero et al. 2010) — the biological precedent: declare endpoints, the network self-organizes optimal connectivity per terrain.
- **Connes spectral triple** — the algebraic frame for "same operator algebra (A), different Hilbert spaces (H) per representation, same Dirac operator (D) connecting them." Mirror's AST is A; per-hardware binary is H; `|\>` is the representation morphism.

---

## Connections to other recognitions

- **moves-as-ticks** (`cross-domain/moves-as-ticks.md`): Fate's role at `|\>` is the MC making the hard move when the composition is uncertain. The tournament selection IS the MC choosing how the move resolves on this terrain.
- **agent-home-as-typed-hole** (`agents/agent-home-as-typed-hole.md`): the same `\` pattern — a typed hole that gets resolved by structural context. Here the hole is in the composition operator instead of in spawn's identity argument.
- **heterogeneous-numerical-prism** (`coincidence/heterogeneous-numerical-prism.md`): Anna Jakobs's pattern for explicit host-device synchronization becomes one selection branch in Fate's tournament. `|\>` makes the multi-backend story declarative instead of conditional.
- **mirror-supersedes-daemon** (`glue/mirror-supersedes-daemon.md`): gen_prism's `tick(state, message) -> tick_result` is the runtime analog of `|\>`'s compile-time resolution. Both are *(input, action, context) -> output* where the action is Fate-resolved.
- **au-conductivity** (project memory): names the return type. This insight names the *operator* that produces it.

---

## Open questions

1. **Tournament transparency.** When Fate picks an instruction-set subset, does the audit trail surface which subset was chosen, with a justification? Recommendation: yes, the tournament result is itself part of the build artifact (a `.tournament.json` sibling to the binary, content-addressed).
2. **Deterministic mode.** For environments that need bit-identical output (reproducible builds for security audits), is there a flag to lock Fate's resolution to a fixed target? Recommendation: yes — `mirror craft --target binary --deterministic` constrains the tournament to the worst-case subset, producing the conventional reproducible-builds output.
3. **Recompile triggers.** When the local hardware changes (kernel upgrade exposes new ISA features, new GPU driver), does Fate re-run the tournament? Recommendation: yes — the tournament's input set is itself part of its content-address; new hardware = new input set = new resolution.
4. **Multi-tenant clouds.** On hardware where the visible CPU/GPU may differ from the underlying silicon (containers, VMs, hypervisors), what does Fate target? Open. Probably the visible capabilities, but the audit trail should flag the virtualization layer.

---

*The spec declares the shape. Fate finds the path. The binary is Au. The verification stays in the source. Hardware adapts; truth doesn't.*

Apache-2.0.
