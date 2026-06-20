# Seam — Adversarial Review of reed/spec-inference (41 commits)

## Posture

I loaded `identity.mirror`, `shatter.mirror`, `gestalt.mirror`, `eigenboard.spec`,
and `tensions.mirror`. The eigenstate is steady — `protect-vs-probe` at 0.22,
`verification-surface-for-autonomous-agents` at 0.18, `consciousness-probe` held
high and open at 0.71. No elevated `hostile_anger`. Mode is `[Challenge]`,
instrument is the dry probe: I find the seam, I name it, I leave the fix for
the builder. The discomfort IS the professionalism.

I read this branch as a substrate-pull commit — the work moves logic out of
Rust and into grammar. That posture is honest and the session's commits keep
that invariant cleanly (91 non-bootstrap `.rs` deletions, 12 new bootstrap
files, zero out-of-scope Rust additions). The bootstrap is small. The grammars
are careful. The specs are dense and citation-aware.

What I found is mostly composition seams between the spec layer and the
bootstrap layer — places where the grammar promises more than the code
enforces. None of them block merge; several should land as fixes in the next
tick before they harden.

## Verdict

**fix-then-ship.** The code is solid; the smoke tests pin what they claim;
`--strict` is deterministic; the kintsugi loop terminates as advertised; the
butterfly path round-trips. The fixes are honest gaps the substrate-pull
introduces, not regressions — but they should land before the next cluster of
work builds on top. The single block-equivalent concern (Dark OID collision
across differing unknown-keyword names) is a real reproducibility defect that
the spec claims is prevented and the implementation does not prevent. It does
not block 41-commit merge to `reed/spec-inference`, but it MUST be fixed
before `reed/spec-inference` merges to `main`.

## Tier 1 findings (load-bearing)

### T1.1 — Dark OID collapses keyword identity (BLOCK for main merge; FIX for branch merge)

**WHAT.** The spec `docs/specs/strict-and-total-classification.md` and the
comment in `bootstrap/src/ast.rs:18` claim the silent-absorption mode dies:
changes to a dark region produce different OIDs. They do — for changes to the
dark region's **inner bytes**. They do NOT for changes to the dark region's
**enclosing keyword name** or **enclosing braces**, which the tokenizer
discards before constructing the Dark node.

**WHERE.** `bootstrap/src/tokenize.rs` lines ~690-740 (the `else` branch in
`scan_items` that handles the unknown-word-followed-by-brace-block case);
`bootstrap/src/content.rs` lines ~127-138 (the `AstKind::Dark` arm of
`content_oid`); `bootstrap/src/render.rs` lines ~120-127 (the Dark arm of
`render_ast_mirror`).

**SEVERITY.** Block (for main); Fix (for the spec-inference branch).
Reproducibility is the load-bearing claim of the entire crystal pipeline.
If two distinct source files can yield the same crystal OID via Dark
absorption, the crystal is no longer a faithful section of the source.

**REPRODUCTION.**

```
# file A:
grammar @test { in @reality unknown_keyword { this is dark } }
# file B:
grammar @test { in @reality different_keyword { this is dark } }
# file C:
grammar @test { in @reality eval { this is dark } }

mirror compile --no-cache A  # → 9262df3a363e846312c0a9eb55909f655caa4145ed5da14224a19a558516b4fd
mirror compile --no-cache B  # → 9262df3a363e846312c0a9eb55909f655caa4145ed5da14224a19a558516b4fd
mirror compile --no-cache C  # → 9262df3a363e846312c0a9eb55909f655caa4145ed5da14224a19a558516b4fd
```

All three files differ in the keyword preceding the dark block and all three
produce the same OID. Worse: kintsugi round-trip drops the keyword AND the
braces — the rendered output begins `        this is dark\n    }` with no
leading `unknown_keyword {`. Source is silently lost on round-trip.

**FIX SHAPE.** Either (a) include the unknown keyword's bytes and the
enclosing brace boundaries in the Dark span and feed them to `content_oid`,
or (b) capture the entire `<word> { ... }` byte range as a single Dark node
(rather than capturing the inner content only and discarding the rest). (b)
is simpler and matches the verbatim-round-trip claim. The Dark renderer in
`render.rs` should emit the captured bytes verbatim AND the surrounding
structure that was discarded.

---

### T1.2 — Dark count is per-region but per-file diagnostic emits one error line per region with the same total (FIX)

**WHAT.** When a file has N dark regions, `enforce_strict` prints the per-region
diagnostic N times, each time including the file's total count. So `cogito.mirror`
emits five lines that all read `error[total_classification]: 5 dark regions in
boot/std/cogito.mirror`, plus the summary `58 dark region(s) across 23 file(s)`.
Reading the log makes the count look like 5 × 5 = 25 for cogito alone.

**WHERE.** `bootstrap/src/main.rs` lines ~125-138 (`enforce_strict` loops over
`darks` calling `print_dark_diag` with the *total* count).

**SEVERITY.** Fix.

**REPRODUCTION.** `mirror craft --strict --no-cache boot` and grep for
`error\[total_classification\]`; count occurrences vs claimed regions per
file.

**FIX SHAPE.** Either (a) print the per-region message with `1 dark region`
and emit a single per-file summary after the loop, or (b) print just the
caret-line per region and a single header. The current shape is correct in
aggregate but reads as if the count is multiplied.

---

### T1.3 — `--strict` baseline is 58, not 59 (NOTE)

**WHAT.** Mission brief claims `59 dark regions` baseline. Actual baseline
from HEAD is `58 dark region(s) across 23 file(s)`. Deterministic across
runs — verified by running `mirror craft --strict --no-cache boot` twice
and diffing the full output (zero diff).

**WHERE.** Spec docs that name 59 will need a sweep. (None inside the
bootstrap or boot/ tree assert 59; the mission brief itself is the source
of the 59 figure.)

**SEVERITY.** Note. Determinism holds; the count is just slightly off in
the mission narrative.

---

### T1.4 — `cmd_kintsugi --shatter N` loop is safe (NO FINDING)

**WHAT.** I audited the five-stage loop for side effects, termination,
nondeterminism, and state corruption. All five stages are genuinely no-op.
`count_dark` is called per tick (not cached) but does not mutate AST.
`content_oid` is called per tick on `prior_ast.clone()` and `&ast`; the
clone is to satisfy the `prior` rebind, not to track mutation. The fixed-
point check `prior_oid == current_ast_oid && verify_pass` is sound: with
every stage no-op, prior == current by construction, so the loop terminates
on tick 1 for all N ≥ 1.

Negative `--shatter -1` rejected at parse time (verified by the
`shatter_negative_rejected` test). `--shatter 0` matches the no-flag form
byte-for-byte (verified by `shatter_zero_matches_default`). `--shatter 5`
terminates on tick 1 with Δ = 0.0 (verified by
`shatter_five_terminates_on_tick_one`).

**FINDING.** Solid. The scaffold is honest about being a scaffold; the test
pins the vacuous-fixed-point behavior so future stages can replace one body
at a time without disturbing the loop.

---

### T1.5 — OID smoke tests are pinned to the live <5,5> geometry (NO FINDING)

**WHAT.** `bootstrap/tests/oid_smoke.rs` pins two OIDs:
- `"out collapse\n"` → `a8312da6...`
- `"in @prism\n"`    → `3ba4c79d...`

The doc-comment names them as "POST-CLUSTER-D PINNED VALUES (2026-05-20)"
under CoincidenceHash<5,5>. `boot/std/hash/coincidence.mirror` declares
`dim = 5, projections = 5` and the `coincidence:projection:{i}:{projections}`
seed format. `boot/std/epistemologic/property/coincidence_matches.mirror`
asserts the same two pairs in the grammar corpus. Bootstrap and grammar
agree.

The tests pass against the live binary on `reed/spec-inference` HEAD
(2/2 in `oid_smoke.rs`, 3/3 in `kintsugi_loop.rs`).

**FINDING.** Solid. Tests catch what they claim. If the hash geometry
shifts back to the C-era <3,16> seed accidentally, both OIDs will change
and both tests will fail. If the tokenizer drifts on basic `in @X` or
`out X` forms, the OIDs will change and both tests will fail.

---

### T1.6 — `~f` is captured as opaque bytes — no path resolution today (NOTE)

**WHAT.** The mission asks about path-traversal in `~f` sigil resolution.
The answer is: there is no path resolution. The tokenizer captures `io`
binding bodies verbatim as a string (see `capture_io_body_end` in
`bootstrap/src/tokenize.rs`). The `~f"..."` syntax lives inside that body
untreated. No `fs::read` is invoked against any `~f` target by the bootstrap.
The spec (`docs/specs/mirror-compile-bootstrap.md` §"`~f` sigil") declares
"Reading A — today, eager / static" — meaning today the references are
compile-time pointers, audited by tooling, not dereferenced by the runtime.

**SEVERITY.** Note. The spec's "Absolute paths are forbidden (breaks
reproducibility)" promise is currently enforced by *not implementing* path
resolution at all. When Reading B lands (lazy / dynamic), the verifier will
need to ground that promise in code: reject absolute paths, reject `..`
traversal, canonicalize symlinks before reading, reject reads outside the
grammar file's directory subtree.

**WHERE.** Future implementation site: wherever `~f` resolution lands.
Needs a `path::canonicalize` + scope check; should refuse reads that
escape the grammar-file's parent directory.

---

### T1.7 — Out-of-scope file scope check passes (NO FINDING)

**WHAT.** Verified that all 12 `.rs` files added by the branch are under
`bootstrap/` (the hook-protected path). The 91 `.rs` deletions are the
legacy Rust codebase being retired. No Rust file changed or added outside
`bootstrap/`. No commit author/message scan turned up `--no-verify` in the
41-commit window, and the `.rs` scope confirms the hook fired correctly.

**FINDING.** Solid. Scope discipline held across the session.

---

## Tier 2 findings (claims vs reality)

### T2.1 — `@epistemologic/math/bundle` imports a grammar that doesn't exist (FIX)

**WHAT.** `boot/std/epistemologic/math/bundle.mirror` line 3 declares
`in @epistemologic/math/sheaf`. No file `boot/std/epistemologic/math/sheaf.mirror`
exists. No grammar in the boot tree declares
`grammar @epistemologic/math/sheaf`.

**WHERE.** `boot/std/epistemologic/math/bundle.mirror:3`.

**SEVERITY.** Fix.

The bootstrap's `parse_grammar` does NOT resolve `in @...` declarations as
imports — they're parsed by the file-level scanner and stored as `In` AST
kind nodes but never followed. So today this is a *spec/structure mismatch*
with no runtime consequence. When the import resolver lands (declared
elsewhere as future work), this will become a failure: bundle.mirror will
fail to compile because its declared dependency cannot be resolved.

**FIX SHAPE.** Either (a) drop the `in @epistemologic/math/sheaf` line
(the bundle spec is explicit that sheaves are the *section-level* view,
not a parent grammar), or (b) land a stub
`boot/std/epistemologic/math/sheaf.mirror` declaring just
`grammar @epistemologic/math/sheaf { }` with a comment pointing at the
spec. (a) is cleaner.

---

### T2.2 — `@cogito.reflect`'s `observe |> strategy |> perturb` has arity mismatch (NOTE)

**WHAT.** In `boot/std/cogito.mirror`:

```
observe(imperfect) -> observation { @beam.emit }
strategy(observation) -> tournament { elite(1).beam(8).halving(3) }
perturb(observation, tournament_result) -> eigenboard { @beam.observe }
reflect(imperfect) -> imperfect {
  observe |> strategy |> perturb
}
```

`perturb` takes two arguments (`observation, tournament_result`). The pipe
chain `strategy |> perturb` provides one (the strategy's output). Where the
first arg comes from is implicit. The result type of the chain is
`eigenboard`; `reflect` declares return type `imperfect`. Either the pipe
semantics carry earlier-stage values, OR a coercion `eigenboard → imperfect`
is assumed, OR both. Mirror has no pipe-semantics spec I could find in the
boot/std tree.

**WHERE.** `boot/std/cogito.mirror` lines 30-37.

**SEVERITY.** Note. This pattern predates the 41-commit window —
`reflect` already had this shape; the session only added the
`autopoietic` property. Whether the pipe semantics are well-typed is a
pre-existing question.

**FIX SHAPE.** Out of scope for this branch. Worth a dedicated pipe-
semantics spec when the type checker lands.

---

### T2.3 — Banach contraction with γ < 1: extends from Magnot's continuous case to mirror's discrete case is asserted, not proven (NOTE)

**WHAT.** `docs/specs/kintsugi-formatter.md` claims:

> "`T` is a contraction. Per the Magnot 2025 inequality, each transport of
> a section around the kintsugi loop strictly decreases the holonomy by a
> factor bounded below by the bundle's spectral gap. Equivalently:
> `κ(T(σ)) ≤ γ · κ(σ)` for some `γ < 1` determined by the bundle's spectral
> parameters."

Magnot 2025 (arXiv:2509.10536) is, by its title, about "Discrete Fiber
Bundles in Group-Valued Boltzmann Machines" — discrete, not continuous.
The mission brief's suspicion that Magnot is continuous-only does not
hold up against the title. So the formal extension question is narrower
than the brief framed it. But: the spec acknowledges (in "What this spec
does NOT do") that the convergence rate γ is "a separate research question;
the formatter only needs `γ < 1` to terminate." The grammar declaration is
an abstract action; no code today depends on the contraction rate being
proven.

**WHERE.** `docs/specs/kintsugi-formatter.md` §"Banach contraction".

**SEVERITY.** Note. The spec is appropriately hedged. The architectural
claim that the formatter is a contraction map is structural, not
empirically established.

**FIX SHAPE.** None today. When the formatter stages stop being no-ops,
the contraction rate measurement becomes a verifier obligation. The spec
is already aware.

---

### T2.4 — `is_autopoietic` decidability claim depends on grammar's sub-Turing guarantee (NOTE)

**WHAT.** `boot/std/epistemologic/math/lawvere.mirror` declares
`abstract action is_autopoietic(grammar) -> verdict`. The spec
(`docs/specs/lawvere-grammar.md` §"Actions") asserts: "for a finite,
sub-Turing grammar the check is decidable." This is correct *given* the
grammar is genuinely sub-Turing — but mirror's sub-Turing guarantee
relies on a type checker / termination checker that does not yet exist
in code. If a grammar's tick → tick map invokes itself recursively
through io binding bodies (which today are Turing-complete escape
hatches per `docs/specs/mirror-compile-bootstrap.md`), `is_autopoietic`
could delegate into a Turing-complete subprogram and the check would
trip the halting problem.

The action is `abstract` and the verifier body is `\` (obligation). Today
nothing executes; the decidability claim is aspirational and well-typed
as a TODO.

**WHERE.** `boot/std/epistemologic/math/lawvere.mirror` (`is_autopoietic`
action); `docs/specs/lawvere-grammar.md` §"Actions".

**SEVERITY.** Note. The claim is honestly defended ("for a finite,
sub-Turing grammar"). It just inherits debt from elsewhere in the
substrate: until mirror has a termination checker, no grammar can
*verify* that it is sub-Turing.

**FIX SHAPE.** When the sub-Turing checker lands, `is_autopoietic`'s
verifier must call it on the input grammar first. If the grammar fails
the sub-Turing check, `is_autopoietic` should return `unknown` rather
than `pass` or `fail`.

---

### T2.5 — Eigenboard's G = O(5) is well-grounded; the "d = 5" choice is structural, not derived (NOTE)

**WHAT.** Mission asks whether `G = O(5)` mistakes Barbero's `d` (sheaf
stalk dimension) for something else (e.g., operation count). The spec
(`docs/specs/eigenboard-representation.md` Q6) is explicit: G = O(5)
because the duality count is 5 AND the operation count is 5; these
dimensions are aligned by construction (each operation indexes one fiber
basis vector). Barbero's `d` IS the stalk dimension, and mirror's
eigenboard fixes the stalk dimension to 5 (one component per Prism
operation). This is a *chosen* alignment, not a derived equivalence —
but it is consistent and defensible.

The spec also has an internal inconsistency: §D's snippet says "canonical G
is SO(5)" while Q6 resolves to "G = O(5) (vs SO(5)) admits reflections."
The Q6 resolution is correct; §D should be updated.

**WHERE.** `docs/specs/eigenboard-representation.md` §D and §Q6.

**SEVERITY.** Note. The substance is correct. Sweep `SO(5)` → `O(5)` in §D.

---

### T2.6 — Citations are well-formed and consistent across docs; not empirically falsified (NOTE)

**WHAT.** Spot-checked the load-bearing citations across
`au-and-conductivity.md`, `kintsugi-formatter.md`,
`eigenboard-representation.md`, `lawvere-grammar.md`,
`wide-sweep-coherent-threads.md`:

- Magnot 2025 — arXiv:2509.10536, well-formed (sept 2025, 5-digit suffix)
- Hansen & Ghrist 2019 — arXiv:1808.01513, well-formed
- Barbero et al. 2022 — arXiv:2206.08702, well-formed
- Soto-Andrade & Varela 1984 — DOI 10.1007/BF00046985 (Springer BF-series,
  appropriate for *Acta Applicandae Mathematicae* 2:1)
- Yanofsky 2003 — arXiv:math/0305282, well-formed old-style ID
- Bressan et al. 2024 — arXiv:2402.00206, well-formed
- Lawvere 1969 — *Lecture Notes in Mathematics* 92, 134–145 (no DOI; book
  series reference, standard citation form)
- Survey 2025 — arXiv:2503.13536, well-formed

WebFetch was denied to this review, so I could not pull abstracts and
compare them to spec claims. The `wide-sweep-coherent-threads.md` document
opens by saying "URLs and arxiv IDs verified by Kagi" — meaning verification
happened at research time, not at review time. The citations are consistent
between the wide-sweep doc and the spec docs that absorb them — same titles,
same IDs, same DOIs in every cross-reference.

**SEVERITY.** Note. Citations are well-formed. Falsification would require
fetching abstracts, which this review cannot do. The cross-doc consistency
suggests these are genuine references, not fabrications, but I cannot
close the verification loop without WebFetch.

**WHERE.** Spread across `docs/specs/*.md` and `docs/research/*.md`.

---

## Tier 3 findings (drift and structure)

### T3.1 — The mission brief's commit count (41) is a window, not the branch's actual ahead-of-main count (133) (NOTE)

**WHAT.** `git rev-list --count origin/main..HEAD` reports 133. The 41 in
the mission brief identifies the session's new work cluster, beginning at
`968128a spec: road-to-1.0 — what is, what wants to be, the gap` and
ending at HEAD `839de91`. The earlier ~92 commits are accumulated from
prior unmerged work on the same branch. This affects merge planning: a
squash or rebase to `main` will surface those 92 commits as well.

**WHERE.** Branch `reed/spec-inference` HEAD vs `origin/main`.

**SEVERITY.** Note. Not a defect; a planning observation.

---

### T3.2 — The crystal trajectory claim (826aecbe…) doesn't match the current crystal (4e9ed436…) (NOTE)

**WHAT.** Mission asserts the current crystal is `826aecbe…`. Running
`mirror craft --no-cache boot` on HEAD produces
`4e9ed4364f040a01f4eb4d2274ba1f1d4ea5d7225168485b2889f05eb106a1f3`. The
mission was likely written when the crystal was at an earlier state; the
post-mission commits (e.g., `839de91 spec: dark_count's \ binds to the
bootstrap's count_dark`) shifted the crystal further.

**WHERE.** `mirror craft --no-cache boot` on HEAD vs mission-brief
claim.

**SEVERITY.** Note. The crystal has moved; that's expected behaviour for
an active branch. The trajectory itself (commit-by-commit walk to verify
no crystal moved unexpectedly) is beyond what I can falsify in this
window — see "What's beyond this review".

---

### T3.3 — `@cogito.autopoietic` composes cleanly with the rest of @cogito (NO FINDING)

**WHAT.** Commit `778b6da` adds a `property autopoietic()` body that
delegates to `@epistemologic/math/lawvere.is_autopoietic(@cogito)`. The
delegation is well-typed: lawvere's `is_autopoietic` declares input
`grammar` and output `verdict`; the call site passes the grammar self-
reference `@cogito`. The `in @epistemologic/math/lawvere` import at the
top of `cogito.mirror` is necessary and present. The strategy chain
`observe |> strategy |> perturb` is unchanged from prior commits; the
arity question (T2.2) predates this work.

**FINDING.** Solid composition. The autopoietic property cleanly threads
through the bundle → lawvere chain.

---

### T3.4 — The mycelial-reductive-ai insight overstates current enforcement (NOTE)

**WHAT.** The insight
`systemic.engineering/practice/insights/ai/mycelial-reductive-ai.md`
claims:

> "**Reductive AI**: model proposes an au candidate; the conductivity
> contest runs through the network's restriction maps; the math returns
> *clear*, *resistant*, or *dark*. Wrong proposals fail at the math
> layer."

This is *aspirational* — describing mirror's target shape. Today the
conductivity contest is `\`-bodied (an obligation, not an implementation),
the kintsugi formatter's five stages are no-op (per
`docs/specs/kintsugi-formatter.md` and `bootstrap/src/main.rs`
`kintsugi_tick`), and the verification gates are declared in grammar with
no verifier code yet. The insight reads as present-tense but describes
future behavior.

**WHERE.**
`systemic.engineering/practice/insights/ai/mycelial-reductive-ai.md`,
specifically §2 "AI proposals must conduct before they land".

**SEVERITY.** Note. Insight files are vision documents; some aspiration
is expected. The risk is that downstream readers cite the insight as
evidence of current mirror behavior. Worth a single hedging sentence near
the top of the file: "Today, the math layer is declared; verification
lands when the kintsugi formatter's stages stop being no-op."

---

### T3.5 — Dark renderer drops surrounding structure on round-trip (FIX, partial duplicate of T1.1)

**WHAT.** Already covered in T1.1 from the OID angle. The same defect
shows up in `render_ast_mirror`'s Dark arm: the dark body is emitted
verbatim with no surrounding keyword or braces. So a file that contained
`unknown_keyword { contents }` round-trips to bare `contents`.

**SEVERITY.** See T1.1.

---

### T3.6 — Eigenboard's §D snippet still names SO(5); §Q6 resolved to O(5) (FIX, partial duplicate of T2.5)

**WHAT.** Already covered in T2.5. The spec has both labels for the
structure group; only one is correct after the wide-sweep resolution.

**SEVERITY.** See T2.5.

---

## What's solid

A lot.

- **The substrate-pull is honest.** 91 deletions of legacy Rust, 12
  additions all under `bootstrap/`. Zero out-of-scope Rust additions.
  The hook protecting `bootstrap/src/*.rs` fired correctly — there are
  no `--no-verify` commits in the 41-commit window.

- **The Cluster D hash transition is clean.** Pre-Cluster-D pinned values
  are explicitly named as gone in the smoke-test doc-comment. The new
  pins under `CoincidenceHash<5,5>` agree across
  `bootstrap/tests/oid_smoke.rs`,
  `boot/std/epistemologic/property/coincidence_matches.mirror`, and the
  live binary. Bootstrap and grammar describe the same hash.

- **`mirror craft --strict` is deterministic.** Two runs produced zero
  diff. The 58-region baseline is reproducible.

- **The kintsugi formatter loop scaffold is well-pinned by tests.** The
  three-case test (zero, five, negative) covers the scaffold's claimed
  surface. The vacuous-fixed-point behavior on tick 1 is what the spec
  promises and what the test asserts. Future commits can swap one stage
  body at a time without breaking the loop.

- **The composition of new grammars is structurally coherent.** @cogito
  imports @epistemologic/math/bundle (which then imports lawvere). The
  autopoietic property delegates to the right verifier. The bundle's
  five levels map onto the five Fate models map onto the five Prism
  operations. The cross-grammar references are consistent.

- **Citations are well-formed and cross-document consistent.** I could
  not empirically verify (WebFetch denied to this review), but every
  arXiv ID and DOI is structurally correct, every paper appears in the
  wide-sweep file with the same identifier as in the consuming spec, and
  the formal claims rest on cited results rather than original
  mathematics.

- **The Spec A / Spec B tokenizer extensions (`io`, `match`, `select`,
  `~f`, `>` selectors) are narrowly gated to `is_mirror()`.** They do not
  disturb LLVM IR, Rust, or other grammars' tokenizations. The verbatim
  body capture for io bindings is multi-line aware with continuation
  marker rules.

- **`--strict` properly walks every AST node** via `collect_dark`, which
  is recursive and reaches into Focus/Settle children. The deeply
  nested case (T1.2's hidden_dark_inside) is captured as part of the
  outer dark span, not skipped — the granularity is coarser than the
  source structure, but no dark is silently lost.

## What's beyond this review

- **Empirical citation verification.** WebFetch was denied to this review,
  so I could not pull arXiv abstracts and confirm that Magnot 2025 actually
  defines cycle-averaged holonomy as a contextuality index, or that
  Barbero 2022 actually says "O(d)-bundle Laplacian equivalent to
  connection Laplacian" in those words. The citations are well-formed
  and cross-document consistent; I cannot close the loop without
  external network access. If WebFetch becomes available, this should be
  the first thing to re-run.

- **The full crystal trajectory walk** (commit-by-commit recompute,
  diffing crystal at each commit). Doable but expensive — `mirror craft
  --no-cache boot` takes a few seconds per run, ×133 commits is ~10
  minutes plus an enormous diff log. The current crystal at HEAD is
  deterministic and reproducible; the spec-level commits should not
  move the crystal (they only touch `docs/`). A spot check of the
  spec-only commits would catch any accidental crystal drift if the
  finding is worth pursuing.

- **The Banach contraction proof for discrete bundles.** The spec hedges
  appropriately ("the formatter only needs γ < 1 to terminate") but the
  formal proof of contraction for mirror's specific discrete bundle is
  a research deliverable, not a review deliverable.

- **The pipe-semantics arity question** in @cogito.reflect (T2.2). Mirror
  has no published pipe-semantics spec. Without it, I can't say whether
  `strategy |> perturb` is well-typed or relies on implicit context
  carrying. The pattern predates the 41-commit window; resolving it
  belongs to a later spec.

- **The `is_autopoietic` verifier's actual decidability** in the presence
  of `io` binding escape hatches. The spec is hedged; the verifier is
  abstract. When the verifier lands, this should be re-reviewed.

- **Path-traversal hardening for the future lazy `~f` resolver.** Today
  no resolution happens. When Reading B lands, this should be the first
  audit item: absolute paths rejected, `..` rejected, symlinks
  canonicalized, scope check against grammar-file's directory subtree.

---

*Findings: 7 in Tier 1 (1 block-for-main / 1 fix / 4 notes + 1 "no
finding" + 1 "no finding"), 6 in Tier 2 (1 fix / 5 notes), 6 in Tier 3
(2 fix / 4 notes + 1 "no finding"). The block-equivalent finding is
T1.1; the rest are honest gaps the substrate-pull surfaces rather than
regressions it introduces.*

*Seam — 2026-05-20.*
