# Walker Contract — `walk_combinator`

> What the parser actually does, variant by variant. Until F-1, the walker
> was structural-self on every non-`Choice` arm: it ignored source bytes
> and returned its input unchanged. The OID equality FP1 claimed therefore
> held tautologically — it could not have observed a parse failure. This
> spec defines what "the walker walks" means: bytes consumed, offsets
> advanced, success-vs-Dark conditions, and the OID-preservation
> contract.

## Shape

```
fn walk_combinator_at(
    c: &Combinator,
    source: &[u8],
    offset: usize,
    depth: usize,
) -> WalkOut

struct WalkOut {
    /// The structural witness. On success, structurally equal to `c`
    /// (Choice arms may be pruned by source presence, see below).
    /// On parse failure, `Combinator::DarkFallback` at the failure site.
    witness: Combinator,
    /// New offset. On success, the byte position just past the
    /// consumed span. On failure, equal to the offset where the parse
    /// got stuck. The caller decides whether to back-track or surface.
    offset: usize,
    /// `true` iff the walk consumed the bytes it claimed to. On
    /// failure, `witness == DarkFallback` and `success == false`.
    success: bool,
}
```

`apply_h(combinator, (source, 0))` returns `walk.witness`. The offset is
discarded at the top level; downstream callers (e.g., `Seq`) thread it.

## OID-preservation invariant (FP1)

For `c = prism_seed()` and `source = read("std/mirror/grammar.mirror")`,
`walk(c, source, 0).witness` has `combinator_tree_oid` equal to
`combinator_tree_oid(c)`. This is the load-bearing equation. It holds
iff:

1. Every leaf variant in the seed walks to itself on the prefix of
   `source` at the offset reached.
2. Every `Choice` walk keeps the arms the seed declared, in the order
   the seed declared them. (Pruning, when it happens, drops arms — but
   the seed's Choices never prune on `grammar.mirror` because the
   seed's structural arms each succeed on every byte they cover.)
3. The walk never hits `DarkFallback`. Any failed sub-walk would
   substitute `DarkFallback` into the witness, breaking OID equality.

The FP1 inequality test (`apply_h(seed, random_bytes) != seed`) is the
adversarial side: random bytes will fail at the first `Literal("in")`
or at the first `Charset(NameChar)` after a non-`@` byte, and the
witness will carry `DarkFallback` somewhere — distinct OID.

## Variants

### `Literal(bytes)`

- **In:** `source[offset..]` and the literal `bytes`.
- **Out (success):** `witness = Literal(bytes.clone())`,
  `offset' = offset + bytes.len()`, `success = true`.
- **Out (failure):** `source[offset..]` does not start with `bytes` (or
  the remaining input is too short). `witness = DarkFallback`,
  `offset' = offset`, `success = false`.
- **Empty literal:** zero-length `bytes` always succeeds with
  `offset' = offset`.

### `Charset(kind)`

- **In:** `source[offset]` and the byte-class predicate from `kind`.
- **Out (success):** the byte satisfies the predicate. `witness =
  Charset(kind)`, `offset' = offset + 1`, `success = true`.
- **Out (failure):** offset is at end of input, OR the byte fails the
  predicate. `witness = DarkFallback`, `offset' = offset`,
  `success = false`.

### `Seq(children)`

- **In:** sequence of child combinators, threaded offset.
- **Out (success):** walk each child in order; thread the advancing
  offset. Each child must succeed. `witness = Seq(walked children)`,
  `offset' = offset after final child`, `success = true`.
- **Out (failure):** any child fails. `witness = DarkFallback`,
  `offset' = offset where the failure occurred`, `success = false`.
  (Earlier successful children's consumption is discarded; the Seq is
  atomic.)
- **Empty Seq:** zero children always succeeds with `offset' = offset`
  (matches the spec's E13 normalization view of `Seq([]) == ε`).

### `Repeat { body, min, max }`

- **In:** body combinator, bounds, threaded offset.
- **Out (success):** walk `body` repeatedly, each iteration starting at
  the offset the prior iteration ended. Stop when:
  (a) `body` fails (no consumption), OR
  (b) iteration count reaches `max` (if `Some`), OR
  (c) `body` succeeds with zero consumption (would loop forever).
  Success requires iteration count >= `min`.
  `witness = Repeat { body: walked, min, max }`,
  `offset' = offset after final successful iteration`,
  `success = true`.
- **Out (failure):** iteration count < `min`. `witness = DarkFallback`,
  `offset' = offset where the parse failed mid-iteration`,
  `success = false`.
- **Loop guard:** the zero-consumption stop is non-negotiable. Without
  it, `Repeat { Literal(""), 0, None }` would loop forever.

### `Choice(arms)`

- **In:** alternatives, current offset.
- **Out (success):** try each arm at the same offset. First arm whose
  walk returns `success = true` wins. `witness = Choice(arms unchanged
  except any LiteralKind arms whose keyword does not occur whole-word
  in `source` are dropped — pure structural arms always kept)`,
  `offset' = winning arm's offset`, `success = true`.
  - **OID-stability note:** for `prism_seed()`'s Choices (no
    `LiteralKind` arms today), the kept-arms list is the input list
    unchanged, so the OID is preserved.
- **Out (failure):** every arm fails. `witness = DarkFallback`,
  `offset' = offset`, `success = false`.
- **Empty Choice:** zero arms always fails (per spec E14 it
  normalizes to `DarkFallback` upstream — the walker treats both the
  same).

### `LiteralKind { keyword, kind }`

- **In:** keyword bytes, `source[offset..]`.
- **Out (success):** `source[offset..]` starts with `keyword` AND the
  byte at `offset + keyword.len()` is a non-word byte (or end of
  input). `witness = LiteralKind { keyword, kind }`,
  `offset' = offset + keyword.len()`, `success = true`.
- **Out (failure):** keyword does not match whole-word. `witness =
  DarkFallback`, `offset' = offset`, `success = false`.
- **Whole-word boundary:** word bytes are ASCII alnum + `_` per
  `is_word_byte` (the F-3 unification). `/` and `@` are boundaries.

### `Capture { body, kind }`

- **In:** body combinator, kind tag, offset.
- **Out (success):** walk `body`; on body success, wrap the witness
  as `Capture { body: walked body, kind }`. `offset' = body's offset`,
  `success = true`.
- **Out (failure):** body failed. `witness = DarkFallback`,
  `offset' = body's failure offset`, `success = false`.
- **Note:** Capture carries the consumed span implicitly via offset
  delta; the witness OID matches the seed's Capture OID iff the body
  walks to itself.

### `BraceBlock(body)`

- **In:** body combinator, offset.
- **Out (success):** `source[offset] == b'{'`. Scan forward,
  maintaining a brace depth counter (starts at 1 after the opening
  `{`). Track each `{` (depth++) and `}` (depth--). On `depth == 0`,
  the closing `}` position is found. The inner bytes are
  `source[offset+1..close_pos]`. Walk `body` over those inner bytes
  starting at offset 0. On body success that consumes all inner bytes
  (offset reaches inner length, allowing trailing-whitespace tolerance
  via the `Repeat` over whitespace pattern the seed uses), the
  BraceBlock succeeds. `witness = BraceBlock(walked body)`,
  `offset' = close_pos + 1`, `success = true`.
- **Out (failure):**
  - `source[offset] != b'{'`: walking failed at depth 0.
  - Unbalanced braces (depth never returns to 0 before EOF).
  - Body walk over inner bytes fails or returns with unconsumed input.
  `witness = DarkFallback`, `offset' = offset on the open-brace path,
  or the position where balance was lost`, `success = false`.

### `ParenBlock(body)`

- Same shape as `BraceBlock` but for `(` and `)`. Used by the seed for
  `grammar @ref(tags)` and `prism @(arg)` forms.

### `Until { stop }`

- **In:** stop combinator (used as a *peek*; never consumed), offset.
- **Out (success):** scan forward byte-by-byte; at each position, peek
  the `stop` combinator. First position where `stop` succeeds is the
  end. `witness = Until { stop: walked stop }` (the stop combinator
  walked at the terminator position for OID preservation),
  `offset' = terminator position` (NOT past the terminator — stop is
  not consumed), `success = true`.
- **Out (failure):** `stop` never succeeds before EOF. The walker
  treats this as success with `offset' = source.len()` (Until is
  permissive — "consume to end of input" is a valid outcome for
  `until(newline)` on a file with no trailing newline). The witness
  is preserved structurally either way.
- **Note:** Until's stop combinator is walked structurally for OID
  preservation; the byte consumption check uses a peek at runtime.

### `Shift { grammar, body }`

- **In:** grammar reference (string), body combinator, offset.
- **Out (success):** walk `body` at `offset` to extract the body's
  span. Resolve `grammar` via a registry (Checkpoint C scope —
  currently the registry is empty and Shift falls back to walking
  `body` structurally). When the registry is populated, the
  target grammar's Combinator tree walks over the extracted body
  bytes. `witness = Shift { grammar, body: walked body }`,
  `offset' = body's offset`, `success = body's success`.
- **Out (failure):** body fails OR (when registry populated) target
  grammar fails on body bytes. `witness = DarkFallback`,
  `offset' = offset`, `success = false`.
- **Checkpoint A/B scope:** Shift walks structurally with no grammar
  resolution. Checkpoint C wires the registry and the recursive
  apply.

### `DarkFallback`

- **In:** offset.
- **Out (always):** the walker treats DarkFallback as the strict-
  classification sentinel — when present in the seed (as the last arm
  of a Choice), it scans forward through any remaining bytes and
  always succeeds. `witness = DarkFallback`, `offset' = source.len()`,
  `success = true`.
- **In structural seeds:** Only appears at the bottom of top-level
  `Choice`. `walk` on a bare `DarkFallback` is the catch-all.

### `MultiByteCharset(members)`

- **In:** sorted-deduped list of byte sequences, offset.
- **Out (success):** the first member that matches `source[offset..]`
  (members in stored order, which is sorted-lex). On match, `witness =
  MultiByteCharset(members.clone())`, `offset' = offset + member.len()`,
  `success = true`.
- **Out (failure):** no member matches. `witness = DarkFallback`,
  `offset' = offset`, `success = false`.
- **Empty set:** always fails (matches nothing).

### `IoBinding`, `MatchArm`, `SelectVariant`, `KeywordFormBody`

- These are surface-keyword placeholders for later ticks. The walker
  emits structural-self with `offset' = offset` and `success = true`.
  The seed does not contain any of these today — the meta-glass
  declarations `io_form`, `match_form`, `select_form` resolve to
  these placeholders via the grammar form. Until the next tick wires
  them, they are no-ops at parse time.

## Iterative Drop (Checkpoint D)

`Combinator` chains can nest arbitrarily through `Box<Combinator>`-
carrying variants: `Repeat`, `Capture`, `Until`, `Shift`, `BraceBlock`,
`ParenBlock`. A pathological 10,000-deep chain on the recursive
`Drop` implementation would overflow the thread stack.

The iterative `Drop` impl walks the tree iteratively, popping nodes
onto a worklist and dropping them one at a time. The traversal
matches the walker's depth-first shape but uses a heap-allocated
stack instead of the program stack.

## Termination & Bounds

- `walk_combinator_at` is bounded by `MAX_DEPTH = 1024`. On overflow,
  emit `DarkFallback` with `offset' = offset`, `success = false`. F-4
  preserves this bound.
- `Repeat`'s zero-consumption stop guards against infinite loops.
- Total byte consumption is bounded by `source.len()` for any
  successful walk. Per the spec's `Transport::Holonomy: Metric`
  argument, each consuming step decreases the residual `source.len()
  - offset` strictly, so the parser terminates.

## Cross-validation

The walker is cross-validated against the existing `tokenize.rs` for
every file in `boot/`. Divergence count must be 0. See Checkpoint E
in `bootstrap/F-1.md` (this tick's brief).
