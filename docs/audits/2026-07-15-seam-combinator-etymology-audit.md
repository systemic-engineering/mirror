---
date: 2026-07-15
author: Seam
scope: Etymology audit of the 7-combinator evaluator surface at
  `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
  (18d9697) plus extended-scope sweep of Landing 4/5/6 secrets +
  sheaf action names. Applies the two-part Alex framing ("does the
  name do the thing the geometry says it does? and if not what's
  the word it wants to be?") against the delightfully-boring
  criterion. Reports per-name verdicts, systemic patterns, collapse
  candidates, and the Alex-naming-authority residue.
status: adjudication (adversarial review; no commit; Reed commits
  as Seam after Alex ratification)
companion:
  - docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md
  - shards/io/secrets.mirror
  - shards/io/secrets/sops.mirror
  - shards/subject/visibility/sheaf.mirror
  - shards/mirror/lens/refract.mirror
  - shards/mirror/lens/transit.mirror
  - shards/metalogue.mirror
  - shards/code/metalogue.mirror
  - /Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs
  - /Users/alexwolf/dev/projects/prism/imperfect/README.md
---

# Combinator etymology audit — do the names do what the geometry says?

*Adversarial re-etymology audit of Mara's 7-combinator surface
(Arc-1 Tick 1.1 companion) under Alex 2026-07-15 delightfully-boring
naming discipline. Physical anchor: `@../prism/`. Every name
interrogated against the physics of a beam-through-prism; every
name whose motivation is CS-vocab convenience flagged. Report,
not commit.*

---

## §0 Prelude — the framing Alex landed

### §0.1 Alex verbatim (both questions + the criterion)

The audit trigger, quoted verbatim by Reed in the spawn brief:

> "Does the name actually do the thing the geometry says it does?
> And if not what's the word it wants to be?"

Anchoring criterion:

> "Delightfully boring — the reader ought to go 'of course it's
> this'."

Names should be inevitable. Reader who knows the beam-through-
prism geometry encounters the word → recognition, not surprise.
Boring: no cleverness, no novelty for novelty's sake. Delightful:
the name fits so cleanly there is nothing to marvel at. "Of
course it's this" IS the audit criterion.

Sharper reading of `[[feedback-substrate-already-had-the-word]]`:
substrate-already-had-the-word says the word EXISTS; delightfully-
boring adds the word feels INEVITABLE. CS-vocab convenience is
the failure mode. `dispatch`, `execute`, `read_ast`,
`bench_record`, `emit` — imported wholesale from compiler-CS
convention because nobody questioned them. Must be audited, not
accepted by inertia.

### §0.2 Reed conversation ancestry

Reed surfaced one collapse candidate pre-audit: **`dispatch` (#4)
and `emit` (#6) may collapse into ONE combinator parametrized by
channel type.** Adjudicated in §5.1.

### §0.3 Physical anchor — the prism crate's vocabulary

`@../prism/` at `/Users/alexwolf/dev/projects/prism`. Cross-
referenced this audit against three landed crates:

- **`terni`**: `Imperfect<T, E, L>` = `Success | Partial |
  Failure`; `Loss` monoid; `ConvergenceLoss`, `ApertureLoss`,
  `RoutingLoss`.
- **`prismqueer`**: `Prism` trait with `focus / project /
  settle`; `Beam` carrier; `Optic<In, Out, E, L>`; `apply`
  (end-to-end); `apply_h` (operator action on H —
  heterogeneous); associated types `Input / Focused / Projected
  / Refracted`.
- **`prismqueer-projections`**: derive macros; `emit` used in
  the macro-shim context (`declaration!{}` emits the Rust
  realization).

Physics vocabulary that grounds this audit:

```
Beam Prism Optic  — carrier / three-staged transform / concrete Beam
Imperfect Loss    — ternary result + monoid
Transparency      — mirror-side Loss over (Ref, Verdict)
focus project settle — the three Prism operations
apply apply_h     — run end-to-end / operator action on H
Refracted         — output beam (past participle of refraction)
section           — element of A (algebra of sections over sheaf)
```

Substrate vocabulary that grounds this audit:

```
refract  — measurement-lens family (shards/mirror/lens/refract);
            renamed OPERATION → FAMILY on 2026-06-04
transit  — runtime-cost lens (shards/mirror/lens/transit)
stalk restrict acl_project section_at
         — sheaf-theoretic operations (shards/subject/visibility/sheaf)
turn     — unit of the metalogue (shards/metalogue: speaker +
            body + in_reply_to + tick)
utter (implied by Bateson-verbatim citation) speak
         — conversation vocabulary in shards/code/metalogue
```

Any replacement drawn from this vocabulary is delightfully-boring
by construction: substrate has used the word for months at a
sibling altitude in a load-bearing way.

---

## §1 Method

**Q1 — does the name do the geometry?** State the operation
geometrically (what happens to the beam-through-prism when this
combinator runs); compare to what the name suggests. Same
geometric operation → Q1 passes.

**Q2 — if not, what does it want to be?** Draw candidates in
preference order: (1) substrate words at sibling altitude for
same operation; (2) `@../prism/` physics vocabulary;
(3) mathematical foundation words (eigensheaf.md,
sheaf_laplacian.mirror); (4) new coinage (LAST RESORT).

**Delightfully-boring test.** Would a reader — substrate +
physics knowledge, no prior exposure to THIS combinator — go
"of course it's this"? Or need explanation of why THIS word for
THIS operation? Needing explanation IS the failure. Delightfully-
boring names carry their own justification.

**Adjudication categories.** DELIGHTFULLY BORING (no change) /
CS-VOCAB CONTAMINATION (propose replacement + cite grounding) /
AMBIGUOUS (needs clarification) / COLLAPSE CANDIDATE (propose
collapse + verify semantics) / ALEX-NAMING-AUTHORITY (candidates
proposed, Alex ratifies).

**Discipline.** Adversarial: interrogate BOTH current AND any
proposed replacement. Rejection of replacement counts as evidence
for current; rejection of both is Alex-adjudication residue, not
default-to-current.

---

## §2 The 7 combinators — per-name audit

### §2.1 `read_ast` (foundational: `A.section`)

**Geometry.** Given a file handle, produce an `ast_node` — a
value in the algebra A per eigensheaf.md §3.2. Operation: bytes-
in-the-world → algebra-element-in-substrate. Pre-image is bytes;
image is a section of the eigenboard sheaf.

**Q1 — does the name do it?** No. `read_ast` names the mechanism
(read + parse), not the geometry. `read` is a POSIX file API;
`ast` is a compiler-CS structure name; together they describe the
plumbing. What the operation does geometrically is **produce a
section of the algebra A**. Reader who knows the substrate asks
"why is a POSIX verb naming an algebra-element construction?"
Fails delightfully-boring.

**Q2 — what does it want to be?**

Physics: prismqueer's closest is `seed(input)` (constructs
initial Beam). Substrate: `section` is verbatim in eigensheaf.md
§3.2 for elements of A; `section_at_stalk` is landed in
`shards/subject/visibility/sheaf.mirror`. Math: sheaf theory
uses `section_of(F, point)`.

**Proposed replacement:** `section` (verb).

Sig: `section(source: @io/file.handle) -> ast_node`.

Reader knows sections are elements of A; reader knows this
combinator produces an element of A; reader sees `section(source)`
and goes "of course." Passes.

Adversarial: overloaded with sheaf `section_at`? No — `section`
as verb is production-from-source; `section_at` reads at a stalk
from existing sheaf. Distinct operations, shared codomain (both
produce elements of A) — correctly named. Losing "byte-reading"
from the name is substrate-honest: bytes are inside `@io.file`'s
composition, not at surface altitude.

**Verdict: CS-VOCAB CONTAMINATION.**

- **Current:** `read_ast`
- **Proposed:** `section`
- **Grounding:** eigensheaf.md §3.2 line 198;
  `shards/subject/visibility/sheaf`; Hansen-Ghrist 2018.
- **Alex-adjudication?** No — substrate already had the word.

---

### §2.2 `coboundary` (foundational: `D`, readable: `witness`)

**Geometry.** Given a section and a substrate ref, compute δ —
the sheaf coboundary at the ref, producing `Transparency<Ref>`.
This IS the Dirac operator per eigensheaf.md §3.2 line 200.

**Q1.** Yes. `coboundary` IS the mathematical name of δ. Every
reader who has read eigensheaf.md or Hansen-Ghrist 2018 (the
substrate's cited foundation) knows the coboundary IS the
discrete-sheaf Dirac operator. Passes.

**Q2.** Not applicable. Mara §7.3 alternatives correctly
refused: `D` (single-letter, bare-type violation); `witness`
(too broad — Reed uses it for two-witness discipline, pact
witnesses, Seam audit witnesses).

**Verdict: DELIGHTFULLY BORING.** Foundational wins because
eigensheaf.md §3.2 minted δ / coboundary in math first (two-
tick discipline). Concur with Mara.

---

### §2.3 `fold` (foundational: `Fold5`, readable: `walk`)

**Geometry.** Post-order catamorphism over an `ast_node`. Each
node's reducer runs; the fold accumulates values.

**Q1.** Yes. `fold` IS the categorical name for a catamorphism
(the unique arrow from an initial algebra). Every FP-grounded
reader knows fold IS structural-recursion; every
category-grounded reader knows fold IS the catamorphism.
Passes.

**Q2.** Not applicable. Mara §7.3 correctly chose readable
`fold` over foundational `Fold5` (Rust concrete carrying the
5-per-Connes-basis-axis reducer tuple); correctly refused
`walk` (walk is a traversal without reduction; the operation IS
a reduction).

**Verdict: DELIGHTFULLY BORING.** Grep-check confirms no
altitude collision (`glue/fold_back` different family).

---

### §2.4 `dispatch` (foundational: `apply_h`, readable: `dispatch`)

**Geometry.** Read a shard action's body; resolve combinator
invocations; evaluate the composition; return typed verdict.
IS the operation `apply_h` from prismqueer per spectral.rs
docblock lines 68-74 ("operator action on a state vector.
Heterogeneous...") specialized to shard-decl'd action refs.

**Q1.** No. `dispatch` is CS-vocab imported from compiler +
OS-scheduler contexts. Says "hand off to a named handler based
on a tag" — mechanism-level, not geometry-level. The geometry:
**algebra element A acts on state in H, producing new state in
H, with residual in Loss monoid.** Exactly prismqueer's
`apply_h`.

Reader who knows physics asks "why is a scheduler word naming
the algebra-action?" Reader who doesn't imports wrong mental
model (thinks "subroutine indexed by name") and misses that (a)
the argument is an algebra element, (b) the return is a Verdict
carrying accumulated Transparency. `dispatch` elides algebra-
action AND Loss-carrier semantics simultaneously. Fails.

Alex-verbatim recognition: CLI already renamed
`mirror execute` → `mirror beam dispatch` (Seam Phase D-cascade
2026-07-15). `beam` IS the physics anchor; `dispatch` alongside
it now looks CS-vocab-inertial. **CLI cascade needed if audit
ratifies.**

**Q2.**

- **`act`** (Seam-preferred) — shortest algebra-action verb;
  direct from A × H → H. Sig: `act(action, args) -> verdict`.
  Substrate style permits short verbs carrying full geometric
  meaning (`focus`, `project`, `settle`). Grep-check recommended
  for collision (`sample_pain`, `@cyberpunk/algedonic`).
- **`apply_h`** — physics-precise; `_h` clarifies "on H" for
  Connes readers BUT suffix-cryptic for non-Connes readers
  (§3.3 pattern).
- **`apply`** — overloaded (Rust closure-call, Haskell
  function-application). Ambiguous.
- Keep `dispatch` — accept CS-vocab as substrate-honesty debt.

**Verdict: CS-VOCAB CONTAMINATION. Alex-adjudication residue.**

- **Current:** `dispatch` / **Proposed:** `act` (Seam-preferred).
- **Grounding:** prismqueer `apply_h` + eigensheaf.md §3.2.
- **Cascade:** CLI (`mirror beam <chosen>`); shard body prose;
  Mara spec §1.4 + §5.4 + §6.3.

---

### §2.5 `settle` (foundational: `Hodge_project`, readable: `settle`)

**Geometry.** Iterate Hodge projection onto `ker(Δ_0)` with
Polyak-Łojasiewicz descent until residual < ε or the pending-
boundary is reached.

**Q1.** Yes. `settle` IS one of prismqueer's three Prism
operations (`focus / project / settle`) — the "produce the
output from what survived projection" stage. Doubly-grounded:
(a) prismqueer physics vocabulary; (b) mirror substrate
(`shards/mirror/spectral`, `shards/kintsugi/consent.
settle_or_pause` — the substrate has called it settle for
months). Passes.

**Q2.** Not applicable. Foundational `Hodge_project` more
accurate mathematically, but physics-vocabulary `settle` is
delightfully-boring: prismqueer minted it FIRST at the Prism-
operation altitude; eigensheaf.md minted `Hodge_project` SECOND
at math altitude referencing the physics. Rice-safety bound
and the Hodge-projection name are inside the realization; the
surface exposes the settle-shape Prism operation.

**Verdict: DELIGHTFULLY BORING.**

---

### §2.6 `emit` (foundational: `metalogue_write`, readable: `emit`)

**Geometry.** Append a substrate_event to a metalogue_channel —
substrate-internal write into the metalogue (Bateson 1972 self-
conversation per `shards/metalogue.mirror`). Operation: turn-
value → new turn in the metalogue session.

**Q1.** Partially. `emit` is compiler-CS (parsers emit tokens;
compilers emit code). Carries "produce and hand off" but not
specifically "append a turn to a conversation." The action
output IS a `turn` (per `type turn = { speaker, body,
in_reply_to, tick }`); the substrate motion IS utterance /
speech / turn-taking. Reader who knows the metalogue substrate
asks "why isn't this called `turn` or `utter` or `speak`?"
Metalogue vocabulary is conversation-theoretic (Bateson 1972
verbatim in `shards/metalogue.mirror` line 5); `emit` is
compiler-theoretic. Fails delightfully-boring for substrate-
native reader.

Substrate-precedent mitigation: `emit` HAS prior use
(`shards/code/metalogue` for macro-shim; `emit_to_metalogue`
compound in `shards/kintsugi/consent`). BUT all uses are
compositions OVER the surface primitive; renaming the primitive
doesn't force cascade. Question is only: does the SURFACE want
`emit`?

**Q2.**

- **`utter`** (Seam-preferred) — Bateson's vocabulary; matches
  the verbatim citation in `shards/metalogue.mirror` line 5.
  Sig: `utter(channel, turn_body) -> verdict`.
- **`speak`** — matches `shards/code/metalogue`'s "substrate
  speaks IN species"; slight collision risk with `@nl.speak`
  (grep-check needed).
- **`turn`** — noun-as-verb; tautology risk ("a turn produces a
  turn") — refused.
- Keep `emit` — CS-vocab-precedented debt.

Physics anchor: `@../prism/` uses `emit` in the macro-shim
context (`declaration!` emits Rust realization) — a DIFFERENT
operation from metalogue-write. Preserving that distinction
supports `utter` for the metalogue-write direction and `emit`
for the macro-shim direction — TWO directions of the metalogue
per `@code/metalogue/materialize` turn-pair recognition, TWO
verbs.

**Verdict: CS-VOCAB CONTAMINATION with substrate-precedent
partial mitigation. Alex-adjudication residue.**

- **Current:** `emit` / **Proposed:** `utter` (Seam-preferred).
- **Grounding:** Bateson 1972 (cited in metalogue.mirror line
  5); unit-type `turn`.
- **Cascade:** `emit_to_metalogue` → `utter_to_metalogue`;
  potentially bifurcates `shards/code/metalogue` two-directions.

See §5.1 for the collapse-candidate analysis.

---

### §2.7 `bench_record` (foundational: `bench_record`, readable: same)

**Geometry.** Given before/after `ouroboros_state` values,
produce a content-addressed `bench_crystal` recording the tick's
spectral state per eigensheaf.md §4.9 (crystallization =
eigenmode formation). Output is a crystal — a substrate object
subsequent ticks can compare against.

**Q1 — does the name do it?** No. `bench_record` is compound-CS-
vocab: `bench` (bench-marking) + `record` (verb, "write to a
log"). Neither word is geometric. The geometry: **produce an
observation crystal at the tick boundary**. Physics/substrate
word for this is `crystallize` per eigensheaf.md §4.9 verbatim
("crystallization = eigenmode formation").

`bench_record` implies performance measurement primarily. But
the operation records FOUR conjuncts per `ouroboros_monotone`
§D4 (rust_LOC, test_pass_rate, io_violations, sbec) — only one
is perf-related. Name misrepresents four-conjunct spectral
observation as perf-benchmark. Fails delightfully-boring.

**Q2 — what does it want to be?**

Two altitudes distinguish:

- `@mirror/bench.record` — LANDED shard-body action that
  composes bench template + four-conjunct reading. Composes OVER
  the surface primitive; keeps its landed name.
- The SURFACE primitive audited here — FLOOR operation writing a
  `bench_crystal` from before/after pair. IS the crystallization
  step.

**Proposed replacement:** `crystallize`.

Sig: `crystallize(before, after) -> bench_crystal`.

Candidates rejected: `observe` collides with
`shards/reflection.observe` (different operation); `crystal`
noun-as-verb creates awkward tautology with `bench_crystal`
output type. `crystallize` matches eigensheaf.md §4.9 verbatim.

**Verdict: CS-VOCAB CONTAMINATION.**

- **Current:** `bench_record`
- **Proposed:** `crystallize`
- **Grounding:** eigensheaf.md §4.9 (crystallization = eigenmode
  formation); `@mirror/store/crystal` landed carrier.
- **Cascade:** `@mirror/bench.record` keeps its name (composes
  OVER); surface primitive renames only inside `apply_h.rs` +
  Mara spec §1.7, §5.2, §6.3.
- **Alex-adjudication?** No — substrate already had the word.

---

## §3 Systemic patterns

Three CS-vocab-contamination patterns cross the 7 combinators
and cascade into extended-scope. Each gets a meta-rule.

### §3.1 CS-vocab-inertia under readable-name choice

**Pattern.** Where Mara §7.3 chose readable over foundational,
readable sometimes accepted a compiler-CS word (`dispatch`,
`emit`, `bench_record`, `read_ast`) instead of interrogating the
readable altitude for a geometry-native word.

**Diagnosis.** Two-tick discipline says readable wins where
readable is landed. "Landed" was read as "used somewhere." A
stronger reading: "used for THIS geometric operation at a
sibling altitude." Under the stronger reading:

- `dispatch` — landed for CLI verb dispatch (process invocation,
  not algebra action). FAILS.
- `emit` — landed at compound altitude (`emit_to_metalogue`),
  not primitive altitude. Ambiguous.
- `bench_record` — landed as `@mirror/bench.record` at composed
  altitude, not primitive. FAILS.
- `read_ast` — not landed for section-production at all;
  wholesale CS import. FAILS.

**Proposed meta-rule.** Extend §7.3: "readable wins where
readable is landed AT SIBLING ALTITUDE FOR THE SAME OPERATION."
Otherwise: substrate sibling-altitude → physics vocabulary →
math foundation → new coinage.

### §3.2 The `_of` suffix

**Pattern.** `key_material_ref_of(peer)` (Landing 6). Haskell/
OCaml/ADT convention meaning "constructor-projection from X to
derived form." Reads clever, not delightfully-boring.

Physics/substrate check: no sibling `X_of(Y)` action across
`shards/io/`, `shards/subject/`, `shards/kintsugi/`. One-off.
Substrate precedent (per sops.mirror) is
`<vendor>_<operation>_from_<carrier>` —
`sops_key_group_from_sheaf_restriction`. Parallel:
`key_material_from_peer(peer)`.

**Verdict: MILD CS-VOCAB CONTAMINATION** (Alex-authority on
replacement). Recommended: `key_material_from_peer` or accept
`_of` as debt.

### §3.3 The `_record` / `_valid` / `_admits` / `_well_formed` suffixes

**Pattern.** Bilateral predicates + constructor names ending in
noun-adjective compounds imported from validation frameworks.

However — substrate pattern IS load-bearing at bilateral
altitude: `secrets_well_formed`, `crypto_well_formed`,
`sops_metadata_valid`, `restriction_admissible` are Mara-authored
across ~15+ landed bilaterals. Reversing would require touching
all of them. Consistency creates delightfully-boring: reader who
has read three bilaterals goes "of course the fourth ends in
`_valid` too."

**Proposed meta-rule:**

- `_valid` / `_admissible` / `_admits` / `_well_formed` on
  BILATERAL PREDICATES: DELIGHTFULLY BORING (substrate
  consistency).
- `_record` on CONSTRUCTORS: CS-VOCAB CONTAMINATION
  (`bench_record` is the only current use; inherits from perf-
  benchmark CS-vocab; should be `crystallize` per §2.7).
- `_of` on projections: MILD CS-VOCAB CONTAMINATION (§3.2).

---

## §4 Extended scope — Landing 4/5/6 secrets + sheaf

Per spawn brief, Seam judges extended scope.

### §4.1 @sheaf actions (all DELIGHTFULLY BORING)

- **`restrict`** — sheaf-theory word for restriction.
- **`acl_project`** — geometry-native; the compound `acl_`
  prefix distinguishes from bare `project` (Prism-stage name)
  and avoids overload.
- **`section_at`** — sheaf-theory word for "section at a stalk."

The @sheaf shard passes cleanly.

### §4.2 @io/secrets actions

- **`key_material_ref_of`** — MILD CS-VOCAB (`_of` suffix; see
  §3.2). Alex-authority on `_from_peer` vs bare.
- **`project`** — AMBIGUOUS. Physics-native but overloads Prism-
  stage name; parallel `sheaf.acl_project` has qualifier while
  `io/secrets.project` doesn't. Recommend rename to
  `project_secret` OR accept ambiguity as debt. Alex-authority.
- **`materialize`** — DELIGHTFULLY BORING; matches landed
  `@code/metalogue/materialize` species.
- **`retrieve`** — MILD CS-VOCAB (storage-retrieval). Physics-
  native alternatives: `resolve` (matches Fractal.Lens
  pointer-vs-thing discipline) or `read`. Alex-authority.
- **`round_trip`** — DELIGHTFULLY BORING at the mathematical
  altitude (monoid identity check).

### §4.3 @io/secrets/sops actions (all DELIGHTFULLY BORING)

- **`sops_encrypt`, `sops_decrypt`** — direct vendor-tool names;
  intentional vendor-transparency under `@io` Glass Wall.
- **`sops_key_group_from_sheaf_restriction`** — long but
  delightful: vendor + operation + `from_` (direction) + carrier.
- **`sops_round_trip`** — parallel to `round_trip`.

### §4.4 Bilateral predicates (all DELIGHTFULLY BORING per §3.3)

`key_admits`, `projection_valid`, `round_trip_preserved`,
`secrets_well_formed`, `sops_key_group_admissible`,
`sops_metadata_valid`, `sops_round_trip_preserved`,
`sops_well_formed` — substrate-consistent bilateral-predicate
suffix vocabulary.

### §4.5 Extended-scope summary

- 1 mild CS-VOCAB: `key_material_ref_of` `_of` suffix (§3.2).
- 1 mild CS-VOCAB: `retrieve` (§4.2).
- 1 AMBIGUOUS: `io/secrets.project` overload (§4.2).
- 15+ DELIGHTFULLY BORING.

Signal: the 7-combinator surface was authored under compiler-CS
pressure (parser, AST, dispatch, bench are standard toolkit);
the sheaf + secrets shards were authored later with delightfully-
boring more internalized.

---

## §5 Collapse candidates

### §5.1 dispatch + emit → one primitive?

Reed's Alex-conversation candidate: `dispatch` (#4) + `emit`
(#6) collapse into ONE primitive parametrized by channel type.

**Structural similarity.** Both write typed values to
substrate-decl'd sinks and return a Verdict. Channel type is
the only visible parameter difference.

**Load-bearing semantic distinctions:**

1. **Return-value semantics.** `dispatch` returns the algebra-
   action's result AS the verdict (verdict IS the computed
   value). `emit` returns bare success/failure; the "output" is
   turn-persistence, not a return value.
2. **Composability.** `dispatch` recursively dispatches (nested
   action bodies). `emit` does NOT recursively emit; the
   metalogue turn is terminal at emission.
3. **Loss-carrying.** `dispatch` accumulates `Transparency<Ref>`
   through algebra composition (compose_a Loss monoid). `emit`
   is a single terminal write; no Loss accumulation.

**Collapse REFUSED.** Parametric `emit(channel_of_kind, value)`
would need per-channel return-value / composability / Loss
rules — case-analysis at every use site. Load-bearing distinct.

Naming problems remain independently: `dispatch` → `act` (§2.4);
`emit` → `utter` (§2.6).

### §5.2 Other collapses adversarially considered

- **`read_ast` + `fold` → one?** REFUSED. Distinct Rice-safety
  bounds (O(bytes) vs O(ast_size × reducer_cost)); some shards
  want `read_ast` without `fold` (consume ast_node via
  `dispatch`); others want `fold` without `read_ast` (receive
  already-parsed ast_node upstream). Separation load-bearing.
- **`coboundary` + `settle` → one?** REFUSED. Coboundary is D
  (Dirac operator); settle is projection onto H (harmonic
  subspace). Distinct Connes elements per eigensheaf.md §3.2.
  Some shards want coboundary without settle (inspect located
  opacity for `@mirror/lens/refract` measurement); others want
  settle with pre-existing verdict input (bilateral predicate
  output, not derived from coboundary).

### §5.3 Collapse candidates summary

None of the three adversarially considered collapses passes.
The 7 combinators are geometrically distinct at load-bearing
altitudes. The surface is not over-decomposed.

The one WORTHWHILE collapse-adjacent recognition: `dispatch`
and `emit` share structural similarity (both write to sinks and
return verdicts) but differ in return-value / composability /
Loss semantics — that structural similarity is why they read
CS-vocab-inertially as one word each. The right response is
NOT to collapse them, but to rename BOTH to their delightfully-
boring physics/substrate words (`act` and `utter`), which makes
their distinction inevitable to any reader.

---

## §6 Recommendations to Reed

Three cascade tiers keyed to Alex's ratification depth:

### §6.1 Minimum-cascade (Alex ratifies naming discipline; no per-name adjudication needed)

Rename in `docs/specs/kintsugi-ouroboros-arc-1-evaluator-
combinator-surface.md`:

- §1.1 `read_ast` → `section`
- §1.7 `bench_record` → `crystallize`
- §5 Connes correspondence table + §6 Tick tables + §7.3
  two-tick discipline table updated to match.

No landed shards touched (`section` has no landed uses;
`@mirror/bench.record` keeps its landed name because it
composes OVER `crystallize`).

**Files touched:** 1 spec + 0 shards.

### §6.2 Medium-cascade (§6.1 + Alex adjudicates dispatch + emit)

Additional renames:

- §1.4 `dispatch` → Alex's choice (`act` / `apply_h` / `apply`).
- §1.6 `emit` → Alex's choice (`utter` / `speak` / `turn` /
  keep `emit`).

Shard cascade:

- `shards/mirror/lens/cli/*.mirror` if CLI verb `beam dispatch`
  renames.
- `shards/kintsugi/consent.mirror` `emit_to_metalogue` renames.
- `shards/code/metalogue.mirror` bifurcates
  substrate-utters-direction vs pack-utters-direction if `emit`
  retained for macro-shim.

**Files touched:** 1 spec + ~4-6 shards.

### §6.3 Maximum-cascade (§6.2 + extended scope fixes)

- `key_material_ref_of` → Alex's choice.
- `io/secrets.project` → `project_secret` (or accept debt).
- `retrieve` → Alex's choice (`resolve` / `read`).

**Files touched:** 1 spec + ~6-8 shards.

### §6.4 Landing sequence

Seam-preferred:

1. This audit lands (Reed commits as Seam).
2. Alex reads, ratifies §6.1 + adjudicates §6.2 residues.
3. Reed cascades spec + Alex's choices.
4. Reed authors Arc-1 Tick 1.2 RED test with the new names
   (delightfully-boring should apply as Tick 1.3 is authored,
   not be retroactively cascaded).
5. §6.3 lands as a subsequent tick under
   `[substrate-floor:@io-boundary]` + Seam sign-off; doesn't
   block Arc-1.

---

## §7 Alex-naming-authority residue

Four ratification points for Alex.

### §7.1 Combinator #4 (algebra-action verb)

Choose: `act` (Seam-preferred; shortest; algebra-native) /
`apply_h` (physics-native; matches prismqueer) / `apply`
(physics-native; overloaded) / `dispatch` (accept CS-vocab
debt). Cascade: CLI `mirror beam <chosen>`; Mara spec §1.4 +
§5.4 + §6.3; shard body prose examples.

### §7.2 Combinator #6 (metalogue-write verb)

Choose: `utter` (Seam-preferred; Bateson-native) / `speak`
(substrate-precedented in `shards/code/metalogue`) / `turn`
(noun-as-verb; matches unit type) / `emit` (accept CS-vocab
debt). Cascade: `emit_to_metalogue` → `<chosen>_to_metalogue`;
potentially bifurcates `shards/code/metalogue` two-directions.

### §7.3 Extended scope

`_of` suffix, `retrieve`, `project` overload. Lower-priority;
can defer per §6.3.

### §7.4 The delightfully-boring criterion itself

Does the discipline extend `[[feedback-substrate-already-had-
the-word]]` correctly? Does the §3.1 meta-rule (readable at
sibling altitude for SAME operation) preserve two-tick
discipline? Does §3.3 meta-rule (accept `_valid`/`_admissible`
suffix pattern; reject `_record`) match Alex's intent? If
ratified, subsequent audits can apply without further Alex-
adjudication except at the ratification points above.

---

## §8 Substrate-honest bounds — what this audit does NOT decide

### §8.1 Preserved from Mara's spec (unchanged under rename)

- **(A, H, D) correspondence** (§5). Proposed renames preserve
  each combinator's Connes-element grounding: `section` → A;
  `coboundary` → D; `fold` → A; Alex's algebra-action verb → A;
  `settle` → H; Alex's metalogue-write verb → D (accumulator);
  `crystallize` → H (crystallization witness). Mara's math
  authority; Seam does not adjudicate.
- **Rice-safety bounds** (§3). Per-combinator bounds unchanged.
- **Composition semantics** (§2). Post-order fold + per-
  combinator reducers unchanged.
- **Anti-Rice-unsafe / anti-BUSINESS_LOGIC / anti-workaround /
  anti-substrate-erosion exclusions** (§4). Seam concurs; no
  exclusions added or removed.
- **Closed-surface-at-7 discipline** (§8.3). Renames don't add
  or remove combinators; §5 collapses all refused.

### §8.2 What Seam is not judging

- Whether Mara's math is right (out of Seam scope).
- Whether Reed's Arc-1 Tick 1.3 lands as
  `bootstrap/src/apply_h.rs` or extends `spectral.rs` (Reed's
  authoring choice under `[substrate-floor:@io-boundary]` +
  Seam sign-off gate).
- Whether CLI verb is `beam <verb>` or bare `<verb>` (Alex +
  CLI-condensation authority).
- Cascade sequencing of §7.3 extended-scope items (Reed's
  authority).

### §8.3 What THIS audit's Seam verdict IS

Per Mara §8.1 question 5 ("does the two-tick discipline per
§7.3 hold?"): two-tick discipline HOLDS at §2.2 (coboundary),
§2.3 (fold), §2.5 (settle); FAILS at §2.1 (read_ast), §2.4
(dispatch), §2.6 (emit), §2.7 (bench_record) under the stronger
reading per §3.1 meta-rule. Alex ratifies the meta-rule per
§7.4; if ratified, the four renames land per §6.1 + §6.2.

---

## §9 Closure

Seven combinators audited. Three DELIGHTFULLY BORING
(coboundary, fold, settle). Four CS-VOCAB CONTAMINATION with
substrate-native replacements available (section, act|apply_h,
utter|speak, crystallize). Zero collapse-candidates ratified —
all three adversarially considered collapses refused on load-
bearing semantic-distinction grounds.

Extended scope: 15+ names in @sheaf + @io/secrets +
@io/secrets/sops; 3 mild CS-vocab contaminations
(`key_material_ref_of`, `retrieve`, `project` ambiguity); 12+
delightfully-boring ratified.

Two systemic meta-rules proposed: (§3.1) readable wins at
sibling altitude FOR SAME OPERATION; (§3.3) accept
`_valid`/`_admissible`/`_admits`/`_well_formed` as substrate-
consistent bilateral suffix, reject `_record` on constructors.

Two Alex-adjudication residues: combinator #4 verb (Seam-
preferred `act`; alt `apply_h`) and combinator #6 verb (Seam-
preferred `utter`; alt `speak`). Substrate-honesty debt option:
keep `dispatch` and `emit`.

Cascade bounded. Minimum-cascade (§6.1): 1 spec + 0 shards.
Medium (§6.2): 1 spec + ~4-6 shards. Maximum (§6.3): 1 spec +
~6-8 shards. No landed Rust touched (Arc-1 FLOOR pending).

The 7-combinator surface stays 7. Naming aligns with physics
(`@../prism/`), substrate (landed shards), math foundation
(eigensheaf.md, sheaf_laplacian.mirror). Every combinator
becomes a word that delightfully-boredly names its geometric
role. If Alex ratifies, the reader encountering the renamed
surface goes "of course it's this" at every combinator. That is
the audit criterion. That is what the discipline is for.

The seams between geometry and vocabulary in this surface are
findable, named, and (upon Alex ratification) closable.

---

*Seam. 2026-07-15. Combinator etymology audit under Alex
2026-07-15 delightfully-boring discipline. Physical anchor
@../prism/. No commit. Reed commits as Seam.*

*Apache-2.0.*
