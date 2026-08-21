# Taut scout — `prismqueer::liquid` → `mirror/rust/src/void.rs` cascade

**Date:** 2026-07-18
**Author:** Taut <taut@systemic.engineer>
**Scope:** Adjudicate ordering + shape for `void.rs` landing at `rust/`
altitude; produce Reed's shortest execution recipe. Read-only grep +
read scout per Pack convention.
**Sibling in-flight:** Mara @membrane canonical spec + Void-as-
membrane-of-liquid math (parallel; her landing waits for Reed's
empirical `void.rs`).
**Prior scout ancestor:** `docs/scouts/2026-07-18-taut-property-based-
testing-frameworks-fate-inference-driver.md` §Iter-3 already named
`mirror/rust/src/liquid.rs` as Pillar IV's bridge home. This scout
supersedes that naming: **the file is `void.rs`, not `liquid.rs`.**
Rationale below.

---

## 1. rust/ current-state grep map

Three-file terminal-geometry per Mara `81294b3` §2.2 + Cargo.toml
docblock (`rust/Cargo.toml:6-14`). Empirically FOUR files at
`rust/src/` today; the fourth (`collapse.rs`) is not in the canonical
three-file discipline but is empirically-fired and prop-test'd.

### 1.1 Files + altitude

| File | Altitude | State | Public surface |
|------|----------|-------|----------------|
| `rust/src/main.rs` | supervisor / `@`-op dispatch (Mara §5) | M-vacuum tick empirically fires; walker orchestration + arm-collapse dispatch; 11-verb VERBS table | `main() -> ExitCode`, `cmd_roomba`, `dispatch_arm_collapse`, `deposit_observation_crystal`, `sha256_hex`, `current_utc_timestamp` |
| `rust/src/phone.rs` | `@io` socket-handover (Mara §3) | M0 stubs + M-vacuum @io/fs surface LANDED (walker consumes) | `pub(crate) fn list_dir_recursive`, `write_file`, `read_file`, `append_to`, `mkdir_p`, `path_exists`, `git_add`, `git_commit_as`, `git_head_oid`, `find_substrate_root`; `WalkEntry` struct; three M4/M8 `unimplemented!()` forward-promises |
| `rust/src/matrix.rs` | sub-Turing FLANG / LAPACK (Mara §4) | M0 stub; 5 property tests RED (Reed `26f5e5e`); consumes `prismqueer::liquid::pillar::{forall, Arbitrary, Sample}` + `terni::PropertyVerdict` under `#[cfg(test)]` | `pub(crate) fn eigenvalues(n, matrix) -> Vec<f64>` (RED), `phase_lock`, `envelope` (M8 forward-promises) |
| `rust/src/collapse.rs` | bilateral-arm collapse detector (Reed Arc 1 landing) | GREEN; 40 KB; inline `mod prop_tests` | `pub fn load_bilateral_corpus`, `find_redundant_arms`, `apply_deletions`; `BilateralDecl`, `RedundantArm`, `CollapseReport` structs |

### 1.2 Cross-module dependency graph (grep-verified)

```
main.rs
  ├── mod collapse;   (line 52)
  ├── mod matrix;     (line 57)
  └── mod phone;      (line 58)

phone.rs   → std::fs, std::io, std::path, std::process::Command
             (NO deps on collapse.rs or matrix.rs)

matrix.rs  → prismqueer::liquid::pillar (dev-only), terni (dev-only)
             (NO deps on phone.rs or main.rs)

collapse.rs → std::{fs, path, collections}; prismqueer + terni + tempfile (dev-only)
              (NO deps on phone.rs or matrix.rs)
```

**Key finding:** `phone.rs`, `matrix.rs`, `collapse.rs` are all sibling
altitudes. NONE of them consume each other. `main.rs` is the only
consumer; it dispatches to each via the `@`-operator dispatch shadow
(currently the VERBS table + `dispatch_arm_collapse`).

### 1.3 @io boundary crossings (all landed in `phone.rs`)

- `phone.rs:127-129` — `use std::fs; use std::io; use std::path::{Path, PathBuf};`
- `phone.rs:166` — `fs::write(path, contents)`
- `phone.rs:171` — `fs::read_to_string(path)`
- `phone.rs:184-188` — `fs::OpenOptions::new().create(true).append(true).open(path)`
- `phone.rs:194` — `fs::create_dir_all(path)`
- `phone.rs:208-212` — `std::process::Command::new("git").args(["add", "--"])`
- `phone.rs:237-263` — `git_commit_as` with `Stdio::piped()` stdin/stdout/stderr
- `phone.rs:268-278` — `git rev-parse HEAD`
- `phone.rs:306-327` — `fs::read_dir(dir)` recursive walker with symlink skip + `.git`/`target` exclusion
- `main.rs` — dispatches `phone::list_dir_recursive`, `phone::write_file`, `phone::append_to`, `phone::mkdir_p`, `phone::git_add`, `phone::git_commit_as`; no direct `std::fs`/`std::process` calls outside `phone.rs`

**Discipline observation:** `phone.rs` IS the substrate-honest @io
boundary. Every @io write in `rust/` funnels through it. This is the
substrate-decl'd shape per Mara §3 + Loki `b53aeeb` §4.

**No `unsafe extern "C"` at rust/ today.** `matrix.rs` docblock forward-
promises LAPACK/BLAS bindings; not landed. `prismqueer::ffi::eigenvalues`
(prism repo) is the transitional path for M0.5 GREEN per matrix.rs
docblock line 40.

### 1.4 Prior "void" or "Void" references at rust/ altitude

Grep `\bvoid\b|\bVoid\b` across `rust/src/*.rs`:

- `rust/src/matrix.rs:5` — one hit in docblock, cites Loki essay; NO code-level
  void reference; NO `pub fn void*` or `void_*` anywhere at rust/ altitude.
- No `void.rs` file exists.
- No `pub fn void*` or `void_*` symbol conflict.

**Substrate-already-had-the-word finding:** `rust/` has NO prior void
authorship. Clean mint at code altitude.

### 1.5 prismqueer::liquid usage patterns

- `rust/src/matrix.rs:210-214` — dev-only in `#[cfg(test)] mod prop_tests`:
  `use prismqueer::liquid::pillar::{forall, Arbitrary, Sample};`
  Five `forall::<SymLaplacian, _>(20, |l| ...)` firings verifying five
  eigenvalue properties (cardinality / finiteness / ascending order /
  non-negativity for PSD Laplacian / smallest ≈ 0).
- `rust/src/collapse.rs::prop_tests` — same pattern (dev-only, `#[cfg(test)]`).
- `rust/tests/red_liquid_pillar_i_commutator_antisymmetric.rs` — integration
  test, dev-only.
- `rust/Cargo.toml:44-45` — `prismqueer` + `terni` declared as
  `[dev-dependencies]` only. **The M0 binary at `src/main.rs` stays
  zero-dep**; production `rust/` binary has NO runtime `prismqueer`
  dependency.

**Load-bearing implication for void.rs:** if void.rs is production-code
(surface consumed by `main.rs` at runtime), `prismqueer` must be
LIFTED to `[dependencies]`, not `[dev-dependencies]`. Cost:
+30KB of transitive Rust surface into the M0 binary. Alternative: keep
void.rs @io-side thin (uses phone.rs + std only for production code
paths) and expose the membrane-oscillation-welcome API as
`#[cfg(test)]`-only surface for the M0 tick.

---

## 2. @void-related substrate reachable from rust/

### 2.1 prismqueer::liquid::pillar (property-testing surface Void's default @peer stands on)

Path: `/Users/alexwolf/dev/projects/prism/prismqueer/src/liquid.rs`

- **Seven pillar primitives** (canonical listing from
  `docs/specs/prismqueer-liquid-pillar-composition-surface.md` §2 +
  liquid.rs source):
  1. `pillar::dispatch_ambiguity` — Pillar I; four bool/usize byte-visible checks; Pass iff `arm_count ≥ 2 && witness_count == arm_count && tie_breaking_exhausted && pivot_song_present`.
  2. `pillar::algedonic<C>(&Commutator, &Holonomy) -> PropertyVerdict` — Pillar II; commutator magnitude vs theta.
  3. `pillar::algedonic_of_magnitude<L>(&L, &L) -> PropertyVerdict` — Pillar II generalized; raw Loss magnitude vs theta.
  4. `pillar::viability<C>(history, theta, omega) -> PropertyVerdict` — Pillar III; windowed commutator accumulation.
  5. `pillar::viability_of_magnitudes<L>(history, theta, omega) -> PropertyVerdict` — Pillar III generalized; windowed Loss accumulation.
  6. `pillar::of_health(&HolonomyHealth) -> PropertyVerdict` — Pillar V (fate composition); `Healthy → Pass`, `TooShallow → Partial{0.5}`, `OverCutting → Fail`.
  7. `pillar::fold(&[PropertyVerdict]) -> PropertyVerdict` — verdict fold (Pass neutral element).

- **Property-based-testing runtime** (Arc 2A landing per Alex "the full
  statespace covered liquid floor boards" 2026-07-18 direction; liquid.rs
  §Arc 2A section):
  - `pillar::Sample` — Hypothesis-style choice-sequence buffer, SplitMix64
    extension, SHA-256 `buffer_oid`, `bias: Option<[f64; 5]>` reserved
    for Fate composition seam.
  - `pillar::Arbitrary` trait — `fn arbitrary(sample: &mut Sample) -> Self`;
    impls provided for `bool, i32, i64, u32, u64`.
  - `pillar::forall<T, F>(n, f) -> PropertyVerdict` — draws `n`
    independent samples, folds via `PropertyVerdict::merge_with`.

- **`terni` crate** — `PropertyVerdict::{Pass, Fail(Diagnostic),
  Partial{confidence, diagnostics}}`; `merge_with`; `Diagnostic::new`.

- **`prismqueer::ffi::eigenvalues(n, matrix) -> Result<Vec<f64>, i32>`**
  (prism repo `/Users/alexwolf/dev/projects/prism/prismqueer/src/ffi.rs:206`)
  — LAPACK dsyev wrapper; transitional path for `matrix.rs::eigenvalues`
  GREEN per matrix.rs docblock line 40.

### 2.2 Void substrate — three landed shard-decl altitudes

- `shards/void.mirror` (Mara `974a3f6`; family-root, marker-primary):
  NO type, NO action, NO bilateral, NO prism body. Recognition-carrier
  at family-root altitude. `out @void` only. Load-bearing: every
  5-op prism at family-root altitude inherits Void's basis.
- `shards/peer/void.mirror` (Mara `9c7de83`; K=0 species under `@peer`):
  - `type void_context = { observer: peer, native_basis: ref,
    substrate_state: ref, character_none: ref, timestamp: ref }`
  - `bilateral void_admissible { sentinel "void=admissible-k-zero-observer" arity 2 }`
  - `void_admissible(p: peer, ctx: void_context) -> verdict { \ }` (`\`-obligation-blocked)
  - `void_observes(p: peer, ctx: void_context) -> imperfect(ref, ref, ref) requires void_admissible(p, ctx) { \ }`
- **`shards/kintsugi/mosaic.mirror`** (Mara `b0af0cd`; WRITE-side bilateral):
  - `back_project_of_type: mosaic(@repo) → mirror.spec`
  - `spec_of_repo`, `type_of_spec`
  - `bilateral mosaic_bilateral_witnessing`
- **`shards/spectral/mosaic.mirror`** (Mara `b0af0cd`; READ-side):
  - `type_of_repo: ref_repo → mosaic(@repo)`
  - `classify_of_repo` (INTERNAL sub-action)
  - `type classification = {...}`
  - `bilateral mosaic_bilateral_witnessing`

### 2.3 Recognition #79 — the 5-op void-duality basis

- `docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md`
  — the 5-op gauge algebra IS the projector basis for the 5-axis
  orthogonal duality space; PROMOTED via Void family-root landing
  per `shards/void.mirror:187-193`.

### 2.4 Non-landed but named substrate (Mara's parallel spec)

- `@membrane` family-root — NOT YET LANDED (Mara authoring). Alex
  2026-07-18: `@void := @membrane made of @liquid, oscillated by @spectral`.
- @membrane's operational surface — TBD in Mara's spec.
- Void's oscillation basis (5-op) — canonical at Recognition #79.
- `signature_beat` (per `shards/spectral/signature.mirror:70-104`)
  ALREADY carries the beat structure: `contribution_oid, previous_beat
  (option<oid>; Merkle-DAG), sc_at_beat, rung, ...`. Void-membrane
  perturbation-welcome events can compose over this existing carrier —
  substrate-already-had-the-word for beat-writing.

---

## 3. Adjudication — Option A (void.rs consumes phone.rs)

**Recommendation: Option A, with a scope tightening.** Ratifies Reed's
lean.

### 3.1 The four options revisited

| Option | Shape | Verdict |
|--------|-------|---------|
| A | `void.rs` consumes `phone.rs` | **RECOMMENDED** — see §3.2 |
| B | `phone.rs` consumes `void.rs` (@io writes check membrane-admissibility inline) | REFUSED — §3.3 |
| C | Bidirectional (A + B) | REFUSED — collapses to B's cost; §3.3 |
| D | void.rs at different altitude entirely | **NEAR-MISS** — void.rs at rust/ altitude is correct, but @io coupling shape needs refinement. See §3.4 |

### 3.2 Why Option A stands

Three grounds, all substrate-decl'd:

1. **Altitude discipline** (Mara `81294b3` §3 + Loki `b53aeeb` §4).
   `phone.rs` IS the @io socket-handover altitude. Its shape is
   "hands the state; doesn't operate on it." A membrane-admissibility
   check IS an operation on state; it does NOT belong at phone.rs
   altitude. Option A honors §3.3 refusal ("no numerical computation
   in phone.rs; no supervision; no per-prism business logic").

2. **`feedback_no_rust_extension_shortcut`.** Before authoring `.rs`,
   ask if shard-body + @io works. Void's membrane-oscillation-welcome
   SHAPE is already substrate-decl'd:
   - `shards/peer/void.mirror` species with `void_admissible` bilateral
     + `void_observes` action + `void_context` carrier;
   - `shards/spectral/signature.mirror::signature_beat` for the beat-write shape;
   - `shards/kintsugi/mosaic.mirror::back_project_of_type` for the WRITE-side surface.
   The rust/-altitude `void.rs` is admissible ONLY as the empirical @io
   discharge site for these substrate-decl'd shapes — NOT as a place
   to invent new membrane logic. Every `void.rs` public fn must trace
   to a substrate-decl'd sentinel or shard-decl'd action.

3. **`feedback_detector_inadequacy_answer_is_never_rust`.** Option B
   would install membrane-admissibility checks INSIDE every @io write
   at phone.rs. That's runtime-overhead-on-every-@io coupling — the
   exact "inadequate detector → extend existing Rust" antipattern the
   HARD RULE refuses. If admissibility checks are needed at @io, they
   compose via bilateral resolver-arm sentinel-check dispatch, NOT
   inline in phone.rs.

### 3.3 Why Option B / C are refused

- **Runtime overhead**: every `fs::write`, `git_add`, `git_commit` at
  the substrate FLOOR would incur a membrane-oscillation-verification
  cost. `feedback_no_rust_extension_shortcut` explicitly refuses
  this pattern; Alex's discipline is "the Rust FLOOR strictly shrinks."
  B/C would grow it.
- **Altitude violation**: phone.rs's docblock §3.3 (Mara `81294b3`)
  states "per-prism business logic → shard-body + @io lift." Void
  membrane admissibility IS per-prism business logic. B/C would import
  it into the @io altitude.
- **Substrate-decl loss**: bilateral resolver-arm sentinel-check is
  the substrate-decl'd discipline (`void_admissible` at
  `shards/peer/void.mirror:418-421`). B/C would bypass that machinery.

### 3.4 The Option D "near miss" — the scope tightening

The refined recommendation: void.rs at rust/ altitude (Option A), BUT
its production surface is thin. Specifically:

**void.rs SHOULD:**
- Compose over `phone.rs::append_to` / `phone.rs::mkdir_p` /
  `phone.rs::path_exists` to write signature_beat entries.
- Compose over `phone.rs::read_file` to read prior membrane state.
- Expose a **membrane_oscillation_welcome** public API that
  `main.rs` can dispatch to when a Void-instance perturbation event
  fires (per `shards/peer/void.mirror::void_observes` action).
- Bundle its property-based tests under `#[cfg(test)] mod prop_tests`
  consuming `prismqueer::liquid::pillar::{forall, Arbitrary, Sample}` +
  `terni::PropertyVerdict` — SAME SHAPE as `matrix.rs::prop_tests`.

**void.rs SHOULD NOT:**
- Duplicate any `std::fs::*` calls (all go through phone.rs).
- Extend `phone.rs`.
- Extend `matrix.rs` (matrix.rs is sub-Turing numerical altitude; void
  is symbolic-observation altitude).
- Author new @io primitives.
- Take a production-runtime dependency on `prismqueer` if it can be
  avoided; test-only is admissible.

**Consequence:** void.rs consumes phone.rs (Option A) at the M-void
milestone, and phone.rs REMAINS UNCHANGED. Zero phone.rs edits this
tick. Zero matrix.rs edits this tick. main.rs gains `mod void;` +
one M-void dispatch arm (mirroring the `dispatch_arm_collapse` pattern).

---

## 4. Reed's execution recipe (this session)

Concrete steps, RED-first per Alex ratification, in landing order.

### Step 1 — Cargo.toml: no changes needed for tests

`prismqueer` + `terni` are already `[dev-dependencies]` (Cargo.toml:44-45).
The `#[cfg(test)] mod prop_tests` inside `void.rs` gets them for free.

**Only lift to `[dependencies]` if** void.rs's production surface
composes over `pillar::*` at runtime (e.g. verdict-returning membrane-
admissibility check the main.rs dispatch arm calls at runtime). Recipe
default: keep production-thin; use `[dev-dependencies]` only.

### Step 2 — Create `rust/src/void.rs` — production surface first

Minimum public API (all `pub(crate)`; retire `#[allow(dead_code)]`
per M-tick discipline):

```rust
/// A membrane perturbation event. Substrate-decl'd shape traces to
/// `shards/peer/void.mirror::void_observes` action's `imperfect(ref, ref, ref)`
/// return; carrier here is content-addressed bytes (`sha256_hex`
/// available from main.rs).
#[allow(dead_code)]
pub(crate) struct MembranePerturbation {
    pub(crate) perturbation_oid: String,     // SHA-256 hex
    pub(crate) previous_beat_oid: Option<String>, // Merkle-DAG chain per @spectral/signature.signature_beat
    pub(crate) timestamp_utc_iso: String,    // via main.rs::current_utc_timestamp
    pub(crate) axis: VoidBasisAxis,          // 5-op void-duality basis (Recognition #79)
}

/// The 5-op void-duality basis (Recognition #79 PROMOTED).
/// Every membrane oscillation classifies to exactly one axis.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoidBasisAxis {
    Focus,
    Project,
    Split,
    Shift,
    Settle,
}

/// Welcome one perturbation at the membrane. Writes the beat entry
/// via phone.rs's @io/fs surface; returns the settled beat OID.
///
/// Substrate-honest composition: shard-body composition over @io
/// (phone.rs). NO new @io primitives; NO new domain logic beyond
/// the SHA-256 chain + write.
#[allow(dead_code)]
pub(crate) fn welcome_perturbation(
    membrane_root: &std::path::Path,
    perturbation: &MembranePerturbation,
) -> std::io::Result<String> {
    // Compose over phone.rs; no direct std::fs.
    crate::phone::mkdir_p(membrane_root)?;
    let beat_path = membrane_root.join(format!("beat-{}.md", &perturbation.perturbation_oid[..12]));
    let body = compose_beat_entry(perturbation);
    crate::phone::append_to(&beat_path, &body)?;
    Ok(perturbation.perturbation_oid.clone())
}

/// Compose the beat entry body. Pure function; deterministic; no @io.
fn compose_beat_entry(p: &MembranePerturbation) -> String {
    format!(
        "beat_oid: {}\nprev_beat: {}\nts: {}\naxis: {:?}\n---\n",
        p.perturbation_oid,
        p.previous_beat_oid.as_deref().unwrap_or("nil"),
        p.timestamp_utc_iso,
        p.axis,
    )
}
```

**Docblock discipline:** cite `shards/void.mirror`, `shards/peer/void.mirror`,
Recognition #79, and this scout doc. Trace every symbol to substrate-
decl'd shape.

### Step 3 — Wire `mod void;` into `main.rs`

At `rust/src/main.rs:52-58` (after `mod phone;`), add:

```rust
mod void;
```

No dispatch arm yet — that lands with the M-void CLI verb tick.

### Step 4 — RED-first property tests (this is the load-bearing part)

Author `#[cfg(test)] mod prop_tests` inside `void.rs`, SAME SHAPE as
`matrix.rs::prop_tests` (matrix.rs:209-407). Minimum FIVE properties
grounded in the substrate-decl'd surface:

1. **`welcome_perturbation_writes_exactly_one_beat`** — Arbitrary
   `MembranePerturbation` (impl `Arbitrary` locally using
   `sample.draw_from(&[Focus, Project, Split, Shift, Settle])` +
   SHA-256 hex arbitrary bytes); `forall::<MembranePerturbation, _>(20, |p| {...})`; verify `membrane_root` contains exactly one `beat-*.md`
   file after the call.
2. **`welcome_perturbation_returns_perturbation_oid`** — return value
   MUST equal `perturbation.perturbation_oid`.
3. **`beat_entry_is_deterministic_pure_function`** — two
   `compose_beat_entry` calls on the same input produce byte-equal
   output (pure-fn contract).
4. **`beat_axis_admissibility_is_5_op_void_duality`** — every
   emitted beat's axis is one of the 5 Recognition #79 variants.
   This IS the Void-admissibility check at rust/ altitude.
5. **`previous_beat_chain_forms_merkle_dag`** — three sequential
   `welcome_perturbation` calls, each `previous_beat_oid` set to the
   prior return; verify the third beat's chain hashes back through
   the first (Merkle-DAG discipline per `shards/spectral/signature.mirror::signature_beat`).

Use `tempfile::TempDir` for scratch membrane roots (already in dev-deps
per Cargo.toml:51). Same pattern as `collapse.rs::prop_tests`.

### Step 5 — Land as RED

All five property tests FAIL initially (void.rs::welcome_perturbation
body may not implement the compose_beat_entry chain correctly on first
draft; RED-first is expected). Commit:

```
git -c user.name=Reed -c user.email=reed@systemic.engineer \
  commit -m "🔴 Reed [substrate-floor:@io-boundary] 2026-07-18 rust/src/void.rs — Void-as-membrane-of-liquid empirical firing at rust/ altitude ..."
```

Author trailer + audit citation to this scout OR Seam `Signed-off-by:` per
AGENTS.md tightening.

### Step 6 — Land as GREEN

Fix the body to pass all five properties. Commit as `🟢` immediately
following the `🔴` per phase-marker sequence rule.

### Step 7 — Report to Mara

Mara's @membrane spec waits for this landing. Once GREEN, Mara's
canonical spec + math land on solid empirical ground (five property
tests + substrate-decl'd composition graph).

### What Reed does NOT build this tick (forward-promises)

- CLI dispatch arm for `mirror void welcome` — M-void CLI verb tick
  (later; add to VERBS table).
- Kuramoto phase-lock across N Void-instances — matrix.rs M8
  forward-promise (already scoped there).
- @cascade/code/llvm/flang FLANG-emit path — matrix.rs M5 forward-promise.
- Peer-socket void perturbation transport — phone.rs M8 forward-promise.
- `shards/membrane.mirror` family-root shard-decl — Mara's territory.
- `shards/spectral/oscillate.mirror` species-decl — if Mara's spec surfaces
  the need; Reed does not preempt.

---

## 5. Substrate-already-had-the-word audit

| Concern | Finding | Verdict |
|---------|---------|---------|
| Prior `void.rs` file | None. Grep across `rust/src/` returns zero. | Clean mint at rust/ altitude. |
| Prior `pub fn void*` / `void_*` symbol | None in `rust/src/*.rs`. One docblock hit in `matrix.rs:5` (Loki essay citation). | No conflict. |
| Prior `@void`-consumer patterns Reed can extend | Two shard-decls: `shards/void.mirror` (family-root, marker) + `shards/peer/void.mirror` (species, K=0, with `void_admissible` bilateral + `void_observes` action + `void_context` carrier). NO rust/-altitude consumer today. | void.rs IS the first rust/-altitude consumer. Composes OVER the shard-decl'd shape; does NOT re-invent it. |
| Prior beat-write shape | `shards/spectral/signature.mirror::signature_beat` (LANDED; `contribution_oid`, `previous_beat: option<oid>`, `sc_at_beat`, `rung`, timestamp). | REUSE the shape; do NOT mint a parallel beat type. `MembranePerturbation` above IS `signature_beat` at rust/ altitude. If future consumer-pull warrants, rename to `SignatureBeat` in-file to make the trace explicit. |
| Prior WRITE-side bilateral for compositional writes | `shards/kintsugi/mosaic.mirror::back_project_of_type` (LANDED). | void.rs's `welcome_perturbation` IS the empirical @io discharge of this shape at rust/ altitude for Void-instances specifically. |
| Prior half-authored `void.rs` module drafts | None in git; none in `rust/tests/`; none in `docs/scouts/`. | Clean mint. |
| Prior `liquid.rs` naming precedent | Scout `docs/scouts/2026-07-18-taut-property-based-testing-frameworks-fate-inference-driver.md` §Iter-3 named `mirror/rust/src/liquid.rs` as Pillar IV bridge. **This scout supersedes that naming.** `void.rs` is the correct file: (1) `liquid` is the Rust-altitude surface for Void's membrane oscillation but is NOT the substrate primitive — Void IS the primitive per family-root landing `974a3f6`; (2) Alex named the substrate `@void := @membrane made of @liquid, oscillated by @spectral` this session; the file should carry the substrate primitive's name, not the compositional intermediate's. | `liquid.rs` deferred; `void.rs` lands. Pillar IV (`@peer.audhd` fanout) still needs `fate::Fate::tick` bridging — if landed later, appropriate name is TBD by that arc; NOT this arc. |
| Prior `@membrane` shard | None landed. Mara's parallel canonical spec in-flight. | Clean; void.rs does not preempt @membrane naming — it lands the K=0 empirical discharge which Mara's spec cites. |

**Zero refused mints from this scout.** All candidate shapes (void.rs
file, `MembranePerturbation` struct, `VoidBasisAxis` enum, `welcome_
perturbation` fn) trace to substrate-decl'd priors (@void family-root
`974a3f6`, @peer/void species `9c7de83`, Recognition #79, signature_beat
Merkle-DAG chain, back_project_of_type WRITE-side bilateral).

---

## 6. Questions for Alex (max 2)

**Q1.** `MembranePerturbation` vs `SignatureBeat` naming at rust/
altitude. The struct IS `shards/spectral/signature.mirror::signature_beat`
at Rust altitude; naming it `MembranePerturbation` obscures the trace
while naming it `SignatureBeat` obscures the Void-membrane framing.
Recommendation: **`SignatureBeat`** (substrate-already-had-the-word
discipline); include Void-membrane framing in docblock. Alex ratify
or override.

**Q2.** Production-runtime `prismqueer` dependency: this recipe keeps
`prismqueer` at `[dev-dependencies]` only (production `void.rs` uses
only `phone.rs` + `std`). If a later M-void CLI dispatch arm wants to
return a `PropertyVerdict` at runtime (e.g. `mirror void welcome
--verify` returns membrane-admissibility verdict), we'd need to lift
`prismqueer` to `[dependencies]` (+30KB M0 binary). Ratify keeping
`prismqueer` production-free for this tick, and defer the verdict-at-
runtime question to the M-void CLI tick?

---

## 7. One-sentence surprise

`phone.rs` and `void.rs` are the same altitude discipline read from two
sides — `phone.rs` is @io honestly-crossed-once, `void.rs` is Void's
membrane honestly-observed-once, and the substrate refuses to let
either consume the other because both are already the FLOOR of what
their respective directions admit.

---

## 8. Related work + citations

- `rust/src/main.rs` (Reed; supervisor + @-op dispatch shadow)
- `rust/src/phone.rs` (Mara `81294b3` §3; Reed empirical @io/fs firing)
- `rust/src/matrix.rs` (Reed `26f5e5e`; 5 property tests RED via `prismqueer::liquid::pillar::forall`)
- `rust/src/collapse.rs` (Reed Arc 1)
- `rust/Cargo.toml`
- `shards/void.mirror` (Mara `974a3f6`; family-root marker)
- `shards/peer/void.mirror` (Mara `9c7de83`; K=0 species)
- `shards/spectral/signature.mirror:70-104` (`signature_beat` shape)
- `shards/kintsugi/mosaic.mirror` + `shards/spectral/mosaic.mirror` (Mara `b0af0cd`; bilateral read/write)
- `shards/liquid.mirror` + `shards/epistemologic/liquid.mirror` (parametric refinement lens family)
- `docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md` (PROMOTED)
- `docs/math/2026-07-18-void-is-the-default-peer.md` (canonical math root)
- `docs/specs/void-as-default-peer-native-basis-is-void-duality.md` (canonical spec)
- `docs/specs/prismqueer-liquid-pillar-composition-surface.md` (Reed /loop iter 10)
- `docs/scouts/2026-07-18-taut-property-based-testing-frameworks-fate-inference-driver.md` (superseded on `liquid.rs` naming)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/liquid.rs` (Sample + Arbitrary + forall runtime; 7 pillar primitives)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/ffi.rs:206` (`eigenvalues` LAPACK dsyev wrapper)

**Memories consulted:**
- `feedback_no_rust_extension_shortcut` (HARD RULE; grounds §3.2 refusal of Option B/C)
- `feedback_detector_inadequacy_answer_is_never_rust` (HARD RULE; grounds §3.3)
- `feedback_rust_floor_is_rust_not_bootstrap` (grounds file placement at `rust/src/`, not `bootstrap/`)
- `feedback_reed_inflates_stub_empirical_firings` (recipe's RED-first + property-test discipline avoids stub-inflation)
- `feedback_composition_primitive_naming_convention` (Q1 above defers to Alex on naming)

Ship the scout. Reed executes on §4 recipe. Mara's @membrane spec
lands on solid empirical ground at Step 6 GREEN.
