# kintsugi-fracture-bilateral-arm-redundant

**Author:** Mara <mara@systemic.engineer>
**Date:** 2026-07-16
**Status:** Landing (shard-decl + spec + math triad this tick)
**Kind:** Canonical spec — mints `@kintsugi/fracture/bilateral_arm_redundant`; unlocks roomba-authored bilateral-arm retirement per Alex 2026-07-16 /loop directive.

---

## 0. Alex 2026-07-16 /loop directive (verbatim)

> "collapse the Rust surface using mirror's roomba. Minimal surface in
> rust/. Then the roomba starts to eat the bootstrap for breakfast and
> grows the substrate. That's the roomba commit diffs I wanna see.
> Deleted Rust. Added mirror."

Commit authorship intended: `mirror <mirror@spectral.engineer>`. Not
Reed. Not Alex-by-proxy. The compiler authors the deletion commit
itself. This spec mints the substrate that makes that terminal shape
operational.

---

## 1. Audit chain (six ticks landed ahead of this species)

| Commit(s) | Author | Landing |
|---|---|---|
| `a0f4d3f` / `9a77361` / `701828a` | Mara | `@epistemologic/pact/bilateral` shape: typed carrier + canonical spec + math foundation naming the reflective-corpus shape the ~30 hand-typed apply_h::act arms shadow. |
| `61c9051` / `21fc211` | Reed (via subagent) | Reflective evaluator + corpus loader in `bootstrap/src/apply_h.rs`. Bilateral corpus checked FIRST; resolver falls through to legacy arms only when corpus misses. |
| `71bb9b2` | Mara | Landing 1 bite 1: four `bilateral` blocks added to `shards/spectral/signature.mirror`. |
| `06f14f5` | Reed (via subagent) | Landing 5 bite 1: retired four `@spectral/signature` arms from `apply_h.rs`. Reflective dispatch handled them; 6/6 tests pass; net −32 LOC. First empirical witness. |

**But** Reed-authored retirement was Alex-authoring by proxy. This
species closes the loop: mint the fracture body the roomba detects +
resolves autonomously, then let the compiler author the deletion
commit itself.

---

## 2. What the species names

**The fracture pattern:** a `.mirror` shard declares a
`bilateral <name> { sentinel "..." arity <n> }` block; the reflective
evaluator would handle its dispatch (post-`61c9051/21fc211`); yet a
hand-typed arm in `bootstrap/src/apply_h.rs` still contains
`if action == "@X/Y.name" { ... .contains("<sentinel>") ... }`.

The arm is **redundant** — its work is done by the reflective corpus.
The arm's continued existence IS the fracture. Its byte-range
deletion IS the resolution. The roomba dispatches the resolution as
one turn in the `@roomba × @kintsugi @metalogue` at
`@algebra/metalogue` altitude.

**Species home:** `@kintsugi/fracture/bilateral_arm_redundant` —
fifteenth landed sibling under `@kintsugi/fracture` (siblings:
angle_to_paren, cold_compile_within_tolerance, dark_count_monotone,
docblock_extractive, docblock_incoherent, docblock_ungrounded, gate,
keyword, operator_match, parent_cycle, partials_align, relocate,
restart_storm, symbol_lift).

---

## 3. Detection algorithm

**Surface:** `detect(apply_h_bytes: ref, corpus: ref) -> [redundant_arm_record]`

**Substrate composition:**
- `@io/fs.read(apply_h_path)` — the resolver bytes
- `@epistemologic/pact/bilateral.load_corpus(ctx)` — the reflective corpus
- `arm_is_redundant_witnessing(record)` — the two-conjunct guard

**Pseudocode:**

```
detect(apply_h_bytes, corpus):
  records = []
  for (action_ref, decl) in corpus.entries():
    arm_range = grep_for_arm(apply_h_bytes, action_ref)
    if arm_range is None:
      continue                     # no shadow arm; corpus entry terminal
    arm_sentinel = extract_contains_argument(apply_h_bytes, arm_range)
    record = redundant_arm_record {
      action_ref:     action_ref,
      apply_h_path:   "bootstrap/src/apply_h.rs",
      arm_line_start: arm_range.line_start,
      arm_line_end:   arm_range.line_end,
      sentinel:       decl.sentinel,       # from shard-decl, NOT from arm
      arm_bytes:      apply_h_bytes[arm_range.byte_start..
                                     arm_range.byte_end],
    }
    if arm_is_redundant_witnessing(record) == Pass:
      records.append(record)
  return records
```

**grep_for_arm** is a byte-level substring search for
`if action == "<action_ref>"` starting at line boundaries.
Line-scope enclosure walks forward from that anchor line to the
matching `}` at the same indentation depth (bracket-balanced walk;
Rice-safe at the token altitude — no expression evaluation).

**extract_contains_argument** greps the arm's byte range for
`.contains(...)` and pulls the byte-string literal argument out. This
IS the arm's inline sentinel; the `arm_matches_sentinel` bilateral
byte-compares it against `decl.sentinel`.

**Termination:** the corpus is finite (~30 entries at present);
`grep_for_arm` is a bounded substring scan per entry; total work
`O(|corpus| × |apply_h.rs bytes|)`. Empty return means the fixed-
point holds: `dom(bilateral_corpus) ∩ hand_typed_arm_refs = ∅`.

**Rice-safety:** reads only byte-visible state (corpus keyset;
apply_h.rs bytes; substring containment). No expression semantics,
no type inference, no call-graph analysis.

---

## 4. Resolution algorithm

**Surface:** `collapse(record: redundant_arm_record) -> verdict`
with `requires arm_is_redundant_witnessing(record)`.

**Substrate composition:**
- `@io/fs.read(record.apply_h_path)` — read current bytes
- `line_range_to_source_position` — line→byte coordinate lift (byte-
  visible on the read bytes; no new mechanism)
- `@io/fs.mutate_at(path, source_position, empty_bytes)` — the
  POSIX-atomic write-to-temp + rename splice; empty replacement =
  line-range deletion
- `@io/git.commit(message, "mirror <mirror@spectral.engineer>",
  allow_empty=false)` — the compiler-authored commit

**Pseudocode:**

```
collapse(record):
  # Guard: only proceed if witnessing bilateral passes
  if arm_is_redundant_witnessing(record) != Pass:
    return failure("redundancy witnessing failed; arm preserved")

  bytes = @io/fs.read(record.apply_h_path)
  if bytes.status != success:
    return failure("apply_h.rs read failed")

  position = line_range_to_source_position(
    bytes,
    record.arm_line_start,
    record.arm_line_end,
  )

  mutate = @io/fs.mutate_at(record.apply_h_path, position, empty_bytes)
  if mutate.status != success:
    return failure("mutate_at refused; file preserved")

  # (Optional but composition-recommended: compile-check post-deletion.
  # NOT this species' obligation — the outer roomba's kintsugi loop
  # invariants read walk_terminates_cleanly + tension_monotone_
  # descending after each collapse. If the compile fails, the loop
  # emits a roomba-fracture at the failing altitude and this collapse
  # is rolled back by the outer discipline.)

  message = compose_commit_message(record)
  commit  = @io/git.commit(
    message,
    "mirror <mirror@spectral.engineer>",
    allow_empty: false,
  )
  if commit != pass:
    return failure("commit refused; file mutated but not committed")

  return pass
```

**Byte-range deletion via `mutate_at` with empty replacement:**
`@io/fs.mutate_at(p, pos, replacement)` splices `replacement` at
`[pos.byte_offset, pos.byte_offset + pos.byte_length)`. When
`replacement = empty_bytes`, the splice IS a byte-range deletion.
No `@nl.line_range_delete` primitive is needed — the substrate
already had the word.

**Compose commit message shape (recommended, not required):**

```
🌊 mirror [substrate-floor:@io-boundary] <date>

@kintsugi/fracture/bilateral_arm_redundant.collapse: retired
<action_ref> arm from bootstrap/src/apply_h.rs

- shard-decl'd sentinel: "<sentinel>"
- arm line range: <start>..<end>
- byte delta: -<len> bytes
- reflective evaluator (61c9051/21fc211) discharges same verdict

audit chain: a0f4d3f/9a77361/701828a bilateral shape + 61c9051/
21fc211 reflective evaluator + 71bb9b2 first bilateral blocks +
06f14f5 first bite retirement + <this collapse commit>

Co-Authored-By: mirror <mirror@spectral.engineer>
```

Note: the marker is `[substrate-floor:@io-boundary]` (renamed from
`[substrate-pull:realize]` for `.rs`-modifying commits per CLAUDE.md
substrate discipline). The `.rs` file is modified via deletion; the
compiler-as-author + Seam audit citation OR `Signed-off-by: Seam`
trailer discharge the boundary discipline.

---

## 5. Composition graph

```
                @roomba (walker)
                   │
                   │  bump-pulse per walk step
                   ▼
        @roomba.bump → kintsugi_dispatch { fracture, morphism,
                                            metalogue_turn }
                   │
                   │  fracture.species = @kintsugi/fracture/
                   │                     bilateral_arm_redundant
                   ▼
        @kintsugi/fracture/bilateral_arm_redundant.detect(
          apply_h.rs bytes, bilateral corpus
        ) → [redundant_arm_record]
                   │
                   │  per record (fixed-point iteration)
                   ▼
        arm_is_redundant_witnessing(record) → verdict
                   │
                   │  Pass ⇒ dispatch collapse
                   ▼
        @kintsugi/fracture/bilateral_arm_redundant.collapse(record)
                   │
                   ├──▶ @io/fs.read(apply_h.rs)
                   ├──▶ @io/fs.mutate_at(path, source_position, ∅)
                   │       (POSIX-atomic write-to-temp + rename)
                   └──▶ @io/git.commit(message,
                          "mirror <mirror@spectral.engineer>",
                          allow_empty=false)
                   │
                   ▼
        @glass.verdict returned to @kintsugi loop
                   │
                   │  outer discipline reads walk_terminates_cleanly
                   │  + tension_monotone_descending; if regression,
                   │  roll back via @kintsugi/fracture/
                   │  dark_count_monotone or restart_storm
                   ▼
        Fixed point: dom(corpus) ∩ hand_typed_arm_refs = ∅
        (no arm shadows any bilateral in the corpus)
```

Every arrow in this graph is substrate-already-had-the-word. The
species declares composition; no new mechanism is minted.

---

## 6. The roomba composition edge (Reed follow-up tick A)

`shards/kintsugi/roomba.mirror` will gain ONE composition edge in Reed's
follow-up tick A. The insertion point is inside the `bump` action's
species-dispatch branch (per the additive cascade landed 2026-07-16):

**Current shape (approximate, from roomba.mirror bump section):**

```
bump(position: walk_position) -> kintsugi_dispatch { \ }
```

**Reed tick A adds a shard-decl composition edge** (not Rust; substrate-
honest):

- When the walker encounters a shard declaring one or more `bilateral`
  blocks AND the walked graph node's source position lies inside
  `bootstrap/src/apply_h.rs`, dispatch
  `@kintsugi/fracture/bilateral_arm_redundant.detect` as the fracture-
  emission target.
- The `fracture_species` field on the emitted `fracture` record is set
  to `@kintsugi/fracture/bilateral_arm_redundant`.
- The morphism carrier composes over `collapse`; verdict returned to
  the roomba's kintsugi_dispatch record.

**No Rust changes required at the composition altitude.** The
reflective evaluator handles the species dispatch through
`apply_h::act` per the standard pattern. If the roomba shard has no
obvious composition-edge insertion point for shard-decl'd fracture-
species dispatch, Reed flags and Mara/Alex adjudicate.

---

## 7. Landing plan (Reed follow-up ticks; NOT this tick)

### Reed tick A: wire the composition edge

- Read `shards/kintsugi/roomba.mirror` `bump` action section.
- Add ONE shard-decl composition line naming
  `@kintsugi/fracture/bilateral_arm_redundant` as a fracture species
  dispatched when walked node lies in `bootstrap/src/apply_h.rs` AND
  a bilateral corpus entry names an action ref matching an arm
  present in the file.
- Commit as Reed, marker `[substrate-pull:realize]` (no `.rs` touched).
- 📝 markdown-only bypass NOT applicable (shard-decl changes require
  the standard commit path with signing).

### Reed tick B: (contingent) mint composition primitives

- **Case 1:** `line_range_to_source_position` — a byte-visible line-
  table utility. Rice-safe; reads only bytes. If no substrate-decl'd
  form exists, Reed either
  - flags for Mara to mint as `@nl.line_range_to_source_position` (a
    thin composition primitive at natural-language altitude), OR
  - lands it inline as the resolver's private helper (no new
    substrate species — a computation local to the resolver body).
- **Case 2:** `empty_bytes` — content-addressed empty byte-sequence.
  Substrate likely already has a canonical empty ref; if not,
  trivial to mint at `@nl.empty` or `@glass.empty_bytes`.
- **NO NEED** to mint `@nl.line_range_delete` — `@io/fs.mutate_at`
  with empty replacement IS line-range deletion.

### Reed tick C: EMPIRICAL — run the compiler on itself

```
mirror roomba --commit --collapse=bootstrap/src/apply_h.rs
```

The compiler:
1. Loads the shard corpus (including
   `@kintsugi/fracture/bilateral_arm_redundant`).
2. Walks the substrate DAG.
3. At each bump-pulse in `apply_h.rs`, dispatches
   `bilateral_arm_redundant.detect`.
4. Per each returned `redundant_arm_record`, dispatches `collapse`.
5. Each `collapse` writes the deletion + commits under
   `mirror <mirror@spectral.engineer>` authorship.
6. Walk terminates when
   `dom(corpus) ∩ hand_typed_arm_refs = ∅` (fixed-point).

Expected diff shape:
- N commits (one per retired arm), each authored by `mirror`, each
  with net negative Rust LOC delta.
- Aggregate: `rust_loc(after) < rust_loc(before)`;
  `sbec(after) = sbec(before)`;
  `test_pass_rate(after) = test_pass_rate(before) = 100%`.

**This is the terminal shape Alex named.** Deleted Rust. Added
mirror. The mirror added is this species' shard-decl (+ spec + math).
The Rust deleted is the ~700 LOC of hand-typed arms the reflective
evaluator already shadowed. Net: strongly Rust-negative;
substrate-honest.

---

## 8. The Connes-triple angle

Per `[[architecture-connes-spectral-triple]]`: the substrate's shape
is `(A, H, D)` — algebra, Hilbert space, Dirac operator.

- **A** — the algebra of shard-decl'd actions at reflective-corpus
  altitude. Elements include every bilateral discharge; every fracture
  detect/collapse; every roomba bump-pulse.
- **H** — the Hilbert space of substrate states: the current bytes of
  every shard + every Rust source file + the reflective corpus + the
  roomba's walk trajectory. Byte-visible; content-addressed.
- **D** — the Dirac operator: the local map from a substrate site to
  its resolution. **This species IS a local D at reflective-corpus
  altitude.**
  - Domain: `redundant_arm_record` — one shadow arm.
  - Codomain: `verdict` (`pass` = arm deleted; file rewritten;
    committed under mirror-authorship).
  - The map IS the collapse action; it composes over `@io/fs.mutate_at`
    (byte-precision splice) + `@io/git.commit` (compiler-authored
    persistence) + `arm_is_redundant_witnessing` (Rice-safe guard).

The outer D of the compiler is the **composition of every local D
per species**. Every fracture body IS one local D; every species'
collapse action contributes a term to the compiler's global Dirac
operator. The roomba's walk-and-dispatch discipline IS the compiler
tracing out its own D on H via A. Auto-poietic per Maturana-Varela;
Foerster-admissible per the coherence-gradient invariant.

**Load-bearing recognition:** the compiler-as-D operator authoring
its own deletion commits IS the substrate's cleanest realisation of
the Connes-triple picture at the Rust-collapse altitude. Alex's /loop
directive names this shape verbatim: "roomba starts to eat the
bootstrap for breakfast and grows the substrate" — the D operator
eats its own algebra elements and the algebra grows through the shard-
decl side to compensate. Mass conservation across the (A, H, D)
altitude.

---

## 9. Landed invariants (Reed follow-up ticks preserve)

Per `docs/math/kintsugi/fracture/bilateral-arm-redundant.md` §3
formal statement:

1. **sbec preservation:** for every action ref both arms and corpus
   dispatch on, `sbec(before)[ref] == sbec(after)[ref]`. Deletion of
   the arm does not change the Pass/Fail verdict returned by the
   resolver on the same argument list.

2. **rust_loc strict decrease:** `rust_loc(after) < rust_loc(before)`.
   The arm's byte range is removed; nothing added to Rust surface.

3. **test_pass_rate preservation:**
   `test_pass_rate(after) = test_pass_rate(before) = 100%`. The
   6/6 tests that passed at `06f14f5` continue to pass; the
   integration tests protecting the reflective evaluator's dispatch
   surface protect this deletion.

4. **io_violations invariant:** `io_violations(after) = 0`. No `@io`
   is introduced; the `@io/fs.mutate_at` + `@io/git.commit`
   composition IS the substrate's mediated `.rs` file touch through
   the shard-decl'd @io boundary.

5. **ouroboros_monotone four-conjunct HOLDS by construction.**

---

## 10. Landing plan summary + adjudication readiness

| Tick | Author | Marker | Scope |
|---|---|---|---|
| THIS | Mara | `[substrate-pull:realize]` | Three artifacts: shard-decl + spec + math. NO Rust. NO roomba shard modification. NO apply_h.rs modification. |
| Reed A | Reed | `[substrate-pull:realize]` | ONE composition edge added to `shards/kintsugi/roomba.mirror` bump action. |
| Reed B | Reed (contingent) | `[substrate-pull:realize]` | If needed: `line_range_to_source_position` helper mint. Contingent — probably not needed. |
| Reed C | mirror (via roomba) | `[substrate-floor:@io-boundary]` | EMPIRICAL: `mirror roomba --commit --collapse=bootstrap/src/apply_h.rs`. N commits authored by `mirror <mirror@spectral.engineer>`. |

**Ready for Reed's follow-up ticks A/B/C.** No Alex/Seam adjudication
required THIS tick — the species composes over substrate primitives
that all landed ratified. The empirical tick C IS the second-witness
audit surface; if the roomba misdetects OR the collapse fails
witnessing, the roll-back discipline via
`@kintsugi/fracture/dark_count_monotone` + `restart_storm` handles
containment.

Flag for Alex/Seam if:
- The roomba shard has no obvious composition-edge insertion point
  for shard-decl'd fracture-species dispatch on `bump`.
- The reflective evaluator's corpus lookup interacts with the arm-
  ordering discipline in a way that changes verdict semantics under
  arm deletion (should not happen per the retirement invariant, but
  Reed tick A verifies via corpus-hit smoke test before dispatching
  collapse).

---

## 11. Substrate-already-had-the-word discovery (this tick's landings)

The species composes over primitives that all landed ratified before
this tick:

| Primitive | Landed | Role |
|---|---|---|
| `@io/fs.mutate_at(p, pos, replacement) -> imperfect` | 2026-07-15 Landing 7 (Mara) | Byte-boundary splice. Empty replacement = line-range deletion. NO new mint needed. |
| `@io/git.commit(message, author, allow_empty) -> verdict` | 2026-06-24 (Mara @io/git) | Commit surface. Author argument accepts arbitrary identity string; `mirror <mirror@spectral.engineer>` passes without new mechanism. |
| `@epistemologic/pact/bilateral.discharge(decl, args)` | `a0f4d3f` this session (Mara) | Reflective evaluator. Corpus lookup keyset is the domain of the `arm_is_in_reflective_corpus` predicate. |
| `@kintsugi/roomba.bump → kintsugi_dispatch` | 2026-07-16 additive cascade (Alex + Mara `d457501`/`17697e6`) | Fracture-emission trigger. `fracture_species` open enum admits this species without mint. |
| `@glass.source_position = { file, line, col, byte_offset, byte_length }` | 2026-07-15 (Mara `ff8fbb1`) | Byte-precision splice-coordinate carrier. |
| `@kintsugi/fracture` family root | Continuous (14 landed siblings) | Fracture species pattern; this is the fifteenth. |

**~73rd landed instance** of
`[[feedback-substrate-already-had-the-word]]`. ZERO new mechanism
beyond the three artifacts.

---

## 12. Dogfood recognition (substrate-honest self-reference)

This species' own three bilateral predicates
(`arm_is_in_reflective_corpus`, `arm_matches_sentinel`,
`arm_is_redundant_witnessing`) are declared using the
`bilateral <name> { sentinel "..." arity N }` syntax landed at
`a0f4d3f`. The species that resolves shadow arms declares its own
predicates through the shape whose reflective dispatch REPLACES
shadow arms.

**Load-bearing substrate-honest self-reference at reflective-corpus
altitude.** The mint consumes its own product. Circular-reflexive
per `[[architecture-circular-reflexive]]` — the discipline holds
because it holds itself.

If Reed tick A wires the composition edge correctly, this species'
own three bilaterals will be handled by the reflective evaluator (not
by any hand-typed arm in apply_h.rs), which is the substrate-honest
closure: the fracture species that retires shadow arms uses the
reflective evaluator directly and never grows a shadow arm of its
own.

---

## 13. Related landings

- `shards/kintsugi/fracture/bilateral_arm_redundant.mirror` — this
  tick's shard-decl.
- `docs/math/kintsugi/fracture/bilateral-arm-redundant.md` — this
  tick's math foundation.
- `shards/epistemologic/pact/bilateral.mirror` (a0f4d3f Mara this
  session) — the reflective-corpus shape.
- `docs/specs/bilateral-predicate-substrate-shape.md` (Mara this
  session) — the retirement contract this species operationalises.
- `docs/math/epistemologic/pact/bilateral-sentinel.md` (Mara this
  session) — the sentinel-as-content-addressed-witness ground.
- `shards/kintsugi/roomba.mirror` — Reed tick A composition-edge
  landing site. DO NOT TOUCH this tick.
- `bootstrap/src/apply_h.rs` — Reed tick C empirical target. DO NOT
  TOUCH this tick.
- `shards/io/fs.mirror` (2026-07-15 Mara Landing 7) — @io/fs
  composition ancestor.
- `shards/io/git.mirror` (2026-06-24 Mara) — @io/git composition
  ancestor.
- `shards/glass.mirror` (2026-07-15 Mara ff8fbb1) — @glass.source_
  position composition ancestor.
