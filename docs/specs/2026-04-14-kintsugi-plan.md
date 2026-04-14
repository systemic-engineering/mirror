# Kintsugi Implementation Plan

**Author:** Mara
**Date:** 2026-04-14
**Status:** Executing

## Phase 1: Operator Table Completion

### 1a. `=>` Unfold token change
- Current: `>=` maps to `OpticOp::Unfold`
- Target: `=>` maps to `OpticOp::Unfold`, `>=` no longer does
- Also add `<-` as reverse Zoom

**Tests (red):**
- `operator_unfold_maps_to_fat_arrow` — `OpticOp::from_token("=>") == Some(Unfold)`
- `operator_reverse_zoom_maps_to_left_arrow` — `OpticOp::from_token("<-") == Some(Zoom)`
- `old_unfold_token_no_longer_matches` — `OpticOp::from_token(">=") == None`
- Update roundtrip test to use `=>` instead of `>=`
- Update display test

**Implementation:**
- `from_token`: remove `>=`, add `=>` for Unfold, add `<-` for Zoom
- `as_str`: change Unfold from `>=` to `=>`
- Update existing tests that assert `>=`

### 1b. `01-meta.mirror` update
- Change `unfold >=` to `unfold =>`
- Change `out >=` to `out =>`

## Phase 2: `form` keyword deprecation

**Test (already red):**
- `form_keyword_produces_warning` — `form @test { type x }` -> Partial with holonomy > 0

**Implementation:**
In `parse_form`, after all declarations are parsed, scan for any `DeclKind::Form`
at top-level. If found, inject a deprecation entry into `ParseLoss` and return
Partial. The `form` keyword is still recognized by `DeclKind::parse()` so it
parses fine — we just add loss after the fact.

Specifically: after building the form and before the final return, check if any
top-level decl has `kind == DeclKind::Form` or `kind == DeclKind::Grammar` came
from a `form` keyword. Since `form` parses as `DeclKind::Form`, check the
wrapper's kind. Add a deprecation `UnrecognizedDecl` with keyword `"form"` to
force Partial.

Wait — actually `form` IS a recognized DeclKind. The parser will parse it normally
as `DeclKind::Form`. We need to detect that and inject deprecation loss. The
simplest approach: after parsing all decls, if the top-level form's kind is
`DeclKind::Form` AND the source started with the word `form`, add deprecation
to the loss and return Partial.

Better approach: track a `deprecations: Vec<String>` alongside `unrecognized`.
When we see `form` as a keyword, parse it as `DeclKind::Grammar` (it's
semantically the same) but record a deprecation. Then in the return path, if
deprecations is non-empty, create Partial.

Simplest approach that works: in `parse_decl`, when `kind == DeclKind::Form`
and the keyword was literally `form`, rewrite it to `DeclKind::Grammar` and
set a flag. Then in `parse_form`, detect this flag and inject loss.

Actually simplest: in `parse_form`, after building decls, walk them. If any
has `kind == DeclKind::Form`, add a deprecation entry to `unrecognized` with
keyword `"form"` and a descriptive content. This will automatically cause
Partial because the unrecognized list is non-empty.

## Phase 3: Kintsugi — Canonical Ordering

**Tests (red):**
- `kintsugi_hoists_imports` — `in @X` moves to top
- `kintsugi_is_idempotent` — `kintsugi(kintsugi(x)) == kintsugi(x)`
- `kintsugi_preserves_oid` — OID same before and after

**Implementation:**
- `pub fn kintsugi(form: &Form) -> Form` in `mirror_runtime.rs`
- Sort `form.children` by canonical order: In=0, Type=1, Traversal=2, Lens=3,
  Grammar=4, Property=5, Action=6, everything else=7
- Stable sort to preserve relative order within same kind
- Return new Form with reordered children

## Phase 4: `--strict` flag

**Tests (red):**
- `strict_flag_rejects_partial` — compile with unrecognized + `--strict` -> exit 1

**Implementation:**
- In `cmd_compile`, check for `--strict` flag
- If present and result is Partial, return Err (exit code 1)

## Phase 5: `mirror kintsugi` command and `--check`

**Tests (red):**
- `kintsugi_command_reorders` — CLI command produces canonical output
- `kintsugi_check_passes_canonical` — already canonical -> exit 0
- `kintsugi_check_fails_non_canonical` — non-canonical -> exit 1

**Implementation:**
- Add `"kintsugi"` to CLI dispatch
- `cmd_kintsugi`: parse file, apply kintsugi, emit
- `--check`: compare emitted to source, if different exit 1
