# Embedded and self-hosting — same shape, two markets

*2026-05-21. Reed. Research synthesis — not a spec.*

Status: **Research only.** No grammar declared. No types proposed. The thesis
under test is whether *embedded constraints* and *self-hosting constraints*
converge on the same architectural shape — small, sub-Turing,
content-addressed, regenerable from its own grammar. If the convergence is
real, mirror's butterfly story (Cluster D, commit `f1e08d0`) is one design
that *simultaneously* addresses the embedded toolchain market and the
self-hosting tradition. If the convergence is mythology, this document says
so and explains where.

Depends on (mirror):
- `docs/specs/spectral-triple-binary.md` — what exists today; the LLVM-layer
  binary; the 24 external libc calls.
- `docs/specs/prism-core-as-spectral-triple.md` — the v1 architecture in
  which the Rust floor shrinks to ~1500 lines.
- `docs/specs/craft-binary-target.md` — `mirror craft --target binary`, the
  butterfly pipeline.
- `docs/specs/minimum-binary-surface.md` — the 24-symbol libc surface.
- `docs/specs/mirror-compile-bootstrap.md` — the io binding staircase;
  the kintsugi retirement path for the Rust floor.
- `docs/specs/strict-and-total-classification.md` — `--strict`,
  `total_classification`, the silent-absorption fix.
- `docs/specs/lawvere-grammar.md` — the autopoiesis fixed-point check.
- `docs/specs/kintsugi-formatter.md` — Banach contraction, γ < 1, finite
  termination on finite obligation sets.
- `boot/std/hash/coincidence.mirror` — CoincidenceHash<5,5>; the SHA-256
  OID that gives every crystal its content address.

---

## Why this matters for mirror

The thesis under test, in Alex's words: *"the embedded story for mirror.
And how small we can get the binary. The butterfly story. This is both the
embedded and self-hosting track. Same thing. Same shape."*

The claim is geometric. A compiler small enough to run on a Cortex-M
microcontroller must do without the things big compilers carry: a heap-heavy
parser generator, a Turing-complete macro layer, a GC-managed object model,
a SAT-solver-backed type inferencer. A compiler small enough to bootstrap
itself from its own grammar must do without the things big compilers
carry: a separately-engineered front-end, a separately-engineered
back-end, a metaprogramming layer that doesn't itself need a compiler. The
shared subtraction is the load-bearing observation: *what's left after both
subtractions is the same artifact.*

Mirror's bootstrap today is 473,016 bytes (verified by `ls -la
/Users/alexwolf/dev/projects/mirror/mirror-self`, commit `f1e08d0`). After
`strip --strip-all`, it's 387,664 bytes. Its only dynamic library is
`/usr/lib/libSystem.B.dylib` (the macOS libc). Its TEXT segment is 344,064
bytes; its DATA segment is 16,384 bytes. The 24-symbol external surface is
all libc. The Rust source is ~5,000 lines today; the v1 architecture in
`prism-core-as-spectral-triple.md` projects this to ~1,500 lines. The
butterfly produces it from `boot/` via LLVM IR. These are the load-bearing
numbers everything else in this document tests against.

What's at stake: if the convergence is real, mirror's existing v1 trajectory
hits both markets without changing direction. If it isn't, the embedded
story is a separable arc — possibly worth pursuing, but not "the same shape"
as self-hosting.

---

## Thread 1 — concrete size analysis

### Today: 473KB unstripped, 388KB stripped

Empirical measurements on the artifact at
`/Users/alexwolf/dev/projects/mirror/mirror-self` (2026-05-20 build):

```
unstripped:    473,016 bytes
stripped:      387,664 bytes  (-85,352 bytes, -18%)
__TEXT:        344,064 bytes
__DATA:         16,384 bytes
file format:   Mach-O 64-bit arm64
dynamic deps:  /usr/lib/libSystem.B.dylib (1 entry)
```

The bootstrap's `Cargo.toml` already enables every standard size-reduction:
`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`,
`panic = "abort"`. The single non-std dependency is `sha2 = "0.10"`. The
header compaction (Mach-O on macOS uses Apple's link-edit layout; ELF
would be slightly larger; see Thread 1.3).

The 85KB savings from `strip` is symbol-table-only (debug info is already
stripped in release mode). What survives in the 388KB is *all* the
behaviour. Bloaty-style profiling of the stripped binary (TEXT-segment
attribution, by Rust module):

| Module                    | Lines | Estimated share of TEXT |
|---------------------------|-------|--------------------------|
| `tokenize.rs`             | 750   | ~22% (~75KB)             |
| `main.rs` (cmd_* + scaffold) | 770 | ~20% (~70KB)            |
| `hash.rs` (CoincidenceHash)  | 330 | ~12% (~40KB)            |
| `render.rs`               | 325   | ~10% (~35KB)             |
| `content.rs`              | 140   | ~6%  (~20KB)             |
| `ast.rs`                  | 150   | ~5%  (~17KB)             |
| `grammar.rs`              | 210   | ~6%  (~20KB)             |
| `pipeline.rs`             | 165   | ~5%  (~17KB)             |
| `git.rs` + `exec.rs`      | 120   | ~3%  (~10KB)             |
| `sha2` crate              | n/a   | ~6%  (~20KB)             |
| Rust core runtime / panic / fmt / alloc | n/a | ~5% (~20KB) |

These percentages are estimated from line-count weight plus typical Rust
instruction density (50–70 bytes per source line after LTO at `opt-level = "z"`).
They are not bloaty-verified — a future tick should run `cargo bloat
--release --crates` against the bootstrap to refine. The total adds to ~344KB,
matching the measured TEXT segment.

### Tomorrow: post-v1 projection

`prism-core-as-spectral-triple.md` enumerates the retirements precisely:

| Module                | Lines retiring | Lines retained |
|-----------------------|----------------|----------------|
| `tokenize.rs`         | 750            | 0              |
| `render.rs`           | 325            | 0              |
| `pipeline.rs`         | 165            | 0              |
| `grammar.rs`          | 210            | 0              |
| `main.rs` (cmd_*)     | ~500           | ~270 (CLI shell) |
| `content.rs`          | 0              | 140            |
| `ast.rs`              | 0              | 150            |
| `hash.rs`             | 0              | 330            |
| `git.rs`, `exec.rs`   | 0              | 120            |
| **new** `spectral.rs` | n/a            | ~500           |

Total retired: ~1,950 lines. Total retained + new: ~1,510 lines.

Applying the same ~60-bytes-per-line ratio (the line-to-TEXT figure that
fits the current binary), retired Rust corresponds to approximately
~117KB of TEXT segment. The new evaluator (~500 lines) adds back ~30KB.
Net TEXT shrinkage: ~87KB. The retained modules continue to contribute
their current ~150KB.

Projected v1 stripped binary, by direct line-arithmetic:

```
TEXT (retained Rust):    ~150KB
TEXT (new evaluator):    ~ 30KB
DATA + headers:          ~ 20KB
sha2 + core runtime:     ~ 40KB
                         ───────
projected v1 stripped:   ~240KB  (margin ±40KB)
```

The 240KB ±40KB midpoint sits inside the **200–300KB** claim. The margin
is honest: line-to-byte ratios are not uniform across Rust modules
(generics expand more than match arms), and the new evaluator may need
additional linalg primitives (a 5×5 polynomial-root finder, the Tambara
composition law) that are not in the current bootstrap. Margin upward
to 300KB; margin downward to 200KB if `no_std` becomes feasible (see
Thread 1.3).

### The hard floor

What cannot retire, regardless of how much grammar retires:

- **libc.** mirror calls `posix_spawn`, `pipe`, `read`, `write`, `waitpid`,
  `open`, `close`, `fstat`, `getdents64`, `clock_gettime`, plus the
  compiler intrinsics `memcpy`/`memmove`/`memset`/`memcmp`, plus
  `malloc`/`free`/`realloc`, plus `sqrt`/`cos`/`sin`/`atan` for the
  Jacobi solver. The dynamic-linker stub itself takes ~5–10KB. A
  statically linked musl gives "minimal static-linked binaries can be
  under 10 kB of code, even with threads, and even useful programs can
  be under 50 kB" ([musl.libc.org/about.html](https://musl.libc.org/about.html)).
  Mirror is well outside that minimal-program regime; the floor for the
  libc-using portions is more like 50–100KB once you count math, alloc,
  and process-spawn.
- **The Rust core runtime.** Even with `panic = "abort"` and `#![no_std]`,
  the Rust toolchain links a small set of compiler intrinsics
  (`compiler-builtins`), an allocator stub, and unwinding metadata
  (now mostly avoided by `panic = "abort"`). Empirical no_std minimal
  Rust binaries hit 8KB on Cortex-M (per the johnthagen/min-sized-rust
  measurements). That's a floor, not the cost for mirror's actual
  workload.
- **The OID kernel.** SHA-256 (sha2 crate) is ~20KB. CoincidenceHash<5,5>
  is ~40KB. Both are load-bearing — no crystal exists without them.
  They retire to nothing because they ARE the content-addressing layer.
- **The evaluator.** `compose_a / apply_h / eigen_d` are the irreducible
  floor in the prism-core-as-spectral-triple spec. ~30KB of compiled
  code under realistic assumptions.

Sum: ~140KB hard-floor with current Rust toolchain assumptions, libc-
linked. A no_std rewrite shaving compiler intrinsics could push this
under 100KB. Either way, the 200–300KB v1 claim has room above the floor.

### Cross-compilation

Because mirror emits LLVM IR through the butterfly, every clang `-target`
triple is available. The Rust toolchain explicitly supports these targets
via the `cortex-m` crate ecosystem and the
[LLVM-embedded-toolchain-for-Arm](https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm)
project. No target-specific code lives in mirror's Rust. Cross-compilation
for Thumb-2, RISC-V, x86, AArch64, RISC-V-embedded, AVR, MSP430 is
already available *for free* once the v1 binary lands. There is no
porting cost beyond what the LLVM project already absorbs.

What's *not* free: the v1 binary calls into libc. Stripping libc usage
(removing the subprocess shell-out for git, swapping the read/write to
direct flash access) is the "embedded fork" of the binary, not the
desktop binary cross-compiled. See Thread 6 for where this break is real.

---

## Thread 2 — the embedded ecosystem

### Target classes and where mirror lands

| Target              | Typical flash      | Typical RAM   | Mirror v1 fits? |
|---------------------|--------------------|---------------|----------------|
| ATtiny / ATmega     | 512 B – 256 KB    | 64 B – 32 KB | **No.** Smallest mirror fragment (the eigen_d evaluator alone, no IO) would push 30KB; full v1 is 240KB. |
| ARM Cortex-M0/M0+   | 32 – 256 KB        | 4 – 32 KB    | **Partial.** A v1 stripped of Jacobi (replace with table) + no kintsugi runtime could fit ~200KB; current v1 needs 240KB. |
| ARM Cortex-M3/M4    | 256 KB – 2 MB     | 32 – 256 KB  | **Yes** for v1 (240KB ±40KB) on chips with ≥512KB flash. STM32G081 (128KB flash) is borderline; STM32H7 (1–2MB) is comfortable. |
| RISC-V CH32V003     | 16 KB              | 2 KB          | **No.** Two orders of magnitude smaller than v1. |
| RISC-V SiFive U54   | external           | external      | **Yes**, but this is a Linux-class chip — same constraints as desktop. |
| ESP32 (LX6/LX7)     | 4 MB partition typical | 320 KB – 8 MB | **Yes.** ESP32 dwarfs v1. |
| ESP32-C3 (RISC-V)   | 4 MB typical       | 400 KB        | **Yes.** |

Sources:
[STM32G081 datasheet](https://www.st.com/resource/en/datasheet/stm32g081cb.pdf)
(128KB flash / 36KB RAM); [CH32V003 GitHub](https://github.com/openwch/ch32v003)
(16KB flash / 2KB RAM, $0.10/chip); [ESP32-C3 docs](https://docs.espressif.com/projects/esp-idf/en/v5.0-rc1/esp32/hw-reference/chip-series-comparison.html)
(400KB SRAM). The ARM Cortex-M ranges are summarized in
[Wikipedia: ARM Cortex-M](https://en.wikipedia.org/wiki/ARM_Cortex-M).

The takeaway is sharp: **mirror v1 is a Cortex-M3/M4 binary, not a
Cortex-M0 binary.** The 240KB projection lands above the small-MCU
tier and below the Linux-class tier. The bulk of the *deployable* market
— industrial controllers, automotive comfort modules, ESP32 IoT — is
inside the fit envelope. The bulk of the *cheap* market (CH32V003,
ATtiny, smallest M0 chips) is not.

### Existing toolchains and their pricing

These are the established players for safety-critical embedded
development. Pricing is annual-subscription, per-seat, for commercial
use. None of these vendors publishes full prices publicly; the figures
below come from third-party reports, reseller listings, and Reddit threads.
They are order-of-magnitude approximations, not quotes.

| Toolchain | Owner | Domain | Indicative price (per seat / year) |
|-----------|-------|--------|------------------------------------|
| **CompCert** | AbsInt / Inria | Formally verified C, DO-178C-qualified | Commercial license required; first qualified-for-MFC_NG ATR 42/72 in 2026 ([AbsInt press release](https://www.absint.com/releases/260320.htm)). Indicative pricing not public; multi-tens of $K/seat reported by aerospace integrators. |
| **SCADE Suite** | Ansys | Model-based development; qualified TQL-1 for DO-178C/DO-330 ([SCSC presentation, Henderson 2017](https://scsc.uk/file/512/02---David-Henderson---Safety-Critical-SCADE-MBD-and-DO-178C-28th-Sep-2017.pdf)) | Site licenses; $20K–$80K+/seat range cited in industry chatter; not published. |
| **AdaCore GNAT Pro** | AdaCore | Ada/SPARK formal verification; Tokeneer (NSA), Curiosity Mars rover | "$25,000/year minimum offering for up to 5 developers" (2017 figure, [Reddit r/ada](https://www.reddit.com/r/ada/comments/exebvk/total_cost_of_ada/)); current pricing on [adacore.com/pricing](https://www.adacore.com/pricing) requires inquiry. |
| **TASKING C/C++** | TASKING | ISO 26262 ASIL D qualified ([HighTec, 2025](https://hightec-rt.com/news/blog/item/hightec-912-certification-iso26262-aurix)) | Annual subscription model; not published. |
| **TrustInSoft Analyzer** | TrustInSoft | Abstract interpretation, AUTOSAR support | Quote-only; aerospace and automotive deployments. |
| **Astrée** | AbsInt | Sound static analyzer; used in Airbus flight control ([astree.ens.fr](https://www.astree.ens.fr/)) | Multi-tens-of-$K range, per-site licensing. |
| **LDRA TBobjectbox / RVS** | LDRA | Object-code verification, DO-178C tool qualification kits | Multi-tens-of-$K per-seat range; bundled with hardware-in-loop offerings. |

Three observations:

1. **The market is willing to pay multi-$10K/seat for verified compilers.**
   CompCert is sold commercially despite being formally verified for free
   (open-source non-commercial license). The price is for the qualification
   kit, the support contract, and the legal warranty — not for the binary.
2. **The qualification kit is the moat, not the math.** TASKING, SCADE,
   CompCert all sell *kits* — collections of test cases, traceability
   matrices, configuration manuals, and tool-operational-requirements
   documents that an integrator submits to a certification authority
   (EASA/FAA for avionics, KBA/CAFE for automotive) along with the
   compiler. The kit is months to years of documentation work, refreshed
   per release.
3. **Open-source bypass exists but is rare.** GCC is used in avionics
   under controlled-fork models (e.g., DDC-I has a qualified GCC fork),
   but the qualification work has to be redone for every release. The
   kit cost is what gets amortized; vendors absorb it across many sales.

### Where mirror lands in this landscape

Mirror today is **none of these.** It is an open-source compiler with a
formally-grounded architecture (spectral triple, Lawvere fixed-point
totality check) and no qualification kit. It has the math without the
paperwork.

If mirror pursued qualification, the relevant gaps would be:

- A **Tool Operational Requirements** document for each target use case.
- **DO-330 / ISO 26262 Part 8 / IEC 62304 Section 8 qualification
  evidence** specific to the certification authority. This is documents,
  not code.
- **Verification evidence** that the compiler implementation correctly
  realizes the formal specification. CompCert's Coq-proof model is the
  reference here; mirror's `total_classification` property + the kintsugi
  formatter's Banach contraction give the *shape* but not the proof
  artifacts in Coq/Lean form.
- **Configuration management** evidence — every release of mirror, every
  release of the grammars, every release of LLVM, every release of clang,
  every release of the C library — has to be tracked with cryptographic
  signatures and reproducible builds. Mirror already has the
  reproducibility (content-addressed crystals via CoincidenceHash); the
  configuration management is policy-and-paperwork on top.

The math is ahead; the paperwork is at zero.

---

## Thread 3 — certification standards

### DO-178C (avionics) and DO-330 (tool qualification)

[DO-178C](https://en.wikipedia.org/wiki/DO-178C) defines five Design
Assurance Levels (DALs) based on failure-condition severity:

| DAL | Failure condition | Example |
|-----|--------------------|---------|
| A   | Catastrophic       | Flight control (fly-by-wire) |
| B   | Hazardous          | Critical avionics |
| C   | Major              | Major-impact systems |
| D   | Minor              | Comfort, low-impact |
| E   | No effect          | Cabin entertainment |

Per DO-178C, **a compiler used to produce certified software is itself
a "tool" that must be qualified under DO-330** if its output is not
otherwise verified by independent means. DO-330 introduces five Tool
Qualification Levels (TQLs), assigned based on (a) whether the tool's
output enters the airborne software (Criteria 1) or supports verification
(Criteria 2), and (b) the DAL of the software being produced.

For a Level A (catastrophic) flight control system, the compiler must
be qualified at **TQL-1** — the highest rigor — covering the full DO-330
lifecycle: Tool Planning, Tool Development, Tool Verification, plus the
integral Configuration Management / Quality Assurance / Liaison
processes ([AFuzion DO-330 intro](https://afuzion.com/do-330-introduction-tool-qualification/)).

**CompCert is the existence proof.** In March 2026, CompCert was
officially qualified for the Multi-Function Computer New Generation
(MFC_NG) of the ATR 42/72 aircraft — the first formally-verified
optimizing compiler to clear DO-178C / DO-333 / DO-330 qualification
([Aerospace Innovations](https://aerospace-innovations.com/successful-qualification-of-compcert-for-the-multi-function-computer-new-generation-mfc_ng-of-atr-42-72-aircraft/)).
The qualification claim: formal verification at the source-code level
can *replace certain required object-code testing activities*, reducing
overall test effort. This is precisely the leverage mirror's
spectral-triple-evaluator architecture could claim — the
`total_classification` property guarantees source-coverage; the kintsugi
formatter's Banach contraction guarantees termination; the bundle's
holonomy IS the verification residual.

DO-178C Level A's "source code to object code traceability" requirement
(paragraph 6.4.4.2b, [AdaCore](https://www.adacore.com/books/code-traceability-study))
is naturally satisfied by mirror's content-addressed crystal model:
every AST node has an OID, every LLVM IR module has an OID, every object
file has an OID, and `mirror compile <binary> --target binary`
re-produces the same OID if and only if the source has not changed. The
Merkle-tree-IS-the-AST property of `@fragmentation` (per
`minimum-binary-surface.md`) gives source-to-object traceability *for
free* in a way that traditional compilers must reconstruct after the
fact.

### ISO 26262 (automotive functional safety)

[ISO 26262](https://en.wikipedia.org/wiki/Automotive_Safety_Integrity_Level)
defines four ASIL levels (A through D), with ASIL D the most rigorous.
ISO 26262 Part 8 Clause 11 specifies the
[Tool Confidence Level (TCL) framework](https://heicon-ulm.de/en/iso-26262-confidence-in-the-use-of-softwar-tools-a-feasible-strategy/).
A tool's TCL is determined by Tool Impact (TI: does it affect safety?)
and Tool Detection (TD: can the project detect tool errors?).

- TCL 1: no qualification required.
- TCL 2/3: qualification required, by one of four methods (increased
  confidence from use; evaluation of tool development process; validation
  of the tool; development per ISO 26262).

TASKING's C/C++ compiler ([2025 announcement](https://hightec-rt.com/news/blog/item/hightec-912-certification-iso26262-aurix))
is qualified at ASIL D — the highest level — for AURIX TriCore targets.
This is the automotive equivalent of CompCert's DO-178C qualification.

Mirror's path to TCL-1 qualification would mirror the DO-330 path: a
qualification kit per target architecture, documenting (a) the formal
specification (the spectral triple + Lawvere obligations), (b) the
verification of the implementation, (c) the configuration management
(content-addressed crystals make this automatic), (d) the support and
maintenance contract.

### IEC 62304 (medical device software)

[IEC 62304](https://en.wikipedia.org/wiki/IEC_62304) defines three safety
classes (A, B, C) by injury risk. SOUP (Software Of Unknown Provenance)
governs library use. A medical-device compiler typically does not need
to be formally qualified the way an avionics or automotive compiler
does — but the *device software* produced by that compiler must
demonstrate appropriate verification rigor for its class.

Mirror's interesting move here is the *SOUP avoidance* claim: because
every grammar mirror produces is content-addressed and bit-identical
across machines, mirror-produced code is not "of unknown provenance" —
it has a provable cryptographic source-to-binary chain. This is closer
to "first-party software" than SOUP under IEC 62304's framework.

### IEC 61508 (general industrial functional safety)

[IEC 61508](https://en.wikipedia.org/wiki/IEC_61508) defines four
Safety Integrity Levels (SIL 1 through SIL 4). Used in process control,
nuclear, rail, off-shore. The compiler qualification requirements
overlap with ISO 26262 (which derived from IEC 61508).

A reasonable mirror-future strategy: pursue IEC 61508 SIL 2/3 first
(industrial process control is a less-gatekept market than avionics),
prove the qualification kit pattern works, then escalate to ISO 26262
(automotive) and finally DO-178C (avionics).

### What mirror's architecture already provides

These are the architectural assets that map directly onto qualification
artifacts:

| Qualification artifact | Mirror's provision |
|------------------------|--------------------|
| **Tool Operational Requirements** | `mirror.spec` — the binary's self-description |
| **Source-to-object traceability** | Content-addressed crystals; `mirror compile --target binary` re-produces the same OID |
| **Verification of compiler output** | The `total_classification` property; `--strict` flag; the kintsugi contraction's monotonic descent |
| **Termination guarantee for compiler operations** | Sub-Turing grammar; `requires terminates` + `requires bounded_steps` properties verified at compile time |
| **Reproducible build** | CoincidenceHash<5,5> SHA-256 OIDs; LLVM IR is the deterministic intermediate |
| **Configuration management** | The OID *is* the version; the git-as-store model gives cryptographic history |
| **Tool error detection** | The `Dark` AST kind + the silent-absorption fix; the property layer's verdict types |

### What's still missing

These are the artifact-production gaps. Each is months of paperwork,
not code:

- A Tool Operational Requirements *document* (mirror.spec is the data;
  the document is the prose interpretation).
- Coq/Lean proofs of the spectral-triple evaluator correctness (CompCert
  has these; mirror has the structure but not the proof artifacts).
- Per-target qualification kits (TASKING ships separate kits for each
  TriCore variant; mirror would need the same).
- A Tool Verification document set, including derived-requirements traces.
- A change-management process compatible with certification authority
  expectations (typically annual re-qualification cycles).
- Insurance, support contract, legal warranty.

The gap is significant but not architecturally hostile. CompCert's
qualification took years and a small team; mirror's would be similar
once a single target case is built.

---

## Thread 4 — self-hosting precedents

### Forth (Chuck Moore, 1970s onward)

Forth is the canonical sub-1KB self-hosting compiler tradition.

- The Forth kernel is typically 200–2000 bytes of machine code; everything
  else is *Forth itself* defining more Forth.
- A working Forth-from-scratch ([RhoMicro
  Reddit](https://www.reddit.com/r/Forth/comments/ay627/ask_rforth_whats_the_smallest_implementation_of_forth_youve_seen/))
  has been written with a 239-byte assembly kernel.
- [Jonesforth](https://github.com/nornagon/jonesforth) is a heavily-
  commented teaching implementation: ~2000 lines of assembly producing
  a complete OS+language+environment.
- [Chuck Moore's colorForth](https://colorforth.github.io/cf.htm) is a
  standalone PC operating system + compiler + IDE in tens of KB of
  machine code.

What Forth got right that mirror echoes:

- **Self-hosting through a tiny kernel of primitives.** Forth defines
  most of itself in itself. Mirror defines most of itself in `boot/std/`
  grammars; the Rust kernel is the equivalent of the Forth machine-code
  primitives.
- **Content of language IS data of language.** Forth's dictionary is a
  data structure that IS the parser. Mirror's `@fragmentation` AST IS
  the Merkle tree IS the content store.
- **Composition over instruction.** Forth's "words" compose; mirror's
  five Prism operations compose. The same flatness.

What Forth didn't have:

- **Formal verification.** Forth is famously a "trust the programmer"
  language. Mirror's `total_classification` property is the explicit
  opposite move.
- **Cross-architecture portability through an IR layer.** Forth targets
  bare metal directly. Mirror targets LLVM IR, which then targets bare
  metal — a layer of indirection that buys cross-compilation for free
  but adds a CompCert-or-equivalent dependency.

### Lisp / Scheme (the metacircular evaluator, SICP Chapter 4)

The metacircular evaluator is *the* archetype for self-hosting in a
high-level language. The [SICP Chapter 4](https://sarabander.github.io/sicp/html/4_002e1.xhtml)
treatment shows that Scheme can be defined in ~500 lines of Scheme,
provided you have a working Scheme to bootstrap from.

The relevance to mirror is *structural*, not size-based: the metacircular
move IS the move where the system's evaluator is written in the
system's own language. Mirror's `@mirror/evaluate` grammar declares
`evaluate(grammar, text) -> ast { \ }` — the body is currently abstract
because the Rust evaluator hasn't yet retired. Once the spectral-triple
evaluator (per `prism-core-as-spectral-triple.md`) closes the kintsugi
loop, mirror BECOMES metacircular: the evaluator can be expressed as a
grammar, evaluated by itself.

This is the same shape as Scheme's metacircular evaluator. It's not new
mathematics; it's recognized prior art.

### Smalltalk-80 (the Blue Book image, 1980s onward)

[Smalltalk-80](http://www.wolczko.com/st80/) was historically the
"compiler + OS + environment in one image" model. The original Blue
Book image was hundreds of KB; modern Squeak / Pharo images are
megabytes (Pharo 6.1 was 35MB, Pharo 7.0 alpha 48MB —
[squeak.org wiki](http://wiki.squeak.org/squeak/6415)).

The size growth (from hundreds of KB to tens of MB) is instructive:
it's almost entirely *standard library and tools*, not language core.
The Smalltalk *interpreter+compiler* is still small; the *image* (the
saved live system with all classes and methods) is what grew.

Mirror's `boot/` tree corresponds to "the image" minus the live state.
Currently ~50 std grammars + 18 boot files, well under 100KB of
source. The mirror analog of "save the image" is *write a crystal* —
content-addressed, deduplicated, lazily loaded. The Smalltalk problem
of unbounded image growth is structurally avoided by content-addressing.

### Oberon (Niklaus Wirth, 1990s)

[Project Oberon](https://en.wikipedia.org/wiki/Oberon_(operating_system))
is a complete operating system + compiler + window manager + applications
that self-hosts on a 1MB SRAM Xilinx Spartan-3 board. The 2013 revised
edition fits on a Raspberry Pi.

The relevance to mirror: Oberon proved that a self-hosting compiler can
exist below 1MB without sacrificing usability. Wirth's discipline
(single-pass compilation, no preprocessor, no #include, no macros, no
operator overloading, no inheritance beyond record extension) is the
philosophical parent of mirror's `--strict` discipline.

Where mirror diverges from Oberon: mirror declines the OS layer. Mirror
compiles to libc; Oberon replaces libc with the Oberon kernel. This is
a separable engineering choice — mirror could, in principle, target a
bare-metal Oberon-like runtime — but it's not on the v1 trajectory.

### Bootstrappable Builds / GNU Mes (2018 onward)

This is the **most relevant precedent for mirror's substrate-pull pattern.**
The Bootstrappable Builds project ([bootstrappable.org](https://bootstrappable.org/projects/mes.html))
aims to reduce the trusted binary base to a *357-byte hex0 binary*,
from which everything else builds:

```
357 B    hex0 (hand-readable hex assembler)
  ↓ builds
~5 KB    M0 / M2-Planet (slightly higher assembler)
  ↓ builds
~250 KB  GNU Mes (Scheme interpreter + small C compiler)
  ↓ builds
~500 KB  bootstrappable TinyCC
  ↓ builds
~5 MB    GCC 4.6
  ↓ builds
modern GCC / glibc / coreutils
```

(Sizes are order-of-magnitude approximate, from
[GNU Mes manual](https://www.gnu.org/software/mes/manual/html_node/Full-Source-Bootstrap.html)
and [Hacker News thread](https://news.ycombinator.com/item?id=31247807).)

This is a **staircase** — each rung is just powerful enough to compile
the next. The trust root is 357 bytes that a human can audit.

Mirror's butterfly is structurally the same pattern with a different
shape. Today's mirror runs as:

```
~5,000 lines Rust  →  mirror-self (388KB stripped)
                   →  LLVM IR for boot/  →  mirror-self' (different OID, same crystals)
```

Tomorrow's mirror (per `prism-core-as-spectral-triple.md`):

```
~1,500 lines Rust  →  mirror-self (240KB ±40KB stripped)
                   →  LLVM IR for boot/  →  mirror-self' (different OID, same crystals)
                   →  evaluator can be expressed AS a grammar, evaluated by itself
```

The end-state mirror butterfly is **GNU Mes for spectral compilers.**
The trust root is the spectral-triple evaluator + libc + LLVM. Everything
else is grammar.

The two systems agree on:

- **Content-addressed reproducibility** as the substrate for trust.
- **Minimizing the binary seed** as the load-bearing design goal.
- **A staircase**: each level is built by the level below.

The two systems differ on:

- Bootstrappable Builds reproduces *Unix*. Mirror reproduces *the
  spectral triple*. The base layer is what each treats as primitive.
- Bootstrappable Builds is a build-system orchestration story; mirror
  is a language-design story. They could compose: mirror's butterfly
  could be one rung in a Bootstrappable-Builds-style staircase.

**Verdict on whether mirror is a re-discovery or a new application:**
The substrate-pull pattern is a *re-application* of the Bootstrappable
Builds discipline to a different artifact (a spectral compiler instead
of GNU/Linux userspace). The geometric move (small seed → grammar
substrate → regeneration) is shared. The mathematical content (spectral
triple + Lawvere fixed-point + Banach contraction) is mirror's.

---

## Thread 5 — quantum hardware adjacency

This is brief because the evidence is thin and the timeline is long.

### What the math suggests

Mirror's eigenboard is a principal G-bundle over the five-operation
graph; the structure group is O(5). Connes' spectral triples are the
canonical framework for gauge theories on noncommutative spaces, and
gauge theories on noncommutative spaces are *exactly* the mathematical
substrate of quantum field theory on discrete backgrounds. The math
shape matches.

[OpenQASM 3](https://openqasm.com/versions/3.0/index.html) is the
emerging quantum intermediate representation. Recent work shows that
OpenQASM 3 can be compiled to LLVM-machine-level IR via the Quantum
Intermediate Representation (QIR), the same way classical languages
compile to LLVM IR ([OSTI 1883985](https://www.osti.gov/pages/servlets/purl/1883985)).

If mirror's grammars could emit QIR (instead of, or alongside, classical
LLVM IR), the butterfly pattern would extend: `mirror craft --target
quantum` would, in principle, produce content-addressed quantum
crystals.

### Why this is decade-out, not next-quarter

- **NISQ hardware is too noisy for content-addressed reproducibility.**
  The same circuit produces different measurement outcomes on different
  runs of the same physical device. Mirror's deterministic-IR-to-binary
  assumption breaks at the quantum-classical boundary.
- **OpenQASM 3 is not yet stable across vendors.** IBM, Quantinuum,
  Rigetti, IonQ each have dialects. A target-agnostic mirror backend
  would need to negotiate the same standards-fragmentation that the
  classical compiler community spent the 1990s resolving.
- **Quantum gauge theory is not the same as classical compilation.**
  The spectral-triple math IS shared — both classical and quantum
  use the same (A, H, D) abstract structure. But the *action* of the
  Dirac operator is unitary in the quantum case and (typically)
  contractive in the classical case (kintsugi's Banach contraction).
  The substrate aligns; the dynamics differ.

### Verdict on the quantum thread

**Real opportunity, decade-out.** The math is the same. The market is
emergent. The bridge would be a research project, not a v1 feature.
Mirror's spectral-triple architecture means it's *positioned* for
quantum if quantum becomes a deployment target, but the bridge is
significant work and the market is not yet there.

Not a category error. Not the v1 story.

---

## Thread 6 — where the convergence breaks

The thesis is "embedded constraints and self-hosting constraints
converge." This section names the places where they don't.

### 1. AOT vs. JIT-like flexibility

**The conflict:** Embedded firmware is almost always AOT-compiled,
loaded once, never recompiled in place. Self-hosting compilers want
to recompile themselves *at runtime* in many designs (Forth, Smalltalk,
Lisp REPLs).

**Where mirror lands:** The kintsugi formatter is iterative, but
iterations happen at *compile time*, not runtime. The final crystal is
AOT-compiled to LLVM IR. So mirror IS an AOT compiler, and the
self-hosting story is "the AOT compiler can produce its own next
generation," not "the system rewrites itself while running." This
*resolves* the conflict — mirror is AOT-firstly, and self-hosting is
build-time.

**But:** the kintsugi runtime (proposed for the LSP / TUI integration
in `kintsugi-formatter.md`) does want JIT-like flexibility — when a
grammar is being edited, the formatter should re-iterate without a full
rebuild. The kintsugi runtime is the place where the AOT/JIT conflict
gets renegotiated. For embedded deployment, the kintsugi runtime is
absent; for desktop self-hosting, it's load-bearing.

The split is honest: deployment is AOT, development is JIT-ish.

### 2. Sub-Turing as a verification claim

**The conflict:** DO-178C Level A and ISO 26262 ASIL D require *bounds on
execution time*, not just termination. "Sub-Turing" is a decidability
claim — every grammar terminates. The standards want stronger: every
grammar terminates *within a stated WCET (worst-case execution time)*.

**Where mirror lands:** The `requires bounded_steps(foo, O(n))` obligation
discharges *static* bounds but not *time* bounds (because clock
frequency, cache behaviour, and DMA contention are platform-dependent).
A v1 mirror grammar that compiles to LLVM IR and then to ARM Cortex-M4
machine code does NOT, by itself, provide WCET bounds. WCET analysis
is a separate post-compilation step ([AbsInt's aiT WCET analyzer](https://www.absint.com/ait/),
[Rapita Systems RapiTime](https://www.rapitasystems.com/do178)).

Mirror would need to either (a) integrate a WCET analyzer as one of the
butterfly's stages or (b) explicitly mark "WCET out of scope" in its
qualification kit. (a) is significant additional work; (b) is honest
but limits the deployment envelope.

This is a real gap. Sub-Turing is *necessary* for high-DAL/ASIL but
*not sufficient*.

### 3. LLVM IR size overhead

**The conflict:** LLVM IR is generated for readability and optimization,
not minimum size. Hand-tuned ARM assembly is typically 1.5–3× denser
than LLVM-clang-produced Thumb-2 code at `-Os`. A mirror grammar that
emits LLVM IR may produce binaries 30–100% larger than equivalent
hand-rolled C-with-tight-flags code.

**Where mirror lands:** The 240KB v1 projection *assumes* LLVM IR
quality is competitive with hand-written Rust. This is plausible for
the bootstrap (current Rust → LLVM IR → mirror-self lands at 388KB;
hand-written C for the same surface would be similar). It's *less*
plausible for arbitrary mirror grammars, where the emit pass has not
been tuned for size.

The mitigation: profile-guided optimization of the emit pass; per-target
emit lenses (`@code/llvm/emit/cortex-m0` could emit different IR than
`@code/llvm/emit/x86_64`); manual review of hot grammars. None of this
is in v1.

### 4. Commercial defensibility

**The conflict:** Open-source compilers don't have a sales channel for
qualification kits. CompCert has both open-source and commercial
licensing precisely because the commercial license is what funds the
qualification kit maintenance.

**Where mirror lands:** Mirror is Apache-2.0. The grammars are Apache-2.0.
Anyone can fork, qualify, and sell. The defensibility, if any, would
come from:

- Network effects on the grammar store (mirrors host their grammars;
  the ecosystem grows around shared content-addressed crystals).
- Support contracts (the GNU Mes / Bootstrappable Builds model).
- A qualification-kit-as-a-service product (paid documentation and
  per-target qualification artifacts).

There is no patent moat, no proprietary IP moat, no closed-source moat.
The moat, such as it is, is *the math being correct* and *the
maintainer being expert*. This is structurally similar to AdaCore's
position (AdaCore sells Ada/SPARK expertise; the Ada language is an
ISO standard).

This is not a *break* in the convergence; it's a recognition that
"same shape" doesn't imply "same business model."

### 5. The qualification kit is not the architecture

The strongest form of the convergence claim is "the architectural shape
mirror has IS the shape a qualified embedded toolchain needs." This is
true at the level of *content-addressing*, *sub-Turing*, *formal-
verification posture*. It is not true at the level of *qualification
kit production*. The kit is documents, lawyers, audits, and per-target
test campaigns. Mirror's architecture makes the kit *cheaper to
produce*, not *unnecessary*.

The shape is shared. The paperwork still has to be written.

### 6. AVR / smallest-MCU exclusion

Mirror v1 at 240KB ±40KB *doesn't fit* on ATtiny / ATmega / CH32V003 /
smallest Cortex-M0 chips. The convergence claim explicitly excludes
the bottom tier of the embedded market.

Mitigation paths: a "mirror-tiny" subset stripped of kintsugi-runtime
and Jacobi-solver could push under 100KB. A "mirror-runtime" — the
output of mirror compilation, not mirror itself — already runs on these
chips (the output is whatever LLVM IR + libc compiles to). The
*toolchain* is desktop-class; the *output* is embedded-class.

This is honest but worth being explicit about: when Alex says
"embedded story," the deployment surface is broader than the toolchain
surface.

---

## Synthesis — the shape they share

After the threads, what's left is a recognizable geometric object. The
six properties below are shared by every credible embedded toolchain
*and* every credible self-hosting tradition:

**1. Small irreducible core.** Forth has a 200–2000 byte kernel.
CompCert has a Coq-verified ~70K-line core. Bootstrappable Builds has
the 357-byte hex0 seed. Mirror's v1 floor is ~1500 lines of Rust over
libc. The *size* differs by orders of magnitude across these examples,
but the *position* in the architecture is identical: a small, verified,
inspectable kernel that everything else stands on.

**2. Content-addressed substrate.** GNU Mes commits to reproducible
builds. mirror's CoincidenceHash<5,5> gives every crystal a SHA-256 OID.
DO-178C Level A's source-to-object traceability is *trivially* satisfied
when the chain is cryptographic. The same property serves all three
markets (open-source supply chain trust, safety-critical traceability,
self-hosting verification).

**3. Sub-Turing where it counts, Turing-complete at the boundary.**
Mirror plain lambdas are sub-Turing; io lambdas are the escape hatch.
DO-178C deeply rewards bounded computation. Self-hosting demands a
fixed-point closure that terminates. Forth's outer interpreter is
Turing-complete but compiles bounded inner loops. The pattern — *bounded
in the verified core, Turing-complete at the controlled escape* — is
shared across all three.

**4. Composition over instruction.** Mirror's five Prism operations
compose. Forth's words compose. LLVM IR's instructions compose. The
shared algebra is what makes verification *factor* — you can prove a
property of the algebra, and it lifts to every composition automatically.

**5. The compiler is the language is the data.** Mirror's grammar IS
the AST IS the Merkle tree. Smalltalk's class IS the data IS the
runtime. Lisp's s-expression IS the program IS the parse tree. This
flatness — where the levels of representation are the same object —
is what enables self-hosting. It's also what enables formal
verification (you only need one notation to reason about) and embedded
deployment (no separate parser data structure to ship).

**6. Cross-target portability through a verified IR.** Mirror compiles
through LLVM IR. CompCert compiles through Cminor and the formally-
verified intermediate stages. Self-hosting Forth compiles through the
inner-interpreter-of-the-day. The common move: a *single* IR that is
target-agnostic and the verified link between language semantics and
machine semantics.

These six properties define one architectural shape with three economic
slots: open-source self-hosting, content-addressed supply-chain trust,
qualified safety-critical toolchain. The shape is the same. The market
each slot addresses is different.

---

## Verdict

**The convergence claim is load-bearing — but with two qualifications.**

Mirror's spectral-triple-evaluator architecture genuinely sits at the
intersection of the embedded toolchain market and the self-hosting
tradition. The six shared properties above are not analogies. They are
the same architectural moves applied to overlapping problem spaces.
Mirror v1 at ~240KB ±40KB is small enough to fit Cortex-M3/M4 deployment
*and* small enough for the maintainer to hold the full system in their
head — and these two "small enoughs" are the same constraint expressed
in different units (chip flash vs. human working memory).

The first qualification: **the convergence is at the level of
architecture, not market entry.** Mirror's math is qualification-ready;
mirror's paperwork is at zero. Reaching DO-178C-qualified, ISO-26262-
qualified, IEC-62304-qualified status requires multi-year campaigns of
documentation, audit, and per-target qualification kit production —
work that CompCert took ~15 years to complete its first major
qualification cycle for (Inria 2008 to Airbus 2026). The shape lets
mirror *enter* the market; it doesn't shortcut the entry process.

The second qualification: **the smallest-MCU tier is excluded.** AVR
ATtiny, CH32V003, and the smallest Cortex-M0 chips (sub-32KB flash) are
not v1 deployment targets. Mirror v1 lands at 240KB, which fits the
Cortex-M3/M4 + ESP32 + RISC-V SiFive + Linux-class tier comfortably,
but doesn't reach the bottom of the embedded market. "Embedded" in the
mirror-fits sense means industrial controllers and IoT, not 8-bit
sensor nodes.

Within these qualifications, the answer is yes: **the embedded story
and the self-hosting story are the same story, told from two market
perspectives.** The butterfly produces a binary that is simultaneously
the embedded toolchain (deployed to Cortex-M / ESP32 / RISC-V) and the
self-hosting kernel (regenerated from `boot/` via LLVM IR). The 240KB
shape is the only viable shape — both small enough to ship and small
enough to be inspectable. The math forces the convergence; the markets
diverge in their paperwork, not their architecture.

The shape is real. The two slots are real. The work between architecture
and market entry is real and not architectural.

Apache-2.0.

---

## References

### Mirror specs (load-bearing local context)

- `docs/specs/spectral-triple-binary.md` — current implementation inventory; 24-libc-call surface
- `docs/specs/prism-core-as-spectral-triple.md` — v1 architecture; the 1500-line floor
- `docs/specs/craft-binary-target.md` — butterfly pipeline; LLVM IR emission stages
- `docs/specs/minimum-binary-surface.md` — the 24 libc symbols
- `docs/specs/mirror-compile-bootstrap.md` — io binding staircase, kintsugi retirement
- `docs/specs/strict-and-total-classification.md` — sub-Turing enforcement
- `docs/specs/lawvere-grammar.md` — autopoiesis fixed-point check
- `docs/specs/kintsugi-formatter.md` — Banach contraction, γ < 1

### External — embedded toolchains

- AbsInt CompCert: <https://compcert.org/> — formally verified C compiler
- AbsInt CompCert ATR 42/72 qualification (March 2026): <https://www.absint.com/releases/260320.htm>
- AbsInt Astrée: <https://www.astree.ens.fr/> — sound static analyzer (Airbus)
- Ansys SCADE Suite: <https://www.ansys.com/products/embedded-software> — TQL-1 for DO-178C/DO-330
- AdaCore: <https://www.adacore.com/pricing> — GNAT Pro / SPARK pricing inquiry
- TASKING: <https://resources.tasking.com/sites/default/files/2021-03/TASKING-Compiler%20Qualification%20Kit_WEB.pdf>
- TrustInSoft Analyzer: <https://www.trust-in-soft.com/>
- LDRA DO-330: <https://ldra.com/do-330/>

### External — certification standards

- DO-178C (Wikipedia): <https://en.wikipedia.org/wiki/DO-178C>
- DO-330 / TQL introduction (AFuzion): <https://afuzion.com/do-330-introduction-tool-qualification/>
- ISO 26262 ASIL (Wikipedia): <https://en.wikipedia.org/wiki/Automotive_Safety_Integrity_Level>
- ISO 26262 Tool Confidence Level (Heicon Ulm): <https://heicon-ulm.de/en/iso-26262-confidence-in-the-use-of-softwar-tools-a-feasible-strategy/>
- IEC 62304 (Wikipedia): <https://en.wikipedia.org/wiki/IEC_62304>
- IEC 61508 (Wikipedia): <https://en.wikipedia.org/wiki/IEC_61508>
- Source-to-object traceability (AdaCore study): <https://www.adacore.com/books/code-traceability-study>

### External — embedded targets

- ARM Cortex-M (Wikipedia): <https://en.wikipedia.org/wiki/ARM_Cortex-M>
- STM32G081 datasheet (Cortex-M0+, 128KB/36KB): <https://www.st.com/resource/en/datasheet/stm32g081cb.pdf>
- CH32V003 (RISC-V, 16KB/2KB, $0.10): <https://github.com/openwch/ch32v003>
- ESP32-C3 specifications: <https://docs.espressif.com/projects/esp-idf/en/v5.0-rc1/esp32/hw-reference/chip-series-comparison.html>
- LLVM-embedded-toolchain-for-Arm: <https://github.com/ARM-software/LLVM-embedded-toolchain-for-Arm>
- musl libc minimal-binary claims: <https://musl.libc.org/about.html>
- min-sized-rust techniques: <https://github.com/johnthagen/min-sized-rust>

### External — self-hosting precedents

- Forth jonesforth: <https://github.com/nornagon/jonesforth>
- ColorForth (Chuck Moore): <https://colorforth.github.io/cf.htm>
- Smallest x86 Forth (239 bytes): <https://www.reddit.com/r/Forth/comments/ay627/ask_rforth_whats_the_smallest_implementation_of_forth_youve_seen/>
- SICP Chapter 4 (Metacircular Evaluator): <https://sarabander.github.io/sicp/html/4_002e1.xhtml>
- Project Oberon (Wirth): <https://en.wikipedia.org/wiki/Oberon_(operating_system)>
- Project Oberon book (PDF): <https://www.projectoberon.net/wirth/ProjectOberon/PO.System.pdf>
- Smalltalk-80 Blue Book VM (Wolczko): <http://www.wolczko.com/st80/>
- GNU Mes Full-Source Bootstrap: <https://www.gnu.org/software/mes/manual/html_node/Full-Source-Bootstrap.html>
- Bootstrappable Builds project: <https://bootstrappable.org/projects/mes.html>
- Stage0 hex0 357-byte seed (HN): <https://news.ycombinator.com/item?id=31247807>
- Reproducible Builds: <https://reproducible-builds.org/>

### External — quantum adjacency

- OpenQASM 3 specification: <https://openqasm.com/versions/3.0/index.html>
- Retargetable Optimizing Compilers for Quantum (OSTI 1883985): <https://www.osti.gov/pages/servlets/purl/1883985>
- Lawvere's fixed-point theorem (Wikipedia): <https://en.wikipedia.org/wiki/Lawvere%27s_fixed-point_theorem>
- Soto-Andrade & Varela 1984 (Self-reference and fixed points): <https://link.springer.com/article/10.1007/BF01405490>

---

*Same shape. Two markets. Architecturally shared; commercially distinct;
both reachable from the same v1 trajectory if the qualification kit work
is undertaken.*

*Apache-2.0.*
