# Taut scout — rust/ floor Eigenobject empirical classification

**Date:** 2026-07-22
**Author:** Taut <taut@systemic.engineer>
**Task:** #316 (this scout) — spawned by Reed at Alex's direction after Eigenform-Stabilizer synthesis reached substrate-truth this afternoon (task #314; Mara's six-commit formalization). Read-only empirical measurement of Reed's four-convergence-point Eigenobject characterization.
**Method:** grep-first; `wc -l` on every `.rs` in `rust/src/` + `rust/fractal/src/` + `rust/singularity/src/`; per-file docblock reading; per-function classification for mixed files. No synthesis. No refactoring proposals. No adjudication — Seam's territory post-scout.

---

## §1 Empirical file inventory

Empirical `wc -l` on every `.rs` file in scope. Bucket assignment via docblock + function-level grep-classification. Files that are mixed (main.rs primarily) get per-function breakdown in §3.

| Path | Lines | Primary bucket | Notes |
|---|---:|---|---|
| `rust/src/phone.rs` | 1818 | **A (@io quarantine)** | ALL @io families landed: fs/git/bytes/socket. Also embeds implicit @io/process via `std::process::Command` inside git_add/git_commit_as/git_head_oid. Zero `unsafe extern "C"`. Includes 4 large `#[cfg(test)] mod *_prop_tests` blocks (~1290 LOC test scaffolding). |
| `rust/src/matrix.rs` | 1435 | **B (FLANG floor)** | Three prod functions (`eigenvalues`, `phase_lock`, `envelope`) at ~50 LOC total delegating to `prismqueer::ffi::*`. Remainder is `mod prop_tests` (~1187 LOC). NO `unsafe extern "C"` in-file — the FFI boundary lives at prismqueer, not matrix.rs itself. See §4 admissibility. |
| `rust/src/void.rs` | 438 | **D (K=0 basis)** | `SignatureBeat` carrier + `VoidBasisAxis` enum + `welcome_perturbation` + `compose_beat_entry` (~145 LOC prod) + `mod prop_tests` (~293 LOC). Composes over phone.rs + fractal::Subject. K=0 rust-altitude witness. |
| `rust/src/book.rs` | 298 | **C (Bootstrap kernel)** | 8-well-known `resolve(@<name>)` + `RegistryError` + `well_known_at_names()` (~130 LOC prod) + `mod tests` (~150 LOC). This IS the initial minimum registry resolver. |
| `rust/src/main.rs` | 1417 | **MIXED (C + E)** | Small C bootstrap kernel (~180 LOC: mod decls + main + at_operator dispatch scaffold + verb table + argv parse). Large E scaffolding (~1237 LOC: cmd_roomba + deposit_observation_crystal + sha256_hex + arm-collapse dispatch + cmd_compile + at_operator_tests). Per-function breakdown §3. |
| `rust/src/liquid.rs` | 2817 | **E (transient scaffolding)** | Property-runtime dispatch (~900 LOC prod: PropertyDecl, SpecProperty, extractors, Verdict, dispatch_property, dispatch_spec_property, ~5 pillar predicates) + property tests (~1900 LOC). Composes over prismqueer::liquid::pillar. Lift admissibility: E-liftable via @liquid family-root + pillar dispatch surface — see §4. |
| `rust/src/compile.rs` | 819 | **E (transient scaffolding)** | SAGA-chain-of-Crystals orchestration: PropertyDischarge, Escalation, Compilation, compile_declarations, compile_from_source, serialize_discharge (~300 LOC prod) + tests (~500 LOC). Composes fractal + liquid. Lift admissibility: E-liftable via SAGA composition primitives — see §4. |
| `rust/src/collapse.rs` | 999 | **E (transient scaffolding)** | Bilateral-arm collapse capability. BilateralDecl + RedundantArm + CollapseReport + load_bilateral_corpus + find_redundant_arms + apply_deletions. First substrate-delta surface birthed FROM the floor per docblock. Lift admissibility: E-liftable via `shards/kintsugi/roomba.mirror` walker primitives + apply_h::act — see §4. |
| `rust/fractal/src/lib.rs` | 47 | **D (K=0 basis)** | Module re-export shell. Substrate identity crate boundary. |
| `rust/fractal/src/subject.rs` | 275 | **D (K=0 basis)** | Subject envelope + SubjectKind enum + Subject::void()/mirror()/human()/peer() constructors + LiquidVoid impl. Identity-carrier per Alex 2026-07-18 direct-transcript. |
| `rust/fractal/src/witnessed.rs` | 161 | **D (K=0 basis)** | Author + Committer + Timestamp + Message + Witnessed (MARA doctrine). ~100 LOC prod + ~60 LOC tests. |
| `rust/fractal/src/mandelbrot.rs` | 146 | **D (K=0 basis)** | Oid content-address + Oid::GENESIS + Mandelbrot<T> parent trait + MandelbrotProvenance. |
| `rust/fractal/src/crystal.rs` | 226 | **E (transient scaffolding)** | Crystal<T> settled-state carrier + crystallize<T>. Genuinely ambiguous D/E: Crystal<T> is compile.rs's SAGA carrier (E-domain) BUT could be argued as ontological ground for content-addressed identity. Naming as E because it's the settled-state fragment consumed by SAGA orchestration; the ontological ground is Oid (kept in D via mandelbrot.rs). Seam adjudicates. |
| `rust/fractal/src/singularity.rs` | 322 | **E (transient scaffolding)** | Singularity<T> optics-hierarchy trait + Iso impl for Crystal<T>. Explicitly scoped by docblock as forward-promise scaffold for physics-research altitude (Lens/Prism/Traversal impls land in `rust/singularity/` crate). E-liftable via `shards/fractal/singularity.mirror`. |
| `rust/singularity/src/lib.rs` | 138 | **E (transient scaffolding)** | v0.1.0 scaffold; empty crate re-exporting fractal::Singularity + smoke tests only. Explicitly forward-promised (no physics research landed). E-classified as "scaffold"; the whole crate could be pruned or held per authorship boundary. |

**Total across all buckets:** 11,356 LOC (matches `wc -l` sum).

---

## §2 Per-bucket line counts

Empirical sums vs Reed's estimates:

| Bucket | Reed's estimate | Empirical count | Files |
|---|---:|---:|---|
| A (@io quarantine) | ~200-400 | **1818** | phone.rs |
| B (FLANG floor) | ~100-200 | **1435** | matrix.rs |
| C (Bootstrap kernel) | ~100-200 | **~478** | book.rs (298) + main.rs bootstrap-kernel functions (~180); see §3 |
| D (K=0 basis) | ~150-250 | **1067** | void.rs (438) + fractal/lib.rs (47) + subject.rs (275) + witnessed.rs (161) + mandelbrot.rs (146) |
| E (transient scaffolding) | ~4-5× Eigenobject | **~6558** | liquid.rs (2817) + compile.rs (819) + collapse.rs (999) + main.rs scaffolding (~1237) + fractal/crystal.rs (226) + fractal/singularity.rs (322) + singularity/lib.rs (138) |

**Eigenobject total (A + B + C + D):** **~4798 LOC** (Reed said ~550-1050; empirical is **~4.6-8.7×** Reed's estimate)

**Total rust/:** **11,356 LOC**

**Contraction ratio:** 11356 / 4798 ≈ **2.37×** (NOT the ~10× Reed asserted)

### Substrate-honest note on the ratio

Reed's ~10× assertion was miscalibrated because A + B contain enormous property-test scaffolding included in wc -l. If we count **production-only** (excluding all `#[cfg(test)]` bodies):

- A prod-only: ~525 LOC (phone.rs functions before mod fs_prop_tests at line 525)
- B prod-only: ~50 LOC (matrix.rs's three delegate functions before mod prop_tests at line 249)
- C prod-only: ~310 LOC (book.rs prod ~150; main.rs bootstrap-kernel prod ~160)
- D prod-only: ~500 LOC (void.rs prod ~145; fractal prod ~355)

**Prod-only Eigenobject:** ~**1385 LOC** — WITHIN Reed's stated envelope (~550-1050 upper edge; ~30% over).

**Prod-only total rust/:** ~4700 LOC (11356 − ~6650 test LOC by rough estimation of `#[cfg(test)]` blocks in each file).

**Prod-only contraction ratio:** ~4700 / ~1385 ≈ **3.4×**, still not 10× but substantially better-shaped than the LOC-total ratio.

**The 10× assertion is empirically not supported.** The real ratio is 2.4× (LOC-total) or 3.4× (prod-only). Substrate-honest naming: Reed inflated. Adjust Eigenobject planning to a 3-4× contraction target, not 10×.

---

## §3 Function-level classification within mixed files

### main.rs (1417 LOC total)

| Function | Lines | Bucket | Rationale |
|---|---:|---|---|
| module docblock | 1-51 | (both) | Frames both bootstrap + at_operator dispatch. |
| `mod` declarations | 52-71 | **C** | Bootstrap-kernel wiring; declares all sibling modules. |
| `use` + `VERSION` const | 72-77 | **C** | Bootstrap primitive. |
| `VERBS` const (11-verb table) | 78-104 | **E** | HARDCODED per docblock; retires at M2 reflective cli-block. CAN lift via `shards/mirror/lens/cli.mirror`. |
| `print_help()` | 111-134 | **E** | Presentation. CAN lift via cli.mirror reflection. |
| `print_version()` | 136-141 | **C** | Trivial; could stay or lift. |
| `enum FileKind` + `classify()` | 143-159 | **E** | Byte-check dispatch classifier. CAN lift via `shards/kintsugi/roomba.mirror` fracture table. |
| `cmd_roomba()` | 161-421 | **E** | Full walker + arm-collapse + pheromone-deposit dispatch. Largest single function (261 LOC). CAN lift via roomba.mirror walker primitives + apply_h::act combinator surface. |
| `deposit_observation_crystal()` | 423-547 | **E** | Pheromone deposit orchestration. CAN lift via shard-body composition over @io/fs.append. |
| `compose_pheromone_commit_message()` | 549-587 | **E** | Commit-message templating. CAN lift via `@nl.compose` (forward-promised in shards). |
| `current_utc_timestamp()` + `format_utc_iso8601()` + `is_leap()` | 589-643 | **E** | Time-formatting utilities. Genuinely needs shard-decl mint (@time/format species not yet landed). Note: Bucket E "needs shard-decl" — see §4. |
| `sha256_hex()` | 645-742 | **E** | SHA-256 self-contained impl (~98 LOC). CAN lift via `shards/spectral/signature.mirror` composition, but @hash/sha256 or @cascade/hash needs species-decl. **Genuinely ambiguous C/E**: is content-addressed OID hashing a bootstrap-kernel primitive (needed to compile mirror at all) or scaffolding (only used for observation-crystal signature)? Seam adjudicates. |
| `dispatch_arm_collapse()` | 744-804 | **E** | Composes collapse.rs + phone.rs. Full scaffolding. |
| `find_git_root()` | 806-823 | **E** | Utility walker. CAN lift via @io/fs shard body. |
| `compose_collapse_commit_message()` | 825-1018 | **E** | Templating. CAN lift via @nl.compose. |
| `at_operator()` (embedded in prior region) | scattered | **MIXED C+E** | Initial entry-point signature = **C**; 7-arm cascade over @io/fs.{list_dir,read,write,append,mkdir_p} + @io/git.commit = **E** per task ("the arms are Bucket E"). |
| `mod at_operator_tests` | 1020-1282 | **E** | Test scaffolding for at_operator dispatch (~263 LOC). |
| `cmd_compile()` | 1284-1362 | **E** | SAGA-chain-of-Crystals verb wiring. CAN lift once @-operator dispatch scaffold matures. |
| `main() fn` | 1364-1417 | **C** | argv parse + entry-point dispatch — this IS the minimum reflective evaluator entry. |

**Bootstrap-kernel portions of main.rs (Bucket C):** ~180 LOC (mod decls + main + at_operator initial dispatch entry-point + print_version + minimal argv routing). The 7-arm cascade INSIDE at_operator (@io/fs.list_dir, .read, .write, .append, .mkdir_p, @io/git.commit) is Bucket E arm-cascade over the initial entry-point per the task classification.

**Scaffolding portions of main.rs (Bucket E):** ~1237 LOC.

### book.rs (298 LOC total)

| Function | Lines | Bucket | Rationale |
|---|---:|---|---|
| `RegistryError` + Display impl | ~30 | **C** | Bootstrap-kernel error carrier. |
| `resolve()` — 8-arm well-known match | ~40 | **C** | The initial book.rs registry resolver. Per task: "Minimum book.rs bootstrap resolver primitive (just enough to resolve the first well-known @<name>; not the full 8-well-known map)." **8-arm map is genuinely ambiguous C/E**: task says "just the first well-known" is C; "full 8-well-known map" is E. Empirically the 8-arm impl exists as one match statement; splitting is Seam's territory. Marking whole file C because it's a single ~40-line dispatch; disambiguating one arm as C and 7 as E would be over-fine. |
| `well_known_at_names()` | ~15 | **C** | Enumeration for error messages. |
| `mod tests` | ~150 | **E** | Test scaffolding. |

### phone.rs (1818 LOC total; all A + test-scaffolding)

Production @io functions (Bucket A): ~525 LOC (lines 1-524, before `mod fs_prop_tests`).

Test scaffolding: ~1290 LOC (lines 525-1818: fs_prop_tests + git_prop_tests + bytes_prop_tests + socket_prop_tests).

All of phone.rs is Bucket A by task classification (@io quarantine including thin bindings). Test bodies stay classified A because they exercise the A-boundary; classifying tests as E would be inconsistent with task definition.

### matrix.rs (1435 LOC total; all B + test-scaffolding)

Production FLANG-delegate functions (Bucket B): ~50 LOC (lines 174-228: `eigenvalues`, `phase_lock`, `envelope`).

Test scaffolding: ~1185 LOC (lines 249-1435: `mod prop_tests`).

All of matrix.rs is Bucket B by task classification.

### void.rs (438 LOC total; all D + test-scaffolding)

Production K=0 basis (Bucket D): ~145 LOC (SignatureBeat + VoidBasisAxis + welcome_perturbation + compose_beat_entry).

Test scaffolding: ~293 LOC (`mod prop_tests`).

---

## §4 Scaffolding-lift admissibility per Bucket-E item

For each Bucket E item, is there LANDED substrate vocabulary that can express it as shard-body composition?

### E1: main.rs `VERBS` const + `print_help()` + argv verb dispatch beyond bootstrap (~130 LOC)

- Vocabulary: `shards/mirror/lens/cli.mirror` — LANDED (10.0KB; contains `prism @mirror/lens/cli`).
- **LANDED** — CAN lift immediately post-Seam via reflective cli-block reading (Mara §5.2 item 4 forward-promise at M2).

### E2: `cmd_roomba()` walker (~261 LOC in main.rs)

- Vocabulary: `shards/kintsugi/roomba.mirror` — LANDED (46.4KB; contains `prism @kintsugi/roomba` + 12 bilaterals: `walk_terminates_cleanly`, `tension_monotone_descending`, `coherence_gradient_admissible`, `knife_verdict_bounded`, `walk_witnessing`, `bump_witnessing`, `vacuum_admissible`, `gc_mark_terminal`, `pivot_admissible`, `pivot_reflection_composed`, `pivot_witnessing`).
- Vocabulary: apply_h::act 7-combinator surface — LANDED per task #140 (per session context).
- **LANDED** — CAN lift via walker primitives + apply_h::act. Currently the walker is instantiated in main.rs via direct calls to phone::list_dir_recursive + classify + arm-collapse dispatch. Lift is straightforward.

### E3: liquid.rs threshold dispatch full cascade (~900 LOC prod)

- Vocabulary: `shards/liquid.mirror` — LANDED (15.3KB; contains `prism @liquid`).
- Vocabulary: `prismqueer::liquid::pillar` — LANDED (Reed iter 1-10 arc per liquid.rs docblock).
- Vocabulary: `shards/mirror/spec/property.mirror` — LANDED (declared per liquid.rs SpecProperty docblock).
- **PARTIALLY LANDED** — The dispatch surface substrate exists (@liquid + pillar + property grammar). The Rust functions `dispatch_property`, `dispatch_spec_property` are shard-body composable BUT the Fiber<T>-sampling arms per Mara §2.3 rows 1-3+6 (algedonic / viability / general expression tree) are forward-promised at iter 4+ (per liquid.rs docblock). Full lift requires those §2.3 rows to land AND the Rondon-Kawaguchi-Jhala 2008 decidability grounding per Mara §3.2 (currently forward-promised).
- **Vocabulary gap:** general expression-tree parser per Mara §3.2 (not landed).

### E4: compile.rs SAGA-chain-of-Crystals orchestration (~300 LOC prod)

- Vocabulary: `fractal::Crystal<T>` + `fractal::Oid` + `fractal::crystallize` — LANDED at fractal/crystal.rs + fractal/mandelbrot.rs.
- Vocabulary: SAGA-chain composition primitives — need to check `shards/mirror/store.mirror` per task instructions (not read this scout; forward-promise).
- Vocabulary: apply_h combinator surface — LANDED per task #140.
- **PARTIALLY LANDED** — The Crystal + Oid substrate carriers exist. The SAGA-chain orchestration itself (walking decls in extraction order, escalation on first-fail, deterministic replay via witnessed) is well-defined algorithmically. Full lift requires substrate-decl'd SAGA composition combinator (verify `shards/mirror/store.mirror` — not verified this scout).
- **Vocabulary gap:** possibly a SAGA-orchestrate combinator (unverified).

### E5: collapse.rs bilateral-arm collapse (~500 LOC prod)

- Vocabulary: `shards/kintsugi/roomba.mirror` walker primitives — LANDED (see E2).
- Vocabulary: `shards/kintsugi/fracture/bilateral_arm_redundant.mirror` — LANDED per collapse.rs docblock composition anchor list.
- Vocabulary: `shards/epistemologic/pact/bilateral.mirror` — LANDED per docblock.
- Vocabulary: apply_h::act — LANDED per task #140.
- **LANDED** — CAN lift immediately post-Seam via bilateral typed carrier + fracture species + walker primitives + apply_h::act combinator surface. The Rice-safe byte-substring analysis is a shard-body composable primitive.

### E6: main.rs argv + CLI beyond bootstrap-primitive (~230 LOC scattered)

- Vocabulary: `shards/mirror/lens/cli.mirror` cli-block reflective evaluation — LANDED (see E1); reflective read of `mirror.spec` cli-block is Mara §2.2 M2 forward-promise.
- **PARTIALLY LANDED** — cli.mirror substrate exists; the reflective reader Rust-side lift is forward-promised at M2.

### E7: at_operator arm cascade beyond bootstrap-primitive (~180 LOC in main.rs)

- Vocabulary: apply_h::act 7-combinator surface — LANDED per task #140.
- Vocabulary: `shards/io.mirror` sub-species `@io/fs`, `@io/git`, `@io/socket`, `@io/network`, `@io/bytes` — LANDED per phone.rs docblock.
- **LANDED** — CAN lift via apply_h::act combinator surface. The current 6-arm dispatch (`@io/fs.list_dir`, `.read`, `.write`, `.append`, `.mkdir_p`, `@io/git.commit`) is a substrate-honest bilateral resolver-arm sentinel-check pattern.

### E8: fractal::witnessed doctrine — NOT scaffolding

Re-classified: fractal::witnessed IS the MARA doctrine carrier — Bucket D basis, not E. Included in D count above.

### E9: singularity/ crate (138 LOC)

- Vocabulary: `shards/fractal/singularity.mirror` — LANDED per singularity/lib.rs docblock (Mara `90f4d27`).
- **LANDED as scaffold** — the crate is explicitly forward-promised (no physics research landed). CAN be either pruned or held in current shape per authorship boundary; no vocabulary gap.

### E10: fractal/crystal.rs + fractal/singularity.rs (548 LOC combined)

- Vocabulary: `shards/fractal/crystal.mirror` + `shards/fractal/singularity.mirror` — LANDED per each file's docblock composition anchors.
- **LANDED** — Crystal<T> is genuinely ambiguous D/E (see §1 note); Singularity Iso is E-scaffold.

### E11: main.rs time-formatting utilities (`current_utc_timestamp`, `format_utc_iso8601`, `is_leap`) (~55 LOC)

- Vocabulary: `shards/uuid/spectral/time.mirror` — probably LANDED (per session's recent tick-collapse work; not verified this scout).
- **PROBABLY LANDED** — @time/now (mentioned in project memory `project_time_facet_admissible`) provides substrate for timestamp; format function needs @time/format species-decl mint (unverified).
- **Vocabulary gap:** possibly @time/format species-decl.

### E12: main.rs `sha256_hex()` (~98 LOC)

- Vocabulary: `shards/spectral/signature.mirror` — LANDED per collapse.rs docblock.
- **LANDED** — the substrate-decl'd content-address discipline is at spectral/signature.mirror. Lifting SHA-256 impl itself would need @hash/sha256 or @cascade/hash species-decl.
- **Vocabulary gap:** @hash/sha256 species-decl mint.

### Substantive vocabulary gaps (needs shard-decl mint)

1. **@io/process species** (see §5)
2. **@hash/sha256 or @cascade/hash species** (for sha256_hex lift)
3. **@time/format species** (for ISO-8601 formatting lift; possibly landed under uuid/spectral/time)
4. **@saga/orchestrate combinator** (unverified; possibly at shards/mirror/store.mirror)
5. **Expression-tree parser vocabulary** per Mara §3.2 (for liquid.rs dispatch_spec_property full arms)

---

## §5 The @io/process empirical gap

**Substrate-decl status at `shards/io.mirror`:**

- `shards/io.mirror:213-218` names `@io/process` in the docblock inventory: *"@io/process — subprocess execution surface. Lifts the floor's `exec` from `boot/std/io.mirror`."*
- `shards/io.mirror:305-309` names it in the substrate lineage: *"exec / process — UNIX fork+exec (Thompson, 1969). The process-creation discipline."*
- `shards/io.mirror:378-384` names it in forward-promise list: *"@io/llvm, @io/flang, @io/fs, @io/git, @io/process are forward-promised; lift-ticks land when..."*
- **NO `species @io/process` declaration** exists at `shards/io.mirror` (grep found zero matches for `species @io/process|species\s+process|@io/process\s+species`).

**Rust-side landing status at `phone.rs`:**

- Zero dedicated `@io/process` primitive functions in phone.rs (no `exec()`, `spawn_process()`, `execute_process()`).
- `std::process::Command` is USED implicitly inside 3 git functions: `git_add()` (~line 360), `git_commit_as()` (~395), `git_head_oid()` (~434). All three treat process-spawn as an internal detail of the @io/git family rather than a separate @io/process primitive.
- phone.rs docblock at lines 62-66 explicitly names the @io families landed (fs/git/bytes/socket) — process is **not** listed.

**Empirical state:** @io/process is a **GAP**. Substrate-decl'd in shards/io.mirror as forward-promise (named but not declared as a species). Rust-side has implicit `std::process::Command` usage but no dedicated @io/process primitives. Bucket A is **incomplete** without @io/process species-decl mint + phone.rs primitive lift.

This matches Reed's task memory: `@io/process is Mara's Tick 1 forward-promise per defc8ef minimal-gap spec`.

---

## §6 Cross-check: bootstrap-of-bootstrap

**Question:** Does the current rust/ admit a self-compiling reflective evaluator kernel? Or is it mirror-compiled-via-cargo only?

**Evidence:**

1. **`rust/src/compile.rs::compile_from_source(source, witnessed) -> Compilation`** DOES exist (verified §compile.rs §Minimum-viable this iteration). It walks bilateral property declarations + spec-body property declarations via `liquid::extract_properties` + `liquid::extract_spec_properties`; dispatches each through `liquid::dispatch_property` / `dispatch_spec_property`; produces a SAGA-chain-of-Crystals.

2. **`mirror compile <file>` verb IS wired** at main.rs::cmd_compile (verified lines 1284-1362). Delegates to compile::compile_from_source with mirror as author/committer.

3. **BUT the compilation output is NOT a compiled binary** — it's a **SAGA chain of PropertyDischarges** (Verdict::Pass/Fail/Defer per property). Per compile.rs::Compilation shape: `{ crystals: Vec<Crystal<Vec<u8>>>, discharges: Vec<PropertyDischarge>, escalation: Escalation }`. This is a **property-verification compilation**, not a Rust-source-compilation.

4. **`mirror.spec` file exists** at project root (verified via file listing). Presumably `mirror compile mirror.spec` would empirically fire (dispatch works, extractors work) and produce a SAGA chain of Verdicts against mirror.spec's bilateral + property declarations. This is Adzic *Specification by Example* form per the docblock — mirror.spec IS the fixpoint spec whose properties compile to Verdicts.

5. **Reflective evaluator for mirror-native source-compilation is NOT landed.** No evidence of a mirror parser (beyond byte-scanning extractors for `bilateral`/`property` blocks). No lambda_0 compiler. No craft target. The `craft` verb per main.rs::VERBS declares `"Settle a grammar directory to lambda_0 (target: binary)"` but is dispatched via the M3+ verb-not-wired path (main.rs::main returns ExitCode 2 with "substrate-decl'd but dispatch lands at M3+" message).

**Substrate-honest answer:** The Bootstrap Kernel (Bucket C) is **PARTIALLY landed as a property-verification-compiler, ASPIRATIONAL as a self-compiling-source-evaluator**.

- What IS landed: mirror compiles `.spec` + `.mirror` bilateral/property declarations to a SAGA chain of Verdicts. This IS the fixpoint-liquid runtime per Mara `docs/specs/2026-07-19-mirror-spec-is-the-fixpoint-liquid-is-the-runtime.md`.
- What is NOT landed: self-compilation of mirror source to a mirror binary. `craft` verb is substrate-decl'd but ExitCode 2 (M3+ forward-promise).

The Foerster-fixpoint chicken-and-egg residue is **partially discharged** at the Verdict-fixpoint altitude but **not yet closed** at the source-to-binary altitude. Reed's Bucket C characterization needs substrate-honest reframing: current rust/ is a **property-verification compiler**, not a self-compiling reflective evaluator.

---

## §7 One-sentence summary

The empirical rust/ contraction ratio is **2.4×** (LOC-total) or **~3.4×** (production-only, excluding property-test scaffolding) — NOT the 10× Reed asserted; the Eigenobject is **~4800 LOC** (LOC-total) or **~1385 LOC** (prod-only, within Reed's stated ~550-1050 envelope with ~30% over); the scaffolding is **~6560 LOC** (LOC-total); the largest liftable Bucket-E items are **liquid.rs (2817)**, **collapse.rs (999)**, **compile.rs (819)**, and **main.rs cmd_roomba (261)**; the most substantive vocabulary gaps are **@io/process species-decl mint** (Bucket A incomplete; Mara's `defc8ef` Tick 1 forward-promise) + **expression-tree parser vocabulary per Mara §3.2** (blocks full liquid.rs `dispatch_spec_property` lift); and **Bucket C (Bootstrap Kernel) is currently landed AS a property-verification compiler and ASPIRATIONAL as a self-compiling reflective evaluator** — `mirror compile <file>` fires and produces SAGA-chains of Verdicts against mirror.spec's bilateral/property declarations, but `mirror craft <dir> → binary` returns ExitCode 2 with substrate-decl'd-but-dispatch-lands-at-M3+ message.

---

## Appendix: Genuinely ambiguous classifications for Seam

1. **book.rs 8-arm well-known map** — task says "just the first well-known" is Bucket C, "full 8-well-known" is Bucket E. Split as 1 arm C + 7 arms E, or whole file C? Empirically the impl is a single 40-line match. Seam adjudicates split boundary.

2. **fractal/crystal.rs** — Crystal<T> is compile.rs's SAGA carrier (E-domain) BUT is arguably ontological ground for content-addressed identity (D-domain). Naming as E because it's the settled-state fragment consumed by SAGA orchestration; Oid stays in D via mandelbrot.rs. Seam confirms/reclassifies.

3. **main.rs `sha256_hex()`** — is content-addressed OID hashing a bootstrap-kernel primitive (needed for @spectral/signature discipline; without it, mirror can't compute Crystal OIDs at all) or scaffolding (only used for observation-crystal signature emission)? Reed's task framing suggested C; empirical usage says scaffolding-only. Seam adjudicates.

4. **fractal/singularity.rs + rust/singularity/** — explicitly scaffold/forward-promise per docblocks; is this Bucket E "transient scaffolding" or something distinct like "Bucket F research outlet"? Task didn't define a research-outlet bucket. Marked E; Seam confirms.

5. **main.rs time-formatting utilities** — currently direct std::time impl; substrate has `@time/now` per project memory but @time/format status not verified this scout. Bucket E "needs shard-decl" for @time/format species-decl mint.

---

*Pure-docs 📝 bypass per CLAUDE.md substrate discipline. No .rs files modified. No refactoring proposals. Seam adjudicates ambiguous classifications post-scout.*
