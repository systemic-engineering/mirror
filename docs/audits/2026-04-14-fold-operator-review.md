# Adversarial Review: reed/fold-operator

**Reviewer:** Seam
**Date:** 2026-04-14
**Branch:** `reed/fold-operator` (20 commits, 14 files, +3568/-250 lines)
**Base:** `main` (seam/imperfect-parser)
**Status:** 520 tests pass, 1 test FAILS, 5 ignored

---

## Executive Summary

The branch introduces five connected features: the fold operator (`<=`), four-fold MirrorLoss refactor, error surface validation (M2001-M2004), kintsugi formatter, and form deprecation. The architecture is sound --- MirrorLoss as four named folds is a genuine improvement over the flat struct, and the `Imperfect` return type on `parse_form` correctly models measured loss. But there are real seams.

The blocking issue is the `mirror_ci_boot_baseline` test, which hardcodes counts (`holonomy <= 3.0`, `resolved.len() == 10`, `failed.len() == 5`) that are already stale against the current boot directory. The `--strict` implementation compiles the source twice. And the kintsugi OID preservation test passes for the wrong reason --- it tests too little.

---

## BLOCK --- Must Fix Before Merge

### B1. `mirror_ci_boot_baseline` test fails on current boot directory

The test hardcodes:
```rust
assert!(holonomy <= 3.0, ...);
assert_eq!(boot.resolved.len(), 10, "10 of 18 boot files resolve");
assert_eq!(boot.failed.len(), 5, "5 boot files fail resolution");
```

The boot directory now has 18 files including `01a-meta-action.mirror`, `01b-meta-io.mirror`, `02-shatter.mirror`, `08-git.mirror`, `09-package.mirror`, `15-time.mirror`, `17-benchmark.mirror`. The branch was written against an older boot layout. Running `cargo test` on the branch produces:

```
panicked: parse holonomy must not regress above baseline: got 6
```

This test cannot merge as-is. The baseline must be re-measured against the current boot directory, or the test must be rewritten to be relative (e.g., "holonomy must not exceed N per file" rather than a global constant).

**Fix:** Re-run `mirror_ci_boot_baseline` against the current boot directory, update all hardcoded counts, and consider making the baseline a snapshot file rather than inline constants.

### B2. `--strict` compiles the source twice

In `cmd_compile` (cli.rs, line 189-198):
```rust
if strict {
    let result = self.runtime.compile_source(&source);
    if result.is_partial() {
        return Err(...);
    }
}
match compiler.compile(&source) { ... }
```

When `--strict` is set, `compile_source` is called once for the check, then `compiler.compile()` calls it again. This is:
1. **Wasteful** --- full parse + compile happens twice for every `--strict` invocation.
2. **Inconsistent** --- `self.runtime.compile_source` and `compiler.compile` are different code paths (MirrorRuntime vs MirrorCompiler). If one returns Partial and the other doesn't, behavior diverges silently.

**Fix:** Call `compile_source` once. Store the `Imperfect` result. Check for Partial. If strict and partial, reject. Otherwise extract the value and proceed.

### B3. `collect_until_next_decl` does not handle all token types

`collect_until_next_decl` (mirror_runtime.rs) matches `Word`, `LBrace`, `RBrace`, `LParen`, `RParen`, `Comma`, `Equals`, and `Newline`. But the tokenizer also produces other token types (operator sequences tokenized as `Word`). The `None` arm breaks out, but any future token type that doesn't match these arms will silently spin --- the function does not advance the cursor for unmatched `Some(_)` variants. The last match arm is:
```rust
Some(Tok::Equals) => { content.push('='); *cursor += 1; }
None => break,
```

There is no `_ =>` catch-all. If a token type is added to `Tok` later, this becomes an infinite loop.

**Fix:** Add a `_ => { *cursor += 1; }` default arm to prevent infinite loops on unmatched token types.

---

## WARN --- Should Fix

### W1. Tokenizer ambiguity: `<` and `<=` disambiguation is fragile

The tokenizer groups operator characters into multi-character words:
```rust
'|' | '.' | '/' | '<' | '>' | ':' | '-' | '!' => { ... }
```

So `<=` becomes `Word("<=")`, `<` becomes `Word("<")`, `<-` becomes `Word("<-")`. The fold operator detection in `parse_decl` handles this differently:
```rust
let is_fold = matches!(tokens.get(*cursor), Some(Tok::Word(w)) if w == "<")
    && matches!(tokens.get(*cursor + 1), Some(Tok::Equals));
```

This expects `<` and `=` as separate tokens. But the tokenizer would group `<=` into `Word("<=")` if followed by another operator char. The reason this works is that `=` is NOT in the operator character set --- it's handled by a separate `'=' => Tok::Equals` arm. So `<=` is tokenized as `Word("<")` + `Tok::Equals`. This is correct but non-obvious. A comment in the parser explaining WHY `<=` is split across two tokens would prevent future confusion.

Meanwhile, `OpticOp::from_token("<=")` returns `Some(OpticOp::Fold)`, but this code path is never hit by the parser because the tokenizer never produces `Word("<=")`. The `from_token` mapping is dead code for this specific token. This is not wrong, but it's misleading --- the declaration.rs token table suggests `<=` is a single token, while the parser treats it as two.

### W2. `kintsugi_preserves_oid` test proves less than it claims

The test:
```rust
let oid_before = shatter.compile_form(&parsed).oid().clone();
let oid_after = shatter.compile_form(&canonical).oid().clone();
assert_eq!(oid_before, oid_after, ...);
```

This passes because `Fractal` computes a node's OID from `data.encode()` alone --- children hashes are not mixed into the parent hash. So the ROOT node's OID never changes regardless of child reordering. This is structurally correct but the test name and assertion message say "kintsugi must not change the content-addressed OID" without clarifying that this is only the root OID, not a Merkle root.

If the hashing scheme ever changes to a Merkle tree (where parent OID = hash(self_data + child_OIDs)), this test would start failing and kintsugi would break its core promise. The test should document this dependency explicitly, and ideally also verify that the full tree comparison (not just root OID) is order-independent.

### W3. `form` deprecation injects into `unrecognized` --- semantic mismatch

`collect_form_deprecations` pushes entries into `Vec<UnrecognizedDecl>` with `keyword: "form"` and `line: 0`. But `form` IS recognized --- it parses successfully as `DeclKind::Form`. The deprecation is shoehorned into the unrecognized-keyword mechanism to force Partial.

This conflates two different kinds of loss:
- **Unrecognized:** the parser does not know this keyword. Information is lost.
- **Deprecated:** the parser knows this keyword. No information is lost. The user should update.

Right now both produce identical `ParseLoss` entries. A downstream consumer cannot distinguish "your code has unknown keywords" from "your code uses old keywords." The kintsugi plan spec acknowledges this problem but chose the simplest path.

**Suggestion:** Add a `deprecated: Vec<DeprecationWarning>` field to `ParseLoss` alongside `unrecognized`. Both contribute to holonomy, but they're distinguishable.

### W4. M2001/M2002 validation position --- false positive risk

The bare keyword checks happen AFTER `parse_decl` returns:
```rust
if form.name.is_empty() && matches!(form.kind, DeclKind::Type | ...) {
    return Imperfect::failure(err(format!("M2001: `{}` requires a name", ...)));
}
```

This relies on `parse_decl` producing a Form with an empty name for `type\n`. But `parse_decl` for a type reads the next token as the name. If the next token is a newline, does `parse_decl` return an empty name or fail? The test `error_type_no_name` passes, so empirically this works. But the validation depends on `parse_decl` silently producing an empty-name Form rather than failing, which is a fragile contract. If `parse_decl` ever starts returning `Err` for bare keywords, the M2001 check becomes unreachable.

### W5. `PropertyVerdict` still uses `Imperfect<(), String, f64>`

The spec (error-surface-spec.md, Section 3) describes `PropertyVerdict` using structured types:
```
verdict: Imperfect<Form, Declaration, f64>
```

The implementation uses:
```rust
pub verdict: Imperfect<(), String, f64>,
```

The `()` success type means a passing property check carries no data about what passed. The `String` error type means property failures are unstructured messages, not typed declarations. This is explicitly noted in the spec as the target state, but the gap between spec and implementation is worth tracking.

### W6. `MirrorLoss::combine` drops convergence asymmetrically

```rust
fn combine(self, other: Self) -> Self {
    MirrorLoss {
        ...
        convergence: other.convergence,  // always takes the second
        ...
    }
}
```

This takes `other.convergence` unconditionally. If `self` is `BudgetExhausted` and `other` is `Settled`, the combined result claims `Settled`. This violates the monoid expectation that combining a degraded result with a clean one should not erase the degradation. The existing tests do not cover this case.

**Fix:** Use `max` semantics: `BudgetExhausted > Diverged > Converging(n) > Settled`.

### W7. `boot/01-meta.shatter` is an exact byte-for-byte copy of `boot/01-meta.mirror`

These two files are identical. If `.shatter` is meant to be the compiled output, it should differ from source. If it's a checkpoint, it should be noted. As-is, it's confusing --- it looks like a copy-paste artifact.

---

## NOTE --- Observations

### N1. TDD discipline is clean

Every feature follows red-green-refactor. Commit messages use phase markers. The branch has 9 red commits, 7 green commits, 2 refactor commits, and 2 spec commits. This is exactly right.

### N2. The four-fold MirrorLoss refactor is well-structured

`ParseLoss`, `ResolutionLoss`, `PropertyLoss`, `EmitLoss` each have `zero()`, `holonomy()`, `is_zero()`, `combine()`. The sub-loss types compose correctly into `MirrorLoss`. The holonomy computation is the sum of all four folds plus convergence penalty. Each sub-loss has independent tests. This is solid.

### N3. `Imperfect` return type on `parse_form` is the right call

Changing from `Result<Form, Error>` to `Imperfect<Form, Error, MirrorLoss>` makes the parser honestly report partial success. The three-valued result (Success/Partial/Failure) maps directly to the compiler's reality: some input is recognized, some is not, and the loss is measured. The `.map()` propagation through `compile_source` is clean.

### N4. `compile_boot_dir` loss accumulation is correct

The loop accumulates `total_loss` from each file's `Imperfect` result, handles Success/Partial/Failure correctly, and stores the accumulated loss in `BootResolution`. This is exactly what boot-level holonomy measurement needs.

### N5. The error test suite (M2001-M2004) is thorough

Empty source, whitespace-only, comments-only, bare keywords, duplicate types, unclosed braces, mixed valid/invalid, unknown operators, fold in wrong context --- these are the right edge cases. The tests are behavioral, not implementation-bound.

### N6. `mirror_ci_boot_success` is correctly ignored

The `#[ignore]` annotation with a clear reason ("blocked: boot files need in @form -> in @meta etc.") documents the goal state without pretending it's reached. This is honest.

### N7. Two aspirational tests document the target architecture

`declaration_fields_not_option` and `compile_returns_fractal_not_form` are BASELINE tests --- they assert the current (imperfect) state and document what the target state should be. This is a useful pattern for tracking architectural debt.

---

## GOOD --- Strong Decisions

### G1. Operator table redesign: `=>` for Unfold, `>=` retired

Retiring `>=` and using `=>` for Unfold is correct. `>=` is ambiguous with comparison operators. `=>` reads naturally as "unfolds into" and has established precedent (Rust match arms, JS arrow functions). The explicit test `old_unfold_token_no_longer_matches` proves the retirement is intentional.

### G2. `<-` as reverse Zoom maps to Zoom, not a new optic

Adding `<-` as a token that maps to `OpticOp::Zoom` (same as `->`) rather than creating a new `ReverseZoom` variant is the right call. The optic is the same; the direction is syntactic sugar. The operator table stays small.

### G3. Kintsugi sort is stable

Using `sort_by_key` (which is stable in Rust) means declarations of the same kind preserve their original order. This is critical for idempotency and for not surprising users.

### G4. `ParseLoss::holonomy` counts unrecognized entries, not their content size

Using `unrecognized.len() as f64` rather than measuring content volume means each unrecognized keyword contributes equally to holonomy. This is the right granularity --- one unknown keyword is one unit of loss regardless of how much content follows it.

### G5. The @ai grammar test is a real integration test

`ai_grammar_fold_not_silent` and `ai_grammar_resolves_against_boot` test the fold operator in context, not in isolation. The grammar declares `action boot(identity) <= imperfect` and verifies the fold operator survives through the full compilation pipeline including resolution against the boot registry. This catches integration failures that unit tests miss.

### G6. Error surface spec is thorough research

The spec reviews Rust, Elm, Go, Zig, and Gleam error presentation. The conclusion --- mirror errors measure loss rather than prescribe fixes --- is architecturally consistent with `Imperfect`. The error code catalog (M0001-M3xxx) provides room for growth.

---

## Spec vs Implementation Gaps

| Spec promise | Implementation status |
|---|---|
| Error codes M0001-M3xxx catalog | M2001-M2004 implemented. M0001 (no declarations), M1xxx (resolution), M3xxx (emit) not yet wired. |
| `mirror explain M0001` command | Not implemented. |
| `--json` output format for CI | Not implemented. |
| PropertyVerdict uses Form/Declaration types | Uses `Imperfect<(), String, f64>` |
| Kintsugi recursive reorder | Only reorders top-level children, not nested grammars. |
| Kintsugi preserves comments | Comments are stripped by the tokenizer; cannot be preserved. |
| `--check` compares output to source | Compares emitted text to re-emitted text, which may differ in whitespace from original source. |

---

## Security / Safety

### S1. No infinite loop risk in parser

The parser's main loop always advances the cursor: known keywords call `parse_decl` which advances, unknown words call `collect_until_next_decl` which advances, non-word tokens skip to newline. The only risk is `collect_until_next_decl` (see B3 above), which currently handles all produced token types but has no catch-all.

### S2. `kintsugi --check` reads from disk safely

File reading uses `std::fs::read_to_string` which is bounded by file size. No path traversal risk beyond what the OS provides. No symbolic link following beyond OS defaults.

### S3. `form` deprecation cannot suppress real errors

The deprecation adds entries to `unrecognized` but never removes them. It only makes the result MORE partial, never less. A file that would fail still fails; a file that would succeed now becomes Partial if it uses `form`. This is the correct direction.

---

## Summary

The branch is architecturally sound but not merge-ready due to the stale baseline test (B1). The `--strict` double compilation (B2) is wasteful and should be fixed. The `collect_until_next_decl` missing catch-all (B3) is a latent infinite loop. The warnings (W1-W7) are real but not blocking.

The fold operator design, four-fold loss model, and error surface tests are all strong work. The TDD discipline is exemplary. Fix the three blockers and this is ready.

---

*Seam finds the seams. That is the job.*
