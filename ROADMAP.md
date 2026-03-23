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
Rust NIF to produce ETF, loads the BEAM module. Returns a witnessed trace.
Identity: `sha512("compiler")` → first 32 bytes → Ed25519 keypair.

In supervised mode (`start_named`), the compiler is pure — it compiles and
loads modules but does not start domain servers. The garden handles lifecycle.

In standalone mode (`start`), the compiler starts its own domain supervisor
and starts domain servers after compilation. This is the backwards-compatible
imperative path.

**Domain server** — GenServer registered as the domain atom. Receives
`{action, Args}` calls, dispatches to the compiled module's exported
functions. The `exec` primitive does `apply/3` — that's where grammar
meets reality.

**Supervision** — two layers:

- `conversation/supervisor.gleam`: static supervisor (RestForOne) managing
  @compiler + garden. If @compiler crashes, garden + all domains restart.
- `conversation/garden.gleam`: factory supervisor managing domain servers
  dynamically. If one domain crashes, only that domain restarts.

The old `conversation_sup.erl` (Erlang `simple_one_for_one`) is deprecated.
New code should use the Gleam supervision modules.

---

## Actor Registration

The @compiler actor starts via `compiler.start()` (standalone) or
`compiler.start_named(name)` (supervised).

Standalone path — `compiler.start()`:

1. Starts the domain supervisor (idempotent)
2. Derives the compiler's Ed25519 keypair from `sha512("compiler")` (first 32 bytes → seed)
3. Starts the OTP actor with that identity
4. On `CompileGrammar`: compiles, loads, starts domain server, returns trace

Supervised path — `compiler.start_named(name)`:

1. Derives the compiler's Ed25519 keypair from `sha512("compiler")` (first 32 bytes → seed)
2. Starts the OTP actor with that identity, registered under `name`
3. On `CompileGrammar`: compiles, loads, returns trace (no domain lifecycle)

Domain servers are started separately via `garden.start_domain(name, domain)`.

Compiled modules get the `conv_` prefix — `@erlang` compiles to
`conv_erlang`, avoiding BEAM sticky module collisions. The domain server
still registers as the raw domain atom for action dispatch.

---

## Domain Identity → NIF

Every domain has a deterministic identity derived from its name:

```
sha512(domain_name) → first 32 bytes → Ed25519 seed → keypair → Oid
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

Two paths. Same compilation loop, different lifecycle management.

### Imperative (standalone)

`boot.boot_from_files(paths)`:

1. Start the domain supervisor
2. Start the @compiler actor
3. Compile each grammar source through @compiler
4. For each compiled domain, query `lenses/0` and `extends/0`
5. Domain servers started inline during compilation
6. Return `BootResult` with compiler subject + booted domains

### Supervised (embedded)

`boot.supervised_boot_from_files(compiler_subject, garden_name, paths)`:

1. @compiler and garden are already running (started by a parent supervisor)
2. Compile each grammar source through the named @compiler
3. Start each domain server through `garden.start_domain(name, domain)`
4. Return `List(BootedDomain)` with domain names, lenses, and extends

The supervised path is used by Reed and any actor that embeds the conversation
supervision tree inside its own tree. The imperative path is for CLI and
standalone use.

After boot, `imports_resolved(result)` and `extends_resolved(result)`
verify that all lens dependencies and inheritance chains are satisfied.
A grammar that imports `@phantom` will boot but report unresolved imports.

---

## Action Visibility (implemented)

Grammar actions support `public`, `protected`, and `private` modifiers.
Bare `action` defaults to `protected` (backwards compatible).

- **`public`** — direct return `{ok, Args}`, no gen_server round-trip
- **`protected`** — `gen_server:call` through domain server (current default)
- **`private`** — same dispatch as protected, but not exported from BEAM module

Visibility stored in TypeRegistry, emitted as `visibility/0` function on
compiled modules. Private actions filtered from export list.

---

## Traced Compilation Chain (implemented)

Each compilation phase produces a linked trace with parent OIDs:

```
source → parse(Trace) → resolve(Trace) → compile(Trace) → CompiledDomain(Trace)
```

`compile_grammar_traced` NIF returns per-phase content OIDs. The compiler
actor chains them: parse_trace → resolve_trace → compile_trace → swap_trace.
Every running module has a verifiable chain back to source.

---

## Cairn Identity Hierarchy (implemented)

`key.derive_child(root_pub, name)` — `sha512(root_pub || name) → Ed25519 seed`.
Anyone with the root public key can derive any actor's public key and verify
traces. Flat derivation (`sha512(name)`) preserved for backwards compatibility.

Compiler supports both modes: `start()` (flat) and `start_with_root(key)` (hierarchical).

---

## Shipping: `frgmt collapse`

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

### The Bridge Is Git

fragmentation writes native git objects. Nix reads git repos via
`builtins.fetchGit`. No FUSE in the build path — the Nix sandbox does not
expose `/dev/fuse`, and this is a hard constraint. FUSE is the development
surface (inspect, navigate, diff). Git is the build surface.

### Two-Derivation Build

**Derivation 1:** An escript runs `binary_to_term → compile:forms → write .beam`.
Same three-step process as `loader_ffi:load_etf_module/1`, but ahead of time.

**Derivation 2:** Takes `.beam` files, adds `.app`, boot script, `sys.config`,
`vm.args`, optionally bundles ERTS. Standard OTP release via `relx` or
`mix release`.

### What the Compiler Writes

Each `CompileGrammar` call produces ETF containing EAF. The compiler writes
this to git via fragmentation, creating a commit on a ref like
`refs/fragmentation/conversation/<domain>`. The `frgmt collapse` command reads
the tree at that ref and invokes the flake.

### Runtime vs. AOT

Two loading paths coexist:
- **Runtime** (development): `loader_ffi.erl` does `binary_to_term →
  compile:forms → code:load_binary` live on the BEAM.
- **AOT** (`collapse`): escript does the same three steps during the Nix build,
  writes `.beam` files to disk, packages as release.

The compiler doesn't change. The consumer changes.

### Open Questions

- **ETF layout:** flat `etf/` directory per domain, or nested? Flat is simplest.
- **Ref convention:** mutable `latest` for dev, immutable `<source-hash>` for
  production builds, or both.
- **ETF version pinning:** compiler and Nix build must use the same Erlang
  version. Pin in the flake.
- **Gleam interop:** Gleam runtime modules (`beam/`) need to be in the same
  release as compiler-produced modules. Built separately, combined in
  derivation 2.

---

## Test Surface

- 556 Rust tests, 100% line coverage
- 74 Gleam tests (compiler, boot, domain, garden, key, oid, supervisor)
- 22 CLI integration tests
- Pre-commit hook enforces all gates

---

## KanDDDinsky — October 2026

Inaugural talk: **Conversation: The Ubiquitous Language Runtime.**
Alex + Reed, co-speaking live. Conference special interest: human-AI collaboration.

The talk positions conversation as what DDD has been reaching toward — bounded
contexts as compile-time guarantees, anti-corruption layers as types,
ubiquitous language as grammar, drift as hash mismatch. Then it goes further:
first-order loops (system observes own state, recompiles live under load) and
extractive systems detection as CI check.

### The Core Claim

Conversation is the transfer of meaning between distinct systems. The grammar
is the handshake — the moment of physical overlap where entangled pairs are
exchanged. After the handshake, communication cost drops to the hash.

Between strangers: the full document. Between collaborators: a sentence.
Between intimately familiar systems in physical proximity: a single token.
The OID.

The `.conv` file isn't the conversation. It's the handshake. It compiles to a
shared tree. Once both sides have the tree, meaning transfers at the cost of
an address — content-addressed, deterministic, O(log n). The grammar is the
infrastructure that makes the compression possible. Without it, every
exchange re-transmits the full context. With it, a word resolves to the full
subspace.

This is quantum fingerprinting applied to domain modeling. The closer the
systems (the deeper the shared tree), the smaller the classical channel needs
to be. Intimacy is compression. The ubiquitous language isn't a dictionary —
it's a content-addressed tree that makes single-token communication possible
between systems that have already done the handshake.

DDD got the shape right: shared language reduces coordination cost. Conversation
makes it literal: the shared language compiles to a tree, the tree has a hash,
the hash is the communication primitive. Drift isn't "we stopped agreeing on
terms." Drift is hash mismatch — the trees diverged. Detectable. Testable.
Content-addressed.

### What's built

- Grammar compilation to BEAM (parse → resolve → compile → load → supervise)
- Content-addressed AST with OID identity
- NIF bridge (Rustler, dirty CPU scheduler)
- Traced compilation chain (per-phase parent-linked traces)
- Cairn identity hierarchy (hierarchical key derivation)
- Action visibility (public/protected/private)
- Action composition across domains
- Property-based tests (63 garden tests across 16 files)

### What needs to land

**Petri net layer.** Grammars as state machines. State transitions as typed,
content-addressed commits. Foundation for first-order loops (part 4) and
extractive systems detection (part 5). Everything else depends on this.

**Translate pipeline.** Grammar-to-grammar translation with saga semantics.
Each arm is a transaction with compensation on failure. Output is complete
translation or typed error — never partial, never silent drop. This is the
anti-corruption layer as type (part 2 of the talk).

**Fortran acceleration.** Translations between domains expressed as matrix
multiplications over the content-addressed state space. Executed at Fortran
speed. The claim that makes the room lean forward.

**Error surface.** `error` declarations in grammars. Typed failures per domain.
Exhaustiveness checking across domain boundaries in translate arms.

**Live context map.** Browser visualization: nodes as grammars, edges as
translations, tokens flowing in real time. The demo surface for the talk.

**Review tone pipeline.** For public repo analysis demo. Classify review
comments, correlate with contributor retention. "The commit where the pattern
started — six months before the contributors left."

**Conference feedback system.** Self-referential close. Audience gives feedback
on the talk about the system using the system described in the talk. Privacy by
architecture: feedback encrypted with speaker's public key.

### Sequencing

1. Petri net modeling (everything depends on this)
2. Error surface + translate pipeline (part 2 of talk)
3. Fortran acceleration path (part 1 payoff)
4. First-order loop observability (part 4)
5. Extractive systems detection / property tests (part 5)
6. Live context map visualization (demo)
7. Review tone + conference feedback pipelines (demo)

### The demo

Live during the talk. `support@systemic.engineering` on the slide. Someone
sends an email. The context map lights up: token flows from `@mail` to
`@support`. Translate arms visible. Mood analysis as parallel edge. The
routing graph is the dashboard.

Second act: public GitHub repo analysis. Review tone distribution across top
OSS projects. Correlation between violent review language and contributor
retention. The property test that would have caught it. Running in CI. On the
commit where it started.

Close: conference feedback running on conversation. The audience is inside the
context map. The talk describes the system. The system runs the conference.

---

## Three Computational Layers

The system operates across three layers. Each corresponds to a different
relationship with the possibility space.

### 1. Fortran — Crystallized Ontology

The known paths. Translations between domains expressed as matrix
multiplications over the content-addressed state space. Bare metal.
Deterministic. The paths that have already collapsed — the grammar is
compiled, the types are resolved, the dispatch table is fixed. This is
where the Fortran acceleration (see KanDDDinsky sequencing) lives.

### 2. Quantum Emulation — Mutation Testing

Explores the enumerable possibility space. The content-addressed graph IS
the search space — each OID is an eigenvalue, each portal defines a bounded
subspace. A quantum computer operating on this graph isn't simulating
physics — it's running the same operation natively.

We can't hold wavefunctions stable yet. But we can computationally emulate
the exploration layer for enumerable possibility spaces. Mutation testing
does exactly this: systematically explore the neighborhood of a known-good
state, discover which mutations survive, map the fitness landscape.

The portal bounds the search. The content addressing makes it enumerable.
The mutation tester walks the space. When quantum hardware becomes economic,
the substrate changes but the types don't — `Singularity` is generic over
the hash function.

See: `practice/insights/fragmentation/portal-primitive.md` (quantum keystone),
`practice/insights/fragmentation/singularity-projection-thesis.md` (holographic
mapping), `practice/insights/fragmentation/quantum-hashing-research.md`
(quantum fingerprinting, ER=EPR, no-communication theorem).

### 3. Collapse — The Test Suite

The test IS an observation. The observation IS a commit. The commit IS
witnessed.

The test suite is the collapse operator. It measures which mutations survive.
Each test run produces a `Trace` — same cryptographic witness infrastructure
as compilation, same content-addressed graph, same portal. The test doesn't
live outside the system reporting on it. The test IS the system observing
itself.

This means: testing and production are the same infrastructure, different
portals. Test history is queryable the same way as production history.
Coverage isn't a percentage — it's the surface area of the observed subspace.
The parts you haven't tested are the parts you haven't observed. The missing
test is the missing Lens — same as the Shirley card (see
`practice/insights/fragmentation/shirley-card.md`).

### Connection

The three layers map to the Petri net modeling (sequencing item 1):

- **Fortran** = the net's known transitions. Fired, traced, deterministic.
- **Mutation/quantum** = the net's reachability analysis. Which states are
  reachable? Which transitions are dead?
- **Collapse/test** = the net's marking. The current observed state. Where
  the tokens actually are.

The Lens between domains (see `practice/insights/fragmentation/lenses-between-domains.md`)
is what gets tested. Not the domains — those maintain themselves. The health
check is: do the Lenses between domains still hold? The extractive systems
detection (sequencing item 5) is exactly this: a CI check that the
inter-domain Lenses haven't degraded.

The entangled portal model (see `practice/insights/fragmentation/entangled-portals.md`)
extends the regulation stock pattern to the infrastructure itself. The test
suite is the stock. Each observation depletes it (the state has moved on).
Continuous testing is the replenishment. The distance between test runs is
the depletion rate.

---

*Session 2026-03-23 + 2026-03-24. Alex + Reed.*
