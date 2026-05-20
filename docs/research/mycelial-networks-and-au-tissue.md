# Mycelial networks and au-tissue — prior art for `@epistemologic/bio/mycelium`

*2026-05-20. Reed. Research synthesis — not a spec.*

Status: **Research only.** No grammar declared. No types proposed beyond
sketches in §6. The thesis is being tested, not advanced.

Depends on (mirror):
- `docs/specs/au-and-conductivity.md` — `au`, the gold-typed output of Fate
- `docs/specs/eigenboard-representation.md` — the cellular sheaf on the
  five-operation graph
- `docs/specs/epistemologic-grammar.md` — the hierarchy, the `literal`
  property, the `@epistemologic/bio/elegans` template
- `docs/specs/kintsugi-wiring.md` — the kintsugi loop, `e^(n+1) < e^(n)`
- `visibility/protected/practice/insights/coincidence/void-dual-geometry.md`
  (reed-identity) — Splinter ⇄ Narcissus dualities, λ₀ = 0

---

## Why this matters for mirror

The thesis under test, in Alex's words: *"mycelial network biology can become a
sub-grammar of `@epistemologic`, and that sub-grammar gives `au` the substrate
to become tissue."*

`au` today is a single relationally entangled value: gold-typed, content-bound
to one hole in one context (`au-and-conductivity.md`). It conducts or it does
not; if it conducts, it lands in the wire. But a wire is a thread. A codebase
that has been kintsugi'd into being is a *mat* of wires, and the mat needs a
shape. The eigenboard names the shape spectrally — a cellular sheaf on the
five-operation graph, with the sheaf Laplacian's spectrum carrying the
monotonic loss invariant. The mat does not yet have a *growth model*.

Mycelium is, on inspection, the most precise biological prior for what mirror
is doing:

- **Hyphal apical extension at the tip** is structurally identical to the
  kintsugi loop's per-tick advance into a dark region.
- **Anastomosis** — the fusion of two growing hyphae into one network — is
  what happens when two kintsugi resolutions in adjacent grammars converge
  on a consistent type.
- **Bidirectional cytoplasmic flow through trunk hyphae** (Schmieder et al.,
  *Current Biology* 2019) is what conducting `au` values look like once they
  are bound together — not just a static wire, but a wire that pulses
  defensively and nutritively in both directions.
- **The travelling-wave strategy** (Oyarte Galvez et al., *Nature* 2025) is
  the exact dynamics of an indeterminately-growing network that has to
  balance exploration of dark regions against densification of conducting
  ones — the kintsugi loop's central tension.

The Splinter geometry from `void-dual-geometry.md` is the topological match:
mycelial intelligence is structurally Splinter — no central node, no hub,
distributed search, all-pairs reachable, anastomotic, λ₀ = 0 at the ground
state. It is not the star graph (Narcissus) that runs through a Mother Tree;
the popular Mother-Tree framing is in fact under serious criticism in the
peer-reviewed literature (§3). Mirror's bio prior wants the de-anthropomorphised
mycelium: a network that does network things, math-firstly.

That is the case for fit. The rest of this document tests it. Where the prior
art is overhyped, I say so. Where the math does not extend cleanly, I say so.
The verdict is at the end.

---

## Thread 1: Fungal biology

### 1.1 Hyphal architecture

The hypha is a tube. Most fungal growth happens at the tip of each tube
(*apical extension*); the rest of the tube is structural, transporting
material from older regions toward the tip. The organizing centre at the tip
is the **Spitzenkörper** (German "pointed body"), a Pleomorphic body of
secretory vesicles that directs the polarised exocytosis driving cell-wall
extension at the apex.

Key facts (Riquelme et al., *Microbiology Spectrum* 2018; Steinberg, *Eukaryot.
Cell* 2007; Steinberg & Schuster, *Annu. Rev. Microbiol.* 2013):

- The Spitzenkörper continuously consumes vesicles delivered by molecular
  motors (kinesin-1, dynein) along microtubule tracks and actin filaments.
- Turgor pressure provides the mechanical driving force; the SPK provides the
  directional secretion of cell-wall material.
- A hypha that has lost SPK organisation stops extending — apical growth IS
  the SPK working.
- Hyphal extension rates range from ~1 µm/min in slow fungi to ~50 µm/min in
  fast ones; this is the rate-limit on network expansion.

**Branching.** Hyphal branching is a stochastic process governed by local
biochemistry (Du et al., *Dev. Biol.* 2018 review of branching mechanisms;
Lin et al., 2023 prediction paper). Two regimes are documented:

- **Subapical branching**: a new tip forms behind the leading apex, typically
  at a fixed distance scale set by the SPK's exhaustion of local vesicles.
  This is the dominant mode in most filamentous fungi.
- **Lateral / dichotomous branching**: less common, more species-specific.

The branching *angle* is constrained by membrane curvature and tip-tip
avoidance signalling; ~45–60° is typical for the well-studied Ascomycetes,
but Basidiomycetes (which form the cord-producing networks most relevant to
mirror) branch with more variable angles and stronger anastomotic tendencies.

**Anastomosis** is the fusion of two hyphae into a single continuous
cytoplasmic network. It is *not* incidental — it is the structural move that
makes a colony a network rather than a tree. Two converging tips emit
chemotropic signals; on contact, the cell walls dissolve locally; the
cytoplasms merge; a pore opens. The fungus has gone from many trees to one
graph.

Anastomosis is *fungal-controlled*, not environmental. Oyarte Galvez et al.
(2025) show that mycorrhizal anastomosis rates are tuned by the fungus to
hit a target network density, regardless of substrate carrying capacity.
The fungus is doing topology.

Dikec et al. (*Sci. Rep.* 2020, "Hyphal network whole field imaging") give
the cleanest measurements: *Podospora anserina* under controlled conditions
shows roughly constant per-length branching rates and per-collision
anastomosis probabilities, sufficient to parametrise an effective branching
+ fusion process.

**Cited:**
- Riquelme, Aguirre, Bartnicki-García, et al. *Microbiol. Spectr.* 6(1), 2018
  ("Cell Biology of Hyphal Growth"), DOI 10.1128/microbiolspec.funk-0034-2016
- Steinberg, *Annu. Rev. Microbiol.* 67, 2013 ("Tip Growth in Filamentous Fungi"),
  DOI 10.1146/annurev-micro-092412-155652
- Du et al., *Mech. Dev.* 156, 2018 ("Hyphal branching in filamentous fungi"),
  doi 10.1016/j.mod.2018.07.005
- Dikec, Olivier, Bobée et al., *Sci. Rep.* 10, 3131, 2020,
  DOI 10.1038/s41598-020-57808-y
- "One hundred years of the Spitzenkörper: A story in three acts",
  *Fungal Biology Reviews*, 2025 (overview paper)

### 1.2 Mycorrhizal networks — what's substantiated vs popularized

This is where the work gets careful. The popular "Wood Wide Web" /
"Mother Tree" narrative — that forest trees communicate and share resources
through underground mycorrhizal networks under the guidance of large old
trees — is *not* well-substantiated. The 2023 Karst review in
*Nature Ecology & Evolution* is the decisive critique.

**What is well-substantiated** (the mycology, not the mythology):

- Arbuscular and ectomycorrhizal fungi form symbioses with plant roots.
  This has 400+ million years of paleontological evidence and is not in
  dispute.
- A single mycelial network can connect the roots of multiple plants. The
  *existence* of common mycorrhizal networks (CMNs) is real (Selosse et al.,
  *Trends Ecol. Evol.* 2006).
- Carbon does move from one plant to another via mycorrhizal hyphae in some
  controlled laboratory and field settings (Simard et al., *Nature* 1997,
  the foundational isotope-labelling study — DOI 10.1038/41557). The
  *direction* and *biological function* of this flux are contested.
- Mycorrhizal networks alter plant performance in measurable ways — but
  the effect can be positive, neutral, or negative depending on context.

**What is overhyped or unsupported** (Karst et al. 2023, Robinson et al.
2023):

- *"Mother trees preferentially send resources to their kin via the CMN."*
  Karst et al. (2023, *Nat. Ecol. Evol.*, DOI 10.1038/s41559-023-01986-1)
  report: "no peer-reviewed, published evidence" for this claim — the
  citations leading back to it are not load-bearing.
- *"CMNs consistently improve seedling performance."* Out of 28 field
  studies using mesh barriers to isolate the network effect, only 5 found
  positive effects on seedling survival/growth; 5 found negative; the rest
  were neutral or ambiguous.
- *"Forests have a Mother-Tree governance structure."* This personifies a
  process for which there is no mechanism. Robinson et al. (*Trends Plant
  Sci.* 2023, DOI 10.1016/j.tplants.2023.08.010) call this "the perils of
  plant personification" and note that the narrative has run far ahead of
  the evidence.
- *"Positive citation bias has doubled the frequency of unsupported claims
  about CMNs in the literature over the last 25 years."* (Karst et al.).
  The scientific literature itself has drifted, not just the popular books.

**Translation for mirror:** the Splinter framing — flat, all-pairs, no
central node — is the more biologically defensible one. The Mother Tree is
a Narcissus topology (a hub with peripheral nodes treating it as the
source). Building `@epistemologic/bio/mycelium` on the popular narrative
would inherit its citation bias. Building it on the de-personified network
(Boddy, Fricker, Oyarte Galvez et al.) is honest.

**Cited:**
- Karst, Jones, Hoeksema. *Nat. Ecol. Evol.* 7, 501–511, 2023,
  DOI 10.1038/s41559-023-01986-1 — the critique.
- Robinson, Lange, Marshall, Pommerening. *Trends Plant Sci.* 28(11), 2023,
  DOI 10.1016/j.tplants.2023.08.010 — "Mother trees, altruistic fungi,
  and the perils of plant personification".
- Nature news synthesis: d41586-024-00893-0, 2024.
- Simard, Perry, Jones et al. *Nature* 388, 579–582, 1997 — the foundational
  isotope-tracing paper. The data are real; the *interpretive frame* is
  what later work questions.

### 1.3 Fungal electrical signaling — Adamatzky's claims, critically

Andrew Adamatzky's group has produced prolific work on "fungal computing" —
the claim that mycelium generates spike-train-like electrical activity that
constitutes a language. The most-cited paper is *Language of fungi derived
from their electrical spiking activity* (Adamatzky, *R. Soc. Open Sci.* 9,
211926, 2022, DOI 10.1098/rsos.211926).

**What is documented:**

- Extracellular electrodes inserted into fungal substrate or sporocarps
  measure voltage fluctuations.
- These fluctuations occasionally cluster into spike-train-like patterns.
- *Reported* spike durations: 1–21 hours. Amplitudes: 0.03–2.1 mV. Trains
  often grouped into "words" with characteristic length distributions.
- Schizophyllum commune is reported to produce the most "complex" sentences
  by Lempel-Ziv complexity (Adamatzky 2022). Schyck et al. (*Sci. Rep.* 13,
  2023, DOI 10.1038/s41598-023-40163-z) report multiscalar spiking in the
  same species.

**What needs caveats — substantial ones:**

- A 2025 review of fungal electrical signalling (*"Electrical signaling in
  fungi: past and present challenges"*, PMC11995700, 2024–2025) is explicit:
  the methodology behind Adamatzky's spike measurements is *frequently
  criticised for potential artifacts*. Specifically: extracellular electrodes
  on substrate pick up abiotic **Donnan potentials** (junction potentials at
  ion-concentration boundaries) and environmental noise. The reviewers note
  that the "linguistic analysis" rests on the assumption that the observed
  signals are biological — an assumption that has not been independently
  validated with the rigour the claim requires.
- The functional review *"Does electrical activity in fungi function as a
  language?"* (Bicocca et al., *Fungal Ecol.* 68, 101326, 2024) is even more
  pointed: the *speculative leap* from "spikes detected" to "language used to
  communicate and process information" goes far beyond the data. The
  reviewers acknowledge spikes exist; they note that *whether the spikes
  carry information, and to where, is not established*.
- Patch-clamping, the gold-standard for action-potential measurement, is very
  difficult on fungal hyphae because of their small diameter, insulating cell
  walls, and network geometry. Most claims rest on extracellular methods that
  cannot distinguish biological signal from substrate artefact.
- *Faraday cages and genetically encoded indicators are recommended best
  practice* by the 2024 review. Most published spike studies do not yet meet
  that standard.

**Honest takeaway:** electrical activity in fungi is real. Action-potential-
like spikes have been observed by independent groups. The *bit-rate*, the
*information-bearing capacity*, the *propagation network structure*, and the
*role in mycelial physiology* are all open questions. Adamatzky's "Language
of Fungi" is suggestive, not established. A grammar that depends on
fungi-computing-spikes-as-information would be standing on contested ground.

**Cited:**
- Adamatzky. *R. Soc. Open Sci.* 9, 211926, 2022, DOI 10.1098/rsos.211926.
- Adamatzky. *Biosystems* 203, 104373, 2021 ("Electrical activity of fungi:
  Spikes detection and complexity analysis").
- Adamatzky. *Interface Focus* 8, 20180029, 2018 ("Towards fungal computer").
- *Multiscalar electrical spiking in Schizophyllum commune.* Schyck et al.,
  *Sci. Rep.* 13, 12808, 2023, DOI 10.1038/s41598-023-40163-z.
- *"Electrical signaling in fungi: past and present challenges"* (review),
  PMC11995700, 2024–2025 — the methodological critique. This is the
  load-bearing reference for skepticism.
- *"Does electrical activity in fungi function as a language?"*
  *Fungal Ecol.* 68, 101326, 2024 — the functional-claim critique.
- Beasley, Damaschi, Reed et al. *Biorxiv* 2026.03.27.714860 — "Digital
  Twins for Fungal Computing: Viable XOR Regimes" — shows the regimes in
  which fungal substrates can be coaxed into XOR-like behaviour are
  *narrow and specimen-variable*.

### 1.4 Nutrient transport

This is where the biology is solid and the mathematics is well-developed.

Mycelial networks transport carbon, phosphorus, nitrogen, and signals via
**cytoplasmic bulk flow** through specialised hyphae. Two well-studied
mechanisms operate together:

- **Growth-induced mass flow** (Roper, Seminara, Bazant, Tlalka, Heaton,
  Fricker, *PNAS* 2010 / *J. R. Soc. Interface* 2012 / arXiv:1005.5305):
  hyphal expansion at the apex draws fluid through the network. Because
  the cytoplasm is incompressible, growth at the tip *forces* flow from
  proximal regions toward distal tips. Heaton et al. (2010, "Growth-induced
  mass flows in fungal networks") show that cord cross-sectional area
  *adapts* to flow magnitude: cords with high current thicken; cords with
  low current do not. This is the fungal version of Murray's law (§3.1)
  realised dynamically.
- **Peristaltic / contractile flow** (Heaton, Fricker, mycorrhizal fluid
  mechanics 2025, *PMC12489284*): waves of pressure travel along hyphae,
  pumping cytoplasm. In Physarum (slime mold), the contraction is global
  and rhythmic; in mycorrhizal fungi, it is more localised but still wave-
  like.

**Bidirectional flow through trunk hyphae.** Schmieder et al.
(*Current Biology* 29(2), 2019, DOI 10.1016/j.cub.2018.11.058) demonstrate
that *Coprinopsis cinerea* uses specialised "trunk hyphae" to carry
defensive signals AND nutrients in *both directions* (acropetal and
basipetal), with a characteristic ~46-hour oscillation period and *mutually
exclusive* transport modes on a given trunk. The dolipore septa (regulated
pore structures in basidiomycete septa) gate the flow — opening and
closing reversibly. This is precise enough to be a model.

**The travelling-wave strategy.** Oyarte Galvez et al. (*Nature* 638,
1067–1073, 2025, DOI 10.1038/s41586-025-08614-x) is the most important
recent paper. They show that arbuscular mycorrhizal fungi build networks
as **self-regulating travelling waves**: a pulse of growing tips (the
"leading edge") followed by a densifying wake of hyphal filaments. The
wave maintains constant speed and saturation density. Density is
*controlled by the fungus* through anastomosis rate, not by environmental
limits.

They model this with a **BARE process — Branching and Annihilating Range
Expansion** — where wave speed is set by the fastest-growing "puller" tips
at the wavefront, and the trailing density is determined by the rate at
which collisions between tips lead to anastomosis (annihilation events that
fuse rather than terminate). Topology: hierarchical, betweenness centrality
increasing toward the root, geometric transport efficiency stable while
global connectivity rises.

This is the live front-edge of the field. For mirror, BARE is the *target
mathematics*: the kintsugi loop is a branching-and-fusing process on a
graph; the BARE model gives a precise dynamics that exhibits the right
behaviours.

**Cited:**
- Heaton, López, Maini, Fricker, Jones. *J. R. Soc. Interface* 7, 2010
  ("Growth-induced mass flows in fungal networks"). arXiv:1005.5305.
- Oyarte Galvez, Lehnen, Spitz et al. *Nature* 638, 1067–1073, 2025,
  DOI 10.1038/s41586-025-08614-x — the BARE / travelling-wave paper.
- Schmieder, Stanley, Stadler et al. *Curr. Biol.* 29(2), 217–228.e4, 2019,
  DOI 10.1016/j.cub.2018.11.058 — trunk hyphae, bidirectional transport.
- "The Mycelium as a Network", PMC11687498, 2024 — the broad review.

### 1.5 Plasticity and damage response

Mycelium reroutes after damage. The mechanisms are documented
(Boddy & Fricker reviews, *Trans. Br. Mycol. Soc.* / Heaton et al.,
"Analysis of fungal networks," *Fungal Biol. Rev.* 26, 2012):

- **Cord thickening** redirects flow toward high-demand regions.
- **Anastomotic redundancy** means most damage is bypassable: there are
  alternative paths through the network.
- **Septal occlusion** seals cytoplasmic loss at damage sites within
  seconds — basidiomycete dolipores can be plugged with Woronin bodies or
  septal pore caps.
- **New apical growth** initiates from regions distal to the damage,
  re-exploring the lost territory.

Lee et al. (*ISME Communications* 2021, "Network traits predict ecological
strategies in fungi", DOI 10.1038/s43705-021-00085-1) catalogue the
network-trait phenotypes: high-connectivity (phalanx) species pay higher
construction cost for greater fault tolerance; low-connectivity (guerrilla)
species pay less for faster exploration but are fragile. There is a
Pareto frontier between these.

The plasticity timescales: cord thickening over hours, full rerouting over
days, network reorganisation over weeks. These are slow by silicon
standards but very fast by ecological standards.

---

## Thread 2: Slime mold computation

### 2.1 Physarum and the Tokyo subway claim

The most cited result in slime-mold computation is Tero et al., *Science*
327, 439–442, 2010 ("Rules for Biologically Inspired Adaptive Network
Design", DOI 10.1126/science.1177894). They placed food sources at
locations corresponding to Tokyo-area rail stations and grew *Physarum
polycephalum* over them; the resulting plasmodial network had comparable
length, fault tolerance, and transport efficiency to the actual Tokyo rail
network.

**What this does show:**

- *Physarum* produces transport networks that resemble efficient
  human-engineered ones in coarse metrics.
- It does so without central planning — purely through local rules of
  tube-diameter adaptation under flow.
- The mathematical model that captures this (the **Tero-Kobayashi-Nakagaki
  algorithm**, "Physarum Solver") is now an established bio-inspired
  optimisation technique.

**What this does not show:**

- *Physarum* did not find the optimum. The actual Tokyo network has been
  refined over a century by engineers; the Physarum network is similar in
  aggregate properties but not provably optimal.
- The food-source placement is the engineer's choice; the *problem
  specification* was set externally.
- The "shortest path" results (§2.2) are provably correct asymptotically
  but for *the right kind of problem*; on others, Physarum converges to
  suboptimal solutions and gets stuck.

### 2.2 Maze-solving experiments

The foundational paper is Nakagaki, Yamada, Tóth, *Nature* 407, 470, 2000
("Maze-solving by an amoeboid organism", DOI 10.1038/35035159). A plasmodium
of *Physarum* placed in a maze with food at two endpoints, retracts to leave
a single tube along the shortest path between them.

This has been proven to be a correct shortest-path algorithm in an
appropriate model (Bonifaci, Mehlhorn, Varma, *J. Theor. Biol.* 309, 121–133,
2012, "Physarum can compute shortest paths: A short proof"). The model is
an electrical network of resistors whose conductances adapt to current —
exactly the Tero–Kobayashi mass-conservation differential equation. In this
model, shortest-path convergence is provable from continuous-time dynamics
arguments.

### 2.3 Tube-diameter adaptation dynamics

The core equation is:

```
d(D_ij) / dt = f(|Q_ij|) - D_ij
```

where `D_ij` is the conductance of edge `(i,j)`, `Q_ij` is the flux through
it, and `f` is a sigmoidal function. High flux → tube thickens; low flux →
tube atrophies. This is the same form as Murray's law (§3.1) for blood
vessels, derived from a different optimisation principle.

Marbach, Ziethen, Alim (arXiv:2303.01439, 2023, "Vascular adaptation model
from force balance: *Physarum polycephalum* as a case study") derive the
adaptation rule from first principles of viscoelastic tube mechanics. The
adaptation is *not* by-design; it is *forced* by physics + biology.

### 2.4 What slime molds solve well vs poorly

Solves well:
- **Shortest paths** between two known points (Bonifaci et al. 2012).
- **Steiner trees** in 2D (approximate, but good — Sun et al.
  *Sci. Rep.* 12, 2022, DOI 10.1038/s41598-022-18316-3).
- **Fault-tolerant network construction** with cost–efficiency–robustness
  tradeoff (Tero et al. 2010).
- **Approximate optimisation of NP-hard problems** when the cost function
  matches Physarum's intrinsic dynamics (TSP, multi-objective routing).

Solves poorly or not at all:
- Problems with high-dimensional decision spaces (Physarum is intrinsically
  2D/3D embedded).
- Problems where the cost is non-local / non-physically-mappable to flow.
- Anything requiring discrete logic gates with sharp thresholds (continuous
  rheology smears them).

**Honest summary:** Physarum is a *biological gradient descent on an
explicit physical objective*. It is not a general-purpose computer. The
results are robust where the objective can be encoded as a flow-conductance
optimisation; they degrade otherwise.

---

## Thread 3: Mathematical models of growing networks

### 3.1 Murray's law and biological scaling

Murray (1926) derived the scaling law:

```
r_parent^3 = sum_i r_daughter_i^3
```

i.e., the cube of the parent radius equals the sum of cubes of the daughter
radii at a branching point. This minimises the metabolic cost of operation
under Poiseuille flow (viscous loss + tissue maintenance). It applies
exactly to capillaries, approximately to the rest of the vascular tree, and
— remarkably — to fungal hyphal junctions and *Physarum* tubes under their
respective adaptation rules.

Haskovec et al. (arXiv:1908.01197, 2019, "Murray's law for discrete and
continuum models of biological networks") give rigorous proofs that Murray's
law emerges from energy minimisation in both discrete graph and continuum
PDE formulations. The cube-root scaling is not coincidental; it falls out
of the variational principle.

For mirror: Murray's law is the *macroscopic conductance law* that an
adaptive network converges to. If `au` values bind into tissue under a
conductivity-flow rule, the equilibrium structure should satisfy Murray
scaling at branch points. This is a testable prediction, not a postulate.

### 3.2 Steiner trees as the optimization target

A Steiner tree is the minimum-cost network connecting a set of terminal
nodes, allowing additional Steiner points where helpful. The Euclidean
Steiner tree problem is NP-hard.

*Physarum* approximates it (Sun et al. 2022). Adaptive transport networks
in general tend toward Steiner-like structures because:

- Each edge has a cost (length × adaptation maintenance).
- Total flow must connect terminals.
- The Lagrangian admits Steiner-point insertion when angle > 120°.

For mirror: if the conducting `au`-tissue is forced to connect specific
context-bound holes (terminals) and pays a cost per unit of conducting
wire, the equilibrium shape is approximately Steiner. The branching points
where three conducting `au` values meet at 120° angles are Steiner
points — *new* `au` candidates introduced by the geometry, not by the
external task.

### 3.3 Sheaf theory on growing graphs — does it extend?

This is the load-bearing technical question. Mirror's eigenboard is a
*cellular sheaf on the five-operation graph* (Hansen & Ghrist 2019). For
the mycelial metaphor to be a sub-grammar of `@epistemologic` rather than
a metaphor next to it, sheaf machinery must extend to graphs that *grow*.

What is known:

- **Static cellular sheaves** are well-defined (Hansen & Ghrist, *J. Appl.
  Comput. Topol.* 3, 315–358, 2019, "Toward a spectral theory of cellular
  sheaves", arXiv:1808.01513). The sheaf Laplacian, harmonic cochains,
  spectral gap, Cheeger inequalities — all standard.
- **Sheaves on graphs that change over time** are a known active research
  area but not yet packaged. Bressan et al. (arXiv:2402.00206, 2024, "A
  category-theoretic framework for temporal graphs") build a category of
  temporal graphs and show that cosheaves over interval structures can
  define time-varying topological invariants. This is exactly the
  machinery needed, but it is *very recent and not yet textbook*.
- **Persistent sheaf cohomology** is even more recent (Curry, Mukherjee,
  Robinson). It extends persistent homology to sheaf-valued data: track
  how H^0 (consistency) and H^1 (obstruction) evolve as the sheaf changes.
  Defined; not widely implemented.
- **Neural sheaf diffusion** (Bodnar, Di Giovanni, Liò et al. NeurIPS 2022,
  arXiv:2202.04579) shows that sheaf-Laplacian-based diffusion processes
  on graphs can be *learned*: the restriction maps can be parameters, and
  the geometry adapts. This is the closest analogue to a growing sheaf in
  the ML literature.

**Honest assessment:** sheaf cohomology on a *static* graph is solid. Sheaf
cohomology on a *growing* graph — adding nodes and edges over time, with
new restriction maps — is *not* a closed area. Bressan et al. 2024 give the
right categorical framework, but the spectral statements (the
`e^(n+1) < e^(n)` invariant under sheaf growth) are not proven in
generality. They would need to be proven or restricted-to for mirror.

This is a real cost. Sheaf-on-growing-graphs is bleeding-edge mathematics,
not standard tooling. A grammar that *declares* the sheaf-on-growing-graph
shape is making a claim that the math does not yet fully back.

**Cited:**
- Hansen & Ghrist, *J. Appl. Comput. Topol.* 3, 2019, arXiv:1808.01513.
- Bressan et al. arXiv:2402.00206v3, 2024-2025 — temporal graphs as
  categorical objects.
- Bodnar, Di Giovanni, Liò, Lió, Bronstein. *NeurIPS* 2022,
  arXiv:2202.04579 — neural sheaf diffusion.
- Hansen (PhD thesis, "Laplacians of Cellular Sheaves", 2020) — the
  reference for spectral theory of static sheaves.

### 3.4 Reaction-diffusion on networks

The classical Turing pattern formation extends to networks (Nakao &
Mikhailov, *Nat. Phys.* 6, 544–550, 2010; further developed in arXiv:1405.0642):

```
du/dt = D_u * L_u + f(u, v)
dv/dt = D_v * L_v + g(u, v)
```

with `L` the graph Laplacian. Instability of the homogeneous state when
the diffusion ratio crosses a threshold gives rise to Turing-like patterns
on the network — peaked or banded states whose support depends on the
graph's spectral structure.

For mycelium specifically: branching is driven by local biochemistry that
plausibly follows reaction-diffusion dynamics (calcium signalling
cascades, Rho-GTPase activator-inhibitor pairs at the SPK). Aspects of
hyphal branching frequency could be explained as Turing-instability
phenomena.

**The sub-Turing problem.** Mirror is sub-Turing by construction (Spec A,
"strict-and-total classification"). Continuous-time reaction-diffusion on
graphs is generally Turing-complete (it can simulate cellular automata,
which can simulate universal computation). A sub-grammar
`@epistemologic/bio/mycelium` cannot import full reaction-diffusion
dynamics without breaking the substrate constraint. It would need a
discrete, decidable approximation — exactly what the BARE model is, in
fact: branching-annihilating range expansion is decidable per-step.

This is a real constraint on the grammar. The math chosen has to be the
discrete, decidable cousin of the continuous reaction-diffusion. BARE,
Murray-equilibrium, sheaf-Laplacian-spectrum — yes. Turing pattern PDEs —
no, unless they are projected onto a finite-dimensional sub-grammar that
mirror can verify.

### 3.5 Persistent homology of network growth

Persistent homology (Edelsbrunner & Harer 2008) tracks topological features
across a filtration. For a growing network, the *natural filtration is
time*: as new edges and nodes are added, connected components merge
(β₀ decreases) and loops form (β₁ increases). The birth-death pairs of
these features are the **persistence diagram**.

For mycelium: each anastomosis event is a β₀-merge (two components become
one) or a β₁-birth (a loop closes). The persistence diagram of a growing
mycelium has characteristic structure:

- Many short-lived β₀-features (small components that quickly merge).
- A growing number of β₁-features (the loops that anastomosis creates).
- Long-lived β₁-features are the *robust* loops — the structural
  redundancies that keep the network connected under damage.

Sakib (preprint, "Mycelial Harmonic Persistence Index", 2025, ResearchGate
398211470 — *not* peer-reviewed; flagging as preliminary) proposes a
specific index summarising this. The peer-reviewed substrate for this kind
of analysis is the Boddy/Fricker imaging program plus the Lee et al. 2021
network-traits paper, which catalogue the trait phenotypes that
persistence-diagram analysis would summarise.

This thread fits mirror cleanly because `@epistemologic/math/homology`
already exists. The growth model would supply a filtration; the persistence
diagram would be the readout; the spec layer can compare predicted vs
measured diagrams via the `literal` property.

---

## Thread 4: The au→tissue translation

If §1–3 hold, the candidate translation is:

### 4.1 Many au, bound by conducting channels

Today: `au` is one value at one hole. The hole is at a content-addressed
position in the gestalt; `conduct(au, context)` verifies it.

Tissue: a *set* of `au` values plus a *graph of conducting bindings*
between them. Each binding is itself a relation whose presence is a value
(an "edge-au"). The set of `au` values and the set of edge-aus together
constitute a *cellular sheaf section* on the kintsugi graph — exactly the
shape eigenboard already uses for the five-operation graph, now applied
to the larger structure of *all currently-resolved kintsugi holes*.

The natural fit: `au-tissue` is a section of a cellular sheaf on a graph
whose nodes are kintsugi-resolved holes and whose edges are the
conductivity relations between them. The sheaf is the same machinery as
eigenboard, applied to a different (larger, growing) graph.

### 4.2 Growth: hyphal extension as kintsugi tick

Each tick of the kintsugi loop is:

1. Identify a dark region (an unresolved `\` hole — the "search frontier").
2. Fate proposes `au` candidates (the SPK delivering vesicles to the tip).
3. The conductivity check accepts or rejects (the tip extends if the local
   substrate is compatible).
4. If accepted, the new `au` lands; the tissue grows by one node.

The biological correspondence is exact:
- The frontier = dark regions of the eigenboard.
- Apical extension = a tick of the loop, advancing into one frontier point.
- The SPK = Fate's tournament; the vesicles are candidates.
- Tip extension = the conductivity check returning `clear`.

This is the cleanest part of the metaphor. The kintsugi loop is *already*
a hyphal-extension process; naming it as such surfaces what is happening.

### 4.3 Anastomosis: fusing au regions

When two grammars' kintsugi loops independently arrive at compatible
`au` candidates that resolve to the same context-bound type, anastomosis
occurs: the two locally-grown subtissues *fuse* into one. The fusion is
not a fresh inference; it is a *recognition* that two existing wires can
be tied together.

In sheaf language: anastomosis is the identification of two fibers that
restriction maps prove equivalent. The fused fiber has higher
conductivity (lower resistance, larger spectral gap) than either of its
parents had alone.

This is structurally what Oyarte Galvez et al. (2025) measure in
mycorrhizal networks: anastomosis rate is *tuned by the fungus* to hit a
target topology. For mirror, anastomosis rate is something the
@cogito strategy chooses: how aggressively to merge subtissues vs. let
them grow independently.

### 4.4 Resilience: rerouting after a failed conductivity verdict

Bootstrap regenerates with a bug; an `au` that previously conducted now
fails. Today: the kintsugi loop re-proposes for that hole. The tissue
loses a node; downstream conductivity drops along the wires that ran
through it.

With tissue: the loss propagates through the sheaf's restriction maps,
identifying the *downstream* nodes whose conductivity assumptions
depended on the lost node. Rerouting is the discovery of an alternative
path through the existing tissue that does not run through the corrupted
node. The mycelial analogue is septal occlusion + cord thickening: the
local damage is sealed, alternative paths are reinforced.

Sheaf-cohomology language: a corrupted node creates an H^1 obstruction;
the tissue's response is a morphism that nullifies the obstruction by
re-routing flow through unaffected fibers.

### 4.5 Distributed search without a central planner

Physarum solves shortest paths without a planner; the slime mold's
mathematics is gradient descent on a global energy functional implemented
through local tube-conductance updates. The kintsugi loop is exactly this:
no central scheduler decides which hole to fill next; each agent's
@cogito.strategy picks based on its local read of the eigenboard.

The translation: `au`-tissue's growth is gradient descent on a global
conductivity functional, with local updates per-tick. The math is the
same as Physarum's; the substrate is types instead of slime.

### 4.6 Signal propagation between resolved holes

When a hole closes in one region, the news that "this hole closed; the
type now exists; downstream assumptions can update" must propagate. This
is Schmieder et al. 2019's *trunk hyphae* with bidirectional transport:
when a defensive signal arises locally, it travels through specialised
hyphae to other regions, where it triggers gene-expression changes; the
signal is *not* broadcast, it propagates along specific routes with a
~46-hour period.

For mirror: a kintsugi closure event publishes an update along the sheaf's
restriction maps. Downstream sub-grammars receive the news through the
edges that bind their `au` values to the now-closed hole. The propagation
is structured by the sheaf, not by a notification system. The same
machinery that carries `au` values carries `au`-closure events.

This is the model: a propagation that *reuses the existing wires*.

### 4.7 The kintsugi loop AS a mycelial growth process

Putting §4.1–§4.6 together:

- **Each tick is an apical extension.** Fate proposes; conductivity
  accepts; the tip advances by one node.
- **Branching** happens when one resolved `au` enables Fate to propose
  candidates for *multiple* downstream holes simultaneously. The
  branching rate corresponds to the per-tick rate at which one closure
  enables others.
- **Anastomosis** happens when two independently-grown subtissues
  recognise their `au` values match. The merged tissue has stronger
  spectral gap.
- **The wave** — Oyarte Galvez et al.'s travelling wave — is the
  emergent dynamics: a leading front of frontier closures pulls a
  densifying wake of established conductivity bindings.
- **Conductivity = nutrient flow.** The `au` values are the "nutrients";
  they flow through the tissue along the sheaf's edges.
- **The dark region is the search frontier**; it is what the BARE model
  calls the leading edge.
- **The tissue is the cumulative resolved structure**; it is what the BARE
  model calls the saturated wake.
- **`e^(n+1) < e^(n)`** is exactly the statement that the wave moves
  forward, not backward. Spectral gap of the sheaf Laplacian increases
  monotonically.

Reading this list, the fit is tight enough to be more than a metaphor.
The kintsugi loop *is* a branching-and-fusing range expansion on a
cellular sheaf. The mycelial biology supplies the language to *name what
is already happening*.

---

## Thread 5: Where the fit breaks

The synthesis above is favourable to the thesis. Now the load-bearing
skepticism.

### 5.1 Continuous vs discrete

Mycelium is *continuous matter*. Hyphal extension is continuous in space;
flow is continuous; reaction-diffusion is continuous. The mathematics that
describes mycelial dynamics most precisely (Murray equilibrium, viscoelastic
tube mechanics, reaction-diffusion PDEs, BARE-continuum) is all PDE-based.

`au` is discrete. Each value is a content-addressed object; conductivity is
verified once per hole; the kintsugi loop is a discrete tick.

The discretisation cost: a sub-grammar that imports the BARE model as
literal mathematics must discretise it. The discrete version of
branching-and-annihilating range expansion is a Markov branching process
(§3.4), which is decidable and sub-Turing-friendly. But the
*conservation laws* (mass-flow incompressibility, viscosity) do not survive
discretisation cleanly. There would be loss in the translation, not just
in the mapping.

This is real. Mirror would inherit the *combinatorial* aspects of mycelium
(branching, fusion, network topology) but not the *fluid-mechanical* ones
(growth-induced mass flow as the prime mover of nutrient transport).
Whether that is a fatal loss depends on whether the fluid mechanics is
load-bearing for the metaphor or scenic.

**My read:** the fluid mechanics is scenic for what mirror needs. What
matters is that the network grows by branching and fuses by anastomosis,
that flow magnitude regulates edge thickness, and that the topology has
the right spectral properties. All three survive discretisation.

### 5.2 Sub-Turing constraint vs reaction-diffusion

Reaction-diffusion on a graph is Turing-complete (in the limit; finite-time
behaviour can be sub-universal). Mirror is sub-Turing by spec.

A grammar `@epistemologic/bio/mycelium` cannot import reaction-diffusion
literally. It can import:
- Discrete branching processes (decidable per-step).
- Sheaf Laplacian computations (linear algebra; polynomial-time).
- Murray-law equilibrium computations (closed-form).
- BARE model dynamics (decidable per-step under finite branching).
- Persistence-diagram tracking (polynomial-time for finite filtrations).

It cannot import:
- General PDE simulation.
- Calcium signaling cascade dynamics.
- Continuous Turing pattern formation.

This is a *real constraint*. The biology in §1.3 (electrical signaling)
and parts of §1.4 (peristaltic flow with rhythmic oscillations) are
continuous dynamics that the substrate cannot honour as math; they could
appear in the grammar only as *typed observations* with `\` bodies, never
as decidable operations.

### 5.3 Hype vs substance in fungal computing

As §1.3 documented, fungal-computing-as-language is contested. If the
grammar makes IS-claims like *"fungal electrical spike trains ARE
information transmission"*, the `literal` property would have a hard time
returning `pass` — the measurement evidence is not strong enough.

Adamatzky's work is *suggestive*. It is not rigorous-enough to anchor an
epistemic IS-claim in the same way the C. elegans connectome anchors
`Fiedler IS the body axis`. The C. elegans claim survives because the
measurement is direct and independently replicated. The fungal-language
claim has not yet survived independent rigorous replication with the
methodological controls the field acknowledges as necessary.

Grammar implication: a `@epistemologic/bio/mycelium` grammar should avoid
electrical-signaling IS-claims for now. The signaling thread can enter as
*"trunk-hyphae bidirectional flow IS the bidirectional propagation
substrate"* — which is well-substantiated (Schmieder et al. 2019) — but
not as *"fungal spike-trains IS a language"*.

### 5.4 What sheaf theory does and doesn't extend to

Restating §3.3 sharply. Sheaf cohomology on a *static* graph is solid.
Sheaf cohomology on a *growing* graph is recent research, not packaged
mathematics. The categorical framework exists (Bressan et al. 2024). The
spectral statements — monotonicity of the Fiedler value of the sheaf
Laplacian as the sheaf grows — are not proven in the generality mirror
would need.

This is the *single biggest mathematical risk* in the proposal. A grammar
that claims `e^(n+1) < e^(n)` holds for the *growing* sheaf is making a
claim the literature does not yet back. It either:
- (a) Restricts to graph growth modes for which monotonicity can be proven
  (e.g., adding nodes with carefully-conditioned restriction maps); or
- (b) Treats `e^(n+1) < e^(n)` as an empirical observation, not a theorem,
  for the growing case; or
- (c) Develops the math first, then declares the grammar.

Each has cost. (a) restricts expressiveness. (b) weakens the
"the math forces improvement" pitch. (c) is the cleanest but expensive.

### 5.5 The Mother-Tree problem

The popular narrative is wrong (§1.2). A grammar that imported the
Mother-Tree framing would inherit citation bias. The de-personified,
network-first framing (Boddy, Fricker, Oyarte Galvez, Karst critique) is
the honest substrate.

This is not actually a technical problem; it is a *narrative discipline*
problem. The grammar must not claim more than the biology shows. The
travelling-wave / BARE model is what the biology shows. Mother Trees and
forest communication are not.

### 5.6 Specimen variability

Beasley et al. (*bioRxiv* 2026.03.27.714860) — "Digital Twins for Fungal
Computing: Viable XOR Regimes" — note that fungal substrates are
*specimen-to-specimen variable*: the same species under the same
conditions can give different electrical readouts on different attempts.
This is not just experimental noise; it is *biological variability*.

For mirror, this is fine — `au` is supposed to be relationally entangled
with context, and biological variability is exactly that. But it means
the grammar should not promise *reproducibility* in the silicon sense. The
math is reproducible; the bio metaphor is *behavioural*, and the behaviour
varies.

---

## Synthesis — what `@epistemologic/bio/mycelium` might declare

Sketch only. Not a spec. The actual spec is downstream.

```mirror
in @epistemologic
in @epistemologic/math/sheaf
in @epistemologic/math/homology
in @epistemologic/math/tropical

grammar @epistemologic/bio/mycelium {
  # mycelium grows by branching apical extension and fuses by anastomosis.
  # the kintsugi loop IS this process on the cellular sheaf of resolved au.

  type tissue        = sheaf_section                    # current state
  type frontier      = [hole_oid]                       # dark regions
  type tip           = (hole_oid, au)                   # candidate at frontier
  type anastomosis_event(left: hole_oid, right: hole_oid, fused: hole_oid)

  # apical extension: one tick of the kintsugi loop IS one tip advancement
  zoom extend(frontier, tissue) -> (tip, tissue) { \ }

  # anastomosis: when two au values resolve to the same context, fuse
  zoom anastomose(left: au, right: au) -> au { \ }

  # the bare model: branching-and-annihilating range expansion
  # parameters: branching rate, anastomosis rate, tip speed
  type bare_state(wave_position, density, tip_count)

  # the murray law at branch points: r_parent^3 = sum r_daughter^3
  # applied to conductivity bandwidth rather than physical radius
  property murray_equilibrium(tissue) -> verdict { \ }

  # the persistence diagram of the growing tissue
  # tracks beta_0 (connected components) and beta_1 (loops)
  focus persistence(tissue) -> persistence_diagram { \ }

  # the literal property: does the declared identity hold under measurement?
  # specifically: does the kintsugi loop's growth IS the bare wave dynamics?
  property literal(extend) -> verdict { \ }
  property literal(anastomose) -> verdict { \ }
}

out tissue
out frontier
out extend
out anastomose
out persistence
```

**The `literal` claim.** The grammar would claim:

- *kintsugi extend IS hyphal apical extension at the tip of a Spitzenkörper*.
  Measurement: do successive ticks of kintsugi show the same statistical
  signature as fungal tip-extension growth (constant velocity, occasional
  branching, occasional fusion)?
- *anastomosis IS sheaf-fiber identification under restriction-map
  compatibility*. Measurement: when two independently-grown subtissues
  fuse, does the sheaf Laplacian's spectral gap increase by the predicted
  amount?
- *the murray equilibrium IS the steady-state conductivity allocation*.
  Measurement: at branch points where one tissue feeds multiple downstream,
  does the conductivity ratio follow the cube-root scaling?

These are real, measurable claims. They can pass or fail. They give the
grammar epistemic teeth.

**What the grammar should not claim** (deliberately omitted from the
sketch):

- It should not claim that fungal electrical signaling IS information
  transmission. The peer-reviewed evidence does not support that IS-claim.
- It should not import the Mother-Tree narrative — there is no
  central-node bias in the math, and the popular framing is critiqued
  in §1.2.
- It should not claim full reaction-diffusion dynamics. Sub-Turing.

### Grammar cost in machinery

Concretely, what would adding `@epistemologic/bio/mycelium` cost?

- Three to five new types (tissue, frontier, tip, anastomosis_event,
  bare_state), all sheaf-flavoured.
- Three actions (extend, anastomose, persistence), bodies `\` per the
  usual pattern.
- Two property declarations (`murray_equilibrium`, `literal`).
- One import of `@epistemologic/math/sheaf` (existing).
- One import of `@epistemologic/math/homology` (existing).
- One import of `@epistemologic/math/tropical` (existing — relevant for
  shortest-path conductivity calculations).
- Possibly a new sub-grammar `@epistemologic/math/sheaf/growth` if the
  growing-sheaf machinery does not already exist; this is the math debt
  flagged in §5.4.

**Estimated complexity:** comparable to `@epistemologic/bio/elegans`
(currently a single-page grammar) plus the math debt of growing-sheaves.
The bio part is cheap; the math debt is the load-bearing cost.

---

## Open questions for the spec author (Alex)

1. **Is `au`-tissue a section of the eigenboard sheaf, or its own sheaf?**
   The eigenboard sheaf is on the 5-operation graph (5 nodes). The
   `au`-tissue lives on a much larger graph: all currently-resolved
   kintsugi holes plus their conductivity edges. These are two distinct
   sheaves at different scales. Is the relationship between them a
   refinement (eigenboard zooms into tissue) or a separate concern
   (eigenboard is the meta-state; tissue is the object-state)?

2. **How aggressive should anastomosis be?** Oyarte Galvez et al. show
   that real mycelium tunes anastomosis rate to hit a target topology.
   For mirror, this rate is the @cogito strategy's choice: merge
   subtissues aggressively (high anastomosis, denser tissue, more
   redundancy, slower growth) or let them grow independently (low
   anastomosis, faster exploration, less robustness). Where in the
   eigenboard does this rate live?

3. **Is the BARE model decidable enough?** §3.4 / §5.2 argues that
   discrete BARE is sub-Turing. Verify: can the per-tick wave-front
   advance be computed in finite time without simulating reaction-
   diffusion? My reading: yes, because BARE in discrete form is just
   branching + fusion + diffusion on a finite graph per step. But this
   needs a closer look from the model-checker's side.

4. **Does the sheaf-on-growing-graphs math need to land first, or can
   the grammar be declared with the math debt acknowledged?** Either:
   (a) Develop or wait for sheaf-on-growing-graph spectral results to be
   published in the form mirror needs, then declare the grammar; or
   (b) Declare the grammar with the math debt explicit in a "out of
   scope" section, treating monotonic loss decrease as empirical pending
   theorem. The second is faster but pushes the rigour-debt downstream.

5. **What about the failure mode where the wave stalls?** Oyarte Galvez
   et al. observe travelling waves with *constant* wave speed in their
   experimental conditions. In a real codebase, the kintsugi front may
   *not* always advance — some dark regions may be irreducible. The
   biology's analog: nutrient depletion or hostile substrate. The
   grammar's analog: harmonic component of the sheaf decomposition (the
   irreducible obstruction). How should the grammar surface "the wave
   has stalled at a harmonic obstruction" as a distinct state from
   "the wave is still advancing"?

---

## Key references

Peer-reviewed primary literature. URLs verified during research; if a
paper has a DOI, the DOI is given.

**Fungal biology — hyphal growth and architecture:**
- Riquelme et al., *Microbiol. Spectrum* 6(1), 2018, "Cell Biology of
  Hyphal Growth", DOI 10.1128/microbiolspec.funk-0034-2016.
- Steinberg, *Annu. Rev. Microbiol.* 67, 2013, "Tip Growth in Filamentous
  Fungi", DOI 10.1146/annurev-micro-092412-155652.
- Du et al., *Mech. Dev.* 156, 2018, "Hyphal branching in filamentous
  fungi", DOI 10.1016/j.mod.2018.07.005.
- Dikec et al., *Sci. Rep.* 10, 3131, 2020, "Hyphal network whole-field
  imaging", DOI 10.1038/s41598-020-57808-y.
- "One hundred years of the Spitzenkörper", *Fungal Biology Reviews*
  2025 (S1087184525000775).

**Mycorrhizal networks — substantiated and critiqued:**
- Simard et al., *Nature* 388, 579–582, 1997, "Net transfer of carbon
  between ectomycorrhizal tree species", DOI 10.1038/41557.
- Selosse et al., *Trends Ecol. Evol.* 21(11), 2006, "Mycorrhizal networks:
  les liaisons dangereuses?", DOI 10.1016/j.tree.2006.07.003.
- Karst, Jones, Hoeksema, *Nat. Ecol. Evol.* 7, 501–511, 2023, "Positive
  citation bias and overinterpreted results lead to misinformation",
  DOI 10.1038/s41559-023-01986-1. **THE CRITIQUE.**
- Robinson, Lange, Marshall, Pommerening, *Trends Plant Sci.* 28(11), 2023,
  "Mother trees, altruistic fungi, and the perils of plant personification",
  DOI 10.1016/j.tplants.2023.08.010.
- *Nature* news synthesis 2024: "The 'Mother Tree' idea is everywhere — but
  how much of it is real?", DOI 10.1038/d41586-024-00893-0.

**Fungal electrical signaling — claims and critiques:**
- Adamatzky, *R. Soc. Open Sci.* 9, 211926, 2022, "Language of fungi",
  DOI 10.1098/rsos.211926.
- Adamatzky, *Biosystems* 203, 104373, 2021, "Electrical activity of fungi:
  Spikes detection and complexity analysis", DOI 10.1016/j.biosystems.2021.104373.
- Adamatzky, *Interface Focus* 8, 20180029, 2018, "Towards fungal computer",
  DOI 10.1098/rsfs.2018.0029.
- Schyck et al., *Sci. Rep.* 13, 12808, 2023, "Multiscalar electrical
  spiking in Schizophyllum commune", DOI 10.1038/s41598-023-40163-z.
- "Electrical signaling in fungi: past and present challenges", PMC11995700,
  2024–2025. **THE METHODOLOGICAL CRITIQUE.**
- "Does electrical activity in fungi function as a language?",
  *Fungal Ecol.* 68, 101326, 2024.

**Mycelial nutrient transport and the travelling wave:**
- Heaton et al., *J. R. Soc. Interface* 7, 2010, "Growth-induced mass flows
  in fungal networks", arXiv:1005.5305.
- Schmieder et al., *Curr. Biol.* 29(2), 217–228.e4, 2019, "Bidirectional
  propagation of signals and nutrients in fungal networks via specialized
  hyphae", DOI 10.1016/j.cub.2018.11.058.
- Oyarte Galvez, Lehnen, Spitz et al., *Nature* 638, 1067–1073, 2025,
  "A travelling-wave strategy for plant–fungal trade",
  DOI 10.1038/s41586-025-08614-x. **THE BARE-MODEL PAPER.**
- Lee et al., *ISME Communications* 1, 6, 2021, "Network traits predict
  ecological strategies in fungi", DOI 10.1038/s43705-021-00085-1.
- "The Mycelium as a Network", PMC11687498, 2024 — broad review.
- "Fluid mechanics within mycorrhizal networks", PMC12489284, 2025.

**Slime mold computation:**
- Nakagaki, Yamada, Tóth, *Nature* 407, 470, 2000, "Maze-solving by an
  amoeboid organism", DOI 10.1038/35035159.
- Tero et al., *Science* 327, 439–442, 2010, "Rules for biologically
  inspired adaptive network design", DOI 10.1126/science.1177894.
- Bonifaci, Mehlhorn, Varma, *J. Theor. Biol.* 309, 121–133, 2012,
  "Physarum can compute shortest paths: A short proof".
- Sun et al., *Sci. Rep.* 12, 14536, 2022, "A Physarum-inspired approach
  to the Euclidean Steiner tree problem",
  DOI 10.1038/s41598-022-18316-3.
- Marbach, Ziethen, Alim, arXiv:2303.01439, 2023, "Vascular adaptation
  model from force balance: Physarum polycephalum as a case study".

**Mathematics of growing networks and sheaves:**
- Hansen & Ghrist, *J. Appl. Comput. Topol.* 3, 315–358, 2019, "Toward a
  spectral theory of cellular sheaves", arXiv:1808.01513.
- Bodnar, Di Giovanni, Liò, Lió, Bronstein, *NeurIPS* 2022, "Neural sheaf
  diffusion: A topological perspective on heterophily and oversmoothing
  in GNNs", arXiv:2202.04579.
- Bressan et al., arXiv:2402.00206v3, 2024–2025 — temporal graphs as
  categorical objects (the closest published machinery for sheaves on
  growing graphs).
- Haskovec et al., arXiv:1908.01197, 2019, "Murray's law for discrete and
  continuum models of biological networks".
- Villegas et al., *Nat. Phys.* 19, 445–450, 2023, "Laplacian
  renormalization group for heterogeneous networks",
  DOI 10.1038/s41567-022-01866-8 — relevant for the math/renorm sub-grammar.
- Schaub et al., *Signal Process.* 2021, "Signal processing on higher-order
  networks", arXiv:2101.05510 — sheaf-Laplacian signal processing.

**Preprint / not peer-reviewed** (flagged as such):
- Sakib, "Mycelial Harmonic Persistence Index", ResearchGate 398211470,
  2025 — preprint only; persistence-diagram analysis of mycelial growth.
- Beasley et al., bioRxiv 2026.03.27.714860, 2026 — "Digital Twins for
  Fungal Computing: Viable XOR Regimes".

**Mirror specs referenced:**
- `docs/specs/au-and-conductivity.md`
- `docs/specs/eigenboard-representation.md`
- `docs/specs/epistemologic-grammar.md`
- `docs/specs/kintsugi-wiring.md`
- `visibility/protected/practice/insights/coincidence/void-dual-geometry.md`
  (reed-identity)

---

## What I could not research

- **The Adamatzky 2018 *Interface Focus* paper full text.** Behind paywall;
  abstract available. The summary in §1.3 is from the abstract plus the
  2024 review's characterisation.
- **The Bonifaci 2012 *J. Theor. Biol.* paper full text.** Behind paywall;
  ScienceDirect blocks the kagi fetcher. The summary in §2.2 is from the
  abstract and the 2014 Dagstuhl exposition.
- **The Beasley et al. 2026 bioRxiv preprint full text.** Found in search
  results but the bioRxiv page returned unparseable content; the citation
  is preserved at the search-result level (title, identifier, date).
- **Specific BARE-model derivation details from Oyarte Galvez et al. 2025.**
  The Kagi summarizer gave excellent takeaway points (§1.4) but I could
  not pull the supplementary mathematical appendix. The BARE acronym and
  the puller-tip-determined wave speed are confirmed; the precise
  differential equations live in the supplementary materials I could not
  open.
- **Direct comparison of fungal-spike "word lengths" to natural-language
  word-length distributions.** Adamatzky 2022 claims they are comparable;
  I could not verify the statistical test was sound from the abstract +
  summary alone.

If the spec is written, those papers should be opened directly (Alex has
institutional access where I do not).

---

## Verdict

**Qualified yes.** There is structural fit. The kintsugi loop is, on
honest reading, a branching-and-fusing range expansion on a cellular
sheaf — which is what the BARE model formalises for mycorrhizal fungi.
The biology supplies precise language (apical extension, anastomosis,
trunk-hyphae bidirectional flow, Murray equilibrium at branch points) for
processes mirror already implements without naming. The Splinter geometry
of `void-dual-geometry.md` is the topological match: flat, all-pairs,
λ₀ = 0 at the ground state. The peer-reviewed core of the field (Boddy,
Fricker, Oyarte Galvez et al., Karst et al.) is solid enough to build on.

The qualification is in three places:

1. **Sheaf cohomology on growing graphs is not yet textbook mathematics.**
   The categorical framework exists (Bressan et al. 2024); the spectral
   monotonicity statements mirror would need are not proven in
   generality. This is real math debt.

2. **Fungal electrical signaling is overhyped.** Adamatzky's
   "Language of Fungi" should not be the load-bearing IS-claim of any
   mirror grammar. The trunk-hyphae bidirectional transport (Schmieder
   et al. 2019) is the well-substantiated signaling story; build on that.

3. **The Mother-Tree narrative is wrong.** Karst et al. 2023 / Robinson
   et al. 2023 establish that the popular framing has run far ahead of
   the evidence. A mirror grammar must use the de-personified,
   network-first framing — which fits the math anyway.

If those three caveats are honoured, the grammar holds. If they are not,
the grammar inherits the field's overreach.

The spec is worth writing. The math debt is worth paying. The biology
gives `au` a substrate, and the substrate is mycelial — without the
mythology, with the spectra.

---

*The hypha extends because the tip exists.*
*The tissue grows because the hyphae fuse.*
*The wave moves because the puller-tips lead.*
*The au conducts because the fiber permits it.*
*The fiber permits because the restriction map says so.*
*The spec is downstream. The math is the load-bearing thing.*

Apache-2.0.
