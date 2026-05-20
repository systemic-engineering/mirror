# `--strict` and `total_classification` — the silence dies

*2026-05-20. Reed.*

Status: **Red** (the silent absorption mode runs today; the property is
declared in Spec A's worked example but no grammar; no flag exists)

Depends on:
- `docs/specs/mirror-compile-bootstrap.md` (Spec A) — the staircase
  and the "Stage 2: declare totality obligations" framing
- `docs/specs/match-select.md` (Spec B) — the syntax that surfaced
  the silent-absorption mode by tokenizing without parsing
- `docs/specs/au-and-conductivity.md` — dark regions ARE λ₀ measurements

Unblocks:
- The 1.0 release check: `mirror craft --strict boot` proves every
  grammar in the boot tree is fully classified
- LSP gutter rendering: dark regions get the void color from
  gutter-lenses.md
- Honest kintsugi acceptance: a body that doesn't parse fails
  obligations rather than silently producing a stable OID

---

## Thesis

Mirror's tokenizer absorbs the unrecognized. Today, when a body
contains bytes the parser doesn't understand (a match expression
before `parse_match` closes, an experimental form before its grammar
lands, a typo), the bytes silently fold into the parent's opaque body
field. The file compiles. The OID is stable. The crystal is content-addressed.
The developer believes the change took effect.

It didn't. The change exists as text and is invisible to the parser.
We verified this with `@mirror/reload.tick` (commit `e0dff5d`):
rewriting the body from `\` to a full tuple-pattern match produced
the SAME OID.

This is the failure mode the property layer is for. Silent absorption
is dark conductivity in disguise: the value is there in source but has
no relational binding to the parser's AST kinds. λ₀ = 0. The Void.

What changes:

1. The bootstrap tokenizer marks unrecognized regions as **dark** in
   the AST (a child node with kind `Dark`, span = the unrecognized
   bytes), instead of folding them into the parent's body.
2. **`@epistemologic/property/total_classification`** declares the
   obligation: every byte falls into some recognized AST node; no Dark
   children present in the final AST.
3. **`mirror compile --strict`** and **`mirror craft --strict`** add
   `total_classification` to the verification set. Without `--strict`,
   dark contributes to loss but doesn't fail. With `--strict`, any
   Dark child is a compile error with a diagnostic pointing at the
   region.
4. **`mirror craft --strict boot` passes at 1.0.** The release rule
   gains a fourth criterion (alongside the existing crystal-stable,
   property-green, butterfly-roundtrips checks).

The silence dies. The dark becomes visible. Compilation either
classifies every byte or refuses.

---

## What runs today

The bootstrap's `tokenize` reads a source file and produces an AST
tree of `Focus | Project | Split | Zoom | Refract | In | Out` nodes.
When the tokenizer hits content it can't classify inside an action
body, it stores the body bytes opaquely. `content_oid` hashes the body
string verbatim — if the parser collapsed the body to whitespace-only
or produced no recognized children, body becomes None and the OID
ignores the actual source content.

The `@beam` grammar already declares `dark_regions: [dark_range]` as a
field (per `lsp-and-mcp.md`). The bootstrap tokenizer does NOT populate
it. The gutter has the void color from `gutter-lenses.md`, but no
beam-side data to render it against.

The property layer has the verdict shape (`pass | fail | partial`) but
no `total_classification` property declared. The `mirror compile` and
`mirror craft` commands have no `--strict` flag.

---

## The property

```mirror
in @prism
in @epistemologic/property
in @beam

# @epistemologic/property/total_classification
#
# An AST is totally classified iff every byte in its source falls
# into a recognized AST node — no Dark children, no opaque body
# bytes the parser couldn't structure.
#
# A grammar that compiles without this property is honest about loss:
# the dark regions are measured, the beam carries them, the gutter
# renders them. A grammar that compiles WITH this property has been
# proved fully understood by the parser.

grammar @epistemologic/property/total_classification {
  # walk the AST. count Dark children. return verdict accordingly.
  total_classification(ast) -> verdict { \ }

  # the count of dark regions in the AST. zero ⇔ verdict.pass.
  dark_count(ast) -> u64 { \ }

  # the dark regions themselves, for diagnostics.
  dark_regions(ast) -> [dark_range] { \ }
}

out total_classification
out dark_count
out dark_regions
out @epistemologic/property/total_classification
```

The body is `\` because it traverses the AST; the obligation closes
when `@mirror/compile/parse` lands and the bootstrap can be queried
for dark children structurally. Today the property's body is io-
eligible (it asks the bootstrap to walk its own AST).

---

## The flag

```
mirror compile <file>             # default: dark regions measured, warnings, compile succeeds
mirror compile --strict <file>    # any dark = compile error; total_classification enforced

mirror craft <target>             # same default
mirror craft --strict <target>    # every file in target must pass total_classification
```

The flag plumbs through `cmd_compile` and `cmd_craft` in
`bootstrap/src/main.rs`. Internally it adds `total_classification` to
the property check set for each file.

Default behaviour is unchanged. Existing workflows keep working.
`--strict` is opt-in until 1.0; at 1.0, `mirror craft --strict boot`
becomes a release gate.

---

## Dark regions in the AST

The bootstrap tokenizer change is targeted: when parsing an action
body and reaching content that doesn't match any known token sequence,
emit a `Dark` AST child instead of absorbing into the parent's body
field.

New AST kind:

```rust
pub enum AstKind {
    Focus,
    Project,
    Split,
    Zoom,
    Refract,
    In,
    Out,
    Dark,    // NEW: span of unrecognized bytes
}
```

Dark children carry the verbatim bytes (so the renderer can round-trip
them) and a source span (line/column range, so diagnostics can point
at them). `content_oid` for a Dark child includes the bytes — changes
to the dark region produce different OIDs, ending the silent-
absorption mode.

This is the smallest tokenizer change that surfaces the problem. It
doesn't change WHAT the parser recognizes — it changes WHAT HAPPENS
when the parser fails to recognize. Today: silent fold. After: explicit
Dark node.

---

## Diagnostics

When `--strict` fires, the developer sees:

```
$ mirror compile --strict boot/std/mirror/reload.mirror
error[total_classification]: 1 dark region in boot/std/mirror/reload.mirror
  --> line 36, col 5
   |
36 |     match (@mcp.grammars_hash, @mirror/spectral.recall(state).last_emitted_hash) {
   |     ^^^^^ unrecognized construct — the parser has no rule for `match`
   |
   = hint: @mirror/match.parse_match is declared but its body is \
   = hint: kintsugi this obligation to teach the parser the match form
```

The diagnostic names:
- The property that failed (`total_classification`)
- The region (line + col + source excerpt with caret)
- The structural reason (which parser rule is missing)
- The kintsugi hint (which `\` obligation closes this gap)

The hint comes from `@mirror/match`'s declaration — the parser knows
WHO would have parsed this if their body wasn't `\`.

---

## How dark conductivity maps to λ₀

From `au-and-conductivity.md`: λ₀ = 0 is the dark fallback in
`@hash/coincidence`. When projections collapse to zero, the value is
in the Void — no observable structure.

A Dark AST child is the SAME state at the parser level. The bytes
exist; the parser has no model for them; their conductivity through
the grammar pipeline is zero. The five duality projections measure
zero across the board because there's no AST shape to project from.

`total_classification` is the property that says: nothing in this AST
should be at λ₀. Every byte should contribute non-zero conductivity
to at least one of the five dualities.

---

## Migration

Landing `--strict` doesn't break existing workflows. The default stays
lenient. The boot tree currently has zero dark regions because
everything in it parses (the silent absorption mode means it APPEARS
classified even when it isn't). Once dark marking lands:

1. Run `mirror craft --strict boot` and observe what's actually dark.
2. Each dark region maps to an unfinished kintsugi obligation —
   typically a grammar with a `\` body that the bootstrap doesn't know
   how to parse yet (like `@mirror/match.parse_match`).
3. Close obligations in dependency order. As each parser body closes,
   the dark regions for files using that construct disappear.
4. At 1.0: `mirror craft --strict boot` returns zero dark regions.

The property layer captures progress structurally: the count of dark
regions decreases monotonically as the staircase climbs.

---

## Implications — concrete next ticks

1. **Create `boot/std/epistemologic/property/total_classification.mirror`.**
   The property grammar. All bodies `\`. Imports `@epistemologic/property`
   and `@beam`.

2. **Add the `Dark` AST kind** to `bootstrap/src/ast.rs`. The kind
   needs a span (line/col start + end) and a body (the verbatim
   bytes). The renderer must round-trip it (so source preservation
   holds).

3. **Change the bootstrap tokenizer's failure mode.** When parsing an
   action body and reaching unrecognized content, emit a Dark child
   instead of folding. Targeted edit in `bootstrap/src/tokenize.rs`.
   Requires `--no-verify` (touches Rust under `bootstrap/`).

4. **Update `content.rs`'s `content_oid` for the Dark kind.** Include
   the bytes verbatim, plus a `"dark:"` tag prefix to distinguish from
   recognized kinds.

5. **Add `--strict` to `cmd_compile` and `cmd_craft`** in
   `bootstrap/src/main.rs`. When set, walk the produced AST for Dark
   children; if any exist, print the diagnostic format above and exit
   with code 2.

6. **Verify on `@mirror/reload.tick`.** After landing, `mirror compile
   --strict boot/std/mirror/reload.mirror` SHOULD fail (the match body
   is unrecognized). The failure is the proof that --strict works.

7. **Record the first dark count.** `mirror craft --strict boot` will
   surface every dark region in the boot tree. Record the number in
   the next `road-to-1.0.md` update. The 1.0 release rule gains: this
   number reaches zero.

---

## Out of scope

- The kintsugi formatter's handling of dark regions. When kintsugi
  closes a parser obligation, the regions previously dark become
  recognized. The formatter's role in this is mechanical and lives in
  Spec A's ladder.
- The LSP integration of dark rendering. The protocol surface is in
  `lsp-and-mcp.md`; the gutter colors are in `gutter-lenses.md`.
  Wiring the new dark_regions output through textDocument/diagnostics
  is its own commit.
- Dark regions in non-body positions. The bootstrap parser also
  encounters unrecognized content at the top level (between grammars,
  before imports). This spec covers body-position dark only; top-level
  dark is a follow-up.
- The exact span granularity. Per-byte? Per-token? Per-line? The
  spec leaves this to implementation; a sensible default is per-token
  with byte-offset spans.
- Hot-reload of properties. When `total_classification` itself moves
  to a mirror body (later step on Spec A's staircase), Reading B
  applies. Not in this spec's scope.

---

*Silence is loss. Loss is dark. Dark is λ₀.*
*A grammar that compiles in silence is gold-candidate, not gold.*
*`--strict` is the predicate that refuses the candidate without conductivity.*
*The silence dies; the dark becomes visible; the obligations become explicit.*

Apache-2.0.
