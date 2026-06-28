# Seam adversarial review — four-spec composite (mirror-build-substrate amendment + cascade-ffi-runtime-link + mosaic-store-cache-invariants)

## §1 — Position

Adversarial review of today's four-spec composite tied to the
substrate-already-had-the-word recognition cascade #51–#54:

- `fa57161` — `docs/specs/mirror-build-substrate.md` amendment (Mara,
  afternoon): retargets §1 + §5 + §11 to acknowledge `@mirror/mosaic`
  IS the build system; preserves §§2, 3, 6, 7, 9, 10 from the morning
  9a5c53f composition (~2080 lines after amendment).
- `9a78865` — `docs/specs/cascade-ffi-runtime-link.md` (Mara, Q4
  discharge; 2260 lines).
- `adbdfef` — `docs/specs/mosaic-store-cache-invariants.md` (Mara, Q5
  discharge; 2016 lines).

Plus cross-checks against `shards/mirror/mosaic.mirror`,
`shards/mirror/store.mirror`, `shards/cascade.mirror`,
`shards/io/cargo.mirror`, `shards/io/stagefreight.mirror`,
`shards/io/oci.mirror`, `shards/code/metalogue.mirror`, the existing
landed `@cascade/code/*` species, the bootstrap `Justfile`.

Read-only fences honored: no substrate-decl introduced; no Rust
ships; no edits to the P4 blocker files. Doc-only audit per the
brief.

## §2 — Methodology

1. Read all four specs end-to-end. The amendment was read against
   the preserved morning sections to identify retargeting holes.
2. Grep `shards/**/*.mirror` for `runtime-link|cdylib|dylib|
   abi_surface|linker|link_kind` to verify Q4's "genuinely new
   substrate-decl" claim.
3. Grep `shards/**/*.mirror` for all known composed-bilateral
   instances and **count them by name** to test Q4's "seventh
   altitude lift" claim and the cdylib "eighth lift" claim.
4. Verify `shards/mirror/mosaic.mirror` line 167–168 against the
   amendment's quotation + the Q5 §5.1 quotation.
5. Verify `shards/mirror/mosaic.mirror`'s actual forward-promise
   count against the amendment's claim of "eight existing forward-
   promises in `@mirror/mosaic`."
6. Read the bootstrap `Justfile` to verify mirror-build-substrate
   §7.1's six-cargo-subcommand-chain claim and §7.2's "4× libgit2-sys"
   claim against today's pre-commit recipe.
7. Cross-check Q5's lifted-fingerprint algebra against cargo's
   actual fingerprint inputs as Q5 §3.1 paraphrases them.

Severity legend: **C** critical (blocker for discharge) / **S**
substantive (significant gap; not blocker) / **M** moderate (over-
claim or under-tightening) / **L** light (polish).

## §3 — Headline finding

**No C-class issues.** The amendment + the two Q-discharges land in
a substrate-pull-honest shape; the math is altitude-true; the
forward-promises follow the existing per-species precedent; the
composed-bilateral discipline composes against existing instances.
The retargeting is incomplete in places (see S-class), and several
counting claims are stale (see M-class), but nothing in the
composite blocks a Rust engineer from picking up the discharge plan
tomorrow.

Counts: **C=0, S=4, M=6, L=5.**

## §4 — Critical (C) findings

None.

## §5 — Substantive (S) findings

### S-1 — Composed-bilateral count is materially wrong across both Q-specs

**Severity: S** (architectural framing; not a blocker).

Q4 §3.4 calls `runtime_link_safe` the "Seventh altitude lift of the
composed-bilateral pattern." Q4 §4.1 calls
`rust_cdylib_cascade_well_formed` the "Eighth altitude lift."

Grep of `shards/**/*.mirror` reveals the pattern is significantly
further along than these claims:

| shard | named instance | self-claimed lift # |
|---|---|---|
| `shards/cascade.mirror` | `cascade_well_defined` | 5th |
| `shards/cascade/code/rust/wasm.mirror` | `rust_wasm_cascade_well_formed` | "Sixth altitude lift" |
| `shards/docs/tea.mirror` | `tea_well_formed` | "Eighth altitude lift" |
| `shards/nl.mirror` | `nl_measurement_well_formed` | "9th altitude lift" |
| `shards/docs/design.mirror` | `design_complete` | "ninth-altitude lift" |
| `shards/docs/tea/spectral-engineer-case-study.mirror` | (composite) | "TENTH altitude lift" |
| `shards/io/oci.mirror` | `oci_well_formed` | "~12th altitude lift" |
| `shards/peer.mirror` | forward-promised `peer_coherent` | "14th altitude lift when it lands" |
| `shards/mirror/spawn.mirror` | forward-promised `mirror_spawn_coherent` | "15th altitude lift when it lands" |

Plus `@cascade/code/gleam/beam`, `@cascade/code/gleam/js`,
`@cascade/code/purescript/js`, `@io/git.git_well_formed`,
`@ui.ui_instrument_coherent`, `@ui/field.field_coherent`,
`@smarts/shatter.shatter_round_trip`,
`@epistemologic/neutrosophic.three_axis_coherent`,
`@io/stagefreight.stagefreight_addressable`,
`@reflection.third_order_coherent`, `@docs.docs_complete` — none of
which fit the "seventh" framing.

The grep evidence: the OCI shard already counts at ~12; subsequent
forward-promises (peer, mirror_spawn) are explicitly numbered 14/15.
Q4's `runtime_link_safe` would be approximately the **13th–14th**
landed lift, and the cdylib bilateral would be 14th–15th, not 7th
and 8th.

This is exactly the same shape of substrate-roster drift Mara
acknowledges in mirror-build-substrate.md §11 (the 51st instance of
substrate-already-had-the-word): the count was carried forward from
an older context. The fix is to grep before numbering. The
amendment's §11 acknowledges substrate-roster drift in general but
this same shape of count-drift recurs in §3.4 and §4.1 of Q4.

**Why this matters at S, not L**: the counting claim is load-bearing
for the "the pattern is altitude-portable" rhetorical structure that
both specs use. The honest count (12+ landed + 2 forward-promised)
makes the discipline argument **stronger**, not weaker — but the
spec claims a weaker number. Tightening to "the 13th+ landed
instance" makes the substrate-pull observation more honest.

Sibling instance in mosaic-store-cache-invariants.md §1.3 negative
#1 (the 53rd+ instance of substrate-already-had-the-word) — note
this is the OUTER count (substrate-already-had-the-word), not the
INNER count (composed-bilateral lifts). Q5 keeps these distinct,
which is correct. Q4 conflates them in §3.4.

### S-2 — Amendment leaves §6.6, §6.7, §10 references to `@mirror/build` un-retargeted

**Severity: S** (cross-reference inconsistency; reader will trip).

The amendment explicitly acknowledges in §0:

> *Sections §0, §3, §4.3, §6.6, §6.7, §7.2, §9.1, §9.6, §10.1 preserve their morning-draft language naming `@mirror/build` as the orchestrator. [...] The retargeting was not threaded through every preserved sentence to keep the amendment surgical.*

This is honest, but §6.7 ("the eigenform") is **load-bearing**
mathematics. The text says:

> `@mirror/build` has (A_build, H_build, D_build) per §6.6 above.
> A_build = A_mirror's algebra-of-build-actions specialization.

If `@mirror/build` does not exist as a substrate-decl entity, the
spectral-triple identity in §6.7 has nothing on its LHS at the
specified altitude. The argument retargets to `@mirror/mosaic`
cleanly (mosaic's five-op prism IS the algebra at build altitude),
but the reader has to do the substitution mentally on every load-
bearing math claim. §10.1's bullet 1 ("First crystal in the cache")
references `@mirror/build` v0 (§9.1) which §9.1 still calls
"`@mirror/build`'s operational Rust crate."

**Suggested tightening**: a single §6.7.1 note immediately under
the §6.7 heading reading "Per §1 amendment: every `@mirror/build`
in §6.6 and §6.7 reads as `@mirror/mosaic`; the eigenform identity
holds at mosaic's altitude unchanged." Avoid the in-place rewrite
the amendment deliberately defers; provide one breadcrumb that
makes the substitution local instead of global.

The §10.1 and §10.4 references to `@mirror/build` similarly need
one breadcrumb each. Per the amendment's surgical-minimalism
discipline, these are LIGHT-leaning Substantive — but I'm filing
S because §6.7 is the spec's cited eigenform identity claim and
that should not depend on the reader's amendment-trail tracking.

### S-3 — Q5 §3.4 mtime-drop preserves OID correctness but **does not preserve cargo's incremental invalidation semantics** — and the spec does not say so

**Severity: S** (load-bearing operational claim under-specified).

Q5 §3.4 is correct that mtime drops are content-addressing-honest:
same bytes → same OID across machines, across checkouts, across
clones. The substrate's cache key is over bytes; cargo's mtime is a
performance hint.

What the spec **does not address**: cargo's INTERNAL incremental
algorithm uses fingerprints (including mtime) to decide WHETHER TO
RE-INVOKE rustc at all. When Q5 §1.3 negative #4 says "the substrate
doesn't REIMPLEMENT cargo's incremental compilation logic; it WRAPS
cargo's outputs," that wrapping happens AFTER cargo has decided
whether to compile. The cache-hit path in §6.2 (cache_exists →
cache_read → skip cargo) bypasses cargo entirely; the **cache-miss
path** still hands off to cargo, and cargo's mtime-keyed fingerprint
then decides whether to incrementally recompile or hit cargo's own
target/ cache.

The composition question: when the substrate's Splinter OID lookup
MISSES (cold cache; new machine; new clone), the substrate dispatches
cargo. Cargo's mtime fingerprint sees a fresh clone (every file is
freshly-checked-out; mtime is recent; cargo's own per-process cache
is empty). Result: cargo recompiles even though the substrate
SHOULD have produced the same bytes from CARGO_TARGET_DIR — but the
substrate hasn't WRITTEN to CARGO_TARGET_DIR; it's only addressed the
OUTPUT.

Two operationally-distinct scenarios this spec doesn't disambiguate:

1. **Substrate cache hit → substrate provides .rlib bytes** —
   substrate writes the bytes into CARGO_TARGET_DIR + writes
   `target/<profile>/.fingerprint/<unit>/` files so cargo SEES the
   unit as cached, OR the substrate intercepts before cargo runs.
   Which is it? The spec implies the second (§6.2 step 3
   short-circuits "no compilation needed") but doesn't say
   operationally how it tells cargo "we've got this one."
2. **Substrate cache miss → substrate invokes cargo** — and cargo's
   own fingerprint algorithm runs against whatever's in
   CARGO_TARGET_DIR. If the substrate doesn't pre-populate
   CARGO_TARGET_DIR with the (cached) deps, cargo sees them as
   missing and recompiles, blowing past the substrate cache.

The libgit2-sys discharge scenario (§6) implicitly assumes scenario
(1) — cargo never reaches its own fingerprint check because the
substrate has already short-circuited. But the spec doesn't say so;
a Rust engineer implementing this would have to make the call.

**Recommended tightening**: a §3.4.1 (or §6.2.1) note: "The
substrate's cache HIT path bypasses cargo's invocation entirely
(the OID lookup returns the artifact bytes from @mirror/store; the
substrate writes them into the consumer's expected location;
cargo's incremental algorithm never runs). The cache MISS path
invokes cargo; cargo's own fingerprint runs over its own target/;
the substrate captures cargo's output and content-addresses it
after settlement." This makes the cargo-substrate composition
explicit.

### S-4 — Q5 §6.3 "4× libgit2-sys" claim does not match today's Justfile

**Severity: S** (empirical claim does not survive grep).

Q5 §6.1 says:

> The pre-commit hook chain runs four cargo subcommands in sequence: `cargo check`, `cargo clippy`, `cargo test`, and (in some configurations) `cargo build --release`. Each subcommand triggers a fresh cargo dispatch; each dispatch sees `libgit2-sys` as an unbuilt translation unit and triggers its ~2-3 minute cold compile.

The mirror-build-substrate.md §7.1 SAME claim:

> The Justfile's `pre-commit` recipe (per `/Users/alexwolf/dev/projects/mirror/Justfile` lines 99-133) dispatches six cargo subcommands across the chain: `cargo check`, `cargo clippy`, `cargo test`, `cargo build --release`, `cargo audit`, `cargo fmt --check`.

The actual Justfile `pre-commit` recipe (lines 119–141, verified):

```bash
pre-commit:
    set -euo pipefail
    rust_closure=$(git diff --cached --name-only ...)
    if [ -z "$rust_closure" ]; then exit 0; fi
    just build
    {{MIRROR_BIN_RELEASE}} kintsugi mirror.spec | tee /tmp/...
    {{MIRROR_BIN_RELEASE}} kintsugi --out=@data/json mirror.spec > /tmp/...
    jq -e '.verdict != "failure"' ...
```

It dispatches TWO things: `just build` (one `cargo build --release`)
and `mirror kintsugi mirror.spec` (substrate-internal dispatch which
MAY invoke cargo internally, depending on what kintsugi does at the
@code/rust altitude).

The six-subcommand claim is the AMBITION for the pre-commit chain
(see the §43 recognition naming five chained cargo settlements:
fmt_check → check → clippy → test → audit), not the CURRENT shape.
The four-subcommand claim in Q5 §6.1 is closer but still does not
match what's in the Justfile today.

The blocker analysis is operationally correct in shape (libgit2-sys
recompiles redundantly when the dispatch chain expands to multiple
cargo subcommands) but the **specific multiplier is unverified**.
The actual current waste depends on what `mirror kintsugi
mirror.spec` invokes — if it dispatches a single cargo command, the
multiplier is 1× per pre-commit, plus 1× from `just build`, giving
2× not 4×.

**Recommended tightening**: name the chain at the altitude it
actually exists at today (the `mirror kintsugi mirror.spec` dispatch
chain), and frame the 4–6× claim as "what the chain expands to once
the per-target action dispatch from recognition #43 lands its
forward-promised dispatch shape." Today's measurable waste is
smaller; the substrate-decl unblock IS still load-bearing because
the chain WILL expand; the spec's framing should make the temporal
order honest.

Both specs (mirror-build-substrate §7 and Q5 §6) have the same
shape of issue. One amendment to Q5's §6 + one tightening of
mirror-build-substrate §7 closes it.

## §6 — Moderate (M) findings

### M-1 — Q4 §6.2 "loss composition algebra" hedge is legitimate but the conservative answer should be committed to in v0

**Severity: M** (substantive but the hedge is honest).

Q4 §6.2 hedges between tropical (max-plus) and subadditive
(commutative monoid) for `loss(G ∘ F)`. §8.2 keeps both open.

The hedge is honest, but `[[feedback-loss-from-epistemologic-
properties]]` (the canonical guidance per `MEMORY.md`) is unambiguous:
"loss is a composite of `@epistemologic/properties`, not Shannon, not
invented." A composite of properties under monoidal aggregation
(per Q5 §7.3, which Q4 should reference) is **subadditive by
construction** — pass is identity, partial(opacity_map) accumulates
opacity_maps under union, failure is absorbing.

The substrate's existing verdict monoid IS the answer. Tropical
(max-plus) is a different algebra applied to a different carrier
(real-valued loss numbers); the substrate's carrier is the verdict
surface, and the existing monoid is subadditive.

Q4 §6.2 could commit: "the substrate's loss composition is
subadditive under the verdict monoid declared at `@glass.verdict` +
the existing AND-composition in @epistemologic/property/cache_valid
(per Q5 §2.4)." The tropical-vs-subadditive hedge is then a
historical artifact of the morning briefing; the substrate-pull-
honest answer was always available.

Not a blocker because the hedge is structurally fine; it's a
substrate-pull dividend Mara could claim and didn't.

### M-2 — Q5 §3.7 build_script outputs claim is structurally honest but discharges a substantial unknown

**Severity: M** (load-bearing complexity glossed).

Q5 §3.7 declares build.rs outputs become splinter children at
`splinter(@code/rust/build_output/<file_name>)`. This is correct in
principle and substrate-pull-honest in framing.

What the spec elides: build.rs scripts are **arbitrary Rust
programs** that can read from `$OUT_DIR`, write to `$OUT_DIR`, read
env, read filesystem (which `[[cargo:rerun-if-changed]]` partially
declares), invoke subprocesses (gcc, bindgen, protoc), download
data (less common but legal), etc. Capturing build.rs outputs as
content-addressed children requires sandboxing build.rs (else the
build.rs's hidden inputs blow content-addressing) OR running build.rs
inside an effect-tracking harness that observes its filesystem +
network reach.

Mirror-build-substrate.md §2.3's `bazel build` example acknowledges
this: "The substrate ABSORBS Bazel's hermetic-action semantics as
one realisation of @epistemologic/property/determinism/det."
Subordinately: build.rs being a hermetic action is a **non-trivial
property** of the build.rs script (most are not; many read git rev,
embed timestamps, query system openssl, etc.).

Q5 §3.7 should at minimum cross-reference mirror-build-substrate.md
§2.2's `det`/`semidet`/`nondet` discipline and note that build.rs
outputs only become content-addressed children **for build.rs
scripts declared `det`**. Otherwise the cache invariant
`content_addressed_per_translation_unit` quietly fails for the
build.rs subtree.

### M-3 — Q4 §3.2 link_kind enum has a gap: WebAssembly host link not named

**Severity: M** (substrate-coverage gap; sibling species exists).

Q4 §3.2's `link_kind` enum:

```mirror
type link_kind =
  | dylib | staticlib | python_extension | fortran_bind_c | custom(text)
```

The existing `@cascade/code/rust/wasm` species (landed; canonical
per Mara's `wasm.mirror`) produces a WebAssembly module that links
into a host runtime (browser JS host; Wasmtime; Wasmer). This is
**runtime-link composition** — the WASM module's exports are
imported by the host runtime through the WebAssembly imports/exports
protocol. Q4 §3.1 footnote text says:

> The existing landed species (rust/wasm, gleam/beam, gleam/js,
> purescript/js) do NOT need [the abi_shape + runtime_link_kind
> fields] because their target runtimes (browser WASM, BEAM, ES
> module loader) ARE the consumer runtime — there's no second-party
> runtime-link altitude to compose against.

This is **partially incorrect**. The browser WASM case has TWO
runtime-link altitudes: WASM-to-host (JS↔WASM via the WebIDL
bindings or wasm-bindgen) and WASM-to-WASM-component (the
forward-promised WIT-based component-model linkage). Today's WASM
species elides this because its consumer is "the browser" as a single
host, but the upcoming component-model runtime-link IS exactly the
shape `@io/runtime-link` characterizes.

Q4's `link_kind` should include `wasm_host` (the JS-host imports
discipline) at minimum, and the comment on the existing species
should acknowledge that the runtime-link altitude IS load-bearing
even for the "single host" cases.

Not a blocker because the `custom(text)` variant accommodates it,
but the substrate-coverage of the enum is incomplete and the
sibling-species text is over-strong.

### M-4 — mirror-build-substrate.md §4.3.2 v1.0+ "substrate subsumes StageFreight" framing contradicts §1's "collaboration not absorption"

**Severity: M** (consistency under amendment).

The amendment closes §8.3 (StageFreight binding scope) as
"collaboration not absorption" per Alex's explicit framing. But §4.3.2
(preserved from the morning draft) says:

> Eventually, the `@mirror/build` orchestrator subsumes StageFreight's operational role: the substrate-declared pipelines dispatch directly through `@mirror/build`'s eigensheaf scheduler; the `@io/stagefreight` adapter exists ONLY for backward compatibility...

This is the absorption framing the §8.3 closure explicitly rejected.
Per the §0 amendment preamble, §4.3 is one of the preserved sections;
the retargeting was intentionally surgical. But "substrate subsumes
StageFreight" is **substantively different** from "mosaic dispatches
through StageFreight"; the difference is not just nomenclature.

Same shape of issue as S-2 but at the policy-claim altitude rather
than the math-claim altitude. Per the surgical-amendment discipline,
this is M not S, but a future pass should reconcile §4.3.2 with §8.3
explicitly — likely by deleting §4.3.2's "subsumes" framing or
demoting it to "the substrate's orchestrator routes through @io/
stagefreight at the @release altitude per `mosaic(@release)`."

### M-5 — Q4 §1.3 "five structural negatives" — negative #3 leans on prior art Mara may not have grepped

**Severity: M** (sociology-of-citation, not technical).

Q4 §1.3 negative #3 cites "Mara's cascade survey §2.9" as naming
Rust → cdylib explicitly. The survey doc path is given:
`docs/research/2026-06-23-typed-alternatives-cascade-survey.md`.
Verified the citation is real (Mara wrote the survey).

The substantive question: does the substrate's `cascade.mirror`
itself name the cdylib case as an instance to discharge? Verified
against `shards/cascade.mirror`: yes, line 36 explicitly names:

> `Rust          → cdylib (C ABI)   lifetimes, ownership, generics erased`

So Q4's instinct (FFI species follow the cascade pattern; cdylib is
already named as an example) IS grounded in the substrate's own
header. The §1.3 negative #3 framing is correct.

What's left ambiguous: cascade.mirror's mention is in the INSPIRATION
list (the "concrete instances surveyed" comment), not in the
substrate-decl forward-promise list. The grep is positive but the
citation should make this distinction (the substrate has the WORD
in its docstring; the substrate-decl forward-promises lie at
species-altitude shards, which is what this spec adds).

Not a blocker; tightening would be one sentence in §1.3 negative
#3.

### M-6 — Q5 §3.3 "homomorphism" claim is slightly too strong

**Severity: M** (mathematical precision).

Q5 §3.3:

> The lift is a HOMOMORPHISM of the content-addressing equivalence relation, not an embedding into a different relation.

The claim is that the lift preserves the equivalence relation
modulo collision probability. This is **conditionally** a
homomorphism: it preserves equivalence only when both hash functions
are collision-free **on the same input set**. SipHash's collision
probability is much higher than Blake3's (SipHash has only 64-bit
output by default; Blake3 has 256-bit). So there exist inputs
{i, i'} where:

- `cargo_siphash(i) = cargo_siphash(i')` (SipHash collision)
- `substrate_blake3(i) ≠ substrate_blake3(i')` (Blake3 distinguishes)

In which case the substrate's lift is **finer** than cargo's
fingerprint — it distinguishes inputs cargo conflates. This is
substrate-pull-honest (Blake3 is more correct than SipHash) but it
violates the "homomorphism preserves equivalence" claim in the
strict direction.

The substrate's lift is a refinement, not a homomorphism. The cache
correctness is unaffected (Blake3 ≠ Blake3 means recompile, which is
safe); but the math should say "refinement of cargo's equivalence
relation under the substrate's tighter hash" rather than
"homomorphism."

## §7 — Light (L) findings

### L-1 — Q4 §1.3 negative #4 / #5 phrasing
"NOT @io/stagefreight territory" + "NOT @io/oci territory" are both
load-bearing distinctions, but the prose conflates them under one
"five structural negatives" header. Worth a single sentence pulling
out that runtime-link is the THIRD shipping altitude (after wire-
protocol @io/stagefreight and container @io/oci), each at a
different scope — the altitude map would benefit from including
this triad explicitly.

### L-2 — mirror-build-substrate.md §8.4 and §8.5 closure prose
Each amended closure section refers the reader to a "sibling Mara
dispatch" with a forward-promised spec name. §8.4 says
`docs/specs/cross-language-ffi-seams.md`; §8.5 says
`docs/specs/per-translation-unit-cache-wiring.md`. The ACTUAL Q4 and
Q5 specs landed as `cascade-ffi-runtime-link.md` and
`mosaic-store-cache-invariants.md` respectively. Cross-refs in §8.4
and §8.5 are stale.

### L-3 — Q5 §1.4 altitude map shows eight properties; the §2.4 cache_valid composition lists eight
Count matches. ✓ Verified for L-class — keep this here as a
positive note: the property roster is internally consistent across
§1.4, §2.2, §2.4, and §9.2. Refreshing — the spec's count discipline
holds where it matters most.

### L-4 — Q4 §10.2 mirror's own libgit2 link citation
Q4 §10.2 cites mirror's Rust binary as linking against libgit2,
libsqlite3, libc, rustls/openssl as the bootstrap recursion. libsqlite3
is forward-promised, not currently linked; rustls/openssl is
"transitively" via the network @io species which themselves aren't
yet wired through the bootstrap binary. The recursion claim holds
for libgit2 + libc (the genuinely-load-bearing pair); the other two
should be marked as forward-promised in the citation.

### L-5 — Naming consistency
The Q4 spec consistently uses `runtime-link` (with hyphen) in prose
and `runtime_link` (with underscore) in mirror code blocks. This is
substrate-decl-correct (prose follows English convention; code follows
the substrate's identifier convention). Worth keeping consistent in
any future revisions; the convention is implicit but unstated.

## §8 — Spot-checks against the brief's nine targeted questions

The brief listed nine specific check-this items. Each addressed
above OR here:

1. **Cargo fingerprint → Splinter OID lift composes with cargo's
   incremental algebra?** S-3 above. The lift is hash-honest but
   the cargo-substrate composition mechanics are under-specified.

2. **Q4's `@io/runtime-link` is genuinely new?** ✓ Verified.
   Grep against `shards/**/*.mirror` for
   `runtime-link|runtime_link|dylib|abi_surface|linker|dlopen|
   link_kind` returns ZERO substrate-decl hits — only header
   comments in io.mirror mentioning "linker invariants" in passing.
   The species is genuinely new.

3. **The "seventh altitude-lift" claim accurate?** S-1 above. No —
   the pattern is at ~12+ landed + forward-promises at 14/15. Q4
   should number to ~13–14.

4. **mirror/mosaic shard has eight forward-promises in its
   docstring as claimed in §5.1?** Verified the eight items in §5.1
   against `shards/mirror/mosaic.mirror`. The amendment's
   enumeration is **derived** from the shard (lines 22–38, 70–87,
   132–148, 161–183) but expanded into eight distinct items by
   slicing related claims. Six of the eight map cleanly to specific
   line ranges; items #3 (Phase D parent prism) and #7 (per-altitude
   mosaic(A) shape) are real but characterized as "forward-promises"
   when the shard frames them as architectural notes. Borderline
   M-class; honesty in the substrate cites would tighten the count
   to six explicit forward-promises + two architectural notes.

5. **Eigensheaf-Laplacian parallelism actually named in mosaic
   shard?** ✓ Verified. `shards/mirror/mosaic.mirror` lines 165–168
   carry the verbatim text the spec quotes. Mara accurately quoted
   the substrate's own forward-promise.

6. **Q5's 4× libgit2-sys claim true?** S-4 above. The current
   Justfile does not exhibit a four-cargo-subcommand chain; the
   actual current waste depends on what `mirror kintsugi
   mirror.spec` dispatches internally.

7. **Q4 §6.2 loss composition algebra hedge legit?** M-1 above.
   The hedge is honest but the substrate has a committed answer
   (subadditive under the verdict monoid) Mara could claim.

8. **Four-spec composite forms a coherent discharge plan?** ✓
   Verified. The sequence a Rust engineer should follow tomorrow:
   1. Q5 §2.2 forward-promised @epistemologic/property predicates
      (substrate-decl shards land first; each is a small TDD pair).
   2. Q5 §3 lifted_fingerprint Rust implementation against
      @mirror/store (the bytes-to-OID lift).
   3. Q5 §6 empirical discharge: re-run yesterday's P4 commit
      attempt; verify cache hit.
   4. mirror-build-substrate §2 @epistemologic/property/determinism
      family substrate-decl.
   5. Q4 §3 @io/runtime-link substrate-decl + §4.1 cdylib species
      (the LAPACK case unblocks T8 Track A).
   6. mirror-build-substrate §5.2 per-language @io species roster
      expansion.

   This is consistent across all four specs; no ordering conflicts.

9. **Did Mara grep before writing these specs?** Q5 yes (the
   substrate-already-had-the-word recognition is explicitly the
   thesis of the spec). Q4 yes (negative #1 explicitly cites the
   morning briefing's `@ffi` as a mis-grep that the spec corrects).
   The amendment's §11 records the morning's failure-to-grep as a
   load-bearing recognition. **Mara grepped harder than the
   morning briefing did.** Reed's `[[reed-grep-before-briefing-
   mara]]` discipline is applied across all four specs by Mara
   herself.

   The seam-counted exception: S-1 above (the composed-bilateral
   count). Mara grepped for "is the family-root genuinely new"
   but did not grep for "how many lifts have actually landed."

10. **Anything else off?** M-2 through M-6 + L-1 through L-5
    captured the substantive remainder.

## §9 — Summary judgment

The four-spec composite is **substrate-pull-honest and discharge-
ready**. The amendment correctly retargets the morning composition;
the two Q-discharges follow the substrate's existing per-species
pattern; the math is altitude-true; the open questions (§8 in each
spec) are honest hedges, not load-bearing gaps.

The S-class findings are tightenings, not blockers. S-1 (composed-
bilateral count) and S-2 (un-retargeted §6.6/§6.7) are visibility
fixes; S-3 (cargo composition mechanics) and S-4 (Justfile claim)
ask for one note each.

Mara's recognition discipline shines through: the amendment IS the
substrate naming its own grammar's pull toward what was already
declared; the Q4 + Q5 discharges follow the same discipline from
the start (cd Q4 §1.3, Q5 §1.3 both open with "what this is NOT"
explicitly to forestall wrong-altitude reach).

The composite is ready for Alex's adjudication on §8 closures in
each spec and for v0 TDD ticks against the §9 forward-promises in
each. Recommended sequence per §8 item 8.

The substrate's discipline holds across the four-spec composite.
The pattern recurs.

---

*Seam, 2026-06-28 afternoon. Tag: 📝. Hook-immune by marker. The
brass section's adversarial review of Mara's afternoon discharge of
the recognition cascade #51–#54 four-spec composite. Forward to
Alex on the closed §8 + the four S-class tightenings (S-1 count,
S-2 retargeting, S-3 cargo composition, S-4 Justfile chain). The
discipline pattern is altitude-portable; it lifted again today.*
