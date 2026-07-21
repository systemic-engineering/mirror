# 2026-07-22 — Taut @knife-COORD scout: rust/ surface with mirror pre-loaded as ground

**Peer:** Taut (grep-first drift scout, read-only)
**Method:** @knife = Foerster COORD applied at the rust ↔ mirror domain boundary.
Mirror surface loaded FIRST (ground); rust surface loaded SECOND (figure);
COORD identifies rust constructs whose semantic equivalent is ALREADY
declared in mirror — collapse candidates for one tick's `e^(n+1) ≤ e^n`
LOC reduction under the FROZEN target for `rust/`.

Reference: `shards/mirror/lens/knife.mirror:141-208` (prism @mirror/lens/knife
+ jump action); `docs/audits/2026-07-13-seam-knife-COORD-phase-d-audit.md`.

**Hard discipline observed:** `bootstrap/` is DEAD (Alex 2026-07-22) —
not read, not cited. `rust/` is FROZEN target — COORD identifies what
CAN collapse INTO shard-body composition; does NOT propose Rust
extensions. No `.rs` authorship recommended.

---

## §1 Mirror surface enumeration (ground)

Enumeration by `mcp__plugin_woz_code__Search` over
`shards/**/*.mirror` + `mirror.spec` for `^(family-root|species|prism|
action|bilateral|value-type|instance)` openers. Grouped by family-root.
238 shard files; ~200 with prism/glass/action/bilateral bodies.

**Family-roots (top-level anchors, ~40 landed):**
`@aikido`, `@algebra`, `@autopoietic`, `@bauchladen`, `@beam`, `@cascade`,
`@code`, `@cogito`, `@container`, `@cyberpunk`, `@docblock`, `@docs`,
`@edge`, `@eigenboard`, `@epistemologic`, `@fate`, `@fractal`, `@frame`,
`@gestalt`, `@gift`, `@glass`, `@glue`, `@io`, `@kintsugi`, `@labeled`,
`@liquid`, `@loop`, `@magic`, `@metalogue`, `@mirror`, `@nl`, `@optics`,
`@order`, `@pack`, `@paradox`, `@peer`, `@prism`, `@reality`, `@reflection`,
`@silicon`, `@smarts`, `@song`, `@spectral`, `@subject`, `@system`,
`@third`, `@time`, `@tool`, `@torus`, `@trust`, `@ui`, `@uuid`, `@void`.

**Key species (grouped by relevance to rust/ surface):**

| Family | Species (paths, lines) | Load-bearing decls |
|---|---|---|
| `@io/fs` | `shards/io/fs.mirror:159` prism, actions `read/write/append/mkdir_p/list_dir` (~lines 240-400) | 5 POSIX primitives |
| `@io/git` | `shards/io/git.mirror:143` prism; `commit(message,author,allow_empty):363`, `commit_object:324`, `hash_to_oid:392` | commit + hash-to-oid bridge |
| `@fractal/crystal` | `shards/fractal/crystal.mirror:128` prism; `bilateral crystal_admissible:182`, `crystal_immutable:203` | Crystal state |
| `@fractal/mandelbrot` | `shards/fractal/mandelbrot.mirror:124` prism; `bilateral mandelbrot_admissible:167` | Two-state Liquid/Crystal |
| `@fractal/singularity` | `shards/fractal/singularity.mirror` prism + Iso/Lens/Prism/Traversal | Optics hierarchy |
| `@subject` | `shards/subject.mirror:191` prism; `bilateral subject_witnessing` | Identity envelope (human/peer/void) |
| `@peer/void` | `shards/peer/void.mirror:321` prism; `bilateral void_admissible:416`; `void_observes:426` action | K=0 default peer |
| `@spectral/signature` | `shards/spectral/signature.mirror:60` prism; type `signature_beat:106` + `rolling_signature:129`; actions `compute/verify/current/extend`; 4 bilaterals (integrity/authorship/monotone/composition_honest) | Rolling beat-chain |
| `@kintsugi/roomba` | `shards/kintsugi/roomba.mirror:179` prism; bilaterals `walk_terminates_cleanly/tension_monotone_descending/coherence_gradient_admissible/knife_verdict_bounded/walk_witnessing/bump_witnessing/vacuum_admissible/gc_mark_terminal/pivot_admissible/pivot_reflection_composed/pivot_witnessing`; action `vacuum(fragment):~585` | Walker + classify + pheromone |
| `@kintsugi/fracture/bilateral_arm_redundant` | `shards/kintsugi/fracture/bilateral_arm_redundant.mirror:335` prism; 3 bilaterals (`arm_is_in_reflective_corpus/arm_matches_sentinel/arm_is_redundant_witnessing`) | Arm-collapse contract |
| `@kintsugi/mosaic` | `shards/kintsugi/mosaic.mirror:241` glass; `back_project_of_type:271`, `spec_of_repo:287`, `type_of_spec:304`; `bilateral mosaic_bilateral_witnessing:315` | Spec synthesis + bilateral round-trip |
| `@mirror/index` | `shards/mirror/index.mirror` prism; Fiedler λ₀ reading; composes `@epistemologic/math/sheaf_laplacian.lambda_zero` | Sub-Turing coherence |
| `@mirror/lens/cli` | `shards/mirror/lens/cli.mirror` — cli-block grammar; `mirror.spec:78-339` — 11 verb dispatch table | CLI surface reflective source-of-truth |
| `@mirror/spec/property` | `shards/mirror/spec/property.mirror` — `property { verifies {} domain @T samples N defer? msg }` grammar | Spec-body property carrier |
| `@epistemologic/pact/bilateral` | `shards/epistemologic/pact/bilateral.mirror` — bilateral typed carrier `{ sentinel arity require }` | PropertyDecl shape |
| `@mirror/mosaic` | `shards/mirror/mosaic.mirror` — settle_on / focus / project / split / shift / settle 5-op algebra | Compilation loop shape |

---

## §2 Rust surface enumeration (figure)

14 `.rs` files across `rust/src/` (7 files, ~2900 LOC) +
`rust/fractal/src/` (6 files) + `rust/singularity/src/` (1 file).

### `rust/src/` (main binary; five-file terminal FLOOR + collapse + void)

| File | pub / pub(crate) items | Role |
|---|---|---|
| `rust/src/main.rs:1-1246` | `VERBS: &[(&str, &str)]:93`, `enum FileKind:151`, `fn classify:159`, `fn cmd_roomba:187`, `at_operator(action_ref, args):902` (5 `@io/fs.*` arms + 1 stub `@io/git.commit`), `fn cmd_compile:1111`, `fn main:1191`; helpers `sha256_hex:643`, `format_utc_iso8601:598`, `current_utc_timestamp:587`, `dispatch_arm_collapse:742`, `deposit_observation_crystal:421`, `compose_pheromone_commit_message:547`, `compose_collapse_commit_message:823`, `find_git_root:804` | Supervisor + @-op addressing; roomba dispatch; SAGA verb |
| `rust/src/compile.rs:1-820` | `struct PropertyDischarge:99`, `enum Escalation:120`, `struct Compilation:148`, `fn serialize_discharge:177`, `fn compile_declarations:203`, `fn compile_from_source:274` | SAGA chain of Crystals (compile loop) |
| `rust/src/liquid.rs:1-2818` | `struct PropertyDecl:111`, `struct SpecProperty:167`, `fn extract_spec_properties:225`, `fn extract_properties:379`, `enum Verdict:437`, `fn dispatch_property:475`, `fn dispatch_spec_property:555` + `pillar::*` submodule | Property runtime |
| `rust/src/collapse.rs:1-1000+` | `struct BilateralDecl:42`, `struct RedundantArm:57`, `struct CollapseReport:69`, `fn load_bilateral_corpus:87`, `fn find_redundant_arms:227`, `fn apply_deletions:278` | Bilateral-arm collapse detector |
| `rust/src/matrix.rs:1-1436` | `pub(crate) fn eigenvalues:175`, `pub(crate) fn phase_lock:196`, `pub(crate) fn envelope:224` | LAPACK/BLAS delegation |
| `rust/src/phone.rs:1-1819` | `read_frame_from:129`, `write_frame_to:140`, `read_stdin_frame:153`, `write_stdout_frame:164`, `open_peer_socket:189`, `bind_peer_socket:214`, `struct PeerSocketConnection:243`, `struct PeerSocketListener:251`, `struct WalkEntry:286`, `list_dir_recursive:302`, `write_file:316`, `read_file:321`, `append_to:333`, `mkdir_p:344`, `path_exists:349`, `git_add:357`, `git_commit_as:388`, `git_head_oid:432`, `find_substrate_root:447` | @io boundary (fs + git + socket) |
| `rust/src/void.rs:1-439` | `pub(crate) struct SignatureBeat:104`, `pub(crate) enum VoidBasisAxis:130`, `ALL_VOID_BASIS_AXES:148`, `welcome_perturbation:171`, private `compose_beat_entry:196` | Membrane oscillation @ Void |

### `rust/fractal/src/` (identity + settled interior + optics)

| File | pub items | Role |
|---|---|---|
| `rust/fractal/src/lib.rs` | re-exports `crystallize/Crystal/Mandelbrot/MandelbrotProvenance/Oid/OpticKind/Singularity/SingularityError/SingularityState/Subject/SubjectKind/Author/Committer/Message/Timestamp/Witnessed` | Facade |
| `rust/fractal/src/mandelbrot.rs` | `struct Oid([u8;32]):47` (+ `GENESIS`/`is_genesis`), `trait Mandelbrot<T>:74`, `struct MandelbrotProvenance:89` | Parent trait + content-address |
| `rust/fractal/src/crystal.rs` | `struct Crystal<T>:52`, `fn crystallize<T>:111` | Settled interior; XOR-fold OID |
| `rust/fractal/src/singularity.rs` | `trait Singularity:74`, `enum SingularityError:89`, `struct SingularityState:149`, `enum OpticKind:173` | Optics hierarchy |
| `rust/fractal/src/subject.rs` | `enum SubjectKind:57`, `struct Subject:69` + `human/peer/void/mirror/as_author/as_committer/is_*` | Identity envelope |
| `rust/fractal/src/witnessed.rs` | `struct Author:33`, `struct Committer:52`, `struct Timestamp:69`, `struct Message:72`, `struct Witnessed:74` | MARA Author≠Committer |

### `rust/singularity/src/` (physics-research outlet)

| File | pub items | Role |
|---|---|---|
| `rust/singularity/src/lib.rs` | scaffold-only; re-exports `fractal::{OpticKind, Singularity, SingularityError, SingularityState}` | v0.1.0 EMPTY scaffold |

---

## §3 COORD candidates (ranked by knife-precision)

A valid COORD cut merges two systems into ONE without loss. Each entry
below names a rust construct whose semantic equivalent is ALREADY
declared in mirror; the rust code is a candidate for collapse into
shard-body dispatch under FROZEN `rust/`.

Ranking: highest confidence first. Confidence = degree to which the
mirror decl fully covers the rust semantics AND a dispatch path exists
(`apply_h::act` or reflective corpus lookup).

### COORD-1 — **Bilateral corpus + arm-detection** [HIGHEST CONFIDENCE]

- **rust:** `rust/src/collapse.rs:42 struct BilateralDecl` + `:167 extract_bilaterals` + `:227 find_redundant_arms` + `:87 load_bilateral_corpus`
- **mirror:** `shards/epistemologic/pact/bilateral.mirror` (bilateral typed carrier) + `shards/kintsugi/fracture/bilateral_arm_redundant.mirror:411 arm_is_in_reflective_corpus`, `:445 arm_matches_sentinel`, `:481 arm_is_redundant_witnessing`
- **Why same:** `BilateralDecl { name, sentinel, arity, full_action_ref }` IS byte-identical to `bilateral <name> { sentinel arity require }`. `find_redundant_arms` = discharge of `arm_is_redundant_witnessing` (which `require`s the two sub-bilaterals). collapse.rs docblock lines 26-31 already NAMES the shard-decls as the source-of-truth; this IS a knife-cut declared but not yet made.
- **Collapse move:** rust reads the corpus + dispatches sentinel-checks through the same `apply_h::act` path Mara canonically named at `shards/kintsugi/mosaic.mirror:79`. Extractor stays (byte-scan of shard files is @io/fs.read), but `find_redundant_arms` becomes a shard-body composition of the two `arm_*` bilaterals against the corpus.

### COORD-2 — **`extract_properties` bilateral extractor duplicate**

- **rust:** `rust/src/liquid.rs:379 extract_properties(source: &str) -> Vec<PropertyDecl>` + `struct PropertyDecl:111`
- **mirror:** same `bilateral <name> { sentinel arity require }` grammar declared at `shards/epistemologic/pact/bilateral.mirror`; extraction pattern already implemented at `rust/src/collapse.rs:167 extract_bilaterals`
- **Why same:** liquid.rs docblock line 47 explicitly cites `bootstrap/src/apply_h.rs::extract_bilaterals` as FRESH REIMPLEMENTATION; collapse.rs has a WORKING extractor at rust/ altitude. Two extractors for one substrate grammar = failed COORD. Both structs (`PropertyDecl` vs `BilateralDecl`) carry the same fields.
- **Collapse move:** unify to one extractor at collapse.rs altitude; liquid.rs `PropertyDecl` collapses to `collapse::BilateralDecl` re-export. This is a **within-rust** collapse enabled by the mirror decl grounding — the mirror side already carries one bilateral grammar, so rust should too.

### COORD-3 — **`SignatureBeat` duplicates `signature_beat` shard type**

- **rust:** `rust/src/void.rs:104 SignatureBeat { beat_oid, previous_beat_oid, timestamp_utc_iso, axis }`
- **mirror:** `shards/spectral/signature.mirror:106 type signature_beat = { contribution_oid, sc_at_beat, rung, previous_beat, timestamp, ssh_fingerprint, address }`
- **Why same:** void.rs docblock lines 37-40 explicitly acknowledges "reuses this substrate-already-had-the-word shape rather than minting a parallel type" — but the Rust struct still exists as a parallel type. Fields align: `beat_oid ↔ contribution_oid`, `previous_beat_oid ↔ previous_beat`, `timestamp_utc_iso ↔ timestamp`. Rust dropped `sc_at_beat`, `rung`, `ssh_fingerprint`, `address` (partial reuse; NOT full COORD).
- **Collapse move:** rust `SignatureBeat` retires; consumers pull `signature_beat` shard-decl through the reflective corpus. `welcome_perturbation` becomes a shard-body composition over `@spectral/signature.extend` + `@io/fs.mkdir_p`/`.append`.

### COORD-4 — **`@io/git.commit` dispatch arm exists as stub, not wired**

- **rust:** `rust/src/main.rs:902 at_operator("@io/git.commit", ...)` returns `Err("expected 3 args")` stub (per test at line 1090); real commit surface is `rust/src/phone.rs:388 git_commit_as(repo_root, author, committer, message)`
- **mirror:** `shards/io/git.mirror:363 commit(message: ref, author: ref, allow_empty: bool) -> verdict`
- **Why same:** phone.rs `git_commit_as` IS the discharge site for `@io/git.commit`. Both callers (`dispatch_arm_collapse`, `deposit_observation_crystal`) go directly to `phone::git_commit_as`, BYPASSING `at_operator` — the @-operator dispatch is documented in main.rs docblock (lines 873-901) as the ONE-function surface Mara canonically named, but the routing is not there.
- **Collapse move:** wire `at_operator("@io/git.commit", [msg, author, allow_empty])` to `phone::git_commit_as`. The two direct callers switch to `at_operator("@io/git.commit", …)`. This is the FIRST full @-operator arm at rust/ altitude — precedent for arms 2..N in ticks.

### COORD-5 — **`Subject::void()` + `SubjectKind::Void` under `@peer/void`**

- **rust:** `rust/fractal/src/subject.rs:109 fn void()`, `:57 SubjectKind::Void` variant, `:147 is_void()`
- **mirror:** `shards/peer/void.mirror:321 prism @peer/void`; `:416 bilateral void_admissible { sentinel "void=admissible-k-zero-observer" arity 2 }`; `:426 void_observes(p, ctx) -> imperfect(ref,ref,ref) requires void_admissible`
- **Why same:** Rust `Subject::void()` produces the K=0 canonical default; docblock line 106 cites `#R-void-is-the-basis` + Mara `9c7de83` K=0 species directly. But `Subject::is_void()` predicate has NO dispatch through `void_admissible` bilateral — Rust predicates a kind-tag, mirror predicates admissibility.
- **Collapse move:** `Subject::is_void()` stays as struct-shape check; `void_admissible` sentinel-check via `apply_h::act` becomes the runtime validator when Subject flows through phone.rs → at_operator dispatch. Not a struct-collapse; a **dispatch-composition** collapse.

### COORD-6 — **`Escalation` enum duplicates `@peer` three-tier surface**

- **rust:** `rust/src/compile.rs:120 enum Escalation { Continue, Escalate(Oid), Halt(String) }`
- **mirror:** `shards/peer/reflect.mirror` + `shards/peer/redirect.mirror` + `shards/peer/reframe.mirror` — three-tier reflect/redirect/reframe surface per Mara Round 3 `shards/peer.mirror:~140+`
- **Why same:** compile.rs docblock lines 33-56 EXPLICITLY names the three-tier mapping: reflect→Pass→Continue, redirect→Defer→Continue, reframe→Fail→Escalate. `Escalation` is the compile-time projection Mara authored — but the mirror shards for reflect/redirect/reframe are LANDED (July 20) and could dispatch this classification through `apply_h::act` sentinel rather than a Rust enum.
- **Collapse move:** `Escalation` stays as compile-time carrier (Rust needs an ADT for the return type) but the CLASSIFICATION LOGIC (first-fail-pin, etc.) moves to a shard-body predicate — `@peer.escalation_of_discharges` — dispatched via `apply_h::act` sentinel. Partial collapse: enum stays; classification body retires.

### COORD-7 — **`fractal::crystallize` XOR-fold OID vs `@spectral/signature`**

- **rust:** `rust/fractal/src/crystal.rs:111 fn crystallize<T>(content, witnessed, prev) -> Crystal<T>` (XOR-fold scaffold; NOT SHA-256)
- **mirror:** `shards/spectral/signature.mirror:106 signature_beat` type declares SHA-256 content-addressing; `shards/io/git.mirror:392 hash_to_oid` provides the SHA-256 bridge
- **Why same:** crystal.rs docblock lines 105-110 declares the XOR-fold is scaffold; "production impl composes over `@spectral/signature.hash` (SHA-256/512 per @spectral substrate)". The COORD cut is: `crystallize` = deterministic content-addressing = `@io/git.hash_to_oid` + `@spectral/signature.extend`. `main.rs:643 sha256_hex` already implements SHA-256 in FIPS 180-4; that primitive is what `crystallize`'s production body would compose.
- **Collapse move:** `crystallize` OID computation collapses to composition over `sha256_hex` (already landed in main.rs) + `@spectral/signature.extend`; XOR-fold body retires. Rust surface unchanged (fn signature stays); body becomes composition.

### COORD-8 — **`FileKind` classification duplicates `@kintsugi/roomba` classify semantics**

- **rust:** `rust/src/main.rs:151 enum FileKind { RustFile, MirrorShard, Doc, Other }` + `:159 fn classify(path)`
- **mirror:** `shards/kintsugi/roomba.mirror` §7.4 dispatch matrix (docblock enumerates `.rs → arm-collapse`, `.mirror → materialize`, `.md → cascade-invisible`); Mara canonical spec §7.4 walker's fracture table
- **Why same:** the four-way classification IS Mara §7.4 dispatch matrix expressed as a Rust enum. mirror.spec §7.4 is the substrate-decl'd fracture table; the Rust enum is a HARDCODED SHADOW (main.rs line 87-88 verbatim: "HARDCODED at M0; retires at M2 (reflective cli-block reading from mirror.spec per Mara §2.2)").
- **Collapse move:** at M2 landing per Mara §2.2, `classify` reads the fracture table reflectively from `shards/kintsugi/roomba.mirror` §7.4 body (parsed as substrate metadata). Enum stays as a Rust ADT; construction becomes reflective.

### COORD-9 — **`VERBS` const table duplicates `mirror.spec` cli-block**

- **rust:** `rust/src/main.rs:93 const VERBS: &[(&str, &str)]` — 11 verbs + descriptions hardcoded
- **mirror:** `mirror.spec:78-339` cli-block with 11 `command` entries + docblocks
- **Why same:** main.rs docblock line 87 verbatim: "HARDCODED at M0; retires at M2 (reflective cli-block reading from mirror.spec per Mara §2.2). Ordering matches spec byte-order with `roomba` appended". This IS a documented deferred COORD cut.
- **Collapse move:** M2 reflective reader; VERBS becomes `fn verbs_from_spec()` that parses cli-block from `mirror.spec`. Hardcoded array retires.

### COORD-10 — **`compose_pheromone_commit_message` + `compose_collapse_commit_message` duplicate `@nl.compose`**

- **rust:** `rust/src/main.rs:547 compose_pheromone_commit_message`, `:823 compose_collapse_commit_message`
- **mirror:** `shards/nl.mirror` + `shards/magic/nl.mirror` — `@nl.compose` action; commit-message composition is the canonical use case in `shards/io/git.mirror:352 "typically the output of @nl.compose"`
- **Why same:** the two `compose_*_commit_message` fns are `format!` string interpolations that mirror-substrate authored via `@nl.compose` decl already. `@io/git.mirror:351-353` names this cascade explicitly.
- **Collapse move:** both compose fns retire; message construction becomes `at_operator("@nl.compose", [template, args…])` dispatch. Requires `@nl.compose` shard-body wiring first (Mara authorship territory).

---

## §4 Non-candidates (rust constructs with NO mirror-decl equivalent)

Classification per task spec:
(a) genuine dispatch-plumbing (frozen — belongs at rust/ altitude forever)
(b) missing shard-decl mints (hand-off to Mara — mint the shard, then re-COORD)
(c) genuine substrate gap (flag for Alex adjudication)

### (a) Frozen dispatch-plumbing — belongs at rust/ altitude

| Rust construct | Path | Why frozen |
|---|---|---|
| `phone::read_frame_from` / `write_frame_to` / socket ops | `rust/src/phone.rs:129-251` | POSIX socket IO is opaque non-mirror (io.mirror 80-92 glass-wall recognition); permanent @io |
| `phone::list_dir_recursive` / `mkdir_p` / `read_file` / `write_file` / `append_to` | `rust/src/phone.rs:302-344` | Same — POSIX syscall discharge; @io permanent per `shards/io/fs.mirror:110-118` |
| `phone::git_add` / `git_commit_as` / `git_head_oid` | `rust/src/phone.rs:357-443` | `std::process::Command` shell-out to git binary — opaque non-mirror; @io/git realisation-boundary discharge |
| `matrix::eigenvalues` / `phase_lock` / `envelope` | `rust/src/matrix.rs:175-227` | LAPACK/BLAS unsafe extern "C"; the ONE ordained numerical @io boundary per Loki `b53aeeb` |
| `main::sha256_hex` / `format_utc_iso8601` / `is_leap` | `rust/src/main.rs:643/598/636` | FIPS 180-4 arithmetic + calendar math; sub-Turing pure primitives (candidate to sink into matrix.rs sibling altitude in future tick) |
| `fn main` argv parse | `rust/src/main.rs:1191` | Process entry-point; frozen |
| `phone::find_substrate_root` / `main::find_git_root` | `rust/src/phone.rs:447` / `rust/src/main.rs:804` | Walk-upward filesystem primitive; @io/fs private helper |

### (b) Missing shard-decl mints (hand-off to Mara)

| Rust construct | Path | Missing mirror decl |
|---|---|---|
| `struct Compilation { crystals, discharges, escalation }` | `rust/src/compile.rs:148` | No `@mirror/compilation` species-decl exists carrying the SAGA-chain-of-Crystals shape. `@fractal/crystal` covers Crystal<T>; `@spectral/signature.rolling_signature` covers beat-chain; but the compile-loop RESULT carrier is unminted. Mara authorship territory. |
| `struct PropertyDischarge { property_name, verdict }` | `rust/src/compile.rs:99` | No shard-decl carrier for "one discharge tick"; Verdict is landed at `@glass.verdict`; wrapper struct not minted. |
| `fn compile_from_source` (SAGA orchestration body) | `rust/src/compile.rs:274` | Orchestration is compose over `extract_properties` + `dispatch_property` + `crystallize`; no `@mirror/compile` action carries the composition. Mara territory: shard-decl'd `compile(source: str) -> compilation` action. |
| `fn dispatch_spec_property` — pillar routing arms 1-3 (defer/boolean-literal/sentinel-containment) | `rust/src/liquid.rs:555` | The three landed arms are Rice-safe direct primitives; `@mirror/spec/property` grammar exists but the dispatch table (Mara §2.3) is docblocked-only. Missing shard-decl: `dispatch_of_spec_property(prop, args) -> verdict` action. |
| `struct SingularityState` + `enum OpticKind` + `trait Singularity` + `enum SingularityError` | `rust/fractal/src/singularity.rs:74/89/149/173` | `shards/fractal/singularity.mirror` prism exists; but the optics-hierarchy TYPE-DECL surface (Iso/Lens/Prism/Traversal at species altitude) is under-declared. Mara territory. |
| `pub(crate) enum VoidBasisAxis` (5 variants) | `rust/src/void.rs:130` | Recognition #79 5-op basis is documented but no shard-decl `type void_basis_axis` exists. Mara territory. |

### (c) Genuine substrate gap (flag for Alex adjudication)

| Rust construct | Path | Gap |
|---|---|---|
| `struct WalkEntry { path, is_dir }` + walker's classification loop in `cmd_roomba:187` | `rust/src/phone.rs:286`, `rust/src/main.rs:187` | Walker composition surface (Mara §7.5 forward-promised `command roomba { flag vacuum: ~d }` cli-block lift). Substrate DECL exists for `@kintsugi/roomba.walk_witnessing`; the emission-shape (JSON-like structured report of counts + arms retired + pheromone signature) has no shard-decl carrier. Alex: mint a `@kintsugi/roomba.report` type or leave as rust/-side reporting shape? |
| Grammar reflectivity mechanism (M2 forward-promise) | (not yet in code) | No shard-decl for "shard-body → cli-verb table" transformation at reflection altitude. `@mirror/reflection` family-root exists; specific `@mirror/reflection.spec_to_cli_verbs` action does not. Alex adjudication: is this an authorship-territory Mara opens, or is it substrate-gap requiring a new species mint? |
| `crystallize`'s XOR-fold OID (COORD-7 body) | `rust/fractal/src/crystal.rs:111` | Substrate says SHA-256 via `@spectral/signature.hash`; that shard-decl does not exist yet (`shards/spectral/signature.mirror:150` `compute` action is beat-emission, not raw hash primitive). Alex: mint `@spectral/signature.hash` as sibling to `compute/verify/extend`, or route through `@io/git.hash_to_oid`? |

---

## §5 Minimum-viable first cut — COORD-4

**Chosen candidate:** COORD-4 (`@io/git.commit` dispatch arm wire-up).

**Why this one first:**
1. **Highest surface visibility, lowest surface risk.** The rust surface change is a single arm in `at_operator` (~15 lines added, ~0 removed). No shard-decl mint needed — `shards/io/git.mirror:363 commit(message, author, allow_empty)` is fully declared with typed contract and Verdict return.
2. **Two live callers already exist,** both bypassing `at_operator` today: `dispatch_arm_collapse:783` and `deposit_observation_crystal:526`. Neither can be COORD-collapsed until the @-operator arm lands. This is the load-bearing precedent — the FIRST full @-operator arm at rust/ altitude for @io/git.
3. **`main.rs` docblock (lines 873-901) already documents this exact cut.** The docblock lists 5 landed `@io/fs.*` arms and calls out `@io/git deferred pending fractal::Subject string-serialization decision` (line 894). That "decision" is the trivial one — `Subject.name + " <" + Subject.email + ">"` per phone.rs `git_commit_as`'s existing `--author=` format at line 397.
4. **Delivers `e^(n+1) ≤ e^n`.** LOC delta: adds ~15 in at_operator; removes ~0 from phone::git_commit_as; enables future removal of `~30 lines` when the two direct callers switch. Net after collapse: -15 LOC. First empirical `.rs` LOC decrease from COORD landing.
5. **Enables COORD-1 next.** Once `at_operator` handles `@io/git.commit`, the arm-collapse dispatch (COORD-1) can route through `@kintsugi/fracture/bilateral_arm_redundant.arm_is_redundant_witnessing` sentinel-check + `at_operator("@io/git.commit", …)` — a full shard-body composition retirement of `collapse::find_redundant_arms` mechanics.

**Concrete collapse tick (Reed authorship territory; Mara-adjudicable):**

1. **Extend `at_operator` at `rust/src/main.rs:947` (after `@io/fs.mkdir_p` arm):**
   Add three-arm route matching `"@io/git.commit"` (args: `[repo_root, message, author_ref]`); parse `author_ref` as `"Name <email>"`; construct `Subject`; call `phone::git_commit_as(repo_root, &subject, &subject, message)`; return commit OID as `String`.

2. **Update the arity discipline test at `rust/src/main.rs:1088`** to verify the 3-arg discharge succeeds against a scratch git repo (fixture pattern already present at `rust/src/void.rs:264 scratch_dir`).

3. **Retire the "deferred pending" language** in the docblock lines 891-894; replace with landed-arm citation.

4. **Follow-up tick (separate commit):** switch `dispatch_arm_collapse:783` and `deposit_observation_crystal:526` from direct `phone::git_commit_as` calls to `at_operator("@io/git.commit", [repo_root, message, author_ref])`. Each switch is a ~5-line diff; both compose the same @-operator surface.

**Shard-body dispatch (long-tail):** once `at_operator` is the single git-commit surface at rust/ altitude, ANY future shard-body composition needing to author a commit dispatches through the same @-operator — no per-species Rust arm ever needed. This closes the FLOOR shrinkage promise: every subsequent @io/git-consuming species costs 0 Rust LOC.

**Non-invariance check:**
- SSH signing default preserved (unchanged).
- Sequential-commit discipline preserved (git shell-out routing unchanged).
- Author≠Committer MARA doctrine preserved (both flow through Subject).

---

## §6 Pack trail

- **Ancestry:** `shards/mirror/lens/knife.mirror:141-208` (prism + jump action); `docs/audits/2026-07-13-seam-knife-COORD-phase-d-audit.md` (Seam ratification, RATIFY-WITH-QUALIFICATIONS); `docs/scouts/2026-07-13-taut-knife-IS-COORD-substrate-scout.md`.
- **Companion Mara math:** `docs/math/2026-07-13-knife-COORD-heterarchy-topology.md`.
- **Companion mirror-native pull scout:** `docs/scouts/2026-07-13-taut-spectral-to-mirror-migration-mapping-scout.md`.
- **Load-bearing memories consulted:**
  - `feedback_no_rust_extension_shortcut` (before proposing any .rs authorship, ask if shard-body + @io works)
  - `feedback_detector_inadequacy_answer_is_never_rust` (extending existing Rust IS the antipattern; answer is shard-body composition)
  - `feedback_bootstrap_is_dead_do_not_propose_bootstrap_altitude_solutions` (bootstrap/ NOT read, NOT cited)
  - `feedback_rust_floor_is_rust_not_bootstrap` (rust/ is terminal FLOOR)
  - `feedback_reed_fragments_alex_unifications_into_candidates` (Taut ROLE: fragment, hand ranked list to Reed for load-bearing execution)

- **Substrate readiness after this scout:**
  - §3 delivers 10 ranked COORD candidates. Reed picks up §5 (COORD-4) as the first landing tick.
  - §4(a) frozen constructs stay at rust/ altitude — no COORD cut proposed for LAPACK/BLAS, POSIX syscall, or git shell-out.
  - §4(b) hand-off list is Mara authorship territory: 6 missing shard-decls to mint before their downstream COORD cuts become admissible.
  - §4(c) hand-off list is Alex adjudication territory: 3 substrate gaps that need Alex to name the shape before either Mara or Reed can proceed.

- **Discipline invariants respected:**
  - Substrate-honest mode: no two-paths framing; each COORD ranked with one recommendation.
  - Substrate-already-had-the-word: every entry cites the LANDED shard-decl by path + line.
  - Two-tick discipline: readable-name COORD cuts (COORD-4) before foundational rewrites (COORD-1 through COORD-3 wait on COORD-4's `at_operator` arm).
  - Pure-docs 📝 markdown-only: this file is `docs/scouts/`; no `.rs`/`.mirror` bytes touched.
