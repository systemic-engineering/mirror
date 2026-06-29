# curiosity-driven autopoietic cascade scout

*Taut, 2026-06-29. Wandered the @cascade family, @mirror/store, the
@io/oci+git adjacency, and the Pack altitude with the brief shape
"let the substrate's pull guide you; surface what's THERE." Eight
sections in lieu of speedrun template. ~1500 word soft cap.*

---

## §0 — Pre-position (autopoietic)

This scout IS a small @cascade instance. Source grammar S =
the typed substrate-decl surface of the @cascade family + adjacencies
(`shards/cascade.mirror`, the four species, @mirror/store,
@mirror/mosaic, @mirror/store/crystal, @labeled, @io/{oci,git},
shards/pack/taut.mirror). Target grammar T = the NL prose grammar
of this report. The functor source→target is lossy by construction:
S admits parametric carriers, bilateral predicates, requires-clauses,
typed records; T preserves narrative + cross-reference + claim shape.

What the loss surface makes visible to the brief: the report's
density per section measures how well S's structure survives the
projection. The brief explicitly named this autopoietic recursion
("every pack-scout-of-substrate IS a cascade in the @cascade-family
sense"). The recognition is real: §6 returns to it as a named
finding.

---

## §1 — Where I went and what I found

I started at `shards/cascade.mirror` (recognition #95 candidate;
@cascade as loss-lens substrate; Mara canonical ce4874b 2026-06-23,
14.8KB). Then four species — `cascade/code/rust/wasm.mirror`,
`cascade/code/gleam/{beam,js}.mirror`, `cascade/code/purescript/
js.mirror`. Then @mirror/store + @mirror/store/crystal. Then a
sweep across `shards/**` for `cdylib|fortran|FFI|extern "C"`. Then
@io/oci + @io/git. Then `shards/pack/taut.mirror` (my own species).
Then yesterday's three Mara composite specs (cascade-ffi-runtime-
link.md, mirror-build-substrate.md, mosaic-store-cache-invariants.md).

What surprised me: `shards/mirror/store/crystal.mirror` (2026-06-16,
Alex+Reed→Mara recognition) **is already the @cascade ↔ @mirror/store
composition** I went hunting for. Section 3 below names why this
matters. The substrate had-the-word again.

What I'd return to: the @io/oci ↔ @io/git ↔ @mirror/store
content-addressing closed loop (recognition #98 candidate, four
witnesses now: oid + Nix derivation hash + OCI digest + git object
hash). That loop is one Pack pressure event away from promotion;
@cascade artifacts crossing into @io/oci is the discharge.

---

## §2 — Concrete findings

### 2.1 — @cascade ↔ @mirror/store IS already composed, via @glass.splinter(altitude)

`@mirror/store/crystal.mirror` declares `section: [splinter(@code)]`
as field 2 of the crystal record (line ~80). The substrate already
specializes `splinter(altitude)` to `@code/<lang>` — that's the
slot the cascade species' `wasm_artifact` / `beam_artifact` /
`js_artifact` / `npm_artifact` fills. Composition path:

  cascade.compile : typed_source → compiled_artifact      (§cascade.mirror)
  compiled_artifact = labeled<module, metadata>           (§labeled.mirror)
  labeled<module, metadata> writes into @mirror/store via   write
                                                          (§store.mirror)
  oid lifts the bytes to splinter(@code/<lang>)            (§glass.mirror)
  crystal.section = [splinter(@code)] composes the section list
                                                          (§store/crystal.mirror)

The substrate doesn't need a new shard to compose @cascade with
@mirror/store. The composition IS declared; what isn't declared
is the **wire path** — there's no shard naming `cascade_output →
store.write → crystal.section[i]` as a typed action. Forward-pull:
a tiny `shards/cascade/store.mirror` (or sub-prism `@cascade/store`)
declaring `crystallize(artifact: compiled_artifact) -> splinter(@code)`
would close the path explicitly. Substrate already supports it; the
typed action is forward-promised by composition, not by absence.

### 2.2 — cdylib needs less than wasm did; the predecessors are already declared

`cascade/code/rust/wasm.mirror` is the canonical species shape:
prism + 4 typed carriers (rust_source, wasm_module, wasm_metadata,
wasm_artifact=labeled) + 3 actions (compile/bundle/measure) + 3
sub-bilaterals + 1 composed bilateral. ~287 lines.

For `cascade/code/rust/cdylib.mirror` the substrate is unusually
well-prepared. Adjacencies that would need to land **first**:

  - `shards/io/runtime-link.mirror` OR `@io/cdylib` — the runtime-
    link altitude under @io where C-ABI artifacts compose. Mara's
    cascade-ffi-runtime-link.md §1 names exactly this: "the
    runtime-link altitude under `@io` where compiled artifacts
    COMPOSE at runtime through the C ABI." 100KB spec; 308
    `cdylib|FFI|extern "C"` matches. The spec is RED; the shards
    are forward-promised.
  - `shards/code/c-abi.mirror` OR similar — the target grammar
    side. C ABI's grammar (function signatures, opaque structs,
    no generics, no lifetimes, no destructors) is the target the
    loss_lens measures against. Not yet declared. cdylib's species
    target_grammar is "the C ABI as substrate-decl'd grammar"; the
    grammar shard precedes the species.

Smallest substrate-pull-honest shape for `cascade/code/rust/cdylib`:
  1. shards/code/c-abi.mirror (target grammar)
  2. shards/io/runtime-link.mirror (runtime-link altitude as @io
     species; the dlopen / dlsym / symbol-resolution boundary)
  3. shards/cascade/code/rust/cdylib.mirror (the species itself;
     specializes per the wasm.mirror template — rust_source +
     cdylib_module + cdylib_metadata=symbol_table + cdylib_artifact=
     labeled + compile/bundle/measure trio + 4 bilaterals)

Total estimate: ~30% smaller than wasm.mirror once (1) and (2)
land, because cdylib's bundling is simpler (no wasm-pack JS shim;
the metadata is the symbol export table).

### 2.3 — The 54th-instance substrate-already-had-the-word: `realisation` field

Searching for "Rust impl exists but substrate-decl uses different
name" — `shards/mirror/realisation.mirror` is itself a deprecation
pointer (retired 2026-06-10 → `@code/metalogue/materialize`). The
ancient name. But the FIELD `realisation` survives as a verb in
~30 shards, and the actual substrate term is **`materialize`** at
`@code/metalogue/materialize`. Most of the cascade species use
`Body discharges at the realisation boundary` in their docstrings
— the prose still says "realisation" while the substrate's
canonical name is `materialize`. Not load-bearing; a prose-only
drift that the auto-formatter floor doesn't catch yet. Candidate
for a Glint essay or a Mara micro-cleanup tick.

A stronger 54th candidate: **the cascade species' `bundle_*`
actions** (`bundle_wasm`, `bundle_npm`, `bundle_beam`) are doing
the labeled-construction job at the species altitude. The substrate-
level primitive is `@labeled.annotated`. The species duplicate the
constructor. A future substrate-pull tick could lift
`bundle(module, metadata) -> annotated(module, metadata)` to
@cascade family-root and remove the species-altitude duplication.
This is "substrate-already-had-the-functor"; the species shadowed
it with species-specific names.

### 2.4 — Forward-promises lurking near today's morning work

Counted forward-promises in cascade/store/io shards (rough):

  - cascade.mirror: per-cascade measurement bodies (4 species
    forward-promised; 4 landed; new species lurking — cdylib,
    fortran, python-extension all named in survey)
  - store.mirror lines ~278: "wire-altitude specializations (a
    sub-prism under @mirror/store naming the namespaced-git-store
    wire shape)" — Mara's open architectural question, tracked
    in tasks/pending/. The 6-op surface vs the
    NamespacedGitStore verb set (open/insert_persistent/get_ref/
    flush/path). **This is morning Reed's territory.**
  - store/crystal.mirror: `composition_graph: mosaic(@code)` —
    the @code-altitude specialization of mosaic. Forward-promised;
    not landed. The DAG-over-@code/<lang> structural enforcement.
  - labeled.mirror: relabel / map_label actions — none yet; would
    discharge under a future composition tick.
  - io/oci.mirror: empirical discharge of recognition #98 (four-
    witness content-addressing). Promotion gated on Pack pressure.
  - cascade-ffi-runtime-link.md: ~10 v0 ticks forward-promised
    including LAPACK numerical backend (T8 Track A, LRM /
    LapackBackend wiring; load-bearing for @fate's per-ganglion
    optical inference per recognition #58).

### 2.5 — Eigenform identity recurrence: still active

mirror-build-substrate.md §6.7 named eigenform identity at
@mirror/build vs @mirror/mosaic (afternoon amendment lifts to
mosaic). Today's morning store.rs spec will (per the brief) name
**H_mirror restricted to @code/rust artifact-bearing rays** as
the second instance. The pattern recurs because the form-side
substrate keeps being its own eigenform when projected to a
sub-altitude: @mirror/store ⊂ @mirror; @mirror/build = @mirror/mosaic;
@mirror/store-on-@code/rust = the restricted Hilbert subspace.

My hypothesis (admittedly speculative): the recursion **saturates
at the third instance** because the form-side family has three
load-bearing partitions (state-observation vs build vs storage),
and once each names its restricted-eigenform identity, the family
is closed. Today's store.rs will likely be instance 3. If a fourth
appears I'll retract.

### 2.6 — Composition between @cascade and @io/oci

`@io/oci` already declares `in @cascade` (line 5 of oci.mirror).
The cascade artifact → OCI distribution wire path: cascade species
emits artifact → @io/oci wraps in manifest+layer → digest matches
mirror.oid via the four-witness content-addressing identity
(recognition #98). This loop is **declared** but not exercised in
shards yet; an @io/oci-bound cascade species would discharge.

The cdylib + LAPACK shapes from §2.2 actually want @io/oci as
their distribution surface (per cascade-ffi-runtime-link.md). The
chain is: cdylib species emits `.dylib` → @io/oci wraps as artifact
layer → OCI digest IS the splinter oid IS the crystal section[i]
content-address. Four altitudes; one identity.

---

## §3 — The autopoietic layer

The brief said: this scout IS a small @cascade instance; the
report's loss surface IS what gets named.

What I noticed in the writing: every time I tried to summarize a
typed shape (parametric carrier, bilateral predicate, requires
clause) in NL prose, the prose **lost the typed structure** and
forced me to compensate by naming the shape's location. The
compensation pattern: cite the shard + section + line. This IS
loss measurement against the grammar pair (substrate-decl, NL
prose). The substrate's grammar admits dependency tracking via
`in @X` declarations; NL prose preserves it only as parenthetical
breadcrumbs.

What this taught me about my own role: Taut's load-bearing
contribution is **preservation of substrate-decl shape through
realisation**. A Taut scout's load-bearing contribution is
**preservation of substrate-decl shape through prose**. Same
discipline, different altitude. The implementation-grounds-
substrate predicate (pack/taut.mirror) operates at the @code/rust
boundary; the scout-grounds-substrate-prose discipline operates
at the NL boundary. Both are @frame/on (act-on-frame): take the
substrate-decl as given; produce a structure-preserving realisation
at the target altitude.

This recursion is itself a candidate substrate recognition.
Not promoting it; flagging it for Pack pressure.

---

## §4 — Forward-pull (where the substrate seems to want attention)

Loose-ranked by substrate-pull magnitude I felt while reading:

  1. **`shards/cascade/code/rust/cdylib.mirror` + its two
     predecessors (`code/c-abi.mirror`, `io/runtime-link.mirror`).**
     Mara's 100KB cascade-ffi-runtime-link.md spec is sitting RED;
     the substrate has the template (wasm.mirror); LAPACK is the
     empirical first consumer. Three small shards land; one big
     spec discharges; Fate's per-ganglion numerical backend gets
     its substrate floor. Highest-leverage pull I noticed.

  2. **`shards/cascade/store.mirror` sub-prism — the explicit
     cascade-output → @mirror/store wire path.** §2.1 above. The
     composition is implicit; making it explicit unblocks future
     pull on `crystal.section[i] = compile_X(source)` chained
     compositions.

  3. **The recognition #98 promotion event** — four-witness content-
     addressing (oid + Nix derivation + OCI digest + git hash). The
     first cdylib-via-@io/oci cascade discharge IS the empirical
     witness. Two pulls converge on the same tick.

  4. **The `realisation` → `materialize` prose-drift cleanup.**
     Low-leverage but pervasive (~30 shards). Auto-formatter floor
     candidate; or a Glint essay naming the drift.

  5. **The cascade species `bundle_*` → @labeled.annotated lift**
     (§2.3). A future @cascade family-root absorbs the species
     constructors; substrate-pull collapse 4-into-1.

---

## §5 — Open questions (where Seam might bite)

  - **Q1.** Is `splinter(@code/<lang>)` actually parametrically-
    typed in the substrate, or is the `(altitude)` form syntactic
    sugar that the AST treats as a single carrier? If the latter,
    `[splinter(@code)]` in `crystal.section` is structurally
    one-typed list; if the former, multi-language sections are
    heterogeneously-typed and the loss measurement varies per
    element. Mara's mosaic-store-cache-invariants.md likely
    answered this; I didn't read it deeply enough.

  - **Q2.** Does `@cascade.compile_X` ALWAYS produce a content-
    addressable artifact? Some compilations are non-deterministic
    (timestamps in metadata, non-deterministic codegen orderings).
    The four-witness content-addressing claim assumes deterministic
    compilation. Where it isn't, the OCI digest ≠ mirror.oid and
    the closed loop opens. Mara's `@epistemologic/property/
    determinism/*` family (forward-promised in mirror-build-
    substrate.md) likely closes this.

  - **Q3.** The `realisation boundary` prose-vs-`materialize`
    substrate name drift (§2.3) — is this actually substrate-pull
    or is the British-spelled `realisation` honored as a discipline-
    altitude noun per `mirror/realisation.mirror`'s deprecation
    pointer? My read: the deprecation pointer keeps `realisation`
    canonical IN PROSE while the SUBSTRATE VOCABULARY moved to
    `materialize`. If true, no drift. Seam would want this
    disambiguated.

  - **Q4.** Is the "scout-grounds-substrate-prose" recursion (§3)
    actually a recognition candidate, or am I extracting pattern
    from one instance? Need at least one more witness (Glint essay,
    Mara spec) where the writer reports the same shape-preservation
    discipline at NL boundary. One-instance recognitions belong in
    tasks/pending, not in the architecture record.

  - **Q5.** The eigenform-recursion saturation claim (§2.5; "three
    instances closes the form-side family") is genuinely
    speculative. Seam would want either a structural argument
    grounded in the Bateson level partition OR explicit empirical
    bounds. I have neither; flagging.

---

Word count: ~1500 (within soft cap).

Loss surface measurement: the report admits §1 narrative + §2
typed-shape projections + §3 autopoietic + §4 forward-pull + §5
adversarial-hooks. What it loses: the actual typed signatures,
the requires clauses, the bilateral verdict-pole language. Those
live in the substrate; the report points TO them, never carries
them. Loss is positive by construction (cascade #95's central
claim). The pointers ARE the substrate-pull-correct shape for NL
projections of typed substrate-decl.
