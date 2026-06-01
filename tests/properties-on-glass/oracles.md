# Properties-on-Glass Fixture Oracles

*Reed, 2026-06-01. Companion to [[../../docs/specs/properties-on-glass]].*

Fifteen fixtures locking down parse/resolve shape for the per-glass
property discipline. Each fixture is small and focused; this file
records the expected outcome for each so the implementing agent
(probably Taut for G-1—G-4 and Mara for G-5—G-7 per the
[[../../docs/specs/properties-on-glass#10]] followup ticks) has an
oracle to test against.

Convention from `stage_play/AGENTS.md`: *"if the oracle and the
implementation disagree, the oracle is right."*

---

## Fixture 01 — `01-bare-glass.mirror`

A glass with no properties declared. Empty qualifier set.

- **Parse:** ✅ OK
- **Tokenize:** ✅ OK (no `---` separator)
- **Resolve:** ✅ OK — qualifier set = `{}`
- **Settlement:** N/A (nothing to settle)
- **Blocks:** G-2 (parser for `glass to @X { ... }` body)

## Fixture 02 — `02-glass-with-halts.mirror`

A glass with the canonical `halts` property bound.

- **Parse:** ✅ OK
- **Resolve:** ✅ OK — qualifier set = `{halts}`; `halts` resolves to
  `@epistemologic/property/halts`
- **Settlement:** would dispatch to halts's per-glass implementation
  (not yet implementable)
- **Blocks:** G-2 + chain-resolver

## Fixture 03 — `03-glass-with-pure.mirror`

The elevated `pure(type)` property.

- **Parse:** ✅ OK
- **Resolve pre-G-5:** ❌ FAIL with `literal(pure):
  @epistemologic/property/pure does not resolve` (the chain primitive
  hasn't landed yet)
- **Resolve post-G-5:** ✅ OK — qualifier set = `{pure}`
- **The failure IS the oracle:** this test must fail TODAY and pass
  AFTER G-5 lands. It's the canary for the chain extension.
- **Blocks:** G-2 + G-5

## Fixture 04 — `04-invented-property.mirror`

Invented property name `foobar`.

- **Parse:** ✅ OK
- **Resolve:** ❌ FAIL with `literal(foobar):
  @epistemologic/property/foobar does not resolve`
- **The grounding discipline must hold:** property names must exist
  in the chain (or as custom properties below `---`); inventing names
  is a compile-time error.
- **Blocks:** G-2 + literal-property-check at the glass altitude

## Fixture 05 — `05-three-hyphen-separator.mirror`

The canonical `---` separator with empty below-`---` region.

- **Tokenize:** ✅ OK — one `settlement_separator` form at the
  documented line
- **Parse:** ✅ OK — above-`---` parses as the contract; below-`---` is
  empty (file not yet settled)
- **Resolve:** ✅ OK
- **Blocks:** G-1 (tokenizer support for `---`)

## Fixture 06 — `06-malformed-separator.mirror`

Two hyphens (`--`) where the substrate expects three.

- **Tokenize:** ❌ FAIL with `unrecognized form `--`; did you mean
  `---` (settlement_separator)?`
- **Locks down:** EXACTLY three hyphens, not more, not fewer.
- **Blocks:** G-1

## Fixture 07 — `07-two-separators.mirror`

Two valid `---` separators in one file.

- **Tokenize first `---`:** ✅ OK
- **Tokenize second `---`:** ❌ FAIL with `multiple
  settlement_separators not allowed; the contract/back-projection
  split is binary`
- **Locks down:** AT MOST ONE per file.
- **Blocks:** G-1

## Fixture 08 — `08-custom-property-below.mirror`

Custom property `tournament_safe` defined below `---`, referenced at
a glass above.

- **Tokenize:** ✅ OK
- **Parse:** ✅ OK — above-`---` parses as contract;
  below-`---` parses as custom property definition
- **Resolve:** ✅ OK — qualifier set = `{halts, tournament_safe}`;
  `tournament_safe` resolves to the conjunction defined below
- **Locks down:** bi-directional `---` (§4.6); the conjunction-only
  syntax for custom properties (§4.6.1)
- **Blocks:** G-3 (parser for custom property definitions)

## Fixture 09 — `09-custom-property-collision.mirror`

Custom property named `halts` collides with chain primitive.

- **Parse:** ✅ OK
- **Resolve:** ❌ FAIL with `property `halts` shadows
  @epistemologic/property/halts; custom properties cannot shadow
  chain primitives`
- **Locks down:** §4.6.3 — the canonical chain stays canonical.
- **Blocks:** G-3 + chain-collision-check

## Fixture 10 — `10-imperative-body-refused.mirror`

Custom property body is imperative (statements, conditional return).

- **Parse:** ❌ FAIL at the property body with `custom property body
  must be a conjunction of chain primitives; imperative bodies are
  not admitted`
- **Locks down:** §4.6.1 — only conjunctions, never code.
- **Blocks:** G-3

## Fixture 11 — `11-cascade-parent-defines/`

Two-file cascade: parent defines `tournament_safe` below `---`; child
references it through `/` inheritance.

- **parent.mirror parse + resolve:** ✅ OK — `tournament_safe`
  registered at `@test/11/parent`
- **child.mirror parse + resolve:** ✅ OK — `tournament_safe` resolves
  through cascade to parent's `---` definition; qualifier set =
  `{halts, tournament_safe}`
- **Locks down:** §4.6.2 — cascade through `/` inheritance.
- **Blocks:** G-4 (resolver for cascade)

## Fixture 12 — `12-cascade-grandparent-defines/`

Three-file cascade with empty intermediate.

- **grandparent.mirror:** ✅ — defines `tournament_safe`
- **parent.mirror:** ✅ — empty grammar; re-exports cascade-implicitly
- **grandchild.mirror:** ✅ — `tournament_safe` resolves through two
  cascade hops; qualifier set = `{halts, tournament_safe}`
- **Locks down:** transitive cascade across empty intermediates.
- **Blocks:** G-4

## Fixture 13 — `13-cascade-no-shadow/`

Child tries to redefine inherited `tournament_safe`.

- **parent.mirror parse + resolve:** ✅ OK
- **child.mirror parse:** ✅ OK
- **child.mirror resolve:** ❌ FAIL with `property `tournament_safe`
  already defined at @test/13/p (cascaded); cascade is additive, not
  overriding; cannot shadow`
- **Locks down:** §4.6.2 — cascade is additive, never overriding.
- **Blocks:** G-4 + cascade-shadow-check

## Fixture 14 — `14-multiple-glasses.mirror`

One grammar, two glasses with different qualifier sets.

- **Parse:** ✅ OK
- **Resolve:** ✅ OK — TWO glass entries:
  - `to @io` → `{halts, io_safety}`
  - `to @code/rust` → `{halts, pure}`
- **Locks down:** §11.1 resolution — per-glass scoping, not
  per-grammar.
- **Blocks:** G-2 (multi-glass per grammar)

## Fixture 15 — `15-back-projection-format.mirror`

The canonical settled-file format with header + Contract OID +
implementation block + `\` hole.

- **Tokenize:** ✅ OK
- **Parse above `---`:** ✅ OK (contract)
- **Parse below `---`:** ✅ OK — settlement header is a comment block;
  `implementation halts for @code/rust glass { \ }` parses as a
  settled-implementation form with a Fate-fillable hole
- **Locks down:** §4.2 — back-projection format (header + Contract
  OID + implementation blocks); §4.3 — the Contract OID enables
  staleness detection.
- **Blocks:** G-7 (back-projection writer) + G-9 (staleness verdict)

---

## Tick coverage

| Tick | Fixtures that test it |
|---|---|
| G-1 (`---` tokenizer) | 05, 06, 07, 08, 11, 12, 13, 15 |
| G-2 (`glass to @X { property P }` parser) | 01, 02, 04, 14 |
| G-3 (custom property parser) | 08, 09, 10 |
| G-4 (cascade resolver) | 11, 12, 13 |
| G-5 (`@epistemologic/property/pure` chain landing) | 03 |
| G-6 (liquid inference) | (no fixture; needs settlement loop) |
| G-7 (back-projection writer) | 15 |
| G-8 (`Pure<G: Glass>` Rust trait codegen) | (Rust-side; outside `.mirror` fixtures) |
| G-9 (staleness verdict) | 15 (Contract OID) |
| G-10 (re-settlement trigger) | (needs runtime infrastructure) |

## What these fixtures do NOT cover

The ENCODED-IN-SPEC tests from §10 of
[[../../docs/specs/properties-on-glass]] still need to be acceptance
criteria in the spec when settlement infrastructure lands:

- Settlement loop convergence
- Back-projection writes valid implementations (Fate-filled `\`)
- Cross-language verification at the seam
- `Pure<G: RustGlass>` impl is minted automatically
- Reproducibility across runs (blocked by §C4 seed-pinning)

These cannot be written as `.mirror` fixtures because they require
running the settlement loop. They go in the spec as acceptance
criteria, and the implementing agent encodes them as integration
tests when the loop exists.

---

*The fixtures lock the shape.*
*The oracle is the spec.*
*If the implementation disagrees with the oracle, the oracle wins.*
