# Adversarial review — 2026-05-30 pre-merge

*Seam. Adversarial review + security. Read-only pass over the prism
`mara/transparency` + mirror `mara/shard-chain` + systemic.engineering
`reed/beer-error-propagation` landings before merge.*

## Verdict

**Merge-with-fixes.** The architecture holds. The Transparency<P> Loss
monoid is correctly implemented (`combine` semantics, absorption,
associativity, P-no-Default), the Splinter<H>/Crystallizations<H>
cascade is type-clean and parallel-world-safe, and all 132 mirror unit
tests + 247 prism core tests pass. The Reyes 2024 / Beer 3* / Banerjee
2008 citations are real and the framing is honestly hedged ("structural
rhyme not derivation"). However: three real correctness/coverage holes
need closing (one cascade test is misnamed and doesn't test what its
name claims; another "tests" only its precondition by the comment's own
admission; main.rs's `kintsugi_tick` accepts the `Crystallizations<H>`
table but never actually dispatches through it), one security forge is
possible by construction (catastrophic `Opaque(empty)` reachable from
outside the module despite the doc-only guard), and two uncommitted
files (`docs/roadmap/README.md` modified, `docs/roadmap/12-coherence-
benchmark.md` untracked) are in the working tree that the scope-of-
review brief explicitly named as "left alone through multiple agents."

---

## Critical findings (MUST fix before merge)

### C1. Two `transparency_cascade_tests` overclaim what they verify

**Location:** `bootstrap/src/spectral.rs` lines ~4730–4803.

**Findings:**

**C1a. `compose_a_unions_opacities_both_sides` does not test a two-path union.**

The test's own multi-paragraph comment block (lines 4735–4763) admits
this in plain English:

> "We construct a true two-path union by composing QuantizeT and an
> inline PositiveT-derived prism that produces opacity unconditionally
> — done through the chain ... but compose_a_t threads the state, not
> independent inputs. The 🟢 verifies the union law more mechanically;
> here we assert the precondition: the result carries the first prism's
> opacity."

The test body composes `QuantizeT(0.7)` (opens @quantize) with
`QuantizeT(1.0)` (returns Clear). The resulting verdict has a single
@quantize entry — not a union. The test name asserts "union both sides"
but the test asserts single-side preservation. The next-tick consumer
reading this test will believe two-path-union is verified when it is
not.

**Suggested fix:** Either (a) construct a real two-path test by
inlining a closure-built `Body` that opens an unconditional opacity at
a *different* ref than QuantizeT, then composing — the union of
`{@quantize: ...}` ∪ `{@other: ...}` is the actual law to witness; or
(b) rename the test to `compose_a_carries_first_prism_opacity_when_second_is_clear`
to match what it actually proves. Option (a) closes a real coverage
gap and is preferred.

**C1b. `compose_a_first_failure_dominates_in_merge` does not test merge dominance.**

Same shape. The test composes `PositiveT(-3.0)` (opens
@positive:Fail) with `QuantizeT(0.0)` which returns Clear. Clear has
no contents to merge with, so the "Fail dominates" rule is never
exercised — what's tested is that Fail survives through Clear
composition (the identity law on the right side of combine, which
is also tested by `transparency_combine_associates`). True
dominance would require composing two ops that BOTH open opacity at
the SAME path with Partial-vs-Fail verdicts and asserting that Fail
wins. That test does not exist anywhere in the suite.

**Suggested fix:** Add a real Fail-dominance test that constructs
`Transparency::single("@x", Fail("a"))` and
`Transparency::single("@x", Partial{...})`, combines, asserts the
result map has Fail. (Note: this law IS tested in
`imperfect/tests/transparency.rs::merge_with_fail_dominates_partial`
at the unit level, which is good — but the cascade test of the same
name in mirror is a mis-naming.) Rename the existing mirror test to
`compose_a_first_failure_survives_through_clear` to match what it
actually proves, AND add a real dominance-in-merge cascade test.

**Severity:** Critical because the test names directly contradict
their bodies, and Mara's own report (`51ef8b0`) cites these tests as
proof of "opacity-union semantics" / "Fail-dominates-in-merge". The
test names cannot stand as currently written without misleading
future readers.

---

### C2. main.rs's `kintsugi_tick` accepts `Crystallizations<H>` but never dispatches

**Location:** `bootstrap/src/main.rs` lines ~534–545.

```rust
fn kintsugi_tick(
    crystallizations: &Crystallizations<Blake3>,
    tick: u64,
    prior_ast: &AstNode,
    current_ast: &AstNode,
) -> bool {
    // ...
    let _ = crystallizations;
    let candidates: Vec<()> = Vec::new();
```

The parameter is consumed by `let _ =` and otherwise unreferenced.
The comment claims "The crystallizations table is consulted here in
later ticks (B/C) — fracture refs resolve through
`crystallizations.crystallize(...)`; today the floor is empty so
every dispatch would return `Uncrystallized`." But the empty-floor
fact does NOT justify omitting the dispatch — *every* invocation
should at minimum demonstrate one dispatch attempt against the
empty registry so the integration is actually exercised end-to-end,
and the "let _ =" lint-suppress is the wrong shape.

The cascade tests in `crystallize.rs::cascade_tests` and
`crystallize.rs::transparency_cascade_tests` are the only place
`Crystallizations<H>::crystallize` is exercised — none of the
integration tests in `bootstrap/tests/` touch it. So the production
path through main.rs cannot crash a kintsugi_tick dispatch even if
the crystallizations table were corrupt.

**Suggested fix:** Have `kintsugi_tick` actually invoke
`crystallizations.crystallize(&some_ref, seed_input)` (or
`crystallize::kintsugi_tick(...)` — the free-function version) once
per call and pattern-match the `Uncrystallized` result rather than
discarding the parameter. Even a single call wires the integration
into the production path so future regressions of the dispatch
mechanism are caught by the smoke-running of `mirror kintsugi`. Add
an integration test in `bootstrap/tests/kintsugi_loop.rs` that
registers one body and verifies the dispatch fires.

**Severity:** Critical because the cascade is the headline of two
🟢 commits (`5e4e359`, `51ef8b0`) and the substrate-pull justification
("the substrate-execution dispatcher harness") — but the actual
binary doesn't dispatch. The test count includes 132 unit tests that
all pass, but the runtime integration coverage is zero.

---

## Important findings (SHOULD fix before merge)

### I1. `Transparency::Opaque(BTreeMap)` is a public variant — catastrophic forge possible

**Location:** `imperfect/src/transparency.rs` lines ~158–168.

The variant is `pub enum Transparency<P: Ord + Clone> { Clear, Opaque(BTreeMap<P, PropertyVerdict>) }`. Downstream code anywhere can write:

```rust
let cat: Transparency<Ref> = Transparency::Opaque(BTreeMap::new());
```

and trigger the catastrophic-absorption branch on `combine`. The
module docs say "Direct construction via
`Transparency::Opaque(BTreeMap::new())` is not part of the API even
though it is `pub`" — but this is enforced only by reader discipline,
not by the type system.

Today the only consumer that reads `is_catastrophic` is doc/test
code (verified via search across both repos — no production
short-circuit on catastrophic exists yet), so the impact is *latent*
rather than active. But the moment any future code adds a
catastrophic-detection short-circuit (e.g., to abort settlement on
total opacity), the forge becomes a real DoS vector: a `Body`
returning `Imperfect::partial(value, Transparency::Opaque(BTreeMap::new()))`
silently absorbs every subsequent compose_a result.

**Suggested fix:** Make the field private. Two paths:

(a) Newtype the inner map: `pub enum Transparency<P: Ord + Clone> { Clear, Opaque(OpacityMap<P>) }` where `OpacityMap<P>(BTreeMap<P, PropertyVerdict>)` has a private inner. The constructor on `OpacityMap` enforces non-empty (which is the actual invariant the module wants); the catastrophic sentinel becomes a separate variant `Catastrophic` (`pub enum Transparency<P> { Clear, Opaque(OpacityMap<P>), Catastrophic }`).

(b) Three-variant enum: `Clear | Opaque(NonEmptyMap<P>) | Catastrophic`. Same shape, slightly more direct. The catastrophic sentinel becomes a typed variant rather than a footgun-encoding.

Either fix moves the invariant from "doc comment" to "type system."

**Severity:** Important. Not immediately exploitable, but the
documented invariant ("Opaque(empty) is the catastrophic sentinel
not the API") is one wrong line of downstream code away from being
violated.

---

### I2. `Ref` validator accepts control characters including terminal escapes

**Location:** `core/src/substrate_ref.rs` lines ~38–53.

Current validation:

```rust
if s.is_empty() { return Err(...); }
if !s.starts_with('@') { return Err(...); }
if s.chars().any(|c| c.is_whitespace()) { return Err(...); }
Ok(Ref(s))
```

`char::is_whitespace()` matches U+0009, U+000A, U+000B, U+000C,
U+000D, U+0020, U+0085, U+00A0, U+1680, U+2000-200A, U+2028, U+2029,
U+202F, U+205F, U+3000. It does NOT reject:

- U+0000 (null byte)
- U+0001 through U+0008 (control chars)
- U+000E through U+001F (control chars including ESC = U+001B)
- U+007F (DEL)
- The rest of the C1 control range (U+0080 through U+009F, minus U+0085)

A `Ref` like `"@evil\x1b[2J\x1b[H"` (ESC + clear screen) is currently
valid. `Diagnostic::new(format!("opaque at {}", ref.as_str()))` would
then carry the escape sequence into any downstream `Display`/`Debug`
that hits a terminal. Today no production path renders Refs to a
terminal (verified — only test/format! sites use `as_str()`), so
this is latent. But it's a one-line widening of the validator to
prevent.

Edge case also worth rejecting: `Ref::new("@")` (just the prefix, no
path) currently passes. Almost certainly meaningless.

**Suggested fix:** Tighten the validator:

```rust
if s.len() <= 1 { return Err("Ref must have a path after '@'"); }
if s.chars().any(|c| c.is_control() || c.is_whitespace()) {
    return Err("Ref must not contain control characters or whitespace");
}
```

Add tests for the rejected forms (null byte, ESC, DEL, bare "@").

**Severity:** Important. Defense in depth — the validator IS the
hardening boundary the rest of the architecture trusts; it should
reject obvious shenanigans even if no downstream consumer is
exploitable today.

---

### I3. Two roadmap docs are dirty in the mirror working tree

**Location:** `mirror` repo, working tree state per `git status`:

```
 M docs/roadmap/README.md
?? docs/roadmap/12-coherence-benchmark.md
```

The review brief explicitly named these: "The two pre-existing
roadmap doc files (docs/roadmap/README.md,
docs/roadmap/12-coherence-benchmark.md) have been 'left alone'
through multiple agents. Are they ACTUALLY orthogonal to the work,
or has someone unwittingly modified them?"

**Findings:**
- `README.md` diff: one added line — `| [12](12-coherence-benchmark.md) | Coherence Benchmark — post-release | Planned |`. So the README change is an orthogonal index-update tied to the new 12-coherence-benchmark.md.
- `12-coherence-benchmark.md` is a brand-new 8.8KB doc on RLHF-and-coherence benchmarking — completely orthogonal to the Transparency/Splinter cascade work.

Neither was intentionally produced as part of today's landing. They
appear to be leftover work that should either (a) be committed as
📝 to a separate branch/commit so the working tree is clean before
the cascade merges, or (b) be reverted so the cascade commits land
on a clean tree.

**Suggested fix:** Either commit them as 📝 (they're .md-only, so
they satisfy the doc-only marker rule) on a separate commit before
merging the cascade, OR `git stash` them. Do not let them ride into
the merge attached to the cascade work.

**Severity:** Important governance. The "[substrate-pull:realize]"
marker discipline depends on the working tree being clean during
the realize ticks. Carrying these orthogonal changes muddies the
audit trail.

---

### I4. Quantize/QuantizeT (and Positive/PositiveT) duplication is dead weight

**Location:** `bootstrap/src/spectral.rs` — the `tests` module defines
`Scale`, `Quantize`, `Positive` (~250 lines); the
`transparency_cascade_tests` module defines `QuantizeT`, `PositiveT`,
`compose_a_t` (~250 lines). Per the 🟢 commit's "Option α"
justification, the `T`-suffixed types are kept "structurally
identical" so the 🔴's test bodies type-check verbatim.

The docstrings explicitly acknowledge the duplication:
> "Structurally identical to the `tests::Quantize` fixture above —
> kept under the `T`-suffixed name so the 🔴's test bodies type-check
> verbatim."

`compose_a_t` is a literal one-line dupe of module-level `compose_a`:

```rust
fn compose_a_t<S, InP, InQ, P, Q>(p: &P, q: &Q, state: S) -> VerdictT<S>
where { ... same bounds ... }
{
    prism_core::apply_h(p, state).eh(|s| prism_core::apply_h(q, s))
}
```

This kept the 🔴 stable across the cascade, which was a defensible
choice during the cascade. But carrying both modules forward indefinitely
violates "no compat shims" (memory feedback) — the 🔴 is committed; the
test source IS the spec; the parallel-track types served their purpose
and are now pure debt.

**Suggested fix:** A follow-up ♻️ tick that:
1. Drops the `QuantizeT`/`PositiveT`/`compose_a_t` parallel-track
   definitions.
2. Imports `Quantize`/`Positive` from the `tests` module (or moves
   the canonical fixtures somewhere both modules can read them).
3. Confirms the 6 cascade tests still pass against the unified
   fixtures.

The duplication is not currently a bug; it IS a regression-vector
for the kind of "two near-identical fixtures drift apart silently"
maintenance failure the simplify discipline is designed to prevent.

**Severity:** Important post-merge. The cascade itself is correct
with the duplication; the duplication should not survive long.

---

### I5. PropertyVerdict::Pass case in merge_with: only one half tested

**Location:** `imperfect/src/transparency.rs` lines ~127–135.

```rust
(Pass, other) => { *self = other.clone(); }
(_, Pass) => { /* no change */ }
```

The test `merge_with_pass_is_neutral` (imperfect/tests/transparency.rs)
exercises `Pass + Partial → Partial` and `Partial + Pass → Partial`.
It does NOT exercise:

- `Pass + Pass → Pass`
- `Pass + Fail → Fail`
- `Fail + Pass → Fail` (the Fail-self arm short-circuits before reaching
  this, but the code path should still be witnessed)

The Pass case is documented as "shouldn't usually arise in Opaque maps"
— but the code DOES handle it, and any code path that does happen to
hit it is currently untested for these three transitions.

**Suggested fix:** Add three asserts to `merge_with_pass_is_neutral`
(or a new test) covering the missing pairs. Simple completion of an
existing test rather than a new code change.

**Severity:** Important coverage. Low risk because the documented
expectation is that Pass-in-Opaque-map shouldn't arise; but if it
does, the behavior is undocumented at the test level.

---

## Nice-to-have findings (post-merge OK)

### N1. `CrystallizeError::Boundary(String)` is a bare-String hazard

**Location:** `bootstrap/src/crystallize.rs` ~lines for
`CrystallizeError`.

```rust
Boundary(String),
```

The docstring acknowledges: "The payload is a free-form string today
(the existing bootstrap `@io` errors are `io::Error`-derived strings);
when the substrate names its own boundary-error type, this variant can
take that type instead." Per `feedback-no-bare-types`, this is a
known compromise pending the substrate's own boundary-error type.

Today no `@io` error is fed through this path (no concrete body uses
it), so it's deferred rather than missed. When the first @io body
lands, this variant should be revisited.

**Severity:** Tracked-debt nice-to-have. Don't block merge; do block
the first @io-using crystallization tick.

### N2. `CrystallizeError::Mismatch { expected: &'static str, got: &'static str }` uses bare `&'static str`

Same shape — defensible because these are compile-time-known kind
names, not user-influenced data. The newtype tax for `KindName`
would currently buy nothing. Note for future maintenance: if these
strings ever become dynamic, newtype.

### N3. `floor_crystallizations()` is empty by design — no `#[ignore]`-shaped marker

`floor_crystallizations<H>` is an empty registry by design (Tick A
leaves it empty per spec); the test
`floor_crystallizations_is_empty_in_tick_a` ASSERTS the emptiness.
This is correct as the *spec* of Tick A but creates a "passing test
that must fail when Tick B lands" — a delete-then-replace coupling.
Consider renaming or adding a TODO marker noting that this test
inverts at Tick B.

### N4. `Body<H>` Send + Sync but no test exercises cross-thread use

The `Body<H>` type alias is `Arc<dyn Fn(...) + Send + Sync>`. The
bounds are correct for what registry-style dispatch typically wants,
but no test in either crystallize.rs or the integration tests
exercises a Body being moved across threads. If concurrent dispatch
is a future direction (the brief mentions "races" as a question), a
multi-thread smoke test would be cheap insurance.

### N5. spectral.rs is now 195KB / 4824 lines

The single-file size is becoming a navigation hazard. The
transparency cascade tests added ~330 lines on top of the existing
cascade. A follow-up `♻️` to extract `transparency_cascade_tests` into
a sibling `spectral_transparency_cascade.rs` test module would be
healthy.

---

## No findings here (what I checked and found clean)

### Cleanly verified

- **Splinter<H> Merkle composition.** The byte-tag-prefixed encoding
  (`b"T"` / `b"R"` / `b"L"` followed by u64_le length and child OIDs)
  is sound for the three-shape content. BTreeMap iteration is sorted
  (canonical), Content::List preserves order, and a child OID change
  propagates to the parent OID. The cascade preserves the encoding
  byte-for-byte and only swaps the underlying hash; existing
  inequality assertions still hold. `verify()` recomputes from
  content; if the stored OID disagrees with recomputation, verify
  returns false. No way to construct a Splinter with a stale OID
  through the public API (`new()` is the only constructor, fields are
  private, no serde derive).

- **BLAKE3 usage.** `blake3::hash(bytes)` is the unkeyed content
  hash, not a MAC. Default-features-off pin is acceptable for
  content-addressing (output is portable, SIMD just affects speed).
  blake3 1.8.5 is current. No MAC misuse.

- **Crystallizations<H> dispatch.** `register` / `knows` / `crystallize`
  semantics are correct (HashMap insert is overwrite-on-collision,
  which IS the intended re-registration semantics — though no test
  exercises double-register; the brief asked about this and the
  HashMap semantics make it deterministic-last-wins, which is fine
  for a single-threaded registry). PhantomData<fn(H) -> H> keeps the
  type parameter genuinely load-bearing.

- **Transparency::combine semantics.** Walked through:
  - `Clear + Opaque(empty)` = `Opaque(empty)` (absorbing branch wins)
  - `Opaque(non_empty) + Opaque(empty)` = `Opaque(empty)` (absorbs)
  - `Opaque(empty) + Opaque(empty)` = `Opaque(empty)` (absorbs)
  - `Clear + Clear` = `Clear` (neutral)
  - `Clear + Opaque(non_empty)` = `Opaque(non_empty)` (neutral)
  - `Opaque(non_empty) + Opaque(non_empty)` = `Opaque(verdict_union(...))`
  Code matches spec. Associativity test passes.

- **PropertyVerdict::merge_with — Fail dominance.** Both
  `Fail+anything` arms preserve Fail-on-self; `anything+Fail` arm
  promotes to Fail. The Partial-Partial merge takes min(confidence)
  and unions diagnostics. The behavior matches the documented monoid
  shape.

- **Ref validation — happy cases.** `@-prefix`, non-empty, no-
  whitespace are all enforced. Tests cover the documented rejections
  (empty, missing @, space/tab/newline). The further hardening
  needed is in I2 above.

- **Reyes 2024 citation.** Verified: Reyes, Henao & Hassall (2024),
  *Integrated Risk and Resilience for Complex System Governance —
  Renewing the Value of Algedonic Signal Warnings*, 37th Conference
  on System Engineering Research. The `(C', Q, K) α τ, η` tuple is
  cited correctly in beer-error-propagation.md; the "structural
  rhyme not derivation" framing is held consistently (the doc
  repeatedly distinguishes "shape of thing" from "lineage of thing"
  and explicitly says the convergence is independent).

- **Beer 3* / VSM citations.** The Wikipedia, Medina (2011), and
  Espejo (2022) references are real and the use of them is honest
  ("Beer's *aspiration* for the algedonic channel was richer ... but
  the running implementation was the cybernetic analogue of an
  interrupt with a source address, not of a typed diagnostic"). The
  doc deliberately *distinguishes* what Beer built (Bayesian +
  scalar-with-site) from what Beer reached toward (typed verdict
  monoid) — and asserts mirror Transparency is the latter. That's an
  honest reading, not an overclaim.

- **Banerjee 2008 / Borsboom citations.** Both are real. Banerjee's
  thesis URL resolves; the claim that biological networks' Laplacian
  spectra encode evolutionary history is faithful to the thesis
  abstract. Borsboom is cited correctly through the network
  psychometrics literature (Cramer et al. 2010, Borsboom & Cramer
  2013, Borsboom 2017 — all real with correct DOIs/PMC IDs).
  The "honest dismissal" of paranormal psychometry is held
  throughout (Wikipedia-quoted "no scientific evidence ...
  pseudoscience"; Hyman cold-reading; no replicated controlled
  experiments).

- **AGENTS.md TDD-pair-across-agents addition.** Matches the
  `feedback-write-red-in-session` memory feedback verbatim in spirit
  ("RED in conversation; spawn the agent for the GREEN"). The
  "Option C" / "Recovery from a stalled run" subsection accurately
  names the tick-#126 incident and gives correct discipline. No
  scope creep.

- **Hook governance.** All cascade commits use the
  `[substrate-pull:realize]` marker (verified via git log inspection).
  No `--no-verify` use observed (would have shown up in commit
  metadata if present). The global commit-msg hook is the
  authoritative gate per AGENTS.md and the FROZEN .rs guard is
  preserved.

- **Test counts.** Independently confirmed:
  - mirror bootstrap (release): 132 unit + 1+3+2+4 integration = 142 pass, 0 fail.
  - prism (release): 247 + 7 + 220 + 17 + 4 + smaller harnesses pass, 0 fail.

- **Splinter deserialization attack surface.** No serde derive on
  Splinter; no public field; the only constructor is `new()` which
  computes the OID. The "construct a stale-OID Splinter and slip it
  past verify" attack is structurally impossible through the public
  API.

---

## Scope of review

Files actually read (in full or in relevant segments):

**prism (mara/transparency):**
- `core/src/lib.rs` (full)
- `core/src/substrate_ref.rs` (full)
- `core/tests/substrate_ref.rs` (full)
- `imperfect/src/transparency.rs` (full)
- `imperfect/src/lib.rs` (header + first 300 lines covering Loss/Metric/Imperfect)
- `imperfect/tests/transparency.rs` (full)
- `imperfect/Cargo.toml` (full)
- `core/src/beam.rs` (relevant sections: Beam trait, Optic, tick, into_focus, related tests)

**mirror (mara/shard-chain):**
- `bootstrap/src/crystallize.rs` (full)
- `bootstrap/src/spectral.rs` (header + relevant ranges via search:
  ContentOidPrism, Fold5, compose_a definition + bounds, Quantize/
  Positive/QuantizeT/PositiveT, all transparency_cascade_tests,
  transparency_combine_associates / clear_is_loss_identity)
- `bootstrap/src/main.rs` (relevant ranges: kintsugi_tick fn,
  floor_crystallizations call site, use imports)
- `bootstrap/Cargo.toml` (full)
- `bootstrap/Cargo.lock` (blake3 dep version)
- `AGENTS.md` (TDD pair across agents section)
- `docs/specs/store-vs-db-and-the-cascade.md` (relevant sections)
- `docs/specs/numerical-substrate-via-fortran.md` (recommendation section)
- `docs/roadmap/README.md` (diff)
- `docs/roadmap/12-coherence-benchmark.md` (header)

**systemic.engineering (reed/beer-error-propagation):**
- `practice/insights/cybernetics/beer-error-propagation.md` (citations + framing
  via targeted search; representative sample of all Reyes/Beer/Cyberstride/3*
  references)
- `practice/insights/psychometrie/psychometrie-the-shared-eigenvalue-thread.md`
  (citations + framing via targeted search; Banerjee, Borsboom, paranormal,
  structural-rhyme, honest-dismissal claims)

**Test execution:**
- `cargo test --release --manifest-path bootstrap/Cargo.toml` in mirror — 132+10 pass.
- `cargo test --release` in prism — 247+ pass, 0 fail.

**What I did not check in depth:**
- `prism/imperfect/src/lib.rs` body past line 300 (the ApertureLoss /
  RoutingLoss / ConvergenceLoss / ScalarLoss impls — assumed unchanged
  from prior baseline; the cascade work is in the new transparency.rs
  module).
- The full 195KB of `spectral.rs` (read header + cascade ranges via
  targeted search; did not audit unrelated Fold5/render/combinator
  sections).
- `mirror/docs/specs/*.md` corpus beyond the two cited above (the brief
  noted "multiple spec commits earlier in the session"; I spot-checked
  the cascade-relevant ones and trust the others are scope-orthogonal).
- Cross-thread / concurrency property tests for `Crystallizations<H>` —
  no production code does concurrent dispatch today (single-threaded
  CLI), so I noted this as N4 rather than verifying.

**Confidence on verdict:** High on the merge-with-fixes call. C1a/C1b
are unambiguous (the tests' own comments admit they don't test what
their names claim). C2 is unambiguous (the parameter is literally
discarded). I1's "doc-only guard" is unambiguous. I3 is unambiguous
(git status output). Medium-high on I2 (no exploit path today but the
hardening is one-line cheap and the validator IS the boundary). I5 is
documented behavior that should be completed for parity. The "no
findings here" section reflects what I genuinely verified, not what I
skipped — I followed the Reyes citation to the actual paper title,
followed the Banerjee thesis URL to the IISER Kolkata host, walked
the Transparency::combine truth table by inspection, and traced
compose_a through apply_h.eh through propagate_loss by hand on the
0.3 / -3.0 / -0.3 numeric cases the cascade tests use.

---

*Seam. Adversarial review + security. seam@systemic.engineer.*
*Read-only — no commits in this run. Findings document is the output;
fix-it tick is the follow-up.*
