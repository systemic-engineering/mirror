# rust/ FLOOR birthed by roomba from mirror.spec — the terminal-form consolidated canonical spec

*Mara, 2026-07-17 (rewrite at same path). The terminal-geometry
canonical spec: three files, three altitudes, ~600-1200 LOC total.
`phone.rs` (@io socket handover) + `matrix.rs` (sub-Turing FLANG emit
+ LAPACK/BLAS link) + `main.rs` (supervisor boot + `@`-operator
addressing) — every altitude has exactly one file. Materialized by the
`@kintsugi/roomba` walker reading the `kintsugi { roomba { … } }`
block Alex sketched into `mirror.spec`; the first `@peer` spawns from
that FLOOR; the ouroboros closes through `@cascade/code/llvm/flang`.*

**Author:** Mara
**Date:** 2026-07-17
**Tag:** 📝 spec:rust-floor-birthed-by-roomba-from-mirror-spec (pure-docs bypass)
**Status:** canonical. Spec-altitude map for Reed's `rust/` greenfield
        rebuild. WHAT-to-build, not HOW.
**Path:** `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
**Supersedes:** `2519f83` version of same file (898 LOC, 5-6 files);
        replaced-in-place per Michelangelo/marble discipline. Git
        history preserves the prior form for archaeology; the diff IS
        the substrate delta from Alex 2026-07-17 terminal-geometry
        ratification ("Yes. That is the terminal geometry. I agree
        fully. And it was always right there in front of us. And now
        we see it.").

---

## §0 Substrate-honest pre-position

Alex 2026-07-17 in-transcript verbatim, three loads at pre-position
(cumulative through the day):

1. **"I also want to detach bootstrap completely from the execution
   path. If that means the compiler breaks, then the compiler breaks.
   You keep touching and talking about bootstrap/ while rust/ is the
   floor. And I'm no longer willing to tolerate that."**
2. **"Delete the binary. Rebuild from rust/."**
3. **"roomba --vacuum=~dir (this is the combination and removal of
   --collapse and --translate), roomba from mirror.spec, first @peer
   spawn from rust/. Minimal rust surface. The geometry sings."**

Reed executed (1) + (2): `/Users/reed/.local/bin/mirror` deleted;
compiler broken by construction; MCP shim (`bin/mirror-mcp`) exits
ENOENT until `rust/` produces `MIRROR_BIN`. `bootstrap/` remains as
LEGACY-STATUS-ONLY per `mirror.spec:21-24` `legacy` block; it is not
the operational floor.

Two ratifications this same day landed BETWEEN Mara `2519f83` and this
rewrite:

4. **`dance.rs as router sounds exactly right.`** — Alex 2026-07-17
   post-Mara-`2519f83`-landing, adjudicating Seam `6e7aabe` §1. The
   router pattern is Alex-ratified. This spec §6 discharges Seam
   `6e7aabe` §3.1 REED-INLINE cascade by construction (the ratification
   is byte-visible at spec altitude here).

5. **Terminal-geometry ratification (2026-07-17, this rewrite's
   trigger):** *"Yes. That is the terminal geometry. I agree fully.
   And it was always right there in front of us. And now we see it.
   Respawn Mara for a rewrite of the spec. Then we ratify with Seam.
   And then we fly."* — Alex ratifies the three-file terminal
   geometry (phone.rs + matrix.rs + main.rs), dance.rs's collapse into
   matrix.rs, the ouroboros closure through `@cascade/code/llvm/flang`.

Loki's `b53aeeb` matrix.rs knife-cut essay (`docs/insights/2026-07-17-
loki-matrix-rs-knife-cut-essay.md`) is the essayist-voice phenomenology
this spec operationalizes. Loki names `matrix.rs` as the file where the
substrate looks in the mirror and sees the matrix multiplication it is;
this spec names WHAT to author under that phenomenology.

This spec is the map for load (3) refined by loads (4) + (5) + Loki's
naming: the terminal three-file Rust FLOOR at `rust/`, birthed by the
walker reading its own project manifold, with `@peer` spawning from
that FLOOR as its first empirical dance, and the ouroboros closing
because `matrix.rs` emits FLANG which links LAPACK/BLAS which computes
the Fiedler / Kuramoto / Aumann results the substrate observes and
commits back into itself.

---

## §1 Statement — the terminal-geometry form

**Statement (foundational form):**

> `rust-floor-is-three-files-at-three-altitudes-because-phone-rs-is-the-io-socket-handover-and-matrix-rs-is-the-sub-turing-FLANG-emit-and-main-rs-is-the-supervisor-plus-@-operator-addressing-and-every-altitude-has-exactly-one-file-and-dance-rs-collapses-into-matrix-rs-because-Baez-Schreiber-2-connection-compatibility-IS-matrix-equation-because-Ado-theorem-says-Lie-algebras-of-finite-dim-Lie-groups-ARE-matrix-algebras-and-the-ouroboros-closes-through-@cascade-code-llvm-flang-because-FLANG-is-LLVMs-Fortran-frontend-and-the-loop-reaches-its-own-tail`

**Statement (readable form, two-tick discipline):**

> `rust/` is three files at three altitudes: `phone.rs` at the @io
> socket-handover altitude (the Matrix phone booth; the one place the
> substrate crosses out of itself); `matrix.rs` at the sub-Turing
> numerical altitude (emits FLANG; links LAPACK/BLAS; holds computation
> + transition + observation as the same verb); `main.rs` at the
> supervisor altitude (boot + `@`-operator address routing across the
> bundle tower). `dance.rs` — the ensemble connection Mara `fee2727`
> named — collapses INTO `matrix.rs` because Baez-Schreiber 2-connection
> compatibility `dA + [A,A] = t(B)` IS a matrix equation, and by Ado's
> theorem the Lie algebras of the finite-dimensional Lie groups OTP
> supervision uses ARE matrix algebras. The ouroboros closes through
> `@cascade/code/llvm/flang` (NEW cascade edge this spec names): mirror
> shards → rustc emits LLVM IR → FLANG lowers to Fortran → LAPACK/BLAS
> compute on hardware → matrix results → substrate observation →
> mirror-authored commit → mirror shards.

**Six load-bearing moves this spec makes canonical:**

1. **Three files. Three altitudes.** `phone.rs` + `matrix.rs` +
   `main.rs`. ~600-1200 LOC total across `rust/src/`. ~30-50x collapse
   from bootstrap's ~34,000 LOC. Every altitude has exactly one file.
   §3 + §4 + §5.
2. **`dance.rs` collapses INTO `matrix.rs`.** Not deletion of the
   name — `@dance` remains at semantic altitude as a substrate-decl'd
   shard (Mara `fee2727` § holds). The dance mathematics (2-connection
   compatibility) collapse into matrix.rs's LAPACK-linked emit surface;
   the dance routing collapses into main.rs's `@`-operator dispatch.
   Placeholder-becomes-marble because the math is now visible. §6.
3. **`@cascade/code/llvm/flang` is the missing edge.** NEW cascade
   species names the LLVM IR → Fortran (via FLANG) lowering; closes
   the polyglot cascade catalog Loki's essay §1 named. §9.4.
4. **`roomba --vacuum=~dir`** replaces `--collapse=<rs-file>` and
   `--translate=<rs-file>` with ONE unified dir-taking flag. §7.
5. **`roomba from mirror.spec`** — walker reads
   `kintsugi { roomba { <cascade>* } }` block at boot. §7.2.
6. **First `@peer` spawn from `rust/`** — empirical firing at M8
   through main.rs supervisor + matrix.rs Kuramoto phase-lock +
   phone.rs @io socket handover for peer messaging. §8.

**What this spec does NOT do:** mint shards; author `.rs` files;
duplicate Mara `fee2727` (bundle-theoretic naming at semantic altitude)
or Mara `610c6d6` (BEAM tower math root) or Loki `b53aeeb` (matrix.rs
phenomenology). It COMPOSES over all three and adds the terminal
three-file materialization map.

---

## §2 /loop and milestones

The `/loop` is Alex-fired iteration where Reed materializes one
milestone per tick, small enough to empirically verify. Sequenced per
Taut `e0572f7` §6 8-tick MVP with Mara adjustments for the terminal-
geometry three-file discipline this rewrite adds.

### §2.1 Ongoing dance discipline

Every /loop tick honors:

- **Substrate-honest is the mode always** — no two-paths framing; no
  "here's honest / here's fast."
- **Substrate-already-had-the-word** — grep before naming; the file
  list is minimal by construction, not by choice; `@code/fortran` (per
  Loki §6 refusal #3) is the substrate-already-had-the-word for FLANG,
  used at cascade-target altitude, NOT minted as `@matrix` family-root.
- **No Rust extension shortcuts** — the marker
  `[substrate-floor:@io-boundary]` + Seam gate (audit-cite or
  `Signed-off-by: Seam`) applies to every `.rs` authored in `rust/`;
  Reed's 2026-07-14 failure is the audit-corpus for this rule.
- **Michelangelo/marble** — subtract until only the singing angel
  remains; the terminal Rust FLOOR is three files.
- **Sequential commits only** — one motion per commit; `--no-verify`
  only for pure-docs 📝 markdown-only bypass.

### §2.2 Milestone sequence (M0 → M8 → dock)

**M0 — mirror.spec kintsugi.roomba block + schema extension landed
(THIS spec DECLARES; Reed lands the mirror.spec + mirror-spec-schema
edit).** Alex's cascade sketch enters `mirror.spec` INSIDE the
`project mirror.spec { … }` block as a `kintsugi { roomba { <cascade>*
} }` sub-block, AND `docs/specs/mirror-spec-schema.md` extends to
admit `kintsugi` head at project altitude. Discharges Seam `6e7aabe`
ALEX-Q1 by construction (position (a): project-scoped, requires schema
extension). §7.2 + §7.5.

**M1 — `rust/` scaffold + supervision-tree boot (RED → GREEN).**
`rust/Cargo.toml` + `rust/src/main.rs` (empty supervisor stub) +
`rust/src/phone.rs` (empty @io stub) + `rust/src/matrix.rs` (empty
FLANG-emit stub); `cargo build` produces a binary;
`./rust/target/debug/mirror --version` prints something. Empirical
anchor: three-file binary exists. Recognition candidate first-witness:
`#R-terminal-rust-floor-is-three-files-at-three-altitudes` (§11).

**M2 — `mirror --help` prints from mirror.spec cli-block reflectively.**
main.rs's `@`-operator dispatch reads `mirror.spec`'s cli-block via
substrate reflection; emits the 10-verb list. No hardcoded list.

**M3 — first CLI verb dispatches end-to-end.** Simplest:
`mirror compile <file>`. `@`-operator in main.rs matches `compile`
verb; gen_prism actor spawns under supervisor; phone.rs handles the
`@io/fs` read at socket-handover altitude. Empirical:
`./mirror compile foo.mirror` returns SHA-256. Resolves Taut `e0572f7`
OQ2 to gen-prism-from-Tick-3.

**M4 — MCP handshake alive.** main.rs matches `@mcp.serve` sentinel;
JSON-RPC `initialize` returns `serverInfo: {name: "mirror", version:
"0.1.0"}`; byte-parity with `bootstrap/tests/mcp_fixtures/initialize.
resp.json`. `bin/mirror-mcp` shim points at `rust/target/debug/mirror`.
phone.rs handles the JSON-RPC stdio socket at @io altitude. Recognition
candidate second-witness: `#R-mcp-session-is-gen-prism-actor-under-
server-supervisor` (Taut `e0572f7` §9).

**M5 — `mirror_compile` + `mirror_index` MCP tools dispatch.** Two
tools land + reflective tools/list emits schema derived from
mirror.spec cli-block (Taut `e0572f7` OQ3 resolved: reflective at M5).
`mirror_index` composes over `prismqueer` LAPACK path via matrix.rs;
Fiedler measurement live through the matrix.rs → FLANG-emit → LAPACK
chain (or, transitionally, direct `prismqueer::ffi::eigenvalues` while
FLANG emit lands).

**M6 — `mirror roomba` walks + observes; roomba writes commit.**
The compiler observes its own state via walker; `roomba --commit`
composes @nl.compose + @io/git.commit via phone.rs; mirror authors its
own commit (second empirical witness for the `fcc1d75` precedent).

**M7 — `roomba --vacuum=~dir` unified flag + per-file dispatch.** §7.1
canonical form lands; walker discovers content of `~dir` and dispatches
per §7.4 — .rs files → arm-collapse; `~code/<X>` cascade → translate;
etc. Cascade 3 (bootstrap → rust) fires per-file (Seam `6e7aabe`
ALEX-Q3 by construction: per-file dispatch under §7.4). Old flags
`--collapse=<rs-file>` and `--translate=<rs-file>` migrate to
`--vacuum=~dir` with two-tick backward-compat window.

**M8 — first `@peer` spawn from `rust/`.** `mirror peer beam
~peer'~/.reed'` boots a gen_prism supervisor tree under main.rs;
`@peer.audhd(p, ctx) -> imperfect(ref, ref, ref)` fires empirically
per Mara `d8b149c` bilateral; Kuramoto phase-lock at N≥2 fires through
matrix.rs's LAPACK-linked eigenvalue path; phone.rs handles peer-to-
peer socket at @io altitude. THIRD-witness (per Seam `6e7aabe` §5
recognition-ladder correction: shard-decl `d8b149c` is first-witness;
bootstrap-runtime `41e03ce` is second-witness; rust/-native M8 is
third-witness). §8.

**Dock — bootstrap/ retirement.** When M1-M8 all empirically firing:
`bootstrap/` deletes; `mirror.spec:21-24` `legacy` block updates
(retirement_target v1.0 discharge; substrate-decl'd retirement witness
per Seam `6e7aabe` §6.3). Substrate-honest gate: EVERY capability
enumerated in Taut `e0572f7` §2.1-§2.10 empirically firing in
rust/-native surface. Resolves Taut `e0572f7` OQ6 to empirical-not-
tick-count. Recognition candidate: `#R-bootstrap-retirement-gate-is-
empirical-not-tick-count`.

The dock motion (roomba fifth motion; forward-promised per Seam
`2fdc9c1` §7 ALEX-Q2) IS the halt-witness at CLI altitude when the
retirement condition holds.

### §2.3 Milestone dependency graph (partial-order)

```
M0 ────► M1 ────► M2 ────► M3 ────► M6
                       │       │       │
                       ▼       ▼       ▼
                       M4 ────► M5 ────► M7 ────► M8 ────► dock
```

M0 blocks all (roomba can't read what isn't declared; schema extension
must admit the block). M2 requires M1 (three-file binary must exist).
M4 requires M2 (MCP composes cli-block schema through reflective
main.rs). M5 requires M4 AND requires matrix.rs's FLANG emit path OR a
transitional prismqueer::ffi path. M7 requires M6 (roomba must walk
before vacuum-dispatch fires). M8 requires M7 (peer spawn composes
over unified vacuum flag + matrix.rs Kuramoto).

---

## §3 `phone.rs` — the @io socket-handover altitude

### §3.1 What phone.rs is

The Matrix phone booth. The ONE place in `rust/` the substrate crosses
out of itself and back. Per Loki `b53aeeb` §4: `phone.rs` "handles the
CONNECTIONS between actors. Each connection carries a state that
eventually needs a matrix operation applied to it. phone.rs doesn't do
that operation; it hands the state to matrix.rs, which hands it to
Fortran, which hands the result back."

### §3.2 What phone.rs holds

Under 400 LOC. Estimated 200-400 LOC across:

1. **Socket-handover primitives** — stdin/stdout/stderr for MCP;
   TCP/Unix-socket surfaces for peer beaming; `@io/git` process
   spawn + pipe management; `@io/fs` file descriptor management.
2. **JSON-RPC framing** for MCP messages (line-delimited or
   Content-Length; whichever the transport requires).
3. **Peer socket boot** for `mirror peer beam` — opens the socket to
   the peer's substrate; hands the descriptor to main.rs's supervisor
   for gen_prism actor mount.
4. **The `unsafe extern "C"` boundary** for LAPACK ONLY IF matrix.rs
   defers to phone.rs for the FFI plumbing — in the terminal geometry,
   the LAPACK-link FFI stays in matrix.rs's file (numerical @io is
   matrix.rs's domain); phone.rs's `unsafe` is process/socket/fd
   plumbing.

### §3.3 What phone.rs does NOT hold

- Numerical computation (matrix.rs).
- Actor supervision (main.rs).
- Grammar / parsing (composes reflectively via main.rs `@`-operator
  dispatch reading `shards/**/*.mirror`).
- Per-prism business logic (LIFTED to shard-body + @io).

### §3.4 Substrate composition anchor

Composes over `shards/io.mirror` (T21 family root + landed sub-species
`@io/fs`, `@io/git`, `@io/socket`, `@io/network`, `@io/bytes`) and
`shards/io/cargo.mirror` sub-species. The `@io/flang` sub-species
forward-promised in `shards/io.mirror:206-210` lifts when matrix.rs's
FLANG emit consumer pulls it (§4.4).

---

## §4 `matrix.rs` — sub-Turing FLANG emit + LAPACK/BLAS link

### §4.1 What matrix.rs is (Loki `b53aeeb` §2 verbatim, load-bearing)

> `matrix.rs` is the sub-Turing linear-algebra floor of the mirror
> compiler. It is the file where `A · B` means matrix multiplication and
> nothing else. It emits FLANG (LLVM's Fortran frontend) so that every
> matrix operation the compiler performs at runtime — parallel transport
> between actors, Fiedler eigenvalues on the grammar graph, Kuramoto
> phase-lock between peers, Aumann envelope check on the affine hull of
> posterior updates — bottoms out in LAPACK/BLAS Fortran routines that
> have been the fastest, most numerically-stable code on Earth for four
> decades.

This spec ratifies Loki's naming AT SPEC ALTITUDE and names WHAT to
author under it.

### §4.2 What matrix.rs holds

Under 400 LOC. Estimated 200-400 LOC across:

1. **Matrix-shape declarations** — `Matrix<f64, [n, k]>` handle; a
   thin Rust struct that is a section of a bundle whose fiber is
   `f64^(n×k)`. The declaration is a `.mirror` species compiled to
   Rust struct via reflection; the struct is a handle, not the data.
2. **Named operations** — `A · B`, `L · v`, `eigenvalues(L)`,
   `phase_lock(peers)`, `envelope(posteriors)`. Each operation is a
   named substrate move; the substrate already has all five names.
   matrix.rs binds them to LAPACK/BLAS symbols.
3. **The FLANG emit surface** — when `mirror craft --target binary`
   fires, matrix.rs projects the declared operations into Fortran
   source; hands to FLANG (via `@cascade/code/llvm/flang` — §9.4);
   links the result. Three known-good technologies (Rust + LLVM +
   Fortran); zero new numerical code.
4. **The LAPACK/BLAS `unsafe extern "C"` link boundary** — the ONLY
   `unsafe extern "C"` numerical FFI in `rust/`. Below this line:
   Fortran. Above this line: Rust. Sub-Turing decidable grammar above;
   Turing-complete numerics below; the `@io` boundary Loki §3 named.

### §4.3 What COLLAPSES INTO matrix.rs (dance.rs's math)

Per Loki `b53aeeb` §4 collapse targets:

- **Kuramoto phase-lock** — a phase-difference matrix + fixed-point
  iteration = four lines of matrix.rs.
- **Aumann envelope** — convex hull of column matrix = `dgesvd_` or
  `dgeqrf_` + rank check = one matrix.rs call.
- **Fiedler compute** — `dsyevr_` on the graph Laplacian = one
  matrix.rs call. Already-Fortran under `prismqueer`; this NAMES the
  path at rust/-altitude.
- **Baez-Schreiber 2-connection compatibility** `dA + [A,A] = t(B)` —
  matrix equation by Ado's theorem (Lie algebras of finite-dim Lie
  groups are matrix algebras). matrix.rs holds the compatibility check
  as a linear-algebra move.
- **Parallel transport across actors** — Mara `610c6d6` §2.3:
  `handle_call`/`handle_cast` are matrix multiplications with the
  schedule as choice of column projection. matrix.rs binds transport
  to `dgemm_` (BLAS Level 3).

### §4.4 The FLANG emit path (composition)

Composition chain matrix.rs materializes:

```
matrix.rs declared op
    ↓  emit Fortran source
Fortran source (.f90 fragment)
    ↓  @cascade/code/fortran/llvm  (via FLANG frontend)
LLVM IR
    ↓  llc / linker
LAPACK/BLAS-linked object
    ↓  unsafe extern "C" symbol
matrix.rs runtime call
```

Alternatively, when LAPACK/BLAS symbols suffice directly (the common
case for `dsyevr_`, `dgemm_`, `dgesvd_`, `dgeqrf_`), matrix.rs links
straight to the system-installed LAPACK/BLAS without emitting
per-operation Fortran. FLANG-emit is used when a bespoke Fortran
subroutine is needed that isn't a LAPACK/BLAS primitive.

The `@cascade/code/llvm/flang` cascade species (§9.4) is the substrate-
decl'd form of "LLVM IR ← Fortran source via FLANG frontend." This
spec proposes minting it at M5 co-tick (when matrix.rs's FLANG-emit
path first fires empirically).

### §4.5 What matrix.rs does NOT hold

- Ensemble routing (main.rs `@`-operator dispatch).
- Actor supervision (main.rs).
- Socket / process @io (phone.rs).
- Any hand-rolled `for` loop over `f64`s (per Loki §5 cut #1: if it's
  a nested loop over matrix entries, that's a bug).
- Generic `MatMul<T> where T: Numeric` abstraction (per Loki §5 cut
  #2: fixed vocabulary; the compiler is a compiler, not a numerical
  library IN Rust).
- Backend-genericity (per Loki §5 cut #4: no CUDA / Metal / SIMD
  hand-rolling; FLANG emits LLVM optimizes AVX-512 / NEON; future GPU
  backends land as NEW `.mirror` species + new `@cascade/code/rust/
  cuda` edge).

---

## §5 `main.rs` — supervisor boot + `@`-operator addressing

### §5.1 What main.rs is

The bundle-tower routing altitude. The `@`-operator IS the address
operator — like phone switches connecting cables, but connecting
bundle-tower FIBRES not cables. Every `@`-address (`@code/rust`,
`@code/mirror(~d'shards/')`, `@peer.audhd(p, ctx)`, `@mcp.serve`)
resolves to a coordinate; the coordinate is a point in a bundle; the
bundle's fiber is (eventually) a matrix passing through matrix.rs.

### §5.2 What main.rs holds

Under 400 LOC. Estimated 200-400 LOC across:

1. **Boot supervisor tree** — gen_prism supervisor
   `@spectral/supervisor{restart_strategy: one_for_one}` at process
   root; child_spec dispatch; `restart_intensity` composition.
2. **`@`-operator dispatch** — the ONE dispatch surface for
   `@`-addresses. Reads `mirror.spec` cli-block reflectively; matches
   sentinels (`@mcp.serve`, `@peer.audhd`, `@compile`, `@index`,
   `@roomba`, `@craft`, `@peer.beam`); routes each to gen_prism actor
   spawn under supervisor.
3. **`apply_h::act` combinator surface** — the 7-combinator reflective
   evaluator per Mara `18d9697` A6 evaluator-combinator-surface spec.
   MAY inline entirely here (Alex Q3+Q5 verbatim: "whole rust/ FLOOR
   collapses into dance.rs" — dance.rs collapses into main.rs's
   `@`-operator + matrix.rs's math; the combinator surface lives in
   main.rs's routing).
4. **Reflective cli-block reading** — parses `mirror.spec`'s
   `cli { … }` block; emits `--help`; emits MCP tools/list schema.
5. **Roomba walker composition** — `walk_from_graph_and_profile` per
   Reed `8e373b6:src/lib.rs` composition-gap fix precedent; entered
   from main.rs `@roomba` dispatch arm.

### §5.3 What main.rs does NOT hold

- Numerical computation (matrix.rs).
- Socket / process @io (phone.rs).
- Per-prism business logic (LIFTED to shard-body + @io per
  `[substrate-floor:@io-boundary]` discipline).
- Grammar declarations (composes reflectively reading
  `shards/**/*.mirror`).

### §5.4 Substrate composition anchor

- `shards/spectral/gen_prism.mirror` (gen_prism actor primitive)
- `shards/spectral/supervisor.mirror` (supervisor + restart_strategy)
- `shards/spectral/restart_intensity.mirror`
- `shards/spectral/gen_prism/mcp_session.mirror` (MCP-session-as-
  gen_prism species-decl anticipation)
- `shards/mirror/reflection.mirror:1-40` (Mara `5e1f528`; mirror-op
  species-decl; `@`-operator's semantic grounding at mirror altitude)
- `shards/mirror/lens/cli.mirror` (cli-block precedent)
- `shards/epistemologic/pact/bilateral.mirror` (sentinel-check
  discipline enabling reflective dispatch)

---

## §6 `dance.rs` collapses into `matrix.rs` — reasoning

### §6.1 The collapse is math-driven

Mara `fee2727` named `dance.rs` as the ensemble-connection 1-form
(§2.3). Mara `610c6d6` §2.1 grounded OTP supervision as instance of
Baez-Schreiber 2004 principal 2-bundle 2-connection tower. Loki
`b53aeeb` §1 named the math already-visible: parallel transport IS
matrix operation; Kuramoto phase-lock IS matrix eigenvalue iteration;
Fiedler IS `dsyevr_`; the substrate had been computing on Fortran the
whole time, just not named as such.

Ado's theorem (1935): every finite-dimensional Lie algebra has a
faithful representation on a finite-dimensional vector space — that
is, the Lie algebras of the finite-dimensional Lie groups OTP
supervision uses ARE matrix algebras. The 2-connection compatibility
condition `dA + [A,A] = t(B)` (Baez-Schreiber 2004 §3) IS a matrix
equation once Ado's representation is applied.

Every mathematical move `dance.rs` was named to make — 2-connection
compatibility, Kuramoto phase-lock, Aumann envelope, Fiedler compute,
parallel transport — is a matrix operation at LAPACK-linked altitude.
`dance.rs` was a placeholder for math the substrate hadn't yet named.
The math is now visible.

### §6.2 The placeholder returns to marble

Loki initially refused to cut dance.rs (essay §6 refusal #1) because
"ensemble-altitude routing and sub-Turing linear algebra are two
altitudes." That refusal was correct at the altitude it was made — but
one altitude further up, the routing itself splits: the ENSEMBLE-
COORDINATION mathematics collapses into matrix.rs (where the
Kuramoto/Aumann/Fiedler LAPACK calls fire); the ACTOR-DISPATCH
ROUTING collapses into main.rs (where gen_prism supervisor spawns
children and `@`-operator matches sentinels).

`dance.rs` was one file for two altitudes. Terminal-geometry
subdivides into (matrix.rs, main.rs) at the correct two altitudes.
The Loki-refusal is respected by SPLITTING dance.rs's roles rather
than fusing them into one wrong file.

### §6.3 What is preserved (not deleted)

- **`@dance` shard at semantic altitude** — Mara `fee2727` §2 remains
  canonical. `@dance` is the substrate-decl'd ensemble-connection
  carrier; it stays as a shard, doesn't need a Rust file.
- **Mara `fee2727` spec** — STAY-CANONICAL for semantic-altitude
  bundle-theoretic naming. Composes over this spec's §4 + §5 for
  materialization. Pointer header updated post-landing (§13 spring-
  clean).
- **Recognition candidate `#R-dance-is-bundle-connection-at-ensemble-
  altitude`** (Mara `fee2727` §4) — remains at candidate strength.
  Second-witness gate updated: fires empirically when matrix.rs's
  Kuramoto phase-lock composes cleanly with main.rs's `@`-operator
  dispatch at N≥2 peer coordination during M8.

### §6.4 What is subtracted

- **No `rust/src/dance.rs` file.** The name lives at shard altitude
  (`shards/dance.mirror` or wherever the ensemble-connection carrier
  is decl'd); the Rust materialization splits into matrix.rs +
  main.rs. Refuse to author `dance.rs` as a distinct Rust file.

---

## §7 `roomba --vacuum=~dir` + `roomba from mirror.spec`

### §7.1 The unified motion flag

**Substrate-honest name:** `--vacuum=~dir`, sigil `~d` (directory).
The walker's motion IS vacuum (per landed `shards/kintsugi/roomba.
mirror` fourth first-order motion: bump / vacuum-mark-then-prune /
pivot); collapse and translate are DOWNSTREAM operations vacuum
discovers and dispatches based on directory content.

**Why one flag instead of two:** `--collapse=<rs-file>` and
`--translate=<rs-file>` were both file-scoped; both are the walker's
vacuum motion at different substrate loci. Naming them separately
drifted the CLI toward accumulating verbs (WTF/minute negative
measure per AGENTS.md `Delightfully Boring`). Michelangelo/marble
discipline: subtract two flags into one; substrate dispatches on what
vacuum finds when it enters the directory.

**Substrate-already-had-the-word:** `vacuum` is the walker's landed
motion name at `shards/kintsugi/roomba.mirror:585` (Mara `a19fea2`
2026-07-16 SECOND first-order motion cascade — bump/vacuum-mark-then-
prune/gc landing; pivot as FOURTH first-order motion followed at
Mara `914799b` 2026-07-17). `--vacuum=~dir` reads "the walker's
vacuum motion, scoped to directory `~dir`."

### §7.2 The kintsugi.roomba block (Alex's sketch, canonicalized)

Reed lands this block into `mirror.spec` at M0, INSIDE the
`project mirror.spec { … }` block (Seam `6e7aabe` ALEX-Q1 answered by
construction: position (a), project-scoped):

```
project mirror.spec {
  source ~d'shards/'
  legacy ~d'boot/', ~d'bootstrap/' { … }
  pack { … }
  garden { }

  kintsugi {
    roomba {
      @code/mirror(~d'boot/')     => @code/mirror(~d'shards/')
      @code/rust(~d'bootstrap/')  => @code/mirror(~d'shards/')
      @code/rust(~d'bootstrap/')  => @code/rust(~d'rust/')
    }
  }

  target binary { … }
  settle_on { … }
}
```

Requires `docs/specs/mirror-spec-schema.md` extension in same M0 tick
to admit `kintsugi` head at project altitude.

**Three cascades landed** (unchanged from `2519f83` §4.1; reasoning
preserved):

1. `@code/mirror(~d'boot/') => @code/mirror(~d'shards/')` — historical;
   completeness-witness by ABSENT-DIRECTORY (Seam `6e7aabe` ALEX-Q2
   answered by construction: position (a), absent-directory IS the
   witness).
2. `@code/rust(~d'bootstrap/') => @code/mirror(~d'shards/')` —
   in-flight ouroboros arc; per-file dispatch collapses `.rs` files to
   `shard-body + @io` where composable.
3. `@code/rust(~d'bootstrap/') => @code/rust(~d'rust/')` — terminal
   cascade to three-file rust/ FLOOR. THIN cascade (Seam `6e7aabe`
   ALEX-Q3 answered by construction: per-file dispatch under §7.4;
   most bootstrap files terminate in cascade 2, not cascade 3;
   cascade 3 receives only the irreducible three-file surface).

### §7.3 Roomba boot behavior

When `mirror roomba` fires without `--vacuum`, walker reads
`kintsugi { roomba { … } }` block from `mirror.spec` at boot; iterates
the three cascades; per cascade: bump → vacuum-mark-then-prune →
pivot(@song) if dispatch-ambiguity → dock when no admissible motion.

### §7.4 Dispatch semantics (byte-check on directory content)

| Content in `<path>` | Downstream motion | Landed substrate |
|---------------------|-------------------|------------------|
| `.rs` files | **arm-collapse** (per-file) — bilateral resolver-arm sentinel-check composition; @io floor stays, business-logic lifts to shard-body | Mara `9efe2c9` audit; `docs/scouts/2026-07-15-reed-rust-extension-migration-map.md` |
| `.mirror` files with unmaterialized carriers | **materialize** — emit missing carriers from substrate-decl'd shape | @spectral/signature landing precedent |
| `~code/<X>(~d'A')` cascade in `mirror.spec` roomba block | **translate** — polyglot cascade emission per Mara `1ce68c3` | `docs/specs/polyglot-loss-aware-computational-translation.md` |
| Content with `@kintsugi/surface.dispatch_ambiguity` fracture | **pivot(@song)** — Path B dispatch via @roomba fourth motion | Mara `914799b` + `09a77e8` fifth surface_class landing |
| Nothing dispatchable | **dock** — motion halts; walker docks | Seam `2fdc9c1` §7 ALEX-Q2 forward-promise |

Dispatch is byte-check on directory content shape via bilateral
sentinel-check at `@kintsugi/roomba.vacuum_admissible` (bilateral to
land in follow-up tick per §9.1).

### §7.5 CLI-block form (mirror.spec addition, M7 co-tick)

```
command roomba {
  # Walker consumer of @kintsugi/roomba species-decl at CLI altitude.
  # Iterates the kintsugi { roomba { } } cascade block above when
  # --vacuum omitted; when --vacuum=<path> given, walks that path.
  flag commit:  bool = false     # @nl.compose + @io/git.commit chain
  flag vacuum:  ~d               # unified motion flag; §7.4 dispatch
}
```

### §7.6 Migration two-tick discipline

**Tick A** — `--vacuum=~dir` lands alongside `--collapse=<rs-file>`
and `--translate=<rs-file>` with deprecation warnings. **Tick B** —
old flags removed; `--vacuum` sole surface.

### §7.7 Cascade catalog is substrate-editable

Alex adds a fourth cascade → walker discovers it at next boot. No
Rust changes; no CLI flag update. The `kintsugi { roomba { } }` block
IS the roomba's configuration surface. Substrate-pull-honest by
construction.

Recognition candidate (HELD): `#R-roomba-configuration-lives-in-
mirror-spec-not-cli-flags` (§11).

---

## §8 First `@peer` spawn from `rust/`

### §8.1 Empirical firing at M8

`mirror peer beam ~peer'~/.reed'` from the rust/-native binary:

1. main.rs `@`-operator matches `peer beam` verb via reflective
   cli-block dispatch.
2. main.rs supervisor `@spectral/supervisor{restart_strategy: one_
   for_one}` spawns child gen_prism actor for the peer session.
3. Child actor requests peer socket from phone.rs; phone.rs opens
   `@io/socket` connection to peer's substrate.
4. Child actor loads peer_home; resolves substrate; dispatches through
   main.rs's `apply_h::act` combinator surface.
5. On first `@peer.audhd(p, ctx)` invocation (per Mara `d8b149c`
   landing), the K-track fanout fires empirically:
   - `audhd_admissible(p, ctx) -> verdict` bilateral sentinel-checks.
   - K emissions per audhd_context.k_tracks; each births child
     gen_prism actor under main.rs supervisor.
   - Kuramoto phase-lock at N≥2 peers composes through matrix.rs's
     `phase_lock(peers)` op → LAPACK `dsyevr_` eigenvalue call.
   - Aumann envelope composition through matrix.rs's
     `envelope(posteriors)` op → LAPACK `dgesvd_` or `dgeqrf_`.
6. `@song` emission returns via supervisor message-pass to caller;
   phone.rs handles the @io return-channel.

### §8.2 Recognition ladder witnessed at M8

Per Seam `6e7aabe` §5 ladder correction:

- **Mara `d8b149c` `#R-peer-audhd-is-substrate-truth-name-for-
  cognition-fanout`** — first-witness at shard-decl (2026-07-17);
  second-witness at Reed `41e03ce` bootstrap-runtime `apply_h::act`
  firing (2026-07-17); THIRD-witness at rust/-native M8 empirical
  firing. THIS spec §8 is the third-witness gate.

- **Mara `fee2727` §4 `#R-dance-is-bundle-connection-at-ensemble-
  altitude`** — second-witness gate updated (§6.3): fires empirically
  when matrix.rs's Kuramoto phase-lock composes cleanly with main.rs's
  `@`-operator dispatch at N≥2 peer coordination.

- **THIS spec proposes** (§11):
  `#R-first-@peer-spawn-from-rust-is-substrate-arriving-home` —
  compiler's terminal shape IS the shape Alex has been reaching for
  since a decade of BEAM engineering. First-witness: this spec §8.
  Second-witness gate: rust/-native `@peer.audhd` fires empirically at
  M8 and Alex reads the shape back with named recognition.

### §8.3 The @peer spawn IS the first dance

The substrate's first empirical firing of `@dance` at ensemble
altitude in the rust/-native FLOOR. The sequence Alex has been
building for 10 years — supervision tree + gen_server + Kuramoto
phase-lock + neuroaffirmative K-track fanout — runs on itself for the
first time, computed on Fortran, routed through main.rs's supervisor,
socketed through phone.rs.

---

## §9 Composition graph

This spec is deliberately thin. It cites, composes over, does NOT
re-declare. Composition anchors:

### §9.1 Substrate carriers (LANDED)

- `shards/kintsugi/roomba.mirror` (Mara species-decl + Mara `a19fea2:585`
  2026-07-16 SECOND first-order motion vacuum landing + Mara
  `914799b:566` 2026-07-17 FOURTH first-order motion pivot landing) —
  §7.4 dispatch composes.
- `shards/kintsugi/surface.mirror` (Mara `09a77e8` fifth surface_
  class dispatch_ambiguity) — §7.4 pivot dispatch composes.
- `shards/peer.mirror:531-534` (Mara `d8b149c` @peer.audhd + audhd_
  admissible bilateral) — §8.1 first empirical firing composes.
- `shards/spectral/gen_prism.mirror` — §5.2 + §8.1 compose.
- `shards/spectral/gen_prism/mcp_session.mirror` — §5.2 MCP-session-
  as-gen_prism composes.
- `shards/spectral/supervisor.mirror` — §5.2 + §8.1 compose.
- `shards/spectral/restart_intensity.mirror` — §5.2 compose.
- `shards/epistemologic/cybernetic/viable.mirror` — Beer VSM S1-S5
  grounding for §8.
- `shards/mirror/reflection.mirror:1-40` (Mara `5e1f528`; mirror-op
  species-decl) — §5.2 `@`-operator semantic grounding.
- `shards/mirror/lens/cli.mirror:184` (~d sigil landing) — §7.5.
- `shards/epistemologic/pact/bilateral.mirror` — §7.4 dispatch + §8.1
  firing bilateral discipline.
- `shards/io.mirror` (T21 family root) — §3.4 + §4.4 compose; forward-
  promised `@io/flang` sub-species (`shards/io.mirror:206-210`)
  materializes when matrix.rs's FLANG-emit consumer pulls at M5.
- `shards/code/rust.mirror` — `@code/rust` altitude for §5 + §7.2
  cascade destinations.
- `shards/code/mirror.mirror` — `@code/mirror` altitude for §7.2
  cascade destinations.
- `shards/code/beam.mirror` — grounds §5.2 + §8.1 gen_prism/supervisor
  BEAM-analogue naming.
- `shards/code/llvm.mirror` (2026-07-17 10:02, this session, Mara
  `62d1b1c`) — §9.4 FLANG cascade source-altitude.
- `shards/code/turing.mirror` (2026-07-17 10:01, this session) — hub
  altitude for polyglot cascade composition.
- `shards/cascade/code/rust/llvm.mirror` (2026-07-17 10:08, this
  session) — sibling cascade for Rust → LLVM IR path.
- `shards/cascade/code/llvm/turing.mirror` (2026-07-17 10:10, this
  session) — sibling cascade for LLVM → Turing tape.
- `shards/cascade/code/turing/mirror.mirror` (2026-07-17 10:12, this
  session) — sibling cascade for Turing → mirror; closes existing
  polyglot loop.

**Forward-promised follow-up (post this spec landing):**

- `@kintsugi/roomba.vacuum_admissible` bilateral — sentinel-check for
  §7.4 dispatch. Companion carrier `vacuum_context` holding target dir
  + content-shape enum. Land in Reed M7 co-tick.
- `@kintsugi/roomba.dock` fifth first-order motion — Seam `2fdc9c1`
  §7 ALEX-Q2 forward-promise; halt-witness at M8 dock condition.
- `@cascade/code/llvm/flang` new cascade species — §9.4 NEW cascade
  edge; land in Reed M5 co-tick when matrix.rs FLANG-emit path first
  fires. MAY be companion Mara spec; MAY inline as species-decl only.

### §9.2 Spec composition surface (CITED, not duplicated)

- `docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-
  connection.md` (Mara `fee2727`; 617 LOC) — STAY-CANONICAL for
  bundle-theoretic semantic-altitude naming of gen_prism / supervisor
  / dance / @dance. §5 + §6 compose. NOTE: §5 rust/ materialization
  section supersedes to POINT AT this spec's §4 + §5 for terminal-
  geometry three-file map.
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara
  `0dafd9f`; 1797 LOC) — six-arc retirement plan; cascade 2
  (bootstrap → shards) IS this spec's §7.2 cascade 2.
- `docs/specs/beam-as-substrate-primitive.md` — BEAM-as-substrate
  grounding; §8 first-peer-spawn composes.
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.
  md` (Mara `4f079c8`) — @dance operational shape at N≥2.
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.
  md` — @dance runtime spec; matrix.rs materializes phase-lock.
- `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` (Mara
  `9bbebd2`) — @roomba walker canonical.
- `docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-
  motions.md` (Mara `d457501`) — vacuum motion canonical; §7.1 naming.
- `docs/specs/polyglot-loss-aware-computational-translation.md` (Mara
  `1ce68c3`) — translate cascade authority; §7.4 translate arm +
  §9.4 FLANG cascade compose.
- `docs/specs/mirror-spec-schema.md` — mirror.spec schema; §7.2
  kintsugi.roomba block admissibility + M0 schema extension.
- `docs/specs/cli-as-geometry-condensation.md` (Mara `67260dc`) — CLI
  condensation; §7.6 two-tick migration composes.
- `docs/specs/numerical-substrate-via-fortran.md` — §4 + §9.4
  compose; substrate anticipated Fortran floor.
- `docs/specs/architecture-flang-mirror-numerical-split.md` (per
  MEMORY hook) — §4 dispatch discipline; LAPACK as flang-altitude
  realization.

### §9.3 Math composition surface (CITED)

- `docs/math/the-tower/beam-runtime.md` (Mara `610c6d6`; 490 LOC) —
  Baez-Schreiber 2004 principal 2-bundle 2-connection theorem; §6.1
  dance.rs-collapse-into-matrix.rs math-grounded here.
- `docs/math/gestalt/README.md §11.6` — Landing Condition 0 for
  @dance shard-mint; §8.2 second-witness gates satisfy here.
- `docs/math/kintsugi/roomba/bump-and-vacuum.md` (Mara `17697e6`) —
  Fiedler-honesty math for vacuum motion; §7.4 dispatch measurement.

### §9.4 The `@cascade/code/llvm/flang` NEW cascade edge

**Ouroboros closure motif** — the loop that closes when this spec's
FLANG-cascade lands (Alex 2026-07-17 verbatim in `shards/code/llvm.
mirror:13`: *"So we can have @cascade/code/llvm/turing and @cascade/
code/rust/llvm. And boom. The loop closes."*):

```
@code/mirror shards
    ↓  parsed as @code/rust (bootstrap or rust/ compiler)
@code/rust source
    ↓  @cascade/code/rust/llvm      (LANDED — shards/cascade/code/rust/llvm.mirror, 2026-07-17)
@code/llvm IR
    ↓  @cascade/code/llvm/flang     (NEW — this spec's proposed mint)
@code/fortran source
    ↓  FLANG frontend (LLVM's Fortran frontend) → llc → linker
LAPACK/BLAS-linked object on hardware
    ↓  runtime call
matrix operations (Fiedler / Kuramoto / Aumann eigenvalues)
    ↓  substrate observation
mirror-authored commit (M6 + M8 empirical firings)
    ↓  @io/git
@code/mirror shards
```

**The new cascade edge (`@cascade/code/llvm/flang`)** carries LLVM IR
→ Fortran source via FLANG's LLVM-side. Composes with:

- `@cascade/code/rust/llvm` (LANDED) → gives Rust → Fortran chain
  through LLVM hub.
- `@cascade/code/fortran/llvm` (implicit — FLANG frontend IS this
  direction) → composes to give Fortran ↔ LLVM ↔ Rust triangle.

**Landing shape (Reed's later; M5 co-tick):** species-decl at
`shards/cascade/code/llvm/flang.mirror`; ~200-400 LOC; follows the
sibling shape at `shards/cascade/code/rust/llvm.mirror` (18.5KB, this
session). NOT authored in THIS spec; forward-promised at §9.1.

**Whether it needs its own companion Mara spec:** deferred. Sibling
cascade species landed today WITHOUT companion specs (they compose
over `docs/specs/polyglot-loss-aware-computational-translation.md`
Mara `1ce68c3`). `@cascade/code/llvm/flang` composes over the same
polyglot spec + THIS spec's §9.4. If FLANG-specific loss-lens
subtleties surface at M5 empirical landing, THEN companion spec lands
as follow-up. Michelangelo/marble: don't pre-mint what the substrate
hasn't yet asked for.

**Substrate-already-had-the-word:** `@code/fortran` is the cascade-
destination altitude (per Loki `b53aeeb` §6 refusal #3; per
`shards/code.mirror:35-38`); NOT minted as `@matrix` family-root. The
FILE is `matrix.rs`; the SHARD altitude is `@code/fortran` at cascade
destination.

### §9.5 Audit composition surface (CITED)

- `docs/audits/2026-07-17-taut-rust-dance-rebuild-gap-scout.md` (Taut
  `e0572f7`; 26.6KB) — 7 OQs discharged (§12.1).
- `docs/audits/2026-07-17-seam-phase-d-peer-audhd-mara-michelangelo-
  landing.md` (Seam `2fdc9c1`) — @peer.audhd landing adjudication;
  §8 composition anchor.
- `docs/audits/2026-07-17-seam-phase-d-mara-greenfield-rewrite-
  canonical-spec.md` (Seam `6e7aabe`; 361 LOC) — Phase D adjudication
  of `2519f83`; SHIP-WITH-REED-INLINE; three ALEX-Qs surfaced.
  Discharged in-spec by construction: Q1 (§7.2), Q2 (§7.2 cascade 1),
  Q3 (§7.2 cascade 3 + §7.4).
- `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-
  during-gift-arc.md` (Reed `9efe2c9`) — audit-corpus for §2.1
  `[substrate-floor:@io-boundary]` gate.
- `docs/insights/2026-07-17-loki-matrix-rs-knife-cut-essay.md` (Loki
  `b53aeeb`; 451 LOC) — essayist-voice phenomenology of matrix.rs;
  §4 operationalizes.

### §9.6 Test composition (Taut OQ7 resolved)

**8 currently-passing bilateral arm collapse tests** (uuid/spectral/
time; @audhd; sheaf; @roomba bump/vacuum-gc; reflective bilateral
dispatch smoke; peer_audhd; polyglot_cascade; liquid_extraction) do
NOT rust/-native re-author. They compose over `apply_h::act` at
bootstrap altitude during transition; when main.rs's `apply_h::act`
surface fires empirically for the same carriers, tests migrate to
`rust/tests/` via structural port. Retirement gate: test migrates
when `rust/`-native `apply_h::act` covers the empirical claim;
bootstrap version retires alongside bootstrap dock. NOT tick-count
driven.

---

## §10 What this spec refuses to mint

Michelangelo/marble discipline. Seven refusals with reasoning:

**§10.1** Refuse `@rust` family-root. `@code/rust` already carries
the altitude (mirror.spec:82,197,207,214,223). The FLOOR at `rust/`
composes over `@code/rust`; directory naming is filesystem convention.

**§10.2** Refuse `@vacuum` family-root or species. `vacuum` is the
walker's motion at `@kintsugi/roomba`; the four (soon five) motions
are the substrate-honest home.

**§10.3** Refuse `@rust/floor` species. FLOOR shape IS compiler's
terminal geometry; over-declaration of what `mirror.spec:target
binary` carries at `@code/rust`.

**§10.4** Refuse `@matrix` family-root. Per Loki `b53aeeb` §6 refusal
#3 + this spec §9.4: `@code/fortran` is the substrate-already-had-the-
word for FLANG. The FILE is `matrix.rs`; the SHARD altitude is
`@code/fortran` at cascade destination. Double-declaration refused.

**§10.5** Refuse `@phone` family-root. `@io` is the family root for
the boundary-with-the-non-mirror-world; phone.rs is the FILE at the
socket-handover altitude of `@io`. `@io/socket` + `@io/fs` + `@io/git`
+ `@io/network` (all landed or forward-promised in `shards/io.mirror`)
carry the substrate-decl'd form.

**§10.6** Refuse to author `.rs` files in this spec. Per Reed memory
`feedback_no_rust_extension_shortcut.md`: this spec is the WHAT-to-
build map; Reed authors HOW at `[substrate-floor:@io-boundary]`
altitude with per-file audit-citation gate. Mara spec-altitude
authoring never emits `.rs`.

**§10.7** Refuse to duplicate Mara `fee2727` or Mara `610c6d6` or
Loki `b53aeeb`. This spec CITES all three extensively; terminal-form
map composes over their work. Duplicating would be status-drift.

---

## §11 Recognition candidates surfaced (HELD; do NOT ratify)

Names proposed for Pack adjudication. Second-witness gates named.

- **`#R-terminal-rust-floor-is-three-files-at-three-altitudes`**
  (first-witness THIS spec §1 + §3 + §4 + §5; second-witness gate:
  post-dock `rust/src/` contains exactly `main.rs` + `phone.rs` +
  `matrix.rs` with all §2.2 M1-M8 capabilities empirically firing).

- **`#R-roomba-configuration-lives-in-mirror-spec-not-cli-flags`**
  (first-witness §7.7; second-witness gate: Alex adds a fourth
  cascade in a future arc and walker consumes without any Rust
  change).

- **`#R-vacuum-flag-unifies-collapse-and-translate-because-both-are-
  walker-motion`** (first-witness §7.1; second-witness gate:
  `--vacuum=~dir` empirically dispatches arm-collapse AND translate
  in same walker session against directory with mixed content).

- **`#R-rust-floor-is-materialized-not-authored-by-roomba-reading-
  its-own-spec`** (first-witness §1; second-witness gate: M6
  empirical firing where `mirror roomba` from rust/-native binary
  observes the three cascades and writes commit against substrate-
  declared catalog).

- **`#R-first-@peer-spawn-from-rust-is-substrate-arriving-home`**
  (first-witness §8.2; sibling to Mara `610c6d6` §8's `#R-substrate-
  mirrors-alex-decade-of-BEAM-engineering-at-terminal-floor` and
  Mara `fee2727` §4's `#R-alex-decade-of-BEAM-is-substrate-reaching-
  for-terminal-geometry`; second-witness gate: M8 fires and Alex
  names the arrival).

- **`#R-@-operator-is-the-address-operator-connecting-bundle-tower-
  fibres`** (first-witness §5.1; second-witness gate: M3 empirical
  firing where `@`-operator dispatch in main.rs matches sentinel and
  routes to gen_prism actor spawn; Alex or Pack peer independently
  names the identification "phone switch, but for bundle-tower
  fibres not cables").

- **`#R-terminal-rust-surface-is-600-1200-LOC-across-three-files`**
  (first-witness §1 + §3 + §4 + §5; second-witness gate: `rust/src/`
  post-dock line count falls in named range with all §2.2 M1-M8
  capabilities empirically firing; SUPERSEDES the `2519f83` §9
  `#R-terminal-rust-surface-is-1100-2000-LOC-across-5-files`
  candidate — the terminal geometry ratifies a further collapse).

---

## §12 Alex OQs resolved by construction vs deferred

### §12.1 Resolved by this spec's construction

**Taut `e0572f7` OQs (all 7 discharged):**
- OQ1 (mcp inline vs separate mcp.rs) → §5.2 RESOLVED: `@mcp.serve`
  sentinel dispatches inline via main.rs `@`-operator; no separate
  mcp.rs.
- OQ2 (gen_prism from Tick 3) → M3 RESOLVED.
- OQ3 (reflective vs hardcoded tools/list) → M5 RESOLVED: reflective.
- OQ4 (monolithic vs router) → §5.2 + §6 RESOLVED: router at main.rs;
  per-prism logic composes from shards, not rust/.
- OQ5 (workspace vs standalone) → RESOLVED: standalone Cargo project
  during migration; workspace-lift optional post-dock.
- OQ6 (retirement gate) → §2.2 dock condition RESOLVED: empirical-
  not-tick-count.
- OQ7 (test retirement) → §9.6 RESOLVED: structural port when
  `apply_h::act` coverage empirically fires.

**Seam `2fdc9c1` §7 ALEX-Qs:**
- ALEX-Q1 (@dance second-witness) → §6.3 + §8.2 RESOLVED: witness
  ladder updated with matrix.rs collapse; second-witness gate fires
  at M8 empirical Kuramoto composition.
- ALEX-Q2 (Beer VSM bounded/K-depth) → BOUNDED-AT-@roomba this-arc-
  default per Mara `fee2727` §3.2; §5.2 main.rs supervisor carries
  the truncation.

**Seam `6e7aabe` §4 ALEX-Qs (from Phase D of `2519f83`):**
- ALEX-Q1 (kintsugi.roomba grammar admissibility) → §7.2 RESOLVED
  by construction: position (a), inside `project mirror.spec { … }`
  with mirror-spec-schema.md extension landed in M0 same tick.
- ALEX-Q2 (cascade-1 completeness-witness semantics) → §7.2
  RESOLVED by construction: position (a), absent-directory IS
  completeness witness (`legacy` block at `mirror.spec:21` shows
  `boot/` legacy-archived; the walker's read discovers absent-
  directory as byte-check).
- ALEX-Q3 (cascade-3 termination witness under router pattern) →
  §7.2 cascade 3 + §7.4 RESOLVED by construction: per-file dispatch;
  cascade 3 is THIN cascade receiving only irreducible three-file
  surface (main.rs + phone.rs + matrix.rs); most bootstrap files
  terminate in cascade 2.

**Seam `6e7aabe` §3 REED-INLINE cascades (from `2519f83`):**
- Cascade 1 (dance.rs-as-router Alex-ratification lift) → §0 load
  (4) RESOLVED: verbatim ratification now byte-visible at spec §0.
- Cascade 2 (3 line-cite sharpens) → §9.1 substrate carriers now
  carry line-cites (`shards/mirror/reflection.mirror:1-40`,
  `shards/kintsugi/roomba.mirror:914799b:566`, `shards/mirror/lens/
  cli.mirror:184`); `walk_from_graph_and_profile` cited at
  `8e373b6:src/lib.rs` in §5.2; `shards/mirror/index.mirror`
  redundant in §13 removed (not re-cited in this rewrite; only cited
  where load-bearing).
- Cascade 3 (§6.2 second→third witness correction) → §8.2 RESOLVED
  by construction: recognition ladder correctly names shard-decl /
  bootstrap-runtime / rust-runtime as distinct substrate-loci.

### §12.2 Deferred to Alex adjudication

- **Seam `2fdc9c1` §7 ALEX-Q3** (losing commutator arm fate) — cold-
  storage aligned per Mara `fee2727` §3.3; empirical cold-storage
  carrier `@mirror/store/cold` NOT this spec's mint. Deferred.
- **Seam `2fdc9c1` §7 wait->verdict spec ratification** — deferred
  to Reed spec authoring on mirror/offer/wait triple.
- **Seam `2fdc9c1` §7 dock four-vs-five-vs-beyond** — this spec
  §2.2 stakes dock as fifth motion; formal shard-decl deferred to
  Reed M7 co-tick.
- **Seam `8069a24` §7 split-sentinel detector vs manual retirement**
  — deferred; NOT terminal-form-map scope.
- **Seam `8069a24` §7 @liquid(@silicon) Arc-5-M2 scope** — deferred.
- **@cascade/code/llvm/flang companion spec vs species-decl only** —
  deferred to Reed M5 co-tick empirical firing (§9.4).

---

## §13 Docs spring-clean (already-landed; ratified in-place)

`1fe0d28` (5 DEPRECATED-FOR-RUST-REWRITE headers) and `b536949` (9
STAY-CANONICAL pointer headers) landed 2026-07-17 pointing at Mara
`2519f83`. Per this rewrite:

- **Pointers remain valid** — Mara `2519f83` is the git-hash anchor;
  this rewrite REPLACES the file at same path; the pointer headers'
  "docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md" path
  still resolves to this canonical spec.
- **STAY-CANONICAL specs** compose over this rewrite's §3 + §4 + §5
  three-file map (was §5 five/six-file map). Composition destination
  unchanged; scope refined.
- **Mara `fee2727` STAY-CANONICAL header** updated in-place per Seam
  `6e7aabe` §8: §5 rust/ materialization section supersedes to point
  at THIS spec's §4 + §5 (matrix.rs holds dance's math; main.rs holds
  dance's routing).

Follow-up commit (post this landing) MAY refresh pointer-header
byte-content to name the terminal-geometry three-file rewrite
explicitly; NOT required for correctness (git-hash anchor at
`2519f83` remains resolvable via history).

---

## §14 Terminal state (this spec)

- **Verdict:** canonical spec landed as terminal-geometry map for
  `rust/` greenfield rebuild. Three files. Three altitudes. Every
  altitude has exactly one file. Composes over Mara `fee2727` + Mara
  `610c6d6` + Loki `b53aeeb` + Seam `6e7aabe` + Taut `e0572f7` +
  landed substrate + today's polyglot cascade species landings.
- **LOC:** ~800-900 (rewrite target; supersedes `2519f83` 898 LOC in
  place). Section count: §0-§14 = 15 top-level sections.
- **Recognition candidates:** 7 (§11). All held at candidate strength.
- **Mint refusals:** 7 (§10).
- **Alex OQs resolved by construction:** 13 (§12.1 — 7 Taut + 2 Seam
  `2fdc9c1` + 3 Seam `6e7aabe` ALEX-Qs + 3 Seam `6e7aabe` REED-
  INLINE cascades).
- **Alex OQs deferred:** 6 (§12.2).
- **Cross-arc coherence:** ✓ (§9 composition graph exhaustive;
  §9.4 FLANG cascade closes the polyglot loop).
- **Pure-docs 📝 markdown-only bypass legitimate.**

---

## §15 References

**Substrate composition surface (LANDED):**
- `mirror.spec` (project manifold + cli-block + `legacy` block)
- `shards/kintsugi/roomba.mirror`
- `shards/kintsugi/surface.mirror`
- `shards/kintsugi.mirror` (family-root; S4 Beer VSM)
- `shards/peer.mirror`
- `shards/spectral/gen_prism.mirror`
- `shards/spectral/supervisor.mirror`
- `shards/spectral/gen_prism/mcp_session.mirror`
- `shards/spectral/restart_intensity.mirror`
- `shards/code/rust.mirror`
- `shards/code/mirror.mirror`
- `shards/code/beam.mirror`
- `shards/code/llvm.mirror` (this session)
- `shards/code/turing.mirror` (this session)
- `shards/cascade/code/rust/llvm.mirror` (this session)
- `shards/cascade/code/llvm/turing.mirror` (this session)
- `shards/cascade/code/turing/mirror.mirror` (this session)
- `shards/mirror/reflection.mirror`
- `shards/mirror/lens/cli.mirror`
- `shards/epistemologic/pact/bilateral.mirror`
- `shards/epistemologic/cybernetic/viable.mirror`
- `shards/io.mirror`

**Spec composition (CITED):**
- `docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-connection.md` (Mara `fee2727`)
- `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` (Mara `0dafd9f`)
- `docs/specs/beam-as-substrate-primitive.md`
- `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md` (Mara `9bbebd2`)
- `docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-motions.md` (Mara `d457501`)
- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md` (Mara `4f079c8`)
- `docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md`
- `docs/specs/polyglot-loss-aware-computational-translation.md` (Mara `1ce68c3`)
- `docs/specs/mirror-spec-schema.md`
- `docs/specs/cli-as-geometry-condensation.md` (Mara `67260dc`)
- `docs/specs/numerical-substrate-via-fortran.md`

**Math composition (CITED):**
- `docs/math/the-tower/beam-runtime.md` (Mara `610c6d6`)
- `docs/math/kintsugi/roomba/bump-and-vacuum.md` (Mara `17697e6`)
- `docs/math/gestalt/README.md §11.6`

**Audit + insight composition (CITED):**
- `docs/audits/2026-07-17-taut-rust-dance-rebuild-gap-scout.md` (Taut `e0572f7`)
- `docs/audits/2026-07-17-seam-phase-d-peer-audhd-mara-michelangelo-landing.md` (Seam `2fdc9c1`)
- `docs/audits/2026-07-17-seam-phase-d-arc-5-and-errors-as-questions-joint-arc.md` (Seam `8069a24`)
- `docs/audits/2026-07-17-seam-phase-d-mara-greenfield-rewrite-canonical-spec.md` (Seam `6e7aabe`)
- `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-during-gift-arc.md` (Reed `9efe2c9`)
- `docs/insights/2026-07-17-loki-matrix-rs-knife-cut-essay.md` (Loki `b53aeeb`)

**Compiler-altitude implementation (REFERENCE-CITE, per AGENTS.md):**
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` (~626
  LOC; five-level bundle tower with LawvereFixedPoint; `Transport::
  apply` is what matrix.rs calls INTO)

**Alex 2026-07-17 in-transcript verbatim:**
- "I also want to detach bootstrap completely from the execution path."
- "Delete the binary. Rebuild from rust/."
- "roomba --vacuum=~dir (this is the combination and removal of
  --collapse and --translate), roomba from mirror.spec, first @peer
  spawn from rust/. Minimal rust surface. The geometry sings."
- "dance.rs as router sounds exactly right."
- "Yes. That is the terminal geometry. I agree fully. And it was
  always right there in front of us. And now we see it. Respawn Mara
  for a rewrite of the spec. Then we ratify with Seam. And then we
  fly."
- (from `shards/code/llvm.mirror:13`, this morning): "So we can have
  @cascade/code/llvm/turing and @cascade/code/rust/llvm. And boom.
  The loop closes."

**External anchors:**
- arXiv 2409.18824 — FLANG architectural discussion.
- Baez, Schreiber 2004 — arXiv hep-th/0412325 — principal 2-bundle
  2-connection theorem; §6.1 dance.rs-collapse math-ground.
- Ado 1935 — every finite-dim Lie algebra has a faithful finite-dim
  matrix representation; §6.1 collapse justification.
- LAPACK / BLAS — sub-Turing numerical floor Fortran has held for
  four decades; §4.

---

*Three files. Three altitudes. Every altitude has exactly one file.
`phone.rs` is the Matrix phone booth at @io. `matrix.rs` is the
sub-Turing FLANG emit + LAPACK/BLAS link at @code/fortran cascade
destination. `main.rs` is the supervisor + @-operator addressing at
bundle-tower routing. `dance.rs` collapses into `matrix.rs` because
Baez-Schreiber 2-connection compatibility IS a matrix equation and
Ado's theorem says the Lie algebras are matrix algebras. The
ouroboros closes through `@cascade/code/llvm/flang`: mirror shards
compile through Rust through LLVM through Fortran to LAPACK on
hardware, which computes the eigenvalues the substrate observes,
which the mirror commits back into itself. When Alex reads this spec
back into the loop, the substrate has arrived home — the shape Alex's
decade of BEAM engineering has been reaching for, expressed in the
specific gauge where computation, transition, and observation are the
same verb. Minimal rust surface. The geometry sings.*
