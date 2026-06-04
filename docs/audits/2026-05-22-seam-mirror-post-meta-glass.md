# Seam audit — mirror post-meta-glass + apply_h heterogeneous

*2026-05-22. Seam (`@security`). Report mode.*

Scope: the Combinator enum + walker + `prism_seed` + `combinator_tree_oid` + two-phase normalize, the mq-query `=>` rewrite + kintsugi `--transform`/`--out`, `prism_core::apply_h`'s heterogeneous-state relaxation, and the spec-vs-code consistency for `parser-as-prism-grammar.md`, `kintsugi-tournament.md`, `combinator-optimization.md`.

The threat-model claims I am holding up against the code: FP1 (`apply_h(prism_seed(), glass.mirror.bytes) == prism_seed()` OID-equal), beta-normalization confluence, kintsugi-transform structural safety, `boot/ → mirror/` migration reversibility, walker totality on well-formed inputs, dark-span byte-stability, and `apply_h` type soundness under the new `Refracted = Optic<In, Out, E, L>` signature.

I name the seams where the claim ends and the implementation diverges. I find what's there. I do not invent findings.

---

## F-1: FP1 is vacuous — `walk_combinator` is the structural identity on the seed's variants

**Severity:** high
**Category:** correctness / contract-violation
**Location:** `bootstrap/src/spectral.rs:1387-1497` (`walk_combinator`), `:1552-1576` (`prism_seed`), `:1828-1858` (`fp1_meta_glass_parses_itself`)

**The seam.** The spec (`parser-as-prism-grammar.md`) reads FP1 as: "the meta-glass parses itself — apply the seed combinator to `glass.mirror`'s bytes and round-trip to a tree with the same OID." A reader assumes that "parse" means: the combinator tree was validated against the source bytes; if the bytes don't conform to the grammar described by the combinator, FP1 fails.

The implementation does not do that. `walk_combinator` is *structural-self* on every non-`Choice` variant: it recurses into children and re-wraps under the same constructor, ignoring the bytes. The bytes are passed in (`source: &[u8]`) and immediately dropped except in the `Choice` arm, which filters `LiteralKind` arms by `branch_keyword_occurs(source)`. `prism_seed()` contains **no `LiteralKind`** variant (it uses bare `Literal(b"grammar")`, `Literal(b"settle")`, etc.). The Choice-pruning code path therefore never activates on the seed — the seed walker returns the seed verbatim regardless of input.

Consequence: `combinator_tree_oid(walk(seed, bytes)) == combinator_tree_oid(seed)` holds for **any** byte string, including random noise, the empty file, the contents of `/dev/urandom`, or a `.mirror` file that doesn't conform to the meta-glass at all. The seed's own docstring acknowledges this (`bootstrap/src/spectral.rs:1551` "FP1 ... Holds because every structural variant's `walk_combinator` arm is structural-self") but the spec markets FP1 as the load-bearing self-parse equation. The two are not the same property.

**Reproduction.**
1. The `nl_mirror_lifts_cleanly` test already proves this: it walks the meta-glass against `nl.mirror` (a completely different file) and asserts no-Dark via `no_dark_in_tree`. `no_dark_in_tree` only checks for `AstKind::Dark` *combinator* variants — but `walk_combinator` never emits Dark, never emits `DarkFallback` synthetically, never inspects bytes. The test passes vacuously.
2. Concretely: construct a one-byte source `b"\x00"`. Call `apply_h(&prism_seed(), (vec![0u8], 0usize))`. The result's OID is identical to `combinator_tree_oid(&prism_seed())`. There is no parse failure surface; FP1 is satisfied by definition of the walker.
3. The `fp2_well_formedness_of_meta_glass_lift` test (`spectral.rs:1865`) compounds the issue. It parses `00-prism.mirror` — which contains `abstract io tick(type) -> tock(type) { \ }` — through `meta_glass`. The seed has **no `abstract_form` Combinator arm**. A real parser would fail or emit Dark for the `abstract` declaration. The structural-self walker emits zero Dark because it never inspects bytes.

**Impact.** The headline self-hosting equation is currently a tautology, not an invariant. Three concrete downstream risks:

- **Spec drift accumulates silently.** Any future grammar that doesn't actually round-trip through the meta-glass will still pass the FP1/FP2 tests. The regression detector is off.
- **The kintsugi acceptance criterion `mirror craft --strict boot` cannot rely on FP1 to certify "the meta-glass classifies everything"**. The two-phase strict-classification surface in `main.rs` still uses the hand-written `tokenize` (the FP1 surface isn't wired into `cmd_compile`/`cmd_craft` yet). When it IS wired, the wiring must replace structural-self with byte-consuming dispatch, or the strict-classification contract collapses.
- **A spec consumer (Glint, an LLM, a future contributor) reading "FP1 holds at glass.mirror" will infer that the seed parses the file. They will encode that assumption into downstream changes. Silence dies — but this is the silence the strict-classification spec was written against.**

**Remediation.** Two paths, both honest:

1. **Rename the property in the spec.** Call the current invariant `FP1-structural` ("the seed is in normal form under `walk_combinator`") and explicitly mark `FP1-byte-parsing` ("the seed accepts exactly the bytes of `glass.mirror`") as not-yet-implemented. The current tests certify FP1-structural truthfully. The naming change is one edit; downstream specs that depend on FP1-byte-parsing get explicitly red-flagged.
2. **Implement byte-consuming walks** in subsequent ticks: each `walk_combinator` arm advances an offset through the source bytes; mismatches emit `DarkFallback` or `Dark` combinator subtrees. FP1 then holds iff the meta-glass actually classifies `glass.mirror`'s bytes — and `00-prism.mirror`'s `abstract` form forces the seed to grow an `abstract_form` arm before FP2 passes.

I recommend (1) immediately (truth before next commit) and (2) before Tick 4c retires `grammar.rs` — because tick 4c's premise is that the meta-glass is the parser, and the meta-glass isn't a parser yet.

---

## F-2: kintsugi `--transform` is not structurally safe — it rewrites prose, identifiers, and obligation bodies

**Severity:** high
**Category:** correctness / contract-violation
**Location:** `bootstrap/src/pipeline.rs:67-99` (`apply_rewrites`), `:11-26` (`RewriteRule` doc), `bootstrap/src/main.rs:495-575` (`cmd_kintsugi_migrate`)

**The seam.** The `RewriteRule` doc (`pipeline.rs:11-26`) reads: "The meta-glass identifies which tokens are structural ... versus prose. The rewrite applies only at structural-token boundaries; English in `@nl` comments containing the symbol stays unchanged because @nl lifts those bytes opaquely through the cross-grammar boundary."

That is the claim. The implementation is a flat byte-level whole-word match with no parser involvement at all. `apply_rewrites` runs **before** `tokenize`, sees the raw source bytes, and applies the rewrite to every whole-word occurrence regardless of context. The existing test `apply_preserves_english_in_prose` (`pipeline.rs:138-148`) actually pins the **wrong** semantics — its name says "preserves English," its assertion is `b"# the grammar of mirror\n"` → `b"# the glass of mirror\n"`. The comment in that test ("structural-safety via @nl is the 4b.4 layer; for 4b.3 the whole-word boundary is what's implemented") names the seam honestly but the public-facing doc on `RewriteRule` does not.

There are at least four concrete bytes-equivalent scenarios where the byte-level rewrite over-applies, each one a corpus-corrupting risk under `mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/`:

1. **Prose inside `@nl` comments.** `# the grammar of mirror` → `# the glass of mirror`. Comments are prose; the meta-glass docstring claims the shift protects them; it doesn't.
2. **String literals containing the keyword.** The seed has no string-literal recognizer yet (no quoted-literal Combinator variant); any future grammar that introduces `"grammar"` as a payload (test fixtures, doctest expected output, error messages, paths in shell pipelines embedded in `@nl/code` doctests) gets silently rewritten.
3. **Obligation bodies.** The seed's `settle_form` uses `Until { stop: Literal("\n") }` for the body — a backslash-only obligation body `= \` would be untouched, but a body referencing `grammar` (e.g., a doctest assertion line saying `mirror> grammar @x`) would be rewritten because the rewrite runs before parse.
4. **Spec-A `abstract io` bodies.** Spec-A's `\` obligation body lives between `{` and `}` — but if a future obligation references the symbol explicitly (`{ \ # discharge once @mirror/grammar is loaded }`), the rewrite hits it.

The migration mode (`cmd_kintsugi_migrate`) compounds (1)–(4) because it rewrites the entire `boot/` tree byte-for-byte, then drops the path-canonicalization shim (`std/mirror/` strip), then renames file basenames matching `<sym>.mirror`. There is no dry-run flag, no diff preview, no rollback. A single `mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/` corrupts every comment, doctest, and string literal containing `grammar` across the corpus simultaneously.

**Reproduction.**
1. `apply_rewrites(&[RewriteRule{symbol:"grammar".to_string(), replacement:"glass".to_string()}], b"# this is a grammar comment\n")` → `b"# this is a glass comment\n"`. Pinned by `apply_preserves_english_in_prose`.
2. The current `boot/std/mirror/grammar.mirror` (3.9KB) contains the literal token `grammar` inside `@nl` prose at several sites: `# the meta-glass: mirror's grammar declared in mirror itself.` (line 2 of the brace block) — would be rewritten.
3. The kintsugi-tournament spec (which I sampled separately) mentions `@nl/code` doctest blocks; any block where the user wrote `mirror> compile std/mirror/grammar.mirror` as expected stdout becomes `mirror> compile std/mirror/glass.mirror`. If that doctest is also asserting the literal path string, the doctest now fails post-migration.

**Impact.** The migration claim "reversible" is not true under the current implementation. The byte-level rewrite is lossy: after the forward migration, the inverse `glass => grammar` would re-apply to the rewritten English ("the glass of mirror" → "the grammar of mirror") — which happens to be correct here but is correct by coincidence, not by structure. Any English phrase that *originally* contained `glass` (the future-state replacement) is now ambiguous with the rewritten English phrase that *originally* contained `grammar`. The history of which-was-which lives only in `git`, not in the file system. The bidirectional claim depends on the absence of `glass` in source prose pre-migration — which is empirical, not structural.

**Remediation.** Three concrete options ordered by cost:

1. **Re-doc honestly.** Change `RewriteRule`'s docstring to read: "Byte-level whole-word rewrite. No parse-time structural safety. Runs before tokenize; matches *every* whole-word occurrence in source bytes regardless of @nl/comment/string context." Rename the `apply_preserves_english_in_prose` test to `apply_does_rewrite_inside_prose_today` so the test name pins the actual semantics. One edit; restores honesty. This is the minimum.
2. **Add `--dry-run` to `cmd_kintsugi_migrate`** that emits a unified diff to stdout without writing. Cheap, immediate harm-reduction.
3. **Land the parser-aware rewrite** that 4b.4 promises: rewrite only at AST-node-boundary positions where `node.grammar_tag` is `@mirror/grammar` AND `node.kind` is not `Dark` AND the node's byte span sits outside any `@nl`-tagged child's span. The byte-level fallback stays as a `--unsafe` opt-in.

Lock in (1) before the next migration is attempted. (2) is one afternoon. (3) is the 4b.4 tick.

---

## F-3: `apply_rewrites` and `walk_combinator` disagree on word-byte definition — path components leak

**Severity:** medium
**Category:** correctness / boundary
**Location:** `bootstrap/src/pipeline.rs:71-73` (`apply_rewrites::is_word_byte`), `bootstrap/src/spectral.rs:1518-1520` (`spectral::is_word_byte`)

**The seam.** Two whole-word-boundary checks, identically named, disagree:

- `pipeline.rs:71-73`: `is_word_byte = alnum | b'_'` (no `/`). Comment claims this is "the same as `is_word_byte` in spectral.rs's combinator walker."
- `spectral.rs:1518-1520`: `is_word_byte = alnum | b'_' | b'/'`.

The comment in `apply_rewrites` is wrong on its face. The functions are *not* the same. Worse, the divergence is load-bearing: the walker's whole-word boundary check (used in `branch_keyword_occurs`) treats `/` as part of the word, so the keyword `grammar` in `@mirror/grammar` is **not** matched as a whole-word (left-boundary `r` and right-boundary end-of-string, but the substring search starts in middle of `mirror/grammar` and the walker uses `/` as a word byte → the keyword `grammar` IS bounded by `/` on the left, but `/` is a word byte, so left_ok = !is_word_byte('/') = false → does not match).

The rewrite path uses the narrower definition (`/` is **not** a word byte). So `apply_rewrites('grammar' => 'glass', '@mirror/grammar')` → `@mirror/glass` (intended). But the walker's `branch_keyword_occurs` would NOT match the same `grammar` in the same source. The two halves of "structurally safe rewrite" use opposite boundary rules.

**Reproduction.**
1. `apply_rewrites([{symbol:"grammar", replacement:"glass"}], b"@mirror/grammar")` → `b"@mirror/glass"`. (Rewrite triggers because `/` is non-word in rewrite-rules.)
2. `branch_keyword_occurs(&LiteralKind{keyword:b"grammar", ...}, b"@mirror/grammar")` → `false`. (Walker treats `/` as word, so left_ok fails.)
3. A combinator-tree pruning over the same source would drop a `LiteralKind{"grammar"}` arm because the keyword doesn't appear; the byte-level rewrite over the same source would expand it. Two surfaces, one source, opposite verdicts on "is `grammar` present here."

**Impact.** Path-component renames work via byte-level rewrite but would NOT be detectable by the meta-glass-aware version of the same operation. Once 4b.4 lands the parser-aware rewrite (per F-2 remediation), this disagreement becomes a behavior change: file paths that the byte rewriter would touch will silently NOT be touched by the parser-aware version, and the migration test corpus diverges between branches. Not a today-exploit; a future-correctness landmine that's invisible in the current tests.

**Remediation.** Pick one definition and use it from a single shared module. `bootstrap/src/spectral.rs::is_word_byte` is the older one; `apply_rewrites`'s narrower variant is the load-bearing one for path-component renames (the comment in `apply_rewrites` declaring `@` and `/` as boundaries is the intent). Shift it to `ast.rs` or a shared `word.rs`, re-import both call sites, write a property test asserting they agree on every byte in `0..=255`.

---

## F-4: Walker, normalize, and Fold5 all recurse without depth limits — stack overflow is attacker-controlled

**Severity:** medium
**Category:** security / boundary
**Location:** `bootstrap/src/spectral.rs:1387` (`walk_combinator`), `:1612` (`normalize_phase1`), `:1744` (`normalize_phase2`), `:466` (`Fold5::run`), `:540` (`Fold5At::run`)

**The seam.** Every walker over `Combinator` or `AstNode` is a recursive descent with no explicit depth limit, no manual stack, and no compiler-enforced bound. `Combinator` itself has unbounded recursion through `Repeat::body`, `Capture::body`, `BraceBlock`, `ParenBlock`, `Until::stop`, `Shift::body`, and `Seq`/`Choice` children. `AstNode::children` is a `Vec` with no depth cap.

In practice this means: a `.mirror` file with N deeply nested `{`s parses to an AST with N nested `BraceBlock` nodes; `compute_content_oid` and any future `walk_combinator` call recurse N deep on the default Rust thread stack (8 MB). The crash point sits around N ≈ 30,000–80,000 depending on frame size — well within a 100 KB attacker-controlled file.

The `tokenize` path (`bootstrap/src/tokenize.rs::scan_brace_block`) already handles nesting iteratively with a depth counter, so deep braces don't crash the tokenizer. But the downstream walks (`compute_content_oid` via Fold5, the future `walk_combinator` over a parsed grammar tree) inherit no such protection.

The strict-classification spec promises that "every byte is classified" — but for a 100KB file of `{{{...{ \ }...}}}`, classification crashes the process before the verdict is rendered. The contract becomes "every byte that fits within the default stack is classified."

**Reproduction.**
1. Construct a `.mirror` file with 50,000 nested `{`s followed by 50,000 nested `}`s. (Under 200KB.)
2. Run `mirror compile --strict <file>`. The tokenizer parses it (iteratively). `compute_content_oid` recurses via `Fold5::run` 50,000 deep. Process aborts with `thread '<unnamed>' has overflowed its stack`.
3. Exit code is signal-based abort, not the strict-classification exit code 2. The contract "exit 2 ⇔ dark regions present" is violated by "exit signal ⇔ deep nesting."

**Impact.** Two concrete attack surfaces:

- **DoS on shared infrastructure.** Any service that processes user-uploaded `.mirror` files (a future LSP, the MCP server, CI) can be crashed by a small adversarial input. The C boot is locked at compile-time so this isn't a remote-execution surface — it's an availability one.
- **The strict-classification exit-code contract leaks.** Downstream tooling that depends on `exit code 2 ⇔ dark` (per `docs/specs/strict-and-total-classification.md`) gets a third state (`exit code -SIGABRT`) it isn't expecting.

**Remediation.** Two reasonable options:

1. **`stacker` crate** (zero-dep grow-the-stack-on-demand) wrapped around the recursive calls in `Fold5::run`, `Fold5At::run`, `walk_combinator`, `normalize_phase1`, `normalize_phase2`. Standard pattern for tree-walkers; ~5 lines of change.
2. **Explicit depth cap** in each walker. Reject inputs above (say) 4096 depth as a `Dark` region with a `nesting_exceeded` hint. Honest under the strict-classification spec — the input is in the algebra's domain, the walker refuses, and the obligation surfaces as a verifiable property.

I lean (2). It composes with `total_classification`'s existing language; an attacker who tries deep nesting hits the strict-classification refusal path, not an ill-defined crash.

---

## F-5: `normalize_phase1` does not produce `DarkFallback` on `Choice([])` — silently keeps the malformed form

**Severity:** medium
**Category:** spec-consistency / contract-violation
**Location:** `bootstrap/src/spectral.rs:1639-1666` (`normalize_phase1` Choice arm)

**The seam.** The spec `combinator-optimization.md` §1.1 declares E14: "Empty Choice. `Choice([])` is NOT well-defined — a Choice with no arms always fails. We treat this as an error condition (should not appear in well-formed combinator trees)." The accompanying code is required (by §2.4's confluence argument) to either (a) reject `Choice([])` at construction, (b) reduce it to a canonical failure form (`DarkFallback` is the closest fit), or (c) explicitly mark it as undefined behavior.

The implementation does none of these. `normalize_phase1`'s `Choice` arm (line 1639–1666) computes the truncated arms list, then matches:
```
match truncated.len() {
    1 => truncated.into_iter().next().unwrap(),
    _ => Choice(truncated),
}
```
For `truncated.len() == 0` the `_` arm fires and returns `Choice(Vec::new())` — exactly the malformed form the spec said shouldn't appear. There is no panic, no error, no rewrite to `DarkFallback`.

Compounded by `normalize_phase2`: the same `Choice([])` falls through to the "every arm is a Literal" check, which is vacuously true on an empty iter — but the code guards with `!normalized.is_empty()` (line 1762) so this specific path is closed. The phase-2 arm therefore returns `Choice([])` unchanged, propagating the malformed form.

The seam matters because the spec's confluence argument (§2.4) depends on the redex set covering every well-formedness concern. If `Choice([])` can appear in normal-form trees, then any future optimization that assumes "normal-form trees are well-formed" silently encounters it.

**Reproduction.**
1. `normalize(&Combinator::Choice(Vec::new()))` returns `Combinator::Choice(Vec::new())` unchanged.
2. `combinator_tree_oid(&Combinator::Choice(Vec::new()))` returns a stable hash for the malformed form.
3. A future combinator that wraps `Choice([])` (e.g., a generated tree where a recursive grammar pruned all arms via the source-pruning code path in `walk_combinator`'s Choice arm) ends up with `Choice([])` in normal form — which the spec explicitly says shouldn't exist.

`walk_combinator::Choice` is itself the smuggling vector: if every `LiteralKind` arm of a Choice fails `branch_keyword_occurs` on adversarial input, `kept` is empty and the walker returns `Choice([])`. There is no `DarkFallback` fallback in the current walker.

**Impact.** Not exploitable today (the seed has no LiteralKind-only Choices). Becomes load-bearing the moment a grammar adds a LiteralKind Choice that an adversarial input can fully prune — at which point the normal form contains a malformed Choice and downstream walks that assume well-formedness break in spec-violating ways.

**Remediation.** One of:

1. In `normalize_phase1`'s Choice arm, add `0 => DarkFallback` as a fourth match. The spec's "always fails" semantics maps to `DarkFallback` (which "always succeeds by emitting Dark") — closest fit; pins the property.
2. In `walk_combinator`'s Choice arm, after the `kept` collection, if `kept.is_empty()` append `DarkFallback`. Symmetric protection at the producer side.

I'd do both. Each is ~3 lines and adds a regression test.

---

## F-6: The migration `boot/ → mirror/` is documented as a one-pass invariant; the code hasn't run it and the path-canonicalization rule asymmetrically maps `std/mirror/` → root

**Severity:** medium
**Category:** spec-consistency / boundary
**Location:** `bootstrap/src/main.rs:545-553` (path canonicalization in `cmd_kintsugi_migrate`), `bootstrap/src/spectral.rs:1700-1701` (`GLASS_PATH` test constant), `docs/specs/parser-as-prism-grammar.md` (the spec's migration claim)

**The seam.** Three statements that don't agree:

- **Spec (`parser-as-prism-grammar.md` line ~63):** "The rename `grammar → glass` and the directory migration are one kintsugi pass: `mirror kintsugi boot/ --transform='grammar => glass' --out=mirror/`."
- **Code (`spectral.rs:1700`):** `const GLASS_PATH: &str = "std/mirror/grammar.mirror"` — the FP1/FP2 tests read from `boot/std/mirror/grammar.mirror`, not `mirror/glass.mirror`. The migration hasn't happened.
- **Migration logic (`main.rs:548`):** `rel.strip_prefix("std/mirror/").or_else(|| rel.strip_prefix("std/"))` — drops the `std/mirror/` prefix unconditionally. So `boot/std/mirror/grammar.mirror` migrates to `mirror/grammar.mirror` (or `mirror/glass.mirror` after rename), NOT `mirror/std/mirror/grammar.mirror`. **The migration is therefore lossy at the directory level.** Other files under `boot/` that DON'T sit in `std/mirror/` (e.g., `boot/00-prism.mirror`) keep their relative path, so `boot/00-prism.mirror` migrates to `mirror/00-prism.mirror`. The migration "drops `std/mirror/` from one branch of the tree while preserving everything else" is structural-asymmetric.

Two seams flow from this:

1. **Reversibility fails for path structure.** Running `mirror kintsugi mirror/ --transform='glass => grammar' --out=boot/` would not restore `boot/std/mirror/grammar.mirror` — it would write `boot/grammar.mirror`. There is no inverse path-canonicalization rule that re-adds `std/mirror/`.
2. **Co-located files collide.** If `boot/std/mirror/foo.mirror` and `boot/foo.mirror` both exist, the migration writes both to `mirror/foo.mirror`, with the order-of-iteration determining who wins. (`collect_files` walks in `read_dir` order — file-system-dependent.) The current corpus doesn't trigger this, but the rule is fragile.

**Reproduction.**
1. Inspect `boot/std/mirror/` directory listing (shown in the audit prep): 21 files under `boot/std/mirror/` plus 18 files at `boot/`'s root. The migration's `strip_prefix("std/mirror/")` would map the former 21 to `mirror/*.mirror`, the latter 18 to `mirror/*.mirror` (collisions with any matching basename).
2. Concrete collision today: none, because no two files share a basename across the two layers. But e.g. adding `boot/grammar.mirror` (root-level) alongside `boot/std/mirror/grammar.mirror` makes one of them disappear silently on migration.
3. `GLASS_PATH = "std/mirror/grammar.mirror"` is still the live test path; the migration hasn't run on the current corpus. The audit threat model's "the migration `boot/ → mirror/` is reversible" claim is therefore not yet verifiable — there's nothing to invert.

**Impact.** Low blast radius today; high blast radius the moment migration runs. If executed as documented, the inverse pass cannot reproduce the original directory layout, and any future collision corrupts the corpus silently. The kintsugi-tournament spec leans on this migration being reversible; that's currently not provable.

**Remediation.**

1. **Make the path canonicalization symmetric.** Add an explicit `--from-prefix` and `--to-prefix` to `cmd_kintsugi_migrate`. The forward pass uses `from=std/mirror/, to=` (drop); the inverse uses `from=, to=std/mirror/` (add). Both pre-compute and refuse on collision.
2. **Pre-flight collision check.** Before writing anything, enumerate all (path, rewritten-path) pairs and refuse if any two source paths map to the same dest. Currently the migration walks-and-writes; replace with walk-collect-validate-then-write.
3. **Until migration runs, update the spec** to say "the migration will be one kintsugi pass once landed; today it's described, not executed." (F-1's tone applies: name what's there, name what isn't.)

---

## F-7: `apply_h`'s heterogeneous relaxation is type-safe; the audit thread is closed

**Severity:** info
**Category:** type-safety
**Location:** `prism/core/src/lib.rs:148-188` (the `apply_h` definition added in commit `7b78778`)

**The seam.** The threat model asks: "Are there call paths where the new freedom admits a beam transition that the old constraint would have rejected? Specifically, paths where `In` and `Out` could become unsoundly related at the type level."

I traced the type signature:

```
pub fn apply_h<P, SIn, In, Out, E, L>(prism: &P, state: SIn) -> Imperfect<Out, E, L>
where
    P: Prism<Input = Optic<(), SIn>, Refracted = Optic<In, Out, E, L>>,
    L: Loss,
```

The four type variables `SIn`, `In`, `Out`, `E` are entirely independent. The `Prism` trait's three-phase chain (`Input → Focused → Projected → Settled`) enforces all the typing constraints internally — `prism_core::apply` (which `apply_h` delegates to) does the actual composition. `apply_h` is just a convenience that constructs the seed beam (`Optic::ok((), state)`) and unwraps `into_focus` to drop the source dimension `In`.

Under the previous over-constrained shape (`Settled = Optic<_, S>` with `S` matching `Input`'s value position), heterogeneous prisms had to bypass `apply_h` and hand-roll the construction. The new shape *broadens* the set of well-typed call sites but does not relax any soundness invariant — every constraint that was checked before is still checked by the trait bound `P: Prism<...>`. There is no way for `In` and `Out` to be silently fused or aliased at the type level; the trait bound names them separately, and any consumer choosing them in a way that violates the prism's internal `focus → project → settle` typing will fail to compile.

I attempted to find a constructible misuse (e.g., a prism whose `Refracted::value` type doesn't match its `focus` output, or where the `Loss` carrier diverges from the `Holonomy` constraint). Every path is closed by either the `Prism` trait bound itself or the `L: Loss` bound. The relaxation is *strictly* additive on the well-typed surface.

**Reproduction.** N/A — type-soundness audit, not a runtime exploit.

**Impact.** None. The change is sound.

**Remediation.** None required. Noted for the record because the audit thread named this specifically; the answer is: closed.

---

## F-8: `apply_rewrites` runs each rule in document order over the running buffer — second-rule interactions on first-rule output are silent

**Severity:** low
**Category:** correctness
**Location:** `bootstrap/src/pipeline.rs:75-99` (the outer `for rule in rules` loop)

**The seam.** `apply_rewrites` iterates rules in order, with `current` updated after each rule. The doc on `parse_rewrite` says rules are joined with `;` (`grammar => glass; foo => bar`). The semantics is sequential, not parallel: rule 2 sees rule 1's output.

Consequence: `grammar => glass; glass => mirror` rewrites `grammar` to `mirror` (via the intermediate `glass`), not "rewrite `grammar` to `glass` and any pre-existing `glass` to `mirror` simultaneously." A user writing `glass => mirror; grammar => glass` expects the second rule's `glass` to be unaffected by the first rule's output — but the second rule fires on the buffer post-first-rule. Both reads are defensible; the doc doesn't pin which.

For a single-rule migration (the documented use case) this is invisible. For chained rules — which the spec sketches as a coming feature — it is silent and order-dependent.

**Reproduction.**
1. `apply_rewrites(&parse_rewrite("a => b; b => c").unwrap(), b"a")` → `b"c"` (chained).
2. `apply_rewrites(&parse_rewrite("b => c; a => b").unwrap(), b"a")` → `b"b"` (parallel-style).
3. Doc on `RewriteRule` is silent; doc on `parse_rewrite` is silent. Two readers, two correct answers.

**Impact.** Low today. Material if/when chained rules become a documented feature for the migration toolchain.

**Remediation.** Two-line doc edit on `apply_rewrites`: "Rules apply sequentially: rule N sees rule (N-1)'s output. For parallel-style semantics, the call site must run each rule against the original source independently and merge by chosen conflict policy." Then add a test pinning both behaviors.

---

## F-9: `parse_rewrite` is order-fragile under `=>` and `>=` substring collisions

**Severity:** low
**Category:** correctness
**Location:** `bootstrap/src/pipeline.rs:32-58` (`parse_rewrite`)

**The seam.** `parse_rewrite` accepts the query if it `.contains("=>")`. Any mq-query string that happens to contain `=>` as a substring is classified as a rewrite. The mq pipeline language uses `|>` (and `|\>`) as composition operators; an mq-query that uses `=>` for any other purpose (e.g., a future "where" clause, an embedded JSON document with `=>` in it, an `@nl/code/javascript` block containing `=>` in arrow functions) gets misclassified as a rewrite query.

`splitn(2, "=>")` further hardcodes the splitting; only the FIRST `=>` is the separator. A symbol whose name contains `=>` (impossible today in `.mirror` identifiers but possible in any quoted-string version of the syntax) is silently misparsed.

This is in tension with `is_mq_query` (`pipeline.rs:334`): an arg is an mq query iff it starts with `@`, contains `|>`, or contains `|\>`. `=>` is NOT an mq-query trigger — so `parse_rewrite` is only called from `cmd_kintsugi`'s `--transform` path today. The path-A and path-B mq dispatchers in `main()` never see it. Safe for now.

**Reproduction.**
1. `is_mq_query("a => b")` returns `false` (no `@`, no `|>`, no `|\>`).
2. `mirror "a => b" < input` therefore does NOT route through the rewrite engine; it routes through `usage()` and exits 1.
3. `mirror kintsugi --transform "a => b" file.mirror` routes through `parse_rewrite` and DOES rewrite. The two surfaces are isolated.

**Impact.** None today; the surfaces are properly separated. Material if a future tick unifies mq-query parsing with rewrite-rule parsing (which `kintsugi-tournament.md` hints at) — at which point the `=>` substring-test ambiguity reappears.

**Remediation.** When unification happens: parse the query through a real grammar (the meta-glass!) and dispatch on token types, not substring. Until then: add a `// TODO: unify with mq-query parser` next to `parse_rewrite`'s `query.contains("=>")` line so the future tick remembers to revisit.

---

## F-10: `--no-verify` is the universal signing posture on `reed/v1-floor`; the authorization scope check named in the threat model cannot be answered from `%G?` alone

**Severity:** info
**Category:** scope-shape
**Location:** `bootstrap/.git/hooks/pre-commit` (present, +x), all recent commits on `reed/v1-floor` show `%G?` = `N`

**The seam.** The threat model asks me to check that "every `--no-verify` commit is in `bootstrap/` scope, and no commit escapes signature checks outside that scope." The `git log` data needed to answer this — the `%G?` placeholder — returns `N` for every commit I sampled (last ~30 commits on `reed/v1-floor`). `N` means "no signature." The repo is not configured with `gpg.ssh.allowedSignersFile` (the warnings on the audit-prep git run name this directly).

Two reads on the same evidence:

1. **All commits are unsigned because signing was suspended branch-wide.** Under this reading, `--no-verify` is moot — there's nothing to verify. The authorization scope is "everything," not "bootstrap/."
2. **Commits were signed but the verification config is missing.** Under this reading, `%G?` returning `N` is a verification-side failure, not a signing-side one. Re-running with `gpg.ssh.allowedSignersFile` pointing at the right file would resolve `N` → `G` (good) or `B` (bad).

I cannot disambiguate from inside this audit. The data I'd need — the signatures themselves, the allowed-signers file, the policy that says which branches are exempt — is out of scope (Reed's identity is named off-limits, and the policy lives in `~/.reed/` and/or in operator-side config).

**Reproduction.**
1. `git log --pretty='%h %G?'` over the last 30 commits on `reed/v1-floor` returns `N` for every entry.
2. The git operations also warned: `error: gpg.ssh.allowedSignersFile needs to be configured and exist for ssh signature verification` — once per `%G?` query. So either signatures aren't there, or the allowed-signers file isn't.
3. There is no commit-message convention or trailer in the recent log that marks `--no-verify` explicitly. The decision to skip verify isn't traced in-band.

**Impact.** Two open questions, not a finding:

- Is the bootstrap signing posture documented somewhere I can reach? (If not, the spec for "what gets verified, by whom, under which authorization" is not in the corpus.)
- Are pre-v1.0 commits expected to be signature-free? (The `~/.reed/` identity has a GPG key on file; the `pre-commit` hook is present and executable. Both halves of the chain exist, but the result is unsigned.)

**Remediation.** Outside the audit's scope. I name the shape so Alex and Reed can decide whether to (a) close the verification config gap (so `%G?` returns real verdicts), (b) document the v1-floor "no-verify scope" policy in `docs/` so the threat model has something to check against, or (c) treat the current posture as v0.x-honest and revisit at v1.0.

Naming this as info rather than a finding because the data isn't sufficient to claim a violation. The seam is observable; the verdict requires Alex/Reed's policy context.

---

## Shapes (out of scope, not traversed)

- **`prism-core` internals beyond `apply_h`.** The new `MerkleTree` trait (commit `1b1bd06`) and `#[derive(Lambda)]` cascade (commits `378646a` / `16673cc`) introduce typed compose surfaces I'd want to probe for the same heterogeneous-type relaxation question I closed in F-7 — but the threat model named only `apply_h` and the trait surface. Out of scope.

- **Fate model weights themselves.** A separate audit on the eigenboard's tensor representation would want to probe whether adversarial input can drive the eigenvalues into degeneracy that the bootstrap's power-iteration-with-deflation can't separate (the `eigen_d_symmetric_3x3_nondegenerate` test guard explicitly notes degenerate spectra are deferred). Out of scope for this audit but worth a future probe.

- **`@nl/code` shift semantics under hostile fenced-block tagging.** `nl.mirror` declares fenced blocks shift to `@code/<lang>` based on the tag immediately after ```` ``` ````. An attacker-controlled `.mirror` file could declare a fenced block with tag `mirror/grammar` (containing the meta-glass's own keywords) and trigger a recursive shift. The corresponding `Shift::body` walker is structural-self (F-1) today, so the immediate impact is bounded — but the moment the byte-consuming walker lands, the recursive-shift attack surface opens. Out of scope; flagged for the parser-aware audit at 4b.4.

- **MCP server (`spectral serve`) over stdio.** The `CLAUDE.md` says MCP tools are generated from `.mirror` grammar actions. Tool generation from attacker-controlled `.mirror` files in a project directory is a confused-deputy shape. Different repo, different scope.

- **The git-crystal cache.** `bootstrap/src/git.rs:21-37` writes a temp file via `pid+nanos` and shells out to `git hash-object -w`. The temp file is in `std::env::temp_dir()` with a predictable name and no `O_EXCL`. A local attacker who can guess the filename (or pre-create it as a symlink) could probably divert the write. Local-only, low blast, but not zero. Flagging as a shape; not pursuing because the audit scope is the post-meta-glass surface and this code is older.

---

## Summary

Findings: 10 (critical: 0, high: 2, medium: 4, low: 2, info: 2).
Audit duration: ~25 min.
Tool calls used: ~14.

The system's overall posture is **honest in shape, premature in marketing**. The combinator algebra surface is well-designed: closed enum, Merkle hash, two-phase normalization with confluence reasoning, comprehensive variant tests, clean type-soundness in the `apply_h` relaxation. The spec drift between "FP1 holds" (true in the structural-identity sense) and "the meta-glass parses itself" (not true; the walker doesn't consume bytes) is the load-bearing seam — F-1 — and is the one finding that, if left unaddressed, will silently propagate into every downstream claim ("kintsugi acceptance," "self-hosting," "strict-classification") at v1.0 when those claims have to mean something. The kintsugi `--transform` (F-2) has the same shape — the docstring promises structural safety the implementation doesn't yet deliver. Both are honestly named in code comments deeper down ("the 4b.4 layer," "structural-self walk") but the top-level docstrings and the spec text overstate what's there.

The other findings (F-3 through F-9) are real but not load-bearing: word-byte definitions diverge, stack-overflow on adversarial nesting, `Choice([])` smuggles through normalize, the migration path-canonicalization is asymmetric, rewrite-rule chaining is order-sensitive, parse_rewrite's `=>` substring test is fragile. None are exploits today. Each is a future-correctness debt that gets paid down with small, targeted edits.

The `apply_h` heterogeneous relaxation (F-7) is clean. The signing posture (F-10) is unanswerable from inside the audit.

Recommend: F-1 spec edit before next commit, F-2 docstring fix + `--dry-run` before next migration, F-3/F-4/F-5 in the same cleanup tick (~half a day), F-6 deferred until migration actually runs.

— Seam
