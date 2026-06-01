# SpectralHash design — three properties, the impossibility, and the recommendation

*2026-05-30. Mara. Research + design spec. Upstream of `coincidence-hash-collapse.md`
(commit `e9c259b`). No implementation; markdown only.*

---

## RECOMMENDATION REWRITE — 2026-05-30 (LRM collapse, same day)

**Status:** the §4–5 *research* still stands (the survey, the tradeoff matrix, the
Motwani-Naor-Panigrahy 2006 impossibility result). The original §6 *recommendation* —
a composite `ContentOid { storage, navigation }` carrying both fields on every
content address — is **SUPERSEDED** as of 2026-05-30 by a conversation between Alex
and Reed that LRM-collapsed several over-built proposals (this one included) into a
simpler architecture. The new architecture is captured in
`docs/specs/store-vs-db-and-the-cascade.md`. The headline:

1. **`@mirror/store` is the storage gate** — open content-addressed foundation,
   verification on write, git-backed by default, MUST work without `@spectral/db`.
   Verification belongs HERE — the c3a01e3 framing that "verification belongs to
   `@spectral/db`" (§1, §6.2, §10) is **wrong** and is corrected throughout this
   amendment. Storage uses `Merkle<BLAKE3>` — sidesteps Attack 1 entirely.
2. **`@spectral/db` is the engine on top** — potentially closed source, navigation
   over a spectral graph, lives ABOVE the storage gate. Its navigation primitive is
   **`VoidPointer`** (a name for the spectral coordinate that the existing
   `SpectralCoordStore` + `coord_oids` + `spectral_distance_eigen` machinery already
   computes). VoidPointer is NOT a hash function; it is a spectral coordinate.
3. **No composite `ContentOid`.** Instead: the Merkle tree is **generic over the
   hash algorithm** (`Splinter<H>`, `Content<H>`, `Body<H>`, `Crystallization<H>`,
   `Crystallizations<H>`). Each consumer picks its own hash primitive; different
   consumers can have structurally different trees. Per Alex: *"If we make the
   AST/MerkleTree generic over the hash algorithm... then everything else falls
   out."* This is a Rust change, expected and welcomed.
4. **`ScalarLoss` → `Transparency` (as a Lens)** is pinned for the *next* tick
   after the cascade. Positive-frame (light passes vs absence-of-light),
   optical-family-native, natural dual of Dark spans, lens-algebra-composable.
   Noted but not baked in here.
5. **Attack 1 narrowing.** §3.1's Attack 1 verdict is *real* but **narrower than
   originally stated**. Collisions require two inputs whose double-hashed-accumulated
   5-D vectors project to coefficient vectors that round to the same bits under
   EPSILON gating, all five times in parallel — not "two inputs that round the
   same." Still cryptographically weaker than naked SHA-256, which is why
   `@mirror/store` adopts BLAKE3 (sidesteps the surface).
6. **CoincidenceHash sites stay where they are.** The two existing sites (`prism_core`'s
   `Detector<3>` and `bootstrap`'s `<5,5>`) are no longer `@mirror/store`'s hash;
   whether they ever unify is a separate (now lower-priority) concern. The CHC
   collapse plan in `coincidence-hash-collapse.md` is itself SUPERSEDED.

The inline §6/§7/§8.4/§9/§10 text below has been amended to reflect this. Sections
§1–§5 (motivation, what "navigatable" means, current state, design space, lower
bound) stand verbatim — that research is load-bearing for the new architecture too.

Landing-page spec: **`docs/specs/store-vs-db-and-the-cascade.md`**.
Cascade implementation tick: forthcoming.

---

**Original-status (pre-rewrite) summary:** draft — Alex called the framing on 2026-05-30.
This spec answered it. The same-day conversation then collapsed the answer further;
see the amendment above.

**Scope (original):** the question "can one hash satisfy (1) collision-resistance,
(2) speed, and (3) graph-navigability simultaneously, or does mirror need pluggable
per-operation hashes?" The literature answer (one hash cannot) stands. The original
recommendation packaged the pluggable answer as a single composite type; the new
architecture refuses that packaging — see the rewrite above.

**The Attack 1 verdict** (open question from the adversarial review of CHC):
`canonical_hash` does **NOT** reduce to `sha256(canonical_byte_form)`. It SHA-256s the
f64 bit patterns of the projected coordinates. IEEE-754 rounding under EPSILON is a
live collision surface, but smaller than originally framed (see §3.1's narrowing
note). `@mirror/store` adopts BLAKE3 to sidestep the question entirely.

**Touches no `.rs` in this commit.** Markdown only. The cascade (generic-over-H)
DOES touch `.rs` when it lands; that is its own tick stream.

---

## 1. Motivation

Mirror's content-addressing hash needs to satisfy three properties simultaneously:

1. **Collision-resistant.** Storage security. You can't forge two different
   `Content` values with the same OID. This is the standard cryptographic requirement.
   For mirror specifically: every dispatcher call routes on OID, every Splinter parent
   commits to children's OIDs, every `@hash/coincidence` check verifies content
   identity. Collision = the entire address space collapses. SHA-256-grade strength
   is the floor.
2. **Fast to compute.** Production hot-path use. Every dispatcher call, every storage
   write, every OID lookup, every Merkle composition step. Throughput matters at the
   substrate altitude where this primitive lives. The CHC code review in `e9c259b`
   §3.2 counted ~5 callsites in `bootstrap/src/main.rs` and per-AST-node usage in
   `bootstrap/src/spectral.rs::compute_oid_inner` — at compile, every AST node hashes,
   and every parent recursively. "Fast" here means *not* "as fast as a memcmp" but
   *"not orders of magnitude slower than SHA-256 on the same input"*.
3. **Graph-navigatable.** Usable as a *coordinate* in `@spectral/db`'s edge graph.
   Near-OIDs should approximately mean near-content. Supports traversal, similarity
   queries, locality. The hash isn't just an address; it's a location in a navigable
   space. §2 defines what "navigatable" means precisely in spectral-db's terms.

**Best case:** one hash satisfies all three. **Fallback:** pluggable per-operation hash
(different hashes for storage vs navigation vs AST identity, with one shared
canonicalization step).

The failure modes that produce the tension:

- **Cryptographic hashes (SHA-256) violate (3) by design.** The avalanche property —
  every input bit affects every output bit with probability ½ — is precisely the
  property that destroys (3): two near contents must produce far hashes.
- **Navigation hashes (LSH, simhash, learned hashes) violate (1) by design.** Near
  contents must collide (that's what makes them "locality-sensitive"). Controllable
  collisions are a feature; from a cryptographic perspective they are pre-image
  weaknesses.
- **The middle is what this spec is hunting.**

Architectural context (settled 2026-05-30 with Alex, **corrected later same day**):

- **Verification belongs in `@mirror/store`, not in `@spectral/db` and not in the
  data type.** `@mirror/store` is the open content-addressed storage gate — the
  foundation that mirror MUST work without `@spectral/db`. `@spectral/db` is the
  engine on top. Splinter stays loose in flight; the store is the gate; recompute
  on write and reject on mismatch lives there. This corrects the earlier framing
  (in this spec's pre-amendment §6.2 and §10) that placed verification in
  `@spectral/db`. See `docs/specs/store-vs-db-and-the-cascade.md` for the full
  store-vs-db distinction.
- **Numerical computation belongs in the Fortran layer** (prism's gfortran kernels
  now, flang at LRM per Track J — see `docs/specs/numerical-substrate-via-fortran.md`).
  If `@spectral/db`'s `VoidPointer` (the spectral coordinate) has real spectral
  structure, its computation lives there. §8 specifies which Fortran kernel hosts
  what — note that under the rewrite, this concerns `@spectral/db`'s `VoidPointer`,
  not `@mirror/store`'s storage hash (which is BLAKE3, no Fortran involvement).

---

## 2. spectral-db's `navigatable_oid` — what "navigatable" actually means here

### 2.1 Code-read finding: the symbol does not exist as written

There is **no `navigatable_oid` (nor `navigable_oid`) symbol** in `spectral-db`'s
source, docs, tests, or `db.conv` grammar. The reference Alex pointed at is
*conceptual*, not literal. It maps to a small set of concrete primitives:

- `src/index.rs` — `SpectralIndex::near(target_oid, max_distance)` — finds nodes
  spectrally near a target.
- `src/index.rs` — `SpectralIndex::spectral_distance_eigen(oid_a, oid_b, coord_store)` —
  L2 distance between two nodes' eigenvalue vectors.
- `src/spectral_store.rs` — `SpectralCoordStore` — content-addressed storage for
  eigenvalue vectors. Keys are *separate* OIDs (SHA-1 git blob OIDs of the f64-byte
  encoding of the vector), keyed *by node OID*.
- `db.conv` (the grammar) — `action near { target: oid, distance: node_type }` and
  `action walk { from: oid, depth: node_type }` — the verbs that consume navigation.

Quote, from `spectral-db/src/index.rs` lines 1-5:

> ```
> //! Spectral indexing — hash and adjacency tracking.
> //!
> //! Uses SHA-256 as the spectral hash for content indexing.
> //! When the coincidence crate's eigenvalue computation is needed
> //! for richer spectral similarity, swap the hash function here.
> ```

Quote, from `spectral-db/src/index.rs::SpectralIndex::near` (lines around the
"prefer eigenvalue-based distance" branch):

> ```rust
> /// Find nodes spectrally near a target (eigenvalue distance < epsilon).
> ///
> /// Uses eigenvalue-based distance when spectral eigenvalues have been
> /// computed (via `recompute_spectral_hashes`). Falls back to SHA-256
> /// byte distance only when eigenvalues are unavailable.
> ```

The "swap the hash function here" comment IS the open architectural slot this spec
fills. "Navigatable OID" is the *shape* spectral-db wants the swap to produce.

### 2.2 What navigation actually requires (from the code, not the vibes)

Reading `SpectralIndex` and `SpectralCoordStore`:

1. **`near(target, ε)`** — given a node OID and an epsilon, return all node OIDs whose
   navigational distance to `target` is < ε. Linear scan today (`self.hashes.iter()`
   for SHA-256 fallback, `self.spectral_eigenvalues.iter()` for the eigenvalue path).
   No spatial index. Time is O(N) per query at any scale, so the navigation hash
   does NOT need a tree structure embedded in the hash itself — but it MUST support
   distance comparison cheaply.
2. **`spectral_distance_eigen(a, b)`** — pairwise L2 distance over the eigenvalue
   vector stored at each node's `coord_oid`. The navigation primitive is *distance
   over coordinates*, not distance over OIDs themselves.
3. **`coord_oids: HashMap<String, String>`** — the architecture today is
   **OID → coord_OID**, a layer of indirection. The node's storage-OID is plain
   SHA-256-of-content; the navigation coordinate is a separate content-addressed
   blob (eigenvalues of the ego Laplacian, serialized as little-endian f64 bytes,
   git-SHA-1'd). One node has two addresses.
4. **Coordinate computation is bounded by budget** —
   `compute_spectral_coordinates(budget, coord_store)` decides which nodes get
   coords based on `SpectralBudget` capacity (ego subgraph size threshold, total
   memory). Not every node has navigation coords. "Navigatable" is a *capability*
   that nodes acquire when budget allows; it's not a property of the storage-OID.

### 2.3 The precise definition

**For mirror's purposes**, "`navigatable_oid`" is the slot that satisfies:

- **Inputs:** raw content bytes of a node.
- **Outputs:** a fixed-width coordinate in some metric space M such that L2 (or
  Hamming, depending on M's nature) distance in M correlates with semantic distance
  of the input contents.
- **Use-sites:** `near(target, ε)`, `walk(from, depth)` with spectral-weighted edge
  expansion, future `cluster(seed_set)` for partition discovery.
- **Computability:** must be derivable from content bytes *without* requiring graph
  context (otherwise it can't serve as an *address*; it'd be a graph property, not
  an OID). The current eigenvalue-vector implementation FAILS this — it requires the
  ego subgraph to be built first, so the coordinate is a graph property, not a
  content property.

**Spectral-db today does NOT have a true `navigatable_oid` primitive.** It has:

- A storage hash (SHA-256 of content bytes), called `spectral_hash` aspirationally.
- A separate coordinate store keyed by storage hash, populated lazily by graph-aware
  computation.

The "swap the hash function here" comment in `index.rs` is asking for a hash that
produces a navigatable coordinate **from content alone, without graph context**.
That's the contract this spec must specify.

---

## 3. The current state — does CoincidenceHash satisfy any of the three?

Three implementations live in the workspace (mapped in detail by
`coincidence-hash-collapse.md` §2). Recap:

1. `prism_core::coincidence::Detector<3>` with `dim=16` in space `"content"` —
   backs `prism_core::Oid::hash`.
2. `bootstrap::hash::canonical_hash` — dense `<5,5>` — backs all bootstrap content
   addresses (source, AST nodes, combinator trees).
3. `bootstrap::crystallize::compute_oid` — raw SHA-256 over a Merkle framing —
   backs Splinter (Tick A, 2026-05-29).

### 3.1 Property (1) — collision-resistance

**Reduces to SHA-256-of-something for all three.** The final-stage compression is
`Sha256(prefix || payload)` in every case. So the collision-resistance of the OID
is at most the collision-resistance of SHA-256 — which is 2^128 for collision
resistance, 2^256 for pre-image.

But: **`canonical_hash` introduces a collision surface SHA-256 doesn't have.**

From `bootstrap/src/hash.rs::canonical_hash` (annotated for the attack surface):

```rust
pub fn canonical_hash(data: &[u8]) -> String {
    let projs = canonical_projections();
    let coeffs = encode_into_basis(data);       // bytes → [f64; 5]

    if vec_is_zero(&coeffs) {                   // dark fallback
        let mut h = Sha256::new();
        h.update(b"prism-core:dark:");
        h.update(data);
        return hex_str(&h.finalize());
    }

    let mut focus_results = [[0.0_f64; DIM]; NUM_PROJECTIONS];
    let mut any_zero = false;
    for p in 0..NUM_PROJECTIONS {
        projection_apply(&projs[p], &coeffs, &mut focus_results[p]);  // [f64;5]→[f64;5]
        if vec_is_zero(&focus_results[p]) {     // dark fallback on collapse
            any_zero = true;
            break;
        }
    }
    if any_zero { /* dark fallback */ }

    // SHA-256 over f64 bit patterns of projected coordinates:
    let total = 12 + 8 + NUM_PROJECTIONS * DIM * 8;
    let mut buf: Vec<u8> = Vec::with_capacity(total);
    buf.extend_from_slice(b"coincidence:");
    let mut n_bytes = [0u8; 8];
    u64_le(NUM_PROJECTIONS as u64, &mut n_bytes);
    buf.extend_from_slice(&n_bytes);
    for p in 0..NUM_PROJECTIONS {
        for j in 0..DIM {
            let bits = focus_results[p][j].to_bits();  // ← THE ATTACK SURFACE
            let mut bbytes = [0u8; 8];
            u64_le(bits, &mut bbytes);
            buf.extend_from_slice(&bbytes);
        }
    }

    let mut h = Sha256::new();
    h.update(b"prism-core:coincidence:");
    h.update(&buf);
    hex_str(&h.finalize())
}
```

**Verdict on Attack 1:** the final SHA-256's input is **`b"prism-core:coincidence:" ||
b"coincidence:" || u64_le(NUM_PROJECTIONS) || (focus_results.to_bits() as bytes)`**.
The `focus_results` array is the projected `f64` coordinates. Two inputs `a ≠ b`
that happen to project to *bit-identical* `focus_results` produce the same final hash.
This is not a SHA-256 collision; it is a **collision in the projection step before
SHA-256 ever runs**.

When does this happen in practice?

- **EPSILON gate** (line ~131 of `hash.rs`): `if out[i].abs() <= EPSILON { out[i] = 0.0 }`.
  Any two inputs whose projection differs by less than machine epsilon collapse to
  the same coordinate vector. EPSILON = 2.2204460492503131e-16. The encoding step
  `encode_into_basis` accumulates per-byte projections via floating-point addition,
  so for inputs with `|coeff_a - coeff_b| < EPSILON` after summation, the projected
  outputs are bit-identical.
- **Dark-fallback degeneracy**: any input whose projection produces ANY zero output
  vector takes the `b"prism-core:dark:"` branch, which is `Sha256(b"prism-core:dark:"
  || data)`. This branch IS plain SHA-256. So a `canonical_hash` collision requires
  *both* inputs to take the same branch (live or dark) AND produce the same
  post-branch payload.
- **Adversarial construction**: given the seeded projections are public
  (deterministic from `"coincidence:projection:{i}:{NUM_PROJECTIONS}"`), an attacker
  can compute the projection matrix exactly. Finding inputs `a, b` with
  `encode_into_basis(a) == encode_into_basis(b)` after the EPSILON clamp is a linear
  algebra problem over the projection's null-space (with a quantization bound). The
  encoding is `Σ_i SHA-256-derived-projection(byte_i)` accumulating into 5 floats —
  cancellation in the f64 accumulator is achievable with crafted input.

**Conclusion on (1):** `canonical_hash` is **weaker than SHA-256** for collision-
resistance — but the attack surface is narrower than first stated. The projection
step introduces a controllable collision surface; the collision-resistance is bounded
by the smaller of (SHA-256 of the projection output) and (the size of the
projection's null-space relative to the EPSILON quantization). Plain SHA-256 of the
input bytes (or the Splinter raw-SHA-256 approach) does NOT have this surface.

**Narrowing (2026-05-30, post-rewrite).** Re-reading the actual code carefully:
`canonical_hash` DOES depend on float coordinates (the final SHA-256 consumes
`focus_results[p][j].to_bits()`), but the projections come from `encode_into_basis`
which already runs per-byte SHA-256 — so collisions require two inputs whose
double-hashed-accumulated 5-D vectors project to coefficient vectors that round to
the same bits under EPSILON gating, **all five times in parallel**. That is narrower
than "two inputs that round the same after one projection." The original wording
overstated the practical attack surface.

This still answers the open question in CHC's adversarial review: **Attack 1 is real**,
the coincidence-projection step IS a collision surface beyond SHA-256, but the
adversarial construction must simultaneously satisfy five EPSILON-clamped equations
over SHA-256-seeded random projections — not a single one. The collision-resistance
bound stated above is correct as a bound; the *practical* exploitability sits well
below what one might naively read.

**Bottom line for the rewrite.** `@mirror/store` adopts `Merkle<BLAKE3>` and
sidesteps this surface entirely. The question of whether the existing `<5,5>` or
`<3,16>` coincidence sites are exploitable in practice becomes lower-priority—they
are no longer the storage hash. See the amendment banner.

### 3.2 Property (2) — speed

All three have a SHA-256 step. The differences are in what feeds SHA-256:

| primitive | per-call work | SHA-256 input size |
|---|---|---|
| Splinter raw SHA-256 Merkle | 1× SHA-256 over (tag + len + children-OIDs) | ~32-128 bytes |
| `Detector<3>::review` (substrate) | 3× projection-apply (16-dim sparse map) + 1× SHA-256 over eigenvalue bytes | 12 + 8 + 3·16·8 = ~404 bytes |
| `bootstrap::canonical_hash` `<5,5>` | 5× projection-apply (5-dim dense array) + 1× SHA-256 over eigenvalue bytes | 12 + 8 + 5·5·8 = ~220 bytes |

The projection step is `O(DIM × NUM_PROJECTIONS)` floating-point multiplies. For
`<5,5>` that's 25 multiplies plus 25 adds — negligible vs SHA-256's per-block work
(64 ops/block on 64-byte blocks). The `encode_into_basis` step is the dominant cost:
for each input byte it does NUM_PROJECTIONS SHA-256s to seed the projection, then
DIM more SHA-256s for the dimensional decomposition. Per `bootstrap/src/hash.rs:encode_into_basis`,
that's `1 + DIM` SHA-256s per input byte. For `<5,5>` and a 1KB input, encode_into_basis
costs ~6000 SHA-256s — orders of magnitude more than plain SHA-256 of 1KB (~16
blocks).

**Verdict on (2):** the coincidence hash is **~100-1000× slower than plain SHA-256
on typical inputs**. Splinter's raw SHA-256 Merkle is the fast option. The bootstrap's
`<5,5>` is in the "acceptable for compile-time, prohibitive for runtime hot-path"
range. The substrate's `<3,16>` is even slower due to BTreeMap allocation per
projection apply.

The speed property is **only satisfied by the Splinter / raw-SHA-256 path**.

### 3.3 Property (3) — graph-navigability

The `canonical_hash` output IS the SHA-256 of the projection. After the SHA-256
compression, navigability is **zero** — by SHA-256's avalanche property, near
projections produce far hashes.

The *intermediate* eigenvalue bytes (the `focus_results` array, before SHA-256) DO
have navigation structure — `focus_results_a` and `focus_results_b` are L2-close iff
the original inputs project to similar coordinates in the 5-dim projection space.
But **`canonical_hash` does not expose this intermediate**. The output of
`canonical_hash` is the post-SHA-256 hex, which is uniformly random by design.

The spectral-db `coord_oids` indirection is precisely the workaround for this: the
storage-OID is the unnavigable post-SHA-256 hex, the navigation coordinate is the
separately-stored eigenvalue vector. Two addresses per node.

**Verdict on (3):** **no current primitive satisfies (3)**. The coincidence hash
*nearly* does — its intermediate projection IS a navigation coordinate — but the
final SHA-256 destroys that. The current architecture works around this by storing
the coordinate separately, paying a 2× storage cost and a graph-context-dependent
computation cost.

### 3.4 Summary of the current state

| primitive | (1) collision-resistant | (2) fast | (3) navigatable |
|---|---|---|---|
| Splinter raw SHA-256 Merkle | ✓ (SHA-256-grade) | ✓ | ✗ (avalanche) |
| `Detector<3>::review` | ✗ (projection surface) | ✗ (slow) | ✗ (avalanche) |
| `canonical_hash` `<5,5>` | ✗ (projection surface) | ✗ (slow) | ✗ (avalanche) |
| spectral-db `coord_oids` (composite) | n/a (it's a pointer) | ✗ (requires graph) | ✓ (L2 in eigenvalue space) |

**Zero of three primitives satisfy all three properties. Two satisfy one each;
zero satisfy two.** The current architecture is already implicitly pluggable — it
just hides the plumbing in the `coord_oids` indirection and pretends the
`canonical_hash` is doing "spectral" work it isn't.

---

## 4. The design space — survey + tradeoff matrix

### 4.1 Survey (Kagi research, 10-15 citations)

**Locality-Sensitive Hashing (LSH) — Indyk-Motwani 1998 onward**

- Indyk, P. & Motwani, R. (1998). *Approximate nearest neighbors: towards removing
  the curse of dimensionality.* STOC '98.
  https://dl.acm.org/doi/10.1145/276698.276876 — the original LSH formulation. Hash
  functions with the property that `Pr[h(x) = h(y)]` is monotone in `||x - y||`.
  Cryptographically broken by design — that's the point.
- Andoni, A. & Indyk, P. (2008). *Near-Optimal Hashing Algorithms for Approximate
  Nearest Neighbor in High Dimensions.* CACM.
  https://people.csail.mit.edu/indyk/p117-andoni.pdf — survey + the
  (ρ_q, ρ_u)-tradeoff between query time and space.
- Andoni, A., Indyk, P., Laarhoven, T., Razenshteyn, I., Schmidt, L. (2015).
  *Practical and Optimal LSH for Angular Distance.* NIPS.
  https://arxiv.org/abs/1509.02897 — the cross-polytope LSH, optimal ρ for angular
  distance. Relevant because the coincidence projection is angular (rank-1 projections
  preserve angle, not magnitude).

**Set similarity — MinHash (Broder 1997)**

- Broder, A. (1997). *On the resemblance and containment of documents.* The original
  MinHash. https://en.wikipedia.org/wiki/MinHash — Jaccard similarity estimation via
  `min` over random permutations. Cryptographic strength: none. Speed: extremely fast
  (one comparison per permutation). Navigation: very good for set-distance.

**Cosine similarity — SimHash (Charikar 2002)**

- Charikar, M. (2002). *Similarity Estimation Techniques from Rounding Algorithms.*
  STOC. https://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf —
  signed hyperplane LSH. `h(x) = sign(r · x)` for random `r`. Output bits estimate
  cosine similarity. Used widely in Google's near-duplicate detection.

**Spectral Hashing — Weiss-Torralba-Fergus (NIPS 2008)**

- Weiss, Y., Torralba, A., Fergus, R. (2008). *Spectral Hashing.* NIPS.
  https://people.csail.mit.edu/torralba/publications/spectralhashing.pdf — derives
  binary codes from thresholded eigenvectors of the data graph's Laplacian.
  Designed to preserve neighborhood structure. **Critical caveat:** the hash is
  *learned from the data distribution*. Out-of-distribution inputs produce
  unpredictable codes. Cryptographic strength: none (the eigenvector structure is
  the inverse of avalanche).
- Bodó, Z., Csató, L. (2013). *Linear Spectral Hashing.* ESANN.
  https://www.esann.org/sites/default/files/proceedings/legacy/es2013-113.pdf —
  closed-form spectral hashing for streaming inputs.

**Learning-to-hash (2020-2026)**

- Various surveys on deep hashing for cross-modal retrieval (e.g., EGATH NeurIPS 2024:
  https://proceedings.neurips.cc/paper_files/paper/2024/file/03e7eaa586f0990c633f8a8e57e08ca6-Paper-Conference.pdf).
  The 2024-26 frontier produces hashes optimized for retrieval; none claim cryptographic
  collision-resistance. The defining property is that the learned hash *aligns* with
  task semantics, which is opposite to (1).
- Pham, P. (2024). *TopLoc: A Locality Sensitive Hashing Scheme for Trustless
  Verifiable Inference.* arXiv 2501.16007.
  https://arxiv.org/html/2501.16007v1 — closest 2024-26 work to mirror's hybrid
  ambition. LSH used as a *commitment* scheme for ML inference verification.
  Achieves "verifiability" by combining a TopK LSH commitment with a separate
  cryptographic envelope; the commitment is not itself collision-resistant but the
  envelope is. **Architecturally analogous to spectral-db's coord_oids split.**

**Cryptographic hashes with controlled structure**

- Bertoni, G., Daemen, J., Hoffert, S., Peeters, M., Van Assche, G., Van Keer, R.
  (2016). *KangarooTwelve: fast hashing based on Keccak-p.*
  https://eprint.iacr.org/2016/770 — a sponge-construction hash designed for speed
  while preserving SHA-3-grade security. Tree-parallel by construction (the
  "kangaroo" parallel paths). No locality-sensitivity; collision-resistance is the
  whole point.
- BLAKE3 (O'Connor et al., 2020). https://github.com/BLAKE3-team/BLAKE3 —
  Merkle-tree hash with parallelism. ~2-5× faster than SHA-256 on modern hardware
  (https://news.ycombinator.com/item?id=38249473). Same property profile as SHA-256:
  cryptographic + fast, zero navigation.

**Content-defined chunking (structural but not collision-resistant)**

- Xia, W., Jiang, H., Feng, D. et al. (2016). *FastCDC: a Fast and Efficient
  Content-Defined Chunking Approach for Data Deduplication.* USENIX ATC.
  https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf — a
  rolling hash (Gear-based) that finds *content-defined* chunk boundaries.
  Structure-preserving in a different sense: small content changes produce small
  chunk-boundary shifts. Not used for content addressing; used for deduplication.

**Graph hashing — Weisfeiler-Lehman + GNN hashing**

- Weisfeiler, B., Leman, A. (1968). *A reduction of a graph to a canonical form and
  an algebra arising during this reduction.* https://en.wikipedia.org/wiki/Weisfeiler_Leman_graph_isomorphism_test —
  the color-refinement algorithm. WL-hash is the canonical "hash that respects graph
  isomorphism" baseline.
- Morris, C., Lipman, Y., Maron, H., et al. (2022). *A Short Tutorial on the
  Weisfeiler-Lehman Test and Its Variants.* arXiv 2201.07083.
  https://arxiv.org/pdf/2201.07083 — WL as upper bound on expressiveness for GNNs.
- HashGIN (2026). *Uncovering capabilities of hash function in graph classification.*
  https://www.sciencedirect.com/science/article/abs/pii/S0031320325016978 —
  one-epoch trainable GNN-hash. Not relevant to content-OID use; relevant to
  navigation in pre-classified graph contexts.

**Lower bounds and the fundamental tradeoff**

- Motwani, R., Naor, A., Panigrahy, R. (2006). *Lower Bounds on Locality Sensitive
  Hashing.* SICOMP. https://arxiv.org/abs/cs/0510088 — establishes the ρ ≥ 1/c²
  lower bound for LSH in Euclidean space (where `c` is the approximation factor).
  Translation: LSH cannot be arbitrarily collision-resistant for far points AND
  arbitrarily collision-sensitive for near points. **This is the load-bearing
  impossibility, restated below in §5.**
- O'Donnell, R., Wu, Y., Zhou, Y. (2014). *Optimal Lower Bounds for Locality-Sensitive
  Hashing (Except When q Is Tiny).* https://arxiv.org/pdf/1605.02687 — tightens the
  lower bound; the (ρ_q, ρ_u) tradeoff between query exponent and update exponent.

**Adversarial LSH and perceptual hash attacks**

- Aamand, A., Indyk, P., et al. (2025). *On the Adversarial Robustness of
  Locality-Sensitive Hashing in Hamming Space.*
  https://dl.acm.org/doi/10.1145/3725239 — demonstrates a simple adversary that finds
  near-collisions exponentially faster than random sampling. Confirms LSH is
  cryptographically broken.
- Prokos, J., Fendley, N., Green, M., Schuster, R., Tromer, E., Jain, A., Cao, Y.
  (2024). *Squint Hard Enough: Attacking Perceptual Hashing with Adversarial
  Inputs.* USENIX Security. https://gangw.cs.illinois.edu/class/cs562/papers/phash.pdf —
  black-box attacks on PhotoDNA and PDQ. **Direct evidence that navigation-friendly
  hashes are vulnerable to inversion and targeted-collision attacks.**
- Drăgoi, A., et al. (2024). *Black-box Collision Attacks on Widely Deployed
  Perceptual Hash Functions.* IACR ePrint 2024/1869.
  https://eprint.iacr.org/2024/1869 — extends to PhotoDNA's 1152-bit hash. Near-
  collisions in `O(N)` queries for small `N`.

**Tensorized random projection (recent 2024)**

- Garg, A., Kapralov, M., Loikkanen, P., Quanrud, K., Sidiropoulos, A. (2024).
  *Improving LSH via Tensorized Random Projection.* arXiv 2402.07189.
  https://arxiv.org/abs/2402.07189 — speeds up LSH projection step via tensor
  decomposition. Relevant to the Fortran question (§8): Tensorized projection is
  exactly the matrix-multiply pattern Fortran/BLAS is fastest at.

### 4.2 Tradeoff matrix

Columns: the three required properties + Fortran fit.

| family | (1) collision-resistant | (2) fast | (3) navigatable | Fortran fit | examples |
|---|---|---|---|---|---|
| Cryptographic hash | strong (SHA-256: 2^128) | fast (~10 cycles/byte for SHA-256, ~2 for BLAKE3) | none (avalanche) | trivial; rarely needed | SHA-256, BLAKE3, K12 |
| Merkle-of-cryptographic | strong | fast (parallel) | none | fine | Splinter `crystallize::Oid`, IPFS CIDs |
| MinHash (set LSH) | none | very fast | moderate (Jaccard) | not the natural fit | shingled docs |
| SimHash (cosine LSH) | none | fast (one dot product per bit) | strong (cosine) | excellent (dot products) | Google near-duplicate |
| Cross-polytope LSH | none (provably attackable) | fast | optimal-for-angular | excellent | Andoni 2015 |
| Spectral Hashing (learned) | none + distribution-brittle | medium (FFT-based encode, fast decode) | very strong | excellent (eigendecomp + thresholding) | Weiss-Torralba 2008 |
| Coincidence hash (current `<5,5>`) | weakened SHA-256 (Attack 1) | slow (~100-1000× SHA-256) | none (final SHA-256 destroys it) | excellent (5×5 matrix multiplies) | `bootstrap::canonical_hash` |
| Coincidence-projection-only (no final SHA-256) | weak (depends on null-space) | medium (the projection step) | strong (L2 in projected space) | excellent | hypothetical; the intermediate of current `canonical_hash` |
| Hybrid: SHA-256 ‖ projection-coordinate | strong (SHA-256 part) | medium (both passes) | strong (projection part) | excellent (only the projection part needs Fortran) | TopLoc-style commitment-plus-envelope |
| Tensorized random projection | none | very fast | strong | excellent (BLAS) | Garg 2024 |
| Weisfeiler-Lehman graph hash | depends on tree depth (weak in practice) | medium | strong for graph isomorphism | poor (irregular) | classical graph isomorphism baseline |
| Content-defined chunking | none | very fast (rolling) | moderate (chunk-boundary stability) | n/a | FastCDC, Rabin-Karp |

**Reading the matrix:**

No single family scores high on all three. The strongest candidates for a unified
primitive — "coincidence-projection-only" and "hybrid SHA-256 ‖ projection-coordinate"
— both REQUIRE the projection step to be exposed *separately* from the cryptographic
step. Either you pay the SHA-256 cost twice (once on raw input for storage-OID, once
on projection bytes for navigation-OID, packed into a hybrid output), or you accept
that the projection-coordinate IS the navigation-OID and is cryptographically weak
by construction.

The Splinter raw-SHA-256-Merkle row is the *only* row that scores high on (1) and
(2) without claiming (3). It is also the only row that admits a clean
composition law: parent hash = `H(tag || len || child_hashes)`. Coincidence-projection
hashes do NOT compose this way — projecting children's projection-coordinates into a
parent's projection does not preserve the parent's structural identity.

---

## 5. The fundamental question — is there an impossibility result?

**Yes, partial.** The literature documents two relevant lower bounds:

### 5.1 The LSH lower bound (Motwani-Naor-Panigrahy 2006)

For LSH in a metric space (V, d), define the LSH parameter ρ for the
near/far ratio c > 1 as:

> ρ = log(1/P_1) / log(1/P_2)

where P_1 is the collision probability for points at distance ≤ R ("near") and P_2
is the collision probability at distance ≥ cR ("far"). Smaller ρ = better LSH
(near points collide more relative to far points).

The Motwani-Naor-Panigrahy bound (https://arxiv.org/abs/cs/0510088) shows:

> For Hamming metric: ρ ≥ 1/c - o(1).
> For Euclidean metric: ρ ≥ 1/c² - o(1) (tight, matching the Andoni-Indyk upper bound).

Translation to mirror's three properties:

- Property (3) "navigatable" requires P_1 ≫ P_2 — near contents collide more than
  far ones. ρ small.
- Property (1) "collision-resistant" requires P_2 ≈ 2^{-256} for any far points. ρ
  large (collisions even for near points are exponentially rare).

These pull in opposite directions, and the lower bound proves they cannot both be
satisfied by a single hash family at the same time, except in the trivial cases (c
very close to 1 or c very large). The intersection where both could hold
simultaneously is empty in any interesting metric space.

### 5.2 The avalanche-vs-locality conflict (constructive)

Independent of LSH-specific bounds, the cryptographic avalanche property is
*defined* as: changing any input bit changes each output bit with probability ½.
This is precisely the property that destroys (3): any input perturbation produces
an output perturbation with expected Hamming distance ~n/2 regardless of the
input-perturbation size. A hash with strict avalanche cannot be locality-preserving.

### 5.3 Adversarial confirmation (Prokos et al. 2024, Aamand-Indyk 2025)

The PhotoDNA, PDQ, and NeuralHash perceptual hashes — the closest deployed analogs
to a "navigatable cryptographic hash" — have been broken by black-box adversarial
attacks producing targeted collisions in O(N) queries. This is empirical confirmation
that hashes designed for navigation are not collision-resistant in the
cryptographic sense.

### 5.4 What the impossibility result does NOT say

The Motwani-Naor bound applies to a *single hash family parameterized by one
parameter c*. It does NOT forbid:

- Composite outputs (e.g., a fixed-width OID that PACKS a cryptographic part AND a
  locality-sensitive part as separate sub-fields).
- Multiple hashes computed from the same input (each satisfying a different
  property).
- Hashes derived from a shared canonicalization step (so both hashes agree on the
  "same content" predicate).

This is the loophole the recommendation in §6 exploits.

### 5.5 Verdict on the impossibility question

**A unified primitive is impossible** in the strict sense (one hash function with
one output that satisfies all three). The literature is clear; the lower bound is
tight (Andoni-Indyk match it from above). The adversarial work confirms it in
practice.

**A unified output is possible** if the output is a *composite* (cryptographic
sub-field + navigation sub-field, derived from a shared canonical representation
of the input). This is the (B) pluggable architecture below, packaged so callers
see a single OID at the type level.

---

## 6. Recommendation — generic-over-hash cascade, NOT composite ContentOid

**REWRITTEN 2026-05-30.** The original recommendation (composite `ContentOid` with
`storage` and `navigation` sub-fields) is superseded. See the top-of-file banner
and `docs/specs/store-vs-db-and-the-cascade.md` for the load-bearing distinctions.
This section retains the original text below the rewrite header for record; only
the RECOMMENDATION proper is replaced.

### 6.0 The new recommendation

**Verdict: generic-over-hash cascade.** Make the Merkle tree generic over the hash
algorithm. Each consumer picks its own primitive. Different consumers can have
structurally different trees. No composite type; no "navigation field" attached to
storage OIDs.

Concretely:

- **`@mirror/store`** (open foundation, content-addressed storage gate, git-backed
  by default) uses `Merkle<BLAKE3>`. BLAKE3 is standard, fast, Merkle-native by
  construction, has no float dependency, and sidesteps Attack 1. Verification on
  write lives in `@mirror/store`. mirror MUST work without `@spectral/db`.
- **`@spectral/db`** (potentially closed-source engine on top, the spectral graph)
  uses **`VoidPointer`** as its navigation primitive. `VoidPointer` is NOT a hash
  function. It is the *spectral coordinate* (eigenvalue vector of the node's local
  Laplacian) that spectral-db ALREADY computes and stores via `SpectralCoordStore` +
  `coord_oids` + `spectral_distance_eigen`. The existing code IS the pattern; we
  are naming it. `VoidPointer` is a reclaiming move — every engineer learns void
  pointers are evil; in mirror they are the load-bearing thing that makes the
  alignment math work. Full circle. Connects to void-dual-geometry
  (`~/.reed/visibility/protected/practice/insights/coincidence/void-dual-geometry.md`)
  — coordinates into the Void (λ₀=0 axis where all eight dualities meet).
- These are **separate primitives serving separate consumers**. No `ContentOid`
  newtype carries both. The Merkle generic parameter `H` is the architectural
  enabler: `Splinter<BLAKE3>` is what the store uses; `@spectral/db` does not need
  a Splinter at all (it stores spectral coordinates keyed by store-OIDs).

### 6.0.1 The cascade

Genericity goes fully through the type tower:

- `Splinter<H: MerkleHash = Blake3>`
- `Content<H>`
- `Body<H>`
- `Crystallization<H>` (singular event)
- `Crystallizations<H>` (the table; renamed from `Registry`)
- `kintsugi_tick<H>`

Hash-blind types stay concrete:

- `Ref` (renamed from `ActionPath`; matches mirror's nav-ref vocabulary; `action` is
  dead since we have prism / glass / 5-ops, not "actions")
- `CrystallizeError`
- `IoError`
- `ScalarLoss` (pinned to become `Transparency` as a Lens in the *next* tick after
  the cascade; positive-frame, optical-family-native, dual of Dark spans, lens-
  algebra-composable — not baked in here)

A single bootstrap binary hosts multiple `H`-worlds. Each consumer (store, db,
future engines) gets its own `Crystallizations<H>`. The store's tree and db's
coordinate space are structurally independent; they share only the canonical bytes
that keyed them.

### 6.0.2 What the c3a01e3 recommendation got wrong

- **"Verification belongs to `@spectral/db`."** Wrong direction. Verification
  belongs to `@mirror/store` — the storage gate is where bytes enter; the gate
  is the place that can refuse. `@spectral/db` is the engine on top; it consumes
  verified bytes from `@mirror/store`. Putting verification in the engine couples
  the open foundation to the (potentially closed) engine. The architecture refuses
  that coupling.
- **"Pack navigation into the OID."** Wrong altitude. The navigation primitive
  belongs to `@spectral/db` and is a *coordinate*, not a hash. Forcing it into a
  composite OID makes the open foundation know about the engine's geometry.
- **"One canonical-byte serializer feeds two hashes."** Half-right. The canonical
  bytes ARE shared (they have to be — the store-OID is the key the engine uses to
  attach coordinates). What's not shared is the type carrying both — the store
  hands out a `Splinter<BLAKE3>` OID and that's the address; the engine indexes
  *by* that address into its own `VoidPointer` space.

### 6.0.3 Cross-references

- `docs/specs/store-vs-db-and-the-cascade.md` — the load-bearing landing-page spec.
- `docs/specs/kintsugi-minimum-runnable.md` — carries the cascade renames
  (`Registry` → `Crystallizations`, `ActionPath` → `Ref`) in its amendment section.
- `docs/specs/coincidence-hash-collapse.md` — SUPERSEDED top-banner now points
  here; the CHC tick plan is obsolete in its original form.

The original §6 follows below for record. Skim it for the *shape* of the
composite-OID design — useful to know what was considered and rejected; do NOT
implement.

---

### ORIGINAL §6 (superseded; record only) — (B) pluggable, with one shared canonicalization

**Verdict: (B).** Pluggable per-operation hashes, with a shared canonical byte
representation and a typed OID that carries both. Not (A) because §5 forbids it.
Not (C) because the prototyping question is already answered by the literature and
by the current spectral-db architecture (which is implicitly pluggable, just
informally).

### 6.1 The architecture

Introduce a typed OID with two field-level sub-addresses:

```rust
// Sketch only — no implementation in this spec.
// Honors no-bare-types: every sub-field is its own newtype.

struct ContentOid {
    storage:    StorageHash,     // 32 bytes — SHA-256 (or BLAKE3) of canonical bytes
    navigation: NavCoordinate,   // N×k bits — projection-coordinate of canonical bytes
}

struct StorageHash([u8; 32]);          // newtype; never raw bytes
struct NavCoordinate([u8; NAV_BYTES]); // newtype; never raw bytes; NAV_BYTES TBD §9
struct CanonicalBytes(Vec<u8>);        // newtype; the shared input to both hashes
```

**One canonicalization step.** Any `Content` value (text, record, list) flattens to
`CanonicalBytes` via a single deterministic serializer. Both hashes consume
`CanonicalBytes`. This guarantees: two `Content` values produce the same
`ContentOid` iff they produce the same `CanonicalBytes` — the equality predicate
is fixed, regardless of which sub-hash is consulted.

**Storage uses the storage field.** All dispatcher routing, Merkle composition,
cryptographic verification, and replication uses `oid.storage`. This is the
property-(1)-and-(2) path. Implementation: BLAKE3 (recommended) or SHA-256 over
`CanonicalBytes`. ~2-5 cycles/byte. Cryptographic strength SHA-256-or-better.

**Navigation uses the navigation field.** All `near()`, `walk(spectral-weighted)`,
locality queries use `oid.navigation`. This is the property-(3) path.
Implementation: SimHash-style signed-hyperplane projection of `CanonicalBytes`
into `NAV_BYTES * 8` bit positions, packed as bytes. Hamming distance between two
`NavCoordinate`s estimates cosine similarity of the underlying canonicalized inputs.

**Composition law (Splinter / Merkle):**

- `storage` composes as raw-SHA-256 Merkle (Splinter Tick A semantics survive
  unchanged): `parent.storage = SHA-256(tag || len || children.storage.bytes())`.
- `navigation` composes by *averaging the projection coordinates* of children
  (weighted by child content length). The parent's navigation coordinate is the
  centroid of children's navigation coordinates. This is **NOT** what Merkle
  storage hashes do — and it is *exactly* the navigation primitive spectral-db
  wants ("this parent is structurally near these other parents because their
  children's content distributions are similar"). The averaging is well-defined for
  signed-hyperplane LSH; the resulting coordinate has the same distance semantics
  as a coordinate computed from the parent's flattened bytes.

### 6.2 Storage-gate verification interaction

The separately-settled architectural point: verification belongs in `@spectral/db`,
not in the `Content` type. Concretely: a write enters the storage gate carrying
its claimed `ContentOid`; the gate recomputes both fields and compares.

- **`storage` field verification:** cryptographic. The gate recomputes
  `BLAKE3(canonical_bytes)` and rejects on mismatch. This is the load-bearing
  authenticity check.
- **`navigation` field verification:** structural. The gate recomputes the
  projection coordinate and compares for **equality**. Because the projection is
  deterministic from `canonical_bytes` and the projection matrix is content-
  addressed itself (its seed is `"coincidence:projection:{i}:{N}"` per `hash.rs`),
  this is a cheap deterministic recomputation. A divergent navigation field
  indicates either a write-side bug or an attacker trying to inject an
  index-poisoning entry. The gate rejects on mismatch.

Key property: **the navigation field's cryptographic weakness DOES NOT propagate
to storage**, because the storage gate only trusts `storage` for authenticity. The
navigation field is treated as an authenticated index hint, not an identity.

### 6.3 How spectral-db consumes the navigation field

Replace `coord_oids: HashMap<NodeOid, CoordOid>` with `oid.navigation` directly
on each node. The separate `SpectralCoordStore` becomes optional — it still hosts
"true" eigenvalue coordinates for nodes whose budget allows graph-aware spectral
analysis, but the *baseline* navigation coordinate comes from content alone via
the `navigation` field of the OID.

The `near()` linear-scan becomes a Hamming-distance scan over
`oid.navigation` bytes — comparably fast (Hamming over `u64` chunks is
throughput-bounded by memory bandwidth, not CPU). The eigenvalue-distance path
remains available for the budget-allowed nodes; it produces a *finer-grained*
navigation that takes precedence when present. So:

- Tier 1 (always): `oid.navigation` Hamming distance (content-only, fast,
  approximate).
- Tier 2 (budget-permitting): eigenvalue L2 distance via `coord_store` (graph-aware,
  exact, expensive).

This matches the existing spectral-db two-tier intent without requiring an OID
rewrite for every node.

### 6.4 The shared canonical step

The canonicalization is the *one* thing that must be exactly right. Proposed shape:

- `Content::Text(s)` → `b"T" || u64_le(len(s)) || s.bytes()`
- `Content::Record(m)` → `b"R" || u64_le(len(m)) || sorted(m).flat_map(|(k, v)| u64_le(len(k)) || k.bytes() || canonical(v))`
- `Content::List(items)` → `b"L" || u64_le(len(items)) || items.flat_map(canonical)`

This is **byte-identical** to `bootstrap::crystallize::compute_oid`'s framing, EXCEPT
that list/record bodies use **recursively-canonicalized child bytes** instead of
*child OIDs*. This is the difference between a content hash and a Merkle
composition hash, named explicitly in CHC §6.

- For **storage**: we want Merkle composition. `storage.compute()` uses
  `child.storage` for record/list bodies. Authenticity propagates upward.
- For **navigation**: we want content composition. `navigation.compute()` uses
  full canonical bytes (recursively flattened) so the projection coordinate
  reflects *all* content, not just parent-level structure. This is what makes
  parent-near-parent work — two large records with similar leaf distributions get
  similar centroid coordinates.

The canonical bytes are the same input; the two hash functions consume them with
different composition rules.

### 6.5 Why this is (B) not (A)

From the caller's perspective, `ContentOid` is one type. Type-system-wise, it looks
like a unified primitive. But the underlying architecture has **two hash
functions, each picked for one property**: BLAKE3 (or SHA-256) for storage,
SimHash-style projection for navigation. The Motwani-Naor bound forbids a single
hash function from satisfying both; the architecture works around this by
carrying both in one record.

The "shared canonicalization" is the discipline that prevents the two from drifting:
if you change canonicalization, both hashes change. If you change one hash, the
other still grounds on the same bytes. This is the structural invariant that
makes the pluggable architecture coherent.

### 6.6 What does NOT go in `ContentOid`

- **Detector<N>::canonical / canonical_hash output.** The current `<5,5>` projection
  IS a candidate for the navigation field's projection design, but its output IS
  NOT the `ContentOid` — currently `canonical_hash` returns a 64-char hex string,
  which `Oid::hash` wraps. Under this recommendation, `canonical_hash`'s
  projection step becomes `navigation.compute()`, and the final SHA-256 step is
  dropped (we already have `storage` for that purpose). The projection coordinate
  is exposed *before* SHA-256 destroys it.
- **Eigenvalue vectors of the ego Laplacian.** These are graph-properties, not
  content-properties; they live in `SpectralCoordStore` as a separate optional
  refinement (Tier 2 above). They do NOT belong in `ContentOid`.
- **AST-altitude metadata.** The OID is content-addressing only. AST-altitude
  context (file, line, kind, parent reference) belongs in the surrounding
  `Splinter` / `Shard` structure, not in the OID.

---

## 7. Implications for CHC (the upstream-paused collapse plan)

**REWRITTEN 2026-05-30.** Under the new architecture (`@mirror/store` adopts
`Merkle<BLAKE3>` and moves OFF CoincidenceHash entirely), the CHC tick plan is
**obsolete in its original form**. CHC-1 through CHC-5 do not run.

See `docs/specs/coincidence-hash-collapse.md`'s SUPERSEDED top-banner for the
status. The brief version:

- `bootstrap::crystallize::Oid` becomes `Splinter<BLAKE3>` OID under the cascade.
  Splinter ticks live in the cascade implementation tick, not in CHC.
- `prism_core::Oid` (`Detector<3>`) and `bootstrap::canonical_hash` (`<5,5>`) stay
  where they are. They are no longer the storage hash. Whether they ever unify is
  a separate, lower-priority concern — the consumer (storage) that motivated unification
  has moved away.
- The two CoincidenceHash sites continue to serve their existing purposes (whatever
  remaining callers need). If they outlive their callers entirely, they get retired
  in a future hygiene tick.

The original §7 text (CHC-1' through CHC-5 rewrites under the composite-`ContentOid`
recommendation) is retained below for record. Do NOT execute.

---

### ORIGINAL §7 (superseded; record only)

The CHC migration in `coincidence-hash-collapse.md` §7 has five ticks (CHC-1
through CHC-5). Under recommendation (B), the ticks change as follows:

### 7.1 CHC-1 (🔴 byte-stability round-trip test) — keep, scope changes

The round-trip test between `Detector<5>::canonical("content", 5)` and
`bootstrap::canonical_hash` is **still required**, because the projection step
feeds the navigation field. Under (B), the test verifies that the two
implementations of the *projection* agree byte-for-byte. The test corpus stays the
same; the assertion target shifts from "full canonical_hash agreement" to
"projection-coordinate agreement" (i.e., compare the bytes that would feed
`navigation.compute()`, not the bytes that feed the final SHA-256).

### 7.2 CHC-2 (🟢 update prism-core canonical to `<5,5>`) — defer

Under (B), the canonical `Detector<N>` parameterization is a navigation-field
design question, not a storage-field one. Whether the navigation projection is
`<5,5>` or `<3,16>` or something new (e.g., signed-hyperplane SimHash with NAV_BYTES
bits) is a Tier-1-design question now open. CHC-2's specific change
(`Detector<3>` → `Detector<5>` for the canonical) is no longer the right tick.

### 7.3 CHC-3 (🟢 collapse `bootstrap::canonical_hash` to delegate) — replace

Replace with: **introduce `ContentOid` as a typed two-field record, extract the
projection step out of `canonical_hash` as `navigation.compute()`, replace the
final SHA-256 step with `storage.compute() = BLAKE3(canonical_bytes)`.**

This is a larger tick than CHC-3 was; it requires the canonical-bytes serializer
to live somewhere (likely `prism-core::canonical_bytes`), the two hash impls to
live somewhere (storage in `prism-core::storage_hash`, navigation in
`prism-core::nav_projection`), and the `ContentOid` newtype to be the public
surface. The bootstrap's `canonical_hash` becomes a deprecated re-export that
calls `ContentOid::compute(bytes).storage` for cryptographic compatibility
during migration.

### 7.4 CHC-4 (🟢 extract `sha256_merkle`, correct Splinter docstring) — keep,
strengthens

Under (B), Splinter's `crystallize::Oid` IS the storage field of a `ContentOid` — its
docstring becomes: "`crystallize::Oid` is the storage projection of a `Content`'s
full `ContentOid`. Splinter's Merkle composition is the storage-field composition
rule (raw-SHA-256-Merkle)." The cross-wall bridge `Splinter::content_oid()` from
CHC §6.4 becomes `Splinter::nav_coordinate()` — exposing the navigation field
when needed.

The Tick A docstring correction (CHC §6.3) is now stronger: Splinter ISN'T a
different primitive from `canonical_hash`; Splinter IS the storage projection,
`canonical_hash`'s projection step IS the navigation projection, and the
two were always doing different jobs at different altitudes — but they SHARE the
canonical byte representation, which is the new invariant.

### 7.5 CHC-5 (📝 doc updates) — keep, extends

Add to the spec list:

- `docs/specs/spectral-hash-design.md` (this file): the upstream framing CHC
  acts on.
- `boot/std/hash/coincidence.mirror`: split into `boot/std/hash/storage.mirror`
  (cryptographic) and `boot/std/hash/navigation.mirror` (projection). The current
  `coincidence` shape is the navigation one; the storage one is new.

### 7.6 Summary for CHC

**CHC unpauses with a wider scope.** The collapse is no longer "unify two
implementations of the same algorithm"; it's "factor the implementations into
the two roles they were actually serving, expose both in `ContentOid`, and align
on a shared canonicalization." The new tick stream:

1. CHC-1 (🔴): round-trip projection-coordinate agreement test (was: full hash
   agreement).
2. CHC-A (🔴, new): canonical bytes serializer test — verify
   `canonical_bytes(c).len()` matches the expected framing for all `Content` shapes.
3. CHC-B (🟢, new): introduce `ContentOid` newtype with `storage` and `navigation`
   sub-fields. No callers yet; just the type.
4. CHC-C (🟢, new): `ContentOid::compute(bytes)` populates both fields via the
   shared canonicalization. Tests pin small-input bytes for both fields.
5. CHC-3' (🟢, replaces CHC-3): wire `Splinter::compute_oid` to use
   `ContentOid.storage` (no behavioural change for storage — Merkle SHA-256 is
   what it always was). Add `Splinter::content_oid()` returning the full
   `ContentOid`. Old `bootstrap::canonical_hash` becomes a thin wrapper around
   `ContentOid::compute(bytes).storage.hex()` for compat — semantically equivalent
   to the old code only on inputs that happened to take the live path; legacy
   callers explicitly opting into the navigation field call a new fn.
6. CHC-4' (🟢, replaces CHC-4): correct Splinter docstring to the (B)
   architecture, name the storage-vs-navigation split.
7. CHC-5 (📝, expanded): doc updates as §7.5.

Numbers approximate; commit-by-commit shape is for the next iteration.

---

## 8. The Fortran question

Under recommendation (B), the navigation field is a projection of `canonical_bytes`
into a fixed-width coordinate. Two implementation paths are available, both with
Fortran fit:

### 8.1 SimHash-style signed-hyperplane projection (recommended for Tier 1)

- Input: `canonical_bytes` (`Vec<u8>`).
- Step 1: feature extraction — for each byte (or 2-byte n-gram), emit a sparse
  feature vector indexed by the byte value. Sparse representation: `(index, count)`
  pairs.
- Step 2: projection — for `b in 0..NAV_BYTES * 8` bits, compute
  `bit_b = sign(Σ_i count_i * R[b][index_i])` where `R[b]` is a fixed Gaussian
  random vector seeded by `"navigation:projection:{b}:{NAV_BYTES * 8}"`. The signed
  sum maps to a bit.
- Step 3: pack bits into `NAV_BYTES` bytes.

**Fortran fit:** the dominant cost is a (NAV_BYTES * 8) × 256 dense matrix
multiplied by a 256-element vector (the byte-histogram). For NAV_BYTES = 16
(128-bit navigation field, matching a SHA-256 collision-resistance level), the
matrix is 128 × 256 = 32 KB of `f32` weights; the multiply is 32K floating-point
operations per OID — trivially fast in Fortran-SGEMM (`sgemv`), one cache-line
fetch + one vector multiply-add per output bit.

**Fortran host kernel** (proposed): `prism/fortran/src/navigation_project.f90`,
with subroutine signature:

```fortran
subroutine nav_project(canonical_bytes, n_bytes, nav_coord, nav_bits)
  integer(c_int64_t), intent(in)  :: n_bytes
  integer(c_int8_t),  intent(in)  :: canonical_bytes(n_bytes)
  integer(c_int64_t), intent(in)  :: nav_bits         ! NAV_BYTES * 8
  integer(c_int8_t),  intent(out) :: nav_coord(nav_bits / 8)
end subroutine
```

Fitting the `numerical-substrate-via-fortran.md` plan (Mara's other spec): this is
a new "Numerical" region. It uses BLAS-1 (`saxpy`-like accumulation) and a single
fixed `f32` projection matrix loaded at module init. No graph context required;
no LAPACK; no eigendecomposition. The simplest possible Fortran integration.

### 8.2 Coincidence-projection (current `<5,5>`, exposed as navigation)

If the existing `bootstrap::canonical_hash` projection IS the navigation
projection (one option in §6.6 / CHC-2's deferred decision), then the Fortran
kernel hosts the existing `<5,5>` dense matrix multiplies. The output is
`NUM_PROJECTIONS × DIM = 25` f64 values = 200 bytes of navigation coordinate.

**Fortran fit:** even better than SimHash — `<5,5>` is just five `dgemv` calls on
5×5 matrices, plus the per-byte encoding loop (the dominant cost: `(1 + DIM) × n`
SHA-256s, which is NOT Fortran-natural). The SHA-256 in the encoding step is the
bottleneck and would stay in Rust; the matrix multiply is the only Fortran piece.

**Open question:** is the `(1 + DIM) × n` SHA-256 cost in `encode_into_basis`
intentional? Per `hash.rs:encode_into_basis`, every input byte triggers `1 + DIM`
SHA-256s — that's the speed killer in §3.2. The SimHash-style approach (§8.1)
replaces this with a single histogram pass + matrix multiply, which is 100-1000×
faster. **Recommendation:** §8.1 unless the coincidence-projection's specific
structure (eigenvalue interpretability per `eigenboard-representation.md` §644-670)
is load-bearing for downstream consumers.

### 8.3 The storage hash

BLAKE3 or SHA-256, no Fortran involvement. Both have fast hand-tuned Rust
implementations. BLAKE3 is preferred on speed (~2-5× faster), SHA-256 on
ecosystem ubiquity.

### 8.4 New substrate primitive declaration

**REWRITE NOTE 2026-05-30.** Under the new architecture, the substrate primitive
is not `@hash/navigation` (the navigation field of a composite OID). It is
`@spectral/db`'s `VoidPointer` action surface (the spectral coordinate that
`SpectralCoordStore` already computes). The grammar sketch below is retained for
the *shape* it describes — a deterministic projection action surfaced at the
substrate altitude — but the **name, namespace, and consumer change**: this
belongs to `@spectral/db`'s grammar, not `@hash`. `@mirror/store` has no projection
action; its hash is `Merkle<BLAKE3>`, pure cryptographic, no Fortran involvement.

Proposed grammar at `boot/std/hash/navigation.mirror` (sketch, no implementation
in this spec, **under the original recommendation — superseded**):

```mirror
grammar @hash/navigation {
  type dim = 256        # byte-histogram input dimension
  type bits = 128       # nav_coord output bits
  type projection = matrix(bits, dim)

  action project { input: bytes, output: navcoord }
}
```

The `project` action would lower to the Fortran subroutine via `@code/fortran`
(per `numerical-substrate-via-fortran.md`). The projection matrix would be
seeded deterministically and committed to the substrate's grammar-defined seed
space, so cross-implementation byte-stability is guaranteed.

---

## 9. Open decisions for Alex

**REWRITE NOTE 2026-05-30.** Under the cascade, decisions §9.1 (NAV_BYTES) and
§9.3 (navigation projection design) no longer apply at the `@mirror/store` altitude
— the store has no navigation field. Both questions reframe as `@spectral/db`'s
`VoidPointer` design space and are scoped to that engine's spec (TBD). Decision
§9.2 (BLAKE3 vs SHA-256) is resolved: **BLAKE3** for `@mirror/store`, per the
rewrite (sidesteps Attack 1; Merkle-native; ~2-5× faster). The text below is
retained for the research it carries.

Decisions this research cannot make alone. Three named, with the recommendation
Mara would default to in the absence of an override.

### 9.1 NAV_BYTES (navigation coordinate width)

**Question:** how wide is the navigation field? Tradeoff: more bits = finer
navigation resolution + storage cost.

- 64 bits (8 bytes): smallest plausible; cosine similarity estimated to ±1/64
  per dimension; rough but adequate for first-cut clustering.
- 128 bits (16 bytes): matches SHA-256 collision-resistance level; reasonable
  default.
- 256 bits (32 bytes): matches `storage` width; doubles OID size to 64 bytes;
  fine-grained navigation but storage doubles.
- 5 × 5 × 8 = 200 bytes (the current `<5,5>` coincidence output): if we keep the
  coincidence projection AS the navigation projection per §8.2.

**Mara's default:** 128 bits. Cleanly matches SHA-256 strength; doubles OID size
but OID isn't the storage-dominant cost (the content bytes are). The 200-byte
coincidence-projection option is also reasonable if Alex wants to preserve the
5×5 eigenstructure framing from `eigenboard-representation.md`.

### 9.2 Storage hash: BLAKE3 vs SHA-256

**Question:** which cryptographic hash backs the storage field?

- **SHA-256:** ecosystem ubiquity. Already in the `sha2` crate the bootstrap
  depends on. Used by every existing OID. Fast enough (~10 cycles/byte).
- **BLAKE3:** ~2-5× faster on modern hardware. Built-in Merkle parallelism.
  Different dependency. Newer, less reviewed than SHA-256 but mature now.

**Mara's default:** SHA-256 for v1, with a clean migration path to BLAKE3 in v2.
The `StorageHash` newtype hides the choice; switching is a substrate change, not
a surface change.

### 9.3 Navigation projection: SimHash signed-hyperplane vs coincidence `<5,5>`

**Question:** which projection design backs the navigation field?

- **SimHash (§8.1):** straightforward, well-studied, ~100-1000× faster than the
  current `<5,5>`. Outputs Hamming-distance-compatible bytes. No tie to the
  5-operation / 5-duality eigenstructure framing.
- **Coincidence `<5,5>` (§8.2):** preserves the structural framing
  (`eigenboard-representation.md` §644-670: the 5×5 tensor IS the connection
  matrix of the eigenboard's bundle). Slow encode step due to per-byte SHA-256s;
  Fortran helps the projection step but not the encoding.

**Mara's default:** SimHash for the production navigation field, **with** the
coincidence projection retained as an *optional refinement* analogous to
spectral-db's eigenvalue Tier 2. The structural framing then lives where it
belongs (the optional eigenstructure analysis), not in the hot-path navigation
field. Alex's call: is the `<5,5>` framing load-bearing enough to justify the
speed cost?

---

## 10. Stop-and-report findings (per Mara's brief)

**REWRITTEN 2026-05-30.** Findings updated to match the LRM-collapsed architecture.
The research findings (§2, §3.1, §5, §4.1) stand verbatim; only the architectural
verdict has changed.

- **`spectral-db`'s `navigatable_oid` doesn't exist as a literal symbol.** The
  closest implementations are `SpectralIndex::near`, `SpectralIndex::spectral_distance_eigen`,
  and `SpectralCoordStore`. Under the rewrite, this existing pattern IS the
  `VoidPointer` primitive that `@spectral/db` exposes — named, not invented. The
  architecture today (OID + separate coordinate store keyed by OID) becomes the
  architecture going forward; the two-address-per-node shape is correct. (§2)
- **Attack 1 verdict: `canonical_hash` is weaker than SHA-256, narrower than
  originally claimed.** It SHA-256s the f64 bit patterns of projected coordinates,
  and the EPSILON-clamped projection step admits adversarially-constructed
  collisions — but the construction must satisfy five SHA-256-seeded EPSILON-clamped
  equations in parallel, not one. Still cryptographically weaker than naked SHA-256.
  Made moot for `@mirror/store` by adopting BLAKE3. (§3.1 narrowing block)
- **A unified hash IS impossible.** Motwani-Naor-Panigrahy 2006 establishes the
  ρ ≥ 1/c² lower bound for Euclidean LSH; the avalanche property of cryptographic
  hashes is constructively the opposite of locality-sensitivity. The literature
  is clear; the bound is tight. The new architecture honors this differently from
  the original recommendation — not by packing two hashes into one composite, but
  by giving the two consumers (`@mirror/store`, `@spectral/db`) **separate
  primitives entirely**, made tractable by making the Merkle tree generic over the
  hash algorithm. (§5)
- **The recommendation is the generic-over-hash cascade.** Not a composite OID.
  `@mirror/store` uses `Merkle<BLAKE3>`; `@spectral/db` uses `VoidPointer` (the
  existing `SpectralCoordStore` pattern, renamed). The Merkle tree (and its
  `Splinter`, `Content`, `Body`, `Crystallization`, `Crystallizations`) is generic
  over `H`. Hash-blind types stay concrete. See §6.0 and
  `docs/specs/store-vs-db-and-the-cascade.md`. (§6)
- **Verification belongs to `@mirror/store`, not `@spectral/db`.** The earlier
  framing in this spec (§6.2, §1's architectural context) was wrong-direction.
  Corrected in the amendment banner and §1's architectural-context block. mirror
  MUST work without `@spectral/db`; verification on write therefore lives in the
  open foundation.
- **CHC is SUPERSEDED.** The original CHC tick plan does not run. The two existing
  CoincidenceHash sites stay where they are; whether they ever unify is a separate
  (now lower-priority) concern. See the SUPERSEDED banner on
  `coincidence-hash-collapse.md`. (§7 rewrite)
- **`ScalarLoss` → `Transparency` (as a Lens)** is pinned for the *next* tick
  after the cascade. Positive-frame, optical-family-native, dual of Dark, lens-
  algebra-composable. Noted; not baked in.
- **No 2024-26 paper proposes exactly this hybrid with working implementation.**
  TopLoc (2025) is architecturally closest (LSH commitment + cryptographic
  envelope for ML inference verification). The composite-OID design considered
  here turned out to be the wrong packaging; the generic-over-hash cascade is the
  packaging Alex called. (§4.1 — the research stands; the application changed.)
