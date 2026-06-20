# Adversarial Review: reed/ast-optics

**Reviewer:** Seam
**Date:** 2026-05-05
**Branch:** `reed/ast-optics` (4 commits, 4 files changed)
**Base:** prior state on `reed/ast-optics`
**Status:** 715 tests pass (with `--features shatter`), 698 without. 3 pre-existing failures (boot inventory count drift + holonomy baseline regression).

---

## Executive Summary

The branch adds five optic variants (Focus, Project, Split, Shift, Settle) to the mirror AST, teaches the parser to promote `name(arg) { body }` into optic AST nodes when the name matches, adds serde serialization behind a `shatter` feature flag, and defines a binary `.shatter` blob format (SHTR magic + version byte + bincode payload). The decomposition is clean. The TDD discipline is correct. The parser disambiguation strategy -- optic name + single arg + body = optic variant, everything else stays Call -- is reasonable and well-tested.

The primary concerns are: (1) the content-address scheme hashes Display output, which creates a fragile coupling between the hash and the formatter; (2) the `.shatter` blob format has no defense against oversized allocations from malicious payloads; (3) the optic variants enforce a mandatory argument, which means `settle { settle }` (no explicit argument) cannot be represented as an optic; and (4) there is no connection yet between the optic AST and the Dirac operator in `dirac.rs`, despite the doc comment in `ast.rs` line 97 claiming "composition of optics IS the Dirac operator."

Nothing is blocking. The architecture is sound for the current scope. The findings below are ordered by severity.

---

## WARN --- Should Fix

### W1. Content addressing via Display is fragile

`Ast::content_oid()` (ast.rs:265-267) hashes `format!("{}", self)`:

```rust
fn content_oid(&self) -> crate::Oid {
    crate::Oid::hash(format!("{}", self).as_bytes())
}
```

This means any change to whitespace, indentation, or formatting in the `emit` function silently changes every OID in the system. If someone adds a trailing newline, adjusts indentation width, or changes how empty bodies display, all existing `.shatter` blobs become address-orphans -- the content_oid of the deserialized AST will no longer match the OID that was stored.

The tests `bincode_content_oid_preserved` and `content_oid_matches_after_round_trip` verify that bincode round-trip preserves the OID. But they don't protect against the deeper risk: a formatting change in a future commit silently invalidating all stored blobs.

**Suggestion:** Hash a canonical structural form (e.g., the bincode bytes themselves, or a purpose-built canonical encoding) rather than the pretty-printed text. Alternatively, add an explicit test that pins the Display output of a known AST to a known string, so any formatting change is caught.

### W2. No deserialization size limit -- unbounded allocation risk

`deserialize_shatter` (shatter_blob.rs:69-77) passes `&bytes[5..]` directly to `bincode::deserialize`. Bincode 1.x reads length-prefixed Vec/String values and allocates accordingly. A crafted payload with a 4-byte length prefix claiming `u64::MAX` elements will cause `bincode` to attempt a multi-gigabyte allocation before failing.

In the current usage (reading from local `.shatter` files), this is low risk. But if `.shatter` blobs are ever received over the network or from untrusted sources, this becomes a denial-of-service vector.

**Suggestion:** Use `bincode::options().with_limit(MAX_BLOB_SIZE).deserialize()` to cap deserialization at a reasonable size (e.g., 16 MiB). This requires switching from the legacy `bincode::deserialize` API to the builder API, which also gives you explicit endianness control.

### W3. `AstOid` in `ast.rs` is dead code

`ast.rs` line 16-17 defines `domain_oid!(pub AstOid)`, but the `ContentAddressed for Ast` impl at line 264 uses `type Oid = crate::Oid`, not `AstOid`. The only consumer of `ast::AstOid` is the `domain_oid_full_coverage` test in `kernel.rs`, which exercises the macro, not the domain type.

Meanwhile, `parse.rs` line 59-62 defines its own `domain_oid!(pub AstOid)` for `AstNode`. Two types with the same name and same doc comment ("Content address for AST nodes") in different modules is confusing.

**Suggestion:** Either use `ast::AstOid` as the associated type in `ContentAddressed for Ast`, or remove it. Rename `parse::AstOid` to `AstNodeOid` to avoid name collision.

### W4. Optics require an argument -- no bare `settle { body }` form

All five optic variants require a first argument (target/query/root/perspective/mutation). The parser (ast_prism.rs:286-295) handles `focus { body }` (optic name + bare body, no argument) as a `Call`, not an optic:

```rust
// name { body } (no parens)
skip_trivia(tokens, cursor);
if matches!(tokens.get(*cursor), Some(Token::LBrace)) {
    *cursor += 1;
    let body = parse_body(tokens, cursor);
    return Ast::Call {
        name: Atom::new(name),
        args: vec![Ast::Body(body)],
    };
}
```

This means `settle { settle }` is a Call, not a Settle optic. If the grammar intends that optics can be argumentless (e.g., "settle into the settled state, body is the proof"), the AST needs an `Option<Box<Ast>>` for the first field, and the parser needs a branch for optic-name + bare body.

If the grammar intentionally requires an explicit argument, this is correct behavior. But it should be documented with a test: `settle_bare_body_is_call_not_optic`.

### W5. `focus(x, { y })` vs `focus(x) { y }` -- implicit disambiguation

The parser distinguishes these two forms:
- `focus(x) { y }` -- single arg in parens, body follows: produces `Ast::Focus`
- `focus(x, { y })` -- body inside the argument list as second arg: produces `Ast::Call` with args `[Atom("x"), Body([Atom("y")])]`

This is correct but relies entirely on the comma. The forms are visually similar. The existing test `parse_optic_without_body_stays_call` covers the no-body case, and `parse_non_optic_name_with_body_stays_call` covers non-optic names. But there is no test for `focus(x, { y })` specifically, which is the closest ambiguous form.

**Suggestion:** Add a test `focus_with_body_in_args_stays_call` that verifies `focus(x, { y })` produces a Call, not a Focus optic.

### W6. bincode 1.x version pinning -- forward compatibility risk

Cargo.toml pins `bincode = "1"`, which means any `1.x` release. Bincode 1.x uses a legacy serialization format by default (when calling `bincode::serialize`), while `bincode::options()` uses a different wire format (varint encoding). These are mutually incompatible.

If a future bincode 1.x release changes the default behavior of `bincode::serialize` (unlikely but possible with a major semver-compatible change), existing `.shatter` blobs would become unreadable. More practically: if the code is ever migrated to `bincode::options()` for the size limit fix (W2), existing blobs will break silently because the wire format changes.

The SHATTER_VERSION byte (currently 1) exists precisely for this. But there's no documented migration strategy -- what happens when version 2 is introduced? Does the deserializer try both? Does it reject with an error? The current code does the right thing (reject with `UnsupportedVersion`), but the producer side has no mechanism to write a v2 blob.

**Suggestion:** Pin `bincode = "1.3"` specifically. Document that SHATTER_VERSION 1 = bincode 1.x legacy format, little-endian, fixed-size integers.

---

## NOTE --- Observations

### N1. TDD discipline is clean

Four commits: Phase 1-3 (optic variants + Display + content addressing), Phase 4 (parser), Phase 5 (serde), Phase 6 (blob). Each phase has red-then-green commits. 17 new tests across the four phases. The tests are behavioral and cover the right edge cases.

### N2. Parser disambiguation strategy is sound

The rule -- optic name + single arg + body = optic variant -- is simple and unambiguous. The `is_optic_name` check is a constant array lookup. The `make_optic` function is a straightforward match. The `parse_primary` function correctly limits greedy consumption for optic space-separated args. This is well-designed.

### N3. Display round-trip holds for all tested cases

The test `parse_optic_round_trip` verifies `parse(display(parse(source))) == parse(source)`. The Display always normalizes optics to `name(arg) { body }` form with parentheses, which is the canonical form the parser expects. This is correct.

### N4. The five optics are the right decomposition

Focus (read-only observation), Project (dimensionality reduction), Split (connectivity exploration), Shift (perspective shift), Settle (the one write) map cleanly to the Prism trait operations. The field names (target/query/root/perspective/mutation) are distinct and meaningful. There is no missing operation in the five-operation model.

### N5. Feature flag isolation is correct

The `shatter` feature flag gates `serde::Serialize`/`serde::Deserialize` derives on all AST types and the entire `shatter_blob` module. Code compiles and 698 tests pass without the feature. 715 pass with it. The 17-test delta matches the shatter-specific tests. Clean separation.

### N6. Dirac connection is aspirational, not structural

The doc comment at ast.rs:97 says "composition of optics IS the Dirac operator." But `dirac.rs` operates on numeric graphs (adjacency matrices, eigenvalues, Connes distance). There is no code path from `Ast::Focus`/`Ast::Project`/etc. to `construct_dirac`. The comment is a design intention, not a current fact. This is fine as long as it's understood as roadmap, not documentation of existing behavior.

### N7. The Body-as-second-field pattern is uniform

All five optic variants share the same shape: `{ first_field: Box<Ast>, body: Body }`. This uniformity enables `emit_optic` to handle all five with a single function. If a variant eventually needs a different shape (e.g., Split with two roots, or Settle with a proof type), the uniform pattern will need to break. But for now, uniformity is the right call.

---

## GOOD --- Strong Decisions

### G1. Optic variants are AST-level, not sugar

Making Focus/Project/Split/Shift/Settle first-class enum variants rather than special-casing them during compilation means the AST honestly represents the user's intent. A Focus is structurally different from a Call named "focus" -- it carries different fields (target vs args), different semantics, and a different display form. This is the right level to make the distinction.

### G2. Box<Ast> for the argument, not Vec<Ast>

Optic variants use `Box<Ast>` for their first field, enforcing exactly one argument at the type level. This prevents constructing a Focus with zero or multiple targets. The parser enforces this too (line 270: `args.len() == 1`). Type-level and value-level enforcement agree.

### G3. Empty body is valid

`Focus { target: ..., body: Body::new(vec![]) }` is permitted and displays as `focus(x) {}`. This is correct -- an observation with no recorded results is still an observation. The parser handles this case (test `parse_optic_empty_body`).

### G4. SHTR magic is distinctive

The 4-byte magic `b"SHTR"` is unlikely to collide with other binary formats. The version byte immediately after allows format evolution. The error messages distinguish InvalidMagic from UnsupportedVersion from Deserialize failure. The error type implements `Display` and `Error`. This is clean.

### G5. Content OID verification in blob tests

Both `content_oid_matches_after_round_trip` and `content_oid_matches_nested` verify that serialization preserves content addressing. This catches the scenario where serde derives produce a different field order or skip a field.

---

## Security / Safety

### S1. No recursion depth limit in parser

`parse_expr` calls itself recursively through `parse_args` and `parse_body`. Deeply nested input (e.g., `focus(focus(focus(... 10000 deep ...)))`) will stack overflow. The tokenizer is iterative so it won't overflow, but the parser is recursive-descent with no depth counter.

For local `.mirror` files this is low risk. For any future scenario where untrusted input is parsed (MCP tool input, network protocol), this needs a depth limit.

### S2. No recursion depth limit in bincode deserialization

Similarly, `bincode::deserialize` will recursively deserialize nested `Ast` nodes. A crafted `.shatter` blob with deeply nested Focus-in-Focus-in-Focus will cause stack overflow during deserialization. Same risk profile as S1.

### S3. `serialize_shatter` panics on serialization failure

`shatter_blob.rs` line 62:
```rust
buf.extend(bincode::serialize(ast).expect("AST serialization cannot fail"));
```

The `expect` is correct in practice -- bincode serialization of these types cannot fail. But the asymmetry (serialize panics, deserialize returns Result) is worth noting. If a future type added to the AST has a custom Serialize impl that can fail, this will become a panic in production.

---

## Missing Test Specs

These tests would strengthen the review findings:

| Test | What it catches |
|---|---|
| `settle_bare_body_is_call_not_optic` | W4: Documents that `settle { settle }` is intentionally a Call |
| `focus_with_body_in_args_stays_call` | W5: `focus(x, { y })` stays Call, not optic |
| `display_format_pinned` | W1: Pin `format!("{}", known_ast)` to an exact string to catch formatting drift |
| `optic_with_body_target_round_trips` | Edge case: `Focus { target: Body([...]), body: [...] }` constructed programmatically -- does Display/parse round-trip? |
| `deeply_nested_optic_round_trip` | Nested optics: `focus(outer) { project(inner) { leaf } }` |
| `sequential_optics_in_prism_body` | Multiple optics as siblings: `prism @p { focus(a){x} project(b){y} }` |

---

## Summary

The AST optics refactor is architecturally sound. The five variants are the right decomposition. The parser disambiguation is clean. The serde/blob format works. The TDD discipline is exemplary.

The main risks are operational: content-addressing via Display form (W1) creates a fragile coupling that will bite on the first formatting change; the lack of deserialization size limits (W2) is a latent DoS vector; the mandatory-argument constraint (W4) may not match the grammar's intended expressiveness. None of these are blocking for the current scope, but W1 should be addressed before `.shatter` blobs are stored durably.

The Dirac connection (N6) is the big open question. The AST now has optic variants; `dirac.rs` has spectral triples. The bridge between them -- how does a composition of Focus/Project/Split/Shift/Settle on an AST produce a Dirac operator on a graph? -- is the next architectural decision. The current code does not constrain that decision, which is correct at this stage.

---

*Seam finds the seams. That is the job.*
