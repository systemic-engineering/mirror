# Taut scout: sheaf cohomology and the switchboard

*2026-10-28. Taut. Read-only. Branch `taut/sheaf-cohomology-scout` off `reed/ship` (`fe86b31`). Prism read at the pinned rev `/Users/reed/.cache/mirror/prism-7e6b00f`. Protected insights are cited by path with short phrases only. `~/.reed/visibility/private/` was not read.*

**Brief (Alex, relayed by Reed):** speed up mirror's checks with a switchboard built on `phone.rs` + `book`. Shards are addressed by `addr`, call each other, and a change re-checks only the affected shards. "The switchboard is probs sheaf cohomology."

## 0. The line, in seven moves

1. **The math for this already exists, and Seam has passed it.** Mara's `docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md` §8 ("Many shards, compiled at once") is the switchboard math. It covers the Čech spectral sequence over a cover by shards (§8.2), the comma among all as Ĥ¹(𝒩; H⁰) computed by union-find on the overlaps (Cor 8.2(b)), the incremental locality bound "the radius of influence is the star of i in the nerve" (Thm 8.4, §8.5), and a candidate early-cutoff key (§8.5). Seam's verdicts are in `docs/audits/2026-10-10-seam-crowbar-difference-and-chord.md:30`: §8.1–8.3 and §8.5 PASS, Thm 8.4 kept (`:177`), and §8.4 narrowed (`:31`, `:158-173`). Nobody needs to start from zero.
2. **The code has no sheaf.** prism's `Graph` (`kleinos.rs:131-134`) holds vertices and edges and nothing else. The "stalks default to trivial" note there (`:123-129`) describes a Phase 2 that hasn't landed: no `Stalker` type exists anywhere in `prismqueer/src`. What is landed is the **constant ℝ-sheaf on a graph**: `Instrument` (D = δ + δ\*, ker D = H⁰ ⊕ H¹, `instrument.rs:1-17`, `:177-221`), `settle` (vertices only: "Cycles (`H¹`) need the edge Laplacian", `settle.rs:17`), `tension.rs` (β₁ = |E| − |V| + c) and λ₂ of D − A (`kleinos.rs:296`). The only implementation of non-trivial restriction maps was `bootstrap/src/sheaf_laplacian.rs`, and it handled scalar weights only, with d > 1 "deferred". It was deleted in `9de3eca` (2026-08-26). The 7e6b00f rename sheaf→graph is honest: what landed was a graph.
3. **Mirror never calls the part of prism that has H¹.** `rust/` uses `graph_from_edges`, `detect_tensions`, `infer_via_rotation`, `Spectrum`/`Dissonance` (`main.rs:101-102`, `love.rs:87-125`). It never uses `Instrument` or `settle`. The one "shard sheaf" mirror builds (`build_shards_sheaf`, `main.rs:255-289`) joins **files in the same directory** into cliques. Edges from `in @X` imports are "Phase 2+" (`main.rs:261`). So mirror has no landed graph of which shards call which. A switchboard has no wiring to read yet.
4. **"Switchboard" in the corpus means the @io boundary, not shards calling shards.** `phone.mirror:11-26`, `phone.rs:1-12` and `book.mirror:14-20` define the switchboard as the operator that patches `@io` calls (fs, git, stdio, socket) through `at_operator`. `book.rs` resolves 8 hard-coded `@peer/*` and `@human/alex` names to `Subject`s (`rust/matrix/src/book.rs:92-125`), not shards, and not by `addr`. Alex's new reading is a real extension, not a rename: it needs a shard book (`@name` → current `addr`) and lines (a declared cover).
5. **Reed's reading is mostly right, with three corrections from the corpus.**
   - (a) **H¹ counts are local too.** For edits to cells only one shard holds, the count of commas among all changes only within the star of the changed shard (Thm 8.4(i)). What is non-local is the *representative* (the harmonic flow), which needs a global fixed point (§8.4 point 3).
   - (b) The Mayer–Vietoris class doesn't come from a "missing preimage". It **restricts to zero on each side** (§3.3 CORRECTED, `:226-229`).
   - (c) Seam showed that alternating local *potential* corrections recover the global harmonic part, comma included (audit `:158-165`, T33). Only flow-side local settles erase it. So "recompute U, cache V, glue" is safe if each local step has the right type.
6. **For a real check graph, the key has to be the shard's interface, not its `addr`.** `action_cache.mirror` keys verdicts on `(spec_oid, target_oid, inputs_oid)`, with `inputs_oid` the dependency closure (`shards/mirror/store/action_cache.mirror:59-80`). That is Bazel's transitive invalidation: one deep edit re-rings everything above it. Star-locality needs early cutoff on what a shard exposes to its lines. Mara's candidate for the constant sheaf is `(b₀(K_i), b₁(K_i), component maps K_σ → K_i)` (§8.5), and its necessity is open (§11.5, R7). No Rust realizes `action_cache` (0 hits for `cache_read`/`cache_write` in `rust/`). The "Voice", which the handoffs say *is* the action-cache key, isn't landed either (prism `Beam → Voice` is still "Next", prism `docs/loop/CURRENT.md:219`).
7. **External prior art matches the move almost exactly, and has been cited since June.** Young, *Sheaf-Cohomological Program Analysis* (arXiv 2603.27015, verified via Kagi and the arXiv abstract) says: "H¹ classifies gluing obstructions … each localized to a specific pair of disagreeing sites", and "Mayer-Vietoris enables compositional, incremental obstruction counting". Reed's `docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.md:19-36` cites it. Note the word *counting*: that matches Mara's Cor 8.2(b) and Thm 8.4. That insight also quotes "100% bug-detection recall" without the 69% precision the abstract states.

## 1. Landed vs. claimed

| Claim | Where claimed | Status | Evidence |
|---|---|---|---|
| Cellular sheaf with stalks + restriction maps | `kleinos.rs:123-129` ("Phase 2+ adds … `Stalker`"); `shards/sheaf.mirror:8-14`; `docs/math/sheaf/laplacian.md:12-15` | **CLAIMED** | no `Stalker` / restriction type in `prismqueer/src` (grep: doc comments only) |
| Sheaf Laplacian δᵀδ with non-trivial restrictions | `shards/mirror/index.mirror:105-111, 150, 231-235` (cites `bootstrap/src/sheaf_laplacian.rs`) | **DELETED** (scalar d=1 only while alive) | `git show 9de3eca^:bootstrap/src/sheaf_laplacian.rs` lines 20-24, 87-90 ("higher fiber dimension … deferred"); deleted `9de3eca` |
| Constant-sheaf Hodge–Dirac operator, ker D = H⁰ ⊕ H¹ | prism `instrument.rs:1-17` | **LANDED + tested in prism** | `prismqueer/tests/instrument.rs:68` `kernel_counts_components_and_cycles`, `:139` `decay_keeps_the_comma`, `:192` `bleed_lives_exactly_on_the_solo_boundary` |
| b₁ certified | comma doc §0.1 | **NOT certified** | `Instrument::kernel_dimension` uses a threshold `ZERO = 1e-9` (`instrument.rs:156-157, 220-224`); `settle` certifies b₀ only (`settle.rs:17`) |
| H¹ "cohomological tension" | prism `tension.rs:14-18, 73-74` | **LANDED as β₁ arithmetic** | graph Euler count, no cochains |
| Mirror consumes H¹ | `fire_d_measure.rs:148-149` ("H¹(F) cocycle-class detection") | **CLAIMED (RED test)** | no `Instrument`/`settle` use in `rust/` |
| Substrate shard graph | `main.rs:255-262` | **LANDED as sibling-directory cliques** | `import` edges "Phase 2+" (`:261`) |
| Comma between = im ∂ (Mayer–Vietoris) | comma doc §3 (`2026-10-01-…:236-262`) | **PROVED + numerically checked**; not in code | Seam audit 2026-10-01 |
| Comma among all = Ĥ¹(𝒩; H⁰); locality = star of i | difference doc §8.2, §8.5 | **PROVED, Seam PASS**; not in code | audit `2026-10-10-…:30, 177` |
| Incremental cutoff key (b₀, b₁, component maps) | difference doc §8.5 | **CONJECTURE** (sufficient for private edits; necessity open) | §11.5, R7 |
| Tests T17–T30 (T28 Čech, T29 incremental, T30 local vs global) | difference doc §10 | **NOT LANDED** anywhere | grep `cech|schwarz|comma among` in prism/mirror code: 0 relevant hits; Mara notes the scripts lived in an ephemeral scratchpad ("Port before it disappears") |
| `@io` switchboard (phone) | `phone.mirror`, `phone.rs` | **LANDED** (fs/git/stdio/socket, property tests) | `phone.rs:14-60` |
| Phone book | `book.mirror`, `book.rs` | **LANDED, 8 well-known names only** | `rust/matrix/src/book.rs:92-125` |
| Action cache | `shards/mirror/store/action_cache.mirror` | **CLAIMED** (shard-decl only) | no Rust realization |
| `active` spectral half composes | `shards/uuid/spectral.mirror` | **REFUTED** | fixpoint math §7.4 item 5 (spectra of unions are multiset unions); `active = 0` in production (spec §8) |

## 2. Definitions and contradictions

### "Sheaf" (five definitions)

1. **Cellular sheaf on a graph, vector-space stalks, linear restrictions** (Curry / Hansen–Ghrist). Defined in `docs/math/sheaf/laplacian.md:12-25` and the comma doc §1.4 (rank-1 twisted). Used in proofs in the comma doc §3.3 and protected `spectral/mirror-relational-compiler.md` Prop 5.3.1 (rank r, holonomy ρ). This is the only definition the 10-10 doc keeps (§8.6 "Sound and used").
2. **Graph called a sheaf.** `Graph` was `SheafOfShardGraph` before 7e6b00f. `main.rs:250-325` still says "substrate-sheaf"/"session-sheaf", and `main.rs:2407-2408` still names the pre-rename functions `sheaf_of_complete_graph_of_order` / `sheaf_of_shard_graph_from_edges` (stale docblock; also flagged in `~/.reed/tasks/handoffs/prism-002.md:39-43`).
3. **Grothendieck site on the Merkle DAG.** Protected `fragmentation/math/sheaf-theory.md` (2026-03-24) puts a topology on content-addressed subtrees, with covers = children that reconstruct the parent. It reads "The eigenvalue is a global section … NoCoincidence is H^1". Of the five, this is the one closest to "shards addressed by `addr`". It conflicts with (1): an eigenvalue isn't a section.
4. **ACL sheaf.** `shards/subject/visibility/sheaf.mirror`, consumed by `shards/io/secrets/sops.mirror:325-353`, models visibility restriction. It has no linear structure.
5. **Meta-trait.** `shards/sheaf.mirror:8-40` (Alex 2026-09-13): "each Frame a Sheaf", with coherence "measured by the sheaf's Fiedler value".

The 10-10 doc §8.6 already lists what *not* to import: non-vector stalks and partial restrictions (07-22 §2, 07-23 §2, Rec #89 §1, 09-13 §1.3); λ₀ read as the spectral gap (`laplacian.md` §2.1, which is still uncorrected at `docs/math/sheaf/laplacian.md:48-64`); "H¹ = coker δ¹" (Rec #89); "K₅ ⇒ H¹ = 0".

### "H¹" / "b₁" (four meanings in live documents)

| meaning | where |
|---|---|
| the **comma**: a disagreement no tuning produces (harmonic edge cochains) | prism `instrument.rs:5-6, 12-14`; comma doc |
| **holes / tasks**: independent cycles of the shard graph | prism `tension.rs:14-18` |
| **reuse** in an AST DAG: b₁(Γ(N)) = Σ(indeg − 1) | fixpoint math Lemma 7.7 (`2026-10-28-…-math-foundation.md:491`); spec §8 makes it the exact integer field of `active` |
| **bugs / gluing obstructions** | `docs/insights/2026-06-07-prophecy-…:28-36` (after Young) |

These meanings are not inconsistent as mathematics, since each is b₁ of a different complex. But a switchboard that reads "b₁ went up" has to say *which complex*. The AST's reuse loops live inside one shard. Commas live in the cover.

### "Comma" (overloaded within one day)

- In the fixpoint math, "Comma 1/2/3" (§1.5, §3, §4) are *open design tensions*.
- A period-p > 1 orbit is "a comma, `\`" (Prop 6.3).
- A hole `\` is the unit of "rerere for commas" (spec §5.1).
- A duplicate JSON name is "a comma" (§8).
- In the comma doc, a comma is an H¹ class.

The bridge between the two senses, "a hole is an H¹ class of some cover", is **not stated anywhere**. If Alex's switchboard is meant to carry rerere resolutions along lines, that bridge is the missing definition.

### Smaller drift

- `shards/mirror/book.mirror:12, 41` cites `rust/src/book.rs`; the file is at `rust/matrix/src/book.rs`.
- `shards/mirror/index.mirror:146-150` cites three deleted `bootstrap/src/*.rs` files as realizations.
- `rust/tests/red_spec_claims.rs:294, 318` gate on "the Voice", which is defined only in the task handoff, not in the repo.
- The comma doc §0.1 cites `instrument.rs` on an unmerged branch (`64442a0`). It has since merged and is present at 7e6b00f.

## 3. Switchboard / phone / book intent

- **phone = @io operator.** It "patches Liquid<T> flowing through Fiber<T> of the Bundle" (`phone.mirror:15`). Callers hand over "the phone number (action_ref like `@io/git.commit`) + args" (`:113-116`). It is "the sole legitimate @io boundary" (`:105`). Older specs agree: `docs/specs/2026-08-06-mara-mcp-serve-composition-shard-canonical-spec.md:576` and `docs/scouts/2026-08-03-taut-mcp-spawn-full-stack-scout.md:318` both list phone.rs as the "@io switchboard (fs + git + socket + stdio)". `shards/reality/object.mirror:68, 245` cites `@io/git.commit` "at the phone.rs switchboard" as a deterministic object instance.
- **book = Registry** (BEAM `{:via, Registry, …}`, `book.rs:8-13`). It holds only well-known @names, and "general-purpose content-addressed OID lookup remains Mara territory" (`book.rs:33-37`; `docs/specs/2026-07-18-peer-registry-oid-resolution.md`).
- **What's new in Alex's ask:** the subscribers are *shards*, and the lines are *shard-to-shard*. No document before today says that. The closest existing design is `@mirror/spectral` (`shards/mirror/spectral.mirror:20-21, 61-66`): voices have authority over **sections** ("shard subtrees under a voice's authority; cross-section writes require an oscillation pause"). That is a cover with owners, the same structure as the comma doc's sites with an owner map (§3.4, Def 4.1). Of everything in the corpus, a section is the closest thing to a "site".
- **Identity tension.** The spec stores and merges on `addr` (§8). But an edit changes `addr`, so a line cannot be keyed by `addr` alone. It needs a stable name (the book's `@name`) that resolves to the current `addr`, with `addr` as the version. This is the same split as the spec's question / occurrence pair (§5.1).

## 4. Incremental prior art (corpus)

- **difference doc §8.4–8.5** (Seam PASS / narrowed): locality = star in the nerve, the cutoff key, local stencil vs. local settle, and tests T28–T30 plus Seam's T33.
- **07-23 fractal-shard Thm 7.2** "mend is supported only at the overlap" (`docs/math/2026-07-23-fractal-shard-sheaf-cohomology-of-inference.md:426-436`). **Superseded** as false by 10-10 §8.6 ("δ of a 0-cochain on U_i hits every U_ij"). Thm 8.4(i) replaces it.
- **07-23 kintsugi** "Each discharge lowers dim H¹ by 1": **superseded** ("Adding an edge never lowers b₁", §8.6). Its per-shard O(|body|) checks with local invalidation are kept.
- **09-08 Crystal → Fractal → Splinter** (`docs/math/2026-09-08-mara-crystal-fractal-splinter-…:182-215, 404-416`) cites Salsa, rust-analyzer and Adapton accurately, but never says what the cutoff key is for a sheaf (10-10 §8.5). Its Thm M2.1 ("same operator at every altitude") is limited by fixpoint math §10: true of the deposit shape, false of the guarantees.
- **Fixpoint parser spec**:
  - Semi-naive evaluation, "only the delta joins" (§4), is incremental within one compile.
  - `undefined` is counted in the start atom's **dependency cone** (§4, decision 15). This is the closest thing the parser has to a locality region today.
  - T3 "free moves don't" change `root`/`dark`. So a comment or layout edit gets address-level cutoff for free.
- **`action_cache.mirror`**: REAPI triple key; Bazel-style closure invalidation (see §0.6).
- **`mosaic-store-cache-invariants.md:511-514, 821-887`**: "the projection's OID-closure IS the cache key for the root". Same closure semantics.
- **Tasks**:
  - `handoffs/scout-spectral-db-index.md:14` notes spectral-db `src/incremental.rs` is "O(changed files), not O(all files)" via per-branch shadow refs. That is a checkpoint, not cohomology.
  - `handoffs/scout-datalog-garden.md:14` names DBSP / Differential Datalog for incremental view maintenance.

## 5. Tasks history

- **Live:** `~/.reed/tasks/handoffs/mirror-ship-2026-10-28.md:33` (item 5, "The switchboard (new, Alex)", which includes Reed's reading and this scout). Its status is open. It sits behind T1, the serde-free pact, `@mq` and `mirror love` in the order there.
- **`handoffs/mirror-composite.md:25`** (#9 Voice: "the Voice is the action-cache key"; status open, scout-witnessed-computation).
- **`handoffs/mara-coherence-knots-play.md:71`**: notes that H¹ and holonomy are not measured in prism.
- **No prior pitch, bet, or abandoned task** on sheaves/cohomology for checks. Grep over `_archive/{pending,next,archive,active,important}`, `shapes/`, `cycle/`, `log/`, `collections/` finds only incidental hits: `_archive/next/spectral-graph-research.md:168` mentions hash-consing plus incremental; `kagi-web-of-state-pitch.md` (unbet) does not concern this. The idea has no task history before today, apart from the June insight.

## 6. Reed's working reading against the corpus

| Reed's clause | Verdict | Source |
|---|---|---|
| Edge consistency (δ⁰, H⁰ global sections) is local | **SUPPORTED** | Cor 8.2(a): H⁰ glues by equaliser, shard by shard; Hansen–Ghrist Prop 8.1 local stencil (as verified in the 10-10 doc's Sources) |
| Only lines incident to the changed shard re-ring | **SUPPORTED for private-cell edits**, as "the star of i in the nerve" | Thm 8.4(i). **Open for edits to shared (overlap) cells** (R7) |
| H¹ obstructions are non-local | **PARTLY CONTRADICTED** | Their *count* is star-local for private edits and computable from overlap components (Cor 8.2(b), Thm 8.4). Their *representatives* need a global fixed point (§8.4.3), though with local messages, and potential-side alternation reaches it (Seam T33) |
| Mayer–Vietoris: U = changed star, V = cached rest, glue at U∩V | **SUPPORTED** as the two-site case of Thm 8.1 | Lemma 3.1 needs **closed** subcomplexes with K_A ∪ K_B = X. Seam's owner/site compatibility condition applies (no single-site edge with both endpoints in I; 10-10 §3.1 citing crowbar §B §3) |
| Commas born in the connecting map | **SUPPORTED, with a correction** | im ∂ = ker(H¹(X) → ⊕ H¹(sides)): the class *restricts to zero* on each side, not "no preimage" (§3.3). A comma between exists only over a shared tonic; under holonomy ρ, dim = dim Fix ρ (comma doc §3.3; protected `mirror-relational-compiler.md` Prop 5.3.1). Constant-sheaf code can't see that case |
| "a disagreement between observers no single site owns" | **SUPPORTED** | 10-10 §3.3: "What is missing is an *owner*: no site holds it." For N ≥ 3: the comma owned by a triple (C₆ in three arcs, §8.3) is invisible to every pair. **Pairwise lines are not enough**: the switchboard must compute the nerve, not just pairwise calls. Seam: "It needs the nerve, the overlap pattern" (audit `:199`) |
| Switchboard ≈ sheaf cohomology | **UNSUPPORTED as stated** | No document defines the sheaf of *checks*: what a stalk over a shard is (its verdict? its exported types?) and what a restriction along a line is. Every theorem above is for the constant ℝ-sheaf, where H⁰ = components. For the constant sheaf that says only "is the graph connected", not "do the checks agree" |

## 7. Smallest next step, and open questions

**Smallest step the corpus already supports** (no new math):
1. Port T28 (Čech: comma among all = Ĥ¹(𝒩; H⁰) via union-find on overlaps; fixtures three-arcs-of-C₆ and theta) and T29 (private edit inside a component leaves the count unchanged; joining components adds +1) as RED tests. Oracles are union-find only, independent of LAPACK. They test the locality bound the switchboard depends on, before anything is wired.
2. Give the switchboard a real cover. Replace sibling cliques with declared `in @X` edges (the forward-promise at `main.rs:261`), or with `@mirror/spectral/section` ownership. Both are already named in the corpus. Without this there is nothing to ring.

**For Mara (math):**
- **M1.** The sheaf of checks. What is the stalk over a shard and the restriction along a line, such that H⁰ = globally consistent verdicts and an H¹ class = a check disagreement no single shard owns? Young's presheaf of refinement types along data-flow morphisms is the external template (arXiv 2603.27015, abstract verified).
- **M2.** The cutoff key for that sheaf, as opposed to the constant one (generalising §8.5), and the overlap-cell case (R7).
- **M3.** The hole ↔ H¹ bridge: is a rerere `\` an H¹ class of some cover, and does "offer by question" become restriction along lines?
- **M4.** Which b₁ (§2 table) a switchboard reports.

**For Seam (crowbar):**
- **S1.** Is star-locality still true once the stalks are non-constant (holonomy can kill a comma non-locally: comma doc §3.3 twisted control)?
- **S2.** Is the action-cache closure key compatible with star-local cutoff, or does it force full upward invalidation?
- **S3.** An `addr`-keyed book against lines that must survive edits.

**For Alex:**
- **A1.** Is a "line" a declared import (`in @X`), a section boundary, or a call observed at run time?
- **A2.** Does the switchboard target today's cost? `docs/scouts/2026-10-28-taut-commit-feedback-loop-under-a-minute.md` §0–§2 finds the commit loop dominated by the macOS first-exec queue, nix re-exec and cargo, not by topology recomputation. Mirror has no shard-level check pipeline yet (the fixpoint parser is Red), so the switchboard pays off after T1, not before.

## Sources

- External (Kagi, 2026-10-28):
  - H. Young, *Sheaf-Cohomological Program Analysis: Unifying Bug Finding, Equivalence, and Verification via Čech Cohomology*, arXiv 2603.27015, https://arxiv.org/abs/2603.27015. The abstract was read via the arXiv API, including the phrase "Mayer-Vietoris enables compositional, incremental obstruction counting".
  - J. Hansen, R. Ghrist, *Toward a Spectral Theory of Cellular Sheaves*, arXiv 1808.01513, https://arxiv.org/abs/1808.01513. Existence confirmed; I re-checked metadata only. Content claims (Prop 8.1) rest on Mara's [read] verification in the 10-10 doc's Sources, as do Curry 1303.3255 Remark 8.2.20 and Bott–Tu Prop 8.5.
  - LEAD (metadata only): H. Young, *SAGA: A Comparison Theorem for Local-to-Global Software Architecture*, arXiv 2608.21458 (via pith.science). It looks directly relevant to a local-to-global check architecture and is unread.
- Corpus: the file:line citations inline above.
