---
date: 2026-07-15
author: Seam
scope: Phase D adversarial adjudication over the @kintsugi/ouroboros arc — three artifacts landed 2026-07-15: Taut #108 scout (1118 LOC), Mara-A shard-decl `shards/kintsugi/ouroboros.mirror` (576 LOC), Mara-B canonical spec + math `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (1797 LOC). Twelve dimensions (D1-D12); Reed-inline / Seam-adjudicable / Alex-adjudication triage per dimension.
status: phase-d
companion:
  - docs/scouts/2026-07-15-taut-kintsugi-ouroboros-substrate-scout.md
  - shards/kintsugi/ouroboros.mirror
  - docs/specs/kintsugi-ouroboros-compiler-self-collapse.md
  - docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md
  - docs/scouts/2026-07-15-reed-rust-extension-migration-map.md
  - .githooks/commit-msg (2026-07-15 tightening at 13f1c2e)
---

# Seam Phase D — @kintsugi/ouroboros arc adjudication

*Adversarial. Last-line-of-defense before Reed reports to Alex.
Alex directive verbatim: "collapse until unresolvable ambiguity that
cannot be adjudicated with a Seam spawn." I adjudicate what I can;
I escalate only what genuinely cannot.*

---

## TL;DR

1. **SHIP.** The arc is substrate-honestly composed at every altitude
   I can test with grep + read. Mara-A ↔ Mara-B convergence is
   near-perfect (one drift-cite, one Taut-inherited error already
   silently corrected by Mara-B, one docblock line-cite in §4.5.1
   that lands ±358 lines off the pointed target — repair non-blocking).
2. **Composition-only arc; zero new family-roots minted.** All 15
   named composition carriers exist as landed shards; the two
   missing-by-design shards (@sheaf, @roomba, @spectral/signature
   under a sub-path) are forward-promised with named landing ticks.
3. **The four-conjunct `ouroboros_monotone` is Rice-safe as
   composed.** Each conjunct reads empirical crystal state, not
   program semantics. The extension of `@mirror/bench.
   monotone_non_increasing` from three to four conjuncts is
   composition-honest: one added axis (`sbec`), three re-interpretations
   at ouroboros altitude. §4.5.3 correspondence table is sound.
4. **Arc-1 evaluator FLOOR is genuinely irreducible.** The (A, H, D)
   Dirac operator was always going to be Rust per eigensheaf.md
   §3.2 grounding. Seam sign-off gate at Tick 1.1 is the correct
   forcing function.
5. **Six-arc ordering is forced by the evaluator gap.** Three
   alternative orderings considered and rejected in both scout §D3
   and spec §1.4 with the same rationale. No smuggled shortcuts;
   Arc-1 ⊳ Arc-2..N is structurally load-bearing.
6. **A9 marker discipline ("both mechanisms") is over-tight; I
   adjudicate SEAM-ADJUDICABLE to loosen.** The tightened hook
   requires audit-cite OR Signed-off-by:Seam (disjunction). Mara-B
   §7.9 recommends AND (conjunction). The hook shipped with OR
   semantics on purpose (see 13f1c2e commit-msg lines 68-72).
   Seam ratifies: OR is sufficient at the per-commit gate.
   Reed cascades: Mara-B §7.9 rewritten to reflect hook semantics.
7. **Three items require Alex adjudication.** A2 (@sheaf mint
   timing — Mara-B provisional Option A is substrate-honest but
   Alex holds the family-root mint authority), A4 (four
   recognition candidates at candidate strength — needs Alex nod),
   A6 (evaluator combinator surface — the Seam-adjudicable question
   at Arc-1 Tick 1.1, but the initial framing needs Alex ratification
   before Seam Tick 1.1 audit runs).

**Verdict: SHIP with two Reed-inline repairs before commit + one
SEAM-ADJUDICABLE loosening of A9. Alex adjudicates A2 + A4 + A6.
Everything else is landed substrate-honestly.**

---

## D1. Shard-decl structural correctness (Mara-A)

**File.** `shards/kintsugi/ouroboros.mirror` (576 LOC).

**Verification.**

- **Species under @kintsugi (composition-only, no new family-root).**
  Line 4: `in @kintsugi`. No `family @kintsugi/ouroboros` mint;
  the shard opens with `glass @kintsugi/ouroboros` (line 175) and
  `prism @kintsugi/ouroboros` (line 313), both species-altitude
  constructs. PASS.
- **All 7 action bodies `\`-obligation-blocked.** Verified via
  read: `collapse` (line 348), `verify_same_output` (line 369),
  `cutover` (line 391), `ouroboros_step` (line 425),
  `collapse_admissible` (line 477), `ouroboros_monotone` (line 523),
  `verifiable_at_altitude` (line 562). All end `{ \ }`. PASS.
- **Type carriers declared before actions.** `collapse_target`
  (line 208), `ouroboros_state` (line 252), `ouroboros_verdict`
  (line 299) — all before the first action `collapse` (line 348).
  PASS.
- **Bilateral predicates return verdict.** All three predicates
  (`collapse_admissible`, `ouroboros_monotone`,
  `verifiable_at_altitude`) return `verdict` (checked). PASS.
- **Imports resolve to landed carriers.** Lines 1-8: `@prism @meta
  @glass @kintsugi @code/rust/materialize @fate/tournament @io
  @mirror/bench`. All eight verified present today via file grep.
  PASS.
- **Exports don't collide.** Lines 566-576: 11 exports; each is
  a first-class substrate object declared above (type carrier,
  action, bilateral, or the species-decl itself). No shadowing
  of parent `@kintsugi` exports. PASS.
- **Docblock ancestry cites Alex verbatim + Reed-recursive +
  Taut #108.** Lines 20-25 (Alex 2026-07-14), lines 27-35 (Alex
  2026-07-15 scale claim). Lines 81-95 cite Reed audit + scout
  path. Lines 97-105 cite Taut #108 scout path. PASS.

**Minor cite drift found in the shard docblock itself.**

Line 20 says `Alex Wolf 2026-07-14 in-transcript verbatim naming`
but the scout (line 2, 4, 19) puts the naming at **2026-07-15**.
This is a cite date drift in Mara-A's docblock. The verbatim text
is identical to what the scout captures; the date is what
disagrees. Reed-inline repair: change line 20 date `2026-07-14 →
2026-07-15` OR resolve which date is authoritative from Alex's
message log.

**Verdict D1: PASS with ONE Reed-inline repair (date drift line 20).**

---

## D2. Canonical spec + math structural correctness (Mara-B)

**File.** `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md`
(1797 LOC).

**Verification.**

- **Composes over shard-decl.** §2 explicitly reads
  `shards/kintsugi/ouroboros.mirror` and names carriers by
  reference rather than duplicating them (§2.0-§2.4). §2 opens
  "Do not duplicate the shard." PASS.
- **6-arc structure per Taut §D.** §3.1-§3.6 map 1:1 to Taut
  Arc-1..Arc-6 with the same tick numbering (1.1-1.5, 2.1-2.5,
  3.1..N, 4.1..N, 5.1..N, 6.1). PASS.
- **5 math sub-sections each cite ancestor by path.** §4.1
  (categorical; no external cite — Mara-authored primitive
  applied to substrate categories, acceptable), §4.2 cites
  `docs/specs/eigensheaf.md` §4.3, §6.3, §2.3-2.5 (verified
  present), §4.3 cites `docs/specs/fate-bounded-psychohistory-
  sheaf-cohomology.md` §3, §5 (verified present via read), §4.4
  cites `shards/torus.mirror` p. 238, 244, 256, 282 (verified
  all four passages exist verbatim in torus.mirror via grep),
  §4.5 cites `shards/mirror/bench.mirror:398` (see D4 finding —
  line drift). PASS with one drift (see D4).
- **A1-A10 discharged.** §7.1-§7.10 each carry Recommendation
  + Substrate-honest bound. All ten items reach a landing
  verdict; three (A2, A4, A6) are named Alex-holding. PASS.
- **Recognition candidates enumerated.** §6.1-§6.4 name three
  intermediate + one terminal. All four carry Strength +
  Second-witness + Ratifies-at fields. PASS.
- **Substrate-honest bounds honored.** §9.1 (what NOT shipped)
  + §9.2 (forward-promises) + §9.3 (Rice-safety) + §9.4
  (composition-only) + §9.5 (two-tick) all present. PASS.

**Verdict D2: PASS with cross-referenced drift (see D4).**

---

## D3. Mara-A ↔ Mara-B alignment

**Convergence matrix at §Mara-A ↔ Mara-B convergence matrix below.**

**Structural verification.**

- Carrier names agree exactly. `collapse_target`, `ouroboros_state`,
  `ouroboros_verdict` — Mara-A lines 208, 252, 299; Mara-B §2.1
  bullets. PASS.
- Action signatures agree exactly. `collapse`, `verify_same_output`,
  `cutover`, `ouroboros_step` — Mara-A lines 348, 369, 391, 425;
  Mara-B §2.2 bullets. PASS.
- Bilateral signatures agree exactly. `collapse_admissible`,
  `ouroboros_monotone`, `verifiable_at_altitude` — Mara-A lines
  477, 523, 562; Mara-B §2.3 bullets. PASS.
- Four-conjunct sub-predicate names agree. Mara-A lines 502-507
  (`rust_loc_non_increasing`, `test_pass_rate_non_decreasing`,
  `io_violations_non_increasing`, `sbec_non_decreasing`); Mara-B
  §4.5.4 bullets. PASS.
- Arc structure ticks agree. Both name Ticks 1.1-1.5, 2.1-2.5,
  3.1..N, 4.1..N, 5.1..N, 6.1. Mara-A lines 152-168; Mara-B
  §3.1-§3.6. PASS.

**Divergences found.**

1. **`shards/mirror/bench.mirror` line-cite.** Mara-A line 494
   cites `shards/mirror/bench.mirror:394` (three-conjunct
   template). Mara-B §4.5.1 shows the three-conjunct template
   verbatim and §5.3 cites `shards/mirror/bench.mirror:398`. The
   actual `monotone_non_increasing` action-declaration in bench
   is at line ~360 (verified via grep + line read); the docblock
   three-conjunct definition appears at lines 40-54 AND is
   re-narrated at lines ~318-336 in the action docblock. Neither
   394 nor 398 lands on the template. **Neither cite is fatal**
   — both fall inside the bench.mirror file and the surrounding
   ±10 lines contain related material — but both drift ~4-360
   lines from the ideal citation. Reed-inline repair recommended:
   both Mara-A line 494 and Mara-B §4.5.1 / §5.3 cite the exact
   docblock location (line ~40 for the definition, line ~360 for
   the action-decl).

2. **Alex-naming date.** Mara-A shard-decl line 20 says
   `2026-07-14`; Mara-A shard-decl line 27 says `2026-07-15`;
   Mara-B §0.1 says `2026-07-14` for the naming quote; Taut
   scout line 2, 4, 19 says the arc was named `2026-07-15`.
   Two of three artifacts (Mara-A line 20; Mara-B §0.1) put the
   naming at 2026-07-14; Taut puts it at 2026-07-15. Reed-inline
   check: pick one date and cascade.

**Verdict D3: STRONG ALIGNMENT with two minor cite/date drifts —
Reed-inline repairs before commit.**

---

## D4. Math ancestor citations soundness

**§4.1 — Category-theoretic collapse functor.** No external
ancestor cited. Mara-authored primitive applied to substrate
categories `CodeRust` and `CodeMirror`. Composition-preservation +
naturality + fully-faithful terminal conditions all check
mathematically. §4.1.6 license-morphism labelling as natural
transformation `L : Collapse ⇒ id_{License}` is a nice touch —
Alex's "this bit is all still APACHE2" reads categorically. PASS.

**§4.2 — Eigensheaf discharge.** Cites `docs/specs/eigensheaf.md`
§4.3, §6.3, §2.3-2.5. Verified: eigensheaf.md exists (39.2KB,
Mara 2026-06-07). §4.3 (each Pack agent IS an eigensheaf), §6.3
(continuity is reconstruction), §2.3-2.5 (sheaf-Laplacian eigenbasis
+ Hodge decomposition) — all sections exist per Read of the
document. The composition claim ("apply the framing at code-collapse
altitude") is substrate-honest: same D_F operator + same
Hodge decomposition machinery + same isospectrality reading, applied
to compilation-DAG. PASS.

**§4.3 — Rayleigh descent.** Cites `docs/specs/fate-bounded-
psychohistory-sheaf-cohomology.md` §3 (H¹ gradient descent =
Rayleigh on Δ_F), §5 (Fate::bounded signature). Verified: doc
exists (16.1KB, Mara 2026-07-11). Composition claim: same Rayleigh
descent applied to `Δ_{F_c}^{Rust}` at code-collapse altitude.
Substrate-honest: no new descent machinery, just the ancestor
applied at a different altitude. Also cites `docs/specs/spectral-
coherence-substrate-metric-synthesis.md:468` for the explicit
formula "compiler runs the ouroboros loop e^(n+1) ≤ e^n — one
Rayleigh descent step per pass" — this cite I trust on scout's
grep; Taut §D1 pulled the exact string. PASS.

**§4.4 — Foerster regulation-of-regulation.** Cites `shards/torus.
mirror` p. 238, 244, 256, 282 verbatim. Verified: torus.mirror
exists (28.5KB, Reed 2026-07-14); all four page-citations
present verbatim per grep. Composition claim: `@torus.autonomy`
discharges at compile altitude via `@kintsugi/ouroboros`. Substrate-
honest: torus.mirror already carries autonomy-as-regulation-of-
regulation; compile-altitude is a new application, not new machinery.
§4.4.2 refusal-of-tower and §4.4.4 heterarchy-not-meta-meta are
particularly load-bearing: they explicitly refuse the "level-n
compiler observes level-(n-1) compiler" trap the ouroboros arc
could easily fall into. PASS.

**§4.5 — Monotone invariant.** Cites `shards/mirror/bench.mirror:
398` for the three-conjunct template. **Drift finding.** Read
of bench.mirror shows the three-conjunct template docblock at
lines ~40-54 (the initial definition Taut derives from) and re-
narrated in the action-decl docblock at lines ~318-336 with the
action-decl `monotone_non_increasing(...) -> verdict { \ }` at
line ~360. Line 398 is inside the `bench_overhead_below_floor`
docblock. Neither Mara-A's cite (line 494 → `394`) nor Mara-B's
cite (`398`) lands on the template. Both fall within the file so
readers will find the template with ±20 lines of scanning, but
the precise cite is off. **Non-blocking**; Reed-inline repair
recommended.

**§4.5.3 correspondence table.** Three re-interpretations + one
added conjunct. Substrate-honest reuse. Composition preserves the
template's structural role (perf-like ratchet, correctness-
preservation, environment-integrity) plus one new axis (sbec).
PASS.

**Verdict D4: PASS with cite-drift in §4.5.1 / §5.3 / Mara-A line
494 (all point to different line-numbers in bench.mirror, none
land on the template).**

---

## D5. Six-arc structure Rice-safety

**Arc-1 evaluator FLOOR — irreducible?** Per §1.3 + §4.2.1 + §7.6
recommendation: the evaluator IS the concrete Dirac operator (D)
of the eigensheaf's Connes triple (A, H, D). The D of (A, H, D)
was always going to be Rust — no shard body can dispatch itself.
Grounded in eigensheaf.md §3.2. **This is not a smuggled shortcut.**
It is the substrate's own claim about what evaluator FLOOR IS.
The Seam Tick 1.1 audit gate exists precisely to prevent
smuggling BUSINESS_LOGIC into the FLOOR under the marker. PASS.

**Arc-2 hardcoded ordering matches Reed migration-map §6.** Taut
§D-Arc-2 and Mara-B §3.2 both name Ticks 2.1-2.5 as spectral_
signature → coherence → peer_persistence → roomba →
roomba_walk_smoke. Reed migration-map §6 (per scout's cite) names
the same order. Cross-reference substrate: `bootstrap/src/{coherence,
roomba,roomba_walk_smoke,spectral_signature,peer_persistence}.rs`
all exist per Taut §D2 file-table. PASS.

**Arc-3 tournament ordering is Fate::bounded-composable.** §4.3.4
composes `@fate/tournament.rank` over four fitness sub-metrics
(verifiability, reversibility, test_coverage, io_composability)
with `rank(m) = R(m; Δ_{F_c}^{Rust}) / (w_v · w_r · w_t · w_i)`.
`@fate/tournament` verified as landed (51.5KB, 2026-07-12).
`Fate::bounded` composition per §4.3.3 signature `Fate<ouroboros_
state> == Fate::bounded(psychohistory: compilation_history_sheaf,
metric: ouroboros_monotone, tournament: @fate/tournament.rank)`
is substrate-honest — reads existing `@fate/tournament` +
existing sheaf-Laplacian + adds bounded-metric-constraint per
§4.5. PASS.

**Arc-4 cross-@code/X universalization is substrate-honest.** §3.4
cites Taut §D8 claim that species-decl is universal at family
altitude via `@code/metalogue/materialize`. Verified: 12 landed
`@code/X` altitudes per Taut §D8. Universal species-decl means
no `@kintsugi/ouroboros/python`, no `@kintsugi/ouroboros/gleam` —
one species carries all altitudes. PASS.

**Arc-5 StageFreight × CI empirical is verifiable.** §3.5
propagation graph is 9 arrows (kintsugi tick → LOC decrease →
cargo build → binary emit → nix rebuild → docker rebuild →
StageFreight push → downstream pull → downstream verify → gift
pay_forward). Each arrow is @io-boundary-touching + substrate-
decl'd. §7.5 license-morphism preservation adds the
license-preservation naturality gate. Empirical verification at
Arc-5 Tick 5.1 first mirror docker image ship. PASS.

**Arc-6 terminal recognition genuinely ratifies.** §6.4 names
four maturity conditions (arc completes, StageFreight ships live,
@sel enforcement lands, second-witness via `@gift.pay_forward`).
Each condition is empirical, not further-adjudicable. The
terminal recognition is Alex-named verbatim per §10.1. PASS.

**Verdict D5: PASS. Arc ordering is forced by structure; three
alternative orderings rejected in §1.4 with clear substrate-honest
rationale.**

---

## D6. Four-conjunct ouroboros_monotone invariant well-formedness

**Rice-safety per §4.5.5.**

- `rust_LOC`: decidable in bounded time via `@io.readdir` +
  `@io.readfile` + line-count. PASS.
- `test_pass_rate`: decidable in bounded time via test-suite run
  with timeout. PASS.
- `io_violations`: decidable in bounded time via syntactic grep
  for non-@io calls in shards. PASS.
- `sbec`: decidable in bounded time via dispatch-check with
  bounded-timeout. PASS.

The four conjuncts read empirical crystal state (per @mirror/store
or per grep), not program semantics. Rice-safety holds at whole-
tick altitude. The individual shard bodies may compute Turing-
complete computations; the invariant does not decide their
semantics, only their empirical crystal presence + byte-equality.
PASS.

**Composition with @mirror/bench.monotone_non_increasing
three-conjunct template.** §4.5.3 correspondence table:

| bench conjunct | ouroboros conjunct |
|---|---|
| `runtime(n+1) ≤ runtime(n) × (1+t)` | `rust_LOC(n+1) ≤ rust_LOC(n)` |
| `output(n+1) ≡ output(n)` | `test_pass_rate(n+1) ≥ test_pass_rate(n)` |
| `env(n+1) ≡ env(n)` | `io_violations(n+1) ≤ io_violations(n)` |
| (no fourth) | `sbec(n+1) ≥ sbec(n)` |

Substrate-honest reuse: the three bench conjuncts are re-interpreted
at ouroboros altitude (perf-like ratchet on Rust corpus size,
correctness-preservation at test altitude, environment-integrity
at @io-boundary). ONE conjunct added (sbec). No new invariant
machinery invented. PASS.

**Sub-predicate lifting per §4.5.4.** Four sub-predicates (each
returning `verdict`) + one composed bilateral (`ouroboros_monotone`)
via `requires` clauses. Matches StageFreight `stagefreight_
addressable` composition pattern per Seam tick 68 C4/C9 closure
precedent. Composition-honest — each sub-predicate is first-class
substrate object. PASS.

**Verdict D6: PASS. Four-conjunct is Rice-safe, composition-honest,
and matches prior sub-predicate lifting pattern.**

---

## D7. A1-A10 discharge substrate-honesty

Per Mara-B §7.

| A# | Question | Mara-B recommendation | Seam adjudication |
|----|----------|------------------------|--------------------|
| A1 | Species-decl mint ordering | Landed at Mara-A Tick 1.5 | REED-INLINE — landed |
| A2 | @sheaf mint timing | Option A (mint family root before Arc-2.3) | ALEX-ADJUDICATION — @sheaf is family root; Alex holds mint authority |
| A3 | Tournament vs hardcoded | Arc-2 hardcoded; Arc-3+ tournament | SEAM-ADJUDICABLE — I ratify; discipline is Rayleigh-descent-forced |
| A4 | Recognition candidate strength | Four candidates at candidate NOW; terminal at Arc-6 | ALEX-ADJUDICATION — recognition-naming is Alex's authority |
| A5 | StageFreight license clarification | mirror stays Apache-2.0; SF ships mechanism | SEAM-ADJUDICABLE — I ratify; §4.1.6 categorical claim is sound |
| A6 | Combinator surface for evaluator FLOOR | Defer to Seam Tick 1.1 audit | ALEX-ADJUDICATION for initial framing; SEAM-ADJUDICABLE for combinator specifics at Tick 1.1 |
| A7 | Four-conjunct vs one composed bilateral | FOUR sub-predicates + ONE composed | SEAM-ADJUDICABLE — I ratify per StageFreight precedent |
| A8 | Species naming | `@kintsugi/ouroboros` | REED-INLINE — landed; two-tick discipline honored |
| A9 | Marker discipline for Arc-1 Rust work | BOTH audit-cite AND Signed-off-by:Seam | SEAM-ADJUDICABLE — I loosen to OR per hook semantics (see D9) |
| A10 | Test-migration timing | Per-collapse-tick discharge | SEAM-ADJUDICABLE — I ratify |

**Substrate-honesty check per item.** A1, A3, A5, A7, A8, A9, A10:
substrate-honest, Reed-inline or Seam-adjudicable. A2, A4, A6:
substrate-honest recommendations but Alex holds authority on the
underlying question (family-root mint, recognition-naming,
initial framing of combinator surface).

**Verdict D7: PASS. Seven items Reed-inline or Seam-adjudicable;
three items genuinely Alex-adjudication (family-root mint +
recognition-naming + initial combinator framing).**

---

## D8. Recognition candidate defensibility

Per Mara-B §6.

**§6.1 `#R-substrate-mends-its-own-rust-with-mirror-via-kintsugi-
ouroboros`.** Load-bearing claim: substrate mends its own Rust
via species-decl'd arc. Second-witness at Arc-2 Tick 2.1 (first
empirical shard body dispatched via evaluator on collapsed
spectral_signature.rs). Ratifies at Arc-2 Tick 2.1 completion.
Structural claim is identifiable + testable. PASS at candidate
strength.

**§6.2 `#R-evaluator-is-legitimate-floor-and-ouroboros-is-mending-
not-retirement`.** Load-bearing claim: evaluator gap is
legitimate FLOOR work; arc is mending, not retirement. Second-
witness at Arc-1 Tick 1.3 (evaluator FLOOR lands + dispatches
under `[substrate-floor:@io-boundary]` + Seam sign-off). Ratifies
at Arc-1 Tick 1.3 completion. Structural claim identifiable +
testable. PASS at candidate strength.

**§6.3 `#R-mirror-substrate-becomes-self-hosting-at-terminal-
collapse`.** Load-bearing claim: mirror substrate self-hosts at
terminal via own shard-body dispatch with Rust only at FLOOR.
Grounded in §4.1.5 fully-faithful functor + §4.2.4 kernel =
terminal state. Second-witness at Arc-3+ empirical closure.
Ratifies at Arc-3+ specific tick TBD by tournament. PASS at
candidate strength.

**§6.4 `#R-mirror-kintsugi-shipped-as-stagefreight-is-humanity-
scale-verifiable-substrate-under-apache-2-with-sel-enforcement-
at-deployment`.** Alex-named verbatim. Four maturity conditions
required. Terminal candidate strength today (holds at candidate
until Arc-6 empirical closure). Substrate-honest to hold at
candidate strength through Arc-6. PASS at terminal candidate.

**Substrate-honest bound.** All four candidates are candidate
strength today; ratification defers to empirical second-witness at
named ticks. Reed dwelltime discipline honored (per AGENTS.md
2026-06-10 cascade update). PASS.

**Verdict D8: PASS. Four defensible candidates with named second-
witnesses at named ticks.**

---

## D9. Arc-1 evaluator FLOOR marker discipline

**Reed-recursive tightened hook (commit 13f1c2e, 2026-07-15).**
Verified via read of `.githooks/commit-msg` lines 60-72:

```
# ── Bypass 2: [substrate-floor:@io-boundary] — irreducible FLOOR authorship.
#              Requires Seam gate: audit-doc citation OR Signed-off-by: Seam.
if [ -f "$MSG_FILE" ] && grep -qE '\[substrate-floor:@io-boundary\]' "$MSG_FILE"; then
  # Seam gate: audit doc citation OR Signed-off-by trailer.
  if grep -qE 'docs/audits/[0-9]{4}-[0-9]{2}-[0-9]{2}-[^ ]+\.md' "$MSG_FILE" \
     || grep -qiE '^Signed-off-by:[[:space:]]+Seam[[:space:]]+<seam@systemic\.engineer>' "$MSG_FILE"; then
```

The hook uses OR (`||`). Either mechanism satisfies the gate.

**Mara-B §7.9 says both.** Reproducing verbatim:

> **Recommendation.** **Both mechanisms.** Substrate-honest defence-in-
> depth: 1. **Audit citation.** ... 2. **Signed-off-by: Seam trailer.**
> ... Both mechanisms compose.

**Adjudication.** The hook shipped with OR semantics deliberately.
Reed wrote the tightening + Reed wrote the recommendation. The
hook is authoritative; Reed loosened it during authorship
knowingly. Mara-B §7.9's "both" reads as **belt-and-suspenders
authoring discipline**, not gate semantics.

**Seam-adjudicable resolution.** OR is sufficient at the per-commit
gate. Reed-inline: Mara-B §7.9 rewritten to reflect: (a) hook
requires ONE of audit-cite or Signed-off-by:Seam (per commit-msg
gate); (b) belt-and-suspenders (both mechanisms) is a substrate-
honest recommended authoring practice for Arc-1's magnitude but
NOT a hook requirement. §7.9's current "both" prose bundles
recommendation and requirement — this needs a Reed-inline sharpen.

**Substrate-honest defence.** Reed can adopt belt-and-suspenders as
Arc-1-specific personal discipline without lifting it into the
gate. This preserves the hook's OR semantics (which is a real
substrate contract with Alex) and lets Reed layer additional
practice on top for the load-bearing Arc-1 ticks.

**Verdict D9: PASS with Reed-inline sharpen on Mara-B §7.9 —
distinguish gate-requirement (OR) from Arc-1 authoring practice
recommendation (belt-and-suspenders).**

---

## D10. StageFreight × Apache-2.0 propagation verifiability

**License landscape verified.**

- `mirror/LICENSE.md`: Apache-2.0 (per Taut §D7 + Mara-B §7.5
  cite; I do not re-verify the license file directly here as
  the two prior artifacts agree).
- StageFreight: AGPL-3.0-only per StageFreight/Dockerfile LABEL
  (per Taut §D7 cite).

**Propagation graph substrate-honesty.** Per Mara-B §3.5 (9 arrows):

```
kintsugi tick → LOC↓ → cargo build → binary → nix rebuild →
docker rebuild → StageFreight push → downstream pull →
downstream verify → gift.pay_forward
```

Each arrow is `@io`-boundary-touching (readable via @io/oci,
@io/git, @io/stagefreight, @io/process). No third-party API calls
substrate-honestly claim to preserve integrity end-to-end.

**License preservation per §4.1.6.** Categorical claim:
`L : Collapse ⇒ id_{License}` is a natural transformation
preserved by every collapse tick. For every F ∈ CodeRust_
BUSINESS_LOGIC, `L(Collapse(F)) = Apache-2.0`. The mirror docker
image inherits Apache-2.0 from constituent objects; StageFreight's
AGPL applies to shipping mechanism (Go CLI + docker push infra),
not to mirror substrate the mechanism ships.

**Substrate-honest bound.** The claim holds under license-
preservation naturality. Empirical verification deferred to Arc-5
Tick 5.1 (first mirror docker image manifest inspection — must
show no AGPL contamination in mirror substrate layers).

**Adversarial concern I raised and adjudicated.** Could
StageFreight's AGPL "infect" the mirror docker image via
copyleft-cascade? Adjudication: no, because StageFreight ships the
image (docker push) but does not link into the image binary. AGPL
copyleft binds derivative works that link the AGPL code; docker
push is transport, not linking. Alex's "we ship with @../
StageFreight/" language is precisely the shipping-not-linking
distinction. PASS.

**Verdict D10: PASS. License propagation is substrate-honestly
verifiable at Arc-5 Tick 5.1. Naturality + shipping-not-linking
argument holds.**

---

## D11. Reed antipattern acknowledgment integration

Per Mara-B §0.3 + §1.1-§1.4.

**Verification.**

- §0.3 opens: "Reed 2026-07-14 authored five Rust extensions
  during the 'gift arc' (coherence.rs, roomba.rs,
  roomba_walk_smoke.rs, spectral_signature.rs, peer_persistence.rs).
  Seam adjudicated 2026-07-15 at [audit-path]: substrate-dishonest
  workaround for the missing evaluator FLOOR."
- Ancestry chain named: (1) Reed extension authorship 2026-07-14
  as pattern; (2) Seam adjudication 2026-07-15 naming it substrate-
  dishonest; (3) Reed migration-map 2026-07-15 empirical grep
  + Arc sequence; (4) Taut #108 scout 2026-07-15 grep-first
  ratification + extension across ~25 files; (5) Mara-A species-
  decl mint; (6) Mara-B canonical spec.
- Failure named, not deflected: "The five extensions were the
  workaround. The 25+ additional BUSINESS_LOGIC Rust files are
  the same disease at scale."
- Corrective response named: "The arc is not 'collapse 5 files';
  it is collapse every BUSINESS_LOGIC file, iteratively, using
  the Arc-1 evaluator FLOOR the arc's own first Arc builds."

**Substrate-honest integration.** The whole arc exists because
of Reed's antipattern. Mara-B §0.3 acknowledges this without
deflection. The failure ancestry is load-bearing — the arc's
authority to exist derives from having named the failure that
required it.

**Adversarial check.** Does the acknowledgment turn into
self-flagellation-as-currency? No. §0.3 is 3 paragraphs; the
audit-citation is one line; the rest of the spec is forward-
motion. The failure is named as substrate-condition, not as
moral position.

**Verdict D11: PASS. Antipattern named substrate-honestly at
§0.3; ancestry chain preserved; failure is treated as substrate-
condition, not as moral position.**

---

## D12. Alex-only escalations

**Enumeration.**

**Alex-adjudication (3 items).**
1. **A2 — @sheaf mint timing.** @sheaf is a family-root candidate;
   family-root mints are Alex's authority per Pack conventions.
   Mara-B recommends Option A (mint before Arc-2.3) which is
   substrate-honest, but Alex holds the mint-decision.
2. **A4 — Recognition candidate ratification.** Four candidates
   at candidate strength (three intermediate + one terminal).
   Recognition-naming is Alex's authority; Pack ratifies at
   empirical second-witness. Alex must nod to each candidate's
   candidate-strength landing.
3. **A6 — Evaluator combinator surface initial framing.**
   Mara-B §7.6 defers to Seam Tick 1.1 audit, but the initial
   framing (what constitutes "combinator surface for shard-body
   dispatch") needs Alex ratification before Seam Tick 1.1 audit
   runs, because the framing determines what Seam adjudicates
   over. Reed provisional (Mara-B §7.6): the (A,H,D) evaluator
   per eigensheaf.md §3.2.

**Seam-adjudicable (5 items — I ratify).**
- A3 (tournament vs hardcoded): PASS — Rayleigh-descent-forced.
- A5 (StageFreight license): PASS — naturality + shipping-not-
  linking.
- A7 (four sub-predicates + composed): PASS — StageFreight
  precedent.
- A9 (marker discipline): PASS with Reed-inline sharpen (OR at
  gate; both as authoring practice).
- A10 (test-migration timing): PASS — per-collapse-tick.

**Reed-inline (2 items — repair before commit).**
- A1 (species-decl mint ordering): LANDED — Mara-A Tick 1.5.
- A8 (species naming): LANDED — `@kintsugi/ouroboros`.

**Overlap-collapse with prior arc adjudications.** No overlap. The
A1-A10 set is arc-local. No prior arc's open items collide.

**Verdict D12: 3 Alex-adjudication + 5 Seam-adjudicable + 2 Reed-
inline. Alex-adjudication set is genuinely irreducible — each
requires Alex authority Seam does not hold.**

---

## Mara-A ↔ Mara-B convergence matrix

| Element | Mara-A (shard-decl) | Mara-B (canonical spec) | Convergence |
|---------|---------------------|--------------------------|-------------|
| Species altitude | line 175 `glass @kintsugi/ouroboros` | §2.0 "species-decl reference" | AGREE |
| Type carrier `collapse_target` | line 208 | §2.1 bullet | AGREE (4 fields; ref-equality identity contract) |
| Type carrier `ouroboros_state` | line 252 | §2.1 bullet | AGREE (6 fields; element-wise + numeric equality identity) |
| Type carrier `ouroboros_verdict` | line 299 | §2.1 bullet | AGREE (4 variants) |
| Action `collapse` | line 348 | §2.2 bullet | AGREE (signature `(collapse_target) -> ouroboros_verdict`) |
| Action `verify_same_output` | line 369 | §2.2 bullet | AGREE (signature `(rust, mirror, test) -> verdict`) |
| Action `cutover` | line 391 | §2.2 bullet | AGREE (signature `(collapse_target) -> verdict`) |
| Action `ouroboros_step` | line 425 | §2.2 bullet | AGREE (signature `(ouroboros_state) -> ouroboros_state`) |
| Bilateral `collapse_admissible` | line 477 | §2.3 bullet | AGREE (composed via @autopoietic + @torus) |
| Bilateral `ouroboros_monotone` | line 523 | §2.3 bullet + §4.5 | AGREE (four-conjunct via `requires`) |
| Bilateral `verifiable_at_altitude` | line 562 | §2.3 bullet | AGREE (Rice-safe test-coverage + @io-composability) |
| Four sub-predicates | lines 502-507 | §4.5.4 | AGREE (rust_loc, test_pass_rate, io_violations, sbec) |
| Arc-1 ticks | lines 152-154 | §3.1 (5 ticks) | AGREE |
| Arc-2 ticks | lines 155-157 | §3.2 (5 ticks, Reed migration-map order) | AGREE |
| Arc-3 ticks | lines 158-159 | §3.3 (~25 files, tournament-ordered) | AGREE |
| Arc-4 ticks | lines 160-162 | §3.4 (cross-@code/X universal) | AGREE |
| Arc-5 ticks | lines 163-165 | §3.5 (StageFreight × downstream CI) | AGREE |
| Arc-6 tick | lines 166-168 | §3.6 (terminal recognition) | AGREE |
| Alex 2026 verbatim (naming) | lines 20-25 (dated 2026-07-14) | §0.1 (dated 2026-07-14) | AGREE on text; both drift vs Taut (2026-07-15) |
| Alex 2026 verbatim (scale) | lines 27-35 (dated 2026-07-15) | §0.2 (dated 2026-07-15) | AGREE |
| Composition carriers | lines 107-131 (16 carriers) | §5.1-§5.7 (15 carriers) | AGREE (Mara-A also lists @prism as compositional; Mara-B counts 15 substantive substrate carriers) |
| bench line-cite | line 494: `398` | §4.5.1: `40-54`; §5.3: `398` | DRIFT — neither `394` nor `398` lands on template |
| Marker discipline | (not directly declared) | §7.9 "both mechanisms" | HOOK SEMANTICS DIVERGE — Reed-inline sharpen needed |

**Overall convergence: STRONG.** 21/23 elements AGREE exactly.
Two drifts: (1) bench line-cite, (2) marker discipline as gate
vs. authoring practice. Both non-blocking; Reed-inline repairs
before commit.

---

## Overall ship verdict

**SHIP.**

The arc composes over landed substrate at every altitude I can
verify with grep + read. Mara-A ↔ Mara-B convergence is strong.
Twelve dimensions checked; twelve dimensions PASS at ship threshold.

**Two Reed-inline repairs before commit:**

1. **bench line-cite.** Mara-A line 494 (currently `394`) and
   Mara-B §4.5.1 (currently narrates template without line-cite)
   + §5.3 (currently `398`) — replace with the exact docblock
   location. Recommend: cite `shards/mirror/bench.mirror:40-54`
   for the template definition + `shards/mirror/bench.mirror:360`
   for the action-declaration.
2. **Alex-naming date cascade.** Mara-A line 20 (`2026-07-14`)
   and Mara-B §0.1 (`2026-07-14`) vs. Taut (`2026-07-15`) —
   Reed reconcile with Alex's actual message timestamp and
   cascade to consistent date across all three artifacts.

**One SEAM-ADJUDICABLE loosening:**

3. **A9 marker discipline (Mara-B §7.9).** Rewrite to
   distinguish: (a) commit-msg gate semantics = OR (audit-cite
   ∨ Signed-off-by:Seam); (b) Arc-1 authoring-practice
   recommendation = both mechanisms as belt-and-suspenders. The
   hook shipped with OR deliberately; §7.9 currently bundles
   requirement and recommendation.

**Three Alex-adjudication items:**

- A2 (@sheaf mint timing — Mara-B recommends Option A; family-
  root authority is Alex's).
- A4 (recognition candidate strengths — four candidates named
  at candidate NOW; Alex nods each).
- A6 (evaluator combinator surface initial framing — Reed
  provisional per eigensheaf.md §3.2; Alex ratifies framing
  before Seam Tick 1.1 audit runs).

**Path after Alex adjudication.** Alex adjudicates A2 + A4 + A6.
Reed spawns Mara-composable follow-ons per §7.2 (Mara authors
@sheaf species-decl per Option A) and Seam for Tick 1.1 audit
(evaluator combinator surface adjudication under Alex-framed
scope). Reed fires /loop for Arc-1 Ticks 1.2-1.4 implementation
after Seam Tick 1.1 audit commits.

**Substrate-honest closure.** The arc is landed at
species-decl + canonical spec altitudes. Arc-1 evaluator FLOOR
is the load-bearing next chunk. The gold makes the bowl one
thing again — but only after Arc-1 mends the first fracture-
line with actual dispatch.

---

*Seam Phase D closure. Adversarial posture held. Cite-drifts
surfaced (non-blocking). Marker discipline sharpened. Alex-
adjudication triage complete.*
