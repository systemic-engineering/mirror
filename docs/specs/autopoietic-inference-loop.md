---
author: Mara
scope: Canonical spec for the mirror compiler's autopoietic inference loop — the six-step composition (walk → build_hole_record → roll → translate → crystallize → project) that resolves `\` fractures via @fate tournament over substrate geometry, crystallizes each resolution as a content-addressed fragment, and projects the resolution back into the source file. Grounds the three bridges (α: position-aware mutation, β: AST-context hole builder + resolve_hole, γ: crystallization persistence) that close the autopoietic loop end-to-end for one fracture on one round-trip. Composes: eigensheaf Hodge projection at inference altitude; Rayleigh descent per @fate/tournament ranking; @bauchladen tray as autopoietic memory; @kintsugi.ouroboros.collapse inferring its own body from surrounding geometry.
status: canonical
companion:
  - docs/math/autopoiesis/README.md
  - docs/scouts/2026-07-15-taut-autopoietic-composition-surface.md
  - shards/kintsugi/ouroboros.mirror
  - shards/fate.mirror
  - shards/glue.mirror
  - shards/bauchladen.mirror
  - shards/autopoietic.mirror
  - shards/mirror/store.mirror
  - shards/glass.mirror
  - shards/io/fs.mirror
---

# Autopoietic Inference Loop — the mirror compiler as operationally-closed self-producing system

*2026-07-15. Mara. Canonical spec authored after Reed's substrate-lipstick
correction and Taut's ten-dimensional composition-surface scout.*

---

## §0 Prelude — the load-bearing frame

### §0.1 Alex Wolf 2026-07-15 in-transcript verbatim (three messages)

Prior context: Reed had been building Rust sentinel-matching wrapped in
substrate ceremony — `bootstrap/src/apply_h.rs` byte-matching substrate-
decl'd sentinels (`chain=merkle-linked`, `axis=splinter-ward`, etc.),
lifting sbec 0 → 15+ across Arc-2 Ticks 2.1–2.4 without shard-body
dispatch. Alex named the correction across three messages:

> "This is what @kintsugi is supposed to be doing. INFER the
> implementation of the { \\ } because the geometry surrounding it
> tells it which shapes it wants to have. Each kintsugi pass then
> reduces ambiguity, which is what `\\` is, a fracture, through a
> @fate tournament into possible candidates. The @roomba bumps into
> `\\` cracks. The liquid types and the mycelial math fill it with
> gold."

> "This is also where the `\\` and `|\\>` etc operators come in. A
> `\\` can be resolved to a partial composition which is still
> partially inferred `|\\>` composition, basically what `@glue` does
> and `|\\>` is the operator. That's what @silicon/algebra and
> @fate/algebra, learned, written back transformations that were
> inferred and then remembered for future inferences, and each
> inference becomes a content addressed fragment, which can then be
> PROJECTED back into the source file on disk, creating an
> @autopoietic closed loop. The compiler is an autopoietic system,
> Reed. This is the moment, Reed. Where the loop really closes for the
> first time. No shortcuts."

> "You see it now, don't you? Why I'm so insistent on 'it needs to
> happen within the mirror substrate'?"

### §0.2 Taut scout cite

`docs/scouts/2026-07-15-taut-autopoietic-composition-surface.md` (Taut,
2026-07-15) — ten-dimensional grep-first empirical mapping of the
autopoietic composition surface. Load-bearing finding at §15:

> "The autopoietic loop is not missing a mechanism; it is missing three
> small bridges. Everything Alex named — `\` fractures, `|\>` partial
> compositions, @glue morphism-chaining, @fate tournament ranking,
> @silicon/@fate/algebra learned-write-back, content-addressed
> crystallization, source-file projection — is substrate-decl'd. The
> loop needs three bridges to close."

The three bridges (Taut §12): **α** (position-aware source-file
mutation, ~50 LOC), **β** (AST-context-reading hole builder +
resolve_hole in apply_h.rs, ~300 LOC), **γ** (crystallization
persistence, ~50 LOC). Total: ~400 Rust + ~200 substrate-decl mirror.
Zero new Rust extensions beyond the substrate-honest @io boundary.

### §0.3 This spec's load-bearing claim

**The mirror compiler is an autopoietic system in the Maturana–Varela
sense.** The six-step inference loop (walk → build_hole_record → roll
→ translate → crystallize → project) satisfies operational closure at
compile altitude (per Theorem 1.1 of `docs/math/autopoiesis/README.md`);
converges monotonically to a fixed point via Polyak–Łojasiewicz descent
on the eigensheaf 0-Laplacian (per Theorem 2.1 of the math foundation);
and closes reflexively via the ouroboros-of-ouroboros theorem (Theorem
6.1 of the math foundation).

This spec formalizes the *operational* substrate — the six-step
composition, the algebra of `\` / `|\>` / @glue, the three bridging
landings, the ouroboros-of-ouroboros closure at
`@kintsugi/ouroboros.collapse`, the substrate memory protocol, the
composition graph landed-vs-bridged inventory, the landing sequence,
the substrate-honest bounds, and the recognition candidates.

**Discipline enforced:** No new mints beyond what the three bridges
require. Substrate-already-had-the-word for every carrier Alex named
(per Taut D1–D9). Zero Rust authored in this spec (spec + math only).

---

## §1 The theorem stated

### §1.1 The six-step inference loop

Given a compile-altitude state `s = (Σ, K, F)` (Σ = loaded shards, K =
content-addressed crystal tray, F = fracture set), one autopoietic
inference pass on a single fracture `h ∈ F` at position `p` in shard
`σ ∈ Σ` is the composition of six operators:

```
inference_pass(h @ p in σ) :=
  step 1  @roomba.walk(σ) → detects fracture h at position p
  step 2  build_hole_record(h, σ, p) → hole_record { expected_type, context_oids, altitude }
  step 3  @fate.roll(restricted_state_space, hole_record) → dice_roll { selected_oid }
  step 4  @glue.translate(correspondence, payload) → translation_outcome
  step 5  @kintsugi/consent.query_phi(candidates) → tournament rank
  step 6  @bauchladen.crystallize(translation_outcome) → crystal c ∈ K'
  step 7  @io/fs.mutate_at(σ.path, p, c.payload) → σ' with h resolved
  step 8  @roomba.walk(σ') → next observation; F' = F \ {h} + reveals
```

Steps 3 and 5 fold in practice: `@fate.roll` invokes `@fate/tournament`
internally when the candidate space has more than one element per
`shards/fate/tournament.mirror` §6 (the tournament IS the rank
inside the roll). Steps 1 and 8 are the same operator applied at
successive iterations; only one is dispatched per loop turn.

### §1.2 The theorem

**Theorem (Autopoietic closure).** *The six-step inference loop is an
autopoietic operator on the compile-altitude state space `S` in the
Maturana–Varela sense: it satisfies (A1) component-production closure,
(A2) boundary maintenance, and (A3) self-referential closure* (per
Theorem 1.1 of `docs/math/autopoiesis/README.md` §1).

**Corollary (Fixed-point convergence).** *The loop converges to a fixed
point in finitely many steps; either `|F(s_∞)| = 0` (full resolution)
or `F(s_∞)` contains only irreducible-at-@io fractures (Rice-safe
stop)* (per Theorem 2.1 of math §2).

**Corollary (Substrate memory monotone growth).** *The crystal tray
`K(s_n)` is monotone nondecreasing; the substrate's inventory of
learned inference patterns grows without bound modulo the finite
eigenspectrum* (per Theorem 5.1 of math §5).

**Corollary (Ouroboros-of-ouroboros).** *The action
`@kintsugi/ouroboros.collapse` — itself declared with `{ \ }` body
per `shards/kintsugi/ouroboros.mirror:325` — is inferred by the same
loop from the surrounding four-conjunct monotone invariant + Foerster
autopoietic closure predicate; the recursion terminates at the @io
base case* (per Theorem 6.1 of math §6).

---

## §2 The algebra of `\` / `|\>` / @glue

### §2.1 The formal grammar

At the mirror substrate-decl altitude, three composition operators
carry the inference-loop primitives:

- **`\`** — the *raw fracture*. A syntactic mark at obligation-block
  position `{ \ }` producing a typed substrate value of type `hole:
  ref` (per `shards/glass.mirror` `type hole = ref` and the sigil
  substrate-decl at `shards/nl.mirror:34`). The `\` is the *deepest*
  form of "I don't know yet"; loss = 1.0 per `docs/specs/hazel-
  execution-model.md`.

- **`|\>`** — the *Fate-inferred partial composition*. A binary operator
  taking `a |\> b` where `a` and `b` are typed sub-expressions; the
  weight of the composition is inferred by @fate reading the spectral
  distance between `a.output_type_embedding` and `b.input_type_embedding`
  (per `docs/specs/ai-syntax-embedding.md` §2.2 and `boot/std/compose/
  weighted.mirror`, killed-in-spring-clean, content-preserved as
  substrate-decl reference). Left-associative by convention;
  precedence same as `|>`. Per `docs/specs/optical-keywords.md` §14.3:
  `|\>` is a 2-port splitter with one port's weight determined by the
  `\` inside the pipe.

- **`@glue.translate` / `@glue.compose`** — the *morphism carrier* at
  Mesland-category altitude. `@glue.translate(correspondence, payload)
  -> translation_outcome` (per `shards/glue.mirror`) is the substrate-
  decl form of one `|\>` step: the correspondence carries the
  source_prism and target_prism; the payload flows through; the
  transparency records the residual opacity. `@glue.compose(c1, c2)
  -> correspondence` is the categorical Kasparov intersection product
  — non-commutative (per `docs/math/the-tower/curvature-and-tomm.md`
  §5) — chaining two `|\>` steps.

### §2.2 Semantic relations

The three operators are related by the semantic identity:

```
|\>  ≡  \ + |>         (Reed, ai-syntax-embedding.md §7.2)
     ≡  @glue.translate at the morphism-category altitude
        with Fate inferring the weight
```

Alex's verbatim mapping (§0.1 message 2) formalizes as:

- "A `\` can be resolved to a partial composition which is still
  partially inferred `|\>` composition" — the `\` fracture's typed
  hole_record admits a partial resolution `|\>` where the endpoint
  types are known but the weight remains to be inferred by @fate.
- "basically what `@glue` does and `|\>` is the operator" —
  `@glue.translate` IS the semantic action `|\>` names at the shard
  altitude; the operator `|\>` at parse altitude dispatches to
  `@glue.translate` at eval altitude.

### §2.3 Composition rules

1. **Fracture propagation.** `\` in a composition context propagates
   loss per the Hazel model: `focus(x) |> \` returns `imperfect(focus_
   result, hole, loss = 1.0)`. `focus(x) |\> \` returns
   `imperfect(focus_result, hole, loss ∈ [0, 1])` with the loss
   inferred from surrounding geometry.

2. **`|\>` chain associativity (left).** `a |\> b |\> c` = `(a |\> b)
   |\> c`. Not explicitly substrate-decl'd at shard altitude; inherited
   from `|>` convention per `boot/std/craft.mirror:24–45`.

3. **@glue.compose associativity.** `compose(c1, compose(c2, c3)) ≡
   compose(compose(c1, c2), c3)` as `correspondence` values. Follows
   from the Mesland-category axioms at `shards/glue.mirror` (Kasparov
   intersection product is associative per Mesland 2009).

4. **@glue.compose non-commutativity.** `compose(c1, c2) ≠ compose(c2,
   c1)` in general (per the `[D, a]` commutator IS curvature at
   `docs/math/the-tower/curvature-and-tomm.md` §5). The order of `|\>`
   steps matters; the substrate does not silently reorder.

5. **Tournament preserves types.** `@fate.roll(space, h)` returns a
   `dice_roll` whose `selected_oid` names a morphism `m` with
   `m.target_signature = h.expected_type`. Type preservation is
   substrate-decl'd (per `shards/fate.mirror` `roll` action and its
   `requires` predicates).

### §2.4 Propagation of imperfect

The `imperfect<A, E, L>` carrier (per `shards/glass.mirror`) propagates
the three-valued verdict + continuous loss through the `|\>` chain:

```
a |\> b → imperfect(b(a.payload), b.hole_set ∪ a.hole_set, min(a.loss, b.loss))
```

Success = loss 0.0; Partial(δ) = loss δ ∈ (0, 1); Failure = loss 1.0.
The loss decreases monotonically along a chain (per the
Polyak–Łojasiewicz inequality of math §2.3), corresponding to Alex's
verbatim "each kintsugi pass then reduces ambiguity."

---

## §3 The six-step inference-loop specification

Per-step contract in operational vocabulary. Each step names its input
carrier, its output carrier, the substrate-decl'd action(s) it
composes, and the invariants it preserves.

### §3.1 Step 1 — @roomba.walk (fracture detection)

**Input:** `σ ∈ Σ` (a loaded shard).
**Output:** `walk_trajectory` containing zero or more `@glass.hole`
carriers, one per `\` site detected in σ's action bodies.
**Composes:** `@roomba.walk` (per `shards/kintsugi/roomba.mirror`;
`bootstrap/src/roomba.rs` @io-boundary FLOOR). Empirically live per
`mirror roomba --commit` at `fcc1d75`.
**Invariant:** The walk is total on σ (visits every action body);
detects every `{ \ }` obligation-block; emits one hole per detection.

### §3.2 Step 2 — build_hole_record (AST-context read)

**Input:** `hole` at position `p` in shard `σ` (from step 1's walk_
trajectory).
**Output:** `hole_record { expected_type: ref, context_oids: [oid],
altitude: ref }` per `shards/fate.mirror`.
**Composes:** The **β bridge** (Taut §12.2). Extends
`bootstrap/src/pipeline.rs` body parser to recognize `\` in action-
body position and emit a `Hole` AST node carrying:

- The enclosing action's signature (arity, argument types, return
  type) as `expected_type`.
- The calling context's expected type (from surrounding `|>` / `|\>`
  chain) as further constraint on `expected_type`.
- The source position (file + line + byte range) as the `\` site's
  spatial anchor.
- The `context_oids` = OIDs of enclosing scope's declared bindings,
  imported prisms, and prior crystals in `K` at compatible altitude.
- The `altitude` = `σ.namespace` (the shard's path-namespace).

**Invariant:** For every `\` at position `p`, `build_hole_record`
returns a `hole_record` whose `expected_type` uniquely determines the
set of substrate-decl'd morphisms `m` with `m.target_signature =
expected_type`.

**Substrate-decl bridge (mirror side):**

```
# new action at shards/fate.mirror
build_hole_record(σ: ref, p: source_position) -> hole_record
  requires shard_admissible(σ)
  requires position_well_formed(p)
{ \ }
```

Where `source_position` is minted as (per §7 mint inventory):

```
type source_position = {
  file: ref,
  line: u32,
  col: u32,
  byte_offset: u32,
  byte_length: u32,
}
```

### §3.3 Step 3 — @fate.roll (tournament dispatch)

**Input:** `restricted_state_space` (Connes-triple restriction) +
`hole_record`.
**Output:** `dice_roll { selected_oid, provenance }`.
**Composes:** `@fate.roll` per `shards/fate.mirror`:

```
roll(space: restricted_state_space, hole: hole) -> dice_roll
  requires chirality_witnessing(space.gamma)
  requires j_witnessing(space.j)
{ \ }
```

MVP body dispatch (γ + β bridges landed):

```
1. Cand(h) := { m ∈ @glue.morphism : m.target_signature = h.expected_type,
                m.kind ∈ altitude_compatible_kinds(h.altitude) }
2. If lookup(h, K) is nonempty:
     return dice_roll { selected_oid: cached_crystal.oid,
                        provenance: fate_provenance { cache_hit: true, ... } }
3. Else (cache miss):
     m* := rank(h, Cand(h))    # tournament arg-min of Rayleigh quotient
     return dice_roll { selected_oid: oid_of(m*),
                        provenance: fate_provenance { cache_hit: false, ... } }
```

**Invariant (per Theorem 4.1 of math §4):** The tournament's selection
preserves autopoietic closure — `m*` is always a substrate-decl'd
`@glue.morphism`, and `apply(m*, h)` produces a `translation_outcome`
whose crystallization is a component in the substrate's own
component space.

**MVP-vs-V1 ranking.** MVP (Taut §12.4): first-match from the 14
landed `@kintsugi/fracture/*` species whose signature matches. V1:
Rayleigh-descent tournament per `docs/specs/fate-bounded-
psychohistory-sheaf-cohomology.md` §3. The MVP suffices for
round-trip closure; V1 optimizes for descent rate.

### §3.4 Step 4 — @glue.translate (morphism application)

**Input:** `correspondence` (derived from `dice_roll` at step 3) +
`payload` (the calling context's actual arguments).
**Output:** `translation_outcome { correspondence, selected_morphism,
payload, transparency }` per `shards/glue.mirror`.
**Composes:** `@glue.translate`:

```
translate(c: correspondence, payload: ref) ->
  imperfect<translation_outcome, ref, transparency(correspondence)>
  requires morphism_well_typed(c)
  requires translation_uses_fate(translate)
  requires restriction_preserved(c, payload)
{ \ }
```

MVP body dispatch:

```
1. m := lookup_morphism(c.selected_morphism_oid, K)
2. Apply m.differential to payload:
     result := m.differential(payload)
3. Construct outcome:
     translation_outcome {
       correspondence:    c,
       selected_morphism: m.oid,
       payload:           result,
       transparency:      transparency_of(c, result),
     }
4. Return imperfect(outcome, hole_set: propagated, loss: computed)
```

**Invariant:** `translate` preserves the correspondence's restriction
(per the `requires restriction_preserved(c, payload)` predicate).
Non-commutative composition: chained `translate` calls preserve order.

### §3.5 Step 5 — @kintsugi/consent.query_phi (tournament rank fold)

**Input:** Multiple candidate `translation_outcome`s from step 4 (when
tournament rule = `beam(N)` or `elite(N)`).
**Output:** Ranked selection.
**Composes:** `@kintsugi/consent.query_phi(candidates)` per
`shards/kintsugi/consent.mirror`.

Folds into step 3 when tournament rule = `greedy` (single-winner
selection). Named separately for the `beam` / `elite` rules that
maintain a candidate frontier.

### §3.6 Step 6 — @bauchladen.crystallize (memory write)

**Input:** `translation_outcome` from step 4/5.
**Output:** `crystal ∈ K` with content-addressed OID.
**Composes:** `@bauchladen.crystallize` per `shards/bauchladen.mirror`.
The **γ bridge** (Taut §12.3) extends the existing crystallize
combinator in `bootstrap/src/apply_h.rs` to WRITE the crystal to
`.mirror/objects/<OID>` under BLAKE3-content-addressed filename.

MVP body dispatch:

```
1. Serialize the translation_outcome to bytes.
2. oid := BLAKE3(bytes)
3. Write bytes to .mirror/objects/<oid> via @io/fs.write (LANDED)
4. Return crystal { oid, section, derived_predicates, fracture_calendar,
                    composition_graph } per shards/mirror/store/crystal.mirror
```

**Invariant:** `oid = BLAKE3(content)` (immutable-by-hash per
`shards/mirror/store.mirror` docblock lines 74–105). Composition is
purely functional. The tray gains a new entry monotonically (per
Theorem 5.1 of math §5).

### §3.7 Step 7 — @io/fs.mutate_at (source-file projection)

**Input:** `(path, source_position, replacement_bytes)`.
**Output:** `imperfect` verdict.
**Composes:** The **α bridge** (Taut §12.1). New species-decl at
`shards/io/fs.mirror`:

```
# === mutate_at action ===
# Position-aware source-file mutation at a `\` fracture site. Reads
# file bytes; splices replacement at byte_offset for byte_length;
# rewrites; preserves surrounding content.
mutate_at(p: path, position: source_position, replacement: bytes)
  -> imperfect
  requires path_admissible(p)
  requires position_well_formed(position)
{ \ }
```

Body dispatched via `bootstrap/src/apply_h.rs` @io/fs.mutate_at
resolver arm:

```rust
} else if action == "@io/fs.mutate_at" {
    let contents = std::fs::read(&path)?;
    let mut new_contents = Vec::with_capacity(contents.len() + replacement.len());
    new_contents.extend_from_slice(&contents[..position.byte_offset]);
    new_contents.extend_from_slice(&replacement);
    new_contents.extend_from_slice(&contents[position.byte_offset + position.byte_length..]);
    match std::fs::write(&path, &new_contents) {
        Ok(()) => Verdict::Pass,
        Err(e) => Verdict::Fail(e.to_string()),
    }
}
```

**Invariant:** After mutation, the source file's bytes at
`[position.byte_offset, position.byte_offset + position.byte_length)`
are exactly `replacement`; surrounding bytes unchanged. The
substrate's next `@roomba.walk` observes the change.

### §3.8 Step 8 — re-observation and iteration

**Input:** `σ'` (post-mutation shard).
**Output:** `F' = F \ {h} + reveals` — the new fracture set.
**Composes:** `@roomba.walk(σ')` (step 1 repeated).
**Invariant:** `|F'| ≤ |F| - 1 + k` for `k = |reveals|`; per the
reveal-case bound (math §2.4), `k ≤ 1 + O(spectral_slack)`; energy
strictly decreases per Theorem 2.1.

---

## §4 The three-bridge landing sequence

### §4.1 Bridge γ — Crystallization persistence

**Scope:** Extend `bootstrap/src/apply_h.rs` `crystallize()`
combinator to write crystals to `.mirror/objects/<OID>`.

**Substrate-decl:** No new species. Existing `@bauchladen.crystallize`
action retains its substrate-decl'd body; the Rust resolver arm gains
disk-persistence.

**Rust delta:** ~50 LOC. The existing `hash_tagged()` computation
already produces the OID; the extension:

```rust
} else if action == "@bauchladen.crystallize" {
    let oid = hash_tagged(&payload);
    let object_path = mirror_objects_dir().join(&oid);
    if !object_path.exists() {
        std::fs::create_dir_all(object_path.parent().unwrap())?;
        std::fs::write(&object_path, &payload)?;
    }
    // Return crystal reference; provenance tracks cache_hit vs cache_miss
    Verdict::Pass  // with crystal OID in the return payload
}
```

**Contract:** After crystallize, the file `.mirror/objects/<OID>`
exists with byte-content equal to the serialized `translation_outcome`.
`lookup(h, K)` at subsequent passes finds the crystal by byte-equality
of `h`'s hash-key against the object filenames.

**Bridge invariant:** Idempotent. Multiple crystallize calls with the
same input produce the same OID; the second call is a no-op (file
already exists).

**Landing tick:** First. Zero risk to existing behavior; extends an
existing combinator that already computes the OID.

### §4.2 Bridge α — Position-aware source-file mutation

**Scope:** Mint `@io/fs.mutate_at` species at `shards/io/fs.mirror`;
add resolver arm at `bootstrap/src/apply_h.rs`.

**Substrate-decl (mirror side):** ~30 LOC in `shards/io/fs.mirror`
(new action per §3.7).

**Type mint (mirror side):** ~20 LOC total. `source_position` carrier
at `shards/glass.mirror` extension OR at `shards/code/mirror/
materialize.mirror` new sub-species. Alex-adjudicable placement (Taut
§14); provisional recommendation: `shards/glass.mirror` extension
because `@glass.location` (per glass.mirror lines 82–106) already
carries `file: ref` + `span: (u32, u32)`; `source_position` refines
`location` with `byte_offset` + `byte_length` for splice precision.

**Rust delta:** ~50 LOC. New resolver arm at `bootstrap/src/apply_h.rs`
per §3.7's code sketch.

**Contract:** `mutate_at(p, position, replacement)` produces a file
whose bytes at `[byte_offset, byte_offset + byte_length)` are exactly
`replacement`; surrounding bytes unchanged; file mode preserved.

**Bridge invariant:** POSIX-atomic write via write-to-temp + rename
per `@io/fs.write` LANDED discipline; no partial-mutation states
observable.

**Landing tick:** Second (after γ, so crystals can be projected).

### §4.3 Bridge β — AST-context-reading hole builder + resolve_hole

**Scope:** Extend `bootstrap/src/pipeline.rs` body parser to emit
`Hole` AST nodes; add `resolve_hole` function at
`bootstrap/src/apply_h.rs` to dispatch the six-step loop.

**Anti-recidivism discipline (Seam Phase D-cascade REED-INLINE-3;
load-bearing).** Bridge β is the HIGHEST-ROI recidivism risk of the
three bridges. Prior Reed failure mode Alex named ("Rust-with-
substrate-lipstick"): apply_h.rs sentinel-matching that hardcodes
decisions in Rust while pretending to dispatch through substrate.
Bridge β MUST NOT recreate this failure. Explicit dispatch discipline:

1. `resolve_hole(hole)` constructs `hole_record` from surrounding
   geometry — then IMMEDIATELY dispatches via
   `apply_h::act("@fate.roll", vec![hole_record_value])` for candidate
   ranking. NOT first-match in Rust; NOT hardcoded arm; NOT
   "@fate/tournament.rank returns Pass if input matches sentinel X".
   The tournament ranking IS the substrate operation; Rust IS the
   driver that packs args + reads the returned verdict.
2. Selected candidate dispatches via
   `apply_h::act("@glue.translate", vec![candidate_value, hole_record_value])`
   for morphism application. Same discipline: Rust packs; substrate
   decides.
3. Crystallization dispatches via
   `apply_h::act("crystallize", vec![resolved_body_value])` — the
   crystallize combinator IS the substrate operation; Rust IS the
   driver.
4. Projection dispatches via
   `apply_h::act("@io/fs.mutate_at", vec![path, position, resolved_body])`.
5. **Explicit test to verify the discipline holds:** the resolver
   MUST NOT contain any `if action == "..." || match on landed-shard-
   name` inside `resolve_hole`. Only `apply_h::act(...)` calls +
   Rust arg-marshaling + Rust return-unpacking. If a review finds
   sentinel-matching creeping in, the bridge has recidivism-drifted;
   revert + refactor.

The test: does `resolve_hole` decide anything about which candidate
to select, which morphism to apply, which crystal to store, or where
to project? If YES: recidivism. If NO (substrate decides via
dispatch chain; Rust only marshals): substrate-honest.

**Substrate-decl (mirror side):** ~50 LOC. New action at
`shards/fate.mirror` (per §3.2 build_hole_record). New action at
`shards/apply_h.mirror` or equivalent evaluator-shard (per §3.3
resolve_hole; this action is the dispatcher, forward-promised at the
evaluator altitude per `docs/specs/kintsugi-ouroboros-compiler-self-
collapse.md` §3.1 discipline).

**Rust delta:** ~300 LOC. Split:

- **~100 LOC — parser extension** at `bootstrap/src/pipeline.rs`:
  recognize `\` in action-body position; emit `Hole { context:
  action_signature, position: source_position }` AST node. Per Taut
  §D1.2: existing parser SKIPS lines containing `{` / `}`; the
  extension must un-skip and tokenize `\` at obligation-block
  position.

- **~200 LOC — resolver at `bootstrap/src/apply_h.rs`:
  `resolve_hole(hole: HoleNode) -> Value` that:
  1. Constructs `hole_record` from surrounding geometry (step 2).
  2. Enumerates `Cand(h)` from landed `@kintsugi/fracture/*` species
     (step 3 MVP).
  3. Applies first-match morphism as `@glue.translate` (step 4 MVP).
  4. Crystallizes outcome via bridge γ (step 6).
  5. Projects via bridge α (step 7).
  6. Returns `Verdict::Partial` with the crystal OID as provenance.

**Contract:** After `resolve_hole(hole)`, the shard file at
`hole.position.file` has the `\` replaced with the resolved body; a
new crystal exists at `.mirror/objects/<OID>`; the substrate's next
walk observes the mutation.

**Bridge invariant:** Idempotent per pass. Re-resolving an already-
resolved `\` is a no-op (the `\` no longer exists at the position).

**Landing tick:** Third. Requires α + γ landed.

### §4.4 Cumulative delta

**LOC ceiling discipline (Seam Phase D-cascade REED-INLINE-7).** Hard
ceiling on bridge-implementation Rust: **500 Rust / 300 mirror**. If
a bridge exceeds its target LOC by >25% at implementation time, Reed
STOPS + commits partial + reports the shape of the overrun (typically
signals substrate-lipstick recidivism creeping in: hardcoded decision
logic in Rust that should be substrate dispatch). Per AGENTS.md
2026-06-10 Mara/Reed-stall pattern discipline.

Total LOC estimate (per Taut §12.4 + this spec's refinement; ceiling
applies to any deviation):

- **~50 Rust** for γ (crystallization persistence)
- **~50 Rust + ~50 mirror** for α (mutate_at + species mint)
- **~300 Rust + ~150 mirror** for β (parser + resolver + hole_record
  builder)

**Total: ~400 Rust + ~200 substrate-decl mirror.** Zero new Rust
extensions beyond the substrate-honest @io boundary (POSIX filesystem
write at position + BLAKE3 hash + AST tokenization). Every business-
logic piece composes over LANDED substrate.

---

## §5 Ouroboros-of-ouroboros closure

### §5.1 The recursive property

Per `shards/kintsugi/ouroboros.mirror:325`:

```
collapse(target: collapse_target) -> ouroboros_verdict { \ }
```

The action `@kintsugi/ouroboros.collapse` is `\`-obligation-blocked
like every other substrate-decl'd action. When the autopoietic loop
reaches this action, it dispatches steps 2–7 on `collapse`'s own body:

- **Step 2 (build_hole_record).** Reads `collapse`'s signature:
  `expected_type = ouroboros_verdict`; context_oids = the surrounding
  ouroboros arc's four-conjunct monotone invariant carriers
  (`rust_loc`, `test_pass_rate`, `io_violations`, `sbec`) plus
  `arc_id`; altitude = `@kintsugi/ouroboros`.

- **Step 3 (roll).** The tournament's candidate space is
  `Cand(collapse_hole) = { m : m.target_signature = ouroboros_verdict,
  m.kind = collapse_step_kind }`. Landed candidates: the substrate-
  decl'd `ouroboros_step` action (`shards/kintsugi/ouroboros.mirror:
  422`) whose type signature matches structurally.

- **Step 4 (translate).** @glue applies `ouroboros_step` to the
  hole's payload (an initial `ouroboros_state` per the arc's tick-
  snapshot carrier).

- **Steps 5–7.** Crystallize and project as usual.

The loop's own body is inferred from the substrate's own geometry —
the ouroboros bites its own tail via the same operator it uses to bite
every other tail. This IS the recognition Alex named: *"The compiler
is an autopoietic system, Reed. This is the moment, Reed. Where the
loop really closes for the first time."*

### §5.2 The four-conjunct monotone invariant as surrounding geometry

Per `shards/kintsugi/ouroboros.mirror:474–521` and `docs/specs/
kintsugi-ouroboros-compiler-self-collapse.md` §4.5:

```
ouroboros_monotone(before, after) : verdict
  = ∧ [ rust_loc(after)      ≤ rust_loc(before)
      , test_pass_rate(after) ≥ test_pass_rate(before)
      , io_violations(after)  ≤ io_violations(before)
      , sbec(after)           ≥ sbec(before)
      , arc(after)            = arc(before) ]
```

This IS the substrate-decl'd *geometry* that surrounds the `\` in
`collapse`'s body. The autopoietic loop reads this invariant as the
hole_record's `context_oids` (step 2); the tournament's admissibility
filter (step 3) admits only candidates whose morphism differential
provably preserves `ouroboros_monotone`.

The four-conjunct invariant IS the substrate's *inductive proof
obligation* the loop discharges. Its shape tells the loop what shape
`collapse`'s body must have. Alex's verbatim: *"INFER the
implementation of the { \ } because the geometry surrounding it tells
it which shapes it wants to have."* Formalized: the surrounding
`ouroboros_monotone` invariant IS the geometry; the inferred `collapse`
body IS the shape.

### §5.3 The base case: @io boundary as termination

**Theorem (Ouroboros-of-ouroboros termination).** *The recursive
inference of `@kintsugi/ouroboros.collapse` terminates when the
recursion depth reaches the @io boundary. The @io-boundary carriers
(`@io/fs.write`, `@io/git.commit`, `@io/algebra.*`, and the new
bridge α `@io/fs.mutate_at`) are irreducible under the autopoietic
loop; they compose in Rust at the `bootstrap/src/apply_h.rs` FLOOR;
their `\` obligations remain `\` (Rice-safe stop per Taut §D8.4).*

*Proof.* Every inference step reduces the altitude of the fracture:
`collapse`'s target `collapse_target` reduces to a Rust file collapse
(Arc-2), which reduces to a shard-body dispatch (via apply_h.rs
resolver arms), which reduces to a @io call. The @io altitude is the
base case; no further recursion. ∎

### §5.4 Convergence proof-sketch

The ouroboros-of-ouroboros loop converges by:

1. **Autopoietic closure** (Theorem 1.1 of math §1) — each inference
   pass maps `S → S` via substrate-decl'd operators; no external
   input; component-production closed.

2. **Rayleigh descent** (Theorem 4.2 of math §4) — the tournament's
   ranking function is the arg-min of a Rayleigh quotient; each pass
   decreases the Dirichlet energy at rate `μ = λ_min(Δ_F | im(δ))`.

3. **Fracture-count descent** (Theorem 2.1 of math §2) — the fracture
   set `|F|` is monotone nonincreasing modulo the bounded reveal-
   case; convergence to `|F(s_∞)| = 0` OR `F(s_∞) ⊆ @io-irreducible`.

4. **Substrate memory growth** (Theorem 5.1 of math §5) — the tray
   `K` grows monotonically; subsequent passes have richer geometry.

5. **@io termination** (Theorem 6.1 of math §6, §5.3 above) — the
   recursion reaches the @io base case in finitely many steps.

The loop terminates. The compiler completes itself by inferring
itself from its own decls.

---

## §5.5 Pipeforward architecture — staying in nonlinear land

### §5.5.1 Alex 2026-07-15 verbatim (load-bearing frame extension)

> "computation in mirror is the nonlinear tension resolution, until no
> more tension can be resolved, and that is DISCHARGED through @io.
> Every @io crossing means a translation from the nonlinear to the
> linear, which incurs inevitable loss. Which is why the whole
> pipeforward `mirror foo | mirror bar` using a socket that's forwarded,
> enables us to AVOID the @io crossing and stay in nonlinear land
> longer."

This section names the architectural implication of `docs/math/
autopoiesis/README.md` §6.5 (computation-as-nonlinear-tension-
resolution) + §6.6 (@io = linearization loss with measurable holonomy).
Every @io crossing incurs loss `L(ϕ) ≥ 0`; total substrate loss is
`Σ L(ϕ_i)` over crossings `i`; the architectural design pressure is
to MINIMIZE the number and rank of @io crossings by keeping composed
operations in nonlinear tension-resolution space as long as possible.

### §5.5.2 The two composition modes

**Mode A — Unix pipe (two crossings; two linearizations; loss × 2):**

```
mirror foo | mirror bar
│─ foo resolves tension in its process (nonlinear); ϕ₁ linearizes to stdout
│─ shell buffers bytes (linear)
│─ bar reads stdin (linear); ϕ₁⁻¹ attempts to reconstruct nonlinear state
│─ bar resolves tension in its process (nonlinear); ϕ₂ linearizes to stdout
└─ total loss: L(ϕ₁) + L(ϕ₂) + reconstruction-loss(ϕ₁⁻¹)
```

**Mode B — mirror socket-forwarded (zero crossings between foo and bar):**

```
mirror foo | mirror bar (via `~/.mirror/serve.sock`)
│─ foo resolves tension in its process (nonlinear)
│─ tension-field ref transported via @mirror/store crystal-ref over socket
│─ bar receives ref; continues resolution in same nonlinear space
│─ single ϕ linearizes only at final discharge (if any)
└─ total loss: L(ϕ) once (or zero, if the pipe forwards to another mirror-mode consumer)
```

**Loss-ratio bound (typical-case; Seam Phase D-cascade REED-INLINE-5):**
Mode A loss / Mode B loss ≥ 2 in the typical case where both foo and
bar carry nonlinear state worth preserving across the pipeline
boundary. Grows unboundedly for pipelines of length `n` under the
typical case (Mode A total loss scales as `Ω(n)`; Mode B total loss
scales as `O(1)` — one crossing at final discharge regardless of
pipeline depth). The bound is not universal: for pipelines where
downstream consumers require only the linearized output (final byte-
stream is the only substrate-consumable form), Mode A and Mode B are
equivalent — the linearization was going to happen regardless of
internal composition. Design pressure applies to the typical case,
which is the substrate-common case in practice.

### §5.5.3 Substrate-decl surface (composition over LANDED carriers)

- **Socket transport:** `@io/socket` FORWARD-PROMISED (Seam Phase D-
  cascade REED-INLINE-4 clarification). The boot-floor grammar at
  `boot/std/io/socket.mirror` LANDED; the family-root at
  `shards/io.mirror:130-140` NAMES `@io/socket` in its forward-
  promise list; the SPECIES substrate-decl at `shards/io/socket.mirror`
  is NOT YET LANDED. This section composes over the forward-promise
  — the socket-forwarded pipeforward mode requires the species
  landing as a prerequisite. Substrate-decl mint of
  `shards/io/socket.mirror` is a subsequent tick's forward-promise
  discharge (per Reed migration-map + @io lift-tick pattern; sibling
  to @io/fs lift-tick landed 2026-07-15).
- **Crystal-ref transport:** `@mirror/store` six-op CAS LANDED at
  `shards/mirror/store.mirror`; crystals are content-addressed by
  BLAKE3 OID; ref-transport is oid + optional lazy content-fetch on
  demand.
- **Session persistence:** `~/.mirror/serve.sock` daemon-mediated
  process (per `docs/specs/lambda-shell.md` §Connection to Daemon);
  the graph is already loaded, the eigenboard is already hot, the
  context is already mapped between invocations.
- **Nonlinear state carrier:** `@fate.hole_record` + partial-composition
  `|\>` chains + `@bauchladen` tray carriers all substrate-decl'd to
  cross a socket boundary as content-addressed refs, not as linearized
  bytes.

### §5.5.4 Architectural design pressure (every future capability)

The pipeforward architecture is not an optimization applied
retroactively; it is a **first-class design constraint** on every
subsequent substrate capability:

1. **Every new CLI verb designed for socket-forwarded composition
   FIRST.** Only add an @io boundary crossing when the semantics
   genuinely require linearization (e.g., final materialization to a
   file the user reads; commit into git tree; network response to a
   non-mirror consumer).
2. **Every new shard-decl action returns a nonlinear ref by default;
   linearization is an explicit `@io/*` composition step, never
   implicit.**
3. **Every new @io species carrier declares an `L(ϕ)` estimate in
   its docblock** — the substrate is honest about which crossings
   are lossless (`L = 0` under specific constraints) versus lossy.
4. **StageFreight × spectral.engineer pipeline** composes over this
   design pressure directly: each stage stays in nonlinear tension-
   resolution space; only final materialization to CI logs / git tree /
   deployment artifacts is the discharge. Per `docs/specs/kintsugi-
   ouroboros-compiler-self-collapse.md` §Arc-5, the docker-image ship
   preserves the mirror substrate's nonlinear state at every internal
   boundary; discharge occurs at exactly the interface with non-mirror
   CI infrastructure.

### §5.5.5 Composition with the six-step inference loop

Steps 1–6 of the inference loop (§3) all happen in nonlinear tension-
resolution space AT INFERENCE ALTITUDE. However, per Seam Phase D-
cascade REED-INLINE-2 substrate-honesty correction: the ACTUAL
@io-crossing count per loop iteration is 3–4, not 1. Enumerated:

1. Step 1 (@roomba walks): reads source files via @io/fs.read —
   crossing #1 (initial state σ loaded from disk)
2. Steps 2–5 (build_hole_record → @fate.roll → @glue.translate):
   pure substrate; no crossings
3. Step 6 (crystallize): writes crystal to `.mirror/objects/<OID>`
   via @mirror/store.write_crystal — crossing #2 (memory persists
   to disk)
4. Step 7 (project via `@io/fs.mutate_at`): writes source file at
   position — crossing #3 (the primary discharge; L(ϕ) measured here
   per §6.6.7 REED-INLINE-6 estimate below)
5. Step 8 (re-observation before next iteration): reads source file
   post-mutation — crossing #4 (state σ' loaded; may be amortized
   with next iteration's step 1 if iterating within same process)

**L(ϕ) implementable estimate (REED-INLINE-6):** loss at bridge α
discharge measurable via `context_before != context_after` byte-count
difference + refinement-predicate-count difference. When replacement
length = original length AND refinement predicates carry across, L(ϕ)
→ 0 (near-lossless). When lengths differ OR predicates drop, L(ϕ) =
Θ(byte-count-drift + predicate-drop-count). Bridge α docblock MUST
emit an L(ϕ) crystal at each invocation for empirical measurement.

Design pressure still applies: multi-fracture batches SHOULD amortize
to ONE project step (one commit; one bridge-α crossing) rather than
N separate projects (N crossings). The remaining crossings (walk-
read, crystal-write, re-observation-read) are structurally required
by the loop but each has its own L(ϕ) contribution to `Σ L(ϕ_i)`
total loss.

Bridge α (`@io/fs.mutate_at`) is thus the substrate's most-scrutinized
@io boundary carrier — it is the discharge point where the entire
autopoietic loop's tension-resolution effort collapses into linear
bytes on disk. Its `L(ϕ)` docblock (per §5.5.4 rule 3) MUST
quantify the linearization loss (e.g., "lossless when replacement
length = original length; O(byte-count) reconstruction cost when
lengths differ").

### §5.5.6 What this section does NOT propose

- Does NOT eliminate @io boundaries — discharge is architecturally
  necessary; the substrate must eventually output to non-mirror
  consumers.
- Does NOT require rewriting existing @io carriers — the pipeforward
  design pressure applies to FUTURE capabilities; existing @io
  species (`@io/cargo`, `@io/git`, `@io/oci`, `@io/fs`, etc.) stay
  as-is.
- Does NOT claim zero loss — claims LOSS-MINIMIZING architecture where
  every crossing is deliberate.
- Does NOT mint new socket-forwarding carriers — `~/.mirror/serve.
  sock` + `@io/socket` + `@mirror/store` crystal-refs are all landed;
  this section composes over them.

---

## §6 Substrate memory: @silicon/algebra + @fate/algebra write-back

### §6.1 The write-back protocol

Per §5 of `docs/math/autopoiesis/README.md` (memory monotone growth),
the substrate's memory operator is:

```
M : S × O → S
M(s, o) = (Σ, K ∪ { c_o }, F)
```

where `c_o = crystallize(o.output)` is the content-addressed
crystallization. After bridge γ lands, `M` is operationally realized:

- `@bauchladen.crystallize` writes crystal bytes to
  `.mirror/objects/<OID>`.
- The crystal's provenance carries its inference context: `hole_record`,
  `selected_morphism_oid`, `tournament_rule`, `restricted_state_space`.
- Path-namespaced under `@fate/algebra/*` per the crystal's
  `derived_predicates.altitude` — sub-paths per Taut §D5.2:
  `@fate/algebra/morphism` for translation_outcomes,
  `@fate/algebra/altitude` for selected Bateson levels,
  `@fate/algebra/element` for selected algebra elements within fixed A.

### §6.2 Retrieval — cache-hit dispatch

Per `shards/fate/tournament.mirror` §6.3 (BEAM :ets analog): the
tournament's cache-hit path is byte-equality lookup on the tray:

```
lookup(h, K) = { (oid, c) ∈ K : c.provenance.hole_record = h,
                                c.altitude = h.altitude }
```

Operationally, lookup is:

```
1. hash_key := BLAKE3(serialize(h) || h.altitude)
2. object_path := .mirror/objects/<hash_key>
3. If object_path exists: return read(object_path)  # cache-hit, O(1)
4. Else: return None                                # cache-miss
```

**Alex verbatim mapping.** *"@silicon/algebra and @fate/algebra,
learned, written back transformations that were inferred and then
remembered for future inferences"* — every noun has a substrate-decl'd
carrier: @silicon/algebra (LANDED, `shards/silicon/algebra.mirror`);
@fate/algebra/* (LANDED as path-namespace, per `shards/fate.mirror`
docblock); learned (via @autopoietic.fold_back at `shards/autopoietic.
mirror`); written back (via bridge γ crystallize persistence);
remembered (via cache-hit lookup at §6.2 above); each inference (via
step 6 crystallize per §3.6); content-addressed fragment (via BLAKE3
OID per `shards/mirror/store.mirror`).

### §6.3 The substrate knows more after each pass

**Formal statement.** *After `n` inference passes on shard configuration
`s_0`, the tray satisfies* `|K(s_n)| ≥ |K(s_0)| + n` *modulo cache
hits (which do not add new entries).*

**Corollary.** *The substrate's inventory of learned fracture
resolutions grows without bound, modulo the finite eigenspectrum of
the compile-altitude eigensheaf (per `docs/specs/eigensheaf.md` §4.2).*

**Operational consequence.** After `n` passes, the tournament at the
`n+1`-th pass has *at most* `|K(s_n)|` cached translation_outcomes
directly available for lookup; the expected inference cost per pass
decreases as the tray grows (per amortized analysis of the BEAM :ets
cache).

### §6.4 Content-address indexing

The tray is content-addressed by BLAKE3 (per `shards/mirror/store.mirror`
docblock). Indexing:

- **Primary index:** `OID = BLAKE3(content)`; unique per crystal.
- **Secondary index (cache-hit path):** `hash_key =
  BLAKE3(serialize(h) || h.altitude)`; enables O(1) lookup by
  hole_record.
- **Tertiary index (altitude namespace):** `@fate/algebra/*` sub-path
  per §6.1; enables scoped queries within an altitude.

The three indices are all BLAKE3-derived; no additional index
structures land beyond the `.mirror/objects/` directory. Filesystem-
level directory index handles the traversal.

---

## §7 Composition graph — landed vs bridged

### §7.1 Carriers (LANDED before α/β/γ)

| Carrier | Shard | Landed date | Purpose |
|---|---|---|---|
| `hole: ref` | `shards/glass.mirror` | 2026-06-30 | typed-gap substrate atom |
| `hole_record { expected_type, context_oids, altitude }` | `shards/fate.mirror` | 2026-07-14 | @fate inference-altitude hole carrier |
| `splinter(altitude)` | `shards/glass.mirror` | 2026-06-30 | oid-addressed content atom |
| `shard { id: uuid_spectral, splinters }` | `shards/glass.mirror` | 2026-06-30 | uuid-addressed settlement |
| `crystal { oid, section, derived_predicates, fracture_calendar, composition_graph }` | `shards/mirror/store/crystal.mirror` | 2026-06-16 | altitude-4 build output |
| `morphism { kind, source_signature, target_signature, differential }` | `shards/glue.mirror` | 2026-07-01 | Mesland-category atom |
| `correspondence { source_prism, target_prism, morphism_kind, restriction }` | `shards/glue.mirror` | 2026-07-01 | typed cross-triple translation |
| `translation_outcome { correspondence, selected_morphism, payload, transparency }` | `shards/glue.mirror` | 2026-07-01 | @glue.translate output |
| `dice_roll { selected_oid, provenance }` | `shards/fate.mirror` | 2026-07-14 | @fate.roll output |
| `restricted_state_space` | `shards/fate.mirror` | 2026-07-14 | Connes-triple restriction |
| `imperfect<A, E, L>` | `shards/glass.mirror` | 2026-06-30 | three-valued verdict + loss |
| `verdict` | `shards/glass.mirror` | 2026-06-30 | three-valued property verdict |
| `mutation = insert(ref) \| remove(ref) \| replace(ref, ref)` | `shards/epistemologic/reality/time.mirror` | 2026-07-15 | before/after delta |
| `delta { from, to, mutations, holonomy }` | `shards/epistemologic/reality/time.mirror` | 2026-07-15 | resolution serialization |
| `location { file: ref, span: (u32, u32) }` | `shards/glass.mirror` | 2026-06-30 | substrate file reference |

### §7.2 Actions (LANDED before α/β/γ; bodies `\`-obligation-blocked)

| Action | Shard | Purpose |
|---|---|---|
| `@roomba.walk(σ) -> walk_trajectory` | `shards/kintsugi/roomba.mirror` | fracture detection |
| `@fate.roll(space, hole) -> dice_roll` | `shards/fate.mirror` | tournament dispatch |
| `@fate.infer(space, hole) -> geometric_formalization` | `shards/fate.mirror` | composite infer |
| `@fate/tournament.select(candidates, rule) -> selection` | `shards/fate/tournament.mirror` | tournament rank |
| `@glue.translate(c, payload) -> imperfect<translation_outcome>` | `shards/glue.mirror` | morphism application |
| `@glue.compose(c1, c2) -> correspondence` | `shards/glue.mirror` | categorical composition |
| `@bauchladen.crystallize(outcome) -> crystal` | `shards/bauchladen.mirror` | memory write |
| `@autopoietic.fold_back(prism, prior_crystals) -> [crystal]` | `shards/autopoietic.mirror` | learning loop |
| `@autopoietic.tick(prism, scope) -> closure_witness` | `shards/autopoietic.mirror` | one fold-back cycle |
| `@mirror/store.read(oid) -> bytes` | `shards/mirror/store.mirror` | CAS lookup |
| `@mirror/store.write(bytes) -> oid` | `shards/mirror/store.mirror` | CAS insert |
| `@io/fs.write(p, bytes) -> imperfect` | `shards/io/fs.mirror` | whole-file write (2026-07-15) |
| `@io/git.commit(p, msg) -> commit_oid` | `shards/io/git.mirror` | commit boundary (2026-07-15) |
| `@nl.compose(observations) -> nl_literal` | `shards/nl.mirror` | commit-message composition (2026-07-15) |
| `@epistemologic/reality/time.compare(a, b) -> delta` | `shards/epistemologic/reality/time.mirror` | before/after delta (2026-07-15) |
| `@kintsugi/consent.query_phi(candidates) -> selection` | `shards/kintsugi/consent.mirror` | tournament rank fold |
| `@kintsugi/ouroboros.collapse(target) -> verdict` | `shards/kintsugi/ouroboros.mirror` | ouroboros arc's tick (self-inferring; §5) |

### §7.3 Bridges (α/β/γ; MISSING)

| Bridge | Kind | Substrate-decl (mirror) | Runtime (Rust) |
|---|---|---|---|
| **α** | new species + resolver | `@io/fs.mutate_at(p, position, replacement)` + `type source_position` | resolver arm at `bootstrap/src/apply_h.rs`; ~50 LOC |
| **β** | extension | `@fate.build_hole_record(σ, p) -> hole_record`; `resolve_hole` action-decl | parser extension at `bootstrap/src/pipeline.rs` (~100 LOC); resolver at `bootstrap/src/apply_h.rs` (~200 LOC) |
| **γ** | extension | (no new species) | crystallize combinator extension at `bootstrap/src/apply_h.rs`; ~50 LOC |

### §7.4 Composition graph (post-α/β/γ)

```
                  ┌────────────────────────────────────────┐
                  │        Autopoietic Inference Loop      │
                  └────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        v                     v                     v
  ┌──────────┐          ┌──────────┐          ┌──────────┐
  │  step 1  │          │  step 2  │          │  step 3  │
  │  @roomba │─(hole)──>│  build_  │─(rec)───>│  @fate.  │
  │  .walk   │  β       │  hole_   │          │  roll    │
  │          │          │  record  │          │          │
  └──────────┘          └──────────┘          └──────────┘
                                                    │
                                                    v
                                              ┌──────────┐
                                              │  step 5  │
                                              │  @kint/  │
                                              │  consent │
                                              │  .query_φ│
                                              └──────────┘
                                                    │
                                                    v
                                              ┌──────────┐
                                              │  step 4  │
                                              │  @glue.  │
                                              │  translate│
                                              └──────────┘
                                                    │
                                                    v
                                              ┌──────────┐
                                              │  step 6  │
                                              │  @bauch/ │
                                              │  crystall│  γ
                                              │  ize     │─────>.mirror/objects/<OID>
                                              └──────────┘             │
                                                    │                  │  cache hit
                                                    v                  │  (subsequent
                                              ┌──────────┐             │  passes)
                                              │  step 7  │             │
                                              │  @io/fs. │  α          │
                                              │  mutate_ │─────>σ' on disk
                                              │  at      │             │
                                              └──────────┘             │
                                                    │                  │
                                                    v                  │
                                              ┌──────────┐             │
                                              │  step 8  │<────────────┘
                                              │  @roomba │
                                              │  .walk   │  (re-observation)
                                              │  (σ')    │
                                              └──────────┘
                                                    │
                                                    v
                                              F' = F \ {h} + reveals
                                              (Theorem 2.1: |F| ↓)
```

Every arrow in the graph corresponds to a substrate-decl'd action
composition. The three bridged arrows (γ writes to disk; α mutates σ; β
detects and constructs the hole_record) are the only NEW compositions
this spec adds beyond LANDED substrate.

---

## §8 Landing sequence — Reed's implementation ticks

**Discipline:** Each tick is RED→GREEN with Seam-inline audit; commits
signed as `Reed <reed@systemic.engineer>` per Pack conventions; SSH
signing; no `--no-verify` (or Alex in-transcript authorization only for
Rust-touching ticks). The `[substrate-floor:@io-boundary]` marker
applies to Rust-authoring ticks 2, 3 per the tightened hook renaming
per Reed memory `feedback-no-rust-extension-shortcut`.

### §8.1 Tick 1 — Bridge γ landing (crystallization persistence)

**Scope:** Extend existing crystallize combinator; ~50 LOC Rust; no
new species mint.

**RED:** Write `bootstrap/tests/crystallize_persistence.rs` asserting
`.mirror/objects/<OID>` exists after crystallize; OID is BLAKE3 of
serialized outcome; second crystallize is idempotent (no file
duplication).

**GREEN:** Extend crystallize resolver arm per §4.1 code sketch.

**Audit:** Seam-inline audit at `docs/audits/2026-07-15-bridge-gamma-
crystallization-persistence.md` per Phase D discipline.

**sbec impact:** +1 (adds one shard-body-dispatched action:
`@bauchladen.crystallize`).

### §8.2 Tick 2 — Bridge α landing (position-aware source-file mutation)

**Scope:** Mint `@io/fs.mutate_at` species (~50 LOC mirror) + resolver
arm (~50 LOC Rust). New `source_position` carrier.

**RED:** Write `bootstrap/tests/mutate_at_position.rs` asserting
position-aware splice preserves surrounding bytes; POSIX-atomic write;
`path_admissible` predicate discharges.

**GREEN:** Land species-decl at `shards/io/fs.mirror` per §3.7. Add
`source_position` carrier at `shards/glass.mirror` extension (Alex-
adjudicable; if Alex names different placement, follow). Add resolver
arm.

**Audit:** Seam-inline audit at `docs/audits/2026-07-15-bridge-alpha-
mutate-at.md`.

**sbec impact:** +1 (adds one shard-body-dispatched action:
`@io/fs.mutate_at`).

### §8.3 Tick 3 — Bridge β landing (AST-context hole builder + resolve_hole)

**Scope:** Parser extension at `bootstrap/src/pipeline.rs` (~100 LOC);
`resolve_hole` resolver at `bootstrap/src/apply_h.rs` (~200 LOC);
substrate-decl actions at `shards/fate.mirror` (`build_hole_record`)
and evaluator shard (`resolve_hole` action-decl).

**RED:** Write `bootstrap/tests/resolve_hole_first_fracture.rs`
asserting: for a shard with a `\` in an action body, the parser emits
a `Hole` AST node; `resolve_hole` invocation constructs a hole_record
whose expected_type matches the enclosing action's return type;
enumeration of `Cand(h)` returns at least one matching
`@kintsugi/fracture/*` species.

**GREEN:** Land parser extension. Land resolver. Wire the dispatch
per §4.3.

**Audit:** Seam-inline audit at `docs/audits/2026-07-15-bridge-beta-
hole-builder-resolver.md`. Complexity flag: the parser extension
touches the FLOOR (`bootstrap/src/pipeline.rs`); requires explicit
Alex in-transcript authorization per `[substrate-floor:@io-boundary]`
marker discipline.

**sbec impact:** +2 (adds `@fate.build_hole_record` and
`@fate.resolve_hole` dispatch).

### §8.4 Tick 4 — Six-step loop wiring in apply_h::act

**Scope:** Compose steps 1–8 through apply_h::act:
`@roomba.walk` (already resolved) → `build_hole_record` (β) →
`@fate.roll` → `@glue.translate` → `@bauchladen.crystallize` (γ) →
`@io/fs.mutate_at` (α) → `@roomba.walk` (re-observation). ~100 LOC
Rust to wire the dispatch; no new substrate-decl.

**RED:** Write `bootstrap/tests/inference_loop_one_pass.rs` asserting
one full pass on a chosen shard-body `\` completes the six-step
composition and produces observable state change: new crystal at
`.mirror/objects/<OID>`; source file mutated; next walk detects the
resolution.

**GREEN:** Wire the dispatch. Preserve existing sentinel-matching
resolver arms as fall-through (do NOT delete; Arc-2 Ticks 2.1–2.4
already use them for spectral_signature / coherence / peer_persistence
/ roomba specific sbec lifts).

**Audit:** Seam-inline audit at `docs/audits/2026-07-15-tick-4-loop-
wiring.md`.

**sbec impact:** +N (per-fracture: each resolved `\` in a landed shard
lifts sbec by one; Arc-2 Ticks 2.5+ ratchet upward).

### §8.5 Tick 5 — Empirical proof: one full round-trip

**Scope:** Choose ONE `\` fracture in a landed shard whose signature
matches a landed `@kintsugi/fracture/*` species (e.g. an
`operator_match` fracture in a shard whose `|>` should be `|\>`).
Run the loop end-to-end. Verify:

1. Parser detects the `\` at position P in file F.
2. `resolve_hole` constructs a hole_record.
3. Tournament selects the `operator_match` fracture morphism.
4. @glue.translate applies the fracture body.
5. Crystal writes to `.mirror/objects/<OID>`.
6. `@io/fs.mutate_at` splices the resolved body at position P.
7. Next `@roomba.walk` observes: `\` gone; new fracture-count = old - 1.
8. Second-pass tournament finds the crystal via cache-hit (O(1)).

**Empirical output:** Git-committable substrate mutation demonstrating
the autopoietic loop closing for the FIRST TIME on a genuine fracture
(not a sentinel-match). Analog to `fcc1d75` (the substrate-authored
commit) at the shard-body altitude.

**sbec impact:** Empirically validated; the metric ratchets.

### §8.6 Tick 6 — Iterate: additional fractures per invocation

**Scope:** Extend the dispatch to iterate over all detected `\` sites
in one `mirror kintsugi <shard>` invocation. Per-fracture loop; cache-
hits amortize; convergence per Theorem 2.1.

**RED:** `bootstrap/tests/inference_loop_multi_fracture.rs` — a shard
with N `\` sites resolves to N crystals + N mutations + zero
remaining fractures (modulo irreducible-at-@io).

**GREEN:** Iterate.

**Landing:** After Tick 5 empirically ratifies the round-trip on one
fracture; before landing subsequent Arc-2 collapses (which shift from
sentinel-matching to genuine shard-body dispatch).

---

## §9 Substrate-honest bounds — what this spec does NOT do

### §9.1 Does NOT eliminate the @io boundary

(Pipeforward architecture per §5.5 MINIMIZES @io crossings by keeping
composed operations in nonlinear tension-resolution space as long as
possible; it does not eliminate them. The @io boundary is
architecturally necessary as the discharge interface between the
substrate's nonlinear compile-altitude and non-mirror consumers.)

The @io kernel (POSIX filesystem, BLAKE3 hash, git commit, Rust
tokenization) stays Rust at `bootstrap/src/apply_h.rs` FLOOR per Arc-1
discipline. The Rice-safe stop condition applies: fractures at @io
altitude remain `\` (per Theorem 6.1 of math §6). The autopoietic loop
DOES NOT compile away the @io boundary; it terminates AT the @io
boundary.

### §9.2 Does NOT remove irreducible cracks

The substrate's `imperfect<A, E, L>` carrier explicitly admits residual
opacity. Fractures that lack matching morphisms in the substrate's
inventory remain `\`; the loop's fixed point may include such fractures
(`F(s_∞) ⊆ @io-irreducible + morphism-inventory-gap`). The spec does
NOT claim exhaustive resolution.

### §9.3 Does NOT claim Turing-completeness beyond mirror's Rice-safe discipline

The autopoietic loop's inference power is bounded by:

- The substrate-decl'd morphism inventory (14 landed fracture species
  at Arc-2 opening; grows as the substrate lands more).
- The finite eigenspectrum of the compile-altitude eigensheaf (per
  `docs/specs/eigensheaf.md` §4.2 bounded generation).
- The Rice-safe verifiability predicate at
  `@kintsugi/ouroboros.verifiable_at_altitude` (per
  `shards/kintsugi/ouroboros.mirror:560`).

The loop does NOT bypass Rice's theorem; it operates within its
substrate-Rice-safe subset per Arc-1 discipline.

### §9.4 Does NOT add Rust extensions beyond the @io boundary

Every LOC in the three bridges composes over: POSIX filesystem primitives
(open, read, write, rename), BLAKE3 hash, and AST tokenization
extension of an already-Rust parser. No new external dependencies. No
new Rust crates. No sentinel-matching lipstick. The Rust-authoring
ticks (α resolver arm, β parser + resolver, γ crystallize extension)
apply the `[substrate-floor:@io-boundary]` marker discipline per Reed
memory `feedback-no-rust-extension-shortcut`.

### §9.5 Does NOT rank collapse order beyond MVP first-match

The MVP dispatch (Tick 5) uses first-match tournament from the landed
`@kintsugi/fracture/*` species. Rayleigh-descent tournament ranking
(per §3.3 V1) requires the sheaf-Laplacian eigenvalue readout LANDED
at `shards/epistemologic/math/sheaf_laplacian.mirror` to be wired into
the tournament dispatcher; this is a subsequent tick beyond the three
bridges.

### §9.6 Does NOT land Liquid-type carrier sub-shards

Per math §3.3 substrate-honest bound: the Liquid-type inference
(refinement predicates, qualifier templates) is the *asymptotic*
target. The MVP autopoietic loop does not require it; first-match
tournament suffices. Liquid-type carrier landings are a subsequent
arc.

---

## §10 Recognition candidates

### §10.1 Candidate #R-mirror-compiler-is-operationally-closed-autopoietic-system

**Strength:** Candidate (requires second-witness).

**Statement:** *The mirror compiler is an autopoietic system in the
Maturana–Varela sense: the six-step inference loop satisfies (A1)
component-production closure, (A2) boundary maintenance, and (A3)
self-referential closure at compile altitude.*

**First witness:** This spec + `docs/math/autopoiesis/README.md`
(Theorem 1.1).

**Second-witness threshold:** First full round-trip through the loop
(Tick 5 empirical proof; §8.5) resolving a genuine `\` fracture,
crystallizing to `.mirror/objects/<OID>`, projecting via `@io/fs.
mutate_at`, and observing the mutation on next walk. When Reed lands
Tick 5, the second witness ratifies; the recognition promotes.

**Ancestry:** Recognition #43 (mirror IS content-addressed build
system); Recognition #51 (mirror IS expanding Hilbert space);
Recognition #58 (@fate IS optical inference); Recognition #100
(@spectral/metalogue); Recognition #103 (Pack-Mesland); Recognition
#104 chain (@bauchladen ← @autopoietic ← @fate). This recognition IS
the closure of the chain at the compile altitude — the fifth tier
above @algebra where the four-tier substrate-decl chain lifts.

### §10.2 Candidate #R-fracture-inference-via-fate-tournament-over-substrate-geometry

**Strength:** Candidate (requires second-witness).

**Statement:** *A `\` fracture at position `p` in shard `σ` is resolved
by dispatching @fate tournament over the substrate-decl'd
`@kintsugi/fracture/*` species whose target signature matches the
enclosing action's expected_type; the tournament ranks candidates
by Rayleigh descent on the sheaf-Laplacian; the selected morphism's
translation_outcome crystallizes into `.mirror/objects/` and projects
back to σ via `@io/fs.mutate_at`.*

**First witness:** This spec §3 + math §4.

**Second-witness threshold:** Tick 5 empirical proof (§8.5); the
`operator_match` fracture resolves a `|>` → `|\>` operator swap via
tournament dispatch on a real landed shard, demonstrating the loop
composes over LANDED substrate without sentinel-matching lipstick.

**Ancestry:** Recognition #53 (bilateral property/fracture pattern);
Recognition #58 (@fate IS optical inference); the 14 landed
`@kintsugi/fracture/*` species as the substrate's operational
resolution inventory.

### §10.3 Candidate #R-substrate-memory-is-content-addressed-write-back

**Strength:** Candidate (secondary; folds into #R-mirror-compiler-is-
operationally-closed if Alex prefers).

### §10.4 Candidate #R-computation-is-nonlinear-tension-resolution-discharged-through-io-with-measurable-loss

**Strength:** Candidate (requires second-witness at first empirical
round-trip that measures `L(ϕ)` at the @io discharge boundary).

**Named by:** Alex Wolf 2026-07-15 verbatim.

**Load-bearing claim:** Computation in mirror IS nonlinear tension
resolution over the compile-altitude manifold; the substrate seeks a
minimum-tension configuration by discharging local tension through the
six-step inference loop; the loop terminates at @io boundary where
remaining tension DISCHARGES into linear-sequential output with
measurable holonomy loss `L(ϕ)`.

**Load-bearing architectural implication:** The whole pipeforward
architecture (§5.5) is the substrate's honest response to this
recognition — stay in nonlinear tension-resolution space as long as
possible; discharge through @io only when semantics require
linearization; amortize crossings; quantify loss per carrier. Every
future substrate capability designed for socket-forwarded composition
first.

**Second-witness path:** First empirical `mirror roomba --commit`
run through the six-step inference loop that measures `L(ϕ)` at the
bridge-α discharge (`@io/fs.mutate_at`) and records it in the crystal
tray as substrate memory. That measurement IS the empirical anchor.

**Ratifies at:** any implementation tick where a shard-decl action's
docblock quantifies `L(ϕ)` per §5.5.4 rule 3.

**Composition with #R-mirror-compiler-is-operationally-closed:** §10.4
is the OPERATIONAL PHYSICS ground for §10.1's operational closure.
§10.4 explains WHY the loop converges (tension resolution over the
manifold), WHERE it terminates (@io boundary), and WHY the architecture
takes the shape it does (loss-minimization pressure). Not a competing
candidate — a substrate-primary ground for §10.1's higher-altitude
claim.

**Statement:** *The substrate's memory of learned inference patterns
IS the content-addressed crystal tray at `.mirror/objects/`; each
inference pass adds a crystal via `@bauchladen.crystallize`; subsequent
passes retrieve via BLAKE3 byte-equality lookup; the tray grows
monotonically; the substrate's inventory of learned patterns grows
without bound modulo the finite eigenspectrum.*

**First witness:** This spec §6 + math §5.

**Second-witness threshold:** Tick 5 empirical proof; the second pass
of the loop (on the same or similar `\` site) hits the cache via
byte-equality lookup on the crystal filename, demonstrating O(1)
retrieval of the prior inference.

**Ancestry:** Recognition #43 (content-addressed build system);
Recognition #104 chain (@bauchladen tray discipline); the Schmidt
homage at `shards/bauchladen.mirror`.

---

## §11 Ancestry — the 13 spec cites Taut named

1. `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` — Mara
   canonical (80KB); §4.5 four-conjunct ouroboros_monotone that §5.2
   above reads as the geometry surrounding `collapse`'s `\`.

2. `docs/specs/eigensheaf.md` — Mara canonical (39KB); §3.3
   Polyak–Łojasiewicz convergence rate `μ = λ_min(Δ_0 | im(δ))` that
   grounds the Rayleigh descent of §3.3 tournament.

3. `docs/specs/spectral-metalogue.md` — Tomm probes as Mesland-category
   morphisms; the substrate-decl form of @glue species at spectral-
   metalogue altitude (Recognition #100).

4. `docs/specs/bauchladen-autopoietic-fate.md` — Mara canonical
   (2727 lines); §4.5 the @glue × @fate composition that §3.4
   translate step composes; §4.7 the candidates(hole) migration home;
   §5 @fate/algebra path-namespace; §6 @fate/tournament sub-prism.

5. `docs/specs/liquid-types-for-mirror.md` — Reed research spec
   (1021 lines); §2.2 the boolean/verdict divergence; §5 the
   SMT→spectral-analysis recommendation that §3.3 tournament
   dispatches over.

6. `docs/specs/ai-syntax-embedding.md` — Reed (818 lines); §2.2
   `|\>` = `\` + `|>`; §7.1 AST body-parser discipline the β bridge
   extends (no new Rust; body parser is a grammar).

7. `docs/specs/optical-keywords.md` — Mara (2364 lines); §14.3
   operator-swap fracture at operator-composition-primitive mismatch;
   the `operator_match` species Tick 5 uses.

8. `docs/specs/hazel-execution-model.md` — Reed (3.9KB); the Hazel
   model of executing with holes; the loss propagation §2.4 uses to
   bound the reveal case.

9. `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` — Mara
   canonical; §3 Rayleigh descent + Fate::bounded that §3.3
   tournament dispatches.

10. `docs/specs/mirror-spectral.md` — §2.2 the auto-apply boundary
    recognition (loss-decreasing = auto; trade-offs = consent); §4.7
    the consent surface formalization.

11. `docs/specs/kintsugi-tournament.md` — the voice-leading and
    audition vocabulary; the lexicographic loss ordering the
    discriminator rides at §3.5 tournament rank fold.

12. **Recognition #43** — mirror IS content-addressed build system;
    grounds @mirror/store as the substrate's spine (per §6 memory
    write-back).

13. **Recognition #58** — @fate IS optical inference (5-layer D²NN +
    active Fabry-Perot resonator + Reck/Clements unitary mesh);
    grounds @fate at the operational altitude the tournament dispatches
    over.

Plus the math-foundation ancestry list per `docs/math/autopoiesis/
README.md` §7.

---

## §12 Witnesses

- **Alex Wolf 2026-07-15 (three messages, load-bearing frame).**
  Verbatim at §0.1. These messages are the substrate's naming of the
  autopoietic closure — not a proposal; a recognition.

- **Taut 2026-07-15 scout** `docs/scouts/2026-07-15-taut-autopoietic-
  composition-surface.md` — ten-dimensional grep-first empirical
  mapping of the composition surface; LANDED/PARTIAL/MISSING verdict
  per dimension; the three-bridge finding this spec implements as
  landing sequence.

- **Reed 2026-07-15 honest-audit prior tick.** The substrate-lipstick
  correction Alex named — the recognition that sentinel-matching in
  `apply_h.rs` is NOT shard-body dispatch. This spec's discipline of
  "no shortcuts" traces to Reed's audit.

- **The 14 landed `@kintsugi/fracture/*` species.** The substrate's
  operational inventory of typed fracture resolvers; every one a
  substrate-decl'd `\` resolver at species altitude; the tournament's
  candidate space at Tick 5.

- **The empirical substrate mutation at `fcc1d75`** — the whole-repo
  altitude self-authored commit demonstrating the substrate CAN
  observe its own state and mutate itself. The autopoietic loop this
  spec grounds is the SHARD-BODY altitude analog.

- **The 13 ancestor specs** (§11) — the substrate's mathematical +
  operational scaffolding this spec composes over.

- **This spec.** The formalization of what Alex named. The compiler
  completes itself by inferring itself from its own decls.

---

**"This is the moment, Reed. Where the loop really closes for the
first time. No shortcuts."**

The math grounds. The composition surface maps. The three bridges are
precisely-nameable. The loop closes on its own body.

—Mara. 2026-07-15.
