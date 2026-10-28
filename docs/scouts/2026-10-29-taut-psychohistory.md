# Taut scout: @psychohistory against the switchboard

*2026-10-29. Taut. Read-only. This follows up `docs/scouts/2026-10-28-taut-sheaf-cohomology-switchboard.md`, on the same branch. Prism was read at `/Users/reed/.cache/mirror/prism-7e6b00f`. Protected insights are cited by path only.*

**Brief (Reed, relaying Alex: "Point them at @psychohistory"):** read `shards/psychohistory.mirror` and everything around it, read `shards/sheaf.mirror` (which my first scout skipped), and test both against the reading Reed and Alex reached after that scout:
- Three covers:
  - lines = `in @X`, plus a lint that any reference to `@X` needs an `in @X`;
  - voices/sections = whose disagreement a comma is;
  - observed calls (the Voice) = a second sheaf, so declared-vs-observed disagreement is itself a comma.
- Interface stalks: a shard's stalk is its interface; a line's stalk is the part of X's interface that Y uses; restriction is that projection; the cutoff key is the address of the restriction's output.
- Hole vs. comma: a hole is no local section at one site; a comma is local sections that don't glue.

## 0. The line, in eight moves

1. **@psychohistory is a declaration with nothing behind it.**
   - `shards/psychohistory.mirror` declares three carriers (`psychohistory_sheaf`, `possibility_projection`, `future_settlement`: `:297-302, :323-328, :347`), two actions (`project_future :356`, `settle_future :364`), three bilaterals (`:375-399`) and one predicate (`:412`).
   - Every field is `ref` and every body is `{ \ }`.
   - **No shard has `in @psychohistory`.** The only places that mention a consumer importing it are the shard's own prose (`:140-141`).
   - **No Rust names it**: zero hits for `psychohistory` in `rust/` or prism `prismqueer/`.
   - The shard says so itself: species, Rust and the empirical fire are "Forward-promised — NOT landed" (`:429-439`).

2. **The one realization there ever was is gone, and it was a Merkle root, not a sheaf.**
   - `psychohistory_root_from_peer_home` (`git show 9de3eca^:bootstrap/src/lib.rs:5071-5082`) BLAKE3-hashed every file under a peer home into one `psychohistory_root_oid`. It called itself a "Substrate-honest stub for the sheaf-Laplacian Δ_F Rayleigh direction".
   - `contribute.rs:58-67` keyed Fate selectors on that root with an "xorshift stub".
   - It landed GREEN in `e571989` ("fate.bounded_by(psychohistory_sheaf) at Rust runtime"), `07ac55a` and `829148b`, then was deleted with `bootstrap/` in `9de3eca`.
   - `shards/peer.mirror:456-457` still cites it as the source of the `psychohistory` field.
   - So the only thing "psychohistory" ever computed was a content address of a whole state. That is today's `addr` at peer-home scope.

3. **The corpus defines the psychohistory sheaf three incompatible ways.**

   | | base space | stalks | where |
   |---|---|---|---|
   | (A) development history | poset of substrate states under inclusion | typed shards, recognitions, essays, probes | `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` §4.1-4.2 (`:577-650`) |
   | (A′) trajectory graph | `psychohistory_moment`s ("one Pack-tick decision, one banking commit") | moments | `shards/song/narrative.mirror:1045-1099`; spec `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` §2 |
   | (B) corpus graph, multi-altitude | commits, ASTs, narrative fragments, peers… | a **spectral triple** (A, H, D), though Def 1.4 two lines later says "vector space" | `docs/math/2026-08-13-mara-recognition-89-…:58-91` |
   | (C) possibility space | "the space-of-all-possible-@frames" (futures) | "possible future-frame carriers" | `shards/psychohistory.mirror:264-302` |

   - The type itself is declared twice: `type psychohistory_sheaf = ref` at `shards/song/narrative.mirror:1099` (consumed by `shards/fate/tournament.mirror:9, 861`) and a four-field record at `shards/psychohistory.mirror:297`.
   - `narrative.mirror:551-555` still says "`@psychohistory` is NOT landed as a family-root". That has been stale since 2026-09-13.

4. **Is it population and prediction? The corpus refuses that reading, then the 09-13 shard restores it.**
   - The founding insight, §2.6, says of Asimov and Turchin: "they assume observer-decoupling and large-N. The substrate's case is observer-EMBEDDED … and small-N (one substrate; not a population). Asimov's frame doesn't apply" (`2026-06-26-…:284-287`).
   - The same insight, §4.4: "Asimov's `ψ` is *predictive* … The substrate's `ψ` is *constitutive* … A section IS the state, not a prediction about it" (`:766-778`).
   - The fate spec takes a middle line: "a statistical field over trajectory-fibers, not a prediction of individual moments" (`:99-101`).
   - The 09-13 shard then re-imports both of the things the insight rejected: "large-population dynamics as statistically-predictable behavior" (`psychohistory.mirror:90-92, 172-176`) and "possibility-space-of-the-future" (`:9-15, :264-268`).
   - There is **no "crisis", "Seldon" or "prime radiant"** anywhere in mirror docs/shards, `practice/insights` or `~/.reed/tasks` (+`_archive`). The Seldon and Mule references sit only in the insight's Asimov summary (`:110-150`).
   - **Zero `psychohistory` hits in `practice/insights` or `~/.reed/tasks`.** It has no task history.
   - **Population altitude has one declared seat:** `shards/peer/colony.mirror:151-160` gives `colony.psychohistory` as "content-addressed root of the colony's shared psychohistory sheaf … lifted to inter-peer altitude" (K ≥ 2 members).

5. **The founding insight already states the line as `in @X`.** Insight §4.3(b) (`:687-694`) reads: "Two shards `A` and `B` compose iff `B`'s `in <foo>` declarations are satisfied by `A`'s carriers … the GLUING in the opposite direction is what the substrate's `in` discipline does. The restriction map is typed … a B that imports A's carriers is a different section from a B that imports A's actions."
   - That is Reed's line stalk ("the part of X's interface Y uses"), stated four months ago, typed by what kind of thing is imported.
   - My June scout `docs/scouts/2026-06-26-taut-psychohistory-cohomology-scout.md:155` named `@epistemologic/pact/composition_closed` as the existing "do local sections glue" carrier at import altitude.
   - That pact checks the **converse** of Reed's lint: every `in @X` resolves to a declaration (`shards/epistemologic/pact/composition_closed.mirror:3-8`), not that every `@X` reference has an `in`.
   - Its Rust discharge (`count_unresolved_imports`, `128e0d2`) was deleted with bootstrap (no hit in `rust/` today).

6. **Hole vs. comma: the psychohistory corpus mixes them in three ways, and Reed's split sorts them.**
   - **Insight §5.2 (`:853-870`)** defines H¹ as candidate recognitions waiting for a second witness: "local data that satisfies the gluing constraint pairwise but DOES NOT extend to a global section".
     - Under Reed's split, a candidate with one witness is a **hole**: no section at the second site.
     - The gloss also contradicts the gluing axiom, under which pairwise-agreeing sections of a sheaf always glue.
   - **What "fracture" means differs three ways:**
     - `dim H⁰ > 1` (`shards/sheaf.mirror:38, 91`);
     - `H¹ > 0` described as "multiple disconnected future-sections" (`shards/psychohistory.mirror:292-294, 345-346`); disconnection is H⁰, not H¹;
     - no global section at all, λ₀ > 0 (`shards/song/narrative.mirror:1143-1170`).
   - **Narrative's repair rule is a local comma repair:** "renegotiating the restriction maps at the offending edges" named by the Fiedler vector (`narrative.mirror:1163-1170`).

7. **The psychohistory math rests on the parts of Rec #89 that Mara has since disowned.**
   - `psychohistory.mirror:30-32, 97-106` grounds itself in Rec #89, and `:381-389` declares `psychohistory_fiedler_monotone_climb` (λ₂ non-decreasing per rotation tick).
   - Rec #89 §3.3 defines H¹ = coker δ¹ (`:177`). That is ill-typed on a graph, where H¹ = coker δ⁰.
   - Thm 8.1's proof says "dim H¹(F) = dim(⊕_{λ>0} eigenspace(λ))" (`:342`). That is the complement of the kernel, not H¹.
   - Prop 4.3 moves "mass" from H¹ to H⁰ (`:208-214`).
   - Mara's 10-10 doc §8.6 lists all of these as "Not imported": λ₀ contraction under iteration, "H¹ = coker δ¹, and H¹ → H⁰ mass transfer", and "Fiedler strict rise as a law … not in general".
   - The comma doc's Prop 2.7 / Thm 2.8 adds: on a fixed frame no letter widens, and ring is an isometry that fixes ker D. So "rotation raises λ₂" has nothing to act on unless the frame changes.

8. **`shards/sheaf.mirror` declares a cellular sheaf. It doesn't change finding #2 at the code level, and refines it at the declaration level.**
   - **Prose (`:73-91`):** the correct Curry/Hansen–Ghrist definition, with vector-space stalks 𝓕(v), restriction maps 𝓕_{v◁e}: 𝓕(v) → 𝓕(e), δ, L = δᵀδ, and ker L = H⁰.
   - **Types (`:240-244`):** `sheaf_of = { stalks: ref, restrictions: ref, base_graph: ref }`, with `base_graph` "the shard-graph the sheaf lives over". The sheaf carrier `sheaf_laplacian` (`:261-266`) has `h_0`, `h_1`, `lambda_2` fields, all `ref`.
   - **Actions:** `restrict(s, e) -> ref { \ }` (`:299`) and `laplacian(s) -> sheaf_laplacian { \ }` (`:306`). Both are holes.
   - **No cover, no nerve, no overlap type.**
   - **Stalks change meaning between prose and types.** In the prose they are vector spaces. In the types they are "an implementation of the meta-trait" (`:230-232`), and restrictions are "a natural transformation … preserving trait operations" (`:327-331`), mapping vertex stalk to vertex stalk (`:293-297`) rather than vertex to edge as `:78` defines. Mara's 10-10 doc §8.6 flags non-vector stalks ("These are not sheaves") among the material it does not import, citing the 09-13 foundation §1.3.
   - **Its "LANDED primitive" (`:249, 305`) is `@epistemologic/math/sheaf_laplacian`.** That shard's restriction type is scalar `{source, target, weight}` (`shards/epistemologic/math/sheaf_laplacian.mirror:171`), and its Rust body was deleted in `9de3eca`.
   - **Revised finding #2:**
     - *Declared:* yes, twice: scalar restrictions in `sheaf_laplacian.mirror`, and abstract `ref`-typed restrictions in `sheaf.mirror`, with no cover.
     - *Realized in code:* never beyond scalar weights (d = 1), and not at all since 2026-08-26.
     - Today's code is the constant sheaf on a graph (prism `Instrument`, `settle`), which mirror doesn't call.

## 1. Landed vs. claimed

| Claim | Where | Status | Evidence |
|---|---|---|---|
| `@psychohistory` family-root | `shards/psychohistory.mirror` | **DECLARED** (types `ref`, bodies `\`) | `:297-412` |
| any shard consumes it (`in @psychohistory`, `requires psychohistory_witnessing`) | `:140-141` | **NONE** | grep `in @psychohistory` / `requires psychohistory`: 0 outside the shard |
| `project_future` "discharges via `prismqueer::spectral::rotation` LANDED" | `:352-356` | **CLAIMED** | `rotation.rs` exists; nothing connects it to psychohistory |
| `prismqueer::psychohistory` | `:426-434` | **NOT LANDED** (self-declared) | no module |
| psychohistory root of a peer | `peer.mirror:456-461`, `colony.mirror:151-155, 164` | **DELETED** (was a BLAKE3 Merkle root over files) | `9de3eca^:bootstrap/src/lib.rs:5071-5082`; `contribute.rs:58-67` |
| `fate.bounded_by(psychohistory_sheaf)` runtime | `fate/tournament.mirror:814-870` (cites Reed `9cf1e3b`, `Features::default()`) | **DELETED** | `e571989`, `829148b` in history; `Features::default` 0 hits today |
| narrative psychohistory carriers | `song/narrative.mirror:1065-1490` (`psychohistory_from_tray`, `psychohistory_consistent`, `cast_shadow`, `psychohistory_extend_with_mission`, `shadow_faithful`) | **DECLARED**, bodies `\` | grep of `rust/` 0 |
| S4 = psychohistory carrier at `@spectral/db` | `system.mirror:208`, `epistemologic/cybernetic/viable.mirror:164`, `beam/system.mirror:67, 164, 190`, `mirror/spec/system.mirror:259-262` (`s4(intelligence: ref) -> prism { \ }`) | **DECLARED** | no Rust |
| Rec #97 System-4 wire | `psychohistory.mirror:121-131` | **NOT LANDED** | `docs/scouts/2026-09-04-taut-psychohistory-cyberpunk-desired-state-substrate-audit.md:42` |
| cellular sheaf (stalks + restrictions) | `sheaf.mirror:73-91, 240-244` | **DECLARED**; restriction body `\` | see §0.8 |
| sheaf Laplacian body | `epistemologic/math/sheaf_laplacian.mirror:248, 278` | **DELETED** (was scalar d = 1) | `9de3eca` |
| λ₂ monotone climb under rotation | `psychohistory.mirror:381-389`; Rec #89 Thm 8.1 | **REFUTED / not imported** | 10-10 doc §8.6; comma doc Prop 2.7 |

## 2. What @psychohistory implies for Reed's three questions

### 2.1 Which altitude is it?

The corpus offers four seats. None is "a prediction layer over the switchboard" with any realization.

| seat | content | fits Reed's frame as | status |
|---|---|---|---|
| **history** (insight §4, narrative, fate spec) | sections over the trajectory of substrate states and commits | the **observed sheaf over time**: the Voice's beats as stalk values (`narrative.mirror:1047-1052`, "one Pack-tick decision, one banking commit, read as a typed local section"). This is the closest match to Reed's second sheaf | declared; constitutive, *not* predictive (insight `:766-778`) |
| **possibility space** (09-13 shard) | the space of possible @frames/futures | nothing in the switchboard; it would be a sheaf over *hypothetical* edits | declared only |
| **System 4** (VSM shards) | environment scanning and futures at `@spectral/db` | a reader of the switchboard's state | declared only |
| **colony** (`peer/colony.mirror`) | one shared root across K ≥ 2 peers | the **voice cover** at population scale: whose commas, aggregated | declared only; the field is a `ref` |

**What "predicting commas" can mean, substrate-honestly:**
- It doesn't mean statistics. The corpus already has exact, local predictions.
- In the 10-10 doc, Thm 8.4(ii)/(iii): a private edge inside one component of K_i leaves the comma among all unchanged, and a private edge joining two components raises it by exactly 1.
- So the effect of a proposed `in @X` edit on the comma count is predictable before you run it, by union-find on the overlaps.
- That is "deriving fractures from topology" in the sense of `docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.md`, which is a sibling of psychohistory, not part of it.
- If Alex wants `@psychohistory/projection`, its smallest honest body is: for an edit, report Δ(comma among all) from the star of the edited shard.

### 2.2 Interface stalks

- **Agrees:** insight §4.3(b) and `sheaf.mirror`'s meta-trait reading (stalk = an implementation; restriction preserves the trait's operations) both have Reed's shape.
- **Extends:** §4.3(b) types the restriction by *what kind* of thing is imported (carriers vs. actions). Reed's line stalk should carry that type, not just a set of names.
- **Contradicts the Laplacian path:**
  - Interface stalks are sets of typed names, not vector spaces. So δᵀδ, λ₂, Fiedler and `Instrument` don't apply to them (the same objection §8.6 raises against `sheaf.mirror`'s stalks).
  - With set-valued stalks, H⁰ (consistent global sections) is fine. The obstruction side needs Čech cohomology with abelian coefficients (Young counts rank over 𝔽₂, arXiv 2603.27015) or non-abelian Čech.
  - The 10-10 doc §8.6 says 07-22 §5 is the only corpus file that states the triple-overlap cocycle correctly, and "the coefficients there are nonabelian".
  - Everything Mara proved about locality (Thm 8.4) is for the constant ℝ-sheaf. The interface sheaf needs its own locality statement.
- **Cutoff key = address of the restriction's output:** this matches Mara's §8.5 shape (the key is what a shard contributes to its overlaps), moved from (b₀, b₁, component maps) to interface projections. Necessity is open in both versions.

### 2.3 Three covers

- **Line cover (`in @X`) plus Reed's lint, measured on today's corpus.** This is an approximate count (my regex over `shards/**/*.mirror`; `in`/`out`/`source` lines excluded; prefix coverage counted).
  - **27 of 410** shards have an `@X` reference in a non-comment line with no covering `in` and no own declaration. Examples: `shards/tool.mirror` (`@tool/git`, `@kintsugi/roomba`), `shards/loom.mirror` (`@loom/gutter`, `@spectral/signature`).
  - **354 more** have uncovered `@X` references only in comments.
- **Whether prose counts decides whether the lint is usable.** `@psychohistory` is referenced in prose by `sheaf.mirror:115` and `song/narrative.mirror:551`, while `psychohistory.mirror:4` has `in @sheaf`.
  - Counting prose would force `sheaf → psychohistory → sheaf`, a cycle.
  - The "Related shards" blocks are bidirectional by habit (`psychohistory.mirror:224-250`, `sheaf.mirror:183-200`).
  - Recommendation for Mara and Seam: the lint reads non-comment lines only, and prose references are reported, not failed.
- **Voice cover:** psychohistory's colony field (one root per K peers) and `@mirror/spectral/section` are the two declared homes. Neither defines overlaps.
- **Declared vs. observed:** narrative's `psychohistory_consistent` (`:1184`) is a *single* sheaf's H⁰ check. Nothing in the corpus compares two sheaves over one base. Reed's "declared-vs-observed disagreement is itself a comma" is new. The natural form is the Mayer–Vietoris / equaliser of two sheaves on the same base, which nobody has written.

### 2.4 Hole vs. comma

Reed's split resolves three corpus confusions:

| corpus statement | under Reed's split |
|---|---|
| one-witness candidate recognition = H¹ (insight §5.2) | **hole**: no section at the second site |
| "fracture" = dim H⁰ > 1 (`sheaf.mirror:38, 91`) | neither: disconnection, sections exist and are independent (no line to disagree along) |
| "fracture" = H¹ > 0 "multiple disconnected future-sections" (`psychohistory.mirror:292-294, 345`) | the first half is a **comma**; the gloss describes H⁰ |
| no global section, λ₀ > 0 (`narrative.mirror:1143-1170`) | **comma** if every site has a local section; **hole** if some site has none. The bilateral doesn't distinguish them |

The split also matches the fixpoint spec, where `undefined` (no section; spec §4) and `\` resolved by rerere (spec §5.1) are already separate counters.

## 3. Tasks and insights

- `~/.reed/tasks` (including `_archive/`, `handoffs/`, `shapes/`, `cycle/`, `log/`, `collections/`): **0** `psychohistor*` hits.
- `practice/insights`: **0** `psychohistor*` hits.
- Asimov appears once, unrelated: `coincidence/uncertainty-currency.md:151`, "INSUFFICIENT DATA" (*The Last Question*). "Seldon" appears once, an IBM engineer in `spectral-db/cobol-architecture.md:190`.
- Grading my own past: my June scout (`2026-06-26-taut-psychohistory-cohomology-scout.md:155-163`) argued that an abstract `@sheaf`/`@psychohistory` family-root would be "a fourth naming of the same machinery". Both were minted on 2026-09-13 with no realization. Still unrealized seven weeks later, the call holds.

## 4. Smallest next step, and open questions

**Smallest step:**
1. Specify the `in @X` lint over non-comment lines, using the 27 shards in §2.3 as the RED fixture. Rebuild `composition_closed` (the converse check) in the same pass.
2. Before adding any carrier, collapse the two `psychohistory_sheaf` types into one (`narrative.mirror:1099` vs. `psychohistory.mirror:297`), and correct the three "fracture" definitions.

**For Mara (math):**
- **P1.** Which base does @psychohistory live on: history (constitutive), possibilities, or the voice cover? The insight and the shard disagree.
- **P2.** H⁰/H¹ for set- or type-valued interface stalks: abelian 𝔽₂ or non-abelian Čech; and a locality theorem analogous to Thm 8.4 for that sheaf.
- **P3.** Two sheaves over one base (declared, observed): define their disagreement, and say whether it is an H¹ class of some cover or an equaliser.
- **P4.** Retire or repair `psychohistory_fiedler_monotone_climb` (`psychohistory.mirror:381-389`).

**For Seam (crowbar):**
- **S1.** Does the interface-projection cutoff key miss changes that alter a shard's behaviour without altering its interface (pacts with the same signature but different meaning)?
- **S2.** Is "a hole is no local section at one site" well-defined when a site's section is `undefined` (well-founded `opaque`) rather than absent?

**For Alex:**
- **A1.** Psychohistory as *constitutive* history (the June insight) or as *predictive* futures (the September shard)?
- **A2.** Is the colony field (one psychohistory per K peers) the voice cover?
