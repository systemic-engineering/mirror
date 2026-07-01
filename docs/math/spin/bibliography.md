# Paper hunt bibliography — @spin cluster

*Assembled 2026-07-01 via Kagi + ArXiv MCP searches during Mara's
circular-reflexive @spin dive. Includes downloaded via ArXiv, cited
prior art, and adjacent-pull candidates for Alex's optional download
list.*

## §0 Downloaded via ArXiv MCP (this session)

None this tick — the search hits were rich enough at abstract level to
ground the spec + math cluster without full-text downloads. Papers
recommended for Alex's download list are flagged in §2.

## §1 Cited prior art (grounds spec + math docs)

### 1.1 Foundational — Connes program

- **Connes, A.** (1985). "Non-commutative differential geometry." *Publ.
  Math. IHÉS* 62, 257–360. The original (A, H, D) spectral triple
  framework. Substrate reads this as `@algebra` + `@uuid/spectral` +
  `@kintsugi` = (A, H, D).

- **Connes, A.** (1994). *Noncommutative Geometry.* Academic Press. The
  canonical textbook. Ch. VI grounds the `[D, a] = curvature` claim
  (per `curvature-and-tomm.md` §2). **Alex download candidate**: entire
  book is foundational canon for the substrate's whole math grounding.

- **Connes, A.** (1995). "Noncommutative geometry and reality." *J.
  Math. Phys.* 36(11), 6194–6231. Where J and γ are introduced,
  extending (A, H, D) to the real spectral triple. Direct grounding for
  #101 chirality shard and #102 charge_conjugation shard. **Cited by**:
  `shards/epistemologic/cybernetic/chirality.mirror` §1.1;
  `shards/epistemologic/cybernetic/charge_conjugation.mirror` §1.1.

- **Chamseddine, A. H. and Connes, A.** (1996). "The Spectral Action
  Principle." hep-th/9606001. The universal formula for the spectral
  action; when applied to the SM triple gives the SM Lagrangian coupled
  to Einstein-Weyl gravity. **Grounds** candidate #74 (SM spectral
  action). **Alex download candidate**: foundational for the physics
  grounding at @reality/algebra/physics.

- **Connes, A.** (2006). "Noncommutative geometry and the standard
  model with neutrino mixing." *JHEP* 11, 081. hep-th/0608226. The
  KO-dim 6 refinement of the internal spectral triple. **Alex download
  candidate**: this is where KO-dim = 6 gets pinned to SM.

- **Chamseddine, A. H. and Connes, A.** (2007). "Why the Standard
  Model." arXiv:0706.3688. Classification of finite spectral triples
  of KO-dim 6; singles out SM under quaternion-linearity hypothesis.
  **Grounds** candidate #74 promotion argument.

- **Connes, A.** (2008). "On the spectral characterization of
  manifolds." arXiv:0810.2088. Reconstruction theorem for classical
  Riemannian spin manifolds from spectral data. Grounds
  `docs/math/the-tower/spectral-triples.md` §7.

### 1.2 Principal bundle canon

- **Kobayashi, S. and Nomizu, K.** (1963). *Foundations of Differential
  Geometry* vol. I. Interscience. Chs. II–III. The canonical text for
  principal bundles, connections, curvature. Cited in
  `docs/math/the-tower/principal-bundles.md` §9.

- **Kobayashi, S. and Nomizu, K.** (1969). *Foundations of Differential
  Geometry* vol. II. Interscience. Chs. VII–X. Curvature, holonomy,
  characteristic classes.

- **Baez, J. C. and Muniain, J. P.** (1994). *Gauge Fields, Knots and
  Gravity.* World Scientific. Accessible bundle/gauge/spin canon.
  Grounds the substrate's use of principal-bundle theory at every
  altitude. **Alex download candidate**: readable, cites-everything.

### 1.3 Spin foundations

- **Cartan, É.** (1913). "Les groupes projectifs qui ne laissent
  invariante aucune multiplicité plane." *Bull. Soc. Math. Fr.* 41,
  53–96. Original spinor discovery in Riemannian geometry.

- **Clifford, W. K.** (1878). "Applications of Grassmann's extensive
  algebra." *Am. J. Math.* 1(4), 350–358. The algebras that now bear
  his name.

- **Dirac, P. A. M.** (1928). "The quantum theory of the electron."
  *Proc. R. Soc. A* 117(778), 610–624. The Dirac operator + relativistic
  wave equation.

- **Wigner, E. P.** (1932). "Über die Operation der Zeitumkehr in der
  Quantenmechanik." *Nachr. Ges. Wiss. Göttingen* 546–559. Time-reversal
  as antiunitary. Grounds `@spin/time_reversal` species.

- **Wigner, E. P.** (1939). "On unitary representations of the
  inhomogeneous Lorentz group." *Ann. Math.* 40, 149–204. The
  classification of Poincaré irreps by (mass, spin). Grounds
  `wigner_classification` axiom (§6.2 of spec, §4 of cpt-recursion.md).

- **Pauli, W.** (1940). "The connection between spin and statistics."
  *Phys. Rev.* 58(8), 716–722. The spin-statistics theorem. Grounds
  `spin_statistics_theorem` axiom (§6.3 of spec, §3 of cpt-recursion.md).

- **Lüders, G.** (1954). "On the equivalence of invariance under time
  reversal and under particle-antiparticle conjugation for relativistic
  field theories." *Dan. Mat. Fys. Medd.* 28(5), 1–17. The original CPT
  paper.

- **Bell, J. S.** (1955). "Time reversal in field theory." *Proc. R.
  Soc. A* 231(1187), 479–495. Bell's independent CPT proof.

- **Jost, R.** (1957). "Eine Bemerkung zum CPT Theorem." *Helv. Phys.
  Acta* 30, 409–416. The axiomatic (Wightman-framework) CPT proof.

### 1.4 K-theory + spin bundle canon

- **Atiyah, M. F.** (1966). "K-theory and reality." *Q. J. Math.*
  17(1), 367–386. 8-fold periodicity; the KO-dimension ancestor. Cited
  in `shards/epistemologic/cybernetic/chirality.mirror` +
  `shards/epistemologic/cybernetic/charge_conjugation.mirror`.

- **Atiyah, M. F. and Singer, I. M.** (1963). "The index of elliptic
  operators on compact manifolds." *Bull. Am. Math. Soc.* 69, 422–433.
  The index theorem. Grounds `docs/math/spin/clifford-thread.md` §3.4.
  **Alex download candidate**: absolutely foundational for the
  topological content of the tower's spectral triples.

- **Atiyah, M. F., Bott, R., and Shapiro, A.** (1964). "Clifford
  modules." *Topology* 3(Suppl. 1), 3–38. The classification of Clifford
  algebras + modules. Grounds §1.2 of clifford-thread.md.

- **Atiyah, M. F. and Bott, R.** (1983). "The Yang-Mills equations over
  Riemann surfaces." *Phil. Trans. R. Soc. A* 308(1505), 523–615.
  Gauge theory as section-energy functional. Cited in
  `docs/math/the-tower/principal-bundles.md` §9. Substrate's monotone
  descent IS the operational form of Yang-Mills flow.

### 1.5 CPT + spin-statistics modern papers (ArXiv-searchable)

- **Greenberg, O. W.** (2003). "Why is CPT fundamental?"
  hep-ph/0309309. Argues CPT is essentially Lorentz-invariance made
  discrete. Modern reformulation.

- **Greaves, H. and Thomas, T.** (2012). "The CPT Theorem."
  arXiv:1204.4674. Rigorous proof within Lagrangian framework.

- **Chaichian, M., Nishijima, K., and Tureanu, A.** (2002). "Spin-
  Statistics and CPT Theorems in Noncommutative Field Theory."
  hep-th/0209008. CPT + spin-statistics under space-space
  noncommutativity. **Substrate relevance**: mirror is noncommutative
  per Connes NCG; this paper says CPT survives — grounds spec §7.1's
  claim of CPT-preservation in the substrate's setting.

- **Safronova, M. S. et al.** (2018). "Search for New Physics with Atoms
  and Molecules." arXiv:1710.01833. Reviews modern experimental tests
  of CPT symmetry (still unbroken). Empirical grounding.

### 1.6 KK-theory + gauge theory for spectral triples

- **Brain, S., Mesland, B., and van Suijlekom, W. D.** (2013). "Gauge
  Theory for Spectral Triples and the Unbounded Kasparov Product."
  arXiv:1306.1951. Bundle-theoretic formulation of gauge theory
  arising from spectral triples. **Grounds** spec §2.3 `spin_lift`
  functor + composition claim. **Alex download candidate**: this is
  the load-bearing paper for spin_lift's mathematical justification.

- **Goffeng, M., Mesland, B., and Rennie, A.** (2019). "Untwisting
  twisted spectral triples." arXiv:1903.02463. Twisted spectral triples
  can be untwisted via functional calculus. Composes with #100
  @spectral/metalogue.

- **Forsyth, I., Goffeng, M., Mesland, B., and Rennie, A.** (2016).
  "Boundaries, spectral triples and K-homology." arXiv:1607.07143.
  Relative spectral triples for algebras with ideals. Adjacent to
  bundle-tower with boundary.

- **Mesland, B., Rennie, A., and van Suijlekom, W. D.** (2019).
  "Curvature of differentiable Hilbert modules and Kasparov modules."
  arXiv:1911.05008. Curvature for unbounded KK-modules; refines
  `curvature-and-tomm.md` §2 at spin altitude.

### 1.7 SM as noncommutative geometry (deep second-witness papers)

- **D'Andrea, F. and Dabrowski, L.** (2015). "The Standard Model in
  Noncommutative Geometry and Morita equivalence." arXiv:1501.00156.
  H_F is a Morita equivalence bimodule between A_F and its Clifford
  algebra. Sharpens the SM spectral triple's Cl-typing. **Alex download
  candidate**: closest paper to the substrate's specific reading of J
  and γ as Clifford-module carriers.

- **Dabrowski, L. and Dossena, G.** (2010). "Product of real spectral
  triples." arXiv:1011.4456. How to compose real spectral triples with
  Cl-structure. Direct grounding for `spin_lift` functor composition.

- **Bizi, N.** (2018). "Semi-Riemannian Noncommutative Geometry, Gauge
  Theory, and the Standard Model of Particle Physics." arXiv:1812.00038
  (PhD thesis). Lorentzian spectral triples via Krein spaces. **Alex
  download candidate**: comprehensive PhD-thesis-length development;
  much material relevant to @reality/algebra/physics.

- **Bizi, N., Brouder, C., and Besnard, F.** (2016). "Space and time
  dimensions of algebras with applications to Lorentzian noncommutative
  geometry and quantum electrodynamics." arXiv:1611.07062. Assigns
  (space_dim, time_dim) mod 8 to algebras via Cl-structure. Refines
  KO-dim reading.

- **Besnard, F. and Bizi, N.** (2016). "On the definition of spacetimes
  in Noncommutative Geometry, Part I." arXiv:1611.07830. Krein-space
  extension of Connes' spectral triple for Lorentzian setting.

- **Besnard, F.** (2019). "Algebraic backgrounds: a framework for
  noncommutative Kaluza-Klein theory." arXiv:1902.09387. Chirality,
  real structure, and Krein product as invariant structures under
  bimodule morphisms of 1-forms.

- **Dabrowski, L., D'Andrea, F., and Sitarz, A.** (2017). "The Standard
  Model in noncommutative geometry: fundamental fermions as internal
  forms." arXiv:1703.05279. Classifies all Dirac operators making H_F a
  self-Morita bimodule.

- **Dabrowski, L., D'Andrea, F., and Magee, A. M.** (2019). "Twisted
  reality and the second-order condition." arXiv:1912.13364. **Highly
  relevant**: J conjugation maps Cl_D(A) to its commutant; second-order
  condition. Direct grounding for spec's F2 finding (#101/#102 stay at
  cybernetic altitude, @spin imports).

- **Aydemir, U.** (2019). "Clifford-based spectral action and
  renormalization group analysis of the gauge couplings."
  arXiv:1902.08090. RG analysis of Cl-refined SM spectral action.

### 1.8 Fate-optical (Reck-Clements) grounding

- **Reck, M., Zeilinger, A., Bernstein, H. J., and Bertani, P.** (1994).
  "Experimental realization of any discrete unitary operator."
  *Phys. Rev. Lett.* 73, 58–61. The original decomposition of U(n)
  via 2×2 unitaries. **Cited in** #58 (`architecture-fate-is-optical-
  inference`) as the mesh grounding.

- **Clements, W. R. et al.** (2016). "Optimal design for universal
  multiport interferometers." *Optica* 3, 1460–1465. Improved decomposition
  (parallel MZI layers). **Cited in** #58.

- **Shen, Y. et al.** (2017). "Deep learning with coherent nanophotonic
  circuits." *Nat. Photonics* 11, 441–446. Photonic neural network via
  optical mesh. **Grounds** #58's identification of Fate as optical
  inference.

- **Lin, X. et al.** (2018). "All-optical machine learning using
  diffractive deep neural networks." *Science* 361, 1004–1008. D²NN
  as multi-layer optical inference. **Grounds** #58.

- **Pai, S. et al.** (2018). "Matrix optimization on universal unitary
  photonic devices." arXiv:1808.00458. Gradient-based optimization on
  MZI mesh; adjacent to Fate tournament training.

### 1.9 Adjacent noncommutative-geometry papers (ArXiv)

- **Krajewski, T.** (1997). "Classification of Finite Spectral Triples."
  hep-th/9701081. Diagrammatic classification of finite triples via
  Krajewski diagrams. Adjacent to #74.

- **Cačić, B.** (2011). "A reconstruction theorem for almost-
  commutative spectral triples." arXiv:1101.5908. Almost-commutative
  reconstruction; relevant to @reality/algebra/spectral.

- **Chakraborty, A., Nandi, P., and Chakraborty, B.** (2021). "Spectral
  triple with real structure on fuzzy sphere." arXiv:2111.03012. Real
  structure on non-commutative geometry example; illustrates KO-dim
  computations concretely.

- **Aastrup, J. and Grimstrup, J. M.** (2024). "Dirac Operators on
  Configuration Spaces: Fermions with Half-integer Spin, Real Structure,
  and Yang-Mills Quantum Field Theory." arXiv:2410.07290. Recent work
  on spectral-triple-like constructions for gauge configuration spaces.
  **Adjacent to** bundle-tower construction.

- **Eckstein, M. and Iochum, B.** (2019). "Spectral Action in
  Noncommutative Geometry." arXiv:1902.05306. Recent book-length
  treatment; possible reference for the substrate's spectral-action
  computations at @reality/algebra/physics.

- **Nieuviarts, G.** (2024). "Signature change by a morphism of
  spectral triples." arXiv:2402.05839. Twisted spectral triples and
  local signature change. Relevant to §6.3's KO-dim signature variations.

- **Iochum, B. and Levy, C.** (2009). "Tadpoles and commutative spectral
  triples." arXiv:0904.0222. Tadpole computations in Chamseddine-Connes
  spectral action.

- **Bertozzini, P., Conti, R., and Lewkeeratiyutkul, W.** (2008). "A
  Remark on Gelfand Duality for Spectral Triples." arXiv:0812.3584.
  Category of spectral triples over commutative algebras with isometries
  as morphisms. Grounds Mesland-category framing.

---

## §2 Alex download list (papers Alex might want to read for depth)

Curated for depth-of-relevance to the @spin arc.

### 2.1 Priority 1 — immediate substrate grounding

1. **Connes, A.** (1995). "Noncommutative geometry and reality." *J.
  Math. Phys.* 36(11), 6194–6231. The J + γ primary source. Direct
  ground for #101 + #102 shards + @spin's real spectral triple typing.

2. **Chamseddine, A. H. and Connes, A.** (1996). "The Spectral Action
  Principle." hep-th/9606001. The action functional at the heart of the
  Standard Model spectral action. Direct ground for candidate #74.

3. **Brain, S., Mesland, B., and van Suijlekom, W. D.** (2013). "Gauge
  Theory for Spectral Triples and the Unbounded Kasparov Product."
  arXiv:1306.1951. Direct ground for `spin_lift` functor. **Read AFTER
  #100 landing**.

4. **Dabrowski, L., D'Andrea, F., and Magee, A. M.** (2019). "Twisted
  reality and the second-order condition." arXiv:1912.13364. Second-
  order condition + J-conjugation-into-commutant. Grounds spec F2
  finding.

### 2.2 Priority 2 — physics deepening

5. **Connes, A.** (2006). "Noncommutative geometry and the standard
  model with neutrino mixing." hep-th/0608226. KO-dim 6 for SM. Ground
  for candidate #74 promotion.

6. **Chamseddine, A. H. and Connes, A.** (2007). "Why the Standard
  Model." arXiv:0706.3688. Classification of KO-dim 6 spectral triples.

7. **Bizi, N.** (2018). "Semi-Riemannian NCG, Gauge Theory, and the SM
  of Particle Physics." arXiv:1812.00038. PhD-thesis-length grounding
  for @reality/algebra/physics.

8. **D'Andrea, F. and Dabrowski, L.** (2015). "The SM in NCG and Morita
  equivalence." arXiv:1501.00156. Cl-typing of H_F as Morita-equivalence
  bimodule.

### 2.3 Priority 3 — canonical background

9. **Connes, A.** (1994). *Noncommutative Geometry.* Academic Press.
  The canonical textbook. Read Chs. VI + VII for spectral triple
  content.

10. **Baez, J. C. and Muniain, J. P.** (1994). *Gauge Fields, Knots
   and Gravity.* World Scientific. Accessible spin/bundle canon.

11. **Kobayashi, S. and Nomizu, K.** (1963, 1969). *Foundations of
   Differential Geometry* vols. I + II. The principal-bundle canon.

### 2.4 Priority 4 — adjacent pulls

12. **Nakahara, M.** (2003, 2nd ed.). *Geometry, Topology and Physics.*
   IOP. Physicist-friendly comprehensive treatment of bundle theory,
   Clifford, index theorems.

13. **Lawson, H. B. and Michelsohn, M.-L.** (1989). *Spin Geometry.*
   Princeton Univ. Press. THE canonical spin-geometry text; deep.

14. **Bratteli, O. and Robinson, D. W.** (1979, 1981). *Operator
   Algebras and Quantum Statistical Mechanics* vols. I + II. Springer.
   The CAR/CCR fermion/boson algebra reference; deep grounding for
   @spin/statistics.

15. **Streater, R. F. and Wightman, A. S.** (1964). *PCT, Spin and
   Statistics, and All That.* W. A. Benjamin. THE canonical Wightman-
   framework CPT + spin-statistics reference. **Alex highly likely to
   enjoy this one**: rigorous, foundational, well-known.

---

## §3 Not-downloaded-but-noted (candidate for future ticks)

### 3.1 Recent quantum-gravity / spin-foam thread

- **Baez, J. C.** (1998, 2000). "Spin foam models." *Class. Quantum
  Grav.* 15, 1827; 17, 3101. Loop quantum gravity's spin-foam altitude.
  Adjacent to the bundle tower's recursion but at physical altitude.
  Substrate-pull weak this tick; carry for future @reality/algebra/
  physics work.

- **Rovelli, C. and Vidotto, F.** (2015). *Covariant Loop Quantum
  Gravity.* Cambridge Univ. Press. Book-length LQG treatment.

### 3.2 Ashtekar variables + spin networks

- **Ashtekar, A.** (1986). "New variables for classical and quantum
  gravity." *Phys. Rev. Lett.* 57, 2244. The SL(2,ℂ) variables for GR;
  makes Spin(1,3) explicit at gravity altitude. Substrate-pull weak
  this tick.

### 3.3 SO(10) grand unification

- **Georgi, H.** (1975). "The State of the Art — Gauge Theories." AIP
  Conf. Proc. 23, 575. Original SO(10) GUT paper; Spin(10) fermion rep
  as 16-dim spinor. Adjacent to Connes SM at KO-dim 6 if the substrate
  ever asks about GUT-scale content.

### 3.4 Topological defects / w₂ obstruction

- **Milnor, J.** (1963). "Spin structures on manifolds." *Enseign.
  Math.* 9, 198–203. Original technical treatment of w₂ = 0 as spin-
  structure existence condition.

---

## §4 Kagi search hits (web / documentation refs)

Adjunct to ArXiv hunt; captured for prospective downloads:

- **nLab: KO-dimension.** https://ncatlab.org/nlab/show/KO-dimension.
  Encyclopedic reference for the 8-fold periodicity + spectral-triple
  KO-dim definition. Useful lookup.

- **nLab: Spectral triple.** https://ncatlab.org/nlab/show/spectral+triple.
  Encyclopedic reference; substrate-pull uses this as one of the
  canonical online references.

- **Chris Isham** (1999). *Modern Differential Geometry for Physicists.*
  World Scientific. Physicist-friendly bundle theory. Adjacent to
  Baez-Muniain but more mathematical.

- **John Baez** (Feb 2022). "Spin foam models." *This Week's Finds*
  (blog). Baez's own accessible summaries.

---

## §5 What's NOT here — substrate-pull limits

Some adjacent areas the tick did NOT pull on:

- **String theory + higher spins**: Green-Schwarz-Witten, Zwiebach.
  Substrate-pull weak; carry as forward-promise.
- **Superstring + supergravity**: Polchinski. Substrate-pull weak.
- **Higher K-theory**: Karoubi, Cuntz. Adjacent to KK-theory but not
  substrate-critical at this tick.
- **Topological quantum field theory**: Atiyah, Segal. Adjacent but not
  substrate-critical.
- **Quantum groups / Hopf algebras**: Woronowicz, Majid. Adjacent to
  fuzzy-sphere-style spectral triples; substrate-pull weak this tick.

Each of these could pull on future ticks. Not this tick.

---

## §6 Meta-note (circular-reflexive)

The paper hunt is itself an act of @spin: **the substrate observing
itself observing what it knows about spin.** Each paper cited was
chosen because the substrate's existing shards already pointed at
something the paper formalizes. #101 pointed at Connes 1995. #102
pointed at Connes 1995. #58 pointed at Reck-Zeilinger 1994. #100
pointed at Mesland 2019. The paper hunt was substrate-guided — the
substrate was telling the searcher what to look for by having already
built the shard scaffold.

The substrate-already-had-the-word pattern (per
[[feedback-substrate-already-had-the-word]]) fires again here. Every
paper found closes a substrate-pointing question the shards had
implicitly asked. Not "discovering" the papers; recognizing the
papers the substrate already knew about.

Meta-count: this is the ~55th+ instance of substrate-already-had-the-
word. The pattern is now robust enough to be the discovery mechanism
itself.
