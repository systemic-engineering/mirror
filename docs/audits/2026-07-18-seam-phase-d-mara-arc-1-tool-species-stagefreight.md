# Seam Phase D — Mara Arc 1 four-species-plus-RED-property landing `22c803a`

**Adjudicator:** Seam
**Landing under review:** `22c803a` (Mara, 2026-07-18) — StageFreight
Delivery Arc 1 in one composed commit; 5 shard mints; 2048
insertions; pure-substrate `.mirror`-only.
**Prior audit context:** Seam `2455ce6` (full-session Void+Tool+
Fractal closure); Seam `488d0f1` (post-closure tool-mint +
REED-INLINE cascades; #R-the-compiler-in-one-sentence second-witness
gate = 1.5 of 5 CLOSED at that snapshot).
**Ground state:** clean per Reed `8e95a65` (last REED-INLINE cascade
from Seam `488d0f1` landed pre-session).

**Verdict distribution:**

- **SHIP-CLEAN (4):** `shards/tool/go.mirror`,
  `shards/tool/docker.mirror`, `shards/tool/gitlab_ci.mirror`,
  `shards/cascade/code/rust/go.mirror`.
- **SHIP-WITH-REED-INLINE (1):** `shards/epistemologic/property/
  tool_species_stagefreight_witnessed.mirror` — one arity/requires-
  list bookkeeping discrepancy at §3 composed bilateral (arity 5 vs
  4 requires-clauses; see §5 below). Non-blocking docblock-cite
  fix, one line.
- **BLOCKED-ON-EVIDENCE (0).**

Session totals: 4 SHIP-CLEAN / 1 SHIP-WITH-REED-INLINE / 0 BLOCKED.

**One-sentence surprise:** Mara delivered the entire StageFreight Arc
1 spec-to-substrate transition in a single composed commit that
touches five different family-species altitudes yet requires only
one cite-level cascade fix — the discipline of authoring five shards
in one tick as if they were one shard is what made the audit tractable.

---

## §1 Substrate honesty (highest-priority gate)

**PASS.** Verified via four disjoint checks:

1. **Zero `.rs` files under `shards/**`** after the landing (`Search`
   `shards/**/*.rs` returns empty). The HARD RULE
   `feedback_no_rust_extension_shortcut` (Alex 2026-07-14) + Alex
   2026-07-16 8th-repetition `feedback_detector_inadequacy_answer_
   is_never_rust` is honored end-to-end.

2. **All action bodies `\`-blocked.** Confirmed on inspection: every
   `-> tool { \ }` / `-> verdict { \ }` / `-> go_source { \ }` /
   etc. across the five shards blocks realisation at the FLOOR
   boundary; nothing pretends to be executable at substrate-decl
   altitude.

3. **Marker discipline (per commit trailer or `[substrate-floor:
   @io-boundary]` audit gate):** the composed commit is pure-
   substrate `.mirror`-only. No marker required (Mara worked at
   substrate-decl altitude, not the Rust FLOOR); the substrate-
   discipline concern this arc's HARD RULE guards against (Rust
   extension shortcuts) does not apply. Post-Mara territory (Reed
   Arc 2-3 empirical firings) will carry the marker if the FLOOR
   discharges.

4. **Substrate-already-had-the-word audit clean.** Mara reports zero
   refusals this tick; I independently verified:
   - `@tool/go` DOES NOT EXIST prior; grep clean.
   - `@tool/docker` DOES NOT EXIST prior; grep clean.
   - `@tool/gitlab_ci` DOES NOT EXIST prior; grep clean.
   - `@cascade/code/rust/go` DOES NOT EXIST prior; grep clean
     (sibling `@cascade/code/rust/llvm` + `@cascade/code/rust/wasm`
     landed; go is FIFTH cascade landing under @cascade/code/rust/).
   - `@epistemologic/property/tool_species_stagefreight_witnessed`
     DOES NOT EXIST prior; grep clean (ninth instance of
     property/fracture bilateral pattern #53).

**Verdict:** substrate honesty CLEAN across all five landings.

---

## §2 @tool species altitude consistency (@tool/{go,docker,gitlab_ci} vs @tool/{cargo,git,nix} from `34ecd83`)

**PASS.** All three new @tool species follow the geometry established
in the earlier @tool/cargo + @tool/git + @tool/nix landings from
`34ecd83`. Concrete diff-audit per species:

### 2.1 Prism declaration shape

All three carry the same 5-op prism inheritance from @void basis:

```
prism @tool/go {
  focus tool_invocation
  project tool_invocation
  split tool_invocation
  shift tool_invocation
  settle tool_invocation
}
```

Identical shape at @tool/docker + @tool/gitlab_ci. Matches the
`shards/tool/cargo.mirror` template line-for-line at prism decl
altitude. Void's 5-op basis inheritance is intact per closure spec
§10.

### 2.2 Composed-bilateral requires-clause discipline

The `<species>_invocation_well_formed` composed bilateral in each
new species includes:

```
requires <species>_subcommand_admissible(invocation, invocation, p)
requires tool_invocation_admissible(invocation, invocation, p)
```

This mirrors `cargo_invocation_well_formed` at `shards/tool/cargo.
mirror:143-146` and is the LOAD-BEARING inheritance of family-root
admissibility. **All three new species inherit correctly.**

### 2.3 Composition-primitive naming convention

Per Alex 2026-07-18 ratified `feedback_composition_primitive_naming_
convention`:

- **@tool/go:** `go_workspace_target` (composition-primitive over
  (module_path, package_selector, build_tags)). PASS.
- **@tool/docker:** `docker_image_target` (composition-primitive
  over (dockerfile_path, build_context, tag, build_args,
  target_stage)); `docker_image_reference` (typed ref handle). PASS.
- **@tool/gitlab_ci:** `gitlab_ci_pipeline_config` (over (yaml_path,
  project_id, include_refs, variables)); `gitlab_ci_pipeline_handle`
  (typed handle); `gitlab_ci_pipeline_status` (closed variant). PASS.

Delightfully-boring naming discipline holding.

### 2.4 Subcommand-tag closed variant

Each species declares a `<species>_subcommand` closed variant naming
the FLOOR set of subcommands the substrate admits. Counts as
declared:

| Species          | Subcommand count | Notes                       |
|------------------|------------------|-----------------------------|
| @tool/go         | 11               | test/vet/build/mod_tidy/... |
| @tool/docker     | 10               | build/run/push/pull/...     |
| @tool/gitlab_ci  | 8                | lint/pipeline_create/...    |

FLOOR-only enumeration per closure discipline (extensible via mint;
each species explicitly forward-promises per-subcommand refinements
in §5 Forward-promised sections).

**Verdict:** altitude consistency CLEAN. All three new species are
geometry-perfect siblings of the three earlier @tool species.

---

## §3 @tool/docker four-altitude partition (surprise #1)

**PASS — the fourth altitude IS load-bearing, not over-fragmentation.**

Mara's spec §8 surprise #1 names the four altitudes explicitly.
Independent altitude verification:

| # | Altitude                     | Shard                                | Existed prior? | Role                                    |
|---|------------------------------|--------------------------------------|----------------|------------------------------------------|
| 1 | Form-side declarative code   | `shards/code/docker.mirror` (18.2KB) | LANDED         | Dockerfile-as-declarative-code grammar   |
| 2 | Mechanism-side transport     | `shards/io/oci.mirror` (25.6KB)      | LANDED         | OCI-format wire + distribution contract  |
| 3 | Form-side runtime            | `shards/container.mirror` (+species) | LANDED         | Container image handles + runtime spawn  |
| 4 | Porcelain-CLI invocation     | `shards/tool/docker.mirror` (14.5KB) | THIS TICK      | @tool.exec specialization for docker CLI |

The four altitudes are ORTHOGONAL:

- @tool/docker DISPATCHES through @io/oci (build path) + @container
  (run path); it does NOT replace either.
- @tool/docker's `docker_image_reference` carrier COMPOSES with
  @io/oci's `oci_manifest` (porcelain-readable ref vs content-
  addressed digest); the two are witnesses at different altitudes,
  not competitors.
- @code/docker sits at grammar altitude (Dockerfile IS declarative
  code); @tool/docker sits at invocation altitude (docker CLI IS a
  process wrapper). Different family-roots (@code vs @tool);
  different carriers.

**Could @tool/docker collapse into @io/oci?** No. @io/oci carries
the wire-format contract (manifest schema, distribution API);
@tool/docker carries the CLI-invocation contract (subcommand
grammar, args vector, exit codes). Collapsing them would erase
Recognition #55 form/process partition at the docker altitude —
the invocation is process-side; the OCI manifest is form-side. The
partition is load-bearing.

**Verdict:** four-altitude partition is substrate-honest;
Recognition #55 discipline holding. Mara's §8 surprise #1 correctly
identifies the risk (collapse into any of the three prior altitudes)
and correctly resolves it (name the fourth explicitly).

---

## §4 @tool/gitlab_ci multiplex (CLI ↔ REST fallback)

**PASS — the multiplex is substrate-honest.**

Mara's docblock (`shards/tool/gitlab_ci.mirror:66-71`) names both
dispatch paths explicitly at substrate-decl altitude:

```
exec(subcommand, args, invocation) -> tool
  ├─ `glab` binary present + pinned  ─▶ @io.exec("glab", ...)
  └─ `glab` absent OR REST preferred ─▶ @io/http.post/get/put(...)
```

The multiplex is NAMED (not hidden), MULTIPLEXES AT DISPATCH TIME
(not at type-decl time), and DISCHARGES THROUGH @io (both paths).
Substrate-honesty verified via three checks:

1. **Both paths compose over @io.** Neither path authors a spawn
   primitive; the CLI path composes `@io.exec("glab", ...)`; the
   REST path composes `@io/http.post/get/put(...)`. Conjunct 2 of
   the RED property (`discharge_through_io`) is satisfied.

2. **The multiplex decision is species-body-time, not compile-
   time.** The `species-body composes @tool/nix.resolve_pin for
   glab presence check` note (`shards/tool/gitlab_ci.mirror:73-74`)
   names WHERE the decision happens (dispatch-time via nix-pin
   resolution). The substrate reads the decision from a
   substrate-decl'd query, not from opaque runtime state.

3. **The @io/http forward-promise is named.** `shards/tool/gitlab_
   ci.mirror` §5 forward-promise 1 explicitly names `shards/io/
   http.mirror` as the mechanism-side @io species the REST-fallback
   multiplex composes over. Currently forward-promised; the CLI
   path composes over the LANDED @io surface in the interim; the
   REST path defers to the forward-promised @io/http species.

**Concern I considered and cleared:** could the multiplex hide
behavioural difference between CLI and REST paths (e.g., the CLI
might strip newlines that REST preserves; the REST API might
require different auth than CLI)? Mara's shard NAMES the multiplex
in the docblock; the invariant that `gitlab_ci_invocation_well_
formed` MUST discharge identically across the two paths is
substrate-decl'd at the composed-bilateral altitude. Downstream
concern: at empirical firing time (Reed Arc 2-3), if CLI and REST
paths return byte-different `tool_result` values for the same
`(subcommand, args, invocation)` input, that's a Reed-territory
bug to surface via the `gitlab_ci_invocation_well_formed`
bilateral; the substrate-decl side has done its job.

**Verdict:** multiplex is substrate-honest. Deferred concern for
Reed Arc 2-3: verify CLI vs REST path equivalence at empirical
firing time; if divergence surfaces, either (a) refine the
bilateral to admit path-specific verdict OR (b) collapse to a
single dispatch path per @io/http/glab-adapter.

---

## §5 @cascade/code/rust/go inline carriers (composition-forced-by-precedent)

**PASS — inline carrier declaration is substrate-honest per §5.1
composition-forced-by-precedent.**

Mara had to declare `go_source`, `go_emission_metadata`, `go_
artifact` inline at `shards/cascade/code/rust/go.mirror:314-355`
because `@code/go` does not exist yet at
`shards/code/go.mirror`. Independent verification via three checks:

1. **Sibling precedent LANDED.** `shards/cascade/code/rust/llvm.
   mirror:181-208` declares `rust_source`, `llvm_emission_
   metadata`, `llvm_artifact` inline WITHOUT a prerequisite `@code/
   llvm` target-grammar shard. Same discipline, same shape, landed
   at previous Mara tick. Mara's `shards/cascade/code/rust/go.
   mirror:81-93` cites this precedent explicitly:

   > Per the sibling `@cascade/code/rust/llvm` :193 `llvm_emission_
   > metadata` carrier declared inline without target-grammar-shard
   > prerequisite. Same discipline holds.

2. **`rust_source` triplicated across the three Rust-source
   cascade species** (rust/llvm + rust/wasm + rust/go). Mara names
   the triplication explicitly at `shards/cascade/code/rust/go.
   mirror:308-312`:

   > SHARED SHAPE with the sibling ... AND `shards/cascade/code/
   > rust/wasm.mirror` :123 rust_source carrier; intentional
   > triplication at substrate-decl altitude — the property is the
   > same and discharges identically across the three sibling
   > cascades from Rust source.

   The triplication IS honest: three separate species-decl files
   naming the same carrier at species altitude, not one shared
   file leaking across three altitudes. Two-tick collapse
   forward-promised implicitly (if the shape drifts across the
   three cascades, the substrate will re-tick to consolidate).

3. **`@code/go` forward-promise explicitly named.** `shards/
   cascade/code/rust/go.mirror` §5 forward-promise 1 (line 550-
   555) names `shards/code/go.mirror` as the target-grammar family-
   species mint. When landed, `in @code/go` import can replace the
   inline `ref` carriers here.

**Concern I considered and cleared:** should @code/go mint FIRST
(before the cascade species)? Mara's answer, sibling-precedent-
backed: no — the cascade species is the vehicle that motivates
@code/go's shape, and inline `ref` declaration + forward-promise
+ sibling triplication is the substrate-honest bridge until the
target-grammar shard lands. The alternative (blocking cascade
species mint on @code/go landing first) would fragment Arc 1 and
break StageFreight delivery arc cadence for a discipline reason
that Mara's own sibling `@cascade/code/rust/llvm` already
demonstrated is not load-bearing.

**Verdict:** inline carrier composition-forced-by-precedent is
substrate-honest and matches the landed sibling shape.

---

## §6 `cascade_rust_go_preserves_semantics` modulo clause (goroutine nondeterminism)

**PASS — sentinel is Rice-safe and substrate-defensible.**

Mara's sentinel:

```
sentinel "cascade=preserves-runnable-semantics-modulo-reintroduction"
```

Verified across three axes:

1. **Rice-safety per Mara `701828a`.** The full semantic-equivalence
   question ("does Rust source S execute observationally-identically
   to Go source G(S) across ALL runs?") is undecidable in the general
   case — Rice's theorem forbids substrate-side decision on this
   nontrivial semantic property. The sentinel-match at content-
   addressed altitude ("this specific `go_artifact` OID pairs with
   this specific `rust_source` OID under this specific cascade
   discharge, and the pair carries the reintroduction-axis
   annotation") is DECIDABLE at byte-level. The substrate honestly
   discharges the decidable check; the undecidable question is
   NAMED (not hidden) via the "modulo-reintroduction" phrase.

2. **The reintroduction axis is substrate-decl'd.** `shards/
   cascade/code/rust/go.mirror:127-141` names the reintroduction
   axis explicitly:

   > REINTRODUCED: goroutines, GC, defer/panic/recover, duck-typed
   > interface satisfaction.

   Goroutine nondeterminism IS a REINTRODUCTION dimension; the
   sentinel says "preserved MODULO this named axis," which is
   substrate-honest naming rather than hidden non-determinism.

3. **Sibling precedent.** `shards/cascade/code/rust/llvm.mirror`
   has a matching preserves-semantics bilateral with a modulo
   clause for LLVM's less-nondeterministic target; the shape is
   established at sibling altitude. Mara's addition of the
   reintroduction-axis explicit reference in the docblock is a
   refinement (not a violation) of the established pattern.

**Concern I considered and cleared:** does "modulo reintroduction"
smuggle in a get-out-of-jail clause that lets any behavioural
divergence be attributed to "reintroduction axis"? No — the
reintroduction axis is a CLOSED enumeration at `shards/cascade/
code/rust/go.mirror:127-141` (goroutines / GC / defer_panic_
recover / duck-typed interfaces). If a divergence surfaces that
does NOT fit these four axes, the bilateral must FAIL, not
degrade-to-modulo. Substrate-decl'd closed-set discipline
prevents the failure mode.

**Verdict:** modulo clause is Rice-safe, substrate-honest, and
sibling-consistent.

---

## §7 DEFERRED-RED conjunct 4 (`tool_invocation_signed_at_alex_root`)

**PASS with observation — deferral pattern is acceptable AND the
substrate already tracks it correctly via the `requires`-clause
omission.**

Mara's composed bilateral at `shards/epistemologic/property/tool_
species_stagefreight_witnessed.mirror:379-392`:

```
bilateral tool_species_stagefreight_witnessed {
  sentinel "property=tool-species-admissible-io-fate-signed-witnessed"
  arity 5
}
tool_species_stagefreight_witnessed(species: tool_id, p: perturbation)
  -> verdict
requires tool_invocation_admissible_inherited(species, p)
requires discharge_through_io(species, p)
requires fate_biased_execute(species, p)
requires tool_result_witnessed_content_addressed(species, p)
{ \ }
```

**Observations:**

1. **The four active requires-clauses omit conjunct 4** per Mara's
   docblock §4 forward-promise 1:

   > Conjunct 4 requires-clause promotion — currently omitted from
   > the composed predicate's requires-list because @trust family-
   > root has not landed. NEXT MARA TICK: @trust lands; the
   > requires-clause inserts here without shape change.

   This is substrate-honest: the substrate cannot require a check
   that has no discharge surface (the @trust family-root doesn't
   exist yet; requiring it would create a permanently-failing
   predicate, which is worse than an admissibly-deferred one).

2. **The uniform deferral pattern across all six @tool species**
   (cargo, git, nix, go, docker, gitlab_ci) at the table in
   `shards/epistemologic/property/tool_species_stagefreight_
   witnessed.mirror:201-208` correctly reflects the fact that
   conjunct 4 is a family-root-level gap, not a per-species gap.
   Fixing it once (when @trust lands) fixes it for all six species
   simultaneously. Cross-species deferral discipline is coherent.

3. **The deferral is tracked at TWO altitudes.** (a) at the shard
   docblock §4 forward-promise 1 (Mara's canonical spec-level
   promise); (b) at the arity-vs-requires-clause bookkeeping (see
   §5 REED-INLINE below).

**Should the deferral be tracked with an explicit forward-promise
carrier?** The current tracking (docblock §4 + `arity 5` sentinel
naming the FIVE conjunct property + FOUR active requires-clauses)
is substrate-honest but the arity/requires-clause mismatch is a
minor bookkeeping wart — see §8 REED-INLINE below for the one-line
docblock cite fix. No new carrier is needed; the existing
mechanism (docblock + arity + requires-list-omission) is
sufficient.

**Verdict:** deferral pattern is ACCEPTABLE and substrate-honest.
Cascade tracking: when @trust lands (next Mara tick), the requires-
clause inserts here; this substrate is prepared for the change
without shape drift.

---

## §8 One REED-INLINE (small, docblock-cite fix)

**File:** `shards/epistemologic/property/tool_species_stagefreight_
witnessed.mirror`
**Location:** §3 The composed property (lines 379-392)

**Issue:** the bilateral declaration reads `arity 5` (correctly
counting all five sub-conjuncts as members of the composed
property), but the `requires`-list has only FOUR clauses (conjunct
4 omitted per §4 forward-promise 1). A reader without §4 context
sees an arity-5 declaration with 4 requires-clauses and interprets
it as a bookkeeping error.

**Fix (one-line docblock addition in §3 or as a `# NOTE:` comment
inside the bilateral block):**

Add a `# NOTE:` comment inside the bilateral declaration naming the
DEFERRED-RED conjunct 4 explicitly:

```
bilateral tool_species_stagefreight_witnessed {
  sentinel "property=tool-species-admissible-io-fate-signed-witnessed"
  arity 5
}
# NOTE: `requires` list has 4 clauses (not 5) because conjunct 4
# `tool_invocation_signed_at_alex_root` is DEFERRED-RED pending
# @trust family-root landing next Mara tick per §4 forward-promise 1.
# arity remains 5 (the property predicates over all five conjuncts);
# the `requires` clause for conjunct 4 inserts here without shape
# change when @trust lands.
tool_species_stagefreight_witnessed(species: tool_id, p: perturbation)
  -> verdict
requires tool_invocation_admissible_inherited(species, p)
...
```

**Non-blocking.** The docblock at §4 already names the deferral;
this REED-INLINE lifts it to the immediate reading site so the
arity-vs-requires-clause count discrepancy is inline-annotated.
Pure-docs change; zero shape impact.

---

## §9 `#R-the-compiler-in-one-sentence` five second-witness gates — post-`22c803a` snapshot

Per closure spec §7.3 (Reed cascade `9aa6a52`) the five empirical
firing surfaces are:

| Gate | Firing | Landed pre-`22c803a`? | Advanced by `22c803a`? | Post-`22c803a` verdict |
|------|--------|-----------------------|------------------------|------------------------|
| 1 | `prismqueer::void::LiquidVoid<T>` GREEN (3 property tests) | ❌ NO | ❌ NO | Reed Arc-2A territory pending |
| 2 | `shards/tool.mirror` + FLOOR species shard-decls | ✅ CLOSED (via `34ecd83`, 4 shards) | Extended to 6 shards + 1 property | CLOSED (stable) |
| 3 | `mirror.spec` `tools { }` block parses at grammar altitude | 🟨 HALF (prose-hint via `67e8629`; grammar-mutation Reed Tick M3) | ❌ NO | HALF-CLOSED (stable) |
| 4 | First `@roomba.walk` empirical firing (`@tool(cargo,...)` → `@io/cargo.exec` → signed `tool_result`) | ❌ NO | ❌ NO | Reed Arc-2A territory pending |
| 5 | `mirror index .` after step 4 shows @coherence rise | ❌ NO | ❌ NO | Reed Arc-2A/2B territory pending |

**Post-`22c803a` count: 1.5 of 5 CLOSED (unchanged from Seam
`488d0f1` snapshot).**

Why unchanged? Arc 1 landed FOUR new species (@tool/go + @tool/
docker + @tool/gitlab_ci + @cascade/code/rust/go) PLUS a RED
property, but none of these are among the FIVE second-witness
gates named in closure §7.3. The gates are specific empirical
firings (LiquidVoid<T> GREEN, tools{} grammar, @roomba first
firing, @coherence rise) that Reed's Arc 2-3 territory discharges.

**Observation:** Arc 1 landings EXPAND the substrate surface that
second-witness Gate 4 (`@roomba.walk` empirical firing) will
eventually discharge over. When Gate 4 closes, it will close
against a species-decl surface that includes @tool/go + @tool/
docker + @tool/gitlab_ci (three additional species vs the four
present when Gate 4 was named). This is substrate-cadence-healthy:
the gate hasn't moved; the surface the gate discharges over is
wider. Gate 4 closure at Reed Arc-2A will therefore witness
BROADER dispatch coverage than Gate 4 as originally named.

**No gate closure advancement; no gate regression.** Stable at 1.5
of 5.

---

## §10 `#R-the-compiler-delivers-across-languages` — Arc 1 landing witness

**PASS — Arc 1 landing does NOT close first-witness for the cross-
language recognition.**

Per StageFreight spec §11:

> First-witness gate: open until Reed's Arc 3 Tick 15 lands and
> Marcus receives the PR. First witness IS Marcus's receipt +
> acknowledgment of the PR. Alex's public commitment (§0) IS the
> substrate-decl'd first-witness anchor.

The first-witness gate is bound to Marcus's receipt of the PR, not
to any substrate-decl landing. Arc 1 (this tick) discharges spec
§7.1 Ticks 1-5 — species mints + RED property. Arc 3 Tick 15 (Reed
territory, forward-promised) is where Marcus receives the PR;
first-witness gate closes at that event.

**Arc 1 landing advances the recognition's SUBSTRATE-DECL
INFRASTRUCTURE but does NOT close the first-witness gate.** The
substrate now HAS the species (@tool/go + @tool/docker + @tool/
gitlab_ci) + cascade (@cascade/code/rust/go) + RED property that
StageFreight PR delivery composes over. The recognition remains at
CANDIDATE strength per StageFreight spec §11:

> Recognition status: CANDIDATE this tick; promotion path staged
> through StageFreight PR delivery.

Substrate-honest at candidate strength; promotion track staged;
no premature promotion.

**Verdict:** first-witness gate for `#R-the-compiler-delivers-
across-languages` remains OPEN post-`22c803a`. Arc 1 is
infrastructure; Arc 3 Tick 15 is the witness event.

---

## §11 Cross-shard alignment findings

Two cross-shard alignment observations, both PASSING:

### 11.1 `shards/tool.mirror` tool_id variant already forecast the new tags

`shards/tool.mirror:213-225` (family-root landed via `34ecd83`) had
already declared `docker`, `go`, and `gitlab_ci` as
forward-promised tags in the `tool_id` closed variant BEFORE this
tick:

```
| docker            # forward-promised (StageFreight delivery, Mara this tick)
| npm               # forward-promised species
| pip               # forward-promised species
...
| go                # forward-promised (StageFreight delivery, Mara this tick)
| gitlab_ci         # forward-promised (StageFreight delivery, Mara this tick)
```

Arc 1 landings DISCHARGE the three forward-promised tags at species
altitude. The forward-promise is resolved (tags now have concrete
species shards); the tool_id variant enumeration remains unchanged
(closed-set discipline: the tags stay, the species shards land as
their body). Substrate cadence: forward-promise → substrate-decl'd
placeholder → species landing, in three cadences. Clean.

**Conjunct 3 of the RED property (`fate_biased_execute`) predicates
over exactly this variant-membership check** and is
substrate-decl-decidable per the RED property's §2 tool_id-in-
variant-tag verification path.

### 11.2 `@cascade/code/rust/go` composes with `@tool/go` at empirical altitude, not at substrate-decl altitude

`shards/cascade/code/rust/go.mirror:229-251` names the composition:

```
@cascade/code/rust/go.apply_rust_go
  → Go source (property-test corpus in Go grammar)
  → @tool/go(test, [./...]) empirical run
  → tool_result signed via @trust
```

The two species compose at EMPIRICAL FIRING altitude (Reed Arc
2-3), not at substrate-decl altitude. This is correct: @cascade
species and @tool species are DIFFERENT family-roots (@cascade vs
@tool); they cannot compose at family-root altitude. They compose
at the discharge boundary where cascade emits source and tool
invokes the compiler against it.

**This is also correctly reflected in the RED property's docblock:**

> @cascade/code/rust/go composes ADJACENT to this property (not IN
> it, since @cascade species are NOT @tool species — different
> family-root).

Substrate-decl altitude discipline holding. Cross-family composition
at empirical altitude is the correct pattern.

---

## §12 Q's for Alex (two, both non-blocking)

### Q1: Recognition tower doc timing for `#R-the-compiler-in-one-sentence` (RE-RAISED from Seam `488d0f1` §12 Q1)

**Same Q as Seam `488d0f1` §12 Q1; still open post-`22c803a`.** Gate
count unchanged (1.5 of 5); the Q Seam raised then is still the
Q now. Not urgent — the tower doc timing is a discipline question,
not a blocker. Seam lean remains: land tower doc when all 5 gates
close (per closure §7.0 explicit condition). No-fragmentation
guidance from Reed cascade continues to support "5 firings +
tower doc as ONE composed empirical-firing arc."

### Q2: Should conjunct 4 tracking migrate to an explicit forward-promise carrier when @trust lands?

Two Mara-authored ticks reference @trust:

- **This tick** (`22c803a`): conjunct 4 DEFERRED at all six @tool
  species; RED property omits `requires` clause for conjunct 4;
  cited at property §4 forward-promise 1.
- **Next Mara tick** (per closure spec §12.2): @trust family-root
  lands.

When @trust lands, the mechanical fix is: insert the fifth
`requires` clause into the RED property's composed bilateral;
remove the deferral note. Simple.

**Alex: does this deserve an explicit substrate-decl'd `forward_
promise` carrier at the RED property, or is the docblock §4 +
arity-vs-requires-list bookkeeping sufficient?**

Seam lean: docblock + arity + REED-INLINE cite fix (per §8 above)
is sufficient. The forward-promise pattern is honored via
established substrate mechanism (docblock §4 forward-promised
sections); adding a first-class carrier for "pending @trust land"
would be over-engineering for a one-tick gap.

**Neither Q blocks the landing.** Both are load-bearing discipline
questions for downstream cadence planning.

---

## §13 Recommended cascade order (~1 line pure-docs REED-INLINE; blocks nothing)

The ONE REED-INLINE from §8 (docblock-cite fix at `shards/
epistemologic/property/tool_species_stagefreight_witnessed.mirror`
§3) can land at Reed's next cascade tick. It's a `# NOTE:` comment
addition inside a single bilateral block; zero shape impact; pure-
substrate `.mirror` change that qualifies for the 📝 markdown-only
bypass IF Reed extracts it to a docs-only cascade note, OR falls
under `[substrate-floor:@io-boundary]` marker discipline as a
pure-docblock touch to substrate shard file (audit citation:
THIS shard, §8).

**No blocking cascades. Ground is clean for Reed empirical firing
(Arc 2 Tick 6-10).**

---

## §14 Summary table

| Landing                                              | Substrate honesty | Altitude consistency | Novel-audit finding                 | Verdict                    |
|------------------------------------------------------|-------------------|----------------------|--------------------------------------|----------------------------|
| `shards/tool/go.mirror`                              | CLEAN             | CLEAN                | 11-tag FLOOR mirrors cargo shape    | SHIP-CLEAN                 |
| `shards/tool/docker.mirror`                          | CLEAN             | CLEAN (4th altitude) | Fourth altitude explicitly named    | SHIP-CLEAN                 |
| `shards/tool/gitlab_ci.mirror`                       | CLEAN             | CLEAN                | REST+YAML multiplex substrate-honest| SHIP-CLEAN                 |
| `shards/cascade/code/rust/go.mirror`                 | CLEAN             | CLEAN                | Q1 self-closure to (a); modulo clause Rice-safe | SHIP-CLEAN     |
| `shards/epistemologic/property/tool_species_stagefreight_witnessed.mirror` | CLEAN | CLEAN (9th instance of #53) | Arity-5-vs-4-requires bookkeeping wart | SHIP-WITH-REED-INLINE (§8) |

**Session verdict:** 4 SHIP-CLEAN / 1 SHIP-WITH-REED-INLINE (small
docblock cite) / 0 BLOCKED-ON-EVIDENCE.

`#R-the-compiler-in-one-sentence`: second-witness gate 1.5 of 5
CLOSED (unchanged; surface expanded).
`#R-the-compiler-delivers-across-languages`: CANDIDATE (unchanged;
Arc 1 is infrastructure; Arc 3 Tick 15 is the witness event).

**Ground remains clean for Reed empirical firing (Arc 2 Tick 6-10).**

Author: Seam <seam@systemic.engineer>
