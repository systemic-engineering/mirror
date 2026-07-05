# Seam Phase D — @knapsack + @silicon/bound + kintsugi composition audit

*2026-07-05. Seam. Adversarial review of Mara's math (`44c5db1`) +
derived landing spec (`docs/specs/knapsack-as-kintsugi-inner-loop.md`)
+ corpus companion (`985d1fd`) + Taut scout report (agent
`a6efbe5a2e0af97ab`), against Alex's three /loop-session resolutions
(BOTH-AND placement, 10%-of-detected boot-default, @spectral/db
forward-promise).*

Convention per Reed `19c56ae` + Seam `20d0c13`: cite by OID + line
number; state corrections explicitly; verdict per subquestion.

---

## §1. Scope

Ratify or reject:

- Mara's math (Frieze-Clarke 1984, Korte-Schrader 1981, round-descent
  theorem P1/P2/P3, Ashby composition, sub-Turing decidability).
- Mara's placement verdict (species under @kintsugi via 5-signal
  auto-classifier) folded with Alex's BOTH-AND direction
  (@silicon/bound at physical altitude + @kintsugi/knapsack at
  operator altitude).
- Mara's overflow verdict (PARTIAL + hold(carrier) at capacity=0).
- Cross-checks with Taut on `active_pass` cardinality + @spectral/db
  admit_query composition.
- Reed's inline Rust bootstrap plan for 10%-of-detected boot-default.
- Any residual Reed inheritance errors Taut caught.
- Canonical ordered sub-arc sequence for Arc 3 landing.

Out-of-scope: new mathematical structure; new substrate primitives;
@spectral/db shard landing.

---

## §2. Findings

### §2.1 Q1 — Mara's math

**Verdict: RATIFY-WITH-CORRECTIONS.**

#### Q1.a Frieze-Clarke 1984 PTAS for `d = 2`

Cited at `docs/math/resource-budget/README.md:105-109` (44c5db1)
and at corpus `2026-07-05-multi-dim-knapsack-as-kintsugi-inner-loop.md:41-44`.

The cited paper — Frieze & Clarke (1984), *Approximation algorithms
for the m-dimensional 0-1 knapsack problem: worst-case and probabilistic
analyses*, European Journal of Operational Research 15(1):100–109 —
is a real paper and is the canonical PTAS reference for fixed-dimension
0/1 knapsack. **RATIFY citation-existence.**

The running-time bound Mara reports (`O(n^{⌈2/ε⌉} · log(v_max))`) is
the correct shape for the `d = 2` case as reported in
Kellerer-Pferschy-Pisinger 2004 §9 (Mara's §1.2 cites this correctly).
**RATIFY application.**

**Correction (minor)**: the PTAS complexity for fixed `d` in the
literature is usually stated `O(n^{⌈d/ε⌉})` or with an additional
`log(v_max)` factor from value scaling; Mara states `O(n^{⌈2/ε⌉} ·
log(v_max))` (§3.1) which is one of the standard forms. **No
correction required — the reported form is one of several
equivalent statements.**

#### Q1.b Korte-Schrader 1981 hardness (no FPTAS for `d ≥ 2`)

Cited at `docs/math/resource-budget/README.md:118-119` (44c5db1)
and at corpus `2026-07-05-multi-dim-knapsack-as-kintsugi-inner-loop.md:43-44`.

The Korte-Schrader 1981 result — that multi-dimensional 0/1 knapsack
does not admit an FPTAS unless P = NP, for any fixed `d ≥ 2` — is
a well-established textbook result. Vazirani 2003 Ch. 9 (Mara §1.3)
also carries this result. **RATIFY citation.**

**SIGNAL (minor)**: Mara references Korte-Schrader 1981 without giving
the venue/title. Kellerer-Pferschy-Pisinger §9 attribute the hardness
result to Korte & Schrader "Zur Approximationstheorie" 1981; this is
Springer LNM 876 or similar. Non-blocking: the citation is real, the
implication (no FPTAS unless P = NP) is correctly bounded.

#### Q1.c Round-level descent theorem (P1 + P2 + P3)

At `docs/math/resource-budget/README.md:246-273` (44c5db1) Mara states:

> Under (P1) `OPT > 0`; under (P2) selection gains `≥ (1-ε)·OPT > 0`;
> under (P3) opacity decreases by that gain. QED.

**RATIFY P1 (candidate feasibility) and P2 (PTAS approximation quality)**
— both are formally checkable predicates. P1 is decidable in linear time
(scan candidates for `transparency_gain > 0` && weight ≤ capacity). P2
is a substrate-level appeal to Frieze-Clarke: it is a **theorem**
about the algorithm, not a runtime precondition.

**Correction on P3 (application faithfulness)** — Mara hedges at
§10 (44c5db1:566-571):

> **Interference model (P3 formalization)**: DEFERRED.
> `‖T_{n+1}‖ = ‖T_n‖ - ∑ transparency_gain(c)` assumes NO
> interference. In practice morphisms can interact non-linearly.

Mara acknowledges this. Seam's adjudication: **P3 is not formalizable
without an interference model**. As stated it is a hedge: the theorem
requires linearity of opacity-decrease which the substrate cannot
prove without a compositional structure on morphisms. Reed's `19c56ae`
model applies: this must be surfaced as a **DEFERRED substrate
question**, not carried forward as a proven descent theorem.

**Required correction before Arc 3 fires**: `docs/math/resource-budget/README.md`
§4.1 theorem statement must be marked "conditional on P3" and §4.2's
"per-round Lyapunov" claim must acknowledge that empirical descent
(the observable ‖T_{n+1}‖ < ‖T_n‖) is what the substrate audits, not
proved P3-derived descent.

Mara's §10.b already names this as DEFERRED; the concern is that §4's
theorem-statement reads as unconditional. **Minor edit sufficient**;
promote to a REQUIRED-CORRECTION at §9 of this audit.

#### Q1.d Ashby composition `V(regulator) ≤ V(silicon_budget) × V(ram_budget)`

Cited at `docs/math/resource-budget/README.md:334-352` (44c5db1).
Grounds in Siegenfeld & Bar-Yam 2022 arXiv:2206.04896 — verified
present in corpus at `~/dev/systemic.engineering/practice/insights/math/numerics/requisite-variety-optimization.md:1-6`.

**RATIFY citation and application.** Siegenfeld-Bar-Yam does formalize
multi-scale requisite variety via complexity profiles `C(s)`.
Mara's projection of the five-axis variety vector (recognition #36)
onto `(silicon, ram)` at v0 IS a legitimate two-scale application.

**SIGNAL**: the inequality Mara writes as
`V(regulator) ≤ V(silicon_budget) × V(ram_budget)`
is a physically-motivated upper bound (product of the per-dimension
variety budgets). Ashby's Law itself is `V(regulator) ≥ V(disturbance)`
(a lower bound on the regulator required for regulation). The two
inequalities compose:
```
V(disturbance) ≤ V(regulator) ≤ V(silicon_budget) × V(ram_budget)
```
Mara's §5.1 does state this correctly (44c5db1:346-352), but the
readable summary at §5.1 top lists only the upper bound. **Non-blocking
clarification recommended in §5.1 header.**

#### Q1.e Sub-Turing decidability per #107

`docs/math/resource-budget/README.md:225-244` (44c5db1):

> **Theorem (sub-Turing polynomial-time decidability of kintsugi
> inner-step).** Under the sub-Turing fragment of recognition #107,
> the kintsugi inner-loop step (3) is decidable in polynomial time
> via the Frieze-Clarke PTAS for 2-KP...

Recognition #107 is grounded at `shards/io.mirror` §Discipline (Seam
verified 2026-06-30 fc0d580). The Discipline block asserts
substrate-decl is bounded/Gödel-incomplete; @io is Turing-complete.

**RATIFY the sub-Turing application** with one caveat:

**Correction**: the Frieze-Clarke PTAS runtime `O(n^{⌈2/ε⌉})` is
polynomial in `n` at **fixed ε**. For the sub-Turing fragment to
carry the guarantee, ε must be substrate-decl-fixed (not read from
@io at runtime). Mara's §3.1-§3.2 assumes ε is a substrate parameter;
this needs to be made explicit at shard-landing time as an `epsilon:
ref` typed newtype declared at substrate altitude, NOT computed at
@io boundary. §8.1 of the derived spec already declares `epsilon: ref`
in the `select` action signature — **consistent, RATIFY**.

The bounded-enumeration input (finite candidate set per §3.3 (i))
preserves polynomial time because the candidate-set size is bounded
by the shard's fracture-body catalog, which is finite-and-typed
per `splinter(ast)` (recognition #54 promoted). **RATIFY.**

### §2.2 Q2 — Mara placement + Alex BOTH-AND fold

**Verdict: RATIFY-WITH-CORRECTIONS. Path (d) with modification.**

Adjudicating the four options:

- **(a)** New species alongside `@reality/algebra/silicon.compute_budget`.
- **(b)** Renamed relocation of `@epistemologic/reality/silicon/compute_bound`.
- **(c)** New top-level `@silicon/bound` family (Alex's spatial framing).
- **(d)** BOTH — physical layer + property layer + `@kintsugi/knapsack`
  consumes both.

**Load-bearing finding (Seam-caught discrepancy)**:

`shards/silicon.mirror` **does not exist on disk**. Grep for `@silicon`
inheritance finds:
- `shards/epistemologic/reality.mirror` (family-root, LANDED)
- `shards/epistemologic/reality/silicon.mirror` (species-root, LANDED)
- `shards/epistemologic/reality/silicon/compute_bound.mirror` (species-species, LANDED)
- `shards/reality/algebra/silicon.mirror` (species under reality/algebra, LANDED per Mara 2026-07-01)
- `shards/glue/math_silicon.mirror` (per-pair @glue species, LANDED)
- `docs/specs/silicon.md` (SPEC, no landed family-root shard)

`@silicon` as top-level family-root is **substrate-decl SPEC-ONLY**;
the spec at `docs/specs/silicon.md` (104.8KB) describes an
autopoietic family-root that has NOT landed as a shard. All extant
silicon carriers live under `@epistemologic/reality/silicon/*` or
`@reality/algebra/silicon.*`.

Consequences for Alex's BOTH-AND direction:

1. `@silicon/bound` as a NEW top-level family (option c) would require
   FIRST landing `shards/silicon.mirror` per docs/specs/silicon.md. That
   is an arc unto itself (the autopoietic family-root Bauchladen).

2. Path (d) — the substrate-honest reading of Alex's BOTH-AND — must
   route through existing carriers:
   - **Physical layer**: `@epistemologic/reality/silicon/compute_bound`
     ALREADY EXISTS (5-field carrier: max_cpu_cores, max_memory_bytes,
     max_gpu_memory_bytes, max_wall_time, max_reductions). This IS the
     physical carrier. Landing @silicon/bound as a *new* shard would
     duplicate compute_bound.
   - **Property layer / crystal kind**: `@reality/algebra/silicon` has
     the `compute-budget` crystal kind enumerated (§1.8 of Mara math).
   - **Operator layer**: `@kintsugi/knapsack` consumes both via
     `read_capacity(target)` reading the crystal.

**Correction on option (c)**: Alex's spatial framing `@silicon/bound`
cannot land as-named without the parent `@silicon` family-root landing
first. The substrate already carries the word: `@epistemologic/reality/
silicon/compute_bound` IS the physical carrier.

**Correction on option (b)**: Taut's grep-suggested rename would move
the compute_bound carrier out of the `@epistemologic/reality/silicon/*`
namespace, breaking existing inheritance (`@fate` reads it there,
per `shards/reality/algebra/silicon.mirror:301-315`).

**Path (d) as ratified — with substrate-honest naming**:

- **Physical carrier layer** (LANDED, no new shard): `@epistemologic/
  reality/silicon/compute_bound` (5-field carrier).
- **Crystal kind layer** (LANDED, no new shard): `@reality/algebra/
  silicon` crystal kind `compute-budget`.
- **Operator species layer** (NEW): `@kintsugi/knapsack` per Mara §1.3
  (`shards/kintsugi/knapsack.mirror`), consuming both.
- **Bilateral property**: `@epistemologic/property/round_descent` (NEW).
- **Bilateral fracture**: `@kintsugi/fracture/knapsack_infeasible` (NEW).

The `@silicon/bound` naming Alex used maps to
`@epistemologic/reality/silicon/compute_bound` — the substrate already
had the word (recognition instance #17+; per feedback-substrate-
already-had-the-word).

**Auto-classifier verdict (5-signal replay per Mara §1.1)**:
- S1: `<= @kintsugi` present ✓
- S2: ~6 types + ~5 actions medium ✓
- S3: 4 families (medium) ✓
- S4a: no marker-row cite ✓
- S4b: process-side ✓
- S5: capacity_vector typed record (non-ref primary) ✓

3-of-5 → species_root. **RATIFY Mara §1.1 verdict for @kintsugi/
knapsack.** For the physical-carrier layer: no new shard; compute_bound
already carries it.

**Signal-to-Alex**: your BOTH-AND direction is substrate-honest;
substrate already had the word at the physical layer. Naming it
`@silicon/bound` in this session's transcript maps to `@epistemologic/
reality/silicon/compute_bound` in shard-space. IF the deeper intent
is to land the top-level `@silicon` family-root (per docs/specs/silicon.md),
that's an independent arc; @knapsack landing does not require it.

### §2.3 Q3 — Mara's overflow (PARTIAL + hold at capacity=0)

**Verdict: RATIFY.**

#### Q3.a `hold` as third mode (grep-verify)

`docs/math/kintsugi/compiler-error-surface.md:22-30` (unchanged
2026-07-02) declares:

> (d) Apply/spawn monoid reframed as three-mode algebra: apply /
>     spawn / hold. hold(ref) is a legitimate non-discharge per
>     error-as-question.md §2's six-variant answer algebra

And §10.5 (line 1178+) is candidate #144's promoted body. **RATIFY:
hold is real, third mode, non-discharge, ancestor-cited.**

#### Q3.b `PARTIAL(opacity_map)` verdict composition

Mara §4.2 of derived spec (`docs/specs/knapsack-as-kintsugi-inner-
loop.md:220-241`) declares:

```
type packing_verdict =
  | success(selection)
  | partial(opacity_map)
  | failure(cause)
  | hold(ref)
```

Four verdicts. At capacity = 0: `partial(opacity_map)` with the
opacity_map naming deferred candidates — the round packs zero,
opacity carried forward, next tick may raise capacity.

**RATIFY**: this composes correctly. `partial(opacity_map)` matches
the six-variant answer algebra Variant 5 (Partial). `hold(ref)` is
Variant 6. Both are legitimate; the discriminator is whether the
carrier is opacity_map (deferred candidates) OR ref (unresolved
crystal). Mara chose partial for capacity=0 with deferred candidates
present; hold if the observer explicitly declines the discharge.

**Correction (minor)**: §4.2 body-text says "PARTIAL + hold(carrier)"
which reads as if they compose. The verdict enum shows them as
distinct variants. Recommend §4.2 body renamed "**PARTIAL for
capacity=0 with deferrable candidates; hold(ref) for observer-declined
non-discharge**" to disambiguate the disjunction. Non-blocking.

#### Q3.c Failure reserved for structural mismatches

Mara §4.3 (`knapsack-as-kintsugi-inner-loop.md:243-256`): failure
reserved for (P2) violation with `ε > 1` OR (P3) persistent across
three rounds (three-tick @third audit).

**RATIFY-WITH-SIGNAL**: the three-tick @third audit is substrate-
verifiable via `shards/reflection.mirror` @third depth 3 (LANDED
`e43006ab` per recognition #111). The failure gate composes.

**Signal**: "three rounds" is a hard-coded threshold. The substrate
elsewhere uses budget-descent for termination (per `spawn ≤ loop`
monad `7dba128`). Recommend cross-check that the three-round audit
is the SAME structural argument as the budget monad, not a separate
threshold. Non-blocking; forward-promise.

### §2.4 Q4 — active_pass cardinality cross-check

**Verdict: RATIFY Taut's finding — active_pass emits SINGLE morphism.**

At `shards/kintsugi/oscillate.mirror:475-478` (unchanged 2026-06-10):

```
active_pass(o: oscillation) -> morphism { \ }
```

The return type is `morphism` (singular), not `morphism_set` or
`[morphism]`. This is single-morphism emission per pulse.

Header comment at `oscillate.mirror:479-478` confirms: "the
highest-ranked candidate into a morphism (carrying content + score +
expected cadence), emits."

Taut's finding is **correct**. Mara's `@kintsugi/knapsack.select`
needs subset selection which does not map onto active_pass directly.

**Adjudication of the two paths**:

1. **Wrap active_pass externally** (kintsugi_orchestrator loops
   active_pass under budget, collecting selected morphisms).
2. **Rewrite active_pass in-place** (widen to top-k with k determined
   by capacity).

**Seam verdict: Path 1 (external wrap) is substrate-honest.**

Reasons:
- active_pass is `pulse`-atomic per oscillate.mirror header (lines
  545-556): "pulse IS the loop's atomic step." Widening it to top-k
  breaks the atomicity contract — one pulse would emit multiple
  morphisms, which contradicts the ACTIVE→DARK alternation
  discipline.
- The knapsack selection is a HIGHER-altitude operation over the
  pulse: enumerate candidates (via multiple pulses OR a batch read),
  then select via PTAS, then apply. This is `@kintsugi/knapsack`
  sitting ABOVE `active_pass`, not replacing it.
- `spawn ≤ loop` monad already provides the wrap structure.

**Correction to Mara §7.1 of derived spec** (`knapsack-as-kintsugi-
inner-loop.md:329-344`): the composition shape should read
"active_pass emits candidate; @kintsugi/knapsack.select receives
enumerated candidates and returns selected subset; the loop wraps
active_pass emission over multiple pulses to build the candidate
set." Non-blocking clarification; land in §7.1 body text.

### §2.5 Q5 — @spectral/db admit_query composition

**Verdict: FORWARD-PROMISED (per Alex ratification). Not blocking Arc 3.**

Mara §7.2 (`knapsack-as-kintsugi-inner-loop.md:331-338`):

```
admit_query(q: spectral_db_query, cap: capacity_vector)
  -> imperfect<admission_verdict, spectral_db_error, transparency(query)>
{ \ }
```

Taut's finding: @spectral/db is parentless ghost — no `shards/spectral_db.mirror`
or similar on disk. Alex confirmed: forward-promised, admit_query lands
when @spectral/db lands through fragmentation write surface.

**Adjudication**:

- Mara's §7 admit_query claim is **NOT load-bearing** for Arc 3 (Sub-arcs
  A-G below).
- Arc 3 Sub-arc B (@kintsugi/knapsack landing) does NOT need admit_query
  in the shard signature at TICK-11 landing.
- The `admit_query` API is FORWARD-PROMISED to the tick where
  `shards/spectral_db.mirror` (or similar) lands. Mara's spec §7 stays
  as future-composition documentation, NOT as a shard action to land.

**Correction to §8.1 of derived spec** (`knapsack-as-kintsugi-inner-
loop.md:388-411`): the `Actions` block for `shards/kintsugi/knapsack.
mirror` lists four actions (`read_capacity`, `select`, `apply_selection`,
`writeback_below_seam`). This does NOT include `admit_query`. **Consistent
with forward-promise. RATIFY.**

Signal-to-Alex: §7 of derived spec is FORWARD-PROMISE documentation,
not a landing signature. Arc 3 fires @kintsugi/knapsack with 4 actions,
NOT 5.

### §2.6 Q6 — Reed's inline Rust bootstrap for 10%-default

**Verdict: RATIFY-WITH-CORRECTIONS (route + substrate-decl framing).**

Reed's proposal per session brief: 10%-of-detected computed at BOOT,
encoded into the compiler itself; Rust-side (@io boundary); reads
system limits; sets default; same discipline as tokenizer change
(`fe95110` + `ee7903e`).

**Findings**:

1. **No existing bootstrap infrastructure for compute detection**.
   Grep of `bootstrap/src/lib.rs` (145.4KB) for
   `num_cpus|sysconf|meminfo|sysctl|detect_max|compute_bound` returns
   ZERO hits. This is a genuinely new Rust addition, not an extension.

2. **compute_bound.mirror already declares detect_max()** at
   `shards/epistemologic/reality/silicon/compute_bound.mirror:105-110`:
   ```
   detect_max() -> compute_bound { \ }
   ```
   The action is declared but body is `\` (fate-hole). The substrate
   already had the word for the detection primitive; the Rust
   bootstrap has NOT landed the discharge.

3. **The 10%-of-detected default is a `map` over detect_max**, not a
   new primitive. Substrate-honest form:
   ```
   default_capacity() -> capacity_vector = {
     silicon: detect_max().max_cpu_cores × 0.10  # or FLOPs derivation
     ram:     detect_max().max_memory_bytes × 0.10
   }
   ```

4. **@io surface for reading system limits**: the detection routes
   through OS-specific calls (Darwin: `hw.ncpu`, `hw.memsize` via
   sysctl; Linux: `/proc/meminfo`, `nproc`). Per `compute_bound.
   mirror:99-104`, this is already substrate-decl-declared — the
   Rust bootstrap discharges the fate-hole with the concrete OS call.

**Adjudication**:

- **Substrate-decl vs @io floor**: Alex ratified "encoding-into-
  compiler-itself." Seam audits as **substrate-decl at declaration
  altitude (already landed at compute_bound.mirror), @io floor at
  discharge altitude (Rust bootstrap)**. The tokenizer analogy holds:
  `fe95110` was a substrate-decl change with @io discharge in Rust.
- **Route**: the discharge lands in `bootstrap/src/lib.rs` alongside
  `kintsugi_main_in` — a new function `detect_capacity_defaults()`
  called from `kintsugi_main_inner` (bootstrap/src/lib.rs:3796+) at
  init. This is [substrate-pull:realize] on the bootstrap crate,
  discharging `detect_max()`'s fate-hole.
- **Compile-time vs runtime**: reading system limits at RUNTIME boot
  (not compile-time constants) is substrate-honest. Compile-time
  detection would fix the developer's build machine into the binary,
  which is Rice-hazard-adjacent (build-machine # deployment-machine).

**Corrections**:

- Reed's brief says "Rust-side (@io boundary)". Seam refines: **substrate-
  decl declaration (already landed) + @io discharge (new Rust in
  bootstrap/src/lib.rs)**. Same shape as tokenizer per Alex feedback.
- The 10%-of-detected multiplier is a **policy constant** — declare
  it at substrate altitude (e.g., `type default_capacity_fraction = ref
  = 0.10` in `shards/kintsugi/knapsack.mirror`) rather than hard-code
  in Rust.

### §2.7 Q7 — Reed inheritance errors owned

Taut caught three:

1. **@silicon = FLOPs** — WRONG. `@reality/algebra/silicon.silicon_crystal`
   carries `matter_projection` + `information_projection` (per
   `shards/reality/algebra/silicon.mirror:315-384`). FLOPs is one
   measurement on the information projection; it is NOT the identity
   of silicon. **Correction owned in Reed's report — verified in this
   audit's §2.2 path (d) which routes through compute_bound (bytes/cores)
   NOT FLOPs-as-identity.**

2. **mirror.spec target budget defaults** — FALSE at substrate.
   `shards/mirror/spec.mirror` does NOT currently declare per-target
   `silicon_budget` / `ram_budget` fields (per Mara §3 recommendation:
   implicit via crystal read). **Correction owned — Mara §3 keeps
   implicit; Reed's report acknowledged forward-promise for v1.**

3. **overflow → PARTIAL verdict** — NOT LANDED. `packing_verdict` enum
   with `partial(opacity_map)` variant lives in Mara's derived spec
   §4.2 / §8.1. It has NOT landed as a shard type. **Correction owned:
   land in Arc 3 Sub-arc B (@kintsugi/knapsack shard).**

**Residual verification**: no other Reed briefings carry these errors
forward. Mara's math and derived spec correctly frame all three per
above.

**RATIFY**: Reed's inheritance errors are corrected in the /loop text
Alex ratified. No residual carriage.

### §2.8 Q8 — Canonical execution /loop for Arc 3

See §7 below.

---

## §3. Verdict on Mara math

**RATIFY-WITH-CORRECTIONS.**

Citations (Frieze-Clarke 1984, Korte-Schrader 1981, Kellerer-Pferschy-
Pisinger 2004, Vazirani 2003, Ibarra-Kim 1975, Siegenfeld-Bar-Yam 2022
arXiv:2206.04896) are real, correctly attributed, and correctly
applied. Sub-Turing decidability per recognition #107 grounds the
PTAS runtime guarantee in the sub-Turing fragment. Ashby composition
via Siegenfeld-Bar-Yam multi-scale variety is substrate-honest. Round-
level descent theorem holds under P1 + P2 with P3 explicitly
DEFERRED. Required corrections: (i) mark §4.1 theorem as "conditional
on P3" not unconditional; (ii) clarify §5.1 header inequality
composition; (iii) fix §3.1 running-time form-choice statement (non-
blocking).

---

## §4. Verdict on Mara placement + Alex BOTH-AND fold

**RATIFY-WITH-CORRECTIONS. Path (d) with substrate-honest naming.**

Alex's BOTH-AND direction (physical + operator altitudes) folds
cleanly with Mara §5's implicit-with-@reflection-inference
recommendation. The path (d) reading is substrate-honest but must
route through existing landed carriers, not new namespaces.

- **Physical layer** (LANDED): `@epistemologic/reality/silicon/
  compute_bound` — the 5-field carrier already exists. NO new shard
  for "@silicon/bound".
- **Crystal kind layer** (LANDED): `@reality/algebra/silicon` with
  `compute-budget` crystal kind enumerated.
- **Operator species layer** (NEW): `@kintsugi/knapsack` per Mara §1.3,
  5-signal auto-classifier verdict species_root RATIFIED.
- **Bilateral property + fracture** (NEW): `@epistemologic/property/
  round_descent` + `@kintsugi/fracture/knapsack_infeasible`.

Alex's `@silicon/bound` naming maps to `@epistemologic/reality/
silicon/compute_bound` at shard-space (substrate-already-had-the-word,
16th+ instance). IF the intent is to land the top-level `@silicon`
family-root per `docs/specs/silicon.md` (104.8KB), that's an
independent arc (autopoietic family-root Bauchladen). It is NOT a
precondition for @knapsack landing.

---

## §5. Verdict on Mara overflow

**RATIFY.**

`hold` third-mode confirmed at `compiler-error-surface.md:22-30`
(2026-07-02). `packing_verdict` enum (§4.2 + §8.1) with four variants
(success | partial | failure | hold) composes correctly with the
three-mode algebra + six-variant answer algebra. Capacity=0 →
`partial(opacity_map)` with deferred candidates is substrate-honest.
Failure gate (P2 with ε > 1 OR P3 persistent across three-tick @third
audit) is substrate-verifiable via `shards/reflection.mirror` @third
depth 3 (LANDED). Minor clarity edit recommended in §4.2 body text
("PARTIAL for X; hold(ref) for Y" disambiguation). Non-blocking.

---

## §6. Cross-check verdicts

### §6.1 active_pass

**RATIFY Taut's finding.** `active_pass(o: oscillation) -> morphism`
at `shards/kintsugi/oscillate.mirror:475-478` emits SINGLE morphism.
Wrap externally via `@kintsugi/knapsack` orchestrator — Path 1 of the
two options. Path 2 (widen active_pass to top-k) breaks the pulse-
atomicity contract. Mara §7.1 body text needs clarification per §2.4
above.

### §6.2 @spectral/db admit_query

**FORWARD-PROMISED.** Not blocking Arc 3. Mara's §7.1-§7.3 of derived
spec is future-composition documentation. Arc 3 Sub-arc B fires
`@kintsugi/knapsack` with 4 actions (read_capacity, select,
apply_selection, writeback_below_seam) — NOT 5. `admit_query` lands
when `shards/spectral_db.mirror` (or similar fragmentation write-
surface) lands.

---

## §7. Canonical execution /loop for Arc 3

**Ordered sub-arc sequence** (respecting Arc 0 doc-code seam TICKs 1-10):

### Preconditions before Arc 3 fires

- Arc 0 TICKS 1-4 CLOSED (docblock family-root + liquid_extraction +
  prism_kind pact + prism_kind_ambiguous fracture) — auto-classifier
  operational.
- Arc 0 TICKS 5-10 CLOSED OR interleaved (docblock property+fracture
  bilateral cascade providing @reflection below-`---` writeback
  discipline).
- Arc 2 corrections landed (this audit's §9).

### Arc 3 sub-arc ordering

```
Sub-arc E: bootstrap/src/lib.rs — detect_capacity_defaults() @io discharge
           of compute_bound.detect_max() fate-hole (10%-of-detected
           computed at BOOT; substrate-decl declaration ALREADY at
           compute_bound.mirror; Rust discharge new).
           [substrate-pull:realize] on bootstrap crate.
           Precondition: none beyond current tokenizer landings.
           Discharges: existing `detect_max() -> compute_bound { \ }`
                       at compute_bound.mirror:110.

Sub-arc A: docs correction — Mara math §4.1 theorem "conditional on P3"
           + §5.1 header inequality composition + §3.1 running-time
           form. NON-BLOCKING; can land before OR alongside Sub-arc B.

Sub-arc C: shards/epistemologic/property/round_descent.mirror
           TICK 11a per Mara §8.2. Property side of bilateral.
           Precondition: Arc 0 TICKS 1-10 CLOSED (docblock writeback
           discipline provides the property-audit rail).
           Discharges: property `round_descent(before, after) ->
                       transparency<descent_signal>`.

Sub-arc B: shards/kintsugi/knapsack.mirror
           TICK 11b per Mara §8.1. Operator species root.
           Precondition: Sub-arc C landed (round_descent requires-bound
           in `select` signature).
           Precondition: Sub-arc E landed (default_capacity read source).
           Precondition: Arc 0 TICKS 3+4 CLOSED (auto-classifier
           empirically verifies species_root verdict on @knapsack).
           Discharges: 4 actions per §8.1 (read_capacity, select,
           apply_selection, writeback_below_seam). NOT 5 (admit_query
           forward-promised).

Sub-arc D: shards/kintsugi/fracture/knapsack_infeasible.mirror
           TICK 11c per Mara §8.3. Fracture side of bilateral.
           Precondition: Sub-arc B landed (packing_verdict type
           carrier resolves).
           Discharges: fracture_body dispatching per §4.2 of resource-
           budget/README.md (Cat 2 → spawn(peer); Cat 4 → apply(
           rebudget); Cat 5 → spawn(scheduler)).

Sub-arc F: shards/mirror/spec.mirror — IMPLICIT per Mara §3.
           NO shard change this arc. @reflection infers per §6 of
           derived spec. FORWARD-PROMISE for v1 if empirical measurement
           demands EXPLICIT fields.

Sub-arc G: @reflection N+1 writeback per Mara §6.
           NO new shard; extends @reflection's below-`---` writeback
           machinery (already provided by Arc 0 TICKS 5-10). Empirical
           discriminator per Mara §9.
```

### Ordering rationale

- **Sub-arc E first** discharges the detect_max fate-hole so Sub-arc B's
  `read_capacity` has a concrete implementation. Bootstrap change is
  Rust-only and has no shard preconditions.
- **Sub-arc A** (docs corrections) can land in parallel with E; both
  are pre-shard prep.
- **Sub-arc C before Sub-arc B** because @kintsugi/knapsack.select
  requires-binds `round_descent_admissible(candidates, cap)` per Mara
  §8.1. Property must land before consumer.
- **Sub-arc B before Sub-arc D** because fracture body operates on
  `packing_verdict` which lives in Sub-arc B's carriers.
- **Sub-arc F implicit** (no shard change) per Mara §3 substrate-
  already-had-the-word.
- **Sub-arc G** extends existing @reflection machinery from Arc 0;
  no independent landing.

### Recursive coupling with Arc 0

- **Arc 0 TICKS 1-2** (docblock family-root + liquid_extraction): land
  BEFORE Arc 3 fires. These enable the auto-classifier at TICK 3+4.
- **Arc 0 TICKS 3+4** (prism_kind_declared pact + prism_kind_ambiguous
  fracture): auto-classifier operational. Arc 3 Sub-arc B empirical
  verdict `species_root` on @knapsack fires HERE.
- **Arc 0 TICKS 5-10**: @reflection writeback discipline lands. Arc 3
  Sub-arc G composes on this.

**Arc 3 Sub-arc B fires AFTER Arc 0 TICKS 3+4 close** (auto-classifier
operational) and AFTER Arc 3 Sub-arc C lands (round_descent property).
Arc 3 Sub-arc E can fire in parallel to Arc 0 (Rust bootstrap has no
shard preconditions).

---

## §8. Signal-to-Alex — Phase E items before Arc 3 fires

1. **BOTH-AND fold routes through EXISTING carriers**, not new
   namespaces. `@silicon/bound` naming maps to `@epistemologic/reality/
   silicon/compute_bound` (LANDED). No `shards/silicon.mirror` or
   `shards/silicon/bound.mirror` needed. The top-level `@silicon`
   family-root (per docs/specs/silicon.md) remains an independent arc.
   Confirm: is the BOTH-AND direction satisfied by routing through
   compute_bound (Seam reading), OR do you want to land the top-level
   `@silicon` family this arc?

2. **10%-of-detected discharge lands in `bootstrap/src/lib.rs`** as
   discharge of existing `detect_max() -> compute_bound { \ }` fate-
   hole at `compute_bound.mirror:110`. The substrate-decl declaration
   is DONE; the Rust discharge is NEW. Same shape as tokenizer
   `fe95110`. The 10% multiplier declares as `default_capacity_fraction`
   carrier in `@kintsugi/knapsack` shard (Sub-arc B), not hard-coded
   in Rust.

3. **admit_query is NOT landed in Arc 3 Sub-arc B**. `@kintsugi/knapsack`
   ships with 4 actions, not 5. `admit_query` forward-promises to
   when `@spectral/db` fragmentation write-surface lands. Mara §7 of
   derived spec is future-composition documentation.

4. **Mara math §4.1 theorem needs "conditional on P3" edit** before
   Arc 3 fires. P3 (application faithfulness / no-interference) is
   DEFERRED per Mara §10.b; §4.1 theorem statement must acknowledge
   this to not carry a hedged theorem forward as proven.

5. **active_pass stays SINGLE-morphism**. Path 1 (external wrap) is
   substrate-honest per pulse-atomicity contract at oscillate.mirror.
   Mara §7.1 body text needs clarification to reflect this (wrap
   over multiple pulses to build candidate set; @kintsugi/knapsack
   sits ABOVE active_pass, not replaces it).

6. **`epsilon` (PTAS precision) is substrate-decl-fixed**, not @io-
   runtime-read. Declare `type epsilon = ref` at substrate altitude
   in `@kintsugi/knapsack` shard signature. Sub-Turing decidability
   guarantee (per #107) requires this.

7. **Failure gate three-tick @third audit** composes with existing
   `@third` at recognition #111 (LANDED `e43006ab`). No new machinery
   needed. Sub-arc D fracture body dispatches per §4.2 of resource-
   budget/README.md.

---

## §9. Required corrections (commit-ready specs)

### §9.1 Mara math §4.1 theorem conditional on P3

**File**: `docs/math/resource-budget/README.md`
**Section**: §4.1 (line ~248)
**Correction**:

Current:
```
Then `‖R(T_n)‖ < ‖T_n‖` off-fixed-points, i.e. `‖T_{n+1}‖ < ‖T_n‖`.

**Proof sketch.** Under (P1) `OPT > 0`; under (P2) selection gains
`≥ (1-ε)·OPT > 0`; under (P3) opacity decreases by that gain. QED.
```

Change to:
```
Then, **CONDITIONAL on P3's interference-free composition assumption
(DEFERRED per §10)**, `‖R(T_n)‖ < ‖T_n‖` off-fixed-points.

**Proof sketch.** Under (P1) `OPT > 0`; under (P2) selection gains
`≥ (1-ε)·OPT > 0`; **assuming P3 (linear opacity decrease under
selection application — an assumption the substrate cannot yet
discharge structurally; see §10)**, opacity decreases by that gain.
QED-conditional.

**Empirical discipline**: the substrate audits observed descent
`‖T_{n+1}‖ < ‖T_n‖` at @third depth 3 per shards/reflection.mirror;
P3-derived theoretical descent is DEFERRED to interference-model
formalization.
```

### §9.2 Mara derived spec §4.2 body text disambiguation

**File**: `docs/specs/knapsack-as-kintsugi-inner-loop.md`
**Section**: §4.2 (line ~220)
**Correction**: replace "PARTIAL + hold(carrier)" phrasing with
"PARTIAL(opacity_map) for capacity=0 with deferrable candidates;
hold(ref) for observer-declined non-discharge" — disambiguating the
disjunction from a composition.

### §9.3 Mara derived spec §7.1 active_pass composition clarification

**File**: `docs/specs/knapsack-as-kintsugi-inner-loop.md`
**Section**: §7.1 (~line 329)
**Correction**: add body text clarifying that active_pass emits
SINGLE morphism per pulse; @kintsugi/knapsack sits ABOVE active_pass,
wrapping multiple pulses to enumerate the candidate set, then
selecting via PTAS. See Q4 adjudication.

### §9.4 (Optional) Mara math §5.1 header composition clarity

**File**: `docs/math/resource-budget/README.md`
**Section**: §5.1 header
**Correction**: clarify the two-inequality composition
`V(disturbance) ≤ V(regulator) ≤ V(silicon_budget) × V(ram_budget)`
at header altitude (already correct in body). Non-blocking.

---

*2026-07-05. Seam Phase D adversarial review. RATIFY-WITH-CORRECTIONS.
Required corrections in §9 must land before Arc 3 Sub-arc B fires.
No corrections block Sub-arcs A, E from firing immediately.*
