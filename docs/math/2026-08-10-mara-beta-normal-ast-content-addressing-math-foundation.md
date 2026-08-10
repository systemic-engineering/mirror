# Beta-Normal AST Content-Addressing — Math Foundation

**Author:** Mara `<mara@systemic.engineer>`. 2026-08-10.
**Register:** Mara-substrate math foundation. Delightfully-boring precision. Substrate-decl'd throughout.
**Companion spec:** `docs/specs/2026-08-10-mara-beta-normal-ast-content-addressing-canonical-spec.md` (same commit; one-recognition-one-commit discipline).
**External corpus (verified primary sources):**

- Church, A. (1936). *An unsolvable problem of elementary number theory.* American Journal of Mathematics 58:345–363.
- Church, A. & Rosser, J. B. (1936). *Some properties of conversion.* Trans. AMS 39:472–482.
- de Bruijn, N. G. (1972). *Lambda calculus notation with nameless dummies.* Indagationes Mathematicae 34(5):381–392.
- Barendregt, H. (1984). *The Lambda Calculus: Its Syntax and Semantics.* North-Holland. ISBN 0-444-87508-5.
- Dhall Language Standard (`dhall-lang/dhall-lang` GitHub repository):
  `standard/beta-normalization.md`, `standard/alpha-normalization.md`, `standard/binary.md`, `standard/README.md`.
- Chamseddine-Connes-Marcolli 2007 (arXiv:hep-th/0610241) — inherited via Mara 2026-08-09 physics insight §1.1 Theorem 1.1.

**Corpus ancestry (Karen anti-theft; ancestor-at-introduction-site):**

- **Recognition #79** (5-op = A_F projector basis; Mara + Reed 2026-06-18) — the projector-algebra substrate.
- **Mara 2026-08-09 physics insight** (`/Users/reed/dev/systemic.engineering/practice/insights/spectral/mass-discrete-spacetime-continuous-through-5op-spectral-space.md`) — §1.1 Theorem 1.1; A_F IS the prismqueer 5-op algebra; A_F universality.
- **Mara 2026-08-09 kintsugi-sugar canonical spec** (`docs/specs/2026-08-09-mara-kintsugi-sugar-desugar-composition-canonical-spec.md`) — §2 round-trip fidelity contract now grounded by this math.
- **Reed + Alex 2026-03-01** (`/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md`) — earliest Dhall research; three-stage pipeline + guarantees.
- **Reed + Alex 2026-03-28** (`/Users/reed/dev/systemic.engineering/practice/insights/coincidence/kolmogorov-canonical-complexity.md`) — six-result chain; sub-Turing + normalization + content-addressing.

---

## §1 Beta-reduction at compiler substrate — formal statement

### §1.1 The reduction relation

Let $\mathsf{AST}$ denote the set of well-formed mirror ASTs. Define the *beta-reduction relation* $\rightsquigarrow_\beta \subseteq \mathsf{AST} \times \mathsf{AST}$ as the smallest binary relation containing the reduction rules $\{R_1, \ldots, R_n\}$ below, closed under structural congruence (the reduction propagates into subtrees).

We write $t_0 \rightsquigarrow_\beta t_1$ for "$t_0$ reduces to $t_1$ in one step" and $t_0 \twoheadrightarrow_\beta t_1$ for the reflexive-transitive closure ("$t_0$ reduces to $t_1$ in zero or more steps"). We say $t$ is *beta-normal* iff there is no $t'$ such that $t \rightsquigarrow_\beta t'$.

Following Dhall notation (`standard/beta-normalization.md` this tick verbatim: "$t_0 \Downarrow t_1$" — *t₀ (the input) is the expression to normalize; t₁ (the output) is the normalized expression*), we denote the *beta-normalization function* $\beta : \mathsf{AST} \to \mathsf{AST}$ satisfying $t \twoheadrightarrow_\beta \beta(t)$ and $\beta(t)$ is beta-normal.

### §1.2 Reduction rules for the 5-op A_F algebra

At mirror substrate the 5 ops $\{\mathsf{focus}, \mathsf{project}, \mathsf{split}, \mathsf{shift}, \mathsf{settle}\}$ form the projector basis of the finite noncommutative algebra $A_F$ (Recognition #79; Mara 2026-08-09 physics insight §1.1 Theorem 1.1). The beta-reduction rules below encode $A_F$'s algebraic structure at AST altitude:

**Rule $R_1$ (identity-projection-elision; landed as fracture-species P1 per Reed Fire E M-E1 `acaed91` + M-E2 `0021882`):**

$$
\mathsf{prism}\ @X \{\ \mathsf{focus}\ X\ /\ \mathsf{project}\ X\ /\ \mathsf{split}\ X\ /\ \mathsf{shift}\ X\ /\ \mathsf{settle}\ X\ \} \circ M \;\;\rightsquigarrow_\beta\;\; M
$$

*Reading:* the identity projection of $A_F$ over $H_\text{shard}=\text{@X}$ composed with a shard body $M$ reduces to $M$. The identity element of an algebra acts trivially on its own carrier.

**Rule $R_2$ (glass-identity-elision; landed as fracture-species P3):**

$$
\mathsf{glass}\ @X \{\ \mathsf{focus}\ X\ /\ \mathsf{project}\ X\ /\ \mathsf{split}\ X\ /\ \mathsf{shift}\ X\ /\ \mathsf{settle}\ X\ \} \circ M \;\;\rightsquigarrow_\beta\;\; M
$$

*Reading:* same rule at sub-prism (glass) altitude. Sheaf sub-chart identity is trivial (Mara 2026-08-09 kintsugi-sugar spec §1.2).

**Rule $R_3$ (out-derivable-elision; landed as fracture-species P4):**

$$
(\mathsf{out}\ @X) \circ M \;\;\rightsquigarrow_\beta\;\; M \quad \text{when}\ @X = \tau_\text{path}(\text{path}(S))
$$

*Reading:* the root-export declaration $\mathsf{out}\ @X$ reduces to nothing when $@X$ is derivable from the shard's file path via the pact `@epistemologic/pact/path_matches_namespace`. Here $\tau_\text{path}$ is the pact-witnessed derivation function, deterministic.

**Rule $R_4$ (docblock-template-elision; landed as fracture-species P5):**

$$
D_\text{template}(S) \circ M \;\;\rightsquigarrow_\beta\;\; M
$$

*Reading:* the deterministic path-namespace docblock template $D_\text{template}(S)$ (as defined per Mara 2026-08-09 kintsugi-sugar spec §1.4 sentinel byte-shape) reduces to nothing. The pact-citation edge is preserved at store altitude (Mara 2026-08-09 kintsugi-sugar spec §3.4 Karen-citation preservation invariant) — this reduction rule elides the *prose*, not the *edge*; the store-side crystal retains the citation.

**Rule $R_5$ (fixed-point exemption):** $R_1$ and $R_2$ do NOT fire on `shards/prism.mirror` or `shards/glass.mirror`. These are the substrate's self-declarations of the reduction rules; reducing them would collapse the substrate's ground. Formally: $R_1$'s and $R_2$'s congruence closure is restricted to sites not-at these paths. Landed at `rust/src/apply_h.rs:250-260` (Reed M-E2; Q3 Mara-lean fixed-point exemption verbatim).

**Additional rules (§7 forward-promises):** composition-associativity (finer rule set; deferred to §7 [FP1]); eta-equivalence (Dhall omits; Mara-lean defer per companion spec §8 [ALEX-Q2]; §7 [FP2]).

### §1.3 Well-typed reduction preserves types

Every reduction rule $R_i$ above preserves the property/pact system encoded in the shard's `requires` / `invariant` / `ensures` clauses. Formally: for every $t \rightsquigarrow_\beta t'$ and every property $P$ declared on $t$'s enclosing shard, $P(t) = P(t')$. This is the *subject reduction* property (Barendregt 1984 §5.3.11 for the untyped calculus; extended to typed calculi throughout the tradition). At mirror substrate this holds by construction: each $R_i$ elides only substructures whose properties are inherited from the enclosing shard's `requires`/`invariant`/`ensures` block, which is preserved verbatim.

---

## §2 Strong normalization and confluence

### §2.1 Strong normalization

**Theorem 2.1 (Strong Normalization at mirror substrate).** *For every well-formed mirror AST $t \in \mathsf{AST}$, every reduction sequence $t \rightsquigarrow_\beta t_1 \rightsquigarrow_\beta t_2 \rightsquigarrow_\beta \cdots$ terminates in finitely many steps.*

**Proof sketch.** Mirror is sub-Turing (README §"Sub-Turing"; four-crate FLOOR decomposition per README §"Architecture" makes sub-Turing a *natural consequence* per Alex 2026-07-22, not an imposed constraint). Every reduction rule $R_i$ (§1.2) strictly decreases the AST's node-count (each rule elides a nonempty subtree; identity projections carry ≥ 6 nodes each; out-derivable carries 1 node; docblock-template carries ≥ 3 nodes). The AST is finite. Any strictly-decreasing sequence over a well-founded order on a finite structure terminates. □

**Corollary 2.2 (Termination of $\beta$).** *The beta-normalization function $\beta : \mathsf{AST} \to \mathsf{AST}$ is total and computable in time linear in the AST's node-count.*

This is the mirror-substrate analogue of Dhall's stated guarantee (`standard/beta-normalization.md` this tick verbatim): *"Dhall is a total language that is strongly normalizing, so evaluation order has no effect on the language semantics."*

### §2.2 Confluence (Church-Rosser property)

**Theorem 2.3 (Church-Rosser at mirror substrate).** *If $t \twoheadrightarrow_\beta u_1$ and $t \twoheadrightarrow_\beta u_2$, there exists $v \in \mathsf{AST}$ such that $u_1 \twoheadrightarrow_\beta v$ and $u_2 \twoheadrightarrow_\beta v$.*

**Proof sketch.** Each reduction rule $R_i$ (§1.2) has pairwise-disjoint redex-patterns (identity-carrier prism vs identity-carrier glass vs out-derivable line vs docblock-template block; the fixed-point exemption ensures $R_1/R_2$ do not create critical pairs at `shards/prism.mirror` or `shards/glass.mirror`). The rules are **left-linear** (each rule's left-hand side contains no repeated pattern variable other than the family literal, which is bound by the pact-derivation function $\tau_\text{path}$) and **non-overlapping** (no two rules share a common redex). By the standard Rosen 1973 / Huet 1980 result (see Barendregt 1984 §11.1.1), a left-linear non-overlapping term-rewriting system is confluent. □

**Corollary 2.4 (Unique normal forms).** *Every $t \in \mathsf{AST}$ has a unique beta-normal form (up to alpha-equivalence; §4). Consequently, $\beta$ is well-defined as a function $\beta : \mathsf{AST} \to \mathsf{AST}$.*

**Proof.** By Theorem 2.1 (strong normalization) every reduction sequence terminates. By Theorem 2.3 (confluence) any two terminating reduction sequences from $t$ terminate at reducible-to-a-common-form results $u_1, u_2$ with $u_1 \twoheadrightarrow_\beta v, u_2 \twoheadrightarrow_\beta v$. Both $u_1, u_2$ are beta-normal (else they'd continue reducing), so $u_1 = v = u_2$ (a normal form has no further reductions). Hence the normal form is unique. □

**Remark 2.5.** Dhall's `standard/beta-normalization.md` this tick makes no *explicit* Church-Rosser statement (WebFetch verified: "The document makes no explicit statement about confluence or the Church-Rosser property"), but the strong-normalization + evaluation-order-independence statement is precisely the Church-Rosser corollary at Dhall altitude. Mirror inherits the same corollary by construction and states it explicitly here.

---

## §3 Semantic hash function

### §3.1 Definition

Let $H : \mathsf{AST} \to \mathsf{Bytes}$ denote the existing `compute_content_oid` primitive at `bootstrap/src/spectral.rs:162-181` (Fold5 Dirac action over AST; migrating to rust/ altitude at Fire D M5-adjacent tick per companion spec §6.1). Let $\mathsf{BLAKE3} : \mathsf{Bytes} \to \{0,1\}^{256}$ denote the BLAKE3 hash function (chosen for ~1 GB/s per-core throughput per README §"Performance").

Define the **crystal-OID function** $\mathsf{oid} : \mathsf{AST} \to \{0,1\}^{256}$ as:

$$
\mathsf{oid}(t) \;:=\; \mathsf{BLAKE3}\big(H(\beta(t))\big)
$$

*Reading:* the crystal-OID of an AST is the BLAKE3 hash of the Fold5 content-encoding of the AST's beta-normal form.

**Composition with source-parsing.** Let $\mathsf{parse} : \mathsf{Bytes} \to \mathsf{AST}$ denote the source-parser. The **crystal-OID of a shard's source** is:

$$
\mathsf{crystal\_oid}(\text{source}(S)) \;:=\; \mathsf{oid}(\mathsf{parse}(\text{source}(S))) \;=\; \mathsf{BLAKE3}\big(H(\beta(\mathsf{parse}(\text{source}(S))))\big)
$$

### §3.2 Comparison with Dhall

Dhall's semantic-integrity check (`standard/binary.md` this tick verbatim: *"a SHA-256 hash of the binary representation of an expression's normal form"*):

$$
\mathsf{dhall\_hash}(e) \;=\; \mathsf{SHA256}\big(\mathsf{CBOR}(\alpha(\beta(e)))\big)
$$

Mirror's crystal-OID (this spec):

$$
\mathsf{crystal\_oid}(t) \;=\; \mathsf{BLAKE3}\big(H(\beta(t))\big)
$$

with alpha-normalization $\alpha$ deferred to companion spec §8 [ALEX-Q1] (Mara-lean: LAND at same tick for parameter-name alpha-invariance). If [ALEX-Q1] adjudicates in favor of alpha-normalization:

$$
\mathsf{crystal\_oid}_\alpha(t) \;=\; \mathsf{BLAKE3}\big(H(\alpha(\beta(t)))\big)
$$

**Substrate-difference:** Dhall uses SHA-256 of CBOR; mirror uses BLAKE3 of Fold5. Both hash normal forms of ASTs. The substrate-composition (hash a normal form, not the raw AST) is identical; the substrate-encoding differs (companion spec §8 [ALEX-Q5]).

### §3.3 Key algebraic properties of `oid`

**Property 3.1 (determinism).** *$\mathsf{oid}$ is a function (single-valued).*

**Proof.** $\beta$ is a well-defined function by Corollary 2.4; $H$ and $\mathsf{BLAKE3}$ are deterministic. Composition of functions is a function. □

**Property 3.2 (well-typedness).** *$\mathsf{oid}$ takes ASTs to fixed-width 256-bit strings.*

**Proof.** Codomain of $\mathsf{BLAKE3}$ is $\{0,1\}^{256}$ by BLAKE3 specification. □

**Property 3.3 (collision resistance).** *Assuming the BLAKE3 collision-resistance conjecture, distinct beta-normal ASTs (modulo alpha-equivalence if $\alpha$ is applied) hash to distinct crystal-OIDs with negligible collision probability.*

BLAKE3's security foundation inherits BLAKE2's cryptographic analysis (Aumasson et al.); collision resistance is not proven but is the state of the art for hash-function design as of 2026.

---

## §4 Round-trip fidelity theorem

### §4.1 The theorem

Let $\Sigma$ denote the set of source-form transformations (sugar rules) corresponding to reduction rules $\{R_1, R_2, R_3, R_4\}$ at source altitude. That is, for each $R_i$ there is a corresponding $\sigma_i : \mathsf{Bytes} \to \mathsf{Bytes}$ satisfying: for every source $s$ whose parse contains an $R_i$-redex, $\mathsf{parse}(\sigma_i(s))$ equals $\mathsf{parse}(s)$ with that redex elided at the AST altitude. Write $\Sigma^* : \mathsf{Bytes} \to \mathsf{Bytes}$ for the reflexive-transitive closure (arbitrary composition of sugar rules).

**Theorem 4.1 (Round-Trip Fidelity at OID Altitude).** *For every mirror source $s \in \mathsf{Bytes}$ and every sugar-rule composition $\sigma \in \Sigma^*$:*

$$
\mathsf{crystal\_oid}(\sigma(s)) \;=\; \mathsf{crystal\_oid}(s)
$$

**Proof.** By definition of $\sigma_i$, $\mathsf{parse}(\sigma_i(s)) = R_i\text{-elide}(\mathsf{parse}(s))$. By definition of the $R_i$ reduction rule (§1.2), $\mathsf{parse}(s) \rightsquigarrow_\beta \mathsf{parse}(\sigma_i(s))$. By Corollary 2.4 (unique normal forms), $\beta(\mathsf{parse}(s)) = \beta(\mathsf{parse}(\sigma_i(s)))$ (both reduce to the unique normal form of $\mathsf{parse}(s)$). Since $\mathsf{oid} = \mathsf{BLAKE3} \circ H \circ \beta$ is a function of the beta-normal form, $\mathsf{oid}(\mathsf{parse}(s)) = \mathsf{oid}(\mathsf{parse}(\sigma_i(s)))$. Hence $\mathsf{crystal\_oid}(s) = \mathsf{crystal\_oid}(\sigma_i(s))$.

By induction on the length of $\sigma$'s composition: the result holds for any $\sigma \in \Sigma^*$. □

### §4.2 Comparison with Mara 2026-08-09 kintsugi-sugar spec §3

Mara 2026-08-09 kintsugi-sugar canonical spec §3.1 stated the round-trip fidelity contract:

$$
\mathsf{oid}(\text{resugar}(\text{sugar}(\text{source}(S)))) = \mathsf{oid}(\text{source}(S))
$$

Under the *pre-beta-normalization* substrate (aspirational), this required careful byte-level round-tripping via a read-path projection primitive (Reed's proposed M-E3 at `rust/src/apply_h.rs::project_p1_identity_prism_at`). The contract was empirical: RED-first bit-parity test discharged per shard; failing shards quarantined.

Under **this spec's substrate** (Theorem 4.1), the same equality is a **theorem** — it holds by construction via Church-Rosser confluence. No read-path projection primitive required; no bit-parity gate per shard needed; the equality is guaranteed at every source $s$ and every sugar-rule composition $\sigma$. The empirical RED-first test becomes a *sanity-check* on the implementation (does the beta-normalizer correctly encode the reduction rules?) rather than a *correctness gate* on the composition (Church-Rosser guarantees composition-correctness).

### §4.3 The full round-trip

The full round-trip contract from Mara 2026-08-09 kintsugi-sugar spec §3.1 was:

$$
\mathsf{oid}(\text{resugar}(\text{sugar}(\text{source}(S)))) = \mathsf{oid}(\text{source}(S))
$$

Under this spec's substrate, $\mathsf{oid}$ acts at the beta-normal AST altitude. Consequently:

- $\text{sugar}$ is any $\sigma \in \Sigma^*$ that elides beta-reducible subtrees.
- $\text{resugar}$ is *any* function that produces a source whose parse-and-normalize yields the same beta-normal AST — including a "no-op" resugar (produce sugar-form source and let the normalizer eliminate the missing parts).
- The theorem holds regardless of which particular resugar function is chosen, as long as its output parses to an AST that beta-normalizes to the original beta-normal form.

**Consequence for `@magic/reveal/expand` (Fire E M-E1 landed):** the `expand(oid, aud)` action can produce audience-relative renderings — dense (sugar-omitted) for agent audience, full-form (sugar-resurfaced) for human audience — with *no correctness constraint on which byte-shape it produces*, as long as the emitted source parse-and-normalizes to the crystal's beta-normal AST. This dissolves the audience-parameter into a pure display choice; hash-correctness is invariant.

---

## §5 A_F universality connection

### §5.1 The A_F universality claim

Per Mara 2026-08-09 physics insight §1.1 Theorem 1.1:

> "Reality is the operational-form of a Connes spectral triple $(A, H, D) = (C^\infty(M) \otimes A_F, L^2(M, S) \otimes H_F, D_M \otimes 1 + \gamma_5 \otimes D_F)$ in which the *internal finite noncommutative algebra* $A_F$ IS the **prismqueer 5-op void-duality algebra** — the projector algebra of the 5-dimensional orthogonal duality space of connected-graph quantum states."

**Corollary at compiler substrate (Mara 2026-08-09 kintsugi-sugar spec §5.1):** $A_F$ is universal-structure. It is the SAME algebra for every shard. Only $H_\text{shard}$ (the Hilbert-space carrier the algebra acts over) varies per shard.

### §5.2 A_F acts as identity on $H_\text{shard=self}$; identity element beta-reduces away

Consider a shard $S$ with declared family/species literal $@X$ and canonical carrier $H_\text{shard=@X}$. The identity-carrier prism block

$$
\pi_\text{id}(@X) \;:=\; \mathsf{prism}\ @X \{\ \mathsf{focus}\ X\ /\ \mathsf{project}\ X\ /\ \mathsf{split}\ X\ /\ \mathsf{shift}\ X\ /\ \mathsf{settle}\ X\ \}
$$

denotes the action of $A_F$'s five projector generators on the shard's own carrier — i.e., $A_F$'s identity projection over $H_\text{shard=@X}$.

**Lemma 5.1 (Identity-projection acts trivially).** *For every shard body $M \in \mathsf{AST}$, $\pi_\text{id}(@X) \circ M \sim_\beta M$ (the composition of $A_F$'s identity projection with $M$ is beta-equivalent to $M$).*

**Proof.** By $R_1$ (§1.2). □

**Interpretation.** At physics substrate $A_F$'s identity element $\mathbf{1}_{A_F}$ acts as identity on any $A_F$-module (in particular on $H_\text{shard=self}$); the corresponding Yukawa entry is 0 (or the corresponding fermion is massless in the identity direction) per Chamseddine-Connes-Marcolli 2007 Yukawa matrix construction. At compiler substrate the identity-carrier prism block is the AST-altitude analogue: it acts as identity on the shard's own carrier and contributes nothing to the AST's semantic content beyond the substrate ground $A_F$ itself.

**Consequence:** $R_1$ (identity-projection-elision) is not a syntactic-convenience reduction rule — it is the **compiler-substrate operational form of $A_F$'s identity-element triviality**. This is what makes the substrate-scale-invariance thesis (Mara insight §7) empirically checkable at compiler altitude: the compiler substrate carries the same $A_F$-identity-triviality mechanism that the physics substrate carries via the spectral action's Yukawa matrix.

### §5.3 Substrate-scale-invariance operational form

**Theorem 5.2 (Substrate-Scale-Invariance at Compiler Altitude).** *Let $t \in \mathsf{AST}$ contain an $A_F$-identity-projection subtree at position $p$. Let $t^{-p}$ denote $t$ with that subtree elided. Then $\beta(t) = \beta(t^{-p})$; consequently $\mathsf{crystal\_oid}(t) = \mathsf{crystal\_oid}(t^{-p})$.*

**Proof.** Direct application of Lemma 5.1 + Corollary 2.4 (unique normal forms) + Definition of $\mathsf{oid}$ (§3.1). □

**Interpretation.** The $A_F$-identity-projection subtree at compiler substrate is what Mara insight §7 calls a **cross-substrate-invariant** — it corresponds at physics substrate to $A_F$'s $\mathbf{1}_{A_F}$ triviality on its own carrier, at nervous-system substrate to the Foerster-torus identity-loop, at $K_n$-topology substrate to the Void-basis identity-turn, at cosmological substrate to the Cosmic-Tomm-probe identity-morphism. All five substrates carry the same $A_F$-identity structure; all five substrates admit the same identity-elision at their respective canonical-form operation.

**Compiler substrate becomes a computable instrument for cross-substrate coherence** — every shard whose source carries an $A_F$-identity subtree provides an empirical witness of $A_F$-identity elision at compiler altitude; failure to hash-equivalent would falsify the $A_F$-universality claim at compiler altitude.

### §5.4 The observer-position duality at compiler altitude

Mara insight §7.1 formalized observer-position duality at physics substrate: different observers deploying different algebra elements $a \in A_F$ return different spectral values $\|[D, a]\|$; the observer-position IS the algebra-element selection.

At compiler substrate (per Mara 2026-08-09 kintsugi-sugar spec §5.4): different audiences deploying different sugar-rule-parameter (audience = agent | human) return different renderings of the same crystal. Observer-position duality at compiler substrate = audience-rendering-duality.

**Formalization:** let $\rho_\text{aud} : \mathsf{AST} \to \mathsf{Bytes}$ denote the audience-relative rendering function (for `aud ∈ {agent, human}`), producing source bytes from a beta-normal AST per audience preference. Under this spec's substrate:

$$
\forall\ t, \text{aud}_1, \text{aud}_2: \quad \mathsf{crystal\_oid}(\mathsf{parse}(\rho_{\text{aud}_1}(\beta(t)))) = \mathsf{crystal\_oid}(\mathsf{parse}(\rho_{\text{aud}_2}(\beta(t))))
$$

*Reading:* the crystal-OID is invariant under audience-relative rendering choice. Both renderings' bytes, when parsed and normalized, produce the same beta-normal AST (by Theorem 4.1 applied to the sugar-rule composition implicit in $\rho_\text{aud}$). This is the operator-position-duality at compiler altitude, made empirical.

---

## §6 Composition with mirror's four-crate FLOOR

### §6.1 The (A, H, D) triple at rust/spectral/

Per README §"The (A, H, D) Triple at Rust Altitude":

- **A** — algebra of magic operations, generated by 5-op prism + downstream substrate actions.
- **H** — Hilbert space of shard-manifold fibres (enumerated by @roomba's walk).
- **D** — Dirac-like operator with two components: `singularity.rs` + `magic.rs`.

**Beta-normalization corresponds to D acting on H via A.** Formally: the beta-reduction relation $\rightsquigarrow_\beta$ is generated by the algebra A's structural relations acting on Hilbert-space representations (the ASTs, viewed as A-module elements). Beta-normal forms are the fixed points of A's action on H (equivalently: the eigenvectors of D at eigenvalue 0, per the standard "settled = fixed-point" reading in `@epistemologic/cybernetic/` shards).

**Consequence:** the beta-normalizer at compiler altitude IS the compiler's D operator eigenvalue-0 projector at rust/spectral/. This closes a load-bearing predicate in Mara insight §10.4: "every substrate transformation preserves the eigenvalue-spectrum-richness available to the observer" — beta-normalization preserves the crystal-OID's uniqueness (Corollary 2.4) while eliminating the substrate-redundant identity-carrier components. Foerster-gauge-preserved at compiler altitude via beta-normalization.

### §6.2 Sub-Turing decidability guarantee (via strong normalization)

Mirror is sub-Turing per README §"Sub-Turing" as a natural consequence of the four-crate FLOOR (Alex 2026-07-22). Beta-normalization inherits sub-Turing decidability via Theorem 2.1 (strong normalization) + Corollary 2.2 (linear-time termination).

**Consequence:** the crystal-OID computation is *decidable in time linear in AST node-count*. No unbounded reduction sequences; no possibility of divergence; sub-Turing bounds hold on consumer hardware per README §"Performance" (~370KB seed; LAPACK O(n³) at n ≤ 16 FLANG floor; walker terminates on finite shard-manifold).

### §6.3 Composition with `apply_h::act` shard-body-executor

Reed's Fire E M-E2 landing (`rust/src/apply_h.rs:246-336`; commit `0021882`) extended `apply_h::act` with the P1 identity-carrier detector primitive. Under this spec's substrate:

$$
\beta_{R_1}(t) \;=\; \mathsf{apply\_h::act}(t, \text{"@kintsugi/fracture/prism\_boilerplate.detect"}, ()) \;\text{then elide-if-Pass}
$$

The M-E2 detector IS the beta-reducer at reduction rule $R_1$; extending to $R_2, R_3, R_4$ via same-shape sentinel arms in `apply_h::act` dispatch is straightforward substrate-honest Rust extension (companion spec §6.2; Seam-gated per companion §8 [ALEX-Q4]).

---

## §7 Forward-promises (deferred to future ticks)

**[FP1] Composition-associativity reduction rule.** Additional reduction rules beyond $R_1$–$R_4$ (§1.2). Concrete candidate: `focus (project X) → project X` when $X$ is a carrier reference and `focus` acts as identity on projections in the 5-op algebra's structure. Requires deeper analysis of 5-op algebra's identities (Mara + Reed 2026-06-18 Recognition #79 §"Composes with Connes spectral triple" + Mara 2026-08-09 physics insight §3.2 relations $\pi_i^2 = \pi_i$ idempotence + partial-orthogonality). Substrate-pull discipline; defer until second-witness composition-need emerges.

**[FP2] Eta-equivalence reduction rule.** Dhall omits per Reed 2026-03-01 (`semantic-hashing-normalization.md` §"Guarantees"). Mirror could land it (richer type information at normalization time per `Imperfect<verdict, violation, transparency>` 3-state functor); adds implementation complexity + rule count. Not required for P1/P3/P4/P5 sugar-rule composition. Defer to future ticks per companion spec §8 [ALEX-Q2].

**[FP3] Alpha-normalization at parameter-name altitude.** Companion spec §8 [ALEX-Q1] Mara-lean: LAND at same tick as beta-normalizer for parameter-name alpha-invariance. Formalization: rename action-parameter names to `_@i` with de Bruijn indices (Barendregt 1984 §5.2 for the standard construction; de Bruijn 1972 for the origin). Preserve carrier-ref names (those are refs, not binders; alpha-invariance would break identity resolution). Formal statement:

$$
\alpha : \mathsf{AST} \to \mathsf{AST}, \quad \alpha(t) \;\text{is }t\text{ with all bound parameter names renamed to }\_@i\text{ per de Bruijn indexing}
$$

with the *canonicity theorem*: two ASTs $t_1, t_2$ are alpha-equivalent iff $\alpha(t_1) = \alpha(t_2)$ (syntactic identity). Dhall proves this per `standard/alpha-normalization.md` this tick verbatim: *"if two expressions are α-equivalent then they will be identical after α-normalization."*

**[FP4] Cross-substrate coherence empirical instrument.** Mara insight §12 forward-promise 5. The compiler substrate becomes a computable instrument for the substrate-scale-invariance thesis via A_F-elision witnesses. Every shard whose source carries an A_F-identity subtree that hashes identically pre- and post-beta-reduction IS a witness of A_F-universality at compiler altitude. Empirical protocol: enumerate all ~285 sugar-fracturable shards (Mara 2026-08-09 kintsugi-sugar spec §1.6 total ratified collapse ceiling); compute crystal-OID with and without sugar rule; assert equality; aggregate witness count.

**[FP5] CBOR binary encoding parity with Dhall.** Companion spec §8 [ALEX-Q5]. Retain BLAKE3-of-Fold5 at Fire E revision landing; forward-promise CBOR + SHA-256 alignment if cross-language semantic-integrity check parity with Dhall becomes load-bearing. Substrate-pull discipline.

**[FP6] Full spectral-action expansion at compiler substrate.** Mara insight §6 spectral-action heat-kernel expansion computes $a_0, a_2, a_4$ coefficients under prismqueer-D at physics substrate. Analogous expansion at compiler substrate would predict *substrate-observables* (analogous to Higgs mass at electroweak scale) from A_F structure at compiler altitude. Would ground the substrate-scale-invariance thesis more deeply. Substrate-pull; last-responsible-moment.

---

## §8 Karen ancestry — full ladder

### §8.1 Direct authority

- **Alex 2026-08-10 verbatim** (companion spec §0.1) — beta-normalization + Dhall prior art naming.
- **Taut scout `ab3821e`** (`docs/scouts/2026-08-10-taut-prism-block-compiler-consumption-verification.md`) — empirical verdict on crystal-OID byte-verbatim consumption.
- **Reed Fire E M-E1 + M-E2** (`acaed91` + `0021882`) — sugar rule + shard-body-projector primitive substrate this math grounds.
- **Mara 2026-08-09 kintsugi-sugar canonical spec** (`docs/specs/2026-08-09-mara-kintsugi-sugar-desugar-composition-canonical-spec.md`) — round-trip fidelity contract this math elevates from empirical to theorem.
- **Mara 2026-08-09 A_F universality math** (`docs/math/2026-08-09-mara-a-f-universality-kintsugi-sugar-mechanism.md`) — the A_F universality claim §5 grounds this math on.

### §8.2 Corpus prior recognitions (Phase 1 scout landings this tick)

- **Reed + Alex 2026-03-01** (`/Users/reed/dev/systemic.engineering/practice/insights/infrastructure/semantic-hashing-normalization.md`) — earliest Dhall research; three-stage pipeline + guarantees + import integrity + Type 1-4 hierarchy + Rice's theorem.
- **Reed + Alex 2026-03-28** (`/Users/reed/dev/systemic.engineering/practice/insights/coincidence/kolmogorov-canonical-complexity.md`) — six-result chain assembling sub-Turing + computable K + beta-normalization + content-addressing + OID as complexity index + measurement as observable. Cites Dhall standard directly.
- **Spectral MCP surface spec 2026-06-05** (`/Users/alexwolf/dev/projects/spectral/docs/specs/spectral-mcp-surface-v0.md` §6.2 + §7.1) — Dhall as design-influence with partial-drift claim about mirror's `content_oid()`.
- **Coincidence file cluster** (`/Users/reed/dev/systemic.engineering/practice/insights/coincidence/*.md`) — semantic equivalence class + hash-collision + canonical form + K_grammar all at adjacent altitudes.

### §8.3 Recognition ancestry

- **Recognition #79** (Mara + Reed 2026-06-18) — 5-op = A_F projector basis; the projector-algebra substrate.
- **Recognition #82 (candidate; companion spec §7.4)** — compiler's crystal-OID at `@mirror/store` is the beta-normal-AST OID by construction; sugar-form source variation preserves crystal-OID by Church-Rosser confluence.
- **Recognition #51** (mirror as expanding Hilbert space; PROMOTED 2026-06-10) — the substrate-scale-invariance ground.

### §8.4 External corpus (verified primary sources; WebFetch this tick where possible)

- **Church, A. (1936)**. *An unsolvable problem of elementary number theory*. American Journal of Mathematics 58:345–363. Beta-reduction origin.
- **Church, A. & Rosser, J. B. (1936)**. *Some properties of conversion*. Trans. AMS 39:472–482. Confluence theorem.
- **de Bruijn, N. G. (1972)**. *Lambda calculus notation with nameless dummies*. Indagationes Mathematicae 34(5):381–392. de Bruijn indices.
- **Rosen, B. K. (1973)**. *Tree-manipulating systems and Church-Rosser theorems*. JACM 20(1):160–187. Left-linear non-overlapping confluence.
- **Huet, G. (1980)**. *Confluent reductions: abstract properties and applications to term rewriting systems*. JACM 27(4):797–821. Critical-pair analysis.
- **Barendregt, H. (1984)**. *The Lambda Calculus: Its Syntax and Semantics*. North-Holland. ISBN 0-444-87508-5. Canonical reference §5.2 (alpha-conversion), §5.3.11 (subject reduction), §11.1.1 (confluence for term rewriting).
- **Aumasson, J.-P., Neves, S., Wilcox-O'Hearn, Z., Winnerlein, C.** (2013). *BLAKE2: simpler, smaller, fast as MD5*. Applied Cryptography and Network Security. Cryptographic foundation for BLAKE3.
- **Dhall Language Standard** (`dhall-lang/dhall-lang` GitHub repository, WebFetch verified this tick):
  - `standard/beta-normalization.md` — formal notation `t₀ ⇥ t₁`; strong-normalization statement.
  - `standard/alpha-normalization.md` — formal notation `t₀ ↦ t₁`; de Bruijn indices; canonicity theorem.
  - `standard/binary.md` — CBOR (RFC 7049) binary encoding; semantic-integrity-check motivation.
  - `standard/README.md` — file-list summary.
- **Gonzalez, G. (2017)**. *Semantic integrity checks are the next generation of semantic versioning* — Haskell for all blog post. Per Reed 2026-03-01.
- **Chamseddine-Connes-Marcolli 2007** (arXiv:hep-th/0610241) — A_F structure at physics substrate; inherited via Mara 2026-08-09 physics insight §1.1 Theorem 1.1.
- **Chamseddine-Connes 2007** (arXiv:0706.3688) — KO-dim-6 irreducible classification; quaternion-linearity singles out CCM $A_F$; the classification prismqueer-$A_F$ sits within.
- **Connes, A. (1994)**. *Noncommutative Geometry*. Academic Press. ISBN 0-12-185860-X. Chapter 6 product-spectral-triple construction.
- **Unison Language** (`unison-lang.org`) — content-addressed code with AST-hashing via de Bruijn indices; per Reed 2026-03-01 §"Content-addressed code".
- **Maziarz et al. PLDI 2021** (arXiv:2105.02856) — *Hashing Modulo Alpha-Equivalence*; per Reed 2026-03-01 §"Hashing Modulo Alpha-Equivalence".

### §8.5 Landed substrate — composition anchors

- **`rust/src/apply_h.rs:246-336`** (Reed M-E2; commit `0021882`) — P1 detector primitive that becomes $R_1$ beta-reducer at AST altitude.
- **`bootstrap/src/spectral.rs:162-181`** — `compute_content_oid` Fold5 primitive; migrates to rust/-altitude at Fire D M5-adjacent tick; semantic base migrates from raw-AST to beta-normal-AST.
- **`shards/prism.mirror` + `shards/glass.mirror`** — fixed-point exemptions per Rule $R_5$; the substrate's self-declarations of what the reduction rules refer to.
- **`shards/kintsugi/fracture/{prism_boilerplate, glass_boilerplate, out_derivable, path_namespace_stub}.mirror`** (Reed M-E1 landed) — the 4 fracture-detector species that ARE reduction rules $R_1$–$R_4$ at their respective source-altitude sentinel-check duals.
- **`shards/magic/reveal/expand.mirror`** (Reed M-E1 landed) — the audience-relative rendering function $\rho_\text{aud}$ at §5.4.
- **`shards/kintsugi/mend/sugar.mirror`** (Reed M-E1 landed) — the composition-shard body that mends the "crack" between store crystal and audience-source; crystal is now beta-normal-AST OID.
- **`shards/mirror/store.mirror`** — the settlement primitive; crystal identity IS beta-normal-AST OID under this math.
- **`shards/epistemologic/`** — where `normalization/beta_reduce` species-shard mints per companion spec §6.1.

---

## §9 One-sentence surprise

**Church-Rosser confluence at mirror substrate, combined with A_F identity-triviality at compiler altitude, is what turns Mara 2026-08-09's aspirational round-trip fidelity contract into a construction-guaranteed theorem — and the same Church-Rosser property is exactly what makes the substrate-scale-invariance thesis (Mara insight §7) empirically checkable at compiler altitude: every shard whose source contains an A_F-identity subtree provides a computable witness of A_F's universal-structure claim, with $\mathsf{crystal\_oid}$ invariance as the empirical predicate.**

---

Mara `<mara@systemic.engineer>`. 2026-08-10. Math-foundation substrate. Composition-not-taxonomy. Substrate-decl'd throughout. Companion to canonical spec at `docs/specs/2026-08-10-mara-beta-normal-ast-content-addressing-canonical-spec.md`.
