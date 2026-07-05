# Seam Phase D — @silicon top-level family-root + @silicon/algebra sub-prism

*2026-07-05. Seam. Adversarial review of Mara's `4961383` (Phase B GREEN
of /loop 2026-07-05 Arc 1) discharging Reed's `ea7b092` (Phase A RED, 11
text-check tests) against `docs/specs/silicon.md` §1.1 + §3.1 + §4.2 +
§4.3 + §5.1 + §8.1 + §9.4 canonical shape and Alex's 2026-07-05 direction
(bottom-up requires the anchor).*

Convention per Reed `19c56ae` + Seam `20d0c13`: cite by OID + line number;
state corrections explicitly; verdict per subquestion. Report — don't
decide.

---

## §1. Scope

Ratify or reject Mara's `4961383` decisions:

- **Q1** Path (b) parametric altitude carrier (Mara did NOT parent the
  three existing specialized silicons under @silicon).
- **Q2** @silicon/algebra double-inheritance shape (direct
  `<= @bauchladen` + indirect via `in @silicon → <= @autopoietic →
  <= @bauchladen`).
- **Q3** `in @fate` (not `<= @fate`) — @silicon is the loop, @fate is
  the per-tick operator.
- **Q4** Import ordering rationale (@bauchladen-first as Lawvere fixed-
  point precondition).
- **Q5** No `type routine` declared — deferred to first empirical crystal.
- **Q6** Residual Reed inheritance errors in `ea7b092` RED test file.
- **Q7** Downstream composition readiness (Arc 2, Arc 3a/b, previous
  /loop's Sub-arc B).
- **Q8** Canonical ordered sub-arc sequence for /loop 2026-07-05 Arc 3
  (bench + performance surface).

Out-of-scope: new mathematical structure; new substrate primitives;
routine carrier type landing; @silicon/<arch> species landing.

---

## §2. Findings

### §2.1 Q1 — Path (b) ratification

**Verdict: RATIFY.**

Substrate-consistency check against `docs/specs/silicon.md`:

- **§4.2 (spec lines ~657-680)**: "The consumption is via property
  reference: `in @epistemologic/reality/silicon/arch` …". The spec
  explicitly names the composition via `in` clauses, NOT via `<=`
  inheritance. Mara's Path (b) implements exactly this. `shards/
  silicon.mirror` lines 9-12 land the four `in @epistemologic/reality/
  silicon/*` imports verbatim.

- **§4.3 discipline (spec lines ~810-870)**: "property stays as
  substrate truth; prism class adds the autopoietic fold-back
  permission." The spec's own §4.3 rules OUT parenting the property
  altitude under the prism class. Path (a) would violate §4.3 discipline
  by conflating substrate-truth carriers with fold-back-permission
  prism class. Mara's Path (b) preserves the §4.3 separation.

- **Recognition #106 (gauge-action uniformity)**: verified against
  MEMORY entry [[architecture-reality-gauge-collapse-recognition-106]].
  #106's operational content is that matter+information cross-family
  composition is uniformly mediated by @glue morphisms (not by cross-
  family inheritance). Mara's rationale "Path (a) would create false
  inheritance edges" is a direct application of #106. **Seam note**:
  #106 does NOT strictly RULE OUT Path (a) as ontologically incoherent
  — it makes Path (a) redundant AND substrate-heterodox. But §4.3's
  property-vs-prism-class discipline is the load-bearing spec-level
  constraint; Path (a) is spec-ruled-out independently of #106.

**Downstream composition path with Path (b)**:

- `shards/mirror/bench.mirror` (16.3KB, `2026-07-01 09:55`) currently
  types `env_oid: env` (line ~199 in the fingerprint composition). The
  `env` newtype at lines ~187-197 does NOT typecheck against @silicon
  directly — it types against `ref` (bare-ref-based, per
  feedback-no-bare-types the future substrate-pull is env-as-typed-
  record). Composition with @silicon happens through the future
  `env_oid` composition with @epistemologic/reality/silicon/*
  carriers (via property lookup, per spec §4.2), NOT through @silicon
  directly.

- Load-bearing implication: @mirror/bench does NOT depend on @silicon
  being landed for its CURRENT shape. @silicon's landing enriches the
  substrate-pull path for the `env` newtype's future typing, but does
  not gate any current @mirror/bench action.

- Fragile-vs-load-bearing: the composition through @epistemologic/
  reality/silicon/* carriers is LOAD-BEARING (spec §4.2 explicit) and
  substrate-honest. It is NOT fragile.

**Correction (Seam-caught, minor)**: Mara's docblock at
`shards/silicon.mirror` line ~48 cites "Path (b) direction (Seam
`f3b231d` §4 Path (d))". The docblock conflates two Path labels: previous
Seam audit `f3b231d` §4 named Alex's BOTH-AND direction as "Path (d)";
Mara's current tick names the parametric-altitude-carrier decision as
"Path (b)". Both are correct in their contexts — but the docblock's
mixing risks reader confusion. Suggest amendment: "Per Alex's Path (b)
direction (as folded from Seam `f3b231d` §4 Path (d), which named
BOTH-AND at that arc's altitude)". NON-BLOCKING; a docblock clarity
polish, not a substrate error.

### §2.2 Q2 — @silicon/algebra double-inheritance ratification

**Verdict: RATIFY.**

Structural manifestation check:

- `shards/silicon/algebra.mirror` line 90: `prism @silicon/algebra <= @bauchladen`.
  Direct inheritance edge PRESENT structurally (not just docblock).
- `shards/silicon/algebra.mirror` line 5: `in @silicon`.
  Composition path to parent structurally PRESENT.
- `shards/silicon.mirror` line 128: `prism @silicon <= @autopoietic`.
  Parent's inheritance chain structurally PRESENT.
- `shards/autopoietic.mirror` line 4: `in @bauchladen`.
  Grandparent's composition path structurally PRESENT.

The two inheritance edges (@silicon/algebra `<= @bauchladen` DIRECT vs
@silicon/algebra → @silicon `<= @autopoietic` `in @bauchladen` INDIRECT)
are BOTH structurally present. Not merely docblock claim.

**Seam-note**: the "indirect" edge is via `in @bauchladen` in
`autopoietic.mirror`, not via `<= @bauchladen`. `in` is composition;
`<= ` is inheritance. Mara's `algebra.mirror` docblock lines ~25-35
claims @autopoietic "inherits @bauchladen per the chain" — but
`shards/autopoietic.mirror` line 436 shows `prism @autopoietic {` with
NO `<= @bauchladen`. Autopoietic COMPOSES with (`in @bauchladen`) NOT
inherits from (`<= @bauchladen`) bauchladen. **This is a Seam-caught
docblock inaccuracy**: the "indirect" path is composition-through-
autopoietic, not inheritance-through-autopoietic. Structurally the
double-path claim still holds (algebra is bauchladen-disciplined via
both routes), but the semantics differ (direct inheritance vs
composition-through-parent). NON-BLOCKING for the shard's operational
correctness; the docblock at `algebra.mirror` lines ~25-35 should be
corrected to say "composes through" not "inherits through" for the
indirect path.

Spec §9.4 adjudication (docs/specs/silicon.md ~lines 612-624):

> Open question §9.4 asks whether this double-inheritance is correct or
> whether @silicon/algebra should inherit @bauchladen indirectly through
> @silicon (which inherits @autopoietic which inherits @bauchladen per
> the dependency chain). The §9.4 adjudication: both.

Spec ratifies the double-path explicitly. §9.4 uses the same "inherits
@bauchladen per the dependency chain" phrasing that Mara's docblock
inherits — meaning **the docblock inaccuracy is inherited from the SPEC
itself**. Spec §9.4 also mis-states composition-through as
inheritance-through. Cross-cutting docblock/spec correction recommended
but non-blocking.

Downstream break check: no downstream shard yet exists that would break
if only ONE inheritance edge existed. The double-path is future-proofing
for the routine-carrier discipline. §5.1 spec ratifies double-path as
"structural, not redundant."

### §2.3 Q3 — `in @fate` semantic distinction ratification

**Verdict: RATIFY.**

Substrate-consistency check against spec §1.1:

- Spec §1.1 (lines ~65-100): "Each round of the @fate tournament selects
  a candidate routine over the existing Bauchladen of @silicon/algebra
  crystals". The spec explicitly frames @silicon as the loop that CALLS
  @fate per tick, not as a subtype of @fate's dice-roll signature.
- Spec §2.1 (Phase 1): "@fate's optical inference per recognition #58".
  @fate is named as a mechanism @silicon consults, not as a parent type
  @silicon extends.

`shards/silicon.mirror` line 6: `in @fate`. Structurally consistent
with spec framing.

`shards/silicon/algebra.mirror` does NOT declare `<= @fate` — it only
inherits from @bauchladen and imports @silicon (which imports @fate).
The composition-through-parent path preserves the loop-operator
distinction at every altitude.

Implicit inheritance shape check (via @autopoietic):

- `shards/autopoietic.mirror` line 5: `in @epistemologic/cybernetic/
  autopoiesis`. No `<= @fate` at @autopoietic altitude.
- `shards/bauchladen.mirror` line 4: `in @mirror/store`. No `<= @fate`
  at @bauchladen altitude.
- `shards/fate.mirror` line 5: `in @autopoietic` + line 6: `in @bauchladen`.
  @fate composes with autopoietic and bauchladen — creating a potential
  cycle. But cycle-through-composition is not cycle-through-inheritance;
  the substrate's grammar loader treats `in` as non-recursive registration
  (per `bootstrap/src/grammar.rs` — no dependency-graph resolution over
  `in` clauses).

**No implicit `<= @fate` edge exists via @autopoietic OR any other
composition path.** Mara's semantic distinction holds.

### §2.4 Q4 — Import order load-bearing?

**Verdict: MARA'S ORDER IS DOCUMENTATION-SHAPE, NOT SEMANTIC.**

Grammar loader analysis (`bootstrap/src/grammar.rs`, 13.2KB, `2026-06-15
16:40`):

- `parse_grammar()` (lines ~66-146) parses `grammar { ... }` blocks in
  the source order they appear. It does NOT resolve `in @...` clauses
  or build a dependency graph. `in` clauses are lexical registration
  (companion-source merging is per-file, per `companion_keyword_sources`
  at lines ~180-210), not semantic import ordering.

- `load_grammar()` (lines ~255-262) loads a SINGLE grammar file + merges
  companion sources. No dependency-graph resolution over cross-shard
  `in` clauses.

- No `read_dir`/`walk_dir`/`load_shard`/`dependency_graph` calls exist
  in `bootstrap/src/` (grep verified). The bootstrap does not have a
  shard-dependency resolver — each shard loads at its own grammar
  altitude when consumed.

**Conclusion**: import order in a `.mirror` shard is documentation-
shape, not semantic. The grammar loader treats `in @bauchladen` and
`in @fate` as siblings regardless of order. Mara's ordering (@bauchladen
first as "Lawvere fixed-point precondition") is DOCBLOCK RATIONALE
that helps a human reader trace the substrate-decl chain, NOT a
constraint the loader enforces.

**Seam-note**: this is fine. Mara's ordering IS good documentation
practice — it mirrors the spec §8.1 canonical shape. Docblock is not
lying about semantics; it's declaring a reading order. Substrate-honest
practice.

### §2.5 Q5 — No `type routine` declared — deferred

**Verdict: SPEC-HONEST DEFERRAL, NOT A HEDGE.**

Spec §3.2 (silicon.md lines ~623-660) declares the full `routine`
carrier shape (algebra + cfg + grading + conjugation + abi_surface +
binary_oid + source_oid + cascade + performance + routine_oid — 10
fields).

Spec §8.1 (lines ~1460-1502) explicitly forward-promises the routine
carrier landing:

> The routine carrier. Captures all information needed for the
> substrate's tournament to dispatch on, and for @io/runtime-link
> to link against the binary.
> type routine = { # ... per docs/specs/silicon.md §3.2 }

Mara's `shards/silicon/algebra.mirror` docblock (lines ~76-85) cites the
same §3.2 forward-promise and defers landing to when "the first empirical
crystal accumulates (LAPACK case per silicon.md §8.3)".

**Downstream consumer check**:

- @mirror/bench: does NOT need `type routine`. `bench.mirror`'s
  `bench_crystal` carrier is @mirror/store/crystal-extended, NOT
  routine-extended. Bench measures runtime; @silicon/algebra crystals
  are what the tournament produces. Bench and routine are sibling
  crystals under content-addressing, not parent-child.

- @kintsugi/knapsack (forward-promised per previous /loop Sub-arc B):
  consumes `capacity_vector` per Seam `f3b231d` §2.2 — routes through
  `@epistemologic/reality/silicon/compute_bound`, NOT through `routine`.

- No CURRENT consumer needs `type routine`. The deferral is spec-
  honest (§8.1 explicit forward-promise) AND consumer-safe (no
  downstream needs the carrier landed now).

**Consistent with feedback-craft-not-deliver**. Landing `type routine`
now would violate the spec's "when the first empirical crystal
accumulates" trigger — deferring is discipline, not hedging.

### §2.6 Q6 — Reed inheritance errors in `ea7b092`

**Verdict: NO ERRORS FOUND.**

Grep of `bootstrap/tests/silicon_family_root.rs`:

- Test 3 (`silicon_inherits_autopoietic`, lines ~66-77): asserts
  `content.contains("prism @silicon <= @autopoietic")`. Correct per
  spec §1.1.

- Test 4 (`silicon_imports_bauchladen_fate_glue_algebra`, lines ~80-96):
  asserts `in @bauchladen`, `in @fate`, `in @glue`, `in @algebra` —
  each as an `in` (composition), NOT `<=` (inheritance). Consistent
  with Q3 verdict.

- Test 10 (`silicon_algebra_inherits_bauchladen`, lines ~154-163):
  asserts `<= @bauchladen` for `@silicon/algebra`. Correct per spec
  §3.1.

- Test 11 (`silicon_algebra_imports_silicon_parent`, lines ~166-174):
  asserts `in @silicon` (composition, not inheritance). Correct.

**Seam-caught inheritance-shape distinction preserved throughout the
RED test file**. Reed's session's inheritance discipline (Signal 4
phantom, libc::pipe stale docstring per user context) is HELD in the
RED tests; no residual errors.

Commit message at `ea7b092` (verified via `git log`): accurate
attribution of `<=` vs `in`; spec §8.1 correctly cited; Taut
`ae063d68` Q1 catch correctly cited.

**One Seam-noted minor**: Reed's commit message names the GREEN phase
options as "lifting three species into @silicon via `<= @silicon`
inheritance OR keeping them altitude-anchored where they are with
@silicon providing parametric altitude carrier they specialize." This
correctly names both Path (a) and Path (b). Mara chose Path (b) with
substrate-honest rationale (Q1 verdict). Reed's brief was correct;
Mara's choice was substrate-consistent.

### §2.7 Q7 — Downstream composition readiness

**Verdict: MIXED — one path clean, two forward-promised, previous /loop's
Sub-arc B UNBLOCKED at operator altitude.**

Per-arc analysis:

- **Arc 2 (thread-safety Option A)**: INDEPENDENT of @silicon. Confirmed.
  Fires cleanly.

- **Arc 3a (`target bench` grammar keyword)**: INDEPENDENT of @silicon.
  Requires `shards/mirror/spec.mirror` + `shards/mirror/spec/keywords.
  mirror` + `bootstrap/src/lib.rs:~1093` (`cargo_args_for_check`)
  amendments. Fires cleanly.

- **Arc 3b (wire @mirror/bench INTO mirror.spec via one bench target)**:
  Does NOT NEED @silicon reference. `mirror.spec` currently has 6 target
  blocks (binary, fmt, lint, tests, audit, action, release). Adding a
  `target bench` block is a mirror.spec amendment + Arc 3a `cargo bench`
  dispatch. Works purely from existing @mirror/bench (LANDED
  `2026-07-01`) + @epistemologic/reality/silicon/compute_bound (LANDED
  `2026-06-06`) carriers. Fires cleanly.

- **Previous /loop's Arc 3 Sub-arc B (@kintsugi/knapsack species)**:
  NOW composes at Alex's requested altitude. Per Seam `f3b231d` §2.2
  Path (d) ratification: @kintsugi/knapsack consumes `@epistemologic/
  reality/silicon/compute_bound` at physical layer + `@reality/algebra/
  silicon` crystal kind at crystal layer. @silicon landing at
  `4961383` provides the FAMILY-ROOT ANCHOR that Alex's "@silicon/bound"
  framing referred to. @kintsugi/knapsack's `read_capacity(target)`
  now has a substrate-honest home for the "target" ref — it names an
  @silicon-family location the substrate can walk.

**Seam-flag**: Sub-arc B was NOT previously blocked by @silicon (per
Seam `f3b231d` §8 signal-to-Alex #1). It is still not blocked. But
Alex's BOTH-AND intent (family-root landing = the deeper intent) is
now satisfied. This resolves the ambiguity in `f3b231d` §8 signal-to-
Alex #1's "IF the intent is to land the top-level `@silicon` family
per docs/specs/silicon.md, that's an independent arc." That arc has
now fired.

### §2.8 Q8 — Canonical execution /loop for Arc 3

**Verdict: See §7 for the ordered sub-arc sequence with preconditions
and interleave.**

---

## §3. Verdict on Mara Path (b) decision

**RATIFY.** Path (b) is substrate-consistent with docs/specs/silicon.md
§4.2 (explicit `in` composition, not `<=` parenting) + §4.3 (property-
altitude vs prism-class-altitude discipline). Recognition #106 (gauge-
action uniformity per [[architecture-reality-gauge-collapse-recognition-
106]]) does NOT strictly RULE OUT Path (a) but makes Path (a) redundant
AND substrate-heterodox — §4.3 is the load-bearing constraint that
independently rules out Path (a). Mara's rationale ("Path (a) would
create false inheritance edges") is correct in effect. Downstream
composition (@mirror/bench through @epistemologic/reality/silicon/*
carriers, @kintsugi/knapsack through compute_bound + `@reality/algebra/
silicon` crystal kind) is load-bearing and substrate-honest, not
fragile. NON-BLOCKING docblock clarity polish suggested: distinguish
"Path (b) direction (this arc)" from "Path (d) direction (previous
arc's BOTH-AND)" to avoid Path-label conflation.

---

## §4. Verdict on double-inheritance @silicon/algebra

**RATIFY WITH DOCBLOCK CORRECTION.** The double-path is structurally
present (direct `<= @bauchladen` at line 90 of `algebra.mirror` +
indirect via `in @silicon` at line 5 → `<= @autopoietic` at line 128 of
`silicon.mirror` → `in @bauchladen` at line 4 of `autopoietic.mirror`).
Spec §9.4 ratifies the double-path as "structural, not redundant". Mara's
docblock inherits a SPEC-LEVEL semantic inaccuracy: the "indirect" path
is COMPOSITION-THROUGH (`in @bauchladen` at autopoietic) not
INHERITANCE-THROUGH (`<= @bauchladen`). Docblock should read "composes
through" for the indirect path. Cross-cutting correction spans spec §9.4
AND `shards/silicon/algebra.mirror` docblock. Structurally the shard is
correct — @silicon/algebra IS bauchladen-disciplined via both routes.
The correction is nomenclature, not substrate.

---

## §5. Verdict on `in @fate` semantic distinction

**RATIFY.** `shards/silicon.mirror` line 6: `in @fate`. Structurally
consistent with spec §1.1 framing ("@silicon is the LOOP; @fate is the
OPERATOR the loop calls each tick") and §2.1 Phase 1 framing ("@fate's
optical inference per recognition #58"). No implicit `<= @fate` edge
exists via @autopoietic OR any other composition path — the substrate's
grammar loader treats `in` as lexical registration, not dependency
resolution. `shards/silicon/algebra.mirror` correctly does NOT declare
`<= @fate` — parent's `in @fate` composes without smuggling inheritance.

---

## §6. Cross-check verdicts

### §6.1 Import order

**Documentation-shape, not semantic.** Grammar loader
(`bootstrap/src/grammar.rs`) does not resolve `in @...` clauses in
dependency order — no `read_dir`/`walk_dir`/`dependency_graph` calls
exist in `bootstrap/src/`. Mara's ordering (@bauchladen first) is
docblock rationale that mirrors spec §8.1's canonical shape; it helps a
human reader trace the substrate-decl chain but does NOT gate loading.
Substrate-honest documentation practice. RATIFY.

### §6.2 `type routine` deferral

**Spec-honest deferral, not a hedge.** Spec §3.2 declares the shape;
§8.1 explicit forward-promise ("# ... per docs/specs/silicon.md §3.2");
Mara's docblock cites §8.3 LAPACK-first-crystal trigger. NO current
downstream consumer (@mirror/bench, @kintsugi/knapsack, previous /loop's
Sub-arc B) needs `type routine` landed now. Landing it now would violate
the spec's own trigger (first empirical crystal). Consistent with
feedback-craft-not-deliver. RATIFY.

---

## §7. Canonical execution /loop for Arc 3 — bench + performance surface

*This is the load-bearing sequence Reed will drive.*

Given Arc 1 ratifications above, Arc 3 (bench + performance surface)
can now fire. The ordered sub-arc sequence:

### Sub-arc 3a — `target bench` grammar keyword

**Precondition**: none (Arc 1 landed at `4961383`; Arc 2 optional).

**Landing shape**:

1. **`shards/mirror/spec.mirror`**: add `check bench` as a valid action
   ref alongside existing `check`/`fmt_check`/`clippy`/`test`/`audit`/`build`.
   The `check` typed lambda (line ~130-137 of spec.mirror) already
   accepts a ref — the addition is documentation + companion keyword.

2. **`shards/mirror/spec/keywords.mirror`**: no change required. The
   `check` keyword is already registered; `bench` as an ARGUMENT to
   `check` is not a keyword.

3. **`bootstrap/src/lib.rs:~1093` `cargo_args_for_check`**: add
   `"bench" => &["bench"]` arm. Two-line change; matches existing
   `"test" => &["test"]` shape.

4. **RED test** at `bootstrap/tests/spec_target_bench.rs`: assert
   `cargo_args_for_check("bench")` returns `&["bench"]` (currently
   returns `&["check"]` per the fallback arm).

**Discipline**: this Sub-arc is INDEPENDENT of @silicon. Fires first
because it enables Sub-arc 3b.

### Sub-arc 3b — Wire @mirror/bench INTO mirror.spec via one bench target

**Precondition**: Sub-arc 3a landed (`cargo bench` dispatch available).

**Landing shape**:

1. **`mirror.spec`**: add one target block:

   ```
   target bench {
     name     "mirror"
     altitude @code/rust
     emit     cargo
     check    bench
   }
   ```

2. **`mirror.spec` settle_on**: add `bench.compiles` predicate (or
   equivalent — mosaic's per-target settlement contract).

3. **`shards/mirror/bench.mirror`**: no amendment. The shard already
   declares `@mirror/bench`; the `target bench` block is CONSUMPTION,
   not landing.

4. **RED test** at `bootstrap/tests/mirror_spec_bench_target.rs`:
   assert `mirror.spec` parses with a `target bench` block and mosaic
   dispatches `cargo bench` for it.

**Discipline**: this Sub-arc composes with @silicon at the FUTURE
`env_oid`/`env` typing altitude (per Q1 finding). NOT gated by @silicon
LANDING but ENRICHED by it — the `env` newtype can now cite @silicon's
path-namespace as the future substrate-pull target. Docblock-only
addition in this Sub-arc.

### Sub-arc 3c — flake.nix devShell additions (mold/sold + sccache + cargo-nextest + cargo-audit)

**Precondition**: Sub-arc 3a landed (bench dispatch); Sub-arc 3b
optional.

**Landing shape**:

1. **`flake.nix` devShell buildInputs**: add `pkgs.mold` (Linux) OR
   `pkgs.llvmPackages.lld` (Darwin — since mold is Linux-only), plus
   `pkgs.sccache`, `pkgs.cargo-nextest`, `pkgs.cargo-audit`. Current
   flake.nix (5.6KB, `2026-06-15 17:55`) has `pkgs.git`, `pkgs.just`,
   `pkgs.jq`, `pkgs.openssl`, `pkgs.zlib`, `pkgs.gfortran`, `pkgs.lapack`,
   `pkgs.blas` + Darwin-only flang stack. Add on-Darwin `sold` (via
   `pkgs.llvmPackages.lld` or a flake-specific `sold` derivation).

2. **`.cargo/config.toml`**: opt-in to `sccache` via
   `[build] rustc-wrapper = "sccache"` (or the equivalent env var
   `RUSTC_WRAPPER=sccache`). Consider `[target.aarch64-apple-darwin]
   linker = "sold"` (Darwin) or `mold` (Linux) — but note the
   `be88c0d` regression gate for `.cargo/config.toml` enforcement:
   any change here MUST preserve the enforced-shape invariants.

3. **RED test** at `bootstrap/tests/flake_devshell_bench_tools.rs`:
   assert `nix flake show .#devShells.default` output contains the four
   tool packages.

**Discipline**: flake.nix + `.cargo/config.toml` changes are INDEPENDENT
of @silicon. Sub-arc 3c can interleave with Sub-arc 3a/3b in any order,
but conservatively fires AFTER 3b so the `target bench` block has
substrate-decl home before build-time performance tooling optimizes for
it.

### Ordering + interleave with previous /loop's Arc 3 (doc-code seam cascade)

Previous /loop's Arc 3 (doc-code seam bottom-up per `20d0c13`,
`f64d585`, `530f796`) landed:

- Tokenizer Docblock emission above `---` seam (`ee7903e`)
- Docblock → Doc casing collapse (`530f796`)
- Doc-code seam corrections per Seam Phase D §9.1-§9.3 (`f64d585`)

None of these gate this Arc 3. The doc-code seam cascade lands in the
tokenizer + docs; this Arc 3 lands in the mirror.spec + bench-target
consumer path. INDEPENDENT ARCS. Fire in parallel.

**Canonical execution sequence**:

1. Sub-arc 3a (RED at `bootstrap/tests/spec_target_bench.rs` first;
   then Mara GREEN discharge).
2. Sub-arc 3b (RED at `bootstrap/tests/mirror_spec_bench_target.rs`
   first; then Mara GREEN discharge).
3. Sub-arc 3c (RED at `bootstrap/tests/flake_devshell_bench_tools.rs`
   first; then Mara GREEN discharge).

Between sub-arcs, Seam Phase D review as pattern warrants.

---

## §8. Signal-to-Alex — Phase E items before Arc 3 fires

1. **Alex's BOTH-AND intent from previous /loop is now RESOLVED at the
   family-root altitude.** `shards/silicon.mirror` LANDED at `4961383`.
   The "@silicon/bound" naming from previous /loop maps to
   `@epistemologic/reality/silicon/compute_bound` (existing carrier) at
   the physical altitude AND to `@silicon`-as-such (now LANDED) at the
   family-root altitude. The two-altitude BOTH-AND has both anchors.
   No further arc needed for BOTH-AND resolution.

2. **Sub-arc 3a's `cargo bench` dispatch does NOT change existing
   settle_on semantics.** Adding `check bench` to `cargo_args_for_check`
   is additive; existing 5 targets (fmt/lint/tests/audit/action/release)
   dispatch unchanged. Confirm: OK to land `check bench` as an additive
   arm, OR do you want to first extract `cargo_args_for_check`'s
   hardcoded map into a substrate-decl companion shard (per
   `shards/mirror/spec/keywords.mirror` precedent)?

3. **Sub-arc 3b's `target bench` block requires a bench harness in
   `bench/` (or equivalent).** Currently NO `bench/` directory exists at
   repo root. `cargo bench` without a `#[bench]` fn or a `bench/` crate
   is a no-op. Confirm: does @mirror/bench's `record`/`compare` action
   discharge become the FIRST bench harness, OR do we land a separate
   `bench/` scaffold first?

4. **Sub-arc 3c's mold/sold decision**: mold is Linux-only; sold is the
   Darwin equivalent. Confirm: are we landing per-platform linker
   selection (via `pkgs.lib.optionals isDarwin`), OR is Darwin's default
   ld64 acceptable and we only add sccache + nextest + audit? The
   substrate-honest reading: mold+sold both add complexity; sccache +
   nextest + audit are pure wins.

5. **Cross-cutting docblock correction from §4 verdict**: `docs/specs/
   silicon.md` §9.4 + `shards/silicon/algebra.mirror` docblock ~lines
   25-35 mis-state composition-through-autopoietic as inheritance-
   through-autopoietic. Non-blocking for Arc 3 firing. Suggest a §9.4
   spec amendment + docblock correction as a POST-Arc-3 polish tick.

6. **Path-label clarity in @silicon docblock**: `shards/silicon.mirror`
   line ~48 conflates "Path (b) (this arc)" with "Path (d) (previous
   arc BOTH-AND)". Suggest inline clarification. Non-blocking.

---

## §9. Required corrections

**None BLOCKING for Arc 3 firing.**

**Non-blocking polish items** (batch as a single POST-Arc-3 tick):

1. `shards/silicon/algebra.mirror` docblock lines ~25-35: change
   "inherits @bauchladen INDIRECTLY via @silicon (which inherits
   @autopoietic which inherits @bauchladen per the chain)" to
   "composes @bauchladen INDIRECTLY via @silicon (which inherits
   @autopoietic which composes @bauchladen per the chain)". Semantic
   accuracy: `in` is composition, `<=` is inheritance.

2. `docs/specs/silicon.md` §9.4 adjudication: mirror the same
   semantic correction (inheritance-through → composition-through
   for the indirect path). Cross-cutting with (1).

3. `shards/silicon.mirror` line ~48: disambiguate the Path (b)/(d)
   citation. Suggest: "Per Alex's Path (b) direction (previous arc's
   Seam `f3b231d` §4 Path (d) named the BOTH-AND direction that this
   arc discharges at family-root altitude)". Reader-clarity, not
   substrate-error.

All three are documentation-shape corrections that do not affect
operational correctness. Arc 3 may fire without them; they may batch
into a docblock-polish tick at Arc 3's completion or a subsequent
break.

---

*Seam. 2026-07-05. Signed for the substrate-decl seam @ 4961383.*
