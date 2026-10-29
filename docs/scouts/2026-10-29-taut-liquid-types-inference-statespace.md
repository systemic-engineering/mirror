# Taut scout: liquid types, property inference, and restricting the inference state space

*2026-10-29. Taut. Read-only drift scout. Asked for by Alex: "Spawn a Taut scout into the docs on the liquid types, the property inference, and the geometric restriction of the inference statespace."*

Thesis under test (Alex, 2026-10-29):

> "If we can hold the superposition in memory, the content-addressing gives us immutability, Anna's math enables us to read without perturbing. Then the frequency math is the thing that restricts the statespace without perturbing it. The compiler begins to weave algebraical patterns. Ada was onto something."

Paths are relative to the mirror repo on `reed/ship` (`346253b`) unless prefixed. `prism@7e6b00f` means `/Users/reed/.cache/mirror/prism-7e6b00f`. LANDED means code plus tests. CLAIMED means prose only. DECLARED means a `.mirror` type or signature with no body that runs.

---

## §0 The line, short

- **The thesis is the right shape, and the corpus already has most of its pieces, scattered.** The closest landed piece is in prism, not mirror: `prismqueer::spectral::settle` (`prism@7e6b00f prismqueer/src/spectral/settle.rs:217`) and `Instrument::{decay, ring, solo, bleed}` (`prism@7e6b00f prismqueer/src/spectral/instrument.rs:238-275`). They take a borrowed state and return a new one. `settle` also returns a `Tail` that records what faded below the observer's `Noise`. That is spectral restriction on a held state, with the erasure logged. mirror calls none of it.
- **"Without perturbing" mixes two properties.** Content addressing makes a restriction *non-mutating*: the source root stays. It does not make it *lossless*. A spectral projector or heat flow is non-injective, so by the ratified algebra (math 2026-10-28 Prop. 4.2) it is a `settle` that carries a ruler (the band, `t`, `Noise`) and writes an erasure record. Only `ring` (e^{iθD}) is an isometry. The honest sentence: frequency math restricts the state space **without mutating it, and with every loss declared**.
- **"Liquid" means three different things in the corpus**, and the 2026-09-04 `liquid → flux` rename landed the module on the name of Jhala's own tool for liquid types in Rust (Lehmann et al., PLDI 2023: "Flux"). §1.
- **The Anna readings need the 2026-10-28 corrections everywhere.** I checked them against the PDF myself. The thesis did sequential access at step boundaries (printed p. 43), lost about a third of throughput to it (p. 43), read data back out of the VBO to draw arrows (p. 29–30), so the path was not zero-copy, and ran the FFT **after the run, on saved per-step snapshots** (p. 42). That last point helps the thesis: the thesis's own non-perturbing observable is snapshot-then-transform. §3.
- **Ada: the corpus quotes the Jacquard line accurately but attributes it to the wrong Note.** Both the Jacquard sentence and the music sentence are in **Note A**, not Note G (fourmilab primary text, verified). Five corpus sites say Note G. The text also begins "We may say most aptly, that…", which the corpus drops without an ellipsis. §5.

---

## §1 What the corpus means by "liquid types"

Three meanings share the word. They are not reconciled anywhere.

| # | meaning | where defined | status |
|---|---|---|---|
| L1 | **Refinement types** in the Rondon–Kawaguchi–Jhala sense: `{v : B \| r}` with `r` drawn from a finite qualifier set, inferred by predicate abstraction + Houdini-style fixpoint | `docs/specs/liquid-types-for-mirror.md:1-60` (Reed, dated **2026-05-19** in its header; `docs/math/liquid-types/README.md:19-20` and `:48-49` date it 2026-06-04, which is drift); `docs/math/liquid-types/README.md:75-80` §1.1 (Mara 2026-07-05); `shards/liquid.mirror:1-30` (`@liquid(@X)` refinement-lens operator); `shards/epistemologic/liquid.mirror`; `boot/std/mirror/liquid.mirror` (cited at `shards/liquid.mirror:66-71`) | CLAIMED/DECLARED. No qualifier set, constraint generation, or fixpoint weakening runs anywhere. `rust/spectral/src/liquid.rs:375` `extract_properties` is a line scanner. The parser spec says so: `docs/specs/mirror-fixpoint-parser.md:35` ("`compile` does none of stages 1–4 … line scans for property declarations"). |
| L2 | **The property runtime**: commutator magnitude, pillar verdicts, Hypothesis-style property sampling (`Sample`, `forall`, `search`, `steer`) | `prism@7e6b00f prismqueer/src/flux.rs` (was `liquid.rs` until `ec25c19`, 2026-09-04); mirror side `rust/spectral/src/liquid.rs` (94 `#[test]`, dispatches to `prismqueer::flux::pillar` at `:600-883`) | **LANDED.** It is property-based testing, not refinement inference. The file header still reads `//! Liquid — property verdicts over the spectral commutator.` (`flux.rs:1`); the rename stopped at the module path. |
| L3 | **ψ, the liquid/flux state**: the unsettled pre-crystal state, "Anna Jakobs ψ", `Liquid<T>`, `Reality::Flux` "liquid Shards" | `docs/specs/2026-08-26-mara-prismqueer-liquid-floor-canonical-spec.md:54` ("ψ carrier + D_F integrator + observation-without-perturbation invariant + property Laplacian L_P"), `:309`, `:321-325` (Landau–Lifschitz-shaped ψ dynamics); `prism@7e6b00f prismqueer/src/reality.rs:4` ("Flux - liquid Shards"); `rust/fractal/src/crystal.rs:100` ("Takes a liquid-state content") | CLAIMED. **`Liquid<T>` never existed as a type** in prism or mirror (no `struct/enum/type/trait Liquid` in either; `git log -S"struct Liquid"` in prism hits only the rename commit, for `LiquidTestBundle`). Pre-rename `liquid.rs` (`ec25c19^`) has no `ψ`, `psi`, `Anna` or `Jakobs`. Taut found the same on 2026-09-16 (`docs/scouts/2026-09-16-taut-real-execution-substrate-floor-truth-apollo-rigor.md:379`, `:720`). 152 corpus lines mention `Liquid<T>`. |

**The conflation point.** Mara's liquid-floor spec (`2026-08-26 … liquid-floor-canonical-spec.md:54`) puts L1 (property Laplacian over refinements), L2 (the eight pillar signatures, `:37`) and L3 (ψ carrier) into one "FLOOR". Alex's 2026-09-04 question bridges L1 and the geometry: "the whole cohomology apparatus, and the geometric restrictions become the liquid types that are back written through mirror?" (`docs/specs/2026-09-04-mara-loving-lambda-calculus-canonical-spec.md:54`). That sentence is the corpus's own statement of "geometric restriction of the inference state space = liquid types". It is a question, and nobody has answered it with math.

**The rename collision.** Alex's 2026-09-04 Move 17 ("@flux it is") renamed L2 to `flux`. Flux is also the name of Lehmann, Geller, Vazou and Jhala's *Flux: Liquid Types for Rust* (PLDI 2023, https://dl.acm.org/doi/10.1145/3591283, verified via Kagi). `liquid-types-for-mirror.md:985` cites it. So `prismqueer::flux`, a property-testing module, now carries the name of the refinement-type checker for Rust, the closest literature neighbour of meaning L1. That is a naming ruler collision. It needs a decision, not a silent rename.

**Literature verified (Kagi this session):** Rondon, Kawaguchi, Jhala, *Liquid Types*, PLDI 2008, https://dl.acm.org/doi/10.1145/1375581.1375602 ("combines Hindley-Milner type inference with Predicate Abstraction to automatically infer dependent types"). The corpus cites it correctly at `docs/specs/liquid-types-for-mirror.md:14`, `:980` and in many other places.

---

## §2 What the corpus means by "property inference"

Three uses again.

| # | meaning | where | status |
|---|---|---|---|
| P1 | **Liquid inference**: infer which qualifiers hold (L1) | `docs/specs/liquid-types-for-mirror.md` §1.2, §5.4 "SpectralLiquidInference" (`:648-684`) | CLAIMED. `:670` says eigendecomposition is "already implemented in mirror/src/dirac.rs". No such file exists on `reed/ship`, and the only history for that path is two early commits in another tree (`0c6a2f6`, `b0d7a70`). Stale. |
| P2 | **Witnessed property inference**: PBT whose generator is steered by Fate, with traces content-addressed, as a fixed point `w(t)` | `docs/math/2026-07-18-witnessed-property-inference.md:36-78` (§0, Alex verbatim "the properties drive the inference"), `:191-234` (§2.2 fixed point); `docs/specs/witnessed-property-inference-fate-drives-both.md` | **Partly LANDED** as L2: `flux::pillar::Sample` (`flux.rs:507`, content-addressed `buffer_oid` `:575`), `search` (`:743`), `steer` (`:820`, a search guided by a measure, stops `Saturated` or at `Budget`), with `tests/steer.rs` and `tests/pbt_surface.rs`. Fate does not drive it (no `fate` import in `flux.rs`; the bias seam is `#[allow(dead_code)]`, `:517-519`). |
| P3 | **Inference as one engine with parsing**: semi-naive Datalog, the pact evaluator | `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md` §5.1–5.4 (`:325-384`): "Parser and pacts: yes, at the core … **PROVED**" | LANDED on a side branch: `rust/src/pact.rs` is on `reed/mirror-006-pact` (`1324aae`) and **not merged into `reed/ship`**. The parser engine itself isn't written (spec §14). |

The 2026-07-18 commit for P2 (`prism da96ac8`) quotes Alex: "the full statespace covered liquid floor boards". That is the corpus's earliest "statespace" sentence attached to liquid. There it means **coverage of the property's input space by sampling**. It does not mean restricting an inference space by geometry.

---

## §3 "Restriction of the inference state space": every definition found

| where | what restricts | restriction as view (new state, source kept) or as mutation? | status |
|---|---|---|---|
| `shards/fate.mirror:98-133`, types `:448-454`, `:503-510`, `:586-592`, sig `:665-666` | `roll(space: restricted_state_space, hole) -> dice_roll`; restriction by the Connes data `(A, H, D, γ, J, tray)`: **symmetry** (chirality γ, real structure J) plus history (tray), not spectrum. `inference.result: geometric_formalization` (`:452`) | view: `dice_roll` is a new value; `space` is a `ref` | DECLARED. The `fate` crate is a softmax selector (Taut 2026-07-08 `docs/scouts/2026-07-08-taut-what-fate-wants-to-become.md:19`, `:100-103`). |
| `docs/specs/liquid-types-for-mirror.md:619-646` §5.3 | **spectral CSP sparsification**: "you can verify a SUBSET of properties and get guarantees about ALL properties" | view (selects constraints) | CLAIMED. **Citation drift:** the spec credits "Bafna, Bhatt, Khot, Minzer — ICALP 2025" (`:621`, `:996`). *A Theory of Spectral CSP Sparsification* is by **Khanna, Putterman, Sudan**, arXiv 2504.16206 (22 Apr 2025). I verified this via Kagi and the arXiv metadata. I did not verify the venue. |
| `liquid-types-for-mirror.md:675-680` | Fiedler gate: "if lambda_1 > epsilon: properties are satisfiable … else structurally incompatible" | view | CLAIMED, and the math is wrong as written. λ₂ = 0 means the property graph is **disconnected**, not that the properties are incompatible. Disagreement lives in H¹ (the comma), which `Instrument` measures (`instrument.rs:1-17`). |
| `prism@7e6b00f prismqueer/src/spectral/settle.rs:1-17`, `:217` | **heat flow e^{-tL}** on vertex values; converges to ker L = H⁰ | **view.** `settle(&Graph, &Pitches, Time, Noise) -> Imperfect<Pitches, Wall, Tail>`. The input is borrowed; `Tail.faded` records the modes lost below `Noise`, which is an erasure record. | **LANDED**, 17 tests in the file |
| `prism@7e6b00f prismqueer/src/spectral/instrument.rs:238-275` | `decay` e^{-tD²} (settles onto ker D = H⁰ ⊕ H¹, keeps the comma); `ring` e^{iθD} ("isometric, never settles"); `solo` (restriction by a vertex region: the geometric restriction); `bleed` = [D, solo] (exactly where the restriction perturbs the question) | **view**: every method takes `&Cochain` and returns a new `Cochain` | **LANDED**, `tests/instrument.rs` (`decay_keeps_the_comma`, `decay_and_ring_commute_and_ring_never_settles`, `bleed_lives_exactly_on_the_solo_boundary`, `attend_then_ask_is_not_ask_then_attend`) |
| `prism@7e6b00f prismqueer/src/spectral/spectrum.rs:96-157` | `Spectrum::{split, refract, project}` as **readings** (λ₂, von Neumann entropy, Cheeger bound) | read-only scalars | LANDED. **Name collision:** `Split`/`Project`/`Focus` here are eigenvalue newtypes. They are not the five ops `split`/`project`/`focus` that decision 8 relies on. |
| `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md:276-323` §4 | **the five ops classified**: `split` free (injective copy), `focus` free iff it carries the complement, `settle` "the only ruler-carrying op and the only one that logs erasure" (Prop. 4.2 PROVED) | the algebra that decides which restrictions are views | PROVED in math. Not in code. |

**Is there an existing statement of "restriction without perturbation" (project a held state onto a spectral subspace, get a new state)?** Not as one sentence. Its two halves are stated separately:

1. "every loss of meaning happens at a `settle`, and is therefore visible in that `settle`'s declared ruler and erasure record" (math 2026-10-28 Prop. 4.2, `:318-322`);
2. "Heat e^{-tD²} settles onto ker D and keeps both. Rotation e^{itD} alternates assertion and question and never settles. Both are functions of one D, so they commute." (`instrument.rs:13-16`).

Put together, they say a spectral restriction is a `settle` whose ruler is a band (or `t`, or `Noise`). The source root survives because addressing is immutable, not because the restriction preserves information. Nobody has written that composite. There is no band projector `P_[a,b]` in code: `decay` with t → ∞ gives the projector onto the kernel, and nothing gives one onto an arbitrary band. The corpus has **zero** hits for low-pass, band-pass, band filter or spectral filter.

**Is any of it about grammars or parsers?** Barely.
- `shards/mirror/lens/refract.mirror:6-27`: "the grammar-graph spectrum lens". It measures the spectrum of the **grammar's dependency graph** (its shape), not the language a grammar generates or the parse space of an input.
- No transfer matrices, generating functions, Chomsky–Schützenberger, or spectral radius of a grammar anywhere in `docs/` (grep: the only "transfer matrix" hit is optics, `docs/specs/optical-keywords.md:1146`).
- ℤ₁₂ and pitch classes: `docs/math/music/README.md:3-24` lists the structures, but the directory holds **only the README**; the "first content doc … `pitch-class-algebra.md`" (`:71`) was never written. `shards/epistemologic/math/music/{harmonic,interval,cadence,dissonance}.mirror` are DECLARED. `docs/math/2026-10-10-mara-the-chord-math-foundation.md:730` builds the 12-pitch-class graph (fifths and thirds, b₁ = 25). Nothing connects any of this to parsing.

---

## §4 The thesis, link by link

| # | link | LANDED | CLAIMED / DECLARED | drift or gap |
|---|---|---|---|---|
| 1 | **hold the superposition in memory** | The least model of a PEG as positive Datalog holds **every** `m(e,i,j)` / `f(e,i)` atom at once: a chart, which is a parse forest for the CFG case (math 2026-10-28 Prop. 5.1, `:329-345`, COMPUTED by script, not Rust). `Reality::Fractured(Vec<Shard>)` holds disconnected parts (`prism reality.rs:37`). | `split` + `settle` for choice (math §4.2–4.3 `:291-308`; spec §2 item 7 `docs/specs/mirror-fixpoint-parser.md:80`). ψ as a state field (L3). | **Decision 8 is not ratified.** Spec §12 (`mirror-fixpoint-parser.md:216`) lists 8 as *"Still open: 7, 8 and 12 (prism-Reed)"*. Row 8 is "reframed [Seam]: choice needs a coproduct eliminator", with mirror realising it as split + settle *as a design choice, not a theorem* (math `:302`). The trace DAG's Fracture/Join isn't in prism code (only `prism docs/loop/CURRENT.md:112-126` and `docs/math/constrained-inference-floor.md:301`, `:567`). `Fracture` ≠ `Reality::Fractured` (H⁰ = 1 with out-degree > 1, versus H⁰ > 1; prism CF2). |
| 2 | **content addressing gives immutability** | prism `MerkleTree` (`merkle.rs:1-5`), `Store::{get, put, has}` (`store.rs:16-31`, no CAS, no ref update), `Sample::buffer_oid` (SHA-256). Git projection `g` COMPUTED by script (math Prop. 7.4, `:470-476`). | BLAKE3 over canonical bytes (decision 5, ratified; math §7.2); atomic ref swap (thesis memory). | **No BLAKE3 and no atomic ref CAS in either repo** (grep). mirror's own `Crystal` address is an **XOR fold**, not a hash: `rust/fractal/src/crystal.rs:115-145`. Its docblock still claims "`crystallize(l1) == crystallize(l2)` iff … identical serialized bytes" (`:106-107`). That is false for an XOR fold (two 33-byte inputs that differ only at bytes 0 and 32 by the same value collide). 22 call sites. prism's `Oid::hash` is "not a cryptographic hash of its input" (math §7.2, COMPUTED). Immutability is a property of the store discipline today, not of the address. |
| 3 | **Anna's math reads without perturbing** | Nothing in code implements an observer. `rotation.rs` cites it in a docblock only (`prism spectral/rotation.rs:56`: "Anna Jakobs 2012 FZJ observation-without-perturbation (LOAD-BEARING)"). | 57 files / 227 lines say "without perturbation" or similar; see §4.1. | Corrections below. The thesis's real non-perturbing observable is **post-hoc FFT on saved per-step snapshots** (p. 42). With immutable roots, that becomes "read at any root, any time". So link 3 holds by construction from link 2, not from Anna's shared-memory pattern. |
| 4 | **frequency math restricts without perturbing** | `spectral::settle` + `Tail` (view, erasure logged); `Instrument::{decay, ring, solo, bleed}` (views); `flux::pillar::steer` (restriction of a *search* by a measure). | `@fate` `restricted_state_space` (symmetry, not spectrum); spectral CSP sparsification (wrong authors); SpectralLiquidInference (stale `dirac.rs`). | Restriction is non-mutating, **not** lossless. Only `ring` is an isometry. No band projector. No spectral object defined on a parse forest. mirror calls none of the prism spectral code (grep over `rust/`). |
| 5 | **Ada: weaving algebraical patterns** | n/a | `shards/loom.mirror:171-174`; loom specs; FLOOR; mirror-algebra §8.6 | Right quote, wrong Note; see §5. |

### §4.1 Anna Jakobs 2012, read from the PDF

Source: `~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`. Page numbers are printed pages (PDF page − 4). Full consent to cite (Alex 2026-10-28).

What the thesis says:
- **Title page:** *Integration von OpenGL-Visualisierungstechniken in GPU-Anwendungen*, **Masterarbeit** des Studienganges Technomathematik, **FH Aachen, Campus Jülich**, August 2012; PGI/JCNS; supervised by Prof. Dr. Martin Reißel and Josef Heinen.
- **Task statement (Aufgabenstellung):** asks for "den gleichzeitigen Zugriff einer OpenGL-basierten Visualisierungs-Anwendung auf die aktuell berechneten Ergebnisse", an OpenCL FFT "für nicht-äquidistante Daten", and a live view of the Fourier transform "um magnetische Phasenübergänge zu erkennen". **These are goals, not results.**
- **p. 29 (Listing 17):** `glFinish` → `clEnqueueAcquireGLObjects` → kernel → `clFinish` → release. The VBO is locked to OpenCL while the kernel runs.
- **p. 29–30:** arrows are not GL primitives, so "mussten zunächst die Spindaten wieder aus dem VBO ausgelesen werden" (`glGetBufferSubData`). The drawing path **copies**.
- **p. 31:** the plan to run visualization and OpenCL in separate threads "konnte … nicht umgesetzt werden". The aim was "während die alten Daten zeitgleich visualisiert worden wären". That aim is double-buffering, and it was not achieved.
- **p. 42:** the FFT is computed "am Ende des Programms" on data "nach jedem der 16 Schritte gespeichert". It is "eine einfache 2D-FFT" with 2ⁿ sampling (p. 35), so equidistant. **Figure 9**, "Spektren der Spindaten nach verschiedenen Schritten" (steps 1, 6, 12, 16): I rendered the page. The images run from a diffuse ellipse (1), through a ring (6, 12), to a few sharp peaks (16). The thesis gives no interpretation. "Held family → choice" is our reading.
- **p. 43:** "Da die Berechnungs- und die Visualisierungsfunktion zum aktuellen Zeitpunkt nacheinander aufgerufen werden, ist ein gleichzeitiger Speicherzugriff nicht möglich". Without that problem, "hätte man das Programm um etwa ein Drittel … beschleunigen können".
- **p. 45:** live FFT after each step is future work ("Interessant wäre eventuell auch …").

Grading the corpus claims (representative; 136 files mention Anna):

| claim | where | grade |
|---|---|---|
| "observation-without-perturbation at shared-memory substrate … by construction" | `shards/glasswall.mirror:30-33`; `docs/specs/2026-09-13-mara-prismqueer-verifies-prismqueer-first-loop-closure-canonical-spec.md:118`; `docs/scouts/2026-09-13-taut-qa-pacts-first-loop-closure-composition-shape.md:81`, `:413` (my own earlier scout) | **OVERCLAIM.** Sequential and step-boundary, with a ~⅓ throughput cost (p. 43). Needs a dated correction. |
| "zero-copy live observation … without perturbing the computation … at the frame-rate of the visualization loop" | `docs/math/2026-07-20-mara-anna-wolf-jspace-alignment-substrate-composition.md:217-232` | Item 4 there ("either OpenCL is writing OR OpenGL is reading, never both") is **correct**. "Zero-copy" is **wrong** for the arrow path (p. 29–30). "Without perturbing" is **wrong** (p. 43). "Live" is **wrong** (p. 31). |
| "Zero-copy VBO … FFT for non-equidistant data enabling real-time phase-transition detection" | `docs/specs/j-space-alignment-substrate.md:45` | **Reads the task statement as a result.** Non-equidistant and real-time were asked for; equidistant, post-hoc 2D FFT was delivered (p. 35, p. 42). |
| Seam REFUTATION #1: "PDF search for 'non-equidistant' and 'nichtäquidistant' returned ZERO matches" | `docs/audits/2026-08-03-seam-phase-d-crown-theorem-arc.md:149` | The conclusion (the FFT is equidistant) **stands**. The search claim is **wrong**: the hyphenated "nicht-äquidistante" appears in the task statement. |
| "b(s) = −s" | `docs/math/2026-07-20-mara-anna-wolf-jspace-alignment-substrate-composition.md:168` | THESIS CLAIM (MT1997 drift term, per Seam's equation check `2026-08-03 … :137-145`). Reading it as "observer drift" is OUR READING. |
| "Diplomarbeit" | `docs/specs/2026-08-26-mara-prismqueer-liquid-floor-canonical-spec.md:16`, `docs/math/qa/proofs.md:44`, `docs/math/FLOOR.md`, ~10 Mara frontmatters | **Wrong degree:** Masterarbeit. |
| "*… in eine hardwarenahe Software*. Diplomarbeit, **RWTH Aachen**" | `docs/math/2026-09-05-mara-loom-editor-…-math-foundation.md:99`; `docs/math/2026-09-05-mara-mirror-algebra-turing-boundary-…-math-foundation.md:77` | **Wrong title, degree and institution.** |
| "the Anna Jakobs 2012 apparatus can observe a System without perturbing its History (weak measurement invariance)" | `docs/math/2026-09-15-mara-prismqueer-truth-…-math-foundation.md:499` | OUR READING presented as the thesis's. The thesis has no weak-measurement content. |
| "ψ = Liquid<T> / flux<T> apparatus" | same two 2026-09-05 docs; `docs/specs/2026-09-09-mara-compose-not-average-…-canonical-spec.md:205` | OUR READING; the type doesn't exist (§1, L3). |
| `prismqueer::spectral::rotation` docblock | `prism@7e6b00f prismqueer/src/spectral/rotation.rs:6`, `:17`, `:56`, `:92`, `:152`, `:202` | Uses "Anna Jakobs 2012 apparatus" for **Fiedler λ₂ via LAPACK**. The thesis has no graph Laplacian. The seam needs to be named: "the apparatus from Anna Jakobs's 2012 thesis, read across". |

---

## §5 Ada Lovelace: the quote is right, the Note is wrong

Primary text: L. F. Menabrea, *Sketch of the Analytical Engine*, with notes by A. A. L., Taylor's *Scientific Memoirs* III (1843), fourmilab transcription https://www.fourmilab.ch/babbage/sketch.html (fetched this session; Gutenberg #75107 agrees). In the transcription, Note A runs from line 999 to Note B at 1667, and Note G starts at 3396.

| passage | actual Note | corpus attribution |
|---|---|---|
| "We may say most aptly, that the Analytical Engine weaves algebraical patterns just as the Jacquard-loom weaves flowers and leaves." | **Note A** (sketch.txt `:1368-1370`) | **Note G** at `shards/loom.mirror:171-174`; `docs/specs/2026-09-05-mara-loom-editor-…-canonical-spec.md:204`, `:712`; `docs/math/2026-09-05-mara-loom-editor-…-math-foundation.md:641`. The quoted words match the original except that the opening "We may say most aptly, that" is dropped without an ellipsis. |
| "…the engine might compose elaborate and scientific pieces of music of any degree of complexity or extent." | **Note A** (`:1232-1235`) | Correct (Note A) at `docs/math/2026-09-08-mara-crystal-fractal-splinter-…-math-foundation.md:325`. **Note G** at `docs/math/FLOOR.md:468` ("Note G 1843 … stated with an example (music)") and `docs/math/2026-09-05-mara-mirror-algebra-turing-boundary-…-math-foundation.md:720` ("(Note G) saw computation-as-music"). |
| "The operating mechanism can even be thrown into action independently of any object to operate upon…" | **Note A** (`:1225`) | Correct in `~/dev/systemic.engineering/practice/insights/cybernetics/bodyminds/ada-lovelace.md:174-176` (protected). |
| "The Analytical Engine has no pretensions whatever to originate anything." | **Note G** (`:3410`) | **Note A** at `…/bodyminds/ada-lovelace.md:170-172` (protected); it introduces the quote as "from Note A". |
| Bernoulli-number program | **Note G** | Correct everywhere I checked. |

Seam's 2026-09-19 audit marks "Ada Lovelace 1843 — Note G" as **VERIFIED** (`docs/audits/2026-09-19-seam-phase-d-floor-eda119b-adversarial-adjudication.md:68`). That verified the source edition, not which Note holds the quote.

Does the thesis's "the compiler begins to weave algebraical patterns" follow from Ada? It is a fair citation of **Note A**. Note A is about the *science of operations* separated from its objects (sketch.txt `:1183-1253`), and that is what a five-op algebra over content-addressed splinters is. The music sentence in Note A is conditional ("Supposing, for instance, that the fundamental relations of pitched sounds … were susceptible of such expression and adaptations"). It is not a claim that computation is music.

---

## §6 Drift list (for whoever folds this)

1. `docs/specs/mirror-fixpoint-parser.md:216`: decision 8 is open. Any sentence saying "ratified decision 8 replaces `/` with split + settle" overstates it.
2. `docs/specs/liquid-types-for-mirror.md:621`, `:996`: the spectral CSP sparsification authors are Khanna, Putterman, Sudan (arXiv 2504.16206).
3. `docs/specs/liquid-types-for-mirror.md:670`: "already implemented in mirror/src/dirac.rs". The file doesn't exist.
4. `docs/specs/liquid-types-for-mirror.md:675-680`: a zero Fiedler gap means disconnected, not incompatible.
5. `docs/math/liquid-types/README.md:19`, `:48`: the spec's date is 2026-05-19 in its own header.
6. `prism@7e6b00f prismqueer/src/flux.rs:1`: the header still says "Liquid".
7. `prism … flux` collides with Jhala et al.'s Flux (liquid types for Rust).
8. `prism … spectral/spectrum.rs:146-168`: `Split`/`Project`/`Focus` eigenvalue newtypes collide with the five op names.
9. `rust/fractal/src/crystal.rs:106-107`: the "iff" claim is false for the XOR fold at `:115-145`.
10. Anna: degree, title and institution errors, plus the five overclaims in §4.1. Seam's "zero matches" search is wrong, though its conclusion stands.
11. Ada: the Jacquard and music sentences are attributed to Note G at five sites; "no pretensions" is attributed to Note A in the protected Ada bodymind note.
12. `docs/math/music/`: README only; the planned `pitch-class-algebra.md` was never written.
13. `rust/src/pact.rs` (the inference engine the parser math leans on) isn't on `reed/ship`.

---

## §7 The line: smallest substrate-honest next step

The corpus supports one move with no new primitive: **state spectral restriction as split-then-settle over a held root, on the code prism already has.**

- `Instrument` gives the operator D and its eigensystem on any graph. `settle.rs` already returns (kept modes, `Tail { moving, faded }`), which is a settle with a declared `Noise` ruler and an erasure record.
- Red test, prism side: for a band ruler `[a, b]` over D's spectrum, `split` gives `(P_[a,b] x, (1 − P_[a,b]) x)`. The pair sums to `x` exactly, so it is injective, which makes it free by math Prop. 4.2. `settle` keeps the band and logs the complement as erasure. The source cochain's address is unchanged. Assertions: lossless as a pair, idempotent (`P² = P`), commutes with `ring` and `decay` (same D), and `bleed` is zero iff the band is a union of `solo`-invariant subspaces. Everything here is already measurable with landed functions except the band projector itself.
- This makes link 4 true in the strong form: **non-mutating by addressing, lossless until `settle`, loss declared by ruler.** It also tells the thesis what "without perturbing" can honestly mean.

What's missing, by owner:

- **Mara (math).** (a) Which complex carries a parse? The candidate is the least-model chart of math §5.1: vertices are atoms `m(e,i,j)` / `f(e,i)`, and edges are rule instances. Does its H⁰/H¹ or λ₂ say anything about ambiguity (CFG choice held open) that the Datalog least model doesn't already say? If it says nothing new, the frequency layer is decoration on the parser and belongs on assertion graphs (where `Instrument` already lives). (b) The answer to Alex's 2026-09-04 question (`loving-lambda-calculus-canonical-spec.md:54`): is "geometric restriction = liquid type" a theorem, a design, or a pun? L1 qualifiers are predicates over values, while spectral bands are subspaces of cochains. The bridge (a qualifier as a projector) has to be defined before anyone claims it. (c) Pick one meaning of "liquid" and rename the other two.
- **Seam (crowbar).** (a) Break "restriction without perturbation" on the gap between non-mutating and lossless. Is any spectral restriction other than `ring` free in Prop. 4.2's sense? (b) Hit the XOR-fold `Crystal` address and the missing CAS before anyone leans on "immutability from content addressing". (c) Check whether `solo`'s mean-of-endpoints action on intervals (`instrument.rs:110-116`) is a projector at all. It isn't idempotent on crossing edges (½ → ¼), so `solo` is an attenuation, not a restriction. That matters if `solo` is read as "geometric restriction".

---

## Sources

- Jakobs, Anna (2012). *Integration von OpenGL-Visualisierungstechniken in GPU-Anwendungen*. Masterarbeit (Technomathematik), FH Aachen, Campus Jülich. Local PDF (consent: Alex 2026-10-28). Pages read: title, Aufgabenstellung, printed pp. 29–31, 35, 41–45.
- Menabrea, L. F., with notes by A. A. L. (1843). *Sketch of the Analytical Engine.* https://www.fourmilab.ch/babbage/sketch.html (fetched); https://www.gutenberg.org/files/75107/75107-h/75107-h.htm (Kagi).
- Rondon, Kawaguchi, Jhala (2008). *Liquid Types.* PLDI. https://dl.acm.org/doi/10.1145/1375581.1375602 (Kagi).
- Lehmann, Geller, Vazou, Jhala (2023). *Flux: Liquid Types for Rust.* PLDI. https://dl.acm.org/doi/10.1145/3591283 (Kagi).
- Khanna, Putterman, Sudan (2025). *A Theory of Spectral CSP Sparsification.* arXiv:2504.16206 (Kagi; arXiv metadata fetched).
