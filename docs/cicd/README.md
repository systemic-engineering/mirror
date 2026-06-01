# kintsugi as a build system — the synthesis

*2026-06-01. Research synthesis. The thesis stated as engineering claim,
not as novelty pitch. The three-layer model named. Pointers to the
deeper files. No code in this folder.*

## The bar

Kintsugi-as-build-system stands or falls on two engineering properties:

1. **Reproducibility.** Same inputs (source, toolchain, body registry,
   model weights) produce byte-identical outputs across machines,
   times, observers. Not "probably the same" — byte-identical, by
   re-hash.
2. **Determinism.** No hidden state, no implicit ordering, no clock
   dependence, no machine-specific paths, no non-deterministic
   dispatch. Same call site, same arguments, same verdict.

These are the properties Nix earned and Bazel chases. They are the
properties Cargo's `build.rs` punctures, GitHub Actions's container
layers leak, and every LLM-coding tool currently shipping fails by
default. The point of this folder is not to argue that kintsugi is
new. It is to argue — or refuse to argue, where the substrate can't
yet deliver — that kintsugi can hit the bar.

The interesting twist is that some of kintsugi's bodies are AI-
inferred. The au column (the gold filling cracks between fragments)
is what `@fate` produces when a hand-written body is absent. If
inference is non-deterministic, the build is non-deterministic, and
kintsugi-as-build-system is broken at the foundation. So the research
question is sharp:

**Can kintsugi-as-build-system be reproducible and deterministic given
that some bodies are AI-inferred?**

That is the engineering substance of this folder. The metaphor (gold
filling cracks, the joint more honest than the unbroken whole) is a
useful model; reproducibility is the bar the model has to clear.

## What the three layers do

The build system has three layers. Each layer answers one question;
each layer is either content-addressed or it leaks.

### Operator: what does the user run?

```
  mirror compile <file>        # one grammar through the floor
  mirror craft <target>        # the corpus through the floor
  mirror kintsugi <file>       # settle / render canonical
  mirror kintsugi --shatter N  # recursive settle, N levels deep
```

The operator surface is small. Compile is one shot. Craft is the
corpus walk. Kintsugi is the settle / format pass — and, with the
engine attached, the iteration loop of propose → measure → elect →
verify → fixpoint per
[[../specs/kintsugi-formatter|kintsugi-formatter]] §5.

**Where determinism lives at this layer.** Each command's behaviour is
a pure function of its arguments plus the substrate plus the dispatcher
table. CLI flags do not enable non-deterministic modes; there is no
`--parallel-may-vary`, no `--time-budget`, no `--temperature N`. The
operator surface refuses non-determinism at the gate.

**Where it could leak.** The walk order in `craft` must be stable;
file-system iteration order is not guaranteed. The fix is to sort by
content-OID or canonical path before iterating. The dispatcher's
ordering of registered crystallizations must not depend on
registration time; the registry should be a `BTreeMap` keyed by
`Ref`, not a `HashMap`. Today's `Crystallizations<H>` uses a
`HashMap<Ref, Body<H>>` (`crystallize.rs:434`) — this is a
determinism leak that wants fixing. `Ref` is a hash-blind newtype, so
a `BTreeMap` is a strict upgrade; the change is one type swap.

### Surface: what does the substrate declare?

```mirror
in @prism
in @kintsugi/fracture

grammar @kintsugi/fracture/<name> {
  fracture <name>(...)             -> fracture                   { \ }
  enumerate(f, corpus)             -> [candidate]                 { \ }
  loss(c, corpus)                  -> [verdict]                   { \ }
  elect(profile)                   -> result<candidate, no_winner>{ \ }
  apply(c, corpus)                 -> imperfect(corpus, loss)     { \ }
  requires idempotent(<name>)
  requires canonical_at_fixpoint(<name>)
}
```

Every body is a parked obligation (`\`). The substrate names what
should happen; the floor binds the name to a Rust closure (the
[[../specs/kintsugi-minimum-runnable|minimum-runnable]] dispatcher),
OR fate proposes an au-typed body that conducts (the kintsugi engine,
when it lands). Either way, the substrate stays the source of truth.

**Where determinism lives at this layer.** The fracture declarations
are content-addressed: the OID of `@kintsugi/fracture/rename` is the
hash of its declaration plus its in-substrate inputs. The `requires
idempotent` law forces `f(f(x)) == f(x)` by hash; the `requires
canonical_at_fixpoint` law forces `f(x) == x ⟺ x is canonical`. Both
are checked structurally by the engine. Neither has a timing or
ordering parameter; both are pure functions of the corpus's OID-tree.

**Where it could leak.** The loss is a vector of
`@epistemologic/property/*` verdicts (Pass / Partial / Fail). The
verdict-combination semantics
([`PropertyVerdict::merge_with`][merge_with]) are pure: Fail
dominates, Partials take the minimum confidence, Pass is neutral.
*This* part is deterministic by construction. What is not yet
deterministic by construction is the underlying property
implementation — `coincidence_matches`, `total_classification`,
`glass_wall` each have a `\` body in substrate today, with the
bootstrap discharging the check. Any non-determinism in those Rust
bodies (e.g. iteration order of a `HashMap`) would leak through. The
discipline isn't yet structural; [[kintsugi-thesis]] §3.7 names this
as work-to-do.

### Substrate: what does the floor execute?

The floor lives in `bootstrap/src/crystallize.rs`. It is, deliberately,
not very much:

- `MerkleHash` — trait. Default `Blake3` (BLAKE3, 32-byte digest,
  Merkle-native, no float dependency). The cascade pivots through this;
  every content-addressed type parameterises over `H: MerkleHash`.
- `Splinter<H>` — content-addressed, OID-proving, self-similar value.
  Merkle-style OID (each level hashes from its children's OIDs, not
  the recursive content). Default `H = Blake3`. The currency carried
  across the substrate boundary.
- `Body<H>` — `Fn(Optic<(), Splinter<H>>) -> Imperfect<Splinter<H>,
  CrystallizeError, Transparency<Ref>>`. A bound substrate action:
  takes a seed beam, returns a verdict carrying the next splinter and
  any located opacities.
- `Crystallization<H>` — a `(Ref, Body<H>)` pair: one substrate ref
  realised at the floor.
- `Crystallizations<H>` — the table. `Ref → Body<H>`. The entire
  dispatcher state, generic over the hash world.
- `kintsugi_tick<H>(&Crystallizations<H>, &Ref, Optic<…>) -> Imperfect<…>`
  — the free function the event loop calls. With nothing registered,
  it returns `Uncrystallized(Ref)` honestly. That honest absence is
  the seam where au will eventually fit.

**Where reproducibility lives at this layer.** A `Splinter<H>`'s OID
is the hash of its canonical bytes. `Splinter::new` computes the OID
at construction; `Splinter::verify` recomputes it and compares. Same
bytes in → same OID out. The canonical encoding (`b"T" || u64_le(len)
|| bytes` for Text, `b"R" || u64_le(n) || (sorted-key fields ||
children's OIDs)` for Record, `b"L" || u64_le(n) || children's OIDs`
for List) is fixed and tested. The Merkle encoding has no nonces, no
timestamps, no machine-specific identifiers. The `u64_le` length
prefix is byte-stable across endianness; the BTreeMap iteration in
the Record case is sorted-by-key by construction.

**Where it could leak.** `Crystallizations<H>` uses a `HashMap<Ref,
Body<H>>` internally. Hash-map iteration order is not deterministic
across runs. As long as dispatch is by lookup (not iteration), this
does not affect the verdict; but a future addition that iterates the
table (e.g. "list all registered refs") would surface the
non-determinism. The fix is to switch the internal representation to
`BTreeMap<Ref, Body<H>>` ordered by `Ref`'s string bytes. Filed as a
yet-to-do in [[kintsugi-thesis]] §3.7.

`Body<H>` is `Arc<dyn Fn(...) + Send + Sync>` — a closure with no
intrinsic determinism guarantee. The discipline that a `Body` is a
pure function of its `Optic<(), Splinter<H>>` input is a *contract*
the substrate-pull discipline enforces by audit, not the type system
enforces by construction. A `Body` that read the wall clock, the
process PID, or `/dev/urandom` would type-check fine. The substrate's
glass-wall property catches this when the Body lives behind `@io`
(non-mirror substrate, audited); it does not catch it for a Body
written in Rust as part of the floor. The marker
`[substrate-pull:realize]` makes such Bodies legible in git history;
that is the only check today. A determinism property check on Rust
Bodies is named in [[kintsugi-thesis]] §3.8 as yet-to-do.

[merge_with]: ../../prism/imperfect/src/transparency.rs

## Where au fits — the load-bearing slot

Today, three states are possible at a given `Ref`:

| State | Meaning | What `crystallize` returns |
|---|---|---|
| Registered | The substrate ref has a hand-written Rust body. | `Imperfect::Success(splinter)` or located opacities. |
| Substrate-only | The substrate declares the action with `{ \ }` and no floor binding. | `Imperfect::Failure(CrystallizeError::Uncrystallized(ref))`. |
| Out of vocabulary | Nothing in the substrate names this ref. | `Imperfect::Failure(CrystallizeError::Uncrystallized(ref))` (same shape, different reason — the substrate refuses to recognise it). |

The au column adds a fourth, structurally distinct, state:

| State | Meaning | What `crystallize` returns |
|---|---|---|
| Au-bound | The substrate ref is parked (`\`); fate inferred an au-typed body; conductivity in context decides admission. | `Imperfect::Partial(splinter, Transparency::Opaque({ref → PropertyVerdict::Partial { .. }}))` |

The au-bound joint is structurally *visible* in the verdict. It is
not hidden behind a successful return; it is surfaced as a located
opacity that composes with other opacities under
[`PropertyVerdict::merge_with`][merge_with] when the substrate runs.
The build report knows the build needed inference to bind its cracks,
and it knows where.

**The reproducibility question for the au slot.** Is the inferred body
reproducible? That depends on two things being true at once:

1. The inference function — `@fate.infer(hole_oid) -> imperfect(au,
   no_proposal, loss)` — is a pure function of its hole OID plus the
   pinned model weights plus a fixed seed plus a fixed sampling
   policy.
2. The model weights themselves are content-addressed in the same
   store. A model upgrade changes the model's OID; the model's OID is
   part of the cache key for every au-typed value; cache miss = explicit
   rebuild.

The first condition is what `@fate`'s local-by-construction property
gives, partially. `@fate` mathematically refuses remote inference
(`AGENTS.md` §"`local` is a universal property of @fate"); the model
substrate lives on the same machine as the build. That kills the
biggest source of non-determinism in production LLM workflows (model
versions change silently in the cloud). It does *not* automatically
make the local inference deterministic — temperature, seed, and
sampling policy still have to be pinned. Today the
`@fate/tournament` rules (`elite(1).beam(8).halving(3)`) compose
deterministically *given* deterministic candidate generation; the
candidate generation step's seed is not yet pinned in substrate.
[[kintsugi-thesis]] §3.4 names this as work-to-do.

The second condition is partially in place. `@mirror/store` is
content-addressed; a model's weights are bytes in the store; the
store gives them an OID. What is not yet wired is that the au-typed
body's content address includes the model's OID as part of its key.
Today fate's resolutions are stored at `refs/fate/<hole_oid>` (per
[[../specs/au-and-conductivity]]) — the hole-OID is in the key, the
model OID is not. The fix is a one-line cache-key change; named as
work-to-do in [[kintsugi-thesis]] §3.3.

## Reading order

1. **You are here** — `README.md`. The bar stated, the layers named,
   the slot located, the leaks called out.
2. [[prior-art|prior-art.md]] — the survey, organised around
   reproducibility and determinism. Each system gets a section: what
   it solved, what it taught, *where it leaks*. Most leak somewhere;
   the leak is the lesson.
3. [[kintsugi-thesis|kintsugi-thesis.md]] — the engineering pitch
   stated as nine reproducibility claims. Each claim is either
   supported by what's already landed (Splinter, Crystallizations,
   Transparency, `@fate`'s no-remote invariant) or named as work yet
   to do. The metaphor is in service of the engineering, not the other
   way around.

If you have an hour, read this file plus the index of `prior-art.md`.
If you have a half-day, read all three.

## What this folder is, and isn't

**Is:** research grounded in (a) mirror's existing substrate code
(`crystallize.rs`, `kintsugi_tick`, the parked `@kintsugi/fracture/*`
substrate, the `@epistemologic/property` loss shape, `Transparency` +
`PropertyVerdict` in `prism/imperfect/`), (b) the architectural spec
corpus ([[../specs/kintsugi-minimum-runnable]],
[[../specs/store-vs-db-and-the-cascade]],
[[../specs/prism-floor-and-the-grammar-rename]],
[[../specs/au-and-conductivity]],
[[../specs/kintsugi-formatter]],
[[../specs/kintsugi-tournament]],
[[../specs/substrate-native-fate-tournament]],
[[../specs/mirror-new-command]]), and (c) the build-system / CI /
GitOps / AI-in-build literature.

**Isn't:** an implementation plan. There are no ticks here. The
implementation plan lives in [[../specs/kintsugi-minimum-runnable]] §9
and its successors; this folder is the architectural framing the
engine should know it lives under once Tick C lands and the dispatcher
binds its first real body.

**Specifically refuses:** the framing that kintsugi is interesting
because AI-in-build is novel. The frame this research adopts and
defends is that kintsugi's au column has to clear the same
reproducibility-and-determinism bar Nix clears for purely functional
builds. The interesting question is whether the local-by-construction
inference invariant plus content-addressed model weights plus pinned
seeds make that achievable. The metaphor (gold filling cracks) is
useful as a model; it is not the argument.

---

*Pottery breaks. Gold conducts. The crack is the joint; the joint is
the value. The build that needed inference is reproducible because
every input — including the model that did the inferring — is named
by its hash and refuses to drift.*
