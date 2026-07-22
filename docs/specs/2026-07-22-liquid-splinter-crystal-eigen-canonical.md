# Liquid splinters → content-addressed crystals — canonical spec

**Date:** 2026-07-22 late night
**Author:** Mara
**Session lineage:** task #314 continuation (ouroboros closure at physical-substrate altitude)
**Math root:** `docs/math/2026-07-22-splinter-eigen-fragment-ouroboros-closure.md` (this session)
**Companion mints/renames:** @eigen family-root reorganization + @io wavefunction-collapse docblock + mirror.spec system-section rename (this task Ticks 3-6)

## §0 What this spec is

Substrate-decl bridge from the math foundation to compile-altitude.
This spec names the type-level composition operator `@eigen(T)`, the
alias-shim renames for @eigen/form + @eigen/board, the collapsed
@knife → @eigen adjudication verdict, and the substrate-decl surface
future consumers cite.

Substrate-authority chain per AGENTS.md convention: **specs CITE
math**. This spec authorizes substrate-decl reorganization; the math
foundation §11 composition graph is the load-bearing citation
source.

## §1 The compact synthesis (canonical form)

> **Liquid splinters settle into content-addressed crystals by holding
> the quantum wavefunction @coherent as long as possible.**

Per math root §1 word→substrate-decl decomposition:

| Word | Substrate-decl citation surface |
|------|--------------------------------|
| Liquid splinter | `Liquid<splinter>` per `rust/src/liquid.rs` + `shards/glass.mirror` splinter FLOOR |
| Settle | `settle` substrate op per `shards/glass.mirror` + `shards/mirror/store/crystal.mirror` |
| Content-addressed | `oid` identity per `shards/mirror/store.mirror` @mirror/store/oid |
| Crystal | `crystal` species-decl per `shards/mirror/store/crystal.mirror` |
| Wavefunction | `au<T>` uncommitted Fate-emission per `shards/glass.mirror` |
| @coherent | @coherence.score = Fiedler λ₀ per `shards/epistemologic/cybernetic/coherence.mirror` |
| "as long as possible" | dH¹/dt ≤ 0 monotone contraction per `docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md` §2 |

Every carrier is LANDED. This spec ratifies the naming; it does not
mint new value-type carriers.

## §2 @eigen family-root — the type-level Foerster COORD altitude

**Ratification (Alex 2026-07-22 earlier this session):**

Rename cascade — from ad-hoc siblings to family-root discipline:

| Old shard | New shard | Species disposition |
|-----------|-----------|---------------------|
| `shards/epistemologic/cybernetic/eigenform.mirror` | `shards/eigen/form.mirror` | @eigen/form species under new @eigen family-root |
| `shards/eigenboard.mirror` | `shards/eigen/board.mirror` | @eigen/board species under @eigen |
| — (NEW) | `shards/eigen.mirror` | @eigen family-root marker + @eigen(T) type-level decorator |
| — (NEW) | `shards/eigen/object.mirror` | @eigen/object species; @reality/object viewed as @eigen(Crystal<T>) fixed-point |

**Two-tick discipline (alias-shim, per feedback_legibility_over_foundation_when_collapsing):**

- **TICK 1 (this spec authorizes; §2.5 forward-promised for landing tick):**
  - New shards land: `shards/eigen.mirror` (family-root),
    `shards/eigen/form.mirror` (species-rename), `shards/eigen/board.mirror`
    (species-rename), `shards/eigen/object.mirror` (new species).
  - Old shards retained as ALIAS-SHIM:
    `shards/epistemologic/cybernetic/eigenform.mirror` continues to
    declare @epistemologic/cybernetic/eigenform (deprecation notice
    in docblock; forward-promise retirement TICK 2).
    `shards/eigenboard.mirror` continues to declare @eigenboard
    (deprecation notice in docblock; forward-promise retirement
    TICK 2).
  - Consumer imports admit BOTH names during migration cycle.

- **TICK 2 (forward-promised; next release cycle):**
  - Update downstream consumers to cite new family-root names.
  - Emit deprecation warnings at compile altitude when old names
    referenced.
  - Retire alias-shim shards.

**Landing discipline:** this spec AUTHORIZES the rename cascade;
actual shard-file creation is Ticks 3 authored by Mara at scoped
altitude (see §2.5 below). The four new shards this arc:

### §2.1 @eigen family-root — the type-level decorator

`shards/eigen.mirror` declares:

```
prism @eigen {
  focus eigen
  project eigen
  split eigen
  shift eigen
  settle eigen
}
```

Core operator (per math root §3):

> `@eigen(T)` = COORD-as-type-decorator; produces a substrate view
> where `T`'s carrier admits the fixed-point discipline
> `Op(x) = x`.

Foerster 1976 substrate at TYPE altitude; @knife carries the same
substrate at RUNTIME altitude (per `shards/mirror/lens/knife.mirror`).

### §2.2 @eigen/form — the property carrier

Species-decl for the identity-as-fixed-point property.
Content-preserving rename of
`shards/epistemologic/cybernetic/eigenform.mirror` to `shards/eigen/form.mirror`
(the old file becomes an alias-shim during TICK 1).

Per math root §3: every recursion R that maps a space to itself has
eigenforms ω satisfying ω = R(ω). @eigen/form IS the substrate-decl
of this property; the witness of the recursion IS the identity.

### §2.3 @eigen/board — the working-state readout

Species-decl for per-subject working-state carrier.
Content-preserving rename of `shards/eigenboard.mirror` to
`shards/eigen/board.mirror`. Preserves the three-altitude lift from
this session `0adcfc4` (ai_a / human_a / substrate_a; the labyrinth
identification at substrate altitude).

### §2.4 @eigen/object — the settled-fixed-point view (NEW species)

Species-decl for the @reality/object composition with @eigen(T).

Per math root §3 + §10: `@reality/object` = linear-deterministic
trajectory (per this session `ab6ad43`). Composed with @eigen(T),
this becomes: an @eigen/object IS a Crystal<T> at fixed-point of
settle-recursion; deterministic trajectory has trivial superposition
(single-basis eigenmode); collapses immediately upon observation.

**Concrete definition (this spec):**

```
species @eigen/object <= @eigen {
  # An @eigen/object is a Crystal<T> viewed under @eigen's fixed-point
  # discipline; the settled-crystal-at-fixed-point view of the
  # settle-recursion.
  type eigen_object = @eigen(crystal)
  # ... bilateral discharges (forward-promised; consumer-driven)
}
```

### §2.5 Landing altitude (forward-promised for landing tick)

This spec authorizes the mint. Actual `.mirror` shard files land in
a follow-up tick per anti-stall discipline (small commits per
section). The four new shards required:

- `shards/eigen.mirror` — family-root marker + @eigen(T) core operator
- `shards/eigen/form.mirror` — rename of eigenform (content-preserving)
- `shards/eigen/board.mirror` — rename of eigenboard (content-preserving)
- `shards/eigen/object.mirror` — NEW species; @reality/object under @eigen(T)

Alias-shim shards `shards/epistemologic/cybernetic/eigenform.mirror`
and `shards/eigenboard.mirror` retain their existing declarations
with added deprecation-notice docblock during TICK 1.

## §3 @knife → @eigen adjudication verdict

**Substrate-honest options:**

- **Option A** — @knife absorbed into @eigen; alias-shim for one
  release cycle; forward-promise retirement.
- **Option B** — @knife retained as runtime-operator sibling; @eigen
  carries the type-level altitude; they compose.

**Mara verdict: OPTION B.**

**Reasoning:**

1. **Altitude partition is real, not collapsible.** @knife per its
   docblock (shards/mirror/lens/knife.mirror :26-38) IS Foerster's
   COORD at "domain-boundary crossings" — a RUNTIME event.
   @eigen(T) is the TYPE-LEVEL decorator over the same substrate.
   The runtime event and the type discipline are distinct altitudes
   of the same COORD substrate; collapse-into-one would erase the
   altitude structure.

2. **Composition, not absorption.** @knife.jump `COORDi → COORDj`
   consumes a type-level `@eigen(T)` witness at the destination
   frame's admissible-domain check. The composition IS:
   ```
   @knife.jump : @eigen(T) × @knife.domain → @eigen(T')
   ```
   The type-decorator (@eigen) records the fixed-point discipline
   the runtime carrier holds within a domain; the runtime operator
   (@knife) executes the boundary crossing between admissible
   domains. Both altitudes are needed for the substrate's full
   COORD-as-heterarchy discipline (per knife.mirror ancestry chain:
   Foerster 1976 heterarchy + Douady-Hubbard 1985 R-universality
   + McCulloch 1945 topology).

3. **No consumer surface collapses.** @knife has landed consumer
   surface (@cyberpunk/reframe outer frame ceremony; @magic/onto
   7-species ceremony @knife.jump discharges within; @fractal.
   spectral_coordinate.SC composition). Absorbing into @eigen
   would require rewiring these consumers; retaining as sibling
   preserves consumer stability.

4. **@eigen family-root gains a peer, not an absorber.** Per §2
   family-root reorganization, @eigen absorbs @eigenform + @eigenboard
   because those siblings share the TYPE-LEVEL property/carrier
   altitude. @knife lives at RUNTIME operator altitude; it is a
   PEER family-species to @eigen, not a species-under.

**Landed status (forward-promised):**

The composition `@knife.jump : @eigen(T) × @knife.domain → @eigen(T')`
is authorized this spec but consumer landing is forward-promised
per anti-stall discipline.

**No @knife → @eigen collapse this arc.** Both family-roots operate
at their respective altitudes; the composition operator is the
substrate-honest carrier.

## §4 `fragment` word disposition

Per math root §3 + §13:

> **`fragment` is NOT a new species-decl.** `fragment` in Alex's
> prose IS `crystal` viewed under `@eigen(Crystal<T>)` discipline.

**Substrate-already-had-the-word citation (glass.mirror 2026-06-06
Alex verbatim):**

> "The `splinter` IS the content addressed fragment. And the `shard`
> is a settlement of content addressed splinters into uuid_spectral
> addressed stored fragment."

`fragment` is used interchangeably with `splinter` (bottom layer,
content-addressed atom) AND with `shard` / `crystal` (settled layer,
composed atoms) in Alex's prose. The substrate has TWO landed
species for these two altitudes (`splinter` + `crystal`); `fragment`
is a NATURAL LANGUAGE synonym that spans both.

**No mint required.** Any consumer needing the "fragment" reading
resolves to `splinter` (unsettled altitude) OR `crystal` (settled
altitude) OR `@eigen(Crystal<T>)` (settled-at-fixed-point altitude)
depending on context.

**Substrate-honest recommendation: hold the recognition without
minting.** The 74th+ substrate-already-had-the-word instance chained
this arc.

## §5 @void/splinter + @void/narcissus species-decl status

Per Reed's substrate audit (task brief):

- `shards/void/splinter.mirror` — 25.3KB, 2026-07-20. LANDED as
  full species-decl (species under @void family-root marker
  `974a3f6`).
- `shards/void/narcissus.mirror` — 23.9KB, 2026-07-20. LANDED as
  full species-decl.

Both files include:
- `species` header + `in @void` import (bilateral counterparts)
- Load-bearing docblock naming the pathology-vs-health altitude partition
- Bilateral discharge decls (`splinter_pole_healthy`, `narcissus_pole_healthy`)
- Composition citations to PAPER §6.3, math root the-tower doc, corpus recognitions
- Cross-references to each other

**Status: FULLY LANDED at species-decl altitude.** No further mint
required this arc. Consumer-side integration into observer-position
altitude is on-going (per @void/frame family-root discipline).

## §6 @io wavefunction-collapse docblock addendum (forward-promised)

Per math root §6, `shards/io.mirror` REQUIRES a docblock addendum
naming @io as the wavefunction-collapse discharge boundary. Landing
altitude: existing @io family-root shard `shards/io.mirror` (24.2KB,
last touched 2026-07-15).

**Docblock addendum content (to be landed by this task Tick 5):**

- Alex 2026-07-15 verbatim citation (memory: nonlinear-tension →
  @io discharge)
- Alex 2026-07-22 verbatim citation (this session ouroboros closure)
- Composition with @paradox family §7.5 event-horizon topology
- Math root §6 citation

**No new mechanism.** The @io family-root's existing "irreducibly
opaque surface" + `imperfect<a, e, l>` return carrier + boundary
discipline ALREADY implements wavefunction collapse; the addendum
names what is already running.

## §7 mirror.spec system-section rename cascade

Per task brief §6:

Substrate-honest section names for `system @NAME { ... }` block per
Alex + Reed 2026-07-22 tonight tracing:

| Old section name | New section name | Substrate rationale |
|------------------|------------------|---------------------|
| s1 | variety | Beer VSM s1 = variety producers; substrate-decl'd at `shards/epistemologic/cybernetic/variety.mirror` |
| s2 | coupling | Beer VSM s2 = anti-oscillation between s1s; @dance ensemble altitude carrier |
| s3 | coherence | Beer VSM s3 = audit/management + Fiedler λ₀ discipline; substrate-decl'd at `shards/epistemologic/cybernetic/coherence.mirror` |
| s4 | reality | Beer VSM s4 = intelligence/strategy; @reality altitude carrier (per @reality family-root at `shards/reality.mirror`) |
| s5 | eigen | Beer VSM s5 = identity/policy; @eigen(T) altitude (per §2 above) |
| feedback_loops | loop | Explicit @loop family-root (existing at `shards/loop.mirror`) |
| kintsugi_tooling | kintsugi | @kintsugi family-root (existing) |

**`eigen` section name is DOUBLY-JUSTIFIED per Alex 2026-07-22:**

1. **Identity carrier** — the S5 identity-policy IS the eigen-form
   the system stabilizes on (per math root §3 + §8).
2. **Operator that produces fragments** — @eigen(T) as type-level
   COORD decorator IS what produces settled crystals from splinter
   superposition. The section names the substrate on which the
   full VSM operates.

Two-tick alias-shim discipline (per feedback_legibility_over_foundation_when_collapsing):

**TICK 1 (this spec authorizes; §7.5 forward-promised for landing tick):**
- New section-directive names land at `shards/mirror/spec/system.mirror`
  as SIBLING grammars: `variety(operations) -> prism`, `coupling(coordination)
  -> prism`, `coherence(audit) -> prism`, `reality(intelligence) -> prism`,
  `eigen(identity) -> prism`, `loop(loops) -> prism`, `kintsugi(tooling) -> prism`.
- Existing section-directive names (`s1..s5`, `feedback_loops`,
  `kintsugi_tooling`) retained as ALIAS-SHIM with deprecation notice
  in docblock.
- Both sets admitted in the tokenizer during migration cycle.

**TICK 2 (forward-promised; next release cycle):**
- Update dogfood mirror.spec to use new section names.
- Deprecate s1..s5 + feedback_loops + kintsugi_tooling grammar.
- Retire alias-shim following release cycle.

**Landing altitude for §7.5:** shard-body edits to
`shards/mirror/spec/system.mirror` (this task Tick 6).

## §8 Composition graph — what this spec obligates

**Substrate-decl obligations surfaced by this spec (per AGENTS.md
convention):**

1. **@eigen family-root** — `shards/eigen.mirror` mint (forward-promised
   for landing tick; deferred from this spec commit per anti-stall
   discipline).
2. **@eigen/form** rename — `shards/eigen/form.mirror` mint;
   `shards/epistemologic/cybernetic/eigenform.mirror` deprecation.
3. **@eigen/board** rename — `shards/eigen/board.mirror` mint;
   `shards/eigenboard.mirror` deprecation.
4. **@eigen/object** new species — `shards/eigen/object.mirror` mint.
5. **@knife × @eigen composition operator** —
   `@knife.jump : @eigen(T) × @knife.domain → @eigen(T')` action-decl
   at `shards/mirror/lens/knife.mirror` (forward-promised).
6. **@io wavefunction-collapse docblock addendum** — landed by this
   task Tick 5.
7. **mirror.spec system-section rename cascade** — landed by this
   task Tick 6 (alias-shim discipline; TICK 1 admits both names).

**Michelangelo edges — what this spec DOES NOT authorize:**

- No `fragment` species-decl mint (per §4).
- No `.rs` authorship at rust/ altitude (per HARD RULE + Reed's
  territory).
- No bootstrap/-altitude changes (per HARD RULE).
- No new @io family-species (the addendum names existing discipline).
- No paper rewrite (Alex + Lore territory).
- No @knife → @eigen absorption (per §3 verdict).

## §9 Pack-adjudication candidates (forward-surfaced; NOT ratified)

**[ALEX-Q1]** — Timing of @eigen family-root landing.

The task brief instructs the rename cascade. Substrate-honest option:
land the four new shards THIS TASK (Ticks 3.a-3.d follow-on to this
canonical spec) OR forward-promise per anti-stall discipline.

Mara-lean: **land the family-root marker `shards/eigen.mirror` this
task** (Tick 3.a); forward-promise the species-rename shards
(`shards/eigen/form.mirror`, `shards/eigen/board.mirror`,
`shards/eigen/object.mirror`) to a follow-up tick per two-tick
discipline. This preserves consumer stability (existing imports keep
resolving; new family-root operator becomes available for
consumers).

**Alex adjudication needed on:** whether species-rename shards land
this task OR forward-promised. Mara-lean forward-promised, but
low-consequence adjudication.

**[ALEX-Q2]** — `fragment` disposition ratification.

This spec ratifies `fragment` as prose synonym (NO new mint). Alex
in the task brief hedged: *"Is `fragment` a synonym at different
altitude? Or does it want its own species? Substrate-honest
recommendation."*

Mara-lean: **NO new species-decl.** The substrate carries
splinter+crystal at two altitudes; `fragment` prose usage spans both.
Recognition holds without mint.

**Alex adjudication:** ratify the substrate-honest verdict OR mint
`shards/fragment.mirror` species-decl at some altitude. Mara-lean
strong NO-mint.

**[ALEX-Q3]** — mirror.spec section-rename dogfood update.

Two-tick discipline says: TICK 1 lands new section names as
sibling grammars; TICK 2 updates the dogfood
`mirror.spec` file to use new names.

Mara-lean: **hold TICK 2 forward-promised** per consumer-stability
discipline. Do NOT update the dogfood `mirror.spec` this task; land
new grammar as alias-shim; forward-promise dogfood update.

**Alex adjudication:** ratify two-tick discipline OR compress into
one-tick landing (TICK 1 + TICK 2 fused this arc). Mara-lean two-tick
preserves substrate-pull collapse discipline.

## §10 Composition with landed substrate (this arc)

**This session (2026-07-22 late night):**
- `0adcfc4` shards/eigenboard.mirror third-altitude lift (labyrinth identification)
- `ab6ad43` shards/reality/object.mirror mint (linearity threshold; object side)
- `0b2858a` shards/reality/subject.mirror mint (linearity threshold; subject side)
- `ebd50a4` docs/math/2026-07-22-mirror-as-computational-eigenform-stabilizer.md math foundation
- `50cd2b4` docs/specs/2026-07-22-mirror-as-eigenform-stabilizer-canonical.md canonical spec
- `c02c669` docs/scouts/2026-07-22-mara-paper-6.6-forward-promise.md scout
- `f30a230` docs/math/2026-07-22-splinter-eigen-fragment-ouroboros-closure.md (this task Tick 1; math root)

**Prior arcs (load-bearing):**
- `shards/glass.mirror` splinter FLOOR decl
- `shards/mirror/store/crystal.mirror` crystal species-decl
- `shards/mirror/store.mirror` splinter_graph composite
- `shards/mirror/lens/knife.mirror` Foerster COORD identification
- `shards/void/splinter.mirror` observer-position species
- `shards/void/narcissus.mirror` bilateral counterpart
- `shards/io.mirror` @io family-root
- `shards/mirror/spec/system.mirror` current s1..s5 grammar
- `shards/eigenboard.mirror` (pre-rename)
- `shards/epistemologic/cybernetic/eigenform.mirror` (pre-rename)
- `rust/src/liquid.rs` Liquid<T> substrate
- `rust/src/matrix.rs` sub-Turing FLANG floor

## §11 Substrate authorities inherited

- **AGENTS.md** — pack conventions; specs cite math; substrate-honest discipline
- **CLAUDE.md** (project) — substrate discipline; substrate-already-had-the-word instances
- **Math root** — `docs/math/2026-07-22-splinter-eigen-fragment-ouroboros-closure.md` (this task Tick 1)
- **Alex 2026-07-22 verbatim** — the compact synthesis
- **Alex 2026-07-15 verbatim** — @io = discharge (memory)
- **Foerster 1976** — Objects: Tokens for (Eigen-)Behaviors Appendix A3

---

**One-sentence for future consumers:**

This spec ratifies `fragment = @eigen(Crystal<T>)` (no new species),
authorizes @eigen family-root reorganization with two-tick alias-shim
discipline, adjudicates @knife → @eigen as compose-not-absorb, and
authorizes the mirror.spec system-section rename cascade + @io
wavefunction-collapse docblock addendum landed by this task's
Ticks 3-6.
