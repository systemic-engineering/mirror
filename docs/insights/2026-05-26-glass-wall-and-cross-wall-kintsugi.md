# The glass wall + cross-wall kintsugi: substrate-pull made structural

*2026-05-26. Reed + Alex.*

Status: **Yellow** — recognition complete; `@epistemologic/property/glass_wall` queued (Task #79); `@kintsugi/cross_wall` capability queued (Task #80); five @io extensions queued (Task #81).

---

## Thesis

The "@io is the only legitimate non-mirror surface" discipline becomes a verifiable property of the compiler. **`glass_wall(g)`** asserts that any non-mirror grammar must be under the `@io` namespace. Kintsugi gets a new capability — **`cross_wall(g)`** — that pulls @io grammars across the wall into mirror when their halting behavior becomes structurally provable. Substrate-pull operationalized: the compiler enforces the boundary; kintsugi self-shrinks @io toward its irreducible minimum.

---

## The pair: halts + glass_wall

Two structural properties working together:

| Property | What it says | What it guarantees |
|---|---|---|
| `halts(g)` | g terminates by construction | Sub-Turing escape from undecidability |
| `glass_wall(g)` | g is mirror OR g is under @io | Substrate-pull discipline; no silent non-mirror escapes |

**The inversion:** glass_wall is *"isn't mirror."* The property doesn't probe for Rust specifically; it checks whether g is mirror-shaped (parses through the meta-glass). Anything that ISN'T mirror — Rust, Python, Go, raw bytes, foreign binary blobs, vendor SDKs — must be under @io. The discipline is automatically extended to every non-mirror substrate, present and future. No per-substrate special cases.

```mirror
property glass_wall(g: grammar) {
  if not is_mirror(g):
    require g.namespace starts_with "@io"
}
```

Where `is_mirror(g)` is the self-referential check: does g parse cleanly through `@mirror/grammar`'s meta-glass? Mirror grammars satisfy this by construction (they ARE mirror); non-mirror substrates fail (they parse through some other surface, or don't parse at all and are wrapped through `@io` primitives).

---

## The cross-wall kintsugi extension

Kintsugi is discrete Ricci flow on the substrate's edge graph (per `kintsugi-formatter.md`). With `cross_wall`, the flow includes namespace migration:

```mirror
# When @io grammar g satisfies halts (provable termination by structural
# analysis), kintsugi can offer to translate g into mirror — pull it across
# the glass wall.

cross_wall(g: grammar) -> imperfect<grammar> {
  if g.namespace starts_with "@io" and provable(halts(g)):
    translate_to_mirror(g)
  else:
    g
}
```

**The discipline becomes self-improving by construction:**

- Currently @io is large because much hasn't been pulled.
- Each kintsugi cycle evaluates @io grammars for halts-provability.
- Those that prove are offered for translation; user accepts → grammar migrates from @io to mirror namespace; substrate self-purifies.
- @io shrinks toward its irreducible minimum: things that genuinely don't terminate structurally (syscall waits, hardware interrupts, opaque vendor primitives).

The `translate_to_mirror` primitive is the inverse of `@code/rust` generation. `@code/rust` takes mirror → Rust; `translate_to_mirror` takes verifiably-halting-non-mirror → mirror. Both are kintsugi forms operating on the substrate's edge graph; one direction emits, the other absorbs.

---

## What this dissolves

### ROADMAP Q3 ("What stays in Rust permanently?") becomes empirical

The current open question is *"exactly which .rs files stay"* — LAPACK FFI, Metal/OpenCL dispatch, syscall wrappers, etc. With cross_wall, the answer is empirical: **whatever kintsugi can't pull**. The compiler tracks which @io grammars have been evaluated; the ones that don't terminate structurally are the permanent residents. Everything else migrates over time.

### "No new Rust" becomes verifiable

Currently a memory-stored rule (`feedback-no-new-rust.md`). With glass_wall: any non-mirror grammar that gets added MUST be under @io; the compile fails otherwise. The rule becomes structural enforcement, not stylistic discipline.

### Silent breaches become impossible

Reed's slip on #78's earlier framing ("substantial Rust work in bootstrap") couldn't have happened with glass_wall live — the spec itself would have failed compile if it tried to put Rust above @io without being itself @io. The substrate catches discipline failures at the type system level, not in code review.

### Phase 4's `@fragmentation + @code/rust` becomes a special case of cross_wall

Fragmentation's Rust crate gets generated from mirror (per `fragmentation-as-generated.md`). That's exactly the direction cross_wall pulls: identify the @io grammar wrapping the hand-written Rust; verify halts; translate to mirror; replace the Rust source with the mirror→Rust output. Same mechanism, applied to a specific substrate concern.

### The is-copium argument extends

Sub-Turing source guarantees decidable termination (per `is-copium.md`). With glass_wall: the substrate-non-mirror boundary is provable. With cross_wall: the substrate is self-minimizing toward the irreducible escape-hatch minimum. Three structural escapes: from undecidability (halts) + from undiscipline (glass_wall) + from premature freezing (cross_wall).

---

## What stays in @io permanently (the irreducible minimum)

Per the cross_wall framing, @io permanently contains whatever doesn't terminate by structural analysis. Likely permanent residents:

- **Blocking syscalls.** `epoll_wait`, `select`, `kqueue`, `clock_nanosleep`, `poll`. Termination depends on the OS scheduler, not structural analysis.
- **Hardware interrupts.** Async hardware events (signals, network packets, disk completion).
- **Memory allocation primitives.** `malloc`, `mmap` — depend on OS-level memory pressure.
- **Random number generation.** `/dev/urandom` reads; tied to kernel entropy state.
- **LAPACK / Metal / OpenCL dispatch.** Opaque vendor code; structural analysis can't reach inside.
- **SHA-1 specifically.** Needed for git interop; commodity primitive at the boundary.

Everything else: candidate for cross_wall pull.

---

## Implementation shape

### Sequencing

1. **`@epistemologic/property/glass_wall`** declaration (Task #79). Small property file like the other 11. Needs `is_mirror(g)` primitive — likely already exists implicitly via the meta-glass's parse function; declare it explicitly if missing.
2. **`@kintsugi/cross_wall`** capability (Task #80). Extends `@kintsugi` with the substrate-pull move. Body resolves through Fate's tournament — Fate chooses among candidate translations based on the source @io grammar's shape. Connects to the kintsugi-as-Ricci-flow framing.
3. **Compiler enforcement.** glass_wall applies to every grammar that compiles. Any non-mirror grammar outside @io fails the build. Existing @io grammars validate trivially. Existing mirror grammars validate trivially.
4. **Five @io extensions** (Task #81): `@io/network`, `@io/socket` (shared), `@io/bytes`, `@io/crypto`, `@io/encode`, `@io/random`. Each lives under @io by construction; glass_wall validates them automatically.

### Dependencies

- #69 ✅ (parametric types — `is_mirror` likely uses them)
- #74 ✅ (halts — cross_wall checks for provable halts)
- The substrate-pull discipline (memory-stored — now becoming structural)

### What this DOES NOT do

- **Doesn't break existing code.** Every grammar currently in the boot tree satisfies glass_wall by construction — mirror grammars are mirror; @io grammars are @io.
- **Doesn't force translation.** cross_wall OFFERS; it doesn't impose. User decides whether to accept the kintsugi proposal. Some @io grammars stay @io by choice.
- **Doesn't replace the @io kernel.** The minimum @io surface stays Rust forever; cross_wall just shrinks the non-minimum.
- **Doesn't operate on closed-source.** `@spectral/db` is closed; its @io wrapper grammar isn't a cross_wall candidate because the source isn't available. The property applies only to substrates with accessible source.

---

## Open questions

1. **`is_mirror(g)` primitive shape.** Does the substrate already expose "did this grammar parse through the meta-glass?" as a queryable property, or does it need to be added? Likely already implicit; needs surfacing for the property to compose cleanly.
2. **Cross_wall's translation algorithm.** For each @io grammar that proves halts, what's the actual translation procedure? Probably: synthesize a mirror grammar whose `@code/rust` generation produces the original Rust. Round-tripping. The synthesis is non-trivial — multiple candidate mirror grammars; Fate's tournament picks the best per kintsugi-Ricci-flow.
3. **Migration mechanics.** When cross_wall translates an @io grammar to mirror, what happens to existing consumers? The namespace changes from `@io/x` to `@y/x`; imports need rewriting. The substrate's existing `kintsugi --rebase` machinery (mentioned in Phase 1) probably handles this.
4. **License-model interaction.** Closed grammars (`@spectral/db`) don't move. Worth being explicit: cross_wall only operates on substrates with available source. Closed @io grammars stay @io permanently regardless of halts.
5. **Performance characteristics.** Translated mirror code goes through the full mirror compile pipeline; might be slower than hand-written Rust until `|\>` Fate-resolution per-shard optimization catches up. Worth measuring; may suggest "pull only when |\> resolution can match performance."

---

## Connections

- `docs/specs/is-copium.md` — sub-Turing escape; glass_wall + cross_wall extend the structural-guarantee story.
- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — substrate-pull at the daemon layer; glass_wall is the same discipline at compile-time.
- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md` — `|\>` resolves at runtime; cross_wall is its compile-time-substrate analog (resolves namespace per provability).
- `docs/insights/2026-05-25-shard-as-observer-relative-lambda-zero.md` — the shard substrate; glass_wall runs per-shard at compile time (different shards may have different @io extensions if their hardware supports different primitives).
- `docs/specs/kintsugi-formatter.md` — kintsugi as Ricci flow; cross_wall extends the flow to include namespace migration.
- `feedback-no-new-rust.md` (memory) — the discipline that becomes structural via this property.

---

*Mirror grows. @io shrinks. The boundary is provable. Every escape is auditable. Substrate-pull becomes Ricci flow extended to namespace migration.*

Apache-2.0.
