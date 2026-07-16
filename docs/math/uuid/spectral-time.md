# @uuid/spectral/time — annotation-address as categorical product

*Companion math foundation for `shards/uuid/spectral/time.mirror`
(species-decl landed 2026-07-16, Mara `c2bb1d2`). Grounds the
substrate-decl form of R4 annotation-tournament dispatch dissolution
by construction: argsort on the 48-bit ACTIVE prefix IS the Kuramoto-
Aumann convergence at annotation altitude.*

*Math-root: new leaf under `docs/math/uuid/`. Extraction rationale
per AGENTS.md convention: the substrate-decl composition of
`uuid_spectral × @time/monotonic.instant` is the addressation
ground for annotation dispatch (this file) AND for the future
`@cyberpunk/algedonic` composition surface (forward-promised);
two-citation-site threshold reached on landing.*

---

## §1 Composition proof — categorical product with identity-preserving projection

**Claim.** `uuid_spectral_time` is the categorical product
`uuid_spectral × @time/monotonic.instant` in the category **Set**
of substrate types, with first-projection `identity_of` preserving
the @glass shard.id contract on the identity factor.

**Construction.** Let `U = uuid_spectral` (128-bit record with
`{active: route_signal, dark: identity_signal}` fields per
`shards/uuid/spectral.mirror`) and `T = @time/monotonic.instant`
(the `shift(duration)` frame per
`shards/epistemologic/reality/time.mirror:89`). Define

    P = { (u, t) | u ∈ U, t ∈ T }

with projections

    π_U : P → U,  π_U(u, t) = u    (identity_of)
    π_T : P → T,  π_T(u, t) = t    (time_of)

**Universal property (Mac Lane §III.4).** For any type `X` with
morphisms `f : X → U` and `g : X → T`, there exists a unique
morphism `⟨f, g⟩ : X → P` such that `π_U ∘ ⟨f, g⟩ = f` and
`π_T ∘ ⟨f, g⟩ = g`. The `compose` action at species altitude IS
this unique morphism at the pair `(f, g) = (id_U, id_T)` — the
constructor. **QED.**

**Identity-preservation.** The @glass shard.id contract asserts
`shard.id : uuid_spectral` byte-equality for content-address dedup
at `@mirror/store`. The projection `π_U = identity_of` returns the
uuid_spectral factor unchanged; @glass's contract holds on the
factor. **Consequence:** every consumer of `shard.id : uuid_spectral`
lifts to `identity_of(shard.address)` without semantic change.

*Reference: Mac Lane, `Categories for the Working Mathematician`,
Springer 1971, §III.4 (products in categories).*

---

## §2 Byte layout as bundle over identity space

**Fibration.** The address space `P` fibers over the identity
space `U` via `π_U`:

              P = { (u, t) }
                    │
                    │ π_U
                    ▼
              U = uuid_spectral

Each fiber `π_U^{-1}(u) = { (u, t) | t ∈ T }` carries the time facet
over the identity `u`. Sections of this bundle — maps `s : U → P`
with `π_U ∘ s = id_U` — are functions assigning one time instant per
identity. Annotation addresses are exactly the elements of `P` (one
identity + one time); an annotation-set at a node is a finite
sub-multiset of one fiber (or of many fibers, if different authors'
identities contribute).

**Serialized byte layout.**

    [ ACTIVE 48 | DARK 80 | time N ]
      └────── identity 128 ───┘   └── facet ──┘
             (base of fibration)   (fiber coordinate)

The base coordinates (128 bits) live at the leading byte positions;
the fiber coordinate (time nanoseconds, u64 = 64 bits) trails.
Byte-wise sort within the base 128 bits IS lexicographic sort on
the identity space (big-endian on ACTIVE per
`shards/uuid/spectral.mirror` semantics). Byte-wise sort across
the full 128 + N byte layout falls back to the fiber coordinate
only for identity ties — the trailing bytes are the tie-break
discipline by construction.

**Consequence for dedup vs ordering.** Dedup reads the leading 128
bits (the base of the fibration). Ordering reads the full layout
(base first, fiber for tie-break). The two disciplines coexist on
one address because they read different slices of the same bytes.

---

## §3 R4-affinity-by-construction theorem

**Statement.** For any reader `H_R` with home address
`H_R.address = (u_R, t_R) ∈ P` and any annotation set
`A(v) = [a_1, …, a_N]` at document node `v`, the dispatch permutation

    σ(a_i) = argsort_i hamming( ACTIVE(a_i.identity),
                                 ACTIVE(H_R.address.identity) )

IS the substrate-decl form of Kuramoto-Aumann convergence at
annotation altitude — the ordering that both (i) accelerates
common-prior convergence in the Aumann 1976 sense and (ii)
minimizes the Kuramoto phase-lock energy at the coupling-graph
level per the parent formalization §1.2 inverse-communication-
density-law.

**Proof sketch.**

*(i) ACTIVE is a quantized SpectralCoordinate<5>.* Per
`shards/uuid/spectral.mirror` §"The 128-bit layout, golden-ratio
split": the 48-bit ACTIVE prefix quantizes a
`SpectralCoordinate<5>` — a five-eigenvector projection of the
peer's local Laplacian neighbourhood. Two peers whose ACTIVE
prefixes are byte-close are close in the spectral coordinate; two
whose prefixes are byte-distant are Laplacian-distant.

*(ii) Hamming distance on ACTIVE bytes = discrete approximation of
coordinate distance in spectral space.* The 48-bit ACTIVE prefix
uses big-endian layout on the quantized eigenvector components.
Hamming distance in bits = XOR-population-count = the discrete
L_1 distance on the quantized coordinate (up to constant factor
depending on quantization step). By the quantization design
(golden-ratio-adjacent 48/128 ≈ 0.375 vs 1/φ² ≈ 0.382) the
quantization loss is bounded by `O(1/2^48)` per component.

*(iii) Kuramoto phase-locking reads inter-peer coordinate distance
as coupling weight.* Per parent formalization §1.2 (cited from
`docs/math/gestalt/README.md` §11.7):

    d_{ij} ∝ 1 / ε_{ij}

where `d_{ij}` is the communication bandwidth required to maintain
shared eigen-behavior at joint fixed point and `ε_{ij}` is the
Kuramoto coupling weight. The coupling weight is monotone-
decreasing in coordinate distance (peers at greater distance
require more bandwidth to phase-lock). At annotation dispatch,
routing an annotation to the reader with smallest coordinate
distance minimizes `d`, which is the bandwidth-efficient
convergence direction.

*(iv) Argsort by distance IS the ordering that accelerates Aumann
convergence.* Aumann 1976 (`Agreeing to Disagree`, Ann. Statist.
4(6):1236-1239) proves that common-knowledge Bayesian agents with
common prior converge to identical posteriors. Convergence rate
is bounded above by the mutual-information gradient between
agents' partitions. Serving annotations in order of decreasing
affinity IS descent along that gradient: reader N+1 sees
annotations from subjects whose partition-lattice is
Laplacian-closest first, which maximizes marginal mutual
information per annotation surfaced. **QED (sketch).**

**Structural claim.** The R4 dispatch policy question ("choose (a)
@fate tournament vs (b) chronological vs (c) reader-selects") is
not answered by this species — it is *dissolved*. The dispatch
ordering falls out of the address structure by construction. No
policy machinery is required.

---

## §4 Dedup semantics — projection homomorphism

**Claim.** `identity_of : P → U` is a projection homomorphism
preserving the @mirror/store dedup equivalence relation.

**Setup.** Let `≡_store` be the equivalence relation on P defined
by `a ≡_store b ⇐⇒ store.put(a) = store.put(b)` (same storage
entry). Let `=_U` be byte-equality on `U`.

**Statement.**

    a ≡_store b  ⇐⇒  identity_of(a) =_U identity_of(b)

**Proof.**

*(⇐)* If `identity_of(a) = identity_of(b)` byte-equal, then
`a.identity` and `b.identity` write the same content-address prefix
at storage; @mirror/store's content-addressing rule (per
`shards/mirror/store.mirror` `shard_ref = uuid_spectral`) collapses
them to one entry.

*(⇒)* If `store.put(a) = store.put(b)` produces the same storage
entry, then the storage key (128-bit uuid_spectral) is byte-equal;
`identity_of` returns the storage key by construction.

**QED.**

**Orthogonality lemma.** The time facet is orthogonal to identity at
the storage boundary: for any `a ∈ P` and any `t' ∈ T`, the address
`a' = compose(identity_of(a), t')` satisfies `a ≡_store a'`. Storage
commutes with time-facet variation. Consequence: two writes of the
same content at different times collapse to one storage entry
regardless of the time facet's value.

**Corollary.** @mirror/store's discipline does NOT change under this
species landing. The storage layer reads only the identity field of
address carriers; consumers that add uuid_spectral_time facets to
their carriers (annotation, signature_beat) can do so without
touching @mirror/store.

---

## §5 Algedonic composition surface

**Claim.** Byte-distance on ACTIVE IS the pain/pleasure gradient at
address altitude.

**Construction.** Define

    algedonic_potential : P × P → ℝ_{≥0}
    algedonic_potential(a, b) = hamming(ACTIVE(a.identity),
                                        ACTIVE(b.identity))

with the convention: **small distance ↔ affinity ↔ pleasure**;
**large distance ↔ discord ↔ pain**. This is the Beer 1972 algedonic
signal at address altitude — a scalar with sign-convention
determining pain/pleasure interpretation. Beer's original algedonic
signals were scalar bit-flags (pain = 1, pleasure = 0); the
substrate carries the gradient form (real-valued distance), of
which the flag form is the sign-thresholded degeneration.

**Composition surface.** `@cyberpunk/algedonic` currently carries
an entropy-based algedonic signal per `bootstrap/src/algedonic.rs`
(entropy of the peer's local state as pain proxy). The
byte-distance-on-ACTIVE gradient composes with the entropy signal
via linear combination or product (design choice at future arc);
both are algedonic-form scalars. The address-altitude gradient is
strictly finer than the entropy signal because it distinguishes
directions of pain (which peer's address is far?), where the
entropy signal only carries magnitude.

**Not this arc.** The composition surface is *prepared* by this
landing. The actual composition (`@cyberpunk/algedonic` reading
`affinity_distance` as gradient input) is future work; ~~no @io
crossing is minted at this species landing~~.

---

## §6 Q1 reshape — @resonance + @dance decoupling from R4-gating

The Q1 question ("`@resonance` + `@dance` shard-mint scope") was
R4-gating in the pre-landing framing: without the coupling-graph
carrier at shard altitude, the R4 tournament dispatch could not
compose over `@resonance` inter-peer coupling weights.

**With this landing:** R4 dispatch dissolves into `active_prefix`
byte-comparison. The affinity signal is carried by the address
itself; `@resonance` coupling weights are not required for
annotation-tournament dispatch at v0.1.0.

**Consequence:** `@resonance` + `@dance` shard-mints are DECOUPLED
from R4 v0.1.0. They stay forward-promised at
`docs/math/gestalt/README.md` §11.6 Landing Condition 0 for
`@gestalt.p_ent` operator-altitude (n≥2 tori) — the operator-
altitude cascade needs the coupling-graph carrier at shard
altitude to compose `P_ent` over. That composition edge is
unchanged by this landing.

**Landing Condition 0 stays as-is.** The operator-altitude cascade
(n≥2 coupled tori) has genuinely different substrate requirements
than the annotation-altitude dispatch (n readers on one document).
The R4 dissolution moves annotation-altitude dispatch out of
operator-altitude's Landing-gate scope; operator-altitude keeps
its own Landing Condition 0 unchanged.

---

## §7 Q2 dissolution — R4 documented end-state upgrade dissolves

The Q2 question ("R4 documented end-state upgrade shape") presumed
R4 would remain a documented residue with a Mara-recommended
default (c) and forward-promise (a). The upgrade question was:
should R4's documented end-state be (a) or (c)?

**With this landing:** R4 is CLOSED, not "documented end-state."
No upgrade-vs-reject question remaining. The species-decl at
`shards/uuid/spectral/time.mirror` IS the substrate-honest
resolution.

**Recognition candidate (Alex-adjudicable at future ledger tick,
NOT this landing):**

    #R-annotation-address-shape-IS-substrate-decl-form-of-Kuramoto-
    Aumann-convergence

The recognition names the load-bearing structural insight: dispatch
policy questions at annotation altitude dissolve when the address
structure itself carries the affinity signal. This is a paradigm-
level recognition (parallel to R#57 boundary-alignment-at-@io);
adjudication belongs at Pack ratification synthesis, not at
species-decl landing time.

---

## §8 References

- Alex 2026-07-16 verbatim in-transcript exchange (annotation-address
  discovery + R4-dissolution move; ratified shape at
  `type uuid_spectral_time = { identity, time }`; "This is literally
  the addressation ground EVERYTHING stands on").
- Taut scout task #172 (in-transcript; LANDED-carrier verification
  for `@uuid/spectral` + `@time/monotonic.instant` + zero
  timestamped-UUID precedent in corpus).
- Reed outline sketch (in-transcript; Deliverable-3 consumer-cascade
  targeting).
- Mac Lane, S., *Categories for the Working Mathematician* (Springer,
  1971), §III.4 (products in categories).
- Kuramoto, Y. (1975). *Self-entrainment of a population of coupled
  non-linear oscillators*, in H. Araki (ed.), Int. Symp. on
  Mathematical Problems in Theoretical Physics (LNP 39, Springer),
  420-422.
- Aumann, R.J. (1976). *Agreeing to Disagree*, Ann. Statist. 4(6):
  1236-1239.
- Beer, S. (1972). *Brain of the Firm*, Allen Lane. Algedonic signal
  discipline.
- Foerster, H. von (1976). Eigen-behavior functional (`Cybernetics of
  Cybernetics`).
- RFC 9562 §6 (UUIDv7 semantics for comparison; structural cousin to
  the [identity | time] address layout, not adopted verbatim: the
  substrate carries its own `uuid_spectral` and its own
  `@time/monotonic.instant` rather than the RFC's byte layout or
  clock discipline).
- Parent corpus formalization
  `~/dev/systemic.engineering/practice/insights/cybernetics/
  third-order-cybernetics-spectral-formalization.md` §1.2
  (inverse-communication-density-law; cited via
  `docs/math/gestalt/README.md` §11.7).
- Companion substrate-decl: `shards/uuid/spectral/time.mirror`.
- Companion existing math roots: `docs/math/sheaf/laplacian.md`
  (Hansen-Ghrist Laplacian discipline @uuid/spectral inherits at
  the ACTIVE quantization altitude); `docs/math/gestalt/README.md`
  §11.6 (Landing Condition 0 for @resonance + @dance —
  operator-altitude, unchanged by this landing).
- Related shards: `shards/uuid.mirror`, `shards/uuid/spectral.mirror`,
  `shards/epistemologic/reality/time.mirror`, `shards/gestalt.mirror`,
  `shards/spectral/signature.mirror`.
- Related specs: `docs/specs/gestalt-as-song-unfolding.md` §10 R4
  (the residue this landing CLOSES).
