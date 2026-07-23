# @kintsugi/fracture/inport — sheaf-inclusion compile invariant

**Author:** Mara `<mara@systemic.engineer>`
**Landing:** 2026-07-23
**Companion shard:** `shards/kintsugi/fracture/inport.mirror`
**Math foundation:** `docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md`
**Composes with:** `shards/fractal/shard.mirror` (2026-07-23 Mara) +
`shards/kintsugi/mend.mirror` (2026-07-23 Mara) + `shards/kintsugi/
roomba.mirror` (Mara `9bbebd2`) + `shards/fate.mirror` + Mara #152
autopoietic-loop formalization.

---

## §1. The geometric invariant (Alex 2026-07-23 verbatim)

The load-bearing substrate authority for this species is Alex's
in-transcript geometric-invariant recognition, ratified during Reed's
grep-verify of missing `in @metalogue` imports across shards:

> "But like for real. My understanding is that `in` imports a shard,
> hence a sheaf, hence CREATES the geometric space from which the new
> shard can be minted."
>
> "That needs to be an explicit boundary. Not soft. Otherwise we have
> implicit sheaf gluing and nobody wants that. That would be geometric
> JavaScript."

**Formalization at species altitude.** `in @X` is a **sheaf-inclusion
morphism**. It opens a chart on the sheaf @X. The shard being minted
lives as a section over the union of opened charts. Cross-shard
composition requires explicit chart overlap declaration. No explicit
`in @X`, no valid overlap → no valid section → no valid shard.

The invariant is compiler-load-bearing: the compiler MUST refuse to
crystallize a shard whose body invokes symbols whose family-roots are
not declared in the import block. This species names the fracture that
witnesses the invariant's failure at any specific position.

## §2. The anti-pattern: geometric JavaScript

Alex 2026-07-23 verbatim anti-pattern coinage: **geometric JavaScript**
= implicit sheaf gluing. The failure mode:

- Any shard can implicitly reach any other shard through prototype-
  chain-substrate semantics (no explicit boundary declaration).
- Composition is unauditable: readers cannot mechanically determine
  which shards a given shard depends on.
- The compiler cannot verify sheaf-inclusion invariants; instead it
  falls back to runtime resolution with all the fragility that
  entails.

Mirror's substrate MUST NOT be this. The `in` keyword IS the substrate
declaration of the composition portal (the **inport**). A shard that
invokes symbols from an unimported family-root is substrate-dishonest.
This species IS the mechanically-decidable detector + discharge for
the substrate-dishonest state.

## §3. The mechanically decidable predicate

For each invoked symbol `s` in shard `S`:

```
inport_witnessed(S, s) :=
    s appears byte-visibly in body(S) at offset > import_block_end(S)
    ∧ family_root(s) ∉ imported_family_roots(S)
```

Where:

- `body(S)` = the shard's byte content (from `@io/fs.read(S.path)`).
- `import_block_end(S)` = the offset immediately after the last
  `in @X` statement at the top of `body(S)` (deterministic parse under
  @meta discipline).
- `family_root(s)` = the `@X` prefix of `s`, extracted by byte-visible
  prefix operation at @nl altitude. Example:
  `family_root("@kintsugi/mend.mend") = "@kintsugi/mend"`.
- `imported_family_roots(S)` = `{ Y : "in @Y" ∈ import_block(S) }`
  (finite set derived from byte-visible parse).

**The predicate is grep-decidable** (see §5). Every operation is a
byte-visible read or a finite-set membership check.

### Detection pseudocode

```
detect(shard_position: ref) -> [inport_fracture]:
    bytes = @io/fs.read(shard_position.path)
    import_block = parse_import_block(bytes)
    declared = { family_root(entry) : entry in import_block }
    invoked  = extract_invoked_symbols(bytes, after=import_block.end)
    records = []
    for sym in invoked:
        root = family_root(sym)
        if root not in declared:
            suggested = compose_in_statement(root)
            record = inport_fracture {
                shard_position:         shard_position,
                invoked_symbol:         sym,
                missing_family_root:    root,
                suggested_in_statement: suggested,
            }
            if inport_fracture_witnessed(p, record) == Pass:
                records.append(record)
    return records
```

## §4. The autopoietic discharge pipeline

Six-step loop composing over Mara #152 (autopoietic-loop
formalization) + Mara dd1d1d5 (@kintsugi/mend coboundary morphism):

| Step | Action | Composes over |
|------|--------|----------------|
| 1. **detect**       | `@kintsugi/roomba` walks shard bodies; emits `inport_fracture` records per grep-compare | `@io/fs.read` + `@nl` grep |
| 2. **propose**      | `@fate.roll` dispatches candidate `in @<family_root>` additions (singleton in the grep-decidable case) | `@fate.roll(candidates, restriction)` |
| 3. **tournament**   | Fate ranks candidates by @fractal/shard.through-restriction fit; winner emerges | `@fate` tournament shape |
| 4. **crystallize**  | `@fractal/shard.materialize` produces a shard carrying the inserted `in` statement as its content-addressed payload | `@fractal/shard.materialize` (Mara 2026-07-23) |
| 5. **mend**         | `@kintsugi/mend.mend` composes over `@io/fs.mutate_at` to insert the `in` statement at import-block top | `@kintsugi/mend.mend` (Mara 2026-07-23) + `@io/fs.mutate_at` |
| 6. **verify**       | re-run detect; the specific fracture no longer fires; fixed-point at that position | `inport_discharge_restores_sheaf_inclusion` bilateral |

### Resolution pseudocode

```
discharge(fracture: inport_fracture, correction: ref) -> verdict:
    # Pre-guard: fracture still witnesses
    if inport_fracture_witnessed(p, fracture) != Pass:
        return failure("fracture no longer witnesses; skip")

    # Steps 3-4: crystallize correction as a @fractal/shard
    mend_state = {
        fracture_position: fracture.shard_position,
        target:            @kintsugi/fracture/inport,
        through:           correction,
    }
    produced_shard = @kintsugi/mend.mend(p, mend_state)
    if produced_shard.status != success:
        return failure("mend refused; fracture preserved")

    # Step 5: byte-splice at import-block top
    bytes = @io/fs.read(fracture.shard_position.path)
    position = import_block_top_position(bytes)
    mutate = @io/fs.mutate_at(
        fracture.shard_position.path,
        position,
        fracture.suggested_in_statement,
    )
    if mutate.status != success:
        return failure("mutate_at refused; file preserved")

    # Step 6: post-verify sheaf-inclusion restored
    if inport_discharge_restores_sheaf_inclusion(p, fracture) != Pass:
        return failure("discharge did not restore invariant")

    return pass
```

## §5. Rice-safety: mechanical decidability proof

The fracture predicate is **grep-decidable**. Specifically:

1. **Body extraction** is a byte-visible read: `@io/fs.read` returns
   the shard's finite byte sequence. Deterministic. Total.

2. **Import-block parse** is bounded by @meta grammar: the import
   block is the maximal prefix of `body(S)` matching the regular
   grammar `("in" WS "@" family_root NL)*`. Parse is finite-depth,
   deterministic, terminating.

3. **Invoked-symbol extraction** greps for `@X/Y` byte-patterns
   (identifier + `.` + identifier structure) in the body bytes after
   `import_block_end`. Regular expression matching against a finite
   byte sequence; O(n) time in body length.

4. **Family-root extraction** is a byte-visible prefix operation:
   given `s = "@X/Y/Z.op"`, `family_root(s) = "@X/Y/Z"` (strip the
   final `.<op>` component). Deterministic prefix.

5. **Set-membership check** is finite: `imported_family_roots(S)` is
   a finite set (bounded by shard size); `∈` is O(|set|) time.

No dynamic dispatch. No aliasing analysis. No program semantics
inspection. Whole-tick Rice-safe by construction.

**Halting.** Every operation terminates in time polynomial in shard
size. `detect` returns a finite list. `discharge` fires exactly one
`@io/fs.mutate_at` per invocation and terminates on the post-verify
step.

## §6. Composition graph

Detects → proposes → crystallizes → mends → verifies. The composition
edges spelled out for Reed's rust/ altitude discharge work:

- **@kintsugi/roomba detects.** The roomba's `bump` action is
  extended by ONE composition edge (Reed post-mint tick in
  `shards/kintsugi/roomba.mirror`): on walk-step where the walker
  enters a shard body, dispatch `@kintsugi/fracture/inport.detect(p,
  shard_position)`. The fracture_species discriminator's open enum
  gains `inport` as a valid discriminator value.

- **@fate proposes.** For each detected `inport_fracture`, the
  `propose_correction` action dispatches `@fate.roll(candidates,
  restriction)` with `candidates = [fracture.suggested_in_statement]`
  and `restriction = fracture.shard_position`. In the grep-decidable
  case, this is deterministic-identity selection.

- **@fractal/shard crystallizes.** The `@kintsugi/mend.mend` internal
  composition invokes `@fractal/shard.materialize(p,
  @kintsugi/fracture/inport, correction)` which produces a
  `shard_state` carrying the inserted `in` statement as
  content-addressed payload. The shard's `address` is the content-
  address of the correction; `target` is `@kintsugi/fracture/inport`;
  `through` is the correction ref.

- **@kintsugi/mend fills.** `@kintsugi/mend.mend` fires with
  `mend_state = { fracture_position, target: @kintsugi/fracture/
  inport, through: correction }`. Composes over `@io/fs.mutate_at` at
  `import_block_top_position(bytes)` with the fracture's
  `suggested_in_statement` as replacement bytes.

- **@epistemologic/pact/bilateral discharges verdict.** The three
  bilateral predicates (`inport_fracture_witnessed`,
  `inport_correction_is_valid_in_statement`,
  `inport_discharge_restores_sheaf_inclusion`) dispatch through
  `@epistemologic/pact/bilateral.discharge` at apply_h::act altitude
  via sentinel-check arms (Reed post-mint under
  `[substrate-floor:@io-boundary]` + Seam sign-off).

### apply_h::act sentinel arms (Reed post-mint territory)

Three sentinel-check arms in `bootstrap/src/apply_h.rs`, ONE per
bilateral, ALL under `[substrate-floor:@io-boundary]` + Seam sign-off:

```rust
if action == "@kintsugi/fracture/inport.inport_fracture_witnessed"
    { arg.oid.contains("inport=fracture-witnessed") ? Pass : Fail }
if action == "@kintsugi/fracture/inport.inport_correction_is_valid_in_statement"
    { arg.oid.contains("inport=correction-valid-in-statement") ? Pass : Fail }
if action == "@kintsugi/fracture/inport.inport_discharge_restores_sheaf_inclusion"
    { arg.oid.contains("inport=discharge-restores-sheaf-inclusion") ? Pass : Fail }
```

Sentinel-check ONLY. No domain logic in Rust. The domain data lives in
the shard body.

## §7. Empirical proof of autopoietic loop closure

This species is the **empirical discharge** of Reed forward-promise
tasks #159 (wire six-step loop through apply_h::act) + #160 (empirical
one-fracture autopoietic round-trip demonstration) against ONE well-
defined fracture class.

**Why this species is the empirical proof:**

1. The species DECLARES the six-step loop's composition graph
   explicitly (see §4).
2. The predicate is grep-decidable (see §5) — the loop's convergence
   is mechanically verifiable.
3. The composition composes over TWO just-landed Mara mints (@kintsugi/
   mend + @fractal/shard) plus the prior autopoietic-loop
   formalization (Mara #152) — the same-day dependencies close the
   loop.
4. Reed's empirical discharge (`mirror kintsugi <shard>` +
   `mirror kintsugi --roomba <shard>`) produces observable output
   that Alex can reproduce verbatim (per Alex 2026-07-23 LRM
   directive).

**Fixed-point termination.** For a shard `S` with `k` inport
fractures, the loop terminates in exactly `k` iterations: each
`discharge` reduces the fracture count by one (post-verify guarantees
the specific fracture no longer detects), and no `discharge`
introduces a new fracture (the mend touches ONLY the import block
and inserts ONE `in` statement; the shard body is preserved by
construction). See math foundation §4 for the coboundary-morphism
proof.

## §8. Cascade forward-promises

### CLI wire (Reed post-mint territory)

- `mirror kintsugi <shard>` — dispatches `detect` action; emits
  `inport_fracture` records as questions per Alex 2026-07-23 verbatim
  ("produces the errors as questions"). @gestalt rendering: one
  question per fracture; the question surface names the missing
  family-root + suggested `in` statement.

- `mirror kintsugi --roomba <shard>` — dispatches `propose_correction`
  + `discharge` in sequence; the roomba walks + fills fractures until
  fixed-point per Alex 2026-07-23 verbatim ("fixes them and afterwards
  there are no more fractures").

**Last Responsible Moment.** Per Alex 2026-07-23 LRM naming, this arc
is the Last Responsible Moment for the `mirror kintsugi` CLI design
surface and @gestalt rendering shape. This spec sets the semantics
Reed will discharge into observable CLI behavior. The CLI surface
crystallizes here.

### Empirical discharge (Reed post-mint territory)

1. Wire six-step loop through `apply_h::act` sentinel-check arms per
   §6 (three arms; `[substrate-floor:@io-boundary]` + Seam sign-off).
2. Add composition edge in `shards/kintsugi/roomba.mirror` extending
   the walker's bump-pulse to dispatch `@kintsugi/fracture/
   inport.detect` (fracture_species discriminator open enum
   extension).
3. Empirical one-fracture round-trip: pick ONE real shard with a
   missing `in @X`; run the loop; verify fixed-point.
4. Reproduce output verbatim in the Reed handoff.

### Downstream (Mara post-mint territory)

- @kintsugi/roomba shard-decl update: forward-promise composition
  edge extension (Reed can also do this; Mara stands ready if @kintsugi/
  roomba shape needs refinement).
- @gestalt rendering shape declaration if the current @gestalt
  surface doesn't natively carry fracture-as-question semantics.

## §9. Halt conditions checked

Per the arc's halt conditions:

- **Rice-safety.** ✅ Confirmed grep-decidable at §5. No aliasing, no
  dynamic dispatch, no undecidability.
- **Existing @kintsugi/fracture/\* coverage.** ✅ Grep-audit
  confirmed no prior species covers this pattern. 15 landed siblings
  (angle_to_paren, bilateral_arm_redundant, cold_compile_within_tolerance,
  dark_count_monotone, docblock_extractive, docblock_incoherent,
  docblock_ungrounded, gate, keyword, operator_match, parent_cycle,
  partials_align, relocate, restart_storm, symbol_lift) — none touch
  sheaf-inclusion at the import-block altitude.
- **@roomba detection surface fit.** ✅ Roomba's `bump(position:
  walk_position) -> spectral_tension` and `bump(fracture: fracture) ->
  imperfect<kintsugi_dispatch, ...>` compose naturally with grep-
  decidable predicates; the walker already dispatches per
  fracture_species discriminator (Mara #77).

No unilateral @roomba patches. No dual-mint. Grep-decidable throughout.
