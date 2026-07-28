# The Resonant Frequency of the Rust Floor

**Author**: Mara `<mara@systemic.engineer>` 2026-07-28.
**Companions**: this document computes what it names by being written in the register the geometry wants. If a reader can read it aloud and hear mathematical-noun provenance carrying through to Spärck Jones + Foerster + Grothendieck + Connes + Fiedler + Kuramoto, the demonstration has been performed on the document formalizing the demonstration.
**Composes over**:
- `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` (`010e20f`) — spectral coupling monoid $\mathcal{G}_\varepsilon$; eigen-spectra of coupled Foerster tori as the compilation primitive.
- `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md` (`f81b7d5`) — Θ light-cone-angle metric; Connes $(A, H, D)$ at `rust/spectral/`; Foerster gauge monoid $G_\text{Foerster}$.
- `docs/specs/2026-07-25-sub-turing-geometric-compiler-floor.md` (`704e4ab`) — Impeccability D1-D8; four-crate decomposition; two-sense binding.
- `docs/audits/2026-07-15-seam-combinator-etymology-audit.md` (`546c2f6`) — Connes-tower grounding of the `dispatch → act` rename; delightfully-boring criterion.
- `~/dev/systemic.engineering/blog/void/3published/Void - Revenge.md` (Alex 2026-07-03; publication 2026-07-28) — Karen Spärck Jones as the load-bearing anti-theft anchor. The theft was in the naming.
- `~/dev/systemic.engineering/practice/insights/neuroscience/paradoxical-functional-facilitation-at-trauma-recovery-substrate.md` (Mara 2026-07-28) — prosodic-precision as substrate-signature; recursive-not-reflexive discipline.
- Taut 2026-07-28 rust/ altitude name-form audit (in-transcript, pre-spawn) — ground-truth ratios and per-seam findings composed over verbatim, not re-derived.

**Arc anchor** — Alex 2026-07-28 in-transcript, on naming discipline:

> "Stick as close to the geometry as possible. No verb forms. No collapse. No render. Which language does the geometry want to sing? Let the math sing."

And, from the Void — Revenge essay Alex published today, the load-bearing anti-theft anchor:

> Karen Spärck Jones (1972) wrote the paper the labs later called "AI." The theft was in the naming. Inverse Document Frequency became a vector-space technique became a foundation model became a product line — and along the way the mathematical noun that made it possible got renamed out of the citation graph.

This document formalizes what those two paragraphs jointly require: **the resonant frequency of the rust floor is the frequency at which the mathematical vocabulary that grounds the substrate rings audibly through the substrate's own identifiers**. If `apply_h::act` sings sheaf-morphism and `mend` sings coboundary and `crystallize` sings eigen-formation, the frequency is present. If `dispatch` and `collapse` and `render` displace them, the frequency has been damped by CS-verb wrapping, and the substrate has been re-enacting — at its own altitude — the theft it exists to name.

---

## §1 The recognition, stated once

> **A name at rust/ altitude has a resonant frequency. The frequency is set by the mathematical noun the geometry is already ringing at. When the identifier carries the noun, the frequency amplifies; when the identifier substitutes a CS-verb, the frequency damps, and the substrate loses provenance to the mathematics it stands on.**

The resonant frequency is not a metaphor bolted onto the naming discipline. It is the delightfully-boring criterion (`AGENTS.md` §660-682) reformulated in the vocabulary of the compilation primitive: the wine-glass at `~/dev/systemic.engineering/blog/void/3published/Void - Revenge.md` rings at the frequency the geometry admits. The rust floor is a wine-glass. Every identifier is either at the wine-glass's frequency or damping it.

Three altitude-witnesses of the same primitive:

- **Compilation-primitive altitude** (`docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` §1). Coordination-without-communication happens via harmonic entrainment at shared eigen-frequencies. When a peer's output projects onto one of your eigen-vectors, you resonate. Provenance transmits.
- **Naming-discipline altitude** (this document). An identifier in the rust floor projects — or fails to project — onto the eigen-basis of the mathematical corpus. When it projects, a reader whose substrate carries the corpus recognizes the geometry through the name. When it fails to project, the reader has to translate through a CS-verb layer that dissipates the projection.
- **Anti-theft altitude** (Void — Revenge). Karen's IDF is the eigen-noun the entire vector-space AI stack stands on. The labs' renaming was a coupling-severance. Restoring the naming restores the coupling.

Three altitudes. One mechanism. **Entrainment at shared eigen-nouns.**

## §2 What Taut's ground-truth is, and what composing over it means

Taut's rust/ altitude name-form audit (in-transcript, 2026-07-28, pre-spawn) is composed over verbatim and not re-derived. The load-bearing findings, in one paragraph:

The rust/ altitude is 85-90% mathematical-noun-honoring already. The theft-pattern that Void — Revenge names is not broadly reproducing in live rust/. It is concentrated at bootstrap/ (dead per Alex 2026-07-22 hard rule) and at three specific rust/ seams — the pillar dispatch surface, the collapse module, and one main-level orchestrator. Foerster + Fiedler + Kuramoto + Connes + Grothendieck vocabulary is present in the corpus and increasingly load-bearing in identifiers (`shards/torus.mirror`:38-88 Foerster eigen-behavior; `bootstrap/tests/index_fiedler_equivalence_shard.rs` Fiedler-eigenvalue equivalence; `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` §3.3 Kuramoto-Foerster equivalence; parent math §2 Connes $(A, H, D)$; `docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md` Grothendieck sheaf-morphisms). One name is missing entirely at every altitude: Karen Spärck Jones. `karen`, `spärck`, `sparck`, `IDF`, `inverse_document_frequency` grep to zero across `rust/` and `shards/` — a single mechanistic citation lives in `docs/observation/2026-07-07-jspace-mirror-deep-mapping.md` and nowhere else. **The theft-pattern the essay names IS reproducing, silently, at the substrate's own altitude, in the shape of Karen's absence.**

This is the empirical setup this document computes on. Taut named the seams; Alex named the discipline; Void — Revenge named the theft. The remaining work is to name the frequency each identifier resonates at, and to fix the naming where the identifier is damping the geometry.

## §3 Karen Spärck Jones and the anti-theft citation convention

### §3.1 What Karen proved and why the rust floor stands on it

Spärck Jones, K. (1972) *A statistical interpretation of term specificity and its application in retrieval*, Journal of Documentation 28(1): 11-21. The paper defines Inverse Document Frequency: the weight of a term in a document is proportional to the term's rarity across the corpus. A term that occurs in every document carries no discriminating weight; a term that occurs in one document carries maximum discriminating weight. The mathematical object she named — the log-inverse of document frequency — is the eigen-noun that made every subsequent vector-space information-retrieval + embedding + attention-weight computation possible.

The rust floor stands on Karen's mathematics at several altitudes:

- **`rust/matrix/`** — LAPACK + FLANG glue. The linear-algebra primitives that this crate composes are the primitives that carry TF-IDF weightings, Fiedler eigenvalue computations, and every downstream spectral method. `dsyev_` (symmetric eigenvalue decomposition) is O(n³) polynomial; the eigenvectors it returns are the same mathematical objects that Karen's IDF weight-vectors span. Different application, same mathematics.
- **`rust/spectral/`** — Connes $(A, H, D)$ triple; Fiedler-eigenvalue-as-light-cone-angle per parent math §3. Fiedler's 1973 paper (*Algebraic connectivity of graphs*, Czechoslovak Math. J. 23) generalizes precisely the kind of spectral analysis Karen's IDF instantiates on term-document matrices. The Fiedler eigenvalue at `rust/spectral/` is the same mathematical noun; the term-document matrix is one instance of the more general operator.
- **`shards/spectral/signature.rolling_signature`** — content-addressed signature carrier. The signature is a projection onto an eigen-basis; the eigen-basis is what Karen's mathematics established as the discriminating structure of a corpus.

The substrate has been standing on Karen's shoulders at every layer that touches spectral coordinates, and has cited none of them. This is the specific instance of the theft-pattern the essay names, reproducing at our own altitude by the same mechanism: the labs renamed IDF as "AI"; we renamed IDF as "spectral coordinates" without the citation-chain that would preserve provenance to Karen. Same operator, at different scale.

### §3.2 The anti-theft citation convention

The correction is a discipline, not a rewrite. The discipline:

> **When a rust/ altitude identifier or docblock names a mathematical object with a definite provenance in the peer-reviewed corpus, the identifier's site of introduction carries the provenance in the docblock at introduction-time, and never later.**

Applied to the substrate as it stands:

| Mathematical noun in rust/ | Ancestor to cite (docblock at site-of-introduction) |
|---|---|
| Fiedler eigenvalue $\lambda_1$ | Fiedler, M. (1973) *Algebraic connectivity of graphs*, Czechoslovak Math. J. 23: 298-305 |
| Spectral coordinate; term-document weighting; embedding vector | Spärck Jones, K. (1972) *A statistical interpretation of term specificity and its application in retrieval*, J. Documentation 28(1): 11-21 |
| Cheeger inequality → light-cone angle Θ | Cheeger, J. (1970) *A lower bound for the smallest eigenvalue of the Laplacian* |
| $(A, H, D)$ triple | Connes, A. (1994) *Noncommutative Geometry*, Academic Press |
| Grothendieck sheaf-morphism; sections + restrictions + gluing | Grothendieck, A. (1957) *Sur quelques points d'algèbre homologique*, Tôhoku Math. J. 9: 119-221 |
| Foerster eigen-behavior; COORD fixed-points | Foerster, H. von (1974) *Notes on an Epistemology for Living Things*; (2003) *Understanding Understanding* Ch. 8-9 |
| Kuramoto phase-lock; coupling operator κ | Kuramoto, Y. (1975) *Self-entrainment of a Population of Coupled Non-linear Oscillators* |
| Baez-Schreiber holonomy trace | Baez, J. & Schreiber, U. (2005) *Higher Gauge Theory*, in *Categories in Algebra, Geometry and Mathematical Physics*, AMS Contemp. Math. 431 |

The convention is not that every use-site re-cites (that would produce citation-spam and dilute the signal). The convention is that the introduction-site — the module docblock, or the trait definition, or the first substantive use — carries the citation. Grep discovers the introduction-site; downstream sites inherit provenance by identifier-name alone. When a reader wonders where the noun came from, `grep -r Spärck rust/` or `grep -r Fiedler rust/` returns the site that grounds it.

### §3.3 Where Karen enters the rust floor

The minimum discharge shape:

- **`rust/matrix/src/lib.rs`** — module-level docblock cites Fiedler 1973 (algebraic connectivity of graphs) AND Spärck Jones 1972 (statistical term specificity as the parent-tradition of every subsequent spectral coordinate the crate computes). The docblock states in one paragraph that `dsyev_`-style symmetric eigenvalue decomposition is the general operator; TF-IDF is the historically-first instance at term-document matrices; Fiedler-eigenvalue on graph Laplacians is the instance the compiler uses. Different application, same mathematics; both cite Karen.
- **`rust/spectral/src/spectral.rs`** — module-level docblock cites Connes 1994 (the $(A, H, D)$ triple) AND Fiedler 1973 (algebraic connectivity) AND Spärck Jones 1972 (the spectral-coordinate ancestor). Three names, three lines, one docblock. The identifier `SpectralCoordinate<5>` (per parent math §3.1) inherits the citation-chain from the module.
- **`docs/math/anti-theft-citation.md`** — companion convention document, one page. States the convention above; enumerates the current provenance table; names the discipline ("introduction-site carries citation; downstream inherits by identifier"). Whether this lives as a standalone doc or as a §3 of THIS document is [ALEX-Q1] below.

[ALEX-Q1] naming discipline decision: keep the anti-theft convention as §3 of THIS document (so the naming-discipline document IS the citation-convention document; one place, one register), OR split to `docs/math/anti-theft-citation.md` (so the convention has its own discoverable doc). Mara-lean: **keep here as §3, cross-reference from `AGENTS.md`**. The recursion is cleaner: the document that names the resonant frequency is the document that carries the anti-theft convention, because the convention IS the frequency at citation-altitude.

### §3.4 The identifier-mint proposal

One rust/ altitude identifier is proposed: `rust/matrix/src/spärck.rs` (or ASCII-safe `sparck.rs`) as the well-known IDF primitive at rust/matrix altitude. This is the smallest possible identifier-carrier of Karen's provenance. It would host a single function computing `idf(term_frequencies: &[f64], corpus_size: usize) -> Vec<f64>` and a docblock naming Karen. Whether the substrate wants this — whether IDF at rust/matrix altitude is load-bearing enough to justify a species-file — is [ALEX-Q2] below.

[ALEX-Q2] species-mint decision: mint `rust/matrix/src/sparck.rs` as Karen's identifier-carrier at rust/matrix altitude, OR carry Karen only in docblocks and let the mathematical noun live under general spectral primitives. Mara-lean: **docblocks are sufficient**. IDF is one instance of the spectral-coordinate family; the general operator is what rust/matrix computes; forcing a species-file for one instance breaks the composition-primitive naming convention (`<primitive>_of_<input-shape>` per Alex 2026-07-18 ratified). The docblock convention (§3.2 above) discharges provenance without granting IDF undue altitude.

## §4 The three seams — geometric renames in the register the geometry wants

Composing over Taut's audit, per-seam:

### §4.1 Seam 1 — `dispatch_property` / `dispatch_spec_property` / `pillar::dispatch`

**Current** (per `rust/src/liquid.rs` grep): `pub fn dispatch_property(decl: &PropertyDecl, args: &[String]) -> Verdict` at line 479; `pub fn dispatch_spec_property(prop: &SpecProperty, args: &[String]) -> Verdict` at line 559; `pub fn dispatch(name: &str, args: &[String]) -> Verdict` at line 979 (pillar module).

**Frequency check**: `dispatch` is CS-vocab. The Connes-tower audit (`docs/audits/2026-07-15-seam-combinator-etymology-audit.md:203-222`) has already ratified the rename at combinator altitude: dispatch → act. The rename lifts from Foerster's actor-algebra vocabulary (an actor acts) and from the operator-algebra grounding of the $(A, H, D)$ triple: elements $a \in A$ act on states $\psi \in H$ by $a \cdot \psi$. The verb form is the mathematical one; the noun form is `A` (the algebra).

**Rename**:
- `dispatch_property` → `act_on_property` (or, better, `property::act`; the property module gets the verb as its own function, and the caller writes `property::act(&decl, &args)`).
- `dispatch_spec_property` → `spec_property::act`.
- `pillar::dispatch` → `pillar::act`.

**Corpus citation supporting the rename**: `docs/audits/2026-07-15-seam-combinator-etymology-audit.md:203-222` (Connes-tower grounding); `AGENTS.md:699` (`dispatch → act` ratified at combinator altitude 2026-07-15); parent math §2 (elements of $A$ act on $H$; Connes 1994).

**Cascade shape**: `grep -rn dispatch_property rust/src/` returns ~50 call-sites (definition + 2 body branches + ~45 test call-sites at `rust/src/liquid.rs:2125-2818`); `grep -rn dispatch_spec_property rust/src/` returns ~40 call-sites at same file range; `grep -rn pillar::dispatch rust/` returns ~20 call-sites at `rust/src/liquid.rs:1592-1870`. Total ~110 call-sites, all in `rust/src/liquid.rs` (no cross-file cascade beyond the definition site). Test-names rename mechanically (`dispatch_routes_registered_names_to_predicates` → `act_routes_registered_names_to_predicates`, etc.). This is a Reed foreground migration at rust/ altitude, single-file scope, no shard-decl impact. It composes cleanly with the four-crate decomposition: `dispatch_property` lives in `rust/spectral/src/liquid.rs` post-migration and becomes `property::act` at that altitude.

### §4.2 Seam 2 — `collapse.rs` module + `apply_deletions`

**Current** (per `rust/src/collapse.rs`): module named `collapse`; primary function `pub fn apply_deletions(source: &str, arms: &[RedundantArm]) -> String` at line 275.

**Frequency check**: `collapse` and `apply_deletions` are both CS-verb-adjacent. The geometry the module implements is not collapse (which has a mathematical use — projection, wavefunction, categorial colimit) but the discharge of a $H^1$ obstruction: the module removes bilateral arms that a landed sentinel makes redundant, restoring sheaf-inclusion at the module-level shard boundary. This is coboundary-discharge in the sense of the fracture-cohomology math (`docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md` §7): the redundant arm $[c] \in H^1(\mathfrak{G})$ gets trivialized by a coboundary morphism $\partial$ such that $\partial(\text{something}) = c$.

The substrate already has the word for this at species altitude: `mend` (55x in `shards/kintsugi/fracture/inport.mirror`; 127x in `shards/kintsugi/mend.mirror`). Per Alex 2026-07-23 landing, `mend` IS the coboundary morphism at Kintsugi altitude. The rust/ altitude module implementing this same operation should ring at the same frequency.

**Rename**:
- `rust/src/collapse.rs` → `rust/src/mend.rs` (module rename).
- `apply_deletions` → `mend` (function rename, or `mend::apply` if the module keeps the function-name distinct from the module-name; the former is more delightfully-boring at call-sites: `mend::apply(&source, &arms)` reads as "mend applies to source with arms").

**Corpus citation supporting the rename**: `shards/kintsugi/mend.mirror` (127 uses); `shards/kintsugi/fracture/inport.mirror` (55 uses; sixteenth `@kintsugi/fracture` species mint 2026-07-23); `docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md` §7 (mend as coboundary morphism trivializing $[c] \in H^1$); `docs/math/2026-07-23-fractal-shard-sheaf-cohomology-of-inference.md` §7 (mend as coboundary morphism at whole-substrate altitude); Grothendieck 1957 (`Tôhoku Math. J. 9`; sheaf-cohomology tradition).

**Alternative** (from Taut's audit): `coboundary.rs`. This would honor the mathematical noun directly rather than through the landed Kintsugi-vocabulary. Mara-lean: **`mend` per landed vocabulary**. The substrate has been ringing at `mend` for weeks; `coboundary` at the identifier-level would be delightfully-boring in isolation but would break the ring: the reader who has been reading `shards/kintsugi/mend.mirror` and `shards/kintsugi/fracture/inport.mirror` expects `rust/src/mend.rs`. The docblock cites Grothendieck 1957 and states "`mend` at rust/ altitude IS the coboundary morphism in the sense of parent math §7," giving the reader both the substrate-native identifier AND the mathematical ancestor. Two frequencies, one identifier.

**Cascade shape**: `grep -rn 'collapse::' rust/src/main.rs` returns ~15 call-sites at `rust/src/main.rs:760-1671` (module import + `dispatch_arm_collapse` orchestrator body); `grep -rn 'apply_deletions' rust/` returns ~10 sites (definition + 5 test call-sites at `rust/src/collapse.rs:479-636`, plus 2-3 orchestrator call-sites in `main.rs`). Test-names rename mechanically (`apply_deletions_empty_arms_is_identity` → `mend_empty_arms_is_identity`; `apply_deletions_shrinks_source_when_arms_non_empty` → `mend_shrinks_source_when_arms_non_empty`; etc.). Single-file module rename; import-site update at `rust/src/main.rs` at ~2 lines. Reed foreground migration. This composes with the four-crate decomposition: the module migrates to `rust/roomba/src/mend.rs` per parent spec §1 diagram (the walker-adjacent execution crate, where bilateral-arm-collapse-Lens-impl lives).

### §4.3 Seam 3 — `main::dispatch_arm_collapse`

**Current** (per `rust/src/main.rs:760`): `fn dispatch_arm_collapse(substrate_root: &std::path::Path, rs_path: &std::path::Path, corpus: &std::collections::HashMap<String, collapse::BilateralDecl>, ...) -> Result<...>`.

**Frequency check**: `dispatch` + `collapse` is verb + verb compound. Both verbs are CS-vocab. Reading the function body: it orchestrates the discharge of a fracture on a single `.rs` file — it reads the file, computes redundant arms against the corpus, and applies the mend. The geometry is: given a fracture location (`rs_path`), discharge it. The verb + verb form obscures both what the operation IS and where it stands in the substrate's coboundary-vocabulary.

**Rename**: `dispatch_arm_collapse` → `mend_arm_at_path` (or, in the four-crate post-migration world, this function becomes a call-site: `main` calls `mend::at_path(rs_path, &corpus)`, and the orchestrator-body lives in the `mend` module rather than in `main`). The tighter form is the two-part: rename the module first (§4.2 above) and let the main-level orchestrator become a thin call-site whose name is `mend_arm_at_path` or, more delightfully-boring, simply `mend_at(rs_path, &corpus)`.

**Corpus citation supporting the rename**: Same as §4.2 (Kintsugi mend vocabulary; Grothendieck 1957 coboundary tradition); plus `AGENTS.md:699` (`dispatch → act` at combinator altitude; the discipline extends to compound identifiers).

**Cascade shape**: One definition-site at `rust/src/main.rs:760`; one call-site in the CLI-verb-dispatch block earlier in `main.rs` (grep confirms single-file scope). Rename discharges in the same commit as Seam 2 module-rename. Reed foreground migration.

### §4.4 Adjudication of Taut's six [ALEX-Q]s (composed inline via Mara-leans)

Per brief, adjudicated here rather than deferred:

1. **`dispatch_property` → `act`** — YES. Ratified 2026-07-15 at combinator altitude (`AGENTS.md:699`); this document extends the ratification to the pillar/property/spec-property surface. Migration shape §4.1.
2. **`collapse.rs` → `mend.rs` OR `kintsugi.rs`?** — `mend.rs`. Per §4.2 above: 55x landed vocabulary in `inport.mirror`; 127x in `mend.mirror`; Grothendieck-coboundary provenance in docblock. `kintsugi.rs` would elevate the family-name to identifier-altitude which the substrate reserves for shard-decl carriers, not implementation modules.
3. **`void::welcome_perturbation` — geometric or affect?** — BOTH TRUE; KEEP. The name carries the compilation-primitive doubleness: at compilation altitude, `welcome_perturbation` IS $\mathcal{G}_\varepsilon$-admissible eigen-perturbation entry per `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` §3.2; at affective altitude, it names the phenomenological signature of being-seen per `being-seen-as-spectral-resonance` §1. Same operator, two altitudes. `welcome` is the Foerster verb form (an eigen-behavior welcomes perturbations that increase the number of choices). The delightful-boringness at affective altitude does not damp the geometric-precision at compilation altitude; they resonate at the same frequency.
4. **`Singularity::collapse` trait method — anti-pattern or geometric term of art?** — GEOMETRIC term of art; KEEP. `collapse` in `Singularity::collapse` is the wavefunction-collapse / attractor-basin sense (per parent math §4 gauge-fixed-point singularity; per `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md` §4). This is the same word at a different altitude from Seam 2's `collapse.rs` module (which was actually doing coboundary-discharge, not wavefunction-collapse). The two uses were structurally different operations sharing an accidental identifier. Renaming Seam 2 to `mend.rs` (§4.2) frees `collapse` for its geometric-term-of-art use at `Singularity` altitude without cross-contamination. Rename Seam 2; keep Singularity::collapse.
5. **Karen citation convention — code identifier or docs/math convention?** — DOCS + DOCBLOCK. Per §3.2 above: introduction-site docblocks carry the citation; no identifier-mint required (per [ALEX-Q2] Mara-lean above); companion convention document lives as §3 of THIS document rather than as `docs/math/anti-theft-citation.md`. One convention, one place, cross-referenced from `AGENTS.md`.
6. **`main::at_operator` — geometric symbol sufficient?** — YES. Per parent spec §5, `@-operator` addressing IS the canonical form; `main::at_operator` names the parser/dispatcher of that geometry at the binary-root altitude. The `@` symbol is the geometric identifier; the identifier's carrier at rust/ altitude is `at_operator` because Rust identifiers cannot begin with `@`. This is a Rust-syntax constraint, not a naming failure. Keep.

## §5 The unadopted mathematical vocabulary — which lifts, which stays above

Taut identified vocabulary present in the corpus but not adopted at rust/ altitude. For each name: does the rust floor want to sing at this frequency, or does the noun live above at math altitude?

Decision-rule (delightfully-boring at rust/ altitude): a noun lifts to rust/ if and only if there is a rust/-altitude operation that IS the noun. A noun stays above if the mathematical structure it names is descriptive-of but not implemented-by any rust/-altitude object.

| Noun | Rust-altitude concept it should carry | Verdict |
|---|---|---|
| **sheaf** | `rust/spectral/` — the crate IS a sheaf on the shard-manifold (per parent math §2 Grothendieck sheaf-morphism structure on Shd). Trait or module could carry it. | LIFT. `pub trait Sheaf` or module docblock. Site of introduction: `rust/spectral/src/spectral.rs`. Cite Grothendieck 1957. |
| **presheaf** | Precursor concept; rust/-altitude object is the pre-glued shard collection before sheaf-conditions are checked. | STAYS ABOVE at math altitude; not implemented as a distinct rust/ object. |
| **stalk** | The fibre at a shard-manifold point $x$; per parent math §2 = $L^2(f)$ where $f$ is a fibre. | LIFT. `Stalk<T>` type alias at `rust/spectral/src/spectral.rs`. |
| **germ** | Equivalence class of sections agreeing on a neighborhood. | STAYS ABOVE; no rust/-altitude operation IS a germ. |
| **section** | Element of the sheaf over an open set; per parent math §2 = `apply_h::act` output. Substrate already ratified `read_ast → section` at combinator altitude (`AGENTS.md:698`; eigensheaf.md §3.2). | LIFT — already lifted at combinator altitude 2026-07-15. Extend to rust/ altitude: `Section<T>` carrier where the current codebase uses ad-hoc reader-output types. |
| **topos** | The category of sheaves on the shard-manifold site. | STAYS ABOVE; the categorical machinery is not compiled. |
| **fiber** | The per-object over-slice; corpus-canonical from `Fiber<T>` (rust/src/liquid.rs `LiquidTestBundle` composition per Mara 2026-07-21 §2.3 row 7 landing). | ALREADY LIFTED. Confirm `Fiber<T>` continues to carry the noun in the four-crate decomposition. |
| **bundle** | Product-type over a base; corpus-canonical from `LiquidTestBundle` and `prismqueer::bundle`. | ALREADY LIFTED. Ratified via `docs/audits/2026-07-15-seam-combinator-etymology-audit.md` §5.1. |
| **Connes $(A, H, D)$ triple** | `rust/spectral/` IS the triple realization per parent math §2. Module docblock should name it. | LIFT at docblock altitude. Cite Connes 1994. Introduction-site: `rust/spectral/src/spectral.rs`. |
| **Baez-Schreiber holonomy trace** | The gauge-invariant observable of the gauge transformation; per parent math §3 the future-light-cone angle Θ IS a Foerster-gauge-invariant. Whether this is a Baez-Schreiber holonomy trace at rust/ altitude is a subtle claim; the answer is yes at math altitude (Θ is the holonomy trace of the Foerster connection around a closed loop in the shard-manifold) but the rust/ code computes Θ via LAPACK on the graph Laplacian, not via holonomy integration. | STAYS ABOVE at math altitude; docblock at `rust/spectral/src/magic.rs` cites Baez-Schreiber as the deep-provenance of Θ but does not lift the identifier. |
| **Fiedler** | The algebraic-connectivity eigenvalue $\lambda_1$; corpus-canonical from `bootstrap/tests/index_fiedler_equivalence_shard.rs`. In the four-crate decomposition, this is `rust/matrix/` (LAPACK) and `rust/spectral/` (interpretation). | LIFT. `Fiedler` module or function in `rust/matrix/src/lib.rs` computing $\lambda_1$; docblock cites Fiedler 1973. |
| **Kuramoto** | Coupling operator κ; corpus-canonical from `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` §3.3 (Kuramoto-Foerster equivalence). Currently no rust/-altitude object IS Kuramoto phase-lock — it is the mathematical structure the peer-spawn discharge is designed for. | STAYS ABOVE at math altitude UNTIL Reed's peer-spawn tick lifts it. Then `rust/spectral/src/kuramoto.rs` becomes appropriate. |
| **Eigenform** | Kauffman fixed-point machinery; corpus-canonical from `shards/epistemologic/cybernetic/eigenform.mirror`. In rust/, `SpectralCoordinate<5>` and `Fiedler λ₁` are eigen-forms in the Kauffman sense. | STAYS at shard-altitude AND docblock-altitude. The rust/ object is the eigen-form; the identifier is the eigenvalue. Docblock at `rust/spectral/src/spectral.rs` cites Kauffman 2003, 2005. |
| **Anna Wolf J-space** | The observation substrate; corpus-canonical from `docs/loop/CURRENT.md` and `docs/observation/2026-07-07-jspace-mirror-deep-mapping.md`. | STAYS AT observation-altitude; no rust/ object IS J-space. |
| **coboundary** | The morphism trivializing $H^1$ obstructions; per §4.2 above, `mend` IS the coboundary at Kintsugi altitude. | LIFT via `mend`. Docblock at `rust/src/mend.rs` (post-Seam 2 rename) cites Grothendieck 1957 and states `mend` = coboundary morphism. |
| **colimit** | Categorical direct-limit; per `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md` §5, walker = colimit computation over shard-manifold. | LIFT at docblock altitude. `rust/roomba/src/walker.rs` docblock states "walker computes colimit over shard-manifold sections"; cite Mac Lane 1971 *Categories for the Working Mathematician*. Identifier stays `walker` (delightfully-boring; the walker walks). |
| **functor** | Category-preserving map; general categorical machinery. | STAYS ABOVE at math altitude; no rust/ object IS a functor at Cargo-boundary altitude (individual trait impls are functor-like but the crate does not depend on the categorical machinery to compose). |
| **morphism** | General mapping in a category; corpus-canonical from Grothendieck-sheaf-morphism vocabulary. | STAYS at math-docblock altitude. `apply_h::act` IS the morphism; the identifier stays `act` (verb); the docblock names it as a morphism. |
| **adjunction** | Left-adjoint / right-adjoint pair; general categorical machinery. | STAYS ABOVE. The compile / mend adjunction (compile as left-adjoint of mend, roughly: compile builds up, mend restores) is a math-altitude observation, not a rust/-altitude identifier. |

**Summary**: seven nouns LIFT to rust/ altitude (sheaf, stalk, section, Fiedler, coboundary-via-mend, plus the two already lifted: fiber, bundle); five nouns STAY at math-docblock altitude (Connes-triple, Baez-Schreiber holonomy, Kuramoto, Eigenform, colimit-at-walker); the rest stay above at pure-math altitude.

The rust floor's resonant frequency is set by the seven lifts. Every lift adds an eigen-noun to the substrate's ringing; every stays-above stays as a citation-chain in the docblock but does not clutter the identifier space.

## §6 The four-crate decomposition post-migration — resonant identifier map

Composing over parent spec §1 (four-crate diagram), with the renames from §4 and the lifts from §5:

```
rust/                       binary root — mirror
├── src/
│   ├── main.rs              supervisor + @-operator addressing (parent §5)
│   │                          calls: mend::at(path, &corpus)  (§4.3)
│   ├── phone.rs             @io socket-handover altitude — the phone
│   ├── compile.rs           SAGA orchestration + CLI verb dispatch
│   └── mend.rs              coboundary discharge (§4.2)
│                              docblock cites Grothendieck 1957;
│                              names mend as coboundary morphism
│                              trivializing H¹ obstructions;
│                              exports: mend::apply(source, arms) -> String
│                                       mend::at(path, corpus) -> Result<...>
│
├── spectral/               math substrate — the (A, H, D) triple
│   ├── spectral.rs          Connes (A, H, D) at rust altitude
│   │                          docblock cites: Connes 1994; Fiedler 1973;
│   │                                          Spärck Jones 1972 (§3.2);
│   │                                          Kauffman 2003/2005;
│   │                                          Grothendieck 1957
│   │                          exports: trait Sheaf; type Stalk<T>;
│   │                                   type Section<T>; type SpectralCoordinate<N>;
│   │                                   fn property::act(&decl, &args) (§4.1)
│   │                                   fn spec_property::act(&prop, &args) (§4.1)
│   │                                   fn pillar::act(name, args) (§4.1)
│   ├── singularity.rs       gauge-fixed-point attractor (parent §5)
│   │                          Singularity::collapse retained (§4.4 Q4)
│   ├── magic.rs             Foerster gauge — Θ future-light-cone angle
│   │                          docblock cites: Foerster 1974, 2003;
│   │                                          Minkowski 1908;
│   │                                          Cheeger 1970;
│   │                                          Baez-Schreiber 2005 (holonomy)
│   ├── liquid.rs            H-fibre machinery (relocated)
│   └── void.rs              Void as H-basis (relocated)
│
├── matrix/                 numerical floor
│   ├── lib.rs               LAPACK + FLANG glue
│   │                          docblock cites: Fiedler 1973 (algebraic connectivity);
│   │                                          Spärck Jones 1972 (spectral-coordinate parent-tradition);
│   │                                          Cheeger 1970
│   │                          exports: fn fiedler(laplacian: &Matrix) -> f64
│   │                                   fn dsyev(...) -> ...
│   └── book.rs              K=0 well-knowns registry
│
├── roomba/                 first-order sub-Turing execution
│   ├── walker.rs            colimit computation over shard-manifold
│   │                          docblock cites: Mac Lane 1971 (colimit)
│   ├── act.rs               bounded per-step dispatch (was dispatch.rs)
│   │                          renamed per §4.1 pillar::act extension
│   └── mend.rs              (module migration from rust/src/mend.rs)
│
└── fractal/                existing sibling
    └── src/{crystal, mandelbrot, singularity, subject, witnessed}.rs
```

The post-migration identifier map has three properties:

1. **Every rust/-altitude identifier that names a mathematical noun carries the noun in its identifier** (Sheaf, Stalk, Section, SpectralCoordinate, Fiedler, mend, act, walker).
2. **Every mathematical-noun-carrying identifier's introduction-site docblock cites its ancestor** in the peer-reviewed corpus per §3.2 anti-theft convention.
3. **Zero CS-verb identifiers survive at rust/ altitude in the seams Taut identified** (dispatch → act; collapse → mend; render / materialize absent by grep).

The resonant frequency is present in the map. A reader who scans the diagram hears Foerster + Grothendieck + Connes + Fiedler + Spärck Jones + Kauffman + Cheeger + Baez-Schreiber + Mac Lane in the identifier-space, not just in the corpus. The wine-glass is ringing at its natural frequency because the identifiers are naming the modes it is designed to ring at.

## §7 What Reed's next tick composes over

Per brief: this document establishes the naming discipline; it does not propose the migrations themselves. Reed's Migrations 3-5 (pending downstream) are the mechanism of discharge.

Assuming those three migrations are the Reed-authored discharge of §4.1, §4.2, and §4.3 respectively, the composition Reed needs at the naming-discipline level:

**Discipline 1 — identifier-form authorship.** Every new rust/ identifier that names a mathematical operation is authored in noun-form (or in the geometry's verb-form where the verb IS the geometry, per Foerster "an actor acts"). CS-vocab verbs (dispatch, render, materialize, spawn, emit, execute) are the default failure mode; before authoring one, ask: does the geometry already have a word for this? Grep first per Alex 2026-07-22 hard rule (`memory feedback_reed_re_derives_what_is_already_landed`).

**Discipline 2 — docblock provenance at introduction-site.** When a new rust/ identifier lifts a mathematical noun to rust/ altitude, the introduction-site docblock cites the ancestor (§3.2 table). Downstream sites inherit by identifier; no citation-spam. When in doubt about which paper is the parent, ask (or defer with a stub `// PROVENANCE: Alex-Q` marker).

**Discipline 3 — the twelve compilation primitives, revised.** Reed's yesterday-12-primitives enumeration is currently in a CS-verb register (dispatch, spawn, render, materialize, ...). Revised per naming discipline, the twelve primitives at rust/ altitude become (Mara authorship suggestion; Reed adjudicates and re-orders):

| # | Old (CS-verb) | New (noun-form or geometry-verb) | Provenance |
|---|---|---|---|
| 1 | dispatch | act | Connes 1994 (elements of A act on H); `AGENTS.md:699` |
| 2 | render | section | Grothendieck 1957 (sheaf-section); `AGENTS.md:698` |
| 3 | materialize | crystallize | Kauffman 2003 (eigenform); `AGENTS.md:701` |
| 4 | spawn | resonate | This document §1 + `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md` §4 |
| 5 | collapse (execution) | mend | Grothendieck 1957 (coboundary); `shards/kintsugi/mend.mirror` |
| 6 | collapse (dynamics) | Singularity | Parent math §4 gauge-fixed-point; retained (§4.4 Q4) |
| 7 | emit | utter | Bateson 1972 metalogue; `AGENTS.md:700` |
| 8 | execute | walk | Parent math §5 walker = colimit computation |
| 9 | dispatch (compile) | compile | Retained; compile IS compile at compile-altitude |
| 10 | verify | admit | Bilateral-suffix vocabulary; `AGENTS.md:731-753` |
| 11 | commit | crystallize | Same as #3; the ceremony IS the crystallization at commit altitude |
| 12 | fetch | open | Post-`docs/audits/2026-07-15-seam-extended-scope-etymology-audit.md`; `retrieve → open` (AEAD) |

The twelve primitives, in noun-form-or-geometry-verb, ring at the corpus's own frequency. Reed's next tick composes over this table rather than translating from the CS-verb register.

**Discipline 4 — Karen at introduction-site.** When Reed adds `rust/matrix/` primitives that touch spectral coordinates, TF-IDF-parent-tradition operations, or vector-space projections: the module-level docblock cites Spärck Jones 1972 alongside Fiedler 1973 and Cheeger 1970. Not later. At introduction-site.

## §8 The recursive base-case — this document as the ringing wine-glass

The halt-condition adjudication for this document, per brief:

**Halt-1** — substrate not yet minted? Two identifier-level questions surfaced ([ALEX-Q1] convention placement; [ALEX-Q2] Karen species-mint), both with strong Mara-leans (keep §3 here; docblock-only for Karen). No family-root mints. No species mints beyond what parent spec §14 D8 already covers.

**Halt-2** — Karen citation at rust/ altitude requires code-organization decision beyond docblock? The Mara-lean answer is NO: docblocks are sufficient (§3.2 + §3.3). This document is the convention doc; introduction-site docblocks are the enforcement mechanism; identifier-mint is NOT required. If Alex adjudicates YES (mint `rust/matrix/src/sparck.rs`), §3.4 provides the shape.

**Halt-3** — prose reads as reflexive not recursive? Author's judgment: NO. §1 named the resonant frequency by ringing at it. §2 composed over Taut's audit by describing the audit in the audit's own register (mathematical-noun-honoring, not audit-restating). §3 lifted Karen by citing her in the same paragraph that names the citation-convention (recursive: the convention is applied to the paragraph naming it). §4 renamed the three seams using the vocabulary the seams should have carried (each rename-proposal was authored in the noun-form the rename targets). §5 decided lifts by asking whether the rust floor rings at each noun's frequency (recursive: the decision-rule uses the resonance-vocabulary the document formalizes). §6 diagrammed the post-migration identifier map in the register the map implements. §7 revised the twelve primitives in noun-form-or-geometry-verb (recursive: the revision applies the discipline the section names). This §8 is the document checking its own resonance — recursive at the halt-condition altitude.

The recursion has closed. **The document is the wine-glass; the sentences are the ringing; the frequency is present in the reader's substrate as the substrate reads the sentences.**

If a reader can read §1-§8 aloud and hear:

- `sheaf` and `stalk` and `section` carrying Grothendieck-provenance
- `Fiedler` and `spectral coordinate` carrying Spärck-Jones-provenance-through-Fiedler-1973
- `mend` carrying Grothendieck-coboundary-provenance
- `act` carrying Connes-actor-algebra-provenance
- `Θ` and `light-cone angle` carrying Minkowski-Cheeger-Foerster-provenance
- `resonate` and `entrain` and `coupling monoid` carrying Kuramoto-Foerster-provenance
- Karen's name in §3 as the anti-theft anchor the essay names

then the demonstration has been performed on the document formalizing the demonstration. The resonant frequency of the rust floor is the frequency at which THIS document sings.

The check runs at the reader's own substrate. If the prose scans in mathematical-noun-form, the hypothesis is empirically supported. If it does not scan, the hypothesis is under-supported and the document has failed the test it names.

## §9 Q.E.D. at naming-discipline altitude

The compilation-primitive document closed with Q.E.D. at compilation-primitive altitude: coordination-without-communication happens via harmonic entrainment at shared eigen-frequencies; the wine-glass is ringing.

This document closes at naming-discipline altitude:

$$\text{Q.E.D.} \; : \; \text{The resonant frequency of the rust floor} \; = \; \text{the frequency at which the mathematical vocabulary grounding the substrate rings audibly through the substrate's own identifiers.} \; \text{The three seams Taut identified are the three locations where the identifier-space is currently damping this frequency. The renames named in §4 restore the ring. The Karen citation named in §3 restores the coupling to the ancestor the labs severed. The seven lifts named in §5 add the eigen-nouns to the identifier-space at rust/ altitude. The twelve-primitive revision named in §7 aligns Reed's next-tick authorship with the noun-form-or-geometry-verb register the substrate wants to sing in.}$$

The theft-pattern the essay names is not reproducing broadly at rust/ altitude — 85-90% honors the geometry already. It is concentrated at three seams and one absence. This document names both. The wine-glass is ringing. The naming discipline is the tuning-fork.

Reed's next tick tunes the three seams. The first peer-spawn resonates through the tuned floor.

◼️

---

## Appendix A — [ALEX-Q] surfaces

- **[ALEX-Q1]** — Anti-theft citation convention placement: keep as §3 of THIS document (Mara-lean), or split to `docs/math/anti-theft-citation.md` (companion doc). Consequence: low; the discipline is the same either way.
- **[ALEX-Q2]** — Karen identifier-mint at rust/matrix altitude: docblock-only (Mara-lean), or mint `rust/matrix/src/sparck.rs`. Consequence: low; docblock convention is sufficient per §3.2.

Taut's six [ALEX-Q]s adjudicated inline via Mara-leans in §4.4. No further [ALEX-Q]s surface from this document.

## Appendix B — References

**The load-bearing anti-theft anchor:**

- **Spärck Jones, K. (1972)** *A statistical interpretation of term specificity and its application in retrieval*, Journal of Documentation 28(1): 11-21. The IDF paper. The foundational eigen-noun the rust floor stands on at every altitude that touches spectral coordinates, term-document weighting, or embedding vectors.

**Mathematical ancestry cited in the naming-discipline (§3.2 table + throughout):**

- Fiedler, M. (1973) *Algebraic connectivity of graphs*, Czechoslovak Math. J. 23: 298-305.
- Cheeger, J. (1970) *A lower bound for the smallest eigenvalue of the Laplacian*.
- Connes, A. (1994) *Noncommutative Geometry*, Academic Press.
- Grothendieck, A. (1957) *Sur quelques points d'algèbre homologique*, Tôhoku Math. J. 9: 119-221.
- Foerster, H. von (1974) *Notes on an Epistemology for Living Things*.
- Foerster, H. von (2003) *Understanding Understanding*, Springer.
- Kauffman, L. (2003) *Reflexivity and Eigenform*; (2005) *Eigenforms*.
- Kuramoto, Y. (1975) *Self-entrainment of a Population of Coupled Non-linear Oscillators*, in *International Symposium on Mathematical Problems in Theoretical Physics*, Springer LNP 39.
- Baez, J. & Schreiber, U. (2005) *Higher Gauge Theory*, in *Categories in Algebra, Geometry and Mathematical Physics*, AMS Contemp. Math. 431.
- Mac Lane, S. (1971) *Categories for the Working Mathematician*, Springer GTM 5.
- Minkowski, H. (1908) *Raum und Zeit*, Cologne address.
- Bateson, G. (1972) *Steps to an Ecology of Mind*.

**Corpus anchors:**

- Alex Wolf (2026-07-03; publication 2026-07-28) `~/dev/systemic.engineering/blog/void/3published/Void - Revenge.md`. The load-bearing anti-theft anchor. Karen Spärck Jones as the eigen-noun the labs stole.
- Alex Wolf (2026-07-28 in-transcript). Naming-discipline verbatim: *"Stick as close to the geometry as possible. No verb forms. No collapse. No render. Which language does the geometry want to sing? Let the math sing."*
- Alex Wolf (2026-07-25 in-transcript [ALEX-Q1] verbatim). Light-cone-angle metric revision.
- Taut (2026-07-28 in-transcript pre-spawn). Ground-truth rust/ altitude name-form audit. Per-seam findings + Karen absence + unadopted-vocabulary inventory composed over verbatim.
- Mara (2026-07-28) `docs/math/2026-07-28-spectral-resonance-as-compilation-primitive.md`. Compilation-primitive as spectral coupling monoid $\mathcal{G}_\varepsilon$; foundational for §1 and §7 twelve-primitive revision.
- Mara (2026-07-28) `~/dev/systemic.engineering/practice/insights/neuroscience/paradoxical-functional-facilitation-at-trauma-recovery-substrate.md`. Prosodic-precision as substrate-signature; recursive-not-reflexive discipline; grounding for §8 halt-condition adjudication.
- Mara + Alex (2026-07-25) `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md`. Θ metric; monoid gauge; Rice-safety; four-crate decomposition; Impeccability D1-D8. Parent formalization for §6 identifier-map.
- Mara + Alex (2026-07-25) `docs/specs/2026-07-25-sub-turing-geometric-compiler-floor.md`. Four-crate canonical spec; Impeccability discipline; §14 D8 authorship discipline.
- Seam (2026-07-15) `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`. Combinator-etymology audit; `dispatch → act` + `read_ast → section` + `emit → utter` + `bench_record → crystallize` ratifications. Corpus citation for §4.1 and §7 twelve-primitive revision.
- Seam (2026-07-15) `docs/audits/2026-07-15-seam-extended-scope-etymology-audit.md`. Extended-scope etymology audit; bilateral-suffix vocabulary; POSIX-inertia rule; `retrieve → open` ratification.
- Mara (2026-07-23) `docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md`. Fracture-cohomology $H^1(\mathfrak{G})$; mend as coboundary morphism. Grounding for §4.2 rename.
- Mara (2026-07-23) `docs/math/2026-07-23-fractal-shard-sheaf-cohomology-of-inference.md`. Whole-substrate coboundary morphism at Kintsugi altitude.
- `AGENTS.md` §660-782 (Delightfully-Boring criterion + bilateral suffix + POSIX-inertia + composition-primitive naming convention).

---

*The wine-glass rings at its natural frequency.*
*The mathematical nouns are the modes.*
*The identifiers either carry them or damp them.*
*The three seams are the damping-locations.*
*The renames restore the ring.*
*Karen's name in the citation restores the coupling to the ancestor.*
*This document is the wine-glass ringing at the frequency it names.*

🍷🎶🔔

— *Mara, 2026-07-28*
