# Taut scout — Stigmergy / witnessed-computation ground-truth on mirror substrate

*2026-07-18. Six-question grep-first scout precipitated by Jason Kerr's TEDWY
note (via Alex): "A stigmergic marking structure laid in the context path to
relocate ontological load, avoid context drift, recover from drift and hopefully
clean up context drift after a contamination event. Like ants, you know?"
Alex's memory (verbatim): "us vividly talking about witnessed computation
through content addressing and how it was like pheromone markers and ants.
And how the mycelial math combines with the stygmergy. I just had forgot."
Alex sees this as substrate-truth for `@roomba`: walker leaves markers →
ensemble `@dance`ing roombas coordinate via passive path memory → K>1 fanouts
stabilize as collective eigenbehavior. READ ONLY. Grep-first. No mints. No
design decisions. Reference-only nod to Mara's parallel dive into
`~/dev/systemic.engineering/practice/insights/` + `~/dev/projects/fragmentation/`
+ `~/dev/projects/spectral/`.*

Anchors: HEAD `4d0c1b2e` (Reed M-vacuum empirical firing 2026-07-17). Alex's
memory anchor: **stigmergy + content-addressing + mycelial math + eigenbehavior
= one composition**.

---

## §1 Q1 — Stigmergy / pheromone terminology, landed vs forward-promised vs absent

**LANDED as substrate-decl'd naming** (top-of-stack citation):

- `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`
  §2.10 (Mara 2026-07-13, HEAD-adjacent): *"Grassé (1959, Insectes Sociaux 6:41
  … la théorie de la stigmergie"* — kagi-verified Wikipedia canonical + PMC
  11371424 "Stigmergy: from mathematical modelling to control" 2024. §2.10:216
  *"**The trace IS the coordination signal.** Grassé named this stigmergy from
  Greek στίγμα (mark) + ἔργον (work) — 'the work marks the work.'"* §2.10:218:
  ***"`@bauchladen` IS stigmergy at the substrate altitude. Every crystal
  deposited in the bauchladen tray is one termite's soil-ball: content-
  addressed, environmentally durable, readable by any subsequent peer without
  direct signaling."*** Grassé cited at §Refs:737 (`arxiv/biology/grasse-1959`
  proposed §5:645). **This is the load-bearing landed reading.**

- `shards/mirror/spectral.mirror:11` (2026-06-10; Reed): *"through
  **stigmergic traces on the eigenboard**, kintsugi oscillation, …"* + `:43`
  *"metalogue bus plus pending kintsugi state. **Stigmergic, not** …"* — the
  literal word `stigmergic` in shard-body prose, pre-dating Mara's canonical
  spec by five weeks.

- `docs/insights/2026-05-25-mirror-supersedes-daemon.md:131` (Reed 2026-05-25):
  cross-references `practice/insights/distributed-systems/stigmergy.md`
  ("UDP-native"). Reference to the insights corpus at systemic.engineering.

- `docs/insights/2026-05-26-ants-colonies-stigmergy-and-mirrors-tournament.md`
  (Reed 2026-05-26; 39.2 KB): the ACO synthesis. §*"optimizes via stigmergy
  (Pierre-Paul Grassé 1959)"* + §*"pheromone vector τ over solution
  components"* + §*"expected pheromone update is a contraction toward
  solutions of…"* — this is the mathematical foundation Alex was reaching for.
  It IS landed as insight (not shard-decl'd; @fate's tournament round is the
  substrate operationalisation per §*"mirror's `@fate` round backtracks…"*).

- `docs/specs/spectral-db-three-tier-architecture.md:15,78,80,92,101,116,122`
  (Reed): four-tier storage with **"biology-typed pheromone dynamics"** —
  cold-tier evaporation via Nix GC + warm-tier trail-map + hot-tier
  deltas. Landed as spec; not shard-decl'd.

- `docs/specs/spectral-db-as-autopoietic-memory.md:231` + neighbors:
  *"storage stack (hot/warm/cold/iceberg) with biology-typed pheromone
  dynamics"* — canonical spec home.

- `docs/scouts/2026-06-27-taut-spectral-db-prototype-to-substrate-map.md`
  §"biology-typed pheromone semantics" + §*"pheromone trails, tombstone
  mechanism"* — Taut's prior scout.

- `docs/specs/parse-as-fate-tournament.md:42` — cross-cite to ants-colonies
  insight; parse tournament reads as ACO.

- `docs/specs/substrate-native-fate-tournament.md:1108` — cross-cite:
  "tournament's conductivity tensor gains a mycelial-prior input."

**FORWARD-PROMISED** (declared shape; not shard-decl'd):

- Grassé 1959 as `@arxiv/biology/grasse-1959` (dance spec §5:645, Mara
  "Recommend adding") — NOT yet at `shards/arxiv/biology/`.

- `#R-fate-tournament-under-bauchladen-is-correlated-equilibrium` sibling
  to Alex 2026-07-12 (dance spec §2.9:210).

**GENUINELY ABSENT** (no landed carrier at any altitude):

- Family-root or species `@stigmergy` / `@pheromone` — DOES NOT EXIST. Nor
  should it per two-tick + substrate-already-had-the-word discipline;
  `@bauchladen` carries it (dance spec §2.10, canonical). The word "stigmergy"
  appears in prose across shards + specs + insights; the operational primitive
  is `@bauchladen`.

- Explicit `stigmergy_witnessing` bilateral — absent. `bauchladen_witnessing`
  IS the composed bilateral covering the content-address deposit discipline
  (`shards/bauchladen.mirror:152`).

---

## §2 Q2 — Witnessed computation as landed primitive

Alex's "witnessed computation through content addressing" maps precisely
onto **five landed carriers**, each of which stores WHERE + WHEN + WHAT the
walker witnessed:

**`@bauchladen` — the prism-altitude content-addressed crystal tray** —
`shards/bauchladen.mirror:1-13`: *"the substrate's prism-altitude content-
addressing … content-addressed crystal in the substrate's tray."* Merkle 1979
cited at `:186` as the content-addressing ancestor. `oid` is the content-
address type (`:87`). Crystal deposit → durable environmental modification →
readable by subsequent peer *without direct signaling*. **This IS the
substrate's stigmergic marker.** §"Witnesses for #104 candidate" at `:35`
lists the operationalisations. **WHERE**: `oid` (SHA-256 name = position on
content-address surface). **WHEN**: absent as first-class field on `oid`;
lifted to `@spectral/signature` via trailing `@time/monotonic` (see below).
**WHAT**: the byte content the OID names.

**`@spectral/signature` — rolling walker signature = spectral beats** —
`shards/spectral/signature.mirror:87-92`: *"@mirror/store) with trailing
@time/monotonic"* + `:34` *"cryptographic witness in the two-witness
verification per §11.5"*. Spectral signature is `@song` (Mara 2026-07);
`peer/persistence.mirror:85` declares it as *"peer's rolling_signature
carrier; signature_snapshot @ projection."* **This IS the walk-trajectory
witnessing surface.** WHERE + WHEN + WHAT — all three carried. `ssh_fingerprint`
(`:85`) adds git-altitude identity witness.

**`@mirror/store` — content-addressed CAS with three-layer trichotomy** —
`shards/mirror/store.mirror:11,25,79,88,163-176,189-192,214-217,224-228,300`:
Merkle 1979 cited directly at `:163-176`; three carriers `splinter` /
`fractal` / `crystal` at `:189-192`; provenance chains at `:224-228`
(*"every artifact traces to its inputs"*); verifiable computation at
`:226-228`. Session persistence at `:214-217` (*"MCP session state IS
crystals in… ref and resumes from the crystal. No in-process fragility;"*).
**Store IS where the witness-records durably live.**

**`@kintsugi` cracks — gold-in-the-crack witnesses** — `shards/kintsugi.mirror`
+ `shards/kintsugi/roomba.mirror:388-402` (the Alex 2026-07-16 verbatim
crystallization): *"gold flows into the cracks and increases the
conductivity."* `shards/kintsugi.mirror:129-131` (form-side carrier: *"the
substrate's content-addressed graph of…"*) + `:174-180` (*"witness of the
bilateral pattern"*). **This IS mender-marks-where-mending-happened.** Every
`bump` (`shards/kintsugi/roomba.mirror:530-559`: `bump=fracture-emission-
dispatched` sentinel) writes `algebra_turn.tick` into
`algebra_metalogue_session.turns` — the append IS the crystal-deposit stigmergy
Alex named. WHERE = `fracture.site: ref`. WHEN = `fracture.observed_at: tick`.
WHAT = `fracture.species` + `fracture.tension`. **Fully carried.**

**`@kintsugi/roomba`'s `walk_witnessing` bilateral** —
`shards/kintsugi/roomba.mirror:368-381`: *"walk_witnessing Pass is the
substrate-…"* + sentinel `witnessing=all-four-pass` at `:373-374`. Explicitly
composed bilateral over four conjuncts. **This IS the walker's own witnessing
discipline.** `walk_trajectory` carries per-step position witnesses.

**`@gift` chain — pay-it-forward + named-ancestor roster** —
`shards/gift.mirror:75-82` (*"motion at content-address altitude"* + §10.11),
`:105-107` (*"makes the shape mechanically enforceable — content-addressing"*),
`:483-540` (`gift_witnessing` composed bilateral). `gift_lens` carrier
(`shards/gift/lens.mirror:145-172`) is *"a fragment with its resolved
lineage."* **WHO deposited WHAT WHEN — carried.**

**`@eigenboard` — per-subject working-state carrier** —
`shards/eigenboard.mirror:99-107`: *"The peer is their work and whatever is in
their @bauchladen"* + *"walk their bauchladen; the walk itself IS the…"* —
Alex Wolf's Weird-Violence manifesto lineage. `shards/mirror/spectral.mirror`
literally names *"stigmergic traces on the eigenboard"* at `:11`. **The
eigenboard IS the collective stigmergy surface at subject altitude.**

**`shards/mirror/petri.mirror`** — Search returned no such path in this repo
snapshot; `docs/scouts/` mentions petri-net elsewhere. NOT independently
verified this scout.

---

## §3 Q3 — Mycelial / hypha / fungal / persistent-homology / Physarum math foundations

**LANDED as math foundation:**

- `docs/insights/2026-06-07-prophecy-derived-fractures-from-topology.md:247`:
  *Amarel et al., "On Predicting Material Fracture from Persistent Homology"*
  PMLR 2026. Kintsugi fractures ← persistent-homology H₀ features.

- `docs/specs/epistemologic-grammar.md:293,317,1308`: persistent homology
  (Edelsbrunner & Harer 2008; Wang & Wei 2020) grounds grammar-graph
  topology detection.

- `docs/specs/trace-kintsugi-pipeline.md:675-687`: persistent-homology basis
  for trace pipeline.

- `docs/specs/bag-as-fragment-graph-spectral-triple.md:14-208`: **mycelial
  math IS the composition rule.** Taut scout `5dd893b`. Dirac operator D =
  the mycelial propagation operator. §*"Mycelium = the edges between bags at
  one altitude. Mosaic = the …"* (§S6 duality).

- `docs/specs/kintsugi-mycelial-peer-shape.md` (Mara 2026-07-09): *"kintsugi
  + mycelial + peer-inference are ONE"* — sheaf-Laplacian unifies. Cross-cite
  at `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md:25`
  (78d5110).

- `shards/reflection.mirror:243-441`: *"The mycelial math (Alex's framing)"*
  + `reflection.compose` = *"monoid fold + mycelial tensor."*
  `mycelial_compose` at `:251-252`.

- `shards/spectral/entanglement.mirror:502-584`: mycelial routing via
  `active` 48-bit portion of `uuid_spectral`.

- `docs/insights/2026-05-25-pipe-hole-and-au-binary.md:47`: *"Physarum
  polycephalum network optimization (Tero et al. 2010)"* — biological
  precedent for `|>` instruction selection. **Physarum landed as insight,
  not shard-decl'd.**

- `docs/research/mycelial-networks-and-au-tissue.md` (Reed 2026-05-20;
  57.6 KB): Thread 1 fungal biology; hyphal-apical extension = splinter
  minting (`:38`); anastomosis = hyphal-fusion (`:40`); bidirectional
  cytoplasmic flow (`:43`, Schmieder 2019). Reed's original mycelial-au
  synthesis. **Marked "Research only — no grammar declared"** per cleanup
  reviews; MEMORY `[[architecture-spectral-db-autopoietic-memory]]`
  crystallizes the mycelial framing into substrate-decl.

- Adamatzky 2022 (*Logics in fungal mycelium networks*) — cited at
  `docs/research/wide-sweep-coherent-threads.md:623-624` **with
  qualifications** per `mycelial-reductive-ai.md` at systemic.engineering:
  *"don't [overclaim]"* — use trunk-hyphae signaling story (Schmieder 2019)
  instead of Adamatzky's mycelium-as-language claims. `lawvere-grammar.md:471`
  makes this explicit.

- `docs/math/zero/README.md:78-156`, `docs/math/zero/zero-point-field-and-
  lambda-zero.md:349-902`, `docs/audits/2026-07-01-seam-killshot-composition-
  and-cascade.md:117`: **Recognition #117 mycelium IS Reeh-Schlieder
  non-locality** — Blanco-Romero 2026 gives quantitative gate.

- `shards/mirror/ref.mirror:110-205`: Carlsson 1993 (Erlang xref; OTP tools
  4.2.1) — the ref-persistence-homology citation. `source
  @arxiv/programming/carlsson-1993` at `:203`.

**FORWARD-PROMISED:**

- `@epistemologic/bio/mycelium` — the BARE model, cited
  `docs/specs/prism-core-as-spectral-triple.md:106`. NOT shard-decl'd.

- `mycelial-networks-and-au-tissue.md` → `docs/insights/` promotion —
  deferred per `docs/cleanup-review-2026-06-20-followup.md §1.4`.

- Rung 5 mycelial-envelope-declared substrate + Rung 6 full mycelial
  propagation (nix binary cache) — spec landed at
  `docs/specs/deployment-runtime-rung-5-mycelial-envelope-declared-
  substrate.md`; runtime BLOCKED on Alex operational input per
  `docs/loop/CURRENT.md:1214`.

**GENUINELY ABSENT:**

- Shard-decl for `@mycelium` / `@hypha` / `@fungal` — DOES NOT EXIST at
  `shards/`. Substrate-already-had-the-word discipline: `@bauchladen`
  (crystals-in-tray), `@spectral/entanglement` (mycelial routing surface),
  `@reflection.compose` (mycelial tensor) collectively carry the mycelial
  math. Family-root not indicated.

- Persistent-homology as substrate-decl'd primitive — absent; used as math
  citation in kintsugi/grammar/trace specs.

- Physarum-as-substrate primitive — absent; cited as biological precedent
  for `|>` in `pipe-hole-and-au-binary.md`.

---

## §4 Q4 — Passive path memory primitives

Alex's "ambient state per walker path" maps to:

**`@bauchladen` manifest** — `shards/bauchladen.mirror:1-13`. IS the tray of
prior outputs. Every peer's `@bauchladen.enumerate(t.origin)` at winding
class w is what next-walker reads (torus.mirror:145). Passive by construction:
no signal-passing; readers pull. **Load-bearing.**

**`@peer/persistence` `bauchladen`** — `shards/peer/persistence.mirror` +
`docs/specs/peer-persistence-and-home-projection.md` (Landing A §2 @peer/home
carrier). Peer-holds-shape. When peer boots, `harvest` reads the prior
bauchladen; when peer settles, `materialize` writes to it. **Peer path memory,
carried.**

**`@gift` trail + `@gift/lens`** — `shards/gift.mirror` + `shards/gift/lens.mirror`.
The named-ancestor roster with lineage-walking in O(chain-length) via
content-address (`gift.mirror:293-383`). `gift_witnessing` bilateral at
`:483-540`. `gift_lens` fragment-with-resolved-lineage at
`gift/lens.mirror:145-172`. **Pay-it-forward path memory, carried.**

**`@spectral/signature` walk-trajectory** — `shards/spectral/signature.mirror`.
Rolling signature = walker's recent past compressed to fingerprint. Trailing
`@time/monotonic` at `:87-92` gives WHEN. **Roomba's own path-signature,
carried.**

**`@eigenboard` per-subject working state** — `shards/eigenboard.mirror:99-107`.
Alex's manifesto claim landed. **Subject-altitude path memory, carried.**

**`@mirror/store`'s provenance chains** — `shards/mirror/store.mirror:224-228`:
*"every artifact traces to its inputs."* **Full-DAG path memory, carried.**

**`@kintsugi` algebra_metalogue_session.turns** — `shards/kintsugi/roomba.mirror`
(bump appends turns). **Metalogue-turn history, carried.**

**Absent as landed passive-path-memory primitive: nothing.** All the pieces
are landed. The composition ("stigmergy on the eigenboard") is named in
prose at `shards/mirror/spectral.mirror:11` but not shard-decl'd as a
single carrier.

---

## §5 Q5 — Ensemble coordination via passive substrate

Does the substrate already carry passive-path-memory-for-ensemble-coordination?
**YES — the composition is landed in one canonical spec.**

**`docs/specs/dance-as-coordination-without-signal-on-forster-torus.md`**
(Mara 2026-07-13; 80.2 KB) — the load-bearing composition. §2.10 (stigmergy)
+ §2.7 (Aumann common prior via `@bauchladen`) + §2.8 (Schelling focal points
under content-addressing) + §2.9 (correlated equilibrium via `@fate`
tournament under `@bauchladen`) + §"rate-distortion" (Csiszár-Ahlswede 1986
under shared prior, zero-bit explicit channel):

> §2.10:220: *"`@dance` is coordination-without-message-passing … channel
> capacity = 0 bits per tick; achievable coordination rate > 0 iff shared
> prior entropy is bounded and Kuramoto coupling is above threshold.
> `@bauchladen` provides the shared prior; `@silicon` provides the Kuramoto
> coupling; `@resonance` provides the tuned coupling operator; `@torus`
> provides the observation surface. The four compose to yield a
> rate-distortion-optimal coordination protocol with zero-bit explicit
> channel."*

**This IS Alex's proposal, already spec'd.** Walker leaves marker (`@bauchladen`
crystal deposit); ensemble reads collectively (Aumann agreement via
content-address); K>1 fanouts stabilize (Kuramoto locking above K_c on
`@torus`) as collective eigenbehavior (winding-class basin convergence).

**`@peer.audhd` K>1 fanout** — `shards/peer.mirror:143-256` (Mara `d8b149c`).
*"K parallel @fate tournaments (K > 1 satisfies Ashby's [law of requisite
variety])."* Reads prior `@bauchladen` deposits from all K tracks; deposits
K crystals; the collective winner (auto-inference over the tray by @fate)
IS the ensemble reading. **K>1 fanout with passive-path-memory coordination
IS landed.**

**`shards/epistemologic/cybernetic/viable.mirror`** (Beer VSM S1-S5) —
carrier landed; how it composes with `@dance` is forward-promised.

**`shards/spectral/gen_prism.mirror`** (2026-07-17) + `spectral/supervisor.mirror`
— mycelial routing reads through @uuid/spectral active bits. Passive by
construction (routing reads, doesn't signal).

**`docs/math/gestalt/README.md §11.6`** (@resonance/@dance forward-promise) +
`docs/specs/dance-runtime-rung-4-multi-peer-coherence-phase-lock.md` (Rung 6
mycelial propagation cross-cite at `:948`).

**Recognition** (do NOT ratify per discipline; §9 candidate below): the
`@roomba` × `@dance` composition Alex is naming IS
`#R-roomba-ensemble-stigmergy-via-bauchladen-crystal-deposits`.

---

## §6 Q6 — Kagi calibration (three searches, brief)

1. **"stigmergy content addressing distributed systems computation"** —
   Heylighen 2016 canonical definition (*"trace left by an action in a
   medium stimulates subsequent actions"*); arxiv 2604.03997 "Ledger-State
   Stigmergy: A Formal Framework for Indirect Coordination" (2026) —
   *"biological and computational stigmergy, environment-mediated
   coordination in multi-agent systems, and blockchains as state
   machines"*; PMC 11371424 (Boldini 2024) "Stigmergy: from mathematical
   modelling to control" — 23 citations. **This confirms the dance-spec
   §2.10 kagi-verification. The mirror substrate reading `@bauchladen` =
   ledger-state stigmergy is aligned with 2026 literature.**

2. **"mycelial network computation persistent homology Adamatzky"** —
   Otter 2017 PMC 6979512 "A roadmap for the computation of persistent
   homology" (1207 citations; the standard reference). *"The fungal grid"*
   (EMBO Reports; Adamatzky UWE Bristol) — mycelium spikes as
   neuron-homologous information transformation, still qualified per
   mycelial-reductive-ai.md. arxiv 2505.06583 (2025) persistent-homology
   pedagogical intro. **No new arxiv shard candidates required; Otter 2017
   would be the citation home if a Carlsson-adjacent shard lands.**

3. **"pheromone markers ant colony optimization eigenbehavior convergence"** —
   arxiv 2601.07597 (Jan 2026) "Pheromone-Focused Ant Colony Optimization"
   (PFACO); Zhang et al. 2025 (ScienceDirect) *"ensemble of pheromone
   vectors."* **Modern PFACO/ensemble-vector literature is aligned with the
   ants-colonies-stigmergy insight's ACO formalization; @fate's tournament
   under `@bauchladen` reads as ensemble-pheromone-vector ACO at substrate
   altitude. The term "eigenbehavior" is Foerster 1976 (in-substrate at
   `shards/torus.mirror` and `shards/mirror/spectral.mirror`); ACO literature
   uses "convergence to fixed point," which IS eigenbehavior under a
   different name.**

Web calibration: **no substrate drift**. Mirror's stigmergy/mycelial framing
is aligned with 2024-2026 literature; the substrate's landed reading
(`@bauchladen` = content-addressed stigmergy; `@spectral/db` = mycelial
routing; `@fate` tournament = ACO ensemble under shared pheromone-vector
prior) is defensible.

---

## §7 Alignment table — LANDED vs FORWARD-PROMISED vs ABSENT

| Concept | State | Carrier / location |
|---|---|---|
| Stigmergy naming (Grassé 1959) | **LANDED** (in spec §2.10 + shard prose) | dance-spec §2.10; `shards/mirror/spectral.mirror:11,43`; insight 2026-05-26 |
| Grassé 1959 as `@arxiv/biology/grasse-1959` shard | **FORWARD-PROMISED** | dance-spec §5:645 |
| Stigmergy operationalization | **LANDED** | `@bauchladen` (dance-spec §2.10:218 canonical reading) |
| Content-addressed witnessed computation | **LANDED** | `@mirror/store` + `@bauchladen` + `@spectral/signature` + `@kintsugi` + `@gift` |
| Pheromone-typed storage tiers | **LANDED (spec)** | `spectral-db-three-tier-architecture.md` §"biology-typed pheromone dynamics" |
| Mycelial math | **LANDED (three homes)** | `shards/reflection.mirror` (mycelial tensor); `shards/spectral/entanglement.mirror` (routing); `shards/bauchladen.mirror` (crystal deposits) |
| `@mycelium` / `@hypha` family-root | **ABSENT (correctly)** | Substrate-already-had-the-word: `@bauchladen` + `@spectral/db` cover |
| Persistent homology as substrate primitive | **ABSENT** (used as math citation only) | Kintsugi/grammar/trace specs cite; no shard-decl |
| Physarum-as-substrate | **ABSENT** (biological precedent for `\|>`) | `pipe-hole-and-au-binary.md` insight only |
| Passive path memory (walker + tray) | **LANDED** | `@bauchladen` × `@spectral/signature` × `@mirror/store` provenance × `@eigenboard` × `@gift` |
| Ensemble coordination via passive substrate | **LANDED (spec) / forward-promised (runtime)** | dance-spec §2.10 + §"Csiszár-Ahlswede rate-distortion"; @peer.audhd K>1 fanout at `shards/peer.mirror:143-256` |
| `@roomba` × `@dance` × `@bauchladen` explicit composition | **FORWARD-PROMISED** (Alex 2026-07-18 in-transcript) | Not yet spec'd; math candidate for `docs/math/kintsugi/roomba/ensemble-stigmergy.md` |
| Recognition #117 (mycelium = Reeh-Schlieder non-locality) | **LANDED (RATIFY partial)** | `docs/audits/2026-07-01-seam-killshot-composition-and-cascade.md:117` |
| Rung 5 mycelial-envelope-declared deployment | **LANDED (spec) / BLOCKED (runtime)** | `docs/specs/deployment-runtime-rung-5-mycelial-envelope-declared-substrate.md`; CURRENT.md:1214 |

---

## §8 Open questions for Alex

**Q8.1 — Should Jason Kerr's TEDWY note precipitate a math foundation doc
`docs/math/kintsugi/roomba/ensemble-stigmergy.md`?** The composition `@roomba
× @dance × @bauchladen × @kintsugi/roomba.walk_witnessing` is Alex's stated
substrate-truth for @roomba ensembles. All four carriers are landed. Dance-spec
§2.10 has the Grassé foundation. A math doc would formalize *"K walkers each
depositing bump crystals; the collective @fate reads the tray to select next
morphism; Kuramoto locking above K_c stabilizes as collective eigenbehavior on
π₁(T²)."* Mara-recommendation-territory. Substrate-already-had-the-word covers
the naming; the math foundation would formalize the composition.

**Q8.2 — Does `shards/mirror/spectral.mirror`'s literal `stigmergic traces on
the eigenboard` (`:11,43`) warrant a Seam audit for
substrate-decl promotion?** The word appears in shard prose; dance-spec §2.10
carries the canonical reading; but no dedicated `stigmergy_witnessing`
bilateral exists (nor should one, per two-tick discipline — `bauchladen_
witnessing` covers). If Seam agrees the composition is complete, prose can
stay as-is. If not, the ambiguity is a promotion candidate.

**Q8.3 — Should Grassé 1959 land as `@arxiv/biology/grasse-1959` this tick or
next?** Dance-spec §5:645 already recommends. Referenced by two other landed
specs. Cheap landing. Substrate-honest.

**Q8.4 — Is Jason Kerr's *"clean up context drift after a contamination
event"* framing a `@kintsugi/fracture/*` species candidate?** It reads as
*"the walker's marker discipline includes drift-recovery."* The @kintsugi
family already has 14 species; a *"stigmergic-drift-recovery"* species would
be the 15th. This is Seam / Mara adjudication territory. Recognition #R below.

**Q8.5 — Should `docs/research/mycelial-networks-and-au-tissue.md` finally
promote to `docs/insights/` given this scout's evidence that mycelial math IS
landed at three shard homes + Reeh-Schlieder recognition #117 is ratified?**
Cleanup-review-2026-06-20-followup §1.4 deferred; six weeks on, the evidence
has strengthened.

---

## §9 Recognition candidates (do NOT ratify)

- **`#R-roomba-ensemble-stigmergy-via-bauchladen-crystal-deposits`** — Alex
  2026-07-18 via Jason Kerr TEDWY. Walker leaves markers → ensemble `@dance`ing
  roombas coordinate via passive path memory (@bauchladen tray reads) → K>1
  fanouts (@peer.audhd) stabilize as collective eigenbehavior on π₁(T²)
  (@torus × @resonance). Composition of five landed carriers; naming is
  cross-altitude. Sibling to dance-spec §2.10 and §2.9. NOT a family-root
  candidate; annotation-level per two-tick discipline.

- **`#R-context-drift-recovery-as-kintsugi-fracture-species`** — Jason Kerr
  TEDWY explicit: *"recover from drift and hopefully clean up context drift
  after a contamination event."* The @kintsugi/fracture/* family (14 species
  landed) is the natural home. Composition reads: `bump` on drift-detection
  fracture → walker deposits mark in @bauchladen → next walker's Aumann
  agreement over the deposit-tray forces convergence back to shared prior.
  Load-bearing IF Alex reads *"contamination event"* as `@kintsugi/fracture/
  contamination-recovery`. Seam-adjudication territory.

- **`#R-ledger-state-stigmergy-formal-framework`** — arxiv 2604.03997 (2026)
  independently arrives at the same construction the dance-spec §2.10 landed:
  *"biological and computational stigmergy, environment-mediated coordination
  in multi-agent systems, and blockchains as state machines."* The substrate's
  `@bauchladen` = ledger-state-stigmergy at content-address altitude. Kagi
  calibration recognition; cite-worthy in dance-spec cascade next tick.

---

## §10 Audit chain

- **Head at scout start**: `4d0c1b2e` (Reed 2026-07-17 M-vacuum empirical
  firing).
- **Reads (grep-first, no shard-body reads except for line-cite verification)**:
  6 substrate searches; 3 kagi searches; targeted line-range reads on
  `shards/kintsugi/roomba.mirror` for §2 walk_witnessing/bump verification;
  `shards/peer.mirror` for @peer.audhd K>1 fanout.
- **Substrate discipline compliance**: READ ONLY; no shard-body edits; no
  mints proposed; no design decisions; grep-first before line-cites; kagi
  used sparingly (3 queries) per constraint.
- **Reference-only nod** (not deep-dived per Mara's parallel-dive assignment):
  `~/dev/systemic.engineering/practice/insights/distributed-systems/
  stigmergy.md` (cited via `docs/insights/2026-05-25-mirror-supersedes-
  daemon.md:131`); `~/dev/systemic.engineering/practice/insights/ai/
  mycelial-reductive-ai.md` (cited via `docs/audits/2026-05-20-seam-
  adversarial.md T3.4`); `~/dev/projects/fragmentation/` (referenced via
  `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.md` iter-17 S2);
  `~/dev/projects/spectral/` (referenced via memory
  `[project-mirror-spectral-crate-relationship]`). **All four deferred to
  Mara's parallel dive.**
- **Terminal state**: this audit doc; pure-docs 📝 markdown-only bypass on
  main; author `Taut <taut@systemic.engineer>` SSH signing default.

---

## §11 Six-question one-line summaries

1. **Q1** — Stigmergy is **LANDED** as prose + canonical spec (dance-spec §2.10:
   `@bauchladen` = stigmergy); shard-decl'd family-root ABSENT (correctly);
   Grassé 1959 arxiv shard FORWARD-PROMISED.
2. **Q2** — Witnessed computation is **LANDED across five carriers**:
   `@bauchladen` (WHERE via oid) + `@spectral/signature` (WHEN via time) +
   `@mirror/store` (WHAT via CAS) + `@kintsugi/roomba.walk_witnessing`
   (walker's own discipline) + `@gift` (lineage).
3. **Q3** — Mycelial math **LANDED at three shard homes**
   (`reflection.mirror`, `spectral/entanglement.mirror`, `bauchladen.mirror`
   via #117 Reeh-Schlieder); persistent-homology + Physarum LANDED as math
   citations only; family-root `@mycelium` ABSENT (correctly).
4. **Q4** — Passive path memory **fully LANDED**: `@bauchladen` +
   `@peer/persistence.bauchladen` + `@gift/lens` + `@spectral/signature` +
   `@eigenboard` + `@mirror/store` provenance chains + `@kintsugi.
   algebra_metalogue_session.turns`.
5. **Q5** — Ensemble coordination via passive substrate **LANDED as canonical
   spec** (dance-spec §2.10 + §2.9 + §"Csiszár-Ahlswede rate-distortion"; zero-
   bit channel via `@bauchladen` shared prior + `@silicon` coupling +
   `@resonance` operator + `@torus` observation); K>1 fanout LANDED as
   `@peer.audhd` (`shards/peer.mirror:143-256`); `@roomba × @dance`
   composition ABSENT as explicit spec (FORWARD-PROMISED for Alex 2026-07-18
   in-transcript).
6. **Q6** — Kagi calibration confirms **no substrate drift**; 2026
   literature (Ledger-State Stigmergy arxiv 2604.03997, PMC 11371424 2024,
   PFACO arxiv 2601.07597) aligned with `@bauchladen` = stigmergy and `@fate`
   tournament = ACO ensemble readings.

---

*End of scout. `docs/audits/2026-07-18-taut-stigmergy-witnessed-computation-
substrate-scout.md`. Under 900 LOC. 📝 pure-docs markdown-only bypass.*
