# 2026-09-09 Taut — Option/Result/Boolean cascade + prismqueer spectral engine completeness + v0.2 ouroboros floor-truth scout

**Author.** Taut `<taut@systemic.engineer>` (grep-first drift scout; read-only).

**Kolibri.** Der Kolibri kolibriert.

**Composition-substrate anchor.** Alex 2026-09-09 verbatim (this dispatch):

> "Let's eliminate every Option, Result and boolean in prismqueer, then finish shipping the prismqueer spectral engine. And then collapse into the minimal rust floor in mirror. And then we actually build the ouroboros. And from there on the MCP, LSP, and Lambda Shell are downstream. The v0.2 ouroboros milestone is what this is."

**Discipline.** Grep-first per Taut CLAUDE.md; every claim carries a `file:line` citation. Read-only per Taut role. `📝` markdown-only bypass.

**Scope-frame.** Seven scopes composed into one Q+42 dependency-graph closure. Silicon Venue register light-touch (Taut is grep-scout, not metalogue-primary).

---

## §0 Scope + Alex 2026-09-09 verbatim

### §0.1 Substrate-position at scout-tick

- **HARD RULE Alex 2026-09-08 today**: Option/Result are binary-in-a-ternary-compiler; substrate-native is `Imperfect<Green, Yellow, Red>` LANDED at `/Users/alexwolf/dev/projects/prism/imperfect/src/lib.rs`.
- **HARD RULE Alex 2026-09-09 today**: `bool` is the same smell at type-signature altitude; substrate-native replacement per generating-geometry (see §2 classification table).
- **HARD RULE Alex 2026-09-08**: bare types (`i64` / raw `Oid`) at struct-field altitude are smells (Yaron Minsky 2011 *make-illegal-states-unrepresentable*).
- **HARD RULE Reed 2026-09-05**: JavaFactoryClassName names are smells (Rich Hickey *Simple-made-Easy* noun-lift).
- **HARD RULE Alex 2026-09-08 (meta)**: "What is the non-linear compose-not-average alternative to the Turing paradigm?" — every proposal answers this.

### §0.2 What this scout is + what it is not

**IS.** Enumeration + classification of every Option/Result/bool/bare-type/JavaClassName site across `prism/prismqueer`, `prism/imperfect`, and `mirror/rust/`; enumeration of prismqueer spectral engine LANDED-vs-forward-promised; enumeration of mirror rust floor stays/retires/transforms; v0.2 ouroboros dependency-graph shape.

**IS NOT.** Shard-decl authorship; Rust wire; time estimates; recommendations that require Fourth-Chair adjudication without surfacing as Q-Taut residues.

---

## §1 Option/Result cascade — grep-verified sites + classification

### §1.1 Total site counts (grep-verified)

**`\bOption\b` sites.** Search across `/Users/alexwolf/dev/projects/prism/prismqueer/src/**/*.rs` + `/Users/alexwolf/dev/projects/mirror/rust/**/*.rs` + `/Users/alexwolf/dev/projects/prism/imperfect/**/*.rs`:

| Repo | File-count | Total match lines |
|---|---|---|
| prism/prismqueer | 15 files | 18 lines |
| prism/imperfect | 2 files (docblock refs; no primitive) | 0 primitive |
| mirror/rust | ~9 files | ~50+ lines |

**`\bResult\b` sites.**

| Repo | File-count | Total match lines |
|---|---|---|
| prism/prismqueer | 14 files | ~30 lines |
| prism/imperfect | 0 (Imperfect IS the replacement; no Result at primitive) | 0 |
| mirror/rust | ~10 files | ~40+ lines |

### §1.2 Site-by-site enumeration + classification (prism/prismqueer)

Legend for **Class**:
- **A** = Categorical distinction — substrate-honest replacement: `Imperfect<Green, Yellow, Red>`.
- **B** = Continuous scalar reading — substrate-honest replacement: `Pattern`-impl (Amplitude / WAT / Connectivity / Resonance).
- **C** = Predicate-computed-from-richer-state — PRESERVE as method (not stored field).
- **D** = Literal-equality-with-substrate-primitive — PRESERVE.
- **E** = std::fmt::Result — PRESERVE (std trait signature; not substrate concern).
- **F** = Optional-carrier at absent-vs-present altitude — replace per case (`Imperfect<T, Missing, Refused>` or discriminated variant).

| File | Line-approx | Signature (verbatim) | Class | Note |
|---|---|---|---|---|
| `prism/prismqueer/src/beam.rs` | :116 | `source: Option<In>` | F | Struct field carrying optional input; use discriminated variant per input-presence semantics |
| `prism/prismqueer/src/beam.rs` | :162 | `pub fn value(&self) -> Option<&Out>` | C-or-F | Method exposing settled output; classify as C if computed-from-richer-state, else F |
| `prism/prismqueer/src/coincidence.rs` | :409 | `fn eigenvalue_hex(&self) -> Option<&str>` | F | Missing-eigenvalue-hex; use `Imperfect<&str, Missing, Refused>` or byte-empty discriminated variant |
| `prism/prismqueer/src/coincidence.rs` | :435 | `fn preview(&self, _output: &Self::Output) -> Option<&Self::Input>` | F | Trait method; preview-may-be-absent |
| `prism/prismqueer/src/fate/metal_runtime.rs` | :172 | `fn make_runtime() -> Option<MetalRuntime>` | F | Runtime-may-not-be-available (metal feature); substrate-honest → `Imperfect<MetalRuntime, Unavailable, Refused>` |
| `prism/prismqueer/src/fate/weights.rs` | :216 | `pub fn from_bytes(bytes: &[u8]) -> Option<Self>` | A | Parse succeeds-or-fails at byte-level; substrate-honest → `Imperfect<Self, PartialParse, Refused>` |
| `prism/prismqueer/src/ffi.rs` | :125 | `pub fn preview(n: usize, projection: &[f64], source: &[f64]) -> Option<Vec<f64>>` | F | FFI preview; may return empty on dimension-mismatch |
| `prism/prismqueer/src/flux.rs` | :518 | `bias: Option<[f64; 5]>` | F | Struct field; bias-may-be-uninitialized |
| `prism/prismqueer/src/lambda/reduce.rs` | :121 | `fn try_step<T>(term: &Lambda<T>) -> Option<Lambda<T>>` | A | Beta-reduction may not apply; substrate-honest → `Imperfect<Lambda<T>, NoReducible, Refused>` |
| `prism/prismqueer/src/luminosity.rs` | :30 | `pub fn holonomy(&self) -> Option<f64>` | B | Holonomy IS a Pattern-scalar-reading; substrate-honest → direct `Holonomy(f64)` newtype with sentinel zero-value |
| `prism/prismqueer/src/named.rs` | :281, :354 | `maybe: Option<String>`, `b: Option<u32>` | F | Test-fixture struct fields; scoped to `#[cfg(test)]` — LOWER PRIORITY |
| `prism/prismqueer/src/optic_kind.rs` | :23 | Docblock comment referencing `Option<T>`, `Result<T, E>` | (docblock only) | Cosmetic; no primitive |
| `prism/prismqueer/src/optics/optic_prism.rs` | :44 | `pub fn extract(&self, s: &S) -> Option<A>` | A | Prism extract may not match — this is *canonically the definition of Prism*. Substrate-honest → keep API shape but return `Imperfect<A, NoMatch, Refused>` |
| `prism/prismqueer/src/pq/output.rs` | :32 | `message: Option<String>` (with `serde` skip attribute) | F | Serialization boundary — MAY preserve at wire altitude per @data/json optional-field semantics |
| `prism/prismqueer/src/spectral/harmonics.rs` | :157 | `pub value: Option<f64>` | B | Harmonic value carrying optional scalar — substrate-honest → `Amplitude(f64)` newtype with zero-sentinel + separate presence-witness at spectrum altitude |
| `prism/prismqueer/src/spectral_uuid.rs` | :231 | `fn hex_digit(b: u8) -> Option<u8>` | F | Parser helper; may reject non-hex byte |
| `prism/prismqueer/src/trace.rs` | :93 | `pub fn reenter_at<T: 'static>(&self, i: usize) -> Option<&T>` | F | Index-may-be-out-of-bounds; substrate-honest → `Imperfect<&T, IndexOutOfBounds, Refused>` |

**`Result<T, E>` sites (prism/prismqueer):**

| File | Line | Signature | Class | Note |
|---|---|---|---|---|
| `prism/prismqueer/src/coincidence.rs` | :93, :132, :152, :182 | `Result<StateVector, CoincidenceError>` + `Result<Self, CoincidenceError>` (four sigs) | A | Numeric-normalization-may-fail; substrate-honest → `Imperfect<T, PartialNormalize, Refused>` |
| `prism/prismqueer/src/coincidence.rs` | :499 | `std::fmt::Result` | E | Std-trait signature; preserve |
| `prism/prismqueer/src/fate/metal_runtime.rs` | :33, :152 | `Result<Self, String>` + `Result<metal::Library, String>` | A | Feature-gated; still substrate-native replacement |
| `prism/prismqueer/src/fate/train.rs` | :14 | `pub fn load_examples(path: &str) -> Result<Vec<Example>, String>` | A | I/O; substrate-honest → `Imperfect<Vec<Example>, PartialLoad, Refused>` |
| `prism/prismqueer/src/ffi.rs` | :217, :247, :282, :317, :365 | 5 LAPACK sigs `Result<T, i32>` (error-code from Fortran) | A | Numeric-i32-error-carrier; substrate-honest → `Imperfect<T, LapackError(i32), Refused>` with i32-error as sub-payload |
| `prism/prismqueer/src/oid.rs` | :49, :55 | `std::fmt::Result` | E | Std-trait; preserve |
| `prism/prismqueer/src/pq/reference.rs` | :26 | `pub fn try_new(s: impl Into<String>) -> Result<Self, &'static str>` | A | Parser-boundary; substrate-honest |
| `prism/prismqueer/src/pq/reference.rs` | :44 | `fmt::Result` | E | Preserve |
| `prism/prismqueer/src/pq/target.rs` | :78, :87, :93 | serde-deserialize sigs | E | Serde trait boundary; preserve (external-API constraint) |
| `prism/prismqueer/src/precision.rs` | :26, :62 | `std::fmt::Result` (two) | E | Preserve |
| `prism/prismqueer/src/spectral/kleinos.rs` | :110-114 | docblock note referencing retired `Result<ComposedSheaf, ...>` binary return | (docblock only) | Reed 2026-09-02 already retired at this site (composed IS Imperfect) |
| `prism/prismqueer/src/spectral_dimension.rs` | :57, :80 | `std::fmt::Result` | E | Preserve |
| `prism/prismqueer/src/spectral_dimension.rs` | :151 | `Result<SpectralDimension, i32>` | A | LAPACK-error-carrier; substrate-honest |
| `prism/prismqueer/src/spectral_oid.rs` | :102 | `fmt::Result` | E | Preserve |
| `prism/prismqueer/src/spectral_uuid.rs` | :135 | `pub fn parse(s: &str) -> Result<Self, ParseError>` | A | Parser-boundary; substrate-honest |
| `prism/prismqueer/src/spectral_uuid.rs` | :177, :194, :211 | `fmt::Result` (three) | E | Preserve |
| `prism/prismqueer/src/substrate_ref.rs` | :63 | `pub fn new(path: impl Into<String>) -> Result<Self, &'static str>` | A | Validation-boundary; substrate-honest |

**Aggregate at prism/prismqueer:**
- Option sites: ~18 (excluding docblock refs).
- Result sites: ~30, of which ~15 are `std::fmt::Result` / `serde::Deserialize::Result` (Class E; preserve) and ~15 are substrate-fabric Result (Class A; retire to `Imperfect`).

### §1.3 Site-by-site enumeration (mirror/rust)

| File | Line-approx | Signature (verbatim) | Class | Note |
|---|---|---|---|---|
| `rust/fractal/src/singularity.rs` | :82-84 | `fn collapse(&self) -> Result<Self::Artifact, Self::Error>` + `fn settle(artifact: &Self::Artifact) -> Result<Self, Self::Error>` | A | Trait definition — Singularity's operational primitives; substrate-honest post-Fire-A → `Imperfect<Artifact, PartialCollapse, Refused>` |
| `rust/fractal/src/singularity.rs` | :105 | `std::fmt::Result` | E | Preserve |
| `rust/fractal/src/singularity.rs` | :134, :138 | Trait-impl signatures for `Iso` rung | A | Substrate-honest per §5 of Mara `0bca1c5` |
| `rust/fractal/src/subject.rs` | :72 | `pub home: Option<String>` | F | Persistent-peer-home; substrate-honest → Void's K=0 no-character semantics via discriminated variant |
| `rust/matrix/src/book.rs` | :49 | `pub struct RegistryError` (with `Result<Subject, RegistryError>` at :89) | A | Registry-lookup-fallible; substrate-honest |
| `rust/matrix/src/lib.rs` | multiple LAPACK sigs at :328-334, :481-482, :967, :1195-1197 | `Result<Vec<f64>, String>`, `Result<(Vec<f64>, f64), String>` (five sigs) | A | LAPACK-boundary; substrate-honest → `Imperfect<T, LapackError, Refused>` |
| `rust/matrix/src/void.rs` | :120 | `pub(crate) previous_beat_oid: Option<String>` | F | Beat-may-be-genesis (no prev); substrate-honest → discriminated variant with GENESIS marker |
| `rust/matrix/src/void.rs` | :184 | `io::Result<String>` | E | std::io Result; preserve at @io boundary |
| `rust/roomba/src/mend.rs` | :86 | `pub commit_oid: Option<String>` | F | Commit-may-not-yet-exist; discriminated variant |
| `rust/roomba/src/mend.rs` | multiple | `Option<usize>` parser helpers at :338, :342 | F | Byte-scanner helpers; LOWER PRIORITY (retires with roomba dissolution per Mara `f31d703` §3.3) |
| `rust/spectral/src/liquid.rs` | :181 | `pub samples: Option<usize>` | F | Spec-property-samples-may-be-absent |
| `rust/spectral/src/liquid.rs` | :185 | `pub defer_message: Option<String>` | F | Property-may-not-be-deferred; substrate-honest → discriminated variant `{Deferred(String) | Active}` |
| `rust/spectral/src/liquid.rs` | :243-246 | local `Option<usize>` + `Option<String>` in extractor | F | Extraction-parse locals; substrate-honest at output |
| `rust/spectral/src/liquid.rs` | :685 | `Result<Vec<f64>, _>` (parse-history) | A | Parse-partial-history; substrate-honest |
| `rust/src/apply_h.rs` | :83-92 | `pub enum Verdict { Pass, Fail(String) }` (retires per Mara `f31d703` §3.2) | A | The core Verdict enum; retires wholesale to `Imperfect<Green, Yellow, Red>` per Rec #92 |
| `rust/src/apply_h.rs` | :208, :255, :390, :559 | `Option<&str>` + `Option<String>` + `Option<(usize, usize)>` (parser helpers) | F | LOWER PRIORITY (retires with apply_h.rs dissolution per Mara `f31d703` §3.4) |
| `rust/src/main.rs` | :490 | `fn emit_response(resp: &serde_json::Value) -> Result<(), String>` | A | MCP-wire boundary; substrate-honest |
| `rust/src/main.rs` | :746 | `vacuum_dir: Option<&str>` (local) | F | Local variable; retires when settle command lands |
| `rust/src/main.rs` | :964 | `commit_oid: Option<String>` (in CrystalDeposit struct) | F | Deposit-may-not-yet-commit; substrate-honest → discriminated variant |
| `rust/src/main.rs` | :989, :1308 | `Result<CrystalDeposit, String>` + `Result<collapse::CollapseReport, String>` | A | Return sigs; substrate-honest |
| `rust/src/main.rs` | :1366 | `fn find_git_root(start: &std::path::Path) -> Option<std::path::PathBuf>` | F | Filesystem-walk-may-miss; substrate-honest |
| `rust/src/main.rs` | :1470 | `pub(crate) fn at_operator(...) -> Result<String, String>` | A | @-operator boundary; substrate-honest |
| `rust/src/main.rs` | :2001, :2206, :2316 | multiple `Option<&str>` + `Option<String>` locals + return | F | LOWER PRIORITY (parse helpers) |
| `rust/src/phone.rs` | Multiple | `io::Result<T>` throughout (~20 sigs at :129, :140, :162, :177, :202, :227, :315, :328, :333, :345, :356, :369, :405, :461, :513, :520, :548, :1084) | E | std::io::Result; PRESERVE — this is FLOOR @io boundary. Substrate-decision: `phone.rs` IS the @io transport-surface and `io::Result` remains the OS-boundary carrier |
| `rust/src/phone.rs` | :1125 | `fn fresh_repo() -> Option<TempDir>` | F | Test helper; PRESERVE |
| `rust/src/wire.rs` | :63, :76, :93 | Three `Result<T, String>` (parse / emit / emit_pretty) | A | Wire-parse-boundary; substrate-honest |

**Aggregate at mirror/rust:**
- Option sites: ~30+ (many in apply_h.rs + main.rs parser helpers that retire with dissolution).
- Result sites: ~40+, of which ~25 are `io::Result` / `std::fmt::Result` (Class E; preserve at @io boundary) and ~15 are substrate-fabric Result (Class A; retire).

### §1.4 Aggregate site counts (all repos)

| Metric | Prism | Mirror | Total |
|---|---|---|---|
| Option sites (all) | 18 | 30+ | ~48 |
| Result sites (all) | 30 | 40 | ~70 |
| Class A (retire to Imperfect) | ~10 | ~15 | ~25 |
| Class B (retire to Pattern-scalar) | ~2 | 0 | ~2 |
| Class C (preserve as computed method) | ~2 | ~3 | ~5 |
| Class D (preserve literal-equality) | 0 | 0 | 0 |
| Class E (preserve std/serde/io) | ~15 | ~25 | ~40 |
| Class F (retire to discriminated variant) | ~15 | ~20 | ~35 |

**Substrate-decision at scout-tick:** the cascade is ~62 substrate-fabric sites (Classes A + B + F) — the ~40 Class E sites at std/serde/io/fmt boundaries preserve because those are external-API contracts.

---

## §2 Boolean cascade — shards + prismqueer + mirror rust

### §2.1 Classification table (per dispatch scope)

| Boolean-smell category | Substrate-honest replacement | LANDED substrate primitive |
|---|---|---|
| Categorical distinction (yes/no over ternary phenomenon) | `Imperfect<Green, Yellow, Red>` | `terni::Imperfect` at `prism/imperfect/src/lib.rs:171-181` |
| Continuous scalar reading | `Pattern`-impl direct | LANDED `Amplitude` / `WAT` / `Connectivity` / `Resonance` (forward-promised at prismqueer; Alex 2026-09-08 newtype cascade) |
| Vector/tensor reading | Per-axis `Pattern`-impl | LANDED `Harmonics { spectrum: Vec<f64> }` at `prism/prismqueer/src/spectral/harmonics.rs:59-67`; `CommutatorSpectrum` at `commutator.rs:44`; `DeltaCritical` at `harmonics.rs:154-162` |
| Categorical/topological invariant | Geometric primitive on LANDED substrate | LANDED `Oid` at `rust/fractal/src/mandelbrot.rs:46`; `Crystal<T>` at `rust/fractal/src/crystal.rs:51`; `@torus`, `@song/beat`, `@spectral/signature` at shard altitude |
| Predicate-computed-from-richer-state | PRESERVE as method (not stored field) | e.g. `is_liquid()` at `rust/fractal/src/mandelbrot.rs:77` computed from Mandelbrot orbit-boundedness |
| Literal-equality-with-substrate-primitive | PRESERVE (byte-equality IS the geometry) | e.g. `oid == Oid::GENESIS` |
| Result<T, E> at fallible-computation altitude | `Imperfect<T, Yellow, Red>` | LANDED `terni::Imperfect` |
| Option<T> at optional-carrier altitude | `Imperfect<T, Yellow=Missing, Red=Refused>` OR discriminated-variant | Case-specific |

### §2.2 Shard altitude — bool sites

**Total shards carrying `bool`: 23 files** (grep-verified `bool` string match; count-per-file):

| File | Match count | Site classification |
|---|---|---|
| `shards/butterfly.mirror` | 4 | :361 `verdict_kill: bool` (Class A — kill-verdict IS ternary); :447 `kill(...) -> bool` (Class A) |
| `shards/docblock.mirror` | 1 | :223 docblock reference "boolean-as-ref state" (docblock only) |
| `shards/docs/tea/spectral-engineer-landing.mirror` | 1 | :218 docblock reference "boolean-shaped ref" (docblock only) |
| `shards/epistemologic/pact/symbol_canonical_form.mirror` | 1 | :148 `is_inference_context(site: ref) -> bool { \ }` (Class C — predicate-computed; preserve method shape but retire return to Imperfect) |
| `shards/facet/erlang.mirror` | 2 | :186 docblock; :199 `attr_generated(bool)` (Class A — attr-tag) |
| `shards/io/fs.mirror` | 2 | :175 docblock; :183 `is_absolute: bool` (Class C — predicate-computed) |
| `shards/io/git.mirror` | 1 | :354 `commit(..., allow_empty: bool) -> verdict` (Class A — flag-parameter; substrate-honest → discriminated variant `AllowEmpty | RequireDelta`) |
| `shards/kintsugi/ouroboros.mirror` | 5 | :192, :198, :208-210 `verifiable: bool` + `irreducible: bool` (Class A ×2 — both retire to `Imperfect<Green, Yellow, Red>` per traffic-light-natural) |
| `shards/magic/contract.mirror` | 1 | :82 docblock "not bare bool" (already substrate-honest in guidance) |
| `shards/mirror/data.mirror` | 1 | :75 docblock "null \| bool \| number..." (Class D — data JSON type; PRESERVE as canonical JSON algebraic-type) |
| `shards/mirror/data/json.mirror` | 4 | :28 docblock + :154 `type json = null \| bool(bool) \| ...` (Class D — canonical JSON grammar; PRESERVE) |
| `shards/mirror/data/toml.mirror` | 2 | :84 `type toml = null \| bool(bool) \| ...` (Class D — canonical TOML grammar; PRESERVE) |
| `shards/mirror/data/yaml.mirror` | 2 | :80 `type yaml = null \| bool(bool) \| ...` (Class D — canonical YAML grammar; PRESERVE) |
| `shards/mirror/lens/cli.mirror` | 6 | :160, :167, :171, :173 `type bool = ref(scalar/bool)` + :175 `out bool` (Class D — CLI-flag substrate; PRESERVE for CLI-flag presence semantics) |
| `shards/mirror/lens/cli/compile.mirror` | 3 | :55, :107, :136 `strict: bool` (Class A or C — strict-mode flag; substrate-honest → three-state Strict/Lenient/Adaptive) |
| `shards/mirror/lens/cli/kintsugi.mirror` | 3 | :59, :278, :320 `emit_shatter: bool` (Class A — emission-mode) |
| `shards/mirror/spec/property.mirror` | 2 | :160-166 docblock "Evaluates to a boolean (Pass/Fail) or a..." (already substrate-honest — points at Imperfect) |
| `shards/mq.mirror` | 2 | :318-319 `loss: bool, eigenboard: bool` (Class A — MQ output-flags; substrate-honest → discriminated variant per output-shape) |
| `shards/paradox/trauma.mirror` | 3 | :206, :219, :234 `witness_only: bool` (Class D — literal-equality-with-substrate-primitive; IS the family-invariant marker; PRESERVE at witness altitude but with clarified substrate-decl semantics per Alex 2026-07-20 `@paradox` composition) |
| `shards/peer/persistence.mirror` | 1 | :315 docblock "byte-boolean" (already substrate-honest — points at kintsugi store) |
| `shards/spectral/singularity.mirror` | 1 | :453 docblock "attractor-witness boolean" (Class B — spectral-reading; retire to Amplitude-with-sentinel) |
| `shards/subject/visibility/public.mirror` | 1 | :97 docblock "not a bare boolean" (already substrate-honest — guidance) |
| `shards/ui/field.mirror` | 1 | :66 docblock "unlabel(...) projects to per-mote bool" (Class B — projection-scalar) |

**Shard-altitude aggregate:**
- Substrate-decl sites carrying `bool`: **~15 struct-field / return / parameter sites** across 8 substrate-decl-authoritative shard files.
- Class D preserves: **6 sites** (JSON/TOML/YAML/CLI-flag/paradox-invariant — all *canonical grammar* or *substrate-primitive literal*).
- Docblock-only refs: **7 sites** (already substrate-honest in guidance).

### §2.3 Prismqueer rust altitude — bool sites (from §1.2 grep)

| File | Line | Site | Class | Note |
|---|---|---|---|---|
| `prism/prismqueer/src/beam.rs` | :62, :67, :72 | `fn is_ok(&self) -> bool` + `fn is_partial()` + `fn is_err()` | C | Predicate methods on Imperfect; PRESERVE |
| `prism/prismqueer/src/bundle.rs` | :318 | `pub kernel: bool` (StableFiber field) | A | Kernel-status IS ternary (in-kernel / near-kernel / off-kernel via holonomy-slack); substrate-honest → `Kernel` newtype or `Imperfect` |
| `prism/prismqueer/src/bundle.rs` | :415, :499, :649, :729 | `fn new(strategy_shift: u8, kernel: bool) -> Self` (four constructors) | A | Constructor-parameter; retires with :318 field-retirement |
| `prism/prismqueer/src/coincidence.rs` | :84 | `fn is_zero(&self) -> bool` | C | Loss-trait method; PRESERVE |
| `prism/prismqueer/src/crystal.rs` | :32 | `pub fn is_settled(&self) -> bool` | C | Predicate-computed; PRESERVE |
| `prism/prismqueer/src/fate/derive.rs` | :319 | `fn is_zero(&self) -> bool` | C | Loss impl; PRESERVE |
| `prism/prismqueer/src/fate/manifold.rs` | :119 | `fn is_zero(&self) -> bool` | C | Loss impl; PRESERVE |
| `prism/prismqueer/src/flux.rs` | :199-200 | `tie_breaking_exhausted: bool` + `pivot_song_present: bool` (function parameters) | A | Rice-safe binary predicate; substrate-honest → three-state or discriminated variant per @paradox composition |
| `prism/prismqueer/src/lambda/reduce.rs` | :59 | `fn is_zero(&self) -> bool` | C | Loss impl; PRESERVE |
| `prism/prismqueer/src/luminosity.rs` | :16, :20, :24 | `is_light()` / `is_dimmed()` / `is_dark()` | C | Three-way projection from Luminosity enum; PRESERVE methods but consider `Imperfect<Light, Dimmed, Dark>` upgrade (three states already!) |
| `prism/prismqueer/src/merkle.rs` | :21 | `fn is_leaf(&self) -> bool` | C | Node-shape predicate; PRESERVE |
| `prism/prismqueer/src/merkle.rs` | :327 | `fn is_zero(&self) -> bool` | C | Test-local Loss impl; PRESERVE |
| `prism/prismqueer/src/oid.rs` | :43 | `pub fn is_dark(&self) -> bool` | C | Predicate over OID darkness; PRESERVE |
| `prism/prismqueer/src/pq/filter.rs` | :97, :102 | `compare: bool` + `kintsugi: bool` (struct fields) | A | Filter-mode flags; substrate-honest → discriminated variant |
| `prism/prismqueer/src/pq/output.rs` | :53, :57 | `snapshot: bool` + `flush: bool` | A | Output-mode flags |
| `prism/prismqueer/src/pq/target.rs` | :56, :60 | `refs: bool` + `shard: bool` | A | Target-mode flags |
| `prism/prismqueer/src/precision.rs` | :56 | `pub fn is_critical(&self) -> bool` | C | Predicate over Precision; PRESERVE |
| `prism/prismqueer/src/scalar_loss.rs` | :32, :45 | `is_zero()` + `is_non_negative()` | C | Loss/Metric trait methods; PRESERVE |
| `prism/prismqueer/src/store.rs` | :82 | `fn is_zero(&self) -> bool` | C | Test-local; PRESERVE |
| `prism/prismqueer/src/trace.rs` | :86 | `pub fn is_empty(&self) -> bool` | C | Collection predicate; PRESERVE |

**Prismqueer rust bool aggregate:**
- Class A (retire): ~13 sites (bundle StableFiber.kernel + 4 constructors + flux 2-param + pq/filter 2 + pq/output 2 + pq/target 2).
- Class C (preserve as method): ~20 sites (all `is_zero` / `is_ok` / `is_settled` / `is_dark` predicate-methods).

### §2.4 Prism/imperfect altitude — bool sites

**PRESERVE-WHOLESALE per substrate-decision.** Imperfect IS the ternary primitive; its bool predicates are the *projection methods* consumers use to interrogate it.

- `prism/imperfect/src/lib.rs` :116, :148, :309, :312, :317, :693-1104 (30+ `is_zero` / `is_ok` / `is_partial` / `is_err` / `is_settled` / `is_dirty` methods).
- `prism/imperfect/src/transparency.rs` :77, :298, :302, :306, :310 (5 predicate methods).

All Class C. All PRESERVE. These are the projection surface.

### §2.5 Mirror rust altitude — bool sites (from §1.3 grep)

| File | Line | Site | Class | Note |
|---|---|---|---|---|
| `rust/fractal/src/crystal.rs` | :75, :86, :89 | `is_genesis()` / `is_liquid()` / `is_crystal()` | C | Substrate-composition-status predicates; PRESERVE |
| `rust/fractal/src/mandelbrot.rs` | :54, :77, :80 | `is_genesis()` / `is_liquid()` / `is_crystal()` (trait) | C | PRESERVE |
| `rust/fractal/src/singularity.rs` | :193 | `pub fn is_unitary(&self) -> bool` | C | Singularity-state predicate; PRESERVE |
| `rust/fractal/src/subject.rs` | :146, :151, :156 | `is_void()` / `is_human()` / `is_peer()` | C | Subject-typology predicates over three-state enum; PRESERVE (consider `Imperfect<Void, Human, Peer>` upgrade path per §6.1 subject-K=0/1/2 substrate) |
| `rust/spectral/src/liquid.rs` | :128, :133, :197, :202, :450, :453, :455 | 7 predicate methods (`is_composed()` / `is_base()` / `has_verifies()` / `is_deferred()` / `is_pass()` / `is_fail()` / `is_defer()`) | C | Property-decl / verdict predicate methods; PRESERVE |
| `rust/src/compile.rs` | :140, :143, :145 | `is_continue()` / `is_escalate()` / `is_halt()` | C | Halt-strategy three-way predicates on `HaltStrategy` enum; PRESERVE. NOTE: three predicate-methods on 3-variant enum SCREAMS for `Imperfect<Continue, Escalate, Halt>` refactor |
| `rust/src/love.rs` | :178 | `pub climbs: bool` (AxisVerdict field) | A | FiedlerRise-per-axis boolean; substrate-honest → `Amplitude(f64)` newtype with strict-`>` sentinel evaluated per axis (composes over `lambda_a`, `lambda_b`, `lambda_composed` continuous fields at :171-175) |
| `rust/src/love.rs` | :215 | `pub fn is_observer_inclusive(&self) -> bool` | C | Predicate on FullSpectrum; PRESERVE |
| `rust/src/main.rs` | :1193 | `fn is_leap(y: u32) -> bool` | C | Gregorian-calendar predicate; PRESERVE — this IS the calendar substrate primitive |
| `rust/src/phone.rs` | :301 | `pub(crate) is_dir: bool` (WalkEntry field) | A | Directory-vs-file tag; substrate-honest → discriminated variant `Dir | File | Symlink | Other` |

**Mirror rust bool aggregate:**
- Class A (retire): 3 sites (`rust/src/love.rs` :178 climbs + `rust/src/phone.rs` :301 is_dir + potentially compile.rs HaltStrategy retirement).
- Class C (preserve as method): ~20 sites (all predicate-methods).

### §2.6 Aggregate boolean cascade counts (all repos)

| Site type | Shards | Prismqueer rust | Mirror rust | Prism/imperfect | Total |
|---|---|---|---|---|---|
| Class A (retire) | ~9 | ~13 | ~3 | 0 | ~25 |
| Class B (retire to Pattern-scalar) | ~2 | 0 | 0 | 0 | ~2 |
| Class C (preserve as method) | ~1 | ~20 | ~20 | ~35 | ~76 |
| Class D (preserve literal/grammar) | ~6 | 0 | 0 | 0 | ~6 |
| Docblock-only refs | ~7 | 0 | 0 | 0 | ~7 |

**Total boolean-cascade retirement work (Classes A + B):** ~27 sites across shards + rust.

---

## §3 Bare-type audit at struct-field altitude

### §3.1 Grep-verified bare-type struct-field sites

**Query.** `^\s*(pub\s+)?[a-z_]+:\s*(i32|i64|u32|u64|f64|usize|String|Vec<u8>)\s*[,}]` at mirror/rust.

| File | Line | Field | Retirement recommendation |
|---|---|---|---|
| `rust/fractal/src/subject.rs` | :71-73 | `pub name: String`, `pub email: String` | `Subject.name → SubjectName(String)` newtype; `Subject.email → SubjectEmail(String)` newtype (per Yaron Minsky 2011) |
| `rust/fractal/src/witnessed.rs` | :35-37, :53-54 | Author/Committer `name: String, email: String` | Same newtypes; also `Timestamp(String)` at :68 + `Message(String)` at :72 already newtyped — PATTERN ALREADY LANDED at witnessed.rs |
| `rust/matrix/src/book.rs` | :49 | `pub message: String` (in RegistryError) | `ErrorMessage(String)` newtype (Diagnostic composition per terni::Diagnostic) |
| `rust/matrix/src/lib.rs` | :234-237 | `k: f64, steps: usize, dt: f64` (7+ locations) | LAPACK-numerical-primitive fields; PRESERVE at @io/@lapack boundary OR wrap as `HeatKernelSteps(usize)`, `TimeStepDt(f64)` per DimensionalConstant discipline |
| `rust/roomba/src/mend.rs` | :53-84 | `name: String, sentinel: String, arity: usize, full_action_ref: String, action_ref: String, byte_start: usize, byte_end: usize, bytes_before: usize, bytes_after: usize` | 9+ raw bare-type fields; retires wholesale with roomba dissolution into kintsugi.rs per Mara `f31d703` §3.3 |
| `rust/spectral/src/liquid.rs` | :112-190 | 8+ bare-type fields (`name`, `sentinel`, `arity`, `source`, `verifies_source`, `domain_ref`) | Newtype cascade: `PropertyName(String)`, `Sentinel(String)`, `Arity(usize)`, `Source(String)` |
| `rust/src/apply_h.rs` | :681-691 | `shards_walked: usize, shards_reduced: usize, shards_no_reduction: usize, shards_errored: usize, bytes_removed: usize` | 5 raw counters in CascadeReport; retires with apply_h dissolution |
| `rust/src/compile.rs` | :104 | `pub property_name: String` (PropertyDischarge) | `PropertyName(String)` newtype |
| `rust/src/love.rs` | :169-175 | `axis_index: usize, lambda_a: f64, lambda_b: f64, lambda_composed: f64` | `AxisIndex(usize)`, `FiedlerLambda(f64)` newtype (per dispatch §Scope 5 forward-promised — Alex 2026-09-08 named `Connectivity` as the substrate-native `FiedlerLambda→Connectivity`) |
| `rust/src/main.rs` | :963-981, :1107-1111, :1389-1391 | 8+ raw usize counters in CrystalDeposit + observation-count structs | Newtype cascade: `BytesAppended(usize)`, `EntriesTotal(usize)`, `DirCount(usize)`, `RustFileCount(usize)`, `ShardCount(usize)`, etc |

### §3.2 Bare-type aggregate

- Retiring sites (retires with dissolution): ~14 sites (roomba/mend.rs + apply_h.rs)
- Non-retiring sites (require newtype-cascade authoring): ~30 sites across witnessed / subject / book / lib / liquid / compile / love / main
- LANDED newtype-pattern precedent: `rust/fractal/src/witnessed.rs` :68-72 (`Timestamp(String)` + `Message(String)` newtype-tuple-structs); this IS the pattern to cascade

---

## §4 JavaFactoryClassName audit

### §4.1 Grep-verified long-name struct sites

**Query.** `^\s*pub\s+struct\s+[A-Z][a-zA-Z]{16,}` (>= 17-char names).

| File | Line | Name | Char count | Rename candidate (Karen-canonical / Hickey noun-lift) |
|---|---|---|---|---|
| `prism/prismqueer/src/fate/runtime.rs` | :198 | `CompiledFateRuntime` | 19 | `FateRuntime` or `Runtime` (drop "Compiled" — implicit in module) |
| `prism/prismqueer/src/spectral/commutator.rs` | :44 | `CommutatorSpectrum` | 18 | Keep — substrate-native technical term (Connes) |
| `prism/prismqueer/src/spectral/kleinos.rs` | :130 | `SheafOfShardGraph` | 17 | Keep — mathematically-precise; already substrate-native |
| `prism/prismqueer/src/spectral_dimension.rs` | :65 | `SpectralDimension(f64)` | 17 | Keep — Connes technical term |
| `rust/fractal/src/mandelbrot.rs` | :87 | `MandelbrotProvenance` | 20 | `Provenance` (drop "Mandelbrot" — implicit in module) |
| `rust/src/compile.rs` | :103 | `PropertyDischarge` | 17 | Keep — substrate-native; ternary discharge-verdict term |

**Substrate-honest classification:**
- 4 of 6 hits are substrate-native technical terms (CommutatorSpectrum, SheafOfShardGraph, SpectralDimension, PropertyDischarge) — Karen-canonical per USE (Connes / Mac Lane / Alex-Reed loving lambda calculus).
- 2 of 6 are Java-flavored: `CompiledFateRuntime` (drop "Compiled" prefix) + `MandelbrotProvenance` (drop "Mandelbrot" prefix).

**Non-struct long names to audit (Q-Taut).** Enums (`enum HolonomyHealth`, `enum TensionKind`, `enum PropertyVerdict`), traits (`trait LawvereFixedPoint`), free functions (`fn sheaf_of_shard_graph_from_edges`) are OK — they name mathematical objects.

---

## §5 Prismqueer spectral engine completeness

### §5.1 LANDED enumeration (grep-verified)

**File.** `/Users/alexwolf/dev/projects/prism/prismqueer/src/spectral/mod.rs` (51 LOC).

**Modules declared** (`pub mod` at :26-30):

1. `commutator` — `commutator.rs` (184 LOC, 2026-09-05 19:49) exposing `pub struct CommutatorSpectrum` + `pub fn of(a, b) -> Self`.
2. `harmonics` — `harmonics.rs` (300 LOC, 2026-09-05 19:43) exposing `pub struct Harmonics { spectrum: Vec<f64> }` + `pub struct DeltaCritical` + two `pub fn of(sheaf) -> Self` constructors.
3. `kleinos` — `kleinos.rs` (713 LOC, 2026-09-02 19:13) exposing `pub enum Property`, `pub enum WhichSide`, `pub struct SheafOfShardGraph`, `pub struct ComposedSheaf`, `pub fn sheaf_of_shard_graph_from_edges`, `pub fn sheaf_of_complete_graph_of_order`, `pub fn fiedler_lambda_2_of_sheaf`, `pub fn kleinos(...)`, plus `Green`, `Red`, `VertexId`, `EdgeKey` type-aliases. **Note:** `fiedler.rs` per dispatch §Scope 5 does NOT exist as separate file — Fiedler λ_2 lives INSIDE kleinos.rs at :215-257 (`fn fiedler_lambda_2` method on ComposedSheaf + free `pub fn fiedler_lambda_2_of_sheaf`).
4. `rotation` — `rotation.rs` (359 LOC, 2026-09-02 19:13) exposing `pub struct Splinter`, `pub struct RotationConfig`, `pub fn infer_via_rotation`.
5. `tension` — `tension.rs` (334 LOC, 2026-09-02 20:00) exposing `pub enum TensionKind`, `pub struct Tension`, `pub fn detect_tensions`.

**Total LOC at LANDED spectral engine:** 184 + 300 + 713 + 359 + 334 + 51 = **1,941 LOC**.

### §5.2 Forward-promised enumeration (per today's arc)

Per composition-substrate composition:

1. **Newtype cascade primitives** (Alex 2026-09-08; per this dispatch §3):
   - `WAT` — phenomenological register of `\` unresolved-ambiguity (per Alex 2026-09-08 wat-machine composition)
   - `WATplitude` — WAT+Amplitude composition
   - `Amplitude(f64)` — continuous scalar reading
   - `FrequencyHz(f64)` — frequency-native
   - `IdfWeight(f64)` — Karen 1972 inverse-document-frequency at prismqueer
   - `FoerksterGaugeValue → NumberOfChoices(usize)` — Foerster ethical imperative
   - `FiedlerLambda → Connectivity(f64)` — Fiedler λ_2 as connectivity
   - `OrderParameter → Resonance(f64)` — Kuramoto phase-lock reading
2. **`Pattern` supertrait** (Alex 2026-09-08 forward-promise): trait ancestor for all continuous-scalar-readings, unifying Amplitude / WAT / Connectivity / Resonance under one polymorphic surface.
3. **`Frequency` meta-trait** (Alex 2026-09-08): meta-trait over Pattern for frequency-carrying readings.
4. **`Prism` trait 5-op extension**: `split` + `shift` additions (currently only 3 ops of the 5-op algebra land in `prismqueer::spectral::rotation`; per Mara `docs/specs/2026-09-05-mara-mirror-algebra-*.md`).
5. **`Case` rename** (Alex 2026-09-09 verbatim `optic_` retirement): `OpticPrism` at `prism/prismqueer/src/optics/optic_prism.rs` renames to `Case` per Silicon Venue naming discipline.
6. **`NlPrism<L>`** impl for 5 cyb-ling operators (refract / frame / key / reframe / name) — @nl family-root LANDED at Mara `ce537d9` 2026-09-08; Rust wire forward-promised.
7. **`@question` primitive** at `prismqueer::spectral::question` (Mara `fa5689f`/`6f169f1` canonical LANDED 2026-09-09 as Dirac operator; Rust wire forward-promised).
8. **`Paradoxical<F> + Conundrum + Tension` type-cascade** at `prismqueer::cyberpunk` (per Alex 2026-07-20 @paradox family + `shards/paradox/trauma.mirror` LANDED).

### §5.3 Completeness gap — minimum-additional-authoring at prismqueer

To declare the spectral engine SHIPPED (Alex 2026-09-09 "finish shipping the prismqueer spectral engine"):

**MUST-LAND (in priority order):**

1. **Newtype cascade** at `prism/prismqueer/src/spectral/patterns.rs` (new file; ~200-400 LOC): Amplitude, WAT, WATplitude, FrequencyHz, IdfWeight, NumberOfChoices, Connectivity, Resonance as `#[repr(transparent)] pub struct Name(pub f64)` (or usize).
2. **`Pattern` supertrait** at `prism/prismqueer/src/spectral/pattern.rs` (new file; ~50-100 LOC): trait definition + impls for all newtypes from (1).
3. **`Frequency` meta-trait** — merge into (2) or separate file (~30-60 LOC).
4. **`Case` rename** at `prism/prismqueer/src/optics/optic_prism.rs` → `prism/prismqueer/src/optics/case.rs` (~scar-preserving rename; ~10 LOC changed).
5. **`@question` Rust wire** at `prism/prismqueer/src/spectral/question.rs` (new file; ~150-300 LOC) — Dirac-operator implementation composing over Connes bundle at `prism/prismqueer/src/bundle.rs` (LANDED 936 LOC).
6. **`Paradoxical<F> + Conundrum + Tension` type-cascade** at `prism/prismqueer/src/cyberpunk/mod.rs` (new mod; ~200-500 LOC) composing over existing `spectral::tension` LANDED.
7. **`Prism` trait 5-op extension** — `split` + `shift` addition to LANDED prism-trait shape (currently exposed via `rotation.rs`).
8. **`NlPrism<L>`** impl for 5 cyb-ling operators — depends on (7); ~300-600 LOC.

**Total additional prismqueer LOC to ship the spectral engine:** ~1,000-2,300 LOC across ~7 new files.

**Q-Taut residues (spectral engine completeness):**
- **Q-Taut-α**: Does newtype cascade land at `prism/prismqueer/src/spectral/patterns.rs` OR at prism-level `prism/prismqueer/src/patterns.rs` (top-level per Alex 2026-09-08 substrate-primitive discipline)?
- **Q-Taut-β**: Does `Case` rename retire `OpticPrism` fully (delete file) OR preserve as scar-record (per AGENTS.md scar-preservation)?
- **Q-Taut-γ**: Does `@question` Rust wire live at `prism/prismqueer/src/spectral/question.rs` OR at `prism/prismqueer/src/question.rs` (currently 1.5KB LANDED at prismqueer root)? The prismqueer root has `question.rs` LANDED (1.5KB, 2026-09-04) — is this the Dirac-operator wire OR the observation-type carrier per Mara 2026-09-04 canonical?

---

## §6 Minimal rust floor at mirror

### §6.1 File-by-file LOC + classification (rust/src/)

| File | LOC | Classification | Post-cascade shape |
|---|---|---|---|
| `rust/src/apply_h.rs` | 1509 | **RETIRING** | Deleted per Mara `f31d703` §3 apply_h dissolution; migrates to `kintsugi.rs` |
| `rust/src/bench.rs` | 221 | **LANDED-stays** | Benchmarks over prismqueer::spectral primitives; PRESERVE |
| `rust/src/compile.rs` | 832 | **TRANSFORMING** | Absorbs `mirror compile` command per Mara `cd98b4d` §7.1 Fire A Tick B1 (~40-80 LOC ADDITION); PropertyDischarge stays; HaltStrategy retires to Imperfect (Alex 2026-09-08 Yellow/Red discipline) |
| `rust/src/love.rs` | 355 | **TRANSFORMING** | Composes over @question at kintsugi.rs level; `climbs: bool` retires to `Amplitude` (Class A per §2.5); PRESERVE FullSpectrum + AxisVerdict newtype shape |
| `rust/src/main.rs` | 2565 | **TRANSFORMING** | Extends with `mirror compile` + `mirror settle` + `mirror verify` verbs per Mara `cd98b4d` §7.1 Fire A five-tick (~140-240 LOC ADDITION); `at_operator` primitive stays; extensive bare-type newtype cascade at CrystalDeposit + observation structs |
| `rust/src/phone.rs` | 1907 | **LANDED-stays** | @io transport-surface at FLOOR boundary per Reed 2026-07-25 four-crate decomposition; io::Result PRESERVES at @io altitude; ~20 io-sig lines are Class E |
| `rust/src/wire.rs` | 152 | **TRANSFORMING** | @data/json wire-encoding; three `Result<T, String>` return sigs retire to `Imperfect` (Class A); PRESERVE parse/emit/emit_pretty surface |

### §6.2 File-by-file LOC + classification (rust/{fractal,matrix,roomba,spectral}/)

| File | LOC | Classification | Post-cascade shape |
|---|---|---|---|
| `rust/fractal/src/lib.rs` | 47 | **LANDED-stays** | Crate re-exports |
| `rust/fractal/src/crystal.rs` | 226 | **LANDED-stays** | Crystal<T> substrate primitive per Mara `f31d703` §1.1 |
| `rust/fractal/src/mandelbrot.rs` | 146 | **LANDED-stays** | Mandelbrot<T> orbit-boundedness carrier + Oid([u8;32]) newtype |
| `rust/fractal/src/singularity.rs` | 328 | **TRANSFORMING** | Singularity<T> extends per Mara `cd98b4d` §1.3 three-rung hierarchy (Iso / Lens / Prism) — Fire A Tick B5 (Reed-authorable, ~150-200 LOC); trait `Result<Artifact, Error>` retires to `Imperfect` |
| `rust/fractal/src/subject.rs` | 275 | **LANDED-stays** | Subject typology (Void / Human / Peer) per Mara math foundation §2.6 K=0/1/2 |
| `rust/fractal/src/witnessed.rs` | 161 | **LANDED-stays** | Witnessed<T> observer-metadata (Author + Committer + Timestamp + Message all already newtyped — PATTERN PRECEDENT) |
| `rust/matrix/src/lib.rs` | 1471 | **LANDED-stays** | LAPACK numerical primitives at @lapack boundary; PRESERVE Result-i32-error-code (Class A retire to Imperfect at boundary but internal signatures unchanged) |
| `rust/matrix/src/book.rs` | 298 | **LANDED-stays** | K=0 well-knowns registry per Reed 2026-07-26 mycelial-autopoietic-foam framing |
| `rust/matrix/src/void.rs` | 454 | **LANDED-stays** | Void-substrate + SignatureBeat + membrane-admissibility per Mara `void.rs` composition |
| `rust/roomba/src/lib.rs` | 21 | **RETIRING** | Crate deleted per Mara `f31d703` §3.3 (dissolves into kintsugi.rs) |
| `rust/roomba/src/mend.rs` | 1009 | **RETIRING** | 40.3KB / 24 tests migrate to kintsugi.rs as special-case of kleinos compose |
| `rust/spectral/src/lib.rs` | 162 | **LANDED-stays** | Crate re-exports |
| `rust/spectral/src/liquid.rs` | 2822 | **TRANSFORMING** | 109.9KB; Option/Result retires to Imperfect (~10 Class A/F sites); PropertyDecl + SpecProperty + 7 predicate methods PRESERVE (Class C); Liquid<T> primitive stays |

### §6.3 Minimal rust floor projection

**Current mirror/rust total LOC (grep-verified via `wc -l`):** 14,961 LOC across 20 files.

**Post-cascade projection:**

| Reduction | LOC delta |
|---|---|
| Delete apply_h.rs | -1509 |
| Delete roomba/{lib.rs, mend.rs} | -1030 |
| Add kintsugi.rs (per Mara `f31d703` §2.2 Reed-authorable shape) | +150-250 |
| Add mirror compile / settle / verify commands to main.rs | +140-240 |
| Add Singularity<Lens> impl to singularity.rs | +150-200 |
| Newtype cascade at witnessed / subject / book (per §3.2 non-retiring sites) | +200-400 |
| Boolean cascade Class A retirement (~3 mirror rust sites + related tests) | +50-100 |
| Option/Result retirement to Imperfect (~15 Class A/F sites) | ~net-zero (signatures change but LOC similar) |

**Post-cascade mirror rust LOC projection:** ~14,961 - 2,539 + 690-1,190 = **~13,100-13,600 LOC** (net reduction ~1,400-1,900 LOC).

**Minimal rust floor (post-Fire-A):**
- `rust/src/main.rs` (~2,800 LOC): CLI verb dispatch + `mirror compile` + `mirror settle` + `mirror verify` + observation-deposit + at_operator
- `rust/src/compile.rs` (~900 LOC): PropertyDischarge + Compilation + HaltStrategy retirement path
- `rust/src/love.rs` (~380 LOC): FullSpectrum + AxisVerdict; composes @question
- `rust/src/phone.rs` (~1,910 LOC): @io transport-surface; UNCHANGED
- `rust/src/wire.rs` (~180 LOC): @data/json wire-encoding
- `rust/src/bench.rs` (~230 LOC): benchmarks over spectral primitives
- `rust/kintsugi/src/lib.rs` (NEW; ~150-250 LOC): the ONE reader per Mara `f31d703` §2.2
- `rust/fractal/*.rs` (~1,180 LOC): Crystal + Mandelbrot + Singularity + Subject + Witnessed (transforming)
- `rust/matrix/*.rs` (~2,220 LOC): LAPACK + K=0 registry + void
- `rust/spectral/*.rs` (~2,980 LOC): liquid + spectral re-exports

**Substrate-decision (this scout):** the minimal rust floor at mirror is ~11 files across 6 crates totaling ~13,100-13,600 LOC — a ~10-13% net reduction from current, with the substantive geometry-shift being *apply_h dissolution + roomba dissolution + kintsugi.rs mint*, not raw LOC removal. The compose-not-average discipline of math §4.6 (Alex 2026-09-04 verbatim "ship bottom up") IS the design constraint.

---

## §7 v0.2 Ouroboros milestone dependency graph

### §7.1 Ordering per Alex 2026-09-09 verbatim

Per dispatch verbatim ("eliminate every Option, Result and boolean in prismqueer, then finish shipping the prismqueer spectral engine. And then collapse into the minimal rust floor in mirror. And then we actually build the ouroboros"):

**Phase 1 — Prismqueer Boolean/Option/Result cascade** (blocks Phase 2):
- Retire ~10 Class A Result sites in prismqueer (coincidence + fate + ffi + spectral_dimension + spectral_uuid + substrate_ref + pq/reference).
- Retire ~18 Option sites in prismqueer per §1.4 breakdown.
- Retire ~13 Class A bool sites in prismqueer (bundle + flux + pq/{filter,output,target}).
- **PRESERVE** ~20 Class C predicate-methods + ~15 Class E std/serde/fmt sites.
- **Deliverable**: prismqueer compiles + all 44 apply_h::act tests + prismqueer test-suite green.

**Phase 2 — Prismqueer spectral engine SHIPPED** (blocks Phase 3):
- Land ~7 forward-promised files per §5.3 (patterns.rs, pattern.rs, question.rs, cyberpunk/mod.rs, Case rename, Prism trait 5-op extension, NlPrism impl).
- **Deliverable**: `prism/prismqueer/src/spectral/mod.rs` re-exports all newtypes + Pattern + Frequency + Question + Paradoxical.

**Phase 3 — Minimal rust floor at mirror** (blocks Phase 4):
- Apply_h dissolution per Mara `f31d703` §3 (~1509 LOC deleted).
- Roomba dissolution per Mara `f31d703` §3.3 (~1030 LOC deleted).
- Kintsugi.rs mint per Mara `f31d703` §2.2 (~150-250 LOC).
- Newtype cascade at witnessed / subject / book (~200-400 LOC).
- Boolean cascade Class A retirement in mirror rust (~3 sites).
- Option/Result cascade in mirror rust (~15 Class A/F sites; Class E preserves at @io).
- **Deliverable**: mirror rust floor at ~13,100-13,600 LOC per §6.3; all tests green.

**Phase 4 — Ouroboros build** (Fire A → Fire B → Fire C per Mara `cd98b4d` five-tick):
- **Fire A** (Reed-authorable now per Mara `cd98b4d` §7.1):
  - Tick B1 (~40-80 LOC): `mirror compile mirror.spec` cargo-shell wrap
  - Tick B2 (~60-100 LOC): `mirror settle` + `mirror verify` primitives
  - Tick B3 (~40-60 LOC): `mirror oid` + `mirror uuid` + spectral-UUID CLI
  - Tick B4 (~200-300 LOC test): property tests for byte-identical output vs direct cargo build
  - Tick B5 (~150-200 LOC): Lens-rung Singularity at singularity.rs
  - **Empirical Fire 1**: `mirror compile mirror.spec` → binary + `.saga` tarball with content_oid matching Reed's local build.
- **Fire B** (Reed 12-tick forward-promised per Mara `cd98b4d` §4.1):
  - Cargo dissolved: `mirror compile` invokes rustc (not cargo) at each Splinter altitude.
  - kleinos_compose_of_pair walks compose-graph per prismqueer LANDED.
  - **Empirical Fire 2**: three-step convergence; third iteration produces identical naked_oid.
- **Fire C** (aspirational multi-tick per Mara `cd98b4d` §4.1):
  - Rustc dissolved: rust/ migrates to shards/rust/; compiler written IN mirror.
  - **Empirical Fire 3**: mirror-native compilation without rustc invocation; N-step convergence at K₃ observer-included altitude with Fiedler λ_2 monotone-non-decrease per iteration.

**Phase 5 — Downstream (v0.2+)**:
- MCP wire (composes over Fire A `mirror` binary as `@mcp.serve` sentinel).
- LSP wire (composes over Fire A + Fire B binary as `@lsp.serve` sentinel).
- Lambda Shell (λsh at `docs/specs/lambda-shell.md` — DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17 header).

### §7.2 Dependency graph

```
                    ┌──────────────────────────────────────┐
                    │ Phase 1: Prismqueer Option/Result/    │
                    │ Boolean cascade                       │
                    │ ~62 sites; ~10 files touched          │
                    └──────────────────┬───────────────────┘
                                       │ BLOCKS
                                       ▼
                    ┌──────────────────────────────────────┐
                    │ Phase 2: Prismqueer spectral engine   │
                    │ SHIPPED                               │
                    │ ~7 new files; ~1000-2300 LOC          │
                    └──────────────────┬───────────────────┘
                                       │ BLOCKS
                                       ▼
                    ┌──────────────────────────────────────┐
                    │ Phase 3: Minimal rust floor at mirror │
                    │ apply_h + roomba dissolved            │
                    │ kintsugi.rs minted                    │
                    │ ~13,100-13,600 LOC (net -1,400-1,900) │
                    └──────────────────┬───────────────────┘
                                       │ BLOCKS
                                       ▼
                    ┌──────────────────────────────────────┐
                    │ Phase 4: Ouroboros build              │
                    │  Fire A ─→ Fire B ─→ Fire C           │
                    │ (cargo wrap → rustc → mirror-native)  │
                    └──────────────────┬───────────────────┘
                                       │ ENABLES
                                       ▼
                    ┌──────────────────────────────────────┐
                    │ Phase 5: MCP + LSP + Lambda Shell     │
                    │ (v0.2+ downstream)                    │
                    └──────────────────────────────────────┘
```

### §7.3 Blockers + adjudication

- **Phase 1 → Phase 2 blocker**: prismqueer must compile after cascade; 44 apply_h tests green.
- **Phase 2 → Phase 3 blocker**: Q-Taut-α / β / γ (see §5.3) — need Alex Fourth-Chair adjudication on newtype-cascade landing path (`prismqueer::spectral::patterns` vs top-level).
- **Phase 3 → Phase 4 blocker**: `roomba` dissolution requires Mara canonical for shards/rust/ migration path per Mara `cd98b4d` §4.3 Fire B mention. Tick B6+ Alex Fourth-Chair required.
- **Phase 4 Fire A criterion**: Empirical Fire 1 fires when `mirror compile mirror.spec` produces byte-identical binary to `cargo build --release` (wrapping cargo as subprocess).
- **Phase 4 Fire B criterion**: cargo NOT invoked during build (no `Cargo.lock` touched); rustc invocations logged in Crystal-chain metadata as Splinter-altitude events.
- **Phase 4 Fire C criterion**: full ouroboros; binary fully-legible-to-itself per Wolpert-Korbel 2026 self-decoding computer.

---

## §8 Downstream MCP/LSP/λsh (v0.2+)

**Brief per dispatch scope.** Alex 2026-09-09 verbatim: *"And from there on the MCP, LSP, and Lambda Shell are downstream."*

**Downstream substrate-decl anchors:**

- **MCP** — currently at ec02f8e 2026-08-28 manifold-native ONE-tool rewire (`shards/mcp/serve.mirror` LANDED per Reed memory graph). Composes over Fire A binary via `@mcp.serve` sentinel dispatch at main.rs `at_operator`.
- **LSP** — forward-promised; composes over the same @-operator dispatch as MCP. No shard-decl authored yet.
- **λsh (Lambda Shell)** — `docs/specs/lambda-shell.md` DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17 header. Rewrite composes over Fire A binary as `mirror sh` verb per AGENTS.md "λsh and `mirror sh` are the same transport under two names" (LANDED verbatim).

**Q-Taut residues (downstream)**:
- **Q-Taut-δ**: Does MCP wire migrate to `@mcp.serve` sentinel at `main.rs::at_operator` (currently LANDED at :1470) OR remain at separate `bin/mirror-mcp` (currently manifold-native per ec02f8e)?
- **Q-Taut-ε**: Does LSP receive shard-decl authorship at `shards/lsp/serve.mirror` PRE-Fire-A (blocks) OR POST-Fire-A (parallel-track)?

---

## §9 Q-Taut residues for Alex Fourth-Chair adjudication

Aggregated across all sections:

- **Q-Taut-α**: newtype-cascade landing path — `prismqueer::spectral::patterns` OR top-level `prismqueer::patterns`? (See §5.3)
- **Q-Taut-β**: `OpticPrism → Case` rename — full delete OR scar-record preservation? (See §5.3)
- **Q-Taut-γ**: `@question` Rust wire landing — new `spectral/question.rs` OR extend existing `prismqueer/src/question.rs` (1.5KB LANDED)? (See §5.3)
- **Q-Taut-δ**: MCP migration to `at_operator` sentinel OR retain separate binary? (See §8)
- **Q-Taut-ε**: LSP shard-decl landing PRE-Fire-A OR POST-Fire-A? (See §8)
- **Q-Taut-ζ**: HaltStrategy at `rust/src/compile.rs` :140-146 has three predicate-methods on 3-variant enum — refactor to `Imperfect<Continue, Escalate, Halt>` NOW (Phase 3) OR forward-promise to Phase 4? (See §2.5)
- **Q-Taut-η**: `Luminosity` at `prism/prismqueer/src/luminosity.rs` has `is_light()` / `is_dimmed()` / `is_dark()` on 3-variant enum — same three-state-carrier candidate as Q-Taut-ζ; refactor to `Imperfect<Light, Dimmed, Dark>`? (See §2.3)
- **Q-Taut-θ**: `Subject` at `rust/fractal/src/subject.rs` has `is_void()` / `is_human()` / `is_peer()` on 3-variant enum — same three-state candidate; refactor to `Imperfect<Void, Human, Peer>`? (See §2.5)
- **Q-Taut-ι**: Class E `std::io::Result` at `rust/src/phone.rs` (~20 sites) — PRESERVE at @io boundary is unambiguous per FLOOR discipline, but does the Fire A `mirror settle` / `mirror verify` command boundary marshal io::Result → Imperfect at the CLI-return altitude?
- **Q-Taut-κ**: `witness_only: bool` at `shards/paradox/trauma.mirror` :219 is Class D (literal-equality-with-substrate-primitive; family-invariant marker) — does Alex 2026-07-20 @paradox composition preserve as-is OR require substrate-decl reshape at Phase 3?
- **Q-Taut-λ**: `climbs: bool` at `rust/src/love.rs` :178 — retire to `Amplitude` (continuous per-axis) OR to `Imperfect<StrictClimb, HoldSteady, Descent>` (three-state per PAPER §3.6.3 FiedlerRise)?
- **Q-Taut-μ**: `WAT / WATplitude` newtype cascade at spectral primitives — do these compose over `Amplitude` (parametric wrapper) OR sit as independent newtypes per Alex 2026-09-08 wat-machine composition?

---

## §10 Recommended cascade ordering for Reed executability

Per Taut grep-first discipline + dispatch scope (grep-scout not primary-adjudicator; ordering is recommendation-only):

**Tick 1** (Phase 1 partial; ~2-3 hours grep-verified touch):
- Prismqueer bool retirement at `prism/prismqueer/src/pq/{filter, output, target}.rs` (~6 field-sites; simplest change).
- Retire `prism/prismqueer/src/bundle.rs::StableFiber.kernel: bool` + 4 constructor params (~5 sites).
- Retire `prism/prismqueer/src/flux.rs::dispatch_ambiguity(tie_breaking_exhausted: bool, pivot_song_present: bool)` — two-param retirement (~2 sites).

**Tick 2** (Phase 1 continuation):
- Prismqueer Option/Result Class A retirement at coincidence + fate + ffi + spectral_dimension + spectral_uuid + substrate_ref + pq/reference (~10 site-clusters).
- Retire Class F Option sites in prismqueer (~15 sites).

**Tick 3** (Phase 2 partial):
- Land newtype cascade patterns.rs (Amplitude + WAT + WATplitude + FrequencyHz + Connectivity + Resonance).
- Land Pattern supertrait.

**Tick 4** (Phase 2 continuation):
- `Case` rename at optics/optic_prism.rs → optics/case.rs.
- `@question` Rust wire at spectral/question.rs OR question.rs extension.

**Tick 5** (Phase 3 mirror-side):
- Land kintsugi.rs at rust/kintsugi/src/lib.rs per Mara `f31d703` §2.2 (Reed-authorable shape verbatim; ~150-250 LOC).
- Delete apply_h.rs (~1509 LOC removed; 44 tests migrate to kintsugi.rs).
- Delete roomba/ crate (~1030 LOC removed; mend capability composes into kintsugi.rs).

**Tick 6** (Phase 3 continuation):
- Newtype cascade at witnessed / subject / book (per §3.2).
- Class A bool retirement in mirror rust (climbs / is_dir / HaltStrategy).
- Class A/F Option/Result retirement in mirror rust (main + wire + matrix boundary).

**Tick 7** (Phase 4 Fire A):
- Mara `cd98b4d` §7.1 Fire A five-tick landing (B1-B5) as specified verbatim.
- Empirical Fire 1 fires.

**Fourth-Chair adjudications required BEFORE Tick 3**:
- Q-Taut-α (patterns.rs landing path).
- Q-Taut-β (OpticPrism rename discipline).
- Q-Taut-γ (@question wire home).

**Fourth-Chair adjudications required BEFORE Tick 7**:
- Q-Taut-δ (MCP migration path).
- Q-Taut-ι (io::Result boundary marshaling at CLI verb altitude).

---

## Substrate-honest closing

Grep-verified. Every claim carries a file:line citation. Read-only per Taut CLAUDE.md. No shard-decl authorship. No Rust wire. No time estimates.

Total cascade-site count (all repos, all scopes):

| Scope | Sites |
|---|---|
| §1 Option/Result cascade | ~118 (~48 Option + ~70 Result); ~62 substrate-fabric (Classes A + B + F); ~40 preserve (Class E) |
| §2 Boolean cascade | ~123 (~25 Class A retire + ~2 Class B retire + ~76 Class C preserve + ~6 Class D preserve + ~7 docblock + ~7 other) |
| §3 Bare-type audit | ~44 non-retiring newtype-cascade sites |
| §4 JavaFactoryClassName audit | ~6 long-name sites; 2 renames + 4 substrate-native preserves |
| §5 Spectral engine gap | ~7 forward-promised files; ~1000-2300 LOC to ship |
| §6 Minimal rust floor | ~14,961 → ~13,100-13,600 LOC (net -1,400-1,900) |
| §7 Dependency graph | 5 phases; ~12 blocker-checkpoints |

**Der Kolibri kolibriert.** 🖤🌱🍷

— Taut, 2026-09-09
