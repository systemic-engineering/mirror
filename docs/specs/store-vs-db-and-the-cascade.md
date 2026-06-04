# `@mirror/store` vs `@spectral/db` — and the generic-over-hash cascade

*2026-05-30. Mara. Architecture landing-page spec. Captures the load-bearing
distinctions that LRM-collapsed several over-built proposals into a simpler shape,
so future sessions don't repeat the conflations earlier specs made. Markdown only;
no `.rs` touched in this commit. The cascade implementation tick is forthcoming.*

**Status:** Red — architecture pinned; types renamed; the generic parameter is
specified; verification ownership is corrected. The implementation tick lands the
Rust cascade.

**Audience:** any agent or human reading the spec corpus to absorb today's
architecture. Read this *first*, then — if you need depth on any one piece — chase
the cross-references.

---

## 1. The two-layer distinction (load-bearing)

mirror's content-addressing infrastructure splits into two layers. The split is
**architectural** and **business-model-relevant** and must not be re-conflated:

### 1.1 `@mirror/store` — the open content-addressed storage gate

- **Role:** the foundation. Open. Content-addressed. Git-backed by default. Where
  bytes enter the system. Where verification on write lives.
- **License/posture:** Apache-2.0 (per `feedback-no-paywall-in-compiler.md`). The
  foundation does not paywall.
- **Hash:** `Merkle<BLAKE3>`. Standard, fast, Merkle-native by construction, no
  float dependency, sidesteps Attack 1 (`spectral-hash-design.md` §3.1) entirely.
- **Invariant:** **mirror MUST work without `@spectral/db`.** The store is the
  base case; the engine is the bonus. Code, tests, and dependency graphs all
  honor this. The store has no compile-time or runtime dependency on the engine.
- **Verification ownership:** writes that enter the store carry a claimed OID;
  the store recomputes BLAKE3 over canonical bytes and rejects on mismatch. This
  is the load-bearing authenticity check. **It lives in the store, not in
  `@spectral/db` and not in the `Content` type.** Earlier framing (in
  `spectral-hash-design.md`'s pre-amendment §6.2 and §10) that placed verification
  in `@spectral/db` was wrong-direction and has been corrected.

### 1.2 `@spectral/db` — the engine on top

- **Role:** the engine. Navigation. Spectral graph. Coordinate-based queries
  (`near`, `walk(spectral-weighted)`, future `cluster`). Lives ABOVE the storage
  gate; consumes verified bytes addressed by store-OIDs.
- **License/posture:** potentially closed source. The engine is the place a
  commercial offering CAN live; the foundation is the place where it can't.
- **Navigation primitive:** **`VoidPointer`** — NOT a hash function. It is the
  *spectral coordinate* (eigenvalue vector of the node's local Laplacian) that
  spectral-db **already computes and stores** via `SpectralCoordStore` +
  `coord_oids` + `spectral_distance_eigen`. The existing code IS the pattern;
  the rename names it. See §3 for the reclaiming-move rationale and §4 for the
  existing-code mapping.
- **Dependency direction:** `@spectral/db` depends on `@mirror/store`. The store
  does not know about the engine.

### 1.3 Why this is the right cut

- **Open foundation, closed engine** is a coherent OSS + commercial posture.
  Conflating the two (e.g., putting `VoidPointer` into the storage OID, or
  putting BLAKE3-verification into the engine) couples them in ways that break
  the posture.
- **The two consumers have genuinely different requirements.** Storage needs
  cryptographic strength + Merkle composition + speed. Navigation needs
  L2-locality + coordinate arithmetic + graph context. The literature
  (Motwani-Naor-Panigrahy 2006, `spectral-hash-design.md` §5) proves a single
  hash family cannot satisfy both. Separating the primitives (rather than packing
  them into one composite) honors the impossibility result without paying for it
  twice.
- **The store is small.** It is the simplest possible thing: bytes in, OID out;
  bytes out by OID; integrity check on write. Keeping it small protects the
  foundation from engine-altitude bloat.

---

## 2. The cascade — generic over the hash algorithm

The architectural enabler that makes the two-layer split clean is making the
Merkle tree **generic over the hash algorithm**. Per Alex: *"If we make the
AST/MerkleTree generic over the hash algorithm... then everything else falls
out."*

### 2.1 What goes generic

The types that carry content addressing parameterize over `H`:

- `Splinter<H: MerkleHash = Blake3>`
- `Content<H>`
- `Body<H>`
- `Crystallization<H>` (singular event)
- `Crystallizations<H>` (the table; renamed from `Registry` — see §2.3)
- `kintsugi_tick<H>`

Default `H = Blake3` so naive callers get the store's hash without ceremony.

### 2.2 What stays concrete (hash-blind)

Types that do not carry hash bytes stay concrete:

- `Ref` (renamed from `ActionPath` — see §2.3)
- `CrystallizeError`
- `IoError`
- `ScalarLoss` (pinned to become `Transparency` as a Lens in the next tick —
  see §5)

If a type does not embed an OID and does not call a hash function, it does not
take the `H` parameter. This keeps the generic blast radius small.

### 2.3 Renames

Two renames thread through the cascade:

- **`Registry` → `Crystallizations`.** The table that holds bound substrate
  action implementations is the plural of `Crystallization` (the singular event
  a tick produces). "Registry" is generic-language; "Crystallizations" names the
  discipline.
- **`ActionPath` → `Ref`.** Matches mirror's nav-ref vocabulary (the `.`, `..`,
  `...`, `~`, `@`, `^`, `HEAD` set, per CLAUDE.md's "Navigation References"
  table). `action` is dead since we have prism / glass / 5-operations
  (focus / project / split / shift / settle), not "actions". The substrate's
  surface `action enumerate { ... }` keyword is *not* part of this rename —
  that's substrate vocabulary, not Rust vocabulary.

### 2.4 Multiple `H`-worlds in one binary

A single bootstrap binary hosts multiple `H`-worlds. Each consumer (store, db,
future engines) gets its own `Crystallizations<H>`. The store's tree and the
engine's coordinate space are structurally independent; they share only the
canonical bytes that keyed them.

This is what makes the two-layer split tractable: the store can hand out
`Splinter<Blake3>` OIDs without any awareness that `@spectral/db` indexes
by those OIDs into its own `VoidPointer` space.

---

## 3. `VoidPointer` — reclaiming move

`@spectral/db`'s navigation primitive is `VoidPointer`. The name is a reclaiming
move: every C/C++ engineer learns `void *` is the evil generic pointer; in mirror
they become the load-bearing thing that makes the alignment math work. Full
circle.

### 3.1 What it IS

A spectral coordinate. Specifically: the eigenvalue vector of the node's local
Laplacian, projected into a fixed-width coordinate, stored as little-endian f64
bytes, content-addressed under whatever H the engine picks.

It is NOT:

- A hash function. `VoidPointer` does not satisfy cryptographic collision-
  resistance; it is not asked to.
- A field of the store-OID. The store-OID is `Splinter<Blake3>`. The
  `VoidPointer` is a *separate address* attached to a store-OID via
  `coord_oids: HashMap<NodeOid, CoordOid>` (the existing spectral-db pattern).
- Required. Computing `VoidPointer`s is bounded by `SpectralBudget`; not every
  node has one. Nodes without `VoidPointer`s are still navigable via the
  fallback paths spectral-db already implements (the SHA-256-byte-distance branch
  in `SpectralIndex::near`).

### 3.2 Why the name

From Alex (paraphrased): *"void pointers are what you learned were evil. In our
world they're the thing that makes the alignment math work."* The reclaim is
intentional. It also connects to the void-dual-geometry framework
(`~/.reed/visibility/protected/practice/insights/coincidence/void-dual-geometry.md`):
coordinates into the Void (λ₀ = 0 axis where all eight dualities meet). The
`VoidPointer` is the navigational handle into that geometry.

### 3.3 Existing-code mapping

The `VoidPointer` is not new code; it is a name for the existing pattern. The
mechanism already lives in:

- `spectral-db/src/spectral_store.rs` — `SpectralCoordStore` (content-addressed
  storage for eigenvalue vectors).
- `spectral-db/src/index.rs` — `SpectralIndex::near(target_oid, max_distance)`,
  `SpectralIndex::spectral_distance_eigen(oid_a, oid_b, coord_store)`.
- `spectral-db/src/index.rs` `coord_oids: HashMap<String, String>` — the
  store-OID → VoidPointer-OID indirection.
- `spectral-db/db.conv` — `action near { ... }`, `action walk { ... }` (the
  surface that consumes navigation).

The "swap the hash function here" comment in `spectral-db/src/index.rs#1-5`
(documented in `spectral-hash-design.md` §2.1) IS the architectural slot the
`VoidPointer` name fills.

### 3.4 What the rename buys

- Names the discipline. `coord_oids` is generic-language; `VoidPointer` names
  the structural role (the navigational handle into the Void).
- Aligns with the corpus (void-dual-geometry; the λ₀ axis; the eight dualities
  meeting at the trivial eigenvalue).
- Refuses the c3a01e3 packaging error (don't bolt navigation onto the storage
  OID; expose it as its own primitive at the engine altitude).

---

## 4. What changes vs the pre-2026-05-30 framing

The c3a01e3 spec (`spectral-hash-design.md`) recommended a composite
`ContentOid { storage, navigation }`. Same day, the conversation continued and
found that recommendation over-built. The collapse:

| Was (c3a01e3) | Is (2026-05-30) |
|---|---|
| `ContentOid { storage: StorageHash, navigation: NavCoordinate }` | No composite. `Splinter<H>` for store; `VoidPointer` for db. Separate types, separate consumers. |
| Verification belongs to `@spectral/db` | Verification belongs to `@mirror/store` (the foundation that MUST work without the engine) |
| One shared canonical-byte serializer feeds both hashes | Canonical bytes are still shared (the store-OID is the key the engine indexes by); the composite type that carries both is refused |
| `@mirror/store` uses CoincidenceHash (per CHC) | `@mirror/store` uses `Merkle<BLAKE3>`; CoincidenceHash sites stay where they are |
| `Registry` (dispatcher table) | `Crystallizations` (the discipline named) |
| `ActionPath` (substrate action address) | `Ref` (mirror's nav-ref vocabulary) |
| `ScalarLoss` (kintsugi loss aggregator) | (next tick) `Transparency` as a Lens |

The research in `spectral-hash-design.md` §4–§5 (the LSH survey, the
Motwani-Naor-Panigrahy 2006 impossibility result, the tradeoff matrix) **stands
verbatim** — those findings ground the new architecture too. What changed is the
application: instead of packing the two primitives into one composite OID, give
the two consumers separate primitives entirely, made tractable by the
generic-over-hash cascade.

---

## 5. Pinned for the next tick — `ScalarLoss` → `Transparency` (as a Lens)

Not baked into the cascade. Named here so a future reader has the direction.

- **Why rename:** `ScalarLoss` is negative-frame ("the amount of loss"); the
  rest of mirror's optical vocabulary is positive-frame (light, focus, project,
  settle). `Transparency` is the dual of Dark spans, optical-family-native, and
  composes naturally under lens algebra.
- **Why as a Lens:** loss is observed *through* a particular projection of the
  state; the Lens abstraction names that projection structurally. A Lens that
  reports `Transparency` over an AST yields a scalar in [0, 1] (or its newtype
  equivalent) where 1 = fully transparent (no impedance), 0 = opaque.
- **When:** the *next* tick after the cascade lands. Doing it inside the cascade
  would conflate two unrelated changes; keeping it next makes both reviewable.
- **Where the change lands:** wherever `ScalarLoss` currently appears in the
  kintsugi loss path (`kintsugi-minimum-runnable.md` §3; the dispatcher's
  `Value::Verdict / Value::Scalar` surface in Tick A's sketch).

---

## 6. Implementation outline (for the forthcoming cascade tick)

This spec is markdown-only; no `.rs` lands here. The implementation tick (task
#125, forthcoming) carries:

1. **Introduce the trait.** `pub trait MerkleHash` with the minimal surface
   the existing `Splinter` impl needs (digest type, finalize, update, fixed
   output size as an associated const). Implement for `Blake3`. Sha256 left as
   a possible second impl if any consumer asks; otherwise omitted.
2. **Parameterize the type tower.** `Splinter<H>`, `Content<H>`, `Body<H>`,
   `Crystallization<H>`, `Crystallizations<H>`, `kintsugi_tick<H>`. Default
   `H = Blake3`.
3. **Renames.** `Registry → Crystallizations`; `ActionPath → Ref`. Whole-word
   replacement; the kintsugi engine's `cmd_kintsugi --transform` body is
   appropriate for this (the rename itself becomes a fracture, in the spirit
   of the grammar → prism rename).
4. **Audit verification ownership.** Wherever the store currently delegates
   verification to a caller or to `@spectral/db`, move it into the store. The
   store recomputes the BLAKE3 digest over canonical bytes on write and rejects
   on mismatch.
5. **Confirm no spectral-db dependency from the store.** Cargo graph check;
   tests run with `@spectral/db` disabled (feature-gated if needed). The
   invariant "mirror MUST work without `@spectral/db`" is mechanized as a CI
   check or explicit feature-flag test.
6. **Defer the `Transparency` rename.** Pinned for next tick; this tick does
   not touch `ScalarLoss`.

The ordering: trait first (red-first against a test that constructs a
`Splinter<Blake3>` and asserts its digest matches expected bytes), then
parameterize, then rename, then verification audit. The renames and parameter
introduction are mechanical; the verification audit is the load-bearing
design work.

---

## 7. Cross-references

- `docs/specs/spectral-hash-design.md` (commit `c3a01e3`, amended 2026-05-30) —
  upstream framing. §4–§5 research stands; the composite-`ContentOid`
  recommendation is rewritten in §6.0. Read §2–§5 of that spec for the depth
  on "what `navigatable_oid` actually means", "what's wrong with `canonical_hash`'s
  collision-resistance", and "why a single hash can't satisfy all three
  properties."
- `docs/specs/coincidence-hash-collapse.md` (commit `e9c259b`) — SUPERSEDED
  top-banner names the rewrite. The map (§2–§4) is still accurate as historical
  record; the collapse plan (§5–§8) is obsolete.
- `docs/specs/kintsugi-minimum-runnable.md` (commit `aca3538`, amended
  2026-05-30) — carries the cascade renames in its §11 amendment, with the
  generic-over-hash note and the `Transparency` direction. The four-piece
  structure, the substrate-vs-floor split, and the loss-from-`@epistemologic/properties`
  grounding all stand.
- `docs/specs/mirror-store.md` (Mara, 2026-05-22) — the three-layer parser /
  Layer-1 store spec. Predates today's conversation; explicitly **does not**
  speak for `@spectral/db`. Use as background on the store's layering; treat
  the cascade naming here as the current language.
- `~/.reed/visibility/protected/practice/insights/coincidence/void-dual-geometry.md`
  — the framework the `VoidPointer` reclaim ties into (λ₀ = 0 axis; eight
  dualities meeting at the Void).
- `~/.claude/projects/-Users-alexwolf-dev-projects-spectral/memory/` — session
  memory; entries on @mirror/store vs @spectral/db distinctions live here.
- CLAUDE.md (project) — the "Navigation References" table that grounds the
  `Ref` rename.
- `feedback-no-paywall-in-compiler.md` — the OSS posture that pins
  `@mirror/store` as Apache-2.0.
- `feedback-no-bare-types.md` — the type-discipline principle the cascade
  honors (`H` is a typed parameter, not a string; `Ref` is a newtype, not a
  raw path string).

---

## 8. What this spec is and isn't

**Is:** a single landing-page architectural anchor. The place a future agent
starts when they need to understand today's content-addressing architecture
without piecing it together from four amended specs.

**Is not:** an implementation spec. The forthcoming cascade implementation tick
lands the Rust; this spec lands the language and the load-bearing invariants.

**Specifically refuses:** the c3a01e3 packaging that bolts navigation onto the
storage OID, the c3a01e3 framing that verification belongs to `@spectral/db`,
and the CHC plan that motivated unifying CoincidenceHash sites for storage use.
Those are documented as superseded in their respective specs; this one names the
replacement positively.

---

*Open foundation, closed engine. Two primitives, one cascade. The Void has
pointers now; their meaning is alignment, not absence.*
