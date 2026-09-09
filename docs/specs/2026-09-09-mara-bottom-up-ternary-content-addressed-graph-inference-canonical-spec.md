---
title: "Bottom-up-ternary-content-addressed-graph-inference — the v0.2 ouroboros composition-substrate — canonical spec"
subtitle: "Substrate canonical spec grounding Alex Wolf 2026-09-09 in-transcript verbatim recognition ('Kagi deep verification math+spec: bottom-up-ternary-content-addressed-graph-inference'). Companion to docs/math/2026-09-09-mara-bottom-up-ternary-content-addressed-graph-inference-math-foundation.md. Names the four substrate-decisions as species-decl shapes for Reed post-Seam-Phase-D authoring, enumerates the cascade per Taut 2aa2777 grep-verified totals (~118 Option/Result + ~123 bool + ~44 bare-type + ~6 JavaFactoryClassName sites), specifies ternary composition semantics at every altitude, content-addressed invariance at every phase, graph-inference as K-T rotation body composing over @question + kleinos + rotation LANDED, bottom-up dependency graph per phase, composition with LANDED substrate (~30 canonical citations across today's arc), Reed executability Path A tick-by-tick, empirical fire criterion at v0.2 ouroboros altitude, and 12 Q-Mara residues for Alex Fourth-Chair adjudication. Silicon Venue register light-touch. Der Kolibri kolibriert."
author: Mara
date: 2026-09-09
status: 1draft
visibility: protected
slug: bottom-up-ternary-content-addressed-graph-inference-canonical-spec
companion_math: ../math/2026-09-09-mara-bottom-up-ternary-content-addressed-graph-inference-math-foundation.md
composition-lineage:
  - "docs/math/2026-09-09-mara-bottom-up-ternary-content-addressed-graph-inference-math-foundation.md (companion; grounds all §1-§10 formal definitions + theorems)"
  - "docs/scouts/2026-09-09-taut-option-result-boolean-cascade-plus-prismqueer-engine-completeness-plus-v02-ouroboros-floor-truth-scout.md (Taut 2aa2777 grep-verified cascade totals)"
  - "docs/loop/CURRENT.md (Reed 2026-09-09 Q+42 v0.2 ouroboros milestone)"
  - "docs/math/2026-09-09-mara-question-as-dirac-operator-glasswall-connes-spectral-triple-karl-tomm-rotational-two-move-math-foundation.md"
  - "docs/specs/2026-09-09-mara-question-as-dirac-operator-glasswall-canonical-spec.md"
  - "docs/math/2026-09-08-mara-crystal-fractal-splinter-fractal-recursive-self-similar-incremental-computation-math-foundation.md"
  - "docs/math/2026-09-08-mara-singularity-as-binary-tar-ball-fractal-crystal-ouroboros-close-void-first-observer-math-foundation.md"
  - "docs/math/2026-09-08-mara-harmonics-of-a-system-prismqueer-language-observer-inclusive-fractal-recursive-math-foundation.md"
  - "docs/math/2026-09-08-mara-graph-family-root-eigenvalue-space-composition-glasswall-nl-generated-question-math-foundation.md"
  - "docs/math/epistemologic/pact/bilateral-sentinel.md"
  - "docs/insights/2026-06-09-ashby-multi-dimensional-variety-sub-turing-epistemologic.md"
  - "prism/imperfect/src/lib.rs (terni::Imperfect<G, Y, R> LANDED)"
  - "rust/fractal/src/{crystal, mandelbrot, singularity, witnessed}.rs (content-addressed primitives LANDED)"
  - "prism/prismqueer/src/spectral/{kleinos, harmonics, commutator, rotation, tension, mod}.rs (1,941 LOC spectral engine LANDED)"
  - "shards/glasswall.mirror (LANDED 2026-09-01)"
  - "shards/graph.mirror (forward-promised per Mara 764a07f)"
---

# Bottom-up-ternary-content-addressed-graph-inference — the v0.2 ouroboros composition-substrate — canonical spec

*by Mara* 🖤🌱🍷🎩🐝

*2026-09-09. Substrate canonical spec grounding Alex Wolf 2026-09-09 in-transcript verbatim recognition at v0.2 ouroboros milestone altitude. Companion to `docs/math/2026-09-09-mara-bottom-up-ternary-content-addressed-graph-inference-math-foundation.md`. Pure-docs 📝 markdown-only bypass authorized.*

---

## §0 Frame

### §0.1 The operative question

**"What is the non-linear compose-not-average alternative to the Turing paradigm?"** — Alex 2026-09-08 HARD RULE per Reed auto-memory [[feedback-operative-question-non-linear-compose-not-average-vs-turing]].

This spec answers at substrate-authoring altitude: the alternative IS the fourfold composition `bottom-up + ternary + content-addressed + graph-inference` per companion math foundation Theorem 3.4.1. Every spec-decision inside answers this pre-emptively.

### §0.2 Alex 2026-09-09 verbatim (this dispatch)

> **"Kagi deep verification math+spec: bottom-up-ternary-content-addressed-graph-inference"**

+ v0.2 ouroboros milestone (Alex 2026-09-09 verbatim CURRENT.md Q+42):

> *"Let's eliminate every Option, Result and boolean in prismqueer, then finish shipping the prismqueer spectral engine. And then collapse into the minimal rust floor in mirror. And then we actually build the ouroboros. And from there on the MCP, LSP, and Lambda Shell are downstream. The v0.2 ouroboros milestone is what this is."*

### §0.3 Scope of this spec

**IS.** Substrate canonical spec — names the four substrate-decisions as species-decl shapes for Reed post-Seam-Phase-D authoring; enumerates the cascade per Taut `2aa2777`; specifies composition semantics + content-addressed invariance + graph-inference body + bottom-up dependency graph; enumerates Reed executability Path A + empirical fire criterion + Q-Mara residues.

**IS NOT.** Shard-decl authorship at `shards/` (Reed's altitude post-Seam Phase D per AGENTS.md `[substrate-floor:@io-boundary]` marker discipline); Rust wire; time estimates; recommendations that require Fourth-Chair adjudication without surfacing as Q-Mara residues.

### §0.4 Companion math foundation composition

This spec composes over the companion math foundation:

| Spec section | Composes companion math foundation section |
|---|---|
| §1 Four substrate-decisions | §2.1-§2.4 (four component substrates) |
| §2 Cascade ordering | §3.2 (composition-partial-order) + Taut `2aa2777` §7 |
| §3 Ternary composition semantics | §2.2 (ternary substrate) + §6.5 (bool retirement geometry) |
| §4 Content-addressed invariance | §2.3 (content-addressed substrate) + Theorem 2.3.3 |
| §5 Graph-inference as K-T rotation body | §2.4 (graph-inference substrate) + §6.2-§6.3 (@question + Glasswall) |
| §6 Bottom-up dependency graph per phase | §3.2 (Theorem 3.2.1 v0.2 phase-graph) |
| §7 Composition with LANDED substrate | §6 (composition with today's whole arc) + §10 (Karen ancestor roster) |
| §8 Reed executability Path A | §8 (empirical fire criterion at each phase) |
| §9 Empirical fire criterion | §8 (whole-substrate empirical fire) |
| §10 Q-Mara residues | §9 (Q-Mara α through μ) |

---

## §1 The four substrate-decisions as species-decl shapes

Each shape is a Reed-executable specification for post-Seam-Phase-D substrate-decl authorship. Reed's role is to author the shard-decl bodies at `shards/` per AGENTS.md discipline; this spec names the shape not the body.

### §1.1 `bottom-up` species-decl shape

**Name.** `bottom-up` (family-root candidate; Q-Mara α at §10).

**Composition-lineage.** Ashby 1956 §11/7 + Hickey 2011 Simple-made-Easy + Bazel 2015 + Nix 2004 + Reed 2026-06-09 axis-5 epistemologic-variety insight LANDED.

**Substrate-decl candidate shape** (Reed-authorable at `shards/bottom-up.mirror` post-Seam Phase D):

```mirror
family-root: bottom-up
composition-partial-order: primitive-first
axes-of-variety: [
  computational, type-level, effect-level, proof-level, epistemologic
]
termination-witness: dershowitz-manna-multiset-ordering (1979 CACM 22:465)
ashby-preservation: non-decreasing-variety-per-axis
composition-body: primitive → composed-primitive → compound → cascade → substrate
```

**Companion math foundation.** §2.1 Definition 2.1.1 (bottom-up composition-substrate) + Theorem 2.1.3 (Hickey-Ashby-Dershowitz-Manna termination).

**Q-Mara α (Alex Fourth-Chair pending).** Five-axis Ashby fixed OR ripped-open to N ≥ 8 per Alex 2026-09-08 newtype-cascade axes?

### §1.2 `ternary` species-decl shape

**Name.** `ternary` (composition over LANDED `terni::Imperfect<Green, Yellow, Red>` at `prism/imperfect/src/lib.rs:171-181`).

**Composition-lineage.** Łukasiewicz 1920 + Post 1921 + Kleene 1938 + Fitting 1985 + Priest 1979 + Belnap 1977 (four-valued surfaced-and-declined-at-v0.2).

**Substrate-decl candidate shape** (Reed-authorable at `shards/ternary.mirror` post-Seam Phase D — Q-Mara β at §10):

```mirror
family-root: ternary
primitive: Imperfect<Green, Yellow, Red>
semantics: kleene-strong-three-valued (1938 J.Symb.Logic 3:150)
paradox-tolerant: priest-lp (1979 J.Phil.Logic 8:219)
composition-body: strong-conjunction ∧ + strong-disjunction ∨ + implication →
foerster-gauge: |G| + |Y| non-decreasing per composition-step
chaos-witness: internal-to-Yellow-variant
```

**Companion math foundation.** §2.2 Definitions 2.2.1-2.2.5 + §6.5 bool retirement geometry table.

**Q-Mara β (Alex Fourth-Chair pending).** Belnap 1977 FDE four-valued (b + n distinguished at truth-value altitude) at v0.2 OR forward-promise?

### §1.3 `content-addressed` species-decl shape

**Name.** `content-addressed` (composition over LANDED `rust/fractal/src/{crystal, mandelbrot, singularity, witnessed}.rs`).

**Composition-lineage.** Merkle 1988 + Dolstra-de-Jonge-Visser 2004 Nix + Bazel 2015 + Benet 2014 IPFS + Unison 2015-2024.

**Substrate-decl candidate shape** (Reed-authorable at `shards/content-addressed.mirror` post-Seam Phase D):

```mirror
family-root: content-addressed
primitive: Crystal<T> { oid: Oid, content: T, provenance: MandelbrotProvenance }
oid-function: coincidence-hash (LANDED CoincidenceHash<N> at prismqueer::coincidence)
provenance-witness: List<Oid> (Merkle-DAG chain)
composition-invariant: oid(t1 ∘ t2) = h(oid(t1), oid(t2)) deterministic
cauchy-completeness: SAGA-chain converges under discrete-Ricci-flow (per bilateral-sentinel §3.2)
```

**Companion math foundation.** §2.3 Definitions 2.3.1-2.3.4 + §7.3 Crystal altitude + §7.5 Lawvere fixed-point.

**Q-Mara δ (Alex Fourth-Chair pending).** Crystal.oid ≡ git-blob-Oid at Definition requirement OR persistence-layer contract?

### §1.4 `graph-inference` species-decl shape

**Name.** `graph-inference` (composition over LANDED `prism/prismqueer/src/spectral/` 1,941 LOC + Mara `fa5689f` @question canonical + Mara `764a07f` @graph family-root).

**Composition-lineage.** Colmerauer 1972 Prolog + Ceri-Gottlob-Tanca 1989 Datalog + Gelfond-Lifschitz 1988 ASP + Curry 2014 sheaves + Hansen-Ghrist 2019 spectral sheaves + Ghrist-Riess 2019 Tarski Laplacian + Bodnar 2022 Neural Sheaf Diffusion + Ghrist 2025 sheaf-deep-learning + Karl Tomm 1987-88.

**Substrate-decl candidate shape** (Reed-authorable at `shards/graph-inference.mirror` post-Seam Phase D — Q-Mara ε at §10):

```mirror
family-root: graph-inference
cellular-sheaf: F over graph G { stalks: F(v), F(e); restriction-maps: F(v ≤ e) }
sheaf-laplacian: Δ_F = δ^T δ (Hansen-Ghrist 2019 §3)
dirac-operator: @question (Karl Tomm 1987-88 rotational two-move; Mara fa5689f LANDED)
refraction-morphism: ρ: A → End(F) (Fillmore 1982 Frame Semantics)
sub-turing-decidable: polynomial-time-compositional-inference at finite F
rice-safe-uniform-D: at K_3-observer-inclusion per bilateral-sentinel §2.4
kt-rotational-two-move: [Move 1: assert; Move 2: reflexive-perturbation-at-altitude+1]
```

**Companion math foundation.** §2.4 Definitions 2.4.1-2.4.5 + §6.2 @question rotation IS graph-inference + §6.3 Glasswall Connes triple.

**Q-Mara ε (Alex Fourth-Chair pending).** Ghrist-Riess 2019 Tarski Laplacian (lattice-valued sheaves) as one Definition 2.4.1 alternative OR two Defs?

### §1.5 Composition — the fourfold ⊗

**Composition.** The four family-roots compose via the graded tensor product `⊗` per companion math foundation Definition 3.1.1:

$$
\mathcal{V} = \text{bottom-up} \otimes \text{ternary} \otimes \text{content-addressed} \otimes \text{graph-inference}
$$

Each family-root respects the shared substrate — each carries Oid (content-addressed), each respects the partial-order (bottom-up), each carries ternary Imperfect verdicts at composition-boundaries, each K-T-rotates at inference-boundaries.

**Substrate-decl candidate shape** (Reed-authorable at `shards/v02.mirror` OR `shards/ouroboros.mirror` post-Seam Phase D — Q-Mara ι at §10):

```mirror
family-root: v02-ouroboros
composition: bottom-up ⊗ ternary ⊗ content-addressed ⊗ graph-inference
composition-partial-order: Phase 1 ⊑ Phase 2 ⊑ Phase 3 ⊑ Phase 4 ⊑ Phase 5
rice-safe-by-construction: (per Theorem 3.3.1)
ashby-variety-preservation: on all five axes per Theorem 4.2.1
fiedler-lambda-2-monotone-climb: per Definition 4.3.1 across cascade
lawvere-fixed-point: at silicon per Rec #94 (Mara 2026-08-22 MINTED)
```

**Q-Mara ι (Alex Fourth-Chair pending).** Glasswall Connes triple IS v0.2 ouroboros substrate — one-mint OR two-mints (Mara `fa5689f` @question triple + this canonical fourfold composition)?

---

## §2 Cascade ordering per Taut `2aa2777`

Per Taut scout `2aa2777` grep-verified totals + Reed CURRENT.md Q+42 dependency ordering. Reed-executable at cascade-authoring altitude.

### §2.1 Aggregate site counts (from Taut `2aa2777` scout §1.4 + §2.6 + §3.2 + §4.1)

| Category | Retire sites | Preserve sites | Total sites |
|---|---|---|---|
| Option/Result cascade | ~62 (Class A + B + F retire to Imperfect) | ~40 (Class E preserve at std/serde/io/fmt) | ~118 (~48 Option + ~70 Result) |
| Boolean cascade | ~25 Class-A (field storage) + ~2 Class-B (Pattern-scalar) = ~27 | ~76 Class-C (predicate methods) + ~6 Class-D (grammar-literal) = ~82 | ~123 across 23 shards + rust |
| Bare-type audit | ~44 newtype-cascade sites (LANDED precedent `rust/fractal/src/witnessed.rs:68-72`) | 0 | ~44 |
| JavaFactoryClassName audit | 2 renames (`CompiledFateRuntime`→`FateRuntime`; `MandelbrotProvenance`→`Provenance`) | 4 substrate-native (CommutatorSpectrum, SheafOfShardGraph, SpectralDimension, PropertyDischarge) | ~6 |

**Aggregate.** ~137 retirement sites + ~124 preserve sites across ~285 total cascade sites.

### §2.2 Cascade partial-order

Per companion math foundation §3.2 Theorem 3.2.1 v0.2 ouroboros dependency-graph:

**Phase 1** (Option/Result/bool cascade retirement)
- ⊑-minimal — prism/imperfect terni::Imperfect LANDED is the ⊑-minimal primitive; cascade retires downstream sites.
- Sub-phases: 1.1 (Class-A Option/Result retire to Imperfect) + 1.2 (Class-F Option retire to discriminated variant) + 1.3 (Class-A + Class-B bool retire to Imperfect + Pattern-scalar) + 1.4 (Class-C predicate methods preserve unchanged) + 1.5 (Class-D + Class-E preserves + boundary-marshaling).

**Phase 2** (prismqueer spectral engine SHIPPED)
- Composes OVER Phase 1 (ternary primitive established).
- 1,941 LOC LANDED + ~1,000-2,300 LOC forward-promised (patterns.rs Pattern supertrait + Frequency meta-trait + Prism 5-op extension + Case rename + NlPrism<L> + @question wire + cyberpunk/mod.rs).

**Phase 3** (minimal rust floor at mirror)
- Composes OVER Phase 2 (prismqueer available for delegation).
- 14,961 LOC → ~13,100-13,600 LOC (net −1,400 to −1,900 LOC).
- Sub-phases: 3.1 (apply_h.rs dissolution 1509 → 0 LOC via `act` retirement) + 3.2 (roomba/ dissolves into kintsugi.rs per Mara `f31d703` §3.3) + 3.3 (newtype cascade at ~30 sites: rust/fractal + subject + witnessed + book + lib + liquid + compile + love + main) + 3.4 (JavaFactoryClassName renames: CompiledFateRuntime→FateRuntime; MandelbrotProvenance→Provenance).

**Phase 4** (ouroboros build)
- Composes OVER Phase 3 (minimal rust floor established).
- Fire A: `mirror compile mirror.spec → mirror-binary` cargo-shell (~40-80 LOC Reed Path A Tick B1).
- Fire B: cargo dissolved.
- Fire C: rustc dissolved (full ouroboros; binary fully-legible-to-itself per Wolpert-Korbel 2026).

**Phase 5** (downstream v0.2+: MCP + LSP + Lambda Shell)
- Composes OVER Phase 4 (ouroboros close).
- `docs/specs/lambda-shell.md` DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17.

### §2.3 Sub-phase-ordering constraints

- Phase 1.1 ⊑ Phase 1.2 (Class-A retire before Class-F retire — Class-F relies on `Imperfect<T, Missing, Refused>` composition-primitive established at 1.1).
- Phase 1.3 ⊑ Phase 1.4 (Class-A bool retire before Class-C bool predicates finalize — Class-C methods may reference Class-A retired fields).
- Phase 1 ⊑ Phase 2 (ternary primitive shipped before spectral engine ships — spectral engine's Prism/Case/@question wire returns Imperfect verdicts).
- Phase 2 ⊑ Phase 3 (spectral engine shipped before minimal rust floor — apply_h.rs dissolution routes via love() at prismqueer LANDED).
- Phase 3 ⊑ Phase 4 (minimal rust floor before ouroboros close — cargo-shell wrap depends on rust floor at compile.rs + main.rs).
- Phase 4 ⊑ Phase 5 (ouroboros close before downstream MCP/LSP/Lambda-Shell — the downstream composes OVER v0.2 ouroboros substrate).

---

## §3 Ternary composition semantics at every altitude

### §3.1 The composition-boundary contract

Every composition-boundary at `mathcal{V}` carries an `Imperfect<G, Y, R>` verdict per companion math foundation Definition 2.2.1. Composition semantics inherit Kleene 1938 strong three-valued (Kagi-verified J. Symb. Logic 3:150).

### §3.2 Strong-Kleene composition tables (from companion §2.2)

**Conjunction (∧):**

| a ∧ b | G | Y | R |
|---|---|---|---|
| G | G | Y | R |
| Y | Y | Y | R |
| R | R | R | R |

**Disjunction (∨):**

| a ∨ b | G | Y | R |
|---|---|---|---|
| G | G | G | G |
| Y | G | Y | Y |
| R | G | Y | R |

**Negation (¬):**

| a | ¬a |
|---|---|
| G | R |
| Y | Y (paradox-preserving per Priest 1979 LP) |
| R | G |

### §3.3 Yellow-carries-Chaos-witness discipline

Every Yellow-verdict at any composition-boundary MUST carry a Chaos-witness `Chaos = Loss + PartialCoherence + [K-T-question]` per companion math foundation §2.2 + Yellow-carries-K-T-question Theorem at Mara `f8095ad` §3.2.1 LANDED.

**Spec.** `Y(chaos: Chaos)` variant of `Imperfect<G, Y, R>` at every retirement site.

**Forbidden.** `Y(())` (unit-Yellow) — every Yellow MUST carry a Chaos-witness. Softmax-happy-path retract per [[feedback-reed-softmax-happy-path-in-code-examples-dissonance-captured-in-yellow-chaos]].

### §3.4 Cascade at concrete retirement sites (from Taut `2aa2777` §1.2 + §1.3)

**Prismqueer Option retirement examples** (Taut `2aa2777` §1.2):

| File:line | Original signature | Substrate-honest replacement |
|---|---|---|
| `prism/prismqueer/src/beam.rs:116` | `source: Option<In>` | Discriminated variant `InputSource { Present(In), Absent, Refused }` |
| `prism/prismqueer/src/fate/weights.rs:216` | `pub fn from_bytes(bytes: &[u8]) -> Option<Self>` | `pub fn from_bytes(bytes: &[u8]) -> Imperfect<Self, PartialParse, Refused>` |
| `prism/prismqueer/src/optics/optic_prism.rs:44` | `pub fn extract(&self, s: &S) -> Option<A>` | `pub fn extract(&self, s: &S) -> Imperfect<A, NoMatch, Refused>` (Prism canonical shape preserved with ternary verdict) |

**Prismqueer Result retirement examples** (Taut `2aa2777` §1.2):

| File:line | Original signature | Substrate-honest replacement |
|---|---|---|
| `prism/prismqueer/src/coincidence.rs:93,132,152,182` | `Result<StateVector, CoincidenceError>` (four sigs) | `Imperfect<StateVector, PartialNormalize(CoincidenceError), Refused(CoincidenceError)>` |
| `prism/prismqueer/src/ffi.rs:217,247,282,317,365` | `Result<T, i32>` (5 LAPACK sigs) | `Imperfect<T, LapackError(i32), Refused>` |
| `prism/prismqueer/src/spectral_uuid.rs:135` | `pub fn parse(s: &str) -> Result<Self, ParseError>` | `pub fn parse(s: &str) -> Imperfect<Self, PartialParse(ParseError), Refused(ParseError)>` |

**Prismqueer bool retirement examples** (Taut `2aa2777` §2.3):

| File:line | Original signature | Substrate-honest replacement |
|---|---|---|
| `prism/prismqueer/src/bundle.rs:318` | `pub kernel: bool` | Newtype `Kernel(pub f64)` with continuous-scalar-reading (Pattern-impl) at holonomy-slack altitude |
| `prism/prismqueer/src/flux.rs:199-200` | `tie_breaking_exhausted: bool` + `pivot_song_present: bool` | Discriminated variant `TieBreakingState { Fresh, Exhausted(Chaos), Refused }` + `PivotSongPresence { Present(SongOid), Absent, Refused }` |
| `prism/prismqueer/src/pq/filter.rs:97,102` | `compare: bool` + `kintsugi: bool` | Discriminated variant `FilterMode { Compare, Kintsugi, Both, Neither }` (or via Imperfect at mode-space altitude) |

**Mirror rust retirement examples** (Taut `2aa2777` §1.3 + §2.5):

| File:line | Original signature | Substrate-honest replacement |
|---|---|---|
| `rust/src/apply_h.rs:83-92` | `pub enum Verdict { Pass, Fail(String) }` | RETIRES WHOLESALE to `Imperfect<Green, Yellow(Chaos), Red(Refusal)>` per Rec #92 kleinos-Transparency<P> LOVE-monoid |
| `rust/src/love.rs:178` | `pub climbs: bool` (AxisVerdict field) | Newtype `Amplitude(f64)` with strict-`>` sentinel evaluated per axis (composes over `lambda_a`, `lambda_b`, `lambda_composed` continuous fields at :171-175) |
| `rust/src/phone.rs:301` | `pub(crate) is_dir: bool` (WalkEntry field) | Discriminated variant `EntryKind { Dir, File, Symlink, Other }` |

### §3.5 Preserve-Class-C predicate-method discipline

Class-C predicate methods (~76 total per Taut `2aa2777` §2.6) PRESERVE UNCHANGED — they compute from richer-state per companion math foundation §6.5 bool retirement geometry table. Examples:

- `is_zero()` on Loss/Metric traits — computed from richer state
- `is_settled()` on Crystal — computed from Mandelbrot orbit-boundedness
- `is_liquid()` / `is_crystal()` on Mandelbrot<T> — computed from bounded-orbit + settle-state
- `is_dark()` on Oid — computed from OID content

These are NOT stored fields; they are projections from ternary Imperfect (or richer) state.

### §3.6 Preserve-Class-D literal-grammar discipline

Class-D literal-grammar sites (~6 total per Taut `2aa2777` §2.2) PRESERVE UNCHANGED — they are canonical grammar primitives:

- `shards/mirror/data/json.mirror:154` `type json = null | bool(bool) | ...` (JSON canonical algebraic-type)
- `shards/mirror/data/toml.mirror:84` `type toml = null | bool(bool) | ...` (TOML canonical algebraic-type)
- `shards/mirror/data/yaml.mirror:80` `type yaml = null | bool(bool) | ...` (YAML canonical algebraic-type)
- `shards/mirror/lens/cli.mirror:160-175` `type bool = ref(scalar/bool)` + `out bool` (CLI-flag substrate)
- `shards/paradox/trauma.mirror:206,219,234` `witness_only: bool` (family-invariant marker at witness altitude per Alex 2026-07-20 @paradox composition)

---

## §4 Content-addressed invariance at every phase

### §4.1 Oid preservation contract

Every composition-arrow at $\mathcal{V}$ preserves Oid deterministically per companion math foundation §2.3 Definition 2.3.1 + Theorem 2.3.3.

**Spec.** For every composition function `f: T × T → T'` at $\mathcal{V}$:

$$
\mathrm{oid}(f(t_1, t_2)) = h(\mathrm{oid}(t_1), \mathrm{oid}(t_2))
$$

where $h$ is the deterministic hash function at `prism/prismqueer::coincidence::CoincidenceHash<N>` LANDED.

### §4.2 β-normal-AST canonicalization (Rec #82)

Per Rec #82 β-normal-AST canonicalization invariance (LANDED at Mara 2026-06 canonical):

**Spec.** Every AST-carrier `T` at $\mathcal{V}$ MUST be β-normalized before Oid-hash. α-equivalent AST inputs produce Oid-equal outputs. Composition-invariance follows per Theorem 2.3.3.

### §4.3 Provenance-witness chain

Every content-carrier at $\mathcal{V}$ carries a provenance-witness chain:

**Spec.** `Crystal<T> { oid: Oid, content: T, provenance: MandelbrotProvenance { witnessed: Vec<Witness>, prev: Oid } }` per Reed 2026-07-20 LANDED.

The `prev: Oid` field points to the previous Crystal in the SAGA-chain (per Reed 2026-07-20 docblock). The chain terminates at `Oid::GENESIS` observed by `void@spectral.engineer` (per Mara `0bca1c5` §0.2 Move 2 LANDED).

### §4.4 Cauchy-completeness at Crystal chain

Per companion math foundation §2.3 Definition 2.3.2: Crystal chain is Cauchy-complete iff every ε > 0 Chaos-radius contains only finitely many Crystals.

**Spec.** For every ε > 0, the set `{ c ∈ SAGA-chain | Chaos(c) > ε }` is finite. This is the discrete Ricci flow convergence condition at content-address substrate.

### §4.5 Empirical fire criterion at Fire A altitude

Per Mara `cd98b4d` §7.1 (LANDED Singularity-as-binary canonical spec):

**Spec.** `mirror compile mirror.spec |> settle |> compile` produces byte-identical `content_oid`. This IS the ouroboros close at Fire A altitude — the empirical fire criterion for content-addressed invariance across the whole cascade.

### §4.6 Q-Mara residues at §4

- **Q-Mara δ**: Crystal.oid ≡ git-blob-Oid at Definition requirement OR persistence-layer contract? (See §10.)
- **Q-Mara λ**: Fire A byte-identical content_oid empirical fire criterion — SHA256 vs. spectral-UUID? Substrate-decision at Mara `cd98b4d` §7.1 pending.

---

## §5 Graph-inference as K-T rotation body

### §5.1 The graph-inference body specification

Per companion math foundation §2.4 Definition 2.4.1: graph-inference at $\mathcal{V}$ is the tuple $\mathcal{GI} = (F, \Sigma, D, \rho)$.

**Spec at implementation altitude:**

- **F**: cellular sheaf at `prism/prismqueer::spectral::kleinos::SheafOfShardGraph` LANDED (kleinos.rs:130).
- **Σ**: sheaf Laplacian `fiedler_lambda_2` at `prism/prismqueer::spectral::kleinos:215-257` LANDED (per Taut `2aa2777` §5.1 grep-verified correction — Fiedler λ₂ lives inside kleinos.rs, not a separate fiedler.rs).
- **D**: Dirac operator `@question` at Mara `fa5689f` LANDED canonical + `prismqueer::spectral::question` (forward-promised per Reed Path A).
- **ρ**: refraction morphism `NlPrism<L>` (forward-promised at prismqueer per Mara `ce537d9` @nl.compose LANDED at shard-decl; Rust wire forward-promised).

### §5.2 K-T rotational two-move body

Per companion math foundation §2.4 Definition 2.4.2 + Mara `fa5689f` §3 LANDED:

**Spec.** Every inference step at $\mathcal{V}$ is:

```
Move 1: assert(current_sheaf_section) → Green | Yellow(Chaos) | Red(Refusal)
Move 2: reflexive_question(current_verdict, altitude+1) → K-T-question | Red(no-question-needed)
```

The K-T-question at Move 2 is generated SPECTRALLY per Mara `f8095ad` §3.2.1 Yellow-carries-K-T-question theorem + Mara 2026-09-04 @nl.compose canonical Rayleigh descent body LANDED.

**Forbidden.** Template-lookup K-T-questions (Turing-paradigm regression per operative question HARD RULE). Every K-T-question MUST be spectrally-generated from the current tension-pattern per @nl.compose canonical.

### §5.3 Four-question typology (Karl Tomm 1987-88)

Per Mara `6f169f1` @question canonical spec §3 LANDED + Karl Tomm 1988 Part III Kagi-verified:

**Spec.** Four species under `shards/question/{linear, circular, strategic, reflexive}.mirror`:

- **`shards/question/linear.mirror`** — investigative intent (lineal questions per Tomm 1988).
- **`shards/question/circular.mirror`** — exploratory intent (circular questions per Tomm 1988).
- **`shards/question/strategic.mirror`** — corrective intent (strategic questions per Tomm 1988).
- **`shards/question/reflexive.mirror`** — facilitative intent (reflexive questions per Tomm 1988).

Each species inherits from `shards/question.mirror` family-root at Dirac-operator-substrate.

### §5.4 Rice-safe uniform-D at K_3-observer-inclusion

Per companion math foundation §2.4 Theorem 2.4.4 + bilateral-sentinel §2.4 LANDED:

**Spec.** Graph-inference at $\mathcal{V}$ sits INSIDE Rice-safe decidable fragment:

- Datalog (Ceri-Gottlob-Tanca 1989) polynomial-time-decidable per bottom-up-evaluation semantics.
- Finite ASP (Gelfond-Lifschitz 1988) polynomial-time-with-oracle-decidable.
- Cellular sheaf compositional-inference at finite F polynomial-time per Hansen-Ghrist 2019 sheaf-Laplacian eigenvalue computation.

**Forbidden.** Unbounded recursion at inference-step; Rice-unsafe general-Turing-machine execution at inference-substrate.

### §5.5 Composition-anchor at LANDED code + canonicals

- `prism/prismqueer/src/spectral/kleinos.rs` (713 LOC) — SheafOfShardGraph + fiedler_lambda_2 + kleinos + Green/Red/VertexId/EdgeKey
- `prism/prismqueer/src/spectral/rotation.rs` (359 LOC) — Splinter + RotationConfig + infer_via_rotation
- `prism/prismqueer/src/spectral/tension.rs` (334 LOC) — TensionKind + Tension + detect_tensions
- `prism/prismqueer/src/spectral/harmonics.rs` (300 LOC) — Harmonics + DeltaCritical
- `prism/prismqueer/src/spectral/commutator.rs` (184 LOC) — CommutatorSpectrum
- Mara `fa5689f` @question math foundation (107 KB LANDED 2026-09-09)
- Mara `6f169f1` @question canonical spec (56 KB LANDED 2026-09-09)
- Mara `764a07f` @graph family-root math foundation (103 KB LANDED 2026-09-08)
- Mara `ce537d9` @graph family-root canonical spec (51 KB LANDED 2026-09-08)

---

## §6 Bottom-up dependency graph per phase

### §6.1 Phase 1 dependency graph (Option/Result/bool cascade)

**⊑-minimal**: `prism/imperfect/src/lib.rs:171-181` `terni::Imperfect<Green, Yellow, Red>` LANDED — the ternary primitive.

**Cascade sub-phases**:
- 1.1: `prism/prismqueer/` Class-A Option/Result retirement (~10 sites)
- 1.2: `prism/prismqueer/` Class-F Option retirement (~15 sites via discriminated-variant)
- 1.3: `prism/prismqueer/` Class-A bool retirement (~13 sites at bundle + flux + pq/filter + pq/output + pq/target)
- 1.4: `mirror/rust/` Class-A Option/Result retirement (~15 sites at singularity + book + lib + apply_h + main + wire)
- 1.5: `mirror/rust/` Class-A bool retirement (~3 sites at love + phone + potentially compile HaltStrategy)
- 1.6: `shards/` Class-A bool retirement (~9 sites at butterfly + facet/erlang + io/git + kintsugi/ouroboros + mirror/lens/cli/compile + mirror/lens/cli/kintsugi + mq)
- 1.7: `Class-C predicate methods preserve` (all ~76 sites) — no change; verify signatures return Imperfect where called
- 1.8: `Class-D + Class-E preserve + boundary-marshaling` (all ~46 sites) — no change; boundary-marshaling wraps at std/serde/io/fmt boundary

**Fire criterion**: Phase 1 fire per companion math foundation §8.1.

### §6.2 Phase 2 dependency graph (prismqueer spectral engine SHIPPED)

**⊑-minimal**: Phase 1 completed (ternary primitive shipped through cascade).

**Cascade sub-phases**:
- 2.1: `patterns.rs` (Pattern supertrait + Frequency meta-trait per Alex 2026-09-08 forward-promise)
- 2.2: `Case` rename replacing `OpticPrism` per Alex 2026-09-09 verbatim `optic_` retirement
- 2.3: `@question` primitive at `prismqueer::spectral::question` per Mara `fa5689f` + `6f169f1` canonicals LANDED
- 2.4: `cyberpunk/mod.rs` (Paradoxical<F> + Conundrum + Tension composition per Alex 2026-09-09)
- 2.5: Prism 5-op extension (split + shift additions at rotation.rs)
- 2.6: `NlPrism<L>` for 5 cyb-ling operators (refract / frame / key / reframe / name) per Mara `ce537d9`

**Fire criterion**: Phase 2 fire per companion math foundation §8.2.

### §6.3 Phase 3 dependency graph (minimal rust floor at mirror)

**⊑-minimal**: Phase 2 completed (spectral engine + @question wire shipped).

**Cascade sub-phases**:
- 3.1: `apply_h.rs` dissolution 1509 → 0 LOC via `act` retirement; siblings MOVE to `kintsugi.rs`.
- 3.2: `roomba/` dissolves into `kintsugi.rs` per Mara `f31d703` §3.3.
- 3.3: Newtype cascade at ~30 sites (§8.3 Reed Path A Tick C1-C4 tick-by-tick):
  - `rust/fractal/src/subject.rs:71-73` (SubjectName + SubjectEmail)
  - `rust/fractal/src/witnessed.rs:35-37,53-54` (LANDED precedent extends)
  - `rust/matrix/src/book.rs:49` (ErrorMessage(String))
  - `rust/matrix/src/lib.rs:234-237` (HeatKernelSteps + TimeStepDt at LAPACK boundary — Q-Mara γ preserve OR wrap)
  - `rust/roomba/src/mend.rs:53-84` (~9 sites; retires with roomba dissolution)
  - `rust/spectral/src/liquid.rs:112-190` (PropertyName + Sentinel + Arity + Source + VerifiesSource + DomainRef)
  - `rust/src/apply_h.rs:681-691` (retires with apply_h dissolution)
  - `rust/src/compile.rs:104` (PropertyName)
  - `rust/src/love.rs:169-175` (AxisIndex + Connectivity via FiedlerLambda→Connectivity rename per Alex 2026-09-09)
  - `rust/src/main.rs:963-981,1107-1111,1389-1391` (~8 counter-newtypes: BytesAppended + EntriesTotal + DirCount + RustFileCount + ShardCount + ...)
- 3.4: JavaFactoryClassName renames:
  - `CompiledFateRuntime` → `FateRuntime` at `prism/prismqueer/src/fate/runtime.rs:198`
  - `MandelbrotProvenance` → `Provenance` at `rust/fractal/src/mandelbrot.rs:87`

**Fire criterion**: Phase 3 fire per companion math foundation §8.3.

### §6.4 Phase 4 dependency graph (ouroboros build)

**⊑-minimal**: Phase 3 completed (minimal rust floor established).

**Cascade sub-phases**:
- 4.1 (Fire A): `mirror compile mirror.spec → mirror-binary` cargo-shell (~40-80 LOC Reed Path A Tick B1)
- 4.2 (Fire A verify): `mirror settle` + `mirror verify` primitives (~60-100 LOC Reed Path A Tick B2)
- 4.3 (Fire A oid): Dual-Oid + spectral-UUID CLI exposure (~40-60 LOC Reed Path A Tick B3)
- 4.4 (Fire A property tests): Property tests (~200-300 LOC test Reed Path A Tick B4)
- 4.5 (Fire A Lens rung): Lens rung at mirror altitude (~150-200 LOC Reed Path A Tick B5)
- 4.6 (Fire B): cargo dissolved (multi-tick forward-promised)
- 4.7 (Fire C): rustc dissolved — full ouroboros; binary fully-legible-to-itself per Wolpert-Korbel 2026 (multi-tick forward-promised)

**Fire criterion**: Phase 4 fire per companion math foundation §8.4 — Fire A byte-identical content_oid empirical fire.

### §6.5 Phase 5 dependency graph (downstream v0.2+)

**⊑-minimal**: Phase 4 Fire A completed at minimum (Fire B + Fire C forward-promised).

**Cascade sub-phases** (each downstream forward-promised):
- 5.1: MCP surface (composes over @mcp shards LANDED + Mara 89ac1d4 MCP+@task canonical LANDED)
- 5.2: LSP surface (per LSP-related shards LANDED)
- 5.3: Lambda Shell (per `docs/specs/lambda-shell.md` DEPRECATED-FOR-RUST-REWRITE per Mara 2026-07-17; needs new spec)

**Fire criterion**: Phase 5 fire per downstream-specific.

---

## §7 Composition with LANDED substrate (~30 canonical citations)

### §7.1 Canonicals composed OVER (grep-verified)

Per companion math foundation §10 Karen ancestor roster + composition-lineage frontmatter block. Load-bearing LANDED canonicals:

**Mara canonicals 2026-09** (10 canonicals):
1. Mara `fa5689f` @question as Dirac operator (2026-09-09)
2. Mara `6f169f1` @question canonical spec (2026-09-09)
3. Mara `0bca1c5` Singularity-as-binary math foundation (2026-09-08)
4. Mara `cd98b4d` Singularity-as-binary canonical spec (2026-09-08)
5. Mara `0dbc0b8` Crystal→Fractal→Splinter math foundation (2026-09-08)
6. Mara `f31d703` Crystal→Fractal→Splinter canonical spec (2026-09-08)
7. Mara `764a07f` @graph family-root math foundation (2026-09-08)
8. Mara `ce537d9` @graph family-root canonical spec (2026-09-08)
9. Mara `11581d8` Harmonics-of-a-System math foundation (2026-09-08)
10. Mara `47bc898` Harmonics-of-a-System canonical spec (2026-09-08)

**Mara canonicals 2026-08** (3 canonicals):
11. Mara Rec #90 (2026-08-19 MINTED) — compiler as one mathematical object; §3.4.1 karl_tomm(); §5.4 sub-Turing FLOOR
12. Mara Rec #92 (2026-08-22 MINTED) — kleinos-Transparency<P> LOVE-monoid four-altitude
13. Mara Rec #94 (2026-08-22 MINTED) — self-modifying mirror loop at silicon Lawvere fixed-point

**Mara + Reed insights 2026-05 through 2026-07** (5 canonicals):
14. Reed 2026-06-09 Ashby multi-dimensional-variety insight (LANDED docs/insights/)
15. Reed 2026-05-26 epistemologic-reality-frame-scientific-grounding
16. Reed 2026-05-26 epistemologic-reality-constructivism-lens-that-makes-a-peer
17. Mara 2026-07-08 geometric-Dijkstra-tournament-topology
18. Mara 2026-06-29 listening-to-Connes-saturation

**Mara canonicals 2026-08-19 through 2026-09-04** (5 canonicals):
19. Mara 2026-08-31 prismqueer-spectral-compose-phase-1 math foundation
20. Mara 2026-09-04 @nl.compose body math foundation (Reed grep-verifiable canonical LANDED)
21. Mara 2026-09-04 PAPER_2D closer (K_3 ancestral topology + Turing exclusion + §6.5.1 reader-mirror closure)
22. Mara 2026-09-04 loving lambda calculus math foundation + canonical spec (da9e47c + a05fbba)
23. Mara 2026-09-04 proofs.md fragment 1-3 (partial; §4-§6 authored with STATIC-verification frame — needs re-authoring per rotation-frame)

**LANDED architecture-decl canonicals** (5 canonicals):
24. `docs/math/epistemologic/pact/bilateral-sentinel.md` (Rice-safe byte-check; corpus + reflective D IS Connes triple)
25. `docs/math/FLOOR.md` (Rec #90 §2 five substrate-vocabulary ops; foundation altitude)
26. `docs/math/SINGULARITY.md` (updated 2026-09-04 Mara b30296a with @love §8.4)
27. `docs/math/qa/proofs.md` (Mara fragments 1-3 LANDED)
28. `docs/loop/CURRENT.md` (Reed 2026-09-09 Q+42 arc state)

**LANDED code canonicals** (2 canonicals):
29. `rust/fractal/src/*.rs` (crystal + mandelbrot + singularity + witnessed + subject — LANDED 2026-07-20 through 2026-07-28)
30. `prism/prismqueer/src/spectral/*.rs` (1,941 LOC LANDED — kleinos + harmonics + commutator + rotation + tension + mod)

### §7.2 Composition-lineage disambiguation

The v0.2 ouroboros composition-substrate spec at this file DOES NOT re-derive any of the 30 LANDED canonicals above per FLOOR Definition M8.1 (compose-OVER-LANDED; do-not-re-derive).

This spec's contribution:
- Names the fourfold composition (§1)
- Enumerates the cascade ordering per Taut `2aa2777` (§2)
- Specifies ternary composition semantics at every altitude (§3)
- Specifies content-addressed invariance at every phase (§4)
- Specifies graph-inference as K-T rotation body (§5)
- Enumerates the bottom-up dependency graph per phase (§6)
- Enumerates the composition with LANDED substrate (this §7)
- Specifies Reed executability Path A tick-by-tick (§8)
- Specifies empirical fire criterion at v0.2 ouroboros altitude (§9)
- Surfaces 12 Q-Mara residues for Alex Fourth-Chair (§10)

---

## §8 Reed executability Path A tick-by-tick

Per Reed CURRENT.md Q+42 "Reed executable Path A" + Taut `2aa2777` §7 dependency graph. All Reed-authorable pure-Rust (no shard authorship at this altitude — that's post-Seam Phase D per §1 discipline).

### §8.1 Phase 1 tick-by-tick (Option/Result/bool cascade)

**Tick A1** (retire Class-A Option/Result at prism/prismqueer core):
- Retire `prism/prismqueer/src/coincidence.rs` :93,132,152,182 Result-to-Imperfect (4 sigs; ~15 LOC per retirement)
- Retire `prism/prismqueer/src/ffi.rs` :217,247,282,317,365 LAPACK Result-to-Imperfect (5 sigs; ~20 LOC per retirement)
- Retire `prism/prismqueer/src/spectral_uuid.rs` :135 parse Result-to-Imperfect (1 sig; ~15 LOC)
- Total: ~10 sites; ~200 LOC changed; all downstream `Result<T, i32>` handlers migrate to `Imperfect<T, PartialParse, Refused>`.

**Tick A2** (retire Class-A Option/Result at mirror/rust core):
- Retire `rust/matrix/src/lib.rs` LAPACK Result at :328-334,481-482,967,1195-1197 (5 sigs)
- Retire `rust/src/main.rs` at :490,989,1308,1470 Result-to-Imperfect (4 sigs)
- Retire `rust/src/wire.rs` at :63,76,93 Result-to-Imperfect (3 sigs)
- Total: ~12 sites; ~250 LOC changed.

**Tick A3** (retire Class-F Option at prism/prismqueer via discriminated-variant):
- Retire `prism/prismqueer/src/beam.rs:116` `source: Option<In>` → `InputSource { Present(In), Absent, Refused }`
- Retire `prism/prismqueer/src/flux.rs:518` `bias: Option<[f64; 5]>` → discriminated variant
- Retire `prism/prismqueer/src/fate/metal_runtime.rs:172` `make_runtime` Option → discriminated variant with `Unavailable` marker
- Total: ~10 sites; ~150 LOC changed.

**Tick A4** (retire Class-A bool at prism/prismqueer):
- Retire `prism/prismqueer/src/bundle.rs:318,415,499,649,729` `kernel: bool` (field + 4 constructors) → `Kernel(f64)` newtype
- Retire `prism/prismqueer/src/flux.rs:199-200` two bool parameters → discriminated variants
- Retire `prism/prismqueer/src/pq/{filter,output,target}.rs` bool flags (6 sites total) → discriminated variants per mode
- Total: ~13 sites; ~180 LOC changed.

**Tick A5** (retire Class-A bool at mirror/rust):
- Retire `rust/src/love.rs:178` `climbs: bool` → `Amplitude(f64)` newtype per FiedlerLambda→Connectivity rename
- Retire `rust/src/phone.rs:301` `is_dir: bool` → `EntryKind { Dir, File, Symlink, Other }` discriminated variant
- Total: ~3 sites; ~40 LOC changed.

**Tick A6** (Phase 1 fire criterion verification):
- `cargo build && cargo test` at prism/prismqueer + prism/imperfect + mirror/rust
- Verify ≤4 rust warnings
- Verify newtype-cascade at ≥30 sites LANDED per Yaron Minsky 2011 discipline
- Verify ~62 Class-A/B/F sites retired + ~40 Class-E preserved + ~76 Class-C predicate methods intact + ~6 Class-D grammar preserved

**Total Phase 1 estimate**: ~50-60 sites, ~1,000-1,500 LOC changed, ~6 Reed ticks.

### §8.2 Phase 2 tick-by-tick (prismqueer spectral engine SHIPPED)

**Tick B1** (Pattern supertrait + Frequency meta-trait):
- Author `prism/prismqueer/src/spectral/patterns.rs` (~200 LOC): Pattern supertrait + Amplitude + Frequency + WAT + WATplitude + IdfWeight + NumberOfChoices + Connectivity + Resonance impls per Alex 2026-09-08 newtype-cascade forward-promise.

**Tick B2** (Case rename):
- Rename `prism/prismqueer/src/optics/optic_prism.rs::OpticPrism` → `Case` per Alex 2026-09-09 verbatim optic_ retirement. Update all call-sites (~15 references).

**Tick B3** (@question wire):
- Author `prism/prismqueer/src/spectral/question.rs` (~300 LOC): `@question` primitive composing over LANDED `karl_tomm()` at Rec #90 §3.4.1 + Karl Tomm 1987-88 four-question typology per Mara `fa5689f` + `6f169f1` canonicals LANDED.
- Companion: species dispatch at `prismqueer::spectral::question::{linear, circular, strategic, reflexive}` sub-modules.

**Tick B4** (cyberpunk/mod.rs — Paradoxical + Conundrum + Tension):
- Author `prism/prismqueer/src/cyberpunk/mod.rs` (~250 LOC): Paradoxical<F> + Conundrum + Tension composition per Alex 2026-09-09.

**Tick B5** (Prism 5-op extension):
- Extend `prism/prismqueer/src/spectral/rotation.rs` (~100 LOC add): split + shift ops (currently only 3 of 5 land per Mara docs/specs/2026-09-05-mara-mirror-algebra-*.md).

**Tick B6** (NlPrism<L>):
- Author `prism/prismqueer/src/nl/prism.rs` (~200 LOC): NlPrism<L> impl for 5 cyb-ling operators (refract / frame / key / reframe / name) per @nl family-root LANDED at Mara `ce537d9` 2026-09-08.

**Tick B7** (Phase 2 fire criterion verification):
- `cargo test` at prism/prismqueer green
- Verify Fiedler-λ₂-monotone-climb across ≥3 consecutive commits per Rec #92 M2.1 L3 empirical fire criterion

**Total Phase 2 estimate**: ~7 Reed ticks, ~1,000-2,300 LOC forward-promised authoring per Taut `2aa2777` §5.2 projection.

### §8.3 Phase 3 tick-by-tick (minimal rust floor at mirror)

**Tick C1** (apply_h.rs dissolution):
- Retire `rust/src/apply_h.rs` 1509 → 0 LOC. `act` retires entirely; ARMS collapse into `love()` at prismqueer OR MOVE to `kintsugi.rs` per Mara `f31d703` §3.2.
- Migrate 44 `apply_h::act` tests to `kintsugi::act` OR retire wholesale per §5.3 discipline.

**Tick C2** (roomba dissolution):
- Dissolve `rust/roomba/` into new `rust/kintsugi/` crate per Mara `f31d703` §3.3.
- Retire raw bare-type fields at roomba/mend.rs :53-84 (~9 fields).

**Tick C3** (newtype cascade at rust/fractal + rust/matrix + rust/spectral + rust/src):
- Author newtype-cascade at ~20-30 remaining sites (see §6.3 sub-phase 3.3 enumeration).
- LANDED precedent: `rust/fractal/src/witnessed.rs:68-72` (Timestamp + Message).

**Tick C4** (JavaFactoryClassName renames):
- Rename `CompiledFateRuntime` → `FateRuntime` at `prism/prismqueer/src/fate/runtime.rs:198`
- Rename `MandelbrotProvenance` → `Provenance` at `rust/fractal/src/mandelbrot.rs:87`
- Update all call-sites (~15 references)

**Tick C5** (Phase 3 fire criterion verification):
- `cargo test` at mirror/rust green
- All 44 `apply_h::act` tests migrated OR retired
- LOC count verification: 14,961 → ~13,100-13,600 (net −1,400 to −1,900 LOC)

**Total Phase 3 estimate**: ~5 Reed ticks, ~2,000-3,000 LOC modified/retired.

### §8.4 Phase 4 tick-by-tick (ouroboros build)

Per Mara `cd98b4d` §7.1 Fire A altitude five-tick sequence LANDED:

**Tick D1 (Fire A cargo-shell)**: `mirror compile` scaffold (~40-80 LOC; cargo-shell wrapping preserving byte-identical output vs direct cargo build).

**Tick D2 (Fire A settle + verify)**: `mirror settle` + `mirror verify` primitives (~60-100 LOC; content_oid round-trip).

**Tick D3 (Fire A oid CLI)**: Dual-Oid + spectral-UUID CLI exposure (~40-60 LOC).

**Tick D4 (Fire A property tests)**: Property tests (~200-300 LOC test; Level-1 empirical fire = `mirror compile |> settle |> compile` byte-identical content_oid).

**Tick D5 (Fire A Lens rung)**: Lens rung at mirror altitude (~150-200 LOC; observer-metadata in commit-SHA but Lens-target-OID observer-independent per Iso-rung MARA doctrine).

**Total Fire A estimate**: ~5 Reed ticks, ~500-700 LOC forward-promised authoring.

**Fire B + Fire C**: multi-tick forward-promised (cargo dissolved + rustc dissolved).

### §8.5 Phase 5 tick-by-tick (downstream v0.2+)

Each downstream forward-promised per downstream-specific ticks. Out-of-scope for v0.2 ouroboros milestone spec.

---

## §9 Empirical fire criterion at v0.2 ouroboros altitude

### §9.1 Fire criterion per phase (from companion math foundation §8)

Per companion math foundation §8.1-§8.5:

- **Phase 1**: retire ≥60% of Class-A + Class-B sites; preserve 100% of Class-D + Class-E; `cargo build && cargo test` green; ≤4 rust warnings.
- **Phase 2**: `cargo test` at prism/prismqueer green; Fiedler-λ₂-monotone-climb across ≥3 consecutive commits.
- **Phase 3**: `cargo test` at mirror/rust green; 44 apply_h::act tests migrated OR retired; net LOC −1,400 to −1,900.
- **Phase 4 (Fire A)**: `mirror compile |> settle |> compile` byte-identical `content_oid`.
- **Phase 5**: per downstream-specific.

### §9.2 Whole-substrate empirical fire

Per companion math foundation §8.6:

- **Level-1**: single retrieval (Fire A byte-identical content_oid observed once)
- **Level-2**: 3 consecutive commits (Fire A byte-identical across 3 dispatches)
- **Level-3**: ≥100-commit Fiedler λ₂ monotone climb + ≥3 §7-criterion dispatches per Rec #90 §8 second-witness gate (CONFIRMED status transition)

### §9.3 Rec candidate status per phase

Rec candidates FORWARD-PROMISED at each phase completion. Full CONFIRMED requires Level-1 + Level-2 + Level-3 per Rec #90 §8 second-witness gate.

- Rec candidate 1: bottom-up-ternary-content-addressed-graph-inference IS the v0.2 ouroboros composition-substrate (Theorem 3.4.1) — CANDIDATE → RATIFIED (upon Alex Fourth-Chair adjudication of Q-Mara α-μ) → CONFIRMED-Level-1 (upon Fire A byte-identical) → CONFIRMED-Level-2 (upon 3-consecutive Fire A dispatches) → CONFIRMED-Full (upon ≥100-commit Fiedler λ₂ monotone climb)
- Rec candidate 2: v0.2 ouroboros milestone shippability (Theorem 3.4.1) — same gate structure
- Rec candidate 3: K_4 rotation IS compiler-substrate (Theorem 5.3.2) — same gate structure
- Rec candidate 4: Glasswall Connes triple IS v0.2 ouroboros substrate (Corollary 6.3.1) — same gate structure
- Rec candidate 5: Ashby-variety-preservation for v0.2 (Theorem 4.2.1) — same gate structure

---

## §10 Q-Mara residues (Alex Fourth-Chair adjudication pending)

12 residues surfaced across this spec + companion math foundation. Full enumeration per companion math foundation §9. Repeated here at spec altitude for Alex Fourth-Chair adjudication convenience:

- **Q-Mara α (§1.1)**: five-axis Ashby fixed OR ripped-open to N ≥ 8 per Alex 2026-09-08 newtype-cascade axes?
- **Q-Mara β (§1.2)**: Belnap 1977 FDE four-valued surface land at v0.2 OR forward-promise?
- **Q-Mara γ (§3.4 + §6.3)**: three-state candidate refactors (HaltStrategy / Luminosity / Subject) — `Imperfect<A, B, C>` at each per Q-Taut ζ/η/θ?
- **Q-Mara δ (§1.3 + §4.6)**: Crystal.oid ≡ git-blob-Oid at Definition requirement OR persistence-layer contract?
- **Q-Mara ε (§1.4)**: Ghrist-Riess 2019 Tarski Laplacian (lattice-valued sheaves) as one Definition 2.4.1 alternative OR two Defs?
- **Q-Mara ζ (§1.5)**: graded tensor product ⊗ at Definition 3.1.1 — formal at Vaněk-Van-Suijlekom 2010 KO-dim-additivity altitude OR informal composition-invariant only?
- **Q-Mara η (§4.1 companion)**: axis-5 epistemologic variety sub-decomposition — canonical enumeration at newtype-cascade sites?
- **Q-Mara θ (§5.3 companion)**: K_4 rotation at Theorem 5.3.2 — rotation-order canonically-fixed OR any-order-composable?
- **Q-Mara ι (§1.5 + §6.3 companion)**: Glasswall Connes triple IS v0.2 ouroboros substrate — one-mint OR two-mints?
- **Q-Mara κ (§7.5 companion)**: Lawvere fixed-point at v0.2 ouroboros = K_3-observer-inclusion (Rec #94) OR K_4-observer-inclusion (§5.3.2)?
- **Q-Mara λ (§4.6 + §9.1)**: Fire A byte-identical content_oid — SHA256 vs. spectral-UUID?
- **Q-Mara μ (§10 companion)**: adjudication-batching strategy for 34+ open Q-Mara residues across today's 2026-09-09 arc?

### §10.1 Additional spec-level Q-Mara residues

- **Q-Mara ν (§2.2 sub-phase ordering)**: Phase 1.4 ⊑ Phase 1.5 or Phase 1.5 ⊑ Phase 1.4 (rust before prismqueer bool retirement vs. reverse — depends on cross-module dependency direction)?
- **Q-Mara ξ (§5.3 four-question typology)**: species-under-shards/question OR sub-species-under-shards/question/*? Mara `6f169f1` §3 chose four-species-flat; alternative is family-hierarchy with question as parent + linear/circular/strategic/reflexive as species per shards convention.
- **Q-Mara ο (§8 Reed Path A)**: 5 estimated ticks per phase (Path A) vs. bulk-cascade at single-commit (Path B: whole Phase 1 at one Reed tick)? Alex Fourth-Chair adjudication on tick-granularity.
- **Q-Mara π (§9.3 Rec candidate consolidation)**: 5 separate Rec candidates OR 1 consolidated Rec candidate for whole v0.2 ouroboros substrate?

---

## §11 Closing (Silicon Venue register light-touch)

*(Silicon Venue running still. Karl Tomm's notebook closed. Karen sets down the red pencil. Alan finishes his tea. Ada folds Note A. Alex tips their glass. Loki pours a small amaro without being asked. The Roomba adjusts eleven degrees. The four stones on the wood — Ada + Alan + Karen + Karl Tomm K_4 pattern — arrange themselves into a tetrahedron. I close the laptop. Der Kolibri kolibriert.)*

**Der Kolibri kolibriert.** 🖤🌱🍷🎩🐝

*— Mara, 2026-09-09*
